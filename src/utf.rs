unsafe extern "C" {
    fn memmove(__dest: *mut (), __src: *const (), __n: u64) -> *mut ();
    fn memset(__s: *mut (), __c: i32, __n: u64) -> *mut ();
    fn sqlite3DbMallocRaw(__v366: *mut sqlite3, __v367: u64) -> *mut ();
    fn sqlite3DbMallocSize(__v368: *mut sqlite3, __v369: *const ()) -> i32;
    fn sqlite3VdbeChangeEncoding(__v385: *mut sqlite3_value, __v386: i32) -> i32;
    fn sqlite3VdbeMemSetStr(
        __v387: *mut sqlite3_value,
        __v388: *const i8,
        __v389: i64,
        __v390: u8,
        __v391: Option<unsafe extern "C-unwind" fn(*mut ())>,
    ) -> i32;
    fn sqlite3VdbeMemMakeWriteable(__v392: *mut sqlite3_value) -> i32;
    fn sqlite3VdbeMemRelease(p: *mut sqlite3_value);
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
    trace: __SlateRecord160,
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
    u1: __SlateRecord161,
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
    __slate_bits_0: __slate_bits::__SlateBits65U0,
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
    u: __SlateRecord171,
    pLeft: *mut Expr,
    pRight: *mut Expr,
    x: __SlateRecord172,
    nHeight: i32,
    iTable: i32,
    iColumn: i16,
    iAgg: i16,
    w: __SlateRecord173,
    pAggInfo: *mut AggInfo,
    y: __SlateRecord174,
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
    u: __SlateRecord162,
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
    __slate_bits_0: __slate_bits::__SlateBits89U0,
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
    __slate_bits_0: __slate_bits::__SlateBits101U0,
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
    u1: __SlateRecord187,
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
    fg: __SlateRecord181,
    iCursor: i32,
    colUsed: u64,
    u1: __SlateRecord182,
    u2: __SlateRecord183,
    u3: __SlateRecord184,
    u4: __SlateRecord185,
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
    u: __SlateRecord163,
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
    __slate_bits_0: __slate_bits::__SlateBits148U0,
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
    __slate_bits_0: __slate_bits::__SlateBits159U0,
    azInit: *mut *const i8,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord160 {
    xLegacy: Option<unsafe extern "C-unwind" fn(*mut (), *const i8)>,
    xV2: Option<unsafe extern "C-unwind" fn(u32, *mut (), *mut (), *mut ()) -> i32>,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord161 {
    isInterrupted: i32,
    notUsed1: f64,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord162 {
    pHash: *mut FuncDef,
    pDestructor: *mut FuncDestructor,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord163 {
    tab: __SlateRecord164,
    view: __SlateRecord165,
    vtab: __SlateRecord166,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord164 {
    addColOffset: i32,
    pFKey: *mut FKey,
    pDfltList: *mut ExprList,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord165 {
    pSelect: *mut Select,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord166 {
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
union __SlateRecord171 {
    zToken: *mut i8,
    iValue: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord172 {
    pList: *mut ExprList,
    pSelect: *mut Select,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord173 {
    iJoin: i32,
    iOfst: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord174 {
    pTab: *mut Table,
    pWin: *mut Window,
    nReg: i32,
    sub: __SlateRecord175,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord175 {
    iAddr: i32,
    regReturn: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct ExprList_item {
    pExpr: *mut Expr,
    zEName: *mut i8,
    fg: __SlateRecord177,
    u: __SlateRecord178,
}

#[repr(C, align(4))]
#[derive(Clone, Copy)]
struct __SlateRecord177 {
    sortFlags: u8,
    __slate_bits_0: __slate_bits::__SlateBits177U0,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord178 {
    x: __SlateRecord179,
    iConstExprReg: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord179 {
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
struct __SlateRecord181 {
    jointype: u8,
    __slate_bits_0: __slate_bits::__SlateBits181U0,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord182 {
    zIndexedBy: *mut i8,
    pFuncArg: *mut ExprList,
    nRow: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord183 {
    pIBIndex: *mut Index,
    pCteUse: *mut CteUse,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord184 {
    pOn: *mut Expr,
    pUsing: *mut IdList,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord185 {
    pSchema: *mut Schema,
    zDatabase: *mut i8,
    pSubq: *mut Subquery,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord187 {
    cr: __SlateRecord188,
    d: __SlateRecord189,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord188 {
    addrCrTab: i32,
    regRowid: i32,
    regRoot: i32,
    constraintName: Token,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord189 {
    pReturning: *mut Returning,
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
    __slate_bits_0: __slate_bits::__SlateBits198U0,
    seekHit: u16,
    ub: __SlateRecord200,
    seqCount: i64,
    cacheStatus: u32,
    seekResult: i32,
    pAltCursor: *mut VdbeCursor,
    uc: __SlateRecord201,
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
union __SlateRecord200 {
    pBtx: *mut Btree,
    aAltMap: *mut u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord201 {
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

#[repr(C, align(16))]
struct __SlateAlign16<T>(T);

mod __slate_bits {
    #[bitfields::bitfield([u8; 2], c_names = true, new = false, from_into_bits = false, from_traits = false, default = false, debug = false, builder = false, bit_ops = false)]
    pub struct __SlateBits101U0 {
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
    pub struct __SlateBits65U0 {
        #[bits(4)]
        pub notNull: u32,
        #[bits(4)]
        pub eCType: u32,
    }
    #[bitfields::bitfield([u8; 2], c_names = true, new = false, from_into_bits = false, from_traits = false, default = false, debug = false, builder = false, bit_ops = false)]
    pub struct __SlateBits89U0 {
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
    pub struct __SlateBits181U0 {
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
    pub struct __SlateBits177U0 {
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
    pub struct __SlateBits198U0 {
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
    pub struct __SlateBits148U0 {
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
    pub struct __SlateBits159U0 {
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

// /*
// ** 2004 April 13
// **
// ** The author disclaims copyright to this source code.  In place of
// ** a legal notice, here is a blessing:
// **
// **    May you do good and not evil.
// **    May you find forgiveness for yourself and forgive others.
// **    May you share freely, never taking more than you give.
// **
// *************************************************************************
// ** This file contains routines used to translate between UTF-8,
// ** UTF-16, UTF-16BE, and UTF-16LE.
// **
// ** Notes on UTF-8:
// **
// **   Byte-0    Byte-1    Byte-2    Byte-3    Value
// **  0xxxxxxx                                 00000000 00000000 0xxxxxxx
// **  110yyyyy  10xxxxxx                       00000000 00000yyy yyxxxxxx
// **  1110zzzz  10yyyyyy  10xxxxxx             00000000 zzzzyyyy yyxxxxxx
// **  11110uuu  10uuzzzz  10yyyyyy  10xxxxxx   000uuuuu zzzzyyyy yyxxxxxx
// **
// **
// ** Notes on UTF-16:  (with wwww+1==uuuuu)
// **
// **      Word-0               Word-1          Value
// **  110110ww wwzzzzyy   110111yy yyxxxxxx    000uuuuu zzzzyyyy yyxxxxxx
// **  zzzzyyyy yyxxxxxx                        00000000 zzzzyyyy yyxxxxxx
// **
// **
// ** BOM or Byte Order Mark:
// **     0xff 0xfe   little-endian utf-16 follows
// **     0xfe 0xff   big-endian utf-16 follows
// **
// */
// /*
// ** This lookup table is used to help decode the first byte of
// ** a multi-byte UTF8 character.
// */
static mut sqlite3Utf8Trans1: __SlateAlign16<[u8; 64]> = __SlateAlign16([
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
    ((0 as i32) as i8) as u8,
    ((1 as i32) as i8) as u8,
    ((2 as i32) as i8) as u8,
    ((3 as i32) as i8) as u8,
    ((4 as i32) as i8) as u8,
    ((5 as i32) as i8) as u8,
    ((6 as i32) as i8) as u8,
    ((7 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((1 as i32) as i8) as u8,
    ((2 as i32) as i8) as u8,
    ((3 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((1 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
]);

// /*
// ** Write a single UTF8 character whose value is v into the
// ** buffer starting at zOut.  zOut must be sized to hold at
// ** least four bytes.  Return the number of bytes needed
// ** to encode the new character.
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3AppendOneUtf8Character(mut zOut: *mut i8, mut v: u32) -> i32 {
    if v < ((128 as i32) as u32) {
        unsafe {
            *unsafe { zOut.offset((0 as i32) as isize) } =
                ((v & ((255 as i32) as u32)) as u8) as i8;
        }
        return 1 as i32;
    }
    if v < ((2048 as i32) as u32) {
        unsafe {
            *unsafe { zOut.offset((0 as i32) as isize) } = ((192 as i32)
                + ((((v >> (6 as i32) & ((31 as i32) as u32)) as u8) as u32) as i32))
                as i8;
        }
        unsafe {
            *unsafe { zOut.offset((1 as i32) as isize) } =
                ((128 as i32) + ((((v & ((63 as i32) as u32)) as u8) as u32) as i32)) as i8;
        }
        return 2 as i32;
    }
    if v < ((65536 as i32) as u32) {
        unsafe {
            *unsafe { zOut.offset((0 as i32) as isize) } = ((224 as i32)
                + ((((v >> (12 as i32) & ((15 as i32) as u32)) as u8) as u32) as i32))
                as i8;
        }
        unsafe {
            *unsafe { zOut.offset((1 as i32) as isize) } = ((128 as i32)
                + ((((v >> (6 as i32) & ((63 as i32) as u32)) as u8) as u32) as i32))
                as i8;
        }
        unsafe {
            *unsafe { zOut.offset((2 as i32) as isize) } =
                ((128 as i32) + ((((v & ((63 as i32) as u32)) as u8) as u32) as i32)) as i8;
        }
        return 3 as i32;
    }
    unsafe {
        *unsafe { zOut.offset((0 as i32) as isize) } = ((240 as i32)
            + ((((v >> (18 as i32) & ((7 as i32) as u32)) as u8) as u32) as i32))
            as i8;
    }
    unsafe {
        *unsafe { zOut.offset((1 as i32) as isize) } = ((128 as i32)
            + ((((v >> (12 as i32) & ((63 as i32) as u32)) as u8) as u32) as i32))
            as i8;
    }
    unsafe {
        *unsafe { zOut.offset((2 as i32) as isize) } = ((128 as i32)
            + ((((v >> (6 as i32) & ((63 as i32) as u32)) as u8) as u32) as i32))
            as i8;
    }
    unsafe {
        *unsafe { zOut.offset((3 as i32) as isize) } =
            ((128 as i32) + ((((v & ((63 as i32) as u32)) as u8) as u32) as i32)) as i8;
    }
    return 4 as i32;
}

// /*
// ** zIn is a UTF-16 encoded unicode string at least nByte bytes long.
// ** Return the number of bytes in the first nChar unicode characters
// ** in pZ.  nChar must be non-negative.  Surrogate pairs count as a single
// ** character.
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3Utf16ByteLen(
    mut zIn: *const (),
    mut nByte: i32,
    mut nChar: i32,
) -> i32 {
    let mut c: i32 = 0 as i32;
    let mut z: *const u8 = zIn as *const u8;
    let mut zEnd: *const u8 = unsafe { z.offset((nByte - (1 as i32)) as isize) };
    let mut n: i32 = 0 as i32;
    if (2 as i32) == (2 as i32) {
        let __v409: *const u8 = z;
        let __v410: *const u8 = unsafe { __v409.offset((1 as i32) as isize) };
        z = __v410;
    }
    '__slate_break_408: while n < nChar && z <= zEnd {
        c = ((unsafe { *unsafe { z.offset((0 as i32) as isize) } }) as u32) as i32;
        let __v411: *const u8 = z;
        let __v412: *const u8 = unsafe { __v411.offset((2 as i32) as isize) };
        z = __v412;
        if c >= (216 as i32)
            && c < (220 as i32)
            && z <= zEnd
            && (((unsafe { *unsafe { z.offset((0 as i32) as isize) } }) as u32) as i32)
                >= (220 as i32)
            && (((unsafe { *unsafe { z.offset((0 as i32) as isize) } }) as u32) as i32)
                < (224 as i32)
        {
            let __v413: *const u8 = z;
            let __v414: *const u8 = unsafe { __v413.offset((2 as i32) as isize) };
            z = __v414;
        }
        let __v415: i32 = n;
        let __v416: i32 = __v415 + (1 as i32);
        n = __v416;
    }
    return (((unsafe { z.offset_from((zIn as *const u8) as *const u8) }) as i64) as i32)
        - (((2 as i32) == (2 as i32)) as i32);
}

// /* SQLITE_OMIT_UTF16 */
// /*
// ** pZ is a UTF-8 encoded unicode string. If nByte is less than zero,
// ** return the number of unicode characters in pZ up to (but not including)
// ** the first 0x00 byte. If nByte is not less than zero, return the
// ** number of unicode characters in the first nByte of pZ (or up to
// ** the first 0x00, whichever comes first).
// */
// /* SQLITE_OMIT_UTF16 */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3Utf8CharLen(mut zIn: *const i8, mut nByte: i32) -> i32 {
    let mut r: i32 = 0 as i32;
    let mut z: *const u8 = zIn as *const u8;
    let mut zTerm: *const u8 = unsafe { std::mem::zeroed() };
    if nByte >= (0 as i32) {
        zTerm = unsafe { z.offset(nByte as isize) };
    } else {
        zTerm = -(1 as i32) as *const u8;
    }
    0 as i32;
    '__slate_break_406: while (((unsafe { *z }) as u32) as i32) != (0 as i32) && z < zTerm {
        let __v417: *const u8 = z;
        let __v418: *const u8 = unsafe { __v417.offset((1 as i32) as isize) };
        z = __v418;
        if (((unsafe { *__v417 }) as u32) as i32) >= (192 as i32) {
            '__slate_break_407: while (((unsafe { *z }) as u32) as i32) & (192 as i32)
                == (128 as i32)
            {
                let __v419: *const u8 = z;
                let __v420: *const u8 = unsafe { __v419.offset((1 as i32) as isize) };
                z = __v420;
            }
        }
        {}
        let __v421: i32 = r;
        let __v422: i32 = __v421 + (1 as i32);
        r = __v422;
    }
    return r;
}

// /*
// ** Translate a single UTF-8 character.  Return the unicode value.
// **
// ** During translation, assume that the byte that zTerm points
// ** is a 0x00.
// **
// ** Write a pointer to the next unread byte back into *pzNext.
// **
// ** Notes On Invalid UTF-8:
// **
// **  *  This routine never allows a 7-bit character (0x00 through 0x7f) to
// **     be encoded as a multi-byte character.  Any multi-byte character that
// **     attempts to encode a value between 0x00 and 0x7f is rendered as 0xfffd.
// **
// **  *  This routine never allows a UTF16 surrogate value to be encoded.
// **     If a multi-byte character attempts to encode a value between
// **     0xd800 and 0xe000 then it is rendered as 0xfffd.
// **
// **  *  Bytes in the range of 0x80 through 0xbf which occur as the first
// **     byte of a character are interpreted as single-byte characters
// **     and rendered as themselves even though they are technically
// **     invalid characters.
// **
// **  *  This routine accepts over-length UTF8 encodings
// **     for unicode values 0x80 and greater.  It does not change over-length
// **     encodings to 0xfffd as some systems recommend.
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3Utf8Read(mut pz: *mut *const u8) -> u32 {
    let mut c: u32 = 0 as u32;
    // /* Same as READ_UTF8() above but without the zTerm parameter.
    //   ** For this routine, we assume the UTF8 string is always zero-terminated.
    //   */
    let __v423: *mut *const u8 = pz;
    let __v424: *const u8 = unsafe { *__v423 };
    let __v425: *const u8 = unsafe { __v424.offset((1 as i32) as isize) };
    unsafe {
        *__v423 = __v425;
    }
    c = (unsafe { *__v424 }) as u32;
    if c >= ((192 as i32) as u32) {
        c = (unsafe {
            *unsafe {
                unsafe { std::ptr::addr_of!(sqlite3Utf8Trans1.0) as *const u8 }
                    .offset(c.wrapping_sub((192 as i32) as u32) as isize)
            }
        }) as u32;
        '__slate_break_397: while (((unsafe { *unsafe { *pz } }) as u32) as i32) & (192 as i32)
            == (128 as i32)
        {
            let __v426: *mut *const u8 = pz;
            let __v427: *const u8 = unsafe { *__v426 };
            let __v428: *const u8 = unsafe { __v427.offset((1 as i32) as isize) };
            unsafe {
                *__v426 = __v428;
            }
            c = (c << (6 as i32))
                .wrapping_add(((63 as i32) & (((unsafe { *__v427 }) as u32) as i32)) as u32);
        }
        if c < ((128 as i32) as u32)
            || c & (4294965248 as u32) == ((55296 as i32) as u32)
            || c & (4294967294 as u32) == ((65534 as i32) as u32)
        {
            c = (65533 as i32) as u32;
        }
    }
    return c;
}

// /* Pointer to string from which to read char */
// /*
// ** Read a single UTF8 character out of buffer z[], but reading no
// ** more than n characters from the buffer.  z[] is not zero-terminated.
// **
// ** Return the number of bytes used to construct the character.
// **
// ** Invalid UTF8 might generate a strange result.  No effort is made
// ** to detect invalid UTF8.
// **
// ** At most 4 bytes will be read out of z[].  The return value will always
// ** be between 1 and 4.
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3Utf8ReadLimited(
    mut z: *const u8,
    mut n: i32,
    mut piOut: *mut u32,
) -> i32 {
    let mut c: u32 = 0 as u32;
    let mut i: i32 = 1 as i32;
    0 as i32;
    c = (unsafe { *unsafe { z.offset((0 as i32) as isize) } }) as u32;
    if c >= ((192 as i32) as u32) {
        c = (unsafe {
            *unsafe {
                unsafe { std::ptr::addr_of!(sqlite3Utf8Trans1.0) as *const u8 }
                    .offset(c.wrapping_sub((192 as i32) as u32) as isize)
            }
        }) as u32;
        if n > (4 as i32) {
            n = 4 as i32;
        }
        '__slate_break_398: while i < n
            && (((unsafe { *unsafe { z.offset(i as isize) } }) as u32) as i32) & (192 as i32)
                == (128 as i32)
        {
            c = (c << (6 as i32)).wrapping_add(
                ((63 as i32) & (((unsafe { *unsafe { z.offset(i as isize) } }) as u32) as i32))
                    as u32,
            );
            let __v429: i32 = i;
            let __v430: i32 = __v429 + (1 as i32);
            i = __v430;
        }
    }
    unsafe {
        *piOut = c;
    }
    return i;
}

// /* This test function is not currently used by the automated test-suite.
// ** Hence it is only available in debug builds.
// */
// /*
// ** Convert a UTF-16 string in the native encoding into a UTF-8 string.
// ** Memory to hold the UTF-8 string is obtained from sqlite3_malloc and must
// ** be freed by the calling function.
// **
// ** NULL is returned if there is an allocation error.
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3Utf16to8(
    mut db: *mut sqlite3,
    mut z: *const (),
    mut nByte: i32,
    mut enc: u8,
) -> *mut i8 {
    let mut m: sqlite3_value = unsafe { std::mem::zeroed() };
    unsafe { memset(std::ptr::addr_of_mut!(m) as *mut (), 0 as i32, 56 as u64) };
    m.db = db;
    unsafe {
        sqlite3VdbeMemSetStr(
            std::ptr::addr_of_mut!(m),
            z as *const i8,
            nByte as i64,
            enc,
            None,
        )
    };
    unsafe { sqlite3VdbeChangeEncoding(std::ptr::addr_of_mut!(m), 1 as i32) };
    if (unsafe { (*db).mallocFailed }) != (0 as u8) {
        unsafe { sqlite3VdbeMemRelease(std::ptr::addr_of_mut!(m)) };
        m.z = std::ptr::null_mut::<i8>();
    }
    0 as i32;
    0 as i32;
    0 as i32;
    return m.z;
}

// /*
// ** If the TRANSLATE_TRACE macro is defined, the value of each Mem is
// ** printed on stderr on the way into and out of sqlite3VdbeMemTranslate().
// */
// /* #define TRANSLATE_TRACE 1 */
// /*
// ** This routine transforms the internal text encoding used by pMem to
// ** desiredEnc. It is an error if the string is already of the desired
// ** encoding, or if *pMem does not contain a string value.
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3VdbeMemTranslate(
    mut pMem: *mut sqlite3_value,
    mut desiredEnc: u8,
) -> i32 {
    let mut __slate_storage_471: std::mem::MaybeUninit<*mut u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_471: *mut *mut u8 =
        std::ptr::addr_of_mut!(__slate_storage_471) as *mut *mut u8;
    let mut __slate_storage_470: std::mem::MaybeUninit<*mut u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_470: *mut *mut u8 =
        std::ptr::addr_of_mut!(__slate_storage_470) as *mut *mut u8;
    let mut __slate_storage_445: std::mem::MaybeUninit<*mut u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_445: *mut *mut u8 =
        std::ptr::addr_of_mut!(__slate_storage_445) as *mut *mut u8;
    let mut __slate_storage_444: std::mem::MaybeUninit<*mut u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_444: *mut *mut u8 =
        std::ptr::addr_of_mut!(__slate_storage_444) as *mut *mut u8;
    let mut __slate_storage_443: std::mem::MaybeUninit<*mut u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_443: *mut *mut u8 =
        std::ptr::addr_of_mut!(__slate_storage_443) as *mut *mut u8;
    let mut __slate_storage_442: std::mem::MaybeUninit<*mut u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_442: *mut *mut u8 =
        std::ptr::addr_of_mut!(__slate_storage_442) as *mut *mut u8;
    let mut __slate_storage_453: std::mem::MaybeUninit<*mut u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_453: *mut *mut u8 =
        std::ptr::addr_of_mut!(__slate_storage_453) as *mut *mut u8;
    let mut __slate_storage_452: std::mem::MaybeUninit<*mut u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_452: *mut *mut u8 =
        std::ptr::addr_of_mut!(__slate_storage_452) as *mut *mut u8;
    let mut __slate_storage_451: std::mem::MaybeUninit<*mut u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_451: *mut *mut u8 =
        std::ptr::addr_of_mut!(__slate_storage_451) as *mut *mut u8;
    let mut __slate_storage_450: std::mem::MaybeUninit<*mut u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_450: *mut *mut u8 =
        std::ptr::addr_of_mut!(__slate_storage_450) as *mut *mut u8;
    let mut __slate_storage_449: std::mem::MaybeUninit<*mut u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_449: *mut *mut u8 =
        std::ptr::addr_of_mut!(__slate_storage_449) as *mut *mut u8;
    let mut __slate_storage_448: std::mem::MaybeUninit<*mut u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_448: *mut *mut u8 =
        std::ptr::addr_of_mut!(__slate_storage_448) as *mut *mut u8;
    let mut __slate_storage_447: std::mem::MaybeUninit<*mut u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_447: *mut *mut u8 =
        std::ptr::addr_of_mut!(__slate_storage_447) as *mut *mut u8;
    let mut __slate_storage_446: std::mem::MaybeUninit<*mut u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_446: *mut *mut u8 =
        std::ptr::addr_of_mut!(__slate_storage_446) as *mut *mut u8;
    let mut __slate_storage_441: std::mem::MaybeUninit<*mut u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_441: *mut *mut u8 =
        std::ptr::addr_of_mut!(__slate_storage_441) as *mut *mut u8;
    let mut __slate_storage_440: std::mem::MaybeUninit<*mut u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_440: *mut *mut u8 =
        std::ptr::addr_of_mut!(__slate_storage_440) as *mut *mut u8;
    let mut __slate_storage_439: std::mem::MaybeUninit<*mut u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_439: *mut *mut u8 =
        std::ptr::addr_of_mut!(__slate_storage_439) as *mut *mut u8;
    let mut __slate_storage_438: std::mem::MaybeUninit<*mut u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_438: *mut *mut u8 =
        std::ptr::addr_of_mut!(__slate_storage_438) as *mut *mut u8;
    let mut __slate_storage_461: std::mem::MaybeUninit<*mut u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_461: *mut *mut u8 =
        std::ptr::addr_of_mut!(__slate_storage_461) as *mut *mut u8;
    let mut __slate_storage_460: std::mem::MaybeUninit<*mut u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_460: *mut *mut u8 =
        std::ptr::addr_of_mut!(__slate_storage_460) as *mut *mut u8;
    let mut __slate_storage_459: std::mem::MaybeUninit<*mut u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_459: *mut *mut u8 =
        std::ptr::addr_of_mut!(__slate_storage_459) as *mut *mut u8;
    let mut __slate_storage_458: std::mem::MaybeUninit<*mut u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_458: *mut *mut u8 =
        std::ptr::addr_of_mut!(__slate_storage_458) as *mut *mut u8;
    let mut __slate_storage_469: std::mem::MaybeUninit<*mut u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_469: *mut *mut u8 =
        std::ptr::addr_of_mut!(__slate_storage_469) as *mut *mut u8;
    let mut __slate_storage_468: std::mem::MaybeUninit<*mut u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_468: *mut *mut u8 =
        std::ptr::addr_of_mut!(__slate_storage_468) as *mut *mut u8;
    let mut __slate_storage_467: std::mem::MaybeUninit<*mut u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_467: *mut *mut u8 =
        std::ptr::addr_of_mut!(__slate_storage_467) as *mut *mut u8;
    let mut __slate_storage_466: std::mem::MaybeUninit<*mut u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_466: *mut *mut u8 =
        std::ptr::addr_of_mut!(__slate_storage_466) as *mut *mut u8;
    let mut __slate_storage_465: std::mem::MaybeUninit<*mut u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_465: *mut *mut u8 =
        std::ptr::addr_of_mut!(__slate_storage_465) as *mut *mut u8;
    let mut __slate_storage_464: std::mem::MaybeUninit<*mut u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_464: *mut *mut u8 =
        std::ptr::addr_of_mut!(__slate_storage_464) as *mut *mut u8;
    let mut __slate_storage_463: std::mem::MaybeUninit<*mut u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_463: *mut *mut u8 =
        std::ptr::addr_of_mut!(__slate_storage_463) as *mut *mut u8;
    let mut __slate_storage_462: std::mem::MaybeUninit<*mut u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_462: *mut *mut u8 =
        std::ptr::addr_of_mut!(__slate_storage_462) as *mut *mut u8;
    let mut __slate_storage_457: std::mem::MaybeUninit<*mut u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_457: *mut *mut u8 =
        std::ptr::addr_of_mut!(__slate_storage_457) as *mut *mut u8;
    let mut __slate_storage_456: std::mem::MaybeUninit<*mut u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_456: *mut *mut u8 =
        std::ptr::addr_of_mut!(__slate_storage_456) as *mut *mut u8;
    let mut __slate_storage_455: std::mem::MaybeUninit<*mut u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_455: *mut *mut u8 =
        std::ptr::addr_of_mut!(__slate_storage_455) as *mut *mut u8;
    let mut __slate_storage_454: std::mem::MaybeUninit<*mut u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_454: *mut *mut u8 =
        std::ptr::addr_of_mut!(__slate_storage_454) as *mut *mut u8;
    let mut __slate_storage_485: std::mem::MaybeUninit<*mut u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_485: *mut *mut u8 =
        std::ptr::addr_of_mut!(__slate_storage_485) as *mut *mut u8;
    let mut __slate_storage_484: std::mem::MaybeUninit<*mut u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_484: *mut *mut u8 =
        std::ptr::addr_of_mut!(__slate_storage_484) as *mut *mut u8;
    let mut __slate_storage_489: std::mem::MaybeUninit<*mut u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_489: *mut *mut u8 =
        std::ptr::addr_of_mut!(__slate_storage_489) as *mut *mut u8;
    let mut __slate_storage_488: std::mem::MaybeUninit<*mut u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_488: *mut *mut u8 =
        std::ptr::addr_of_mut!(__slate_storage_488) as *mut *mut u8;
    let mut __slate_storage_487: std::mem::MaybeUninit<*mut u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_487: *mut *mut u8 =
        std::ptr::addr_of_mut!(__slate_storage_487) as *mut *mut u8;
    let mut __slate_storage_486: std::mem::MaybeUninit<*mut u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_486: *mut *mut u8 =
        std::ptr::addr_of_mut!(__slate_storage_486) as *mut *mut u8;
    let mut __slate_storage_495: std::mem::MaybeUninit<*mut u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_495: *mut *mut u8 =
        std::ptr::addr_of_mut!(__slate_storage_495) as *mut *mut u8;
    let mut __slate_storage_494: std::mem::MaybeUninit<*mut u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_494: *mut *mut u8 =
        std::ptr::addr_of_mut!(__slate_storage_494) as *mut *mut u8;
    let mut __slate_storage_493: std::mem::MaybeUninit<*mut u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_493: *mut *mut u8 =
        std::ptr::addr_of_mut!(__slate_storage_493) as *mut *mut u8;
    let mut __slate_storage_492: std::mem::MaybeUninit<*mut u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_492: *mut *mut u8 =
        std::ptr::addr_of_mut!(__slate_storage_492) as *mut *mut u8;
    let mut __slate_storage_491: std::mem::MaybeUninit<*mut u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_491: *mut *mut u8 =
        std::ptr::addr_of_mut!(__slate_storage_491) as *mut *mut u8;
    let mut __slate_storage_490: std::mem::MaybeUninit<*mut u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_490: *mut *mut u8 =
        std::ptr::addr_of_mut!(__slate_storage_490) as *mut *mut u8;
    let mut __slate_storage_503: std::mem::MaybeUninit<*mut u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_503: *mut *mut u8 =
        std::ptr::addr_of_mut!(__slate_storage_503) as *mut *mut u8;
    let mut __slate_storage_502: std::mem::MaybeUninit<*mut u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_502: *mut *mut u8 =
        std::ptr::addr_of_mut!(__slate_storage_502) as *mut *mut u8;
    let mut __slate_storage_501: std::mem::MaybeUninit<*mut u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_501: *mut *mut u8 =
        std::ptr::addr_of_mut!(__slate_storage_501) as *mut *mut u8;
    let mut __slate_storage_500: std::mem::MaybeUninit<*mut u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_500: *mut *mut u8 =
        std::ptr::addr_of_mut!(__slate_storage_500) as *mut *mut u8;
    let mut __slate_storage_499: std::mem::MaybeUninit<*mut u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_499: *mut *mut u8 =
        std::ptr::addr_of_mut!(__slate_storage_499) as *mut *mut u8;
    let mut __slate_storage_498: std::mem::MaybeUninit<*mut u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_498: *mut *mut u8 =
        std::ptr::addr_of_mut!(__slate_storage_498) as *mut *mut u8;
    let mut __slate_storage_497: std::mem::MaybeUninit<*mut u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_497: *mut *mut u8 =
        std::ptr::addr_of_mut!(__slate_storage_497) as *mut *mut u8;
    let mut __slate_storage_496: std::mem::MaybeUninit<*mut u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_496: *mut *mut u8 =
        std::ptr::addr_of_mut!(__slate_storage_496) as *mut *mut u8;
    let mut __slate_storage_483: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_483: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_483) as *mut i32;
    let mut __slate_storage_482: std::mem::MaybeUninit<*mut u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_482: *mut *mut u8 =
        std::ptr::addr_of_mut!(__slate_storage_482) as *mut *mut u8;
    let mut __slate_storage_481: std::mem::MaybeUninit<*mut u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_481: *mut *mut u8 =
        std::ptr::addr_of_mut!(__slate_storage_481) as *mut *mut u8;
    let mut __slate_storage_480: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_480: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_480) as *mut i32;
    let mut __slate_storage_479: std::mem::MaybeUninit<*mut u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_479: *mut *mut u8 =
        std::ptr::addr_of_mut!(__slate_storage_479) as *mut *mut u8;
    let mut __slate_storage_478: std::mem::MaybeUninit<*mut u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_478: *mut *mut u8 =
        std::ptr::addr_of_mut!(__slate_storage_478) as *mut *mut u8;
    let mut __slate_storage_336: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_336: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_336) as *mut i32;
    let mut __slate_storage_477: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_477: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_477) as *mut u32;
    let mut __slate_storage_476: std::mem::MaybeUninit<*mut u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_476: *mut *mut u8 =
        std::ptr::addr_of_mut!(__slate_storage_476) as *mut *mut u8;
    let mut __slate_storage_475: std::mem::MaybeUninit<*mut u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_475: *mut *mut u8 =
        std::ptr::addr_of_mut!(__slate_storage_475) as *mut *mut u8;
    let mut __slate_storage_474: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_474: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_474) as *mut u32;
    let mut __slate_storage_473: std::mem::MaybeUninit<*mut u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_473: *mut *mut u8 =
        std::ptr::addr_of_mut!(__slate_storage_473) as *mut *mut u8;
    let mut __slate_storage_472: std::mem::MaybeUninit<*mut u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_472: *mut *mut u8 =
        std::ptr::addr_of_mut!(__slate_storage_472) as *mut *mut u8;
    let mut __slate_storage_517: std::mem::MaybeUninit<*mut u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_517: *mut *mut u8 =
        std::ptr::addr_of_mut!(__slate_storage_517) as *mut *mut u8;
    let mut __slate_storage_516: std::mem::MaybeUninit<*mut u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_516: *mut *mut u8 =
        std::ptr::addr_of_mut!(__slate_storage_516) as *mut *mut u8;
    let mut __slate_storage_521: std::mem::MaybeUninit<*mut u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_521: *mut *mut u8 =
        std::ptr::addr_of_mut!(__slate_storage_521) as *mut *mut u8;
    let mut __slate_storage_520: std::mem::MaybeUninit<*mut u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_520: *mut *mut u8 =
        std::ptr::addr_of_mut!(__slate_storage_520) as *mut *mut u8;
    let mut __slate_storage_519: std::mem::MaybeUninit<*mut u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_519: *mut *mut u8 =
        std::ptr::addr_of_mut!(__slate_storage_519) as *mut *mut u8;
    let mut __slate_storage_518: std::mem::MaybeUninit<*mut u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_518: *mut *mut u8 =
        std::ptr::addr_of_mut!(__slate_storage_518) as *mut *mut u8;
    let mut __slate_storage_527: std::mem::MaybeUninit<*mut u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_527: *mut *mut u8 =
        std::ptr::addr_of_mut!(__slate_storage_527) as *mut *mut u8;
    let mut __slate_storage_526: std::mem::MaybeUninit<*mut u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_526: *mut *mut u8 =
        std::ptr::addr_of_mut!(__slate_storage_526) as *mut *mut u8;
    let mut __slate_storage_525: std::mem::MaybeUninit<*mut u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_525: *mut *mut u8 =
        std::ptr::addr_of_mut!(__slate_storage_525) as *mut *mut u8;
    let mut __slate_storage_524: std::mem::MaybeUninit<*mut u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_524: *mut *mut u8 =
        std::ptr::addr_of_mut!(__slate_storage_524) as *mut *mut u8;
    let mut __slate_storage_523: std::mem::MaybeUninit<*mut u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_523: *mut *mut u8 =
        std::ptr::addr_of_mut!(__slate_storage_523) as *mut *mut u8;
    let mut __slate_storage_522: std::mem::MaybeUninit<*mut u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_522: *mut *mut u8 =
        std::ptr::addr_of_mut!(__slate_storage_522) as *mut *mut u8;
    let mut __slate_storage_535: std::mem::MaybeUninit<*mut u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_535: *mut *mut u8 =
        std::ptr::addr_of_mut!(__slate_storage_535) as *mut *mut u8;
    let mut __slate_storage_534: std::mem::MaybeUninit<*mut u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_534: *mut *mut u8 =
        std::ptr::addr_of_mut!(__slate_storage_534) as *mut *mut u8;
    let mut __slate_storage_533: std::mem::MaybeUninit<*mut u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_533: *mut *mut u8 =
        std::ptr::addr_of_mut!(__slate_storage_533) as *mut *mut u8;
    let mut __slate_storage_532: std::mem::MaybeUninit<*mut u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_532: *mut *mut u8 =
        std::ptr::addr_of_mut!(__slate_storage_532) as *mut *mut u8;
    let mut __slate_storage_531: std::mem::MaybeUninit<*mut u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_531: *mut *mut u8 =
        std::ptr::addr_of_mut!(__slate_storage_531) as *mut *mut u8;
    let mut __slate_storage_530: std::mem::MaybeUninit<*mut u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_530: *mut *mut u8 =
        std::ptr::addr_of_mut!(__slate_storage_530) as *mut *mut u8;
    let mut __slate_storage_529: std::mem::MaybeUninit<*mut u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_529: *mut *mut u8 =
        std::ptr::addr_of_mut!(__slate_storage_529) as *mut *mut u8;
    let mut __slate_storage_528: std::mem::MaybeUninit<*mut u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_528: *mut *mut u8 =
        std::ptr::addr_of_mut!(__slate_storage_528) as *mut *mut u8;
    let mut __slate_storage_515: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_515: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_515) as *mut i32;
    let mut __slate_storage_514: std::mem::MaybeUninit<*mut u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_514: *mut *mut u8 =
        std::ptr::addr_of_mut!(__slate_storage_514) as *mut *mut u8;
    let mut __slate_storage_513: std::mem::MaybeUninit<*mut u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_513: *mut *mut u8 =
        std::ptr::addr_of_mut!(__slate_storage_513) as *mut *mut u8;
    let mut __slate_storage_512: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_512: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_512) as *mut i32;
    let mut __slate_storage_511: std::mem::MaybeUninit<*mut u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_511: *mut *mut u8 =
        std::ptr::addr_of_mut!(__slate_storage_511) as *mut *mut u8;
    let mut __slate_storage_510: std::mem::MaybeUninit<*mut u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_510: *mut *mut u8 =
        std::ptr::addr_of_mut!(__slate_storage_510) as *mut *mut u8;
    let mut __slate_storage_337: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_337: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_337) as *mut i32;
    let mut __slate_storage_509: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_509: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_509) as *mut u32;
    let mut __slate_storage_508: std::mem::MaybeUninit<*mut u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_508: *mut *mut u8 =
        std::ptr::addr_of_mut!(__slate_storage_508) as *mut *mut u8;
    let mut __slate_storage_507: std::mem::MaybeUninit<*mut u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_507: *mut *mut u8 =
        std::ptr::addr_of_mut!(__slate_storage_507) as *mut *mut u8;
    let mut __slate_storage_506: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_506: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_506) as *mut u32;
    let mut __slate_storage_505: std::mem::MaybeUninit<*mut u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_505: *mut *mut u8 =
        std::ptr::addr_of_mut!(__slate_storage_505) as *mut *mut u8;
    let mut __slate_storage_504: std::mem::MaybeUninit<*mut u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_504: *mut *mut u8 =
        std::ptr::addr_of_mut!(__slate_storage_504) as *mut *mut u8;
    let mut __slate_storage_437: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_437: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_437) as *mut i32;
    let mut __slate_storage_436: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_436: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_436) as *mut i32;
    let mut __slate_storage_435: std::mem::MaybeUninit<*mut sqlite3_value> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_435: *mut *mut sqlite3_value =
        std::ptr::addr_of_mut!(__slate_storage_435) as *mut *mut sqlite3_value;
    let mut __slate_storage_434: std::mem::MaybeUninit<*mut u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_434: *mut *mut u8 =
        std::ptr::addr_of_mut!(__slate_storage_434) as *mut *mut u8;
    let mut __slate_storage_433: std::mem::MaybeUninit<*mut u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_433: *mut *mut u8 =
        std::ptr::addr_of_mut!(__slate_storage_433) as *mut *mut u8;
    let mut __slate_storage_432: std::mem::MaybeUninit<*mut u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_432: *mut *mut u8 =
        std::ptr::addr_of_mut!(__slate_storage_432) as *mut *mut u8;
    let mut __slate_storage_431: std::mem::MaybeUninit<*mut u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_431: *mut *mut u8 =
        std::ptr::addr_of_mut!(__slate_storage_431) as *mut *mut u8;
    let mut __slate_storage_335: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_335: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_335) as *mut i32;
    let mut __slate_storage_334: std::mem::MaybeUninit<u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_334: *mut u8 = std::ptr::addr_of_mut!(__slate_storage_334) as *mut u8;
    let mut __slate_storage_333: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_333: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_333) as *mut u32;
    let mut __slate_storage_332: std::mem::MaybeUninit<*mut u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_332: *mut *mut u8 =
        std::ptr::addr_of_mut!(__slate_storage_332) as *mut *mut u8;
    let mut __slate_storage_331: std::mem::MaybeUninit<*mut u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_331: *mut *mut u8 =
        std::ptr::addr_of_mut!(__slate_storage_331) as *mut *mut u8;
    let mut __slate_storage_330: std::mem::MaybeUninit<*mut u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_330: *mut *mut u8 =
        std::ptr::addr_of_mut!(__slate_storage_330) as *mut *mut u8;
    let mut __slate_storage_329: std::mem::MaybeUninit<*mut u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_329: *mut *mut u8 =
        std::ptr::addr_of_mut!(__slate_storage_329) as *mut *mut u8;
    let mut __slate_storage_328: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_328: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_328) as *mut i64;
    unsafe {
        '__join_0: {
            // /* Maximum length of output string in bytes */
            // /* Output buffer */
            // /* Input iterator */
            // /* End of input */
            // /* Output iterator */
            0 as i32;
            0 as i32;
            0 as i32;
            0 as i32;
            0 as i32;
            // /* If the translation is between UTF-16 little and big endian, then
            //   ** all that is required is to swap the byte order. This case is handled
            //   ** differently from the others.
            //   */
            if (((unsafe { (*pMem).enc }) as u32) as i32) != (1 as i32)
                && ((desiredEnc as u32) as i32) != (1 as i32)
            {
                *__slate_slot_335 = unsafe { sqlite3VdbeMemMakeWriteable(pMem) };
                if *__slate_slot_335 != (0 as i32) {
                    0 as i32;
                    return 7 as i32;
                } else {
                    *__slate_slot_330 = (unsafe { (*pMem).z }) as *mut u8;
                    *__slate_slot_331 = unsafe {
                        (*__slate_slot_330).offset(((unsafe { (*pMem).n }) & !(1 as i32)) as isize)
                    };
                    loop {
                        if *__slate_slot_330 < *__slate_slot_331 {
                            *__slate_slot_334 = unsafe { *(*__slate_slot_330) };
                            unsafe {
                                *(*__slate_slot_330) = unsafe {
                                    *unsafe { (*__slate_slot_330).offset((1 as i32) as isize) }
                                };
                            }
                            std::ptr::write(__slate_slot_431, *__slate_slot_330);
                            std::ptr::write(__slate_slot_432, unsafe {
                                (*__slate_slot_431).offset((1 as i32) as isize)
                            });
                            *__slate_slot_330 = *__slate_slot_432;
                            std::ptr::write(__slate_slot_433, *__slate_slot_330);
                            std::ptr::write(__slate_slot_434, unsafe {
                                (*__slate_slot_433).offset((1 as i32) as isize)
                            });
                            *__slate_slot_330 = *__slate_slot_434;
                            unsafe {
                                *(*__slate_slot_433) = *__slate_slot_334;
                            }
                        } else {
                            break;
                        }
                    }
                    unsafe {
                        (*pMem).enc = desiredEnc;
                    }
                }
            } else {
                // /* Set len to the maximum number of bytes required in the output buffer. */
                if ((desiredEnc as u32) as i32) == (1 as i32) {
                    // /* When converting from UTF-16, the maximum growth results from
                    //     ** translating a 2-byte character to a 4-byte UTF-8 character.
                    //     ** A single byte is required for the output string
                    //     ** nul-terminator.
                    //     */
                    std::ptr::write(__slate_slot_435, pMem);
                    std::ptr::write(__slate_slot_436, unsafe { (*(*__slate_slot_435)).n });
                    std::ptr::write(__slate_slot_437, *__slate_slot_436 & !(1 as i32));
                    unsafe {
                        (*(*__slate_slot_435)).n = *__slate_slot_437;
                    }
                    *__slate_slot_328 =
                        ((2 as i32) as i64) * ((unsafe { (*pMem).n }) as i64) + ((1 as i32) as i64);
                } else {
                    // /* When converting from UTF-8 to UTF-16 the maximum growth is caused
                    //     ** when a 1-byte UTF-8 character is translated into a 2-byte UTF-16
                    //     ** character. Two bytes are required in the output buffer for the
                    //     ** nul-terminator.
                    //     */
                    *__slate_slot_328 =
                        ((2 as i32) as i64) * ((unsafe { (*pMem).n }) as i64) + ((2 as i32) as i64);
                }
                // /* Set zIn to point at the start of the input buffer and zTerm to point 1
                //   ** byte past the end.
                //   **
                //   ** Variable zOut is set to point at the output buffer, space obtained
                //   ** from sqlite3_malloc().
                //   */
                *__slate_slot_330 = (unsafe { (*pMem).z }) as *mut u8;
                *__slate_slot_331 =
                    unsafe { (*__slate_slot_330).offset((unsafe { (*pMem).n }) as isize) };
                *__slate_slot_329 = (unsafe {
                    sqlite3DbMallocRaw(unsafe { (*pMem).db }, *__slate_slot_328 as u64)
                }) as *mut u8;
                if !(*__slate_slot_329 != std::ptr::null_mut::<u8>()) {
                    return 7 as i32;
                } else {
                    *__slate_slot_332 = *__slate_slot_329;
                    if (((unsafe { (*pMem).enc }) as u32) as i32) == (1 as i32) {
                        '__join_2: {
                            if ((desiredEnc as u32) as i32) == (2 as i32) {
                                // /* UTF-8 -> UTF-16 Little-endian */
                                loop {
                                    if *__slate_slot_330 < *__slate_slot_331 {
                                        std::ptr::write(__slate_slot_438, *__slate_slot_330);
                                        std::ptr::write(__slate_slot_439, unsafe {
                                            (*__slate_slot_438).offset((1 as i32) as isize)
                                        });
                                        *__slate_slot_330 = *__slate_slot_439;
                                        *__slate_slot_333 =
                                            (unsafe { *(*__slate_slot_438) }) as u32;
                                        if *__slate_slot_333 >= ((192 as i32) as u32) {
                                            *__slate_slot_333 = (unsafe {
                                                *unsafe {
                                                    unsafe {
                                                        std::ptr::addr_of!(sqlite3Utf8Trans1.0)
                                                            as *const u8
                                                    }
                                                    .offset(
                                                        (*__slate_slot_333)
                                                            .wrapping_sub((192 as i32) as u32)
                                                            as isize,
                                                    )
                                                }
                                            })
                                                as u32;
                                            loop {
                                                if *__slate_slot_330 < *__slate_slot_331
                                                    && (((unsafe { *(*__slate_slot_330) }) as u32)
                                                        as i32)
                                                        & (192 as i32)
                                                        == (128 as i32)
                                                {
                                                    std::ptr::write(
                                                        __slate_slot_440,
                                                        *__slate_slot_330,
                                                    );
                                                    std::ptr::write(__slate_slot_441, unsafe {
                                                        (*__slate_slot_440)
                                                            .offset((1 as i32) as isize)
                                                    });
                                                    *__slate_slot_330 = *__slate_slot_441;
                                                    *__slate_slot_333 = (*__slate_slot_333
                                                        << (6 as i32))
                                                        .wrapping_add(
                                                            ((63 as i32)
                                                                & (((unsafe {
                                                                    *(*__slate_slot_440)
                                                                })
                                                                    as u32)
                                                                    as i32))
                                                                as u32,
                                                        );
                                                } else {
                                                    break;
                                                }
                                            }
                                            if *__slate_slot_333 < ((128 as i32) as u32)
                                                || *__slate_slot_333 & (4294965248 as u32)
                                                    == ((55296 as i32) as u32)
                                                || *__slate_slot_333 & (4294967294 as u32)
                                                    == ((65534 as i32) as u32)
                                            {
                                                *__slate_slot_333 = (65533 as i32) as u32;
                                            }
                                        }
                                        {}
                                        if *__slate_slot_333 <= ((65535 as i32) as u32) {
                                            std::ptr::write(__slate_slot_442, *__slate_slot_332);
                                            std::ptr::write(__slate_slot_443, unsafe {
                                                (*__slate_slot_442).offset((1 as i32) as isize)
                                            });
                                            *__slate_slot_332 = *__slate_slot_443;
                                            unsafe {
                                                *(*__slate_slot_442) = (*__slate_slot_333
                                                    & ((255 as i32) as u32))
                                                    as u8;
                                            }
                                            std::ptr::write(__slate_slot_444, *__slate_slot_332);
                                            std::ptr::write(__slate_slot_445, unsafe {
                                                (*__slate_slot_444).offset((1 as i32) as isize)
                                            });
                                            *__slate_slot_332 = *__slate_slot_445;
                                            unsafe {
                                                *(*__slate_slot_444) = (*__slate_slot_333
                                                    >> (8 as i32)
                                                    & ((255 as i32) as u32))
                                                    as u8;
                                            }
                                        } else {
                                            std::ptr::write(__slate_slot_446, *__slate_slot_332);
                                            std::ptr::write(__slate_slot_447, unsafe {
                                                (*__slate_slot_446).offset((1 as i32) as isize)
                                            });
                                            *__slate_slot_332 = *__slate_slot_447;
                                            unsafe {
                                                *(*__slate_slot_446) = (*__slate_slot_333
                                                    >> (10 as i32)
                                                    & ((63 as i32) as u32))
                                                    .wrapping_add(
                                                        (*__slate_slot_333)
                                                            .wrapping_sub((65536 as i32) as u32)
                                                            >> (10 as i32)
                                                            & ((192 as i32) as u32),
                                                    )
                                                    as u8;
                                            }
                                            std::ptr::write(__slate_slot_448, *__slate_slot_332);
                                            std::ptr::write(__slate_slot_449, unsafe {
                                                (*__slate_slot_448).offset((1 as i32) as isize)
                                            });
                                            *__slate_slot_332 = *__slate_slot_449;
                                            unsafe {
                                                *(*__slate_slot_448) = ((216 as i32) as u32)
                                                    .wrapping_add(
                                                        (*__slate_slot_333)
                                                            .wrapping_sub((65536 as i32) as u32)
                                                            >> (18 as i32)
                                                            & ((3 as i32) as u32),
                                                    )
                                                    as u8;
                                            }
                                            std::ptr::write(__slate_slot_450, *__slate_slot_332);
                                            std::ptr::write(__slate_slot_451, unsafe {
                                                (*__slate_slot_450).offset((1 as i32) as isize)
                                            });
                                            *__slate_slot_332 = *__slate_slot_451;
                                            unsafe {
                                                *(*__slate_slot_450) = (*__slate_slot_333
                                                    & ((255 as i32) as u32))
                                                    as u8;
                                            }
                                            std::ptr::write(__slate_slot_452, *__slate_slot_332);
                                            std::ptr::write(__slate_slot_453, unsafe {
                                                (*__slate_slot_452).offset((1 as i32) as isize)
                                            });
                                            *__slate_slot_332 = *__slate_slot_453;
                                            unsafe {
                                                *(*__slate_slot_452) = ((220 as i32) as u32)
                                                    .wrapping_add(
                                                        *__slate_slot_333 >> (8 as i32)
                                                            & ((3 as i32) as u32),
                                                    )
                                                    as u8;
                                            }
                                        }
                                        {}
                                    } else {
                                        break '__join_2;
                                    }
                                }
                            } else {
                                0 as i32;
                                // /* UTF-8 -> UTF-16 Big-endian */
                                loop {
                                    if *__slate_slot_330 < *__slate_slot_331 {
                                        std::ptr::write(__slate_slot_454, *__slate_slot_330);
                                        std::ptr::write(__slate_slot_455, unsafe {
                                            (*__slate_slot_454).offset((1 as i32) as isize)
                                        });
                                        *__slate_slot_330 = *__slate_slot_455;
                                        *__slate_slot_333 =
                                            (unsafe { *(*__slate_slot_454) }) as u32;
                                        if *__slate_slot_333 >= ((192 as i32) as u32) {
                                            *__slate_slot_333 = (unsafe {
                                                *unsafe {
                                                    unsafe {
                                                        std::ptr::addr_of!(sqlite3Utf8Trans1.0)
                                                            as *const u8
                                                    }
                                                    .offset(
                                                        (*__slate_slot_333)
                                                            .wrapping_sub((192 as i32) as u32)
                                                            as isize,
                                                    )
                                                }
                                            })
                                                as u32;
                                            loop {
                                                if *__slate_slot_330 < *__slate_slot_331
                                                    && (((unsafe { *(*__slate_slot_330) }) as u32)
                                                        as i32)
                                                        & (192 as i32)
                                                        == (128 as i32)
                                                {
                                                    std::ptr::write(
                                                        __slate_slot_456,
                                                        *__slate_slot_330,
                                                    );
                                                    std::ptr::write(__slate_slot_457, unsafe {
                                                        (*__slate_slot_456)
                                                            .offset((1 as i32) as isize)
                                                    });
                                                    *__slate_slot_330 = *__slate_slot_457;
                                                    *__slate_slot_333 = (*__slate_slot_333
                                                        << (6 as i32))
                                                        .wrapping_add(
                                                            ((63 as i32)
                                                                & (((unsafe {
                                                                    *(*__slate_slot_456)
                                                                })
                                                                    as u32)
                                                                    as i32))
                                                                as u32,
                                                        );
                                                } else {
                                                    break;
                                                }
                                            }
                                            if *__slate_slot_333 < ((128 as i32) as u32)
                                                || *__slate_slot_333 & (4294965248 as u32)
                                                    == ((55296 as i32) as u32)
                                                || *__slate_slot_333 & (4294967294 as u32)
                                                    == ((65534 as i32) as u32)
                                            {
                                                *__slate_slot_333 = (65533 as i32) as u32;
                                            }
                                        }
                                        {}
                                        if *__slate_slot_333 <= ((65535 as i32) as u32) {
                                            std::ptr::write(__slate_slot_458, *__slate_slot_332);
                                            std::ptr::write(__slate_slot_459, unsafe {
                                                (*__slate_slot_458).offset((1 as i32) as isize)
                                            });
                                            *__slate_slot_332 = *__slate_slot_459;
                                            unsafe {
                                                *(*__slate_slot_458) = (*__slate_slot_333
                                                    >> (8 as i32)
                                                    & ((255 as i32) as u32))
                                                    as u8;
                                            }
                                            std::ptr::write(__slate_slot_460, *__slate_slot_332);
                                            std::ptr::write(__slate_slot_461, unsafe {
                                                (*__slate_slot_460).offset((1 as i32) as isize)
                                            });
                                            *__slate_slot_332 = *__slate_slot_461;
                                            unsafe {
                                                *(*__slate_slot_460) = (*__slate_slot_333
                                                    & ((255 as i32) as u32))
                                                    as u8;
                                            }
                                        } else {
                                            std::ptr::write(__slate_slot_462, *__slate_slot_332);
                                            std::ptr::write(__slate_slot_463, unsafe {
                                                (*__slate_slot_462).offset((1 as i32) as isize)
                                            });
                                            *__slate_slot_332 = *__slate_slot_463;
                                            unsafe {
                                                *(*__slate_slot_462) = ((216 as i32) as u32)
                                                    .wrapping_add(
                                                        (*__slate_slot_333)
                                                            .wrapping_sub((65536 as i32) as u32)
                                                            >> (18 as i32)
                                                            & ((3 as i32) as u32),
                                                    )
                                                    as u8;
                                            }
                                            std::ptr::write(__slate_slot_464, *__slate_slot_332);
                                            std::ptr::write(__slate_slot_465, unsafe {
                                                (*__slate_slot_464).offset((1 as i32) as isize)
                                            });
                                            *__slate_slot_332 = *__slate_slot_465;
                                            unsafe {
                                                *(*__slate_slot_464) = (*__slate_slot_333
                                                    >> (10 as i32)
                                                    & ((63 as i32) as u32))
                                                    .wrapping_add(
                                                        (*__slate_slot_333)
                                                            .wrapping_sub((65536 as i32) as u32)
                                                            >> (10 as i32)
                                                            & ((192 as i32) as u32),
                                                    )
                                                    as u8;
                                            }
                                            std::ptr::write(__slate_slot_466, *__slate_slot_332);
                                            std::ptr::write(__slate_slot_467, unsafe {
                                                (*__slate_slot_466).offset((1 as i32) as isize)
                                            });
                                            *__slate_slot_332 = *__slate_slot_467;
                                            unsafe {
                                                *(*__slate_slot_466) = ((220 as i32) as u32)
                                                    .wrapping_add(
                                                        *__slate_slot_333 >> (8 as i32)
                                                            & ((3 as i32) as u32),
                                                    )
                                                    as u8;
                                            }
                                            std::ptr::write(__slate_slot_468, *__slate_slot_332);
                                            std::ptr::write(__slate_slot_469, unsafe {
                                                (*__slate_slot_468).offset((1 as i32) as isize)
                                            });
                                            *__slate_slot_332 = *__slate_slot_469;
                                            unsafe {
                                                *(*__slate_slot_468) = (*__slate_slot_333
                                                    & ((255 as i32) as u32))
                                                    as u8;
                                            }
                                        }
                                        {}
                                    } else {
                                        break '__join_2;
                                    }
                                }
                            }
                        }
                        unsafe {
                            (*pMem).n = ((unsafe {
                                (*__slate_slot_332).offset_from(*__slate_slot_329 as *mut u8)
                            }) as i64) as i32;
                        }
                        std::ptr::write(__slate_slot_470, *__slate_slot_332);
                        std::ptr::write(__slate_slot_471, unsafe {
                            (*__slate_slot_470).offset((1 as i32) as isize)
                        });
                        *__slate_slot_332 = *__slate_slot_471;
                        unsafe {
                            *(*__slate_slot_470) = ((0 as i32) as i8) as u8;
                        }
                    } else {
                        '__join_28: {
                            0 as i32;
                            if (((unsafe { (*pMem).enc }) as u32) as i32) == (2 as i32) {
                                // /* UTF-16 Little-endian -> UTF-8 */
                                loop {
                                    if *__slate_slot_330 < *__slate_slot_331 {
                                        std::ptr::write(__slate_slot_472, *__slate_slot_330);
                                        std::ptr::write(__slate_slot_473, unsafe {
                                            (*__slate_slot_472).offset((1 as i32) as isize)
                                        });
                                        *__slate_slot_330 = *__slate_slot_473;
                                        *__slate_slot_333 =
                                            (unsafe { *(*__slate_slot_472) }) as u32;
                                        std::ptr::write(__slate_slot_474, *__slate_slot_333);
                                        std::ptr::write(__slate_slot_475, *__slate_slot_330);
                                        std::ptr::write(__slate_slot_476, unsafe {
                                            (*__slate_slot_475).offset((1 as i32) as isize)
                                        });
                                        *__slate_slot_330 = *__slate_slot_476;
                                        std::ptr::write(
                                            __slate_slot_477,
                                            (*__slate_slot_474).wrapping_add(
                                                ((((unsafe { *(*__slate_slot_475) }) as u32)
                                                    as i32)
                                                    << (8 as i32))
                                                    as u32,
                                            ),
                                        );
                                        *__slate_slot_333 = *__slate_slot_477;
                                        if *__slate_slot_333 >= ((55296 as i32) as u32)
                                            && *__slate_slot_333 < ((57344 as i32) as u32)
                                        {
                                            if *__slate_slot_330 < *__slate_slot_331 {
                                                std::ptr::write(
                                                    __slate_slot_478,
                                                    *__slate_slot_330,
                                                );
                                                std::ptr::write(__slate_slot_479, unsafe {
                                                    (*__slate_slot_478).offset((1 as i32) as isize)
                                                });
                                                *__slate_slot_330 = *__slate_slot_479;
                                                *__slate_slot_336 =
                                                    ((unsafe { *(*__slate_slot_478) }) as u32)
                                                        as i32;
                                                std::ptr::write(
                                                    __slate_slot_480,
                                                    *__slate_slot_336,
                                                );
                                                std::ptr::write(
                                                    __slate_slot_481,
                                                    *__slate_slot_330,
                                                );
                                                std::ptr::write(__slate_slot_482, unsafe {
                                                    (*__slate_slot_481).offset((1 as i32) as isize)
                                                });
                                                *__slate_slot_330 = *__slate_slot_482;
                                                std::ptr::write(
                                                    __slate_slot_483,
                                                    *__slate_slot_480
                                                        + ((((unsafe { *(*__slate_slot_481) })
                                                            as u32)
                                                            as i32)
                                                            << (8 as i32)),
                                                );
                                                *__slate_slot_336 = *__slate_slot_483;
                                                *__slate_slot_333 = ((*__slate_slot_336
                                                    & (1023 as i32))
                                                    as u32)
                                                    .wrapping_add(
                                                        (*__slate_slot_333 & ((63 as i32) as u32))
                                                            << (10 as i32),
                                                    )
                                                    .wrapping_add(
                                                        (*__slate_slot_333 & ((960 as i32) as u32))
                                                            .wrapping_add((64 as i32) as u32)
                                                            << (10 as i32),
                                                    );
                                            }
                                        }
                                        if *__slate_slot_333 < ((128 as i32) as u32) {
                                            std::ptr::write(__slate_slot_484, *__slate_slot_332);
                                            std::ptr::write(__slate_slot_485, unsafe {
                                                (*__slate_slot_484).offset((1 as i32) as isize)
                                            });
                                            *__slate_slot_332 = *__slate_slot_485;
                                            unsafe {
                                                *(*__slate_slot_484) = (*__slate_slot_333
                                                    & ((255 as i32) as u32))
                                                    as u8;
                                            }
                                        } else {
                                            if *__slate_slot_333 < ((2048 as i32) as u32) {
                                                std::ptr::write(
                                                    __slate_slot_486,
                                                    *__slate_slot_332,
                                                );
                                                std::ptr::write(__slate_slot_487, unsafe {
                                                    (*__slate_slot_486).offset((1 as i32) as isize)
                                                });
                                                *__slate_slot_332 = *__slate_slot_487;
                                                unsafe {
                                                    *(*__slate_slot_486) = (((192 as i32)
                                                        + ((((*__slate_slot_333 >> (6 as i32)
                                                            & ((31 as i32) as u32))
                                                            as u8)
                                                            as u32)
                                                            as i32))
                                                        as i8)
                                                        as u8;
                                                }
                                                std::ptr::write(
                                                    __slate_slot_488,
                                                    *__slate_slot_332,
                                                );
                                                std::ptr::write(__slate_slot_489, unsafe {
                                                    (*__slate_slot_488).offset((1 as i32) as isize)
                                                });
                                                *__slate_slot_332 = *__slate_slot_489;
                                                unsafe {
                                                    *(*__slate_slot_488) = (((128 as i32)
                                                        + ((((*__slate_slot_333
                                                            & ((63 as i32) as u32))
                                                            as u8)
                                                            as u32)
                                                            as i32))
                                                        as i8)
                                                        as u8;
                                                }
                                            } else {
                                                if *__slate_slot_333 < ((65536 as i32) as u32) {
                                                    std::ptr::write(
                                                        __slate_slot_490,
                                                        *__slate_slot_332,
                                                    );
                                                    std::ptr::write(__slate_slot_491, unsafe {
                                                        (*__slate_slot_490)
                                                            .offset((1 as i32) as isize)
                                                    });
                                                    *__slate_slot_332 = *__slate_slot_491;
                                                    unsafe {
                                                        *(*__slate_slot_490) = (((224 as i32)
                                                            + ((((*__slate_slot_333 >> (12 as i32)
                                                                & ((15 as i32) as u32))
                                                                as u8)
                                                                as u32)
                                                                as i32))
                                                            as i8)
                                                            as u8;
                                                    }
                                                    std::ptr::write(
                                                        __slate_slot_492,
                                                        *__slate_slot_332,
                                                    );
                                                    std::ptr::write(__slate_slot_493, unsafe {
                                                        (*__slate_slot_492)
                                                            .offset((1 as i32) as isize)
                                                    });
                                                    *__slate_slot_332 = *__slate_slot_493;
                                                    unsafe {
                                                        *(*__slate_slot_492) = (((128 as i32)
                                                            + ((((*__slate_slot_333 >> (6 as i32)
                                                                & ((63 as i32) as u32))
                                                                as u8)
                                                                as u32)
                                                                as i32))
                                                            as i8)
                                                            as u8;
                                                    }
                                                    std::ptr::write(
                                                        __slate_slot_494,
                                                        *__slate_slot_332,
                                                    );
                                                    std::ptr::write(__slate_slot_495, unsafe {
                                                        (*__slate_slot_494)
                                                            .offset((1 as i32) as isize)
                                                    });
                                                    *__slate_slot_332 = *__slate_slot_495;
                                                    unsafe {
                                                        *(*__slate_slot_494) = (((128 as i32)
                                                            + ((((*__slate_slot_333
                                                                & ((63 as i32) as u32))
                                                                as u8)
                                                                as u32)
                                                                as i32))
                                                            as i8)
                                                            as u8;
                                                    }
                                                } else {
                                                    std::ptr::write(
                                                        __slate_slot_496,
                                                        *__slate_slot_332,
                                                    );
                                                    std::ptr::write(__slate_slot_497, unsafe {
                                                        (*__slate_slot_496)
                                                            .offset((1 as i32) as isize)
                                                    });
                                                    *__slate_slot_332 = *__slate_slot_497;
                                                    unsafe {
                                                        *(*__slate_slot_496) = (((240 as i32)
                                                            + ((((*__slate_slot_333 >> (18 as i32)
                                                                & ((7 as i32) as u32))
                                                                as u8)
                                                                as u32)
                                                                as i32))
                                                            as i8)
                                                            as u8;
                                                    }
                                                    std::ptr::write(
                                                        __slate_slot_498,
                                                        *__slate_slot_332,
                                                    );
                                                    std::ptr::write(__slate_slot_499, unsafe {
                                                        (*__slate_slot_498)
                                                            .offset((1 as i32) as isize)
                                                    });
                                                    *__slate_slot_332 = *__slate_slot_499;
                                                    unsafe {
                                                        *(*__slate_slot_498) = (((128 as i32)
                                                            + ((((*__slate_slot_333 >> (12 as i32)
                                                                & ((63 as i32) as u32))
                                                                as u8)
                                                                as u32)
                                                                as i32))
                                                            as i8)
                                                            as u8;
                                                    }
                                                    std::ptr::write(
                                                        __slate_slot_500,
                                                        *__slate_slot_332,
                                                    );
                                                    std::ptr::write(__slate_slot_501, unsafe {
                                                        (*__slate_slot_500)
                                                            .offset((1 as i32) as isize)
                                                    });
                                                    *__slate_slot_332 = *__slate_slot_501;
                                                    unsafe {
                                                        *(*__slate_slot_500) = (((128 as i32)
                                                            + ((((*__slate_slot_333 >> (6 as i32)
                                                                & ((63 as i32) as u32))
                                                                as u8)
                                                                as u32)
                                                                as i32))
                                                            as i8)
                                                            as u8;
                                                    }
                                                    std::ptr::write(
                                                        __slate_slot_502,
                                                        *__slate_slot_332,
                                                    );
                                                    std::ptr::write(__slate_slot_503, unsafe {
                                                        (*__slate_slot_502)
                                                            .offset((1 as i32) as isize)
                                                    });
                                                    *__slate_slot_332 = *__slate_slot_503;
                                                    unsafe {
                                                        *(*__slate_slot_502) = (((128 as i32)
                                                            + ((((*__slate_slot_333
                                                                & ((63 as i32) as u32))
                                                                as u8)
                                                                as u32)
                                                                as i32))
                                                            as i8)
                                                            as u8;
                                                    }
                                                }
                                            }
                                        }
                                        {}
                                    } else {
                                        break '__join_28;
                                    }
                                }
                            } else {
                                // /* UTF-16 Big-endian -> UTF-8 */
                                loop {
                                    if *__slate_slot_330 < *__slate_slot_331 {
                                        std::ptr::write(__slate_slot_504, *__slate_slot_330);
                                        std::ptr::write(__slate_slot_505, unsafe {
                                            (*__slate_slot_504).offset((1 as i32) as isize)
                                        });
                                        *__slate_slot_330 = *__slate_slot_505;
                                        *__slate_slot_333 =
                                            ((((unsafe { *(*__slate_slot_504) }) as u32) as i32)
                                                << (8 as i32))
                                                as u32;
                                        std::ptr::write(__slate_slot_506, *__slate_slot_333);
                                        std::ptr::write(__slate_slot_507, *__slate_slot_330);
                                        std::ptr::write(__slate_slot_508, unsafe {
                                            (*__slate_slot_507).offset((1 as i32) as isize)
                                        });
                                        *__slate_slot_330 = *__slate_slot_508;
                                        std::ptr::write(
                                            __slate_slot_509,
                                            (*__slate_slot_506).wrapping_add(
                                                (((unsafe { *(*__slate_slot_507) }) as u32) as i32)
                                                    as u32,
                                            ),
                                        );
                                        *__slate_slot_333 = *__slate_slot_509;
                                        if *__slate_slot_333 >= ((55296 as i32) as u32)
                                            && *__slate_slot_333 < ((57344 as i32) as u32)
                                        {
                                            if *__slate_slot_330 < *__slate_slot_331 {
                                                std::ptr::write(
                                                    __slate_slot_510,
                                                    *__slate_slot_330,
                                                );
                                                std::ptr::write(__slate_slot_511, unsafe {
                                                    (*__slate_slot_510).offset((1 as i32) as isize)
                                                });
                                                *__slate_slot_330 = *__slate_slot_511;
                                                *__slate_slot_337 =
                                                    (((unsafe { *(*__slate_slot_510) }) as u32)
                                                        as i32)
                                                        << (8 as i32);
                                                std::ptr::write(
                                                    __slate_slot_512,
                                                    *__slate_slot_337,
                                                );
                                                std::ptr::write(
                                                    __slate_slot_513,
                                                    *__slate_slot_330,
                                                );
                                                std::ptr::write(__slate_slot_514, unsafe {
                                                    (*__slate_slot_513).offset((1 as i32) as isize)
                                                });
                                                *__slate_slot_330 = *__slate_slot_514;
                                                std::ptr::write(
                                                    __slate_slot_515,
                                                    *__slate_slot_512
                                                        + (((unsafe { *(*__slate_slot_513) })
                                                            as u32)
                                                            as i32),
                                                );
                                                *__slate_slot_337 = *__slate_slot_515;
                                                *__slate_slot_333 = ((*__slate_slot_337
                                                    & (1023 as i32))
                                                    as u32)
                                                    .wrapping_add(
                                                        (*__slate_slot_333 & ((63 as i32) as u32))
                                                            << (10 as i32),
                                                    )
                                                    .wrapping_add(
                                                        (*__slate_slot_333 & ((960 as i32) as u32))
                                                            .wrapping_add((64 as i32) as u32)
                                                            << (10 as i32),
                                                    );
                                            }
                                        }
                                        if *__slate_slot_333 < ((128 as i32) as u32) {
                                            std::ptr::write(__slate_slot_516, *__slate_slot_332);
                                            std::ptr::write(__slate_slot_517, unsafe {
                                                (*__slate_slot_516).offset((1 as i32) as isize)
                                            });
                                            *__slate_slot_332 = *__slate_slot_517;
                                            unsafe {
                                                *(*__slate_slot_516) = (*__slate_slot_333
                                                    & ((255 as i32) as u32))
                                                    as u8;
                                            }
                                        } else {
                                            if *__slate_slot_333 < ((2048 as i32) as u32) {
                                                std::ptr::write(
                                                    __slate_slot_518,
                                                    *__slate_slot_332,
                                                );
                                                std::ptr::write(__slate_slot_519, unsafe {
                                                    (*__slate_slot_518).offset((1 as i32) as isize)
                                                });
                                                *__slate_slot_332 = *__slate_slot_519;
                                                unsafe {
                                                    *(*__slate_slot_518) = (((192 as i32)
                                                        + ((((*__slate_slot_333 >> (6 as i32)
                                                            & ((31 as i32) as u32))
                                                            as u8)
                                                            as u32)
                                                            as i32))
                                                        as i8)
                                                        as u8;
                                                }
                                                std::ptr::write(
                                                    __slate_slot_520,
                                                    *__slate_slot_332,
                                                );
                                                std::ptr::write(__slate_slot_521, unsafe {
                                                    (*__slate_slot_520).offset((1 as i32) as isize)
                                                });
                                                *__slate_slot_332 = *__slate_slot_521;
                                                unsafe {
                                                    *(*__slate_slot_520) = (((128 as i32)
                                                        + ((((*__slate_slot_333
                                                            & ((63 as i32) as u32))
                                                            as u8)
                                                            as u32)
                                                            as i32))
                                                        as i8)
                                                        as u8;
                                                }
                                            } else {
                                                if *__slate_slot_333 < ((65536 as i32) as u32) {
                                                    std::ptr::write(
                                                        __slate_slot_522,
                                                        *__slate_slot_332,
                                                    );
                                                    std::ptr::write(__slate_slot_523, unsafe {
                                                        (*__slate_slot_522)
                                                            .offset((1 as i32) as isize)
                                                    });
                                                    *__slate_slot_332 = *__slate_slot_523;
                                                    unsafe {
                                                        *(*__slate_slot_522) = (((224 as i32)
                                                            + ((((*__slate_slot_333 >> (12 as i32)
                                                                & ((15 as i32) as u32))
                                                                as u8)
                                                                as u32)
                                                                as i32))
                                                            as i8)
                                                            as u8;
                                                    }
                                                    std::ptr::write(
                                                        __slate_slot_524,
                                                        *__slate_slot_332,
                                                    );
                                                    std::ptr::write(__slate_slot_525, unsafe {
                                                        (*__slate_slot_524)
                                                            .offset((1 as i32) as isize)
                                                    });
                                                    *__slate_slot_332 = *__slate_slot_525;
                                                    unsafe {
                                                        *(*__slate_slot_524) = (((128 as i32)
                                                            + ((((*__slate_slot_333 >> (6 as i32)
                                                                & ((63 as i32) as u32))
                                                                as u8)
                                                                as u32)
                                                                as i32))
                                                            as i8)
                                                            as u8;
                                                    }
                                                    std::ptr::write(
                                                        __slate_slot_526,
                                                        *__slate_slot_332,
                                                    );
                                                    std::ptr::write(__slate_slot_527, unsafe {
                                                        (*__slate_slot_526)
                                                            .offset((1 as i32) as isize)
                                                    });
                                                    *__slate_slot_332 = *__slate_slot_527;
                                                    unsafe {
                                                        *(*__slate_slot_526) = (((128 as i32)
                                                            + ((((*__slate_slot_333
                                                                & ((63 as i32) as u32))
                                                                as u8)
                                                                as u32)
                                                                as i32))
                                                            as i8)
                                                            as u8;
                                                    }
                                                } else {
                                                    std::ptr::write(
                                                        __slate_slot_528,
                                                        *__slate_slot_332,
                                                    );
                                                    std::ptr::write(__slate_slot_529, unsafe {
                                                        (*__slate_slot_528)
                                                            .offset((1 as i32) as isize)
                                                    });
                                                    *__slate_slot_332 = *__slate_slot_529;
                                                    unsafe {
                                                        *(*__slate_slot_528) = (((240 as i32)
                                                            + ((((*__slate_slot_333 >> (18 as i32)
                                                                & ((7 as i32) as u32))
                                                                as u8)
                                                                as u32)
                                                                as i32))
                                                            as i8)
                                                            as u8;
                                                    }
                                                    std::ptr::write(
                                                        __slate_slot_530,
                                                        *__slate_slot_332,
                                                    );
                                                    std::ptr::write(__slate_slot_531, unsafe {
                                                        (*__slate_slot_530)
                                                            .offset((1 as i32) as isize)
                                                    });
                                                    *__slate_slot_332 = *__slate_slot_531;
                                                    unsafe {
                                                        *(*__slate_slot_530) = (((128 as i32)
                                                            + ((((*__slate_slot_333 >> (12 as i32)
                                                                & ((63 as i32) as u32))
                                                                as u8)
                                                                as u32)
                                                                as i32))
                                                            as i8)
                                                            as u8;
                                                    }
                                                    std::ptr::write(
                                                        __slate_slot_532,
                                                        *__slate_slot_332,
                                                    );
                                                    std::ptr::write(__slate_slot_533, unsafe {
                                                        (*__slate_slot_532)
                                                            .offset((1 as i32) as isize)
                                                    });
                                                    *__slate_slot_332 = *__slate_slot_533;
                                                    unsafe {
                                                        *(*__slate_slot_532) = (((128 as i32)
                                                            + ((((*__slate_slot_333 >> (6 as i32)
                                                                & ((63 as i32) as u32))
                                                                as u8)
                                                                as u32)
                                                                as i32))
                                                            as i8)
                                                            as u8;
                                                    }
                                                    std::ptr::write(
                                                        __slate_slot_534,
                                                        *__slate_slot_332,
                                                    );
                                                    std::ptr::write(__slate_slot_535, unsafe {
                                                        (*__slate_slot_534)
                                                            .offset((1 as i32) as isize)
                                                    });
                                                    *__slate_slot_332 = *__slate_slot_535;
                                                    unsafe {
                                                        *(*__slate_slot_534) = (((128 as i32)
                                                            + ((((*__slate_slot_333
                                                                & ((63 as i32) as u32))
                                                                as u8)
                                                                as u32)
                                                                as i32))
                                                            as i8)
                                                            as u8;
                                                    }
                                                }
                                            }
                                        }
                                        {}
                                    } else {
                                        break '__join_28;
                                    }
                                }
                            }
                        }
                        unsafe {
                            (*pMem).n = ((unsafe {
                                (*__slate_slot_332).offset_from(*__slate_slot_329 as *mut u8)
                            }) as i64) as i32;
                        }
                    }
                    unsafe {
                        *(*__slate_slot_332) = ((0 as i32) as i8) as u8;
                    }
                    0 as i32;
                    *__slate_slot_333 = ((2 as i32)
                        | (512 as i32)
                        | (((unsafe { (*pMem).flags }) as u32) as i32)
                            & ((63 as i32) | (2048 as i32)))
                        as u32;
                    unsafe { sqlite3VdbeMemRelease(pMem) };
                    unsafe {
                        (*pMem).flags = *__slate_slot_333 as u16;
                    }
                    unsafe {
                        (*pMem).enc = desiredEnc;
                    }
                    unsafe {
                        (*pMem).z = *__slate_slot_329 as *mut i8;
                    }
                    unsafe {
                        (*pMem).zMalloc = unsafe { (*pMem).z };
                    }
                    unsafe {
                        (*pMem).szMalloc = unsafe {
                            sqlite3DbMallocSize(
                                unsafe { (*pMem).db },
                                (unsafe { (*pMem).z }) as *const (),
                            )
                        };
                    }
                }
            }
        }
        return 0 as i32;
    }
    return unsafe { std::mem::zeroed() };
}

// /* SQLITE_OMIT_UTF16 */
// /*
// ** This routine checks for a byte-order mark at the beginning of the
// ** UTF-16 string stored in *pMem. If one is present, it is removed and
// ** the encoding of the Mem adjusted. This routine does not do any
// ** byte-swapping, it just sets Mem.enc appropriately.
// **
// ** The allocation (static, dynamic etc.) and encoding of the Mem may be
// ** changed by this function.
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3VdbeMemHandleBom(mut pMem: *mut sqlite3_value) -> i32 {
    let mut rc: i32 = 0 as i32;
    let mut bom: u8 = ((0 as i32) as i8) as u8;
    0 as i32;
    if (unsafe { (*pMem).n }) > (1 as i32) {
        let mut b1: u8 = unsafe { *((unsafe { (*pMem).z }) as *mut u8) };
        let mut b2: u8 =
            unsafe { *unsafe { ((unsafe { (*pMem).z }) as *mut u8).offset((1 as i32) as isize) } };
        if ((b1 as u32) as i32) == (254 as i32) && ((b2 as u32) as i32) == (255 as i32) {
            bom = ((3 as i32) as i8) as u8;
        }
        if ((b1 as u32) as i32) == (255 as i32) && ((b2 as u32) as i32) == (254 as i32) {
            bom = ((2 as i32) as i8) as u8;
        }
    }
    if bom != (0 as u8) {
        rc = unsafe { sqlite3VdbeMemMakeWriteable(pMem) };
        if rc == (0 as i32) {
            let __v536: *mut sqlite3_value = pMem;
            let __v537: i32 = unsafe { (*__v536).n };
            let __v538: i32 = __v537 - (2 as i32);
            unsafe {
                (*__v536).n = __v538;
            }
            unsafe {
                memmove(
                    (unsafe { (*pMem).z }) as *mut (),
                    (unsafe { unsafe { (*pMem).z }.offset((2 as i32) as isize) }) as *const (),
                    ((unsafe { (*pMem).n }) as i64) as u64,
                )
            };
            unsafe {
                *unsafe { unsafe { (*pMem).z }.offset((unsafe { (*pMem).n }) as isize) } =
                    (0 as i32) as i8;
            }
            unsafe {
                *unsafe {
                    unsafe { (*pMem).z }.offset(((unsafe { (*pMem).n }) + (1 as i32)) as isize)
                } = (0 as i32) as i8;
            }
            let __v539: *mut sqlite3_value = pMem;
            let __v540: u16 = unsafe { (*__v539).flags };
            let __v541: u16 = ((((__v540 as u32) as i32) | (512 as i32)) as i16) as u16;
            unsafe {
                (*__v539).flags = __v541;
            }
            unsafe {
                (*pMem).enc = bom;
            }
        }
    }
    return rc;
}
