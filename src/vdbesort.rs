//! 2011-07-09
//!
//! The author disclaims copyright to this source code.  In place of
//! a legal notice, here is a blessing:
//!
//!    May you do good and not evil.
//!    May you find forgiveness for yourself and forgive others.
//!    May you share freely, never taking more than you give.
//!
//!
//! This file contains code for the VdbeSorter object, used in concert with
//! a VdbeCursor to sort large numbers of keys for CREATE INDEX statements
//! or by SELECT statements with ORDER BY clauses that cannot be satisfied
//! using indexes and without LIMIT clauses.
//!
//! The VdbeSorter object implements a multi-threaded external merge sort
//! algorithm that is efficient even if the number of elements being sorted
//! exceeds the available memory.
//!
//! Here is the (internal, non-API) interface between this module and the
//! rest of the SQLite system:
//!
//!    sqlite3VdbeSorterInit()       Create a new VdbeSorter object.
//!
//!    sqlite3VdbeSorterWrite()      Add a single new row to the VdbeSorter
//!                                  object.  The row is a binary blob in the
//!                                  OP_MakeRecord format that contains both
//!                                  the ORDER BY key columns and result columns
//!                                  in the case of a SELECT w/ ORDER BY, or
//!                                  the complete record for an index entry
//!                                  in the case of a CREATE INDEX.
//!
//!    sqlite3VdbeSorterRewind()     Sort all content previously added.
//!                                  Position the read cursor on the
//!                                  first sorted element.
//!
//!    sqlite3VdbeSorterNext()       Advance the read cursor to the next sorted
//!                                  element.
//!
//!    sqlite3VdbeSorterRowkey()     Return the complete binary blob for the
//!                                  row currently under the read cursor.
//!
//!    sqlite3VdbeSorterCompare()    Compare the binary blob for the row
//!                                  currently under the read cursor against
//!                                  another binary blob X and report if
//!                                  X is strictly less than the read cursor.
//!                                  Used to enforce uniqueness in a
//!                                  CREATE UNIQUE INDEX statement.
//!
//!    sqlite3VdbeSorterClose()      Close the VdbeSorter object and reclaim
//!                                  all resources.
//!
//!    sqlite3VdbeSorterReset()      Refurbish the VdbeSorter for reuse.  This
//!                                  is like Close() followed by Init() only
//!                                  much faster.
//!
//! The interfaces above must be called in a particular order.  Write() can
//! only occur in between Init()/Reset() and Rewind().  Next(), Rowkey(), and
//! Compare() can only occur in between Rewind() and Close()/Reset(). i.e.
//!
//!   Init()
//!   for each record: Write()
//!   Rewind()
//!     Rowkey()/Compare()
//!   Next()
//!   Close()
//!
//! Algorithm:
//!
//! Records passed to the sorter via calls to Write() are initially held
//! unsorted in main memory. Assuming the amount of memory used never exceeds
//! a threshold, when Rewind() is called the set of records is sorted using
//! an in-memory merge sort. In this case, no temporary files are required
//! and subsequent calls to Rowkey(), Next() and Compare() read records
//! directly from main memory.
//!
//! If the amount of space used to store records in main memory exceeds the
//! threshold, then the set of records currently in memory are sorted and
//! written to a temporary file in "Packed Memory Array" (PMA) format.
//! A PMA created at this point is known as a "level-0 PMA". Higher levels
//! of PMAs may be created by merging existing PMAs together - for example
//! merging two or more level-0 PMAs together creates a level-1 PMA.
//!
//! The threshold for the amount of main memory to use before flushing
//! records to a PMA is roughly the same as the limit configured for the
//! page-cache of the main database. Specifically, the threshold is set to
//! the value returned by "PRAGMA main.page_size" multiplied by
//! that returned by "PRAGMA main.cache_size", in bytes.
//!
//! If the sorter is running in single-threaded mode, then all PMAs generated
//! are appended to a single temporary file. Or, if the sorter is running in
//! multi-threaded mode then up to (N+1) temporary files may be opened, where
//! N is the configured number of worker threads. In this case, instead of
//! sorting the records and writing the PMA to a temporary file itself, the
//! calling thread usually launches a worker thread to do so. Except, if
//! there are already N worker threads running, the main thread does the work
//! itself.
//!
//! The sorter is running in multi-threaded mode if (a) the library was built
//! with pre-processor symbol SQLITE_MAX_WORKER_THREADS set to a value greater
//! than zero, and (b) worker threads have been enabled at runtime by calling
//! "PRAGMA threads=N" with some value of N greater than 0.
//!
//! When Rewind() is called, any data remaining in memory is flushed to a
//! final PMA. So at this point the data is stored in some number of sorted
//! PMAs within temporary files on disk.
//!
//! If there are fewer than SORTER_MAX_MERGE_COUNT PMAs in total and the
//! sorter is running in single-threaded mode, then these PMAs are merged
//! incrementally as keys are retrieved from the sorter by the VDBE.  The
//! MergeEngine object, described in further detail below, performs this
//! merge.
//!
//! Or, if running in multi-threaded mode, then a background thread is
//! launched to merge the existing PMAs. Once the background thread has
//! merged T bytes of data into a single sorted PMA, the main thread
//! begins reading keys from that PMA while the background thread proceeds
//! with merging the next T bytes of data. And so on.
//!
//! Parameter T is set to half the value of the memory threshold used
//! by Write() above to determine when to create a new PMA.
//!
//! If there are more than SORTER_MAX_MERGE_COUNT PMAs in total when
//! Rewind() is called, then a hierarchy of incremental-merges is used.
//! First, T bytes of data from the first SORTER_MAX_MERGE_COUNT PMAs on
//! disk are merged together. Then T bytes of data from the second set, and
//! so on, such that no operation ever merges more than SORTER_MAX_MERGE_COUNT
//! PMAs at a time. This done is to improve locality.
//!
//! If running in multi-threaded mode and there are more than
//! SORTER_MAX_MERGE_COUNT PMAs on disk when Rewind() is called, then more
//! than one background thread may be created. Specifically, there may be
//! one background thread for each temporary file on disk, and one background
//! thread to merge the output of each of the others to a single PMA for
//! the main thread to read from.
unsafe extern "C" {
    static mut sqlite3Config: Sqlite3Config;
    fn sqlite3_free(__v840: *mut ());
    fn memcpy(__dest: *mut (), __src: *const (), __n: u64) -> *mut ();
    fn memset(__s: *mut (), __c: i32, __n: u64) -> *mut ();
    fn memcmp(__s1: *const (), __s2: *const (), __n: u64) -> i32;
    fn sqlite3OsRead(__v850: *mut sqlite3_file, __v851: *mut (), amt: i32, offset: i64) -> i32;
    fn sqlite3OsWrite(__v854: *mut sqlite3_file, __v855: *const (), amt: i32, offset: i64) -> i32;
    fn sqlite3OsFileControlHint(__v858: *mut sqlite3_file, __v859: i32, __v860: *mut ());
    fn sqlite3OsFetch(id: *mut sqlite3_file, __v862: i64, __v863: i32, __v864: *mut *mut ())
    -> i32;
    fn sqlite3OsUnfetch(__v865: *mut sqlite3_file, __v866: i64, __v867: *mut ()) -> i32;
    fn sqlite3OsOpenMalloc(
        __v868: *mut sqlite3_vfs,
        __v869: *const i8,
        __v870: *mut *mut sqlite3_file,
        __v871: i32,
        __v872: *mut i32,
    ) -> i32;
    fn sqlite3OsCloseFree(__v873: *mut sqlite3_file);
    fn sqlite3BtreeGetPageSize(__v874: *mut Btree) -> i32;
    fn sqlite3BtreeEnter(__v875: *mut Btree);
    fn sqlite3BtreeLeave(__v876: *mut Btree);
    fn sqlite3VdbeRecordUnpack(__v877: i32, __v878: *const (), __v879: *mut UnpackedRecord);
    fn sqlite3VdbeRecordCompare(__v880: i32, __v881: *const (), __v882: *mut UnpackedRecord)
    -> i32;
    fn sqlite3VdbeRecordCompareWithSkip(
        __v883: i32,
        __v884: *const (),
        __v885: *mut UnpackedRecord,
        __v886: i32,
    ) -> i32;
    fn sqlite3VdbeAllocUnpackedRecord(__v887: *mut KeyInfo) -> *mut UnpackedRecord;
    fn sqlite3Malloc(__v888: u64) -> *mut ();
    fn sqlite3MallocZero(__v889: u64) -> *mut ();
    fn sqlite3DbMallocZero(__v890: *mut sqlite3, __v891: u64) -> *mut ();
    fn sqlite3Realloc(__v892: *mut (), __v893: u64) -> *mut ();
    fn sqlite3DbFree(__v894: *mut sqlite3, __v895: *mut ());
    fn sqlite3MallocSize(__v896: *const ()) -> i32;
    fn sqlite3HeapNearlyFull() -> i32;
    fn sqlite3FaultSim(__v897: i32) -> i32;
    fn sqlite3PutVarint(__v898: *mut u8, __v899: u64) -> i32;
    fn sqlite3GetVarint(__v900: *const u8, __v901: *mut u64) -> u8;
    fn sqlite3GetVarint32(__v902: *const u8, __v903: *mut u32) -> u8;
    fn sqlite3VarintLen(v: u64) -> i32;
    fn sqlite3TempInMemory(__v905: *const sqlite3) -> i32;
    fn sqlite3Get8byte(__v906: *const u8) -> u64;
    fn sqlite3ThreadCreate(
        __v907: *mut *mut SQLiteThread,
        __v908: Option<unsafe extern "C-unwind" fn(*mut ()) -> *mut ()>,
        __v909: *mut (),
    ) -> i32;
    fn sqlite3ThreadJoin(__v910: *mut SQLiteThread, __v911: *mut *mut ()) -> i32;
    fn sqlite3VdbeSerialGet(__v912: *const u8, __v913: u32, __v914: *mut sqlite3_value);
    fn sqlite3VdbeMemClearAndResize(pMem: *mut sqlite3_value, n: i32) -> i32;
}

