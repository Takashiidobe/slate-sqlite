//! 2010 July 12
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
//! This file contains an implementation of the "dbstat" virtual table.
//!
//! The dbstat virtual table is used to extract low-level storage
//! information from an SQLite database in order to implement the
//! "sqlite3_analyzer" utility.  See the ../tool/spaceanal.tcl script
//! for an example implementation.
//!
//! Additional information is available on the "dbstat.html" page of the
//! official SQLite documentation.
unsafe extern "C" {
    fn sqlite3_mprintf(__v490: *const i8, ...) -> *mut i8;
    fn sqlite3_malloc(__v491: i32) -> *mut ();
    fn sqlite3_malloc64(__v492: u64) -> *mut ();
    fn sqlite3_free(__v493: *mut ());
    fn sqlite3_prepare_v2(
        db: *mut sqlite3,
        zSql: *const i8,
        nByte: i32,
        ppStmt: *mut *mut sqlite3_stmt,
        pzTail: *mut *const i8,
    ) -> i32;
    fn sqlite3_step(__v499: *mut sqlite3_stmt) -> i32;
    fn sqlite3_column_int64(__v500: *mut sqlite3_stmt, iCol: i32) -> i64;
    fn sqlite3_column_text(__v502: *mut sqlite3_stmt, iCol: i32) -> *const u8;
    fn sqlite3_finalize(pStmt: *mut sqlite3_stmt) -> i32;
    fn sqlite3_reset(pStmt: *mut sqlite3_stmt) -> i32;
    fn sqlite3_value_double(__v506: *mut sqlite3_value) -> f64;
    fn sqlite3_value_text(__v507: *mut sqlite3_value) -> *const u8;
    fn sqlite3_context_db_handle(__v508: *mut sqlite3_context) -> *mut sqlite3;
    fn sqlite3_result_int(__v509: *mut sqlite3_context, __v510: i32);
    fn sqlite3_result_int64(__v511: *mut sqlite3_context, __v512: i64);
    fn sqlite3_result_text(
        __v513: *mut sqlite3_context,
        __v514: *const i8,
        __v515: i32,
        __v516: Option<unsafe extern "C-unwind" fn(*mut ())>,
    );
    fn sqlite3_create_module(
        db: *mut sqlite3,
        zName: *const i8,
        p: *const sqlite3_module,
        pClientData: *mut (),
    ) -> i32;
    fn sqlite3_declare_vtab(__v521: *mut sqlite3, zSQL: *const i8) -> i32;
    fn sqlite3_str_new(__v523: *mut sqlite3) -> *mut sqlite3_str;
    fn sqlite3_str_finish(__v524: *mut sqlite3_str) -> *mut i8;
    fn sqlite3_str_appendf(__v525: *mut sqlite3_str, zFormat: *const i8, ...);
    fn sqlite3_vtab_config(__v527: *mut sqlite3, op: i32, ...) -> i32;
    fn memcpy(__dest: *mut (), __src: *const (), __n: u64) -> *mut ();
    fn memset(__s: *mut (), __c: i32, __n: u64) -> *mut ();
    fn sqlite3OsFileControl(__v535: *mut sqlite3_file, __v536: i32, __v537: *mut ()) -> i32;
    fn sqlite3PagerGet(pPager: *mut Pager, pgno: u32, ppPage: *mut *mut PgHdr, clrFlag: i32)
    -> i32;
    fn sqlite3PagerUnref(__v542: *mut PgHdr);
    fn sqlite3PagerGetData(__v543: *mut PgHdr) -> *mut ();
    fn sqlite3PagerPagecount(__v544: *mut Pager, __v545: *mut i32);
    fn sqlite3PagerFile(__v546: *mut Pager) -> *mut sqlite3_file;
    fn sqlite3BtreeGetPageSize(__v547: *mut Btree) -> i32;
    fn sqlite3BtreeGetReserveNoMutex(p: *mut Btree) -> i32;
    fn sqlite3BtreePager(__v549: *mut Btree) -> *mut Pager;
    fn sqlite3BtreeEnter(__v550: *mut Btree);
    fn sqlite3BtreeLeave(__v551: *mut Btree);
    fn sqlite3CorruptError(__v552: i32) -> i32;
    fn sqlite3TokenInit(__v553: *mut Token, __v554: *mut i8);
    fn sqlite3GetVarint(__v555: *const u8, __v556: *mut u64) -> u8;
    fn sqlite3GetVarint32(__v557: *const u8, __v558: *mut u32) -> u8;
    fn sqlite3FindDb(__v559: *mut sqlite3, __v560: *mut Token) -> i32;
    fn sqlite3FindDbName(__v561: *mut sqlite3, __v562: *const i8) -> i32;
    fn sqlite3Get4byte(__v563: *const u8) -> u32;
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
struct _ht {
    count: u32,
    chain: *mut HashElem,
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
    __slate_bits_0: __slate_bits::__SlateBits68U0,
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
    u: __SlateRecord171,
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
    __slate_bits_0: __slate_bits::__SlateBits92U0,
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
    u: __SlateRecord179,
    pLeft: *mut Expr,
    pRight: *mut Expr,
    x: __SlateRecord180,
    nHeight: i32,
    iTable: i32,
    iColumn: i16,
    iAgg: i16,
    w: __SlateRecord181,
    pAggInfo: *mut AggInfo,
    y: __SlateRecord182,
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
    fg: __SlateRecord189,
    iCursor: i32,
    colUsed: u64,
    u1: __SlateRecord190,
    u2: __SlateRecord191,
    u3: __SlateRecord192,
    u4: __SlateRecord193,
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
    u1: __SlateRecord195,
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
union __SlateRecord179 {
    zToken: *mut i8,
    iValue: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord180 {
    pList: *mut ExprList,
    pSelect: *mut Select,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord181 {
    iJoin: i32,
    iOfst: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord182 {
    pTab: *mut Table,
    pWin: *mut Window,
    nReg: i32,
    sub: __SlateRecord183,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord183 {
    iAddr: i32,
    regReturn: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct ExprList_item {
    pExpr: *mut Expr,
    zEName: *mut i8,
    fg: __SlateRecord185,
    u: __SlateRecord186,
}

#[repr(C, align(4))]
#[derive(Clone, Copy)]
struct __SlateRecord185 {
    sortFlags: u8,
    __slate_bits_0: __slate_bits::__SlateBits185U0,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord186 {
    x: __SlateRecord187,
    iConstExprReg: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord187 {
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
struct __SlateRecord189 {
    jointype: u8,
    __slate_bits_0: __slate_bits::__SlateBits189U0,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord190 {
    zIndexedBy: *mut i8,
    pFuncArg: *mut ExprList,
    nRow: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord191 {
    pIBIndex: *mut Index,
    pCteUse: *mut CteUse,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord192 {
    pOn: *mut Expr,
    pUsing: *mut IdList,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord193 {
    pSchema: *mut Schema,
    zDatabase: *mut i8,
    pSubq: *mut Subquery,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord195 {
    cr: __SlateRecord196,
    d: __SlateRecord197,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord196 {
    addrCrTab: i32,
    regRowid: i32,
    regRoot: i32,
    constraintName: Token,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord197 {
    pReturning: *mut Returning,
}

// Requires access to internal data structures
// The pager and btree modules arrange objects in memory so that there are
// always approximately 200 bytes of addressable memory following each page
// buffer. This way small buffer overreads caused by corrupt database pages
// do not cause undefined behaviour. This module pads each page buffer
// by the following number of bytes for the same purpose.
/// Page paths:
///
///   The value of the 'path' column describes the path taken from the
///   root-node of the b-tree structure to each page. The value of the
///   root-node path is '/'.
///
///   The value of the path for the left-most child page of the root of
///   a b-tree is '/000/'. (Btrees store content ordered from left to right
///   so the pages to the left have smaller keys than the pages to the right.)
///   The next to left-most child of the root page is
///   '/001', and so on, each sibling page identified by a 3-digit hex
///   value. The children of the 451st left-most sibling have paths such
///   as '/1c2/000/, '/1c2/001/' etc.
///
///   Overflow pages are specified by appending a '+' character and a
///   six-digit hexadecimal value to the path to the cell they are linked
///   from. For example, the three overflow pages in a chain linked from
///   the left-most cell of the 450th child of the root page are identified
///   by the paths:
///
///      '/1c2/000+000000'         // First page in overflow chain
///      '/1c2/000+000001'         // Second page in overflow chain
///      '/1c2/000+000002'         // Third page in overflow chain
///
///   If the paths are sorted using the BINARY collation sequence, then
///   the overflow pages associated with a cell will appear earlier in the
///   sort-order than its child page:
///
///      '/1c2/000/'               // Left-most child of 451st child of root
///  0 Name of table or index
///  1 Path to page from root (NULL for agg)
///  2 Page number (page count for aggregates)
///  3 'internal', 'leaf', 'overflow', or NULL
///  4 Cells on page (0 for overflow)
///  5 Bytes of payload on this page
///  6 Bytes of unused space on this page
///  7 Largest payload size of all cells
///  8 Offset of page in file (NULL for agg)
///  9 Size of the page (sum for aggregate)
/// 10 Database schema being analyzed
/// 11 aggregate info for each table
static mut zDbstatSchema: __SlateAlign16<[i8; 258]> = __SlateAlign16([
    67 as i8, 82 as i8, 69 as i8, 65 as i8, 84 as i8, 69 as i8, 32 as i8, 84 as i8, 65 as i8,
    66 as i8, 76 as i8, 69 as i8, 32 as i8, 120 as i8, 40 as i8, 32 as i8, 110 as i8, 97 as i8,
    109 as i8, 101 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8,
    84 as i8, 69 as i8, 88 as i8, 84 as i8, 44 as i8, 32 as i8, 112 as i8, 97 as i8, 116 as i8,
    104 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8, 84 as i8,
    69 as i8, 88 as i8, 84 as i8, 44 as i8, 32 as i8, 112 as i8, 97 as i8, 103 as i8, 101 as i8,
    110 as i8, 111 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8, 73 as i8, 78 as i8,
    84 as i8, 69 as i8, 71 as i8, 69 as i8, 82 as i8, 44 as i8, 32 as i8, 112 as i8, 97 as i8,
    103 as i8, 101 as i8, 116 as i8, 121 as i8, 112 as i8, 101 as i8, 32 as i8, 32 as i8, 32 as i8,
    84 as i8, 69 as i8, 88 as i8, 84 as i8, 44 as i8, 32 as i8, 110 as i8, 99 as i8, 101 as i8,
    108 as i8, 108 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8, 73 as i8,
    78 as i8, 84 as i8, 69 as i8, 71 as i8, 69 as i8, 82 as i8, 44 as i8, 32 as i8, 112 as i8,
    97 as i8, 121 as i8, 108 as i8, 111 as i8, 97 as i8, 100 as i8, 32 as i8, 32 as i8, 32 as i8,
    32 as i8, 73 as i8, 78 as i8, 84 as i8, 69 as i8, 71 as i8, 69 as i8, 82 as i8, 44 as i8,
    32 as i8, 117 as i8, 110 as i8, 117 as i8, 115 as i8, 101 as i8, 100 as i8, 32 as i8, 32 as i8,
    32 as i8, 32 as i8, 32 as i8, 73 as i8, 78 as i8, 84 as i8, 69 as i8, 71 as i8, 69 as i8,
    82 as i8, 44 as i8, 32 as i8, 109 as i8, 120 as i8, 95 as i8, 112 as i8, 97 as i8, 121 as i8,
    108 as i8, 111 as i8, 97 as i8, 100 as i8, 32 as i8, 73 as i8, 78 as i8, 84 as i8, 69 as i8,
    71 as i8, 69 as i8, 82 as i8, 44 as i8, 32 as i8, 112 as i8, 103 as i8, 111 as i8, 102 as i8,
    102 as i8, 115 as i8, 101 as i8, 116 as i8, 32 as i8, 32 as i8, 32 as i8, 73 as i8, 78 as i8,
    84 as i8, 69 as i8, 71 as i8, 69 as i8, 82 as i8, 44 as i8, 32 as i8, 112 as i8, 103 as i8,
    115 as i8, 105 as i8, 122 as i8, 101 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8, 32 as i8,
    73 as i8, 78 as i8, 84 as i8, 69 as i8, 71 as i8, 69 as i8, 82 as i8, 44 as i8, 32 as i8,
    115 as i8, 99 as i8, 104 as i8, 101 as i8, 109 as i8, 97 as i8, 32 as i8, 32 as i8, 32 as i8,
    32 as i8, 32 as i8, 84 as i8, 69 as i8, 88 as i8, 84 as i8, 32 as i8, 72 as i8, 73 as i8,
    68 as i8, 68 as i8, 69 as i8, 78 as i8, 44 as i8, 32 as i8, 97 as i8, 103 as i8, 103 as i8,
    114 as i8, 101 as i8, 103 as i8, 97 as i8, 116 as i8, 101 as i8, 32 as i8, 32 as i8, 66 as i8,
    79 as i8, 79 as i8, 76 as i8, 69 as i8, 65 as i8, 78 as i8, 32 as i8, 72 as i8, 73 as i8,
    68 as i8, 68 as i8, 69 as i8, 78 as i8, 41 as i8, 0 as i8,
]);

/// Size information for a single cell within a btree page
#[repr(C)]
#[derive(Clone, Copy)]
struct StatCell {
    /// Bytes of local payload
    nLocal: i32,
    /// Child node (or 0 if this is a leaf)
    iChildPg: u32,
    /// Entries in aOvfl[]
    nOvfl: i32,
    /// Array of overflow page numbers
    aOvfl: *mut u32,
    /// Bytes of payload on final overflow page
    nLastOvfl: i32,
    /// Iterates through aOvfl[]
    iOvfl: i32,
}

/// Size information for a single btree page
#[repr(C)]
#[derive(Clone, Copy)]
struct StatPage {
    /// Page number
    iPgno: u32,
    /// Page buffer from sqlite3_malloc()
    aPg: *mut u8,
    /// Current cell
    iCell: i32,
    /// Path to this page
    zPath: *mut i8,
    /// Variables populated by statDecodePage():
    /// Copy of flags byte
    flags: u8,
    /// Number of cells on page
    nCell: i32,
    /// Number of unused bytes on page
    nUnused: i32,
    /// Array of parsed cells
    aCell: *mut StatCell,
    /// Right-child page number (or 0)
    iRightChildPg: u32,
    /// Largest payload of any cell on the page
    nMxPayload: i32,
}

/// The cursor for scanning the dbstat virtual table
#[repr(C)]
#[derive(Clone, Copy)]
struct StatCursor {
    /// base class.  MUST BE FIRST!
    base: sqlite3_vtab_cursor,
    /// Iterates through set of root pages
    pStmt: *mut sqlite3_stmt,
    /// After pStmt has returned SQLITE_DONE
    isEof: u8,
    /// Aggregate results for each table
    isAgg: u8,
    /// Schema used for this query
    iDb: i32,
    /// Pages in path to current page
    aPage: [StatPage; 32],
    /// Current entry in aPage[]
    iPage: i32,
    /// Values to return.
    /// Value of 'pageno' column
    iPageno: u32,
    /// Value of 'name' column
    zName: *mut i8,
    /// Value of 'path' column
    zPath: *mut i8,
    /// Value of 'pagetype' column
    zPagetype: *mut i8,
    /// Number of pages in current btree
    nPage: i32,
    /// Value of 'ncell' column
    nCell: i32,
    /// Value of 'mx_payload' column
    nMxPayload: i32,
    /// Value of 'unused' column
    nUnused: i64,
    /// Value of 'payload' column
    nPayload: i64,
    /// Value of 'pgOffset' column
    iOffset: i64,
    /// Value of 'pgSize' column
    szPage: i64,
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
    pub struct __SlateBits68U0 {
        #[bits(4)]
        pub notNull: u32,
        #[bits(4)]
        pub eCType: u32,
    }
    #[bitfields::bitfield([u8; 2], c_names = true, new = false, from_into_bits = false, from_traits = false, default = false, debug = false, builder = false, bit_ops = false)]
    pub struct __SlateBits185U0 {
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
    pub struct __SlateBits189U0 {
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
}

/// An instance of the DBSTAT virtual table
#[repr(C)]
#[derive(Clone, Copy)]
struct StatTable {
    /// base class.  MUST BE FIRST!
    base: sqlite3_vtab,
    /// Database connection that owns this vtab
    db: *mut sqlite3,
    /// Index of database to analyze
    iDb: i32,
}

/// Connect to or create a new DBSTAT virtual table.
#[unsafe(link_section = ".text.slate_distinct.dbstat.statConnect")]
extern "C-unwind" fn statConnect(
    mut db: *mut sqlite3,
    mut pAux: *mut (),
    mut argc: i32,
    mut argv: *const *const i8,
    mut ppVtab: *mut *mut sqlite3_vtab,
    mut pzErr: *mut *mut i8,
) -> i32 {
    let mut pTab: *mut StatTable = std::ptr::null_mut::<StatTable>();
    let mut rc: i32 = 0 as i32;
    let mut iDb: i32 = 0 as i32;
    pAux;
    if argc >= (4 as i32) {
        let mut nm: Token = unsafe { std::mem::zeroed() };
        unsafe {
            sqlite3TokenInit(
                std::ptr::addr_of_mut!(nm),
                (unsafe { *unsafe { argv.offset((3 as i32) as isize) } }) as *mut i8,
            )
        };
        iDb = unsafe { sqlite3FindDb(db, std::ptr::addr_of_mut!(nm)) };
        if iDb < (0 as i32) {
            unsafe {
                *pzErr = unsafe {
                    sqlite3_mprintf(
                        (b"no such database: %s\0".as_ptr() as *mut i8) as *const i8,
                        unsafe { *unsafe { argv.offset((3 as i32) as isize) } },
                    )
                };
            }
            return 1 as i32;
        }
    } else {
        iDb = 0 as i32;
    }
    unsafe { sqlite3_vtab_config(db, 3 as i32) };
    rc = unsafe {
        sqlite3_declare_vtab(db, unsafe {
            std::ptr::addr_of!(zDbstatSchema.0) as *const i8
        })
    };
    if rc == (0 as i32) {
        pTab = (unsafe { sqlite3_malloc64(40 as u64) }) as *mut StatTable;
        if pTab == std::ptr::null_mut::<StatTable>() {
            rc = 7 as i32;
        }
    }
    0 as i32;
    if rc == (0 as i32) {
        unsafe { memset(pTab as *mut (), 0 as i32, 40 as u64) };
        unsafe {
            (*pTab).db = db;
        }
        unsafe {
            (*pTab).iDb = iDb;
        }
    }
    unsafe {
        *ppVtab = pTab as *mut sqlite3_vtab;
    }
    return rc;
}

/// Disconnect from or destroy the DBSTAT virtual table.
#[unsafe(link_section = ".text.slate_distinct.dbstat.statDisconnect")]
extern "C-unwind" fn statDisconnect(mut pVtab: *mut sqlite3_vtab) -> i32 {
    unsafe { sqlite3_free(pVtab as *mut ()) };
    return 0 as i32;
}

/// Compute the best query strategy and return the result in idxNum.
///
///   idxNum-Bit        Meaning
///   ----------        ----------------------------------------------
///      0x01           There is a schema=? term in the WHERE clause
///      0x02           There is a name=? term in the WHERE clause
///      0x04           There is an aggregate=? term in the WHERE clause
///      0x08           Output should be ordered by name and path
#[unsafe(link_section = ".text.slate_distinct.dbstat.statBestIndex")]
extern "C-unwind" fn statBestIndex(
    mut tab: *mut sqlite3_vtab,
    mut pIdxInfo: *mut sqlite3_index_info,
) -> i32 {
    let mut i: i32 = 0 as i32;
    let mut iSchema: i32 = -(1 as i32);
    let mut iName: i32 = -(1 as i32);
    let mut iAgg: i32 = -(1 as i32);
    tab;
    // Look for a valid schema=? constraint.  If found, change the idxNum to
    // 1 and request the value of that constraint be sent to xFilter.  And
    // lower the cost estimate to encourage the constrained version to be
    // used.
    i = 0 as i32;
    '__slate_break_566: loop {
        if !(i < unsafe { (*pIdxInfo).nConstraint }) {
            break;
        }
        if (((unsafe { (*unsafe { unsafe { (*pIdxInfo).aConstraint }.offset(i as isize) }).op })
            as u32) as i32)
            != (2 as i32)
        {
        } else {
            if (((unsafe {
                (*unsafe { unsafe { (*pIdxInfo).aConstraint }.offset(i as isize) }).usable
            }) as u32) as i32)
                == (0 as i32)
            {
                // Force DBSTAT table should always be the right-most table in a join
                return 19 as i32;
            }
            '__slate_break_567: {
                match unsafe {
                    (*unsafe { unsafe { (*pIdxInfo).aConstraint }.offset(i as isize) }).iColumn
                } {
                    0 => {
                        // name
                        iName = i;
                    }
                    10 => {
                        // schema
                        iSchema = i;
                    }
                    11 => {
                        // aggregate
                        iAgg = i;
                    }
                    _ => {}
                }
            }
        }
        let __v590: i32 = i;
        let __v591: i32 = __v590 + (1 as i32);
        i = __v591;
    }
    i = 0 as i32;
    if iSchema >= (0 as i32) {
        let __v592: i32 = i;
        let __v593: i32 = __v592 + (1 as i32);
        i = __v593;
        unsafe {
            (*unsafe { unsafe { (*pIdxInfo).aConstraintUsage }.offset(iSchema as isize) })
                .argvIndex = __v593;
        }
        unsafe {
            (*unsafe { unsafe { (*pIdxInfo).aConstraintUsage }.offset(iSchema as isize) }).omit =
                ((1 as i32) as i8) as u8;
        }
        let __v594: *mut sqlite3_index_info = pIdxInfo;
        let __v595: i32 = unsafe { (*__v594).idxNum };
        let __v596: i32 = __v595 | (1 as i32);
        unsafe {
            (*__v594).idxNum = __v596;
        }
    }
    if iName >= (0 as i32) {
        let __v597: i32 = i;
        let __v598: i32 = __v597 + (1 as i32);
        i = __v598;
        unsafe {
            (*unsafe { unsafe { (*pIdxInfo).aConstraintUsage }.offset(iName as isize) })
                .argvIndex = __v598;
        }
        let __v599: *mut sqlite3_index_info = pIdxInfo;
        let __v600: i32 = unsafe { (*__v599).idxNum };
        let __v601: i32 = __v600 | (2 as i32);
        unsafe {
            (*__v599).idxNum = __v601;
        }
    }
    if iAgg >= (0 as i32) {
        let __v602: i32 = i;
        let __v603: i32 = __v602 + (1 as i32);
        i = __v603;
        unsafe {
            (*unsafe { unsafe { (*pIdxInfo).aConstraintUsage }.offset(iAgg as isize) }).argvIndex =
                __v603;
        }
        let __v604: *mut sqlite3_index_info = pIdxInfo;
        let __v605: i32 = unsafe { (*__v604).idxNum };
        let __v606: i32 = __v605 | (4 as i32);
        unsafe {
            (*__v604).idxNum = __v606;
        }
    }
    unsafe {
        (*pIdxInfo).estimatedCost = 1.0f64;
    }
    // Records are always returned in ascending order of (name, path).
    // If this will satisfy the client, set the orderByConsumed flag so that
    // SQLite does not do an external sort.
    if (unsafe { (*pIdxInfo).nOrderBy }) == (1 as i32)
        && (unsafe {
            (*unsafe { unsafe { (*pIdxInfo).aOrderBy }.offset((0 as i32) as isize) }).iColumn
        }) == (0 as i32)
        && (((unsafe {
            (*unsafe { unsafe { (*pIdxInfo).aOrderBy }.offset((0 as i32) as isize) }).desc
        }) as u32) as i32)
            == (0 as i32)
        || (unsafe { (*pIdxInfo).nOrderBy }) == (2 as i32)
            && (unsafe {
                (*unsafe { unsafe { (*pIdxInfo).aOrderBy }.offset((0 as i32) as isize) }).iColumn
            }) == (0 as i32)
            && (((unsafe {
                (*unsafe { unsafe { (*pIdxInfo).aOrderBy }.offset((0 as i32) as isize) }).desc
            }) as u32) as i32)
                == (0 as i32)
            && (unsafe {
                (*unsafe { unsafe { (*pIdxInfo).aOrderBy }.offset((1 as i32) as isize) }).iColumn
            }) == (1 as i32)
            && (((unsafe {
                (*unsafe { unsafe { (*pIdxInfo).aOrderBy }.offset((1 as i32) as isize) }).desc
            }) as u32) as i32)
                == (0 as i32)
    {
        unsafe {
            (*pIdxInfo).orderByConsumed = 1 as i32;
        }
        let __v607: *mut sqlite3_index_info = pIdxInfo;
        let __v608: i32 = unsafe { (*__v607).idxNum };
        let __v609: i32 = __v608 | (8 as i32);
        unsafe {
            (*__v607).idxNum = __v609;
        }
    }
    let __v610: *mut sqlite3_index_info = pIdxInfo;
    let __v611: i32 = unsafe { (*__v610).idxFlags };
    let __v612: i32 = __v611 | (2 as i32);
    unsafe {
        (*__v610).idxFlags = __v612;
    }
    return 0 as i32;
}

/// Open a new DBSTAT cursor.
#[unsafe(link_section = ".text.slate_distinct.dbstat.statOpen")]
extern "C-unwind" fn statOpen(
    mut pVTab: *mut sqlite3_vtab,
    mut ppCursor: *mut *mut sqlite3_vtab_cursor,
) -> i32 {
    let mut pTab: *mut StatTable = pVTab as *mut StatTable;
    let mut pCsr: *mut StatCursor = unsafe { std::mem::zeroed() };
    pCsr = (unsafe { sqlite3_malloc64(2152 as u64) }) as *mut StatCursor;
    if pCsr == std::ptr::null_mut::<StatCursor>() {
        return 7 as i32;
    } else {
        unsafe { memset(pCsr as *mut (), 0 as i32, 2152 as u64) };
        unsafe {
            (*pCsr).base.pVtab = pVTab;
        }
        unsafe {
            (*pCsr).iDb = unsafe { (*pTab).iDb };
        }
    }
    unsafe {
        *ppCursor = pCsr as *mut sqlite3_vtab_cursor;
    }
    return 0 as i32;
}

fn statClearCells(mut p: *mut StatPage) {
    let mut i: i32 = 0 as i32;
    if (unsafe { (*p).aCell }) != std::ptr::null_mut::<StatCell>() {
        i = 0 as i32;
        '__slate_break_568: loop {
            if !(i < unsafe { (*p).nCell }) {
                break;
            }
            unsafe {
                sqlite3_free(
                    (unsafe { (*unsafe { unsafe { (*p).aCell }.offset(i as isize) }).aOvfl })
                        as *mut (),
                )
            };
            let __v613: i32 = i;
            let __v614: i32 = __v613 + (1 as i32);
            i = __v614;
        }
        unsafe { sqlite3_free((unsafe { (*p).aCell }) as *mut ()) };
    }
    unsafe {
        (*p).nCell = 0 as i32;
    }
    unsafe {
        (*p).aCell = std::ptr::null_mut::<StatCell>();
    }
}

fn statClearPage(mut p: *mut StatPage) {
    let mut aPg: *mut u8 = unsafe { (*p).aPg };
    statClearCells(p);
    unsafe { sqlite3_free((unsafe { (*p).zPath }) as *mut ()) };
    unsafe { memset(p as *mut (), 0 as i32, 64 as u64) };
    unsafe {
        (*p).aPg = aPg;
    }
}

fn statResetCsr(mut pCsr: *mut StatCursor) {
    let mut i: i32 = 0 as i32;
    // In some circumstances, specifically if an OOM has occurred, the call
    // to sqlite3_reset() may cause the pager to be reset (emptied). It is
    // important that statClearPage() is called to free any page refs before
    // this happens. dbsqlfuzz 9ed3e4e3816219d3509d711636c38542bf3f40b1.
    i = 0 as i32;
    '__slate_break_569: loop {
        if !(i < ((((2048 as u64) / (64 as u64)) as u32) as i32)) {
            break;
        }
        statClearPage(unsafe {
            unsafe { (*pCsr).aPage.as_mut_ptr() as *mut StatPage }.offset(i as isize)
        });
        unsafe {
            sqlite3_free(
                (unsafe {
                    (*unsafe {
                        unsafe { (*pCsr).aPage.as_mut_ptr() as *mut StatPage }.offset(i as isize)
                    })
                    .aPg
                }) as *mut (),
            )
        };
        unsafe {
            (*unsafe {
                unsafe { (*pCsr).aPage.as_mut_ptr() as *mut StatPage }.offset(i as isize)
            })
            .aPg = std::ptr::null_mut::<u8>();
        }
        let __v615: i32 = i;
        let __v616: i32 = __v615 + (1 as i32);
        i = __v616;
    }
    unsafe { sqlite3_reset(unsafe { (*pCsr).pStmt }) };
    unsafe {
        (*pCsr).iPage = 0 as i32;
    }
    unsafe { sqlite3_free((unsafe { (*pCsr).zPath }) as *mut ()) };
    unsafe {
        (*pCsr).zPath = std::ptr::null_mut::<i8>();
    }
    unsafe {
        (*pCsr).isEof = ((0 as i32) as i8) as u8;
    }
}

/// Resize the space-used counters inside of the cursor
fn statResetCounts(mut pCsr: *mut StatCursor) {
    unsafe {
        (*pCsr).nCell = 0 as i32;
    }
    unsafe {
        (*pCsr).nMxPayload = 0 as i32;
    }
    unsafe {
        (*pCsr).nUnused = (0 as i32) as i64;
    }
    unsafe {
        (*pCsr).nPayload = (0 as i32) as i64;
    }
    unsafe {
        (*pCsr).szPage = (0 as i32) as i64;
    }
    unsafe {
        (*pCsr).nPage = 0 as i32;
    }
}

/// Close a DBSTAT cursor.
#[unsafe(link_section = ".text.slate_distinct.dbstat.statClose")]
extern "C-unwind" fn statClose(mut pCursor: *mut sqlite3_vtab_cursor) -> i32 {
    let mut pCsr: *mut StatCursor = pCursor as *mut StatCursor;
    statResetCsr(pCsr);
    unsafe { sqlite3_finalize(unsafe { (*pCsr).pStmt }) };
    unsafe { sqlite3_free(pCsr as *mut ()) };
    return 0 as i32;
}

/// For a single cell on a btree page, compute the number of bytes of
/// content (payload) stored on that page.  That is to say, compute the
/// number of bytes of content not found on overflow pages.
///
/// # Arguments
///
/// * `nUsable` - Usable bytes per page
/// * `flags` - Page flags
/// * `nTotal` - Total record (payload) size
fn getLocalPayload(mut nUsable: i32, mut flags: u8, mut nTotal: i32) -> i32 {
    let mut nLocal: i32 = 0 as i32;
    let mut nMinLocal: i32 = 0 as i32;
    let mut nMaxLocal: i32 = 0 as i32;
    if ((flags as u32) as i32) == (13 as i32) {
        // Table leaf node
        nMinLocal = (nUsable - (12 as i32)) * (32 as i32) / (255 as i32) - (23 as i32);
        nMaxLocal = nUsable - (35 as i32);
    } else {
        // Index interior and leaf nodes
        nMinLocal = (nUsable - (12 as i32)) * (32 as i32) / (255 as i32) - (23 as i32);
        nMaxLocal = (nUsable - (12 as i32)) * (64 as i32) / (255 as i32) - (23 as i32);
    }
    nLocal = nMinLocal + (nTotal - nMinLocal) % (nUsable - (4 as i32));
    if nLocal > nMaxLocal {
        nLocal = nMinLocal;
    }
    return nLocal;
}

/// Populate the StatPage object with information about the all
/// cells found on the page currently under analysis.
fn statDecodePage(mut pBt: *mut Btree, mut p: *mut StatPage) -> i32 {
    let mut __slate_storage_625: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_625: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_625) as *mut i32;
    let mut __slate_storage_624: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_624: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_624) as *mut i32;
    let mut __slate_storage_634: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_634: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_634) as *mut i32;
    let mut __slate_storage_633: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_633: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_633) as *mut i32;
    let mut __slate_storage_426: std::mem::MaybeUninit<*mut PgHdr> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_426: *mut *mut PgHdr =
        std::ptr::addr_of_mut!(__slate_storage_426) as *mut *mut PgHdr;
    let mut __slate_storage_425: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_425: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_425) as *mut u32;
    let mut __slate_storage_424: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_424: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_424) as *mut i32;
    let mut __slate_storage_423: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_423: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_423) as *mut i32;
    let mut __slate_storage_422: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_422: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_422) as *mut i32;
    let mut __slate_storage_632: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_632: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_632) as *mut i32;
    let mut __slate_storage_631: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_631: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_631) as *mut i32;
    let mut __slate_storage_421: std::mem::MaybeUninit<u64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_421: *mut u64 = std::ptr::addr_of_mut!(__slate_storage_421) as *mut u64;
    let mut __slate_storage_630: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_630: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_630) as *mut i32;
    let mut __slate_storage_629: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_629: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_629) as *mut i32;
    let mut __slate_storage_628: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_628: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_628) as *mut i32; // Bytes of payload stored locally
    let mut __slate_storage_420: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_420: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_420) as *mut i32; // Bytes of payload total (local+overflow)
    let mut __slate_storage_419: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_419: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_419) as *mut u32;
    let mut __slate_storage_627: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_627: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_627) as *mut i32;
    let mut __slate_storage_626: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_626: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_626) as *mut i32;
    let mut __slate_storage_418: std::mem::MaybeUninit<*mut StatCell> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_418: *mut *mut StatCell =
        std::ptr::addr_of_mut!(__slate_storage_418) as *mut *mut StatCell; // Usable bytes per page
    let mut __slate_storage_417: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_417: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_417) as *mut i32; // Used to iterate through cells
    let mut __slate_storage_416: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_416: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_416) as *mut i32;
    let mut __slate_storage_623: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_623: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_623) as *mut u32;
    let mut __slate_storage_622: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_622: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_622) as *mut i32;
    let mut __slate_storage_621: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_621: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_621) as *mut i32;
    let mut __slate_storage_415: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_415: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_415) as *mut i32;
    let mut __slate_storage_620: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_620: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_620) as *mut i32;
    let mut __slate_storage_619: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_619: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_619) as *mut i32;
    let mut __slate_storage_618: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_618: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_618) as *mut i32;
    let mut __slate_storage_617: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_617: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_617) as *mut i32;
    let mut __slate_storage_414: std::mem::MaybeUninit<*mut u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_414: *mut *mut u8 =
        std::ptr::addr_of_mut!(__slate_storage_414) as *mut *mut u8;
    let mut __slate_storage_413: std::mem::MaybeUninit<*mut u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_413: *mut *mut u8 =
        std::ptr::addr_of_mut!(__slate_storage_413) as *mut *mut u8;
    let mut __slate_storage_412: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_412: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_412) as *mut i32;
    let mut __slate_storage_411: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_411: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_411) as *mut i32;
    let mut __slate_storage_410: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_410: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_410) as *mut i32;
    let mut __slate_storage_409: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_409: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_409) as *mut i32;
    let mut __slate_storage_408: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_408: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_408) as *mut i32;
    unsafe {
        '__join_0: {
            std::ptr::write(__slate_slot_413, unsafe { (*p).aPg });
            std::ptr::write(__slate_slot_414, unsafe {
                (*__slate_slot_413).offset(
                    (if (unsafe { (*p).iPgno }) == ((1 as i32) as u32) {
                        100 as i32
                    } else {
                        0 as i32
                    }) as isize,
                )
            });
            unsafe {
                (*p).flags = unsafe { *unsafe { (*__slate_slot_414).offset((0 as i32) as isize) } };
            }
            if (((unsafe { (*p).flags }) as u32) as i32) == (10 as i32)
                || (((unsafe { (*p).flags }) as u32) as i32) == (13 as i32)
            {
                *__slate_slot_411 = 1 as i32;
                *__slate_slot_410 = 8 as i32;
            } else {
                if (((unsafe { (*p).flags }) as u32) as i32) == (5 as i32)
                    || (((unsafe { (*p).flags }) as u32) as i32) == (2 as i32)
                {
                    *__slate_slot_411 = 0 as i32;
                    *__slate_slot_410 = 12 as i32;
                } else {
                    break '__join_0;
                }
            }
            if (unsafe { (*p).iPgno }) == ((1 as i32) as u32) {
                std::ptr::write(__slate_slot_617, *__slate_slot_410);
                std::ptr::write(__slate_slot_618, *__slate_slot_617 + (100 as i32));
                *__slate_slot_410 = *__slate_slot_618;
            }
            unsafe {
                (*p).nCell = (((unsafe {
                    *unsafe {
                        unsafe { (*__slate_slot_414).offset((3 as i32) as isize) }
                            .offset((0 as i32) as isize)
                    }
                }) as u32) as i32)
                    << (8 as i32)
                    | (((unsafe {
                        *unsafe {
                            unsafe { (*__slate_slot_414).offset((3 as i32) as isize) }
                                .offset((1 as i32) as isize)
                        }
                    }) as u32) as i32);
            }
            unsafe {
                (*p).nMxPayload = 0 as i32;
            }
            *__slate_slot_412 = unsafe { sqlite3BtreeGetPageSize(pBt) };
            *__slate_slot_408 = ((((unsafe {
                *unsafe {
                    unsafe { (*__slate_slot_414).offset((5 as i32) as isize) }
                        .offset((0 as i32) as isize)
                }
            }) as u32) as i32)
                << (8 as i32)
                | (((unsafe {
                    *unsafe {
                        unsafe { (*__slate_slot_414).offset((5 as i32) as isize) }
                            .offset((1 as i32) as isize)
                    }
                }) as u32) as i32))
                - *__slate_slot_410
                - (2 as i32) * unsafe { (*p).nCell };
            std::ptr::write(__slate_slot_619, *__slate_slot_408);
            std::ptr::write(
                __slate_slot_620,
                *__slate_slot_619
                    + (((unsafe { *unsafe { (*__slate_slot_414).offset((7 as i32) as isize) } })
                        as u32) as i32),
            );
            *__slate_slot_408 = *__slate_slot_620;
            *__slate_slot_409 = (((unsafe {
                *unsafe {
                    unsafe { (*__slate_slot_414).offset((1 as i32) as isize) }
                        .offset((0 as i32) as isize)
                }
            }) as u32) as i32)
                << (8 as i32)
                | (((unsafe {
                    *unsafe {
                        unsafe { (*__slate_slot_414).offset((1 as i32) as isize) }
                            .offset((1 as i32) as isize)
                    }
                }) as u32) as i32);
            loop {
                if *__slate_slot_409 != (0 as i32) {
                    if *__slate_slot_409 >= *__slate_slot_412 {
                        break '__join_0;
                    } else {
                        std::ptr::write(__slate_slot_621, *__slate_slot_408);
                        std::ptr::write(
                            __slate_slot_622,
                            *__slate_slot_621
                                + ((((unsafe {
                                    *unsafe {
                                        unsafe {
                                            (*__slate_slot_413)
                                                .offset((*__slate_slot_409 + (2 as i32)) as isize)
                                        }
                                        .offset((0 as i32) as isize)
                                    }
                                }) as u32) as i32)
                                    << (8 as i32)
                                    | (((unsafe {
                                        *unsafe {
                                            unsafe {
                                                (*__slate_slot_413).offset(
                                                    (*__slate_slot_409 + (2 as i32)) as isize,
                                                )
                                            }
                                            .offset((1 as i32) as isize)
                                        }
                                    }) as u32) as i32)),
                        );
                        *__slate_slot_408 = *__slate_slot_622;
                        *__slate_slot_415 = (((unsafe {
                            *unsafe {
                                unsafe { (*__slate_slot_413).offset(*__slate_slot_409 as isize) }
                                    .offset((0 as i32) as isize)
                            }
                        }) as u32) as i32)
                            << (8 as i32)
                            | (((unsafe {
                                *unsafe {
                                    unsafe {
                                        (*__slate_slot_413).offset(*__slate_slot_409 as isize)
                                    }
                                    .offset((1 as i32) as isize)
                                }
                            }) as u32) as i32);
                        if *__slate_slot_415 < *__slate_slot_409 + (4 as i32)
                            && *__slate_slot_415 > (0 as i32)
                        {
                            break '__join_0;
                        } else {
                            *__slate_slot_409 = *__slate_slot_415;
                        }
                    }
                } else {
                    break;
                }
            }
            unsafe {
                (*p).nUnused = *__slate_slot_408;
            }
            if *__slate_slot_411 != (0 as i32) {
                *__slate_slot_623 = (0 as i32) as u32;
            } else {
                *__slate_slot_623 = unsafe {
                    sqlite3Get4byte(
                        (unsafe { (*__slate_slot_414).offset((8 as i32) as isize) }) as *const u8,
                    )
                };
            }
            '__join_1: {
                unsafe {
                    (*p).iRightChildPg = *__slate_slot_623;
                }
                if (unsafe { (*p).nCell }) != (0 as i32) {
                    unsafe { sqlite3BtreeEnter(pBt) };
                    *__slate_slot_417 =
                        *__slate_slot_412 - unsafe { sqlite3BtreeGetReserveNoMutex(pBt) };
                    unsafe { sqlite3BtreeLeave(pBt) };
                    unsafe {
                        (*p).aCell = (unsafe {
                            sqlite3_malloc64(
                                ((((unsafe { (*p).nCell }) + (1 as i32)) as i64) as u64)
                                    .wrapping_mul(32 as u64),
                            )
                        }) as *mut StatCell;
                    }
                    if (unsafe { (*p).aCell }) == std::ptr::null_mut::<StatCell>() {
                        return 7 as i32;
                    } else {
                        unsafe {
                            memset(
                                (unsafe { (*p).aCell }) as *mut (),
                                0 as i32,
                                ((((unsafe { (*p).nCell }) + (1 as i32)) as i64) as u64)
                                    .wrapping_mul(32 as u64),
                            )
                        };
                        *__slate_slot_416 = 0 as i32;
                        '__loop_2: loop {
                            if *__slate_slot_416 < unsafe { (*p).nCell } {
                                std::ptr::write(__slate_slot_418, unsafe {
                                    unsafe { (*p).aCell }.offset(*__slate_slot_416 as isize)
                                });
                                *__slate_slot_409 = (((unsafe {
                                    *unsafe {
                                        unsafe {
                                            (*__slate_slot_413).offset(
                                                (*__slate_slot_410 + *__slate_slot_416 * (2 as i32))
                                                    as isize,
                                            )
                                        }
                                        .offset((0 as i32) as isize)
                                    }
                                }) as u32)
                                    as i32)
                                    << (8 as i32)
                                    | (((unsafe {
                                        *unsafe {
                                            unsafe {
                                                (*__slate_slot_413).offset(
                                                    (*__slate_slot_410
                                                        + *__slate_slot_416 * (2 as i32))
                                                        as isize,
                                                )
                                            }
                                            .offset((1 as i32) as isize)
                                        }
                                    }) as u32) as i32);
                                if *__slate_slot_409 < *__slate_slot_410
                                    || *__slate_slot_409 >= *__slate_slot_412
                                {
                                    break '__join_0;
                                } else {
                                    if !(*__slate_slot_411 != (0 as i32)) {
                                        unsafe {
                                            (*(*__slate_slot_418)).iChildPg = unsafe {
                                                sqlite3Get4byte(
                                                    (unsafe {
                                                        (*__slate_slot_413)
                                                            .offset(*__slate_slot_409 as isize)
                                                    })
                                                        as *const u8,
                                                )
                                            };
                                        }
                                        std::ptr::write(__slate_slot_626, *__slate_slot_409);
                                        std::ptr::write(
                                            __slate_slot_627,
                                            *__slate_slot_626 + (4 as i32),
                                        );
                                        *__slate_slot_409 = *__slate_slot_627;
                                    }
                                    '__join_3: {
                                        if (((unsafe { (*p).flags }) as u32) as i32) == (5 as i32) {
                                            // A table interior node. nPayload==0.
                                        } else {
                                            std::ptr::write(__slate_slot_628, *__slate_slot_409);
                                            if (((unsafe {
                                                *unsafe {
                                                    (*__slate_slot_413)
                                                        .offset(*__slate_slot_409 as isize)
                                                }
                                            })
                                                as u32)
                                                as i32)
                                                < (((((128 as i32) as i8) as u8) as u32) as i32)
                                            {
                                                *__slate_slot_419 = (unsafe {
                                                    *unsafe {
                                                        (*__slate_slot_413)
                                                            .offset(*__slate_slot_409 as isize)
                                                    }
                                                })
                                                    as u32;
                                                *__slate_slot_629 = 1 as i32;
                                            } else {
                                                *__slate_slot_629 = ((unsafe {
                                                    sqlite3GetVarint32(
                                                        (unsafe {
                                                            (*__slate_slot_413)
                                                                .offset(*__slate_slot_409 as isize)
                                                        })
                                                            as *const u8,
                                                        std::ptr::addr_of_mut!(*__slate_slot_419),
                                                    )
                                                })
                                                    as u32)
                                                    as i32;
                                            }
                                            std::ptr::write(
                                                __slate_slot_630,
                                                *__slate_slot_628
                                                    + ((((*__slate_slot_629 as i8) as u8) as u32)
                                                        as i32),
                                            );
                                            *__slate_slot_409 = *__slate_slot_630;
                                            if (((unsafe { (*p).flags }) as u32) as i32)
                                                == (13 as i32)
                                            {
                                                std::ptr::write(
                                                    __slate_slot_631,
                                                    *__slate_slot_409,
                                                );
                                                std::ptr::write(
                                                    __slate_slot_632,
                                                    *__slate_slot_631
                                                        + (((unsafe {
                                                            sqlite3GetVarint(
                                                                (unsafe {
                                                                    (*__slate_slot_413).offset(
                                                                        *__slate_slot_409 as isize,
                                                                    )
                                                                })
                                                                    as *const u8,
                                                                std::ptr::addr_of_mut!(
                                                                    *__slate_slot_421
                                                                ),
                                                            )
                                                        })
                                                            as u32)
                                                            as i32),
                                                );
                                                *__slate_slot_409 = *__slate_slot_632;
                                            }
                                            if *__slate_slot_419
                                                > ((unsafe { (*p).nMxPayload }) as u32)
                                            {
                                                unsafe {
                                                    (*p).nMxPayload = *__slate_slot_419 as i32;
                                                }
                                            }
                                            *__slate_slot_420 = getLocalPayload(
                                                *__slate_slot_417,
                                                unsafe { (*p).flags },
                                                *__slate_slot_419 as i32,
                                            );
                                            if *__slate_slot_420 < (0 as i32) {
                                                break '__join_0;
                                            } else {
                                                unsafe {
                                                    (*(*__slate_slot_418)).nLocal =
                                                        *__slate_slot_420;
                                                }
                                                0 as i32;
                                                0 as i32;
                                                if *__slate_slot_419 > (*__slate_slot_420 as u32) {
                                                    std::ptr::write(
                                                        __slate_slot_423,
                                                        ((*__slate_slot_419)
                                                            .wrapping_sub(*__slate_slot_420 as u32)
                                                            .wrapping_add(*__slate_slot_417 as u32)
                                                            .wrapping_sub((4 as i32) as u32)
                                                            .wrapping_sub((1 as i32) as u32)
                                                            / ((*__slate_slot_417 - (4 as i32))
                                                                as u32))
                                                            as i32,
                                                    );
                                                    if *__slate_slot_409
                                                        + *__slate_slot_420
                                                        + (4 as i32)
                                                        > *__slate_slot_417
                                                        || *__slate_slot_419
                                                            > ((2147483647 as i32) as u32)
                                                    {
                                                        break '__join_0;
                                                    } else {
                                                        unsafe {
                                                            (*(*__slate_slot_418)).nLastOvfl =
                                                                (*__slate_slot_419)
                                                                    .wrapping_sub(
                                                                        *__slate_slot_420 as u32,
                                                                    )
                                                                    .wrapping_sub(
                                                                        ((*__slate_slot_423
                                                                            - (1 as i32))
                                                                            * (*__slate_slot_417
                                                                                - (4 as i32)))
                                                                            as u32,
                                                                    )
                                                                    as i32;
                                                        }
                                                        unsafe {
                                                            (*(*__slate_slot_418)).nOvfl =
                                                                *__slate_slot_423;
                                                        }
                                                        unsafe {
                                                            (*(*__slate_slot_418)).aOvfl = (unsafe {
                                                                sqlite3_malloc64(
                                                                    (4 as u64).wrapping_mul(
                                                                        (*__slate_slot_423 as i64)
                                                                            as u64,
                                                                    ),
                                                                )
                                                            })
                                                                as *mut u32;
                                                        }
                                                        if (unsafe { (*(*__slate_slot_418)).aOvfl })
                                                            == std::ptr::null_mut::<u32>()
                                                        {
                                                            return 7 as i32;
                                                        } else {
                                                            unsafe {
                                                                *unsafe {
                                                                    unsafe {
                                                                        (*(*__slate_slot_418)).aOvfl
                                                                    }
                                                                    .offset((0 as i32) as isize)
                                                                } = unsafe {
                                                                    sqlite3Get4byte(
                                                                        (unsafe {
                                                                            (*__slate_slot_413).offset((*__slate_slot_409 + *__slate_slot_420) as isize)
                                                                        })
                                                                            as *const u8,
                                                                    )
                                                                };
                                                            }
                                                            *__slate_slot_422 = 1 as i32;
                                                            loop {
                                                                if *__slate_slot_422
                                                                    < *__slate_slot_423
                                                                {
                                                                    std::ptr::write(
                                                                        __slate_slot_425,
                                                                        unsafe {
                                                                            *unsafe {
                                                                                unsafe { (*(*__slate_slot_418)).aOvfl }.offset((*__slate_slot_422 - (1 as i32)) as isize)
                                                                            }
                                                                        },
                                                                    );
                                                                    std::ptr::write(
                                                                        __slate_slot_426,
                                                                        std::ptr::null_mut::<PgHdr>(
                                                                        ),
                                                                    );
                                                                    *__slate_slot_424 = unsafe {
                                                                        sqlite3PagerGet(
                                                                            unsafe {
                                                                                sqlite3BtreePager(
                                                                                    pBt,
                                                                                )
                                                                            },
                                                                            *__slate_slot_425,
                                                                            std::ptr::addr_of_mut!(
                                                                                *__slate_slot_426
                                                                            ),
                                                                            0 as i32,
                                                                        )
                                                                    };
                                                                    if *__slate_slot_424
                                                                        != (0 as i32)
                                                                    {
                                                                        break '__loop_2;
                                                                    } else {
                                                                        unsafe {
                                                                            *unsafe {
                                                                                unsafe { (*(*__slate_slot_418)).aOvfl }.offset(*__slate_slot_422 as isize)
                                                                            } = unsafe {
                                                                                sqlite3Get4byte((unsafe { sqlite3PagerGetData(*__slate_slot_426) }) as *const u8)
                                                                            };
                                                                        }
                                                                        unsafe {
                                                                            sqlite3PagerUnref(
                                                                                *__slate_slot_426,
                                                                            )
                                                                        };
                                                                        std::ptr::write(
                                                                            __slate_slot_633,
                                                                            *__slate_slot_422,
                                                                        );
                                                                        std::ptr::write(
                                                                            __slate_slot_634,
                                                                            *__slate_slot_633
                                                                                + (1 as i32),
                                                                        );
                                                                        *__slate_slot_422 =
                                                                            *__slate_slot_634;
                                                                    }
                                                                } else {
                                                                    break '__join_3;
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    std::ptr::write(__slate_slot_624, *__slate_slot_416);
                                    std::ptr::write(
                                        __slate_slot_625,
                                        *__slate_slot_624 + (1 as i32),
                                    );
                                    *__slate_slot_416 = *__slate_slot_625;
                                }
                            } else {
                                break '__join_1;
                            }
                        }
                        0 as i32;
                        return *__slate_slot_424;
                    }
                }
            }
            return 0 as i32;
        }
        unsafe {
            (*p).flags = ((0 as i32) as i8) as u8;
        }
        statClearCells(p);
        return 0 as i32;
    }
    return unsafe { std::mem::zeroed() };
}

/// Populate the pCsr->iOffset and pCsr->szPage member variables. Based on
/// the current value of pCsr->iPageno.
fn statSizeAndOffset(mut pCsr: *mut StatCursor) {
    let mut pTab: *mut StatTable =
        (unsafe { (*(pCsr as *mut sqlite3_vtab_cursor)).pVtab }) as *mut StatTable;
    let mut pBt: *mut Btree = unsafe {
        (*unsafe {
            unsafe { (*unsafe { (*pTab).db }).aDb }.offset((unsafe { (*pTab).iDb }) as isize)
        })
        .pBt
    };
    let mut pPager: *mut Pager = unsafe { sqlite3BtreePager(pBt) };
    let mut fd: *mut sqlite3_file = unsafe { std::mem::zeroed() };
    let mut x: __SlateAlign16<[i64; 2]> = __SlateAlign16([0 as i64; 2]);
    // If connected to a ZIPVFS backend, find the page size and
    // offset from ZIPVFS.
    fd = unsafe { sqlite3PagerFile(pPager) };
    unsafe {
        *unsafe { (x.0.as_mut_ptr() as *mut i64).offset((0 as i32) as isize) } =
            ((unsafe { (*pCsr).iPageno }) as u64) as i64;
    }
    if (unsafe { sqlite3OsFileControl(fd, 230440 as i32, std::ptr::addr_of_mut!(x.0) as *mut ()) })
        == (0 as i32)
    {
        unsafe {
            (*pCsr).iOffset =
                unsafe { *unsafe { (x.0.as_mut_ptr() as *mut i64).offset((0 as i32) as isize) } };
        }
        let __v635: *mut StatCursor = pCsr;
        let __v636: i64 = unsafe { (*__v635).szPage };
        let __v637: i64 = __v636
            + unsafe { *unsafe { (x.0.as_mut_ptr() as *mut i64).offset((1 as i32) as isize) } };
        unsafe {
            (*__v635).szPage = __v637;
        }
    } else {
        // Not ZIPVFS: The default page size and offset
        let __v638: *mut StatCursor = pCsr;
        let __v639: i64 = unsafe { (*__v638).szPage };
        let __v640: i64 = __v639 + ((unsafe { sqlite3BtreeGetPageSize(pBt) }) as i64);
        unsafe {
            (*__v638).szPage = __v640;
        }
        unsafe {
            (*pCsr).iOffset = (unsafe { (*pCsr).szPage })
                * ((unsafe { (*pCsr).iPageno }.wrapping_sub((1 as i32) as u32) as u64) as i64);
        }
    }
}

/// Load a copy of the page data for page iPg into the buffer belonging
/// to page object pPg. Allocate the buffer if necessary. Return SQLITE_OK
/// if successful, or an SQLite error code otherwise.
///
/// # Arguments
///
/// * `pBt` - Load page from this b-tree
/// * `iPg` - Page number to load
/// * `pPg` - Load page into this object
fn statGetPage(mut pBt: *mut Btree, mut iPg: u32, mut pPg: *mut StatPage) -> i32 {
    let mut pgsz: i32 = unsafe { sqlite3BtreeGetPageSize(pBt) };
    let mut pDbPage: *mut PgHdr = std::ptr::null_mut::<PgHdr>();
    let mut rc: i32 = 0 as i32;
    if (unsafe { (*pPg).aPg }) == std::ptr::null_mut::<u8>() {
        unsafe {
            (*pPg).aPg = (unsafe { sqlite3_malloc(pgsz + (256 as i32)) }) as *mut u8;
        }
        if (unsafe { (*pPg).aPg }) == std::ptr::null_mut::<u8>() {
            return 7 as i32;
        }
        unsafe {
            memset(
                (unsafe { unsafe { (*pPg).aPg }.offset(pgsz as isize) }) as *mut (),
                0 as i32,
                ((256 as i32) as i64) as u64,
            )
        };
    }
    rc = unsafe {
        sqlite3PagerGet(
            unsafe { sqlite3BtreePager(pBt) },
            iPg,
            std::ptr::addr_of_mut!(pDbPage),
            0 as i32,
        )
    };
    if rc == (0 as i32) {
        let mut a: *const u8 = (unsafe { sqlite3PagerGetData(pDbPage) }) as *const u8;
        unsafe {
            memcpy(
                (unsafe { (*pPg).aPg }) as *mut (),
                a as *const (),
                (pgsz as i64) as u64,
            )
        };
        unsafe { sqlite3PagerUnref(pDbPage) };
    }
    return rc;
}

/// Move a DBSTAT cursor to the next entry.  Normally, the next
/// entry will be the next page, but in aggregated mode (pCsr->isAgg!=0),
/// the next entry is the next btree.
#[unsafe(link_section = ".text.slate_distinct.dbstat.statNext")]
extern "C-unwind" fn statNext(mut pCursor: *mut sqlite3_vtab_cursor) -> i32 {
    let mut __slate_storage_687: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_687: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_687) as *mut i64;
    let mut __slate_storage_686: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_686: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_686) as *mut i64;
    let mut __slate_storage_685: std::mem::MaybeUninit<*mut StatCursor> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_685: *mut *mut StatCursor =
        std::ptr::addr_of_mut!(__slate_storage_685) as *mut *mut StatCursor;
    let mut __slate_storage_682: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_682: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_682) as *mut i32;
    let mut __slate_storage_681: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_681: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_681) as *mut i32;
    let mut __slate_storage_684: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_684: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_684) as *mut i32;
    let mut __slate_storage_683: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_683: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_683) as *mut i32;
    let mut __slate_storage_680: std::mem::MaybeUninit<*mut i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_680: *mut *mut i8 =
        std::ptr::addr_of_mut!(__slate_storage_680) as *mut *mut i8;
    let mut __slate_storage_679: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_679: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_679) as *mut i64;
    let mut __slate_storage_678: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_678: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_678) as *mut i64;
    let mut __slate_storage_677: std::mem::MaybeUninit<*mut StatCursor> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_677: *mut *mut StatCursor =
        std::ptr::addr_of_mut!(__slate_storage_677) as *mut *mut StatCursor;
    let mut __slate_storage_676: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_676: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_676) as *mut i32;
    let mut __slate_storage_675: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_675: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_675) as *mut i32;
    let mut __slate_storage_674: std::mem::MaybeUninit<*mut StatCursor> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_674: *mut *mut StatCursor =
        std::ptr::addr_of_mut!(__slate_storage_674) as *mut *mut StatCursor;
    let mut __slate_storage_459: std::mem::MaybeUninit<*mut StatPage> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_459: *mut *mut StatPage =
        std::ptr::addr_of_mut!(__slate_storage_459) as *mut *mut StatPage;
    let mut __slate_storage_458: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_458: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_458) as *mut i32;
    let mut __slate_storage_641: std::mem::MaybeUninit<*mut i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_641: *mut *mut i8 =
        std::ptr::addr_of_mut!(__slate_storage_641) as *mut *mut i8;
    let mut __slate_storage_453: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_453: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_453) as *mut u32;
    let mut __slate_storage_452: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_452: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_452) as *mut i32;
    let mut __slate_storage_673: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_673: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_673) as *mut i32;
    let mut __slate_storage_672: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_672: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_672) as *mut i32;
    let mut __slate_storage_671: std::mem::MaybeUninit<*mut StatPage> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_671: *mut *mut StatPage =
        std::ptr::addr_of_mut!(__slate_storage_671) as *mut *mut StatPage;
    let mut __slate_storage_670: std::mem::MaybeUninit<*mut i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_670: *mut *mut i8 =
        std::ptr::addr_of_mut!(__slate_storage_670) as *mut *mut i8;
    let mut __slate_storage_669: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_669: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_669) as *mut i32;
    let mut __slate_storage_668: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_668: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_668) as *mut i32;
    let mut __slate_storage_667: std::mem::MaybeUninit<*mut StatCursor> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_667: *mut *mut StatCursor =
        std::ptr::addr_of_mut!(__slate_storage_667) as *mut *mut StatCursor;
    let mut __slate_storage_666: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_666: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_666) as *mut i32;
    let mut __slate_storage_665: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_665: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_665) as *mut i32;
    let mut __slate_storage_664: std::mem::MaybeUninit<*mut StatCursor> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_664: *mut *mut StatCursor =
        std::ptr::addr_of_mut!(__slate_storage_664) as *mut *mut StatCursor;
    let mut __slate_storage_663: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_663: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_663) as *mut i32;
    let mut __slate_storage_662: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_662: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_662) as *mut i32;
    let mut __slate_storage_661: std::mem::MaybeUninit<*mut StatCursor> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_661: *mut *mut StatCursor =
        std::ptr::addr_of_mut!(__slate_storage_661) as *mut *mut StatCursor;
    let mut __slate_storage_660: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_660: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_660) as *mut i32;
    let mut __slate_storage_659: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_659: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_659) as *mut i32;
    let mut __slate_storage_658: std::mem::MaybeUninit<*mut StatPage> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_658: *mut *mut StatPage =
        std::ptr::addr_of_mut!(__slate_storage_658) as *mut *mut StatPage;
    let mut __slate_storage_657: std::mem::MaybeUninit<*mut i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_657: *mut *mut i8 =
        std::ptr::addr_of_mut!(__slate_storage_657) as *mut *mut i8;
    let mut __slate_storage_656: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_656: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_656) as *mut i32;
    let mut __slate_storage_655: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_655: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_655) as *mut i32;
    let mut __slate_storage_654: std::mem::MaybeUninit<*mut StatCell> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_654: *mut *mut StatCell =
        std::ptr::addr_of_mut!(__slate_storage_654) as *mut *mut StatCell;
    let mut __slate_storage_647: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_647: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_647) as *mut i64;
    let mut __slate_storage_646: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_646: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_646) as *mut i64;
    let mut __slate_storage_645: std::mem::MaybeUninit<*mut StatCursor> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_645: *mut *mut StatCursor =
        std::ptr::addr_of_mut!(__slate_storage_645) as *mut *mut StatCursor;
    let mut __slate_storage_653: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_653: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_653) as *mut i64;
    let mut __slate_storage_652: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_652: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_652) as *mut i64;
    let mut __slate_storage_651: std::mem::MaybeUninit<*mut StatCursor> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_651: *mut *mut StatCursor =
        std::ptr::addr_of_mut!(__slate_storage_651) as *mut *mut StatCursor;
    let mut __slate_storage_650: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_650: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_650) as *mut i64;
    let mut __slate_storage_649: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_649: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_649) as *mut i64;
    let mut __slate_storage_648: std::mem::MaybeUninit<*mut StatCursor> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_648: *mut *mut StatCursor =
        std::ptr::addr_of_mut!(__slate_storage_648) as *mut *mut StatCursor;
    let mut __slate_storage_644: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_644: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_644) as *mut i32;
    let mut __slate_storage_643: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_643: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_643) as *mut i32;
    let mut __slate_storage_642: std::mem::MaybeUninit<*mut StatCursor> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_642: *mut *mut StatCursor =
        std::ptr::addr_of_mut!(__slate_storage_642) as *mut *mut StatCursor;
    let mut __slate_storage_457: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_457: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_457) as *mut i32;
    let mut __slate_storage_456: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_456: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_456) as *mut i32;
    let mut __slate_storage_455: std::mem::MaybeUninit<*mut StatCell> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_455: *mut *mut StatCell =
        std::ptr::addr_of_mut!(__slate_storage_455) as *mut *mut StatCell;
    // Continue analyzing the btree previously started
    let mut __slate_storage_454: std::mem::MaybeUninit<*mut StatPage> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_454: *mut *mut StatPage =
        std::ptr::addr_of_mut!(__slate_storage_454) as *mut *mut StatPage;
    let mut __slate_storage_451: std::mem::MaybeUninit<*mut Pager> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_451: *mut *mut Pager =
        std::ptr::addr_of_mut!(__slate_storage_451) as *mut *mut Pager;
    let mut __slate_storage_450: std::mem::MaybeUninit<*mut Btree> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_450: *mut *mut Btree =
        std::ptr::addr_of_mut!(__slate_storage_450) as *mut *mut Btree;
    let mut __slate_storage_449: std::mem::MaybeUninit<*mut StatTable> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_449: *mut *mut StatTable =
        std::ptr::addr_of_mut!(__slate_storage_449) as *mut *mut StatTable;
    let mut __slate_storage_448: std::mem::MaybeUninit<*mut StatCursor> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_448: *mut *mut StatCursor =
        std::ptr::addr_of_mut!(__slate_storage_448) as *mut *mut StatCursor;
    let mut __slate_storage_447: std::mem::MaybeUninit<*mut i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_447: *mut *mut i8 =
        std::ptr::addr_of_mut!(__slate_storage_447) as *mut *mut i8;
    let mut __slate_storage_446: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_446: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_446) as *mut i32;
    let mut __slate_storage_445: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_445: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_445) as *mut i32;
    unsafe {
        std::ptr::write(__slate_slot_448, pCursor as *mut StatCursor);
        std::ptr::write(
            __slate_slot_449,
            (unsafe { (*pCursor).pVtab }) as *mut StatTable,
        );
        std::ptr::write(__slate_slot_450, unsafe {
            (*unsafe {
                unsafe { (*unsafe { (*(*__slate_slot_449)).db }).aDb }
                    .offset((unsafe { (*(*__slate_slot_448)).iDb }) as isize)
            })
            .pBt
        });
        std::ptr::write(__slate_slot_451, unsafe {
            sqlite3BtreePager(*__slate_slot_450)
        });
        unsafe { sqlite3_free((unsafe { (*(*__slate_slot_448)).zPath }) as *mut ()) };
        unsafe {
            (*(*__slate_slot_448)).zPath = std::ptr::null_mut::<i8>();
        }
        '__join_0: {
            '__loop_48: loop {
                if (unsafe { (*(*__slate_slot_448)).iPage }) < (0 as i32) {
                    // Start measuring space on the next btree
                    statResetCounts(*__slate_slot_448);
                    *__slate_slot_445 =
                        unsafe { sqlite3_step(unsafe { (*(*__slate_slot_448)).pStmt }) };
                    if *__slate_slot_445 == (100 as i32) {
                        std::ptr::write(
                            __slate_slot_453,
                            ((unsafe {
                                sqlite3_column_int64(
                                    unsafe { (*(*__slate_slot_448)).pStmt },
                                    1 as i32,
                                )
                            }) as i32) as u32,
                        );
                        unsafe {
                            sqlite3PagerPagecount(
                                *__slate_slot_451,
                                std::ptr::addr_of_mut!(*__slate_slot_452),
                            )
                        };
                        if *__slate_slot_452 == (0 as i32) {
                            unsafe {
                                (*(*__slate_slot_448)).isEof = ((1 as i32) as i8) as u8;
                            }
                            return unsafe {
                                sqlite3_reset(unsafe { (*(*__slate_slot_448)).pStmt })
                            };
                        } else {
                            *__slate_slot_445 =
                                statGetPage(*__slate_slot_450, *__slate_slot_453, unsafe {
                                    unsafe {
                                        (*(*__slate_slot_448)).aPage.as_mut_ptr() as *mut StatPage
                                    }
                                    .offset((0 as i32) as isize)
                                });
                            unsafe {
                                (*unsafe {
                                    unsafe {
                                        (*(*__slate_slot_448)).aPage.as_mut_ptr() as *mut StatPage
                                    }
                                    .offset((0 as i32) as isize)
                                })
                                .iPgno = *__slate_slot_453;
                            }
                            unsafe {
                                (*unsafe {
                                    unsafe {
                                        (*(*__slate_slot_448)).aPage.as_mut_ptr() as *mut StatPage
                                    }
                                    .offset((0 as i32) as isize)
                                })
                                .iCell = 0 as i32;
                            }
                            if !((unsafe { (*(*__slate_slot_448)).isAgg }) != (0 as u8)) {
                                std::ptr::write(__slate_slot_641, unsafe {
                                    sqlite3_mprintf((b"/\0".as_ptr() as *mut i8) as *const i8)
                                });
                                *__slate_slot_447 = *__slate_slot_641;
                                unsafe {
                                    (*unsafe {
                                        unsafe {
                                            (*(*__slate_slot_448)).aPage.as_mut_ptr()
                                                as *mut StatPage
                                        }
                                        .offset((0 as i32) as isize)
                                    })
                                    .zPath = *__slate_slot_641;
                                }
                                if *__slate_slot_447 == std::ptr::null_mut::<i8>() {
                                    *__slate_slot_445 = 7 as i32;
                                }
                            }
                            unsafe {
                                (*(*__slate_slot_448)).iPage = 0 as i32;
                            }
                            unsafe {
                                (*(*__slate_slot_448)).nPage = 1 as i32;
                            }
                        }
                    } else {
                        unsafe {
                            (*(*__slate_slot_448)).isEof = ((1 as i32) as i8) as u8;
                        }
                        return unsafe { sqlite3_reset(unsafe { (*(*__slate_slot_448)).pStmt }) };
                    }
                } else {
                    std::ptr::write(__slate_slot_454, unsafe {
                        unsafe { (*(*__slate_slot_448)).aPage.as_mut_ptr() as *mut StatPage }
                            .offset((unsafe { (*(*__slate_slot_448)).iPage }) as isize)
                    });
                    if !((unsafe { (*(*__slate_slot_448)).isAgg }) != (0 as u8)) {
                        statResetCounts(*__slate_slot_448);
                    }
                    loop {
                        if (unsafe { (*(*__slate_slot_454)).iCell })
                            < unsafe { (*(*__slate_slot_454)).nCell }
                        {
                            std::ptr::write(__slate_slot_455, unsafe {
                                unsafe { (*(*__slate_slot_454)).aCell }
                                    .offset((unsafe { (*(*__slate_slot_454)).iCell }) as isize)
                            });
                            loop {
                                if (unsafe { (*(*__slate_slot_455)).iOvfl })
                                    < unsafe { (*(*__slate_slot_455)).nOvfl }
                                {
                                    unsafe { sqlite3BtreeEnter(*__slate_slot_450) };
                                    *__slate_slot_456 =
                                        (unsafe { sqlite3BtreeGetPageSize(*__slate_slot_450) })
                                            - unsafe {
                                                sqlite3BtreeGetReserveNoMutex(*__slate_slot_450)
                                            };
                                    unsafe { sqlite3BtreeLeave(*__slate_slot_450) };
                                    std::ptr::write(__slate_slot_642, *__slate_slot_448);
                                    std::ptr::write(__slate_slot_643, unsafe {
                                        (*(*__slate_slot_642)).nPage
                                    });
                                    std::ptr::write(
                                        __slate_slot_644,
                                        *__slate_slot_643 + (1 as i32),
                                    );
                                    unsafe {
                                        (*(*__slate_slot_642)).nPage = *__slate_slot_644;
                                    }
                                    statSizeAndOffset(*__slate_slot_448);
                                    if (unsafe { (*(*__slate_slot_455)).iOvfl })
                                        < (unsafe { (*(*__slate_slot_455)).nOvfl }) - (1 as i32)
                                    {
                                        std::ptr::write(__slate_slot_645, *__slate_slot_448);
                                        std::ptr::write(__slate_slot_646, unsafe {
                                            (*(*__slate_slot_645)).nPayload
                                        });
                                        std::ptr::write(
                                            __slate_slot_647,
                                            *__slate_slot_646
                                                + ((*__slate_slot_456 - (4 as i32)) as i64),
                                        );
                                        unsafe {
                                            (*(*__slate_slot_645)).nPayload = *__slate_slot_647;
                                        }
                                    } else {
                                        std::ptr::write(__slate_slot_648, *__slate_slot_448);
                                        std::ptr::write(__slate_slot_649, unsafe {
                                            (*(*__slate_slot_648)).nPayload
                                        });
                                        std::ptr::write(
                                            __slate_slot_650,
                                            *__slate_slot_649
                                                + ((unsafe { (*(*__slate_slot_455)).nLastOvfl })
                                                    as i64),
                                        );
                                        unsafe {
                                            (*(*__slate_slot_648)).nPayload = *__slate_slot_650;
                                        }
                                        std::ptr::write(__slate_slot_651, *__slate_slot_448);
                                        std::ptr::write(__slate_slot_652, unsafe {
                                            (*(*__slate_slot_651)).nUnused
                                        });
                                        std::ptr::write(
                                            __slate_slot_653,
                                            *__slate_slot_652
                                                + ((*__slate_slot_456
                                                    - (4 as i32)
                                                    - unsafe { (*(*__slate_slot_455)).nLastOvfl })
                                                    as i64),
                                        );
                                        unsafe {
                                            (*(*__slate_slot_651)).nUnused = *__slate_slot_653;
                                        }
                                    }
                                    *__slate_slot_457 = unsafe { (*(*__slate_slot_455)).iOvfl };
                                    std::ptr::write(__slate_slot_654, *__slate_slot_455);
                                    std::ptr::write(__slate_slot_655, unsafe {
                                        (*(*__slate_slot_654)).iOvfl
                                    });
                                    std::ptr::write(
                                        __slate_slot_656,
                                        *__slate_slot_655 + (1 as i32),
                                    );
                                    unsafe {
                                        (*(*__slate_slot_654)).iOvfl = *__slate_slot_656;
                                    }
                                    if !((unsafe { (*(*__slate_slot_448)).isAgg }) != (0 as u8)) {
                                        unsafe {
                                            (*(*__slate_slot_448)).zName = (unsafe {
                                                sqlite3_column_text(
                                                    unsafe { (*(*__slate_slot_448)).pStmt },
                                                    0 as i32,
                                                )
                                            })
                                                as *mut i8;
                                        }
                                        unsafe {
                                            (*(*__slate_slot_448)).iPageno = unsafe {
                                                *unsafe {
                                                    unsafe { (*(*__slate_slot_455)).aOvfl }
                                                        .offset(*__slate_slot_457 as isize)
                                                }
                                            };
                                        }
                                        unsafe {
                                            (*(*__slate_slot_448)).zPagetype =
                                                b"overflow\0".as_ptr() as *mut i8;
                                        }
                                        std::ptr::write(__slate_slot_657, unsafe {
                                            sqlite3_mprintf(
                                                (b"%s%.3x+%.6x\0".as_ptr() as *mut i8) as *const i8,
                                                unsafe { (*(*__slate_slot_454)).zPath },
                                                unsafe { (*(*__slate_slot_454)).iCell },
                                                *__slate_slot_457,
                                            )
                                        });
                                        *__slate_slot_447 = *__slate_slot_657;
                                        unsafe {
                                            (*(*__slate_slot_448)).zPath = *__slate_slot_657;
                                        }
                                        return if *__slate_slot_447 == std::ptr::null_mut::<i8>() {
                                            7 as i32
                                        } else {
                                            0 as i32
                                        };
                                    }
                                } else {
                                    break;
                                }
                            }
                            if (unsafe { (*(*__slate_slot_454)).iRightChildPg }) != (0 as u32) {
                                break;
                            } else {
                                std::ptr::write(__slate_slot_658, *__slate_slot_454);
                                std::ptr::write(__slate_slot_659, unsafe {
                                    (*(*__slate_slot_658)).iCell
                                });
                                std::ptr::write(__slate_slot_660, *__slate_slot_659 + (1 as i32));
                                unsafe {
                                    (*(*__slate_slot_658)).iCell = *__slate_slot_660;
                                }
                            }
                        } else {
                            break;
                        }
                    }
                    if !((unsafe { (*(*__slate_slot_454)).iRightChildPg }) != (0 as u32))
                        || (unsafe { (*(*__slate_slot_454)).iCell })
                            > unsafe { (*(*__slate_slot_454)).nCell }
                    {
                        statClearPage(*__slate_slot_454);
                        std::ptr::write(__slate_slot_661, *__slate_slot_448);
                        std::ptr::write(__slate_slot_662, unsafe { (*(*__slate_slot_661)).iPage });
                        std::ptr::write(__slate_slot_663, *__slate_slot_662 - (1 as i32));
                        unsafe {
                            (*(*__slate_slot_661)).iPage = *__slate_slot_663;
                        }
                        if (unsafe { (*(*__slate_slot_448)).isAgg }) != (0 as u8)
                            && (unsafe { (*(*__slate_slot_448)).iPage }) < (0 as i32)
                        {
                            // label-statNext-done:  When computing aggregate space usage over
                            // an entire btree, this is the exit point from this function
                            return 0 as i32;
                        } else {
                            continue '__loop_48;
                        }
                    } else {
                        std::ptr::write(__slate_slot_664, *__slate_slot_448);
                        std::ptr::write(__slate_slot_665, unsafe { (*(*__slate_slot_664)).iPage });
                        std::ptr::write(__slate_slot_666, *__slate_slot_665 + (1 as i32));
                        unsafe {
                            (*(*__slate_slot_664)).iPage = *__slate_slot_666;
                        }
                        if (unsafe { (*(*__slate_slot_448)).iPage })
                            >= ((((2048 as u64) / (64 as u64)) as u32) as i32)
                        {
                            break '__loop_48;
                        } else {
                            0 as i32;
                            if (unsafe { (*(*__slate_slot_454)).iCell })
                                == unsafe { (*(*__slate_slot_454)).nCell }
                            {
                                unsafe {
                                    (*unsafe { (*__slate_slot_454).offset((1 as i32) as isize) })
                                        .iPgno = unsafe { (*(*__slate_slot_454)).iRightChildPg };
                                }
                            } else {
                                unsafe {
                                    (*unsafe { (*__slate_slot_454).offset((1 as i32) as isize) })
                                        .iPgno = unsafe {
                                        (*unsafe {
                                            unsafe { (*(*__slate_slot_454)).aCell }.offset(
                                                (unsafe { (*(*__slate_slot_454)).iCell }) as isize,
                                            )
                                        })
                                        .iChildPg
                                    };
                                }
                            }
                            *__slate_slot_445 = statGetPage(
                                *__slate_slot_450,
                                unsafe {
                                    (*unsafe { (*__slate_slot_454).offset((1 as i32) as isize) })
                                        .iPgno
                                },
                                unsafe { (*__slate_slot_454).offset((1 as i32) as isize) },
                            );
                            std::ptr::write(__slate_slot_667, *__slate_slot_448);
                            std::ptr::write(__slate_slot_668, unsafe {
                                (*(*__slate_slot_667)).nPage
                            });
                            std::ptr::write(__slate_slot_669, *__slate_slot_668 + (1 as i32));
                            unsafe {
                                (*(*__slate_slot_667)).nPage = *__slate_slot_669;
                            }
                            unsafe {
                                (*unsafe { (*__slate_slot_454).offset((1 as i32) as isize) })
                                    .iCell = 0 as i32;
                            }
                            if !((unsafe { (*(*__slate_slot_448)).isAgg }) != (0 as u8)) {
                                std::ptr::write(__slate_slot_670, unsafe {
                                    sqlite3_mprintf(
                                        (b"%s%.3x/\0".as_ptr() as *mut i8) as *const i8,
                                        unsafe { (*(*__slate_slot_454)).zPath },
                                        unsafe { (*(*__slate_slot_454)).iCell },
                                    )
                                });
                                *__slate_slot_447 = *__slate_slot_670;
                                unsafe {
                                    (*unsafe { (*__slate_slot_454).offset((1 as i32) as isize) })
                                        .zPath = *__slate_slot_670;
                                }
                                if *__slate_slot_447 == std::ptr::null_mut::<i8>() {
                                    *__slate_slot_445 = 7 as i32;
                                }
                            }
                            std::ptr::write(__slate_slot_671, *__slate_slot_454);
                            std::ptr::write(__slate_slot_672, unsafe {
                                (*(*__slate_slot_671)).iCell
                            });
                            std::ptr::write(__slate_slot_673, *__slate_slot_672 + (1 as i32));
                            unsafe {
                                (*(*__slate_slot_671)).iCell = *__slate_slot_673;
                            }
                        }
                    }
                }
                // Populate the StatCursor fields with the values to be returned
                // by the xColumn() and xRowid() methods.
                if *__slate_slot_445 == (0 as i32) {
                    std::ptr::write(__slate_slot_459, unsafe {
                        unsafe { (*(*__slate_slot_448)).aPage.as_mut_ptr() as *mut StatPage }
                            .offset((unsafe { (*(*__slate_slot_448)).iPage }) as isize)
                    });
                    unsafe {
                        (*(*__slate_slot_448)).zName = (unsafe {
                            sqlite3_column_text(unsafe { (*(*__slate_slot_448)).pStmt }, 0 as i32)
                        }) as *mut i8;
                    }
                    unsafe {
                        (*(*__slate_slot_448)).iPageno = unsafe { (*(*__slate_slot_459)).iPgno };
                    }
                    *__slate_slot_445 = statDecodePage(*__slate_slot_450, *__slate_slot_459);
                    if *__slate_slot_445 == (0 as i32) {
                        '__join_9: {
                            '__join_12: {
                                statSizeAndOffset(*__slate_slot_448);
                                let __t0: i32 =
                                    ((unsafe { (*(*__slate_slot_459)).flags }) as u32) as i32;
                                if __t0 == (5 as i32) {
                                    break '__join_12;
                                } else {
                                    if __t0 == (2 as i32) {
                                        break '__join_12;
                                    } else {
                                        if __t0 == (13 as i32) {
                                        } else {
                                            if __t0 == (10 as i32) {
                                            } else {
                                                unsafe {
                                                    (*(*__slate_slot_448)).zPagetype =
                                                        b"corrupted\0".as_ptr() as *mut i8;
                                                }
                                                break '__join_9;
                                            }
                                        }
                                    }
                                }
                                unsafe {
                                    (*(*__slate_slot_448)).zPagetype =
                                        b"leaf\0".as_ptr() as *mut i8;
                                }
                                // table leaf
                                // index leaf
                                break '__join_9;
                            }
                            unsafe {
                                (*(*__slate_slot_448)).zPagetype =
                                    b"internal\0".as_ptr() as *mut i8;
                            }
                            // table internal
                            // index internal
                        }
                        std::ptr::write(__slate_slot_674, *__slate_slot_448);
                        std::ptr::write(__slate_slot_675, unsafe { (*(*__slate_slot_674)).nCell });
                        std::ptr::write(
                            __slate_slot_676,
                            *__slate_slot_675 + unsafe { (*(*__slate_slot_459)).nCell },
                        );
                        unsafe {
                            (*(*__slate_slot_674)).nCell = *__slate_slot_676;
                        }
                        std::ptr::write(__slate_slot_677, *__slate_slot_448);
                        std::ptr::write(__slate_slot_678, unsafe {
                            (*(*__slate_slot_677)).nUnused
                        });
                        std::ptr::write(
                            __slate_slot_679,
                            *__slate_slot_678
                                + ((unsafe { (*(*__slate_slot_459)).nUnused }) as i64),
                        );
                        unsafe {
                            (*(*__slate_slot_677)).nUnused = *__slate_slot_679;
                        }
                        if (unsafe { (*(*__slate_slot_459)).nMxPayload })
                            > unsafe { (*(*__slate_slot_448)).nMxPayload }
                        {
                            unsafe {
                                (*(*__slate_slot_448)).nMxPayload =
                                    unsafe { (*(*__slate_slot_459)).nMxPayload };
                            }
                        }
                        if !((unsafe { (*(*__slate_slot_448)).isAgg }) != (0 as u8)) {
                            std::ptr::write(__slate_slot_680, unsafe {
                                sqlite3_mprintf(
                                    (b"%s\0".as_ptr() as *mut i8) as *const i8,
                                    unsafe { (*(*__slate_slot_459)).zPath },
                                )
                            });
                            *__slate_slot_447 = *__slate_slot_680;
                            unsafe {
                                (*(*__slate_slot_448)).zPath = *__slate_slot_680;
                            }
                            if *__slate_slot_447 == std::ptr::null_mut::<i8>() {
                                *__slate_slot_445 = 7 as i32;
                            }
                        }
                        *__slate_slot_446 = 0 as i32;
                        *__slate_slot_458 = 0 as i32;
                        loop {
                            if *__slate_slot_458 < unsafe { (*(*__slate_slot_459)).nCell } {
                                std::ptr::write(__slate_slot_683, *__slate_slot_446);
                                std::ptr::write(
                                    __slate_slot_684,
                                    *__slate_slot_683
                                        + unsafe {
                                            (*unsafe {
                                                unsafe { (*(*__slate_slot_459)).aCell }
                                                    .offset(*__slate_slot_458 as isize)
                                            })
                                            .nLocal
                                        },
                                );
                                *__slate_slot_446 = *__slate_slot_684;
                                std::ptr::write(__slate_slot_681, *__slate_slot_458);
                                std::ptr::write(__slate_slot_682, *__slate_slot_681 + (1 as i32));
                                *__slate_slot_458 = *__slate_slot_682;
                            } else {
                                break;
                            }
                        }
                        std::ptr::write(__slate_slot_685, *__slate_slot_448);
                        std::ptr::write(__slate_slot_686, unsafe {
                            (*(*__slate_slot_685)).nPayload
                        });
                        std::ptr::write(
                            __slate_slot_687,
                            *__slate_slot_686 + (*__slate_slot_446 as i64),
                        );
                        unsafe {
                            (*(*__slate_slot_685)).nPayload = *__slate_slot_687;
                        }
                        // If computing aggregate space usage by btree, continue with the
                        // next page.  The loop will exit via the return at label-statNext-done
                        if !((unsafe { (*(*__slate_slot_448)).isAgg }) != (0 as u8)) {
                            break '__join_0;
                        }
                    } else {
                        break '__join_0;
                    }
                } else {
                    break '__join_0;
                }
            }
            statResetCsr(*__slate_slot_448);
            return unsafe { sqlite3CorruptError(657 as i32) };
        }
        return *__slate_slot_445;
    }
    // Tail recursion
    return unsafe { std::mem::zeroed() };
}

