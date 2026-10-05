unsafe extern "C" {
    static mut sqlite3StdType: [*const i8; 0];
    static mut sqlite3UpperToLower: [u8; 0];
    static mut sqlite3CtypeMap: [u8; 0];
    static mut sqlite3Config: Sqlite3Config;
    fn sqlite3_log(iErrCode: i32, zFormat: *const i8, ...);
    fn memcpy(__dest: *mut (), __src: *const (), __n: u64) -> *mut ();
    fn strncmp(__s1: *const i8, __s2: *const i8, __n: u64) -> i32;
    fn strspn(__s: *const i8, __accept: *const i8) -> u64;
    fn strlen(__s: *const i8) -> u64;
    fn sqlite3OsGetLastError(__v677: *mut sqlite3_vfs) -> i32;
    fn sqlite3DbMallocRawNN(__v683: *mut sqlite3, __v684: u64) -> *mut ();
    fn sqlite3DbRealloc(__v685: *mut sqlite3, __v686: *mut (), __v687: u64) -> *mut ();
    fn sqlite3DbFree(__v688: *mut sqlite3, __v689: *mut ());
    fn sqlite3VMPrintf(
        __v696: *mut sqlite3,
        __v697: *const i8,
        __v698: core::ffi::VaList<'_>,
    ) -> *mut i8;
    fn sqlite3RowSetClear(__v712: *mut ());
    fn sqlite3ValueSetStr(
        __v772: *mut sqlite3_value,
        __v773: i32,
        __v774: *const (),
        __v775: u8,
        __v776: Option<unsafe extern "C-unwind" fn(*mut ())>,
    );
    fn sqlite3ValueSetNull(__v777: *mut sqlite3_value);
    fn sqlite3ValueNew(__v778: *mut sqlite3) -> *mut sqlite3_value;
    fn exp(__x: f64) -> f64;
    fn __builtin_clzll(__v793: u64) -> i32;
    fn __builtin_inff() -> f32;
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
    __slate_bits_0: __slate_bits::__SlateBits75U0,
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
    __slate_bits_0: __slate_bits::__SlateBits101U0,
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
    __slate_bits_0: __slate_bits::__SlateBits113U0,
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

// /*
// ** Digit pairs used to convert a U64 or I64 into text, two digits
// ** at a time.
// */
#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord201 {
    a: [i8; 201],
    forceAlignment: i16,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord202 {
    a: [i8; 21],
    forceAlignment: u16,
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
    pub struct __SlateBits75U0 {
        #[bits(4)]
        pub notNull: u32,
        #[bits(4)]
        pub eCType: u32,
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
    pub struct __SlateBits101U0 {
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
    #[bitfields::bitfield([u8; 2], c_names = true, new = false, from_into_bits = false, from_traits = false, default = false, debug = false, builder = false, bit_ops = false)]
    pub struct __SlateBits113U0 {
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

static mut aBase: __SlateAlign16<[u64; 27]> = __SlateAlign16([
    9223372036854775808 as u64,
    11529215046068469760 as u64,
    14411518807585587200 as u64,
    18014398509481984000 as u64,
    11258999068426240000 as u64,
    14073748835532800000 as u64,
    17592186044416000000 as u64,
    10995116277760000000 as u64,
    13743895347200000000 as u64,
    17179869184000000000 as u64,
    10737418240000000000 as u64,
    13421772800000000000 as u64,
    16777216000000000000 as u64,
    10485760000000000000 as u64,
    13107200000000000000 as u64,
    16384000000000000000 as u64,
    10240000000000000000 as u64,
    12800000000000000000 as u64,
    16000000000000000000 as u64,
    10000000000000000000 as u64,
    12500000000000000000 as u64,
    15625000000000000000 as u64,
    9765625000000000000 as u64,
    12207031250000000000 as u64,
    15258789062500000000 as u64,
    9536743164062500000 as u64,
    11920928955078125000 as u64,
]);

static mut aScale: __SlateAlign16<[u64; 26]> = __SlateAlign16([
    9244100769003082158 as u64,
    14934650266808366570 as u64,
    12064114410120881697 as u64,
    9745314011399999080 as u64,
    15744403932561434696 as u64,
    12718228212127407596 as u64,
    10273702932711667006 as u64,
    16598062275523971834 as u64,
    13407807929942597099 as u64,
    10830740992659433045 as u64,
    17498005798264095394 as u64,
    14134776518227074636 as u64,
    11417981541647679048 as u64,
    14757395258967641292 as u64,
    14901161193847656250 as u64,
    12037062152420224081 as u64,
    9723461371658033917 as u64,
    15709099088952724969 as u64,
    12689709186578246116 as u64,
    10250665447337476733 as u64,
    16560843210556190337 as u64,
    13377742608693866209 as u64,
    10806454419566533849 as u64,
    17458768723248864463 as u64,
    14103081061443981063 as u64,
    11392378155556871081 as u64,
]);

static mut aScaleLo: __SlateAlign16<[u32; 26]> = __SlateAlign16([
    (542869869 as i32) as u32,
    (1376144557 as i32) as u32,
    2938827448 as u32,
    (1517765799 as i32) as u32,
    2939790453 as u32,
    3180165454 as u32,
    (1417589883 as i32) as u32,
    (213165475 as i32) as u32,
    2465418594 as u32,
    (980027385 as i32) as u32,
    4209144473 as u32,
    2862080332 as u32,
    (2002690661 as i32) as u32,
    3435973836 as u32,
    (0 as i32) as u32,
    2576388278 as u32,
    (1772103867 as i32) as u32,
    3893260104 as u32,
    (1589665232 as i32) as u32,
    (341348116 as i32) as u32,
    2400610505 as u32,
    (1838497324 as i32) as u32,
    (1253945104 as i32) as u32,
    3160619833 as u32,
    (176566145 as i32) as u32,
    (1812439746 as i32) as u32,
]);

static mut sqlite3DigitPairs: __SlateRecord201 = {
    let mut __t0: __SlateRecord201 = unsafe { std::mem::zeroed() };
    __t0.a = [
        48 as i8, 48 as i8, 48 as i8, 49 as i8, 48 as i8, 50 as i8, 48 as i8, 51 as i8, 48 as i8,
        52 as i8, 48 as i8, 53 as i8, 48 as i8, 54 as i8, 48 as i8, 55 as i8, 48 as i8, 56 as i8,
        48 as i8, 57 as i8, 49 as i8, 48 as i8, 49 as i8, 49 as i8, 49 as i8, 50 as i8, 49 as i8,
        51 as i8, 49 as i8, 52 as i8, 49 as i8, 53 as i8, 49 as i8, 54 as i8, 49 as i8, 55 as i8,
        49 as i8, 56 as i8, 49 as i8, 57 as i8, 50 as i8, 48 as i8, 50 as i8, 49 as i8, 50 as i8,
        50 as i8, 50 as i8, 51 as i8, 50 as i8, 52 as i8, 50 as i8, 53 as i8, 50 as i8, 54 as i8,
        50 as i8, 55 as i8, 50 as i8, 56 as i8, 50 as i8, 57 as i8, 51 as i8, 48 as i8, 51 as i8,
        49 as i8, 51 as i8, 50 as i8, 51 as i8, 51 as i8, 51 as i8, 52 as i8, 51 as i8, 53 as i8,
        51 as i8, 54 as i8, 51 as i8, 55 as i8, 51 as i8, 56 as i8, 51 as i8, 57 as i8, 52 as i8,
        48 as i8, 52 as i8, 49 as i8, 52 as i8, 50 as i8, 52 as i8, 51 as i8, 52 as i8, 52 as i8,
        52 as i8, 53 as i8, 52 as i8, 54 as i8, 52 as i8, 55 as i8, 52 as i8, 56 as i8, 52 as i8,
        57 as i8, 53 as i8, 48 as i8, 53 as i8, 49 as i8, 53 as i8, 50 as i8, 53 as i8, 51 as i8,
        53 as i8, 52 as i8, 53 as i8, 53 as i8, 53 as i8, 54 as i8, 53 as i8, 55 as i8, 53 as i8,
        56 as i8, 53 as i8, 57 as i8, 54 as i8, 48 as i8, 54 as i8, 49 as i8, 54 as i8, 50 as i8,
        54 as i8, 51 as i8, 54 as i8, 52 as i8, 54 as i8, 53 as i8, 54 as i8, 54 as i8, 54 as i8,
        55 as i8, 54 as i8, 56 as i8, 54 as i8, 57 as i8, 55 as i8, 48 as i8, 55 as i8, 49 as i8,
        55 as i8, 50 as i8, 55 as i8, 51 as i8, 55 as i8, 52 as i8, 55 as i8, 53 as i8, 55 as i8,
        54 as i8, 55 as i8, 55 as i8, 55 as i8, 56 as i8, 55 as i8, 57 as i8, 56 as i8, 48 as i8,
        56 as i8, 49 as i8, 56 as i8, 50 as i8, 56 as i8, 51 as i8, 56 as i8, 52 as i8, 56 as i8,
        53 as i8, 56 as i8, 54 as i8, 56 as i8, 55 as i8, 56 as i8, 56 as i8, 56 as i8, 57 as i8,
        57 as i8, 48 as i8, 57 as i8, 49 as i8, 57 as i8, 50 as i8, 57 as i8, 51 as i8, 57 as i8,
        52 as i8, 57 as i8, 53 as i8, 57 as i8, 54 as i8, 57 as i8, 55 as i8, 57 as i8, 56 as i8,
        57 as i8, 57 as i8, 0 as i8,
    ];
    __t0
};

static mut x: __SlateAlign16<[u8; 32]> = __SlateAlign16([
    ((10 as i32) as i8) as u8,
    ((10 as i32) as i8) as u8,
    ((9 as i32) as i8) as u8,
    ((9 as i32) as i8) as u8,
    ((8 as i32) as i8) as u8,
    ((8 as i32) as i8) as u8,
    ((7 as i32) as i8) as u8,
    ((7 as i32) as i8) as u8,
    ((7 as i32) as i8) as u8,
    ((6 as i32) as i8) as u8,
    ((6 as i32) as i8) as u8,
    ((6 as i32) as i8) as u8,
    ((5 as i32) as i8) as u8,
    ((5 as i32) as i8) as u8,
    ((5 as i32) as i8) as u8,
    ((4 as i32) as i8) as u8,
    ((4 as i32) as i8) as u8,
    ((4 as i32) as i8) as u8,
    ((4 as i32) as i8) as u8,
    ((3 as i32) as i8) as u8,
    ((3 as i32) as i8) as u8,
    ((3 as i32) as i8) as u8,
    ((3 as i32) as i8) as u8,
    ((3 as i32) as i8) as u8,
    ((3 as i32) as i8) as u8,
    ((2 as i32) as i8) as u8,
    ((2 as i32) as i8) as u8,
    ((2 as i32) as i8) as u8,
    ((2 as i32) as i8) as u8,
    ((2 as i32) as i8) as u8,
    ((2 as i32) as i8) as u8,
    ((2 as i32) as i8) as u8,
]);

static mut a: __SlateAlign16<[i16; 8]> = __SlateAlign16([
    (0 as i32) as i16,
    (2 as i32) as i16,
    (3 as i32) as i16,
    (5 as i32) as i16,
    (6 as i32) as i16,
    (7 as i32) as i16,
    (8 as i32) as i16,
    (9 as i32) as i16,
]);

// /* Convenient short-hand */
// /*
// ** Some systems have stricmp().  Others have strcasecmp().  Because
// ** there is no consistency, we will define our own.
// **
// ** IMPLEMENTATION-OF: R-30243-02494 The sqlite3_stricmp() and
// ** sqlite3_strnicmp() APIs allow applications and extensions to compare
// ** the contents of two buffers containing UTF-8 strings in a
// ** case-independent fashion, using the same definition of "case
// ** independence" that SQLite uses internally when comparing identifiers.
// */
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.util.sqlite3_stricmp")]
extern "C-unwind" fn sqlite3_stricmp(mut zLeft: *const i8, mut zRight: *const i8) -> i32 {
    if zLeft == std::ptr::null::<i8>() {
        return if zRight != std::ptr::null::<i8>() {
            -(1 as i32)
        } else {
            0 as i32
        };
    } else {
        if zRight == std::ptr::null::<i8>() {
            return 1 as i32;
        }
    }
    return sqlite3StrICmp(zLeft, zRight);
}

#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.util.sqlite3_strnicmp")]
extern "C-unwind" fn sqlite3_strnicmp(
    mut zLeft: *const i8,
    mut zRight: *const i8,
    mut N: i32,
) -> i32 {
    let mut a_439: *mut u8 = unsafe { std::mem::zeroed() };
    let mut b: *mut u8 = unsafe { std::mem::zeroed() };
    if zLeft == std::ptr::null::<i8>() {
        return if zRight != std::ptr::null::<i8>() {
            -(1 as i32)
        } else {
            0 as i32
        };
    } else {
        if zRight == std::ptr::null::<i8>() {
            return 1 as i32;
        }
    }
    a_439 = zLeft as *mut u8;
    b = zRight as *mut u8;
    '__slate_break_791: loop {
        let __v840: i32 = N;
        let __v841: i32 = __v840 - (1 as i32);
        N = __v841;
        if !(__v840 > (0 as i32)
            && (((unsafe { *a_439 }) as u32) as i32) != (0 as i32)
            && (((unsafe {
                *unsafe {
                    unsafe { std::ptr::addr_of!(sqlite3UpperToLower) as *const u8 }
                        .offset((((unsafe { *a_439 }) as u32) as i32) as isize)
                }
            }) as u32) as i32)
                == (((unsafe {
                    *unsafe {
                        unsafe { std::ptr::addr_of!(sqlite3UpperToLower) as *const u8 }
                            .offset((((unsafe { *b }) as u32) as i32) as isize)
                    }
                }) as u32) as i32))
        {
            break;
        }
        let __v842: *mut u8 = a_439;
        let __v843: *mut u8 = unsafe { __v842.offset((1 as i32) as isize) };
        a_439 = __v843;
        let __v844: *mut u8 = b;
        let __v845: *mut u8 = unsafe { __v844.offset((1 as i32) as isize) };
        b = __v845;
    }
    return if N < (0 as i32) {
        0 as i32
    } else {
        (((unsafe {
            *unsafe {
                unsafe { std::ptr::addr_of!(sqlite3UpperToLower) as *const u8 }
                    .offset((((unsafe { *a_439 }) as u32) as i32) as isize)
            }
        }) as u32) as i32)
            - (((unsafe {
                *unsafe {
                    unsafe { std::ptr::addr_of!(sqlite3UpperToLower) as *const u8 }
                        .offset((((unsafe { *b }) as u32) as i32) as isize)
                }
            }) as u32) as i32)
    };
}

#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3StrICmp(mut zLeft: *const i8, mut zRight: *const i8) -> i32 {
    let mut a_432: *mut u8 = unsafe { std::mem::zeroed() };
    let mut b: *mut u8 = unsafe { std::mem::zeroed() };
    let mut c: i32 = 0 as i32;
    let mut x_435: i32 = 0 as i32;
    a_432 = zLeft as *mut u8;
    b = zRight as *mut u8;
    '__slate_break_790: loop {
        c = ((unsafe { *a_432 }) as u32) as i32;
        x_435 = ((unsafe { *b }) as u32) as i32;
        if c == x_435 {
            if c == (0 as i32) {
                break '__slate_break_790;
            }
        } else {
            c = (((unsafe {
                *unsafe {
                    unsafe { std::ptr::addr_of!(sqlite3UpperToLower) as *const u8 }
                        .offset(c as isize)
                }
            }) as u32) as i32)
                - (((unsafe {
                    *unsafe {
                        unsafe { std::ptr::addr_of!(sqlite3UpperToLower) as *const u8 }
                            .offset(x_435 as isize)
                    }
                }) as u32) as i32);
            if c != (0 as i32) {
                break '__slate_break_790;
            }
        }
        let __v846: *mut u8 = a_432;
        let __v847: *mut u8 = unsafe { __v846.offset((1 as i32) as isize) };
        a_432 = __v847;
        let __v848: *mut u8 = b;
        let __v849: *mut u8 = unsafe { __v848.offset((1 as i32) as isize) };
        b = __v849;
    }
    return c;
}

// /* SQLITE_OMIT_FLOATING_POINT */
// /*
// ** Compute a string length that is limited to what can be stored in
// ** lower 30 bits of a 32-bit signed integer.
// **
// ** The value returned will never be negative.  Nor will it ever be greater
// ** than the actual length of the string.  For very long strings (greater
// ** than 1GiB) the value returned might be less than the true string length.
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3Strlen30(mut z: *const i8) -> i32 {
    if z == std::ptr::null::<i8>() {
        return 0 as i32;
    }
    return (1073741823 as i32) & (((unsafe { strlen(z) }) as u32) as i32);
}

// /*
// ** Return the declared type of a column.  Or return zDflt if the column
// ** has no declared type.
// **
// ** The column type is an extra string stored after the zero-terminator on
// ** the column name if and only if the COLFLAG_HASTYPE flag is set.
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3ColumnType(mut pCol: *mut Column, mut zDflt: *mut i8) -> *mut i8 {
    if (((unsafe { (*pCol).colFlags }) as u32) as i32) & (4 as i32) != (0 as i32) {
        return unsafe {
            unsafe {
                unsafe { (*pCol).zCnName }
                    .offset((unsafe { strlen((unsafe { (*pCol).zCnName }) as *const i8) }) as isize)
            }
            .offset((1 as i32) as isize)
        };
    } else {
        if ((unsafe { (*pCol).__slate_bits_0.__get_eCType() }) as i32) != (0 as i32) {
            0 as i32;
            return (unsafe {
                *unsafe {
                    unsafe { std::ptr::addr_of_mut!(sqlite3StdType) as *mut *const i8 }.offset(
                        (((unsafe { (*pCol).__slate_bits_0.__get_eCType() }) as i32) - (1 as i32))
                            as isize,
                    )
                }
            }) as *mut i8;
        } else {
            return zDflt;
        }
    }
    return unsafe { std::mem::zeroed() };
}

// /*
// ** Return true if the floating point value is Not a Number (NaN).
// **
// ** Use the math library isnan() function if compiled with SQLITE_HAVE_ISNAN.
// ** Otherwise, we have our own implementation that works on most systems.
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3IsNaN(mut x_382: f64) -> i32 {
    // /* The value return */
    let mut rc: i32 = 0 as i32;
    rc = x_382.is_nan() as i32;
    // /* HAVE_ISNAN */
    {}
    return rc;
}

// /* SQLITE_OMIT_FLOATING_POINT */
// /*
// ** Return true if the floating point value is NaN or +Inf or -Inf.
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3IsOverflow(mut x_384: f64) -> i32 {
    // /* The value return */
    let mut rc: i32 = 0 as i32;
    let mut y: u64 = 0 as u64;
    unsafe {
        memcpy(
            std::ptr::addr_of_mut!(y) as *mut (),
            std::ptr::addr_of_mut!(x_384) as *const (),
            8 as u64,
        )
    };
    rc = (y & (((2047 as i32) as i64) as u64) << (52 as i32)
        == (((2047 as i32) as i64) as u64) << (52 as i32)) as i32;
    return rc;
}

// /*
// ** Decode a floating-point value into an approximate decimal
// ** representation.
// **
// ** If iRound<=0 then round to -iRound significant digits to the
// ** the right of the decimal point, or to a maximum of mxRound total
// ** significant digits.
// **
// ** If iRound>0 round to min(iRound,mxRound) significant digits total.
// **
// ** mxRound must be positive.
// **
// ** The significant digits of the decimal representation are
// ** stored in p->z[] which is a often (but not always) a pointer
// ** into the middle of p->zBuf[].  There are p->n significant digits.
// ** The p->z[] array is *not* zero-terminated.
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3FpDecode(
    mut p: *mut FpDecode,
    mut r: f64,
    mut iRound: i32,
    mut mxRound: i32,
) {
    // /* Index into zBuf[] where to put next character */
    let mut i: i32 = 0 as i32;
    // /* Number of digits */
    let mut n: i32 = 0 as i32;
    // /* mantissa */
    let mut v: u64 = 0 as u64;
    // /* Base-2 and base-10 exponent */
    let mut e: i32 = 0 as i32;
    let mut exp: i32 = 0 as i32;
    // /* Local alias for p->zBuf */
    let mut zBuf: *mut i8 = unsafe { std::mem::zeroed() };
    // /* Local alias for p->z */
    let mut z: *mut i8 = unsafe { std::mem::zeroed() };
    unsafe {
        (*p).isSpecial = (0 as i32) as i8;
    }
    0 as i32;
    // /* Convert negative numbers to positive.  Deal with Infinity, 0.0, and
    //   ** NaN. */
    if r < 0.0f64 {
        unsafe {
            (*p).sign = (45 as i32) as i8;
        }
        r = -r;
    } else {
        if r == 0.0f64 {
            unsafe {
                (*p).sign = (43 as i32) as i8;
            }
            unsafe {
                (*p).n = 1 as i32;
            }
            unsafe {
                (*p).iDP = 1 as i32;
            }
            unsafe {
                (*p).z = b"0\0".as_ptr() as *mut i8;
            }
            return;
        } else {
            unsafe {
                (*p).sign = (43 as i32) as i8;
            }
        }
    }
    unsafe {
        memcpy(
            std::ptr::addr_of_mut!(v) as *mut (),
            std::ptr::addr_of_mut!(r) as *const (),
            ((8 as i32) as i64) as u64,
        )
    };
    e = ((v >> (52 as i32) & (((2047 as i32) as i64) as u64)) as u32) as i32;
    if e == (2047 as i32) {
        unsafe {
            (*p).isSpecial =
                ((1 as i32) + ((v != ((9218868437227405312 as i64) as u64)) as i32)) as i8;
        }
        unsafe {
            (*p).n = 0 as i32;
        }
        unsafe {
            (*p).iDP = 0 as i32;
        }
        unsafe {
            (*p).z = unsafe { (*p).zBuf.as_mut_ptr() as *mut i8 };
        }
        return;
    }
    let __v850: u64 = v;
    let __v851: u64 = __v850 & (4503599627370495 as u64);
    v = __v851;
    if e == (0 as i32) {
        let mut nn: i32 = countLeadingZeros(v);
        let __v852: u64 = v;
        let __v853: u64 = __v852 << nn;
        v = __v853;
        e = -(1074 as i32) - nn;
    } else {
        v = v << (11 as i32) | (((1 as i32) as i64) as u64) << (63 as i32);
        let __v854: i32 = e;
        let __v855: i32 = __v854 - (1086 as i32);
        e = __v855;
    }
    sqlite3Fp2Convert10(
        v,
        e,
        if iRound <= (0 as i32) || iRound >= (18 as i32) {
            18 as i32
        } else {
            iRound + (1 as i32)
        },
        std::ptr::addr_of_mut!(v),
        std::ptr::addr_of_mut!(exp),
    );
    // /* Extract significant digits, start at the right-most slot in p->zBuf
    //   ** and working back to the right.  "i" keeps track of the next slot in
    //   ** which to store a digit. */
    0 as i32;
    0 as i32;
    zBuf = unsafe { (*p).zBuf.as_mut_ptr() as *mut i8 };
    i = 20 as i32;
    '__slate_break_818: while v >= (((10 as i32) as i64) as u64) {
        let mut kk: i32 = ((v % (((100 as i32) as i64) as u64))
            .wrapping_mul(((2 as i32) as i64) as u64) as u32) as i32;
        0 as i32;
        0 as i32;
        0 as i32;
        unsafe {
            *((unsafe { zBuf.offset((i - (2 as i32)) as isize) }) as *mut u16) = unsafe {
                *((unsafe {
                    unsafe { std::ptr::addr_of!(sqlite3DigitPairs.a) as *const i8 }
                        .offset(kk as isize)
                }) as *mut u16)
            };
        }
        let __v856: i32 = i;
        let __v857: i32 = __v856 - (2 as i32);
        i = __v857;
        let __v858: u64 = v;
        let __v859: u64 = __v858 / (((100 as i32) as i64) as u64);
        v = __v859;
    }
    if v != (0 as u64) {
        0 as i32;
        0 as i32;
        let __v860: i32 = i;
        let __v861: i32 = __v860 - (1 as i32);
        i = __v861;
        unsafe {
            *unsafe { zBuf.offset(__v861 as isize) } =
                (v.wrapping_add(((48 as i32) as i64) as u64) as u8) as i8;
        }
    }
    // /* SQLITE_AVOID_U64_DIVIDE */
    0 as i32;
    // /* Total number of digits extracted */
    n = (20 as i32) - i;
    0 as i32;
    0 as i32;
    unsafe {
        (*p).iDP = n + exp;
    }
    if iRound <= (0 as i32) {
        iRound = (unsafe { (*p).iDP }) - iRound;
        if iRound == (0 as i32)
            && ((unsafe { *unsafe { zBuf.offset(i as isize) } }) as i32) >= (53 as i32)
        {
            iRound = 1 as i32;
            let __v862: i32 = i;
            let __v863: i32 = __v862 - (1 as i32);
            i = __v863;
            unsafe {
                *unsafe { zBuf.offset(__v863 as isize) } = (48 as i32) as i8;
            }
            let __v864: i32 = n;
            let __v865: i32 = __v864 + (1 as i32);
            n = __v865;
            let __v866: *mut FpDecode = p;
            let __v867: i32 = unsafe { (*__v866).iDP };
            let __v868: i32 = __v867 + (1 as i32);
            unsafe {
                (*__v866).iDP = __v868;
            }
        }
    }
    // /* z points to the first digit */
    z = unsafe { zBuf.offset(i as isize) };
    if iRound > (0 as i32) && (iRound < n || n > mxRound) {
        if iRound > mxRound {
            iRound = mxRound;
        }
        if iRound == (17 as i32) {
            // /* If the precision is exactly 17, which only happens with the "!"
            //       ** flag (ex: "%!.17g") then try to reduce the precision if that
            //       ** yields text that will round-trip to the original floating-point.
            //       ** value.  Thus, for exaple, 49.47 will render as 49.47, rather than
            //       ** as 49.469999999999999. */
            if ((unsafe { *unsafe { z.offset((15 as i32) as isize) } }) as i32) == (57 as i32)
                && ((unsafe { *unsafe { z.offset((14 as i32) as isize) } }) as i32) == (57 as i32)
            {
                let mut jj: i32 = 0 as i32;
                let mut kk: i32 = 0 as i32;
                let mut v2: u64 = 0 as u64;
                jj = 14 as i32;
                '__slate_break_819: loop {
                    if !(jj > (0 as i32)
                        && ((unsafe { *unsafe { z.offset((jj - (1 as i32)) as isize) } }) as i32)
                            == (57 as i32))
                    {
                        break;
                    }
                    let __v869: i32 = jj;
                    let __v870: i32 = __v869 - (1 as i32);
                    jj = __v870;
                }
                if jj == (0 as i32) {
                    v2 = ((1 as i32) as i64) as u64;
                } else {
                    v2 = ((((unsafe { *unsafe { z.offset((0 as i32) as isize) } }) as i32)
                        - (48 as i32)) as i64) as u64;
                    kk = 1 as i32;
                    '__slate_break_820: loop {
                        if !(kk < jj) {
                            break;
                        }
                        v2 = v2
                            .wrapping_mul(((10 as i32) as i64) as u64)
                            .wrapping_add(
                                (((unsafe { *unsafe { z.offset(kk as isize) } }) as i32) as i64)
                                    as u64,
                            )
                            .wrapping_sub(((48 as i32) as i64) as u64);
                        let __v871: i32 = kk;
                        let __v872: i32 = __v871 + (1 as i32);
                        kk = __v872;
                    }
                    let __v873: u64 = v2;
                    let __v874: u64 = __v873.wrapping_add(((1 as i32) as i64) as u64);
                    v2 = __v874;
                }
                if r == sqlite3Fp10Convert2(v2, exp + n - jj) {
                    iRound = jj + (1 as i32);
                }
            } else {
                if (unsafe { (*p).iDP }) >= n
                    || ((unsafe { *unsafe { z.offset((15 as i32) as isize) } }) as i32)
                        == (48 as i32)
                        && ((unsafe { *unsafe { z.offset((14 as i32) as isize) } }) as i32)
                            == (48 as i32)
                        && ((unsafe { *unsafe { z.offset((13 as i32) as isize) } }) as i32)
                            == (48 as i32)
                {
                    let mut jj: i32 = 0 as i32;
                    let mut kk: i32 = 0 as i32;
                    let mut v2: u64 = 0 as u64;
                    0 as i32;
                    jj = 13 as i32;
                    '__slate_break_821: loop {
                        if !(((unsafe { *unsafe { z.offset((jj - (1 as i32)) as isize) } }) as i32)
                            == (48 as i32))
                        {
                            break;
                        }
                        let __v875: i32 = jj;
                        let __v876: i32 = __v875 - (1 as i32);
                        jj = __v876;
                    }
                    v2 = ((((unsafe { *unsafe { z.offset((0 as i32) as isize) } }) as i32)
                        - (48 as i32)) as i64) as u64;
                    kk = 1 as i32;
                    '__slate_break_822: loop {
                        if !(kk < jj) {
                            break;
                        }
                        v2 = v2
                            .wrapping_mul(((10 as i32) as i64) as u64)
                            .wrapping_add(
                                (((unsafe { *unsafe { z.offset(kk as isize) } }) as i32) as i64)
                                    as u64,
                            )
                            .wrapping_sub(((48 as i32) as i64) as u64);
                        let __v877: i32 = kk;
                        let __v878: i32 = __v877 + (1 as i32);
                        kk = __v878;
                    }
                    if r == sqlite3Fp10Convert2(v2, exp + n - jj) {
                        iRound = jj + (1 as i32);
                    }
                }
            }
        }
        n = iRound;
        if ((unsafe { *unsafe { z.offset(iRound as isize) } }) as i32) >= (53 as i32) {
            let mut j: i32 = iRound - (1 as i32);
            // /*exit-by-break*/
            '__slate_break_823: while (1 as i32) != (0 as i32) {
                let __v879: *mut i8 = unsafe { z.offset(j as isize) };
                let __v880: i8 = unsafe { *__v879 };
                let __v881: i8 = ((__v880 as i32) + (1 as i32)) as i8;
                unsafe {
                    *__v879 = __v881;
                }
                if ((unsafe { *unsafe { z.offset(j as isize) } }) as i32) <= (57 as i32) {
                    break '__slate_break_823;
                }
                unsafe {
                    *unsafe { z.offset(j as isize) } = (48 as i32) as i8;
                }
                if j == (0 as i32) {
                    let __v882: *mut i8 = z;
                    let __v883: *mut i8 = unsafe { __v882.offset(-((1 as i32) as isize)) };
                    z = __v883;
                    unsafe {
                        *unsafe { z.offset((0 as i32) as isize) } = (49 as i32) as i8;
                    }
                    let __v884: i32 = n;
                    let __v885: i32 = __v884 + (1 as i32);
                    n = __v885;
                    let __v886: *mut FpDecode = p;
                    let __v887: i32 = unsafe { (*__v886).iDP };
                    let __v888: i32 = __v887 + (1 as i32);
                    unsafe {
                        (*__v886).iDP = __v888;
                    }
                    break '__slate_break_823;
                } else {
                    let __v889: i32 = j;
                    let __v890: i32 = __v889 - (1 as i32);
                    j = __v890;
                }
            }
        }
    }
    0 as i32;
    '__slate_break_824: while ((unsafe { *unsafe { z.offset((n - (1 as i32)) as isize) } }) as i32)
        == (48 as i32)
    {
        let __v891: i32 = n;
        let __v892: i32 = __v891 - (1 as i32);
        n = __v892;
        0 as i32;
    }
    unsafe {
        (*p).n = n;
    }
    unsafe {
        (*p).z = z;
    }
}

