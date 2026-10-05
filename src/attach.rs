unsafe extern "C" {
    fn sqlite3_snprintf(__v450: i32, __v451: *mut i8, __v452: *const i8, ...) -> *mut i8;
    fn sqlite3_free(__v453: *mut ());
    fn sqlite3_free_filename(__v454: *const i8);
    fn sqlite3_value_text(__v455: *mut sqlite3_value) -> *const u8;
    fn sqlite3_context_db_handle(__v456: *mut sqlite3_context) -> *mut sqlite3;
    fn sqlite3_result_error(__v457: *mut sqlite3_context, __v458: *const i8, __v459: i32);
    fn sqlite3_result_error_code(__v460: *mut sqlite3_context, __v461: i32);
    fn sqlite3_vfs_find(zVfsName: *const i8) -> *mut sqlite3_vfs;
    fn memcpy(__dest: *mut (), __src: *const (), __n: u64) -> *mut ();
    fn memset(__s: *mut (), __c: i32, __n: u64) -> *mut ();
    fn sqlite3PagerLockingMode(__v469: *mut Pager, __v470: i32) -> i32;
    fn sqlite3BtreeOpen(
        pVfs: *mut sqlite3_vfs,
        zFilename: *const i8,
        db: *mut sqlite3,
        ppBtree: *mut *mut Btree,
        flags: i32,
        vfsFlags: i32,
    ) -> i32;
    fn sqlite3BtreeClose(__v477: *mut Btree) -> i32;
    fn sqlite3BtreeSetPagerFlags(__v478: *mut Btree, __v479: u32) -> i32;
    fn sqlite3BtreeSecureDelete(__v480: *mut Btree, __v481: i32) -> i32;
    fn sqlite3BtreeTxnState(__v482: *mut Btree) -> i32;
    fn sqlite3BtreeIsInBackup(__v483: *mut Btree) -> i32;
    fn sqlite3BtreePager(__v484: *mut Btree) -> *mut Pager;
    fn sqlite3BtreeEnter(__v485: *mut Btree);
    fn sqlite3BtreeEnterAll(__v486: *mut sqlite3);
    fn sqlite3BtreeLeave(__v487: *mut Btree);
    fn sqlite3BtreeLeaveAll(__v488: *mut sqlite3);
    fn sqlite3VdbeAddOp1(__v489: *mut Vdbe, __v490: i32, __v491: i32) -> i32;
    fn sqlite3VdbeAddFunctionCall(
        __v492: *mut Parse,
        __v493: i32,
        __v494: i32,
        __v495: i32,
        __v496: i32,
        __v497: *const FuncDef,
        __v498: i32,
    ) -> i32;
    fn sqlite3WalkExpr(__v499: *mut Walker, __v500: *mut Expr) -> i32;
    fn sqlite3WalkExprList(__v501: *mut Walker, __v502: *mut ExprList) -> i32;
    fn sqlite3WalkSelect(__v503: *mut Walker, __v504: *mut Select) -> i32;
    fn sqlite3WalkWinDefnDummyCallback(__v505: *mut Walker, __v506: *mut Select);
    fn sqlite3StrICmp(__v507: *const i8, __v508: *const i8) -> i32;
    fn sqlite3DbMallocRawNN(__v509: *mut sqlite3, __v510: u64) -> *mut ();
    fn sqlite3DbStrDup(__v511: *mut sqlite3, __v512: *const i8) -> *mut i8;
    fn sqlite3DbRealloc(__v513: *mut sqlite3, __v514: *mut (), __v515: u64) -> *mut ();
    fn sqlite3DbFree(__v516: *mut sqlite3, __v517: *mut ());
    fn sqlite3MPrintf(__v518: *mut sqlite3, __v519: *const i8, ...) -> *mut i8;
    fn sqlite3ErrorMsg(__v520: *mut Parse, __v521: *const i8, ...);
    fn sqlite3GetTempRange(__v522: *mut Parse, __v523: i32) -> i32;
    fn sqlite3ExprDelete(__v524: *mut sqlite3, __v525: *mut Expr);
    fn sqlite3Init(__v526: *mut sqlite3, __v527: *mut *mut i8) -> i32;
    fn sqlite3ResetAllSchemasOfConnection(__v528: *mut sqlite3);
    fn sqlite3CollapseDatabaseArray(__v529: *mut sqlite3);
    fn sqlite3ParseUri(
        __v530: *const i8,
        __v531: *const i8,
        __v532: *mut u32,
        __v533: *mut *mut sqlite3_vfs,
        __v534: *mut *mut i8,
        __v535: *mut *mut i8,
    ) -> i32;
    fn sqlite3ExprCode(__v536: *mut Parse, __v537: *mut Expr, __v538: i32);
    fn sqlite3GetVdbe(__v539: *mut Parse) -> *mut Vdbe;
    fn sqlite3AuthCheck(
        __v540: *mut Parse,
        __v541: i32,
        __v542: *const i8,
        __v543: *const i8,
        __v544: *const i8,
    ) -> i32;
    fn sqlite3ReadSchema(pParse: *mut Parse) -> i32;
    fn sqlite3ResolveExprNames(__v568: *mut NameContext, __v569: *mut Expr) -> i32;
    fn sqlite3FindDbName(__v570: *mut sqlite3, __v571: *const i8) -> i32;
    fn sqlite3SchemaGet(__v572: *mut sqlite3, __v573: *mut Btree) -> *mut Schema;
    fn sqlite3OomFault(__v574: *mut sqlite3) -> *mut ();
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
    trace: __SlateRecord162,
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
    u1: __SlateRecord163,
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
    __slate_bits_0: __slate_bits::__SlateBits61U0,
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
struct DbFixer {
    pParse: *mut Parse,
    w: Walker,
    pSchema: *mut Schema,
    bTemp: u8,
    zDb: *const i8,
    zType: *const i8,
    pName: *const Token,
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
    u: __SlateRecord173,
    pLeft: *mut Expr,
    pRight: *mut Expr,
    x: __SlateRecord174,
    nHeight: i32,
    iTable: i32,
    iColumn: i16,
    iAgg: i16,
    w: __SlateRecord175,
    pAggInfo: *mut AggInfo,
    y: __SlateRecord176,
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
    u: __SlateRecord164,
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
    __slate_bits_0: __slate_bits::__SlateBits87U0,
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
struct NameContext {
    pParse: *mut Parse,
    pSrcList: *mut SrcList,
    uNC: __SlateRecord188,
    pNext: *mut NameContext,
    nRef: i32,
    nNcErr: i32,
    ncFlags: i32,
    nNestedSelect: u32,
    pWinSelect: *mut Select,
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
    u1: __SlateRecord190,
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
    fg: __SlateRecord183,
    iCursor: i32,
    colUsed: u64,
    u1: __SlateRecord184,
    u2: __SlateRecord185,
    u3: __SlateRecord186,
    u4: __SlateRecord187,
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
    u: __SlateRecord165,
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
struct Walker {
    pParse: *mut Parse,
    xExprCallback: Option<unsafe extern "C-unwind" fn(*mut Walker, *mut Expr) -> i32>,
    xSelectCallback: Option<unsafe extern "C-unwind" fn(*mut Walker, *mut Select) -> i32>,
    xSelectCallback2: Option<unsafe extern "C-unwind" fn(*mut Walker, *mut Select)>,
    walkerDepth: i32,
    eCode: u16,
    mWFlags: u16,
    u: __SlateRecord193,
}

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
    __slate_bits_0: __slate_bits::__SlateBits161U0,
    azInit: *mut *const i8,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord162 {
    xLegacy: Option<unsafe extern "C-unwind" fn(*mut (), *const i8)>,
    xV2: Option<unsafe extern "C-unwind" fn(u32, *mut (), *mut (), *mut ()) -> i32>,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord163 {
    isInterrupted: i32,
    notUsed1: f64,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord164 {
    pHash: *mut FuncDef,
    pDestructor: *mut FuncDestructor,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord165 {
    tab: __SlateRecord166,
    view: __SlateRecord167,
    vtab: __SlateRecord168,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord166 {
    addColOffset: i32,
    pFKey: *mut FKey,
    pDfltList: *mut ExprList,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord167 {
    pSelect: *mut Select,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord168 {
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
union __SlateRecord173 {
    zToken: *mut i8,
    iValue: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord174 {
    pList: *mut ExprList,
    pSelect: *mut Select,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord175 {
    iJoin: i32,
    iOfst: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord176 {
    pTab: *mut Table,
    pWin: *mut Window,
    nReg: i32,
    sub: __SlateRecord177,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord177 {
    iAddr: i32,
    regReturn: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct ExprList_item {
    pExpr: *mut Expr,
    zEName: *mut i8,
    fg: __SlateRecord179,
    u: __SlateRecord180,
}

#[repr(C, align(4))]
#[derive(Clone, Copy)]
struct __SlateRecord179 {
    sortFlags: u8,
    __slate_bits_0: __slate_bits::__SlateBits179U0,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord180 {
    x: __SlateRecord181,
    iConstExprReg: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord181 {
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
struct __SlateRecord183 {
    jointype: u8,
    __slate_bits_0: __slate_bits::__SlateBits183U0,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord184 {
    zIndexedBy: *mut i8,
    pFuncArg: *mut ExprList,
    nRow: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord185 {
    pIBIndex: *mut Index,
    pCteUse: *mut CteUse,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord186 {
    pOn: *mut Expr,
    pUsing: *mut IdList,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord187 {
    pSchema: *mut Schema,
    zDatabase: *mut i8,
    pSubq: *mut Subquery,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord188 {
    pEList: *mut ExprList,
    pAggInfo: *mut AggInfo,
    pUpsert: *mut Upsert,
    iBaseReg: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord190 {
    cr: __SlateRecord191,
    d: __SlateRecord192,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord191 {
    addrCrTab: i32,
    regRowid: i32,
    regRoot: i32,
    constraintName: Token,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord192 {
    pReturning: *mut Returning,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord193 {
    pNC: *mut NameContext,
    n: i32,
    iCur: i32,
    sz: i32,
    pSrcList: *mut SrcList,
    pCCurHint: *mut CCurHint,
    pRefSrcList: *mut RefSrcList,
    aiCol: *mut i32,
    pIdxCover: *mut IdxCover,
    pGroupBy: *mut ExprList,
    pSelect: *mut Select,
    pRewrite: *mut WindowRewrite,
    pConst: *mut WhereConst,
    pRename: *mut RenameCtx,
    pTab: *mut Table,
    pCovIdxCk: *mut CoveringIndexCheck,
    pSrcItem: *mut SrcItem,
    pFix: *mut DbFixer,
    aMem: *mut sqlite3_value,
    pCheckOnCtx: *mut CheckOnCtx,
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
// ** This file contains code used to implement the ATTACH and DETACH commands.
// */
#[repr(C)]
#[derive(Clone, Copy)]
struct CCurHint {}

#[repr(C)]
#[derive(Clone, Copy)]
struct RefSrcList {}

#[repr(C)]
#[derive(Clone, Copy)]
struct IdxCover {}

#[repr(C)]
#[derive(Clone, Copy)]
struct WindowRewrite {}

#[repr(C)]
#[derive(Clone, Copy)]
struct WhereConst {}

#[repr(C)]
#[derive(Clone, Copy)]
struct RenameCtx {}

#[repr(C)]
#[derive(Clone, Copy)]
struct CoveringIndexCheck {}

#[repr(C)]
#[derive(Clone, Copy)]
struct CheckOnCtx {}

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
    pub struct __SlateBits61U0 {
        #[bits(4)]
        pub notNull: u32,
        #[bits(4)]
        pub eCType: u32,
    }
    #[bitfields::bitfield([u8; 2], c_names = true, new = false, from_into_bits = false, from_traits = false, default = false, debug = false, builder = false, bit_ops = false)]
    pub struct __SlateBits179U0 {
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
    pub struct __SlateBits183U0 {
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
    pub struct __SlateBits87U0 {
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
    pub struct __SlateBits161U0 {
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
}

static mut detach_func: FuncDef = FuncDef {
    nArg: (1 as i32) as i16,
    funcFlags: (1 as i32) as u32,
    pUserData: std::ptr::null_mut::<()>(),
    pNext: std::ptr::null_mut::<FuncDef>(),
    xSFunc: Some(detachFunc),
    xFinalize: None,
    xValue: None,
    xInverse: None,
    zName: (b"sqlite_detach\0".as_ptr() as *mut i8) as *const i8,
    u: {
        let mut __t0: __SlateRecord164 = unsafe { std::mem::zeroed() };
        __t0.pHash = std::ptr::null_mut::<FuncDef>();
        __t0
    },
};

static mut attach_func: FuncDef = FuncDef {
    nArg: (3 as i32) as i16,
    funcFlags: (1 as i32) as u32,
    pUserData: std::ptr::null_mut::<()>(),
    pNext: std::ptr::null_mut::<FuncDef>(),
    xSFunc: Some(attachFunc),
    xFinalize: None,
    xValue: None,
    xInverse: None,
    zName: (b"sqlite_attach\0".as_ptr() as *mut i8) as *const i8,
    u: {
        let mut __t0: __SlateRecord164 = unsafe { std::mem::zeroed() };
        __t0.pHash = std::ptr::null_mut::<FuncDef>();
        __t0
    },
};

// /*
// ** Return true if zName points to a name that may be used to refer to
// ** database iDb attached to handle db.
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3DbIsNamed(
    mut db: *mut sqlite3,
    mut iDb: i32,
    mut zName: *const i8,
) -> i32 {
    let __v601: bool;
    if (unsafe {
        sqlite3StrICmp(
            (unsafe { (*unsafe { unsafe { (*db).aDb }.offset(iDb as isize) }).zDbSName })
                as *const i8,
            zName,
        )
    }) == (0 as i32)
    {
        __v601 = true as bool;
    } else {
        let __v602: bool;
        if iDb == (0 as i32) {
            __v602 =
                (unsafe { sqlite3StrICmp((b"main\0".as_ptr() as *mut i8) as *const i8, zName) })
                    == (0 as i32);
        } else {
            __v602 = false as bool;
        }
        __v601 = __v602;
    }
    return __v601 as i32;
}

// /*
// ** Called by the parser to compile an ATTACH statement.
// **
// **     ATTACH p AS pDbname KEY pKey
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3Attach(
    mut pParse: *mut Parse,
    mut p: *mut Expr,
    mut pDbname: *mut Expr,
    mut pKey: *mut Expr,
) {
    // /* nArg */
    // /* funcFlags */
    // /* pUserData */
    // /* pNext */
    // /* xSFunc */
    // /* xFinalize */
    // /* xValue, xInverse */
    // /* zName */
    codeAttach(
        pParse,
        24 as i32,
        unsafe { std::ptr::addr_of!(attach_func) },
        p,
        p,
        pDbname,
        pKey,
    );
}

// /* The parser context */
// /* Either SQLITE_ATTACH or SQLITE_DETACH */
// /* FuncDef wrapper for detachFunc() or attachFunc() */
// /* Expression to pass to authorization callback */
// /* Name of database file */
// /* Name of the database to use internally */
// /* Database key for encryption extension */
// /*
// ** Called by the parser to compile a DETACH statement.
// **
// **     DETACH pDbname
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3Detach(mut pParse: *mut Parse, mut pDbname: *mut Expr) {
    // /* nArg */
    // /* funcFlags */
    // /* pUserData */
    // /* pNext */
    // /* xSFunc */
    // /* xFinalize */
    // /* xValue, xInverse */
    // /* zName */
    codeAttach(
        pParse,
        25 as i32,
        unsafe { std::ptr::addr_of!(detach_func) },
        pDbname,
        std::ptr::null_mut::<Expr>(),
        std::ptr::null_mut::<Expr>(),
        pDbname,
    );
}

// /*
// ** Initialize a DbFixer structure.  This routine must be called prior
// ** to passing the structure to one of the sqliteFixAAAA() routines below.
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3FixInit(
    mut pFix: *mut DbFixer,
    mut pParse: *mut Parse,
    mut iDb: i32,
    mut zType: *const i8,
    mut pName: *const Token,
) {
    let mut db: *mut sqlite3 = unsafe { (*pParse).db };
    0 as i32;
    unsafe {
        (*pFix).pParse = pParse;
    }
    unsafe {
        (*pFix).zDb = (unsafe { (*unsafe { unsafe { (*db).aDb }.offset(iDb as isize) }).zDbSName })
            as *const i8;
    }
    unsafe {
        (*pFix).pSchema =
            unsafe { (*unsafe { unsafe { (*db).aDb }.offset(iDb as isize) }).pSchema };
    }
    unsafe {
        (*pFix).zType = zType;
    }
    unsafe {
        (*pFix).pName = pName;
    }
    unsafe {
        (*pFix).bTemp = (iDb == (1 as i32)) as u8;
    }
    unsafe {
        (*pFix).w.pParse = pParse;
    }
    unsafe {
        (*pFix).w.xExprCallback = Some(fixExprCb);
    }
    unsafe {
        (*pFix).w.xSelectCallback = Some(fixSelectCb);
    }
    unsafe {
        (*pFix).w.xSelectCallback2 = unsafe {
            std::mem::transmute::<
                *const (),
                Option<unsafe extern "C-unwind" fn(*mut Walker, *mut Select)>,
            >(sqlite3WalkWinDefnDummyCallback as *const ())
        };
    }
    unsafe {
        (*pFix).w.walkerDepth = 0 as i32;
    }
    unsafe {
        (*pFix).w.eCode = ((0 as i32) as i16) as u16;
    }
    unsafe {
        (*pFix).w.u.pFix = pFix;
    }
}

// /* The fixer to be initialized */
// /* Error messages will be written here */
// /* This is the database that must be used */
// /* "view", "trigger", or "index" */
// /* Name of the view, trigger, or index */
// /*
// ** The following set of routines walk through the parse tree and assign
// ** a specific database to all table references where the database name
// ** was left unspecified in the original SQL statement.  The pFix structure
// ** must have been initialized by a prior call to sqlite3FixInit().
// **
// ** These routines are used to make sure that an index, trigger, or
// ** view in one database does not refer to objects in a different database.
// ** (Exception: indices, triggers, and views in the TEMP database are
// ** allowed to refer to anything.)  If a reference is explicitly made
// ** to an object in a different database, an error message is added to
// ** pParse->zErrMsg and these routines return non-zero.  If everything
// ** checks out, these routines return 0.
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3FixSrcList(mut pFix: *mut DbFixer, mut pList: *mut SrcList) -> i32 {
    let mut res: i32 = 0 as i32;
    if pList != std::ptr::null_mut::<SrcList>() {
        let mut s: Select = unsafe { std::mem::zeroed() };
        unsafe { memset(std::ptr::addr_of_mut!(s) as *mut (), 0 as i32, 120 as u64) };
        s.pSrc = pList;
        res = unsafe {
            sqlite3WalkSelect(
                unsafe { std::ptr::addr_of_mut!((*pFix).w) },
                std::ptr::addr_of_mut!(s),
            )
        };
    }
    return res;
}

// /* Context of the fixation */
// /* The Source list to check and modify */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3FixSelect(mut pFix: *mut DbFixer, mut pSelect: *mut Select) -> i32 {
    return unsafe { sqlite3WalkSelect(unsafe { std::ptr::addr_of_mut!((*pFix).w) }, pSelect) };
}

// /* Context of the fixation */
// /* The SELECT statement to be fixed to one database */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3FixExpr(mut pFix: *mut DbFixer, mut pExpr: *mut Expr) -> i32 {
    return unsafe { sqlite3WalkExpr(unsafe { std::ptr::addr_of_mut!((*pFix).w) }, pExpr) };
}

// /* Context of the fixation */
// /* The expression to be fixed to one database */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3FixTriggerStep(
    mut pFix: *mut DbFixer,
    mut pStep: *mut TriggerStep,
) -> i32 {
    '__slate_break_599: while pStep != std::ptr::null_mut::<TriggerStep>() {
        let __v603: bool;
        if (unsafe {
            sqlite3WalkSelect(unsafe { std::ptr::addr_of_mut!((*pFix).w) }, unsafe {
                (*pStep).pSelect
            })
        }) != (0 as i32)
        {
            __v603 = true as bool;
        } else {
            __v603 = (unsafe {
                sqlite3WalkExpr(unsafe { std::ptr::addr_of_mut!((*pFix).w) }, unsafe {
                    (*pStep).pWhere
                })
            }) != (0 as i32);
        }
        let __v604: bool;
        if __v603 {
            __v604 = true as bool;
        } else {
            __v604 = (unsafe {
                sqlite3WalkExprList(unsafe { std::ptr::addr_of_mut!((*pFix).w) }, unsafe {
                    (*pStep).pExprList
                })
            }) != (0 as i32);
        }
        let __v605: bool;
        if __v604 {
            __v605 = true as bool;
        } else {
            __v605 = sqlite3FixSrcList(pFix, unsafe { (*pStep).pSrc }) != (0 as i32);
        }
        if __v605 {
            return 1 as i32;
        }
        let mut pUp: *mut Upsert = unsafe { std::mem::zeroed() };
        pUp = unsafe { (*pStep).pUpsert };
        '__slate_break_600: while pUp != std::ptr::null_mut::<Upsert>() {
            let __v606: bool;
            if (unsafe {
                sqlite3WalkExprList(unsafe { std::ptr::addr_of_mut!((*pFix).w) }, unsafe {
                    (*pUp).pUpsertTarget
                })
            }) != (0 as i32)
            {
                __v606 = true as bool;
            } else {
                __v606 = (unsafe {
                    sqlite3WalkExpr(unsafe { std::ptr::addr_of_mut!((*pFix).w) }, unsafe {
                        (*pUp).pUpsertTargetWhere
                    })
                }) != (0 as i32);
            }
            let __v607: bool;
            if __v606 {
                __v607 = true as bool;
            } else {
                __v607 = (unsafe {
                    sqlite3WalkExprList(unsafe { std::ptr::addr_of_mut!((*pFix).w) }, unsafe {
                        (*pUp).pUpsertSet
                    })
                }) != (0 as i32);
            }
            let __v608: bool;
            if __v607 {
                __v608 = true as bool;
            } else {
                __v608 = (unsafe {
                    sqlite3WalkExpr(unsafe { std::ptr::addr_of_mut!((*pFix).w) }, unsafe {
                        (*pUp).pUpsertWhere
                    })
                }) != (0 as i32);
            }
            if __v608 {
                return 1 as i32;
            }
            pUp = unsafe { (*pUp).pNextUpsert };
        }
        pStep = unsafe { (*pStep).pNext };
    }
    return 0 as i32;
}

// /*
// ** Resolve an expression that was part of an ATTACH or DETACH statement. This
// ** is slightly different from resolving a normal SQL expression, because simple
// ** identifiers are treated as strings, not possible column names or aliases.
// **
// ** i.e. if the parser sees:
// **
// **     ATTACH DATABASE abc AS def
// **
// ** it treats the two expressions as literal strings 'abc' and 'def' instead of
// ** looking for columns of the same name.
// **
// ** This only applies to the root node of pExpr, so the statement:
// **
// **     ATTACH DATABASE abc||def AS 'db2'
// **
// ** will fail because neither abc or def can be resolved.
// */
// /* Context of the fixation */
// /* The trigger step be fixed to one database */
fn resolveAttachExpr(mut pName: *mut NameContext, mut pExpr: *mut Expr) -> i32 {
    let mut rc: i32 = 0 as i32;
    if pExpr != std::ptr::null_mut::<Expr>() {
        if (((unsafe { (*pExpr).op }) as u32) as i32) != (60 as i32) {
            rc = unsafe { sqlite3ResolveExprNames(pName, pExpr) };
        } else {
            unsafe {
                (*pExpr).op = ((118 as i32) as i8) as u8;
            }
        }
    }
    return rc;
}

// /*
// ** An SQL user-function registered to do the work of an ATTACH statement. The
// ** three arguments to the function come directly from an attach statement:
// **
// **     ATTACH DATABASE x AS y KEY z
// **
// **     SELECT sqlite_attach(x, y, z)
// **
// ** If the optional "KEY z" syntax is omitted, an SQL NULL is passed as the
// ** third argument.
// **
// ** If the db->init.reopenMemdb flags is set, then instead of attaching a
// ** new database, close the database on db->init.iDb and reopen it as an
// ** empty MemDB.
// */
#[unsafe(link_section = ".text.slate_distinct.attach.attachFunc")]
extern "C-unwind" fn attachFunc(
    mut context: *mut sqlite3_context,
    mut NotUsed: i32,
    mut argv: *mut *mut sqlite3_value,
) {
    let mut __slate_storage_384: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_384: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_384) as *mut i32;
    let mut __slate_storage_625: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_625: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_625) as *mut u32;
    let mut __slate_storage_624: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_624: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_624) as *mut u32;
    let mut __slate_storage_623: std::mem::MaybeUninit<*mut sqlite3> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_623: *mut *mut sqlite3 =
        std::ptr::addr_of_mut!(__slate_storage_623) as *mut *mut sqlite3;
    let mut __slate_storage_383: std::mem::MaybeUninit<*mut Pager> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_383: *mut *mut Pager =
        std::ptr::addr_of_mut!(__slate_storage_383) as *mut *mut Pager;
    let mut __slate_storage_382: std::mem::MaybeUninit<*mut Schema> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_382: *mut *mut Schema =
        std::ptr::addr_of_mut!(__slate_storage_382) as *mut *mut Schema;
    let mut __slate_storage_609: std::mem::MaybeUninit<bool> = std::mem::MaybeUninit::uninit();
    let __slate_slot_609: *mut bool = std::ptr::addr_of_mut!(__slate_storage_609) as *mut bool;
    let mut __slate_storage_381: std::mem::MaybeUninit<*mut Btree> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_381: *mut *mut Btree =
        std::ptr::addr_of_mut!(__slate_storage_381) as *mut *mut Btree;
    let mut __slate_storage_622: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_622: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_622) as *mut i32;
    let mut __slate_storage_621: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_621: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_621) as *mut i32;
    let mut __slate_storage_620: std::mem::MaybeUninit<*mut sqlite3> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_620: *mut *mut sqlite3 =
        std::ptr::addr_of_mut!(__slate_storage_620) as *mut *mut sqlite3;
    let mut __slate_storage_619: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_619: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_619) as *mut u32;
    let mut __slate_storage_618: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_618: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_618) as *mut u32;
    let mut __slate_storage_615: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_615: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_615) as *mut u32;
    let mut __slate_storage_614: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_614: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_614) as *mut u32;
    let mut __slate_storage_613: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_613: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_613) as *mut u32;
    let mut __slate_storage_612: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_612: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_612) as *mut u32;
    let mut __slate_storage_617: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_617: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_617) as *mut u32;
    let mut __slate_storage_616: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_616: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_616) as *mut u32;
    let mut __slate_storage_611: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_611: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_611) as *mut i32;
    let mut __slate_storage_610: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_610: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_610) as *mut i32;
    let mut __slate_storage_380: std::mem::MaybeUninit<*mut sqlite3_vfs> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_380: *mut *mut sqlite3_vfs =
        std::ptr::addr_of_mut!(__slate_storage_380) as *mut *mut sqlite3_vfs;
    let mut __slate_storage_379: std::mem::MaybeUninit<*mut i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_379: *mut *mut i8 =
        std::ptr::addr_of_mut!(__slate_storage_379) as *mut *mut i8;
    let mut __slate_storage_378: std::mem::MaybeUninit<*mut Db> = std::mem::MaybeUninit::uninit();
    let __slate_slot_378: *mut *mut Db =
        std::ptr::addr_of_mut!(__slate_storage_378) as *mut *mut Db;
    let mut __slate_storage_377: std::mem::MaybeUninit<*mut Db> = std::mem::MaybeUninit::uninit();
    let __slate_slot_377: *mut *mut Db =
        std::ptr::addr_of_mut!(__slate_storage_377) as *mut *mut Db;
    let mut __slate_storage_376: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_376: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_376) as *mut u32;
    let mut __slate_storage_375: std::mem::MaybeUninit<*mut i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_375: *mut *mut i8 =
        std::ptr::addr_of_mut!(__slate_storage_375) as *mut *mut i8;
    let mut __slate_storage_374: std::mem::MaybeUninit<*mut i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_374: *mut *mut i8 =
        std::ptr::addr_of_mut!(__slate_storage_374) as *mut *mut i8;
    let mut __slate_storage_373: std::mem::MaybeUninit<*const i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_373: *mut *const i8 =
        std::ptr::addr_of_mut!(__slate_storage_373) as *mut *const i8;
    let mut __slate_storage_372: std::mem::MaybeUninit<*const i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_372: *mut *const i8 =
        std::ptr::addr_of_mut!(__slate_storage_372) as *mut *const i8;
    let mut __slate_storage_371: std::mem::MaybeUninit<*mut sqlite3> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_371: *mut *mut sqlite3 =
        std::ptr::addr_of_mut!(__slate_storage_371) as *mut *mut sqlite3;
    let mut __slate_storage_370: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_370: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_370) as *mut i32;
    let mut __slate_storage_369: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_369: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_369) as *mut i32;
    unsafe {
        '__join_64: {
            std::ptr::write(__slate_slot_370, 0 as i32);
            std::ptr::write(__slate_slot_371, unsafe {
                sqlite3_context_db_handle(context)
            });
            std::ptr::write(__slate_slot_374, std::ptr::null_mut::<i8>());
            std::ptr::write(__slate_slot_375, std::ptr::null_mut::<i8>());
            // /* New array of Db pointers */
            // /* Db object for the newly attached database */
            std::ptr::write(__slate_slot_378, std::ptr::null_mut::<Db>());
            std::ptr::write(__slate_slot_379, std::ptr::null_mut::<i8>());
            NotUsed;
            *__slate_slot_373 = (unsafe {
                sqlite3_value_text(unsafe { *unsafe { argv.offset((0 as i32) as isize) } })
            }) as *const i8;
            *__slate_slot_372 = (unsafe {
                sqlite3_value_text(unsafe { *unsafe { argv.offset((1 as i32) as isize) } })
            }) as *const i8;
            if *__slate_slot_373 == std::ptr::null::<i8>() {
                *__slate_slot_373 = (b"\0".as_ptr() as *mut i8) as *const i8;
            }
        }
        if *__slate_slot_372 == std::ptr::null::<i8>() {
            *__slate_slot_372 = (b"\0".as_ptr() as *mut i8) as *const i8;
        }
        '__join_4: {
            '__join_27: {
                if ((unsafe {
                    (*(*__slate_slot_371))
                        .init
                        .__slate_bits_0
                        .__get_reopenMemdb()
                }) as i32)
                    != (0 as i32)
                {
                    // /* This is not a real ATTACH.  Instead, this routine is being called
                    //     ** from sqlite3_deserialize() to close database db->init.iDb and
                    //     ** reopen it as a MemDB */
                    std::ptr::write(__slate_slot_381, std::ptr::null_mut::<Btree>());
                    *__slate_slot_378 = unsafe {
                        unsafe { (*(*__slate_slot_371)).aDb }.offset(
                            (((unsafe { (*(*__slate_slot_371)).init.iDb }) as u32) as i32) as isize,
                        )
                    };
                    0 as i32;
                    if (unsafe { sqlite3BtreeTxnState(unsafe { (*(*__slate_slot_378)).pBt }) })
                        != (0 as i32)
                    {
                        *__slate_slot_609 = true as bool;
                    } else {
                        *__slate_slot_609 = (unsafe {
                            sqlite3BtreeIsInBackup(unsafe { (*(*__slate_slot_378)).pBt })
                        }) != (0 as i32);
                    }
                    if *__slate_slot_609 {
                        *__slate_slot_370 = 5 as i32;
                        break '__join_4;
                    } else {
                        *__slate_slot_380 = unsafe {
                            sqlite3_vfs_find((b"memdb\0".as_ptr() as *mut i8) as *const i8)
                        };
                        if *__slate_slot_380 == std::ptr::null_mut::<sqlite3_vfs>() {
                            return;
                        } else {
                            *__slate_slot_370 = unsafe {
                                sqlite3BtreeOpen(
                                    *__slate_slot_380,
                                    (b"x\0\0".as_ptr() as *mut i8) as *const i8,
                                    *__slate_slot_371,
                                    std::ptr::addr_of_mut!(*__slate_slot_381),
                                    0 as i32,
                                    256 as i32,
                                )
                            };
                            if *__slate_slot_370 == (0 as i32) {
                                std::ptr::write(__slate_slot_382, unsafe {
                                    sqlite3SchemaGet(*__slate_slot_371, *__slate_slot_381)
                                });
                                if *__slate_slot_382 != std::ptr::null_mut::<Schema>() {
                                    // /* Both the Btree and the new Schema were allocated successfully.
                                    //         ** Close the old db and update the aDb[] slot with the new memdb
                                    //         ** values.  */
                                    unsafe {
                                        sqlite3BtreeClose(unsafe { (*(*__slate_slot_378)).pBt })
                                    };
                                    unsafe {
                                        (*(*__slate_slot_378)).pBt = *__slate_slot_381;
                                    }
                                    unsafe {
                                        (*(*__slate_slot_378)).pSchema = *__slate_slot_382;
                                    }
                                } else {
                                    unsafe { sqlite3BtreeClose(*__slate_slot_381) };
                                    *__slate_slot_370 = 7 as i32;
                                }
                            }
                            if *__slate_slot_370 != (0 as i32) {
                                break '__join_4;
                            }
                        }
                    }
                } else {
                    // /* This is a real ATTACH
                    //     **
                    //     ** Check for the following errors:
                    //     **
                    //     **     * Too many attached databases,
                    //     **     * Transaction currently open
                    //     **     * Specified database name already being used.
                    //     */
                    if (unsafe { (*(*__slate_slot_371)).nDb })
                        >= (unsafe {
                            *unsafe {
                                unsafe { (*(*__slate_slot_371)).aLimit.as_mut_ptr() as *mut i32 }
                                    .offset((7 as i32) as isize)
                            }
                        }) + (2 as i32)
                    {
                        *__slate_slot_379 = unsafe {
                            sqlite3MPrintf(
                                *__slate_slot_371,
                                (b"too many attached databases - max %d\0".as_ptr() as *mut i8)
                                    as *const i8,
                                unsafe {
                                    *unsafe {
                                        unsafe {
                                            (*(*__slate_slot_371)).aLimit.as_mut_ptr() as *mut i32
                                        }
                                        .offset((7 as i32) as isize)
                                    }
                                },
                            )
                        };
                        break '__join_4;
                    } else {
                        *__slate_slot_369 = 0 as i32;
                        '__join_57: {
                            loop {
                                if *__slate_slot_369 < unsafe { (*(*__slate_slot_371)).nDb } {
                                    0 as i32;
                                    if sqlite3DbIsNamed(
                                        *__slate_slot_371,
                                        *__slate_slot_369,
                                        *__slate_slot_372,
                                    ) != (0 as i32)
                                    {
                                        break '__join_57;
                                    } else {
                                        std::ptr::write(__slate_slot_610, *__slate_slot_369);
                                        std::ptr::write(
                                            __slate_slot_611,
                                            *__slate_slot_610 + (1 as i32),
                                        );
                                        *__slate_slot_369 = *__slate_slot_611;
                                    }
                                } else {
                                    break;
                                }
                            }
                            // /* Allocate the new entry in the db->aDb[] array and initialize the schema
                            //     ** hash tables.
                            //     */
                            if (unsafe { (*(*__slate_slot_371)).aDb })
                                == unsafe {
                                    (*(*__slate_slot_371)).aDbStatic.as_mut_ptr() as *mut Db
                                }
                            {
                                *__slate_slot_377 = (unsafe {
                                    sqlite3DbMallocRawNN(
                                        *__slate_slot_371,
                                        (32 as u64).wrapping_mul(((3 as i32) as i64) as u64),
                                    )
                                }) as *mut Db;
                                if *__slate_slot_377 == std::ptr::null_mut::<Db>() {
                                    return;
                                } else {
                                    unsafe {
                                        memcpy(
                                            *__slate_slot_377 as *mut (),
                                            (unsafe { (*(*__slate_slot_371)).aDb }) as *const (),
                                            (32 as u64).wrapping_mul(((2 as i32) as i64) as u64),
                                        )
                                    };
                                }
                            } else {
                                *__slate_slot_377 = (unsafe {
                                    sqlite3DbRealloc(
                                        *__slate_slot_371,
                                        (unsafe { (*(*__slate_slot_371)).aDb }) as *mut (),
                                        (32 as u64).wrapping_mul(
                                            (((1 as i32) as i64)
                                                + ((unsafe { (*(*__slate_slot_371)).nDb }) as i64))
                                                as u64,
                                        ),
                                    )
                                }) as *mut Db;
                                if *__slate_slot_377 == std::ptr::null_mut::<Db>() {
                                    return;
                                }
                            }
                            unsafe {
                                (*(*__slate_slot_371)).aDb = *__slate_slot_377;
                            }
                            *__slate_slot_378 = unsafe {
                                unsafe { (*(*__slate_slot_371)).aDb }
                                    .offset((unsafe { (*(*__slate_slot_371)).nDb }) as isize)
                            };
                            unsafe { memset(*__slate_slot_378 as *mut (), 0 as i32, 32 as u64) };
                            // /* Open the database file. If the btree is successfully opened, use
                            //     ** it to obtain the database schema. At this point the schema may
                            //     ** or may not be initialized.
                            //     */
                            *__slate_slot_376 = unsafe { (*(*__slate_slot_371)).openFlags };
                            *__slate_slot_370 = unsafe {
                                sqlite3ParseUri(
                                    unsafe { (*unsafe { (*(*__slate_slot_371)).pVfs }).zName },
                                    *__slate_slot_373,
                                    std::ptr::addr_of_mut!(*__slate_slot_376),
                                    std::ptr::addr_of_mut!(*__slate_slot_380),
                                    std::ptr::addr_of_mut!(*__slate_slot_374),
                                    std::ptr::addr_of_mut!(*__slate_slot_375),
                                )
                            };
                            if *__slate_slot_370 != (0 as i32) {
                                if *__slate_slot_370 == (7 as i32) {
                                    unsafe { sqlite3OomFault(*__slate_slot_371) };
                                }
                                unsafe {
                                    sqlite3_result_error(
                                        context,
                                        *__slate_slot_375 as *const i8,
                                        -(1 as i32),
                                    )
                                };
                                unsafe { sqlite3_free(*__slate_slot_375 as *mut ()) };
                                return;
                            } else {
                                if (unsafe { (*(*__slate_slot_371)).flags })
                                    & (((32 as i32) as i64) as u64) << (32 as i32)
                                    == (((0 as i32) as i64) as u64)
                                {
                                    std::ptr::write(__slate_slot_612, *__slate_slot_376);
                                    std::ptr::write(
                                        __slate_slot_613,
                                        *__slate_slot_612 & (!((4 as i32) | (2 as i32)) as u32),
                                    );
                                    *__slate_slot_376 = *__slate_slot_613;
                                    std::ptr::write(__slate_slot_614, *__slate_slot_376);
                                    std::ptr::write(
                                        __slate_slot_615,
                                        *__slate_slot_614 | ((1 as i32) as u32),
                                    );
                                    *__slate_slot_376 = *__slate_slot_615;
                                } else {
                                    if (unsafe { (*(*__slate_slot_371)).flags })
                                        & (((16 as i32) as i64) as u64) << (32 as i32)
                                        == (((0 as i32) as i64) as u64)
                                    {
                                        std::ptr::write(__slate_slot_616, *__slate_slot_376);
                                        std::ptr::write(
                                            __slate_slot_617,
                                            *__slate_slot_616 & (!(4 as i32) as u32),
                                        );
                                        *__slate_slot_376 = *__slate_slot_617;
                                    }
                                }
                                0 as i32;
                                std::ptr::write(__slate_slot_618, *__slate_slot_376);
                                std::ptr::write(
                                    __slate_slot_619,
                                    *__slate_slot_618 | ((256 as i32) as u32),
                                );
                                *__slate_slot_376 = *__slate_slot_619;
                                *__slate_slot_370 = unsafe {
                                    sqlite3BtreeOpen(
                                        *__slate_slot_380,
                                        *__slate_slot_374 as *const i8,
                                        *__slate_slot_371,
                                        unsafe {
                                            std::ptr::addr_of_mut!((*(*__slate_slot_378)).pBt)
                                        },
                                        0 as i32,
                                        *__slate_slot_376 as i32,
                                    )
                                };
                                std::ptr::write(__slate_slot_620, *__slate_slot_371);
                                std::ptr::write(__slate_slot_621, unsafe {
                                    (*(*__slate_slot_620)).nDb
                                });
                                std::ptr::write(__slate_slot_622, *__slate_slot_621 + (1 as i32));
                                unsafe {
                                    (*(*__slate_slot_620)).nDb = *__slate_slot_622;
                                }
                                unsafe {
                                    (*(*__slate_slot_378)).zDbSName = unsafe {
                                        sqlite3DbStrDup(*__slate_slot_371, *__slate_slot_372)
                                    };
                                }
                                break '__join_27;
                            }
                        }
                        *__slate_slot_379 = unsafe {
                            sqlite3MPrintf(
                                *__slate_slot_371,
                                (b"database %s is already in use\0".as_ptr() as *mut i8)
                                    as *const i8,
                                *__slate_slot_372,
                            )
                        };
                        break '__join_4;
                    }
                }
            }
            unsafe {
                (*(*__slate_slot_371)).noSharedCache = ((0 as i32) as i8) as u8;
            }
            if *__slate_slot_370 == (19 as i32) {
                *__slate_slot_370 = 1 as i32;
                *__slate_slot_379 = unsafe {
                    sqlite3MPrintf(
                        *__slate_slot_371,
                        (b"database is already attached\0".as_ptr() as *mut i8) as *const i8,
                    )
                };
            } else {
                if *__slate_slot_370 == (0 as i32) {
                    unsafe {
                        (*(*__slate_slot_378)).pSchema = unsafe {
                            sqlite3SchemaGet(*__slate_slot_371, unsafe {
                                (*(*__slate_slot_378)).pBt
                            })
                        };
                    }
                    if !((unsafe { (*(*__slate_slot_378)).pSchema })
                        != std::ptr::null_mut::<Schema>())
                    {
                        *__slate_slot_370 = 7 as i32;
                    } else {
                        if (unsafe { (*unsafe { (*(*__slate_slot_378)).pSchema }).file_format })
                            != (0 as u8)
                            && (((unsafe { (*unsafe { (*(*__slate_slot_378)).pSchema }).enc })
                                as u32) as i32)
                                != (((unsafe { (*(*__slate_slot_371)).enc }) as u32) as i32)
                        {
                            *__slate_slot_379 = unsafe {
                                sqlite3MPrintf(*__slate_slot_371, (b"attached databases must use the same text encoding as main database\0".as_ptr() as *mut i8) as *const i8)
                            };
                            *__slate_slot_370 = 1 as i32;
                        }
                    }
                    unsafe { sqlite3BtreeEnter(unsafe { (*(*__slate_slot_378)).pBt }) };
                    *__slate_slot_383 =
                        unsafe { sqlite3BtreePager(unsafe { (*(*__slate_slot_378)).pBt }) };
                    unsafe {
                        sqlite3PagerLockingMode(
                            *__slate_slot_383,
                            ((unsafe { (*(*__slate_slot_371)).dfltLockMode }) as u32) as i32,
                        )
                    };
                    unsafe {
                        sqlite3BtreeSecureDelete(unsafe { (*(*__slate_slot_378)).pBt }, unsafe {
                            sqlite3BtreeSecureDelete(
                                unsafe {
                                    (*unsafe {
                                        unsafe { (*(*__slate_slot_371)).aDb }
                                            .offset((0 as i32) as isize)
                                    })
                                    .pBt
                                },
                                -(1 as i32),
                            )
                        })
                    };
                    unsafe {
                        sqlite3BtreeSetPagerFlags(
                            unsafe { (*(*__slate_slot_378)).pBt },
                            ((((3 as i32) as i64) as u64)
                                | (unsafe { (*(*__slate_slot_371)).flags })
                                    & (((56 as i32) as i64) as u64))
                                as u32,
                        )
                    };
                    unsafe { sqlite3BtreeLeave(unsafe { (*(*__slate_slot_378)).pBt }) };
                }
            }
            unsafe {
                (*(*__slate_slot_378)).safety_level = (((2 as i32) + (1 as i32)) as i8) as u8;
            }
            if *__slate_slot_370 == (0 as i32)
                && (unsafe { (*(*__slate_slot_378)).zDbSName }) == std::ptr::null_mut::<i8>()
            {
                *__slate_slot_370 = 7 as i32;
            }
            unsafe { sqlite3_free_filename(*__slate_slot_374 as *const i8) };
            // /* If the file was opened successfully, read the schema for the new database.
            //   ** If this fails, or if opening the file failed, then close the file and
            //   ** remove the entry from the db->aDb[] array. i.e. put everything back the
            //   ** way we found it.
            //   */
            if *__slate_slot_370 == (0 as i32) {
                unsafe { sqlite3BtreeEnterAll(*__slate_slot_371) };
                unsafe {
                    (*(*__slate_slot_371)).init.iDb = ((0 as i32) as i8) as u8;
                }
                std::ptr::write(__slate_slot_623, *__slate_slot_371);
                std::ptr::write(__slate_slot_624, unsafe { (*(*__slate_slot_623)).mDbFlags });
                std::ptr::write(__slate_slot_625, *__slate_slot_624 & (!(16 as i32) as u32));
                unsafe {
                    (*(*__slate_slot_623)).mDbFlags = *__slate_slot_625;
                }
                if !(((unsafe {
                    (*(*__slate_slot_371))
                        .init
                        .__slate_bits_0
                        .__get_reopenMemdb()
                }) as i32)
                    != (0 as i32))
                {
                    *__slate_slot_370 = unsafe {
                        sqlite3Init(*__slate_slot_371, std::ptr::addr_of_mut!(*__slate_slot_379))
                    };
                }
                unsafe { sqlite3BtreeLeaveAll(*__slate_slot_371) };
                0 as i32;
            }
            if *__slate_slot_370 != (0 as i32) {
                if !(((unsafe {
                    (*(*__slate_slot_371))
                        .init
                        .__slate_bits_0
                        .__get_reopenMemdb()
                }) as i32)
                    != (0 as i32))
                {
                    std::ptr::write(
                        __slate_slot_384,
                        (unsafe { (*(*__slate_slot_371)).nDb }) - (1 as i32),
                    );
                    0 as i32;
                    if (unsafe {
                        (*unsafe {
                            unsafe { (*(*__slate_slot_371)).aDb }.offset(*__slate_slot_384 as isize)
                        })
                        .pBt
                    }) != std::ptr::null_mut::<Btree>()
                    {
                        unsafe {
                            sqlite3BtreeClose(unsafe {
                                (*unsafe {
                                    unsafe { (*(*__slate_slot_371)).aDb }
                                        .offset(*__slate_slot_384 as isize)
                                })
                                .pBt
                            })
                        };
                        unsafe {
                            (*unsafe {
                                unsafe { (*(*__slate_slot_371)).aDb }
                                    .offset(*__slate_slot_384 as isize)
                            })
                            .pBt = std::ptr::null_mut::<Btree>();
                        }
                        unsafe {
                            (*unsafe {
                                unsafe { (*(*__slate_slot_371)).aDb }
                                    .offset(*__slate_slot_384 as isize)
                            })
                            .pSchema = std::ptr::null_mut::<Schema>();
                        }
                    }
                    unsafe { sqlite3ResetAllSchemasOfConnection(*__slate_slot_371) };
                    unsafe {
                        (*(*__slate_slot_371)).nDb = *__slate_slot_384;
                    }
                    if *__slate_slot_370 == (7 as i32)
                        || *__slate_slot_370 == (10 as i32) | (12 as i32) << (8 as i32)
                    {
                        unsafe { sqlite3OomFault(*__slate_slot_371) };
                        unsafe { sqlite3DbFree(*__slate_slot_371, *__slate_slot_379 as *mut ()) };
                        *__slate_slot_379 = unsafe {
                            sqlite3MPrintf(
                                *__slate_slot_371,
                                (b"out of memory\0".as_ptr() as *mut i8) as *const i8,
                            )
                        };
                    } else {
                        if *__slate_slot_379 == std::ptr::null_mut::<i8>() {
                            *__slate_slot_379 = unsafe {
                                sqlite3MPrintf(
                                    *__slate_slot_371,
                                    (b"unable to open database: %s\0".as_ptr() as *mut i8)
                                        as *const i8,
                                    *__slate_slot_373,
                                )
                            };
                        }
                    }
                }
            } else {
                return;
            }
        }
        if *__slate_slot_379 != std::ptr::null_mut::<i8>() {
            unsafe { sqlite3_result_error(context, *__slate_slot_379 as *const i8, -(1 as i32)) };
            unsafe { sqlite3DbFree(*__slate_slot_371, *__slate_slot_379 as *mut ()) };
        }
        // /* Return an error if we get here */
        if *__slate_slot_370 != (0 as i32) {
            unsafe { sqlite3_result_error_code(context, *__slate_slot_370) };
        }
    }
}

// /*
// ** An SQL user-function registered to do the work of an DETACH statement. The
// ** three arguments to the function come directly from a detach statement:
// **
// **     DETACH DATABASE x
// **
// **     SELECT sqlite_detach(x)
// */
#[unsafe(link_section = ".text.slate_distinct.attach.detachFunc")]
extern "C-unwind" fn detachFunc(
    mut context: *mut sqlite3_context,
    mut NotUsed: i32,
    mut argv: *mut *mut sqlite3_value,
) {
    let mut __slate_storage_396: std::mem::MaybeUninit<*mut Trigger> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_396: *mut *mut Trigger =
        std::ptr::addr_of_mut!(__slate_storage_396) as *mut *mut Trigger;
    let mut __slate_storage_628: std::mem::MaybeUninit<bool> = std::mem::MaybeUninit::uninit();
    let __slate_slot_628: *mut bool = std::ptr::addr_of_mut!(__slate_storage_628) as *mut bool;
    let mut __slate_storage_627: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_627: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_627) as *mut i32;
    let mut __slate_storage_626: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_626: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_626) as *mut i32;
    let mut __slate_storage_395: std::mem::MaybeUninit<__SlateAlign16<[i8; 128]>> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_395: *mut [i8; 128] =
        std::ptr::addr_of_mut!(__slate_storage_395) as *mut [i8; 128];
    let mut __slate_storage_394: std::mem::MaybeUninit<*mut HashElem> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_394: *mut *mut HashElem =
        std::ptr::addr_of_mut!(__slate_storage_394) as *mut *mut HashElem;
    let mut __slate_storage_393: std::mem::MaybeUninit<*mut Db> = std::mem::MaybeUninit::uninit();
    let __slate_slot_393: *mut *mut Db =
        std::ptr::addr_of_mut!(__slate_storage_393) as *mut *mut Db;
    let mut __slate_storage_392: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_392: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_392) as *mut i32;
    let mut __slate_storage_391: std::mem::MaybeUninit<*mut sqlite3> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_391: *mut *mut sqlite3 =
        std::ptr::addr_of_mut!(__slate_storage_391) as *mut *mut sqlite3;
    let mut __slate_storage_390: std::mem::MaybeUninit<*const i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_390: *mut *const i8 =
        std::ptr::addr_of_mut!(__slate_storage_390) as *mut *const i8;
    unsafe {
        '__join_20: {
            std::ptr::write(
                __slate_slot_390,
                (unsafe {
                    sqlite3_value_text(unsafe { *unsafe { argv.offset((0 as i32) as isize) } })
                }) as *const i8,
            );
            std::ptr::write(__slate_slot_391, unsafe {
                sqlite3_context_db_handle(context)
            });
            std::ptr::write(__slate_slot_393, std::ptr::null_mut::<Db>());
            NotUsed;
            if *__slate_slot_390 == std::ptr::null::<i8>() {
                *__slate_slot_390 = (b"\0".as_ptr() as *mut i8) as *const i8;
            }
        }
        *__slate_slot_392 = 0 as i32;
        '__loop_16: loop {
            if *__slate_slot_392 < unsafe { (*(*__slate_slot_391)).nDb } {
                *__slate_slot_393 = unsafe {
                    unsafe { (*(*__slate_slot_391)).aDb }.offset(*__slate_slot_392 as isize)
                };
                if (unsafe { (*(*__slate_slot_393)).pBt }) == std::ptr::null_mut::<Btree>() {
                } else {
                    if sqlite3DbIsNamed(*__slate_slot_391, *__slate_slot_392, *__slate_slot_390)
                        != (0 as i32)
                    {
                        break '__loop_16;
                    }
                }
                std::ptr::write(__slate_slot_626, *__slate_slot_392);
                std::ptr::write(__slate_slot_627, *__slate_slot_626 + (1 as i32));
                *__slate_slot_392 = *__slate_slot_627;
            } else {
                break;
            }
        }
        '__join_0: {
            if *__slate_slot_392 >= unsafe { (*(*__slate_slot_391)).nDb } {
                unsafe {
                    sqlite3_snprintf(
                        ((128 as u64) as u32) as i32,
                        (*__slate_slot_395).as_mut_ptr() as *mut i8,
                        (b"no such database: %s\0".as_ptr() as *mut i8) as *const i8,
                        *__slate_slot_390,
                    )
                };
            } else {
                if *__slate_slot_392 < (2 as i32) {
                    unsafe {
                        sqlite3_snprintf(
                            ((128 as u64) as u32) as i32,
                            (*__slate_slot_395).as_mut_ptr() as *mut i8,
                            (b"cannot detach database %s\0".as_ptr() as *mut i8) as *const i8,
                            *__slate_slot_390,
                        )
                    };
                } else {
                    if (unsafe { sqlite3BtreeTxnState(unsafe { (*(*__slate_slot_393)).pBt }) })
                        != (0 as i32)
                    {
                        *__slate_slot_628 = true as bool;
                    } else {
                        *__slate_slot_628 = (unsafe {
                            sqlite3BtreeIsInBackup(unsafe { (*(*__slate_slot_393)).pBt })
                        }) != (0 as i32);
                    }
                    if *__slate_slot_628 {
                        unsafe {
                            sqlite3_snprintf(
                                ((128 as u64) as u32) as i32,
                                (*__slate_slot_395).as_mut_ptr() as *mut i8,
                                (b"database %s is locked\0".as_ptr() as *mut i8) as *const i8,
                                *__slate_slot_390,
                            )
                        };
                    } else {
                        // /* If any TEMP triggers reference the schema being detached, move those
                        //   ** triggers to reference the TEMP schema itself. */
                        0 as i32;
                        *__slate_slot_394 = unsafe {
                            (*unsafe {
                                std::ptr::addr_of_mut!(
                                    (*unsafe {
                                        (*unsafe {
                                            unsafe { (*(*__slate_slot_391)).aDb }
                                                .offset((1 as i32) as isize)
                                        })
                                        .pSchema
                                    })
                                    .trigHash
                                )
                            })
                            .first
                        };
                        loop {
                            if *__slate_slot_394 != std::ptr::null_mut::<HashElem>() {
                                std::ptr::write(
                                    __slate_slot_396,
                                    (unsafe { (*(*__slate_slot_394)).data }) as *mut Trigger,
                                );
                                if (unsafe { (*(*__slate_slot_396)).pTabSchema })
                                    == unsafe { (*(*__slate_slot_393)).pSchema }
                                {
                                    unsafe {
                                        (*(*__slate_slot_396)).pTabSchema =
                                            unsafe { (*(*__slate_slot_396)).pSchema };
                                    }
                                }
                                *__slate_slot_394 = unsafe { (*(*__slate_slot_394)).next };
                            } else {
                                break;
                            }
                        }
                        unsafe { sqlite3BtreeClose(unsafe { (*(*__slate_slot_393)).pBt }) };
                        unsafe {
                            (*(*__slate_slot_393)).pBt = std::ptr::null_mut::<Btree>();
                        }
                        unsafe {
                            (*(*__slate_slot_393)).pSchema = std::ptr::null_mut::<Schema>();
                        }
                        unsafe { sqlite3CollapseDatabaseArray(*__slate_slot_391) };
                        return;
                    }
                }
            }
        }
        unsafe {
            sqlite3_result_error(
                context,
                ((*__slate_slot_395).as_mut_ptr() as *mut i8) as *const i8,
                -(1 as i32),
            )
        };
    }
}

