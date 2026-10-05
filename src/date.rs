unsafe extern "C" {
    static mut sqlite3UpperToLower: [u8; 0];
    static mut sqlite3CtypeMap: [u8; 0];
    static mut sqlite3Config: Sqlite3Config;
    fn sqlite3_value_double(__v560: *mut sqlite3_value) -> f64;
    fn sqlite3_value_text(__v561: *mut sqlite3_value) -> *const u8;
    fn sqlite3_value_bytes(__v562: *mut sqlite3_value) -> i32;
    fn sqlite3_value_type(__v563: *mut sqlite3_value) -> i32;
    fn sqlite3_context_db_handle(__v564: *mut sqlite3_context) -> *mut sqlite3;
    fn sqlite3_result_double(__v565: *mut sqlite3_context, __v566: f64);
    fn sqlite3_result_error(__v567: *mut sqlite3_context, __v568: *const i8, __v569: i32);
    fn sqlite3_result_int64(__v570: *mut sqlite3_context, __v571: i64);
    fn sqlite3_result_text(
        __v572: *mut sqlite3_context,
        __v573: *const i8,
        __v574: i32,
        __v575: Option<unsafe extern "C-unwind" fn(*mut ())>,
    );
    fn sqlite3_str_new(__v576: *mut sqlite3) -> *mut sqlite3_str;
    fn sqlite3_str_free(__v577: *mut sqlite3_str);
    fn sqlite3_result_str(__v578: *mut sqlite3_context, __v579: *mut sqlite3_str, __v580: i32);
    fn sqlite3_str_appendf(__v581: *mut sqlite3_str, zFormat: *const i8, ...);
    fn sqlite3_str_append(__v583: *mut sqlite3_str, zIn: *const i8, N: i32);
    fn sqlite3_str_appendchar(__v586: *mut sqlite3_str, N: i32, C: i8);
    fn sqlite3_stricmp(__v589: *const i8, __v590: *const i8) -> i32;
    fn sqlite3_strnicmp(__v591: *const i8, __v592: *const i8, __v593: i32) -> i32;
    fn memset(__s: *mut (), __c: i32, __n: u64) -> *mut ();
    fn strchr(__s: *const i8, __c: i32) -> *mut i8;
    fn sqlite3NotPureFunc(__v599: *mut sqlite3_context) -> i32;
    fn sqlite3StrICmp(__v600: *const i8, __v601: *const i8) -> i32;
    fn sqlite3Strlen30(__v602: *const i8) -> i32;
    fn sqlite3DbStrNDup(__v603: *mut sqlite3, __v604: *const i8, __v605: u64) -> *mut i8;
    fn sqlite3DbFree(__v606: *mut sqlite3, __v607: *mut ());
    fn sqlite3InsertBuiltinFuncs(__v608: *mut FuncDef, __v609: i32);
    fn sqlite3RealSameAsInt(__v610: f64, __v611: i64) -> i32;
    fn sqlite3AtoF(z: *const i8, __v613: *mut f64) -> i32;
    fn sqlite3StrAccumInit(
        __v614: *mut sqlite3_str,
        __v615: *mut sqlite3,
        __v616: *mut i8,
        __v617: i32,
        __v618: i32,
    );
    fn sqlite3StmtCurrentTime(__v619: *mut sqlite3_context) -> i64;
    fn localtime_r(__timer: *const i64, __tp: *mut tm) -> *mut tm;
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
    __slate_bits_0: __slate_bits::__SlateBits79U0,
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
    u: __SlateRecord175,
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
    __slate_bits_0: __slate_bits::__SlateBits103U0,
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
    __slate_bits_0: __slate_bits::__SlateBits115U0,
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
struct tm {
    tm_sec: i32,
    tm_min: i32,
    tm_hour: i32,
    tm_mday: i32,
    tm_mon: i32,
    tm_year: i32,
    tm_wday: i32,
    tm_yday: i32,
    tm_isdst: i32,
    tm_gmtoff: i64,
    tm_zone: *const i8,
}

// /*
// ** 2003 October 31
// **
// ** The author disclaims copyright to this source code.  In place of
// ** a legal notice, here is a blessing:
// **
// **    May you do good and not evil.
// **    May you find forgiveness for yourself and forgive others.
// **    May you share freely, never taking more than you give.
// **
// *************************************************************************
// ** This file contains the C functions that implement date and time
// ** functions for SQLite.
// **
// ** There is only one exported symbol in this file - the function
// ** sqlite3RegisterDateTimeFunctions() found at the bottom of the file.
// ** All other code has file scope.
// **
// ** SQLite processes all times and dates as julian day numbers.  The
// ** dates and times are stored as the number of days since noon
// ** in Greenwich on November 24, 4714 B.C. according to the Gregorian
// ** calendar system.
// **
// ** 1970-01-01 00:00:00 is JD 2440587.5
// ** 2000-01-01 00:00:00 is JD 2451544.5
// **
// ** This implementation requires years to be expressed as a 4-digit number
// ** which means that only dates between 0000-01-01 and 9999-12-31 can
// ** be represented, even though julian day numbers allow a much wider
// ** range of dates.
// **
// ** The Gregorian calendar system is used for all dates and times,
// ** even those that predate the Gregorian calendar.  Historians usually
// ** use the julian calendar for dates prior to 1582-10-15 and for some
// ** dates afterwards, depending on locale.  Beware of this difference.
// **
// ** The conversion algorithms are implemented based on descriptions
// ** in the following text:
// **
// **      Jean Meeus
// **      Astronomical Algorithms, 2nd Edition, 1998
// **      ISBN 0-943396-61-1
// **      Willmann-Bell, Inc
// **      Richmond, Virginia (USA)
// */
// /*
// ** The MSVC CRT on Windows CE may not have a localtime() function.
// ** So declare a substitute.  The substitute function itself is
// ** defined in "os_win.c".
// */
// /*
// ** A structure for holding a single date and time.
// */
#[repr(C)]
#[derive(Clone, Copy)]
struct DateTime {
    iJD: i64,
    // /* The julian day number times 86400000 */
    Y: i32,
    M: i32,
    D: i32,
    // /* Year, month, and day */
    h: i32,
    m: i32,
    // /* Hour and minutes */
    tz: i32,
    // /* Timezone offset in minutes */
    s: f64,
    // /* Seconds */
    validJD: i8,
    // /* True (1) if iJD is valid */
    validYMD: i8,
    // /* True (1) if Y,M,D are valid */
    validHMS: i8,
    // /* True (1) if h,m,s are valid */
    nFloor: i8,
    // /* Days to implement "floor" */
    __slate_bits_0: __slate_bits::__SlateBits205U0,
}

// /* Date at which to calculate offset */
// /* Write error here if one occurs */
// /* SQLITE_OMIT_LOCALTIME */
// /*
// ** The following table defines various date transformations of the form
// **
// **            'NNN days'
// **
// ** Where NNN is an arbitrary floating-point number and "days" can be one
// ** of several units of time.
// */
#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord207 {
    nName: u8,
    // /* Length of the name */
    zName: [i8; 7],
    // /* Name of the transformation */
    rLimit: f32,
    // /* Maximum NNN value for this transform */
    rXform: f32,
    // /* 0 */
    // /* 1 */
    // /* 2 */
    // /* 3 */
    // /* 4 */
    // /* 5 */
    // /* Constant used for this transform */
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
    pub struct __SlateBits79U0 {
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
    pub struct __SlateBits103U0 {
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
    pub struct __SlateBits115U0 {
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
    pub struct __SlateBits205U0 {
        #[bits(1)]
        pub rawS: u32,
        #[bits(1)]
        pub isError: u32,
        #[bits(1)]
        pub useSubsec: u32,
        #[bits(1)]
        pub isUtc: u32,
        #[bits(1)]
        pub isLocal: u32,
        #[bits(3, access = na)]
        pub __slate_pad_5: u8,
    }
}

static mut aMx: [u16; 6] = [
    ((12 as i32) as i16) as u16,
    ((14 as i32) as i16) as u16,
    ((24 as i32) as i16) as u16,
    ((31 as i32) as i16) as u16,
    ((59 as i32) as i16) as u16,
    ((14712 as i32) as i16) as u16,
];

static mut aXformType: __SlateAlign16<[__SlateRecord207; 6]> = __SlateAlign16([
    __SlateRecord207 {
        nName: ((6 as i32) as i8) as u8,
        zName: [
            115 as i8, 101 as i8, 99 as i8, 111 as i8, 110 as i8, 100 as i8, 0 as i8,
        ],
        rLimit: 464270000000000.0f64 as f32,
        rXform: 1.0f64 as f32,
    },
    __SlateRecord207 {
        nName: ((6 as i32) as i8) as u8,
        zName: [
            109 as i8, 105 as i8, 110 as i8, 117 as i8, 116 as i8, 101 as i8, 0 as i8,
        ],
        rLimit: 7737900000000.0f64 as f32,
        rXform: 60.0f64 as f32,
    },
    __SlateRecord207 {
        nName: ((4 as i32) as i8) as u8,
        zName: [
            104 as i8, 111 as i8, 117 as i8, 114 as i8, 0 as i8, 0 as i8, 0 as i8,
        ],
        rLimit: 128970000000.0f64 as f32,
        rXform: 3600.0f64 as f32,
    },
    __SlateRecord207 {
        nName: ((3 as i32) as i8) as u8,
        zName: [
            100 as i8, 97 as i8, 121 as i8, 0 as i8, 0 as i8, 0 as i8, 0 as i8,
        ],
        rLimit: 5373485.0f64 as f32,
        rXform: 86400.0f64 as f32,
    },
    __SlateRecord207 {
        nName: ((5 as i32) as i8) as u8,
        zName: [
            109 as i8, 111 as i8, 110 as i8, 116 as i8, 104 as i8, 0 as i8, 0 as i8,
        ],
        rLimit: 176546.0f64 as f32,
        rXform: 2592000.0f64 as f32,
    },
    __SlateRecord207 {
        nName: ((4 as i32) as i8) as u8,
        zName: [
            121 as i8, 101 as i8, 97 as i8, 114 as i8, 0 as i8, 0 as i8, 0 as i8,
        ],
        rLimit: 14713.0f64 as f32,
        rXform: 31536000.0f64 as f32,
    },
]);

static mut aDateTimeFuncs: __SlateAlign16<[FuncDef; 10]> = __SlateAlign16([
    FuncDef {
        nArg: -(1 as i32) as i16,
        funcFlags: ((8388608 as i32) | (8192 as i32) | (1 as i32) | (2048 as i32)) as u32,
        pUserData: (unsafe { std::ptr::addr_of_mut!(sqlite3Config) }) as *mut (),
        pNext: std::ptr::null_mut::<FuncDef>(),
        xSFunc: Some(juliandayFunc),
        xFinalize: None,
        xValue: None,
        xInverse: None,
        zName: (b"julianday\0".as_ptr() as *mut i8) as *const i8,
        u: {
            let mut __t0: __SlateRecord175 = unsafe { std::mem::zeroed() };
            __t0.pHash = std::ptr::null_mut::<FuncDef>();
            __t0
        },
    },
    FuncDef {
        nArg: -(1 as i32) as i16,
        funcFlags: ((8388608 as i32) | (8192 as i32) | (1 as i32) | (2048 as i32)) as u32,
        pUserData: (unsafe { std::ptr::addr_of_mut!(sqlite3Config) }) as *mut (),
        pNext: std::ptr::null_mut::<FuncDef>(),
        xSFunc: Some(unixepochFunc),
        xFinalize: None,
        xValue: None,
        xInverse: None,
        zName: (b"unixepoch\0".as_ptr() as *mut i8) as *const i8,
        u: {
            let mut __t1: __SlateRecord175 = unsafe { std::mem::zeroed() };
            __t1.pHash = std::ptr::null_mut::<FuncDef>();
            __t1
        },
    },
    FuncDef {
        nArg: -(1 as i32) as i16,
        funcFlags: ((8388608 as i32) | (8192 as i32) | (1 as i32) | (2048 as i32)) as u32,
        pUserData: (unsafe { std::ptr::addr_of_mut!(sqlite3Config) }) as *mut (),
        pNext: std::ptr::null_mut::<FuncDef>(),
        xSFunc: Some(dateFunc),
        xFinalize: None,
        xValue: None,
        xInverse: None,
        zName: (b"date\0".as_ptr() as *mut i8) as *const i8,
        u: {
            let mut __t2: __SlateRecord175 = unsafe { std::mem::zeroed() };
            __t2.pHash = std::ptr::null_mut::<FuncDef>();
            __t2
        },
    },
    FuncDef {
        nArg: -(1 as i32) as i16,
        funcFlags: ((8388608 as i32) | (8192 as i32) | (1 as i32) | (2048 as i32)) as u32,
        pUserData: (unsafe { std::ptr::addr_of_mut!(sqlite3Config) }) as *mut (),
        pNext: std::ptr::null_mut::<FuncDef>(),
        xSFunc: Some(timeFunc),
        xFinalize: None,
        xValue: None,
        xInverse: None,
        zName: (b"time\0".as_ptr() as *mut i8) as *const i8,
        u: {
            let mut __t3: __SlateRecord175 = unsafe { std::mem::zeroed() };
            __t3.pHash = std::ptr::null_mut::<FuncDef>();
            __t3
        },
    },
    FuncDef {
        nArg: -(1 as i32) as i16,
        funcFlags: ((8388608 as i32) | (8192 as i32) | (1 as i32) | (2048 as i32)) as u32,
        pUserData: (unsafe { std::ptr::addr_of_mut!(sqlite3Config) }) as *mut (),
        pNext: std::ptr::null_mut::<FuncDef>(),
        xSFunc: Some(datetimeFunc),
        xFinalize: None,
        xValue: None,
        xInverse: None,
        zName: (b"datetime\0".as_ptr() as *mut i8) as *const i8,
        u: {
            let mut __t4: __SlateRecord175 = unsafe { std::mem::zeroed() };
            __t4.pHash = std::ptr::null_mut::<FuncDef>();
            __t4
        },
    },
    FuncDef {
        nArg: -(1 as i32) as i16,
        funcFlags: ((8388608 as i32) | (8192 as i32) | (1 as i32) | (2048 as i32)) as u32,
        pUserData: (unsafe { std::ptr::addr_of_mut!(sqlite3Config) }) as *mut (),
        pNext: std::ptr::null_mut::<FuncDef>(),
        xSFunc: Some(strftimeFunc),
        xFinalize: None,
        xValue: None,
        xInverse: None,
        zName: (b"strftime\0".as_ptr() as *mut i8) as *const i8,
        u: {
            let mut __t5: __SlateRecord175 = unsafe { std::mem::zeroed() };
            __t5.pHash = std::ptr::null_mut::<FuncDef>();
            __t5
        },
    },
    FuncDef {
        nArg: (2 as i32) as i16,
        funcFlags: ((8388608 as i32) | (8192 as i32) | (1 as i32) | (2048 as i32)) as u32,
        pUserData: (unsafe { std::ptr::addr_of_mut!(sqlite3Config) }) as *mut (),
        pNext: std::ptr::null_mut::<FuncDef>(),
        xSFunc: Some(timediffFunc),
        xFinalize: None,
        xValue: None,
        xInverse: None,
        zName: (b"timediff\0".as_ptr() as *mut i8) as *const i8,
        u: {
            let mut __t6: __SlateRecord175 = unsafe { std::mem::zeroed() };
            __t6.pHash = std::ptr::null_mut::<FuncDef>();
            __t6
        },
    },
    FuncDef {
        nArg: (0 as i32) as i16,
        funcFlags: ((8388608 as i32) | (8192 as i32) | (1 as i32)) as u32,
        pUserData: std::ptr::null_mut::<()>(),
        pNext: std::ptr::null_mut::<FuncDef>(),
        xSFunc: Some(ctimeFunc),
        xFinalize: None,
        xValue: None,
        xInverse: None,
        zName: (b"current_time\0".as_ptr() as *mut i8) as *const i8,
        u: {
            let mut __t7: __SlateRecord175 = unsafe { std::mem::zeroed() };
            __t7.pHash = std::ptr::null_mut::<FuncDef>();
            __t7
        },
    },
    FuncDef {
        nArg: (0 as i32) as i16,
        funcFlags: ((8388608 as i32) | (8192 as i32) | (1 as i32)) as u32,
        pUserData: std::ptr::null_mut::<()>(),
        pNext: std::ptr::null_mut::<FuncDef>(),
        xSFunc: Some(ctimestampFunc),
        xFinalize: None,
        xValue: None,
        xInverse: None,
        zName: (b"current_timestamp\0".as_ptr() as *mut i8) as *const i8,
        u: {
            let mut __t8: __SlateRecord175 = unsafe { std::mem::zeroed() };
            __t8.pHash = std::ptr::null_mut::<FuncDef>();
            __t8
        },
    },
    FuncDef {
        nArg: (0 as i32) as i16,
        funcFlags: ((8388608 as i32) | (8192 as i32) | (1 as i32)) as u32,
        pUserData: std::ptr::null_mut::<()>(),
        pNext: std::ptr::null_mut::<FuncDef>(),
        xSFunc: Some(cdateFunc),
        xFinalize: None,
        xValue: None,
        xInverse: None,
        zName: (b"current_date\0".as_ptr() as *mut i8) as *const i8,
        u: {
            let mut __t9: __SlateRecord175 = unsafe { std::mem::zeroed() };
            __t9.pHash = std::ptr::null_mut::<FuncDef>();
            __t9
        },
    },
]);

// /* !defined(SQLITE_OMIT_DATETIME_FUNCS) */
// /*
// ** This function registered all of the above C functions as SQL
// ** functions.  This should be the only routine in this file with
// ** external linkage.
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3RegisterDateTimeFunctions() {
    unsafe {
        sqlite3InsertBuiltinFuncs(
            unsafe { std::ptr::addr_of_mut!(aDateTimeFuncs.0) as *mut FuncDef },
            (((720 as u64) / (72 as u64)) as u32) as i32,
        )
    };
}

// /* Raw numeric value stored in s */
// /* An overflow has occurred */
// /* Display subsecond precision */
// /* Time is known to be UTC */
// /* Time is known to be localtime */
// /*
// ** Convert zDate into one or more integers according to the conversion
// ** specifier zFormat.
// **
// ** zFormat[] contains 4 characters for each integer converted, except for
// ** the last integer which is specified by three characters.  The meaning
// ** of a four-character format specifiers ABCD is:
// **
// **    A:   number of digits to convert.  Always "2" or "4".
// **    B:   minimum value.  Always "0" or "1".
// **    C:   maximum value, decoded as:
// **           a:  12
// **           b:  14
// **           c:  24
// **           d:  31
// **           e:  59
// **           f:  9999
// **    D:   the separator character, or \000 to indicate this is the
// **         last number to convert.
// **
// ** Example:  To translate an ISO-8601 date YYYY-MM-DD, the format would
// ** be "40f-21a-20c".  The "40f-" indicates the 4-digit year followed by "-".
// ** The "21a-" indicates the 2-digit month followed by "-".  The "20c" indicates
// ** the 2-digit day which is the last integer in the set.
// **
// ** The function returns the number of successful conversions.
// */
unsafe extern "C-unwind" fn getDigits(
    mut zDate: *const i8,
    mut zFormat: *const i8,
    mut __va_args: ...
) -> i32 {
    let mut __slate_storage_718: std::mem::MaybeUninit<*const i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_718: *mut *const i8 =
        std::ptr::addr_of_mut!(__slate_storage_718) as *mut *const i8;
    let mut __slate_storage_717: std::mem::MaybeUninit<*const i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_717: *mut *const i8 =
        std::ptr::addr_of_mut!(__slate_storage_717) as *mut *const i8;
    let mut __slate_storage_716: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_716: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_716) as *mut i32;
    let mut __slate_storage_715: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_715: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_715) as *mut i32;
    let mut __slate_storage_714: std::mem::MaybeUninit<*const i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_714: *mut *const i8 =
        std::ptr::addr_of_mut!(__slate_storage_714) as *mut *const i8;
    let mut __slate_storage_713: std::mem::MaybeUninit<*const i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_713: *mut *const i8 =
        std::ptr::addr_of_mut!(__slate_storage_713) as *mut *const i8;
    let mut __slate_storage_710: std::mem::MaybeUninit<i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_710: *mut i8 = std::ptr::addr_of_mut!(__slate_storage_710) as *mut i8;
    let mut __slate_storage_709: std::mem::MaybeUninit<i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_709: *mut i8 = std::ptr::addr_of_mut!(__slate_storage_709) as *mut i8;
    let mut __slate_storage_712: std::mem::MaybeUninit<*const i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_712: *mut *const i8 =
        std::ptr::addr_of_mut!(__slate_storage_712) as *mut *const i8;
    let mut __slate_storage_711: std::mem::MaybeUninit<*const i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_711: *mut *const i8 =
        std::ptr::addr_of_mut!(__slate_storage_711) as *mut *const i8;
    let mut __slate_storage_355: std::mem::MaybeUninit<u16> = std::mem::MaybeUninit::uninit();
    let __slate_slot_355: *mut u16 = std::ptr::addr_of_mut!(__slate_storage_355) as *mut u16;
    let mut __slate_storage_354: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_354: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_354) as *mut i32;
    let mut __slate_storage_353: std::mem::MaybeUninit<i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_353: *mut i8 = std::ptr::addr_of_mut!(__slate_storage_353) as *mut i8;
    let mut __slate_storage_352: std::mem::MaybeUninit<i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_352: *mut i8 = std::ptr::addr_of_mut!(__slate_storage_352) as *mut i8;
    let mut __slate_storage_351: std::mem::MaybeUninit<i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_351: *mut i8 = std::ptr::addr_of_mut!(__slate_storage_351) as *mut i8;
    let mut __slate_storage_350: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_350: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_350) as *mut i32;
    let mut __slate_storage_349: std::mem::MaybeUninit<core::ffi::VaList<'_>> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_349: *mut core::ffi::VaList<'_> =
        std::ptr::addr_of_mut!(__slate_storage_349) as *mut core::ffi::VaList<'_>;
    unsafe {
        // /* The aMx[] array translates the 3rd character of each format
        //   ** spec into a max size:    a   b   c   d   e      f */
        std::ptr::write(__slate_slot_350, 0 as i32);
        *__slate_slot_349 = __va_args.clone();
        '__loop_6: loop {
            std::ptr::write(
                __slate_slot_352,
                (((unsafe { *unsafe { zFormat.offset((0 as i32) as isize) } }) as i32)
                    - (48 as i32)) as i8,
            );
            std::ptr::write(
                __slate_slot_353,
                (((unsafe { *unsafe { zFormat.offset((1 as i32) as isize) } }) as i32)
                    - (48 as i32)) as i8,
            );
            std::ptr::write(__slate_slot_354, 0 as i32);
            0 as i32;
            *__slate_slot_355 = unsafe {
                *unsafe {
                    unsafe { std::ptr::addr_of!(aMx) as *const u16 }.offset(
                        (((unsafe { *unsafe { zFormat.offset((2 as i32) as isize) } }) as i32)
                            - (97 as i32)) as isize,
                    )
                }
            };
            *__slate_slot_351 = unsafe { *unsafe { zFormat.offset((3 as i32) as isize) } };
            *__slate_slot_354 = 0 as i32;
            loop {
                std::ptr::write(__slate_slot_709, *__slate_slot_352);
                std::ptr::write(
                    __slate_slot_710,
                    ((*__slate_slot_709 as i32) - (1 as i32)) as i8,
                );
                *__slate_slot_352 = *__slate_slot_710;
                if *__slate_slot_709 != (0 as i8) {
                    if !((((unsafe {
                        *unsafe {
                            unsafe { std::ptr::addr_of!(sqlite3CtypeMap) as *const u8 }
                                .offset(((((unsafe { *zDate }) as u8) as u32) as i32) as isize)
                        }
                    }) as u32) as i32)
                        & (4 as i32)
                        != (0 as i32))
                    {
                        break '__loop_6;
                    } else {
                        *__slate_slot_354 = *__slate_slot_354 * (10 as i32)
                            + ((unsafe { *zDate }) as i32)
                            - (48 as i32);
                        std::ptr::write(__slate_slot_711, zDate);
                        std::ptr::write(__slate_slot_712, unsafe {
                            (*__slate_slot_711).offset((1 as i32) as isize)
                        });
                        zDate = *__slate_slot_712;
                    }
                } else {
                    break;
                }
            }
            if *__slate_slot_354 < (*__slate_slot_353 as i32)
                || *__slate_slot_354 > ((*__slate_slot_355 as u32) as i32)
                || (*__slate_slot_351 as i32) != (0 as i32)
                    && (*__slate_slot_351 as i32) != ((unsafe { *zDate }) as i32)
            {
                break;
            } else {
                unsafe {
                    *unsafe { (*__slate_slot_349).next_arg::<*mut i32>() } = *__slate_slot_354;
                }
                std::ptr::write(__slate_slot_713, zDate);
                std::ptr::write(__slate_slot_714, unsafe {
                    (*__slate_slot_713).offset((1 as i32) as isize)
                });
                zDate = *__slate_slot_714;
                std::ptr::write(__slate_slot_715, *__slate_slot_350);
                std::ptr::write(__slate_slot_716, *__slate_slot_715 + (1 as i32));
                *__slate_slot_350 = *__slate_slot_716;
                std::ptr::write(__slate_slot_717, zFormat);
                std::ptr::write(__slate_slot_718, unsafe {
                    (*__slate_slot_717).offset((4 as i32) as isize)
                });
                zFormat = *__slate_slot_718;
                if !(*__slate_slot_351 != (0 as i8)) {
                    break;
                }
            }
        }
        {}
        return *__slate_slot_350;
    }
    return unsafe { std::mem::zeroed() };
}

