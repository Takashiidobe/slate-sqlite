//! 2008 December 3
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
//! This module implements an object we call a "RowSet".
//!
//! The RowSet object is a collection of rowids.  Rowids
//! are inserted into the RowSet in an arbitrary order.  Inserts
//! can be intermixed with tests to see if a given rowid has been
//! previously inserted into the RowSet.
//!
//! After all inserts are finished, it is possible to extract the
//! elements of the RowSet in sorted order.  Once this extraction
//! process has started, no new elements may be inserted.
//!
//! Hence, the primitive operations for a RowSet are:
//!
//!    CREATE
//!    INSERT
//!    TEST
//!    SMALLEST
//!    DESTROY
//!
//! The CREATE and DESTROY primitives are the constructor and destructor,
//! obviously.  The INSERT primitive adds a new element to the RowSet.
//! TEST checks to see if an element is already in the RowSet.  SMALLEST
//! extracts the least value from the RowSet.
//!
//! The INSERT primitive might allocate additional memory.  Memory is
//! allocated in chunks so most INSERTs do no allocation.  There is an
//! upper bound on the size of allocated memory.  No memory is freed
//! until DESTROY.
//!
//! The TEST primitive includes a "batch" number.  The TEST primitive
//! will only see elements that were inserted before the last change
//! in the batch number.  In other words, if an INSERT occurs between
//! two TESTs where the TESTs have the same batch number, then the
//! value added by the INSERT will not be visible to the second TEST.
//! The initial batch number is zero, so if the very first TEST contains
//! a non-zero batch number, it will see all prior INSERTs.
//!
//! No INSERTs may occurs after a SMALLEST.  An assertion will fail if
//! that is attempted.
//!
//! The cost of an INSERT is roughly constant.  (Sometimes new memory
//! has to be allocated on an INSERT.)  The cost of a TEST with a new
//! batch number is O(NlogN) where N is the number of elements in the RowSet.
//! The cost of a TEST using the same batch number is O(logN).  The cost
//! of the first SMALLEST is O(NlogN).  Second and subsequent SMALLEST
//! primitives are constant time.  The cost of DESTROY is O(N).
//!
//! TEST and SMALLEST may not be used by the same RowSet.  This used to
//! be possible, but the feature was not used, so it was removed in order
//! to simplify the code.
unsafe extern "C" {
    fn memset(__s: *mut (), __c: i32, __n: u64) -> *mut ();
    fn sqlite3DbMallocRawNN(__v336: *mut sqlite3, __v337: u64) -> *mut ();
    fn sqlite3DbFree(__v338: *mut sqlite3, __v339: *mut ());
    fn sqlite3DbMallocSize(__v340: *mut sqlite3, __v341: *const ()) -> i32;
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

// Target size for allocation chunks.
// The number of rowset entries per allocation chunk.
/// Each entry in a RowSet is an instance of the following object.
///
/// This same object is reused to store a linked list of trees of RowSetEntry
/// objects.  In that alternative use, pRight points to the next entry
/// in the list, pLeft points to the tree, and v is unused.  The
/// RowSet.pForest value points to the head of this forest list.
#[repr(C)]
#[derive(Clone, Copy)]
struct RowSetEntry {
    /// ROWID value for this entry
    v: i64,
    /// Right subtree (larger entries) or list
    pRight: *mut RowSetEntry,
    /// Left subtree (smaller entries)
    pLeft: *mut RowSetEntry,
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

/// RowSetEntry objects are allocated in large chunks (instances of the
/// following structure) to reduce memory allocation overhead.  The
/// chunks are kept on a linked list so that they can be deallocated
/// when the RowSet is destroyed.
#[repr(C)]
#[derive(Clone, Copy)]
struct RowSetChunk {
    /// Next chunk on list of them all
    pNextChunk: *mut RowSetChunk,
    /// Allocated entries
    aEntry: [RowSetEntry; 42],
}

/// A RowSet in an instance of the following structure.
///
/// A typedef of this structure if found in sqliteInt.h.
#[repr(C)]
#[derive(Clone, Copy)]
struct RowSet {
    /// List of all chunk allocations
    pChunk: *mut RowSetChunk,
    /// The database connection
    db: *mut sqlite3,
    /// List of entries using pRight
    pEntry: *mut RowSetEntry,
    /// Last entry on the pEntry list
    pLast: *mut RowSetEntry,
    /// Source of new entry objects
    pFresh: *mut RowSetEntry,
    /// List of binary trees of entries
    pForest: *mut RowSetEntry,
    /// Number of objects on pFresh
    nFresh: u16,
    /// Various flags
    rsFlags: u16,
    /// Current insert batch
    iBatch: i32,
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

// Allowed values for RowSet.rsFlags
// True if RowSet.pEntry is sorted
// True if sqlite3RowSetNext() has been called
/// Allocate a RowSet object.  Return NULL if a memory allocation
/// error occurs.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3RowSetInit(mut db: *mut sqlite3) -> *mut RowSet {
    let mut p: *mut RowSet = (unsafe { sqlite3DbMallocRawNN(db, 56 as u64) }) as *mut RowSet;
    if p != std::ptr::null_mut::<RowSet>() {
        let mut N: i32 = unsafe { sqlite3DbMallocSize(db, p as *const ()) };
        unsafe {
            (*p).pChunk = std::ptr::null_mut::<RowSetChunk>();
        }
        unsafe {
            (*p).db = db;
        }
        unsafe {
            (*p).pEntry = std::ptr::null_mut::<RowSetEntry>();
        }
        unsafe {
            (*p).pLast = std::ptr::null_mut::<RowSetEntry>();
        }
        unsafe {
            (*p).pForest = std::ptr::null_mut::<RowSetEntry>();
        }
        unsafe {
            (*p).pFresh = (unsafe {
                (p as *mut i8).offset(
                    ((56 as u64).wrapping_add(((7 as i32) as i64) as u64)
                        & ((!(7 as i32) as i64) as u64)) as isize,
                )
            }) as *mut RowSetEntry;
        }
        unsafe {
            (*p).nFresh = (((N as i64) as u64).wrapping_sub(
                (56 as u64).wrapping_add(((7 as i32) as i64) as u64)
                    & ((!(7 as i32) as i64) as u64),
            ) / (24 as u64)) as u16;
        }
        unsafe {
            (*p).rsFlags = ((1 as i32) as i16) as u16;
        }
        unsafe {
            (*p).iBatch = 0 as i32;
        }
    }
    return p;
}

/// Deallocate all chunks from a RowSet.  This frees all memory that
/// the RowSet has allocated over its lifetime.  This routine is
/// the destructor for the RowSet.
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.rowset.sqlite3RowSetClear")]
extern "C-unwind" fn sqlite3RowSetClear(mut pArg: *mut ()) {
    let mut p: *mut RowSet = pArg as *mut RowSet;
    let mut pChunk: *mut RowSetChunk = unsafe { std::mem::zeroed() };
    let mut pNextChunk: *mut RowSetChunk = unsafe { std::mem::zeroed() };
    pChunk = unsafe { (*p).pChunk };
    '__slate_break_352: while pChunk != std::ptr::null_mut::<RowSetChunk>() {
        pNextChunk = unsafe { (*pChunk).pNextChunk };
        unsafe { sqlite3DbFree(unsafe { (*p).db }, pChunk as *mut ()) };
        pChunk = pNextChunk;
    }
    unsafe {
        (*p).pChunk = std::ptr::null_mut::<RowSetChunk>();
    }
    unsafe {
        (*p).nFresh = ((0 as i32) as i16) as u16;
    }
    unsafe {
        (*p).pEntry = std::ptr::null_mut::<RowSetEntry>();
    }
    unsafe {
        (*p).pLast = std::ptr::null_mut::<RowSetEntry>();
    }
    unsafe {
        (*p).pForest = std::ptr::null_mut::<RowSetEntry>();
    }
    unsafe {
        (*p).rsFlags = ((1 as i32) as i16) as u16;
    }
}

/// Deallocate all chunks from a RowSet.  This frees all memory that
/// the RowSet has allocated over its lifetime.  This routine is
/// the destructor for the RowSet.
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.rowset.sqlite3RowSetDelete")]
extern "C-unwind" fn sqlite3RowSetDelete(mut pArg: *mut ()) {
    sqlite3RowSetClear(pArg);
    unsafe { sqlite3DbFree(unsafe { (*(pArg as *mut RowSet)).db }, pArg) };
}

/// Allocate a new RowSetEntry object that is associated with the
/// given RowSet.  Return a pointer to the new and completely uninitialized
/// object.
///
/// In an OOM situation, the RowSet.db->mallocFailed flag is set and this
/// routine returns NULL.
fn rowSetEntryAlloc(mut p: *mut RowSet) -> *mut RowSetEntry {
    0 as i32;
    if (((unsafe { (*p).nFresh }) as u32) as i32) == (0 as i32) {
        // We could allocate a fresh RowSetEntry each time one is needed, but it
        // is more efficient to pull a preallocated entry from the pool
        let mut pNew: *mut RowSetChunk = unsafe { std::mem::zeroed() };
        pNew =
            (unsafe { sqlite3DbMallocRawNN(unsafe { (*p).db }, 1016 as u64) }) as *mut RowSetChunk;
        if pNew == std::ptr::null_mut::<RowSetChunk>() {
            return std::ptr::null_mut::<RowSetEntry>();
        }
        unsafe {
            (*pNew).pNextChunk = unsafe { (*p).pChunk };
        }
        unsafe {
            (*p).pChunk = pNew;
        }
        unsafe {
            (*p).pFresh = unsafe { (*pNew).aEntry.as_mut_ptr() as *mut RowSetEntry };
        }
        unsafe {
            (*p).nFresh = (((((1024 as i32) - (8 as i32)) as i64) as u64) / (24 as u64)) as u16;
        }
    }
    let __v371: *mut RowSet = p;
    let __v372: u16 = unsafe { (*__v371).nFresh };
    let __v373: u16 = ((((__v372 as u32) as i32) - (1 as i32)) as i16) as u16;
    unsafe {
        (*__v371).nFresh = __v373;
    }
    let __v374: *mut RowSet = p;
    let __v375: *mut RowSetEntry = unsafe { (*__v374).pFresh };
    let __v376: *mut RowSetEntry = unsafe { __v375.offset((1 as i32) as isize) };
    unsafe {
        (*__v374).pFresh = __v376;
    }
    return __v375;
}

/// Insert a new value into a RowSet.
///
/// The mallocFailed flag of the database connection is set if a
/// memory allocation fails.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3RowSetInsert(mut p: *mut RowSet, mut rowid: i64) {
    let mut pEntry: *mut RowSetEntry = unsafe { std::mem::zeroed() }; // The new entry
    let mut pLast: *mut RowSetEntry = unsafe { std::mem::zeroed() }; // The last prior entry
    // This routine is never called after sqlite3RowSetNext()
    0 as i32;
    pEntry = rowSetEntryAlloc(p);
    if pEntry == std::ptr::null_mut::<RowSetEntry>() {
        return;
    }
    unsafe {
        (*pEntry).v = rowid;
    }
    unsafe {
        (*pEntry).pRight = std::ptr::null_mut::<RowSetEntry>();
    }
    pLast = unsafe { (*p).pLast };
    if pLast != std::ptr::null_mut::<RowSetEntry>() {
        if rowid <= unsafe { (*pLast).v } {
            // Avoid unnecessary sorts by preserving the ROWSET_SORTED flags
            // where possible
            let __v361: *mut RowSet = p;
            let __v362: u16 = unsafe { (*__v361).rsFlags };
            let __v363: u16 = ((((__v362 as u32) as i32) & !(1 as i32)) as i16) as u16;
            unsafe {
                (*__v361).rsFlags = __v363;
            }
        }
        unsafe {
            (*pLast).pRight = pEntry;
        }
    } else {
        unsafe {
            (*p).pEntry = pEntry;
        }
    }
    unsafe {
        (*p).pLast = pEntry;
    }
}