// /*
// ** Check for interrupts and invoke progress callback.
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3ProgressCheck(mut p: *mut Parse) {
    let mut db: *mut sqlite3 = unsafe { (*p).db };
    if (unsafe {
        std::sync::atomic::AtomicI32::load_volatile(
            std::sync::atomic::AtomicI32::from_ptr_raw(
                (unsafe { std::ptr::addr_of_mut!((*db).u1.isInterrupted) }) as *mut i32,
            ),
            std::sync::atomic::Ordering::Relaxed,
        )
    }) != (0 as i32)
    {
        let __v893: *mut Parse = p;
        let __v894: i32 = unsafe { (*__v893).nErr };
        let __v895: i32 = __v894 + (1 as i32);
        unsafe {
            (*__v893).nErr = __v895;
        }
        unsafe {
            (*p).rc = 9 as i32;
        }
    }
    if (unsafe { (*db).xProgress }) != None {
        if (unsafe { (*p).rc }) == (9 as i32) {
            unsafe {
                (*p).nProgressSteps = (0 as i32) as u32;
            }
        } else {
            let __v896: *mut Parse = p;
            let __v897: u32 = unsafe { (*__v896).nProgressSteps };
            let __v898: u32 = __v897.wrapping_add((1 as i32) as u32);
            unsafe {
                (*__v896).nProgressSteps = __v898;
            }
            if __v898 >= unsafe { (*db).nProgressOps } {
                if (unsafe { unsafe { (*db).xProgress }.unwrap()(unsafe { (*db).pProgressArg }) })
                    != (0 as i32)
                {
                    let __v899: *mut Parse = p;
                    let __v900: i32 = unsafe { (*__v899).nErr };
                    let __v901: i32 = __v900 + (1 as i32);
                    unsafe {
                        (*__v899).nErr = __v901;
                    }
                    unsafe {
                        (*p).rc = 9 as i32;
                    }
                }
                unsafe {
                    (*p).nProgressSteps = (0 as i32) as u32;
                }
            }
        }
    }
}

// /*
// ** Add an error message to pParse->zErrMsg and increment pParse->nErr.
// **
// ** This function should be used to report any error that occurs while
// ** compiling an SQL statement (i.e. within sqlite3_prepare()). The
// ** last thing the sqlite3_prepare() function does is copy the error
// ** stored by this function into the database handle using sqlite3Error().
// ** Functions sqlite3Error() or sqlite3ErrorWithMsg() should be used
// ** during statement execution (sqlite3_step() etc.).
// */
#[unsafe(no_mangle)]
unsafe extern "C-unwind" fn sqlite3ErrorMsg(
    mut pParse: *mut Parse,
    mut zFormat: *const i8,
    mut __va_args: ...
) {
    let mut zMsg: *mut i8 = unsafe { std::mem::zeroed() };
    let mut ap: core::ffi::VaList<'_> = unsafe { std::mem::zeroed() };
    let mut db: *mut sqlite3 = unsafe { (*pParse).db };
    0 as i32;
    0 as i32;
    unsafe {
        (*db).errByteOffset = -(2 as i32);
    }
    ap = __va_args.clone();
    zMsg = unsafe { sqlite3VMPrintf(db, zFormat, ap.clone()) };
    {}
    if (unsafe { (*db).errByteOffset }) < -(1 as i32) {
        unsafe {
            (*db).errByteOffset = -(1 as i32);
        }
    }
    if (unsafe { (*db).suppressErr }) != (0 as u8) {
        unsafe { sqlite3DbFree(db, zMsg as *mut ()) };
        if (unsafe { (*db).mallocFailed }) != (0 as u8) {
            let __v902: *mut Parse = pParse;
            let __v903: i32 = unsafe { (*__v902).nErr };
            let __v904: i32 = __v903 + (1 as i32);
            unsafe {
                (*__v902).nErr = __v904;
            }
            unsafe {
                (*pParse).rc = 7 as i32;
            }
        }
    } else {
        let __v905: *mut Parse = pParse;
        let __v906: i32 = unsafe { (*__v905).nErr };
        let __v907: i32 = __v906 + (1 as i32);
        unsafe {
            (*__v905).nErr = __v907;
        }
        unsafe { sqlite3DbFree(db, (unsafe { (*pParse).zErrMsg }) as *mut ()) };
        unsafe {
            (*pParse).zErrMsg = zMsg;
        }
        unsafe {
            (*pParse).rc = 1 as i32;
        }
        unsafe {
            (*pParse).pWith = std::ptr::null_mut::<With>();
        }
    }
}

// /*
// ** If database connection db is currently parsing SQL, then transfer
// ** error code errCode to that parser if the parser has not already
// ** encountered some other kind of error.
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3ErrorToParser(mut db: *mut sqlite3, mut errCode: i32) -> i32 {
    let mut pParse: *mut Parse = unsafe { std::mem::zeroed() };
    let __v908: bool;
    if db == std::ptr::null_mut::<sqlite3>() {
        __v908 = true as bool;
    } else {
        let __v909: *mut Parse = unsafe { (*db).pParse };
        pParse = __v909;
        __v908 = __v909 == std::ptr::null_mut::<Parse>();
    }
    if __v908 {
        return errCode;
    }
    unsafe {
        (*pParse).rc = errCode;
    }
    let __v910: *mut Parse = pParse;
    let __v911: i32 = unsafe { (*__v910).nErr };
    let __v912: i32 = __v911 + (1 as i32);
    unsafe {
        (*__v910).nErr = __v912;
    }
    return errCode;
}

// /*
// ** Convert an SQL-style quoted string into a normal string by removing
// ** the quote characters.  The conversion is done in-place.  If the
// ** input does not begin with a quote character, then this routine
// ** is a no-op.
// **
// ** The input string must be zero-terminated.  A new zero-terminator
// ** is added to the dequoted string.
// **
// ** The return value is -1 if no dequoting occurs or the length of the
// ** dequoted string, exclusive of the zero terminator, if dequoting does
// ** occur.
// **
// ** 2002-02-14: This routine is extended to remove MS-Access style
// ** brackets from around identifiers.  For example:  "[a-b-c]" becomes
// ** "a-b-c".
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3Dequote(mut z: *mut i8) {
    let mut quote: i8 = 0 as i8;
    let mut i: i32 = 0 as i32;
    let mut j: i32 = 0 as i32;
    if z == std::ptr::null_mut::<i8>() {
        return;
    }
    quote = unsafe { *unsafe { z.offset((0 as i32) as isize) } };
    if !((((unsafe {
        *unsafe {
            unsafe { std::ptr::addr_of!(sqlite3CtypeMap) as *const u8 }
                .offset((((quote as u8) as u32) as i32) as isize)
        }
    }) as u32) as i32)
        & (128 as i32)
        != (0 as i32))
    {
        return;
    }
    if (quote as i32) == (91 as i32) {
        quote = (93 as i32) as i8;
    }
    i = 1 as i32;
    j = 0 as i32;
    '__slate_break_786: loop {
        0 as i32;
        if ((unsafe { *unsafe { z.offset(i as isize) } }) as i32) == (quote as i32) {
            if ((unsafe { *unsafe { z.offset((i + (1 as i32)) as isize) } }) as i32)
                == (quote as i32)
            {
                let __v915: i32 = j;
                let __v916: i32 = __v915 + (1 as i32);
                j = __v916;
                unsafe {
                    *unsafe { z.offset(__v915 as isize) } = quote;
                }
                let __v917: i32 = i;
                let __v918: i32 = __v917 + (1 as i32);
                i = __v918;
            } else {
                break '__slate_break_786;
            }
        } else {
            let __v919: i32 = j;
            let __v920: i32 = __v919 + (1 as i32);
            j = __v920;
            unsafe {
                *unsafe { z.offset(__v919 as isize) } = unsafe { *unsafe { z.offset(i as isize) } };
            }
        }
        let __v913: i32 = i;
        let __v914: i32 = __v913 + (1 as i32);
        i = __v914;
    }
    unsafe {
        *unsafe { z.offset(j as isize) } = (0 as i32) as i8;
    }
}

#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3DequoteExpr(mut p: *mut Expr) {
    0 as i32;
    0 as i32;
    let __v921: *mut Expr = p;
    let __v922: u32 = unsafe { (*__v921).flags };
    let __v923: u32 = __v922
        | ((if ((unsafe { *unsafe { unsafe { (*p).u.zToken }.offset((0 as i32) as isize) } })
            as i32)
            == (34 as i32)
        {
            (67108864 as i32) | (128 as i32)
        } else {
            67108864 as i32
        }) as u32);
    unsafe {
        (*__v921).flags = __v923;
    }
    sqlite3Dequote(unsafe { (*p).u.zToken });
}

