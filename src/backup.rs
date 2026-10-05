unsafe extern "C" {
    static mut sqlite3PendingByte: i32;
    fn sqlite3_free(__v480: *mut ());
    fn sqlite3_mutex_enter(__v481: *mut sqlite3_mutex);
    fn sqlite3_mutex_leave(__v482: *mut sqlite3_mutex);
    fn memcpy(__dest: *mut (), __src: *const (), __n: u64) -> *mut ();
    fn memset(__s: *mut (), __c: i32, __n: u64) -> *mut ();
    fn sqlite3OsWrite(__v498: *mut sqlite3_file, __v499: *const (), amt: i32, offset: i64) -> i32;
    fn sqlite3OsTruncate(__v502: *mut sqlite3_file, size: i64) -> i32;
    fn sqlite3OsFileSize(__v504: *mut sqlite3_file, pSize: *mut i64) -> i32;
    fn sqlite3OsFileControl(__v506: *mut sqlite3_file, __v507: i32, __v508: *mut ()) -> i32;
    fn sqlite3PagerGetJournalMode(__v509: *mut Pager) -> i32;
    fn sqlite3PagerBackupPtr(__v510: *mut Pager) -> *mut *mut sqlite3_backup;
    fn sqlite3PagerGet(pPager: *mut Pager, pgno: u32, ppPage: *mut *mut PgHdr, clrFlag: i32)
    -> i32;
    fn sqlite3PagerUnref(__v515: *mut PgHdr);
    fn sqlite3PagerWrite(__v516: *mut PgHdr) -> i32;
    fn sqlite3PagerGetData(__v517: *mut PgHdr) -> *mut ();
    fn sqlite3PagerGetExtra(__v518: *mut PgHdr) -> *mut ();
    fn sqlite3PagerPagecount(__v519: *mut Pager, __v520: *mut i32);
    fn sqlite3PagerCommitPhaseOne(__v521: *mut Pager, zSuper: *const i8, __v523: i32) -> i32;
    fn sqlite3PagerSync(pPager: *mut Pager, zSuper: *const i8) -> i32;
    fn sqlite3PagerFile(__v526: *mut Pager) -> *mut sqlite3_file;
    fn sqlite3PagerIsMemdb(__v527: *mut Pager) -> i32;
    fn sqlite3PagerClearCache(__v528: *mut Pager);
    fn sqlite3PagerTruncateImage(__v529: *mut Pager, __v530: u32);
    fn sqlite3BtreeSetPageSize(p: *mut Btree, nPagesize: i32, nReserve: i32, eFix: i32) -> i32;
    fn sqlite3BtreeGetPageSize(__v535: *mut Btree) -> i32;
    fn sqlite3BtreeLastPage(__v536: *mut Btree) -> u32;
    fn sqlite3BtreeBeginTrans(__v537: *mut Btree, __v538: i32, __v539: *mut i32) -> i32;
    fn sqlite3BtreeCommitPhaseOne(__v540: *mut Btree, __v541: *const i8) -> i32;
    fn sqlite3BtreeCommitPhaseTwo(__v542: *mut Btree, __v543: i32) -> i32;
    fn sqlite3BtreeRollback(__v544: *mut Btree, __v545: i32, __v546: i32) -> i32;
    fn sqlite3BtreeTxnState(__v547: *mut Btree) -> i32;
    fn sqlite3BtreeUpdateMeta(__v550: *mut Btree, idx: i32, value: u32) -> i32;
    fn sqlite3BtreeNewDb(p: *mut Btree) -> i32;
    fn sqlite3BtreePager(__v554: *mut Btree) -> *mut Pager;
    fn sqlite3BtreeSetVersion(pBt: *mut Btree, iVersion: i32) -> i32;
    fn sqlite3BtreeEnter(__v557: *mut Btree);
    fn sqlite3BtreeLeave(__v558: *mut Btree);
    fn sqlite3Strlen30(__v559: *const i8) -> i32;
    fn sqlite3MallocZero(__v560: u64) -> *mut ();
    fn sqlite3DbFree(__v561: *mut sqlite3, __v562: *mut ());
    fn sqlite3ResetAllSchemasOfConnection(__v563: *mut sqlite3);
    fn sqlite3LeaveMutexAndCloseZombie(__v564: *mut sqlite3);
    fn sqlite3ErrorWithMsg(__v565: *mut sqlite3, __v566: i32, __v567: *const i8, ...);
    fn sqlite3Error(__v568: *mut sqlite3, __v569: i32);
    fn sqlite3FindDbName(__v570: *mut sqlite3, __v571: *const i8) -> i32;
    fn sqlite3OpenTempDatabase(__v572: *mut Parse) -> i32;
    fn sqlite3ParseObjectInit(__v577: *mut Parse, __v578: *mut sqlite3);
    fn sqlite3ParseObjectReset(__v579: *mut Parse);
    fn sqlite3Put4byte(__v580: *mut u8, __v581: u32);
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
    trace: __SlateRecord174,
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
    u1: __SlateRecord175,
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
struct sqlite3_pcache_page {
    pBuf: *mut (),
    pExtra: *mut (),
}

#[repr(C)]
#[derive(Clone, Copy)]
struct sqlite3_backup {
    pDestDb: *mut sqlite3,
    zDestDb: *mut i8,
    pDest: *mut Btree,
    iDestSchema: u32,
    bDestLocked: i32,
    iNext: u32,
    pSrcDb: *mut sqlite3,
    pSrc: *mut Btree,
    rc: i32,
    nRemaining: u32,
    nPagecount: u32,
    isAttached: i32,
    pNext: *mut sqlite3_backup,
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
struct Bitvec {}

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
    __slate_bits_0: __slate_bits::__SlateBits70U0,
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
    u: __SlateRecord185,
    pLeft: *mut Expr,
    pRight: *mut Expr,
    x: __SlateRecord186,
    nHeight: i32,
    iTable: i32,
    iColumn: i16,
    iAgg: i16,
    w: __SlateRecord187,
    pAggInfo: *mut AggInfo,
    y: __SlateRecord188,
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
    u: __SlateRecord176,
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
    __slate_bits_0: __slate_bits::__SlateBits94U0,
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
    __slate_bits_0: __slate_bits::__SlateBits106U0,
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
    u1: __SlateRecord201,
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
    fg: __SlateRecord195,
    iCursor: i32,
    colUsed: u64,
    u1: __SlateRecord196,
    u2: __SlateRecord197,
    u3: __SlateRecord198,
    u4: __SlateRecord199,
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
    u: __SlateRecord177,
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
struct PgHdr {
    pPage: *mut sqlite3_pcache_page,
    pData: *mut (),
    pExtra: *mut (),
    pCache: *mut PCache,
    pDirty: *mut PgHdr,
    pPager: *mut Pager,
    pgno: u32,
    flags: u16,
    nRef: i64,
    pDirtyNext: *mut PgHdr,
    pDirtyPrev: *mut PgHdr,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct Btree {
    db: *mut sqlite3,
    pBt: *mut BtShared,
    inTrans: u8,
    sharable: u8,
    locked: u8,
    hasIncrblobCur: u8,
    wantToLock: i32,
    nBackup: i32,
    iBDataVersion: u32,
    pNext: *mut Btree,
    pPrev: *mut Btree,
    lock: BtLock,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct BtCursor {
    eState: u8,
    curFlags: u8,
    curPagerFlags: u8,
    hints: u8,
    skipNext: i32,
    pBtree: *mut Btree,
    aOverflow: *mut u32,
    pKey: *mut (),
    pBt: *mut BtShared,
    pNext: *mut BtCursor,
    info: CellInfo,
    nKey: i64,
    pgnoRoot: u32,
    iPage: i8,
    curIntKey: u8,
    ix: u16,
    aiIdx: [u16; 19],
    pKeyInfo: *mut KeyInfo,
    pPage: *mut MemPage,
    apPage: [*mut MemPage; 19],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct BtShared {
    pPager: *mut Pager,
    db: *mut sqlite3,
    pCursor: *mut BtCursor,
    pPage1: *mut MemPage,
    openFlags: u8,
    autoVacuum: u8,
    incrVacuum: u8,
    bDoTruncate: u8,
    inTransaction: u8,
    max1bytePayload: u8,
    nReserveWanted: u8,
    btsFlags: u16,
    maxLocal: u16,
    minLocal: u16,
    maxLeaf: u16,
    minLeaf: u16,
    pageSize: u32,
    usableSize: u32,
    nTransaction: i32,
    nPage: u32,
    pSchema: *mut (),
    xFreeSchema: Option<unsafe extern "C-unwind" fn(*mut ())>,
    mutex: *mut sqlite3_mutex,
    pHasContent: *mut Bitvec,
    nRef: i32,
    pNext: *mut BtShared,
    pLock: *mut BtLock,
    pWriter: *mut Btree,
    pTmpSpace: *mut u8,
    nPreformatSize: i32,
}

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
struct PCache {}

#[repr(C)]
#[derive(Clone, Copy)]
struct sqlite3InitInfo {
    newTnum: u32,
    iDb: u8,
    busy: u8,
    __slate_bits_0: __slate_bits::__SlateBits173U0,
    azInit: *mut *const i8,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord174 {
    xLegacy: Option<unsafe extern "C-unwind" fn(*mut (), *const i8)>,
    xV2: Option<unsafe extern "C-unwind" fn(u32, *mut (), *mut (), *mut ()) -> i32>,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord175 {
    isInterrupted: i32,
    notUsed1: f64,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord176 {
    pHash: *mut FuncDef,
    pDestructor: *mut FuncDestructor,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord177 {
    tab: __SlateRecord178,
    view: __SlateRecord179,
    vtab: __SlateRecord180,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord178 {
    addColOffset: i32,
    pFKey: *mut FKey,
    pDfltList: *mut ExprList,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord179 {
    pSelect: *mut Select,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord180 {
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
union __SlateRecord185 {
    zToken: *mut i8,
    iValue: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord186 {
    pList: *mut ExprList,
    pSelect: *mut Select,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord187 {
    iJoin: i32,
    iOfst: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord188 {
    pTab: *mut Table,
    pWin: *mut Window,
    nReg: i32,
    sub: __SlateRecord189,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord189 {
    iAddr: i32,
    regReturn: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct ExprList_item {
    pExpr: *mut Expr,
    zEName: *mut i8,
    fg: __SlateRecord191,
    u: __SlateRecord192,
}

#[repr(C, align(4))]
#[derive(Clone, Copy)]
struct __SlateRecord191 {
    sortFlags: u8,
    __slate_bits_0: __slate_bits::__SlateBits191U0,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord192 {
    x: __SlateRecord193,
    iConstExprReg: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord193 {
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
struct __SlateRecord195 {
    jointype: u8,
    __slate_bits_0: __slate_bits::__SlateBits195U0,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord196 {
    zIndexedBy: *mut i8,
    pFuncArg: *mut ExprList,
    nRow: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord197 {
    pIBIndex: *mut Index,
    pCteUse: *mut CteUse,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord198 {
    pOn: *mut Expr,
    pUsing: *mut IdList,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord199 {
    pSchema: *mut Schema,
    zDatabase: *mut i8,
    pSubq: *mut Subquery,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord201 {
    cr: __SlateRecord202,
    d: __SlateRecord203,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord202 {
    addrCrTab: i32,
    regRowid: i32,
    regRoot: i32,
    constraintName: Token,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord203 {
    pReturning: *mut Returning,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct MemPage {
    isInit: u8,
    intKey: u8,
    intKeyLeaf: u8,
    pgno: u32,
    leaf: u8,
    hdrOffset: u8,
    childPtrSize: u8,
    max1bytePayload: u8,
    nOverflow: u8,
    maxLocal: u16,
    minLocal: u16,
    cellOffset: u16,
    nFree: i32,
    nCell: u16,
    maskPage: u16,
    aiOvfl: [u16; 4],
    apOvfl: [*mut u8; 4],
    pBt: *mut BtShared,
    aData: *mut u8,
    aDataEnd: *mut u8,
    aCellIdx: *mut u8,
    aDataOfst: *mut u8,
    pDbPage: *mut PgHdr,
    xCellSize: Option<unsafe extern "C-unwind" fn(*mut MemPage, *mut u8) -> u16>,
    xParseCell: Option<unsafe extern "C-unwind" fn(*mut MemPage, *mut u8, *mut CellInfo)>,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct BtLock {
    pBtree: *mut Btree,
    iTable: u32,
    eLock: u8,
    pNext: *mut BtLock,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct CellInfo {
    nKey: i64,
    pPayload: *mut u8,
    nPayload: u32,
    nLocal: u16,
    nSize: u16,
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
    pub struct __SlateBits70U0 {
        #[bits(4)]
        pub notNull: u32,
        #[bits(4)]
        pub eCType: u32,
    }
    #[bitfields::bitfield([u8; 2], c_names = true, new = false, from_into_bits = false, from_traits = false, default = false, debug = false, builder = false, bit_ops = false)]
    pub struct __SlateBits191U0 {
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
    pub struct __SlateBits195U0 {
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
    pub struct __SlateBits94U0 {
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
    pub struct __SlateBits173U0 {
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
    pub struct __SlateBits106U0 {
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
// ** Create an sqlite3_backup process to copy the contents of zSrcDb from
// ** connection handle pSrcDb to zDestDb in pDestDb. If successful, return
// ** a pointer to the new sqlite3_backup object.
// **
// ** If an error occurs, NULL is returned and an error code and error message
// ** stored in database handle pDestDb.
// */
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.backup.sqlite3_backup_init")]
extern "C-unwind" fn sqlite3_backup_init(
    mut pDestDb: *mut sqlite3,
    mut zDestDb: *const i8,
    mut pSrcDb: *mut sqlite3,
    mut zSrcDb: *const i8,
) -> *mut sqlite3_backup {
    // /* Value to return */
    let mut p: *mut sqlite3_backup = unsafe { std::mem::zeroed() };
    // /* Lock the source database handle. The destination database
    //   ** handle is not locked in this routine, but it is locked in
    //   ** sqlite3_backup_step(). The user is required to ensure that no
    //   ** other thread accesses the destination handle for the duration
    //   ** of the backup operation.  Any attempt to use the destination
    //   ** database connection while a backup is in progress may cause
    //   ** a malfunction or a deadlock.
    //   */
    unsafe { sqlite3_mutex_enter(unsafe { (*pSrcDb).mutex }) };
    unsafe { sqlite3_mutex_enter(unsafe { (*pDestDb).mutex }) };
    if pSrcDb == pDestDb {
        unsafe {
            sqlite3ErrorWithMsg(
                pDestDb,
                1 as i32,
                (b"source and destination must be distinct\0".as_ptr() as *mut i8) as *const i8,
            )
        };
        p = std::ptr::null_mut::<sqlite3_backup>();
    } else {
        let mut nDest: i32 = unsafe { sqlite3Strlen30(zDestDb) };
        // /* Allocate space for a new sqlite3_backup object...
        //     ** EVIDENCE-OF: R-64852-21591 The sqlite3_backup object is created by a
        //     ** call to sqlite3_backup_init() and is destroyed by a call to
        //     ** sqlite3_backup_finish(). */
        p = (unsafe {
            sqlite3MallocZero(
                (80 as u64)
                    .wrapping_add((nDest as i64) as u64)
                    .wrapping_add(((1 as i32) as i64) as u64),
            )
        }) as *mut sqlite3_backup;
        if !(p != std::ptr::null_mut::<sqlite3_backup>()) {
            unsafe { sqlite3Error(pDestDb, 7 as i32) };
        } else {
            unsafe {
                (*p).zDestDb = (unsafe { p.offset((1 as i32) as isize) }) as *mut i8;
            }
            unsafe {
                memcpy(
                    (unsafe { (*p).zDestDb }) as *mut (),
                    zDestDb as *const (),
                    (nDest as i64) as u64,
                )
            };
        }
    }
    // /* If the allocation succeeded, populate the new object. */
    if p != std::ptr::null_mut::<sqlite3_backup>() {
        // /* Do not store the pointer to the destination b-tree at this point.
        //     ** This is because there is nothing preventing it from being detached
        //     ** or otherwise freed before the first call to sqlite3_backup_step()
        //     ** on this object. The source b-tree does not have this problem, as
        //     ** incrementing Btree.nBackup (see below) effectively locks the object. */
        let mut pDest: *mut Btree = findBtree(pDestDb, pDestDb, zDestDb);
        unsafe {
            (*p).pSrc = findBtree(pDestDb, pSrcDb, zSrcDb);
        }
        unsafe {
            (*p).pDestDb = pDestDb;
        }
        unsafe {
            (*p).pSrcDb = pSrcDb;
        }
        unsafe {
            (*p).iNext = (1 as i32) as u32;
        }
        unsafe {
            (*p).isAttached = 0 as i32;
        }
        let __v593: bool;
        if std::ptr::null_mut::<Btree>() == unsafe { (*p).pSrc }
            || std::ptr::null_mut::<Btree>() == pDest
        {
            __v593 = true as bool;
        } else {
            __v593 = checkReadTransaction(pDestDb, pDest) != (0 as i32);
        }
        if __v593 {
            // /* One (or both) of the named databases did not exist or an OOM
            //       ** error was hit. Or there is a transaction open on the destination
            //       ** database. The error has already been written into the pDestDb
            //       ** handle. All that is left to do here is free the sqlite3_backup
            //       ** structure.  */
            unsafe { sqlite3_free(p as *mut ()) };
            p = std::ptr::null_mut::<sqlite3_backup>();
        }
    }
    if p != std::ptr::null_mut::<sqlite3_backup>() {
        let __v594: *mut Btree = unsafe { (*p).pSrc };
        let __v595: i32 = unsafe { (*__v594).nBackup };
        let __v596: i32 = __v595 + (1 as i32);
        unsafe {
            (*__v594).nBackup = __v596;
        }
    }
    unsafe { sqlite3_mutex_leave(unsafe { (*pDestDb).mutex }) };
    unsafe { sqlite3_mutex_leave(unsafe { (*pSrcDb).mutex }) };
    return p;
}

// /*
// ** Copy nPage pages from the source b-tree to the destination.
// */
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.backup.sqlite3_backup_step")]
extern "C-unwind" fn sqlite3_backup_step(mut p: *mut sqlite3_backup, mut nPage: i32) -> i32 {
    let mut rc: i32 = 0 as i32;
    // /* Destination journal mode */
    let mut destMode: i32 = 0 as i32;
    // /* Source page size */
    let mut pgszSrc: i32 = 0 as i32;
    // /* Destination page size */
    let mut pgszDest: i32 = 0 as i32;
    unsafe { sqlite3_mutex_enter(unsafe { (*unsafe { (*p).pSrcDb }).mutex }) };
    unsafe { sqlite3BtreeEnter(unsafe { (*p).pSrc }) };
    if (unsafe { (*p).pDestDb }) != std::ptr::null_mut::<sqlite3>() {
        unsafe { sqlite3_mutex_enter(unsafe { (*unsafe { (*p).pDestDb }).mutex }) };
    }
    rc = unsafe { (*p).rc };
    if !(isFatalError(rc) != (0 as i32)) {
        // /* Source pager */
        let mut pSrcPager: *mut Pager = unsafe { sqlite3BtreePager(unsafe { (*p).pSrc }) };
        // /* Dest btree */
        let mut pDest: *mut Btree = std::ptr::null_mut::<Btree>();
        // /* Dest pager */
        let mut pDestPager: *mut Pager = std::ptr::null_mut::<Pager>();
        // /* Iterator variable */
        let mut ii: i32 = 0 as i32;
        // /* Size of source db in pages */
        let mut nSrcPage: i32 = -(1 as i32);
        // /* True if src db requires unlocking */
        let mut bCloseTrans: i32 = 0 as i32;
        // /* If the source pager is currently in a write-transaction, return
        //     ** SQLITE_BUSY immediately.
        //     */
        if (unsafe { (*p).pDestDb }) != std::ptr::null_mut::<sqlite3>()
            && (((unsafe { (*unsafe { (*unsafe { (*p).pSrc }).pBt }).inTransaction }) as u32)
                as i32)
                == (2 as i32)
        {
            rc = 5 as i32;
        } else {
            rc = 0 as i32;
        }
        // /* If there is no open read-transaction on the source database, open
        //     ** one now. If a transaction is opened here, then it will be closed
        //     ** before this function exits.
        //     */
        let __v597: bool;
        if rc == (0 as i32) {
            __v597 = (0 as i32) == unsafe { sqlite3BtreeTxnState(unsafe { (*p).pSrc }) };
        } else {
            __v597 = false as bool;
        }
        if __v597 {
            rc = unsafe {
                sqlite3BtreeBeginTrans(unsafe { (*p).pSrc }, 0 as i32, std::ptr::null_mut::<i32>())
            };
            bCloseTrans = 1 as i32;
        }
        // /* Locate the destination btree and pager. */
        let __v598: *mut Btree = unsafe { (*p).pDest };
        pDest = __v598;
        if __v598 == std::ptr::null_mut::<Btree>() {
            pDest = findBtree(
                unsafe { (*p).pDestDb },
                unsafe { (*p).pDestDb },
                (unsafe { (*p).zDestDb }) as *const i8,
            );
        }
        if pDest == std::ptr::null_mut::<Btree>() {
            rc = 1 as i32;
        } else {
            pDestPager = unsafe { sqlite3BtreePager(pDest) };
        }
        // /* If the destination database has not yet been locked (i.e. if this
        //     ** is the first call to backup_step() for the current backup operation),
        //     ** try to set its page size to the same as the source database. This
        //     ** is especially important on ZipVFS systems, as in that case it is
        //     ** not possible to create a database file that uses one page size by
        //     ** writing to it with another.  */
        let __v599: bool;
        if (unsafe { (*p).bDestLocked }) == (0 as i32) && rc == (0 as i32) {
            __v599 = setDestPgsz(pDest, unsafe { (*p).pSrc }) == (7 as i32);
        } else {
            __v599 = false as bool;
        }
        if __v599 {
            rc = 7 as i32;
        }
        // /* Lock the destination database, if it is not locked already. */
        let __v600: bool;
        if (0 as i32) == rc && (unsafe { (*p).bDestLocked }) == (0 as i32) {
            let __v601: i32 = unsafe {
                sqlite3BtreeBeginTrans(
                    pDest,
                    2 as i32,
                    (unsafe { std::ptr::addr_of_mut!((*p).iDestSchema) }) as *mut i32,
                )
            };
            rc = __v601;
            __v600 = (0 as i32) == __v601;
        } else {
            __v600 = false as bool;
        }
        if __v600 {
            unsafe {
                (*p).bDestLocked = 1 as i32;
            }
            unsafe {
                (*p).pDest = pDest;
            }
        }
        // /* Do not allow backup if the destination database is in WAL mode
        //     ** and the page sizes are different between source and destination */
        if rc == (0 as i32) {
            pgszSrc = unsafe { sqlite3BtreeGetPageSize(unsafe { (*p).pSrc }) };
            pgszDest = unsafe { sqlite3BtreeGetPageSize(unsafe { (*p).pDest }) };
            destMode = unsafe {
                sqlite3PagerGetJournalMode(unsafe { sqlite3BtreePager(unsafe { (*p).pDest }) })
            };
            let __v602: bool;
            if destMode == (5 as i32) {
                __v602 = true as bool;
            } else {
                __v602 = (unsafe { sqlite3PagerIsMemdb(pDestPager) }) != (0 as i32);
            }
            if __v602 && pgszSrc != pgszDest {
                rc = 8 as i32;
            }
        }
        // /* Now that there is a read-lock on the source database, query the
        //     ** source pager for the number of pages in the database.
        //     */
        nSrcPage = (unsafe { sqlite3BtreeLastPage(unsafe { (*p).pSrc }) }) as i32;
        0 as i32;
        ii = 0 as i32;
        '__slate_break_587: loop {
            if !((nPage < (0 as i32) || ii < nPage)
                && (unsafe { (*p).iNext }) <= (nSrcPage as u32)
                && !(rc != (0 as i32)))
            {
                break;
            }
            // /* Source page number */
            let mut iSrcPg: u32 = unsafe { (*p).iNext };
            if iSrcPg
                != (((unsafe { sqlite3PendingByte }) as u32)
                    / unsafe { (*unsafe { (*unsafe { (*p).pSrc }).pBt }).pageSize })
                .wrapping_add((1 as i32) as u32)
            {
                // /* Source page object */
                let mut pSrcPg: *mut PgHdr = unsafe { std::mem::zeroed() };
                rc = unsafe {
                    sqlite3PagerGet(pSrcPager, iSrcPg, std::ptr::addr_of_mut!(pSrcPg), 2 as i32)
                };
                if rc == (0 as i32) {
                    rc = backupOnePage(
                        p,
                        iSrcPg,
                        (unsafe { sqlite3PagerGetData(pSrcPg) }) as *const u8,
                        0 as i32,
                    );
                    unsafe { sqlite3PagerUnref(pSrcPg) };
                }
            }
            let __v605: *mut sqlite3_backup = p;
            let __v606: u32 = unsafe { (*__v605).iNext };
            let __v607: u32 = __v606.wrapping_add((1 as i32) as u32);
            unsafe {
                (*__v605).iNext = __v607;
            }
            let __v603: i32 = ii;
            let __v604: i32 = __v603 + (1 as i32);
            ii = __v604;
        }
        if rc == (0 as i32) {
            unsafe {
                (*p).nPagecount = nSrcPage as u32;
            }
            unsafe {
                (*p).nRemaining =
                    ((nSrcPage + (1 as i32)) as u32).wrapping_sub(unsafe { (*p).iNext });
            }
            if (unsafe { (*p).iNext }) > (nSrcPage as u32) {
                rc = 101 as i32;
            } else {
                if !((unsafe { (*p).isAttached }) != (0 as i32)) {
                    attachBackupObject(p);
                }
            }
        }
        // /* Update the schema version field in the destination database. This
        //     ** is to make sure that the schema-version really does change in
        //     ** the case where the source and destination databases have the
        //     ** same schema version.
        //     */
        if rc == (101 as i32) {
            if nSrcPage == (0 as i32) {
                rc = unsafe { sqlite3BtreeNewDb(unsafe { (*p).pDest }) };
                nSrcPage = 1 as i32;
            }
            if rc == (0 as i32) || rc == (101 as i32) {
                rc = unsafe {
                    sqlite3BtreeUpdateMeta(
                        unsafe { (*p).pDest },
                        1 as i32,
                        unsafe { (*p).iDestSchema }.wrapping_add((1 as i32) as u32),
                    )
                };
            }
            if rc == (0 as i32) {
                if (unsafe { (*p).pDestDb }) != std::ptr::null_mut::<sqlite3>() {
                    unsafe { sqlite3ResetAllSchemasOfConnection(unsafe { (*p).pDestDb }) };
                }
                if destMode == (5 as i32) {
                    rc = unsafe { sqlite3BtreeSetVersion(unsafe { (*p).pDest }, 2 as i32) };
                }
            }
            if rc == (0 as i32) {
                let mut nDestTruncate: i32 = 0 as i32;
                // /* Set nDestTruncate to the final number of pages in the destination
                //         ** database. The complication here is that the destination page
                //         ** size may be different to the source page size.
                //         **
                //         ** If the source page size is smaller than the destination page size,
                //         ** round up. In this case the call to sqlite3OsTruncate() below will
                //         ** fix the size of the file. However it is important to call
                //         ** sqlite3PagerTruncateImage() here so that any pages in the
                //         ** destination file that lie beyond the nDestTruncate page mark are
                //         ** journalled by PagerCommitPhaseOne() before they are destroyed
                //         ** by the file truncation.
                //         */
                0 as i32;
                0 as i32;
                if pgszSrc < pgszDest {
                    let mut ratio: i32 = pgszDest / pgszSrc;
                    nDestTruncate = (nSrcPage + ratio - (1 as i32)) / ratio;
                    if nDestTruncate
                        == ((((unsafe { sqlite3PendingByte }) as u32)
                            / unsafe { (*unsafe { (*unsafe { (*p).pDest }).pBt }).pageSize })
                        .wrapping_add((1 as i32) as u32) as i32)
                    {
                        let __v608: i32 = nDestTruncate;
                        let __v609: i32 = __v608 - (1 as i32);
                        nDestTruncate = __v609;
                    }
                } else {
                    nDestTruncate = nSrcPage * (pgszSrc / pgszDest);
                }
                0 as i32;
                if pgszSrc < pgszDest {
                    // /* If the source page-size is smaller than the destination page-size,
                    //           ** two extra things may need to happen:
                    //           **
                    //           **   * The destination may need to be truncated, and
                    //           **
                    //           **   * Data stored on the pages immediately following the
                    //           **     pending-byte page in the source database may need to be
                    //           **     copied into the destination database.
                    //           */
                    let mut iSize: i64 = (pgszSrc as i64) * (nSrcPage as i64);
                    let mut pFile: *mut sqlite3_file = unsafe { sqlite3PagerFile(pDestPager) };
                    let mut iPg: u32 = 0 as u32;
                    let mut nDstPage: i32 = 0 as i32;
                    let mut iOff: i64 = 0 as i64;
                    let mut iEnd: i64 = 0 as i64;
                    0 as i32;
                    0 as i32;
                    // /* This block ensures that all data required to recreate the original
                    //           ** database has been stored in the journal for pDestPager and the
                    //           ** journal synced to disk. So at this point we may safely modify
                    //           ** the database file in any way, knowing that if a power failure
                    //           ** occurs, the original database will be reconstructed from the
                    //           ** journal file.  */
                    unsafe { sqlite3PagerPagecount(pDestPager, std::ptr::addr_of_mut!(nDstPage)) };
                    iPg = nDestTruncate as u32;
                    '__slate_break_588: while rc == (0 as i32) && iPg <= (nDstPage as u32) {
                        if iPg
                            != (((unsafe { sqlite3PendingByte }) as u32)
                                / unsafe { (*unsafe { (*unsafe { (*p).pDest }).pBt }).pageSize })
                            .wrapping_add((1 as i32) as u32)
                        {
                            let mut pPg: *mut PgHdr = unsafe { std::mem::zeroed() };
                            rc = unsafe {
                                sqlite3PagerGet(
                                    pDestPager,
                                    iPg,
                                    std::ptr::addr_of_mut!(pPg),
                                    0 as i32,
                                )
                            };
                            if rc == (0 as i32) {
                                rc = unsafe { sqlite3PagerWrite(pPg) };
                                unsafe { sqlite3PagerUnref(pPg) };
                            }
                        }
                        let __v610: u32 = iPg;
                        let __v611: u32 = __v610.wrapping_add((1 as i32) as u32);
                        iPg = __v611;
                    }
                    if rc == (0 as i32) {
                        rc = unsafe {
                            sqlite3PagerCommitPhaseOne(pDestPager, std::ptr::null::<i8>(), 1 as i32)
                        };
                    }
                    // /* Write the extra pages and truncate the database file as required */
                    iEnd = if (((unsafe { sqlite3PendingByte }) + pgszDest) as i64) < iSize {
                        ((unsafe { sqlite3PendingByte }) + pgszDest) as i64
                    } else {
                        iSize
                    };
                    iOff = ((unsafe { sqlite3PendingByte }) + pgszSrc) as i64;
                    '__slate_break_589: loop {
                        if !(rc == (0 as i32) && iOff < iEnd) {
                            break;
                        }
                        let mut pSrcPg: *mut PgHdr = std::ptr::null_mut::<PgHdr>();
                        let mut iSrcPg: u32 =
                            ((iOff / (pgszSrc as i64) + ((1 as i32) as i64)) as i32) as u32;
                        rc = unsafe {
                            sqlite3PagerGet(
                                pSrcPager,
                                iSrcPg,
                                std::ptr::addr_of_mut!(pSrcPg),
                                0 as i32,
                            )
                        };
                        if rc == (0 as i32) {
                            let mut zData: *mut u8 =
                                (unsafe { sqlite3PagerGetData(pSrcPg) }) as *mut u8;
                            rc =
                                unsafe { sqlite3OsWrite(pFile, zData as *const (), pgszSrc, iOff) };
                        }
                        unsafe { sqlite3PagerUnref(pSrcPg) };
                        let __v612: i64 = iOff;
                        let __v613: i64 = __v612 + (pgszSrc as i64);
                        iOff = __v613;
                    }
                    if rc == (0 as i32) {
                        rc = backupTruncateFile(pFile, iSize);
                    }
                    // /* Sync the database file to disk. */
                    if rc == (0 as i32) {
                        rc = unsafe { sqlite3PagerSync(pDestPager, std::ptr::null::<i8>()) };
                    }
                } else {
                    unsafe { sqlite3PagerTruncateImage(pDestPager, nDestTruncate as u32) };
                    rc = unsafe {
                        sqlite3PagerCommitPhaseOne(pDestPager, std::ptr::null::<i8>(), 0 as i32)
                    };
                }
                // /* Finish committing the transaction to the destination database. */
                let __v614: bool;
                if (0 as i32) == rc {
                    let __v615: i32 =
                        unsafe { sqlite3BtreeCommitPhaseTwo(unsafe { (*p).pDest }, 0 as i32) };
                    rc = __v615;
                    __v614 = (0 as i32) == __v615;
                } else {
                    __v614 = false as bool;
                }
                if __v614 {
                    rc = 101 as i32;
                }
            }
        }
        // /* If bCloseTrans is true, then this function opened a read transaction
        //     ** on the source database. Close the read transaction here. There is
        //     ** no need to check the return values of the btree methods here, as
        //     ** "committing" a read-only transaction cannot fail.
        //     */
        if bCloseTrans != (0 as i32) {
            {}
            unsafe { sqlite3BtreeCommitPhaseOne(unsafe { (*p).pSrc }, std::ptr::null::<i8>()) };
            unsafe { sqlite3BtreeCommitPhaseTwo(unsafe { (*p).pSrc }, 0 as i32) };
            0 as i32;
        }
        if rc == (10 as i32) | (12 as i32) << (8 as i32) {
            rc = 7 as i32;
        }
        unsafe {
            (*p).rc = rc;
        }
    }
    if (unsafe { (*p).pDestDb }) != std::ptr::null_mut::<sqlite3>() {
        unsafe { sqlite3_mutex_leave(unsafe { (*unsafe { (*p).pDestDb }).mutex }) };
    }
    unsafe { sqlite3BtreeLeave(unsafe { (*p).pSrc }) };
    unsafe { sqlite3_mutex_leave(unsafe { (*unsafe { (*p).pSrcDb }).mutex }) };
    return rc;
}

// /*
// ** Release all resources associated with an sqlite3_backup* handle.
// */
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.backup.sqlite3_backup_finish")]
extern "C-unwind" fn sqlite3_backup_finish(mut p: *mut sqlite3_backup) -> i32 {
    // /* Ptr to head of pagers backup list */
    let mut pp: *mut *mut sqlite3_backup = unsafe { std::mem::zeroed() };
    // /* Source database connection */
    let mut pSrcDb: *mut sqlite3 = unsafe { std::mem::zeroed() };
    // /* Value to return */
    let mut rc: i32 = 0 as i32;
    // /* Enter the mutexes */
    if p == std::ptr::null_mut::<sqlite3_backup>() {
        return 0 as i32;
    }
    pSrcDb = unsafe { (*p).pSrcDb };
    unsafe { sqlite3_mutex_enter(unsafe { (*pSrcDb).mutex }) };
    unsafe { sqlite3BtreeEnter(unsafe { (*p).pSrc }) };
    if (unsafe { (*p).pDestDb }) != std::ptr::null_mut::<sqlite3>() {
        unsafe { sqlite3_mutex_enter(unsafe { (*unsafe { (*p).pDestDb }).mutex }) };
    }
    // /* Detach this backup from the source pager. */
    if (unsafe { (*p).pDestDb }) != std::ptr::null_mut::<sqlite3>() {
        let __v616: *mut Btree = unsafe { (*p).pSrc };
        let __v617: i32 = unsafe { (*__v616).nBackup };
        let __v618: i32 = __v617 - (1 as i32);
        unsafe {
            (*__v616).nBackup = __v618;
        }
    }
    if (unsafe { (*p).isAttached }) != (0 as i32) {
        pp = unsafe { sqlite3PagerBackupPtr(unsafe { sqlite3BtreePager(unsafe { (*p).pSrc }) }) };
        0 as i32;
        '__slate_break_590: while (unsafe { *pp }) != p {
            pp = unsafe { std::ptr::addr_of_mut!((*unsafe { *pp }).pNext) };
            0 as i32;
        }
        unsafe {
            *pp = unsafe { (*p).pNext };
        }
    }
    // /* If a transaction is still open on the Btree, roll it back. */
    if (unsafe { (*p).pDest }) != std::ptr::null_mut::<Btree>() {
        unsafe { sqlite3BtreeRollback(unsafe { (*p).pDest }, 0 as i32, 0 as i32) };
    }
    // /* Set the error code of the destination database handle. */
    rc = if (unsafe { (*p).rc }) == (101 as i32) {
        0 as i32
    } else {
        unsafe { (*p).rc }
    };
    if (unsafe { (*p).pDestDb }) != std::ptr::null_mut::<sqlite3>() {
        unsafe { sqlite3Error(unsafe { (*p).pDestDb }, rc) };
        // /* Exit the mutexes and free the backup context structure. */
        unsafe { sqlite3LeaveMutexAndCloseZombie(unsafe { (*p).pDestDb }) };
    }
    unsafe { sqlite3BtreeLeave(unsafe { (*p).pSrc }) };
    if (unsafe { (*p).pDestDb }) != std::ptr::null_mut::<sqlite3>() {
        // /* EVIDENCE-OF: R-64852-21591 The sqlite3_backup object is created by a
        //     ** call to sqlite3_backup_init() and is destroyed by a call to
        //     ** sqlite3_backup_finish(). */
        unsafe { sqlite3_free(p as *mut ()) };
    }
    unsafe { sqlite3LeaveMutexAndCloseZombie(pSrcDb) };
    return rc;
}

// /*
// ** Return the number of pages still to be backed up as of the most recent
// ** call to sqlite3_backup_step().
// */
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.backup.sqlite3_backup_remaining")]
extern "C-unwind" fn sqlite3_backup_remaining(mut p: *mut sqlite3_backup) -> i32 {
    return (unsafe { (*p).nRemaining }) as i32;
}

// /*
// ** Return the total number of pages in the source database as of the most
// ** recent call to sqlite3_backup_step().
// */
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.backup.sqlite3_backup_pagecount")]
extern "C-unwind" fn sqlite3_backup_pagecount(mut p: *mut sqlite3_backup) -> i32 {
    return (unsafe { (*p).nPagecount }) as i32;
}

// /*
// ** Copy the complete content of pBtFrom into pBtTo.  A transaction
// ** must be active for both files.
// **
// ** The size of file pTo may be reduced by this operation. If anything
// ** goes wrong, the transaction on pTo is rolled back. If successful, the
// ** transaction is committed before returning.
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3BtreeCopyFile(mut pTo: *mut Btree, mut pFrom: *mut Btree) -> i32 {
    let mut __slate_storage_621: std::mem::MaybeUninit<u16> = std::mem::MaybeUninit::uninit();
    let __slate_slot_621: *mut u16 = std::ptr::addr_of_mut!(__slate_storage_621) as *mut u16;
    let mut __slate_storage_620: std::mem::MaybeUninit<u16> = std::mem::MaybeUninit::uninit();
    let __slate_slot_620: *mut u16 = std::ptr::addr_of_mut!(__slate_storage_620) as *mut u16;
    let mut __slate_storage_619: std::mem::MaybeUninit<*mut BtShared> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_619: *mut *mut BtShared =
        std::ptr::addr_of_mut!(__slate_storage_619) as *mut *mut BtShared;
    let mut __slate_storage_479: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_479: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_479) as *mut i64;
    let mut __slate_storage_478: std::mem::MaybeUninit<sqlite3_backup> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_478: *mut sqlite3_backup =
        std::ptr::addr_of_mut!(__slate_storage_478) as *mut sqlite3_backup;
    let mut __slate_storage_477: std::mem::MaybeUninit<*mut sqlite3_file> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_477: *mut *mut sqlite3_file =
        std::ptr::addr_of_mut!(__slate_storage_477) as *mut *mut sqlite3_file;
    let mut __slate_storage_476: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_476: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_476) as *mut i32;
    unsafe {
        '__join_0: {
            // /* File descriptor for database pTo */
            unsafe { sqlite3BtreeEnter(pTo) };
            unsafe { sqlite3BtreeEnter(pFrom) };
            0 as i32;
            *__slate_slot_477 = unsafe { sqlite3PagerFile(unsafe { sqlite3BtreePager(pTo) }) };
            if (unsafe { (*(*__slate_slot_477)).pMethods })
                != std::ptr::null::<sqlite3_io_methods>()
            {
                std::ptr::write(
                    __slate_slot_479,
                    ((unsafe { sqlite3BtreeGetPageSize(pFrom) }) as i64)
                        * (((unsafe { sqlite3BtreeLastPage(pFrom) }) as u64) as i64),
                );
                *__slate_slot_476 = unsafe {
                    sqlite3OsFileControl(
                        *__slate_slot_477,
                        11 as i32,
                        std::ptr::addr_of_mut!(*__slate_slot_479) as *mut (),
                    )
                };
                if *__slate_slot_476 == (12 as i32) {
                    *__slate_slot_476 = 0 as i32;
                }
                if *__slate_slot_476 != (0 as i32) {
                    break '__join_0;
                }
            }
            // /* Set up an sqlite3_backup object. sqlite3_backup.pDestDb must be set
            //   ** to 0. This is used by the implementations of sqlite3_backup_step()
            //   ** and sqlite3_backup_finish() to detect that they are being called
            //   ** from this function, not directly by the user.
            //   */
            unsafe {
                memset(
                    std::ptr::addr_of_mut!(*__slate_slot_478) as *mut (),
                    0 as i32,
                    80 as u64,
                )
            };
            (*__slate_slot_478).pSrcDb = unsafe { (*pFrom).db };
            (*__slate_slot_478).pSrc = pFrom;
            (*__slate_slot_478).pDest = pTo;
            (*__slate_slot_478).iNext = (1 as i32) as u32;
            // /* 0x7FFFFFFF is the hard limit for the number of pages in a database
            //   ** file. By passing this as the number of pages to copy to
            //   ** sqlite3_backup_step(), we can guarantee that the copy finishes
            //   ** within a single call (unless an error occurs). The assert() statement
            //   ** checks this assumption - (p->rc) should be set to either SQLITE_DONE
            //   ** or an error code.  */
            sqlite3_backup_step(std::ptr::addr_of_mut!(*__slate_slot_478), 2147483647 as i32);
            0 as i32;
            *__slate_slot_476 = sqlite3_backup_finish(std::ptr::addr_of_mut!(*__slate_slot_478));
            if *__slate_slot_476 == (0 as i32) {
                std::ptr::write(__slate_slot_619, unsafe { (*pTo).pBt });
                std::ptr::write(__slate_slot_620, unsafe { (*(*__slate_slot_619)).btsFlags });
                std::ptr::write(
                    __slate_slot_621,
                    ((((*__slate_slot_620 as u32) as i32) & !(2 as i32)) as i16) as u16,
                );
                unsafe {
                    (*(*__slate_slot_619)).btsFlags = *__slate_slot_621;
                }
            } else {
                unsafe {
                    sqlite3PagerClearCache(unsafe { sqlite3BtreePager((*__slate_slot_478).pDest) })
                };
            }
            0 as i32;
        }
        unsafe { sqlite3BtreeLeave(pFrom) };
        unsafe { sqlite3BtreeLeave(pTo) };
        return *__slate_slot_476;
    }
    return unsafe { std::mem::zeroed() };
}

// /*
// ** Restart the backup process. This is called when the pager layer
// ** detects that the database has been modified by an external database
// ** connection. In this case there is no way of knowing which of the
// ** pages that have been copied into the destination database are still
// ** valid and which are not, so the entire process needs to be restarted.
// **
// ** It is assumed that the mutex associated with the BtShared object
// ** corresponding to the source database is held when this function is
// ** called.
// */
// /* SQLITE_OMIT_VACUUM */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3BackupRestart(mut pBackup: *mut sqlite3_backup) {
    // /* Iterator variable */
    let mut p: *mut sqlite3_backup = unsafe { std::mem::zeroed() };
    p = pBackup;
    '__slate_break_592: while p != std::ptr::null_mut::<sqlite3_backup>() {
        0 as i32;
        unsafe {
            (*p).iNext = (1 as i32) as u32;
        }
        p = unsafe { (*p).pNext };
    }
}

#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3BackupUpdate(
    mut pBackup: *mut sqlite3_backup,
    mut iPage: u32,
    mut aData: *const u8,
) {
    if pBackup != std::ptr::null_mut::<sqlite3_backup>() {
        backupUpdate(pBackup, iPage, aData);
    }
}

// /*
// ** 2009 January 28
// **
// ** The author disclaims copyright to this source code.  In place of
// ** a legal notice, here is a blessing:
// **
// **    May you do good and not evil.
// **    May you find forgiveness for yourself and forgive others.
// **    May you share freely, never taking more than you give.
// **
// *************************************************************************
// ** This file contains the implementation of the sqlite3_backup_XXX()
// ** API functions and the related features.
// */
// /*
// ** Structure allocated for each backup operation.
// */
// /* Destination database handle */
// /* Destination b-tree file */
// /* Original schema cookie in destination */
// /* True once a write-transaction is open on pDest */
// /* Page number of the next source page to copy */
// /* Source database handle */
// /* Source b-tree file */
// /* Backup process error code */
// /* These two variables are set by every call to backup_step(). They are
//   ** read by calls to backup_remaining() and backup_pagecount().
//   */
// /* Number of pages left to copy */
// /* Total number of pages to copy */
// /* True once backup has been registered with pager */
// /* Next backup associated with source pager */
// /*
// ** THREAD SAFETY NOTES:
// **
// **   Once it has been created using backup_init(), a single sqlite3_backup
// **   structure may be accessed via two groups of thread-safe entry points:
// **
// **     * Via the sqlite3_backup_XXX() API function backup_step() and
// **       backup_finish(). Both these functions obtain the source database
// **       handle mutex and the mutex associated with the source BtShared
// **       structure, in that order.
// **
// **     * Via the BackupUpdate() and BackupRestart() functions, which are
// **       invoked by the pager layer to report various state changes in
// **       the page cache associated with the source database. The mutex
// **       associated with the source database BtShared structure will always
// **       be held when either of these functions are invoked.
// **
// **   The other sqlite3_backup_XXX() API functions, backup_remaining() and
// **   backup_pagecount() are not thread-safe functions. If they are called
// **   while some other thread is calling backup_step() or backup_finish(),
// **   the values returned may be invalid. There is no way for a call to
// **   BackupUpdate() or BackupRestart() to interfere with backup_remaining()
// **   or backup_pagecount().
// **
// **   Depending on the SQLite configuration, the database handles and/or
// **   the Btree objects may have their own mutexes that require locking.
// **   Non-sharable Btrees (in-memory databases for example), do not have
// **   associated mutexes.
// */
// /*
// ** Return a pointer corresponding to database zDb (i.e. "main", "temp")
// ** in connection handle pDb. If such a database cannot be found, return
// ** a NULL pointer and write an error message to pErrorDb.
// **
// ** If the "temp" database is requested, it may need to be opened by this
// ** function. If an error occurs while doing so, return 0 and write an
// ** error message to pErrorDb.
// */
fn findBtree(mut pErrorDb: *mut sqlite3, mut pDb: *mut sqlite3, mut zDb: *const i8) -> *mut Btree {
    let mut i: i32 = 0 as i32;
    0 as i32;
    i = unsafe { sqlite3FindDbName(pDb, zDb) };
    if i == (1 as i32) {
        let mut sParse: Parse = unsafe { std::mem::zeroed() };
        let mut rc: i32 = 0 as i32;
        unsafe { sqlite3ParseObjectInit(std::ptr::addr_of_mut!(sParse), pDb) };
        if (unsafe { sqlite3OpenTempDatabase(std::ptr::addr_of_mut!(sParse)) }) != (0 as i32) {
            unsafe {
                sqlite3ErrorWithMsg(
                    pErrorDb,
                    sParse.rc,
                    (b"%s\0".as_ptr() as *mut i8) as *const i8,
                    sParse.zErrMsg,
                )
            };
            rc = 1 as i32;
        }
        unsafe { sqlite3DbFree(pErrorDb, sParse.zErrMsg as *mut ()) };
        unsafe { sqlite3ParseObjectReset(std::ptr::addr_of_mut!(sParse)) };
        if rc != (0 as i32) {
            return std::ptr::null_mut::<Btree>();
        }
    }
    if i < (0 as i32) {
        unsafe {
            sqlite3ErrorWithMsg(
                pErrorDb,
                1 as i32,
                (b"unknown database %s\0".as_ptr() as *mut i8) as *const i8,
                zDb,
            )
        };
        return std::ptr::null_mut::<Btree>();
    }
    return unsafe { (*unsafe { unsafe { (*pDb).aDb }.offset(i as isize) }).pBt };
}

// /*
// ** Attempt to set the page size of the destination to match the page size
// ** of the source.
// */
fn setDestPgsz(mut pDest: *mut Btree, mut pSrc: *mut Btree) -> i32 {
    return unsafe {
        sqlite3BtreeSetPageSize(
            pDest,
            unsafe { sqlite3BtreeGetPageSize(pSrc) },
            0 as i32,
            0 as i32,
        )
    };
}

// /*
// ** Check that there is no open read-transaction on the b-tree passed as the
// ** second argument. If there is not, return SQLITE_OK. Otherwise, if there
// ** is an open read-transaction, return SQLITE_ERROR and leave an error
// ** message in database handle db.
// */
fn checkReadTransaction(mut db: *mut sqlite3, mut p: *mut Btree) -> i32 {
    if (unsafe { sqlite3BtreeTxnState(p) }) != (0 as i32) {
        unsafe {
            sqlite3ErrorWithMsg(
                db,
                1 as i32,
                (b"destination database is in use\0".as_ptr() as *mut i8) as *const i8,
            )
        };
        return 1 as i32;
    }
    return 0 as i32;
}

// /* Database to write to */
// /* Name of database within pDestDb */
// /* Database connection to read from */
// /* Name of database within pSrcDb */
// /*
// ** Argument rc is an SQLite error code. Return true if this error is
// ** considered fatal if encountered during a backup operation. All errors
// ** are considered fatal except for SQLITE_BUSY and SQLITE_LOCKED.
// */
fn isFatalError(mut rc: i32) -> i32 {
    return (rc != (0 as i32) && rc != (5 as i32) && rc != (6 as i32)) as i32;
}

// /*
// ** Parameter zSrcData points to a buffer containing the data for
// ** page iSrcPg from the source database. Copy this data into the
// ** destination database.
// */
fn backupOnePage(
    mut p: *mut sqlite3_backup,
    mut iSrcPg: u32,
    mut zSrcData: *const u8,
    mut bUpdate: i32,
) -> i32 {
    let mut pDestPager: *mut Pager = unsafe { sqlite3BtreePager(unsafe { (*p).pDest }) };
    let mut nSrcPgsz: i32 = unsafe { sqlite3BtreeGetPageSize(unsafe { (*p).pSrc }) };
    let mut nDestPgsz: i32 = unsafe { sqlite3BtreeGetPageSize(unsafe { (*p).pDest }) };
    let mut nCopy: i32 = if nSrcPgsz < nDestPgsz {
        nSrcPgsz
    } else {
        nDestPgsz
    };
    let mut iEnd: i64 = ((iSrcPg as u64) as i64) * (nSrcPgsz as i64);
    let mut rc: i32 = 0 as i32;
    let mut iOff: i64 = 0 as i64;
    0 as i32;
    0 as i32;
    0 as i32;
    0 as i32;
    0 as i32;
    0 as i32;
    // /* This loop runs once for each destination page spanned by the source
    //   ** page. For each iteration, variable iOff is set to the byte offset
    //   ** of the destination page.
    //   */
    iOff = iEnd - (nSrcPgsz as i64);
    '__slate_break_586: loop {
        if !(rc == (0 as i32) && iOff < iEnd) {
            break;
        }
        let mut pDestPg: *mut PgHdr = std::ptr::null_mut::<PgHdr>();
        let mut iDest: u32 =
            (((iOff / (nDestPgsz as i64)) as i32) as u32).wrapping_add((1 as i32) as u32);
        if iDest
            == (((unsafe { sqlite3PendingByte }) as u32)
                / unsafe { (*unsafe { (*unsafe { (*p).pDest }).pBt }).pageSize })
            .wrapping_add((1 as i32) as u32)
        {
        } else {
            let __v624: i32 = unsafe {
                sqlite3PagerGet(pDestPager, iDest, std::ptr::addr_of_mut!(pDestPg), 0 as i32)
            };
            rc = __v624;
            let __v625: bool;
            if (0 as i32) == __v624 {
                let __v626: i32 = unsafe { sqlite3PagerWrite(pDestPg) };
                rc = __v626;
                __v625 = (0 as i32) == __v626;
            } else {
                __v625 = false as bool;
            }
            if __v625 {
                let mut zIn: *const u8 =
                    unsafe { zSrcData.offset((iOff % (nSrcPgsz as i64)) as isize) };
                let mut zDestData: *mut u8 = (unsafe { sqlite3PagerGetData(pDestPg) }) as *mut u8;
                let mut zOut: *mut u8 =
                    unsafe { zDestData.offset((iOff % (nDestPgsz as i64)) as isize) };
                // /* Copy the data from the source page into the destination page.
                //       ** Then clear the Btree layer MemPage.isInit flag. Both this module
                //       ** and the pager code use this trick (clearing the first byte
                //       ** of the page 'extra' space to invalidate the Btree layers
                //       ** cached parse of the page). MemPage.isInit is marked
                //       ** "MUST BE FIRST" for this purpose.
                //       */
                unsafe { memcpy(zOut as *mut (), zIn as *const (), (nCopy as i64) as u64) };
                unsafe {
                    *unsafe {
                        ((unsafe { sqlite3PagerGetExtra(pDestPg) }) as *mut u8)
                            .offset((0 as i32) as isize)
                    } = ((0 as i32) as i8) as u8;
                }
                if iOff == ((0 as i32) as i64) && bUpdate == (0 as i32) {
                    unsafe {
                        sqlite3Put4byte(unsafe { zOut.offset((28 as i32) as isize) }, unsafe {
                            sqlite3BtreeLastPage(unsafe { (*p).pSrc })
                        })
                    };
                }
            }
            unsafe { sqlite3PagerUnref(pDestPg) };
        }
        let __v622: i64 = iOff;
        let __v623: i64 = __v622 + (nDestPgsz as i64);
        iOff = __v623;
    }
    return rc;
}

// /* Backup handle */
// /* Source database page to backup */
// /* Source database page data */
// /* True for an update, false otherwise */
// /*
// ** If pFile is currently larger than iSize bytes, then truncate it to
// ** exactly iSize bytes. If pFile is not larger than iSize bytes, then
// ** this function is a no-op.
// **
// ** Return SQLITE_OK if everything is successful, or an SQLite error
// ** code if an error occurs.
// */
fn backupTruncateFile(mut pFile: *mut sqlite3_file, mut iSize: i64) -> i32 {
    let mut iCurrent: i64 = 0 as i64;
    let mut rc: i32 = unsafe { sqlite3OsFileSize(pFile, std::ptr::addr_of_mut!(iCurrent)) };
    if rc == (0 as i32) && iCurrent > iSize {
        rc = unsafe { sqlite3OsTruncate(pFile, iSize) };
    }
    return rc;
}

// /*
// ** Register this backup object with the associated source pager for
// ** callbacks when pages are changed or the cache invalidated.
// */
fn attachBackupObject(mut p: *mut sqlite3_backup) {
    let mut pp: *mut *mut sqlite3_backup = unsafe { std::mem::zeroed() };
    0 as i32;
    pp = unsafe { sqlite3PagerBackupPtr(unsafe { sqlite3BtreePager(unsafe { (*p).pSrc }) }) };
    unsafe {
        (*p).pNext = unsafe { *pp };
    }
    unsafe {
        *pp = p;
    }
    unsafe {
        (*p).isAttached = 1 as i32;
    }
}

// /*
// ** This function is called after the contents of page iPage of the
// ** source database have been modified. If page iPage has already been
// ** copied into the destination database, then the data written to the
// ** destination is now invalidated. The destination copy of iPage needs
// ** to be updated with the new data before the backup operation is
// ** complete.
// **
// ** It is assumed that the mutex associated with the BtShared object
// ** corresponding to the source database is held when this function is
// ** called.
// */
fn backupUpdate(mut p: *mut sqlite3_backup, mut iPage: u32, mut aData: *const u8) {
    0 as i32;
    '__slate_break_591: loop {
        0 as i32;
        if !(isFatalError(unsafe { (*p).rc }) != (0 as i32)) && iPage < unsafe { (*p).iNext } {
            // /* The backup process p has already copied page iPage. But now it
            //       ** has been modified by a transaction on the source pager. Copy
            //       ** the new data into the backup.
            //       */
            let mut rc: i32 = 0 as i32;
            0 as i32;
            unsafe { sqlite3_mutex_enter(unsafe { (*unsafe { (*p).pDestDb }).mutex }) };
            rc = backupOnePage(p, iPage, aData, 1 as i32);
            unsafe { sqlite3_mutex_leave(unsafe { (*unsafe { (*p).pDestDb }).mutex }) };
            0 as i32;
            if rc != (0 as i32) {
                unsafe {
                    (*p).rc = rc;
                }
            }
        }
        let __v627: *mut sqlite3_backup = unsafe { (*p).pNext };
        p = __v627;
        if !(__v627 != std::ptr::null_mut::<sqlite3_backup>()) {
            break;
        }
    }
}