/// Merge two lists of RowSetEntry objects.  Remove duplicates.
///
/// The input lists are connected via pRight pointers and are
/// assumed to each already be in sorted order.
///
/// # Arguments
///
/// * `pA` - First sorted list to be merged
/// * `pB` - Second sorted list to be merged
fn rowSetEntryMerge(mut pA: *mut RowSetEntry, mut pB: *mut RowSetEntry) -> *mut RowSetEntry {
    let mut head: RowSetEntry = unsafe { std::mem::zeroed() };
    let mut pTail: *mut RowSetEntry = unsafe { std::mem::zeroed() };
    pTail = std::ptr::addr_of_mut!(head);
    0 as i32;
    '__slate_break_353: loop {
        0 as i32;
        0 as i32;
        if (unsafe { (*pA).v }) <= unsafe { (*pB).v } {
            if (unsafe { (*pA).v }) < unsafe { (*pB).v } {
                let __v377: *mut RowSetEntry = pA;
                unsafe {
                    (*pTail).pRight = __v377;
                }
                pTail = __v377;
            }
            pA = unsafe { (*pA).pRight };
            if pA == std::ptr::null_mut::<RowSetEntry>() {
                unsafe {
                    (*pTail).pRight = pB;
                }
                break '__slate_break_353;
            }
        } else {
            let __v378: *mut RowSetEntry = pB;
            unsafe {
                (*pTail).pRight = __v378;
            }
            pTail = __v378;
            pB = unsafe { (*pB).pRight };
            if pB == std::ptr::null_mut::<RowSetEntry>() {
                unsafe {
                    (*pTail).pRight = pA;
                }
                break '__slate_break_353;
            }
        }
    }
    return head.pRight;
}