// /*
// ** Parse a timezone extension on the end of a date-time.
// ** The extension is of the form:
// **
// **        (+/-)HH:MM
// **
// ** Or the "zulu" notation:
// **
// **        Z
// **
// ** If the parse is successful, write the number of minutes
// ** of change in p->tz and return 0.  If a parser error occurs,
// ** return non-zero.
// **
// ** A missing specifier is not considered an error.
// */
fn parseTimezone(mut zDate: *const i8, mut p: *mut DateTime) -> i32 {
    let mut __slate_storage_728: std::mem::MaybeUninit<*const i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_728: *mut *const i8 =
        std::ptr::addr_of_mut!(__slate_storage_728) as *mut *const i8;
    let mut __slate_storage_727: std::mem::MaybeUninit<*const i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_727: *mut *const i8 =
        std::ptr::addr_of_mut!(__slate_storage_727) as *mut *const i8;
    let mut __slate_storage_726: std::mem::MaybeUninit<*const i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_726: *mut *const i8 =
        std::ptr::addr_of_mut!(__slate_storage_726) as *mut *const i8;
    let mut __slate_storage_725: std::mem::MaybeUninit<*const i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_725: *mut *const i8 =
        std::ptr::addr_of_mut!(__slate_storage_725) as *mut *const i8;
    let mut __slate_storage_724: std::mem::MaybeUninit<*const i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_724: *mut *const i8 =
        std::ptr::addr_of_mut!(__slate_storage_724) as *mut *const i8;
    let mut __slate_storage_723: std::mem::MaybeUninit<*const i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_723: *mut *const i8 =
        std::ptr::addr_of_mut!(__slate_storage_723) as *mut *const i8;
    let mut __slate_storage_722: std::mem::MaybeUninit<*const i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_722: *mut *const i8 =
        std::ptr::addr_of_mut!(__slate_storage_722) as *mut *const i8;
    let mut __slate_storage_721: std::mem::MaybeUninit<*const i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_721: *mut *const i8 =
        std::ptr::addr_of_mut!(__slate_storage_721) as *mut *const i8;
    let mut __slate_storage_720: std::mem::MaybeUninit<*const i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_720: *mut *const i8 =
        std::ptr::addr_of_mut!(__slate_storage_720) as *mut *const i8;
    let mut __slate_storage_719: std::mem::MaybeUninit<*const i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_719: *mut *const i8 =
        std::ptr::addr_of_mut!(__slate_storage_719) as *mut *const i8;
    let mut __slate_storage_363: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_363: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_363) as *mut i32;
    let mut __slate_storage_362: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_362: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_362) as *mut i32;
    let mut __slate_storage_361: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_361: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_361) as *mut i32;
    let mut __slate_storage_360: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_360: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_360) as *mut i32;
    unsafe {
        std::ptr::write(__slate_slot_360, 0 as i32);
        loop {
            if (((unsafe {
                *unsafe {
                    unsafe { std::ptr::addr_of!(sqlite3CtypeMap) as *const u8 }
                        .offset(((((unsafe { *zDate }) as u8) as u32) as i32) as isize)
                }
            }) as u32) as i32)
                & (1 as i32)
                != (0 as i32)
            {
                std::ptr::write(__slate_slot_719, zDate);
                std::ptr::write(__slate_slot_720, unsafe {
                    (*__slate_slot_719).offset((1 as i32) as isize)
                });
                zDate = *__slate_slot_720;
            } else {
                break;
            }
        }
        '__join_1: {
            unsafe {
                (*p).tz = 0 as i32;
            }
            *__slate_slot_363 = (unsafe { *zDate }) as i32;
            if *__slate_slot_363 == (45 as i32) {
                *__slate_slot_360 = -(1 as i32);
            } else {
                if *__slate_slot_363 == (43 as i32) {
                    *__slate_slot_360 = 1 as i32;
                } else {
                    if *__slate_slot_363 == (90 as i32) || *__slate_slot_363 == (122 as i32) {
                        std::ptr::write(__slate_slot_721, zDate);
                        std::ptr::write(__slate_slot_722, unsafe {
                            (*__slate_slot_721).offset((1 as i32) as isize)
                        });
                        zDate = *__slate_slot_722;
                        unsafe {
                            (*p).__slate_bits_0.__set_isLocal((0 as i32) as u32);
                        }
                        unsafe {
                            (*p).__slate_bits_0.__set_isUtc((1 as i32) as u32);
                        }
                        break '__join_1;
                    } else {
                        return (*__slate_slot_363 != (0 as i32)) as i32;
                    }
                }
            }
            std::ptr::write(__slate_slot_723, zDate);
            std::ptr::write(__slate_slot_724, unsafe {
                (*__slate_slot_723).offset((1 as i32) as isize)
            });
            zDate = *__slate_slot_724;
            if (unsafe {
                getDigits(
                    zDate,
                    (b"20b:20e\0".as_ptr() as *mut i8) as *const i8,
                    std::ptr::addr_of_mut!(*__slate_slot_361),
                    std::ptr::addr_of_mut!(*__slate_slot_362),
                )
            }) != (2 as i32)
            {
                return 1 as i32;
            } else {
                std::ptr::write(__slate_slot_725, zDate);
                std::ptr::write(__slate_slot_726, unsafe {
                    (*__slate_slot_725).offset((5 as i32) as isize)
                });
                zDate = *__slate_slot_726;
                unsafe {
                    (*p).tz =
                        *__slate_slot_360 * (*__slate_slot_362 + *__slate_slot_361 * (60 as i32));
                }
                // /* Forum post 2025-09-17T10:12:14z */
                if (unsafe { (*p).tz }) == (0 as i32) {
                    unsafe {
                        (*p).__slate_bits_0.__set_isLocal((0 as i32) as u32);
                    }
                    unsafe {
                        (*p).__slate_bits_0.__set_isUtc((1 as i32) as u32);
                    }
                }
            }
        }
        loop {
            if (((unsafe {
                *unsafe {
                    unsafe { std::ptr::addr_of!(sqlite3CtypeMap) as *const u8 }
                        .offset(((((unsafe { *zDate }) as u8) as u32) as i32) as isize)
                }
            }) as u32) as i32)
                & (1 as i32)
                != (0 as i32)
            {
                std::ptr::write(__slate_slot_727, zDate);
                std::ptr::write(__slate_slot_728, unsafe {
                    (*__slate_slot_727).offset((1 as i32) as isize)
                });
                zDate = *__slate_slot_728;
            } else {
                break;
            }
        }
        return (((unsafe { *zDate }) as i32) != (0 as i32)) as i32;
    }
    return unsafe { std::mem::zeroed() };
}

// /*
// ** Parse times of the form HH:MM or HH:MM:SS or HH:MM:SS.FFFF.
// ** The HH, MM, and SS must each be exactly 2 digits.  The
// ** fractional seconds FFFF can be one or more digits.
// **
// ** Return 1 if there is a parsing error and 0 on success.
// */
fn parseHhMmSs(mut zDate: *const i8, mut p: *mut DateTime) -> i32 {
    let mut h: i32 = 0 as i32;
    let mut m: i32 = 0 as i32;
    let mut s: i32 = 0 as i32;
    let mut ms: f64 = 0.0f64;
    if (unsafe {
        getDigits(
            zDate,
            (b"20c:20e\0".as_ptr() as *mut i8) as *const i8,
            std::ptr::addr_of_mut!(h),
            std::ptr::addr_of_mut!(m),
        )
    }) != (2 as i32)
    {
        return 1 as i32;
    }
    let __v729: *const i8 = zDate;
    let __v730: *const i8 = unsafe { __v729.offset((5 as i32) as isize) };
    zDate = __v730;
    if ((unsafe { *zDate }) as i32) == (58 as i32) {
        let __v731: *const i8 = zDate;
        let __v732: *const i8 = unsafe { __v731.offset((1 as i32) as isize) };
        zDate = __v732;
        if (unsafe {
            getDigits(
                zDate,
                (b"20e\0".as_ptr() as *mut i8) as *const i8,
                std::ptr::addr_of_mut!(s),
            )
        }) != (1 as i32)
        {
            return 1 as i32;
        }
        let __v733: *const i8 = zDate;
        let __v734: *const i8 = unsafe { __v733.offset((2 as i32) as isize) };
        zDate = __v734;
        if ((unsafe { *zDate }) as i32) == (46 as i32)
            && (((unsafe {
                *unsafe {
                    unsafe { std::ptr::addr_of!(sqlite3CtypeMap) as *const u8 }.offset(
                        ((((unsafe { *unsafe { zDate.offset((1 as i32) as isize) } }) as u8) as u32)
                            as i32) as isize,
                    )
                }
            }) as u32) as i32)
                & (4 as i32)
                != (0 as i32)
        {
            let mut rScale: f64 = 1.0f64;
            let __v735: *const i8 = zDate;
            let __v736: *const i8 = unsafe { __v735.offset((1 as i32) as isize) };
            zDate = __v736;
            '__slate_break_629: while (((unsafe {
                *unsafe {
                    unsafe { std::ptr::addr_of!(sqlite3CtypeMap) as *const u8 }
                        .offset(((((unsafe { *zDate }) as u8) as u32) as i32) as isize)
                }
            }) as u32) as i32)
                & (4 as i32)
                != (0 as i32)
            {
                ms = ms * 10.0f64 + (((unsafe { *zDate }) as i32) as f64) - ((48 as i32) as f64);
                let __v737: f64 = rScale;
                let __v738: f64 = __v737 * 10.0f64;
                rScale = __v738;
                let __v739: *const i8 = zDate;
                let __v740: *const i8 = unsafe { __v739.offset((1 as i32) as isize) };
                zDate = __v740;
            }
            let __v741: f64 = ms;
            let __v742: f64 = __v741 / rScale;
            ms = __v742;
            // /* Truncate to avoid problems with sub-milliseconds
            //       ** rounding. https://sqlite.org/forum/forumpost/766a2c9231 */
            if ms > 0.999f64 {
                ms = 0.999f64;
            }
        }
    } else {
        s = 0 as i32;
    }
    unsafe {
        (*p).validJD = (0 as i32) as i8;
    }
    unsafe {
        (*p).__slate_bits_0.__set_rawS((0 as i32) as u32);
    }
    unsafe {
        (*p).validHMS = (1 as i32) as i8;
    }
    unsafe {
        (*p).h = h;
    }
    unsafe {
        (*p).m = m;
    }
    unsafe {
        (*p).s = (s as f64) + ms;
    }
    if parseTimezone(zDate, p) != (0 as i32) {
        return 1 as i32;
    }
    return 0 as i32;
}

