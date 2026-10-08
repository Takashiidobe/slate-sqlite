//! 2016-09-07
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
//! This file implements an in-memory VFS. A database is held as a contiguous
//! block of memory.
//!
//! This file also implements interface sqlite3_serialize() and
//! sqlite3_deserialize().
unsafe extern "C" {
    static mut sqlite3Config: Sqlite3Config;
    fn sqlite3_exec(
        __v566: *mut sqlite3,
        sql: *const i8,
        callback: Option<
            unsafe extern "C-unwind" fn(*mut (), i32, *mut *mut i8, *mut *mut i8) -> i32,
        >,
        __v569: *mut (),
        errmsg: *mut *mut i8,
    ) -> i32;
    fn sqlite3_mprintf(__v571: *const i8, ...) -> *mut i8;
    fn sqlite3_snprintf(__v572: i32, __v573: *mut i8, __v574: *const i8, ...) -> *mut i8;
    fn sqlite3_malloc64(__v575: u64) -> *mut ();
    fn sqlite3_free(__v576: *mut ());
    fn sqlite3_prepare_v2(
        db: *mut sqlite3,
        zSql: *const i8,
        nByte: i32,
        ppStmt: *mut *mut sqlite3_stmt,
        pzTail: *mut *const i8,
    ) -> i32;
    fn sqlite3_step(__v582: *mut sqlite3_stmt) -> i32;
    fn sqlite3_column_int(__v583: *mut sqlite3_stmt, iCol: i32) -> i32;
    fn sqlite3_column_int64(__v585: *mut sqlite3_stmt, iCol: i32) -> i64;
    fn sqlite3_finalize(pStmt: *mut sqlite3_stmt) -> i32;
    fn sqlite3_reset(pStmt: *mut sqlite3_stmt) -> i32;
    fn sqlite3_vfs_find(zVfsName: *const i8) -> *mut sqlite3_vfs;
    fn sqlite3_vfs_register(__v590: *mut sqlite3_vfs, makeDflt: i32) -> i32;
    fn sqlite3_mutex_alloc(__v592: i32) -> *mut sqlite3_mutex;
    fn sqlite3_mutex_free(__v593: *mut sqlite3_mutex);
    fn sqlite3_mutex_enter(__v594: *mut sqlite3_mutex);
    fn sqlite3_mutex_leave(__v595: *mut sqlite3_mutex);
    fn sqlite3_file_control(
        __v596: *mut sqlite3,
        zDbName: *const i8,
        op: i32,
        __v599: *mut (),
    ) -> i32;
    fn memcpy(__dest: *mut (), __src: *const (), __n: u64) -> *mut ();
    fn memset(__s: *mut (), __c: i32, __n: u64) -> *mut ();
    fn strcmp(__s1: *const i8, __s2: *const i8) -> i32;
    fn sqlite3PagerGet(pPager: *mut Pager, pgno: u32, ppPage: *mut *mut PgHdr, clrFlag: i32)
    -> i32;
    fn sqlite3PagerUnref(__v622: *mut PgHdr);
    fn sqlite3PagerGetData(__v623: *mut PgHdr) -> *mut ();
    fn sqlite3BtreeGetPageSize(__v624: *mut Btree) -> i32;
    fn sqlite3BtreePager(__v625: *mut Btree) -> *mut Pager;
    fn sqlite3Strlen30(__v626: *const i8) -> i32;
    fn sqlite3Malloc(__v627: u64) -> *mut ();
    fn sqlite3Realloc(__v628: *mut (), __v629: u64) -> *mut ();
    fn sqlite3MutexAlloc(__v630: i32) -> *mut sqlite3_mutex;
    fn sqlite3FindDbName(__v632: *mut sqlite3, __v633: *const i8) -> i32;
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
struct sqlite3_stmt {}

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
struct sqlite3_pcache_page {
    pBuf: *mut (),
    pExtra: *mut (),
}

#[repr(C)]
#[derive(Clone, Copy)]
struct sqlite3_pcache {}

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
    trace: __SlateRecord173,
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
    u1: __SlateRecord174,
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
    u: __SlateRecord175,
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
    __slate_bits_0: __slate_bits::__SlateBits73U0,
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
    u: __SlateRecord176,
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
    __slate_bits_0: __slate_bits::__SlateBits97U0,
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
    u: __SlateRecord184,
    pLeft: *mut Expr,
    pRight: *mut Expr,
    x: __SlateRecord185,
    nHeight: i32,
    iTable: i32,
    iColumn: i16,
    iAgg: i16,
    w: __SlateRecord186,
    pAggInfo: *mut AggInfo,
    y: __SlateRecord187,
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
    fg: __SlateRecord194,
    iCursor: i32,
    colUsed: u64,
    u1: __SlateRecord195,
    u2: __SlateRecord196,
    u3: __SlateRecord197,
    u4: __SlateRecord198,
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
    __slate_bits_0: __slate_bits::__SlateBits109U0,
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
    u1: __SlateRecord200,
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
struct Pager {}

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
struct Btree {}

#[repr(C)]
#[derive(Clone, Copy)]
struct Vdbe {}

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
struct DbClientData {
    pNext: *mut DbClientData,
    pData: *mut (),
    xDestructor: Option<unsafe extern "C-unwind" fn(*mut ())>,
    zName: [i8; 0],
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
    __slate_bits_0: __slate_bits::__SlateBits172U0,
    azInit: *mut *const i8,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord173 {
    xLegacy: Option<unsafe extern "C-unwind" fn(*mut (), *const i8)>,
    xV2: Option<unsafe extern "C-unwind" fn(u32, *mut (), *mut (), *mut ()) -> i32>,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord174 {
    isInterrupted: i32,
    notUsed1: f64,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord175 {
    pHash: *mut FuncDef,
    pDestructor: *mut FuncDestructor,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord176 {
    tab: __SlateRecord177,
    view: __SlateRecord178,
    vtab: __SlateRecord179,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord177 {
    addColOffset: i32,
    pFKey: *mut FKey,
    pDfltList: *mut ExprList,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord178 {
    pSelect: *mut Select,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord179 {
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
union __SlateRecord184 {
    zToken: *mut i8,
    iValue: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord185 {
    pList: *mut ExprList,
    pSelect: *mut Select,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord186 {
    iJoin: i32,
    iOfst: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord187 {
    pTab: *mut Table,
    pWin: *mut Window,
    nReg: i32,
    sub: __SlateRecord188,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord188 {
    iAddr: i32,
    regReturn: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct ExprList_item {
    pExpr: *mut Expr,
    zEName: *mut i8,
    fg: __SlateRecord190,
    u: __SlateRecord191,
}

#[repr(C, align(4))]
#[derive(Clone, Copy)]
struct __SlateRecord190 {
    sortFlags: u8,
    __slate_bits_0: __slate_bits::__SlateBits190U0,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord191 {
    x: __SlateRecord192,
    iConstExprReg: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord192 {
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
struct __SlateRecord194 {
    jointype: u8,
    __slate_bits_0: __slate_bits::__SlateBits194U0,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord195 {
    zIndexedBy: *mut i8,
    pFuncArg: *mut ExprList,
    nRow: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord196 {
    pIBIndex: *mut Index,
    pCteUse: *mut CteUse,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord197 {
    pOn: *mut Expr,
    pUsing: *mut IdList,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord198 {
    pSchema: *mut Schema,
    zDatabase: *mut i8,
    pSubq: *mut Subquery,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord200 {
    cr: __SlateRecord201,
    d: __SlateRecord202,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord201 {
    addrCrTab: i32,
    regRowid: i32,
    regRoot: i32,
    constraintName: Token,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord202 {
    pReturning: *mut Returning,
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

// Access to a lower-level VFS that (might) implement dynamic loading,
// access to randomness, etc.
// Forward declaration of objects used by this utility
/// Storage for a memdb file.
///
/// An memdb object can be shared or separate.  Shared memdb objects can be
/// used by more than one database connection.  Mutexes are used by shared
/// memdb objects to coordinate access.  Separate memdb objects are only
/// connected to a single database connection and do not require additional
/// mutexes.
///
/// Shared memdb objects have .zFName!=0 and .pMutex!=0.  They are created
/// using "file:/name?vfs=memdb".  The first character of the name must be
/// "/" or else the object will be a separate memdb object.  All shared
/// memdb objects are stored in memdb_g.apMemStore[] in an arbitrary order.
///
/// Separate memdb objects are created using a name that does not begin
/// with "/" or using sqlite3_deserialize().
///
/// Access rules for shared MemStore objects:
///
///   *  .zFName is initialized when the object is created and afterwards
///      is unchanged until the object is destroyed.  So it can be accessed
///      at any time as long as we know the object is not being destroyed,
///      which means while either the SQLITE_MUTEX_STATIC_VFS1 or
///      .pMutex is held or the object is not part of memdb_g.apMemStore[].
///
///   *  Can .pMutex can only be changed while holding the
///      SQLITE_MUTEX_STATIC_VFS1 mutex or while the object is not part
///      of memdb_g.apMemStore[].
///
///   *  Other fields can only be changed while holding the .pMutex mutex
///      or when the .nRef is less than zero and the object is not part of
///      memdb_g.apMemStore[].
///
///   *  The .aData pointer has the added requirement that it can can only
///      be changed (for resizing) when nMmap is zero.
#[repr(C)]
#[derive(Clone, Copy)]
struct MemStore {
    /// Size of the file
    sz: i64,
    /// Space allocated to aData
    szAlloc: i64,
    /// Maximum allowed size of the file
    szMax: i64,
    /// content of the file
    aData: *mut u8,
    /// Used by shared stores only
    pMutex: *mut sqlite3_mutex,
    /// Number of memory mapped pages
    nMmap: i32,
    /// Flags
    mFlags: u32,
    /// Number of readers
    nRdLock: i32,
    /// Number of writers.  (Always 0 or 1)
    nWrLock: i32,
    /// Number of users of this MemStore
    nRef: i32,
    /// The filename for shared stores
    zFName: *mut i8,
}

/// An open file
#[repr(C)]
#[derive(Clone, Copy)]
struct MemFile {
    /// IO methods
    base: sqlite3_file,
    /// The storage
    pStore: *mut MemStore,
    /// Most recent lock against this file
    eLock: i32,
}

/// File-scope variables for holding the memdb files that are accessible
/// to multiple database connections in separate threads.
///
/// Must hold SQLITE_MUTEX_STATIC_VFS1 to access any part of this object.
#[repr(C)]
#[derive(Clone, Copy)]
struct MemFS {
    /// Number of shared MemStore objects
    nMemStore: i32,
    /// Array of all shared MemStore objects
    apMemStore: *mut *mut MemStore,
}

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
    pub struct __SlateBits73U0 {
        #[bits(4)]
        pub notNull: u32,
        #[bits(4)]
        pub eCType: u32,
    }
    #[bitfields::bitfield([u8; 2], c_names = true, new = false, from_into_bits = false, from_traits = false, default = false, debug = false, builder = false, bit_ops = false)]
    pub struct __SlateBits190U0 {
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
    pub struct __SlateBits194U0 {
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
    pub struct __SlateBits97U0 {
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
    pub struct __SlateBits172U0 {
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
    pub struct __SlateBits109U0 {
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

static mut memdb_g: MemFS = unsafe { std::mem::zeroed() };

/// iVersion
/// szOsFile (set when registered)
/// mxPathname
/// pNext
/// zName
/// pAppData (set when registered)
/// xOpen
/// memdbDelete,
///
/// xDelete
/// xAccess
/// xFullPathname
/// xDlOpen
/// xDlError
/// xDlSym
/// xDlClose
/// xRandomness
/// xSleep
/// memdbCurrentTime,
///
/// xCurrentTime
/// xGetLastError
/// xCurrentTimeInt64
/// xSetSystemCall
/// xGetSystemCall
/// xNextSystemCall
static mut memdb_vfs: sqlite3_vfs = sqlite3_vfs {
    iVersion: 2 as i32,
    szOsFile: 0 as i32,
    mxPathname: 1024 as i32,
    pNext: std::ptr::null_mut::<sqlite3_vfs>(),
    zName: (b"memdb\0".as_ptr() as *mut i8) as *const i8,
    pAppData: std::ptr::null_mut::<()>(),
    xOpen: Some(memdbOpen),
    xDelete: None,
    xAccess: Some(memdbAccess),
    xFullPathname: Some(memdbFullPathname),
    xDlOpen: Some(memdbDlOpen),
    xDlError: Some(memdbDlError),
    xDlSym: Some(memdbDlSym),
    xDlClose: Some(memdbDlClose),
    xRandomness: Some(memdbRandomness),
    xSleep: Some(memdbSleep),
    xCurrentTime: None,
    xGetLastError: Some(memdbGetLastError),
    xCurrentTimeInt64: Some(memdbCurrentTimeInt64),
    xSetSystemCall: None,
    xGetSystemCall: None,
    xNextSystemCall: None,
};

/// iVersion
/// xClose
/// xRead
/// xWrite
/// xTruncate
/// xSync
/// xFileSize
/// xLock
/// xUnlock
/// memdbCheckReservedLock,
///
/// xCheckReservedLock
/// xFileControl
/// memdbSectorSize,
///
/// xSectorSize
/// xDeviceCharacteristics
/// xShmMap
/// xShmLock
/// xShmBarrier
/// xShmUnmap
/// xFetch
/// xUnfetch
static mut memdb_io_methods: sqlite3_io_methods = sqlite3_io_methods {
    iVersion: 3 as i32,
    xClose: Some(memdbClose),
    xRead: Some(memdbRead),
    xWrite: Some(memdbWrite),
    xTruncate: Some(memdbTruncate),
    xSync: Some(memdbSync),
    xFileSize: Some(memdbFileSize),
    xLock: Some(memdbLock),
    xUnlock: Some(memdbUnlock),
    xCheckReservedLock: None,
    xFileControl: Some(memdbFileControl),
    xSectorSize: None,
    xDeviceCharacteristics: Some(memdbDeviceCharacteristics),
    xShmMap: None,
    xShmLock: None,
    xShmBarrier: None,
    xShmUnmap: None,
    xFetch: Some(memdbFetch),
    xUnfetch: Some(memdbUnfetch),
};

/// Enter/leave the mutex on a MemStore
fn memdbEnter(mut p: *mut MemStore) {
    unsafe { sqlite3_mutex_enter(unsafe { (*p).pMutex }) };
}

fn memdbLeave(mut p: *mut MemStore) {
    unsafe { sqlite3_mutex_leave(unsafe { (*p).pMutex }) };
}

/// Close an memdb-file.
/// Free the underlying MemStore object when its refcount drops to zero
/// or less.
#[unsafe(link_section = ".text.slate_distinct.memdb.memdbClose")]
extern "C-unwind" fn memdbClose(mut pFile: *mut sqlite3_file) -> i32 {
    let mut p: *mut MemStore = unsafe { (*(pFile as *mut MemFile)).pStore };
    if (unsafe { (*p).zFName }) != std::ptr::null_mut::<i8>() {
        let mut i: i32 = 0 as i32;
        let mut pVfsMutex: *mut sqlite3_mutex = unsafe { sqlite3MutexAlloc(11 as i32) };
        unsafe { sqlite3_mutex_enter(pVfsMutex) };
        i = 0 as i32;
        '__slate_break_698: loop {
            if !(i < unsafe { memdb_g.nMemStore }) {
                break;
            }
            if (unsafe { *unsafe { unsafe { memdb_g.apMemStore }.offset(i as isize) } }) == p {
                memdbEnter(p);
                if (unsafe { (*p).nRef }) == (1 as i32) {
                    let __v712: i32 = unsafe { memdb_g.nMemStore };
                    let __v713: i32 = __v712 - (1 as i32);
                    unsafe {
                        memdb_g.nMemStore = __v713;
                    }
                    unsafe {
                        *unsafe { unsafe { memdb_g.apMemStore }.offset(i as isize) } = unsafe {
                            *unsafe { unsafe { memdb_g.apMemStore }.offset(__v713 as isize) }
                        };
                    }
                    if (unsafe { memdb_g.nMemStore }) == (0 as i32) {
                        unsafe { sqlite3_free((unsafe { memdb_g.apMemStore }) as *mut ()) };
                        unsafe {
                            memdb_g.apMemStore = std::ptr::null_mut::<*mut MemStore>();
                        }
                    }
                }
                break '__slate_break_698;
            }
            let __v710: i32 = i;
            let __v711: i32 = __v710 + (1 as i32);
            i = __v711;
        }
        unsafe { sqlite3_mutex_leave(pVfsMutex) };
    } else {
        memdbEnter(p);
    }
    let __v714: *mut MemStore = p;
    let __v715: i32 = unsafe { (*__v714).nRef };
    let __v716: i32 = __v715 - (1 as i32);
    unsafe {
        (*__v714).nRef = __v716;
    }
    if (unsafe { (*p).nRef }) <= (0 as i32) {
        if (unsafe { (*p).mFlags }) & ((1 as i32) as u32) != (0 as u32) {
            unsafe { sqlite3_free((unsafe { (*p).aData }) as *mut ()) };
        }
        memdbLeave(p);
        unsafe { sqlite3_mutex_free(unsafe { (*p).pMutex }) };
        unsafe { sqlite3_free(p as *mut ()) };
    } else {
        memdbLeave(p);
    }
    return 0 as i32;
}

/// Read data from an memdb-file.
#[unsafe(link_section = ".text.slate_distinct.memdb.memdbRead")]
extern "C-unwind" fn memdbRead(
    mut pFile: *mut sqlite3_file,
    mut zBuf: *mut (),
    mut iAmt: i32,
    mut iOfst: i64,
) -> i32 {
    let mut p: *mut MemStore = unsafe { (*(pFile as *mut MemFile)).pStore };
    memdbEnter(p);
    if iOfst + (iAmt as i64) > unsafe { (*p).sz } {
        unsafe { memset(zBuf, 0 as i32, (iAmt as i64) as u64) };
        if iOfst < unsafe { (*p).sz } {
            unsafe {
                memcpy(
                    zBuf,
                    (unsafe { unsafe { (*p).aData }.offset(iOfst as isize) }) as *const (),
                    ((unsafe { (*p).sz }) - iOfst) as u64,
                )
            };
        }
        memdbLeave(p);
        return (10 as i32) | (2 as i32) << (8 as i32);
    }
    unsafe {
        memcpy(
            zBuf,
            (unsafe { unsafe { (*p).aData }.offset(iOfst as isize) }) as *const (),
            (iAmt as i64) as u64,
        )
    };
    memdbLeave(p);
    return 0 as i32;
}

/// Try to enlarge the memory allocation to hold at least sz bytes
fn memdbEnlarge(mut p: *mut MemStore, mut newSz: i64) -> i32 {
    let mut pNew: *mut u8 = unsafe { std::mem::zeroed() };
    if (unsafe { (*p).mFlags }) & ((2 as i32) as u32) == ((0 as i32) as u32)
        || (unsafe { (*p).nMmap }) > (0 as i32)
    {
        return 13 as i32;
    }
    if newSz > unsafe { (*p).szMax } {
        return 13 as i32;
    }
    let __v746: i64 = newSz;
    let __v747: i64 = __v746 * ((2 as i32) as i64);
    newSz = __v747;
    if newSz > unsafe { (*p).szMax } {
        newSz = unsafe { (*p).szMax };
    }
    pNew = (unsafe { sqlite3Realloc((unsafe { (*p).aData }) as *mut (), newSz as u64) }) as *mut u8;
    if pNew == std::ptr::null_mut::<u8>() {
        return (10 as i32) | (12 as i32) << (8 as i32);
    }
    unsafe {
        (*p).aData = pNew;
    }
    unsafe {
        (*p).szAlloc = newSz;
    }
    return 0 as i32;
}

/// Write data to an memdb-file.
#[unsafe(link_section = ".text.slate_distinct.memdb.memdbWrite")]
extern "C-unwind" fn memdbWrite(
    mut pFile: *mut sqlite3_file,
    mut z: *const (),
    mut iAmt: i32,
    mut iOfst: i64,
) -> i32 {
    let mut p: *mut MemStore = unsafe { (*(pFile as *mut MemFile)).pStore };
    memdbEnter(p);
    if (unsafe { (*p).mFlags }) & ((4 as i32) as u32) != (0 as u32) {
        // Can't happen: memdbLock() will return SQLITE_READONLY before
        // reaching this point
        memdbLeave(p);
        return (10 as i32) | (3 as i32) << (8 as i32);
    }
    if iOfst + (iAmt as i64) > unsafe { (*p).sz } {
        let mut rc: i32 = 0 as i32;
        let __v717: bool;
        if iOfst + (iAmt as i64) > unsafe { (*p).szAlloc } {
            let __v718: i32 = memdbEnlarge(p, iOfst + (iAmt as i64));
            rc = __v718;
            __v717 = __v718 != (0 as i32);
        } else {
            __v717 = false as bool;
        }
        if __v717 {
            memdbLeave(p);
            return rc;
        }
        if iOfst > unsafe { (*p).sz } {
            unsafe {
                memset(
                    (unsafe { unsafe { (*p).aData }.offset((unsafe { (*p).sz }) as isize) })
                        as *mut (),
                    0 as i32,
                    (iOfst - unsafe { (*p).sz }) as u64,
                )
            };
        }
        unsafe {
            (*p).sz = iOfst + (iAmt as i64);
        }
    }
    unsafe {
        memcpy(
            (unsafe { unsafe { (*p).aData }.offset(iOfst as isize) }) as *mut (),
            z,
            (iAmt as i64) as u64,
        )
    };
    memdbLeave(p);
    return 0 as i32;
}

/// Truncate an memdb-file.
///
/// In rollback mode (which is always the case for memdb, as it does not
/// support WAL mode) the truncate() method is only used to reduce
/// the size of a file, never to increase the size.
#[unsafe(link_section = ".text.slate_distinct.memdb.memdbTruncate")]
extern "C-unwind" fn memdbTruncate(mut pFile: *mut sqlite3_file, mut size: i64) -> i32 {
    let mut p: *mut MemStore = unsafe { (*(pFile as *mut MemFile)).pStore };
    let mut rc: i32 = 0 as i32;
    memdbEnter(p);
    if size > unsafe { (*p).sz } {
        // This can only happen with a corrupt wal mode db
        rc = 11 as i32;
    } else {
        unsafe {
            (*p).sz = size;
        }
    }
    memdbLeave(p);
    return rc;
}

/// Sync an memdb-file.
#[unsafe(link_section = ".text.slate_distinct.memdb.memdbSync")]
extern "C-unwind" fn memdbSync(mut pFile: *mut sqlite3_file, mut flags: i32) -> i32 {
    pFile;
    flags;
    return 0 as i32;
}

/// Return the current file-size of an memdb-file.
#[unsafe(link_section = ".text.slate_distinct.memdb.memdbFileSize")]
extern "C-unwind" fn memdbFileSize(mut pFile: *mut sqlite3_file, mut pSize: *mut i64) -> i32 {
    let mut p: *mut MemStore = unsafe { (*(pFile as *mut MemFile)).pStore };
    memdbEnter(p);
    unsafe {
        *pSize = unsafe { (*p).sz };
    }
    memdbLeave(p);
    return 0 as i32;
}

/// Lock an memdb-file.
#[unsafe(link_section = ".text.slate_distinct.memdb.memdbLock")]
extern "C-unwind" fn memdbLock(mut pFile: *mut sqlite3_file, mut eLock: i32) -> i32 {
    let mut pThis: *mut MemFile = pFile as *mut MemFile;
    let mut p: *mut MemStore = unsafe { (*pThis).pStore };
    let mut rc: i32 = 0 as i32;
    if eLock <= unsafe { (*pThis).eLock } {
        return 0 as i32;
    }
    memdbEnter(p);
    0 as i32;
    0 as i32;
    0 as i32;
    if eLock > (1 as i32) && (unsafe { (*p).mFlags }) & ((4 as i32) as u32) != (0 as u32) {
        rc = 8 as i32;
    } else {
        '__slate_break_699: {
            match eLock {
                1 => {
                    0 as i32;
                    if (unsafe { (*p).nWrLock }) > (0 as i32) {
                        rc = 5 as i32;
                    } else {
                        let __v719: *mut MemStore = p;
                        let __v720: i32 = unsafe { (*__v719).nRdLock };
                        let __v721: i32 = __v720 + (1 as i32);
                        unsafe {
                            (*__v719).nRdLock = __v721;
                        }
                    }
                    break '__slate_break_699;
                    {}
                    0 as i32;
                    if (unsafe { (*pThis).eLock }) == (1 as i32) {
                        if (unsafe { (*p).nWrLock }) > (0 as i32) {
                            rc = 5 as i32;
                        } else {
                            unsafe {
                                (*p).nWrLock = 1 as i32;
                            }
                        }
                    }
                }
                2 | 3 => {
                    0 as i32;
                    if (unsafe { (*pThis).eLock }) == (1 as i32) {
                        if (unsafe { (*p).nWrLock }) > (0 as i32) {
                            rc = 5 as i32;
                        } else {
                            unsafe {
                                (*p).nWrLock = 1 as i32;
                            }
                        }
                    }
                }
                _ => {
                    0 as i32;
                    0 as i32;
                    if (unsafe { (*p).nRdLock }) > (1 as i32) {
                        rc = 5 as i32;
                    } else {
                        if (unsafe { (*pThis).eLock }) == (1 as i32) {
                            unsafe {
                                (*p).nWrLock = 1 as i32;
                            }
                        }
                    }
                }
            }
        }
    }
    if rc == (0 as i32) {
        unsafe {
            (*pThis).eLock = eLock;
        }
    }
    memdbLeave(p);
    return rc;
}

/// Unlock an memdb-file.
#[unsafe(link_section = ".text.slate_distinct.memdb.memdbUnlock")]
extern "C-unwind" fn memdbUnlock(mut pFile: *mut sqlite3_file, mut eLock: i32) -> i32 {
    let mut pThis: *mut MemFile = pFile as *mut MemFile;
    let mut p: *mut MemStore = unsafe { (*pThis).pStore };
    if eLock >= unsafe { (*pThis).eLock } {
        return 0 as i32;
    }
    memdbEnter(p);
    0 as i32;
    if eLock == (1 as i32) {
        if (unsafe { (*pThis).eLock }) > (1 as i32) {
            let __v722: *mut MemStore = p;
            let __v723: i32 = unsafe { (*__v722).nWrLock };
            let __v724: i32 = __v723 - (1 as i32);
            unsafe {
                (*__v722).nWrLock = __v724;
            }
        }
    } else {
        if (unsafe { (*pThis).eLock }) > (1 as i32) {
            let __v725: *mut MemStore = p;
            let __v726: i32 = unsafe { (*__v725).nWrLock };
            let __v727: i32 = __v726 - (1 as i32);
            unsafe {
                (*__v725).nWrLock = __v727;
            }
        }
        let __v728: *mut MemStore = p;
        let __v729: i32 = unsafe { (*__v728).nRdLock };
        let __v730: i32 = __v729 - (1 as i32);
        unsafe {
            (*__v728).nRdLock = __v730;
        }
    }
    unsafe {
        (*pThis).eLock = eLock;
    }
    memdbLeave(p);
    return 0 as i32;
}

/// File control method. For custom operations on an memdb-file.
#[unsafe(link_section = ".text.slate_distinct.memdb.memdbFileControl")]
extern "C-unwind" fn memdbFileControl(
    mut pFile: *mut sqlite3_file,
    mut op: i32,
    mut pArg: *mut (),
) -> i32 {
    let mut p: *mut MemStore = unsafe { (*(pFile as *mut MemFile)).pStore };
    let mut rc: i32 = 12 as i32;
    memdbEnter(p);
    if op == (12 as i32) {
        unsafe {
            *(pArg as *mut *mut i8) = unsafe {
                sqlite3_mprintf(
                    (b"memdb(%p,%lld)\0".as_ptr() as *mut i8) as *const i8,
                    unsafe { (*p).aData },
                    unsafe { (*p).sz },
                )
            };
        }
        rc = 0 as i32;
    }
    if op == (36 as i32) {
        let mut iLimit: i64 = unsafe { *(pArg as *mut i64) };
        if iLimit < unsafe { (*p).sz } {
            if iLimit < ((0 as i32) as i64) {
                iLimit = unsafe { (*p).szMax };
            } else {
                iLimit = unsafe { (*p).sz };
            }
        }
        unsafe {
            (*p).szMax = iLimit;
        }
        unsafe {
            *(pArg as *mut i64) = iLimit;
        }
        rc = 0 as i32;
    }
    memdbLeave(p);
    return rc;
}

/// Return the device characteristic flags supported by an memdb-file.
#[unsafe(link_section = ".text.slate_distinct.memdb.memdbDeviceCharacteristics")]
extern "C-unwind" fn memdbDeviceCharacteristics(mut pFile: *mut sqlite3_file) -> i32 {
    pFile;
    return (1 as i32) | (4096 as i32) | (512 as i32) | (1024 as i32);
}

/// Fetch a page of a memory-mapped file
#[unsafe(link_section = ".text.slate_distinct.memdb.memdbFetch")]
extern "C-unwind" fn memdbFetch(
    mut pFile: *mut sqlite3_file,
    mut iOfst: i64,
    mut iAmt: i32,
    mut pp: *mut *mut (),
) -> i32 {
    let mut p: *mut MemStore = unsafe { (*(pFile as *mut MemFile)).pStore };
    memdbEnter(p);
    if iOfst + (iAmt as i64) > unsafe { (*p).sz }
        || (unsafe { (*p).mFlags }) & ((2 as i32) as u32) != ((0 as i32) as u32)
    {
        unsafe {
            *pp = std::ptr::null_mut::<()>();
        }
    } else {
        let __v731: *mut MemStore = p;
        let __v732: i32 = unsafe { (*__v731).nMmap };
        let __v733: i32 = __v732 + (1 as i32);
        unsafe {
            (*__v731).nMmap = __v733;
        }
        unsafe {
            *pp = (unsafe { unsafe { (*p).aData }.offset(iOfst as isize) }) as *mut ();
        }
    }
    memdbLeave(p);
    return 0 as i32;
}

/// Release a memory-mapped page
#[unsafe(link_section = ".text.slate_distinct.memdb.memdbUnfetch")]
extern "C-unwind" fn memdbUnfetch(
    mut pFile: *mut sqlite3_file,
    mut iOfst: i64,
    mut pPage: *mut (),
) -> i32 {
    let mut p: *mut MemStore = unsafe { (*(pFile as *mut MemFile)).pStore };
    iOfst;
    pPage;
    memdbEnter(p);
    let __v734: *mut MemStore = p;
    let __v735: i32 = unsafe { (*__v734).nMmap };
    let __v736: i32 = __v735 - (1 as i32);
    unsafe {
        (*__v734).nMmap = __v736;
    }
    memdbLeave(p);
    return 0 as i32;
}

/// Open an mem file handle.
#[unsafe(link_section = ".text.slate_distinct.memdb.memdbOpen")]
extern "C-unwind" fn memdbOpen(
    mut pVfs: *mut sqlite3_vfs,
    mut zName: *const i8,
    mut pFd: *mut sqlite3_file,
    mut flags: i32,
    mut pOutFlags: *mut i32,
) -> i32 {
    let mut pFile: *mut MemFile = pFd as *mut MemFile;
    let mut p: *mut MemStore = std::ptr::null_mut::<MemStore>();
    let mut szName: i32 = 0 as i32;
    pVfs;
    unsafe { memset(pFile as *mut (), 0 as i32, 24 as u64) };
    szName = unsafe { sqlite3Strlen30(zName) };
    if szName > (1 as i32)
        && (((unsafe { *unsafe { zName.offset((0 as i32) as isize) } }) as i32) == (47 as i32)
            || ((unsafe { *unsafe { zName.offset((0 as i32) as isize) } }) as i32) == (92 as i32))
    {
        let mut i: i32 = 0 as i32;
        let mut pVfsMutex: *mut sqlite3_mutex = unsafe { sqlite3MutexAlloc(11 as i32) };
        unsafe { sqlite3_mutex_enter(pVfsMutex) };
        i = 0 as i32;
        '__slate_break_701: loop {
            if !(i < unsafe { memdb_g.nMemStore }) {
                break;
            }
            if (unsafe {
                strcmp(
                    (unsafe {
                        (*unsafe { *unsafe { unsafe { memdb_g.apMemStore }.offset(i as isize) } })
                            .zFName
                    }) as *const i8,
                    zName,
                )
            }) == (0 as i32)
            {
                p = unsafe { *unsafe { unsafe { memdb_g.apMemStore }.offset(i as isize) } };
                break '__slate_break_701;
            }
            let __v737: i32 = i;
            let __v738: i32 = __v737 + (1 as i32);
            i = __v738;
        }
        if p == std::ptr::null_mut::<MemStore>() {
            let mut apNew: *mut *mut MemStore = unsafe { std::mem::zeroed() };
            p = (unsafe {
                sqlite3Malloc(
                    (72 as u64)
                        .wrapping_add((szName as i64) as u64)
                        .wrapping_add(((3 as i32) as i64) as u64),
                )
            }) as *mut MemStore;
            if p == std::ptr::null_mut::<MemStore>() {
                unsafe { sqlite3_mutex_leave(pVfsMutex) };
                return 7 as i32;
            }
            apNew = (unsafe {
                sqlite3Realloc(
                    (unsafe { memdb_g.apMemStore }) as *mut (),
                    (8 as u64).wrapping_mul(
                        (((1 as i32) as i64) + ((unsafe { memdb_g.nMemStore }) as i64)) as u64,
                    ),
                )
            }) as *mut *mut MemStore;
            if apNew == std::ptr::null_mut::<*mut MemStore>() {
                unsafe { sqlite3_free(p as *mut ()) };
                unsafe { sqlite3_mutex_leave(pVfsMutex) };
                return 7 as i32;
            }
            let __v739: i32 = unsafe { memdb_g.nMemStore };
            let __v740: i32 = __v739 + (1 as i32);
            unsafe {
                memdb_g.nMemStore = __v740;
            }
            unsafe {
                *unsafe { apNew.offset(__v739 as isize) } = p;
            }
            unsafe {
                memdb_g.apMemStore = apNew;
            }
            unsafe { memset(p as *mut (), 0 as i32, 72 as u64) };
            unsafe {
                (*p).mFlags = ((2 as i32) | (1 as i32)) as u32;
            }
            unsafe {
                (*p).szMax = unsafe { sqlite3Config.mxMemdbSize };
            }
            unsafe {
                (*p).zFName = (unsafe { p.offset((1 as i32) as isize) }) as *mut i8;
            }
            unsafe {
                memcpy(
                    (unsafe { (*p).zFName }) as *mut (),
                    zName as *const (),
                    ((szName + (1 as i32)) as i64) as u64,
                )
            };
            unsafe {
                (*p).pMutex = unsafe { sqlite3_mutex_alloc(0 as i32) };
            }
            if (unsafe { (*p).pMutex }) == std::ptr::null_mut::<sqlite3_mutex>() {
                let __v741: i32 = unsafe { memdb_g.nMemStore };
                let __v742: i32 = __v741 - (1 as i32);
                unsafe {
                    memdb_g.nMemStore = __v742;
                }
                unsafe { sqlite3_free(p as *mut ()) };
                unsafe { sqlite3_mutex_leave(pVfsMutex) };
                return 7 as i32;
            }
            unsafe {
                (*p).nRef = 1 as i32;
            }
            memdbEnter(p);
        } else {
            memdbEnter(p);
            let __v743: *mut MemStore = p;
            let __v744: i32 = unsafe { (*__v743).nRef };
            let __v745: i32 = __v744 + (1 as i32);
            unsafe {
                (*__v743).nRef = __v745;
            }
        }
        unsafe { sqlite3_mutex_leave(pVfsMutex) };
    } else {
        p = (unsafe { sqlite3Malloc(72 as u64) }) as *mut MemStore;
        if p == std::ptr::null_mut::<MemStore>() {
            return 7 as i32;
        }
        unsafe { memset(p as *mut (), 0 as i32, 72 as u64) };
        unsafe {
            (*p).mFlags = ((2 as i32) | (1 as i32)) as u32;
        }
        unsafe {
            (*p).szMax = unsafe { sqlite3Config.mxMemdbSize };
        }
    }
    unsafe {
        (*pFile).pStore = p;
    }
    if pOutFlags != std::ptr::null_mut::<i32>() {
        unsafe {
            *pOutFlags = flags | (128 as i32);
        }
    }
    unsafe {
        (*pFd).pMethods = unsafe { std::ptr::addr_of!(memdb_io_methods) };
    }
    memdbLeave(p);
    return 0 as i32;
}

/// Test for access permissions. Return true if the requested permission
/// is available, or false otherwise.
///
/// With memdb, no files ever exist on disk.  So always return false.
#[unsafe(link_section = ".text.slate_distinct.memdb.memdbAccess")]
extern "C-unwind" fn memdbAccess(
    mut pVfs: *mut sqlite3_vfs,
    mut zPath: *const i8,
    mut flags: i32,
    mut pResOut: *mut i32,
) -> i32 {
    pVfs;
    zPath;
    flags;
    unsafe {
        *pResOut = 0 as i32;
    }
    return 0 as i32;
}

/// Populate buffer zOut with the full canonical pathname corresponding
/// to the pathname in zPath. zOut is guaranteed to point to a buffer
/// of at least (INST_MAX_PATHNAME+1) bytes.
#[unsafe(link_section = ".text.slate_distinct.memdb.memdbFullPathname")]
extern "C-unwind" fn memdbFullPathname(
    mut pVfs: *mut sqlite3_vfs,
    mut zPath: *const i8,
    mut nOut: i32,
    mut zOut: *mut i8,
) -> i32 {
    pVfs;
    unsafe {
        sqlite3_snprintf(
            nOut,
            zOut,
            (b"%s\0".as_ptr() as *mut i8) as *const i8,
            zPath,
        )
    };
    return 0 as i32;
}

/// Open the dynamic library located at zPath and return a handle.
#[unsafe(link_section = ".text.slate_distinct.memdb.memdbDlOpen")]
extern "C-unwind" fn memdbDlOpen(mut pVfs: *mut sqlite3_vfs, mut zPath: *const i8) -> *mut () {
    return unsafe {
        unsafe { (*((unsafe { (*pVfs).pAppData }) as *mut sqlite3_vfs)).xDlOpen }.unwrap()(
            (unsafe { (*pVfs).pAppData }) as *mut sqlite3_vfs,
            zPath,
        )
    };
}

/// Populate the buffer zErrMsg (size nByte bytes) with a human readable
/// utf-8 string describing the most recent error encountered associated
/// with dynamic libraries.
#[unsafe(link_section = ".text.slate_distinct.memdb.memdbDlError")]
extern "C-unwind" fn memdbDlError(
    mut pVfs: *mut sqlite3_vfs,
    mut nByte: i32,
    mut zErrMsg: *mut i8,
) {
    unsafe {
        unsafe { (*((unsafe { (*pVfs).pAppData }) as *mut sqlite3_vfs)).xDlError }.unwrap()(
            (unsafe { (*pVfs).pAppData }) as *mut sqlite3_vfs,
            nByte,
            zErrMsg,
        )
    };
}

/// Return a pointer to the symbol zSymbol in the dynamic library pHandle.
#[unsafe(link_section = ".text.slate_distinct.memdb.memdbDlSym")]
extern "C-unwind" fn memdbDlSym(
    mut pVfs: *mut sqlite3_vfs,
    mut p: *mut (),
    mut zSym: *const i8,
) -> Option<unsafe extern "C-unwind" fn()> {
    return unsafe {
        unsafe { (*((unsafe { (*pVfs).pAppData }) as *mut sqlite3_vfs)).xDlSym }.unwrap()(
            (unsafe { (*pVfs).pAppData }) as *mut sqlite3_vfs,
            p,
            zSym,
        )
    };
}

/// Close the dynamic library handle pHandle.
#[unsafe(link_section = ".text.slate_distinct.memdb.memdbDlClose")]
extern "C-unwind" fn memdbDlClose(mut pVfs: *mut sqlite3_vfs, mut pHandle: *mut ()) {
    unsafe {
        unsafe { (*((unsafe { (*pVfs).pAppData }) as *mut sqlite3_vfs)).xDlClose }.unwrap()(
            (unsafe { (*pVfs).pAppData }) as *mut sqlite3_vfs,
            pHandle,
        )
    };
}

/// Populate the buffer pointed to by zBufOut with nByte bytes of
/// random data.
#[unsafe(link_section = ".text.slate_distinct.memdb.memdbRandomness")]
extern "C-unwind" fn memdbRandomness(
    mut pVfs: *mut sqlite3_vfs,
    mut nByte: i32,
    mut zBufOut: *mut i8,
) -> i32 {
    return unsafe {
        unsafe { (*((unsafe { (*pVfs).pAppData }) as *mut sqlite3_vfs)).xRandomness }.unwrap()(
            (unsafe { (*pVfs).pAppData }) as *mut sqlite3_vfs,
            nByte,
            zBufOut,
        )
    };
}

/// Sleep for nMicro microseconds. Return the number of microseconds
/// actually slept.
#[unsafe(link_section = ".text.slate_distinct.memdb.memdbSleep")]
extern "C-unwind" fn memdbSleep(mut pVfs: *mut sqlite3_vfs, mut nMicro: i32) -> i32 {
    return unsafe {
        unsafe { (*((unsafe { (*pVfs).pAppData }) as *mut sqlite3_vfs)).xSleep }.unwrap()(
            (unsafe { (*pVfs).pAppData }) as *mut sqlite3_vfs,
            nMicro,
        )
    };
}

/// static int memdbCurrentTime(sqlite3_vfs*, double*);
#[unsafe(link_section = ".text.slate_distinct.memdb.memdbGetLastError")]
extern "C-unwind" fn memdbGetLastError(
    mut pVfs: *mut sqlite3_vfs,
    mut a: i32,
    mut b: *mut i8,
) -> i32 {
    return unsafe {
        unsafe { (*((unsafe { (*pVfs).pAppData }) as *mut sqlite3_vfs)).xGetLastError }.unwrap()(
            (unsafe { (*pVfs).pAppData }) as *mut sqlite3_vfs,
            a,
            b,
        )
    };
}

#[unsafe(link_section = ".text.slate_distinct.memdb.memdbCurrentTimeInt64")]
extern "C-unwind" fn memdbCurrentTimeInt64(mut pVfs: *mut sqlite3_vfs, mut p: *mut i64) -> i32 {
    return unsafe {
        unsafe { (*((unsafe { (*pVfs).pAppData }) as *mut sqlite3_vfs)).xCurrentTimeInt64 }.unwrap()(
            (unsafe { (*pVfs).pAppData }) as *mut sqlite3_vfs,
            p,
        )
    };
}

/// Translate a database connection pointer and schema name into a
/// MemFile pointer.
fn memdbFromDbSchema(mut db: *mut sqlite3, mut zSchema: *const i8) -> *mut MemFile {
    let mut p: *mut MemFile = std::ptr::null_mut::<MemFile>();
    let mut pStore: *mut MemStore = unsafe { std::mem::zeroed() };
    let mut rc: i32 = unsafe {
        sqlite3_file_control(db, zSchema, 7 as i32, std::ptr::addr_of_mut!(p) as *mut ())
    };
    if rc != (0 as i32) {
        return std::ptr::null_mut::<MemFile>();
    }
    if (unsafe { (*p).base.pMethods }) != unsafe { std::ptr::addr_of!(memdb_io_methods) } {
        return std::ptr::null_mut::<MemFile>();
    }
    pStore = unsafe { (*p).pStore };
    memdbEnter(pStore);
    if (unsafe { (*pStore).zFName }) != std::ptr::null_mut::<i8>() {
        p = std::ptr::null_mut::<MemFile>();
    }
    memdbLeave(pStore);
    return p;
}

/// Return the serialization of a database
///
/// # Arguments
///
/// * `db` - The database connection
/// * `zSchema` - Which database within the connection
/// * `piSize` - Write size here, if not NULL
/// * `mFlags` - Maybe SQLITE_SERIALIZE_NOCOPY
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.memdb.sqlite3_serialize")]
extern "C-unwind" fn sqlite3_serialize(
    mut db: *mut sqlite3,
    mut zSchema: *const i8,
    mut piSize: *mut i64,
    mut mFlags: u32,
) -> *mut u8 {
    let mut __slate_storage_709: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_709: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_709) as *mut i32;
    let mut __slate_storage_708: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_708: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_708) as *mut i32;
    let mut __slate_storage_549: std::mem::MaybeUninit<*mut u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_549: *mut *mut u8 =
        std::ptr::addr_of_mut!(__slate_storage_549) as *mut *mut u8;
    let mut __slate_storage_548: std::mem::MaybeUninit<*mut PgHdr> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_548: *mut *mut PgHdr =
        std::ptr::addr_of_mut!(__slate_storage_548) as *mut *mut PgHdr;
    let mut __slate_storage_547: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_547: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_547) as *mut i32;
    let mut __slate_storage_546: std::mem::MaybeUninit<*mut Pager> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_546: *mut *mut Pager =
        std::ptr::addr_of_mut!(__slate_storage_546) as *mut *mut Pager;
    let mut __slate_storage_545: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_545: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_545) as *mut i32;
    let mut __slate_storage_707: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_707: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_707) as *mut i32;
    let mut __slate_storage_544: std::mem::MaybeUninit<*mut MemStore> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_544: *mut *mut MemStore =
        std::ptr::addr_of_mut!(__slate_storage_544) as *mut *mut MemStore;
    let mut __slate_storage_543: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_543: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_543) as *mut i32;
    let mut __slate_storage_542: std::mem::MaybeUninit<*mut i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_542: *mut *mut i8 =
        std::ptr::addr_of_mut!(__slate_storage_542) as *mut *mut i8;
    let mut __slate_storage_541: std::mem::MaybeUninit<*mut u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_541: *mut *mut u8 =
        std::ptr::addr_of_mut!(__slate_storage_541) as *mut *mut u8;
    let mut __slate_storage_540: std::mem::MaybeUninit<*mut sqlite3_stmt> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_540: *mut *mut sqlite3_stmt =
        std::ptr::addr_of_mut!(__slate_storage_540) as *mut *mut sqlite3_stmt;
    let mut __slate_storage_539: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_539: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_539) as *mut i32;
    let mut __slate_storage_538: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_538: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_538) as *mut i64;
    let mut __slate_storage_537: std::mem::MaybeUninit<*mut Btree> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_537: *mut *mut Btree =
        std::ptr::addr_of_mut!(__slate_storage_537) as *mut *mut Btree;
    let mut __slate_storage_536: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_536: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_536) as *mut i32;
    let mut __slate_storage_535: std::mem::MaybeUninit<*mut MemFile> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_535: *mut *mut MemFile =
        std::ptr::addr_of_mut!(__slate_storage_535) as *mut *mut MemFile;
    unsafe {
        '__join_31: {
            std::ptr::write(__slate_slot_539, 0 as i32);
            std::ptr::write(__slate_slot_540, std::ptr::null_mut::<sqlite3_stmt>());
            std::ptr::write(__slate_slot_541, std::ptr::null_mut::<u8>());
            unsafe { sqlite3_mutex_enter(unsafe { (*db).mutex }) };
            if zSchema == std::ptr::null::<i8>() {
                zSchema = (unsafe {
                    (*unsafe { unsafe { (*db).aDb }.offset((0 as i32) as isize) }).zDbSName
                }) as *const i8;
            }
        }
        *__slate_slot_535 = memdbFromDbSchema(db, zSchema);
        *__slate_slot_536 = unsafe { sqlite3FindDbName(db, zSchema) };
        if piSize != std::ptr::null_mut::<i64>() {
            unsafe {
                *piSize = -(1 as i32) as i64;
            }
        }
        if *__slate_slot_536 < (0 as i32) {
        } else {
            if *__slate_slot_535 != std::ptr::null_mut::<MemFile>() {
                std::ptr::write(__slate_slot_544, unsafe { (*(*__slate_slot_535)).pStore });
                0 as i32;
                if piSize != std::ptr::null_mut::<i64>() {
                    unsafe {
                        *piSize = unsafe { (*(*__slate_slot_544)).sz };
                    }
                }
                if mFlags & ((1 as i32) as u32) != (0 as u32) {
                    *__slate_slot_541 = unsafe { (*(*__slate_slot_544)).aData };
                } else {
                    *__slate_slot_541 = (unsafe {
                        sqlite3_malloc64((unsafe { (*(*__slate_slot_544)).sz }) as u64)
                    }) as *mut u8;
                    if *__slate_slot_541 != std::ptr::null_mut::<u8>() {
                        unsafe {
                            memcpy(
                                *__slate_slot_541 as *mut (),
                                (unsafe { (*(*__slate_slot_544)).aData }) as *const (),
                                (unsafe { (*(*__slate_slot_544)).sz }) as u64,
                            )
                        };
                    }
                }
            } else {
                *__slate_slot_537 = unsafe {
                    (*unsafe { unsafe { (*db).aDb }.offset(*__slate_slot_536 as isize) }).pBt
                };
                if *__slate_slot_537 == std::ptr::null_mut::<Btree>() {
                } else {
                    *__slate_slot_539 = unsafe { sqlite3BtreeGetPageSize(*__slate_slot_537) };
                    *__slate_slot_542 = unsafe {
                        sqlite3_mprintf(
                            (b"PRAGMA \"%w\".page_count\0".as_ptr() as *mut i8) as *const i8,
                            zSchema,
                        )
                    };
                    if *__slate_slot_542 != std::ptr::null_mut::<i8>() {
                        *__slate_slot_707 = unsafe {
                            sqlite3_prepare_v2(
                                db,
                                *__slate_slot_542 as *const i8,
                                -(1 as i32),
                                std::ptr::addr_of_mut!(*__slate_slot_540),
                                std::ptr::null_mut::<*const i8>(),
                            )
                        };
                    } else {
                        *__slate_slot_707 = 7 as i32;
                    }
                    *__slate_slot_543 = *__slate_slot_707;
                    unsafe { sqlite3_free(*__slate_slot_542 as *mut ()) };
                    if *__slate_slot_543 != (0 as i32) {
                    } else {
                        '__join_1: {
                            *__slate_slot_543 = unsafe { sqlite3_step(*__slate_slot_540) };
                            if *__slate_slot_543 == (100 as i32) {
                                *__slate_slot_538 =
                                    (unsafe { sqlite3_column_int64(*__slate_slot_540, 0 as i32) })
                                        * (*__slate_slot_539 as i64);
                                if *__slate_slot_538 == ((0 as i32) as i64) {
                                    unsafe { sqlite3_reset(*__slate_slot_540) };
                                    unsafe {
                                        sqlite3_exec(
                                            db,
                                            (b"BEGIN IMMEDIATE; COMMIT;\0".as_ptr() as *mut i8)
                                                as *const i8,
                                            None,
                                            std::ptr::null_mut::<()>(),
                                            std::ptr::null_mut::<*mut i8>(),
                                        )
                                    };
                                    *__slate_slot_543 = unsafe { sqlite3_step(*__slate_slot_540) };
                                    if *__slate_slot_543 == (100 as i32) {
                                        *__slate_slot_538 = (unsafe {
                                            sqlite3_column_int64(*__slate_slot_540, 0 as i32)
                                        }) * (*__slate_slot_539 as i64);
                                    }
                                }
                                if piSize != std::ptr::null_mut::<i64>() {
                                    unsafe {
                                        *piSize = *__slate_slot_538;
                                    }
                                }
                                if mFlags & ((1 as i32) as u32) != (0 as u32) {
                                    *__slate_slot_541 = std::ptr::null_mut::<u8>();
                                } else {
                                    *__slate_slot_541 =
                                        (unsafe { sqlite3_malloc64(*__slate_slot_538 as u64) })
                                            as *mut u8;
                                    if *__slate_slot_541 != std::ptr::null_mut::<u8>() {
                                        std::ptr::write(__slate_slot_545, unsafe {
                                            sqlite3_column_int(*__slate_slot_540, 0 as i32)
                                        });
                                        std::ptr::write(__slate_slot_546, unsafe {
                                            sqlite3BtreePager(*__slate_slot_537)
                                        });
                                        *__slate_slot_547 = 1 as i32;
                                        loop {
                                            if *__slate_slot_547 <= *__slate_slot_545 {
                                                std::ptr::write(
                                                    __slate_slot_548,
                                                    std::ptr::null_mut::<PgHdr>(),
                                                );
                                                std::ptr::write(__slate_slot_549, unsafe {
                                                    (*__slate_slot_541).offset(
                                                        ((*__slate_slot_539 as i64)
                                                            * ((*__slate_slot_547 - (1 as i32))
                                                                as i64))
                                                            as isize,
                                                    )
                                                });
                                                *__slate_slot_543 = unsafe {
                                                    sqlite3PagerGet(
                                                        *__slate_slot_546,
                                                        *__slate_slot_547 as u32,
                                                        std::ptr::addr_of_mut!(*__slate_slot_548),
                                                        0 as i32,
                                                    )
                                                };
                                                if *__slate_slot_543 == (0 as i32) {
                                                    unsafe {
                                                        memcpy(
                                                            *__slate_slot_549 as *mut (),
                                                            (unsafe {
                                                                sqlite3PagerGetData(
                                                                    *__slate_slot_548,
                                                                )
                                                            })
                                                                as *const (),
                                                            (*__slate_slot_539 as i64) as u64,
                                                        )
                                                    };
                                                } else {
                                                    unsafe {
                                                        memset(
                                                            *__slate_slot_549 as *mut (),
                                                            0 as i32,
                                                            (*__slate_slot_539 as i64) as u64,
                                                        )
                                                    };
                                                }
                                                unsafe { sqlite3PagerUnref(*__slate_slot_548) };
                                                std::ptr::write(
                                                    __slate_slot_708,
                                                    *__slate_slot_547,
                                                );
                                                std::ptr::write(
                                                    __slate_slot_709,
                                                    *__slate_slot_708 + (1 as i32),
                                                );
                                                *__slate_slot_547 = *__slate_slot_709;
                                            } else {
                                                break '__join_1;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        unsafe { sqlite3_finalize(*__slate_slot_540) };
                    }
                }
            }
        }
        unsafe { sqlite3_mutex_leave(unsafe { (*db).mutex }) };
        return *__slate_slot_541;
    }
    return unsafe { std::mem::zeroed() };
}

/// Convert zSchema to a MemDB and initialize its content.
///
/// # Arguments
///
/// * `db` - The database connection
/// * `zSchema` - Which DB to reopen with the deserialization
/// * `pData` - The serialized database content
/// * `szDb` - Number bytes in the deserialization
/// * `szBuf` - Total size of buffer pData[]
/// * `mFlags` - Zero or more SQLITE_DESERIALIZE_* flags
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.memdb.sqlite3_deserialize")]
extern "C-unwind" fn sqlite3_deserialize(
    mut db: *mut sqlite3,
    mut zSchema: *const i8,
    mut pData: *mut u8,
    mut szDb: i64,
    mut szBuf: i64,
    mut mFlags: u32,
) -> i32 {
    let mut __slate_storage_562: std::mem::MaybeUninit<*mut MemStore> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_562: *mut *mut MemStore =
        std::ptr::addr_of_mut!(__slate_storage_562) as *mut *mut MemStore;
    let mut __slate_storage_561: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_561: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_561) as *mut i32;
    let mut __slate_storage_560: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_560: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_560) as *mut i32;
    let mut __slate_storage_559: std::mem::MaybeUninit<*mut sqlite3_stmt> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_559: *mut *mut sqlite3_stmt =
        std::ptr::addr_of_mut!(__slate_storage_559) as *mut *mut sqlite3_stmt;
    let mut __slate_storage_558: std::mem::MaybeUninit<*mut i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_558: *mut *mut i8 =
        std::ptr::addr_of_mut!(__slate_storage_558) as *mut *mut i8;
    let mut __slate_storage_557: std::mem::MaybeUninit<*mut MemFile> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_557: *mut *mut MemFile =
        std::ptr::addr_of_mut!(__slate_storage_557) as *mut *mut MemFile;
    unsafe {
        '__join_14: {
            std::ptr::write(__slate_slot_559, std::ptr::null_mut::<sqlite3_stmt>());
            unsafe { sqlite3_mutex_enter(unsafe { (*db).mutex }) };
            if zSchema == std::ptr::null::<i8>() {
                zSchema = (unsafe {
                    (*unsafe { unsafe { (*db).aDb }.offset((0 as i32) as isize) }).zDbSName
                }) as *const i8;
            }
        }
        *__slate_slot_561 = unsafe { sqlite3FindDbName(db, zSchema) };
        {}
        if *__slate_slot_561 < (2 as i32) && *__slate_slot_561 != (0 as i32) {
            *__slate_slot_560 = 1 as i32;
        } else {
            *__slate_slot_558 = unsafe {
                sqlite3_mprintf(
                    (b"ATTACH x AS %Q\0".as_ptr() as *mut i8) as *const i8,
                    zSchema,
                )
            };
            if *__slate_slot_558 == std::ptr::null_mut::<i8>() {
                *__slate_slot_560 = 7 as i32;
            } else {
                *__slate_slot_560 = unsafe {
                    sqlite3_prepare_v2(
                        db,
                        *__slate_slot_558 as *const i8,
                        -(1 as i32),
                        std::ptr::addr_of_mut!(*__slate_slot_559),
                        std::ptr::null_mut::<*const i8>(),
                    )
                };
                unsafe { sqlite3_free(*__slate_slot_558 as *mut ()) };
            }
            if *__slate_slot_560 != (0 as i32) {
            } else {
                unsafe {
                    (*db).init.iDb = (*__slate_slot_561 as i8) as u8;
                }
                unsafe {
                    (*db)
                        .init
                        .__slate_bits_0
                        .__set_reopenMemdb((1 as i32) as u32);
                }
                unsafe { sqlite3_step(*__slate_slot_559) };
                unsafe {
                    (*db)
                        .init
                        .__slate_bits_0
                        .__set_reopenMemdb((0 as i32) as u32);
                }
                *__slate_slot_560 = unsafe { sqlite3_finalize(*__slate_slot_559) };
                if *__slate_slot_560 != (0 as i32) {
                } else {
                    *__slate_slot_557 = memdbFromDbSchema(db, zSchema);
                    if *__slate_slot_557 == std::ptr::null_mut::<MemFile>() {
                        *__slate_slot_560 = 1 as i32;
                    } else {
                        std::ptr::write(__slate_slot_562, unsafe { (*(*__slate_slot_557)).pStore });
                        unsafe {
                            (*(*__slate_slot_562)).aData = pData;
                        }
                        pData = std::ptr::null_mut::<u8>();
                        unsafe {
                            (*(*__slate_slot_562)).sz = szDb;
                        }
                        unsafe {
                            (*(*__slate_slot_562)).szAlloc = szBuf;
                        }
                        unsafe {
                            (*(*__slate_slot_562)).szMax = szBuf;
                        }
                        if (unsafe { (*(*__slate_slot_562)).szMax })
                            < unsafe { sqlite3Config.mxMemdbSize }
                        {
                            unsafe {
                                (*(*__slate_slot_562)).szMax = unsafe { sqlite3Config.mxMemdbSize };
                            }
                        }
                        unsafe {
                            (*(*__slate_slot_562)).mFlags = mFlags;
                        }
                        *__slate_slot_560 = 0 as i32;
                    }
                }
            }
        }
        if pData != std::ptr::null_mut::<u8>()
            && mFlags & ((1 as i32) as u32) != ((0 as i32) as u32)
        {
            unsafe { sqlite3_free(pData as *mut ()) };
        }
        unsafe { sqlite3_mutex_leave(unsafe { (*db).mutex }) };
        return *__slate_slot_560;
    }
    return unsafe { std::mem::zeroed() };
}

/// Return true if the VFS is the memvfs.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3IsMemdb(mut pVfs: *const sqlite3_vfs) -> i32 {
    return (pVfs == ((unsafe { std::ptr::addr_of_mut!(memdb_vfs) }) as *const sqlite3_vfs)) as i32;
}

/// This routine is called when the extension is loaded.
/// Register the new VFS.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3MemdbInit() -> i32 {
    let mut pLower: *mut sqlite3_vfs = unsafe { sqlite3_vfs_find(std::ptr::null::<i8>()) };
    let mut sz: u32 = 0 as u32;
    if pLower == std::ptr::null_mut::<sqlite3_vfs>() {
        return 1 as i32;
    }
    sz = (unsafe { (*pLower).szOsFile }) as u32;
    unsafe {
        memdb_vfs.pAppData = pLower as *mut ();
    }
    // The following conditional can only be true when compiled for
    // Windows x86 and SQLITE_MAX_MMAP_SIZE=0.  We always leave
    // it in, to be safe, but it is marked as NO_TEST since there
    // is no way to reach it under most builds.
    if (sz as u64) < (24 as u64) {
        sz = (24 as u64) as u32;
    }
    unsafe {
        memdb_vfs.szOsFile = sz as i32;
    }
    return unsafe { sqlite3_vfs_register(unsafe { std::ptr::addr_of_mut!(memdb_vfs) }, 0 as i32) };
}