// /*
// ** If the input token p is quoted, try to adjust the token to remove
// ** the quotes.  This is not always possible:
// **
// **     "abc"     ->   abc
// **     "ab""cd"  ->   (not possible because of the interior "")
// **
// ** Remove the quotes if possible.  This is a optimization.  The overall
// ** system should still return the correct answer even if this routine
// ** is always a no-op.
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3DequoteToken(mut p: *mut Token) {
    let mut i: u32 = 0 as u32;
    if (unsafe { (*p).n }) < ((2 as i32) as u32) {
        return;
    }
    if !((((unsafe {
        *unsafe {
            unsafe { std::ptr::addr_of!(sqlite3CtypeMap) as *const u8 }.offset(
                ((((unsafe { *unsafe { unsafe { (*p).z }.offset((0 as i32) as isize) } }) as u8)
                    as u32) as i32) as isize,
            )
        }
    }) as u32) as i32)
        & (128 as i32)
        != (0 as i32))
    {
        return;
    }
    i = (1 as i32) as u32;
    '__slate_break_789: while i < unsafe { (*p).n }.wrapping_sub((1 as i32) as u32) {
        if (((unsafe {
            *unsafe {
                unsafe { std::ptr::addr_of!(sqlite3CtypeMap) as *const u8 }.offset(
                    ((((unsafe { *unsafe { unsafe { (*p).z }.offset(i as isize) } }) as u8) as u32)
                        as i32) as isize,
                )
            }
        }) as u32) as i32)
            & (128 as i32)
            != (0 as i32)
        {
            return;
        }
        let __v924: u32 = i;
        let __v925: u32 = __v924.wrapping_add((1 as i32) as u32);
        i = __v925;
    }
    let __v926: *mut Token = p;
    let __v927: u32 = unsafe { (*__v926).n };
    let __v928: u32 = __v927.wrapping_sub((2 as i32) as u32);
    unsafe {
        (*__v926).n = __v928;
    }
    let __v929: *mut Token = p;
    let __v930: *const i8 = unsafe { (*__v929).z };
    let __v931: *const i8 = unsafe { __v930.offset((1 as i32) as isize) };
    unsafe {
        (*__v929).z = __v931;
    }
}

// /*
// ** Expression p is a QNUMBER (quoted number). Dequote the value in p->u.zToken
// ** and set the type to INTEGER or FLOAT. "Quoted" integers or floats are those
// ** that contain '_' characters that must be removed before further processing.
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3DequoteNumber(mut pParse: *mut Parse, mut p: *mut Expr) {
    0 as i32;
    if p != std::ptr::null_mut::<Expr>() {
        let mut pIn: *const i8 = (unsafe { (*p).u.zToken }) as *const i8;
        let mut pOut: *mut i8 = unsafe { (*p).u.zToken };
        let mut bHex: i32 = (((unsafe { *unsafe { pIn.offset((0 as i32) as isize) } }) as i32)
            == (48 as i32)
            && (((unsafe { *unsafe { pIn.offset((1 as i32) as isize) } }) as i32) == (120 as i32)
                || ((unsafe { *unsafe { pIn.offset((1 as i32) as isize) } }) as i32)
                    == (88 as i32))) as i32;
        let mut iValue: i32 = 0 as i32;
        0 as i32;
        unsafe {
            (*p).op = ((156 as i32) as i8) as u8;
        }
        '__slate_break_787: loop {
            if ((unsafe { *pIn }) as i32) != (95 as i32) {
                let __v932: *mut i8 = pOut;
                let __v933: *mut i8 = unsafe { __v932.offset((1 as i32) as isize) };
                pOut = __v933;
                unsafe {
                    *__v932 = unsafe { *pIn };
                }
                if ((unsafe { *pIn }) as i32) == (101 as i32)
                    || ((unsafe { *pIn }) as i32) == (69 as i32)
                    || ((unsafe { *pIn }) as i32) == (46 as i32)
                {
                    unsafe {
                        (*p).op = ((154 as i32) as i8) as u8;
                    }
                }
            } else {
                if bHex == (0 as i32)
                    && (!((((unsafe {
                        *unsafe {
                            unsafe { std::ptr::addr_of!(sqlite3CtypeMap) as *const u8 }.offset(
                                ((((unsafe { *unsafe { pIn.offset(-(1 as i32) as isize) } }) as u8)
                                    as u32) as i32) as isize,
                            )
                        }
                    }) as u32) as i32)
                        & (4 as i32)
                        != (0 as i32))
                        || !((((unsafe {
                            *unsafe {
                                unsafe { std::ptr::addr_of!(sqlite3CtypeMap) as *const u8 }.offset(
                                    ((((unsafe { *unsafe { pIn.offset((1 as i32) as isize) } })
                                        as u8) as u32) as i32)
                                        as isize,
                                )
                            }
                        }) as u32) as i32)
                            & (4 as i32)
                            != (0 as i32)))
                    || bHex == (1 as i32)
                        && (!((((unsafe {
                            *unsafe {
                                unsafe { std::ptr::addr_of!(sqlite3CtypeMap) as *const u8 }.offset(
                                    ((((unsafe { *unsafe { pIn.offset(-(1 as i32) as isize) } })
                                        as u8) as u32) as i32)
                                        as isize,
                                )
                            }
                        }) as u32) as i32)
                            & (8 as i32)
                            != (0 as i32))
                            || !((((unsafe {
                                *unsafe {
                                    unsafe { std::ptr::addr_of!(sqlite3CtypeMap) as *const u8 }
                                        .offset(
                                            ((((unsafe {
                                                *unsafe { pIn.offset((1 as i32) as isize) }
                                            }) as u8)
                                                as u32)
                                                as i32)
                                                as isize,
                                        )
                                }
                            }) as u32) as i32)
                                & (8 as i32)
                                != (0 as i32)))
                {
                    unsafe {
                        sqlite3ErrorMsg(
                            pParse,
                            (b"unrecognized token: \"%s\"\0".as_ptr() as *mut i8) as *const i8,
                            unsafe { (*p).u.zToken },
                        )
                    };
                }
            }
            let __v934: *const i8 = pIn;
            let __v935: *const i8 = unsafe { __v934.offset((1 as i32) as isize) };
            pIn = __v935;
            if !((unsafe { *__v934 }) != (0 as i8)) {
                break;
            }
        }
        if bHex != (0 as i32) {
            unsafe {
                (*p).op = ((156 as i32) as i8) as u8;
            }
        }
        // /* tag-20240227-a: If after dequoting, the number is an integer that
        //     ** fits in 32 bits, then it must be converted into EP_IntValue.  Other
        //     ** parts of the code expect this.  See also tag-20240227-b. */
        let __v936: bool;
        if (((unsafe { (*p).op }) as u32) as i32) == (156 as i32) {
            __v936 = sqlite3GetInt32(
                (unsafe { (*p).u.zToken }) as *const i8,
                std::ptr::addr_of_mut!(iValue),
            ) != (0 as i32);
        } else {
            __v936 = false as bool;
        }
        if __v936 {
            unsafe {
                (*p).u.iValue = iValue;
            }
            let __v937: *mut Expr = p;
            let __v938: u32 = unsafe { (*__v937).flags };
            let __v939: u32 = __v938 | ((2048 as i32) as u32);
            unsafe {
                (*__v937).flags = __v939;
            }
        }
    }
}

// /*
// ** Generate a Token object from a string
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3TokenInit(mut p: *mut Token, mut z: *mut i8) {
    unsafe {
        (*p).z = z as *const i8;
    }
    unsafe {
        (*p).n = sqlite3Strlen30(z as *const i8) as u32;
    }
}

// /*
// ** 2001 September 15
// **
// ** The author disclaims copyright to this source code.  In place of
// ** a legal notice, here is a blessing:
// **
// **    May you do good and not evil.
// **    May you find forgiveness for yourself and forgive others.
// **    May you share freely, never taking more than you give.
// **
// *************************************************************************
// ** Utility functions used throughout sqlite.
// **
// ** This file contains functions for allocating memory, comparing
// ** strings, and stuff like that.
// **
// */
// /* Work around a bug in older Microsoft compilers
// ** Forum post 2026-04-10T06:33:11z */
// /* SQLITE_OMIT_FLOATING_POINT */
// /*
// ** Calls to sqlite3FaultSim() are used to simulate a failure during testing,
// ** or to bypass normal error detection during testing in order to let
// ** execute proceed further downstream.
// **
// ** In deployment, sqlite3FaultSim() *always* return SQLITE_OK (0).  The
// ** sqlite3FaultSim() function only returns non-zero during testing.
// **
// ** During testing, if the test harness has set a fault-sim callback using
// ** a call to sqlite3_test_control(SQLITE_TESTCTRL_FAULT_INSTALL), then
// ** each call to sqlite3FaultSim() is relayed to that application-supplied
// ** callback and the integer return value form the application-supplied
// ** callback is returned by sqlite3FaultSim().
// **
// ** The integer argument to sqlite3FaultSim() is a code to identify which
// ** sqlite3FaultSim() instance is being invoked. Each call to sqlite3FaultSim()
// ** should have a unique code.  To prevent legacy testing applications from
// ** breaking, the codes should not be changed or reused.
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3FaultSim(mut iTest: i32) -> i32 {
    let mut xCallback: Option<unsafe extern "C-unwind" fn(i32) -> i32> =
        unsafe { sqlite3Config.xTestCallback };
    let __v940: i32;
    if xCallback != None {
        __v940 = unsafe { xCallback.unwrap()(iTest) };
    } else {
        __v940 = 0 as i32;
    }
    return __v940;
}

// /*
// ** Check to make sure we have a valid db pointer.  This test is not
// ** foolproof but it does provide some measure of protection against
// ** misuse of the interface such as passing in db pointers that are
// ** NULL or which have been previously closed.  If this routine returns
// ** 1 it means that the db pointer is valid and 0 if it should not be
// ** dereferenced for any reason.  The calling function should invoke
// ** SQLITE_MISUSE immediately.
// **
// ** sqlite3SafetyCheckOk() requires that the db pointer be valid for
// ** use.  sqlite3SafetyCheckSickOrOk() allows a db pointer that failed to
// ** open properly and is not fit for general use but which can be
// ** used as an argument to sqlite3_errmsg() or sqlite3_close().
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3SafetyCheckOk(mut db: *mut sqlite3) -> i32 {
    let mut eOpenState: u8 = 0 as u8;
    if db == std::ptr::null_mut::<sqlite3>() {
        logBadConnection((b"NULL\0".as_ptr() as *mut i8) as *const i8);
        return 0 as i32;
    }
    eOpenState = unsafe { (*db).eOpenState };
    if ((eOpenState as u32) as i32) != (118 as i32) {
        if sqlite3SafetyCheckSickOrOk(db) != (0 as i32) {
            {}
            logBadConnection((b"unopened\0".as_ptr() as *mut i8) as *const i8);
        }
        return 0 as i32;
    } else {
        return 1 as i32;
    }
    return unsafe { std::mem::zeroed() };
}

#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3SafetyCheckSickOrOk(mut db: *mut sqlite3) -> i32 {
    let mut eOpenState: u8 = 0 as u8;
    eOpenState = unsafe { (*db).eOpenState };
    if ((eOpenState as u32) as i32) != (186 as i32)
        && ((eOpenState as u32) as i32) != (118 as i32)
        && ((eOpenState as u32) as i32) != (109 as i32)
    {
        {}
        logBadConnection((b"invalid\0".as_ptr() as *mut i8) as *const i8);
        return 0 as i32;
    } else {
        return 1 as i32;
    }
    return unsafe { std::mem::zeroed() };
}

// /*
// ** ARMv6, ARMv7, PPC32 are known to not support hardware u64 division.
// */
// /*
// ** Render an signed 64-bit integer as text.  Store the result in zOut[] and
// ** return the length of the string that was stored, in bytes.  The value
// ** returned does not include the zero terminator at the end of the output
// ** string.
// **
// ** The caller must ensure that zOut[] is at least 21 bytes in size.
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3Int64ToText(mut v: i64, mut zOut: *mut i8) -> i32 {
    let mut i: i32 = 0 as i32;
    let mut x_517: u64 = 0 as u64;
    let mut u: __SlateRecord202 = unsafe { std::mem::zeroed() };
    if v > ((0 as i32) as i64) {
        x_517 = v as u64;
    } else {
        if v == ((0 as i32) as i64) {
            unsafe {
                *unsafe { zOut.offset((0 as i32) as isize) } = (48 as i32) as i8;
            }
            unsafe {
                *unsafe { zOut.offset((1 as i32) as isize) } = (0 as i32) as i8;
            }
            return 1 as i32;
        } else {
            x_517 = if v
                == (-(1 as i32) as i64)
                    - ((((4294967295 as u32) as u64) as i64)
                        | ((2147483647 as i32) as i64) << (32 as i32))
            {
                (((1 as i32) as i64) as u64) << (63 as i32)
            } else {
                -v as u64
            };
        }
    }
    i = ((21 as u64).wrapping_sub(((1 as i32) as i64) as u64) as u32) as i32;
    unsafe {
        *unsafe { unsafe { u.a.as_mut_ptr() as *mut i8 }.offset(i as isize) } = (0 as i32) as i8;
    }
    '__slate_break_802: while x_517 >= (((10 as i32) as i64) as u64) {
        let mut kk: i32 = ((x_517 % (((100 as i32) as i64) as u64))
            .wrapping_mul(((2 as i32) as i64) as u64) as u32) as i32;
        0 as i32;
        0 as i32;
        unsafe {
            *((unsafe { unsafe { u.a.as_mut_ptr() as *mut i8 }.offset((i - (2 as i32)) as isize) })
                as *mut u16) = unsafe {
                *((unsafe {
                    unsafe { std::ptr::addr_of!(sqlite3DigitPairs.a) as *const i8 }
                        .offset(kk as isize)
                }) as *mut u16)
            };
        }
        let __v941: i32 = i;
        let __v942: i32 = __v941 - (2 as i32);
        i = __v942;
        let __v943: u64 = x_517;
        let __v944: u64 = __v943 / (((100 as i32) as i64) as u64);
        x_517 = __v944;
    }
    if x_517 != (0 as u64) {
        let __v945: i32 = i;
        let __v946: i32 = __v945 - (1 as i32);
        i = __v946;
        unsafe {
            *unsafe { unsafe { u.a.as_mut_ptr() as *mut i8 }.offset(__v946 as isize) } =
                (x_517.wrapping_add(((48 as i32) as i64) as u64) as u8) as i8;
        }
    }
    // /* SQLITE_AVOID_U64_DIVIDE */
    if v < ((0 as i32) as i64) {
        let __v947: i32 = i;
        let __v948: i32 = __v947 - (1 as i32);
        i = __v948;
        unsafe {
            *unsafe { unsafe { u.a.as_mut_ptr() as *mut i8 }.offset(__v948 as isize) } =
                (45 as i32) as i8;
        }
    }
    unsafe {
        memcpy(
            zOut as *mut (),
            (unsafe { unsafe { u.a.as_mut_ptr() as *mut i8 }.offset(i as isize) }) as *const (),
            (21 as u64).wrapping_sub((i as i64) as u64),
        )
    };
    return ((21 as u64)
        .wrapping_sub(((1 as i32) as i64) as u64)
        .wrapping_sub((i as i64) as u64) as u32) as i32;
}

