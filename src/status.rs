unsafe extern "C" {
    static mut sqlite3Config: Sqlite3Config;
    fn sqlite3_msize(__v409: *mut ()) -> u64;
    fn sqlite3_mutex_enter(__v410: *mut sqlite3_mutex);
    fn sqlite3_mutex_leave(__v411: *mut sqlite3_mutex);
    fn sqlite3PagerMemUsed(__v430: *mut Pager) -> i32;
    fn sqlite3PagerCacheStat(__v431: *mut Pager, __v432: i32, __v433: i32, __v434: *mut u64);
    fn sqlite3BtreeGetPageSize(__v435: *mut Btree) -> i32;
    fn sqlite3BtreePager(__v436: *mut Btree) -> *mut Pager;
    fn sqlite3BtreeEnterAll(__v437: *mut sqlite3);
    fn sqlite3BtreeConnectionCount(__v438: *mut Btree) -> i32;
    fn sqlite3BtreeLeaveAll(__v439: *mut sqlite3);
    fn sqlite3VdbeDelete(__v440: *mut Vdbe);
    fn sqlite3MisuseError(__v441: i32) -> i32;
    fn sqlite3Pcache1Mutex() -> *mut sqlite3_mutex;
    fn sqlite3MallocMutex() -> *mut sqlite3_mutex;
    fn sqlite3DeleteTable(__v451: *mut sqlite3, __v452: *mut Table);
    fn sqlite3DeleteTrigger(__v453: *mut sqlite3, __v454: *mut Trigger);
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
    trace: __SlateRecord170,
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
    u1: __SlateRecord171,
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
    __slate_bits_0: __slate_bits::__SlateBits73U0,
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
    u: __SlateRecord181,
    pLeft: *mut Expr,
    pRight: *mut Expr,
    x: __SlateRecord182,
    nHeight: i32,
    iTable: i32,
    iColumn: i16,
    iAgg: i16,
    w: __SlateRecord183,
    pAggInfo: *mut AggInfo,
    y: __SlateRecord184,
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
    u: __SlateRecord172,
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
    __slate_bits_0: __slate_bits::__SlateBits97U0,
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
    u1: __SlateRecord197,
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
    fg: __SlateRecord191,
    iCursor: i32,
    colUsed: u64,
    u1: __SlateRecord192,
    u2: __SlateRecord193,
    u3: __SlateRecord194,
    u4: __SlateRecord195,
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
    u: __SlateRecord173,
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
struct Pager {}

#[repr(C)]
#[derive(Clone, Copy)]
struct Btree {}

#[repr(C)]
#[derive(Clone, Copy)]
struct BtCursor {}

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
    __slate_bits_0: __slate_bits::__SlateBits158U0,
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
    __slate_bits_0: __slate_bits::__SlateBits169U0,
    azInit: *mut *const i8,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord170 {
    xLegacy: Option<unsafe extern "C-unwind" fn(*mut (), *const i8)>,
    xV2: Option<unsafe extern "C-unwind" fn(u32, *mut (), *mut (), *mut ()) -> i32>,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord171 {
    isInterrupted: i32,
    notUsed1: f64,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord172 {
    pHash: *mut FuncDef,
    pDestructor: *mut FuncDestructor,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord173 {
    tab: __SlateRecord174,
    view: __SlateRecord175,
    vtab: __SlateRecord176,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord174 {
    addColOffset: i32,
    pFKey: *mut FKey,
    pDfltList: *mut ExprList,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord175 {
    pSelect: *mut Select,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord176 {
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
union __SlateRecord181 {
    zToken: *mut i8,
    iValue: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord182 {
    pList: *mut ExprList,
    pSelect: *mut Select,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord183 {
    iJoin: i32,
    iOfst: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord184 {
    pTab: *mut Table,
    pWin: *mut Window,
    nReg: i32,
    sub: __SlateRecord185,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord185 {
    iAddr: i32,
    regReturn: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct ExprList_item {
    pExpr: *mut Expr,
    zEName: *mut i8,
    fg: __SlateRecord187,
    u: __SlateRecord188,
}

#[repr(C, align(4))]
#[derive(Clone, Copy)]
struct __SlateRecord187 {
    sortFlags: u8,
    __slate_bits_0: __slate_bits::__SlateBits187U0,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord188 {
    x: __SlateRecord189,
    iConstExprReg: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord189 {
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
struct __SlateRecord191 {
    jointype: u8,
    __slate_bits_0: __slate_bits::__SlateBits191U0,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord192 {
    zIndexedBy: *mut i8,
    pFuncArg: *mut ExprList,
    nRow: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord193 {
    pIBIndex: *mut Index,
    pCteUse: *mut CteUse,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord194 {
    pOn: *mut Expr,
    pUsing: *mut IdList,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord195 {
    pSchema: *mut Schema,
    zDatabase: *mut i8,
    pSubq: *mut Subquery,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord197 {
    cr: __SlateRecord198,
    d: __SlateRecord199,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord198 {
    addrCrTab: i32,
    regRowid: i32,
    regRoot: i32,
    constraintName: Token,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord199 {
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

#[repr(C)]
#[derive(Clone, Copy)]
struct VdbeSorter {}

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
struct VdbeTxtBlbCache {
    pCValue: *mut i8,
    iOffset: i64,
    iCol: i32,
    cacheStatus: u32,
    colCacheCtr: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct VdbeCursor {
    eCurType: u8,
    iDb: i8,
    nullRow: u8,
    deferredMoveto: u8,
    isTable: u8,
    __slate_bits_0: __slate_bits::__SlateBits209U0,
    seekHit: u16,
    ub: __SlateRecord211,
    seqCount: i64,
    cacheStatus: u32,
    seekResult: i32,
    pAltCursor: *mut VdbeCursor,
    uc: __SlateRecord212,
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
union __SlateRecord211 {
    pBtx: *mut Btree,
    aAltMap: *mut u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord212 {
    pCursor: *mut BtCursor,
    pVCur: *mut sqlite3_vtab_cursor,
    pSorter: *mut VdbeSorter,
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
union MemValue {
    r: f64,
    i: i64,
    nZero: i32,
    zPType: *const i8,
    pDef: *mut FuncDef,
}

// /*
// ** 2008 June 18
// **
// ** The author disclaims copyright to this source code.  In place of
// ** a legal notice, here is a blessing:
// **
// **    May you do good and not evil.
// **    May you find forgiveness for yourself and forgive others.
// **    May you share freely, never taking more than you give.
// **
// *************************************************************************
// **
// ** This module implements the sqlite3_status() interface and related
// ** functionality.
// */
// /*
// ** Variables in which to record status information.
// */
#[repr(C)]
#[derive(Clone, Copy)]
struct sqlite3StatType {
    nowValue: [i64; 10],
    // /* Current value */
    mxValue: [i64; 10],
}

mod __slate_bits {
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
    #[bitfields::bitfield([u8; 3], c_names = true, new = false, from_into_bits = false, from_traits = false, default = false, debug = false, builder = false, bit_ops = false)]
    pub struct __SlateBits191U0 {
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
    pub struct __SlateBits187U0 {
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
    pub struct __SlateBits209U0 {
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
    pub struct __SlateBits158U0 {
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
    pub struct __SlateBits169U0 {
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

// /* Maximum value */
static mut sqlite3Stat: sqlite3StatType = sqlite3StatType {
    nowValue: {
        let mut __t0: [i64; 10] = unsafe { std::mem::zeroed() };
        __t0[0] = (0 as i32) as i64;
        __t0
    },
    mxValue: {
        let mut __t1: [i64; 10] = unsafe { std::mem::zeroed() };
        __t1[0] = (0 as i32) as i64;
        __t1
    },
};

// /*
// ** Elements of sqlite3Stat[] are protected by either the memory allocator
// ** mutex, or by the pcache1 mutex.  The following array determines which.
// */
static mut statMutex: [i8; 10] = [
    (0 as i32) as i8,
    (1 as i32) as i8,
    (1 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (1 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
];

#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.status.sqlite3_status")]
extern "C-unwind" fn sqlite3_status(
    mut op: i32,
    mut pCurrent: *mut i32,
    mut pHighwater: *mut i32,
    mut resetFlag: i32,
) -> i32 {
    let mut iCur: i64 = (0 as i32) as i64;
    let mut iHwtr: i64 = (0 as i32) as i64;
    let mut rc: i32 = 0 as i32;
    rc = sqlite3_status64(
        op,
        std::ptr::addr_of_mut!(iCur),
        std::ptr::addr_of_mut!(iHwtr),
        resetFlag,
    );
    if rc == (0 as i32) {
        unsafe {
            *pCurrent = iCur as i32;
        }
        unsafe {
            *pHighwater = iHwtr as i32;
        }
    }
    return rc;
}

// /*
// ** Query status information.
// */
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.status.sqlite3_status64")]
extern "C-unwind" fn sqlite3_status64(
    mut op: i32,
    mut pCurrent: *mut i64,
    mut pHighwater: *mut i64,
    mut resetFlag: i32,
) -> i32 {
    let mut pMutex: *mut sqlite3_mutex = unsafe { std::mem::zeroed() };
    {}
    if op < (0 as i32) || op >= ((((80 as u64) / (8 as u64)) as u32) as i32) {
        return unsafe { sqlite3MisuseError(143 as i32) };
    }
    let __v465: *mut sqlite3_mutex;
    if (unsafe {
        *unsafe { unsafe { std::ptr::addr_of!(statMutex) as *const i8 }.offset(op as isize) }
    }) != (0 as i8)
    {
        __v465 = unsafe { sqlite3Pcache1Mutex() };
    } else {
        __v465 = unsafe { sqlite3MallocMutex() };
    }
    pMutex = __v465;
    unsafe { sqlite3_mutex_enter(pMutex) };
    unsafe {
        *pCurrent = unsafe {
            *unsafe {
                unsafe { std::ptr::addr_of_mut!(sqlite3Stat.nowValue) as *mut i64 }
                    .offset(op as isize)
            }
        };
    }
    unsafe {
        *pHighwater = unsafe {
            *unsafe {
                unsafe { std::ptr::addr_of_mut!(sqlite3Stat.mxValue) as *mut i64 }
                    .offset(op as isize)
            }
        };
    }
    if resetFlag != (0 as i32) {
        unsafe {
            *unsafe {
                unsafe { std::ptr::addr_of_mut!(sqlite3Stat.mxValue) as *mut i64 }
                    .offset(op as isize)
            } = unsafe {
                *unsafe {
                    unsafe { std::ptr::addr_of_mut!(sqlite3Stat.nowValue) as *mut i64 }
                        .offset(op as isize)
                }
            };
        }
    }
    unsafe { sqlite3_mutex_leave(pMutex) };
    // /* Prevent warning when SQLITE_THREADSAFE=0 */
    pMutex;
    return 0 as i32;
}

// /* The database connection whose status is desired */
// /* Status verb */
// /* Write current value here */
// /* Write high-water mark here */
// /* Reset high-water mark if true */
// /*
// ** 32-bit variant of sqlite3_db_status64()
// */
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.status.sqlite3_db_status")]
extern "C-unwind" fn sqlite3_db_status(
    mut db: *mut sqlite3,
    mut op: i32,
    mut pCurrent: *mut i32,
    mut pHighwtr: *mut i32,
    mut resetFlag: i32,
) -> i32 {
    let mut C: i64 = (0 as i32) as i64;
    let mut H: i64 = (0 as i32) as i64;
    let mut rc: i32 = 0 as i32;
    rc = sqlite3_db_status64(
        db,
        op,
        std::ptr::addr_of_mut!(C),
        std::ptr::addr_of_mut!(H),
        resetFlag,
    );
    if rc == (0 as i32) {
        unsafe {
            *pCurrent = (C & ((2147483647 as i32) as i64)) as i32;
        }
        unsafe {
            *pHighwtr = (H & ((2147483647 as i32) as i64)) as i32;
        }
    }
    return rc;
}

// /*
// ** Query status information for a single database connection
// */
// /* The database connection whose status is desired */
// /* Status verb */
// /* Write current value here */
// /* Write high-water mark here */
// /* Reset high-water mark if true */
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.status.sqlite3_db_status64")]
extern "C-unwind" fn sqlite3_db_status64(
    mut db: *mut sqlite3,
    mut op: i32,
    mut pCurrent: *mut i64,
    mut pHighwtr: *mut i64,
    mut resetFlag: i32,
) -> i32 {
    // /* Return code */
    let mut rc: i32 = 0 as i32;
    unsafe { sqlite3_mutex_enter(unsafe { (*db).mutex }) };
    '__slate_break_456: {
        match op {
            0 => {
                let mut H: i32 = 0 as i32;
                unsafe {
                    *pCurrent = sqlite3LookasideUsed(db, std::ptr::addr_of_mut!(H)) as i64;
                }
                unsafe {
                    *pHighwtr = H as i64;
                }
                if resetFlag != (0 as i32) {
                    let mut p: *mut LookasideSlot = unsafe { (*db).lookaside.pFree };
                    if p != std::ptr::null_mut::<LookasideSlot>() {
                        '__slate_break_457: while (unsafe { (*p).pNext })
                            != std::ptr::null_mut::<LookasideSlot>()
                        {
                            p = unsafe { (*p).pNext };
                        }
                        unsafe {
                            (*p).pNext = unsafe { (*db).lookaside.pInit };
                        }
                        unsafe {
                            (*db).lookaside.pInit = unsafe { (*db).lookaside.pFree };
                        }
                        unsafe {
                            (*db).lookaside.pFree = std::ptr::null_mut::<LookasideSlot>();
                        }
                    }
                    p = unsafe { (*db).lookaside.pSmallFree };
                    if p != std::ptr::null_mut::<LookasideSlot>() {
                        '__slate_break_458: while (unsafe { (*p).pNext })
                            != std::ptr::null_mut::<LookasideSlot>()
                        {
                            p = unsafe { (*p).pNext };
                        }
                        unsafe {
                            (*p).pNext = unsafe { (*db).lookaside.pSmallInit };
                        }
                        unsafe {
                            (*db).lookaside.pSmallInit = unsafe { (*db).lookaside.pSmallFree };
                        }
                        unsafe {
                            (*db).lookaside.pSmallFree = std::ptr::null_mut::<LookasideSlot>();
                        }
                    }
                }
            }
            4 | 5 | 6 => {
                {}
                {}
                {}
                0 as i32;
                0 as i32;
                unsafe {
                    *pCurrent = (0 as i32) as i64;
                }
                unsafe {
                    *pHighwtr = ((unsafe {
                        *unsafe {
                            unsafe { (*db).lookaside.anStat.as_mut_ptr() as *mut u32 }
                                .offset((op - (4 as i32)) as isize)
                        }
                    }) as u64) as i64;
                }
                if resetFlag != (0 as i32) {
                    unsafe {
                        *unsafe {
                            unsafe { (*db).lookaside.anStat.as_mut_ptr() as *mut u32 }
                                .offset((op - (4 as i32)) as isize)
                        } = (0 as i32) as u32;
                    }
                }
                break '__slate_break_456;
                // /*
                //     ** Return an approximation for the amount of memory currently used
                //     ** by all pagers associated with the given database connection.  The
                //     ** highwater mark is meaningless and is returned as zero.
                //     */
            }
            11 | 1 => {
                let mut totalUsed: i64 = (0 as i32) as i64;
                let mut i: i32 = 0 as i32;
                unsafe { sqlite3BtreeEnterAll(db) };
                i = 0 as i32;
                '__slate_break_459: loop {
                    if !(i < unsafe { (*db).nDb }) {
                        break;
                    }
                    let mut pBt: *mut Btree =
                        unsafe { (*unsafe { unsafe { (*db).aDb }.offset(i as isize) }).pBt };
                    if pBt != std::ptr::null_mut::<Btree>() {
                        let mut pPager: *mut Pager = unsafe { sqlite3BtreePager(pBt) };
                        let mut nByte: i32 = unsafe { sqlite3PagerMemUsed(pPager) };
                        if op == (11 as i32) {
                            nByte = nByte / unsafe { sqlite3BtreeConnectionCount(pBt) };
                        }
                        let __v468: i64 = totalUsed;
                        let __v469: i64 = __v468 + (nByte as i64);
                        totalUsed = __v469;
                    }
                    let __v466: i32 = i;
                    let __v467: i32 = __v466 + (1 as i32);
                    i = __v467;
                }
                unsafe { sqlite3BtreeLeaveAll(db) };
                unsafe {
                    *pCurrent = totalUsed;
                }
                unsafe {
                    *pHighwtr = (0 as i32) as i64;
                }
                break '__slate_break_456;
                // /*
                //     ** *pCurrent gets an accurate estimate of the amount of memory used
                //     ** to store the schema for all databases (main, temp, and any ATTACHed
                //     ** databases.  *pHighwtr is set to zero.
                //     */
            }
            2 => {
                // /* Used to iterate through schemas */
                let mut i: i32 = 0 as i32;
                // /* Used to accumulate return value */
                let mut nByte: i32 = 0 as i32;
                unsafe { sqlite3BtreeEnterAll(db) };
                unsafe {
                    (*db).pnBytesFreed = std::ptr::addr_of_mut!(nByte);
                }
                0 as i32;
                unsafe {
                    (*db).lookaside.pEnd = unsafe { (*db).lookaside.pStart };
                }
                i = 0 as i32;
                '__slate_break_460: loop {
                    if !(i < unsafe { (*db).nDb }) {
                        break;
                    }
                    let mut pSchema: *mut Schema =
                        unsafe { (*unsafe { unsafe { (*db).aDb }.offset(i as isize) }).pSchema };
                    if pSchema != std::ptr::null_mut::<Schema>() {
                        let mut p: *mut HashElem = unsafe { std::mem::zeroed() };
                        let __v472: i32 = nByte;
                        let __v473: i32 = (__v472 as u32).wrapping_add(
                            ((unsafe {
                                unsafe { sqlite3Config.m.xRoundup }.unwrap()(
                                    ((40 as u64) as u32) as i32,
                                )
                            }) as u32)
                                .wrapping_mul(
                                    unsafe { (*pSchema).tblHash.count }
                                        .wrapping_add(unsafe { (*pSchema).trigHash.count })
                                        .wrapping_add(unsafe { (*pSchema).idxHash.count })
                                        .wrapping_add(unsafe { (*pSchema).fkeyHash.count }),
                                ),
                        ) as i32;
                        nByte = __v473;
                        let __v474: i32 = nByte;
                        let __v475: i32 = (((__v474 as i64) as u64).wrapping_add(unsafe {
                            sqlite3_msize((unsafe { (*pSchema).tblHash.ht }) as *mut ())
                        }) as u32) as i32;
                        nByte = __v475;
                        let __v476: i32 = nByte;
                        let __v477: i32 = (((__v476 as i64) as u64).wrapping_add(unsafe {
                            sqlite3_msize((unsafe { (*pSchema).trigHash.ht }) as *mut ())
                        }) as u32) as i32;
                        nByte = __v477;
                        let __v478: i32 = nByte;
                        let __v479: i32 = (((__v478 as i64) as u64).wrapping_add(unsafe {
                            sqlite3_msize((unsafe { (*pSchema).idxHash.ht }) as *mut ())
                        }) as u32) as i32;
                        nByte = __v479;
                        let __v480: i32 = nByte;
                        let __v481: i32 = (((__v480 as i64) as u64).wrapping_add(unsafe {
                            sqlite3_msize((unsafe { (*pSchema).fkeyHash.ht }) as *mut ())
                        }) as u32) as i32;
                        nByte = __v481;
                        p = unsafe {
                            (*unsafe { std::ptr::addr_of_mut!((*pSchema).trigHash) }).first
                        };
                        '__slate_break_461: while p != std::ptr::null_mut::<HashElem>() {
                            unsafe {
                                sqlite3DeleteTrigger(db, (unsafe { (*p).data }) as *mut Trigger)
                            };
                            p = unsafe { (*p).next };
                        }
                        p = unsafe {
                            (*unsafe { std::ptr::addr_of_mut!((*pSchema).tblHash) }).first
                        };
                        '__slate_break_462: while p != std::ptr::null_mut::<HashElem>() {
                            unsafe { sqlite3DeleteTable(db, (unsafe { (*p).data }) as *mut Table) };
                            p = unsafe { (*p).next };
                        }
                    }
                    let __v470: i32 = i;
                    let __v471: i32 = __v470 + (1 as i32);
                    i = __v471;
                }
                unsafe {
                    (*db).pnBytesFreed = std::ptr::null_mut::<i32>();
                }
                unsafe {
                    (*db).lookaside.pEnd = unsafe { (*db).lookaside.pTrueEnd };
                }
                unsafe { sqlite3BtreeLeaveAll(db) };
                unsafe {
                    *pHighwtr = (0 as i32) as i64;
                }
                unsafe {
                    *pCurrent = nByte as i64;
                }
                break '__slate_break_456;
                // /*
                //     ** *pCurrent gets an accurate estimate of the amount of memory used
                //     ** to store all prepared statements.
                //     ** *pHighwtr is set to zero.
                //     */
            }
            3 => {
                // /* Used to iterate through VMs */
                let mut pVdbe: *mut Vdbe = unsafe { std::mem::zeroed() };
                // /* Used to accumulate return value */
                let mut nByte: i32 = 0 as i32;
                unsafe {
                    (*db).pnBytesFreed = std::ptr::addr_of_mut!(nByte);
                }
                0 as i32;
                unsafe {
                    (*db).lookaside.pEnd = unsafe { (*db).lookaside.pStart };
                }
                pVdbe = unsafe { (*db).pVdbe };
                '__slate_break_463: while pVdbe != std::ptr::null_mut::<Vdbe>() {
                    unsafe { sqlite3VdbeDelete(pVdbe) };
                    pVdbe = unsafe { (*pVdbe).pVNext };
                }
                unsafe {
                    (*db).lookaside.pEnd = unsafe { (*db).lookaside.pTrueEnd };
                }
                unsafe {
                    (*db).pnBytesFreed = std::ptr::null_mut::<i32>();
                }
                // /* IMP: R-64479-57858 */
                unsafe {
                    *pHighwtr = (0 as i32) as i64;
                }
                unsafe {
                    *pCurrent = nByte as i64;
                }
                break '__slate_break_456;
                // /*
                //     ** Set *pCurrent to the total cache hits or misses encountered by all
                //     ** pagers the database handle is connected to. *pHighwtr is always set
                //     ** to zero.
                //     */
            }
            12 => {
                op = (9 as i32) + (1 as i32);
                // /* no break */
                {}
                let mut i: i32 = 0 as i32;
                let mut nRet: u64 = ((0 as i32) as i64) as u64;
                0 as i32;
                0 as i32;
                i = 0 as i32;
                '__slate_break_464: loop {
                    if !(i < unsafe { (*db).nDb }) {
                        break;
                    }
                    if (unsafe { (*unsafe { unsafe { (*db).aDb }.offset(i as isize) }).pBt })
                        != std::ptr::null_mut::<Btree>()
                    {
                        let mut pPager: *mut Pager = unsafe {
                            sqlite3BtreePager(unsafe {
                                (*unsafe { unsafe { (*db).aDb }.offset(i as isize) }).pBt
                            })
                        };
                        unsafe {
                            sqlite3PagerCacheStat(
                                pPager,
                                op,
                                resetFlag,
                                std::ptr::addr_of_mut!(nRet),
                            )
                        };
                    }
                    let _v488: i32 = i;
                    let _v489: i32 = _v488 + (1 as i32);
                    i = _v489;
                }
                // /* IMP: R-42420-56072 */
                unsafe {
                    *pHighwtr = (0 as i32) as i64;
                }
                // /* IMP: R-54100-20147 */
                // /* IMP: R-29431-39229 */
                unsafe {
                    *pCurrent = nRet as i64;
                }
                break '__slate_break_456;
                // /* Set *pCurrent to the number of bytes that the db database connection
                //     ** has spilled to the filesystem in temporary files that could have been
                //     ** stored in memory, had sufficient memory been available.
                //     ** The *pHighwater is always set to zero.
                //     */
            }
            7 | 8 | 9 => {
                let mut i: i32 = 0 as i32;
                let mut nRet: u64 = ((0 as i32) as i64) as u64;
                0 as i32;
                0 as i32;
                i = 0 as i32;
                '__slate_break_464: loop {
                    if !(i < unsafe { (*db).nDb }) {
                        break;
                    }
                    if (unsafe { (*unsafe { unsafe { (*db).aDb }.offset(i as isize) }).pBt })
                        != std::ptr::null_mut::<Btree>()
                    {
                        let mut pPager: *mut Pager = unsafe {
                            sqlite3BtreePager(unsafe {
                                (*unsafe { unsafe { (*db).aDb }.offset(i as isize) }).pBt
                            })
                        };
                        unsafe {
                            sqlite3PagerCacheStat(
                                pPager,
                                op,
                                resetFlag,
                                std::ptr::addr_of_mut!(nRet),
                            )
                        };
                    }
                    let __v482: i32 = i;
                    let __v483: i32 = __v482 + (1 as i32);
                    i = __v483;
                }
                // /* IMP: R-42420-56072 */
                unsafe {
                    *pHighwtr = (0 as i32) as i64;
                }
                // /* IMP: R-54100-20147 */
                // /* IMP: R-29431-39229 */
                unsafe {
                    *pCurrent = nRet as i64;
                }
                break '__slate_break_456;
                // /* Set *pCurrent to the number of bytes that the db database connection
                //     ** has spilled to the filesystem in temporary files that could have been
                //     ** stored in memory, had sufficient memory been available.
                //     ** The *pHighwater is always set to zero.
                //     */
            }
            13 => {
                let mut nRet: u64 = ((0 as i32) as i64) as u64;
                if (unsafe { (*unsafe { unsafe { (*db).aDb }.offset((1 as i32) as isize) }).pBt })
                    != std::ptr::null_mut::<Btree>()
                {
                    let mut pPager: *mut Pager = unsafe {
                        sqlite3BtreePager(unsafe {
                            (*unsafe { unsafe { (*db).aDb }.offset((1 as i32) as isize) }).pBt
                        })
                    };
                    unsafe {
                        sqlite3PagerCacheStat(
                            pPager,
                            9 as i32,
                            resetFlag,
                            std::ptr::addr_of_mut!(nRet),
                        )
                    };
                    let __v484: u64 = nRet;
                    let __v485: u64 = __v484.wrapping_mul(
                        ((unsafe {
                            sqlite3BtreeGetPageSize(unsafe {
                                (*unsafe { unsafe { (*db).aDb }.offset((1 as i32) as isize) }).pBt
                            })
                        }) as i64) as u64,
                    );
                    nRet = __v485;
                }
                let __v486: u64 = nRet;
                let __v487: u64 = __v486.wrapping_add(unsafe { (*db).nSpill });
                nRet = __v487;
                if resetFlag != (0 as i32) {
                    unsafe {
                        (*db).nSpill = ((0 as i32) as i64) as u64;
                    }
                }
                unsafe {
                    *pHighwtr = (0 as i32) as i64;
                }
                unsafe {
                    *pCurrent = nRet as i64;
                }
                break '__slate_break_456;
                // /* Set *pCurrent to non-zero if there are unresolved deferred foreign
                //     ** key constraints.  Set *pCurrent to zero if all foreign key constraints
                //     ** have been satisfied.  The *pHighwtr is always set to zero.
                //     */
            }
            10 => {
                // /* IMP: R-11967-56545 */
                unsafe {
                    *pHighwtr = (0 as i32) as i64;
                }
                unsafe {
                    *pCurrent = ((unsafe { (*db).nDeferredImmCons }) > ((0 as i32) as i64)
                        || (unsafe { (*db).nDeferredCons }) > ((0 as i32) as i64))
                        as i64;
                }
            }
            _ => {
                rc = 1 as i32;
            }
        }
    }
    unsafe { sqlite3_mutex_leave(unsafe { (*db).mutex }) };
    return rc;
}

// /* SQLITE_STATUS_MEMORY_USED */
// /* SQLITE_STATUS_PAGECACHE_USED */
// /* SQLITE_STATUS_PAGECACHE_OVERFLOW */
// /* SQLITE_STATUS_SCRATCH_USED */
// /* SQLITE_STATUS_SCRATCH_OVERFLOW */
// /* SQLITE_STATUS_MALLOC_SIZE */
// /* SQLITE_STATUS_PARSER_STACK */
// /* SQLITE_STATUS_PAGECACHE_SIZE */
// /* SQLITE_STATUS_SCRATCH_SIZE */
// /* SQLITE_STATUS_MALLOC_COUNT */
// /* The "wsdStat" macro will resolve to the status information
// ** state vector.  If writable static data is unsupported on the target,
// ** we have to locate the state vector at run-time.  In the more common
// ** case where writable static data is supported, wsdStat can refer directly
// ** to the "sqlite3Stat" state vector declared above.
// */
// /*
// ** Return the current value of a status parameter.  The caller must
// ** be holding the appropriate mutex.
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3StatusValue(mut op: i32) -> i64 {
    {}
    0 as i32;
    0 as i32;
    0 as i32;
    return unsafe {
        *unsafe {
            unsafe { std::ptr::addr_of_mut!(sqlite3Stat.nowValue) as *mut i64 }.offset(op as isize)
        }
    };
}

// /*
// ** Add N to the value of a status record.  The caller must hold the
// ** appropriate mutex.  (Locking is checked by assert()).
// **
// ** The StatusUp() routine can accept positive or negative values for N.
// ** The value of N is added to the current status value and the high-water
// ** mark is adjusted if necessary.
// **
// ** The StatusDown() routine lowers the current value by N.  The highwater
// ** mark is unchanged.  N must be non-negative for StatusDown().
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3StatusUp(mut op: i32, mut N: i32) {
    {}
    0 as i32;
    0 as i32;
    0 as i32;
    let __v488: *mut i64 = unsafe {
        unsafe { std::ptr::addr_of_mut!(sqlite3Stat.nowValue) as *mut i64 }.offset(op as isize)
    };
    let __v489: i64 = unsafe { *__v488 };
    let __v490: i64 = __v489 + (N as i64);
    unsafe {
        *__v488 = __v490;
    }
    if (unsafe {
        *unsafe {
            unsafe { std::ptr::addr_of_mut!(sqlite3Stat.nowValue) as *mut i64 }.offset(op as isize)
        }
    }) > unsafe {
        *unsafe {
            unsafe { std::ptr::addr_of_mut!(sqlite3Stat.mxValue) as *mut i64 }.offset(op as isize)
        }
    } {
        unsafe {
            *unsafe {
                unsafe { std::ptr::addr_of_mut!(sqlite3Stat.mxValue) as *mut i64 }
                    .offset(op as isize)
            } = unsafe {
                *unsafe {
                    unsafe { std::ptr::addr_of_mut!(sqlite3Stat.nowValue) as *mut i64 }
                        .offset(op as isize)
                }
            };
        }
    }
}

#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3StatusDown(mut op: i32, mut N: i32) {
    {}
    0 as i32;
    0 as i32;
    0 as i32;
    0 as i32;
    let __v491: *mut i64 = unsafe {
        unsafe { std::ptr::addr_of_mut!(sqlite3Stat.nowValue) as *mut i64 }.offset(op as isize)
    };
    let __v492: i64 = unsafe { *__v491 };
    let __v493: i64 = __v492 - (N as i64);
    unsafe {
        *__v491 = __v493;
    }
}

// /*
// ** Adjust the highwater mark if necessary.
// ** The caller must hold the appropriate mutex.
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3StatusHighwater(mut op: i32, mut X: i32) {
    let mut newValue: i64 = 0 as i64;
    {}
    0 as i32;
    newValue = X as i64;
    0 as i32;
    0 as i32;
    0 as i32;
    0 as i32;
    if newValue
        > unsafe {
            *unsafe {
                unsafe { std::ptr::addr_of_mut!(sqlite3Stat.mxValue) as *mut i64 }
                    .offset(op as isize)
            }
        }
    {
        unsafe {
            *unsafe {
                unsafe { std::ptr::addr_of_mut!(sqlite3Stat.mxValue) as *mut i64 }
                    .offset(op as isize)
            } = newValue;
        }
    }
}

// /*
// ** Count the number of slots of lookaside memory that are outstanding
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3LookasideUsed(mut db: *mut sqlite3, mut pHighwater: *mut i32) -> i32 {
    let mut nInit: u32 = countLookasideSlots(unsafe { (*db).lookaside.pInit });
    let mut nFree: u32 = countLookasideSlots(unsafe { (*db).lookaside.pFree });
    let __v494: u32 = nInit;
    let __v495: u32 =
        __v494.wrapping_add(countLookasideSlots(unsafe { (*db).lookaside.pSmallInit }));
    nInit = __v495;
    let __v496: u32 = nFree;
    let __v497: u32 =
        __v496.wrapping_add(countLookasideSlots(unsafe { (*db).lookaside.pSmallFree }));
    nFree = __v497;
    // /* SQLITE_OMIT_TWOSIZE_LOOKASIDE */
    0 as i32;
    if pHighwater != std::ptr::null_mut::<i32>() {
        unsafe {
            *pHighwater = unsafe { (*db).lookaside.nSlot }.wrapping_sub(nInit) as i32;
        }
    }
    return unsafe { (*db).lookaside.nSlot }.wrapping_sub(nInit.wrapping_add(nFree)) as i32;
}

// /*
// ** Return the number of LookasideSlot elements on the linked list
// */
fn countLookasideSlots(mut p: *mut LookasideSlot) -> u32 {
    let mut cnt: u32 = (0 as i32) as u32;
    '__slate_break_455: while p != std::ptr::null_mut::<LookasideSlot>() {
        p = unsafe { (*p).pNext };
        let __v498: u32 = cnt;
        let __v499: u32 = __v498.wrapping_add((1 as i32) as u32);
        cnt = __v499;
    }
    return cnt;
}