// /*
// ** Put the DateTime object into its error state.
// */
fn datetimeError(mut p: *mut DateTime) {
    unsafe { memset(p as *mut (), 0 as i32, 48 as u64) };
    unsafe {
        (*p).__slate_bits_0.__set_isError((1 as i32) as u32);
    }
}

// /*
// ** Convert from YYYY-MM-DD HH:MM:SS to julian day.  We always assume
// ** that the YYYY-MM-DD is according to the Gregorian calendar.
// **
// ** Reference:  Meeus page 61
// */
fn computeJD(mut p: *mut DateTime) {
    let mut Y: i32 = 0 as i32;
    let mut M: i32 = 0 as i32;
    let mut D: i32 = 0 as i32;
    let mut A: i32 = 0 as i32;
    let mut B: i32 = 0 as i32;
    let mut X1: i32 = 0 as i32;
    let mut X2: i32 = 0 as i32;
    if (unsafe { (*p).validJD }) != (0 as i8) {
        return;
    }
    if (unsafe { (*p).validYMD }) != (0 as i8) {
        Y = unsafe { (*p).Y };
        M = unsafe { (*p).M };
        D = unsafe { (*p).D };
    } else {
        // /* If no YMD specified, assume 2000-Jan-01 */
        Y = 2000 as i32;
        M = 1 as i32;
        D = 1 as i32;
    }
    if Y < -(4713 as i32)
        || Y > (9999 as i32)
        || ((unsafe { (*p).__slate_bits_0.__get_rawS() }) as i32) != (0 as i32)
    {
        datetimeError(p);
        return;
    }
    if M <= (2 as i32) {
        let __v743: i32 = Y;
        let __v744: i32 = __v743 - (1 as i32);
        Y = __v744;
        let __v745: i32 = M;
        let __v746: i32 = __v745 + (12 as i32);
        M = __v746;
    }
    A = (Y + (4800 as i32)) / (100 as i32);
    B = (38 as i32) - A + A / (4 as i32);
    X1 = (36525 as i32) * (Y + (4716 as i32)) / (100 as i32);
    X2 = (306001 as i32) * (M + (1 as i32)) / (10000 as i32);
    unsafe {
        (*p).iJD = ((((X1 + X2 + D + B) as f64) - 1524.5f64) * ((86400000 as i32) as f64)) as i64;
    }
    unsafe {
        (*p).validJD = (1 as i32) as i8;
    }
    if (unsafe { (*p).validHMS }) != (0 as i8) {
        let __v747: *mut DateTime = p;
        let __v748: i64 = unsafe { (*__v747).iJD };
        let __v749: i64 = __v748
            + ((((unsafe { (*p).h }) * (3600000 as i32) + (unsafe { (*p).m }) * (60000 as i32))
                as i64)
                + (((unsafe { (*p).s }) * ((1000 as i32) as f64) + 0.5f64) as i64));
        unsafe {
            (*__v747).iJD = __v749;
        }
        if (unsafe { (*p).tz }) != (0 as i32) {
            let __v750: *mut DateTime = p;
            let __v751: i64 = unsafe { (*__v750).iJD };
            let __v752: i64 = __v751 - (((unsafe { (*p).tz }) * (60000 as i32)) as i64);
            unsafe {
                (*__v750).iJD = __v752;
            }
            unsafe {
                (*p).validYMD = (0 as i32) as i8;
            }
            unsafe {
                (*p).validHMS = (0 as i32) as i8;
            }
            unsafe {
                (*p).tz = 0 as i32;
            }
            unsafe {
                (*p).__slate_bits_0.__set_isUtc((1 as i32) as u32);
            }
            unsafe {
                (*p).__slate_bits_0.__set_isLocal((0 as i32) as u32);
            }
        }
    }
}

// /*
// ** Given the YYYY-MM-DD information current in p, determine if there
// ** is day-of-month overflow and set nFloor to the number of days that
// ** would need to be subtracted from the date in order to bring the
// ** date back to the end of the month.
// */
fn computeFloor(mut p: *mut DateTime) {
    0 as i32;
    0 as i32;
    0 as i32;
    if (unsafe { (*p).D }) <= (28 as i32) {
        unsafe {
            (*p).nFloor = (0 as i32) as i8;
        }
    } else {
        if (1 as i32) << unsafe { (*p).M } & (5546 as i32) != (0 as i32) {
            unsafe {
                (*p).nFloor = (0 as i32) as i8;
            }
        } else {
            if (unsafe { (*p).M }) != (2 as i32) {
                unsafe {
                    (*p).nFloor = ((unsafe { (*p).D }) == (31 as i32)) as i8;
                }
            } else {
                if (unsafe { (*p).Y }) % (4 as i32) != (0 as i32)
                    || (unsafe { (*p).Y }) % (100 as i32) == (0 as i32)
                        && (unsafe { (*p).Y }) % (400 as i32) != (0 as i32)
                {
                    unsafe {
                        (*p).nFloor = ((unsafe { (*p).D }) - (28 as i32)) as i8;
                    }
                } else {
                    unsafe {
                        (*p).nFloor = ((unsafe { (*p).D }) - (29 as i32)) as i8;
                    }
                }
            }
        }
    }
}

// /*
// ** Parse dates of the form
// **
// **     YYYY-MM-DD HH:MM:SS.FFF
// **     YYYY-MM-DD HH:MM:SS
// **     YYYY-MM-DD HH:MM
// **     YYYY-MM-DD
// **
// ** Write the result into the DateTime structure and return 0
// ** on success and 1 if the input string is not a well-formed
// ** date.
// */
fn parseYyyyMmDd(mut zDate: *const i8, mut p: *mut DateTime) -> i32 {
    let mut Y: i32 = 0 as i32;
    let mut M: i32 = 0 as i32;
    let mut D: i32 = 0 as i32;
    let mut neg: i32 = 0 as i32;
    if ((unsafe { *unsafe { zDate.offset((0 as i32) as isize) } }) as i32) == (45 as i32) {
        let __v753: *const i8 = zDate;
        let __v754: *const i8 = unsafe { __v753.offset((1 as i32) as isize) };
        zDate = __v754;
        neg = 1 as i32;
    } else {
        neg = 0 as i32;
    }
    if (unsafe {
        getDigits(
            zDate,
            (b"40f-21a-21d\0".as_ptr() as *mut i8) as *const i8,
            std::ptr::addr_of_mut!(Y),
            std::ptr::addr_of_mut!(M),
            std::ptr::addr_of_mut!(D),
        )
    }) != (3 as i32)
    {
        return 1 as i32;
    }
    let __v755: *const i8 = zDate;
    let __v756: *const i8 = unsafe { __v755.offset((10 as i32) as isize) };
    zDate = __v756;
    '__slate_break_631: while (((unsafe {
        *unsafe {
            unsafe { std::ptr::addr_of!(sqlite3CtypeMap) as *const u8 }
                .offset(((((unsafe { *zDate }) as u8) as u32) as i32) as isize)
        }
    }) as u32) as i32)
        & (1 as i32)
        != (0 as i32)
        || (84 as i32) == (((unsafe { *(zDate as *mut u8) }) as u32) as i32)
    {
        let __v757: *const i8 = zDate;
        let __v758: *const i8 = unsafe { __v757.offset((1 as i32) as isize) };
        zDate = __v758;
    }
    if parseHhMmSs(zDate, p) == (0 as i32) {
        // /* We got the time */
    } else {
        if ((unsafe { *zDate }) as i32) == (0 as i32) {
            unsafe {
                (*p).validHMS = (0 as i32) as i8;
            }
        } else {
            return 1 as i32;
        }
    }
    unsafe {
        (*p).validJD = (0 as i32) as i8;
    }
    unsafe {
        (*p).validYMD = (1 as i32) as i8;
    }
    unsafe {
        (*p).Y = if neg != (0 as i32) { -Y } else { Y };
    }
    unsafe {
        (*p).M = M;
    }
    unsafe {
        (*p).D = D;
    }
    computeFloor(p);
    if (unsafe { (*p).tz }) != (0 as i32) {
        computeJD(p);
    }
    return 0 as i32;
}

// /*
// ** Clear the YMD and HMS and the TZ
// */
fn clearYMD_HMS_TZ(mut p: *mut DateTime) {
    unsafe {
        (*p).validYMD = (0 as i32) as i8;
    }
    unsafe {
        (*p).validHMS = (0 as i32) as i8;
    }
    unsafe {
        (*p).tz = 0 as i32;
    }
}

// /* Forward declaration */
// /*
// ** Set the time to the current time reported by the VFS.
// **
// ** Return the number of errors.
// */
fn setDateTimeToCurrent(mut context: *mut sqlite3_context, mut p: *mut DateTime) -> i32 {
    unsafe {
        (*p).iJD = unsafe { sqlite3StmtCurrentTime(context) };
    }
    if (unsafe { (*p).iJD }) > ((0 as i32) as i64) {
        unsafe {
            (*p).validJD = (1 as i32) as i8;
        }
        unsafe {
            (*p).__slate_bits_0.__set_isUtc((1 as i32) as u32);
        }
        unsafe {
            (*p).__slate_bits_0.__set_isLocal((0 as i32) as u32);
        }
        clearYMD_HMS_TZ(p);
        return 0 as i32;
    } else {
        return 1 as i32;
    }
    return unsafe { std::mem::zeroed() };
}

// /*
// ** Input "r" is a numeric quantity which might be a julian day number,
// ** or the number of seconds since 1970.  If the value if r is within
// ** range of a julian day number, install it as such and set validJD.
// ** If the value is a valid unix timestamp, put it in p->s and set p->rawS.
// */
fn setRawDateNumber(mut p: *mut DateTime, mut r: f64) {
    unsafe {
        (*p).s = r;
    }
    unsafe {
        (*p).__slate_bits_0.__set_rawS((1 as i32) as u32);
    }
    if r >= 0.0f64 && r < 5373484.5f64 {
        unsafe {
            (*p).iJD = (r * 86400000.0f64 + 0.5f64) as i64;
        }
        unsafe {
            (*p).validJD = (1 as i32) as i8;
        }
    }
}

// /*
// ** Attempt to parse the given string into a julian day number.  Return
// ** the number of errors.
// **
// ** The following are acceptable forms for the input string:
// **
// **      YYYY-MM-DD HH:MM:SS.FFF  +/-HH:MM
// **      DDDD.DD
// **      now
// **
// ** In the first form, the +/-HH:MM is always optional.  The fractional
// ** seconds extension (the ".FFF") is optional.  The seconds portion
// ** (":SS.FFF") is option.  The year and date can be omitted as long
// ** as there is a time string.  The time string can be omitted as long
// ** as there is a year and date.
// */
fn parseDateOrTime(
    mut context: *mut sqlite3_context,
    mut zDate: *const i8,
    mut p: *mut DateTime,
) -> i32 {
    let mut r: f64 = 0 as f64;
    if parseYyyyMmDd(zDate, p) == (0 as i32) {
        return 0 as i32;
    } else {
        if parseHhMmSs(zDate, p) == (0 as i32) {
            return 0 as i32;
        } else {
            let __v759: bool;
            if (unsafe { sqlite3StrICmp(zDate, (b"now\0".as_ptr() as *mut i8) as *const i8) })
                == (0 as i32)
            {
                __v759 = (unsafe { sqlite3NotPureFunc(context) }) != (0 as i32);
            } else {
                __v759 = false as bool;
            }
            if __v759 {
                return setDateTimeToCurrent(context, p);
            } else {
                if (unsafe { sqlite3AtoF(zDate, std::ptr::addr_of_mut!(r)) }) > (0 as i32) {
                    setRawDateNumber(p, r);
                    return 0 as i32;
                } else {
                    let __v760: bool;
                    if (unsafe {
                        sqlite3StrICmp(zDate, (b"subsec\0".as_ptr() as *mut i8) as *const i8)
                    }) == (0 as i32)
                    {
                        __v760 = true as bool;
                    } else {
                        __v760 = (unsafe {
                            sqlite3StrICmp(zDate, (b"subsecond\0".as_ptr() as *mut i8) as *const i8)
                        }) == (0 as i32);
                    }
                    let __v761: bool;
                    if __v760 {
                        __v761 = (unsafe { sqlite3NotPureFunc(context) }) != (0 as i32);
                    } else {
                        __v761 = false as bool;
                    }
                    if __v761 {
                        unsafe {
                            (*p).__slate_bits_0.__set_useSubsec((1 as i32) as u32);
                        }
                        return setDateTimeToCurrent(context, p);
                    }
                }
            }
        }
    }
    return 1 as i32;
}

// /* The julian day number for 9999-12-31 23:59:59.999 is 5373484.4999999.
// ** Multiplying this by 86400000 gives 464269060799999 as the maximum value
// ** for DateTime.iJD.
// **
// ** But some older compilers (ex: gcc 4.2.1 on older Macs) cannot deal with
// ** such a large integer literal, so we have to encode it.
// */
// /*
// ** Return TRUE if the given julian day number is within range.
// **
// ** The input is the JulianDay times 86400000.
// */
fn validJulianDay(mut iJD: i64) -> i32 {
    return (iJD >= ((0 as i32) as i64)
        && iJD <= ((108096 as i32) as i64) << (32 as i32) | ((275971583 as i32) as i64))
        as i32;
}

// /*
// ** Compute the Year, Month, and Day from the julian day number.
// */
fn computeYMD(mut p: *mut DateTime) {
    let mut Z: i32 = 0 as i32;
    let mut alpha: i32 = 0 as i32;
    let mut A: i32 = 0 as i32;
    let mut B: i32 = 0 as i32;
    let mut C: i32 = 0 as i32;
    let mut D: i32 = 0 as i32;
    let mut E: i32 = 0 as i32;
    let mut X1: i32 = 0 as i32;
    if (unsafe { (*p).validYMD }) != (0 as i8) {
        return;
    }
    if !((unsafe { (*p).validJD }) != (0 as i8)) {
        unsafe {
            (*p).Y = 2000 as i32;
        }
        unsafe {
            (*p).M = 1 as i32;
        }
        unsafe {
            (*p).D = 1 as i32;
        }
    } else {
        if !(validJulianDay(unsafe { (*p).iJD }) != (0 as i32)) {
            datetimeError(p);
            return;
        } else {
            Z = (((unsafe { (*p).iJD }) + ((43200000 as i32) as i64)) / ((86400000 as i32) as i64))
                as i32;
            alpha = ((((Z as f64) + 32044.75f64) / 36524.25f64) as i32) - (52 as i32);
            A = Z + (1 as i32) + alpha - (alpha + (100 as i32)) / (4 as i32) + (25 as i32);
            B = A + (1524 as i32);
            C = (((B as f64) - 122.1f64) / 365.25f64) as i32;
            D = (36525 as i32) * (C & (32767 as i32)) / (100 as i32);
            E = (((B - D) as f64) / 30.6001f64) as i32;
            X1 = (30.6001f64 * (E as f64)) as i32;
            unsafe {
                (*p).D = B - D - X1;
            }
            unsafe {
                (*p).M = if E < (14 as i32) {
                    E - (1 as i32)
                } else {
                    E - (13 as i32)
                };
            }
            unsafe {
                (*p).Y = if (unsafe { (*p).M }) > (2 as i32) {
                    C - (4716 as i32)
                } else {
                    C - (4715 as i32)
                };
            }
        }
    }
    unsafe {
        (*p).validYMD = (1 as i32) as i8;
    }
}

// /*
// ** Compute the Hour, Minute, and Seconds from the julian day number.
// */
fn computeHMS(mut p: *mut DateTime) {
    // /* milliseconds, minutes into the day */
    let mut day_ms: i32 = 0 as i32;
    let mut day_min: i32 = 0 as i32;
    if (unsafe { (*p).validHMS }) != (0 as i8) {
        return;
    }
    computeJD(p);
    day_ms =
        (((unsafe { (*p).iJD }) + ((43200000 as i32) as i64)) % ((86400000 as i32) as i64)) as i32;
    unsafe {
        (*p).s = ((day_ms % (60000 as i32)) as f64) / 1000.0f64;
    }
    day_min = day_ms / (60000 as i32);
    unsafe {
        (*p).m = day_min % (60 as i32);
    }
    unsafe {
        (*p).h = day_min / (60 as i32);
    }
    unsafe {
        (*p).__slate_bits_0.__set_rawS((0 as i32) as u32);
    }
    unsafe {
        (*p).validHMS = (1 as i32) as i8;
    }
}

// /*
// ** Compute both YMD and HMS
// */
fn computeYMD_HMS(mut p: *mut DateTime) {
    computeYMD(p);
    computeHMS(p);
}

