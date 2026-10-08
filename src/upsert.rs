//! 2018-04-12
//!
//! The author disclaims copyright to this source code.  In place of
//! a legal notice, here is a blessing:
//!
//!    May you do good and not evil.
//!    May you find forgiveness for yourself and forgive others.
//!    May you share freely, never taking more than you give.
//!
//!
//! This file contains code to implement various aspects of UPSERT
//! processing and handling of the Upsert object.
unsafe extern "C" {
    fn sqlite3_snprintf(__v362: i32, __v363: *mut i8, __v364: *const i8, ...) -> *mut i8;
    fn memset(__s: *mut (), __c: i32, __n: u64) -> *mut ();
    fn sqlite3VdbeAddOp1(__v368: *mut Vdbe, __v369: i32, __v370: i32) -> i32;
    fn sqlite3VdbeAddOp2(__v371: *mut Vdbe, __v372: i32, __v373: i32, __v374: i32) -> i32;
    fn sqlite3VdbeAddOp3(
        __v375: *mut Vdbe,
        __v376: i32,
        __v377: i32,
        __v378: i32,
        __v379: i32,
    ) -> i32;
    fn sqlite3VdbeAddOp4(
        __v380: *mut Vdbe,
        __v381: i32,
        __v382: i32,
        __v383: i32,
        __v384: i32,
        zP4: *const i8,
        __v386: i32,
    ) -> i32;
    fn sqlite3VdbeAddOp4Int(
        __v387: *mut Vdbe,
        __v388: i32,
        __v389: i32,
        __v390: i32,
        __v391: i32,
        __v392: i32,
    ) -> i32;
    fn sqlite3VdbeJumpHere(__v393: *mut Vdbe, addr: i32);
    fn sqlite3VdbeComment(__v395: *mut Vdbe, __v396: *const i8, ...);
    fn sqlite3VdbeNoopComment(__v397: *mut Vdbe, __v398: *const i8, ...);
    fn sqlite3DbMallocZero(__v399: *mut sqlite3, __v400: u64) -> *mut ();
    fn sqlite3DbFree(__v401: *mut sqlite3, __v402: *mut ());
    fn sqlite3ErrorMsg(__v403: *mut Parse, __v404: *const i8, ...);
    fn sqlite3GetTempReg(__v405: *mut Parse) -> i32;
    fn sqlite3ReleaseTempReg(__v406: *mut Parse, __v407: i32);
    fn sqlite3ExprDelete(__v408: *mut sqlite3, __v409: *mut Expr);
    fn sqlite3ExprListDelete(__v410: *mut sqlite3, __v411: *mut ExprList);
    fn sqlite3PrimaryKeyIndex(__v412: *mut Table) -> *mut Index;
    fn sqlite3TableColumnToIndex(__v413: *mut Index, __v414: i32) -> i32;
    fn sqlite3TableColumnToStorage(__v415: *mut Table, __v416: i16) -> i16;
    fn sqlite3Update(
        __v417: *mut Parse,
        __v418: *mut SrcList,
        __v419: *mut ExprList,
        __v420: *mut Expr,
        __v421: i32,
        __v422: *mut ExprList,
        __v423: *mut Expr,
        __v424: *mut Upsert,
    );
    fn sqlite3ExprCompare(
        __v425: *const Parse,
        __v426: *const Expr,
        __v427: *const Expr,
        __v428: i32,
    ) -> i32;
    fn sqlite3MayAbort(__v429: *mut Parse);
    fn sqlite3ExprDup(__v430: *mut sqlite3, __v431: *const Expr, __v432: i32) -> *mut Expr;
    fn sqlite3ExprListDup(
        __v433: *mut sqlite3,
        __v434: *const ExprList,
        __v435: i32,
    ) -> *mut ExprList;
    fn sqlite3SrcListDup(__v436: *mut sqlite3, __v437: *const SrcList, __v438: i32)
    -> *mut SrcList;
    fn sqlite3ResolveExprNames(__v439: *mut NameContext, __v440: *mut Expr) -> i32;
    fn sqlite3ResolveExprListNames(__v441: *mut NameContext, __v442: *mut ExprList) -> i32;
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
    trace: __SlateRecord156,
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
    u1: __SlateRecord157,
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
    u: __SlateRecord158,
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
    u: __SlateRecord159,
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
    u: __SlateRecord167,
    pLeft: *mut Expr,
    pRight: *mut Expr,
    x: __SlateRecord168,
    nHeight: i32,
    iTable: i32,
    iColumn: i16,
    iAgg: i16,
    w: __SlateRecord169,
    pAggInfo: *mut AggInfo,
    y: __SlateRecord170,
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
struct RenameToken {}

#[repr(C)]
#[derive(Clone, Copy)]
struct SrcItem {
    zName: *mut i8,
    zAlias: *mut i8,
    pSTab: *mut Table,
    fg: __SlateRecord177,
    iCursor: i32,
    colUsed: u64,
    u1: __SlateRecord178,
    u2: __SlateRecord179,
    u3: __SlateRecord180,
    u4: __SlateRecord181,
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
struct NameContext {
    pParse: *mut Parse,
    pSrcList: *mut SrcList,
    uNC: __SlateRecord182,
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
    __slate_bits_0: __slate_bits::__SlateBits99U0,
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
    __slate_bits_0: __slate_bits::__SlateBits155U0,
    azInit: *mut *const i8,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord156 {
    xLegacy: Option<unsafe extern "C-unwind" fn(*mut (), *const i8)>,
    xV2: Option<unsafe extern "C-unwind" fn(u32, *mut (), *mut (), *mut ()) -> i32>,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord157 {
    isInterrupted: i32,
    notUsed1: f64,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord158 {
    pHash: *mut FuncDef,
    pDestructor: *mut FuncDestructor,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord159 {
    tab: __SlateRecord160,
    view: __SlateRecord161,
    vtab: __SlateRecord162,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord160 {
    addColOffset: i32,
    pFKey: *mut FKey,
    pDfltList: *mut ExprList,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord161 {
    pSelect: *mut Select,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord162 {
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
union __SlateRecord167 {
    zToken: *mut i8,
    iValue: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord168 {
    pList: *mut ExprList,
    pSelect: *mut Select,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord169 {
    iJoin: i32,
    iOfst: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord170 {
    pTab: *mut Table,
    pWin: *mut Window,
    nReg: i32,
    sub: __SlateRecord171,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord171 {
    iAddr: i32,
    regReturn: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct ExprList_item {
    pExpr: *mut Expr,
    zEName: *mut i8,
    fg: __SlateRecord173,
    u: __SlateRecord174,
}

#[repr(C, align(4))]
#[derive(Clone, Copy)]
struct __SlateRecord173 {
    sortFlags: u8,
    __slate_bits_0: __slate_bits::__SlateBits173U0,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord174 {
    x: __SlateRecord175,
    iConstExprReg: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord175 {
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
struct __SlateRecord177 {
    jointype: u8,
    __slate_bits_0: __slate_bits::__SlateBits177U0,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord178 {
    zIndexedBy: *mut i8,
    pFuncArg: *mut ExprList,
    nRow: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord179 {
    pIBIndex: *mut Index,
    pCteUse: *mut CteUse,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord180 {
    pOn: *mut Expr,
    pUsing: *mut IdList,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord181 {
    pSchema: *mut Schema,
    zDatabase: *mut i8,
    pSubq: *mut Subquery,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord182 {
    pEList: *mut ExprList,
    pAggInfo: *mut AggInfo,
    pUpsert: *mut Upsert,
    iBaseReg: i32,
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
    pub struct __SlateBits173U0 {
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
    pub struct __SlateBits177U0 {
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
    pub struct __SlateBits155U0 {
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
    pub struct __SlateBits99U0 {
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

/// Free a list of Upsert objects
fn upsertDelete(mut db: *mut sqlite3, mut p: *mut Upsert) {
    '__slate_break_465: loop {
        let mut pNext: *mut Upsert = unsafe { (*p).pNextUpsert };
        unsafe { sqlite3ExprListDelete(db, unsafe { (*p).pUpsertTarget }) };
        unsafe { sqlite3ExprDelete(db, unsafe { (*p).pUpsertTargetWhere }) };
        unsafe { sqlite3ExprListDelete(db, unsafe { (*p).pUpsertSet }) };
        unsafe { sqlite3ExprDelete(db, unsafe { (*p).pUpsertWhere }) };
        unsafe { sqlite3DbFree(db, unsafe { (*p).pToFree }) };
        unsafe { sqlite3DbFree(db, p as *mut ()) };
        p = pNext;
        if !(p != std::ptr::null_mut::<Upsert>()) {
            break;
        }
    }
}

#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3UpsertDelete(mut db: *mut sqlite3, mut p: *mut Upsert) {
    if p != std::ptr::null_mut::<Upsert>() {
        upsertDelete(db, p);
    }
}

/// Duplicate an Upsert object.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3UpsertDup(mut db: *mut sqlite3, mut p: *mut Upsert) -> *mut Upsert {
    if p == std::ptr::null_mut::<Upsert>() {
        return std::ptr::null_mut::<Upsert>();
    }
    return sqlite3UpsertNew(
        db,
        unsafe {
            sqlite3ExprListDup(
                db,
                (unsafe { (*p).pUpsertTarget }) as *const ExprList,
                0 as i32,
            )
        },
        unsafe {
            sqlite3ExprDup(
                db,
                (unsafe { (*p).pUpsertTargetWhere }) as *const Expr,
                0 as i32,
            )
        },
        unsafe {
            sqlite3ExprListDup(
                db,
                (unsafe { (*p).pUpsertSet }) as *const ExprList,
                0 as i32,
            )
        },
        unsafe { sqlite3ExprDup(db, (unsafe { (*p).pUpsertWhere }) as *const Expr, 0 as i32) },
        sqlite3UpsertDup(db, unsafe { (*p).pNextUpsert }),
    );
}

/// Create a new Upsert object.
///
/// # Arguments
///
/// * `db` - Determines which memory allocator to use
/// * `pTarget` - Target argument to ON CONFLICT, or NULL
/// * `pTargetWhere` - Optional WHERE clause on the target
/// * `pSet` - UPDATE columns, or NULL for a DO NOTHING
/// * `pWhere` - WHERE clause for the ON CONFLICT UPDATE
/// * `pNext` - Next ON CONFLICT clause in the list
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3UpsertNew(
    mut db: *mut sqlite3,
    mut pTarget: *mut ExprList,
    mut pTargetWhere: *mut Expr,
    mut pSet: *mut ExprList,
    mut pWhere: *mut Expr,
    mut pNext: *mut Upsert,
) -> *mut Upsert {
    let mut pNew: *mut Upsert = unsafe { std::mem::zeroed() };
    pNew = (unsafe { sqlite3DbMallocZero(db, 88 as u64) }) as *mut Upsert;
    if pNew == std::ptr::null_mut::<Upsert>() {
        unsafe { sqlite3ExprListDelete(db, pTarget) };
        unsafe { sqlite3ExprDelete(db, pTargetWhere) };
        unsafe { sqlite3ExprListDelete(db, pSet) };
        unsafe { sqlite3ExprDelete(db, pWhere) };
        sqlite3UpsertDelete(db, pNext);
        return std::ptr::null_mut::<Upsert>();
    } else {
        unsafe {
            (*pNew).pUpsertTarget = pTarget;
        }
        unsafe {
            (*pNew).pUpsertTargetWhere = pTargetWhere;
        }
        unsafe {
            (*pNew).pUpsertSet = pSet;
        }
        unsafe {
            (*pNew).pUpsertWhere = pWhere;
        }
        unsafe {
            (*pNew).isDoUpdate = (pSet != std::ptr::null_mut::<ExprList>()) as u8;
        }
        unsafe {
            (*pNew).pNextUpsert = pNext;
        }
    }
    return pNew;
}

/// Analyze the ON CONFLICT clause described by pUpsert.  Resolve all
/// symbols in the conflict-target.
///
/// Return SQLITE_OK if everything works, or an error code is something
/// is wrong.
///
/// # Arguments
///
/// * `pParse` - The parsing context
/// * `pTabList` - Table into which we are inserting
/// * `pUpsert` - The ON CONFLICT clauses
/// * `pAll` - Complete list of all ON CONFLICT clauses
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3UpsertAnalyzeTarget(
    mut pParse: *mut Parse,
    mut pTabList: *mut SrcList,
    mut pUpsert: *mut Upsert,
    mut pAll: *mut Upsert,
) -> i32 {
    let mut pTab: *mut Table = unsafe { std::mem::zeroed() }; // That table into which we are inserting
    let mut rc: i32 = 0 as i32; // Result code
    let mut iCursor: i32 = 0 as i32; // Cursor used by pTab
    let mut pIdx: *mut Index = unsafe { std::mem::zeroed() }; // One of the indexes of pTab
    let mut pTarget: *mut ExprList = unsafe { std::mem::zeroed() }; // The conflict-target clause
    let mut pTerm: *mut Expr = unsafe { std::mem::zeroed() }; // One term of the conflict-target clause
    let mut sNC: NameContext = unsafe { std::mem::zeroed() }; // Context for resolving symbolic names
    let mut sCol: __SlateAlign16<[Expr; 2]> = __SlateAlign16(unsafe { std::mem::zeroed() }); // Index column converted into an Expr
    let mut nClause: i32 = 0 as i32; // Counter of ON CONFLICT clauses
    0 as i32;
    0 as i32;
    0 as i32;
    0 as i32;
    // Resolve all symbolic names in the conflict-target clause, which
    // includes both the list of columns and the optional partial-index
    // WHERE clause.
    unsafe { memset(std::ptr::addr_of_mut!(sNC) as *mut (), 0 as i32, 56 as u64) };
    sNC.pParse = pParse;
    sNC.pSrcList = pTabList;
    '__slate_break_466: loop {
        if !(pUpsert != std::ptr::null_mut::<Upsert>()
            && (unsafe { (*pUpsert).pUpsertTarget }) != std::ptr::null_mut::<ExprList>())
        {
            break;
        }
        rc = unsafe {
            sqlite3ResolveExprListNames(std::ptr::addr_of_mut!(sNC), unsafe {
                (*pUpsert).pUpsertTarget
            })
        };
        if rc != (0 as i32) {
            return rc;
        }
        rc = unsafe {
            sqlite3ResolveExprNames(std::ptr::addr_of_mut!(sNC), unsafe {
                (*pUpsert).pUpsertTargetWhere
            })
        };
        if rc != (0 as i32) {
            return rc;
        }
        // Check to see if the conflict target matches the rowid.
        pTab = unsafe {
            (*unsafe {
                unsafe { std::ptr::addr_of_mut!((*pTabList).a) as *mut SrcItem }
                    .offset((0 as i32) as isize)
            })
            .pSTab
        };
        pTarget = unsafe { (*pUpsert).pUpsertTarget };
        iCursor = unsafe {
            (*unsafe {
                unsafe { std::ptr::addr_of_mut!((*pTabList).a) as *mut SrcItem }
                    .offset((0 as i32) as isize)
            })
            .iCursor
        };
        let __v482: bool;
        if (unsafe { (*pTab).tabFlags }) & ((128 as i32) as u32) == ((0 as i32) as u32)
            && (unsafe { (*pTarget).nExpr }) == (1 as i32)
        {
            let __v483: *mut Expr = unsafe {
                (*unsafe {
                    unsafe { std::ptr::addr_of_mut!((*pTarget).a) as *mut ExprList_item }
                        .offset((0 as i32) as isize)
                })
                .pExpr
            };
            pTerm = __v483;
            __v482 = (((unsafe { (*__v483).op }) as u32) as i32) == (168 as i32);
        } else {
            __v482 = false as bool;
        }
        if __v482 && ((unsafe { (*pTerm).iColumn }) as i32) == -(1 as i32) {
            // The conflict-target is the rowid of the primary table
            0 as i32;
        } else {
            // Initialize sCol[0..1] to be an expression parse tree for a
            // single column of an index.  The sCol[0] node will be the TK_COLLATE
            // operator and sCol[1] will be the TK_COLUMN operator.  Code below
            // will populate the specific collation and column number values
            // prior to comparing against the conflict-target expression.
            unsafe {
                memset(
                    (sCol.0.as_mut_ptr() as *mut Expr) as *mut (),
                    0 as i32,
                    144 as u64,
                )
            };
            unsafe {
                (*unsafe { (sCol.0.as_mut_ptr() as *mut Expr).offset((0 as i32) as isize) }).op =
                    ((114 as i32) as i8) as u8;
            }
            unsafe {
                (*unsafe { (sCol.0.as_mut_ptr() as *mut Expr).offset((0 as i32) as isize) })
                    .pLeft =
                    unsafe { (sCol.0.as_mut_ptr() as *mut Expr).offset((1 as i32) as isize) };
            }
            unsafe {
                (*unsafe { (sCol.0.as_mut_ptr() as *mut Expr).offset((1 as i32) as isize) }).op =
                    ((168 as i32) as i8) as u8;
            }
            unsafe {
                (*unsafe { (sCol.0.as_mut_ptr() as *mut Expr).offset((1 as i32) as isize) })
                    .iTable = unsafe {
                    (*unsafe {
                        unsafe { std::ptr::addr_of_mut!((*pTabList).a) as *mut SrcItem }
                            .offset((0 as i32) as isize)
                    })
                    .iCursor
                };
            }
            // Check for matches against other indexes
            pIdx = unsafe { (*pTab).pIndex };
            '__slate_break_467: while pIdx != std::ptr::null_mut::<Index>() {
                '__slate_continue_467: {
                    let mut ii: i32 = 0 as i32;
                    let mut jj: i32 = 0 as i32;
                    let mut nn: i32 = 0 as i32;
                    if !((((unsafe { (*pIdx).onError }) as u32) as i32) != (0 as i32)) {
                    } else {
                        if (unsafe { (*pTarget).nExpr })
                            != (((unsafe { (*pIdx).nKeyCol }) as u32) as i32)
                        {
                        } else {
                            if (unsafe { (*pIdx).pPartIdxWhere }) != std::ptr::null_mut::<Expr>() {
                                if (unsafe { (*pUpsert).pUpsertTargetWhere })
                                    == std::ptr::null_mut::<Expr>()
                                {
                                    break '__slate_continue_467;
                                }
                                if (unsafe {
                                    sqlite3ExprCompare(
                                        pParse as *const Parse,
                                        (unsafe { (*pUpsert).pUpsertTargetWhere }) as *const Expr,
                                        (unsafe { (*pIdx).pPartIdxWhere }) as *const Expr,
                                        iCursor,
                                    )
                                }) != (0 as i32)
                                {
                                    break '__slate_continue_467;
                                }
                            }
                            nn = ((unsafe { (*pIdx).nKeyCol }) as u32) as i32;
                            ii = 0 as i32;
                            '__slate_break_468: loop {
                                if !(ii < nn) {
                                    break;
                                }
                                let mut pExpr: *mut Expr = unsafe { std::mem::zeroed() };
                                unsafe {
                                    (*unsafe {
                                        (sCol.0.as_mut_ptr() as *mut Expr)
                                            .offset((0 as i32) as isize)
                                    })
                                    .u
                                    .zToken = (unsafe {
                                        *unsafe { unsafe { (*pIdx).azColl }.offset(ii as isize) }
                                    }) as *mut i8;
                                }
                                if ((unsafe {
                                    *unsafe { unsafe { (*pIdx).aiColumn }.offset(ii as isize) }
                                }) as i32)
                                    == -(2 as i32)
                                {
                                    0 as i32;
                                    0 as i32;
                                    0 as i32;
                                    pExpr = unsafe {
                                        (*unsafe {
                                            unsafe {
                                                std::ptr::addr_of_mut!(
                                                    (*unsafe { (*pIdx).aColExpr }).a
                                                )
                                                    as *mut ExprList_item
                                            }
                                            .offset(ii as isize)
                                        })
                                        .pExpr
                                    };
                                    if (((unsafe { (*pExpr).op }) as u32) as i32) != (114 as i32) {
                                        unsafe {
                                            (*unsafe {
                                                (sCol.0.as_mut_ptr() as *mut Expr)
                                                    .offset((0 as i32) as isize)
                                            })
                                            .pLeft = pExpr;
                                        }
                                        pExpr = unsafe {
                                            (sCol.0.as_mut_ptr() as *mut Expr)
                                                .offset((0 as i32) as isize)
                                        };
                                    }
                                } else {
                                    unsafe {
                                        (*unsafe {
                                            (sCol.0.as_mut_ptr() as *mut Expr)
                                                .offset((0 as i32) as isize)
                                        })
                                        .pLeft = unsafe {
                                            (sCol.0.as_mut_ptr() as *mut Expr)
                                                .offset((1 as i32) as isize)
                                        };
                                    }
                                    unsafe {
                                        (*unsafe {
                                            (sCol.0.as_mut_ptr() as *mut Expr)
                                                .offset((1 as i32) as isize)
                                        })
                                        .iColumn = unsafe {
                                            *unsafe {
                                                unsafe { (*pIdx).aiColumn }.offset(ii as isize)
                                            }
                                        };
                                    }
                                    pExpr = unsafe {
                                        (sCol.0.as_mut_ptr() as *mut Expr)
                                            .offset((0 as i32) as isize)
                                    };
                                }
                                jj = 0 as i32;
                                '__slate_break_469: loop {
                                    if !(jj < nn) {
                                        break;
                                    }
                                    if (unsafe {
                                        sqlite3ExprCompare(
                                            std::ptr::null::<Parse>(),
                                            (unsafe {
                                                (*unsafe {
                                                    unsafe {
                                                        std::ptr::addr_of_mut!((*pTarget).a)
                                                            as *mut ExprList_item
                                                    }
                                                    .offset(jj as isize)
                                                })
                                                .pExpr
                                            })
                                                as *const Expr,
                                            pExpr as *const Expr,
                                            iCursor,
                                        )
                                    }) < (2 as i32)
                                    {
                                        break '__slate_break_469; // Column ii of the index matches column jj of target
                                    }
                                    let __v486: i32 = jj;
                                    let __v487: i32 = __v486 + (1 as i32);
                                    jj = __v487;
                                }
                                if jj >= nn {
                                    // The target contains no match for column jj of the index
                                    break '__slate_break_468;
                                }
                                let __v484: i32 = ii;
                                let __v485: i32 = __v484 + (1 as i32);
                                ii = __v485;
                            }
                            if ii < nn {
                                // Column ii of the index did not match any term of the conflict target.
                                // Continue the search with the next index.
                            } else {
                                unsafe {
                                    (*pUpsert).pUpsertIdx = pIdx;
                                }
                                if sqlite3UpsertOfIndex(pAll, pIdx) != pUpsert {
                                    // Really this should be an error.  The isDup ON CONFLICT clause will
                                    // never fire.  But this problem was not discovered until three years
                                    // after multi-CONFLICT upsert was added, and so we silently ignore
                                    // the problem to prevent breaking applications that might actually
                                    // have redundant ON CONFLICT clauses.
                                    unsafe {
                                        (*pUpsert).isDup = ((1 as i32) as i8) as u8;
                                    }
                                }
                                break '__slate_break_467;
                            }
                        }
                    }
                }
                pIdx = unsafe { (*pIdx).pNext };
            }
            if (unsafe { (*pUpsert).pUpsertIdx }) == std::ptr::null_mut::<Index>() {
                let mut zWhich: __SlateAlign16<[i8; 16]> = __SlateAlign16([0 as i8; 16]);
                if nClause == (0 as i32)
                    && (unsafe { (*pUpsert).pNextUpsert }) == std::ptr::null_mut::<Upsert>()
                {
                    unsafe {
                        *unsafe {
                            (zWhich.0.as_mut_ptr() as *mut i8).offset((0 as i32) as isize)
                        } = (0 as i32) as i8;
                    }
                } else {
                    unsafe {
                        sqlite3_snprintf(
                            ((16 as u64) as u32) as i32,
                            zWhich.0.as_mut_ptr() as *mut i8,
                            (b"%r \0".as_ptr() as *mut i8) as *const i8,
                            nClause + (1 as i32),
                        )
                    };
                }
                unsafe {
                    sqlite3ErrorMsg(pParse, (b"%sON CONFLICT clause does not match any PRIMARY KEY or UNIQUE constraint\0".as_ptr() as *mut i8) as *const i8, zWhich.0.as_mut_ptr() as *mut i8)
                };
                return 1 as i32;
            }
        }
        pUpsert = unsafe { (*pUpsert).pNextUpsert };
        let __v480: i32 = nClause;
        let __v481: i32 = __v480 + (1 as i32);
        nClause = __v481;
    }
    return 0 as i32;
}

/// Return true if pUpsert is the last ON CONFLICT clause with a
/// conflict target, or if pUpsert is followed by another ON CONFLICT
/// clause that targets the INTEGER PRIMARY KEY.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3UpsertNextIsIPK(mut pUpsert: *mut Upsert) -> i32 {
    let mut pNext: *mut Upsert = unsafe { std::mem::zeroed() };
    if pUpsert == std::ptr::null_mut::<Upsert>() {
        return 0 as i32;
    }
    pNext = unsafe { (*pUpsert).pNextUpsert };
    '__slate_break_472: while (1 as i32) != (0 as i32) {
        // exit-by-return
        if pNext == std::ptr::null_mut::<Upsert>() {
            return 1 as i32;
        }
        if (unsafe { (*pNext).pUpsertTarget }) == std::ptr::null_mut::<ExprList>() {
            return 1 as i32;
        }
        if (unsafe { (*pNext).pUpsertIdx }) == std::ptr::null_mut::<Index>() {
            return 1 as i32;
        }
        if !((unsafe { (*pNext).isDup }) != (0 as u8)) {
            return 0 as i32;
        }
        pNext = unsafe { (*pNext).pNextUpsert };
    }
    return 0 as i32;
}

/// Given the list of ON CONFLICT clauses described by pUpsert, and
/// a particular index pIdx, return a pointer to the particular ON CONFLICT
/// clause that applies to the index.  Or, if the index is not subject to
/// any ON CONFLICT clause, return NULL.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3UpsertOfIndex(
    mut pUpsert: *mut Upsert,
    mut pIdx: *mut Index,
) -> *mut Upsert {
    '__slate_break_473: while pUpsert != std::ptr::null_mut::<Upsert>()
        && (unsafe { (*pUpsert).pUpsertTarget }) != std::ptr::null_mut::<ExprList>()
        && (unsafe { (*pUpsert).pUpsertIdx }) != pIdx
    {
        pUpsert = unsafe { (*pUpsert).pNextUpsert };
    }
    return pUpsert;
}

/// Generate bytecode that does an UPDATE as part of an upsert.
///
/// If pIdx is NULL, then the UNIQUE constraint that failed was the IPK.
/// In this case parameter iCur is a cursor open on the table b-tree that
/// currently points to the conflicting table row. Otherwise, if pIdx
/// is not NULL, then pIdx is the constraint that failed and iCur is a
/// cursor points to the conflicting row.
///
/// # Arguments
///
/// * `pParse` - The parsing and code-generating context
/// * `pUpsert` - The ON CONFLICT clause for the upsert
/// * `pTab` - The table being updated
/// * `pIdx` - The UNIQUE constraint that failed
/// * `iCur` - Cursor for pIdx (or pTab if pIdx==NULL)
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3UpsertDoUpdate(
    mut pParse: *mut Parse,
    mut pUpsert: *mut Upsert,
    mut pTab: *mut Table,
    mut pIdx: *mut Index,
    mut iCur: i32,
) {
    let mut v: *mut Vdbe = unsafe { (*pParse).pVdbe };
    let mut db: *mut sqlite3 = unsafe { (*pParse).db };
    let mut pSrc: *mut SrcList = unsafe { std::mem::zeroed() }; // FROM clause for the UPDATE
    let mut iDataCur: i32 = 0 as i32;
    let mut i: i32 = 0 as i32;
    let mut pTop: *mut Upsert = pUpsert;
    0 as i32;
    0 as i32;
    iDataCur = unsafe { (*pUpsert).iDataCur };
    pUpsert = sqlite3UpsertOfIndex(pTop, pIdx);
    unsafe {
        sqlite3VdbeNoopComment(
            v,
            (b"Begin DO UPDATE of UPSERT\0".as_ptr() as *mut i8) as *const i8,
        )
    };
    if pIdx != std::ptr::null_mut::<Index>() && iCur != iDataCur {
        if (unsafe { (*pTab).tabFlags }) & ((128 as i32) as u32) == ((0 as i32) as u32) {
            let mut regRowid: i32 = unsafe { sqlite3GetTempReg(pParse) };
            unsafe { sqlite3VdbeAddOp2(v, 144 as i32, iCur, regRowid) };
            unsafe { sqlite3VdbeAddOp3(v, 30 as i32, iDataCur, 0 as i32, regRowid) };
            {}
            unsafe { sqlite3ReleaseTempReg(pParse, regRowid) };
        } else {
            let mut pPk: *mut Index = unsafe { sqlite3PrimaryKeyIndex(pTab) };
            let mut nPk: i32 = ((unsafe { (*pPk).nKeyCol }) as u32) as i32;
            let mut iPk: i32 = (unsafe { (*pParse).nMem }) + (1 as i32);
            let __v488: *mut Parse = pParse;
            let __v489: i32 = unsafe { (*__v488).nMem };
            let __v490: i32 = __v489 + nPk;
            unsafe {
                (*__v488).nMem = __v490;
            }
            i = 0 as i32;
            '__slate_break_475: loop {
                if !(i < nPk) {
                    break;
                }
                let mut k: i32 = 0 as i32;
                0 as i32;
                k = unsafe {
                    sqlite3TableColumnToIndex(
                        pIdx,
                        (unsafe { *unsafe { unsafe { (*pPk).aiColumn }.offset(i as isize) } })
                            as i32,
                    )
                };
                unsafe { sqlite3VdbeAddOp3(v, 96 as i32, iCur, k, iPk + i) };
                unsafe {
                    sqlite3VdbeComment(
                        v,
                        (b"%s.%s\0".as_ptr() as *mut i8) as *const i8,
                        unsafe { (*pIdx).zName },
                        unsafe {
                            (*unsafe {
                                unsafe { (*pTab).aCol }.offset(
                                    ((unsafe {
                                        *unsafe { unsafe { (*pPk).aiColumn }.offset(i as isize) }
                                    }) as i32) as isize,
                                )
                            })
                            .zCnName
                        },
                    )
                };
                let __v491: i32 = i;
                let __v492: i32 = __v491 + (1 as i32);
                i = __v492;
            }
            {}
            i = unsafe { sqlite3VdbeAddOp4Int(v, 29 as i32, iDataCur, 0 as i32, iPk, nPk) };
            {}
            unsafe {
                sqlite3VdbeAddOp4(
                    v,
                    72 as i32,
                    11 as i32,
                    2 as i32,
                    0 as i32,
                    (b"corrupt database\0".as_ptr() as *mut i8) as *const i8,
                    -(1 as i32),
                )
            };
            unsafe { sqlite3MayAbort(pParse) };
            unsafe { sqlite3VdbeJumpHere(v, i) };
        }
    }
    // pUpsert does not own pTop->pUpsertSrc - the outer INSERT statement does.
    // So we have to make a copy before passing it down into sqlite3Update()
    pSrc = unsafe {
        sqlite3SrcListDup(
            db,
            (unsafe { (*pTop).pUpsertSrc }) as *const SrcList,
            0 as i32,
        )
    };
    // excluded.* columns of type REAL need to be converted to a hard real
    i = 0 as i32;
    '__slate_break_478: loop {
        if !(i < ((unsafe { (*pTab).nCol }) as i32)) {
            break;
        }
        if ((unsafe { (*unsafe { unsafe { (*pTab).aCol }.offset(i as isize) }).affinity }) as i32)
            == (69 as i32)
        {
            let mut iStorage: i32 = (unsafe { (*pTop).regData })
                + ((unsafe { sqlite3TableColumnToStorage(pTab, i as i16) }) as i32);
            unsafe { sqlite3VdbeAddOp1(v, 89 as i32, iStorage) };
        }
        let __v493: i32 = i;
        let __v494: i32 = __v493 + (1 as i32);
        i = __v494;
    }
    unsafe {
        sqlite3Update(
            pParse,
            pSrc,
            unsafe {
                sqlite3ExprListDup(
                    db,
                    (unsafe { (*pUpsert).pUpsertSet }) as *const ExprList,
                    0 as i32,
                )
            },
            unsafe {
                sqlite3ExprDup(
                    db,
                    (unsafe { (*pUpsert).pUpsertWhere }) as *const Expr,
                    0 as i32,
                )
            },
            2 as i32,
            std::ptr::null_mut::<ExprList>(),
            std::ptr::null_mut::<Expr>(),
            pUpsert,
        )
    };
    unsafe {
        sqlite3VdbeNoopComment(
            v,
            (b"End DO UPDATE of UPSERT\0".as_ptr() as *mut i8) as *const i8,
        )
    };
}
