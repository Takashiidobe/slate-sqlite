//! 2010 February 1
//!
//! The author disclaims copyright to this source code.  In place of
//! a legal notice, here is a blessing:
//!
//!    May you do good and not evil.
//!    May you find forgiveness for yourself and forgive others.
//!    May you share freely, never taking more than you give.
//!
//!
//!
//! This file contains the implementation of a write-ahead log (WAL) used in
//! "journal_mode=WAL" mode.
//!
//! WRITE-AHEAD LOG (WAL) FILE FORMAT
//!
//! A WAL file consists of a header followed by zero or more "frames".
//! Each frame records the revised content of a single page from the
//! database file.  All changes to the database are recorded by writing
//! frames into the WAL.  Transactions commit when a frame is written that
//! contains a commit marker.  A single WAL can and usually does record
//! multiple transactions.  Periodically, the content of the WAL is
//! transferred back into the database file in an operation called a
//! "checkpoint".
//!
//! A single WAL file can be used multiple times.  In other words, the
//! WAL can fill up with frames and then be checkpointed and then new
//! frames can overwrite the old ones.  A WAL always grows from beginning
//! toward the end.  Checksums and counters attached to each frame are
//! used to determine which frames within the WAL are valid and which
//! are leftovers from prior checkpoints.
//!
//! The WAL header is 32 bytes in size and consists of the following eight
//! big-endian 32-bit unsigned integer values:
//!
//!     0: Magic number.  0x377f0682 or 0x377f0683
//!     4: File format version.  Currently 3007000
//!     8: Database page size.  Example: 1024
//!    12: Checkpoint sequence number
//!    16: Salt-1, random integer incremented with each checkpoint
//!    20: Salt-2, a different random integer changing with each ckpt
//!    24: Checksum-1 (first part of checksum for first 24 bytes of header).
//!    28: Checksum-2 (second part of checksum for first 24 bytes of header).
//!
//! Immediately following the wal-header are zero or more frames. Each
//! frame consists of a 24-byte frame-header followed by <page-size> bytes
//! of page data. The frame-header is six big-endian 32-bit unsigned
//! integer values, as follows:
//!
//!     0: Page number.
//!     4: For commit records, the size of the database image in pages
//!        after the commit. For all other records, zero.
//!     8: Salt-1 (copied from the header)
//!    12: Salt-2 (copied from the header)
//!    16: Checksum-1.
//!    20: Checksum-2.
//!
//! A frame is considered valid if and only if the following conditions are
//! true:
//!
//!    (1) The salt-1 and salt-2 values in the frame-header match
//!        salt values in the wal-header
//!
//!    (2) The checksum values in the final 8 bytes of the frame-header
//!        exactly match the checksum computed consecutively on the
//!        WAL header and the first 8 bytes and the content of all frames
//!        up to and including the current frame.
//!
//! The checksum is computed using 32-bit big-endian integers if the
//! magic number in the first 4 bytes of the WAL is 0x377f0683 and it
//! is computed using little-endian if the magic number is 0x377f0682.
//! The checksum values are always stored in the frame header in a
//! big-endian format regardless of which byte order is used to compute
//! the checksum.  The checksum is computed by interpreting the input as
//! an even number of unsigned 32-bit integers: x[0] through x[N].  The
//! algorithm used for the checksum is as follows:
//!
//!   for i from 0 to n-1 step 2:
//!     s0 += x[i] + s1;
//!     s1 += x[i+1] + s0;
//!   endfor
//!
//! Note that s0 and s1 are both weighted checksums using fibonacci weights
//! in reverse order (the largest fibonacci weight occurs on the first element
//! of the sequence being summed.)  The s1 value spans all 32-bit
//! terms of the sequence whereas s0 omits the final term.
//!
//! On a checkpoint, the WAL is first VFS.xSync-ed, then valid content of the
//! WAL is transferred into the database, then the database is VFS.xSync-ed.
//! The VFS.xSync operations serve as write barriers - all writes launched
//! before the xSync must complete before any write that launches after the
//! xSync begins.
//!
//! After each checkpoint, the salt-1 value is incremented and the salt-2
//! value is randomized.  This prevents old and new frames in the WAL from
//! being considered valid at the same time and being checkpointing together
//! following a crash.
//!
//! READER ALGORITHM
//!
//! To read a page from the database (call it page number P), a reader
//! first checks the WAL to see if it contains page P.  If so, then the
//! last valid instance of page P that is a followed by a commit frame
//! or is a commit frame itself becomes the value read.  If the WAL
//! contains no copies of page P that are valid and which are a commit
//! frame or are followed by a commit frame, then page P is read from
//! the database file.
//!
//! To start a read transaction, the reader records the index of the last
//! valid frame in the WAL.  The reader uses this recorded "mxFrame" value
//! for all subsequent read operations.  New transactions can be appended
//! to the WAL, but as long as the reader uses its original mxFrame value
//! and ignores the newly appended content, it will see a consistent snapshot
//! of the database from a single point in time.  This technique allows
//! multiple concurrent readers to view different versions of the database
//! content simultaneously.
//!
//! The reader algorithm in the previous paragraphs works correctly, but
//! because frames for page P can appear anywhere within the WAL, the
//! reader has to scan the entire WAL looking for page P frames.  If the
//! WAL is large (multiple megabytes is typical) that scan can be slow,
//! and read performance suffers.  To overcome this problem, a separate
//! data structure called the wal-index is maintained to expedite the
//! search for frames of a particular page.
//!
//! WAL-INDEX FORMAT
//!
//! Conceptually, the wal-index is shared memory, though VFS implementations
//! might choose to implement the wal-index using a mmapped file.  Because
//! the wal-index is shared memory, SQLite does not support journal_mode=WAL
//! on a network filesystem.  All users of the database must be able to
//! share memory.
//!
//! In the default unix and windows implementation, the wal-index is a mmapped
//! file whose name is the database name with a "-shm" suffix added.  For that
//! reason, the wal-index is sometimes called the "shm" file.
//!
//! The wal-index is transient.  After a crash, the wal-index can (and should
//! be) reconstructed from the original WAL file.  In fact, the VFS is required
//! to either truncate or zero the header of the wal-index when the last
//! connection to it closes.  Because the wal-index is transient, it can
//! use an architecture-specific format; it does not have to be cross-platform.
//! Hence, unlike the database and WAL file formats which store all values
//! as big endian, the wal-index can store multi-byte values in the native
//! byte order of the host computer.
//!
//! The purpose of the wal-index is to answer this question quickly:  Given
//! a page number P and a maximum frame index M, return the index of the
//! last frame in the wal before frame M for page P in the WAL, or return
//! NULL if there are no frames for page P in the WAL prior to M.
//!
//! The wal-index consists of a header region, followed by an one or
//! more index blocks.
//!
//! The wal-index header contains the total number of frames within the WAL
//! in the mxFrame field.
//!
//! Each index block except for the first contains information on
//! HASHTABLE_NPAGE frames. The first index block contains information on
//! HASHTABLE_NPAGE_ONE frames. The values of HASHTABLE_NPAGE_ONE and
//! HASHTABLE_NPAGE are selected so that together the wal-index header and
//! first index block are the same size as all other index blocks in the
//! wal-index.  The values are:
//!
//!   HASHTABLE_NPAGE      4096
//!   HASHTABLE_NPAGE_ONE  4062
//!
//! Each index block contains two sections, a page-mapping that contains the
//! database page number associated with each wal frame, and a hash-table
//! that allows readers to query an index block for a specific page number.
//! The page-mapping is an array of HASHTABLE_NPAGE (or HASHTABLE_NPAGE_ONE
//! for the first index block) 32-bit page numbers. The first entry in the
//! first index-block contains the database page number corresponding to the
//! first frame in the WAL file. The first entry in the second index block
//! in the WAL file corresponds to the (HASHTABLE_NPAGE_ONE+1)th frame in
//! the log, and so on.
//!
//! The last index block in a wal-index usually contains less than the full
//! complement of HASHTABLE_NPAGE (or HASHTABLE_NPAGE_ONE) page-numbers,
//! depending on the contents of the WAL file. This does not change the
//! allocated size of the page-mapping array - the page-mapping array merely
//! contains unused entries.
//!
//! Even without using the hash table, the last frame for page P
//! can be found by scanning the page-mapping sections of each index block
//! starting with the last index block and moving toward the first, and
//! within each index block, starting at the end and moving toward the
//! beginning.  The first entry that equals P corresponds to the frame
//! holding the content for that page.
//!
//! The hash table consists of HASHTABLE_NSLOT 16-bit unsigned integers.
//! HASHTABLE_NSLOT = 2*HASHTABLE_NPAGE, and there is one entry in the
//! hash table for each page number in the mapping section, so the hash
//! table is never more than half full.  The expected number of collisions
//! prior to finding a match is 1.  Each entry of the hash table is an
//! 1-based index of an entry in the mapping section of the same
//! index block.   Let K be the 1-based index of the largest entry in
//! the mapping section.  (For index blocks other than the last, K will
//! always be exactly HASHTABLE_NPAGE (4096) and for the last index block
//! K will be (mxFrame%HASHTABLE_NPAGE).)  Unused slots of the hash table
//! contain a value of 0.
//!
//! To look for page P in the hash table, first compute a hash iKey on
//! P as follows:
//!
//!      iKey = (P * 383) % HASHTABLE_NSLOT
//!
//! Then start scanning entries of the hash table, starting with iKey
//! (wrapping around to the beginning when the end of the hash table is
//! reached) until an unused hash slot is found. Let the first unused slot
//! be at index iUnused.  (iUnused might be less than iKey if there was
//! wrap-around.) Because the hash table is never more than half full,
//! the search is guaranteed to eventually hit an unused entry.  Let
//! iMax be the value between iKey and iUnused, closest to iUnused,
//! where aHash[iMax]==P.  If there is no iMax entry (if there exists
//! no hash slot such that aHash[i]==p) then page P is not in the
//! current index block.  Otherwise the iMax-th mapping entry of the
//! current index block corresponds to the last entry that references
//! page P.
//!
//! A hash search begins with the last index block and moves toward the
//! first index block, looking for entries corresponding to page P.  On
//! average, only two or three slots in each index block need to be
//! examined in order to either find the last entry for page P, or to
//! establish that no such entry exists in the block.  Each index block
//! holds over 4000 entries.  So two or three index blocks are sufficient
//! to cover a typical 10 megabyte WAL file, assuming 1K pages.  8 or 10
//! comparisons (on average) suffice to either locate a frame in the
//! WAL or to establish that the frame does not exist in the WAL.  This
//! is much faster than scanning the entire 10MB WAL.
//!
//! Note that entries are added in order of increasing K.  Hence, one
//! reader might be using some value K0 and a second reader that started
//! at a later time (after additional transactions were added to the WAL
//! and to the wal-index) might be using a different value K1, where K1>K0.
//! Both readers can use the same hash table and mapping section to get
//! the correct result.  There may be entries in the hash table with
//! K>K0 but to the first reader, those entries will appear to be unused
//! slots in the hash table and so the first reader will get an answer as
//! if no values greater than K0 had ever been inserted into the hash table
//! in the first place - which is what reader one wants.  Meanwhile, the
//! second reader using K1 will see additional values that were inserted
//! later, which is exactly what reader two wants.
//!
//! When a rollback occurs, the value of K is decreased. Hash table entries
//! that correspond to frames greater than the new K value are removed
//! from the hash table at this point.
unsafe extern "C" {
    fn sqlite3_malloc(__v828: i32) -> *mut ();
    fn sqlite3_malloc64(__v829: u64) -> *mut ();
    fn sqlite3_free(__v830: *mut ());
    fn sqlite3_randomness(N: i32, P: *mut ());
    fn sqlite3_log(iErrCode: i32, zFormat: *const i8, ...);
    fn memcpy(__dest: *mut (), __src: *const (), __n: u64) -> *mut ();
    fn memset(__s: *mut (), __c: i32, __n: u64) -> *mut ();
    fn memcmp(__s1: *const (), __s2: *const (), __n: u64) -> i32;
    fn sqlite3OsClose(__v844: *mut sqlite3_file);
    fn sqlite3OsRead(__v845: *mut sqlite3_file, __v846: *mut (), amt: i32, offset: i64) -> i32;
    fn sqlite3OsWrite(__v849: *mut sqlite3_file, __v850: *const (), amt: i32, offset: i64) -> i32;
    fn sqlite3OsTruncate(__v853: *mut sqlite3_file, size: i64) -> i32;
    fn sqlite3OsSync(__v855: *mut sqlite3_file, __v856: i32) -> i32;
    fn sqlite3OsFileSize(__v857: *mut sqlite3_file, pSize: *mut i64) -> i32;
    fn sqlite3OsLock(__v859: *mut sqlite3_file, __v860: i32) -> i32;
    fn sqlite3OsFileControl(__v861: *mut sqlite3_file, __v862: i32, __v863: *mut ()) -> i32;
    fn sqlite3OsFileControlHint(__v864: *mut sqlite3_file, __v865: i32, __v866: *mut ());
    fn sqlite3OsDeviceCharacteristics(id: *mut sqlite3_file) -> i32;
    fn sqlite3OsShmMap(
        __v868: *mut sqlite3_file,
        __v869: i32,
        __v870: i32,
        __v871: i32,
        __v872: *mut *mut (),
    ) -> i32;
    fn sqlite3OsShmLock(id: *mut sqlite3_file, __v874: i32, __v875: i32, __v876: i32) -> i32;
    fn sqlite3OsShmBarrier(id: *mut sqlite3_file);
    fn sqlite3OsShmUnmap(id: *mut sqlite3_file, __v879: i32) -> i32;
    fn sqlite3OsUnfetch(__v880: *mut sqlite3_file, __v881: i64, __v882: *mut ()) -> i32;
    fn sqlite3OsOpen(
        __v883: *mut sqlite3_vfs,
        __v884: *const i8,
        __v885: *mut sqlite3_file,
        __v886: i32,
        __v887: *mut i32,
    ) -> i32;
    fn sqlite3OsDelete(__v888: *mut sqlite3_vfs, __v889: *const i8, __v890: i32) -> i32;
    fn sqlite3OsSleep(__v891: *mut sqlite3_vfs, __v892: i32) -> i32;
    fn sqlite3SectorSize(__v893: *mut sqlite3_file) -> i32;
    fn sqlite3CorruptError(__v894: i32) -> i32;
    fn sqlite3CantopenError(__v895: i32) -> i32;
    fn sqlite3MallocZero(__v896: u64) -> *mut ();
    fn sqlite3Realloc(__v897: *mut (), __v898: u64) -> *mut ();
    fn sqlite3FaultSim(__v899: i32) -> i32;
    fn sqlite3BeginBenignMalloc();
    fn sqlite3EndBenignMalloc();
    fn sqlite3Get4byte(__v900: *const u8) -> u32;
    fn sqlite3Put4byte(__v901: *mut u8, __v902: u32);
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
struct sqlite3_value {}

#[repr(C)]
#[derive(Clone, Copy)]
struct sqlite3_context {}

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
struct sqlite3_pcache_page {
    pBuf: *mut (),
    pExtra: *mut (),
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
struct _ht {
    count: u32,
    chain: *mut HashElem,
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
struct PgHdr {
    pPage: *mut sqlite3_pcache_page,
    pData: *mut (),
    pExtra: *mut (),
    pCache: *mut PCache,
    pDirty: *mut PgHdr,
    pPager: *mut Pager,
    pgno: u32,
    flags: u16,
    nRef: i64,
    pDirtyNext: *mut PgHdr,
    pDirtyPrev: *mut PgHdr,
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
    trace: __SlateRecord163,
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
    u1: __SlateRecord164,
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
    u: __SlateRecord165,
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
    __slate_bits_0: __slate_bits::__SlateBits63U0,
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
    u: __SlateRecord166,
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
    __slate_bits_0: __slate_bits::__SlateBits87U0,
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
    u: __SlateRecord174,
    pLeft: *mut Expr,
    pRight: *mut Expr,
    x: __SlateRecord175,
    nHeight: i32,
    iTable: i32,
    iColumn: i16,
    iAgg: i16,
    w: __SlateRecord176,
    pAggInfo: *mut AggInfo,
    y: __SlateRecord177,
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
struct RenameToken {}

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
struct SrcItem {
    zName: *mut i8,
    zAlias: *mut i8,
    pSTab: *mut Table,
    fg: __SlateRecord184,
    iCursor: i32,
    colUsed: u64,
    u1: __SlateRecord185,
    u2: __SlateRecord186,
    u3: __SlateRecord187,
    u4: __SlateRecord188,
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
struct TableLock {}

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
    __slate_bits_0: __slate_bits::__SlateBits99U0,
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
    u1: __SlateRecord190,
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
struct VtabCtx {}

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
struct Pager {}

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
struct Btree {}

#[repr(C)]
#[derive(Clone, Copy)]
struct Vdbe {}

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
struct PCache {}

#[repr(C)]
#[derive(Clone, Copy)]
struct sqlite3InitInfo {
    newTnum: u32,
    iDb: u8,
    busy: u8,
    __slate_bits_0: __slate_bits::__SlateBits162U0,
    azInit: *mut *const i8,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord163 {
    xLegacy: Option<unsafe extern "C-unwind" fn(*mut (), *const i8)>,
    xV2: Option<unsafe extern "C-unwind" fn(u32, *mut (), *mut (), *mut ()) -> i32>,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord164 {
    isInterrupted: i32,
    notUsed1: f64,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord165 {
    pHash: *mut FuncDef,
    pDestructor: *mut FuncDestructor,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord166 {
    tab: __SlateRecord167,
    view: __SlateRecord168,
    vtab: __SlateRecord169,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord167 {
    addColOffset: i32,
    pFKey: *mut FKey,
    pDfltList: *mut ExprList,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord168 {
    pSelect: *mut Select,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord169 {
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
union __SlateRecord174 {
    zToken: *mut i8,
    iValue: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord175 {
    pList: *mut ExprList,
    pSelect: *mut Select,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord176 {
    iJoin: i32,
    iOfst: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord177 {
    pTab: *mut Table,
    pWin: *mut Window,
    nReg: i32,
    sub: __SlateRecord178,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord178 {
    iAddr: i32,
    regReturn: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct ExprList_item {
    pExpr: *mut Expr,
    zEName: *mut i8,
    fg: __SlateRecord180,
    u: __SlateRecord181,
}

#[repr(C, align(4))]
#[derive(Clone, Copy)]
struct __SlateRecord180 {
    sortFlags: u8,
    __slate_bits_0: __slate_bits::__SlateBits180U0,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord181 {
    x: __SlateRecord182,
    iConstExprReg: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord182 {
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
struct __SlateRecord184 {
    jointype: u8,
    __slate_bits_0: __slate_bits::__SlateBits184U0,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord185 {
    zIndexedBy: *mut i8,
    pFuncArg: *mut ExprList,
    nRow: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord186 {
    pIBIndex: *mut Index,
    pCteUse: *mut CteUse,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord187 {
    pOn: *mut Expr,
    pUsing: *mut IdList,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord188 {
    pSchema: *mut Schema,
    zDatabase: *mut i8,
    pSubq: *mut Subquery,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord190 {
    cr: __SlateRecord191,
    d: __SlateRecord192,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord191 {
    addrCrTab: i32,
    regRowid: i32,
    regRoot: i32,
    constraintName: Token,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord192 {
    pReturning: *mut Returning,
}

// The maximum (and only) versions of the wal and wal-index formats
// that may be interpreted by this version of SQLite.
//
// If a client begins recovering a WAL file and finds that (a) the checksum
// values in the wal-header are correct and (b) the version field is not
// WAL_MAX_VERSION, recovery fails and SQLite returns SQLITE_CANTOPEN.
//
// Similarly, if a client successfully reads a wal-index header (i.e. the
// checksum test is successful) and finds that the version field is not
// WALINDEX_MAX_VERSION, then no read-transaction is opened and SQLite
// returns SQLITE_CANTOPEN.
// Index numbers for various locking bytes.   WAL_NREADER is the number
// of available reader locks and should be at least 3.  The default
// is SQLITE_SHM_NLOCK==8 and  WAL_NREADER==5.
//
// Technically, the various VFSes are free to implement these locks however
// they see fit.  However, compatibility is encouraged so that VFSes can
// interoperate.  The standard implementation used on both unix and windows
// is for the index number to indicate a byte offset into the
// WalCkptInfo.aLock[] array in the wal-index header.  In other words, all
// locks are on the shm file.  The WALINDEX_LOCK_OFFSET constant (which
// should be 120) is the location in the shm file for the first locking
// byte.
/// The following object holds a copy of the wal-index header content.
///
/// The actual header in the wal-index consists of two copies of this
/// object followed by one instance of the WalCkptInfo object.
/// For all versions of SQLite through 3.10.0 and probably beyond,
/// the locking bytes (WalCkptInfo.aLock) start at offset 120 and
/// the total header size is 136 bytes.
///
/// The szPage value can be any power of 2 between 512 and 32768, inclusive.
/// Or it can be 1 to represent a 65536-byte page.  The latter case was
/// added in 3.7.1 when support for 64K pages was added.
#[repr(C)]
#[derive(Clone, Copy)]
struct WalIndexHdr {
    /// Wal-index version
    iVersion: u32,
    /// Unused (padding) field
    unused: u32,
    /// Counter incremented each transaction
    iChange: u32,
    /// 1 when initialized
    isInit: u8,
    /// True if checksums in WAL are big-endian
    bigEndCksum: u8,
    /// Database page size in bytes. 1==64K
    szPage: u16,
    /// Index of last valid frame in the WAL
    mxFrame: u32,
    /// Size of database in pages
    nPage: u32,
    /// Checksum of last frame in log
    aFrameCksum: [u32; 2],
    /// Two salt values copied from WAL header
    aSalt: [u32; 2],
    /// Checksum over all prior fields
    aCksum: [u32; 2],
}

/// A copy of the following object occurs in the wal-index immediately
/// following the second copy of the WalIndexHdr.  This object stores
/// information used by checkpoint.
///
/// nBackfill is the number of frames in the WAL that have been written
/// back into the database. (We call the act of moving content from WAL to
/// database "backfilling".)  The nBackfill number is never greater than
/// WalIndexHdr.mxFrame.  nBackfill can only be increased by threads
/// holding the WAL_CKPT_LOCK lock (which includes a recovery thread).
/// However, a WAL_WRITE_LOCK thread can move the value of nBackfill from
/// mxFrame back to zero when the WAL is reset.
///
/// nBackfillAttempted is the largest value of nBackfill that a checkpoint
/// has attempted to achieve.  Normally nBackfill==nBackfillAtempted, however
/// the nBackfillAttempted is set before any backfilling is done and the
/// nBackfill is only set after all backfilling completes.  So if a checkpoint
/// crashes, nBackfillAttempted might be larger than nBackfill.  The
/// WalIndexHdr.mxFrame must never be less than nBackfillAttempted.
///
/// The aLock[] field is a set of bytes used for locking.  These bytes should
/// never be read or written.
///
/// There is one entry in aReadMark[] for each reader lock.  If a reader
/// holds read-lock K, then the value in aReadMark[K] is no greater than
/// the mxFrame for that reader.  The value READMARK_NOT_USED (0xffffffff)
/// for any aReadMark[] means that entry is unused.  aReadMark[0] is
/// a special case; its value is never used and it exists as a place-holder
/// to avoid having to offset aReadMark[] indexes by one.  Readers holding
/// WAL_READ_LOCK(0) always ignore the entire WAL and read all content
/// directly from the database.
///
/// The value of aReadMark[K] may only be changed by a thread that
/// is holding an exclusive lock on WAL_READ_LOCK(K).  Thus, the value of
/// aReadMark[K] cannot changed while there is a reader is using that mark
/// since the reader will be holding a shared lock on WAL_READ_LOCK(K).
///
/// The checkpointer may only transfer frames from WAL to database where
/// the frame numbers are less than or equal to every aReadMark[] that is
/// in use (that is, every aReadMark[j] for which there is a corresponding
/// WAL_READ_LOCK(j)).  New readers (usually) pick the aReadMark[] with the
/// largest value and will increase an unused aReadMark[] to mxFrame if there
/// is not already an aReadMark[] equal to mxFrame.  The exception to the
/// previous sentence is when nBackfill equals mxFrame (meaning that everything
/// in the WAL has been backfilled into the database) then new readers
/// will choose aReadMark[0] which has value 0 and hence such reader will
/// get all their all content directly from the database file and ignore
/// the WAL.
///
/// Writers normally append new frames to the end of the WAL.  However,
/// if nBackfill equals mxFrame (meaning that all WAL content has been
/// written back into the database) and if no readers are using the WAL
/// (in other words, if there are no WAL_READ_LOCK(i) where i>0) then
/// the writer will first "reset" the WAL back to the beginning and start
/// writing new content beginning at frame 1.
///
/// We assume that 32-bit loads are atomic and so no locks are needed in
/// order to read from any aReadMark[] entries.
#[repr(C)]
#[derive(Clone, Copy)]
struct WalCkptInfo {
    /// Number of WAL frames backfilled into DB
    nBackfill: u32,
    /// Reader marks
    aReadMark: [u32; 5],
    /// Reserved space for locks
    aLock: [u8; 8],
    /// WAL frames perhaps written, or maybe not
    nBackfillAttempted: u32,
    /// Available for future enhancements
    notUsed0: u32,
}

// This is a schematic view of the complete 136-byte header of the
// wal-index file (also known as the -shm file):
//
//   0: | iVersion                    | **      +-----------------------------+  |
//   4: | (unused padding)            |  |
//      +-----------------------------+  |
//   8: | iChange                     |  |
//      +-------+-------+-------------+  |
//  12: | bInit |  bBig |   szPage    |  |
//      +-------+-------+-------------+  |
//  16: | mxFrame                     |  |  First copy of the
//      +-----------------------------+  |  WalIndexHdr object
//  20: | nPage                       |  |
//      +-----------------------------+  |
//  24: | aFrameCksum                 |  |
//      |                             |  |
//      +-----------------------------+  |
//  32: | aSalt                       |  |
//      |                             |  |
//      +-----------------------------+  |
//  40: | aCksum                      |  |
//      |                             | /
//  48: | iVersion                    | **      +-----------------------------+  |
//  52: | (unused padding)            |  |
//      +-----------------------------+  |
//  56: | iChange                     |  |
//      +-------+-------+-------------+  |
//  60: | bInit |  bBig |   szPage    |  |
//      +-------+-------+-------------+  |  Second copy of the
//  64: | mxFrame                     |  |  WalIndexHdr
//      +-----------------------------+  |
//  68: | nPage                       |  |
//      +-----------------------------+  |
//  72: | aFrameCksum                 |  |
//      |                             |  |
//      +-----------------------------+  |
//  80: | aSalt                       |  |
//      |                             |  |
//      +-----------------------------+  |
//  88: | aCksum                      |  |
//      |                             | /
//  96: | nBackfill                   |
// 100: | 5 read marks                |
//      |                             |
//      |                             |
//      |                             |
//      |                             |
// 120: | Write | Ckpt  | Rcvr | Rd0  | **      +-------+-------+------+------+  ) 8 lock bytes
//      | Read1 | Read2 | Rd3  | Rd4  | /
// 128: | nBackfillAttempted          |
// 132: | (unused padding)            |
// A block of WALINDEX_LOCK_RESERVED bytes beginning at
// WALINDEX_LOCK_OFFSET is reserved for locks. Since some systems
// only support mandatory file-locks, we do not read or write data
// from the region of the file on which locks are applied.
// Size of header before each frame in wal
// Size of write ahead log header, including checksum.
// WAL magic value. Either this value, or the same value with the least
// significant bit also set (WAL_MAGIC | 0x00000001) is stored in 32-bit
// big-endian format in the first 4 bytes of a WAL file.
//
// If the LSB is set, then the checksums for each frame within the WAL
// file are calculated by treating all data as an array of 32-bit
// big-endian words. Otherwise, they are calculated by interpreting
// all data as 32-bit little-endian words.
// Return the offset of frame iFrame in the write-ahead log file,
// assuming a database page size of szPage bytes. The offset returned
// is to the start of the write-ahead log frame-header.
/// An open write-ahead log file is represented by an instance of the
/// following object.
///
/// writeLock:
///   This is usually set to 1 whenever the WRITER lock is held. However,
///   if it is set to 2, then the WRITER lock is held but must be released
///   by walHandleException() if a SEH exception is thrown.
#[repr(C)]
#[derive(Clone, Copy)]
struct Wal {
    /// The VFS used to create pDbFd
    pVfs: *mut sqlite3_vfs,
    /// File handle for the database file
    pDbFd: *mut sqlite3_file,
    /// File handle for WAL file
    pWalFd: *mut sqlite3_file,
    /// Value to pass to log callback (or 0)
    iCallback: u32,
    /// Truncate WAL to this size upon reset
    mxWalSize: i64,
    /// Size of array apWiData
    nWiData: i32,
    /// Size of first block written to WAL file
    szFirstBlock: i32,
    /// Pointer to wal-index content in memory
    apWiData: *mut *mut u32,
    /// Database page size
    szPage: u32,
    /// Which read lock is being held.  -1 for none
    readLock: i16,
    /// Flags to use to sync header writes
    syncFlags: u8,
    /// Non-zero if connection is in exclusive mode
    exclusiveMode: u8,
    /// True if in a write transaction
    writeLock: u8,
    /// True if holding a checkpoint lock
    ckptLock: u8,
    /// WAL_RDWR, WAL_RDONLY, or WAL_SHM_RDONLY
    readOnly: u8,
    /// True to truncate WAL file on commit
    truncateOnCommit: u8,
    /// Fsync the WAL header if true
    syncHeader: u8,
    /// Pad transactions out to the next sector
    padToSectorBoundary: u8,
    /// SHM content is read-only and unreliable
    bShmUnreliable: u8,
    /// Wal-index header for current transaction
    hdr: WalIndexHdr,
    /// Ignore wal frames before this one
    minFrame: u32,
    /// On commit, recalculate checksums from here
    iReCksum: u32,
    /// Name of WAL file
    zWalName: *const i8,
    /// Checkpoint sequence counter in the wal-header
    nCkpt: u32,
}

// Candidate values for Wal.exclusiveMode.
// Possible values for WAL.readOnly
// Normal read/write connection
// The WAL file is readonly
// The SHM file is readonly
/// Each page of the wal-index mapping contains a hash-table made up of
/// an array of HASHTABLE_NSLOT elements of the following type.
/// This structure is used to implement an iterator that loops through
/// all frames in the WAL in database page order. Where two or more frames
/// correspond to the same database page, the iterator visits only the
/// frame most recently written to the WAL (in other words, the frame with
/// the largest index).
///
/// The internals of this structure are only accessed by:
///
///   walIteratorInit() - Create a new iterator,
///   walIteratorNext() - Step an iterator,
///   walIteratorFree() - Free an iterator.
///
/// This functionality is used by the checkpoint code (see walCheckpoint()).
#[repr(C)]
#[derive(Clone, Copy)]
struct WalIterator {
    /// Last result returned from the iterator
    iPrior: u32,
    /// Number of entries in aSegment[]
    nSegment: i32,
    /// One for every 32KB page in the wal-index
    aSegment: [WalSegment; 0],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct WalSegment {
    /// Next slot in aIndex[] not yet returned
    iNext: i32,
    /// i0, i1, i2... such that aPgno[iN] ascend
    aIndex: *mut u16,
    /// Array of page numbers.
    aPgno: *mut u32,
    /// Nr. of entries in aPgno[] and aIndex[]
    nEntry: i32,
    /// Frame number associated with aPgno[0]
    iZero: i32,
}

// Size (in bytes) of a WalIterator object suitable for N or fewer segments
// Define the parameters of the hash tables in the wal-index file. There
// is a hash-table following every HASHTABLE_NPAGE page numbers in the
// wal-index.
//
// Changing any of these constants will alter the wal-index format and
// create incompatibilities.
// Must be power of 2
// Should be prime
// Must be a power of 2
// The block of page numbers associated with the first hash-table in a
// wal-index is smaller than usual. This is so that there is a complete
// hash-table on each aligned 32KB page of the wal-index.
// The wal-index is divided into pages of WALINDEX_PGSZ bytes each.
// Structured Exception Handling (SEH) is a Windows-specific technique
// for catching exceptions raised while accessing memory-mapped files.
//
// The -DSQLITE_USE_SEH compile-time option means to use SEH to catch and
// deal with system-level errors that arise during WAL -shm file processing.
// Without this compile-time option, any system-level faults that appear
// while accessing the memory-mapped -shm file will cause a process-wide
// signal to be deliver, which will more than likely cause the entire
// process to exit.
/// Obtain a pointer to the iPage'th page of the wal-index. The wal-index
/// is broken into pages of WALINDEX_PGSZ bytes. Wal-index pages are
/// numbered from zero.
///
/// If the wal-index is currently smaller the iPage pages then the size
/// of the wal-index might be increased, but only if it is safe to do
/// so.  It is safe to enlarge the wal-index if pWal->writeLock is true
/// or pWal->exclusiveMode==WAL_HEAPMEMORY_MODE.
///
/// Three possible result scenarios:
///
///   (1)  rc==SQLITE_OK    and *ppPage==Requested-Wal-Index-Page
///   (2)  rc>=SQLITE_ERROR and *ppPage==NULL
///   (3)  rc==SQLITE_OK    and *ppPage==NULL  // only if iPage==0
///
/// Scenario (3) can only occur when pWal->writeLock is false and iPage==0
///
/// # Arguments
///
/// * `pWal` - The WAL context
/// * `iPage` - The page we seek
/// * `ppPage` - Write the page pointer here
fn walIndexPageRealloc(mut pWal: *mut Wal, mut iPage: i32, mut ppPage: *mut *mut u32) -> i32 {
    let mut rc: i32 = 0 as i32;
    // Enlarge the pWal->apWiData[] array if required
    if (unsafe { (*pWal).nWiData }) <= iPage {
        let mut nByte: i64 =
            (8 as u64).wrapping_mul((((1 as i32) as i64) + (iPage as i64)) as u64) as i64;
        let mut apNew: *mut *mut u32 = unsafe { std::mem::zeroed() };
        apNew = (unsafe { sqlite3Realloc((unsafe { (*pWal).apWiData }) as *mut (), nByte as u64) })
            as *mut *mut u32;
        if !(apNew != std::ptr::null_mut::<*mut u32>()) {
            unsafe {
                *ppPage = std::ptr::null_mut::<u32>();
            }
            return 7 as i32;
        }
        unsafe {
            memset(
                (unsafe { apNew.offset((unsafe { (*pWal).nWiData }) as isize) }) as *mut (),
                0 as i32,
                (8 as u64).wrapping_mul(
                    ((iPage + (1 as i32) - unsafe { (*pWal).nWiData }) as i64) as u64,
                ),
            )
        };
        unsafe {
            (*pWal).apWiData = apNew;
        }
        unsafe {
            (*pWal).nWiData = iPage + (1 as i32);
        }
    }
    // Request a pointer to the required page from the VFS
    0 as i32;
    if (((unsafe { (*pWal).exclusiveMode }) as u32) as i32) == (2 as i32) {
        unsafe {
            *unsafe { unsafe { (*pWal).apWiData }.offset(iPage as isize) } = (unsafe {
                sqlite3MallocZero(
                    (2 as u64)
                        .wrapping_mul((((4096 as i32) * (2 as i32)) as i64) as u64)
                        .wrapping_add((((4096 as i32) as i64) as u64).wrapping_mul(4 as u64)),
                )
            })
                as *mut u32;
        }
        if !((unsafe { *unsafe { unsafe { (*pWal).apWiData }.offset(iPage as isize) } })
            != std::ptr::null_mut::<u32>())
        {
            rc = 7 as i32;
        }
    } else {
        rc = unsafe {
            sqlite3OsShmMap(
                unsafe { (*pWal).pDbFd },
                iPage,
                ((2 as u64)
                    .wrapping_mul((((4096 as i32) * (2 as i32)) as i64) as u64)
                    .wrapping_add((((4096 as i32) as i64) as u64).wrapping_mul(4 as u64))
                    as u32) as i32,
                ((unsafe { (*pWal).writeLock }) as u32) as i32,
                (unsafe { unsafe { (*pWal).apWiData }.offset(iPage as isize) }) as *mut *mut (),
            )
        };
        0 as i32;
        {}
        if rc == (0 as i32) {
            let __v999: bool;
            if iPage > (0 as i32) {
                __v999 = (unsafe { sqlite3FaultSim(600 as i32) }) != (0 as i32);
            } else {
                __v999 = false as bool;
            }
            if __v999 {
                rc = 7 as i32;
            }
        } else {
            if rc & (255 as i32) == (8 as i32) {
                let __v1000: *mut Wal = pWal;
                let __v1001: u8 = unsafe { (*__v1000).readOnly };
                let __v1002: u8 = ((((__v1001 as u32) as i32) | (2 as i32)) as i8) as u8;
                unsafe {
                    (*__v1000).readOnly = __v1002;
                }
                if rc == (8 as i32) {
                    rc = 0 as i32;
                }
            }
        }
    }
    unsafe {
        *ppPage = unsafe { *unsafe { unsafe { (*pWal).apWiData }.offset(iPage as isize) } };
    }
    0 as i32;
    return rc;
}

#[repr(C)]
#[derive(Clone, Copy)]
struct Sublist {
    /// Number of elements in aList
    nList: i32,
    /// Pointer to sub-list content
    aList: *mut u16,
}

/// # Arguments
///
/// * `pWal` - The WAL context
/// * `iPage` - The page we seek
/// * `ppPage` - Write the page pointer here
fn walIndexPage(mut pWal: *mut Wal, mut iPage: i32, mut ppPage: *mut *mut u32) -> i32 {
    0 as i32;
    {}
    let __v1003: bool;
    if (unsafe { (*pWal).nWiData }) <= iPage {
        __v1003 = true as bool;
    } else {
        let __v1004: *mut u32 =
            unsafe { *unsafe { unsafe { (*pWal).apWiData }.offset(iPage as isize) } };
        unsafe {
            *ppPage = __v1004;
        }
        __v1003 = __v1004 == std::ptr::null_mut::<u32>();
    }
    if __v1003 {
        return walIndexPageRealloc(pWal, iPage, ppPage);
    }
    return 0 as i32;
}

#[repr(C, align(16))]
struct __SlateAlign16<T>(T);

mod __slate_bits {
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
    pub struct __SlateBits63U0 {
        #[bits(4)]
        pub notNull: u32,
        #[bits(4)]
        pub eCType: u32,
    }
    #[bitfields::bitfield([u8; 2], c_names = true, new = false, from_into_bits = false, from_traits = false, default = false, debug = false, builder = false, bit_ops = false)]
    pub struct __SlateBits180U0 {
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
    #[bitfields::bitfield([u8; 3], c_names = true, new = false, from_into_bits = false, from_traits = false, default = false, debug = false, builder = false, bit_ops = false)]
    pub struct __SlateBits184U0 {
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
    pub struct __SlateBits87U0 {
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
    pub struct __SlateBits162U0 {
        #[bits(1)]
        pub orphanTrigger: u32,
        #[bits(2)]
        pub imposterTable: u32,
        #[bits(1)]
        pub reopenMemdb: u32,
        #[bits(4, access = na)]
        pub __slate_pad_3: u8,
    }
    #[bitfields::bitfield([u8; 2], c_names = true, new = false, from_into_bits = false, from_traits = false, default = false, debug = false, builder = false, bit_ops = false)]
    pub struct __SlateBits99U0 {
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
}

/// Return a pointer to the WalCkptInfo structure in the wal-index.
fn walCkptInfo(mut pWal: *mut Wal) -> *mut WalCkptInfo {
    0 as i32;
    0 as i32;
    {}
    return (unsafe {
        unsafe { *unsafe { unsafe { (*pWal).apWiData }.offset((0 as i32) as isize) } }
            .offset(((48 as u64) / (((2 as i32) as i64) as u64)) as isize)
    }) as *mut WalCkptInfo;
}

/// Return a pointer to the WalIndexHdr structure in the wal-index.
fn walIndexHdr(mut pWal: *mut Wal) -> *mut WalIndexHdr {
    0 as i32;
    0 as i32;
    {}
    return (unsafe { *unsafe { unsafe { (*pWal).apWiData }.offset((0 as i32) as isize) } })
        as *mut WalIndexHdr;
}

// The argument to this macro must be of type u32. On a little-endian
// architecture, it returns the u32 value that results from interpreting
// the 4 bytes as a big-endian value. On a big-endian architecture, it
// returns the value that would be produced by interpreting the 4 bytes
// of the input value as a little-endian integer.
/// Generate or extend an 8 byte checksum based on the data in
/// array aByte[] and the initial values of aIn[0] and aIn[1] (or
/// initial values of 0 and 0 if aIn==NULL).
///
/// The checksum is written back into aOut[] before returning.
///
/// nByte must be a positive multiple of 8.
///
/// # Arguments
///
/// * `nativeCksum` - True for native byte-order, false for non-native
/// * `a` - Content to be checksummed
/// * `nByte` - Bytes of content in a[].  Must be a multiple of 8.
/// * `aIn` - Initial checksum value input
/// * `aOut` - OUT: Final checksum value output
fn walChecksumBytes(
    mut nativeCksum: i32,
    mut a: *mut u8,
    mut nByte: i32,
    mut aIn: *const u32,
    mut aOut: *mut u32,
) {
    let mut s1: u32 = 0 as u32;
    let mut s2: u32 = 0 as u32;
    let mut aData: *mut u32 = a as *mut u32;
    let mut aEnd: *mut u32 = (unsafe { a.offset(nByte as isize) }) as *mut u32;
    if aIn != std::ptr::null::<u32>() {
        s1 = unsafe { *unsafe { aIn.offset((0 as i32) as isize) } };
        s2 = unsafe { *unsafe { aIn.offset((1 as i32) as isize) } };
    } else {
        s2 = (0 as i32) as u32;
        s1 = (0 as i32) as u32;
    }
    // nByte is a multiple of 8 between 8 and 65536
    0 as i32;
    if !(nativeCksum != (0 as i32)) {
        '__slate_break_957: loop {
            let __v1005: u32 = s1;
            let __v1006: u32 = __v1005.wrapping_add(
                (((unsafe { *unsafe { aData.offset((0 as i32) as isize) } })
                    & ((255 as i32) as u32))
                    << (24 as i32))
                    .wrapping_add(
                        ((unsafe { *unsafe { aData.offset((0 as i32) as isize) } })
                            & ((65280 as i32) as u32))
                            << (8 as i32),
                    )
                    .wrapping_add(
                        ((unsafe { *unsafe { aData.offset((0 as i32) as isize) } })
                            & ((16711680 as i32) as u32))
                            >> (8 as i32),
                    )
                    .wrapping_add(
                        ((unsafe { *unsafe { aData.offset((0 as i32) as isize) } })
                            & (4278190080 as u32))
                            >> (24 as i32),
                    )
                    .wrapping_add(s2),
            );
            s1 = __v1006;
            let __v1007: u32 = s2;
            let __v1008: u32 = __v1007.wrapping_add(
                (((unsafe { *unsafe { aData.offset((1 as i32) as isize) } })
                    & ((255 as i32) as u32))
                    << (24 as i32))
                    .wrapping_add(
                        ((unsafe { *unsafe { aData.offset((1 as i32) as isize) } })
                            & ((65280 as i32) as u32))
                            << (8 as i32),
                    )
                    .wrapping_add(
                        ((unsafe { *unsafe { aData.offset((1 as i32) as isize) } })
                            & ((16711680 as i32) as u32))
                            >> (8 as i32),
                    )
                    .wrapping_add(
                        ((unsafe { *unsafe { aData.offset((1 as i32) as isize) } })
                            & (4278190080 as u32))
                            >> (24 as i32),
                    )
                    .wrapping_add(s1),
            );
            s2 = __v1008;
            let __v1009: *mut u32 = aData;
            let __v1010: *mut u32 = unsafe { __v1009.offset((2 as i32) as isize) };
            aData = __v1010;
            if !(aData < aEnd) {
                break;
            }
        }
    } else {
        if nByte % (64 as i32) == (0 as i32) {
            '__slate_break_958: loop {
                let __v1011: u32 = s1;
                let __v1012: *mut u32 = aData;
                let __v1013: *mut u32 = unsafe { __v1012.offset((1 as i32) as isize) };
                aData = __v1013;
                let __v1014: u32 = __v1011.wrapping_add(unsafe { *__v1012 }.wrapping_add(s2));
                s1 = __v1014;
                let __v1015: u32 = s2;
                let __v1016: *mut u32 = aData;
                let __v1017: *mut u32 = unsafe { __v1016.offset((1 as i32) as isize) };
                aData = __v1017;
                let __v1018: u32 = __v1015.wrapping_add(unsafe { *__v1016 }.wrapping_add(s1));
                s2 = __v1018;
                let __v1019: u32 = s1;
                let __v1020: *mut u32 = aData;
                let __v1021: *mut u32 = unsafe { __v1020.offset((1 as i32) as isize) };
                aData = __v1021;
                let __v1022: u32 = __v1019.wrapping_add(unsafe { *__v1020 }.wrapping_add(s2));
                s1 = __v1022;
                let __v1023: u32 = s2;
                let __v1024: *mut u32 = aData;
                let __v1025: *mut u32 = unsafe { __v1024.offset((1 as i32) as isize) };
                aData = __v1025;
                let __v1026: u32 = __v1023.wrapping_add(unsafe { *__v1024 }.wrapping_add(s1));
                s2 = __v1026;
                let __v1027: u32 = s1;
                let __v1028: *mut u32 = aData;
                let __v1029: *mut u32 = unsafe { __v1028.offset((1 as i32) as isize) };
                aData = __v1029;
                let __v1030: u32 = __v1027.wrapping_add(unsafe { *__v1028 }.wrapping_add(s2));
                s1 = __v1030;
                let __v1031: u32 = s2;
                let __v1032: *mut u32 = aData;
                let __v1033: *mut u32 = unsafe { __v1032.offset((1 as i32) as isize) };
                aData = __v1033;
                let __v1034: u32 = __v1031.wrapping_add(unsafe { *__v1032 }.wrapping_add(s1));
                s2 = __v1034;
                let __v1035: u32 = s1;
                let __v1036: *mut u32 = aData;
                let __v1037: *mut u32 = unsafe { __v1036.offset((1 as i32) as isize) };
                aData = __v1037;
                let __v1038: u32 = __v1035.wrapping_add(unsafe { *__v1036 }.wrapping_add(s2));
                s1 = __v1038;
                let __v1039: u32 = s2;
                let __v1040: *mut u32 = aData;
                let __v1041: *mut u32 = unsafe { __v1040.offset((1 as i32) as isize) };
                aData = __v1041;
                let __v1042: u32 = __v1039.wrapping_add(unsafe { *__v1040 }.wrapping_add(s1));
                s2 = __v1042;
                let __v1043: u32 = s1;
                let __v1044: *mut u32 = aData;
                let __v1045: *mut u32 = unsafe { __v1044.offset((1 as i32) as isize) };
                aData = __v1045;
                let __v1046: u32 = __v1043.wrapping_add(unsafe { *__v1044 }.wrapping_add(s2));
                s1 = __v1046;
                let __v1047: u32 = s2;
                let __v1048: *mut u32 = aData;
                let __v1049: *mut u32 = unsafe { __v1048.offset((1 as i32) as isize) };
                aData = __v1049;
                let __v1050: u32 = __v1047.wrapping_add(unsafe { *__v1048 }.wrapping_add(s1));
                s2 = __v1050;
                let __v1051: u32 = s1;
                let __v1052: *mut u32 = aData;
                let __v1053: *mut u32 = unsafe { __v1052.offset((1 as i32) as isize) };
                aData = __v1053;
                let __v1054: u32 = __v1051.wrapping_add(unsafe { *__v1052 }.wrapping_add(s2));
                s1 = __v1054;
                let __v1055: u32 = s2;
                let __v1056: *mut u32 = aData;
                let __v1057: *mut u32 = unsafe { __v1056.offset((1 as i32) as isize) };
                aData = __v1057;
                let __v1058: u32 = __v1055.wrapping_add(unsafe { *__v1056 }.wrapping_add(s1));
                s2 = __v1058;
                let __v1059: u32 = s1;
                let __v1060: *mut u32 = aData;
                let __v1061: *mut u32 = unsafe { __v1060.offset((1 as i32) as isize) };
                aData = __v1061;
                let __v1062: u32 = __v1059.wrapping_add(unsafe { *__v1060 }.wrapping_add(s2));
                s1 = __v1062;
                let __v1063: u32 = s2;
                let __v1064: *mut u32 = aData;
                let __v1065: *mut u32 = unsafe { __v1064.offset((1 as i32) as isize) };
                aData = __v1065;
                let __v1066: u32 = __v1063.wrapping_add(unsafe { *__v1064 }.wrapping_add(s1));
                s2 = __v1066;
                let __v1067: u32 = s1;
                let __v1068: *mut u32 = aData;
                let __v1069: *mut u32 = unsafe { __v1068.offset((1 as i32) as isize) };
                aData = __v1069;
                let __v1070: u32 = __v1067.wrapping_add(unsafe { *__v1068 }.wrapping_add(s2));
                s1 = __v1070;
                let __v1071: u32 = s2;
                let __v1072: *mut u32 = aData;
                let __v1073: *mut u32 = unsafe { __v1072.offset((1 as i32) as isize) };
                aData = __v1073;
                let __v1074: u32 = __v1071.wrapping_add(unsafe { *__v1072 }.wrapping_add(s1));
                s2 = __v1074;
                if !(aData < aEnd) {
                    break;
                }
            }
        } else {
            '__slate_break_959: loop {
                let __v1075: u32 = s1;
                let __v1076: *mut u32 = aData;
                let __v1077: *mut u32 = unsafe { __v1076.offset((1 as i32) as isize) };
                aData = __v1077;
                let __v1078: u32 = __v1075.wrapping_add(unsafe { *__v1076 }.wrapping_add(s2));
                s1 = __v1078;
                let __v1079: u32 = s2;
                let __v1080: *mut u32 = aData;
                let __v1081: *mut u32 = unsafe { __v1080.offset((1 as i32) as isize) };
                aData = __v1081;
                let __v1082: u32 = __v1079.wrapping_add(unsafe { *__v1080 }.wrapping_add(s1));
                s2 = __v1082;
                if !(aData < aEnd) {
                    break;
                }
            }
        }
    }
    0 as i32;
    unsafe {
        *unsafe { aOut.offset((0 as i32) as isize) } = s1;
    }
    unsafe {
        *unsafe { aOut.offset((1 as i32) as isize) } = s2;
    }
}

/// If there is the possibility of concurrent access to the SHM file
/// from multiple threads and/or processes, then do a memory barrier.
fn walShmBarrier(mut pWal: *mut Wal) {
    if (((unsafe { (*pWal).exclusiveMode }) as u32) as i32) != (2 as i32) {
        unsafe { sqlite3OsShmBarrier(unsafe { (*pWal).pDbFd }) };
    }
}

// Add the SQLITE_NO_TSAN as part of the return-type of a function
// definition as a hint that the function contains constructs that
// might give false-positive TSAN warnings.
//
// See tag-20200519-1.
/// Write the header information in pWal->hdr into the wal-index.
///
/// The checksum on pWal->hdr is updated before it is written.
fn walIndexWriteHdr(mut pWal: *mut Wal) {
    let mut aHdr: *mut WalIndexHdr = walIndexHdr(pWal);
    let mut nCksum: i32 = ((40 as u64) as u32) as i32;
    0 as i32;
    unsafe {
        (*pWal).hdr.isInit = ((1 as i32) as i8) as u8;
    }
    unsafe {
        (*pWal).hdr.iVersion = (3007000 as i32) as u32;
    }
    walChecksumBytes(
        1 as i32,
        (unsafe { std::ptr::addr_of_mut!((*pWal).hdr) }) as *mut u8,
        nCksum,
        std::ptr::null::<u32>(),
        unsafe { (*pWal).hdr.aCksum.as_mut_ptr() as *mut u32 },
    );
    // Possible TSAN false-positive.  See tag-20200519-1
    unsafe {
        memcpy(
            (unsafe { aHdr.offset((1 as i32) as isize) }) as *mut (),
            (unsafe { std::ptr::addr_of_mut!((*pWal).hdr) }) as *const (),
            48 as u64,
        )
    };
    walShmBarrier(pWal);
    unsafe {
        memcpy(
            (unsafe { aHdr.offset((0 as i32) as isize) }) as *mut (),
            (unsafe { std::ptr::addr_of_mut!((*pWal).hdr) }) as *const (),
            48 as u64,
        )
    };
}

/// This function encodes a single frame header and writes it to a buffer
/// supplied by the caller. A frame-header is made up of a series of
/// 4-byte big-endian integers, as follows:
///
///     0: Page number.
///     4: For commit records, the size of the database image in pages
///        after the commit. For all other records, zero.
///     8: Salt-1 (copied from the wal-header)
///    12: Salt-2 (copied from the wal-header)
///    16: Checksum-1.
///    20: Checksum-2.
///
/// # Arguments
///
/// * `pWal` - The write-ahead log
/// * `iPage` - Database page number for frame
/// * `nTruncate` - New db size (or 0 for non-commit frames)
/// * `aData` - Pointer to page data
/// * `aFrame` - OUT: Write encoded frame here
fn walEncodeFrame(
    mut pWal: *mut Wal,
    mut iPage: u32,
    mut nTruncate: u32,
    mut aData: *mut u8,
    mut aFrame: *mut u8,
) {
    let mut nativeCksum: i32 = 0 as i32; // True for native byte-order checksums
    let mut aCksum: *mut u32 = unsafe { (*pWal).hdr.aFrameCksum.as_mut_ptr() as *mut u32 };
    0 as i32;
    unsafe { sqlite3Put4byte(unsafe { aFrame.offset((0 as i32) as isize) }, iPage) };
    unsafe { sqlite3Put4byte(unsafe { aFrame.offset((4 as i32) as isize) }, nTruncate) };
    if (unsafe { (*pWal).iReCksum }) == ((0 as i32) as u32) {
        unsafe {
            memcpy(
                (unsafe { aFrame.offset((8 as i32) as isize) }) as *mut (),
                (unsafe { (*pWal).hdr.aSalt.as_mut_ptr() as *mut u32 }) as *const (),
                ((8 as i32) as i64) as u64,
            )
        };
        nativeCksum = ((((unsafe { (*pWal).hdr.bigEndCksum }) as u32) as i32) == (0 as i32)) as i32;
        walChecksumBytes(nativeCksum, aFrame, 8 as i32, aCksum as *const u32, aCksum);
        walChecksumBytes(
            nativeCksum,
            aData,
            (unsafe { (*pWal).szPage }) as i32,
            aCksum as *const u32,
            aCksum,
        );
        unsafe {
            sqlite3Put4byte(unsafe { aFrame.offset((16 as i32) as isize) }, unsafe {
                *unsafe { aCksum.offset((0 as i32) as isize) }
            })
        };
        unsafe {
            sqlite3Put4byte(unsafe { aFrame.offset((20 as i32) as isize) }, unsafe {
                *unsafe { aCksum.offset((1 as i32) as isize) }
            })
        };
    } else {
        unsafe {
            memset(
                (unsafe { aFrame.offset((8 as i32) as isize) }) as *mut (),
                0 as i32,
                ((16 as i32) as i64) as u64,
            )
        };
    }
}

/// Check to see if the frame with header in aFrame[] and content
/// in aData[] is valid.  If it is a valid frame, fill *piPage and
/// *pnTruncate and return true.  Return if the frame is not valid.
///
/// # Arguments
///
/// * `pWal` - The write-ahead log
/// * `piPage` - OUT: Database page number for frame
/// * `pnTruncate` - OUT: New db size (or 0 if not commit)
/// * `aData` - Pointer to page data (for checksum)
/// * `aFrame` - Frame data
fn walDecodeFrame(
    mut pWal: *mut Wal,
    mut piPage: *mut u32,
    mut pnTruncate: *mut u32,
    mut aData: *mut u8,
    mut aFrame: *mut u8,
) -> i32 {
    let mut nativeCksum: i32 = 0 as i32; // True for native byte-order checksums
    let mut aCksum: *mut u32 = unsafe { (*pWal).hdr.aFrameCksum.as_mut_ptr() as *mut u32 };
    let mut pgno: u32 = 0 as u32; // Page number of the frame
    0 as i32;
    // A frame is only valid if the salt values in the frame-header
    // match the salt values in the wal-header.
    if (unsafe {
        memcmp(
            (unsafe { std::ptr::addr_of_mut!((*pWal).hdr.aSalt) }) as *const (),
            (unsafe { aFrame.offset((8 as i32) as isize) }) as *const (),
            ((8 as i32) as i64) as u64,
        )
    }) != (0 as i32)
    {
        return 0 as i32;
    }
    // A frame is only valid if the page number is greater than zero.
    pgno = unsafe { sqlite3Get4byte((unsafe { aFrame.offset((0 as i32) as isize) }) as *const u8) };
    if pgno == ((0 as i32) as u32) {
        return 0 as i32;
    }
    // Need a valid page size
    if !((unsafe { (*pWal).szPage }) != (0 as u32)) {
        return 0 as i32;
    }
    // A frame is only valid if a checksum of the WAL header,
    // all prior frames, the first 16 bytes of this frame-header,
    // and the frame-data matches the checksum in the last 8
    // bytes of this frame-header.
    nativeCksum = ((((unsafe { (*pWal).hdr.bigEndCksum }) as u32) as i32) == (0 as i32)) as i32;
    walChecksumBytes(nativeCksum, aFrame, 8 as i32, aCksum as *const u32, aCksum);
    walChecksumBytes(
        nativeCksum,
        aData,
        (unsafe { (*pWal).szPage }) as i32,
        aCksum as *const u32,
        aCksum,
    );
    let __v1083: bool;
    if (unsafe { *unsafe { aCksum.offset((0 as i32) as isize) } })
        != unsafe { sqlite3Get4byte((unsafe { aFrame.offset((16 as i32) as isize) }) as *const u8) }
    {
        __v1083 = true as bool;
    } else {
        __v1083 = (unsafe { *unsafe { aCksum.offset((1 as i32) as isize) } })
            != unsafe {
                sqlite3Get4byte((unsafe { aFrame.offset((20 as i32) as isize) }) as *const u8)
            };
    }
    if __v1083 {
        // Checksum failed.
        return 0 as i32;
    }
    // If we reach this point, the frame is valid.  Return the page number
    // and the new database size.
    unsafe {
        *piPage = pgno;
    }
    unsafe {
        *pnTruncate = unsafe {
            sqlite3Get4byte((unsafe { aFrame.offset((4 as i32) as isize) }) as *const u8)
        };
    }
    return 1 as i32;
}

/// Set or release locks on the WAL.  Locks are either shared or exclusive.
/// A lock cannot be moved directly between shared and exclusive - it must go
/// through the unlocked state first.
///
/// In locking_mode=EXCLUSIVE, all of these routines become no-ops.
fn walLockShared(mut pWal: *mut Wal, mut lockIdx: i32) -> i32 {
    let mut rc: i32 = 0 as i32;
    if (unsafe { (*pWal).exclusiveMode }) != (0 as u8) {
        return 0 as i32;
    }
    rc = unsafe {
        sqlite3OsShmLock(
            unsafe { (*pWal).pDbFd },
            lockIdx,
            1 as i32,
            (2 as i32) | (4 as i32),
        )
    };
    {}
    return rc;
}

fn walUnlockShared(mut pWal: *mut Wal, mut lockIdx: i32) {
    if (unsafe { (*pWal).exclusiveMode }) != (0 as u8) {
        return;
    }
    unsafe {
        sqlite3OsShmLock(
            unsafe { (*pWal).pDbFd },
            lockIdx,
            1 as i32,
            (1 as i32) | (4 as i32),
        )
    };
    {}
}

fn walLockExclusive(mut pWal: *mut Wal, mut lockIdx: i32, mut n: i32) -> i32 {
    let mut rc: i32 = 0 as i32;
    if (unsafe { (*pWal).exclusiveMode }) != (0 as u8) {
        return 0 as i32;
    }
    rc = unsafe {
        sqlite3OsShmLock(
            unsafe { (*pWal).pDbFd },
            lockIdx,
            n,
            (2 as i32) | (8 as i32),
        )
    };
    {}
    return rc;
}

fn walUnlockExclusive(mut pWal: *mut Wal, mut lockIdx: i32, mut n: i32) {
    if (unsafe { (*pWal).exclusiveMode }) != (0 as u8) {
        return;
    }
    unsafe {
        sqlite3OsShmLock(
            unsafe { (*pWal).pDbFd },
            lockIdx,
            n,
            (1 as i32) | (8 as i32),
        )
    };
    {}
}

/// Compute a hash on a page number.  The resulting hash value must land
/// between 0 and (HASHTABLE_NSLOT-1).  The walNextHash() function advances
/// the hash to the next value in the event of a collision.
fn walHash(mut iPage: u32) -> i32 {
    0 as i32;
    0 as i32;
    return (iPage.wrapping_mul((383 as i32) as u32)
        & (((4096 as i32) * (2 as i32) - (1 as i32)) as u32)) as i32;
}

fn walNextHash(mut iPriorHash: i32) -> i32 {
    return iPriorHash + (1 as i32) & (4096 as i32) * (2 as i32) - (1 as i32);
}

/// An instance of the WalHashLoc object is used to describe the location
/// of a page hash table in the wal-index.  This becomes the return value
/// from walHashGet().
#[repr(C)]
#[derive(Clone, Copy)]
struct WalHashLoc {
    /// Start of the wal-index hash table
    aHash: *mut u16,
    /// aPgno[1] is the page of first frame indexed
    aPgno: *mut u32,
    /// One less than the frame number of first indexed
    iZero: u32,
}

/// Return pointers to the hash table and page number array stored on
/// page iHash of the wal-index. The wal-index is broken into 32KB pages
/// numbered starting from 0.
///
/// Set output variable pLoc->aHash to point to the start of the hash table
/// in the wal-index file. Set pLoc->iZero to one less than the frame
/// number of the first frame indexed by this hash table. If a
/// slot in the hash table is set to N, it refers to frame number
/// (pLoc->iZero+N) in the log.
///
/// Finally, set pLoc->aPgno so that pLoc->aPgno[0] is the page number of the
/// first frame indexed by the hash table, frame (pLoc->iZero).
///
/// # Arguments
///
/// * `pWal` - WAL handle
/// * `iHash` - Find the iHash'th table
/// * `pLoc` - OUT: Hash table location
fn walHashGet(mut pWal: *mut Wal, mut iHash: i32, mut pLoc: *mut WalHashLoc) -> i32 {
    let mut rc: i32 = 0 as i32; // Return code
    rc = walIndexPage(pWal, iHash, unsafe {
        std::ptr::addr_of_mut!((*pLoc).aPgno)
    });
    0 as i32;
    if (unsafe { (*pLoc).aPgno }) != std::ptr::null_mut::<u32>() {
        unsafe {
            (*pLoc).aHash =
                (unsafe { unsafe { (*pLoc).aPgno }.offset((4096 as i32) as isize) }) as *mut u16;
        }
        if iHash == (0 as i32) {
            unsafe {
                (*pLoc).aPgno = unsafe {
                    unsafe { (*pLoc).aPgno }.offset(
                        ((48 as u64)
                            .wrapping_mul(((2 as i32) as i64) as u64)
                            .wrapping_add(40 as u64)
                            / (4 as u64)) as isize,
                    )
                };
            }
            unsafe {
                (*pLoc).iZero = (0 as i32) as u32;
            }
        } else {
            unsafe {
                (*pLoc).iZero = (((4096 as i32) as i64) as u64)
                    .wrapping_sub(
                        (48 as u64)
                            .wrapping_mul(((2 as i32) as i64) as u64)
                            .wrapping_add(40 as u64)
                            / (4 as u64),
                    )
                    .wrapping_add((((iHash - (1 as i32)) * (4096 as i32)) as i64) as u64)
                    as u32;
            }
        }
    } else {
        if rc == (0 as i32) {
            rc = 1 as i32;
        }
    }
    return rc;
}

/// Return the number of the wal-index page that contains the hash-table
/// and page-number array that contain entries corresponding to WAL frame
/// iFrame. The wal-index is broken up into 32KB pages. Wal-index pages
/// are numbered starting from 0.
fn walFramePage(mut iFrame: u32) -> i32 {
    let mut iHash: i32 = (((iFrame.wrapping_add((4096 as i32) as u32) as u64)
        .wrapping_sub(
            (((4096 as i32) as i64) as u64).wrapping_sub(
                (48 as u64)
                    .wrapping_mul(((2 as i32) as i64) as u64)
                    .wrapping_add(40 as u64)
                    / (4 as u64),
            ),
        )
        .wrapping_sub(((1 as i32) as i64) as u64)
        / (((4096 as i32) as i64) as u64)) as u32) as i32;
    0 as i32;
    0 as i32;
    return iHash;
}

/// Return the page number associated with frame iFrame in this WAL.
fn walFramePgno(mut pWal: *mut Wal, mut iFrame: u32) -> u32 {
    let mut iHash: i32 = walFramePage(iFrame);
    0 as i32;
    {}
    if iHash == (0 as i32) {
        return unsafe {
            std::ptr::read_volatile(unsafe {
                unsafe { *unsafe { unsafe { (*pWal).apWiData }.offset((0 as i32) as isize) } }
                    .offset(
                        ((48 as u64)
                            .wrapping_mul(((2 as i32) as i64) as u64)
                            .wrapping_add(40 as u64)
                            / (4 as u64))
                            .wrapping_add(iFrame as u64)
                            .wrapping_sub(((1 as i32) as i64) as u64)
                            as isize,
                    )
            })
        };
    }
    return unsafe {
        std::ptr::read_volatile(unsafe {
            unsafe { *unsafe { unsafe { (*pWal).apWiData }.offset(iHash as isize) } }.offset(
                ((iFrame.wrapping_sub((1 as i32) as u32) as u64).wrapping_sub(
                    (((4096 as i32) as i64) as u64).wrapping_sub(
                        (48 as u64)
                            .wrapping_mul(((2 as i32) as i64) as u64)
                            .wrapping_add(40 as u64)
                            / (4 as u64),
                    ),
                ) % (((4096 as i32) as i64) as u64)) as isize,
            )
        })
    };
}

/// Remove entries from the hash table that point to WAL slots greater
/// than pWal->hdr.mxFrame.
///
/// This function is called whenever pWal->hdr.mxFrame is decreased due
/// to a rollback or savepoint.
///
/// At most only the hash table containing pWal->hdr.mxFrame needs to be
/// updated.  Any later hash tables will be automatically cleared when
/// pWal->hdr.mxFrame advances to the point where those hash tables are
/// actually needed.
fn walCleanupHash(mut pWal: *mut Wal) {
    let mut sLoc: WalHashLoc = unsafe { std::mem::zeroed() }; // Hash table location
    let mut iLimit: i32 = 0 as i32; // Zero values greater than this
    let mut nByte: i32 = 0 as i32; // Number of bytes to zero in aPgno[]
    let mut i: i32 = 0 as i32; // Used to iterate through aHash[]
    0 as i32;
    {}
    {}
    {}
    if (unsafe { (*pWal).hdr.mxFrame }) == ((0 as i32) as u32) {
        return;
    }
    // Obtain pointers to the hash-table and page-number array containing
    // the entry that corresponds to frame pWal->hdr.mxFrame. It is guaranteed
    // that the page said hash-table and array reside on is already mapped.(1)
    0 as i32;
    0 as i32;
    i = walHashGet(
        pWal,
        walFramePage(unsafe { (*pWal).hdr.mxFrame }),
        std::ptr::addr_of_mut!(sLoc),
    );
    if i != (0 as i32) {
        return;
    }
    // Defense-in-depth, in case (1) above is wrong
    // Zero all hash-table entries that correspond to frame numbers greater
    // than pWal->hdr.mxFrame.
    iLimit = unsafe { (*pWal).hdr.mxFrame }.wrapping_sub(sLoc.iZero) as i32;
    0 as i32;
    i = 0 as i32;
    '__slate_break_960: loop {
        if !(i < (4096 as i32) * (2 as i32)) {
            break;
        }
        if (((unsafe { std::ptr::read_volatile(unsafe { sLoc.aHash.offset(i as isize) }) }) as u32)
            as i32)
            > iLimit
        {
            unsafe {
                std::ptr::write_volatile(
                    unsafe { sLoc.aHash.offset(i as isize) },
                    ((0 as i32) as i16) as u16,
                )
            };
        }
        let __v1084: i32 = i;
        let __v1085: i32 = __v1084 + (1 as i32);
        i = __v1085;
    }
    // Zero the entries in the aPgno array that correspond to frames with
    // frame numbers greater than pWal->hdr.mxFrame.
    nByte = ((unsafe {
        (sLoc.aHash as *mut i8)
            .offset_from(((unsafe { sLoc.aPgno.offset(iLimit as isize) }) as *mut i8) as *mut i8)
    }) as i64) as i32;
    0 as i32;
    unsafe {
        memset(
            (unsafe { sLoc.aPgno.offset(iLimit as isize) }) as *mut (),
            0 as i32,
            (nByte as i64) as u64,
        )
    };
}

/// Set an entry in the wal-index that will map database page number
/// pPage into WAL frame iFrame.
fn walIndexAppend(mut pWal: *mut Wal, mut iFrame: u32, mut iPage: u32) -> i32 {
    let mut rc: i32 = 0 as i32; // Return code
    let mut sLoc: WalHashLoc = unsafe { std::mem::zeroed() }; // Wal-index hash table location
    rc = walHashGet(pWal, walFramePage(iFrame), std::ptr::addr_of_mut!(sLoc));
    // Assuming the wal-index file was successfully mapped, populate the
    // page number array and hash table entry.
    if rc == (0 as i32) {
        let mut iKey: i32 = 0 as i32; // Hash table key
        let mut idx: i32 = 0 as i32; // Value to write to hash-table slot
        let mut nCollide: i32 = 0 as i32; // Number of hash collisions
        idx = iFrame.wrapping_sub(sLoc.iZero) as i32;
        0 as i32;
        // If this is the first entry to be added to this hash-table, zero the
        // entire hash table and aPgno[] array before proceeding.
        if idx == (1 as i32) {
            let mut nByte: i32 = ((unsafe {
                ((unsafe { sLoc.aHash.offset(((4096 as i32) * (2 as i32)) as isize) }) as *mut u8)
                    .offset_from((sLoc.aPgno as *mut u8) as *mut u8)
            }) as i64) as i32;
            0 as i32;
            unsafe { memset(sLoc.aPgno as *mut (), 0 as i32, (nByte as i64) as u64) };
        }
        // If the entry in aPgno[] is already set, then the previous writer
        // must have exited unexpectedly in the middle of a transaction (after
        // writing one or more dirty pages to the WAL to free up memory).
        // Remove the remnants of that writers uncommitted transaction from
        // the hash-table before writing any new entries.
        if (unsafe {
            std::ptr::read_volatile(unsafe { sLoc.aPgno.offset((idx - (1 as i32)) as isize) })
        }) != (0 as u32)
        {
            walCleanupHash(pWal);
            0 as i32;
        }
        // Write the aPgno[] array entry and the hash-table slot.
        nCollide = idx;
        iKey = walHash(iPage);
        '__slate_break_961: while (unsafe {
            std::ptr::read_volatile(unsafe { sLoc.aHash.offset(iKey as isize) })
        }) != (0 as u16)
        {
            let __v1086: i32 = nCollide;
            let __v1087: i32 = __v1086 - (1 as i32);
            nCollide = __v1087;
            if __v1086 == (0 as i32) {
                return unsafe { sqlite3CorruptError(1341 as i32) };
            }
            iKey = walNextHash(iKey);
        }
        unsafe {
            std::ptr::write_volatile(
                unsafe {
                    sLoc.aPgno
                        .offset((idx - (1 as i32) & (4096 as i32) - (1 as i32)) as isize)
                },
                iPage,
            )
        };
        unsafe {
            std::sync::atomic::AtomicU16::store_volatile(
                std::sync::atomic::AtomicU16::from_ptr_raw(
                    (unsafe { sLoc.aHash.offset(iKey as isize) }) as *mut u16,
                ),
                (idx as i16) as u16,
                std::sync::atomic::Ordering::Relaxed,
            )
        };
    }
    return rc;
}

/// Recover the wal-index by reading the write-ahead log file.
///
/// This routine first tries to establish an exclusive lock on the
/// wal-index to prevent other threads/processes from doing anything
/// with the WAL or wal-index while recovery is running.  The
/// WAL_RECOVER_LOCK is also held so that other threads will know
/// that this thread is running recovery.  If unable to establish
/// the necessary locks, this routine returns SQLITE_BUSY.
fn walIndexRecover(mut pWal: *mut Wal) -> i32 {
    let mut __slate_storage_1094: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1094: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1094) as *mut i32;
    let mut __slate_storage_1093: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1093: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1093) as *mut i32;
    let mut __slate_storage_519: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_519: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_519) as *mut i32;
    let mut __slate_storage_518: std::mem::MaybeUninit<*mut WalCkptInfo> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_518: *mut *mut WalCkptInfo =
        std::ptr::addr_of_mut!(__slate_storage_518) as *mut *mut WalCkptInfo;
    let mut __slate_storage_1090: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1090: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_1090) as *mut u32;
    let mut __slate_storage_1089: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1089: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_1089) as *mut u32;
    let mut __slate_storage_1092: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1092: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_1092) as *mut u32;
    let mut __slate_storage_1091: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1091: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_1091) as *mut u32; // dbsize field from frame header
    let mut __slate_storage_517: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_517: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_517) as *mut u32; // Database page number for frame
    let mut __slate_storage_516: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_516: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_516) as *mut u32;
    let mut __slate_storage_515: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_515: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_515) as *mut i64;
    let mut __slate_storage_514: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_514: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_514) as *mut u32;
    let mut __slate_storage_513: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_513: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_513) as *mut u32;
    let mut __slate_storage_512: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_512: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_512) as *mut u32;
    let mut __slate_storage_511: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_511: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_511) as *mut u32; // Index of last frame read
    let mut __slate_storage_510: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_510: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_510) as *mut u32;
    let mut __slate_storage_509: std::mem::MaybeUninit<*mut u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_509: *mut *mut u32 =
        std::ptr::addr_of_mut!(__slate_storage_509) as *mut *mut u32;
    let mut __slate_storage_1088: std::mem::MaybeUninit<bool> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1088: *mut bool = std::ptr::addr_of_mut!(__slate_storage_1088) as *mut bool; // Last frame in wal, based on nSize alone
    let mut __slate_storage_508: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_508: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_508) as *mut u32; // Current 32KB wal-index page
    let mut __slate_storage_507: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_507: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_507) as *mut u32; // True if this frame is valid
    let mut __slate_storage_506: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_506: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_506) as *mut i32; // Magic value read from WAL header
    let mut __slate_storage_505: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_505: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_505) as *mut u32; // Magic value read from WAL header
    let mut __slate_storage_504: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_504: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_504) as *mut u32; // Page size according to the log
    let mut __slate_storage_503: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_503: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_503) as *mut i32; // Pointer to data part of aFrame buffer
    let mut __slate_storage_502: std::mem::MaybeUninit<*mut u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_502: *mut *mut u8 =
        std::ptr::addr_of_mut!(__slate_storage_502) as *mut *mut u8; // Number of bytes in buffer aFrame[]
    let mut __slate_storage_501: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_501: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_501) as *mut i32; // Malloc'd buffer to load entire frame
    let mut __slate_storage_500: std::mem::MaybeUninit<*mut u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_500: *mut *mut u8 =
        std::ptr::addr_of_mut!(__slate_storage_500) as *mut *mut u8; // Heap copy of *-shm hash being populated
    let mut __slate_storage_499: std::mem::MaybeUninit<*mut u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_499: *mut *mut u32 =
        std::ptr::addr_of_mut!(__slate_storage_499) as *mut *mut u32; // Buffer to load WAL header into
    let mut __slate_storage_498: std::mem::MaybeUninit<__SlateAlign16<[u8; 32]>> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_498: *mut [u8; 32] =
        std::ptr::addr_of_mut!(__slate_storage_498) as *mut [u8; 32]; // Lock offset to lock for checkpoint
    let mut __slate_storage_497: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_497: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_497) as *mut i32;
    let mut __slate_storage_496: std::mem::MaybeUninit<[u32; 2]> = std::mem::MaybeUninit::uninit();
    let __slate_slot_496: *mut [u32; 2] =
        std::ptr::addr_of_mut!(__slate_storage_496) as *mut [u32; 2]; // Size of log file
    let mut __slate_storage_495: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_495: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_495) as *mut i64; // Return Code
    let mut __slate_storage_494: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_494: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_494) as *mut i32;
    unsafe {
        std::ptr::write(__slate_slot_496, [(0 as i32) as u32, (0 as i32) as u32]);
        // Obtain an exclusive lock on all byte in the locking range not already
        // locked by the caller. The caller is guaranteed to have locked the
        // WAL_WRITE_LOCK byte, and may have also locked the WAL_CKPT_LOCK byte.
        // If successful, the same bytes that are locked here are unlocked before
        // this function returns.
        0 as i32;
        0 as i32;
        0 as i32;
        0 as i32;
        *__slate_slot_497 = (1 as i32) + (((unsafe { (*pWal).ckptLock }) as u32) as i32);
        *__slate_slot_494 = walLockExclusive(
            pWal,
            *__slate_slot_497,
            (3 as i32) + (0 as i32) - *__slate_slot_497,
        );
        if *__slate_slot_494 != (0 as i32) {
            return *__slate_slot_494;
        } else {
            '__join_0: {
                {}
                unsafe {
                    memset(
                        (unsafe { std::ptr::addr_of_mut!((*pWal).hdr) }) as *mut (),
                        0 as i32,
                        48 as u64,
                    )
                };
                *__slate_slot_494 = unsafe {
                    sqlite3OsFileSize(
                        unsafe { (*pWal).pWalFd },
                        std::ptr::addr_of_mut!(*__slate_slot_495),
                    )
                };
                if *__slate_slot_494 != (0 as i32) {
                } else {
                    if *__slate_slot_495 > ((32 as i32) as i64) {
                        std::ptr::write(__slate_slot_499, std::ptr::null_mut::<u32>());
                        std::ptr::write(__slate_slot_500, std::ptr::null_mut::<u8>());
                        // Read in the WAL header.
                        *__slate_slot_494 = unsafe {
                            sqlite3OsRead(
                                unsafe { (*pWal).pWalFd },
                                ((*__slate_slot_498).as_mut_ptr() as *mut u8) as *mut (),
                                32 as i32,
                                (0 as i32) as i64,
                            )
                        };
                        if *__slate_slot_494 != (0 as i32) {
                            break '__join_0;
                        } else {
                            // If the database page size is not a power of two, or is greater than
                            // SQLITE_MAX_PAGE_SIZE, conclude that the WAL file contains no valid
                            // data. Similarly, if the 'magic' value is invalid, ignore the whole
                            // WAL file.
                            *__slate_slot_504 = unsafe {
                                sqlite3Get4byte(
                                    (unsafe {
                                        ((*__slate_slot_498).as_mut_ptr() as *mut u8)
                                            .offset((0 as i32) as isize)
                                    }) as *const u8,
                                )
                            };
                            *__slate_slot_503 = (unsafe {
                                sqlite3Get4byte(
                                    (unsafe {
                                        ((*__slate_slot_498).as_mut_ptr() as *mut u8)
                                            .offset((8 as i32) as isize)
                                    }) as *const u8,
                                )
                            }) as i32;
                            if *__slate_slot_504 & (4294967294 as u32)
                                != ((931071618 as i32) as u32)
                                || *__slate_slot_503 & *__slate_slot_503 - (1 as i32) != (0 as i32)
                                || *__slate_slot_503 > (65536 as i32)
                                || *__slate_slot_503 < (512 as i32)
                            {
                            } else {
                                unsafe {
                                    (*pWal).hdr.bigEndCksum =
                                        (*__slate_slot_504 & ((1 as i32) as u32)) as u8;
                                }
                                unsafe {
                                    (*pWal).szPage = *__slate_slot_503 as u32;
                                }
                                unsafe {
                                    (*pWal).nCkpt = unsafe {
                                        sqlite3Get4byte(
                                            (unsafe {
                                                ((*__slate_slot_498).as_mut_ptr() as *mut u8)
                                                    .offset((12 as i32) as isize)
                                            })
                                                as *const u8,
                                        )
                                    };
                                }
                                unsafe {
                                    memcpy(
                                        (unsafe { std::ptr::addr_of_mut!((*pWal).hdr.aSalt) })
                                            as *mut (),
                                        (unsafe {
                                            ((*__slate_slot_498).as_mut_ptr() as *mut u8)
                                                .offset((16 as i32) as isize)
                                        }) as *const (),
                                        ((8 as i32) as i64) as u64,
                                    )
                                };
                                // Verify that the WAL header checksum is correct
                                walChecksumBytes(
                                    ((((unsafe { (*pWal).hdr.bigEndCksum }) as u32) as i32)
                                        == (0 as i32)) as i32,
                                    (*__slate_slot_498).as_mut_ptr() as *mut u8,
                                    (32 as i32) - (2 as i32) * (4 as i32),
                                    std::ptr::null::<u32>(),
                                    unsafe { (*pWal).hdr.aFrameCksum.as_mut_ptr() as *mut u32 },
                                );
                                if (unsafe {
                                    *unsafe {
                                        unsafe { (*pWal).hdr.aFrameCksum.as_mut_ptr() as *mut u32 }
                                            .offset((0 as i32) as isize)
                                    }
                                }) != unsafe {
                                    sqlite3Get4byte(
                                        (unsafe {
                                            ((*__slate_slot_498).as_mut_ptr() as *mut u8)
                                                .offset((24 as i32) as isize)
                                        }) as *const u8,
                                    )
                                } {
                                    *__slate_slot_1088 = true as bool;
                                } else {
                                    *__slate_slot_1088 = (unsafe {
                                        *unsafe {
                                            unsafe {
                                                (*pWal).hdr.aFrameCksum.as_mut_ptr() as *mut u32
                                            }
                                            .offset((1 as i32) as isize)
                                        }
                                    }) != unsafe {
                                        sqlite3Get4byte(
                                            (unsafe {
                                                ((*__slate_slot_498).as_mut_ptr() as *mut u8)
                                                    .offset((28 as i32) as isize)
                                            })
                                                as *const u8,
                                        )
                                    };
                                }
                                if *__slate_slot_1088 {
                                } else {
                                    // Verify that the version number on the WAL format is one that
                                    // are able to understand
                                    *__slate_slot_505 = unsafe {
                                        sqlite3Get4byte(
                                            (unsafe {
                                                ((*__slate_slot_498).as_mut_ptr() as *mut u8)
                                                    .offset((4 as i32) as isize)
                                            })
                                                as *const u8,
                                        )
                                    };
                                    if *__slate_slot_505 != ((3007000 as i32) as u32) {
                                        *__slate_slot_494 =
                                            unsafe { sqlite3CantopenError(1473 as i32) };
                                    } else {
                                        // Malloc a buffer to read frames into.
                                        *__slate_slot_501 = *__slate_slot_503 + (24 as i32);
                                        *__slate_slot_500 = (unsafe {
                                            sqlite3_malloc64(
                                                ((*__slate_slot_501 as i64) as u64).wrapping_add(
                                                    (2 as u64)
                                                        .wrapping_mul(
                                                            (((4096 as i32) * (2 as i32)) as i64)
                                                                as u64,
                                                        )
                                                        .wrapping_add(
                                                            (((4096 as i32) as i64) as u64)
                                                                .wrapping_mul(4 as u64),
                                                        ),
                                                ),
                                            )
                                        })
                                            as *mut u8;
                                        {}
                                        if !(*__slate_slot_500 != std::ptr::null_mut::<u8>()) {
                                            *__slate_slot_494 = 7 as i32;
                                            break '__join_0;
                                        } else {
                                            *__slate_slot_502 = unsafe {
                                                (*__slate_slot_500).offset((24 as i32) as isize)
                                            };
                                            *__slate_slot_499 = (unsafe {
                                                (*__slate_slot_502)
                                                    .offset(*__slate_slot_503 as isize)
                                            })
                                                as *mut u32;
                                            // Read all frames from the log file.
                                            *__slate_slot_508 = (((*__slate_slot_495
                                                - ((32 as i32) as i64))
                                                / (*__slate_slot_501 as i64))
                                                as i32)
                                                as u32;
                                            *__slate_slot_507 = (0 as i32) as u32;
                                            loop {
                                                if *__slate_slot_507
                                                    <= (walFramePage(*__slate_slot_508) as u32)
                                                {
                                                    std::ptr::write(
                                                        __slate_slot_511,
                                                        (if (*__slate_slot_508 as u64)
                                                            < (((4096 as i32) as i64) as u64)
                                                                .wrapping_sub(
                                                                    (48 as u64)
                                                                        .wrapping_mul(
                                                                            ((2 as i32) as i64)
                                                                                as u64,
                                                                        )
                                                                        .wrapping_add(40 as u64)
                                                                        / (4 as u64),
                                                                )
                                                                .wrapping_add(
                                                                    (*__slate_slot_507)
                                                                        .wrapping_mul(
                                                                            (4096 as i32) as u32,
                                                                        )
                                                                        as u64,
                                                                )
                                                        {
                                                            *__slate_slot_508 as u64
                                                        } else {
                                                            (((4096 as i32) as i64) as u64)
                                                                .wrapping_sub(
                                                                    (48 as u64)
                                                                        .wrapping_mul(
                                                                            ((2 as i32) as i64)
                                                                                as u64,
                                                                        )
                                                                        .wrapping_add(40 as u64)
                                                                        / (4 as u64),
                                                                )
                                                                .wrapping_add(
                                                                    (*__slate_slot_507)
                                                                        .wrapping_mul(
                                                                            (4096 as i32) as u32,
                                                                        )
                                                                        as u64,
                                                                )
                                                        })
                                                            as u32,
                                                    );
                                                    std::ptr::write(
                                                        __slate_slot_512,
                                                        (((1 as i32) as i64) as u64).wrapping_add(
                                                            if *__slate_slot_507
                                                                == ((0 as i32) as u32)
                                                            {
                                                                ((0 as i32) as i64) as u64
                                                            } else {
                                                                (((4096 as i32) as i64) as u64)
                                                                    .wrapping_sub(
                                                                        (48 as u64)
                                                                            .wrapping_mul(
                                                                                ((2 as i32) as i64)
                                                                                    as u64,
                                                                            )
                                                                            .wrapping_add(
                                                                                40 as u64,
                                                                            )
                                                                            / (4 as u64),
                                                                    )
                                                                    .wrapping_add(
                                                                        (*__slate_slot_507)
                                                                            .wrapping_sub(
                                                                                (1 as i32) as u32,
                                                                            )
                                                                            .wrapping_mul(
                                                                                (4096 as i32)
                                                                                    as u32,
                                                                            )
                                                                            as u64,
                                                                    )
                                                            },
                                                        )
                                                            as u32,
                                                    );
                                                    *__slate_slot_494 = walIndexPage(
                                                        pWal,
                                                        *__slate_slot_507 as i32,
                                                        std::ptr::addr_of_mut!(*__slate_slot_509)
                                                            as *mut *mut u32,
                                                    );
                                                    0 as i32;
                                                    if *__slate_slot_509
                                                        == std::ptr::null_mut::<u32>()
                                                    {
                                                        break;
                                                    } else {
                                                        {}
                                                        unsafe {
                                                            *unsafe {
                                                                unsafe { (*pWal).apWiData }.offset(
                                                                    *__slate_slot_507 as isize,
                                                                )
                                                            } = *__slate_slot_499 as *mut u32;
                                                        }
                                                        *__slate_slot_510 = *__slate_slot_512;
                                                        loop {
                                                            if *__slate_slot_510
                                                                <= *__slate_slot_511
                                                            {
                                                                std::ptr::write(
                                                                    __slate_slot_515,
                                                                    ((32 as i32) as i64)
                                                                        + (((*__slate_slot_510)
                                                                            .wrapping_sub(
                                                                                (1 as i32) as u32,
                                                                            )
                                                                            as u64)
                                                                            as i64)
                                                                            * ((*__slate_slot_503
                                                                                + (24 as i32))
                                                                                as i64),
                                                                );
                                                                // Read and decode the next log frame.
                                                                *__slate_slot_494 = unsafe {
                                                                    sqlite3OsRead(
                                                                        unsafe { (*pWal).pWalFd },
                                                                        *__slate_slot_500
                                                                            as *mut (),
                                                                        *__slate_slot_501,
                                                                        *__slate_slot_515,
                                                                    )
                                                                };
                                                                if *__slate_slot_494 != (0 as i32) {
                                                                    break;
                                                                } else {
                                                                    *__slate_slot_506 =
                                                                        walDecodeFrame(
                                                                            pWal,
                                                                            std::ptr::addr_of_mut!(
                                                                                *__slate_slot_516
                                                                            ),
                                                                            std::ptr::addr_of_mut!(
                                                                                *__slate_slot_517
                                                                            ),
                                                                            *__slate_slot_502,
                                                                            *__slate_slot_500,
                                                                        );
                                                                    if !(*__slate_slot_506
                                                                        != (0 as i32))
                                                                    {
                                                                        break;
                                                                    } else {
                                                                        *__slate_slot_494 =
                                                                            walIndexAppend(
                                                                                pWal,
                                                                                *__slate_slot_510,
                                                                                *__slate_slot_516,
                                                                            );
                                                                        if *__slate_slot_494
                                                                            != (0 as i32)
                                                                        {
                                                                            break;
                                                                        } else {
                                                                            // If nTruncate is non-zero, this is a commit record.
                                                                            if *__slate_slot_517
                                                                                != (0 as u32)
                                                                            {
                                                                                unsafe {
                                                                                    (*pWal).hdr.mxFrame = *__slate_slot_510;
                                                                                }
                                                                                unsafe {
                                                                                    (*pWal).hdr.nPage = *__slate_slot_517;
                                                                                }
                                                                                unsafe {
                                                                                    (*pWal).hdr.szPage = ((*__slate_slot_503 & (65280 as i32) | *__slate_slot_503 >> (16 as i32)) as i16) as u16;
                                                                                }
                                                                                {}
                                                                                {}
                                                                                unsafe {
                                                                                    *unsafe {
                                                                                        ((*__slate_slot_496).as_mut_ptr() as *mut u32).offset((0 as i32) as isize)
                                                                                    } = unsafe {
                                                                                        *unsafe {
                                                                                            unsafe { (*pWal).hdr.aFrameCksum.as_mut_ptr() as *mut u32 }.offset((0 as i32) as isize)
                                                                                        }
                                                                                    };
                                                                                }
                                                                                unsafe {
                                                                                    *unsafe {
                                                                                        ((*__slate_slot_496).as_mut_ptr() as *mut u32).offset((1 as i32) as isize)
                                                                                    } = unsafe {
                                                                                        *unsafe {
                                                                                            unsafe { (*pWal).hdr.aFrameCksum.as_mut_ptr() as *mut u32 }.offset((1 as i32) as isize)
                                                                                        }
                                                                                    };
                                                                                }
                                                                            }
                                                                            std::ptr::write(
                                                                                __slate_slot_1091,
                                                                                *__slate_slot_510,
                                                                            );
                                                                            std::ptr::write(__slate_slot_1092, (*__slate_slot_1091).wrapping_add((1 as i32) as u32));
                                                                            *__slate_slot_510 =
                                                                                *__slate_slot_1092;
                                                                        }
                                                                    }
                                                                }
                                                            } else {
                                                                break;
                                                            }
                                                        }
                                                        unsafe {
                                                            *unsafe {
                                                                unsafe { (*pWal).apWiData }.offset(
                                                                    *__slate_slot_507 as isize,
                                                                )
                                                            } = *__slate_slot_509 as *mut u32;
                                                        }
                                                        {}
                                                        *__slate_slot_513 = (if *__slate_slot_507
                                                            == ((0 as i32) as u32)
                                                        {
                                                            (48 as u64)
                                                                .wrapping_mul(
                                                                    ((2 as i32) as i64) as u64,
                                                                )
                                                                .wrapping_add(40 as u64)
                                                        } else {
                                                            ((0 as i32) as i64) as u64
                                                        })
                                                            as u32;
                                                        *__slate_slot_514 = ((*__slate_slot_513
                                                            as u64)
                                                            / (4 as u64))
                                                            as u32;
                                                        // Memcpy() should work fine here, on all reasonable implementations.
                                                        // Technically, memcpy() might change the destination to some
                                                        // intermediate value before setting to the final value, and that might
                                                        // cause a concurrent reader to malfunction.  Memcpy() is allowed to
                                                        // do that, according to the spec, but no memcpy() implementation that
                                                        // we know of actually does that, which is why we say that memcpy()
                                                        // is safe for this.  Memcpy() is certainly a lot faster.
                                                        unsafe {
                                                            memcpy(
                                                                (unsafe {
                                                                    (*__slate_slot_509).offset(
                                                                        *__slate_slot_514 as isize,
                                                                    )
                                                                })
                                                                    as *mut (),
                                                                (unsafe {
                                                                    (*__slate_slot_499).offset(
                                                                        *__slate_slot_514 as isize,
                                                                    )
                                                                })
                                                                    as *const (),
                                                                (2 as u64)
                                                                    .wrapping_mul(
                                                                        (((4096 as i32)
                                                                            * (2 as i32))
                                                                            as i64)
                                                                            as u64,
                                                                    )
                                                                    .wrapping_add(
                                                                        (((4096 as i32) as i64)
                                                                            as u64)
                                                                            .wrapping_mul(4 as u64),
                                                                    )
                                                                    .wrapping_sub(
                                                                        *__slate_slot_513 as u64,
                                                                    ),
                                                            )
                                                        };
                                                        0 as i32;
                                                        {}
                                                        if *__slate_slot_510 <= *__slate_slot_511 {
                                                            break;
                                                        } else {
                                                            std::ptr::write(
                                                                __slate_slot_1089,
                                                                *__slate_slot_507,
                                                            );
                                                            std::ptr::write(
                                                                __slate_slot_1090,
                                                                (*__slate_slot_1089).wrapping_add(
                                                                    (1 as i32) as u32,
                                                                ),
                                                            );
                                                            *__slate_slot_507 = *__slate_slot_1090;
                                                        }
                                                    }
                                                } else {
                                                    break;
                                                }
                                            }
                                            {}
                                            unsafe { sqlite3_free(*__slate_slot_500 as *mut ()) };
                                        }
                                    }
                                }
                            }
                        }
                    }
                    if *__slate_slot_494 == (0 as i32) {
                        unsafe {
                            *unsafe {
                                unsafe { (*pWal).hdr.aFrameCksum.as_mut_ptr() as *mut u32 }
                                    .offset((0 as i32) as isize)
                            } = unsafe {
                                *unsafe {
                                    ((*__slate_slot_496).as_mut_ptr() as *mut u32)
                                        .offset((0 as i32) as isize)
                                }
                            };
                        }
                        unsafe {
                            *unsafe {
                                unsafe { (*pWal).hdr.aFrameCksum.as_mut_ptr() as *mut u32 }
                                    .offset((1 as i32) as isize)
                            } = unsafe {
                                *unsafe {
                                    ((*__slate_slot_496).as_mut_ptr() as *mut u32)
                                        .offset((1 as i32) as isize)
                                }
                            };
                        }
                        walIndexWriteHdr(pWal);
                        // Reset the checkpoint-header. This is safe because this thread is
                        // currently holding locks that exclude all other writers and
                        // checkpointers. Then set the values of read-mark slots 1 through N.
                        *__slate_slot_518 = walCkptInfo(pWal);
                        unsafe {
                            std::ptr::write_volatile(
                                std::ptr::addr_of_mut!((*(*__slate_slot_518)).nBackfill),
                                (0 as i32) as u32,
                            )
                        };
                        unsafe {
                            std::ptr::write_volatile(
                                std::ptr::addr_of_mut!((*(*__slate_slot_518)).nBackfillAttempted),
                                unsafe { (*pWal).hdr.mxFrame },
                            )
                        };
                        unsafe {
                            std::ptr::write_volatile(
                                unsafe {
                                    unsafe {
                                        (*(*__slate_slot_518)).aReadMark.as_mut_ptr() as *mut u32
                                    }
                                    .offset((0 as i32) as isize)
                                },
                                (0 as i32) as u32,
                            )
                        };
                        *__slate_slot_519 = 1 as i32;
                        loop {
                            if *__slate_slot_519 < (8 as i32) - (3 as i32) {
                                *__slate_slot_494 = walLockExclusive(
                                    pWal,
                                    (3 as i32) + *__slate_slot_519,
                                    1 as i32,
                                );
                                if *__slate_slot_494 == (0 as i32) {
                                    if *__slate_slot_519 == (1 as i32)
                                        && (unsafe { (*pWal).hdr.mxFrame }) != (0 as u32)
                                    {
                                        unsafe {
                                            std::ptr::write_volatile(
                                                unsafe {
                                                    unsafe {
                                                        (*(*__slate_slot_518))
                                                            .aReadMark
                                                            .as_mut_ptr()
                                                            as *mut u32
                                                    }
                                                    .offset(*__slate_slot_519 as isize)
                                                },
                                                unsafe { (*pWal).hdr.mxFrame },
                                            )
                                        };
                                    } else {
                                        unsafe {
                                            std::ptr::write_volatile(
                                                unsafe {
                                                    unsafe {
                                                        (*(*__slate_slot_518))
                                                            .aReadMark
                                                            .as_mut_ptr()
                                                            as *mut u32
                                                    }
                                                    .offset(*__slate_slot_519 as isize)
                                                },
                                                4294967295 as u32,
                                            )
                                        };
                                    }
                                    0 as i32;
                                    {}
                                    walUnlockExclusive(
                                        pWal,
                                        (3 as i32) + *__slate_slot_519,
                                        1 as i32,
                                    );
                                } else {
                                    if *__slate_slot_494 != (5 as i32) {
                                        break '__join_0;
                                    }
                                }
                                std::ptr::write(__slate_slot_1093, *__slate_slot_519);
                                std::ptr::write(__slate_slot_1094, *__slate_slot_1093 + (1 as i32));
                                *__slate_slot_519 = *__slate_slot_1094;
                            } else {
                                break;
                            }
                        }
                        // If more than one frame was recovered from the log file, report an
                        // event via sqlite3_log(). This is to help with identifying performance
                        // problems caused by applications routinely shutting down without
                        // checkpointing the log file.
                        if (unsafe { (*pWal).hdr.nPage }) != (0 as u32) {
                            unsafe {
                                sqlite3_log(
                                    (27 as i32) | (1 as i32) << (8 as i32),
                                    (b"recovered %d frames from WAL file %s\0".as_ptr() as *mut i8)
                                        as *const i8,
                                    unsafe { (*pWal).hdr.mxFrame },
                                    unsafe { (*pWal).zWalName },
                                )
                            };
                        }
                    }
                }
            }
            {}
            walUnlockExclusive(
                pWal,
                *__slate_slot_497,
                (3 as i32) + (0 as i32) - *__slate_slot_497,
            );
            return *__slate_slot_494;
        }
    }
    return unsafe { std::mem::zeroed() };
}

/// Close an open wal-index.
fn walIndexClose(mut pWal: *mut Wal, mut isDelete: i32) {
    if (((unsafe { (*pWal).exclusiveMode }) as u32) as i32) == (2 as i32)
        || (unsafe { (*pWal).bShmUnreliable }) != (0 as u8)
    {
        let mut i: i32 = 0 as i32;
        i = 0 as i32;
        '__slate_break_966: loop {
            if !(i < unsafe { (*pWal).nWiData }) {
                break;
            }
            unsafe {
                sqlite3_free(
                    (unsafe { *unsafe { unsafe { (*pWal).apWiData }.offset(i as isize) } })
                        as *mut (),
                )
            };
            unsafe {
                *unsafe { unsafe { (*pWal).apWiData }.offset(i as isize) } =
                    std::ptr::null_mut::<u32>();
            }
            let __v1095: i32 = i;
            let __v1096: i32 = __v1095 + (1 as i32);
            i = __v1096;
        }
    }
    if (((unsafe { (*pWal).exclusiveMode }) as u32) as i32) != (2 as i32) {
        unsafe { sqlite3OsShmUnmap(unsafe { (*pWal).pDbFd }, isDelete) };
    }
}