// /*
// ** On recent Windows platforms, the localtime_s() function is available
// ** as part of the "Secure CRT". It is essentially equivalent to
// ** localtime_r() available under most POSIX platforms, except that the
// ** order of the parameters is reversed.
// **
// ** See http://msdn.microsoft.com/en-us/library/a442x3ye(VS.80).aspx.
// **
// ** If the user has not indicated to use localtime_r() or localtime_s()
// ** already, check for an MSVC build environment that provides
// ** localtime_s().
// */
// /*
// ** The following routine implements the rough equivalent of localtime_r()
// ** using whatever operating-system specific localtime facility that
// ** is available.  This routine returns 0 on success and
// ** non-zero on any kind of error.
// **
// ** If the sqlite3GlobalConfig.bLocaltimeFault variable is non-zero then this
// ** routine will always fail.  If bLocaltimeFault is nonzero and
// ** sqlite3GlobalConfig.xAltLocaltime is not NULL, then xAltLocaltime() is
// ** invoked in place of the OS-defined localtime() function.
// **
// ** EVIDENCE-OF: R-62172-00036 In this implementation, the standard C
// ** library function localtime_r() is used to assist in the calculation of
// ** local time.
// */
fn osLocaltime(mut t: *mut i64, mut pTm: *mut tm) -> i32 {
    let mut rc: i32 = 0 as i32;
    if (unsafe { sqlite3Config.bLocaltimeFault }) != (0 as i32) {
        if (unsafe { sqlite3Config.xAltLocaltime }) != None {
            return unsafe {
                unsafe { sqlite3Config.xAltLocaltime }.unwrap()(t as *const (), pTm as *mut ())
            };
        } else {
            return 1 as i32;
        }
    }
    rc = ((unsafe { localtime_r(t as *const i64, pTm) }) == std::ptr::null_mut::<tm>()) as i32;
    // /* HAVE_LOCALTIME_R || HAVE_LOCALTIME_S */
    return rc;
}

// /* SQLITE_OMIT_LOCALTIME */
// /*
// ** Assuming the input DateTime is UTC, move it to its localtime equivalent.
// */
fn toLocaltime(mut p: *mut DateTime, mut pCtx: *mut sqlite3_context) -> i32 {
    let mut t: i64 = 0 as i64;
    let mut sLocal: tm = unsafe { std::mem::zeroed() };
    let mut iYearDiff: i32 = 0 as i32;
    // /* Initialize the contents of sLocal to avoid a compiler warning. */
    unsafe {
        memset(
            std::ptr::addr_of_mut!(sLocal) as *mut (),
            0 as i32,
            56 as u64,
        )
    };
    computeJD(p);
    // /* 1970-01-01 */
    if (unsafe { (*p).iJD }) < ((2108667600 as i32) as i64) * ((100000 as i32) as i64)
        || (unsafe { (*p).iJD }) > ((2130141456 as i32) as i64) * ((100000 as i32) as i64)
    {
        // /* EVIDENCE-OF: R-55269-29598 The localtime_r() C function normally only
        //     ** works for years between 1970 and 2037. For dates outside this range,
        //     ** SQLite attempts to map the year into an equivalent year within this
        //     ** range, do the calculation, then map the year back.
        //     */
        let mut x: DateTime = unsafe { *p };
        computeYMD_HMS(std::ptr::addr_of_mut!(x));
        iYearDiff = (2000 as i32) + x.Y % (4 as i32) - x.Y;
        let __v762: i32 = x.Y;
        let __v763: i32 = __v762 + iYearDiff;
        x.Y = __v763;
        x.validJD = (0 as i32) as i8;
        computeJD(std::ptr::addr_of_mut!(x));
        t = x.iJD / ((1000 as i32) as i64) - ((21086676 as i32) as i64) * ((10000 as i32) as i64);
    } else {
        iYearDiff = 0 as i32;
        t = (unsafe { (*p).iJD }) / ((1000 as i32) as i64)
            - ((21086676 as i32) as i64) * ((10000 as i32) as i64);
    }
    // /* 2038-01-18 */
    if osLocaltime(std::ptr::addr_of_mut!(t), std::ptr::addr_of_mut!(sLocal)) != (0 as i32) {
        unsafe {
            sqlite3_result_error(
                pCtx,
                (b"local time unavailable\0".as_ptr() as *mut i8) as *const i8,
                -(1 as i32),
            )
        };
        return 1 as i32;
    }
    unsafe {
        (*p).Y = sLocal.tm_year + (1900 as i32) - iYearDiff;
    }
    unsafe {
        (*p).M = sLocal.tm_mon + (1 as i32);
    }
    unsafe {
        (*p).D = sLocal.tm_mday;
    }
    unsafe {
        (*p).h = sLocal.tm_hour;
    }
    unsafe {
        (*p).m = sLocal.tm_min;
    }
    unsafe {
        (*p).s = (sLocal.tm_sec as f64)
            + (((unsafe { (*p).iJD }) % ((1000 as i32) as i64)) as f64) * 0.001f64;
    }
    unsafe {
        (*p).validYMD = (1 as i32) as i8;
    }
    unsafe {
        (*p).validHMS = (1 as i32) as i8;
    }
    unsafe {
        (*p).validJD = (0 as i32) as i8;
    }
    unsafe {
        (*p).__slate_bits_0.__set_rawS((0 as i32) as u32);
    }
    unsafe {
        (*p).tz = 0 as i32;
    }
    unsafe {
        (*p).__slate_bits_0.__set_isError((0 as i32) as u32);
    }
    return 0 as i32;
}

// /*
// ** If the DateTime p is raw number, try to figure out if it is
// ** a julian day number of a unix timestamp.  Set the p value
// ** appropriately.
// */
fn autoAdjustDate(mut p: *mut DateTime) {
    if !(((unsafe { (*p).__slate_bits_0.__get_rawS() }) as i32) != (0 as i32))
        || (unsafe { (*p).validJD }) != (0 as i8)
    {
        unsafe {
            (*p).__slate_bits_0.__set_rawS((0 as i32) as u32);
        }
    // /* -4713-11-24 12:00:00 */
    } else {
        if (unsafe { (*p).s }) >= (((-(21086676 as i32) as i64) * ((10000 as i32) as i64)) as f64)
            && (unsafe { (*p).s })
                <= ((((25340230 as i32) as i64) * ((10000 as i32) as i64) + ((799 as i32) as i64))
                    as f64)
        {
            let mut r: f64 = (unsafe { (*p).s }) * 1000.0f64 + 210866760000000.0f64;
            clearYMD_HMS_TZ(p);
            unsafe {
                (*p).iJD = (r + 0.5f64) as i64;
            }
            unsafe {
                (*p).validJD = (1 as i32) as i8;
            }
            unsafe {
                (*p).__slate_bits_0.__set_rawS((0 as i32) as u32);
            }
        }
    }
    // /*  9999-12-31 23:59:59 */
}