// /*
// ** The string z[] is an text representation of a real number.
// ** Convert this string to a double and write it into *pResult.
// **
// ** z[] must be UTF-8 and zero-terminated.
// **
// ** Return positive if the result is a valid real number (or integer) and
// ** zero or negative if the string is empty or contains extraneous text.
// ** Lower bits of the return value contain addition information about the
// ** parse:
// **
// **   bit 0       =>   Set if any prefix of the input is valid.  Clear if
// **                    there is no prefix of the input that can be seen as
// **                    a valid floating point number.
// **   bit 1       =>   Set if the input contains a decimal point or eNNN
// **                    clause.  Zero if the input is an integer.
// **   bit 2       =>   The input is exactly 0.0, not an underflow from
// **                    some value near zero.
// **   bit 3       =>   Set if there are more than about 19 significant
// **                    digits in the input.
// **
// ** If the input contains a syntax error but begins with text that might
// ** be a valid number of some kind, then the result is negative.  The
// ** result is only zero if no prefix of the input could be interpreted as
// ** a number.
// **
// ** Leading and trailing whitespace is ignored.  Valid numbers are in
// ** one of the formats below:
// **
// **    [+-]digits[E[+-]digits]
// **    [+-]digits.[digits][E[+-]digits]
// **    [+-].digits[E[+-]digits]
// **
// ** Algorithm sketch:  Compute an unsigned 64-bit integer s and a base-10
// ** exponent d such that the value encoding by the input is s*pow(10,d).
// ** Then invoke sqlite3Fp10Convert2() to calculated the closest possible
// ** IEEE754 double.  The sign is added back afterwards, if the input string
// ** starts with a "-".  The use of an unsigned 64-bit s mantissa means that
// ** only about the first 19 significant digits of the input can contribute
// ** to the result.  This can result in suboptimal rounding decisions when
// ** correct rounding requires more than 19 input digits.  For example,
// ** this routine renders "3500000000000000.2500001" as
// ** 3500000000000000.0 instead of 3500000000000000.5 because the decision
// ** to round up instead of using banker's rounding to round down is determined
// ** by the 23rd significant digit, which this routine ignores. It is not
// ** possible to do better without some kind of BigNum.
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3AtoF(mut zIn: *const i8, mut pResult: *mut f64) -> i32 {
    let mut __slate_storage_998: std::mem::MaybeUninit<*const u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_998: *mut *const u8 =
        std::ptr::addr_of_mut!(__slate_storage_998) as *mut *const u8;
    let mut __slate_storage_997: std::mem::MaybeUninit<*const u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_997: *mut *const u8 =
        std::ptr::addr_of_mut!(__slate_storage_997) as *mut *const u8;
    let mut __slate_storage_996: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_996: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_996) as *mut i32;
    let mut __slate_storage_995: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_995: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_995) as *mut i32;
    let mut __slate_storage_992: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_992: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_992) as *mut i32;
    let mut __slate_storage_991: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_991: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_991) as *mut i32;
    let mut __slate_storage_988: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_988: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_988) as *mut u32;
    let mut __slate_storage_990: std::mem::MaybeUninit<*const u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_990: *mut *const u8 =
        std::ptr::addr_of_mut!(__slate_storage_990) as *mut *const u8;
    let mut __slate_storage_989: std::mem::MaybeUninit<*const u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_989: *mut *const u8 =
        std::ptr::addr_of_mut!(__slate_storage_989) as *mut *const u8;
    let mut __slate_storage_987: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_987: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_987) as *mut i32;
    let mut __slate_storage_986: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_986: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_986) as *mut i32;
    let mut __slate_storage_985: std::mem::MaybeUninit<*const u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_985: *mut *const u8 =
        std::ptr::addr_of_mut!(__slate_storage_985) as *mut *const u8;
    let mut __slate_storage_984: std::mem::MaybeUninit<*const u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_984: *mut *const u8 =
        std::ptr::addr_of_mut!(__slate_storage_984) as *mut *const u8;
    let mut __slate_storage_511: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_511: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_511) as *mut i32;
    let mut __slate_storage_994: std::mem::MaybeUninit<*const u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_994: *mut *const u8 =
        std::ptr::addr_of_mut!(__slate_storage_994) as *mut *const u8;
    let mut __slate_storage_993: std::mem::MaybeUninit<*const u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_993: *mut *const u8 =
        std::ptr::addr_of_mut!(__slate_storage_993) as *mut *const u8;
    let mut __slate_storage_983: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_983: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_983) as *mut u32;
    let mut __slate_storage_980: std::mem::MaybeUninit<*const u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_980: *mut *const u8 =
        std::ptr::addr_of_mut!(__slate_storage_980) as *mut *const u8;
    let mut __slate_storage_979: std::mem::MaybeUninit<*const u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_979: *mut *const u8 =
        std::ptr::addr_of_mut!(__slate_storage_979) as *mut *const u8;
    let mut __slate_storage_982: std::mem::MaybeUninit<*const u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_982: *mut *const u8 =
        std::ptr::addr_of_mut!(__slate_storage_982) as *mut *const u8;
    let mut __slate_storage_981: std::mem::MaybeUninit<*const u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_981: *mut *const u8 =
        std::ptr::addr_of_mut!(__slate_storage_981) as *mut *const u8;
    let mut __slate_storage_978: std::mem::MaybeUninit<*const u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_978: *mut *const u8 =
        std::ptr::addr_of_mut!(__slate_storage_978) as *mut *const u8;
    let mut __slate_storage_977: std::mem::MaybeUninit<*const u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_977: *mut *const u8 =
        std::ptr::addr_of_mut!(__slate_storage_977) as *mut *const u8;
    let mut __slate_storage_510: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_510: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_510) as *mut i32;
    let mut __slate_storage_976: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_976: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_976) as *mut i32;
    let mut __slate_storage_975: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_975: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_975) as *mut i32;
    let mut __slate_storage_974: std::mem::MaybeUninit<*const u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_974: *mut *const u8 =
        std::ptr::addr_of_mut!(__slate_storage_974) as *mut *const u8;
    let mut __slate_storage_973: std::mem::MaybeUninit<*const u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_973: *mut *const u8 =
        std::ptr::addr_of_mut!(__slate_storage_973) as *mut *const u8;
    let mut __slate_storage_972: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_972: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_972) as *mut i32;
    let mut __slate_storage_971: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_971: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_971) as *mut i32;
    let mut __slate_storage_970: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_970: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_970) as *mut i32;
    let mut __slate_storage_969: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_969: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_969) as *mut i32;
    let mut __slate_storage_968: std::mem::MaybeUninit<*const u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_968: *mut *const u8 =
        std::ptr::addr_of_mut!(__slate_storage_968) as *mut *const u8;
    let mut __slate_storage_967: std::mem::MaybeUninit<*const u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_967: *mut *const u8 =
        std::ptr::addr_of_mut!(__slate_storage_967) as *mut *const u8;
    let mut __slate_storage_952: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_952: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_952) as *mut u32;
    let mut __slate_storage_958: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_958: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_958) as *mut i32;
    let mut __slate_storage_957: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_957: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_957) as *mut i32;
    let mut __slate_storage_956: std::mem::MaybeUninit<*const u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_956: *mut *const u8 =
        std::ptr::addr_of_mut!(__slate_storage_956) as *mut *const u8;
    let mut __slate_storage_955: std::mem::MaybeUninit<*const u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_955: *mut *const u8 =
        std::ptr::addr_of_mut!(__slate_storage_955) as *mut *const u8;
    let mut __slate_storage_954: std::mem::MaybeUninit<*const u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_954: *mut *const u8 =
        std::ptr::addr_of_mut!(__slate_storage_954) as *mut *const u8;
    let mut __slate_storage_953: std::mem::MaybeUninit<*const u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_953: *mut *const u8 =
        std::ptr::addr_of_mut!(__slate_storage_953) as *mut *const u8;
    let mut __slate_storage_951: std::mem::MaybeUninit<*const u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_951: *mut *const u8 =
        std::ptr::addr_of_mut!(__slate_storage_951) as *mut *const u8;
    let mut __slate_storage_950: std::mem::MaybeUninit<*const u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_950: *mut *const u8 =
        std::ptr::addr_of_mut!(__slate_storage_950) as *mut *const u8;
    let mut __slate_storage_961: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_961: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_961) as *mut u32;
    let mut __slate_storage_960: std::mem::MaybeUninit<*const u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_960: *mut *const u8 =
        std::ptr::addr_of_mut!(__slate_storage_960) as *mut *const u8;
    let mut __slate_storage_959: std::mem::MaybeUninit<*const u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_959: *mut *const u8 =
        std::ptr::addr_of_mut!(__slate_storage_959) as *mut *const u8;
    let mut __slate_storage_964: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_964: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_964) as *mut u32;
    let mut __slate_storage_963: std::mem::MaybeUninit<*const u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_963: *mut *const u8 =
        std::ptr::addr_of_mut!(__slate_storage_963) as *mut *const u8;
    let mut __slate_storage_962: std::mem::MaybeUninit<*const u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_962: *mut *const u8 =
        std::ptr::addr_of_mut!(__slate_storage_962) as *mut *const u8;
    let mut __slate_storage_966: std::mem::MaybeUninit<*const u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_966: *mut *const u8 =
        std::ptr::addr_of_mut!(__slate_storage_966) as *mut *const u8;
    let mut __slate_storage_965: std::mem::MaybeUninit<*const u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_965: *mut *const u8 =
        std::ptr::addr_of_mut!(__slate_storage_965) as *mut *const u8;
    let mut __slate_storage_949: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_949: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_949) as *mut u32;
    let mut __slate_storage_509: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_509: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_509) as *mut u32;
    let mut __slate_storage_508: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_508: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_508) as *mut i32;
    let mut __slate_storage_507: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_507: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_507) as *mut i32;
    let mut __slate_storage_506: std::mem::MaybeUninit<u64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_506: *mut u64 = std::ptr::addr_of_mut!(__slate_storage_506) as *mut u64;
    let mut __slate_storage_505: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_505: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_505) as *mut i32;
    let mut __slate_storage_504: std::mem::MaybeUninit<*const u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_504: *mut *const u8 =
        std::ptr::addr_of_mut!(__slate_storage_504) as *mut *const u8;
    unsafe {
        std::ptr::write(__slate_slot_504, zIn as *const u8);
        // /* True for a negative value */
        std::ptr::write(__slate_slot_505, 0 as i32);
        // /* mantissa */
        std::ptr::write(__slate_slot_506, ((0 as i32) as i64) as u64);
        // /* Value is s * pow(10,d) */
        std::ptr::write(__slate_slot_507, 0 as i32);
        // /* 1: digit seen 2: fp 4: hard-zero */
        std::ptr::write(__slate_slot_508, 0 as i32);
        // /* Value of a single digit */
        '__join_34: {
            '__join_40: {
                '__join_41: {
                    '__join_42: {
                        '__loop_48: loop {
                            std::ptr::write(
                                __slate_slot_949,
                                ((unsafe {
                                    *unsafe { (*__slate_slot_504).offset((0 as i32) as isize) }
                                }) as u32)
                                    .wrapping_sub((48 as i32) as u32),
                            );
                            *__slate_slot_509 = *__slate_slot_949;
                            if *__slate_slot_949 < ((10 as i32) as u32) {
                                break '__join_40;
                            } else {
                                if (((unsafe {
                                    *unsafe { (*__slate_slot_504).offset((0 as i32) as isize) }
                                }) as u32) as i32)
                                    == (45 as i32)
                                {
                                    break '__join_41;
                                } else {
                                    if (((unsafe {
                                        *unsafe { (*__slate_slot_504).offset((0 as i32) as isize) }
                                    }) as u32) as i32)
                                        == (43 as i32)
                                    {
                                        break '__join_42;
                                    } else {
                                        if (((unsafe {
                                            *unsafe {
                                                unsafe {
                                                    std::ptr::addr_of!(sqlite3CtypeMap) as *const u8
                                                }
                                                .offset(
                                                    (((unsafe {
                                                        *unsafe {
                                                            (*__slate_slot_504)
                                                                .offset((0 as i32) as isize)
                                                        }
                                                    })
                                                        as u32)
                                                        as i32)
                                                        as isize,
                                                )
                                            }
                                        }) as u32)
                                            as i32)
                                            & (1 as i32)
                                            != (0 as i32)
                                        {
                                            loop {
                                                std::ptr::write(
                                                    __slate_slot_965,
                                                    *__slate_slot_504,
                                                );
                                                std::ptr::write(__slate_slot_966, unsafe {
                                                    (*__slate_slot_965).offset((1 as i32) as isize)
                                                });
                                                *__slate_slot_504 = *__slate_slot_966;
                                                if !((((unsafe {
                                                    *unsafe {
                                                        unsafe {
                                                            std::ptr::addr_of!(sqlite3CtypeMap)
                                                                as *const u8
                                                        }
                                                        .offset(
                                                            (((unsafe {
                                                                *unsafe {
                                                                    (*__slate_slot_504)
                                                                        .offset((0 as i32) as isize)
                                                                }
                                                            })
                                                                as u32)
                                                                as i32)
                                                                as isize,
                                                        )
                                                    }
                                                })
                                                    as u32)
                                                    as i32)
                                                    & (1 as i32)
                                                    != (0 as i32))
                                                {
                                                    continue '__loop_48;
                                                }
                                            }
                                        } else {
                                            break;
                                        }
                                    }
                                }
                            }
                        }
                        *__slate_slot_506 = ((0 as i32) as i64) as u64;
                        break '__join_34;
                    }
                    std::ptr::write(__slate_slot_962, *__slate_slot_504);
                    std::ptr::write(__slate_slot_963, unsafe {
                        (*__slate_slot_962).offset((1 as i32) as isize)
                    });
                    *__slate_slot_504 = *__slate_slot_963;
                    std::ptr::write(
                        __slate_slot_964,
                        ((unsafe { *unsafe { (*__slate_slot_504).offset((0 as i32) as isize) } })
                            as u32)
                            .wrapping_sub((48 as i32) as u32),
                    );
                    *__slate_slot_509 = *__slate_slot_964;
                    if *__slate_slot_964 < ((10 as i32) as u32) {
                        break '__join_40;
                    } else {
                        break '__join_34;
                    }
                }
                *__slate_slot_505 = 1 as i32;
                std::ptr::write(__slate_slot_959, *__slate_slot_504);
                std::ptr::write(__slate_slot_960, unsafe {
                    (*__slate_slot_959).offset((1 as i32) as isize)
                });
                *__slate_slot_504 = *__slate_slot_960;
                std::ptr::write(
                    __slate_slot_961,
                    ((unsafe { *unsafe { (*__slate_slot_504).offset((0 as i32) as isize) } })
                        as u32)
                        .wrapping_sub((48 as i32) as u32),
                );
                *__slate_slot_509 = *__slate_slot_961;
                if *__slate_slot_961 < ((10 as i32) as u32) {
                } else {
                    break '__join_34;
                }
            }
            *__slate_slot_508 = 1 as i32;
            *__slate_slot_506 = *__slate_slot_509 as u64;
            std::ptr::write(__slate_slot_950, *__slate_slot_504);
            std::ptr::write(__slate_slot_951, unsafe {
                (*__slate_slot_950).offset((1 as i32) as isize)
            });
            *__slate_slot_504 = *__slate_slot_951;
            loop {
                std::ptr::write(
                    __slate_slot_952,
                    ((unsafe { *unsafe { (*__slate_slot_504).offset((0 as i32) as isize) } })
                        as u32)
                        .wrapping_sub((48 as i32) as u32),
                );
                *__slate_slot_509 = *__slate_slot_952;
                if *__slate_slot_952 < ((10 as i32) as u32) {
                    *__slate_slot_506 = (*__slate_slot_506)
                        .wrapping_mul(((10 as i32) as i64) as u64)
                        .wrapping_add(*__slate_slot_509 as u64);
                    std::ptr::write(__slate_slot_953, *__slate_slot_504);
                    std::ptr::write(__slate_slot_954, unsafe {
                        (*__slate_slot_953).offset((1 as i32) as isize)
                    });
                    *__slate_slot_504 = *__slate_slot_954;
                    if *__slate_slot_506
                        >= (((4294967295 as u32) as u64)
                            | ((4294967295 as u32) as u64) << (32 as i32))
                            .wrapping_sub(((9 as i32) as i64) as u64)
                            / (((10 as i32) as i64) as u64)
                    {
                        break;
                    }
                } else {
                    break '__join_34;
                }
            }
            *__slate_slot_508 = 9 as i32;
            loop {
                if (((unsafe {
                    *unsafe {
                        unsafe { std::ptr::addr_of!(sqlite3CtypeMap) as *const u8 }.offset(
                            (((unsafe {
                                *unsafe { (*__slate_slot_504).offset((0 as i32) as isize) }
                            }) as u32) as i32) as isize,
                        )
                    }
                }) as u32) as i32)
                    & (4 as i32)
                    != (0 as i32)
                {
                    std::ptr::write(__slate_slot_955, *__slate_slot_504);
                    std::ptr::write(__slate_slot_956, unsafe {
                        (*__slate_slot_955).offset((1 as i32) as isize)
                    });
                    *__slate_slot_504 = *__slate_slot_956;
                    std::ptr::write(__slate_slot_957, *__slate_slot_507);
                    std::ptr::write(__slate_slot_958, *__slate_slot_957 + (1 as i32));
                    *__slate_slot_507 = *__slate_slot_958;
                } else {
                    break '__join_34;
                }
            }
        }
        // /* if decimal point is present */
        if (((unsafe { *(*__slate_slot_504) }) as u32) as i32) == (46 as i32) {
            '__join_23: {
                std::ptr::write(__slate_slot_967, *__slate_slot_504);
                std::ptr::write(__slate_slot_968, unsafe {
                    (*__slate_slot_967).offset((1 as i32) as isize)
                });
                *__slate_slot_504 = *__slate_slot_968;
                if (((unsafe {
                    *unsafe {
                        unsafe { std::ptr::addr_of!(sqlite3CtypeMap) as *const u8 }.offset(
                            (((unsafe {
                                *unsafe { (*__slate_slot_504).offset((0 as i32) as isize) }
                            }) as u32) as i32) as isize,
                        )
                    }
                }) as u32) as i32)
                    & (4 as i32)
                    != (0 as i32)
                {
                    std::ptr::write(__slate_slot_969, *__slate_slot_508);
                    std::ptr::write(__slate_slot_970, *__slate_slot_969 | (1 as i32));
                    *__slate_slot_508 = *__slate_slot_970;
                    loop {
                        if *__slate_slot_506
                            < (((4294967295 as u32) as u64)
                                | ((4294967295 as u32) as u64) << (32 as i32))
                                .wrapping_sub(((9 as i32) as i64) as u64)
                                / (((10 as i32) as i64) as u64)
                        {
                            *__slate_slot_506 = (*__slate_slot_506)
                                .wrapping_mul(((10 as i32) as i64) as u64)
                                .wrapping_add(
                                    ((((unsafe {
                                        *unsafe { (*__slate_slot_504).offset((0 as i32) as isize) }
                                    }) as u32) as i32) as i64)
                                        as u64,
                                )
                                .wrapping_sub(((48 as i32) as i64) as u64);
                            std::ptr::write(__slate_slot_971, *__slate_slot_507);
                            std::ptr::write(__slate_slot_972, *__slate_slot_971 - (1 as i32));
                            *__slate_slot_507 = *__slate_slot_972;
                        } else {
                            *__slate_slot_508 = 11 as i32;
                        }
                        std::ptr::write(__slate_slot_973, *__slate_slot_504);
                        std::ptr::write(__slate_slot_974, unsafe {
                            (*__slate_slot_973).offset((1 as i32) as isize)
                        });
                        *__slate_slot_504 = *__slate_slot_974;
                        if !((((unsafe {
                            *unsafe {
                                unsafe { std::ptr::addr_of!(sqlite3CtypeMap) as *const u8 }.offset(
                                    (((unsafe { *(*__slate_slot_974) }) as u32) as i32) as isize,
                                )
                            }
                        }) as u32) as i32)
                            & (4 as i32)
                            != (0 as i32))
                        {
                            break '__join_23;
                        }
                    }
                } else {
                    if *__slate_slot_508 == (0 as i32) {
                        unsafe {
                            *pResult = 0.0f64;
                        }
                        return 0 as i32;
                    }
                }
            }
            std::ptr::write(__slate_slot_975, *__slate_slot_508);
            std::ptr::write(__slate_slot_976, *__slate_slot_975 | (2 as i32));
            *__slate_slot_508 = *__slate_slot_976;
        } else {
            if *__slate_slot_508 == (0 as i32) {
                unsafe {
                    *pResult = 0.0f64;
                }
                return 0 as i32;
            }
        }
        // /* if exponent is present */
        if (((unsafe { *(*__slate_slot_504) }) as u32) as i32) == (101 as i32)
            || (((unsafe { *(*__slate_slot_504) }) as u32) as i32) == (69 as i32)
        {
            std::ptr::write(__slate_slot_977, *__slate_slot_504);
            std::ptr::write(__slate_slot_978, unsafe {
                (*__slate_slot_977).offset((1 as i32) as isize)
            });
            *__slate_slot_504 = *__slate_slot_978;
            // /* get sign of exponent */
            if (((unsafe { *(*__slate_slot_504) }) as u32) as i32) == (45 as i32) {
                *__slate_slot_510 = -(1 as i32);
                std::ptr::write(__slate_slot_979, *__slate_slot_504);
                std::ptr::write(__slate_slot_980, unsafe {
                    (*__slate_slot_979).offset((1 as i32) as isize)
                });
                *__slate_slot_504 = *__slate_slot_980;
            } else {
                *__slate_slot_510 = 1 as i32;
                if (((unsafe { *(*__slate_slot_504) }) as u32) as i32) == (43 as i32) {
                    std::ptr::write(__slate_slot_981, *__slate_slot_504);
                    std::ptr::write(__slate_slot_982, unsafe {
                        (*__slate_slot_981).offset((1 as i32) as isize)
                    });
                    *__slate_slot_504 = *__slate_slot_982;
                }
            }
            // /* copy digits to exponent */
            std::ptr::write(
                __slate_slot_983,
                ((unsafe { *unsafe { (*__slate_slot_504).offset((0 as i32) as isize) } }) as u32)
                    .wrapping_sub((48 as i32) as u32),
            );
            *__slate_slot_509 = *__slate_slot_983;
            if *__slate_slot_983 < ((10 as i32) as u32) {
                std::ptr::write(__slate_slot_511, *__slate_slot_509 as i32);
                std::ptr::write(__slate_slot_984, *__slate_slot_504);
                std::ptr::write(__slate_slot_985, unsafe {
                    (*__slate_slot_984).offset((1 as i32) as isize)
                });
                *__slate_slot_504 = *__slate_slot_985;
                std::ptr::write(__slate_slot_986, *__slate_slot_508);
                std::ptr::write(__slate_slot_987, *__slate_slot_986 | (2 as i32));
                *__slate_slot_508 = *__slate_slot_987;
                loop {
                    std::ptr::write(
                        __slate_slot_988,
                        ((unsafe { *unsafe { (*__slate_slot_504).offset((0 as i32) as isize) } })
                            as u32)
                            .wrapping_sub((48 as i32) as u32),
                    );
                    *__slate_slot_509 = *__slate_slot_988;
                    if *__slate_slot_988 < ((10 as i32) as u32) {
                        *__slate_slot_511 = (if *__slate_slot_511 < (10000 as i32) {
                            ((*__slate_slot_511 * (10 as i32)) as u32)
                                .wrapping_add(*__slate_slot_509)
                        } else {
                            (10000 as i32) as u32
                        }) as i32;
                        std::ptr::write(__slate_slot_989, *__slate_slot_504);
                        std::ptr::write(__slate_slot_990, unsafe {
                            (*__slate_slot_989).offset((1 as i32) as isize)
                        });
                        *__slate_slot_504 = *__slate_slot_990;
                    } else {
                        break;
                    }
                }
                std::ptr::write(__slate_slot_991, *__slate_slot_507);
                std::ptr::write(
                    __slate_slot_992,
                    *__slate_slot_991 + *__slate_slot_510 * *__slate_slot_511,
                );
                *__slate_slot_507 = *__slate_slot_992;
            } else {
                // /* Leave z[0] at 'e' or '+' or '-',
                //             ** so that the return is 0 or -1 */
                std::ptr::write(__slate_slot_993, *__slate_slot_504);
                std::ptr::write(__slate_slot_994, unsafe {
                    (*__slate_slot_993).offset(-((1 as i32) as isize))
                });
                *__slate_slot_504 = *__slate_slot_994;
            }
        }
        // /* Convert s*pow(10,d) into real */
        if *__slate_slot_506 == (((0 as i32) as i64) as u64) {
            unsafe {
                *pResult = 0.0f64;
            }
            std::ptr::write(__slate_slot_995, *__slate_slot_508);
            std::ptr::write(__slate_slot_996, *__slate_slot_995 | (4 as i32));
            *__slate_slot_508 = *__slate_slot_996;
        } else {
            unsafe {
                *pResult = sqlite3Fp10Convert2(*__slate_slot_506, *__slate_slot_507);
            }
        }
        if *__slate_slot_505 != (0 as i32) {
            unsafe {
                *pResult = -unsafe { *pResult };
            }
        }
        0 as i32;
        // /* return true if number and no extra non-whitespace characters after */
        if (((unsafe { *unsafe { (*__slate_slot_504).offset((0 as i32) as isize) } }) as u32)
            as i32)
            == (0 as i32)
        {
            return *__slate_slot_508;
        } else {
            if (((unsafe {
                *unsafe {
                    unsafe { std::ptr::addr_of!(sqlite3CtypeMap) as *const u8 }.offset(
                        (((unsafe { *unsafe { (*__slate_slot_504).offset((0 as i32) as isize) } })
                            as u32) as i32) as isize,
                    )
                }
            }) as u32) as i32)
                & (1 as i32)
                != (0 as i32)
            {
                loop {
                    std::ptr::write(__slate_slot_997, *__slate_slot_504);
                    std::ptr::write(__slate_slot_998, unsafe {
                        (*__slate_slot_997).offset((1 as i32) as isize)
                    });
                    *__slate_slot_504 = *__slate_slot_998;
                    if !((((unsafe {
                        *unsafe {
                            unsafe { std::ptr::addr_of!(sqlite3CtypeMap) as *const u8 }.offset(
                                (((unsafe { *(*__slate_slot_504) }) as u32) as i32) as isize,
                            )
                        }
                    }) as u32) as i32)
                        & (1 as i32)
                        != (0 as i32))
                    {
                        break;
                    }
                }
                if (((unsafe { *unsafe { (*__slate_slot_504).offset((0 as i32) as isize) } })
                    as u32) as i32)
                    == (0 as i32)
                {
                    return *__slate_slot_508;
                }
            }
            return ((4294967280 as u32) | (*__slate_slot_508 as u32)) as i32;
        }
    }
    return unsafe { std::mem::zeroed() };
}