/// Open a connection to the WAL file zWalName. The database file must
/// already be opened on connection pDbFd. The buffer that zWalName points
/// to must remain valid for the lifetime of the returned Wal* handle.
///
/// A SHARED lock should be held on the database file when this function
/// is called. The purpose of this SHARED lock is to prevent any other
/// client from unlinking the WAL or wal-index file. If another process
/// were to do this just after this client opened one of these files, the
/// system would be badly broken.
///
/// If the log file is successfully opened, SQLITE_OK is returned and
/// *ppWal is set to point to a new WAL handle. If an error occurs,
/// an SQLite error code is returned and *ppWal is left unmodified.
///
/// # Arguments
///
/// * `pVfs` - vfs module to open wal and wal-index
/// * `pDbFd` - The open database file
/// * `zWalName` - Name of the WAL file
/// * `bNoShm` - True to run in heap-memory mode
/// * `mxWalSize` - Truncate WAL to this size on reset
/// * `ppWal` - OUT: Allocated Wal handle
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3WalOpen(
    mut pVfs: *mut sqlite3_vfs,
    mut pDbFd: *mut sqlite3_file,
    mut zWalName: *const i8,
    mut bNoShm: i32,
    mut mxWalSize: i64,
    mut ppWal: *mut *mut Wal,
) -> i32 {
    let mut rc: i32 = 0 as i32; // Return Code
    let mut pRet: *mut Wal = unsafe { std::mem::zeroed() }; // Object to allocate and return
    let mut flags: i32 = 0 as i32; // Flags passed to OsOpen()
    0 as i32;
    0 as i32;
    // Verify the values of various constants.  Any changes to the values
    // of these constants would result in an incompatible on-disk format
    // for the -shm file.  Any change that causes one of these asserts to
    // fail is a backward compatibility problem, even if the change otherwise
    // works.
    //
    // This table also serves as a helpful cross-reference when trying to
    // interpret hex dumps of the -shm file.
    0 as i32;
    0 as i32;
    0 as i32;
    0 as i32;
    0 as i32;
    0 as i32;
    0 as i32;
    0 as i32;
    0 as i32;
    0 as i32;
    0 as i32;
    0 as i32;
    0 as i32;
    0 as i32;
    0 as i32;
    0 as i32;
    0 as i32;
    0 as i32;
    0 as i32;
    0 as i32;
    0 as i32;
    // In the amalgamation, the os_unix.c and os_win.c source files come before
    // this source file.  Verify that the #defines of the locking byte offsets
    // in os_unix.c and os_win.c agree with the WALINDEX_LOCK_OFFSET value.
    // For that matter, if the lock offset ever changes from its initial design
    // value of 120, we need to know that so there is an assert() to check it.
    // Allocate an instance of struct Wal to return.
    unsafe {
        *ppWal = std::ptr::null_mut::<Wal>();
    }
    pRet = (unsafe {
        sqlite3MallocZero((144 as u64).wrapping_add(((unsafe { (*pVfs).szOsFile }) as i64) as u64))
    }) as *mut Wal;
    if !(pRet != std::ptr::null_mut::<Wal>()) {
        return 7 as i32;
    }
    unsafe {
        (*pRet).pVfs = pVfs;
    }
    unsafe {
        (*pRet).pWalFd = (unsafe { pRet.offset((1 as i32) as isize) }) as *mut sqlite3_file;
    }
    unsafe {
        (*pRet).pDbFd = pDbFd;
    }
    unsafe {
        (*pRet).readLock = -(1 as i32) as i16;
    }
    unsafe {
        (*pRet).mxWalSize = mxWalSize;
    }
    unsafe {
        (*pRet).zWalName = zWalName;
    }
    unsafe {
        (*pRet).syncHeader = ((1 as i32) as i8) as u8;
    }
    unsafe {
        (*pRet).padToSectorBoundary = ((1 as i32) as i8) as u8;
    }
    unsafe {
        (*pRet).exclusiveMode = ((if bNoShm != (0 as i32) {
            2 as i32
        } else {
            0 as i32
        }) as i8) as u8;
    }
    // Open file handle on the write-ahead log file.
    flags = (2 as i32) | (4 as i32) | (524288 as i32);
    rc = unsafe {
        sqlite3OsOpen(
            pVfs,
            zWalName,
            unsafe { (*pRet).pWalFd },
            flags,
            std::ptr::addr_of_mut!(flags),
        )
    };
    if rc == (0 as i32) && flags & (1 as i32) != (0 as i32) {
        unsafe {
            (*pRet).readOnly = ((1 as i32) as i8) as u8;
        }
    }
    if rc != (0 as i32) {
        walIndexClose(pRet, 0 as i32);
        unsafe { sqlite3OsClose(unsafe { (*pRet).pWalFd }) };
        unsafe { sqlite3_free(pRet as *mut ()) };
    } else {
        let mut iDC: i32 = unsafe { sqlite3OsDeviceCharacteristics(pDbFd) };
        if iDC & (1024 as i32) != (0 as i32) {
            unsafe {
                (*pRet).syncHeader = ((0 as i32) as i8) as u8;
            }
        }
        if iDC & (4096 as i32) != (0 as i32) {
            unsafe {
                (*pRet).padToSectorBoundary = ((0 as i32) as i8) as u8;
            }
        }
        unsafe {
            *ppWal = pRet;
        }
        {}
    }
    return rc;
}