// /*
// ** Process a modifier to a date-time stamp.  The modifiers are
// ** as follows:
// **
// **     NNN days
// **     NNN hours
// **     NNN minutes
// **     NNN.NNNN seconds
// **     NNN months
// **     NNN years
// **     +/-YYYY-MM-DD HH:MM:SS.SSS
// **     ceiling
// **     floor
// **     start of month
// **     start of year
// **     start of week
// **     start of day
// **     weekday N
// **     unixepoch
// **     auto
// **     localtime
// **     utc
// **     subsec
// **     subsecond
// **
// ** Return 0 on success and 1 if there is any kind of error. If the error
// ** is in a system call (i.e. localtime()), then an error message is written
// ** to context pCtx. If the error is an unrecognized modifier, no error is
// ** written to pCtx.
// */
fn parseModifier(
    mut pCtx: *mut sqlite3_context,
    mut z: *const i8,
    mut n: i32,
    mut p: *mut DateTime,
    mut idx: i32,
) -> i32 {
    let mut rc: i32 = 1 as i32;
    let mut r: f64 = 0 as f64;
    '__slate_break_637: {
        match ((unsafe {
            *unsafe {
                unsafe { std::ptr::addr_of!(sqlite3UpperToLower) as *const u8 }.offset(
                    ((((unsafe { *unsafe { z.offset((0 as i32) as isize) } }) as u8) as u32) as i32)
                        as isize,
                )
            }
        }) as u32) as i32
        {
            97 => {
                // /*
                //       **    auto
                //       **
                //       ** If rawS is available, then interpret as a julian day number, or
                //       ** a unix timestamp, depending on its magnitude.
                //       */
                if (unsafe { sqlite3_stricmp(z, (b"auto\0".as_ptr() as *mut i8) as *const i8) })
                    == (0 as i32)
                {
                    // /* IMP: R-33611-57934 */
                    if idx > (1 as i32) {
                        return 1 as i32;
                    }
                    autoAdjustDate(p);
                    rc = 0 as i32;
                }
            }
            99 => {
                // /*
                //       **    ceiling
                //       **
                //       ** Resolve day-of-month overflow by rolling forward into the next
                //       ** month.  As this is the default action, this modifier is really
                //       ** a no-op that is only included for symmetry.  See "floor".
                //       */
                if (unsafe { sqlite3_stricmp(z, (b"ceiling\0".as_ptr() as *mut i8) as *const i8) })
                    == (0 as i32)
                {
                    computeJD(p);
                    clearYMD_HMS_TZ(p);
                    rc = 0 as i32;
                    unsafe {
                        (*p).nFloor = (0 as i32) as i8;
                    }
                }
            }
            101 => {
                // /*
                //       **    end of day
                //       **    end of month
                //       **    end of year
                //       **
                //       ** Move the date forwards to the last millisecond of the current
                //       ** day, month or year.
                //       */
                if (unsafe {
                    sqlite3_strnicmp(z, (b"end of \0".as_ptr() as *mut i8) as *const i8, 7 as i32)
                }) != (0 as i32)
                {
                } else {
                    if !((unsafe { (*p).validJD }) != (0 as i8))
                        && !((unsafe { (*p).validYMD }) != (0 as i8))
                        && !((unsafe { (*p).validHMS }) != (0 as i8))
                    {
                    } else {
                        let __v764: *const i8 = z;
                        let __v765: *const i8 = unsafe { __v764.offset((7 as i32) as isize) };
                        z = __v765;
                        computeYMD(p);
                        unsafe {
                            (*p).validHMS = (1 as i32) as i8;
                        }
                        unsafe {
                            (*p).h = 23 as i32;
                        }
                        unsafe {
                            (*p).m = 59 as i32;
                        }
                        unsafe {
                            (*p).s = 59.999f64;
                        }
                        unsafe {
                            (*p).__slate_bits_0.__set_rawS((0 as i32) as u32);
                        }
                        unsafe {
                            (*p).tz = 0 as i32;
                        }
                        unsafe {
                            (*p).validJD = (0 as i32) as i8;
                        }
                        if (unsafe {
                            sqlite3_stricmp(z, (b"month\0".as_ptr() as *mut i8) as *const i8)
                        }) == (0 as i32)
                        {
                            unsafe {
                                (*p).D = 1 as i32;
                            }
                            let __v766: *mut DateTime = p;
                            let __v767: i32 = unsafe { (*__v766).M };
                            let __v768: i32 = __v767 + (1 as i32);
                            unsafe {
                                (*__v766).M = __v768;
                            }
                            if (unsafe { (*p).M }) > (12 as i32) {
                                let __v769: *mut DateTime = p;
                                let __v770: i32 = unsafe { (*__v769).Y };
                                let __v771: i32 = __v770 + (1 as i32);
                                unsafe {
                                    (*__v769).Y = __v771;
                                }
                                unsafe {
                                    (*p).M = 1 as i32;
                                }
                            }
                            computeFloor(p);
                            computeJD(p);
                            let __v772: *mut DateTime = p;
                            let __v773: i64 = unsafe { (*__v772).iJD };
                            let __v774: i64 = __v773
                                - (((((unsafe { (*p).nFloor }) as i32) + (1 as i32))
                                    * (86400000 as i32)) as i64);
                            unsafe {
                                (*__v772).iJD = __v774;
                            }
                            clearYMD_HMS_TZ(p);
                            rc = 0 as i32;
                        } else {
                            if (unsafe {
                                sqlite3_stricmp(z, (b"year\0".as_ptr() as *mut i8) as *const i8)
                            }) == (0 as i32)
                            {
                                unsafe {
                                    (*p).M = 12 as i32;
                                }
                                unsafe {
                                    (*p).D = 31 as i32;
                                }
                                rc = 0 as i32;
                            } else {
                                if (unsafe {
                                    sqlite3_stricmp(z, (b"day\0".as_ptr() as *mut i8) as *const i8)
                                }) == (0 as i32)
                                {
                                    rc = 0 as i32;
                                }
                            }
                        }
                    }
                }
            }
            102 => {
                // /*
                //       **    floor
                //       **
                //       ** Resolve day-of-month overflow by rolling back to the end of the
                //       ** previous month.
                //       */
                if (unsafe { sqlite3_stricmp(z, (b"floor\0".as_ptr() as *mut i8) as *const i8) })
                    == (0 as i32)
                {
                    computeJD(p);
                    let __v775: *mut DateTime = p;
                    let __v776: i64 = unsafe { (*__v775).iJD };
                    let __v777: i64 =
                        __v776 - ((((unsafe { (*p).nFloor }) as i32) * (86400000 as i32)) as i64);
                    unsafe {
                        (*__v775).iJD = __v777;
                    }
                    clearYMD_HMS_TZ(p);
                    rc = 0 as i32;
                }
            }
            106 => {
                // /*
                //       **    julianday
                //       **
                //       ** Always interpret the prior number as a julian-day value.  If this
                //       ** is not the first modifier, or if the prior argument is not a numeric
                //       ** value in the allowed range of julian day numbers understood by
                //       ** SQLite (0..5373484.5) then the result will be NULL.
                //       */
                if (unsafe {
                    sqlite3_stricmp(z, (b"julianday\0".as_ptr() as *mut i8) as *const i8)
                }) == (0 as i32)
                {
                    // /* IMP: R-31176-64601 */
                    if idx > (1 as i32) {
                        return 1 as i32;
                    }
                    if (unsafe { (*p).validJD }) != (0 as i8)
                        && ((unsafe { (*p).__slate_bits_0.__get_rawS() }) as i32) != (0 as i32)
                    {
                        rc = 0 as i32;
                        unsafe {
                            (*p).__slate_bits_0.__set_rawS((0 as i32) as u32);
                        }
                    }
                }
            }
            108 => {
                // /*    localtime
                //       **
                //       ** Assuming the current time value is UTC (a.k.a. GMT), shift it to
                //       ** show local time.
                //       */
                let __v778: bool;
                if (unsafe {
                    sqlite3_stricmp(z, (b"localtime\0".as_ptr() as *mut i8) as *const i8)
                }) == (0 as i32)
                {
                    __v778 = (unsafe { sqlite3NotPureFunc(pCtx) }) != (0 as i32);
                } else {
                    __v778 = false as bool;
                }
                if __v778 {
                    let __v779: i32;
                    if ((unsafe { (*p).__slate_bits_0.__get_isLocal() }) as i32) != (0 as i32) {
                        __v779 = 0 as i32;
                    } else {
                        __v779 = toLocaltime(p, pCtx);
                    }
                    rc = __v779;
                    unsafe {
                        (*p).__slate_bits_0.__set_isUtc((0 as i32) as u32);
                    }
                    unsafe {
                        (*p).__slate_bits_0.__set_isLocal((1 as i32) as u32);
                    }
                }
            }
            117 => {
                // /*
                //       **    unixepoch
                //       **
                //       ** Treat the current value of p->s as the number of
                //       ** seconds since 1970.  Convert to a real julian day number.
                //       */
                if (unsafe {
                    sqlite3_stricmp(z, (b"unixepoch\0".as_ptr() as *mut i8) as *const i8)
                }) == (0 as i32)
                    && ((unsafe { (*p).__slate_bits_0.__get_rawS() }) as i32) != (0 as i32)
                {
                    // /* IMP: R-49255-55373 */
                    if idx > (1 as i32) {
                        return 1 as i32;
                    }
                    r = (unsafe { (*p).s }) * 1000.0f64 + 210866760000000.0f64;
                    if r >= 0.0f64 && r < 464269060800000.0f64 {
                        clearYMD_HMS_TZ(p);
                        unsafe {
                            (*p).iJD = (r + 0.5f64) as i64;
                        }
                        unsafe {
                            (*p).validJD = (1 as i32) as i8;
                        }
                        unsafe {
                            (*p).__slate_bits_0.__set_rawS((0 as i32) as u32);
                        }
                        rc = 0 as i32;
                    }
                } else {
                    let __v780: bool;
                    if (unsafe { sqlite3_stricmp(z, (b"utc\0".as_ptr() as *mut i8) as *const i8) })
                        == (0 as i32)
                    {
                        __v780 = (unsafe { sqlite3NotPureFunc(pCtx) }) != (0 as i32);
                    } else {
                        __v780 = false as bool;
                    }
                    if __v780 {
                        if ((unsafe { (*p).__slate_bits_0.__get_isUtc() }) as i32) == (0 as i32) {
                            // /* Original localtime */
                            let mut iOrigJD: i64 = 0 as i64;
                            // /* Guess at the corresponding utc time */
                            let mut iGuess: i64 = 0 as i64;
                            // /* Safety to prevent infinite loop */
                            let mut cnt: i32 = 0 as i32;
                            // /* Guess is off by this much */
                            let mut iErr: i64 = 0 as i64;
                            computeJD(p);
                            let __v781: i64 = unsafe { (*p).iJD };
                            iOrigJD = __v781;
                            iGuess = __v781;
                            iErr = (0 as i32) as i64;
                            '__slate_break_649: loop {
                                let mut new: DateTime = unsafe { std::mem::zeroed() };
                                unsafe {
                                    memset(
                                        std::ptr::addr_of_mut!(new) as *mut (),
                                        0 as i32,
                                        48 as u64,
                                    )
                                };
                                let __v782: i64 = iGuess;
                                let __v783: i64 = __v782 - iErr;
                                iGuess = __v783;
                                new.iJD = iGuess;
                                new.validJD = (1 as i32) as i8;
                                rc = toLocaltime(std::ptr::addr_of_mut!(new), pCtx);
                                if rc != (0 as i32) {
                                    return rc;
                                }
                                computeJD(std::ptr::addr_of_mut!(new));
                                iErr = new.iJD - iOrigJD;
                                let __v784: bool;
                                if iErr != (0 as i64) {
                                    let __v785: i32 = cnt;
                                    let __v786: i32 = __v785 + (1 as i32);
                                    cnt = __v786;
                                    __v784 = __v785 < (3 as i32);
                                } else {
                                    __v784 = false as bool;
                                }
                                if !__v784 {
                                    break;
                                }
                            }
                            unsafe { memset(p as *mut (), 0 as i32, 48 as u64) };
                            unsafe {
                                (*p).iJD = iGuess;
                            }
                            unsafe {
                                (*p).validJD = (1 as i32) as i8;
                            }
                            unsafe {
                                (*p).__slate_bits_0.__set_isUtc((1 as i32) as u32);
                            }
                            unsafe {
                                (*p).__slate_bits_0.__set_isLocal((0 as i32) as u32);
                            }
                        }
                        rc = 0 as i32;
                    }
                }
            }
            119 => {
                // /*
                //       **    weekday N
                //       **    weekday -N
                //       **
                //       ** Move the date forward (for N>=0) or backward (for N<=-0) to the
                //       ** same time on the next/previous occurrence of weekday abs(N)
                //       ** where 0==Sunday, 1==Monday, and so forth.  If the date is already
                //       ** on the appropriate weekday, this is a no-op.
                //       */
                let __v787: bool;
                if (unsafe {
                    sqlite3_strnicmp(
                        z,
                        (b"weekday \0".as_ptr() as *mut i8) as *const i8,
                        8 as i32,
                    )
                }) == (0 as i32)
                {
                    __v787 = (unsafe {
                        sqlite3AtoF(
                            unsafe { z.offset((8 as i32) as isize) },
                            std::ptr::addr_of_mut!(r),
                        )
                    }) > (0 as i32);
                } else {
                    __v787 = false as bool;
                }
                let __v788: bool;
                if __v787 && r >= -6.0f64 && r <= 6.0f64 {
                    let __v789: i32 = r as i32;
                    n = __v789;
                    __v788 = (unsafe { sqlite3RealSameAsInt(r, __v789 as i64) }) != (0 as i32);
                } else {
                    __v788 = false as bool;
                }
                if __v788 {
                    let mut Z: i64 = 0 as i64;
                    computeYMD_HMS(p);
                    unsafe {
                        (*p).tz = 0 as i32;
                    }
                    unsafe {
                        (*p).validJD = (0 as i32) as i8;
                    }
                    computeJD(p);
                    Z = ((unsafe { (*p).iJD }) + ((129600000 as i32) as i64))
                        / ((86400000 as i32) as i64)
                        % ((7 as i32) as i64);
                    if n < (0 as i32) {
                        n = -n;
                    }
                    if Z != (n as i64) {
                        if Z > (n as i64) {
                            let __v790: i64 = Z;
                            let __v791: i64 = __v790 - ((7 as i32) as i64);
                            Z = __v791;
                        }
                        let __v792: *mut DateTime = p;
                        let __v793: i64 = unsafe { (*__v792).iJD };
                        let __v794: i64 = __v793 + ((n as i64) - Z) * ((86400000 as i32) as i64);
                        unsafe {
                            (*__v792).iJD = __v794;
                        }
                        if ((unsafe { strchr(unsafe { z.offset((8 as i32) as isize) }, 45 as i32) })
                            as *const i8)
                            != std::ptr::null::<i8>()
                        {
                            let __v795: *mut DateTime = p;
                            let __v796: i64 = unsafe { (*__v795).iJD };
                            let __v797: i64 = __v796 - (((7 as i32) * (86400000 as i32)) as i64);
                            unsafe {
                                (*__v795).iJD = __v797;
                            }
                        }
                    }
                    clearYMD_HMS_TZ(p);
                    rc = 0 as i32;
                }
            }
            115 => {
                // /*
                //       **    start of day
                //       **    start of month
                //       **    start of year
                //       **
                //       ** Move the date backwards to the beginning of the current day,
                //       ** or month or year.
                //       **
                //       **    subsecond
                //       **    subsec
                //       **
                //       ** Show subsecond precision in the output of datetime() and
                //       ** unixepoch() and strftime('%s').
                //       */
                if (unsafe {
                    sqlite3_strnicmp(
                        z,
                        (b"start of \0".as_ptr() as *mut i8) as *const i8,
                        9 as i32,
                    )
                }) != (0 as i32)
                {
                    let __v798: bool;
                    if (unsafe {
                        sqlite3_stricmp(z, (b"subsec\0".as_ptr() as *mut i8) as *const i8)
                    }) == (0 as i32)
                    {
                        __v798 = true as bool;
                    } else {
                        __v798 = (unsafe {
                            sqlite3_stricmp(z, (b"subsecond\0".as_ptr() as *mut i8) as *const i8)
                        }) == (0 as i32);
                    }
                    if __v798 {
                        unsafe {
                            (*p).__slate_bits_0.__set_useSubsec((1 as i32) as u32);
                        }
                        rc = 0 as i32;
                    }
                } else {
                    if !((unsafe { (*p).validJD }) != (0 as i8))
                        && !((unsafe { (*p).validYMD }) != (0 as i8))
                        && !((unsafe { (*p).validHMS }) != (0 as i8))
                    {
                    } else {
                        let __v799: *const i8 = z;
                        let __v800: *const i8 = unsafe { __v799.offset((9 as i32) as isize) };
                        z = __v800;
                        computeYMD(p);
                        unsafe {
                            (*p).validHMS = (1 as i32) as i8;
                        }
                        unsafe {
                            (*p).m = 0 as i32;
                        }
                        unsafe {
                            (*p).h = 0 as i32;
                        }
                        unsafe {
                            (*p).s = 0.0f64;
                        }
                        unsafe {
                            (*p).__slate_bits_0.__set_rawS((0 as i32) as u32);
                        }
                        unsafe {
                            (*p).tz = 0 as i32;
                        }
                        unsafe {
                            (*p).validJD = (0 as i32) as i8;
                        }
                        if (unsafe {
                            sqlite3_stricmp(z, (b"month\0".as_ptr() as *mut i8) as *const i8)
                        }) == (0 as i32)
                        {
                            unsafe {
                                (*p).D = 1 as i32;
                            }
                            rc = 0 as i32;
                        } else {
                            if (unsafe {
                                sqlite3_stricmp(z, (b"year\0".as_ptr() as *mut i8) as *const i8)
                            }) == (0 as i32)
                            {
                                unsafe {
                                    (*p).M = 1 as i32;
                                }
                                unsafe {
                                    (*p).D = 1 as i32;
                                }
                                rc = 0 as i32;
                            } else {
                                if (unsafe {
                                    sqlite3_stricmp(z, (b"day\0".as_ptr() as *mut i8) as *const i8)
                                }) == (0 as i32)
                                {
                                    rc = 0 as i32;
                                }
                            }
                        }
                    }
                }
            }
            43 | 45 | 48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 | 56 | 57 => {
                let mut rRounder: f64 = 0 as f64;
                let mut i: i32 = 0 as i32;
                let mut rx: i32 = 0 as i32;
                let mut Y: i32 = 0 as i32;
                let mut M: i32 = 0 as i32;
                let mut D: i32 = 0 as i32;
                let mut h: i32 = 0 as i32;
                let mut m: i32 = 0 as i32;
                let mut x: i32 = 0 as i32;
                let mut z2: *const i8 = z;
                let mut zCopy: *mut i8 = unsafe { std::mem::zeroed() };
                let mut db: *mut sqlite3 = unsafe { sqlite3_context_db_handle(pCtx) };
                let mut z0: i8 = unsafe { *unsafe { z.offset((0 as i32) as isize) } };
                n = 1 as i32;
                '__slate_break_657: loop {
                    if !((unsafe { *unsafe { z.offset(n as isize) } }) != (0 as i8)) {
                        break;
                    }
                    if ((unsafe { *unsafe { z.offset(n as isize) } }) as i32) == (58 as i32) {
                        break '__slate_break_657;
                    }
                    if (((unsafe {
                        *unsafe {
                            unsafe { std::ptr::addr_of!(sqlite3CtypeMap) as *const u8 }.offset(
                                ((((unsafe { *unsafe { z.offset(n as isize) } }) as u8) as u32)
                                    as i32) as isize,
                            )
                        }
                    }) as u32) as i32)
                        & (1 as i32)
                        != (0 as i32)
                    {
                        break '__slate_break_657;
                    }
                    if ((unsafe { *unsafe { z.offset(n as isize) } }) as i32) == (45 as i32) {
                        let __v803: bool;
                        if n == (5 as i32) {
                            __v803 = (unsafe {
                                getDigits(
                                    unsafe { z.offset((1 as i32) as isize) },
                                    (b"40f\0".as_ptr() as *mut i8) as *const i8,
                                    std::ptr::addr_of_mut!(Y),
                                )
                            }) == (1 as i32);
                        } else {
                            __v803 = false as bool;
                        }
                        if __v803 {
                            break '__slate_break_657;
                        }
                        let __v804: bool;
                        if n == (6 as i32) {
                            __v804 = (unsafe {
                                getDigits(
                                    unsafe { z.offset((1 as i32) as isize) },
                                    (b"50f\0".as_ptr() as *mut i8) as *const i8,
                                    std::ptr::addr_of_mut!(Y),
                                )
                            }) == (1 as i32);
                        } else {
                            __v804 = false as bool;
                        }
                        if __v804 {
                            break '__slate_break_657;
                        }
                    }
                    let __v801: i32 = n;
                    let __v802: i32 = __v801 + (1 as i32);
                    n = __v802;
                }
                zCopy = unsafe { sqlite3DbStrNDup(db, z, (n as i64) as u64) };
                if zCopy == std::ptr::null_mut::<i8>() {
                } else {
                    rx = ((unsafe { sqlite3AtoF(zCopy as *const i8, std::ptr::addr_of_mut!(r)) })
                        <= (0 as i32)) as i32;
                    unsafe { sqlite3DbFree(db, zCopy as *mut ()) };
                    if rx != (0 as i32) {
                        0 as i32;
                    } else {
                        if ((unsafe { *unsafe { z.offset(n as isize) } }) as i32) == (45 as i32) {
                            // /* A modifier of the form (+|-)YYYY-MM-DD adds or subtracts the
                            //         ** specified number of years, months, and days.  MM is limited to
                            //         ** the range 0-11 and DD is limited to 0-30.
                            //         */
                            // /* Must start with +/- */
                            if (z0 as i32) != (43 as i32) && (z0 as i32) != (45 as i32) {
                                break '__slate_break_637;
                            }
                            if n == (5 as i32) {
                                if (unsafe {
                                    getDigits(
                                        unsafe { z.offset((1 as i32) as isize) },
                                        (b"40f-20a-20d\0".as_ptr() as *mut i8) as *const i8,
                                        std::ptr::addr_of_mut!(Y),
                                        std::ptr::addr_of_mut!(M),
                                        std::ptr::addr_of_mut!(D),
                                    )
                                }) != (3 as i32)
                                {
                                    break '__slate_break_637;
                                }
                            } else {
                                0 as i32;
                                if (unsafe {
                                    getDigits(
                                        unsafe { z.offset((1 as i32) as isize) },
                                        (b"50f-20a-20d\0".as_ptr() as *mut i8) as *const i8,
                                        std::ptr::addr_of_mut!(Y),
                                        std::ptr::addr_of_mut!(M),
                                        std::ptr::addr_of_mut!(D),
                                    )
                                }) != (3 as i32)
                                {
                                    break '__slate_break_637;
                                }
                                let __v805: *const i8 = z;
                                let __v806: *const i8 =
                                    unsafe { __v805.offset((1 as i32) as isize) };
                                z = __v806;
                            }
                            // /* M range 0..11 */
                            if M >= (12 as i32) {
                                break '__slate_break_637;
                            }
                            // /* D range 0..30 */
                            if D >= (31 as i32) {
                                break '__slate_break_637;
                            }
                            computeYMD_HMS(p);
                            unsafe {
                                (*p).validJD = (0 as i32) as i8;
                            }
                            if (z0 as i32) == (45 as i32) {
                                let __v807: *mut DateTime = p;
                                let __v808: i32 = unsafe { (*__v807).Y };
                                let __v809: i32 = __v808 - Y;
                                unsafe {
                                    (*__v807).Y = __v809;
                                }
                                let __v810: *mut DateTime = p;
                                let __v811: i32 = unsafe { (*__v810).M };
                                let __v812: i32 = __v811 - M;
                                unsafe {
                                    (*__v810).M = __v812;
                                }
                                D = -D;
                            } else {
                                let __v813: *mut DateTime = p;
                                let __v814: i32 = unsafe { (*__v813).Y };
                                let __v815: i32 = __v814 + Y;
                                unsafe {
                                    (*__v813).Y = __v815;
                                }
                                let __v816: *mut DateTime = p;
                                let __v817: i32 = unsafe { (*__v816).M };
                                let __v818: i32 = __v817 + M;
                                unsafe {
                                    (*__v816).M = __v818;
                                }
                            }
                            x = if (unsafe { (*p).M }) > (0 as i32) {
                                ((unsafe { (*p).M }) - (1 as i32)) / (12 as i32)
                            } else {
                                ((unsafe { (*p).M }) - (12 as i32)) / (12 as i32)
                            };
                            let __v819: *mut DateTime = p;
                            let __v820: i32 = unsafe { (*__v819).Y };
                            let __v821: i32 = __v820 + x;
                            unsafe {
                                (*__v819).Y = __v821;
                            }
                            let __v822: *mut DateTime = p;
                            let __v823: i32 = unsafe { (*__v822).M };
                            let __v824: i32 = __v823 - x * (12 as i32);
                            unsafe {
                                (*__v822).M = __v824;
                            }
                            computeFloor(p);
                            computeJD(p);
                            unsafe {
                                (*p).validHMS = (0 as i32) as i8;
                            }
                            unsafe {
                                (*p).validYMD = (0 as i32) as i8;
                            }
                            let __v825: *mut DateTime = p;
                            let __v826: i64 = unsafe { (*__v825).iJD };
                            let __v827: i64 = __v826 + (D as i64) * ((86400000 as i32) as i64);
                            unsafe {
                                (*__v825).iJD = __v827;
                            }
                            if ((unsafe { *unsafe { z.offset((11 as i32) as isize) } }) as i32)
                                == (0 as i32)
                            {
                                rc = 0 as i32;
                                break '__slate_break_637;
                            }
                            let __v828: bool;
                            if (((unsafe {
                                *unsafe {
                                    unsafe { std::ptr::addr_of!(sqlite3CtypeMap) as *const u8 }
                                        .offset(
                                            ((((unsafe {
                                                *unsafe { z.offset((11 as i32) as isize) }
                                            }) as u8)
                                                as u32)
                                                as i32)
                                                as isize,
                                        )
                                }
                            }) as u32) as i32)
                                & (1 as i32)
                                != (0 as i32)
                            {
                                __v828 = (unsafe {
                                    getDigits(
                                        unsafe { z.offset((12 as i32) as isize) },
                                        (b"20c:20e\0".as_ptr() as *mut i8) as *const i8,
                                        std::ptr::addr_of_mut!(h),
                                        std::ptr::addr_of_mut!(m),
                                    )
                                }) == (2 as i32);
                            } else {
                                __v828 = false as bool;
                            }
                            if __v828 {
                                z2 = unsafe { z.offset((12 as i32) as isize) };
                                n = 2 as i32;
                            } else {
                                break '__slate_break_637;
                            }
                        }
                        if ((unsafe { *unsafe { z2.offset(n as isize) } }) as i32) == (58 as i32) {
                            // /* A modifier of the form (+|-)HH:MM:SS.FFF adds (or subtracts) the
                            //         ** specified number of hours, minutes, seconds, and fractional seconds
                            //         ** to the time.  The ".FFF" may be omitted.  The ":SS.FFF" may be
                            //         ** omitted.
                            //         */
                            let mut tx: DateTime = unsafe { std::mem::zeroed() };
                            let mut day: i64 = 0 as i64;
                            if !((((unsafe {
                                *unsafe {
                                    unsafe { std::ptr::addr_of!(sqlite3CtypeMap) as *const u8 }
                                        .offset(((((unsafe { *z2 }) as u8) as u32) as i32) as isize)
                                }
                            }) as u32) as i32)
                                & (4 as i32)
                                != (0 as i32))
                            {
                                let __v829: *const i8 = z2;
                                let __v830: *const i8 =
                                    unsafe { __v829.offset((1 as i32) as isize) };
                                z2 = __v830;
                            }
                            unsafe {
                                memset(std::ptr::addr_of_mut!(tx) as *mut (), 0 as i32, 48 as u64)
                            };
                            if parseHhMmSs(z2, std::ptr::addr_of_mut!(tx)) != (0 as i32) {
                            } else {
                                computeJD(std::ptr::addr_of_mut!(tx));
                                let __v831: i64 = tx.iJD;
                                let __v832: i64 = __v831 - ((43200000 as i32) as i64);
                                tx.iJD = __v832;
                                day = tx.iJD / ((86400000 as i32) as i64);
                                let __v833: i64 = tx.iJD;
                                let __v834: i64 = __v833 - day * ((86400000 as i32) as i64);
                                tx.iJD = __v834;
                                if (z0 as i32) == (45 as i32) {
                                    tx.iJD = -tx.iJD;
                                }
                                computeJD(p);
                                clearYMD_HMS_TZ(p);
                                let __v835: *mut DateTime = p;
                                let __v836: i64 = unsafe { (*__v835).iJD };
                                let __v837: i64 = __v836 + tx.iJD;
                                unsafe {
                                    (*__v835).iJD = __v837;
                                }
                                rc = 0 as i32;
                            }
                        } else {
                            // /* If control reaches this point, it means the transformation is
                            //       ** one of the forms like "+NNN days".  */
                            let __v838: *const i8 = z;
                            let __v839: *const i8 = unsafe { __v838.offset(n as isize) };
                            z = __v839;
                            '__slate_break_663: while (((unsafe {
                                *unsafe {
                                    unsafe { std::ptr::addr_of!(sqlite3CtypeMap) as *const u8 }
                                        .offset(((((unsafe { *z }) as u8) as u32) as i32) as isize)
                                }
                            }) as u32)
                                as i32)
                                & (1 as i32)
                                != (0 as i32)
                            {
                                let __v840: *const i8 = z;
                                let __v841: *const i8 =
                                    unsafe { __v840.offset((1 as i32) as isize) };
                                z = __v841;
                            }
                            n = unsafe { sqlite3Strlen30(z) };
                            if n < (3 as i32) || n > (10 as i32) {
                            } else {
                                if (((unsafe {
                                    *unsafe {
                                        unsafe {
                                            std::ptr::addr_of!(sqlite3UpperToLower) as *const u8
                                        }
                                        .offset(
                                            ((((unsafe {
                                                *unsafe { z.offset((n - (1 as i32)) as isize) }
                                            }) as u8)
                                                as u32)
                                                as i32)
                                                as isize,
                                        )
                                    }
                                }) as u32) as i32)
                                    == (115 as i32)
                                {
                                    let __v842: i32 = n;
                                    let __v843: i32 = __v842 - (1 as i32);
                                    n = __v843;
                                }
                                computeJD(p);
                                0 as i32;
                                rRounder = if r < ((0 as i32) as f64) {
                                    -0.5f64
                                } else {
                                    0.5f64
                                };
                                unsafe {
                                    (*p).nFloor = (0 as i32) as i8;
                                }
                                i = 0 as i32;
                                '__slate_break_664: loop {
                                    if !(i < ((((96 as u64) / (16 as u64)) as u32) as i32)) {
                                        break;
                                    }
                                    let __v846: bool;
                                    if (((unsafe {
                                        (*unsafe {
                                            unsafe {
                                                std::ptr::addr_of!(aXformType.0)
                                                    as *const __SlateRecord207
                                            }
                                            .offset(i as isize)
                                        })
                                        .nName
                                    }) as u32) as i32)
                                        == n
                                    {
                                        __v846 = (unsafe {
                                            sqlite3_strnicmp(
                                                unsafe {
                                                    (*unsafe {
                                                        unsafe {
                                                            std::ptr::addr_of!(aXformType.0)
                                                                as *const __SlateRecord207
                                                        }
                                                        .offset(i as isize)
                                                    })
                                                    .zName
                                                    .as_ptr()
                                                        as *const i8
                                                },
                                                z,
                                                n,
                                            )
                                        }) == (0 as i32);
                                    } else {
                                        __v846 = false as bool;
                                    }
                                    if __v846
                                        && r > (-unsafe {
                                            (*unsafe {
                                                unsafe {
                                                    std::ptr::addr_of!(aXformType.0)
                                                        as *const __SlateRecord207
                                                }
                                                .offset(i as isize)
                                            })
                                            .rLimit
                                        } as f64)
                                        && r < ((unsafe {
                                            (*unsafe {
                                                unsafe {
                                                    std::ptr::addr_of!(aXformType.0)
                                                        as *const __SlateRecord207
                                                }
                                                .offset(i as isize)
                                            })
                                            .rLimit
                                        }) as f64)
                                    {
                                        '__slate_break_665: {
                                            match i {
                                                4 => {
                                                    // /* Special processing to add months */
                                                    0 as i32;
                                                    computeYMD_HMS(p);
                                                    let __v847: *mut DateTime = p;
                                                    let __v848: i32 = unsafe { (*__v847).M };
                                                    let __v849: i32 = __v848 + (r as i32);
                                                    unsafe {
                                                        (*__v847).M = __v849;
                                                    }
                                                    x = if (unsafe { (*p).M }) > (0 as i32) {
                                                        ((unsafe { (*p).M }) - (1 as i32))
                                                            / (12 as i32)
                                                    } else {
                                                        ((unsafe { (*p).M }) - (12 as i32))
                                                            / (12 as i32)
                                                    };
                                                    let __v850: *mut DateTime = p;
                                                    let __v851: i32 = unsafe { (*__v850).Y };
                                                    let __v852: i32 = __v851 + x;
                                                    unsafe {
                                                        (*__v850).Y = __v852;
                                                    }
                                                    let __v853: *mut DateTime = p;
                                                    let __v854: i32 = unsafe { (*__v853).M };
                                                    let __v855: i32 = __v854 - x * (12 as i32);
                                                    unsafe {
                                                        (*__v853).M = __v855;
                                                    }
                                                    computeFloor(p);
                                                    unsafe {
                                                        (*p).validJD = (0 as i32) as i8;
                                                    }
                                                    let __v856: f64 = r;
                                                    let __v857: f64 = __v856 - ((r as i32) as f64);
                                                    r = __v857;
                                                    break '__slate_break_665;
                                                    // /* Special processing to add years */
                                                }
                                                5 => {
                                                    let mut y: i32 = r as i32;
                                                    0 as i32;
                                                    computeYMD_HMS(p);
                                                    0 as i32;
                                                    let __v858: *mut DateTime = p;
                                                    let __v859: i32 = unsafe { (*__v858).Y };
                                                    let __v860: i32 = __v859 + y;
                                                    unsafe {
                                                        (*__v858).Y = __v860;
                                                    }
                                                    computeFloor(p);
                                                    unsafe {
                                                        (*p).validJD = (0 as i32) as i8;
                                                    }
                                                    let __v861: f64 = r;
                                                    let __v862: f64 = __v861 - ((r as i32) as f64);
                                                    r = __v862;
                                                }
                                                _ => {}
                                            }
                                        }
                                        computeJD(p);
                                        let __v863: *mut DateTime = p;
                                        let __v864: i64 = unsafe { (*__v863).iJD };
                                        let __v865: i64 = __v864
                                            + ((r
                                                * 1000.0f64
                                                * ((unsafe {
                                                    (*unsafe {
                                                        unsafe {
                                                            std::ptr::addr_of!(aXformType.0)
                                                                as *const __SlateRecord207
                                                        }
                                                        .offset(i as isize)
                                                    })
                                                    .rXform
                                                })
                                                    as f64)
                                                + rRounder)
                                                as i64);
                                        unsafe {
                                            (*__v863).iJD = __v865;
                                        }
                                        rc = 0 as i32;
                                        break '__slate_break_664;
                                    }
                                    let __v844: i32 = i;
                                    let __v845: i32 = __v844 + (1 as i32);
                                    i = __v845;
                                }
                                clearYMD_HMS_TZ(p);
                            }
                        }
                    }
                }
            }
            _ => {}
        }
    }
    return rc;
}