/// Sort all elements on the list of RowSetEntry objects into order of
/// increasing v.
fn rowSetEntrySort(mut pIn: *mut RowSetEntry) -> *mut RowSetEntry {
    let mut i: u32 = 0 as u32;
    let mut pNext: *mut RowSetEntry = unsafe { std::mem::zeroed() };
    let mut aBucket: __SlateAlign16<[*mut RowSetEntry; 40]> =
        __SlateAlign16([0 as *mut RowSetEntry; 40]);
    unsafe {
        memset(
            (aBucket.0.as_mut_ptr() as *mut *mut RowSetEntry) as *mut (),
            0 as i32,
            320 as u64,
        )
    };
    '__slate_break_354: while pIn != std::ptr::null_mut::<RowSetEntry>() {
        pNext = unsafe { (*pIn).pRight };
        unsafe {
            (*pIn).pRight = std::ptr::null_mut::<RowSetEntry>();
        }
        i = (0 as i32) as u32;
        '__slate_break_355: while (unsafe {
            *unsafe { (aBucket.0.as_mut_ptr() as *mut *mut RowSetEntry).offset(i as isize) }
        }) != std::ptr::null_mut::<RowSetEntry>()
        {
            pIn = rowSetEntryMerge(
                unsafe {
                    *unsafe { (aBucket.0.as_mut_ptr() as *mut *mut RowSetEntry).offset(i as isize) }
                },
                pIn,
            );
            unsafe {
                *unsafe { (aBucket.0.as_mut_ptr() as *mut *mut RowSetEntry).offset(i as isize) } =
                    std::ptr::null_mut::<RowSetEntry>();
            }
            let __v379: u32 = i;
            let __v380: u32 = __v379.wrapping_add((1 as i32) as u32);
            i = __v380;
        }
        unsafe {
            *unsafe { (aBucket.0.as_mut_ptr() as *mut *mut RowSetEntry).offset(i as isize) } = pIn;
        }
        pIn = pNext;
    }
    pIn = unsafe {
        *unsafe { (aBucket.0.as_mut_ptr() as *mut *mut RowSetEntry).offset((0 as i32) as isize) }
    };
    i = (1 as i32) as u32;
    '__slate_break_356: while (i as u64) < (320 as u64) / (8 as u64) {
        if (unsafe {
            *unsafe { (aBucket.0.as_mut_ptr() as *mut *mut RowSetEntry).offset(i as isize) }
        }) == std::ptr::null_mut::<RowSetEntry>()
        {
        } else {
            let __v383: *mut RowSetEntry;
            if pIn != std::ptr::null_mut::<RowSetEntry>() {
                __v383 = rowSetEntryMerge(pIn, unsafe {
                    *unsafe { (aBucket.0.as_mut_ptr() as *mut *mut RowSetEntry).offset(i as isize) }
                });
            } else {
                __v383 = unsafe {
                    *unsafe { (aBucket.0.as_mut_ptr() as *mut *mut RowSetEntry).offset(i as isize) }
                };
            }
            pIn = __v383;
        }
        let __v381: u32 = i;
        let __v382: u32 = __v381.wrapping_add((1 as i32) as u32);
        i = __v382;
    }
    return pIn;
}