/// Change the size to which the WAL file is truncated on each reset.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3WalLimit(mut pWal: *mut Wal, mut iLimit: i64) {
    if pWal != std::ptr::null_mut::<Wal>() {
        unsafe {
            (*pWal).mxWalSize = iLimit;
        }
    }
}

/// Find the smallest page number out of all pages held in the WAL that
/// has not been returned by any prior invocation of this method on the
/// same WalIterator object.   Write into *piFrame the frame index where
/// that page was last written into the WAL.  Write into *piPage the page
/// number.
///
/// Return 0 on success.  If there are no pages in the WAL with a page
/// number larger than *piPage, then return 1.
///
/// # Arguments
///
/// * `p` - Iterator
/// * `piPage` - OUT: The page number of the next page
/// * `piFrame` - OUT: Wal frame index of next page
fn walIteratorNext(mut p: *mut WalIterator, mut piPage: *mut u32, mut piFrame: *mut u32) -> i32 {
    let mut iMin: u32 = 0 as u32; // Result pgno must be greater than iMin
    let mut iRet: u32 = 4294967295 as u32; // 0xffffffff is never a valid page number
    let mut i: i32 = 0 as i32; // For looping through segments
    iMin = unsafe { (*p).iPrior };
    0 as i32;
    i = (unsafe { (*p).nSegment }) - (1 as i32);
    '__slate_break_967: loop {
        if !(i >= (0 as i32)) {
            break;
        }
        let mut pSegment: *mut WalSegment = unsafe {
            unsafe { std::ptr::addr_of_mut!((*p).aSegment) as *mut WalSegment }.offset(i as isize)
        };
        '__slate_break_968: while (unsafe { (*pSegment).iNext }) < unsafe { (*pSegment).nEntry } {
            let mut iPg: u32 = unsafe {
                *unsafe {
                    unsafe { (*pSegment).aPgno }.offset(
                        (((unsafe {
                            *unsafe {
                                unsafe { (*pSegment).aIndex }
                                    .offset((unsafe { (*pSegment).iNext }) as isize)
                            }
                        }) as u32) as i32) as isize,
                    )
                }
            };
            if iPg > iMin {
                if iPg < iRet {
                    iRet = iPg;
                    unsafe {
                        *piFrame = ((unsafe { (*pSegment).iZero })
                            + (((unsafe {
                                *unsafe {
                                    unsafe { (*pSegment).aIndex }
                                        .offset((unsafe { (*pSegment).iNext }) as isize)
                                }
                            }) as u32) as i32)) as u32;
                    }
                }
                break '__slate_break_968;
            }
            let __v1099: *mut WalSegment = pSegment;
            let __v1100: i32 = unsafe { (*__v1099).iNext };
            let __v1101: i32 = __v1100 + (1 as i32);
            unsafe {
                (*__v1099).iNext = __v1101;
            }
        }
        let __v1097: i32 = i;
        let __v1098: i32 = __v1097 - (1 as i32);
        i = __v1098;
    }
    let __v1102: u32 = iRet;
    unsafe {
        (*p).iPrior = __v1102;
    }
    unsafe {
        *piPage = __v1102;
    }
    return (iRet == (4294967295 as u32)) as i32;
}

