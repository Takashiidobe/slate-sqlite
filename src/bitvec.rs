//! 2008 February 16
//!
//! The author disclaims copyright to this source code.  In place of
//! a legal notice, here is a blessing:
//!
//!    May you do good and not evil.
//!    May you find forgiveness for yourself and forgive others.
//!    May you share freely, never taking more than you give.
//!
//!
//! This file implements an object that represents a fixed-length
//! bitmap.  Bits are numbered starting with 1.
//!
//! A bitmap is used to record which pages of a database file have been
//! journalled during a transaction, or which pages have the "dont-write"
//! property.  Usually only a few pages are meet either condition.
//! So the bitmap is usually sparse and has low cardinality.
//! But sometimes (for example when during a DROP of a large table) most
//! or all of the pages in a database can get journalled.  In those cases,
//! the bitmap becomes dense with high cardinality.  The algorithm needs
//! to handle both cases well.
//!
//! The size of the bitmap is fixed when the object is created.
//!
//! All bits are clear when the bitmap is created.  Individual bits
//! may be set or cleared one at a time.
//!
//! Test operations are about 100 times more common that set operations.
//! Clear operations are exceedingly rare.  There are usually between
//! 5 and 500 set operations per Bitvec object, though the number of sets can
//! sometimes grow into tens of thousands or larger.  The size of the
//! Bitvec object is the number of pages in the database file at the
//! start of a transaction, and is thus usually less than a few thousand,
//! but can be as large as 2 billion for a really big database.
unsafe extern "C" {
    fn sqlite3_malloc64(__v330: u64) -> *mut ();
    fn sqlite3_free(__v331: *mut ());
    fn sqlite3_randomness(N: i32, P: *mut ());
    fn memcpy(__dest: *mut (), __src: *const (), __n: u64) -> *mut ();
    fn memset(__s: *mut (), __c: i32, __n: u64) -> *mut ();
    fn sqlite3MallocZero(__v340: u64) -> *mut ();
    fn sqlite3DbMallocRaw(__v341: *mut sqlite3, __v342: u64) -> *mut ();
    fn sqlite3DbFree(__v343: *mut sqlite3, __v344: *mut ());
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
    __slate_bits_0: __slate_bits::__SlateBits63U0,
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
    __slate_bits_0: __slate_bits::__SlateBits87U0,
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
    u1: __SlateRecord183,
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
struct VtabCtx {}

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
struct Btree {}

#[repr(C)]
#[derive(Clone, Copy)]
struct Vdbe {}

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

// Size of the Bitvec structure in bytes.
// Round the union size down to the nearest pointer boundary, since that's how
// it will be aligned within the Bitvec struct.
// Type of the array "element" for the bitmap representation.
// Should be a power of 2, and ideally, evenly divide into BITVEC_USIZE.
// Setting this to the "natural word" size of your CPU may improve
// performance.
// Size, in bits, of the bitmap element.
// Number of elements in a bitmap array.
// Number of bits in the bitmap array.
// Number of u32 values in hash table.
// Maximum number of entries in hash table before
// sub-dividing and re-hashing.
// Hashing function for the aHash representation.
// Empirical testing showed that the *37 multiplier
// (an arbitrary prime)in the hash function provided
// no fewer collisions than the no-op *1.
/// A bitmap is an instance of the following structure.
///
/// This bitmap records the existence of zero or more bits
/// with values between 1 and iSize, inclusive.
///
/// There are three possible representations of the bitmap.
/// If iSize<=BITVEC_NBIT, then Bitvec.u.aBitmap[] is a straight
/// bitmap.  The least significant bit is bit 1.
///
/// If iSize>BITVEC_NBIT and iDivisor==0 then Bitvec.u.aHash[] is
/// a hash table that will hold up to BITVEC_MXHASH distinct values.
///
/// Otherwise, the value i is redirected into one of BITVEC_NPTR
/// sub-bitmaps pointed to by Bitvec.u.apSub[].  Each subbitmap
/// handles up to iDivisor separate values of i.  apSub[0] holds
/// values between 1 and iDivisor.  apSub[1] holds values between
/// iDivisor+1 and 2*iDivisor.  apSub[N] holds values between
/// N*iDivisor+1 and (N+1)*iDivisor.  Each subbitmap is normalized
/// to hold deal with values between 1 and iDivisor.
#[repr(C)]
#[derive(Clone, Copy)]
struct Bitvec {
    /// Maximum bit index.  Max iSize is 4,294,967,296.
    iSize: u32,
    /// Number of bits that are set - only valid for aHash
    /// element.  Max is BITVEC_NINT.  For BITVEC_SZ of 512,
    /// this would be 125.
    nSet: u32,
    /// Number of bits handled by each apSub[] entry.
    ///
    /// Should >=0 for apSub element.
    ///
    /// Max iDivisor is max(u32) / BITVEC_NPTR + 1.
    ///
    /// For a BITVEC_SZ of 512, this would be 34,359,739.
    iDivisor: u32,
    u: __SlateRecord186,
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
union __SlateRecord183 {
    cr: __SlateRecord184,
    d: __SlateRecord185,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord184 {
    addrCrTab: i32,
    regRowid: i32,
    regRoot: i32,
    constraintName: Token,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord185 {
    pReturning: *mut Returning,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord186 {
    /// Bitmap representation
    aBitmap: [u8; 496],
    /// Hash table representation
    aHash: [u32; 124],
    /// Recursive representation
    apSub: [*mut Bitvec; 62],
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
    pub struct __SlateBits63U0 {
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

/// Create a new bitmap object able to handle bits between 0 and iSize,
/// inclusive.  Return a pointer to the new object.  Return NULL if
/// malloc fails.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3BitvecCreate(mut iSize: u32) -> *mut Bitvec {
    let mut p: *mut Bitvec = unsafe { std::mem::zeroed() };
    0 as i32;
    p = (unsafe { sqlite3MallocZero(512 as u64) }) as *mut Bitvec;
    if p != std::ptr::null_mut::<Bitvec>() {
        unsafe {
            (*p).iSize = iSize;
        }
    }
    return p;
}

/// Check to see if the i-th bit is set.  Return true or false.
/// If p is NULL (if the bitmap has not been created) or if
/// i is out of range, then return false.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3BitvecTestNotNull(mut p: *mut Bitvec, mut i: u32) -> i32 {
    0 as i32;
    let __v372: u32 = i;
    let __v373: u32 = __v372.wrapping_sub((1 as i32) as u32);
    i = __v373;
    if i >= unsafe { (*p).iSize } {
        return 0 as i32;
    }
    '__slate_break_359: while (unsafe { (*p).iDivisor }) != (0 as u32) {
        let mut bin: u32 = i / unsafe { (*p).iDivisor };
        i = i % unsafe { (*p).iDivisor };
        p = unsafe {
            *unsafe {
                unsafe { (*p).u.apSub.as_mut_ptr() as *mut *mut Bitvec }.offset(bin as isize)
            }
        };
        if !(p != std::ptr::null_mut::<Bitvec>()) {
            return 0 as i32;
        }
    }
    if ((unsafe { (*p).iSize }) as u64)
        <= (((((512 as i32) as i64) as u64)
            .wrapping_sub((((3 as i32) as i64) as u64).wrapping_mul(4 as u64))
            / (8 as u64))
            .wrapping_mul(8 as u64)
            / (1 as u64))
            .wrapping_mul(((8 as i32) as i64) as u64)
    {
        return ((((unsafe {
            *unsafe {
                unsafe { (*p).u.aBitmap.as_mut_ptr() as *mut u8 }
                    .offset((i / ((8 as i32) as u32)) as isize)
            }
        }) as u32) as i32)
            & (1 as i32) << (i & (((8 as i32) - (1 as i32)) as u32))
            != (0 as i32)) as i32;
    } else {
        let mut h: u32 = 0 as u32;
        let __v374: u32 = i;
        let __v375: u32 = __v374.wrapping_add((1 as i32) as u32);
        i = __v375;
        h = ((__v374.wrapping_mul((1 as i32) as u32) as u64)
            % (((((512 as i32) as i64) as u64)
                .wrapping_sub((((3 as i32) as i64) as u64).wrapping_mul(4 as u64))
                / (8 as u64))
                .wrapping_mul(8 as u64)
                / (4 as u64))) as u32;
        '__slate_break_360: while (unsafe {
            *unsafe { unsafe { (*p).u.aHash.as_mut_ptr() as *mut u32 }.offset(h as isize) }
        }) != (0 as u32)
        {
            if (unsafe {
                *unsafe { unsafe { (*p).u.aHash.as_mut_ptr() as *mut u32 }.offset(h as isize) }
            }) == i
            {
                return 1 as i32;
            }
            h = ((h.wrapping_add((1 as i32) as u32) as u64)
                % (((((512 as i32) as i64) as u64)
                    .wrapping_sub((((3 as i32) as i64) as u64).wrapping_mul(4 as u64))
                    / (8 as u64))
                    .wrapping_mul(8 as u64)
                    / (4 as u64))) as u32;
        }
        return 0 as i32;
    }
    return unsafe { std::mem::zeroed() };
}

#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3BitvecTest(mut p: *mut Bitvec, mut i: u32) -> i32 {
    let __v371: bool;
    if p != std::ptr::null_mut::<Bitvec>() {
        __v371 = sqlite3BitvecTestNotNull(p, i) != (0 as i32);
    } else {
        __v371 = false as bool;
    }
    return __v371 as i32;
}

/// Set the i-th bit.  Return 0 on success and an error code if
/// anything goes wrong.
///
/// This routine might cause sub-bitmaps to be allocated.  Failing
/// to get the memory needed to hold the sub-bitmap is the only
/// that can go wrong with an insert, assuming p and i are valid.
///
/// The calling function must ensure that p is a valid Bitvec object
/// and that the value for "i" is within range of the Bitvec object.
/// Otherwise the behavior is undefined.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3BitvecSet(mut p: *mut Bitvec, mut i: u32) -> i32 {
    let mut __slate_storage_394: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_394: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_394) as *mut u32;
    let mut __slate_storage_393: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_393: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_393) as *mut u32;
    let mut __slate_storage_392: std::mem::MaybeUninit<*mut Bitvec> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_392: *mut *mut Bitvec =
        std::ptr::addr_of_mut!(__slate_storage_392) as *mut *mut Bitvec;
    let mut __slate_storage_389: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_389: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_389) as *mut u32;
    let mut __slate_storage_388: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_388: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_388) as *mut u32;
    let mut __slate_storage_391: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_391: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_391) as *mut i32;
    let mut __slate_storage_390: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_390: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_390) as *mut i32;
    let mut __slate_storage_387: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_387: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_387) as *mut u32;
    let mut __slate_storage_386: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_386: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_386) as *mut u32;
    let mut __slate_storage_385: std::mem::MaybeUninit<*mut Bitvec> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_385: *mut *mut Bitvec =
        std::ptr::addr_of_mut!(__slate_storage_385) as *mut *mut Bitvec;
    let mut __slate_storage_308: std::mem::MaybeUninit<*mut u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_308: *mut *mut u32 =
        std::ptr::addr_of_mut!(__slate_storage_308) as *mut *mut u32;
    let mut __slate_storage_307: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_307: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_307) as *mut i32;
    let mut __slate_storage_306: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_306: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_306) as *mut u32;
    let mut __slate_storage_384: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_384: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_384) as *mut u32;
    let mut __slate_storage_383: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_383: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_383) as *mut u32;
    let mut __slate_storage_382: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_382: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_382) as *mut u32;
    let mut __slate_storage_381: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_381: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_381) as *mut u32;
    let mut __slate_storage_380: std::mem::MaybeUninit<u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_380: *mut u8 = std::ptr::addr_of_mut!(__slate_storage_380) as *mut u8;
    let mut __slate_storage_379: std::mem::MaybeUninit<u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_379: *mut u8 = std::ptr::addr_of_mut!(__slate_storage_379) as *mut u8;
    let mut __slate_storage_378: std::mem::MaybeUninit<*mut u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_378: *mut *mut u8 =
        std::ptr::addr_of_mut!(__slate_storage_378) as *mut *mut u8;
    let mut __slate_storage_305: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_305: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_305) as *mut u32;
    let mut __slate_storage_377: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_377: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_377) as *mut u32;
    let mut __slate_storage_376: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_376: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_376) as *mut u32;
    let mut __slate_storage_304: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_304: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_304) as *mut u32;
    unsafe {
        if p == std::ptr::null_mut::<Bitvec>() {
            return 0 as i32;
        } else {
            0 as i32;
            0 as i32;
            std::ptr::write(__slate_slot_376, i);
            std::ptr::write(
                __slate_slot_377,
                (*__slate_slot_376).wrapping_sub((1 as i32) as u32),
            );
            i = *__slate_slot_377;
            loop {
                if ((unsafe { (*p).iSize }) as u64)
                    > (((((512 as i32) as i64) as u64)
                        .wrapping_sub((((3 as i32) as i64) as u64).wrapping_mul(4 as u64))
                        / (8 as u64))
                        .wrapping_mul(8 as u64)
                        / (1 as u64))
                        .wrapping_mul(((8 as i32) as i64) as u64)
                    && (unsafe { (*p).iDivisor }) != (0 as u32)
                {
                    std::ptr::write(__slate_slot_305, i / unsafe { (*p).iDivisor });
                    i = i % unsafe { (*p).iDivisor };
                    if (unsafe {
                        *unsafe {
                            unsafe { (*p).u.apSub.as_mut_ptr() as *mut *mut Bitvec }
                                .offset(*__slate_slot_305 as isize)
                        }
                    }) == std::ptr::null_mut::<Bitvec>()
                    {
                        unsafe {
                            *unsafe {
                                unsafe { (*p).u.apSub.as_mut_ptr() as *mut *mut Bitvec }
                                    .offset(*__slate_slot_305 as isize)
                            } = sqlite3BitvecCreate(unsafe { (*p).iDivisor });
                        }
                        if (unsafe {
                            *unsafe {
                                unsafe { (*p).u.apSub.as_mut_ptr() as *mut *mut Bitvec }
                                    .offset(*__slate_slot_305 as isize)
                            }
                        }) == std::ptr::null_mut::<Bitvec>()
                        {
                            return 7 as i32;
                        }
                    }
                    p = unsafe {
                        *unsafe {
                            unsafe { (*p).u.apSub.as_mut_ptr() as *mut *mut Bitvec }
                                .offset(*__slate_slot_305 as isize)
                        }
                    };
                } else {
                    break;
                }
            }
            if ((unsafe { (*p).iSize }) as u64)
                <= (((((512 as i32) as i64) as u64)
                    .wrapping_sub((((3 as i32) as i64) as u64).wrapping_mul(4 as u64))
                    / (8 as u64))
                    .wrapping_mul(8 as u64)
                    / (1 as u64))
                    .wrapping_mul(((8 as i32) as i64) as u64)
            {
                std::ptr::write(__slate_slot_378, unsafe {
                    unsafe { (*p).u.aBitmap.as_mut_ptr() as *mut u8 }
                        .offset((i / ((8 as i32) as u32)) as isize)
                });
                std::ptr::write(__slate_slot_379, unsafe { *(*__slate_slot_378) });
                std::ptr::write(
                    __slate_slot_380,
                    ((((*__slate_slot_379 as u32) as i32)
                        | (1 as i32) << (i & (((8 as i32) - (1 as i32)) as u32)))
                        as i8) as u8,
                );
                unsafe {
                    *(*__slate_slot_378) = *__slate_slot_380;
                }
                return 0 as i32;
            } else {
                '__join_0: {
                    std::ptr::write(__slate_slot_381, i);
                    std::ptr::write(
                        __slate_slot_382,
                        (*__slate_slot_381).wrapping_add((1 as i32) as u32),
                    );
                    i = *__slate_slot_382;
                    *__slate_slot_304 = (((*__slate_slot_381).wrapping_mul((1 as i32) as u32)
                        as u64)
                        % (((((512 as i32) as i64) as u64)
                            .wrapping_sub((((3 as i32) as i64) as u64).wrapping_mul(4 as u64))
                            / (8 as u64))
                            .wrapping_mul(8 as u64)
                            / (4 as u64))) as u32;
                    // if there wasn't a hash collision, and this doesn't
                    //
                    // completely fill the hash, then just add it without
                    //
                    // worrying about sub-dividing and re-hashing.
                    if !((unsafe {
                        *unsafe {
                            unsafe { (*p).u.aHash.as_mut_ptr() as *mut u32 }
                                .offset(*__slate_slot_304 as isize)
                        }
                    }) != (0 as u32))
                    {
                        if ((unsafe { (*p).nSet }) as u64)
                            < (((((512 as i32) as i64) as u64)
                                .wrapping_sub((((3 as i32) as i64) as u64).wrapping_mul(4 as u64))
                                / (8 as u64))
                                .wrapping_mul(8 as u64)
                                / (4 as u64))
                                .wrapping_sub(((1 as i32) as i64) as u64)
                        {
                            break '__join_0;
                        }
                    } else {
                        // there was a collision, check to see if it's already
                        //
                        // in hash, if not, try to find a spot for it
                        loop {
                            if (unsafe {
                                *unsafe {
                                    unsafe { (*p).u.aHash.as_mut_ptr() as *mut u32 }
                                        .offset(*__slate_slot_304 as isize)
                                }
                            }) == i
                            {
                                return 0 as i32;
                            } else {
                                std::ptr::write(__slate_slot_383, *__slate_slot_304);
                                std::ptr::write(
                                    __slate_slot_384,
                                    (*__slate_slot_383).wrapping_add((1 as i32) as u32),
                                );
                                *__slate_slot_304 = *__slate_slot_384;
                                if (*__slate_slot_304 as u64)
                                    >= ((((512 as i32) as i64) as u64).wrapping_sub(
                                        (((3 as i32) as i64) as u64).wrapping_mul(4 as u64),
                                    ) / (8 as u64))
                                        .wrapping_mul(8 as u64)
                                        / (4 as u64)
                                {
                                    *__slate_slot_304 = (0 as i32) as u32;
                                }
                                if !((unsafe {
                                    *unsafe {
                                        unsafe { (*p).u.aHash.as_mut_ptr() as *mut u32 }
                                            .offset(*__slate_slot_304 as isize)
                                    }
                                }) != (0 as u32))
                                {
                                    break;
                                }
                            }
                        }
                        // we didn't find it in the hash.  h points to the first
                        //
                        // available free spot. check to see if this is going to
                        //
                        // make our hash too "full".
                    }
                    if ((unsafe { (*p).nSet }) as u64)
                        >= ((((512 as i32) as i64) as u64)
                            .wrapping_sub((((3 as i32) as i64) as u64).wrapping_mul(4 as u64))
                            / (8 as u64))
                            .wrapping_mul(8 as u64)
                            / (4 as u64)
                            / (((2 as i32) as i64) as u64)
                    {
                        std::ptr::write(
                            __slate_slot_308,
                            (unsafe {
                                sqlite3DbMallocRaw(std::ptr::null_mut::<sqlite3>(), 496 as u64)
                            }) as *mut u32,
                        );
                        if *__slate_slot_308 == std::ptr::null_mut::<u32>() {
                            return 7 as i32;
                        } else {
                            unsafe {
                                memcpy(
                                    *__slate_slot_308 as *mut (),
                                    (unsafe { (*p).u.aHash.as_mut_ptr() as *mut u32 }) as *const (),
                                    496 as u64,
                                )
                            };
                            unsafe {
                                memset(
                                    (unsafe { (*p).u.apSub.as_mut_ptr() as *mut *mut Bitvec })
                                        as *mut (),
                                    0 as i32,
                                    496 as u64,
                                )
                            };
                            unsafe {
                                (*p).iDivisor = (unsafe { (*p).iSize })
                                    / ((((((512 as i32) as i64) as u64).wrapping_sub(
                                        (((3 as i32) as i64) as u64).wrapping_mul(4 as u64),
                                    ) / (8 as u64))
                                        .wrapping_mul(8 as u64)
                                        / (8 as u64))
                                        as u32);
                            }
                            if (unsafe { (*p).iSize })
                                % ((((((512 as i32) as i64) as u64).wrapping_sub(
                                    (((3 as i32) as i64) as u64).wrapping_mul(4 as u64),
                                ) / (8 as u64))
                                    .wrapping_mul(8 as u64)
                                    / (8 as u64)) as u32)
                                != ((0 as i32) as u32)
                            {
                                std::ptr::write(__slate_slot_385, p);
                                std::ptr::write(__slate_slot_386, unsafe {
                                    (*(*__slate_slot_385)).iDivisor
                                });
                                std::ptr::write(
                                    __slate_slot_387,
                                    (*__slate_slot_386).wrapping_add((1 as i32) as u32),
                                );
                                unsafe {
                                    (*(*__slate_slot_385)).iDivisor = *__slate_slot_387;
                                }
                            }
                            if ((unsafe { (*p).iDivisor }) as u64)
                                < (((((512 as i32) as i64) as u64).wrapping_sub(
                                    (((3 as i32) as i64) as u64).wrapping_mul(4 as u64),
                                ) / (8 as u64))
                                    .wrapping_mul(8 as u64)
                                    / (1 as u64))
                                    .wrapping_mul(((8 as i32) as i64) as u64)
                            {
                                unsafe {
                                    (*p).iDivisor = (((((512 as i32) as i64) as u64).wrapping_sub(
                                        (((3 as i32) as i64) as u64).wrapping_mul(4 as u64),
                                    ) / (8 as u64))
                                        .wrapping_mul(8 as u64)
                                        / (1 as u64))
                                        .wrapping_mul(((8 as i32) as i64) as u64)
                                        as u32;
                                }
                            }
                            *__slate_slot_307 = sqlite3BitvecSet(p, i);
                            *__slate_slot_306 = (0 as i32) as u32;
                            loop {
                                if (*__slate_slot_306 as u64)
                                    < ((((512 as i32) as i64) as u64).wrapping_sub(
                                        (((3 as i32) as i64) as u64).wrapping_mul(4 as u64),
                                    ) / (8 as u64))
                                        .wrapping_mul(8 as u64)
                                        / (4 as u64)
                                {
                                    if (unsafe {
                                        *unsafe {
                                            (*__slate_slot_308).offset(*__slate_slot_306 as isize)
                                        }
                                    }) != (0 as u32)
                                    {
                                        std::ptr::write(__slate_slot_390, *__slate_slot_307);
                                        std::ptr::write(
                                            __slate_slot_391,
                                            *__slate_slot_390
                                                | sqlite3BitvecSet(p, unsafe {
                                                    *unsafe {
                                                        (*__slate_slot_308)
                                                            .offset(*__slate_slot_306 as isize)
                                                    }
                                                }),
                                        );
                                        *__slate_slot_307 = *__slate_slot_391;
                                    }
                                    std::ptr::write(__slate_slot_388, *__slate_slot_306);
                                    std::ptr::write(
                                        __slate_slot_389,
                                        (*__slate_slot_388).wrapping_add((1 as i32) as u32),
                                    );
                                    *__slate_slot_306 = *__slate_slot_389;
                                } else {
                                    break;
                                }
                            }
                            unsafe {
                                sqlite3DbFree(
                                    std::ptr::null_mut::<sqlite3>(),
                                    *__slate_slot_308 as *mut (),
                                )
                            };
                            return *__slate_slot_307;
                        }
                    }
                }
                std::ptr::write(__slate_slot_392, p);
                std::ptr::write(__slate_slot_393, unsafe { (*(*__slate_slot_392)).nSet });
                std::ptr::write(
                    __slate_slot_394,
                    (*__slate_slot_393).wrapping_add((1 as i32) as u32),
                );
                unsafe {
                    (*(*__slate_slot_392)).nSet = *__slate_slot_394;
                }
                unsafe {
                    *unsafe {
                        unsafe { (*p).u.aHash.as_mut_ptr() as *mut u32 }
                            .offset(*__slate_slot_304 as isize)
                    } = i;
                }
                return 0 as i32;
            }
        }
    }
    return unsafe { std::mem::zeroed() };
}