// /*
// ** This procedure generates VDBE code for a single invocation of either the
// ** sqlite_detach() or sqlite_attach() SQL user functions.
// */
fn codeAttach(
    mut pParse: *mut Parse,
    mut r#type: i32,
    mut pFunc: *const FuncDef,
    mut pAuthArg: *mut Expr,
    mut pFilename: *mut Expr,
    mut pDbname: *mut Expr,
    mut pKey: *mut Expr,
) {
    let mut __slate_storage_411: std::mem::MaybeUninit<*mut i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_411: *mut *mut i8 =
        std::ptr::addr_of_mut!(__slate_storage_411) as *mut *mut i8;
    let mut __slate_storage_630: std::mem::MaybeUninit<bool> = std::mem::MaybeUninit::uninit();
    let __slate_slot_630: *mut bool = std::ptr::addr_of_mut!(__slate_storage_630) as *mut bool;
    let mut __slate_storage_629: std::mem::MaybeUninit<bool> = std::mem::MaybeUninit::uninit();
    let __slate_slot_629: *mut bool = std::ptr::addr_of_mut!(__slate_storage_629) as *mut bool;
    let mut __slate_storage_410: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_410: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_410) as *mut i32;
    let mut __slate_storage_409: std::mem::MaybeUninit<*mut sqlite3> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_409: *mut *mut sqlite3 =
        std::ptr::addr_of_mut!(__slate_storage_409) as *mut *mut sqlite3;
    let mut __slate_storage_408: std::mem::MaybeUninit<*mut Vdbe> = std::mem::MaybeUninit::uninit();
    let __slate_slot_408: *mut *mut Vdbe =
        std::ptr::addr_of_mut!(__slate_storage_408) as *mut *mut Vdbe;
    let mut __slate_storage_407: std::mem::MaybeUninit<NameContext> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_407: *mut NameContext =
        std::ptr::addr_of_mut!(__slate_storage_407) as *mut NameContext;
    let mut __slate_storage_406: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_406: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_406) as *mut i32;
    unsafe {
        '__join_0: {
            std::ptr::write(__slate_slot_409, unsafe { (*pParse).db });
            if (0 as i32) != unsafe { sqlite3ReadSchema(pParse) } {
            } else {
                if (unsafe { (*pParse).nErr }) != (0 as i32) {
                } else {
                    unsafe {
                        memset(
                            std::ptr::addr_of_mut!(*__slate_slot_407) as *mut (),
                            0 as i32,
                            56 as u64,
                        )
                    };
                    (*__slate_slot_407).pParse = pParse;
                    if (0 as i32)
                        != resolveAttachExpr(std::ptr::addr_of_mut!(*__slate_slot_407), pFilename)
                    {
                        *__slate_slot_629 = true as bool;
                    } else {
                        *__slate_slot_629 = (0 as i32)
                            != resolveAttachExpr(
                                std::ptr::addr_of_mut!(*__slate_slot_407),
                                pDbname,
                            );
                    }
                    if *__slate_slot_629 {
                        *__slate_slot_630 = true as bool;
                    } else {
                        *__slate_slot_630 = (0 as i32)
                            != resolveAttachExpr(std::ptr::addr_of_mut!(*__slate_slot_407), pKey);
                    }
                    if *__slate_slot_630 {
                    } else {
                        if pAuthArg != std::ptr::null_mut::<Expr>() {
                            if (((unsafe { (*pAuthArg).op }) as u32) as i32) == (118 as i32) {
                                0 as i32;
                                *__slate_slot_411 = unsafe { (*pAuthArg).u.zToken };
                            } else {
                                *__slate_slot_411 = std::ptr::null_mut::<i8>();
                            }
                            *__slate_slot_406 = unsafe {
                                sqlite3AuthCheck(
                                    pParse,
                                    r#type,
                                    *__slate_slot_411 as *const i8,
                                    std::ptr::null::<i8>(),
                                    std::ptr::null::<i8>(),
                                )
                            };
                            if *__slate_slot_406 != (0 as i32) {
                                break '__join_0;
                            }
                        }
                        // /* SQLITE_OMIT_AUTHORIZATION */
                        *__slate_slot_408 = unsafe { sqlite3GetVdbe(pParse) };
                        *__slate_slot_410 = unsafe { sqlite3GetTempRange(pParse, 4 as i32) };
                        unsafe { sqlite3ExprCode(pParse, pFilename, *__slate_slot_410) };
                        unsafe { sqlite3ExprCode(pParse, pDbname, *__slate_slot_410 + (1 as i32)) };
                        unsafe { sqlite3ExprCode(pParse, pKey, *__slate_slot_410 + (2 as i32)) };
                        0 as i32;
                        if *__slate_slot_408 != std::ptr::null_mut::<Vdbe>() {
                            unsafe {
                                sqlite3VdbeAddFunctionCall(
                                    pParse,
                                    0 as i32,
                                    *__slate_slot_410 + (3 as i32)
                                        - ((unsafe { (*pFunc).nArg }) as i32),
                                    *__slate_slot_410 + (3 as i32),
                                    (unsafe { (*pFunc).nArg }) as i32,
                                    pFunc,
                                    0 as i32,
                                )
                            };
                            // /* Code an OP_Expire. For an ATTACH statement, set P1 to true (expire this
                            //     ** statement only). For DETACH, set it to false (expire all existing
                            //     ** statements).
                            //     */
                            unsafe {
                                sqlite3VdbeAddOp1(
                                    *__slate_slot_408,
                                    168 as i32,
                                    (r#type == (24 as i32)) as i32,
                                )
                            };
                        }
                    }
                }
            }
        }
        unsafe { sqlite3ExprDelete(*__slate_slot_409, pFilename) };
        unsafe { sqlite3ExprDelete(*__slate_slot_409, pDbname) };
        unsafe { sqlite3ExprDelete(*__slate_slot_409, pKey) };
    }
}