/// This function merges two sorted lists into a single sorted list.
///
/// aLeft[] and aRight[] are arrays of indices.  The sort key is
/// aContent[aLeft[]] and aContent[aRight[]].  Upon entry, the following
/// is guaranteed for all J<K:
///
///        aContent[aLeft[J]] < aContent[aLeft[K]]
///        aContent[aRight[J]] < aContent[aRight[K]]
///
/// This routine overwrites aRight[] with a new (probably longer) sequence
/// of indices such that the aRight[] contains every index that appears in
/// either aLeft[] or the old aRight[] and such that the second condition
/// above is still met.
///
/// The aContent[aLeft[X]] values will be unique for all X.  And the
/// aContent[aRight[X]] values will be unique too.  But there might be
/// one or more combinations of X and Y such that
///
///      aLeft[X]!=aRight[Y]  &&  aContent[aLeft[X]] == aContent[aRight[Y]]
///
/// When that happens, omit the aLeft[X] and use the aRight[Y] index.
///
/// # Arguments
///
/// * `aContent` - Pages in wal - keys for the sort
/// * `aLeft` - IN: Left hand input list
/// * `nLeft` - IN: Elements in array *paLeft
/// * `paRight` - IN/OUT: Right hand input list
/// * `pnRight` - IN/OUT: Elements in *paRight
/// * `aTmp` - Temporary buffer
fn walMerge(
    mut aContent: *const u32,
    mut aLeft: *mut u16,
    mut nLeft: i32,
    mut paRight: *mut *mut u16,
    mut pnRight: *mut i32,
    mut aTmp: *mut u16,
) {
    let mut iLeft: i32 = 0 as i32; // Current index in aLeft
    let mut iRight: i32 = 0 as i32; // Current index in aRight
    let mut iOut: i32 = 0 as i32; // Current index in output buffer
    let mut nRight: i32 = unsafe { *pnRight };
    let mut aRight: *mut u16 = unsafe { *paRight };
    0 as i32;
    '__slate_break_969: while iRight < nRight || iLeft < nLeft {
        let mut logpage: u16 = 0 as u16;
        let mut dbpage: u32 = 0 as u32;
        if iLeft < nLeft
            && (iRight >= nRight
                || (unsafe {
                    *unsafe {
                        aContent.offset(
                            (((unsafe { *unsafe { aLeft.offset(iLeft as isize) } }) as u32) as i32)
                                as isize,
                        )
                    }
                }) < unsafe {
                    *unsafe {
                        aContent.offset(
                            (((unsafe { *unsafe { aRight.offset(iRight as isize) } }) as u32)
                                as i32) as isize,
                        )
                    }
                })
        {
            let __v1103: i32 = iLeft;
            let __v1104: i32 = __v1103 + (1 as i32);
            iLeft = __v1104;
            logpage = unsafe { *unsafe { aLeft.offset(__v1103 as isize) } };
        } else {
            let __v1105: i32 = iRight;
            let __v1106: i32 = __v1105 + (1 as i32);
            iRight = __v1106;
            logpage = unsafe { *unsafe { aRight.offset(__v1105 as isize) } };
        }
        dbpage = unsafe { *unsafe { aContent.offset(((logpage as u32) as i32) as isize) } };
        let __v1107: i32 = iOut;
        let __v1108: i32 = __v1107 + (1 as i32);
        iOut = __v1108;
        unsafe {
            *unsafe { aTmp.offset(__v1107 as isize) } = logpage;
        }
        if iLeft < nLeft
            && (unsafe {
                *unsafe {
                    aContent.offset(
                        (((unsafe { *unsafe { aLeft.offset(iLeft as isize) } }) as u32) as i32)
                            as isize,
                    )
                }
            }) == dbpage
        {
            let __v1109: i32 = iLeft;
            let __v1110: i32 = __v1109 + (1 as i32);
            iLeft = __v1110;
        }
        0 as i32;
        0 as i32;
    }
    unsafe {
        *paRight = aLeft;
    }
    unsafe {
        *pnRight = iOut;
    }
    unsafe {
        memcpy(
            aLeft as *mut (),
            aTmp as *const (),
            (2 as u64).wrapping_mul((iOut as i64) as u64),
        )
    };
}

/// Sort the elements in list aList using aContent[] as the sort key.
/// Remove elements with duplicate keys, preferring to keep the
/// larger aList[] values.
///
/// The aList[] entries are indices into aContent[].  The values in
/// aList[] are to be sorted so that for all J<K:
///
///      aContent[aList[J]] < aContent[aList[K]]
///
/// For any X and Y such that
///
///      aContent[aList[X]] == aContent[aList[Y]]
///
/// Keep the larger of the two values aList[X] and aList[Y] and discard
/// the smaller.
///
/// # Arguments
///
/// * `aContent` - Pages in wal
/// * `aBuffer` - Buffer of at least *pnList items to use
/// * `aList` - IN/OUT: List to sort
/// * `pnList` - IN/OUT: Number of elements in aList[]
fn walMergesort(
    mut aContent: *const u32,
    mut aBuffer: *mut u16,
    mut aList: *mut u16,
    mut pnList: *mut i32,
) {
    let mut nList: i32 = unsafe { *pnList }; // Size of input list
    let mut nMerge: i32 = 0 as i32; // Number of elements in list aMerge
    let mut aMerge: *mut u16 = std::ptr::null_mut::<u16>(); // List to be merged
    let mut iList: i32 = 0 as i32; // Index into input list
    let mut iSub: u32 = (0 as i32) as u32; // Index into aSub array
    let mut aSub: __SlateAlign16<[Sublist; 13]> = __SlateAlign16(unsafe { std::mem::zeroed() }); // Array of sub-lists
    unsafe {
        memset(
            (aSub.0.as_mut_ptr() as *mut Sublist) as *mut (),
            0 as i32,
            208 as u64,
        )
    };
    0 as i32;
    0 as i32;
    iList = 0 as i32;
    '__slate_break_970: loop {
        if !(iList < nList) {
            break;
        }
        nMerge = 1 as i32;
        aMerge = unsafe { aList.offset(iList as isize) };
        iSub = (0 as i32) as u32;
        '__slate_break_971: while iList & (1 as i32) << iSub != (0 as i32) {
            let mut p: *mut Sublist = unsafe { std::mem::zeroed() };
            0 as i32;
            p = unsafe { (aSub.0.as_mut_ptr() as *mut Sublist).offset(iSub as isize) };
            0 as i32;
            0 as i32;
            walMerge(
                aContent,
                unsafe { (*p).aList },
                unsafe { (*p).nList },
                std::ptr::addr_of_mut!(aMerge),
                std::ptr::addr_of_mut!(nMerge),
                aBuffer,
            );
            let __v1113: u32 = iSub;
            let __v1114: u32 = __v1113.wrapping_add((1 as i32) as u32);
            iSub = __v1114;
        }
        unsafe {
            (*unsafe { (aSub.0.as_mut_ptr() as *mut Sublist).offset(iSub as isize) }).aList =
                aMerge;
        }
        unsafe {
            (*unsafe { (aSub.0.as_mut_ptr() as *mut Sublist).offset(iSub as isize) }).nList =
                nMerge;
        }
        let __v1111: i32 = iList;
        let __v1112: i32 = __v1111 + (1 as i32);
        iList = __v1112;
    }
    let __v1115: u32 = iSub;
    let __v1116: u32 = __v1115.wrapping_add((1 as i32) as u32);
    iSub = __v1116;
    '__slate_break_972: while iSub < (((((208 as u64) / (16 as u64)) as u32) as i32) as u32) {
        if nList & (1 as i32) << iSub != (0 as i32) {
            let mut p: *mut Sublist = unsafe { std::mem::zeroed() };
            0 as i32;
            p = unsafe { (aSub.0.as_mut_ptr() as *mut Sublist).offset(iSub as isize) };
            0 as i32;
            0 as i32;
            walMerge(
                aContent,
                unsafe { (*p).aList },
                unsafe { (*p).nList },
                std::ptr::addr_of_mut!(aMerge),
                std::ptr::addr_of_mut!(nMerge),
                aBuffer,
            );
        }
        let __v1117: u32 = iSub;
        let __v1118: u32 = __v1117.wrapping_add((1 as i32) as u32);
        iSub = __v1118;
    }
    0 as i32;
    unsafe {
        *pnList = nMerge;
    }
}

/// Free an iterator allocated by walIteratorInit().
fn walIteratorFree(mut p: *mut WalIterator) {
    unsafe { sqlite3_free(p as *mut ()) };
}

/// Construct a WalInterator object that can be used to loop over all
/// pages in the WAL following frame nBackfill in ascending order. Frames
/// nBackfill or earlier may be included - excluding them is an optimization
/// only. The caller must hold the checkpoint lock.
///
/// On success, make *pp point to the newly allocated WalInterator object
/// return SQLITE_OK. Otherwise, return an error code. If this routine
/// returns an error, the value of *pp is undefined.
///
/// The calling routine should invoke walIteratorFree() to destroy the
/// WalIterator object when it has finished with it.
fn walIteratorInit(mut pWal: *mut Wal, mut nBackfill: u32, mut pp: *mut *mut WalIterator) -> i32 {
    let mut p: *mut WalIterator = unsafe { std::mem::zeroed() }; // Return value
    let mut nSegment: i32 = 0 as i32; // Number of segments to merge
    let mut iLast: u32 = 0 as u32; // Last frame in log
    let mut nByte: i64 = 0 as i64; // Number of bytes to allocate
    let mut i: i32 = 0 as i32; // Iterator variable
    let mut aTmp: *mut u16 = unsafe { std::mem::zeroed() }; // Temp space used by merge-sort
    let mut rc: i32 = 0 as i32; // Return Code
    // This routine only runs while holding the checkpoint lock. And
    // it only runs if there is actually content in the log (mxFrame>0).
    0 as i32;
    iLast = unsafe { (*pWal).hdr.mxFrame };
    // Allocate space for the WalIterator object.
    nSegment = walFramePage(iLast) + (1 as i32);
    nByte = (8 as u64)
        .wrapping_add(((nSegment as i64) as u64).wrapping_mul(32 as u64))
        .wrapping_add((iLast as u64).wrapping_mul(2 as u64)) as i64;
    p = (unsafe {
        sqlite3_malloc64((nByte as u64).wrapping_add((2 as u64).wrapping_mul(
            (if iLast > ((4096 as i32) as u32) {
                (4096 as i32) as u32
            } else {
                iLast
            }) as u64,
        )))
    }) as *mut WalIterator;
    if !(p != std::ptr::null_mut::<WalIterator>()) {
        return 7 as i32;
    }
    unsafe { memset(p as *mut (), 0 as i32, nByte as u64) };
    unsafe {
        (*p).nSegment = nSegment;
    }
    aTmp = (unsafe { (p as *mut u8).offset(nByte as isize) }) as *mut u16;
    {}
    i = walFramePage(nBackfill.wrapping_add((1 as i32) as u32));
    '__slate_break_973: loop {
        if !(rc == (0 as i32) && i < nSegment) {
            break;
        }
        let mut sLoc: WalHashLoc = unsafe { std::mem::zeroed() };
        rc = walHashGet(pWal, i, std::ptr::addr_of_mut!(sLoc));
        if rc == (0 as i32) {
            let mut j: i32 = 0 as i32; // Counter variable
            let mut nEntry: i32 = 0 as i32; // Number of entries in this segment
            let mut aIndex: *mut u16 = unsafe { std::mem::zeroed() }; // Sorted index for this segment
            if i + (1 as i32) == nSegment {
                nEntry = iLast.wrapping_sub(sLoc.iZero) as i32;
            } else {
                nEntry = ((unsafe {
                    (sLoc.aHash as *mut u32).offset_from((sLoc.aPgno as *mut u32) as *mut u32)
                }) as i64) as i32;
            }
            aIndex = unsafe {
                ((unsafe {
                    unsafe { std::ptr::addr_of_mut!((*p).aSegment) as *mut WalSegment }
                        .offset((unsafe { (*p).nSegment }) as isize)
                }) as *mut u16)
                    .offset(sLoc.iZero as isize)
            };
            let __v1121: u32 = sLoc.iZero;
            let __v1122: u32 = __v1121.wrapping_add((1 as i32) as u32);
            sLoc.iZero = __v1122;
            j = 0 as i32;
            '__slate_break_974: loop {
                if !(j < nEntry) {
                    break;
                }
                unsafe {
                    *unsafe { aIndex.offset(j as isize) } = (j as i16) as u16;
                }
                let __v1123: i32 = j;
                let __v1124: i32 = __v1123 + (1 as i32);
                j = __v1124;
            }
            walMergesort(
                (sLoc.aPgno as *mut u32) as *const u32,
                aTmp,
                aIndex,
                std::ptr::addr_of_mut!(nEntry),
            );
            unsafe {
                (*unsafe {
                    unsafe { std::ptr::addr_of_mut!((*p).aSegment) as *mut WalSegment }
                        .offset(i as isize)
                })
                .iZero = sLoc.iZero as i32;
            }
            unsafe {
                (*unsafe {
                    unsafe { std::ptr::addr_of_mut!((*p).aSegment) as *mut WalSegment }
                        .offset(i as isize)
                })
                .nEntry = nEntry;
            }
            unsafe {
                (*unsafe {
                    unsafe { std::ptr::addr_of_mut!((*p).aSegment) as *mut WalSegment }
                        .offset(i as isize)
                })
                .aIndex = aIndex;
            }
            unsafe {
                (*unsafe {
                    unsafe { std::ptr::addr_of_mut!((*p).aSegment) as *mut WalSegment }
                        .offset(i as isize)
                })
                .aPgno = sLoc.aPgno as *mut u32;
            }
        }
        let __v1119: i32 = i;
        let __v1120: i32 = __v1119 + (1 as i32);
        i = __v1120;
    }
    if rc != (0 as i32) {
        {}
        walIteratorFree(p);
        p = std::ptr::null_mut::<WalIterator>();
    }
    unsafe {
        *pp = p;
    }
    return rc;
}

/// Attempt to obtain the exclusive WAL lock defined by parameters lockIdx and
/// n. If the attempt fails and parameter xBusy is not NULL, then it is a
/// busy-handler function. Invoke it and retry the lock until either the
/// lock is successfully obtained or the busy-handler returns 0.
///
/// # Arguments
///
/// * `pWal` - WAL connection
/// * `xBusy` - Function to call when busy
/// * `pBusyArg` - Context argument for xBusyHandler
/// * `lockIdx` - Offset of first byte to lock
/// * `n` - Number of bytes to lock
fn walBusyLock(
    mut pWal: *mut Wal,
    mut xBusy: Option<unsafe extern "C-unwind" fn(*mut ()) -> i32>,
    mut pBusyArg: *mut (),
    mut lockIdx: i32,
    mut n: i32,
) -> i32 {
    let mut rc: i32 = 0 as i32;
    '__slate_break_975: loop {
        rc = walLockExclusive(pWal, lockIdx, n);
        let __v1125: bool;
        if xBusy != None && rc == (5 as i32) {
            __v1125 = (unsafe { xBusy.unwrap()(pBusyArg) }) != (0 as i32);
        } else {
            __v1125 = false as bool;
        }
        if !__v1125 {
            break;
        }
    }
    return rc;
}

/// The cache of the wal-index header must be valid to call this function.
/// Return the page-size in bytes used by the database.
fn walPagesize(mut pWal: *mut Wal) -> i32 {
    return ((((unsafe { (*pWal).hdr.szPage }) as u32) as i32) & (65024 as i32))
        + (((((unsafe { (*pWal).hdr.szPage }) as u32) as i32) & (1 as i32)) << (16 as i32));
}

