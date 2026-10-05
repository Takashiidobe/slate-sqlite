unsafe extern "C" {
    static mut sqlite3UpperToLower: [u8; 0];
    static mut sqlite3CtypeMap: [u8; 0];
    fn sqlite3_free(__v343: *mut ());
    fn sqlite3_log(iErrCode: i32, zFormat: *const i8, ...);
    fn sqlite3Malloc(__v352: u64) -> *mut ();
    fn sqlite3DbStrDup(__v353: *mut sqlite3, __v354: *const i8) -> *mut i8;
    fn sqlite3DbNNFreeNN(__v355: *mut sqlite3, __v356: *mut ());
    fn sqlite3ErrorMsg(__v357: *mut Parse, __v358: *const i8, ...);
    fn sqlite3DeleteTable(__v363: *mut sqlite3, __v364: *mut Table);
    fn sqlite3DeleteTrigger(__v365: *mut sqlite3, __v366: *mut Trigger);
    fn sqlite3ErrStr(__v367: i32) -> *const i8;
    fn sqlite3OomFault(__v370: *mut sqlite3) -> *mut ();
    fn sqlite3ParserAlloc(
        __v371: Option<unsafe extern "C-unwind" fn(u64) -> *mut ()>,
        __v372: *mut Parse,
    ) -> *mut ();
    fn sqlite3ParserFree(__v373: *mut (), __v374: Option<unsafe extern "C-unwind" fn(*mut ())>);
    fn sqlite3Parser(__v375: *mut (), __v376: i32, __v377: Token);
    fn sqlite3ParserFallback(__v378: i32) -> i32;
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
    trace: __SlateRecord153,
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
    u1: __SlateRecord154,
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
    __slate_bits_0: __slate_bits::__SlateBits60U0,
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
    u: __SlateRecord164,
    pLeft: *mut Expr,
    pRight: *mut Expr,
    x: __SlateRecord165,
    nHeight: i32,
    iTable: i32,
    iColumn: i16,
    iAgg: i16,
    w: __SlateRecord166,
    pAggInfo: *mut AggInfo,
    y: __SlateRecord167,
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
    u: __SlateRecord155,
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
    __slate_bits_0: __slate_bits::__SlateBits84U0,
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
    __slate_bits_0: __slate_bits::__SlateBits96U0,
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
    u1: __SlateRecord180,
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
    fg: __SlateRecord174,
    iCursor: i32,
    colUsed: u64,
    u1: __SlateRecord175,
    u2: __SlateRecord176,
    u3: __SlateRecord177,
    u4: __SlateRecord178,
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
    u: __SlateRecord156,
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
    __slate_bits_0: __slate_bits::__SlateBits152U0,
    azInit: *mut *const i8,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord153 {
    xLegacy: Option<unsafe extern "C-unwind" fn(*mut (), *const i8)>,
    xV2: Option<unsafe extern "C-unwind" fn(u32, *mut (), *mut (), *mut ()) -> i32>,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord154 {
    isInterrupted: i32,
    notUsed1: f64,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord155 {
    pHash: *mut FuncDef,
    pDestructor: *mut FuncDestructor,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord156 {
    tab: __SlateRecord157,
    view: __SlateRecord158,
    vtab: __SlateRecord159,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord157 {
    addColOffset: i32,
    pFKey: *mut FKey,
    pDfltList: *mut ExprList,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord158 {
    pSelect: *mut Select,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord159 {
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
union __SlateRecord164 {
    zToken: *mut i8,
    iValue: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord165 {
    pList: *mut ExprList,
    pSelect: *mut Select,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord166 {
    iJoin: i32,
    iOfst: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord167 {
    pTab: *mut Table,
    pWin: *mut Window,
    nReg: i32,
    sub: __SlateRecord168,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord168 {
    iAddr: i32,
    regReturn: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct ExprList_item {
    pExpr: *mut Expr,
    zEName: *mut i8,
    fg: __SlateRecord170,
    u: __SlateRecord171,
}

#[repr(C, align(4))]
#[derive(Clone, Copy)]
struct __SlateRecord170 {
    sortFlags: u8,
    __slate_bits_0: __slate_bits::__SlateBits170U0,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord171 {
    x: __SlateRecord172,
    iConstExprReg: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord172 {
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
struct __SlateRecord174 {
    jointype: u8,
    __slate_bits_0: __slate_bits::__SlateBits174U0,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord175 {
    zIndexedBy: *mut i8,
    pFuncArg: *mut ExprList,
    nRow: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord176 {
    pIBIndex: *mut Index,
    pCteUse: *mut CteUse,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord177 {
    pOn: *mut Expr,
    pUsing: *mut IdList,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord178 {
    pSchema: *mut Schema,
    zDatabase: *mut i8,
    pSubq: *mut Subquery,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord180 {
    cr: __SlateRecord181,
    d: __SlateRecord182,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord181 {
    addrCrTab: i32,
    regRowid: i32,
    regRoot: i32,
    constraintName: Token,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord182 {
    pReturning: *mut Returning,
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
    pub struct __SlateBits60U0 {
        #[bits(4)]
        pub notNull: u32,
        #[bits(4)]
        pub eCType: u32,
    }
    #[bitfields::bitfield([u8; 2], c_names = true, new = false, from_into_bits = false, from_traits = false, default = false, debug = false, builder = false, bit_ops = false)]
    pub struct __SlateBits170U0 {
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
    pub struct __SlateBits174U0 {
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
    pub struct __SlateBits84U0 {
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
    pub struct __SlateBits152U0 {
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
    pub struct __SlateBits96U0 {
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
// ** An tokenizer for SQL
// **
// ** This file contains C code that splits an SQL input string up into
// ** individual tokens and sends those tokens one-by-one over to the
// ** parser for analysis.
// */
// /* Character classes for tokenizing
// **
// ** In the sqlite3GetToken() function, a switch() on aiClass[c] is implemented
// ** using a lookup table, whereas a switch() directly on c uses a binary search.
// ** The lookup table is much faster.  To maximize speed, and to ensure that
// ** a lookup table is used, all of the classes need to be small integers and
// ** all of them need to be used within the switch.
// */
// /* The letter 'x', or start of BLOB literal */
// /* First letter of a keyword */
// /* Alphabetics or '_'.  Usable in a keyword */
// /* Digits */
// /* '$' */
// /* '@', '#', ':'.  Alphabetic SQL variables */
// /* '?'.  Numeric SQL variables */
// /* Space characters */
// /* '"', '\'', or '`'.  String literals, quoted ids */
// /* '['.   [...] style quoted ids */
// /* '|'.   Bitwise OR or concatenate */
// /* '-'.  Minus or SQL-style comment */
// /* '<'.  Part of < or <= or <> */
// /* '>'.  Part of > or >= */
// /* '='.  Part of = or == */
// /* '!'.  Part of != */
// /* '/'.  / or c-style comment */
// /* '(' */
// /* ')' */
// /* ';' */
// /* '+' */
// /* '*' */
// /* '%' */
// /* ',' */
// /* '&' */
// /* '~' */
// /* '.' */
// /* unicode characters usable in IDs */
// /* Illegal character */
// /* 0x00 */
// /* First byte of UTF8 BOM:  0xEF 0xBB 0xBF */
static mut aiClass: __SlateAlign16<[u8; 256]> = __SlateAlign16([
    ((29 as i32) as i8) as u8,
    ((28 as i32) as i8) as u8,
    ((28 as i32) as i8) as u8,
    ((28 as i32) as i8) as u8,
    ((28 as i32) as i8) as u8,
    ((28 as i32) as i8) as u8,
    ((28 as i32) as i8) as u8,
    ((28 as i32) as i8) as u8,
    ((28 as i32) as i8) as u8,
    ((7 as i32) as i8) as u8,
    ((7 as i32) as i8) as u8,
    ((28 as i32) as i8) as u8,
    ((7 as i32) as i8) as u8,
    ((7 as i32) as i8) as u8,
    ((28 as i32) as i8) as u8,
    ((28 as i32) as i8) as u8,
    ((28 as i32) as i8) as u8,
    ((28 as i32) as i8) as u8,
    ((28 as i32) as i8) as u8,
    ((28 as i32) as i8) as u8,
    ((28 as i32) as i8) as u8,
    ((28 as i32) as i8) as u8,
    ((28 as i32) as i8) as u8,
    ((28 as i32) as i8) as u8,
    ((28 as i32) as i8) as u8,
    ((28 as i32) as i8) as u8,
    ((28 as i32) as i8) as u8,
    ((28 as i32) as i8) as u8,
    ((28 as i32) as i8) as u8,
    ((28 as i32) as i8) as u8,
    ((28 as i32) as i8) as u8,
    ((28 as i32) as i8) as u8,
    ((7 as i32) as i8) as u8,
    ((15 as i32) as i8) as u8,
    ((8 as i32) as i8) as u8,
    ((5 as i32) as i8) as u8,
    ((4 as i32) as i8) as u8,
    ((22 as i32) as i8) as u8,
    ((24 as i32) as i8) as u8,
    ((8 as i32) as i8) as u8,
    ((17 as i32) as i8) as u8,
    ((18 as i32) as i8) as u8,
    ((21 as i32) as i8) as u8,
    ((20 as i32) as i8) as u8,
    ((23 as i32) as i8) as u8,
    ((11 as i32) as i8) as u8,
    ((26 as i32) as i8) as u8,
    ((16 as i32) as i8) as u8,
    ((3 as i32) as i8) as u8,
    ((3 as i32) as i8) as u8,
    ((3 as i32) as i8) as u8,
    ((3 as i32) as i8) as u8,
    ((3 as i32) as i8) as u8,
    ((3 as i32) as i8) as u8,
    ((3 as i32) as i8) as u8,
    ((3 as i32) as i8) as u8,
    ((3 as i32) as i8) as u8,
    ((3 as i32) as i8) as u8,
    ((5 as i32) as i8) as u8,
    ((19 as i32) as i8) as u8,
    ((12 as i32) as i8) as u8,
    ((14 as i32) as i8) as u8,
    ((13 as i32) as i8) as u8,
    ((6 as i32) as i8) as u8,
    ((5 as i32) as i8) as u8,
    ((1 as i32) as i8) as u8,
    ((1 as i32) as i8) as u8,
    ((1 as i32) as i8) as u8,
    ((1 as i32) as i8) as u8,
    ((1 as i32) as i8) as u8,
    ((1 as i32) as i8) as u8,
    ((1 as i32) as i8) as u8,
    ((1 as i32) as i8) as u8,
    ((1 as i32) as i8) as u8,
    ((1 as i32) as i8) as u8,
    ((1 as i32) as i8) as u8,
    ((1 as i32) as i8) as u8,
    ((1 as i32) as i8) as u8,
    ((1 as i32) as i8) as u8,
    ((1 as i32) as i8) as u8,
    ((1 as i32) as i8) as u8,
    ((1 as i32) as i8) as u8,
    ((1 as i32) as i8) as u8,
    ((1 as i32) as i8) as u8,
    ((1 as i32) as i8) as u8,
    ((1 as i32) as i8) as u8,
    ((1 as i32) as i8) as u8,
    ((1 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((2 as i32) as i8) as u8,
    ((2 as i32) as i8) as u8,
    ((9 as i32) as i8) as u8,
    ((28 as i32) as i8) as u8,
    ((28 as i32) as i8) as u8,
    ((28 as i32) as i8) as u8,
    ((2 as i32) as i8) as u8,
    ((8 as i32) as i8) as u8,
    ((1 as i32) as i8) as u8,
    ((1 as i32) as i8) as u8,
    ((1 as i32) as i8) as u8,
    ((1 as i32) as i8) as u8,
    ((1 as i32) as i8) as u8,
    ((1 as i32) as i8) as u8,
    ((1 as i32) as i8) as u8,
    ((1 as i32) as i8) as u8,
    ((1 as i32) as i8) as u8,
    ((1 as i32) as i8) as u8,
    ((1 as i32) as i8) as u8,
    ((1 as i32) as i8) as u8,
    ((1 as i32) as i8) as u8,
    ((1 as i32) as i8) as u8,
    ((1 as i32) as i8) as u8,
    ((1 as i32) as i8) as u8,
    ((1 as i32) as i8) as u8,
    ((1 as i32) as i8) as u8,
    ((1 as i32) as i8) as u8,
    ((1 as i32) as i8) as u8,
    ((1 as i32) as i8) as u8,
    ((1 as i32) as i8) as u8,
    ((1 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((2 as i32) as i8) as u8,
    ((2 as i32) as i8) as u8,
    ((28 as i32) as i8) as u8,
    ((10 as i32) as i8) as u8,
    ((28 as i32) as i8) as u8,
    ((25 as i32) as i8) as u8,
    ((28 as i32) as i8) as u8,
    ((27 as i32) as i8) as u8,
    ((27 as i32) as i8) as u8,
    ((27 as i32) as i8) as u8,
    ((27 as i32) as i8) as u8,
    ((27 as i32) as i8) as u8,
    ((27 as i32) as i8) as u8,
    ((27 as i32) as i8) as u8,
    ((27 as i32) as i8) as u8,
    ((27 as i32) as i8) as u8,
    ((27 as i32) as i8) as u8,
    ((27 as i32) as i8) as u8,
    ((27 as i32) as i8) as u8,
    ((27 as i32) as i8) as u8,
    ((27 as i32) as i8) as u8,
    ((27 as i32) as i8) as u8,
    ((27 as i32) as i8) as u8,
    ((27 as i32) as i8) as u8,
    ((27 as i32) as i8) as u8,
    ((27 as i32) as i8) as u8,
    ((27 as i32) as i8) as u8,
    ((27 as i32) as i8) as u8,
    ((27 as i32) as i8) as u8,
    ((27 as i32) as i8) as u8,
    ((27 as i32) as i8) as u8,
    ((27 as i32) as i8) as u8,
    ((27 as i32) as i8) as u8,
    ((27 as i32) as i8) as u8,
    ((27 as i32) as i8) as u8,
    ((27 as i32) as i8) as u8,
    ((27 as i32) as i8) as u8,
    ((27 as i32) as i8) as u8,
    ((27 as i32) as i8) as u8,
    ((27 as i32) as i8) as u8,
    ((27 as i32) as i8) as u8,
    ((27 as i32) as i8) as u8,
    ((27 as i32) as i8) as u8,
    ((27 as i32) as i8) as u8,
    ((27 as i32) as i8) as u8,
    ((27 as i32) as i8) as u8,
    ((27 as i32) as i8) as u8,
    ((27 as i32) as i8) as u8,
    ((27 as i32) as i8) as u8,
    ((27 as i32) as i8) as u8,
    ((27 as i32) as i8) as u8,
    ((27 as i32) as i8) as u8,
    ((27 as i32) as i8) as u8,
    ((27 as i32) as i8) as u8,
    ((27 as i32) as i8) as u8,
    ((27 as i32) as i8) as u8,
    ((27 as i32) as i8) as u8,
    ((27 as i32) as i8) as u8,
    ((27 as i32) as i8) as u8,
    ((27 as i32) as i8) as u8,
    ((27 as i32) as i8) as u8,
    ((27 as i32) as i8) as u8,
    ((27 as i32) as i8) as u8,
    ((27 as i32) as i8) as u8,
    ((27 as i32) as i8) as u8,
    ((27 as i32) as i8) as u8,
    ((27 as i32) as i8) as u8,
    ((27 as i32) as i8) as u8,
    ((27 as i32) as i8) as u8,
    ((27 as i32) as i8) as u8,
    ((27 as i32) as i8) as u8,
    ((27 as i32) as i8) as u8,
    ((27 as i32) as i8) as u8,
    ((27 as i32) as i8) as u8,
    ((27 as i32) as i8) as u8,
    ((27 as i32) as i8) as u8,
    ((27 as i32) as i8) as u8,
    ((27 as i32) as i8) as u8,
    ((27 as i32) as i8) as u8,
    ((27 as i32) as i8) as u8,
    ((27 as i32) as i8) as u8,
    ((27 as i32) as i8) as u8,
    ((27 as i32) as i8) as u8,
    ((27 as i32) as i8) as u8,
    ((27 as i32) as i8) as u8,
    ((27 as i32) as i8) as u8,
    ((27 as i32) as i8) as u8,
    ((27 as i32) as i8) as u8,
    ((27 as i32) as i8) as u8,
    ((27 as i32) as i8) as u8,
    ((27 as i32) as i8) as u8,
    ((27 as i32) as i8) as u8,
    ((27 as i32) as i8) as u8,
    ((27 as i32) as i8) as u8,
    ((27 as i32) as i8) as u8,
    ((27 as i32) as i8) as u8,
    ((27 as i32) as i8) as u8,
    ((27 as i32) as i8) as u8,
    ((27 as i32) as i8) as u8,
    ((27 as i32) as i8) as u8,
    ((27 as i32) as i8) as u8,
    ((27 as i32) as i8) as u8,
    ((27 as i32) as i8) as u8,
    ((27 as i32) as i8) as u8,
    ((27 as i32) as i8) as u8,
    ((27 as i32) as i8) as u8,
    ((27 as i32) as i8) as u8,
    ((27 as i32) as i8) as u8,
    ((27 as i32) as i8) as u8,
    ((27 as i32) as i8) as u8,
    ((27 as i32) as i8) as u8,
    ((27 as i32) as i8) as u8,
    ((27 as i32) as i8) as u8,
    ((27 as i32) as i8) as u8,
    ((27 as i32) as i8) as u8,
    ((27 as i32) as i8) as u8,
    ((27 as i32) as i8) as u8,
    ((27 as i32) as i8) as u8,
    ((30 as i32) as i8) as u8,
    ((27 as i32) as i8) as u8,
    ((27 as i32) as i8) as u8,
    ((27 as i32) as i8) as u8,
    ((27 as i32) as i8) as u8,
    ((27 as i32) as i8) as u8,
    ((27 as i32) as i8) as u8,
    ((27 as i32) as i8) as u8,
    ((27 as i32) as i8) as u8,
    ((27 as i32) as i8) as u8,
    ((27 as i32) as i8) as u8,
    ((27 as i32) as i8) as u8,
    ((27 as i32) as i8) as u8,
    ((27 as i32) as i8) as u8,
    ((27 as i32) as i8) as u8,
    ((27 as i32) as i8) as u8,
    ((27 as i32) as i8) as u8,
]);

static mut zKWText: __SlateAlign16<[i8; 666]> = __SlateAlign16([
    (82 as i32) as i8,
    (69 as i32) as i8,
    (73 as i32) as i8,
    (78 as i32) as i8,
    (68 as i32) as i8,
    (69 as i32) as i8,
    (88 as i32) as i8,
    (69 as i32) as i8,
    (68 as i32) as i8,
    (69 as i32) as i8,
    (83 as i32) as i8,
    (67 as i32) as i8,
    (65 as i32) as i8,
    (80 as i32) as i8,
    (69 as i32) as i8,
    (65 as i32) as i8,
    (67 as i32) as i8,
    (72 as i32) as i8,
    (69 as i32) as i8,
    (67 as i32) as i8,
    (75 as i32) as i8,
    (69 as i32) as i8,
    (89 as i32) as i8,
    (66 as i32) as i8,
    (69 as i32) as i8,
    (70 as i32) as i8,
    (79 as i32) as i8,
    (82 as i32) as i8,
    (69 as i32) as i8,
    (73 as i32) as i8,
    (71 as i32) as i8,
    (78 as i32) as i8,
    (79 as i32) as i8,
    (82 as i32) as i8,
    (69 as i32) as i8,
    (71 as i32) as i8,
    (69 as i32) as i8,
    (88 as i32) as i8,
    (80 as i32) as i8,
    (76 as i32) as i8,
    (65 as i32) as i8,
    (73 as i32) as i8,
    (78 as i32) as i8,
    (83 as i32) as i8,
    (84 as i32) as i8,
    (69 as i32) as i8,
    (65 as i32) as i8,
    (68 as i32) as i8,
    (68 as i32) as i8,
    (65 as i32) as i8,
    (84 as i32) as i8,
    (65 as i32) as i8,
    (66 as i32) as i8,
    (65 as i32) as i8,
    (83 as i32) as i8,
    (69 as i32) as i8,
    (76 as i32) as i8,
    (69 as i32) as i8,
    (67 as i32) as i8,
    (84 as i32) as i8,
    (65 as i32) as i8,
    (66 as i32) as i8,
    (76 as i32) as i8,
    (69 as i32) as i8,
    (70 as i32) as i8,
    (84 as i32) as i8,
    (72 as i32) as i8,
    (69 as i32) as i8,
    (78 as i32) as i8,
    (68 as i32) as i8,
    (69 as i32) as i8,
    (70 as i32) as i8,
    (69 as i32) as i8,
    (82 as i32) as i8,
    (82 as i32) as i8,
    (65 as i32) as i8,
    (66 as i32) as i8,
    (76 as i32) as i8,
    (69 as i32) as i8,
    (76 as i32) as i8,
    (83 as i32) as i8,
    (69 as i32) as i8,
    (88 as i32) as i8,
    (67 as i32) as i8,
    (76 as i32) as i8,
    (85 as i32) as i8,
    (68 as i32) as i8,
    (69 as i32) as i8,
    (76 as i32) as i8,
    (69 as i32) as i8,
    (84 as i32) as i8,
    (69 as i32) as i8,
    (77 as i32) as i8,
    (80 as i32) as i8,
    (79 as i32) as i8,
    (82 as i32) as i8,
    (65 as i32) as i8,
    (82 as i32) as i8,
    (89 as i32) as i8,
    (73 as i32) as i8,
    (83 as i32) as i8,
    (78 as i32) as i8,
    (85 as i32) as i8,
    (76 as i32) as i8,
    (76 as i32) as i8,
    (83 as i32) as i8,
    (65 as i32) as i8,
    (86 as i32) as i8,
    (69 as i32) as i8,
    (80 as i32) as i8,
    (79 as i32) as i8,
    (73 as i32) as i8,
    (78 as i32) as i8,
    (84 as i32) as i8,
    (69 as i32) as i8,
    (82 as i32) as i8,
    (83 as i32) as i8,
    (69 as i32) as i8,
    (67 as i32) as i8,
    (84 as i32) as i8,
    (73 as i32) as i8,
    (69 as i32) as i8,
    (83 as i32) as i8,
    (78 as i32) as i8,
    (79 as i32) as i8,
    (84 as i32) as i8,
    (78 as i32) as i8,
    (85 as i32) as i8,
    (76 as i32) as i8,
    (76 as i32) as i8,
    (73 as i32) as i8,
    (75 as i32) as i8,
    (69 as i32) as i8,
    (88 as i32) as i8,
    (67 as i32) as i8,
    (69 as i32) as i8,
    (80 as i32) as i8,
    (84 as i32) as i8,
    (82 as i32) as i8,
    (65 as i32) as i8,
    (78 as i32) as i8,
    (83 as i32) as i8,
    (65 as i32) as i8,
    (67 as i32) as i8,
    (84 as i32) as i8,
    (73 as i32) as i8,
    (79 as i32) as i8,
    (78 as i32) as i8,
    (65 as i32) as i8,
    (84 as i32) as i8,
    (85 as i32) as i8,
    (82 as i32) as i8,
    (65 as i32) as i8,
    (76 as i32) as i8,
    (84 as i32) as i8,
    (69 as i32) as i8,
    (82 as i32) as i8,
    (65 as i32) as i8,
    (73 as i32) as i8,
    (83 as i32) as i8,
    (69 as i32) as i8,
    (88 as i32) as i8,
    (67 as i32) as i8,
    (76 as i32) as i8,
    (85 as i32) as i8,
    (83 as i32) as i8,
    (73 as i32) as i8,
    (86 as i32) as i8,
    (69 as i32) as i8,
    (88 as i32) as i8,
    (73 as i32) as i8,
    (83 as i32) as i8,
    (84 as i32) as i8,
    (83 as i32) as i8,
    (67 as i32) as i8,
    (79 as i32) as i8,
    (78 as i32) as i8,
    (83 as i32) as i8,
    (84 as i32) as i8,
    (82 as i32) as i8,
    (65 as i32) as i8,
    (73 as i32) as i8,
    (78 as i32) as i8,
    (84 as i32) as i8,
    (79 as i32) as i8,
    (70 as i32) as i8,
    (70 as i32) as i8,
    (83 as i32) as i8,
    (69 as i32) as i8,
    (84 as i32) as i8,
    (82 as i32) as i8,
    (73 as i32) as i8,
    (71 as i32) as i8,
    (71 as i32) as i8,
    (69 as i32) as i8,
    (82 as i32) as i8,
    (65 as i32) as i8,
    (78 as i32) as i8,
    (71 as i32) as i8,
    (69 as i32) as i8,
    (78 as i32) as i8,
    (69 as i32) as i8,
    (82 as i32) as i8,
    (65 as i32) as i8,
    (84 as i32) as i8,
    (69 as i32) as i8,
    (68 as i32) as i8,
    (69 as i32) as i8,
    (84 as i32) as i8,
    (65 as i32) as i8,
    (67 as i32) as i8,
    (72 as i32) as i8,
    (65 as i32) as i8,
    (86 as i32) as i8,
    (73 as i32) as i8,
    (78 as i32) as i8,
    (71 as i32) as i8,
    (76 as i32) as i8,
    (79 as i32) as i8,
    (66 as i32) as i8,
    (69 as i32) as i8,
    (71 as i32) as i8,
    (73 as i32) as i8,
    (78 as i32) as i8,
    (78 as i32) as i8,
    (69 as i32) as i8,
    (82 as i32) as i8,
    (69 as i32) as i8,
    (70 as i32) as i8,
    (69 as i32) as i8,
    (82 as i32) as i8,
    (69 as i32) as i8,
    (78 as i32) as i8,
    (67 as i32) as i8,
    (69 as i32) as i8,
    (83 as i32) as i8,
    (85 as i32) as i8,
    (78 as i32) as i8,
    (73 as i32) as i8,
    (81 as i32) as i8,
    (85 as i32) as i8,
    (69 as i32) as i8,
    (82 as i32) as i8,
    (89 as i32) as i8,
    (87 as i32) as i8,
    (73 as i32) as i8,
    (84 as i32) as i8,
    (72 as i32) as i8,
    (79 as i32) as i8,
    (85 as i32) as i8,
    (84 as i32) as i8,
    (69 as i32) as i8,
    (82 as i32) as i8,
    (69 as i32) as i8,
    (76 as i32) as i8,
    (69 as i32) as i8,
    (65 as i32) as i8,
    (83 as i32) as i8,
    (69 as i32) as i8,
    (65 as i32) as i8,
    (84 as i32) as i8,
    (84 as i32) as i8,
    (65 as i32) as i8,
    (67 as i32) as i8,
    (72 as i32) as i8,
    (66 as i32) as i8,
    (69 as i32) as i8,
    (84 as i32) as i8,
    (87 as i32) as i8,
    (69 as i32) as i8,
    (69 as i32) as i8,
    (78 as i32) as i8,
    (79 as i32) as i8,
    (84 as i32) as i8,
    (72 as i32) as i8,
    (73 as i32) as i8,
    (78 as i32) as i8,
    (71 as i32) as i8,
    (82 as i32) as i8,
    (79 as i32) as i8,
    (85 as i32) as i8,
    (80 as i32) as i8,
    (83 as i32) as i8,
    (67 as i32) as i8,
    (65 as i32) as i8,
    (83 as i32) as i8,
    (67 as i32) as i8,
    (65 as i32) as i8,
    (68 as i32) as i8,
    (69 as i32) as i8,
    (70 as i32) as i8,
    (65 as i32) as i8,
    (85 as i32) as i8,
    (76 as i32) as i8,
    (84 as i32) as i8,
    (67 as i32) as i8,
    (65 as i32) as i8,
    (83 as i32) as i8,
    (69 as i32) as i8,
    (67 as i32) as i8,
    (79 as i32) as i8,
    (76 as i32) as i8,
    (76 as i32) as i8,
    (65 as i32) as i8,
    (84 as i32) as i8,
    (69 as i32) as i8,
    (67 as i32) as i8,
    (82 as i32) as i8,
    (69 as i32) as i8,
    (65 as i32) as i8,
    (84 as i32) as i8,
    (69 as i32) as i8,
    (67 as i32) as i8,
    (85 as i32) as i8,
    (82 as i32) as i8,
    (82 as i32) as i8,
    (69 as i32) as i8,
    (78 as i32) as i8,
    (84 as i32) as i8,
    (95 as i32) as i8,
    (68 as i32) as i8,
    (65 as i32) as i8,
    (84 as i32) as i8,
    (69 as i32) as i8,
    (73 as i32) as i8,
    (77 as i32) as i8,
    (77 as i32) as i8,
    (69 as i32) as i8,
    (68 as i32) as i8,
    (73 as i32) as i8,
    (65 as i32) as i8,
    (84 as i32) as i8,
    (69 as i32) as i8,
    (74 as i32) as i8,
    (79 as i32) as i8,
    (73 as i32) as i8,
    (78 as i32) as i8,
    (83 as i32) as i8,
    (69 as i32) as i8,
    (82 as i32) as i8,
    (84 as i32) as i8,
    (77 as i32) as i8,
    (65 as i32) as i8,
    (84 as i32) as i8,
    (67 as i32) as i8,
    (72 as i32) as i8,
    (80 as i32) as i8,
    (76 as i32) as i8,
    (65 as i32) as i8,
    (78 as i32) as i8,
    (65 as i32) as i8,
    (76 as i32) as i8,
    (89 as i32) as i8,
    (90 as i32) as i8,
    (69 as i32) as i8,
    (80 as i32) as i8,
    (82 as i32) as i8,
    (65 as i32) as i8,
    (71 as i32) as i8,
    (77 as i32) as i8,
    (65 as i32) as i8,
    (84 as i32) as i8,
    (69 as i32) as i8,
    (82 as i32) as i8,
    (73 as i32) as i8,
    (65 as i32) as i8,
    (76 as i32) as i8,
    (73 as i32) as i8,
    (90 as i32) as i8,
    (69 as i32) as i8,
    (68 as i32) as i8,
    (69 as i32) as i8,
    (70 as i32) as i8,
    (69 as i32) as i8,
    (82 as i32) as i8,
    (82 as i32) as i8,
    (69 as i32) as i8,
    (68 as i32) as i8,
    (73 as i32) as i8,
    (83 as i32) as i8,
    (84 as i32) as i8,
    (73 as i32) as i8,
    (78 as i32) as i8,
    (67 as i32) as i8,
    (84 as i32) as i8,
    (85 as i32) as i8,
    (80 as i32) as i8,
    (68 as i32) as i8,
    (65 as i32) as i8,
    (84 as i32) as i8,
    (69 as i32) as i8,
    (86 as i32) as i8,
    (65 as i32) as i8,
    (76 as i32) as i8,
    (85 as i32) as i8,
    (69 as i32) as i8,
    (83 as i32) as i8,
    (86 as i32) as i8,
    (73 as i32) as i8,
    (82 as i32) as i8,
    (84 as i32) as i8,
    (85 as i32) as i8,
    (65 as i32) as i8,
    (76 as i32) as i8,
    (87 as i32) as i8,
    (65 as i32) as i8,
    (89 as i32) as i8,
    (83 as i32) as i8,
    (87 as i32) as i8,
    (72 as i32) as i8,
    (69 as i32) as i8,
    (78 as i32) as i8,
    (87 as i32) as i8,
    (72 as i32) as i8,
    (69 as i32) as i8,
    (82 as i32) as i8,
    (69 as i32) as i8,
    (67 as i32) as i8,
    (85 as i32) as i8,
    (82 as i32) as i8,
    (83 as i32) as i8,
    (73 as i32) as i8,
    (86 as i32) as i8,
    (69 as i32) as i8,
    (65 as i32) as i8,
    (66 as i32) as i8,
    (79 as i32) as i8,
    (82 as i32) as i8,
    (84 as i32) as i8,
    (65 as i32) as i8,
    (70 as i32) as i8,
    (84 as i32) as i8,
    (69 as i32) as i8,
    (82 as i32) as i8,
    (69 as i32) as i8,
    (78 as i32) as i8,
    (65 as i32) as i8,
    (77 as i32) as i8,
    (69 as i32) as i8,
    (65 as i32) as i8,
    (78 as i32) as i8,
    (68 as i32) as i8,
    (82 as i32) as i8,
    (79 as i32) as i8,
    (80 as i32) as i8,
    (65 as i32) as i8,
    (82 as i32) as i8,
    (84 as i32) as i8,
    (73 as i32) as i8,
    (84 as i32) as i8,
    (73 as i32) as i8,
    (79 as i32) as i8,
    (78 as i32) as i8,
    (65 as i32) as i8,
    (85 as i32) as i8,
    (84 as i32) as i8,
    (79 as i32) as i8,
    (73 as i32) as i8,
    (78 as i32) as i8,
    (67 as i32) as i8,
    (82 as i32) as i8,
    (69 as i32) as i8,
    (77 as i32) as i8,
    (69 as i32) as i8,
    (78 as i32) as i8,
    (84 as i32) as i8,
    (67 as i32) as i8,
    (65 as i32) as i8,
    (83 as i32) as i8,
    (84 as i32) as i8,
    (67 as i32) as i8,
    (79 as i32) as i8,
    (76 as i32) as i8,
    (85 as i32) as i8,
    (77 as i32) as i8,
    (78 as i32) as i8,
    (67 as i32) as i8,
    (79 as i32) as i8,
    (77 as i32) as i8,
    (77 as i32) as i8,
    (73 as i32) as i8,
    (84 as i32) as i8,
    (67 as i32) as i8,
    (79 as i32) as i8,
    (78 as i32) as i8,
    (70 as i32) as i8,
    (76 as i32) as i8,
    (73 as i32) as i8,
    (67 as i32) as i8,
    (84 as i32) as i8,
    (67 as i32) as i8,
    (82 as i32) as i8,
    (79 as i32) as i8,
    (83 as i32) as i8,
    (83 as i32) as i8,
    (67 as i32) as i8,
    (85 as i32) as i8,
    (82 as i32) as i8,
    (82 as i32) as i8,
    (69 as i32) as i8,
    (78 as i32) as i8,
    (84 as i32) as i8,
    (95 as i32) as i8,
    (84 as i32) as i8,
    (73 as i32) as i8,
    (77 as i32) as i8,
    (69 as i32) as i8,
    (83 as i32) as i8,
    (84 as i32) as i8,
    (65 as i32) as i8,
    (77 as i32) as i8,
    (80 as i32) as i8,
    (82 as i32) as i8,
    (69 as i32) as i8,
    (67 as i32) as i8,
    (69 as i32) as i8,
    (68 as i32) as i8,
    (73 as i32) as i8,
    (78 as i32) as i8,
    (71 as i32) as i8,
    (70 as i32) as i8,
    (65 as i32) as i8,
    (73 as i32) as i8,
    (76 as i32) as i8,
    (65 as i32) as i8,
    (83 as i32) as i8,
    (84 as i32) as i8,
    (70 as i32) as i8,
    (73 as i32) as i8,
    (76 as i32) as i8,
    (84 as i32) as i8,
    (69 as i32) as i8,
    (82 as i32) as i8,
    (69 as i32) as i8,
    (80 as i32) as i8,
    (76 as i32) as i8,
    (65 as i32) as i8,
    (67 as i32) as i8,
    (69 as i32) as i8,
    (70 as i32) as i8,
    (73 as i32) as i8,
    (82 as i32) as i8,
    (83 as i32) as i8,
    (84 as i32) as i8,
    (70 as i32) as i8,
    (79 as i32) as i8,
    (76 as i32) as i8,
    (76 as i32) as i8,
    (79 as i32) as i8,
    (87 as i32) as i8,
    (73 as i32) as i8,
    (78 as i32) as i8,
    (71 as i32) as i8,
    (70 as i32) as i8,
    (82 as i32) as i8,
    (79 as i32) as i8,
    (77 as i32) as i8,
    (70 as i32) as i8,
    (85 as i32) as i8,
    (76 as i32) as i8,
    (76 as i32) as i8,
    (73 as i32) as i8,
    (77 as i32) as i8,
    (73 as i32) as i8,
    (84 as i32) as i8,
    (73 as i32) as i8,
    (70 as i32) as i8,
    (79 as i32) as i8,
    (82 as i32) as i8,
    (68 as i32) as i8,
    (69 as i32) as i8,
    (82 as i32) as i8,
    (69 as i32) as i8,
    (83 as i32) as i8,
    (84 as i32) as i8,
    (82 as i32) as i8,
    (73 as i32) as i8,
    (67 as i32) as i8,
    (84 as i32) as i8,
    (79 as i32) as i8,
    (84 as i32) as i8,
    (72 as i32) as i8,
    (69 as i32) as i8,
    (82 as i32) as i8,
    (83 as i32) as i8,
    (79 as i32) as i8,
    (86 as i32) as i8,
    (69 as i32) as i8,
    (82 as i32) as i8,
    (69 as i32) as i8,
    (84 as i32) as i8,
    (85 as i32) as i8,
    (82 as i32) as i8,
    (78 as i32) as i8,
    (73 as i32) as i8,
    (78 as i32) as i8,
    (71 as i32) as i8,
    (82 as i32) as i8,
    (73 as i32) as i8,
    (71 as i32) as i8,
    (72 as i32) as i8,
    (84 as i32) as i8,
    (82 as i32) as i8,
    (79 as i32) as i8,
    (76 as i32) as i8,
    (76 as i32) as i8,
    (66 as i32) as i8,
    (65 as i32) as i8,
    (67 as i32) as i8,
    (75 as i32) as i8,
    (82 as i32) as i8,
    (79 as i32) as i8,
    (87 as i32) as i8,
    (83 as i32) as i8,
    (85 as i32) as i8,
    (78 as i32) as i8,
    (66 as i32) as i8,
    (79 as i32) as i8,
    (85 as i32) as i8,
    (78 as i32) as i8,
    (68 as i32) as i8,
    (69 as i32) as i8,
    (68 as i32) as i8,
    (85 as i32) as i8,
    (78 as i32) as i8,
    (73 as i32) as i8,
    (79 as i32) as i8,
    (78 as i32) as i8,
    (85 as i32) as i8,
    (83 as i32) as i8,
    (73 as i32) as i8,
    (78 as i32) as i8,
    (71 as i32) as i8,
    (86 as i32) as i8,
    (65 as i32) as i8,
    (67 as i32) as i8,
    (85 as i32) as i8,
    (85 as i32) as i8,
    (77 as i32) as i8,
    (86 as i32) as i8,
    (73 as i32) as i8,
    (69 as i32) as i8,
    (87 as i32) as i8,
    (73 as i32) as i8,
    (78 as i32) as i8,
    (68 as i32) as i8,
    (79 as i32) as i8,
    (87 as i32) as i8,
    (66 as i32) as i8,
    (89 as i32) as i8,
    (73 as i32) as i8,
    (78 as i32) as i8,
    (73 as i32) as i8,
    (84 as i32) as i8,
    (73 as i32) as i8,
    (65 as i32) as i8,
    (76 as i32) as i8,
    (76 as i32) as i8,
    (89 as i32) as i8,
    (80 as i32) as i8,
    (82 as i32) as i8,
    (73 as i32) as i8,
    (77 as i32) as i8,
    (65 as i32) as i8,
    (82 as i32) as i8,
    (89 as i32) as i8,
]);

static mut aKWHash: __SlateAlign16<[u8; 127]> = __SlateAlign16([
    ((84 as i32) as i8) as u8,
    ((92 as i32) as i8) as u8,
    ((134 as i32) as i8) as u8,
    ((82 as i32) as i8) as u8,
    ((105 as i32) as i8) as u8,
    ((29 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((94 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((85 as i32) as i8) as u8,
    ((72 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((53 as i32) as i8) as u8,
    ((35 as i32) as i8) as u8,
    ((86 as i32) as i8) as u8,
    ((15 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((42 as i32) as i8) as u8,
    ((97 as i32) as i8) as u8,
    ((54 as i32) as i8) as u8,
    ((89 as i32) as i8) as u8,
    ((135 as i32) as i8) as u8,
    ((19 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((140 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((40 as i32) as i8) as u8,
    ((129 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((22 as i32) as i8) as u8,
    ((107 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((9 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((123 as i32) as i8) as u8,
    ((80 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((78 as i32) as i8) as u8,
    ((6 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((65 as i32) as i8) as u8,
    ((103 as i32) as i8) as u8,
    ((147 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((136 as i32) as i8) as u8,
    ((115 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((48 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((90 as i32) as i8) as u8,
    ((24 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((17 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((27 as i32) as i8) as u8,
    ((70 as i32) as i8) as u8,
    ((23 as i32) as i8) as u8,
    ((26 as i32) as i8) as u8,
    ((5 as i32) as i8) as u8,
    ((60 as i32) as i8) as u8,
    ((142 as i32) as i8) as u8,
    ((110 as i32) as i8) as u8,
    ((122 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((73 as i32) as i8) as u8,
    ((91 as i32) as i8) as u8,
    ((71 as i32) as i8) as u8,
    ((145 as i32) as i8) as u8,
    ((61 as i32) as i8) as u8,
    ((120 as i32) as i8) as u8,
    ((74 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((49 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((11 as i32) as i8) as u8,
    ((41 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((113 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((109 as i32) as i8) as u8,
    ((10 as i32) as i8) as u8,
    ((111 as i32) as i8) as u8,
    ((116 as i32) as i8) as u8,
    ((125 as i32) as i8) as u8,
    ((14 as i32) as i8) as u8,
    ((50 as i32) as i8) as u8,
    ((124 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((100 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((18 as i32) as i8) as u8,
    ((121 as i32) as i8) as u8,
    ((144 as i32) as i8) as u8,
    ((56 as i32) as i8) as u8,
    ((130 as i32) as i8) as u8,
    ((139 as i32) as i8) as u8,
    ((88 as i32) as i8) as u8,
    ((83 as i32) as i8) as u8,
    ((37 as i32) as i8) as u8,
    ((30 as i32) as i8) as u8,
    ((126 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((108 as i32) as i8) as u8,
    ((51 as i32) as i8) as u8,
    ((131 as i32) as i8) as u8,
    ((128 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((34 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((132 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((98 as i32) as i8) as u8,
    ((38 as i32) as i8) as u8,
    ((39 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((20 as i32) as i8) as u8,
    ((45 as i32) as i8) as u8,
    ((117 as i32) as i8) as u8,
    ((93 as i32) as i8) as u8,
]);

static mut aKWNext: __SlateAlign16<[u8; 148]> = __SlateAlign16([
    ((0 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((4 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((43 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((106 as i32) as i8) as u8,
    ((114 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((2 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((143 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((13 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((141 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((119 as i32) as i8) as u8,
    ((52 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((137 as i32) as i8) as u8,
    ((12 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((62 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((138 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((133 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((36 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((28 as i32) as i8) as u8,
    ((77 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((59 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((47 as i32) as i8) as u8,
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
    ((69 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((146 as i32) as i8) as u8,
    ((3 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((58 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((1 as i32) as i8) as u8,
    ((75 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((31 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((127 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((104 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((64 as i32) as i8) as u8,
    ((66 as i32) as i8) as u8,
    ((63 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((46 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((16 as i32) as i8) as u8,
    ((8 as i32) as i8) as u8,
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
    ((81 as i32) as i8) as u8,
    ((101 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((112 as i32) as i8) as u8,
    ((21 as i32) as i8) as u8,
    ((7 as i32) as i8) as u8,
    ((67 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((79 as i32) as i8) as u8,
    ((96 as i32) as i8) as u8,
    ((118 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((68 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((99 as i32) as i8) as u8,
    ((44 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((55 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((76 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((95 as i32) as i8) as u8,
    ((32 as i32) as i8) as u8,
    ((33 as i32) as i8) as u8,
    ((57 as i32) as i8) as u8,
    ((25 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((102 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((87 as i32) as i8) as u8,
]);

static mut aKWLen: __SlateAlign16<[u8; 148]> = __SlateAlign16([
    ((0 as i32) as i8) as u8,
    ((7 as i32) as i8) as u8,
    ((7 as i32) as i8) as u8,
    ((5 as i32) as i8) as u8,
    ((4 as i32) as i8) as u8,
    ((6 as i32) as i8) as u8,
    ((4 as i32) as i8) as u8,
    ((5 as i32) as i8) as u8,
    ((3 as i32) as i8) as u8,
    ((6 as i32) as i8) as u8,
    ((7 as i32) as i8) as u8,
    ((3 as i32) as i8) as u8,
    ((6 as i32) as i8) as u8,
    ((6 as i32) as i8) as u8,
    ((7 as i32) as i8) as u8,
    ((7 as i32) as i8) as u8,
    ((3 as i32) as i8) as u8,
    ((8 as i32) as i8) as u8,
    ((2 as i32) as i8) as u8,
    ((6 as i32) as i8) as u8,
    ((5 as i32) as i8) as u8,
    ((4 as i32) as i8) as u8,
    ((4 as i32) as i8) as u8,
    ((3 as i32) as i8) as u8,
    ((10 as i32) as i8) as u8,
    ((4 as i32) as i8) as u8,
    ((7 as i32) as i8) as u8,
    ((6 as i32) as i8) as u8,
    ((9 as i32) as i8) as u8,
    ((4 as i32) as i8) as u8,
    ((2 as i32) as i8) as u8,
    ((6 as i32) as i8) as u8,
    ((5 as i32) as i8) as u8,
    ((9 as i32) as i8) as u8,
    ((9 as i32) as i8) as u8,
    ((4 as i32) as i8) as u8,
    ((7 as i32) as i8) as u8,
    ((3 as i32) as i8) as u8,
    ((2 as i32) as i8) as u8,
    ((4 as i32) as i8) as u8,
    ((4 as i32) as i8) as u8,
    ((6 as i32) as i8) as u8,
    ((11 as i32) as i8) as u8,
    ((6 as i32) as i8) as u8,
    ((2 as i32) as i8) as u8,
    ((7 as i32) as i8) as u8,
    ((5 as i32) as i8) as u8,
    ((5 as i32) as i8) as u8,
    ((9 as i32) as i8) as u8,
    ((6 as i32) as i8) as u8,
    ((10 as i32) as i8) as u8,
    ((4 as i32) as i8) as u8,
    ((6 as i32) as i8) as u8,
    ((2 as i32) as i8) as u8,
    ((3 as i32) as i8) as u8,
    ((7 as i32) as i8) as u8,
    ((5 as i32) as i8) as u8,
    ((9 as i32) as i8) as u8,
    ((6 as i32) as i8) as u8,
    ((6 as i32) as i8) as u8,
    ((4 as i32) as i8) as u8,
    ((5 as i32) as i8) as u8,
    ((5 as i32) as i8) as u8,
    ((10 as i32) as i8) as u8,
    ((6 as i32) as i8) as u8,
    ((5 as i32) as i8) as u8,
    ((7 as i32) as i8) as u8,
    ((4 as i32) as i8) as u8,
    ((5 as i32) as i8) as u8,
    ((7 as i32) as i8) as u8,
    ((6 as i32) as i8) as u8,
    ((7 as i32) as i8) as u8,
    ((7 as i32) as i8) as u8,
    ((6 as i32) as i8) as u8,
    ((5 as i32) as i8) as u8,
    ((7 as i32) as i8) as u8,
    ((3 as i32) as i8) as u8,
    ((7 as i32) as i8) as u8,
    ((4 as i32) as i8) as u8,
    ((7 as i32) as i8) as u8,
    ((6 as i32) as i8) as u8,
    ((12 as i32) as i8) as u8,
    ((9 as i32) as i8) as u8,
    ((4 as i32) as i8) as u8,
    ((6 as i32) as i8) as u8,
    ((5 as i32) as i8) as u8,
    ((4 as i32) as i8) as u8,
    ((7 as i32) as i8) as u8,
    ((6 as i32) as i8) as u8,
    ((12 as i32) as i8) as u8,
    ((8 as i32) as i8) as u8,
    ((8 as i32) as i8) as u8,
    ((2 as i32) as i8) as u8,
    ((6 as i32) as i8) as u8,
    ((6 as i32) as i8) as u8,
    ((7 as i32) as i8) as u8,
    ((6 as i32) as i8) as u8,
    ((4 as i32) as i8) as u8,
    ((5 as i32) as i8) as u8,
    ((9 as i32) as i8) as u8,
    ((5 as i32) as i8) as u8,
    ((5 as i32) as i8) as u8,
    ((6 as i32) as i8) as u8,
    ((3 as i32) as i8) as u8,
    ((4 as i32) as i8) as u8,
    ((9 as i32) as i8) as u8,
    ((13 as i32) as i8) as u8,
    ((2 as i32) as i8) as u8,
    ((2 as i32) as i8) as u8,
    ((4 as i32) as i8) as u8,
    ((6 as i32) as i8) as u8,
    ((6 as i32) as i8) as u8,
    ((8 as i32) as i8) as u8,
    ((5 as i32) as i8) as u8,
    ((17 as i32) as i8) as u8,
    ((12 as i32) as i8) as u8,
    ((7 as i32) as i8) as u8,
    ((9 as i32) as i8) as u8,
    ((4 as i32) as i8) as u8,
    ((4 as i32) as i8) as u8,
    ((6 as i32) as i8) as u8,
    ((7 as i32) as i8) as u8,
    ((5 as i32) as i8) as u8,
    ((9 as i32) as i8) as u8,
    ((4 as i32) as i8) as u8,
    ((4 as i32) as i8) as u8,
    ((5 as i32) as i8) as u8,
    ((2 as i32) as i8) as u8,
    ((5 as i32) as i8) as u8,
    ((8 as i32) as i8) as u8,
    ((6 as i32) as i8) as u8,
    ((4 as i32) as i8) as u8,
    ((9 as i32) as i8) as u8,
    ((5 as i32) as i8) as u8,
    ((8 as i32) as i8) as u8,
    ((4 as i32) as i8) as u8,
    ((3 as i32) as i8) as u8,
    ((9 as i32) as i8) as u8,
    ((5 as i32) as i8) as u8,
    ((5 as i32) as i8) as u8,
    ((6 as i32) as i8) as u8,
    ((4 as i32) as i8) as u8,
    ((6 as i32) as i8) as u8,
    ((2 as i32) as i8) as u8,
    ((2 as i32) as i8) as u8,
    ((9 as i32) as i8) as u8,
    ((3 as i32) as i8) as u8,
    ((7 as i32) as i8) as u8,
]);

static mut aKWOffset: __SlateAlign16<[u16; 148]> = __SlateAlign16([
    ((0 as i32) as i16) as u16,
    ((0 as i32) as i16) as u16,
    ((2 as i32) as i16) as u16,
    ((2 as i32) as i16) as u16,
    ((8 as i32) as i16) as u16,
    ((9 as i32) as i16) as u16,
    ((14 as i32) as i16) as u16,
    ((16 as i32) as i16) as u16,
    ((20 as i32) as i16) as u16,
    ((23 as i32) as i16) as u16,
    ((25 as i32) as i16) as u16,
    ((25 as i32) as i16) as u16,
    ((29 as i32) as i16) as u16,
    ((33 as i32) as i16) as u16,
    ((36 as i32) as i16) as u16,
    ((41 as i32) as i16) as u16,
    ((46 as i32) as i16) as u16,
    ((48 as i32) as i16) as u16,
    ((53 as i32) as i16) as u16,
    ((54 as i32) as i16) as u16,
    ((59 as i32) as i16) as u16,
    ((62 as i32) as i16) as u16,
    ((65 as i32) as i16) as u16,
    ((67 as i32) as i16) as u16,
    ((69 as i32) as i16) as u16,
    ((78 as i32) as i16) as u16,
    ((81 as i32) as i16) as u16,
    ((86 as i32) as i16) as u16,
    ((90 as i32) as i16) as u16,
    ((90 as i32) as i16) as u16,
    ((94 as i32) as i16) as u16,
    ((99 as i32) as i16) as u16,
    ((101 as i32) as i16) as u16,
    ((105 as i32) as i16) as u16,
    ((111 as i32) as i16) as u16,
    ((119 as i32) as i16) as u16,
    ((123 as i32) as i16) as u16,
    ((123 as i32) as i16) as u16,
    ((123 as i32) as i16) as u16,
    ((126 as i32) as i16) as u16,
    ((129 as i32) as i16) as u16,
    ((132 as i32) as i16) as u16,
    ((137 as i32) as i16) as u16,
    ((142 as i32) as i16) as u16,
    ((146 as i32) as i16) as u16,
    ((147 as i32) as i16) as u16,
    ((152 as i32) as i16) as u16,
    ((156 as i32) as i16) as u16,
    ((160 as i32) as i16) as u16,
    ((168 as i32) as i16) as u16,
    ((174 as i32) as i16) as u16,
    ((181 as i32) as i16) as u16,
    ((184 as i32) as i16) as u16,
    ((184 as i32) as i16) as u16,
    ((187 as i32) as i16) as u16,
    ((189 as i32) as i16) as u16,
    ((195 as i32) as i16) as u16,
    ((198 as i32) as i16) as u16,
    ((206 as i32) as i16) as u16,
    ((211 as i32) as i16) as u16,
    ((216 as i32) as i16) as u16,
    ((219 as i32) as i16) as u16,
    ((222 as i32) as i16) as u16,
    ((226 as i32) as i16) as u16,
    ((236 as i32) as i16) as u16,
    ((239 as i32) as i16) as u16,
    ((244 as i32) as i16) as u16,
    ((244 as i32) as i16) as u16,
    ((248 as i32) as i16) as u16,
    ((252 as i32) as i16) as u16,
    ((259 as i32) as i16) as u16,
    ((265 as i32) as i16) as u16,
    ((271 as i32) as i16) as u16,
    ((277 as i32) as i16) as u16,
    ((277 as i32) as i16) as u16,
    ((283 as i32) as i16) as u16,
    ((284 as i32) as i16) as u16,
    ((288 as i32) as i16) as u16,
    ((295 as i32) as i16) as u16,
    ((299 as i32) as i16) as u16,
    ((306 as i32) as i16) as u16,
    ((312 as i32) as i16) as u16,
    ((324 as i32) as i16) as u16,
    ((333 as i32) as i16) as u16,
    ((335 as i32) as i16) as u16,
    ((341 as i32) as i16) as u16,
    ((346 as i32) as i16) as u16,
    ((348 as i32) as i16) as u16,
    ((355 as i32) as i16) as u16,
    ((359 as i32) as i16) as u16,
    ((370 as i32) as i16) as u16,
    ((377 as i32) as i16) as u16,
    ((378 as i32) as i16) as u16,
    ((385 as i32) as i16) as u16,
    ((391 as i32) as i16) as u16,
    ((397 as i32) as i16) as u16,
    ((402 as i32) as i16) as u16,
    ((408 as i32) as i16) as u16,
    ((412 as i32) as i16) as u16,
    ((415 as i32) as i16) as u16,
    ((424 as i32) as i16) as u16,
    ((429 as i32) as i16) as u16,
    ((433 as i32) as i16) as u16,
    ((439 as i32) as i16) as u16,
    ((441 as i32) as i16) as u16,
    ((444 as i32) as i16) as u16,
    ((453 as i32) as i16) as u16,
    ((455 as i32) as i16) as u16,
    ((457 as i32) as i16) as u16,
    ((466 as i32) as i16) as u16,
    ((470 as i32) as i16) as u16,
    ((476 as i32) as i16) as u16,
    ((482 as i32) as i16) as u16,
    ((490 as i32) as i16) as u16,
    ((495 as i32) as i16) as u16,
    ((495 as i32) as i16) as u16,
    ((495 as i32) as i16) as u16,
    ((511 as i32) as i16) as u16,
    ((520 as i32) as i16) as u16,
    ((523 as i32) as i16) as u16,
    ((527 as i32) as i16) as u16,
    ((532 as i32) as i16) as u16,
    ((539 as i32) as i16) as u16,
    ((544 as i32) as i16) as u16,
    ((553 as i32) as i16) as u16,
    ((557 as i32) as i16) as u16,
    ((560 as i32) as i16) as u16,
    ((565 as i32) as i16) as u16,
    ((567 as i32) as i16) as u16,
    ((571 as i32) as i16) as u16,
    ((579 as i32) as i16) as u16,
    ((585 as i32) as i16) as u16,
    ((588 as i32) as i16) as u16,
    ((597 as i32) as i16) as u16,
    ((602 as i32) as i16) as u16,
    ((610 as i32) as i16) as u16,
    ((610 as i32) as i16) as u16,
    ((614 as i32) as i16) as u16,
    ((623 as i32) as i16) as u16,
    ((628 as i32) as i16) as u16,
    ((633 as i32) as i16) as u16,
    ((639 as i32) as i16) as u16,
    ((642 as i32) as i16) as u16,
    ((645 as i32) as i16) as u16,
    ((648 as i32) as i16) as u16,
    ((650 as i32) as i16) as u16,
    ((655 as i32) as i16) as u16,
    ((659 as i32) as i16) as u16,
]);

static mut aKWCode: __SlateAlign16<[u8; 148]> = __SlateAlign16([
    ((0 as i32) as i8) as u8,
    ((99 as i32) as i8) as u8,
    ((117 as i32) as i8) as u8,
    ((162 as i32) as i8) as u8,
    ((39 as i32) as i8) as u8,
    ((59 as i32) as i8) as u8,
    ((41 as i32) as i8) as u8,
    ((125 as i32) as i8) as u8,
    ((68 as i32) as i8) as u8,
    ((33 as i32) as i8) as u8,
    ((133 as i32) as i8) as u8,
    ((63 as i32) as i8) as u8,
    ((64 as i32) as i8) as u8,
    ((48 as i32) as i8) as u8,
    ((2 as i32) as i8) as u8,
    ((66 as i32) as i8) as u8,
    ((164 as i32) as i8) as u8,
    ((38 as i32) as i8) as u8,
    ((24 as i32) as i8) as u8,
    ((139 as i32) as i8) as u8,
    ((16 as i32) as i8) as u8,
    ((119 as i32) as i8) as u8,
    ((160 as i32) as i8) as u8,
    ((11 as i32) as i8) as u8,
    ((132 as i32) as i8) as u8,
    ((161 as i32) as i8) as u8,
    ((92 as i32) as i8) as u8,
    ((129 as i32) as i8) as u8,
    ((21 as i32) as i8) as u8,
    ((21 as i32) as i8) as u8,
    ((43 as i32) as i8) as u8,
    ((51 as i32) as i8) as u8,
    ((83 as i32) as i8) as u8,
    ((13 as i32) as i8) as u8,
    ((138 as i32) as i8) as u8,
    ((95 as i32) as i8) as u8,
    ((52 as i32) as i8) as u8,
    ((19 as i32) as i8) as u8,
    ((67 as i32) as i8) as u8,
    ((122 as i32) as i8) as u8,
    ((48 as i32) as i8) as u8,
    ((137 as i32) as i8) as u8,
    ((6 as i32) as i8) as u8,
    ((28 as i32) as i8) as u8,
    ((116 as i32) as i8) as u8,
    ((119 as i32) as i8) as u8,
    ((163 as i32) as i8) as u8,
    ((72 as i32) as i8) as u8,
    ((9 as i32) as i8) as u8,
    ((20 as i32) as i8) as u8,
    ((120 as i32) as i8) as u8,
    ((152 as i32) as i8) as u8,
    ((70 as i32) as i8) as u8,
    ((69 as i32) as i8) as u8,
    ((131 as i32) as i8) as u8,
    ((78 as i32) as i8) as u8,
    ((90 as i32) as i8) as u8,
    ((96 as i32) as i8) as u8,
    ((40 as i32) as i8) as u8,
    ((148 as i32) as i8) as u8,
    ((48 as i32) as i8) as u8,
    ((5 as i32) as i8) as u8,
    ((119 as i32) as i8) as u8,
    ((126 as i32) as i8) as u8,
    ((124 as i32) as i8) as u8,
    ((3 as i32) as i8) as u8,
    ((26 as i32) as i8) as u8,
    ((82 as i32) as i8) as u8,
    ((119 as i32) as i8) as u8,
    ((14 as i32) as i8) as u8,
    ((32 as i32) as i8) as u8,
    ((49 as i32) as i8) as u8,
    ((153 as i32) as i8) as u8,
    ((93 as i32) as i8) as u8,
    ((147 as i32) as i8) as u8,
    ((35 as i32) as i8) as u8,
    ((31 as i32) as i8) as u8,
    ((121 as i32) as i8) as u8,
    ((158 as i32) as i8) as u8,
    ((114 as i32) as i8) as u8,
    ((17 as i32) as i8) as u8,
    ((101 as i32) as i8) as u8,
    ((8 as i32) as i8) as u8,
    ((144 as i32) as i8) as u8,
    ((128 as i32) as i8) as u8,
    ((47 as i32) as i8) as u8,
    ((4 as i32) as i8) as u8,
    ((30 as i32) as i8) as u8,
    ((71 as i32) as i8) as u8,
    ((98 as i32) as i8) as u8,
    ((7 as i32) as i8) as u8,
    ((141 as i32) as i8) as u8,
    ((45 as i32) as i8) as u8,
    ((130 as i32) as i8) as u8,
    ((140 as i32) as i8) as u8,
    ((81 as i32) as i8) as u8,
    ((97 as i32) as i8) as u8,
    ((159 as i32) as i8) as u8,
    ((150 as i32) as i8) as u8,
    ((73 as i32) as i8) as u8,
    ((27 as i32) as i8) as u8,
    ((29 as i32) as i8) as u8,
    ((100 as i32) as i8) as u8,
    ((44 as i32) as i8) as u8,
    ((134 as i32) as i8) as u8,
    ((88 as i32) as i8) as u8,
    ((127 as i32) as i8) as u8,
    ((15 as i32) as i8) as u8,
    ((50 as i32) as i8) as u8,
    ((36 as i32) as i8) as u8,
    ((61 as i32) as i8) as u8,
    ((10 as i32) as i8) as u8,
    ((37 as i32) as i8) as u8,
    ((119 as i32) as i8) as u8,
    ((101 as i32) as i8) as u8,
    ((101 as i32) as i8) as u8,
    ((86 as i32) as i8) as u8,
    ((89 as i32) as i8) as u8,
    ((42 as i32) as i8) as u8,
    ((85 as i32) as i8) as u8,
    ((167 as i32) as i8) as u8,
    ((74 as i32) as i8) as u8,
    ((84 as i32) as i8) as u8,
    ((87 as i32) as i8) as u8,
    ((143 as i32) as i8) as u8,
    ((119 as i32) as i8) as u8,
    ((149 as i32) as i8) as u8,
    ((18 as i32) as i8) as u8,
    ((146 as i32) as i8) as u8,
    ((75 as i32) as i8) as u8,
    ((94 as i32) as i8) as u8,
    ((166 as i32) as i8) as u8,
    ((151 as i32) as i8) as u8,
    ((119 as i32) as i8) as u8,
    ((12 as i32) as i8) as u8,
    ((77 as i32) as i8) as u8,
    ((76 as i32) as i8) as u8,
    ((91 as i32) as i8) as u8,
    ((135 as i32) as i8) as u8,
    ((145 as i32) as i8) as u8,
    ((79 as i32) as i8) as u8,
    ((80 as i32) as i8) as u8,
    ((165 as i32) as i8) as u8,
    ((62 as i32) as i8) as u8,
    ((34 as i32) as i8) as u8,
    ((65 as i32) as i8) as u8,
    ((136 as i32) as i8) as u8,
    ((123 as i32) as i8) as u8,
]);

#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.tokenize.sqlite3_keyword_count")]
extern "C-unwind" fn sqlite3_keyword_count() -> i32 {
    return 147 as i32;
}

#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.tokenize.sqlite3_keyword_name")]
extern "C-unwind" fn sqlite3_keyword_name(
    mut i: i32,
    mut pzName: *mut *const i8,
    mut pnName: *mut i32,
) -> i32 {
    if i < (0 as i32) || i >= (147 as i32) {
        return 1 as i32;
    }
    let __v403: i32 = i;
    let __v404: i32 = __v403 + (1 as i32);
    i = __v404;
    unsafe {
        *pzName = unsafe {
            unsafe { std::ptr::addr_of!(zKWText.0) as *const i8 }.offset(
                (((unsafe {
                    *unsafe {
                        unsafe { std::ptr::addr_of!(aKWOffset.0) as *const u16 }.offset(i as isize)
                    }
                }) as u32) as i32) as isize,
            )
        };
    }
    unsafe {
        *pnName = ((unsafe {
            *unsafe { unsafe { std::ptr::addr_of!(aKWLen.0) as *const u8 }.offset(i as isize) }
        }) as u32) as i32;
    }
    return 0 as i32;
}

#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.tokenize.sqlite3_keyword_check")]
extern "C-unwind" fn sqlite3_keyword_check(mut zName: *const i8, mut nName: i32) -> i32 {
    return ((60 as i32) != sqlite3KeywordCode(zName as *const u8, nName)) as i32;
}

// /*         x0  x1  x2  x3  x4  x5  x6  x7  x8  x9  xa  xb  xc  xd  xe  xf */
// /* 0x */
// /* 1x */
// /* 2x */
// /* 3x */
// /* 4x */
// /* 5x */
// /* 6x */
// /* 7x */
// /* 8x */
// /* 9x */
// /* Ax */
// /* Bx */
// /* Cx */
// /* Dx */
// /* Ex */
// /* Fx */
// /*
// ** The charMap() macro maps alphabetic characters (only) into their
// ** lower-case ASCII equivalent.  On ASCII machines, this is just
// ** an upper-to-lower case map.  On EBCDIC machines we also need
// ** to adjust the encoding.  The mapping is only valid for alphabetics
// ** which are the only characters for which this feature is used.
// **
// ** Used by keywordhash.h
// */
// /*
// ** The sqlite3KeywordCode function looks up an identifier to determine if
// ** it is a keyword.  If it is a keyword, the token code of that keyword is
// ** returned.  If the input is not a keyword, TK_ID is returned.
// **
// ** The implementation of this routine was generated by a program,
// ** mkkeywordhash.c, located in the tool subdirectory of the distribution.
// ** The output of the mkkeywordhash.c program is written into a file
// ** named keywordhash.h and then included into this source file by
// ** the #include below.
// */
// /*
// ** If X is a character that can be used in an identifier then
// ** IdChar(X) will be true.  Otherwise it is false.
// **
// ** For ASCII, any character with the high-order bit set is
// ** allowed in an identifier.  For 7-bit characters,
// ** sqlite3IsIdChar[X] must be 1.
// **
// ** For EBCDIC, the rules are more complex but have the same
// ** end result.
// **
// ** Ticket #1066.  the SQL standard does not allow '$' in the
// ** middle of identifiers.  But many SQL implementations do.
// ** SQLite will allow '$' in identifiers for compatibility.
// ** But the feature is undocumented.
// */
// /* Make the IdChar function accessible from ctime.c and alter.c */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3IsIdChar(mut c: u8) -> i32 {
    return ((((unsafe {
        *unsafe {
            unsafe { std::ptr::addr_of!(sqlite3CtypeMap) as *const u8 }
                .offset(((c as u32) as i32) as isize)
        }
    }) as u32) as i32)
        & (70 as i32)
        != (0 as i32)) as i32;
}

#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3KeywordCode(mut z: *const u8, mut n: i32) -> i32 {
    let mut id: i32 = 60 as i32;
    if n >= (2 as i32) {
        keywordCode(
            (z as *mut i8) as *const i8,
            n as i64,
            std::ptr::addr_of_mut!(id),
        );
    }
    return id;
}

// /*
// ** Run the parser on the given SQL string.
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3RunParser(mut pParse: *mut Parse, mut zSql: *const i8) -> i32 {
    // /* Number of errors encountered */
    let mut nErr: i32 = 0 as i32;
    // /* The LEMON-generated LALR(1) parser */
    let mut pEngine: *mut () = unsafe { std::mem::zeroed() };
    // /* Length of the next token token */
    let mut n: i64 = (0 as i32) as i64;
    // /* type of the next token */
    let mut tokenType: i32 = 0 as i32;
    // /* type of the previous token */
    let mut lastTokenParsed: i32 = -(1 as i32);
    // /* The database connection */
    let mut db: *mut sqlite3 = unsafe { (*pParse).db };
    // /* Max length of an SQL string */
    let mut mxSqlLen: i64 = 0 as i64;
    // /* Outer parse context, if any */
    let mut pParentParse: *mut Parse = std::ptr::null_mut::<Parse>();
    {}
    0 as i32;
    mxSqlLen = (unsafe {
        *unsafe { unsafe { (*db).aLimit.as_mut_ptr() as *mut i32 }.offset((1 as i32) as isize) }
    }) as i64;
    if (unsafe { (*db).nVdbeActive }) == (0 as i32) {
        unsafe {
            std::sync::atomic::AtomicI32::store_volatile(
                std::sync::atomic::AtomicI32::from_ptr_raw(
                    (unsafe { std::ptr::addr_of_mut!((*db).u1.isInterrupted) }) as *mut i32,
                ),
                0 as i32,
                std::sync::atomic::Ordering::Relaxed,
            )
        };
    }
    unsafe {
        (*pParse).rc = 0 as i32;
    }
    unsafe {
        (*pParse).zTail = zSql;
    }
    pEngine = unsafe {
        sqlite3ParserAlloc(
            unsafe {
                std::mem::transmute::<*const (), Option<unsafe extern "C-unwind" fn(u64) -> *mut ()>>(
                    sqlite3Malloc as *const (),
                )
            },
            pParse,
        )
    };
    if pEngine == std::ptr::null_mut::<()>() {
        unsafe { sqlite3OomFault(db) };
        return 7 as i32;
    }
    0 as i32;
    0 as i32;
    0 as i32;
    0 as i32;
    pParentParse = unsafe { (*db).pParse };
    unsafe {
        (*db).pParse = pParse;
    }
    '__slate_break_400: while (1 as i32) != (0 as i32) {
        '__slate_continue_400: {
            n = sqlite3GetToken(
                (zSql as *mut u8) as *const u8,
                std::ptr::addr_of_mut!(tokenType),
            );
            let __v405: i64 = mxSqlLen;
            let __v406: i64 = __v405 - n;
            mxSqlLen = __v406;
            if mxSqlLen < ((0 as i32) as i64) {
                unsafe {
                    (*pParse).rc = 18 as i32;
                }
                let __v407: *mut Parse = pParse;
                let __v408: i32 = unsafe { (*__v407).nErr };
                let __v409: i32 = __v408 + (1 as i32);
                unsafe {
                    (*__v407).nErr = __v409;
                }
                break '__slate_break_400;
            }
            if tokenType >= (165 as i32) {
                0 as i32;
                if (unsafe {
                    std::sync::atomic::AtomicI32::load_volatile(
                        std::sync::atomic::AtomicI32::from_ptr_raw(
                            (unsafe { std::ptr::addr_of_mut!((*db).u1.isInterrupted) }) as *mut i32,
                        ),
                        std::sync::atomic::Ordering::Relaxed,
                    )
                }) != (0 as i32)
                {
                    unsafe {
                        (*pParse).rc = 9 as i32;
                    }
                    let __v410: *mut Parse = pParse;
                    let __v411: i32 = unsafe { (*__v410).nErr };
                    let __v412: i32 = __v411 + (1 as i32);
                    unsafe {
                        (*__v410).nErr = __v412;
                    }
                    break '__slate_break_400;
                }
                if tokenType == (184 as i32) {
                    let __v413: *const i8 = zSql;
                    let __v414: *const i8 = unsafe { __v413.offset(n as isize) };
                    zSql = __v414;
                    break '__slate_continue_400;
                }
                if ((unsafe { *unsafe { zSql.offset((0 as i32) as isize) } }) as i32) == (0 as i32)
                {
                    // /* Upon reaching the end of input, call the parser two more times
                    //         ** with tokens TK_SEMI and 0, in that order. */
                    if lastTokenParsed == (1 as i32) {
                        tokenType = 0 as i32;
                    } else {
                        if lastTokenParsed == (0 as i32) {
                            break '__slate_break_400;
                        } else {
                            tokenType = 1 as i32;
                        }
                    }
                    n = (0 as i32) as i64;
                } else {
                    if tokenType == (165 as i32) {
                        0 as i32;
                        tokenType = analyzeWindowKeyword(
                            (unsafe { zSql.offset((6 as i32) as isize) }) as *const u8,
                        );
                    } else {
                        if tokenType == (166 as i32) {
                            0 as i32;
                            tokenType = analyzeOverKeyword(
                                (unsafe { zSql.offset((4 as i32) as isize) }) as *const u8,
                                lastTokenParsed,
                            );
                        } else {
                            if tokenType == (167 as i32) {
                                0 as i32;
                                tokenType = analyzeFilterKeyword(
                                    (unsafe { zSql.offset((6 as i32) as isize) }) as *const u8,
                                    lastTokenParsed,
                                );
                            // /* SQLITE_OMIT_WINDOWFUNC */
                            } else {
                                if tokenType == (185 as i32)
                                    && ((unsafe { (*db).init.busy }) != (0 as u8)
                                        || (unsafe { (*db).flags })
                                            & (((64 as i32) as i64) as u64) << (32 as i32)
                                            != (((0 as i32) as i64) as u64))
                                {
                                    // /* Ignore SQL comments if either (1) we are reparsing the schema or
                                    //         ** (2) SQLITE_DBCONFIG_ENABLE_COMMENTS is turned on (the default). */
                                    let __v415: *const i8 = zSql;
                                    let __v416: *const i8 = unsafe { __v415.offset(n as isize) };
                                    zSql = __v416;
                                    break '__slate_continue_400;
                                } else {
                                    if tokenType != (183 as i32) {
                                        let mut x: Token = unsafe { std::mem::zeroed() };
                                        x.z = zSql;
                                        x.n = (n as i32) as u32;
                                        unsafe {
                                            sqlite3ErrorMsg(
                                                pParse,
                                                (b"unrecognized token: \"%T\"\0".as_ptr()
                                                    as *mut i8)
                                                    as *const i8,
                                                std::ptr::addr_of_mut!(x),
                                            )
                                        };
                                        break '__slate_break_400;
                                    }
                                }
                            }
                        }
                    }
                }
            }
            unsafe {
                (*pParse).sLastToken.z = zSql;
            }
            unsafe {
                (*pParse).sLastToken.n = (n as i32) as u32;
            }
            unsafe { sqlite3Parser(pEngine, tokenType, unsafe { (*pParse).sLastToken }) };
            lastTokenParsed = tokenType;
            let __v417: *const i8 = zSql;
            let __v418: *const i8 = unsafe { __v417.offset(n as isize) };
            zSql = __v418;
            0 as i32;
            if (unsafe { (*pParse).rc }) != (0 as i32) {
                break '__slate_break_400;
            }
        }
    }
    0 as i32;
    unsafe {
        sqlite3ParserFree(pEngine, unsafe {
            std::mem::transmute::<*const (), Option<unsafe extern "C-unwind" fn(*mut ())>>(
                sqlite3_free as *const (),
            )
        })
    };
    if (unsafe { (*db).mallocFailed }) != (0 as u8) {
        unsafe {
            (*pParse).rc = 7 as i32;
        }
    }
    if (unsafe { (*pParse).zErrMsg }) != std::ptr::null_mut::<i8>()
        || (unsafe { (*pParse).rc }) != (0 as i32) && (unsafe { (*pParse).rc }) != (101 as i32)
    {
        if (unsafe { (*pParse).zErrMsg }) == std::ptr::null_mut::<i8>() {
            unsafe {
                (*pParse).zErrMsg = unsafe {
                    sqlite3DbStrDup(db, unsafe { sqlite3ErrStr(unsafe { (*pParse).rc }) })
                };
            }
        }
        if (((unsafe { (*pParse).prepFlags }) as u32) as i32) & (16 as i32) == (0 as i32) {
            unsafe {
                sqlite3_log(
                    unsafe { (*pParse).rc },
                    (b"%s in \"%s\"\0".as_ptr() as *mut i8) as *const i8,
                    unsafe { (*pParse).zErrMsg },
                    unsafe { (*pParse).zTail },
                )
            };
        }
        let __v419: i32 = nErr;
        let __v420: i32 = __v419 + (1 as i32);
        nErr = __v420;
    }
    unsafe {
        (*pParse).zTail = zSql;
    }
    unsafe { sqlite3_free((unsafe { (*pParse).apVtabLock }) as *mut ()) };
    if (unsafe { (*pParse).pNewTable }) != std::ptr::null_mut::<Table>()
        && !((((unsafe { (*pParse).eParseMode }) as u32) as i32) != (0 as i32))
    {
        // /* If the pParse->declareVtab flag is set, do not delete any table
        //     ** structure built up in pParse->pNewTable. The calling code (see vtab.c)
        //     ** will take responsibility for freeing the Table structure.
        //     */
        unsafe { sqlite3DeleteTable(db, unsafe { (*pParse).pNewTable }) };
    }
    if (unsafe { (*pParse).pNewTrigger }) != std::ptr::null_mut::<Trigger>()
        && !((((unsafe { (*pParse).eParseMode }) as u32) as i32) >= (2 as i32))
    {
        unsafe { sqlite3DeleteTrigger(db, unsafe { (*pParse).pNewTrigger }) };
    }
    if (unsafe { (*pParse).pVList }) != std::ptr::null_mut::<i32>() {
        unsafe { sqlite3DbNNFreeNN(db, (unsafe { (*pParse).pVList }) as *mut ()) };
    }
    unsafe {
        (*db).pParse = pParentParse;
    }
    0 as i32;
    return nErr;
}

// /* SQLITE_OMIT_WINDOWFUNC */
// /*
// ** Return the length (in bytes) of the token that begins at z[0].
// ** Store the token type in *tokenType before returning.
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3GetToken(mut z: *const u8, mut tokenType: *mut i32) -> i64 {
    let mut i: i64 = 0 as i64;
    let mut c: i32 = 0 as i32;
    // /* Switch on the character-class of the first byte
    //                           ** of the token. See the comment on the CC_ defines
    //                           ** above. */
    '__slate_break_382: {
        match ((unsafe {
            *unsafe {
                unsafe { std::ptr::addr_of!(aiClass.0) as *const u8 }
                    .offset((((unsafe { *z }) as u32) as i32) as isize)
            }
        }) as u32) as i32
        {
            7 => {
                {}
                {}
                {}
                {}
                {}
                i = (1 as i32) as i64;
                '__slate_break_383: loop {
                    if !((((unsafe {
                        *unsafe {
                            unsafe { std::ptr::addr_of!(sqlite3CtypeMap) as *const u8 }.offset(
                                (((unsafe { *unsafe { z.offset(i as isize) } }) as u32) as i32)
                                    as isize,
                            )
                        }
                    }) as u32) as i32)
                        & (1 as i32)
                        != (0 as i32))
                    {
                        break;
                    }
                    let __v421: i64 = i;
                    let __v422: i64 = __v421 + ((1 as i32) as i64);
                    i = __v422;
                }
                unsafe {
                    *tokenType = 184 as i32;
                }
                return i;
            }
            11 => {
                if (((unsafe { *unsafe { z.offset((1 as i32) as isize) } }) as u32) as i32)
                    == (45 as i32)
                {
                    i = (2 as i32) as i64;
                    '__slate_break_384: loop {
                        let __v423: i32 =
                            ((unsafe { *unsafe { z.offset(i as isize) } }) as u32) as i32;
                        c = __v423;
                        if !(__v423 != (0 as i32) && c != (10 as i32)) {
                            break;
                        }
                        let __v424: i64 = i;
                        let __v425: i64 = __v424 + ((1 as i32) as i64);
                        i = __v425;
                    }
                    unsafe {
                        *tokenType = 185 as i32;
                    }
                    return i;
                } else {
                    if (((unsafe { *unsafe { z.offset((1 as i32) as isize) } }) as u32) as i32)
                        == (62 as i32)
                    {
                        unsafe {
                            *tokenType = 113 as i32;
                        }
                        return ((2 as i32)
                            + (((((unsafe { *unsafe { z.offset((2 as i32) as isize) } }) as u32)
                                as i32)
                                == (62 as i32)) as i32)) as i64;
                    }
                }
                unsafe {
                    *tokenType = 108 as i32;
                }
                return (1 as i32) as i64;
            }
            17 => {
                unsafe {
                    *tokenType = 22 as i32;
                }
                return (1 as i32) as i64;
            }
            18 => {
                unsafe {
                    *tokenType = 23 as i32;
                }
                return (1 as i32) as i64;
            }
            19 => {
                unsafe {
                    *tokenType = 1 as i32;
                }
                return (1 as i32) as i64;
            }
            20 => {
                unsafe {
                    *tokenType = 107 as i32;
                }
                return (1 as i32) as i64;
            }
            21 => {
                unsafe {
                    *tokenType = 109 as i32;
                }
                return (1 as i32) as i64;
            }
            16 => {
                if (((unsafe { *unsafe { z.offset((1 as i32) as isize) } }) as u32) as i32)
                    != (42 as i32)
                    || (((unsafe { *unsafe { z.offset((2 as i32) as isize) } }) as u32) as i32)
                        == (0 as i32)
                {
                    unsafe {
                        *tokenType = 110 as i32;
                    }
                    return (1 as i32) as i64;
                }
                i = (3 as i32) as i64;
                let __v426: i32 =
                    ((unsafe { *unsafe { z.offset((2 as i32) as isize) } }) as u32) as i32;
                c = __v426;
                '__slate_break_385: loop {
                    let __v427: bool;
                    if c != (42 as i32)
                        || (((unsafe { *unsafe { z.offset(i as isize) } }) as u32) as i32)
                            != (47 as i32)
                    {
                        let __v428: i32 =
                            ((unsafe { *unsafe { z.offset(i as isize) } }) as u32) as i32;
                        c = __v428;
                        __v427 = __v428 != (0 as i32);
                    } else {
                        __v427 = false as bool;
                    }
                    if !__v427 {
                        break;
                    }
                    let __v429: i64 = i;
                    let __v430: i64 = __v429 + ((1 as i32) as i64);
                    i = __v430;
                }
                if c != (0 as i32) {
                    let __v431: i64 = i;
                    let __v432: i64 = __v431 + ((1 as i32) as i64);
                    i = __v432;
                }
                unsafe {
                    *tokenType = 185 as i32;
                }
                return i;
            }
            22 => {
                unsafe {
                    *tokenType = 111 as i32;
                }
                return (1 as i32) as i64;
            }
            14 => {
                unsafe {
                    *tokenType = 54 as i32;
                }
                return ((1 as i32)
                    + (((((unsafe { *unsafe { z.offset((1 as i32) as isize) } }) as u32) as i32)
                        == (61 as i32)) as i32)) as i64;
            }
            12 => {
                let __v433: i32 =
                    ((unsafe { *unsafe { z.offset((1 as i32) as isize) } }) as u32) as i32;
                c = __v433;
                if __v433 == (61 as i32) {
                    unsafe {
                        *tokenType = 56 as i32;
                    }
                    return (2 as i32) as i64;
                } else {
                    if c == (62 as i32) {
                        unsafe {
                            *tokenType = 53 as i32;
                        }
                        return (2 as i32) as i64;
                    } else {
                        if c == (60 as i32) {
                            unsafe {
                                *tokenType = 105 as i32;
                            }
                            return (2 as i32) as i64;
                        } else {
                            unsafe {
                                *tokenType = 57 as i32;
                            }
                            return (1 as i32) as i64;
                        }
                    }
                }
                let _v485: i32 =
                    ((unsafe { *unsafe { z.offset((1 as i32) as isize) } }) as u32) as i32;
                c = _v485;
                if _v485 == (61 as i32) {
                    unsafe {
                        *tokenType = 58 as i32;
                    }
                    return (2 as i32) as i64;
                } else {
                    if c == (62 as i32) {
                        unsafe {
                            *tokenType = 106 as i32;
                        }
                        return (2 as i32) as i64;
                    } else {
                        unsafe {
                            *tokenType = 55 as i32;
                        }
                        return (1 as i32) as i64;
                    }
                }
                if (((unsafe { *unsafe { z.offset((1 as i32) as isize) } }) as u32) as i32)
                    != (61 as i32)
                {
                    unsafe {
                        *tokenType = 186 as i32;
                    }
                    return (1 as i32) as i64;
                } else {
                    unsafe {
                        *tokenType = 53 as i32;
                    }
                    return (2 as i32) as i64;
                }
                if (((unsafe { *unsafe { z.offset((1 as i32) as isize) } }) as u32) as i32)
                    != (124 as i32)
                {
                    unsafe {
                        *tokenType = 104 as i32;
                    }
                    return (1 as i32) as i64;
                } else {
                    unsafe {
                        *tokenType = 112 as i32;
                    }
                    return (2 as i32) as i64;
                }
                unsafe {
                    *tokenType = 25 as i32;
                }
                return (1 as i32) as i64;
            }
            13 => {
                let __v434: i32 =
                    ((unsafe { *unsafe { z.offset((1 as i32) as isize) } }) as u32) as i32;
                c = __v434;
                if __v434 == (61 as i32) {
                    unsafe {
                        *tokenType = 58 as i32;
                    }
                    return (2 as i32) as i64;
                } else {
                    if c == (62 as i32) {
                        unsafe {
                            *tokenType = 106 as i32;
                        }
                        return (2 as i32) as i64;
                    } else {
                        unsafe {
                            *tokenType = 55 as i32;
                        }
                        return (1 as i32) as i64;
                    }
                }
                if (((unsafe { *unsafe { z.offset((1 as i32) as isize) } }) as u32) as i32)
                    != (61 as i32)
                {
                    unsafe {
                        *tokenType = 186 as i32;
                    }
                    return (1 as i32) as i64;
                } else {
                    unsafe {
                        *tokenType = 53 as i32;
                    }
                    return (2 as i32) as i64;
                }
                if (((unsafe { *unsafe { z.offset((1 as i32) as isize) } }) as u32) as i32)
                    != (124 as i32)
                {
                    unsafe {
                        *tokenType = 104 as i32;
                    }
                    return (1 as i32) as i64;
                } else {
                    unsafe {
                        *tokenType = 112 as i32;
                    }
                    return (2 as i32) as i64;
                }
                unsafe {
                    *tokenType = 25 as i32;
                }
                return (1 as i32) as i64;
            }
            15 => {
                if (((unsafe { *unsafe { z.offset((1 as i32) as isize) } }) as u32) as i32)
                    != (61 as i32)
                {
                    unsafe {
                        *tokenType = 186 as i32;
                    }
                    return (1 as i32) as i64;
                } else {
                    unsafe {
                        *tokenType = 53 as i32;
                    }
                    return (2 as i32) as i64;
                }
                if (((unsafe { *unsafe { z.offset((1 as i32) as isize) } }) as u32) as i32)
                    != (124 as i32)
                {
                    unsafe {
                        *tokenType = 104 as i32;
                    }
                    return (1 as i32) as i64;
                } else {
                    unsafe {
                        *tokenType = 112 as i32;
                    }
                    return (2 as i32) as i64;
                }
                unsafe {
                    *tokenType = 25 as i32;
                }
                return (1 as i32) as i64;
            }
            10 => {
                if (((unsafe { *unsafe { z.offset((1 as i32) as isize) } }) as u32) as i32)
                    != (124 as i32)
                {
                    unsafe {
                        *tokenType = 104 as i32;
                    }
                    return (1 as i32) as i64;
                } else {
                    unsafe {
                        *tokenType = 112 as i32;
                    }
                    return (2 as i32) as i64;
                }
                unsafe {
                    *tokenType = 25 as i32;
                }
                return (1 as i32) as i64;
            }
            23 => {
                unsafe {
                    *tokenType = 25 as i32;
                }
                return (1 as i32) as i64;
            }
            24 => {
                unsafe {
                    *tokenType = 103 as i32;
                }
                return (1 as i32) as i64;
            }
            25 => {
                unsafe {
                    *tokenType = 115 as i32;
                }
                return (1 as i32) as i64;
            }
            8 => {
                let mut delim: i32 =
                    ((unsafe { *unsafe { z.offset((0 as i32) as isize) } }) as u32) as i32;
                {}
                {}
                {}
                i = (1 as i32) as i64;
                '__slate_break_386: loop {
                    let __v435: i32 = ((unsafe { *unsafe { z.offset(i as isize) } }) as u32) as i32;
                    c = __v435;
                    if !(__v435 != (0 as i32)) {
                        break;
                    }
                    if c == delim {
                        if (((unsafe { *unsafe { z.offset((i + ((1 as i32) as i64)) as isize) } })
                            as u32) as i32)
                            == delim
                        {
                            let __v438: i64 = i;
                            let __v439: i64 = __v438 + ((1 as i32) as i64);
                            i = __v439;
                        } else {
                            break '__slate_break_386;
                        }
                    }
                    let __v436: i64 = i;
                    let __v437: i64 = __v436 + ((1 as i32) as i64);
                    i = __v437;
                }
                if c == (39 as i32) {
                    unsafe {
                        *tokenType = 118 as i32;
                    }
                    return i + ((1 as i32) as i64);
                } else {
                    if c != (0 as i32) {
                        unsafe {
                            *tokenType = 60 as i32;
                        }
                        return i + ((1 as i32) as i64);
                    } else {
                        unsafe {
                            *tokenType = 186 as i32;
                        }
                        return i;
                    }
                }
                if !((((unsafe {
                    *unsafe {
                        unsafe { std::ptr::addr_of!(sqlite3CtypeMap) as *const u8 }.offset(
                            (((unsafe { *unsafe { z.offset((1 as i32) as isize) } }) as u32) as i32)
                                as isize,
                        )
                    }
                }) as u32) as i32)
                    & (4 as i32)
                    != (0 as i32))
                {
                    unsafe {
                        *tokenType = 142 as i32;
                    }
                    return (1 as i32) as i64;
                }
                // /* If the next character is a digit, this is a floating point
                //       ** number that begins with ".".  Fall thru into the next case */
                // /* no break */
                {}
                {}
                {}
                {}
                {}
                {}
                {}
                {}
                {}
                {}
                {}
                {}
                unsafe {
                    *tokenType = 156 as i32;
                }
                if (((unsafe { *unsafe { z.offset((0 as i32) as isize) } }) as u32) as i32)
                    == (48 as i32)
                    && ((((unsafe { *unsafe { z.offset((1 as i32) as isize) } }) as u32) as i32)
                        == (120 as i32)
                        || (((unsafe { *unsafe { z.offset((1 as i32) as isize) } }) as u32) as i32)
                            == (88 as i32))
                    && (((unsafe {
                        *unsafe {
                            unsafe { std::ptr::addr_of!(sqlite3CtypeMap) as *const u8 }.offset(
                                (((unsafe { *unsafe { z.offset((2 as i32) as isize) } }) as u32)
                                    as i32) as isize,
                            )
                        }
                    }) as u32) as i32)
                        & (8 as i32)
                        != (0 as i32)
                {
                    i = (3 as i32) as i64;
                    '__slate_break_387: loop {
                        if !((1 as i32) != (0 as i32)) {
                            break;
                        }
                        if (((unsafe {
                            *unsafe {
                                unsafe { std::ptr::addr_of!(sqlite3CtypeMap) as *const u8 }.offset(
                                    (((unsafe { *unsafe { z.offset(i as isize) } }) as u32) as i32)
                                        as isize,
                                )
                            }
                        }) as u32) as i32)
                            & (8 as i32)
                            == (0 as i32)
                        {
                            if (((unsafe { *unsafe { z.offset(i as isize) } }) as u32) as i32)
                                == (95 as i32)
                            {
                                unsafe {
                                    *tokenType = 183 as i32;
                                }
                            } else {
                                break '__slate_break_387;
                            }
                        }
                        let _v486: i64 = i;
                        let _v487: i64 = _v486 + ((1 as i32) as i64);
                        i = _v487;
                    }
                } else {
                    i = (0 as i32) as i64;
                    '__slate_break_388: loop {
                        if !((1 as i32) != (0 as i32)) {
                            break;
                        }
                        if (((unsafe {
                            *unsafe {
                                unsafe { std::ptr::addr_of!(sqlite3CtypeMap) as *const u8 }.offset(
                                    (((unsafe { *unsafe { z.offset(i as isize) } }) as u32) as i32)
                                        as isize,
                                )
                            }
                        }) as u32) as i32)
                            & (4 as i32)
                            == (0 as i32)
                        {
                            if (((unsafe { *unsafe { z.offset(i as isize) } }) as u32) as i32)
                                == (95 as i32)
                            {
                                unsafe {
                                    *tokenType = 183 as i32;
                                }
                            } else {
                                break '__slate_break_388;
                            }
                        }
                        let _v488: i64 = i;
                        let _v489: i64 = _v488 + ((1 as i32) as i64);
                        i = _v489;
                    }
                    if (((unsafe { *unsafe { z.offset(i as isize) } }) as u32) as i32)
                        == (46 as i32)
                    {
                        if (unsafe { *tokenType }) == (156 as i32) {
                            unsafe {
                                *tokenType = 154 as i32;
                            }
                        }
                        let _v490: i64 = i;
                        let _v491: i64 = _v490 + ((1 as i32) as i64);
                        i = _v491;
                        '__slate_break_389: loop {
                            if !((1 as i32) != (0 as i32)) {
                                break;
                            }
                            if (((unsafe {
                                *unsafe {
                                    unsafe { std::ptr::addr_of!(sqlite3CtypeMap) as *const u8 }
                                        .offset(
                                            (((unsafe { *unsafe { z.offset(i as isize) } }) as u32)
                                                as i32)
                                                as isize,
                                        )
                                }
                            }) as u32) as i32)
                                & (4 as i32)
                                == (0 as i32)
                            {
                                if (((unsafe { *unsafe { z.offset(i as isize) } }) as u32) as i32)
                                    == (95 as i32)
                                {
                                    unsafe {
                                        *tokenType = 183 as i32;
                                    }
                                } else {
                                    break '__slate_break_389;
                                }
                            }
                            let _v492: i64 = i;
                            let _v493: i64 = _v492 + ((1 as i32) as i64);
                            i = _v493;
                        }
                    }
                    if ((((unsafe { *unsafe { z.offset(i as isize) } }) as u32) as i32)
                        == (101 as i32)
                        || (((unsafe { *unsafe { z.offset(i as isize) } }) as u32) as i32)
                            == (69 as i32))
                        && ((((unsafe {
                            *unsafe {
                                unsafe { std::ptr::addr_of!(sqlite3CtypeMap) as *const u8 }.offset(
                                    (((unsafe {
                                        *unsafe { z.offset((i + ((1 as i32) as i64)) as isize) }
                                    }) as u32) as i32) as isize,
                                )
                            }
                        }) as u32) as i32)
                            & (4 as i32)
                            != (0 as i32)
                            || ((((unsafe {
                                *unsafe { z.offset((i + ((1 as i32) as i64)) as isize) }
                            }) as u32) as i32)
                                == (43 as i32)
                                || (((unsafe {
                                    *unsafe { z.offset((i + ((1 as i32) as i64)) as isize) }
                                }) as u32) as i32)
                                    == (45 as i32))
                                && (((unsafe {
                                    *unsafe {
                                        unsafe { std::ptr::addr_of!(sqlite3CtypeMap) as *const u8 }
                                            .offset(
                                                (((unsafe {
                                                    *unsafe {
                                                        z.offset((i + ((2 as i32) as i64)) as isize)
                                                    }
                                                })
                                                    as u32)
                                                    as i32)
                                                    as isize,
                                            )
                                    }
                                }) as u32) as i32)
                                    & (4 as i32)
                                    != (0 as i32))
                    {
                        if (unsafe { *tokenType }) == (156 as i32) {
                            unsafe {
                                *tokenType = 154 as i32;
                            }
                        }
                        let _v494: i64 = i;
                        let _v495: i64 = _v494 + ((2 as i32) as i64);
                        i = _v495;
                        '__slate_break_390: loop {
                            if !((1 as i32) != (0 as i32)) {
                                break;
                            }
                            if (((unsafe {
                                *unsafe {
                                    unsafe { std::ptr::addr_of!(sqlite3CtypeMap) as *const u8 }
                                        .offset(
                                            (((unsafe { *unsafe { z.offset(i as isize) } }) as u32)
                                                as i32)
                                                as isize,
                                        )
                                }
                            }) as u32) as i32)
                                & (4 as i32)
                                == (0 as i32)
                            {
                                if (((unsafe { *unsafe { z.offset(i as isize) } }) as u32) as i32)
                                    == (95 as i32)
                                {
                                    unsafe {
                                        *tokenType = 183 as i32;
                                    }
                                } else {
                                    break '__slate_break_390;
                                }
                            }
                            let _v496: i64 = i;
                            let _v497: i64 = _v496 + ((1 as i32) as i64);
                            i = _v497;
                        }
                    }
                }
                '__slate_break_391: loop {
                    if !((((unsafe {
                        *unsafe {
                            unsafe { std::ptr::addr_of!(sqlite3CtypeMap) as *const u8 }.offset(
                                (((unsafe { *unsafe { z.offset(i as isize) } }) as u32) as i32)
                                    as isize,
                            )
                        }
                    }) as u32) as i32)
                        & (70 as i32)
                        != (0 as i32))
                    {
                        break;
                    }
                    unsafe {
                        *tokenType = 186 as i32;
                    }
                    let _v498: i64 = i;
                    let _v499: i64 = _v498 + ((1 as i32) as i64);
                    i = _v499;
                }
                return i;
            }
            26 => {
                if !((((unsafe {
                    *unsafe {
                        unsafe { std::ptr::addr_of!(sqlite3CtypeMap) as *const u8 }.offset(
                            (((unsafe { *unsafe { z.offset((1 as i32) as isize) } }) as u32) as i32)
                                as isize,
                        )
                    }
                }) as u32) as i32)
                    & (4 as i32)
                    != (0 as i32))
                {
                    unsafe {
                        *tokenType = 142 as i32;
                    }
                    return (1 as i32) as i64;
                }
                // /* If the next character is a digit, this is a floating point
                //       ** number that begins with ".".  Fall thru into the next case */
                // /* no break */
                {}
                {}
                {}
                {}
                {}
                {}
                {}
                {}
                {}
                {}
                {}
                {}
                unsafe {
                    *tokenType = 156 as i32;
                }
                if (((unsafe { *unsafe { z.offset((0 as i32) as isize) } }) as u32) as i32)
                    == (48 as i32)
                    && ((((unsafe { *unsafe { z.offset((1 as i32) as isize) } }) as u32) as i32)
                        == (120 as i32)
                        || (((unsafe { *unsafe { z.offset((1 as i32) as isize) } }) as u32) as i32)
                            == (88 as i32))
                    && (((unsafe {
                        *unsafe {
                            unsafe { std::ptr::addr_of!(sqlite3CtypeMap) as *const u8 }.offset(
                                (((unsafe { *unsafe { z.offset((2 as i32) as isize) } }) as u32)
                                    as i32) as isize,
                            )
                        }
                    }) as u32) as i32)
                        & (8 as i32)
                        != (0 as i32)
                {
                    i = (3 as i32) as i64;
                    '__slate_break_387: loop {
                        if !((1 as i32) != (0 as i32)) {
                            break;
                        }
                        if (((unsafe {
                            *unsafe {
                                unsafe { std::ptr::addr_of!(sqlite3CtypeMap) as *const u8 }.offset(
                                    (((unsafe { *unsafe { z.offset(i as isize) } }) as u32) as i32)
                                        as isize,
                                )
                            }
                        }) as u32) as i32)
                            & (8 as i32)
                            == (0 as i32)
                        {
                            if (((unsafe { *unsafe { z.offset(i as isize) } }) as u32) as i32)
                                == (95 as i32)
                            {
                                unsafe {
                                    *tokenType = 183 as i32;
                                }
                            } else {
                                break '__slate_break_387;
                            }
                        }
                        let _v500: i64 = i;
                        let _v501: i64 = _v500 + ((1 as i32) as i64);
                        i = _v501;
                    }
                } else {
                    i = (0 as i32) as i64;
                    '__slate_break_388: loop {
                        if !((1 as i32) != (0 as i32)) {
                            break;
                        }
                        if (((unsafe {
                            *unsafe {
                                unsafe { std::ptr::addr_of!(sqlite3CtypeMap) as *const u8 }.offset(
                                    (((unsafe { *unsafe { z.offset(i as isize) } }) as u32) as i32)
                                        as isize,
                                )
                            }
                        }) as u32) as i32)
                            & (4 as i32)
                            == (0 as i32)
                        {
                            if (((unsafe { *unsafe { z.offset(i as isize) } }) as u32) as i32)
                                == (95 as i32)
                            {
                                unsafe {
                                    *tokenType = 183 as i32;
                                }
                            } else {
                                break '__slate_break_388;
                            }
                        }
                        let _v502: i64 = i;
                        let _v503: i64 = _v502 + ((1 as i32) as i64);
                        i = _v503;
                    }
                    if (((unsafe { *unsafe { z.offset(i as isize) } }) as u32) as i32)
                        == (46 as i32)
                    {
                        if (unsafe { *tokenType }) == (156 as i32) {
                            unsafe {
                                *tokenType = 154 as i32;
                            }
                        }
                        let _v504: i64 = i;
                        let _v505: i64 = _v504 + ((1 as i32) as i64);
                        i = _v505;
                        '__slate_break_389: loop {
                            if !((1 as i32) != (0 as i32)) {
                                break;
                            }
                            if (((unsafe {
                                *unsafe {
                                    unsafe { std::ptr::addr_of!(sqlite3CtypeMap) as *const u8 }
                                        .offset(
                                            (((unsafe { *unsafe { z.offset(i as isize) } }) as u32)
                                                as i32)
                                                as isize,
                                        )
                                }
                            }) as u32) as i32)
                                & (4 as i32)
                                == (0 as i32)
                            {
                                if (((unsafe { *unsafe { z.offset(i as isize) } }) as u32) as i32)
                                    == (95 as i32)
                                {
                                    unsafe {
                                        *tokenType = 183 as i32;
                                    }
                                } else {
                                    break '__slate_break_389;
                                }
                            }
                            let _v506: i64 = i;
                            let _v507: i64 = _v506 + ((1 as i32) as i64);
                            i = _v507;
                        }
                    }
                    if ((((unsafe { *unsafe { z.offset(i as isize) } }) as u32) as i32)
                        == (101 as i32)
                        || (((unsafe { *unsafe { z.offset(i as isize) } }) as u32) as i32)
                            == (69 as i32))
                        && ((((unsafe {
                            *unsafe {
                                unsafe { std::ptr::addr_of!(sqlite3CtypeMap) as *const u8 }.offset(
                                    (((unsafe {
                                        *unsafe { z.offset((i + ((1 as i32) as i64)) as isize) }
                                    }) as u32) as i32) as isize,
                                )
                            }
                        }) as u32) as i32)
                            & (4 as i32)
                            != (0 as i32)
                            || ((((unsafe {
                                *unsafe { z.offset((i + ((1 as i32) as i64)) as isize) }
                            }) as u32) as i32)
                                == (43 as i32)
                                || (((unsafe {
                                    *unsafe { z.offset((i + ((1 as i32) as i64)) as isize) }
                                }) as u32) as i32)
                                    == (45 as i32))
                                && (((unsafe {
                                    *unsafe {
                                        unsafe { std::ptr::addr_of!(sqlite3CtypeMap) as *const u8 }
                                            .offset(
                                                (((unsafe {
                                                    *unsafe {
                                                        z.offset((i + ((2 as i32) as i64)) as isize)
                                                    }
                                                })
                                                    as u32)
                                                    as i32)
                                                    as isize,
                                            )
                                    }
                                }) as u32) as i32)
                                    & (4 as i32)
                                    != (0 as i32))
                    {
                        if (unsafe { *tokenType }) == (156 as i32) {
                            unsafe {
                                *tokenType = 154 as i32;
                            }
                        }
                        let _v508: i64 = i;
                        let _v509: i64 = _v508 + ((2 as i32) as i64);
                        i = _v509;
                        '__slate_break_390: loop {
                            if !((1 as i32) != (0 as i32)) {
                                break;
                            }
                            if (((unsafe {
                                *unsafe {
                                    unsafe { std::ptr::addr_of!(sqlite3CtypeMap) as *const u8 }
                                        .offset(
                                            (((unsafe { *unsafe { z.offset(i as isize) } }) as u32)
                                                as i32)
                                                as isize,
                                        )
                                }
                            }) as u32) as i32)
                                & (4 as i32)
                                == (0 as i32)
                            {
                                if (((unsafe { *unsafe { z.offset(i as isize) } }) as u32) as i32)
                                    == (95 as i32)
                                {
                                    unsafe {
                                        *tokenType = 183 as i32;
                                    }
                                } else {
                                    break '__slate_break_390;
                                }
                            }
                            let _v510: i64 = i;
                            let _v511: i64 = _v510 + ((1 as i32) as i64);
                            i = _v511;
                        }
                    }
                }
                '__slate_break_391: while (((unsafe {
                    *unsafe {
                        unsafe { std::ptr::addr_of!(sqlite3CtypeMap) as *const u8 }.offset(
                            (((unsafe { *unsafe { z.offset(i as isize) } }) as u32) as i32)
                                as isize,
                        )
                    }
                }) as u32) as i32)
                    & (70 as i32)
                    != (0 as i32)
                {
                    '__slate_continue_391: {
                        {
                            unsafe {
                                *tokenType = 186 as i32;
                            }
                            let _v512: i64 = i;
                            let _v513: i64 = _v512 + ((1 as i32) as i64);
                            i = _v513;
                        }
                    }
                }
                return i;
            }
            3 => {
                {}
                {}
                {}
                {}
                {}
                {}
                {}
                {}
                {}
                {}
                {}
                unsafe {
                    *tokenType = 156 as i32;
                }
                if (((unsafe { *unsafe { z.offset((0 as i32) as isize) } }) as u32) as i32)
                    == (48 as i32)
                    && ((((unsafe { *unsafe { z.offset((1 as i32) as isize) } }) as u32) as i32)
                        == (120 as i32)
                        || (((unsafe { *unsafe { z.offset((1 as i32) as isize) } }) as u32) as i32)
                            == (88 as i32))
                    && (((unsafe {
                        *unsafe {
                            unsafe { std::ptr::addr_of!(sqlite3CtypeMap) as *const u8 }.offset(
                                (((unsafe { *unsafe { z.offset((2 as i32) as isize) } }) as u32)
                                    as i32) as isize,
                            )
                        }
                    }) as u32) as i32)
                        & (8 as i32)
                        != (0 as i32)
                {
                    {
                        {
                            i = (3 as i32) as i64;
                            '__slate_break_387: loop {
                                if !((1 as i32) != (0 as i32)) {
                                    break;
                                }
                                '__slate_continue_387: {
                                    {
                                        if (((unsafe {
                                            *unsafe {
                                                unsafe {
                                                    std::ptr::addr_of!(sqlite3CtypeMap) as *const u8
                                                }
                                                .offset(
                                                    (((unsafe { *unsafe { z.offset(i as isize) } })
                                                        as u32)
                                                        as i32)
                                                        as isize,
                                                )
                                            }
                                        }) as u32)
                                            as i32)
                                            & (8 as i32)
                                            == (0 as i32)
                                        {
                                            {
                                                if (((unsafe { *unsafe { z.offset(i as isize) } })
                                                    as u32)
                                                    as i32)
                                                    == (95 as i32)
                                                {
                                                    {
                                                        unsafe {
                                                            *tokenType = 183 as i32;
                                                        }
                                                    }
                                                } else {
                                                    {
                                                        break '__slate_break_387;
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                                let __v440: i64 = i;
                                let __v441: i64 = __v440 + ((1 as i32) as i64);
                                i = __v441;
                            }
                        }
                    }
                } else {
                    {
                        {
                            i = (0 as i32) as i64;
                            '__slate_break_388: loop {
                                if !((1 as i32) != (0 as i32)) {
                                    break;
                                }
                                '__slate_continue_388: {
                                    {
                                        if (((unsafe {
                                            *unsafe {
                                                unsafe {
                                                    std::ptr::addr_of!(sqlite3CtypeMap) as *const u8
                                                }
                                                .offset(
                                                    (((unsafe { *unsafe { z.offset(i as isize) } })
                                                        as u32)
                                                        as i32)
                                                        as isize,
                                                )
                                            }
                                        }) as u32)
                                            as i32)
                                            & (4 as i32)
                                            == (0 as i32)
                                        {
                                            {
                                                if (((unsafe { *unsafe { z.offset(i as isize) } })
                                                    as u32)
                                                    as i32)
                                                    == (95 as i32)
                                                {
                                                    {
                                                        unsafe {
                                                            *tokenType = 183 as i32;
                                                        }
                                                    }
                                                } else {
                                                    {
                                                        break '__slate_break_388;
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                                let __v442: i64 = i;
                                let __v443: i64 = __v442 + ((1 as i32) as i64);
                                i = __v443;
                            }
                        }
                        if (((unsafe { *unsafe { z.offset(i as isize) } }) as u32) as i32)
                            == (46 as i32)
                        {
                            {
                                if (unsafe { *tokenType }) == (156 as i32) {
                                    unsafe {
                                        *tokenType = 154 as i32;
                                    }
                                }
                                {
                                    let __v444: i64 = i;
                                    let __v445: i64 = __v444 + ((1 as i32) as i64);
                                    i = __v445;
                                    '__slate_break_389: loop {
                                        if !((1 as i32) != (0 as i32)) {
                                            break;
                                        }
                                        '__slate_continue_389: {
                                            {
                                                if (((unsafe {
                                                    *unsafe {
                                                        unsafe {
                                                            std::ptr::addr_of!(sqlite3CtypeMap)
                                                                as *const u8
                                                        }
                                                        .offset(
                                                            (((unsafe {
                                                                *unsafe { z.offset(i as isize) }
                                                            })
                                                                as u32)
                                                                as i32)
                                                                as isize,
                                                        )
                                                    }
                                                })
                                                    as u32)
                                                    as i32)
                                                    & (4 as i32)
                                                    == (0 as i32)
                                                {
                                                    {
                                                        if (((unsafe {
                                                            *unsafe { z.offset(i as isize) }
                                                        })
                                                            as u32)
                                                            as i32)
                                                            == (95 as i32)
                                                        {
                                                            {
                                                                unsafe {
                                                                    *tokenType = 183 as i32;
                                                                }
                                                            }
                                                        } else {
                                                            {
                                                                break '__slate_break_389;
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        let __v446: i64 = i;
                                        let __v447: i64 = __v446 + ((1 as i32) as i64);
                                        i = __v447;
                                    }
                                }
                            }
                        }
                        if ((((unsafe { *unsafe { z.offset(i as isize) } }) as u32) as i32)
                            == (101 as i32)
                            || (((unsafe { *unsafe { z.offset(i as isize) } }) as u32) as i32)
                                == (69 as i32))
                            && ((((unsafe {
                                *unsafe {
                                    unsafe { std::ptr::addr_of!(sqlite3CtypeMap) as *const u8 }
                                        .offset(
                                            (((unsafe {
                                                *unsafe {
                                                    z.offset((i + ((1 as i32) as i64)) as isize)
                                                }
                                            }) as u32)
                                                as i32)
                                                as isize,
                                        )
                                }
                            }) as u32) as i32)
                                & (4 as i32)
                                != (0 as i32)
                                || ((((unsafe {
                                    *unsafe { z.offset((i + ((1 as i32) as i64)) as isize) }
                                }) as u32) as i32)
                                    == (43 as i32)
                                    || (((unsafe {
                                        *unsafe { z.offset((i + ((1 as i32) as i64)) as isize) }
                                    }) as u32) as i32)
                                        == (45 as i32))
                                    && (((unsafe {
                                        *unsafe {
                                            unsafe {
                                                std::ptr::addr_of!(sqlite3CtypeMap) as *const u8
                                            }
                                            .offset(
                                                (((unsafe {
                                                    *unsafe {
                                                        z.offset((i + ((2 as i32) as i64)) as isize)
                                                    }
                                                })
                                                    as u32)
                                                    as i32)
                                                    as isize,
                                            )
                                        }
                                    }) as u32) as i32)
                                        & (4 as i32)
                                        != (0 as i32))
                        {
                            {
                                if (unsafe { *tokenType }) == (156 as i32) {
                                    unsafe {
                                        *tokenType = 154 as i32;
                                    }
                                }
                                {
                                    let __v448: i64 = i;
                                    let __v449: i64 = __v448 + ((2 as i32) as i64);
                                    i = __v449;
                                    '__slate_break_390: loop {
                                        if !((1 as i32) != (0 as i32)) {
                                            break;
                                        }
                                        '__slate_continue_390: {
                                            {
                                                if (((unsafe {
                                                    *unsafe {
                                                        unsafe {
                                                            std::ptr::addr_of!(sqlite3CtypeMap)
                                                                as *const u8
                                                        }
                                                        .offset(
                                                            (((unsafe {
                                                                *unsafe { z.offset(i as isize) }
                                                            })
                                                                as u32)
                                                                as i32)
                                                                as isize,
                                                        )
                                                    }
                                                })
                                                    as u32)
                                                    as i32)
                                                    & (4 as i32)
                                                    == (0 as i32)
                                                {
                                                    {
                                                        if (((unsafe {
                                                            *unsafe { z.offset(i as isize) }
                                                        })
                                                            as u32)
                                                            as i32)
                                                            == (95 as i32)
                                                        {
                                                            {
                                                                unsafe {
                                                                    *tokenType = 183 as i32;
                                                                }
                                                            }
                                                        } else {
                                                            {
                                                                break '__slate_break_390;
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        let __v450: i64 = i;
                                        let __v451: i64 = __v450 + ((1 as i32) as i64);
                                        i = __v451;
                                    }
                                }
                            }
                        }
                    }
                }
                '__slate_break_391: while (((unsafe {
                    *unsafe {
                        unsafe { std::ptr::addr_of!(sqlite3CtypeMap) as *const u8 }.offset(
                            (((unsafe { *unsafe { z.offset(i as isize) } }) as u32) as i32)
                                as isize,
                        )
                    }
                }) as u32) as i32)
                    & (70 as i32)
                    != (0 as i32)
                {
                    '__slate_continue_391: {
                        {
                            unsafe {
                                *tokenType = 186 as i32;
                            }
                            let __v452: i64 = i;
                            let __v453: i64 = __v452 + ((1 as i32) as i64);
                            i = __v453;
                        }
                    }
                }
                return i;
            }
            9 => {
                i = (1 as i32) as i64;
                let __v454: i32 =
                    ((unsafe { *unsafe { z.offset((0 as i32) as isize) } }) as u32) as i32;
                c = __v454;
                '__slate_break_392: loop {
                    let __v455: bool;
                    if c != (93 as i32) {
                        let __v456: i32 =
                            ((unsafe { *unsafe { z.offset(i as isize) } }) as u32) as i32;
                        c = __v456;
                        __v455 = __v456 != (0 as i32);
                    } else {
                        __v455 = false as bool;
                    }
                    if !__v455 {
                        break;
                    }
                    '__slate_continue_392: {
                        {}
                    }
                    let __v457: i64 = i;
                    let __v458: i64 = __v457 + ((1 as i32) as i64);
                    i = __v458;
                }
                unsafe {
                    *tokenType = if c == (93 as i32) {
                        60 as i32
                    } else {
                        186 as i32
                    };
                }
                return i;
            }
            6 => {
                unsafe {
                    *tokenType = 157 as i32;
                }
                i = (1 as i32) as i64;
                '__slate_break_393: loop {
                    if !((((unsafe {
                        *unsafe {
                            unsafe { std::ptr::addr_of!(sqlite3CtypeMap) as *const u8 }.offset(
                                (((unsafe { *unsafe { z.offset(i as isize) } }) as u32) as i32)
                                    as isize,
                            )
                        }
                    }) as u32) as i32)
                        & (4 as i32)
                        != (0 as i32))
                    {
                        break;
                    }
                    '__slate_continue_393: {
                        {}
                    }
                    let __v459: i64 = i;
                    let __v460: i64 = __v459 + ((1 as i32) as i64);
                    i = __v460;
                }
                return i;
            }
            4 | 5 => {
                let mut n: i64 = (0 as i32) as i64;
                {}
                {}
                {}
                {}
                unsafe {
                    *tokenType = 157 as i32;
                }
                i = (1 as i32) as i64;
                '__slate_break_394: loop {
                    let __v461: i32 = ((unsafe { *unsafe { z.offset(i as isize) } }) as u32) as i32;
                    c = __v461;
                    if !(__v461 != (0 as i32)) {
                        break;
                    }
                    '__slate_continue_394: {
                        {
                            if (((unsafe {
                                *unsafe {
                                    unsafe { std::ptr::addr_of!(sqlite3CtypeMap) as *const u8 }
                                        .offset(((((c as i8) as u8) as u32) as i32) as isize)
                                }
                            }) as u32) as i32)
                                & (70 as i32)
                                != (0 as i32)
                            {
                                {
                                    let __v464: i64 = n;
                                    let __v465: i64 = __v464 + ((1 as i32) as i64);
                                    n = __v465;
                                }
                            } else {
                                if c == (40 as i32) && n > ((0 as i32) as i64) {
                                    {
                                        '__slate_break_395: loop {
                                            '__slate_continue_395: {
                                                {
                                                    let __v466: i64 = i;
                                                    let __v467: i64 = __v466 + ((1 as i32) as i64);
                                                    i = __v467;
                                                }
                                            }
                                            let __v468: i32 =
                                                ((unsafe { *unsafe { z.offset(i as isize) } })
                                                    as u32)
                                                    as i32;
                                            c = __v468;
                                            if !(__v468 != (0 as i32)
                                                && !((((unsafe {
                                                    *unsafe {
                                                        unsafe {
                                                            std::ptr::addr_of!(sqlite3CtypeMap)
                                                                as *const u8
                                                        }
                                                        .offset(
                                                            ((((c as i8) as u8) as u32) as i32)
                                                                as isize,
                                                        )
                                                    }
                                                })
                                                    as u32)
                                                    as i32)
                                                    & (1 as i32)
                                                    != (0 as i32))
                                                && c != (41 as i32))
                                            {
                                                break;
                                            }
                                        }
                                        if c == (41 as i32) {
                                            {
                                                let __v469: i64 = i;
                                                let __v470: i64 = __v469 + ((1 as i32) as i64);
                                                i = __v470;
                                            }
                                        } else {
                                            {
                                                unsafe {
                                                    *tokenType = 186 as i32;
                                                }
                                            }
                                        }
                                        break '__slate_break_394;
                                    }
                                } else {
                                    if c == (58 as i32)
                                        && (((unsafe {
                                            *unsafe { z.offset((i + ((1 as i32) as i64)) as isize) }
                                        }) as u32)
                                            as i32)
                                            == (58 as i32)
                                    {
                                        {
                                            let __v471: i64 = i;
                                            let __v472: i64 = __v471 + ((1 as i32) as i64);
                                            i = __v472;
                                        }
                                    } else {
                                        {
                                            break '__slate_break_394;
                                        }
                                    }
                                }
                            }
                        }
                    }
                    let __v462: i64 = i;
                    let __v463: i64 = __v462 + ((1 as i32) as i64);
                    i = __v463;
                }
                if n == ((0 as i32) as i64) {
                    unsafe {
                        *tokenType = 186 as i32;
                    }
                }
                return i;
            }
            1 => {
                if (((unsafe {
                    *unsafe {
                        unsafe { std::ptr::addr_of!(aiClass.0) as *const u8 }.offset(
                            (((unsafe { *unsafe { z.offset((1 as i32) as isize) } }) as u32) as i32)
                                as isize,
                        )
                    }
                }) as u32) as i32)
                    > (2 as i32)
                {
                    {
                        i = (1 as i32) as i64;
                        break '__slate_break_382;
                    }
                }
                i = (2 as i32) as i64;
                '__slate_break_396: loop {
                    if !((((unsafe {
                        *unsafe {
                            unsafe { std::ptr::addr_of!(aiClass.0) as *const u8 }.offset(
                                (((unsafe { *unsafe { z.offset(i as isize) } }) as u32) as i32)
                                    as isize,
                            )
                        }
                    }) as u32) as i32)
                        <= (2 as i32))
                    {
                        break;
                    }
                    '__slate_continue_396: {
                        {}
                    }
                    let __v473: i64 = i;
                    let __v474: i64 = __v473 + ((1 as i32) as i64);
                    i = __v474;
                }
                if (((unsafe {
                    *unsafe {
                        unsafe { std::ptr::addr_of!(sqlite3CtypeMap) as *const u8 }.offset(
                            (((unsafe { *unsafe { z.offset(i as isize) } }) as u32) as i32)
                                as isize,
                        )
                    }
                }) as u32) as i32)
                    & (70 as i32)
                    != (0 as i32)
                {
                    {
                        // /* This token started out using characters that can appear in keywords,
                        //         ** but z[i] is a character not allowed within keywords, so this must
                        //         ** be an identifier instead */
                        let __v475: i64 = i;
                        let __v476: i64 = __v475 + ((1 as i32) as i64);
                        i = __v476;
                        break '__slate_break_382;
                    }
                }
                unsafe {
                    *tokenType = 60 as i32;
                }
                return keywordCode((z as *mut i8) as *const i8, i, tokenType);
            }
            0 => {
                {}
                {}
                if (((unsafe { *unsafe { z.offset((1 as i32) as isize) } }) as u32) as i32)
                    == (39 as i32)
                {
                    {
                        unsafe {
                            *tokenType = 155 as i32;
                        }
                        {
                            i = (2 as i32) as i64;
                            '__slate_break_397: loop {
                                if !((((unsafe {
                                    *unsafe {
                                        unsafe { std::ptr::addr_of!(sqlite3CtypeMap) as *const u8 }
                                            .offset(
                                                (((unsafe { *unsafe { z.offset(i as isize) } })
                                                    as u32)
                                                    as i32)
                                                    as isize,
                                            )
                                    }
                                }) as u32) as i32)
                                    & (8 as i32)
                                    != (0 as i32))
                                {
                                    break;
                                }
                                '__slate_continue_397: {
                                    {}
                                }
                                let __v477: i64 = i;
                                let __v478: i64 = __v477 + ((1 as i32) as i64);
                                i = __v478;
                            }
                        }
                        if (((unsafe { *unsafe { z.offset(i as isize) } }) as u32) as i32)
                            != (39 as i32)
                            || i % ((2 as i32) as i64) != (0 as i64)
                        {
                            {
                                unsafe {
                                    *tokenType = 186 as i32;
                                }
                                '__slate_break_398: loop {
                                    if !((unsafe { *unsafe { z.offset(i as isize) } }) != (0 as u8)
                                        && (((unsafe { *unsafe { z.offset(i as isize) } }) as u32)
                                            as i32)
                                            != (39 as i32))
                                    {
                                        break;
                                    }
                                    '__slate_continue_398: {
                                        {
                                            let __v479: i64 = i;
                                            let __v480: i64 = __v479 + ((1 as i32) as i64);
                                            i = __v480;
                                        }
                                    }
                                }
                            }
                        }
                        if (unsafe { *unsafe { z.offset(i as isize) } }) != (0 as u8) {
                            let __v481: i64 = i;
                            let __v482: i64 = __v481 + ((1 as i32) as i64);
                            i = __v482;
                        }
                        return i;
                    }
                }
                // /* If it is not a BLOB literal, then it must be an ID, since no
                //       ** SQL keywords start with the letter 'x'.  Fall through */
                // /* no break */
                {}
                i = (1 as i32) as i64;
            }
            2 | 27 => {
                i = (1 as i32) as i64;
            }
            30 => {
                if (((unsafe { *unsafe { z.offset((1 as i32) as isize) } }) as u32) as i32)
                    == (187 as i32)
                    && (((unsafe { *unsafe { z.offset((2 as i32) as isize) } }) as u32) as i32)
                        == (191 as i32)
                {
                    {
                        unsafe {
                            *tokenType = 184 as i32;
                        }
                        return (3 as i32) as i64;
                    }
                }
                i = (1 as i32) as i64;
            }
            29 => {
                unsafe {
                    *tokenType = 186 as i32;
                }
                return (0 as i32) as i64;
            }
            _ => {
                unsafe {
                    *tokenType = 186 as i32;
                }
                return (1 as i32) as i64;
            }
        }
    }
    '__slate_break_399: while (((unsafe {
        *unsafe {
            unsafe { std::ptr::addr_of!(sqlite3CtypeMap) as *const u8 }
                .offset((((unsafe { *unsafe { z.offset(i as isize) } }) as u32) as i32) as isize)
        }
    }) as u32) as i32)
        & (70 as i32)
        != (0 as i32)
    {
        let __v483: i64 = i;
        let __v484: i64 = __v483 + ((1 as i32) as i64);
        i = __v484;
    }
    unsafe {
        *tokenType = 60 as i32;
    }
    return i;
}

fn keywordCode(mut z: *const i8, mut n: i64, mut pType: *mut i32) -> i64 {
    let mut i: i64 = 0 as i64;
    let mut j: i64 = 0 as i64;
    let mut zKW: *const i8 = unsafe { std::mem::zeroed() };
    0 as i32;
    i = ((((((unsafe {
        *unsafe {
            unsafe { std::ptr::addr_of!(sqlite3UpperToLower) as *const u8 }.offset(
                ((((unsafe { *unsafe { z.offset((0 as i32) as isize) } }) as u8) as u32) as i32)
                    as isize,
            )
        }
    }) as u32) as i32)
        * (4 as i32)
        ^ (((unsafe {
            *unsafe {
                unsafe { std::ptr::addr_of!(sqlite3UpperToLower) as *const u8 }.offset(
                    ((((unsafe { *unsafe { z.offset((n - ((1 as i32) as i64)) as isize) } }) as u8)
                        as u32) as i32) as isize,
                )
            }
        }) as u32) as i32)
            * (3 as i32)) as i64)
        ^ n * ((1 as i32) as i64))
        % ((127 as i32) as i64);
    i = (((unsafe {
        *unsafe { unsafe { std::ptr::addr_of!(aKWHash.0) as *const u8 }.offset(i as isize) }
    }) as u32) as i32) as i64;
    '__slate_break_379: while i > ((0 as i32) as i64) {
        if ((((unsafe {
            *unsafe { unsafe { std::ptr::addr_of!(aKWLen.0) as *const u8 }.offset(i as isize) }
        }) as u32) as i32) as i64)
            != n
        {
        } else {
            zKW = unsafe {
                unsafe { std::ptr::addr_of!(zKWText.0) as *const i8 }.offset(
                    (((unsafe {
                        *unsafe {
                            unsafe { std::ptr::addr_of!(aKWOffset.0) as *const u16 }
                                .offset(i as isize)
                        }
                    }) as u32) as i32) as isize,
                )
            };
            if ((unsafe { *unsafe { z.offset((0 as i32) as isize) } }) as i32) & !(32 as i32)
                != ((unsafe { *unsafe { zKW.offset((0 as i32) as isize) } }) as i32)
            {
            } else {
                if ((unsafe { *unsafe { z.offset((1 as i32) as isize) } }) as i32) & !(32 as i32)
                    != ((unsafe { *unsafe { zKW.offset((1 as i32) as isize) } }) as i32)
                {
                } else {
                    j = (2 as i32) as i64;
                    '__slate_break_380: while j < n
                        && ((unsafe { *unsafe { z.offset(j as isize) } }) as i32) & !(32 as i32)
                            == ((unsafe { *unsafe { zKW.offset(j as isize) } }) as i32)
                    {
                        let __v485: i64 = j;
                        let __v486: i64 = __v485 + ((1 as i32) as i64);
                        j = __v486;
                    }
                    if j < n {
                    } else {
                        // /* REINDEX */
                        {}
                        // /* INDEXED */
                        {}
                        // /* INDEX */
                        {}
                        // /* DESC */
                        {}
                        // /* ESCAPE */
                        {}
                        // /* EACH */
                        {}
                        // /* CHECK */
                        {}
                        // /* KEY */
                        {}
                        // /* BEFORE */
                        {}
                        // /* FOREIGN */
                        {}
                        // /* FOR */
                        {}
                        // /* IGNORE */
                        {}
                        // /* REGEXP */
                        {}
                        // /* EXPLAIN */
                        {}
                        // /* INSTEAD */
                        {}
                        // /* ADD */
                        {}
                        // /* DATABASE */
                        {}
                        // /* AS */
                        {}
                        // /* SELECT */
                        {}
                        // /* TABLE */
                        {}
                        // /* LEFT */
                        {}
                        // /* THEN */
                        {}
                        // /* END */
                        {}
                        // /* DEFERRABLE */
                        {}
                        // /* ELSE */
                        {}
                        // /* EXCLUDE */
                        {}
                        // /* DELETE */
                        {}
                        // /* TEMPORARY */
                        {}
                        // /* TEMP */
                        {}
                        // /* OR */
                        {}
                        // /* ISNULL */
                        {}
                        // /* NULLS */
                        {}
                        // /* SAVEPOINT */
                        {}
                        // /* INTERSECT */
                        {}
                        // /* TIES */
                        {}
                        // /* NOTNULL */
                        {}
                        // /* NOT */
                        {}
                        // /* NO */
                        {}
                        // /* NULL */
                        {}
                        // /* LIKE */
                        {}
                        // /* EXCEPT */
                        {}
                        // /* TRANSACTION */
                        {}
                        // /* ACTION */
                        {}
                        // /* ON */
                        {}
                        // /* NATURAL */
                        {}
                        // /* ALTER */
                        {}
                        // /* RAISE */
                        {}
                        // /* EXCLUSIVE */
                        {}
                        // /* EXISTS */
                        {}
                        // /* CONSTRAINT */
                        {}
                        // /* INTO */
                        {}
                        // /* OFFSET */
                        {}
                        // /* OF */
                        {}
                        // /* SET */
                        {}
                        // /* TRIGGER */
                        {}
                        // /* RANGE */
                        {}
                        // /* GENERATED */
                        {}
                        // /* DETACH */
                        {}
                        // /* HAVING */
                        {}
                        // /* GLOB */
                        {}
                        // /* BEGIN */
                        {}
                        // /* INNER */
                        {}
                        // /* REFERENCES */
                        {}
                        // /* UNIQUE */
                        {}
                        // /* QUERY */
                        {}
                        // /* WITHOUT */
                        {}
                        // /* WITH */
                        {}
                        // /* OUTER */
                        {}
                        // /* RELEASE */
                        {}
                        // /* ATTACH */
                        {}
                        // /* BETWEEN */
                        {}
                        // /* NOTHING */
                        {}
                        // /* GROUPS */
                        {}
                        // /* GROUP */
                        {}
                        // /* CASCADE */
                        {}
                        // /* ASC */
                        {}
                        // /* DEFAULT */
                        {}
                        // /* CASE */
                        {}
                        // /* COLLATE */
                        {}
                        // /* CREATE */
                        {}
                        // /* CURRENT_DATE */
                        {}
                        // /* IMMEDIATE */
                        {}
                        // /* JOIN */
                        {}
                        // /* INSERT */
                        {}
                        // /* MATCH */
                        {}
                        // /* PLAN */
                        {}
                        // /* ANALYZE */
                        {}
                        // /* PRAGMA */
                        {}
                        // /* MATERIALIZED */
                        {}
                        // /* DEFERRED */
                        {}
                        // /* DISTINCT */
                        {}
                        // /* IS */
                        {}
                        // /* UPDATE */
                        {}
                        // /* VALUES */
                        {}
                        // /* VIRTUAL */
                        {}
                        // /* ALWAYS */
                        {}
                        // /* WHEN */
                        {}
                        // /* WHERE */
                        {}
                        // /* RECURSIVE */
                        {}
                        // /* ABORT */
                        {}
                        // /* AFTER */
                        {}
                        // /* RENAME */
                        {}
                        // /* AND */
                        {}
                        // /* DROP */
                        {}
                        // /* PARTITION */
                        {}
                        // /* AUTOINCREMENT */
                        {}
                        // /* TO */
                        {}
                        // /* IN */
                        {}
                        // /* CAST */
                        {}
                        // /* COLUMN */
                        {}
                        // /* COMMIT */
                        {}
                        // /* CONFLICT */
                        {}
                        // /* CROSS */
                        {}
                        // /* CURRENT_TIMESTAMP */
                        {}
                        // /* CURRENT_TIME */
                        {}
                        // /* CURRENT */
                        {}
                        // /* PRECEDING */
                        {}
                        // /* FAIL */
                        {}
                        // /* LAST */
                        {}
                        // /* FILTER */
                        {}
                        // /* REPLACE */
                        {}
                        // /* FIRST */
                        {}
                        // /* FOLLOWING */
                        {}
                        // /* FROM */
                        {}
                        // /* FULL */
                        {}
                        // /* LIMIT */
                        {}
                        // /* IF */
                        {}
                        // /* ORDER */
                        {}
                        // /* RESTRICT */
                        {}
                        // /* OTHERS */
                        {}
                        // /* OVER */
                        {}
                        // /* RETURNING */
                        {}
                        // /* RIGHT */
                        {}
                        // /* ROLLBACK */
                        {}
                        // /* ROWS */
                        {}
                        // /* ROW */
                        {}
                        // /* UNBOUNDED */
                        {}
                        // /* UNION */
                        {}
                        // /* USING */
                        {}
                        // /* VACUUM */
                        {}
                        // /* VIEW */
                        {}
                        // /* WINDOW */
                        {}
                        // /* DO */
                        {}
                        // /* BY */
                        {}
                        // /* INITIALLY */
                        {}
                        // /* ALL */
                        {}
                        // /* PRIMARY */
                        {}
                        unsafe {
                            *pType = ((unsafe {
                                *unsafe {
                                    unsafe { std::ptr::addr_of!(aKWCode.0) as *const u8 }
                                        .offset(i as isize)
                                }
                            }) as u32) as i32;
                        }
                        break '__slate_break_379;
                    }
                }
            }
        }
        i = ((unsafe {
            *unsafe { unsafe { std::ptr::addr_of!(aKWNext.0) as *const u8 }.offset(i as isize) }
        }) as u64) as i64;
    }
    return n;
}

// /*
// ** Return the id of the next token in string (*pz). Before returning, set
// ** (*pz) to point to the byte following the parsed token.
// */
fn getToken(mut pz: *mut *const u8) -> i32 {
    let mut z: *const u8 = unsafe { *pz };
    // /* Token type to return */
    let mut t: i32 = 0 as i32;
    '__slate_break_381: loop {
        let __v487: *const u8 = z;
        let __v488: *const u8 =
            unsafe { __v487.offset(sqlite3GetToken(z, std::ptr::addr_of_mut!(t)) as isize) };
        z = __v488;
        if !(t == (184 as i32) || t == (185 as i32)) {
            break;
        }
    }
    let __v489: bool;
    if t == (60 as i32)
        || t == (118 as i32)
        || t == (119 as i32)
        || t == (165 as i32)
        || t == (166 as i32)
    {
        __v489 = true as bool;
    } else {
        __v489 = (unsafe { sqlite3ParserFallback(t) }) == (60 as i32);
    }
    if __v489 {
        t = 60 as i32;
    }
    unsafe {
        *pz = z;
    }
    return t;
}

// /*
// ** The following three functions are called immediately after the tokenizer
// ** reads the keywords WINDOW, OVER and FILTER, respectively, to determine
// ** whether the token should be treated as a keyword or an SQL identifier.
// ** This cannot be handled by the usual lemon %fallback method, due to
// ** the ambiguity in some constructions. e.g.
// **
// **   SELECT sum(x) OVER ...
// **
// ** In the above, "OVER" might be a keyword, or it might be an alias for the
// ** sum(x) expression. If a "%fallback ID OVER" directive were added to
// ** grammar, then SQLite would always treat "OVER" as an alias, making it
// ** impossible to call a window-function without a FILTER clause.
// **
// ** WINDOW is treated as a keyword if:
// **
// **   * the following token is an identifier, or a keyword that can fallback
// **     to being an identifier, and
// **   * the token after than one is TK_AS.
// **
// ** OVER is a keyword if:
// **
// **   * the previous token was TK_RP, and
// **   * the next token is either TK_LP or an identifier.
// **
// ** FILTER is a keyword if:
// **
// **   * the previous token was TK_RP, and
// **   * the next token is TK_LP.
// */
fn analyzeWindowKeyword(mut z: *const u8) -> i32 {
    let mut t: i32 = 0 as i32;
    t = getToken(std::ptr::addr_of_mut!(z));
    if t != (60 as i32) {
        return 60 as i32;
    }
    t = getToken(std::ptr::addr_of_mut!(z));
    if t != (24 as i32) {
        return 60 as i32;
    }
    return 165 as i32;
}

fn analyzeOverKeyword(mut z: *const u8, mut lastToken: i32) -> i32 {
    if lastToken == (23 as i32) {
        let mut t: i32 = getToken(std::ptr::addr_of_mut!(z));
        if t == (22 as i32) || t == (60 as i32) {
            return 166 as i32;
        }
    }
    return 60 as i32;
}

fn analyzeFilterKeyword(mut z: *const u8, mut lastToken: i32) -> i32 {
    let __v490: bool;
    if lastToken == (23 as i32) {
        __v490 = getToken(std::ptr::addr_of_mut!(z)) == (22 as i32);
    } else {
        __v490 = false as bool;
    }
    if __v490 {
        return 167 as i32;
    }
    return 60 as i32;
}