// /* SQLITE_OMIT_ATTACH */
// /*
// ** Expression callback used by sqlite3FixAAAA() routines.
// */
#[unsafe(link_section = ".text.slate_distinct.attach.fixExprCb")]
extern "C-unwind" fn fixExprCb(mut p: *mut Walker, mut pExpr: *mut Expr) -> i32 {
    let mut pFix: *mut DbFixer = unsafe { (*p).u.pFix };
    if !((unsafe { (*pFix).bTemp }) != (0 as u8)) {
        let __v631: *mut Expr = pExpr;
        let __v632: u32 = unsafe { (*__v631).flags };
        let __v633: u32 = __v632 | ((1073741824 as i32) as u32);
        unsafe {
            (*__v631).flags = __v633;
        }
    }
    if (((unsafe { (*pExpr).op }) as u32) as i32) == (157 as i32) {
        if (unsafe { (*unsafe { (*unsafe { (*pFix).pParse }).db }).init.busy }) != (0 as u8) {
            unsafe {
                (*pExpr).op = ((122 as i32) as i8) as u8;
            }
        } else {
            unsafe {
                sqlite3ErrorMsg(
                    unsafe { (*pFix).pParse },
                    (b"%s cannot use variables\0".as_ptr() as *mut i8) as *const i8,
                    unsafe { (*pFix).zType },
                )
            };
            return 2 as i32;
        }
    }
    return 0 as i32;
}