/// The following is guaranteed when this function is called:
///
///   a) the WRITER lock is held,
///   b) the entire log file has been checkpointed, and
///   c) any existing readers are reading exclusively from the database
///      file - there are no readers that may attempt to read a frame from
///      the log file.
///
/// This function updates the shared-memory structures so that the next
/// client to write to the database (which may be this one) does so by
/// writing frames into the start of the log file.
///
/// The value of parameter salt1 is used as the aSalt[1] value in the
/// new wal-index header. It should be passed a pseudo-random value (i.e.
/// one obtained from sqlite3_randomness()).
fn walRestartHdr(mut pWal: *mut Wal, mut salt1: u32) {
    let mut pInfo: *mut WalCkptInfo = walCkptInfo(pWal);
    let mut i: i32 = 0 as i32; // Loop counter
    let mut aSalt: *mut u32 = unsafe { (*pWal).hdr.aSalt.as_mut_ptr() as *mut u32 }; // Big-endian salt values
    let __v1126: *mut Wal = pWal;
    let __v1127: u32 = unsafe { (*__v1126).nCkpt };
    let __v1128: u32 = __v1127.wrapping_add((1 as i32) as u32);
    unsafe {
        (*__v1126).nCkpt = __v1128;
    }
    unsafe {
        (*pWal).hdr.mxFrame = (0 as i32) as u32;
    }
    unsafe {
        sqlite3Put4byte(
            (unsafe { aSalt.offset((0 as i32) as isize) }) as *mut u8,
            ((1 as i32) as u32).wrapping_add(unsafe {
                sqlite3Get4byte(
                    ((unsafe { aSalt.offset((0 as i32) as isize) }) as *mut u8) as *const u8,
                )
            }),
        )
    };
    unsafe {
        memcpy(
            (unsafe {
                unsafe { (*pWal).hdr.aSalt.as_mut_ptr() as *mut u32 }.offset((1 as i32) as isize)
            }) as *mut (),
            std::ptr::addr_of_mut!(salt1) as *const (),
            ((4 as i32) as i64) as u64,
        )
    };
    walIndexWriteHdr(pWal);
    unsafe {
        std::sync::atomic::AtomicU32::store_volatile(
            std::sync::atomic::AtomicU32::from_ptr_raw(
                (unsafe { std::ptr::addr_of_mut!((*pInfo).nBackfill) }) as *mut u32,
            ),
            (0 as i32) as u32,
            std::sync::atomic::Ordering::Relaxed,
        )
    };
    unsafe {
        std::ptr::write_volatile(
            std::ptr::addr_of_mut!((*pInfo).nBackfillAttempted),
            (0 as i32) as u32,
        )
    };
    unsafe {
        std::ptr::write_volatile(
            unsafe {
                unsafe { (*pInfo).aReadMark.as_mut_ptr() as *mut u32 }.offset((1 as i32) as isize)
            },
            (0 as i32) as u32,
        )
    };
    i = 2 as i32;
    '__slate_break_976: loop {
        if !(i < (8 as i32) - (3 as i32)) {
            break;
        }
        unsafe {
            std::ptr::write_volatile(
                unsafe {
                    unsafe { (*pInfo).aReadMark.as_mut_ptr() as *mut u32 }.offset(i as isize)
                },
                4294967295 as u32,
            )
        };
        let __v1129: i32 = i;
        let __v1130: i32 = __v1129 + (1 as i32);
        i = __v1130;
    }
    0 as i32;
}

/// Copy as much content as we can from the WAL back into the database file
/// in response to an sqlite3_wal_checkpoint() request or the equivalent.
///
/// The amount of information copies from WAL to database might be limited
/// by active readers.  This routine will never overwrite a database page
/// that a concurrent reader might be using.
///
/// All I/O barrier operations (a.k.a fsyncs) occur in this routine when
/// SQLite is in WAL-mode in synchronous=NORMAL.  That means that if
/// checkpoints are always run by a background thread or background
/// process, foreground threads will never block on a lengthy fsync call.
///
/// Fsync is called on the WAL before writing content out of the WAL and
/// into the database.  This ensures that if the new content is persistent
/// in the WAL and can be recovered following a power-loss or hard reset.
///
/// Fsync is also called on the database file if (and only if) the entire
/// WAL content is copied into the database file.  This second fsync makes
/// it safe to delete the WAL since the new content will persist in the
/// database file.
///
/// This routine uses and updates the nBackfill field of the wal-index header.
/// This is the only routine that will increase the value of nBackfill.
/// (A WAL reset or recovery will revert nBackfill to zero, but not increase
/// its value.)
///
/// The caller must be holding sufficient locks to ensure that no other
/// checkpoint is running (in any other thread or process) at the same
/// time.
///
/// # Arguments
///
/// * `pWal` - Wal connection
/// * `db` - Check for interrupts on this handle
/// * `eMode` - One of PASSIVE, FULL or RESTART
/// * `xBusy` - Function to call when busy
/// * `pBusyArg` - Context argument for xBusyHandler
/// * `sync_flags` - Flags for OsSync() (or 0)
/// * `zBuf` - Temporary buffer to use
fn walCheckpoint(
    mut pWal: *mut Wal,
    mut db: *mut sqlite3,
    mut eMode: i32,
    mut xBusy: Option<unsafe extern "C-unwind" fn(*mut ()) -> i32>,
    mut pBusyArg: *mut (),
    mut sync_flags: i32,
    mut zBuf: *mut u8,
) -> i32 {
    let mut __slate_storage_632: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_632: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_632) as *mut u32;
    let mut __slate_storage_631: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_631: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_631) as *mut i64;
    let mut __slate_storage_1135: std::mem::MaybeUninit<bool> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1135: *mut bool = std::ptr::addr_of_mut!(__slate_storage_1135) as *mut bool;
    let mut __slate_storage_630: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_630: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_630) as *mut i64; // Current size of database file
    let mut __slate_storage_629: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_629: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_629) as *mut i64;
    let mut __slate_storage_628: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_628: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_628) as *mut i64;
    // Now that read-lock slot 0 is locked, check that the wal has not been
    // wrapped since the header was read for this checkpoint. If it was, then
    // there was no work to do anyway.  In this case the
    // (pInfo->nBackfill<pWal->hdr.mxFrame) test above only passed because
    // pInfo->nBackfill had already been set to 0 by the writer that wrapped
    // the wal file. It would also be dangerous to proceed, as there may be
    // fewer than pWal->hdr.mxFrame valid frames in the wal file.
    let mut __slate_storage_627: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_627: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_627) as *mut i32;
    let mut __slate_storage_626: std::mem::MaybeUninit<*mut WalIndexHdr> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_626: *mut *mut WalIndexHdr =
        std::ptr::addr_of_mut!(__slate_storage_626) as *mut *mut WalIndexHdr;
    let mut __slate_storage_625: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_625: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_625) as *mut u32;
    let mut __slate_storage_1134: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1134: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1134) as *mut i32;
    let mut __slate_storage_1133: std::mem::MaybeUninit<bool> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1133: *mut bool = std::ptr::addr_of_mut!(__slate_storage_1133) as *mut bool;
    let mut __slate_storage_1132: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1132: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1132) as *mut i32;
    let mut __slate_storage_1131: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1131: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1131) as *mut i32;
    let mut __slate_storage_624: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_624: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_624) as *mut u32;
    let mut __slate_storage_623: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_623: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_623) as *mut u32; // The checkpoint status information
    let mut __slate_storage_622: std::mem::MaybeUninit<*mut WalCkptInfo> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_622: *mut *mut WalCkptInfo =
        std::ptr::addr_of_mut!(__slate_storage_622) as *mut *mut WalCkptInfo; // Loop counter
    let mut __slate_storage_621: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_621: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_621) as *mut i32; // Max database page to write
    let mut __slate_storage_620: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_620: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_620) as *mut u32; // Max frame that can be backfilled
    let mut __slate_storage_619: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_619: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_619) as *mut u32; // Wal frame containing data for iDbpage
    let mut __slate_storage_618: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_618: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_618) as *mut u32; // Next database page to write
    let mut __slate_storage_617: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_617: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_617) as *mut u32; // Wal iterator context
    let mut __slate_storage_616: std::mem::MaybeUninit<*mut WalIterator> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_616: *mut *mut WalIterator =
        std::ptr::addr_of_mut!(__slate_storage_616) as *mut *mut WalIterator; // Database page-size
    let mut __slate_storage_615: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_615: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_615) as *mut i32; // Return code
    let mut __slate_storage_614: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_614: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_614) as *mut i32;
    unsafe {
        '__join_0: {
            std::ptr::write(__slate_slot_614, 0 as i32);
            std::ptr::write(__slate_slot_616, std::ptr::null_mut::<WalIterator>());
            std::ptr::write(__slate_slot_617, (0 as i32) as u32);
            std::ptr::write(__slate_slot_618, (0 as i32) as u32);
            *__slate_slot_615 = walPagesize(pWal);
            {}
            {}
            *__slate_slot_622 = walCkptInfo(pWal);
            if (unsafe {
                std::ptr::read_volatile(std::ptr::addr_of!((*(*__slate_slot_622)).nBackfill))
            }) < unsafe { (*pWal).hdr.mxFrame }
            {
                // EVIDENCE-OF: R-62920-47450 The busy-handler callback is never invoked
                // in the SQLITE_CHECKPOINT_PASSIVE mode.
                0 as i32;
                // Compute in mxSafeFrame the index of the last frame of the WAL that is
                // safe to write into the database.  Frames beyond mxSafeFrame might
                // overwrite database pages that are in use by active readers and thus
                // cannot be backfilled from the WAL.
                *__slate_slot_619 = unsafe { (*pWal).hdr.mxFrame };
                *__slate_slot_620 = unsafe { (*pWal).hdr.nPage };
                *__slate_slot_621 = 1 as i32;
                loop {
                    if *__slate_slot_621 < (8 as i32) - (3 as i32) {
                        std::ptr::write(__slate_slot_623, unsafe {
                            std::sync::atomic::AtomicU32::load_volatile(
                                std::sync::atomic::AtomicU32::from_ptr_raw(
                                    (unsafe {
                                        unsafe {
                                            (*(*__slate_slot_622)).aReadMark.as_mut_ptr()
                                                as *mut u32
                                        }
                                        .offset(*__slate_slot_621 as isize)
                                    }) as *mut u32,
                                ),
                                std::sync::atomic::Ordering::Relaxed,
                            )
                        });
                        0 as i32;
                        {}
                        if *__slate_slot_619 > *__slate_slot_623 {
                            0 as i32;
                            *__slate_slot_614 = walBusyLock(
                                pWal,
                                xBusy,
                                pBusyArg,
                                (3 as i32) + *__slate_slot_621,
                                1 as i32,
                            );
                            if *__slate_slot_614 == (0 as i32) {
                                std::ptr::write(
                                    __slate_slot_624,
                                    if *__slate_slot_621 == (1 as i32) {
                                        *__slate_slot_619
                                    } else {
                                        4294967295 as u32
                                    },
                                );
                                unsafe {
                                    std::sync::atomic::AtomicU32::store_volatile(
                                        std::sync::atomic::AtomicU32::from_ptr_raw(
                                            (unsafe {
                                                unsafe {
                                                    (*(*__slate_slot_622)).aReadMark.as_mut_ptr()
                                                        as *mut u32
                                                }
                                                .offset(*__slate_slot_621 as isize)
                                            })
                                                as *mut u32,
                                        ),
                                        *__slate_slot_624,
                                        std::sync::atomic::Ordering::Relaxed,
                                    )
                                };
                                0 as i32;
                                {}
                                walUnlockExclusive(pWal, (3 as i32) + *__slate_slot_621, 1 as i32);
                            } else {
                                if *__slate_slot_614 == (5 as i32) {
                                    *__slate_slot_619 = *__slate_slot_623;
                                    xBusy = None;
                                } else {
                                    break '__join_0;
                                }
                            }
                        }
                        std::ptr::write(__slate_slot_1131, *__slate_slot_621);
                        std::ptr::write(__slate_slot_1132, *__slate_slot_1131 + (1 as i32));
                        *__slate_slot_621 = *__slate_slot_1132;
                    } else {
                        break;
                    }
                }
                // Allocate the iterator
                if (unsafe {
                    std::ptr::read_volatile(std::ptr::addr_of!((*(*__slate_slot_622)).nBackfill))
                }) < *__slate_slot_619
                {
                    *__slate_slot_614 = walIteratorInit(
                        pWal,
                        unsafe {
                            std::ptr::read_volatile(std::ptr::addr_of!(
                                (*(*__slate_slot_622)).nBackfill
                            ))
                        },
                        std::ptr::addr_of_mut!(*__slate_slot_616),
                    );
                    0 as i32;
                }
                if *__slate_slot_616 != std::ptr::null_mut::<WalIterator>() {
                    std::ptr::write(
                        __slate_slot_1134,
                        walBusyLock(pWal, xBusy, pBusyArg, (3 as i32) + (0 as i32), 1 as i32),
                    );
                    *__slate_slot_614 = *__slate_slot_1134;
                    *__slate_slot_1133 = *__slate_slot_1134 == (0 as i32);
                } else {
                    *__slate_slot_1133 = false as bool;
                }
                if *__slate_slot_1133 {
                    std::ptr::write(__slate_slot_625, unsafe {
                        std::ptr::read_volatile(std::ptr::addr_of!(
                            (*(*__slate_slot_622)).nBackfill
                        ))
                    });
                    std::ptr::write(__slate_slot_626, walIndexHdr(pWal) as *mut WalIndexHdr);
                    std::ptr::write(__slate_slot_627, unsafe {
                        memcmp(
                            (unsafe { (*(*__slate_slot_626)).aSalt.as_mut_ptr() as *mut u32 })
                                as *const (),
                            (unsafe { (*pWal).hdr.aSalt.as_mut_ptr() as *mut u32 }) as *const (),
                            8 as u64,
                        )
                    });
                    if (0 as i32) == *__slate_slot_627 {
                        unsafe {
                            std::ptr::write_volatile(
                                std::ptr::addr_of_mut!((*(*__slate_slot_622)).nBackfillAttempted),
                                *__slate_slot_619,
                            )
                        };
                        0 as i32;
                        {}
                        // Sync the WAL to disk
                        *__slate_slot_614 = unsafe {
                            sqlite3OsSync(
                                unsafe { (*pWal).pWalFd },
                                sync_flags >> (2 as i32) & (3 as i32),
                            )
                        };
                        // If the database may grow as a result of this checkpoint, hint
                        // about the eventual size of the db file to the VFS layer.
                        if *__slate_slot_614 == (0 as i32) {
                            std::ptr::write(
                                __slate_slot_628,
                                ((*__slate_slot_620 as u64) as i64) * (*__slate_slot_615 as i64),
                            );
                            unsafe {
                                sqlite3OsFileControl(
                                    unsafe { (*pWal).pDbFd },
                                    39 as i32,
                                    std::ptr::null_mut::<()>(),
                                )
                            };
                            *__slate_slot_614 = unsafe {
                                sqlite3OsFileSize(
                                    unsafe { (*pWal).pDbFd },
                                    std::ptr::addr_of_mut!(*__slate_slot_629),
                                )
                            };
                            if *__slate_slot_614 == (0 as i32)
                                && *__slate_slot_629 < *__slate_slot_628
                            {
                                if *__slate_slot_629
                                    + ((65536 as i32) as i64)
                                    + (((unsafe { (*pWal).hdr.mxFrame }) as u64) as i64)
                                        * (*__slate_slot_615 as i64)
                                    < *__slate_slot_628
                                {
                                    // If the size of the final database is larger than the current
                                    // database plus the amount of data in the wal file, plus the
                                    // maximum size of the pending-byte page (65536 bytes), then
                                    // must be corruption somewhere.
                                    *__slate_slot_614 = unsafe { sqlite3CorruptError(2293 as i32) };
                                } else {
                                    unsafe {
                                        sqlite3OsFileControlHint(
                                            unsafe { (*pWal).pDbFd },
                                            5 as i32,
                                            std::ptr::addr_of_mut!(*__slate_slot_628) as *mut (),
                                        )
                                    };
                                }
                            }
                        }
                        // Iterate through the contents of the WAL, copying data to the
                        // db file
                        '__join_17: {
                            loop {
                                if *__slate_slot_614 == (0 as i32) {
                                    *__slate_slot_1135 = (0 as i32)
                                        == walIteratorNext(
                                            *__slate_slot_616,
                                            std::ptr::addr_of_mut!(*__slate_slot_617),
                                            std::ptr::addr_of_mut!(*__slate_slot_618),
                                        );
                                } else {
                                    *__slate_slot_1135 = false as bool;
                                }
                                if *__slate_slot_1135 {
                                    0 as i32;
                                    0 as i32;
                                    {}
                                    if (unsafe {
                                        std::sync::atomic::AtomicI32::load_volatile(
                                            std::sync::atomic::AtomicI32::from_ptr_raw(
                                                (unsafe {
                                                    std::ptr::addr_of_mut!((*db).u1.isInterrupted)
                                                })
                                                    as *mut i32,
                                            ),
                                            std::sync::atomic::Ordering::Relaxed,
                                        )
                                    }) != (0 as i32)
                                    {
                                        break;
                                    } else {
                                        if !(*__slate_slot_618 <= *__slate_slot_625
                                            || *__slate_slot_618 > *__slate_slot_619
                                            || *__slate_slot_617 > *__slate_slot_620)
                                        {
                                            *__slate_slot_630 = ((32 as i32) as i64)
                                                + (((*__slate_slot_618)
                                                    .wrapping_sub((1 as i32) as u32)
                                                    as u64)
                                                    as i64)
                                                    * ((*__slate_slot_615 + (24 as i32)) as i64)
                                                + ((24 as i32) as i64);
                                            // testcase( IS_BIG_INT(iOffset) ); // requires a 4GiB WAL file
                                            *__slate_slot_614 = unsafe {
                                                sqlite3OsRead(
                                                    unsafe { (*pWal).pWalFd },
                                                    zBuf as *mut (),
                                                    *__slate_slot_615,
                                                    *__slate_slot_630,
                                                )
                                            };
                                            if *__slate_slot_614 != (0 as i32) {
                                                break '__join_17;
                                            } else {
                                                *__slate_slot_630 = (((*__slate_slot_617)
                                                    .wrapping_sub((1 as i32) as u32)
                                                    as u64)
                                                    as i64)
                                                    * (*__slate_slot_615 as i64);
                                                {}
                                                *__slate_slot_614 = unsafe {
                                                    sqlite3OsWrite(
                                                        unsafe { (*pWal).pDbFd },
                                                        zBuf as *const (),
                                                        *__slate_slot_615,
                                                        *__slate_slot_630,
                                                    )
                                                };
                                                if *__slate_slot_614 != (0 as i32) {
                                                    break '__join_17;
                                                }
                                            }
                                        }
                                    }
                                } else {
                                    break '__join_17;
                                }
                            }
                            *__slate_slot_614 = if (unsafe { (*db).mallocFailed }) != (0 as u8) {
                                7 as i32
                            } else {
                                9 as i32
                            };
                        }
                        unsafe {
                            sqlite3OsFileControl(
                                unsafe { (*pWal).pDbFd },
                                37 as i32,
                                std::ptr::null_mut::<()>(),
                            )
                        };
                        // If work was actually accomplished...
                        if *__slate_slot_614 == (0 as i32) {
                            if *__slate_slot_619
                                == unsafe {
                                    std::ptr::read_volatile(std::ptr::addr_of!(
                                        (*walIndexHdr(pWal)).mxFrame
                                    ))
                                }
                            {
                                std::ptr::write(
                                    __slate_slot_631,
                                    (((unsafe { (*pWal).hdr.nPage }) as u64) as i64)
                                        * (*__slate_slot_615 as i64),
                                );
                                {}
                                *__slate_slot_614 = unsafe {
                                    sqlite3OsTruncate(unsafe { (*pWal).pDbFd }, *__slate_slot_631)
                                };
                                if *__slate_slot_614 == (0 as i32) {
                                    *__slate_slot_614 = unsafe {
                                        sqlite3OsSync(
                                            unsafe { (*pWal).pDbFd },
                                            sync_flags >> (2 as i32) & (3 as i32),
                                        )
                                    };
                                }
                            }
                            if *__slate_slot_614 == (0 as i32) {
                                unsafe {
                                    std::sync::atomic::AtomicU32::store_volatile(
                                        std::sync::atomic::AtomicU32::from_ptr_raw(
                                            (unsafe {
                                                std::ptr::addr_of_mut!(
                                                    (*(*__slate_slot_622)).nBackfill
                                                )
                                            })
                                                as *mut u32,
                                        ),
                                        *__slate_slot_619,
                                        std::sync::atomic::Ordering::Relaxed,
                                    )
                                };
                                0 as i32;
                                {}
                            }
                        }
                    }
                    // Release the reader lock held while backfilling
                    walUnlockExclusive(pWal, (3 as i32) + (0 as i32), 1 as i32);
                }
                if *__slate_slot_614 == (5 as i32) {
                    // Reset the return code so as not to report a checkpoint failure
                    // just because there are active readers.
                    *__slate_slot_614 = 0 as i32;
                }
            }
            // If this is an SQLITE_CHECKPOINT_RESTART or TRUNCATE operation, and the
            // entire wal file has been copied into the database file, then block
            // until all readers have finished using the wal file. This ensures that
            // the next process to write to the database restarts the wal file.
            if *__slate_slot_614 == (0 as i32) && eMode != (0 as i32) {
                0 as i32;
                0 as i32;
                {}
                if (unsafe {
                    std::ptr::read_volatile(std::ptr::addr_of!((*(*__slate_slot_622)).nBackfill))
                }) < unsafe { (*pWal).hdr.mxFrame }
                {
                    *__slate_slot_614 = 5 as i32;
                } else {
                    if eMode >= (2 as i32) {
                        unsafe {
                            sqlite3_randomness(
                                4 as i32,
                                std::ptr::addr_of_mut!(*__slate_slot_632) as *mut (),
                            )
                        };
                        0 as i32;
                        *__slate_slot_614 = walBusyLock(
                            pWal,
                            xBusy,
                            pBusyArg,
                            (3 as i32) + (1 as i32),
                            (8 as i32) - (3 as i32) - (1 as i32),
                        );
                        if *__slate_slot_614 == (0 as i32) {
                            if eMode == (3 as i32) {
                                // IMPLEMENTATION-OF: R-44699-57140 This mode works the same way as
                                // SQLITE_CHECKPOINT_RESTART with the addition that it also
                                // truncates the log file to zero bytes just prior to a
                                // successful return.
                                //
                                // In theory, it might be safe to do this without updating the
                                // wal-index header in shared memory, as all subsequent reader or
                                // writer clients should see that the entire log file has been
                                // checkpointed and behave accordingly. This seems unsafe though,
                                // as it would leave the system in a state where the contents of
                                // the wal-index header do not match the contents of the
                                // file-system. To avoid this, update the wal-index header to
                                // indicate that the log file contains zero valid frames.
                                walRestartHdr(pWal, *__slate_slot_632);
                                *__slate_slot_614 = unsafe {
                                    sqlite3OsTruncate(unsafe { (*pWal).pWalFd }, (0 as i32) as i64)
                                };
                            }
                            walUnlockExclusive(
                                pWal,
                                (3 as i32) + (1 as i32),
                                (8 as i32) - (3 as i32) - (1 as i32),
                            );
                        }
                    }
                }
            }
        }
        {}
        walIteratorFree(*__slate_slot_616);
        return *__slate_slot_614;
    }
    return unsafe { std::mem::zeroed() };
}

/// If the WAL file is currently larger than nMax bytes in size, truncate
/// it to exactly nMax bytes. If an error occurs while doing so, ignore it.
fn walLimitSize(mut pWal: *mut Wal, mut nMax: i64) {
    let mut sz: i64 = 0 as i64;
    let mut rx: i32 = 0 as i32;
    unsafe { sqlite3BeginBenignMalloc() };
    rx = unsafe { sqlite3OsFileSize(unsafe { (*pWal).pWalFd }, std::ptr::addr_of_mut!(sz)) };
    if rx == (0 as i32) && sz > nMax {
        rx = unsafe { sqlite3OsTruncate(unsafe { (*pWal).pWalFd }, nMax) };
    }
    unsafe { sqlite3EndBenignMalloc() };
    if rx != (0 as i32) {
        unsafe {
            sqlite3_log(
                rx,
                (b"cannot limit WAL size: %s\0".as_ptr() as *mut i8) as *const i8,
                unsafe { (*pWal).zWalName },
            )
        };
    }
}

/// Close a connection to a log file.
///
/// # Arguments
///
/// * `pWal` - Wal to close
/// * `db` - For interrupt flag
/// * `sync_flags` - Flags to pass to OsSync() (or 0)
/// * `zBuf` - Buffer of at least nBuf bytes
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3WalClose(
    mut pWal: *mut Wal,
    mut db: *mut sqlite3,
    mut sync_flags: i32,
    mut nBuf: i32,
    mut zBuf: *mut u8,
) -> i32 {
    let mut rc: i32 = 0 as i32;
    if pWal != std::ptr::null_mut::<Wal>() {
        let mut isDelete: i32 = 0 as i32; // True to unlink wal and wal-index files
        0 as i32;
        // If an EXCLUSIVE lock can be obtained on the database file (using the
        // ordinary, rollback-mode locking methods, this guarantees that the
        // connection associated with this log file is the only connection to
        // the database. In this case checkpoint the database and unlink both
        // the wal and wal-index files.
        //
        // The EXCLUSIVE lock is not released before returning.
        let __v994: bool;
        if zBuf != std::ptr::null_mut::<u8>() {
            let __v995: i32 = unsafe { sqlite3OsLock(unsafe { (*pWal).pDbFd }, 4 as i32) };
            rc = __v995;
            __v994 = (0 as i32) == __v995;
        } else {
            __v994 = false as bool;
        }
        if __v994 {
            if (((unsafe { (*pWal).exclusiveMode }) as u32) as i32) == (0 as i32) {
                unsafe {
                    (*pWal).exclusiveMode = ((1 as i32) as i8) as u8;
                }
            }
            rc = sqlite3WalCheckpoint(
                pWal,
                db,
                0 as i32,
                None,
                std::ptr::null_mut::<()>(),
                sync_flags,
                nBuf,
                zBuf,
                std::ptr::null_mut::<i32>(),
                std::ptr::null_mut::<i32>(),
            );
            if rc == (0 as i32) {
                let mut bPersist: i32 = -(1 as i32);
                unsafe {
                    sqlite3OsFileControlHint(
                        unsafe { (*pWal).pDbFd },
                        10 as i32,
                        std::ptr::addr_of_mut!(bPersist) as *mut (),
                    )
                };
                if bPersist != (1 as i32) {
                    // Try to delete the WAL file if the checkpoint completed and
                    // fsynced (rc==SQLITE_OK) and if we are not in persistent-wal
                    // mode (!bPersist)
                    isDelete = 1 as i32;
                } else {
                    if (unsafe { (*pWal).mxWalSize }) >= ((0 as i32) as i64) {
                        // Try to truncate the WAL file to zero bytes if the checkpoint
                        // completed and fsynced (rc==SQLITE_OK) and we are in persistent
                        // WAL mode (bPersist) and if the PRAGMA journal_size_limit is a
                        // non-negative value (pWal->mxWalSize>=0).  Note that we truncate
                        // to zero bytes as truncating to the journal_size_limit might
                        // leave a corrupt WAL file on disk.
                        walLimitSize(pWal, (0 as i32) as i64);
                    }
                }
            }
        }
        walIndexClose(pWal, isDelete);
        unsafe { sqlite3OsClose(unsafe { (*pWal).pWalFd }) };
        if isDelete != (0 as i32) {
            unsafe { sqlite3BeginBenignMalloc() };
            unsafe {
                sqlite3OsDelete(
                    unsafe { (*pWal).pVfs },
                    unsafe { (*pWal).zWalName },
                    0 as i32,
                )
            };
            unsafe { sqlite3EndBenignMalloc() };
        }
        {}
        unsafe { sqlite3_free((unsafe { (*pWal).apWiData }) as *mut ()) };
        unsafe { sqlite3_free(pWal as *mut ()) };
    }
    return rc;
}

/// Try to read the wal-index header.  Return 0 on success and 1 if
/// there is a problem.
///
/// The wal-index is in shared memory.  Another thread or process might
/// be writing the header at the same time this procedure is trying to
/// read it, which might result in inconsistency.  A dirty read is detected
/// by verifying that both copies of the header are the same and also by
/// a checksum on the header.
///
/// If and only if the read is consistent and the header is different from
/// pWal->hdr, then pWal->hdr is updated to the content of the new header
/// and *pChanged is set to 1.
///
/// If the checksum cannot be verified return non-zero. If the header
/// is read successfully and the checksum verified, return zero.
fn walIndexTryHdr(mut pWal: *mut Wal, mut pChanged: *mut i32) -> i32 {
    let mut aCksum: [u32; 2] = [0 as u32; 2]; // Checksum on the header content
    let mut h1: WalIndexHdr = unsafe { std::mem::zeroed() };
    let mut h2: WalIndexHdr = unsafe { std::mem::zeroed() }; // Two copies of the header content
    let mut aHdr: *mut WalIndexHdr = unsafe { std::mem::zeroed() }; // Header in shared memory
    // The first page of the wal-index must be mapped at this point.
    0 as i32;
    // Read the header. This might happen concurrently with a write to the
    // same area of shared memory on a different CPU in a SMP,
    // meaning it is possible that an inconsistent snapshot is read
    // from the file. If this happens, return non-zero.
    //
    // tag-20200519-1:
    // There are two copies of the header at the beginning of the wal-index.
    // When reading, read [0] first then [1].  Writes are in the reverse order.
    // Memory barriers are used to prevent the compiler or the hardware from
    // reordering the reads and writes.  TSAN and similar tools can sometimes
    // give false-positive warnings about these accesses because the tools do not
    // account for the double-read and the memory barrier. The use of mutexes
    // here would be problematic as the memory being accessed is potentially
    // shared among multiple processes and not all mutex implementations work
    // reliably in that environment.
    aHdr = walIndexHdr(pWal);
    unsafe {
        memcpy(
            std::ptr::addr_of_mut!(h1) as *mut (),
            ((unsafe { aHdr.offset((0 as i32) as isize) }) as *mut ()) as *const (),
            48 as u64,
        )
    }; // Possible TSAN false-positive
    walShmBarrier(pWal);
    unsafe {
        memcpy(
            std::ptr::addr_of_mut!(h2) as *mut (),
            ((unsafe { aHdr.offset((1 as i32) as isize) }) as *mut ()) as *const (),
            48 as u64,
        )
    };
    if (unsafe {
        memcmp(
            std::ptr::addr_of_mut!(h1) as *const (),
            std::ptr::addr_of_mut!(h2) as *const (),
            48 as u64,
        )
    }) != (0 as i32)
    {
        return 1 as i32; // Dirty read
    }
    if ((h1.isInit as u32) as i32) == (0 as i32) {
        return 1 as i32; // Malformed header - probably all zeros
    }
    walChecksumBytes(
        1 as i32,
        std::ptr::addr_of_mut!(h1) as *mut u8,
        ((48 as u64).wrapping_sub(8 as u64) as u32) as i32,
        std::ptr::null::<u32>(),
        aCksum.as_mut_ptr() as *mut u32,
    );
    if (unsafe { *unsafe { (aCksum.as_mut_ptr() as *mut u32).offset((0 as i32) as isize) } })
        != unsafe { *unsafe { (h1.aCksum.as_mut_ptr() as *mut u32).offset((0 as i32) as isize) } }
        || (unsafe { *unsafe { (aCksum.as_mut_ptr() as *mut u32).offset((1 as i32) as isize) } })
            != unsafe {
                *unsafe { (h1.aCksum.as_mut_ptr() as *mut u32).offset((1 as i32) as isize) }
            }
    {
        return 1 as i32; // Checksum does not match
    }
    if (unsafe {
        memcmp(
            (unsafe { std::ptr::addr_of_mut!((*pWal).hdr) }) as *const (),
            std::ptr::addr_of_mut!(h1) as *const (),
            48 as u64,
        )
    }) != (0 as i32)
    {
        unsafe {
            *pChanged = 1 as i32;
        }
        unsafe {
            memcpy(
                (unsafe { std::ptr::addr_of_mut!((*pWal).hdr) }) as *mut (),
                std::ptr::addr_of_mut!(h1) as *const (),
                48 as u64,
            )
        };
        unsafe {
            (*pWal).szPage = (((((unsafe { (*pWal).hdr.szPage }) as u32) as i32) & (65024 as i32))
                + (((((unsafe { (*pWal).hdr.szPage }) as u32) as i32) & (1 as i32)) << (16 as i32)))
                as u32;
        }
        {}
        {}
    }
    // The header was successfully read. Return zero.
    return 0 as i32;
}