/// Clear the i-th bit.
///
/// pBuf must be a pointer to at least BITVEC_SZ bytes of temporary storage
/// that BitvecClear can use to rebuilt its hash table.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3BitvecClear(mut p: *mut Bitvec, mut i: u32, mut pBuf: *mut ()) {
    if p == std::ptr::null_mut::<Bitvec>() {
        return;
    }
    0 as i32;
    let __v395: u32 = i;
    let __v396: u32 = __v395.wrapping_sub((1 as i32) as u32);
    i = __v396;
    '__slate_break_364: while (unsafe { (*p).iDivisor }) != (0 as u32) {
        let mut bin: u32 = i / unsafe { (*p).iDivisor };
        i = i % unsafe { (*p).iDivisor };
        p = unsafe {
            *unsafe {
                unsafe { (*p).u.apSub.as_mut_ptr() as *mut *mut Bitvec }.offset(bin as isize)
            }
        };
        if !(p != std::ptr::null_mut::<Bitvec>()) {
            return;
        }
    }
    if ((unsafe { (*p).iSize }) as u64)
        <= (((((512 as i32) as i64) as u64)
            .wrapping_sub((((3 as i32) as i64) as u64).wrapping_mul(4 as u64))
            / (8 as u64))
            .wrapping_mul(8 as u64)
            / (1 as u64))
            .wrapping_mul(((8 as i32) as i64) as u64)
    {
        let __v397: *mut u8 = unsafe {
            unsafe { (*p).u.aBitmap.as_mut_ptr() as *mut u8 }
                .offset((i / ((8 as i32) as u32)) as isize)
        };
        let __v398: u8 = unsafe { *__v397 };
        let __v399: u8 = ((((__v398 as u32) as i32)
            & !((((((1 as i32) << (i & (((8 as i32) - (1 as i32)) as u32))) as i8) as u8) as u32)
                as i32)) as i8) as u8;
        unsafe {
            *__v397 = __v399;
        }
    } else {
        let mut j: u32 = 0 as u32;
        let mut aiValues: *mut u32 = pBuf as *mut u32;
        unsafe {
            memcpy(
                aiValues as *mut (),
                (unsafe { (*p).u.aHash.as_mut_ptr() as *mut u32 }) as *const (),
                496 as u64,
            )
        };
        unsafe {
            memset(
                (unsafe { (*p).u.aHash.as_mut_ptr() as *mut u32 }) as *mut (),
                0 as i32,
                496 as u64,
            )
        };
        unsafe {
            (*p).nSet = (0 as i32) as u32;
        }
        j = (0 as i32) as u32;
        '__slate_break_365: while (j as u64)
            < ((((512 as i32) as i64) as u64)
                .wrapping_sub((((3 as i32) as i64) as u64).wrapping_mul(4 as u64))
                / (8 as u64))
                .wrapping_mul(8 as u64)
                / (4 as u64)
        {
            if (unsafe { *unsafe { aiValues.offset(j as isize) } }) != (0 as u32)
                && (unsafe { *unsafe { aiValues.offset(j as isize) } })
                    != i.wrapping_add((1 as i32) as u32)
            {
                let mut h: u32 = ((unsafe { *unsafe { aiValues.offset(j as isize) } }
                    .wrapping_sub((1 as i32) as u32)
                    .wrapping_mul((1 as i32) as u32) as u64)
                    % (((((512 as i32) as i64) as u64)
                        .wrapping_sub((((3 as i32) as i64) as u64).wrapping_mul(4 as u64))
                        / (8 as u64))
                        .wrapping_mul(8 as u64)
                        / (4 as u64))) as u32;
                let __v402: *mut Bitvec = p;
                let __v403: u32 = unsafe { (*__v402).nSet };
                let __v404: u32 = __v403.wrapping_add((1 as i32) as u32);
                unsafe {
                    (*__v402).nSet = __v404;
                }
                '__slate_break_366: while (unsafe {
                    *unsafe { unsafe { (*p).u.aHash.as_mut_ptr() as *mut u32 }.offset(h as isize) }
                }) != (0 as u32)
                {
                    let __v405: u32 = h;
                    let __v406: u32 = __v405.wrapping_add((1 as i32) as u32);
                    h = __v406;
                    if (h as u64)
                        >= ((((512 as i32) as i64) as u64)
                            .wrapping_sub((((3 as i32) as i64) as u64).wrapping_mul(4 as u64))
                            / (8 as u64))
                            .wrapping_mul(8 as u64)
                            / (4 as u64)
                    {
                        h = (0 as i32) as u32;
                    }
                }
                unsafe {
                    *unsafe {
                        unsafe { (*p).u.aHash.as_mut_ptr() as *mut u32 }.offset(h as isize)
                    } = unsafe { *unsafe { aiValues.offset(j as isize) } };
                }
            }
            let __v400: u32 = j;
            let __v401: u32 = __v400.wrapping_add((1 as i32) as u32);
            j = __v401;
        }
    }
}