#[repr(C)]
#[derive(Clone, Copy)]
struct sqlite3_file {
    pMethods: *const sqlite3_io_methods,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct sqlite3_io_methods {
    iVersion: i32,
    xClose: Option<unsafe extern "C-unwind" fn(*mut sqlite3_file) -> i32>,
    xRead: Option<unsafe extern "C-unwind" fn(*mut sqlite3_file, *mut (), i32, i64) -> i32>,
    xWrite: Option<unsafe extern "C-unwind" fn(*mut sqlite3_file, *const (), i32, i64) -> i32>,
    xTruncate: Option<unsafe extern "C-unwind" fn(*mut sqlite3_file, i64) -> i32>,
    xSync: Option<unsafe extern "C-unwind" fn(*mut sqlite3_file, i32) -> i32>,
    xFileSize: Option<unsafe extern "C-unwind" fn(*mut sqlite3_file, *mut i64) -> i32>,
    xLock: Option<unsafe extern "C-unwind" fn(*mut sqlite3_file, i32) -> i32>,
    xUnlock: Option<unsafe extern "C-unwind" fn(*mut sqlite3_file, i32) -> i32>,
    xCheckReservedLock: Option<unsafe extern "C-unwind" fn(*mut sqlite3_file, *mut i32) -> i32>,
    xFileControl: Option<unsafe extern "C-unwind" fn(*mut sqlite3_file, i32, *mut ()) -> i32>,
    xSectorSize: Option<unsafe extern "C-unwind" fn(*mut sqlite3_file) -> i32>,
    xDeviceCharacteristics: Option<unsafe extern "C-unwind" fn(*mut sqlite3_file) -> i32>,
    xShmMap:
        Option<unsafe extern "C-unwind" fn(*mut sqlite3_file, i32, i32, i32, *mut *mut ()) -> i32>,
    xShmLock: Option<unsafe extern "C-unwind" fn(*mut sqlite3_file, i32, i32, i32) -> i32>,
    xShmBarrier: Option<unsafe extern "C-unwind" fn(*mut sqlite3_file)>,
    xShmUnmap: Option<unsafe extern "C-unwind" fn(*mut sqlite3_file, i32) -> i32>,
    xFetch: Option<unsafe extern "C-unwind" fn(*mut sqlite3_file, i64, i32, *mut *mut ()) -> i32>,
    xUnfetch: Option<unsafe extern "C-unwind" fn(*mut sqlite3_file, i64, *mut ()) -> i32>,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct sqlite3_vfs {
    iVersion: i32,
    szOsFile: i32,
    mxPathname: i32,
    pNext: *mut sqlite3_vfs,
    zName: *const i8,
    pAppData: *mut (),
    xOpen: Option<
        unsafe extern "C-unwind" fn(
            *mut sqlite3_vfs,
            *const i8,
            *mut sqlite3_file,
            i32,
            *mut i32,
        ) -> i32,
    >,
    xDelete: Option<unsafe extern "C-unwind" fn(*mut sqlite3_vfs, *const i8, i32) -> i32>,
    xAccess: Option<unsafe extern "C-unwind" fn(*mut sqlite3_vfs, *const i8, i32, *mut i32) -> i32>,
    xFullPathname:
        Option<unsafe extern "C-unwind" fn(*mut sqlite3_vfs, *const i8, i32, *mut i8) -> i32>,
    xDlOpen: Option<unsafe extern "C-unwind" fn(*mut sqlite3_vfs, *const i8) -> *mut ()>,
    xDlError: Option<unsafe extern "C-unwind" fn(*mut sqlite3_vfs, i32, *mut i8)>,
    xDlSym: Option<
        unsafe extern "C-unwind" fn(
            *mut sqlite3_vfs,
            *mut (),
            *const i8,
        ) -> Option<unsafe extern "C-unwind" fn()>,
    >,
    xDlClose: Option<unsafe extern "C-unwind" fn(*mut sqlite3_vfs, *mut ())>,
    xRandomness: Option<unsafe extern "C-unwind" fn(*mut sqlite3_vfs, i32, *mut i8) -> i32>,
    xSleep: Option<unsafe extern "C-unwind" fn(*mut sqlite3_vfs, i32) -> i32>,
    xCurrentTime: Option<unsafe extern "C-unwind" fn(*mut sqlite3_vfs, *mut f64) -> i32>,
    xGetLastError: Option<unsafe extern "C-unwind" fn(*mut sqlite3_vfs, i32, *mut i8) -> i32>,
    xCurrentTimeInt64: Option<unsafe extern "C-unwind" fn(*mut sqlite3_vfs, *mut i64) -> i32>,
    xSetSystemCall: Option<
        unsafe extern "C-unwind" fn(
            *mut sqlite3_vfs,
            *const i8,
            Option<unsafe extern "C-unwind" fn()>,
        ) -> i32,
    >,
    xGetSystemCall: Option<
        unsafe extern "C-unwind" fn(
            *mut sqlite3_vfs,
            *const i8,
        ) -> Option<unsafe extern "C-unwind" fn()>,
    >,
    xNextSystemCall: Option<unsafe extern "C-unwind" fn(*mut sqlite3_vfs, *const i8) -> *const i8>,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct sqlite3_mutex {}

#[repr(C)]
#[derive(Clone, Copy)]
struct sqlite3_mem_methods {
    xMalloc: Option<unsafe extern "C-unwind" fn(i32) -> *mut ()>,
    xFree: Option<unsafe extern "C-unwind" fn(*mut ())>,
    xRealloc: Option<unsafe extern "C-unwind" fn(*mut (), i32) -> *mut ()>,
    xSize: Option<unsafe extern "C-unwind" fn(*mut ()) -> i32>,
    xRoundup: Option<unsafe extern "C-unwind" fn(i32) -> i32>,
    xInit: Option<unsafe extern "C-unwind" fn(*mut ()) -> i32>,
    xShutdown: Option<unsafe extern "C-unwind" fn(*mut ())>,
    pAppData: *mut (),
}

#[repr(C)]
#[derive(Clone, Copy)]
struct sqlite3_module {
    iVersion: i32,
    xCreate: Option<
        unsafe extern "C-unwind" fn(
            *mut sqlite3,
            *mut (),
            i32,
            *const *const i8,
            *mut *mut sqlite3_vtab,
            *mut *mut i8,
        ) -> i32,
    >,
    xConnect: Option<
        unsafe extern "C-unwind" fn(
            *mut sqlite3,
            *mut (),
            i32,
            *const *const i8,
            *mut *mut sqlite3_vtab,
            *mut *mut i8,
        ) -> i32,
    >,
    xBestIndex:
        Option<unsafe extern "C-unwind" fn(*mut sqlite3_vtab, *mut sqlite3_index_info) -> i32>,
    xDisconnect: Option<unsafe extern "C-unwind" fn(*mut sqlite3_vtab) -> i32>,
    xDestroy: Option<unsafe extern "C-unwind" fn(*mut sqlite3_vtab) -> i32>,
    xOpen: Option<
        unsafe extern "C-unwind" fn(*mut sqlite3_vtab, *mut *mut sqlite3_vtab_cursor) -> i32,
    >,
    xClose: Option<unsafe extern "C-unwind" fn(*mut sqlite3_vtab_cursor) -> i32>,
    xFilter: Option<
        unsafe extern "C-unwind" fn(
            *mut sqlite3_vtab_cursor,
            i32,
            *const i8,
            i32,
            *mut *mut sqlite3_value,
        ) -> i32,
    >,
    xNext: Option<unsafe extern "C-unwind" fn(*mut sqlite3_vtab_cursor) -> i32>,
    xEof: Option<unsafe extern "C-unwind" fn(*mut sqlite3_vtab_cursor) -> i32>,
    xColumn: Option<
        unsafe extern "C-unwind" fn(*mut sqlite3_vtab_cursor, *mut sqlite3_context, i32) -> i32,
    >,
    xRowid: Option<unsafe extern "C-unwind" fn(*mut sqlite3_vtab_cursor, *mut i64) -> i32>,
    xUpdate: Option<
        unsafe extern "C-unwind" fn(
            *mut sqlite3_vtab,
            i32,
            *mut *mut sqlite3_value,
            *mut i64,
        ) -> i32,
    >,
    xBegin: Option<unsafe extern "C-unwind" fn(*mut sqlite3_vtab) -> i32>,
    xSync: Option<unsafe extern "C-unwind" fn(*mut sqlite3_vtab) -> i32>,
    xCommit: Option<unsafe extern "C-unwind" fn(*mut sqlite3_vtab) -> i32>,
    xRollback: Option<unsafe extern "C-unwind" fn(*mut sqlite3_vtab) -> i32>,
    xFindFunction: Option<
        unsafe extern "C-unwind" fn(
            *mut sqlite3_vtab,
            i32,
            *const i8,
            *mut Option<
                unsafe extern "C-unwind" fn(*mut sqlite3_context, i32, *mut *mut sqlite3_value),
            >,
            *mut *mut (),
        ) -> i32,
    >,
    xRename: Option<unsafe extern "C-unwind" fn(*mut sqlite3_vtab, *const i8) -> i32>,
    xSavepoint: Option<unsafe extern "C-unwind" fn(*mut sqlite3_vtab, i32) -> i32>,
    xRelease: Option<unsafe extern "C-unwind" fn(*mut sqlite3_vtab, i32) -> i32>,
    xRollbackTo: Option<unsafe extern "C-unwind" fn(*mut sqlite3_vtab, i32) -> i32>,
    xShadowName: Option<unsafe extern "C-unwind" fn(*const i8) -> i32>,
    xIntegrity: Option<
        unsafe extern "C-unwind" fn(
            *mut sqlite3_vtab,
            *const i8,
            *const i8,
            i32,
            *mut *mut i8,
        ) -> i32,
    >,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct sqlite3_index_info {
    nConstraint: i32,
    aConstraint: *mut sqlite3_index_constraint,
    nOrderBy: i32,
    aOrderBy: *mut sqlite3_index_orderby,
    aConstraintUsage: *mut sqlite3_index_constraint_usage,
    idxNum: i32,
    idxStr: *mut i8,
    needToFreeIdxStr: i32,
    orderByConsumed: i32,
    estimatedCost: f64,
    estimatedRows: i64,
    idxFlags: i32,
    colUsed: u64,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct sqlite3_vtab {
    pModule: *const sqlite3_module,
    nRef: i32,
    zErrMsg: *mut i8,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct sqlite3_vtab_cursor {
    pVtab: *mut sqlite3_vtab,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct sqlite3_mutex_methods {
    xMutexInit: Option<unsafe extern "C-unwind" fn() -> i32>,
    xMutexEnd: Option<unsafe extern "C-unwind" fn() -> i32>,
    xMutexAlloc: Option<unsafe extern "C-unwind" fn(i32) -> *mut sqlite3_mutex>,
    xMutexFree: Option<unsafe extern "C-unwind" fn(*mut sqlite3_mutex)>,
    xMutexEnter: Option<unsafe extern "C-unwind" fn(*mut sqlite3_mutex)>,
    xMutexTry: Option<unsafe extern "C-unwind" fn(*mut sqlite3_mutex) -> i32>,
    xMutexLeave: Option<unsafe extern "C-unwind" fn(*mut sqlite3_mutex)>,
    xMutexHeld: Option<unsafe extern "C-unwind" fn(*mut sqlite3_mutex) -> i32>,
    xMutexNotheld: Option<unsafe extern "C-unwind" fn(*mut sqlite3_mutex) -> i32>,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct sqlite3_pcache_page {
    pBuf: *mut (),
    pExtra: *mut (),
}

#[repr(C)]
#[derive(Clone, Copy)]
struct sqlite3_pcache_methods2 {
    iVersion: i32,
    pArg: *mut (),
    xInit: Option<unsafe extern "C-unwind" fn(*mut ()) -> i32>,
    xShutdown: Option<unsafe extern "C-unwind" fn(*mut ())>,
    xCreate: Option<unsafe extern "C-unwind" fn(i32, i32, i32) -> *mut sqlite3_pcache>,
    xCachesize: Option<unsafe extern "C-unwind" fn(*mut sqlite3_pcache, i32)>,
    xPagecount: Option<unsafe extern "C-unwind" fn(*mut sqlite3_pcache) -> i32>,
    xFetch: Option<
        unsafe extern "C-unwind" fn(*mut sqlite3_pcache, u32, i32) -> *mut sqlite3_pcache_page,
    >,
    xUnpin: Option<unsafe extern "C-unwind" fn(*mut sqlite3_pcache, *mut sqlite3_pcache_page, i32)>,
    xRekey: Option<
        unsafe extern "C-unwind" fn(*mut sqlite3_pcache, *mut sqlite3_pcache_page, u32, u32),
    >,
    xTruncate: Option<unsafe extern "C-unwind" fn(*mut sqlite3_pcache, u32)>,
    xDestroy: Option<unsafe extern "C-unwind" fn(*mut sqlite3_pcache)>,
    xShrink: Option<unsafe extern "C-unwind" fn(*mut sqlite3_pcache)>,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct sqlite3_index_constraint {
    iColumn: i32,
    op: u8,
    usable: u8,
    iTermOffset: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct sqlite3_index_orderby {
    iColumn: i32,
    desc: u8,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct sqlite3_index_constraint_usage {
    argvIndex: i32,
    omit: u8,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct Hash {
    htsize: u32,
    count: u32,
    first: *mut HashElem,
    ht: *mut _ht,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct sqlite3_pcache {}

#[repr(C)]
#[derive(Clone, Copy)]
struct HashElem {
    next: *mut HashElem,
    prev: *mut HashElem,
    data: *mut (),
    pKey: *const i8,
    h: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct BusyHandler {
    xBusyHandler: Option<unsafe extern "C-unwind" fn(*mut (), i32) -> i32>,
    pBusyArg: *mut (),
    nBusy: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct SubrtnSig {
    selId: i32,
    bComplete: u8,
    zAff: *mut i8,
    iTable: i32,
    iAddr: i32,
    regReturn: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct VdbeOp {
    opcode: u8,
    p4type: i8,
    p5: u16,
    p1: i32,
    p2: i32,
    p3: i32,
    p4: p4union,
    zComment: *mut i8,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _ht {
    count: u32,
    chain: *mut HashElem,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct SubProgram {
    aOp: *mut VdbeOp,
    nOp: i32,
    nMem: i32,
    nCsr: i32,
    aOnce: *mut u8,
    token: *mut (),
    pNext: *mut SubProgram,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct Db {
    zDbSName: *mut i8,
    pBt: *mut Btree,
    safety_level: u8,
    bSyncSet: u8,
    pSchema: *mut Schema,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct Schema {
    schema_cookie: i32,
    iGeneration: i32,
    tblHash: Hash,
    idxHash: Hash,
    trigHash: Hash,
    fkeyHash: Hash,
    pSeqTab: *mut Table,
    file_format: u8,
    enc: u8,
    schemaFlags: u16,
    cache_size: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct Lookaside {
    bDisable: u32,
    sz: u16,
    szTrue: u16,
    bMalloced: u8,
    nSlot: u32,
    anStat: [u32; 3],
    pInit: *mut LookasideSlot,
    pFree: *mut LookasideSlot,
    pSmallInit: *mut LookasideSlot,
    pSmallFree: *mut LookasideSlot,
    pMiddle: *mut (),
    pStart: *mut (),
    pEnd: *mut (),
    pTrueEnd: *mut (),
}

#[repr(C)]
#[derive(Clone, Copy)]
struct LookasideSlot {
    pNext: *mut LookasideSlot,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct sqlite3 {
    pVfs: *mut sqlite3_vfs,
    pVdbe: *mut Vdbe,
    pDfltColl: *mut CollSeq,
    mutex: *mut sqlite3_mutex,
    aDb: *mut Db,
    nDb: i32,
    mDbFlags: u32,
    flags: u64,
    lastRowid: i64,
    szMmap: i64,
    nSchemaLock: u32,
    openFlags: u32,
    errCode: i32,
    errByteOffset: i32,
    errMask: i32,
    iSysErrno: i32,
    dbOptFlags: u32,
    enc: u8,
    autoCommit: u8,
    temp_store: u8,
    mallocFailed: u8,
    bBenignMalloc: u8,
    dfltLockMode: u8,
    nextAutovac: i8,
    suppressErr: u8,
    vtabOnConflict: u8,
    isTransactionSavepoint: u8,
    mTrace: u8,
    noSharedCache: u8,
    nSqlExec: u8,
    eOpenState: u8,
    nFpDigit: u8,
    nextPagesize: i32,
    nChange: i64,
    nTotalChange: i64,
    aLimit: [i32; 15],
    nMaxSorterMmap: i32,
    init: sqlite3InitInfo,
    nVdbeActive: i32,
    nVdbeRead: i32,
    nVdbeWrite: i32,
    nVdbeExec: i32,
    nVDestroy: i32,
    nExtension: i32,
    aExtension: *mut *mut (),
    trace: __SlateRecord174,
    pTraceArg: *mut (),
    xProfile: Option<unsafe extern "C-unwind" fn(*mut (), *const i8, u64)>,
    pProfileArg: *mut (),
    pCommitArg: *mut (),
    xCommitCallback: Option<unsafe extern "C-unwind" fn(*mut ()) -> i32>,
    pRollbackArg: *mut (),
    xRollbackCallback: Option<unsafe extern "C-unwind" fn(*mut ())>,
    pUpdateArg: *mut (),
    xUpdateCallback: Option<unsafe extern "C-unwind" fn(*mut (), i32, *const i8, *const i8, i64)>,
    pAutovacPagesArg: *mut (),
    xAutovacDestr: Option<unsafe extern "C-unwind" fn(*mut ())>,
    xAutovacPages: Option<unsafe extern "C-unwind" fn(*mut (), *const i8, u32, u32, u32) -> u32>,
    pParse: *mut Parse,
    xWalCallback: Option<unsafe extern "C-unwind" fn(*mut (), *mut sqlite3, *const i8, i32) -> i32>,
    pWalArg: *mut (),
    xCollNeeded: Option<unsafe extern "C-unwind" fn(*mut (), *mut sqlite3, i32, *const i8)>,
    xCollNeeded16: Option<unsafe extern "C-unwind" fn(*mut (), *mut sqlite3, i32, *const ())>,
    pCollNeededArg: *mut (),
    pErr: *mut sqlite3_value,
    u1: __SlateRecord175,
    lookaside: Lookaside,
    xAuth: Option<
        unsafe extern "C-unwind" fn(
            *mut (),
            i32,
            *const i8,
            *const i8,
            *const i8,
            *const i8,
        ) -> i32,
    >,
    pAuthArg: *mut (),
    xProgress: Option<unsafe extern "C-unwind" fn(*mut ()) -> i32>,
    pProgressArg: *mut (),
    nProgressOps: u32,
    nVTrans: i32,
    aModule: Hash,
    pVtabCtx: *mut VtabCtx,
    aVTrans: *mut *mut VTable,
    pDisconnect: *mut VTable,
    aFunc: Hash,
    aCollSeq: Hash,
    busyHandler: BusyHandler,
    aDbStatic: [Db; 2],
    pSavepoint: *mut Savepoint,
    nAnalysisLimit: i32,
    busyTimeout: i32,
    nSavepoint: i32,
    nStatement: i32,
    nDeferredCons: i64,
    nDeferredImmCons: i64,
    pnBytesFreed: *mut i32,
    pDbData: *mut DbClientData,
    nSpill: u64,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct FuncDef {
    nArg: i16,
    funcFlags: u32,
    pUserData: *mut (),
    pNext: *mut FuncDef,
    xSFunc: Option<unsafe extern "C-unwind" fn(*mut sqlite3_context, i32, *mut *mut sqlite3_value)>,
    xFinalize: Option<unsafe extern "C-unwind" fn(*mut sqlite3_context)>,
    xValue: Option<unsafe extern "C-unwind" fn(*mut sqlite3_context)>,
    xInverse:
        Option<unsafe extern "C-unwind" fn(*mut sqlite3_context, i32, *mut *mut sqlite3_value)>,
    zName: *const i8,
    u: __SlateRecord176,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct FuncDestructor {
    nRef: i32,
    xDestroy: Option<unsafe extern "C-unwind" fn(*mut ())>,
    pUserData: *mut (),
}

#[repr(C)]
#[derive(Clone, Copy)]
struct Savepoint {
    zName: *mut i8,
    nDeferredCons: i64,
    nDeferredImmCons: i64,
    pNext: *mut Savepoint,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct Module {
    pModule: *const sqlite3_module,
    zName: *const i8,
    nRefModule: i32,
    pAux: *mut (),
    xDestroy: Option<unsafe extern "C-unwind" fn(*mut ())>,
    pEpoTab: *mut Table,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct Column {
    zCnName: *mut i8,
    __slate_bits_0: __slate_bits::__SlateBits75U0,
    affinity: i8,
    szEst: u8,
    hName: u8,
    iDflt: u16,
    colFlags: u16,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct CollSeq {
    zName: *mut i8,
    enc: u8,
    pUser: *mut (),
    xCmp: Option<unsafe extern "C-unwind" fn(*mut (), i32, *const (), i32, *const ()) -> i32>,
    xDel: Option<unsafe extern "C-unwind" fn(*mut ())>,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct VTable {
    db: *mut sqlite3,
    pMod: *mut Module,
    pVtab: *mut sqlite3_vtab,
    nRef: i32,
    bConstraint: u8,
    bAllSchemas: u8,
    eVtabRisk: u8,
    iSavepoint: i32,
    pNext: *mut VTable,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct Table {
    zName: *mut i8,
    aCol: *mut Column,
    pIndex: *mut Index,
    zColAff: *mut i8,
    pCheck: *mut ExprList,
    tnum: u32,
    nTabRef: u32,
    tabFlags: u32,
    iPKey: i16,
    nCol: i16,
    nNVCol: i16,
    nRowLogEst: i16,
    szTabRow: i16,
    keyConf: u8,
    eTabType: u8,
    u: __SlateRecord177,
    pTrigger: *mut Trigger,
    pSchema: *mut Schema,
    aHx: [u8; 16],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct FKey {
    pFrom: *mut Table,
    pNextFrom: *mut FKey,
    zTo: *mut i8,
    pNextTo: *mut FKey,
    pPrevTo: *mut FKey,
    nCol: i32,
    isDeferred: u8,
    aAction: [u8; 2],
    apTrigger: [*mut Trigger; 2],
    aCol: [sColMap; 0],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct KeyInfo {
    nRef: u32,
    enc: u8,
    nKeyField: u16,
    nAllField: u16,
    db: *mut sqlite3,
    aSortFlags: *mut u8,
    aColl: [*mut CollSeq; 0],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct UnpackedRecord {
    pKeyInfo: *mut KeyInfo,
    aMem: *mut sqlite3_value,
    u: __SlateRecord182,
    n: i32,
    nField: u16,
    default_rc: i8,
    errCode: u8,
    r1: i8,
    r2: i8,
    eqSeen: u8,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct Index {
    zName: *mut i8,
    aiColumn: *mut i16,
    aiRowLogEst: *mut i16,
    pTable: *mut Table,
    zColAff: *mut i8,
    pNext: *mut Index,
    pSchema: *mut Schema,
    aSortOrder: *mut u8,
    azColl: *mut *const i8,
    pPartIdxWhere: *mut Expr,
    aColExpr: *mut ExprList,
    tnum: u32,
    szIdxRow: i16,
    nKeyCol: u16,
    nColumn: u16,
    onError: u8,
    __slate_bits_0: __slate_bits::__SlateBits99U0,
    colNotIdxed: u64,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct Token {
    z: *const i8,
    n: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct AggInfo {
    directMode: u8,
    useSortingIdx: u8,
    nSortingColumn: u32,
    sortingIdx: i32,
    sortingIdxPTab: i32,
    iFirstReg: i32,
    pGroupBy: *mut ExprList,
    aCol: *mut AggInfo_col,
    nColumn: i32,
    nAccumulator: i32,
    aFunc: *mut AggInfo_func,
    nFunc: i32,
    selId: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct Expr {
    op: u8,
    affExpr: i8,
    op2: u8,
    flags: u32,
    u: __SlateRecord186,
    pLeft: *mut Expr,
    pRight: *mut Expr,
    x: __SlateRecord187,
    nHeight: i32,
    iTable: i32,
    iColumn: i16,
    iAgg: i16,
    w: __SlateRecord188,
    pAggInfo: *mut AggInfo,
    y: __SlateRecord189,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct ExprList {
    nExpr: i32,
    nAlloc: i32,
    a: [ExprList_item; 0],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct IdList {
    nId: i32,
    a: [IdList_item; 0],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct Subquery {
    pSelect: *mut Select,
    addrFillSub: i32,
    regReturn: i32,
    regResult: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct RenameToken {}

#[repr(C)]
#[derive(Clone, Copy)]
struct SrcItem {
    zName: *mut i8,
    zAlias: *mut i8,
    pSTab: *mut Table,
    fg: __SlateRecord196,
    iCursor: i32,
    colUsed: u64,
    u1: __SlateRecord197,
    u2: __SlateRecord198,
    u3: __SlateRecord199,
    u4: __SlateRecord200,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct SrcList {
    nSrc: i32,
    nAlloc: u32,
    a: [SrcItem; 0],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct Upsert {
    pUpsertTarget: *mut ExprList,
    pUpsertTargetWhere: *mut Expr,
    pUpsertSet: *mut ExprList,
    pUpsertWhere: *mut Expr,
    pNextUpsert: *mut Upsert,
    isDoUpdate: u8,
    isDup: u8,
    pToFree: *mut (),
    pUpsertIdx: *mut Index,
    pUpsertSrc: *mut SrcList,
    regData: i32,
    iDataCur: i32,
    iIdxCur: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct SQLiteThread {}

#[repr(C)]
#[derive(Clone, Copy)]
struct Select {
    op: u8,
    nSelectRow: i16,
    selFlags: u32,
    iLimit: i32,
    iOffset: i32,
    selId: u32,
    pEList: *mut ExprList,
    pSrc: *mut SrcList,
    pWhere: *mut Expr,
    pGroupBy: *mut ExprList,
    pHaving: *mut Expr,
    pOrderBy: *mut ExprList,
    pPrior: *mut Select,
    pNext: *mut Select,
    pLimit: *mut Expr,
    pWith: *mut With,
    pWin: *mut Window,
    pWinDefn: *mut Window,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct AutoincInfo {
    pNext: *mut AutoincInfo,
    pTab: *mut Table,
    iDb: i32,
    regCtr: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct TriggerPrg {
    pTrigger: *mut Trigger,
    pNext: *mut TriggerPrg,
    pProgram: *mut SubProgram,
    orconf: i32,
    aColmask: [u32; 2],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct IndexedExpr {
    pExpr: *mut Expr,
    iDataCur: i32,
    iIdxCur: i32,
    iIdxCol: i32,
    bMaybeNullRow: u8,
    aff: u8,
    pIENext: *mut IndexedExpr,
    zIdxName: *const i8,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct TableLock {}

#[repr(C)]
#[derive(Clone, Copy)]
struct ParseCleanup {
    pNext: *mut ParseCleanup,
    pPtr: *mut (),
    xCleanup: Option<unsafe extern "C-unwind" fn(*mut sqlite3, *mut ())>,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct Parse {
    db: *mut sqlite3,
    zErrMsg: *mut i8,
    pVdbe: *mut Vdbe,
    rc: i32,
    nQueryLoop: i16,
    nested: u8,
    nTempReg: u8,
    isMultiWrite: u8,
    disableLookaside: u8,
    prepFlags: u8,
    withinRJSubrtn: u8,
    mSubrtnSig: u8,
    eTriggerOp: u8,
    eOrconf: u8,
    __slate_bits_0: __slate_bits::__SlateBits111U0,
    nRangeReg: i32,
    iRangeReg: i32,
    nErr: i32,
    nTab: i32,
    nMem: i32,
    iSelfTab: i32,
    nNestSel: i32,
    nLabel: i32,
    nLabelAlloc: i32,
    aLabel: *mut i32,
    pConstExpr: *mut ExprList,
    pIdxEpr: *mut IndexedExpr,
    pIdxPartExpr: *mut IndexedExpr,
    writeMask: u32,
    cookieMask: u32,
    nMaxArg: i32,
    nSelect: i32,
    nProgressSteps: u32,
    nTableLock: i32,
    pToplevel: *mut Parse,
    pTriggerTab: *mut Table,
    pTriggerPrg: *mut TriggerPrg,
    pCleanup: *mut ParseCleanup,
    aTempReg: [i32; 8],
    pOuterParse: *mut Parse,
    sNameToken: Token,
    oldmask: u32,
    newmask: u32,
    u1: __SlateRecord202,
    pAinc: *mut AutoincInfo,
    aTableLock: *mut TableLock,
    sLastToken: Token,
    nVar: i16,
    aVnbmc: [u64; 2],
    iPkSortOrder: u8,
    explain: u8,
    eParseMode: u8,
    nVtabLock: i32,
    nHeight: i32,
    addrExplain: i32,
    pVList: *mut i32,
    pReprepare: *mut Vdbe,
    zTail: *const i8,
    pNewTable: *mut Table,
    pNewIndex: *mut Index,
    pNewTrigger: *mut Trigger,
    zAuthContext: *const i8,
    sArg: Token,
    apVtabLock: *mut *mut Table,
    pWith: *mut With,
    pRename: *mut RenameToken,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct Trigger {
    zName: *mut i8,
    table: *mut i8,
    op: u8,
    tr_tm: u8,
    bReturning: u8,
    pWhen: *mut Expr,
    pColumns: *mut IdList,
    pSchema: *mut Schema,
    pTabSchema: *mut Schema,
    step_list: *mut TriggerStep,
    pNext: *mut Trigger,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct TriggerStep {
    op: u8,
    orconf: u8,
    pTrig: *mut Trigger,
    pSelect: *mut Select,
    pSrc: *mut SrcList,
    pWhere: *mut Expr,
    pExprList: *mut ExprList,
    pIdList: *mut IdList,
    pUpsert: *mut Upsert,
    zSpan: *mut i8,
    pNext: *mut TriggerStep,
    pLast: *mut TriggerStep,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct Returning {
    pParse: *mut Parse,
    pReturnEL: *mut ExprList,
    retTrig: Trigger,
    retTStep: TriggerStep,
    iRetCur: i32,
    nRetCol: i32,
    iRetReg: i32,
    zName: [i8; 40],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct Sqlite3Config {
    bMemstat: i32,
    bCoreMutex: u8,
    bFullMutex: u8,
    bOpenUri: u8,
    bUseCis: u8,
    bSmallMalloc: u8,
    bExtraSchemaChecks: u8,
    mxStrlen: i32,
    neverCorrupt: i32,
    szLookaside: i32,
    nLookaside: i32,
    nStmtSpill: i32,
    m: sqlite3_mem_methods,
    mutex: sqlite3_mutex_methods,
    pcache2: sqlite3_pcache_methods2,
    pHeap: *mut (),
    nHeap: i32,
    mnReq: i32,
    mxReq: i32,
    szMmap: i64,
    mxMmap: i64,
    pPage: *mut (),
    szPage: i32,
    nPage: i32,
    mxParserStack: i32,
    sharedCacheEnabled: i32,
    szPma: u32,
    isInit: i32,
    inProgress: i32,
    isMutexInit: i32,
    isMallocInit: i32,
    isPCacheInit: i32,
    nRefInitMutex: i32,
    pInitMutex: *mut sqlite3_mutex,
    xLog: Option<unsafe extern "C-unwind" fn(*mut (), i32, *const i8)>,
    pLogArg: *mut (),
    mxMemdbSize: i64,
    xTestCallback: Option<unsafe extern "C-unwind" fn(i32) -> i32>,
    bLocaltimeFault: i32,
    xAltLocaltime: Option<unsafe extern "C-unwind" fn(*const (), *mut ()) -> i32>,
    iOnceResetThreshold: i32,
    szSorterRef: u32,
    iPrngSeed: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct Cte {
    zName: *mut i8,
    pCols: *mut ExprList,
    pSelect: *mut Select,
    zCteErr: *const i8,
    pUse: *mut CteUse,
    eM10d: u8,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct VtabCtx {}

#[repr(C)]
#[derive(Clone, Copy)]
struct With {
    nCte: i32,
    bView: i32,
    pOuter: *mut With,
    a: [Cte; 0],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct CteUse {
    nUse: i32,
    addrM9e: i32,
    regRtn: i32,
    iCur: i32,
    nRowEst: i16,
    eM10d: u8,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct Btree {}

#[repr(C)]
#[derive(Clone, Copy)]
struct BtCursor {}

#[repr(C)]
#[derive(Clone, Copy)]
struct DbClientData {
    pNext: *mut DbClientData,
    pData: *mut (),
    xDestructor: Option<unsafe extern "C-unwind" fn(*mut ())>,
    zName: [i8; 0],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct Window {
    zName: *mut i8,
    zBase: *mut i8,
    pPartition: *mut ExprList,
    pOrderBy: *mut ExprList,
    eFrmType: u8,
    eStart: u8,
    eEnd: u8,
    bImplicitFrame: u8,
    eExclude: u8,
    pStart: *mut Expr,
    pEnd: *mut Expr,
    ppThis: *mut *mut Window,
    pNextWin: *mut Window,
    pFilter: *mut Expr,
    pWFunc: *mut FuncDef,
    iEphCsr: i32,
    regAccum: i32,
    regResult: i32,
    csrApp: i32,
    regApp: i32,
    regPart: i32,
    pOwner: *mut Expr,
    nBufferCol: i32,
    iArgCol: i32,
    regOne: i32,
    regStartRowid: i32,
    regEndRowid: i32,
    bExprArgs: u8,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct VdbeCursor {
    eCurType: u8,
    iDb: i8,
    nullRow: u8,
    deferredMoveto: u8,
    isTable: u8,
    __slate_bits_0: __slate_bits::__SlateBits214U0,
    seekHit: u16,
    ub: __SlateRecord216,
    seqCount: i64,
    cacheStatus: u32,
    seekResult: i32,
    pAltCursor: *mut VdbeCursor,
    uc: __SlateRecord217,
    pKeyInfo: *mut KeyInfo,
    iHdrOffset: u32,
    pgnoRoot: u32,
    nField: i16,
    nHdrParsed: u16,
    movetoTarget: i64,
    aOffset: *mut u32,
    aRow: *const u8,
    payloadSize: u32,
    szRow: u32,
    pCache: *mut VdbeTxtBlbCache,
    aType: [u32; 0],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct VdbeTxtBlbCache {
    pCValue: *mut i8,
    iOffset: i64,
    iCol: i32,
    cacheStatus: u32,
    colCacheCtr: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
union p4union {
    i: i32,
    p: *mut (),
    z: *mut i8,
    pFunc: *mut FuncDef,
    pCtx: *mut sqlite3_context,
    pColl: *mut CollSeq,
    pMem: *mut sqlite3_value,
    pVtab: *mut VTable,
    pKeyInfo: *mut KeyInfo,
    ai: *mut u32,
    pProgram: *mut SubProgram,
    pTab: *mut Table,
    pSubrtnSig: *mut SubrtnSig,
    pIdx: *mut Index,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct sqlite3InitInfo {
    newTnum: u32,
    iDb: u8,
    busy: u8,
    __slate_bits_0: __slate_bits::__SlateBits173U0,
    azInit: *mut *const i8,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord174 {
    xLegacy: Option<unsafe extern "C-unwind" fn(*mut (), *const i8)>,
    xV2: Option<unsafe extern "C-unwind" fn(u32, *mut (), *mut (), *mut ()) -> i32>,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord175 {
    isInterrupted: i32,
    notUsed1: f64,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord176 {
    pHash: *mut FuncDef,
    pDestructor: *mut FuncDestructor,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord177 {
    tab: __SlateRecord178,
    view: __SlateRecord179,
    vtab: __SlateRecord180,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord178 {
    addColOffset: i32,
    pFKey: *mut FKey,
    pDfltList: *mut ExprList,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord179 {
    pSelect: *mut Select,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord180 {
    nArg: i32,
    azArg: *mut *mut i8,
    p: *mut VTable,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct sColMap {
    iFrom: i32,
    zCol: *mut i8,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord182 {
    z: *mut i8,
    i: i64,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct AggInfo_col {
    pTab: *mut Table,
    pCExpr: *mut Expr,
    iTable: i32,
    iColumn: i32,
    iSorterColumn: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct AggInfo_func {
    pFExpr: *mut Expr,
    pFunc: *mut FuncDef,
    iDistinct: i32,
    iDistAddr: i32,
    iOBTab: i32,
    bOBPayload: u8,
    bOBUnique: u8,
    bUseSubtype: u8,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord186 {
    zToken: *mut i8,
    iValue: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord187 {
    pList: *mut ExprList,
    pSelect: *mut Select,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord188 {
    iJoin: i32,
    iOfst: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord189 {
    pTab: *mut Table,
    pWin: *mut Window,
    nReg: i32,
    sub: __SlateRecord190,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord190 {
    iAddr: i32,
    regReturn: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct ExprList_item {
    pExpr: *mut Expr,
    zEName: *mut i8,
    fg: __SlateRecord192,
    u: __SlateRecord193,
}

#[repr(C, align(4))]
#[derive(Clone, Copy)]
struct __SlateRecord192 {
    sortFlags: u8,
    __slate_bits_0: __slate_bits::__SlateBits192U0,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord193 {
    x: __SlateRecord194,
    iConstExprReg: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord194 {
    iOrderByCol: u16,
    iAlias: u16,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct IdList_item {
    zName: *mut i8,
}

#[repr(C, align(4))]
#[derive(Clone, Copy)]
struct __SlateRecord196 {
    jointype: u8,
    __slate_bits_0: __slate_bits::__SlateBits196U0,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord197 {
    zIndexedBy: *mut i8,
    pFuncArg: *mut ExprList,
    nRow: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord198 {
    pIBIndex: *mut Index,
    pCteUse: *mut CteUse,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord199 {
    pOn: *mut Expr,
    pUsing: *mut IdList,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord200 {
    pSchema: *mut Schema,
    zDatabase: *mut i8,
    pSubq: *mut Subquery,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord202 {
    cr: __SlateRecord203,
    d: __SlateRecord204,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord203 {
    addrCrTab: i32,
    regRowid: i32,
    regRoot: i32,
    constraintName: Token,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord204 {
    pReturning: *mut Returning,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct VdbeFrame {
    v: *mut Vdbe,
    pParent: *mut VdbeFrame,
    aOp: *mut VdbeOp,
    aMem: *mut sqlite3_value,
    apCsr: *mut *mut VdbeCursor,
    aOnce: *mut u8,
    token: *mut (),
    lastRowid: i64,
    pAuxData: *mut AuxData,
    nCursor: i32,
    pc: i32,
    nOp: i32,
    nMem: i32,
    nChildMem: i32,
    nChildCsr: i32,
    nChange: i64,
    nDbChange: i64,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct sqlite3_value {
    u: MemValue,
    z: *mut i8,
    n: i32,
    flags: u16,
    enc: u8,
    eSubtype: u8,
    db: *mut sqlite3,
    szMalloc: i32,
    uTemp: u32,
    zMalloc: *mut i8,
    xDel: Option<unsafe extern "C-unwind" fn(*mut ())>,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct AuxData {
    iAuxOp: i32,
    iAuxArg: i32,
    pAux: *mut (),
    xDeleteAux: Option<unsafe extern "C-unwind" fn(*mut ())>,
    pNextAux: *mut AuxData,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct sqlite3_context {
    pOut: *mut sqlite3_value,
    pFunc: *mut FuncDef,
    pMem: *mut sqlite3_value,
    pVdbe: *mut Vdbe,
    iOp: i32,
    isError: i32,
    enc: u8,
    skipFlag: u8,
    argc: u16,
    argv: [*mut sqlite3_value; 0],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct Vdbe {
    db: *mut sqlite3,
    ppVPrev: *mut *mut Vdbe,
    pVNext: *mut Vdbe,
    pParse: *mut Parse,
    nVar: i16,
    nMem: i32,
    nCursor: i32,
    cacheCtr: u32,
    pc: i32,
    rc: i32,
    nChange: i64,
    iStatement: i32,
    iCurrentTime: i64,
    nFkConstraint: i64,
    nStmtDefCons: i64,
    nStmtDefImmCons: i64,
    aMem: *mut sqlite3_value,
    apArg: *mut *mut sqlite3_value,
    apCsr: *mut *mut VdbeCursor,
    aVar: *mut sqlite3_value,
    aOp: *mut VdbeOp,
    nOp: i32,
    nOpAlloc: i32,
    aColName: *mut sqlite3_value,
    pResultRow: *mut sqlite3_value,
    zErrMsg: *mut i8,
    pVList: *mut i32,
    startTime: i64,
    nResColumn: u16,
    nResAlloc: u16,
    errorAction: u8,
    minWriteFileFormat: u8,
    prepFlags: u8,
    eVdbeState: u8,
    __slate_bits_0: __slate_bits::__SlateBits162U0,
    btreeMask: u32,
    lockMask: u32,
    aCounter: [u32; 9],
    zSql: *mut i8,
    pFree: *mut (),
    pFrame: *mut VdbeFrame,
    pDelFrame: *mut VdbeFrame,
    nFrame: i32,
    expmask: u32,
    smimask: u32,
    pProgram: *mut SubProgram,
    pAuxData: *mut AuxData,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord216 {
    pBtx: *mut Btree,
    aAltMap: *mut u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord217 {
    pCursor: *mut BtCursor,
    pVCur: *mut sqlite3_vtab_cursor,
    pSorter: *mut VdbeSorter,
}

// If SQLITE_DEBUG_SORTER_THREADS is defined, this module outputs various
// messages to stderr that may be helpful in understanding the performance
// characteristics of the sorter in multi-threaded mode.
// Hard-coded maximum amount of data to accumulate in memory before flushing
// to a level 0 PMA. The purpose of this limit is to prevent various integer
// overflows. 512MiB.
// Merge PMAs together
// Incrementally read one PMA
// Incrementally write one PMA
// A record being sorted
// A sub-task in the sort process
// Temporary file object wrapper
// In-memory list of records
// Read & merge multiple PMAs
/// A container for a temp file handle and the current amount of data
/// stored in the file.
#[repr(C)]
#[derive(Clone, Copy)]
struct SorterFile {
    /// File handle
    pFd: *mut sqlite3_file,
    /// Bytes of data stored in pFd
    iEof: i64,
}

#[repr(C)]
#[derive(Clone, Copy)]
union MemValue {
    r: f64,
    i: i64,
    nZero: i32,
    zPType: *const i8,
    pDef: *mut FuncDef,
}

/// An in-memory list of objects to be sorted.
///
/// If aMemory==0 then each object is allocated separately and the objects
/// are connected using SorterRecord.u.pNext.  If aMemory!=0 then all objects
/// are stored in the aMemory[] bulk memory, one right after the other, and
/// are connected using SorterRecord.u.iNext.
#[repr(C)]
#[derive(Clone, Copy)]
struct SorterList {
    /// Linked list of records
    pList: *mut SorterRecord,
    /// If non-NULL, bulk memory to hold pList
    aMemory: *mut u8,
    /// Size of pList as PMA in bytes
    szPMA: i64,
}

/// The MergeEngine object is used to combine two or more smaller PMAs into
/// one big PMA using a merge operation.  Separate PMAs all need to be
/// combined into one big PMA in order to be able to step through the sorted
/// records in order.
///
/// The aReadr[] array contains a PmaReader object for each of the PMAs being
/// merged.  An aReadr[] object either points to a valid key or else is at EOF.
/// ("EOF" means "End Of File".  When aReadr[] is at EOF there is no more data.)
/// For the purposes of the paragraphs below, we assume that the array is
/// actually N elements in size, where N is the smallest power of 2 greater
/// to or equal to the number of PMAs being merged. The extra aReadr[] elements
/// are treated as if they are empty (always at EOF).
///
/// The aTree[] array is also N elements in size. The value of N is stored in
/// the MergeEngine.nTree variable.
///
/// The final (N/2) elements of aTree[] contain the results of comparing
/// pairs of PMA keys together. Element i contains the result of
/// comparing aReadr[2*i-N] and aReadr[2*i-N+1]. Whichever key is smaller, the
/// aTree element is set to the index of it.
///
/// For the purposes of this comparison, EOF is considered greater than any
/// other key value. If the keys are equal (only possible with two EOF
/// values), it doesn't matter which index is stored.
///
/// The (N/4) elements of aTree[] that precede the final (N/2) described
/// above contains the index of the smallest of each block of 4 PmaReaders
/// And so on. So that aTree[1] contains the index of the PmaReader that
/// currently points to the smallest key value. aTree[0] is unused.
///
/// Example:
///
///     aReadr[0] -> Banana
///     aReadr[1] -> Feijoa
///     aReadr[2] -> Elderberry
///     aReadr[3] -> Currant
///     aReadr[4] -> Grapefruit
///     aReadr[5] -> Apple
///     aReadr[6] -> Durian
///     aReadr[7] -> EOF
///
///     aTree[] = { X, 5   0, 5    0, 3, 5, 6 }
///
/// The current element is "Apple" (the value of the key indicated by
/// PmaReader 5). When the Next() operation is invoked, PmaReader 5 will
/// be advanced to the next key in its segment. Say the next key is
/// "Eggplant":
///
///     aReadr[5] -> Eggplant
///
/// The contents of aTree[] are updated first by comparing the new PmaReader
/// 5 key to the current key of PmaReader 4 (still "Grapefruit"). The PmaReader
/// 5 value is still smaller, so aTree[6] is set to 5. And so on up the tree.
/// The value of PmaReader 6 - "Durian" - is now smaller than that of PmaReader
/// 5, so aTree[3] is set to 6. Key 0 is smaller than key 6 (Banana<Durian),
/// so the value written into element 1 of the array is 0. As follows:
///
///     aTree[] = { X, 0   0, 6    0, 3, 5, 6 }
///
/// In other words, each time we advance to the next sorter element, log2(N)
/// key comparison operations are required, where N is the number of segments
/// being merged (rounded up to the next power of 2).
#[repr(C)]
#[derive(Clone, Copy)]
struct MergeEngine {
    /// Used size of aTree/aReadr (power of 2)
    nTree: i32,
    /// Used by this thread only
    pTask: *mut SortSubtask,
    /// Current state of incremental merge
    aTree: *mut i32,
    /// Array of PmaReaders to merge data from
    aReadr: *mut PmaReader,
}

/// This object represents a single thread of control in a sort operation.
/// Exactly VdbeSorter.nTask instances of this object are allocated
/// as part of each VdbeSorter object. Instances are never allocated any
/// other way. VdbeSorter.nTask is set to the number of worker threads allowed
/// (see SQLITE_CONFIG_WORKER_THREADS) plus one (the main thread).  Thus for
/// single-threaded operation, there is exactly one instance of this object
/// and for multi-threaded operation there are two or more instances.
///
/// Essentially, this structure contains all those fields of the VdbeSorter
/// structure for which each thread requires a separate instance. For example,
/// each thread requeries its own UnpackedRecord object to unpack records in
/// as part of comparison operations.
///
/// Before a background thread is launched, variable bDone is set to 0. Then,
/// right before it exits, the thread itself sets bDone to 1. This is used for
/// two purposes:
///
///   1. When flushing the contents of memory to a level-0 PMA on disk, to
///      attempt to select a SortSubtask for which there is not already an
///      active background thread (since doing so causes the main thread
///      to block until it finishes).
///
///   2. If SQLITE_DEBUG_SORTER_THREADS is defined, to determine if a call
///      to sqlite3ThreadJoin() is likely to block. Cases that are likely to
///      block provoke debugging output.
///
/// In both cases, the effects of the main thread seeing (bDone==0) even
/// after the thread has finished are not dire. So we don't worry about
/// memory barriers and such here.
#[repr(C)]
#[derive(Clone, Copy)]
struct SortSubtask {
    /// Background thread, if any
    pThread: *mut SQLiteThread,
    /// Set if thread is finished but not joined
    bDone: i32,
    /// Number of PMAs currently in file
    nPMA: i32,
    /// Sorter that owns this sub-task
    pSorter: *mut VdbeSorter,
    /// Space to unpack a record
    pUnpacked: *mut UnpackedRecord,
    /// List for thread to write to a PMA
    list: SorterList,
    /// Compare function to use
    xCompare: Option<
        unsafe extern "C-unwind" fn(
            *mut SortSubtask,
            *mut i32,
            *const (),
            i32,
            *const (),
            i32,
        ) -> i32,
    >,
    /// Temp file for level-0 PMAs
    file: SorterFile,
    /// Space for other PMAs
    file2: SorterFile,
    /// Total bytes written by this task
    nSpill: u64,
}

/// Main sorter structure. A single instance of this is allocated for each
/// sorter cursor created by the VDBE.
///
/// mxKeysize:
///   As records are added to the sorter by calls to sqlite3VdbeSorterWrite(),
///   this variable is updated so as to be set to the size on disk of the
///   largest record in the sorter.
#[repr(C)]
#[derive(Clone, Copy)]
struct VdbeSorter {
    /// Minimum PMA size, in bytes
    mnPmaSize: i32,
    /// Maximum PMA size, in bytes.  0==no limit
    mxPmaSize: i32,
    /// Largest serialized key seen so far
    mxKeysize: i32,
    /// Main database page size
    pgsz: i32,
    /// Readr data from here after Rewind()
    pReader: *mut PmaReader,
    /// Or here, if bUseThreads==0
    pMerger: *mut MergeEngine,
    /// Database connection
    db: *mut sqlite3,
    /// How to compare records
    pKeyInfo: *mut KeyInfo,
    /// Used by VdbeSorterCompare()
    pUnpacked: *mut UnpackedRecord,
    /// List of in-memory records
    list: SorterList,
    /// Offset of free space in list.aMemory
    iMemory: i32,
    /// Size of list.aMemory allocation in bytes
    nMemory: i32,
    /// True if one or more PMAs created
    bUsePMA: u8,
    /// True to use background threads
    bUseThreads: u8,
    /// Previous thread used to flush PMA
    iPrev: u8,
    /// Size of aTask[] array
    nTask: u8,
    typeMask: u8,
    /// One or more subtasks
    aTask: [SortSubtask; 0],
}

// Size (in bytes) of a VdbeSorter object that works with N or fewer subtasks
/// An instance of the following object is used to read records out of a
/// PMA, in sorted order.  The next key to be read is cached in nKey/aKey.
/// aKey might point into aMap or into aBuffer.  If neither of those locations
/// contain a contiguous representation of the key, then aAlloc is allocated
/// and the key is copied into aAlloc and aKey is made to point to aAlloc.
///
/// pFd==0 at EOF.
#[repr(C)]
#[derive(Clone, Copy)]
struct PmaReader {
    /// Current read offset
    iReadOff: i64,
    /// 1 byte past EOF for this PmaReader
    iEof: i64,
    /// Bytes of space at aAlloc
    nAlloc: i32,
    /// Number of bytes in key
    nKey: i32,
    /// File handle we are reading from
    pFd: *mut sqlite3_file,
    /// Space for aKey if aBuffer and pMap wont work
    aAlloc: *mut u8,
    /// Pointer to current key
    aKey: *mut u8,
    /// Current read buffer
    aBuffer: *mut u8,
    /// Size of read buffer in bytes
    nBuffer: i32,
    /// Pointer to mapping of entire file
    aMap: *mut u8,
    /// Incremental merger
    pIncr: *mut IncrMerger,
}

/// Normally, a PmaReader object iterates through an existing PMA stored
/// within a temp file. However, if the PmaReader.pIncr variable points to
/// an object of the following type, it may be used to iterate/merge through
/// multiple PMAs simultaneously.
///
/// There are two types of IncrMerger object - single (bUseThread==0) and
/// multi-threaded (bUseThread==1).
///
/// A multi-threaded IncrMerger object uses two temporary files - aFile[0]
/// and aFile[1]. Neither file is allowed to grow to more than mxSz bytes in
/// size. When the IncrMerger is initialized, it reads enough data from
/// pMerger to populate aFile[0]. It then sets variables within the
/// corresponding PmaReader object to read from that file and kicks off
/// a background thread to populate aFile[1] with the next mxSz bytes of
/// sorted record data from pMerger.
///
/// When the PmaReader reaches the end of aFile[0], it blocks until the
/// background thread has finished populating aFile[1]. It then exchanges
/// the contents of the aFile[0] and aFile[1] variables within this structure,
/// sets the PmaReader fields to read from the new aFile[0] and kicks off
/// another background thread to populate the new aFile[1]. And so on, until
/// the contents of pMerger are exhausted.
///
/// A single-threaded IncrMerger does not open any temporary files of its
/// own. Instead, it has exclusive access to mxSz bytes of space beginning
/// at offset iStartOff of file pTask->file2. And instead of using a
/// background thread to prepare data for the PmaReader, with a single
/// threaded IncrMerger the allocate part of pTask->file2 is "refilled" with
/// keys from pMerger by the calling thread whenever the PmaReader runs out
/// of data.
#[repr(C)]
#[derive(Clone, Copy)]
struct IncrMerger {
    /// Task that owns this merger
    pTask: *mut SortSubtask,
    /// Merge engine thread reads data from
    pMerger: *mut MergeEngine,
    /// Offset to start writing file at
    iStartOff: i64,
    /// Maximum bytes of data to store
    mxSz: i32,
    /// Set to true when merge is finished
    bEof: i32,
    /// True to use a bg thread for this object
    bUseThread: i32,
    /// aFile[0] for reading, [1] for writing
    aFile: [SorterFile; 2],
}

/// An instance of this object is used for writing a PMA.
///
/// The PMA is written one record at a time.  Each record is of an arbitrary
/// size.  But I/O is more efficient if it occurs in page-sized blocks where
/// each block is aligned on a page boundary.  This object caches writes to
/// the PMA so that aligned, page-size blocks are written.
#[repr(C)]
#[derive(Clone, Copy)]
struct PmaWriter {
    /// Non-zero if in an error state
    eFWErr: i32,
    /// Pointer to write buffer
    aBuffer: *mut u8,
    /// Size of write buffer in bytes
    nBuffer: i32,
    /// First byte of buffer to write
    iBufStart: i32,
    /// Last byte of buffer to write
    iBufEnd: i32,
    /// Offset of start of buffer in file
    iWriteOff: i64,
    /// File handle to write to
    pFd: *mut sqlite3_file,
    /// Total number of bytes written
    nPmaSpill: u64,
}

/// This object is the header on a single record while that record is being
/// held in memory and prior to being written out as part of a PMA.
///
/// How the linked list is connected depends on how memory is being managed
/// by this module. If using a separate allocation for each in-memory record
/// (VdbeSorter.list.aMemory==0), then the list is always connected using the
/// SorterRecord.u.pNext pointers.
///
/// Or, if using the single large allocation method (VdbeSorter.list.aMemory!=0),
/// then while records are being accumulated the list is linked using the
/// SorterRecord.u.iNext offset. This is because the aMemory[] array may
/// be sqlite3Realloc()ed while records are being accumulated. Once the VM
/// has finished passing records to the sorter, or when the in-memory buffer
/// is full, the list is sorted. As part of the sorting process, it is
/// converted to use the SorterRecord.u.pNext pointers. See function
/// vdbeSorterSort() for details.
#[repr(C)]
#[derive(Clone, Copy)]
struct SorterRecord {
    /// Size of the record in bytes
    nVal: i32,
    u: __SlateRecord238,
    // The data for the record immediately follows this header
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord238 {
    /// Pointer to next record in list
    pNext: *mut SorterRecord,
    /// Offset within aMemory of next record
    iNext: i32,
}

#[repr(C, align(16))]
struct __SlateAlign16<T>(T);

mod __slate_bits {
    #[bitfields::bitfield([u8; 2], c_names = true, new = false, from_into_bits = false, from_traits = false, default = false, debug = false, builder = false, bit_ops = false)]
    pub struct __SlateBits111U0 {
        #[bits(1)]
        pub disableTriggers: u32,
        #[bits(1)]
        pub mayAbort: u32,
        #[bits(1)]
        pub hasCompound: u32,
        #[bits(1)]
        pub bReturning: u32,
        #[bits(1)]
        pub bHasExists: u32,
        #[bits(1)]
        pub colNamesSet: u32,
        #[bits(1)]
        pub bHasWith: u32,
        #[bits(1)]
        pub okConstFactor: u32,
        #[bits(1)]
        pub checkSchema: u32,
        #[bits(1)]
        pub usesAinc: u32,
        #[bits(6, access = na)]
        pub __slate_pad_10: u8,
    }
    #[bitfields::bitfield(
        u8,
        c_names = true,
        new = false,
        from_into_bits = false,
        from_traits = false,
        default = false,
        debug = false,
        builder = false,
        bit_ops = false
    )]
    pub struct __SlateBits75U0 {
        #[bits(4)]
        pub notNull: u32,
        #[bits(4)]
        pub eCType: u32,
    }
    #[bitfields::bitfield([u8; 2], c_names = true, new = false, from_into_bits = false, from_traits = false, default = false, debug = false, builder = false, bit_ops = false)]
    pub struct __SlateBits99U0 {
        #[bits(2)]
        pub idxType: u32,
        #[bits(1)]
        pub bUnordered: u32,
        #[bits(1)]
        pub uniqNotNull: u32,
        #[bits(1)]
        pub isResized: u32,
        #[bits(1)]
        pub isCovering: u32,
        #[bits(1)]
        pub noSkipScan: u32,
        #[bits(1)]
        pub hasStat1: u32,
        #[bits(1)]
        pub bNoQuery: u32,
        #[bits(1)]
        pub bAscKeyBug: u32,
        #[bits(1)]
        pub bHasVCol: u32,
        #[bits(1)]
        pub bHasExpr: u32,
        #[bits(4, access = na)]
        pub __slate_pad_11: u8,
    }
    #[bitfields::bitfield([u8; 3], c_names = true, new = false, from_into_bits = false, from_traits = false, default = false, debug = false, builder = false, bit_ops = false)]
    pub struct __SlateBits196U0 {
        #[bits(1)]
        pub notIndexed: u32,
        #[bits(1)]
        pub isIndexedBy: u32,
        #[bits(1)]
        pub isSubquery: u32,
        #[bits(1)]
        pub isTabFunc: u32,
        #[bits(1)]
        pub isCorrelated: u32,
        #[bits(1)]
        pub isMaterialized: u32,
        #[bits(1)]
        pub viaCoroutine: u32,
        #[bits(1)]
        pub isRecursive: u32,
        #[bits(1)]
        pub fromDDL: u32,
        #[bits(1)]
        pub isCte: u32,
        #[bits(1)]
        pub notCte: u32,
        #[bits(1)]
        pub isUsing: u32,
        #[bits(1)]
        pub isOn: u32,
        #[bits(1)]
        pub isSynthUsing: u32,
        #[bits(1)]
        pub isNestedFrom: u32,
        #[bits(1)]
        pub rowidUsed: u32,
        #[bits(1)]
        pub fixedSchema: u32,
        #[bits(1)]
        pub hadSchema: u32,
        #[bits(1)]
        pub fromExists: u32,
        #[bits(5, access = na)]
        pub __slate_pad_19: u8,
    }
    #[bitfields::bitfield([u8; 2], c_names = true, new = false, from_into_bits = false, from_traits = false, default = false, debug = false, builder = false, bit_ops = false)]
    pub struct __SlateBits192U0 {
        #[bits(2)]
        pub eEName: u32,
        #[bits(1)]
        pub done: u32,
        #[bits(1)]
        pub reusable: u32,
        #[bits(1)]
        pub bSorterRef: u32,
        #[bits(1)]
        pub bNulls: u32,
        #[bits(1)]
        pub bUsed: u32,
        #[bits(1)]
        pub bUsingTerm: u32,
        #[bits(1)]
        pub bNoExpand: u32,
        #[bits(7, access = na)]
        pub __slate_pad_8: u8,
    }
    #[bitfields::bitfield(
        u8,
        c_names = true,
        new = false,
        from_into_bits = false,
        from_traits = false,
        default = false,
        debug = false,
        builder = false,
        bit_ops = false
    )]
    pub struct __SlateBits214U0 {
        #[bits(1)]
        pub isEphemeral: u32,
        #[bits(1)]
        pub useRandomRowid: u32,
        #[bits(1)]
        pub isOrdered: u32,
        #[bits(1)]
        pub noReuse: u32,
        #[bits(1)]
        pub colCache: u32,
        #[bits(3, access = na)]
        pub __slate_pad_5: u8,
    }
    #[bitfields::bitfield(
        u16,
        c_names = true,
        new = false,
        from_into_bits = false,
        from_traits = false,
        default = false,
        debug = false,
        builder = false,
        bit_ops = false
    )]
    pub struct __SlateBits162U0 {
        #[bits(2)]
        pub expired: u32,
        #[bits(2)]
        pub explain: u32,
        #[bits(1)]
        pub changeCntOn: u32,
        #[bits(1)]
        pub usesStmtJournal: u32,
        #[bits(1)]
        pub readOnly: u32,
        #[bits(1)]
        pub bIsReader: u32,
        #[bits(1)]
        pub haveEqpOps: u32,
        #[bits(7, access = na)]
        pub __slate_pad_7: u8,
    }
    #[bitfields::bitfield(
        u8,
        c_names = true,
        new = false,
        from_into_bits = false,
        from_traits = false,
        default = false,
        debug = false,
        builder = false,
        bit_ops = false
    )]
    pub struct __SlateBits173U0 {
        #[bits(1)]
        pub orphanTrigger: u32,
        #[bits(2)]
        pub imposterTable: u32,
        #[bits(1)]
        pub reopenMemdb: u32,
        #[bits(4, access = na)]
        pub __slate_pad_3: u8,
    }
}

// Return a pointer to the buffer containing the record data for SorterRecord
// object p. Should be used as if:
//
//   void *SRVAL(SorterRecord *p) { return (void*)&p[1]; }
// Maximum number of PMAs that a single MergeEngine can merge
/// Free all memory belonging to the PmaReader object passed as the
/// argument. All structure fields are set to zero before returning.
fn vdbePmaReaderClear(mut pReadr: *mut PmaReader) {
    unsafe { sqlite3_free((unsafe { (*pReadr).aAlloc }) as *mut ()) };
    unsafe { sqlite3_free((unsafe { (*pReadr).aBuffer }) as *mut ()) };
    if (unsafe { (*pReadr).aMap }) != std::ptr::null_mut::<u8>() {
        unsafe {
            sqlite3OsUnfetch(
                unsafe { (*pReadr).pFd },
                (0 as i32) as i64,
                (unsafe { (*pReadr).aMap }) as *mut (),
            )
        };
    }
    vdbeIncrFree(unsafe { (*pReadr).pIncr });
    unsafe { memset(pReadr as *mut (), 0 as i32, 80 as u64) };
}

/// Read the next nByte bytes of data from the PMA p.
/// If successful, set *ppOut to point to a buffer containing the data
/// and return SQLITE_OK. Otherwise, if an error occurs, return an SQLite
/// error code.
///
/// The buffer returned in *ppOut is only valid until the
/// next call to this function.
///
/// # Arguments
///
/// * `p` - PmaReader from which to take the blob
/// * `nByte` - Bytes of data to read
/// * `ppOut` - OUT: Pointer to buffer containing data
fn vdbePmaReadBlob(mut p: *mut PmaReader, mut nByte: i32, mut ppOut: *mut *mut u8) -> i32 {
    let mut iBuf: i32 = 0 as i32; // Offset within buffer to read from
    let mut nAvail: i32 = 0 as i32; // Bytes of data available in buffer
    if (unsafe { (*p).aMap }) != std::ptr::null_mut::<u8>() {
        unsafe {
            *ppOut = unsafe { unsafe { (*p).aMap }.offset((unsafe { (*p).iReadOff }) as isize) };
        }
        let __v1004: *mut PmaReader = p;
        let __v1005: i64 = unsafe { (*__v1004).iReadOff };
        let __v1006: i64 = __v1005 + (nByte as i64);
        unsafe {
            (*__v1004).iReadOff = __v1006;
        }
        return 0 as i32;
    }
    0 as i32;
    // If there is no more data to be read from the buffer, read the next
    // p->nBuffer bytes of data from the file into it. Or, if there are less
    // than p->nBuffer bytes remaining in the PMA, read all remaining data.
    iBuf = ((unsafe { (*p).iReadOff }) % ((unsafe { (*p).nBuffer }) as i64)) as i32;
    if iBuf == (0 as i32) {
        let mut nRead: i32 = 0 as i32; // Bytes to read from disk
        let mut rc: i32 = 0 as i32; // sqlite3OsRead() return code
        // Determine how many bytes of data to read.
        if (unsafe { (*p).iEof }) - unsafe { (*p).iReadOff } > ((unsafe { (*p).nBuffer }) as i64) {
            nRead = unsafe { (*p).nBuffer };
        } else {
            nRead = ((unsafe { (*p).iEof }) - unsafe { (*p).iReadOff }) as i32;
        }
        0 as i32;
        // Readr data from the file. Return early if an error occurs.
        rc = unsafe {
            sqlite3OsRead(
                unsafe { (*p).pFd },
                (unsafe { (*p).aBuffer }) as *mut (),
                nRead,
                unsafe { (*p).iReadOff },
            )
        };
        0 as i32;
        if rc != (0 as i32) {
            return rc;
        }
    }
    nAvail = (unsafe { (*p).nBuffer }) - iBuf;
    if nByte <= nAvail {
        // The requested data is available in the in-memory buffer. In this
        // case there is no need to make a copy of the data, just return a
        // pointer into the buffer to the caller.
        unsafe {
            *ppOut = unsafe { unsafe { (*p).aBuffer }.offset(iBuf as isize) };
        }
        let __v1007: *mut PmaReader = p;
        let __v1008: i64 = unsafe { (*__v1007).iReadOff };
        let __v1009: i64 = __v1008 + (nByte as i64);
        unsafe {
            (*__v1007).iReadOff = __v1009;
        }
    } else {
        // The requested data is not all available in the in-memory buffer.
        // In this case, allocate space at p->aAlloc[] to copy the requested
        // range into. Then return a copy of pointer p->aAlloc to the caller.
        let mut nRem: i32 = 0 as i32; // Bytes remaining to copy
        // Extend the p->aAlloc[] allocation if required.
        if (unsafe { (*p).nAlloc }) < nByte {
            let mut aNew: *mut u8 = unsafe { std::mem::zeroed() };
            let mut nNew: i64 = if ((128 as i32) as i64)
                > ((2 as i32) as i64) * ((unsafe { (*p).nAlloc }) as i64)
            {
                (128 as i32) as i64
            } else {
                ((2 as i32) as i64) * ((unsafe { (*p).nAlloc }) as i64)
            };
            '__slate_break_938: while (nByte as i64) > nNew {
                nNew = nNew * ((2 as i32) as i64);
            }
            aNew = (unsafe { sqlite3Realloc((unsafe { (*p).aAlloc }) as *mut (), nNew as u64) })
                as *mut u8;
            if !(aNew != std::ptr::null_mut::<u8>()) {
                return 7 as i32;
            }
            unsafe {
                (*p).nAlloc = nNew as i32;
            }
            unsafe {
                (*p).aAlloc = aNew;
            }
        }
        // Copy as much data as is available in the buffer into the start of
        // p->aAlloc[].
        unsafe {
            memcpy(
                (unsafe { (*p).aAlloc }) as *mut (),
                (unsafe { unsafe { (*p).aBuffer }.offset(iBuf as isize) }) as *const (),
                (nAvail as i64) as u64,
            )
        };
        let __v1010: *mut PmaReader = p;
        let __v1011: i64 = unsafe { (*__v1010).iReadOff };
        let __v1012: i64 = __v1011 + (nAvail as i64);
        unsafe {
            (*__v1010).iReadOff = __v1012;
        }
        nRem = nByte - nAvail;
        // The following loop copies up to p->nBuffer bytes per iteration into
        // the p->aAlloc[] buffer.
        '__slate_break_939: while nRem > (0 as i32) {
            let mut rc: i32 = 0 as i32; // vdbePmaReadBlob() return code
            let mut nCopy: i32 = 0 as i32; // Number of bytes to copy
            let mut aNext: *mut u8 = std::ptr::null_mut::<u8>(); // Pointer to buffer to copy data from
            nCopy = nRem;
            if nRem > unsafe { (*p).nBuffer } {
                nCopy = unsafe { (*p).nBuffer };
            }
            rc = vdbePmaReadBlob(p, nCopy, std::ptr::addr_of_mut!(aNext));
            if rc != (0 as i32) {
                return rc;
            }
            0 as i32;
            0 as i32;
            unsafe {
                memcpy(
                    (unsafe { unsafe { (*p).aAlloc }.offset((nByte - nRem) as isize) }) as *mut (),
                    aNext as *const (),
                    (nCopy as i64) as u64,
                )
            };
            let __v1013: i32 = nRem;
            let __v1014: i32 = __v1013 - nCopy;
            nRem = __v1014;
        }
        unsafe {
            *ppOut = unsafe { (*p).aAlloc };
        }
    }
    return 0 as i32;
}

/// Read a varint from the stream of data accessed by p. Set *pnOut to
/// the value read.
fn vdbePmaReadVarint(mut p: *mut PmaReader, mut pnOut: *mut u64) -> i32 {
    let mut iBuf: i32 = 0 as i32;
    if (unsafe { (*p).aMap }) != std::ptr::null_mut::<u8>() {
        let __v1015: *mut PmaReader = p;
        let __v1016: i64 = unsafe { (*__v1015).iReadOff };
        let __v1017: i64 = __v1016
            + ((((unsafe {
                sqlite3GetVarint(
                    (unsafe { unsafe { (*p).aMap }.offset((unsafe { (*p).iReadOff }) as isize) })
                        as *const u8,
                    pnOut,
                )
            }) as u32) as i32) as i64);
        unsafe {
            (*__v1015).iReadOff = __v1017;
        }
    } else {
        iBuf = ((unsafe { (*p).iReadOff }) % ((unsafe { (*p).nBuffer }) as i64)) as i32;
        if iBuf != (0 as i32) && (unsafe { (*p).nBuffer }) - iBuf >= (9 as i32) {
            let __v1018: *mut PmaReader = p;
            let __v1019: i64 = unsafe { (*__v1018).iReadOff };
            let __v1020: i64 = __v1019
                + ((((unsafe {
                    sqlite3GetVarint(
                        (unsafe { unsafe { (*p).aBuffer }.offset(iBuf as isize) }) as *const u8,
                        pnOut,
                    )
                }) as u32) as i32) as i64);
            unsafe {
                (*__v1018).iReadOff = __v1020;
            }
        } else {
            let mut aVarint: __SlateAlign16<[u8; 16]> = __SlateAlign16([0 as u8; 16]);
            let mut a: *mut u8 = unsafe { std::mem::zeroed() };
            let mut i: i32 = 0 as i32;
            let mut rc: i32 = 0 as i32;
            '__slate_break_940: loop {
                rc = vdbePmaReadBlob(p, 1 as i32, std::ptr::addr_of_mut!(a));
                if rc != (0 as i32) {
                    return rc;
                }
                let __v1021: i32 = i;
                let __v1022: i32 = __v1021 + (1 as i32);
                i = __v1022;
                unsafe {
                    *unsafe {
                        (aVarint.0.as_mut_ptr() as *mut u8).offset((__v1021 & (15 as i32)) as isize)
                    } = unsafe { *unsafe { a.offset((0 as i32) as isize) } };
                }
                if !((((unsafe { *unsafe { a.offset((0 as i32) as isize) } }) as u32) as i32)
                    & (128 as i32)
                    != (0 as i32))
                {
                    break;
                }
            }
            unsafe { sqlite3GetVarint((aVarint.0.as_mut_ptr() as *mut u8) as *const u8, pnOut) };
        }
    }
    return 0 as i32;
}

/// Attempt to memory map file pFile. If successful, set *pp to point to the
/// new mapping and return SQLITE_OK. If the mapping is not attempted
/// (because the file is too large or the VFS layer is configured not to use
/// mmap), return SQLITE_OK and set *pp to NULL.
///
/// Or, if an error occurs, return an SQLite error code. The final value of
/// *pp is undefined in this case.
fn vdbeSorterMapFile(
    mut pTask: *mut SortSubtask,
    mut pFile: *mut SorterFile,
    mut pp: *mut *mut u8,
) -> i32 {
    let mut rc: i32 = 0 as i32;
    if (unsafe { (*pFile).iEof })
        <= ((unsafe { (*unsafe { (*unsafe { (*pTask).pSorter }).db }).nMaxSorterMmap }) as i64)
    {
        let mut pFd: *mut sqlite3_file = unsafe { (*pFile).pFd };
        if (unsafe { (*unsafe { (*pFd).pMethods }).iVersion }) >= (3 as i32) {
            rc = unsafe {
                sqlite3OsFetch(
                    pFd,
                    (0 as i32) as i64,
                    (unsafe { (*pFile).iEof }) as i32,
                    pp as *mut *mut (),
                )
            };
            {}
        }
    }
    return rc;
}

/// Attach PmaReader pReadr to file pFile (if it is not already attached to
/// that file) and seek it to offset iOff within the file.  Return SQLITE_OK
/// if successful, or an SQLite error code if an error occurs.
///
/// # Arguments
///
/// * `pTask` - Task context
/// * `pReadr` - Reader whose cursor is to be moved
/// * `pFile` - Sorter file to read from
/// * `iOff` - Offset in pFile
fn vdbePmaReaderSeek(
    mut pTask: *mut SortSubtask,
    mut pReadr: *mut PmaReader,
    mut pFile: *mut SorterFile,
    mut iOff: i64,
) -> i32 {
    let mut rc: i32 = 0 as i32;
    0 as i32;
    if (unsafe { sqlite3FaultSim(201 as i32) }) != (0 as i32) {
        return (10 as i32) | (1 as i32) << (8 as i32);
    }
    if (unsafe { (*pReadr).aMap }) != std::ptr::null_mut::<u8>() {
        unsafe {
            sqlite3OsUnfetch(
                unsafe { (*pReadr).pFd },
                (0 as i32) as i64,
                (unsafe { (*pReadr).aMap }) as *mut (),
            )
        };
        unsafe {
            (*pReadr).aMap = std::ptr::null_mut::<u8>();
        }
    }
    unsafe {
        (*pReadr).iReadOff = iOff;
    }
    unsafe {
        (*pReadr).iEof = unsafe { (*pFile).iEof };
    }
    unsafe {
        (*pReadr).pFd = unsafe { (*pFile).pFd };
    }
    rc = vdbeSorterMapFile(pTask, pFile, unsafe {
        std::ptr::addr_of_mut!((*pReadr).aMap)
    });
    if rc == (0 as i32) && (unsafe { (*pReadr).aMap }) == std::ptr::null_mut::<u8>() {
        let mut pgsz: i32 = unsafe { (*unsafe { (*pTask).pSorter }).pgsz };
        let mut iBuf: i32 = ((unsafe { (*pReadr).iReadOff }) % (pgsz as i64)) as i32;
        if (unsafe { (*pReadr).aBuffer }) == std::ptr::null_mut::<u8>() {
            unsafe {
                (*pReadr).aBuffer = (unsafe { sqlite3Malloc((pgsz as i64) as u64) }) as *mut u8;
            }
            if (unsafe { (*pReadr).aBuffer }) == std::ptr::null_mut::<u8>() {
                rc = 7 as i32;
            }
            unsafe {
                (*pReadr).nBuffer = pgsz;
            }
        }
        if rc == (0 as i32) && iBuf != (0 as i32) {
            let mut nRead: i32 = pgsz - iBuf;
            if (unsafe { (*pReadr).iReadOff }) + (nRead as i64) > unsafe { (*pReadr).iEof } {
                nRead = ((unsafe { (*pReadr).iEof }) - unsafe { (*pReadr).iReadOff }) as i32;
            }
            rc = unsafe {
                sqlite3OsRead(
                    unsafe { (*pReadr).pFd },
                    (unsafe { unsafe { (*pReadr).aBuffer }.offset(iBuf as isize) }) as *mut (),
                    nRead,
                    unsafe { (*pReadr).iReadOff },
                )
            };
            {}
        }
    }
    return rc;
}

/// Advance PmaReader pReadr to the next key in its PMA. Return SQLITE_OK if
/// no error occurs, or an SQLite error code if one does.
fn vdbePmaReaderNext(mut pReadr: *mut PmaReader) -> i32 {
    let mut rc: i32 = 0 as i32; // Return Code
    let mut nRec: u64 = ((0 as i32) as i64) as u64; // Size of record in bytes
    if (unsafe { (*pReadr).iReadOff }) >= unsafe { (*pReadr).iEof } {
        let mut pIncr: *mut IncrMerger = unsafe { (*pReadr).pIncr };
        let mut bEof: i32 = 1 as i32;
        if pIncr != std::ptr::null_mut::<IncrMerger>() {
            rc = vdbeIncrSwap(pIncr);
            if rc == (0 as i32) && (unsafe { (*pIncr).bEof }) == (0 as i32) {
                rc = vdbePmaReaderSeek(
                    unsafe { (*pIncr).pTask },
                    pReadr,
                    unsafe {
                        unsafe { (*pIncr).aFile.as_mut_ptr() as *mut SorterFile }
                            .offset((0 as i32) as isize)
                    },
                    unsafe { (*pIncr).iStartOff },
                );
                bEof = 0 as i32;
            }
        }
        if bEof != (0 as i32) {
            // This is an EOF condition
            vdbePmaReaderClear(pReadr);
            {}
            return rc;
        }
    }
    if rc == (0 as i32) {
        rc = vdbePmaReadVarint(pReadr, std::ptr::addr_of_mut!(nRec));
    }
    if rc == (0 as i32) {
        unsafe {
            (*pReadr).nKey = (nRec as u32) as i32;
        }
        rc = vdbePmaReadBlob(pReadr, (nRec as u32) as i32, unsafe {
            std::ptr::addr_of_mut!((*pReadr).aKey)
        });
        {}
    }
    return rc;
}

/// Initialize PmaReader pReadr to scan through the PMA stored in file pFile
/// starting at offset iStart and ending at offset iEof-1. This function
/// leaves the PmaReader pointing to the first key in the PMA (or EOF if the
/// PMA is empty).
///
/// If the pnByte parameter is NULL, then it is assumed that the file
/// contains a single PMA, and that that PMA omits the initial length varint.
///
/// # Arguments
///
/// * `pTask` - Task context
/// * `pFile` - Sorter file to read from
/// * `iStart` - Start offset in pFile
/// * `pReadr` - PmaReader to populate
/// * `pnByte` - IN/OUT: Increment this value by PMA size
fn vdbePmaReaderInit(
    mut pTask: *mut SortSubtask,
    mut pFile: *mut SorterFile,
    mut iStart: i64,
    mut pReadr: *mut PmaReader,
    mut pnByte: *mut i64,
) -> i32 {
    let mut rc: i32 = 0 as i32;
    0 as i32;
    0 as i32;
    0 as i32;
    0 as i32;
    rc = vdbePmaReaderSeek(pTask, pReadr, pFile, iStart);
    if rc == (0 as i32) {
        let mut nByte: u64 = ((0 as i32) as i64) as u64; // Size of PMA in bytes
        rc = vdbePmaReadVarint(pReadr, std::ptr::addr_of_mut!(nByte));
        unsafe {
            (*pReadr).iEof = ((unsafe { (*pReadr).iReadOff }) as u64).wrapping_add(nByte) as i64;
        }
        let __v1023: *mut i64 = pnByte;
        let __v1024: i64 = unsafe { *__v1023 };
        let __v1025: i64 = (__v1024 as u64).wrapping_add(nByte) as i64;
        unsafe {
            *__v1023 = __v1025;
        }
    }
    if rc == (0 as i32) {
        rc = vdbePmaReaderNext(pReadr);
    }
    return rc;
}

/// A version of vdbeSorterCompare() that assumes that it has already been
/// determined that the first field of key1 is equal to the first field of
/// key2.
///
/// # Arguments
///
/// * `pTask` - Subtask context (for pKeyInfo)
/// * `pbKey2Cached` - True if pTask->pUnpacked is pKey2
/// * `nKey1` - Left side of comparison
/// * `nKey2` - Right side of comparison
fn vdbeSorterCompareTail(
    mut pTask: *mut SortSubtask,
    mut pbKey2Cached: *mut i32,
    mut pKey1: *const (),
    mut nKey1: i32,
    mut pKey2: *const (),
    mut nKey2: i32,
) -> i32 {
    let mut r2: *mut UnpackedRecord = unsafe { (*pTask).pUnpacked };
    if (unsafe { *pbKey2Cached }) == (0 as i32) {
        unsafe { sqlite3VdbeRecordUnpack(nKey2, pKey2, r2) };
        unsafe {
            *pbKey2Cached = 1 as i32;
        }
    }
    return unsafe { sqlite3VdbeRecordCompareWithSkip(nKey1, pKey1, r2, 1 as i32) };
}

/// Compare key1 (buffer pKey1, size nKey1 bytes) with key2 (buffer pKey2,
/// size nKey2 bytes). Use (pTask->pKeyInfo) for the collation sequences
/// used by the comparison. Return the result of the comparison.
///
/// If IN/OUT parameter *pbKey2Cached is true when this function is called,
/// it is assumed that (pTask->pUnpacked) contains the unpacked version
/// of key2. If it is false, (pTask->pUnpacked) is populated with the unpacked
/// version of key2 and *pbKey2Cached set to true before returning.
///
/// If an OOM error is encountered, (pTask->pUnpacked->error_rc) is set
/// to SQLITE_NOMEM.
///
/// # Arguments
///
/// * `pTask` - Subtask context (for pKeyInfo)
/// * `pbKey2Cached` - True if pTask->pUnpacked is pKey2
/// * `nKey1` - Left side of comparison
/// * `nKey2` - Right side of comparison
#[unsafe(link_section = ".text.slate_distinct.vdbesort.vdbeSorterCompare")]
extern "C-unwind" fn vdbeSorterCompare(
    mut pTask: *mut SortSubtask,
    mut pbKey2Cached: *mut i32,
    mut pKey1: *const (),
    mut nKey1: i32,
    mut pKey2: *const (),
    mut nKey2: i32,
) -> i32 {
    let mut r2: *mut UnpackedRecord = unsafe { (*pTask).pUnpacked };
    if !((unsafe { *pbKey2Cached }) != (0 as i32)) {
        unsafe { sqlite3VdbeRecordUnpack(nKey2, pKey2, r2) };
        unsafe {
            *pbKey2Cached = 1 as i32;
        }
    }
    return unsafe { sqlite3VdbeRecordCompare(nKey1, pKey1, r2) };
}

/// A specially optimized version of vdbeSorterCompare() that assumes that
/// the first field of each key is a TEXT value and that the collation
/// sequence to compare them with is BINARY.
///
/// # Arguments
///
/// * `pTask` - Subtask context (for pKeyInfo)
/// * `pbKey2Cached` - True if pTask->pUnpacked is pKey2
/// * `nKey1` - Left side of comparison
/// * `nKey2` - Right side of comparison
#[unsafe(link_section = ".text.slate_distinct.vdbesort.vdbeSorterCompareText")]
extern "C-unwind" fn vdbeSorterCompareText(
    mut pTask: *mut SortSubtask,
    mut pbKey2Cached: *mut i32,
    mut pKey1: *const (),
    mut nKey1: i32,
    mut pKey2: *const (),
    mut nKey2: i32,
) -> i32 {
    let mut p1: *const u8 = pKey1 as *const u8;
    let mut p2: *const u8 = pKey2 as *const u8;
    let mut v1: *const u8 = unsafe {
        p1.offset(
            (((unsafe { *unsafe { p1.offset((0 as i32) as isize) } }) as u32) as i32) as isize,
        )
    }; // Pointer to value 1
    let mut v2: *const u8 = unsafe {
        p2.offset(
            (((unsafe { *unsafe { p2.offset((0 as i32) as isize) } }) as u32) as i32) as isize,
        )
    }; // Pointer to value 2
    let mut n1: i32 = 0 as i32;
    let mut n2: i32 = 0 as i32;
    let mut res: i32 = 0 as i32;
    n1 = ((unsafe { *unsafe { p1.offset((1 as i32) as isize) } }) as u32) as i32;
    if n1 >= (128 as i32) {
        unsafe {
            sqlite3GetVarint32(
                unsafe { p1.offset((1 as i32) as isize) },
                std::ptr::addr_of_mut!(n1) as *mut u32,
            )
        };
    }
    n2 = ((unsafe { *unsafe { p2.offset((1 as i32) as isize) } }) as u32) as i32;
    if n2 >= (128 as i32) {
        unsafe {
            sqlite3GetVarint32(
                unsafe { p2.offset((1 as i32) as isize) },
                std::ptr::addr_of_mut!(n2) as *mut u32,
            )
        };
    }
    res = unsafe {
        memcmp(
            v1 as *const (),
            v2 as *const (),
            ((((if n1 < n2 { n1 } else { n2 }) - (13 as i32)) / (2 as i32)) as i64) as u64,
        )
    };
    if res == (0 as i32) {
        res = n1 - n2;
    }
    if res == (0 as i32) {
        if (((unsafe { (*unsafe { (*unsafe { (*pTask).pSorter }).pKeyInfo }).nKeyField }) as u32)
            as i32)
            > (1 as i32)
        {
            res = vdbeSorterCompareTail(pTask, pbKey2Cached, pKey1, nKey1, pKey2, nKey2);
        }
    } else {
        0 as i32;
        0 as i32;
        if (unsafe {
            *unsafe {
                unsafe { (*unsafe { (*unsafe { (*pTask).pSorter }).pKeyInfo }).aSortFlags }
                    .offset((0 as i32) as isize)
            }
        }) != (0 as u8)
        {
            res = res * -(1 as i32);
        }
    }
    return res;
}

/// A specially optimized version of vdbeSorterCompare() that assumes that
/// the first field of each key is an INTEGER value.
///
/// # Arguments
///
/// * `pTask` - Subtask context (for pKeyInfo)
/// * `pbKey2Cached` - True if pTask->pUnpacked is pKey2
/// * `nKey1` - Left side of comparison
/// * `nKey2` - Right side of comparison
#[unsafe(link_section = ".text.slate_distinct.vdbesort.vdbeSorterCompareInt")]
extern "C-unwind" fn vdbeSorterCompareInt(
    mut pTask: *mut SortSubtask,
    mut pbKey2Cached: *mut i32,
    mut pKey1: *const (),
    mut nKey1: i32,
    mut pKey2: *const (),
    mut nKey2: i32,
) -> i32 {
    let mut p1: *const u8 = pKey1 as *const u8;
    let mut p2: *const u8 = pKey2 as *const u8;
    let mut s1: i32 = ((unsafe { *unsafe { p1.offset((1 as i32) as isize) } }) as u32) as i32; // Left hand serial type
    let mut s2: i32 = ((unsafe { *unsafe { p2.offset((1 as i32) as isize) } }) as u32) as i32; // Right hand serial type
    let mut v1: *const u8 = unsafe {
        p1.offset(
            (((unsafe { *unsafe { p1.offset((0 as i32) as isize) } }) as u32) as i32) as isize,
        )
    }; // Pointer to value 1
    let mut v2: *const u8 = unsafe {
        p2.offset(
            (((unsafe { *unsafe { p2.offset((0 as i32) as isize) } }) as u32) as i32) as isize,
        )
    }; // Pointer to value 2
    let mut res: i32 = 0 as i32; // Return value
    0 as i32;
    0 as i32;
    if s1 == s2 {
        // The two values have the same sign. Compare using memcmp().
        let mut n: u8 = unsafe {
            *unsafe { unsafe { std::ptr::addr_of!(aLen) as *const u8 }.offset(s1 as isize) }
        };
        let mut i: i32 = 0 as i32;
        res = 0 as i32;
        i = 0 as i32;
        '__slate_break_941: loop {
            if !(i < ((n as u32) as i32)) {
                break;
            }
            let __v1028: i32 = (((unsafe { *unsafe { v1.offset(i as isize) } }) as u32) as i32)
                - (((unsafe { *unsafe { v2.offset(i as isize) } }) as u32) as i32);
            res = __v1028;
            if __v1028 != (0 as i32) {
                if ((((unsafe { *unsafe { v1.offset((0 as i32) as isize) } }) as u32) as i32)
                    ^ (((unsafe { *unsafe { v2.offset((0 as i32) as isize) } }) as u32) as i32))
                    & (128 as i32)
                    != (0 as i32)
                {
                    res = if (((unsafe { *unsafe { v1.offset((0 as i32) as isize) } }) as u32)
                        as i32)
                        & (128 as i32)
                        != (0 as i32)
                    {
                        -(1 as i32)
                    } else {
                        1 as i32
                    };
                }
                break '__slate_break_941;
            }
            let __v1026: i32 = i;
            let __v1027: i32 = __v1026 + (1 as i32);
            i = __v1027;
        }
    } else {
        if s1 > (7 as i32) && s2 > (7 as i32) {
            res = s1 - s2;
        } else {
            if s2 > (7 as i32) {
                res = 1 as i32;
            } else {
                if s1 > (7 as i32) {
                    res = -(1 as i32);
                } else {
                    res = s1 - s2;
                }
            }
            0 as i32;
            if res > (0 as i32) {
                if (((unsafe { *v1 }) as u32) as i32) & (128 as i32) != (0 as i32) {
                    res = -(1 as i32);
                }
            } else {
                if (((unsafe { *v2 }) as u32) as i32) & (128 as i32) != (0 as i32) {
                    res = 1 as i32;
                }
            }
        }
    }
    0 as i32;
    if res == (0 as i32) {
        if (((unsafe { (*unsafe { (*unsafe { (*pTask).pSorter }).pKeyInfo }).nKeyField }) as u32)
            as i32)
            > (1 as i32)
        {
            res = vdbeSorterCompareTail(pTask, pbKey2Cached, pKey1, nKey1, pKey2, nKey2);
        }
    } else {
        if (unsafe {
            *unsafe {
                unsafe { (*unsafe { (*unsafe { (*pTask).pSorter }).pKeyInfo }).aSortFlags }
                    .offset((0 as i32) as isize)
            }
        }) != (0 as u8)
        {
            0 as i32;
            res = res * -(1 as i32);
        }
    }
    return res;
}

static mut aLen: [u8; 10] = [
    ((0 as i32) as i8) as u8,
    ((1 as i32) as i8) as u8,
    ((2 as i32) as i8) as u8,
    ((3 as i32) as i8) as u8,
    ((4 as i32) as i8) as u8,
    ((6 as i32) as i8) as u8,
    ((8 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
];

/// Helper function for vdbeSorterCompareReal().
///
/// The first elements of both pKey1 and pKey2 have been decoded into double
/// values r1 and r2.  Do the comparison between those keys and return the
/// result.  If r1==r2, break the tie with a comparison of subsequent elements
/// from each key.
///
/// # Arguments
///
/// * `pTask` - Subtask context (for pKeyInfo)
/// * `pbKey2Cached` - True if pTask->pUnpacked is pKey2
/// * `nKey1` - Left side of comparison
/// * `nKey2` - Right side of comparison
/// * `r1` - REAL value of first element of pKey1
/// * `r2` - REAL value of first element of pKey2
fn vdbeSorterFinishRealCompare(
    mut pTask: *mut SortSubtask,
    mut pbKey2Cached: *mut i32,
    mut pKey1: *const (),
    mut nKey1: i32,
    mut pKey2: *const (),
    mut nKey2: i32,
    mut r1: f64,
    mut r2: f64,
) -> i32 {
    let mut res: i32 = 0 as i32;
    if r1 < r2 {
        res = -(1 as i32);
    } else {
        if r1 > r2 {
            res = 1 as i32;
        } else {
            res = 0 as i32;
        }
    }
    0 as i32;
    if res == (0 as i32) {
        if (((unsafe { (*unsafe { (*unsafe { (*pTask).pSorter }).pKeyInfo }).nKeyField }) as u32)
            as i32)
            > (1 as i32)
        {
            res = vdbeSorterCompareTail(pTask, pbKey2Cached, pKey1, nKey1, pKey2, nKey2);
        }
    } else {
        if (unsafe {
            *unsafe {
                unsafe { (*unsafe { (*unsafe { (*pTask).pSorter }).pKeyInfo }).aSortFlags }
                    .offset((0 as i32) as isize)
            }
        }) != (0 as u8)
        {
            0 as i32;
            res = res * -(1 as i32);
        }
    }
    return res;
}

/// Helper function for vdbeSorterCompareReal().
///
/// Buffer p[] is a record where the first term is guaranteed to be either
/// a floating-point value, or an integer stand-in for a floating point
/// value (a MEM_IntReal).  Whatever its format, extract the value and
/// return it.
fn vdbeSorterGetReal(mut p: *const u8) -> f64 {
    let mut r: f64 = 0 as f64; // the return value
    0 as i32; // 1-byte headers: nAllField<13
    0 as i32; // first fields proven numeric
    if (((unsafe { *unsafe { p.offset((1 as i32) as isize) } }) as u32) as i32) == (7 as i32) {
        let mut x: u64 = unsafe {
            sqlite3Get8byte(unsafe {
                p.offset(
                    (((unsafe { *unsafe { p.offset((0 as i32) as isize) } }) as u32) as i32)
                        as isize,
                )
            })
        };
        {}
        0 as i32;
        unsafe {
            memcpy(
                std::ptr::addr_of_mut!(r) as *mut (),
                std::ptr::addr_of_mut!(x) as *const (),
                8 as u64,
            )
        };
    } else {
        let mut m: sqlite3_value = unsafe { std::mem::zeroed() };
        unsafe {
            m.u.i = (0 as i32) as i64;
        }
        unsafe {
            sqlite3VdbeSerialGet(
                unsafe {
                    p.offset(
                        (((unsafe { *unsafe { p.offset((0 as i32) as isize) } }) as u32) as i32)
                            as isize,
                    )
                },
                (unsafe { *unsafe { p.offset((1 as i32) as isize) } }) as u32,
                std::ptr::addr_of_mut!(m),
            )
        };
        0 as i32;
        r = (unsafe { m.u.i }) as f64;
    }
    return r;
}

/// Helper function for vdbeSorterCompareReal()
///
/// This routine handles the case of comparing two floating-point values
/// where one or both of the floating-point are represented by integers.
/// In other words, where one both is an MEM_RealInt.
///
/// This subroutine is factored out from vdbeSorterCompareReal() for
/// efficiency.  If inlined into vdbeSorterCompareReal(), this routine
/// will use extra stack space and consume CPU cycles setting up and
/// breaking down that stack space, even if in the common case where
/// this path is not used.
///
/// # Arguments
///
/// * `pTask` - Subtask context (for pKeyInfo)
/// * `pbKey2Cached` - True if pTask->pUnpacked is pKey2
/// * `nKey1` - Left side of comparison
/// * `nKey2` - Right side of comparison
fn vdbeSorterCompareRealInt(
    mut pTask: *mut SortSubtask,
    mut pbKey2Cached: *mut i32,
    mut pKey1: *const (),
    mut nKey1: i32,
    mut pKey2: *const (),
    mut nKey2: i32,
) -> i32 {
    let mut p1: *const u8 = pKey1 as *const u8;
    let mut p2: *const u8 = pKey2 as *const u8;
    if (((unsafe { *unsafe { p1.offset((1 as i32) as isize) } }) as u32) as i32) == (6 as i32)
        || (((unsafe { *unsafe { p2.offset((1 as i32) as isize) } }) as u32) as i32) == (6 as i32)
    {
        // 64-bit integer values cannot be represented exactly by a double so
        // must be handled by the generalized comparison function.
        return vdbeSorterCompare(pTask, pbKey2Cached, pKey1, nKey1, pKey2, nKey2);
    } else {
        let mut r1: f64 = vdbeSorterGetReal(p1);
        let mut r2: f64 = vdbeSorterGetReal(p2);
        return vdbeSorterFinishRealCompare(
            pTask,
            pbKey2Cached,
            pKey1,
            nKey1,
            pKey2,
            nKey2,
            r1,
            r2,
        );
    }
    return unsafe { std::mem::zeroed() };
}

/// Comparison function optimized for the case where the first term
/// of both keys are either MEM_Real or MEM_RealInt.
///
/// See also vdbeSorterCompareInt() for MEM_Int values and
/// vdbeSorterCompareText() for MEM_Str values.  The general
/// case is vdbeSorterCompare() which handles anything, but is slower.
///
/// # Arguments
///
/// * `pTask` - Subtask context (for pKeyInfo)
/// * `pbKey2Cached` - True if pTask->pUnpacked is pKey2
/// * `nKey1` - Left side of comparison
/// * `nKey2` - Right side of comparison
#[unsafe(link_section = ".text.slate_distinct.vdbesort.vdbeSorterCompareReal")]
extern "C-unwind" fn vdbeSorterCompareReal(
    mut pTask: *mut SortSubtask,
    mut pbKey2Cached: *mut i32,
    mut pKey1: *const (),
    mut nKey1: i32,
    mut pKey2: *const (),
    mut nKey2: i32,
) -> i32 {
    let mut p1: *const u8 = pKey1 as *const u8; // Left key record
    let mut p2: *const u8 = pKey2 as *const u8; // Right key record
    let mut x: u64 = 0 as u64; // A real value stored as an integer
    let mut r1: f64 = 0 as f64; // First element of pKey1
    let mut r2: f64 = 0 as f64; // First element of pKey2
    0 as i32; // 1-byte headers: nAllField<13
    0 as i32; // first field guaranteed numeric
    0 as i32; // first field guaranteed numeric
    if (((unsafe { *unsafe { p1.offset((1 as i32) as isize) } }) as u32) as i32) != (7 as i32)
        || (((unsafe { *unsafe { p2.offset((1 as i32) as isize) } }) as u32) as i32) != (7 as i32)
    {
        // One or both floating point values are stored as INTEGER.  This might
        // be because of the MEM_RealInt encoding.  Try to optimize that case.
        return vdbeSorterCompareRealInt(pTask, pbKey2Cached, pKey1, nKey1, pKey2, nKey2);
    }
    0 as i32;
    x = unsafe {
        sqlite3Get8byte(unsafe { p1.offset((((unsafe { *p1 }) as u32) as i32) as isize) })
    };
    {}
    0 as i32;
    unsafe {
        memcpy(
            std::ptr::addr_of_mut!(r1) as *mut (),
            std::ptr::addr_of_mut!(x) as *const (),
            8 as u64,
        )
    };
    x = unsafe {
        sqlite3Get8byte(unsafe { p2.offset((((unsafe { *p2 }) as u32) as i32) as isize) })
    };
    {}
    0 as i32;
    unsafe {
        memcpy(
            std::ptr::addr_of_mut!(r2) as *mut (),
            std::ptr::addr_of_mut!(x) as *const (),
            8 as u64,
        )
    };
    return vdbeSorterFinishRealCompare(pTask, pbKey2Cached, pKey1, nKey1, pKey2, nKey2, r1, r2);
}

/// Initialize the temporary index cursor just opened as a sorter cursor.
///
/// Usually, the sorter module uses the value of (pCsr->pKeyInfo->nKeyField)
/// to determine the number of fields that should be compared from the
/// records being sorted. However, if the value passed as argument nField
/// is non-zero and the sorter is able to guarantee a stable sort, nField
/// is used instead. This is used when sorting records for a CREATE INDEX
/// statement. In this case, keys are always delivered to the sorter in
/// order of the primary key, which happens to be make up the final part
/// of the records being sorted. So if the sort is stable, there is never
/// any reason to compare PK fields and they can be ignored for a small
/// performance boost.
///
/// The sorter can guarantee a stable sort when running in single-threaded
/// mode, but not in multi-threaded mode.
///
/// SQLITE_OK is returned if successful, or an SQLite error code otherwise.
///
/// # Arguments
///
/// * `db` - Database connection (for malloc())
/// * `nField` - Number of key fields in each record
/// * `pCsr` - Cursor that holds the new sorter
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3VdbeSorterInit(
    mut db: *mut sqlite3,
    mut nField: i32,
    mut pCsr: *mut VdbeCursor,
) -> i32 {
    let mut pgsz: i32 = 0 as i32; // Page size of main database
    let mut i: i32 = 0 as i32; // Used to iterate through aTask[]
    let mut pSorter: *mut VdbeSorter = unsafe { std::mem::zeroed() }; // The new sorter
    let mut pKeyInfo: *mut KeyInfo = unsafe { std::mem::zeroed() }; // Copy of pCsr->pKeyInfo with db==0
    let mut szKeyInfo: i32 = 0 as i32; // Size of pCsr->pKeyInfo in bytes
    let mut sz: i64 = 0 as i64; // Size of pSorter in bytes
    let mut rc: i32 = 0 as i32;
    let mut nWorker: i32 = 0 as i32;
    // Initialize the upper limit on the number of worker threads
    if (unsafe { sqlite3TempInMemory(db as *const sqlite3) }) != (0 as i32)
        || (((unsafe { sqlite3Config.bCoreMutex }) as u32) as i32) == (0 as i32)
    {
        nWorker = 0 as i32;
    } else {
        nWorker = unsafe {
            *unsafe {
                unsafe { (*db).aLimit.as_mut_ptr() as *mut i32 }.offset((11 as i32) as isize)
            }
        };
    }
    // Do not allow the total number of threads (main thread + all workers)
    // to exceed the maximum merge count
    0 as i32;
    0 as i32;
    0 as i32;
    0 as i32;
    0 as i32;
    szKeyInfo = ((32 as u64).wrapping_add(
        (((((unsafe { (*unsafe { (*pCsr).pKeyInfo }).nAllField }) as u32) as i32) as i64) as u64)
            .wrapping_mul(8 as u64),
    ) as u32) as i32;
    sz = (96 as u64).wrapping_add((((nWorker + (1 as i32)) as i64) as u64).wrapping_mul(104 as u64))
        as i64;
    pSorter =
        (unsafe { sqlite3DbMallocZero(db, (sz + (szKeyInfo as i64)) as u64) }) as *mut VdbeSorter;
    unsafe {
        (*pCsr).uc.pSorter = pSorter;
    }
    if pSorter == std::ptr::null_mut::<VdbeSorter>() {
        rc = 7 as i32;
    } else {
        let mut pBt: *mut Btree =
            unsafe { (*unsafe { unsafe { (*db).aDb }.offset((0 as i32) as isize) }).pBt };
        let __v973: *mut KeyInfo =
            (unsafe { (pSorter as *mut u8).offset(sz as isize) }) as *mut KeyInfo;
        pKeyInfo = __v973;
        unsafe {
            (*pSorter).pKeyInfo = __v973;
        }
        unsafe {
            memcpy(
                pKeyInfo as *mut (),
                (unsafe { (*pCsr).pKeyInfo }) as *const (),
                (szKeyInfo as i64) as u64,
            )
        };
        unsafe {
            (*pKeyInfo).db = std::ptr::null_mut::<sqlite3>();
        }
        if nField != (0 as i32) && nWorker == (0 as i32) {
            unsafe {
                (*pKeyInfo).nKeyField = (nField as i16) as u16;
            }
            0 as i32;
        }
        // It is OK that pKeyInfo reuses the aSortFlags field from pCsr->pKeyInfo,
        // since the pCsr->pKeyInfo->aSortFlags[] array is invariant and lives
        // longer that pSorter.
        0 as i32;
        unsafe { sqlite3BtreeEnter(pBt) };
        let __v974: i32 = unsafe { sqlite3BtreeGetPageSize(pBt) };
        pgsz = __v974;
        unsafe {
            (*pSorter).pgsz = __v974;
        }
        unsafe { sqlite3BtreeLeave(pBt) };
        unsafe {
            (*pSorter).nTask = ((nWorker + (1 as i32)) as i8) as u8;
        }
        unsafe {
            (*pSorter).iPrev = ((nWorker - (1 as i32)) as i8) as u8;
        }
        unsafe {
            (*pSorter).bUseThreads =
                ((((unsafe { (*pSorter).nTask }) as u32) as i32) > (1 as i32)) as u8;
        }
        unsafe {
            (*pSorter).db = db;
        }
        i = 0 as i32;
        '__slate_break_942: loop {
            if !(i < (((unsafe { (*pSorter).nTask }) as u32) as i32)) {
                break;
            }
            let mut pTask: *mut SortSubtask = unsafe {
                unsafe { std::ptr::addr_of_mut!((*pSorter).aTask) as *mut SortSubtask }
                    .offset(i as isize)
            };
            unsafe {
                (*pTask).pSorter = pSorter;
            }
            let __v975: i32 = i;
            let __v976: i32 = __v975 + (1 as i32);
            i = __v976;
        }
        if !((unsafe { sqlite3TempInMemory(db as *const sqlite3) }) != (0 as i32)) {
            let mut mxCache: i64 = 0 as i64; // Cache size in bytes
            let mut szPma: u32 = unsafe { sqlite3Config.szPma };
            unsafe {
                (*pSorter).mnPmaSize = szPma.wrapping_mul(pgsz as u32) as i32;
            }
            mxCache = (unsafe {
                (*unsafe { (*unsafe { unsafe { (*db).aDb }.offset((0 as i32) as isize) }).pSchema })
                    .cache_size
            }) as i64;
            if mxCache < ((0 as i32) as i64) {
                // A negative cache-size value C indicates that the cache is abs(C)
                // KiB in size.
                mxCache = mxCache * (-(1024 as i32) as i64);
            } else {
                mxCache = mxCache * (pgsz as i64);
            }
            mxCache = if mxCache < (((1 as i32) << (29 as i32)) as i64) {
                mxCache
            } else {
                ((1 as i32) << (29 as i32)) as i64
            };
            unsafe {
                (*pSorter).mxPmaSize = if (unsafe { (*pSorter).mnPmaSize }) > (mxCache as i32) {
                    unsafe { (*pSorter).mnPmaSize }
                } else {
                    mxCache as i32
                };
            }
            // Avoid large memory allocations if the application has requested
            // SQLITE_CONFIG_SMALL_MALLOC.
            if (((unsafe { sqlite3Config.bSmallMalloc }) as u32) as i32) == (0 as i32) {
                0 as i32;
                unsafe {
                    (*pSorter).nMemory = pgsz;
                }
                unsafe {
                    (*pSorter).list.aMemory =
                        (unsafe { sqlite3Malloc((pgsz as i64) as u64) }) as *mut u8;
                }
                if !((unsafe { (*pSorter).list.aMemory }) != std::ptr::null_mut::<u8>()) {
                    rc = 7 as i32;
                }
            }
        }
        if (((unsafe { (*pKeyInfo).nAllField }) as u32) as i32) < (13 as i32)
            && ((unsafe {
                *unsafe {
                    unsafe { std::ptr::addr_of_mut!((*pKeyInfo).aColl) as *mut *mut CollSeq }
                        .offset((0 as i32) as isize)
                }
            }) == std::ptr::null_mut::<CollSeq>()
                || (unsafe {
                    *unsafe {
                        unsafe { std::ptr::addr_of_mut!((*pKeyInfo).aColl) as *mut *mut CollSeq }
                            .offset((0 as i32) as isize)
                    }
                }) == unsafe { (*db).pDfltColl })
            && (((unsafe {
                *unsafe { unsafe { (*pKeyInfo).aSortFlags }.offset((0 as i32) as isize) }
            }) as u32) as i32)
                & (2 as i32)
                == (0 as i32)
        {
            unsafe {
                (*pSorter).typeMask = (((1 as i32) | (2 as i32) | (4 as i32)) as i8) as u8;
            }
        }
    }
    return rc;
}

// Defined at the top of this function
/// Free the list of sorted records starting at pRecord.
fn vdbeSorterRecordFree(mut db: *mut sqlite3, mut pRecord: *mut SorterRecord) {
    let mut p: *mut SorterRecord = unsafe { std::mem::zeroed() };
    let mut pNext: *mut SorterRecord = unsafe { std::mem::zeroed() };
    p = pRecord;
    '__slate_break_943: while p != std::ptr::null_mut::<SorterRecord>() {
        pNext = unsafe { (*p).u.pNext };
        unsafe { sqlite3DbFree(db, p as *mut ()) };
        p = pNext;
    }
}

/// Free all resources owned by the object indicated by argument pTask. All
/// fields of *pTask are zeroed before returning.
fn vdbeSortSubtaskCleanup(mut db: *mut sqlite3, mut pTask: *mut SortSubtask) {
    unsafe { sqlite3DbFree(db, (unsafe { (*pTask).pUnpacked }) as *mut ()) };
    // pTask->list.aMemory can only be non-zero if it was handed memory
    // from the main thread.  That only occurs SQLITE_MAX_WORKER_THREADS>0
    if (unsafe { (*pTask).list.aMemory }) != std::ptr::null_mut::<u8>() {
        unsafe { sqlite3_free((unsafe { (*pTask).list.aMemory }) as *mut ()) };
    } else {
        0 as i32;
        vdbeSorterRecordFree(std::ptr::null_mut::<sqlite3>(), unsafe {
            (*pTask).list.pList
        });
    }
    if (unsafe { (*pTask).file.pFd }) != std::ptr::null_mut::<sqlite3_file>() {
        unsafe { sqlite3OsCloseFree(unsafe { (*pTask).file.pFd }) };
    }
    if (unsafe { (*pTask).file2.pFd }) != std::ptr::null_mut::<sqlite3_file>() {
        unsafe { sqlite3OsCloseFree(unsafe { (*pTask).file2.pFd }) };
    }
    unsafe { memset(pTask as *mut (), 0 as i32, 104 as u64) };
}

/// Join thread pTask->thread.
fn vdbeSorterJoinThread(mut pTask: *mut SortSubtask) -> i32 {
    let mut rc: i32 = 0 as i32;
    if (unsafe { (*pTask).pThread }) != std::ptr::null_mut::<SQLiteThread>() {
        let mut pRet: *mut () = ((1 as i32) as i64) as *mut ();
        {}
        unsafe { sqlite3ThreadJoin(unsafe { (*pTask).pThread }, std::ptr::addr_of_mut!(pRet)) };
        {}
        rc = (pRet as i64) as i32;
        0 as i32;
        unsafe {
            (*pTask).bDone = 0 as i32;
        }
        unsafe {
            (*pTask).pThread = std::ptr::null_mut::<SQLiteThread>();
        }
    }
    return rc;
}

/// Launch a background thread to run xTask(pIn).
///
/// # Arguments
///
/// * `pTask` - Thread will use this task object
/// * `xTask` - Routine to run in a separate thread
/// * `pIn` - Argument passed into xTask()
fn vdbeSorterCreateThread(
    mut pTask: *mut SortSubtask,
    mut xTask: Option<unsafe extern "C-unwind" fn(*mut ()) -> *mut ()>,
    mut pIn: *mut (),
) -> i32 {
    0 as i32;
    return unsafe {
        sqlite3ThreadCreate(
            unsafe { std::ptr::addr_of_mut!((*pTask).pThread) },
            xTask,
            pIn,
        )
    };
}

/// Join all outstanding threads launched by SorterWrite() to create
/// level-0 PMAs.
fn vdbeSorterJoinAll(mut pSorter: *mut VdbeSorter, mut rcin: i32) -> i32 {
    let mut rc: i32 = rcin;
    let mut i: i32 = 0 as i32;
    // This function is always called by the main user thread.
    //
    // If this function is being called after SorterRewind() has been called,
    // it is possible that thread pSorter->aTask[pSorter->nTask-1].pThread
    // is currently attempt to join one of the other threads. To avoid a race
    // condition where this thread also attempts to join the same object, join
    // thread pSorter->aTask[pSorter->nTask-1].pThread first.
    i = (((unsafe { (*pSorter).nTask }) as u32) as i32) - (1 as i32);
    '__slate_break_944: loop {
        if !(i >= (0 as i32)) {
            break;
        }
        let mut pTask: *mut SortSubtask = unsafe {
            unsafe { std::ptr::addr_of_mut!((*pSorter).aTask) as *mut SortSubtask }
                .offset(i as isize)
        };
        let mut rc2: i32 = vdbeSorterJoinThread(pTask);
        if rc == (0 as i32) {
            rc = rc2;
        }
        let __v1029: i32 = i;
        let __v1030: i32 = __v1029 - (1 as i32);
        i = __v1030;
    }
    return rc;
}

/// Allocate a new MergeEngine object capable of handling up to
/// nReader PmaReader inputs.
///
/// nReader is automatically rounded up to the next power of two.
/// nReader may not exceed SORTER_MAX_MERGE_COUNT even after rounding up.
fn vdbeMergeEngineNew(mut nReader: i32) -> *mut MergeEngine {
    let mut N: i32 = 2 as i32; // Smallest power of two >= nReader
    let mut nByte: i64 = 0 as i64; // Total bytes of space to allocate
    let mut pNew: *mut MergeEngine = unsafe { std::mem::zeroed() }; // Pointer to allocated object to return
    0 as i32;
    '__slate_break_945: while N < nReader {
        let __v1031: i32 = N;
        let __v1032: i32 = __v1031 + N;
        N = __v1032;
    }
    nByte = (32 as u64)
        .wrapping_add(((N as i64) as u64).wrapping_mul((4 as u64).wrapping_add(80 as u64)))
        as i64;
    let __v1033: *mut MergeEngine;
    if (unsafe { sqlite3FaultSim(100 as i32) }) != (0 as i32) {
        __v1033 = std::ptr::null_mut::<MergeEngine>();
    } else {
        __v1033 = (unsafe { sqlite3MallocZero(nByte as u64) }) as *mut MergeEngine;
    }
    pNew = __v1033;
    if pNew != std::ptr::null_mut::<MergeEngine>() {
        unsafe {
            (*pNew).nTree = N;
        }
        unsafe {
            (*pNew).pTask = std::ptr::null_mut::<SortSubtask>();
        }
        unsafe {
            (*pNew).aReadr = (unsafe { pNew.offset((1 as i32) as isize) }) as *mut PmaReader;
        }
        unsafe {
            (*pNew).aTree = (unsafe { unsafe { (*pNew).aReadr }.offset(N as isize) }) as *mut i32;
        }
    }
    return pNew;
}

/// Free the MergeEngine object passed as the only argument.
fn vdbeMergeEngineFree(mut pMerger: *mut MergeEngine) {
    let mut i: i32 = 0 as i32;
    if pMerger != std::ptr::null_mut::<MergeEngine>() {
        i = 0 as i32;
        '__slate_break_946: loop {
            if !(i < unsafe { (*pMerger).nTree }) {
                break;
            }
            vdbePmaReaderClear(unsafe { unsafe { (*pMerger).aReadr }.offset(i as isize) });
            let __v1034: i32 = i;
            let __v1035: i32 = __v1034 + (1 as i32);
            i = __v1035;
        }
    }
    unsafe { sqlite3_free(pMerger as *mut ()) };
}

/// Free all resources associated with the IncrMerger object indicated by
/// the first argument.
fn vdbeIncrFree(mut pIncr: *mut IncrMerger) {
    if pIncr != std::ptr::null_mut::<IncrMerger>() {
        if (unsafe { (*pIncr).bUseThread }) != (0 as i32) {
            vdbeSorterJoinThread(unsafe { (*pIncr).pTask });
            if (unsafe {
                (*unsafe {
                    unsafe { (*pIncr).aFile.as_mut_ptr() as *mut SorterFile }
                        .offset((0 as i32) as isize)
                })
                .pFd
            }) != std::ptr::null_mut::<sqlite3_file>()
            {
                unsafe {
                    sqlite3OsCloseFree(unsafe {
                        (*unsafe {
                            unsafe { (*pIncr).aFile.as_mut_ptr() as *mut SorterFile }
                                .offset((0 as i32) as isize)
                        })
                        .pFd
                    })
                };
            }
            if (unsafe {
                (*unsafe {
                    unsafe { (*pIncr).aFile.as_mut_ptr() as *mut SorterFile }
                        .offset((1 as i32) as isize)
                })
                .pFd
            }) != std::ptr::null_mut::<sqlite3_file>()
            {
                unsafe {
                    sqlite3OsCloseFree(unsafe {
                        (*unsafe {
                            unsafe { (*pIncr).aFile.as_mut_ptr() as *mut SorterFile }
                                .offset((1 as i32) as isize)
                        })
                        .pFd
                    })
                };
            }
        }
        vdbeMergeEngineFree(unsafe { (*pIncr).pMerger });
        unsafe { sqlite3_free(pIncr as *mut ()) };
    }
}

/// Reset a sorting cursor back to its original empty state.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3VdbeSorterReset(mut db: *mut sqlite3, mut pSorter: *mut VdbeSorter) {
    let mut i: i32 = 0 as i32;
    vdbeSorterJoinAll(pSorter, 0 as i32);
    0 as i32;
    if (unsafe { (*pSorter).pReader }) != std::ptr::null_mut::<PmaReader>() {
        vdbePmaReaderClear(unsafe { (*pSorter).pReader });
        unsafe { sqlite3DbFree(db, (unsafe { (*pSorter).pReader }) as *mut ()) };
        unsafe {
            (*pSorter).pReader = std::ptr::null_mut::<PmaReader>();
        }
    }
    vdbeMergeEngineFree(unsafe { (*pSorter).pMerger });
    unsafe {
        (*pSorter).pMerger = std::ptr::null_mut::<MergeEngine>();
    }
    i = 0 as i32;
    '__slate_break_947: loop {
        if !(i < (((unsafe { (*pSorter).nTask }) as u32) as i32)) {
            break;
        }
        let mut pTask: *mut SortSubtask = unsafe {
            unsafe { std::ptr::addr_of_mut!((*pSorter).aTask) as *mut SortSubtask }
                .offset(i as isize)
        };
        vdbeSortSubtaskCleanup(db, pTask);
        unsafe {
            (*pTask).pSorter = pSorter;
        }
        let __v977: i32 = i;
        let __v978: i32 = __v977 + (1 as i32);
        i = __v978;
    }
    if (unsafe { (*pSorter).list.aMemory }) == std::ptr::null_mut::<u8>() {
        vdbeSorterRecordFree(std::ptr::null_mut::<sqlite3>(), unsafe {
            (*pSorter).list.pList
        });
    }
    unsafe {
        (*pSorter).list.pList = std::ptr::null_mut::<SorterRecord>();
    }
    unsafe {
        (*pSorter).list.szPMA = (0 as i32) as i64;
    }
    unsafe {
        (*pSorter).bUsePMA = ((0 as i32) as i8) as u8;
    }
    unsafe {
        (*pSorter).iMemory = 0 as i32;
    }
    unsafe {
        (*pSorter).mxKeysize = 0 as i32;
    }
    unsafe { sqlite3DbFree(db, (unsafe { (*pSorter).pUnpacked }) as *mut ()) };
    unsafe {
        (*pSorter).pUnpacked = std::ptr::null_mut::<UnpackedRecord>();
    }
}

/// Free any cursor components allocated by sqlite3VdbeSorterXXX routines.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3VdbeSorterClose(mut db: *mut sqlite3, mut pCsr: *mut VdbeCursor) {
    let mut pSorter: *mut VdbeSorter = unsafe { std::mem::zeroed() };
    0 as i32;
    pSorter = unsafe { (*pCsr).uc.pSorter };
    if pSorter != std::ptr::null_mut::<VdbeSorter>() {
        // Increment db->nSpill by the total number of bytes of data written
        // to temp files by this sort operation.
        let mut ii: i32 = 0 as i32;
        ii = 0 as i32;
        '__slate_break_948: loop {
            if !(ii < (((unsafe { (*pSorter).nTask }) as u32) as i32)) {
                break;
            }
            let __v981: *mut sqlite3 = db;
            let __v982: u64 = unsafe { (*__v981).nSpill };
            let __v983: u64 = __v982.wrapping_add(unsafe {
                (*unsafe {
                    unsafe { std::ptr::addr_of_mut!((*pSorter).aTask) as *mut SortSubtask }
                        .offset(ii as isize)
                })
                .nSpill
            });
            unsafe {
                (*__v981).nSpill = __v983;
            }
            let __v979: i32 = ii;
            let __v980: i32 = __v979 + (1 as i32);
            ii = __v980;
        }
        sqlite3VdbeSorterReset(db, pSorter);
        unsafe { sqlite3_free((unsafe { (*pSorter).list.aMemory }) as *mut ()) };
        unsafe { sqlite3DbFree(db, pSorter as *mut ()) };
        unsafe {
            (*pCsr).uc.pSorter = std::ptr::null_mut::<VdbeSorter>();
        }
    }
}

/// The first argument is a file-handle open on a temporary file. The file
/// is guaranteed to be nByte bytes or smaller in size. This function
/// attempts to extend the file to nByte bytes in size and to ensure that
/// the VFS has memory mapped it.
///
/// Whether or not the file does end up memory mapped of course depends on
/// the specific VFS implementation.
fn vdbeSorterExtendFile(mut db: *mut sqlite3, mut pFd: *mut sqlite3_file, mut nByte: i64) {
    if nByte <= ((unsafe { (*db).nMaxSorterMmap }) as i64)
        && (unsafe { (*unsafe { (*pFd).pMethods }).iVersion }) >= (3 as i32)
    {
        let mut p: *mut () = std::ptr::null_mut::<()>();
        let mut chunksize: i32 = (4 as i32) * (1024 as i32);
        unsafe {
            sqlite3OsFileControlHint(pFd, 6 as i32, std::ptr::addr_of_mut!(chunksize) as *mut ())
        };
        unsafe {
            sqlite3OsFileControlHint(pFd, 5 as i32, std::ptr::addr_of_mut!(nByte) as *mut ())
        };
        unsafe {
            sqlite3OsFetch(
                pFd,
                (0 as i32) as i64,
                nByte as i32,
                std::ptr::addr_of_mut!(p),
            )
        };
        if p != std::ptr::null_mut::<()>() {
            unsafe { sqlite3OsUnfetch(pFd, (0 as i32) as i64, p) };
        }
    }
}

/// Allocate space for a file-handle and open a temporary file. If successful,
/// set *ppFd to point to the malloc'd file-handle and return SQLITE_OK.
/// Otherwise, set *ppFd to 0 and return an SQLite error code.
///
/// # Arguments
///
/// * `db` - Database handle doing sort
/// * `nExtend` - Attempt to extend file to this size
fn vdbeSorterOpenTempFile(
    mut db: *mut sqlite3,
    mut nExtend: i64,
    mut ppFd: *mut *mut sqlite3_file,
) -> i32 {
    let mut rc: i32 = 0 as i32;
    if (unsafe { sqlite3FaultSim(202 as i32) }) != (0 as i32) {
        return (10 as i32) | (13 as i32) << (8 as i32);
    }
    rc = unsafe {
        sqlite3OsOpenMalloc(
            unsafe { (*db).pVfs },
            std::ptr::null::<i8>(),
            ppFd,
            (4096 as i32) | (2 as i32) | (4 as i32) | (16 as i32) | (8 as i32),
            std::ptr::addr_of_mut!(rc),
        )
    };
    if rc == (0 as i32) {
        let mut max: i64 = (2147418112 as i32) as i64;
        unsafe {
            sqlite3OsFileControlHint(
                unsafe { *ppFd },
                18 as i32,
                std::ptr::addr_of_mut!(max) as *mut (),
            )
        };
        if nExtend > ((0 as i32) as i64) {
            vdbeSorterExtendFile(db, unsafe { *ppFd }, nExtend);
        }
    }
    return rc;
}

/// If it has not already been allocated, allocate the UnpackedRecord
/// structure at pTask->pUnpacked. Return SQLITE_OK if successful (or
/// if no allocation was required), or SQLITE_NOMEM otherwise.
fn vdbeSortAllocUnpacked(mut pTask: *mut SortSubtask) -> i32 {
    if (unsafe { (*pTask).pUnpacked }) == std::ptr::null_mut::<UnpackedRecord>() {
        unsafe {
            (*pTask).pUnpacked = unsafe {
                sqlite3VdbeAllocUnpackedRecord(unsafe { (*unsafe { (*pTask).pSorter }).pKeyInfo })
            };
        }
        if (unsafe { (*pTask).pUnpacked }) == std::ptr::null_mut::<UnpackedRecord>() {
            return 7 as i32;
        }
        unsafe {
            (*unsafe { (*pTask).pUnpacked }).nField =
                unsafe { (*unsafe { (*unsafe { (*pTask).pSorter }).pKeyInfo }).nKeyField };
        }
        unsafe {
            (*unsafe { (*pTask).pUnpacked }).errCode = ((0 as i32) as i8) as u8;
        }
    }
    return 0 as i32;
}

/// Merge the two sorted lists p1 and p2 into a single list.
///
/// # Arguments
///
/// * `pTask` - Calling thread context
/// * `p1` - First list to merge
/// * `p2` - Second list to merge
fn vdbeSorterMerge(
    mut pTask: *mut SortSubtask,
    mut p1: *mut SorterRecord,
    mut p2: *mut SorterRecord,
) -> *mut SorterRecord {
    let mut pFinal: *mut SorterRecord = std::ptr::null_mut::<SorterRecord>();
    let mut pp: *mut *mut SorterRecord = std::ptr::addr_of_mut!(pFinal);
    let mut bCached: i32 = 0 as i32;
    0 as i32;
    '__slate_break_949: loop {
        let mut res: i32 = 0 as i32;
        res = unsafe {
            unsafe { (*pTask).xCompare }.unwrap()(
                pTask,
                std::ptr::addr_of_mut!(bCached),
                ((unsafe { p1.offset((1 as i32) as isize) }) as *mut ()) as *const (),
                unsafe { (*p1).nVal },
                ((unsafe { p2.offset((1 as i32) as isize) }) as *mut ()) as *const (),
                unsafe { (*p2).nVal },
            )
        };
        if res <= (0 as i32) {
            unsafe {
                *pp = p1;
            }
            pp = unsafe { std::ptr::addr_of_mut!((*p1).u.pNext) };
            p1 = unsafe { (*p1).u.pNext };
            if p1 == std::ptr::null_mut::<SorterRecord>() {
                unsafe {
                    *pp = p2;
                }
                break '__slate_break_949;
            }
        } else {
            unsafe {
                *pp = p2;
            }
            pp = unsafe { std::ptr::addr_of_mut!((*p2).u.pNext) };
            p2 = unsafe { (*p2).u.pNext };
            bCached = 0 as i32;
            if p2 == std::ptr::null_mut::<SorterRecord>() {
                unsafe {
                    *pp = p1;
                }
                break '__slate_break_949;
            }
        }
    }
    return pFinal;
}

/// Return the SorterCompare function to compare values collected by the
/// sorter object passed as the only argument.
fn vdbeSorterGetCompare(
    mut p: *mut VdbeSorter,
) -> Option<
    unsafe extern "C-unwind" fn(*mut SortSubtask, *mut i32, *const (), i32, *const (), i32) -> i32,
> {
    if (((unsafe { (*p).typeMask }) as u32) as i32) & (1 as i32) != (0 as i32) {
        return Some(vdbeSorterCompareInt);
    } else {
        if (((unsafe { (*p).typeMask }) as u32) as i32) & (2 as i32) != (0 as i32) {
            return Some(vdbeSorterCompareText);
        } else {
            if (((unsafe { (*p).typeMask }) as u32) as i32) & (4 as i32) != (0 as i32) {
                return Some(vdbeSorterCompareReal);
            }
        }
    }
    return Some(vdbeSorterCompare);
}

/// Sort the linked list of records headed at pTask->pList. Return
/// SQLITE_OK if successful, or an SQLite error code (i.e. SQLITE_NOMEM) if
/// an error occurs.
fn vdbeSorterSort(mut pTask: *mut SortSubtask, mut pList: *mut SorterList) -> i32 {
    let mut i: i32 = 0 as i32;
    let mut p: *mut SorterRecord = unsafe { std::mem::zeroed() };
    let mut rc: i32 = 0 as i32;
    let mut aSlot: __SlateAlign16<[*mut SorterRecord; 64]> =
        __SlateAlign16([0 as *mut SorterRecord; 64]);
    rc = vdbeSortAllocUnpacked(pTask);
    if rc != (0 as i32) {
        return rc;
    }
    p = unsafe { (*pList).pList };
    unsafe {
        (*pTask).xCompare = vdbeSorterGetCompare(unsafe { (*pTask).pSorter });
    }
    unsafe {
        memset(
            (aSlot.0.as_mut_ptr() as *mut *mut SorterRecord) as *mut (),
            0 as i32,
            512 as u64,
        )
    };
    '__slate_break_950: while p != std::ptr::null_mut::<SorterRecord>() {
        let mut pNext: *mut SorterRecord = unsafe { std::mem::zeroed() };
        if (unsafe { (*pList).aMemory }) != std::ptr::null_mut::<u8>() {
            if (p as *mut u8) == unsafe { (*pList).aMemory } {
                pNext = std::ptr::null_mut::<SorterRecord>();
            } else {
                0 as i32;
                pNext = (unsafe {
                    unsafe { (*pList).aMemory }.offset((unsafe { (*p).u.iNext }) as isize)
                }) as *mut SorterRecord;
            }
        } else {
            pNext = unsafe { (*p).u.pNext };
        }
        unsafe {
            (*p).u.pNext = std::ptr::null_mut::<SorterRecord>();
        }
        i = 0 as i32;
        '__slate_break_951: loop {
            if !((unsafe {
                *unsafe { (aSlot.0.as_mut_ptr() as *mut *mut SorterRecord).offset(i as isize) }
            }) != std::ptr::null_mut::<SorterRecord>())
            {
                break;
            }
            p = vdbeSorterMerge(pTask, p, unsafe {
                *unsafe { (aSlot.0.as_mut_ptr() as *mut *mut SorterRecord).offset(i as isize) }
            });
            // ,--Each aSlot[] holds twice as much as the previous. So we cannot use
            // |  up all 64 aSlots[] with only a 64-bit address space.
            // v
            0 as i32;
            unsafe {
                *unsafe { (aSlot.0.as_mut_ptr() as *mut *mut SorterRecord).offset(i as isize) } =
                    std::ptr::null_mut::<SorterRecord>();
            }
            let __v1036: i32 = i;
            let __v1037: i32 = __v1036 + (1 as i32);
            i = __v1037;
        }
        unsafe {
            *unsafe { (aSlot.0.as_mut_ptr() as *mut *mut SorterRecord).offset(i as isize) } = p;
        }
        p = pNext;
    }
    p = std::ptr::null_mut::<SorterRecord>();
    i = 0 as i32;
    '__slate_break_952: loop {
        if !(i < ((((512 as u64) / (8 as u64)) as u32) as i32)) {
            break;
        }
        if (unsafe {
            *unsafe { (aSlot.0.as_mut_ptr() as *mut *mut SorterRecord).offset(i as isize) }
        }) == std::ptr::null_mut::<SorterRecord>()
        {
        } else {
            let __v1040: *mut SorterRecord;
            if p != std::ptr::null_mut::<SorterRecord>() {
                __v1040 = vdbeSorterMerge(pTask, p, unsafe {
                    *unsafe { (aSlot.0.as_mut_ptr() as *mut *mut SorterRecord).offset(i as isize) }
                });
            } else {
                __v1040 = unsafe {
                    *unsafe { (aSlot.0.as_mut_ptr() as *mut *mut SorterRecord).offset(i as isize) }
                };
            }
            p = __v1040;
        }
        let __v1038: i32 = i;
        let __v1039: i32 = __v1038 + (1 as i32);
        i = __v1039;
    }
    unsafe {
        (*pList).pList = p;
    }
    0 as i32;
    return ((unsafe { (*unsafe { (*pTask).pUnpacked }).errCode }) as u32) as i32;
}

/// Initialize a PMA-writer object.
///
/// # Arguments
///
/// * `pFd` - File handle to write to
/// * `p` - Object to populate
/// * `nBuf` - Buffer size
/// * `iStart` - Offset of pFd to begin writing at
fn vdbePmaWriterInit(
    mut pFd: *mut sqlite3_file,
    mut p: *mut PmaWriter,
    mut nBuf: i32,
    mut iStart: i64,
) {
    unsafe { memset(p as *mut (), 0 as i32, 56 as u64) };
    unsafe {
        (*p).aBuffer = (unsafe { sqlite3Malloc((nBuf as i64) as u64) }) as *mut u8;
    }
    if !((unsafe { (*p).aBuffer }) != std::ptr::null_mut::<u8>()) {
        unsafe {
            (*p).eFWErr = 7 as i32;
        }
    } else {
        let __v1041: i32 = (iStart % (nBuf as i64)) as i32;
        unsafe {
            (*p).iBufStart = __v1041;
        }
        unsafe {
            (*p).iBufEnd = __v1041;
        }
        unsafe {
            (*p).iWriteOff = iStart - ((unsafe { (*p).iBufStart }) as i64);
        }
        unsafe {
            (*p).nBuffer = nBuf;
        }
        unsafe {
            (*p).pFd = pFd;
        }
    }
}

/// Write nData bytes of data to the PMA. Return SQLITE_OK
/// if successful, or an SQLite error code if an error occurs.
fn vdbePmaWriteBlob(mut p: *mut PmaWriter, mut pData: *mut u8, mut nData: i32) {
    let mut nRem: i32 = nData;
    '__slate_break_953: while nRem > (0 as i32) && (unsafe { (*p).eFWErr }) == (0 as i32) {
        let mut nCopy: i32 = nRem;
        if nCopy > (unsafe { (*p).nBuffer }) - unsafe { (*p).iBufEnd } {
            nCopy = (unsafe { (*p).nBuffer }) - unsafe { (*p).iBufEnd };
        }
        unsafe {
            memcpy(
                (unsafe { unsafe { (*p).aBuffer }.offset((unsafe { (*p).iBufEnd }) as isize) })
                    as *mut (),
                (unsafe { pData.offset((nData - nRem) as isize) }) as *const (),
                (nCopy as i64) as u64,
            )
        };
        let __v1042: *mut PmaWriter = p;
        let __v1043: i32 = unsafe { (*__v1042).iBufEnd };
        let __v1044: i32 = __v1043 + nCopy;
        unsafe {
            (*__v1042).iBufEnd = __v1044;
        }
        if (unsafe { (*p).iBufEnd }) == unsafe { (*p).nBuffer } {
            unsafe {
                (*p).eFWErr = unsafe {
                    sqlite3OsWrite(
                        unsafe { (*p).pFd },
                        (unsafe {
                            unsafe { (*p).aBuffer }.offset((unsafe { (*p).iBufStart }) as isize)
                        }) as *const (),
                        (unsafe { (*p).iBufEnd }) - unsafe { (*p).iBufStart },
                        (unsafe { (*p).iWriteOff }) + ((unsafe { (*p).iBufStart }) as i64),
                    )
                };
            }
            let __v1045: *mut PmaWriter = p;
            let __v1046: u64 = unsafe { (*__v1045).nPmaSpill };
            let __v1047: u64 = __v1046.wrapping_add(
                (((unsafe { (*p).iBufEnd }) - unsafe { (*p).iBufStart }) as i64) as u64,
            );
            unsafe {
                (*__v1045).nPmaSpill = __v1047;
            }
            unsafe {
                (*p).iBufEnd = 0 as i32;
            }
            unsafe {
                (*p).iBufStart = 0 as i32;
            }
            let __v1048: *mut PmaWriter = p;
            let __v1049: i64 = unsafe { (*__v1048).iWriteOff };
            let __v1050: i64 = __v1049 + ((unsafe { (*p).nBuffer }) as i64);
            unsafe {
                (*__v1048).iWriteOff = __v1050;
            }
        }
        0 as i32;
        let __v1051: i32 = nRem;
        let __v1052: i32 = __v1051 - nCopy;
        nRem = __v1052;
    }
}

/// Flush any buffered data to disk and clean up the PMA-writer object.
/// The results of using the PMA-writer after this call are undefined.
/// Return SQLITE_OK if flushing the buffered data succeeds or is not
/// required. Otherwise, return an SQLite error code.
///
/// Before returning, set *piEof to the offset immediately following the
/// last byte written to the file. Also, increment (*pnSpill) by the total
/// number of bytes written to the file.
fn vdbePmaWriterFinish(mut p: *mut PmaWriter, mut piEof: *mut i64, mut pnSpill: *mut u64) -> i32 {
    let mut rc: i32 = 0 as i32;
    if (unsafe { (*p).eFWErr }) == (0 as i32)
        && (unsafe { (*p).aBuffer }) != std::ptr::null_mut::<u8>()
        && (unsafe { (*p).iBufEnd }) > unsafe { (*p).iBufStart }
    {
        unsafe {
            (*p).eFWErr = unsafe {
                sqlite3OsWrite(
                    unsafe { (*p).pFd },
                    (unsafe {
                        unsafe { (*p).aBuffer }.offset((unsafe { (*p).iBufStart }) as isize)
                    }) as *const (),
                    (unsafe { (*p).iBufEnd }) - unsafe { (*p).iBufStart },
                    (unsafe { (*p).iWriteOff }) + ((unsafe { (*p).iBufStart }) as i64),
                )
            };
        }
        let __v1053: *mut PmaWriter = p;
        let __v1054: u64 = unsafe { (*__v1053).nPmaSpill };
        let __v1055: u64 = __v1054
            .wrapping_add((((unsafe { (*p).iBufEnd }) - unsafe { (*p).iBufStart }) as i64) as u64);
        unsafe {
            (*__v1053).nPmaSpill = __v1055;
        }
    }
    unsafe {
        *piEof = (unsafe { (*p).iWriteOff }) + ((unsafe { (*p).iBufEnd }) as i64);
    }
    let __v1056: *mut u64 = pnSpill;
    let __v1057: u64 = unsafe { *__v1056 };
    let __v1058: u64 = __v1057.wrapping_add(unsafe { (*p).nPmaSpill });
    unsafe {
        *__v1056 = __v1058;
    }
    unsafe { sqlite3_free((unsafe { (*p).aBuffer }) as *mut ()) };
    rc = unsafe { (*p).eFWErr };
    unsafe { memset(p as *mut (), 0 as i32, 56 as u64) };
    return rc;
}

/// Write value iVal encoded as a varint to the PMA. Return
/// SQLITE_OK if successful, or an SQLite error code if an error occurs.
fn vdbePmaWriteVarint(mut p: *mut PmaWriter, mut iVal: u64) {
    let mut nByte: i32 = 0 as i32;
    let mut aByte: [u8; 10] = [0 as u8; 10];
    nByte = unsafe { sqlite3PutVarint(aByte.as_mut_ptr() as *mut u8, iVal) };
    vdbePmaWriteBlob(p, aByte.as_mut_ptr() as *mut u8, nByte);
}

/// Write the current contents of in-memory linked-list pList to a level-0
/// PMA in the temp file belonging to sub-task pTask. Return SQLITE_OK if
/// successful, or an SQLite error code otherwise.
///
/// The format of a PMA is:
///
///     * A varint. This varint contains the total number of bytes of content
///       in the PMA (not including the varint itself).
///
///     * One or more records packed end-to-end in order of ascending keys.
///       Each record consists of a varint followed by a blob of data (the
///       key). The varint is the number of bytes in the blob of data.
fn vdbeSorterListToPMA(mut pTask: *mut SortSubtask, mut pList: *mut SorterList) -> i32 {
    let mut db: *mut sqlite3 = unsafe { (*unsafe { (*pTask).pSorter }).db };
    let mut rc: i32 = 0 as i32; // Return code
    let mut writer: PmaWriter = unsafe { std::mem::zeroed() }; // Object used to write to the file
    {}
    unsafe {
        memset(
            std::ptr::addr_of_mut!(writer) as *mut (),
            0 as i32,
            56 as u64,
        )
    };
    0 as i32;
    // If the first temporary PMA file has not been opened, open it now.
    if (unsafe { (*pTask).file.pFd }) == std::ptr::null_mut::<sqlite3_file>() {
        rc = vdbeSorterOpenTempFile(db, (0 as i32) as i64, unsafe {
            std::ptr::addr_of_mut!((*pTask).file.pFd)
        });
        0 as i32;
        0 as i32;
        0 as i32;
    }
    // Try to get the file to memory map
    if rc == (0 as i32) {
        vdbeSorterExtendFile(
            db,
            unsafe { (*pTask).file.pFd },
            (unsafe { (*pTask).file.iEof }) + unsafe { (*pList).szPMA } + ((9 as i32) as i64),
        );
    }
    // Sort the list
    if rc == (0 as i32) {
        rc = vdbeSorterSort(pTask, pList);
    }
    if rc == (0 as i32) {
        let mut p: *mut SorterRecord = unsafe { std::mem::zeroed() };
        let mut pNext: *mut SorterRecord = std::ptr::null_mut::<SorterRecord>();
        vdbePmaWriterInit(
            unsafe { (*pTask).file.pFd },
            std::ptr::addr_of_mut!(writer),
            unsafe { (*unsafe { (*pTask).pSorter }).pgsz },
            unsafe { (*pTask).file.iEof },
        );
        let __v1059: *mut SortSubtask = pTask;
        let __v1060: i32 = unsafe { (*__v1059).nPMA };
        let __v1061: i32 = __v1060 + (1 as i32);
        unsafe {
            (*__v1059).nPMA = __v1061;
        }
        vdbePmaWriteVarint(
            std::ptr::addr_of_mut!(writer),
            (unsafe { (*pList).szPMA }) as u64,
        );
        p = unsafe { (*pList).pList };
        '__slate_break_954: while p != std::ptr::null_mut::<SorterRecord>() {
            pNext = unsafe { (*p).u.pNext };
            vdbePmaWriteVarint(
                std::ptr::addr_of_mut!(writer),
                ((unsafe { (*p).nVal }) as i64) as u64,
            );
            vdbePmaWriteBlob(
                std::ptr::addr_of_mut!(writer),
                ((unsafe { p.offset((1 as i32) as isize) }) as *mut ()) as *mut u8,
                unsafe { (*p).nVal },
            );
            if (unsafe { (*pList).aMemory }) == std::ptr::null_mut::<u8>() {
                unsafe { sqlite3_free(p as *mut ()) };
            }
            p = pNext;
        }
        unsafe {
            (*pList).pList = p;
        }
        rc = vdbePmaWriterFinish(
            std::ptr::addr_of_mut!(writer),
            unsafe { std::ptr::addr_of_mut!((*pTask).file.iEof) },
            unsafe { std::ptr::addr_of_mut!((*pTask).nSpill) },
        );
    }
    {}
    0 as i32;
    0 as i32;
    return rc;
}

/// Advance the MergeEngine to its next entry.
/// Set *pbEof to true there is no next entry because
/// the MergeEngine has reached the end of all its inputs.
///
/// Return SQLITE_OK if successful or an error code if an error occurs.
///
/// # Arguments
///
/// * `pMerger` - The merge engine to advance to the next row
/// * `pbEof` - Set TRUE at EOF.  Set false for more content
fn vdbeMergeEngineStep(mut pMerger: *mut MergeEngine, mut pbEof: *mut i32) -> i32 {
    let mut rc: i32 = 0 as i32;
    let mut iPrev: i32 =
        unsafe { *unsafe { unsafe { (*pMerger).aTree }.offset((1 as i32) as isize) } }; // Index of PmaReader to advance
    let mut pTask: *mut SortSubtask = unsafe { (*pMerger).pTask };
    // Advance the current PmaReader
    rc = vdbePmaReaderNext(unsafe { unsafe { (*pMerger).aReadr }.offset(iPrev as isize) });
    // Update contents of aTree[]
    if rc == (0 as i32) {
        let mut i: i32 = 0 as i32; // Index of aTree[] to recalculate
        let mut pReadr1: *mut PmaReader = unsafe { std::mem::zeroed() }; // First PmaReader to compare
        let mut pReadr2: *mut PmaReader = unsafe { std::mem::zeroed() }; // Second PmaReader to compare
        let mut bCached: i32 = 0 as i32;
        // Find the first two PmaReaders to compare. The one that was just
        // advanced (iPrev) and the one next to it in the array.
        pReadr1 = unsafe { unsafe { (*pMerger).aReadr }.offset((iPrev & (65534 as i32)) as isize) };
        pReadr2 = unsafe { unsafe { (*pMerger).aReadr }.offset((iPrev | (1 as i32)) as isize) };
        i = ((unsafe { (*pMerger).nTree }) + iPrev) / (2 as i32);
        '__slate_break_955: while i > (0 as i32) {
            // Compare pReadr1 and pReadr2. Store the result in variable iRes.
            let mut iRes: i32 = 0 as i32;
            if (unsafe { (*pReadr1).pFd }) == std::ptr::null_mut::<sqlite3_file>() {
                iRes = 1 as i32;
            } else {
                if (unsafe { (*pReadr2).pFd }) == std::ptr::null_mut::<sqlite3_file>() {
                    iRes = -(1 as i32);
                } else {
                    iRes = unsafe {
                        unsafe { (*pTask).xCompare }.unwrap()(
                            pTask,
                            std::ptr::addr_of_mut!(bCached),
                            (unsafe { (*pReadr1).aKey }) as *const (),
                            unsafe { (*pReadr1).nKey },
                            (unsafe { (*pReadr2).aKey }) as *const (),
                            unsafe { (*pReadr2).nKey },
                        )
                    };
                }
            }
            // If pReadr1 contained the smaller value, set aTree[i] to its index.
            // Then set pReadr2 to the next PmaReader to compare to pReadr1. In this
            // case there is no cache of pReadr2 in pTask->pUnpacked, so set
            // pKey2 to point to the record belonging to pReadr2.
            //
            // Alternatively, if pReadr2 contains the smaller of the two values,
            // set aTree[i] to its index and update pReadr1. If vdbeSorterCompare()
            // was actually called above, then pTask->pUnpacked now contains
            // a value equivalent to pReadr2. So set pKey2 to NULL to prevent
            // vdbeSorterCompare() from decoding pReadr2 again.
            //
            // If the two values were equal, then the value from the oldest
            // PMA should be considered smaller. The VdbeSorter.aReadr[] array
            // is sorted from oldest to newest, so pReadr1 contains older values
            // than pReadr2 iff (pReadr1<pReadr2).
            if iRes < (0 as i32) || iRes == (0 as i32) && pReadr1 < pReadr2 {
                unsafe {
                    *unsafe { unsafe { (*pMerger).aTree }.offset(i as isize) } =
                        ((unsafe {
                            pReadr1.offset_from((unsafe { (*pMerger).aReadr }) as *mut PmaReader)
                        }) as i64) as i32;
                }
                pReadr2 = unsafe {
                    unsafe { (*pMerger).aReadr }.offset(
                        (unsafe {
                            *unsafe {
                                unsafe { (*pMerger).aTree }.offset((i ^ (1 as i32)) as isize)
                            }
                        }) as isize,
                    )
                };
                bCached = 0 as i32;
            } else {
                if (unsafe { (*pReadr1).pFd }) != std::ptr::null_mut::<sqlite3_file>() {
                    bCached = 0 as i32;
                }
                unsafe {
                    *unsafe { unsafe { (*pMerger).aTree }.offset(i as isize) } =
                        ((unsafe {
                            pReadr2.offset_from((unsafe { (*pMerger).aReadr }) as *mut PmaReader)
                        }) as i64) as i32;
                }
                pReadr1 = unsafe {
                    unsafe { (*pMerger).aReadr }.offset(
                        (unsafe {
                            *unsafe {
                                unsafe { (*pMerger).aTree }.offset((i ^ (1 as i32)) as isize)
                            }
                        }) as isize,
                    )
                };
            }
            i = i / (2 as i32);
        }
        unsafe {
            *pbEof = ((unsafe {
                (*unsafe {
                    unsafe { (*pMerger).aReadr }.offset(
                        (unsafe {
                            *unsafe { unsafe { (*pMerger).aTree }.offset((1 as i32) as isize) }
                        }) as isize,
                    )
                })
                .pFd
            }) == std::ptr::null_mut::<sqlite3_file>()) as i32;
        }
    }
    return if rc == (0 as i32) {
        ((unsafe { (*unsafe { (*pTask).pUnpacked }).errCode }) as u32) as i32
    } else {
        rc
    };
}

/// The main routine for background threads that write level-0 PMAs.
#[unsafe(link_section = ".text.slate_distinct.vdbesort.vdbeSorterFlushThread")]
extern "C-unwind" fn vdbeSorterFlushThread(mut pCtx: *mut ()) -> *mut () {
    let mut pTask: *mut SortSubtask = pCtx as *mut SortSubtask;
    let mut rc: i32 = 0 as i32; // Return code
    0 as i32;
    rc = vdbeSorterListToPMA(pTask, unsafe { std::ptr::addr_of_mut!((*pTask).list) });
    unsafe {
        (*pTask).bDone = 1 as i32;
    }
    return (rc as i64) as *mut ();
}

/// Flush the current contents of VdbeSorter.list to a new PMA, possibly
/// using a background thread.
fn vdbeSorterFlushPMA(mut pSorter: *mut VdbeSorter) -> i32 {
    let mut rc: i32 = 0 as i32;
    let mut i: i32 = 0 as i32;
    let mut pTask: *mut SortSubtask = std::ptr::null_mut::<SortSubtask>(); // Thread context used to create new PMA
    let mut nWorker: i32 = (((unsafe { (*pSorter).nTask }) as u32) as i32) - (1 as i32);
    // Set the flag to indicate that at least one PMA has been written.
    // Or will be, anyhow.
    unsafe {
        (*pSorter).bUsePMA = ((1 as i32) as i8) as u8;
    }
    // Select a sub-task to sort and flush the current list of in-memory
    // records to disk. If the sorter is running in multi-threaded mode,
    // round-robin between the first (pSorter->nTask-1) tasks. Except, if
    // the background thread from a sub-tasks previous turn is still running,
    // skip it. If the first (pSorter->nTask-1) sub-tasks are all still busy,
    // fall back to using the final sub-task. The first (pSorter->nTask-1)
    // sub-tasks are preferred as they use background threads - the final
    // sub-task uses the main thread.
    i = 0 as i32;
    '__slate_break_956: loop {
        if !(i < nWorker) {
            break;
        }
        let mut iTest: i32 =
            ((((unsafe { (*pSorter).iPrev }) as u32) as i32) + i + (1 as i32)) % nWorker;
        pTask = unsafe {
            unsafe { std::ptr::addr_of_mut!((*pSorter).aTask) as *mut SortSubtask }
                .offset(iTest as isize)
        };
        if (unsafe { (*pTask).bDone }) != (0 as i32) {
            rc = vdbeSorterJoinThread(pTask);
        }
        if rc != (0 as i32) || (unsafe { (*pTask).pThread }) == std::ptr::null_mut::<SQLiteThread>()
        {
            break '__slate_break_956;
        }
        let __v1062: i32 = i;
        let __v1063: i32 = __v1062 + (1 as i32);
        i = __v1063;
    }
    if rc == (0 as i32) {
        if i == nWorker {
            // Use the foreground thread for this operation
            rc = vdbeSorterListToPMA(
                unsafe {
                    unsafe { std::ptr::addr_of_mut!((*pSorter).aTask) as *mut SortSubtask }
                        .offset(nWorker as isize)
                },
                unsafe { std::ptr::addr_of_mut!((*pSorter).list) },
            );
        } else {
            // Launch a background thread for this operation
            let mut aMem: *mut u8 = unsafe { std::mem::zeroed() };
            let mut pCtx: *mut () = unsafe { std::mem::zeroed() };
            0 as i32;
            0 as i32;
            0 as i32;
            0 as i32;
            aMem = unsafe { (*pTask).list.aMemory };
            pCtx = pTask as *mut ();
            unsafe {
                (*pSorter).iPrev = (((unsafe {
                    pTask.offset_from(
                        (unsafe { std::ptr::addr_of_mut!((*pSorter).aTask) as *mut SortSubtask })
                            as *mut SortSubtask,
                    )
                }) as i64) as i8) as u8;
            }
            unsafe {
                (*pTask).list = unsafe { (*pSorter).list };
            }
            unsafe {
                (*pSorter).list.pList = std::ptr::null_mut::<SorterRecord>();
            }
            unsafe {
                (*pSorter).list.szPMA = (0 as i32) as i64;
            }
            if aMem != std::ptr::null_mut::<u8>() {
                unsafe {
                    (*pSorter).list.aMemory = aMem;
                }
                unsafe {
                    (*pSorter).nMemory = unsafe { sqlite3MallocSize(aMem as *const ()) };
                }
            } else {
                if (unsafe { (*pSorter).list.aMemory }) != std::ptr::null_mut::<u8>() {
                    unsafe {
                        (*pSorter).list.aMemory = (unsafe {
                            sqlite3Malloc(((unsafe { (*pSorter).nMemory }) as i64) as u64)
                        }) as *mut u8;
                    }
                    if !((unsafe { (*pSorter).list.aMemory }) != std::ptr::null_mut::<u8>()) {
                        return 7 as i32;
                    }
                }
            }
            rc = vdbeSorterCreateThread(pTask, Some(vdbeSorterFlushThread), pCtx);
        }
    }
    return rc;
}

/// Add a record to the sorter.
///
/// # Arguments
///
/// * `pCsr` - Sorter cursor
/// * `pVal` - Memory cell containing record
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3VdbeSorterWrite(
    mut pCsr: *const VdbeCursor,
    mut pVal: *mut sqlite3_value,
) -> i32 {
    let mut pSorter: *mut VdbeSorter = unsafe { std::mem::zeroed() };
    let mut rc: i32 = 0 as i32; // Return Code
    let mut pNew: *mut SorterRecord = unsafe { std::mem::zeroed() }; // New list element
    let mut bFlush: i32 = 0 as i32; // True to flush contents of memory to PMA
    let mut nReq: i64 = 0 as i64; // Bytes of memory required
    let mut nPMA: i64 = 0 as i64; // Bytes of PMA space required
    let mut t: i32 = 0 as i32; // serial type of first record field
    0 as i32;
    pSorter = unsafe { (*pCsr).uc.pSorter };
    t = ((unsafe { *((unsafe { unsafe { (*pVal).z }.offset((1 as i32) as isize) }) as *const u8) })
        as u32) as i32;
    if t >= (128 as i32) {
        unsafe {
            sqlite3GetVarint32(
                (unsafe { unsafe { (*pVal).z }.offset((1 as i32) as isize) }) as *const u8,
                std::ptr::addr_of_mut!(t) as *mut u32,
            )
        };
    }
    if t > (0 as i32) && t < (10 as i32) {
        if t == (7 as i32) {
            let __v984: *mut VdbeSorter = pSorter;
            let __v985: u8 = unsafe { (*__v984).typeMask };
            let __v986: u8 = ((((__v985 as u32) as i32) & (4 as i32)) as i8) as u8;
            unsafe {
                (*__v984).typeMask = __v986;
            }
        } else {
            let __v987: *mut VdbeSorter = pSorter;
            let __v988: u8 = unsafe { (*__v987).typeMask };
            let __v989: u8 = ((((__v988 as u32) as i32) & ((1 as i32) | (4 as i32))) as i8) as u8;
            unsafe {
                (*__v987).typeMask = __v989;
            }
        }
    } else {
        if t > (10 as i32) && t & (1 as i32) != (0 as i32) {
            let __v990: *mut VdbeSorter = pSorter;
            let __v991: u8 = unsafe { (*__v990).typeMask };
            let __v992: u8 = ((((__v991 as u32) as i32) & (2 as i32)) as i8) as u8;
            unsafe {
                (*__v990).typeMask = __v992;
            }
        } else {
            unsafe {
                (*pSorter).typeMask = ((0 as i32) as i8) as u8;
            }
        }
    }
    0 as i32;
    // Figure out whether or not the current contents of memory should be
    // flushed to a PMA before continuing. If so, do so.
    //
    // If using the single large allocation mode (pSorter->aMemory!=0), then
    // flush the contents of memory to a new PMA if (a) at least one value is
    // already in memory and (b) the new value will not fit in memory.
    //
    // Or, if using separate allocations for each record, flush the contents
    // of memory to a PMA if either of the following are true:
    //
    //   * The total memory allocated for the in-memory list is greater
    //     than (page-size * cache-size), or
    //
    //   * The total memory allocated for the in-memory list is greater
    //     than (page-size * 10) and sqlite3HeapNearlyFull() returns true.
    nReq = (((unsafe { (*pVal).n }) as i64) as u64).wrapping_add(16 as u64) as i64;
    nPMA = ((unsafe { (*pVal).n })
        + unsafe { sqlite3VarintLen(((unsafe { (*pVal).n }) as i64) as u64) }) as i64;
    if (unsafe { (*pSorter).mxPmaSize }) != (0 as i32) {
        if (unsafe { (*pSorter).list.aMemory }) != std::ptr::null_mut::<u8>() {
            bFlush = ((unsafe { (*pSorter).iMemory }) != (0 as i32)
                && ((unsafe { (*pSorter).iMemory }) as i64) + nReq
                    > ((unsafe { (*pSorter).mxPmaSize }) as i64)) as i32;
        } else {
            let __v993: bool;
            if (unsafe { (*pSorter).list.szPMA }) > ((unsafe { (*pSorter).mxPmaSize }) as i64) {
                __v993 = true as bool;
            } else {
                let __v994: bool;
                if (unsafe { (*pSorter).list.szPMA }) > ((unsafe { (*pSorter).mnPmaSize }) as i64) {
                    __v994 = (unsafe { sqlite3HeapNearlyFull() }) != (0 as i32);
                } else {
                    __v994 = false as bool;
                }
                __v993 = __v994;
            }
            bFlush = __v993 as i32;
        }
        if bFlush != (0 as i32) {
            rc = vdbeSorterFlushPMA(pSorter);
            unsafe {
                (*pSorter).list.szPMA = (0 as i32) as i64;
            }
            unsafe {
                (*pSorter).iMemory = 0 as i32;
            }
            0 as i32;
        }
    }
    let __v995: *mut VdbeSorter = pSorter;
    let __v996: i64 = unsafe { (*__v995).list.szPMA };
    let __v997: i64 = __v996 + nPMA;
    unsafe {
        (*__v995).list.szPMA = __v997;
    }
    if nPMA > ((unsafe { (*pSorter).mxKeysize }) as i64) {
        unsafe {
            (*pSorter).mxKeysize = nPMA as i32;
        }
    }
    if (unsafe { (*pSorter).list.aMemory }) != std::ptr::null_mut::<u8>() {
        let mut nMin: i32 = (((unsafe { (*pSorter).iMemory }) as i64) + nReq) as i32;
        if nMin > unsafe { (*pSorter).nMemory } {
            let mut aNew: *mut u8 = unsafe { std::mem::zeroed() };
            let mut nNew: i64 = ((2 as i32) as i64) * ((unsafe { (*pSorter).nMemory }) as i64);
            let mut iListOff: i32 = -(1 as i32);
            if (unsafe { (*pSorter).list.pList }) != std::ptr::null_mut::<SorterRecord>() {
                iListOff = ((unsafe {
                    ((unsafe { (*pSorter).list.pList }) as *mut u8)
                        .offset_from((unsafe { (*pSorter).list.aMemory }) as *mut u8)
                }) as i64) as i32;
            }
            '__slate_break_957: while nNew < (nMin as i64) {
                nNew = nNew * ((2 as i32) as i64);
            }
            if nNew > ((unsafe { (*pSorter).mxPmaSize }) as i64) {
                nNew = (unsafe { (*pSorter).mxPmaSize }) as i64;
            }
            if nNew < (nMin as i64) {
                nNew = nMin as i64;
            }
            aNew = (unsafe {
                sqlite3Realloc((unsafe { (*pSorter).list.aMemory }) as *mut (), nNew as u64)
            }) as *mut u8;
            if !(aNew != std::ptr::null_mut::<u8>()) {
                return 7 as i32;
            }
            if iListOff >= (0 as i32) {
                unsafe {
                    (*pSorter).list.pList =
                        (unsafe { aNew.offset(iListOff as isize) }) as *mut SorterRecord;
                }
            }
            unsafe {
                (*pSorter).list.aMemory = aNew;
            }
            unsafe {
                (*pSorter).nMemory = nNew as i32;
            }
        }
        pNew = (unsafe {
            unsafe { (*pSorter).list.aMemory }.offset((unsafe { (*pSorter).iMemory }) as isize)
        }) as *mut SorterRecord;
        let __v998: *mut VdbeSorter = pSorter;
        let __v999: i32 = unsafe { (*__v998).iMemory };
        let __v1000: i32 =
            ((__v999 as i64) + (nReq + ((7 as i32) as i64) & (!(7 as i32) as i64))) as i32;
        unsafe {
            (*__v998).iMemory = __v1000;
        }
        if (unsafe { (*pSorter).list.pList }) != std::ptr::null_mut::<SorterRecord>() {
            unsafe {
                (*pNew).u.iNext = ((unsafe {
                    ((unsafe { (*pSorter).list.pList }) as *mut u8)
                        .offset_from((unsafe { (*pSorter).list.aMemory }) as *mut u8)
                }) as i64) as i32;
            }
        }
    } else {
        pNew = (unsafe { sqlite3Malloc(nReq as u64) }) as *mut SorterRecord;
        if pNew == std::ptr::null_mut::<SorterRecord>() {
            return 7 as i32;
        }
        unsafe {
            (*pNew).u.pNext = unsafe { (*pSorter).list.pList };
        }
    }
    unsafe {
        memcpy(
            (unsafe { pNew.offset((1 as i32) as isize) }) as *mut (),
            (unsafe { (*pVal).z }) as *const (),
            ((unsafe { (*pVal).n }) as i64) as u64,
        )
    };
    unsafe {
        (*pNew).nVal = unsafe { (*pVal).n };
    }
    unsafe {
        (*pSorter).list.pList = pNew;
    }
    return rc;
}

/// Read keys from pIncr->pMerger and populate pIncr->aFile[1]. The format
/// of the data stored in aFile[1] is the same as that used by regular PMAs,
/// except that the number-of-bytes varint is omitted from the start.
fn vdbeIncrPopulate(mut pIncr: *mut IncrMerger) -> i32 {
    let mut rc: i32 = 0 as i32;
    let mut rc2: i32 = 0 as i32;
    let mut iStart: i64 = unsafe { (*pIncr).iStartOff };
    let mut pOut: *mut SorterFile = unsafe {
        unsafe { (*pIncr).aFile.as_mut_ptr() as *mut SorterFile }.offset((1 as i32) as isize)
    };
    let mut pTask: *mut SortSubtask = unsafe { (*pIncr).pTask };
    let mut pMerger: *mut MergeEngine = unsafe { (*pIncr).pMerger };
    let mut writer: PmaWriter = unsafe { std::mem::zeroed() };
    0 as i32;
    {}
    vdbePmaWriterInit(
        unsafe { (*pOut).pFd },
        std::ptr::addr_of_mut!(writer),
        unsafe { (*unsafe { (*pTask).pSorter }).pgsz },
        iStart,
    );
    '__slate_break_958: while rc == (0 as i32) {
        let mut dummy: i32 = 0 as i32;
        let mut pReader: *mut PmaReader = unsafe {
            unsafe { (*pMerger).aReadr }.offset(
                (unsafe { *unsafe { unsafe { (*pMerger).aTree }.offset((1 as i32) as isize) } })
                    as isize,
            )
        };
        let mut nKey: i32 = unsafe { (*pReader).nKey };
        let mut iEof: i64 = writer.iWriteOff + (writer.iBufEnd as i64);
        // Check if the output file is full or if the input has been exhausted.
        // In either case exit the loop.
        if (unsafe { (*pReader).pFd }) == std::ptr::null_mut::<sqlite3_file>() {
            break '__slate_break_958;
        }
        if iEof + (nKey as i64) + ((unsafe { sqlite3VarintLen((nKey as i64) as u64) }) as i64)
            > iStart + ((unsafe { (*pIncr).mxSz }) as i64)
        {
            break '__slate_break_958;
        }
        // Write the next key to the output.
        vdbePmaWriteVarint(std::ptr::addr_of_mut!(writer), (nKey as i64) as u64);
        vdbePmaWriteBlob(
            std::ptr::addr_of_mut!(writer),
            unsafe { (*pReader).aKey },
            nKey,
        );
        0 as i32;
        rc = vdbeMergeEngineStep(unsafe { (*pIncr).pMerger }, std::ptr::addr_of_mut!(dummy));
    }
    rc2 = vdbePmaWriterFinish(
        std::ptr::addr_of_mut!(writer),
        unsafe { std::ptr::addr_of_mut!((*pOut).iEof) },
        unsafe { std::ptr::addr_of_mut!((*pTask).nSpill) },
    );
    if rc == (0 as i32) {
        rc = rc2;
    }
    {}
    return rc;
}

/// The main routine for background threads that populate aFile[1] of
/// multi-threaded IncrMerger objects.
#[unsafe(link_section = ".text.slate_distinct.vdbesort.vdbeIncrPopulateThread")]
extern "C-unwind" fn vdbeIncrPopulateThread(mut pCtx: *mut ()) -> *mut () {
    let mut pIncr: *mut IncrMerger = pCtx as *mut IncrMerger;
    let mut pRet: *mut () = (vdbeIncrPopulate(pIncr) as i64) as *mut ();
    unsafe {
        (*unsafe { (*pIncr).pTask }).bDone = 1 as i32;
    }
    return pRet;
}

/// Launch a background thread to populate aFile[1] of pIncr.
fn vdbeIncrBgPopulate(mut pIncr: *mut IncrMerger) -> i32 {
    let mut p: *mut () = pIncr as *mut ();
    0 as i32;
    return vdbeSorterCreateThread(unsafe { (*pIncr).pTask }, Some(vdbeIncrPopulateThread), p);
}

/// This function is called when the PmaReader corresponding to pIncr has
/// finished reading the contents of aFile[0]. Its purpose is to "refill"
/// aFile[0] such that the PmaReader should start rereading it from the
/// beginning.
///
/// For single-threaded objects, this is accomplished by literally reading
/// keys from pIncr->pMerger and repopulating aFile[0].
///
/// For multi-threaded objects, all that is required is to wait until the
/// background thread is finished (if it is not already) and then swap
/// aFile[0] and aFile[1] in place. If the contents of pMerger have not
/// been exhausted, this function also launches a new background thread
/// to populate the new aFile[1].
///
/// SQLITE_OK is returned on success, or an SQLite error code otherwise.
fn vdbeIncrSwap(mut pIncr: *mut IncrMerger) -> i32 {
    let mut rc: i32 = 0 as i32;
    if (unsafe { (*pIncr).bUseThread }) != (0 as i32) {
        rc = vdbeSorterJoinThread(unsafe { (*pIncr).pTask });
        if rc == (0 as i32) {
            let mut f0: SorterFile = unsafe {
                *unsafe {
                    unsafe { (*pIncr).aFile.as_mut_ptr() as *mut SorterFile }
                        .offset((0 as i32) as isize)
                }
            };
            unsafe {
                *unsafe {
                    unsafe { (*pIncr).aFile.as_mut_ptr() as *mut SorterFile }
                        .offset((0 as i32) as isize)
                } = unsafe {
                    *unsafe {
                        unsafe { (*pIncr).aFile.as_mut_ptr() as *mut SorterFile }
                            .offset((1 as i32) as isize)
                    }
                };
            }
            unsafe {
                *unsafe {
                    unsafe { (*pIncr).aFile.as_mut_ptr() as *mut SorterFile }
                        .offset((1 as i32) as isize)
                } = f0;
            }
        }
        if rc == (0 as i32) {
            if (unsafe {
                (*unsafe {
                    unsafe { (*pIncr).aFile.as_mut_ptr() as *mut SorterFile }
                        .offset((0 as i32) as isize)
                })
                .iEof
            }) == unsafe { (*pIncr).iStartOff }
            {
                unsafe {
                    (*pIncr).bEof = 1 as i32;
                }
            } else {
                rc = vdbeIncrBgPopulate(pIncr);
            }
        }
    } else {
        rc = vdbeIncrPopulate(pIncr);
        unsafe {
            *unsafe {
                unsafe { (*pIncr).aFile.as_mut_ptr() as *mut SorterFile }
                    .offset((0 as i32) as isize)
            } = unsafe {
                *unsafe {
                    unsafe { (*pIncr).aFile.as_mut_ptr() as *mut SorterFile }
                        .offset((1 as i32) as isize)
                }
            };
        }
        if (unsafe {
            (*unsafe {
                unsafe { (*pIncr).aFile.as_mut_ptr() as *mut SorterFile }
                    .offset((0 as i32) as isize)
            })
            .iEof
        }) == unsafe { (*pIncr).iStartOff }
        {
            unsafe {
                (*pIncr).bEof = 1 as i32;
            }
        }
    }
    return rc;
}

/// Allocate and return a new IncrMerger object to read data from pMerger.
///
/// If an OOM condition is encountered, return NULL. In this case free the
/// pMerger argument before returning.
///
/// # Arguments
///
/// * `pTask` - The thread that will be using the new IncrMerger
/// * `pMerger` - The MergeEngine that the IncrMerger will control
/// * `ppOut` - Write the new IncrMerger here
fn vdbeIncrMergerNew(
    mut pTask: *mut SortSubtask,
    mut pMerger: *mut MergeEngine,
    mut ppOut: *mut *mut IncrMerger,
) -> i32 {
    let mut rc: i32 = 0 as i32;
    let mut pIncr: *mut IncrMerger = unsafe { std::mem::zeroed() };
    let __v1064: *mut ();
    if (unsafe { sqlite3FaultSim(100 as i32) }) != (0 as i32) {
        __v1064 = std::ptr::null_mut::<()>();
    } else {
        __v1064 = unsafe { sqlite3MallocZero(72 as u64) };
    }
    let __v1065: *mut IncrMerger = __v1064 as *mut IncrMerger;
    unsafe {
        *ppOut = __v1065;
    }
    pIncr = __v1065;
    if pIncr != std::ptr::null_mut::<IncrMerger>() {
        unsafe {
            (*pIncr).pMerger = pMerger;
        }
        unsafe {
            (*pIncr).pTask = pTask;
        }
        unsafe {
            (*pIncr).mxSz = if (unsafe { (*unsafe { (*pTask).pSorter }).mxKeysize }) + (9 as i32)
                > (unsafe { (*unsafe { (*pTask).pSorter }).mxPmaSize }) / (2 as i32)
            {
                (unsafe { (*unsafe { (*pTask).pSorter }).mxKeysize }) + (9 as i32)
            } else {
                (unsafe { (*unsafe { (*pTask).pSorter }).mxPmaSize }) / (2 as i32)
            };
        }
        let __v1066: *mut SortSubtask = pTask;
        let __v1067: i64 = unsafe { (*__v1066).file2.iEof };
        let __v1068: i64 = __v1067 + ((unsafe { (*pIncr).mxSz }) as i64);
        unsafe {
            (*__v1066).file2.iEof = __v1068;
        }
    } else {
        vdbeMergeEngineFree(pMerger);
        rc = 7 as i32;
    }
    0 as i32;
    return rc;
}

/// Set the "use-threads" flag on object pIncr.
fn vdbeIncrMergerSetThreads(mut pIncr: *mut IncrMerger) {
    unsafe {
        (*pIncr).bUseThread = 1 as i32;
    }
    let __v1069: *mut SortSubtask = unsafe { (*pIncr).pTask };
    let __v1070: i64 = unsafe { (*__v1069).file2.iEof };
    let __v1071: i64 = __v1070 - ((unsafe { (*pIncr).mxSz }) as i64);
    unsafe {
        (*__v1069).file2.iEof = __v1071;
    }
}

/// Recompute pMerger->aTree[iOut] by comparing the next keys on the
/// two PmaReaders that feed that entry.  Neither of the PmaReaders
/// are advanced.  This routine merely does the comparison.
///
/// # Arguments
///
/// * `pMerger` - Merge engine containing PmaReaders to compare
/// * `iOut` - Store the result in pMerger->aTree[iOut]
fn vdbeMergeEngineCompare(mut pMerger: *mut MergeEngine, mut iOut: i32) {
    let mut i1: i32 = 0 as i32;
    let mut i2: i32 = 0 as i32;
    let mut iRes: i32 = 0 as i32;
    let mut p1: *mut PmaReader = unsafe { std::mem::zeroed() };
    let mut p2: *mut PmaReader = unsafe { std::mem::zeroed() };
    0 as i32;
    if iOut >= (unsafe { (*pMerger).nTree }) / (2 as i32) {
        i1 = (iOut - (unsafe { (*pMerger).nTree }) / (2 as i32)) * (2 as i32);
        i2 = i1 + (1 as i32);
    } else {
        i1 =
            unsafe { *unsafe { unsafe { (*pMerger).aTree }.offset((iOut * (2 as i32)) as isize) } };
        i2 = unsafe {
            *unsafe {
                unsafe { (*pMerger).aTree }.offset((iOut * (2 as i32) + (1 as i32)) as isize)
            }
        };
    }
    p1 = unsafe { unsafe { (*pMerger).aReadr }.offset(i1 as isize) };
    p2 = unsafe { unsafe { (*pMerger).aReadr }.offset(i2 as isize) };
    if (unsafe { (*p1).pFd }) == std::ptr::null_mut::<sqlite3_file>() {
        iRes = i2;
    } else {
        if (unsafe { (*p2).pFd }) == std::ptr::null_mut::<sqlite3_file>() {
            iRes = i1;
        } else {
            let mut pTask: *mut SortSubtask = unsafe { (*pMerger).pTask };
            let mut bCached: i32 = 0 as i32;
            let mut res: i32 = 0 as i32;
            0 as i32; // from vdbeSortSubtaskMain()
            res = unsafe {
                unsafe { (*pTask).xCompare }.unwrap()(
                    pTask,
                    std::ptr::addr_of_mut!(bCached),
                    (unsafe { (*p1).aKey }) as *const (),
                    unsafe { (*p1).nKey },
                    (unsafe { (*p2).aKey }) as *const (),
                    unsafe { (*p2).nKey },
                )
            };
            if res <= (0 as i32) {
                iRes = i1;
            } else {
                iRes = i2;
            }
        }
    }
    unsafe {
        *unsafe { unsafe { (*pMerger).aTree }.offset(iOut as isize) } = iRes;
    }
}

// Allowed values for the eMode parameter to vdbeMergeEngineInit()
// and vdbePmaReaderIncrMergeInit().
//
// Only INCRINIT_NORMAL is valid in single-threaded builds (when
// SQLITE_MAX_WORKER_THREADS==0).  The other values are only used
// when there exists one or more separate worker threads.
/// Initialize the MergeEngine object passed as the second argument. Once this
/// function returns, the first key of merged data may be read from the
/// MergeEngine object in the usual fashion.
///
/// If argument eMode is INCRINIT_ROOT, then it is assumed that any IncrMerge
/// objects attached to the PmaReader objects that the merger reads from have
/// already been populated, but that they have not yet populated aFile[0] and
/// set the PmaReader objects up to read from it. In this case all that is
/// required is to call vdbePmaReaderNext() on each PmaReader to point it at
/// its first key.
///
/// Otherwise, if eMode is any value other than INCRINIT_ROOT, then use
/// vdbePmaReaderIncrMergeInit() to initialize each PmaReader that feeds data
/// to pMerger.
///
/// SQLITE_OK is returned if successful, or an SQLite error code otherwise.
///
/// # Arguments
///
/// * `pTask` - Thread that will run pMerger
/// * `pMerger` - MergeEngine to initialize
/// * `eMode` - One of the INCRINIT_XXX constants
fn vdbeMergeEngineInit(
    mut pTask: *mut SortSubtask,
    mut pMerger: *mut MergeEngine,
    mut eMode: i32,
) -> i32 {
    let mut rc: i32 = 0 as i32; // Return code
    let mut i: i32 = 0 as i32; // For looping over PmaReader objects
    let mut nTree: i32 = 0 as i32; // Number of subtrees to merge
    // Failure to allocate the merge would have been detected prior to
    // invoking this routine
    0 as i32;
    // eMode is always INCRINIT_NORMAL in single-threaded mode
    0 as i32;
    // Verify that the MergeEngine is assigned to a single thread
    0 as i32;
    unsafe {
        (*pMerger).pTask = pTask;
    }
    nTree = unsafe { (*pMerger).nTree };
    i = 0 as i32;
    '__slate_break_961: loop {
        if !(i < nTree) {
            break;
        }
        if (8 as i32) > (0 as i32) && eMode == (2 as i32) {
            // PmaReaders should be normally initialized in order, as if they are
            // reading from the same temp file this makes for more linear file IO.
            // However, in the INCRINIT_ROOT case, if PmaReader aReadr[nTask-1] is
            // in use it will block the vdbePmaReaderNext() call while it uses
            // the main thread to fill its buffer. So calling PmaReaderNext()
            // on this PmaReader before any of the multi-threaded PmaReaders takes
            // better advantage of multi-processor hardware.
            rc = vdbePmaReaderNext(unsafe {
                unsafe { (*pMerger).aReadr }.offset((nTree - i - (1 as i32)) as isize)
            });
        } else {
            rc = vdbePmaReaderIncrInit(
                unsafe { unsafe { (*pMerger).aReadr }.offset(i as isize) },
                0 as i32,
            );
        }
        if rc != (0 as i32) {
            return rc;
        }
        let __v1072: i32 = i;
        let __v1073: i32 = __v1072 + (1 as i32);
        i = __v1073;
    }
    i = (unsafe { (*pMerger).nTree }) - (1 as i32);
    '__slate_break_962: loop {
        if !(i > (0 as i32)) {
            break;
        }
        vdbeMergeEngineCompare(pMerger, i);
        let __v1074: i32 = i;
        let __v1075: i32 = __v1074 - (1 as i32);
        i = __v1075;
    }
    return ((unsafe { (*unsafe { (*pTask).pUnpacked }).errCode }) as u32) as i32;
}

/// The PmaReader passed as the first argument is guaranteed to be an
/// incremental-reader (pReadr->pIncr!=0). This function serves to open
/// and/or initialize the temp file related fields of the IncrMerge
/// object at (pReadr->pIncr).
///
/// If argument eMode is set to INCRINIT_NORMAL, then all PmaReaders
/// in the sub-tree headed by pReadr are also initialized. Data is then
/// loaded into the buffers belonging to pReadr and it is set to point to
/// the first key in its range.
///
/// If argument eMode is set to INCRINIT_TASK, then pReadr is guaranteed
/// to be a multi-threaded PmaReader and this function is being called in a
/// background thread. In this case all PmaReaders in the sub-tree are
/// initialized as for INCRINIT_NORMAL and the aFile[1] buffer belonging to
/// pReadr is populated. However, pReadr itself is not set up to point
/// to its first key. A call to vdbePmaReaderNext() is still required to do
/// that.
///
/// The reason this function does not call vdbePmaReaderNext() immediately
/// in the INCRINIT_TASK case is that vdbePmaReaderNext() assumes that it has
/// to block on thread (pTask->thread) before accessing aFile[1]. But, since
/// this entire function is being run by thread (pTask->thread), that will
/// lead to the current background thread attempting to join itself.
///
/// Finally, if argument eMode is set to INCRINIT_ROOT, it may be assumed
/// that pReadr->pIncr is a multi-threaded IncrMerge objects, and that all
/// child-trees have already been initialized using IncrInit(INCRINIT_TASK).
/// In this case vdbePmaReaderNext() is called on all child PmaReaders and
/// the current PmaReader set to point to the first key in its range.
///
/// SQLITE_OK is returned if successful, or an SQLite error code otherwise.
fn vdbePmaReaderIncrMergeInit(mut pReadr: *mut PmaReader, mut eMode: i32) -> i32 {
    let mut rc: i32 = 0 as i32;
    let mut pIncr: *mut IncrMerger = unsafe { (*pReadr).pIncr };
    let mut pTask: *mut SortSubtask = unsafe { (*pIncr).pTask };
    let mut db: *mut sqlite3 = unsafe { (*unsafe { (*pTask).pSorter }).db };
    // eMode is always INCRINIT_NORMAL in single-threaded mode
    0 as i32;
    rc = vdbeMergeEngineInit(pTask, unsafe { (*pIncr).pMerger }, eMode);
    // Set up the required files for pIncr. A multi-threaded IncrMerge object
    // requires two temp files to itself, whereas a single-threaded object
    // only requires a region of pTask->file2.
    if rc == (0 as i32) {
        let mut mxSz: i32 = unsafe { (*pIncr).mxSz };
        if (unsafe { (*pIncr).bUseThread }) != (0 as i32) {
            rc = vdbeSorterOpenTempFile(db, mxSz as i64, unsafe {
                std::ptr::addr_of_mut!(
                    (*unsafe {
                        unsafe { (*pIncr).aFile.as_mut_ptr() as *mut SorterFile }
                            .offset((0 as i32) as isize)
                    })
                    .pFd
                )
            });
            if rc == (0 as i32) {
                rc = vdbeSorterOpenTempFile(db, mxSz as i64, unsafe {
                    std::ptr::addr_of_mut!(
                        (*unsafe {
                            unsafe { (*pIncr).aFile.as_mut_ptr() as *mut SorterFile }
                                .offset((1 as i32) as isize)
                        })
                        .pFd
                    )
                });
            }
        } else {
            if (unsafe { (*pTask).file2.pFd }) == std::ptr::null_mut::<sqlite3_file>() {
                0 as i32;
                rc = vdbeSorterOpenTempFile(db, unsafe { (*pTask).file2.iEof }, unsafe {
                    std::ptr::addr_of_mut!((*pTask).file2.pFd)
                });
                unsafe {
                    (*pTask).file2.iEof = (0 as i32) as i64;
                }
            }
            if rc == (0 as i32) {
                unsafe {
                    (*unsafe {
                        unsafe { (*pIncr).aFile.as_mut_ptr() as *mut SorterFile }
                            .offset((1 as i32) as isize)
                    })
                    .pFd = unsafe { (*pTask).file2.pFd };
                }
                unsafe {
                    (*pIncr).iStartOff = unsafe { (*pTask).file2.iEof };
                }
                let __v1076: *mut SortSubtask = pTask;
                let __v1077: i64 = unsafe { (*__v1076).file2.iEof };
                let __v1078: i64 = __v1077 + (mxSz as i64);
                unsafe {
                    (*__v1076).file2.iEof = __v1078;
                }
            }
        }
        // if( !pIncr->bUseThread )
    }
    if rc == (0 as i32) && (unsafe { (*pIncr).bUseThread }) != (0 as i32) {
        // Use the current thread to populate aFile[1], even though this
        // PmaReader is multi-threaded. If this is an INCRINIT_TASK object,
        // then this function is already running in background thread
        // pIncr->pTask->thread.
        //
        // If this is the INCRINIT_ROOT object, then it is running in the
        // main VDBE thread. But that is Ok, as that thread cannot return
        // control to the VDBE or proceed with anything useful until the
        // first results are ready from this merger object anyway.
        0 as i32;
        rc = vdbeIncrPopulate(pIncr);
    }
    if rc == (0 as i32) && ((8 as i32) == (0 as i32) || eMode != (1 as i32)) {
        rc = vdbePmaReaderNext(pReadr);
    }
    return rc;
}

/// The main routine for vdbePmaReaderIncrMergeInit() operations run in
/// background threads.
#[unsafe(link_section = ".text.slate_distinct.vdbesort.vdbePmaReaderBgIncrInit")]
extern "C-unwind" fn vdbePmaReaderBgIncrInit(mut pCtx: *mut ()) -> *mut () {
    let mut pReader: *mut PmaReader = pCtx as *mut PmaReader;
    let mut pRet: *mut () = (vdbePmaReaderIncrMergeInit(pReader, 1 as i32) as i64) as *mut ();
    unsafe {
        (*unsafe { (*unsafe { (*pReader).pIncr }).pTask }).bDone = 1 as i32;
    }
    return pRet;
}

/// If the PmaReader passed as the first argument is not an incremental-reader
/// (if pReadr->pIncr==0), then this function is a no-op. Otherwise, it invokes
/// the vdbePmaReaderIncrMergeInit() function with the parameters passed to
/// this routine to initialize the incremental merge.
///
/// If the IncrMerger object is multi-threaded (IncrMerger.bUseThread==1),
/// then a background thread is launched to call vdbePmaReaderIncrMergeInit().
/// Or, if the IncrMerger is single threaded, the same function is called
/// using the current thread.
fn vdbePmaReaderIncrInit(mut pReadr: *mut PmaReader, mut eMode: i32) -> i32 {
    let mut pIncr: *mut IncrMerger = unsafe { (*pReadr).pIncr }; // Incremental merger
    let mut rc: i32 = 0 as i32; // Return code
    if pIncr != std::ptr::null_mut::<IncrMerger>() {
        0 as i32;
        if (unsafe { (*pIncr).bUseThread }) != (0 as i32) {
            let mut pCtx: *mut () = pReadr as *mut ();
            rc = vdbeSorterCreateThread(
                unsafe { (*pIncr).pTask },
                Some(vdbePmaReaderBgIncrInit),
                pCtx,
            );
        } else {
            rc = vdbePmaReaderIncrMergeInit(pReadr, eMode);
        }
    }
    return rc;
}

/// Allocate a new MergeEngine object to merge the contents of nPMA level-0
/// PMAs from pTask->file. If no error occurs, set *ppOut to point to
/// the new object and return SQLITE_OK. Or, if an error does occur, set *ppOut
/// to NULL and return an SQLite error code.
///
/// When this function is called, *piOffset is set to the offset of the
/// first PMA to read from pTask->file. Assuming no error occurs, it is
/// set to the offset immediately following the last byte of the last
/// PMA before returning. If an error does occur, then the final value of
/// *piOffset is undefined.
///
/// # Arguments
///
/// * `pTask` - Sorter task to read from
/// * `nPMA` - Number of PMAs to read
/// * `piOffset` - IN/OUT: Readr offset in pTask->file
/// * `ppOut` - OUT: New merge-engine
fn vdbeMergeEngineLevel0(
    mut pTask: *mut SortSubtask,
    mut nPMA: i32,
    mut piOffset: *mut i64,
    mut ppOut: *mut *mut MergeEngine,
) -> i32 {
    let mut pNew: *mut MergeEngine = unsafe { std::mem::zeroed() }; // Merge engine to return
    let mut iOff: i64 = unsafe { *piOffset };
    let mut i: i32 = 0 as i32;
    let mut rc: i32 = 0 as i32;
    let __v1079: *mut MergeEngine = vdbeMergeEngineNew(nPMA);
    pNew = __v1079;
    unsafe {
        *ppOut = __v1079;
    }
    if pNew == std::ptr::null_mut::<MergeEngine>() {
        rc = 7 as i32;
    }
    i = 0 as i32;
    '__slate_break_963: loop {
        if !(i < nPMA && rc == (0 as i32)) {
            break;
        }
        let mut nDummy: i64 = (0 as i32) as i64;
        let mut pReadr: *mut PmaReader = unsafe { unsafe { (*pNew).aReadr }.offset(i as isize) };
        rc = vdbePmaReaderInit(
            pTask,
            unsafe { std::ptr::addr_of_mut!((*pTask).file) },
            iOff,
            pReadr,
            std::ptr::addr_of_mut!(nDummy),
        );
        iOff = unsafe { (*pReadr).iEof };
        let __v1080: i32 = i;
        let __v1081: i32 = __v1080 + (1 as i32);
        i = __v1081;
    }
    if rc != (0 as i32) {
        vdbeMergeEngineFree(pNew);
        unsafe {
            *ppOut = std::ptr::null_mut::<MergeEngine>();
        }
    }
    unsafe {
        *piOffset = iOff;
    }
    return rc;
}

/// Return the depth of a tree comprising nPMA PMAs, assuming a fanout of
/// SORTER_MAX_MERGE_COUNT. The returned value does not include leaf nodes.
///
/// i.e.
///
///   nPMA<=16    -> TreeDepth() == 0
///   nPMA<=256   -> TreeDepth() == 1
///   nPMA<=65536 -> TreeDepth() == 2
fn vdbeSorterTreeDepth(mut nPMA: i32) -> i32 {
    let mut nDepth: i32 = 0 as i32;
    let mut nDiv: i64 = (16 as i32) as i64;
    '__slate_break_964: while nDiv < (nPMA as i64) {
        nDiv = nDiv * ((16 as i32) as i64);
        let __v1082: i32 = nDepth;
        let __v1083: i32 = __v1082 + (1 as i32);
        nDepth = __v1083;
    }
    return nDepth;
}

/// pRoot is the root of an incremental merge-tree with depth nDepth (according
/// to vdbeSorterTreeDepth()). pLeaf is the iSeq'th leaf to be added to the
/// tree, counting from zero. This function adds pLeaf to the tree.
///
/// If successful, SQLITE_OK is returned. If an error occurs, an SQLite error
/// code is returned and pLeaf is freed.
///
/// # Arguments
///
/// * `pTask` - Task context
/// * `nDepth` - Depth of tree according to TreeDepth()
/// * `iSeq` - Sequence number of leaf within tree
/// * `pRoot` - Root of tree
/// * `pLeaf` - Leaf to add to tree
fn vdbeSorterAddToTree(
    mut pTask: *mut SortSubtask,
    mut nDepth: i32,
    mut iSeq: i32,
    mut pRoot: *mut MergeEngine,
    mut pLeaf: *mut MergeEngine,
) -> i32 {
    let mut rc: i32 = 0 as i32;
    let mut nDiv: i32 = 1 as i32;
    let mut i: i32 = 0 as i32;
    let mut p: *mut MergeEngine = pRoot;
    let mut pIncr: *mut IncrMerger = unsafe { std::mem::zeroed() };
    rc = vdbeIncrMergerNew(pTask, pLeaf, std::ptr::addr_of_mut!(pIncr));
    i = 1 as i32;
    '__slate_break_965: loop {
        if !(i < nDepth) {
            break;
        }
        nDiv = nDiv * (16 as i32);
        let __v1084: i32 = i;
        let __v1085: i32 = __v1084 + (1 as i32);
        i = __v1085;
    }
    i = 1 as i32;
    '__slate_break_966: loop {
        if !(i < nDepth && rc == (0 as i32)) {
            break;
        }
        let mut iIter: i32 = iSeq / nDiv % (16 as i32);
        let mut pReadr: *mut PmaReader = unsafe { unsafe { (*p).aReadr }.offset(iIter as isize) };
        if (unsafe { (*pReadr).pIncr }) == std::ptr::null_mut::<IncrMerger>() {
            let mut pNew: *mut MergeEngine = vdbeMergeEngineNew(16 as i32);
            if pNew == std::ptr::null_mut::<MergeEngine>() {
                rc = 7 as i32;
            } else {
                rc = vdbeIncrMergerNew(pTask, pNew, unsafe {
                    std::ptr::addr_of_mut!((*pReadr).pIncr)
                });
            }
        }
        if rc == (0 as i32) {
            p = unsafe { (*unsafe { (*pReadr).pIncr }).pMerger };
            nDiv = nDiv / (16 as i32);
        }
        let __v1086: i32 = i;
        let __v1087: i32 = __v1086 + (1 as i32);
        i = __v1087;
    }
    if rc == (0 as i32) {
        unsafe {
            (*unsafe { unsafe { (*p).aReadr }.offset((iSeq % (16 as i32)) as isize) }).pIncr =
                pIncr;
        }
    } else {
        vdbeIncrFree(pIncr);
    }
    return rc;
}

/// This function is called as part of a SorterRewind() operation on a sorter
/// that has already written two or more level-0 PMAs to one or more temp
/// files. It builds a tree of MergeEngine/IncrMerger/PmaReader objects that
/// can be used to incrementally merge all PMAs on disk.
///
/// If successful, SQLITE_OK is returned and *ppOut set to point to the
/// MergeEngine object at the root of the tree before returning. Or, if an
/// error occurs, an SQLite error code is returned and the final value
/// of *ppOut is undefined.
///
/// # Arguments
///
/// * `pSorter` - The VDBE cursor that implements the sort
/// * `ppOut` - Write the MergeEngine here
fn vdbeSorterMergeTreeBuild(mut pSorter: *mut VdbeSorter, mut ppOut: *mut *mut MergeEngine) -> i32 {
    let mut pMain: *mut MergeEngine = std::ptr::null_mut::<MergeEngine>();
    let mut rc: i32 = 0 as i32;
    let mut iTask: i32 = 0 as i32;
    // If the sorter uses more than one task, then create the top-level
    // MergeEngine here. This MergeEngine will read data from exactly
    // one PmaReader per sub-task.
    0 as i32;
    if (((unsafe { (*pSorter).nTask }) as u32) as i32) > (1 as i32) {
        pMain = vdbeMergeEngineNew(((unsafe { (*pSorter).nTask }) as u32) as i32);
        if pMain == std::ptr::null_mut::<MergeEngine>() {
            rc = 7 as i32;
        }
    }
    iTask = 0 as i32;
    '__slate_break_967: loop {
        if !(rc == (0 as i32) && iTask < (((unsafe { (*pSorter).nTask }) as u32) as i32)) {
            break;
        }
        let mut pTask: *mut SortSubtask = unsafe {
            unsafe { std::ptr::addr_of_mut!((*pSorter).aTask) as *mut SortSubtask }
                .offset(iTask as isize)
        };
        0 as i32;
        if (8 as i32) == (0 as i32) || (unsafe { (*pTask).nPMA }) != (0 as i32) {
            let mut pRoot: *mut MergeEngine = std::ptr::null_mut::<MergeEngine>(); // Root node of tree for this task
            let mut nDepth: i32 = vdbeSorterTreeDepth(unsafe { (*pTask).nPMA });
            let mut iReadOff: i64 = (0 as i32) as i64;
            if (unsafe { (*pTask).nPMA }) <= (16 as i32) {
                rc = vdbeMergeEngineLevel0(
                    pTask,
                    unsafe { (*pTask).nPMA },
                    std::ptr::addr_of_mut!(iReadOff),
                    std::ptr::addr_of_mut!(pRoot),
                );
            } else {
                let mut i: i32 = 0 as i32;
                let mut iSeq: i32 = 0 as i32;
                pRoot = vdbeMergeEngineNew(16 as i32);
                if pRoot == std::ptr::null_mut::<MergeEngine>() {
                    rc = 7 as i32;
                }
                i = 0 as i32;
                '__slate_break_968: loop {
                    if !(i < unsafe { (*pTask).nPMA } && rc == (0 as i32)) {
                        break;
                    }
                    let mut pMerger: *mut MergeEngine = std::ptr::null_mut::<MergeEngine>(); // New level-0 PMA merger
                    let mut nReader: i32 = 0 as i32; // Number of level-0 PMAs to merge
                    nReader = if (unsafe { (*pTask).nPMA }) - i < (16 as i32) {
                        (unsafe { (*pTask).nPMA }) - i
                    } else {
                        16 as i32
                    };
                    rc = vdbeMergeEngineLevel0(
                        pTask,
                        nReader,
                        std::ptr::addr_of_mut!(iReadOff),
                        std::ptr::addr_of_mut!(pMerger),
                    );
                    if rc == (0 as i32) {
                        let __v1092: i32 = iSeq;
                        let __v1093: i32 = __v1092 + (1 as i32);
                        iSeq = __v1093;
                        rc = vdbeSorterAddToTree(pTask, nDepth, __v1092, pRoot, pMerger);
                    }
                    let __v1090: i32 = i;
                    let __v1091: i32 = __v1090 + (16 as i32);
                    i = __v1091;
                }
            }
            if rc == (0 as i32) {
                if pMain != std::ptr::null_mut::<MergeEngine>() {
                    rc = vdbeIncrMergerNew(pTask, pRoot, unsafe {
                        std::ptr::addr_of_mut!(
                            (*unsafe { unsafe { (*pMain).aReadr }.offset(iTask as isize) }).pIncr
                        )
                    });
                } else {
                    0 as i32;
                    pMain = pRoot;
                }
            } else {
                vdbeMergeEngineFree(pRoot);
            }
        }
        let __v1088: i32 = iTask;
        let __v1089: i32 = __v1088 + (1 as i32);
        iTask = __v1089;
    }
    if rc != (0 as i32) {
        vdbeMergeEngineFree(pMain);
        pMain = std::ptr::null_mut::<MergeEngine>();
    }
    unsafe {
        *ppOut = pMain;
    }
    return rc;
}

/// This function is called as part of an sqlite3VdbeSorterRewind() operation
/// on a sorter that has written two or more PMAs to temporary files. It sets
/// up either VdbeSorter.pMerger (for single threaded sorters) or pReader
/// (for multi-threaded sorters) so that it can be used to iterate through
/// all records stored in the sorter.
///
/// SQLITE_OK is returned if successful, or an SQLite error code otherwise.
fn vdbeSorterSetupMerge(mut pSorter: *mut VdbeSorter) -> i32 {
    let mut rc: i32 = 0 as i32; // Return code
    let mut pTask0: *mut SortSubtask = unsafe {
        unsafe { std::ptr::addr_of_mut!((*pSorter).aTask) as *mut SortSubtask }
            .offset((0 as i32) as isize)
    };
    let mut pMain: *mut MergeEngine = std::ptr::null_mut::<MergeEngine>();
    let mut db: *mut sqlite3 = unsafe { (*unsafe { (*pTask0).pSorter }).db };
    let mut i: i32 = 0 as i32;
    let mut xCompare: Option<
        unsafe extern "C-unwind" fn(
            *mut SortSubtask,
            *mut i32,
            *const (),
            i32,
            *const (),
            i32,
        ) -> i32,
    > = vdbeSorterGetCompare(pSorter);
    i = 0 as i32;
    '__slate_break_969: loop {
        if !(i < (((unsafe { (*pSorter).nTask }) as u32) as i32)) {
            break;
        }
        unsafe {
            (*unsafe {
                unsafe { std::ptr::addr_of_mut!((*pSorter).aTask) as *mut SortSubtask }
                    .offset(i as isize)
            })
            .xCompare = xCompare;
        }
        let __v1094: i32 = i;
        let __v1095: i32 = __v1094 + (1 as i32);
        i = __v1095;
    }
    rc = vdbeSorterMergeTreeBuild(pSorter, std::ptr::addr_of_mut!(pMain));
    if rc == (0 as i32) {
        0 as i32;
        if (unsafe { (*pSorter).bUseThreads }) != (0 as u8) {
            let mut iTask: i32 = 0 as i32;
            let mut pReadr: *mut PmaReader = std::ptr::null_mut::<PmaReader>();
            let mut pLast: *mut SortSubtask = unsafe {
                unsafe { std::ptr::addr_of_mut!((*pSorter).aTask) as *mut SortSubtask }
                    .offset(((((unsafe { (*pSorter).nTask }) as u32) as i32) - (1 as i32)) as isize)
            };
            rc = vdbeSortAllocUnpacked(pLast);
            if rc == (0 as i32) {
                pReadr = (unsafe { sqlite3DbMallocZero(db, 80 as u64) }) as *mut PmaReader;
                unsafe {
                    (*pSorter).pReader = pReadr;
                }
                if pReadr == std::ptr::null_mut::<PmaReader>() {
                    rc = 7 as i32;
                }
            }
            if rc == (0 as i32) {
                rc = vdbeIncrMergerNew(pLast, pMain, unsafe {
                    std::ptr::addr_of_mut!((*pReadr).pIncr)
                });
                if rc == (0 as i32) {
                    vdbeIncrMergerSetThreads(unsafe { (*pReadr).pIncr });
                    iTask = 0 as i32;
                    '__slate_break_970: loop {
                        if !(iTask < (((unsafe { (*pSorter).nTask }) as u32) as i32) - (1 as i32)) {
                            break;
                        }
                        let mut pIncr: *mut IncrMerger = unsafe { std::mem::zeroed() };
                        let __v1098: *mut IncrMerger = unsafe {
                            (*unsafe { unsafe { (*pMain).aReadr }.offset(iTask as isize) }).pIncr
                        };
                        pIncr = __v1098;
                        if __v1098 != std::ptr::null_mut::<IncrMerger>() {
                            vdbeIncrMergerSetThreads(pIncr);
                            0 as i32;
                        }
                        let __v1096: i32 = iTask;
                        let __v1097: i32 = __v1096 + (1 as i32);
                        iTask = __v1097;
                    }
                    iTask = 0 as i32;
                    '__slate_break_971: loop {
                        if !(rc == (0 as i32)
                            && iTask < (((unsafe { (*pSorter).nTask }) as u32) as i32))
                        {
                            break;
                        }
                        // Check that:
                        //
                        // a) The incremental merge object is configured to use the
                        //    right task, and
                        // b) If it is using task (nTask-1), it is configured to run
                        //    in single-threaded mode. This is important, as the
                        //    root merge (INCRINIT_ROOT) will be using the same task
                        //    object.
                        let mut p: *mut PmaReader =
                            unsafe { unsafe { (*pMain).aReadr }.offset(iTask as isize) };
                        0 as i32; // a
                        // b
                        rc = vdbePmaReaderIncrInit(p, 1 as i32);
                        let __v1099: i32 = iTask;
                        let __v1100: i32 = __v1099 + (1 as i32);
                        iTask = __v1100;
                    }
                }
                pMain = std::ptr::null_mut::<MergeEngine>();
            }
            if rc == (0 as i32) {
                rc = vdbePmaReaderIncrMergeInit(pReadr, 2 as i32);
            }
        } else {
            rc = vdbeMergeEngineInit(pTask0, pMain, 0 as i32);
            unsafe {
                (*pSorter).pMerger = pMain;
            }
            pMain = std::ptr::null_mut::<MergeEngine>();
        }
    }
    if rc != (0 as i32) {
        vdbeMergeEngineFree(pMain);
    }
    return rc;
}

/// Once the sorter has been populated by calls to sqlite3VdbeSorterWrite,
/// this function is called to prepare for iterating through the records
/// in sorted order.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3VdbeSorterRewind(
    mut pCsr: *const VdbeCursor,
    mut pbEof: *mut i32,
) -> i32 {
    let mut pSorter: *mut VdbeSorter = unsafe { std::mem::zeroed() };
    let mut rc: i32 = 0 as i32; // Return code
    0 as i32;
    pSorter = unsafe { (*pCsr).uc.pSorter };
    0 as i32;
    // If no data has been written to disk, then do not do so now. Instead,
    // sort the VdbeSorter.pRecord list. The vdbe layer will read data directly
    // from the in-memory list.
    if (((unsafe { (*pSorter).bUsePMA }) as u32) as i32) == (0 as i32) {
        if (unsafe { (*pSorter).list.pList }) != std::ptr::null_mut::<SorterRecord>() {
            unsafe {
                *pbEof = 0 as i32;
            }
            rc = vdbeSorterSort(
                unsafe {
                    unsafe { std::ptr::addr_of_mut!((*pSorter).aTask) as *mut SortSubtask }
                        .offset((0 as i32) as isize)
                },
                unsafe { std::ptr::addr_of_mut!((*pSorter).list) },
            );
        } else {
            unsafe {
                *pbEof = 1 as i32;
            }
        }
        return rc;
    }
    // Write the current in-memory list to a PMA. When the VdbeSorterWrite()
    // function flushes the contents of memory to disk, it immediately always
    // creates a new list consisting of a single key immediately afterwards.
    // So the list is never empty at this point.
    0 as i32;
    rc = vdbeSorterFlushPMA(pSorter);
    // Join all threads
    rc = vdbeSorterJoinAll(pSorter, rc);
    {}
    // Assuming no errors have occurred, set up a merger structure to
    // incrementally read and merge all remaining PMAs.
    0 as i32;
    if rc == (0 as i32) {
        rc = vdbeSorterSetupMerge(pSorter);
        unsafe {
            *pbEof = 0 as i32;
        }
    }
    {}
    return rc;
}

/// Advance to the next element in the sorter.  Return value:
///
///    SQLITE_OK     success
///    SQLITE_DONE   end of data
///    otherwise     some kind of error.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3VdbeSorterNext(
    mut db: *mut sqlite3,
    mut pCsr: *const VdbeCursor,
) -> i32 {
    let mut pSorter: *mut VdbeSorter = unsafe { std::mem::zeroed() };
    let mut rc: i32 = 0 as i32; // Return code
    0 as i32;
    pSorter = unsafe { (*pCsr).uc.pSorter };
    0 as i32;
    if (unsafe { (*pSorter).bUsePMA }) != (0 as u8) {
        0 as i32;
        0 as i32;
        0 as i32;
        if (unsafe { (*pSorter).bUseThreads }) != (0 as u8) {
            rc = vdbePmaReaderNext(unsafe { (*pSorter).pReader });
            if rc == (0 as i32)
                && (unsafe { (*unsafe { (*pSorter).pReader }).pFd })
                    == std::ptr::null_mut::<sqlite3_file>()
            {
                rc = 101 as i32;
            }
        } else {
            let mut res: i32 = 0 as i32;
            0 as i32;
            0 as i32;
            rc = vdbeMergeEngineStep(unsafe { (*pSorter).pMerger }, std::ptr::addr_of_mut!(res));
            if rc == (0 as i32) && res != (0 as i32) {
                rc = 101 as i32;
            }
        }
    // if( !pSorter->bUseThreads )
    } else {
        let mut pFree: *mut SorterRecord = unsafe { (*pSorter).list.pList };
        unsafe {
            (*pSorter).list.pList = unsafe { (*pFree).u.pNext };
        }
        unsafe {
            (*pFree).u.pNext = std::ptr::null_mut::<SorterRecord>();
        }
        if (unsafe { (*pSorter).list.aMemory }) == std::ptr::null_mut::<u8>() {
            vdbeSorterRecordFree(db, pFree);
        }
        rc = if (unsafe { (*pSorter).list.pList }) != std::ptr::null_mut::<SorterRecord>() {
            0 as i32
        } else {
            101 as i32
        };
    }
    return rc;
}

/// Return a pointer to a buffer owned by the sorter that contains the
/// current key.
///
/// # Arguments
///
/// * `pSorter` - Sorter object
/// * `pnKey` - OUT: Size of current key in bytes
fn vdbeSorterRowkey(mut pSorter: *const VdbeSorter, mut pnKey: *mut i32) -> *mut () {
    let mut pKey: *mut () = unsafe { std::mem::zeroed() };
    if (unsafe { (*pSorter).bUsePMA }) != (0 as u8) {
        let mut pReader: *mut PmaReader = unsafe { std::mem::zeroed() };
        if (unsafe { (*pSorter).bUseThreads }) != (0 as u8) {
            pReader = unsafe { (*pSorter).pReader };
        } else {
            pReader = unsafe {
                unsafe { (*unsafe { (*pSorter).pMerger }).aReadr }.offset(
                    (unsafe {
                        *unsafe {
                            unsafe { (*unsafe { (*pSorter).pMerger }).aTree }
                                .offset((1 as i32) as isize)
                        }
                    }) as isize,
                )
            };
        }
        // if( !pSorter->bUseThreads )
        unsafe {
            *pnKey = unsafe { (*pReader).nKey };
        }
        pKey = (unsafe { (*pReader).aKey }) as *mut ();
    } else {
        unsafe {
            *pnKey = unsafe { (*unsafe { (*pSorter).list.pList }).nVal };
        }
        pKey = (unsafe { unsafe { (*pSorter).list.pList }.offset((1 as i32) as isize) }) as *mut ();
    }
    return pKey;
}

/// Copy the current sorter key into the memory cell pOut.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3VdbeSorterRowkey(
    mut pCsr: *const VdbeCursor,
    mut pOut: *mut sqlite3_value,
) -> i32 {
    let mut pSorter: *mut VdbeSorter = unsafe { std::mem::zeroed() };
    let mut pKey: *mut () = unsafe { std::mem::zeroed() };
    let mut nKey: i32 = 0 as i32; // Sorter key to copy into pOut
    0 as i32;
    pSorter = unsafe { (*pCsr).uc.pSorter };
    pKey = vdbeSorterRowkey(pSorter as *const VdbeSorter, std::ptr::addr_of_mut!(nKey));
    if (unsafe { sqlite3VdbeMemClearAndResize(pOut, nKey) }) != (0 as i32) {
        return 7 as i32;
    }
    unsafe {
        (*pOut).n = nKey;
    }
    unsafe {
        (*pOut).flags = (((((unsafe { (*pOut).flags }) as u32) as i32)
            & !((3519 as i32) | (1024 as i32))
            | (16 as i32)) as i16) as u16;
    }
    unsafe {
        memcpy(
            (unsafe { (*pOut).z }) as *mut (),
            pKey as *const (),
            (nKey as i64) as u64,
        )
    };
    return 0 as i32;
}

/// Compare the key in memory cell pVal with the key that the sorter cursor
/// passed as the first argument currently points to. For the purposes of
/// the comparison, ignore the rowid field at the end of each record.
///
/// If the sorter cursor key contains any NULL values, consider it to be
/// less than pVal. Even if pVal also contains NULL values.
///
/// If an error occurs, return an SQLite error code (i.e. SQLITE_NOMEM).
/// Otherwise, set *pRes to a negative, zero or positive value if the
/// key in pVal is smaller than, equal to or larger than the current sorter
/// key.
///
/// This routine forms the core of the OP_SorterCompare opcode, which in
/// turn is used to verify uniqueness when constructing a UNIQUE INDEX.
///
/// # Arguments
///
/// * `pCsr` - Sorter cursor
/// * `pVal` - Value to compare to current sorter key
/// * `nKeyCol` - Compare this many columns
/// * `pRes` - OUT: Result of comparison
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3VdbeSorterCompare(
    mut pCsr: *const VdbeCursor,
    mut pVal: *mut sqlite3_value,
    mut nKeyCol: i32,
    mut pRes: *mut i32,
) -> i32 {
    let mut pSorter: *mut VdbeSorter = unsafe { std::mem::zeroed() };
    let mut r2: *mut UnpackedRecord = unsafe { std::mem::zeroed() };
    let mut pKeyInfo: *mut KeyInfo = unsafe { std::mem::zeroed() };
    let mut i: i32 = 0 as i32;
    let mut pKey: *mut () = unsafe { std::mem::zeroed() };
    let mut nKey: i32 = 0 as i32; // Sorter key to compare pVal with
    0 as i32;
    pSorter = unsafe { (*pCsr).uc.pSorter };
    r2 = unsafe { (*pSorter).pUnpacked };
    pKeyInfo = unsafe { (*pCsr).pKeyInfo };
    if r2 == std::ptr::null_mut::<UnpackedRecord>() {
        let __v1001: *mut UnpackedRecord = unsafe { sqlite3VdbeAllocUnpackedRecord(pKeyInfo) };
        unsafe {
            (*pSorter).pUnpacked = __v1001;
        }
        r2 = __v1001;
        if r2 == std::ptr::null_mut::<UnpackedRecord>() {
            return 7 as i32;
        }
        unsafe {
            (*r2).nField = (nKeyCol as i16) as u16;
        }
    }
    0 as i32;
    pKey = vdbeSorterRowkey(pSorter as *const VdbeSorter, std::ptr::addr_of_mut!(nKey));
    unsafe { sqlite3VdbeRecordUnpack(nKey, pKey as *const (), r2) };
    i = 0 as i32;
    '__slate_break_972: loop {
        if !(i < nKeyCol) {
            break;
        }
        if (((unsafe { (*unsafe { unsafe { (*r2).aMem }.offset(i as isize) }).flags }) as u32)
            as i32)
            & (1 as i32)
            != (0 as i32)
        {
            unsafe {
                *pRes = -(1 as i32);
            }
            return 0 as i32;
        }
        let __v1002: i32 = i;
        let __v1003: i32 = __v1002 + (1 as i32);
        i = __v1003;
    }
    unsafe {
        *pRes = unsafe {
            sqlite3VdbeRecordCompare(
                unsafe { (*pVal).n },
                (unsafe { (*pVal).z }) as *const (),
                r2,
            )
        };
    }
    return 0 as i32;
}