// This is the value that walTryBeginRead returns when it needs to
// be retried.
/// Read the wal-index header from the wal-index and into pWal->hdr.
/// If the wal-header appears to be corrupt, try to reconstruct the
/// wal-index from the WAL before returning.
///
/// Set *pChanged to 1 if the wal-index header value in pWal->hdr is
/// changed by this operation.  If pWal->hdr is unchanged, set *pChanged
/// to 0.
///
/// If the wal-index header is successfully read, return SQLITE_OK.
/// Otherwise an SQLite error code.
fn walIndexReadHdr(mut pWal: *mut Wal, mut pChanged: *mut i32) -> i32 {
    let mut rc: i32 = 0 as i32; // Return code
    let mut badHdr: i32 = 0 as i32; // True if a header read failed
    let mut page0: *mut u32 = unsafe { std::mem::zeroed() }; // Chunk of wal-index containing header
    // Ensure that page 0 of the wal-index (the page that contains the
    // wal-index header) is mapped. Return early if an error occurs here.
    0 as i32;
    rc = walIndexPage(pWal, 0 as i32, std::ptr::addr_of_mut!(page0));
    if rc != (0 as i32) {
        0 as i32; // READONLY changed to OK in walIndexPage
        if rc == (8 as i32) | (5 as i32) << (8 as i32) {
            // The SQLITE_READONLY_CANTINIT return means that the shared-memory
            // was openable but is not writable, and this thread is unable to
            // confirm that another write-capable connection has the shared-memory
            // open, and hence the content of the shared-memory is unreliable,
            // since the shared-memory might be inconsistent with the WAL file
            // and there is no writer on hand to fix it.
            0 as i32;
            0 as i32;
            0 as i32;
            unsafe {
                (*pWal).bShmUnreliable = ((1 as i32) as i8) as u8;
            }
            unsafe {
                (*pWal).exclusiveMode = ((2 as i32) as i8) as u8;
            }
            unsafe {
                *pChanged = 1 as i32;
            }
        } else {
            return rc; // Any other non-OK return is just an error
        }
    } else {
        // page0 can be NULL if the SHM is zero bytes in size and pWal->writeLock
        // is zero, which prevents the SHM from growing
        {}
    }
    0 as i32;
    // If the first page of the wal-index has been mapped, try to read the
    // wal-index header immediately, without holding any lock. This usually
    // works, but may fail if the wal-index header is corrupt or currently
    // being modified by another thread or process.
    let __v1136: i32;
    if page0 != std::ptr::null_mut::<u32>() {
        __v1136 = walIndexTryHdr(pWal, pChanged);
    } else {
        __v1136 = 1 as i32;
    }
    badHdr = __v1136;
    // If the first attempt failed, it might have been due to a race
    // with a writer.  So get a WRITE lock and try again.
    if badHdr != (0 as i32) {
        if (((unsafe { (*pWal).bShmUnreliable }) as u32) as i32) == (0 as i32)
            && (((unsafe { (*pWal).readOnly }) as u32) as i32) & (2 as i32) != (0 as i32)
        {
            let __v1137: i32 = walLockShared(pWal, 0 as i32);
            rc = __v1137;
            if (0 as i32) == __v1137 {
                walUnlockShared(pWal, 0 as i32);
                rc = (8 as i32) | (1 as i32) << (8 as i32);
            }
        } else {
            let mut bWriteLock: i32 = ((unsafe { (*pWal).writeLock }) as u32) as i32;
            let __v1138: bool;
            if bWriteLock != (0 as i32) {
                __v1138 = true as bool;
            } else {
                let __v1139: i32 = walLockExclusive(pWal, 0 as i32, 1 as i32);
                rc = __v1139;
                __v1138 = (0 as i32) == __v1139;
            }
            if __v1138 {
                // If the write-lock was just obtained, set writeLock to 2 instead of
                // the usual 1. This causes walIndexPage() to behave as if the
                // write-lock were held (so that it allocates new pages as required),
                // and walHandleException() to unlock the write-lock if a SEH exception
                // is thrown.
                if !(bWriteLock != (0 as i32)) {
                    unsafe {
                        (*pWal).writeLock = ((2 as i32) as i8) as u8;
                    }
                }
                let __v1140: i32 = walIndexPage(pWal, 0 as i32, std::ptr::addr_of_mut!(page0));
                rc = __v1140;
                if (0 as i32) == __v1140 {
                    badHdr = walIndexTryHdr(pWal, pChanged);
                    if badHdr != (0 as i32) {
                        // If the wal-index header is still malformed even while holding
                        // a WRITE lock, it can only mean that the header is corrupted and
                        // needs to be reconstructed.  So run recovery to do exactly that.
                        // Disable blocking locks first.
                        {}
                        rc = walIndexRecover(pWal);
                        unsafe {
                            *pChanged = 1 as i32;
                        }
                    }
                }
                if bWriteLock == (0 as i32) {
                    unsafe {
                        (*pWal).writeLock = ((0 as i32) as i8) as u8;
                    }
                    walUnlockExclusive(pWal, 0 as i32, 1 as i32);
                }
            }
        }
    }
    // If the header is read successfully, check the version number to make
    // sure the wal-index was not constructed with some future format that
    // this version of SQLite cannot understand.
    if badHdr == (0 as i32) && (unsafe { (*pWal).hdr.iVersion }) != ((3007000 as i32) as u32) {
        rc = unsafe { sqlite3CantopenError(2747 as i32) };
    }
    if (unsafe { (*pWal).bShmUnreliable }) != (0 as u8) {
        if rc != (0 as i32) {
            walIndexClose(pWal, 0 as i32);
            unsafe {
                (*pWal).bShmUnreliable = ((0 as i32) as i8) as u8;
            }
            0 as i32;
            // walIndexRecover() might have returned SHORT_READ if a concurrent
            // writer truncated the WAL out from under it.  If that happens, it
            // indicates that a writer has fixed the SHM file for us, so retry
            if rc == (10 as i32) | (2 as i32) << (8 as i32) {
                rc = -(1 as i32);
            }
        }
        unsafe {
            (*pWal).exclusiveMode = ((0 as i32) as i8) as u8;
        }
    }
    return rc;
}

/// Open a transaction in a connection where the shared-memory is read-only
/// and where we cannot verify that there is a separate write-capable connection
/// on hand to keep the shared-memory up-to-date with the WAL file.
///
/// This can happen, for example, when the shared-memory is implemented by
/// memory-mapping a *-shm file, where a prior writer has shut down and
/// left the *-shm file on disk, and now the present connection is trying
/// to use that database but lacks write permission on the *-shm file.
/// Other scenarios are also possible, depending on the VFS implementation.
///
/// Precondition:
///
///    The *-wal file has been read and an appropriate wal-index has been
///    constructed in pWal->apWiData[] using heap memory instead of shared
///    memory.
///
/// If this function returns SQLITE_OK, then the read transaction has
/// been successfully opened. In this case output variable (*pChanged)
/// is set to true before returning if the caller should discard the
/// contents of the page cache before proceeding. Or, if it returns
/// WAL_RETRY, then the heap memory wal-index has been discarded and
/// the caller should retry opening the read transaction from the
/// beginning (including attempting to map the *-shm file).
///
/// If an error occurs, an SQLite error code is returned.
fn walBeginShmUnreliable(mut pWal: *mut Wal, mut pChanged: *mut i32) -> i32 {
    let mut __slate_storage_1144: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1144: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1144) as *mut i32;
    let mut __slate_storage_1143: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1143: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1143) as *mut i32;
    let mut __slate_storage_675: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_675: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_675) as *mut i32;
    let mut __slate_storage_1142: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1142: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_1142) as *mut i64;
    let mut __slate_storage_1141: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1141: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_1141) as *mut i64; // dbsize field from frame header
    let mut __slate_storage_674: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_674: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_674) as *mut u32; // Database page number for frame
    let mut __slate_storage_673: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_673: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_673) as *mut u32; // Saved copy of pWal->hdr.aFrameCksum
    let mut __slate_storage_672: std::mem::MaybeUninit<[u32; 2]> = std::mem::MaybeUninit::uninit();
    let __slate_slot_672: *mut [u32; 2] =
        std::ptr::addr_of_mut!(__slate_storage_672) as *mut [u32; 2]; // Return code
    let mut __slate_storage_671: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_671: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_671) as *mut i32; // Dummy argument for xShmMap
    let mut __slate_storage_670: std::mem::MaybeUninit<*mut ()> = std::mem::MaybeUninit::uninit();
    let __slate_slot_670: *mut *mut () =
        std::ptr::addr_of_mut!(__slate_storage_670) as *mut *mut (); // Pointer to data part of aFrame buffer
    let mut __slate_storage_669: std::mem::MaybeUninit<*mut u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_669: *mut *mut u8 =
        std::ptr::addr_of_mut!(__slate_storage_669) as *mut *mut u8; // Number of bytes in buffer aFrame[]
    let mut __slate_storage_668: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_668: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_668) as *mut i32; // Malloc'd buffer to load entire frame
    let mut __slate_storage_667: std::mem::MaybeUninit<*mut u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_667: *mut *mut u8 =
        std::ptr::addr_of_mut!(__slate_storage_667) as *mut *mut u8; // Buffer to load WAL header into
    let mut __slate_storage_666: std::mem::MaybeUninit<__SlateAlign16<[u8; 32]>> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_666: *mut [u8; 32] =
        std::ptr::addr_of_mut!(__slate_storage_666) as *mut [u8; 32]; // Current offset when reading wal file
    let mut __slate_storage_665: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_665: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_665) as *mut i64; // Size of wal file on disk in bytes
    let mut __slate_storage_664: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_664: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_664) as *mut i64;
    unsafe {
        '__join_5: {
            std::ptr::write(__slate_slot_667, std::ptr::null_mut::<u8>());
            0 as i32;
            0 as i32;
            0 as i32;
            // Take WAL_READ_LOCK(0). This has the effect of preventing any
            // writers from running a checkpoint, but does not stop them
            // from running recovery.
            *__slate_slot_671 = walLockShared(pWal, (3 as i32) + (0 as i32));
            if *__slate_slot_671 != (0 as i32) {
                if *__slate_slot_671 == (5 as i32) {
                    *__slate_slot_671 = -(1 as i32);
                }
            } else {
                unsafe {
                    (*pWal).readLock = (0 as i32) as i16;
                }
                // Check to see if a separate writer has attached to the shared-memory area,
                // thus making the shared-memory "reliable" again.  Do this by invoking
                // the xShmMap() routine of the VFS and looking to see if the return
                // is SQLITE_READONLY instead of SQLITE_READONLY_CANTINIT.
                //
                // If the shared-memory is now "reliable" return WAL_RETRY, which will
                // cause the heap-memory WAL-index to be discarded and the actual
                // shared memory to be used in its place.
                //
                // This step is important because, even though this connection is holding
                // the WAL_READ_LOCK(0) which prevents a checkpoint, a writer might
                // have already checkpointed the WAL file and, while the current
                // is active, wrap the WAL and start overwriting frames that this
                // process wants to use.
                //
                // Once sqlite3OsShmMap() has been called for an sqlite3_file and has
                // returned any SQLITE_READONLY value, it must return only SQLITE_READONLY
                // or SQLITE_READONLY_CANTINIT or some error for all subsequent invocations,
                // even if some external agent does a "chmod" to make the shared-memory
                // writable by us, until sqlite3OsShmUnmap() has been called.
                // This is a requirement on the VFS implementation.
                *__slate_slot_671 = unsafe {
                    sqlite3OsShmMap(
                        unsafe { (*pWal).pDbFd },
                        0 as i32,
                        ((2 as u64)
                            .wrapping_mul((((4096 as i32) * (2 as i32)) as i64) as u64)
                            .wrapping_add((((4096 as i32) as i64) as u64).wrapping_mul(4 as u64))
                            as u32) as i32,
                        0 as i32,
                        std::ptr::addr_of_mut!(*__slate_slot_670),
                    )
                };
                0 as i32; // SQLITE_OK not possible for read-only connection
                if *__slate_slot_671 != (8 as i32) | (5 as i32) << (8 as i32) {
                    *__slate_slot_671 = if *__slate_slot_671 == (8 as i32) {
                        -(1 as i32)
                    } else {
                        *__slate_slot_671
                    };
                } else {
                    // We reach this point only if the real shared-memory is still unreliable.
                    // Assume the in-memory WAL-index substitute is correct and load it
                    // into pWal->hdr.
                    unsafe {
                        memcpy(
                            (unsafe { std::ptr::addr_of_mut!((*pWal).hdr) }) as *mut (),
                            (walIndexHdr(pWal) as *mut ()) as *const (),
                            48 as u64,
                        )
                    };
                    // Make sure some writer hasn't come in and changed the WAL file out
                    // from under us, then disconnected, while we were not looking.
                    *__slate_slot_671 = unsafe {
                        sqlite3OsFileSize(
                            unsafe { (*pWal).pWalFd },
                            std::ptr::addr_of_mut!(*__slate_slot_664),
                        )
                    };
                    if *__slate_slot_671 != (0 as i32) {
                    } else {
                        if *__slate_slot_664 < ((32 as i32) as i64) {
                            // If the wal file is too small to contain a wal-header and the
                            // wal-index header has mxFrame==0, then it must be safe to proceed
                            // reading the database file only. However, the page cache cannot
                            // be trusted, as a read/write connection may have connected, written
                            // the db, run a checkpoint, truncated the wal file and disconnected
                            // since this client's last read transaction.
                            unsafe {
                                *pChanged = 1 as i32;
                            }
                            *__slate_slot_671 =
                                if (unsafe { (*pWal).hdr.mxFrame }) == ((0 as i32) as u32) {
                                    0 as i32
                                } else {
                                    -(1 as i32)
                                };
                        } else {
                            // Check the salt keys at the start of the wal file still match.
                            *__slate_slot_671 = unsafe {
                                sqlite3OsRead(
                                    unsafe { (*pWal).pWalFd },
                                    ((*__slate_slot_666).as_mut_ptr() as *mut u8) as *mut (),
                                    32 as i32,
                                    (0 as i32) as i64,
                                )
                            };
                            if *__slate_slot_671 != (0 as i32) {
                            } else {
                                if (unsafe {
                                    memcmp(
                                        (unsafe { std::ptr::addr_of_mut!((*pWal).hdr.aSalt) })
                                            as *const (),
                                        (unsafe {
                                            ((*__slate_slot_666).as_mut_ptr() as *mut u8)
                                                .offset((16 as i32) as isize)
                                        }) as *const (),
                                        ((8 as i32) as i64) as u64,
                                    )
                                }) != (0 as i32)
                                {
                                    // Some writer has wrapped the WAL file while we were not looking.
                                    // Return WAL_RETRY which will cause the in-memory WAL-index to be
                                    // rebuilt.
                                    *__slate_slot_671 = -(1 as i32);
                                } else {
                                    // Allocate a buffer to read frames into
                                    0 as i32;
                                    0 as i32;
                                    *__slate_slot_668 = unsafe { (*pWal).szPage }
                                        .wrapping_add((24 as i32) as u32)
                                        as i32;
                                    *__slate_slot_667 = (unsafe {
                                        sqlite3_malloc64((*__slate_slot_668 as i64) as u64)
                                    })
                                        as *mut u8;
                                    if *__slate_slot_667 == std::ptr::null_mut::<u8>() {
                                        *__slate_slot_671 = 7 as i32;
                                    } else {
                                        *__slate_slot_669 = unsafe {
                                            (*__slate_slot_667).offset((24 as i32) as isize)
                                        };
                                        // Check to see if a complete transaction has been appended to the
                                        // wal file since the heap-memory wal-index was created. If so, the
                                        // heap-memory wal-index is discarded and WAL_RETRY returned to
                                        // the caller.
                                        unsafe {
                                            *unsafe {
                                                ((*__slate_slot_672).as_mut_ptr() as *mut u32)
                                                    .offset((0 as i32) as isize)
                                            } = unsafe {
                                                *unsafe {
                                                    unsafe {
                                                        (*pWal).hdr.aFrameCksum.as_mut_ptr()
                                                            as *mut u32
                                                    }
                                                    .offset((0 as i32) as isize)
                                                }
                                            };
                                        }
                                        unsafe {
                                            *unsafe {
                                                ((*__slate_slot_672).as_mut_ptr() as *mut u32)
                                                    .offset((1 as i32) as isize)
                                            } = unsafe {
                                                *unsafe {
                                                    unsafe {
                                                        (*pWal).hdr.aFrameCksum.as_mut_ptr()
                                                            as *mut u32
                                                    }
                                                    .offset((1 as i32) as isize)
                                                }
                                            };
                                        }
                                        *__slate_slot_665 = ((32 as i32) as i64)
                                            + ((unsafe { (*pWal).hdr.mxFrame }
                                                .wrapping_add((1 as i32) as u32)
                                                .wrapping_sub((1 as i32) as u32)
                                                as u64)
                                                as i64)
                                                * ((unsafe { (*pWal).szPage }
                                                    .wrapping_add((24 as i32) as u32)
                                                    as u64)
                                                    as i64);
                                        '__join_6: {
                                            loop {
                                                if *__slate_slot_665 + (*__slate_slot_668 as i64)
                                                    <= *__slate_slot_664
                                                {
                                                    // Read and decode the next log frame.
                                                    *__slate_slot_671 = unsafe {
                                                        sqlite3OsRead(
                                                            unsafe { (*pWal).pWalFd },
                                                            *__slate_slot_667 as *mut (),
                                                            *__slate_slot_668,
                                                            *__slate_slot_665,
                                                        )
                                                    };
                                                    if *__slate_slot_671 != (0 as i32) {
                                                        break '__join_6;
                                                    } else {
                                                        if !(walDecodeFrame(
                                                            pWal,
                                                            std::ptr::addr_of_mut!(
                                                                *__slate_slot_673
                                                            ),
                                                            std::ptr::addr_of_mut!(
                                                                *__slate_slot_674
                                                            ),
                                                            *__slate_slot_669,
                                                            *__slate_slot_667,
                                                        ) != (0 as i32))
                                                        {
                                                            break '__join_6;
                                                        } else {
                                                            // If nTruncate is non-zero, then a complete transaction has been
                                                            // appended to this wal file. Set rc to WAL_RETRY and break out of
                                                            // the loop.
                                                            if *__slate_slot_674 != (0 as u32) {
                                                                break;
                                                            } else {
                                                                std::ptr::write(
                                                                    __slate_slot_1141,
                                                                    *__slate_slot_665,
                                                                );
                                                                std::ptr::write(
                                                                    __slate_slot_1142,
                                                                    *__slate_slot_1141
                                                                        + (*__slate_slot_668
                                                                            as i64),
                                                                );
                                                                *__slate_slot_665 =
                                                                    *__slate_slot_1142;
                                                            }
                                                        }
                                                    }
                                                } else {
                                                    break '__join_6;
                                                }
                                            }
                                            *__slate_slot_671 = -(1 as i32);
                                        }
                                        unsafe {
                                            *unsafe {
                                                unsafe {
                                                    (*pWal).hdr.aFrameCksum.as_mut_ptr() as *mut u32
                                                }
                                                .offset((0 as i32) as isize)
                                            } = unsafe {
                                                *unsafe {
                                                    ((*__slate_slot_672).as_mut_ptr() as *mut u32)
                                                        .offset((0 as i32) as isize)
                                                }
                                            };
                                        }
                                        unsafe {
                                            *unsafe {
                                                unsafe {
                                                    (*pWal).hdr.aFrameCksum.as_mut_ptr() as *mut u32
                                                }
                                                .offset((1 as i32) as isize)
                                            } = unsafe {
                                                *unsafe {
                                                    ((*__slate_slot_672).as_mut_ptr() as *mut u32)
                                                        .offset((1 as i32) as isize)
                                                }
                                            };
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        unsafe { sqlite3_free(*__slate_slot_667 as *mut ()) };
        if *__slate_slot_671 != (0 as i32) {
            *__slate_slot_675 = 0 as i32;
            loop {
                if *__slate_slot_675 < unsafe { (*pWal).nWiData } {
                    unsafe {
                        sqlite3_free(
                            (unsafe {
                                *unsafe {
                                    unsafe { (*pWal).apWiData }.offset(*__slate_slot_675 as isize)
                                }
                            }) as *mut (),
                        )
                    };
                    unsafe {
                        *unsafe {
                            unsafe { (*pWal).apWiData }.offset(*__slate_slot_675 as isize)
                        } = std::ptr::null_mut::<u32>();
                    }
                    std::ptr::write(__slate_slot_1143, *__slate_slot_675);
                    std::ptr::write(__slate_slot_1144, *__slate_slot_1143 + (1 as i32));
                    *__slate_slot_675 = *__slate_slot_1144;
                } else {
                    break;
                }
            }
            unsafe {
                (*pWal).bShmUnreliable = ((0 as i32) as i8) as u8;
            }
            sqlite3WalEndReadTransaction(pWal);
            unsafe {
                *pChanged = 1 as i32;
            }
        }
        return *__slate_slot_671;
    }
    return unsafe { std::mem::zeroed() };
}

// The final argument passed to walTryBeginRead() is of type (int*). The
// caller should invoke walTryBeginRead as follows:
//
//   int cnt = 0;
//   do {
//     rc = walTryBeginRead(..., &cnt);
//   }while( rc==WAL_RETRY );
//
// The final value of "cnt" is of no use to the caller. It is used by
// the implementation of walTryBeginRead() as follows:
//
//   + Each time walTryBeginRead() is called, it is incremented. Once
//     it reaches WAL_RETRY_PROTOCOL_LIMIT - indicating that walTryBeginRead()
//     has many times been invoked and failed with WAL_RETRY - walTryBeginRead()
//     returns SQLITE_PROTOCOL.
//
//   + If SQLITE_ENABLE_SETLK_TIMEOUT is defined and walTryBeginRead() failed
//     because a blocking lock timed out (SQLITE_BUSY_TIMEOUT from the OS
//     layer), the WAL_RETRY_BLOCKED_MASK bit is set in "cnt". In this case
//     the next invocation of walTryBeginRead() may omit an expected call to
//     sqlite3OsSleep(). There has already been a delay when the previous call
//     waited on a lock.
/// Attempt to start a read transaction.  This might fail due to a race or
/// other transient condition.  When that happens, it returns WAL_RETRY to
/// indicate to the caller that it is safe to retry immediately.
///
/// On success return SQLITE_OK.  On a permanent failure (such an
/// I/O error or an SQLITE_BUSY because another process is running
/// recovery) return a positive error code.
///
/// The useWal parameter is true to force the use of the WAL and disable
/// the case where the WAL is bypassed because it has been completely
/// checkpointed.  If useWal==0 then this routine calls walIndexReadHdr()
/// to make a copy of the wal-index header into pWal->hdr.  If the
/// wal-index header has changed, *pChanged is set to 1 (as an indication
/// to the caller that the local page cache is obsolete and needs to be
/// flushed.)  When useWal==1, the wal-index header is assumed to already
/// be loaded and the pChanged parameter is unused.
///
/// The caller must set the cnt parameter to the number of prior calls to
/// this routine during the current read attempt that returned WAL_RETRY.
/// This routine will start taking more aggressive measures to clear the
/// race conditions after multiple WAL_RETRY returns, and after an excessive
/// number of errors will ultimately return SQLITE_PROTOCOL.  The
/// SQLITE_PROTOCOL return indicates that some other process has gone rogue
/// and is not honoring the locking protocol.  There is a vanishingly small
/// chance that SQLITE_PROTOCOL could be returned because of a run of really
/// bad luck when there is lots of contention for the wal-index, but that
/// possibility is so small that it can be safely neglected, we believe.
///
/// On success, this routine obtains a read lock on
/// WAL_READ_LOCK(pWal->readLock).  The pWal->readLock integer is
/// in the range 0 <= pWal->readLock < WAL_NREADER.  If pWal->readLock==(-1)
/// that means the Wal does not hold any read lock.  The reader must not
/// access any database page that is modified by a WAL frame up to and
/// including frame number aReadMark[pWal->readLock].  The reader will
/// use WAL frames up to and including pWal->hdr.mxFrame if pWal->readLock>0
/// Or if pWal->readLock==0, then the reader will ignore the WAL
/// completely and get all content directly from the database file.
/// If the useWal parameter is 1 then the WAL will never be ignored and
/// this routine will always set pWal->readLock>0 on success.
/// When the read transaction is completed, the caller must release the
/// lock on WAL_READ_LOCK(pWal->readLock) and set pWal->readLock to -1.
///
/// This routine uses the nBackfill and aReadMark[] fields of the header
/// to select a particular WAL_READ_LOCK() that strives to let the
/// checkpoint process do as much work as possible.  This routine might
/// update values of the aReadMark[] array in the header, but if it does
/// so it takes care to hold an exclusive lock on the corresponding
/// WAL_READ_LOCK() while changing values.
fn walTryBeginRead(
    mut pWal: *mut Wal,
    mut pChanged: *mut i32,
    mut useWal: i32,
    mut pCnt: *mut i32,
) -> i32 {
    let mut pInfo: *mut WalCkptInfo = unsafe { std::mem::zeroed() }; // Checkpoint information in wal-index
    let mut rc: i32 = 0 as i32; // Return code
    0 as i32; // Not currently locked
    // useWal may only be set for read/write connections
    0 as i32;
    // Take steps to avoid spinning forever if there is a protocol error.
    //
    // Circumstances that cause a RETRY should only last for the briefest
    // instances of time.  No I/O or other system calls are done while the
    // locks are held, so the locks should not be held for very long. But
    // if we are unlucky, another process that is holding a lock might get
    // paged out or take a page-fault that is time-consuming to resolve,
    // during the few nanoseconds that it is holding the lock.  In that case,
    // it might take longer than normal for the lock to free.
    //
    // After 5 RETRYs, we begin calling sqlite3OsSleep().  The first few
    // calls to sqlite3OsSleep() have a delay of 1 microsecond.  Really this
    // is more of a scheduler yield than an actual delay.  But on the 10th
    // an subsequent retries, the delays start becoming longer and longer,
    // so that on the 100th (and last) RETRY we delay for 323 milliseconds.
    // The total delay time before giving up is less than 10 seconds.
    let __v1145: *mut i32 = pCnt;
    let __v1146: i32 = unsafe { *__v1145 };
    let __v1147: i32 = __v1146 + (1 as i32);
    unsafe {
        *__v1145 = __v1147;
    }
    if (unsafe { *pCnt }) > (5 as i32) {
        let mut nDelay: i32 = 1 as i32; // Pause time in microseconds
        let mut cnt: i32 = (unsafe { *pCnt }) & !(0 as i32);
        if cnt > (100 as i32) {
            return 15 as i32;
        }
        if (unsafe { *pCnt }) >= (10 as i32) {
            nDelay = (cnt - (9 as i32)) * (cnt - (9 as i32)) * (39 as i32);
        }
        unsafe { sqlite3OsSleep(unsafe { (*pWal).pVfs }, nDelay) };
        let __v1148: *mut i32 = pCnt;
        let __v1149: i32 = unsafe { *__v1148 };
        let __v1150: i32 = __v1149 & !(0 as i32);
        unsafe {
            *__v1148 = __v1150;
        }
    }
    if !(useWal != (0 as i32)) {
        0 as i32;
        if (((unsafe { (*pWal).bShmUnreliable }) as u32) as i32) == (0 as i32) {
            rc = walIndexReadHdr(pWal, pChanged);
        }
        if rc == (5 as i32) {
            // If there is not a recovery running in another thread or process
            // then convert BUSY errors to WAL_RETRY.  If recovery is known to
            // be running, convert BUSY to BUSY_RECOVERY.  There is a race here
            // which might cause WAL_RETRY to be returned even if BUSY_RECOVERY
            // would be technically correct.  But the race is benign since with
            // WAL_RETRY this routine will be called again and will probably be
            // right on the second iteration.
            0 as i32;
            if (unsafe { *unsafe { unsafe { (*pWal).apWiData }.offset((0 as i32) as isize) } })
                == std::ptr::null_mut::<u32>()
            {
                // This branch is taken when the xShmMap() method returns SQLITE_BUSY.
                // We assume this is a transient condition, so return WAL_RETRY. The
                // xShmMap() implementation used by the default unix and win32 VFS
                // modules may return SQLITE_BUSY due to a race condition in the
                // code that determines whether or not the shared-memory region
                // must be zeroed before the requested page is returned.
                rc = -(1 as i32);
            } else {
                let __v1151: i32 = walLockShared(pWal, 2 as i32);
                rc = __v1151;
                if (0 as i32) == __v1151 {
                    walUnlockShared(pWal, 2 as i32);
                    rc = -(1 as i32);
                } else {
                    if rc == (5 as i32) {
                        rc = (5 as i32) | (1 as i32) << (8 as i32);
                    }
                }
            }
        }
        {}
        if rc != (0 as i32) {
            return rc;
        } else {
            if (unsafe { (*pWal).bShmUnreliable }) != (0 as u8) {
                return walBeginShmUnreliable(pWal, pChanged);
            }
        }
    }
    0 as i32;
    0 as i32;
    pInfo = walCkptInfo(pWal);
    0 as i32;
    {}
    let mut mxReadMark: u32 = 0 as u32; // Largest aReadMark[] value
    let mut mxI: i32 = 0 as i32; // Index of largest aReadMark[] value
    let mut i: i32 = 0 as i32; // Loop counter
    let mut mxFrame: u32 = 0 as u32; // Wal frame to lock to
    if !(useWal != (0 as i32))
        && (unsafe {
            std::sync::atomic::AtomicU32::load_volatile(
                std::sync::atomic::AtomicU32::from_ptr_raw(
                    (unsafe { std::ptr::addr_of_mut!((*pInfo).nBackfill) }) as *mut u32,
                ),
                std::sync::atomic::Ordering::Relaxed,
            )
        }) == unsafe { (*pWal).hdr.mxFrame }
    {
        // The WAL has been completely backfilled (or it is empty).
        // and can be safely ignored.
        rc = walLockShared(pWal, (3 as i32) + (0 as i32));
        walShmBarrier(pWal);
        if rc == (0 as i32) {
            if (unsafe {
                memcmp(
                    (walIndexHdr(pWal) as *mut ()) as *const (),
                    (unsafe { std::ptr::addr_of_mut!((*pWal).hdr) }) as *const (),
                    48 as u64,
                )
            }) != (0 as i32)
            {
                // It is not safe to allow the reader to continue here if frames
                // may have been appended to the log before READ_LOCK(0) was obtained.
                // When holding READ_LOCK(0), the reader ignores the entire log file,
                // which implies that the database file contains a trustworthy
                // snapshot. Since holding READ_LOCK(0) prevents a checkpoint from
                // happening, this is usually correct.
                //
                // However, if frames have been appended to the log (or if the log
                // is wrapped and written for that matter) before the READ_LOCK(0)
                // is obtained, that is not necessarily true. A checkpointer may
                // have started to backfill the appended frames but crashed before
                // it finished. Leaving a corrupt image in the database file.
                walUnlockShared(pWal, (3 as i32) + (0 as i32));
                return -(1 as i32);
            }
            unsafe {
                (*pWal).readLock = (0 as i32) as i16;
            }
            return 0 as i32;
        } else {
            if rc != (5 as i32) {
                return rc;
            }
        }
    }
    // If we get this far, it means that the reader will want to use
    // the WAL to get at content from recent commits.  The job now is
    // to select one of the aReadMark[] entries that is closest to
    // but not exceeding pWal->hdr.mxFrame and lock that entry.
    mxReadMark = (0 as i32) as u32;
    mxI = 0 as i32;
    mxFrame = unsafe { (*pWal).hdr.mxFrame };
    i = 1 as i32;
    '__slate_break_982: loop {
        if !(i < (8 as i32) - (3 as i32)) {
            break;
        }
        let mut thisMark: u32 = unsafe {
            std::sync::atomic::AtomicU32::load_volatile(
                std::sync::atomic::AtomicU32::from_ptr_raw(
                    (unsafe {
                        unsafe { (*pInfo).aReadMark.as_mut_ptr() as *mut u32 }.offset(i as isize)
                    }) as *mut u32,
                ),
                std::sync::atomic::Ordering::Relaxed,
            )
        };
        0 as i32;
        {}
        if mxReadMark <= thisMark && thisMark <= mxFrame {
            0 as i32;
            mxReadMark = thisMark;
            mxI = i;
        }
        let __v1152: i32 = i;
        let __v1153: i32 = __v1152 + (1 as i32);
        i = __v1153;
    }
    if (((unsafe { (*pWal).readOnly }) as u32) as i32) & (2 as i32) == (0 as i32)
        && (mxReadMark < mxFrame || mxI == (0 as i32))
    {
        i = 1 as i32;
        '__slate_break_983: loop {
            if !(i < (8 as i32) - (3 as i32)) {
                break;
            }
            rc = walLockExclusive(pWal, (3 as i32) + i, 1 as i32);
            if rc == (0 as i32) {
                unsafe {
                    std::sync::atomic::AtomicU32::store_volatile(
                        std::sync::atomic::AtomicU32::from_ptr_raw(
                            (unsafe {
                                unsafe { (*pInfo).aReadMark.as_mut_ptr() as *mut u32 }
                                    .offset(i as isize)
                            }) as *mut u32,
                        ),
                        mxFrame,
                        std::sync::atomic::Ordering::Relaxed,
                    )
                };
                mxReadMark = mxFrame;
                mxI = i;
                walUnlockExclusive(pWal, (3 as i32) + i, 1 as i32);
                break '__slate_break_983;
            } else {
                if rc != (5 as i32) {
                    return rc;
                }
            }
            let __v1154: i32 = i;
            let __v1155: i32 = __v1154 + (1 as i32);
            i = __v1155;
        }
    }
    if mxI == (0 as i32) {
        0 as i32;
        return if rc == (5 as i32) {
            -(1 as i32)
        } else {
            (8 as i32) | (5 as i32) << (8 as i32)
        };
    }
    0 as i32;
    rc = walLockShared(pWal, (3 as i32) + mxI);
    {}
    if rc != (0 as i32) {
        0 as i32;
        0 as i32;
        return if rc & (255 as i32) == (5 as i32) {
            -(1 as i32)
        } else {
            rc
        };
    }
    // Now that the read-lock has been obtained, check that neither the
    // value in the aReadMark[] array or the contents of the wal-index
    // header have changed.
    //
    // It is necessary to check that the wal-index header did not change
    // between the time it was read and when the shared-lock was obtained
    // on WAL_READ_LOCK(mxI) was obtained to account for the possibility
    // that the log file may have been wrapped by a writer, or that frames
    // that occur later in the log than pWal->hdr.mxFrame may have been
    // copied into the database by a checkpointer. If either of these things
    // happened, then reading the database with the current value of
    // pWal->hdr.mxFrame risks reading a corrupted snapshot. So, retry
    // instead.
    //
    // Before checking that the live wal-index header has not changed
    // since it was read, set Wal.minFrame to the first frame in the wal
    // file that has not yet been checkpointed. This client will not need
    // to read any frames earlier than minFrame from the wal file - they
    // can be safely read directly from the database file.
    //
    // Because a ShmBarrier() call is made between taking the copy of
    // nBackfill and checking that the wal-header in shared-memory still
    // matches the one cached in pWal->hdr, it is guaranteed that the
    // checkpointer that set nBackfill was not working with a wal-index
    // header newer than that cached in pWal->hdr. If it were, that could
    // cause a problem. The checkpointer could omit to checkpoint
    // a version of page X that lies before pWal->minFrame (call that version
    // A) on the basis that there is a newer version (version B) of the same
    // page later in the wal file. But if version B happens to like past
    // frame pWal->hdr.mxFrame - then the client would incorrectly assume
    // that it can read version A from the database file. However, since
    // we can guarantee that the checkpointer that set nBackfill could not
    // see any pages past pWal->hdr.mxFrame, this problem does not come up.
    unsafe {
        (*pWal).minFrame = unsafe {
            std::sync::atomic::AtomicU32::load_volatile(
                std::sync::atomic::AtomicU32::from_ptr_raw(
                    (unsafe { std::ptr::addr_of_mut!((*pInfo).nBackfill) }) as *mut u32,
                ),
                std::sync::atomic::Ordering::Relaxed,
            )
        }
        .wrapping_add((1 as i32) as u32);
    }
    0 as i32;
    {}
    walShmBarrier(pWal);
    let __v1156: bool;
    if (unsafe {
        std::sync::atomic::AtomicU32::load_volatile(
            std::sync::atomic::AtomicU32::from_ptr_raw(
                (unsafe {
                    unsafe { (*pInfo).aReadMark.as_mut_ptr() as *mut u32 }.offset(mxI as isize)
                }) as *mut u32,
            ),
            std::sync::atomic::Ordering::Relaxed,
        )
    }) != mxReadMark
    {
        __v1156 = true as bool;
    } else {
        __v1156 = (unsafe {
            memcmp(
                (walIndexHdr(pWal) as *mut ()) as *const (),
                (unsafe { std::ptr::addr_of_mut!((*pWal).hdr) }) as *const (),
                48 as u64,
            )
        }) != (0 as i32);
    }
    if __v1156 {
        walUnlockShared(pWal, (3 as i32) + mxI);
        return -(1 as i32);
    } else {
        0 as i32;
        unsafe {
            (*pWal).readLock = mxI as i16;
        }
    }
    return rc;
}

/// This function does the work of sqlite3WalBeginReadTransaction() (see
/// below). That function simply calls this one inside an SEH_TRY{...} block.
fn walBeginReadTransaction(mut pWal: *mut Wal, mut pChanged: *mut i32) -> i32 {
    let mut rc: i32 = 0 as i32; // Return code
    let mut cnt: i32 = 0 as i32; // Number of TryBeginRead attempts
    0 as i32;
    0 as i32;
    '__slate_break_984: loop {
        rc = walTryBeginRead(pWal, pChanged, 0 as i32, std::ptr::addr_of_mut!(cnt));
        if !(rc == -(1 as i32)) {
            break;
        }
    }
    {}
    {}
    {}
    {}
    return rc;
}

/// Begin a read transaction on the database.
///
/// This routine used to be called sqlite3OpenSnapshot() and with good reason:
/// it takes a snapshot of the state of the WAL and wal-index for the current
/// instant in time.  The current thread will continue to use this snapshot.
/// Other threads might append new content to the WAL and wal-index but
/// that extra content is ignored by the current thread.
///
/// If the database contents have changes since the previous read
/// transaction, then *pChanged is set to 1 before returning.  The
/// Pager layer will use this to know that its cache is stale and
/// needs to be flushed.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3WalBeginReadTransaction(
    mut pWal: *mut Wal,
    mut pChanged: *mut i32,
) -> i32 {
    let mut rc: i32 = 0 as i32;
    {}
    rc = walBeginReadTransaction(pWal, pChanged);
    {}
    0 as i32;
    return rc;
}

/// Finish with a read transaction.  All this does is release the
/// read-lock.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3WalEndReadTransaction(mut pWal: *mut Wal) {
    0 as i32;
    if ((unsafe { (*pWal).readLock }) as i32) >= (0 as i32) {
        sqlite3WalEndWriteTransaction(pWal);
        walUnlockShared(pWal, (3 as i32) + ((unsafe { (*pWal).readLock }) as i32));
        unsafe {
            (*pWal).readLock = -(1 as i32) as i16;
        }
    }
}

/// Search the wal file for page pgno. If found, set *piRead to the frame that
/// contains the page. Otherwise, if pgno is not in the wal file, set *piRead
/// to zero.
///
/// Return SQLITE_OK if successful, or an error code if an error occurs. If an
/// error does occur, the final value of *piRead is undefined.
///
/// # Arguments
///
/// * `pWal` - WAL handle
/// * `pgno` - Database page number to read data for
/// * `piRead` - OUT: Frame number (or zero)
fn walFindFrame(mut pWal: *mut Wal, mut pgno: u32, mut piRead: *mut u32) -> i32 {
    let mut iRead: u32 = (0 as i32) as u32; // If !=0, WAL frame to return data from
    let mut iLast: u32 = unsafe { (*pWal).hdr.mxFrame }; // Last page in WAL for this reader
    let mut iHash: i32 = 0 as i32; // Used to loop through N hash tables
    let mut iMinHash: i32 = 0 as i32;
    // This routine is only be called from within a read transaction.
    0 as i32;
    // If the "last page" field of the wal-index header snapshot is 0, then
    // no data will be read from the wal under any circumstances. Return early
    // in this case as an optimization.  Likewise, if pWal->readLock==0,
    // then the WAL is ignored by the reader so return early, as if the
    // WAL were empty.
    if iLast == ((0 as i32) as u32)
        || ((unsafe { (*pWal).readLock }) as i32) == (0 as i32)
            && (((unsafe { (*pWal).bShmUnreliable }) as u32) as i32) == (0 as i32)
    {
        unsafe {
            *piRead = (0 as i32) as u32;
        }
        return 0 as i32;
    }
    // Search the hash table or tables for an entry matching page number
    // pgno. Each iteration of the following for() loop searches one
    // hash table (each hash table indexes up to HASHTABLE_NPAGE frames).
    //
    // This code might run concurrently to the code in walIndexAppend()
    // that adds entries to the wal-index (and possibly to this hash
    // table). This means the value just read from the hash
    // slot (aHash[iKey]) may have been added before or after the
    // current read transaction was opened. Values added after the
    // read transaction was opened may have been written incorrectly -
    // i.e. these slots may contain garbage data. However, we assume
    // that any slots written before the current read transaction was
    // opened remain unmodified.
    //
    // For the reasons above, the if(...) condition featured in the inner
    // loop of the following block is more stringent that would be required
    // if we had exclusive access to the hash-table:
    //
    //   (aPgno[iFrame]==pgno):
    //     This condition filters out normal hash-table collisions.
    //
    //   (iFrame<=iLast):
    //     This condition filters out entries that were added to the hash
    //     table after the current read-transaction had started.
    iMinHash = walFramePage(unsafe { (*pWal).minFrame });
    iHash = walFramePage(iLast);
    '__slate_break_985: loop {
        if !(iHash >= iMinHash) {
            break;
        }
        let mut sLoc: WalHashLoc = unsafe { std::mem::zeroed() }; // Hash table location
        let mut iKey: i32 = 0 as i32; // Hash slot index
        let mut nCollide: i32 = 0 as i32; // Number of hash collisions remaining
        let mut rc: i32 = 0 as i32; // Error code
        let mut iH: u32 = 0 as u32;
        rc = walHashGet(pWal, iHash, std::ptr::addr_of_mut!(sLoc));
        if rc != (0 as i32) {
            return rc;
        }
        nCollide = (4096 as i32) * (2 as i32);
        iKey = walHash(pgno);
        0 as i32;
        {}
        '__slate_break_986: loop {
            let __v1159: u32 = (unsafe {
                std::sync::atomic::AtomicU16::load_volatile(
                    std::sync::atomic::AtomicU16::from_ptr_raw(
                        (unsafe { sLoc.aHash.offset(iKey as isize) }) as *mut u16,
                    ),
                    std::sync::atomic::Ordering::Relaxed,
                )
            }) as u32;
            iH = __v1159;
            if !(__v1159 != ((0 as i32) as u32)) {
                break;
            }
            let mut iFrame: u32 = iH.wrapping_add(sLoc.iZero);
            if iFrame <= iLast
                && iFrame >= unsafe { (*pWal).minFrame }
                && (unsafe {
                    std::ptr::read_volatile(unsafe {
                        sLoc.aPgno.offset(
                            (iH.wrapping_sub((1 as i32) as u32)
                                & (((4096 as i32) - (1 as i32)) as u32))
                                as isize,
                        )
                    })
                }) == pgno
            {
                0 as i32;
                iRead = iFrame;
            }
            let __v1160: i32 = nCollide;
            let __v1161: i32 = __v1160 - (1 as i32);
            nCollide = __v1161;
            if __v1160 == (0 as i32) {
                unsafe {
                    *piRead = (0 as i32) as u32;
                }
                return unsafe { sqlite3CorruptError(3600 as i32) };
            }
            iKey = walNextHash(iKey);
        }
        if iRead != (0 as u32) {
            break '__slate_break_985;
        }
        let __v1157: i32 = iHash;
        let __v1158: i32 = __v1157 - (1 as i32);
        iHash = __v1158;
    }
    unsafe {
        *piRead = iRead;
    }
    return 0 as i32;
}