/// Destroy a bitmap object.  Reclaim all memory used.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3BitvecDestroy(mut p: *mut Bitvec) {
    if p == std::ptr::null_mut::<Bitvec>() {
        return;
    }
    if (unsafe { (*p).iDivisor }) != (0 as u32) {
        let mut i: u32 = 0 as u32;
        i = (0 as i32) as u32;
        '__slate_break_367: while i
            < ((((((512 as i32) as i64) as u64)
                .wrapping_sub((((3 as i32) as i64) as u64).wrapping_mul(4 as u64))
                / (8 as u64))
                .wrapping_mul(8 as u64)
                / (8 as u64)) as u32)
        {
            sqlite3BitvecDestroy(unsafe {
                *unsafe {
                    unsafe { (*p).u.apSub.as_mut_ptr() as *mut *mut Bitvec }.offset(i as isize)
                }
            });
            let __v407: u32 = i;
            let __v408: u32 = __v407.wrapping_add((1 as i32) as u32);
            i = __v408;
        }
    }
    unsafe { sqlite3_free(p as *mut ()) };
}

/// Return the value of the iSize parameter specified when Bitvec *p
/// was created.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3BitvecSize(mut p: *mut Bitvec) -> u32 {
    return unsafe { (*p).iSize };
}

// Let V[] be an array of unsigned characters sufficient to hold
// up to N bits.  Let I be an integer between 0 and N.  0<=I<N.
// Then the following macros can be used to set, clear, or test
// individual bits within V.
/// This routine runs an extensive test of the Bitvec code.
///
/// The input is an array of integers that acts as a program
/// to test the Bitvec.  The integers are opcodes followed
/// by 0, 1, or 3 operands, depending on the opcode.  Another
/// opcode follows immediately after the last operand.
///
/// There are opcodes numbered starting with 0.  0 is the
/// "halt" opcode and causes the test to end.
///
///    0          Halt and return the number of errors
///    1 N S X    Set N bits beginning with S and incrementing by X
///    2 N S X    Clear N bits beginning with S and incrementing by X
///    3 N        Set N randomly chosen bits
///    4 N        Clear N randomly chosen bits
///    5 N S X    Set N bits from S increment X in array only, not in bitvec
///    6          Invoice sqlite3ShowBitvec() on the Bitvec object so far
///    7 X        Show compile-time parameters and the hash of X
///
/// The opcodes 1 through 4 perform set and clear operations are performed
/// on both a Bitvec object and on a linear array of bits obtained from malloc.
/// Opcode 5 works on the linear array only, not on the Bitvec.
/// Opcode 5 is used to deliberately induce a fault in order to
/// confirm that error detection works.  Opcodes 6 and greater are
/// state output opcodes.  Opcodes 6 and greater are no-ops unless
/// SQLite has been compiled with SQLITE_DEBUG.
///
/// At the conclusion of the test the linear array is compared
/// against the Bitvec object.  If there are any differences,
/// an error is returned.  If they are the same, zero is returned.
///
/// If a memory allocation error occurs, return -1.
///
/// sz is the size of the Bitvec.  Or if sz is negative, make the size
/// 2*(unsigned)(-sz) and disabled the linear vector check.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3BitvecBuiltinTest(mut sz: i32, mut aOp: *mut i32) -> i32 {
    let mut __slate_storage_427: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_427: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_427) as *mut i32;
    let mut __slate_storage_426: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_426: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_426) as *mut i32;
    let mut __slate_storage_409: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_409: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_409) as *mut i32;
    let mut __slate_storage_422: std::mem::MaybeUninit<u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_422: *mut u8 = std::ptr::addr_of_mut!(__slate_storage_422) as *mut u8;
    let mut __slate_storage_421: std::mem::MaybeUninit<u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_421: *mut u8 = std::ptr::addr_of_mut!(__slate_storage_421) as *mut u8;
    let mut __slate_storage_420: std::mem::MaybeUninit<*mut u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_420: *mut *mut u8 =
        std::ptr::addr_of_mut!(__slate_storage_420) as *mut *mut u8;
    let mut __slate_storage_425: std::mem::MaybeUninit<u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_425: *mut u8 = std::ptr::addr_of_mut!(__slate_storage_425) as *mut u8;
    let mut __slate_storage_424: std::mem::MaybeUninit<u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_424: *mut u8 = std::ptr::addr_of_mut!(__slate_storage_424) as *mut u8;
    let mut __slate_storage_423: std::mem::MaybeUninit<*mut u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_423: *mut *mut u8 =
        std::ptr::addr_of_mut!(__slate_storage_423) as *mut *mut u8;
    let mut __slate_storage_419: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_419: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_419) as *mut i32;
    let mut __slate_storage_418: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_418: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_418) as *mut i32;
    let mut __slate_storage_417: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_417: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_417) as *mut i32;
    let mut __slate_storage_416: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_416: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_416) as *mut i32;
    let mut __slate_storage_415: std::mem::MaybeUninit<*mut i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_415: *mut *mut i32 =
        std::ptr::addr_of_mut!(__slate_storage_415) as *mut *mut i32;
    let mut __slate_storage_414: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_414: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_414) as *mut i32;
    let mut __slate_storage_413: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_413: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_413) as *mut i32;
    let mut __slate_storage_412: std::mem::MaybeUninit<*mut i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_412: *mut *mut i32 =
        std::ptr::addr_of_mut!(__slate_storage_412) as *mut *mut i32;
    let mut __slate_storage_411: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_411: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_411) as *mut i32;
    let mut __slate_storage_410: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_410: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_410) as *mut i32;
    let mut __slate_storage_329: std::mem::MaybeUninit<*mut ()> = std::mem::MaybeUninit::uninit();
    let __slate_slot_329: *mut *mut () =
        std::ptr::addr_of_mut!(__slate_storage_329) as *mut *mut ();
    let mut __slate_storage_328: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_328: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_328) as *mut i32;
    let mut __slate_storage_327: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_327: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_327) as *mut i32;
    let mut __slate_storage_326: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_326: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_326) as *mut i32;
    let mut __slate_storage_325: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_325: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_325) as *mut i32;
    let mut __slate_storage_324: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_324: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_324) as *mut i32;
    let mut __slate_storage_323: std::mem::MaybeUninit<*mut u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_323: *mut *mut u8 =
        std::ptr::addr_of_mut!(__slate_storage_323) as *mut *mut u8;
    let mut __slate_storage_322: std::mem::MaybeUninit<*mut Bitvec> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_322: *mut *mut Bitvec =
        std::ptr::addr_of_mut!(__slate_storage_322) as *mut *mut Bitvec;
    unsafe {
        '__join_26: {
            std::ptr::write(__slate_slot_322, std::ptr::null_mut::<Bitvec>());
            std::ptr::write(__slate_slot_323, std::ptr::null_mut::<u8>());
            std::ptr::write(__slate_slot_324, -(1 as i32));
            // Allocate the Bitvec to be tested and a linear array of
            // bits to act as the reference
            if sz <= (0 as i32) {
                *__slate_slot_322 =
                    sqlite3BitvecCreate(((2 as i32) as u32).wrapping_mul(-sz as u32));
                *__slate_slot_323 = std::ptr::null_mut::<u8>();
            } else {
                *__slate_slot_322 = sqlite3BitvecCreate(sz as u32);
                *__slate_slot_323 = (unsafe {
                    sqlite3MallocZero(
                        ((((7 as i32) as i64) + (sz as i64)) / ((8 as i32) as i64)
                            + ((1 as i32) as i64)) as u64,
                    )
                }) as *mut u8;
            }
        }
        '__join_0: {
            *__slate_slot_329 = unsafe { sqlite3_malloc64(((512 as i32) as i64) as u64) };
            if *__slate_slot_322 == std::ptr::null_mut::<Bitvec>()
                || *__slate_slot_329 == std::ptr::null_mut::<()>()
                || *__slate_slot_323 == std::ptr::null_mut::<u8>() && sz > (0 as i32)
            {
            } else {
                // NULL pBitvec tests
                sqlite3BitvecSet(std::ptr::null_mut::<Bitvec>(), (1 as i32) as u32);
                sqlite3BitvecClear(
                    std::ptr::null_mut::<Bitvec>(),
                    (1 as i32) as u32,
                    *__slate_slot_329,
                );
                // Run the program
                *__slate_slot_325 = 0 as i32;
                *__slate_slot_327 = 0 as i32;
                loop {
                    std::ptr::write(__slate_slot_409, unsafe {
                        *unsafe { aOp.offset(*__slate_slot_327 as isize) }
                    });
                    *__slate_slot_328 = *__slate_slot_409;
                    if *__slate_slot_409 != (0 as i32) {
                        if *__slate_slot_328 >= (6 as i32) {
                            std::ptr::write(__slate_slot_410, *__slate_slot_327);
                            std::ptr::write(__slate_slot_411, *__slate_slot_410 + (1 as i32));
                            *__slate_slot_327 = *__slate_slot_411;
                        } else {
                            '__join_19: {
                                '__join_21: {
                                    let __t0: i32 = *__slate_slot_328;
                                    if __t0 == (1 as i32) {
                                        break '__join_21;
                                    } else {
                                        if __t0 == (2 as i32) {
                                            break '__join_21;
                                        } else {
                                            if __t0 == (5 as i32) {
                                                break '__join_21;
                                            } else {
                                                if __t0 == (3 as i32) {
                                                } else {
                                                    if __t0 == (4 as i32) {}
                                                }
                                            }
                                        }
                                    }
                                    *__slate_slot_326 = 2 as i32;
                                    unsafe {
                                        sqlite3_randomness(
                                            ((4 as u64) as u32) as i32,
                                            std::ptr::addr_of_mut!(*__slate_slot_325) as *mut (),
                                        )
                                    };
                                    break '__join_19;
                                }
                                *__slate_slot_326 = 4 as i32;
                                *__slate_slot_325 = (unsafe {
                                    *unsafe {
                                        aOp.offset((*__slate_slot_327 + (2 as i32)) as isize)
                                    }
                                }) - (1 as i32);
                                std::ptr::write(__slate_slot_412, unsafe {
                                    aOp.offset((*__slate_slot_327 + (2 as i32)) as isize)
                                });
                                std::ptr::write(__slate_slot_413, unsafe { *(*__slate_slot_412) });
                                std::ptr::write(
                                    __slate_slot_414,
                                    *__slate_slot_413
                                        + unsafe {
                                            *unsafe {
                                                aOp.offset(
                                                    (*__slate_slot_327 + (3 as i32)) as isize,
                                                )
                                            }
                                        },
                                );
                                unsafe {
                                    *(*__slate_slot_412) = *__slate_slot_414;
                                }
                            }
                            std::ptr::write(__slate_slot_415, unsafe {
                                aOp.offset((*__slate_slot_327 + (1 as i32)) as isize)
                            });
                            std::ptr::write(__slate_slot_416, unsafe { *(*__slate_slot_415) });
                            std::ptr::write(__slate_slot_417, *__slate_slot_416 - (1 as i32));
                            unsafe {
                                *(*__slate_slot_415) = *__slate_slot_417;
                            }
                            if *__slate_slot_417 > (0 as i32) {
                                *__slate_slot_326 = 0 as i32;
                            }
                            std::ptr::write(__slate_slot_418, *__slate_slot_327);
                            std::ptr::write(
                                __slate_slot_419,
                                *__slate_slot_418 + *__slate_slot_326,
                            );
                            *__slate_slot_327 = *__slate_slot_419;
                            *__slate_slot_325 = (*__slate_slot_325 & (2147483647 as i32)) % sz;
                            if *__slate_slot_328 & (1 as i32) != (0 as i32) {
                                if *__slate_slot_323 != std::ptr::null_mut::<u8>() {
                                    std::ptr::write(__slate_slot_420, unsafe {
                                        (*__slate_slot_323).offset(
                                            (*__slate_slot_325 + (1 as i32) >> (3 as i32)) as isize,
                                        )
                                    });
                                    std::ptr::write(__slate_slot_421, unsafe {
                                        *(*__slate_slot_420)
                                    });
                                    std::ptr::write(
                                        __slate_slot_422,
                                        ((((*__slate_slot_421 as u32) as i32)
                                            | (1 as i32)
                                                << (*__slate_slot_325 + (1 as i32) & (7 as i32)))
                                            as i8) as u8,
                                    );
                                    unsafe {
                                        *(*__slate_slot_420) = *__slate_slot_422;
                                    }
                                }
                                if *__slate_slot_328 != (5 as i32) {
                                    if sqlite3BitvecSet(
                                        *__slate_slot_322,
                                        (*__slate_slot_325 + (1 as i32)) as u32,
                                    ) != (0 as i32)
                                    {
                                        break '__join_0;
                                    }
                                }
                            } else {
                                if *__slate_slot_323 != std::ptr::null_mut::<u8>() {
                                    std::ptr::write(__slate_slot_423, unsafe {
                                        (*__slate_slot_323).offset(
                                            (*__slate_slot_325 + (1 as i32) >> (3 as i32)) as isize,
                                        )
                                    });
                                    std::ptr::write(__slate_slot_424, unsafe {
                                        *(*__slate_slot_423)
                                    });
                                    std::ptr::write(
                                        __slate_slot_425,
                                        ((((*__slate_slot_424 as u32) as i32)
                                            & !((((((1 as i32)
                                                << (*__slate_slot_325 + (1 as i32) & (7 as i32)))
                                                as i8)
                                                as u8)
                                                as u32)
                                                as i32))
                                            as i8) as u8,
                                    );
                                    unsafe {
                                        *(*__slate_slot_423) = *__slate_slot_425;
                                    }
                                }
                                sqlite3BitvecClear(
                                    *__slate_slot_322,
                                    (*__slate_slot_325 + (1 as i32)) as u32,
                                    *__slate_slot_329,
                                );
                            }
                        }
                    } else {
                        break;
                    }
                }
                '__join_1: {
                    // Test to make sure the linear array exactly matches the
                    // Bitvec object.  Start with the assumption that they do
                    // match (rc==0).  Change rc to non-zero if a discrepancy
                    // is found.
                    if *__slate_slot_323 != std::ptr::null_mut::<u8>() {
                        *__slate_slot_324 =
                            ((sqlite3BitvecTest(std::ptr::null_mut::<Bitvec>(), (0 as i32) as u32)
                                + sqlite3BitvecTest(*__slate_slot_322, (sz + (1 as i32)) as u32)
                                + sqlite3BitvecTest(*__slate_slot_322, (0 as i32) as u32))
                                as u32)
                                .wrapping_add(
                                    sqlite3BitvecSize(*__slate_slot_322).wrapping_sub(sz as u32),
                                ) as i32;
                        *__slate_slot_325 = 1 as i32;
                        loop {
                            if *__slate_slot_325 <= sz {
                                if (((((unsafe {
                                    *unsafe {
                                        (*__slate_slot_323)
                                            .offset((*__slate_slot_325 >> (3 as i32)) as isize)
                                    }
                                }) as u32) as i32)
                                    & (1 as i32) << (*__slate_slot_325 & (7 as i32))
                                    != (0 as i32)) as i32)
                                    != sqlite3BitvecTest(
                                        *__slate_slot_322,
                                        *__slate_slot_325 as u32,
                                    )
                                {
                                    break;
                                } else {
                                    std::ptr::write(__slate_slot_426, *__slate_slot_325);
                                    std::ptr::write(
                                        __slate_slot_427,
                                        *__slate_slot_426 + (1 as i32),
                                    );
                                    *__slate_slot_325 = *__slate_slot_427;
                                }
                            } else {
                                break '__join_1;
                            }
                        }
                        *__slate_slot_324 = *__slate_slot_325;
                    } else {
                        *__slate_slot_324 = 0 as i32;
                    }
                }
                // Free allocated structure
            }
        }
        unsafe { sqlite3_free(*__slate_slot_329) };
        unsafe { sqlite3_free(*__slate_slot_323 as *mut ()) };
        sqlite3BitvecDestroy(*__slate_slot_322);
        return *__slate_slot_324;
    }
    return unsafe { std::mem::zeroed() };
}