/// The input, pIn, is a binary tree (or subtree) of RowSetEntry objects.
/// Convert this tree into a linked list connected by the pRight pointers
/// and return pointers to the first and last elements of the new list.
///
/// # Arguments
///
/// * `pIn` - Root of the input tree
/// * `ppFirst` - Write head of the output list here
/// * `ppLast` - Write tail of the output list here
fn rowSetTreeToList(
    mut pIn: *mut RowSetEntry,
    mut ppFirst: *mut *mut RowSetEntry,
    mut ppLast: *mut *mut RowSetEntry,
) {
    0 as i32;
    if (unsafe { (*pIn).pLeft }) != std::ptr::null_mut::<RowSetEntry>() {
        let mut p: *mut RowSetEntry = unsafe { std::mem::zeroed() };
        rowSetTreeToList(unsafe { (*pIn).pLeft }, ppFirst, std::ptr::addr_of_mut!(p));
        unsafe {
            (*p).pRight = pIn;
        }
    } else {
        unsafe {
            *ppFirst = pIn;
        }
    }
    if (unsafe { (*pIn).pRight }) != std::ptr::null_mut::<RowSetEntry>() {
        rowSetTreeToList(
            unsafe { (*pIn).pRight },
            unsafe { std::ptr::addr_of_mut!((*pIn).pRight) },
            ppLast,
        );
    } else {
        unsafe {
            *ppLast = pIn;
        }
    }
    0 as i32;
}