/// Search the wal file for page pgno. If found, set *piRead to the frame that
/// contains the page. Otherwise, if pgno is not in the wal file, set *piRead
/// to zero.
///
/// Return SQLITE_OK if successful, or an error code if an error occurs. If an
/// error does occur, the final value of *piRead is undefined.
///
/// The difference between this function and walFindFrame() is that this
/// function wraps walFindFrame() in an SEH_TRY{...} block.
///
/// # Arguments
///
/// * `pWal` - WAL handle
/// * `pgno` - Database page number to read data for
/// * `piRead` - OUT: Frame number (or zero)
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3WalFindFrame(
    mut pWal: *mut Wal,
    mut pgno: u32,
    mut piRead: *mut u32,
) -> i32 {
    let mut rc: i32 = 0 as i32;
    {}
    rc = walFindFrame(pWal, pgno, piRead);
    {}
    0 as i32;
    return rc;
}

/// Read the contents of frame iRead from the wal file into buffer pOut
/// (which is nOut bytes in size). Return SQLITE_OK if successful, or an
/// error code otherwise.
///
/// # Arguments
///
/// * `pWal` - WAL handle
/// * `iRead` - Frame to read
/// * `nOut` - Size of buffer pOut in bytes
/// * `pOut` - Buffer to write page data to
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3WalReadFrame(
    mut pWal: *mut Wal,
    mut iRead: u32,
    mut nOut: i32,
    mut pOut: *mut u8,
) -> i32 {
    let mut sz: i32 = 0 as i32;
    let mut iOffset: i64 = 0 as i64;
    sz = ((unsafe { (*pWal).hdr.szPage }) as u32) as i32;
    sz = (sz & (65024 as i32)) + ((sz & (1 as i32)) << (16 as i32));
    if nOut > sz {
        unsafe {
            memset(
                (unsafe { pOut.offset(sz as isize) }) as *mut (),
                0 as i32,
                ((nOut - sz) as i64) as u64,
            )
        };
        nOut = sz;
    }
    {}
    {}
    iOffset = ((32 as i32) as i64)
        + ((iRead.wrapping_sub((1 as i32) as u32) as u64) as i64) * ((sz + (24 as i32)) as i64)
        + ((24 as i32) as i64);
    // testcase( IS_BIG_INT(iOffset) ); // requires a 4GiB WAL
    return unsafe { sqlite3OsRead(unsafe { (*pWal).pWalFd }, pOut as *mut (), nOut, iOffset) };
}

/// Return the size of the database in pages (or zero, if unknown).
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3WalDbsize(mut pWal: *mut Wal) -> u32 {
    if pWal != std::ptr::null_mut::<Wal>() && ((unsafe { (*pWal).readLock }) as i32) >= (0 as i32) {
        return unsafe { (*pWal).hdr.nPage };
    }
    return (0 as i32) as u32;
}

/// This function starts a write transaction on the WAL.
///
/// A read transaction must have already been started by a prior call
/// to sqlite3WalBeginReadTransaction().
///
/// If another thread or process has written into the database since
/// the read transaction was started, then it is not possible for this
/// thread to write as doing so would cause a fork.  So this routine
/// returns SQLITE_BUSY in that case and no write transaction is started.
///
/// There can only be a single writer active at a time.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3WalBeginWriteTransaction(mut pWal: *mut Wal) -> i32 {
    let mut rc: i32 = 0 as i32;
    // Cannot start a write transaction without first holding a read
    // transaction.
    0 as i32;
    0 as i32;
    if (unsafe { (*pWal).readOnly }) != (0 as u8) {
        return 8 as i32;
    }
    // Only one writer allowed at a time.  Get the write lock.  Return
    // SQLITE_BUSY if unable.
    rc = walLockExclusive(pWal, 0 as i32, 1 as i32);
    if rc != (0 as i32) {
        return rc;
    }
    unsafe {
        (*pWal).writeLock = ((1 as i32) as i8) as u8;
    }
    // If another connection has written to the database file since the
    // time the read transaction on this connection was started, then
    // the write is disallowed.
    {}
    if (unsafe {
        memcmp(
            (unsafe { std::ptr::addr_of_mut!((*pWal).hdr) }) as *const (),
            (walIndexHdr(pWal) as *mut ()) as *const (),
            48 as u64,
        )
    }) != (0 as i32)
    {
        rc = (5 as i32) | (2 as i32) << (8 as i32);
    }
    {}
    0 as i32;
    if rc != (0 as i32) {
        walUnlockExclusive(pWal, 0 as i32, 1 as i32);
        unsafe {
            (*pWal).writeLock = ((0 as i32) as i8) as u8;
        }
    }
    return rc;
}

/// End a write transaction.  The commit has already been done.  This
/// routine merely releases the lock.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3WalEndWriteTransaction(mut pWal: *mut Wal) -> i32 {
    if (unsafe { (*pWal).writeLock }) != (0 as u8) {
        walUnlockExclusive(pWal, 0 as i32, 1 as i32);
        unsafe {
            (*pWal).writeLock = ((0 as i32) as i8) as u8;
        }
        unsafe {
            (*pWal).iReCksum = (0 as i32) as u32;
        }
        unsafe {
            (*pWal).truncateOnCommit = ((0 as i32) as i8) as u8;
        }
    }
    return 0 as i32;
}

/// If any data has been written (but not committed) to the log file, this
/// function moves the write-pointer back to the start of the transaction.
///
/// Additionally, the callback function is invoked for each frame written
/// to the WAL since the start of the transaction. If the callback returns
/// other than SQLITE_OK, it is not invoked again and the error code is
/// returned to the caller.
///
/// Otherwise, if the callback function does not return an error, this
/// function returns SQLITE_OK.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3WalUndo(
    mut pWal: *mut Wal,
    mut xUndo: Option<unsafe extern "C-unwind" fn(*mut (), u32) -> i32>,
    mut pUndoCtx: *mut (),
) -> i32 {
    let mut rc: i32 = 0 as i32;
    if (unsafe { (*pWal).writeLock }) != (0 as u8) {
        let mut iMax: u32 = unsafe { (*pWal).hdr.mxFrame };
        let mut iFrame: u32 = 0 as u32;
        {}
        // Restore the clients cache of the wal-index header to the state it
        // was in before the client began writing to the database.
        unsafe {
            memcpy(
                (unsafe { std::ptr::addr_of_mut!((*pWal).hdr) }) as *mut (),
                (walIndexHdr(pWal) as *mut ()) as *const (),
                48 as u64,
            )
        };
        iFrame = unsafe { (*pWal).hdr.mxFrame }.wrapping_add((1 as i32) as u32);
        '__slate_break_987: while rc == (0 as i32) && iFrame <= iMax {
            // This call cannot fail. Unless the page for which the page number
            // is passed as the second argument is (a) in the cache and
            // (b) has an outstanding reference, then xUndo is either a no-op
            // (if (a) is false) or simply expels the page from the cache (if (b)
            // is false).
            //
            // If the upper layer is doing a rollback, it is guaranteed that there
            // are no outstanding references to any page other than page 1. And
            // page 1 is never written to the log until the transaction is
            // committed. As a result, the call to xUndo may not fail.
            0 as i32;
            rc = unsafe { xUndo.unwrap()(pUndoCtx, walFramePgno(pWal, iFrame)) };
            let __v996: u32 = iFrame;
            let __v997: u32 = __v996.wrapping_add((1 as i32) as u32);
            iFrame = __v997;
        }
        if iMax != unsafe { (*pWal).hdr.mxFrame } {
            walCleanupHash(pWal);
        }
        {}
        0 as i32;
        unsafe {
            (*pWal).iReCksum = (0 as i32) as u32;
        }
    }
    return rc;
}

/// Argument aWalData must point to an array of WAL_SAVEPOINT_NDATA u32
/// values. This function populates the array with values required to
/// "rollback" the write position of the WAL handle back to the current
/// point in the event of a savepoint rollback (via WalSavepointUndo()).
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3WalSavepoint(mut pWal: *mut Wal, mut aWalData: *mut u32) {
    0 as i32;
    unsafe {
        *unsafe { aWalData.offset((0 as i32) as isize) } = unsafe { (*pWal).hdr.mxFrame };
    }
    unsafe {
        *unsafe { aWalData.offset((1 as i32) as isize) } = unsafe {
            *unsafe {
                unsafe { (*pWal).hdr.aFrameCksum.as_mut_ptr() as *mut u32 }
                    .offset((0 as i32) as isize)
            }
        };
    }
    unsafe {
        *unsafe { aWalData.offset((2 as i32) as isize) } = unsafe {
            *unsafe {
                unsafe { (*pWal).hdr.aFrameCksum.as_mut_ptr() as *mut u32 }
                    .offset((1 as i32) as isize)
            }
        };
    }
    unsafe {
        *unsafe { aWalData.offset((3 as i32) as isize) } = unsafe { (*pWal).nCkpt };
    }
}

/// Move the write position of the WAL back to the point identified by
/// the values in the aWalData[] array. aWalData must point to an array
/// of WAL_SAVEPOINT_NDATA u32 values that has been previously populated
/// by a call to WalSavepoint().
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3WalSavepointUndo(mut pWal: *mut Wal, mut aWalData: *mut u32) -> i32 {
    let mut rc: i32 = 0 as i32;
    0 as i32;
    0 as i32;
    if (unsafe { *unsafe { aWalData.offset((3 as i32) as isize) } }) != unsafe { (*pWal).nCkpt } {
        // This savepoint was opened immediately after the write-transaction
        // was started. Right after that, the writer decided to wrap around
        // to the start of the log. Update the savepoint values to match.
        unsafe {
            *unsafe { aWalData.offset((0 as i32) as isize) } = (0 as i32) as u32;
        }
        unsafe {
            *unsafe { aWalData.offset((3 as i32) as isize) } = unsafe { (*pWal).nCkpt };
        }
    }
    if (unsafe { *unsafe { aWalData.offset((0 as i32) as isize) } })
        < unsafe { (*pWal).hdr.mxFrame }
    {
        unsafe {
            (*pWal).hdr.mxFrame = unsafe { *unsafe { aWalData.offset((0 as i32) as isize) } };
        }
        unsafe {
            *unsafe {
                unsafe { (*pWal).hdr.aFrameCksum.as_mut_ptr() as *mut u32 }
                    .offset((0 as i32) as isize)
            } = unsafe { *unsafe { aWalData.offset((1 as i32) as isize) } };
        }
        unsafe {
            *unsafe {
                unsafe { (*pWal).hdr.aFrameCksum.as_mut_ptr() as *mut u32 }
                    .offset((1 as i32) as isize)
            } = unsafe { *unsafe { aWalData.offset((2 as i32) as isize) } };
        }
        {}
        walCleanupHash(pWal);
        {}
        0 as i32;
        if (unsafe { (*pWal).iReCksum }) > unsafe { (*pWal).hdr.mxFrame } {
            unsafe {
                (*pWal).iReCksum = (0 as i32) as u32;
            }
        }
    }
    return rc;
}

/// This function is called just before writing a set of frames to the log
/// file (see sqlite3WalFrames()). It checks to see if, instead of appending
/// to the current log file, it is possible to overwrite the start of the
/// existing log file with the new frames (i.e. "reset" the log). If so,
/// it sets pWal->hdr.mxFrame to 0. Otherwise, pWal->hdr.mxFrame is left
/// unchanged.
///
/// SQLITE_OK is returned if no error is encountered (regardless of whether
/// or not pWal->hdr.mxFrame is modified). An SQLite error code is returned
/// if an error occurs.
fn walRestartLog(mut pWal: *mut Wal) -> i32 {
    let mut rc: i32 = 0 as i32;
    let mut cnt: i32 = 0 as i32;
    if ((unsafe { (*pWal).readLock }) as i32) == (0 as i32) {
        let mut pInfo: *mut WalCkptInfo = walCkptInfo(pWal);
        0 as i32;
        if (unsafe { std::ptr::read_volatile(std::ptr::addr_of!((*pInfo).nBackfill)) })
            > ((0 as i32) as u32)
        {
            let mut salt1: u32 = 0 as u32;
            unsafe { sqlite3_randomness(4 as i32, std::ptr::addr_of_mut!(salt1) as *mut ()) };
            rc = walLockExclusive(
                pWal,
                (3 as i32) + (1 as i32),
                (8 as i32) - (3 as i32) - (1 as i32),
            );
            if rc == (0 as i32) {
                // If all readers are using WAL_READ_LOCK(0) (in other words if no
                // readers are currently using the WAL), then the transactions
                // frames will overwrite the start of the existing log. Update the
                // wal-index header to reflect this.
                //
                // In theory it would be Ok to update the cache of the header only
                // at this point. But updating the actual wal-index header is also
                // safe and means there is no special case for sqlite3WalUndo()
                // to handle if this transaction is rolled back.
                walRestartHdr(pWal, salt1);
                walUnlockExclusive(
                    pWal,
                    (3 as i32) + (1 as i32),
                    (8 as i32) - (3 as i32) - (1 as i32),
                );
            } else {
                if rc != (5 as i32) {
                    return rc;
                }
            }
        }
        walUnlockShared(pWal, (3 as i32) + (0 as i32));
        unsafe {
            (*pWal).readLock = -(1 as i32) as i16;
        }
        cnt = 0 as i32;
        '__slate_break_988: loop {
            let mut notUsed: i32 = 0 as i32;
            rc = walTryBeginRead(
                pWal,
                std::ptr::addr_of_mut!(notUsed),
                1 as i32,
                std::ptr::addr_of_mut!(cnt),
            );
            if !(rc == -(1 as i32)) {
                break;
            }
        }
        0 as i32; // BUSY not possible when useWal==1
        {}
        {}
        {}
    }
    return rc;
}

/// Information about the current state of the WAL file and where
/// the next fsync should occur - passed from sqlite3WalFrames() into
/// walWriteToLog().
#[repr(C)]
#[derive(Clone, Copy)]
struct WalWriter {
    /// The complete WAL information
    pWal: *mut Wal,
    /// The WAL file to which we write
    pFd: *mut sqlite3_file,
    /// Fsync at this offset
    iSyncPoint: i64,
    /// Flags for the fsync
    syncFlags: i32,
    /// Size of one page
    szPage: i32,
}

/// Write iAmt bytes of content into the WAL file beginning at iOffset.
/// Do a sync when crossing the p->iSyncPoint boundary.
///
/// In other words, if iSyncPoint is in between iOffset and iOffset+iAmt,
/// first write the part before iSyncPoint, then sync, then write the
/// rest.
///
/// # Arguments
///
/// * `p` - WAL to write to
/// * `pContent` - Content to be written
/// * `iAmt` - Number of bytes to write
/// * `iOffset` - Start writing at this offset
fn walWriteToLog(
    mut p: *mut WalWriter,
    mut pContent: *mut (),
    mut iAmt: i32,
    mut iOffset: i64,
) -> i32 {
    let mut rc: i32 = 0 as i32;
    if iOffset < unsafe { (*p).iSyncPoint } && iOffset + (iAmt as i64) >= unsafe { (*p).iSyncPoint }
    {
        let mut iFirstAmt: i32 = ((unsafe { (*p).iSyncPoint }) - iOffset) as i32;
        rc = unsafe {
            sqlite3OsWrite(
                unsafe { (*p).pFd },
                pContent as *const (),
                iFirstAmt,
                iOffset,
            )
        };
        if rc != (0 as i32) {
            return rc;
        }
        let __v1162: i64 = iOffset;
        let __v1163: i64 = __v1162 + (iFirstAmt as i64);
        iOffset = __v1163;
        let __v1164: i32 = iAmt;
        let __v1165: i32 = __v1164 - iFirstAmt;
        iAmt = __v1165;
        pContent = (unsafe { (pContent as *mut i8).offset(iFirstAmt as isize) }) as *mut ();
        0 as i32;
        rc = unsafe {
            sqlite3OsSync(
                unsafe { (*p).pFd },
                (unsafe { (*p).syncFlags }) & (3 as i32),
            )
        };
        if iAmt == (0 as i32) || rc != (0 as i32) {
            return rc;
        }
    }
    rc = unsafe { sqlite3OsWrite(unsafe { (*p).pFd }, pContent as *const (), iAmt, iOffset) };
    return rc;
}

/// Write out a single frame of the WAL
///
/// # Arguments
///
/// * `p` - Where to write the frame
/// * `pPage` - The page of the frame to be written
/// * `nTruncate` - The commit flag.  Usually 0.  >0 for commit
/// * `iOffset` - Byte offset at which to write
fn walWriteOneFrame(
    mut p: *mut WalWriter,
    mut pPage: *mut PgHdr,
    mut nTruncate: i32,
    mut iOffset: i64,
) -> i32 {
    let mut rc: i32 = 0 as i32; // Result code from subfunctions
    let mut pData: *mut () = unsafe { std::mem::zeroed() }; // Data actually written
    let mut aFrame: __SlateAlign16<[u8; 24]> = __SlateAlign16([0 as u8; 24]); // Buffer to assemble frame-header in
    pData = unsafe { (*pPage).pData };
    walEncodeFrame(
        unsafe { (*p).pWal },
        unsafe { (*pPage).pgno },
        nTruncate as u32,
        pData as *mut u8,
        aFrame.0.as_mut_ptr() as *mut u8,
    );
    rc = walWriteToLog(
        p,
        (aFrame.0.as_mut_ptr() as *mut u8) as *mut (),
        ((24 as u64) as u32) as i32,
        iOffset,
    );
    if rc != (0 as i32) {
        return rc;
    }
    // Write the page data
    rc = walWriteToLog(
        p,
        pData,
        unsafe { (*p).szPage },
        (iOffset as u64).wrapping_add(24 as u64) as i64,
    );
    return rc;
}

