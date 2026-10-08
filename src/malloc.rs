//! 2001 September 15
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
//! Memory allocation functions used throughout sqlite.
unsafe extern "C" {
    static mut sqlite3CtypeMap: [u8; 0];
    static mut sqlite3Config: Sqlite3Config;
    fn sqlite3_initialize() -> i32;
    fn sqlite3_mutex_enter(__v499: *mut sqlite3_mutex);
    fn sqlite3_mutex_leave(__v500: *mut sqlite3_mutex);
    fn sqlite3_status64(op: i32, pCurrent: *mut i64, pHighwater: *mut i64, resetFlag: i32) -> i32;
    fn memcpy(__dest: *mut (), __src: *const (), __n: u64) -> *mut ();
    fn memset(__s: *mut (), __c: i32, __n: u64) -> *mut ();
    fn strlen(__s: *const i8) -> u64;
    fn sqlite3MemSetDefault();
    fn sqlite3MutexAlloc(__v545: i32) -> *mut sqlite3_mutex;
    fn sqlite3StatusValue(__v546: i32) -> i64;
    fn sqlite3StatusUp(__v547: i32, __v548: i32);
    fn sqlite3StatusDown(__v549: i32, __v550: i32);
    fn sqlite3StatusHighwater(__v551: i32, __v552: i32);
    fn sqlite3ErrorMsg(__v556: *mut Parse, __v557: *const i8, ...);
    fn sqlite3Error(__v558: *mut sqlite3, __v559: i32);
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
    trace: __SlateRecord166,
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
    u1: __SlateRecord167,
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
    u: __SlateRecord168,
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
    u: __SlateRecord169,
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
    u: __SlateRecord177,
    pLeft: *mut Expr,
    pRight: *mut Expr,
    x: __SlateRecord178,
    nHeight: i32,
    iTable: i32,
    iColumn: i16,
    iAgg: i16,
    w: __SlateRecord179,
    pAggInfo: *mut AggInfo,
    y: __SlateRecord180,
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
struct RenameToken {}

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
    fg: __SlateRecord187,
    iCursor: i32,
    colUsed: u64,
    u1: __SlateRecord188,
    u2: __SlateRecord189,
    u3: __SlateRecord190,
    u4: __SlateRecord191,
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
struct TriggerPrg {
    pTrigger: *mut Trigger,
    pNext: *mut TriggerPrg,
    pProgram: *mut SubProgram,
    orconf: i32,
    aColmask: [u32; 2],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct TableLock {}

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
    u1: __SlateRecord193,
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
struct VtabCtx {}

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
struct sqlite3InitInfo {
    newTnum: u32,
    iDb: u8,
    busy: u8,
    __slate_bits_0: __slate_bits::__SlateBits165U0,
    azInit: *mut *const i8,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord166 {
    xLegacy: Option<unsafe extern "C-unwind" fn(*mut (), *const i8)>,
    xV2: Option<unsafe extern "C-unwind" fn(u32, *mut (), *mut (), *mut ()) -> i32>,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord167 {
    isInterrupted: i32,
    notUsed1: f64,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord168 {
    pHash: *mut FuncDef,
    pDestructor: *mut FuncDestructor,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord169 {
    tab: __SlateRecord170,
    view: __SlateRecord171,
    vtab: __SlateRecord172,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord170 {
    addColOffset: i32,
    pFKey: *mut FKey,
    pDfltList: *mut ExprList,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord171 {
    pSelect: *mut Select,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord172 {
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
union __SlateRecord177 {
    zToken: *mut i8,
    iValue: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord178 {
    pList: *mut ExprList,
    pSelect: *mut Select,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord179 {
    iJoin: i32,
    iOfst: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord180 {
    pTab: *mut Table,
    pWin: *mut Window,
    nReg: i32,
    sub: __SlateRecord181,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord181 {
    iAddr: i32,
    regReturn: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct ExprList_item {
    pExpr: *mut Expr,
    zEName: *mut i8,
    fg: __SlateRecord183,
    u: __SlateRecord184,
}

#[repr(C, align(4))]
#[derive(Clone, Copy)]
struct __SlateRecord183 {
    sortFlags: u8,
    __slate_bits_0: __slate_bits::__SlateBits183U0,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord184 {
    x: __SlateRecord185,
    iConstExprReg: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord185 {
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
struct __SlateRecord187 {
    jointype: u8,
    __slate_bits_0: __slate_bits::__SlateBits187U0,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord188 {
    zIndexedBy: *mut i8,
    pFuncArg: *mut ExprList,
    nRow: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord189 {
    pIBIndex: *mut Index,
    pCteUse: *mut CteUse,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord190 {
    pOn: *mut Expr,
    pUsing: *mut IdList,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord191 {
    pSchema: *mut Schema,
    zDatabase: *mut i8,
    pSubq: *mut Subquery,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord193 {
    cr: __SlateRecord194,
    d: __SlateRecord195,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord194 {
    addrCrTab: i32,
    regRowid: i32,
    regRoot: i32,
    constraintName: Token,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord195 {
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

/// Attempt to release up to n bytes of non-essential memory currently
/// held by SQLite. An example of non-essential memory is memory used to
/// cache database pages that are not currently in use.
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.malloc.sqlite3_release_memory")]
extern "C-unwind" fn sqlite3_release_memory(mut n: i32) -> i32 {
    // IMPLEMENTATION-OF: R-34391-24921 The sqlite3_release_memory() routine
    // is a no-op returning zero if SQLite is not compiled with
    // SQLITE_ENABLE_MEMORY_MANAGEMENT.
    n;
    return 0 as i32;
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
    pub struct __SlateBits183U0 {
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
    pub struct __SlateBits187U0 {
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
    pub struct __SlateBits165U0 {
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

// Default value of the hard heap limit.  0 means "no limit".
/// State information local to the memory allocation subsystem.
#[repr(C)]
#[derive(Clone, Copy)]
struct Mem0Global {
    /// Mutex to serialize access
    mutex: *mut sqlite3_mutex,
    /// The soft heap limit
    alarmThreshold: i64,
    /// The hard upper bound on memory
    hardLimit: i64,
    /// True if heap is nearly "full" where "full" is defined by the
    /// sqlite3_soft_heap_limit() setting.
    nearlyFull: i32,
}

static mut mem0: Mem0Global = Mem0Global {
    mutex: std::ptr::null_mut::<sqlite3_mutex>(),
    alarmThreshold: (0 as i32) as i64,
    hardLimit: (0 as i32) as i64,
    nearlyFull: 0 as i32,
};

/// Return the memory allocator mutex. sqlite3_status() needs it.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3MallocMutex() -> *mut sqlite3_mutex {
    return unsafe { mem0.mutex };
}

/// Deprecated external interface.  It used to set an alarm callback
/// that was invoked when memory usage grew too large.  Now it is a
/// no-op.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3_memory_alarm(
    mut xCallback: Option<unsafe extern "C-unwind" fn(*mut (), i64, i32)>,
    mut pArg: *mut (),
    mut iThreshold: i64,
) -> i32 {
    xCallback;
    pArg;
    iThreshold;
    return 0 as i32;
}

/// Set the soft heap-size limit for the library.  An argument of
/// zero disables the limit.  A negative argument is a no-op used to
/// obtain the return value.
///
/// The return value is the value of the heap limit just before this
/// interface was called.
///
/// If the hard heap limit is enabled, then the soft heap limit cannot
/// be disabled nor raised above the hard heap limit.
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.malloc.sqlite3_soft_heap_limit64")]
extern "C-unwind" fn sqlite3_soft_heap_limit64(mut n: i64) -> i64 {
    let mut priorLimit: i64 = 0 as i64;
    let mut excess: i64 = 0 as i64;
    let mut nUsed: i64 = 0 as i64;
    let mut rc: i32 = unsafe { sqlite3_initialize() };
    if rc != (0 as i32) {
        return -(1 as i32) as i64;
    }
    unsafe { sqlite3_mutex_enter(unsafe { mem0.mutex }) };
    priorLimit = unsafe { mem0.alarmThreshold };
    if n < ((0 as i32) as i64) {
        unsafe { sqlite3_mutex_leave(unsafe { mem0.mutex }) };
        return priorLimit;
    }
    if (unsafe { mem0.hardLimit }) > ((0 as i32) as i64)
        && (n > unsafe { mem0.hardLimit } || n == ((0 as i32) as i64))
    {
        n = unsafe { mem0.hardLimit };
    }
    unsafe {
        mem0.alarmThreshold = n;
    }
    nUsed = unsafe { sqlite3StatusValue(0 as i32) };
    unsafe {
        std::sync::atomic::AtomicI32::from_ptr(
            (unsafe { std::ptr::addr_of_mut!(mem0.nearlyFull) }) as *mut i32,
        )
        .store(
            (n > ((0 as i32) as i64) && n <= nUsed) as i32,
            std::sync::atomic::Ordering::Relaxed,
        )
    };
    unsafe { sqlite3_mutex_leave(unsafe { mem0.mutex }) };
    excess = sqlite3_memory_used() - n;
    if excess > ((0 as i32) as i64) {
        sqlite3_release_memory((excess & ((2147483647 as i32) as i64)) as i32);
    }
    return priorLimit;
}

#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.malloc.sqlite3_soft_heap_limit")]
extern "C-unwind" fn sqlite3_soft_heap_limit(mut n: i32) {
    if n < (0 as i32) {
        n = 0 as i32;
    }
    sqlite3_soft_heap_limit64(n as i64);
}

/// Set the hard heap-size limit for the library. An argument of zero
/// disables the hard heap limit.  A negative argument is a no-op used
/// to obtain the return value without affecting the hard heap limit.
///
/// The return value is the value of the hard heap limit just prior to
/// calling this interface.
///
/// Setting the hard heap limit will also activate the soft heap limit
/// and constrain the soft heap limit to be no more than the hard heap
/// limit.
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.malloc.sqlite3_hard_heap_limit64")]
extern "C-unwind" fn sqlite3_hard_heap_limit64(mut n: i64) -> i64 {
    let mut priorLimit: i64 = 0 as i64;
    let mut rc: i32 = unsafe { sqlite3_initialize() };
    if rc != (0 as i32) {
        return -(1 as i32) as i64;
    }
    unsafe { sqlite3_mutex_enter(unsafe { mem0.mutex }) };
    priorLimit = unsafe { mem0.hardLimit };
    if n >= ((0 as i32) as i64) {
        unsafe {
            mem0.hardLimit = n;
        }
        if n < unsafe { mem0.alarmThreshold }
            || (unsafe { mem0.alarmThreshold }) == ((0 as i32) as i64)
        {
            unsafe {
                mem0.alarmThreshold = n;
            }
        }
    }
    unsafe { sqlite3_mutex_leave(unsafe { mem0.mutex }) };
    return priorLimit;
}

/// Initialize the memory allocation subsystem.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3MallocInit() -> i32 {
    let mut rc: i32 = 0 as i32;
    if (unsafe { sqlite3Config.m.xMalloc }) == None {
        unsafe { sqlite3MemSetDefault() };
    }
    unsafe {
        mem0.mutex = unsafe { sqlite3MutexAlloc(3 as i32) };
    }
    if (unsafe { sqlite3Config.pPage }) == std::ptr::null_mut::<()>()
        || (unsafe { sqlite3Config.szPage }) < (512 as i32)
        || (unsafe { sqlite3Config.nPage }) <= (0 as i32)
    {
        unsafe {
            sqlite3Config.pPage = std::ptr::null_mut::<()>();
        }
        unsafe {
            sqlite3Config.szPage = 0 as i32;
        }
    }
    rc = unsafe { unsafe { sqlite3Config.m.xInit }.unwrap()(unsafe { sqlite3Config.m.pAppData }) };
    if rc != (0 as i32) {
        unsafe {
            memset(
                (unsafe { std::ptr::addr_of_mut!(mem0) }) as *mut (),
                0 as i32,
                32 as u64,
            )
        };
    }
    return rc;
}

/// Return true if the heap is currently under memory pressure - in other
/// words if the amount of heap used is close to the limit set by
/// sqlite3_soft_heap_limit().
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3HeapNearlyFull() -> i32 {
    return unsafe {
        std::sync::atomic::AtomicI32::from_ptr(
            (unsafe { std::ptr::addr_of_mut!(mem0.nearlyFull) }) as *mut i32,
        )
        .load(std::sync::atomic::Ordering::Relaxed)
    };
}

/// Deinitialize the memory allocation subsystem.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3MallocEnd() {
    if (unsafe { sqlite3Config.m.xShutdown }) != None {
        unsafe {
            unsafe { sqlite3Config.m.xShutdown }.unwrap()(unsafe { sqlite3Config.m.pAppData })
        };
    }
    unsafe {
        memset(
            (unsafe { std::ptr::addr_of_mut!(mem0) }) as *mut (),
            0 as i32,
            32 as u64,
        )
    };
}

/// Return the amount of memory currently checked out.
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.malloc.sqlite3_memory_used")]
extern "C-unwind" fn sqlite3_memory_used() -> i64 {
    let mut res: i64 = 0 as i64;
    let mut mx: i64 = 0 as i64;
    unsafe {
        sqlite3_status64(
            0 as i32,
            std::ptr::addr_of_mut!(res),
            std::ptr::addr_of_mut!(mx),
            0 as i32,
        )
    };
    return res;
}

/// Return the maximum amount of memory that has ever been
/// checked out since either the beginning of this process
/// or since the most recent reset.
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.malloc.sqlite3_memory_highwater")]
extern "C-unwind" fn sqlite3_memory_highwater(mut resetFlag: i32) -> i64 {
    let mut res: i64 = 0 as i64;
    let mut mx: i64 = 0 as i64;
    unsafe {
        sqlite3_status64(
            0 as i32,
            std::ptr::addr_of_mut!(res),
            std::ptr::addr_of_mut!(mx),
            resetFlag,
        )
    };
    return mx;
}

/// Trigger the alarm
fn sqlite3MallocAlarm(mut nByte: i32) {
    if (unsafe { mem0.alarmThreshold }) <= ((0 as i32) as i64) {
        return;
    }
    unsafe { sqlite3_mutex_leave(unsafe { mem0.mutex }) };
    sqlite3_release_memory(nByte);
    unsafe { sqlite3_mutex_enter(unsafe { mem0.mutex }) };
}

// No-op for production builds
/// Do a memory allocation with statistics and alarms.  Assume the
/// lock is already held.
fn mallocWithAlarm(mut n: i32, mut pp: *mut *mut ()) {
    let mut p: *mut () = unsafe { std::mem::zeroed() };
    let mut nFull: i32 = 0 as i32;
    0 as i32;
    0 as i32;
    // In Firefox (circa 2017-02-08), xRoundup() is remapped to an internal
    // implementation of malloc_good_size(), which must be called in debug
    // mode and specifically when the DMD "Dark Matter Detector" is enabled
    // or else a crash results.  Hence, do not attempt to optimize out the
    // following xRoundup() call.
    nFull = unsafe { unsafe { sqlite3Config.m.xRoundup }.unwrap()(n) };
    unsafe { sqlite3StatusHighwater(5 as i32, n) };
    if (unsafe { mem0.alarmThreshold }) > ((0 as i32) as i64) {
        let mut nUsed: i64 = unsafe { sqlite3StatusValue(0 as i32) };
        if nUsed >= (unsafe { mem0.alarmThreshold }) - (nFull as i64) {
            unsafe {
                std::sync::atomic::AtomicI32::from_ptr(
                    (unsafe { std::ptr::addr_of_mut!(mem0.nearlyFull) }) as *mut i32,
                )
                .store(1 as i32, std::sync::atomic::Ordering::Relaxed)
            };
            sqlite3MallocAlarm(nFull);
            if (unsafe { mem0.hardLimit }) != (0 as i64) {
                nUsed = unsafe { sqlite3StatusValue(0 as i32) };
                if nUsed >= (unsafe { mem0.hardLimit }) - (nFull as i64) {
                    {}
                    unsafe {
                        *pp = std::ptr::null_mut::<()>();
                    }
                    return;
                }
            }
        } else {
            unsafe {
                std::sync::atomic::AtomicI32::from_ptr(
                    (unsafe { std::ptr::addr_of_mut!(mem0.nearlyFull) }) as *mut i32,
                )
                .store(0 as i32, std::sync::atomic::Ordering::Relaxed)
            };
        }
    }
    p = unsafe { unsafe { sqlite3Config.m.xMalloc }.unwrap()(nFull) };
    if p != std::ptr::null_mut::<()>() {
        nFull = sqlite3MallocSize(p as *const ());
        unsafe { sqlite3StatusUp(0 as i32, nFull) };
        unsafe { sqlite3StatusUp(9 as i32, 1 as i32) };
    }
    unsafe {
        *pp = p;
    }
}

/// Allocate memory.  This routine is like sqlite3_malloc() except that it
/// assumes the memory subsystem has already been initialized.
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.malloc.sqlite3Malloc")]
extern "C-unwind" fn sqlite3Malloc(mut n: u64) -> *mut () {
    let mut p: *mut () = unsafe { std::mem::zeroed() };
    if n == (((0 as i32) as i64) as u64) || n > (((2147483391 as i32) as i64) as u64) {
        p = std::ptr::null_mut::<()>();
    } else {
        if (unsafe { sqlite3Config.bMemstat }) != (0 as i32) {
            unsafe { sqlite3_mutex_enter(unsafe { mem0.mutex }) };
            mallocWithAlarm((n as u32) as i32, std::ptr::addr_of_mut!(p));
            unsafe { sqlite3_mutex_leave(unsafe { mem0.mutex }) };
        } else {
            p = unsafe { unsafe { sqlite3Config.m.xMalloc }.unwrap()((n as u32) as i32) };
        }
    }
    0 as i32; // IMP: R-11148-40995
    return p;
}

/// This version of the memory allocation is for use by the application.
/// First make sure the memory subsystem is initialized, then do the
/// allocation.
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.malloc.sqlite3_malloc")]
extern "C-unwind" fn sqlite3_malloc(mut n: i32) -> *mut () {
    if (unsafe { sqlite3_initialize() }) != (0 as i32) {
        return std::ptr::null_mut::<()>();
    }
    let __v571: *mut ();
    if n <= (0 as i32) {
        __v571 = std::ptr::null_mut::<()>();
    } else {
        __v571 = sqlite3Malloc((n as i64) as u64);
    }
    return __v571;
}

#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.malloc.sqlite3_malloc64")]
extern "C-unwind" fn sqlite3_malloc64(mut n: u64) -> *mut () {
    if (unsafe { sqlite3_initialize() }) != (0 as i32) {
        return std::ptr::null_mut::<()>();
    }
    return sqlite3Malloc(n);
}

/// TRUE if p is a lookaside memory allocation from db
fn isLookaside(mut db: *mut sqlite3, mut p: *const ()) -> i32 {
    return ((p as u64) >= ((unsafe { (*db).lookaside.pStart }) as u64)
        && (p as u64) < ((unsafe { (*db).lookaside.pTrueEnd }) as u64)) as i32;
}

/// Return the size of a memory allocation previously obtained from
/// sqlite3Malloc() or sqlite3_malloc().
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3MallocSize(mut p: *const ()) -> i32 {
    0 as i32;
    return unsafe { unsafe { sqlite3Config.m.xSize }.unwrap()(p as *mut ()) };
}

fn lookasideMallocSize(mut db: *mut sqlite3, mut p: *const ()) -> i32 {
    return if p < ((unsafe { (*db).lookaside.pMiddle }) as *const ()) {
        ((unsafe { (*db).lookaside.szTrue }) as u32) as i32
    } else {
        128 as i32
    };
}

#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3DbMallocSize(mut db: *mut sqlite3, mut p: *const ()) -> i32 {
    0 as i32;
    if db != std::ptr::null_mut::<sqlite3>() {
        if (p as u64) < ((unsafe { (*db).lookaside.pTrueEnd }) as u64) {
            if (p as u64) >= ((unsafe { (*db).lookaside.pMiddle }) as u64) {
                0 as i32;
                return 128 as i32;
            }
            if (p as u64) >= ((unsafe { (*db).lookaside.pStart }) as u64) {
                0 as i32;
                return ((unsafe { (*db).lookaside.szTrue }) as u32) as i32;
            }
        }
    }
    return unsafe { unsafe { sqlite3Config.m.xSize }.unwrap()(p as *mut ()) };
}

#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.malloc.sqlite3_msize")]
extern "C-unwind" fn sqlite3_msize(mut p: *mut ()) -> u64 {
    0 as i32;
    0 as i32;
    let __v572: i32;
    if p != std::ptr::null_mut::<()>() {
        __v572 = unsafe { unsafe { sqlite3Config.m.xSize }.unwrap()(p) };
    } else {
        __v572 = 0 as i32;
    }
    return (__v572 as i64) as u64;
}

/// Free memory previously obtained from sqlite3Malloc().
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.malloc.sqlite3_free")]
extern "C-unwind" fn sqlite3_free(mut p: *mut ()) {
    if p == std::ptr::null_mut::<()>() {
        return;
    }
    // IMP: R-49053-54554
    0 as i32;
    0 as i32;
    if (unsafe { sqlite3Config.bMemstat }) != (0 as i32) {
        unsafe { sqlite3_mutex_enter(unsafe { mem0.mutex }) };
        unsafe { sqlite3StatusDown(0 as i32, sqlite3MallocSize(p as *const ())) };
        unsafe { sqlite3StatusDown(9 as i32, 1 as i32) };
        unsafe { unsafe { sqlite3Config.m.xFree }.unwrap()(p) };
        unsafe { sqlite3_mutex_leave(unsafe { mem0.mutex }) };
    } else {
        unsafe { unsafe { sqlite3Config.m.xFree }.unwrap()(p) };
    }
}

/// Add the size of memory allocation "p" to the count in
/// *db->pnBytesFreed.
fn measureAllocationSize(mut db: *mut sqlite3, mut p: *mut ()) {
    let __v611: *mut i32 = unsafe { (*db).pnBytesFreed };
    let __v612: i32 = unsafe { *__v611 };
    let __v613: i32 = __v612 + sqlite3DbMallocSize(db, p as *const ());
    unsafe {
        *__v611 = __v613;
    }
}

/// Free memory that might be associated with a particular database
/// connection.  Calling sqlite3DbFree(D,X) for X==0 is a harmless no-op.
/// The sqlite3DbFreeNN(D,X) version requires that X be non-NULL.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3DbFreeNN(mut db: *mut sqlite3, mut p: *mut ()) {
    0 as i32;
    0 as i32;
    if db != std::ptr::null_mut::<sqlite3>() {
        if (p as u64) < ((unsafe { (*db).lookaside.pEnd }) as u64) {
            if (p as u64) >= ((unsafe { (*db).lookaside.pMiddle }) as u64) {
                let mut pBuf: *mut LookasideSlot = p as *mut LookasideSlot;
                0 as i32;
                unsafe {
                    (*pBuf).pNext = unsafe { (*db).lookaside.pSmallFree };
                }
                unsafe {
                    (*db).lookaside.pSmallFree = pBuf;
                }
                return;
            }
            if (p as u64) >= ((unsafe { (*db).lookaside.pStart }) as u64) {
                let mut pBuf: *mut LookasideSlot = p as *mut LookasideSlot;
                0 as i32;
                unsafe {
                    (*pBuf).pNext = unsafe { (*db).lookaside.pFree };
                }
                unsafe {
                    (*db).lookaside.pFree = pBuf;
                }
                return;
            }
        }
        if (unsafe { (*db).pnBytesFreed }) != std::ptr::null_mut::<i32>() {
            measureAllocationSize(db, p);
            return;
        }
    }
    0 as i32;
    0 as i32;
    0 as i32;
    {}
    sqlite3_free(p);
}

#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3DbNNFreeNN(mut db: *mut sqlite3, mut p: *mut ()) {
    0 as i32;
    0 as i32;
    0 as i32;
    if (p as u64) < ((unsafe { (*db).lookaside.pEnd }) as u64) {
        if (p as u64) >= ((unsafe { (*db).lookaside.pMiddle }) as u64) {
            let mut pBuf: *mut LookasideSlot = p as *mut LookasideSlot;
            0 as i32;
            unsafe {
                (*pBuf).pNext = unsafe { (*db).lookaside.pSmallFree };
            }
            unsafe {
                (*db).lookaside.pSmallFree = pBuf;
            }
            return;
        }
        if (p as u64) >= ((unsafe { (*db).lookaside.pStart }) as u64) {
            let mut pBuf: *mut LookasideSlot = p as *mut LookasideSlot;
            0 as i32;
            unsafe {
                (*pBuf).pNext = unsafe { (*db).lookaside.pFree };
            }
            unsafe {
                (*db).lookaside.pFree = pBuf;
            }
            return;
        }
    }
    if (unsafe { (*db).pnBytesFreed }) != std::ptr::null_mut::<i32>() {
        measureAllocationSize(db, p);
        return;
    }
    0 as i32;
    0 as i32;
    {}
    sqlite3_free(p);
}

#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.malloc.sqlite3DbFree")]
extern "C-unwind" fn sqlite3DbFree(mut db: *mut sqlite3, mut p: *mut ()) {
    0 as i32;
    if p != std::ptr::null_mut::<()>() {
        sqlite3DbFreeNN(db, p);
    }
}

/// Change the size of an existing memory allocation
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3Realloc(mut pOld: *mut (), mut nBytes: u64) -> *mut () {
    let mut nOld: i32 = 0 as i32;
    let mut nNew: i32 = 0 as i32;
    let mut nDiff: i32 = 0 as i32;
    let mut pNew: *mut () = unsafe { std::mem::zeroed() };
    0 as i32;
    0 as i32;
    if pOld == std::ptr::null_mut::<()>() {
        return sqlite3Malloc(nBytes); // IMP: R-04300-56712
    }
    if nBytes == (((0 as i32) as i64) as u64) {
        sqlite3_free(pOld); // IMP: R-26507-47431
        return std::ptr::null_mut::<()>();
    }
    if nBytes > (((2147483391 as i32) as i64) as u64) {
        return std::ptr::null_mut::<()>();
    }
    nOld = sqlite3MallocSize(pOld as *const ());
    // IMPLEMENTATION-OF: R-46199-30249 SQLite guarantees that the second
    // argument to xRealloc is always a value returned by a prior call to
    // xRoundup.
    nNew = unsafe { unsafe { sqlite3Config.m.xRoundup }.unwrap()((nBytes as u32) as i32) };
    if nOld == nNew {
        pNew = pOld;
    } else {
        if (unsafe { sqlite3Config.bMemstat }) != (0 as i32) {
            let mut nUsed: i64 = 0 as i64;
            unsafe { sqlite3_mutex_enter(unsafe { mem0.mutex }) };
            unsafe { sqlite3StatusHighwater(5 as i32, (nBytes as u32) as i32) };
            nDiff = nNew - nOld;
            let __v600: bool;
            if nDiff > (0 as i32) {
                let __v601: i64 = unsafe { sqlite3StatusValue(0 as i32) };
                nUsed = __v601;
                __v600 = __v601 >= (unsafe { mem0.alarmThreshold }) - (nDiff as i64);
            } else {
                __v600 = false as bool;
            }
            if __v600 {
                sqlite3MallocAlarm(nDiff);
                if (unsafe { mem0.hardLimit }) > ((0 as i32) as i64)
                    && nUsed >= (unsafe { mem0.hardLimit }) - (nDiff as i64)
                {
                    unsafe { sqlite3_mutex_leave(unsafe { mem0.mutex }) };
                    {}
                    return std::ptr::null_mut::<()>();
                }
            }
            pNew = unsafe { unsafe { sqlite3Config.m.xRealloc }.unwrap()(pOld, nNew) };
            if pNew != std::ptr::null_mut::<()>() {
                nNew = sqlite3MallocSize(pNew as *const ());
                unsafe { sqlite3StatusUp(0 as i32, nNew - nOld) };
            }
            unsafe { sqlite3_mutex_leave(unsafe { mem0.mutex }) };
        } else {
            pNew = unsafe { unsafe { sqlite3Config.m.xRealloc }.unwrap()(pOld, nNew) };
        }
    }
    0 as i32; // IMP: R-11148-40995
    return pNew;
}

/// The public interface to sqlite3Realloc.  Make sure that the memory
/// subsystem is initialized prior to invoking sqliteRealloc.
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.malloc.sqlite3_realloc")]
extern "C-unwind" fn sqlite3_realloc(mut pOld: *mut (), mut n: i32) -> *mut () {
    if (unsafe { sqlite3_initialize() }) != (0 as i32) {
        return std::ptr::null_mut::<()>();
    }
    if n < (0 as i32) {
        n = 0 as i32;
    }
    // IMP: R-26507-47431
    return sqlite3Realloc(pOld, (n as i64) as u64);
}

#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.malloc.sqlite3_realloc64")]
extern "C-unwind" fn sqlite3_realloc64(mut pOld: *mut (), mut n: u64) -> *mut () {
    if (unsafe { sqlite3_initialize() }) != (0 as i32) {
        return std::ptr::null_mut::<()>();
    }
    return sqlite3Realloc(pOld, n);
}

/// Allocate and zero memory.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3MallocZero(mut n: u64) -> *mut () {
    let mut p: *mut () = sqlite3Malloc(n);
    if p != std::ptr::null_mut::<()>() {
        unsafe { memset(p, 0 as i32, n) };
    }
    return p;
}

/// Allocate and zero memory.  If the allocation fails, make
/// the mallocFailed flag in the connection pointer.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3DbMallocZero(mut db: *mut sqlite3, mut n: u64) -> *mut () {
    let mut p: *mut () = unsafe { std::mem::zeroed() };
    {}
    p = sqlite3DbMallocRaw(db, n);
    if p != std::ptr::null_mut::<()>() {
        unsafe { memset(p, 0 as i32, n) };
    }
    return p;
}

/// Finish the work of sqlite3DbMallocRawNN for the unusual and
/// slower case when the allocation cannot be fulfilled using lookaside.
fn dbMallocRawFinish(mut db: *mut sqlite3, mut n: u64) -> *mut () {
    let mut p: *mut () = unsafe { std::mem::zeroed() };
    0 as i32;
    p = sqlite3Malloc(n);
    if !(p != std::ptr::null_mut::<()>()) {
        sqlite3OomFault(db);
    }
    {}
    return p;
}

/// Allocate memory, either lookaside (if possible) or heap.
/// If the allocation fails, set the mallocFailed flag in
/// the connection pointer.
///
/// If db!=0 and db->mallocFailed is true (indicating a prior malloc
/// failure on the same database connection) then always return 0.
/// Hence for a particular database connection, once malloc starts
/// failing, it fails consistently until mallocFailed is reset.
/// This is an important assumption.  There are many places in the
/// code that do things like this:
///
///         int *a = (int*)sqlite3DbMallocRaw(db, 100);
///         int *b = (int*)sqlite3DbMallocRaw(db, 200);
///         if( b ) a[10] = 9;
///
/// In other words, if a subsequent malloc (ex: "b") worked, it is assumed
/// that all prior mallocs (ex: "a") worked too.
///
/// The sqlite3MallocRawNN() variant guarantees that the "db" parameter is
/// not a NULL pointer.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3DbMallocRaw(mut db: *mut sqlite3, mut n: u64) -> *mut () {
    let mut p: *mut () = unsafe { std::mem::zeroed() };
    if db != std::ptr::null_mut::<sqlite3>() {
        return sqlite3DbMallocRawNN(db, n);
    }
    p = sqlite3Malloc(n);
    {}
    return p;
}

#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3DbMallocRawNN(mut db: *mut sqlite3, mut n: u64) -> *mut () {
    let mut pBuf: *mut LookasideSlot = unsafe { std::mem::zeroed() };
    0 as i32;
    0 as i32;
    0 as i32;
    if n > (((((unsafe { (*db).lookaside.sz }) as u32) as i32) as i64) as u64) {
        if !((unsafe { (*db).lookaside.bDisable }) != (0 as u32)) {
            let __v573: *mut u32 = unsafe {
                unsafe { (*db).lookaside.anStat.as_mut_ptr() as *mut u32 }
                    .offset((1 as i32) as isize)
            };
            let __v574: u32 = unsafe { *__v573 };
            let __v575: u32 = __v574.wrapping_add((1 as i32) as u32);
            unsafe {
                *__v573 = __v575;
            }
        } else {
            if (unsafe { (*db).mallocFailed }) != (0 as u8) {
                return std::ptr::null_mut::<()>();
            }
        }
        return dbMallocRawFinish(db, n);
    }
    if n <= (((128 as i32) as i64) as u64) {
        let __v576: *mut LookasideSlot = unsafe { (*db).lookaside.pSmallFree };
        pBuf = __v576;
        if __v576 != std::ptr::null_mut::<LookasideSlot>() {
            unsafe {
                (*db).lookaside.pSmallFree = unsafe { (*pBuf).pNext };
            }
            let __v577: *mut u32 = unsafe {
                unsafe { (*db).lookaside.anStat.as_mut_ptr() as *mut u32 }
                    .offset((0 as i32) as isize)
            };
            let __v578: u32 = unsafe { *__v577 };
            let __v579: u32 = __v578.wrapping_add((1 as i32) as u32);
            unsafe {
                *__v577 = __v579;
            }
            return pBuf as *mut ();
        } else {
            let __v580: *mut LookasideSlot = unsafe { (*db).lookaside.pSmallInit };
            pBuf = __v580;
            if __v580 != std::ptr::null_mut::<LookasideSlot>() {
                unsafe {
                    (*db).lookaside.pSmallInit = unsafe { (*pBuf).pNext };
                }
                let __v581: *mut u32 = unsafe {
                    unsafe { (*db).lookaside.anStat.as_mut_ptr() as *mut u32 }
                        .offset((0 as i32) as isize)
                };
                let __v582: u32 = unsafe { *__v581 };
                let __v583: u32 = __v582.wrapping_add((1 as i32) as u32);
                unsafe {
                    *__v581 = __v583;
                }
                return pBuf as *mut ();
            }
        }
    }
    let __v584: *mut LookasideSlot = unsafe { (*db).lookaside.pFree };
    pBuf = __v584;
    if __v584 != std::ptr::null_mut::<LookasideSlot>() {
        unsafe {
            (*db).lookaside.pFree = unsafe { (*pBuf).pNext };
        }
        let __v585: *mut u32 = unsafe {
            unsafe { (*db).lookaside.anStat.as_mut_ptr() as *mut u32 }.offset((0 as i32) as isize)
        };
        let __v586: u32 = unsafe { *__v585 };
        let __v587: u32 = __v586.wrapping_add((1 as i32) as u32);
        unsafe {
            *__v585 = __v587;
        }
        return pBuf as *mut ();
    } else {
        let __v588: *mut LookasideSlot = unsafe { (*db).lookaside.pInit };
        pBuf = __v588;
        if __v588 != std::ptr::null_mut::<LookasideSlot>() {
            unsafe {
                (*db).lookaside.pInit = unsafe { (*pBuf).pNext };
            }
            let __v589: *mut u32 = unsafe {
                unsafe { (*db).lookaside.anStat.as_mut_ptr() as *mut u32 }
                    .offset((0 as i32) as isize)
            };
            let __v590: u32 = unsafe { *__v589 };
            let __v591: u32 = __v590.wrapping_add((1 as i32) as u32);
            unsafe {
                *__v589 = __v591;
            }
            return pBuf as *mut ();
        } else {
            let __v592: *mut u32 = unsafe {
                unsafe { (*db).lookaside.anStat.as_mut_ptr() as *mut u32 }
                    .offset((2 as i32) as isize)
            };
            let __v593: u32 = unsafe { *__v592 };
            let __v594: u32 = __v593.wrapping_add((1 as i32) as u32);
            unsafe {
                *__v592 = __v594;
            }
        }
    }
    return dbMallocRawFinish(db, n);
}

/// Resize the block of memory pointed to by p to n bytes. If the
/// resize fails, set the mallocFailed flag in the connection object.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3DbRealloc(mut db: *mut sqlite3, mut p: *mut (), mut n: u64) -> *mut () {
    0 as i32;
    if p == std::ptr::null_mut::<()>() {
        return sqlite3DbMallocRawNN(db, n);
    }
    0 as i32;
    if (p as u64) < ((unsafe { (*db).lookaside.pEnd }) as u64) {
        if (p as u64) >= ((unsafe { (*db).lookaside.pMiddle }) as u64) {
            if n <= (((128 as i32) as i64) as u64) {
                return p;
            }
        } else {
            if (p as u64) >= ((unsafe { (*db).lookaside.pStart }) as u64) {
                if n <= (((((unsafe { (*db).lookaside.szTrue }) as u32) as i32) as i64) as u64) {
                    return p;
                }
            }
        }
    }
    return dbReallocFinish(db, p, n);
}

/// Forward declaration
fn dbReallocFinish(mut db: *mut sqlite3, mut p: *mut (), mut n: u64) -> *mut () {
    let mut pNew: *mut () = std::ptr::null_mut::<()>();
    0 as i32;
    0 as i32;
    if (((unsafe { (*db).mallocFailed }) as u32) as i32) == (0 as i32) {
        if isLookaside(db, p as *const ()) != (0 as i32) {
            pNew = sqlite3DbMallocRawNN(db, n);
            if pNew != std::ptr::null_mut::<()>() {
                unsafe {
                    memcpy(
                        pNew,
                        p as *const (),
                        (lookasideMallocSize(db, p as *const ()) as i64) as u64,
                    )
                };
                sqlite3DbFree(db, p);
            }
        } else {
            0 as i32;
            0 as i32;
            {}
            pNew = sqlite3Realloc(p, n);
            if !(pNew != std::ptr::null_mut::<()>()) {
                sqlite3OomFault(db);
            }
            {}
        }
    }
    return pNew;
}

/// Attempt to reallocate p.  If the reallocation fails, then free p
/// and set the mallocFailed flag in the database connection.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3DbReallocOrFree(
    mut db: *mut sqlite3,
    mut p: *mut (),
    mut n: u64,
) -> *mut () {
    let mut pNew: *mut () = unsafe { std::mem::zeroed() };
    pNew = sqlite3DbRealloc(db, p, n);
    if !(pNew != std::ptr::null_mut::<()>()) {
        sqlite3DbFree(db, p);
    }
    return pNew;
}

/// Make a copy of a string in memory obtained from sqliteMalloc(). These
/// functions call sqlite3MallocRaw() directly instead of sqliteMalloc(). This
/// is because when memory debugging is turned on, these two functions are
/// called via macros that record the current file and line number in the
/// ThreadData structure.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3DbStrDup(mut db: *mut sqlite3, mut z: *const i8) -> *mut i8 {
    let mut zNew: *mut i8 = unsafe { std::mem::zeroed() };
    let mut n: u64 = 0 as u64;
    if z == std::ptr::null::<i8>() {
        return std::ptr::null_mut::<i8>();
    }
    n = unsafe { strlen(z) }.wrapping_add(((1 as i32) as i64) as u64);
    zNew = sqlite3DbMallocRaw(db, n) as *mut i8;
    if zNew != std::ptr::null_mut::<i8>() {
        unsafe { memcpy(zNew as *mut (), z as *const (), n) };
    }
    return zNew;
}

#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3DbStrNDup(
    mut db: *mut sqlite3,
    mut z: *const i8,
    mut n: u64,
) -> *mut i8 {
    let mut zNew: *mut i8 = unsafe { std::mem::zeroed() };
    0 as i32;
    0 as i32;
    0 as i32;
    let __v595: *mut ();
    if z != std::ptr::null::<i8>() {
        __v595 = sqlite3DbMallocRawNN(db, n.wrapping_add(((1 as i32) as i64) as u64));
    } else {
        __v595 = std::ptr::null_mut::<()>();
    }
    zNew = __v595 as *mut i8;
    if zNew != std::ptr::null_mut::<i8>() {
        unsafe { memcpy(zNew as *mut (), z as *const (), n) };
        unsafe {
            *unsafe { zNew.offset(n as isize) } = (0 as i32) as i8;
        }
    }
    return zNew;
}

/// The text between zStart and zEnd represents a phrase within a larger
/// SQL statement.  Make a copy of this phrase in space obtained form
/// sqlite3DbMalloc().  Omit leading and trailing whitespace.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3DbSpanDup(
    mut db: *mut sqlite3,
    mut zStart: *const i8,
    mut zEnd: *const i8,
) -> *mut i8 {
    let mut n: i32 = 0 as i32;
    '__slate_break_567: while (((unsafe {
        *unsafe {
            unsafe { std::ptr::addr_of!(sqlite3CtypeMap) as *const u8 }.offset(
                ((((unsafe { *unsafe { zStart.offset((0 as i32) as isize) } }) as u8) as u32)
                    as i32) as isize,
            )
        }
    }) as u32) as i32)
        & (1 as i32)
        != (0 as i32)
    {
        let __v596: *const i8 = zStart;
        let __v597: *const i8 = unsafe { __v596.offset((1 as i32) as isize) };
        zStart = __v597;
    }
    n = ((unsafe { zEnd.offset_from(zStart as *const i8) }) as i64) as i32;
    '__slate_break_568: while (((unsafe {
        *unsafe {
            unsafe { std::ptr::addr_of!(sqlite3CtypeMap) as *const u8 }.offset(
                ((((unsafe { *unsafe { zStart.offset((n - (1 as i32)) as isize) } }) as u8) as u32)
                    as i32) as isize,
            )
        }
    }) as u32) as i32)
        & (1 as i32)
        != (0 as i32)
    {
        let __v598: i32 = n;
        let __v599: i32 = __v598 - (1 as i32);
        n = __v599;
    }
    return sqlite3DbStrNDup(db, zStart, (n as i64) as u64);
}

/// Free any prior content in *pz and replace it with a copy of zNew.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3SetString(
    mut pz: *mut *mut i8,
    mut db: *mut sqlite3,
    mut zNew: *const i8,
) {
    let mut z: *mut i8 = sqlite3DbStrDup(db, zNew);
    sqlite3DbFree(db, (unsafe { *pz }) as *mut ());
    unsafe {
        *pz = z;
    }
}

/// Call this routine to record the fact that an OOM (out-of-memory) error
/// has happened.  This routine will set db->mallocFailed, and also
/// temporarily disable the lookaside memory allocator and interrupt
/// any running VDBEs.
///
/// Always return a NULL pointer so that this routine can be invoked using
///
///      return sqlite3OomFault(db);
///
/// and thereby avoid unnecessary stack frame allocations for the overwhelmingly
/// common case where no OOM occurs.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3OomFault(mut db: *mut sqlite3) -> *mut () {
    if (((unsafe { (*db).mallocFailed }) as u32) as i32) == (0 as i32)
        && (((unsafe { (*db).bBenignMalloc }) as u32) as i32) == (0 as i32)
    {
        unsafe {
            (*db).mallocFailed = ((1 as i32) as i8) as u8;
        }
        if (unsafe { (*db).nVdbeExec }) > (0 as i32) {
            unsafe {
                std::sync::atomic::AtomicI32::store_volatile(
                    std::sync::atomic::AtomicI32::from_ptr_raw(
                        (unsafe { std::ptr::addr_of_mut!((*db).u1.isInterrupted) }) as *mut i32,
                    ),
                    1 as i32,
                    std::sync::atomic::Ordering::Relaxed,
                )
            };
        }
        let __v602: *mut sqlite3 = db;
        let __v603: u32 = unsafe { (*__v602).lookaside.bDisable };
        let __v604: u32 = __v603.wrapping_add((1 as i32) as u32);
        unsafe {
            (*__v602).lookaside.bDisable = __v604;
        }
        unsafe {
            (*db).lookaside.sz = ((0 as i32) as i16) as u16;
        }
        if (unsafe { (*db).pParse }) != std::ptr::null_mut::<Parse>() {
            let mut pParse: *mut Parse = unsafe { std::mem::zeroed() };
            unsafe {
                sqlite3ErrorMsg(
                    unsafe { (*db).pParse },
                    (b"out of memory\0".as_ptr() as *mut i8) as *const i8,
                )
            };
            unsafe {
                (*unsafe { (*db).pParse }).rc = 7 as i32;
            }
            pParse = unsafe { (*unsafe { (*db).pParse }).pOuterParse };
            '__slate_break_570: while pParse != std::ptr::null_mut::<Parse>() {
                let __v605: *mut Parse = pParse;
                let __v606: i32 = unsafe { (*__v605).nErr };
                let __v607: i32 = __v606 + (1 as i32);
                unsafe {
                    (*__v605).nErr = __v607;
                }
                unsafe {
                    (*pParse).rc = 7 as i32;
                }
                pParse = unsafe { (*pParse).pOuterParse };
            }
        }
    }
    return std::ptr::null_mut::<()>();
}

/// This routine reactivates the memory allocator and clears the
/// db->mallocFailed flag as necessary.
///
/// The memory allocator is not restarted if there are running
/// VDBEs.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3OomClear(mut db: *mut sqlite3) {
    if (unsafe { (*db).mallocFailed }) != (0 as u8) && (unsafe { (*db).nVdbeExec }) == (0 as i32) {
        unsafe {
            (*db).mallocFailed = ((0 as i32) as i8) as u8;
        }
        unsafe {
            std::sync::atomic::AtomicI32::store_volatile(
                std::sync::atomic::AtomicI32::from_ptr_raw(
                    (unsafe { std::ptr::addr_of_mut!((*db).u1.isInterrupted) }) as *mut i32,
                ),
                0 as i32,
                std::sync::atomic::Ordering::Relaxed,
            )
        };
        0 as i32;
        let __v608: *mut sqlite3 = db;
        let __v609: u32 = unsafe { (*__v608).lookaside.bDisable };
        let __v610: u32 = __v609.wrapping_sub((1 as i32) as u32);
        unsafe {
            (*__v608).lookaside.bDisable = __v610;
        }
        unsafe {
            (*db).lookaside.sz = ((if (unsafe { (*db).lookaside.bDisable }) != (0 as u32) {
                0 as i32
            } else {
                ((unsafe { (*db).lookaside.szTrue }) as u32) as i32
            }) as i16) as u16;
        }
    }
}

/// Take actions at the end of an API call to deal with error codes.
fn apiHandleError(mut db: *mut sqlite3, mut rc: i32) -> i32 {
    if (unsafe { (*db).mallocFailed }) != (0 as u8) || rc == (10 as i32) | (12 as i32) << (8 as i32)
    {
        sqlite3OomClear(db);
        unsafe { sqlite3Error(db, 7 as i32) };
        return 7 as i32;
    }
    return rc & unsafe { (*db).errMask };
}

/// This function must be called before exiting any API function (i.e.
/// returning control to the user) that has called sqlite3_malloc or
/// sqlite3_realloc.
///
/// The returned value is normally a copy of the second argument to this
/// function. However, if a malloc() failure has occurred since the previous
/// invocation SQLITE_NOMEM is returned instead.
///
/// If an OOM as occurred, then the connection error-code (the value
/// returned by sqlite3_errcode()) is set to SQLITE_NOMEM.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3ApiExit(mut db: *mut sqlite3, mut rc: i32) -> i32 {
    // If the db handle must hold the connection handle mutex here.
    // Otherwise the read (and possible write) of db->mallocFailed
    // is unsafe, as is the call to sqlite3Error().
    0 as i32;
    0 as i32;
    if (unsafe { (*db).mallocFailed }) != (0 as u8) || rc != (0 as i32) {
        return apiHandleError(db, rc);
    }
    return 0 as i32;
}
