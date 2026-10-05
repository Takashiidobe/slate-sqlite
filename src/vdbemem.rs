unsafe extern "C" {
    fn sqlite3_free(__v600: *mut ());
    fn sqlite3_msize(__v601: *mut ()) -> u64;
    fn sqlite3_str_appendf(__v602: *mut sqlite3_str, zFormat: *const i8, ...);
    fn memcpy(__dest: *mut (), __src: *const (), __n: u64) -> *mut ();
    fn memset(__s: *mut (), __c: i32, __n: u64) -> *mut ();
    fn memcmp(__s1: *const (), __s2: *const (), __n: u64) -> i32;
    fn strlen(__s: *const i8) -> u64;
    fn sqlite3BtreePayload(__v614: *mut BtCursor, offset: u32, amt: u32, __v617: *mut ()) -> i32;
    fn sqlite3BtreePayloadFetch(__v618: *mut BtCursor, pAmt: *mut u32) -> *const ();
    fn sqlite3BtreeMaxRecordSize(__v620: *mut BtCursor) -> i64;
    fn sqlite3CorruptError(__v624: i32) -> i32;
    fn sqlite3Strlen30(__v625: *const i8) -> i32;
    fn sqlite3DbMallocZero(__v626: *mut sqlite3, __v627: u64) -> *mut ();
    fn sqlite3DbMallocRaw(__v628: *mut sqlite3, __v629: u64) -> *mut ();
    fn sqlite3DbStrNDup(__v630: *mut sqlite3, __v631: *const i8, __v632: u64) -> *mut i8;
    fn sqlite3Realloc(__v633: *mut (), __v634: u64) -> *mut ();
    fn sqlite3DbReallocOrFree(__v635: *mut sqlite3, __v636: *mut (), __v637: u64) -> *mut ();
    fn sqlite3DbFree(__v638: *mut sqlite3, __v639: *mut ());
    fn sqlite3DbFreeNN(__v640: *mut sqlite3, __v641: *mut ());
    fn sqlite3DbMallocSize(__v642: *mut sqlite3, __v643: *const ()) -> i32;
    fn sqlite3IsNaN(__v644: f64) -> i32;
    fn sqlite3MPrintf(__v645: *mut sqlite3, __v646: *const i8, ...) -> *mut i8;
    fn sqlite3ErrorToParser(__v647: *mut sqlite3, __v648: i32) -> i32;
    fn sqlite3RowSetInit(__v649: *mut sqlite3) -> *mut RowSet;
    fn sqlite3RowSetDelete(__v650: *mut ());
    fn sqlite3RowSetClear(__v651: *mut ());
    fn sqlite3Int64ToText(__v655: i64, __v656: *mut i8) -> i32;
    fn sqlite3AtoF(z: *const i8, __v658: *mut f64) -> i32;
    fn sqlite3Atoi64(__v659: *const i8, __v660: *mut i64, __v661: i32, __v662: u8) -> i32;
    fn sqlite3DecOrHexToI64(__v663: *const i8, __v664: *mut i64) -> i32;
    fn sqlite3HexToBlob(__v665: *mut sqlite3, z: *const i8, n: i32) -> *mut ();
    fn sqlite3ValueApplyAffinity(__v687: *mut sqlite3_value, __v688: u8, __v689: u8);
    fn sqlite3AffinityType(__v690: *const i8, __v691: *mut Column) -> i8;
    fn sqlite3OomFault(__v693: *mut sqlite3) -> *mut ();
    fn sqlite3RCStrUnref(__v694: *mut ());
    fn sqlite3StrAccumInit(
        __v695: *mut sqlite3_str,
        __v696: *mut sqlite3,
        __v697: *mut i8,
        __v698: i32,
        __v699: i32,
    );
    fn sqlite3VdbeMemTranslate(__v772: *mut sqlite3_value, __v773: u8) -> i32;
    fn sqlite3VdbeMemHandleBom(pMem: *mut sqlite3_value) -> i32;
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
    trace: __SlateRecord168,
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
    u1: __SlateRecord169,
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
    __slate_bits_0: __slate_bits::__SlateBits68U0,
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
    u: __SlateRecord180,
    pLeft: *mut Expr,
    pRight: *mut Expr,
    x: __SlateRecord181,
    nHeight: i32,
    iTable: i32,
    iColumn: i16,
    iAgg: i16,
    w: __SlateRecord182,
    pAggInfo: *mut AggInfo,
    y: __SlateRecord183,
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
    u: __SlateRecord170,
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
    __slate_bits_0: __slate_bits::__SlateBits92U0,
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
    __slate_bits_0: __slate_bits::__SlateBits104U0,
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
    u1: __SlateRecord196,
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
struct RowSet {}

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
    fg: __SlateRecord190,
    iCursor: i32,
    colUsed: u64,
    u1: __SlateRecord191,
    u2: __SlateRecord192,
    u3: __SlateRecord193,
    u4: __SlateRecord194,
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
    u: __SlateRecord171,
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
struct UnpackedRecord {
    pKeyInfo: *mut KeyInfo,
    aMem: *mut sqlite3_value,
    u: __SlateRecord176,
    n: i32,
    nField: u16,
    default_rc: i8,
    errCode: u8,
    r1: i8,
    r2: i8,
    eqSeen: u8,
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
    __slate_bits_0: __slate_bits::__SlateBits156U0,
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
    __slate_bits_0: __slate_bits::__SlateBits167U0,
    azInit: *mut *const i8,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord168 {
    xLegacy: Option<unsafe extern "C-unwind" fn(*mut (), *const i8)>,
    xV2: Option<unsafe extern "C-unwind" fn(u32, *mut (), *mut (), *mut ()) -> i32>,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord169 {
    isInterrupted: i32,
    notUsed1: f64,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord170 {
    pHash: *mut FuncDef,
    pDestructor: *mut FuncDestructor,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord171 {
    tab: __SlateRecord172,
    view: __SlateRecord173,
    vtab: __SlateRecord174,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord172 {
    addColOffset: i32,
    pFKey: *mut FKey,
    pDfltList: *mut ExprList,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord173 {
    pSelect: *mut Select,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord174 {
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
union __SlateRecord176 {
    z: *mut i8,
    i: i64,
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
union __SlateRecord180 {
    zToken: *mut i8,
    iValue: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord181 {
    pList: *mut ExprList,
    pSelect: *mut Select,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord182 {
    iJoin: i32,
    iOfst: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord183 {
    pTab: *mut Table,
    pWin: *mut Window,
    nReg: i32,
    sub: __SlateRecord184,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord184 {
    iAddr: i32,
    regReturn: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct ExprList_item {
    pExpr: *mut Expr,
    zEName: *mut i8,
    fg: __SlateRecord186,
    u: __SlateRecord187,
}

#[repr(C, align(4))]
#[derive(Clone, Copy)]
struct __SlateRecord186 {
    sortFlags: u8,
    __slate_bits_0: __slate_bits::__SlateBits186U0,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord187 {
    x: __SlateRecord188,
    iConstExprReg: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord188 {
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
struct __SlateRecord190 {
    jointype: u8,
    __slate_bits_0: __slate_bits::__SlateBits190U0,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord191 {
    zIndexedBy: *mut i8,
    pFuncArg: *mut ExprList,
    nRow: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord192 {
    pIBIndex: *mut Index,
    pCteUse: *mut CteUse,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord193 {
    pOn: *mut Expr,
    pUsing: *mut IdList,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord194 {
    pSchema: *mut Schema,
    zDatabase: *mut i8,
    pSubq: *mut Subquery,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord196 {
    cr: __SlateRecord197,
    d: __SlateRecord198,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord197 {
    addrCrTab: i32,
    regRowid: i32,
    regRoot: i32,
    constraintName: Token,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord198 {
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
    __slate_bits_0: __slate_bits::__SlateBits207U0,
    seekHit: u16,
    ub: __SlateRecord209,
    seqCount: i64,
    cacheStatus: u32,
    seekResult: i32,
    pAltCursor: *mut VdbeCursor,
    uc: __SlateRecord210,
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
union __SlateRecord209 {
    pBtx: *mut Btree,
    aAltMap: *mut u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord210 {
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
// ** Context object passed by sqlite3Stat4ProbeSetValue() through to
// ** valueNew(). See comments above valueNew() for details.
// */
#[repr(C)]
#[derive(Clone, Copy)]
struct ValueNewStat4Ctx {
    pParse: *mut Parse,
    pIdx: *mut Index,
    ppRec: *mut *mut UnpackedRecord,
    iVal: i32,
}

mod __slate_bits {
    #[bitfields::bitfield([u8; 2], c_names = true, new = false, from_into_bits = false, from_traits = false, default = false, debug = false, builder = false, bit_ops = false)]
    pub struct __SlateBits104U0 {
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
    pub struct __SlateBits68U0 {
        #[bits(4)]
        pub notNull: u32,
        #[bits(4)]
        pub eCType: u32,
    }
    #[bitfields::bitfield([u8; 2], c_names = true, new = false, from_into_bits = false, from_traits = false, default = false, debug = false, builder = false, bit_ops = false)]
    pub struct __SlateBits92U0 {
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
    pub struct __SlateBits190U0 {
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
    pub struct __SlateBits186U0 {
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
    pub struct __SlateBits207U0 {
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
    pub struct __SlateBits156U0 {
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
    pub struct __SlateBits167U0 {
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
// ** Set the iIdx'th entry of array aMem[] to contain integer value val.
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3MemSetArrayInt64(
    mut aMem: *mut sqlite3_value,
    mut iIdx: i32,
    mut val: i64,
) {
    sqlite3VdbeMemSetInt64(unsafe { aMem.offset(iIdx as isize) }, val);
}

// /* Compare a floating point value to an integer.  Return true if the two
// ** values are the same within the precision of the floating point value.
// **
// ** This function assumes that i was obtained by assignment from r1.
// **
// ** For some versions of GCC on 32-bit machines, if you do the more obvious
// ** comparison of "r1==(double)i" you sometimes get an answer of false even
// ** though the r1 and (double)i values are bit-for-bit the same.
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3RealSameAsInt(mut r1: f64, mut i: i64) -> i32 {
    let mut r2: f64 = i as f64;
    return (r1 == 0.0f64
        || (unsafe {
            memcmp(
                std::ptr::addr_of_mut!(r1) as *const (),
                std::ptr::addr_of_mut!(r2) as *const (),
                8 as u64,
            )
        }) == (0 as i32)
            && i >= -(2251799813685248 as i64)
            && i < (2251799813685248 as i64)) as i32;
}

// /* Convert a floating point value to its closest integer.  Do so in
// ** a way that avoids 'outside the range of representable values' warnings
// ** from UBSAN.
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3RealToI64(mut r: f64) -> i64 {
    if r < -9.223372036854775e18f64 {
        return (-(1 as i32) as i64)
            - ((((4294967295 as u32) as u64) as i64)
                | ((2147483647 as i32) as i64) << (32 as i32));
    }
    if r > 9.223372036854775e18f64 {
        return (((4294967295 as u32) as u64) as i64) | ((2147483647 as i32) as i64) << (32 as i32);
    }
    return r as i64;
}

// /* This function is only available internally, it is not part of the
// ** external API. It works in a similar way to sqlite3_value_text(),
// ** except the data returned is in the encoding specified by the second
// ** parameter, which must be one of SQLITE_UTF16BE, SQLITE_UTF16LE or
// ** SQLITE_UTF8.
// **
// ** (2006-02-16:)  The enc value can be or-ed with SQLITE_UTF16_ALIGNED.
// ** If that is the case, then the result must be aligned on an even byte
// ** boundary.
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3ValueText(mut pVal: *mut sqlite3_value, mut enc: u8) -> *const () {
    if !(pVal != std::ptr::null_mut::<sqlite3_value>()) {
        return std::ptr::null::<()>();
    }
    0 as i32;
    0 as i32;
    0 as i32;
    if (((unsafe { (*pVal).flags }) as u32) as i32) & ((2 as i32) | (512 as i32))
        == (2 as i32) | (512 as i32)
        && (((unsafe { (*pVal).enc }) as u32) as i32) == ((enc as u32) as i32)
    {
        0 as i32;
        return (unsafe { (*pVal).z }) as *const ();
    }
    if (((unsafe { (*pVal).flags }) as u32) as i32) & (1 as i32) != (0 as i32) {
        return std::ptr::null::<()>();
    }
    return valueToText(pVal, enc);
}

// /* Return true if sqlit3_value object pVal is a string or blob value
// ** that uses the destructor specified in the second argument.
// **
// ** TODO:  Maybe someday promote this interface into a published API so
// ** that third-party extensions can get access to it?
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3ValueIsOfClass(
    mut pVal: *const sqlite3_value,
    mut xFree: Option<unsafe extern "C-unwind" fn(*mut ())>,
) -> i32 {
    if pVal != std::ptr::null::<sqlite3_value>()
        && (((unsafe { (*pVal).flags }) as u32) as i32) & ((2 as i32) | (16 as i32)) != (0 as i32)
        && (((unsafe { (*pVal).flags }) as u32) as i32) & (4096 as i32) != (0 as i32)
        && (unsafe { (*pVal).xDel }) == xFree
    {
        return 1 as i32;
    } else {
        return 0 as i32;
    }
    return unsafe { std::mem::zeroed() };
}

#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3ValueBytes(mut pVal: *mut sqlite3_value, mut enc: u8) -> i32 {
    let mut p: *mut sqlite3_value = pVal;
    0 as i32;
    if (((unsafe { (*p).flags }) as u32) as i32) & (2 as i32) != (0 as i32)
        && (((unsafe { (*pVal).enc }) as u32) as i32) == ((enc as u32) as i32)
    {
        return unsafe { (*p).n };
    }
    if (((unsafe { (*p).flags }) as u32) as i32) & (2 as i32) != (0 as i32)
        && ((enc as u32) as i32) != (1 as i32)
        && (((unsafe { (*pVal).enc }) as u32) as i32) != (1 as i32)
    {
        return unsafe { (*p).n };
    }
    if (((unsafe { (*p).flags }) as u32) as i32) & (16 as i32) != (0 as i32) {
        if (((unsafe { (*p).flags }) as u32) as i32) & (1024 as i32) != (0 as i32) {
            return (unsafe { (*p).n }) + unsafe { (*p).u.nZero };
        } else {
            return unsafe { (*p).n };
        }
    }
    if (((unsafe { (*p).flags }) as u32) as i32) & (1 as i32) != (0 as i32) {
        return 0 as i32;
    }
    return valueBytes(pVal, enc);
}

// /* The database connection */
// /* The expression to evaluate */
// /* Encoding to use */
// /* Affinity to use */
// /* Write the new value here */
// /*
// ** Change the string value of an sqlite3_value object
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3ValueSetStr(
    mut v: *mut sqlite3_value,
    mut n: i32,
    mut z: *const (),
    mut enc: u8,
    mut xDel: Option<unsafe extern "C-unwind" fn(*mut ())>,
) {
    if v != std::ptr::null_mut::<sqlite3_value>() {
        sqlite3VdbeMemSetStr(v, z as *const i8, n as i64, enc, xDel);
    }
}

#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3ValueSetNull(mut p: *mut sqlite3_value) {
    sqlite3VdbeMemSetNull(p);
}

// /* Value to be set */
// /* Length of string z */
// /* Text of the new string */
// /* Encoding to use */
// /* Destructor for the string */
// /*
// ** Free an sqlite3_value object
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3ValueFree(mut v: *mut sqlite3_value) {
    if !(v != std::ptr::null_mut::<sqlite3_value>()) {
        return;
    }
    sqlite3VdbeMemRelease(v);
    unsafe { sqlite3DbFreeNN(unsafe { (*v).db }, v as *mut ()) };
}

// /*
// ** Create a new sqlite3_value object.
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3ValueNew(mut db: *mut sqlite3) -> *mut sqlite3_value {
    let mut p: *mut sqlite3_value =
        (unsafe { sqlite3DbMallocZero(db, 56 as u64) }) as *mut sqlite3_value;
    if p != std::ptr::null_mut::<sqlite3_value>() {
        unsafe {
            (*p).flags = ((1 as i32) as i16) as u16;
        }
        unsafe {
            (*p).db = db;
        }
    }
    return p;
}

// /* The database connection */
// /* The expression to evaluate */
// /* Encoding to use */
// /* Affinity to use */
// /* Write the new value here */
// /* Second argument for valueNew() */
// /*
// ** Create a new sqlite3_value object, containing the value of pExpr.
// **
// ** This only works for very simple expressions that consist of one constant
// ** token (i.e. "5", "5.1", "'a string'"). If the expression can
// ** be converted directly into a value, then the value is allocated and
// ** a pointer written to *ppVal. The caller is responsible for deallocating
// ** the value by passing it to sqlite3ValueFree() later on. If the expression
// ** cannot be converted to a value, then *ppVal is set to NULL.
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3ValueFromExpr(
    mut db: *mut sqlite3,
    mut pExpr: *const Expr,
    mut enc: u8,
    mut affinity: u8,
    mut ppVal: *mut *mut sqlite3_value,
) -> i32 {
    let __v787: i32;
    if pExpr != std::ptr::null::<Expr>() {
        __v787 = valueFromExpr(
            db,
            pExpr,
            enc,
            affinity,
            ppVal,
            std::ptr::null_mut::<ValueNewStat4Ctx>(),
        );
    } else {
        __v787 = 0 as i32;
    }
    return __v787;
}

// /* A no-op destructor */
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.vdbemem.sqlite3NoopDestructor")]
extern "C-unwind" fn sqlite3NoopDestructor(mut p: *mut ()) {
    p;
}

// /*
// ** If pMem is an object with a valid string representation, this routine
// ** ensures the internal encoding for the string representation is
// ** 'desiredEnc', one of SQLITE_UTF8, SQLITE_UTF16LE or SQLITE_UTF16BE.
// **
// ** If pMem is not a string object, or the encoding of the string
// ** representation is already stored using the requested encoding, then this
// ** routine is a no-op.
// **
// ** SQLITE_OK is returned if the conversion is successful (or not required).
// ** SQLITE_NOMEM may be returned if a malloc() fails during conversion
// ** between formats.
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3VdbeChangeEncoding(
    mut pMem: *mut sqlite3_value,
    mut desiredEnc: i32,
) -> i32 {
    let mut rc: i32 = 0 as i32;
    0 as i32;
    0 as i32;
    0 as i32;
    if !((((unsafe { (*pMem).flags }) as u32) as i32) & (2 as i32) != (0 as i32)) {
        unsafe {
            (*pMem).enc = (desiredEnc as i8) as u8;
        }
        return 0 as i32;
    }
    if (((unsafe { (*pMem).enc }) as u32) as i32) == desiredEnc {
        return 0 as i32;
    }
    0 as i32;
    // /* MemTranslate() may return SQLITE_OK or SQLITE_NOMEM. If NOMEM is returned,
    //   ** then the encoding of the value may not have changed.
    //   */
    rc = unsafe { sqlite3VdbeMemTranslate(pMem, (desiredEnc as i8) as u8) };
    0 as i32;
    0 as i32;
    0 as i32;
    return rc;
}

// /*
// ** Return true if the Mem object contains a TEXT or BLOB that is
// ** too large - whose size exceeds SQLITE_MAX_LENGTH.
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3VdbeMemTooBig(mut p: *mut sqlite3_value) -> i32 {
    0 as i32;
    if (((unsafe { (*p).flags }) as u32) as i32) & ((2 as i32) | (16 as i32)) != (0 as i32) {
        let mut n: i32 = unsafe { (*p).n };
        if (((unsafe { (*p).flags }) as u32) as i32) & (1024 as i32) != (0 as i32) {
            let __v788: i32 = n;
            let __v789: i32 = __v788 + unsafe { (*p).u.nZero };
            n = __v789;
        }
        return (n > unsafe {
            *unsafe {
                unsafe { (*unsafe { (*p).db }).aLimit.as_mut_ptr() as *mut i32 }
                    .offset((0 as i32) as isize)
            }
        }) as i32;
    }
    return 0 as i32;
}

// /*
// ** Make a full copy of pFrom into pTo.  Prior contents of pTo are
// ** freed before the copy is made.
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3VdbeMemCopy(
    mut pTo: *mut sqlite3_value,
    mut pFrom: *const sqlite3_value,
) -> i32 {
    let mut rc: i32 = 0 as i32;
    0 as i32;
    if (((unsafe { (*pTo).flags }) as u32) as i32) & ((32768 as i32) | (4096 as i32)) != (0 as i32)
    {
        vdbeMemClearExternAndSetNull(pTo);
    }
    unsafe { memcpy(pTo as *mut (), pFrom as *const (), 24 as u64) };
    let __v790: *mut sqlite3_value = pTo;
    let __v791: u16 = unsafe { (*__v790).flags };
    let __v792: u16 = ((((__v791 as u32) as i32) & !(4096 as i32)) as i16) as u16;
    unsafe {
        (*__v790).flags = __v792;
    }
    if (((unsafe { (*pTo).flags }) as u32) as i32) & ((2 as i32) | (16 as i32)) != (0 as i32) {
        if (0 as i32) == (((unsafe { (*pFrom).flags }) as u32) as i32) & (8192 as i32) {
            let __v793: *mut sqlite3_value = pTo;
            let __v794: u16 = unsafe { (*__v793).flags };
            let __v795: u16 = ((((__v794 as u32) as i32) | (16384 as i32)) as i16) as u16;
            unsafe {
                (*__v793).flags = __v795;
            }
            rc = sqlite3VdbeMemMakeWriteable(pTo);
        }
    }
    return rc;
}

#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3VdbeMemShallowCopy(
    mut pTo: *mut sqlite3_value,
    mut pFrom: *const sqlite3_value,
    mut srcType: i32,
) {
    0 as i32;
    0 as i32;
    if (((unsafe { (*pTo).flags }) as u32) as i32) & ((32768 as i32) | (4096 as i32)) != (0 as i32)
    {
        vdbeClrCopy(pTo, pFrom, srcType);
        return;
    }
    unsafe { memcpy(pTo as *mut (), pFrom as *const (), 24 as u64) };
    if (((unsafe { (*pFrom).flags }) as u32) as i32) & (8192 as i32) == (0 as i32) {
        let __v796: *mut sqlite3_value = pTo;
        let __v797: u16 = unsafe { (*__v796).flags };
        let __v798: u16 = ((((__v797 as u32) as i32)
            & !((4096 as i32) | (8192 as i32) | (16384 as i32))) as i16)
            as u16;
        unsafe {
            (*__v796).flags = __v798;
        }
        0 as i32;
        let __v799: *mut sqlite3_value = pTo;
        let __v800: u16 = unsafe { (*__v799).flags };
        let __v801: u16 = ((((__v800 as u32) as i32) | srcType) as i16) as u16;
        unsafe {
            (*__v799).flags = __v801;
        }
    }
}

// /*
// ** Transfer the contents of pFrom to pTo. Any existing value in pTo is
// ** freed. If pFrom contains ephemeral data, a copy is made.
// **
// ** pFrom contains an SQL NULL when this routine returns.
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3VdbeMemMove(
    mut pTo: *mut sqlite3_value,
    mut pFrom: *mut sqlite3_value,
) {
    0 as i32;
    0 as i32;
    0 as i32;
    sqlite3VdbeMemRelease(pTo);
    unsafe { memcpy(pTo as *mut (), pFrom as *const (), 56 as u64) };
    unsafe {
        (*pFrom).flags = ((1 as i32) as i16) as u16;
    }
    unsafe {
        (*pFrom).szMalloc = 0 as i32;
    }
}

// /*
// ** Make sure the given Mem is \u0000 terminated.
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3VdbeMemNulTerminate(mut pMem: *mut sqlite3_value) -> i32 {
    0 as i32;
    0 as i32;
    {}
    {}
    if (((unsafe { (*pMem).flags }) as u32) as i32) & ((512 as i32) | (2 as i32)) != (2 as i32) {
        // /* Nothing to do */
        return 0 as i32;
    } else {
        return vdbeMemAddTerminator(pMem);
    }
    return unsafe { std::mem::zeroed() };
}

// /*
// ** Change the value of a Mem to be a string or a BLOB.
// **
// ** The memory management strategy depends on the value of the xDel
// ** parameter. If the value passed is SQLITE_TRANSIENT, then the
// ** string is copied into a (possibly existing) buffer managed by the
// ** Mem structure. Otherwise, any existing buffer is freed and the
// ** pointer copied.
// **
// ** If the string is too large (if it exceeds the SQLITE_LIMIT_LENGTH
// ** size limit) then no memory allocation occurs.  If the string can be
// ** stored without allocating memory, then it is.  If a memory allocation
// ** is required to store the string, then value of pMem is unchanged.  In
// ** either case, SQLITE_TOOBIG is returned.
// **
// ** The "enc" parameter is the text encoding for the string, or zero
// ** to store a blob.
// **
// ** If n is negative, then the string consists of all bytes up to but
// ** excluding the first zero character.  The n parameter must be
// ** non-negative for blobs.
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3VdbeMemSetStr(
    mut pMem: *mut sqlite3_value,
    mut z: *const i8,
    mut n: i64,
    mut enc: u8,
    mut xDel: Option<unsafe extern "C-unwind" fn(*mut ())>,
) -> i32 {
    // /* New value for pMem->n */
    let mut nByte: i64 = n;
    // /* Maximum allowed string or blob size */
    let mut iLimit: i32 = 0 as i32;
    // /* New value for pMem->flags */
    let mut flags: u16 = 0 as u16;
    0 as i32;
    0 as i32;
    0 as i32;
    0 as i32;
    // /* If z is a NULL pointer, set pMem to contain an SQL NULL. */
    if !(z != std::ptr::null::<i8>()) {
        sqlite3VdbeMemSetNull(pMem);
        return 0 as i32;
    }
    if (unsafe { (*pMem).db }) != std::ptr::null_mut::<sqlite3>() {
        iLimit = unsafe {
            *unsafe {
                unsafe { (*unsafe { (*pMem).db }).aLimit.as_mut_ptr() as *mut i32 }
                    .offset((0 as i32) as isize)
            }
        };
    } else {
        iLimit = 1000000000 as i32;
    }
    if nByte < ((0 as i32) as i64) {
        0 as i32;
        if ((enc as u32) as i32) == (1 as i32) {
            nByte = (unsafe { strlen(z) }) as i64;
        } else {
            nByte = (0 as i32) as i64;
            '__slate_break_782: loop {
                if !(nByte <= (iLimit as i64)
                    && ((unsafe { *unsafe { z.offset(nByte as isize) } }) as i32)
                        | ((unsafe { *unsafe { z.offset((nByte + ((1 as i32) as i64)) as isize) } })
                            as i32)
                        != (0 as i32))
                {
                    break;
                }
                let __v802: i64 = nByte;
                let __v803: i64 = __v802 + ((2 as i32) as i64);
                nByte = __v803;
            }
        }
        flags = (((2 as i32) | (512 as i32)) as i16) as u16;
    } else {
        if ((enc as u32) as i32) == (0 as i32) {
            flags = ((16 as i32) as i16) as u16;
            enc = ((1 as i32) as i8) as u8;
        } else {
            flags = ((2 as i32) as i16) as u16;
        }
    }
    if nByte > (iLimit as i64) {
        if xDel != None
            && xDel
                != unsafe {
                    std::mem::transmute::<usize, Option<unsafe extern "C-unwind" fn(*mut ())>>(
                        -(1 as i32) as usize,
                    )
                }
        {
            if xDel
                == unsafe {
                    std::mem::transmute::<*const (), Option<unsafe extern "C-unwind" fn(*mut ())>>(
                        sqlite3RowSetClear as *const (),
                    )
                }
            {
                unsafe { sqlite3DbFree(unsafe { (*pMem).db }, z as *mut ()) };
            } else {
                unsafe { xDel.unwrap()(z as *mut ()) };
            }
        }
        sqlite3VdbeMemSetNull(pMem);
        return unsafe { sqlite3ErrorToParser(unsafe { (*pMem).db }, 18 as i32) };
    }
    // /* The following block sets the new values of Mem.z and Mem.xDel. It
    //   ** also sets a flag in local variable "flags" to indicate the memory
    //   ** management (one of MEM_Dyn or MEM_Static).
    //   */
    if xDel
        == unsafe {
            std::mem::transmute::<usize, Option<unsafe extern "C-unwind" fn(*mut ())>>(
                -(1 as i32) as usize,
            )
        }
    {
        let mut nAlloc: i64 = nByte;
        if ((flags as u32) as i32) & (512 as i32) != (0 as i32) {
            let __v804: i64 = nAlloc;
            let __v805: i64 = __v804
                + ((if ((enc as u32) as i32) == (1 as i32) {
                    1 as i32
                } else {
                    2 as i32
                }) as i64);
            nAlloc = __v805;
        }
        {}
        {}
        {}
        if sqlite3VdbeMemClearAndResize(
            pMem,
            (if nAlloc > ((32 as i32) as i64) {
                nAlloc
            } else {
                (32 as i32) as i64
            }) as i32,
        ) != (0 as i32)
        {
            return 7 as i32;
        }
        0 as i32;
        unsafe {
            memcpy(
                (unsafe { (*pMem).z }) as *mut (),
                z as *const (),
                nAlloc as u64,
            )
        };
    } else {
        sqlite3VdbeMemRelease(pMem);
        unsafe {
            (*pMem).z = z as *mut i8;
        }
        if xDel
            == unsafe {
                std::mem::transmute::<*const (), Option<unsafe extern "C-unwind" fn(*mut ())>>(
                    sqlite3RowSetClear as *const (),
                )
            }
        {
            unsafe {
                (*pMem).zMalloc = unsafe { (*pMem).z };
            }
            unsafe {
                (*pMem).szMalloc = unsafe {
                    sqlite3DbMallocSize(
                        unsafe { (*pMem).db },
                        (unsafe { (*pMem).zMalloc }) as *const (),
                    )
                };
            }
        } else {
            unsafe {
                (*pMem).xDel = xDel;
            }
            let __v806: u16 = flags;
            let __v807: u16 = ((((__v806 as u32) as i32)
                | if xDel == None {
                    8192 as i32
                } else {
                    4096 as i32
                }) as i16) as u16;
            flags = __v807;
        }
    }
    unsafe {
        (*pMem).n = (nByte & ((2147483647 as i32) as i64)) as i32;
    }
    unsafe {
        (*pMem).flags = flags;
    }
    unsafe {
        (*pMem).enc = enc;
    }
    let __v808: bool;
    if ((enc as u32) as i32) > (1 as i32) {
        __v808 = (unsafe { sqlite3VdbeMemHandleBom(pMem) }) != (0 as i32);
    } else {
        __v808 = false as bool;
    }
    if __v808 {
        return 7 as i32;
    }
    return 0 as i32;
}

// /* Memory cell to set to string value */
// /* String pointer */
// /* Bytes in string, or negative */
// /* Encoding of z.  0 for BLOBs */
// /* Destructor function */
// /* Like sqlite3VdbeMemSetStr() except:
// **
// **   enc is always SQLITE_UTF8
// **   pMem->db is always non-NULL
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3VdbeMemSetText(
    mut pMem: *mut sqlite3_value,
    mut z: *const i8,
    mut n: i64,
    mut xDel: Option<unsafe extern "C-unwind" fn(*mut ())>,
) -> i32 {
    // /* New value for pMem->n */
    let mut nByte: i64 = n;
    let mut flags: u16 = 0 as u16;
    0 as i32;
    0 as i32;
    0 as i32;
    0 as i32;
    // /* If z is a NULL pointer, set pMem to contain an SQL NULL. */
    if !(z != std::ptr::null::<i8>()) {
        sqlite3VdbeMemSetNull(pMem);
        return 0 as i32;
    }
    if nByte < ((0 as i32) as i64) {
        nByte = (unsafe { strlen(z) }) as i64;
        flags = (((2 as i32) | (512 as i32)) as i16) as u16;
    } else {
        flags = ((2 as i32) as i16) as u16;
    }
    if nByte
        > ((unsafe {
            *unsafe {
                unsafe { (*unsafe { (*pMem).db }).aLimit.as_mut_ptr() as *mut i32 }
                    .offset((0 as i32) as isize)
            }
        }) as i64)
    {
        if xDel != None
            && xDel
                != unsafe {
                    std::mem::transmute::<usize, Option<unsafe extern "C-unwind" fn(*mut ())>>(
                        -(1 as i32) as usize,
                    )
                }
        {
            if xDel
                == unsafe {
                    std::mem::transmute::<*const (), Option<unsafe extern "C-unwind" fn(*mut ())>>(
                        sqlite3RowSetClear as *const (),
                    )
                }
            {
                unsafe { sqlite3DbFree(unsafe { (*pMem).db }, z as *mut ()) };
            } else {
                unsafe { xDel.unwrap()(z as *mut ()) };
            }
        }
        sqlite3VdbeMemSetNull(pMem);
        return unsafe { sqlite3ErrorToParser(unsafe { (*pMem).db }, 18 as i32) };
    }
    // /* The following block sets the new values of Mem.z and Mem.xDel. It
    //   ** also sets a flag in local variable "flags" to indicate the memory
    //   ** management (one of MEM_Dyn or MEM_Static).
    //   */
    if xDel
        == unsafe {
            std::mem::transmute::<usize, Option<unsafe extern "C-unwind" fn(*mut ())>>(
                -(1 as i32) as usize,
            )
        }
    {
        let mut nAlloc: i64 = nByte + ((1 as i32) as i64);
        {}
        {}
        if sqlite3VdbeMemClearAndResize(
            pMem,
            (if nAlloc > ((32 as i32) as i64) {
                nAlloc
            } else {
                (32 as i32) as i64
            }) as i32,
        ) != (0 as i32)
        {
            return 7 as i32;
        }
        0 as i32;
        unsafe {
            memcpy(
                (unsafe { (*pMem).z }) as *mut (),
                z as *const (),
                nByte as u64,
            )
        };
        unsafe {
            *unsafe { unsafe { (*pMem).z }.offset(nByte as isize) } = (0 as i32) as i8;
        }
    } else {
        sqlite3VdbeMemRelease(pMem);
        unsafe {
            (*pMem).z = z as *mut i8;
        }
        if xDel
            == unsafe {
                std::mem::transmute::<*const (), Option<unsafe extern "C-unwind" fn(*mut ())>>(
                    sqlite3RowSetClear as *const (),
                )
            }
        {
            unsafe {
                (*pMem).zMalloc = unsafe { (*pMem).z };
            }
            unsafe {
                (*pMem).szMalloc = unsafe {
                    sqlite3DbMallocSize(
                        unsafe { (*pMem).db },
                        (unsafe { (*pMem).zMalloc }) as *const (),
                    )
                };
            }
            unsafe {
                (*pMem).xDel = None;
            }
        } else {
            if xDel == None {
                unsafe {
                    (*pMem).xDel = xDel;
                }
                let __v809: u16 = flags;
                let __v810: u16 = ((((__v809 as u32) as i32) | (8192 as i32)) as i16) as u16;
                flags = __v810;
            } else {
                unsafe {
                    (*pMem).xDel = xDel;
                }
                let __v811: u16 = flags;
                let __v812: u16 = ((((__v811 as u32) as i32) | (4096 as i32)) as i16) as u16;
                flags = __v812;
            }
        }
    }
    unsafe {
        (*pMem).flags = flags;
    }
    unsafe {
        (*pMem).n = (nByte & ((2147483647 as i32) as i64)) as i32;
    }
    unsafe {
        (*pMem).enc = ((1 as i32) as i8) as u8;
    }
    return 0 as i32;
}

// /*
// ** Delete any previous value and set the value stored in *pMem to val,
// ** manifest type INTEGER.
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3VdbeMemSetInt64(mut pMem: *mut sqlite3_value, mut val: i64) {
    if (((unsafe { (*pMem).flags }) as u32) as i32) & ((32768 as i32) | (4096 as i32)) != (0 as i32)
    {
        vdbeReleaseAndSetInt64(pMem, val);
    } else {
        unsafe {
            (*pMem).u.i = val;
        }
        unsafe {
            (*pMem).flags = ((4 as i32) as i16) as u16;
        }
    }
}

// /*
// ** Delete any previous value and set the value stored in *pMem to val,
// ** manifest type REAL.
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3VdbeMemSetDouble(mut pMem: *mut sqlite3_value, mut val: f64) {
    sqlite3VdbeMemSetNull(pMem);
    if !((unsafe { sqlite3IsNaN(val) }) != (0 as i32)) {
        unsafe {
            (*pMem).u.r = val;
        }
        unsafe {
            (*pMem).flags = ((8 as i32) as i16) as u16;
        }
    }
}

// /*
// ** Set the value stored in *pMem should already be a NULL.
// ** Also store a pointer to go with it.
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3VdbeMemSetPointer(
    mut pMem: *mut sqlite3_value,
    mut pPtr: *mut (),
    mut zPType: *const i8,
    mut xDestructor: Option<unsafe extern "C-unwind" fn(*mut ())>,
) {
    0 as i32;
    vdbeMemClear(pMem);
    unsafe {
        (*pMem).u.zPType = if zPType != std::ptr::null::<i8>() {
            zPType
        } else {
            (b"\0".as_ptr() as *mut i8) as *const i8
        };
    }
    unsafe {
        (*pMem).z = pPtr as *mut i8;
    }
    unsafe {
        (*pMem).flags = (((1 as i32) | (4096 as i32) | (2048 as i32) | (512 as i32)) as i16) as u16;
    }
    unsafe {
        (*pMem).eSubtype = ((112 as i32) as i8) as u8;
    }
    unsafe {
        (*pMem).xDel = {
            let __t0: Option<unsafe extern "C-unwind" fn(*mut ())> = if xDestructor != None {
                xDestructor
            } else {
                Some(sqlite3NoopDestructor)
            };
            __t0
        };
    }
}

// /*
// ** Initialize bulk memory to be a consistent Mem object.
// **
// ** The minimum amount of initialization feasible is performed.
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3VdbeMemInit(
    mut pMem: *mut sqlite3_value,
    mut db: *mut sqlite3,
    mut flags: u16,
) {
    0 as i32;
    unsafe {
        (*pMem).flags = flags;
    }
    unsafe {
        (*pMem).db = db;
    }
    unsafe {
        (*pMem).szMalloc = 0 as i32;
    }
}

// /*
// ** Delete any previous value and set the value stored in *pMem to NULL.
// **
// ** This routine calls the Mem.xDel destructor to dispose of values that
// ** require the destructor.  But it preserves the Mem.zMalloc memory allocation.
// ** To free all resources, use sqlite3VdbeMemRelease(), which both calls this
// ** routine to invoke the destructor and deallocates Mem.zMalloc.
// **
// ** Use this routine to reset the Mem prior to insert a new value.
// **
// ** Use sqlite3VdbeMemRelease() to complete erase the Mem prior to abandoning it.
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3VdbeMemSetNull(mut pMem: *mut sqlite3_value) {
    if (((unsafe { (*pMem).flags }) as u32) as i32) & ((32768 as i32) | (4096 as i32)) != (0 as i32)
    {
        vdbeMemClearExternAndSetNull(pMem);
    } else {
        unsafe {
            (*pMem).flags = ((1 as i32) as i16) as u16;
        }
    }
}

// /*
// ** Delete any previous value and set the value to be a BLOB of length
// ** n containing all zeros.
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3VdbeMemSetZeroBlob(mut pMem: *mut sqlite3_value, mut n: i32) {
    sqlite3VdbeMemRelease(pMem);
    unsafe {
        (*pMem).flags = (((16 as i32) | (1024 as i32)) as i16) as u16;
    }
    unsafe {
        (*pMem).n = 0 as i32;
    }
    if n < (0 as i32) {
        n = 0 as i32;
    }
    unsafe {
        (*pMem).u.nZero = n;
    }
    unsafe {
        (*pMem).enc = ((1 as i32) as i8) as u8;
    }
    unsafe {
        (*pMem).z = std::ptr::null_mut::<i8>();
    }
}

// /*
// ** Delete any previous value and set the value of pMem to be an
// ** empty boolean index.
// **
// ** Return SQLITE_OK on success and SQLITE_NOMEM if a memory allocation
// ** error occurs.
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3VdbeMemSetRowSet(mut pMem: *mut sqlite3_value) -> i32 {
    let mut db: *mut sqlite3 = unsafe { (*pMem).db };
    let mut p: *mut RowSet = unsafe { std::mem::zeroed() };
    0 as i32;
    0 as i32;
    sqlite3VdbeMemRelease(pMem);
    p = unsafe { sqlite3RowSetInit(db) };
    if p == std::ptr::null_mut::<RowSet>() {
        return 7 as i32;
    }
    unsafe {
        (*pMem).z = p as *mut i8;
    }
    unsafe {
        (*pMem).flags = (((16 as i32) | (4096 as i32)) as i16) as u16;
    }
    unsafe {
        (*pMem).xDel = unsafe {
            std::mem::transmute::<*const (), Option<unsafe extern "C-unwind" fn(*mut ())>>(
                sqlite3RowSetDelete as *const (),
            )
        };
    }
    return 0 as i32;
}

// /*
// ** If pMem is already a string, detect if it is a zero-terminated
// ** string, or make it into one if possible, and mark it as such.
// **
// ** This is an optimization.  Correct operation continues even if
// ** this routine is a no-op.
// **
// ** Return true if the strig is zero-terminated after this routine is
// ** called and false if it is not.
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3VdbeMemZeroTerminateIfAble(mut pMem: *mut sqlite3_value) -> i32 {
    if (((unsafe { (*pMem).flags }) as u32) as i32)
        & ((2 as i32) | (512 as i32) | (16384 as i32) | (8192 as i32))
        != (2 as i32)
    {
        // /* pMem must be a string, and it cannot be an ephemeral or static string */
        return 0 as i32;
    }
    if (((unsafe { (*pMem).enc }) as u32) as i32) != (1 as i32) {
        return 0 as i32;
    }
    0 as i32;
    if (((unsafe { (*pMem).flags }) as u32) as i32) & (4096 as i32) != (0 as i32) {
        let __v813: bool;
        if (unsafe { (*pMem).xDel })
            == unsafe {
                std::mem::transmute::<*const (), Option<unsafe extern "C-unwind" fn(*mut ())>>(
                    sqlite3_free as *const (),
                )
            }
        {
            __v813 = (unsafe { sqlite3_msize((unsafe { (*pMem).z }) as *mut ()) })
                >= ((((unsafe { (*pMem).n }) + (1 as i32)) as i64) as u64);
        } else {
            __v813 = false as bool;
        }
        if __v813 {
            unsafe {
                *unsafe { unsafe { (*pMem).z }.offset((unsafe { (*pMem).n }) as isize) } =
                    (0 as i32) as i8;
            }
            let __v814: *mut sqlite3_value = pMem;
            let __v815: u16 = unsafe { (*__v814).flags };
            let __v816: u16 = ((((__v815 as u32) as i32) | (512 as i32)) as i16) as u16;
            unsafe {
                (*__v814).flags = __v816;
            }
            return 1 as i32;
        }
        if (unsafe { (*pMem).xDel })
            == unsafe {
                std::mem::transmute::<*const (), Option<unsafe extern "C-unwind" fn(*mut ())>>(
                    sqlite3RCStrUnref as *const (),
                )
            }
        {
            // /* Blindly assume that all RCStr objects are zero-terminated */
            let __v817: *mut sqlite3_value = pMem;
            let __v818: u16 = unsafe { (*__v817).flags };
            let __v819: u16 = ((((__v818 as u32) as i32) | (512 as i32)) as i16) as u16;
            unsafe {
                (*__v817).flags = __v819;
            }
            return 1 as i32;
        }
    } else {
        if (unsafe { (*pMem).szMalloc }) >= (unsafe { (*pMem).n }) + (1 as i32) {
            unsafe {
                *unsafe { unsafe { (*pMem).z }.offset((unsafe { (*pMem).n }) as isize) } =
                    (0 as i32) as i8;
            }
            let __v820: *mut sqlite3_value = pMem;
            let __v821: u16 = unsafe { (*__v820).flags };
            let __v822: u16 = ((((__v821 as u32) as i32) | (512 as i32)) as i16) as u16;
            unsafe {
                (*__v820).flags = __v822;
            }
            return 1 as i32;
        }
    }
    return 0 as i32;
}

// /*
// ** Change pMem so that its MEM_Str or MEM_Blob value is stored in
// ** MEM.zMalloc, where it can be safely written.
// **
// ** Return SQLITE_OK on success or SQLITE_NOMEM if malloc fails.
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3VdbeMemMakeWriteable(mut pMem: *mut sqlite3_value) -> i32 {
    0 as i32;
    0 as i32;
    0 as i32;
    if (((unsafe { (*pMem).flags }) as u32) as i32) & ((2 as i32) | (16 as i32)) != (0 as i32) {
        let __v823: i32;
        if (((unsafe { (*pMem).flags }) as u32) as i32) & (1024 as i32) != (0 as i32) {
            __v823 = sqlite3VdbeMemExpandBlob(pMem);
        } else {
            __v823 = 0 as i32;
        }
        if __v823 != (0 as i32) {
            return 7 as i32;
        }
        if (unsafe { (*pMem).szMalloc }) == (0 as i32)
            || (unsafe { (*pMem).z }) != unsafe { (*pMem).zMalloc }
        {
            let mut rc: i32 = vdbeMemAddTerminator(pMem);
            if rc != (0 as i32) {
                return rc;
            }
        }
    }
    let __v824: *mut sqlite3_value = pMem;
    let __v825: u16 = unsafe { (*__v824).flags };
    let __v826: u16 = ((((__v825 as u32) as i32) & !(16384 as i32)) as i16) as u16;
    unsafe {
        (*__v824).flags = __v826;
    }
    return 0 as i32;
}

// /*
// ** Add MEM_Str to the set of representations for the given Mem.  This
// ** routine is only called if pMem is a number of some kind, not a NULL
// ** or a BLOB.
// **
// ** Existing representations MEM_Int, MEM_Real, or MEM_IntReal are invalidated
// ** if bForce is true but are retained if bForce is false.
// **
// ** A MEM_Null value will never be passed to this function. This function is
// ** used for converting values to text for returning to the user (i.e. via
// ** sqlite3_value_text()), or for ensuring that values to be used as btree
// ** keys are strings. In the former case a NULL pointer is returned the
// ** user and the latter is an internal programming error.
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3VdbeMemStringify(
    mut pMem: *mut sqlite3_value,
    mut enc: u8,
    mut bForce: u8,
) -> i32 {
    let mut nByte: i32 = 32 as i32;
    0 as i32;
    0 as i32;
    0 as i32;
    0 as i32;
    0 as i32;
    0 as i32;
    0 as i32;
    if sqlite3VdbeMemClearAndResize(pMem, nByte) != (0 as i32) {
        unsafe {
            (*pMem).enc = ((0 as i32) as i8) as u8;
        }
        return 7 as i32;
    }
    vdbeMemRenderNum(nByte, unsafe { (*pMem).z }, pMem);
    0 as i32;
    0 as i32;
    unsafe {
        (*pMem).enc = ((1 as i32) as i8) as u8;
    }
    let __v827: *mut sqlite3_value = pMem;
    let __v828: u16 = unsafe { (*__v827).flags };
    let __v829: u16 = ((((__v828 as u32) as i32) | ((2 as i32) | (512 as i32))) as i16) as u16;
    unsafe {
        (*__v827).flags = __v829;
    }
    if bForce != (0 as u8) {
        let __v830: *mut sqlite3_value = pMem;
        let __v831: u16 = unsafe { (*__v830).flags };
        let __v832: u16 =
            ((((__v831 as u32) as i32) & !((4 as i32) | (8 as i32) | (32 as i32))) as i16) as u16;
        unsafe {
            (*__v830).flags = __v832;
        }
    }
    sqlite3VdbeChangeEncoding(pMem, (enc as u32) as i32);
    return 0 as i32;
}

#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3VdbeIntValue(mut pMem: *const sqlite3_value) -> i64 {
    let mut flags: i32 = 0 as i32;
    0 as i32;
    0 as i32;
    0 as i32;
    flags = ((unsafe { (*pMem).flags }) as u32) as i32;
    if flags & ((4 as i32) | (32 as i32)) != (0 as i32) {
        {}
        return unsafe { (*pMem).u.i };
    } else {
        if flags & (8 as i32) != (0 as i32) {
            return sqlite3RealToI64(unsafe { (*pMem).u.r });
        } else {
            if flags & ((2 as i32) | (16 as i32)) != (0 as i32)
                && (unsafe { (*pMem).z }) != std::ptr::null_mut::<i8>()
            {
                return memIntValue(pMem);
            } else {
                return (0 as i32) as i64;
            }
        }
    }
    return unsafe { std::mem::zeroed() };
}

// /*
// ** Convert pMem to type integer.  Invalidate any prior representations.
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3VdbeMemIntegerify(mut pMem: *mut sqlite3_value) -> i32 {
    0 as i32;
    0 as i32;
    0 as i32;
    0 as i32;
    unsafe {
        (*pMem).u.i = sqlite3VdbeIntValue(pMem as *const sqlite3_value);
    }
    unsafe {
        (*pMem).flags = (((((unsafe { (*pMem).flags }) as u32) as i32)
            & !((3519 as i32) | (1024 as i32))
            | (4 as i32)) as i16) as u16;
    }
    return 0 as i32;
}

// /*
// ** Return the best representation of pMem that we can get into a
// ** double.  If pMem is already a double or an integer, return its
// ** value.  If it is a string or blob, try to convert it to a double.
// ** If it is a NULL, return 0.0.
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3VdbeRealValue(mut pMem: *mut sqlite3_value) -> f64 {
    0 as i32;
    0 as i32;
    0 as i32;
    if (((unsafe { (*pMem).flags }) as u32) as i32) & (8 as i32) != (0 as i32) {
        return unsafe { (*pMem).u.r };
    } else {
        if (((unsafe { (*pMem).flags }) as u32) as i32) & ((4 as i32) | (32 as i32)) != (0 as i32) {
            {}
            return (unsafe { (*pMem).u.i }) as f64;
        } else {
            if (((unsafe { (*pMem).flags }) as u32) as i32) & ((2 as i32) | (16 as i32))
                != (0 as i32)
            {
                return sqlite3MemRealValueNoRC(pMem);
            } else {
                // /* (double)0 In case of SQLITE_OMIT_FLOATING_POINT... */
                return (0 as i32) as f64;
            }
        }
    }
    return unsafe { std::mem::zeroed() };
}

// /*
// ** Invoke sqlite3AtoF() on the text value of pMem.  Write the
// ** translation of the text input into *pValue.
// **
// ** The caller must ensure that pMem->db!=0 and that pMem is in
// ** mode MEM_Str or MEM_Blob.
// **
// ** Result code invariants:
// **
// **    rc==0         =>   ERROR: Input string not well-formed, or OOM
// **    rc<0          =>   Some prefix of the input is well-formed
// **    rc>0          =>   All of the input is well-formed
// **    (rc&2)==0     =>   The number is expressed as an integer, with no
// **                       decimal point or eNNN suffix.
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3MemRealValueRC(
    mut pMem: *mut sqlite3_value,
    mut pValue: *mut f64,
) -> i32 {
    {}
    0 as i32;
    if (unsafe { (*pMem).z }) == std::ptr::null_mut::<i8>() {
        unsafe {
            *pValue = 0.0f64;
        }
        return 0 as i32;
    } else {
        let __v833: bool;
        if (((unsafe { (*pMem).enc }) as u32) as i32) == (1 as i32) {
            let __v834: bool;
            if (((unsafe { (*pMem).flags }) as u32) as i32) & (512 as i32) != (0 as i32) {
                __v834 = true as bool;
            } else {
                __v834 = sqlite3VdbeMemZeroTerminateIfAble(pMem) != (0 as i32);
            }
            __v833 = __v834;
        } else {
            __v833 = false as bool;
        }
        if __v833 {
            return unsafe { sqlite3AtoF((unsafe { (*pMem).z }) as *const i8, pValue) };
        } else {
            if (unsafe { (*pMem).n }) == (0 as i32) {
                unsafe {
                    *pValue = 0.0f64;
                }
                return 0 as i32;
            } else {
                return sqlite3MemRealValueRCSlowPath(pMem, pValue);
            }
        }
    }
    return unsafe { std::mem::zeroed() };
}

// /*
// ** Return 1 if pMem represents true, and return 0 if pMem represents false.
// ** Return the value ifNull if pMem is NULL.
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3VdbeBooleanValue(mut pMem: *mut sqlite3_value, mut ifNull: i32) -> i32 {
    {}
    if (((unsafe { (*pMem).flags }) as u32) as i32) & ((4 as i32) | (32 as i32)) != (0 as i32) {
        return ((unsafe { (*pMem).u.i }) != ((0 as i32) as i64)) as i32;
    }
    if (((unsafe { (*pMem).flags }) as u32) as i32) & (1 as i32) != (0 as i32) {
        return ifNull;
    }
    return (sqlite3VdbeRealValue(pMem) != 0.0f64) as i32;
}

// /*
// ** The MEM structure is already a MEM_Real or MEM_IntReal. Try to
// ** make it a MEM_Int if we can.
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3VdbeIntegerAffinity(mut pMem: *mut sqlite3_value) {
    0 as i32;
    0 as i32;
    0 as i32;
    0 as i32;
    0 as i32;
    if (((unsafe { (*pMem).flags }) as u32) as i32) & (32 as i32) != (0 as i32) {
        unsafe {
            (*pMem).flags = (((((unsafe { (*pMem).flags }) as u32) as i32)
                & !((3519 as i32) | (1024 as i32))
                | (4 as i32)) as i16) as u16;
        }
    } else {
        let mut ix: i64 = sqlite3RealToI64(unsafe { (*pMem).u.r });
        // /* Only mark the value as an integer if
        //     **
        //     **    (1) the round-trip conversion real->int->real is a no-op, and
        //     **    (2) The integer is neither the largest nor the smallest
        //     **        possible integer (ticket #3922)
        //     **
        //     ** The second and third terms in the following conditional enforces
        //     ** the second condition under the assumption that addition overflow causes
        //     ** values to wrap around.
        //     */
        if (unsafe { (*pMem).u.r }) == (ix as f64)
            && ix
                > (-(1 as i32) as i64)
                    - ((((4294967295 as u32) as u64) as i64)
                        | ((2147483647 as i32) as i64) << (32 as i32))
            && ix
                < (((4294967295 as u32) as u64) as i64)
                    | ((2147483647 as i32) as i64) << (32 as i32)
        {
            unsafe {
                (*pMem).u.i = ix;
            }
            unsafe {
                (*pMem).flags = (((((unsafe { (*pMem).flags }) as u32) as i32)
                    & !((3519 as i32) | (1024 as i32))
                    | (4 as i32)) as i16) as u16;
            }
        }
    }
}

// /*
// ** Convert pMem so that it is of type MEM_Real.
// ** Invalidate any prior representations.
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3VdbeMemRealify(mut pMem: *mut sqlite3_value) -> i32 {
    0 as i32;
    0 as i32;
    0 as i32;
    unsafe {
        (*pMem).u.r = sqlite3VdbeRealValue(pMem);
    }
    unsafe {
        (*pMem).flags = (((((unsafe { (*pMem).flags }) as u32) as i32)
            & !((3519 as i32) | (1024 as i32))
            | (8 as i32)) as i16) as u16;
    }
    return 0 as i32;
}

// /*
// ** Convert pMem so that it has type MEM_Real or MEM_Int.
// ** Invalidate any prior representations.
// **
// ** Every effort is made to force the conversion, even if the input
// ** is a string that does not look completely like a number.  Convert
// ** as much of the string as we can and ignore the rest.
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3VdbeMemNumerify(mut pMem: *mut sqlite3_value) -> i32 {
    0 as i32;
    {}
    {}
    {}
    {}
    if (((unsafe { (*pMem).flags }) as u32) as i32)
        & ((4 as i32) | (8 as i32) | (32 as i32) | (1 as i32))
        == (0 as i32)
    {
        let mut rc: i32 = 0 as i32;
        let mut ix: i64 = 0 as i64;
        0 as i32;
        0 as i32;
        rc = sqlite3MemRealValueRC(pMem, unsafe { std::ptr::addr_of_mut!((*pMem).u.r) });
        let __v835: bool;
        if rc & (2 as i32) == (0 as i32) {
            __v835 = (unsafe {
                sqlite3Atoi64(
                    (unsafe { (*pMem).z }) as *const i8,
                    std::ptr::addr_of_mut!(ix),
                    unsafe { (*pMem).n },
                    unsafe { (*pMem).enc },
                )
            }) < (2 as i32);
        } else {
            __v835 = false as bool;
        }
        let __v836: bool;
        if __v835 {
            __v836 = true as bool;
        } else {
            let __v837: i64 = sqlite3RealToI64(unsafe { (*pMem).u.r });
            ix = __v837;
            __v836 = sqlite3RealSameAsInt(unsafe { (*pMem).u.r }, __v837) != (0 as i32);
        }
        if __v836 {
            unsafe {
                (*pMem).u.i = ix;
            }
            unsafe {
                (*pMem).flags = (((((unsafe { (*pMem).flags }) as u32) as i32)
                    & !((3519 as i32) | (1024 as i32))
                    | (4 as i32)) as i16) as u16;
            }
        } else {
            unsafe {
                (*pMem).flags = (((((unsafe { (*pMem).flags }) as u32) as i32)
                    & !((3519 as i32) | (1024 as i32))
                    | (8 as i32)) as i16) as u16;
            }
        }
    }
    0 as i32;
    let __v838: *mut sqlite3_value = pMem;
    let __v839: u16 = unsafe { (*__v838).flags };
    let __v840: u16 =
        ((((__v839 as u32) as i32) & !((2 as i32) | (16 as i32) | (1024 as i32))) as i16) as u16;
    unsafe {
        (*__v838).flags = __v840;
    }
    return 0 as i32;
}

// /*
// ** Cast the datatype of the value in pMem according to the affinity
// ** "aff".  Casting is different from applying affinity in that a cast
// ** is forced.  In other words, the value is converted into the desired
// ** affinity even if that results in loss of data.  This routine is
// ** used (for example) to implement the SQL "cast()" operator.
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3VdbeMemCast(
    mut pMem: *mut sqlite3_value,
    mut aff: u8,
    mut encoding: u8,
) -> i32 {
    if (((unsafe { (*pMem).flags }) as u32) as i32) & (1 as i32) != (0 as i32) {
        return 0 as i32;
    }
    match (aff as u32) as i32 {
        65 => {
            // /* Really a cast to BLOB */
            if (((unsafe { (*pMem).flags }) as u32) as i32) & (16 as i32) == (0 as i32) {
                unsafe { sqlite3ValueApplyAffinity(pMem, ((66 as i32) as i8) as u8, encoding) };
                0 as i32;
                if (((unsafe { (*pMem).flags }) as u32) as i32) & (2 as i32) != (0 as i32) {
                    unsafe {
                        (*pMem).flags = (((((unsafe { (*pMem).flags }) as u32) as i32)
                            & !((3519 as i32) | (1024 as i32))
                            | (16 as i32)) as i16) as u16;
                    }
                }
            } else {
                let __v841: *mut sqlite3_value = pMem;
                let __v842: u16 = unsafe { (*__v841).flags };
                let __v843: u16 =
                    ((((__v842 as u32) as i32) & !((3519 as i32) & !(16 as i32))) as i16) as u16;
                unsafe {
                    (*__v841).flags = __v843;
                }
            }
        }
        67 => {
            sqlite3VdbeMemNumerify(pMem);
        }
        68 => {
            sqlite3VdbeMemIntegerify(pMem);
        }
        69 => {
            sqlite3VdbeMemRealify(pMem);
        }
        _ => {
            let mut rc: i32 = 0 as i32;
            0 as i32;
            0 as i32;
            let __v844: *mut sqlite3_value = pMem;
            let __v845: u16 = unsafe { (*__v844).flags };
            let __v846: u16 = ((((__v845 as u32) as i32)
                | ((((unsafe { (*pMem).flags }) as u32) as i32) & (16 as i32)) >> (3 as i32))
                as i16) as u16;
            unsafe {
                (*__v844).flags = __v846;
            }
            unsafe { sqlite3ValueApplyAffinity(pMem, ((66 as i32) as i8) as u8, encoding) };
            0 as i32;
            let __v847: *mut sqlite3_value = pMem;
            let __v848: u16 = unsafe { (*__v847).flags };
            let __v849: u16 = ((((__v848 as u32) as i32)
                & !((4 as i32) | (8 as i32) | (32 as i32) | (16 as i32) | (1024 as i32)))
                as i16) as u16;
            unsafe {
                (*__v847).flags = __v849;
            }
            if ((encoding as u32) as i32) != (1 as i32) {
                let __v850: *mut sqlite3_value = pMem;
                let __v851: i32 = unsafe { (*__v850).n };
                let __v852: i32 = __v851 & !(1 as i32);
                unsafe {
                    (*__v850).n = __v852;
                }
            }
            rc = sqlite3VdbeChangeEncoding(pMem, (encoding as u32) as i32);
            if rc != (0 as i32) {
                return rc;
            }
            sqlite3VdbeMemZeroTerminateIfAble(pMem);
        }
    }
    return 0 as i32;
}

// /* Memory cell to set to string value */
// /* String pointer */
// /* Bytes in string, or negative */
// /* Destructor function */
// /*
// ** Move data out of a btree key or data field and into a Mem structure.
// ** The data is payload from the entry that pCur is currently pointing
// ** to.  offset and amt determine what portion of the data or key to retrieve.
// ** The result is written into the pMem element.
// **
// ** The pMem object must have been initialized.  This routine will use
// ** pMem->zMalloc to hold the content from the btree, if possible.  New
// ** pMem->zMalloc space will be allocated if necessary.  The calling routine
// ** is responsible for making sure that the pMem object is eventually
// ** destroyed.
// **
// ** If this routine fails for any reason (malloc returns NULL or unable
// ** to read from the disk) then the pMem is left in an inconsistent state.
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3VdbeMemFromBtree(
    mut pCur: *mut BtCursor,
    mut offset: u32,
    mut amt: u32,
    mut pMem: *mut sqlite3_value,
) -> i32 {
    let mut rc: i32 = 0 as i32;
    unsafe {
        (*pMem).flags = ((1 as i32) as i16) as u16;
    }
    {}
    {}
    if amt >= ((2147483391 as i32) as u32) {
        return 7 as i32;
    }
    if (amt as u64).wrapping_add(offset as u64)
        > ((unsafe { sqlite3BtreeMaxRecordSize(pCur) }) as u64)
    {
        return unsafe { sqlite3CorruptError(1476 as i32) };
    }
    let __v853: i32 =
        sqlite3VdbeMemClearAndResize(pMem, amt.wrapping_add((1 as i32) as u32) as i32);
    rc = __v853;
    if (0 as i32) == __v853 {
        rc = unsafe { sqlite3BtreePayload(pCur, offset, amt, (unsafe { (*pMem).z }) as *mut ()) };
        if rc == (0 as i32) {
            // /* Overrun area used when reading malformed records */
            unsafe {
                *unsafe { unsafe { (*pMem).z }.offset(amt as isize) } = (0 as i32) as i8;
            }
            unsafe {
                (*pMem).flags = ((16 as i32) as i16) as u16;
            }
            unsafe {
                (*pMem).n = amt as i32;
            }
        } else {
            sqlite3VdbeMemRelease(pMem);
        }
    }
    return rc;
}

// /* Cursor pointing at record to retrieve. */
// /* Offset from the start of data to return bytes from. */
// /* Number of bytes to return. */
// /* OUT: Return data in this Mem structure. */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3VdbeMemFromBtreeZeroOffset(
    mut pCur: *mut BtCursor,
    mut amt: u32,
    mut pMem: *mut sqlite3_value,
) -> i32 {
    // /* Number of bytes available on the local btree page */
    let mut available: u32 = (0 as i32) as u32;
    // /* Return code */
    let mut rc: i32 = 0 as i32;
    0 as i32;
    0 as i32;
    // /* Note: the calls to BtreeKeyFetch() and DataFetch() below assert()
    //   ** that both the BtShared and database handle mutexes are held. */
    0 as i32;
    unsafe {
        (*pMem).z = (unsafe { sqlite3BtreePayloadFetch(pCur, std::ptr::addr_of_mut!(available)) })
            as *mut i8;
    }
    0 as i32;
    if amt <= available {
        unsafe {
            (*pMem).flags = (((16 as i32) | (16384 as i32)) as i16) as u16;
        }
        unsafe {
            (*pMem).n = amt as i32;
        }
    } else {
        rc = sqlite3VdbeMemFromBtree(pCur, (0 as i32) as u32, amt, pMem);
    }
    return rc;
}

// /*
// ** Release any memory resources held by the Mem.  Both the memory that is
// ** free by Mem.xDel and the Mem.zMalloc allocation are freed.
// **
// ** Use this routine prior to clean up prior to abandoning a Mem, or to
// ** reset a Mem back to its minimum memory utilization.
// **
// ** Use sqlite3VdbeMemSetNull() to release just the Mem.xDel space
// ** prior to inserting new content into the Mem.
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3VdbeMemRelease(mut p: *mut sqlite3_value) {
    0 as i32;
    if (((unsafe { (*p).flags }) as u32) as i32) & ((32768 as i32) | (4096 as i32)) != (0 as i32)
        || (unsafe { (*p).szMalloc }) != (0 as i32)
    {
        vdbeMemClear(p);
    }
}

// /* Like sqlite3VdbeMemRelease() but faster for cases where we
// ** know in advance that the Mem is not MEM_Dyn or MEM_Agg.
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3VdbeMemReleaseMalloc(mut p: *mut sqlite3_value) {
    0 as i32;
    if (unsafe { (*p).szMalloc }) != (0 as i32) {
        vdbeMemClear(p);
    }
}

// /*
// ** Memory cell pMem contains the context of an aggregate function.
// ** This routine calls the finalize method for that function.  The
// ** result of the aggregate is stored back into pMem.
// **
// ** Return SQLITE_ERROR if the finalizer reports an error.  SQLITE_OK
// ** otherwise.
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3VdbeMemFinalize(
    mut pMem: *mut sqlite3_value,
    mut pFunc: *mut FuncDef,
) -> i32 {
    let mut ctx: sqlite3_context = unsafe { std::mem::zeroed() };
    let mut t: sqlite3_value = unsafe { std::mem::zeroed() };
    0 as i32;
    0 as i32;
    0 as i32;
    0 as i32;
    0 as i32;
    0 as i32;
    unsafe { memset(std::ptr::addr_of_mut!(ctx) as *mut (), 0 as i32, 48 as u64) };
    unsafe { memset(std::ptr::addr_of_mut!(t) as *mut (), 0 as i32, 56 as u64) };
    t.flags = ((1 as i32) as i16) as u16;
    t.db = unsafe { (*pMem).db };
    ctx.pOut = std::ptr::addr_of_mut!(t);
    ctx.pMem = pMem;
    ctx.pFunc = pFunc;
    ctx.enc = unsafe { (*t.db).enc };
    // /* IMP: R-24505-23230 */
    unsafe { unsafe { (*pFunc).xFinalize }.unwrap()(std::ptr::addr_of_mut!(ctx)) };
    0 as i32;
    if (unsafe { (*pMem).szMalloc }) > (0 as i32) {
        unsafe {
            sqlite3DbFreeNN(
                unsafe { (*pMem).db },
                (unsafe { (*pMem).zMalloc }) as *mut (),
            )
        };
    }
    unsafe {
        memcpy(
            pMem as *mut (),
            std::ptr::addr_of_mut!(t) as *const (),
            56 as u64,
        )
    };
    return ctx.isError;
}

// /*
// ** Memory cell pAccum contains the context of an aggregate function.
// ** This routine calls the xValue method for that function and stores
// ** the results in memory cell pMem.
// **
// ** SQLITE_ERROR is returned if xValue() reports an error. SQLITE_OK
// ** otherwise.
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3VdbeMemAggValue(
    mut pAccum: *mut sqlite3_value,
    mut pOut: *mut sqlite3_value,
    mut pFunc: *mut FuncDef,
) -> i32 {
    let mut ctx: sqlite3_context = unsafe { std::mem::zeroed() };
    0 as i32;
    0 as i32;
    0 as i32;
    0 as i32;
    0 as i32;
    unsafe { memset(std::ptr::addr_of_mut!(ctx) as *mut (), 0 as i32, 48 as u64) };
    sqlite3VdbeMemSetNull(pOut);
    ctx.pOut = pOut;
    ctx.pMem = pAccum;
    ctx.pFunc = pFunc;
    ctx.enc = unsafe { (*unsafe { (*pAccum).db }).enc };
    unsafe { unsafe { (*pFunc).xValue }.unwrap()(std::ptr::addr_of_mut!(ctx)) };
    return ctx.isError;
}

// /*
// ** Make sure pMem->z points to a writable allocation of at least n bytes.
// **
// ** If the bPreserve argument is true, then copy of the content of
// ** pMem->z into the new allocation.  pMem must be either a string or
// ** blob if bPreserve is true.  If bPreserve is false, any prior content
// ** in pMem->z is discarded.
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3VdbeMemGrow(
    mut pMem: *mut sqlite3_value,
    mut n: i32,
    mut bPreserve: i32,
) -> i32 {
    0 as i32;
    0 as i32;
    {}
    // /* If the bPreserve flag is set to true, then the memory cell must already
    //   ** contain a valid string or blob value.  */
    0 as i32;
    {}
    0 as i32;
    if (unsafe { (*pMem).szMalloc }) > (0 as i32)
        && bPreserve != (0 as i32)
        && (unsafe { (*pMem).z }) == unsafe { (*pMem).zMalloc }
    {
        if (unsafe { (*pMem).db }) != std::ptr::null_mut::<sqlite3>() {
            let __v854: *mut i8 = (unsafe {
                sqlite3DbReallocOrFree(
                    unsafe { (*pMem).db },
                    (unsafe { (*pMem).z }) as *mut (),
                    (n as i64) as u64,
                )
            }) as *mut i8;
            unsafe {
                (*pMem).zMalloc = __v854;
            }
            unsafe {
                (*pMem).z = __v854;
            }
        } else {
            unsafe {
                (*pMem).zMalloc = (unsafe {
                    sqlite3Realloc((unsafe { (*pMem).z }) as *mut (), (n as i64) as u64)
                }) as *mut i8;
            }
            if (unsafe { (*pMem).zMalloc }) == std::ptr::null_mut::<i8>() {
                unsafe { sqlite3_free((unsafe { (*pMem).z }) as *mut ()) };
            }
            unsafe {
                (*pMem).z = unsafe { (*pMem).zMalloc };
            }
        }
        bPreserve = 0 as i32;
    } else {
        if (unsafe { (*pMem).szMalloc }) > (0 as i32) {
            unsafe {
                sqlite3DbFreeNN(
                    unsafe { (*pMem).db },
                    (unsafe { (*pMem).zMalloc }) as *mut (),
                )
            };
        }
        unsafe {
            (*pMem).zMalloc =
                (unsafe { sqlite3DbMallocRaw(unsafe { (*pMem).db }, (n as i64) as u64) })
                    as *mut i8;
        }
    }
    if (unsafe { (*pMem).zMalloc }) == std::ptr::null_mut::<i8>() {
        sqlite3VdbeMemSetNull(pMem);
        unsafe {
            (*pMem).z = std::ptr::null_mut::<i8>();
        }
        unsafe {
            (*pMem).szMalloc = 0 as i32;
        }
        return 7 as i32;
    } else {
        unsafe {
            (*pMem).szMalloc = unsafe {
                sqlite3DbMallocSize(
                    unsafe { (*pMem).db },
                    (unsafe { (*pMem).zMalloc }) as *const (),
                )
            };
        }
    }
    if bPreserve != (0 as i32) && (unsafe { (*pMem).z }) != std::ptr::null_mut::<i8>() {
        0 as i32;
        unsafe {
            memcpy(
                (unsafe { (*pMem).zMalloc }) as *mut (),
                (unsafe { (*pMem).z }) as *const (),
                ((unsafe { (*pMem).n }) as i64) as u64,
            )
        };
    }
    if (((unsafe { (*pMem).flags }) as u32) as i32) & (4096 as i32) != (0 as i32) {
        0 as i32;
        unsafe { unsafe { (*pMem).xDel }.unwrap()((unsafe { (*pMem).z }) as *mut ()) };
    }
    unsafe {
        (*pMem).z = unsafe { (*pMem).zMalloc };
    }
    let __v855: *mut sqlite3_value = pMem;
    let __v856: u16 = unsafe { (*__v855).flags };
    let __v857: u16 = ((((__v856 as u32) as i32)
        & !((4096 as i32) | (16384 as i32) | (8192 as i32))) as i16) as u16;
    unsafe {
        (*__v855).flags = __v857;
    }
    return 0 as i32;
}

// /*
// ** Change the pMem->zMalloc allocation to be at least szNew bytes.
// ** If pMem->zMalloc already meets or exceeds the requested size, this
// ** routine is a no-op.
// **
// ** Any prior string or blob content in the pMem object may be discarded.
// ** The pMem->xDel destructor is called, if it exists.  Though MEM_Str
// ** and MEM_Blob values may be discarded, MEM_Int, MEM_Real, MEM_IntReal,
// ** and MEM_Null values are preserved.
// **
// ** Return SQLITE_OK on success or an error code (probably SQLITE_NOMEM)
// ** if unable to complete the resizing.
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3VdbeMemClearAndResize(
    mut pMem: *mut sqlite3_value,
    mut szNew: i32,
) -> i32 {
    0 as i32;
    0 as i32;
    if (unsafe { (*pMem).szMalloc }) < szNew {
        return sqlite3VdbeMemGrow(pMem, szNew, 0 as i32);
    }
    0 as i32;
    unsafe {
        (*pMem).z = unsafe { (*pMem).zMalloc };
    }
    let __v858: *mut sqlite3_value = pMem;
    let __v859: u16 = unsafe { (*__v858).flags };
    let __v860: u16 = ((((__v859 as u32) as i32)
        & ((1 as i32) | (4 as i32) | (8 as i32) | (32 as i32))) as i16)
        as u16;
    unsafe {
        (*__v858).flags = __v860;
    }
    return 0 as i32;
}

// /*
// ** If the given Mem* has a zero-filled tail, turn it into an ordinary
// ** blob stored in dynamically allocated space.
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3VdbeMemExpandBlob(mut pMem: *mut sqlite3_value) -> i32 {
    let mut nByte: i32 = 0 as i32;
    0 as i32;
    0 as i32;
    0 as i32;
    {}
    0 as i32;
    0 as i32;
    // /* Set nByte to the number of bytes required to store the expanded blob. */
    nByte = (unsafe { (*pMem).n }) + unsafe { (*pMem).u.nZero };
    if nByte <= (0 as i32) {
        if (((unsafe { (*pMem).flags }) as u32) as i32) & (16 as i32) == (0 as i32) {
            return 0 as i32;
        }
        nByte = 1 as i32;
    }
    if sqlite3VdbeMemGrow(pMem, nByte, 1 as i32) != (0 as i32) {
        return 7 as i32;
    }
    0 as i32;
    0 as i32;
    unsafe {
        memset(
            (unsafe { unsafe { (*pMem).z }.offset((unsafe { (*pMem).n }) as isize) }) as *mut (),
            0 as i32,
            ((unsafe { (*pMem).u.nZero }) as i64) as u64,
        )
    };
    let __v861: *mut sqlite3_value = pMem;
    let __v862: i32 = unsafe { (*__v861).n };
    let __v863: i32 = __v862 + unsafe { (*pMem).u.nZero };
    unsafe {
        (*__v861).n = __v863;
    }
    let __v864: *mut sqlite3_value = pMem;
    let __v865: u16 = unsafe { (*__v864).flags };
    let __v866: u16 = ((((__v865 as u32) as i32) & !((1024 as i32) | (512 as i32))) as i16) as u16;
    unsafe {
        (*__v864).flags = __v866;
    }
    return 0 as i32;
}

// /*
// ** 2004 May 26
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
// ** This file contains code use to manipulate "Mem" structure.  A "Mem"
// ** stores a single value in the VDBE.  Mem is an opaque structure visible
// ** only within the VDBE.  Interface routines refer to a Mem using the
// ** name sqlite_value
// */
// /* True if X is a power of two.  0 is considered a power of two here.
// ** In other words, return true if X has at most one bit set.
// */
// /*
// ** Render a Mem object which is one of MEM_Int, MEM_Real, or MEM_IntReal
// ** into a buffer.
// */
fn vdbeMemRenderNum(mut sz: i32, mut zBuf: *mut i8, mut p: *mut sqlite3_value) {
    let mut acc: sqlite3_str = unsafe { std::mem::zeroed() };
    0 as i32;
    0 as i32;
    if (((unsafe { (*p).flags }) as u32) as i32) & ((4 as i32) | (32 as i32)) != (0 as i32) {
        unsafe {
            (*p).n = unsafe { sqlite3Int64ToText(unsafe { (*p).u.i }, zBuf) };
        }
        if (((unsafe { (*p).flags }) as u32) as i32) & (32 as i32) != (0 as i32) {
            unsafe {
                memcpy(
                    (unsafe { zBuf.offset((unsafe { (*p).n }) as isize) }) as *mut (),
                    (b".0\0".as_ptr() as *mut i8) as *const (),
                    ((3 as i32) as i64) as u64,
                )
            };
            let __v867: *mut sqlite3_value = p;
            let __v868: i32 = unsafe { (*__v867).n };
            let __v869: i32 = __v868 + (2 as i32);
            unsafe {
                (*__v867).n = __v869;
            }
        }
    } else {
        unsafe {
            sqlite3StrAccumInit(
                std::ptr::addr_of_mut!(acc),
                std::ptr::null_mut::<sqlite3>(),
                zBuf,
                sz,
                0 as i32,
            )
        };
        unsafe {
            sqlite3_str_appendf(
                std::ptr::addr_of_mut!(acc),
                (b"%!.*g\0".as_ptr() as *mut i8) as *const i8,
                if (unsafe { (*p).db }) != std::ptr::null_mut::<sqlite3>() {
                    ((unsafe { (*unsafe { (*p).db }).nFpDigit }) as u32) as i32
                } else {
                    17 as i32
                },
                unsafe { (*p).u.r },
            )
        };
        0 as i32;
        // /* Fast version of sqlite3StrAccumFinish(&acc) */
        unsafe {
            *unsafe { zBuf.offset(acc.nChar as isize) } = (0 as i32) as i8;
        }
        unsafe {
            (*p).n = acc.nChar as i32;
        }
    }
}

// /*
// ** It is already known that pMem contains an unterminated string.
// ** Add the zero terminator.
// **
// ** Three bytes of zero are added.  In this way, there is guaranteed
// ** to be a double-zero byte at an even byte boundary in order to
// ** terminate a UTF16 string, even if the initial size of the buffer
// ** is an odd number of bytes.
// */
fn vdbeMemAddTerminator(mut pMem: *mut sqlite3_value) -> i32 {
    if sqlite3VdbeMemGrow(pMem, (unsafe { (*pMem).n }) + (3 as i32), 1 as i32) != (0 as i32) {
        return 7 as i32;
    }
    unsafe {
        *unsafe { unsafe { (*pMem).z }.offset((unsafe { (*pMem).n }) as isize) } = (0 as i32) as i8;
    }
    unsafe {
        *unsafe { unsafe { (*pMem).z }.offset(((unsafe { (*pMem).n }) + (1 as i32)) as isize) } =
            (0 as i32) as i8;
    }
    unsafe {
        *unsafe { unsafe { (*pMem).z }.offset(((unsafe { (*pMem).n }) + (2 as i32)) as isize) } =
            (0 as i32) as i8;
    }
    let __v870: *mut sqlite3_value = pMem;
    let __v871: u16 = unsafe { (*__v870).flags };
    let __v872: u16 = ((((__v871 as u32) as i32) | (512 as i32)) as i16) as u16;
    unsafe {
        (*__v870).flags = __v872;
    }
    return 0 as i32;
}

// /* SQLITE_OMIT_WINDOWFUNC */
// /*
// ** If the memory cell contains a value that must be freed by
// ** invoking the external callback in Mem.xDel, then this routine
// ** will free that value.  It also sets Mem.flags to MEM_Null.
// **
// ** This is a helper routine for sqlite3VdbeMemSetNull() and
// ** for sqlite3VdbeMemRelease().  Use those other routines as the
// ** entry point for releasing Mem resources.
// */
fn vdbeMemClearExternAndSetNull(mut p: *mut sqlite3_value) {
    0 as i32;
    0 as i32;
    if (((unsafe { (*p).flags }) as u32) as i32) & (32768 as i32) != (0 as i32) {
        sqlite3VdbeMemFinalize(p, unsafe { (*p).u.pDef });
        0 as i32;
        {}
    }
    if (((unsafe { (*p).flags }) as u32) as i32) & (4096 as i32) != (0 as i32) {
        0 as i32;
        unsafe { unsafe { (*p).xDel }.unwrap()((unsafe { (*p).z }) as *mut ()) };
    }
    unsafe {
        (*p).flags = ((1 as i32) as i16) as u16;
    }
}

// /*
// ** Release memory held by the Mem p, both external memory cleared
// ** by p->xDel and memory in p->zMalloc.
// **
// ** This is a helper routine invoked by sqlite3VdbeMemRelease() in
// ** the unusual case where there really is memory in p that needs
// ** to be freed.
// */
fn vdbeMemClear(mut p: *mut sqlite3_value) {
    if (((unsafe { (*p).flags }) as u32) as i32) & ((32768 as i32) | (4096 as i32)) != (0 as i32) {
        vdbeMemClearExternAndSetNull(p);
    }
    if (unsafe { (*p).szMalloc }) != (0 as i32) {
        unsafe { sqlite3DbFreeNN(unsafe { (*p).db }, (unsafe { (*p).zMalloc }) as *mut ()) };
        unsafe {
            (*p).szMalloc = 0 as i32;
        }
    }
    unsafe {
        (*p).z = std::ptr::null_mut::<i8>();
    }
}

// /*
// ** Return some kind of integer value which is the best we can do
// ** at representing the value that *pMem describes as an integer.
// ** If pMem is an integer, then the value is exact.  If pMem is
// ** a floating-point then the value returned is the integer part.
// ** If pMem is a string or blob, then we make an attempt to convert
// ** it into an integer and return that.  If pMem represents an
// ** an SQL-NULL value, return 0.
// **
// ** If pMem represents a string value, its encoding might be changed.
// */
fn memIntValue(mut pMem: *const sqlite3_value) -> i64 {
    let mut value: i64 = (0 as i32) as i64;
    unsafe {
        sqlite3Atoi64(
            (unsafe { (*pMem).z }) as *const i8,
            std::ptr::addr_of_mut!(value),
            unsafe { (*pMem).n },
            unsafe { (*pMem).enc },
        )
    };
    return value;
}

// /*
// ** This routine implements the uncommon and slower path for
// ** sqlite3MemRealValueRC() that has to deal with input strings
// ** that are not UTF8 or that are not zero-terminated.  It is
// ** broken out into a separate no-inline routine so that the
// ** main sqlite3MemRealValueRC() routine can avoid unnecessary
// ** stack pushes.
// **
// ** A text->float translation of pMem->z is written into *pValue.
// **
// ** Result code invariants:
// **
// **    rc==0         =>   ERROR: Input string not well-formed, or OOM
// **    rc<0          =>   Some prefix of the input is well-formed
// **    rc>0          =>   All of the input is well-formed
// **    (rc&2)==0     =>   The number is expressed as an integer, with no
// **                       decimal point or eNNN suffix.
// */
fn sqlite3MemRealValueRCSlowPath(mut pMem: *mut sqlite3_value, mut pValue: *mut f64) -> i32 {
    let mut rc: i32 = 0 as i32;
    unsafe {
        *pValue = 0.0f64;
    }
    if (((unsafe { (*pMem).enc }) as u32) as i32) == (1 as i32) {
        let mut zCopy: *mut i8 = unsafe {
            sqlite3DbStrNDup(
                unsafe { (*pMem).db },
                (unsafe { (*pMem).z }) as *const i8,
                ((unsafe { (*pMem).n }) as i64) as u64,
            )
        };
        if zCopy != std::ptr::null_mut::<i8>() {
            rc = unsafe { sqlite3AtoF(zCopy as *const i8, pValue) };
            unsafe { sqlite3DbFree(unsafe { (*pMem).db }, zCopy as *mut ()) };
        }
        return rc;
    } else {
        let mut n: i32 = 0 as i32;
        let mut i: i32 = 0 as i32;
        let mut j: i32 = 0 as i32;
        let mut zCopy: *mut i8 = unsafe { std::mem::zeroed() };
        let mut z: *const i8 = unsafe { std::mem::zeroed() };
        n = (unsafe { (*pMem).n }) & !(1 as i32);
        zCopy = (unsafe {
            sqlite3DbMallocRaw(
                unsafe { (*pMem).db },
                ((n / (2 as i32) + (2 as i32)) as i64) as u64,
            )
        }) as *mut i8;
        if zCopy != std::ptr::null_mut::<i8>() {
            z = (unsafe { (*pMem).z }) as *const i8;
            if (((unsafe { (*pMem).enc }) as u32) as i32) == (2 as i32) {
                j = 0 as i32;
                i = 0 as i32;
                '__slate_break_778: loop {
                    if !(i < n - (1 as i32)) {
                        break;
                    }
                    unsafe {
                        *unsafe { zCopy.offset(j as isize) } =
                            unsafe { *unsafe { z.offset(i as isize) } };
                    }
                    if ((unsafe { *unsafe { z.offset((i + (1 as i32)) as isize) } }) as i32)
                        != (0 as i32)
                    {
                        break '__slate_break_778;
                    }
                    let __v873: i32 = i;
                    let __v874: i32 = __v873 + (2 as i32);
                    i = __v874;
                    let __v875: i32 = j;
                    let __v876: i32 = __v875 + (1 as i32);
                    j = __v876;
                }
            } else {
                j = 0 as i32;
                i = 0 as i32;
                '__slate_break_779: loop {
                    if !(i < n - (1 as i32)) {
                        break;
                    }
                    if ((unsafe { *unsafe { z.offset(i as isize) } }) as i32) != (0 as i32) {
                        break '__slate_break_779;
                    }
                    unsafe {
                        *unsafe { zCopy.offset(j as isize) } =
                            unsafe { *unsafe { z.offset((i + (1 as i32)) as isize) } };
                    }
                    let __v877: i32 = i;
                    let __v878: i32 = __v877 + (2 as i32);
                    i = __v878;
                    let __v879: i32 = j;
                    let __v880: i32 = __v879 + (1 as i32);
                    j = __v880;
                }
            }
            0 as i32;
            unsafe {
                *unsafe { zCopy.offset(j as isize) } = (0 as i32) as i8;
            }
            rc = unsafe { sqlite3AtoF(zCopy as *const i8, pValue) };
            if i < n {
                rc = -(100 as i32);
            }
            unsafe { sqlite3DbFree(unsafe { (*pMem).db }, zCopy as *mut ()) };
        }
        return rc;
    }
    return unsafe { std::mem::zeroed() };
}

// /*
// ** This routine acts as a bridge from sqlite3VdbeRealValue() to
// ** sqlite3VdbeRealValueRC, allowing sqlite3VdbeRealValue() to avoid
// ** stuffing values onto the stack.
// */
fn sqlite3MemRealValueNoRC(mut pMem: *mut sqlite3_value) -> f64 {
    let mut r: f64 = 0 as f64;
    sqlite3MemRealValueRC(pMem, std::ptr::addr_of_mut!(r));
    return r;
}

// /*
// ** The pMem is known to contain content that needs to be destroyed prior
// ** to a value change.  So invoke the destructor, then set the value to
// ** a 64-bit integer.
// */
fn vdbeReleaseAndSetInt64(mut pMem: *mut sqlite3_value, mut val: i64) {
    sqlite3VdbeMemSetNull(pMem);
    unsafe {
        (*pMem).u.i = val;
    }
    unsafe {
        (*pMem).flags = ((4 as i32) as i16) as u16;
    }
}

// /*
// ** Make an shallow copy of pFrom into pTo.  Prior contents of
// ** pTo are freed.  The pFrom->z field is not duplicated.  If
// ** pFrom->z is used, then pTo->z points to the same thing as pFrom->z
// ** and flags gets srcType (either MEM_Ephem or MEM_Static).
// */
fn vdbeClrCopy(mut pTo: *mut sqlite3_value, mut pFrom: *const sqlite3_value, mut eType: i32) {
    vdbeMemClearExternAndSetNull(pTo);
    0 as i32;
    sqlite3VdbeMemShallowCopy(pTo, pFrom, eType);
}

// /* Cursor pointing at record to retrieve. */
// /* Number of bytes to return. */
// /* OUT: Return data in this Mem structure. */
// /*
// ** The pVal argument is known to be a value other than NULL.
// ** Convert it into a string with encoding enc and return a pointer
// ** to a zero-terminated version of that string.
// */
fn valueToText(mut pVal: *mut sqlite3_value, mut enc: u8) -> *const () {
    0 as i32;
    0 as i32;
    0 as i32;
    0 as i32;
    0 as i32;
    if (((unsafe { (*pVal).flags }) as u32) as i32) & ((16 as i32) | (2 as i32)) != (0 as i32) {
        let __v881: i32;
        if (((unsafe { (*pVal).flags }) as u32) as i32) & (1024 as i32) != (0 as i32) {
            __v881 = sqlite3VdbeMemExpandBlob(pVal);
        } else {
            __v881 = 0 as i32;
        }
        if __v881 != (0 as i32) {
            return std::ptr::null::<()>();
        }
        let __v882: *mut sqlite3_value = pVal;
        let __v883: u16 = unsafe { (*__v882).flags };
        let __v884: u16 = ((((__v883 as u32) as i32) | (2 as i32)) as i16) as u16;
        unsafe {
            (*__v882).flags = __v884;
        }
        if (((unsafe { (*pVal).enc }) as u32) as i32) != ((enc as u32) as i32) & !(8 as i32) {
            sqlite3VdbeChangeEncoding(pVal, ((enc as u32) as i32) & !(8 as i32));
        }
        if ((enc as u32) as i32) & (8 as i32) != (0 as i32)
            && (1 as i32) == (1 as i32) & (((unsafe { (*pVal).z }) as i64) as i32)
        {
            0 as i32;
            if sqlite3VdbeMemMakeWriteable(pVal) != (0 as i32) {
                return std::ptr::null::<()>();
            }
        }
        // /* IMP: R-31275-44060 */
        sqlite3VdbeMemNulTerminate(pVal);
    } else {
        sqlite3VdbeMemStringify(pVal, enc, ((0 as i32) as i8) as u8);
        0 as i32;
    }
    0 as i32;
    if (((unsafe { (*pVal).enc }) as u32) as i32) == ((enc as u32) as i32) & !(8 as i32) {
        0 as i32;
        return (unsafe { (*pVal).z }) as *const ();
    } else {
        return std::ptr::null::<()>();
    }
    return unsafe { std::mem::zeroed() };
}

// /*
// ** Allocate and return a pointer to a new sqlite3_value object. If
// ** the second argument to this function is NULL, the object is allocated
// ** by calling sqlite3ValueNew().
// **
// ** Otherwise, if the second argument is non-zero, then this function is
// ** being called indirectly by sqlite3Stat4ProbeSetValue(). If it has not
// ** already been allocated, allocate the UnpackedRecord structure that
// ** that function will return to its caller here. Then return a pointer to
// ** an sqlite3_value within the UnpackedRecord.a[] array.
// */
fn valueNew(mut db: *mut sqlite3, mut p: *mut ValueNewStat4Ctx) -> *mut sqlite3_value {
    p;
    // /* defined(SQLITE_ENABLE_STAT4) */
    return sqlite3ValueNew(db);
}

// /*
// ** The expression object indicated by the second argument is guaranteed
// ** to be a scalar SQL function. If
// **
// **   * all function arguments are SQL literals,
// **   * one of the SQLITE_FUNC_CONSTANT or _SLOCHNG function flags is set, and
// **   * the SQLITE_FUNC_NEEDCOLL function flag is not set,
// **
// ** then this routine attempts to invoke the SQL function. Assuming no
// ** error occurs, output parameter (*ppVal) is set to point to a value
// ** object containing the result before returning SQLITE_OK.
// **
// ** Affinity aff is applied to the result of the function before returning.
// ** If the result is a text value, the sqlite3_value object uses encoding
// ** enc.
// **
// ** If the conditions above are not met, this function returns SQLITE_OK
// ** and sets (*ppVal) to NULL. Or, if an error occurs, (*ppVal) is set to
// ** NULL and an SQLite error code returned.
// */
// /* defined(SQLITE_ENABLE_STAT4) */
// /*
// ** Extract a value from the supplied expression in the manner described
// ** above sqlite3ValueFromExpr(). Allocate the sqlite3_value object
// ** using valueNew().
// **
// ** If pCtx is NULL and an error occurs after the sqlite3_value object
// ** has been allocated, it is freed before returning. Or, if pCtx is not
// ** NULL, it is assumed that the caller will free any allocated object
// ** in all cases.
// */
fn valueFromExpr(
    mut db: *mut sqlite3,
    mut pExpr: *const Expr,
    mut enc: u8,
    mut affinity: u8,
    mut ppVal: *mut *mut sqlite3_value,
    mut pCtx: *mut ValueNewStat4Ctx,
) -> i32 {
    let mut __slate_storage_889: std::mem::MaybeUninit<u16> = std::mem::MaybeUninit::uninit();
    let __slate_slot_889: *mut u16 = std::ptr::addr_of_mut!(__slate_storage_889) as *mut u16;
    let mut __slate_storage_888: std::mem::MaybeUninit<u16> = std::mem::MaybeUninit::uninit();
    let __slate_slot_888: *mut u16 = std::ptr::addr_of_mut!(__slate_storage_888) as *mut u16;
    let mut __slate_storage_887: std::mem::MaybeUninit<*mut sqlite3_value> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_887: *mut *mut sqlite3_value =
        std::ptr::addr_of_mut!(__slate_storage_887) as *mut *mut sqlite3_value;
    let mut __slate_storage_886: std::mem::MaybeUninit<bool> = std::mem::MaybeUninit::uninit();
    let __slate_slot_886: *mut bool = std::ptr::addr_of_mut!(__slate_storage_886) as *mut bool;
    let mut __slate_storage_581: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_581: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_581) as *mut i64;
    let mut __slate_storage_582: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_582: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_582) as *mut i32;
    let mut __slate_storage_580: std::mem::MaybeUninit<*mut Expr> = std::mem::MaybeUninit::uninit();
    let __slate_slot_580: *mut *mut Expr =
        std::ptr::addr_of_mut!(__slate_storage_580) as *mut *mut Expr;
    let mut __slate_storage_579: std::mem::MaybeUninit<u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_579: *mut u8 = std::ptr::addr_of_mut!(__slate_storage_579) as *mut u8;
    let mut __slate_storage_885: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_885: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_885) as *mut i32;
    let mut __slate_storage_578: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_578: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_578) as *mut i32;
    let mut __slate_storage_577: std::mem::MaybeUninit<*const i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_577: *mut *const i8 =
        std::ptr::addr_of_mut!(__slate_storage_577) as *mut *const i8;
    let mut __slate_storage_576: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_576: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_576) as *mut i32;
    let mut __slate_storage_575: std::mem::MaybeUninit<*mut sqlite3_value> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_575: *mut *mut sqlite3_value =
        std::ptr::addr_of_mut!(__slate_storage_575) as *mut *mut sqlite3_value;
    let mut __slate_storage_574: std::mem::MaybeUninit<*mut i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_574: *mut *mut i8 =
        std::ptr::addr_of_mut!(__slate_storage_574) as *mut *mut i8;
    let mut __slate_storage_573: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_573: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_573) as *mut i32;
    unsafe {
        std::ptr::write(__slate_slot_574, std::ptr::null_mut::<i8>());
        std::ptr::write(__slate_slot_575, std::ptr::null_mut::<sqlite3_value>());
        std::ptr::write(__slate_slot_576, 1 as i32);
        std::ptr::write(__slate_slot_577, (b"\0".as_ptr() as *mut i8) as *const i8);
        std::ptr::write(__slate_slot_578, 0 as i32);
        0 as i32;
        loop {
            std::ptr::write(__slate_slot_885, ((unsafe { (*pExpr).op }) as u32) as i32);
            *__slate_slot_573 = *__slate_slot_885;
            if *__slate_slot_885 == (173 as i32) || *__slate_slot_573 == (181 as i32) {
                pExpr = (unsafe { (*pExpr).pLeft }) as *const Expr;
            } else {
                break;
            }
        }
        if *__slate_slot_573 == (176 as i32) {
            *__slate_slot_573 = ((unsafe { (*pExpr).op2 }) as u32) as i32;
        }
        // /* Compressed expressions only appear when parsing the DEFAULT clause
        //   ** on a table column definition, and hence only when pCtx==0.  This
        //   ** check ensures that an EP_TokenOnly expression is never passed down
        //   ** into valueFromFunction(). */
        0 as i32;
        if *__slate_slot_573 == (36 as i32) {
            0 as i32;
            *__slate_slot_579 = (unsafe {
                sqlite3AffinityType(
                    (unsafe { (*pExpr).u.zToken }) as *const i8,
                    std::ptr::null_mut::<Column>(),
                )
            }) as u8;
            *__slate_slot_578 = valueFromExpr(
                db,
                (unsafe { (*pExpr).pLeft }) as *const Expr,
                enc,
                *__slate_slot_579,
                ppVal,
                pCtx,
            );
            {}
            if (unsafe { *ppVal }) != std::ptr::null_mut::<sqlite3_value>() {
                // /* zero-blobs only come from functions, not literal values.  And
                //       ** functions are only processed under STAT4 */
                0 as i32;
                sqlite3VdbeMemCast(unsafe { *ppVal }, *__slate_slot_579, enc);
                unsafe { sqlite3ValueApplyAffinity(unsafe { *ppVal }, affinity, enc) };
            }
            return *__slate_slot_578;
        } else {
            // /* Handle negative integers in a single step.  This is needed in the
            //   ** case when the value is -9223372036854775808. Except - do not do this
            //   ** for hexadecimal literals.  */
            if *__slate_slot_573 == (174 as i32) {
                std::ptr::write(__slate_slot_580, unsafe { (*pExpr).pLeft });
                if (((unsafe { (*(*__slate_slot_580)).op }) as u32) as i32) == (156 as i32)
                    || (((unsafe { (*(*__slate_slot_580)).op }) as u32) as i32) == (154 as i32)
                {
                    if (unsafe { (*(*__slate_slot_580)).flags }) & ((2048 as i32) as u32)
                        != ((0 as i32) as u32)
                        || ((unsafe {
                            *unsafe {
                                unsafe { (*(*__slate_slot_580)).u.zToken }
                                    .offset((0 as i32) as isize)
                            }
                        }) as i32)
                            != (48 as i32)
                        || ((unsafe {
                            *unsafe {
                                unsafe { (*(*__slate_slot_580)).u.zToken }
                                    .offset((1 as i32) as isize)
                            }
                        }) as i32)
                            & !(32 as i32)
                            != (88 as i32)
                    {
                        pExpr = *__slate_slot_580 as *const Expr;
                        *__slate_slot_573 = ((unsafe { (*pExpr).op }) as u32) as i32;
                        *__slate_slot_576 = -(1 as i32);
                        *__slate_slot_577 = (b"-\0".as_ptr() as *mut i8) as *const i8;
                    }
                }
            }
            '__join_0: {
                if *__slate_slot_573 == (118 as i32)
                    || *__slate_slot_573 == (154 as i32)
                    || *__slate_slot_573 == (156 as i32)
                {
                    *__slate_slot_575 = valueNew(db, pCtx);
                    if *__slate_slot_575 == std::ptr::null_mut::<sqlite3_value>() {
                        break '__join_0;
                    } else {
                        if (unsafe { (*pExpr).flags }) & ((2048 as i32) as u32)
                            != ((0 as i32) as u32)
                        {
                            sqlite3VdbeMemSetInt64(
                                *__slate_slot_575,
                                ((unsafe { (*pExpr).u.iValue }) as i64)
                                    * (*__slate_slot_576 as i64),
                            );
                        } else {
                            '__join_16: {
                                if *__slate_slot_573 == (156 as i32) {
                                    *__slate_slot_886 = (0 as i32)
                                        == unsafe {
                                            sqlite3DecOrHexToI64(
                                                (unsafe { (*pExpr).u.zToken }) as *const i8,
                                                std::ptr::addr_of_mut!(*__slate_slot_581),
                                            )
                                        };
                                } else {
                                    *__slate_slot_886 = false as bool;
                                }
                            }
                            if *__slate_slot_886 {
                                sqlite3VdbeMemSetInt64(
                                    *__slate_slot_575,
                                    *__slate_slot_581 * (*__slate_slot_576 as i64),
                                );
                            } else {
                                *__slate_slot_574 = unsafe {
                                    sqlite3MPrintf(
                                        db,
                                        (b"%s%s\0".as_ptr() as *mut i8) as *const i8,
                                        *__slate_slot_577,
                                        unsafe { (*pExpr).u.zToken },
                                    )
                                };
                                if *__slate_slot_574 == std::ptr::null_mut::<i8>() {
                                    break '__join_0;
                                } else {
                                    sqlite3ValueSetStr(
                                        *__slate_slot_575,
                                        -(1 as i32),
                                        *__slate_slot_574 as *const (),
                                        ((1 as i32) as i8) as u8,
                                        unsafe {
                                            std::mem::transmute::<
                                                *const (),
                                                Option<unsafe extern "C-unwind" fn(*mut ())>,
                                            >(
                                                sqlite3RowSetClear as *const ()
                                            )
                                        },
                                    );
                                }
                            }
                        }
                        if ((affinity as u32) as i32) == (65 as i32) {
                            if *__slate_slot_573 == (154 as i32) {
                                0 as i32;
                                unsafe {
                                    sqlite3AtoF(
                                        (unsafe { (*(*__slate_slot_575)).z }) as *const i8,
                                        unsafe {
                                            std::ptr::addr_of_mut!((*(*__slate_slot_575)).u.r)
                                        },
                                    )
                                };
                                unsafe {
                                    (*(*__slate_slot_575)).flags = ((8 as i32) as i16) as u16;
                                }
                            } else {
                                if *__slate_slot_573 == (156 as i32) {
                                    // /* This case is required by -9223372036854775808 and other strings
                                    //         ** that look like integers but cannot be handled by the
                                    //         ** sqlite3DecOrHexToI64() call above.  */
                                    unsafe {
                                        sqlite3ValueApplyAffinity(
                                            *__slate_slot_575,
                                            ((67 as i32) as i8) as u8,
                                            ((1 as i32) as i8) as u8,
                                        )
                                    };
                                }
                            }
                        } else {
                            unsafe {
                                sqlite3ValueApplyAffinity(
                                    *__slate_slot_575,
                                    affinity,
                                    ((1 as i32) as i8) as u8,
                                )
                            };
                        }
                        0 as i32;
                        if (((unsafe { (*(*__slate_slot_575)).flags }) as u32) as i32)
                            & ((4 as i32) | (32 as i32) | (8 as i32))
                            != (0 as i32)
                        {
                            {}
                            {}
                            std::ptr::write(__slate_slot_887, *__slate_slot_575);
                            std::ptr::write(__slate_slot_888, unsafe {
                                (*(*__slate_slot_887)).flags
                            });
                            std::ptr::write(
                                __slate_slot_889,
                                ((((*__slate_slot_888 as u32) as i32) & !(2 as i32)) as i16) as u16,
                            );
                            unsafe {
                                (*(*__slate_slot_887)).flags = *__slate_slot_889;
                            }
                        }
                        if ((enc as u32) as i32) != (1 as i32) {
                            *__slate_slot_578 =
                                sqlite3VdbeChangeEncoding(*__slate_slot_575, (enc as u32) as i32);
                        }
                    }
                } else {
                    if *__slate_slot_573 == (174 as i32) {
                        // /* This branch happens for multiple negative signs.  Ex: -(-5) */
                        if (0 as i32)
                            == valueFromExpr(
                                db,
                                (unsafe { (*pExpr).pLeft }) as *const Expr,
                                enc,
                                affinity,
                                std::ptr::addr_of_mut!(*__slate_slot_575),
                                pCtx,
                            )
                            && *__slate_slot_575 != std::ptr::null_mut::<sqlite3_value>()
                        {
                            sqlite3VdbeMemNumerify(*__slate_slot_575);
                            if (((unsafe { (*(*__slate_slot_575)).flags }) as u32) as i32)
                                & (8 as i32)
                                != (0 as i32)
                            {
                                unsafe {
                                    (*(*__slate_slot_575)).u.r =
                                        -unsafe { (*(*__slate_slot_575)).u.r };
                                }
                            } else {
                                if (unsafe { (*(*__slate_slot_575)).u.i })
                                    == (-(1 as i32) as i64)
                                        - ((((4294967295 as u32) as u64) as i64)
                                            | ((2147483647 as i32) as i64) << (32 as i32))
                                {
                                    unsafe {
                                        (*(*__slate_slot_575)).u.r = -(((-(1 as i32) as i64)
                                            - ((((4294967295 as u32) as u64) as i64)
                                                | ((2147483647 as i32) as i64) << (32 as i32)))
                                            as f64);
                                    }
                                    unsafe {
                                        (*(*__slate_slot_575)).flags =
                                            (((((unsafe { (*(*__slate_slot_575)).flags }) as u32)
                                                as i32)
                                                & !((3519 as i32) | (1024 as i32))
                                                | (8 as i32))
                                                as i16)
                                                as u16;
                                    }
                                } else {
                                    unsafe {
                                        (*(*__slate_slot_575)).u.i =
                                            -unsafe { (*(*__slate_slot_575)).u.i };
                                    }
                                }
                            }
                            unsafe { sqlite3ValueApplyAffinity(*__slate_slot_575, affinity, enc) };
                        }
                    } else {
                        if *__slate_slot_573 == (122 as i32) {
                            *__slate_slot_575 = valueNew(db, pCtx);
                            if *__slate_slot_575 == std::ptr::null_mut::<sqlite3_value>() {
                                break '__join_0;
                            } else {
                                sqlite3VdbeMemSetNull(*__slate_slot_575);
                            }
                        } else {
                            if *__slate_slot_573 == (155 as i32) {
                                0 as i32;
                                0 as i32;
                                0 as i32;
                                *__slate_slot_575 = valueNew(db, pCtx);
                                if !(*__slate_slot_575 != std::ptr::null_mut::<sqlite3_value>()) {
                                    break '__join_0;
                                } else {
                                    *__slate_slot_574 = unsafe {
                                        unsafe { (*pExpr).u.zToken }.offset((2 as i32) as isize)
                                    };
                                    *__slate_slot_582 = (unsafe {
                                        sqlite3Strlen30(*__slate_slot_574 as *const i8)
                                    }) - (1 as i32);
                                    0 as i32;
                                    sqlite3VdbeMemSetStr(
                                        *__slate_slot_575,
                                        (unsafe {
                                            sqlite3HexToBlob(
                                                db,
                                                *__slate_slot_574 as *const i8,
                                                *__slate_slot_582,
                                            )
                                        }) as *const i8,
                                        (*__slate_slot_582 / (2 as i32)) as i64,
                                        ((0 as i32) as i8) as u8,
                                        unsafe {
                                            std::mem::transmute::<
                                                *const (),
                                                Option<unsafe extern "C-unwind" fn(*mut ())>,
                                            >(
                                                sqlite3RowSetClear as *const ()
                                            )
                                        },
                                    );
                                }
                            } else {
                                if *__slate_slot_573 == (171 as i32) {
                                    0 as i32;
                                    *__slate_slot_575 = valueNew(db, pCtx);
                                    if *__slate_slot_575 != std::ptr::null_mut::<sqlite3_value>() {
                                        unsafe {
                                            (*(*__slate_slot_575)).flags =
                                                ((4 as i32) as i16) as u16;
                                        }
                                        unsafe {
                                            (*(*__slate_slot_575)).u.i = (((unsafe {
                                                *unsafe {
                                                    unsafe { (*pExpr).u.zToken }
                                                        .offset((4 as i32) as isize)
                                                }
                                            })
                                                as i32)
                                                == (0 as i32))
                                                as i64;
                                        }
                                        unsafe {
                                            sqlite3ValueApplyAffinity(
                                                *__slate_slot_575,
                                                affinity,
                                                enc,
                                            )
                                        };
                                    }
                                }
                            }
                        }
                    }
                }
                unsafe {
                    *ppVal = *__slate_slot_575;
                }
                return *__slate_slot_578;
            }
            unsafe { sqlite3OomFault(db) };
            unsafe { sqlite3DbFree(db, *__slate_slot_574 as *mut ()) };
            0 as i32;
            0 as i32;
            sqlite3ValueFree(*__slate_slot_575);
            return 7 as i32;
        }
    }
    return unsafe { std::mem::zeroed() };
}

// /*
// ** The sqlite3ValueBytes() routine returns the number of bytes in the
// ** sqlite3_value object assuming that it uses the encoding "enc".
// ** The valueBytes() routine is a helper function.
// */
fn valueBytes(mut pVal: *mut sqlite3_value, mut enc: u8) -> i32 {
    return if valueToText(pVal, enc) != std::ptr::null::<()>() {
        unsafe { (*pVal).n }
    } else {
        0 as i32
    };
}