// /*
// ** If zNum represents an integer that will fit in 32-bits, then set
// ** *pValue to that integer and return true.  Otherwise return false.
// **
// ** This routine accepts both decimal and hexadecimal notation for integers.
// **
// ** Any non-numeric characters that following zNum are ignored.
// ** This is different from sqlite3Atoi64() which requires the
// ** input number to be zero-terminated.
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3GetInt32(mut zNum: *const i8, mut pValue: *mut i32) -> i32 {
    let mut v: i64 = (0 as i32) as i64;
    let mut i: i32 = 0 as i32;
    let mut c: i32 = 0 as i32;
    let mut neg: i32 = 0 as i32;
    if ((unsafe { *unsafe { zNum.offset((0 as i32) as isize) } }) as i32) == (45 as i32) {
        neg = 1 as i32;
        let __v999: *const i8 = zNum;
        let __v1000: *const i8 = unsafe { __v999.offset((1 as i32) as isize) };
        zNum = __v1000;
    } else {
        if ((unsafe { *unsafe { zNum.offset((0 as i32) as isize) } }) as i32) == (43 as i32) {
            let __v1001: *const i8 = zNum;
            let __v1002: *const i8 = unsafe { __v1001.offset((1 as i32) as isize) };
            zNum = __v1002;
        } else {
            if ((unsafe { *unsafe { zNum.offset((0 as i32) as isize) } }) as i32) == (48 as i32)
                && (((unsafe { *unsafe { zNum.offset((1 as i32) as isize) } }) as i32)
                    == (120 as i32)
                    || ((unsafe { *unsafe { zNum.offset((1 as i32) as isize) } }) as i32)
                        == (88 as i32))
                && (((unsafe {
                    *unsafe {
                        unsafe { std::ptr::addr_of!(sqlite3CtypeMap) as *const u8 }.offset(
                            ((((unsafe { *unsafe { zNum.offset((2 as i32) as isize) } }) as u8)
                                as u32) as i32) as isize,
                        )
                    }
                }) as u32) as i32)
                    & (8 as i32)
                    != (0 as i32)
            {
                let mut u: u32 = (0 as i32) as u32;
                let __v1003: *const i8 = zNum;
                let __v1004: *const i8 = unsafe { __v1003.offset((2 as i32) as isize) };
                zNum = __v1004;
                '__slate_break_813: while ((unsafe { *unsafe { zNum.offset((0 as i32) as isize) } })
                    as i32)
                    == (48 as i32)
                {
                    let __v1005: *const i8 = zNum;
                    let __v1006: *const i8 = unsafe { __v1005.offset((1 as i32) as isize) };
                    zNum = __v1006;
                }
                i = 0 as i32;
                '__slate_break_814: loop {
                    if !(i < (8 as i32)
                        && (((unsafe {
                            *unsafe {
                                unsafe { std::ptr::addr_of!(sqlite3CtypeMap) as *const u8 }.offset(
                                    ((((unsafe { *unsafe { zNum.offset(i as isize) } }) as u8)
                                        as u32) as i32)
                                        as isize,
                                )
                            }
                        }) as u32) as i32)
                            & (8 as i32)
                            != (0 as i32))
                    {
                        break;
                    }
                    u = u.wrapping_mul((16 as i32) as u32).wrapping_add(
                        ((sqlite3HexToInt((unsafe { *unsafe { zNum.offset(i as isize) } }) as i32)
                            as u32) as i32) as u32,
                    );
                    let __v1007: i32 = i;
                    let __v1008: i32 = __v1007 + (1 as i32);
                    i = __v1008;
                }
                if u & (2147483648 as u32) == ((0 as i32) as u32)
                    && (((unsafe {
                        *unsafe {
                            unsafe { std::ptr::addr_of!(sqlite3CtypeMap) as *const u8 }.offset(
                                ((((unsafe { *unsafe { zNum.offset(i as isize) } }) as u8) as u32)
                                    as i32) as isize,
                            )
                        }
                    }) as u32) as i32)
                        & (8 as i32)
                        == (0 as i32)
                {
                    unsafe {
                        memcpy(
                            pValue as *mut (),
                            std::ptr::addr_of_mut!(u) as *const (),
                            ((4 as i32) as i64) as u64,
                        )
                    };
                    return 1 as i32;
                } else {
                    return 0 as i32;
                }
            }
        }
    }
    if !((((unsafe {
        *unsafe {
            unsafe { std::ptr::addr_of!(sqlite3CtypeMap) as *const u8 }.offset(
                ((((unsafe { *unsafe { zNum.offset((0 as i32) as isize) } }) as u8) as u32) as i32)
                    as isize,
            )
        }
    }) as u32) as i32)
        & (4 as i32)
        != (0 as i32))
    {
        return 0 as i32;
    }
    '__slate_break_815: while ((unsafe { *unsafe { zNum.offset((0 as i32) as isize) } }) as i32)
        == (48 as i32)
    {
        let __v1009: *const i8 = zNum;
        let __v1010: *const i8 = unsafe { __v1009.offset((1 as i32) as isize) };
        zNum = __v1010;
    }
    i = 0 as i32;
    '__slate_break_816: loop {
        let __v1011: bool;
        if i < (11 as i32) {
            let __v1012: i32 =
                ((unsafe { *unsafe { zNum.offset(i as isize) } }) as i32) - (48 as i32);
            c = __v1012;
            __v1011 = __v1012 >= (0 as i32);
        } else {
            __v1011 = false as bool;
        }
        if !(__v1011 && c <= (9 as i32)) {
            break;
        }
        v = v * ((10 as i32) as i64) + (c as i64);
        let __v1013: i32 = i;
        let __v1014: i32 = __v1013 + (1 as i32);
        i = __v1014;
    }
    // /* The longest decimal representation of a 32 bit integer is 10 digits:
    //   **
    //   **             1234567890
    //   **     2^31 -> 2147483648
    //   */
    {}
    if i > (10 as i32) {
        return 0 as i32;
    }
    {}
    if v - (neg as i64) > ((2147483647 as i32) as i64) {
        return 0 as i32;
    }
    if neg != (0 as i32) {
        v = -v;
    }
    unsafe {
        *pValue = v as i32;
    }
    return 1 as i32;
}

// /*
// ** Try to convert z into an unsigned 32-bit integer.  Return true on
// ** success and false if there is an error.
// **
// ** Only decimal notation is accepted.
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3GetUInt32(mut z: *const i8, mut pI: *mut u32) -> i32 {
    let mut v: u64 = ((0 as i32) as i64) as u64;
    let mut i: i32 = 0 as i32;
    i = 0 as i32;
    '__slate_break_825: loop {
        if !((((unsafe {
            *unsafe {
                unsafe { std::ptr::addr_of!(sqlite3CtypeMap) as *const u8 }.offset(
                    ((((unsafe { *unsafe { z.offset(i as isize) } }) as u8) as u32) as i32)
                        as isize,
                )
            }
        }) as u32) as i32)
            & (4 as i32)
            != (0 as i32))
        {
            break;
        }
        v = v
            .wrapping_mul(((10 as i32) as i64) as u64)
            .wrapping_add((((unsafe { *unsafe { z.offset(i as isize) } }) as i32) as i64) as u64)
            .wrapping_sub(((48 as i32) as i64) as u64);
        if v > ((4294967296 as i64) as u64) {
            unsafe {
                *pI = (0 as i32) as u32;
            }
            return 0 as i32;
        }
        let __v1015: i32 = i;
        let __v1016: i32 = __v1015 + (1 as i32);
        i = __v1016;
    }
    if i == (0 as i32) || ((unsafe { *unsafe { z.offset(i as isize) } }) as i32) != (0 as i32) {
        unsafe {
            *pI = (0 as i32) as u32;
        }
        return 0 as i32;
    }
    unsafe {
        *pI = v as u32;
    }
    return 1 as i32;
}

// /*
// ** Return a 32-bit integer value extracted from a string.  If the
// ** string is not an integer, just return 0.
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3Atoi(mut z: *const i8) -> i32 {
    let mut x_556: i32 = 0 as i32;
    sqlite3GetInt32(z, std::ptr::addr_of_mut!(x_556));
    return x_556;
}

// /*
// ** Convert an integer into a LogEst.  In other words, compute an
// ** approximation for 10*log2(x).
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3LogEst(mut x_633: u64) -> i16 {
    let mut y: i16 = (40 as i32) as i16;
    if x_633 < (((8 as i32) as i64) as u64) {
        if x_633 < (((2 as i32) as i64) as u64) {
            return (0 as i32) as i16;
        }
        '__slate_break_835: while x_633 < (((8 as i32) as i64) as u64) {
            let __v1017: i16 = y;
            let __v1018: i16 = ((__v1017 as i32) - (10 as i32)) as i16;
            y = __v1018;
            let __v1019: u64 = x_633;
            let __v1020: u64 = __v1019 << (1 as i32);
            x_633 = __v1020;
        }
    } else {
        // /*OPTIMIZATION-IF-TRUE*/
        '__slate_break_836: while x_633 > (((255 as i32) as i64) as u64) {
            let __v1021: i16 = y;
            let __v1022: i16 = ((__v1021 as i32) + (40 as i32)) as i16;
            y = __v1022;
            let __v1023: u64 = x_633;
            let __v1024: u64 = __v1023 >> (4 as i32);
            x_633 = __v1024;
        }
        '__slate_break_837: while x_633 > (((15 as i32) as i64) as u64) {
            let __v1025: i16 = y;
            let __v1026: i16 = ((__v1025 as i32) + (10 as i32)) as i16;
            y = __v1026;
            let __v1027: u64 = x_633;
            let __v1028: u64 = __v1027 >> (1 as i32);
            x_633 = __v1028;
        }
    }
    return (((unsafe {
        *unsafe {
            unsafe { std::ptr::addr_of_mut!(a.0) as *mut i16 }
                .offset((x_633 & (((7 as i32) as i64) as u64)) as isize)
        }
    }) as i32)
        + (y as i32)
        - (10 as i32)) as i16;
}

// /*
// ** Find (an approximate) sum of two LogEst values.  This computation is
// ** not a simple "+" operator because LogEst is stored as a logarithmic
// ** value.
// **
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3LogEstAdd(mut a_630: i16, mut b: i16) -> i16 {
    // /* 0,1 */
    // /* 2,3 */
    // /* 4,5 */
    // /* 6,7,8 */
    // /* 9,10,11 */
    // /* 12-14 */
    // /* 15-18 */
    // /* 19-24 */
    // /* 25-31 */
    if (a_630 as i32) >= (b as i32) {
        if (a_630 as i32) > (b as i32) + (49 as i32) {
            return a_630;
        }
        if (a_630 as i32) > (b as i32) + (31 as i32) {
            return ((a_630 as i32) + (1 as i32)) as i16;
        }
        return ((a_630 as i32)
            + (((unsafe {
                *unsafe {
                    unsafe { std::ptr::addr_of!(x.0) as *const u8 }
                        .offset(((a_630 as i32) - (b as i32)) as isize)
                }
            }) as u32) as i32)) as i16;
    } else {
        if (b as i32) > (a_630 as i32) + (49 as i32) {
            return b;
        }
        if (b as i32) > (a_630 as i32) + (31 as i32) {
            return ((b as i32) + (1 as i32)) as i16;
        }
        return ((b as i32)
            + (((unsafe {
                *unsafe {
                    unsafe { std::ptr::addr_of!(x.0) as *const u8 }
                        .offset(((b as i32) - (a_630 as i32)) as isize)
                }
            }) as u32) as i32)) as i16;
    }
    return unsafe { std::mem::zeroed() };
}

// /*
// ** Convert a double into a LogEst
// ** In other words, compute an approximation for 10*log2(x).
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3LogEstFromDouble(mut x_636: f64) -> i16 {
    let mut a_637: u64 = 0 as u64;
    let mut e: i16 = 0 as i16;
    0 as i32;
    if x_636 <= ((1 as i32) as f64) {
        return (0 as i32) as i16;
    }
    if x_636 <= ((2000000000 as i32) as f64) {
        return sqlite3LogEst(x_636 as u64);
    }
    unsafe {
        memcpy(
            std::ptr::addr_of_mut!(a_637) as *mut (),
            std::ptr::addr_of_mut!(x_636) as *const (),
            ((8 as i32) as i64) as u64,
        )
    };
    e = ((a_637 >> (52 as i32)).wrapping_sub(((1022 as i32) as i64) as u64) as u16) as i16;
    return ((e as i32) * (10 as i32)) as i16;
}

// /*
// ** Convert a LogEst into an integer.
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3LogEstToInt(mut x_639: i16) -> u64 {
    let mut n: u64 = 0 as u64;
    n = (((x_639 as i32) % (10 as i32)) as i64) as u64;
    let __v1029: i16 = x_639;
    let __v1030: i16 = ((__v1029 as i32) / (10 as i32)) as i16;
    x_639 = __v1030;
    if n >= (((5 as i32) as i64) as u64) {
        let __v1031: u64 = n;
        let __v1032: u64 = __v1031.wrapping_sub(((2 as i32) as i64) as u64);
        n = __v1032;
    } else {
        if n >= (((1 as i32) as i64) as u64) {
            let __v1033: u64 = n;
            let __v1034: u64 = __v1033.wrapping_sub(((1 as i32) as i64) as u64);
            n = __v1034;
        }
    }
    if (x_639 as i32) > (60 as i32) {
        return ((((4294967295 as u32) as u64) as i64) | ((2147483647 as i32) as i64) << (32 as i32))
            as u64;
    }
    return if (x_639 as i32) >= (3 as i32) {
        n.wrapping_add(((8 as i32) as i64) as u64) << (x_639 as i32) - (3 as i32)
    } else {
        n.wrapping_add(((8 as i32) as i64) as u64) >> (3 as i32) - (x_639 as i32)
    };
}

// /*
// ** Add a new name/number pair to a VList.  This might require that the
// ** VList object be reallocated, so return the new VList.  If an OOM
// ** error occurs, the original VList returned and the
// ** db->mallocFailed flag is set.
// **
// ** A VList is really just an array of integers.  To destroy a VList,
// ** simply pass it to sqlite3DbFree().
// **
// ** The first integer is the number of integers allocated for the whole
// ** VList.  The second integer is the number of integers actually used.
// ** Each name/number pair is encoded by subsequent groups of 3 or more
// ** integers.
// **
// ** Each name/number pair starts with two integers which are the numeric
// ** value for the pair and the size of the name/number pair, respectively.
// ** The text name overlays one or more following integers.  The text name
// ** is always zero-terminated.
// **
// ** Conceptually:
// **
// **    struct VList {
// **      int nAlloc;   // Number of allocated slots
// **      int nUsed;    // Number of used slots
// **      struct VListEntry {
// **        int iValue;    // Value for this entry
// **        int nSlot;     // Slots used by this entry
// **        // ... variable name goes here
// **      } a[0];
// **    }
// **
// ** During code generation, pointers to the variable names within the
// ** VList are taken.  When that happens, nAlloc is set to zero as an
// ** indication that the VList may never again be enlarged, since the
// ** accompanying realloc() would invalidate the pointers.
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3VListAdd(
    mut db: *mut sqlite3,
    mut pIn: *mut i32,
    mut zName: *const i8,
    mut nName: i32,
    mut iVal: i32,
) -> *mut i32 {
    // /* number of sizeof(int) objects needed for zName */
    let mut nInt: i32 = 0 as i32;
    // /* Pointer to where zName will be stored */
    let mut z: *mut i8 = unsafe { std::mem::zeroed() };
    // /* Index in pIn[] where zName is stored */
    let mut i: i32 = 0 as i32;
    nInt = nName / (4 as i32) + (3 as i32);
    // /* Verify ok to add new elements */
    0 as i32;
    if pIn == std::ptr::null_mut::<i32>()
        || (unsafe { *unsafe { pIn.offset((1 as i32) as isize) } }) + nInt
            > unsafe { *unsafe { pIn.offset((0 as i32) as isize) } }
    {
        // /* Enlarge the allocation */
        let mut nAlloc: i64 = (if pIn != std::ptr::null_mut::<i32>() {
            ((2 as i32) as i64) * ((unsafe { *unsafe { pIn.offset((0 as i32) as isize) } }) as i64)
        } else {
            (10 as i32) as i64
        }) + (nInt as i64);
        let mut pOut: *mut i32 = (unsafe {
            sqlite3DbRealloc(db, pIn as *mut (), (nAlloc as u64).wrapping_mul(4 as u64))
        }) as *mut i32;
        if pOut == std::ptr::null_mut::<i32>() {
            return pIn;
        }
        if pIn == std::ptr::null_mut::<i32>() {
            unsafe {
                *unsafe { pOut.offset((1 as i32) as isize) } = 2 as i32;
            }
        }
        pIn = pOut;
        unsafe {
            *unsafe { pIn.offset((0 as i32) as isize) } = nAlloc as i32;
        }
    }
    i = unsafe { *unsafe { pIn.offset((1 as i32) as isize) } };
    unsafe {
        *unsafe { pIn.offset(i as isize) } = iVal;
    }
    unsafe {
        *unsafe { pIn.offset((i + (1 as i32)) as isize) } = nInt;
    }
    z = (unsafe { pIn.offset((i + (2 as i32)) as isize) }) as *mut i8;
    unsafe {
        *unsafe { pIn.offset((1 as i32) as isize) } = i + nInt;
    }
    0 as i32;
    unsafe { memcpy(z as *mut (), zName as *const (), (nName as i64) as u64) };
    unsafe {
        *unsafe { z.offset(nName as isize) } = (0 as i32) as i8;
    }
    return pIn;
}

// /* The database connection used for malloc() */
// /* The input VList.  Might be NULL */
// /* Name of symbol to add */
// /* Bytes of text in zName */
// /* Value to associate with zName */
// /*
// ** Return a pointer to the name of a variable in the given VList that
// ** has the value iVal.  Or return a NULL if there is no such variable in
// ** the list
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3VListNumToName(mut pIn: *mut i32, mut iVal: i32) -> *const i8 {
    let mut i: i32 = 0 as i32;
    let mut mx: i32 = 0 as i32;
    if pIn == std::ptr::null_mut::<i32>() {
        return std::ptr::null::<i8>();
    }
    mx = unsafe { *unsafe { pIn.offset((1 as i32) as isize) } };
    i = 2 as i32;
    '__slate_break_838: loop {
        if (unsafe { *unsafe { pIn.offset(i as isize) } }) == iVal {
            return ((unsafe { pIn.offset((i + (2 as i32)) as isize) }) as *mut i8) as *const i8;
        }
        let __v1035: i32 = i;
        let __v1036: i32 = __v1035 + unsafe { *unsafe { pIn.offset((i + (1 as i32)) as isize) } };
        i = __v1036;
        if !(i < mx) {
            break;
        }
    }
    return std::ptr::null::<i8>();
}

// /*
// ** Return the number of the variable named zName, if it is in VList.
// ** or return 0 if there is no such variable.
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3VListNameToNum(
    mut pIn: *mut i32,
    mut zName: *const i8,
    mut nName: i32,
) -> i32 {
    let mut i: i32 = 0 as i32;
    let mut mx: i32 = 0 as i32;
    if pIn == std::ptr::null_mut::<i32>() {
        return 0 as i32;
    }
    mx = unsafe { *unsafe { pIn.offset((1 as i32) as isize) } };
    i = 2 as i32;
    '__slate_break_839: loop {
        let mut z: *const i8 = (unsafe { pIn.offset((i + (2 as i32)) as isize) }) as *const i8;
        if (unsafe { strncmp(z, zName, (nName as i64) as u64) }) == (0 as i32)
            && ((unsafe { *unsafe { z.offset(nName as isize) } }) as i32) == (0 as i32)
        {
            return unsafe { *unsafe { pIn.offset(i as isize) } };
        }
        let __v1037: i32 = i;
        let __v1038: i32 = __v1037 + unsafe { *unsafe { pIn.offset((i + (1 as i32)) as isize) } };
        i = __v1038;
        if !(i < mx) {
            break;
        }
    }
    return 0 as i32;
}