#[unsafe(link_section = ".text.slate_distinct.dbstat.statEof")]
extern "C-unwind" fn statEof(mut pCursor: *mut sqlite3_vtab_cursor) -> i32 {
    let mut pCsr: *mut StatCursor = pCursor as *mut StatCursor;
    return ((unsafe { (*pCsr).isEof }) as u32) as i32;
}

/// Initialize a cursor according to the query plan idxNum using the
/// arguments in argv[0].  See statBestIndex() for a description of the
/// meaning of the bits in idxNum.
#[unsafe(link_section = ".text.slate_distinct.dbstat.statFilter")]
extern "C-unwind" fn statFilter(
    mut pCursor: *mut sqlite3_vtab_cursor,
    mut idxNum: i32,
    mut idxStr: *const i8,
    mut argc: i32,
    mut argv: *mut *mut sqlite3_value,
) -> i32 {
    let mut pCsr: *mut StatCursor = pCursor as *mut StatCursor;
    let mut pTab: *mut StatTable = (unsafe { (*pCursor).pVtab }) as *mut StatTable;
    let mut pSql: *mut sqlite3_str = unsafe { std::mem::zeroed() }; // Query of btrees to analyze
    let mut zSql: *mut i8 = unsafe { std::mem::zeroed() }; // String value of pSql
    let mut iArg: i32 = 0 as i32; // Count of argv[] parameters used so far
    let mut rc: i32 = 0 as i32; // Result of this operation
    let mut zName: *const i8 = std::ptr::null::<i8>(); // Only provide analysis of this table
    argc;
    idxStr;
    statResetCsr(pCsr);
    unsafe { sqlite3_finalize(unsafe { (*pCsr).pStmt }) };
    unsafe {
        (*pCsr).pStmt = std::ptr::null_mut::<sqlite3_stmt>();
    }
    if idxNum & (1 as i32) != (0 as i32) {
        // schema=? constraint is present.  Get its value
        let mut zDbase: *const i8 = unsafe { std::mem::zeroed() };
        let __v688: i32 = iArg;
        let __v689: i32 = __v688 + (1 as i32);
        iArg = __v689;
        zDbase =
            (unsafe { sqlite3_value_text(unsafe { *unsafe { argv.offset(__v688 as isize) } }) })
                as *const i8;
        unsafe {
            (*pCsr).iDb = unsafe { sqlite3FindDbName(unsafe { (*pTab).db }, zDbase) };
        }
        if (unsafe { (*pCsr).iDb }) < (0 as i32) {
            unsafe {
                (*pCsr).iDb = 0 as i32;
            }
            unsafe {
                (*pCsr).isEof = ((1 as i32) as i8) as u8;
            }
            return 0 as i32;
        }
    } else {
        unsafe {
            (*pCsr).iDb = unsafe { (*pTab).iDb };
        }
    }
    if idxNum & (2 as i32) != (0 as i32) {
        // name=? constraint is present
        let __v690: i32 = iArg;
        let __v691: i32 = __v690 + (1 as i32);
        iArg = __v691;
        zName = (unsafe { sqlite3_value_text(unsafe { *unsafe { argv.offset(__v690 as isize) } }) })
            as *const i8;
    }
    if idxNum & (4 as i32) != (0 as i32) {
        // aggregate=? constraint is present
        let __v692: i32 = iArg;
        let __v693: i32 = __v692 + (1 as i32);
        iArg = __v693;
        unsafe {
            (*pCsr).isAgg = ((unsafe {
                sqlite3_value_double(unsafe { *unsafe { argv.offset(__v692 as isize) } })
            }) != 0.0f64) as u8;
        }
    } else {
        unsafe {
            (*pCsr).isAgg = ((0 as i32) as i8) as u8;
        }
    }
    pSql = unsafe { sqlite3_str_new(unsafe { (*pTab).db }) };
    unsafe {
        sqlite3_str_appendf(pSql, (b"SELECT * FROM (SELECT 'sqlite_schema' AS name,1 AS rootpage,'table' AS type UNION ALL SELECT name,rootpage,type FROM \"%w\".sqlite_schema WHERE rootpage!=0)\0".as_ptr() as *mut i8) as *const i8, unsafe { (*unsafe { unsafe { (*unsafe { (*pTab).db }).aDb }.offset((unsafe { (*pCsr).iDb }) as isize) }).zDbSName })
    };
    if zName != std::ptr::null::<i8>() {
        unsafe {
            sqlite3_str_appendf(
                pSql,
                (b"WHERE name=%Q\0".as_ptr() as *mut i8) as *const i8,
                zName,
            )
        };
    }
    if idxNum & (8 as i32) != (0 as i32) {
        unsafe {
            sqlite3_str_appendf(pSql, (b" ORDER BY name\0".as_ptr() as *mut i8) as *const i8)
        };
    }
    zSql = unsafe { sqlite3_str_finish(pSql) };
    if zSql == std::ptr::null_mut::<i8>() {
        return 7 as i32;
    } else {
        rc = unsafe {
            sqlite3_prepare_v2(
                unsafe { (*pTab).db },
                zSql as *const i8,
                -(1 as i32),
                unsafe { std::ptr::addr_of_mut!((*pCsr).pStmt) },
                std::ptr::null_mut::<*const i8>(),
            )
        };
        unsafe { sqlite3_free(zSql as *mut ()) };
    }
    if rc == (0 as i32) {
        unsafe {
            (*pCsr).iPage = -(1 as i32);
        }
        rc = statNext(pCursor);
    }
    return rc;
}

