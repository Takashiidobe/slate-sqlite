unsafe extern "C" {
    fn sqlite3_snprintf(__v408: i32, __v409: *mut i8, __v410: *const i8, ...) -> *mut i8;
    fn sqlite3_randomness(N: i32, P: *mut ());
    fn sqlite3_uri_int64(__v413: *const i8, __v414: *const i8, __v415: i64) -> i64;
    fn sqlite3_errmsg(__v416: *mut sqlite3) -> *const i8;
    fn sqlite3_prepare_v2(
        db: *mut sqlite3,
        zSql: *const i8,
        nByte: i32,
        ppStmt: *mut *mut sqlite3_stmt,
        pzTail: *mut *const i8,
    ) -> i32;
    fn sqlite3_step(__v422: *mut sqlite3_stmt) -> i32;
    fn sqlite3_column_text(__v423: *mut sqlite3_stmt, iCol: i32) -> *const u8;
    fn sqlite3_finalize(pStmt: *mut sqlite3_stmt) -> i32;
    fn sqlite3_value_text(__v426: *mut sqlite3_value) -> *const u8;
    fn sqlite3_value_type(__v427: *mut sqlite3_value) -> i32;
    fn strncmp(__s1: *const i8, __s2: *const i8, __n: u64) -> i32;
    fn sqlite3OsFileSize(__v431: *mut sqlite3_file, pSize: *mut i64) -> i32;
    fn sqlite3PagerGetJournalMode(__v433: *mut Pager) -> i32;
    fn sqlite3PagerFile(__v434: *mut Pager) -> *mut sqlite3_file;
    fn sqlite3PagerIsMemdb(__v435: *mut Pager) -> i32;
    fn sqlite3BtreeClose(__v436: *mut Btree) -> i32;
    fn sqlite3BtreeSetCacheSize(__v437: *mut Btree, __v438: i32) -> i32;
    fn sqlite3BtreeSetSpillSize(__v439: *mut Btree, __v440: i32) -> i32;
    fn sqlite3BtreeSetPagerFlags(__v441: *mut Btree, __v442: u32) -> i32;
    fn sqlite3BtreeSetPageSize(p: *mut Btree, nPagesize: i32, nReserve: i32, eFix: i32) -> i32;
    fn sqlite3BtreeGetPageSize(__v447: *mut Btree) -> i32;
    fn sqlite3BtreeGetRequestedReserve(__v448: *mut Btree) -> i32;
    fn sqlite3BtreeSetAutoVacuum(__v449: *mut Btree, __v450: i32) -> i32;
    fn sqlite3BtreeGetAutoVacuum(__v451: *mut Btree) -> i32;
    fn sqlite3BtreeBeginTrans(__v452: *mut Btree, __v453: i32, __v454: *mut i32) -> i32;
    fn sqlite3BtreeCommit(__v455: *mut Btree) -> i32;
    fn sqlite3BtreeGetFilename(__v456: *mut Btree) -> *const i8;
    fn sqlite3BtreeCopyFile(__v457: *mut Btree, __v458: *mut Btree) -> i32;
    fn sqlite3BtreeGetMeta(pBtree: *mut Btree, idx: i32, pValue: *mut u32);
    fn sqlite3BtreeUpdateMeta(__v462: *mut Btree, idx: i32, value: u32) -> i32;
    fn sqlite3BtreePager(__v465: *mut Btree) -> *mut Pager;
    fn sqlite3VdbeAddOp2(__v466: *mut Vdbe, __v467: i32, __v468: i32, __v469: i32) -> i32;
    fn sqlite3VdbeUsesBtree(__v470: *mut Vdbe, __v471: i32);
    fn sqlite3DbFree(__v472: *mut sqlite3, __v473: *mut ());
    fn sqlite3VMPrintf(
        __v474: *mut sqlite3,
        __v475: *const i8,
        __v476: core::ffi::VaList<'_>,
    ) -> *mut i8;
    fn sqlite3SetString(__v477: *mut *mut i8, __v478: *mut sqlite3, __v479: *const i8);
    fn sqlite3ExprDelete(__v480: *mut sqlite3, __v481: *mut Expr);
    fn sqlite3ResetAllSchemasOfConnection(__v482: *mut sqlite3);
    fn sqlite3ExprCode(__v483: *mut Parse, __v484: *mut Expr, __v485: i32);
    fn sqlite3GetVdbe(__v493: *mut Parse) -> *mut Vdbe;
    fn sqlite3TwoPartName(
        __v494: *mut Parse,
        __v495: *mut Token,
        __v496: *mut Token,
        __v497: *mut *mut Token,
    ) -> i32;
    fn sqlite3ResolveSelfReference(
        __v498: *mut Parse,
        __v499: *mut Table,
        __v500: i32,
        __v501: *mut Expr,
        __v502: *mut ExprList,
    ) -> i32;
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
    __slate_bits_0: __slate_bits::__SlateBits69U0,
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
    u: __SlateRecord168,
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
    __slate_bits_0: __slate_bits::__SlateBits93U0,
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
    __slate_bits_0: __slate_bits::__SlateBits105U0,
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
    __slate_bits_0: __slate_bits::__SlateBits154U0,
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
    __slate_bits_0: __slate_bits::__SlateBits204U0,
    seekHit: u16,
    ub: __SlateRecord206,
    seqCount: i64,
    cacheStatus: u32,
    seekResult: i32,
    pAltCursor: *mut VdbeCursor,
    uc: __SlateRecord207,
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
union __SlateRecord206 {
    pBtx: *mut Btree,
    aAltMap: *mut u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord207 {
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
    pub struct __SlateBits105U0 {
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
    pub struct __SlateBits69U0 {
        #[bits(4)]
        pub notNull: u32,
        #[bits(4)]
        pub eCType: u32,
    }
    #[bitfields::bitfield([u8; 2], c_names = true, new = false, from_into_bits = false, from_traits = false, default = false, debug = false, builder = false, bit_ops = false)]
    pub struct __SlateBits93U0 {
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
    pub struct __SlateBits204U0 {
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
    pub struct __SlateBits154U0 {
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
}

// /* Write error message here */
// /* Database connection */
// /* Which attached DB to vacuum */
// /* Write results here, if not NULL. VACUUM INTO */
static mut aCopy: [u8; 10] = [
    ((1 as i32) as i8) as u8,
    ((1 as i32) as i8) as u8,
    ((3 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((5 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((6 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((8 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
];

// /*
// ** The VACUUM command is used to clean up the database,
// ** collapse free space, etc.  It is modelled after the VACUUM command
// ** in PostgreSQL.  The VACUUM command works as follows:
// **
// **   (1)  Create a new transient database file
// **   (2)  Copy all content from the database being vacuumed into
// **        the new transient database file
// **   (3)  Copy content from the transient database back into the
// **        original database.
// **
// ** The transient database requires temporary disk space approximately
// ** equal to the size of the original database.  The copy operation of
// ** step (3) requires additional temporary disk space approximately equal
// ** to the size of the original database for the rollback journal.
// ** Hence, temporary disk space that is approximately 2x the size of the
// ** original database is required.  Every page of the database is written
// ** approximately 3 times:  Once for step (2) and twice for step (3).
// ** Two writes per page are required in step (3) because the original
// ** database content must be written into the rollback journal prior to
// ** overwriting the database with the vacuumed content.
// **
// ** Only 1x temporary space and only 1x writes would be required if
// ** the copy of step (3) were replaced by deleting the original database
// ** and renaming the transient database as the original.  But that will
// ** not work if other processes are attached to the original database.
// ** And a power loss in between deleting the original and renaming the
// ** transient would cause the database file to appear to be deleted
// ** following reboot.
// */
// /* SQLITE_OMIT_VACUUM && SQLITE_OMIT_ATTACH */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3Vacuum(
    mut pParse: *mut Parse,
    mut pNm: *mut Token,
    mut pInto: *mut Expr,
) {
    let mut __slate_storage_523: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_523: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_523) as *mut i32;
    let mut __slate_storage_522: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_522: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_522) as *mut i32;
    let mut __slate_storage_521: std::mem::MaybeUninit<*mut Parse> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_521: *mut *mut Parse =
        std::ptr::addr_of_mut!(__slate_storage_521) as *mut *mut Parse;
    let mut __slate_storage_520: std::mem::MaybeUninit<bool> = std::mem::MaybeUninit::uninit();
    let __slate_slot_520: *mut bool = std::ptr::addr_of_mut!(__slate_storage_520) as *mut bool;
    let mut __slate_storage_377: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_377: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_377) as *mut i32;
    let mut __slate_storage_376: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_376: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_376) as *mut i32;
    let mut __slate_storage_375: std::mem::MaybeUninit<*mut Vdbe> = std::mem::MaybeUninit::uninit();
    let __slate_slot_375: *mut *mut Vdbe =
        std::ptr::addr_of_mut!(__slate_storage_375) as *mut *mut Vdbe;
    unsafe {
        '__join_0: {
            std::ptr::write(__slate_slot_375, unsafe { sqlite3GetVdbe(pParse) });
            std::ptr::write(__slate_slot_376, 0 as i32);
            if *__slate_slot_375 == std::ptr::null_mut::<Vdbe>() {
            } else {
                if (unsafe { (*pParse).nErr }) != (0 as i32) {
                } else {
                    if pNm != std::ptr::null_mut::<Token>() {
                        // /* Default behavior:  Report an error if the argument to VACUUM is
                        //     ** not recognized */
                        *__slate_slot_376 = unsafe {
                            sqlite3TwoPartName(pParse, pNm, pNm, std::ptr::addr_of_mut!(pNm))
                        };
                        if *__slate_slot_376 < (0 as i32) {
                            break '__join_0;
                        }
                    }
                    if *__slate_slot_376 != (1 as i32) {
                        std::ptr::write(__slate_slot_377, 0 as i32);
                        if pInto != std::ptr::null_mut::<Expr>() {
                            *__slate_slot_520 = (unsafe {
                                sqlite3ResolveSelfReference(
                                    pParse,
                                    std::ptr::null_mut::<Table>(),
                                    0 as i32,
                                    pInto,
                                    std::ptr::null_mut::<ExprList>(),
                                )
                            }) == (0 as i32);
                        } else {
                            *__slate_slot_520 = false as bool;
                        }
                        if *__slate_slot_520 {
                            std::ptr::write(__slate_slot_521, pParse);
                            std::ptr::write(__slate_slot_522, unsafe {
                                (*(*__slate_slot_521)).nMem
                            });
                            std::ptr::write(__slate_slot_523, *__slate_slot_522 + (1 as i32));
                            unsafe {
                                (*(*__slate_slot_521)).nMem = *__slate_slot_523;
                            }
                            *__slate_slot_377 = *__slate_slot_523;
                            unsafe { sqlite3ExprCode(pParse, pInto, *__slate_slot_377) };
                        }
                        unsafe {
                            sqlite3VdbeAddOp2(
                                *__slate_slot_375,
                                5 as i32,
                                *__slate_slot_376,
                                *__slate_slot_377,
                            )
                        };
                        unsafe { sqlite3VdbeUsesBtree(*__slate_slot_375, *__slate_slot_376) };
                    }
                }
            }
        }
        unsafe { sqlite3ExprDelete(unsafe { (*pParse).db }, pInto) };
        return;
    }
}

// /*
// ** This routine implements the OP_Vacuum opcode of the VDBE.
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3RunVacuum(
    mut pzErrMsg: *mut *mut i8,
    mut db: *mut sqlite3,
    mut iDb: i32,
    mut pOut: *mut sqlite3_value,
) -> i32 {
    let mut __slate_storage_550: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_550: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_550) as *mut i32;
    let mut __slate_storage_549: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_549: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_549) as *mut i32;
    let mut __slate_storage_406: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_406: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_406) as *mut i32;
    let mut __slate_storage_405: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_405: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_405) as *mut u32;
    let mut __slate_storage_548: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_548: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_548) as *mut u32;
    let mut __slate_storage_547: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_547: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_547) as *mut u32;
    let mut __slate_storage_546: std::mem::MaybeUninit<*mut sqlite3> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_546: *mut *mut sqlite3 =
        std::ptr::addr_of_mut!(__slate_storage_546) as *mut *mut sqlite3;
    let mut __slate_storage_545: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_545: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_545) as *mut i32;
    let mut __slate_storage_544: std::mem::MaybeUninit<bool> = std::mem::MaybeUninit::uninit();
    let __slate_slot_544: *mut bool = std::ptr::addr_of_mut!(__slate_storage_544) as *mut bool;
    let mut __slate_storage_543: std::mem::MaybeUninit<bool> = std::mem::MaybeUninit::uninit();
    let __slate_slot_543: *mut bool = std::ptr::addr_of_mut!(__slate_storage_543) as *mut bool;
    let mut __slate_storage_404: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_404: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_404) as *mut i32;
    let mut __slate_storage_542: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_542: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_542) as *mut u32;
    let mut __slate_storage_541: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_541: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_541) as *mut u32;
    let mut __slate_storage_540: std::mem::MaybeUninit<*mut sqlite3> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_540: *mut *mut sqlite3 =
        std::ptr::addr_of_mut!(__slate_storage_540) as *mut *mut sqlite3;
    let mut __slate_storage_539: std::mem::MaybeUninit<bool> = std::mem::MaybeUninit::uninit();
    let __slate_slot_539: *mut bool = std::ptr::addr_of_mut!(__slate_storage_539) as *mut bool;
    let mut __slate_storage_403: std::mem::MaybeUninit<*const i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_403: *mut *const i8 =
        std::ptr::addr_of_mut!(__slate_storage_403) as *mut *const i8;
    let mut __slate_storage_402: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_402: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_402) as *mut i64;
    let mut __slate_storage_401: std::mem::MaybeUninit<*mut sqlite3_file> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_401: *mut *mut sqlite3_file =
        std::ptr::addr_of_mut!(__slate_storage_401) as *mut *mut sqlite3_file;
    let mut __slate_storage_538: std::mem::MaybeUninit<u64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_538: *mut u64 = std::ptr::addr_of_mut!(__slate_storage_538) as *mut u64;
    let mut __slate_storage_537: std::mem::MaybeUninit<u64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_537: *mut u64 = std::ptr::addr_of_mut!(__slate_storage_537) as *mut u64;
    let mut __slate_storage_536: std::mem::MaybeUninit<*mut sqlite3> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_536: *mut *mut sqlite3 =
        std::ptr::addr_of_mut!(__slate_storage_536) as *mut *mut sqlite3;
    let mut __slate_storage_535: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_535: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_535) as *mut u32;
    let mut __slate_storage_534: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_534: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_534) as *mut u32;
    let mut __slate_storage_533: std::mem::MaybeUninit<*mut sqlite3> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_533: *mut *mut sqlite3 =
        std::ptr::addr_of_mut!(__slate_storage_533) as *mut *mut sqlite3;
    let mut __slate_storage_532: std::mem::MaybeUninit<u64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_532: *mut u64 = std::ptr::addr_of_mut!(__slate_storage_532) as *mut u64;
    let mut __slate_storage_531: std::mem::MaybeUninit<u64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_531: *mut u64 = std::ptr::addr_of_mut!(__slate_storage_531) as *mut u64;
    let mut __slate_storage_530: std::mem::MaybeUninit<*mut sqlite3> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_530: *mut *mut sqlite3 =
        std::ptr::addr_of_mut!(__slate_storage_530) as *mut *mut sqlite3;
    let mut __slate_storage_529: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_529: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_529) as *mut u32;
    let mut __slate_storage_528: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_528: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_528) as *mut u32;
    let mut __slate_storage_527: std::mem::MaybeUninit<*mut sqlite3> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_527: *mut *mut sqlite3 =
        std::ptr::addr_of_mut!(__slate_storage_527) as *mut *mut sqlite3;
    let mut __slate_storage_526: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_526: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_526) as *mut u32;
    let mut __slate_storage_525: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_525: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_525) as *mut u32;
    let mut __slate_storage_524: std::mem::MaybeUninit<*mut sqlite3> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_524: *mut *mut sqlite3 =
        std::ptr::addr_of_mut!(__slate_storage_524) as *mut *mut sqlite3;
    let mut __slate_storage_400: std::mem::MaybeUninit<__SlateAlign16<[i8; 42]>> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_400: *mut [i8; 42] =
        std::ptr::addr_of_mut!(__slate_storage_400) as *mut [i8; 42];
    let mut __slate_storage_399: std::mem::MaybeUninit<u64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_399: *mut u64 = std::ptr::addr_of_mut!(__slate_storage_399) as *mut u64;
    let mut __slate_storage_398: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_398: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_398) as *mut u32;
    let mut __slate_storage_397: std::mem::MaybeUninit<*const i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_397: *mut *const i8 =
        std::ptr::addr_of_mut!(__slate_storage_397) as *mut *const i8;
    let mut __slate_storage_396: std::mem::MaybeUninit<*const i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_396: *mut *const i8 =
        std::ptr::addr_of_mut!(__slate_storage_396) as *mut *const i8;
    let mut __slate_storage_395: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_395: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_395) as *mut i32;
    let mut __slate_storage_394: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_394: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_394) as *mut i32;
    let mut __slate_storage_393: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_393: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_393) as *mut i32;
    let mut __slate_storage_392: std::mem::MaybeUninit<*mut Db> = std::mem::MaybeUninit::uninit();
    let __slate_slot_392: *mut *mut Db =
        std::ptr::addr_of_mut!(__slate_storage_392) as *mut *mut Db;
    let mut __slate_storage_391: std::mem::MaybeUninit<u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_391: *mut u8 = std::ptr::addr_of_mut!(__slate_storage_391) as *mut u8;
    let mut __slate_storage_390: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_390: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_390) as *mut u32;
    let mut __slate_storage_389: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_389: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_389) as *mut i64;
    let mut __slate_storage_388: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_388: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_388) as *mut i64;
    let mut __slate_storage_387: std::mem::MaybeUninit<u64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_387: *mut u64 = std::ptr::addr_of_mut!(__slate_storage_387) as *mut u64;
    let mut __slate_storage_386: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_386: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_386) as *mut u32;
    let mut __slate_storage_385: std::mem::MaybeUninit<*mut Btree> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_385: *mut *mut Btree =
        std::ptr::addr_of_mut!(__slate_storage_385) as *mut *mut Btree;
    let mut __slate_storage_384: std::mem::MaybeUninit<*mut Btree> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_384: *mut *mut Btree =
        std::ptr::addr_of_mut!(__slate_storage_384) as *mut *mut Btree;
    let mut __slate_storage_383: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_383: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_383) as *mut i32;
    unsafe {
        // /* Return code from service routines */
        std::ptr::write(__slate_slot_383, 0 as i32);
        // /* The database being vacuumed */
        // /* The temporary database we vacuum into */
        // /* Saved value of db->mDbFlags */
        // /* Saved value of db->flags */
        // /* Saved value of db->nChange */
        // /* Saved value of db->nTotalChange */
        // /* Saved value of db->openFlags */
        // /* Saved trace settings */
        // /* Database to detach at end of vacuum */
        std::ptr::write(__slate_slot_392, std::ptr::null_mut::<Db>());
        // /* True if vacuuming a :memory: database */
        // /* Bytes of reserved space at the end of each page */
        // /* Number of attached databases */
        // /* Schema name of database to vacuum */
        // /* Name of output file */
        // /* sync flags for output db */
        std::ptr::write(__slate_slot_398, (1 as i32) as u32);
        // /* Random value used for zDbVacuum[] */
        // /* Name of the ATTACH-ed database used for vacuum */
        if !((unsafe { (*db).autoCommit }) != (0 as u8)) {
            unsafe {
                sqlite3SetString(
                    pzErrMsg,
                    db,
                    (b"cannot VACUUM from within a transaction\0".as_ptr() as *mut i8) as *const i8,
                )
            };
            // /* IMP: R-12218-18073 */
            return 1 as i32;
        } else {
            if (unsafe { (*db).nVdbeActive }) > (1 as i32) {
                unsafe {
                    sqlite3SetString(
                        pzErrMsg,
                        db,
                        (b"cannot VACUUM - SQL statements in progress\0".as_ptr() as *mut i8)
                            as *const i8,
                    )
                };
                // /* IMP: R-15610-35227 */
                return 1 as i32;
            } else {
                *__slate_slot_390 = unsafe { (*db).openFlags };
                if pOut != std::ptr::null_mut::<sqlite3_value>() {
                    if (unsafe { sqlite3_value_type(pOut) }) != (3 as i32) {
                        unsafe {
                            sqlite3SetString(
                                pzErrMsg,
                                db,
                                (b"non-text filename\0".as_ptr() as *mut i8) as *const i8,
                            )
                        };
                        return 1 as i32;
                    } else {
                        *__slate_slot_397 = (unsafe { sqlite3_value_text(pOut) }) as *const i8;
                        std::ptr::write(__slate_slot_524, db);
                        std::ptr::write(__slate_slot_525, unsafe {
                            (*(*__slate_slot_524)).openFlags
                        });
                        std::ptr::write(__slate_slot_526, *__slate_slot_525 & (!(1 as i32) as u32));
                        unsafe {
                            (*(*__slate_slot_524)).openFlags = *__slate_slot_526;
                        }
                        std::ptr::write(__slate_slot_527, db);
                        std::ptr::write(__slate_slot_528, unsafe {
                            (*(*__slate_slot_527)).openFlags
                        });
                        std::ptr::write(
                            __slate_slot_529,
                            *__slate_slot_528 | (((4 as i32) | (2 as i32)) as u32),
                        );
                        unsafe {
                            (*(*__slate_slot_527)).openFlags = *__slate_slot_529;
                        }
                    }
                } else {
                    *__slate_slot_397 = (b"\0".as_ptr() as *mut i8) as *const i8;
                }
                '__join_2: {
                    // /* Save the current value of the database flags so that it can be
                    //   ** restored before returning. Then set the writable-schema flag, and
                    //   ** disable CHECK and foreign key constraints.  */
                    *__slate_slot_387 = unsafe { (*db).flags };
                    *__slate_slot_386 = unsafe { (*db).mDbFlags };
                    *__slate_slot_388 = unsafe { (*db).nChange };
                    *__slate_slot_389 = unsafe { (*db).nTotalChange };
                    *__slate_slot_391 = unsafe { (*db).mTrace };
                    std::ptr::write(__slate_slot_530, db);
                    std::ptr::write(__slate_slot_531, unsafe { (*(*__slate_slot_530)).flags });
                    std::ptr::write(
                        __slate_slot_532,
                        *__slate_slot_531
                            | (((((1 as i32) | (512 as i32)) as i64) as u64)
                                | (((64 as i32) as i64) as u64) << (32 as i32)
                                | (((16 as i32) as i64) as u64) << (32 as i32)
                                | (((32 as i32) as i64) as u64) << (32 as i32)),
                    );
                    unsafe {
                        (*(*__slate_slot_530)).flags = *__slate_slot_532;
                    }
                    std::ptr::write(__slate_slot_533, db);
                    std::ptr::write(__slate_slot_534, unsafe { (*(*__slate_slot_533)).mDbFlags });
                    std::ptr::write(
                        __slate_slot_535,
                        *__slate_slot_534 | (((2 as i32) | (4 as i32)) as u32),
                    );
                    unsafe {
                        (*(*__slate_slot_533)).mDbFlags = *__slate_slot_535;
                    }
                    std::ptr::write(__slate_slot_536, db);
                    std::ptr::write(__slate_slot_537, unsafe { (*(*__slate_slot_536)).flags });
                    std::ptr::write(
                        __slate_slot_538,
                        *__slate_slot_537
                            & !(((((16384 as i32) | (4096 as i32) | (268435456 as i32)) as i64)
                                as u64)
                                | (((1 as i32) as i64) as u64) << (32 as i32)),
                    );
                    unsafe {
                        (*(*__slate_slot_536)).flags = *__slate_slot_538;
                    }
                    unsafe {
                        (*db).mTrace = ((0 as i32) as i8) as u8;
                    }
                    *__slate_slot_396 = (unsafe {
                        (*unsafe { unsafe { (*db).aDb }.offset(iDb as isize) }).zDbSName
                    }) as *const i8;
                    *__slate_slot_384 =
                        unsafe { (*unsafe { unsafe { (*db).aDb }.offset(iDb as isize) }).pBt };
                    *__slate_slot_393 = unsafe {
                        sqlite3PagerIsMemdb(unsafe { sqlite3BtreePager(*__slate_slot_384) })
                    };
                    // /* Attach the temporary database as 'vacuum_XXXXXX'. The synchronous pragma
                    //   ** can be set to 'off' for this file, as it is not recovered if a crash
                    //   ** occurs anyway. The integrity of the database is maintained by a
                    //   ** (possibly synchronous) transaction opened on the main database before
                    //   ** sqlite3BtreeCopyFile() is called.
                    //   **
                    //   ** An optimization would be to use a non-journaled pager.
                    //   ** (Later:) I tried setting "PRAGMA vacuum_XXXXXX.journal_mode=OFF" but
                    //   ** that actually made the VACUUM run slower.  Very little journalling
                    //   ** actually occurs when doing a vacuum since the vacuum_db is initially
                    //   ** empty.  Only the journal header is written.  Apparently it takes more
                    //   ** time to parse and run the PRAGMA to turn journalling off than it does
                    //   ** to write the journal header file.
                    //   */
                    unsafe {
                        sqlite3_randomness(
                            ((8 as u64) as u32) as i32,
                            std::ptr::addr_of_mut!(*__slate_slot_399) as *mut (),
                        )
                    };
                    unsafe {
                        sqlite3_snprintf(
                            ((42 as u64) as u32) as i32,
                            (*__slate_slot_400).as_mut_ptr() as *mut i8,
                            (b"vacuum_%016llx\0".as_ptr() as *mut i8) as *const i8,
                            *__slate_slot_399,
                        )
                    };
                    *__slate_slot_395 = unsafe { (*db).nDb };
                    *__slate_slot_383 = unsafe {
                        execSqlF(
                            db,
                            pzErrMsg,
                            (b"ATTACH %Q AS %s\0".as_ptr() as *mut i8) as *const i8,
                            *__slate_slot_397,
                            (*__slate_slot_400).as_mut_ptr() as *mut i8,
                        )
                    };
                    unsafe {
                        (*db).openFlags = *__slate_slot_390;
                    }
                    if *__slate_slot_383 != (0 as i32) {
                    } else {
                        0 as i32;
                        *__slate_slot_392 =
                            unsafe { unsafe { (*db).aDb }.offset(*__slate_slot_395 as isize) };
                        0 as i32;
                        *__slate_slot_385 = unsafe { (*(*__slate_slot_392)).pBt };
                        *__slate_slot_394 =
                            unsafe { sqlite3BtreeGetRequestedReserve(*__slate_slot_384) };
                        if pOut != std::ptr::null_mut::<sqlite3_value>() {
                            std::ptr::write(__slate_slot_401, unsafe {
                                sqlite3PagerFile(unsafe { sqlite3BtreePager(*__slate_slot_385) })
                            });
                            std::ptr::write(__slate_slot_402, (0 as i32) as i64);
                            if (unsafe { (*(*__slate_slot_401)).pMethods })
                                != std::ptr::null::<sqlite3_io_methods>()
                            {
                                *__slate_slot_539 = (unsafe {
                                    sqlite3OsFileSize(
                                        *__slate_slot_401,
                                        std::ptr::addr_of_mut!(*__slate_slot_402),
                                    )
                                }) != (0 as i32)
                                    || *__slate_slot_402 > ((0 as i32) as i64);
                            } else {
                                *__slate_slot_539 = false as bool;
                            }
                            if *__slate_slot_539 {
                                *__slate_slot_383 = 1 as i32;
                                unsafe {
                                    sqlite3SetString(
                                        pzErrMsg,
                                        db,
                                        (b"output file already exists\0".as_ptr() as *mut i8)
                                            as *const i8,
                                    )
                                };
                                break '__join_2;
                            } else {
                                std::ptr::write(__slate_slot_540, db);
                                std::ptr::write(__slate_slot_541, unsafe {
                                    (*(*__slate_slot_540)).mDbFlags
                                });
                                std::ptr::write(
                                    __slate_slot_542,
                                    *__slate_slot_541 | ((8 as i32) as u32),
                                );
                                unsafe {
                                    (*(*__slate_slot_540)).mDbFlags = *__slate_slot_542;
                                }
                                // /* For a VACUUM INTO, the pager-flags are set to the same values as
                                //     ** they are for the database being vacuumed, except that PAGER_CACHESPILL
                                //     ** is always set. */
                                *__slate_slot_398 =
                                    ((((((unsafe {
                                        (*unsafe { unsafe { (*db).aDb }.offset(iDb as isize) })
                                            .safety_level
                                    }) as u32) as i32)
                                        as i64) as u64)
                                        | (unsafe { (*db).flags }) & (((56 as i32) as i64) as u64))
                                        as u32;
                                // /* If the VACUUM INTO target file is a URI filename and if the
                                //     ** "reserve=N" query parameter is present, reset the reserve to the
                                //     ** amount specified, if the amount is within range */
                                *__slate_slot_403 =
                                    unsafe { sqlite3BtreeGetFilename(*__slate_slot_385) };
                                if *__slate_slot_403 != std::ptr::null::<i8>() {
                                    std::ptr::write(
                                        __slate_slot_404,
                                        (unsafe {
                                            sqlite3_uri_int64(
                                                *__slate_slot_403,
                                                (b"reserve\0".as_ptr() as *mut i8) as *const i8,
                                                *__slate_slot_394 as i64,
                                            )
                                        }) as i32,
                                    );
                                    if *__slate_slot_404 >= (0 as i32)
                                        && *__slate_slot_404 <= (255 as i32)
                                    {
                                        *__slate_slot_394 = *__slate_slot_404;
                                    }
                                }
                            }
                        }
                        unsafe {
                            sqlite3BtreeSetCacheSize(*__slate_slot_385, unsafe {
                                (*unsafe {
                                    (*unsafe { unsafe { (*db).aDb }.offset(iDb as isize) }).pSchema
                                })
                                .cache_size
                            })
                        };
                        unsafe {
                            sqlite3BtreeSetSpillSize(*__slate_slot_385, unsafe {
                                sqlite3BtreeSetSpillSize(*__slate_slot_384, 0 as i32)
                            })
                        };
                        unsafe {
                            sqlite3BtreeSetPagerFlags(
                                *__slate_slot_385,
                                *__slate_slot_398 | ((32 as i32) as u32),
                            )
                        };
                        // /* Begin a transaction and take an exclusive lock on the main database
                        //   ** file. This is done before the sqlite3BtreeGetPageSize(pMain) call below,
                        //   ** to ensure that we do not try to change the page-size on a WAL database.
                        //   */
                        *__slate_slot_383 =
                            execSql(db, pzErrMsg, (b"BEGIN\0".as_ptr() as *mut i8) as *const i8);
                        if *__slate_slot_383 != (0 as i32) {
                        } else {
                            *__slate_slot_383 = unsafe {
                                sqlite3BtreeBeginTrans(
                                    *__slate_slot_384,
                                    if pOut == std::ptr::null_mut::<sqlite3_value>() {
                                        2 as i32
                                    } else {
                                        0 as i32
                                    },
                                    std::ptr::null_mut::<i32>(),
                                )
                            };
                            if *__slate_slot_383 != (0 as i32) {
                            } else {
                                // /* Do not attempt to change the page size for a WAL database */
                                if (unsafe {
                                    sqlite3PagerGetJournalMode(unsafe {
                                        sqlite3BtreePager(*__slate_slot_384)
                                    })
                                }) == (5 as i32)
                                    && pOut == std::ptr::null_mut::<sqlite3_value>()
                                {
                                    unsafe {
                                        (*db).nextPagesize = 0 as i32;
                                    }
                                }
                                if (unsafe {
                                    sqlite3BtreeSetPageSize(
                                        *__slate_slot_385,
                                        unsafe { sqlite3BtreeGetPageSize(*__slate_slot_384) },
                                        *__slate_slot_394,
                                        0 as i32,
                                    )
                                }) != (0 as i32)
                                {
                                    *__slate_slot_543 = true as bool;
                                } else {
                                    if !(*__slate_slot_393 != (0 as i32)) {
                                        *__slate_slot_544 = (unsafe {
                                            sqlite3BtreeSetPageSize(
                                                *__slate_slot_385,
                                                unsafe { (*db).nextPagesize },
                                                *__slate_slot_394,
                                                0 as i32,
                                            )
                                        }) != (0 as i32);
                                    } else {
                                        *__slate_slot_544 = false as bool;
                                    }
                                    *__slate_slot_543 = *__slate_slot_544;
                                }
                                if *__slate_slot_543 || (unsafe { (*db).mallocFailed }) != (0 as u8)
                                {
                                    *__slate_slot_383 = 7 as i32;
                                } else {
                                    if ((unsafe { (*db).nextAutovac }) as i32) >= (0 as i32) {
                                        *__slate_slot_545 = (unsafe { (*db).nextAutovac }) as i32;
                                    } else {
                                        *__slate_slot_545 =
                                            unsafe { sqlite3BtreeGetAutoVacuum(*__slate_slot_384) };
                                    }
                                    unsafe {
                                        sqlite3BtreeSetAutoVacuum(
                                            *__slate_slot_385,
                                            *__slate_slot_545,
                                        )
                                    };
                                    // /* Query the schema of the main database. Create a mirror schema
                                    //   ** in the temporary database.
                                    //   */
                                    // /* force new CREATE statements into vacuum_db */
                                    unsafe {
                                        (*db).init.iDb = (*__slate_slot_395 as i8) as u8;
                                    }
                                    *__slate_slot_383 = unsafe {
                                        execSqlF(db, pzErrMsg, (b"SELECT sql FROM \"%w\".sqlite_schema WHERE type='table'AND name<>'sqlite_sequence' AND coalesce(rootpage,1)>0\0".as_ptr() as *mut i8) as *const i8, *__slate_slot_396)
                                    };
                                    if *__slate_slot_383 != (0 as i32) {
                                    } else {
                                        *__slate_slot_383 = unsafe {
                                            execSqlF(db, pzErrMsg, (b"SELECT sql FROM \"%w\".sqlite_schema WHERE type='index'\0".as_ptr() as *mut i8) as *const i8, *__slate_slot_396)
                                        };
                                        if *__slate_slot_383 != (0 as i32) {
                                        } else {
                                            unsafe {
                                                (*db).init.iDb = ((0 as i32) as i8) as u8;
                                            }
                                            // /* Loop through the tables in the main database. For each, do
                                            //   ** an "INSERT INTO vacuum_db.xxx SELECT * FROM main.xxx;" to copy
                                            //   ** the contents to the temporary database.
                                            //   */
                                            *__slate_slot_383 = unsafe {
                                                execSqlF(db, pzErrMsg, (b"SELECT'INSERT INTO %s.'||quote(name)||' SELECT*FROM\"%w\".'||quote(name)FROM %s.sqlite_schema WHERE type='table'AND coalesce(rootpage,1)>0\0".as_ptr() as *mut i8) as *const i8, (*__slate_slot_400).as_mut_ptr() as *mut i8, *__slate_slot_396, (*__slate_slot_400).as_mut_ptr() as *mut i8)
                                            };
                                            0 as i32;
                                            std::ptr::write(__slate_slot_546, db);
                                            std::ptr::write(__slate_slot_547, unsafe {
                                                (*(*__slate_slot_546)).mDbFlags
                                            });
                                            std::ptr::write(
                                                __slate_slot_548,
                                                *__slate_slot_547 & (!(4 as i32) as u32),
                                            );
                                            unsafe {
                                                (*(*__slate_slot_546)).mDbFlags = *__slate_slot_548;
                                            }
                                            if *__slate_slot_383 != (0 as i32) {
                                            } else {
                                                // /* Copy the triggers, views, and virtual tables from the main database
                                                //   ** over to the temporary database.  None of these objects has any
                                                //   ** associated storage, so all we have to do is copy their entries
                                                //   ** from the schema table.
                                                //   */
                                                *__slate_slot_383 = unsafe {
                                                    execSqlF(db, pzErrMsg, (b"INSERT INTO %s.sqlite_schema SELECT*FROM \"%w\".sqlite_schema WHERE type IN('view','trigger') OR(type='table'AND rootpage=0)\0".as_ptr() as *mut i8) as *const i8, (*__slate_slot_400).as_mut_ptr() as *mut i8, *__slate_slot_396)
                                                };
                                                if *__slate_slot_383 != (0 as i32) {
                                                } else {
                                                    // /* At this point, there is a write transaction open on both the
                                                    //   ** vacuum database and the main database. Assuming no error occurs,
                                                    //   ** both transactions are closed by this block - the main database
                                                    //   ** transaction by sqlite3BtreeCopyFile() and the other by an explicit
                                                    //   ** call to sqlite3BtreeCommit().
                                                    //   */
                                                    // /* This array determines which meta meta values are preserved in the
                                                    //     ** vacuum.  Even entries are the meta value number and odd entries
                                                    //     ** are an increment to apply to the meta value after the vacuum.
                                                    //     ** The increment is used to increase the schema cookie so that other
                                                    //     ** connections to the same database will know to reread the schema.
                                                    //     */
                                                    // /* Add one to the old schema cookie */
                                                    // /* Preserve the default page cache size */
                                                    // /* Preserve the text encoding */
                                                    // /* Preserve the user version */
                                                    // /* Preserve the application id */
                                                    0 as i32;
                                                    0 as i32;
                                                    // /* Copy Btree meta values */
                                                    *__slate_slot_406 = 0 as i32;
                                                    loop {
                                                        if *__slate_slot_406
                                                            < ((((10 as u64) / (1 as u64)) as u32)
                                                                as i32)
                                                        {
                                                            // /* GetMeta() and UpdateMeta() cannot fail in this context because
                                                            //       ** we already have page 1 loaded into cache and marked dirty. */
                                                            unsafe {
                                                                sqlite3BtreeGetMeta(
                                                                    *__slate_slot_384,
                                                                    ((unsafe {
                                                                        *unsafe {
                                                                            unsafe {
                                                                                std::ptr::addr_of!(
                                                                                    aCopy
                                                                                )
                                                                                    as *const u8
                                                                            }
                                                                            .offset(
                                                                                *__slate_slot_406
                                                                                    as isize,
                                                                            )
                                                                        }
                                                                    })
                                                                        as u32)
                                                                        as i32,
                                                                    std::ptr::addr_of_mut!(
                                                                        *__slate_slot_405
                                                                    ),
                                                                )
                                                            };
                                                            *__slate_slot_383 = unsafe {
                                                                sqlite3BtreeUpdateMeta(*__slate_slot_385, ((unsafe { *unsafe { unsafe { std::ptr::addr_of!(aCopy) as *const u8 }.offset(*__slate_slot_406 as isize) } }) as u32) as i32, (*__slate_slot_405).wrapping_add((((unsafe { *unsafe { unsafe { std::ptr::addr_of!(aCopy) as *const u8 }.offset((*__slate_slot_406 + (1 as i32)) as isize) } }) as u32) as i32) as u32))
                                                            };
                                                            if *__slate_slot_383 != (0 as i32) {
                                                                break '__join_2;
                                                            } else {
                                                                std::ptr::write(
                                                                    __slate_slot_549,
                                                                    *__slate_slot_406,
                                                                );
                                                                std::ptr::write(
                                                                    __slate_slot_550,
                                                                    *__slate_slot_549 + (2 as i32),
                                                                );
                                                                *__slate_slot_406 =
                                                                    *__slate_slot_550;
                                                            }
                                                        } else {
                                                            break;
                                                        }
                                                    }
                                                    if pOut == std::ptr::null_mut::<sqlite3_value>()
                                                    {
                                                        *__slate_slot_383 = unsafe {
                                                            sqlite3BtreeCopyFile(
                                                                *__slate_slot_384,
                                                                *__slate_slot_385,
                                                            )
                                                        };
                                                    }
                                                    if *__slate_slot_383 != (0 as i32) {
                                                    } else {
                                                        *__slate_slot_383 = unsafe {
                                                            sqlite3BtreeCommit(*__slate_slot_385)
                                                        };
                                                        if *__slate_slot_383 != (0 as i32) {
                                                        } else {
                                                            '__join_4: {
                                                                if pOut
                                                                    == std::ptr::null_mut::<
                                                                        sqlite3_value,
                                                                    >(
                                                                    )
                                                                {
                                                                    unsafe {
                                                                        sqlite3BtreeSetAutoVacuum(
                                                                            *__slate_slot_384,
                                                                            unsafe {
                                                                                sqlite3BtreeGetAutoVacuum(*__slate_slot_385)
                                                                            },
                                                                        )
                                                                    };
                                                                }
                                                            }
                                                            0 as i32;
                                                            if pOut
                                                                == std::ptr::null_mut::<sqlite3_value>(
                                                                )
                                                            {
                                                                *__slate_slot_394 = unsafe {
                                                                    sqlite3BtreeGetRequestedReserve(
                                                                        *__slate_slot_385,
                                                                    )
                                                                };
                                                                *__slate_slot_383 = unsafe {
                                                                    sqlite3BtreeSetPageSize(
                                                                        *__slate_slot_384,
                                                                        unsafe {
                                                                            sqlite3BtreeGetPageSize(
                                                                                *__slate_slot_385,
                                                                            )
                                                                        },
                                                                        *__slate_slot_394,
                                                                        1 as i32,
                                                                    )
                                                                };
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
                unsafe {
                    (*db).init.iDb = ((0 as i32) as i8) as u8;
                }
                // /* Restore the original value of db->flags */
                unsafe {
                    (*db).mDbFlags = *__slate_slot_386;
                }
                unsafe {
                    (*db).flags = *__slate_slot_387;
                }
                unsafe {
                    (*db).nChange = *__slate_slot_388;
                }
                unsafe {
                    (*db).nTotalChange = *__slate_slot_389;
                }
                unsafe {
                    (*db).mTrace = *__slate_slot_391;
                }
                unsafe {
                    sqlite3BtreeSetPageSize(*__slate_slot_384, -(1 as i32), 0 as i32, 1 as i32)
                };
                // /* Currently there is an SQL level transaction open on the vacuum
                //   ** database. No locks are held on any other files (since the main file
                //   ** was committed at the btree level). So it safe to end the transaction
                //   ** by manually setting the autoCommit flag to true and detaching the
                //   ** vacuum database. The vacuum_db journal file is deleted when the pager
                //   ** is closed by the DETACH.
                //   */
                unsafe {
                    (*db).autoCommit = ((1 as i32) as i8) as u8;
                }
                if *__slate_slot_392 != std::ptr::null_mut::<Db>() {
                    unsafe { sqlite3BtreeClose(unsafe { (*(*__slate_slot_392)).pBt }) };
                    unsafe {
                        (*(*__slate_slot_392)).pBt = std::ptr::null_mut::<Btree>();
                    }
                    unsafe {
                        (*(*__slate_slot_392)).pSchema = std::ptr::null_mut::<Schema>();
                    }
                }
                // /* This both clears the schemas and reduces the size of the db->aDb[]
                //   ** array. */
                unsafe { sqlite3ResetAllSchemasOfConnection(db) };
                return *__slate_slot_383;
            }
        }
    }
    return unsafe { std::mem::zeroed() };
}

// /*
// ** 2003 April 6
// **
// ** The author disclaims copyright to this source code.  In place of
// ** a legal notice, here is a blessing:
// **
// **    May you do good and not evil.
// **    May you find forgiveness for yourself and forgive others.
// **    May you share freely, never taking more than you give.
// **
// *************************************************************************
// ** This file contains code used to implement the VACUUM command.
// **
// ** Most of the code in this file may be omitted by defining the
// ** SQLITE_OMIT_VACUUM macro.
// */
// /*
// ** Execute zSql on database db.
// **
// ** If zSql returns rows, then each row will have exactly one
// ** column.  (This will only happen if zSql begins with "SELECT".)
// ** Take each row of result and call execSql() again recursively.
// **
// ** The execSqlF() routine does the same thing, except it accepts
// ** a format string as its third argument
// */
fn execSql(mut db: *mut sqlite3, mut pzErrMsg: *mut *mut i8, mut zSql: *const i8) -> i32 {
    let mut pStmt: *mut sqlite3_stmt = unsafe { std::mem::zeroed() };
    let mut rc: i32 = 0 as i32;
    // /* printf("SQL: [%s]\n", zSql); fflush(stdout); */
    rc = unsafe {
        sqlite3_prepare_v2(
            db,
            zSql,
            -(1 as i32),
            std::ptr::addr_of_mut!(pStmt),
            std::ptr::null_mut::<*const i8>(),
        )
    };
    if rc != (0 as i32) {
        return rc;
    }
    '__slate_break_503: loop {
        let __v551: i32 = unsafe { sqlite3_step(pStmt) };
        rc = __v551;
        if !((100 as i32) == __v551) {
            break;
        }
        let mut zSubSql: *const i8 = (unsafe { sqlite3_column_text(pStmt, 0 as i32) }) as *const i8;
        0 as i32;
        // /* The secondary SQL must be one of CREATE TABLE, CREATE INDEX,
        //     ** or INSERT.  Historically there have been attacks that first
        //     ** corrupt the sqlite_schema.sql field with other kinds of statements
        //     ** then run VACUUM to get those statements to execute at inappropriate
        //     ** times. */
        if zSubSql != std::ptr::null::<i8>()
            && ((unsafe {
                strncmp(
                    zSubSql,
                    (b"CRE\0".as_ptr() as *mut i8) as *const i8,
                    ((3 as i32) as i64) as u64,
                )
            }) == (0 as i32)
                || (unsafe {
                    strncmp(
                        zSubSql,
                        (b"INS\0".as_ptr() as *mut i8) as *const i8,
                        ((3 as i32) as i64) as u64,
                    )
                }) == (0 as i32))
        {
            rc = execSql(db, pzErrMsg, zSubSql);
            if rc != (0 as i32) {
                break '__slate_break_503;
            }
        }
    }
    0 as i32;
    if rc == (101 as i32) {
        rc = 0 as i32;
    }
    if rc != (0 as i32) {
        unsafe { sqlite3SetString(pzErrMsg, db, unsafe { sqlite3_errmsg(db) }) };
    }
    unsafe { sqlite3_finalize(pStmt) };
    return rc;
}

unsafe extern "C-unwind" fn execSqlF(
    mut db: *mut sqlite3,
    mut pzErrMsg: *mut *mut i8,
    mut zSql: *const i8,
    mut __va_args: ...
) -> i32 {
    let mut z: *mut i8 = unsafe { std::mem::zeroed() };
    let mut ap: core::ffi::VaList<'_> = unsafe { std::mem::zeroed() };
    let mut rc: i32 = 0 as i32;
    ap = __va_args.clone();
    z = unsafe { sqlite3VMPrintf(db, zSql, ap.clone()) };
    {}
    if z == std::ptr::null_mut::<i8>() {
        return 7 as i32;
    }
    rc = execSql(db, pzErrMsg, z as *const i8);
    unsafe { sqlite3DbFree(db, z as *mut ()) };
    return rc;
}