/// Convert a sorted list of elements (connected by pRight) into a binary
/// tree with depth of iDepth.  A depth of 1 means the tree contains a single
/// node taken from the head of *ppList.  A depth of 2 means a tree with
/// three nodes.  And so forth.
///
/// Use as many entries from the input list as required and update the
/// *ppList to point to the unused elements of the list.  If the input
/// list contains too few elements, then construct an incomplete tree
/// and leave *ppList set to NULL.
///
/// Return a pointer to the root of the constructed binary tree.
fn rowSetNDeepTree(mut ppList: *mut *mut RowSetEntry, mut iDepth: i32) -> *mut RowSetEntry {
    let mut p: *mut RowSetEntry = unsafe { std::mem::zeroed() }; // Root of the new tree
    let mut pLeft: *mut RowSetEntry = unsafe { std::mem::zeroed() }; // Left subtree
    if (unsafe { *ppList }) == std::ptr::null_mut::<RowSetEntry>() {
        // Prevent unnecessary deep recursion when we run out of entries
        return std::ptr::null_mut::<RowSetEntry>();
    }
    if iDepth > (1 as i32) {
        // This branch causes a *balanced* tree to be generated.  A valid tree
        // is still generated without this branch, but the tree is wildly
        // unbalanced and inefficient.
        pLeft = rowSetNDeepTree(ppList, iDepth - (1 as i32));
        p = unsafe { *ppList };
        if p == std::ptr::null_mut::<RowSetEntry>() {
            // It is safe to always return here, but the resulting tree
            // would be unbalanced
            return pLeft;
        }
        unsafe {
            (*p).pLeft = pLeft;
        }
        unsafe {
            *ppList = unsafe { (*p).pRight };
        }
        unsafe {
            (*p).pRight = rowSetNDeepTree(ppList, iDepth - (1 as i32));
        }
    } else {
        p = unsafe { *ppList };
        unsafe {
            *ppList = unsafe { (*p).pRight };
        }
        unsafe {
            (*p).pRight = std::ptr::null_mut::<RowSetEntry>();
        }
        unsafe {
            (*p).pLeft = std::ptr::null_mut::<RowSetEntry>();
        }
    }
    return p;
}