#[unsafe(link_section = ".text.slate_distinct.dbstat.statColumn")]
extern "C-unwind" fn statColumn(
    mut pCursor: *mut sqlite3_vtab_cursor,
    mut ctx: *mut sqlite3_context,
    mut i: i32,
) -> i32 {
    let mut pCsr: *mut StatCursor = pCursor as *mut StatCursor;
    '__slate_break_588: {
        match i {
            0 => {
                unsafe {
                    sqlite3_result_text(
                        ctx,
                        (unsafe { (*pCsr).zName }) as *const i8,
                        -(1 as i32),
                        unsafe {
                            std::mem::transmute::<usize, Option<unsafe extern "C-unwind" fn(*mut ())>>(
                                -(1 as i32) as usize,
                            )
                        },
                    )
                }; // name
            }
            1 => {
                if !((unsafe { (*pCsr).isAgg }) != (0 as u8)) {
                    unsafe {
                        sqlite3_result_text(
                            ctx,
                            (unsafe { (*pCsr).zPath }) as *const i8,
                            -(1 as i32),
                            unsafe {
                                std::mem::transmute::<
                                    usize,
                                    Option<unsafe extern "C-unwind" fn(*mut ())>,
                                >(-(1 as i32) as usize)
                            },
                        )
                    };
                }
                // path
            }
            2 => {
                if (unsafe { (*pCsr).isAgg }) != (0 as u8) {
                    unsafe { sqlite3_result_int64(ctx, (unsafe { (*pCsr).nPage }) as i64) };
                } else {
                    unsafe {
                        sqlite3_result_int64(ctx, ((unsafe { (*pCsr).iPageno }) as u64) as i64)
                    };
                }
                // pageno
            }
            3 => {
                if !((unsafe { (*pCsr).isAgg }) != (0 as u8)) {
                    unsafe {
                        sqlite3_result_text(
                            ctx,
                            (unsafe { (*pCsr).zPagetype }) as *const i8,
                            -(1 as i32),
                            None,
                        )
                    };
                }
                // pagetype
            }
            4 => {
                unsafe { sqlite3_result_int64(ctx, (unsafe { (*pCsr).nCell }) as i64) }; // ncell
            }
            5 => {
                unsafe { sqlite3_result_int64(ctx, unsafe { (*pCsr).nPayload }) }; // payload
            }
            6 => {
                unsafe { sqlite3_result_int64(ctx, unsafe { (*pCsr).nUnused }) }; // unused
            }
            7 => {
                unsafe { sqlite3_result_int64(ctx, (unsafe { (*pCsr).nMxPayload }) as i64) }; // mx_payload
            }
            8 => {
                if !((unsafe { (*pCsr).isAgg }) != (0 as u8)) {
                    unsafe { sqlite3_result_int64(ctx, unsafe { (*pCsr).iOffset }) };
                }
                // pgoffset
            }
            9 => {
                unsafe { sqlite3_result_int64(ctx, unsafe { (*pCsr).szPage }) }; // pgsize
            }
            10 => {
                // schema
                let mut db: *mut sqlite3 = unsafe { sqlite3_context_db_handle(ctx) };
                let mut iDb: i32 = unsafe { (*pCsr).iDb };
                unsafe {
                    sqlite3_result_text(
                        ctx,
                        (unsafe {
                            (*unsafe { unsafe { (*db).aDb }.offset(iDb as isize) }).zDbSName
                        }) as *const i8,
                        -(1 as i32),
                        None,
                    )
                };
            }
            _ => {
                // aggregate
                unsafe { sqlite3_result_int(ctx, ((unsafe { (*pCsr).isAgg }) as u32) as i32) };
            }
        }
    }
    return 0 as i32;
}

