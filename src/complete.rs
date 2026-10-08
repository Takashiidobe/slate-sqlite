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
//! An tokenizer for SQL
//!
//! This file contains C code that implements the sqlite3_complete() API.
//! This code used to be part of the tokenizer.c source file.  But by
//! separating it out, the code will be automatically omitted from
//! static links that do not use it.
unsafe extern "C" {
    static mut sqlite3CtypeMap: [u8; 0];
    fn sqlite3_initialize() -> i32;
    fn sqlite3_strnicmp(__v296: *const i8, __v297: *const i8, __v298: i32) -> i32;
    fn sqlite3ValueText(__v299: *mut sqlite3_value, __v300: u8) -> *const ();
    fn sqlite3ValueSetStr(
        __v301: *mut sqlite3_value,
        __v302: i32,
        __v303: *const (),
        __v304: u8,
        __v305: Option<unsafe extern "C-unwind" fn(*mut ())>,
    );
    fn sqlite3ValueFree(__v306: *mut sqlite3_value);
    fn sqlite3ValueNew(__v307: *mut sqlite3) -> *mut sqlite3_value;
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
    trace: __SlateRecord154,
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
    u1: __SlateRecord155,
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
    u: __SlateRecord156,
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
    __slate_bits_0: __slate_bits::__SlateBits61U0,
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
    u: __SlateRecord157,
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
    __slate_bits_0: __slate_bits::__SlateBits85U0,
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
    u: __SlateRecord165,
    pLeft: *mut Expr,
    pRight: *mut Expr,
    x: __SlateRecord166,
    nHeight: i32,
    iTable: i32,
    iColumn: i16,
    iAgg: i16,
    w: __SlateRecord167,
    pAggInfo: *mut AggInfo,
    y: __SlateRecord168,
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
    fg: __SlateRecord175,
    iCursor: i32,
    colUsed: u64,
    u1: __SlateRecord176,
    u2: __SlateRecord177,
    u3: __SlateRecord178,
    u4: __SlateRecord179,
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
    __slate_bits_0: __slate_bits::__SlateBits97U0,
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
    u1: __SlateRecord181,
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
    __slate_bits_0: __slate_bits::__SlateBits153U0,
    azInit: *mut *const i8,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord154 {
    xLegacy: Option<unsafe extern "C-unwind" fn(*mut (), *const i8)>,
    xV2: Option<unsafe extern "C-unwind" fn(u32, *mut (), *mut (), *mut ()) -> i32>,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord155 {
    isInterrupted: i32,
    notUsed1: f64,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord156 {
    pHash: *mut FuncDef,
    pDestructor: *mut FuncDestructor,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord157 {
    tab: __SlateRecord158,
    view: __SlateRecord159,
    vtab: __SlateRecord160,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord158 {
    addColOffset: i32,
    pFKey: *mut FKey,
    pDfltList: *mut ExprList,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord159 {
    pSelect: *mut Select,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord160 {
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
union __SlateRecord165 {
    zToken: *mut i8,
    iValue: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord166 {
    pList: *mut ExprList,
    pSelect: *mut Select,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord167 {
    iJoin: i32,
    iOfst: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord168 {
    pTab: *mut Table,
    pWin: *mut Window,
    nReg: i32,
    sub: __SlateRecord169,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord169 {
    iAddr: i32,
    regReturn: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct ExprList_item {
    pExpr: *mut Expr,
    zEName: *mut i8,
    fg: __SlateRecord171,
    u: __SlateRecord172,
}

#[repr(C, align(4))]
#[derive(Clone, Copy)]
struct __SlateRecord171 {
    sortFlags: u8,
    __slate_bits_0: __slate_bits::__SlateBits171U0,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord172 {
    x: __SlateRecord173,
    iConstExprReg: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord173 {
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
struct __SlateRecord175 {
    jointype: u8,
    __slate_bits_0: __slate_bits::__SlateBits175U0,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord176 {
    zIndexedBy: *mut i8,
    pFuncArg: *mut ExprList,
    nRow: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord177 {
    pIBIndex: *mut Index,
    pCteUse: *mut CteUse,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord178 {
    pOn: *mut Expr,
    pUsing: *mut IdList,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord179 {
    pSchema: *mut Schema,
    zDatabase: *mut i8,
    pSubq: *mut Subquery,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord181 {
    cr: __SlateRecord182,
    d: __SlateRecord183,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord182 {
    addrCrTab: i32,
    regRowid: i32,
    regRoot: i32,
    constraintName: Token,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord183 {
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
    pub struct __SlateBits61U0 {
        #[bits(4)]
        pub notNull: u32,
        #[bits(4)]
        pub eCType: u32,
    }
    #[bitfields::bitfield([u8; 2], c_names = true, new = false, from_into_bits = false, from_traits = false, default = false, debug = false, builder = false, bit_ops = false)]
    pub struct __SlateBits171U0 {
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
    pub struct __SlateBits175U0 {
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
    pub struct __SlateBits85U0 {
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
    pub struct __SlateBits153U0 {
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
    pub struct __SlateBits97U0 {
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

// This is defined in tokenize.c.  We just have to import the definition.
// Token types used by the sqlite3_complete() routine.  See the header
// comments on that procedure for additional information.
/// Return zero if the given SQL string is complete - if all comments,
/// string and blob literals, and quoted identifiers have been closed and
/// if the entire string ends with ";" and possible with ";END;" if the
/// string is a CREATE TRIGGER statement.  A non-zero return indicates
/// that the string is incomplete.  Bits of the return value indicate
/// what is missing and is needed to close out the statement.
///
/// Special handling is require for CREATE TRIGGER statements.
/// Whenever the CREATE TRIGGER keywords are seen, the statement
/// must end with ";END;".
///
/// Let the return code be a value R.  R is split up into various
/// subfields, at byte boundaries:
///
///    R = 0xwwwwwwww00xxyyzz
///
/// In other words, zz is the least significant byte, yy is the next
/// most significant byte, xx is the third byte, wwwwwwww is a 32-bit
/// value from the middle.
///
///   zz == SQLITE_OK       Input is complete
///   zz == SQLITE_ERROR    Input is incomplete
///   zz == SQLITE_MISUSE   Input is a NULL pointer
///   zz != 0               New values for zz may be added in the future
///
///   yy == 0x01            Need a semicolon at the end
///   yy == 0x02            Need "END" and a semicolon
///   yy == 0x03            Need semicolon, "END", and semicolon
///   yy != 0               New values for yy may be added in the future
///
///   xx == '\''            Incomplete string or blob literal
///   xx == '"'             Incomplete quoted identifier
///   xx == '`'             Incompelte MySQL-style quoted identifier
///   xx == ']'             Incomplete SQLServer-style quoted identifer
///   xx == '-'             Incomplete SQL-style comment
///   xx == '/'             Incomplete C-style comment
///   xx != 0               New values of xx may be added in the future
///
///   wwwwwwww              Interpret as a signed integer, the number
///                         of unmatched "(".  Negative means there are
///                         more ")" and "(".
///
///   ((R>>24)&0xff)!=0     New uses for the 4th byte may be added
///                         in the future
///
/// This implementation uses a state machine with 8 states:
///
///   (0) INVALID   We have not yet seen a non-whitespace character.
///
///   (1) START     At the beginning or end of an SQL statement.  This routine
///                 returns 1 if it ends in the START state and 0 if it ends
///                 in any other state.
///
///   (2) NORMAL    We are in the middle of statement which ends with a single
///                 semicolon.
///
///   (3) EXPLAIN   The keyword EXPLAIN has been seen at the beginning of
///                 a statement.
///
///   (4) CREATE    The keyword CREATE has been seen at the beginning of a
///                 statement, possibly preceded by EXPLAIN and/or followed by
///                 TEMP or TEMPORARY
///
///   (5) TRIGGER   We are in the middle of a trigger definition that must be
///                 ended by a semicolon, the keyword END, and another semicolon.
///
///   (6) SEMI      We've seen the first semicolon in the ";END;" that occurs at
///                 the end of a trigger definition.
///
///   (7) END       We've seen the ";END" of the ";END;" that occurs at the end
///                 of a trigger definition.
///
/// Transitions between states above are determined by tokens extracted
/// from the input.  The following tokens are significant:
///
///   (0) tkSEMI      A semicolon.
///   (1) tkWS        Whitespace.
///   (2) tkOTHER     Any other SQL token.
///   (3) tkEXPLAIN   The "explain" keyword.
///   (4) tkCREATE    The "create" keyword.
///   (5) tkTEMP      The "temp" or "temporary" keyword.
///   (6) tkTRIGGER   The "trigger" keyword.
///   (7) tkEND       The "end" keyword.
///
/// Whitespace never causes a state transition and is always ignored.
/// This means that a SQL string of all whitespace is invalid.
///
/// If we compile with SQLITE_OMIT_TRIGGER, all of the computation needed
/// to recognize the end of a trigger can be omitted.  All we have to do
/// is look for a semicolon that is not part of an string or comment.
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.complete.sqlite3_incomplete")]
extern "C-unwind" fn sqlite3_incomplete(mut zSql: *const i8) -> i64 {
    let mut __slate_storage_353: std::mem::MaybeUninit<*const i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_353: *mut *const i8 =
        std::ptr::addr_of_mut!(__slate_storage_353) as *mut *const i8;
    let mut __slate_storage_352: std::mem::MaybeUninit<*const i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_352: *mut *const i8 =
        std::ptr::addr_of_mut!(__slate_storage_352) as *mut *const i8;
    let mut __slate_storage_351: std::mem::MaybeUninit<*const i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_351: *mut *const i8 =
        std::ptr::addr_of_mut!(__slate_storage_351) as *mut *const i8;
    let mut __slate_storage_350: std::mem::MaybeUninit<*const i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_350: *mut *const i8 =
        std::ptr::addr_of_mut!(__slate_storage_350) as *mut *const i8;
    let mut __slate_storage_349: std::mem::MaybeUninit<bool> = std::mem::MaybeUninit::uninit();
    let __slate_slot_349: *mut bool = std::ptr::addr_of_mut!(__slate_storage_349) as *mut bool;
    let mut __slate_storage_348: std::mem::MaybeUninit<bool> = std::mem::MaybeUninit::uninit();
    let __slate_slot_348: *mut bool = std::ptr::addr_of_mut!(__slate_storage_348) as *mut bool;
    let mut __slate_storage_347: std::mem::MaybeUninit<bool> = std::mem::MaybeUninit::uninit();
    let __slate_slot_347: *mut bool = std::ptr::addr_of_mut!(__slate_storage_347) as *mut bool;
    let mut __slate_storage_346: std::mem::MaybeUninit<bool> = std::mem::MaybeUninit::uninit();
    let __slate_slot_346: *mut bool = std::ptr::addr_of_mut!(__slate_storage_346) as *mut bool;
    let mut __slate_storage_345: std::mem::MaybeUninit<bool> = std::mem::MaybeUninit::uninit();
    let __slate_slot_345: *mut bool = std::ptr::addr_of_mut!(__slate_storage_345) as *mut bool;
    let mut __slate_storage_344: std::mem::MaybeUninit<bool> = std::mem::MaybeUninit::uninit();
    let __slate_slot_344: *mut bool = std::ptr::addr_of_mut!(__slate_storage_344) as *mut bool;
    let mut __slate_storage_343: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_343: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_343) as *mut i32;
    let mut __slate_storage_342: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_342: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_342) as *mut i32;
    // Keywords and unquoted identifiers
    let mut __slate_storage_287: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_287: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_287) as *mut i32;
    let mut __slate_storage_341: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_341: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_341) as *mut i32;
    let mut __slate_storage_340: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_340: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_340) as *mut i32;
    let mut __slate_storage_339: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_339: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_339) as *mut i32;
    let mut __slate_storage_338: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_338: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_338) as *mut i32;
    let mut __slate_storage_337: std::mem::MaybeUninit<*const i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_337: *mut *const i8 =
        std::ptr::addr_of_mut!(__slate_storage_337) as *mut *const i8;
    let mut __slate_storage_336: std::mem::MaybeUninit<*const i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_336: *mut *const i8 =
        std::ptr::addr_of_mut!(__slate_storage_336) as *mut *const i8;
    let mut __slate_storage_335: std::mem::MaybeUninit<*const i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_335: *mut *const i8 =
        std::ptr::addr_of_mut!(__slate_storage_335) as *mut *const i8;
    let mut __slate_storage_334: std::mem::MaybeUninit<*const i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_334: *mut *const i8 =
        std::ptr::addr_of_mut!(__slate_storage_334) as *mut *const i8;
    let mut __slate_storage_286: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_286: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_286) as *mut i32;
    let mut __slate_storage_333: std::mem::MaybeUninit<*const i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_333: *mut *const i8 =
        std::ptr::addr_of_mut!(__slate_storage_333) as *mut *const i8;
    let mut __slate_storage_332: std::mem::MaybeUninit<*const i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_332: *mut *const i8 =
        std::ptr::addr_of_mut!(__slate_storage_332) as *mut *const i8;
    let mut __slate_storage_331: std::mem::MaybeUninit<*const i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_331: *mut *const i8 =
        std::ptr::addr_of_mut!(__slate_storage_331) as *mut *const i8;
    let mut __slate_storage_330: std::mem::MaybeUninit<*const i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_330: *mut *const i8 =
        std::ptr::addr_of_mut!(__slate_storage_330) as *mut *const i8;
    let mut __slate_storage_329: std::mem::MaybeUninit<*const i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_329: *mut *const i8 =
        std::ptr::addr_of_mut!(__slate_storage_329) as *mut *const i8;
    let mut __slate_storage_328: std::mem::MaybeUninit<*const i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_328: *mut *const i8 =
        std::ptr::addr_of_mut!(__slate_storage_328) as *mut *const i8;
    let mut __slate_storage_327: std::mem::MaybeUninit<*const i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_327: *mut *const i8 =
        std::ptr::addr_of_mut!(__slate_storage_327) as *mut *const i8;
    let mut __slate_storage_326: std::mem::MaybeUninit<*const i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_326: *mut *const i8 =
        std::ptr::addr_of_mut!(__slate_storage_326) as *mut *const i8;
    let mut __slate_storage_325: std::mem::MaybeUninit<*const i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_325: *mut *const i8 =
        std::ptr::addr_of_mut!(__slate_storage_325) as *mut *const i8;
    let mut __slate_storage_324: std::mem::MaybeUninit<*const i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_324: *mut *const i8 =
        std::ptr::addr_of_mut!(__slate_storage_324) as *mut *const i8;
    let mut __slate_storage_323: std::mem::MaybeUninit<*const i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_323: *mut *const i8 =
        std::ptr::addr_of_mut!(__slate_storage_323) as *mut *const i8;
    let mut __slate_storage_322: std::mem::MaybeUninit<*const i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_322: *mut *const i8 =
        std::ptr::addr_of_mut!(__slate_storage_322) as *mut *const i8; // Nested parentheses
    let mut __slate_storage_283: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_283: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_283) as *mut i32; // unmatched structure character
    let mut __slate_storage_282: std::mem::MaybeUninit<u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_282: *mut u8 = std::ptr::addr_of_mut!(__slate_storage_282) as *mut u8; // Value of the next token
    let mut __slate_storage_281: std::mem::MaybeUninit<u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_281: *mut u8 = std::ptr::addr_of_mut!(__slate_storage_281) as *mut u8; // Current state, using numbers defined in header comment
    let mut __slate_storage_280: std::mem::MaybeUninit<u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_280: *mut u8 = std::ptr::addr_of_mut!(__slate_storage_280) as *mut u8;
    unsafe {
        std::ptr::write(__slate_slot_280, ((0 as i32) as i8) as u8);
        std::ptr::write(__slate_slot_282, ((0 as i32) as i8) as u8);
        std::ptr::write(__slate_slot_283, 0 as i32);
        // A complex statement machine used to detect the end of a CREATE TRIGGER
        // statement.  This is the normal case.
        // Token:
        //
        // State:       **  SEMI  WS  OTHER  EXPLAIN  CREATE  TEMP  TRIGGER  END
        //
        // 0 INVALID:
        // 1   START:
        // 2  NORMAL:
        // 3 EXPLAIN:
        // 4  CREATE:
        // 5 TRIGGER:
        // 6    SEMI:
        // 7     END:
        // Mapping state number to yy value for the return
        // 0 INVALID
        // 1 START
        // 2 NORMAL
        // 3 EXPLAIN
        // 4 CREATE
        // 5 TRIGGER
        // 6 SEMI
        // 7 END
        '__join_2: {
            '__join_69: {
                '__join_62: {
                    '__join_55: {
                        '__loop_3: loop {
                            if (unsafe { *zSql }) != (0 as i8) {
                                '__join_4: {
                                    '__join_76: {
                                        let __t1: i32 = (unsafe { *zSql }) as i32;
                                        if __t1 == (59 as i32) {
                                            // A semicolon
                                            *__slate_slot_281 = ((0 as i32) as i8) as u8;
                                            break '__join_4;
                                        } else {
                                            if __t1 == (32 as i32) {
                                                break '__join_76;
                                            } else {
                                                if __t1 == (13 as i32) {
                                                    break '__join_76;
                                                } else {
                                                    if __t1 == (9 as i32) {
                                                        break '__join_76;
                                                    } else {
                                                        if __t1 == (10 as i32) {
                                                            break '__join_76;
                                                        } else {
                                                            if __t1 == (12 as i32) {
                                                                break '__join_76;
                                                            } else {
                                                                if __t1 == (47 as i32) {
                                                                    // C-style comments
                                                                    if ((unsafe {
                                                                        *unsafe {
                                                                            zSql.offset(
                                                                                (1 as i32) as isize,
                                                                            )
                                                                        }
                                                                    })
                                                                        as i32)
                                                                        != (42 as i32)
                                                                    {
                                                                        *__slate_slot_281 =
                                                                            ((2 as i32) as i8)
                                                                                as u8;
                                                                        break '__join_4;
                                                                    } else {
                                                                        std::ptr::write(
                                                                            __slate_slot_322,
                                                                            zSql,
                                                                        );
                                                                        std::ptr::write(
                                                                            __slate_slot_323,
                                                                            unsafe {
                                                                                (*__slate_slot_322)
                                                                                    .offset(
                                                                                    (2 as i32)
                                                                                        as isize,
                                                                                )
                                                                            },
                                                                        );
                                                                        zSql = *__slate_slot_323;
                                                                        loop {
                                                                            if (unsafe {
                                                                                *unsafe {
                                                                                    zSql.offset((0 as i32) as isize)
                                                                                }
                                                                            }) != (0 as i8)
                                                                                && (((unsafe {
                                                                                    *unsafe {
                                                                                        zSql.offset((0 as i32) as isize)
                                                                                    }
                                                                                })
                                                                                    as i32)
                                                                                    != (42 as i32)
                                                                                    || ((unsafe {
                                                                                        *unsafe {
                                                                                            zSql.offset((1 as i32) as isize)
                                                                                        }
                                                                                    })
                                                                                        as i32)
                                                                                        != (47
                                                                                            as i32))
                                                                            {
                                                                                std::ptr::write(__slate_slot_324, zSql);
                                                                                std::ptr::write(__slate_slot_325, unsafe { (*__slate_slot_324).offset((1 as i32) as isize) });
                                                                                zSql = *__slate_slot_325;
                                                                            } else {
                                                                                break;
                                                                            }
                                                                        }
                                                                        if ((unsafe {
                                                                            *unsafe {
                                                                                zSql.offset(
                                                                                    (0 as i32)
                                                                                        as isize,
                                                                                )
                                                                            }
                                                                        })
                                                                            as i32)
                                                                            == (0 as i32)
                                                                        {
                                                                            break '__join_69;
                                                                        } else {
                                                                            std::ptr::write(
                                                                                __slate_slot_326,
                                                                                zSql,
                                                                            );
                                                                            std::ptr::write(
                                                                                __slate_slot_327,
                                                                                unsafe {
                                                                                    (*__slate_slot_326).offset((1 as i32) as isize)
                                                                                },
                                                                            );
                                                                            zSql =
                                                                                *__slate_slot_327;
                                                                            *__slate_slot_281 =
                                                                                ((1 as i32) as i8)
                                                                                    as u8;
                                                                            break '__join_4;
                                                                        }
                                                                    }
                                                                } else {
                                                                    if __t1 == (45 as i32) {
                                                                        // SQL-style comments from "--" to end of line
                                                                        if ((unsafe {
                                                                            *unsafe {
                                                                                zSql.offset(
                                                                                    (1 as i32)
                                                                                        as isize,
                                                                                )
                                                                            }
                                                                        })
                                                                            as i32)
                                                                            != (45 as i32)
                                                                        {
                                                                            *__slate_slot_281 =
                                                                                ((2 as i32) as i8)
                                                                                    as u8;
                                                                            break '__join_4;
                                                                        } else {
                                                                            loop {
                                                                                if (unsafe {
                                                                                    *zSql
                                                                                }) != (0 as i8)
                                                                                    && ((unsafe {
                                                                                        *zSql
                                                                                    })
                                                                                        as i32)
                                                                                        != (10
                                                                                            as i32)
                                                                                {
                                                                                    std::ptr::write(__slate_slot_328, zSql);
                                                                                    std::ptr::write(__slate_slot_329, unsafe { (*__slate_slot_328).offset((1 as i32) as isize) });
                                                                                    zSql = *__slate_slot_329;
                                                                                } else {
                                                                                    break;
                                                                                }
                                                                            }
                                                                            if ((unsafe { *zSql })
                                                                                as i32)
                                                                                == (0 as i32)
                                                                            {
                                                                                break '__join_62;
                                                                            } else {
                                                                                *__slate_slot_281 =
                                                                                    ((1 as i32)
                                                                                        as i8)
                                                                                        as u8;
                                                                                break '__join_4;
                                                                            }
                                                                        }
                                                                    } else {
                                                                        if __t1 == (91 as i32) {
                                                                            // Microsoft-style identifiers in [...]
                                                                            std::ptr::write(
                                                                                __slate_slot_330,
                                                                                zSql,
                                                                            );
                                                                            std::ptr::write(
                                                                                __slate_slot_331,
                                                                                unsafe {
                                                                                    (*__slate_slot_330).offset((1 as i32) as isize)
                                                                                },
                                                                            );
                                                                            zSql =
                                                                                *__slate_slot_331;
                                                                            loop {
                                                                                if (unsafe {
                                                                                    *zSql
                                                                                }) != (0 as i8)
                                                                                    && ((unsafe {
                                                                                        *zSql
                                                                                    })
                                                                                        as i32)
                                                                                        != (93
                                                                                            as i32)
                                                                                {
                                                                                    std::ptr::write(__slate_slot_332, zSql);
                                                                                    std::ptr::write(__slate_slot_333, unsafe { (*__slate_slot_332).offset((1 as i32) as isize) });
                                                                                    zSql = *__slate_slot_333;
                                                                                } else {
                                                                                    break;
                                                                                }
                                                                            }
                                                                            if ((unsafe { *zSql })
                                                                                as i32)
                                                                                == (0 as i32)
                                                                            {
                                                                                break '__join_55;
                                                                            } else {
                                                                                *__slate_slot_281 =
                                                                                    ((2 as i32)
                                                                                        as i8)
                                                                                        as u8;
                                                                                break '__join_4;
                                                                            }
                                                                        } else {
                                                                            if __t1 == (96 as i32) {
                                                                            } else {
                                                                                if __t1
                                                                                    == (34 as i32)
                                                                                {
                                                                                } else {
                                                                                    if __t1
                                                                                        == (39
                                                                                            as i32)
                                                                                    {
                                                                                    } else {
                                                                                        if __t1 == (40 as i32) {
std::ptr::write(__slate_slot_338, *__slate_slot_283);
std::ptr::write(__slate_slot_339, *__slate_slot_338 + (1 as i32));
*__slate_slot_283 = *__slate_slot_339;
*__slate_slot_281 = ((2 as i32) as i8) as u8;
break '__join_4;
} else {
if __t1 == (41 as i32) {
std::ptr::write(__slate_slot_340, *__slate_slot_283);
std::ptr::write(__slate_slot_341, *__slate_slot_340 - (1 as i32));
*__slate_slot_283 = *__slate_slot_341;
*__slate_slot_281 = ((2 as i32) as i8) as u8;
break '__join_4;
} else {
if (((unsafe { *unsafe { unsafe { std::ptr::addr_of!(sqlite3CtypeMap) as *const u8 }.offset(((((unsafe { *zSql }) as u8) as u32) as i32) as isize) } }) as u32) as i32) & (70 as i32) != (0 as i32) {
*__slate_slot_287 = 1 as i32;
loop {
if (((unsafe { *unsafe { unsafe { std::ptr::addr_of!(sqlite3CtypeMap) as *const u8 }.offset(((((unsafe { *unsafe { zSql.offset(*__slate_slot_287 as isize) } }) as u8) as u32) as i32) as isize) } }) as u32) as i32) & (70 as i32) != (0 as i32) {
std::ptr::write(__slate_slot_342, *__slate_slot_287);
std::ptr::write(__slate_slot_343, *__slate_slot_342 + (1 as i32));
*__slate_slot_287 = *__slate_slot_343;
} else {
break;
}
}
'__join_5: {
'__join_39: {
'__join_33: {
let __t0: i32 = (unsafe { *zSql }) as i32;
if __t0 == (99 as i32) {
break '__join_39;
} else {
if __t0 == (67 as i32) {
break '__join_39;
} else {
if __t0 == (116 as i32) {
break '__join_33;
} else {
if __t0 == (84 as i32) {
break '__join_33;
} else {
if __t0 == (101 as i32) {
} else {
if __t0 == (69 as i32) {
} else {
*__slate_slot_281 = ((2 as i32) as i8) as u8;
break '__join_5;
}
}
}
}
}
}
if *__slate_slot_287 == (3 as i32) {
*__slate_slot_348 = (unsafe { sqlite3_strnicmp(zSql, (b"end\0".as_ptr() as *mut i8) as *const i8, 3 as i32) }) == (0 as i32);
} else {
*__slate_slot_348 = false as bool;
}
if *__slate_slot_348 {
*__slate_slot_281 = ((7 as i32) as i8) as u8;
break '__join_5;
} else {
if *__slate_slot_287 == (7 as i32) {
*__slate_slot_349 = (unsafe { sqlite3_strnicmp(zSql, (b"explain\0".as_ptr() as *mut i8) as *const i8, 7 as i32) }) == (0 as i32);
} else {
*__slate_slot_349 = false as bool;
}
if *__slate_slot_349 {
*__slate_slot_281 = ((3 as i32) as i8) as u8;
break '__join_5;
} else {
*__slate_slot_281 = ((2 as i32) as i8) as u8;
break '__join_5;
}
}
}
if *__slate_slot_287 == (7 as i32) {
*__slate_slot_345 = (unsafe { sqlite3_strnicmp(zSql, (b"trigger\0".as_ptr() as *mut i8) as *const i8, 7 as i32) }) == (0 as i32);
} else {
*__slate_slot_345 = false as bool;
}
if *__slate_slot_345 {
*__slate_slot_281 = ((6 as i32) as i8) as u8;
break '__join_5;
} else {
if *__slate_slot_287 == (4 as i32) {
*__slate_slot_346 = (unsafe { sqlite3_strnicmp(zSql, (b"temp\0".as_ptr() as *mut i8) as *const i8, 4 as i32) }) == (0 as i32);
} else {
*__slate_slot_346 = false as bool;
}
if *__slate_slot_346 {
*__slate_slot_281 = ((5 as i32) as i8) as u8;
break '__join_5;
} else {
if *__slate_slot_287 == (9 as i32) {
*__slate_slot_347 = (unsafe { sqlite3_strnicmp(zSql, (b"temporary\0".as_ptr() as *mut i8) as *const i8, 9 as i32) }) == (0 as i32);
} else {
*__slate_slot_347 = false as bool;
}
if *__slate_slot_347 {
*__slate_slot_281 = ((5 as i32) as i8) as u8;
break '__join_5;
} else {
*__slate_slot_281 = ((2 as i32) as i8) as u8;
break '__join_5;
}
}
}
}
if *__slate_slot_287 == (6 as i32) {
*__slate_slot_344 = (unsafe { sqlite3_strnicmp(zSql, (b"create\0".as_ptr() as *mut i8) as *const i8, 6 as i32) }) == (0 as i32);
} else {
*__slate_slot_344 = false as bool;
}
if *__slate_slot_344 {
*__slate_slot_281 = ((4 as i32) as i8) as u8;
} else {
*__slate_slot_281 = ((2 as i32) as i8) as u8;
}
}
std::ptr::write(__slate_slot_350, zSql);
std::ptr::write(__slate_slot_351, unsafe { (*__slate_slot_350).offset((*__slate_slot_287 - (1 as i32)) as isize) });
zSql = *__slate_slot_351;
break '__join_4;
} else {
// Operators and special symbols
*__slate_slot_281 = ((2 as i32) as i8) as u8;
break '__join_4;
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
                                                }
                                            }
                                        }
                                        std::ptr::write(
                                            __slate_slot_286,
                                            (unsafe { *zSql }) as i32,
                                        );
                                        std::ptr::write(__slate_slot_334, zSql);
                                        std::ptr::write(__slate_slot_335, unsafe {
                                            (*__slate_slot_334).offset((1 as i32) as isize)
                                        });
                                        zSql = *__slate_slot_335;
                                        loop {
                                            if (unsafe { *zSql }) != (0 as i8)
                                                && ((unsafe { *zSql }) as i32) != *__slate_slot_286
                                            {
                                                std::ptr::write(__slate_slot_336, zSql);
                                                std::ptr::write(__slate_slot_337, unsafe {
                                                    (*__slate_slot_336).offset((1 as i32) as isize)
                                                });
                                                zSql = *__slate_slot_337;
                                            } else {
                                                break;
                                            }
                                        }
                                        if ((unsafe { *zSql }) as i32) == (0 as i32) {
                                            break '__loop_3;
                                        } else {
                                            *__slate_slot_281 = ((2 as i32) as i8) as u8;
                                            break '__join_4;
                                        }
                                    }
                                    // White space is ignored
                                    *__slate_slot_281 = ((1 as i32) as i8) as u8;
                                }
                                *__slate_slot_280 = unsafe {
                                    *unsafe {
                                        unsafe {
                                            (*unsafe {
                                                unsafe {
                                                    std::ptr::addr_of!(trans.0) as *const [u8; 8]
                                                }
                                                .offset(
                                                    ((*__slate_slot_280 as u32) as i32) as isize,
                                                )
                                            })
                                            .as_ptr()
                                                as *const u8
                                        }
                                        .offset(((*__slate_slot_281 as u32) as i32) as isize)
                                    }
                                };
                                std::ptr::write(__slate_slot_352, zSql);
                                std::ptr::write(__slate_slot_353, unsafe {
                                    (*__slate_slot_352).offset((1 as i32) as isize)
                                });
                                zSql = *__slate_slot_353;
                            } else {
                                break '__join_2;
                            }
                        }
                        *__slate_slot_282 = (*__slate_slot_286 as i8) as u8;
                        break '__join_2;
                    }
                    *__slate_slot_282 = ((93 as i32) as i8) as u8;
                    break '__join_2;
                }
                if ((*__slate_slot_280 as u32) as i32) != (1 as i32) {
                    *__slate_slot_282 = ((45 as i32) as i8) as u8;
                    break '__join_2;
                } else {
                    break '__join_2;
                }
            }
            *__slate_slot_282 = ((47 as i32) as i8) as u8;
        }
        '__join_0: {
            if ((*__slate_slot_280 as u32) as i32) == (1 as i32) {
                *__slate_slot_283 = 0 as i32;
            }
        }
        return (((*__slate_slot_283 as i64) as u64) << (32 as i32)
            | (*__slate_slot_282 as u64) << (16 as i32)
            | ((unsafe {
                *unsafe {
                    unsafe { std::ptr::addr_of!(statemap) as *const u8 }
                        .offset(((*__slate_slot_280 as u32) as i32) as isize)
                }
            }) as u64)
                << (8 as i32)
            | ((((((*__slate_slot_280 as u32) as i32) != (1 as i32)) as i32) as i64) as u64))
            as i64;
    }
    // Grave-accent quoted symbols used by MySQL
    // single- and double-quoted strings
    return unsafe { std::mem::zeroed() };
}

static mut trans: __SlateAlign16<[[u8; 8]; 8]> = __SlateAlign16([
    [
        ((1 as i32) as i8) as u8,
        ((0 as i32) as i8) as u8,
        ((2 as i32) as i8) as u8,
        ((3 as i32) as i8) as u8,
        ((4 as i32) as i8) as u8,
        ((2 as i32) as i8) as u8,
        ((2 as i32) as i8) as u8,
        ((2 as i32) as i8) as u8,
    ],
    [
        ((1 as i32) as i8) as u8,
        ((1 as i32) as i8) as u8,
        ((2 as i32) as i8) as u8,
        ((3 as i32) as i8) as u8,
        ((4 as i32) as i8) as u8,
        ((2 as i32) as i8) as u8,
        ((2 as i32) as i8) as u8,
        ((2 as i32) as i8) as u8,
    ],
    [
        ((1 as i32) as i8) as u8,
        ((2 as i32) as i8) as u8,
        ((2 as i32) as i8) as u8,
        ((2 as i32) as i8) as u8,
        ((2 as i32) as i8) as u8,
        ((2 as i32) as i8) as u8,
        ((2 as i32) as i8) as u8,
        ((2 as i32) as i8) as u8,
    ],
    [
        ((1 as i32) as i8) as u8,
        ((3 as i32) as i8) as u8,
        ((3 as i32) as i8) as u8,
        ((2 as i32) as i8) as u8,
        ((4 as i32) as i8) as u8,
        ((2 as i32) as i8) as u8,
        ((2 as i32) as i8) as u8,
        ((2 as i32) as i8) as u8,
    ],
    [
        ((1 as i32) as i8) as u8,
        ((4 as i32) as i8) as u8,
        ((2 as i32) as i8) as u8,
        ((2 as i32) as i8) as u8,
        ((2 as i32) as i8) as u8,
        ((4 as i32) as i8) as u8,
        ((5 as i32) as i8) as u8,
        ((2 as i32) as i8) as u8,
    ],
    [
        ((6 as i32) as i8) as u8,
        ((5 as i32) as i8) as u8,
        ((5 as i32) as i8) as u8,
        ((5 as i32) as i8) as u8,
        ((5 as i32) as i8) as u8,
        ((5 as i32) as i8) as u8,
        ((5 as i32) as i8) as u8,
        ((5 as i32) as i8) as u8,
    ],
    [
        ((6 as i32) as i8) as u8,
        ((6 as i32) as i8) as u8,
        ((5 as i32) as i8) as u8,
        ((5 as i32) as i8) as u8,
        ((5 as i32) as i8) as u8,
        ((5 as i32) as i8) as u8,
        ((5 as i32) as i8) as u8,
        ((7 as i32) as i8) as u8,
    ],
    [
        ((1 as i32) as i8) as u8,
        ((7 as i32) as i8) as u8,
        ((5 as i32) as i8) as u8,
        ((5 as i32) as i8) as u8,
        ((5 as i32) as i8) as u8,
        ((5 as i32) as i8) as u8,
        ((5 as i32) as i8) as u8,
        ((5 as i32) as i8) as u8,
    ],
]);

static mut statemap: [u8; 8] = [
    ((1 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((1 as i32) as i8) as u8,
    ((1 as i32) as i8) as u8,
    ((1 as i32) as i8) as u8,
    ((3 as i32) as i8) as u8,
    ((2 as i32) as i8) as u8,
    ((1 as i32) as i8) as u8,
];

#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.complete.sqlite3_complete")]
extern "C-unwind" fn sqlite3_complete(mut zSql: *const i8) -> i32 {
    return (sqlite3_incomplete(zSql) == ((0 as i32) as i64)) as i32;
}

/// This routine is the same as the sqlite3_complete() routine described
/// above, except that the parameter is required to be UTF-16 encoded, not
/// UTF-8.
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.complete.sqlite3_complete16")]
extern "C-unwind" fn sqlite3_complete16(mut zSql: *const ()) -> i32 {
    let mut pVal: *mut sqlite3_value = unsafe { std::mem::zeroed() };
    let mut zSql8: *const i8 = unsafe { std::mem::zeroed() };
    let mut rc: i32 = 0 as i32;
    rc = unsafe { sqlite3_initialize() };
    if rc != (0 as i32) {
        return rc;
    }
    pVal = unsafe { sqlite3ValueNew(std::ptr::null_mut::<sqlite3>()) };
    unsafe { sqlite3ValueSetStr(pVal, -(1 as i32), zSql, ((2 as i32) as i8) as u8, None) };
    zSql8 = (unsafe { sqlite3ValueText(pVal, ((1 as i32) as i8) as u8) }) as *const i8;
    if zSql8 != std::ptr::null::<i8>() {
        rc = (sqlite3_incomplete(zSql8) == ((0 as i32) as i64)) as i32;
    } else {
        rc = 7 as i32;
    }
    unsafe { sqlite3ValueFree(pVal) };
    return rc & (255 as i32);
}