/// This function is called as part of committing a transaction within which
/// one or more frames have been overwritten. It updates the checksums for
/// all frames written to the wal file by the current transaction starting
/// with the earliest to have been overwritten.
///
/// SQLITE_OK is returned if successful, or an SQLite error code otherwise.
fn walRewriteChecksums(mut pWal: *mut Wal, mut iLast: u32) -> i32 {
    let mut szPage: i32 = (unsafe { (*pWal).szPage }) as i32; // Database page size
    let mut rc: i32 = 0 as i32; // Return code
    let mut aBuf: *mut u8 = unsafe { std::mem::zeroed() }; // Buffer to load data from wal file into
    let mut aFrame: __SlateAlign16<[u8; 24]> = __SlateAlign16([0 as u8; 24]); // Buffer to assemble frame-headers in
    let mut iRead: u32 = 0 as u32; // Next frame to read from wal file
    let mut iCksumOff: i64 = 0 as i64;
    aBuf = (unsafe { sqlite3_malloc(szPage + (24 as i32)) }) as *mut u8;
    if aBuf == std::ptr::null_mut::<u8>() {
        return 7 as i32;
    }
    // Find the checksum values to use as input for the recalculating the
    // first checksum. If the first frame is frame 1 (implying that the current
    // transaction restarted the wal file), these values must be read from the
    // wal-file header. Otherwise, read them from the frame header of the
    // previous frame.
    0 as i32;
    if (unsafe { (*pWal).iReCksum }) == ((1 as i32) as u32) {
        iCksumOff = (24 as i32) as i64;
    } else {
        iCksumOff = ((32 as i32) as i64)
            + ((unsafe { (*pWal).iReCksum }
                .wrapping_sub((1 as i32) as u32)
                .wrapping_sub((1 as i32) as u32) as u64) as i64)
                * ((szPage + (24 as i32)) as i64)
            + ((16 as i32) as i64);
    }
    rc = unsafe {
        sqlite3OsRead(
            unsafe { (*pWal).pWalFd },
            aBuf as *mut (),
            ((4 as u64).wrapping_mul(((2 as i32) as i64) as u64) as u32) as i32,
            iCksumOff,
        )
    };
    unsafe {
        *unsafe {
            unsafe { (*pWal).hdr.aFrameCksum.as_mut_ptr() as *mut u32 }.offset((0 as i32) as isize)
        } = unsafe { sqlite3Get4byte(aBuf as *const u8) };
    }
    unsafe {
        *unsafe {
            unsafe { (*pWal).hdr.aFrameCksum.as_mut_ptr() as *mut u32 }.offset((1 as i32) as isize)
        } = unsafe { sqlite3Get4byte((unsafe { aBuf.offset((4 as u64) as isize) }) as *const u8) };
    }
    iRead = unsafe { (*pWal).iReCksum };
    unsafe {
        (*pWal).iReCksum = (0 as i32) as u32;
    }
    '__slate_break_989: while rc == (0 as i32) && iRead <= iLast {
        let mut iOff: i64 = ((32 as i32) as i64)
            + ((iRead.wrapping_sub((1 as i32) as u32) as u64) as i64)
                * ((szPage + (24 as i32)) as i64);
        rc = unsafe {
            sqlite3OsRead(
                unsafe { (*pWal).pWalFd },
                aBuf as *mut (),
                szPage + (24 as i32),
                iOff,
            )
        };
        if rc == (0 as i32) {
            let mut iPgno: u32 = 0 as u32;
            let mut nDbSize: u32 = 0 as u32;
            iPgno = unsafe { sqlite3Get4byte(aBuf as *const u8) };
            nDbSize = unsafe {
                sqlite3Get4byte((unsafe { aBuf.offset((4 as i32) as isize) }) as *const u8)
            };
            walEncodeFrame(
                pWal,
                iPgno,
                nDbSize,
                unsafe { aBuf.offset((24 as i32) as isize) },
                aFrame.0.as_mut_ptr() as *mut u8,
            );
            rc = unsafe {
                sqlite3OsWrite(
                    unsafe { (*pWal).pWalFd },
                    (aFrame.0.as_mut_ptr() as *mut u8) as *const (),
                    ((24 as u64) as u32) as i32,
                    iOff,
                )
            };
        }
        let __v1166: u32 = iRead;
        let __v1167: u32 = __v1166.wrapping_add((1 as i32) as u32);
        iRead = __v1167;
    }
    unsafe { sqlite3_free(aBuf as *mut ()) };
    return rc;
}

/// Write a set of frames to the log. The caller must hold the write-lock
/// on the log file (obtained using sqlite3WalBeginWriteTransaction()).
///
/// # Arguments
///
/// * `pWal` - Wal handle to write to
/// * `szPage` - Database page-size in bytes
/// * `pList` - List of dirty pages to write
/// * `nTruncate` - Database size after this commit
/// * `isCommit` - True if this is a commit
/// * `sync_flags` - Flags to pass to OsSync() (or 0)
fn walFrames(
    mut pWal: *mut Wal,
    mut szPage: i32,
    mut pList: *mut PgHdr,
    mut nTruncate: u32,
    mut isCommit: i32,
    mut sync_flags: i32,
) -> i32 {
    let mut rc: i32 = 0 as i32; // Used to catch return codes
    let mut iFrame: u32 = 0 as u32; // Next frame address
    let mut p: *mut PgHdr = unsafe { std::mem::zeroed() }; // Iterator to run through pList with.
    let mut pLast: *mut PgHdr = std::ptr::null_mut::<PgHdr>(); // Last frame in list
    let mut nExtra: i32 = 0 as i32; // Number of extra copies of last page
    let mut szFrame: i32 = 0 as i32; // The size of a single frame
    let mut iOffset: i64 = 0 as i64; // Next byte to write in WAL file
    let mut w: WalWriter = unsafe { std::mem::zeroed() }; // The writer
    let mut iFirst: u32 = (0 as i32) as u32; // First frame that may be overwritten
    let mut pLive: *mut WalIndexHdr = unsafe { std::mem::zeroed() }; // Pointer to shared header
    0 as i32;
    0 as i32;
    // If this frame set completes a transaction, then nTruncate>0.  If
    // nTruncate==0 then this frame set does not complete the transaction.
    0 as i32;
    pLive = walIndexHdr(pWal) as *mut WalIndexHdr;
    if (unsafe {
        memcmp(
            (unsafe { std::ptr::addr_of_mut!((*pWal).hdr) }) as *const (),
            (pLive as *mut ()) as *const (),
            48 as u64,
        )
    }) != (0 as i32)
    {
        iFirst = unsafe { (*pLive).mxFrame }.wrapping_add((1 as i32) as u32);
    }
    // See if it is possible to write these frames into the start of the
    // log file, instead of appending to it at pWal->hdr.mxFrame.
    let __v1168: i32 = walRestartLog(pWal);
    rc = __v1168;
    if (0 as i32) != __v1168 {
        return rc;
    }
    // If this is the first frame written into the log, write the WAL
    // header to the start of the WAL file. See comments at the top of
    // this source file for a description of the WAL header format.
    iFrame = unsafe { (*pWal).hdr.mxFrame };
    if iFrame == ((0 as i32) as u32) {
        let mut aWalHdr: __SlateAlign16<[u8; 32]> = __SlateAlign16([0 as u8; 32]); // Buffer to assemble wal-header in
        let mut aCksum: [u32; 2] = [0 as u32; 2]; // Checksum for wal-header
        unsafe {
            sqlite3Put4byte(
                unsafe { (aWalHdr.0.as_mut_ptr() as *mut u8).offset((0 as i32) as isize) },
                ((931071618 as i32) | (0 as i32)) as u32,
            )
        };
        unsafe {
            sqlite3Put4byte(
                unsafe { (aWalHdr.0.as_mut_ptr() as *mut u8).offset((4 as i32) as isize) },
                (3007000 as i32) as u32,
            )
        };
        unsafe {
            sqlite3Put4byte(
                unsafe { (aWalHdr.0.as_mut_ptr() as *mut u8).offset((8 as i32) as isize) },
                szPage as u32,
            )
        };
        unsafe {
            sqlite3Put4byte(
                unsafe { (aWalHdr.0.as_mut_ptr() as *mut u8).offset((12 as i32) as isize) },
                unsafe { (*pWal).nCkpt },
            )
        };
        if (unsafe { (*pWal).nCkpt }) == ((0 as i32) as u32) {
            unsafe {
                sqlite3_randomness(
                    8 as i32,
                    (unsafe { (*pWal).hdr.aSalt.as_mut_ptr() as *mut u32 }) as *mut (),
                )
            };
        }
        unsafe {
            memcpy(
                (unsafe { (aWalHdr.0.as_mut_ptr() as *mut u8).offset((16 as i32) as isize) })
                    as *mut (),
                (unsafe { (*pWal).hdr.aSalt.as_mut_ptr() as *mut u32 }) as *const (),
                ((8 as i32) as i64) as u64,
            )
        };
        walChecksumBytes(
            1 as i32,
            aWalHdr.0.as_mut_ptr() as *mut u8,
            (32 as i32) - (2 as i32) * (4 as i32),
            std::ptr::null::<u32>(),
            aCksum.as_mut_ptr() as *mut u32,
        );
        unsafe {
            sqlite3Put4byte(
                unsafe { (aWalHdr.0.as_mut_ptr() as *mut u8).offset((24 as i32) as isize) },
                unsafe {
                    *unsafe { (aCksum.as_mut_ptr() as *mut u32).offset((0 as i32) as isize) }
                },
            )
        };
        unsafe {
            sqlite3Put4byte(
                unsafe { (aWalHdr.0.as_mut_ptr() as *mut u8).offset((28 as i32) as isize) },
                unsafe {
                    *unsafe { (aCksum.as_mut_ptr() as *mut u32).offset((1 as i32) as isize) }
                },
            )
        };
        unsafe {
            (*pWal).szPage = szPage as u32;
        }
        unsafe {
            (*pWal).hdr.bigEndCksum = ((0 as i32) as i8) as u8;
        }
        unsafe {
            *unsafe {
                unsafe { (*pWal).hdr.aFrameCksum.as_mut_ptr() as *mut u32 }
                    .offset((0 as i32) as isize)
            } = unsafe {
                *unsafe { (aCksum.as_mut_ptr() as *mut u32).offset((0 as i32) as isize) }
            };
        }
        unsafe {
            *unsafe {
                unsafe { (*pWal).hdr.aFrameCksum.as_mut_ptr() as *mut u32 }
                    .offset((1 as i32) as isize)
            } = unsafe {
                *unsafe { (aCksum.as_mut_ptr() as *mut u32).offset((1 as i32) as isize) }
            };
        }
        unsafe {
            (*pWal).truncateOnCommit = ((1 as i32) as i8) as u8;
        }
        rc = unsafe {
            sqlite3OsWrite(
                unsafe { (*pWal).pWalFd },
                (aWalHdr.0.as_mut_ptr() as *mut u8) as *const (),
                ((32 as u64) as u32) as i32,
                (0 as i32) as i64,
            )
        };
        {}
        if rc != (0 as i32) {
            return rc;
        }
        // Sync the header (unless SQLITE_IOCAP_SEQUENTIAL is true or unless
        // all syncing is turned off by PRAGMA synchronous=OFF).  Otherwise
        // an out-of-order write following a WAL restart could result in
        // database corruption.  See the ticket:
        //
        //     https://sqlite.org/src/info/ff5be73dee
        if (unsafe { (*pWal).syncHeader }) != (0 as u8) {
            rc = unsafe {
                sqlite3OsSync(
                    unsafe { (*pWal).pWalFd },
                    sync_flags >> (2 as i32) & (3 as i32),
                )
            };
            if rc != (0 as i32) {
                return rc;
            }
        }
    }
    if ((unsafe { (*pWal).szPage }) as i32) != szPage {
        return unsafe { sqlite3CorruptError(4131 as i32) }; // TH3 test case: cov1/corrupt155.test
    }
    // Setup information needed to write frames into the WAL
    w.pWal = pWal;
    w.pFd = unsafe { (*pWal).pWalFd };
    w.iSyncPoint = (0 as i32) as i64;
    w.syncFlags = sync_flags;
    w.szPage = szPage;
    iOffset = ((32 as i32) as i64)
        + ((iFrame
            .wrapping_add((1 as i32) as u32)
            .wrapping_sub((1 as i32) as u32) as u64) as i64)
            * ((szPage + (24 as i32)) as i64);
    szFrame = szPage + (24 as i32);
    // Write all frames into the log file exactly once
    p = pList;
    '__slate_break_990: while p != std::ptr::null_mut::<PgHdr>() {
        '__slate_continue_990: {
            let mut nDbSize: i32 = 0 as i32; // 0 normally.  Positive == commit flag
            // Check if this page has already been written into the wal file by
            // the current transaction. If so, overwrite the existing frame and
            // set Wal.writeLock to WAL_WRITELOCK_RECKSUM - indicating that
            // checksums must be recomputed when the transaction is committed.
            if iFirst != (0 as u32)
                && ((unsafe { (*p).pDirty }) != std::ptr::null_mut::<PgHdr>()
                    || isCommit == (0 as i32))
            {
                let mut iWrite: u32 = (0 as i32) as u32;
                walFindFrame(pWal, unsafe { (*p).pgno }, std::ptr::addr_of_mut!(iWrite));
                0 as i32;
                if iWrite >= iFirst {
                    let mut iOff: i64 = ((32 as i32) as i64)
                        + ((iWrite.wrapping_sub((1 as i32) as u32) as u64) as i64)
                            * ((szPage + (24 as i32)) as i64)
                        + ((24 as i32) as i64);
                    let mut pData: *mut () = unsafe { std::mem::zeroed() };
                    if (unsafe { (*pWal).iReCksum }) == ((0 as i32) as u32)
                        || iWrite < unsafe { (*pWal).iReCksum }
                    {
                        unsafe {
                            (*pWal).iReCksum = iWrite;
                        }
                    }
                    pData = unsafe { (*p).pData };
                    rc = unsafe {
                        sqlite3OsWrite(unsafe { (*pWal).pWalFd }, pData as *const (), szPage, iOff)
                    };
                    if rc != (0 as i32) {
                        return rc;
                    }
                    let __v1169: *mut PgHdr = p;
                    let __v1170: u16 = unsafe { (*__v1169).flags };
                    let __v1171: u16 = ((((__v1170 as u32) as i32) & !(64 as i32)) as i16) as u16;
                    unsafe {
                        (*__v1169).flags = __v1171;
                    }
                    break '__slate_continue_990;
                }
            }
            let __v1172: u32 = iFrame;
            let __v1173: u32 = __v1172.wrapping_add((1 as i32) as u32);
            iFrame = __v1173;
            0 as i32;
            nDbSize = (if isCommit != (0 as i32)
                && (unsafe { (*p).pDirty }) == std::ptr::null_mut::<PgHdr>()
            {
                nTruncate
            } else {
                (0 as i32) as u32
            }) as i32;
            rc = walWriteOneFrame(std::ptr::addr_of_mut!(w), p, nDbSize, iOffset);
            if rc != (0 as i32) {
                return rc;
            }
            pLast = p;
            let __v1174: i64 = iOffset;
            let __v1175: i64 = __v1174 + (szFrame as i64);
            iOffset = __v1175;
            let __v1176: *mut PgHdr = p;
            let __v1177: u16 = unsafe { (*__v1176).flags };
            let __v1178: u16 = ((((__v1177 as u32) as i32) | (64 as i32)) as i16) as u16;
            unsafe {
                (*__v1176).flags = __v1178;
            }
        }
        p = unsafe { (*p).pDirty };
    }
    // Recalculate checksums within the wal file if required.
    if isCommit != (0 as i32) && (unsafe { (*pWal).iReCksum }) != (0 as u32) {
        rc = walRewriteChecksums(pWal, iFrame);
        if rc != (0 as i32) {
            return rc;
        }
    }
    // If this is the end of a transaction, then we might need to pad
    // the transaction and/or sync the WAL file.
    //
    // Padding and syncing only occur if this set of frames complete a
    // transaction and if PRAGMA synchronous=FULL.  If synchronous==NORMAL
    // or synchronous==OFF, then no padding or syncing are needed.
    //
    // If SQLITE_IOCAP_POWERSAFE_OVERWRITE is defined, then padding is not
    // needed and only the sync is done.  If padding is needed, then the
    // final frame is repeated (with its commit mark) until the next sector
    // boundary is crossed.  Only the part of the WAL prior to the last
    // sector boundary is synced; the part of the last frame that extends
    // past the sector boundary is written after the sync.
    if isCommit != (0 as i32) && sync_flags & (3 as i32) != (0 as i32) {
        let mut bSync: i32 = 1 as i32;
        if (unsafe { (*pWal).padToSectorBoundary }) != (0 as u8) {
            let mut sectorSize: i32 = unsafe { sqlite3SectorSize(unsafe { (*pWal).pWalFd }) };
            w.iSyncPoint = (iOffset + (sectorSize as i64) - ((1 as i32) as i64))
                / (sectorSize as i64)
                * (sectorSize as i64);
            bSync = (w.iSyncPoint == iOffset) as i32;
            {}
            '__slate_break_991: while iOffset < w.iSyncPoint {
                rc = walWriteOneFrame(std::ptr::addr_of_mut!(w), pLast, nTruncate as i32, iOffset);
                if rc != (0 as i32) {
                    return rc;
                }
                let __v1179: i64 = iOffset;
                let __v1180: i64 = __v1179 + (szFrame as i64);
                iOffset = __v1180;
                let __v1181: i32 = nExtra;
                let __v1182: i32 = __v1181 + (1 as i32);
                nExtra = __v1182;
                0 as i32;
            }
        }
        if bSync != (0 as i32) {
            0 as i32;
            rc = unsafe { sqlite3OsSync(w.pFd, sync_flags & (3 as i32)) };
        }
    }
    // If this frame set completes the first transaction in the WAL and
    // if PRAGMA journal_size_limit is set, then truncate the WAL to the
    // journal size limit, if possible.
    if isCommit != (0 as i32)
        && (unsafe { (*pWal).truncateOnCommit }) != (0 as u8)
        && (unsafe { (*pWal).mxWalSize }) >= ((0 as i32) as i64)
    {
        let mut sz: i64 = unsafe { (*pWal).mxWalSize };
        if ((32 as i32) as i64)
            + ((iFrame
                .wrapping_add(nExtra as u32)
                .wrapping_add((1 as i32) as u32)
                .wrapping_sub((1 as i32) as u32) as u64) as i64)
                * ((szPage + (24 as i32)) as i64)
            > unsafe { (*pWal).mxWalSize }
        {
            sz = ((32 as i32) as i64)
                + ((iFrame
                    .wrapping_add(nExtra as u32)
                    .wrapping_add((1 as i32) as u32)
                    .wrapping_sub((1 as i32) as u32) as u64) as i64)
                    * ((szPage + (24 as i32)) as i64);
        }
        walLimitSize(pWal, sz);
        unsafe {
            (*pWal).truncateOnCommit = ((0 as i32) as i8) as u8;
        }
    }
    // Append data to the wal-index. It is not necessary to lock the
    // wal-index to do this as the SQLITE_SHM_WRITE lock held on the wal-index
    // guarantees that there are no other writers, and no data that may
    // be in use by existing readers is being overwritten.
    iFrame = unsafe { (*pWal).hdr.mxFrame };
    p = pList;
    '__slate_break_992: while p != std::ptr::null_mut::<PgHdr>() && rc == (0 as i32) {
        if (((unsafe { (*p).flags }) as u32) as i32) & (64 as i32) == (0 as i32) {
        } else {
            let __v1183: u32 = iFrame;
            let __v1184: u32 = __v1183.wrapping_add((1 as i32) as u32);
            iFrame = __v1184;
            rc = walIndexAppend(pWal, iFrame, unsafe { (*p).pgno });
        }
        p = unsafe { (*p).pDirty };
    }
    0 as i32;
    '__slate_break_993: while rc == (0 as i32) && nExtra > (0 as i32) {
        let __v1185: u32 = iFrame;
        let __v1186: u32 = __v1185.wrapping_add((1 as i32) as u32);
        iFrame = __v1186;
        let __v1187: i32 = nExtra;
        let __v1188: i32 = __v1187 - (1 as i32);
        nExtra = __v1188;
        rc = walIndexAppend(pWal, iFrame, unsafe { (*pLast).pgno });
    }
    if rc == (0 as i32) {
        // Update the private copy of the header.
        unsafe {
            (*pWal).hdr.szPage = ((szPage & (65280 as i32) | szPage >> (16 as i32)) as i16) as u16;
        }
        {}
        {}
        unsafe {
            (*pWal).hdr.mxFrame = iFrame;
        }
        if isCommit != (0 as i32) {
            let __v1189: *mut Wal = pWal;
            let __v1190: u32 = unsafe { (*__v1189).hdr.iChange };
            let __v1191: u32 = __v1190.wrapping_add((1 as i32) as u32);
            unsafe {
                (*__v1189).hdr.iChange = __v1191;
            }
            unsafe {
                (*pWal).hdr.nPage = nTruncate;
            }
        }
        // If this is a commit, update the wal-index header too.
        if isCommit != (0 as i32) {
            walIndexWriteHdr(pWal);
            unsafe {
                (*pWal).iCallback = iFrame;
            }
        }
    }
    {}
    return rc;
}

/// Write a set of frames to the log. The caller must hold the write-lock
/// on the log file (obtained using sqlite3WalBeginWriteTransaction()).
///
/// The difference between this function and walFrames() is that this
/// function wraps walFrames() in an SEH_TRY{...} block.
///
/// # Arguments
///
/// * `pWal` - Wal handle to write to
/// * `szPage` - Database page-size in bytes
/// * `pList` - List of dirty pages to write
/// * `nTruncate` - Database size after this commit
/// * `isCommit` - True if this is a commit
/// * `sync_flags` - Flags to pass to OsSync() (or 0)
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3WalFrames(
    mut pWal: *mut Wal,
    mut szPage: i32,
    mut pList: *mut PgHdr,
    mut nTruncate: u32,
    mut isCommit: i32,
    mut sync_flags: i32,
) -> i32 {
    let mut rc: i32 = 0 as i32;
    {}
    rc = walFrames(pWal, szPage, pList, nTruncate, isCommit, sync_flags);
    {}
    0 as i32;
    return rc;
}

/// This routine is called to implement sqlite3_wal_checkpoint() and
/// related interfaces.
///
/// Obtain a CHECKPOINT lock and then backfill as much information as
/// we can from WAL into the database.
///
/// If parameter xBusy is not NULL, it is a pointer to a busy-handler
/// callback. In this case this function runs a blocking checkpoint.
///
/// # Arguments
///
/// * `pWal` - Wal connection
/// * `db` - Check this handle's interrupt flag
/// * `eMode` - PASSIVE, FULL, RESTART, or TRUNCATE
/// * `xBusy` - Function to call when busy
/// * `pBusyArg` - Context argument for xBusyHandler
/// * `sync_flags` - Flags to sync db file with (or 0)
/// * `nBuf` - Size of temporary buffer
/// * `zBuf` - Temporary buffer to use
/// * `pnLog` - OUT: Number of frames in WAL
/// * `pnCkpt` - OUT: Number of backfilled frames in WAL
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3WalCheckpoint(
    mut pWal: *mut Wal,
    mut db: *mut sqlite3,
    mut eMode: i32,
    mut xBusy: Option<unsafe extern "C-unwind" fn(*mut ()) -> i32>,
    mut pBusyArg: *mut (),
    mut sync_flags: i32,
    mut nBuf: i32,
    mut zBuf: *mut u8,
    mut pnLog: *mut i32,
    mut pnCkpt: *mut i32,
) -> i32 {
    let mut rc: i32 = 0 as i32; // Return code
    let mut isChanged: i32 = 0 as i32; // True if a new wal-index header is loaded
    let mut eMode2: i32 = eMode; // Mode to pass to walCheckpoint()
    let mut xBusy2: Option<unsafe extern "C-unwind" fn(*mut ()) -> i32> = xBusy; // Busy handler for eMode2
    0 as i32;
    0 as i32;
    // EVIDENCE-OF: R-62920-47450 The busy-handler callback is never invoked
    // in the SQLITE_CHECKPOINT_PASSIVE mode.
    0 as i32;
    0 as i32;
    if (unsafe { (*pWal).readOnly }) != (0 as u8) {
        return 8 as i32;
    }
    {}
    // Enable blocking locks, if possible.
    {}
    if xBusy2 != None {
        0 as i32;
    }
    // IMPLEMENTATION-OF: R-62028-47212 All calls obtain an exclusive
    // "checkpoint" lock on the database file.
    // EVIDENCE-OF: R-10421-19736 If any other process is running a
    // checkpoint operation at the same time, the lock cannot be obtained and
    // SQLITE_BUSY is returned.
    // EVIDENCE-OF: R-53820-33897 Even if there is a busy-handler configured,
    // it will not be invoked in this case.
    if eMode != -(1 as i32) {
        rc = walLockExclusive(pWal, 1 as i32, 1 as i32);
        {}
        {}
        if rc == (0 as i32) {
            unsafe {
                (*pWal).ckptLock = ((1 as i32) as i8) as u8;
            }
            // IMPLEMENTATION-OF: R-59782-36818 The SQLITE_CHECKPOINT_FULL, RESTART
            // and TRUNCATE modes also obtain the exclusive "writer" lock on the
            // database file.
            //
            // EVIDENCE-OF: R-60642-04082 If the writer lock cannot be obtained
            // immediately, and a busy-handler is configured, it is invoked and the
            // writer lock retried until either the busy-handler returns 0 or the
            // lock is successfully obtained.
            if eMode != (0 as i32) {
                rc = walBusyLock(pWal, xBusy2, pBusyArg, 0 as i32, 1 as i32);
                if rc == (0 as i32) {
                    unsafe {
                        (*pWal).writeLock = ((1 as i32) as i8) as u8;
                    }
                } else {
                    if rc == (5 as i32) {
                        eMode2 = 0 as i32;
                        xBusy2 = None;
                        rc = 0 as i32;
                    }
                }
            }
        }
    } else {
        rc = 0 as i32;
    }
    // Read the wal-index header.
    {}
    if rc == (0 as i32) {
        // For a passive checkpoint, do not re-enable blocking locks after
        // reading the wal-index header. A passive checkpoint should not block
        // or invoke the busy handler. The only lock such a checkpoint may
        // attempt to obtain is a lock on a read-slot, and it should give up
        // immediately and do a partial checkpoint if it cannot obtain it.
        {}
        rc = walIndexReadHdr(pWal, std::ptr::addr_of_mut!(isChanged));
        if eMode2 > (0 as i32) {
            0 as i32;
        }
        if isChanged != (0 as i32)
            && (unsafe { (*unsafe { (*unsafe { (*pWal).pDbFd }).pMethods }).iVersion })
                >= (3 as i32)
        {
            unsafe {
                sqlite3OsUnfetch(
                    unsafe { (*pWal).pDbFd },
                    (0 as i32) as i64,
                    std::ptr::null_mut::<()>(),
                )
            };
        }
    }
    // Copy data from the log to the database file.
    if rc == (0 as i32) {
        unsafe { sqlite3FaultSim(660 as i32) };
        let __v998: bool;
        if (unsafe { (*pWal).hdr.mxFrame }) != (0 as u32) {
            __v998 = walPagesize(pWal) != nBuf;
        } else {
            __v998 = false as bool;
        }
        if __v998 {
            rc = unsafe { sqlite3CorruptError(4397 as i32) };
        } else {
            if eMode2 != -(1 as i32) {
                rc = walCheckpoint(pWal, db, eMode2, xBusy2, pBusyArg, sync_flags, zBuf);
            }
        }
        // If no error occurred, set the output variables.
        if rc == (0 as i32) || rc == (5 as i32) {
            if pnLog != std::ptr::null_mut::<i32>() {
                unsafe {
                    *pnLog = (unsafe { (*pWal).hdr.mxFrame }) as i32;
                }
            }
            0 as i32;
            {}
            if pnCkpt != std::ptr::null_mut::<i32>() {
                unsafe {
                    *pnCkpt = (unsafe {
                        std::ptr::read_volatile(std::ptr::addr_of!((*walCkptInfo(pWal)).nBackfill))
                    }) as i32;
                }
            }
        }
    }
    {}
    0 as i32;
    if isChanged != (0 as i32) {
        // If a new wal-index header was loaded before the checkpoint was
        // performed, then the pager-cache associated with pWal is now
        // out of date. So zero the cached wal-index header to ensure that
        // next time the pager opens a snapshot on this database it knows that
        // the cache needs to be reset.
        unsafe {
            memset(
                (unsafe { std::ptr::addr_of_mut!((*pWal).hdr) }) as *mut (),
                0 as i32,
                48 as u64,
            )
        };
    }
    {}
    {}
    // Release the locks.
    sqlite3WalEndWriteTransaction(pWal);
    if (unsafe { (*pWal).ckptLock }) != (0 as u8) {
        walUnlockExclusive(pWal, 1 as i32, 1 as i32);
        unsafe {
            (*pWal).ckptLock = ((0 as i32) as i8) as u8;
        }
    }
    {}
    return if rc == (0 as i32) && eMode != eMode2 {
        5 as i32
    } else {
        rc
    };
}

/// Return the value to pass to a sqlite3_wal_hook callback, the
/// number of frames in the WAL at the point of the last commit since
/// sqlite3WalCallback() was called.  If no commits have occurred since
/// the last call, then return 0.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3WalCallback(mut pWal: *mut Wal) -> i32 {
    let mut ret: u32 = (0 as i32) as u32;
    if pWal != std::ptr::null_mut::<Wal>() {
        ret = unsafe { (*pWal).iCallback };
        unsafe {
            (*pWal).iCallback = (0 as i32) as u32;
        }
    }
    return ret as i32;
}

/// This function is called to change the WAL subsystem into or out
/// of locking_mode=EXCLUSIVE.
///
/// If op is zero, then attempt to change from locking_mode=EXCLUSIVE
/// into locking_mode=NORMAL.  This means that we must acquire a lock
/// on the pWal->readLock byte.  If the WAL is already in locking_mode=NORMAL
/// or if the acquisition of the lock fails, then return 0.  If the
/// transition out of exclusive-mode is successful, return 1.  This
/// operation must occur while the pager is still holding the exclusive
/// lock on the main database file.
///
/// If op is one, then change from locking_mode=NORMAL into
/// locking_mode=EXCLUSIVE.  This means that the pWal->readLock must
/// be released.  Return 1 if the transition is made and 0 if the
/// WAL is already in exclusive-locking mode - meaning that this
/// routine is a no-op.  The pager must already hold the exclusive lock
/// on the main database file before invoking this operation.
///
/// If op is negative, then do a dry-run of the op==1 case but do
/// not actually change anything. The pager uses this to see if it
/// should acquire the database exclusive lock prior to invoking
/// the op==1 case.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3WalExclusiveMode(mut pWal: *mut Wal, mut op: i32) -> i32 {
    let mut rc: i32 = 0 as i32;
    0 as i32;
    0 as i32;
    // pWal->readLock is usually set, but might be -1 if there was a
    // prior error while attempting to acquire are read-lock. This cannot
    // happen if the connection is actually in exclusive mode (as no xShmLock
    // locks are taken in this case). Nor should the pager attempt to
    // upgrade to exclusive-mode following such an error.
    0 as i32;
    0 as i32;
    if op == (0 as i32) {
        if (((unsafe { (*pWal).exclusiveMode }) as u32) as i32) != (0 as i32) {
            unsafe {
                (*pWal).exclusiveMode = ((0 as i32) as i8) as u8;
            }
            if walLockShared(pWal, (3 as i32) + ((unsafe { (*pWal).readLock }) as i32))
                != (0 as i32)
            {
                unsafe {
                    (*pWal).exclusiveMode = ((1 as i32) as i8) as u8;
                }
            }
            rc = ((((unsafe { (*pWal).exclusiveMode }) as u32) as i32) == (0 as i32)) as i32;
        } else {
            // Already in locking_mode=NORMAL
            rc = 0 as i32;
        }
    } else {
        if op > (0 as i32) {
            0 as i32;
            0 as i32;
            walUnlockShared(pWal, (3 as i32) + ((unsafe { (*pWal).readLock }) as i32));
            unsafe {
                (*pWal).exclusiveMode = ((1 as i32) as i8) as u8;
            }
            rc = 1 as i32;
        } else {
            rc = ((((unsafe { (*pWal).exclusiveMode }) as u32) as i32) == (0 as i32)) as i32;
        }
    }
    return rc;
}

/// Return true if the argument is non-NULL and the WAL module is using
/// heap-memory for the wal-index. Otherwise, if the argument is NULL or the
/// WAL module is using shared-memory, return false.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3WalHeapMemory(mut pWal: *mut Wal) -> i32 {
    return (pWal != std::ptr::null_mut::<Wal>()
        && (((unsafe { (*pWal).exclusiveMode }) as u32) as i32) == (2 as i32)) as i32;
}

/// Return the sqlite3_file object for the WAL file
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3WalFile(mut pWal: *mut Wal) -> *mut sqlite3_file {
    return unsafe { (*pWal).pWalFd };
}
