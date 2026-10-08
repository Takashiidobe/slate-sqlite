//! 2017-10-11
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
//! This file contains an implementation of the "sqlite_dbpage" virtual table.
//!
//! The sqlite_dbpage virtual table is used to read or write whole raw
//! pages of the database file.  The pager interface is used so that
//! uncommitted changes and changes recorded in the WAL file are correctly
//! retrieved.
//!
//! Usage example:
//!
//!    SELECT data FROM sqlite_dbpage('aux1') WHERE pgno=123;
//!
//! This is an eponymous virtual table so it does not need to be created before
//! use.  The optional argument to the sqlite_dbpage() table name is the
//! schema for the database file that is to be read.  The default schema is
//! "main".
//!
//! The data field of sqlite_dbpage table can be updated.  The new
//! value must be a BLOB which is the correct page size, otherwise the
//! update fails.  INSERT operations also work, and operate as if they
//! where REPLACE.  The size of the database can be extended by INSERT-ing
//! new pages on the end.
//!
//! Rows may not be deleted.  However, doing an INSERT to page number N
//! with NULL page data causes the N-th page and all subsequent pages to be
//! deleted and the database to be truncated.
unsafe extern "C" {
    static mut sqlite3PendingByte: i32;
    fn sqlite3_mprintf(__v426: *const i8, ...) -> *mut i8;
    fn sqlite3_malloc64(__v427: u64) -> *mut ();
    fn sqlite3_free(__v428: *mut ());
    fn sqlite3_value_blob(__v429: *mut sqlite3_value) -> *const ();
    fn sqlite3_value_int64(__v430: *mut sqlite3_value) -> i64;
    fn sqlite3_value_text(__v431: *mut sqlite3_value) -> *const u8;
    fn sqlite3_value_bytes(__v432: *mut sqlite3_value) -> i32;
    fn sqlite3_value_type(__v433: *mut sqlite3_value) -> i32;
    fn sqlite3_context_db_handle(__v434: *mut sqlite3_context) -> *mut sqlite3;
    fn sqlite3_result_blob(
        __v435: *mut sqlite3_context,
        __v436: *const (),
        __v437: i32,
        __v438: Option<unsafe extern "C-unwind" fn(*mut ())>,
    );
    fn sqlite3_result_int64(__v439: *mut sqlite3_context, __v440: i64);
    fn sqlite3_result_text(
        __v441: *mut sqlite3_context,
        __v442: *const i8,
        __v443: i32,
        __v444: Option<unsafe extern "C-unwind" fn(*mut ())>,
    );
    fn sqlite3_result_zeroblob(__v445: *mut sqlite3_context, n: i32);
    fn sqlite3_create_module(
        db: *mut sqlite3,
        zName: *const i8,
        p: *const sqlite3_module,
        pClientData: *mut (),
    ) -> i32;
    fn sqlite3_declare_vtab(__v451: *mut sqlite3, zSQL: *const i8) -> i32;
    fn sqlite3_vtab_config(__v453: *mut sqlite3, op: i32, ...) -> i32;
    fn memcpy(__dest: *mut (), __src: *const (), __n: u64) -> *mut ();
    fn memset(__s: *mut (), __c: i32, __n: u64) -> *mut ();
    fn sqlite3PagerGet(pPager: *mut Pager, pgno: u32, ppPage: *mut *mut PgHdr, clrFlag: i32)
    -> i32;
    fn sqlite3PagerUnref(__v465: *mut PgHdr);
    fn sqlite3PagerUnrefPageOne(__v466: *mut PgHdr);
    fn sqlite3PagerWrite(__v467: *mut PgHdr) -> i32;
    fn sqlite3PagerGetData(__v468: *mut PgHdr) -> *mut ();
    fn sqlite3PagerTruncateImage(__v469: *mut Pager, __v470: u32);
    fn sqlite3BtreeGetPageSize(__v471: *mut Btree) -> i32;
    fn sqlite3BtreeLastPage(__v472: *mut Btree) -> u32;
    fn sqlite3BtreeBeginTrans(__v473: *mut Btree, __v474: i32, __v475: *mut i32) -> i32;
    fn sqlite3BtreePager(__v476: *mut Btree) -> *mut Pager;
    fn sqlite3BtreeEnter(__v477: *mut Btree);
    fn sqlite3BtreeLeave(__v478: *mut Btree);
    fn sqlite3FindDbName(__v479: *mut sqlite3, __v480: *const i8) -> i32;
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
struct sqlite3_pcache_page {
    pBuf: *mut (),
    pExtra: *mut (),
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
    trace: __SlateRecord164,
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
    u1: __SlateRecord165,
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
    u: __SlateRecord166,
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
    __slate_bits_0: __slate_bits::__SlateBits64U0,
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
    u: __SlateRecord167,
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
    __slate_bits_0: __slate_bits::__SlateBits88U0,
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
    u: __SlateRecord175,
    pLeft: *mut Expr,
    pRight: *mut Expr,
    x: __SlateRecord176,
    nHeight: i32,
    iTable: i32,
    iColumn: i16,
    iAgg: i16,
    w: __SlateRecord177,
    pAggInfo: *mut AggInfo,
    y: __SlateRecord178,
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
struct RenameToken {}

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
    fg: __SlateRecord185,
    iCursor: i32,
    colUsed: u64,
    u1: __SlateRecord186,
    u2: __SlateRecord187,
    u3: __SlateRecord188,
    u4: __SlateRecord189,
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
struct TableLock {}

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
    u1: __SlateRecord191,
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
struct VtabCtx {}

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
struct Pager {}

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
struct PCache {}

#[repr(C)]
#[derive(Clone, Copy)]
struct sqlite3InitInfo {
    newTnum: u32,
    iDb: u8,
    busy: u8,
    __slate_bits_0: __slate_bits::__SlateBits163U0,
    azInit: *mut *const i8,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord164 {
    xLegacy: Option<unsafe extern "C-unwind" fn(*mut (), *const i8)>,
    xV2: Option<unsafe extern "C-unwind" fn(u32, *mut (), *mut (), *mut ()) -> i32>,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord165 {
    isInterrupted: i32,
    notUsed1: f64,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord166 {
    pHash: *mut FuncDef,
    pDestructor: *mut FuncDestructor,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord167 {
    tab: __SlateRecord168,
    view: __SlateRecord169,
    vtab: __SlateRecord170,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord168 {
    addColOffset: i32,
    pFKey: *mut FKey,
    pDfltList: *mut ExprList,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord169 {
    pSelect: *mut Select,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord170 {
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
union __SlateRecord175 {
    zToken: *mut i8,
    iValue: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord176 {
    pList: *mut ExprList,
    pSelect: *mut Select,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord177 {
    iJoin: i32,
    iOfst: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord178 {
    pTab: *mut Table,
    pWin: *mut Window,
    nReg: i32,
    sub: __SlateRecord179,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord179 {
    iAddr: i32,
    regReturn: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct ExprList_item {
    pExpr: *mut Expr,
    zEName: *mut i8,
    fg: __SlateRecord181,
    u: __SlateRecord182,
}

#[repr(C, align(4))]
#[derive(Clone, Copy)]
struct __SlateRecord181 {
    sortFlags: u8,
    __slate_bits_0: __slate_bits::__SlateBits181U0,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord182 {
    x: __SlateRecord183,
    iConstExprReg: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord183 {
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
struct __SlateRecord185 {
    jointype: u8,
    __slate_bits_0: __slate_bits::__SlateBits185U0,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord186 {
    zIndexedBy: *mut i8,
    pFuncArg: *mut ExprList,
    nRow: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord187 {
    pIBIndex: *mut Index,
    pCteUse: *mut CteUse,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord188 {
    pOn: *mut Expr,
    pUsing: *mut IdList,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord189 {
    pSchema: *mut Schema,
    zDatabase: *mut i8,
    pSubq: *mut Subquery,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord191 {
    cr: __SlateRecord192,
    d: __SlateRecord193,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord192 {
    addrCrTab: i32,
    regRowid: i32,
    regRoot: i32,
    constraintName: Token,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord193 {
    pReturning: *mut Returning,
}

// Requires access to internal data structures
#[repr(C)]
#[derive(Clone, Copy)]
struct DbpageCursor {
    /// Base class.  Must be first
    base: sqlite3_vtab_cursor,
    /// Current page number
    pgno: u32,
    /// Last page to visit on this scan
    mxPgno: u32,
    /// Pager being read/written
    pPager: *mut Pager,
    /// Page 1 of the database
    pPage1: *mut PgHdr,
    /// Index of database to analyze
    iDb: i32,
    /// Size of each page in bytes
    szPage: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct DbpageTable {
    /// Base class.  Must be first
    base: sqlite3_vtab,
    /// The database
    db: *mut sqlite3,
    /// Database to truncate
    iDbTrunc: i32,
    /// Size to truncate to
    pgnoTrunc: u32,
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
    pub struct __SlateBits64U0 {
        #[bits(4)]
        pub notNull: u32,
        #[bits(4)]
        pub eCType: u32,
    }
    #[bitfields::bitfield([u8; 2], c_names = true, new = false, from_into_bits = false, from_traits = false, default = false, debug = false, builder = false, bit_ops = false)]
    pub struct __SlateBits181U0 {
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
    pub struct __SlateBits185U0 {
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
    pub struct __SlateBits88U0 {
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
    pub struct __SlateBits163U0 {
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

// Columns
/// Connect to or create a dbpagevfs virtual table.
#[unsafe(link_section = ".text.slate_distinct.dbpage.dbpageConnect")]
extern "C-unwind" fn dbpageConnect(
    mut db: *mut sqlite3,
    mut pAux: *mut (),
    mut argc: i32,
    mut argv: *const *const i8,
    mut ppVtab: *mut *mut sqlite3_vtab,
    mut pzErr: *mut *mut i8,
) -> i32 {
    let mut pTab: *mut DbpageTable = std::ptr::null_mut::<DbpageTable>();
    let mut rc: i32 = 0 as i32;
    pAux;
    argc;
    argv;
    pzErr;
    unsafe { sqlite3_vtab_config(db, 3 as i32) };
    unsafe { sqlite3_vtab_config(db, 4 as i32) };
    rc = unsafe {
        sqlite3_declare_vtab(
            db,
            (b"CREATE TABLE x(pgno INTEGER PRIMARY KEY, data BLOB, schema HIDDEN)\0".as_ptr()
                as *mut i8) as *const i8,
        )
    };
    if rc == (0 as i32) {
        pTab = (unsafe { sqlite3_malloc64(40 as u64) }) as *mut DbpageTable;
        if pTab == std::ptr::null_mut::<DbpageTable>() {
            rc = 7 as i32;
        }
    }
    0 as i32;
    if rc == (0 as i32) {
        unsafe { memset(pTab as *mut (), 0 as i32, 40 as u64) };
        unsafe {
            (*pTab).db = db;
        }
    }
    unsafe {
        *ppVtab = pTab as *mut sqlite3_vtab;
    }
    return rc;
}

/// Disconnect from or destroy a dbpagevfs virtual table.
#[unsafe(link_section = ".text.slate_distinct.dbpage.dbpageDisconnect")]
extern "C-unwind" fn dbpageDisconnect(mut pVtab: *mut sqlite3_vtab) -> i32 {
    unsafe { sqlite3_free(pVtab as *mut ()) };
    return 0 as i32;
}

/// idxNum:
///
///     0     schema=main, full table scan
///     1     schema=main, pgno=?1
///     2     schema=?1, full table scan
///     3     schema=?1, pgno=?2
#[unsafe(link_section = ".text.slate_distinct.dbpage.dbpageBestIndex")]
extern "C-unwind" fn dbpageBestIndex(
    mut tab: *mut sqlite3_vtab,
    mut pIdxInfo: *mut sqlite3_index_info,
) -> i32 {
    let mut i: i32 = 0 as i32;
    let mut iPlan: i32 = 0 as i32;
    tab;
    // If there is a schema= constraint, it must be honored.  Report a
    // ridiculously large estimated cost if the schema= constraint is
    // unavailable
    i = 0 as i32;
    '__slate_break_483: loop {
        if !(i < unsafe { (*pIdxInfo).nConstraint }) {
            break;
        }
        let mut p: *mut sqlite3_index_constraint =
            unsafe { unsafe { (*pIdxInfo).aConstraint }.offset(i as isize) };
        if (unsafe { (*p).iColumn }) != (2 as i32) {
        } else {
            if (((unsafe { (*p).op }) as u32) as i32) != (2 as i32) {
            } else {
                if !((unsafe { (*p).usable }) != (0 as u8)) {
                    // No solution.
                    return 19 as i32;
                }
                iPlan = 2 as i32;
                unsafe {
                    (*unsafe { unsafe { (*pIdxInfo).aConstraintUsage }.offset(i as isize) })
                        .argvIndex = 1 as i32;
                }
                unsafe {
                    (*unsafe { unsafe { (*pIdxInfo).aConstraintUsage }.offset(i as isize) }).omit =
                        ((1 as i32) as i8) as u8;
                }
                break '__slate_break_483;
            }
        }
        let __v496: i32 = i;
        let __v497: i32 = __v496 + (1 as i32);
        i = __v497;
    }
    // If we reach this point, it means that either there is no schema=
    // constraint (in which case we use the "main" schema) or else the
    // schema constraint was accepted.  Lower the estimated cost accordingly
    unsafe {
        (*pIdxInfo).estimatedCost = 1000000.0f64;
    }
    // Check for constraints against pgno
    i = 0 as i32;
    '__slate_break_484: loop {
        if !(i < unsafe { (*pIdxInfo).nConstraint }) {
            break;
        }
        let mut p: *mut sqlite3_index_constraint =
            unsafe { unsafe { (*pIdxInfo).aConstraint }.offset(i as isize) };
        if (unsafe { (*p).usable }) != (0 as u8)
            && (unsafe { (*p).iColumn }) <= (0 as i32)
            && (((unsafe { (*p).op }) as u32) as i32) == (2 as i32)
        {
            unsafe {
                (*pIdxInfo).estimatedRows = (1 as i32) as i64;
            }
            unsafe {
                (*pIdxInfo).idxFlags = 1 as i32;
            }
            unsafe {
                (*pIdxInfo).estimatedCost = 1.0f64;
            }
            unsafe {
                (*unsafe { unsafe { (*pIdxInfo).aConstraintUsage }.offset(i as isize) })
                    .argvIndex = if iPlan != (0 as i32) {
                    2 as i32
                } else {
                    1 as i32
                };
            }
            unsafe {
                (*unsafe { unsafe { (*pIdxInfo).aConstraintUsage }.offset(i as isize) }).omit =
                    ((1 as i32) as i8) as u8;
            }
            let __v500: i32 = iPlan;
            let __v501: i32 = __v500 | (1 as i32);
            iPlan = __v501;
            break '__slate_break_484;
        }
        let __v498: i32 = i;
        let __v499: i32 = __v498 + (1 as i32);
        i = __v499;
    }
    unsafe {
        (*pIdxInfo).idxNum = iPlan;
    }
    if (unsafe { (*pIdxInfo).nOrderBy }) >= (1 as i32)
        && (unsafe {
            (*unsafe { unsafe { (*pIdxInfo).aOrderBy }.offset((0 as i32) as isize) }).iColumn
        }) <= (0 as i32)
        && (((unsafe {
            (*unsafe { unsafe { (*pIdxInfo).aOrderBy }.offset((0 as i32) as isize) }).desc
        }) as u32) as i32)
            == (0 as i32)
    {
        unsafe {
            (*pIdxInfo).orderByConsumed = 1 as i32;
        }
    }
    return 0 as i32;
}

/// Open a new dbpagevfs cursor.
#[unsafe(link_section = ".text.slate_distinct.dbpage.dbpageOpen")]
extern "C-unwind" fn dbpageOpen(
    mut pVTab: *mut sqlite3_vtab,
    mut ppCursor: *mut *mut sqlite3_vtab_cursor,
) -> i32 {
    let mut pCsr: *mut DbpageCursor = unsafe { std::mem::zeroed() };
    pCsr = (unsafe { sqlite3_malloc64(40 as u64) }) as *mut DbpageCursor;
    if pCsr == std::ptr::null_mut::<DbpageCursor>() {
        return 7 as i32;
    } else {
        unsafe { memset(pCsr as *mut (), 0 as i32, 40 as u64) };
        unsafe {
            (*pCsr).base.pVtab = pVTab;
        }
        unsafe {
            (*pCsr).pgno = (0 as i32) as u32;
        }
    }
    unsafe {
        *ppCursor = pCsr as *mut sqlite3_vtab_cursor;
    }
    return 0 as i32;
}

/// Close a dbpagevfs cursor.
#[unsafe(link_section = ".text.slate_distinct.dbpage.dbpageClose")]
extern "C-unwind" fn dbpageClose(mut pCursor: *mut sqlite3_vtab_cursor) -> i32 {
    let mut pCsr: *mut DbpageCursor = pCursor as *mut DbpageCursor;
    if (unsafe { (*pCsr).pPage1 }) != std::ptr::null_mut::<PgHdr>() {
        unsafe { sqlite3PagerUnrefPageOne(unsafe { (*pCsr).pPage1 }) };
    }
    unsafe { sqlite3_free(pCsr as *mut ()) };
    return 0 as i32;
}

/// Move a dbpagevfs cursor to the next entry in the file.
#[unsafe(link_section = ".text.slate_distinct.dbpage.dbpageNext")]
extern "C-unwind" fn dbpageNext(mut pCursor: *mut sqlite3_vtab_cursor) -> i32 {
    let mut rc: i32 = 0 as i32;
    let mut pCsr: *mut DbpageCursor = pCursor as *mut DbpageCursor;
    let __v502: *mut DbpageCursor = pCsr;
    let __v503: u32 = unsafe { (*__v502).pgno };
    let __v504: u32 = __v503.wrapping_add((1 as i32) as u32);
    unsafe {
        (*__v502).pgno = __v504;
    }
    return rc;
}

#[unsafe(link_section = ".text.slate_distinct.dbpage.dbpageEof")]
extern "C-unwind" fn dbpageEof(mut pCursor: *mut sqlite3_vtab_cursor) -> i32 {
    let mut pCsr: *mut DbpageCursor = pCursor as *mut DbpageCursor;
    return ((unsafe { (*pCsr).pgno }) > unsafe { (*pCsr).mxPgno }) as i32;
}

/// idxNum:
///
///     0     schema=main, full table scan
///     1     schema=main, pgno=?1
///     2     schema=?1, full table scan
///     3     schema=?1, pgno=?2
///
/// idxStr is not used
#[unsafe(link_section = ".text.slate_distinct.dbpage.dbpageFilter")]
extern "C-unwind" fn dbpageFilter(
    mut pCursor: *mut sqlite3_vtab_cursor,
    mut idxNum: i32,
    mut idxStr: *const i8,
    mut argc: i32,
    mut argv: *mut *mut sqlite3_value,
) -> i32 {
    let mut pCsr: *mut DbpageCursor = pCursor as *mut DbpageCursor;
    let mut pTab: *mut DbpageTable = (unsafe { (*pCursor).pVtab }) as *mut DbpageTable;
    let mut rc: i32 = 0 as i32;
    let mut db: *mut sqlite3 = unsafe { (*pTab).db };
    let mut pBt: *mut Btree = unsafe { std::mem::zeroed() };
    idxStr;
    argc;
    // Default setting is no rows of result
    unsafe {
        (*pCsr).pgno = (1 as i32) as u32;
    }
    unsafe {
        (*pCsr).mxPgno = (0 as i32) as u32;
    }
    if idxNum & (2 as i32) != (0 as i32) {
        let mut zSchema: *const i8 = unsafe { std::mem::zeroed() };
        0 as i32;
        zSchema = (unsafe {
            sqlite3_value_text(unsafe { *unsafe { argv.offset((0 as i32) as isize) } })
        }) as *const i8;
        unsafe {
            (*pCsr).iDb = unsafe { sqlite3FindDbName(db, zSchema) };
        }
        if (unsafe { (*pCsr).iDb }) < (0 as i32) {
            return 0 as i32;
        }
    } else {
        unsafe {
            (*pCsr).iDb = 0 as i32;
        }
    }
    pBt =
        unsafe { (*unsafe { unsafe { (*db).aDb }.offset((unsafe { (*pCsr).iDb }) as isize) }).pBt };
    if pBt == std::ptr::null_mut::<Btree>() {
        return 0 as i32;
    }
    unsafe {
        (*pCsr).pPager = unsafe { sqlite3BtreePager(pBt) };
    }
    unsafe {
        (*pCsr).szPage = unsafe { sqlite3BtreeGetPageSize(pBt) };
    }
    unsafe {
        (*pCsr).mxPgno = unsafe { sqlite3BtreeLastPage(pBt) };
    }
    if idxNum & (1 as i32) != (0 as i32) {
        let mut iPg: i64 = unsafe {
            sqlite3_value_int64(unsafe { *unsafe { argv.offset((idxNum >> (1 as i32)) as isize) } })
        };
        0 as i32;
        if iPg < ((1 as i32) as i64) || iPg > (((unsafe { (*pCsr).mxPgno }) as u64) as i64) {
            unsafe {
                (*pCsr).pgno = (1 as i32) as u32;
            }
            unsafe {
                (*pCsr).mxPgno = (0 as i32) as u32;
            }
        } else {
            unsafe {
                (*pCsr).pgno = (iPg as i32) as u32;
            }
            unsafe {
                (*pCsr).mxPgno = unsafe { (*pCsr).pgno };
            }
        }
    } else {
        0 as i32;
    }
    if (unsafe { (*pCsr).pPage1 }) != std::ptr::null_mut::<PgHdr>() {
        unsafe { sqlite3PagerUnrefPageOne(unsafe { (*pCsr).pPage1 }) };
    }
    rc = unsafe {
        sqlite3PagerGet(
            unsafe { (*pCsr).pPager },
            (1 as i32) as u32,
            unsafe { std::ptr::addr_of_mut!((*pCsr).pPage1) },
            0 as i32,
        )
    };
    return rc;
}

#[unsafe(link_section = ".text.slate_distinct.dbpage.dbpageColumn")]
extern "C-unwind" fn dbpageColumn(
    mut pCursor: *mut sqlite3_vtab_cursor,
    mut ctx: *mut sqlite3_context,
    mut i: i32,
) -> i32 {
    let mut pCsr: *mut DbpageCursor = pCursor as *mut DbpageCursor;
    let mut rc: i32 = 0 as i32;
    '__slate_break_485: {
        match i {
            0 => {
                // pgno
                unsafe { sqlite3_result_int64(ctx, ((unsafe { (*pCsr).pgno }) as u64) as i64) };
            }
            1 => {
                // data
                let mut pDbPage: *mut PgHdr = std::ptr::null_mut::<PgHdr>();
                if (unsafe { (*pCsr).pgno })
                    == (((unsafe { sqlite3PendingByte }) / unsafe { (*pCsr).szPage } + (1 as i32))
                        as u32)
                {
                    // The pending byte page. Assume it is zeroed out. Attempting to
                    // request this page from the page is an SQLITE_CORRUPT error.
                    unsafe { sqlite3_result_zeroblob(ctx, unsafe { (*pCsr).szPage }) };
                } else {
                    rc = unsafe {
                        sqlite3PagerGet(
                            unsafe { (*pCsr).pPager },
                            unsafe { (*pCsr).pgno },
                            std::ptr::addr_of_mut!(pDbPage),
                            0 as i32,
                        )
                    };
                    if rc == (0 as i32) {
                        unsafe {
                            sqlite3_result_blob(
                                ctx,
                                (unsafe { sqlite3PagerGetData(pDbPage) }) as *const (),
                                unsafe { (*pCsr).szPage },
                                unsafe {
                                    std::mem::transmute::<
                                        usize,
                                        Option<unsafe extern "C-unwind" fn(*mut ())>,
                                    >(-(1 as i32) as usize)
                                },
                            )
                        };
                    }
                    unsafe { sqlite3PagerUnref(pDbPage) };
                }
            }
            _ => {
                // schema
                let mut db: *mut sqlite3 = unsafe { sqlite3_context_db_handle(ctx) };
                unsafe {
                    sqlite3_result_text(
                        ctx,
                        (unsafe {
                            (*unsafe {
                                unsafe { (*db).aDb }.offset((unsafe { (*pCsr).iDb }) as isize)
                            })
                            .zDbSName
                        }) as *const i8,
                        -(1 as i32),
                        None,
                    )
                };
            }
        }
    }
    return rc;
}

#[unsafe(link_section = ".text.slate_distinct.dbpage.dbpageRowid")]
extern "C-unwind" fn dbpageRowid(
    mut pCursor: *mut sqlite3_vtab_cursor,
    mut pRowid: *mut i64,
) -> i32 {
    let mut pCsr: *mut DbpageCursor = pCursor as *mut DbpageCursor;
    unsafe {
        *pRowid = ((unsafe { (*pCsr).pgno }) as u64) as i64;
    }
    return 0 as i32;
}

/// Open write transactions. Since we do not know in advance which database
/// files will be written by the sqlite_dbpage virtual table, start a write
/// transaction on them all.
///
/// Return SQLITE_OK if successful, or an SQLite error code otherwise.
fn dbpageBeginTrans(mut pTab: *mut DbpageTable) -> i32 {
    let mut db: *mut sqlite3 = unsafe { (*pTab).db };
    let mut rc: i32 = 0 as i32;
    let mut i: i32 = 0 as i32;
    i = 0 as i32;
    '__slate_break_486: loop {
        if !(rc == (0 as i32) && i < unsafe { (*db).nDb }) {
            break;
        }
        let mut pBt: *mut Btree =
            unsafe { (*unsafe { unsafe { (*db).aDb }.offset(i as isize) }).pBt };
        if pBt != std::ptr::null_mut::<Btree>() {
            rc = unsafe { sqlite3BtreeBeginTrans(pBt, 1 as i32, std::ptr::null_mut::<i32>()) };
        }
        let __v505: i32 = i;
        let __v506: i32 = __v505 + (1 as i32);
        i = __v506;
    }
    return rc;
}

#[unsafe(link_section = ".text.slate_distinct.dbpage.dbpageUpdate")]
extern "C-unwind" fn dbpageUpdate(
    mut pVtab: *mut sqlite3_vtab,
    mut argc: i32,
    mut argv: *mut *mut sqlite3_value,
    mut pRowid: *mut i64,
) -> i32 {
    let mut __slate_storage_411: std::mem::MaybeUninit<*mut u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_411: *mut *mut u8 =
        std::ptr::addr_of_mut!(__slate_storage_411) as *mut *mut u8;
    let mut __slate_storage_508: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_508: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_508) as *mut i32;
    let mut __slate_storage_410: std::mem::MaybeUninit<*const ()> = std::mem::MaybeUninit::uninit();
    let __slate_slot_410: *mut *const () =
        std::ptr::addr_of_mut!(__slate_storage_410) as *mut *const ();
    let mut __slate_storage_507: std::mem::MaybeUninit<bool> = std::mem::MaybeUninit::uninit();
    let __slate_slot_507: *mut bool = std::ptr::addr_of_mut!(__slate_storage_507) as *mut bool;
    let mut __slate_storage_409: std::mem::MaybeUninit<*const i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_409: *mut *const i8 =
        std::ptr::addr_of_mut!(__slate_storage_409) as *mut *const i8;
    let mut __slate_storage_408: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_408: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_408) as *mut i32;
    let mut __slate_storage_407: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_407: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_407) as *mut i32;
    let mut __slate_storage_406: std::mem::MaybeUninit<*mut Pager> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_406: *mut *mut Pager =
        std::ptr::addr_of_mut!(__slate_storage_406) as *mut *mut Pager;
    let mut __slate_storage_405: std::mem::MaybeUninit<*mut Btree> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_405: *mut *mut Btree =
        std::ptr::addr_of_mut!(__slate_storage_405) as *mut *mut Btree;
    let mut __slate_storage_404: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_404: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_404) as *mut i32;
    let mut __slate_storage_403: std::mem::MaybeUninit<*mut i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_403: *mut *mut i8 =
        std::ptr::addr_of_mut!(__slate_storage_403) as *mut *mut i8;
    let mut __slate_storage_402: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_402: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_402) as *mut i32;
    let mut __slate_storage_401: std::mem::MaybeUninit<*mut PgHdr> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_401: *mut *mut PgHdr =
        std::ptr::addr_of_mut!(__slate_storage_401) as *mut *mut PgHdr;
    let mut __slate_storage_400: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_400: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_400) as *mut i64;
    let mut __slate_storage_399: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_399: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_399) as *mut u32;
    let mut __slate_storage_398: std::mem::MaybeUninit<*mut DbpageTable> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_398: *mut *mut DbpageTable =
        std::ptr::addr_of_mut!(__slate_storage_398) as *mut *mut DbpageTable;
    unsafe {
        '__join_0: {
            std::ptr::write(__slate_slot_398, pVtab as *mut DbpageTable);
            std::ptr::write(__slate_slot_401, std::ptr::null_mut::<PgHdr>());
            std::ptr::write(__slate_slot_402, 0 as i32);
            std::ptr::write(__slate_slot_403, std::ptr::null_mut::<i8>());
            pRowid;
            if (unsafe { (*unsafe { (*(*__slate_slot_398)).db }).flags })
                & (((268435456 as i32) as i64) as u64)
                != (0 as u64)
            {
                *__slate_slot_403 = b"read-only\0".as_ptr() as *mut i8;
            } else {
                if argc == (1 as i32) {
                    *__slate_slot_403 = b"cannot delete\0".as_ptr() as *mut i8;
                } else {
                    if (unsafe {
                        sqlite3_value_type(unsafe { *unsafe { argv.offset((0 as i32) as isize) } })
                    }) == (5 as i32)
                    {
                        *__slate_slot_400 = unsafe {
                            sqlite3_value_int64(unsafe {
                                *unsafe { argv.offset((2 as i32) as isize) }
                            })
                        };
                        *__slate_slot_408 = 1 as i32;
                    } else {
                        *__slate_slot_400 = ((((unsafe {
                            sqlite3_value_int64(unsafe {
                                *unsafe { argv.offset((0 as i32) as isize) }
                            })
                        }) as i32) as u32) as u64)
                            as i64;
                        if (unsafe {
                            sqlite3_value_int64(unsafe {
                                *unsafe { argv.offset((1 as i32) as isize) }
                            })
                        }) != *__slate_slot_400
                        {
                            *__slate_slot_403 = b"cannot insert\0".as_ptr() as *mut i8;
                            break '__join_0;
                        } else {
                            *__slate_slot_408 = 0 as i32;
                        }
                    }
                    if (unsafe {
                        sqlite3_value_type(unsafe { *unsafe { argv.offset((4 as i32) as isize) } })
                    }) == (5 as i32)
                    {
                        *__slate_slot_404 = 0 as i32;
                    } else {
                        std::ptr::write(
                            __slate_slot_409,
                            (unsafe {
                                sqlite3_value_text(unsafe {
                                    *unsafe { argv.offset((4 as i32) as isize) }
                                })
                            }) as *const i8,
                        );
                        *__slate_slot_404 = unsafe {
                            sqlite3FindDbName(
                                unsafe { (*(*__slate_slot_398)).db },
                                *__slate_slot_409,
                            )
                        };
                        if *__slate_slot_404 < (0 as i32) {
                            *__slate_slot_403 = b"no such schema\0".as_ptr() as *mut i8;
                            break '__join_0;
                        }
                    }
                    *__slate_slot_405 = unsafe {
                        (*unsafe {
                            unsafe { (*unsafe { (*(*__slate_slot_398)).db }).aDb }
                                .offset(*__slate_slot_404 as isize)
                        })
                        .pBt
                    };
                    if *__slate_slot_400 < ((1 as i32) as i64)
                        || *__slate_slot_400 > (((4294967294 as u32) as u64) as i64)
                        || *__slate_slot_405 == std::ptr::null_mut::<Btree>()
                    {
                        *__slate_slot_403 = b"bad page number\0".as_ptr() as *mut i8;
                    } else {
                        *__slate_slot_399 = (*__slate_slot_400 as i32) as u32;
                        *__slate_slot_407 = unsafe { sqlite3BtreeGetPageSize(*__slate_slot_405) };
                        if (unsafe {
                            sqlite3_value_type(unsafe {
                                *unsafe { argv.offset((3 as i32) as isize) }
                            })
                        }) != (4 as i32)
                        {
                            *__slate_slot_507 = true as bool;
                        } else {
                            *__slate_slot_507 = (unsafe {
                                sqlite3_value_bytes(unsafe {
                                    *unsafe { argv.offset((3 as i32) as isize) }
                                })
                            }) != *__slate_slot_407;
                        }
                        if *__slate_slot_507 {
                            if (unsafe {
                                sqlite3_value_type(unsafe {
                                    *unsafe { argv.offset((3 as i32) as isize) }
                                })
                            }) == (5 as i32)
                                && *__slate_slot_408 != (0 as i32)
                                && *__slate_slot_399 > ((1 as i32) as u32)
                            {
                                // "INSERT INTO dbpage($PGNO,NULL)" causes page number $PGNO and
                                // all subsequent pages to be deleted.
                                unsafe {
                                    (*(*__slate_slot_398)).iDbTrunc = *__slate_slot_404;
                                }
                                unsafe {
                                    (*(*__slate_slot_398)).pgnoTrunc =
                                        (*__slate_slot_399).wrapping_sub((1 as i32) as u32);
                                }
                                *__slate_slot_399 = (1 as i32) as u32;
                            } else {
                                *__slate_slot_403 = b"bad page value\0".as_ptr() as *mut i8;
                                break '__join_0;
                            }
                        }
                        if dbpageBeginTrans(*__slate_slot_398) != (0 as i32) {
                            *__slate_slot_403 = b"failed to open transaction\0".as_ptr() as *mut i8;
                        } else {
                            *__slate_slot_406 = unsafe { sqlite3BtreePager(*__slate_slot_405) };
                            *__slate_slot_402 = unsafe {
                                sqlite3PagerGet(
                                    *__slate_slot_406,
                                    *__slate_slot_399,
                                    std::ptr::addr_of_mut!(*__slate_slot_401),
                                    0 as i32,
                                )
                            };
                            if *__slate_slot_402 == (0 as i32) {
                                std::ptr::write(__slate_slot_410, unsafe {
                                    sqlite3_value_blob(unsafe {
                                        *unsafe { argv.offset((3 as i32) as isize) }
                                    })
                                });
                                std::ptr::write(__slate_slot_508, unsafe {
                                    sqlite3PagerWrite(*__slate_slot_401)
                                });
                                *__slate_slot_402 = *__slate_slot_508;
                                if *__slate_slot_508 == (0 as i32)
                                    && *__slate_slot_410 != std::ptr::null::<()>()
                                {
                                    std::ptr::write(
                                        __slate_slot_411,
                                        (unsafe { sqlite3PagerGetData(*__slate_slot_401) })
                                            as *mut u8,
                                    );
                                    unsafe {
                                        memcpy(
                                            *__slate_slot_411 as *mut (),
                                            *__slate_slot_410,
                                            (*__slate_slot_407 as i64) as u64,
                                        )
                                    };
                                    unsafe {
                                        (*(*__slate_slot_398)).pgnoTrunc = (0 as i32) as u32;
                                    }
                                }
                            }
                            if *__slate_slot_402 != (0 as i32) {
                                unsafe {
                                    (*(*__slate_slot_398)).pgnoTrunc = (0 as i32) as u32;
                                }
                            }
                            unsafe { sqlite3PagerUnref(*__slate_slot_401) };
                            return *__slate_slot_402;
                        }
                    }
                }
            }
        }
        unsafe {
            (*(*__slate_slot_398)).pgnoTrunc = (0 as i32) as u32;
        }
        unsafe { sqlite3_free((unsafe { (*pVtab).zErrMsg }) as *mut ()) };
        unsafe {
            (*pVtab).zErrMsg = unsafe {
                sqlite3_mprintf(
                    (b"%s\0".as_ptr() as *mut i8) as *const i8,
                    *__slate_slot_403,
                )
            };
        }
        return 1 as i32;
    }
    return unsafe { std::mem::zeroed() };
}

#[unsafe(link_section = ".text.slate_distinct.dbpage.dbpageBegin")]
extern "C-unwind" fn dbpageBegin(mut pVtab: *mut sqlite3_vtab) -> i32 {
    let mut pTab: *mut DbpageTable = pVtab as *mut DbpageTable;
    unsafe {
        (*pTab).pgnoTrunc = (0 as i32) as u32;
    }
    return 0 as i32;
}

/// Invoke sqlite3PagerTruncate() as necessary, just prior to COMMIT
#[unsafe(link_section = ".text.slate_distinct.dbpage.dbpageSync")]
extern "C-unwind" fn dbpageSync(mut pVtab: *mut sqlite3_vtab) -> i32 {
    let mut pTab: *mut DbpageTable = pVtab as *mut DbpageTable;
    if (unsafe { (*pTab).pgnoTrunc }) > ((0 as i32) as u32) {
        let mut pBt: *mut Btree = unsafe {
            (*unsafe {
                unsafe { (*unsafe { (*pTab).db }).aDb }
                    .offset((unsafe { (*pTab).iDbTrunc }) as isize)
            })
            .pBt
        };
        let mut pPager: *mut Pager = unsafe { sqlite3BtreePager(pBt) };
        unsafe { sqlite3BtreeEnter(pBt) };
        if (unsafe { (*pTab).pgnoTrunc }) < unsafe { sqlite3BtreeLastPage(pBt) } {
            unsafe { sqlite3PagerTruncateImage(pPager, unsafe { (*pTab).pgnoTrunc }) };
        }
        unsafe { sqlite3BtreeLeave(pBt) };
    }
    unsafe {
        (*pTab).pgnoTrunc = (0 as i32) as u32;
    }
    return 0 as i32;
}

/// Cancel any pending truncate.
#[unsafe(link_section = ".text.slate_distinct.dbpage.dbpageRollbackTo")]
extern "C-unwind" fn dbpageRollbackTo(mut pVtab: *mut sqlite3_vtab, mut notUsed1: i32) -> i32 {
    let mut pTab: *mut DbpageTable = pVtab as *mut DbpageTable;
    unsafe {
        (*pTab).pgnoTrunc = (0 as i32) as u32;
    }
    notUsed1;
    return 0 as i32;
}

/// Invoke this routine to register the "dbpage" virtual table module
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.dbpage.sqlite3DbpageRegister")]
extern "C-unwind" fn sqlite3DbpageRegister(mut db: *mut sqlite3) -> i32 {
    // iVersion
    // xCreate
    // xConnect
    // xBestIndex
    // xDisconnect
    // xDestroy
    // xOpen - open a cursor
    // xClose - close a cursor
    // xFilter - configure scan constraints
    // xNext - advance a cursor
    // xEof - check for end of scan
    // xColumn - read data
    // xRowid - read data
    // xUpdate
    // xBegin
    // xSync
    // xCommit
    // xRollback
    // xFindMethod
    // xRename
    // xSavepoint
    // xRelease
    // xRollbackTo
    // xShadowName
    // xIntegrity
    return unsafe {
        sqlite3_create_module(
            db,
            (b"sqlite_dbpage\0".as_ptr() as *mut i8) as *const i8,
            (unsafe { std::ptr::addr_of_mut!(dbpage_module) }) as *const sqlite3_module,
            std::ptr::null_mut::<()>(),
        )
    };
}

static mut dbpage_module: sqlite3_module = sqlite3_module {
    iVersion: 2 as i32,
    xCreate: Some(dbpageConnect),
    xConnect: Some(dbpageConnect),
    xBestIndex: Some(dbpageBestIndex),
    xDisconnect: Some(dbpageDisconnect),
    xDestroy: Some(dbpageDisconnect),
    xOpen: Some(dbpageOpen),
    xClose: Some(dbpageClose),
    xFilter: Some(dbpageFilter),
    xNext: Some(dbpageNext),
    xEof: Some(dbpageEof),
    xColumn: Some(dbpageColumn),
    xRowid: Some(dbpageRowid),
    xUpdate: Some(dbpageUpdate),
    xBegin: Some(dbpageBegin),
    xSync: Some(dbpageSync),
    xCommit: None,
    xRollback: None,
    xFindFunction: None,
    xRename: None,
    xSavepoint: None,
    xRelease: None,
    xRollbackTo: Some(dbpageRollbackTo),
    xShadowName: None,
    xIntegrity: None,
};
