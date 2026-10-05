unsafe extern "C" {
    static mut sqlite3StrBINARY: [i8; 0];
    static mut sqlite3UpperToLower: [u8; 0];
    static mut sqlite3BuiltinFunctions: FuncDefHash;
    fn sqlite3HashInit(__v399: *mut Hash);
    fn sqlite3HashInsert(__v400: *mut Hash, pKey: *const i8, pData: *mut ()) -> *mut ();
    fn sqlite3HashFind(__v403: *const Hash, pKey: *const i8) -> *mut ();
    fn sqlite3HashClear(__v405: *mut Hash);
    fn memcpy(__dest: *mut (), __src: *const (), __n: u64) -> *mut ();
    fn memset(__s: *mut (), __c: i32, __n: u64) -> *mut ();
    fn sqlite3BtreeSchema(
        __v412: *mut Btree,
        __v413: i32,
        __v414: Option<unsafe extern "C-unwind" fn(*mut ())>,
    ) -> *mut ();
    fn sqlite3StrICmp(__v415: *const i8, __v416: *const i8) -> i32;
    fn sqlite3Strlen30(__v417: *const i8) -> i32;
    fn sqlite3DbMallocZero(__v418: *mut sqlite3, __v419: u64) -> *mut ();
    fn sqlite3DbStrDup(__v420: *mut sqlite3, __v421: *const i8) -> *mut i8;
    fn sqlite3DbFree(__v422: *mut sqlite3, __v423: *mut ());
    fn sqlite3ErrorMsg(__v424: *mut Parse, __v425: *const i8, ...);
    fn sqlite3DeleteTable(__v426: *mut sqlite3, __v427: *mut Table);
    fn sqlite3DeleteTrigger(__v437: *mut sqlite3, __v438: *mut Trigger);
    fn sqlite3ValueText(__v449: *mut sqlite3_value, __v450: u8) -> *const ();
    fn sqlite3ValueSetStr(
        __v451: *mut sqlite3_value,
        __v452: i32,
        __v453: *const (),
        __v454: u8,
        __v455: Option<unsafe extern "C-unwind" fn(*mut ())>,
    );
    fn sqlite3ValueFree(__v456: *mut sqlite3_value);
    fn sqlite3ValueNew(__v457: *mut sqlite3) -> *mut sqlite3_value;
    fn sqlite3ExpirePreparedStatements(__v458: *mut sqlite3, __v459: i32);
    fn sqlite3OomFault(__v467: *mut sqlite3) -> *mut ();
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
    trace: __SlateRecord157,
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
    u1: __SlateRecord158,
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
    __slate_bits_0: __slate_bits::__SlateBits62U0,
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
    u: __SlateRecord168,
    pLeft: *mut Expr,
    pRight: *mut Expr,
    x: __SlateRecord169,
    nHeight: i32,
    iTable: i32,
    iColumn: i16,
    iAgg: i16,
    w: __SlateRecord170,
    pAggInfo: *mut AggInfo,
    y: __SlateRecord171,
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
    u: __SlateRecord159,
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
    __slate_bits_0: __slate_bits::__SlateBits88U0,
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
    __slate_bits_0: __slate_bits::__SlateBits100U0,
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
    u1: __SlateRecord184,
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
    fg: __SlateRecord178,
    iCursor: i32,
    colUsed: u64,
    u1: __SlateRecord179,
    u2: __SlateRecord180,
    u3: __SlateRecord181,
    u4: __SlateRecord182,
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
    u: __SlateRecord160,
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
    __slate_bits_0: __slate_bits::__SlateBits156U0,
    azInit: *mut *const i8,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord157 {
    xLegacy: Option<unsafe extern "C-unwind" fn(*mut (), *const i8)>,
    xV2: Option<unsafe extern "C-unwind" fn(u32, *mut (), *mut (), *mut ()) -> i32>,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord158 {
    isInterrupted: i32,
    notUsed1: f64,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord159 {
    pHash: *mut FuncDef,
    pDestructor: *mut FuncDestructor,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord160 {
    tab: __SlateRecord161,
    view: __SlateRecord162,
    vtab: __SlateRecord163,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord161 {
    addColOffset: i32,
    pFKey: *mut FKey,
    pDfltList: *mut ExprList,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord162 {
    pSelect: *mut Select,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord163 {
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
union __SlateRecord168 {
    zToken: *mut i8,
    iValue: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord169 {
    pList: *mut ExprList,
    pSelect: *mut Select,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord170 {
    iJoin: i32,
    iOfst: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord171 {
    pTab: *mut Table,
    pWin: *mut Window,
    nReg: i32,
    sub: __SlateRecord172,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord172 {
    iAddr: i32,
    regReturn: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct ExprList_item {
    pExpr: *mut Expr,
    zEName: *mut i8,
    fg: __SlateRecord174,
    u: __SlateRecord175,
}

#[repr(C, align(4))]
#[derive(Clone, Copy)]
struct __SlateRecord174 {
    sortFlags: u8,
    __slate_bits_0: __slate_bits::__SlateBits174U0,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord175 {
    x: __SlateRecord176,
    iConstExprReg: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord176 {
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
struct __SlateRecord178 {
    jointype: u8,
    __slate_bits_0: __slate_bits::__SlateBits178U0,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord179 {
    zIndexedBy: *mut i8,
    pFuncArg: *mut ExprList,
    nRow: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord180 {
    pIBIndex: *mut Index,
    pCteUse: *mut CteUse,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord181 {
    pOn: *mut Expr,
    pUsing: *mut IdList,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord182 {
    pSchema: *mut Schema,
    zDatabase: *mut i8,
    pSubq: *mut Subquery,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord184 {
    cr: __SlateRecord185,
    d: __SlateRecord186,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord185 {
    addrCrTab: i32,
    regRowid: i32,
    regRoot: i32,
    constraintName: Token,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord186 {
    pReturning: *mut Returning,
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
    pub struct __SlateBits62U0 {
        #[bits(4)]
        pub notNull: u32,
        #[bits(4)]
        pub eCType: u32,
    }
    #[bitfields::bitfield([u8; 2], c_names = true, new = false, from_into_bits = false, from_traits = false, default = false, debug = false, builder = false, bit_ops = false)]
    pub struct __SlateBits174U0 {
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
    pub struct __SlateBits178U0 {
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
    pub struct __SlateBits88U0 {
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
    pub struct __SlateBits156U0 {
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
    pub struct __SlateBits100U0 {
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

static mut aEnc: [u8; 3] = [
    ((3 as i32) as i8) as u8,
    ((2 as i32) as i8) as u8,
    ((1 as i32) as i8) as u8,
];

// /* The function we are evaluating for match quality */
// /* Desired number of arguments.  (-1)==any */
// /* Desired text encoding */
// /*
// ** Search a FuncDefHash for a function with the given name.  Return
// ** a pointer to the matching FuncDef if found, or 0 if there is no match.
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3FunctionSearch(mut h: i32, mut zFunc: *const i8) -> *mut FuncDef {
    let mut p: *mut FuncDef = unsafe { std::mem::zeroed() };
    p = unsafe {
        *unsafe {
            unsafe { std::ptr::addr_of_mut!(sqlite3BuiltinFunctions.a) as *mut *mut FuncDef }
                .offset(h as isize)
        }
    };
    '__slate_break_470: while p != std::ptr::null_mut::<FuncDef>() {
        0 as i32;
        if (unsafe { sqlite3StrICmp(unsafe { (*p).zName }, zFunc) }) == (0 as i32) {
            return p;
        }
        p = unsafe { (*p).u.pHash };
    }
    return std::ptr::null_mut::<FuncDef>();
}

// /* Hash of the name */
// /* Name of function */
// /*
// ** Insert a new FuncDef into a FuncDefHash hash table.
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3InsertBuiltinFuncs(mut aDef: *mut FuncDef, mut nDef: i32) {
    let mut i: i32 = 0 as i32;
    i = 0 as i32;
    '__slate_break_471: loop {
        if !(i < nDef) {
            break;
        }
        let mut pOther: *mut FuncDef = unsafe { std::mem::zeroed() };
        let mut zName: *const i8 = unsafe { (*unsafe { aDef.offset(i as isize) }).zName };
        let mut nName: i32 = unsafe { sqlite3Strlen30(zName) };
        let mut h: i32 = (((unsafe { *unsafe { zName.offset((0 as i32) as isize) } }) as i32)
            + nName)
            % (23 as i32);
        0 as i32;
        pOther = sqlite3FunctionSearch(h, zName);
        if pOther != std::ptr::null_mut::<FuncDef>() {
            0 as i32;
            unsafe {
                (*unsafe { aDef.offset(i as isize) }).pNext = unsafe { (*pOther).pNext };
            }
            unsafe {
                (*pOther).pNext = unsafe { aDef.offset(i as isize) };
            }
        } else {
            unsafe {
                (*unsafe { aDef.offset(i as isize) }).pNext = std::ptr::null_mut::<FuncDef>();
            }
            unsafe {
                (*unsafe { aDef.offset(i as isize) }).u.pHash = unsafe {
                    *unsafe {
                        unsafe {
                            std::ptr::addr_of_mut!(sqlite3BuiltinFunctions.a) as *mut *mut FuncDef
                        }
                        .offset(h as isize)
                    }
                };
            }
            unsafe {
                *unsafe {
                    unsafe {
                        std::ptr::addr_of_mut!(sqlite3BuiltinFunctions.a) as *mut *mut FuncDef
                    }
                    .offset(h as isize)
                } = unsafe { aDef.offset(i as isize) };
            }
        }
        let __v477: i32 = i;
        let __v478: i32 = __v477 + (1 as i32);
        i = __v478;
    }
}

// /* List of global functions to be inserted */
// /* Length of the apDef[] list */
// /*
// ** Locate a user function given a name, a number of arguments and a flag
// ** indicating whether the function prefers UTF-16 over UTF-8.  Return a
// ** pointer to the FuncDef structure that defines that function, or return
// ** NULL if the function does not exist.
// **
// ** If the createFlag argument is true, then a new (blank) FuncDef
// ** structure is created and liked into the "db" structure if a
// ** no matching function previously existed.
// **
// ** If nArg is -2, then the first valid function found is returned.  A
// ** function is valid if xSFunc is non-zero.  The nArg==(-2)
// ** case is used to see if zName is a valid function name for some number
// ** of arguments.  If nArg is -2, then createFlag must be 0.
// **
// ** If createFlag is false, then a function with the required name and
// ** number of arguments may be returned even if the eTextRep flag does not
// ** match that requested.
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3FindFunction(
    mut db: *mut sqlite3,
    mut zName: *const i8,
    mut nArg: i32,
    mut enc: u8,
    mut createFlag: u8,
) -> *mut FuncDef {
    // /* Iterator variable */
    let mut p: *mut FuncDef = unsafe { std::mem::zeroed() };
    // /* Best match found so far */
    let mut pBest: *mut FuncDef = std::ptr::null_mut::<FuncDef>();
    // /* Score of best match */
    let mut bestScore: i32 = 0 as i32;
    // /* Hash value */
    let mut h: i32 = 0 as i32;
    // /* Length of the name */
    let mut nName: i32 = 0 as i32;
    0 as i32;
    0 as i32;
    nName = unsafe { sqlite3Strlen30(zName) };
    // /* First search for a match amongst the application-defined functions.
    //   */
    p = (unsafe {
        sqlite3HashFind(
            (unsafe { std::ptr::addr_of_mut!((*db).aFunc) }) as *const Hash,
            zName,
        )
    }) as *mut FuncDef;
    '__slate_break_472: while p != std::ptr::null_mut::<FuncDef>() {
        let mut score: i32 = matchQuality(p, nArg, enc);
        if score > bestScore {
            pBest = p;
            bestScore = score;
        }
        p = unsafe { (*p).pNext };
    }
    // /* If no match is found, search the built-in functions.
    //   **
    //   ** If the DBFLAG_PreferBuiltin flag is set, then search the built-in
    //   ** functions even if a prior app-defined function was found.  And give
    //   ** priority to built-in functions.
    //   **
    //   ** Except, if createFlag is true, that means that we are trying to
    //   ** install a new function.  Whatever FuncDef structure is returned it will
    //   ** have fields overwritten with new information appropriate for the
    //   ** new function.  But the FuncDefs for built-in functions are read-only.
    //   ** So we must not search for built-ins when creating a new function.
    //   */
    if !(createFlag != (0 as u8))
        && (pBest == std::ptr::null_mut::<FuncDef>()
            || (unsafe { (*db).mDbFlags }) & ((2 as i32) as u32) != ((0 as i32) as u32))
    {
        bestScore = 0 as i32;
        h = ((((unsafe {
            *unsafe {
                unsafe { std::ptr::addr_of!(sqlite3UpperToLower) as *const u8 }.offset(
                    ((((unsafe { *unsafe { zName.offset((0 as i32) as isize) } }) as u8) as u32)
                        as i32) as isize,
                )
            }
        }) as u32) as i32)
            + nName)
            % (23 as i32);
        p = sqlite3FunctionSearch(h, zName);
        '__slate_break_473: while p != std::ptr::null_mut::<FuncDef>() {
            let mut score: i32 = matchQuality(p, nArg, enc);
            if score > bestScore {
                pBest = p;
                bestScore = score;
            }
            p = unsafe { (*p).pNext };
        }
    }
    // /* If the createFlag parameter is true and the search did not reveal an
    //   ** exact match for the name, number of arguments and encoding, then add a
    //   ** new entry to the hash table and return it.
    //   */
    let __v479: bool;
    if createFlag != (0 as u8) && bestScore < (6 as i32) {
        let __v480: *mut FuncDef = (unsafe {
            sqlite3DbMallocZero(
                db,
                (72 as u64)
                    .wrapping_add((nName as i64) as u64)
                    .wrapping_add(((1 as i32) as i64) as u64),
            )
        }) as *mut FuncDef;
        pBest = __v480;
        __v479 = __v480 != std::ptr::null_mut::<FuncDef>();
    } else {
        __v479 = false as bool;
    }
    if __v479 {
        let mut pOther: *mut FuncDef = unsafe { std::mem::zeroed() };
        let mut z: *mut u8 = unsafe { std::mem::zeroed() };
        unsafe {
            (*pBest).zName = (unsafe { pBest.offset((1 as i32) as isize) }) as *const i8;
        }
        unsafe {
            (*pBest).nArg = ((nArg as i16) as u16) as i16;
        }
        unsafe {
            (*pBest).funcFlags = enc as u32;
        }
        unsafe {
            memcpy(
                ((unsafe { pBest.offset((1 as i32) as isize) }) as *mut i8) as *mut (),
                zName as *const (),
                ((nName + (1 as i32)) as i64) as u64,
            )
        };
        z = (unsafe { (*pBest).zName }) as *mut u8;
        '__slate_break_474: while (unsafe { *z }) != (0 as u8) {
            unsafe {
                *z = unsafe {
                    *unsafe {
                        unsafe { std::ptr::addr_of!(sqlite3UpperToLower) as *const u8 }
                            .offset((((unsafe { *z }) as u32) as i32) as isize)
                    }
                };
            }
            let __v481: *mut u8 = z;
            let __v482: *mut u8 = unsafe { __v481.offset((1 as i32) as isize) };
            z = __v482;
        }
        pOther = (unsafe {
            sqlite3HashInsert(
                unsafe { std::ptr::addr_of_mut!((*db).aFunc) },
                unsafe { (*pBest).zName },
                pBest as *mut (),
            )
        }) as *mut FuncDef;
        if pOther == pBest {
            unsafe { sqlite3DbFree(db, pBest as *mut ()) };
            unsafe { sqlite3OomFault(db) };
            return std::ptr::null_mut::<FuncDef>();
        } else {
            unsafe {
                (*pBest).pNext = pOther;
            }
        }
    }
    if pBest != std::ptr::null_mut::<FuncDef>()
        && ((unsafe { (*pBest).xSFunc }) != None || createFlag != (0 as u8))
    {
        return pBest;
    }
    return std::ptr::null_mut::<FuncDef>();
}

// /* Database connection */
// /* Name of the collating sequence */
// /* Create a new entry if true */
// /*
// ** Parameter zName points to a UTF-8 encoded string nName bytes long.
// ** Return the CollSeq* pointer for the collation sequence named zName
// ** for the encoding 'enc' from the database 'db'.
// **
// ** If the entry specified is not found and 'create' is true, then create a
// ** new entry.  Otherwise return NULL.
// **
// ** A separate function sqlite3LocateCollSeq() is a wrapper around
// ** this routine.  sqlite3LocateCollSeq() invokes the collation factory
// ** if necessary and generates an error message if the collating sequence
// ** cannot be found.
// **
// ** See also: sqlite3LocateCollSeq(), sqlite3GetCollSeq()
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3FindCollSeq(
    mut db: *mut sqlite3,
    mut enc: u8,
    mut zName: *const i8,
    mut create: i32,
) -> *mut CollSeq {
    let mut pColl: *mut CollSeq = unsafe { std::mem::zeroed() };
    0 as i32;
    0 as i32;
    if zName != std::ptr::null::<i8>() {
        pColl = findCollSeqEntry(db, zName, create);
        if pColl != std::ptr::null_mut::<CollSeq>() {
            let __v483: *mut CollSeq = pColl;
            let __v484: *mut CollSeq =
                unsafe { __v483.offset((((enc as u32) as i32) - (1 as i32)) as isize) };
            pColl = __v484;
        }
    } else {
        pColl = unsafe { (*db).pDfltColl };
    }
    return pColl;
}

// /* Parsing context */
// /* The desired encoding for the collating sequence */
// /* Collating sequence with native encoding, or NULL */
// /* Collating sequence name */
// /*
// ** This function returns the collation sequence for database native text
// ** encoding identified by the string zName.
// **
// ** If the requested collation sequence is not available, or not available
// ** in the database native encoding, the collation factory is invoked to
// ** request it. If the collation factory does not supply such a sequence,
// ** and the sequence is available in another text encoding, then that is
// ** returned instead.
// **
// ** If no versions of the requested collations sequence are available, or
// ** another error occurs, NULL is returned and an error message written into
// ** pParse.
// **
// ** This routine is a wrapper around sqlite3FindCollSeq().  This routine
// ** invokes the collation factory if the named collation cannot be found
// ** and generates an error message.
// **
// ** See also: sqlite3FindCollSeq(), sqlite3GetCollSeq()
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3LocateCollSeq(
    mut pParse: *mut Parse,
    mut zName: *const i8,
) -> *mut CollSeq {
    let mut db: *mut sqlite3 = unsafe { (*pParse).db };
    let mut enc: u8 = unsafe { (*db).enc };
    let mut initbusy: u8 = unsafe { (*db).init.busy };
    let mut pColl: *mut CollSeq = unsafe { std::mem::zeroed() };
    pColl = sqlite3FindCollSeq(db, enc, zName, (initbusy as u32) as i32);
    if !(initbusy != (0 as u8))
        && (!(pColl != std::ptr::null_mut::<CollSeq>()) || !((unsafe { (*pColl).xCmp }) != None))
    {
        pColl = sqlite3GetCollSeq(pParse, enc, pColl, zName);
    }
    return pColl;
}

// /* Database connection to search */
// /* Desired text encoding */
// /* Name of the collating sequence.  Might be NULL */
// /* True to create CollSeq if doesn't already exist */
// /*
// ** Change the text encoding for a database connection. This means that
// ** the pDfltColl must change as well.
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3SetTextEncoding(mut db: *mut sqlite3, mut enc: u8) {
    0 as i32;
    unsafe {
        (*db).enc = enc;
    }
    // /* EVIDENCE-OF: R-08308-17224 The default collating function for all
    //   ** strings is BINARY.
    //   */
    unsafe {
        (*db).pDfltColl = sqlite3FindCollSeq(
            db,
            enc,
            unsafe { std::ptr::addr_of!(sqlite3StrBINARY) as *const i8 },
            0 as i32,
        );
    }
    unsafe { sqlite3ExpirePreparedStatements(db, 1 as i32) };
}

// /*
// ** This routine is called on a collation sequence before it is used to
// ** check that it is defined. An undefined collation sequence exists when
// ** a database is loaded that contains references to collation sequences
// ** that have not been defined by sqlite3_create_collation() etc.
// **
// ** If required, this routine calls the 'collation needed' callback to
// ** request a definition of the collating sequence. If this doesn't work,
// ** an equivalent collating sequence that uses a text encoding different
// ** from the main database is substituted, if one is available.
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3CheckCollSeq(mut pParse: *mut Parse, mut pColl: *mut CollSeq) -> i32 {
    if pColl != std::ptr::null_mut::<CollSeq>() && (unsafe { (*pColl).xCmp }) == None {
        let mut zName: *const i8 = (unsafe { (*pColl).zName }) as *const i8;
        let mut db: *mut sqlite3 = unsafe { (*pParse).db };
        let mut p: *mut CollSeq = sqlite3GetCollSeq(pParse, unsafe { (*db).enc }, pColl, zName);
        if !(p != std::ptr::null_mut::<CollSeq>()) {
            return 1 as i32;
        }
        0 as i32;
    }
    return 0 as i32;
}

// /*
// ** This function is responsible for invoking the collation factory callback
// ** or substituting a collation sequence of a different encoding when the
// ** requested collation sequence is not available in the desired encoding.
// **
// ** If it is not NULL, then pColl must point to the database native encoding
// ** collation sequence with name zName, length nName.
// **
// ** The return value is either the collation sequence to be used in database
// ** db for collation type name zName, length nName, or NULL, if no collation
// ** sequence can be found.  If no collation is found, leave an error message.
// **
// ** See also: sqlite3LocateCollSeq(), sqlite3FindCollSeq()
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3GetCollSeq(
    mut pParse: *mut Parse,
    mut enc: u8,
    mut pColl: *mut CollSeq,
    mut zName: *const i8,
) -> *mut CollSeq {
    let mut p: *mut CollSeq = unsafe { std::mem::zeroed() };
    let mut db: *mut sqlite3 = unsafe { (*pParse).db };
    p = pColl;
    if !(p != std::ptr::null_mut::<CollSeq>()) {
        p = sqlite3FindCollSeq(db, enc, zName, 0 as i32);
    }
    if !(p != std::ptr::null_mut::<CollSeq>()) || !((unsafe { (*p).xCmp }) != None) {
        // /* No collation sequence of this type for this encoding is registered.
        //     ** Call the collation factory to see if it can supply us with one.
        //     */
        callCollNeeded(db, (enc as u32) as i32, zName);
        p = sqlite3FindCollSeq(db, enc, zName, 0 as i32);
    }
    let __v485: bool;
    if p != std::ptr::null_mut::<CollSeq>() && !((unsafe { (*p).xCmp }) != None) {
        __v485 = synthCollSeq(db, p) != (0 as i32);
    } else {
        __v485 = false as bool;
    }
    if __v485 {
        p = std::ptr::null_mut::<CollSeq>();
    }
    0 as i32;
    if p == std::ptr::null_mut::<CollSeq>() {
        unsafe {
            sqlite3ErrorMsg(
                pParse,
                (b"no such collation sequence: %s\0".as_ptr() as *mut i8) as *const i8,
                zName,
            )
        };
        unsafe {
            (*pParse).rc = (1 as i32) | (1 as i32) << (8 as i32);
        }
    }
    return p;
}

// /* An open database */
// /* Name of the function.  zero-terminated */
// /* Number of arguments.  -1 means any number */
// /* Preferred text encoding */
// /* Create new entry if true and does not otherwise exist */
// /*
// ** Free all resources held by the schema structure. The void* argument points
// ** at a Schema struct. This function does not call sqlite3DbFree(db, ) on the
// ** pointer itself, it just cleans up subsidiary resources (i.e. the contents
// ** of the schema hash tables).
// **
// ** The Schema.cache_size variable is not cleared.
// */
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.callback.sqlite3SchemaClear")]
extern "C-unwind" fn sqlite3SchemaClear(mut p: *mut ()) {
    let mut temp1: Hash = unsafe { std::mem::zeroed() };
    let mut temp2: Hash = unsafe { std::mem::zeroed() };
    let mut pElem: *mut HashElem = unsafe { std::mem::zeroed() };
    let mut pSchema: *mut Schema = p as *mut Schema;
    let mut xdb: sqlite3 = unsafe { std::mem::zeroed() };
    unsafe { memset(std::ptr::addr_of_mut!(xdb) as *mut (), 0 as i32, 800 as u64) };
    temp1 = unsafe { (*pSchema).tblHash };
    temp2 = unsafe { (*pSchema).trigHash };
    unsafe { sqlite3HashInit(unsafe { std::ptr::addr_of_mut!((*pSchema).trigHash) }) };
    unsafe { sqlite3HashClear(unsafe { std::ptr::addr_of_mut!((*pSchema).idxHash) }) };
    pElem = unsafe { (*std::ptr::addr_of_mut!(temp2)).first };
    '__slate_break_475: while pElem != std::ptr::null_mut::<HashElem>() {
        unsafe {
            sqlite3DeleteTrigger(
                std::ptr::addr_of_mut!(xdb),
                (unsafe { (*pElem).data }) as *mut Trigger,
            )
        };
        pElem = unsafe { (*pElem).next };
    }
    unsafe { sqlite3HashClear(std::ptr::addr_of_mut!(temp2)) };
    unsafe { sqlite3HashInit(unsafe { std::ptr::addr_of_mut!((*pSchema).tblHash) }) };
    pElem = unsafe { (*std::ptr::addr_of_mut!(temp1)).first };
    '__slate_break_476: while pElem != std::ptr::null_mut::<HashElem>() {
        let mut pTab: *mut Table = (unsafe { (*pElem).data }) as *mut Table;
        unsafe { sqlite3DeleteTable(std::ptr::addr_of_mut!(xdb), pTab) };
        pElem = unsafe { (*pElem).next };
    }
    unsafe { sqlite3HashClear(std::ptr::addr_of_mut!(temp1)) };
    unsafe { sqlite3HashClear(unsafe { std::ptr::addr_of_mut!((*pSchema).fkeyHash) }) };
    unsafe {
        (*pSchema).pSeqTab = std::ptr::null_mut::<Table>();
    }
    if (((unsafe { (*pSchema).schemaFlags }) as u32) as i32) & (1 as i32) != (0 as i32) {
        let __v486: *mut Schema = pSchema;
        let __v487: i32 = unsafe { (*__v486).iGeneration };
        let __v488: i32 = __v487 + (1 as i32);
        unsafe {
            (*__v486).iGeneration = __v488;
        }
    }
    let __v489: *mut Schema = pSchema;
    let __v490: u16 = unsafe { (*__v489).schemaFlags };
    let __v491: u16 = ((((__v490 as u32) as i32) & !((1 as i32) | (8 as i32))) as i16) as u16;
    unsafe {
        (*__v489).schemaFlags = __v491;
    }
}

// /*
// ** Find and return the schema associated with a BTree.  Create
// ** a new one if necessary.
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3SchemaGet(mut db: *mut sqlite3, mut pBt: *mut Btree) -> *mut Schema {
    let mut p: *mut Schema = unsafe { std::mem::zeroed() };
    if pBt != std::ptr::null_mut::<Btree>() {
        p = (unsafe {
            sqlite3BtreeSchema(pBt, ((120 as u64) as u32) as i32, Some(sqlite3SchemaClear))
        }) as *mut Schema;
    } else {
        p = (unsafe { sqlite3DbMallocZero(std::ptr::null_mut::<sqlite3>(), 120 as u64) })
            as *mut Schema;
    }
    if !(p != std::ptr::null_mut::<Schema>()) {
        unsafe { sqlite3OomFault(db) };
    } else {
        if (0 as i32) == (((unsafe { (*p).file_format }) as u32) as i32) {
            unsafe { sqlite3HashInit(unsafe { std::ptr::addr_of_mut!((*p).tblHash) }) };
            unsafe { sqlite3HashInit(unsafe { std::ptr::addr_of_mut!((*p).idxHash) }) };
            unsafe { sqlite3HashInit(unsafe { std::ptr::addr_of_mut!((*p).trigHash) }) };
            unsafe { sqlite3HashInit(unsafe { std::ptr::addr_of_mut!((*p).fkeyHash) }) };
            unsafe {
                (*p).enc = ((1 as i32) as i8) as u8;
            }
        }
    }
    return p;
}

// /*
// ** 2005 May 23
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
// ** This file contains functions used to access the internal hash tables
// ** of user defined functions and collation sequences.
// */
// /*
// ** Invoke the 'collation needed' callback to request a collation sequence
// ** in the encoding enc of name zName, length nName.
// */
fn callCollNeeded(mut db: *mut sqlite3, mut enc: i32, mut zName: *const i8) {
    0 as i32;
    if (unsafe { (*db).xCollNeeded }) != None {
        let mut zExternal: *mut i8 = unsafe { sqlite3DbStrDup(db, zName) };
        if !(zExternal != std::ptr::null_mut::<i8>()) {
            return;
        }
        unsafe {
            unsafe { (*db).xCollNeeded }.unwrap()(
                unsafe { (*db).pCollNeededArg },
                db,
                enc,
                zExternal as *const i8,
            )
        };
        unsafe { sqlite3DbFree(db, zExternal as *mut ()) };
    }
    if (unsafe { (*db).xCollNeeded16 }) != None {
        let mut zExternal: *const i8 = unsafe { std::mem::zeroed() };
        let mut pTmp: *mut sqlite3_value = unsafe { sqlite3ValueNew(db) };
        unsafe {
            sqlite3ValueSetStr(
                pTmp,
                -(1 as i32),
                zName as *const (),
                ((1 as i32) as i8) as u8,
                None,
            )
        };
        zExternal = (unsafe { sqlite3ValueText(pTmp, ((2 as i32) as i8) as u8) }) as *const i8;
        if zExternal != std::ptr::null::<i8>() {
            unsafe {
                unsafe { (*db).xCollNeeded16 }.unwrap()(
                    unsafe { (*db).pCollNeededArg },
                    db,
                    ((unsafe { (*db).enc }) as u32) as i32,
                    zExternal as *const (),
                )
            };
        }
        unsafe { sqlite3ValueFree(pTmp) };
    }
}

// /*
// ** This routine is called if the collation factory fails to deliver a
// ** collation function in the best encoding but there may be other versions
// ** of this collation function (for other text encodings) available. Use one
// ** of these instead if they exist. Avoid a UTF-8 <-> UTF-16 conversion if
// ** possible.
// */
fn synthCollSeq(mut db: *mut sqlite3, mut pColl: *mut CollSeq) -> i32 {
    let mut pColl2: *mut CollSeq = unsafe { std::mem::zeroed() };
    let mut z: *mut i8 = unsafe { (*pColl).zName };
    let mut i: i32 = 0 as i32;
    i = 0 as i32;
    '__slate_break_468: loop {
        if !(i < (3 as i32)) {
            break;
        }
        pColl2 = sqlite3FindCollSeq(
            db,
            unsafe {
                *unsafe { unsafe { std::ptr::addr_of!(aEnc) as *const u8 }.offset(i as isize) }
            },
            z as *const i8,
            0 as i32,
        );
        if (unsafe { (*pColl2).xCmp }) != None {
            unsafe { memcpy(pColl as *mut (), pColl2 as *const (), 40 as u64) };
            // /* Do not copy the destructor */
            unsafe {
                (*pColl).xDel = None;
            }
            return 0 as i32;
        }
        let __v492: i32 = i;
        let __v493: i32 = __v492 + (1 as i32);
        i = __v493;
    }
    return 1 as i32;
}

// /*
// ** Locate and return an entry from the db.aCollSeq hash table. If the entry
// ** specified by zName and nName is not found and parameter 'create' is
// ** true, then create a new entry. Otherwise return NULL.
// **
// ** Each pointer stored in the sqlite3.aCollSeq hash table contains an
// ** array of three CollSeq structures. The first is the collation sequence
// ** preferred for UTF-8, the second UTF-16le, and the third UTF-16be.
// **
// ** Stored immediately after the three collation sequences is a copy of
// ** the collation sequence name. A pointer to this string is stored in
// ** each collation sequence structure.
// */
fn findCollSeqEntry(mut db: *mut sqlite3, mut zName: *const i8, mut create: i32) -> *mut CollSeq {
    let mut pColl: *mut CollSeq = unsafe { std::mem::zeroed() };
    pColl = (unsafe {
        sqlite3HashFind(
            (unsafe { std::ptr::addr_of_mut!((*db).aCollSeq) }) as *const Hash,
            zName,
        )
    }) as *mut CollSeq;
    if std::ptr::null_mut::<CollSeq>() == pColl && create != (0 as i32) {
        let mut nName: i32 = (unsafe { sqlite3Strlen30(zName) }) + (1 as i32);
        pColl = (unsafe {
            sqlite3DbMallocZero(
                db,
                (((3 as i32) as i64) as u64)
                    .wrapping_mul(40 as u64)
                    .wrapping_add((nName as i64) as u64),
            )
        }) as *mut CollSeq;
        if pColl != std::ptr::null_mut::<CollSeq>() {
            let mut pDel: *mut CollSeq = std::ptr::null_mut::<CollSeq>();
            unsafe {
                (*unsafe { pColl.offset((0 as i32) as isize) }).zName =
                    (unsafe { pColl.offset((3 as i32) as isize) }) as *mut i8;
            }
            unsafe {
                (*unsafe { pColl.offset((0 as i32) as isize) }).enc = ((1 as i32) as i8) as u8;
            }
            unsafe {
                (*unsafe { pColl.offset((1 as i32) as isize) }).zName =
                    (unsafe { pColl.offset((3 as i32) as isize) }) as *mut i8;
            }
            unsafe {
                (*unsafe { pColl.offset((1 as i32) as isize) }).enc = ((2 as i32) as i8) as u8;
            }
            unsafe {
                (*unsafe { pColl.offset((2 as i32) as isize) }).zName =
                    (unsafe { pColl.offset((3 as i32) as isize) }) as *mut i8;
            }
            unsafe {
                (*unsafe { pColl.offset((2 as i32) as isize) }).enc = ((3 as i32) as i8) as u8;
            }
            unsafe {
                memcpy(
                    (unsafe { (*unsafe { pColl.offset((0 as i32) as isize) }).zName }) as *mut (),
                    zName as *const (),
                    (nName as i64) as u64,
                )
            };
            pDel = (unsafe {
                sqlite3HashInsert(
                    unsafe { std::ptr::addr_of_mut!((*db).aCollSeq) },
                    (unsafe { (*unsafe { pColl.offset((0 as i32) as isize) }).zName }) as *const i8,
                    pColl as *mut (),
                )
            }) as *mut CollSeq;
            // /* If a malloc() failure occurred in sqlite3HashInsert(), it will
            //       ** return the pColl pointer to be deleted (because it wasn't added
            //       ** to the hash table).
            //       */
            0 as i32;
            if pDel != std::ptr::null_mut::<CollSeq>() {
                unsafe { sqlite3OomFault(db) };
                unsafe { sqlite3DbFree(db, pDel as *mut ()) };
                pColl = std::ptr::null_mut::<CollSeq>();
            }
        }
    }
    return pColl;
}

// /* During the search for the best function definition, this procedure
// ** is called to test how well the function passed as the first argument
// ** matches the request for a function with nArg arguments in a system
// ** that uses encoding enc. The value returned indicates how well the
// ** request is matched. A higher value indicates a better match.
// **
// ** If nArg is -1 that means to only return a match (non-zero) if p->nArg
// ** is also -1.  In other words, we are searching for a function that
// ** takes a variable number of arguments.
// **
// ** If nArg is -2 that means that we are searching for any function
// ** regardless of the number of arguments it uses, so return a positive
// ** match score for any
// **
// ** The returned value is always between 0 and 6, as follows:
// **
// ** 0: Not a match.
// ** 1: UTF8/16 conversion required and function takes any number of arguments.
// ** 2: UTF16 byte order change required and function takes any number of args.
// ** 3: encoding matches and function takes any number of arguments
// ** 4: UTF8/16 conversion required - argument count matches exactly
// ** 5: UTF16 byte order conversion required - argument count matches exactly
// ** 6: Perfect match:  encoding and argument count match exactly.
// **
// ** If nArg==(-2) then any function with a non-null xSFunc is
// ** a perfect match and any function with xSFunc NULL is
// ** a non-match.
// */
// /* The score for a perfect match */
fn matchQuality(mut p: *mut FuncDef, mut nArg: i32, mut enc: u8) -> i32 {
    let mut r#match: i32 = 0 as i32;
    0 as i32;
    0 as i32;
    // /* Wrong number of arguments means "no match" */
    if ((unsafe { (*p).nArg }) as i32) != nArg {
        if nArg == -(2 as i32) {
            return if (unsafe { (*p).xSFunc }) == None {
                0 as i32
            } else {
                6 as i32
            };
        }
        if ((unsafe { (*p).nArg }) as i32) >= (0 as i32) {
            return 0 as i32;
        }
        // /* Special p->nArg values available to built-in functions only:
        //     **    -3     1 or more arguments required
        //     **    -4     2 or more arguments required
        //     */
        if ((unsafe { (*p).nArg }) as i32) < -(2 as i32)
            && nArg < -(2 as i32) - ((unsafe { (*p).nArg }) as i32)
        {
            return 0 as i32;
        }
    }
    // /* Give a better score to a function with a specific number of arguments
    //   ** than to function that accepts any number of arguments. */
    if ((unsafe { (*p).nArg }) as i32) == nArg {
        r#match = 4 as i32;
    } else {
        r#match = 1 as i32;
    }
    // /* Bonus points if the text encoding matches */
    if (((enc as u32) as i32) as u32) == (unsafe { (*p).funcFlags }) & ((3 as i32) as u32) {
        // /* Exact encoding match */
        let __v494: i32 = r#match;
        let __v495: i32 = __v494 + (2 as i32);
        r#match = __v495;
    } else {
        if (((enc as u32) as i32) as u32) & unsafe { (*p).funcFlags } & ((2 as i32) as u32)
            != ((0 as i32) as u32)
        {
            // /* Both are UTF16, but with different byte orders */
            let __v496: i32 = r#match;
            let __v497: i32 = __v496 + (1 as i32);
            r#match = __v497;
        }
    }
    return r#match;
}