#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3PutVarint(mut p: *mut u8, mut v: u64) -> i32 {
    if v <= (((127 as i32) as i64) as u64) {
        unsafe {
            *unsafe { p.offset((0 as i32) as isize) } = (v & (((127 as i32) as i64) as u64)) as u8;
        }
        return 1 as i32;
    }
    if v <= (((16383 as i32) as i64) as u64) {
        unsafe {
            *unsafe { p.offset((0 as i32) as isize) } =
                (v >> (7 as i32) & (((127 as i32) as i64) as u64) | (((128 as i32) as i64) as u64))
                    as u8;
        }
        unsafe {
            *unsafe { p.offset((1 as i32) as isize) } = (v & (((127 as i32) as i64) as u64)) as u8;
        }
        return 2 as i32;
    }
    return putVarint64(p, v);
}

// /*
// ** Read a 64-bit variable-length integer from memory starting at p[0].
// ** Return the number of bytes read.  The value is stored in *v.
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3GetVarint(mut p: *const u8, mut v: *mut u64) -> u8 {
    let mut iKey: u64 = (unsafe { *unsafe { p.offset((0 as i32) as isize) } }) as u64;
    let mut pStart: *const u8 = p;
    if iKey >= (((128 as i32) as i64) as u64) {
        let mut x_594: u8 = 0 as u8;
        let __v1039: *const u8 = p;
        let __v1040: *const u8 = unsafe { __v1039.offset((1 as i32) as isize) };
        p = __v1040;
        let __v1041: u8 = unsafe { *__v1040 };
        x_594 = __v1041;
        iKey = iKey << (7 as i32) ^ ((((__v1041 as u32) as i32) as i64) as u64);
        if ((x_594 as u32) as i32) >= (128 as i32) {
            let __v1042: *const u8 = p;
            let __v1043: *const u8 = unsafe { __v1042.offset((1 as i32) as isize) };
            p = __v1043;
            let __v1044: u8 = unsafe { *__v1043 };
            x_594 = __v1044;
            iKey = iKey << (7 as i32) ^ ((((__v1044 as u32) as i32) as i64) as u64);
            if ((x_594 as u32) as i32) >= (128 as i32) {
                let __v1045: *const u8 = p;
                let __v1046: *const u8 = unsafe { __v1045.offset((1 as i32) as isize) };
                p = __v1046;
                let __v1047: u8 = unsafe { *__v1046 };
                x_594 = __v1047;
                iKey = iKey << (7 as i32)
                    ^ (((270548992 as i32) as i64) as u64)
                    ^ ((((__v1047 as u32) as i32) as i64) as u64);
                if ((x_594 as u32) as i32) >= (128 as i32) {
                    let __v1048: *const u8 = p;
                    let __v1049: *const u8 = unsafe { __v1048.offset((1 as i32) as isize) };
                    p = __v1049;
                    let __v1050: u8 = unsafe { *__v1049 };
                    x_594 = __v1050;
                    iKey = iKey << (7 as i32)
                        ^ (((16384 as i32) as i64) as u64)
                        ^ ((((__v1050 as u32) as i32) as i64) as u64);
                    if ((x_594 as u32) as i32) >= (128 as i32) {
                        let __v1051: *const u8 = p;
                        let __v1052: *const u8 = unsafe { __v1051.offset((1 as i32) as isize) };
                        p = __v1052;
                        let __v1053: u8 = unsafe { *__v1052 };
                        x_594 = __v1053;
                        iKey = iKey << (7 as i32)
                            ^ (((16384 as i32) as i64) as u64)
                            ^ ((((__v1053 as u32) as i32) as i64) as u64);
                        if ((x_594 as u32) as i32) >= (128 as i32) {
                            let __v1054: *const u8 = p;
                            let __v1055: *const u8 = unsafe { __v1054.offset((1 as i32) as isize) };
                            p = __v1055;
                            let __v1056: u8 = unsafe { *__v1055 };
                            x_594 = __v1056;
                            iKey = iKey << (7 as i32)
                                ^ (((16384 as i32) as i64) as u64)
                                ^ ((((__v1056 as u32) as i32) as i64) as u64);
                            if ((x_594 as u32) as i32) >= (128 as i32) {
                                let __v1057: *const u8 = p;
                                let __v1058: *const u8 =
                                    unsafe { __v1057.offset((1 as i32) as isize) };
                                p = __v1058;
                                let __v1059: u8 = unsafe { *__v1058 };
                                x_594 = __v1059;
                                iKey = iKey << (7 as i32)
                                    ^ (((16384 as i32) as i64) as u64)
                                    ^ ((((__v1059 as u32) as i32) as i64) as u64);
                                if ((x_594 as u32) as i32) >= (128 as i32) {
                                    let __v1060: *const u8 = p;
                                    let __v1061: *const u8 =
                                        unsafe { __v1060.offset((1 as i32) as isize) };
                                    p = __v1061;
                                    iKey = iKey << (8 as i32)
                                        ^ (((32768 as i32) as i64) as u64)
                                        ^ (((((unsafe { *__v1061 }) as u32) as i32) as i64) as u64);
                                }
                            }
                        }
                    }
                }
            } else {
                let __v1062: u64 = iKey;
                let __v1063: u64 = __v1062 ^ (((2113536 as i32) as i64) as u64);
                iKey = __v1063;
            }
        } else {
            let __v1064: u64 = iKey;
            let __v1065: u64 = __v1064 ^ (((16384 as i32) as i64) as u64);
            iKey = __v1065;
        }
    }
    unsafe {
        *v = iKey;
    }
    return ((((((((unsafe { p.offset_from(pStart as *const u8) }) as i64) as i8) as u8) as u32)
        as i32)
        + (1 as i32)) as i8) as u8;
}

// /*
// ** Return the value of a variable-length integer without computing
// ** its length.  This is an optimization on sqlite3GetVarint() for the
// ** cases when the return value of sqlite3GetVarint() is not needed.
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3VarintValue(mut p: *const u8) -> i64 {
    let mut iKey: u64 = (unsafe { *unsafe { p.offset((0 as i32) as isize) } }) as u64;
    if iKey >= (((128 as i32) as i64) as u64) {
        let mut x_597: u8 = 0 as u8;
        let __v1066: *const u8 = p;
        let __v1067: *const u8 = unsafe { __v1066.offset((1 as i32) as isize) };
        p = __v1067;
        let __v1068: u8 = unsafe { *__v1067 };
        x_597 = __v1068;
        iKey = iKey << (7 as i32) ^ ((((__v1068 as u32) as i32) as i64) as u64);
        if ((x_597 as u32) as i32) >= (128 as i32) {
            let __v1069: *const u8 = p;
            let __v1070: *const u8 = unsafe { __v1069.offset((1 as i32) as isize) };
            p = __v1070;
            let __v1071: u8 = unsafe { *__v1070 };
            x_597 = __v1071;
            iKey = iKey << (7 as i32) ^ ((((__v1071 as u32) as i32) as i64) as u64);
            if ((x_597 as u32) as i32) >= (128 as i32) {
                let __v1072: *const u8 = p;
                let __v1073: *const u8 = unsafe { __v1072.offset((1 as i32) as isize) };
                p = __v1073;
                let __v1074: u8 = unsafe { *__v1073 };
                x_597 = __v1074;
                iKey = iKey << (7 as i32)
                    ^ (((270548992 as i32) as i64) as u64)
                    ^ ((((__v1074 as u32) as i32) as i64) as u64);
                if ((x_597 as u32) as i32) >= (128 as i32) {
                    let __v1075: *const u8 = p;
                    let __v1076: *const u8 = unsafe { __v1075.offset((1 as i32) as isize) };
                    p = __v1076;
                    let __v1077: u8 = unsafe { *__v1076 };
                    x_597 = __v1077;
                    iKey = iKey << (7 as i32)
                        ^ (((16384 as i32) as i64) as u64)
                        ^ ((((__v1077 as u32) as i32) as i64) as u64);
                    if ((x_597 as u32) as i32) >= (128 as i32) {
                        let __v1078: *const u8 = p;
                        let __v1079: *const u8 = unsafe { __v1078.offset((1 as i32) as isize) };
                        p = __v1079;
                        let __v1080: u8 = unsafe { *__v1079 };
                        x_597 = __v1080;
                        iKey = iKey << (7 as i32)
                            ^ (((16384 as i32) as i64) as u64)
                            ^ ((((__v1080 as u32) as i32) as i64) as u64);
                        if ((x_597 as u32) as i32) >= (128 as i32) {
                            let __v1081: *const u8 = p;
                            let __v1082: *const u8 = unsafe { __v1081.offset((1 as i32) as isize) };
                            p = __v1082;
                            let __v1083: u8 = unsafe { *__v1082 };
                            x_597 = __v1083;
                            iKey = iKey << (7 as i32)
                                ^ (((16384 as i32) as i64) as u64)
                                ^ ((((__v1083 as u32) as i32) as i64) as u64);
                            if ((x_597 as u32) as i32) >= (128 as i32) {
                                let __v1084: *const u8 = p;
                                let __v1085: *const u8 =
                                    unsafe { __v1084.offset((1 as i32) as isize) };
                                p = __v1085;
                                let __v1086: u8 = unsafe { *__v1085 };
                                x_597 = __v1086;
                                iKey = iKey << (7 as i32)
                                    ^ (((16384 as i32) as i64) as u64)
                                    ^ ((((__v1086 as u32) as i32) as i64) as u64);
                                if ((x_597 as u32) as i32) >= (128 as i32) {
                                    let __v1087: *const u8 = p;
                                    let __v1088: *const u8 =
                                        unsafe { __v1087.offset((1 as i32) as isize) };
                                    p = __v1088;
                                    iKey = iKey << (8 as i32)
                                        ^ (((32768 as i32) as i64) as u64)
                                        ^ (((((unsafe { *__v1088 }) as u32) as i32) as i64) as u64);
                                }
                            }
                        }
                    }
                }
            } else {
                let __v1089: u64 = iKey;
                let __v1090: u64 = __v1089 ^ (((2113536 as i32) as i64) as u64);
                iKey = __v1090;
            }
        } else {
            let __v1091: u64 = iKey;
            let __v1092: u64 = __v1091 ^ (((16384 as i32) as i64) as u64);
            iKey = __v1092;
        }
    }
    return unsafe { *(std::ptr::addr_of_mut!(iKey) as *mut i64) };
}

// /*
// ** Read a 32-bit variable-length integer from memory starting at p[0].
// ** Return the number of bytes read.  The value is stored in *v.
// **
// ** If the varint stored in p[0] is larger than can fit in a 32-bit unsigned
// ** integer, then set *v to 0xffffffff.
// **
// ** A MACRO version, getVarint32, is provided which inlines the
// ** single-byte case.  All code should use the MACRO version as
// ** this function assumes the single-byte case has already been handled.
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3GetVarint32(mut p: *const u8, mut v: *mut u32) -> u8 {
    let mut v64: u64 = 0 as u64;
    let mut n: u8 = 0 as u8;
    // /* Assume that the single-byte case has already been handled by
    //   ** the getVarint32() macro */
    0 as i32;
    if (((unsafe { *unsafe { p.offset((1 as i32) as isize) } }) as u32) as i32) & (128 as i32)
        == (0 as i32)
    {
        // /* This is the two-byte case */
        unsafe {
            *v = (((((unsafe { *unsafe { p.offset((0 as i32) as isize) } }) as u32) as i32)
                & (127 as i32))
                << (7 as i32)
                | (((unsafe { *unsafe { p.offset((1 as i32) as isize) } }) as u32) as i32))
                as u32;
        }
        return ((2 as i32) as i8) as u8;
    }
    if (((unsafe { *unsafe { p.offset((2 as i32) as isize) } }) as u32) as i32) & (128 as i32)
        == (0 as i32)
    {
        // /* This is the three-byte case */
        unsafe {
            *v = (((((unsafe { *unsafe { p.offset((0 as i32) as isize) } }) as u32) as i32)
                & (127 as i32))
                << (14 as i32)
                | ((((unsafe { *unsafe { p.offset((1 as i32) as isize) } }) as u32) as i32)
                    & (127 as i32))
                    << (7 as i32)
                | (((unsafe { *unsafe { p.offset((2 as i32) as isize) } }) as u32) as i32))
                as u32;
        }
        return ((3 as i32) as i8) as u8;
    }
    // /* four or more bytes */
    n = sqlite3GetVarint(p, std::ptr::addr_of_mut!(v64));
    0 as i32;
    if v64 & ((((1 as i32) as i64) as u64) << (32 as i32)).wrapping_sub(((1 as i32) as i64) as u64)
        != v64
    {
        unsafe {
            *v = 4294967295 as u32;
        }
    } else {
        unsafe {
            *v = v64 as u32;
        }
    }
    return n;
}

// /*
// ** Return the number of bytes that will be needed to store the given
// ** 64-bit integer.
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3VarintLen(mut v: u64) -> i32 {
    let mut i: i32 = 0 as i32;
    i = 1 as i32;
    '__slate_break_829: loop {
        let __v1093: u64 = v;
        let __v1094: u64 = __v1093 >> (7 as i32);
        v = __v1094;
        if !(__v1094 != (((0 as i32) as i64) as u64)) {
            break;
        }
        0 as i32;
        let __v1095: i32 = i;
        let __v1096: i32 = __v1095 + (1 as i32);
        i = __v1096;
    }
    return i;
}

// /*
// ** Convert zNum to a 64-bit signed integer.  zNum must be decimal. This
// ** routine does *not* accept hexadecimal notation.
// **
// ** Returns:
// **
// **    -1    Not even a prefix of the input text looks like an integer
// **     0    Successful transformation.  Fits in a 64-bit signed integer.
// **     1    Excess non-space text after the integer value
// **     2    Integer too large for a 64-bit signed integer or is malformed
// **     3    Special case of 9223372036854775808
// **
// ** length is the number of bytes in the string (bytes, not characters).
// ** The string is not necessarily zero-terminated.  The encoding is
// ** given by enc.
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3Atoi64(
    mut zNum: *const i8,
    mut pNum: *mut i64,
    mut length: i32,
    mut enc: u8,
) -> i32 {
    let mut incr: i32 = 0 as i32;
    let mut u: u64 = ((0 as i32) as i64) as u64;
    // /* assume positive */
    let mut neg: i32 = 0 as i32;
    let mut i: i32 = 0 as i32;
    let mut j: i32 = 0 as i32;
    let mut c: u32 = (0 as i32) as u32;
    // /* True if input contains UTF16 with high byte non-zero */
    let mut nonNum: i32 = 0 as i32;
    // /* Baseline return code */
    let mut rc: i32 = 0 as i32;
    let mut zStart: *const i8 = unsafe { std::mem::zeroed() };
    let mut zEnd: *const i8 = unsafe { zNum.offset(length as isize) };
    0 as i32;
    if ((enc as u32) as i32) == (1 as i32) {
        incr = 1 as i32;
    } else {
        incr = 2 as i32;
        let __v1097: i32 = length;
        let __v1098: i32 = __v1097 & !(1 as i32);
        length = __v1098;
        0 as i32;
        i = (3 as i32) - ((enc as u32) as i32);
        '__slate_break_805: loop {
            if !(i < length
                && ((unsafe { *unsafe { zNum.offset(i as isize) } }) as i32) == (0 as i32))
            {
                break;
            }
            let __v1099: i32 = i;
            let __v1100: i32 = __v1099 + (2 as i32);
            i = __v1100;
        }
        nonNum = (i < length) as i32;
        zEnd = unsafe { zNum.offset((i ^ (1 as i32)) as isize) };
        let __v1101: *const i8 = zNum;
        let __v1102: *const i8 =
            unsafe { __v1101.offset((((enc as u32) as i32) & (1 as i32)) as isize) };
        zNum = __v1102;
    }
    '__slate_break_806: while zNum < zEnd
        && (((unsafe {
            *unsafe {
                unsafe { std::ptr::addr_of!(sqlite3CtypeMap) as *const u8 }
                    .offset(((((unsafe { *zNum }) as u8) as u32) as i32) as isize)
            }
        }) as u32) as i32)
            & (1 as i32)
            != (0 as i32)
    {
        let __v1103: *const i8 = zNum;
        let __v1104: *const i8 = unsafe { __v1103.offset(incr as isize) };
        zNum = __v1104;
    }
    if zNum < zEnd {
        if ((unsafe { *zNum }) as i32) == (45 as i32) {
            neg = 1 as i32;
            let __v1105: *const i8 = zNum;
            let __v1106: *const i8 = unsafe { __v1105.offset(incr as isize) };
            zNum = __v1106;
        } else {
            if ((unsafe { *zNum }) as i32) == (43 as i32) {
                let __v1107: *const i8 = zNum;
                let __v1108: *const i8 = unsafe { __v1107.offset(incr as isize) };
                zNum = __v1108;
            }
        }
    }
    zStart = zNum;
    // /* Skip leading zeros. */
    '__slate_break_807: while zNum < zEnd
        && ((unsafe { *unsafe { zNum.offset((0 as i32) as isize) } }) as i32) == (48 as i32)
    {
        let __v1109: *const i8 = zNum;
        let __v1110: *const i8 = unsafe { __v1109.offset(incr as isize) };
        zNum = __v1110;
    }
    i = 0 as i32;
    '__slate_break_808: loop {
        let __v1111: bool;
        if (unsafe { zNum.offset(i as isize) }) < zEnd {
            let __v1112: u32 = (((unsafe { *unsafe { zNum.offset(i as isize) } }) as i32) as u32)
                .wrapping_sub((48 as i32) as u32);
            c = __v1112;
            __v1111 = __v1112 <= ((9 as i32) as u32);
        } else {
            __v1111 = false as bool;
        }
        if !__v1111 {
            break;
        }
        u = u
            .wrapping_mul(((10 as i32) as i64) as u64)
            .wrapping_add(c as u64);
        let __v1113: i32 = i;
        let __v1114: i32 = __v1113 + incr;
        i = __v1114;
    }
    {}
    {}
    {}
    if u > (((((4294967295 as u32) as u64) as i64) | ((2147483647 as i32) as i64) << (32 as i32))
        as u64)
    {
        // /* This test and assignment is needed only to suppress UB warnings
        //     ** from clang and -fsanitize=undefined.  This test and assignment make
        //     ** the code a little larger and slower, and no harm comes from omitting
        //     ** them, but we must appease the undefined-behavior pharisees. */
        unsafe {
            *pNum = if neg != (0 as i32) {
                (-(1 as i32) as i64)
                    - ((((4294967295 as u32) as u64) as i64)
                        | ((2147483647 as i32) as i64) << (32 as i32))
            } else {
                (((4294967295 as u32) as u64) as i64) | ((2147483647 as i32) as i64) << (32 as i32)
            };
        }
    } else {
        if neg != (0 as i32) {
            unsafe {
                *pNum = -(u as i64);
            }
        } else {
            unsafe {
                *pNum = u as i64;
            }
        }
    }
    rc = 0 as i32;
    // /* No digits */
    if i == (0 as i32) && zStart == zNum {
        rc = -(1 as i32);
    // /* UTF16 with high-order bytes non-zero */
    } else {
        if nonNum != (0 as i32) {
            rc = 1 as i32;
        // /* Extra bytes at the end */
        } else {
            if (unsafe { zNum.offset(i as isize) }) < zEnd {
                let mut jj: i32 = i;
                '__slate_break_809: loop {
                    if !((((unsafe {
                        *unsafe {
                            unsafe { std::ptr::addr_of!(sqlite3CtypeMap) as *const u8 }.offset(
                                ((((unsafe { *unsafe { zNum.offset(jj as isize) } }) as u8) as u32)
                                    as i32) as isize,
                            )
                        }
                    }) as u32) as i32)
                        & (1 as i32)
                        != (0 as i32))
                    {
                        // /* Extra non-space text after the integer */
                        rc = 1 as i32;
                        break '__slate_break_809;
                    }
                    let __v1115: i32 = jj;
                    let __v1116: i32 = __v1115 + incr;
                    jj = __v1116;
                    if !((unsafe { zNum.offset(jj as isize) }) < zEnd) {
                        break;
                    }
                }
            }
        }
    }
    if i < (19 as i32) * incr {
        // /* Less than 19 digits, so we know that it fits in 64 bits */
        0 as i32;
        return rc;
    } else {
        // /* zNum is a 19-digit numbers.  Compare it against 9223372036854775808. */
        let __v1117: i32;
        if i > (19 as i32) * incr {
            __v1117 = 1 as i32;
        } else {
            __v1117 = compare2pow63(zNum, incr);
        }
        j = __v1117;
        if j < (0 as i32) {
            // /* zNum is less than 9223372036854775808 so it fits */
            0 as i32;
            return rc;
        } else {
            unsafe {
                *pNum = if neg != (0 as i32) {
                    (-(1 as i32) as i64)
                        - ((((4294967295 as u32) as u64) as i64)
                            | ((2147483647 as i32) as i64) << (32 as i32))
                } else {
                    (((4294967295 as u32) as u64) as i64)
                        | ((2147483647 as i32) as i64) << (32 as i32)
                };
            }
            if j > (0 as i32) {
                // /* zNum is greater than 9223372036854775808 so it overflows */
                return 2 as i32;
            } else {
                // /* zNum is exactly 9223372036854775808.  Fits if negative.  The
                //         ** special case 2 overflow if positive */
                0 as i32;
                return if neg != (0 as i32) { rc } else { 3 as i32 };
            }
        }
    }
    return unsafe { std::mem::zeroed() };
}

