//! 2001 September 15
//!
//! The author disclaims copyright to this source code.  In place of
//! a legal notice, here is a blessing:
//!
//!    May you do good and not evil.
//!    May you find forgiveness for yourself and forgive others.
//!    May you share freely, never taking more than you give.
//!
//!
//! This file contains the sqlite3_get_table() and sqlite3_free_table()
//! interface routines.  These are just wrappers around the main
//! interface routine of sqlite3_exec().
//!
//! These routines are in a separate files so that they will not be linked
//! if they are not used.
unsafe extern "C" {
    fn sqlite3_exec(
        __v314: *mut sqlite3,
        sql: *const i8,
        callback: Option<
            unsafe extern "C-unwind" fn(*mut (), i32, *mut *mut i8, *mut *mut i8) -> i32,
        >,
        __v317: *mut (),
        errmsg: *mut *mut i8,
    ) -> i32;
    fn sqlite3_mprintf(__v326: *const i8, ...) -> *mut i8;
    fn sqlite3_malloc64(__v327: u64) -> *mut ();
    fn sqlite3_free(__v328: *mut ());
    fn memcpy(__dest: *mut (), __src: *const (), __n: u64) -> *mut ();
    fn sqlite3Strlen30(__v332: *const i8) -> i32;
    fn sqlite3Realloc(__v333: *mut (), __v334: u64) -> *mut ();
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
struct sqlite3_mutex {}

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
struct sqlite3_value {}

#[repr(C)]
#[derive(Clone, Copy)]
struct sqlite3_context {}

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
struct sqlite3_vtab {
    pModule: *const sqlite3_module,
    nRef: i32,
    zErrMsg: *mut i8,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct sqlite3_vtab_cursor {
    pVtab: *mut sqlite3_vtab,
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
struct HashElem {
    next: *mut HashElem,
    prev: *mut HashElem,
    data: *mut (),
    pKey: *const i8,
    h: u32,
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
struct _ht {
    count: u32,
    chain: *mut HashElem,
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
    zComment: *mut i8,
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
struct Db {
    zDbSName: *mut i8,
    pBt: *mut Btree,
    safety_level: u8,
    bSyncSet: u8,
    pSchema: *mut Schema,
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
    trace: __SlateRecord155,
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
    u1: __SlateRecord156,
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
    u: __SlateRecord157,
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
struct Savepoint {
    zName: *mut i8,
    nDeferredCons: i64,
    nDeferredImmCons: i64,
    pNext: *mut Savepoint,
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
struct CollSeq {
    zName: *mut i8,
    enc: u8,
    pUser: *mut (),
    xCmp: Option<unsafe extern "C-unwind" fn(*mut (), i32, *const (), i32, *const ()) -> i32>,
    xDel: Option<unsafe extern "C-unwind" fn(*mut ())>,
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
    u: __SlateRecord158,
    pTrigger: *mut Trigger,
    pSchema: *mut Schema,
    aHx: [u8; 16],
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
    __slate_bits_0: __slate_bits::__SlateBits86U0,
    colNotIdxed: u64,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct Token {
    z: *const i8,
    n: u32,
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
struct Expr {
    op: u8,
    affExpr: i8,
    op2: u8,
    flags: u32,
    u: __SlateRecord166,
    pLeft: *mut Expr,
    pRight: *mut Expr,
    x: __SlateRecord167,
    nHeight: i32,
    iTable: i32,
    iColumn: i16,
    iAgg: i16,
    w: __SlateRecord168,
    pAggInfo: *mut AggInfo,
    y: __SlateRecord169,
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
struct IdList {
    nId: i32,
    a: [IdList_item; 0],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct RenameToken {}

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
    fg: __SlateRecord176,
    iCursor: i32,
    colUsed: u64,
    u1: __SlateRecord177,
    u2: __SlateRecord178,
    u3: __SlateRecord179,
    u4: __SlateRecord180,
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
struct AutoincInfo {
    pNext: *mut AutoincInfo,
    pTab: *mut Table,
    iDb: i32,
    regCtr: i32,
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
struct TableLock {}

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
    zIdxName: *const i8,
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
    __slate_bits_0: __slate_bits::__SlateBits98U0,
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
    u1: __SlateRecord182,
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
struct VtabCtx {}

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
struct DbClientData {
    pNext: *mut DbClientData,
    pData: *mut (),
    xDestructor: Option<unsafe extern "C-unwind" fn(*mut ())>,
    zName: [i8; 0],
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
    __slate_bits_0: __slate_bits::__SlateBits154U0,
    azInit: *mut *const i8,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord155 {
    xLegacy: Option<unsafe extern "C-unwind" fn(*mut (), *const i8)>,
    xV2: Option<unsafe extern "C-unwind" fn(u32, *mut (), *mut (), *mut ()) -> i32>,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord156 {
    isInterrupted: i32,
    notUsed1: f64,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord157 {
    pHash: *mut FuncDef,
    pDestructor: *mut FuncDestructor,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord158 {
    tab: __SlateRecord159,
    view: __SlateRecord160,
    vtab: __SlateRecord161,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord159 {
    addColOffset: i32,
    pFKey: *mut FKey,
    pDfltList: *mut ExprList,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord160 {
    pSelect: *mut Select,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord161 {
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
union __SlateRecord166 {
    zToken: *mut i8,
    iValue: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord167 {
    pList: *mut ExprList,
    pSelect: *mut Select,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord168 {
    iJoin: i32,
    iOfst: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord169 {
    pTab: *mut Table,
    pWin: *mut Window,
    nReg: i32,
    sub: __SlateRecord170,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord170 {
    iAddr: i32,
    regReturn: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct ExprList_item {
    pExpr: *mut Expr,
    zEName: *mut i8,
    fg: __SlateRecord172,
    u: __SlateRecord173,
}

#[repr(C, align(4))]
#[derive(Clone, Copy)]
struct __SlateRecord172 {
    sortFlags: u8,
    __slate_bits_0: __slate_bits::__SlateBits172U0,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord173 {
    x: __SlateRecord174,
    iConstExprReg: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord174 {
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
struct __SlateRecord176 {
    jointype: u8,
    __slate_bits_0: __slate_bits::__SlateBits176U0,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord177 {
    zIndexedBy: *mut i8,
    pFuncArg: *mut ExprList,
    nRow: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord178 {
    pIBIndex: *mut Index,
    pCteUse: *mut CteUse,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord179 {
    pOn: *mut Expr,
    pUsing: *mut IdList,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord180 {
    pSchema: *mut Schema,
    zDatabase: *mut i8,
    pSubq: *mut Subquery,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord182 {
    cr: __SlateRecord183,
    d: __SlateRecord184,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord183 {
    addrCrTab: i32,
    regRowid: i32,
    regRoot: i32,
    constraintName: Token,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord184 {
    pReturning: *mut Returning,
}

/// This structure is used to pass data from sqlite3_get_table() through
/// to the callback function is uses to build the result.
#[repr(C)]
#[derive(Clone, Copy)]
struct TabResult {
    /// Accumulated output
    azResult: *mut *mut i8,
    /// Error message text, if an error occurs
    zErrMsg: *mut i8,
    /// Slots allocated for azResult[]
    nAlloc: u32,
    /// Number of rows in the result
    nRow: u32,
    /// Number of columns in the result
    nColumn: u32,
    /// Slots used in azResult[].  (nRow+1)*nColumn
    nData: u32,
    /// Return code from sqlite3_exec()
    rc: i32,
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
    pub struct __SlateBits172U0 {
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
    pub struct __SlateBits176U0 {
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
    pub struct __SlateBits86U0 {
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
    pub struct __SlateBits154U0 {
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
    pub struct __SlateBits98U0 {
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

/// This routine is called once for each row in the result table.  Its job
/// is to fill in the TabResult structure appropriately, allocating new
/// memory as necessary.
#[unsafe(link_section = ".text.slate_distinct.table.sqlite3_get_table_cb")]
extern "C-unwind" fn sqlite3_get_table_cb(
    mut pArg: *mut (),
    mut nCol: i32,
    mut argv: *mut *mut i8,
    mut colv: *mut *mut i8,
) -> i32 {
    let mut __slate_storage_357: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_357: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_357) as *mut u32;
    let mut __slate_storage_356: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_356: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_356) as *mut u32;
    let mut __slate_storage_355: std::mem::MaybeUninit<*mut TabResult> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_355: *mut *mut TabResult =
        std::ptr::addr_of_mut!(__slate_storage_355) as *mut *mut TabResult;
    let mut __slate_storage_351: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_351: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_351) as *mut i32;
    let mut __slate_storage_350: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_350: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_350) as *mut i32;
    let mut __slate_storage_354: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_354: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_354) as *mut u32;
    let mut __slate_storage_353: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_353: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_353) as *mut u32;
    let mut __slate_storage_352: std::mem::MaybeUninit<*mut TabResult> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_352: *mut *mut TabResult =
        std::ptr::addr_of_mut!(__slate_storage_352) as *mut *mut TabResult;
    let mut __slate_storage_301: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_301: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_301) as *mut i32;
    let mut __slate_storage_346: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_346: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_346) as *mut i32;
    let mut __slate_storage_345: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_345: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_345) as *mut i32;
    let mut __slate_storage_349: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_349: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_349) as *mut u32;
    let mut __slate_storage_348: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_348: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_348) as *mut u32;
    let mut __slate_storage_347: std::mem::MaybeUninit<*mut TabResult> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_347: *mut *mut TabResult =
        std::ptr::addr_of_mut!(__slate_storage_347) as *mut *mut TabResult;
    let mut __slate_storage_300: std::mem::MaybeUninit<*mut *mut i8> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_300: *mut *mut *mut i8 =
        std::ptr::addr_of_mut!(__slate_storage_300) as *mut *mut *mut i8; // A single column of result
    let mut __slate_storage_299: std::mem::MaybeUninit<*mut i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_299: *mut *mut i8 =
        std::ptr::addr_of_mut!(__slate_storage_299) as *mut *mut i8; // Loop counter
    let mut __slate_storage_298: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_298: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_298) as *mut i32; // Slots needed in p->azResult[]
    let mut __slate_storage_297: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_297: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_297) as *mut i32; // Result accumulator
    let mut __slate_storage_296: std::mem::MaybeUninit<*mut TabResult> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_296: *mut *mut TabResult =
        std::ptr::addr_of_mut!(__slate_storage_296) as *mut *mut TabResult;
    unsafe {
        '__join_20: {
            std::ptr::write(__slate_slot_296, pArg as *mut TabResult);
            // Make sure there is enough space in p->azResult to hold everything
            // we need to remember from this invocation of the callback.
            if (unsafe { (*(*__slate_slot_296)).nRow }) == ((0 as i32) as u32)
                && argv != std::ptr::null_mut::<*mut i8>()
            {
                *__slate_slot_297 = nCol * (2 as i32);
            } else {
                *__slate_slot_297 = nCol;
            }
        }
        '__join_0: {
            if unsafe { (*(*__slate_slot_296)).nData }.wrapping_add(*__slate_slot_297 as u32)
                > unsafe { (*(*__slate_slot_296)).nAlloc }
            {
                unsafe {
                    (*(*__slate_slot_296)).nAlloc = unsafe { (*(*__slate_slot_296)).nAlloc }
                        .wrapping_mul((2 as i32) as u32)
                        .wrapping_add(*__slate_slot_297 as u32);
                }
                *__slate_slot_300 = (unsafe {
                    sqlite3Realloc(
                        (unsafe { (*(*__slate_slot_296)).azResult }) as *mut (),
                        (8 as u64).wrapping_mul((unsafe { (*(*__slate_slot_296)).nAlloc }) as u64),
                    )
                }) as *mut *mut i8;
                if *__slate_slot_300 == std::ptr::null_mut::<*mut i8>() {
                    break '__join_0;
                } else {
                    unsafe {
                        (*(*__slate_slot_296)).azResult = *__slate_slot_300;
                    }
                }
            }
            '__join_10: {
                // If this is the first row, then generate an extra row containing
                // the names of all columns.
                if (unsafe { (*(*__slate_slot_296)).nRow }) == ((0 as i32) as u32) {
                    unsafe {
                        (*(*__slate_slot_296)).nColumn = nCol as u32;
                    }
                    *__slate_slot_298 = 0 as i32;
                    loop {
                        if *__slate_slot_298 < nCol {
                            *__slate_slot_299 = unsafe {
                                sqlite3_mprintf(
                                    (b"%s\0".as_ptr() as *mut i8) as *const i8,
                                    unsafe { *unsafe { colv.offset(*__slate_slot_298 as isize) } },
                                )
                            };
                            if *__slate_slot_299 == std::ptr::null_mut::<i8>() {
                                break '__join_0;
                            } else {
                                std::ptr::write(__slate_slot_347, *__slate_slot_296);
                                std::ptr::write(__slate_slot_348, unsafe {
                                    (*(*__slate_slot_347)).nData
                                });
                                std::ptr::write(
                                    __slate_slot_349,
                                    (*__slate_slot_348).wrapping_add((1 as i32) as u32),
                                );
                                unsafe {
                                    (*(*__slate_slot_347)).nData = *__slate_slot_349;
                                }
                                unsafe {
                                    *unsafe {
                                        unsafe { (*(*__slate_slot_296)).azResult }
                                            .offset(*__slate_slot_348 as isize)
                                    } = *__slate_slot_299;
                                }
                                std::ptr::write(__slate_slot_345, *__slate_slot_298);
                                std::ptr::write(__slate_slot_346, *__slate_slot_345 + (1 as i32));
                                *__slate_slot_298 = *__slate_slot_346;
                            }
                        } else {
                            break '__join_10;
                        }
                    }
                } else {
                    if ((unsafe { (*(*__slate_slot_296)).nColumn }) as i32) != nCol {
                        unsafe {
                            sqlite3_free((unsafe { (*(*__slate_slot_296)).zErrMsg }) as *mut ())
                        };
                        unsafe {
                            (*(*__slate_slot_296)).zErrMsg = unsafe {
                                sqlite3_mprintf((b"sqlite3_get_table() called with two or more incompatible queries\0".as_ptr() as *mut i8) as *const i8)
                            };
                        }
                        unsafe {
                            (*(*__slate_slot_296)).rc = 1 as i32;
                        }
                        return 1 as i32;
                    }
                }
            }
            // Copy over the row data
            if argv != std::ptr::null_mut::<*mut i8>() {
                *__slate_slot_298 = 0 as i32;
                loop {
                    if *__slate_slot_298 < nCol {
                        if (unsafe { *unsafe { argv.offset(*__slate_slot_298 as isize) } })
                            == std::ptr::null_mut::<i8>()
                        {
                            *__slate_slot_299 = std::ptr::null_mut::<i8>();
                        } else {
                            std::ptr::write(
                                __slate_slot_301,
                                (unsafe {
                                    sqlite3Strlen30(
                                        (unsafe {
                                            *unsafe { argv.offset(*__slate_slot_298 as isize) }
                                        }) as *const i8,
                                    )
                                }) + (1 as i32),
                            );
                            *__slate_slot_299 =
                                (unsafe { sqlite3_malloc64((*__slate_slot_301 as i64) as u64) })
                                    as *mut i8;
                            if *__slate_slot_299 == std::ptr::null_mut::<i8>() {
                                break '__join_0;
                            } else {
                                unsafe {
                                    memcpy(
                                        *__slate_slot_299 as *mut (),
                                        (unsafe {
                                            *unsafe { argv.offset(*__slate_slot_298 as isize) }
                                        }) as *const (),
                                        (*__slate_slot_301 as i64) as u64,
                                    )
                                };
                            }
                        }
                        std::ptr::write(__slate_slot_352, *__slate_slot_296);
                        std::ptr::write(__slate_slot_353, unsafe { (*(*__slate_slot_352)).nData });
                        std::ptr::write(
                            __slate_slot_354,
                            (*__slate_slot_353).wrapping_add((1 as i32) as u32),
                        );
                        unsafe {
                            (*(*__slate_slot_352)).nData = *__slate_slot_354;
                        }
                        unsafe {
                            *unsafe {
                                unsafe { (*(*__slate_slot_296)).azResult }
                                    .offset(*__slate_slot_353 as isize)
                            } = *__slate_slot_299;
                        }
                        std::ptr::write(__slate_slot_350, *__slate_slot_298);
                        std::ptr::write(__slate_slot_351, *__slate_slot_350 + (1 as i32));
                        *__slate_slot_298 = *__slate_slot_351;
                    } else {
                        break;
                    }
                }
                std::ptr::write(__slate_slot_355, *__slate_slot_296);
                std::ptr::write(__slate_slot_356, unsafe { (*(*__slate_slot_355)).nRow });
                std::ptr::write(
                    __slate_slot_357,
                    (*__slate_slot_356).wrapping_add((1 as i32) as u32),
                );
                unsafe {
                    (*(*__slate_slot_355)).nRow = *__slate_slot_357;
                }
            }
            return 0 as i32;
        }
        unsafe {
            (*(*__slate_slot_296)).rc = 7 as i32;
        }
        return 1 as i32;
    }
    return unsafe { std::mem::zeroed() };
}

/// Query the database.  But instead of invoking a callback for each row,
/// malloc() for space to hold the result and return the entire results
/// at the conclusion of the call.
///
/// The result that is written to ***pazResult is held in memory obtained
/// from malloc().  But the caller cannot free this memory directly.
/// Instead, the entire table should be passed to sqlite3_free_table() when
/// the calling procedure is finished using it.
///
/// # Arguments
///
/// * `db` - The database on which the SQL executes
/// * `zSql` - The SQL to be executed
/// * `pazResult` - Write the result table here
/// * `pnRow` - Write the number of rows in the result here
/// * `pnColumn` - Write the number of columns of result here
/// * `pzErrMsg` - Write error messages here
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.table.sqlite3_get_table")]
extern "C-unwind" fn sqlite3_get_table(
    mut db: *mut sqlite3,
    mut zSql: *const i8,
    mut pazResult: *mut *mut *mut i8,
    mut pnRow: *mut i32,
    mut pnColumn: *mut i32,
    mut pzErrMsg: *mut *mut i8,
) -> i32 {
    let mut rc: i32 = 0 as i32;
    let mut res: TabResult = unsafe { std::mem::zeroed() };
    unsafe {
        *pazResult = std::ptr::null_mut::<*mut i8>();
    }
    if pnColumn != std::ptr::null_mut::<i32>() {
        unsafe {
            *pnColumn = 0 as i32;
        }
    }
    if pnRow != std::ptr::null_mut::<i32>() {
        unsafe {
            *pnRow = 0 as i32;
        }
    }
    if pzErrMsg != std::ptr::null_mut::<*mut i8>() {
        unsafe {
            *pzErrMsg = std::ptr::null_mut::<i8>();
        }
    }
    res.zErrMsg = std::ptr::null_mut::<i8>();
    res.nRow = (0 as i32) as u32;
    res.nColumn = (0 as i32) as u32;
    res.nData = (1 as i32) as u32;
    res.nAlloc = (20 as i32) as u32;
    res.rc = 0 as i32;
    res.azResult =
        (unsafe { sqlite3_malloc64((8 as u64).wrapping_mul(res.nAlloc as u64)) }) as *mut *mut i8;
    if res.azResult == std::ptr::null_mut::<*mut i8>() {
        unsafe {
            (*db).errCode = 7 as i32;
        }
        return 7 as i32;
    }
    unsafe {
        *unsafe { res.azResult.offset((0 as i32) as isize) } = std::ptr::null_mut::<i8>();
    }
    rc = unsafe {
        sqlite3_exec(
            db,
            zSql,
            Some(sqlite3_get_table_cb),
            std::ptr::addr_of_mut!(res) as *mut (),
            pzErrMsg,
        )
    };
    0 as i32;
    unsafe {
        *unsafe { res.azResult.offset((0 as i32) as isize) } =
            (((res.nData as u64) as i64) as *mut ()) as *mut i8;
    }
    if rc & (255 as i32) == (4 as i32) {
        sqlite3_free_table(unsafe { res.azResult.offset((1 as i32) as isize) });
        if res.zErrMsg != std::ptr::null_mut::<i8>() {
            if pzErrMsg != std::ptr::null_mut::<*mut i8>() {
                unsafe { sqlite3_free((unsafe { *pzErrMsg }) as *mut ()) };
                unsafe {
                    *pzErrMsg = unsafe {
                        sqlite3_mprintf((b"%s\0".as_ptr() as *mut i8) as *const i8, res.zErrMsg)
                    };
                }
            }
            unsafe { sqlite3_free(res.zErrMsg as *mut ()) };
        }
        unsafe {
            (*db).errCode = res.rc;
        }
        // Assume 32-bit assignment is atomic
        return res.rc;
    }
    unsafe { sqlite3_free(res.zErrMsg as *mut ()) };
    if rc != (0 as i32) {
        sqlite3_free_table(unsafe { res.azResult.offset((1 as i32) as isize) });
        return rc;
    }
    if res.nAlloc > res.nData {
        let mut azNew: *mut *mut i8 = unsafe { std::mem::zeroed() };
        azNew = (unsafe {
            sqlite3Realloc(
                res.azResult as *mut (),
                (8 as u64).wrapping_mul(res.nData as u64),
            )
        }) as *mut *mut i8;
        if azNew == std::ptr::null_mut::<*mut i8>() {
            sqlite3_free_table(unsafe { res.azResult.offset((1 as i32) as isize) });
            unsafe {
                (*db).errCode = 7 as i32;
            }
            return 7 as i32;
        }
        res.azResult = azNew;
    }
    unsafe {
        *pazResult = unsafe { res.azResult.offset((1 as i32) as isize) };
    }
    if pnColumn != std::ptr::null_mut::<i32>() {
        unsafe {
            *pnColumn = res.nColumn as i32;
        }
    }
    if pnRow != std::ptr::null_mut::<i32>() {
        unsafe {
            *pnRow = res.nRow as i32;
        }
    }
    return rc;
}

/// This routine frees the space the sqlite3_get_table() malloced.
///
/// # Arguments
///
/// * `azResult` - Result returned from sqlite3_get_table()
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.table.sqlite3_free_table")]
extern "C-unwind" fn sqlite3_free_table(mut azResult: *mut *mut i8) {
    if azResult != std::ptr::null_mut::<*mut i8>() {
        let mut i: i32 = 0 as i32;
        let mut n: i32 = 0 as i32;
        let __v341: *mut *mut i8 = azResult;
        let __v342: *mut *mut i8 = unsafe { __v341.offset(-((1 as i32) as isize)) };
        azResult = __v342;
        0 as i32;
        n = ((unsafe { *unsafe { azResult.offset((0 as i32) as isize) } }) as i64) as i32;
        i = 1 as i32;
        '__slate_break_340: loop {
            if !(i < n) {
                break;
            }
            if (unsafe { *unsafe { azResult.offset(i as isize) } }) != std::ptr::null_mut::<i8>() {
                unsafe {
                    sqlite3_free((unsafe { *unsafe { azResult.offset(i as isize) } }) as *mut ())
                };
            }
            let __v343: i32 = i;
            let __v344: i32 = __v343 + (1 as i32);
            i = __v344;
        }
        unsafe { sqlite3_free(azResult as *mut ()) };
    }
}