// /* Function context */
// /* The text of the modifier */
// /* Length of zMod in bytes */
// /* The date/time value to be modified */
// /* Parameter index of the modifier */
// /*
// ** Process time function arguments.  argv[0] is a date-time stamp.
// ** argv[1] and following are modifiers.  Parse them all and write
// ** the resulting time into the DateTime structure p.  Return 0
// ** on success and 1 if there are any errors.
// **
// ** If there are zero parameters (if even argv[0] is undefined)
// ** then assume a default value of "now" for argv[0].
// */
fn isDate(
    mut context: *mut sqlite3_context,
    mut argc: i32,
    mut argv: *mut *mut sqlite3_value,
    mut p: *mut DateTime,
) -> i32 {
    let mut i: i32 = 0 as i32;
    let mut n: i32 = 0 as i32;
    let mut z: *const u8 = unsafe { std::mem::zeroed() };
    let mut eType: i32 = 0 as i32;
    unsafe { memset(p as *mut (), 0 as i32, 48 as u64) };
    if argc == (0 as i32) {
        if !((unsafe { sqlite3NotPureFunc(context) }) != (0 as i32)) {
            return 1 as i32;
        }
        return setDateTimeToCurrent(context, p);
    }
    let __v866: i32 =
        unsafe { sqlite3_value_type(unsafe { *unsafe { argv.offset((0 as i32) as isize) } }) };
    eType = __v866;
    if __v866 == (2 as i32) || eType == (1 as i32) {
        setRawDateNumber(p, unsafe {
            sqlite3_value_double(unsafe { *unsafe { argv.offset((0 as i32) as isize) } })
        });
    } else {
        z = unsafe { sqlite3_value_text(unsafe { *unsafe { argv.offset((0 as i32) as isize) } }) };
        let __v867: bool;
        if !(z != std::ptr::null::<u8>()) {
            __v867 = true as bool;
        } else {
            __v867 = parseDateOrTime(context, (z as *mut i8) as *const i8, p) != (0 as i32);
        }
        if __v867 {
            return 1 as i32;
        }
    }
    i = 1 as i32;
    '__slate_break_666: loop {
        if !(i < argc) {
            break;
        }
        z = unsafe { sqlite3_value_text(unsafe { *unsafe { argv.offset(i as isize) } }) };
        n = unsafe { sqlite3_value_bytes(unsafe { *unsafe { argv.offset(i as isize) } }) };
        let __v870: bool;
        if z == std::ptr::null::<u8>() {
            __v870 = true as bool;
        } else {
            __v870 = parseModifier(context, (z as *mut i8) as *const i8, n, p, i) != (0 as i32);
        }
        if __v870 {
            return 1 as i32;
        }
        let __v868: i32 = i;
        let __v869: i32 = __v868 + (1 as i32);
        i = __v869;
    }
    computeJD(p);
    let __v871: bool;
    if ((unsafe { (*p).__slate_bits_0.__get_isError() }) as i32) != (0 as i32) {
        __v871 = true as bool;
    } else {
        __v871 = !(validJulianDay(unsafe { (*p).iJD }) != (0 as i32));
    }
    if __v871 {
        return 1 as i32;
    }
    if argc == (1 as i32)
        && (unsafe { (*p).validYMD }) != (0 as i8)
        && (unsafe { (*p).D }) > (28 as i32)
    {
        // /* Make sure a YYYY-MM-DD is normalized.
        //     ** Example: 2023-02-31 -> 2023-03-03 */
        0 as i32;
        unsafe {
            (*p).validYMD = (0 as i32) as i8;
        }
    }
    return 0 as i32;
}

// /*
// ** The following routines implement the various date and time functions
// ** of SQLite.
// */
// /*
// **    julianday( TIMESTRING, MOD, MOD, ...)
// **
// ** Return the julian day number of the date specified in the arguments
// */
#[unsafe(link_section = ".text.slate_distinct.date.juliandayFunc")]
extern "C-unwind" fn juliandayFunc(
    mut context: *mut sqlite3_context,
    mut argc: i32,
    mut argv: *mut *mut sqlite3_value,
) {
    let mut x: DateTime = unsafe { std::mem::zeroed() };
    if isDate(context, argc, argv, std::ptr::addr_of_mut!(x)) == (0 as i32) {
        computeJD(std::ptr::addr_of_mut!(x));
        unsafe { sqlite3_result_double(context, (x.iJD as f64) / 86400000.0f64) };
    }
}

// /*
// **    unixepoch( TIMESTRING, MOD, MOD, ...)
// **
// ** Return the number of seconds (including fractional seconds) since
// ** the unix epoch of 1970-01-01 00:00:00 GMT.
// */
#[unsafe(link_section = ".text.slate_distinct.date.unixepochFunc")]
extern "C-unwind" fn unixepochFunc(
    mut context: *mut sqlite3_context,
    mut argc: i32,
    mut argv: *mut *mut sqlite3_value,
) {
    let mut x: DateTime = unsafe { std::mem::zeroed() };
    if isDate(context, argc, argv, std::ptr::addr_of_mut!(x)) == (0 as i32) {
        computeJD(std::ptr::addr_of_mut!(x));
        if (x.__slate_bits_0.__get_useSubsec() as i32) != (0 as i32) {
            unsafe {
                sqlite3_result_double(
                    context,
                    ((x.iJD - ((21086676 as i32) as i64) * ((10000000 as i32) as i64)) as f64)
                        / 1000.0f64,
                )
            };
        } else {
            unsafe {
                sqlite3_result_int64(
                    context,
                    x.iJD / ((1000 as i32) as i64)
                        - ((21086676 as i32) as i64) * ((10000 as i32) as i64),
                )
            };
        }
    }
}

// /*
// **    datetime( TIMESTRING, MOD, MOD, ...)
// **
// ** Return YYYY-MM-DD HH:MM:SS
// */
#[unsafe(link_section = ".text.slate_distinct.date.datetimeFunc")]
extern "C-unwind" fn datetimeFunc(
    mut context: *mut sqlite3_context,
    mut argc: i32,
    mut argv: *mut *mut sqlite3_value,
) {
    let mut x: DateTime = unsafe { std::mem::zeroed() };
    if isDate(context, argc, argv, std::ptr::addr_of_mut!(x)) == (0 as i32) {
        let mut Y: i32 = 0 as i32;
        let mut s: i32 = 0 as i32;
        let mut n: i32 = 0 as i32;
        let mut zBuf: __SlateAlign16<[i8; 32]> = __SlateAlign16([0 as i8; 32]);
        computeYMD_HMS(std::ptr::addr_of_mut!(x));
        Y = x.Y;
        if Y < (0 as i32) {
            Y = -Y;
        }
        unsafe {
            *unsafe { (zBuf.0.as_mut_ptr() as *mut i8).offset((1 as i32) as isize) } =
                ((48 as i32) + Y / (1000 as i32) % (10 as i32)) as i8;
        }
        unsafe {
            *unsafe { (zBuf.0.as_mut_ptr() as *mut i8).offset((2 as i32) as isize) } =
                ((48 as i32) + Y / (100 as i32) % (10 as i32)) as i8;
        }
        unsafe {
            *unsafe { (zBuf.0.as_mut_ptr() as *mut i8).offset((3 as i32) as isize) } =
                ((48 as i32) + Y / (10 as i32) % (10 as i32)) as i8;
        }
        unsafe {
            *unsafe { (zBuf.0.as_mut_ptr() as *mut i8).offset((4 as i32) as isize) } =
                ((48 as i32) + Y % (10 as i32)) as i8;
        }
        unsafe {
            *unsafe { (zBuf.0.as_mut_ptr() as *mut i8).offset((5 as i32) as isize) } =
                (45 as i32) as i8;
        }
        unsafe {
            *unsafe { (zBuf.0.as_mut_ptr() as *mut i8).offset((6 as i32) as isize) } =
                ((48 as i32) + x.M / (10 as i32) % (10 as i32)) as i8;
        }
        unsafe {
            *unsafe { (zBuf.0.as_mut_ptr() as *mut i8).offset((7 as i32) as isize) } =
                ((48 as i32) + x.M % (10 as i32)) as i8;
        }
        unsafe {
            *unsafe { (zBuf.0.as_mut_ptr() as *mut i8).offset((8 as i32) as isize) } =
                (45 as i32) as i8;
        }
        unsafe {
            *unsafe { (zBuf.0.as_mut_ptr() as *mut i8).offset((9 as i32) as isize) } =
                ((48 as i32) + x.D / (10 as i32) % (10 as i32)) as i8;
        }
        unsafe {
            *unsafe { (zBuf.0.as_mut_ptr() as *mut i8).offset((10 as i32) as isize) } =
                ((48 as i32) + x.D % (10 as i32)) as i8;
        }
        unsafe {
            *unsafe { (zBuf.0.as_mut_ptr() as *mut i8).offset((11 as i32) as isize) } =
                (32 as i32) as i8;
        }
        unsafe {
            *unsafe { (zBuf.0.as_mut_ptr() as *mut i8).offset((12 as i32) as isize) } =
                ((48 as i32) + x.h / (10 as i32) % (10 as i32)) as i8;
        }
        unsafe {
            *unsafe { (zBuf.0.as_mut_ptr() as *mut i8).offset((13 as i32) as isize) } =
                ((48 as i32) + x.h % (10 as i32)) as i8;
        }
        unsafe {
            *unsafe { (zBuf.0.as_mut_ptr() as *mut i8).offset((14 as i32) as isize) } =
                (58 as i32) as i8;
        }
        unsafe {
            *unsafe { (zBuf.0.as_mut_ptr() as *mut i8).offset((15 as i32) as isize) } =
                ((48 as i32) + x.m / (10 as i32) % (10 as i32)) as i8;
        }
        unsafe {
            *unsafe { (zBuf.0.as_mut_ptr() as *mut i8).offset((16 as i32) as isize) } =
                ((48 as i32) + x.m % (10 as i32)) as i8;
        }
        unsafe {
            *unsafe { (zBuf.0.as_mut_ptr() as *mut i8).offset((17 as i32) as isize) } =
                (58 as i32) as i8;
        }
        if (x.__slate_bits_0.__get_useSubsec() as i32) != (0 as i32) {
            s = (1000.0f64 * x.s + 0.5f64) as i32;
            unsafe {
                *unsafe { (zBuf.0.as_mut_ptr() as *mut i8).offset((18 as i32) as isize) } =
                    ((48 as i32) + s / (10000 as i32) % (10 as i32)) as i8;
            }
            unsafe {
                *unsafe { (zBuf.0.as_mut_ptr() as *mut i8).offset((19 as i32) as isize) } =
                    ((48 as i32) + s / (1000 as i32) % (10 as i32)) as i8;
            }
            unsafe {
                *unsafe { (zBuf.0.as_mut_ptr() as *mut i8).offset((20 as i32) as isize) } =
                    (46 as i32) as i8;
            }
            unsafe {
                *unsafe { (zBuf.0.as_mut_ptr() as *mut i8).offset((21 as i32) as isize) } =
                    ((48 as i32) + s / (100 as i32) % (10 as i32)) as i8;
            }
            unsafe {
                *unsafe { (zBuf.0.as_mut_ptr() as *mut i8).offset((22 as i32) as isize) } =
                    ((48 as i32) + s / (10 as i32) % (10 as i32)) as i8;
            }
            unsafe {
                *unsafe { (zBuf.0.as_mut_ptr() as *mut i8).offset((23 as i32) as isize) } =
                    ((48 as i32) + s % (10 as i32)) as i8;
            }
            unsafe {
                *unsafe { (zBuf.0.as_mut_ptr() as *mut i8).offset((24 as i32) as isize) } =
                    (0 as i32) as i8;
            }
            n = 24 as i32;
        } else {
            s = x.s as i32;
            unsafe {
                *unsafe { (zBuf.0.as_mut_ptr() as *mut i8).offset((18 as i32) as isize) } =
                    ((48 as i32) + s / (10 as i32) % (10 as i32)) as i8;
            }
            unsafe {
                *unsafe { (zBuf.0.as_mut_ptr() as *mut i8).offset((19 as i32) as isize) } =
                    ((48 as i32) + s % (10 as i32)) as i8;
            }
            unsafe {
                *unsafe { (zBuf.0.as_mut_ptr() as *mut i8).offset((20 as i32) as isize) } =
                    (0 as i32) as i8;
            }
            n = 20 as i32;
        }
        if x.Y < (0 as i32) {
            unsafe {
                *unsafe { (zBuf.0.as_mut_ptr() as *mut i8).offset((0 as i32) as isize) } =
                    (45 as i32) as i8;
            }
            unsafe {
                sqlite3_result_text(
                    context,
                    (zBuf.0.as_mut_ptr() as *mut i8) as *const i8,
                    n,
                    unsafe {
                        std::mem::transmute::<usize, Option<unsafe extern "C-unwind" fn(*mut ())>>(
                            -(1 as i32) as usize,
                        )
                    },
                )
            };
        } else {
            unsafe {
                sqlite3_result_text(
                    context,
                    (unsafe { (zBuf.0.as_mut_ptr() as *mut i8).offset((1 as i32) as isize) })
                        as *const i8,
                    n - (1 as i32),
                    unsafe {
                        std::mem::transmute::<usize, Option<unsafe extern "C-unwind" fn(*mut ())>>(
                            -(1 as i32) as usize,
                        )
                    },
                )
            };
        }
    }
}