// /*
// ** Select callback used by sqlite3FixAAAA() routines.
// */
#[unsafe(link_section = ".text.slate_distinct.attach.fixSelectCb")]
extern "C-unwind" fn fixSelectCb(mut p: *mut Walker, mut pSelect: *mut Select) -> i32 {
    let mut pFix: *mut DbFixer = unsafe { (*p).u.pFix };
    let mut i: i32 = 0 as i32;
    let mut pItem: *mut SrcItem = unsafe { std::mem::zeroed() };
    let mut db: *mut sqlite3 = unsafe { (*unsafe { (*pFix).pParse }).db };
    let mut iDb: i32 = unsafe { sqlite3FindDbName(db, unsafe { (*pFix).zDb }) };
    let mut pList: *mut SrcList = unsafe { (*pSelect).pSrc };
    if pList == std::ptr::null_mut::<SrcList>() {
        return 0 as i32;
    }
    i = 0 as i32;
    let __v634: *mut SrcItem = unsafe { std::ptr::addr_of_mut!((*pList).a) as *mut SrcItem };
    pItem = __v634;
    '__slate_break_596: while i < unsafe { (*pList).nSrc } {
        if (((unsafe { (*pFix).bTemp }) as u32) as i32) == (0 as i32)
            && ((unsafe { (*pItem).fg.__slate_bits_0.__get_isSubquery() }) as i32) == (0 as i32)
        {
            if ((unsafe { (*pItem).fg.__slate_bits_0.__get_fixedSchema() }) as i32) == (0 as i32)
                && (unsafe { (*pItem).u4.zDatabase }) != std::ptr::null_mut::<i8>()
            {
                if iDb
                    != unsafe {
                        sqlite3FindDbName(db, (unsafe { (*pItem).u4.zDatabase }) as *const i8)
                    }
                {
                    unsafe {
                        sqlite3ErrorMsg(
                            unsafe { (*pFix).pParse },
                            (b"%s %T cannot reference objects in database %s\0".as_ptr() as *mut i8)
                                as *const i8,
                            unsafe { (*pFix).zType },
                            unsafe { (*pFix).pName },
                            unsafe { (*pItem).u4.zDatabase },
                        )
                    };
                    return 2 as i32;
                }
                unsafe { sqlite3DbFree(db, (unsafe { (*pItem).u4.zDatabase }) as *mut ()) };
                unsafe {
                    (*pItem).fg.__slate_bits_0.__set_notCte((1 as i32) as u32);
                }
                unsafe {
                    (*pItem)
                        .fg
                        .__slate_bits_0
                        .__set_hadSchema((1 as i32) as u32);
                }
            }
            unsafe {
                (*pItem).u4.pSchema = unsafe { (*pFix).pSchema };
            }
            unsafe {
                (*pItem).fg.__slate_bits_0.__set_fromDDL((1 as i32) as u32);
            }
            unsafe {
                (*pItem)
                    .fg
                    .__slate_bits_0
                    .__set_fixedSchema((1 as i32) as u32);
            }
        }
        let __v639: bool;
        if ((unsafe {
            (*unsafe {
                unsafe { std::ptr::addr_of_mut!((*pList).a) as *mut SrcItem }.offset(i as isize)
            })
            .fg
            .__slate_bits_0
            .__get_isUsing()
        }) as i32)
            == (0 as i32)
        {
            __v639 = (unsafe {
                sqlite3WalkExpr(unsafe { std::ptr::addr_of_mut!((*pFix).w) }, unsafe {
                    (*unsafe {
                        unsafe { std::ptr::addr_of_mut!((*pList).a) as *mut SrcItem }
                            .offset(i as isize)
                    })
                    .u3
                    .pOn
                })
            }) != (0 as i32);
        } else {
            __v639 = false as bool;
        }
        if __v639 {
            return 2 as i32;
        }
        let __v635: i32 = i;
        let __v636: i32 = __v635 + (1 as i32);
        i = __v636;
        let __v637: *mut SrcItem = pItem;
        let __v638: *mut SrcItem = unsafe { __v637.offset((1 as i32) as isize) };
        pItem = __v638;
    }
    if (unsafe { (*pSelect).pWith }) != std::ptr::null_mut::<With>() {
        i = 0 as i32;
        '__slate_break_598: loop {
            if !(i < unsafe { (*unsafe { (*pSelect).pWith }).nCte }) {
                break;
            }
            if (unsafe {
                sqlite3WalkSelect(p, unsafe {
                    (*unsafe {
                        unsafe {
                            std::ptr::addr_of_mut!((*unsafe { (*pSelect).pWith }).a) as *mut Cte
                        }
                        .offset(i as isize)
                    })
                    .pSelect
                })
            }) != (0 as i32)
            {
                return 2 as i32;
            }
            let __v640: i32 = i;
            let __v641: i32 = __v640 + (1 as i32);
            i = __v641;
        }
    }
    return 0 as i32;
}
