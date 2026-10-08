//! 2008 August 16
//!
//! The author disclaims copyright to this source code.  In place of
//! a legal notice, here is a blessing:
//!
//!    May you do good and not evil.
//!    May you find forgiveness for yourself and forgive others.
//!    May you share freely, never taking more than you give.
//!
//!
//! This file contains routines used for walking the parser tree for
//! an SQL statement.
unsafe extern "C" {
    fn sqlite3SelectPopWith(__v351: *mut Walker, __v352: *mut Select);
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
    trace: __SlateRecord159,
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
    u1: __SlateRecord160,
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
    u: __SlateRecord161,
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
    __slate_bits_0: __slate_bits::__SlateBits60U0,
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
    u: __SlateRecord162,
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
    u: __SlateRecord170,
    pLeft: *mut Expr,
    pRight: *mut Expr,
    x: __SlateRecord171,
    nHeight: i32,
    iTable: i32,
    iColumn: i16,
    iAgg: i16,
    w: __SlateRecord172,
    pAggInfo: *mut AggInfo,
    y: __SlateRecord173,
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
    fg: __SlateRecord180,
    iCursor: i32,
    colUsed: u64,
    u1: __SlateRecord181,
    u2: __SlateRecord182,
    u3: __SlateRecord183,
    u4: __SlateRecord184,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct RenameToken {}

#[repr(C)]
#[derive(Clone, Copy)]
struct SrcList {
    nSrc: i32,
    nAlloc: u32,
    a: [SrcItem; 0],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct NameContext {
    pParse: *mut Parse,
    pSrcList: *mut SrcList,
    uNC: __SlateRecord185,
    pNext: *mut NameContext,
    nRef: i32,
    nNcErr: i32,
    ncFlags: i32,
    nNestedSelect: u32,
    pWinSelect: *mut Select,
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
struct TableLock {}

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
struct Walker {
    pParse: *mut Parse,
    xExprCallback: Option<unsafe extern "C-unwind" fn(*mut Walker, *mut Expr) -> i32>,
    xSelectCallback: Option<unsafe extern "C-unwind" fn(*mut Walker, *mut Select) -> i32>,
    xSelectCallback2: Option<unsafe extern "C-unwind" fn(*mut Walker, *mut Select)>,
    walkerDepth: i32,
    eCode: u16,
    mWFlags: u16,
    u: __SlateRecord190,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct VtabCtx {}

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
    __slate_bits_0: __slate_bits::__SlateBits158U0,
    azInit: *mut *const i8,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord159 {
    xLegacy: Option<unsafe extern "C-unwind" fn(*mut (), *const i8)>,
    xV2: Option<unsafe extern "C-unwind" fn(u32, *mut (), *mut (), *mut ()) -> i32>,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord160 {
    isInterrupted: i32,
    notUsed1: f64,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord161 {
    pHash: *mut FuncDef,
    pDestructor: *mut FuncDestructor,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord162 {
    tab: __SlateRecord163,
    view: __SlateRecord164,
    vtab: __SlateRecord165,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord163 {
    addColOffset: i32,
    pFKey: *mut FKey,
    pDfltList: *mut ExprList,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord164 {
    pSelect: *mut Select,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord165 {
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
union __SlateRecord170 {
    zToken: *mut i8,
    iValue: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord171 {
    pList: *mut ExprList,
    pSelect: *mut Select,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord172 {
    iJoin: i32,
    iOfst: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord173 {
    pTab: *mut Table,
    pWin: *mut Window,
    nReg: i32,
    sub: __SlateRecord174,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord174 {
    iAddr: i32,
    regReturn: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct ExprList_item {
    pExpr: *mut Expr,
    zEName: *mut i8,
    fg: __SlateRecord176,
    u: __SlateRecord177,
}

#[repr(C, align(4))]
#[derive(Clone, Copy)]
struct __SlateRecord176 {
    sortFlags: u8,
    __slate_bits_0: __slate_bits::__SlateBits176U0,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord177 {
    x: __SlateRecord178,
    iConstExprReg: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord178 {
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
struct __SlateRecord180 {
    jointype: u8,
    __slate_bits_0: __slate_bits::__SlateBits180U0,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord181 {
    zIndexedBy: *mut i8,
    pFuncArg: *mut ExprList,
    nRow: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord182 {
    pIBIndex: *mut Index,
    pCteUse: *mut CteUse,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord183 {
    pOn: *mut Expr,
    pUsing: *mut IdList,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord184 {
    pSchema: *mut Schema,
    zDatabase: *mut i8,
    pSubq: *mut Subquery,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord185 {
    pEList: *mut ExprList,
    pAggInfo: *mut AggInfo,
    pUpsert: *mut Upsert,
    iBaseReg: i32,
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
union __SlateRecord190 {
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
    pub struct __SlateBits176U0 {
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
    pub struct __SlateBits180U0 {
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
    pub struct __SlateBits158U0 {
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

/// Walk all expressions linked into the list of Window objects passed
/// as the second argument.
fn walkWindowList(mut pWalker: *mut Walker, mut pList: *mut Window, mut bOneOnly: i32) -> i32 {
    let mut pWin: *mut Window = unsafe { std::mem::zeroed() };
    pWin = pList;
    '__slate_break_353: while pWin != std::ptr::null_mut::<Window>() {
        let mut rc: i32 = 0 as i32;
        rc = sqlite3WalkExprList(pWalker, unsafe { (*pWin).pOrderBy });
        if rc != (0 as i32) {
            return 2 as i32;
        }
        rc = sqlite3WalkExprList(pWalker, unsafe { (*pWin).pPartition });
        if rc != (0 as i32) {
            return 2 as i32;
        }
        rc = sqlite3WalkExpr(pWalker, unsafe { (*pWin).pFilter });
        if rc != (0 as i32) {
            return 2 as i32;
        }
        rc = sqlite3WalkExpr(pWalker, unsafe { (*pWin).pStart });
        if rc != (0 as i32) {
            return 2 as i32;
        }
        rc = sqlite3WalkExpr(pWalker, unsafe { (*pWin).pEnd });
        if rc != (0 as i32) {
            return 2 as i32;
        }
        if bOneOnly != (0 as i32) {
            break '__slate_break_353;
        }
        pWin = unsafe { (*pWin).pNextWin };
    }
    return 0 as i32;
}

/// Walk an expression tree.  Invoke the callback once for each node
/// of the expression, while descending.  (In other words, the callback
/// is invoked before visiting children.)
///
/// The return value from the callback should be one of the WRC_*
/// constants to specify how to proceed with the walk.
///
///    WRC_Continue      Continue descending down the tree.
///
///    WRC_Prune         Do not descend into child nodes, but allow
///                      the walk to continue with sibling nodes.
///
///    WRC_Abort         Do no more callbacks.  Unwind the stack and
///                      return from the top-level walk call.
///
/// The return value from this routine is WRC_Abort to abandon the tree walk
/// and WRC_Continue to continue.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3WalkExprNN(mut pWalker: *mut Walker, mut pExpr: *mut Expr) -> i32 {
    let mut rc: i32 = 0 as i32;
    {}
    {}
    '__slate_break_354: while (1 as i32) != (0 as i32) {
        '__slate_continue_354: {
            rc = unsafe { unsafe { (*pWalker).xExprCallback }.unwrap()(pWalker, pExpr) };
            if rc != (0 as i32) {
                return rc & (2 as i32);
            }
            if !((unsafe { (*pExpr).flags }) & (((65536 as i32) | (8388608 as i32)) as u32)
                != ((0 as i32) as u32))
            {
                0 as i32;
                let __v359: bool;
                if (unsafe { (*pExpr).pLeft }) != std::ptr::null_mut::<Expr>() {
                    __v359 = sqlite3WalkExprNN(pWalker, unsafe { (*pExpr).pLeft }) != (0 as i32);
                } else {
                    __v359 = false as bool;
                }
                if __v359 {
                    return 2 as i32;
                }
                if (unsafe { (*pExpr).pRight }) != std::ptr::null_mut::<Expr>() {
                    0 as i32;
                    pExpr = unsafe { (*pExpr).pRight };
                    break '__slate_continue_354;
                } else {
                    if (unsafe { (*pExpr).flags }) & ((4096 as i32) as u32) != ((0 as i32) as u32) {
                        0 as i32;
                        if sqlite3WalkSelect(pWalker, unsafe { (*pExpr).x.pSelect }) != (0 as i32) {
                            return 2 as i32;
                        }
                    } else {
                        if (unsafe { (*pExpr).x.pList }) != std::ptr::null_mut::<ExprList>() {
                            if sqlite3WalkExprList(pWalker, unsafe { (*pExpr).x.pList })
                                != (0 as i32)
                            {
                                return 2 as i32;
                            }
                        }
                        if (unsafe { (*pExpr).flags }) & ((16777216 as i32) as u32)
                            != ((0 as i32) as u32)
                        {
                            if walkWindowList(pWalker, unsafe { (*pExpr).y.pWin }, 1 as i32)
                                != (0 as i32)
                            {
                                return 2 as i32;
                            }
                        }
                    }
                }
            }
            break '__slate_break_354;
        }
    }
    return 0 as i32;
}

#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3WalkExpr(mut pWalker: *mut Walker, mut pExpr: *mut Expr) -> i32 {
    let __v358: i32;
    if pExpr != std::ptr::null_mut::<Expr>() {
        __v358 = sqlite3WalkExprNN(pWalker, pExpr);
    } else {
        __v358 = 0 as i32;
    }
    return __v358;
}

/// Call sqlite3WalkExpr() for every expression in list p or until
/// an abort request is seen.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3WalkExprList(mut pWalker: *mut Walker, mut p: *mut ExprList) -> i32 {
    let mut i: i32 = 0 as i32;
    let mut pItem: *mut ExprList_item = unsafe { std::mem::zeroed() };
    if p != std::ptr::null_mut::<ExprList>() {
        i = unsafe { (*p).nExpr };
        let __v360: *mut ExprList_item =
            unsafe { std::ptr::addr_of_mut!((*p).a) as *mut ExprList_item };
        pItem = __v360;
        '__slate_break_355: while i > (0 as i32) {
            if sqlite3WalkExpr(pWalker, unsafe { (*pItem).pExpr }) != (0 as i32) {
                return 2 as i32;
            }
            let __v361: i32 = i;
            let __v362: i32 = __v361 - (1 as i32);
            i = __v362;
            let __v363: *mut ExprList_item = pItem;
            let __v364: *mut ExprList_item = unsafe { __v363.offset((1 as i32) as isize) };
            pItem = __v364;
        }
    }
    return 0 as i32;
}

/// This is a no-op callback for Walker->xSelectCallback2.  If this
/// callback is set, then the Select->pWinDefn list is traversed.
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.walker.sqlite3WalkWinDefnDummyCallback")]
extern "C-unwind" fn sqlite3WalkWinDefnDummyCallback(mut pWalker: *mut Walker, mut p: *mut Select) {
    pWalker;
    p;
    // No-op
}

/// Walk all expressions associated with SELECT statement p.  Do
/// not invoke the SELECT callback on p, but do (of course) invoke
/// any expr callbacks and SELECT callbacks that come from subqueries.
/// Return WRC_Abort or WRC_Continue.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3WalkSelectExpr(mut pWalker: *mut Walker, mut p: *mut Select) -> i32 {
    if sqlite3WalkExprList(pWalker, unsafe { (*p).pEList }) != (0 as i32) {
        return 2 as i32;
    }
    if sqlite3WalkExpr(pWalker, unsafe { (*p).pWhere }) != (0 as i32) {
        return 2 as i32;
    }
    if sqlite3WalkExprList(pWalker, unsafe { (*p).pGroupBy }) != (0 as i32) {
        return 2 as i32;
    }
    if sqlite3WalkExpr(pWalker, unsafe { (*p).pHaving }) != (0 as i32) {
        return 2 as i32;
    }
    if sqlite3WalkExprList(pWalker, unsafe { (*p).pOrderBy }) != (0 as i32) {
        return 2 as i32;
    }
    if sqlite3WalkExpr(pWalker, unsafe { (*p).pLimit }) != (0 as i32) {
        return 2 as i32;
    }
    if (unsafe { (*p).pWinDefn }) != std::ptr::null_mut::<Window>() {
        let mut pParse: *mut Parse = unsafe { std::mem::zeroed() };
        let __v366: bool;
        if (unsafe { (*pWalker).xSelectCallback2 }) == Some(sqlite3WalkWinDefnDummyCallback) {
            __v366 = true as bool;
        } else {
            let __v367: *mut Parse = unsafe { (*pWalker).pParse };
            pParse = __v367;
            __v366 = __v367 != std::ptr::null_mut::<Parse>()
                && (((unsafe { (*pParse).eParseMode }) as u32) as i32) >= (2 as i32);
        }
        if __v366
            || (unsafe { (*pWalker).xSelectCallback2 })
                == unsafe {
                    std::mem::transmute::<
                        *const (),
                        Option<unsafe extern "C-unwind" fn(*mut Walker, *mut Select)>,
                    >(sqlite3SelectPopWith as *const ())
                }
        {
            // The following may return WRC_Abort if there are unresolvable
            // symbols (e.g. a table that does not exist) in a window definition.
            let mut rc: i32 = walkWindowList(pWalker, unsafe { (*p).pWinDefn }, 0 as i32);
            return rc;
        }
    }
    return 0 as i32;
}

/// Walk the parse trees associated with all subqueries in the
/// FROM clause of SELECT statement p.  Do not invoke the select
/// callback on p, but do invoke it on each FROM clause subquery
/// and on any subqueries further down in the tree.  Return
/// WRC_Abort or WRC_Continue;
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3WalkSelectFrom(mut pWalker: *mut Walker, mut p: *mut Select) -> i32 {
    let mut pSrc: *mut SrcList = unsafe { std::mem::zeroed() };
    let mut i: i32 = 0 as i32;
    let mut pItem: *mut SrcItem = unsafe { std::mem::zeroed() };
    pSrc = unsafe { (*p).pSrc };
    if pSrc != std::ptr::null_mut::<SrcList>() {
        i = unsafe { (*pSrc).nSrc };
        let __v368: *mut SrcItem = unsafe { std::ptr::addr_of_mut!((*pSrc).a) as *mut SrcItem };
        pItem = __v368;
        '__slate_break_356: while i > (0 as i32) {
            let __v373: bool;
            if ((unsafe { (*pItem).fg.__slate_bits_0.__get_isSubquery() }) as i32) != (0 as i32) {
                __v373 =
                    sqlite3WalkSelect(pWalker, unsafe { (*unsafe { (*pItem).u4.pSubq }).pSelect })
                        != (0 as i32);
            } else {
                __v373 = false as bool;
            }
            if __v373 {
                return 2 as i32;
            }
            let __v374: bool;
            if ((unsafe { (*pItem).fg.__slate_bits_0.__get_isTabFunc() }) as i32) != (0 as i32) {
                __v374 =
                    sqlite3WalkExprList(pWalker, unsafe { (*pItem).u1.pFuncArg }) != (0 as i32);
            } else {
                __v374 = false as bool;
            }
            if __v374 {
                return 2 as i32;
            }
            let __v369: i32 = i;
            let __v370: i32 = __v369 - (1 as i32);
            i = __v370;
            let __v371: *mut SrcItem = pItem;
            let __v372: *mut SrcItem = unsafe { __v371.offset((1 as i32) as isize) };
            pItem = __v372;
        }
    }
    return 0 as i32;
}

/// Call sqlite3WalkExpr() for every expression in Select statement p.
/// Invoke sqlite3WalkSelect() for subqueries in the FROM clause and
/// on the compound select chain, p->pPrior.
///
/// If it is not NULL, the xSelectCallback() callback is invoked before
/// the walk of the expressions and FROM clause. The xSelectCallback2()
/// method is invoked following the walk of the expressions and FROM clause,
/// but only if both xSelectCallback and xSelectCallback2 are both non-NULL
/// and if the expressions and FROM clause both return WRC_Continue;
///
/// Return WRC_Continue under normal conditions.  Return WRC_Abort if
/// there is an abort request.
///
/// If the Walker does not have an xSelectCallback() then this routine
/// is a no-op returning WRC_Continue.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3WalkSelect(mut pWalker: *mut Walker, mut p: *mut Select) -> i32 {
    let mut rc: i32 = 0 as i32;
    if p == std::ptr::null_mut::<Select>() {
        return 0 as i32;
    }
    if (unsafe { (*pWalker).xSelectCallback }) == None {
        return 0 as i32;
    }
    '__slate_break_357: loop {
        rc = unsafe { unsafe { (*pWalker).xSelectCallback }.unwrap()(pWalker, p) };
        if rc != (0 as i32) {
            return rc & (2 as i32);
        }
        let __v365: bool;
        if sqlite3WalkSelectExpr(pWalker, p) != (0 as i32) {
            __v365 = true as bool;
        } else {
            __v365 = sqlite3WalkSelectFrom(pWalker, p) != (0 as i32);
        }
        if __v365 {
            return 2 as i32;
        }
        if (unsafe { (*pWalker).xSelectCallback2 }) != None {
            unsafe { unsafe { (*pWalker).xSelectCallback2 }.unwrap()(pWalker, p) };
        }
        p = unsafe { (*p).pPrior };
        if !(p != std::ptr::null_mut::<Select>()) {
            break;
        }
    }
    return 0 as i32;
}

/// Increase the walkerDepth when entering a subquery, and
/// decrease when leaving the subquery.
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.walker.sqlite3WalkerDepthIncrease")]
extern "C-unwind" fn sqlite3WalkerDepthIncrease(
    mut pWalker: *mut Walker,
    mut pSelect: *mut Select,
) -> i32 {
    pSelect;
    let __v375: *mut Walker = pWalker;
    let __v376: i32 = unsafe { (*__v375).walkerDepth };
    let __v377: i32 = __v376 + (1 as i32);
    unsafe {
        (*__v375).walkerDepth = __v377;
    }
    return 0 as i32;
}

#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.walker.sqlite3WalkerDepthDecrease")]
extern "C-unwind" fn sqlite3WalkerDepthDecrease(
    mut pWalker: *mut Walker,
    mut pSelect: *mut Select,
) {
    pSelect;
    let __v378: *mut Walker = pWalker;
    let __v379: i32 = unsafe { (*__v378).walkerDepth };
    let __v380: i32 = __v379 - (1 as i32);
    unsafe {
        (*__v378).walkerDepth = __v380;
    }
}

/// No-op routine for the parse-tree walker.
///
/// When this routine is the Walker.xExprCallback then expression trees
/// are walked without any actions being taken at each node.  Presumably,
/// when this routine is used for Walker.xExprCallback then
/// Walker.xSelectCallback is set to do something useful for every
/// subquery in the parser tree.
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.walker.sqlite3ExprWalkNoop")]
extern "C-unwind" fn sqlite3ExprWalkNoop(mut NotUsed: *mut Walker, mut NotUsed2: *mut Expr) -> i32 {
    NotUsed;
    NotUsed2;
    return 0 as i32;
}

/// No-op routine for the parse-tree walker for SELECT statements.
/// subquery in the parser tree.
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.walker.sqlite3SelectWalkNoop")]
extern "C-unwind" fn sqlite3SelectWalkNoop(
    mut NotUsed: *mut Walker,
    mut NotUsed2: *mut Select,
) -> i32 {
    NotUsed;
    NotUsed2;
    return 0 as i32;
}