// /*
// **    time( TIMESTRING, MOD, MOD, ...)
// **
// ** Return HH:MM:SS
// */
#[unsafe(link_section = ".text.slate_distinct.date.timeFunc")]
extern "C-unwind" fn timeFunc(
    mut context: *mut sqlite3_context,
    mut argc: i32,
    mut argv: *mut *mut sqlite3_value,
) {
    let mut x: DateTime = unsafe { std::mem::zeroed() };
    if isDate(context, argc, argv, std::ptr::addr_of_mut!(x)) == (0 as i32) {
        let mut s: i32 = 0 as i32;
        let mut n: i32 = 0 as i32;
        let mut zBuf: __SlateAlign16<[i8; 16]> = __SlateAlign16([0 as i8; 16]);
        computeHMS(std::ptr::addr_of_mut!(x));
        unsafe {
            *unsafe { (zBuf.0.as_mut_ptr() as *mut i8).offset((0 as i32) as isize) } =
                ((48 as i32) + x.h / (10 as i32) % (10 as i32)) as i8;
        }
        unsafe {
            *unsafe { (zBuf.0.as_mut_ptr() as *mut i8).offset((1 as i32) as isize) } =
                ((48 as i32) + x.h % (10 as i32)) as i8;
        }
        unsafe {
            *unsafe { (zBuf.0.as_mut_ptr() as *mut i8).offset((2 as i32) as isize) } =
                (58 as i32) as i8;
        }
        unsafe {
            *unsafe { (zBuf.0.as_mut_ptr() as *mut i8).offset((3 as i32) as isize) } =
                ((48 as i32) + x.m / (10 as i32) % (10 as i32)) as i8;
        }
        unsafe {
            *unsafe { (zBuf.0.as_mut_ptr() as *mut i8).offset((4 as i32) as isize) } =
                ((48 as i32) + x.m % (10 as i32)) as i8;
        }
        unsafe {
            *unsafe { (zBuf.0.as_mut_ptr() as *mut i8).offset((5 as i32) as isize) } =
                (58 as i32) as i8;
        }
        if (x.__slate_bits_0.__get_useSubsec() as i32) != (0 as i32) {
            s = (1000.0f64 * x.s + 0.5f64) as i32;
            unsafe {
                *unsafe { (zBuf.0.as_mut_ptr() as *mut i8).offset((6 as i32) as isize) } =
                    ((48 as i32) + s / (10000 as i32) % (10 as i32)) as i8;
            }
            unsafe {
                *unsafe { (zBuf.0.as_mut_ptr() as *mut i8).offset((7 as i32) as isize) } =
                    ((48 as i32) + s / (1000 as i32) % (10 as i32)) as i8;
            }
            unsafe {
                *unsafe { (zBuf.0.as_mut_ptr() as *mut i8).offset((8 as i32) as isize) } =
                    (46 as i32) as i8;
            }
            unsafe {
                *unsafe { (zBuf.0.as_mut_ptr() as *mut i8).offset((9 as i32) as isize) } =
                    ((48 as i32) + s / (100 as i32) % (10 as i32)) as i8;
            }
            unsafe {
                *unsafe { (zBuf.0.as_mut_ptr() as *mut i8).offset((10 as i32) as isize) } =
                    ((48 as i32) + s / (10 as i32) % (10 as i32)) as i8;
            }
            unsafe {
                *unsafe { (zBuf.0.as_mut_ptr() as *mut i8).offset((11 as i32) as isize) } =
                    ((48 as i32) + s % (10 as i32)) as i8;
            }
            unsafe {
                *unsafe { (zBuf.0.as_mut_ptr() as *mut i8).offset((12 as i32) as isize) } =
                    (0 as i32) as i8;
            }
            n = 12 as i32;
        } else {
            s = x.s as i32;
            unsafe {
                *unsafe { (zBuf.0.as_mut_ptr() as *mut i8).offset((6 as i32) as isize) } =
                    ((48 as i32) + s / (10 as i32) % (10 as i32)) as i8;
            }
            unsafe {
                *unsafe { (zBuf.0.as_mut_ptr() as *mut i8).offset((7 as i32) as isize) } =
                    ((48 as i32) + s % (10 as i32)) as i8;
            }
            unsafe {
                *unsafe { (zBuf.0.as_mut_ptr() as *mut i8).offset((8 as i32) as isize) } =
                    (0 as i32) as i8;
            }
            n = 8 as i32;
        }
        unsafe {
            sqlite3_result_text(
                context,
                (zBuf.0.as_mut_ptr() as *mut i8) as *const i8,
                n,
                unsafe {
                    std::mem::transmute::<usize, Option<unsafe extern "C-unwind" fn(*mut ())>>(
                        -(1 as i32) as usize,
                    )
                },
            )
        };
    }
}

// /*
// **    date( TIMESTRING, MOD, MOD, ...)
// **
// ** Return YYYY-MM-DD
// */
#[unsafe(link_section = ".text.slate_distinct.date.dateFunc")]
extern "C-unwind" fn dateFunc(
    mut context: *mut sqlite3_context,
    mut argc: i32,
    mut argv: *mut *mut sqlite3_value,
) {
    let mut x: DateTime = unsafe { std::mem::zeroed() };
    if isDate(context, argc, argv, std::ptr::addr_of_mut!(x)) == (0 as i32) {
        let mut Y: i32 = 0 as i32;
        let mut zBuf: __SlateAlign16<[i8; 16]> = __SlateAlign16([0 as i8; 16]);
        computeYMD(std::ptr::addr_of_mut!(x));
        Y = x.Y;
        if Y < (0 as i32) {
            Y = -Y;
        }
        unsafe {
            *unsafe { (zBuf.0.as_mut_ptr() as *mut i8).offset((1 as i32) as isize) } =
                ((48 as i32) + Y / (1000 as i32) % (10 as i32)) as i8;
        }
        unsafe {
            *unsafe { (zBuf.0.as_mut_ptr() as *mut i8).offset((2 as i32) as isize) } =
                ((48 as i32) + Y / (100 as i32) % (10 as i32)) as i8;
        }
        unsafe {
            *unsafe { (zBuf.0.as_mut_ptr() as *mut i8).offset((3 as i32) as isize) } =
                ((48 as i32) + Y / (10 as i32) % (10 as i32)) as i8;
        }
        unsafe {
            *unsafe { (zBuf.0.as_mut_ptr() as *mut i8).offset((4 as i32) as isize) } =
                ((48 as i32) + Y % (10 as i32)) as i8;
        }
        unsafe {
            *unsafe { (zBuf.0.as_mut_ptr() as *mut i8).offset((5 as i32) as isize) } =
                (45 as i32) as i8;
        }
        unsafe {
            *unsafe { (zBuf.0.as_mut_ptr() as *mut i8).offset((6 as i32) as isize) } =
                ((48 as i32) + x.M / (10 as i32) % (10 as i32)) as i8;
        }
        unsafe {
            *unsafe { (zBuf.0.as_mut_ptr() as *mut i8).offset((7 as i32) as isize) } =
                ((48 as i32) + x.M % (10 as i32)) as i8;
        }
        unsafe {
            *unsafe { (zBuf.0.as_mut_ptr() as *mut i8).offset((8 as i32) as isize) } =
                (45 as i32) as i8;
        }
        unsafe {
            *unsafe { (zBuf.0.as_mut_ptr() as *mut i8).offset((9 as i32) as isize) } =
                ((48 as i32) + x.D / (10 as i32) % (10 as i32)) as i8;
        }
        unsafe {
            *unsafe { (zBuf.0.as_mut_ptr() as *mut i8).offset((10 as i32) as isize) } =
                ((48 as i32) + x.D % (10 as i32)) as i8;
        }
        unsafe {
            *unsafe { (zBuf.0.as_mut_ptr() as *mut i8).offset((11 as i32) as isize) } =
                (0 as i32) as i8;
        }
        if x.Y < (0 as i32) {
            unsafe {
                *unsafe { (zBuf.0.as_mut_ptr() as *mut i8).offset((0 as i32) as isize) } =
                    (45 as i32) as i8;
            }
            unsafe {
                sqlite3_result_text(
                    context,
                    (zBuf.0.as_mut_ptr() as *mut i8) as *const i8,
                    11 as i32,
                    unsafe {
                        std::mem::transmute::<usize, Option<unsafe extern "C-unwind" fn(*mut ())>>(
                            -(1 as i32) as usize,
                        )
                    },
                )
            };
        } else {
            unsafe {
                sqlite3_result_text(
                    context,
                    (unsafe { (zBuf.0.as_mut_ptr() as *mut i8).offset((1 as i32) as isize) })
                        as *const i8,
                    10 as i32,
                    unsafe {
                        std::mem::transmute::<usize, Option<unsafe extern "C-unwind" fn(*mut ())>>(
                            -(1 as i32) as usize,
                        )
                    },
                )
            };
        }
    }
}

// /*
// ** Compute the number of days after the most recent January 1.
// **
// ** In other words, compute the zero-based day number for the
// ** current year:
// **
// **   Jan01 = 0,  Jan02 = 1, ..., Jan31 = 30, Feb01 = 31, ...
// **   Dec31 = 364 or 365.
// */
fn daysAfterJan01(mut pDate: *mut DateTime) -> i32 {
    let mut jan01: DateTime = unsafe { *pDate };
    0 as i32;
    0 as i32;
    0 as i32;
    jan01.validJD = (0 as i32) as i8;
    jan01.M = 1 as i32;
    jan01.D = 1 as i32;
    computeJD(std::ptr::addr_of_mut!(jan01));
    return (((unsafe { (*pDate).iJD }) - jan01.iJD + ((43200000 as i32) as i64))
        / ((86400000 as i32) as i64)) as i32;
}

// /*
// ** Return the number of days after the most recent Monday.
// **
// ** In other words, return the day of the week according
// ** to this code:
// **
// **   0=Monday, 1=Tuesday, 2=Wednesday, ..., 6=Sunday.
// */
fn daysAfterMonday(mut pDate: *mut DateTime) -> i32 {
    0 as i32;
    return ((((unsafe { (*pDate).iJD }) + ((43200000 as i32) as i64)) / ((86400000 as i32) as i64))
        as i32)
        % (7 as i32);
}

// /*
// ** Return the number of days after the most recent Sunday.
// **
// ** In other words, return the day of the week according
// ** to this code:
// **
// **   0=Sunday, 1=Monday, 2=Tuesday, ..., 6=Saturday
// */
fn daysAfterSunday(mut pDate: *mut DateTime) -> i32 {
    0 as i32;
    return ((((unsafe { (*pDate).iJD }) + ((129600000 as i32) as i64)) / ((86400000 as i32) as i64))
        as i32)
        % (7 as i32);
}