/// Convert a sorted list of elements into a binary tree. Make the tree
/// as deep as it needs to be in order to contain the entire list.
fn rowSetListToTree(mut pList: *mut RowSetEntry) -> *mut RowSetEntry {
    let mut iDepth: i32 = 0 as i32; // Depth of the tree so far
    let mut p: *mut RowSetEntry = unsafe { std::mem::zeroed() }; // Current tree root
    let mut pLeft: *mut RowSetEntry = unsafe { std::mem::zeroed() }; // Left subtree
    0 as i32;
    p = pList;
    pList = unsafe { (*p).pRight };
    unsafe {
        (*p).pRight = std::ptr::null_mut::<RowSetEntry>();
    }
    unsafe {
        (*p).pLeft = std::ptr::null_mut::<RowSetEntry>();
    }
    iDepth = 1 as i32;
    '__slate_break_357: loop {
        if !(pList != std::ptr::null_mut::<RowSetEntry>()) {
            break;
        }
        pLeft = p;
        p = pList;
        pList = unsafe { (*p).pRight };
        unsafe {
            (*p).pLeft = pLeft;
        }
        unsafe {
            (*p).pRight = rowSetNDeepTree(std::ptr::addr_of_mut!(pList), iDepth);
        }
        let __v384: i32 = iDepth;
        let __v385: i32 = __v384 + (1 as i32);
        iDepth = __v385;
    }
    return p;
}

/// Extract the smallest element from the RowSet.
/// Write the element into *pRowid.  Return 1 on success.  Return
/// 0 if the RowSet is already empty.
///
/// After this routine has been called, the sqlite3RowSetInsert()
/// routine may not be called again.
///
/// This routine may not be called after sqlite3RowSetTest() has
/// been used.  Older versions of RowSet allowed that, but as the
/// capability was not used by the code generator, it was removed
/// for code economy.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3RowSetNext(mut p: *mut RowSet, mut pRowid: *mut i64) -> i32 {
    0 as i32;
    0 as i32; // Cannot be used with sqlite3RowSetText()
    // Merge the forest into a single sorted list on first call
    if (((unsafe { (*p).rsFlags }) as u32) as i32) & (2 as i32) == (0 as i32) {
        if (((unsafe { (*p).rsFlags }) as u32) as i32) & (1 as i32) == (0 as i32) {
            unsafe {
                (*p).pEntry = rowSetEntrySort(unsafe { (*p).pEntry });
            }
        }
        let __v368: *mut RowSet = p;
        let __v369: u16 = unsafe { (*__v368).rsFlags };
        let __v370: u16 = ((((__v369 as u32) as i32) | ((1 as i32) | (2 as i32))) as i16) as u16;
        unsafe {
            (*__v368).rsFlags = __v370;
        }
    }
    // Return the next entry on the list
    if (unsafe { (*p).pEntry }) != std::ptr::null_mut::<RowSetEntry>() {
        unsafe {
            *pRowid = unsafe { (*unsafe { (*p).pEntry }).v };
        }
        unsafe {
            (*p).pEntry = unsafe { (*unsafe { (*p).pEntry }).pRight };
        }
        if (unsafe { (*p).pEntry }) == std::ptr::null_mut::<RowSetEntry>() {
            // Free memory immediately, rather than waiting on sqlite3_finalize()
            sqlite3RowSetClear(p as *mut ());
        }
        return 1 as i32;
    } else {
        return 0 as i32;
    }
    return unsafe { std::mem::zeroed() };
}

