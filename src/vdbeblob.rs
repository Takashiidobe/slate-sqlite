unsafe extern "C" {
    fn sqlite3_errmsg(__v429: *mut sqlite3) -> *const i8;
    fn sqlite3_step(__v430: *mut sqlite3_stmt) -> i32;
    fn sqlite3_finalize(pStmt: *mut sqlite3_stmt) -> i32;
    fn sqlite3_mutex_enter(__v451: *mut sqlite3_mutex);
    fn sqlite3_mutex_leave(__v452: *mut sqlite3_mutex);
    fn sqlite3BtreePayloadChecked(
        __v453: *mut BtCursor,
        offset: u32,
        amt: u32,
        __v456: *mut (),
    ) -> i32;
    fn sqlite3BtreePutData(__v457: *mut BtCursor, offset: u32, amt: u32, __v460: *mut ()) -> i32;
    fn sqlite3BtreeIncrblobCursor(__v461: *mut BtCursor);
    fn sqlite3BtreeEnterAll(__v462: *mut sqlite3);
    fn sqlite3BtreeEnterCursor(__v463: *mut BtCursor);
    fn sqlite3BtreeLeaveCursor(__v464: *mut BtCursor);
    fn sqlite3BtreeLeaveAll(__v465: *mut sqlite3);
    fn sqlite3VdbeCreate(__v466: *mut Parse) -> *mut Vdbe;
    fn sqlite3VdbeAddOp4Int(
        __v467: *mut Vdbe,
        __v468: i32,
        __v469: i32,
        __v470: i32,
        __v471: i32,
        __v472: i32,
    ) -> i32;
    fn sqlite3VdbeAddOpList(
        __v473: *mut Vdbe,
        nOp: i32,
        aOp: *const VdbeOpList,
        iLineno: i32,
    ) -> *mut VdbeOp;
    fn sqlite3VdbeChangeP5(__v477: *mut Vdbe, P5: u16);
    fn sqlite3VdbeChangeP4(__v479: *mut Vdbe, addr: i32, zP4: *const i8, N: i32);
    fn sqlite3VdbeUsesBtree(__v483: *mut Vdbe, __v484: i32);
    fn sqlite3VdbeMakeReady(__v485: *mut Vdbe, __v486: *mut Parse);
    fn sqlite3VdbeFinalize(__v487: *mut Vdbe) -> i32;
    fn sqlite3MisuseError(__v488: i32) -> i32;
    fn sqlite3DbMallocZero(__v489: *mut sqlite3, __v490: u64) -> *mut ();
    fn sqlite3DbFree(__v491: *mut sqlite3, __v492: *mut ());
    fn sqlite3MPrintf(__v493: *mut sqlite3, __v494: *const i8, ...) -> *mut i8;
    fn sqlite3ErrorMsg(__v495: *mut Parse, __v496: *const i8, ...);
    fn sqlite3LocateTable(
        __v497: *mut Parse,
        flags: u32,
        __v499: *const i8,
        __v500: *const i8,
    ) -> *mut Table;
    fn sqlite3ColumnIndex(pTab: *mut Table, zCol: *const i8) -> i32;
    fn sqlite3ErrorWithMsg(__v503: *mut sqlite3, __v504: i32, __v505: *const i8, ...);
    fn sqlite3Error(__v506: *mut sqlite3, __v507: i32);
    fn sqlite3SchemaToIndex(db: *mut sqlite3, __v509: *mut Schema) -> i32;
    fn sqlite3ApiExit(db: *mut sqlite3, __v511: i32) -> i32;
    fn sqlite3OpenTempDatabase(__v512: *mut Parse) -> i32;
    fn sqlite3ParseObjectInit(__v513: *mut Parse, __v514: *mut sqlite3);
    fn sqlite3ParseObjectReset(__v515: *mut Parse);
    fn sqlite3VdbeSerialTypeLen(__v516: u32) -> u32;
    fn sqlite3VdbeExec(__v517: *mut Vdbe) -> i32;
    fn sqlite3VdbeMemSetInt64(__v518: *mut sqlite3_value, __v519: i64);
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
    trace: __SlateRecord164,
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
    u1: __SlateRecord165,
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
struct sqlite3_stmt {}

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
struct sqlite3_blob {}

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
    __slate_bits_0: __slate_bits::__SlateBits67U0,
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
    u: __SlateRecord175,
    pLeft: *mut Expr,
    pRight: *mut Expr,
    x: __SlateRecord176,
    nHeight: i32,
    iTable: i32,
    iColumn: i16,
    iAgg: i16,
    w: __SlateRecord177,
    pAggInfo: *mut AggInfo,
    y: __SlateRecord178,
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
    u: __SlateRecord166,
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
    __slate_bits_0: __slate_bits::__SlateBits91U0,
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
    __slate_bits_0: __slate_bits::__SlateBits103U0,
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
    u1: __SlateRecord191,
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
    fg: __SlateRecord185,
    iCursor: i32,
    colUsed: u64,
    u1: __SlateRecord186,
    u2: __SlateRecord187,
    u3: __SlateRecord188,
    u4: __SlateRecord189,
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
    u: __SlateRecord167,
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
    __slate_bits_0: __slate_bits::__SlateBits150U0,
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
struct VdbeOpList {
    opcode: u8,
    p1: i8,
    p2: i8,
    p3: i8,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct sqlite3InitInfo {
    newTnum: u32,
    iDb: u8,
    busy: u8,
    __slate_bits_0: __slate_bits::__SlateBits163U0,
    azInit: *mut *const i8,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord164 {
    xLegacy: Option<unsafe extern "C-unwind" fn(*mut (), *const i8)>,
    xV2: Option<unsafe extern "C-unwind" fn(u32, *mut (), *mut (), *mut ()) -> i32>,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord165 {
    isInterrupted: i32,
    notUsed1: f64,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord166 {
    pHash: *mut FuncDef,
    pDestructor: *mut FuncDestructor,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord167 {
    tab: __SlateRecord168,
    view: __SlateRecord169,
    vtab: __SlateRecord170,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord168 {
    addColOffset: i32,
    pFKey: *mut FKey,
    pDfltList: *mut ExprList,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord169 {
    pSelect: *mut Select,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord170 {
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
union __SlateRecord175 {
    zToken: *mut i8,
    iValue: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord176 {
    pList: *mut ExprList,
    pSelect: *mut Select,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord177 {
    iJoin: i32,
    iOfst: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord178 {
    pTab: *mut Table,
    pWin: *mut Window,
    nReg: i32,
    sub: __SlateRecord179,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord179 {
    iAddr: i32,
    regReturn: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct ExprList_item {
    pExpr: *mut Expr,
    zEName: *mut i8,
    fg: __SlateRecord181,
    u: __SlateRecord182,
}

#[repr(C, align(4))]
#[derive(Clone, Copy)]
struct __SlateRecord181 {
    sortFlags: u8,
    __slate_bits_0: __slate_bits::__SlateBits181U0,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord182 {
    x: __SlateRecord183,
    iConstExprReg: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord183 {
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
struct __SlateRecord185 {
    jointype: u8,
    __slate_bits_0: __slate_bits::__SlateBits185U0,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord186 {
    zIndexedBy: *mut i8,
    pFuncArg: *mut ExprList,
    nRow: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord187 {
    pIBIndex: *mut Index,
    pCteUse: *mut CteUse,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord188 {
    pOn: *mut Expr,
    pUsing: *mut IdList,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord189 {
    pSchema: *mut Schema,
    zDatabase: *mut i8,
    pSubq: *mut Subquery,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord191 {
    cr: __SlateRecord192,
    d: __SlateRecord193,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord192 {
    addrCrTab: i32,
    regRowid: i32,
    regRoot: i32,
    constraintName: Token,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord193 {
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
    __slate_bits_0: __slate_bits::__SlateBits202U0,
    seekHit: u16,
    ub: __SlateRecord204,
    seqCount: i64,
    cacheStatus: u32,
    seekResult: i32,
    pAltCursor: *mut VdbeCursor,
    uc: __SlateRecord205,
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
union __SlateRecord204 {
    pBtx: *mut Btree,
    aAltMap: *mut u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord205 {
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
// ** 2007 May 1
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
// ** This file contains code used to implement incremental BLOB I/O.
// */
// /*
// ** Valid sqlite3_blob* handles point to Incrblob structures.
// */
#[repr(C)]
#[derive(Clone, Copy)]
struct Incrblob {
    nByte: i32,
    // /* Size of open blob, in bytes */
    iOffset: i32,
    // /* Byte offset of blob in cursor data */
    iCol: u16,
    // /* Table column this handle is open on */
    pCsr: *mut BtCursor,
    // /* Cursor pointing at blob row */
    pStmt: *mut sqlite3_stmt,
    // /* Statement holding cursor open */
    db: *mut sqlite3,
    // /* The associated database */
    zDb: *mut i8,
    // /* Database name */
    pTab: *mut Table,
}

#[repr(C, align(16))]
struct __SlateAlign16<T>(T);

mod __slate_bits {
    #[bitfields::bitfield([u8; 2], c_names = true, new = false, from_into_bits = false, from_traits = false, default = false, debug = false, builder = false, bit_ops = false)]
    pub struct __SlateBits103U0 {
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
    pub struct __SlateBits67U0 {
        #[bits(4)]
        pub notNull: u32,
        #[bits(4)]
        pub eCType: u32,
    }
    #[bitfields::bitfield([u8; 2], c_names = true, new = false, from_into_bits = false, from_traits = false, default = false, debug = false, builder = false, bit_ops = false)]
    pub struct __SlateBits91U0 {
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
    pub struct __SlateBits185U0 {
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
    pub struct __SlateBits181U0 {
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
    pub struct __SlateBits202U0 {
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
    pub struct __SlateBits150U0 {
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
    pub struct __SlateBits163U0 {
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

// /* The database connection */
// /* The attached database containing the blob */
// /* The table containing the blob */
// /* The column containing the blob */
// /* The row containing the glob */
// /* True -> read/write access, false -> read-only */
// /* Handle for accessing the blob returned here */
static mut iLn: i32 = 0 as i32;

static mut openBlob: __SlateAlign16<[VdbeOpList; 6]> = __SlateAlign16([
    VdbeOpList {
        opcode: ((171 as i32) as i8) as u8,
        p1: (0 as i32) as i8,
        p2: (0 as i32) as i8,
        p3: (0 as i32) as i8,
    },
    VdbeOpList {
        opcode: ((114 as i32) as i8) as u8,
        p1: (0 as i32) as i8,
        p2: (0 as i32) as i8,
        p3: (0 as i32) as i8,
    },
    VdbeOpList {
        opcode: ((31 as i32) as i8) as u8,
        p1: (0 as i32) as i8,
        p2: (5 as i32) as i8,
        p3: (1 as i32) as i8,
    },
    VdbeOpList {
        opcode: ((96 as i32) as i8) as u8,
        p1: (0 as i32) as i8,
        p2: (0 as i32) as i8,
        p3: (1 as i32) as i8,
    },
    VdbeOpList {
        opcode: ((86 as i32) as i8) as u8,
        p1: (1 as i32) as i8,
        p2: (0 as i32) as i8,
        p3: (0 as i32) as i8,
    },
    VdbeOpList {
        opcode: ((72 as i32) as i8) as u8,
        p1: (0 as i32) as i8,
        p2: (0 as i32) as i8,
        p3: (0 as i32) as i8,
    },
]);

// /*
// ** Open a blob handle.
// */
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.vdbeblob.sqlite3_blob_open")]
extern "C-unwind" fn sqlite3_blob_open(
    mut db: *mut sqlite3,
    mut zDb: *const i8,
    mut zTable: *const i8,
    mut zColumn: *const i8,
    mut iRow: i64,
    mut wrFlag: i32,
    mut ppBlob: *mut *mut sqlite3_blob,
) -> i32 {
    let mut __slate_storage_549: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_549: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_549) as *mut i32;
    let mut __slate_storage_548: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_548: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_548) as *mut i32;
    let mut __slate_storage_397: std::mem::MaybeUninit<*mut VdbeOp> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_397: *mut *mut VdbeOp =
        std::ptr::addr_of_mut!(__slate_storage_397) as *mut *mut VdbeOp;
    let mut __slate_storage_396: std::mem::MaybeUninit<*mut Vdbe> = std::mem::MaybeUninit::uninit();
    let __slate_slot_396: *mut *mut Vdbe =
        std::ptr::addr_of_mut!(__slate_storage_396) as *mut *mut Vdbe;
    let mut __slate_storage_547: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_547: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_547) as *mut i32;
    let mut __slate_storage_546: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_546: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_546) as *mut i32;
    let mut __slate_storage_393: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_393: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_393) as *mut i32;
    let mut __slate_storage_545: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_545: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_545) as *mut i32;
    let mut __slate_storage_544: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_544: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_544) as *mut i32;
    let mut __slate_storage_392: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_392: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_392) as *mut i32;
    let mut __slate_storage_391: std::mem::MaybeUninit<*mut FKey> = std::mem::MaybeUninit::uninit();
    let __slate_slot_391: *mut *mut FKey =
        std::ptr::addr_of_mut!(__slate_storage_391) as *mut *mut FKey;
    let mut __slate_storage_390: std::mem::MaybeUninit<*mut Index> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_390: *mut *mut Index =
        std::ptr::addr_of_mut!(__slate_storage_390) as *mut *mut Index;
    let mut __slate_storage_389: std::mem::MaybeUninit<*const i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_389: *mut *const i8 =
        std::ptr::addr_of_mut!(__slate_storage_389) as *mut *const i8;
    let mut __slate_storage_543: std::mem::MaybeUninit<bool> = std::mem::MaybeUninit::uninit();
    let __slate_slot_543: *mut bool = std::ptr::addr_of_mut!(__slate_storage_543) as *mut bool;
    let mut __slate_storage_542: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_542: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_542) as *mut i32;
    let mut __slate_storage_541: std::mem::MaybeUninit<bool> = std::mem::MaybeUninit::uninit();
    let __slate_slot_541: *mut bool = std::ptr::addr_of_mut!(__slate_storage_541) as *mut bool;
    let mut __slate_storage_388: std::mem::MaybeUninit<Parse> = std::mem::MaybeUninit::uninit();
    let __slate_slot_388: *mut Parse = std::ptr::addr_of_mut!(__slate_storage_388) as *mut Parse;
    let mut __slate_storage_387: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_387: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_387) as *mut i32;
    let mut __slate_storage_386: std::mem::MaybeUninit<*mut Incrblob> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_386: *mut *mut Incrblob =
        std::ptr::addr_of_mut!(__slate_storage_386) as *mut *mut Incrblob;
    let mut __slate_storage_385: std::mem::MaybeUninit<*mut Table> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_385: *mut *mut Table =
        std::ptr::addr_of_mut!(__slate_storage_385) as *mut *mut Table;
    let mut __slate_storage_384: std::mem::MaybeUninit<*mut i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_384: *mut *mut i8 =
        std::ptr::addr_of_mut!(__slate_storage_384) as *mut *mut i8;
    let mut __slate_storage_383: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_383: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_383) as *mut i32;
    let mut __slate_storage_382: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_382: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_382) as *mut i32;
    let mut __slate_storage_381: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_381: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_381) as *mut i32;
    unsafe {
        std::ptr::write(__slate_slot_381, 0 as i32);
        // /* Index of zColumn in row-record */
        std::ptr::write(__slate_slot_383, 0 as i32);
        std::ptr::write(__slate_slot_384, std::ptr::null_mut::<i8>());
        std::ptr::write(__slate_slot_386, std::ptr::null_mut::<Incrblob>());
        unsafe {
            *ppBlob = std::ptr::null_mut::<sqlite3_blob>();
        }
        // /* wrFlag = (wrFlag ? 1 : 0); */
        wrFlag = !(!(wrFlag != (0 as i32))) as i32;
        unsafe { sqlite3_mutex_enter(unsafe { (*db).mutex }) };
        *__slate_slot_386 = (unsafe { sqlite3DbMallocZero(db, 56 as u64) }) as *mut Incrblob;
        '__join_5: {
            '__join_41: {
                '__join_37: {
                    '__loop_6: loop {
                        if (1 as i32) != (0 as i32) {
                            unsafe {
                                sqlite3ParseObjectInit(
                                    std::ptr::addr_of_mut!(*__slate_slot_388),
                                    db,
                                )
                            };
                            if !(*__slate_slot_386 != std::ptr::null_mut::<Incrblob>()) {
                                break '__join_5;
                            } else {
                                unsafe { sqlite3DbFree(db, *__slate_slot_384 as *mut ()) };
                                *__slate_slot_384 = std::ptr::null_mut::<i8>();
                                unsafe { sqlite3BtreeEnterAll(db) };
                                *__slate_slot_385 = unsafe {
                                    sqlite3LocateTable(
                                        std::ptr::addr_of_mut!(*__slate_slot_388),
                                        (0 as i32) as u32,
                                        zTable,
                                        zDb,
                                    )
                                };
                                if *__slate_slot_385 != std::ptr::null_mut::<Table>()
                                    && (((unsafe { (*(*__slate_slot_385)).eTabType }) as u32)
                                        as i32)
                                        == (1 as i32)
                                {
                                    *__slate_slot_385 = std::ptr::null_mut::<Table>();
                                    unsafe {
                                        sqlite3ErrorMsg(
                                            std::ptr::addr_of_mut!(*__slate_slot_388),
                                            (b"cannot open virtual table: %s\0".as_ptr() as *mut i8)
                                                as *const i8,
                                            zTable,
                                        )
                                    };
                                }
                                if *__slate_slot_385 != std::ptr::null_mut::<Table>()
                                    && !((unsafe { (*(*__slate_slot_385)).tabFlags })
                                        & ((128 as i32) as u32)
                                        == ((0 as i32) as u32))
                                {
                                    *__slate_slot_385 = std::ptr::null_mut::<Table>();
                                    unsafe {
                                        sqlite3ErrorMsg(
                                            std::ptr::addr_of_mut!(*__slate_slot_388),
                                            (b"cannot open table without rowid: %s\0".as_ptr()
                                                as *mut i8)
                                                as *const i8,
                                            zTable,
                                        )
                                    };
                                }
                                if *__slate_slot_385 != std::ptr::null_mut::<Table>()
                                    && (unsafe { (*(*__slate_slot_385)).tabFlags })
                                        & ((96 as i32) as u32)
                                        != ((0 as i32) as u32)
                                {
                                    *__slate_slot_385 = std::ptr::null_mut::<Table>();
                                    unsafe {
                                        sqlite3ErrorMsg(
                                            std::ptr::addr_of_mut!(*__slate_slot_388),
                                            (b"cannot open table with generated columns: %s\0"
                                                .as_ptr()
                                                as *mut i8)
                                                as *const i8,
                                            zTable,
                                        )
                                    };
                                }
                                if *__slate_slot_385 != std::ptr::null_mut::<Table>()
                                    && (((unsafe { (*(*__slate_slot_385)).eTabType }) as u32)
                                        as i32)
                                        == (2 as i32)
                                {
                                    *__slate_slot_385 = std::ptr::null_mut::<Table>();
                                    unsafe {
                                        sqlite3ErrorMsg(
                                            std::ptr::addr_of_mut!(*__slate_slot_388),
                                            (b"cannot open view: %s\0".as_ptr() as *mut i8)
                                                as *const i8,
                                            zTable,
                                        )
                                    };
                                }
                                if *__slate_slot_385 == std::ptr::null_mut::<Table>() {
                                    *__slate_slot_541 = true as bool;
                                } else {
                                    std::ptr::write(__slate_slot_542, unsafe {
                                        sqlite3SchemaToIndex(db, unsafe {
                                            (*(*__slate_slot_385)).pSchema
                                        })
                                    });
                                    *__slate_slot_387 = *__slate_slot_542;
                                    if *__slate_slot_542 == (1 as i32) {
                                        *__slate_slot_543 = (unsafe {
                                            sqlite3OpenTempDatabase(std::ptr::addr_of_mut!(
                                                *__slate_slot_388
                                            ))
                                        }) != (0 as i32);
                                    } else {
                                        *__slate_slot_543 = false as bool;
                                    }
                                    *__slate_slot_541 = *__slate_slot_543;
                                }
                                if *__slate_slot_541 {
                                    break '__join_41;
                                } else {
                                    unsafe {
                                        (*(*__slate_slot_386)).pTab = *__slate_slot_385;
                                    }
                                    unsafe {
                                        (*(*__slate_slot_386)).zDb = unsafe {
                                            (*unsafe {
                                                unsafe { (*db).aDb }
                                                    .offset(*__slate_slot_387 as isize)
                                            })
                                            .zDbSName
                                        };
                                    }
                                    // /* Now search pTab for the exact column. */
                                    *__slate_slot_382 =
                                        unsafe { sqlite3ColumnIndex(*__slate_slot_385, zColumn) };
                                    if *__slate_slot_382 < (0 as i32) {
                                        break '__join_37;
                                    } else {
                                        // /* If the value is being opened for writing, check that the
                                        //     ** column is not indexed, and that it is not part of a foreign key.
                                        //     */
                                        if wrFlag != (0 as i32) {
                                            '__join_26: {
                                                std::ptr::write(
                                                    __slate_slot_389,
                                                    std::ptr::null::<i8>(),
                                                );
                                                if (unsafe { (*db).flags })
                                                    & (((16384 as i32) as i64) as u64)
                                                    != (0 as u64)
                                                {
                                                    // /* Check that the column is not part of an FK child key definition. It
                                                    //         ** is not necessary to check if it is part of a parent key, as parent
                                                    //         ** key columns must be indexed. The check below will pick up this
                                                    //         ** case.  */
                                                    0 as i32;
                                                    *__slate_slot_391 = unsafe {
                                                        (*(*__slate_slot_385)).u.tab.pFKey
                                                    };
                                                    loop {
                                                        if *__slate_slot_391
                                                            != std::ptr::null_mut::<FKey>()
                                                        {
                                                            *__slate_slot_392 = 0 as i32;
                                                            loop {
                                                                if *__slate_slot_392
                                                                    < unsafe {
                                                                        (*(*__slate_slot_391)).nCol
                                                                    }
                                                                {
                                                                    '__join_30: {
                                                                        if (unsafe {
                                                                            (*unsafe { unsafe { std::ptr::addr_of_mut!((*(*__slate_slot_391)).aCol) as *mut sColMap }.offset(*__slate_slot_392 as isize) }).iFrom
                                                                        }) == *__slate_slot_382
                                                                        {
                                                                            *__slate_slot_389 =
                                                                                (b"foreign key\0"
                                                                                    .as_ptr()
                                                                                    as *mut i8)
                                                                                    as *const i8;
                                                                        }
                                                                    }
                                                                    std::ptr::write(
                                                                        __slate_slot_544,
                                                                        *__slate_slot_392,
                                                                    );
                                                                    std::ptr::write(
                                                                        __slate_slot_545,
                                                                        *__slate_slot_544
                                                                            + (1 as i32),
                                                                    );
                                                                    *__slate_slot_392 =
                                                                        *__slate_slot_545;
                                                                } else {
                                                                    break;
                                                                }
                                                            }
                                                            *__slate_slot_391 = unsafe {
                                                                (*(*__slate_slot_391)).pNextFrom
                                                            };
                                                        } else {
                                                            break '__join_26;
                                                        }
                                                    }
                                                }
                                            }
                                            *__slate_slot_390 =
                                                unsafe { (*(*__slate_slot_385)).pIndex };
                                            loop {
                                                if *__slate_slot_390
                                                    != std::ptr::null_mut::<Index>()
                                                {
                                                    *__slate_slot_393 = 0 as i32;
                                                    loop {
                                                        if *__slate_slot_393
                                                            < (((unsafe {
                                                                (*(*__slate_slot_390)).nKeyCol
                                                            })
                                                                as u32)
                                                                as i32)
                                                        {
                                                            // /* FIXME: Be smarter about indexes that use expressions */
                                                            if ((unsafe {
                                                                *unsafe {
                                                                    unsafe {
                                                                        (*(*__slate_slot_390))
                                                                            .aiColumn
                                                                    }
                                                                    .offset(
                                                                        *__slate_slot_393 as isize,
                                                                    )
                                                                }
                                                            })
                                                                as i32)
                                                                == *__slate_slot_382
                                                                || ((unsafe {
                                                                    *unsafe {
                                                                        unsafe {
                                                                            (*(*__slate_slot_390))
                                                                                .aiColumn
                                                                        }
                                                                        .offset(
                                                                            *__slate_slot_393
                                                                                as isize,
                                                                        )
                                                                    }
                                                                })
                                                                    as i32)
                                                                    == -(2 as i32)
                                                            {
                                                                *__slate_slot_389 = (b"indexed\0"
                                                                    .as_ptr()
                                                                    as *mut i8)
                                                                    as *const i8;
                                                            }
                                                            std::ptr::write(
                                                                __slate_slot_546,
                                                                *__slate_slot_393,
                                                            );
                                                            std::ptr::write(
                                                                __slate_slot_547,
                                                                *__slate_slot_546 + (1 as i32),
                                                            );
                                                            *__slate_slot_393 = *__slate_slot_547;
                                                        } else {
                                                            break;
                                                        }
                                                    }
                                                    *__slate_slot_390 =
                                                        unsafe { (*(*__slate_slot_390)).pNext };
                                                } else {
                                                    break;
                                                }
                                            }
                                            if *__slate_slot_389 != std::ptr::null::<i8>() {
                                                break '__loop_6;
                                            }
                                        }
                                        unsafe {
                                            (*(*__slate_slot_386)).pStmt = (unsafe {
                                                sqlite3VdbeCreate(std::ptr::addr_of_mut!(
                                                    *__slate_slot_388
                                                ))
                                            })
                                                as *mut sqlite3_stmt;
                                        }
                                        0 as i32;
                                        if (unsafe { (*(*__slate_slot_386)).pStmt })
                                            != std::ptr::null_mut::<sqlite3_stmt>()
                                        {
                                            // /* This VDBE program seeks a btree cursor to the identified
                                            //       ** db/table/row entry. The reason for using a vdbe program instead
                                            //       ** of writing code to use the b-tree layer directly is that the
                                            //       ** vdbe program will take advantage of the various transaction,
                                            //       ** locking and error handling infrastructure built into the vdbe.
                                            //       **
                                            //       ** After seeking the cursor, the vdbe executes an OP_ResultRow.
                                            //       ** Code external to the Vdbe then "borrows" the b-tree cursor and
                                            //       ** uses it to implement the blob_read(), blob_write() and
                                            //       ** blob_bytes() functions.
                                            //       **
                                            //       ** The sqlite3_blob_close() function finalizes the vdbe program,
                                            //       ** which closes the b-tree cursor and (possibly) commits the
                                            //       ** transaction.
                                            //       */
                                            // /* 0: Acquire a read or write lock */
                                            // /* 1: Open a cursor */
                                            // /* blobSeekToRow() will initialize r[1] to the desired rowid */
                                            // /* 2: Seek the cursor to rowid=r[1] */
                                            // /* 3  */
                                            // /* 4  */
                                            // /* 5  */
                                            std::ptr::write(
                                                __slate_slot_396,
                                                (unsafe { (*(*__slate_slot_386)).pStmt })
                                                    as *mut Vdbe,
                                            );
                                            unsafe {
                                                sqlite3VdbeAddOp4Int(
                                                    *__slate_slot_396,
                                                    2 as i32,
                                                    *__slate_slot_387,
                                                    wrFlag,
                                                    unsafe {
                                                        (*unsafe { (*(*__slate_slot_385)).pSchema })
                                                            .schema_cookie
                                                    },
                                                    unsafe {
                                                        (*unsafe { (*(*__slate_slot_385)).pSchema })
                                                            .iGeneration
                                                    },
                                                )
                                            };
                                            unsafe {
                                                sqlite3VdbeChangeP5(
                                                    *__slate_slot_396,
                                                    ((1 as i32) as i16) as u16,
                                                )
                                            };
                                            0 as i32;
                                            *__slate_slot_397 = unsafe {
                                                sqlite3VdbeAddOpList(
                                                    *__slate_slot_396,
                                                    (((24 as u64) / (4 as u64)) as u32) as i32,
                                                    unsafe {
                                                        std::ptr::addr_of!(openBlob.0)
                                                            as *const VdbeOpList
                                                    },
                                                    unsafe { iLn },
                                                )
                                            };
                                            // /* Make sure a mutex is held on the table to be accessed */
                                            unsafe {
                                                sqlite3VdbeUsesBtree(
                                                    *__slate_slot_396,
                                                    *__slate_slot_387,
                                                )
                                            };
                                            if (((unsafe { (*db).mallocFailed }) as u32) as i32)
                                                == (0 as i32)
                                            {
                                                0 as i32;
                                                // /* Configure the OP_TableLock instruction */
                                                unsafe {
                                                    (*unsafe {
                                                        (*__slate_slot_397)
                                                            .offset((0 as i32) as isize)
                                                    })
                                                    .p1 = *__slate_slot_387;
                                                }
                                                unsafe {
                                                    (*unsafe {
                                                        (*__slate_slot_397)
                                                            .offset((0 as i32) as isize)
                                                    })
                                                    .p2 = (unsafe { (*(*__slate_slot_385)).tnum })
                                                        as i32;
                                                }
                                                unsafe {
                                                    (*unsafe {
                                                        (*__slate_slot_397)
                                                            .offset((0 as i32) as isize)
                                                    })
                                                    .p3 = wrFlag;
                                                }
                                                unsafe {
                                                    sqlite3VdbeChangeP4(
                                                        *__slate_slot_396,
                                                        2 as i32,
                                                        (unsafe { (*(*__slate_slot_385)).zName })
                                                            as *const i8,
                                                        0 as i32,
                                                    )
                                                };
                                            }
                                            if (((unsafe { (*db).mallocFailed }) as u32) as i32)
                                                == (0 as i32)
                                            {
                                                // /* Remove either the OP_OpenWrite or OpenRead. Set the P2
                                                //         ** parameter of the other to pTab->tnum.  */
                                                if wrFlag != (0 as i32) {
                                                    unsafe {
                                                        (*unsafe {
                                                            (*__slate_slot_397)
                                                                .offset((1 as i32) as isize)
                                                        })
                                                        .opcode = ((116 as i32) as i8) as u8;
                                                    }
                                                }
                                                unsafe {
                                                    (*unsafe {
                                                        (*__slate_slot_397)
                                                            .offset((1 as i32) as isize)
                                                    })
                                                    .p2 = (unsafe { (*(*__slate_slot_385)).tnum })
                                                        as i32;
                                                }
                                                unsafe {
                                                    (*unsafe {
                                                        (*__slate_slot_397)
                                                            .offset((1 as i32) as isize)
                                                    })
                                                    .p3 = *__slate_slot_387;
                                                }
                                                // /* Configure the number of columns. Configure the cursor to
                                                //         ** think that the table has one more column than it really
                                                //         ** does. An OP_Column to retrieve this imaginary column will
                                                //         ** always return an SQL NULL. This is useful because it means
                                                //         ** we can invoke OP_Column to fill in the vdbe cursors type
                                                //         ** and offset cache without causing any IO.
                                                //         */
                                                unsafe {
                                                    (*unsafe {
                                                        (*__slate_slot_397)
                                                            .offset((1 as i32) as isize)
                                                    })
                                                    .p4type = -(3 as i32) as i8;
                                                }
                                                unsafe {
                                                    (*unsafe {
                                                        (*__slate_slot_397)
                                                            .offset((1 as i32) as isize)
                                                    })
                                                    .p4
                                                    .i = ((unsafe { (*(*__slate_slot_385)).nCol })
                                                        as i32)
                                                        + (1 as i32);
                                                }
                                                unsafe {
                                                    (*unsafe {
                                                        (*__slate_slot_397)
                                                            .offset((3 as i32) as isize)
                                                    })
                                                    .p2 = (unsafe { (*(*__slate_slot_385)).nCol })
                                                        as i32;
                                                }
                                                (*__slate_slot_388).nVar = (0 as i32) as i16;
                                                (*__slate_slot_388).nMem = 1 as i32;
                                                (*__slate_slot_388).nTab = 1 as i32;
                                                unsafe {
                                                    sqlite3VdbeMakeReady(
                                                        *__slate_slot_396,
                                                        std::ptr::addr_of_mut!(*__slate_slot_388),
                                                    )
                                                };
                                            }
                                        }
                                        unsafe {
                                            (*(*__slate_slot_386)).iCol =
                                                (*__slate_slot_382 as i16) as u16;
                                        }
                                        unsafe {
                                            (*(*__slate_slot_386)).db = db;
                                        }
                                        unsafe { sqlite3BtreeLeaveAll(db) };
                                        if (unsafe { (*db).mallocFailed }) != (0 as u8) {
                                            break '__join_5;
                                        } else {
                                            *__slate_slot_383 = blobSeekToRow(
                                                *__slate_slot_386,
                                                iRow,
                                                std::ptr::addr_of_mut!(*__slate_slot_384),
                                            );
                                            std::ptr::write(__slate_slot_548, *__slate_slot_381);
                                            std::ptr::write(
                                                __slate_slot_549,
                                                *__slate_slot_548 + (1 as i32),
                                            );
                                            *__slate_slot_381 = *__slate_slot_549;
                                            if *__slate_slot_549 >= (50 as i32)
                                                || *__slate_slot_383 != (17 as i32)
                                            {
                                                break '__join_5;
                                            } else {
                                                unsafe {
                                                    sqlite3ParseObjectReset(std::ptr::addr_of_mut!(
                                                        *__slate_slot_388
                                                    ))
                                                };
                                            }
                                        }
                                    }
                                }
                            }
                        } else {
                            break '__join_5;
                        }
                    }
                    unsafe { sqlite3DbFree(db, *__slate_slot_384 as *mut ()) };
                    *__slate_slot_384 = unsafe {
                        sqlite3MPrintf(
                            db,
                            (b"cannot open %s column for writing\0".as_ptr() as *mut i8)
                                as *const i8,
                            *__slate_slot_389,
                        )
                    };
                    *__slate_slot_383 = 1 as i32;
                    unsafe { sqlite3BtreeLeaveAll(db) };
                    break '__join_5;
                }
                unsafe { sqlite3DbFree(db, *__slate_slot_384 as *mut ()) };
                *__slate_slot_384 = unsafe {
                    sqlite3MPrintf(
                        db,
                        (b"no such column: \"%s\"\0".as_ptr() as *mut i8) as *const i8,
                        zColumn,
                    )
                };
                *__slate_slot_383 = 1 as i32;
                unsafe { sqlite3BtreeLeaveAll(db) };
                break '__join_5;
            }
            if (*__slate_slot_388).zErrMsg != std::ptr::null_mut::<i8>() {
                unsafe { sqlite3DbFree(db, *__slate_slot_384 as *mut ()) };
                *__slate_slot_384 = (*__slate_slot_388).zErrMsg;
                (*__slate_slot_388).zErrMsg = std::ptr::null_mut::<i8>();
            }
            *__slate_slot_383 = 1 as i32;
            unsafe { sqlite3BtreeLeaveAll(db) };
        }
        if *__slate_slot_383 == (0 as i32)
            && (((unsafe { (*db).mallocFailed }) as u32) as i32) == (0 as i32)
        {
            unsafe {
                *ppBlob = *__slate_slot_386 as *mut sqlite3_blob;
            }
        } else {
            if *__slate_slot_386 != std::ptr::null_mut::<Incrblob>()
                && (unsafe { (*(*__slate_slot_386)).pStmt }) != std::ptr::null_mut::<sqlite3_stmt>()
            {
                unsafe {
                    sqlite3VdbeFinalize((unsafe { (*(*__slate_slot_386)).pStmt }) as *mut Vdbe)
                };
            }
            unsafe { sqlite3DbFree(db, *__slate_slot_386 as *mut ()) };
        }
        unsafe {
            sqlite3ErrorWithMsg(
                db,
                *__slate_slot_383,
                (if *__slate_slot_384 != std::ptr::null_mut::<i8>() {
                    b"%s\0".as_ptr() as *mut i8
                } else {
                    std::ptr::null_mut::<i8>()
                }) as *const i8,
                *__slate_slot_384,
            )
        };
        unsafe { sqlite3DbFree(db, *__slate_slot_384 as *mut ()) };
        unsafe { sqlite3ParseObjectReset(std::ptr::addr_of_mut!(*__slate_slot_388)) };
        *__slate_slot_383 = unsafe { sqlite3ApiExit(db, *__slate_slot_383) };
        unsafe { sqlite3_mutex_leave(unsafe { (*db).mutex }) };
        return *__slate_slot_383;
    }
    return unsafe { std::mem::zeroed() };
}

// /*
// ** Move an existing blob handle to point to a different row of the same
// ** database table.
// **
// ** If an error occurs, or if the specified row does not exist or does not
// ** contain a blob or text value, then an error code is returned and the
// ** database handle error code and message set. If this happens, then all
// ** subsequent calls to sqlite3_blob_xxx() functions (except blob_close())
// ** immediately return SQLITE_ABORT.
// */
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.vdbeblob.sqlite3_blob_reopen")]
extern "C-unwind" fn sqlite3_blob_reopen(mut pBlob: *mut sqlite3_blob, mut iRow: i64) -> i32 {
    let mut rc: i32 = 0 as i32;
    let mut p: *mut Incrblob = pBlob as *mut Incrblob;
    let mut db: *mut sqlite3 = unsafe { std::mem::zeroed() };
    if p == std::ptr::null_mut::<Incrblob>() {
        return unsafe { sqlite3MisuseError(508 as i32) };
    }
    db = unsafe { (*p).db };
    unsafe { sqlite3_mutex_enter(unsafe { (*db).mutex }) };
    if (unsafe { (*p).pStmt }) == std::ptr::null_mut::<sqlite3_stmt>() {
        // /* If there is no statement handle, then the blob-handle has
        //     ** already been invalidated. Return SQLITE_ABORT in this case.
        //     */
        rc = 4 as i32;
    } else {
        let mut zErr: *mut i8 = unsafe { std::mem::zeroed() };
        unsafe {
            (*((unsafe { (*p).pStmt }) as *mut Vdbe)).rc = 0 as i32;
        }
        rc = blobSeekToRow(p, iRow, std::ptr::addr_of_mut!(zErr));
        if rc != (0 as i32) {
            unsafe {
                sqlite3ErrorWithMsg(
                    db,
                    rc,
                    (if zErr != std::ptr::null_mut::<i8>() {
                        b"%s\0".as_ptr() as *mut i8
                    } else {
                        std::ptr::null_mut::<i8>()
                    }) as *const i8,
                    zErr,
                )
            };
            unsafe { sqlite3DbFree(db, zErr as *mut ()) };
        }
        0 as i32;
    }
    rc = unsafe { sqlite3ApiExit(db, rc) };
    0 as i32;
    unsafe { sqlite3_mutex_leave(unsafe { (*db).mutex }) };
    return rc;
}

// /*
// ** Close a blob handle that was previously created using
// ** sqlite3_blob_open().
// */
// /* #ifndef SQLITE_OMIT_INCRBLOB */
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.vdbeblob.sqlite3_blob_close")]
extern "C-unwind" fn sqlite3_blob_close(mut pBlob: *mut sqlite3_blob) -> i32 {
    let mut p: *mut Incrblob = pBlob as *mut Incrblob;
    let mut rc: i32 = 0 as i32;
    let mut db: *mut sqlite3 = unsafe { std::mem::zeroed() };
    if p != std::ptr::null_mut::<Incrblob>() {
        let mut pStmt: *mut sqlite3_stmt = unsafe { (*p).pStmt };
        db = unsafe { (*p).db };
        unsafe { sqlite3_mutex_enter(unsafe { (*db).mutex }) };
        unsafe { sqlite3DbFree(db, p as *mut ()) };
        unsafe { sqlite3_mutex_leave(unsafe { (*db).mutex }) };
        rc = unsafe { sqlite3_finalize(pStmt) };
    } else {
        rc = 0 as i32;
    }
    return rc;
}

// /*
// ** Query a blob handle for the size of the data.
// **
// ** The Incrblob.nByte field is fixed for the lifetime of the Incrblob
// ** so no mutex is required for access.
// */
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.vdbeblob.sqlite3_blob_bytes")]
extern "C-unwind" fn sqlite3_blob_bytes(mut pBlob: *mut sqlite3_blob) -> i32 {
    let mut p: *mut Incrblob = pBlob as *mut Incrblob;
    return if p != std::ptr::null_mut::<Incrblob>()
        && (unsafe { (*p).pStmt }) != std::ptr::null_mut::<sqlite3_stmt>()
    {
        unsafe { (*p).nByte }
    } else {
        0 as i32
    };
}

// /*
// ** Read data from a blob handle.
// */
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.vdbeblob.sqlite3_blob_read")]
extern "C-unwind" fn sqlite3_blob_read(
    mut pBlob: *mut sqlite3_blob,
    mut z: *mut (),
    mut n: i32,
    mut iOffset: i32,
) -> i32 {
    return blobReadWrite(pBlob, z, n, iOffset, unsafe {
        std::mem::transmute::<
            *const (),
            Option<unsafe extern "C-unwind" fn(*mut BtCursor, u32, u32, *mut ()) -> i32>,
        >(sqlite3BtreePayloadChecked as *const ())
    });
}

// /*
// ** Write data to a blob handle.
// */
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.vdbeblob.sqlite3_blob_write")]
extern "C-unwind" fn sqlite3_blob_write(
    mut pBlob: *mut sqlite3_blob,
    mut z: *const (),
    mut n: i32,
    mut iOffset: i32,
) -> i32 {
    return blobReadWrite(pBlob, z as *mut (), n, iOffset, unsafe {
        std::mem::transmute::<
            *const (),
            Option<unsafe extern "C-unwind" fn(*mut BtCursor, u32, u32, *mut ()) -> i32>,
        >(sqlite3BtreePutData as *const ())
    });
}

// /* Table object */
// /*
// ** This function is used by both blob_open() and blob_reopen(). It seeks
// ** the b-tree cursor associated with blob handle p to point to row iRow.
// ** If successful, SQLITE_OK is returned and subsequent calls to
// ** sqlite3_blob_read() or sqlite3_blob_write() access the specified row.
// **
// ** If an error occurs, or if the specified row does not exist or does not
// ** contain a value of type TEXT or BLOB in the column nominated when the
// ** blob handle was opened, then an error code is returned and *pzErr may
// ** be set to point to a buffer containing an error message. It is the
// ** responsibility of the caller to free the error message buffer using
// ** sqlite3DbFree().
// **
// ** If an error does occur, then the b-tree cursor is closed. All subsequent
// ** calls to sqlite3_blob_read(), blob_write() or blob_reopen() will
// ** immediately return SQLITE_ABORT.
// */
fn blobSeekToRow(mut p: *mut Incrblob, mut iRow: i64, mut pzErr: *mut *mut i8) -> i32 {
    // /* Error code */
    let mut rc: i32 = 0 as i32;
    // /* Error message */
    let mut zErr: *mut i8 = std::ptr::null_mut::<i8>();
    let mut v: *mut Vdbe = (unsafe { (*p).pStmt }) as *mut Vdbe;
    // /* Set the value of register r[1] in the SQL statement to integer iRow.
    //   ** This is done directly as a performance optimization
    //   */
    unsafe {
        sqlite3VdbeMemSetInt64(
            unsafe { unsafe { (*v).aMem }.offset((1 as i32) as isize) },
            iRow,
        )
    };
    // /* If the statement has been run before (and is paused at the OP_ResultRow)
    //   ** then back it up to the point where it does the OP_NotExists.  This could
    //   ** have been down with an extra OP_Goto, but simply setting the program
    //   ** counter is faster. */
    if (unsafe { (*v).pc }) > (4 as i32) {
        unsafe {
            (*v).pc = 4 as i32;
        }
        0 as i32;
        rc = unsafe { sqlite3VdbeExec(v) };
    } else {
        rc = unsafe { sqlite3_step(unsafe { (*p).pStmt }) };
    }
    if rc == (100 as i32) {
        let mut pC: *mut VdbeCursor =
            unsafe { *unsafe { unsafe { (*v).apCsr }.offset((0 as i32) as isize) } };
        let mut r#type: u32 = 0 as u32;
        0 as i32;
        0 as i32;
        r#type = if (((unsafe { (*pC).nHdrParsed }) as u32) as i32)
            > (((unsafe { (*p).iCol }) as u32) as i32)
        {
            unsafe {
                *unsafe {
                    unsafe { std::ptr::addr_of_mut!((*pC).aType) as *mut u32 }
                        .offset((((unsafe { (*p).iCol }) as u32) as i32) as isize)
                }
            }
        } else {
            (0 as i32) as u32
        };
        {}
        {}
        if r#type < ((12 as i32) as u32) {
            zErr = unsafe {
                sqlite3MPrintf(
                    unsafe { (*p).db },
                    (b"cannot open value of type %s\0".as_ptr() as *mut i8) as *const i8,
                    if r#type == ((0 as i32) as u32) {
                        b"null\0".as_ptr() as *mut i8
                    } else {
                        if r#type == ((7 as i32) as u32) {
                            b"real\0".as_ptr() as *mut i8
                        } else {
                            b"integer\0".as_ptr() as *mut i8
                        }
                    },
                )
            };
            rc = 1 as i32;
            unsafe { sqlite3_finalize(unsafe { (*p).pStmt }) };
            unsafe {
                (*p).pStmt = std::ptr::null_mut::<sqlite3_stmt>();
            }
        } else {
            unsafe {
                (*p).iOffset = (unsafe {
                    *unsafe {
                        unsafe { std::ptr::addr_of_mut!((*pC).aType) as *mut u32 }.offset(
                            ((((unsafe { (*p).iCol }) as u32) as i32)
                                + ((unsafe { (*pC).nField }) as i32))
                                as isize,
                        )
                    }
                }) as i32;
            }
            unsafe {
                (*p).nByte = (unsafe { sqlite3VdbeSerialTypeLen(r#type) }) as i32;
            }
            unsafe {
                (*p).pCsr = unsafe { (*pC).uc.pCursor };
            }
            unsafe { sqlite3BtreeIncrblobCursor(unsafe { (*p).pCsr }) };
        }
    }
    if rc == (100 as i32) {
        rc = 0 as i32;
    } else {
        if (unsafe { (*p).pStmt }) != std::ptr::null_mut::<sqlite3_stmt>() {
            rc = unsafe { sqlite3_finalize(unsafe { (*p).pStmt }) };
            unsafe {
                (*p).pStmt = std::ptr::null_mut::<sqlite3_stmt>();
            }
            if rc == (0 as i32) {
                zErr = unsafe {
                    sqlite3MPrintf(
                        unsafe { (*p).db },
                        (b"no such rowid: %lld\0".as_ptr() as *mut i8) as *const i8,
                        iRow,
                    )
                };
                rc = 1 as i32;
            } else {
                zErr = unsafe {
                    sqlite3MPrintf(
                        unsafe { (*p).db },
                        (b"%s\0".as_ptr() as *mut i8) as *const i8,
                        unsafe { sqlite3_errmsg(unsafe { (*p).db }) },
                    )
                };
            }
        }
    }
    0 as i32;
    0 as i32;
    unsafe {
        *pzErr = zErr;
    }
    return rc;
}

// /*
// ** Perform a read or write operation on a blob
// */
fn blobReadWrite(
    mut pBlob: *mut sqlite3_blob,
    mut z: *mut (),
    mut n: i32,
    mut iOffset: i32,
    mut xCall: Option<unsafe extern "C-unwind" fn(*mut BtCursor, u32, u32, *mut ()) -> i32>,
) -> i32 {
    let mut rc: i32 = 0 as i32;
    let mut p: *mut Incrblob = pBlob as *mut Incrblob;
    let mut v: *mut Vdbe = unsafe { std::mem::zeroed() };
    let mut db: *mut sqlite3 = unsafe { std::mem::zeroed() };
    if p == std::ptr::null_mut::<Incrblob>() {
        return unsafe { sqlite3MisuseError(393 as i32) };
    }
    db = unsafe { (*p).db };
    unsafe { sqlite3_mutex_enter(unsafe { (*db).mutex }) };
    v = (unsafe { (*p).pStmt }) as *mut Vdbe;
    if n < (0 as i32)
        || iOffset < (0 as i32)
        || (iOffset as i64) + (n as i64) > ((unsafe { (*p).nByte }) as i64)
    {
        // /* Request is out of range. Return a transient error. */
        rc = 1 as i32;
    } else {
        if v == std::ptr::null_mut::<Vdbe>() {
            // /* If there is no statement handle, then the blob-handle has
            //     ** already been invalidated. Return SQLITE_ABORT in this case.
            //     */
            rc = 4 as i32;
        } else {
            // /* Call either BtreeData() or BtreePutData(). If SQLITE_ABORT is
            //     ** returned, clean-up the statement handle.
            //     */
            0 as i32;
            unsafe { sqlite3BtreeEnterCursor(unsafe { (*p).pCsr }) };
            rc = unsafe {
                xCall.unwrap()(
                    unsafe { (*p).pCsr },
                    (iOffset + unsafe { (*p).iOffset }) as u32,
                    n as u32,
                    z,
                )
            };
            unsafe { sqlite3BtreeLeaveCursor(unsafe { (*p).pCsr }) };
            if rc == (4 as i32) {
                unsafe { sqlite3VdbeFinalize(v) };
                unsafe {
                    (*p).pStmt = std::ptr::null_mut::<sqlite3_stmt>();
                }
            } else {
                unsafe {
                    (*v).rc = rc;
                }
            }
        }
    }
    unsafe { sqlite3Error(db, rc) };
    rc = unsafe { sqlite3ApiExit(db, rc) };
    unsafe { sqlite3_mutex_leave(unsafe { (*db).mutex }) };
    return rc;
}