// /*
// **    strftime( FORMAT, TIMESTRING, MOD, MOD, ...)
// **
// ** Return a string described by FORMAT.  Conversions as follows:
// **
// **   %d  day of month  01-31
// **   %e  day of month  1-31
// **   %f  ** fractional seconds  SS.SSS
// **   %F  ISO date.  YYYY-MM-DD
// **   %G  ISO year corresponding to %V 0000-9999.
// **   %g  2-digit ISO year corresponding to %V 00-99
// **   %H  hour 00-24
// **   %k  hour  0-24  (leading zero converted to space)
// **   %I  hour 01-12
// **   %j  day of year 001-366
// **   %J  ** julian day number
// **   %l  hour  1-12  (leading zero converted to space)
// **   %m  month 01-12
// **   %M  minute 00-59
// **   %p  "AM" or "PM"
// **   %P  "am" or "pm"
// **   %R  time as HH:MM
// **   %s  seconds since 1970-01-01
// **   %S  seconds 00-59
// **   %T  time as HH:MM:SS
// **   %u  day of week 1-7  Monday==1, Sunday==7
// **   %w  day of week 0-6  Sunday==0, Monday==1
// **   %U  week of year 00-53  (First Sunday is start of week 01)
// **   %V  week of year 01-53  (First week containing Thursday is week 01)
// **   %W  week of year 00-53  (First Monday is start of week 01)
// **   %Y  year 0000-9999
// **   %%  %
// */
#[unsafe(link_section = ".text.slate_distinct.date.strftimeFunc")]
extern "C-unwind" fn strftimeFunc(
    mut context: *mut sqlite3_context,
    mut argc: i32,
    mut argv: *mut *mut sqlite3_value,
) {
    let mut x: DateTime = unsafe { std::mem::zeroed() };
    let mut i: u64 = 0 as u64;
    let mut j: u64 = 0 as u64;
    let mut db: *mut sqlite3 = unsafe { std::mem::zeroed() };
    let mut zFmt: *const i8 = unsafe { std::mem::zeroed() };
    let mut pRes: *mut sqlite3_str = unsafe { std::mem::zeroed() };
    if argc == (0 as i32) {
        return;
    }
    zFmt = (unsafe { sqlite3_value_text(unsafe { *unsafe { argv.offset((0 as i32) as isize) } }) })
        as *const i8;
    let __v872: bool;
    if zFmt == std::ptr::null::<i8>() {
        __v872 = true as bool;
    } else {
        __v872 = isDate(
            context,
            argc - (1 as i32),
            unsafe { argv.offset((1 as i32) as isize) },
            std::ptr::addr_of_mut!(x),
        ) != (0 as i32);
    }
    if __v872 {
        return;
    }
    db = unsafe { sqlite3_context_db_handle(context) };
    pRes = unsafe { sqlite3_str_new(db) };
    computeJD(std::ptr::addr_of_mut!(x));
    computeYMD_HMS(std::ptr::addr_of_mut!(x));
    j = ((0 as i32) as i64) as u64;
    i = ((0 as i32) as i64) as u64;
    '__slate_break_667: while (unsafe { *unsafe { zFmt.offset(i as isize) } }) != (0 as i8) {
        let mut cf: i8 = 0 as i8;
        if ((unsafe { *unsafe { zFmt.offset(i as isize) } }) as i32) != (37 as i32) {
        } else {
            if j < i {
                unsafe {
                    sqlite3_str_append(
                        pRes,
                        unsafe { zFmt.offset(j as isize) },
                        (i.wrapping_sub(j) as u32) as i32,
                    )
                };
            }
            let __v875: u64 = i;
            let __v876: u64 = __v875.wrapping_add(((1 as i32) as i64) as u64);
            i = __v876;
            j = i.wrapping_add(((1 as i32) as i64) as u64);
            cf = unsafe { *unsafe { zFmt.offset(i as isize) } };
            '__slate_break_668: {
                match cf as i32 {
                    100 | 101 => {
                        // /* Fall thru */
                        unsafe {
                            sqlite3_str_appendf(
                                pRes,
                                (if (cf as i32) == (100 as i32) {
                                    b"%02d\0".as_ptr() as *mut i8
                                } else {
                                    b"%2d\0".as_ptr() as *mut i8
                                }) as *const i8,
                                x.D,
                            )
                        };
                        break '__slate_break_668;
                        // /* Fractional seconds.  (Non-standard) */
                    }
                    102 => {
                        let mut s: f64 = x.s;
                        if s > 59.999f64 {
                            s = 59.999f64;
                        }
                        unsafe {
                            sqlite3_str_appendf(
                                pRes,
                                (b"%06.3f\0".as_ptr() as *mut i8) as *const i8,
                                s,
                            )
                        };
                    }
                    70 => {
                        unsafe {
                            sqlite3_str_appendf(
                                pRes,
                                (b"%04d-%02d-%02d\0".as_ptr() as *mut i8) as *const i8,
                                x.Y,
                                x.M,
                                x.D,
                            )
                        };
                        break '__slate_break_668;
                        // /* Fall thru */
                    }
                    71 | 103 => {
                        let mut y: DateTime = x;
                        0 as i32;
                        // /* Move y so that it is the Thursday in the same week as x */
                        let __v877: i64 = y.iJD;
                        let __v878: i64 = __v877
                            + ((((3 as i32) - daysAfterMonday(std::ptr::addr_of_mut!(x)))
                                * (86400000 as i32)) as i64);
                        y.iJD = __v878;
                        y.validYMD = (0 as i32) as i8;
                        computeYMD(std::ptr::addr_of_mut!(y));
                        if (cf as i32) == (103 as i32) {
                            unsafe {
                                sqlite3_str_appendf(
                                    pRes,
                                    (b"%02d\0".as_ptr() as *mut i8) as *const i8,
                                    y.Y % (100 as i32),
                                )
                            };
                        } else {
                            unsafe {
                                sqlite3_str_appendf(
                                    pRes,
                                    (b"%04d\0".as_ptr() as *mut i8) as *const i8,
                                    y.Y,
                                )
                            };
                        }
                    }
                    72 | 107 => {
                        unsafe {
                            sqlite3_str_appendf(
                                pRes,
                                (if (cf as i32) == (72 as i32) {
                                    b"%02d\0".as_ptr() as *mut i8
                                } else {
                                    b"%2d\0".as_ptr() as *mut i8
                                }) as *const i8,
                                x.h,
                            )
                        };
                        break '__slate_break_668;
                        // /* Fall thru */
                    }
                    73 | 108 => {
                        let mut h: i32 = x.h;
                        if h > (12 as i32) {
                            let __v879: i32 = h;
                            let __v880: i32 = __v879 - (12 as i32);
                            h = __v880;
                        }
                        if h == (0 as i32) {
                            h = 12 as i32;
                        }
                        unsafe {
                            sqlite3_str_appendf(
                                pRes,
                                (if (cf as i32) == (73 as i32) {
                                    b"%02d\0".as_ptr() as *mut i8
                                } else {
                                    b"%2d\0".as_ptr() as *mut i8
                                }) as *const i8,
                                h,
                            )
                        };
                        break '__slate_break_668;
                        // /* Day of year.  Jan01==1, Jan02==2, and so forth */
                    }
                    106 => {
                        unsafe {
                            sqlite3_str_appendf(
                                pRes,
                                (b"%03d\0".as_ptr() as *mut i8) as *const i8,
                                daysAfterJan01(std::ptr::addr_of_mut!(x)) + (1 as i32),
                            )
                        };
                        break '__slate_break_668;
                        // /* Julian day number.  (Non-standard) */
                    }
                    74 => {
                        unsafe {
                            sqlite3_str_appendf(
                                pRes,
                                (b"%.16g\0".as_ptr() as *mut i8) as *const i8,
                                (x.iJD as f64) / 86400000.0f64,
                            )
                        };
                    }
                    109 => {
                        unsafe {
                            sqlite3_str_appendf(
                                pRes,
                                (b"%02d\0".as_ptr() as *mut i8) as *const i8,
                                x.M,
                            )
                        };
                    }
                    77 => {
                        unsafe {
                            sqlite3_str_appendf(
                                pRes,
                                (b"%02d\0".as_ptr() as *mut i8) as *const i8,
                                x.m,
                            )
                        };
                        break '__slate_break_668;
                        // /* Fall thru */
                    }
                    112 | 80 => {
                        if x.h >= (12 as i32) {
                            unsafe {
                                sqlite3_str_append(
                                    pRes,
                                    (if (cf as i32) == (112 as i32) {
                                        b"PM\0".as_ptr() as *mut i8
                                    } else {
                                        b"pm\0".as_ptr() as *mut i8
                                    }) as *const i8,
                                    2 as i32,
                                )
                            };
                        } else {
                            unsafe {
                                sqlite3_str_append(
                                    pRes,
                                    (if (cf as i32) == (112 as i32) {
                                        b"AM\0".as_ptr() as *mut i8
                                    } else {
                                        b"am\0".as_ptr() as *mut i8
                                    }) as *const i8,
                                    2 as i32,
                                )
                            };
                        }
                    }
                    82 => {
                        unsafe {
                            sqlite3_str_appendf(
                                pRes,
                                (b"%02d:%02d\0".as_ptr() as *mut i8) as *const i8,
                                x.h,
                                x.m,
                            )
                        };
                    }
                    115 => {
                        if (x.__slate_bits_0.__get_useSubsec() as i32) != (0 as i32) {
                            unsafe {
                                sqlite3_str_appendf(
                                    pRes,
                                    (b"%.3f\0".as_ptr() as *mut i8) as *const i8,
                                    ((x.iJD
                                        - ((21086676 as i32) as i64) * ((10000000 as i32) as i64))
                                        as f64)
                                        / 1000.0f64,
                                )
                            };
                        } else {
                            let mut iS: i64 = x.iJD / ((1000 as i32) as i64)
                                - ((21086676 as i32) as i64) * ((10000 as i32) as i64);
                            unsafe {
                                sqlite3_str_appendf(
                                    pRes,
                                    (b"%lld\0".as_ptr() as *mut i8) as *const i8,
                                    iS,
                                )
                            };
                        }
                    }
                    83 => {
                        unsafe {
                            sqlite3_str_appendf(
                                pRes,
                                (b"%02d\0".as_ptr() as *mut i8) as *const i8,
                                x.s as i32,
                            )
                        };
                    }
                    84 => {
                        unsafe {
                            sqlite3_str_appendf(
                                pRes,
                                (b"%02d:%02d:%02d\0".as_ptr() as *mut i8) as *const i8,
                                x.h,
                                x.m,
                                x.s as i32,
                            )
                        };
                        break '__slate_break_668;
                        // /* Day of week.  1 to 7.  Monday==1, Sunday==7 */
                    }
                    117 | 119 => {
                        let mut c: i8 = (((daysAfterSunday(std::ptr::addr_of_mut!(x)) as i8)
                            as i32)
                            + (48 as i32)) as i8;
                        if (c as i32) == (48 as i32) && (cf as i32) == (117 as i32) {
                            c = (55 as i32) as i8;
                        }
                        unsafe { sqlite3_str_appendchar(pRes, 1 as i32, c) };
                        break '__slate_break_668;
                        // /* Day of week.  0 to 6.  Sunday==0, Monday==1 */
                        // /* Week num. 00-53. First Sun of the year is week 01 */
                    }
                    85 => {
                        unsafe {
                            sqlite3_str_appendf(
                                pRes,
                                (b"%02d\0".as_ptr() as *mut i8) as *const i8,
                                (daysAfterJan01(std::ptr::addr_of_mut!(x))
                                    - daysAfterSunday(std::ptr::addr_of_mut!(x))
                                    + (7 as i32))
                                    / (7 as i32),
                            )
                        };
                        break '__slate_break_668;
                        // /* Week num. 01-53. First week with a Thur is week 01 */
                    }
                    86 => {
                        let mut y: DateTime = x;
                        // /* Adjust y so that is the Thursday in the same week as x */
                        0 as i32;
                        let __v881: i64 = y.iJD;
                        let __v882: i64 = __v881
                            + ((((3 as i32) - daysAfterMonday(std::ptr::addr_of_mut!(x)))
                                * (86400000 as i32)) as i64);
                        y.iJD = __v882;
                        y.validYMD = (0 as i32) as i8;
                        computeYMD(std::ptr::addr_of_mut!(y));
                        unsafe {
                            sqlite3_str_appendf(
                                pRes,
                                (b"%02d\0".as_ptr() as *mut i8) as *const i8,
                                daysAfterJan01(std::ptr::addr_of_mut!(y)) / (7 as i32) + (1 as i32),
                            )
                        };
                        break '__slate_break_668;
                        // /* Week num. 00-53. First Mon of the year is week 01 */
                    }
                    87 => {
                        unsafe {
                            sqlite3_str_appendf(
                                pRes,
                                (b"%02d\0".as_ptr() as *mut i8) as *const i8,
                                (daysAfterJan01(std::ptr::addr_of_mut!(x))
                                    - daysAfterMonday(std::ptr::addr_of_mut!(x))
                                    + (7 as i32))
                                    / (7 as i32),
                            )
                        };
                    }
                    89 => {
                        unsafe {
                            sqlite3_str_appendf(
                                pRes,
                                (b"%04d\0".as_ptr() as *mut i8) as *const i8,
                                x.Y,
                            )
                        };
                    }
                    37 => {
                        unsafe { sqlite3_str_appendchar(pRes, 1 as i32, (37 as i32) as i8) };
                    }
                    _ => {
                        unsafe { sqlite3_str_free(pRes) };
                        return;
                    }
                }
            }
        }
        let __v873: u64 = i;
        let __v874: u64 = __v873.wrapping_add(((1 as i32) as i64) as u64);
        i = __v874;
    }
    if j < i {
        unsafe {
            sqlite3_str_append(
                pRes,
                unsafe { zFmt.offset(j as isize) },
                (i.wrapping_sub(j) as u32) as i32,
            )
        };
    }
    unsafe { sqlite3_result_str(context, pRes, 2 as i32) };
}

// /*
// ** current_time()
// **
// ** This function returns the same value as time('now').
// */
#[unsafe(link_section = ".text.slate_distinct.date.ctimeFunc")]
extern "C-unwind" fn ctimeFunc(
    mut context: *mut sqlite3_context,
    mut NotUsed: i32,
    mut NotUsed2: *mut *mut sqlite3_value,
) {
    NotUsed;
    NotUsed2;
    timeFunc(
        context,
        0 as i32,
        std::ptr::null_mut::<*mut sqlite3_value>(),
    );
}

// /*
// ** current_date()
// **
// ** This function returns the same value as date('now').
// */
#[unsafe(link_section = ".text.slate_distinct.date.cdateFunc")]
extern "C-unwind" fn cdateFunc(
    mut context: *mut sqlite3_context,
    mut NotUsed: i32,
    mut NotUsed2: *mut *mut sqlite3_value,
) {
    NotUsed;
    NotUsed2;
    dateFunc(
        context,
        0 as i32,
        std::ptr::null_mut::<*mut sqlite3_value>(),
    );
}

// /*
// ** timediff(DATE1, DATE2)
// **
// ** Return the amount of time that must be added to DATE2 in order to
// ** convert it into DATE2.  The time difference format is:
// **
// **     +YYYY-MM-DD HH:MM:SS.SSS
// **
// ** The initial "+" becomes "-" if DATE1 occurs before DATE2.  For
// ** date/time values A and B, the following invariant should hold:
// **
// **     datetime(A) == (datetime(B, timediff(A,B))
// **
// ** Both DATE arguments must be either a julian day number, or an
// ** ISO-8601 string.  The unix timestamps are not supported by this
// ** routine.
// */
#[unsafe(link_section = ".text.slate_distinct.date.timediffFunc")]
extern "C-unwind" fn timediffFunc(
    mut context: *mut sqlite3_context,
    mut NotUsed1: i32,
    mut argv: *mut *mut sqlite3_value,
) {
    let mut sign: i8 = 0 as i8;
    let mut Y: i32 = 0 as i32;
    let mut M: i32 = 0 as i32;
    let mut d1: DateTime = unsafe { std::mem::zeroed() };
    let mut d2: DateTime = unsafe { std::mem::zeroed() };
    let mut sRes: sqlite3_str = unsafe { std::mem::zeroed() };
    NotUsed1;
    if isDate(
        context,
        1 as i32,
        unsafe { argv.offset((0 as i32) as isize) },
        std::ptr::addr_of_mut!(d1),
    ) != (0 as i32)
    {
        return;
    }
    if isDate(
        context,
        1 as i32,
        unsafe { argv.offset((1 as i32) as isize) },
        std::ptr::addr_of_mut!(d2),
    ) != (0 as i32)
    {
        return;
    }
    computeYMD_HMS(std::ptr::addr_of_mut!(d1));
    computeYMD_HMS(std::ptr::addr_of_mut!(d2));
    if d1.iJD >= d2.iJD {
        sign = (43 as i32) as i8;
        Y = d1.Y - d2.Y;
        if Y != (0 as i32) {
            d2.Y = d1.Y;
            d2.validJD = (0 as i32) as i8;
            computeJD(std::ptr::addr_of_mut!(d2));
        }
        M = d1.M - d2.M;
        if M < (0 as i32) {
            let __v883: i32 = Y;
            let __v884: i32 = __v883 - (1 as i32);
            Y = __v884;
            let __v885: i32 = M;
            let __v886: i32 = __v885 + (12 as i32);
            M = __v886;
        }
        if M != (0 as i32) {
            d2.M = d1.M;
            d2.validJD = (0 as i32) as i8;
            computeJD(std::ptr::addr_of_mut!(d2));
        }
        '__slate_break_696: while d1.iJD < d2.iJD {
            let __v887: i32 = M;
            let __v888: i32 = __v887 - (1 as i32);
            M = __v888;
            if M < (0 as i32) {
                M = 11 as i32;
                let __v889: i32 = Y;
                let __v890: i32 = __v889 - (1 as i32);
                Y = __v890;
            }
            let __v891: i32 = d2.M;
            let __v892: i32 = __v891 - (1 as i32);
            d2.M = __v892;
            if d2.M < (1 as i32) {
                d2.M = 12 as i32;
                let __v893: i32 = d2.Y;
                let __v894: i32 = __v893 - (1 as i32);
                d2.Y = __v894;
            }
            d2.validJD = (0 as i32) as i8;
            computeJD(std::ptr::addr_of_mut!(d2));
        }
        let __v895: i64 = d1.iJD;
        let __v896: i64 = __v895 - d2.iJD;
        d1.iJD = __v896;
        let __v897: i64 = d1.iJD;
        let __v898: i64 = (__v897 as u64).wrapping_add(
            (((1486995408 as i32) as i64) as u64).wrapping_mul(((100000 as i32) as i64) as u64),
        ) as i64;
        d1.iJD = __v898;
    // /* d1<d2 */
    } else {
        sign = (45 as i32) as i8;
        Y = d2.Y - d1.Y;
        if Y != (0 as i32) {
            d2.Y = d1.Y;
            d2.validJD = (0 as i32) as i8;
            computeJD(std::ptr::addr_of_mut!(d2));
        }
        M = d2.M - d1.M;
        if M < (0 as i32) {
            let __v899: i32 = Y;
            let __v900: i32 = __v899 - (1 as i32);
            Y = __v900;
            let __v901: i32 = M;
            let __v902: i32 = __v901 + (12 as i32);
            M = __v902;
        }
        if M != (0 as i32) {
            d2.M = d1.M;
            d2.validJD = (0 as i32) as i8;
            computeJD(std::ptr::addr_of_mut!(d2));
        }
        '__slate_break_697: while d1.iJD > d2.iJD {
            let __v903: i32 = M;
            let __v904: i32 = __v903 - (1 as i32);
            M = __v904;
            if M < (0 as i32) {
                M = 11 as i32;
                let __v905: i32 = Y;
                let __v906: i32 = __v905 - (1 as i32);
                Y = __v906;
            }
            let __v907: i32 = d2.M;
            let __v908: i32 = __v907 + (1 as i32);
            d2.M = __v908;
            if d2.M > (12 as i32) {
                d2.M = 1 as i32;
                let __v909: i32 = d2.Y;
                let __v910: i32 = __v909 + (1 as i32);
                d2.Y = __v910;
            }
            d2.validJD = (0 as i32) as i8;
            computeJD(std::ptr::addr_of_mut!(d2));
        }
        d1.iJD = d2.iJD - d1.iJD;
        let __v911: i64 = d1.iJD;
        let __v912: i64 = (__v911 as u64).wrapping_add(
            (((1486995408 as i32) as i64) as u64).wrapping_mul(((100000 as i32) as i64) as u64),
        ) as i64;
        d1.iJD = __v912;
    }
    clearYMD_HMS_TZ(std::ptr::addr_of_mut!(d1));
    computeYMD_HMS(std::ptr::addr_of_mut!(d1));
    unsafe {
        sqlite3StrAccumInit(
            std::ptr::addr_of_mut!(sRes),
            std::ptr::null_mut::<sqlite3>(),
            std::ptr::null_mut::<i8>(),
            0 as i32,
            100 as i32,
        )
    };
    unsafe {
        sqlite3_str_appendf(
            std::ptr::addr_of_mut!(sRes),
            (b"%c%04d-%02d-%02d %02d:%02d:%06.3f\0".as_ptr() as *mut i8) as *const i8,
            sign as i32,
            Y,
            M,
            d1.D - (1 as i32),
            d1.h,
            d1.m,
            d1.s,
        )
    };
    unsafe { sqlite3_result_str(context, std::ptr::addr_of_mut!(sRes), 1 as i32) };
}

// /*
// ** current_timestamp()
// **
// ** This function returns the same value as datetime('now').
// */
#[unsafe(link_section = ".text.slate_distinct.date.ctimestampFunc")]
extern "C-unwind" fn ctimestampFunc(
    mut context: *mut sqlite3_context,
    mut NotUsed: i32,
    mut NotUsed2: *mut *mut sqlite3_value,
) {
    NotUsed;
    NotUsed2;
    datetimeFunc(
        context,
        0 as i32,
        std::ptr::null_mut::<*mut sqlite3_value>(),
    );
}