/// Check to see if element iRowid was inserted into the rowset as
/// part of any insert batch prior to iBatch.  Return 1 or 0.
///
/// If this is the first test of a new batch and if there exist entries
/// on pRowSet->pEntry, then sort those entries into the forest at
/// pRowSet->pForest so that they can be tested.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3RowSetTest(
    mut pRowSet: *mut RowSet,
    mut iBatch: i32,
    mut iRowid: i64,
) -> i32 {
    let mut p: *mut RowSetEntry = unsafe { std::mem::zeroed() };
    let mut pTree: *mut RowSetEntry = unsafe { std::mem::zeroed() };
    // This routine is never called after sqlite3RowSetNext()
    0 as i32;
    // Sort entries into the forest on the first test of a new batch.
    // To save unnecessary work, only do this when the batch number changes.
    if iBatch != unsafe { (*pRowSet).iBatch } {
        p = unsafe { (*pRowSet).pEntry };
        if p != std::ptr::null_mut::<RowSetEntry>() {
            let mut ppPrevTree: *mut *mut RowSetEntry =
                unsafe { std::ptr::addr_of_mut!((*pRowSet).pForest) };
            if (((unsafe { (*pRowSet).rsFlags }) as u32) as i32) & (1 as i32) == (0 as i32) {
                // Only sort the current set of entries if they need it
                p = rowSetEntrySort(p);
            }
            pTree = unsafe { (*pRowSet).pForest };
            '__slate_break_358: while pTree != std::ptr::null_mut::<RowSetEntry>() {
                ppPrevTree = unsafe { std::ptr::addr_of_mut!((*pTree).pRight) };
                if (unsafe { (*pTree).pLeft }) == std::ptr::null_mut::<RowSetEntry>() {
                    unsafe {
                        (*pTree).pLeft = rowSetListToTree(p);
                    }
                    break '__slate_break_358;
                } else {
                    let mut pAux: *mut RowSetEntry = unsafe { std::mem::zeroed() };
                    let mut pTail: *mut RowSetEntry = unsafe { std::mem::zeroed() };
                    rowSetTreeToList(
                        unsafe { (*pTree).pLeft },
                        std::ptr::addr_of_mut!(pAux),
                        std::ptr::addr_of_mut!(pTail),
                    );
                    unsafe {
                        (*pTree).pLeft = std::ptr::null_mut::<RowSetEntry>();
                    }
                    p = rowSetEntryMerge(pAux, p);
                }
                pTree = unsafe { (*pTree).pRight };
            }
            if pTree == std::ptr::null_mut::<RowSetEntry>() {
                let __v364: *mut RowSetEntry = rowSetEntryAlloc(pRowSet);
                pTree = __v364;
                unsafe {
                    *ppPrevTree = __v364;
                }
                if pTree != std::ptr::null_mut::<RowSetEntry>() {
                    unsafe {
                        (*pTree).v = (0 as i32) as i64;
                    }
                    unsafe {
                        (*pTree).pRight = std::ptr::null_mut::<RowSetEntry>();
                    }
                    unsafe {
                        (*pTree).pLeft = rowSetListToTree(p);
                    }
                }
            }
            unsafe {
                (*pRowSet).pEntry = std::ptr::null_mut::<RowSetEntry>();
            }
            unsafe {
                (*pRowSet).pLast = std::ptr::null_mut::<RowSetEntry>();
            }
            let __v365: *mut RowSet = pRowSet;
            let __v366: u16 = unsafe { (*__v365).rsFlags };
            let __v367: u16 = ((((__v366 as u32) as i32) | (1 as i32)) as i16) as u16;
            unsafe {
                (*__v365).rsFlags = __v367;
            }
        }
        unsafe {
            (*pRowSet).iBatch = iBatch;
        }
    }
    // Test to see if the iRowid value appears anywhere in the forest.
    // Return 1 if it does and 0 if not.
    pTree = unsafe { (*pRowSet).pForest };
    '__slate_break_359: while pTree != std::ptr::null_mut::<RowSetEntry>() {
        p = unsafe { (*pTree).pLeft };
        '__slate_break_360: while p != std::ptr::null_mut::<RowSetEntry>() {
            if (unsafe { (*p).v }) < iRowid {
                p = unsafe { (*p).pRight };
            } else {
                if (unsafe { (*p).v }) > iRowid {
                    p = unsafe { (*p).pLeft };
                } else {
                    return 1 as i32;
                }
            }
        }
        pTree = unsafe { (*pTree).pRight };
    }
    return 0 as i32;
}
