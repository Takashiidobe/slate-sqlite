//! 2020-03-23
//!
//! The author disclaims copyright to this source code.  In place of
//! a legal notice, here is a blessing:
//!
//!    May you do good and not evil.
//!    May you find forgiveness for yourself and forgive others.
//!    May you share freely, never taking more than you give.
//!
//!
//!
//! This file implements virtual-tables for examining the bytecode content
//! of a prepared statement.
unsafe extern "C" {
    fn sqlite3_mprintf(__v401: *const i8, ...) -> *mut i8;
    fn sqlite3_malloc(__v402: i32) -> *mut ();
    fn sqlite3_free(__v403: *mut ());
    fn sqlite3_prepare_v2(
        db: *mut sqlite3,
        zSql: *const i8,
        nByte: i32,
        ppStmt: *mut *mut sqlite3_stmt,
        pzTail: *mut *const i8,
    ) -> i32;
    fn sqlite3_finalize(pStmt: *mut sqlite3_stmt) -> i32;
    fn sqlite3_value_pointer(__v410: *mut sqlite3_value, __v411: *const i8) -> *mut ();
    fn sqlite3_value_text(__v412: *mut sqlite3_value) -> *const u8;
    fn sqlite3_value_type(__v413: *mut sqlite3_value) -> i32;
    fn sqlite3_result_int(__v414: *mut sqlite3_context, __v415: i32);
    fn sqlite3_result_text(
        __v416: *mut sqlite3_context,
        __v417: *const i8,
        __v418: i32,
        __v419: Option<unsafe extern "C-unwind" fn(*mut ())>,
    );
    fn sqlite3_create_module(
        db: *mut sqlite3,
        zName: *const i8,
        p: *const sqlite3_module,
        pClientData: *mut (),
    ) -> i32;
    fn sqlite3_declare_vtab(__v424: *mut sqlite3, zSQL: *const i8) -> i32;
    fn memset(__s: *mut (), __c: i32, __n: u64) -> *mut ();
    fn sqlite3VdbeNextOpcode(
        __v430: *mut Vdbe,
        __v431: *mut sqlite3_value,
        __v432: i32,
        __v433: *mut i32,
        __v434: *mut i32,
        __v435: *mut *mut VdbeOp,
    ) -> i32;
    fn sqlite3VdbeDisplayP4(__v436: *mut sqlite3, __v437: *mut VdbeOp) -> *mut i8;
    fn sqlite3VdbeDisplayComment(
        __v438: *mut sqlite3,
        __v439: *const VdbeOp,
        __v440: *const i8,
    ) -> *mut i8;
    fn sqlite3VdbeMemInit(__v441: *mut sqlite3_value, __v442: *mut sqlite3, __v443: u16);
    fn sqlite3VdbeMemSetNull(__v444: *mut sqlite3_value);
    fn sqlite3VdbeMemRelease(p: *mut sqlite3_value);
    fn sqlite3OpcodeName(__v446: i32) -> *const i8;
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
struct sqlite3_stmt {}

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
struct _ht {
    count: u32,
    chain: *mut HashElem,
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
    __slate_bits_0: __slate_bits::__SlateBits67U0,
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
    u: __SlateRecord165,
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
    __slate_bits_0: __slate_bits::__SlateBits91U0,
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
struct ParseCleanup {
    pNext: *mut ParseCleanup,
    pPtr: *mut (),
    xCleanup: Option<unsafe extern "C-unwind" fn(*mut sqlite3, *mut ())>,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct TableLock {}

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
    u1: __SlateRecord189,
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
struct VtabCtx {}

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
struct Btree {}

#[repr(C)]
#[derive(Clone, Copy)]
struct BtCursor {}

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
struct VdbeCursor {
    eCurType: u8,
    iDb: i8,
    nullRow: u8,
    deferredMoveto: u8,
    isTable: u8,
    __slate_bits_0: __slate_bits::__SlateBits200U0,
    seekHit: u16,
    ub: __SlateRecord202,
    seqCount: i64,
    cacheStatus: u32,
    seekResult: i32,
    pAltCursor: *mut VdbeCursor,
    uc: __SlateRecord203,
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
struct VdbeTxtBlbCache {
    pCValue: *mut i8,
    iOffset: i64,
    iCol: i32,
    cacheStatus: u32,
    colCacheCtr: u32,
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
union __SlateRecord189 {
    cr: __SlateRecord190,
    d: __SlateRecord191,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord190 {
    addrCrTab: i32,
    regRowid: i32,
    regRoot: i32,
    constraintName: Token,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord191 {
    pReturning: *mut Returning,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct VdbeSorter {}

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
struct AuxData {
    iAuxOp: i32,
    iAuxArg: i32,
    pAux: *mut (),
    xDeleteAux: Option<unsafe extern "C-unwind" fn(*mut ())>,
    pNextAux: *mut AuxData,
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
union __SlateRecord202 {
    pBtx: *mut Btree,
    aAltMap: *mut u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord203 {
    pCursor: *mut BtCursor,
    pVCur: *mut sqlite3_vtab_cursor,
    pSorter: *mut VdbeSorter,
}

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
union MemValue {
    r: f64,
    i: i64,
    nZero: i32,
    zPType: *const i8,
    pDef: *mut FuncDef,
}

/// An instance of the bytecode() table-valued function.
#[repr(C)]
#[derive(Clone, Copy)]
struct bytecodevtab {
    /// Base class - must be first
    base: sqlite3_vtab,
    /// Database connection
    db: *mut sqlite3,
    /// 2 for tables_used().  0 for bytecode().
    bTablesUsed: i32,
}

/// A cursor for scanning through the bytecode
#[repr(C)]
#[derive(Clone, Copy)]
struct bytecodevtab_cursor {
    /// Base class - must be first
    base: sqlite3_vtab_cursor,
    /// The statement whose bytecode is displayed
    pStmt: *mut sqlite3_stmt,
    /// The rowid of the output table
    iRowid: i32,
    /// Address
    iAddr: i32,
    /// Cursors owns pStmt and must finalize it
    needFinalize: i32,
    /// Provide a listing of subprograms
    showSubprograms: i32,
    /// Operand array
    aOp: *mut VdbeOp,
    /// Rendered P4 value
    zP4: *mut i8,
    /// tables_used.type
    zType: *const i8,
    /// tables_used.schema
    zSchema: *const i8,
    /// tables_used.name
    zName: *const i8,
    /// Subprograms
    sub: sqlite3_value,
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
    pub struct __SlateBits200U0 {
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
}

/// Create a new bytecode() table-valued function.
#[unsafe(link_section = ".text.slate_distinct.vdbevtab.bytecodevtabConnect")]
extern "C-unwind" fn bytecodevtabConnect(
    mut db: *mut sqlite3,
    mut pAux: *mut (),
    mut argc: i32,
    mut argv: *const *const i8,
    mut ppVtab: *mut *mut sqlite3_vtab,
    mut pzErr: *mut *mut i8,
) -> i32 {
    let mut pNew: *mut bytecodevtab = unsafe { std::mem::zeroed() };
    let mut rc: i32 = 0 as i32;
    let mut isTabUsed: i32 = (pAux != std::ptr::null_mut::<()>()) as i32;
    let mut azSchema: __SlateAlign16<[*const i8; 2]> = __SlateAlign16([(b"CREATE TABLE x(addr INT,opcode TEXT,p1 INT,p2 INT,p3 INT,p4 TEXT,p5 INT,comment TEXT,subprog TEXT,nexec INT,ncycle INT,stmt HIDDEN);\0".as_ptr() as *mut i8) as *const i8, (b"CREATE TABLE x(type TEXT,schema TEXT,name TEXT,wr INT,subprog TEXT,stmt HIDDEN);\0".as_ptr() as *mut i8) as *const i8]); // bytecode() schema
    // Tables_used() schema
    argc;
    argv;
    pzErr;
    rc = unsafe {
        sqlite3_declare_vtab(db, unsafe {
            *unsafe { (azSchema.0.as_mut_ptr() as *mut *const i8).offset(isTabUsed as isize) }
        })
    };
    if rc == (0 as i32) {
        pNew = (unsafe { sqlite3_malloc(((40 as u64) as u32) as i32) }) as *mut bytecodevtab;
        unsafe {
            *ppVtab = pNew as *mut sqlite3_vtab;
        }
        if pNew == std::ptr::null_mut::<bytecodevtab>() {
            return 7 as i32;
        }
        unsafe { memset(pNew as *mut (), 0 as i32, 40 as u64) };
        unsafe {
            (*pNew).db = db;
        }
        unsafe {
            (*pNew).bTablesUsed = isTabUsed * (2 as i32);
        }
    }
    return rc;
}

/// This method is the destructor for bytecodevtab objects.
#[unsafe(link_section = ".text.slate_distinct.vdbevtab.bytecodevtabDisconnect")]
extern "C-unwind" fn bytecodevtabDisconnect(mut pVtab: *mut sqlite3_vtab) -> i32 {
    let mut p: *mut bytecodevtab = pVtab as *mut bytecodevtab;
    unsafe { sqlite3_free(p as *mut ()) };
    return 0 as i32;
}

/// Constructor for a new bytecodevtab_cursor object.
#[unsafe(link_section = ".text.slate_distinct.vdbevtab.bytecodevtabOpen")]
extern "C-unwind" fn bytecodevtabOpen(
    mut p: *mut sqlite3_vtab,
    mut ppCursor: *mut *mut sqlite3_vtab_cursor,
) -> i32 {
    let mut pVTab: *mut bytecodevtab = p as *mut bytecodevtab;
    let mut pCur: *mut bytecodevtab_cursor = unsafe { std::mem::zeroed() };
    pCur = (unsafe { sqlite3_malloc(((128 as u64) as u32) as i32) }) as *mut bytecodevtab_cursor;
    if pCur == std::ptr::null_mut::<bytecodevtab_cursor>() {
        return 7 as i32;
    }
    unsafe { memset(pCur as *mut (), 0 as i32, 128 as u64) };
    unsafe {
        sqlite3VdbeMemInit(
            unsafe { std::ptr::addr_of_mut!((*pCur).sub) },
            unsafe { (*pVTab).db },
            ((1 as i32) as i16) as u16,
        )
    };
    unsafe {
        *ppCursor = unsafe { std::ptr::addr_of_mut!((*pCur).base) };
    }
    return 0 as i32;
}

/// Clear all internal content from a bytecodevtab cursor.
fn bytecodevtabCursorClear(mut pCur: *mut bytecodevtab_cursor) {
    unsafe { sqlite3_free((unsafe { (*pCur).zP4 }) as *mut ()) };
    unsafe {
        (*pCur).zP4 = std::ptr::null_mut::<i8>();
    }
    unsafe { sqlite3VdbeMemRelease(unsafe { std::ptr::addr_of_mut!((*pCur).sub) }) };
    unsafe { sqlite3VdbeMemSetNull(unsafe { std::ptr::addr_of_mut!((*pCur).sub) }) };
    if (unsafe { (*pCur).needFinalize }) != (0 as i32) {
        unsafe { sqlite3_finalize(unsafe { (*pCur).pStmt }) };
    }
    unsafe {
        (*pCur).pStmt = std::ptr::null_mut::<sqlite3_stmt>();
    }
    unsafe {
        (*pCur).needFinalize = 0 as i32;
    }
    unsafe {
        (*pCur).zType = std::ptr::null::<i8>();
    }
    unsafe {
        (*pCur).zSchema = std::ptr::null::<i8>();
    }
    unsafe {
        (*pCur).zName = std::ptr::null::<i8>();
    }
}

/// Destructor for a bytecodevtab_cursor.
#[unsafe(link_section = ".text.slate_distinct.vdbevtab.bytecodevtabClose")]
extern "C-unwind" fn bytecodevtabClose(mut cur: *mut sqlite3_vtab_cursor) -> i32 {
    let mut pCur: *mut bytecodevtab_cursor = cur as *mut bytecodevtab_cursor;
    bytecodevtabCursorClear(pCur);
    unsafe { sqlite3_free(pCur as *mut ()) };
    return 0 as i32;
}

/// Advance a bytecodevtab_cursor to its next row of output.
#[unsafe(link_section = ".text.slate_distinct.vdbevtab.bytecodevtabNext")]
extern "C-unwind" fn bytecodevtabNext(mut cur: *mut sqlite3_vtab_cursor) -> i32 {
    let mut pCur: *mut bytecodevtab_cursor = cur as *mut bytecodevtab_cursor;
    let mut pTab: *mut bytecodevtab = (unsafe { (*cur).pVtab }) as *mut bytecodevtab;
    let mut rc: i32 = 0 as i32;
    if (unsafe { (*pCur).zP4 }) != std::ptr::null_mut::<i8>() {
        unsafe { sqlite3_free((unsafe { (*pCur).zP4 }) as *mut ()) };
        unsafe {
            (*pCur).zP4 = std::ptr::null_mut::<i8>();
        }
    }
    if (unsafe { (*pCur).zName }) != std::ptr::null::<i8>() {
        unsafe {
            (*pCur).zName = std::ptr::null::<i8>();
        }
        unsafe {
            (*pCur).zType = std::ptr::null::<i8>();
        }
        unsafe {
            (*pCur).zSchema = std::ptr::null::<i8>();
        }
    }
    rc = unsafe {
        sqlite3VdbeNextOpcode(
            (unsafe { (*pCur).pStmt }) as *mut Vdbe,
            if (unsafe { (*pCur).showSubprograms }) != (0 as i32) {
                unsafe { std::ptr::addr_of_mut!((*pCur).sub) }
            } else {
                std::ptr::null_mut::<sqlite3_value>()
            },
            unsafe { (*pTab).bTablesUsed },
            unsafe { std::ptr::addr_of_mut!((*pCur).iRowid) },
            unsafe { std::ptr::addr_of_mut!((*pCur).iAddr) },
            unsafe { std::ptr::addr_of_mut!((*pCur).aOp) },
        )
    };
    if rc != (0 as i32) {
        unsafe { sqlite3VdbeMemSetNull(unsafe { std::ptr::addr_of_mut!((*pCur).sub) }) };
        unsafe {
            (*pCur).aOp = std::ptr::null_mut::<VdbeOp>();
        }
    }
    return 0 as i32;
}

/// Return TRUE if the cursor has been moved off of the last
/// row of output.
#[unsafe(link_section = ".text.slate_distinct.vdbevtab.bytecodevtabEof")]
extern "C-unwind" fn bytecodevtabEof(mut cur: *mut sqlite3_vtab_cursor) -> i32 {
    let mut pCur: *mut bytecodevtab_cursor = cur as *mut bytecodevtab_cursor;
    return ((unsafe { (*pCur).aOp }) == std::ptr::null_mut::<VdbeOp>()) as i32;
}

/// Return values of columns for the row at which the bytecodevtab_cursor
/// is currently pointing.
///
/// # Arguments
///
/// * `cur` - The cursor
/// * `ctx` - First argument to sqlite3_result_...()
/// * `i` - Which column to return
#[unsafe(link_section = ".text.slate_distinct.vdbevtab.bytecodevtabColumn")]
extern "C-unwind" fn bytecodevtabColumn(
    mut cur: *mut sqlite3_vtab_cursor,
    mut ctx: *mut sqlite3_context,
    mut i: i32,
) -> i32 {
    let mut pCur: *mut bytecodevtab_cursor = cur as *mut bytecodevtab_cursor;
    let mut pVTab: *mut bytecodevtab = (unsafe { (*cur).pVtab }) as *mut bytecodevtab;
    let mut pOp: *mut VdbeOp =
        unsafe { unsafe { (*pCur).aOp }.offset((unsafe { (*pCur).iAddr }) as isize) };
    if (unsafe { (*pVTab).bTablesUsed }) != (0 as i32) {
        if i == (4 as i32) {
            i = 8 as i32;
        } else {
            if i <= (2 as i32) && (unsafe { (*pCur).zType }) == std::ptr::null::<i8>() {
                let mut pSchema: *mut Schema = unsafe { std::mem::zeroed() };
                let mut k: *mut HashElem = unsafe { std::mem::zeroed() };
                let mut iDb: i32 = unsafe { (*pOp).p3 };
                let mut iRoot: u32 = (unsafe { (*pOp).p2 }) as u32;
                let mut db: *mut sqlite3 = unsafe { (*pVTab).db };
                pSchema =
                    unsafe { (*unsafe { unsafe { (*db).aDb }.offset(iDb as isize) }).pSchema };
                unsafe {
                    (*pCur).zSchema = (unsafe {
                        (*unsafe { unsafe { (*db).aDb }.offset(iDb as isize) }).zDbSName
                    }) as *const i8;
                }
                k = unsafe { (*unsafe { std::ptr::addr_of_mut!((*pSchema).tblHash) }).first };
                '__slate_break_449: while k != std::ptr::null_mut::<HashElem>() {
                    let mut pTab: *mut Table = (unsafe { (*k).data }) as *mut Table;
                    if !((((unsafe { (*pTab).eTabType }) as u32) as i32) == (1 as i32))
                        && (unsafe { (*pTab).tnum }) == iRoot
                    {
                        unsafe {
                            (*pCur).zName = (unsafe { (*pTab).zName }) as *const i8;
                        }
                        unsafe {
                            (*pCur).zType = (b"table\0".as_ptr() as *mut i8) as *const i8;
                        }
                        break '__slate_break_449;
                    }
                    k = unsafe { (*k).next };
                }
                if (unsafe { (*pCur).zName }) == std::ptr::null::<i8>() {
                    k = unsafe { (*unsafe { std::ptr::addr_of_mut!((*pSchema).idxHash) }).first };
                    '__slate_break_451: while k != std::ptr::null_mut::<HashElem>() {
                        let mut pIdx: *mut Index = (unsafe { (*k).data }) as *mut Index;
                        if (unsafe { (*pIdx).tnum }) == iRoot {
                            unsafe {
                                (*pCur).zName = (unsafe { (*pIdx).zName }) as *const i8;
                            }
                            unsafe {
                                (*pCur).zType = (b"index\0".as_ptr() as *mut i8) as *const i8;
                            }
                        }
                        k = unsafe { (*k).next };
                    }
                }
            }
            let __v462: i32 = i;
            let __v463: i32 = __v462 + (20 as i32);
            i = __v463;
        }
    }
    '__slate_break_453: {
        match i {
            0 => {
                unsafe { sqlite3_result_int(ctx, unsafe { (*pCur).iAddr }) }; // addr
            }
            1 => {
                unsafe {
                    sqlite3_result_text(
                        ctx,
                        ((unsafe { sqlite3OpcodeName(((unsafe { (*pOp).opcode }) as u32) as i32) })
                            as *mut i8) as *const i8,
                        -(1 as i32),
                        None,
                    )
                }; // opcode
            }
            2 => {
                unsafe { sqlite3_result_int(ctx, unsafe { (*pOp).p1 }) }; // p1
            }
            3 => {
                unsafe { sqlite3_result_int(ctx, unsafe { (*pOp).p2 }) }; // p2
            }
            4 => {
                unsafe { sqlite3_result_int(ctx, unsafe { (*pOp).p3 }) }; // p3
            }
            5 | 7 => {
                if (unsafe { (*pCur).zP4 }) == std::ptr::null_mut::<i8>() {
                    unsafe {
                        (*pCur).zP4 = unsafe { sqlite3VdbeDisplayP4(unsafe { (*pVTab).db }, pOp) };
                    }
                }
                // p4
                // comment
                if i == (5 as i32) {
                    unsafe {
                        sqlite3_result_text(
                            ctx,
                            (unsafe { (*pCur).zP4 }) as *const i8,
                            -(1 as i32),
                            None,
                        )
                    };
                } else {
                    let mut zCom: *mut i8 = unsafe {
                        sqlite3VdbeDisplayComment(
                            unsafe { (*pVTab).db },
                            pOp as *const VdbeOp,
                            (unsafe { (*pCur).zP4 }) as *const i8,
                        )
                    };
                    unsafe {
                        sqlite3_result_text(ctx, zCom as *const i8, -(1 as i32), unsafe {
                            std::mem::transmute::<
                                *const (),
                                Option<unsafe extern "C-unwind" fn(*mut ())>,
                            >(sqlite3_free as *const ())
                        })
                    };
                }
            }
            6 => {
                unsafe { sqlite3_result_int(ctx, ((unsafe { (*pOp).p5 }) as u32) as i32) }; // p5
            }
            8 => {
                // subprog
                let mut aOp: *mut VdbeOp = unsafe { (*pCur).aOp };
                0 as i32;
                0 as i32;
                if (unsafe { (*pCur).iRowid }) == (unsafe { (*pCur).iAddr }) + (1 as i32) {
                    break '__slate_break_453; // Result is NULL for the main program
                } else {
                    if (unsafe { (*unsafe { aOp.offset((0 as i32) as isize) }).p4.z })
                        != std::ptr::null_mut::<i8>()
                    {
                        unsafe {
                            sqlite3_result_text(
                                ctx,
                                (unsafe {
                                    unsafe { (*unsafe { aOp.offset((0 as i32) as isize) }).p4.z }
                                        .offset((3 as i32) as isize)
                                }) as *const i8,
                                -(1 as i32),
                                None,
                            )
                        };
                    } else {
                        unsafe {
                            sqlite3_result_text(
                                ctx,
                                (b"(FK)\0".as_ptr() as *mut i8) as *const i8,
                                4 as i32,
                                None,
                            )
                        };
                    }
                }
            }
            9 | 10 => {
                unsafe { sqlite3_result_int(ctx, 0 as i32) }; // nexec
                // ncycle
            }
            20 => {
                unsafe { sqlite3_result_text(ctx, unsafe { (*pCur).zType }, -(1 as i32), None) }; // tables_used.type
            }
            21 => {
                unsafe { sqlite3_result_text(ctx, unsafe { (*pCur).zSchema }, -(1 as i32), None) }; // tables_used.schema
            }
            22 => {
                unsafe { sqlite3_result_text(ctx, unsafe { (*pCur).zName }, -(1 as i32), None) }; // tables_used.name
            }
            23 => {
                unsafe {
                    sqlite3_result_int(
                        ctx,
                        ((((unsafe { (*pOp).opcode }) as u32) as i32) == (116 as i32)) as i32,
                    )
                }; // tables_used.wr
            }
            _ => {}
        }
    }
    return 0 as i32;
}

/// Return the rowid for the current row.  In this implementation, the
/// rowid is the same as the output value.
#[unsafe(link_section = ".text.slate_distinct.vdbevtab.bytecodevtabRowid")]
extern "C-unwind" fn bytecodevtabRowid(
    mut cur: *mut sqlite3_vtab_cursor,
    mut pRowid: *mut i64,
) -> i32 {
    let mut pCur: *mut bytecodevtab_cursor = cur as *mut bytecodevtab_cursor;
    unsafe {
        *pRowid = (unsafe { (*pCur).iRowid }) as i64;
    }
    return 0 as i32;
}

/// Initialize a cursor.
///
///    idxNum==0     means show all subprograms
///    idxNum==1     means show only the main bytecode and omit subprograms.
#[unsafe(link_section = ".text.slate_distinct.vdbevtab.bytecodevtabFilter")]
extern "C-unwind" fn bytecodevtabFilter(
    mut pVtabCursor: *mut sqlite3_vtab_cursor,
    mut idxNum: i32,
    mut idxStr: *const i8,
    mut argc: i32,
    mut argv: *mut *mut sqlite3_value,
) -> i32 {
    let mut pCur: *mut bytecodevtab_cursor = pVtabCursor as *mut bytecodevtab_cursor;
    let mut pVTab: *mut bytecodevtab = (unsafe { (*pVtabCursor).pVtab }) as *mut bytecodevtab;
    let mut rc: i32 = 0 as i32;
    idxStr;
    bytecodevtabCursorClear(pCur);
    unsafe {
        (*pCur).iRowid = 0 as i32;
    }
    unsafe {
        (*pCur).iAddr = 0 as i32;
    }
    unsafe {
        (*pCur).showSubprograms = (idxNum == (0 as i32)) as i32;
    }
    0 as i32;
    if (unsafe { sqlite3_value_type(unsafe { *unsafe { argv.offset((0 as i32) as isize) } }) })
        == (3 as i32)
    {
        let mut zSql: *const i8 = (unsafe {
            sqlite3_value_text(unsafe { *unsafe { argv.offset((0 as i32) as isize) } })
        }) as *const i8;
        if zSql == std::ptr::null::<i8>() {
            rc = 7 as i32;
        } else {
            rc = unsafe {
                sqlite3_prepare_v2(
                    unsafe { (*pVTab).db },
                    zSql,
                    -(1 as i32),
                    unsafe { std::ptr::addr_of_mut!((*pCur).pStmt) },
                    std::ptr::null_mut::<*const i8>(),
                )
            };
            unsafe {
                (*pCur).needFinalize = 1 as i32;
            }
        }
    } else {
        unsafe {
            (*pCur).pStmt = (unsafe {
                sqlite3_value_pointer(
                    unsafe { *unsafe { argv.offset((0 as i32) as isize) } },
                    (b"stmt-pointer\0".as_ptr() as *mut i8) as *const i8,
                )
            }) as *mut sqlite3_stmt;
        }
    }
    if (unsafe { (*pCur).pStmt }) == std::ptr::null_mut::<sqlite3_stmt>() {
        unsafe {
            (*pVTab).base.zErrMsg = unsafe {
                sqlite3_mprintf(
                    (b"argument to %s() is not a valid SQL statement\0".as_ptr() as *mut i8)
                        as *const i8,
                    if (unsafe { (*pVTab).bTablesUsed }) != (0 as i32) {
                        b"tables_used\0".as_ptr() as *mut i8
                    } else {
                        b"bytecode\0".as_ptr() as *mut i8
                    },
                )
            };
        }
        rc = 1 as i32;
    } else {
        bytecodevtabNext(pVtabCursor);
    }
    return rc;
}

/// We must have a single stmt=? constraint that will be passed through
/// into the xFilter method.  If there is no valid stmt=? constraint,
/// then return an SQLITE_CONSTRAINT error.
#[unsafe(link_section = ".text.slate_distinct.vdbevtab.bytecodevtabBestIndex")]
extern "C-unwind" fn bytecodevtabBestIndex(
    mut tab: *mut sqlite3_vtab,
    mut pIdxInfo: *mut sqlite3_index_info,
) -> i32 {
    let mut i: i32 = 0 as i32;
    let mut rc: i32 = 19 as i32;
    let mut p: *mut sqlite3_index_constraint = unsafe { std::mem::zeroed() };
    let mut pVTab: *mut bytecodevtab = tab as *mut bytecodevtab;
    let mut iBaseCol: i32 = if (unsafe { (*pVTab).bTablesUsed }) != (0 as i32) {
        4 as i32
    } else {
        10 as i32
    };
    unsafe {
        (*pIdxInfo).estimatedCost = (100 as i32) as f64;
    }
    unsafe {
        (*pIdxInfo).estimatedRows = (100 as i32) as i64;
    }
    unsafe {
        (*pIdxInfo).idxNum = 0 as i32;
    }
    i = 0 as i32;
    let __v464: *mut sqlite3_index_constraint = unsafe { (*pIdxInfo).aConstraint };
    p = __v464;
    '__slate_break_459: while i < unsafe { (*pIdxInfo).nConstraint } {
        if (((unsafe { (*p).usable }) as u32) as i32) == (0 as i32) {
        } else {
            if (((unsafe { (*p).op }) as u32) as i32) == (2 as i32)
                && (unsafe { (*p).iColumn }) == iBaseCol + (1 as i32)
            {
                rc = 0 as i32;
                unsafe {
                    (*unsafe { unsafe { (*pIdxInfo).aConstraintUsage }.offset(i as isize) }).omit =
                        ((1 as i32) as i8) as u8;
                }
                unsafe {
                    (*unsafe { unsafe { (*pIdxInfo).aConstraintUsage }.offset(i as isize) })
                        .argvIndex = 1 as i32;
                }
            }
            if (((unsafe { (*p).op }) as u32) as i32) == (71 as i32)
                && (unsafe { (*p).iColumn }) == iBaseCol
            {
                unsafe {
                    (*unsafe { unsafe { (*pIdxInfo).aConstraintUsage }.offset(i as isize) }).omit =
                        ((1 as i32) as i8) as u8;
                }
                unsafe {
                    (*pIdxInfo).idxNum = 1 as i32;
                }
            }
        }
        let __v465: i32 = i;
        let __v466: i32 = __v465 + (1 as i32);
        i = __v466;
        let __v467: *mut sqlite3_index_constraint = p;
        let __v468: *mut sqlite3_index_constraint = unsafe { __v467.offset((1 as i32) as isize) };
        p = __v468;
    }
    return rc;
}

/// This following structure defines all the methods for the
/// virtual table.
/// iVersion
/// xCreate
/// xConnect
/// xBestIndex
/// xDisconnect
/// xDestroy
/// xOpen
/// xClose
/// xFilter
/// xNext
/// xEof
/// xColumn
/// xRowid
/// xUpdate
/// xBegin
/// xSync
/// xCommit
/// xRollback
/// xFindMethod
/// xRename
/// xSavepoint
/// xRelease
/// xRollbackTo
/// xShadowName
/// xIntegrity
static mut bytecodevtabModule: sqlite3_module = sqlite3_module {
    iVersion: 0 as i32,
    xCreate: None,
    xConnect: Some(bytecodevtabConnect),
    xBestIndex: Some(bytecodevtabBestIndex),
    xDisconnect: Some(bytecodevtabDisconnect),
    xDestroy: None,
    xOpen: Some(bytecodevtabOpen),
    xClose: Some(bytecodevtabClose),
    xFilter: Some(bytecodevtabFilter),
    xNext: Some(bytecodevtabNext),
    xEof: Some(bytecodevtabEof),
    xColumn: Some(bytecodevtabColumn),
    xRowid: Some(bytecodevtabRowid),
    xUpdate: None,
    xBegin: None,
    xSync: None,
    xCommit: None,
    xRollback: None,
    xFindFunction: None,
    xRename: None,
    xSavepoint: None,
    xRelease: None,
    xRollbackTo: None,
    xShadowName: None,
    xIntegrity: None,
};

#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.vdbevtab.sqlite3VdbeBytecodeVtabInit")]
extern "C-unwind" fn sqlite3VdbeBytecodeVtabInit(mut db: *mut sqlite3) -> i32 {
    let mut rc: i32 = 0 as i32;
    rc = unsafe {
        sqlite3_create_module(
            db,
            (b"bytecode\0".as_ptr() as *mut i8) as *const i8,
            (unsafe { std::ptr::addr_of_mut!(bytecodevtabModule) }) as *const sqlite3_module,
            std::ptr::null_mut::<()>(),
        )
    };
    if rc == (0 as i32) {
        rc = unsafe {
            sqlite3_create_module(
                db,
                (b"tables_used\0".as_ptr() as *mut i8) as *const i8,
                (unsafe { std::ptr::addr_of_mut!(bytecodevtabModule) }) as *const sqlite3_module,
                std::ptr::addr_of_mut!(db) as *mut (),
            )
        };
    }
    return rc;
}