// /*
// ** Transform a UTF-8 integer literal, in either decimal or hexadecimal,
// ** into a 64-bit signed integer.  This routine accepts hexadecimal literals,
// ** whereas sqlite3Atoi64() does not.
// **
// ** Returns:
// **
// **     0    Successful transformation.  Fits in a 64-bit signed integer.
// **     1    Excess text after the integer value
// **     2    Integer too large for a 64-bit signed integer or is malformed
// **     3    Special case of 9223372036854775808
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3DecOrHexToI64(mut z: *const i8, mut pOut: *mut i64) -> i32 {
    if ((unsafe { *unsafe { z.offset((0 as i32) as isize) } }) as i32) == (48 as i32)
        && (((unsafe { *unsafe { z.offset((1 as i32) as isize) } }) as i32) == (120 as i32)
            || ((unsafe { *unsafe { z.offset((1 as i32) as isize) } }) as i32) == (88 as i32))
    {
        let mut u: u64 = ((0 as i32) as i64) as u64;
        let mut i: i32 = 0 as i32;
        let mut k: i32 = 0 as i32;
        i = 2 as i32;
        '__slate_break_810: loop {
            if !(((unsafe { *unsafe { z.offset(i as isize) } }) as i32) == (48 as i32)) {
                break;
            }
            let __v1118: i32 = i;
            let __v1119: i32 = __v1118 + (1 as i32);
            i = __v1119;
        }
        k = i;
        '__slate_break_811: loop {
            if !((((unsafe {
                *unsafe {
                    unsafe { std::ptr::addr_of!(sqlite3CtypeMap) as *const u8 }.offset(
                        ((((unsafe { *unsafe { z.offset(k as isize) } }) as u8) as u32) as i32)
                            as isize,
                    )
                }
            }) as u32) as i32)
                & (8 as i32)
                != (0 as i32))
            {
                break;
            }
            u = u.wrapping_mul(((16 as i32) as i64) as u64).wrapping_add(
                (((sqlite3HexToInt((unsafe { *unsafe { z.offset(k as isize) } }) as i32) as u32)
                    as i32) as i64) as u64,
            );
            let __v1120: i32 = k;
            let __v1121: i32 = __v1120 + (1 as i32);
            k = __v1121;
        }
        unsafe {
            memcpy(
                pOut as *mut (),
                std::ptr::addr_of_mut!(u) as *const (),
                ((8 as i32) as i64) as u64,
            )
        };
        if k - i > (16 as i32) {
            return 2 as i32;
        }
        if ((unsafe { *unsafe { z.offset(k as isize) } }) as i32) != (0 as i32) {
            return 1 as i32;
        }
        return 0 as i32;
    } else {
        let mut n: i32 = (((((1073741823 as i32) as i64) as u64)
            & unsafe { strspn(z, (b"+- \n\t0123456789\0".as_ptr() as *mut i8) as *const i8) })
            as u32) as i32;
        if (unsafe { *unsafe { z.offset(n as isize) } }) != (0 as i8) {
            let __v1122: i32 = n;
            let __v1123: i32 = __v1122 + (1 as i32);
            n = __v1123;
        }
        return sqlite3Atoi64(z, pOut, n, ((1 as i32) as i8) as u8);
    }
    // /* SQLITE_OMIT_HEX_INTEGER */
    return unsafe { std::mem::zeroed() };
}

// /*
// ** Set the most recent error code and error string for the sqlite
// ** handle "db". The error code is set to "err_code".
// **
// ** If it is not NULL, string zFormat specifies the format of the
// ** error string.  zFormat and any string tokens that follow it are
// ** assumed to be encoded in UTF-8.
// **
// ** To clear the most recent error for sqlite handle "db", sqlite3Error
// ** should be called with err_code set to SQLITE_OK and zFormat set
// ** to NULL.
// */
#[unsafe(no_mangle)]
unsafe extern "C-unwind" fn sqlite3ErrorWithMsg(
    mut db: *mut sqlite3,
    mut err_code: i32,
    mut zFormat: *const i8,
    mut __va_args: ...
) {
    0 as i32;
    unsafe {
        (*db).errCode = err_code;
    }
    sqlite3SystemError(db, err_code);
    if zFormat == std::ptr::null::<i8>() {
        sqlite3Error(db, err_code);
    } else {
        let __v1124: bool;
        if (unsafe { (*db).pErr }) != std::ptr::null_mut::<sqlite3_value>() {
            __v1124 = true as bool;
        } else {
            let __v1125: *mut sqlite3_value = unsafe { sqlite3ValueNew(db) };
            unsafe {
                (*db).pErr = __v1125;
            }
            __v1124 = __v1125 != std::ptr::null_mut::<sqlite3_value>();
        }
        if __v1124 {
            let mut z: *mut i8 = unsafe { std::mem::zeroed() };
            let mut ap: core::ffi::VaList<'_> = unsafe { std::mem::zeroed() };
            ap = __va_args.clone();
            z = unsafe { sqlite3VMPrintf(db, zFormat, ap.clone()) };
            {}
            unsafe {
                sqlite3ValueSetStr(
                    unsafe { (*db).pErr },
                    -(1 as i32),
                    z as *const (),
                    ((1 as i32) as i8) as u8,
                    unsafe {
                        std::mem::transmute::<*const (), Option<unsafe extern "C-unwind" fn(*mut ())>>(
                            sqlite3RowSetClear as *const (),
                        )
                    },
                )
            };
        }
    }
}

// /*
// ** Set the current error code to err_code and clear any prior error message.
// ** Also set iSysErrno (by calling sqlite3System) if the err_code indicates
// ** that would be appropriate.
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3Error(mut db: *mut sqlite3, mut err_code: i32) {
    0 as i32;
    unsafe {
        (*db).errCode = err_code;
    }
    if err_code != (0 as i32) || (unsafe { (*db).pErr }) != std::ptr::null_mut::<sqlite3_value>() {
        sqlite3ErrorFinish(db, err_code);
    } else {
        unsafe {
            (*db).errByteOffset = -(1 as i32);
        }
    }
}

// /*
// ** The equivalent of sqlite3Error(db, SQLITE_OK).  Clear the error state
// ** and error message.
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3ErrorClear(mut db: *mut sqlite3) {
    0 as i32;
    unsafe {
        (*db).errCode = 0 as i32;
    }
    unsafe {
        (*db).errByteOffset = -(1 as i32);
    }
    if (unsafe { (*db).pErr }) != std::ptr::null_mut::<sqlite3_value>() {
        unsafe { sqlite3ValueSetNull(unsafe { (*db).pErr }) };
    }
}

// /*
// ** Load the sqlite3.iSysErrno field if that is an appropriate thing
// ** to do based on the SQLite error code in rc.
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3SystemError(mut db: *mut sqlite3, mut rc: i32) {
    if rc == (10 as i32) | (12 as i32) << (8 as i32) {
        return;
    }
    let __v1126: i32 = rc;
    let __v1127: i32 = __v1126 & (255 as i32);
    rc = __v1127;
    if rc == (14 as i32) || rc == (10 as i32) {
        unsafe {
            (*db).iSysErrno = unsafe { sqlite3OsGetLastError(unsafe { (*db).pVfs }) };
        }
    }
}

// /*
// ** Convert a BLOB literal of the form "x'hhhhhh'" into its binary
// ** value.  Return a pointer to its binary value.  Space to hold the
// ** binary value has been obtained from malloc and must be freed by
// ** the calling routine.
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3HexToBlob(
    mut db: *mut sqlite3,
    mut z: *const i8,
    mut n: i32,
) -> *mut () {
    let mut zBlob: *mut i8 = unsafe { std::mem::zeroed() };
    let mut i: i32 = 0 as i32;
    zBlob = (unsafe { sqlite3DbMallocRawNN(db, ((n / (2 as i32) + (1 as i32)) as i64) as u64) })
        as *mut i8;
    let __v1128: i32 = n;
    let __v1129: i32 = __v1128 - (1 as i32);
    n = __v1129;
    if zBlob != std::ptr::null_mut::<i8>() {
        i = 0 as i32;
        '__slate_break_830: loop {
            if !(i < n) {
                break;
            }
            unsafe {
                *unsafe { zBlob.offset((i / (2 as i32)) as isize) } = (((sqlite3HexToInt(
                    (unsafe { *unsafe { z.offset(i as isize) } }) as i32,
                ) as u32)
                    as i32)
                    << (4 as i32)
                    | ((sqlite3HexToInt(
                        (unsafe { *unsafe { z.offset((i + (1 as i32)) as isize) } }) as i32,
                    ) as u32) as i32))
                    as i8;
            }
            let __v1130: i32 = i;
            let __v1131: i32 = __v1130 + (2 as i32);
            i = __v1131;
        }
        unsafe {
            *unsafe { zBlob.offset((i / (2 as i32)) as isize) } = (0 as i32) as i8;
        }
    }
    return zBlob as *mut ();
}

// /* SQLITE_BYTEORDER!=4321 */
// /*
// ** Translate a single byte of Hex into an integer.
// ** This routine only works if h really is a valid hexadecimal
// ** character:  0..9a..fA..F
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3HexToInt(mut h: i32) -> u8 {
    0 as i32;
    let __v1132: i32 = h;
    let __v1133: i32 = __v1132 + (9 as i32) * ((1 as i32) & h >> (6 as i32));
    h = __v1133;
    return ((h & (15 as i32)) as i8) as u8;
}

// /*
// ** Attempt to add, subtract, or multiply the 64-bit signed value iB against
// ** the other 64-bit signed integer at *pA and store the result in *pA.
// ** Return 0 on success.  Or if the operation would have resulted in an
// ** overflow, leave *pA unchanged and return 1.
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3AddInt64(mut pA: *mut i64, mut iB: i64) -> i32 {
    let mut iA: i64 = unsafe { *pA };
    {}
    {}
    {}
    {}
    if iB >= ((0 as i32) as i64) {
        {}
        {}
        if iA > ((0 as i32) as i64)
            && ((((4294967295 as u32) as u64) as i64) | ((2147483647 as i32) as i64) << (32 as i32))
                - iA
                < iB
        {
            return 1 as i32;
        }
    } else {
        {}
        {}
        if iA < ((0 as i32) as i64)
            && -(iA
                + ((((4294967295 as u32) as u64) as i64)
                    | ((2147483647 as i32) as i64) << (32 as i32)))
                > iB + ((1 as i32) as i64)
        {
            return 1 as i32;
        }
    }
    let __v1134: *mut i64 = pA;
    let __v1135: i64 = unsafe { *__v1134 };
    let __v1136: i64 = __v1135 + iB;
    unsafe {
        *__v1134 = __v1136;
    }
    return 0 as i32;
}

#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3SubInt64(mut pA: *mut i64, mut iB: i64) -> i32 {
    {}
    if iB
        == (-(1 as i32) as i64)
            - ((((4294967295 as u32) as u64) as i64) | ((2147483647 as i32) as i64) << (32 as i32))
    {
        {}
        {}
        if (unsafe { *pA }) >= ((0 as i32) as i64) {
            return 1 as i32;
        }
        let __v1137: *mut i64 = pA;
        let __v1138: i64 = unsafe { *__v1137 };
        let __v1139: i64 = __v1138 - iB;
        unsafe {
            *__v1137 = __v1139;
        }
        return 0 as i32;
    } else {
        return sqlite3AddInt64(pA, -iB);
    }
    return unsafe { std::mem::zeroed() };
}

#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3MulInt64(mut pA: *mut i64, mut iB: i64) -> i32 {
    let mut iA: i64 = unsafe { *pA };
    if iB > ((0 as i32) as i64) {
        if iA
            > ((((4294967295 as u32) as u64) as i64) | ((2147483647 as i32) as i64) << (32 as i32))
                / iB
        {
            return 1 as i32;
        }
        if iA
            < ((-(1 as i32) as i64)
                - ((((4294967295 as u32) as u64) as i64)
                    | ((2147483647 as i32) as i64) << (32 as i32)))
                / iB
        {
            return 1 as i32;
        }
    } else {
        if iB < ((0 as i32) as i64) {
            if iA > ((0 as i32) as i64) {
                if iB
                    < ((-(1 as i32) as i64)
                        - ((((4294967295 as u32) as u64) as i64)
                            | ((2147483647 as i32) as i64) << (32 as i32)))
                        / iA
                {
                    return 1 as i32;
                }
            } else {
                if iA < ((0 as i32) as i64) {
                    if iB
                        == (-(1 as i32) as i64)
                            - ((((4294967295 as u32) as u64) as i64)
                                | ((2147483647 as i32) as i64) << (32 as i32))
                    {
                        return 1 as i32;
                    }
                    if iA
                        == (-(1 as i32) as i64)
                            - ((((4294967295 as u32) as u64) as i64)
                                | ((2147483647 as i32) as i64) << (32 as i32))
                    {
                        return 1 as i32;
                    }
                    if -iA
                        > ((((4294967295 as u32) as u64) as i64)
                            | ((2147483647 as i32) as i64) << (32 as i32))
                            / -iB
                    {
                        return 1 as i32;
                    }
                }
            }
        }
    }
    unsafe {
        *pA = iA * iB;
    }
    return 0 as i32;
}

// /*
// ** Compute the absolute value of a 32-bit signed integer, if possible.  Or
// ** if the integer has a value of -2147483648, return +2147483647
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3AbsInt32(mut x_629: i32) -> i32 {
    if x_629 >= (0 as i32) {
        return x_629;
    }
    if x_629 == ((2147483648 as u32) as i32) {
        return 2147483647 as i32;
    }
    return -x_629;
}

// /*
// ** Compute an 8-bit hash on a string that is insensitive to case differences
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3StrIHash(mut z: *const i8) -> u8 {
    let mut h: u8 = ((0 as i32) as i8) as u8;
    if z == std::ptr::null::<i8>() {
        return ((0 as i32) as i8) as u8;
    }
    '__slate_break_792: while (unsafe { *unsafe { z.offset((0 as i32) as isize) } }) != (0 as i8) {
        let __v1140: u8 = h;
        let __v1141: u8 = ((((__v1140 as u32) as i32)
            + (((unsafe {
                *unsafe {
                    unsafe { std::ptr::addr_of!(sqlite3UpperToLower) as *const u8 }.offset(
                        ((((unsafe { *unsafe { z.offset((0 as i32) as isize) } }) as u8) as u32)
                            as i32) as isize,
                    )
                }
            }) as u32) as i32)) as i8) as u8;
        h = __v1141;
        let __v1142: *const i8 = z;
        let __v1143: *const i8 = unsafe { __v1142.offset((1 as i32) as isize) };
        z = __v1143;
    }
    return h;
}

// /*
// ** Read an unsigned 32-bit integer from an unaligned big-endian array of bytes.
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3Get4byte(mut p: *const u8) -> u32 {
    // /* Test this limb using -DSQLITE_BYTEORDER=0 */
    {}
    return ((unsafe { *unsafe { p.offset((0 as i32) as isize) } }) as u32) << (24 as i32)
        | (((((unsafe { *unsafe { p.offset((1 as i32) as isize) } }) as u32) as i32) << (16 as i32))
            as u32)
        | (((((unsafe { *unsafe { p.offset((2 as i32) as isize) } }) as u32) as i32) << (8 as i32))
            as u32)
        | ((((unsafe { *unsafe { p.offset((3 as i32) as isize) } }) as u32) as i32) as u32);
}

// /*
// ** Read an unsigned 64-bit integer from an unaligned big-endian byte array.
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3Get8byte(mut p: *const u8) -> u64 {
    // /* Test this limb using -DSQLITE_BYTEORDER=0 */
    {}
    return (((unsafe { *unsafe { p.offset((0 as i32) as isize) } }) as u64) << (56 as i32))
        .wrapping_add(
            ((unsafe { *unsafe { p.offset((1 as i32) as isize) } }) as u64) << (48 as i32),
        )
        .wrapping_add(
            ((unsafe { *unsafe { p.offset((2 as i32) as isize) } }) as u64) << (40 as i32),
        )
        .wrapping_add(
            ((unsafe { *unsafe { p.offset((3 as i32) as isize) } }) as u64) << (32 as i32),
        )
        .wrapping_add(
            ((unsafe { *unsafe { p.offset((4 as i32) as isize) } }) as u64) << (24 as i32),
        )
        .wrapping_add(
            ((unsafe { *unsafe { p.offset((5 as i32) as isize) } }) as u64) << (16 as i32),
        )
        .wrapping_add(((unsafe { *unsafe { p.offset((6 as i32) as isize) } }) as u64) << (8 as i32))
        .wrapping_add(
            ((unsafe { *unsafe { p.offset((7 as i32) as isize) } }) as u64) << (0 as i32),
        );
}

// /* Only used for little-endian machines */
// /*
// ** Byte-swap a 64-bit unsigned integer.
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3BSwap64(mut x_608: u64) -> u64 {
    // /* Test this limb using -DSQLITE_BYTEORDER=0 */
    x_608 = x_608 << (32 as i32) | x_608 >> (32 as i32);
    x_608 = (x_608 & (281470681808895 as u64)) << (16 as i32)
        | (x_608 & (18446462603027742720 as u64)) >> (16 as i32);
    x_608 = (x_608 & (71777214294589695 as u64)) << (8 as i32)
        | (x_608 & (18374966859414961920 as u64)) >> (8 as i32);
    return x_608;
}

// /* Write an unsigned 32-bit integer into an unaligned big-endian array of bytes.
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3Put4byte(mut p: *mut u8, mut v: u32) {
    // /* Test this limb using -DSQLITE_BYTEORDER=0 */
    unsafe {
        *unsafe { p.offset((0 as i32) as isize) } = (v >> (24 as i32)) as u8;
    }
    unsafe {
        *unsafe { p.offset((1 as i32) as isize) } = (v >> (16 as i32)) as u8;
    }
    unsafe {
        *unsafe { p.offset((2 as i32) as isize) } = (v >> (8 as i32)) as u8;
    }
    unsafe {
        *unsafe { p.offset((3 as i32) as isize) } = v as u8;
    }
}

// /*
// ** Helper function for sqlite3Error() - called rarely.  Broken out into
// ** a separate routine to avoid unnecessary register saves on entry to
// ** sqlite3Error().
// */
fn sqlite3ErrorFinish(mut db: *mut sqlite3, mut err_code: i32) {
    if (unsafe { (*db).pErr }) != std::ptr::null_mut::<sqlite3_value>() {
        unsafe { sqlite3ValueSetNull(unsafe { (*db).pErr }) };
    }
    sqlite3SystemError(db, err_code);
}

// /*
// ** Two inputs are multiplied to get a 128-bit result.  Write the
// ** lower 64-bits of the result into *pLo, and return the high-order
// ** 64 bits.
// */
fn sqlite3Multiply128(mut a_444: u64, mut b: u64, mut pLo: *mut u64) -> u64 {
    let mut r: u128 = (a_444 as u128).wrapping_mul(b as u128);
    unsafe {
        *pLo = r as u64;
    }
    return (r >> (64 as i32)) as u64;
}

// /*
// ** A is an unsigned 96-bit integer formed by (a<<32)+aLo.
// ** B is an unsigned 64-bit integer.
// **
// ** Compute the upper 96 bits of 160-bit result of A*B.
// **
// ** Write ((A*B)>>64 & 0xffffffff) (the middle 32 bits of A*B)
// ** into *pLo.  Return the upper 64 bits of A*B.
// **
// ** The lower 64 bits of A*B are discarded.
// */
fn sqlite3Multiply160(mut a_449: u64, mut aLo: u32, mut b: u64, mut pLo: *mut u32) -> u64 {
    let mut r: u128 = (a_449 as u128).wrapping_mul(b as u128);
    let __v1144: u128 = r;
    let __v1145: u128 = __v1144.wrapping_add((aLo as u128).wrapping_mul(b as u128) >> (32 as i32));
    r = __v1145;
    unsafe {
        *pLo = (r >> (32 as i32) & ((4294967295 as u32) as u128)) as u32;
    }
    return (r >> (64 as i32)) as u64;
}