#[unsafe(link_section = ".text.slate_distinct.dbstat.statRowid")]
extern "C-unwind" fn statRowid(mut pCursor: *mut sqlite3_vtab_cursor, mut pRowid: *mut i64) -> i32 {
    let mut pCsr: *mut StatCursor = pCursor as *mut StatCursor;
    unsafe {
        *pRowid = ((unsafe { (*pCsr).iPageno }) as u64) as i64;
    }
    return 0 as i32;
}

/// Invoke this routine to register the "dbstat" virtual table module
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.dbstat.sqlite3DbstatRegister")]
extern "C-unwind" fn sqlite3DbstatRegister(mut db: *mut sqlite3) -> i32 {
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
            (b"dbstat\0".as_ptr() as *mut i8) as *const i8,
            (unsafe { std::ptr::addr_of_mut!(dbstat_module) }) as *const sqlite3_module,
            std::ptr::null_mut::<()>(),
        )
    };
}

static mut dbstat_module: sqlite3_module = sqlite3_module {
    iVersion: 0 as i32,
    xCreate: Some(statConnect),
    xConnect: Some(statConnect),
    xBestIndex: Some(statBestIndex),
    xDisconnect: Some(statDisconnect),
    xDestroy: Some(statDisconnect),
    xOpen: Some(statOpen),
    xClose: Some(statClose),
    xFilter: Some(statFilter),
    xNext: Some(statNext),
    xEof: Some(statEof),
    xColumn: Some(statColumn),
    xRowid: Some(statRowid),
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
