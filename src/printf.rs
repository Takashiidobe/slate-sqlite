unsafe extern "C" {
    static mut sqlite3Config: Sqlite3Config;
    static mut sqlite3OomStr: sqlite3_str;
    fn sqlite3_initialize() -> i32;
    fn sqlite3_malloc(__v600: i32) -> *mut ();
    fn sqlite3_malloc64(__v601: u64) -> *mut ();
    fn sqlite3_realloc64(__v602: *mut (), __v603: u64) -> *mut ();
    fn sqlite3_free(__v604: *mut ());
    fn sqlite3_value_double(__v605: *mut sqlite3_value) -> f64;
    fn sqlite3_value_int64(__v606: *mut sqlite3_value) -> i64;
    fn sqlite3_value_text(__v607: *mut sqlite3_value) -> *const u8;
    fn memcpy(__dest: *mut (), __src: *const (), __n: u64) -> *mut ();
    fn memmove(__dest: *mut (), __src: *const (), __n: u64) -> *mut ();
    fn memset(__s: *mut (), __c: i32, __n: u64) -> *mut ();
    fn strchr(__s: *const i8, __c: i32) -> *mut i8;
    fn strlen(__s: *const i8) -> u64;
    fn sqlite3Strlen30(__v644: *const i8) -> i32;
    fn sqlite3DbMallocRaw(__v645: *mut sqlite3, __v646: u64) -> *mut ();
    fn sqlite3Realloc(__v647: *mut (), __v648: u64) -> *mut ();
    fn sqlite3DbRealloc(__v649: *mut sqlite3, __v650: *mut (), __v651: u64) -> *mut ();
    fn sqlite3DbFree(__v652: *mut sqlite3, __v653: *mut ());
    fn sqlite3DbMallocSize(__v654: *mut sqlite3, __v655: *const ()) -> i32;
    fn sqlite3FpDecode(__v656: *mut FpDecode, __v657: f64, __v658: i32, __v659: i32);
    fn sqlite3ErrorToParser(__v665: *mut sqlite3, __v666: i32) -> i32;
    fn sqlite3AppendOneUtf8Character(__v667: *mut i8, __v668: u32) -> i32;
    fn sqlite3OomFault(__v669: *mut sqlite3) -> *mut ();
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
    trace: __SlateRecord178,
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
    u1: __SlateRecord179,
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
struct sqlite3_mutex {}

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
struct sqlite3_value {}

#[repr(C)]
#[derive(Clone, Copy)]
struct sqlite3_context {}

#[repr(C)]
#[derive(Clone, Copy)]
struct sqlite3_vtab {
    pModule: *const sqlite3_module,
    nRef: i32,
    zErrMsg: *mut i8,
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
struct sqlite3_vtab_cursor {
    pVtab: *mut sqlite3_vtab,
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
struct sqlite3_str {
    db: *mut sqlite3,
    zText: *mut i8,
    nAlloc: u32,
    mxAlloc: u32,
    nChar: u32,
    accError: u8,
    printfFlags: u8,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct sqlite3_pcache {}

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
struct _ht {
    count: u32,
    chain: *mut HashElem,
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
struct AutoincInfo {
    pNext: *mut AutoincInfo,
    pTab: *mut Table,
    iDb: i32,
    regCtr: i32,
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
struct Column {
    zCnName: *mut i8,
    __slate_bits_0: __slate_bits::__SlateBits78U0,
    affinity: i8,
    szEst: u8,
    hName: u8,
    iDflt: u16,
    colFlags: u16,
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
struct Db {
    zDbSName: *mut i8,
    pBt: *mut Btree,
    safety_level: u8,
    bSyncSet: u8,
    pSchema: *mut Schema,
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
struct Expr {
    op: u8,
    affExpr: i8,
    op2: u8,
    flags: u32,
    u: __SlateRecord189,
    pLeft: *mut Expr,
    pRight: *mut Expr,
    x: __SlateRecord190,
    nHeight: i32,
    iTable: i32,
    iColumn: i16,
    iAgg: i16,
    w: __SlateRecord191,
    pAggInfo: *mut AggInfo,
    y: __SlateRecord192,
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
struct FpDecode {
    n: i32,
    iDP: i32,
    z: *mut i8,
    zBuf: [i8; 21],
    sign: i8,
    isSpecial: i8,
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
    u: __SlateRecord180,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct IdList {
    nId: i32,
    a: [IdList_item; 0],
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
    __slate_bits_0: __slate_bits::__SlateBits104U0,
    colNotIdxed: u64,
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
    __slate_bits_0: __slate_bits::__SlateBits116U0,
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
    u1: __SlateRecord205,
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
struct ParseCleanup {
    pNext: *mut ParseCleanup,
    pPtr: *mut (),
    xCleanup: Option<unsafe extern "C-unwind" fn(*mut sqlite3, *mut ())>,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct PrintfArguments {
    nArg: i32,
    nUsed: i32,
    apArg: *mut *mut sqlite3_value,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct RCStr {
    nRCRef: u64,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct RenameToken {}

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
struct Savepoint {
    zName: *mut i8,
    nDeferredCons: i64,
    nDeferredImmCons: i64,
    pNext: *mut Savepoint,
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
    fg: __SlateRecord199,
    iCursor: i32,
    colUsed: u64,
    u1: __SlateRecord200,
    u2: __SlateRecord201,
    u3: __SlateRecord202,
    u4: __SlateRecord203,
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
    u: __SlateRecord181,
    pTrigger: *mut Trigger,
    pSchema: *mut Schema,
    aHx: [u8; 16],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct TableLock {}

#[repr(C)]
#[derive(Clone, Copy)]
struct Token {
    z: *const i8,
    n: u32,
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
struct TriggerPrg {
    pTrigger: *mut Trigger,
    pNext: *mut TriggerPrg,
    pProgram: *mut SubProgram,
    orconf: i32,
    aColmask: [u32; 2],
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
struct VtabCtx {}

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
    __slate_bits_0: __slate_bits::__SlateBits177U0,
    azInit: *mut *const i8,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord178 {
    xLegacy: Option<unsafe extern "C-unwind" fn(*mut (), *const i8)>,
    xV2: Option<unsafe extern "C-unwind" fn(u32, *mut (), *mut (), *mut ()) -> i32>,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord179 {
    isInterrupted: i32,
    notUsed1: f64,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord180 {
    pHash: *mut FuncDef,
    pDestructor: *mut FuncDestructor,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord181 {
    tab: __SlateRecord182,
    view: __SlateRecord183,
    vtab: __SlateRecord184,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord182 {
    addColOffset: i32,
    pFKey: *mut FKey,
    pDfltList: *mut ExprList,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord183 {
    pSelect: *mut Select,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord184 {
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
union __SlateRecord189 {
    zToken: *mut i8,
    iValue: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord190 {
    pList: *mut ExprList,
    pSelect: *mut Select,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord191 {
    iJoin: i32,
    iOfst: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord192 {
    pTab: *mut Table,
    pWin: *mut Window,
    nReg: i32,
    sub: __SlateRecord193,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord193 {
    iAddr: i32,
    regReturn: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct ExprList_item {
    pExpr: *mut Expr,
    zEName: *mut i8,
    fg: __SlateRecord195,
    u: __SlateRecord196,
}

#[repr(C, align(4))]
#[derive(Clone, Copy)]
struct __SlateRecord195 {
    sortFlags: u8,
    __slate_bits_0: __slate_bits::__SlateBits195U0,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord196 {
    x: __SlateRecord197,
    iConstExprReg: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord197 {
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
struct __SlateRecord199 {
    jointype: u8,
    __slate_bits_0: __slate_bits::__SlateBits199U0,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord200 {
    zIndexedBy: *mut i8,
    pFuncArg: *mut ExprList,
    nRow: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord201 {
    pIBIndex: *mut Index,
    pCteUse: *mut CteUse,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord202 {
    pOn: *mut Expr,
    pUsing: *mut IdList,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord203 {
    pSchema: *mut Schema,
    zDatabase: *mut i8,
    pSubq: *mut Subquery,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord205 {
    cr: __SlateRecord206,
    d: __SlateRecord207,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord206 {
    addrCrTab: i32,
    regRowid: i32,
    regRoot: i32,
    constraintName: Token,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord207 {
    pReturning: *mut Returning,
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

// /*
// ** The "printf" code that follows dates from the 1980's.  It is in
// ** the public domain.
// **
// **************************************************************************
// **
// ** This file contains code for a set of "printf"-like routines.  These
// ** routines format strings much like the printf() from the standard C
// ** library, though the implementation here has enhancements to support
// ** SQLite.
// */
// /*
// ** Conversion types fall into various categories as defined by the
// ** following enumeration.
// */
// /* non-decimal integer types.  %x %o */
// /* Floating point.  %f */
// /* Exponentional notation. %e and %E */
// /* Floating or exponential, depending on exponent. %g */
// /* Return number of characters processed so far. %n */
// /* Strings. %s */
// /* Dynamically allocated strings. %z */
// /* Percent symbol. %% */
// /* Characters. %c */
// /* The rest are extensions, not normally found in printf() */
// /* Strings with '\'' doubled.  %q */
// /* Strings with '\'' doubled and enclosed in '',
//                             NULL pointers replaced by SQL NULL.  %Q */
// /* a pointer to a Token structure */
// /* a pointer to a SrcItem */
// /* The %p conversion */
// /* %w -> Strings with '\"' doubled */
// /* %r -> 1st, 2nd, 3rd, 4th, etc.  English only */
// /* %d or %u, but not %x, %o */
// /* %j -> JSON string literal w/o "..." */
// /* %J -> JSON string literal with "..." */
// /* Any unrecognized conversion type */
// /*
// ** An "etByte" is an 8-bit unsigned value.
// */
// /*
// ** Each builtin conversion character (ex: the 'd' in "%d") is described
// ** by an instance of the following structure
// */
// /* Information about each format field */
#[repr(C)]
#[derive(Clone, Copy)]
struct et_info {
    fmttype: i8,
    // /* The format field code letter */
    base: u8,
    // /* The base for radix conversion */
    flags: u8,
    // /* One or more of FLAG_ constants below */
    r#type: u8,
    // /* Conversion paradigm */
    charset: u8,
    // /* Offset into aDigits[] of the digits string */
    prefix: u8,
    // /* Offset into aPrefix[] of the prefix string */
    iNxt: i8,
    // /* Next with same hash, or 0 for end of chain */
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
    pub struct __SlateBits78U0 {
        #[bits(4)]
        pub notNull: u32,
        #[bits(4)]
        pub eCType: u32,
    }
    #[bitfields::bitfield([u8; 2], c_names = true, new = false, from_into_bits = false, from_traits = false, default = false, debug = false, builder = false, bit_ops = false)]
    pub struct __SlateBits195U0 {
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
    pub struct __SlateBits199U0 {
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
    pub struct __SlateBits104U0 {
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
    pub struct __SlateBits177U0 {
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
    pub struct __SlateBits116U0 {
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

// /*
// ** Allowed values for et_info.flags
// */
// /* True if the value to convert is signed */
// /* Allow infinite precision */
// /*
// ** The table is searched by hash.  In the case of %C where C is the character
// ** and that character has ASCII value j, then the hash is j%25.
// **
// ** The order of the entries in fmtinfo[] and the hash chain was entered
// ** manually, but based on the output of the following TCL script:
// */
// /*****  Beginning of script ******/
static mut aDigits: __SlateAlign16<[i8; 33]> = __SlateAlign16([
    48 as i8, 49 as i8, 50 as i8, 51 as i8, 52 as i8, 53 as i8, 54 as i8, 55 as i8, 56 as i8,
    57 as i8, 65 as i8, 66 as i8, 67 as i8, 68 as i8, 69 as i8, 70 as i8, 48 as i8, 49 as i8,
    50 as i8, 51 as i8, 52 as i8, 53 as i8, 54 as i8, 55 as i8, 56 as i8, 57 as i8, 97 as i8,
    98 as i8, 99 as i8, 100 as i8, 101 as i8, 102 as i8, 0 as i8,
]);

static mut aHex: __SlateAlign16<[i8; 17]> = __SlateAlign16([
    48 as i8, 49 as i8, 50 as i8, 51 as i8, 52 as i8, 53 as i8, 54 as i8, 55 as i8, 56 as i8,
    57 as i8, 97 as i8, 98 as i8, 99 as i8, 100 as i8, 101 as i8, 102 as i8, 0 as i8,
]);

static mut aPrefix: [i8; 7] = [
    45 as i8, 120 as i8, 48 as i8, 0 as i8, 88 as i8, 48 as i8, 0 as i8,
];

static mut fmtinfo: __SlateAlign16<[et_info; 25]> = __SlateAlign16([
    et_info {
        fmttype: (100 as i32) as i8,
        base: ((10 as i32) as i8) as u8,
        flags: ((1 as i32) as i8) as u8,
        r#type: ((16 as i32) as i8) as u8,
        charset: ((0 as i32) as i8) as u8,
        prefix: ((0 as i32) as i8) as u8,
        iNxt: (0 as i32) as i8,
    },
    et_info {
        fmttype: (101 as i32) as i8,
        base: ((0 as i32) as i8) as u8,
        flags: ((1 as i32) as i8) as u8,
        r#type: ((2 as i32) as i8) as u8,
        charset: ((30 as i32) as i8) as u8,
        prefix: ((0 as i32) as i8) as u8,
        iNxt: (0 as i32) as i8,
    },
    et_info {
        fmttype: (102 as i32) as i8,
        base: ((0 as i32) as i8) as u8,
        flags: ((1 as i32) as i8) as u8,
        r#type: ((1 as i32) as i8) as u8,
        charset: ((0 as i32) as i8) as u8,
        prefix: ((0 as i32) as i8) as u8,
        iNxt: (0 as i32) as i8,
    },
    et_info {
        fmttype: (103 as i32) as i8,
        base: ((0 as i32) as i8) as u8,
        flags: ((1 as i32) as i8) as u8,
        r#type: ((3 as i32) as i8) as u8,
        charset: ((30 as i32) as i8) as u8,
        prefix: ((0 as i32) as i8) as u8,
        iNxt: (0 as i32) as i8,
    },
    et_info {
        fmttype: (106 as i32) as i8,
        base: ((0 as i32) as i8) as u8,
        flags: ((0 as i32) as i8) as u8,
        r#type: ((17 as i32) as i8) as u8,
        charset: ((0 as i32) as i8) as u8,
        prefix: ((0 as i32) as i8) as u8,
        iNxt: (0 as i32) as i8,
    },
    et_info {
        fmttype: (105 as i32) as i8,
        base: ((10 as i32) as i8) as u8,
        flags: ((1 as i32) as i8) as u8,
        r#type: ((16 as i32) as i8) as u8,
        charset: ((0 as i32) as i8) as u8,
        prefix: ((0 as i32) as i8) as u8,
        iNxt: (0 as i32) as i8,
    },
    et_info {
        fmttype: (81 as i32) as i8,
        base: ((0 as i32) as i8) as u8,
        flags: ((4 as i32) as i8) as u8,
        r#type: ((10 as i32) as i8) as u8,
        charset: ((0 as i32) as i8) as u8,
        prefix: ((0 as i32) as i8) as u8,
        iNxt: (4 as i32) as i8,
    },
    et_info {
        fmttype: (112 as i32) as i8,
        base: ((16 as i32) as i8) as u8,
        flags: ((0 as i32) as i8) as u8,
        r#type: ((13 as i32) as i8) as u8,
        charset: ((0 as i32) as i8) as u8,
        prefix: ((1 as i32) as i8) as u8,
        iNxt: (0 as i32) as i8,
    },
    et_info {
        fmttype: (83 as i32) as i8,
        base: ((0 as i32) as i8) as u8,
        flags: ((0 as i32) as i8) as u8,
        r#type: ((12 as i32) as i8) as u8,
        charset: ((0 as i32) as i8) as u8,
        prefix: ((0 as i32) as i8) as u8,
        iNxt: (0 as i32) as i8,
    },
    et_info {
        fmttype: (84 as i32) as i8,
        base: ((0 as i32) as i8) as u8,
        flags: ((0 as i32) as i8) as u8,
        r#type: ((11 as i32) as i8) as u8,
        charset: ((0 as i32) as i8) as u8,
        prefix: ((0 as i32) as i8) as u8,
        iNxt: (0 as i32) as i8,
    },
    et_info {
        fmttype: (110 as i32) as i8,
        base: ((0 as i32) as i8) as u8,
        flags: ((0 as i32) as i8) as u8,
        r#type: ((4 as i32) as i8) as u8,
        charset: ((0 as i32) as i8) as u8,
        prefix: ((0 as i32) as i8) as u8,
        iNxt: (0 as i32) as i8,
    },
    et_info {
        fmttype: (111 as i32) as i8,
        base: ((8 as i32) as i8) as u8,
        flags: ((0 as i32) as i8) as u8,
        r#type: ((0 as i32) as i8) as u8,
        charset: ((0 as i32) as i8) as u8,
        prefix: ((2 as i32) as i8) as u8,
        iNxt: (0 as i32) as i8,
    },
    et_info {
        fmttype: (37 as i32) as i8,
        base: ((0 as i32) as i8) as u8,
        flags: ((0 as i32) as i8) as u8,
        r#type: ((7 as i32) as i8) as u8,
        charset: ((0 as i32) as i8) as u8,
        prefix: ((0 as i32) as i8) as u8,
        iNxt: (7 as i32) as i8,
    },
    et_info {
        fmttype: (113 as i32) as i8,
        base: ((0 as i32) as i8) as u8,
        flags: ((4 as i32) as i8) as u8,
        r#type: ((9 as i32) as i8) as u8,
        charset: ((0 as i32) as i8) as u8,
        prefix: ((0 as i32) as i8) as u8,
        iNxt: (16 as i32) as i8,
    },
    et_info {
        fmttype: (114 as i32) as i8,
        base: ((10 as i32) as i8) as u8,
        flags: ((1 as i32) as i8) as u8,
        r#type: ((15 as i32) as i8) as u8,
        charset: ((0 as i32) as i8) as u8,
        prefix: ((0 as i32) as i8) as u8,
        iNxt: (0 as i32) as i8,
    },
    et_info {
        fmttype: (115 as i32) as i8,
        base: ((0 as i32) as i8) as u8,
        flags: ((4 as i32) as i8) as u8,
        r#type: ((5 as i32) as i8) as u8,
        charset: ((0 as i32) as i8) as u8,
        prefix: ((0 as i32) as i8) as u8,
        iNxt: (0 as i32) as i8,
    },
    et_info {
        fmttype: (88 as i32) as i8,
        base: ((16 as i32) as i8) as u8,
        flags: ((0 as i32) as i8) as u8,
        r#type: ((0 as i32) as i8) as u8,
        charset: ((0 as i32) as i8) as u8,
        prefix: ((4 as i32) as i8) as u8,
        iNxt: (0 as i32) as i8,
    },
    et_info {
        fmttype: (117 as i32) as i8,
        base: ((10 as i32) as i8) as u8,
        flags: ((0 as i32) as i8) as u8,
        r#type: ((16 as i32) as i8) as u8,
        charset: ((0 as i32) as i8) as u8,
        prefix: ((0 as i32) as i8) as u8,
        iNxt: (0 as i32) as i8,
    },
    et_info {
        fmttype: (119 as i32) as i8,
        base: ((0 as i32) as i8) as u8,
        flags: ((4 as i32) as i8) as u8,
        r#type: ((14 as i32) as i8) as u8,
        charset: ((0 as i32) as i8) as u8,
        prefix: ((0 as i32) as i8) as u8,
        iNxt: (0 as i32) as i8,
    },
    et_info {
        fmttype: (69 as i32) as i8,
        base: ((0 as i32) as i8) as u8,
        flags: ((1 as i32) as i8) as u8,
        r#type: ((2 as i32) as i8) as u8,
        charset: ((14 as i32) as i8) as u8,
        prefix: ((0 as i32) as i8) as u8,
        iNxt: (18 as i32) as i8,
    },
    et_info {
        fmttype: (120 as i32) as i8,
        base: ((16 as i32) as i8) as u8,
        flags: ((0 as i32) as i8) as u8,
        r#type: ((0 as i32) as i8) as u8,
        charset: ((16 as i32) as i8) as u8,
        prefix: ((1 as i32) as i8) as u8,
        iNxt: (0 as i32) as i8,
    },
    et_info {
        fmttype: (71 as i32) as i8,
        base: ((0 as i32) as i8) as u8,
        flags: ((1 as i32) as i8) as u8,
        r#type: ((3 as i32) as i8) as u8,
        charset: ((14 as i32) as i8) as u8,
        prefix: ((0 as i32) as i8) as u8,
        iNxt: (0 as i32) as i8,
    },
    et_info {
        fmttype: (122 as i32) as i8,
        base: ((0 as i32) as i8) as u8,
        flags: ((4 as i32) as i8) as u8,
        r#type: ((6 as i32) as i8) as u8,
        charset: ((0 as i32) as i8) as u8,
        prefix: ((0 as i32) as i8) as u8,
        iNxt: (0 as i32) as i8,
    },
    et_info {
        fmttype: (74 as i32) as i8,
        base: ((0 as i32) as i8) as u8,
        flags: ((0 as i32) as i8) as u8,
        r#type: ((18 as i32) as i8) as u8,
        charset: ((0 as i32) as i8) as u8,
        prefix: ((0 as i32) as i8) as u8,
        iNxt: (0 as i32) as i8,
    },
    et_info {
        fmttype: (99 as i32) as i8,
        base: ((0 as i32) as i8) as u8,
        flags: ((0 as i32) as i8) as u8,
        r#type: ((8 as i32) as i8) as u8,
        charset: ((0 as i32) as i8) as u8,
        prefix: ((0 as i32) as i8) as u8,
        iNxt: (23 as i32) as i8,
    },
]);

// /* Accumulate results here */
// /* Format string */
// /* arguments */
static mut zOrd: [i8; 9] = [
    116 as i8, 104 as i8, 115 as i8, 116 as i8, 110 as i8, 100 as i8, 114 as i8, 100 as i8, 0 as i8,
];

// /*
// ** Print into memory obtained from sqlite3_malloc()().  Omit the internal
// ** %-conversion extensions.
// */
#[unsafe(no_mangle)]
unsafe extern "C-unwind" fn sqlite3_mprintf(mut zFormat: *const i8, mut __va_args: ...) -> *mut i8 {
    let mut ap: core::ffi::VaList<'_> = unsafe { std::mem::zeroed() };
    let mut z: *mut i8 = unsafe { std::mem::zeroed() };
    if (unsafe { sqlite3_initialize() }) != (0 as i32) {
        return std::ptr::null_mut::<i8>();
    }
    ap = __va_args.clone();
    z = sqlite3_vmprintf(zFormat, ap.clone());
    {}
    return z;
}

// /*
// ** Print into memory obtained from sqlite3_malloc().  Omit the internal
// ** %-conversion extensions.
// */
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.printf.sqlite3_vmprintf")]
extern "C-unwind" fn sqlite3_vmprintf(
    mut zFormat: *const i8,
    mut ap: core::ffi::VaList<'_>,
) -> *mut i8 {
    let mut z: *mut i8 = unsafe { std::mem::zeroed() };
    let mut zBase: __SlateAlign16<[i8; 70]> = __SlateAlign16([0 as i8; 70]);
    let mut acc: sqlite3_str = unsafe { std::mem::zeroed() };
    if (unsafe { sqlite3_initialize() }) != (0 as i32) {
        return std::ptr::null_mut::<i8>();
    }
    sqlite3StrAccumInit(
        std::ptr::addr_of_mut!(acc),
        std::ptr::null_mut::<sqlite3>(),
        zBase.0.as_mut_ptr() as *mut i8,
        ((70 as u64) as u32) as i32,
        1000000000 as i32,
    );
    sqlite3_str_vappendf(std::ptr::addr_of_mut!(acc), zFormat, ap.clone());
    z = sqlite3StrAccumFinish(std::ptr::addr_of_mut!(acc));
    return z;
}

#[unsafe(no_mangle)]
unsafe extern "C-unwind" fn sqlite3_snprintf(
    mut n: i32,
    mut zBuf: *mut i8,
    mut zFormat: *const i8,
    mut __va_args: ...
) -> *mut i8 {
    let mut acc: sqlite3_str = unsafe { std::mem::zeroed() };
    let mut ap: core::ffi::VaList<'_> = unsafe { std::mem::zeroed() };
    if n <= (0 as i32) {
        return zBuf;
    }
    sqlite3StrAccumInit(
        std::ptr::addr_of_mut!(acc),
        std::ptr::null_mut::<sqlite3>(),
        zBuf,
        n,
        0 as i32,
    );
    ap = __va_args.clone();
    sqlite3_str_vappendf(std::ptr::addr_of_mut!(acc), zFormat, ap.clone());
    {}
    unsafe {
        *unsafe { zBuf.offset(acc.nChar as isize) } = (0 as i32) as i8;
    }
    return zBuf;
}

// /*
// ** sqlite3_snprintf() works like snprintf() except that it ignores the
// ** current locale settings.  This is important for SQLite because we
// ** are not able to use a "," as the decimal point in place of "." as
// ** specified by some locales.
// **
// ** Oops:  The first two arguments of sqlite3_snprintf() are backwards
// ** from the snprintf() standard.  Unfortunately, it is too late to change
// ** this without breaking compatibility, so we just have to live with the
// ** mistake.
// **
// ** sqlite3_vsnprintf() is the varargs version.
// */
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.printf.sqlite3_vsnprintf")]
extern "C-unwind" fn sqlite3_vsnprintf(
    mut n: i32,
    mut zBuf: *mut i8,
    mut zFormat: *const i8,
    mut ap: core::ffi::VaList<'_>,
) -> *mut i8 {
    let mut acc: sqlite3_str = unsafe { std::mem::zeroed() };
    if n <= (0 as i32) {
        return zBuf;
    }
    sqlite3StrAccumInit(
        std::ptr::addr_of_mut!(acc),
        std::ptr::null_mut::<sqlite3>(),
        zBuf,
        n,
        0 as i32,
    );
    sqlite3_str_vappendf(std::ptr::addr_of_mut!(acc), zFormat, ap.clone());
    unsafe {
        *unsafe { zBuf.offset(acc.nChar as isize) } = (0 as i32) as i8;
    }
    return zBuf;
}

// /* Allocate and initialize a new dynamic string object */
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.printf.sqlite3_str_new")]
extern "C-unwind" fn sqlite3_str_new(mut db: *mut sqlite3) -> *mut sqlite3_str {
    let mut p: *mut sqlite3_str = (unsafe { sqlite3_malloc64(32 as u64) }) as *mut sqlite3_str;
    if p != std::ptr::null_mut::<sqlite3_str>() {
        sqlite3StrAccumInit(
            p,
            std::ptr::null_mut::<sqlite3>(),
            std::ptr::null_mut::<i8>(),
            0 as i32,
            if db != std::ptr::null_mut::<sqlite3>() {
                unsafe {
                    *unsafe {
                        unsafe { (*db).aLimit.as_mut_ptr() as *mut i32 }.offset((0 as i32) as isize)
                    }
                }
            } else {
                1000000000 as i32
            },
        );
    } else {
        p = (unsafe { std::ptr::addr_of!(sqlite3OomStr) }) as *mut sqlite3_str;
    }
    return p;
}

// /* Finalize a string created using sqlite3_str_new().
// */
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.printf.sqlite3_str_finish")]
extern "C-unwind" fn sqlite3_str_finish(mut p: *mut sqlite3_str) -> *mut i8 {
    let mut z: *mut i8 = unsafe { std::mem::zeroed() };
    if p != std::ptr::null_mut::<sqlite3_str>()
        && p != ((unsafe { std::ptr::addr_of!(sqlite3OomStr) }) as *mut sqlite3_str)
    {
        z = sqlite3StrAccumFinish(p);
        unsafe { sqlite3_free(p as *mut ()) };
    } else {
        z = std::ptr::null_mut::<i8>();
    }
    return z;
}

// /*
// ** Destroy a dynamically allocate sqlite3_str object and all
// ** of its content, all in one call.
// */
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.printf.sqlite3_str_free")]
extern "C-unwind" fn sqlite3_str_free(mut p: *mut sqlite3_str) {
    if p != std::ptr::null_mut::<sqlite3_str>()
        && p != ((unsafe { std::ptr::addr_of!(sqlite3OomStr) }) as *mut sqlite3_str)
    {
        sqlite3_str_reset(p);
        unsafe { sqlite3_free(p as *mut ()) };
    }
}

// /*
// ** variable-argument wrapper around sqlite3_str_vappendf(). The bFlags argument
// ** can contain the bit SQLITE_PRINTF_INTERNAL enable internal formats.
// */
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.printf.sqlite3_str_appendf")]
unsafe extern "C-unwind" fn sqlite3_str_appendf(
    mut p: *mut sqlite3_str,
    mut zFormat: *const i8,
    mut __va_args: ...
) {
    let mut ap: core::ffi::VaList<'_> = unsafe { std::mem::zeroed() };
    ap = __va_args.clone();
    sqlite3_str_vappendf(p, zFormat, ap.clone());
    {}
}

// /*
// ** On machines with a small stack size, you can redefine the
// ** SQLITE_PRINT_BUF_SIZE to be something smaller, if desired.
// */
// /* Size of the output buffer */
// /*
// ** Hard limit on the precision of floating-point conversions.
// */
// /* Forward reference */
// /*
// ** Render a string given by "fmt" into the StrAccum object.
// */
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.printf.sqlite3_str_vappendf")]
extern "C-unwind" fn sqlite3_str_vappendf(
    mut pAccum: *mut sqlite3_str,
    mut fmt: *const i8,
    mut ap: core::ffi::VaList<'_>,
) {
    let mut __slate_storage_743: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_743: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_743) as *mut i32;
    let mut __slate_storage_745: std::mem::MaybeUninit<*const i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_745: *mut *const i8 =
        std::ptr::addr_of_mut!(__slate_storage_745) as *mut *const i8;
    let mut __slate_storage_744: std::mem::MaybeUninit<*const i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_744: *mut *const i8 =
        std::ptr::addr_of_mut!(__slate_storage_744) as *mut *const i8;
    let mut __slate_storage_970: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_970: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_970) as *mut i64;
    let mut __slate_storage_969: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_969: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_969) as *mut i64;
    let mut __slate_storage_487: std::mem::MaybeUninit<*mut Select> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_487: *mut *mut Select =
        std::ptr::addr_of_mut!(__slate_storage_487) as *mut *mut Select;
    let mut __slate_storage_486: std::mem::MaybeUninit<*mut SrcItem> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_486: *mut *mut SrcItem =
        std::ptr::addr_of_mut!(__slate_storage_486) as *mut *mut SrcItem;
    let mut __slate_storage_484: std::mem::MaybeUninit<*mut Expr> = std::mem::MaybeUninit::uninit();
    let __slate_slot_484: *mut *mut Expr =
        std::ptr::addr_of_mut!(__slate_storage_484) as *mut *mut Expr;
    let mut __slate_storage_485: std::mem::MaybeUninit<*mut Token> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_485: *mut *mut Token =
        std::ptr::addr_of_mut!(__slate_storage_485) as *mut *mut Token;
    let mut __slate_storage_968: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_968: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_968) as *mut i64;
    let mut __slate_storage_967: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_967: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_967) as *mut i64;
    let mut __slate_storage_966: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_966: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_966) as *mut i64;
    let mut __slate_storage_965: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_965: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_965) as *mut i64;
    let mut __slate_storage_940: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_940: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_940) as *mut i64;
    let mut __slate_storage_939: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_939: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_939) as *mut i64;
    let mut __slate_storage_945: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_945: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_945) as *mut i64;
    let mut __slate_storage_944: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_944: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_944) as *mut i64;
    let mut __slate_storage_947: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_947: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_947) as *mut i64;
    let mut __slate_storage_946: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_946: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_946) as *mut i64;
    let mut __slate_storage_957: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_957: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_957) as *mut i64;
    let mut __slate_storage_956: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_956: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_956) as *mut i64;
    let mut __slate_storage_955: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_955: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_955) as *mut i64;
    let mut __slate_storage_954: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_954: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_954) as *mut i64;
    let mut __slate_storage_953: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_953: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_953) as *mut i64;
    let mut __slate_storage_952: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_952: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_952) as *mut i64;
    let mut __slate_storage_951: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_951: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_951) as *mut i64;
    let mut __slate_storage_950: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_950: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_950) as *mut i64;
    let mut __slate_storage_949: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_949: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_949) as *mut i64;
    let mut __slate_storage_948: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_948: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_948) as *mut i64;
    let mut __slate_storage_943: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_943: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_943) as *mut i64;
    let mut __slate_storage_942: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_942: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_942) as *mut i64;
    let mut __slate_storage_941: std::mem::MaybeUninit<i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_941: *mut i8 = std::ptr::addr_of_mut!(__slate_storage_941) as *mut i8;
    let mut __slate_storage_959: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_959: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_959) as *mut i64;
    let mut __slate_storage_958: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_958: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_958) as *mut i64;
    let mut __slate_storage_964: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_964: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_964) as *mut i64;
    let mut __slate_storage_963: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_963: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_963) as *mut i64;
    let mut __slate_storage_962: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_962: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_962) as *mut i64;
    let mut __slate_storage_961: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_961: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_961) as *mut i64;
    let mut __slate_storage_960: std::mem::MaybeUninit<i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_960: *mut i8 = std::ptr::addr_of_mut!(__slate_storage_960) as *mut i8;
    let mut __slate_storage_936: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_936: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_936) as *mut i64;
    let mut __slate_storage_935: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_935: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_935) as *mut i64;
    let mut __slate_storage_938: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_938: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_938) as *mut i64;
    let mut __slate_storage_937: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_937: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_937) as *mut i64;
    let mut __slate_storage_934: std::mem::MaybeUninit<*mut i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_934: *mut *mut i8 =
        std::ptr::addr_of_mut!(__slate_storage_934) as *mut *mut i8;
    let mut __slate_storage_933: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_933: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_933) as *mut i64;
    let mut __slate_storage_932: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_932: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_932) as *mut i64;
    let mut __slate_storage_931: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_931: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_931) as *mut i64;
    let mut __slate_storage_930: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_930: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_930) as *mut i64;
    let mut __slate_storage_929: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_929: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_929) as *mut i64;
    let mut __slate_storage_928: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_928: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_928) as *mut i64;
    let mut __slate_storage_923: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_923: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_923) as *mut i64;
    let mut __slate_storage_922: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_922: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_922) as *mut i64;
    let mut __slate_storage_925: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_925: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_925) as *mut i64;
    let mut __slate_storage_924: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_924: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_924) as *mut i64;
    let mut __slate_storage_927: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_927: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_927) as *mut i64;
    let mut __slate_storage_926: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_926: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_926) as *mut i64;
    let mut __slate_storage_483: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_483: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_483) as *mut i64;
    let mut __slate_storage_482: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_482: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_482) as *mut i64;
    let mut __slate_storage_913: std::mem::MaybeUninit<i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_913: *mut i8 = std::ptr::addr_of_mut!(__slate_storage_913) as *mut i8;
    let mut __slate_storage_912: std::mem::MaybeUninit<bool> = std::mem::MaybeUninit::uninit();
    let __slate_slot_912: *mut bool = std::ptr::addr_of_mut!(__slate_storage_912) as *mut bool;
    let mut __slate_storage_917: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_917: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_917) as *mut i64;
    let mut __slate_storage_916: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_916: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_916) as *mut i64;
    let mut __slate_storage_915: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_915: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_915) as *mut i64;
    let mut __slate_storage_914: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_914: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_914) as *mut i64;
    let mut __slate_storage_921: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_921: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_921) as *mut i64;
    let mut __slate_storage_920: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_920: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_920) as *mut i64;
    let mut __slate_storage_919: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_919: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_919) as *mut i64;
    let mut __slate_storage_918: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_918: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_918) as *mut i64;
    let mut __slate_storage_481: std::mem::MaybeUninit<i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_481: *mut i8 = std::ptr::addr_of_mut!(__slate_storage_481) as *mut i8;
    let mut __slate_storage_480: std::mem::MaybeUninit<*mut i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_480: *mut *mut i8 =
        std::ptr::addr_of_mut!(__slate_storage_480) as *mut *mut i8;
    let mut __slate_storage_479: std::mem::MaybeUninit<i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_479: *mut i8 = std::ptr::addr_of_mut!(__slate_storage_479) as *mut i8;
    let mut __slate_storage_478: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_478: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_478) as *mut i32;
    let mut __slate_storage_477: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_477: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_477) as *mut i64;
    let mut __slate_storage_476: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_476: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_476) as *mut i64;
    let mut __slate_storage_475: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_475: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_475) as *mut i64;
    let mut __slate_storage_474: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_474: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_474) as *mut i64;
    let mut __slate_storage_911: std::mem::MaybeUninit<*mut i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_911: *mut *mut i8 =
        std::ptr::addr_of_mut!(__slate_storage_911) as *mut *mut i8;
    let mut __slate_storage_910: std::mem::MaybeUninit<*mut i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_910: *mut *mut i8 =
        std::ptr::addr_of_mut!(__slate_storage_910) as *mut *mut i8;
    let mut __slate_storage_909: std::mem::MaybeUninit<bool> = std::mem::MaybeUninit::uninit();
    let __slate_slot_909: *mut bool = std::ptr::addr_of_mut!(__slate_storage_909) as *mut bool;
    let mut __slate_storage_473: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_473: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_473) as *mut i64;
    let mut __slate_storage_906: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_906: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_906) as *mut i64;
    let mut __slate_storage_905: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_905: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_905) as *mut i64;
    let mut __slate_storage_908: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_908: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_908) as *mut i64;
    let mut __slate_storage_907: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_907: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_907) as *mut i64;
    let mut __slate_storage_472: std::mem::MaybeUninit<*mut i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_472: *mut *mut i8 =
        std::ptr::addr_of_mut!(__slate_storage_472) as *mut *mut i8;
    let mut __slate_storage_471: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_471: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_471) as *mut i64;
    let mut __slate_storage_470: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_470: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_470) as *mut i64;
    let mut __slate_storage_904: std::mem::MaybeUninit<bool> = std::mem::MaybeUninit::uninit();
    let __slate_slot_904: *mut bool = std::ptr::addr_of_mut!(__slate_storage_904) as *mut bool;
    let mut __slate_storage_902: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_902: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_902) as *mut i64;
    let mut __slate_storage_901: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_901: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_901) as *mut i64;
    let mut __slate_storage_903: std::mem::MaybeUninit<u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_903: *mut u8 = std::ptr::addr_of_mut!(__slate_storage_903) as *mut u8;
    let mut __slate_storage_900: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_900: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_900) as *mut i64;
    let mut __slate_storage_899: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_899: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_899) as *mut i64;
    let mut __slate_storage_896: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_896: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_896) as *mut i64;
    let mut __slate_storage_895: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_895: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_895) as *mut i64;
    let mut __slate_storage_898: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_898: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_898) as *mut i64;
    let mut __slate_storage_897: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_897: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_897) as *mut i64;
    let mut __slate_storage_469: std::mem::MaybeUninit<u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_469: *mut u8 = std::ptr::addr_of_mut!(__slate_storage_469) as *mut u8;
    let mut __slate_storage_468: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_468: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_468) as *mut i64;
    let mut __slate_storage_467: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_467: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_467) as *mut i64;
    let mut __slate_storage_466: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_466: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_466) as *mut i64;
    let mut __slate_storage_465: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_465: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_465) as *mut i64;
    let mut __slate_storage_464: std::mem::MaybeUninit<*mut i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_464: *mut *mut i8 =
        std::ptr::addr_of_mut!(__slate_storage_464) as *mut *mut i8;
    let mut __slate_storage_894: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_894: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_894) as *mut i64;
    let mut __slate_storage_893: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_893: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_893) as *mut i64;
    let mut __slate_storage_892: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_892: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_892) as *mut i64;
    let mut __slate_storage_891: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_891: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_891) as *mut i64;
    let mut __slate_storage_463: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_463: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_463) as *mut i64;
    let mut __slate_storage_884: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_884: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_884) as *mut i64;
    let mut __slate_storage_883: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_883: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_883) as *mut i64;
    let mut __slate_storage_888: std::mem::MaybeUninit<*mut u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_888: *mut *mut u8 =
        std::ptr::addr_of_mut!(__slate_storage_888) as *mut *mut u8;
    let mut __slate_storage_887: std::mem::MaybeUninit<*mut u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_887: *mut *mut u8 =
        std::ptr::addr_of_mut!(__slate_storage_887) as *mut *mut u8;
    let mut __slate_storage_886: std::mem::MaybeUninit<*mut u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_886: *mut *mut u8 =
        std::ptr::addr_of_mut!(__slate_storage_886) as *mut *mut u8;
    let mut __slate_storage_885: std::mem::MaybeUninit<*mut u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_885: *mut *mut u8 =
        std::ptr::addr_of_mut!(__slate_storage_885) as *mut *mut u8;
    let mut __slate_storage_462: std::mem::MaybeUninit<*mut u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_462: *mut *mut u8 =
        std::ptr::addr_of_mut!(__slate_storage_462) as *mut *mut u8;
    let mut __slate_storage_890: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_890: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_890) as *mut i64;
    let mut __slate_storage_889: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_889: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_889) as *mut i64;
    let mut __slate_storage_882: std::mem::MaybeUninit<u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_882: *mut u8 = std::ptr::addr_of_mut!(__slate_storage_882) as *mut u8;
    let mut __slate_storage_881: std::mem::MaybeUninit<u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_881: *mut u8 = std::ptr::addr_of_mut!(__slate_storage_881) as *mut u8;
    let mut __slate_storage_880: std::mem::MaybeUninit<*mut sqlite3_str> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_880: *mut *mut sqlite3_str =
        std::ptr::addr_of_mut!(__slate_storage_880) as *mut *mut sqlite3_str;
    let mut __slate_storage_879: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_879: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_879) as *mut i64;
    let mut __slate_storage_878: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_878: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_878) as *mut i64;
    let mut __slate_storage_877: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_877: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_877) as *mut i64;
    let mut __slate_storage_876: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_876: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_876) as *mut i64;
    let mut __slate_storage_461: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_461: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_461) as *mut i64;
    let mut __slate_storage_875: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_875: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_875) as *mut i64;
    let mut __slate_storage_874: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_874: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_874) as *mut i64;
    let mut __slate_storage_873: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_873: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_873) as *mut i64;
    let mut __slate_storage_872: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_872: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_872) as *mut i64;
    let mut __slate_storage_460: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_460: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_460) as *mut i64;
    let mut __slate_storage_871: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_871: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_871) as *mut i64;
    let mut __slate_storage_870: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_870: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_870) as *mut i64;
    let mut __slate_storage_869: std::mem::MaybeUninit<*mut i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_869: *mut *mut i8 =
        std::ptr::addr_of_mut!(__slate_storage_869) as *mut *mut i8;
    let mut __slate_storage_868: std::mem::MaybeUninit<*mut i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_868: *mut *mut i8 =
        std::ptr::addr_of_mut!(__slate_storage_868) as *mut *mut i8;
    let mut __slate_storage_867: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_867: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_867) as *mut i32;
    let mut __slate_storage_866: std::mem::MaybeUninit<*mut i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_866: *mut *mut i8 =
        std::ptr::addr_of_mut!(__slate_storage_866) as *mut *mut i8;
    let mut __slate_storage_865: std::mem::MaybeUninit<*mut i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_865: *mut *mut i8 =
        std::ptr::addr_of_mut!(__slate_storage_865) as *mut *mut i8;
    let mut __slate_storage_459: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_459: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_459) as *mut u32;
    let mut __slate_storage_864: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_864: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_864) as *mut u32;
    let mut __slate_storage_863: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_863: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_863) as *mut u32;
    let mut __slate_storage_862: std::mem::MaybeUninit<*mut sqlite3_str> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_862: *mut *mut sqlite3_str =
        std::ptr::addr_of_mut!(__slate_storage_862) as *mut *mut sqlite3_str;
    let mut __slate_storage_458: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_458: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_458) as *mut i32;
    let mut __slate_storage_457: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_457: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_457) as *mut i64;
    let mut __slate_storage_861: std::mem::MaybeUninit<*mut i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_861: *mut *mut i8 =
        std::ptr::addr_of_mut!(__slate_storage_861) as *mut *mut i8;
    let mut __slate_storage_860: std::mem::MaybeUninit<*mut i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_860: *mut *mut i8 =
        std::ptr::addr_of_mut!(__slate_storage_860) as *mut *mut i8;
    let mut __slate_storage_859: std::mem::MaybeUninit<*mut i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_859: *mut *mut i8 =
        std::ptr::addr_of_mut!(__slate_storage_859) as *mut *mut i8;
    let mut __slate_storage_858: std::mem::MaybeUninit<*mut i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_858: *mut *mut i8 =
        std::ptr::addr_of_mut!(__slate_storage_858) as *mut *mut i8;
    let mut __slate_storage_857: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_857: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_857) as *mut i32;
    let mut __slate_storage_856: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_856: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_856) as *mut i32;
    let mut __slate_storage_855: std::mem::MaybeUninit<*mut i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_855: *mut *mut i8 =
        std::ptr::addr_of_mut!(__slate_storage_855) as *mut *mut i8;
    let mut __slate_storage_854: std::mem::MaybeUninit<*mut i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_854: *mut *mut i8 =
        std::ptr::addr_of_mut!(__slate_storage_854) as *mut *mut i8;
    let mut __slate_storage_851: std::mem::MaybeUninit<*mut i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_851: *mut *mut i8 =
        std::ptr::addr_of_mut!(__slate_storage_851) as *mut *mut i8;
    let mut __slate_storage_850: std::mem::MaybeUninit<*mut i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_850: *mut *mut i8 =
        std::ptr::addr_of_mut!(__slate_storage_850) as *mut *mut i8;
    let mut __slate_storage_853: std::mem::MaybeUninit<*mut i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_853: *mut *mut i8 =
        std::ptr::addr_of_mut!(__slate_storage_853) as *mut *mut i8;
    let mut __slate_storage_852: std::mem::MaybeUninit<*mut i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_852: *mut *mut i8 =
        std::ptr::addr_of_mut!(__slate_storage_852) as *mut *mut i8;
    let mut __slate_storage_849: std::mem::MaybeUninit<*mut i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_849: *mut *mut i8 =
        std::ptr::addr_of_mut!(__slate_storage_849) as *mut *mut i8;
    let mut __slate_storage_848: std::mem::MaybeUninit<*mut i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_848: *mut *mut i8 =
        std::ptr::addr_of_mut!(__slate_storage_848) as *mut *mut i8;
    let mut __slate_storage_845: std::mem::MaybeUninit<*mut i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_845: *mut *mut i8 =
        std::ptr::addr_of_mut!(__slate_storage_845) as *mut *mut i8;
    let mut __slate_storage_844: std::mem::MaybeUninit<*mut i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_844: *mut *mut i8 =
        std::ptr::addr_of_mut!(__slate_storage_844) as *mut *mut i8;
    let mut __slate_storage_847: std::mem::MaybeUninit<*mut i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_847: *mut *mut i8 =
        std::ptr::addr_of_mut!(__slate_storage_847) as *mut *mut i8;
    let mut __slate_storage_846: std::mem::MaybeUninit<*mut i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_846: *mut *mut i8 =
        std::ptr::addr_of_mut!(__slate_storage_846) as *mut *mut i8;
    let mut __slate_storage_843: std::mem::MaybeUninit<*mut i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_843: *mut *mut i8 =
        std::ptr::addr_of_mut!(__slate_storage_843) as *mut *mut i8;
    let mut __slate_storage_842: std::mem::MaybeUninit<*mut i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_842: *mut *mut i8 =
        std::ptr::addr_of_mut!(__slate_storage_842) as *mut *mut i8;
    let mut __slate_storage_841: std::mem::MaybeUninit<*mut i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_841: *mut *mut i8 =
        std::ptr::addr_of_mut!(__slate_storage_841) as *mut *mut i8;
    let mut __slate_storage_840: std::mem::MaybeUninit<*mut i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_840: *mut *mut i8 =
        std::ptr::addr_of_mut!(__slate_storage_840) as *mut *mut i8;
    let mut __slate_storage_839: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_839: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_839) as *mut i64;
    let mut __slate_storage_838: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_838: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_838) as *mut i64;
    let mut __slate_storage_837: std::mem::MaybeUninit<*mut i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_837: *mut *mut i8 =
        std::ptr::addr_of_mut!(__slate_storage_837) as *mut *mut i8;
    let mut __slate_storage_836: std::mem::MaybeUninit<*mut i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_836: *mut *mut i8 =
        std::ptr::addr_of_mut!(__slate_storage_836) as *mut *mut i8;
    let mut __slate_storage_456: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_456: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_456) as *mut i32;
    let mut __slate_storage_835: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_835: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_835) as *mut i64;
    let mut __slate_storage_834: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_834: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_834) as *mut i64;
    let mut __slate_storage_833: std::mem::MaybeUninit<*mut i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_833: *mut *mut i8 =
        std::ptr::addr_of_mut!(__slate_storage_833) as *mut *mut i8;
    let mut __slate_storage_832: std::mem::MaybeUninit<*mut i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_832: *mut *mut i8 =
        std::ptr::addr_of_mut!(__slate_storage_832) as *mut *mut i8;
    let mut __slate_storage_455: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_455: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_455) as *mut i32;
    let mut __slate_storage_831: std::mem::MaybeUninit<*mut i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_831: *mut *mut i8 =
        std::ptr::addr_of_mut!(__slate_storage_831) as *mut *mut i8;
    let mut __slate_storage_830: std::mem::MaybeUninit<*mut i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_830: *mut *mut i8 =
        std::ptr::addr_of_mut!(__slate_storage_830) as *mut *mut i8;
    let mut __slate_storage_814: std::mem::MaybeUninit<*mut i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_814: *mut *mut i8 =
        std::ptr::addr_of_mut!(__slate_storage_814) as *mut *mut i8;
    let mut __slate_storage_813: std::mem::MaybeUninit<*mut i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_813: *mut *mut i8 =
        std::ptr::addr_of_mut!(__slate_storage_813) as *mut *mut i8;
    let mut __slate_storage_816: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_816: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_816) as *mut i32;
    let mut __slate_storage_815: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_815: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_815) as *mut i32;
    let mut __slate_storage_823: std::mem::MaybeUninit<*mut i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_823: *mut *mut i8 =
        std::ptr::addr_of_mut!(__slate_storage_823) as *mut *mut i8;
    let mut __slate_storage_822: std::mem::MaybeUninit<*mut i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_822: *mut *mut i8 =
        std::ptr::addr_of_mut!(__slate_storage_822) as *mut *mut i8;
    let mut __slate_storage_821: std::mem::MaybeUninit<*mut i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_821: *mut *mut i8 =
        std::ptr::addr_of_mut!(__slate_storage_821) as *mut *mut i8;
    let mut __slate_storage_820: std::mem::MaybeUninit<*mut i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_820: *mut *mut i8 =
        std::ptr::addr_of_mut!(__slate_storage_820) as *mut *mut i8;
    let mut __slate_storage_819: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_819: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_819) as *mut i32;
    let mut __slate_storage_818: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_818: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_818) as *mut i32;
    let mut __slate_storage_817: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_817: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_817) as *mut i32;
    let mut __slate_storage_829: std::mem::MaybeUninit<*mut i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_829: *mut *mut i8 =
        std::ptr::addr_of_mut!(__slate_storage_829) as *mut *mut i8;
    let mut __slate_storage_828: std::mem::MaybeUninit<*mut i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_828: *mut *mut i8 =
        std::ptr::addr_of_mut!(__slate_storage_828) as *mut *mut i8;
    let mut __slate_storage_827: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_827: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_827) as *mut i32;
    let mut __slate_storage_826: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_826: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_826) as *mut i32;
    let mut __slate_storage_825: std::mem::MaybeUninit<*mut i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_825: *mut *mut i8 =
        std::ptr::addr_of_mut!(__slate_storage_825) as *mut *mut i8;
    let mut __slate_storage_824: std::mem::MaybeUninit<*mut i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_824: *mut *mut i8 =
        std::ptr::addr_of_mut!(__slate_storage_824) as *mut *mut i8;
    let mut __slate_storage_812: std::mem::MaybeUninit<*mut i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_812: *mut *mut i8 =
        std::ptr::addr_of_mut!(__slate_storage_812) as *mut *mut i8;
    let mut __slate_storage_811: std::mem::MaybeUninit<*mut i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_811: *mut *mut i8 =
        std::ptr::addr_of_mut!(__slate_storage_811) as *mut *mut i8;
    let mut __slate_storage_810: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_810: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_810) as *mut i64;
    let mut __slate_storage_809: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_809: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_809) as *mut i64;
    let mut __slate_storage_808: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_808: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_808) as *mut i64;
    let mut __slate_storage_807: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_807: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_807) as *mut i64;
    let mut __slate_storage_806: std::mem::MaybeUninit<*mut i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_806: *mut *mut i8 =
        std::ptr::addr_of_mut!(__slate_storage_806) as *mut *mut i8;
    let mut __slate_storage_805: std::mem::MaybeUninit<*mut i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_805: *mut *mut i8 =
        std::ptr::addr_of_mut!(__slate_storage_805) as *mut *mut i8;
    let mut __slate_storage_454: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_454: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_454) as *mut i64;
    let mut __slate_storage_453: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_453: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_453) as *mut i32;
    let mut __slate_storage_452: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_452: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_452) as *mut i32;
    let mut __slate_storage_451: std::mem::MaybeUninit<FpDecode> = std::mem::MaybeUninit::uninit();
    let __slate_slot_451: *mut FpDecode =
        std::ptr::addr_of_mut!(__slate_storage_451) as *mut FpDecode;
    let mut __slate_storage_800: std::mem::MaybeUninit<i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_800: *mut i8 = std::ptr::addr_of_mut!(__slate_storage_800) as *mut i8;
    let mut __slate_storage_802: std::mem::MaybeUninit<*const i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_802: *mut *const i8 =
        std::ptr::addr_of_mut!(__slate_storage_802) as *mut *const i8;
    let mut __slate_storage_801: std::mem::MaybeUninit<*const i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_801: *mut *const i8 =
        std::ptr::addr_of_mut!(__slate_storage_801) as *mut *const i8;
    let mut __slate_storage_804: std::mem::MaybeUninit<*mut i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_804: *mut *mut i8 =
        std::ptr::addr_of_mut!(__slate_storage_804) as *mut *mut i8;
    let mut __slate_storage_803: std::mem::MaybeUninit<*mut i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_803: *mut *mut i8 =
        std::ptr::addr_of_mut!(__slate_storage_803) as *mut *mut i8;
    let mut __slate_storage_450: std::mem::MaybeUninit<i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_450: *mut i8 = std::ptr::addr_of_mut!(__slate_storage_450) as *mut i8;
    let mut __slate_storage_449: std::mem::MaybeUninit<*const i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_449: *mut *const i8 =
        std::ptr::addr_of_mut!(__slate_storage_449) as *mut *const i8;
    let mut __slate_storage_799: std::mem::MaybeUninit<*mut i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_799: *mut *mut i8 =
        std::ptr::addr_of_mut!(__slate_storage_799) as *mut *mut i8;
    let mut __slate_storage_798: std::mem::MaybeUninit<*mut i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_798: *mut *mut i8 =
        std::ptr::addr_of_mut!(__slate_storage_798) as *mut *mut i8;
    let mut __slate_storage_791: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_791: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_791) as *mut i32;
    let mut __slate_storage_790: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_790: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_790) as *mut i32;
    let mut __slate_storage_797: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_797: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_797) as *mut i64;
    let mut __slate_storage_796: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_796: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_796) as *mut i64;
    let mut __slate_storage_795: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_795: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_795) as *mut i32;
    let mut __slate_storage_794: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_794: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_794) as *mut i32;
    let mut __slate_storage_793: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_793: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_793) as *mut i64;
    let mut __slate_storage_792: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_792: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_792) as *mut i64;
    let mut __slate_storage_789: std::mem::MaybeUninit<*mut i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_789: *mut *mut i8 =
        std::ptr::addr_of_mut!(__slate_storage_789) as *mut *mut i8;
    let mut __slate_storage_788: std::mem::MaybeUninit<*mut i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_788: *mut *mut i8 =
        std::ptr::addr_of_mut!(__slate_storage_788) as *mut *mut i8;
    let mut __slate_storage_448: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_448: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_448) as *mut i32;
    let mut __slate_storage_447: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_447: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_447) as *mut i64;
    let mut __slate_storage_446: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_446: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_446) as *mut i64;
    let mut __slate_storage_787: std::mem::MaybeUninit<*mut i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_787: *mut *mut i8 =
        std::ptr::addr_of_mut!(__slate_storage_787) as *mut *mut i8;
    let mut __slate_storage_786: std::mem::MaybeUninit<*mut i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_786: *mut *mut i8 =
        std::ptr::addr_of_mut!(__slate_storage_786) as *mut *mut i8;
    let mut __slate_storage_445: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_445: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_445) as *mut i64;
    let mut __slate_storage_785: std::mem::MaybeUninit<*mut i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_785: *mut *mut i8 =
        std::ptr::addr_of_mut!(__slate_storage_785) as *mut *mut i8;
    let mut __slate_storage_784: std::mem::MaybeUninit<*mut i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_784: *mut *mut i8 =
        std::ptr::addr_of_mut!(__slate_storage_784) as *mut *mut i8;
    let mut __slate_storage_444: std::mem::MaybeUninit<u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_444: *mut u8 = std::ptr::addr_of_mut!(__slate_storage_444) as *mut u8;
    let mut __slate_storage_443: std::mem::MaybeUninit<*const i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_443: *mut *const i8 =
        std::ptr::addr_of_mut!(__slate_storage_443) as *mut *const i8;
    let mut __slate_storage_783: std::mem::MaybeUninit<*mut i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_783: *mut *mut i8 =
        std::ptr::addr_of_mut!(__slate_storage_783) as *mut *mut i8;
    let mut __slate_storage_782: std::mem::MaybeUninit<*mut i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_782: *mut *mut i8 =
        std::ptr::addr_of_mut!(__slate_storage_782) as *mut *mut i8;
    let mut __slate_storage_781: std::mem::MaybeUninit<*mut i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_781: *mut *mut i8 =
        std::ptr::addr_of_mut!(__slate_storage_781) as *mut *mut i8;
    let mut __slate_storage_780: std::mem::MaybeUninit<*mut i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_780: *mut *mut i8 =
        std::ptr::addr_of_mut!(__slate_storage_780) as *mut *mut i8;
    let mut __slate_storage_442: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_442: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_442) as *mut i32;
    let mut __slate_storage_779: std::mem::MaybeUninit<*mut i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_779: *mut *mut i8 =
        std::ptr::addr_of_mut!(__slate_storage_779) as *mut *mut i8;
    let mut __slate_storage_778: std::mem::MaybeUninit<u64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_778: *mut u64 = std::ptr::addr_of_mut!(__slate_storage_778) as *mut u64;
    let mut __slate_storage_777: std::mem::MaybeUninit<u64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_777: *mut u64 = std::ptr::addr_of_mut!(__slate_storage_777) as *mut u64;
    let mut __slate_storage_440: std::mem::MaybeUninit<u64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_440: *mut u64 = std::ptr::addr_of_mut!(__slate_storage_440) as *mut u64;
    let mut __slate_storage_776: std::mem::MaybeUninit<u64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_776: *mut u64 = std::ptr::addr_of_mut!(__slate_storage_776) as *mut u64;
    let mut __slate_storage_775: std::mem::MaybeUninit<u64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_775: *mut u64 = std::ptr::addr_of_mut!(__slate_storage_775) as *mut u64;
    let mut __slate_storage_439: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_439: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_439) as *mut i64;
    let mut __slate_storage_774: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_774: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_774) as *mut i32;
    let mut __slate_storage_773: std::mem::MaybeUninit<bool> = std::mem::MaybeUninit::uninit();
    let __slate_slot_773: *mut bool = std::ptr::addr_of_mut!(__slate_storage_773) as *mut bool;
    let mut __slate_storage_772: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_772: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_772) as *mut i32;
    let mut __slate_storage_771: std::mem::MaybeUninit<*const i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_771: *mut *const i8 =
        std::ptr::addr_of_mut!(__slate_storage_771) as *mut *const i8;
    let mut __slate_storage_770: std::mem::MaybeUninit<*const i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_770: *mut *const i8 =
        std::ptr::addr_of_mut!(__slate_storage_770) as *mut *const i8;
    let mut __slate_storage_769: std::mem::MaybeUninit<bool> = std::mem::MaybeUninit::uninit();
    let __slate_slot_769: *mut bool = std::ptr::addr_of_mut!(__slate_storage_769) as *mut bool;
    let mut __slate_storage_768: std::mem::MaybeUninit<*const i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_768: *mut *const i8 =
        std::ptr::addr_of_mut!(__slate_storage_768) as *mut *const i8;
    let mut __slate_storage_767: std::mem::MaybeUninit<*const i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_767: *mut *const i8 =
        std::ptr::addr_of_mut!(__slate_storage_767) as *mut *const i8;
    let mut __slate_storage_764: std::mem::MaybeUninit<*const i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_764: *mut *const i8 =
        std::ptr::addr_of_mut!(__slate_storage_764) as *mut *const i8;
    let mut __slate_storage_763: std::mem::MaybeUninit<*const i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_763: *mut *const i8 =
        std::ptr::addr_of_mut!(__slate_storage_763) as *mut *const i8;
    let mut __slate_storage_766: std::mem::MaybeUninit<*const i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_766: *mut *const i8 =
        std::ptr::addr_of_mut!(__slate_storage_766) as *mut *const i8;
    let mut __slate_storage_765: std::mem::MaybeUninit<*const i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_765: *mut *const i8 =
        std::ptr::addr_of_mut!(__slate_storage_765) as *mut *const i8;
    let mut __slate_storage_438: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_438: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_438) as *mut u32;
    let mut __slate_storage_762: std::mem::MaybeUninit<*const i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_762: *mut *const i8 =
        std::ptr::addr_of_mut!(__slate_storage_762) as *mut *const i8;
    let mut __slate_storage_761: std::mem::MaybeUninit<*const i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_761: *mut *const i8 =
        std::ptr::addr_of_mut!(__slate_storage_761) as *mut *const i8;
    let mut __slate_storage_760: std::mem::MaybeUninit<*const i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_760: *mut *const i8 =
        std::ptr::addr_of_mut!(__slate_storage_760) as *mut *const i8;
    let mut __slate_storage_759: std::mem::MaybeUninit<*const i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_759: *mut *const i8 =
        std::ptr::addr_of_mut!(__slate_storage_759) as *mut *const i8;
    let mut __slate_storage_758: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_758: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_758) as *mut i32;
    let mut __slate_storage_757: std::mem::MaybeUninit<*const i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_757: *mut *const i8 =
        std::ptr::addr_of_mut!(__slate_storage_757) as *mut *const i8;
    let mut __slate_storage_756: std::mem::MaybeUninit<*const i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_756: *mut *const i8 =
        std::ptr::addr_of_mut!(__slate_storage_756) as *mut *const i8;
    let mut __slate_storage_755: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_755: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_755) as *mut i32;
    let mut __slate_storage_754: std::mem::MaybeUninit<*const i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_754: *mut *const i8 =
        std::ptr::addr_of_mut!(__slate_storage_754) as *mut *const i8;
    let mut __slate_storage_753: std::mem::MaybeUninit<*const i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_753: *mut *const i8 =
        std::ptr::addr_of_mut!(__slate_storage_753) as *mut *const i8;
    let mut __slate_storage_437: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_437: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_437) as *mut u32;
    let mut __slate_storage_752: std::mem::MaybeUninit<*const i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_752: *mut *const i8 =
        std::ptr::addr_of_mut!(__slate_storage_752) as *mut *const i8;
    let mut __slate_storage_751: std::mem::MaybeUninit<*const i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_751: *mut *const i8 =
        std::ptr::addr_of_mut!(__slate_storage_751) as *mut *const i8;
    let mut __slate_storage_750: std::mem::MaybeUninit<*const i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_750: *mut *const i8 =
        std::ptr::addr_of_mut!(__slate_storage_750) as *mut *const i8;
    let mut __slate_storage_749: std::mem::MaybeUninit<*const i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_749: *mut *const i8 =
        std::ptr::addr_of_mut!(__slate_storage_749) as *mut *const i8;
    let mut __slate_storage_748: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_748: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_748) as *mut i32;
    let mut __slate_storage_747: std::mem::MaybeUninit<*const i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_747: *mut *const i8 =
        std::ptr::addr_of_mut!(__slate_storage_747) as *mut *const i8;
    let mut __slate_storage_746: std::mem::MaybeUninit<*const i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_746: *mut *const i8 =
        std::ptr::addr_of_mut!(__slate_storage_746) as *mut *const i8;
    let mut __slate_storage_436: std::mem::MaybeUninit<__SlateAlign16<[i8; 70]>> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_436: *mut [i8; 70] =
        std::ptr::addr_of_mut!(__slate_storage_436) as *mut [i8; 70];
    let mut __slate_storage_435: std::mem::MaybeUninit<*mut PrintfArguments> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_435: *mut *mut PrintfArguments =
        std::ptr::addr_of_mut!(__slate_storage_435) as *mut *mut PrintfArguments;
    let mut __slate_storage_434: std::mem::MaybeUninit<u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_434: *mut u8 = std::ptr::addr_of_mut!(__slate_storage_434) as *mut u8;
    let mut __slate_storage_433: std::mem::MaybeUninit<u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_433: *mut u8 = std::ptr::addr_of_mut!(__slate_storage_433) as *mut u8;
    let mut __slate_storage_432: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_432: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_432) as *mut i32;
    let mut __slate_storage_431: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_431: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_431) as *mut i32;
    let mut __slate_storage_430: std::mem::MaybeUninit<*mut i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_430: *mut *mut i8 =
        std::ptr::addr_of_mut!(__slate_storage_430) as *mut *mut i8;
    let mut __slate_storage_429: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_429: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_429) as *mut i32;
    let mut __slate_storage_428: std::mem::MaybeUninit<*mut i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_428: *mut *mut i8 =
        std::ptr::addr_of_mut!(__slate_storage_428) as *mut *mut i8;
    let mut __slate_storage_427: std::mem::MaybeUninit<*const et_info> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_427: *mut *const et_info =
        std::ptr::addr_of_mut!(__slate_storage_427) as *mut *const et_info;
    let mut __slate_storage_426: std::mem::MaybeUninit<f64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_426: *mut f64 = std::ptr::addr_of_mut!(__slate_storage_426) as *mut f64;
    let mut __slate_storage_425: std::mem::MaybeUninit<u64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_425: *mut u64 = std::ptr::addr_of_mut!(__slate_storage_425) as *mut u64;
    let mut __slate_storage_424: std::mem::MaybeUninit<i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_424: *mut i8 = std::ptr::addr_of_mut!(__slate_storage_424) as *mut i8;
    let mut __slate_storage_423: std::mem::MaybeUninit<u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_423: *mut u8 = std::ptr::addr_of_mut!(__slate_storage_423) as *mut u8;
    let mut __slate_storage_422: std::mem::MaybeUninit<u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_422: *mut u8 = std::ptr::addr_of_mut!(__slate_storage_422) as *mut u8;
    let mut __slate_storage_421: std::mem::MaybeUninit<u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_421: *mut u8 = std::ptr::addr_of_mut!(__slate_storage_421) as *mut u8;
    let mut __slate_storage_420: std::mem::MaybeUninit<u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_420: *mut u8 = std::ptr::addr_of_mut!(__slate_storage_420) as *mut u8;
    let mut __slate_storage_419: std::mem::MaybeUninit<u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_419: *mut u8 = std::ptr::addr_of_mut!(__slate_storage_419) as *mut u8;
    let mut __slate_storage_418: std::mem::MaybeUninit<u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_418: *mut u8 = std::ptr::addr_of_mut!(__slate_storage_418) as *mut u8;
    let mut __slate_storage_417: std::mem::MaybeUninit<u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_417: *mut u8 = std::ptr::addr_of_mut!(__slate_storage_417) as *mut u8;
    let mut __slate_storage_416: std::mem::MaybeUninit<u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_416: *mut u8 = std::ptr::addr_of_mut!(__slate_storage_416) as *mut u8;
    let mut __slate_storage_415: std::mem::MaybeUninit<u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_415: *mut u8 = std::ptr::addr_of_mut!(__slate_storage_415) as *mut u8;
    let mut __slate_storage_414: std::mem::MaybeUninit<u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_414: *mut u8 = std::ptr::addr_of_mut!(__slate_storage_414) as *mut u8;
    let mut __slate_storage_413: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_413: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_413) as *mut i64;
    let mut __slate_storage_412: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_412: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_412) as *mut i32;
    let mut __slate_storage_411: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_411: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_411) as *mut i64;
    let mut __slate_storage_410: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_410: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_410) as *mut i64;
    let mut __slate_storage_409: std::mem::MaybeUninit<*mut i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_409: *mut *mut i8 =
        std::ptr::addr_of_mut!(__slate_storage_409) as *mut *mut i8;
    let mut __slate_storage_408: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_408: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_408) as *mut i32;
    unsafe {
        '__join_1: {
            // /* Next character in the format string */
            // /* Pointer to the conversion buffer */
            // /* Precision of the current field */
            // /* Length of the field */
            // /* A general purpose loop counter */
            // /* Width of the current field */
            // /* True if "-" flag is present */
            // /* '+' or ' ' or 0 for prefix */
            // /* True if "#" flag is present */
            // /* True if "!" flag is present */
            // /* True if field width constant starts with zero */
            // /* 1 for the "l" flag, 2 for "ll", 0 by default */
            // /* Loop termination flag */
            // /* Thousands separator for %d and %u */
            // /* Conversion paradigm */
            std::ptr::write(__slate_slot_422, ((19 as i32) as i8) as u8);
            // /* True for SQLITE_PRINTF_SQLFUNC */
            // /* Prefix character.  "+" or "-" or " " or '\0'. */
            // /* Value for integer types */
            // /* Value for real types */
            // /* Pointer to the appropriate info structure */
            // /* Rendering buffer */
            // /* Size of the rendering buffer */
            // /* Malloced memory used by some conversion */
            std::ptr::write(__slate_slot_430, std::ptr::null_mut::<i8>());
            // /* exponent of real numbers */
            // /* True if decimal point should be shown */
            // /* True if trailing zeros should be removed */
            // /* Arguments for SQLITE_PRINTF_SQLFUNC */
            std::ptr::write(__slate_slot_435, std::ptr::null_mut::<PrintfArguments>());
            // /* Conversion buffer */
            // /* pAccum never starts out with an empty buffer that was obtained from
            //   ** malloc().  This precondition is required by the mprintf("%z...")
            //   ** optimization. */
            0 as i32;
            *__slate_slot_409 = std::ptr::null_mut::<i8>();
            if (((unsafe { (*pAccum).printfFlags }) as u32) as i32) & (2 as i32) != (0 as i32) {
                *__slate_slot_435 = unsafe { ap.next_arg::<*mut PrintfArguments>() };
                *__slate_slot_423 = ((1 as i32) as i8) as u8;
            } else {
                *__slate_slot_423 = ((0 as i32) as i8) as u8;
            }
        }
        '__join_0: {
            '__join_405: {
                '__loop_1: loop {
                    std::ptr::write(__slate_slot_743, (unsafe { *fmt }) as i32);
                    *__slate_slot_408 = *__slate_slot_743;
                    if *__slate_slot_743 != (0 as i32) {
                        if *__slate_slot_408 != (37 as i32) {
                            *__slate_slot_409 = fmt as *mut i8;
                            fmt = (unsafe { strchr(fmt, 37 as i32) }) as *const i8;
                            if fmt == std::ptr::null::<i8>() {
                                fmt = (unsafe {
                                    (*__slate_slot_409).offset(
                                        (unsafe { strlen(*__slate_slot_409 as *const i8) })
                                            as isize,
                                    )
                                }) as *const i8;
                            }
                            sqlite3StrAppend64(
                                pAccum,
                                *__slate_slot_409 as *const i8,
                                (unsafe { fmt.offset_from(*__slate_slot_409 as *const i8) }) as i64,
                            );
                            if ((unsafe { *fmt }) as i32) == (0 as i32) {
                                break '__join_0;
                            }
                        }
                        std::ptr::write(__slate_slot_746, fmt);
                        std::ptr::write(__slate_slot_747, unsafe {
                            (*__slate_slot_746).offset((1 as i32) as isize)
                        });
                        fmt = *__slate_slot_747;
                        std::ptr::write(__slate_slot_748, (unsafe { *(*__slate_slot_747) }) as i32);
                        *__slate_slot_408 = *__slate_slot_748;
                        if *__slate_slot_748 == (0 as i32) {
                            break '__join_405;
                        } else {
                            // /* Find out what flags are present */
                            *__slate_slot_418 = ((0 as i32) as i8) as u8;
                            *__slate_slot_417 = ((0 as i32) as i8) as u8;
                            *__slate_slot_416 = ((0 as i32) as i8) as u8;
                            *__slate_slot_421 = ((0 as i32) as i8) as u8;
                            *__slate_slot_415 = ((0 as i32) as i8) as u8;
                            *__slate_slot_414 = ((0 as i32) as i8) as u8;
                            *__slate_slot_420 = ((0 as i32) as i8) as u8;
                            *__slate_slot_413 = (0 as i32) as i64;
                            *__slate_slot_419 = ((0 as i32) as i8) as u8;
                            *__slate_slot_410 = -(1 as i32) as i64;
                            loop {
                                '__join_364: {
                                    let __t1: i32 = *__slate_slot_408;
                                    if __t1 == (45 as i32) {
                                        *__slate_slot_414 = ((1 as i32) as i8) as u8;
                                        break '__join_364;
                                    } else {
                                        if __t1 == (43 as i32) {
                                            *__slate_slot_415 = ((43 as i32) as i8) as u8;
                                            break '__join_364;
                                        } else {
                                            if __t1 == (32 as i32) {
                                                *__slate_slot_415 = ((32 as i32) as i8) as u8;
                                                break '__join_364;
                                            } else {
                                                if __t1 == (35 as i32) {
                                                    *__slate_slot_416 = ((1 as i32) as i8) as u8;
                                                    break '__join_364;
                                                } else {
                                                    if __t1 == (33 as i32) {
                                                        *__slate_slot_417 =
                                                            ((1 as i32) as i8) as u8;
                                                        break '__join_364;
                                                    } else {
                                                        if __t1 == (48 as i32) {
                                                            *__slate_slot_418 =
                                                                ((1 as i32) as i8) as u8;
                                                            break '__join_364;
                                                        } else {
                                                            if __t1 == (44 as i32) {
                                                                *__slate_slot_421 =
                                                                    ((44 as i32) as i8) as u8;
                                                                break '__join_364;
                                                            } else {
                                                                if __t1 == (108 as i32) {
                                                                    '__join_392: {
                                                                        *__slate_slot_419 =
                                                                            ((1 as i32) as i8)
                                                                                as u8;
                                                                        std::ptr::write(
                                                                            __slate_slot_749,
                                                                            fmt,
                                                                        );
                                                                        std::ptr::write(
                                                                            __slate_slot_750,
                                                                            unsafe {
                                                                                (*__slate_slot_749)
                                                                                    .offset(
                                                                                    (1 as i32)
                                                                                        as isize,
                                                                                )
                                                                            },
                                                                        );
                                                                        fmt = *__slate_slot_750;
                                                                        *__slate_slot_408 = (unsafe {
                                                                            *(*__slate_slot_750)
                                                                        })
                                                                            as i32;
                                                                        if *__slate_slot_408
                                                                            == (108 as i32)
                                                                        {
                                                                            std::ptr::write(
                                                                                __slate_slot_751,
                                                                                fmt,
                                                                            );
                                                                            std::ptr::write(
                                                                                __slate_slot_752,
                                                                                unsafe {
                                                                                    (*__slate_slot_751).offset((1 as i32) as isize)
                                                                                },
                                                                            );
                                                                            fmt = *__slate_slot_752;
                                                                            *__slate_slot_408 = (unsafe {
                                                                                *(*__slate_slot_752)
                                                                            })
                                                                                as i32;
                                                                            *__slate_slot_419 =
                                                                                ((2 as i32) as i8)
                                                                                    as u8;
                                                                        }
                                                                    }
                                                                    *__slate_slot_420 =
                                                                        ((1 as i32) as i8) as u8;
                                                                    break '__join_364;
                                                                } else {
                                                                    if __t1 == (49 as i32) {
                                                                    } else {
                                                                        if __t1 == (50 as i32) {
                                                                        } else {
                                                                            if __t1 == (51 as i32) {
                                                                            } else {
                                                                                if __t1
                                                                                    == (52 as i32)
                                                                                {
                                                                                } else {
                                                                                    if __t1
                                                                                        == (53
                                                                                            as i32)
                                                                                    {
                                                                                    } else {
                                                                                        if __t1 == (54 as i32) {
} else {
if __t1 == (55 as i32) {
} else {
if __t1 == (56 as i32) {
} else {
if __t1 == (57 as i32) {
} else {
if __t1 == (42 as i32) {
if *__slate_slot_423 != (0 as u8) {
*__slate_slot_413 = (getIntArg(*__slate_slot_435) as i32) as i64;
} else {
*__slate_slot_413 = (unsafe { ap.next_arg::<i32>() }) as i64;
}
if *__slate_slot_413 < ((0 as i32) as i64) {
*__slate_slot_414 = ((1 as i32) as i8) as u8;
*__slate_slot_413 = if *__slate_slot_413 >= (-(2147483647 as i32) as i64) { -(*__slate_slot_413) } else { (0 as i32) as i64 };
}
std::ptr::write(__slate_slot_758, (unsafe { *unsafe { fmt.offset((1 as i32) as isize) } }) as i32);
*__slate_slot_408 = *__slate_slot_758;
if *__slate_slot_758 != (46 as i32) && *__slate_slot_408 != (108 as i32) {
std::ptr::write(__slate_slot_759, fmt);
std::ptr::write(__slate_slot_760, unsafe { (*__slate_slot_759).offset((1 as i32) as isize) });
fmt = *__slate_slot_760;
*__slate_slot_408 = (unsafe { *(*__slate_slot_760) }) as i32;
*__slate_slot_420 = ((1 as i32) as i8) as u8;
break '__join_364;
} else {
break '__join_364;
}
} else {
if __t1 == (46 as i32) {
std::ptr::write(__slate_slot_761, fmt);
std::ptr::write(__slate_slot_762, unsafe { (*__slate_slot_761).offset((1 as i32) as isize) });
fmt = *__slate_slot_762;
*__slate_slot_408 = (unsafe { *(*__slate_slot_762) }) as i32;
if *__slate_slot_408 == (42 as i32) {
if *__slate_slot_423 != (0 as u8) {
*__slate_slot_410 = (getIntArg(*__slate_slot_435) as i32) as i64;
} else {
*__slate_slot_410 = (unsafe { ap.next_arg::<i32>() }) as i64;
}
if *__slate_slot_410 < ((0 as i32) as i64) {
*__slate_slot_410 = if *__slate_slot_410 >= (-(2147483647 as i32) as i64) { -(*__slate_slot_410) } else { -(1 as i32) as i64 };
}
std::ptr::write(__slate_slot_763, fmt);
std::ptr::write(__slate_slot_764, unsafe { (*__slate_slot_763).offset((1 as i32) as isize) });
fmt = *__slate_slot_764;
*__slate_slot_408 = (unsafe { *(*__slate_slot_764) }) as i32;
} else {
std::ptr::write(__slate_slot_438, (0 as i32) as u32);
loop {
if *__slate_slot_408 >= (48 as i32) && *__slate_slot_408 <= (57 as i32) {
*__slate_slot_438 = (*__slate_slot_438).wrapping_mul((10 as i32) as u32).wrapping_add(*__slate_slot_408 as u32).wrapping_sub((48 as i32) as u32);
std::ptr::write(__slate_slot_765, fmt);
std::ptr::write(__slate_slot_766, unsafe { (*__slate_slot_765).offset((1 as i32) as isize) });
fmt = *__slate_slot_766;
*__slate_slot_408 = (unsafe { *(*__slate_slot_766) }) as i32;
} else {
break;
}
}
{
}
*__slate_slot_410 = ((*__slate_slot_438 & ((2147483647 as i32) as u32)) as u64) as i64;
}
if *__slate_slot_408 == (108 as i32) {
std::ptr::write(__slate_slot_767, fmt);
std::ptr::write(__slate_slot_768, unsafe { (*__slate_slot_767).offset(-((1 as i32) as isize)) });
fmt = *__slate_slot_768;
break '__join_364;
} else {
*__slate_slot_420 = ((1 as i32) as i8) as u8;
break '__join_364;
}
} else {
*__slate_slot_420 = ((1 as i32) as i8) as u8;
break '__join_364;
}
}
}
}
}
}
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    std::ptr::write(
                                        __slate_slot_437,
                                        (*__slate_slot_408 - (48 as i32)) as u32,
                                    );
                                    loop {
                                        std::ptr::write(__slate_slot_753, fmt);
                                        std::ptr::write(__slate_slot_754, unsafe {
                                            (*__slate_slot_753).offset((1 as i32) as isize)
                                        });
                                        fmt = *__slate_slot_754;
                                        std::ptr::write(
                                            __slate_slot_755,
                                            (unsafe { *(*__slate_slot_754) }) as i32,
                                        );
                                        *__slate_slot_408 = *__slate_slot_755;
                                        if *__slate_slot_755 >= (48 as i32)
                                            && *__slate_slot_408 <= (57 as i32)
                                        {
                                            *__slate_slot_437 = (*__slate_slot_437)
                                                .wrapping_mul((10 as i32) as u32)
                                                .wrapping_add(*__slate_slot_408 as u32)
                                                .wrapping_sub((48 as i32) as u32);
                                        } else {
                                            break;
                                        }
                                    }
                                    {}
                                    *__slate_slot_413 =
                                        ((*__slate_slot_437 & ((2147483647 as i32) as u32)) as u64)
                                            as i64;
                                    if *__slate_slot_408 != (46 as i32)
                                        && *__slate_slot_408 != (108 as i32)
                                    {
                                        *__slate_slot_420 = ((1 as i32) as i8) as u8;
                                    } else {
                                        std::ptr::write(__slate_slot_756, fmt);
                                        std::ptr::write(__slate_slot_757, unsafe {
                                            (*__slate_slot_756).offset(-((1 as i32) as isize))
                                        });
                                        fmt = *__slate_slot_757;
                                    }
                                }
                                if !(*__slate_slot_420 != (0 as u8)) {
                                    std::ptr::write(__slate_slot_770, fmt);
                                    std::ptr::write(__slate_slot_771, unsafe {
                                        (*__slate_slot_770).offset((1 as i32) as isize)
                                    });
                                    fmt = *__slate_slot_771;
                                    std::ptr::write(
                                        __slate_slot_772,
                                        (unsafe { *(*__slate_slot_771) }) as i32,
                                    );
                                    *__slate_slot_408 = *__slate_slot_772;
                                    *__slate_slot_769 = *__slate_slot_772 != (0 as i32);
                                } else {
                                    *__slate_slot_769 = false as bool;
                                }
                                if !(*__slate_slot_769) {
                                    break;
                                }
                            }
                            // /* Fetch the info entry for the field */
                            // /* Fast hash-table lookup */
                            0 as i32;
                            *__slate_slot_412 =
                                ((*__slate_slot_408 as u32) % ((25 as i32) as u32)) as i32;
                            if ((unsafe {
                                (*unsafe {
                                    unsafe { std::ptr::addr_of!(fmtinfo.0) as *const et_info }
                                        .offset(*__slate_slot_412 as isize)
                                })
                                .fmttype
                            }) as i32)
                                == *__slate_slot_408
                            {
                                *__slate_slot_773 = true as bool;
                            } else {
                                std::ptr::write(
                                    __slate_slot_774,
                                    (unsafe {
                                        (*unsafe {
                                            unsafe {
                                                std::ptr::addr_of!(fmtinfo.0) as *const et_info
                                            }
                                            .offset(*__slate_slot_412 as isize)
                                        })
                                        .iNxt
                                    }) as i32,
                                );
                                *__slate_slot_412 = *__slate_slot_774;
                                *__slate_slot_773 = ((unsafe {
                                    (*unsafe {
                                        unsafe { std::ptr::addr_of!(fmtinfo.0) as *const et_info }
                                            .offset(*__slate_slot_774 as isize)
                                    })
                                    .fmttype
                                }) as i32)
                                    == *__slate_slot_408;
                            }
                            if *__slate_slot_773 {
                                *__slate_slot_427 = unsafe {
                                    unsafe { std::ptr::addr_of!(fmtinfo.0) as *const et_info }
                                        .offset(*__slate_slot_412 as isize)
                                };
                                *__slate_slot_422 = unsafe { (*(*__slate_slot_427)).r#type };
                            } else {
                                *__slate_slot_427 = unsafe {
                                    unsafe { std::ptr::addr_of!(fmtinfo.0) as *const et_info }
                                        .offset((0 as i32) as isize)
                                };
                                *__slate_slot_422 = ((19 as i32) as i8) as u8;
                            }
                            '__join_2: {
                                '__join_11: {
                                    '__join_351: {
                                        '__join_352: {
                                            '__join_300: {
                                                '__join_153: {
                                                    '__join_175: {
                                                        '__join_148: {
                                                            // /*
                                                            //     ** At this point, variables are initialized as follows:
                                                            //     **
                                                            //     **   flag_alternateform          TRUE if a '#' is present.
                                                            //     **   flag_altform2               TRUE if a '!' is present.
                                                            //     **   flag_prefix                 '+' or ' ' or zero
                                                            //     **   flag_leftjustify            TRUE if a '-' is present or if the
                                                            //     **                               field width was negative.
                                                            //     **   flag_zeropad                TRUE if the width began with 0.
                                                            //     **   flag_long                   1 for "l", 2 for "ll"
                                                            //     **   width                       The specified field width.  This is
                                                            //     **                               always non-negative.  Zero is the default.
                                                            //     **   precision                   The specified precision.  The default
                                                            //     **                               is -1.
                                                            //     **   xtype                       The class of the conversion.
                                                            //     **   infop                       Pointer to the appropriate info struct.
                                                            //     */
                                                            0 as i32;
                                                            0 as i32;
                                                            let __t0: i32 =
                                                                (*__slate_slot_422 as u32) as i32;
                                                            if __t0 == (13 as i32) {
                                                                *__slate_slot_419 = ((if (8 as u64)
                                                                    == (8 as u64)
                                                                {
                                                                    2 as i32
                                                                } else {
                                                                    if (8 as u64) == (8 as u64) {
                                                                        1 as i32
                                                                    } else {
                                                                        0 as i32
                                                                    }
                                                                })
                                                                    as i8)
                                                                    as u8;
                                                                // /* no break */
                                                                {}
                                                                break '__join_352;
                                                            } else {
                                                                if __t0 == (15 as i32) {
                                                                    break '__join_352;
                                                                } else {
                                                                    if __t0 == (0 as i32) {
                                                                        break '__join_352;
                                                                    } else {
                                                                        if __t0 == (16 as i32) {
                                                                            break '__join_351;
                                                                        } else {
                                                                            if __t0 == (1 as i32) {
                                                                                break '__join_300;
                                                                            } else {
                                                                                if __t0
                                                                                    == (2 as i32)
                                                                                {
                                                                                    break '__join_300;
                                                                                } else {
                                                                                    if __t0
                                                                                        == (3
                                                                                            as i32)
                                                                                    {
                                                                                        break '__join_300;
                                                                                    } else {
                                                                                        if __t0 == (4 as i32) {
if !(*__slate_slot_423 != (0 as u8)) {
unsafe {
*unsafe { ap.next_arg::<*mut i32>() } = (unsafe { (*pAccum).nChar }) as i32;
}
}
*__slate_slot_413 = (0 as i32) as i64;
*__slate_slot_411 = (0 as i32) as i64;
break '__join_11;
} else {
if __t0 == (7 as i32) {
unsafe {
*unsafe { ((*__slate_slot_436).as_mut_ptr() as *mut i8).offset((0 as i32) as isize) } = (37 as i32) as i8;
}
*__slate_slot_409 = (*__slate_slot_436).as_mut_ptr() as *mut i8;
*__slate_slot_411 = (1 as i32) as i64;
break '__join_11;
} else {
if __t0 == (8 as i32) {
'__join_185: {
if *__slate_slot_423 != (0 as u8) {
*__slate_slot_409 = getTextArg(*__slate_slot_435);
*__slate_slot_411 = (1 as i32) as i64;
if *__slate_slot_409 != std::ptr::null_mut::<i8>() {
std::ptr::write(__slate_slot_865, *__slate_slot_409);
std::ptr::write(__slate_slot_866, unsafe { (*__slate_slot_865).offset((1 as i32) as isize) });
*__slate_slot_409 = *__slate_slot_866;
std::ptr::write(__slate_slot_867, (unsafe { *(*__slate_slot_865) }) as i32);
*__slate_slot_408 = *__slate_slot_867;
unsafe {
*unsafe { ((*__slate_slot_436).as_mut_ptr() as *mut i8).offset((0 as i32) as isize) } = *__slate_slot_867 as i8;
}
if *__slate_slot_408 & (192 as i32) == (192 as i32) {
loop {
if *__slate_slot_411 < ((4 as i32) as i64) && ((unsafe { *unsafe { (*__slate_slot_409).offset((0 as i32) as isize) } }) as i32) & (192 as i32) == (128 as i32) {
std::ptr::write(__slate_slot_868, *__slate_slot_409);
std::ptr::write(__slate_slot_869, unsafe { (*__slate_slot_868).offset((1 as i32) as isize) });
*__slate_slot_409 = *__slate_slot_869;
std::ptr::write(__slate_slot_870, *__slate_slot_411);
std::ptr::write(__slate_slot_871, *__slate_slot_870 + ((1 as i32) as i64));
*__slate_slot_411 = *__slate_slot_871;
unsafe {
*unsafe { ((*__slate_slot_436).as_mut_ptr() as *mut i8).offset(*__slate_slot_870 as isize) } = unsafe { *(*__slate_slot_868) };
}
} else {
break '__join_185;
}
}
}
} else {
unsafe {
*unsafe { ((*__slate_slot_436).as_mut_ptr() as *mut i8).offset((0 as i32) as isize) } = (0 as i32) as i8;
}
}
} else {
std::ptr::write(__slate_slot_459, unsafe { ap.next_arg::<u32>() });
*__slate_slot_411 = (unsafe { sqlite3AppendOneUtf8Character((*__slate_slot_436).as_mut_ptr() as *mut i8, *__slate_slot_459) }) as i64;
}
}
'__join_176: {
if *__slate_slot_410 > ((1 as i32) as i64) {
std::ptr::write(__slate_slot_460, (1 as i32) as i64);
std::ptr::write(__slate_slot_872, *__slate_slot_413);
std::ptr::write(__slate_slot_873, *__slate_slot_872 - (*__slate_slot_410 - ((1 as i32) as i64)));
*__slate_slot_413 = *__slate_slot_873;
if *__slate_slot_413 > ((1 as i32) as i64) && !(*__slate_slot_414 != (0 as u8)) {
sqlite3StrAppendchar64(pAccum, *__slate_slot_413 - ((1 as i32) as i64), (32 as i32) as i8);
*__slate_slot_413 = (0 as i32) as i64;
}
sqlite3StrAppend64(pAccum, ((*__slate_slot_436).as_mut_ptr() as *mut i8) as *const i8, *__slate_slot_411);
std::ptr::write(__slate_slot_874, *__slate_slot_410);
std::ptr::write(__slate_slot_875, *__slate_slot_874 - ((1 as i32) as i64));
*__slate_slot_410 = *__slate_slot_875;
loop {
if *__slate_slot_410 > ((1 as i32) as i64) {
if *__slate_slot_460 > *__slate_slot_410 - ((1 as i32) as i64) {
*__slate_slot_460 = *__slate_slot_410 - ((1 as i32) as i64);
}
*__slate_slot_461 = *__slate_slot_411 * *__slate_slot_460;
if sqlite3StrAccumEnlargeIfNeeded(pAccum, *__slate_slot_461) != (0 as i32) {
break '__join_176;
} else {
sqlite3_str_append(pAccum, (unsafe { unsafe { (*pAccum).zText }.offset(((((unsafe { (*pAccum).nChar }) as u64) as i64) - *__slate_slot_461) as isize) }) as *const i8, *__slate_slot_461 as i32);
std::ptr::write(__slate_slot_876, *__slate_slot_410);
std::ptr::write(__slate_slot_877, *__slate_slot_876 - *__slate_slot_460);
*__slate_slot_410 = *__slate_slot_877;
std::ptr::write(__slate_slot_878, *__slate_slot_460);
std::ptr::write(__slate_slot_879, *__slate_slot_878 * ((2 as i32) as i64));
*__slate_slot_460 = *__slate_slot_879;
}
} else {
break '__join_176;
}
}
}
}
*__slate_slot_409 = (*__slate_slot_436).as_mut_ptr() as *mut i8;
*__slate_slot_417 = ((1 as i32) as i8) as u8;
break '__join_153;
} else {
if __t0 == (5 as i32) {
break '__join_175;
} else {
if __t0 == (6 as i32) {
break '__join_175;
} else {
if __t0 == (17 as i32) {
break '__join_148;
} else {
if __t0 == (18 as i32) {
break '__join_148;
} else {
if __t0 == (9 as i32) {
} else {
if __t0 == (10 as i32) {
} else {
if __t0 == (14 as i32) {
} else {
if __t0 == (11 as i32) {
if (((unsafe { (*pAccum).printfFlags }) as u32) as i32) & (1 as i32) == (0 as i32) {
return;
} else {
'__join_30: {
if *__slate_slot_416 != (0 as u8) {
// /* %#T means an Expr pointer that uses Expr.u.zToken */
std::ptr::write(__slate_slot_484, unsafe { ap.next_arg::<*mut Expr>() });
if *__slate_slot_484 != std::ptr::null_mut::<Expr>() && !((unsafe { (*(*__slate_slot_484)).flags }) & ((2048 as i32) as u32) != ((0 as i32) as u32)) {
sqlite3_str_appendall(pAccum, (unsafe { (*(*__slate_slot_484)).u.zToken }) as *const i8);
sqlite3RecordErrorOffsetOfExpr(unsafe { (*pAccum).db }, *__slate_slot_484 as *const Expr);
}
} else {
// /* %T means a Token pointer */
std::ptr::write(__slate_slot_485, unsafe { ap.next_arg::<*mut Token>() });
0 as i32;
if *__slate_slot_485 != std::ptr::null_mut::<Token>() && (unsafe { (*(*__slate_slot_485)).n }) != (0 as u32) {
sqlite3_str_append(pAccum, unsafe { (*(*__slate_slot_485)).z }, (unsafe { (*(*__slate_slot_485)).n }) as i32);
sqlite3RecordErrorByteOffset(unsafe { (*pAccum).db }, unsafe { (*(*__slate_slot_485)).z });
}
}
}
*__slate_slot_413 = (0 as i32) as i64;
*__slate_slot_411 = (0 as i32) as i64;
break '__join_11;
}
} else {
if __t0 == (12 as i32) {
if (((unsafe { (*pAccum).printfFlags }) as u32) as i32) & (1 as i32) == (0 as i32) {
return;
} else {
*__slate_slot_486 = unsafe { ap.next_arg::<*mut SrcItem>() };
0 as i32;
if (unsafe { (*(*__slate_slot_486)).zAlias }) != std::ptr::null_mut::<i8>() && !(*__slate_slot_417 != (0 as u8)) {
sqlite3_str_appendall(pAccum, (unsafe { (*(*__slate_slot_486)).zAlias }) as *const i8);
} else {
if (unsafe { (*(*__slate_slot_486)).zName }) != std::ptr::null_mut::<i8>() {
if ((unsafe { (*(*__slate_slot_486)).fg.__slate_bits_0.__get_fixedSchema() }) as i32) == (0 as i32) && ((unsafe { (*(*__slate_slot_486)).fg.__slate_bits_0.__get_isSubquery() }) as i32) == (0 as i32) && (unsafe { (*(*__slate_slot_486)).u4.zDatabase }) != std::ptr::null_mut::<i8>() {
sqlite3_str_appendall(pAccum, (unsafe { (*(*__slate_slot_486)).u4.zDatabase }) as *const i8);
sqlite3_str_append(pAccum, (b".\0".as_ptr() as *mut i8) as *const i8, 1 as i32);
}
sqlite3_str_appendall(pAccum, (unsafe { (*(*__slate_slot_486)).zName }) as *const i8);
} else {
if (unsafe { (*(*__slate_slot_486)).zAlias }) != std::ptr::null_mut::<i8>() {
sqlite3_str_appendall(pAccum, (unsafe { (*(*__slate_slot_486)).zAlias }) as *const i8);
// /* Because of tag-20240424-1 */
} else {
if ((unsafe { (*(*__slate_slot_486)).fg.__slate_bits_0.__get_isSubquery() }) as i32) != (0 as i32) {
std::ptr::write(__slate_slot_487, unsafe { (*unsafe { (*(*__slate_slot_486)).u4.pSubq }).pSelect });
0 as i32;
if (unsafe { (*(*__slate_slot_487)).selFlags }) & ((2048 as i32) as u32) != (0 as u32) {
unsafe { sqlite3_str_appendf(pAccum, (b"(join-%u)\0".as_ptr() as *mut i8) as *const i8, unsafe { (*(*__slate_slot_487)).selId }) };
} else {
if (unsafe { (*(*__slate_slot_487)).selFlags }) & ((1024 as i32) as u32) != (0 as u32) {
0 as i32;
unsafe { sqlite3_str_appendf(pAccum, (b"%u-ROW VALUES CLAUSE\0".as_ptr() as *mut i8) as *const i8, unsafe { (*(*__slate_slot_486)).u1.nRow }) };
} else {
unsafe { sqlite3_str_appendf(pAccum, (b"(subquery-%u)\0".as_ptr() as *mut i8) as *const i8, unsafe { (*(*__slate_slot_487)).selId }) };
}
}
}
}
}
}
*__slate_slot_413 = (0 as i32) as i64;
*__slate_slot_411 = (0 as i32) as i64;
break '__join_11;
}
} else {
break '__loop_1;
}
}
}
}
}
}
}
}
}
}
}
}
                                                                                    }
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                            std::ptr::write(
                                                                __slate_slot_478,
                                                                0 as i32,
                                                            );
                                                            if *__slate_slot_423 != (0 as u8) {
                                                                *__slate_slot_480 =
                                                                    getTextArg(*__slate_slot_435);
                                                            } else {
                                                                *__slate_slot_480 = unsafe {
                                                                    ap.next_arg::<*mut i8>()
                                                                };
                                                            }
                                                            if *__slate_slot_480
                                                                == std::ptr::null_mut::<i8>()
                                                            {
                                                                *__slate_slot_480 =
                                                                    if ((*__slate_slot_422 as u32)
                                                                        as i32)
                                                                        == (10 as i32)
                                                                    {
                                                                        b"NULL\0".as_ptr()
                                                                            as *mut i8
                                                                    } else {
                                                                        b"(NULL)\0".as_ptr()
                                                                            as *mut i8
                                                                    };
                                                            } else {
                                                                if ((*__slate_slot_422 as u32)
                                                                    as i32)
                                                                    == (10 as i32)
                                                                {
                                                                    *__slate_slot_478 = 1 as i32;
                                                                }
                                                            }
                                                            if ((*__slate_slot_422 as u32) as i32)
                                                                == (14 as i32)
                                                            {
                                                                *__slate_slot_481 =
                                                                    (34 as i32) as i8;
                                                                *__slate_slot_416 =
                                                                    ((0 as i32) as i8) as u8;
                                                            } else {
                                                                *__slate_slot_481 =
                                                                    (39 as i32) as i8;
                                                            }
                                                            // /* For %q, %Q, and %w, the precision is the number of bytes (or
                                                            //         ** characters if the ! flags is present) to use from the input.
                                                            //         ** Because of the extra quoting characters inserted, the number
                                                            //         ** of output characters may be larger than the precision.
                                                            //         */
                                                            *__slate_slot_476 = *__slate_slot_410;
                                                            *__slate_slot_477 = (0 as i32) as i64;
                                                            *__slate_slot_474 = (0 as i32) as i64;
                                                            loop {
                                                                if *__slate_slot_476
                                                                    != ((0 as i32) as i64)
                                                                {
                                                                    std::ptr::write(
                                                                        __slate_slot_913,
                                                                        unsafe {
                                                                            *unsafe {
                                                                                (*__slate_slot_480).offset(*__slate_slot_474 as isize)
                                                                            }
                                                                        },
                                                                    );
                                                                    *__slate_slot_479 =
                                                                        *__slate_slot_913;
                                                                    *__slate_slot_912 =
                                                                        (*__slate_slot_913 as i32)
                                                                            != (0 as i32);
                                                                } else {
                                                                    *__slate_slot_912 =
                                                                        false as bool;
                                                                }
                                                                if *__slate_slot_912 {
                                                                    if (*__slate_slot_479 as i32)
                                                                        == (*__slate_slot_481
                                                                            as i32)
                                                                    {
                                                                        std::ptr::write(
                                                                            __slate_slot_918,
                                                                            *__slate_slot_477,
                                                                        );
                                                                        std::ptr::write(
                                                                            __slate_slot_919,
                                                                            *__slate_slot_918
                                                                                + ((1 as i32)
                                                                                    as i64),
                                                                        );
                                                                        *__slate_slot_477 =
                                                                            *__slate_slot_919;
                                                                    }
                                                                    '__join_81: {
                                                                        if *__slate_slot_417
                                                                            != (0 as u8)
                                                                            && (*__slate_slot_479
                                                                                as i32)
                                                                                & (192 as i32)
                                                                                == (192 as i32)
                                                                        {
                                                                            loop {
                                                                                if ((unsafe {
                                                                                    *unsafe {
                                                                                        (*__slate_slot_480).offset((*__slate_slot_474 + ((1 as i32) as i64)) as isize)
                                                                                    }
                                                                                })
                                                                                    as i32)
                                                                                    & (192 as i32)
                                                                                    == (128 as i32)
                                                                                {
                                                                                    std::ptr::write(__slate_slot_920, *__slate_slot_474);
                                                                                    std::ptr::write(__slate_slot_921, *__slate_slot_920 + ((1 as i32) as i64));
                                                                                    *__slate_slot_474 = *__slate_slot_921;
                                                                                } else {
                                                                                    break '__join_81;
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                    std::ptr::write(
                                                                        __slate_slot_914,
                                                                        *__slate_slot_474,
                                                                    );
                                                                    std::ptr::write(
                                                                        __slate_slot_915,
                                                                        *__slate_slot_914
                                                                            + ((1 as i32) as i64),
                                                                    );
                                                                    *__slate_slot_474 =
                                                                        *__slate_slot_915;
                                                                    std::ptr::write(
                                                                        __slate_slot_916,
                                                                        *__slate_slot_476,
                                                                    );
                                                                    std::ptr::write(
                                                                        __slate_slot_917,
                                                                        *__slate_slot_916
                                                                            - ((1 as i32) as i64),
                                                                    );
                                                                    *__slate_slot_476 =
                                                                        *__slate_slot_917;
                                                                } else {
                                                                    break;
                                                                }
                                                            }
                                                            if *__slate_slot_416 != (0 as u8) {
                                                                // /* For %#q, do unistr()-style backslash escapes for
                                                                //           ** all control characters, and for backslash itself.
                                                                //           ** For %#Q, do the same but only if there is at least
                                                                //           ** one control character. */
                                                                std::ptr::write(
                                                                    __slate_slot_482,
                                                                    (0 as i32) as i64,
                                                                );
                                                                std::ptr::write(
                                                                    __slate_slot_483,
                                                                    (0 as i32) as i64,
                                                                );
                                                                *__slate_slot_476 =
                                                                    (0 as i32) as i64;
                                                                loop {
                                                                    if *__slate_slot_476
                                                                        < *__slate_slot_474
                                                                    {
                                                                        if ((unsafe {
                                                                            *unsafe {
                                                                                (*__slate_slot_480).offset(*__slate_slot_476 as isize)
                                                                            }
                                                                        })
                                                                            as i32)
                                                                            == (92 as i32)
                                                                        {
                                                                            std::ptr::write(
                                                                                __slate_slot_924,
                                                                                *__slate_slot_482,
                                                                            );
                                                                            std::ptr::write(
                                                                                __slate_slot_925,
                                                                                *__slate_slot_924
                                                                                    + ((1 as i32)
                                                                                        as i64),
                                                                            );
                                                                            *__slate_slot_482 =
                                                                                *__slate_slot_925;
                                                                        } else {
                                                                            if (((unsafe {
                                                                                *unsafe {
                                                                                    (*__slate_slot_480 as *mut u8).offset(*__slate_slot_476 as isize)
                                                                                }
                                                                            })
                                                                                as u32)
                                                                                as i32)
                                                                                <= (31 as i32)
                                                                            {
                                                                                std::ptr::write(__slate_slot_926, *__slate_slot_483);
                                                                                std::ptr::write(__slate_slot_927, *__slate_slot_926 + ((1 as i32) as i64));
                                                                                *__slate_slot_483 = *__slate_slot_927;
                                                                            }
                                                                        }
                                                                        std::ptr::write(
                                                                            __slate_slot_922,
                                                                            *__slate_slot_476,
                                                                        );
                                                                        std::ptr::write(
                                                                            __slate_slot_923,
                                                                            *__slate_slot_922
                                                                                + ((1 as i32)
                                                                                    as i64),
                                                                        );
                                                                        *__slate_slot_476 =
                                                                            *__slate_slot_923;
                                                                    } else {
                                                                        break;
                                                                    }
                                                                }
                                                                if *__slate_slot_483 != (0 as i64)
                                                                    || ((*__slate_slot_422 as u32)
                                                                        as i32)
                                                                        == (9 as i32)
                                                                {
                                                                    std::ptr::write(
                                                                        __slate_slot_928,
                                                                        *__slate_slot_477,
                                                                    );
                                                                    std::ptr::write(__slate_slot_929, *__slate_slot_928 + (*__slate_slot_482 + ((5 as i32) as i64) * *__slate_slot_483));
                                                                    *__slate_slot_477 =
                                                                        *__slate_slot_929;
                                                                    if ((*__slate_slot_422 as u32)
                                                                        as i32)
                                                                        == (10 as i32)
                                                                    {
                                                                        std::ptr::write(
                                                                            __slate_slot_930,
                                                                            *__slate_slot_477,
                                                                        );
                                                                        std::ptr::write(
                                                                            __slate_slot_931,
                                                                            *__slate_slot_930
                                                                                + ((10 as i32)
                                                                                    as i64),
                                                                        );
                                                                        *__slate_slot_477 =
                                                                            *__slate_slot_931;
                                                                        *__slate_slot_478 =
                                                                            2 as i32;
                                                                    }
                                                                } else {
                                                                    *__slate_slot_416 =
                                                                        ((0 as i32) as i8) as u8;
                                                                }
                                                            }
                                                            std::ptr::write(
                                                                __slate_slot_932,
                                                                *__slate_slot_477,
                                                            );
                                                            std::ptr::write(
                                                                __slate_slot_933,
                                                                *__slate_slot_932
                                                                    + (*__slate_slot_474
                                                                        + ((3 as i32) as i64)),
                                                            );
                                                            *__slate_slot_477 = *__slate_slot_933;
                                                            if *__slate_slot_477
                                                                > ((70 as i32) as i64)
                                                            {
                                                                std::ptr::write(
                                                                    __slate_slot_934,
                                                                    printfTempBuf(
                                                                        pAccum,
                                                                        *__slate_slot_477,
                                                                    ),
                                                                );
                                                                *__slate_slot_430 =
                                                                    *__slate_slot_934;
                                                                *__slate_slot_409 =
                                                                    *__slate_slot_934;
                                                                if *__slate_slot_409
                                                                    == std::ptr::null_mut::<i8>()
                                                                {
                                                                    return;
                                                                }
                                                            } else {
                                                                *__slate_slot_409 =
                                                                    (*__slate_slot_436).as_mut_ptr()
                                                                        as *mut i8;
                                                            }
                                                            *__slate_slot_475 = (0 as i32) as i64;
                                                            if *__slate_slot_478 != (0 as i32) {
                                                                if *__slate_slot_478 == (2 as i32) {
                                                                    unsafe {
                                                                        memcpy(
                                                                            (unsafe {
                                                                                (*__slate_slot_409).offset(*__slate_slot_475 as isize)
                                                                            })
                                                                                as *mut (),
                                                                            (b"unistr('\0".as_ptr()
                                                                                as *mut i8)
                                                                                as *const (),
                                                                            ((8 as i32) as i64)
                                                                                as u64,
                                                                        )
                                                                    };
                                                                    std::ptr::write(
                                                                        __slate_slot_935,
                                                                        *__slate_slot_475,
                                                                    );
                                                                    std::ptr::write(
                                                                        __slate_slot_936,
                                                                        *__slate_slot_935
                                                                            + ((8 as i32) as i64),
                                                                    );
                                                                    *__slate_slot_475 =
                                                                        *__slate_slot_936;
                                                                } else {
                                                                    std::ptr::write(
                                                                        __slate_slot_937,
                                                                        *__slate_slot_475,
                                                                    );
                                                                    std::ptr::write(
                                                                        __slate_slot_938,
                                                                        *__slate_slot_937
                                                                            + ((1 as i32) as i64),
                                                                    );
                                                                    *__slate_slot_475 =
                                                                        *__slate_slot_938;
                                                                    unsafe {
                                                                        *unsafe {
                                                                            (*__slate_slot_409)
                                                                                .offset(
                                                                                *__slate_slot_937
                                                                                    as isize,
                                                                            )
                                                                        } = (39 as i32) as i8;
                                                                    }
                                                                }
                                                            }
                                                            '__join_41: {
                                                                *__slate_slot_476 =
                                                                    *__slate_slot_474;
                                                                if *__slate_slot_416 != (0 as u8) {
                                                                    *__slate_slot_474 =
                                                                        (0 as i32) as i64;
                                                                    loop {
                                                                        if *__slate_slot_474
                                                                            < *__slate_slot_476
                                                                        {
                                                                            std::ptr::write(
                                                                                __slate_slot_941,
                                                                                unsafe {
                                                                                    *unsafe {
                                                                                        (*__slate_slot_480).offset(*__slate_slot_474 as isize)
                                                                                    }
                                                                                },
                                                                            );
                                                                            *__slate_slot_479 =
                                                                                *__slate_slot_941;
                                                                            std::ptr::write(
                                                                                __slate_slot_942,
                                                                                *__slate_slot_475,
                                                                            );
                                                                            std::ptr::write(
                                                                                __slate_slot_943,
                                                                                *__slate_slot_942
                                                                                    + ((1 as i32)
                                                                                        as i64),
                                                                            );
                                                                            *__slate_slot_475 =
                                                                                *__slate_slot_943;
                                                                            unsafe {
                                                                                *unsafe { (*__slate_slot_409).offset(*__slate_slot_942 as isize) } = *__slate_slot_941;
                                                                            }
                                                                            if (*__slate_slot_479 as i32) == (*__slate_slot_481 as i32) {
std::ptr::write(__slate_slot_944, *__slate_slot_475);
std::ptr::write(__slate_slot_945, *__slate_slot_944 + ((1 as i32) as i64));
*__slate_slot_475 = *__slate_slot_945;
unsafe {
*unsafe { (*__slate_slot_409).offset(*__slate_slot_944 as isize) } = *__slate_slot_479;
}
} else {
if (*__slate_slot_479 as i32) == (92 as i32) {
std::ptr::write(__slate_slot_946, *__slate_slot_475);
std::ptr::write(__slate_slot_947, *__slate_slot_946 + ((1 as i32) as i64));
*__slate_slot_475 = *__slate_slot_947;
unsafe {
*unsafe { (*__slate_slot_409).offset(*__slate_slot_946 as isize) } = (92 as i32) as i8;
}
} else {
if (((*__slate_slot_479 as u8) as u32) as i32) <= (31 as i32) {
unsafe {
*unsafe { (*__slate_slot_409).offset((*__slate_slot_475 - ((1 as i32) as i64)) as isize) } = (92 as i32) as i8;
}
std::ptr::write(__slate_slot_948, *__slate_slot_475);
std::ptr::write(__slate_slot_949, *__slate_slot_948 + ((1 as i32) as i64));
*__slate_slot_475 = *__slate_slot_949;
unsafe {
*unsafe { (*__slate_slot_409).offset(*__slate_slot_948 as isize) } = (117 as i32) as i8;
}
std::ptr::write(__slate_slot_950, *__slate_slot_475);
std::ptr::write(__slate_slot_951, *__slate_slot_950 + ((1 as i32) as i64));
*__slate_slot_475 = *__slate_slot_951;
unsafe {
*unsafe { (*__slate_slot_409).offset(*__slate_slot_950 as isize) } = (48 as i32) as i8;
}
std::ptr::write(__slate_slot_952, *__slate_slot_475);
std::ptr::write(__slate_slot_953, *__slate_slot_952 + ((1 as i32) as i64));
*__slate_slot_475 = *__slate_slot_953;
unsafe {
*unsafe { (*__slate_slot_409).offset(*__slate_slot_952 as isize) } = (48 as i32) as i8;
}
std::ptr::write(__slate_slot_954, *__slate_slot_475);
std::ptr::write(__slate_slot_955, *__slate_slot_954 + ((1 as i32) as i64));
*__slate_slot_475 = *__slate_slot_955;
unsafe {
*unsafe { (*__slate_slot_409).offset(*__slate_slot_954 as isize) } = (if (*__slate_slot_479 as i32) >= (16 as i32) { 49 as i32 } else { 48 as i32 }) as i8;
}
std::ptr::write(__slate_slot_956, *__slate_slot_475);
std::ptr::write(__slate_slot_957, *__slate_slot_956 + ((1 as i32) as i64));
*__slate_slot_475 = *__slate_slot_957;
unsafe {
*unsafe { (*__slate_slot_409).offset(*__slate_slot_956 as isize) } = unsafe { *unsafe { unsafe { std::ptr::addr_of!(aHex.0) as *const i8 }.offset(((*__slate_slot_479 as i32) & (15 as i32)) as isize) } };
}
}
}
}
                                                                            std::ptr::write(
                                                                                __slate_slot_939,
                                                                                *__slate_slot_474,
                                                                            );
                                                                            std::ptr::write(
                                                                                __slate_slot_940,
                                                                                *__slate_slot_939
                                                                                    + ((1 as i32)
                                                                                        as i64),
                                                                            );
                                                                            *__slate_slot_474 =
                                                                                *__slate_slot_940;
                                                                        } else {
                                                                            break '__join_41;
                                                                        }
                                                                    }
                                                                } else {
                                                                    *__slate_slot_474 =
                                                                        (0 as i32) as i64;
                                                                    loop {
                                                                        if *__slate_slot_474
                                                                            < *__slate_slot_476
                                                                        {
                                                                            std::ptr::write(
                                                                                __slate_slot_960,
                                                                                unsafe {
                                                                                    *unsafe {
                                                                                        (*__slate_slot_480).offset(*__slate_slot_474 as isize)
                                                                                    }
                                                                                },
                                                                            );
                                                                            *__slate_slot_479 =
                                                                                *__slate_slot_960;
                                                                            std::ptr::write(
                                                                                __slate_slot_961,
                                                                                *__slate_slot_475,
                                                                            );
                                                                            std::ptr::write(
                                                                                __slate_slot_962,
                                                                                *__slate_slot_961
                                                                                    + ((1 as i32)
                                                                                        as i64),
                                                                            );
                                                                            *__slate_slot_475 =
                                                                                *__slate_slot_962;
                                                                            unsafe {
                                                                                *unsafe { (*__slate_slot_409).offset(*__slate_slot_961 as isize) } = *__slate_slot_960;
                                                                            }
                                                                            if (*__slate_slot_479 as i32) == (*__slate_slot_481 as i32) {
std::ptr::write(__slate_slot_963, *__slate_slot_475);
std::ptr::write(__slate_slot_964, *__slate_slot_963 + ((1 as i32) as i64));
*__slate_slot_475 = *__slate_slot_964;
unsafe {
*unsafe { (*__slate_slot_409).offset(*__slate_slot_963 as isize) } = *__slate_slot_479;
}
}
                                                                            std::ptr::write(
                                                                                __slate_slot_958,
                                                                                *__slate_slot_474,
                                                                            );
                                                                            std::ptr::write(
                                                                                __slate_slot_959,
                                                                                *__slate_slot_958
                                                                                    + ((1 as i32)
                                                                                        as i64),
                                                                            );
                                                                            *__slate_slot_474 =
                                                                                *__slate_slot_959;
                                                                        } else {
                                                                            break '__join_41;
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                            if *__slate_slot_478 != (0 as i32) {
                                                                std::ptr::write(
                                                                    __slate_slot_965,
                                                                    *__slate_slot_475,
                                                                );
                                                                std::ptr::write(
                                                                    __slate_slot_966,
                                                                    *__slate_slot_965
                                                                        + ((1 as i32) as i64),
                                                                );
                                                                *__slate_slot_475 =
                                                                    *__slate_slot_966;
                                                                unsafe {
                                                                    *unsafe {
                                                                        (*__slate_slot_409).offset(
                                                                            *__slate_slot_965
                                                                                as isize,
                                                                        )
                                                                    } = (39 as i32) as i8;
                                                                }
                                                                if *__slate_slot_478 == (2 as i32) {
                                                                    std::ptr::write(
                                                                        __slate_slot_967,
                                                                        *__slate_slot_475,
                                                                    );
                                                                    std::ptr::write(
                                                                        __slate_slot_968,
                                                                        *__slate_slot_967
                                                                            + ((1 as i32) as i64),
                                                                    );
                                                                    *__slate_slot_475 =
                                                                        *__slate_slot_968;
                                                                    unsafe {
                                                                        *unsafe {
                                                                            (*__slate_slot_409)
                                                                                .offset(
                                                                                *__slate_slot_967
                                                                                    as isize,
                                                                            )
                                                                        } = (41 as i32) as i8;
                                                                    }
                                                                }
                                                            }
                                                            unsafe {
                                                                *unsafe {
                                                                    (*__slate_slot_409).offset(
                                                                        *__slate_slot_475 as isize,
                                                                    )
                                                                } = (0 as i32) as i8;
                                                            }
                                                            *__slate_slot_411 = *__slate_slot_475;
                                                            break '__join_153;
                                                        }
                                                        if *__slate_slot_423 != (0 as u8) {
                                                            *__slate_slot_464 =
                                                                getTextArg(*__slate_slot_435);
                                                        } else {
                                                            *__slate_slot_464 =
                                                                unsafe { ap.next_arg::<*mut i8>() };
                                                        }
                                                        *__slate_slot_468 =
                                                            sqlite3_str_length(pAccum) as i64;
                                                        if *__slate_slot_464
                                                            == std::ptr::null_mut::<i8>()
                                                        {
                                                            if ((*__slate_slot_422 as u32) as i32)
                                                                == (18 as i32)
                                                            {
                                                                sqlite3_str_append(
                                                                    pAccum,
                                                                    (b"null\0".as_ptr() as *mut i8)
                                                                        as *const i8,
                                                                    4 as i32,
                                                                );
                                                            }
                                                        } else {
                                                            if ((*__slate_slot_422 as u32) as i32)
                                                                == (18 as i32)
                                                            {
                                                                sqlite3_str_append(
                                                                    pAccum,
                                                                    (b"\"\0".as_ptr() as *mut i8)
                                                                        as *const i8,
                                                                    1 as i32,
                                                                );
                                                            }
                                                            '__join_131: {
                                                                *__slate_slot_467 =
                                                                    *__slate_slot_410;
                                                                {}
                                                                if *__slate_slot_467
                                                                    < ((0 as i32) as i64)
                                                                {
                                                                    *__slate_slot_467 =
                                                                        (2147483647 as i32) as i64;
                                                                } else {
                                                                    if *__slate_slot_417
                                                                        != (0 as u8)
                                                                    {
                                                                        // /* Convert precision from code-points to bytes */
                                                                        *__slate_slot_465 =
                                                                            (0 as i32) as i64;
                                                                        loop {
                                                                            if *__slate_slot_465
                                                                                < *__slate_slot_467
                                                                                && (unsafe {
                                                                                    *unsafe {
                                                                                        (*__slate_slot_464).offset(*__slate_slot_465 as isize)
                                                                                    }
                                                                                }) != (0 as i8)
                                                                            {
                                                                                if ((unsafe {
                                                                                    *unsafe {
                                                                                        (*__slate_slot_464).offset(*__slate_slot_465 as isize)
                                                                                    }
                                                                                })
                                                                                    as i32)
                                                                                    & (192 as i32)
                                                                                    == (128 as i32)
                                                                                {
                                                                                    std::ptr::write(__slate_slot_897, *__slate_slot_467);
                                                                                    std::ptr::write(__slate_slot_898, *__slate_slot_897 + ((1 as i32) as i64));
                                                                                    *__slate_slot_467 = *__slate_slot_898;
                                                                                }
                                                                                std::ptr::write(__slate_slot_895, *__slate_slot_465);
                                                                                std::ptr::write(__slate_slot_896, *__slate_slot_895 + ((1 as i32) as i64));
                                                                                *__slate_slot_465 = *__slate_slot_896;
                                                                            } else {
                                                                                break;
                                                                            }
                                                                        }
                                                                        if *__slate_slot_465
                                                                            == *__slate_slot_467
                                                                        {
                                                                            loop {
                                                                                if ((unsafe {
                                                                                    *unsafe {
                                                                                        (*__slate_slot_464).offset(*__slate_slot_467 as isize)
                                                                                    }
                                                                                })
                                                                                    as i32)
                                                                                    & (192 as i32)
                                                                                    == (128 as i32)
                                                                                {
                                                                                    std::ptr::write(__slate_slot_899, *__slate_slot_467);
                                                                                    std::ptr::write(__slate_slot_900, *__slate_slot_899 + ((1 as i32) as i64));
                                                                                    *__slate_slot_467 = *__slate_slot_900;
                                                                                } else {
                                                                                    break '__join_131;
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                            *__slate_slot_466 = (0 as i32) as i64;
                                                            *__slate_slot_465 = (0 as i32) as i64;
                                                            '__loop_120: loop {
                                                                if *__slate_slot_465
                                                                    < *__slate_slot_467
                                                                {
                                                                    std::ptr::write(
                                                                        __slate_slot_903,
                                                                        unsafe {
                                                                            *unsafe {
                                                                                (*__slate_slot_464 as *mut u8).offset(*__slate_slot_465 as isize)
                                                                            }
                                                                        },
                                                                    );
                                                                    *__slate_slot_469 =
                                                                        *__slate_slot_903;
                                                                    if ((*__slate_slot_903 as u32)
                                                                        as i32)
                                                                        <= (31 as i32)
                                                                        || ((*__slate_slot_469
                                                                            as u32)
                                                                            as i32)
                                                                            == (34 as i32)
                                                                        || ((*__slate_slot_469
                                                                            as u32)
                                                                            as i32)
                                                                            == (92 as i32)
                                                                    {
                                                                        if *__slate_slot_466
                                                                            < *__slate_slot_465
                                                                        {
                                                                            sqlite3StrAppend64(pAccum, (unsafe { (*__slate_slot_464).offset(*__slate_slot_466 as isize) }) as *const i8, *__slate_slot_465 - *__slate_slot_466);
                                                                        }
                                                                        *__slate_slot_466 =
                                                                            *__slate_slot_465
                                                                                + ((1 as i32)
                                                                                    as i64);
                                                                        if ((*__slate_slot_469
                                                                            as u32)
                                                                            as i32)
                                                                            == (0 as i32)
                                                                        {
                                                                            break '__loop_120;
                                                                        } else {
                                                                            sqlite3_str_appendchar(
                                                                                pAccum,
                                                                                1 as i32,
                                                                                (92 as i32) as i8,
                                                                            );
                                                                            if ((*__slate_slot_469
                                                                                as u32)
                                                                                as i32)
                                                                                > (31 as i32)
                                                                            {
                                                                                sqlite3_str_appendchar(pAccum, 1 as i32, *__slate_slot_469 as i8);
                                                                            } else {
                                                                                if (1 as u32) << ((*__slate_slot_469 as u32) as i32) & ((14080 as i32) as u32) != ((0 as i32) as u32) {
*__slate_slot_469 = (unsafe { *unsafe { (b"btn?fr\0".as_ptr() as *mut i8).offset((((*__slate_slot_469 as u32) as i32) - (8 as i32)) as isize) } }) as u8;
sqlite3_str_appendchar(pAccum, 1 as i32, *__slate_slot_469 as i8);
} else {
sqlite3_str_append(pAccum, (b"u00\0".as_ptr() as *mut i8) as *const i8, 3 as i32);
sqlite3_str_appendchar(pAccum, 1 as i32, unsafe { *unsafe { unsafe { std::ptr::addr_of!(aHex.0) as *const i8 }.offset((((*__slate_slot_469 as u32) as i32) >> (4 as i32)) as isize) } });
sqlite3_str_appendchar(pAccum, 1 as i32, unsafe { *unsafe { unsafe { std::ptr::addr_of!(aHex.0) as *const i8 }.offset((((*__slate_slot_469 as u32) as i32) & (15 as i32)) as isize) } });
}
                                                                            }
                                                                        }
                                                                    }
                                                                    std::ptr::write(
                                                                        __slate_slot_901,
                                                                        *__slate_slot_465,
                                                                    );
                                                                    std::ptr::write(
                                                                        __slate_slot_902,
                                                                        *__slate_slot_901
                                                                            + ((1 as i32) as i64),
                                                                    );
                                                                    *__slate_slot_465 =
                                                                        *__slate_slot_902;
                                                                } else {
                                                                    break;
                                                                }
                                                            }
                                                            if *__slate_slot_466 < *__slate_slot_465
                                                            {
                                                                sqlite3StrAppend64(
                                                                    pAccum,
                                                                    (unsafe {
                                                                        (*__slate_slot_464).offset(
                                                                            *__slate_slot_466
                                                                                as isize,
                                                                        )
                                                                    })
                                                                        as *const i8,
                                                                    *__slate_slot_465
                                                                        - *__slate_slot_466,
                                                                );
                                                            }
                                                            if ((*__slate_slot_422 as u32) as i32)
                                                                == (18 as i32)
                                                            {
                                                                sqlite3_str_append(
                                                                    pAccum,
                                                                    (b"\"\0".as_ptr() as *mut i8)
                                                                        as *const i8,
                                                                    1 as i32,
                                                                );
                                                            }
                                                        }
                                                        if *__slate_slot_413 > ((0 as i32) as i64) {
                                                            *__slate_slot_904 =
                                                                sqlite3_str_errcode(pAccum)
                                                                    == (0 as i32);
                                                        } else {
                                                            *__slate_slot_904 = false as bool;
                                                        }
                                                        if *__slate_slot_904 {
                                                            '__join_103: {
                                                                std::ptr::write(
                                                                    __slate_slot_470,
                                                                    (sqlite3_str_length(pAccum)
                                                                        as i64)
                                                                        - *__slate_slot_468,
                                                                );
                                                                std::ptr::write(
                                                                    __slate_slot_471,
                                                                    *__slate_slot_470,
                                                                );
                                                                if *__slate_slot_417 != (0 as u8)
                                                                    && *__slate_slot_470
                                                                        > ((0 as i32) as i64)
                                                                {
                                                                    *__slate_slot_472 =
                                                                        sqlite3_str_value(pAccum);
                                                                    *__slate_slot_465 =
                                                                        *__slate_slot_468;
                                                                    loop {
                                                                        if (unsafe {
                                                                            *unsafe {
                                                                                (*__slate_slot_472).offset(*__slate_slot_465 as isize)
                                                                            }
                                                                        }) != (0 as i8)
                                                                        {
                                                                            if ((unsafe {
                                                                                *unsafe {
                                                                                    (*__slate_slot_472).offset(*__slate_slot_465 as isize)
                                                                                }
                                                                            })
                                                                                as i32)
                                                                                & (192 as i32)
                                                                                == (128 as i32)
                                                                            {
                                                                                std::ptr::write(__slate_slot_907, *__slate_slot_471);
                                                                                std::ptr::write(__slate_slot_908, *__slate_slot_907 - ((1 as i32) as i64));
                                                                                *__slate_slot_471 = *__slate_slot_908;
                                                                            }
                                                                            std::ptr::write(
                                                                                __slate_slot_905,
                                                                                *__slate_slot_465,
                                                                            );
                                                                            std::ptr::write(
                                                                                __slate_slot_906,
                                                                                *__slate_slot_905
                                                                                    + ((1 as i32)
                                                                                        as i64),
                                                                            );
                                                                            *__slate_slot_465 =
                                                                                *__slate_slot_906;
                                                                        } else {
                                                                            break '__join_103;
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                            if *__slate_slot_413 > *__slate_slot_471
                                                            {
                                                                std::ptr::write(
                                                                    __slate_slot_473,
                                                                    *__slate_slot_413
                                                                        - *__slate_slot_471,
                                                                );
                                                                0 as i32;
                                                                sqlite3StrAppendchar64(
                                                                    pAccum,
                                                                    (*__slate_slot_473 as i32)
                                                                        as i64,
                                                                    (32 as i32) as i8,
                                                                );
                                                                if !(*__slate_slot_414 != (0 as u8))
                                                                    && *__slate_slot_470
                                                                        > ((0 as i32) as i64)
                                                                {
                                                                    *__slate_slot_909 =
                                                                        sqlite3_str_errcode(pAccum)
                                                                            == (0 as i32);
                                                                } else {
                                                                    *__slate_slot_909 =
                                                                        false as bool;
                                                                }
                                                                if *__slate_slot_909 {
                                                                    *__slate_slot_472 =
                                                                        sqlite3_str_value(pAccum);
                                                                    std::ptr::write(
                                                                        __slate_slot_910,
                                                                        *__slate_slot_472,
                                                                    );
                                                                    std::ptr::write(
                                                                        __slate_slot_911,
                                                                        unsafe {
                                                                            (*__slate_slot_910)
                                                                                .offset(
                                                                                *__slate_slot_468
                                                                                    as isize,
                                                                            )
                                                                        },
                                                                    );
                                                                    *__slate_slot_472 =
                                                                        *__slate_slot_911;
                                                                    unsafe {
                                                                        memmove(
                                                                            (unsafe {
                                                                                (*__slate_slot_472).offset(*__slate_slot_473 as isize)
                                                                            })
                                                                                as *mut (),
                                                                            *__slate_slot_472
                                                                                as *const (),
                                                                            *__slate_slot_470
                                                                                as u64,
                                                                        )
                                                                    };
                                                                    unsafe {
                                                                        memset(
                                                                            *__slate_slot_472
                                                                                as *mut (),
                                                                            32 as i32,
                                                                            *__slate_slot_473
                                                                                as u64,
                                                                        )
                                                                    };
                                                                    break '__join_2;
                                                                } else {
                                                                    break '__join_2;
                                                                }
                                                            } else {
                                                                break '__join_2;
                                                            }
                                                        } else {
                                                            break '__join_2;
                                                        }
                                                    }
                                                    if *__slate_slot_423 != (0 as u8) {
                                                        *__slate_slot_409 =
                                                            getTextArg(*__slate_slot_435);
                                                        *__slate_slot_422 =
                                                            ((5 as i32) as i8) as u8;
                                                    } else {
                                                        *__slate_slot_409 =
                                                            unsafe { ap.next_arg::<*mut i8>() };
                                                    }
                                                    if *__slate_slot_409
                                                        == std::ptr::null_mut::<i8>()
                                                    {
                                                        *__slate_slot_409 =
                                                            b"\0".as_ptr() as *mut i8;
                                                    } else {
                                                        if ((*__slate_slot_422 as u32) as i32)
                                                            == (6 as i32)
                                                        {
                                                            if (unsafe { (*pAccum).nChar })
                                                                == ((0 as i32) as u32)
                                                                && (unsafe { (*pAccum).mxAlloc })
                                                                    != (0 as u32)
                                                                && *__slate_slot_413
                                                                    == ((0 as i32) as i64)
                                                                && *__slate_slot_410
                                                                    < ((0 as i32) as i64)
                                                                && (((unsafe { (*pAccum).accError })
                                                                    as u32)
                                                                    as i32)
                                                                    == (0 as i32)
                                                            {
                                                                // /* Special optimization for sqlite3_mprintf("%z..."):
                                                                //             ** Extend an existing memory allocation rather than creating
                                                                //             ** a new one. */
                                                                0 as i32;
                                                                unsafe {
                                                                    (*pAccum).zText =
                                                                        *__slate_slot_409;
                                                                }
                                                                unsafe {
                                                                    (*pAccum).nAlloc = (unsafe {
                                                                        sqlite3DbMallocSize(
                                                                            unsafe { (*pAccum).db },
                                                                            *__slate_slot_409
                                                                                as *const (),
                                                                        )
                                                                    })
                                                                        as u32;
                                                                }
                                                                unsafe {
                                                                    (*pAccum).nChar = ((2147483647
                                                                        as i32)
                                                                        & (((unsafe {
                                                                            strlen(
                                                                                *__slate_slot_409
                                                                                    as *const i8,
                                                                            )
                                                                        })
                                                                            as u32)
                                                                            as i32))
                                                                        as u32;
                                                                }
                                                                std::ptr::write(
                                                                    __slate_slot_880,
                                                                    pAccum,
                                                                );
                                                                std::ptr::write(
                                                                    __slate_slot_881,
                                                                    unsafe {
                                                                        (*(*__slate_slot_880))
                                                                            .printfFlags
                                                                    },
                                                                );
                                                                std::ptr::write(
                                                                    __slate_slot_882,
                                                                    ((((*__slate_slot_881 as u32)
                                                                        as i32)
                                                                        | (4 as i32))
                                                                        as i8)
                                                                        as u8,
                                                                );
                                                                unsafe {
                                                                    (*(*__slate_slot_880))
                                                                        .printfFlags =
                                                                        *__slate_slot_882;
                                                                }
                                                                *__slate_slot_411 =
                                                                    (0 as i32) as i64;
                                                                break '__join_11;
                                                            } else {
                                                                *__slate_slot_430 =
                                                                    *__slate_slot_409;
                                                            }
                                                        }
                                                    }
                                                    if *__slate_slot_410 >= ((0 as i32) as i64) {
                                                        if *__slate_slot_417 != (0 as u8) {
                                                            // /* Set length to the number of bytes needed in order to display
                                                            //             ** precision characters */
                                                            std::ptr::write(
                                                                __slate_slot_462,
                                                                *__slate_slot_409 as *mut u8,
                                                            );
                                                            loop {
                                                                std::ptr::write(
                                                                    __slate_slot_883,
                                                                    *__slate_slot_410,
                                                                );
                                                                std::ptr::write(
                                                                    __slate_slot_884,
                                                                    *__slate_slot_883
                                                                        - ((1 as i32) as i64),
                                                                );
                                                                *__slate_slot_410 =
                                                                    *__slate_slot_884;
                                                                if *__slate_slot_883
                                                                    > ((0 as i32) as i64)
                                                                    && (unsafe {
                                                                        *unsafe {
                                                                            (*__slate_slot_462)
                                                                                .offset(
                                                                                    (0 as i32)
                                                                                        as isize,
                                                                                )
                                                                        }
                                                                    }) != (0 as u8)
                                                                {
                                                                    '__join_156: {
                                                                        std::ptr::write(
                                                                            __slate_slot_885,
                                                                            *__slate_slot_462,
                                                                        );
                                                                        std::ptr::write(
                                                                            __slate_slot_886,
                                                                            unsafe {
                                                                                (*__slate_slot_885)
                                                                                    .offset(
                                                                                    (1 as i32)
                                                                                        as isize,
                                                                                )
                                                                            },
                                                                        );
                                                                        *__slate_slot_462 =
                                                                            *__slate_slot_886;
                                                                        if (((unsafe {
                                                                            *(*__slate_slot_885)
                                                                        })
                                                                            as u32)
                                                                            as i32)
                                                                            >= (192 as i32)
                                                                        {
                                                                            loop {
                                                                                if (((unsafe {
                                                                                    *(*__slate_slot_462)
                                                                                })
                                                                                    as u32)
                                                                                    as i32)
                                                                                    & (192 as i32)
                                                                                    == (128 as i32)
                                                                                {
                                                                                    std::ptr::write(__slate_slot_887, *__slate_slot_462);
                                                                                    std::ptr::write(__slate_slot_888, unsafe { (*__slate_slot_887).offset((1 as i32) as isize) });
                                                                                    *__slate_slot_462 = *__slate_slot_888;
                                                                                } else {
                                                                                    break '__join_156;
                                                                                }
                                                                            }
                                                                        }
                                                                    }
                                                                    {}
                                                                } else {
                                                                    break;
                                                                }
                                                            }
                                                            *__slate_slot_411 = (unsafe {
                                                                (*__slate_slot_462).offset_from(
                                                                    (*__slate_slot_409 as *mut u8)
                                                                        as *mut u8,
                                                                )
                                                            })
                                                                as i64;
                                                        } else {
                                                            *__slate_slot_411 = (0 as i32) as i64;
                                                            loop {
                                                                if *__slate_slot_411
                                                                    < *__slate_slot_410
                                                                    && (unsafe {
                                                                        *unsafe {
                                                                            (*__slate_slot_409)
                                                                                .offset(
                                                                                *__slate_slot_411
                                                                                    as isize,
                                                                            )
                                                                        }
                                                                    }) != (0 as i8)
                                                                {
                                                                    std::ptr::write(
                                                                        __slate_slot_889,
                                                                        *__slate_slot_411,
                                                                    );
                                                                    std::ptr::write(
                                                                        __slate_slot_890,
                                                                        *__slate_slot_889
                                                                            + ((1 as i32) as i64),
                                                                    );
                                                                    *__slate_slot_411 =
                                                                        *__slate_slot_890;
                                                                } else {
                                                                    break '__join_153;
                                                                }
                                                            }
                                                        }
                                                    } else {
                                                        *__slate_slot_411 = (unsafe {
                                                            strlen(*__slate_slot_409 as *const i8)
                                                        })
                                                            as i64;
                                                    }
                                                }
                                                if *__slate_slot_417 != (0 as u8)
                                                    && *__slate_slot_413 > ((0 as i32) as i64)
                                                {
                                                    // /* Adjust width to account for extra bytes in UTF-8 characters */
                                                    std::ptr::write(
                                                        __slate_slot_463,
                                                        *__slate_slot_411 - ((1 as i32) as i64),
                                                    );
                                                    loop {
                                                        if *__slate_slot_463 >= ((0 as i32) as i64)
                                                        {
                                                            std::ptr::write(
                                                                __slate_slot_891,
                                                                *__slate_slot_463,
                                                            );
                                                            std::ptr::write(
                                                                __slate_slot_892,
                                                                *__slate_slot_891
                                                                    - ((1 as i32) as i64),
                                                            );
                                                            *__slate_slot_463 = *__slate_slot_892;
                                                            if ((unsafe {
                                                                *unsafe {
                                                                    (*__slate_slot_409).offset(
                                                                        *__slate_slot_891 as isize,
                                                                    )
                                                                }
                                                            })
                                                                as i32)
                                                                & (192 as i32)
                                                                == (128 as i32)
                                                            {
                                                                std::ptr::write(
                                                                    __slate_slot_893,
                                                                    *__slate_slot_413,
                                                                );
                                                                std::ptr::write(
                                                                    __slate_slot_894,
                                                                    *__slate_slot_893
                                                                        + ((1 as i32) as i64),
                                                                );
                                                                *__slate_slot_413 =
                                                                    *__slate_slot_894;
                                                            }
                                                        } else {
                                                            break '__join_11;
                                                        }
                                                    }
                                                } else {
                                                    break '__join_11;
                                                }
                                            }
                                            // /* Size needed to hold the output */
                                            if *__slate_slot_423 != (0 as u8) {
                                                *__slate_slot_426 = getDoubleArg(*__slate_slot_435);
                                            } else {
                                                *__slate_slot_426 = unsafe { ap.next_arg::<f64>() };
                                            }
                                            // /* Set default precision */
                                            if *__slate_slot_410 < ((0 as i32) as i64) {
                                                *__slate_slot_410 = (6 as i32) as i64;
                                            }
                                            if *__slate_slot_410 > ((100000000 as i32) as i64) {
                                                *__slate_slot_410 = (100000000 as i32) as i64;
                                            }
                                            if ((*__slate_slot_422 as u32) as i32) == (1 as i32) {
                                                *__slate_slot_452 = -(*__slate_slot_410) as i32;
                                            } else {
                                                if ((*__slate_slot_422 as u32) as i32) == (3 as i32)
                                                {
                                                    '__join_288: {
                                                        if *__slate_slot_410 == ((0 as i32) as i64)
                                                        {
                                                            *__slate_slot_410 = (1 as i32) as i64;
                                                        }
                                                    }
                                                    *__slate_slot_452 = *__slate_slot_410 as i32;
                                                } else {
                                                    *__slate_slot_452 = (*__slate_slot_410
                                                        + ((1 as i32) as i64))
                                                        as i32;
                                                }
                                            }
                                            unsafe {
                                                sqlite3FpDecode(
                                                    std::ptr::addr_of_mut!(*__slate_slot_451),
                                                    *__slate_slot_426,
                                                    *__slate_slot_452,
                                                    if *__slate_slot_417 != (0 as u8) {
                                                        20 as i32
                                                    } else {
                                                        16 as i32
                                                    },
                                                )
                                            };
                                            if (*__slate_slot_451).isSpecial != (0 as i8) {
                                                if ((*__slate_slot_451).isSpecial as i32)
                                                    == (2 as i32)
                                                {
                                                    if *__slate_slot_418 != (0 as u8) {
                                                        *__slate_slot_409 =
                                                            b"null\0".as_ptr() as *mut i8;
                                                        *__slate_slot_411 = (4 as i32) as i64;
                                                        break '__join_11;
                                                    } else {
                                                        *__slate_slot_409 =
                                                            b"NaN\0".as_ptr() as *mut i8;
                                                        *__slate_slot_411 = (3 as i32) as i64;
                                                        break '__join_11;
                                                    }
                                                } else {
                                                    if *__slate_slot_418 != (0 as u8) {
                                                        unsafe {
                                                            *unsafe {
                                                                (*__slate_slot_451)
                                                                    .z
                                                                    .offset((0 as i32) as isize)
                                                            } = (57 as i32) as i8;
                                                        }
                                                        (*__slate_slot_451).iDP = 1000 as i32;
                                                        (*__slate_slot_451).n = 1 as i32;
                                                    } else {
                                                        unsafe {
                                                            memcpy(
                                                                ((*__slate_slot_436).as_mut_ptr()
                                                                    as *mut i8)
                                                                    as *mut (),
                                                                (b"-Inf\0".as_ptr() as *mut i8)
                                                                    as *const (),
                                                                ((5 as i32) as i64) as u64,
                                                            )
                                                        };
                                                        *__slate_slot_409 = (*__slate_slot_436)
                                                            .as_mut_ptr()
                                                            as *mut i8;
                                                        if ((*__slate_slot_451).sign as i32)
                                                            == (45 as i32)
                                                        {
                                                            // /* no-op */
                                                        } else {
                                                            if *__slate_slot_415 != (0 as u8) {
                                                                unsafe {
                                                                    *unsafe {
                                                                        ((*__slate_slot_436)
                                                                            .as_mut_ptr()
                                                                            as *mut i8)
                                                                            .offset(
                                                                                (0 as i32) as isize,
                                                                            )
                                                                    } = *__slate_slot_415 as i8;
                                                                }
                                                            } else {
                                                                std::ptr::write(
                                                                    __slate_slot_805,
                                                                    *__slate_slot_409,
                                                                );
                                                                std::ptr::write(
                                                                    __slate_slot_806,
                                                                    unsafe {
                                                                        (*__slate_slot_805).offset(
                                                                            (1 as i32) as isize,
                                                                        )
                                                                    },
                                                                );
                                                                *__slate_slot_409 =
                                                                    *__slate_slot_806;
                                                            }
                                                        }
                                                        *__slate_slot_411 = (unsafe {
                                                            strlen(*__slate_slot_409 as *const i8)
                                                        })
                                                            as i64;
                                                        break '__join_11;
                                                    }
                                                }
                                            }
                                            if ((*__slate_slot_451).sign as i32) == (45 as i32) {
                                                if *__slate_slot_416 != (0 as u8)
                                                    && !(*__slate_slot_415 != (0 as u8))
                                                    && ((*__slate_slot_422 as u32) as i32)
                                                        == (1 as i32)
                                                    && (*__slate_slot_451).iDP <= *__slate_slot_452
                                                {
                                                    // /* Suppress the minus sign if all of the following are true:
                                                    //             **   *  The value displayed is zero
                                                    //             **   *  The '#' flag is used
                                                    //             **   *  The '+' flag is not used, and
                                                    //             **   *  The format is %f
                                                    //             */
                                                    *__slate_slot_424 = (0 as i32) as i8;
                                                } else {
                                                    *__slate_slot_424 = (45 as i32) as i8;
                                                }
                                            } else {
                                                *__slate_slot_424 = *__slate_slot_415 as i8;
                                            }
                                            *__slate_slot_431 =
                                                (*__slate_slot_451).iDP - (1 as i32);
                                            // /*
                                            //         ** If the field type is etGENERIC, then convert to either etEXP
                                            //         ** or etFLOAT, as appropriate.
                                            //         */
                                            if ((*__slate_slot_422 as u32) as i32) == (3 as i32) {
                                                0 as i32;
                                                std::ptr::write(
                                                    __slate_slot_807,
                                                    *__slate_slot_410,
                                                );
                                                std::ptr::write(
                                                    __slate_slot_808,
                                                    *__slate_slot_807 - ((1 as i32) as i64),
                                                );
                                                *__slate_slot_410 = *__slate_slot_808;
                                                *__slate_slot_434 =
                                                    !(*__slate_slot_416 != (0 as u8)) as u8;
                                                if *__slate_slot_431 < -(4 as i32)
                                                    || (*__slate_slot_431 as i64)
                                                        > *__slate_slot_410
                                                {
                                                    *__slate_slot_422 = ((2 as i32) as i8) as u8;
                                                } else {
                                                    *__slate_slot_410 = *__slate_slot_410
                                                        - (*__slate_slot_431 as i64);
                                                    *__slate_slot_422 = ((1 as i32) as i8) as u8;
                                                }
                                            } else {
                                                *__slate_slot_434 = *__slate_slot_417;
                                            }
                                            if ((*__slate_slot_422 as u32) as i32) == (2 as i32) {
                                                *__slate_slot_432 = 0 as i32;
                                            } else {
                                                *__slate_slot_432 =
                                                    (*__slate_slot_451).iDP - (1 as i32);
                                            }
                                            *__slate_slot_454 = ((if *__slate_slot_432 > (0 as i32)
                                            {
                                                *__slate_slot_432
                                            } else {
                                                0 as i32
                                            })
                                                as i64)
                                                + *__slate_slot_410
                                                + *__slate_slot_413
                                                + ((10 as i32) as i64);
                                            if *__slate_slot_421 != (0 as u8)
                                                && *__slate_slot_432 > (0 as i32)
                                            {
                                                std::ptr::write(
                                                    __slate_slot_809,
                                                    *__slate_slot_454,
                                                );
                                                std::ptr::write(
                                                    __slate_slot_810,
                                                    *__slate_slot_809
                                                        + (((*__slate_slot_432 + (2 as i32))
                                                            / (3 as i32))
                                                            as i64),
                                                );
                                                *__slate_slot_454 = *__slate_slot_810;
                                            }
                                            if *__slate_slot_454
                                                + (((unsafe { (*pAccum).nChar }) as u64) as i64)
                                                >= (((unsafe { (*pAccum).nAlloc }) as u64) as i64)
                                            {
                                                if (unsafe { (*pAccum).mxAlloc })
                                                    == ((0 as i32) as u32)
                                                    && (((unsafe { (*pAccum).accError }) as u32)
                                                        as i32)
                                                        == (0 as i32)
                                                {
                                                    // /* Unable to allocate space in pAccum, perhaps because it
                                                    //             ** is coming from sqlite3_snprintf() or similar.  We'll have
                                                    //             ** to render into temporary space and the memcpy() it over. */
                                                    *__slate_slot_409 = (unsafe {
                                                        sqlite3_malloc(*__slate_slot_454 as i32)
                                                    })
                                                        as *mut i8;
                                                    if *__slate_slot_409
                                                        == std::ptr::null_mut::<i8>()
                                                    {
                                                        sqlite3StrAccumSetError(
                                                            pAccum,
                                                            ((7 as i32) as i8) as u8,
                                                        );
                                                        return;
                                                    } else {
                                                        *__slate_slot_430 = *__slate_slot_409;
                                                    }
                                                } else {
                                                    if (sqlite3StrAccumEnlarge(
                                                        pAccum,
                                                        *__slate_slot_454,
                                                    )
                                                        as i64)
                                                        < *__slate_slot_454
                                                    {
                                                        *__slate_slot_411 = (0 as i32) as i64;
                                                        *__slate_slot_413 = (0 as i32) as i64;
                                                        break '__join_11;
                                                    } else {
                                                        *__slate_slot_409 = unsafe {
                                                            unsafe { (*pAccum).zText }.offset(
                                                                (unsafe { (*pAccum).nChar })
                                                                    as isize,
                                                            )
                                                        };
                                                    }
                                                }
                                            } else {
                                                *__slate_slot_409 = unsafe {
                                                    unsafe { (*pAccum).zText }.offset(
                                                        (unsafe { (*pAccum).nChar }) as isize,
                                                    )
                                                };
                                            }
                                            *__slate_slot_428 = *__slate_slot_409;
                                            *__slate_slot_433 =
                                                (((if *__slate_slot_410 > ((0 as i32) as i64) {
                                                    1 as i32
                                                } else {
                                                    0 as i32
                                                }) | ((*__slate_slot_416 as u32) as i32)
                                                    | ((*__slate_slot_417 as u32) as i32))
                                                    as i8)
                                                    as u8;
                                            // /* The sign in front of the number */
                                            if *__slate_slot_424 != (0 as i8) {
                                                std::ptr::write(
                                                    __slate_slot_811,
                                                    *__slate_slot_409,
                                                );
                                                std::ptr::write(__slate_slot_812, unsafe {
                                                    (*__slate_slot_811).offset((1 as i32) as isize)
                                                });
                                                *__slate_slot_409 = *__slate_slot_812;
                                                unsafe {
                                                    *(*__slate_slot_811) = *__slate_slot_424;
                                                }
                                            }
                                            '__join_233: {
                                                // /* Digits prior to the decimal point */
                                                *__slate_slot_453 = 0 as i32;
                                                0 as i32;
                                                if *__slate_slot_432 < (0 as i32) {
                                                    std::ptr::write(
                                                        __slate_slot_813,
                                                        *__slate_slot_409,
                                                    );
                                                    std::ptr::write(__slate_slot_814, unsafe {
                                                        (*__slate_slot_813)
                                                            .offset((1 as i32) as isize)
                                                    });
                                                    *__slate_slot_409 = *__slate_slot_814;
                                                    unsafe {
                                                        *(*__slate_slot_813) = (48 as i32) as i8;
                                                    }
                                                } else {
                                                    if *__slate_slot_421 != (0 as u8) {
                                                        loop {
                                                            if *__slate_slot_432 >= (0 as i32) {
                                                                if *__slate_slot_453
                                                                    < (*__slate_slot_451).n
                                                                {
                                                                    std::ptr::write(
                                                                        __slate_slot_818,
                                                                        *__slate_slot_453,
                                                                    );
                                                                    std::ptr::write(
                                                                        __slate_slot_819,
                                                                        *__slate_slot_818
                                                                            + (1 as i32),
                                                                    );
                                                                    *__slate_slot_453 =
                                                                        *__slate_slot_819;
                                                                    *__slate_slot_817 = (unsafe {
                                                                        *unsafe {
                                                                            (*__slate_slot_451)
                                                                                .z
                                                                                .offset(
                                                                                *__slate_slot_818
                                                                                    as isize,
                                                                            )
                                                                        }
                                                                    })
                                                                        as i32;
                                                                } else {
                                                                    *__slate_slot_817 = 48 as i32;
                                                                }
                                                                std::ptr::write(
                                                                    __slate_slot_820,
                                                                    *__slate_slot_409,
                                                                );
                                                                std::ptr::write(
                                                                    __slate_slot_821,
                                                                    unsafe {
                                                                        (*__slate_slot_820).offset(
                                                                            (1 as i32) as isize,
                                                                        )
                                                                    },
                                                                );
                                                                *__slate_slot_409 =
                                                                    *__slate_slot_821;
                                                                unsafe {
                                                                    *(*__slate_slot_820) =
                                                                        *__slate_slot_817 as i8;
                                                                }
                                                                if *__slate_slot_432 % (3 as i32)
                                                                    == (0 as i32)
                                                                    && *__slate_slot_432
                                                                        > (1 as i32)
                                                                {
                                                                    std::ptr::write(
                                                                        __slate_slot_822,
                                                                        *__slate_slot_409,
                                                                    );
                                                                    std::ptr::write(
                                                                        __slate_slot_823,
                                                                        unsafe {
                                                                            (*__slate_slot_822)
                                                                                .offset(
                                                                                    (1 as i32)
                                                                                        as isize,
                                                                                )
                                                                        },
                                                                    );
                                                                    *__slate_slot_409 =
                                                                        *__slate_slot_823;
                                                                    unsafe {
                                                                        *(*__slate_slot_822) =
                                                                            (44 as i32) as i8;
                                                                    }
                                                                }
                                                                std::ptr::write(
                                                                    __slate_slot_815,
                                                                    *__slate_slot_432,
                                                                );
                                                                std::ptr::write(
                                                                    __slate_slot_816,
                                                                    *__slate_slot_815 - (1 as i32),
                                                                );
                                                                *__slate_slot_432 =
                                                                    *__slate_slot_816;
                                                            } else {
                                                                break '__join_233;
                                                            }
                                                        }
                                                    } else {
                                                        *__slate_slot_453 =
                                                            *__slate_slot_432 + (1 as i32);
                                                        if *__slate_slot_453 > (*__slate_slot_451).n
                                                        {
                                                            *__slate_slot_453 =
                                                                (*__slate_slot_451).n;
                                                        }
                                                        unsafe {
                                                            memcpy(
                                                                *__slate_slot_409 as *mut (),
                                                                (*__slate_slot_451).z as *const (),
                                                                (*__slate_slot_453 as i64) as u64,
                                                            )
                                                        };
                                                        std::ptr::write(
                                                            __slate_slot_824,
                                                            *__slate_slot_409,
                                                        );
                                                        std::ptr::write(__slate_slot_825, unsafe {
                                                            (*__slate_slot_824)
                                                                .offset(*__slate_slot_453 as isize)
                                                        });
                                                        *__slate_slot_409 = *__slate_slot_825;
                                                        std::ptr::write(
                                                            __slate_slot_826,
                                                            *__slate_slot_432,
                                                        );
                                                        std::ptr::write(
                                                            __slate_slot_827,
                                                            *__slate_slot_826 - *__slate_slot_453,
                                                        );
                                                        *__slate_slot_432 = *__slate_slot_827;
                                                        if *__slate_slot_432 >= (0 as i32) {
                                                            unsafe {
                                                                memset(
                                                                    *__slate_slot_409 as *mut (),
                                                                    48 as i32,
                                                                    ((*__slate_slot_432
                                                                        + (1 as i32))
                                                                        as i64)
                                                                        as u64,
                                                                )
                                                            };
                                                            std::ptr::write(
                                                                __slate_slot_828,
                                                                *__slate_slot_409,
                                                            );
                                                            std::ptr::write(
                                                                __slate_slot_829,
                                                                unsafe {
                                                                    (*__slate_slot_828).offset(
                                                                        (*__slate_slot_432
                                                                            + (1 as i32))
                                                                            as isize,
                                                                    )
                                                                },
                                                            );
                                                            *__slate_slot_409 = *__slate_slot_829;
                                                            *__slate_slot_432 = -(1 as i32);
                                                        }
                                                    }
                                                }
                                            }
                                            // /* The decimal point */
                                            if *__slate_slot_433 != (0 as u8) {
                                                std::ptr::write(
                                                    __slate_slot_830,
                                                    *__slate_slot_409,
                                                );
                                                std::ptr::write(__slate_slot_831, unsafe {
                                                    (*__slate_slot_830).offset((1 as i32) as isize)
                                                });
                                                *__slate_slot_409 = *__slate_slot_831;
                                                unsafe {
                                                    *(*__slate_slot_830) = (46 as i32) as i8;
                                                }
                                            }
                                            // /* "0" digits after the decimal point but before the first
                                            //         ** significant digit of the number */
                                            if *__slate_slot_432 < -(1 as i32)
                                                && *__slate_slot_410 > ((0 as i32) as i64)
                                            {
                                                std::ptr::write(
                                                    __slate_slot_455,
                                                    -(1 as i32) - *__slate_slot_432,
                                                );
                                                if (*__slate_slot_455 as i64) > *__slate_slot_410 {
                                                    *__slate_slot_455 = *__slate_slot_410 as i32;
                                                }
                                                unsafe {
                                                    memset(
                                                        *__slate_slot_409 as *mut (),
                                                        48 as i32,
                                                        (*__slate_slot_455 as i64) as u64,
                                                    )
                                                };
                                                std::ptr::write(
                                                    __slate_slot_832,
                                                    *__slate_slot_409,
                                                );
                                                std::ptr::write(__slate_slot_833, unsafe {
                                                    (*__slate_slot_832)
                                                        .offset(*__slate_slot_455 as isize)
                                                });
                                                *__slate_slot_409 = *__slate_slot_833;
                                                std::ptr::write(
                                                    __slate_slot_834,
                                                    *__slate_slot_410,
                                                );
                                                std::ptr::write(
                                                    __slate_slot_835,
                                                    *__slate_slot_834 - (*__slate_slot_455 as i64),
                                                );
                                                *__slate_slot_410 = *__slate_slot_835;
                                            }
                                            // /* Significant digits after the decimal point */
                                            if *__slate_slot_410 > ((0 as i32) as i64) {
                                                std::ptr::write(
                                                    __slate_slot_456,
                                                    (*__slate_slot_451).n - *__slate_slot_453,
                                                );
                                                if (*__slate_slot_456 as i64) > *__slate_slot_410 {
                                                    *__slate_slot_456 = *__slate_slot_410 as i32;
                                                }
                                                if *__slate_slot_456 > (0 as i32) {
                                                    unsafe {
                                                        memcpy(
                                                            *__slate_slot_409 as *mut (),
                                                            (unsafe {
                                                                (*__slate_slot_451).z.offset(
                                                                    *__slate_slot_453 as isize,
                                                                )
                                                            })
                                                                as *const (),
                                                            (*__slate_slot_456 as i64) as u64,
                                                        )
                                                    };
                                                    std::ptr::write(
                                                        __slate_slot_836,
                                                        *__slate_slot_409,
                                                    );
                                                    std::ptr::write(__slate_slot_837, unsafe {
                                                        (*__slate_slot_836)
                                                            .offset(*__slate_slot_456 as isize)
                                                    });
                                                    *__slate_slot_409 = *__slate_slot_837;
                                                    std::ptr::write(
                                                        __slate_slot_838,
                                                        *__slate_slot_410,
                                                    );
                                                    std::ptr::write(
                                                        __slate_slot_839,
                                                        *__slate_slot_838
                                                            - (*__slate_slot_456 as i64),
                                                    );
                                                    *__slate_slot_410 = *__slate_slot_839;
                                                }
                                                if *__slate_slot_410 > ((0 as i32) as i64)
                                                    && !(*__slate_slot_434 != (0 as u8))
                                                {
                                                    unsafe {
                                                        memset(
                                                            *__slate_slot_409 as *mut (),
                                                            48 as i32,
                                                            *__slate_slot_410 as u64,
                                                        )
                                                    };
                                                    std::ptr::write(
                                                        __slate_slot_840,
                                                        *__slate_slot_409,
                                                    );
                                                    std::ptr::write(__slate_slot_841, unsafe {
                                                        (*__slate_slot_840)
                                                            .offset(*__slate_slot_410 as isize)
                                                    });
                                                    *__slate_slot_409 = *__slate_slot_841;
                                                }
                                            }
                                            // /* Remove trailing zeros and the "." if no digits follow the "." */
                                            if *__slate_slot_434 != (0 as u8)
                                                && *__slate_slot_433 != (0 as u8)
                                            {
                                                loop {
                                                    if ((unsafe {
                                                        *unsafe {
                                                            (*__slate_slot_409)
                                                                .offset(-(1 as i32) as isize)
                                                        }
                                                    })
                                                        as i32)
                                                        == (48 as i32)
                                                    {
                                                        std::ptr::write(
                                                            __slate_slot_842,
                                                            *__slate_slot_409,
                                                        );
                                                        std::ptr::write(__slate_slot_843, unsafe {
                                                            (*__slate_slot_842)
                                                                .offset(-((1 as i32) as isize))
                                                        });
                                                        *__slate_slot_409 = *__slate_slot_843;
                                                        unsafe {
                                                            *(*__slate_slot_843) = (0 as i32) as i8;
                                                        }
                                                    } else {
                                                        break;
                                                    }
                                                }
                                                0 as i32;
                                                if ((unsafe {
                                                    *unsafe {
                                                        (*__slate_slot_409)
                                                            .offset(-(1 as i32) as isize)
                                                    }
                                                })
                                                    as i32)
                                                    == (46 as i32)
                                                {
                                                    if *__slate_slot_417 != (0 as u8) {
                                                        std::ptr::write(
                                                            __slate_slot_844,
                                                            *__slate_slot_409,
                                                        );
                                                        std::ptr::write(__slate_slot_845, unsafe {
                                                            (*__slate_slot_844)
                                                                .offset((1 as i32) as isize)
                                                        });
                                                        *__slate_slot_409 = *__slate_slot_845;
                                                        unsafe {
                                                            *(*__slate_slot_844) =
                                                                (48 as i32) as i8;
                                                        }
                                                    } else {
                                                        std::ptr::write(
                                                            __slate_slot_846,
                                                            *__slate_slot_409,
                                                        );
                                                        std::ptr::write(__slate_slot_847, unsafe {
                                                            (*__slate_slot_846)
                                                                .offset(-((1 as i32) as isize))
                                                        });
                                                        *__slate_slot_409 = *__slate_slot_847;
                                                        unsafe {
                                                            *(*__slate_slot_847) = (0 as i32) as i8;
                                                        }
                                                    }
                                                }
                                            }
                                            // /* Add the "eNNN" suffix */
                                            if ((*__slate_slot_422 as u32) as i32) == (2 as i32) {
                                                *__slate_slot_431 =
                                                    (*__slate_slot_451).iDP - (1 as i32);
                                                std::ptr::write(
                                                    __slate_slot_848,
                                                    *__slate_slot_409,
                                                );
                                                std::ptr::write(__slate_slot_849, unsafe {
                                                    (*__slate_slot_848).offset((1 as i32) as isize)
                                                });
                                                *__slate_slot_409 = *__slate_slot_849;
                                                unsafe {
                                                    *(*__slate_slot_848) = unsafe {
                                                        *unsafe {
                                                            unsafe {
                                                                std::ptr::addr_of!(aDigits.0)
                                                                    as *const i8
                                                            }
                                                            .offset(
                                                                (((unsafe {
                                                                    (*(*__slate_slot_427)).charset
                                                                })
                                                                    as u32)
                                                                    as i32)
                                                                    as isize,
                                                            )
                                                        }
                                                    };
                                                }
                                                if *__slate_slot_431 < (0 as i32) {
                                                    std::ptr::write(
                                                        __slate_slot_850,
                                                        *__slate_slot_409,
                                                    );
                                                    std::ptr::write(__slate_slot_851, unsafe {
                                                        (*__slate_slot_850)
                                                            .offset((1 as i32) as isize)
                                                    });
                                                    *__slate_slot_409 = *__slate_slot_851;
                                                    unsafe {
                                                        *(*__slate_slot_850) = (45 as i32) as i8;
                                                    }
                                                    *__slate_slot_431 = -(*__slate_slot_431);
                                                } else {
                                                    std::ptr::write(
                                                        __slate_slot_852,
                                                        *__slate_slot_409,
                                                    );
                                                    std::ptr::write(__slate_slot_853, unsafe {
                                                        (*__slate_slot_852)
                                                            .offset((1 as i32) as isize)
                                                    });
                                                    *__slate_slot_409 = *__slate_slot_853;
                                                    unsafe {
                                                        *(*__slate_slot_852) = (43 as i32) as i8;
                                                    }
                                                }
                                                if *__slate_slot_431 >= (100 as i32) {
                                                    // /* 100's digit */
                                                    std::ptr::write(
                                                        __slate_slot_854,
                                                        *__slate_slot_409,
                                                    );
                                                    std::ptr::write(__slate_slot_855, unsafe {
                                                        (*__slate_slot_854)
                                                            .offset((1 as i32) as isize)
                                                    });
                                                    *__slate_slot_409 = *__slate_slot_855;
                                                    unsafe {
                                                        *(*__slate_slot_854) = (*__slate_slot_431
                                                            / (100 as i32)
                                                            + (48 as i32))
                                                            as i8;
                                                    }
                                                    std::ptr::write(
                                                        __slate_slot_856,
                                                        *__slate_slot_431,
                                                    );
                                                    std::ptr::write(
                                                        __slate_slot_857,
                                                        *__slate_slot_856 % (100 as i32),
                                                    );
                                                    *__slate_slot_431 = *__slate_slot_857;
                                                }
                                                // /* 10's digit */
                                                std::ptr::write(
                                                    __slate_slot_858,
                                                    *__slate_slot_409,
                                                );
                                                std::ptr::write(__slate_slot_859, unsafe {
                                                    (*__slate_slot_858).offset((1 as i32) as isize)
                                                });
                                                *__slate_slot_409 = *__slate_slot_859;
                                                unsafe {
                                                    *(*__slate_slot_858) = (*__slate_slot_431
                                                        / (10 as i32)
                                                        + (48 as i32))
                                                        as i8;
                                                }
                                                // /* 1's digit */
                                                std::ptr::write(
                                                    __slate_slot_860,
                                                    *__slate_slot_409,
                                                );
                                                std::ptr::write(__slate_slot_861, unsafe {
                                                    (*__slate_slot_860).offset((1 as i32) as isize)
                                                });
                                                *__slate_slot_409 = *__slate_slot_861;
                                                unsafe {
                                                    *(*__slate_slot_860) = (*__slate_slot_431
                                                        % (10 as i32)
                                                        + (48 as i32))
                                                        as i8;
                                                }
                                            }
                                            *__slate_slot_411 = (unsafe {
                                                (*__slate_slot_409)
                                                    .offset_from(*__slate_slot_428 as *mut i8)
                                            })
                                                as i64;
                                            0 as i32;
                                            if *__slate_slot_411 < *__slate_slot_413 {
                                                '__join_200: {
                                                    std::ptr::write(
                                                        __slate_slot_457,
                                                        *__slate_slot_413 - *__slate_slot_411,
                                                    );
                                                    if *__slate_slot_414 != (0 as u8) {
                                                        unsafe {
                                                            memset(
                                                                *__slate_slot_409 as *mut (),
                                                                32 as i32,
                                                                *__slate_slot_457 as u64,
                                                            )
                                                        };
                                                    } else {
                                                        if !(*__slate_slot_418 != (0 as u8)) {
                                                            unsafe {
                                                                memmove(
                                                                    (unsafe {
                                                                        (*__slate_slot_428).offset(
                                                                            *__slate_slot_457
                                                                                as isize,
                                                                        )
                                                                    })
                                                                        as *mut (),
                                                                    *__slate_slot_428 as *const (),
                                                                    *__slate_slot_411 as u64,
                                                                )
                                                            };
                                                            unsafe {
                                                                memset(
                                                                    *__slate_slot_428 as *mut (),
                                                                    32 as i32,
                                                                    *__slate_slot_457 as u64,
                                                                )
                                                            };
                                                        } else {
                                                            std::ptr::write(
                                                                __slate_slot_458,
                                                                ((*__slate_slot_424 as i32)
                                                                    != (0 as i32))
                                                                    as i32,
                                                            );
                                                            unsafe {
                                                                memmove(
                                                                    (unsafe {
                                                                        unsafe {
                                                                            (*__slate_slot_428)
                                                                                .offset(
                                                                                *__slate_slot_457
                                                                                    as isize,
                                                                            )
                                                                        }
                                                                        .offset(
                                                                            *__slate_slot_458
                                                                                as isize,
                                                                        )
                                                                    })
                                                                        as *mut (),
                                                                    (unsafe {
                                                                        (*__slate_slot_428).offset(
                                                                            *__slate_slot_458
                                                                                as isize,
                                                                        )
                                                                    })
                                                                        as *const (),
                                                                    (*__slate_slot_411
                                                                        - (*__slate_slot_458
                                                                            as i64))
                                                                        as u64,
                                                                )
                                                            };
                                                            unsafe {
                                                                memset(
                                                                    (unsafe {
                                                                        (*__slate_slot_428).offset(
                                                                            *__slate_slot_458
                                                                                as isize,
                                                                        )
                                                                    })
                                                                        as *mut (),
                                                                    48 as i32,
                                                                    *__slate_slot_457 as u64,
                                                                )
                                                            };
                                                        }
                                                    }
                                                }
                                                *__slate_slot_411 = *__slate_slot_413;
                                            }
                                            if *__slate_slot_430 == std::ptr::null_mut::<i8>() {
                                                // /* The result is being rendered directory into pAccum.  This
                                                //           ** is the common and fast case */
                                                0 as i32;
                                                std::ptr::write(__slate_slot_862, pAccum);
                                                std::ptr::write(__slate_slot_863, unsafe {
                                                    (*(*__slate_slot_862)).nChar
                                                });
                                                std::ptr::write(
                                                    __slate_slot_864,
                                                    ((((*__slate_slot_863 as u64) as i64)
                                                        + *__slate_slot_411)
                                                        as i32)
                                                        as u32,
                                                );
                                                unsafe {
                                                    (*(*__slate_slot_862)).nChar =
                                                        *__slate_slot_864;
                                                }
                                                unsafe {
                                                    *unsafe {
                                                        (*__slate_slot_428)
                                                            .offset(*__slate_slot_411 as isize)
                                                    } = (0 as i32) as i8;
                                                }
                                                break '__join_2;
                                            } else {
                                                // /* We were unable to render directly into pAccum because we
                                                //           ** couldn't allocate sufficient memory.  We need to memcpy()
                                                //           ** the rendering (or some prefix thereof) into the output
                                                //           ** buffer. */
                                                unsafe {
                                                    *unsafe {
                                                        (*__slate_slot_409)
                                                            .offset((0 as i32) as isize)
                                                    } = (0 as i32) as i8;
                                                }
                                                *__slate_slot_409 = *__slate_slot_430;
                                                break '__join_11;
                                            }
                                        }
                                        *__slate_slot_421 = ((0 as i32) as i8) as u8;
                                        // /* no break */
                                        {}
                                    }
                                    if (((unsafe { (*(*__slate_slot_427)).flags }) as u32) as i32)
                                        & (1 as i32)
                                        != (0 as i32)
                                    {
                                        if *__slate_slot_423 != (0 as u8) {
                                            *__slate_slot_439 = getIntArg(*__slate_slot_435);
                                        } else {
                                            if *__slate_slot_419 != (0 as u8) {
                                                if ((*__slate_slot_419 as u32) as i32) == (2 as i32)
                                                {
                                                    *__slate_slot_439 =
                                                        unsafe { ap.next_arg::<i64>() };
                                                } else {
                                                    *__slate_slot_439 =
                                                        unsafe { ap.next_arg::<i64>() };
                                                }
                                            } else {
                                                *__slate_slot_439 =
                                                    (unsafe { ap.next_arg::<i32>() }) as i64;
                                            }
                                        }
                                        if *__slate_slot_439 < ((0 as i32) as i64) {
                                            {}
                                            {}
                                            *__slate_slot_425 = !(*__slate_slot_439) as u64;
                                            std::ptr::write(__slate_slot_775, *__slate_slot_425);
                                            std::ptr::write(
                                                __slate_slot_776,
                                                (*__slate_slot_775)
                                                    .wrapping_add(((1 as i32) as i64) as u64),
                                            );
                                            *__slate_slot_425 = *__slate_slot_776;
                                            *__slate_slot_424 = (45 as i32) as i8;
                                        } else {
                                            *__slate_slot_425 = *__slate_slot_439 as u64;
                                            *__slate_slot_424 = *__slate_slot_415 as i8;
                                        }
                                    } else {
                                        if *__slate_slot_423 != (0 as u8) {
                                            *__slate_slot_425 = getIntArg(*__slate_slot_435) as u64;
                                        } else {
                                            if *__slate_slot_419 != (0 as u8) {
                                                if ((*__slate_slot_419 as u32) as i32) == (2 as i32)
                                                {
                                                    *__slate_slot_425 =
                                                        unsafe { ap.next_arg::<u64>() };
                                                } else {
                                                    *__slate_slot_425 =
                                                        unsafe { ap.next_arg::<u64>() };
                                                }
                                            } else {
                                                *__slate_slot_425 =
                                                    (unsafe { ap.next_arg::<u32>() }) as u64;
                                            }
                                        }
                                        *__slate_slot_424 = (0 as i32) as i8;
                                    }
                                    if *__slate_slot_425 == (((0 as i32) as i64) as u64) {
                                        *__slate_slot_416 = ((0 as i32) as i8) as u8;
                                    }
                                    if *__slate_slot_418 != (0 as u8)
                                        && *__slate_slot_410
                                            < *__slate_slot_413
                                                - ((((*__slate_slot_424 as i32) != (0 as i32))
                                                    as i32)
                                                    as i64)
                                    {
                                        *__slate_slot_410 = *__slate_slot_413
                                            - ((((*__slate_slot_424 as i32) != (0 as i32)) as i32)
                                                as i64);
                                    }
                                    if *__slate_slot_410
                                        < (((70 as i32) - (10 as i32) - (70 as i32) / (3 as i32))
                                            as i64)
                                    {
                                        *__slate_slot_429 = 70 as i32;
                                        *__slate_slot_428 =
                                            (*__slate_slot_436).as_mut_ptr() as *mut i8;
                                    } else {
                                        *__slate_slot_440 = (*__slate_slot_410 as u64)
                                            .wrapping_add(((10 as i32) as i64) as u64);
                                        if *__slate_slot_421 != (0 as u8) {
                                            std::ptr::write(__slate_slot_777, *__slate_slot_440);
                                            std::ptr::write(
                                                __slate_slot_778,
                                                (*__slate_slot_777).wrapping_add(
                                                    (*__slate_slot_410 / ((3 as i32) as i64))
                                                        as u64,
                                                ),
                                            );
                                            *__slate_slot_440 = *__slate_slot_778;
                                        }
                                        std::ptr::write(
                                            __slate_slot_779,
                                            printfTempBuf(pAccum, *__slate_slot_440 as i64),
                                        );
                                        *__slate_slot_430 = *__slate_slot_779;
                                        *__slate_slot_428 = *__slate_slot_779;
                                        if *__slate_slot_428 == std::ptr::null_mut::<i8>() {
                                            return;
                                        } else {
                                            *__slate_slot_429 = (*__slate_slot_440 as u32) as i32;
                                        }
                                    }
                                    *__slate_slot_409 = unsafe {
                                        (*__slate_slot_428)
                                            .offset((*__slate_slot_429 - (1 as i32)) as isize)
                                    };
                                    if ((*__slate_slot_422 as u32) as i32) == (15 as i32) {
                                        std::ptr::write(
                                            __slate_slot_442,
                                            ((*__slate_slot_425 % (((10 as i32) as i64) as u64))
                                                as u32)
                                                as i32,
                                        );
                                        if *__slate_slot_442 >= (4 as i32)
                                            || *__slate_slot_425 / (((10 as i32) as i64) as u64)
                                                % (((10 as i32) as i64) as u64)
                                                == (((1 as i32) as i64) as u64)
                                        {
                                            *__slate_slot_442 = 0 as i32;
                                        }
                                        std::ptr::write(__slate_slot_780, *__slate_slot_409);
                                        std::ptr::write(__slate_slot_781, unsafe {
                                            (*__slate_slot_780).offset(-((1 as i32) as isize))
                                        });
                                        *__slate_slot_409 = *__slate_slot_781;
                                        unsafe {
                                            *(*__slate_slot_781) = unsafe {
                                                *unsafe {
                                                    unsafe { std::ptr::addr_of!(zOrd) as *const i8 }
                                                        .offset(
                                                            (*__slate_slot_442 * (2 as i32)
                                                                + (1 as i32))
                                                                as isize,
                                                        )
                                                }
                                            };
                                        }
                                        std::ptr::write(__slate_slot_782, *__slate_slot_409);
                                        std::ptr::write(__slate_slot_783, unsafe {
                                            (*__slate_slot_782).offset(-((1 as i32) as isize))
                                        });
                                        *__slate_slot_409 = *__slate_slot_783;
                                        unsafe {
                                            *(*__slate_slot_783) = unsafe {
                                                *unsafe {
                                                    unsafe { std::ptr::addr_of!(zOrd) as *const i8 }
                                                        .offset(
                                                            (*__slate_slot_442 * (2 as i32))
                                                                as isize,
                                                        )
                                                }
                                            };
                                        }
                                    }
                                    std::ptr::write(__slate_slot_443, unsafe {
                                        unsafe { std::ptr::addr_of!(aDigits.0) as *const i8 }
                                            .offset(
                                                (((unsafe { (*(*__slate_slot_427)).charset })
                                                    as u32)
                                                    as i32)
                                                    as isize,
                                            )
                                    });
                                    std::ptr::write(__slate_slot_444, unsafe {
                                        (*(*__slate_slot_427)).base
                                    });
                                    // /* Convert to ascii */
                                    loop {
                                        std::ptr::write(__slate_slot_784, *__slate_slot_409);
                                        std::ptr::write(__slate_slot_785, unsafe {
                                            (*__slate_slot_784).offset(-((1 as i32) as isize))
                                        });
                                        *__slate_slot_409 = *__slate_slot_785;
                                        unsafe {
                                            *(*__slate_slot_785) = unsafe {
                                                *unsafe {
                                                    (*__slate_slot_443).offset(
                                                        (*__slate_slot_425
                                                            % ((((*__slate_slot_444 as u32) as i32)
                                                                as i64)
                                                                as u64))
                                                            as isize,
                                                    )
                                                }
                                            };
                                        }
                                        *__slate_slot_425 = *__slate_slot_425
                                            / ((((*__slate_slot_444 as u32) as i32) as i64) as u64);
                                        if !(*__slate_slot_425 > (((0 as i32) as i64) as u64)) {
                                            break;
                                        }
                                    }
                                    *__slate_slot_411 = (unsafe {
                                        unsafe {
                                            (*__slate_slot_428)
                                                .offset((*__slate_slot_429 - (1 as i32)) as isize)
                                        }
                                        .offset_from(*__slate_slot_409 as *mut i8)
                                    })
                                        as i64;
                                    // /* zero pad */
                                    if *__slate_slot_410 > *__slate_slot_411 {
                                        std::ptr::write(
                                            __slate_slot_445,
                                            *__slate_slot_410 - *__slate_slot_411,
                                        );
                                        std::ptr::write(__slate_slot_786, *__slate_slot_409);
                                        std::ptr::write(__slate_slot_787, unsafe {
                                            (*__slate_slot_786)
                                                .offset(-(*__slate_slot_445 as isize))
                                        });
                                        *__slate_slot_409 = *__slate_slot_787;
                                        unsafe {
                                            memset(
                                                *__slate_slot_409 as *mut (),
                                                48 as i32,
                                                *__slate_slot_445 as u64,
                                            )
                                        };
                                        *__slate_slot_411 = *__slate_slot_410;
                                    }
                                    '__join_307: {
                                        if *__slate_slot_421 != (0 as u8) {
                                            // /* Number of "," to insert */
                                            std::ptr::write(
                                                __slate_slot_446,
                                                (*__slate_slot_411 - ((1 as i32) as i64))
                                                    / ((3 as i32) as i64),
                                            );
                                            std::ptr::write(
                                                __slate_slot_447,
                                                (*__slate_slot_411 - ((1 as i32) as i64))
                                                    % ((3 as i32) as i64)
                                                    + ((1 as i32) as i64),
                                            );
                                            std::ptr::write(__slate_slot_788, *__slate_slot_409);
                                            std::ptr::write(__slate_slot_789, unsafe {
                                                (*__slate_slot_788)
                                                    .offset(-(*__slate_slot_446 as isize))
                                            });
                                            *__slate_slot_409 = *__slate_slot_789;
                                            *__slate_slot_448 = 0 as i32;
                                            loop {
                                                if *__slate_slot_446 > ((0 as i32) as i64) {
                                                    unsafe {
                                                        *unsafe {
                                                            (*__slate_slot_409)
                                                                .offset(*__slate_slot_448 as isize)
                                                        } = unsafe {
                                                            *unsafe {
                                                                (*__slate_slot_409).offset(
                                                                    ((*__slate_slot_448 as i64)
                                                                        + *__slate_slot_446)
                                                                        as isize,
                                                                )
                                                            }
                                                        };
                                                    }
                                                    std::ptr::write(
                                                        __slate_slot_792,
                                                        *__slate_slot_447,
                                                    );
                                                    std::ptr::write(
                                                        __slate_slot_793,
                                                        *__slate_slot_792 - ((1 as i32) as i64),
                                                    );
                                                    *__slate_slot_447 = *__slate_slot_793;
                                                    if *__slate_slot_447 == ((0 as i32) as i64) {
                                                        std::ptr::write(
                                                            __slate_slot_794,
                                                            *__slate_slot_448,
                                                        );
                                                        std::ptr::write(
                                                            __slate_slot_795,
                                                            *__slate_slot_794 + (1 as i32),
                                                        );
                                                        *__slate_slot_448 = *__slate_slot_795;
                                                        unsafe {
                                                            *unsafe {
                                                                (*__slate_slot_409).offset(
                                                                    *__slate_slot_795 as isize,
                                                                )
                                                            } = *__slate_slot_421 as i8;
                                                        }
                                                        std::ptr::write(
                                                            __slate_slot_796,
                                                            *__slate_slot_446,
                                                        );
                                                        std::ptr::write(
                                                            __slate_slot_797,
                                                            *__slate_slot_796 - ((1 as i32) as i64),
                                                        );
                                                        *__slate_slot_446 = *__slate_slot_797;
                                                        *__slate_slot_447 = (3 as i32) as i64;
                                                    }
                                                    std::ptr::write(
                                                        __slate_slot_790,
                                                        *__slate_slot_448,
                                                    );
                                                    std::ptr::write(
                                                        __slate_slot_791,
                                                        *__slate_slot_790 + (1 as i32),
                                                    );
                                                    *__slate_slot_448 = *__slate_slot_791;
                                                } else {
                                                    break '__join_307;
                                                }
                                            }
                                        }
                                    }
                                    // /* Add sign */
                                    if *__slate_slot_424 != (0 as i8) {
                                        std::ptr::write(__slate_slot_798, *__slate_slot_409);
                                        std::ptr::write(__slate_slot_799, unsafe {
                                            (*__slate_slot_798).offset(-((1 as i32) as isize))
                                        });
                                        *__slate_slot_409 = *__slate_slot_799;
                                        unsafe {
                                            *(*__slate_slot_799) = *__slate_slot_424;
                                        }
                                    }
                                    '__join_301: {
                                        // /* Add "0" or "0x" */
                                        if *__slate_slot_416 != (0 as u8)
                                            && (unsafe { (*(*__slate_slot_427)).prefix })
                                                != (0 as u8)
                                        {
                                            *__slate_slot_449 = unsafe {
                                                unsafe { std::ptr::addr_of!(aPrefix) as *const i8 }
                                                    .offset(
                                                        (((unsafe { (*(*__slate_slot_427)).prefix })
                                                            as u32)
                                                            as i32)
                                                            as isize,
                                                    )
                                            };
                                            loop {
                                                std::ptr::write(__slate_slot_800, unsafe {
                                                    *(*__slate_slot_449)
                                                });
                                                *__slate_slot_450 = *__slate_slot_800;
                                                if (*__slate_slot_800 as i32) != (0 as i32) {
                                                    std::ptr::write(
                                                        __slate_slot_803,
                                                        *__slate_slot_409,
                                                    );
                                                    std::ptr::write(__slate_slot_804, unsafe {
                                                        (*__slate_slot_803)
                                                            .offset(-((1 as i32) as isize))
                                                    });
                                                    *__slate_slot_409 = *__slate_slot_804;
                                                    unsafe {
                                                        *(*__slate_slot_804) = *__slate_slot_450;
                                                    }
                                                    std::ptr::write(
                                                        __slate_slot_801,
                                                        *__slate_slot_449,
                                                    );
                                                    std::ptr::write(__slate_slot_802, unsafe {
                                                        (*__slate_slot_801)
                                                            .offset((1 as i32) as isize)
                                                    });
                                                    *__slate_slot_449 = *__slate_slot_802;
                                                } else {
                                                    break '__join_301;
                                                }
                                            }
                                        }
                                    }
                                    *__slate_slot_411 = (unsafe {
                                        unsafe {
                                            (*__slate_slot_428)
                                                .offset((*__slate_slot_429 - (1 as i32)) as isize)
                                        }
                                        .offset_from(*__slate_slot_409 as *mut i8)
                                    })
                                        as i64;
                                }
                                // /*
                                //     ** The text of the conversion is pointed to by "bufpt" and is
                                //     ** "length" characters long.  The field width is "width".  Do
                                //     ** the output.  Both length and width are in bytes, not characters,
                                //     ** at this point.  If the "!" flag was present on string conversions
                                //     ** indicating that width and precision should be expressed in characters,
                                //     ** then the values have been translated prior to reaching this point.
                                //     */
                                std::ptr::write(__slate_slot_969, *__slate_slot_413);
                                std::ptr::write(
                                    __slate_slot_970,
                                    *__slate_slot_969 - *__slate_slot_411,
                                );
                                *__slate_slot_413 = *__slate_slot_970;
                                if *__slate_slot_413 > ((0 as i32) as i64) {
                                    if !(*__slate_slot_414 != (0 as u8)) {
                                        sqlite3StrAppendchar64(
                                            pAccum,
                                            *__slate_slot_413,
                                            (32 as i32) as i8,
                                        );
                                    }
                                    sqlite3StrAppend64(
                                        pAccum,
                                        *__slate_slot_409 as *const i8,
                                        *__slate_slot_411,
                                    );
                                    if *__slate_slot_414 != (0 as u8) {
                                        sqlite3StrAppendchar64(
                                            pAccum,
                                            *__slate_slot_413,
                                            (32 as i32) as i8,
                                        );
                                    }
                                } else {
                                    sqlite3StrAppend64(
                                        pAccum,
                                        *__slate_slot_409 as *const i8,
                                        *__slate_slot_411,
                                    );
                                }
                                if *__slate_slot_430 != std::ptr::null_mut::<i8>() {
                                    unsafe {
                                        sqlite3DbFree(
                                            unsafe { (*pAccum).db },
                                            *__slate_slot_430 as *mut (),
                                        )
                                    };
                                    *__slate_slot_430 = std::ptr::null_mut::<i8>();
                                }
                                // /* End for loop over the format string */
                            }
                            std::ptr::write(__slate_slot_744, fmt);
                            std::ptr::write(__slate_slot_745, unsafe {
                                (*__slate_slot_744).offset((1 as i32) as isize)
                            });
                            fmt = *__slate_slot_745;
                        }
                    } else {
                        break '__join_0;
                    }
                }
                0 as i32;
                return;
            }
            sqlite3_str_append(pAccum, (b"%\0".as_ptr() as *mut i8) as *const i8, 1 as i32);
        }
        // /* End of function */
    }
    // /* %j: JSON string literal w/o "..." */
    // /* %J: Generate a JSON string literal */
    // /* %q: Escape ' characters */
    // /* %Q: Escape ' and enclose in '...' */
    // /* %w: Escape " characters */
    // /* End switch over the format type */
}

// /*
// ** Append N bytes of text from z to the StrAccum object.  Increase the
// ** size of the memory allocation for StrAccum if necessary.
// */
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.printf.sqlite3_str_append")]
extern "C-unwind" fn sqlite3_str_append(mut p: *mut sqlite3_str, mut z: *const i8, mut N: i32) {
    0 as i32;
    0 as i32;
    0 as i32;
    0 as i32;
    if unsafe { (*p).nChar }.wrapping_add(N as u32) >= unsafe { (*p).nAlloc } {
        enlargeAndAppend(p, z, N);
    } else {
        if N != (0 as i32) {
            0 as i32;
            let __v971: *mut sqlite3_str = p;
            let __v972: u32 = unsafe { (*__v971).nChar };
            let __v973: u32 = __v972.wrapping_add(N as u32);
            unsafe {
                (*__v971).nChar = __v973;
            }
            unsafe {
                memcpy(
                    (unsafe {
                        unsafe { (*p).zText }
                            .offset(unsafe { (*p).nChar }.wrapping_sub(N as u32) as isize)
                    }) as *mut (),
                    z as *const (),
                    (N as i64) as u64,
                )
            };
        }
    }
}

// /*
// ** Append the complete text of zero-terminated string z[] to the p string.
// */
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.printf.sqlite3_str_appendall")]
extern "C-unwind" fn sqlite3_str_appendall(mut p: *mut sqlite3_str, mut z: *const i8) {
    sqlite3_str_append(p, z, unsafe { sqlite3Strlen30(z) });
}

// /*
// ** Append N copies of character c to the given string buffer.
// */
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.printf.sqlite3_str_appendchar")]
extern "C-unwind" fn sqlite3_str_appendchar(mut p: *mut sqlite3_str, mut N: i32, mut c: i8) {
    {}
    let __v974: bool;
    if (((unsafe { (*p).nChar }) as u64) as i64) + (N as i64)
        >= (((unsafe { (*p).nAlloc }) as u64) as i64)
    {
        let __v975: i32 = sqlite3StrAccumEnlarge(p, N as i64);
        N = __v975;
        __v974 = __v975 <= (0 as i32);
    } else {
        __v974 = false as bool;
    }
    if __v974 {
        return;
    }
    '__slate_break_741: loop {
        let __v976: i32 = N;
        let __v977: i32 = __v976 - (1 as i32);
        N = __v977;
        if !(__v976 > (0 as i32)) {
            break;
        }
        let __v978: *mut sqlite3_str = p;
        let __v979: u32 = unsafe { (*__v978).nChar };
        let __v980: u32 = __v979.wrapping_add((1 as i32) as u32);
        unsafe {
            (*__v978).nChar = __v980;
        }
        unsafe {
            *unsafe { unsafe { (*p).zText }.offset(__v979 as isize) } = c;
        }
    }
}

// /*
// ** Reset an StrAccum string.  Reclaim all malloced memory.
// */
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.printf.sqlite3_str_reset")]
extern "C-unwind" fn sqlite3_str_reset(mut p: *mut sqlite3_str) {
    if (((unsafe { (*p).printfFlags }) as u32) as i32) & (4 as i32) != (0 as i32) {
        unsafe { sqlite3DbFree(unsafe { (*p).db }, (unsafe { (*p).zText }) as *mut ()) };
        let __v981: *mut sqlite3_str = p;
        let __v982: u8 = unsafe { (*__v981).printfFlags };
        let __v983: u8 = ((((__v982 as u32) as i32) & !(4 as i32)) as i8) as u8;
        unsafe {
            (*__v981).printfFlags = __v983;
        }
    } else {
        if p == ((unsafe { std::ptr::addr_of!(sqlite3OomStr) }) as *mut sqlite3_str) {
            return;
        }
    }
    unsafe {
        (*p).nAlloc = (0 as i32) as u32;
    }
    unsafe {
        (*p).nChar = (0 as i32) as u32;
    }
    unsafe {
        (*p).zText = std::ptr::null_mut::<i8>();
    }
}

// /* Truncate the text of the string to be no more than N bytes. */
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.printf.sqlite3_str_truncate")]
extern "C-unwind" fn sqlite3_str_truncate(mut p: *mut sqlite3_str, mut N: i32) {
    if p != std::ptr::null_mut::<sqlite3_str>()
        && N >= (0 as i32)
        && (N as u32) < unsafe { (*p).nChar }
    {
        unsafe {
            (*p).nChar = N as u32;
        }
        unsafe {
            *unsafe { unsafe { (*p).zText }.offset((unsafe { (*p).nChar }) as isize) } =
                (0 as i32) as i8;
        }
    }
}

// /* Return any error code associated with p */
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.printf.sqlite3_str_errcode")]
extern "C-unwind" fn sqlite3_str_errcode(mut p: *mut sqlite3_str) -> i32 {
    return if p != std::ptr::null_mut::<sqlite3_str>() {
        ((unsafe { (*p).accError }) as u32) as i32
    } else {
        7 as i32
    };
}

// /* Return the current length of p in bytes */
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.printf.sqlite3_str_length")]
extern "C-unwind" fn sqlite3_str_length(mut p: *mut sqlite3_str) -> i32 {
    return (if p != std::ptr::null_mut::<sqlite3_str>() {
        unsafe { (*p).nChar }
    } else {
        (0 as i32) as u32
    }) as i32;
}

// /* Return the current value for p */
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.printf.sqlite3_str_value")]
extern "C-unwind" fn sqlite3_str_value(mut p: *mut sqlite3_str) -> *mut i8 {
    if p == std::ptr::null_mut::<sqlite3_str>() || (unsafe { (*p).nChar }) == ((0 as i32) as u32) {
        return std::ptr::null_mut::<i8>();
    }
    unsafe {
        *unsafe { unsafe { (*p).zText }.offset((unsafe { (*p).nChar }) as isize) } =
            (0 as i32) as i8;
    }
    return unsafe { (*p).zText };
}

// /*
// ** Format and write a message to the log if logging is enabled.
// */
#[unsafe(no_mangle)]
unsafe extern "C-unwind" fn sqlite3_log(
    mut iErrCode: i32,
    mut zFormat: *const i8,
    mut __va_args: ...
) {
    // /* Vararg list */
    let mut ap: core::ffi::VaList<'_> = unsafe { std::mem::zeroed() };
    if (unsafe { sqlite3Config.xLog }) != None {
        ap = __va_args.clone();
        renderLogMsg(iErrCode, zFormat, ap.clone());
        {}
    }
}

// /*
// ** Print into memory obtained from sqliteMalloc().  Use the internal
// ** %-conversion extensions.
// */
#[unsafe(no_mangle)]
unsafe extern "C-unwind" fn sqlite3MPrintf(
    mut db: *mut sqlite3,
    mut zFormat: *const i8,
    mut __va_args: ...
) -> *mut i8 {
    let mut ap: core::ffi::VaList<'_> = unsafe { std::mem::zeroed() };
    let mut z: *mut i8 = unsafe { std::mem::zeroed() };
    ap = __va_args.clone();
    z = sqlite3VMPrintf(db, zFormat, ap.clone());
    {}
    return z;
}

// /*
// ** Print into memory obtained from sqliteMalloc().  Use the internal
// ** %-conversion extensions.
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3VMPrintf(
    mut db: *mut sqlite3,
    mut zFormat: *const i8,
    mut ap: core::ffi::VaList<'_>,
) -> *mut i8 {
    let mut z: *mut i8 = unsafe { std::mem::zeroed() };
    let mut zBase: __SlateAlign16<[i8; 70]> = __SlateAlign16([0 as i8; 70]);
    let mut acc: sqlite3_str = unsafe { std::mem::zeroed() };
    0 as i32;
    sqlite3StrAccumInit(
        std::ptr::addr_of_mut!(acc),
        db,
        zBase.0.as_mut_ptr() as *mut i8,
        ((70 as u64) as u32) as i32,
        unsafe {
            *unsafe { unsafe { (*db).aLimit.as_mut_ptr() as *mut i32 }.offset((0 as i32) as isize) }
        },
    );
    acc.printfFlags = ((1 as i32) as i8) as u8;
    sqlite3_str_vappendf(std::ptr::addr_of_mut!(acc), zFormat, ap.clone());
    z = sqlite3StrAccumFinish(std::ptr::addr_of_mut!(acc));
    if ((acc.accError as u32) as i32) == (7 as i32) {
        unsafe { sqlite3OomFault(db) };
    }
    return z;
}

// /*****************************************************************************
// ** Reference counted string/blob storage
// *****************************************************************************/
// /*
// ** Increase the reference count of the string by one.
// **
// ** The input parameter is returned.
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3RCStrRef(mut z: *mut i8) -> *mut i8 {
    let mut p: *mut RCStr = z as *mut RCStr;
    0 as i32;
    let __v984: *mut RCStr = p;
    let __v985: *mut RCStr = unsafe { __v984.offset(-((1 as i32) as isize)) };
    p = __v985;
    let __v986: *mut RCStr = p;
    let __v987: u64 = unsafe { (*__v986).nRCRef };
    let __v988: u64 = __v987.wrapping_add(((1 as i32) as i64) as u64);
    unsafe {
        (*__v986).nRCRef = __v988;
    }
    return z;
}

// /*
// ** Decrease the reference count by one.  Free the string when the
// ** reference count reaches zero.
// */
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.printf.sqlite3RCStrUnref")]
extern "C-unwind" fn sqlite3RCStrUnref(mut z: *mut ()) {
    let mut p: *mut RCStr = z as *mut RCStr;
    0 as i32;
    let __v989: *mut RCStr = p;
    let __v990: *mut RCStr = unsafe { __v989.offset(-((1 as i32) as isize)) };
    p = __v990;
    0 as i32;
    if (unsafe { (*p).nRCRef }) >= (((2 as i32) as i64) as u64) {
        let __v991: *mut RCStr = p;
        let __v992: u64 = unsafe { (*__v991).nRCRef };
        let __v993: u64 = __v992.wrapping_sub(((1 as i32) as i64) as u64);
        unsafe {
            (*__v991).nRCRef = __v993;
        }
    } else {
        unsafe { sqlite3_free(p as *mut ()) };
    }
}

// /*
// ** Create a new string that is capable of holding N bytes of text, not counting
// ** the zero byte at the end.  The string is uninitialized.
// **
// ** The reference count is initially 1.  Call sqlite3RCStrUnref() to free the
// ** newly allocated string.
// **
// ** This routine returns 0 on an OOM.
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3RCStrNew(mut N: u64) -> *mut i8 {
    let mut p: *mut RCStr = (unsafe {
        sqlite3_malloc64(
            N.wrapping_add(8 as u64)
                .wrapping_add(((1 as i32) as i64) as u64),
        )
    }) as *mut RCStr;
    if p == std::ptr::null_mut::<RCStr>() {
        return std::ptr::null_mut::<i8>();
    }
    unsafe {
        (*p).nRCRef = ((1 as i32) as i64) as u64;
    }
    return (unsafe { p.offset((1 as i32) as isize) }) as *mut i8;
}

// /*
// ** Change the size of the string so that it is able to hold N bytes.
// ** The string might be reallocated, so return the new allocation.
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3RCStrResize(mut z: *mut i8, mut N: u64) -> *mut i8 {
    let mut p: *mut RCStr = z as *mut RCStr;
    let mut pNew: *mut RCStr = unsafe { std::mem::zeroed() };
    0 as i32;
    let __v994: *mut RCStr = p;
    let __v995: *mut RCStr = unsafe { __v994.offset(-((1 as i32) as isize)) };
    p = __v995;
    0 as i32;
    pNew = (unsafe {
        sqlite3_realloc64(
            p as *mut (),
            N.wrapping_add(8 as u64)
                .wrapping_add(((1 as i32) as i64) as u64),
        )
    }) as *mut RCStr;
    if pNew == std::ptr::null_mut::<RCStr>() {
        unsafe { sqlite3_free(p as *mut ()) };
        return std::ptr::null_mut::<i8>();
    } else {
        return (unsafe { pNew.offset((1 as i32) as isize) }) as *mut i8;
    }
    return unsafe { std::mem::zeroed() };
}

// /*
// ** Initialize a string accumulator.
// **
// ** p:     The accumulator to be initialized.
// ** db:    Pointer to a database connection.  May be NULL.  Lookaside
// **        memory is used if not NULL. db->mallocFailed is set appropriately
// **        when not NULL.
// ** zBase: An initial buffer.  May be NULL in which case the initial buffer
// **        is malloced.
// ** n:     Size of zBase in bytes.  If total space requirements never exceed
// **        n then no memory allocations ever occur.
// ** mx:    Maximum number of bytes to accumulate.  If mx==0 then no memory
// **        allocations will ever occur.
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3StrAccumInit(
    mut p: *mut sqlite3_str,
    mut db: *mut sqlite3,
    mut zBase: *mut i8,
    mut n: i32,
    mut mx: i32,
) {
    unsafe {
        (*p).zText = zBase;
    }
    unsafe {
        (*p).db = db;
    }
    unsafe {
        (*p).nAlloc = n as u32;
    }
    unsafe {
        (*p).mxAlloc = mx as u32;
    }
    unsafe {
        (*p).nChar = (0 as i32) as u32;
    }
    unsafe {
        (*p).accError = ((0 as i32) as i8) as u8;
    }
    unsafe {
        (*p).printfFlags = ((0 as i32) as i8) as u8;
    }
}

// /*
// ** Enlarge the memory allocation on a StrAccum object so that it is
// ** able to accept at least N more bytes of text.
// **
// ** Return the number of bytes of text that StrAccum is able to accept
// ** after the attempted enlargement.  The value returned might be zero.
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3StrAccumEnlarge(mut p: *mut sqlite3_str, mut N: i64) -> i32 {
    let mut zNew: *mut i8 = unsafe { std::mem::zeroed() };
    // /* Only called if really needed */
    0 as i32;
    if (unsafe { (*p).accError }) != (0 as u8) {
        {}
        {}
        return 0 as i32;
    }
    if (unsafe { (*p).mxAlloc }) == ((0 as i32) as u32) {
        sqlite3StrAccumSetError(p, ((18 as i32) as i8) as u8);
        return unsafe { (*p).nAlloc }
            .wrapping_sub(unsafe { (*p).nChar })
            .wrapping_sub((1 as i32) as u32) as i32;
    } else {
        let mut zOld: *mut i8 =
            if (((unsafe { (*p).printfFlags }) as u32) as i32) & (4 as i32) != (0 as i32) {
                unsafe { (*p).zText }
            } else {
                std::ptr::null_mut::<i8>()
            };
        let mut szNew: i64 = (((unsafe { (*p).nChar }) as u64) as i64) + N + ((1 as i32) as i64);
        if szNew + (((unsafe { (*p).nChar }) as u64) as i64)
            <= (((unsafe { (*p).mxAlloc }) as u64) as i64)
        {
            // /* Force exponential buffer size growth as long as it does not overflow,
            //       ** to avoid having to call this routine too often */
            let __v996: i64 = szNew;
            let __v997: i64 = __v996 + (((unsafe { (*p).nChar }) as u64) as i64);
            szNew = __v997;
        }
        if szNew > (((unsafe { (*p).mxAlloc }) as u64) as i64) {
            sqlite3_str_reset(p);
            sqlite3StrAccumSetError(p, ((18 as i32) as i8) as u8);
            return 0 as i32;
        } else {
            unsafe {
                (*p).nAlloc = (szNew as i32) as u32;
            }
        }
        if (unsafe { (*p).db }) != std::ptr::null_mut::<sqlite3>() {
            zNew = (unsafe {
                sqlite3DbRealloc(
                    unsafe { (*p).db },
                    zOld as *mut (),
                    (unsafe { (*p).nAlloc }) as u64,
                )
            }) as *mut i8;
        } else {
            zNew = (unsafe { sqlite3Realloc(zOld as *mut (), (unsafe { (*p).nAlloc }) as u64) })
                as *mut i8;
        }
        if zNew != std::ptr::null_mut::<i8>() {
            0 as i32;
            if !((((unsafe { (*p).printfFlags }) as u32) as i32) & (4 as i32) != (0 as i32))
                && (unsafe { (*p).nChar }) > ((0 as i32) as u32)
            {
                unsafe {
                    memcpy(
                        zNew as *mut (),
                        (unsafe { (*p).zText }) as *const (),
                        (unsafe { (*p).nChar }) as u64,
                    )
                };
            }
            unsafe {
                (*p).zText = zNew;
            }
            unsafe {
                (*p).nAlloc =
                    (unsafe { sqlite3DbMallocSize(unsafe { (*p).db }, zNew as *const ()) }) as u32;
            }
            let __v998: *mut sqlite3_str = p;
            let __v999: u8 = unsafe { (*__v998).printfFlags };
            let __v1000: u8 = ((((__v999 as u32) as i32) | (4 as i32)) as i8) as u8;
            unsafe {
                (*__v998).printfFlags = __v1000;
            }
        } else {
            sqlite3_str_reset(p);
            sqlite3StrAccumSetError(p, ((7 as i32) as i8) as u8);
            return 0 as i32;
        }
    }
    0 as i32;
    return N as i32;
}

#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3StrAccumEnlargeIfNeeded(mut p: *mut sqlite3_str, mut N: i64) -> i32 {
    if N + (((unsafe { (*p).nChar }) as u64) as i64) >= (((unsafe { (*p).nAlloc }) as u64) as i64) {
        sqlite3StrAccumEnlarge(p, N);
    }
    return ((unsafe { (*p).accError }) as u32) as i32;
}

#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3StrAccumFinish(mut p: *mut sqlite3_str) -> *mut i8 {
    if (unsafe { (*p).zText }) != std::ptr::null_mut::<i8>() {
        unsafe {
            *unsafe { unsafe { (*p).zText }.offset((unsafe { (*p).nChar }) as isize) } =
                (0 as i32) as i8;
        }
        if (unsafe { (*p).mxAlloc }) > ((0 as i32) as u32)
            && !((((unsafe { (*p).printfFlags }) as u32) as i32) & (4 as i32) != (0 as i32))
        {
            return strAccumFinishRealloc(p);
        }
    }
    return unsafe { (*p).zText };
}

// /*  0 */
// /*  1 */
// /*  2 */
// /*  3 */
// /*  4 */
// /* Hash: 6 */
// /*  5 */
// /*  6 */
// /*  7 */
// /* Hash: 12 */
// /*  8 */
// /*  9 */
// /* 10 */
// /* 11 */
// /* 12 */
// /* 13 */
// /* 14 */
// /* 15 */
// /* 16 */
// /* Hash: 13 */
// /* 17 */
// /* 18 */
// /* Hash: 19 */
// /* 19 */
// /* 20 */
// /* 21 */
// /* 22 */
// /* 23 */
// /* Hash: 24 */
// /* 24 */
// /* Additional Notes:
// **
// **    %S    Takes a pointer to SrcItem.  Shows name or database.name
// **    %!S   Like %S but prefer the zName over the zAlias
// */
// /*
// ** Set the StrAccum object to an error mode.
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3StrAccumSetError(mut p: *mut sqlite3_str, mut eError: u8) {
    0 as i32;
    unsafe {
        (*p).accError = eError;
    }
    if (unsafe { (*p).mxAlloc }) != (0 as u32) {
        sqlite3_str_reset(p);
    }
    if ((eError as u32) as i32) == (18 as i32) {
        unsafe { sqlite3ErrorToParser(unsafe { (*p).db }, (eError as u32) as i32) };
    }
}

// /*
// ** The z string points to the first character of a token that is
// ** associated with an error.  If db does not already have an error
// ** byte offset recorded, try to compute the error byte offset for
// ** z and set the error byte offset in db.
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3RecordErrorByteOffset(mut db: *mut sqlite3, mut z: *const i8) {
    let mut pParse: *const Parse = unsafe { std::mem::zeroed() };
    let mut zText: *const i8 = unsafe { std::mem::zeroed() };
    let mut zEnd: *const i8 = unsafe { std::mem::zeroed() };
    0 as i32;
    if db == std::ptr::null_mut::<sqlite3>() {
        return;
    }
    if (unsafe { (*db).errByteOffset }) != -(2 as i32) {
        return;
    }
    pParse = (unsafe { (*db).pParse }) as *const Parse;
    if pParse == std::ptr::null::<Parse>() {
        return;
    }
    zText = unsafe { (*pParse).zTail };
    if zText == std::ptr::null::<i8>() {
        return;
    }
    zEnd = unsafe { zText.offset((unsafe { strlen(zText) }) as isize) };
    if (z as u64) >= (zText as u64) && (z as u64) < (zEnd as u64) {
        unsafe {
            (*db).errByteOffset = ((unsafe { z.offset_from(zText as *const i8) }) as i64) as i32;
        }
    }
}

// /*
// ** If pExpr has a byte offset for the start of a token, record that as
// ** as the error offset.
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3RecordErrorOffsetOfExpr(mut db: *mut sqlite3, mut pExpr: *const Expr) {
    '__slate_break_740: while pExpr != std::ptr::null::<Expr>()
        && ((unsafe { (*pExpr).flags }) & (((1 as i32) | (2 as i32)) as u32) != ((0 as i32) as u32)
            || (unsafe { (*pExpr).w.iOfst }) <= (0 as i32))
    {
        pExpr = (unsafe { (*pExpr).pLeft }) as *const Expr;
    }
    if pExpr == std::ptr::null::<Expr>() {
        return;
    }
    if (unsafe { (*pExpr).flags }) & ((1073741824 as i32) as u32) != ((0 as i32) as u32) {
        return;
    }
    unsafe {
        (*db).errByteOffset = unsafe { (*pExpr).w.iOfst };
    }
}

// /*
// ** Extra argument values from a PrintfArguments object
// */
fn getIntArg(mut p: *mut PrintfArguments) -> i64 {
    if (unsafe { (*p).nArg }) <= unsafe { (*p).nUsed } {
        return (0 as i32) as i64;
    }
    let __v1001: *mut PrintfArguments = p;
    let __v1002: i32 = unsafe { (*__v1001).nUsed };
    let __v1003: i32 = __v1002 + (1 as i32);
    unsafe {
        (*__v1001).nUsed = __v1003;
    }
    return unsafe {
        sqlite3_value_int64(unsafe { *unsafe { unsafe { (*p).apArg }.offset(__v1002 as isize) } })
    };
}

fn getDoubleArg(mut p: *mut PrintfArguments) -> f64 {
    if (unsafe { (*p).nArg }) <= unsafe { (*p).nUsed } {
        return 0.0f64;
    }
    let __v1004: *mut PrintfArguments = p;
    let __v1005: i32 = unsafe { (*__v1004).nUsed };
    let __v1006: i32 = __v1005 + (1 as i32);
    unsafe {
        (*__v1004).nUsed = __v1006;
    }
    return unsafe {
        sqlite3_value_double(unsafe { *unsafe { unsafe { (*p).apArg }.offset(__v1005 as isize) } })
    };
}

fn getTextArg(mut p: *mut PrintfArguments) -> *mut i8 {
    if (unsafe { (*p).nArg }) <= unsafe { (*p).nUsed } {
        return std::ptr::null_mut::<i8>();
    }
    let __v1007: *mut PrintfArguments = p;
    let __v1008: i32 = unsafe { (*__v1007).nUsed };
    let __v1009: i32 = __v1008 + (1 as i32);
    unsafe {
        (*__v1007).nUsed = __v1009;
    }
    return (unsafe {
        sqlite3_value_text(unsafe { *unsafe { unsafe { (*p).apArg }.offset(__v1008 as isize) } })
    }) as *mut i8;
}

// /*
// ** Allocate memory for a temporary buffer needed for printf rendering.
// **
// ** If the requested size of the temp buffer is larger than the size
// ** of the output buffer in pAccum, then cause an SQLITE_TOOBIG error.
// ** Do the size check before the memory allocation to prevent rogue
// ** SQL from requesting large allocations using the precision or width
// ** field of the printf() function.
// */
fn printfTempBuf(mut pAccum: *mut sqlite3_str, mut n: i64) -> *mut i8 {
    let mut z: *mut i8 = unsafe { std::mem::zeroed() };
    if (unsafe { (*pAccum).accError }) != (0 as u8) {
        return std::ptr::null_mut::<i8>();
    }
    if n > (((unsafe { (*pAccum).nAlloc }) as u64) as i64)
        && n > (((unsafe { (*pAccum).mxAlloc }) as u64) as i64)
    {
        sqlite3StrAccumSetError(pAccum, ((18 as i32) as i8) as u8);
        return std::ptr::null_mut::<i8>();
    }
    z = (unsafe { sqlite3_malloc(n as i32) }) as *mut i8;
    if z == std::ptr::null_mut::<i8>() {
        sqlite3StrAccumSetError(pAccum, ((7 as i32) as i8) as u8);
    }
    return z;
}

fn sqlite3StrAppend64(mut p: *mut sqlite3_str, mut z: *const i8, mut N: i64) {
    0 as i32;
    0 as i32;
    0 as i32;
    0 as i32;
    if (((unsafe { (*p).nChar }) as u64) as i64) + N >= (((unsafe { (*p).nAlloc }) as u64) as i64) {
        enlargeAndAppend(p, z, N as i32);
    } else {
        if N != (0 as i64) {
            0 as i32;
            let __v1010: *mut sqlite3_str = p;
            let __v1011: u32 = unsafe { (*__v1010).nChar };
            let __v1012: u32 = ((((__v1011 as u64) as i64) + N) as i32) as u32;
            unsafe {
                (*__v1010).nChar = __v1012;
            }
            unsafe {
                memcpy(
                    (unsafe {
                        unsafe { (*p).zText }
                            .offset(((((unsafe { (*p).nChar }) as u64) as i64) - N) as isize)
                    }) as *mut (),
                    z as *const (),
                    N as u64,
                )
            };
        }
    }
}

fn sqlite3StrAppendchar64(mut p: *mut sqlite3_str, mut N: i64, mut c: i8) {
    {}
    let __v1013: bool;
    if (((unsafe { (*p).nChar }) as u64) as i64) + N >= (((unsafe { (*p).nAlloc }) as u64) as i64) {
        let __v1014: i64 = sqlite3StrAccumEnlarge(p, N) as i64;
        N = __v1014;
        __v1013 = __v1014 <= ((0 as i32) as i64);
    } else {
        __v1013 = false as bool;
    }
    if __v1013 {
        return;
    }
    '__slate_break_742: loop {
        let __v1015: i64 = N;
        let __v1016: i64 = __v1015 - ((1 as i32) as i64);
        N = __v1016;
        if !(__v1015 > ((0 as i32) as i64)) {
            break;
        }
        let __v1017: *mut sqlite3_str = p;
        let __v1018: u32 = unsafe { (*__v1017).nChar };
        let __v1019: u32 = __v1018.wrapping_add((1 as i32) as u32);
        unsafe {
            (*__v1017).nChar = __v1019;
        }
        unsafe {
            *unsafe { unsafe { (*p).zText }.offset(__v1018 as isize) } = c;
        }
    }
}

// /*
// ** The StrAccum "p" is not large enough to accept N new bytes of z[].
// ** So enlarge if first, then do the append.
// **
// ** This is a helper routine to sqlite3_str_append() that does special-case
// ** work (enlarging the buffer) using tail recursion, so that the
// ** sqlite3_str_append() routine can use fast calling semantics.
// */
fn enlargeAndAppend(mut p: *mut sqlite3_str, mut z: *const i8, mut N: i32) {
    N = sqlite3StrAccumEnlarge(p, N as i64);
    0 as i32;
    if N > (0 as i32) {
        unsafe {
            memcpy(
                (unsafe { unsafe { (*p).zText }.offset((unsafe { (*p).nChar }) as isize) })
                    as *mut (),
                z as *const (),
                (N as i64) as u64,
            )
        };
        let __v1020: *mut sqlite3_str = p;
        let __v1021: u32 = unsafe { (*__v1020).nChar };
        let __v1022: u32 = __v1021.wrapping_add(N as u32);
        unsafe {
            (*__v1020).nChar = __v1022;
        }
    }
}

// /*
// ** Finish off a string by making sure it is zero-terminated.
// ** Return a pointer to the resulting string.  Return a NULL
// ** pointer if any kind of error was encountered.
// */
fn strAccumFinishRealloc(mut p: *mut sqlite3_str) -> *mut i8 {
    let mut zText: *mut i8 = unsafe { std::mem::zeroed() };
    0 as i32;
    zText = (unsafe {
        sqlite3DbMallocRaw(
            unsafe { (*p).db },
            (((1 as i32) as i64) as u64).wrapping_add((unsafe { (*p).nChar }) as u64),
        )
    }) as *mut i8;
    if zText != std::ptr::null_mut::<i8>() {
        unsafe {
            memcpy(
                zText as *mut (),
                (unsafe { (*p).zText }) as *const (),
                unsafe { (*p).nChar }.wrapping_add((1 as i32) as u32) as u64,
            )
        };
        let __v1023: *mut sqlite3_str = p;
        let __v1024: u8 = unsafe { (*__v1023).printfFlags };
        let __v1025: u8 = ((((__v1024 as u32) as i32) | (4 as i32)) as i8) as u8;
        unsafe {
            (*__v1023).printfFlags = __v1025;
        }
    } else {
        sqlite3StrAccumSetError(p, ((7 as i32) as i8) as u8);
    }
    unsafe {
        (*p).zText = zText;
    }
    return zText;
}

// /* Maximum size of an sqlite3_log() message. */
// /*
// ** This is the routine that actually formats the sqlite3_log() message.
// ** We house it in a separate routine from sqlite3_log() to avoid using
// ** stack space on small-stack systems when logging is disabled.
// **
// ** sqlite3_log() must render into a static buffer.  It cannot dynamically
// ** allocate memory because it might be called while the memory allocator
// ** mutex is held.
// **
// ** sqlite3_str_vappendf() might ask for *temporary* memory allocations for
// ** certain format characters (%q) or for very large precisions or widths.
// ** Care must be taken that any sqlite3_log() calls that occur while the
// ** memory mutex is held do not use these mechanisms.
// */
fn renderLogMsg(mut iErrCode: i32, mut zFormat: *const i8, mut ap: core::ffi::VaList<'_>) {
    // /* String accumulator */
    let mut acc: sqlite3_str = unsafe { std::mem::zeroed() };
    // /* Complete log message */
    let mut zMsg: __SlateAlign16<[i8; 700]> = __SlateAlign16([0 as i8; 700]);
    sqlite3StrAccumInit(
        std::ptr::addr_of_mut!(acc),
        std::ptr::null_mut::<sqlite3>(),
        zMsg.0.as_mut_ptr() as *mut i8,
        ((700 as u64) as u32) as i32,
        0 as i32,
    );
    sqlite3_str_vappendf(std::ptr::addr_of_mut!(acc), zFormat, ap.clone());
    unsafe {
        unsafe { sqlite3Config.xLog }.unwrap()(
            unsafe { sqlite3Config.pLogArg },
            iErrCode,
            sqlite3StrAccumFinish(std::ptr::addr_of_mut!(acc)) as *const i8,
        )
    };
}