// /*
// ** Return a u64 with the N-th bit set.
// */
// /*
// ** Range of powers of 10 that we need to deal with when converting
// ** IEEE754 doubles to and from decimal.
// */
// /*
// ** For any p between -348 and +347, return the integer part of
// **
// **    pow(10,p) * pow(2,63-pow10to2(p))
// **
// ** Or, in other words, for any p in range, return the most significant
// ** 64 bits of pow(10,p).  The pow(10,p) value is shifted left or right,
// ** as appropriate so the most significant 64 bits fit exactly into a
// ** 64-bit unsigned integer.
// **
// ** Write into *pLo the next 32 significant bits of the answer after
// ** the first 64.
// **
// ** Algorithm:
// **
// ** (1) For p between 0 and 26, return the value directly from the aBase[]
// **     lookup table.
// **
// ** (2) For p outside the range 0 to 26, use aScale[] for the initial value
// **     then refine that result (if necessary) by a single multiplication
// **     against aBase[].
// **
// ** The constant tables aBase[], aScale[], and aScaleLo[] are generated
// ** by the C program at ../tool/mkfptab.c run with the --round option.
// */
fn powerOfTen(mut p: i32, mut pLo: *mut u32) -> u64 {
    // /*  0: 1.0e+0 << 63 */
    // /*  1: 1.0e+1 << 60 */
    // /*  2: 1.0e+2 << 57 */
    // /*  3: 1.0e+3 << 54 */
    // /*  4: 1.0e+4 << 50 */
    // /*  5: 1.0e+5 << 47 */
    // /*  6: 1.0e+6 << 44 */
    // /*  7: 1.0e+7 << 40 */
    // /*  8: 1.0e+8 << 37 */
    // /*  9: 1.0e+9 << 34 */
    // /* 10: 1.0e+10 << 30 */
    // /* 11: 1.0e+11 << 27 */
    // /* 12: 1.0e+12 << 24 */
    // /* 13: 1.0e+13 << 20 */
    // /* 14: 1.0e+14 << 17 */
    // /* 15: 1.0e+15 << 14 */
    // /* 16: 1.0e+16 << 10 */
    // /* 17: 1.0e+17 << 7 */
    // /* 18: 1.0e+18 << 4 */
    // /* 19: 1.0e+19 >> 0 */
    // /* 20: 1.0e+20 >> 3 */
    // /* 21: 1.0e+21 >> 6 */
    // /* 22: 1.0e+22 >> 10 */
    // /* 23: 1.0e+23 >> 13 */
    // /* 24: 1.0e+24 >> 16 */
    // /* 25: 1.0e+25 >> 20 */
    // /* 26: 1.0e+26 >> 23 */
    // /*  0: 1.0e-351 << 1229 */
    // /*  1: 1.0e-324 << 1140 */
    // /*  2: 1.0e-297 << 1050 */
    // /*  3: 1.0e-270 << 960 */
    // /*  4: 1.0e-243 << 871 */
    // /*  5: 1.0e-216 << 781 */
    // /*  6: 1.0e-189 << 691 */
    // /*  7: 1.0e-162 << 602 */
    // /*  8: 1.0e-135 << 512 */
    // /*  9: 1.0e-108 << 422 */
    // /* 10: 1.0e-81 << 333 */
    // /* 11: 1.0e-54 << 243 */
    // /* 12: 1.0e-27 << 153 */
    // /* 13: 1.0e-1 << 67 (special case) */
    // /* 14: 1.0e+27 >> 26 */
    // /* 15: 1.0e+54 >> 116 */
    // /* 16: 1.0e+81 >> 206 */
    // /* 17: 1.0e+108 >> 295 */
    // /* 18: 1.0e+135 >> 385 */
    // /* 19: 1.0e+162 >> 475 */
    // /* 20: 1.0e+189 >> 564 */
    // /* 21: 1.0e+216 >> 654 */
    // /* 22: 1.0e+243 >> 744 */
    // /* 23: 1.0e+270 >> 833 */
    // /* 24: 1.0e+297 >> 923 */
    // /* 25: 1.0e+324 >> 1013 */
    // /*  0: 1.0e-351 << 1229 */
    // /*  1: 1.0e-324 << 1140 */
    // /*  2: 1.0e-297 << 1050 */
    // /*  3: 1.0e-270 << 960 */
    // /*  4: 1.0e-243 << 871 */
    // /*  5: 1.0e-216 << 781 */
    // /*  6: 1.0e-189 << 691 */
    // /*  7: 1.0e-162 << 602 */
    // /*  8: 1.0e-135 << 512 */
    // /*  9: 1.0e-108 << 422 */
    // /* 10: 1.0e-81 << 333 */
    // /* 11: 1.0e-54 << 243 */
    // /* 12: 1.0e-27 << 153 */
    // /* 13: 1.0e-1 << 67 (special case) */
    // /* 14: 1.0e+27 >> 26 */
    // /* 15: 1.0e+54 >> 116 */
    // /* 16: 1.0e+81 >> 206 */
    // /* 17: 1.0e+108 >> 295 */
    // /* 18: 1.0e+135 >> 385 */
    // /* 19: 1.0e+162 >> 475 */
    // /* 20: 1.0e+189 >> 564 */
    // /* 21: 1.0e+216 >> 654 */
    // /* 22: 1.0e+243 >> 744 */
    // /* 23: 1.0e+270 >> 833 */
    // /* 24: 1.0e+297 >> 923 */
    // /* 25: 1.0e+324 >> 1013 */
    let mut g: i32 = 0 as i32;
    let mut n: i32 = 0 as i32;
    let mut s: u64 = 0 as u64;
    let mut x_463: u64 = 0 as u64;
    let mut lo: u32 = 0 as u32;
    0 as i32;
    if p < (0 as i32) {
        if p == -(1 as i32) {
            unsafe {
                *pLo = unsafe {
                    *unsafe {
                        unsafe { std::ptr::addr_of!(aScaleLo.0) as *const u32 }
                            .offset((13 as i32) as isize)
                    }
                };
            }
            return unsafe {
                *unsafe {
                    unsafe { std::ptr::addr_of!(aScale.0) as *const u64 }
                        .offset((13 as i32) as isize)
                }
            };
        }
        g = p / (27 as i32);
        n = p % (27 as i32);
        if n != (0 as i32) {
            let __v1146: i32 = g;
            let __v1147: i32 = __v1146 - (1 as i32);
            g = __v1147;
            let __v1148: i32 = n;
            let __v1149: i32 = __v1148 + (27 as i32);
            n = __v1149;
        }
    } else {
        if p < (27 as i32) {
            unsafe {
                *pLo = (0 as i32) as u32;
            }
            return unsafe {
                *unsafe { unsafe { std::ptr::addr_of!(aBase.0) as *const u64 }.offset(p as isize) }
            };
        } else {
            g = p / (27 as i32);
            n = p % (27 as i32);
        }
    }
    s = unsafe {
        *unsafe {
            unsafe { std::ptr::addr_of!(aScale.0) as *const u64 }.offset((g + (13 as i32)) as isize)
        }
    };
    if n == (0 as i32) {
        unsafe {
            *pLo = unsafe {
                *unsafe {
                    unsafe { std::ptr::addr_of!(aScaleLo.0) as *const u32 }
                        .offset((g + (13 as i32)) as isize)
                }
            };
        }
        return s;
    }
    x_463 = sqlite3Multiply160(
        s,
        unsafe {
            *unsafe {
                unsafe { std::ptr::addr_of!(aScaleLo.0) as *const u32 }
                    .offset((g + (13 as i32)) as isize)
            }
        },
        unsafe {
            *unsafe { unsafe { std::ptr::addr_of!(aBase.0) as *const u64 }.offset(n as isize) }
        },
        std::ptr::addr_of_mut!(lo),
    );
    if (((1 as i32) as i64) as u64) << (63 as i32) & x_463 == (((0 as i32) as i64) as u64) {
        x_463 = x_463 << (1 as i32) | ((lo >> (31 as i32) & ((1 as i32) as u32)) as u64);
        lo = lo << (1 as i32) | ((1 as i32) as u32);
    }
    unsafe {
        *pLo = lo;
    }
    return x_463;
}

// /*
// ** pow10to2(x) computes floor(log2(pow(10,x))).
// ** pow2to10(y) computes floor(log10(pow(2,y))).
// **
// ** Conceptually, pow10to2(p) converts a base-10 exponent p into
// ** a corresponding base-2 exponent, and pow2to10(e) converts a base-2
// ** exponent into a base-10 exponent.
// **
// ** The conversions are based on the observation that:
// **
// **     ln(10.0)/ln(2.0) == 108853/32768     (approximately)
// **     ln(2.0)/ln(10.0) == 78913/262144     (approximately)
// **
// ** These ratios are approximate, but they are accurate to 5 digits,
// ** which is close enough for the usage here.  Right-shift is used
// ** for division so that rounding of negative numbers happens in the
// ** right direction.
// */
fn pwr10to2(mut p: i32) -> i32 {
    return p * (108853 as i32) >> (15 as i32);
}

fn pwr2to10(mut p: i32) -> i32 {
    return p * (78913 as i32) >> (18 as i32);
}

// /*
// ** Count leading zeros for a 64-bit unsigned integer.
// */
fn countLeadingZeros(mut m: u64) -> i32 {
    return m.leading_zeros() as i32;
}

// /*
// ** Given m and e, which represent a quantity r == m*pow(2,e),
// ** return values *pD and *pP such that r == (*pD)*pow(10,*pP),
// ** approximately.  *pD should contain at least n significant digits.
// **
// ** The input m is required to have its highest bit set.  In other words,
// ** m should be left-shifted, and e decremented, to maximize the value of m.
// */
fn sqlite3Fp2Convert10(mut m: u64, mut e: i32, mut n: i32, mut pD: *mut u64, mut pP: *mut i32) {
    let mut p: i32 = 0 as i32;
    let mut h: u64 = 0 as u64;
    let mut d1: u64 = 0 as u64;
    let mut d2: u32 = 0 as u32;
    0 as i32;
    p = n - (1 as i32) - pwr2to10(e + (63 as i32));
    h = sqlite3Multiply128(
        m,
        powerOfTen(p, std::ptr::addr_of_mut!(d2)),
        std::ptr::addr_of_mut!(d1),
    );
    0 as i32;
    0 as i32;
    if n == (18 as i32) {
        let __v1150: u64 = h;
        let __v1151: u64 = __v1150 >> -(e + pwr10to2(p) + (2 as i32));
        h = __v1151;
        unsafe {
            *pD = h.wrapping_add(h << (1 as i32) & (((2 as i32) as i64) as u64)) >> (1 as i32);
        }
    } else {
        unsafe {
            *pD = h >> -(e + pwr10to2(p) + (1 as i32));
        }
    }
    unsafe {
        *pP = -p;
    }
}

// /*
// ** Return an IEEE754 floating point value that approximates d*pow(10,p).
// **
// ** The (current) algorithm is adapted from the work of Ross Cox at
// ** https://github.com/rsc/fpfmt
// */
fn sqlite3Fp10Convert2(mut d: u64, mut p: i32) -> f64 {
    let mut b: i32 = 0 as i32;
    let mut lp: i32 = 0 as i32;
    let mut e: i32 = 0 as i32;
    let mut adj: i32 = 0 as i32;
    let mut s: i32 = 0 as i32;
    let mut pwr10l: u32 = 0 as u32;
    let mut mid1: u32 = 0 as u32;
    let mut pwr10h: u64 = 0 as u64;
    let mut x_492: u64 = 0 as u64;
    let mut hi: u64 = 0 as u64;
    let mut lo: u64 = 0 as u64;
    let mut sticky: u64 = 0 as u64;
    let mut u: u64 = 0 as u64;
    let mut m: u64 = 0 as u64;
    let mut r: f64 = 0 as f64;
    if p < -(348 as i32) {
        return 0.0f64;
    }
    if p > (347 as i32) {
        return f32::INFINITY as f64;
    }
    b = (64 as i32) - countLeadingZeros(d);
    lp = pwr10to2(p);
    e = (53 as i32) - b - lp;
    if e > (1074 as i32) {
        if e >= (1130 as i32) {
            return 0.0f64;
        }
        e = 1074 as i32;
    }
    s = -(e - ((64 as i32) - b) + lp + (3 as i32));
    pwr10h = powerOfTen(p, std::ptr::addr_of_mut!(pwr10l));
    if pwr10l != ((0 as i32) as u32) {
        let __v1152: u64 = pwr10h;
        let __v1153: u64 = __v1152.wrapping_add(((1 as i32) as i64) as u64);
        pwr10h = __v1153;
        pwr10l = !pwr10l;
    }
    x_492 = d << (64 as i32) - b;
    hi = sqlite3Multiply128(x_492, pwr10h, std::ptr::addr_of_mut!(lo));
    mid1 = (lo >> (32 as i32)) as u32;
    sticky = ((1 as i32) as i64) as u64;
    if hi & ((((1 as i32) as i64) as u64) << s).wrapping_sub(((1 as i32) as i64) as u64)
        == (((0 as i32) as i64) as u64)
    {
        let mut mid2: u32 = (sqlite3Multiply128(
            x_492,
            (pwr10l as u64) << (32 as i32),
            std::ptr::addr_of_mut!(lo),
        ) >> (32 as i32)) as u32;
        sticky = (mid1.wrapping_sub(mid2) > ((1 as i32) as u32)) as u64;
        let __v1154: u64 = hi;
        let __v1155: u64 = __v1154.wrapping_sub((((mid1 < mid2) as i32) as i64) as u64);
        hi = __v1155;
    }
    u = hi >> s | sticky;
    adj = (u
        >= ((((1 as i32) as i64) as u64) << (55 as i32)).wrapping_sub(((2 as i32) as i64) as u64))
        as i32;
    if adj != (0 as i32) {
        u = u >> adj | u & (((1 as i32) as i64) as u64);
        let __v1156: i32 = e;
        let __v1157: i32 = __v1156 - adj;
        e = __v1157;
    }
    m = u
        .wrapping_add(((1 as i32) as i64) as u64)
        .wrapping_add(u >> (2 as i32) & (((1 as i32) as i64) as u64))
        >> (2 as i32);
    if e <= -(972 as i32) {
        return f32::INFINITY as f64;
    }
    if m & (((1 as i32) as i64) as u64) << (52 as i32) != (((0 as i32) as i64) as u64) {
        m = m & !((((1 as i32) as i64) as u64) << (52 as i32))
            | ((((1075 as i32) - e) as i64) as u64) << (52 as i32);
    }
    unsafe {
        memcpy(
            std::ptr::addr_of_mut!(r) as *mut (),
            std::ptr::addr_of_mut!(m) as *const (),
            ((8 as i32) as i64) as u64,
        )
    };
    return r;
}

// /*
// ** Compare the 19-character string zNum against the text representation
// ** value 2^63:  9223372036854775808.  Return negative, zero, or positive
// ** if zNum is less than, equal to, or greater than the string.
// ** Note that zNum must contain exactly 19 characters.
// **
// ** Unlike memcmp() this routine is guaranteed to return the difference
// ** in the values of the last digit if the only difference is in the
// ** last digit.  So, for example,
// **
// **      compare2pow63("9223372036854775800", 1)
// **
// ** will return -8.
// */
fn compare2pow63(mut zNum: *const i8, mut incr: i32) -> i32 {
    let mut c: i32 = 0 as i32;
    let mut i: i32 = 0 as i32;
    // /* 012345678901234567 */
    let mut pow63: *const i8 = (b"922337203685477580\0".as_ptr() as *mut i8) as *const i8;
    i = 0 as i32;
    '__slate_break_804: loop {
        if !(c == (0 as i32) && i < (18 as i32)) {
            break;
        }
        c = (((unsafe { *unsafe { zNum.offset((i * incr) as isize) } }) as i32)
            - ((unsafe { *unsafe { pow63.offset(i as isize) } }) as i32))
            * (10 as i32);
        let __v1158: i32 = i;
        let __v1159: i32 = __v1158 + (1 as i32);
        i = __v1159;
    }
    if c == (0 as i32) {
        c = ((unsafe { *unsafe { zNum.offset(((18 as i32) * incr) as isize) } }) as i32)
            - (56 as i32);
        {}
        {}
        {}
    }
    return c;
}

// /*
// ** The variable-length integer encoding is as follows:
// **
// ** KEY:
// **         A = 0xxxxxxx    7 bits of data and one flag bit
// **         B = 1xxxxxxx    7 bits of data and one flag bit
// **         C = xxxxxxxx    8 bits of data
// **
// **  7 bits - A
// ** 14 bits - BA
// ** 21 bits - BBA
// ** 28 bits - BBBA
// ** 35 bits - BBBBA
// ** 42 bits - BBBBBA
// ** 49 bits - BBBBBBA
// ** 56 bits - BBBBBBBA
// ** 64 bits - BBBBBBBBC
// */
// /*
// ** Write a 64-bit variable-length integer to memory starting at p[0].
// ** The length of data write will be between 1 and 9 bytes.  The number
// ** of bytes written is returned.
// **
// ** A variable-length integer consists of the lower 7 bits of each byte
// ** for all bytes that have the 8th bit set and one byte with the 8th
// ** bit clear.  Except, if we get to the 9th byte, it stores the full
// ** 8 bits and is the last byte.
// */
fn putVarint64(mut p: *mut u8, mut v: u64) -> i32 {
    let mut i: i32 = 0 as i32;
    let mut j: i32 = 0 as i32;
    let mut n: i32 = 0 as i32;
    let mut buf: [u8; 10] = [0 as u8; 10];
    if v & ((4278190080 as u32) as u64) << (32 as i32) != (0 as u64) {
        unsafe {
            *unsafe { p.offset((8 as i32) as isize) } = v as u8;
        }
        let __v1160: u64 = v;
        let __v1161: u64 = __v1160 >> (8 as i32);
        v = __v1161;
        i = 7 as i32;
        '__slate_break_826: loop {
            if !(i >= (0 as i32)) {
                break;
            }
            unsafe {
                *unsafe { p.offset(i as isize) } =
                    (v & (((127 as i32) as i64) as u64) | (((128 as i32) as i64) as u64)) as u8;
            }
            let __v1164: u64 = v;
            let __v1165: u64 = __v1164 >> (7 as i32);
            v = __v1165;
            let __v1162: i32 = i;
            let __v1163: i32 = __v1162 - (1 as i32);
            i = __v1163;
        }
        return 9 as i32;
    }
    n = 0 as i32;
    '__slate_break_827: loop {
        let __v1166: i32 = n;
        let __v1167: i32 = __v1166 + (1 as i32);
        n = __v1167;
        unsafe {
            *unsafe { (buf.as_mut_ptr() as *mut u8).offset(__v1166 as isize) } =
                (v & (((127 as i32) as i64) as u64) | (((128 as i32) as i64) as u64)) as u8;
        }
        let __v1168: u64 = v;
        let __v1169: u64 = __v1168 >> (7 as i32);
        v = __v1169;
        if !(v != (((0 as i32) as i64) as u64)) {
            break;
        }
    }
    let __v1170: *mut u8 = unsafe { (buf.as_mut_ptr() as *mut u8).offset((0 as i32) as isize) };
    let __v1171: u8 = unsafe { *__v1170 };
    let __v1172: u8 = ((((__v1171 as u32) as i32) & (127 as i32)) as i8) as u8;
    unsafe {
        *__v1170 = __v1172;
    }
    0 as i32;
    i = 0 as i32;
    let __v1173: i32 = n - (1 as i32);
    j = __v1173;
    '__slate_break_828: loop {
        if !(j >= (0 as i32)) {
            break;
        }
        unsafe {
            *unsafe { p.offset(i as isize) } =
                unsafe { *unsafe { (buf.as_mut_ptr() as *mut u8).offset(j as isize) } };
        }
        let __v1174: i32 = j;
        let __v1175: i32 = __v1174 - (1 as i32);
        j = __v1175;
        let __v1176: i32 = i;
        let __v1177: i32 = __v1176 + (1 as i32);
        i = __v1177;
    }
    return n;
}

// /* !SQLITE_OMIT_BLOB_LITERAL */
// /*
// ** Log an error that is an API call on a connection pointer that should
// ** not have been used.  The "type" of connection pointer is given as the
// ** argument.  The zType is a word like "NULL" or "closed" or "invalid".
// */
fn logBadConnection(mut zType: *const i8) {
    unsafe {
        sqlite3_log(
            21 as i32,
            (b"API call with %s database connection pointer\0".as_ptr() as *mut i8) as *const i8,
            zType,
        )
    };
}
