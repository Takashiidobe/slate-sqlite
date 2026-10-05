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
    trace: __SlateRecord167,
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
    u1: __SlateRecord168,
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
    __slate_bits_0: __slate_bits::__SlateBits72U0,
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
    u: __SlateRecord178,
    pLeft: *mut Expr,
    pRight: *mut Expr,
    x: __SlateRecord179,
    nHeight: i32,
    iTable: i32,
    iColumn: i16,
    iAgg: i16,
    w: __SlateRecord180,
    pAggInfo: *mut AggInfo,
    y: __SlateRecord181,
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
    u: __SlateRecord169,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct FuncDefHash {
    a: [*mut FuncDef; 23],
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
    __slate_bits_0: __slate_bits::__SlateBits98U0,
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
    __slate_bits_0: __slate_bits::__SlateBits110U0,
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
    u1: __SlateRecord194,
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
    fg: __SlateRecord188,
    iCursor: i32,
    colUsed: u64,
    u1: __SlateRecord189,
    u2: __SlateRecord190,
    u3: __SlateRecord191,
    u4: __SlateRecord192,
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
    u: __SlateRecord170,
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
    __slate_bits_0: __slate_bits::__SlateBits166U0,
    azInit: *mut *const i8,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord167 {
    xLegacy: Option<unsafe extern "C-unwind" fn(*mut (), *const i8)>,
    xV2: Option<unsafe extern "C-unwind" fn(u32, *mut (), *mut (), *mut ()) -> i32>,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord168 {
    isInterrupted: i32,
    notUsed1: f64,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord169 {
    pHash: *mut FuncDef,
    pDestructor: *mut FuncDestructor,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord170 {
    tab: __SlateRecord171,
    view: __SlateRecord172,
    vtab: __SlateRecord173,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord171 {
    addColOffset: i32,
    pFKey: *mut FKey,
    pDfltList: *mut ExprList,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord172 {
    pSelect: *mut Select,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord173 {
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
union __SlateRecord178 {
    zToken: *mut i8,
    iValue: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord179 {
    pList: *mut ExprList,
    pSelect: *mut Select,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord180 {
    iJoin: i32,
    iOfst: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord181 {
    pTab: *mut Table,
    pWin: *mut Window,
    nReg: i32,
    sub: __SlateRecord182,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord182 {
    iAddr: i32,
    regReturn: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct ExprList_item {
    pExpr: *mut Expr,
    zEName: *mut i8,
    fg: __SlateRecord184,
    u: __SlateRecord185,
}

#[repr(C, align(4))]
#[derive(Clone, Copy)]
struct __SlateRecord184 {
    sortFlags: u8,
    __slate_bits_0: __slate_bits::__SlateBits184U0,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord185 {
    x: __SlateRecord186,
    iConstExprReg: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord186 {
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
struct __SlateRecord188 {
    jointype: u8,
    __slate_bits_0: __slate_bits::__SlateBits188U0,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord189 {
    zIndexedBy: *mut i8,
    pFuncArg: *mut ExprList,
    nRow: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord190 {
    pIBIndex: *mut Index,
    pCteUse: *mut CteUse,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord191 {
    pOn: *mut Expr,
    pUsing: *mut IdList,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord192 {
    pSchema: *mut Schema,
    zDatabase: *mut i8,
    pSubq: *mut Subquery,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord194 {
    cr: __SlateRecord195,
    d: __SlateRecord196,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord195 {
    addrCrTab: i32,
    regRowid: i32,
    regRoot: i32,
    constraintName: Token,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord196 {
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
    pub struct __SlateBits72U0 {
        #[bits(4)]
        pub notNull: u32,
        #[bits(4)]
        pub eCType: u32,
    }
    #[bitfields::bitfield([u8; 2], c_names = true, new = false, from_into_bits = false, from_traits = false, default = false, debug = false, builder = false, bit_ops = false)]
    pub struct __SlateBits184U0 {
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
    pub struct __SlateBits188U0 {
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
    pub struct __SlateBits98U0 {
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
    pub struct __SlateBits166U0 {
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
    pub struct __SlateBits110U0 {
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
// ** Tracing flags set by SQLITE_TESTCTRL_TRACEFLAGS.
// */
#[unsafe(no_mangle)]
static mut sqlite3TreeTrace: u32 = (0 as i32) as u32;

#[unsafe(no_mangle)]
static mut sqlite3WhereTrace: u32 = (0 as i32) as u32;

// /*
// ** Properties of opcodes.  The OPFLG_INITIALIZER macro is
// ** created by mkopcodeh.awk during compilation.  Data is obtained
// ** from the comments following the "case OP_xxxx:" statements in
// ** the vdbe.c file.
// */
#[unsafe(no_mangle)]
static mut sqlite3OpcodeProperty: __SlateAlign16<[u8; 192]> = __SlateAlign16([
    ((0 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((16 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((65 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((129 as i32) as i8) as u8,
    ((1 as i32) as i8) as u8,
    ((1 as i32) as i8) as u8,
    ((129 as i32) as i8) as u8,
    ((131 as i32) as i8) as u8,
    ((131 as i32) as i8) as u8,
    ((1 as i32) as i8) as u8,
    ((1 as i32) as i8) as u8,
    ((3 as i32) as i8) as u8,
    ((3 as i32) as i8) as u8,
    ((1 as i32) as i8) as u8,
    ((18 as i32) as i8) as u8,
    ((1 as i32) as i8) as u8,
    ((201 as i32) as i8) as u8,
    ((201 as i32) as i8) as u8,
    ((201 as i32) as i8) as u8,
    ((201 as i32) as i8) as u8,
    ((1 as i32) as i8) as u8,
    ((73 as i32) as i8) as u8,
    ((73 as i32) as i8) as u8,
    ((73 as i32) as i8) as u8,
    ((73 as i32) as i8) as u8,
    ((201 as i32) as i8) as u8,
    ((73 as i32) as i8) as u8,
    ((193 as i32) as i8) as u8,
    ((1 as i32) as i8) as u8,
    ((65 as i32) as i8) as u8,
    ((65 as i32) as i8) as u8,
    ((193 as i32) as i8) as u8,
    ((1 as i32) as i8) as u8,
    ((1 as i32) as i8) as u8,
    ((65 as i32) as i8) as u8,
    ((65 as i32) as i8) as u8,
    ((65 as i32) as i8) as u8,
    ((65 as i32) as i8) as u8,
    ((38 as i32) as i8) as u8,
    ((38 as i32) as i8) as u8,
    ((65 as i32) as i8) as u8,
    ((65 as i32) as i8) as u8,
    ((9 as i32) as i8) as u8,
    ((35 as i32) as i8) as u8,
    ((11 as i32) as i8) as u8,
    ((129 as i32) as i8) as u8,
    ((3 as i32) as i8) as u8,
    ((3 as i32) as i8) as u8,
    ((11 as i32) as i8) as u8,
    ((11 as i32) as i8) as u8,
    ((11 as i32) as i8) as u8,
    ((11 as i32) as i8) as u8,
    ((11 as i32) as i8) as u8,
    ((11 as i32) as i8) as u8,
    ((1 as i32) as i8) as u8,
    ((1 as i32) as i8) as u8,
    ((3 as i32) as i8) as u8,
    ((3 as i32) as i8) as u8,
    ((3 as i32) as i8) as u8,
    ((1 as i32) as i8) as u8,
    ((65 as i32) as i8) as u8,
    ((1 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((2 as i32) as i8) as u8,
    ((2 as i32) as i8) as u8,
    ((8 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((16 as i32) as i8) as u8,
    ((16 as i32) as i8) as u8,
    ((16 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((16 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((16 as i32) as i8) as u8,
    ((16 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((16 as i32) as i8) as u8,
    ((16 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((2 as i32) as i8) as u8,
    ((2 as i32) as i8) as u8,
    ((2 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((18 as i32) as i8) as u8,
    ((30 as i32) as i8) as u8,
    ((32 as i32) as i8) as u8,
    ((64 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((16 as i32) as i8) as u8,
    ((16 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((38 as i32) as i8) as u8,
    ((38 as i32) as i8) as u8,
    ((38 as i32) as i8) as u8,
    ((38 as i32) as i8) as u8,
    ((38 as i32) as i8) as u8,
    ((38 as i32) as i8) as u8,
    ((38 as i32) as i8) as u8,
    ((38 as i32) as i8) as u8,
    ((38 as i32) as i8) as u8,
    ((38 as i32) as i8) as u8,
    ((64 as i32) as i8) as u8,
    ((64 as i32) as i8) as u8,
    ((18 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((64 as i32) as i8) as u8,
    ((16 as i32) as i8) as u8,
    ((64 as i32) as i8) as u8,
    ((64 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((64 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((64 as i32) as i8) as u8,
    ((64 as i32) as i8) as u8,
    ((16 as i32) as i8) as u8,
    ((16 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((64 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((80 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((64 as i32) as i8) as u8,
    ((4 as i32) as i8) as u8,
    ((4 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((64 as i32) as i8) as u8,
    ((80 as i32) as i8) as u8,
    ((64 as i32) as i8) as u8,
    ((16 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((16 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((16 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((6 as i32) as i8) as u8,
    ((16 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((4 as i32) as i8) as u8,
    ((26 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((64 as i32) as i8) as u8,
    ((16 as i32) as i8) as u8,
    ((80 as i32) as i8) as u8,
    ((64 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((16 as i32) as i8) as u8,
    ((16 as i32) as i8) as u8,
    ((2 as i32) as i8) as u8,
    ((18 as i32) as i8) as u8,
    ((18 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
]);

// /*
// ** Name of the default collating sequence
// */
#[unsafe(no_mangle)]
static mut sqlite3StrBINARY: [i8; 7] = [
    66 as i8, 73 as i8, 78 as i8, 65 as i8, 82 as i8, 89 as i8, 0 as i8,
];

// /*
// ** Standard typenames.  These names must match the COLTYPE_* definitions.
// ** Adjust the SQLITE_N_STDTYPE value if adding or removing entries.
// **
// **    sqlite3StdType[]            The actual names of the datatypes.
// **
// **    sqlite3StdTypeLen[]         The length (in bytes) of each entry
// **                                in sqlite3StdType[].
// **
// **    sqlite3StdTypeAffinity[]    The affinity associated with each entry
// **                                in sqlite3StdType[].
// */
#[unsafe(no_mangle)]
static mut sqlite3StdTypeLen: [u8; 6] = [
    ((3 as i32) as i8) as u8,
    ((4 as i32) as i8) as u8,
    ((3 as i32) as i8) as u8,
    ((7 as i32) as i8) as u8,
    ((4 as i32) as i8) as u8,
    ((4 as i32) as i8) as u8,
];

#[unsafe(no_mangle)]
static mut sqlite3StdTypeAffinity: [i8; 6] = [
    (67 as i32) as i8,
    (65 as i32) as i8,
    (68 as i32) as i8,
    (68 as i32) as i8,
    (69 as i32) as i8,
    (66 as i32) as i8,
];

#[unsafe(no_mangle)]
static mut sqlite3StdType: __SlateAlign16<[*const i8; 6]> = __SlateAlign16([
    (b"ANY\0".as_ptr() as *mut i8) as *const i8,
    (b"BLOB\0".as_ptr() as *mut i8) as *const i8,
    (b"INT\0".as_ptr() as *mut i8) as *const i8,
    (b"INTEGER\0".as_ptr() as *mut i8) as *const i8,
    (b"REAL\0".as_ptr() as *mut i8) as *const i8,
    (b"TEXT\0".as_ptr() as *mut i8) as *const i8,
]);

// /*
// ** 2008 June 13
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
// ** This file contains definitions of global variables and constants.
// */
// /* An array to map all upper-case characters into their corresponding
// ** lower-case character.
// **
// ** SQLite only considers US-ASCII (or EBCDIC) characters.  We do not
// ** handle case conversions for the UTF character set since the tables
// ** involved are nearly as big or bigger than SQLite itself.
// */
#[unsafe(no_mangle)]
static mut sqlite3UpperToLower: __SlateAlign16<[u8; 274]> = __SlateAlign16([
    ((0 as i32) as i8) as u8,
    ((1 as i32) as i8) as u8,
    ((2 as i32) as i8) as u8,
    ((3 as i32) as i8) as u8,
    ((4 as i32) as i8) as u8,
    ((5 as i32) as i8) as u8,
    ((6 as i32) as i8) as u8,
    ((7 as i32) as i8) as u8,
    ((8 as i32) as i8) as u8,
    ((9 as i32) as i8) as u8,
    ((10 as i32) as i8) as u8,
    ((11 as i32) as i8) as u8,
    ((12 as i32) as i8) as u8,
    ((13 as i32) as i8) as u8,
    ((14 as i32) as i8) as u8,
    ((15 as i32) as i8) as u8,
    ((16 as i32) as i8) as u8,
    ((17 as i32) as i8) as u8,
    ((18 as i32) as i8) as u8,
    ((19 as i32) as i8) as u8,
    ((20 as i32) as i8) as u8,
    ((21 as i32) as i8) as u8,
    ((22 as i32) as i8) as u8,
    ((23 as i32) as i8) as u8,
    ((24 as i32) as i8) as u8,
    ((25 as i32) as i8) as u8,
    ((26 as i32) as i8) as u8,
    ((27 as i32) as i8) as u8,
    ((28 as i32) as i8) as u8,
    ((29 as i32) as i8) as u8,
    ((30 as i32) as i8) as u8,
    ((31 as i32) as i8) as u8,
    ((32 as i32) as i8) as u8,
    ((33 as i32) as i8) as u8,
    ((34 as i32) as i8) as u8,
    ((35 as i32) as i8) as u8,
    ((36 as i32) as i8) as u8,
    ((37 as i32) as i8) as u8,
    ((38 as i32) as i8) as u8,
    ((39 as i32) as i8) as u8,
    ((40 as i32) as i8) as u8,
    ((41 as i32) as i8) as u8,
    ((42 as i32) as i8) as u8,
    ((43 as i32) as i8) as u8,
    ((44 as i32) as i8) as u8,
    ((45 as i32) as i8) as u8,
    ((46 as i32) as i8) as u8,
    ((47 as i32) as i8) as u8,
    ((48 as i32) as i8) as u8,
    ((49 as i32) as i8) as u8,
    ((50 as i32) as i8) as u8,
    ((51 as i32) as i8) as u8,
    ((52 as i32) as i8) as u8,
    ((53 as i32) as i8) as u8,
    ((54 as i32) as i8) as u8,
    ((55 as i32) as i8) as u8,
    ((56 as i32) as i8) as u8,
    ((57 as i32) as i8) as u8,
    ((58 as i32) as i8) as u8,
    ((59 as i32) as i8) as u8,
    ((60 as i32) as i8) as u8,
    ((61 as i32) as i8) as u8,
    ((62 as i32) as i8) as u8,
    ((63 as i32) as i8) as u8,
    ((64 as i32) as i8) as u8,
    ((97 as i32) as i8) as u8,
    ((98 as i32) as i8) as u8,
    ((99 as i32) as i8) as u8,
    ((100 as i32) as i8) as u8,
    ((101 as i32) as i8) as u8,
    ((102 as i32) as i8) as u8,
    ((103 as i32) as i8) as u8,
    ((104 as i32) as i8) as u8,
    ((105 as i32) as i8) as u8,
    ((106 as i32) as i8) as u8,
    ((107 as i32) as i8) as u8,
    ((108 as i32) as i8) as u8,
    ((109 as i32) as i8) as u8,
    ((110 as i32) as i8) as u8,
    ((111 as i32) as i8) as u8,
    ((112 as i32) as i8) as u8,
    ((113 as i32) as i8) as u8,
    ((114 as i32) as i8) as u8,
    ((115 as i32) as i8) as u8,
    ((116 as i32) as i8) as u8,
    ((117 as i32) as i8) as u8,
    ((118 as i32) as i8) as u8,
    ((119 as i32) as i8) as u8,
    ((120 as i32) as i8) as u8,
    ((121 as i32) as i8) as u8,
    ((122 as i32) as i8) as u8,
    ((91 as i32) as i8) as u8,
    ((92 as i32) as i8) as u8,
    ((93 as i32) as i8) as u8,
    ((94 as i32) as i8) as u8,
    ((95 as i32) as i8) as u8,
    ((96 as i32) as i8) as u8,
    ((97 as i32) as i8) as u8,
    ((98 as i32) as i8) as u8,
    ((99 as i32) as i8) as u8,
    ((100 as i32) as i8) as u8,
    ((101 as i32) as i8) as u8,
    ((102 as i32) as i8) as u8,
    ((103 as i32) as i8) as u8,
    ((104 as i32) as i8) as u8,
    ((105 as i32) as i8) as u8,
    ((106 as i32) as i8) as u8,
    ((107 as i32) as i8) as u8,
    ((108 as i32) as i8) as u8,
    ((109 as i32) as i8) as u8,
    ((110 as i32) as i8) as u8,
    ((111 as i32) as i8) as u8,
    ((112 as i32) as i8) as u8,
    ((113 as i32) as i8) as u8,
    ((114 as i32) as i8) as u8,
    ((115 as i32) as i8) as u8,
    ((116 as i32) as i8) as u8,
    ((117 as i32) as i8) as u8,
    ((118 as i32) as i8) as u8,
    ((119 as i32) as i8) as u8,
    ((120 as i32) as i8) as u8,
    ((121 as i32) as i8) as u8,
    ((122 as i32) as i8) as u8,
    ((123 as i32) as i8) as u8,
    ((124 as i32) as i8) as u8,
    ((125 as i32) as i8) as u8,
    ((126 as i32) as i8) as u8,
    ((127 as i32) as i8) as u8,
    ((128 as i32) as i8) as u8,
    ((129 as i32) as i8) as u8,
    ((130 as i32) as i8) as u8,
    ((131 as i32) as i8) as u8,
    ((132 as i32) as i8) as u8,
    ((133 as i32) as i8) as u8,
    ((134 as i32) as i8) as u8,
    ((135 as i32) as i8) as u8,
    ((136 as i32) as i8) as u8,
    ((137 as i32) as i8) as u8,
    ((138 as i32) as i8) as u8,
    ((139 as i32) as i8) as u8,
    ((140 as i32) as i8) as u8,
    ((141 as i32) as i8) as u8,
    ((142 as i32) as i8) as u8,
    ((143 as i32) as i8) as u8,
    ((144 as i32) as i8) as u8,
    ((145 as i32) as i8) as u8,
    ((146 as i32) as i8) as u8,
    ((147 as i32) as i8) as u8,
    ((148 as i32) as i8) as u8,
    ((149 as i32) as i8) as u8,
    ((150 as i32) as i8) as u8,
    ((151 as i32) as i8) as u8,
    ((152 as i32) as i8) as u8,
    ((153 as i32) as i8) as u8,
    ((154 as i32) as i8) as u8,
    ((155 as i32) as i8) as u8,
    ((156 as i32) as i8) as u8,
    ((157 as i32) as i8) as u8,
    ((158 as i32) as i8) as u8,
    ((159 as i32) as i8) as u8,
    ((160 as i32) as i8) as u8,
    ((161 as i32) as i8) as u8,
    ((162 as i32) as i8) as u8,
    ((163 as i32) as i8) as u8,
    ((164 as i32) as i8) as u8,
    ((165 as i32) as i8) as u8,
    ((166 as i32) as i8) as u8,
    ((167 as i32) as i8) as u8,
    ((168 as i32) as i8) as u8,
    ((169 as i32) as i8) as u8,
    ((170 as i32) as i8) as u8,
    ((171 as i32) as i8) as u8,
    ((172 as i32) as i8) as u8,
    ((173 as i32) as i8) as u8,
    ((174 as i32) as i8) as u8,
    ((175 as i32) as i8) as u8,
    ((176 as i32) as i8) as u8,
    ((177 as i32) as i8) as u8,
    ((178 as i32) as i8) as u8,
    ((179 as i32) as i8) as u8,
    ((180 as i32) as i8) as u8,
    ((181 as i32) as i8) as u8,
    ((182 as i32) as i8) as u8,
    ((183 as i32) as i8) as u8,
    ((184 as i32) as i8) as u8,
    ((185 as i32) as i8) as u8,
    ((186 as i32) as i8) as u8,
    ((187 as i32) as i8) as u8,
    ((188 as i32) as i8) as u8,
    ((189 as i32) as i8) as u8,
    ((190 as i32) as i8) as u8,
    ((191 as i32) as i8) as u8,
    ((192 as i32) as i8) as u8,
    ((193 as i32) as i8) as u8,
    ((194 as i32) as i8) as u8,
    ((195 as i32) as i8) as u8,
    ((196 as i32) as i8) as u8,
    ((197 as i32) as i8) as u8,
    ((198 as i32) as i8) as u8,
    ((199 as i32) as i8) as u8,
    ((200 as i32) as i8) as u8,
    ((201 as i32) as i8) as u8,
    ((202 as i32) as i8) as u8,
    ((203 as i32) as i8) as u8,
    ((204 as i32) as i8) as u8,
    ((205 as i32) as i8) as u8,
    ((206 as i32) as i8) as u8,
    ((207 as i32) as i8) as u8,
    ((208 as i32) as i8) as u8,
    ((209 as i32) as i8) as u8,
    ((210 as i32) as i8) as u8,
    ((211 as i32) as i8) as u8,
    ((212 as i32) as i8) as u8,
    ((213 as i32) as i8) as u8,
    ((214 as i32) as i8) as u8,
    ((215 as i32) as i8) as u8,
    ((216 as i32) as i8) as u8,
    ((217 as i32) as i8) as u8,
    ((218 as i32) as i8) as u8,
    ((219 as i32) as i8) as u8,
    ((220 as i32) as i8) as u8,
    ((221 as i32) as i8) as u8,
    ((222 as i32) as i8) as u8,
    ((223 as i32) as i8) as u8,
    ((224 as i32) as i8) as u8,
    ((225 as i32) as i8) as u8,
    ((226 as i32) as i8) as u8,
    ((227 as i32) as i8) as u8,
    ((228 as i32) as i8) as u8,
    ((229 as i32) as i8) as u8,
    ((230 as i32) as i8) as u8,
    ((231 as i32) as i8) as u8,
    ((232 as i32) as i8) as u8,
    ((233 as i32) as i8) as u8,
    ((234 as i32) as i8) as u8,
    ((235 as i32) as i8) as u8,
    ((236 as i32) as i8) as u8,
    ((237 as i32) as i8) as u8,
    ((238 as i32) as i8) as u8,
    ((239 as i32) as i8) as u8,
    ((240 as i32) as i8) as u8,
    ((241 as i32) as i8) as u8,
    ((242 as i32) as i8) as u8,
    ((243 as i32) as i8) as u8,
    ((244 as i32) as i8) as u8,
    ((245 as i32) as i8) as u8,
    ((246 as i32) as i8) as u8,
    ((247 as i32) as i8) as u8,
    ((248 as i32) as i8) as u8,
    ((249 as i32) as i8) as u8,
    ((250 as i32) as i8) as u8,
    ((251 as i32) as i8) as u8,
    ((252 as i32) as i8) as u8,
    ((253 as i32) as i8) as u8,
    ((254 as i32) as i8) as u8,
    ((255 as i32) as i8) as u8,
    ((1 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((1 as i32) as i8) as u8,
    ((1 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((1 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((1 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((1 as i32) as i8) as u8,
    ((1 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((1 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((1 as i32) as i8) as u8,
]);

// /* All of the upper-to-lower conversion data is above.  The following
// ** 18 integers are completely unrelated.  They are appended to the
// ** sqlite3UpperToLower[] array to avoid UBSAN warnings.  Here's what is
// ** going on:
// **
// ** The SQL comparison operators (<>, =, >, <=, <, and >=) are implemented
// ** by invoking sqlite3MemCompare(A,B) which compares values A and B and
// ** returns negative, zero, or positive if A is less then, equal to, or
// ** greater than B, respectively.  Then the true false results is found by
// ** consulting sqlite3aLTb[opcode], sqlite3aEQb[opcode], or
// ** sqlite3aGTb[opcode] depending on whether the result of compare(A,B)
// ** is negative, zero, or positive, where opcode is the specific opcode.
// ** The only works because the comparison opcodes are consecutive and in
// ** this order: NE EQ GT LE LT GE.  Various assert()s throughout the code
// ** ensure that is the case.
// **
// ** These elements must be appended to another array.  Otherwise the
// ** index (here shown as [256-OP_Ne]) would be out-of-bounds and thus
// ** be undefined behavior.  That's goofy, but the C-standards people thought
// ** it was a good idea, so here we are.
// */
// /* NE  EQ  GT  LE  LT  GE  */
// /* aLTb[]: Use when compare(A,B) less than zero */
// /* aEQb[]: Use when compare(A,B) equals zero */
// /* aGTb[]: Use when compare(A,B) greater than zero*/
#[unsafe(no_mangle)]
static mut sqlite3aLTb: *const u8 = unsafe {
    unsafe { std::ptr::addr_of!(sqlite3UpperToLower.0) as *const u8 }
        .offset(((256 as i32) - (53 as i32)) as isize)
};

#[unsafe(no_mangle)]
static mut sqlite3aEQb: *const u8 = unsafe {
    unsafe { std::ptr::addr_of!(sqlite3UpperToLower.0) as *const u8 }
        .offset(((256 as i32) + (6 as i32) - (53 as i32)) as isize)
};

#[unsafe(no_mangle)]
static mut sqlite3aGTb: *const u8 = unsafe {
    unsafe { std::ptr::addr_of!(sqlite3UpperToLower.0) as *const u8 }
        .offset(((256 as i32) + (12 as i32) - (53 as i32)) as isize)
};

// /*
// ** The following 256 byte lookup table is used to support SQLites built-in
// ** equivalents to the following standard library functions:
// **
// **   isspace()                        0x01
// **   isalpha()                        0x02
// **   isdigit()                        0x04
// **   isalnum()                        0x06
// **   isxdigit()                       0x08
// **   toupper()                        0x20
// **   SQLite identifier character      0x40   $, _, or non-ascii
// **   Quote character                  0x80
// **
// ** Bit 0x20 is set if the mapped character requires translation to upper
// ** case. i.e. if the character is a lower-case ASCII character.
// ** If x is a lower-case ASCII character, then its upper-case equivalent
// ** is (x - 0x20). Therefore toupper() can be implemented as:
// **
// **   (x & ~(map[x]&0x20))
// **
// ** The equivalent of tolower() is implemented using the sqlite3UpperToLower[]
// ** array. tolower() is used more often than toupper() by SQLite.
// **
// ** Bit 0x40 is set if the character is non-alphanumeric and can be used in an
// ** SQLite identifier.  Identifiers are alphanumerics, "_", "$", and any
// ** non-ASCII UTF character. Hence the test for whether or not a character is
// ** part of an identifier is 0x46.
// */
#[unsafe(no_mangle)]
static mut sqlite3CtypeMap: __SlateAlign16<[u8; 256]> = __SlateAlign16([
    ((0 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((1 as i32) as i8) as u8,
    ((1 as i32) as i8) as u8,
    ((1 as i32) as i8) as u8,
    ((1 as i32) as i8) as u8,
    ((1 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((1 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((128 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((64 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((128 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((12 as i32) as i8) as u8,
    ((12 as i32) as i8) as u8,
    ((12 as i32) as i8) as u8,
    ((12 as i32) as i8) as u8,
    ((12 as i32) as i8) as u8,
    ((12 as i32) as i8) as u8,
    ((12 as i32) as i8) as u8,
    ((12 as i32) as i8) as u8,
    ((12 as i32) as i8) as u8,
    ((12 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((10 as i32) as i8) as u8,
    ((10 as i32) as i8) as u8,
    ((10 as i32) as i8) as u8,
    ((10 as i32) as i8) as u8,
    ((10 as i32) as i8) as u8,
    ((10 as i32) as i8) as u8,
    ((2 as i32) as i8) as u8,
    ((2 as i32) as i8) as u8,
    ((2 as i32) as i8) as u8,
    ((2 as i32) as i8) as u8,
    ((2 as i32) as i8) as u8,
    ((2 as i32) as i8) as u8,
    ((2 as i32) as i8) as u8,
    ((2 as i32) as i8) as u8,
    ((2 as i32) as i8) as u8,
    ((2 as i32) as i8) as u8,
    ((2 as i32) as i8) as u8,
    ((2 as i32) as i8) as u8,
    ((2 as i32) as i8) as u8,
    ((2 as i32) as i8) as u8,
    ((2 as i32) as i8) as u8,
    ((2 as i32) as i8) as u8,
    ((2 as i32) as i8) as u8,
    ((2 as i32) as i8) as u8,
    ((2 as i32) as i8) as u8,
    ((2 as i32) as i8) as u8,
    ((128 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((64 as i32) as i8) as u8,
    ((128 as i32) as i8) as u8,
    ((42 as i32) as i8) as u8,
    ((42 as i32) as i8) as u8,
    ((42 as i32) as i8) as u8,
    ((42 as i32) as i8) as u8,
    ((42 as i32) as i8) as u8,
    ((42 as i32) as i8) as u8,
    ((34 as i32) as i8) as u8,
    ((34 as i32) as i8) as u8,
    ((34 as i32) as i8) as u8,
    ((34 as i32) as i8) as u8,
    ((34 as i32) as i8) as u8,
    ((34 as i32) as i8) as u8,
    ((34 as i32) as i8) as u8,
    ((34 as i32) as i8) as u8,
    ((34 as i32) as i8) as u8,
    ((34 as i32) as i8) as u8,
    ((34 as i32) as i8) as u8,
    ((34 as i32) as i8) as u8,
    ((34 as i32) as i8) as u8,
    ((34 as i32) as i8) as u8,
    ((34 as i32) as i8) as u8,
    ((34 as i32) as i8) as u8,
    ((34 as i32) as i8) as u8,
    ((34 as i32) as i8) as u8,
    ((34 as i32) as i8) as u8,
    ((34 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((64 as i32) as i8) as u8,
    ((64 as i32) as i8) as u8,
    ((64 as i32) as i8) as u8,
    ((64 as i32) as i8) as u8,
    ((64 as i32) as i8) as u8,
    ((64 as i32) as i8) as u8,
    ((64 as i32) as i8) as u8,
    ((64 as i32) as i8) as u8,
    ((64 as i32) as i8) as u8,
    ((64 as i32) as i8) as u8,
    ((64 as i32) as i8) as u8,
    ((64 as i32) as i8) as u8,
    ((64 as i32) as i8) as u8,
    ((64 as i32) as i8) as u8,
    ((64 as i32) as i8) as u8,
    ((64 as i32) as i8) as u8,
    ((64 as i32) as i8) as u8,
    ((64 as i32) as i8) as u8,
    ((64 as i32) as i8) as u8,
    ((64 as i32) as i8) as u8,
    ((64 as i32) as i8) as u8,
    ((64 as i32) as i8) as u8,
    ((64 as i32) as i8) as u8,
    ((64 as i32) as i8) as u8,
    ((64 as i32) as i8) as u8,
    ((64 as i32) as i8) as u8,
    ((64 as i32) as i8) as u8,
    ((64 as i32) as i8) as u8,
    ((64 as i32) as i8) as u8,
    ((64 as i32) as i8) as u8,
    ((64 as i32) as i8) as u8,
    ((64 as i32) as i8) as u8,
    ((64 as i32) as i8) as u8,
    ((64 as i32) as i8) as u8,
    ((64 as i32) as i8) as u8,
    ((64 as i32) as i8) as u8,
    ((64 as i32) as i8) as u8,
    ((64 as i32) as i8) as u8,
    ((64 as i32) as i8) as u8,
    ((64 as i32) as i8) as u8,
    ((64 as i32) as i8) as u8,
    ((64 as i32) as i8) as u8,
    ((64 as i32) as i8) as u8,
    ((64 as i32) as i8) as u8,
    ((64 as i32) as i8) as u8,
    ((64 as i32) as i8) as u8,
    ((64 as i32) as i8) as u8,
    ((64 as i32) as i8) as u8,
    ((64 as i32) as i8) as u8,
    ((64 as i32) as i8) as u8,
    ((64 as i32) as i8) as u8,
    ((64 as i32) as i8) as u8,
    ((64 as i32) as i8) as u8,
    ((64 as i32) as i8) as u8,
    ((64 as i32) as i8) as u8,
    ((64 as i32) as i8) as u8,
    ((64 as i32) as i8) as u8,
    ((64 as i32) as i8) as u8,
    ((64 as i32) as i8) as u8,
    ((64 as i32) as i8) as u8,
    ((64 as i32) as i8) as u8,
    ((64 as i32) as i8) as u8,
    ((64 as i32) as i8) as u8,
    ((64 as i32) as i8) as u8,
    ((64 as i32) as i8) as u8,
    ((64 as i32) as i8) as u8,
    ((64 as i32) as i8) as u8,
    ((64 as i32) as i8) as u8,
    ((64 as i32) as i8) as u8,
    ((64 as i32) as i8) as u8,
    ((64 as i32) as i8) as u8,
    ((64 as i32) as i8) as u8,
    ((64 as i32) as i8) as u8,
    ((64 as i32) as i8) as u8,
    ((64 as i32) as i8) as u8,
    ((64 as i32) as i8) as u8,
    ((64 as i32) as i8) as u8,
    ((64 as i32) as i8) as u8,
    ((64 as i32) as i8) as u8,
    ((64 as i32) as i8) as u8,
    ((64 as i32) as i8) as u8,
    ((64 as i32) as i8) as u8,
    ((64 as i32) as i8) as u8,
    ((64 as i32) as i8) as u8,
    ((64 as i32) as i8) as u8,
    ((64 as i32) as i8) as u8,
    ((64 as i32) as i8) as u8,
    ((64 as i32) as i8) as u8,
    ((64 as i32) as i8) as u8,
    ((64 as i32) as i8) as u8,
    ((64 as i32) as i8) as u8,
    ((64 as i32) as i8) as u8,
    ((64 as i32) as i8) as u8,
    ((64 as i32) as i8) as u8,
    ((64 as i32) as i8) as u8,
    ((64 as i32) as i8) as u8,
    ((64 as i32) as i8) as u8,
    ((64 as i32) as i8) as u8,
    ((64 as i32) as i8) as u8,
    ((64 as i32) as i8) as u8,
    ((64 as i32) as i8) as u8,
    ((64 as i32) as i8) as u8,
    ((64 as i32) as i8) as u8,
    ((64 as i32) as i8) as u8,
    ((64 as i32) as i8) as u8,
    ((64 as i32) as i8) as u8,
    ((64 as i32) as i8) as u8,
    ((64 as i32) as i8) as u8,
    ((64 as i32) as i8) as u8,
    ((64 as i32) as i8) as u8,
    ((64 as i32) as i8) as u8,
    ((64 as i32) as i8) as u8,
    ((64 as i32) as i8) as u8,
    ((64 as i32) as i8) as u8,
    ((64 as i32) as i8) as u8,
    ((64 as i32) as i8) as u8,
    ((64 as i32) as i8) as u8,
    ((64 as i32) as i8) as u8,
    ((64 as i32) as i8) as u8,
    ((64 as i32) as i8) as u8,
    ((64 as i32) as i8) as u8,
    ((64 as i32) as i8) as u8,
    ((64 as i32) as i8) as u8,
    ((64 as i32) as i8) as u8,
    ((64 as i32) as i8) as u8,
    ((64 as i32) as i8) as u8,
    ((64 as i32) as i8) as u8,
    ((64 as i32) as i8) as u8,
]);

// /* 00..07    ........ */
// /* 08..0f    ........ */
// /* 10..17    ........ */
// /* 18..1f    ........ */
// /* 20..27     !"#$%&' */
// /* 28..2f    ()*+,-./ */
// /* 30..37    01234567 */
// /* 38..3f    89:;<=>? */
// /* 40..47    @ABCDEFG */
// /* 48..4f    HIJKLMNO */
// /* 50..57    PQRSTUVW */
// /* 58..5f    XYZ[\]^_ */
// /* 60..67    `abcdefg */
// /* 68..6f    hijklmno */
// /* 70..77    pqrstuvw */
// /* 78..7f    xyz{|}~. */
// /* 80..87    ........ */
// /* 88..8f    ........ */
// /* 90..97    ........ */
// /* 98..9f    ........ */
// /* a0..a7    ........ */
// /* a8..af    ........ */
// /* b0..b7    ........ */
// /* b8..bf    ........ */
// /* c0..c7    ........ */
// /* c8..cf    ........ */
// /* d0..d7    ........ */
// /* d8..df    ........ */
// /* e0..e7    ........ */
// /* e8..ef    ........ */
// /* f0..f7    ........ */
// /* f8..ff    ........ */
// /* EVIDENCE-OF: R-02982-34736 In order to maintain full backwards
// ** compatibility for legacy applications, the URI filename capability is
// ** disabled by default.
// **
// ** EVIDENCE-OF: R-38799-08373 URI filenames can be enabled or disabled
// ** using the SQLITE_USE_URI=1 or SQLITE_USE_URI=0 compile-time options.
// **
// ** EVIDENCE-OF: R-43642-56306 By default, URI handling is globally
// ** disabled. The default value may be changed by compiling with the
// ** SQLITE_USE_URI symbol defined.
// */
// /* EVIDENCE-OF: R-38720-18127 The default setting is determined by the
// ** SQLITE_ALLOW_COVERING_INDEX_SCAN compile-time option, or is "on" if
// ** that compile-time option is omitted.
// */
// /* The minimum PMA size is set to this value multiplied by the database
// ** page size in bytes.
// */
// /* Statement journals spill to disk when their size exceeds the following
// ** threshold (in bytes). 0 means that statement journals are created and
// ** written to disk immediately (the default behavior for SQLite versions
// ** before 3.12.0).  -1 means always keep the entire statement journal in
// ** memory.  (The statement journal is also always held entirely in memory
// ** if journal_mode=MEMORY or if temp_store=MEMORY, regardless of this
// ** setting.)
// */
// /*
// ** The default lookaside-configuration, the format "SZ,N".  SZ is the
// ** number of bytes in each lookaside slot (should be a multiple of 8)
// ** and N is the number of slots.  The lookaside-configuration can be
// ** changed as start-time using sqlite3_config(SQLITE_CONFIG_LOOKASIDE)
// ** or at run-time for an individual database connection using
// ** sqlite3_db_config(db, SQLITE_DBCONFIG_LOOKASIDE);
// **
// ** With the two-size-lookaside enhancement, less lookaside is required.
// ** The default configuration of 1200,40 actually provides 30 1200-byte slots
// ** and 93 128-byte slots, which is more lookaside than is available
// ** using the older 1200,100 configuration without two-size-lookaside.
// */
// /* 48KB of memory */
// /* The default maximum size of an in-memory database created using
// ** sqlite3_deserialize()
// */
// /*
// ** The following singleton contains the global configuration for
// ** the SQLite library.
// */
#[unsafe(no_mangle)]
static mut sqlite3Config: Sqlite3Config = Sqlite3Config {
    bMemstat: 1 as i32,
    bCoreMutex: ((1 as i32) as i8) as u8,
    bFullMutex: ((1 as i32) == (1 as i32)) as u8,
    bOpenUri: ((0 as i32) as i8) as u8,
    bUseCis: ((1 as i32) as i8) as u8,
    bSmallMalloc: ((0 as i32) as i8) as u8,
    bExtraSchemaChecks: ((1 as i32) as i8) as u8,
    mxStrlen: 2147483646 as i32,
    neverCorrupt: 0 as i32,
    szLookaside: 1200 as i32,
    nLookaside: 40 as i32,
    nStmtSpill: (64 as i32) * (1024 as i32),
    m: sqlite3_mem_methods {
        xMalloc: None,
        xFree: None,
        xRealloc: None,
        xSize: None,
        xRoundup: None,
        xInit: None,
        xShutdown: None,
        pAppData: std::ptr::null_mut::<()>(),
    },
    mutex: sqlite3_mutex_methods {
        xMutexInit: None,
        xMutexEnd: None,
        xMutexAlloc: None,
        xMutexFree: None,
        xMutexEnter: None,
        xMutexTry: None,
        xMutexLeave: None,
        xMutexHeld: None,
        xMutexNotheld: None,
    },
    pcache2: sqlite3_pcache_methods2 {
        iVersion: 0 as i32,
        pArg: std::ptr::null_mut::<()>(),
        xInit: None,
        xShutdown: None,
        xCreate: None,
        xCachesize: None,
        xPagecount: None,
        xFetch: None,
        xUnpin: None,
        xRekey: None,
        xTruncate: None,
        xDestroy: None,
        xShrink: None,
    },
    pHeap: std::ptr::null_mut::<()>(),
    nHeap: 0 as i32,
    mnReq: 0 as i32,
    mxReq: 0 as i32,
    szMmap: (0 as i32) as i64,
    mxMmap: (2147418112 as i32) as i64,
    pPage: std::ptr::null_mut::<()>(),
    szPage: 0 as i32,
    nPage: 20 as i32,
    mxParserStack: 0 as i32,
    sharedCacheEnabled: 0 as i32,
    szPma: (250 as i32) as u32,
    isInit: 0 as i32,
    inProgress: 0 as i32,
    isMutexInit: 0 as i32,
    isMallocInit: 0 as i32,
    isPCacheInit: 0 as i32,
    nRefInitMutex: 0 as i32,
    pInitMutex: std::ptr::null_mut::<sqlite3_mutex>(),
    xLog: None,
    pLogArg: std::ptr::null_mut::<()>(),
    mxMemdbSize: (1073741824 as i32) as i64,
    xTestCallback: None,
    bLocaltimeFault: 0 as i32,
    xAltLocaltime: None,
    iOnceResetThreshold: 2147483646 as i32,
    szSorterRef: (2147483647 as i32) as u32,
    iPrngSeed: (0 as i32) as u32,
};

// /* bMemstat */
// /* bCoreMutex */
// /* bFullMutex */
// /* bOpenUri */
// /* bUseCis */
// /* bSmallMalloc */
// /* bExtraSchemaChecks */
// /* mxStrlen */
// /* neverCorrupt */
// /* szLookaside, nLookaside */
// /* nStmtSpill */
// /* m */
// /* mutex */
// /* pcache2 */
// /* pHeap */
// /* nHeap */
// /* mnHeap, mxHeap */
// /* szMmap */
// /* mxMmap */
// /* pPage */
// /* szPage */
// /* nPage */
// /* mxParserStack */
// /* sharedCacheEnabled */
// /* szPma */
// /* All the rest should always be initialized to zero */
// /* isInit */
// /* inProgress */
// /* isMutexInit */
// /* isMallocInit */
// /* isPCacheInit */
// /* nRefInitMutex */
// /* pInitMutex */
// /* xLog */
// /* pLogArg */
// /* mxMemdbSize */
// /* xTestCallback */
// /* bLocaltimeFault */
// /* xAltLocaltime */
// /* iOnceResetThreshold */
// /* szSorterRef */
// /* iPrngSeed */
// /*
// ** Hash table for global functions - functions common to all
// ** database connections.  After initialization, this table is
// ** read-only.
// */
#[unsafe(no_mangle)]
static mut sqlite3BuiltinFunctions: FuncDefHash = unsafe { std::mem::zeroed() };

// /*
// ** This singleton is an sqlite3_str object that is returned if
// ** sqlite3_malloc() fails to provide space for a real one.  This
// ** sqlite3_str object accepts no new text and always returns
// ** an SQLITE_NOMEM error.
// */
#[unsafe(no_mangle)]
static mut sqlite3OomStr: sqlite3_str = sqlite3_str {
    db: std::ptr::null_mut::<sqlite3>(),
    zText: std::ptr::null_mut::<i8>(),
    nAlloc: (0 as i32) as u32,
    mxAlloc: (0 as i32) as u32,
    nChar: (0 as i32) as u32,
    accError: ((7 as i32) as i8) as u8,
    printfFlags: ((0 as i32) as i8) as u8,
};

// /*
// ** The value of the "pending" byte must be 0x40000000 (1 byte past the
// ** 1-gibabyte boundary) in a compatible database.  SQLite never uses
// ** the database page that contains the pending byte.  It never attempts
// ** to read or write that page.  The pending byte page is set aside
// ** for use by the VFS layers as space for managing file locks.
// **
// ** During testing, it is often desirable to move the pending byte to
// ** a different position in the file.  This allows code that has to
// ** deal with the pending byte to run on files that are much smaller
// ** than 1 GiB.  The sqlite3_test_control() interface can be used to
// ** move the pending byte.
// **
// ** IMPORTANT:  Changing the pending byte to any value other than
// ** 0x40000000 results in an incompatible database file format!
// ** Changing the pending byte during operation will result in undefined
// ** and incorrect behavior.
// */
#[unsafe(no_mangle)]
static mut sqlite3PendingByte: i32 = 1073741824 as i32;
