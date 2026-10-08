//! 2005 May 25
//!
//! The author disclaims copyright to this source code.  In place of
//! a legal notice, here is a blessing:
//!
//!    May you do good and not evil.
//!    May you find forgiveness for yourself and forgive others.
//!    May you share freely, never taking more than you give.
//!
//!
//! This file contains the implementation of the sqlite3_prepare()
//! interface, and routines that contribute to loading the database schema
//! from disk.
unsafe extern "C" {
    static mut sqlite3StdType: [*const i8; 0];
    static mut sqlite3UpperToLower: [u8; 0];
    static mut sqlite3Config: Sqlite3Config;
    fn sqlite3_exec(
        __v585: *mut sqlite3,
        sql: *const i8,
        callback: Option<
            unsafe extern "C-unwind" fn(*mut (), i32, *mut *mut i8, *mut *mut i8) -> i32,
        >,
        __v588: *mut (),
        errmsg: *mut *mut i8,
    ) -> i32;
    fn sqlite3_errmsg(__v590: *mut sqlite3) -> *const i8;
    fn sqlite3_sql(pStmt: *mut sqlite3_stmt) -> *const i8;
    fn sqlite3_stmt_isexplain(pStmt: *mut sqlite3_stmt) -> i32;
    fn sqlite3_finalize(pStmt: *mut sqlite3_stmt) -> i32;
    fn sqlite3_mutex_enter(__v626: *mut sqlite3_mutex);
    fn sqlite3_mutex_leave(__v627: *mut sqlite3_mutex);
    fn memset(__s: *mut (), __c: i32, __n: u64) -> *mut ();
    fn sqlite3BtreeSetCacheSize(__v631: *mut Btree, __v632: i32) -> i32;
    fn sqlite3BtreeLastPage(__v633: *mut Btree) -> u32;
    fn sqlite3BtreeBeginTrans(__v634: *mut Btree, __v635: i32, __v636: *mut i32) -> i32;
    fn sqlite3BtreeCommit(__v637: *mut Btree) -> i32;
    fn sqlite3BtreeTxnState(__v638: *mut Btree) -> i32;
    fn sqlite3BtreeSchemaLocked(pBtree: *mut Btree) -> i32;
    fn sqlite3BtreeGetMeta(pBtree: *mut Btree, idx: i32, pValue: *mut u32);
    fn sqlite3BtreeEnter(__v643: *mut Btree);
    fn sqlite3BtreeEnterAll(__v644: *mut sqlite3);
    fn sqlite3BtreeLeave(__v645: *mut Btree);
    fn sqlite3BtreeLeaveAll(__v646: *mut sqlite3);
    fn sqlite3VdbeFinalize(__v647: *mut Vdbe) -> i32;
    fn sqlite3VdbeResetStepResult(__v648: *mut Vdbe);
    fn sqlite3VdbeDb(__v649: *mut Vdbe) -> *mut sqlite3;
    fn sqlite3VdbePrepareFlags(__v650: *mut Vdbe) -> u8;
    fn sqlite3VdbeSetSql(__v651: *mut Vdbe, z: *const i8, n: i32, __v654: u8);
    fn sqlite3VdbeSwap(__v655: *mut Vdbe, __v656: *mut Vdbe);
    fn sqlite3CorruptError(__v657: i32) -> i32;
    fn sqlite3MisuseError(__v658: i32) -> i32;
    fn sqlite3DbMallocRaw(__v659: *mut sqlite3, __v660: u64) -> *mut ();
    fn sqlite3DbStrNDup(__v661: *mut sqlite3, __v662: *const i8, __v663: u64) -> *mut i8;
    fn sqlite3DbFree(__v664: *mut sqlite3, __v665: *mut ());
    fn sqlite3DbNNFreeNN(__v666: *mut sqlite3, __v667: *mut ());
    fn sqlite3MPrintf(__v668: *mut sqlite3, __v669: *const i8, ...) -> *mut i8;
    fn sqlite3SetString(__v670: *mut *mut i8, __v671: *mut sqlite3, __v672: *const i8);
    fn sqlite3ErrorMsg(__v673: *mut Parse, __v674: *const i8, ...);
    fn sqlite3RunParser(__v675: *mut Parse, __v676: *const i8) -> i32;
    fn sqlite3ExprListDelete(__v677: *mut sqlite3, __v678: *mut ExprList);
    fn sqlite3ResetAllSchemasOfConnection(__v690: *mut sqlite3);
    fn sqlite3ResetOneSchema(__v691: *mut sqlite3, __v692: i32);
    fn sqlite3CommitInternalChanges(__v693: *mut sqlite3);
    fn sqlite3FaultSim(__v694: i32) -> i32;
    fn sqlite3FindIndex(__v695: *mut sqlite3, __v696: *const i8, __v697: *const i8) -> *mut Index;
    fn sqlite3SafetyCheckOk(__v698: *mut sqlite3) -> i32;
    fn sqlite3GetUInt32(__v699: *const i8, __v700: *mut u32) -> i32;
    fn sqlite3Utf16ByteLen(pData: *const (), nByte: i32, nChar: i32) -> i32;
    fn sqlite3Utf8CharLen(pData: *const i8, nByte: i32) -> i32;
    fn sqlite3ErrorWithMsg(__v706: *mut sqlite3, __v707: i32, __v708: *const i8, ...);
    fn sqlite3Error(__v709: *mut sqlite3, __v710: i32);
    fn sqlite3ErrorClear(__v711: *mut sqlite3);
    fn sqlite3ErrStr(__v712: i32) -> *const i8;
    fn sqlite3SetTextEncoding(db: *mut sqlite3, __v715: u8);
    fn sqlite3AbsInt32(__v716: i32) -> i32;
    fn sqlite3Utf16to8(__v717: *mut sqlite3, __v718: *const (), __v719: i32, __v720: u8)
    -> *mut i8;
    fn sqlite3AnalysisLoad(__v721: *mut sqlite3, iDB: i32) -> i32;
    fn sqlite3OomFault(__v725: *mut sqlite3) -> *mut ();
    fn sqlite3ApiExit(db: *mut sqlite3, __v727: i32) -> i32;
    fn sqlite3VtabUnlockList(__v728: *mut sqlite3);
    fn sqlite3TransferBindings(__v729: *mut sqlite3_stmt, __v730: *mut sqlite3_stmt) -> i32;
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
struct sqlite3_mem_methods {
    xMalloc: Option<unsafe extern "C-unwind" fn(i32) -> *mut ()>,
    xFree: Option<unsafe extern "C-unwind" fn(*mut ())>,
    xRealloc: Option<unsafe extern "C-unwind" fn(*mut (), i32) -> *mut ()>,
    xSize: Option<unsafe extern "C-unwind" fn(*mut ()) -> i32>,
    xRoundup: Option<unsafe extern "C-unwind" fn(i32) -> i32>,
    xInit: Option<unsafe extern "C-unwind" fn(*mut ()) -> i32>,
    xShutdown: Option<unsafe extern "C-unwind" fn(*mut ())>,
    pAppData: *mut (),
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
struct sqlite3_mutex_methods {
    xMutexInit: Option<unsafe extern "C-unwind" fn() -> i32>,
    xMutexEnd: Option<unsafe extern "C-unwind" fn() -> i32>,
    xMutexAlloc: Option<unsafe extern "C-unwind" fn(i32) -> *mut sqlite3_mutex>,
    xMutexFree: Option<unsafe extern "C-unwind" fn(*mut sqlite3_mutex)>,
    xMutexEnter: Option<unsafe extern "C-unwind" fn(*mut sqlite3_mutex)>,
    xMutexTry: Option<unsafe extern "C-unwind" fn(*mut sqlite3_mutex) -> i32>,
    xMutexLeave: Option<unsafe extern "C-unwind" fn(*mut sqlite3_mutex)>,
    xMutexHeld: Option<unsafe extern "C-unwind" fn(*mut sqlite3_mutex) -> i32>,
    xMutexNotheld: Option<unsafe extern "C-unwind" fn(*mut sqlite3_mutex) -> i32>,
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
struct sqlite3_pcache_page {
    pBuf: *mut (),
    pExtra: *mut (),
}

#[repr(C)]
#[derive(Clone, Copy)]
struct sqlite3_pcache {}

#[repr(C)]
#[derive(Clone, Copy)]
struct sqlite3_pcache_methods2 {
    iVersion: i32,
    pArg: *mut (),
    xInit: Option<unsafe extern "C-unwind" fn(*mut ()) -> i32>,
    xShutdown: Option<unsafe extern "C-unwind" fn(*mut ())>,
    xCreate: Option<unsafe extern "C-unwind" fn(i32, i32, i32) -> *mut sqlite3_pcache>,
    xCachesize: Option<unsafe extern "C-unwind" fn(*mut sqlite3_pcache, i32)>,
    xPagecount: Option<unsafe extern "C-unwind" fn(*mut sqlite3_pcache) -> i32>,
    xFetch: Option<
        unsafe extern "C-unwind" fn(*mut sqlite3_pcache, u32, i32) -> *mut sqlite3_pcache_page,
    >,
    xUnpin: Option<unsafe extern "C-unwind" fn(*mut sqlite3_pcache, *mut sqlite3_pcache_page, i32)>,
    xRekey: Option<
        unsafe extern "C-unwind" fn(*mut sqlite3_pcache, *mut sqlite3_pcache_page, u32, u32),
    >,
    xTruncate: Option<unsafe extern "C-unwind" fn(*mut sqlite3_pcache, u32)>,
    xDestroy: Option<unsafe extern "C-unwind" fn(*mut sqlite3_pcache)>,
    xShrink: Option<unsafe extern "C-unwind" fn(*mut sqlite3_pcache)>,
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
    trace: __SlateRecord166,
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
    u1: __SlateRecord167,
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
    u: __SlateRecord168,
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
    __slate_bits_0: __slate_bits::__SlateBits73U0,
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
    u: __SlateRecord169,
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
    __slate_bits_0: __slate_bits::__SlateBits97U0,
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
    u: __SlateRecord177,
    pLeft: *mut Expr,
    pRight: *mut Expr,
    x: __SlateRecord178,
    nHeight: i32,
    iTable: i32,
    iColumn: i16,
    iAgg: i16,
    w: __SlateRecord179,
    pAggInfo: *mut AggInfo,
    y: __SlateRecord180,
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
    fg: __SlateRecord187,
    iCursor: i32,
    colUsed: u64,
    u1: __SlateRecord188,
    u2: __SlateRecord189,
    u3: __SlateRecord190,
    u4: __SlateRecord191,
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
    __slate_bits_0: __slate_bits::__SlateBits109U0,
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
    u1: __SlateRecord193,
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
struct __SlateRecord196 {
    db: *mut sqlite3,
    pzErrMsg: *mut *mut i8,
    iDb: i32,
    rc: i32,
    mInitFlags: u32,
    nInitRow: u32,
    mxPage: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct Sqlite3Config {
    bMemstat: i32,
    bCoreMutex: u8,
    bFullMutex: u8,
    bOpenUri: u8,
    bUseCis: u8,
    bSmallMalloc: u8,
    bExtraSchemaChecks: u8,
    mxStrlen: i32,
    neverCorrupt: i32,
    szLookaside: i32,
    nLookaside: i32,
    nStmtSpill: i32,
    m: sqlite3_mem_methods,
    mutex: sqlite3_mutex_methods,
    pcache2: sqlite3_pcache_methods2,
    pHeap: *mut (),
    nHeap: i32,
    mnReq: i32,
    mxReq: i32,
    szMmap: i64,
    mxMmap: i64,
    pPage: *mut (),
    szPage: i32,
    nPage: i32,
    mxParserStack: i32,
    sharedCacheEnabled: i32,
    szPma: u32,
    isInit: i32,
    inProgress: i32,
    isMutexInit: i32,
    isMallocInit: i32,
    isPCacheInit: i32,
    nRefInitMutex: i32,
    pInitMutex: *mut sqlite3_mutex,
    xLog: Option<unsafe extern "C-unwind" fn(*mut (), i32, *const i8)>,
    pLogArg: *mut (),
    mxMemdbSize: i64,
    xTestCallback: Option<unsafe extern "C-unwind" fn(i32) -> i32>,
    bLocaltimeFault: i32,
    xAltLocaltime: Option<unsafe extern "C-unwind" fn(*const (), *mut ()) -> i32>,
    iOnceResetThreshold: i32,
    szSorterRef: u32,
    iPrngSeed: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct Btree {}

#[repr(C)]
#[derive(Clone, Copy)]
struct Vdbe {}

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
    __slate_bits_0: __slate_bits::__SlateBits165U0,
    azInit: *mut *const i8,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord166 {
    xLegacy: Option<unsafe extern "C-unwind" fn(*mut (), *const i8)>,
    xV2: Option<unsafe extern "C-unwind" fn(u32, *mut (), *mut (), *mut ()) -> i32>,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord167 {
    isInterrupted: i32,
    notUsed1: f64,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord168 {
    pHash: *mut FuncDef,
    pDestructor: *mut FuncDestructor,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord169 {
    tab: __SlateRecord170,
    view: __SlateRecord171,
    vtab: __SlateRecord172,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord170 {
    addColOffset: i32,
    pFKey: *mut FKey,
    pDfltList: *mut ExprList,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord171 {
    pSelect: *mut Select,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord172 {
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
union __SlateRecord177 {
    zToken: *mut i8,
    iValue: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord178 {
    pList: *mut ExprList,
    pSelect: *mut Select,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord179 {
    iJoin: i32,
    iOfst: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord180 {
    pTab: *mut Table,
    pWin: *mut Window,
    nReg: i32,
    sub: __SlateRecord181,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord181 {
    iAddr: i32,
    regReturn: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct ExprList_item {
    pExpr: *mut Expr,
    zEName: *mut i8,
    fg: __SlateRecord183,
    u: __SlateRecord184,
}

#[repr(C, align(4))]
#[derive(Clone, Copy)]
struct __SlateRecord183 {
    sortFlags: u8,
    __slate_bits_0: __slate_bits::__SlateBits183U0,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord184 {
    x: __SlateRecord185,
    iConstExprReg: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord185 {
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
struct __SlateRecord187 {
    jointype: u8,
    __slate_bits_0: __slate_bits::__SlateBits187U0,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord188 {
    zIndexedBy: *mut i8,
    pFuncArg: *mut ExprList,
    nRow: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord189 {
    pIBIndex: *mut Index,
    pCteUse: *mut CteUse,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord190 {
    pOn: *mut Expr,
    pUsing: *mut IdList,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord191 {
    pSchema: *mut Schema,
    zDatabase: *mut i8,
    pSubq: *mut Subquery,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord193 {
    cr: __SlateRecord194,
    d: __SlateRecord195,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord194 {
    addrCrTab: i32,
    regRowid: i32,
    regRoot: i32,
    constraintName: Token,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord195 {
    pReturning: *mut Returning,
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
    pub struct __SlateBits73U0 {
        #[bits(4)]
        pub notNull: u32,
        #[bits(4)]
        pub eCType: u32,
    }
    #[bitfields::bitfield([u8; 2], c_names = true, new = false, from_into_bits = false, from_traits = false, default = false, debug = false, builder = false, bit_ops = false)]
    pub struct __SlateBits183U0 {
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
    pub struct __SlateBits187U0 {
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
    pub struct __SlateBits97U0 {
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
    pub struct __SlateBits165U0 {
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
    pub struct __SlateBits109U0 {
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

/// Fill the InitData structure with an error message that indicates
/// that the database is corrupt.
///
/// # Arguments
///
/// * `pData` - Initialization context
/// * `azObj` - Type and name of object being parsed
/// * `zExtra` - Error information
fn corruptSchema(mut pData: *mut __SlateRecord196, mut azObj: *mut *mut i8, mut zExtra: *const i8) {
    let mut db: *mut sqlite3 = unsafe { (*pData).db };
    if (unsafe { (*db).mallocFailed }) != (0 as u8) {
        unsafe {
            (*pData).rc = 7 as i32;
        }
    } else {
        if (unsafe { *unsafe { unsafe { (*pData).pzErrMsg }.offset((0 as i32) as isize) } })
            != std::ptr::null_mut::<i8>()
        {
            // A error message has already been generated.  Do not overwrite it
        } else {
            if (unsafe { (*pData).mInitFlags }) & ((7 as i32) as u32) != (0 as u32) {
                unsafe {
                    *unsafe { (*pData).pzErrMsg } = unsafe {
                        sqlite3MPrintf(
                            db,
                            (b"error in %s %s after %s: %s\0".as_ptr() as *mut i8) as *const i8,
                            unsafe { *unsafe { azObj.offset((0 as i32) as isize) } },
                            unsafe { *unsafe { azObj.offset((1 as i32) as isize) } },
                            unsafe {
                                *unsafe {
                                    unsafe {
                                        std::ptr::addr_of_mut!(azAlterType.0) as *mut *const i8
                                    }
                                    .offset(
                                        ((unsafe { (*pData).mInitFlags }) & ((7 as i32) as u32))
                                            .wrapping_sub((1 as i32) as u32)
                                            as isize,
                                    )
                                }
                            },
                            zExtra,
                        )
                    };
                }
                unsafe {
                    (*pData).rc = 1 as i32;
                }
            } else {
                if (unsafe { (*db).flags }) & (((1 as i32) as i64) as u64) != (0 as u64) {
                    unsafe {
                        (*pData).rc = unsafe { sqlite3CorruptError(46 as i32) };
                    }
                } else {
                    let mut z: *mut i8 = unsafe { std::mem::zeroed() };
                    let mut zObj: *const i8 =
                        (if (unsafe { *unsafe { azObj.offset((1 as i32) as isize) } })
                            != std::ptr::null_mut::<i8>()
                        {
                            unsafe { *unsafe { azObj.offset((1 as i32) as isize) } }
                        } else {
                            b"?\0".as_ptr() as *mut i8
                        }) as *const i8;
                    z = unsafe {
                        sqlite3MPrintf(
                            db,
                            (b"malformed database schema (%s)\0".as_ptr() as *mut i8) as *const i8,
                            zObj,
                        )
                    };
                    if zExtra != std::ptr::null::<i8>()
                        && (unsafe { *unsafe { zExtra.offset((0 as i32) as isize) } }) != (0 as i8)
                    {
                        z = unsafe {
                            sqlite3MPrintf(
                                db,
                                (b"%z - %s\0".as_ptr() as *mut i8) as *const i8,
                                z,
                                zExtra,
                            )
                        };
                    }
                    unsafe {
                        *unsafe { (*pData).pzErrMsg } = z;
                    }
                    unsafe {
                        (*pData).rc = unsafe { sqlite3CorruptError(53 as i32) };
                    }
                }
            }
        }
    }
}

static mut azAlterType: __SlateAlign16<[*const i8; 4]> = __SlateAlign16([
    (b"rename\0".as_ptr() as *mut i8) as *const i8,
    (b"drop column\0".as_ptr() as *mut i8) as *const i8,
    (b"add column\0".as_ptr() as *mut i8) as *const i8,
    (b"drop constraint\0".as_ptr() as *mut i8) as *const i8,
]);

/// Check to see if any sibling index (another index on the same table)
/// of pIndex has the same root page number, and if it does, return true.
/// This would indicate a corrupt schema.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3IndexHasDuplicateRootPage(mut pIndex: *mut Index) -> i32 {
    let mut p: *mut Index = unsafe { std::mem::zeroed() };
    p = unsafe { (*unsafe { (*pIndex).pTable }).pIndex };
    '__slate_break_746: while p != std::ptr::null_mut::<Index>() {
        if (unsafe { (*p).tnum }) == unsafe { (*pIndex).tnum } && p != pIndex {
            return 1 as i32;
        }
        p = unsafe { (*p).pNext };
    }
    return 0 as i32;
}

/// This is the callback routine for the code that initializes the
/// database.  See sqlite3Init() below for additional information.
/// This routine is also called from the OP_ParseSchema opcode of the VDBE.
///
/// Each callback contains the following information:
///
///     argv[0] = type of object: "table", "index", "trigger", or "view".
///     argv[1] = name of thing being created
///     argv[2] = associated table if an index or trigger
///     argv[3] = root page number for table or index. 0 for trigger or view.
///     argv[4] = SQL text for the CREATE statement.
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.prepare.sqlite3InitCallback")]
extern "C-unwind" fn sqlite3InitCallback(
    mut pInit: *mut (),
    mut argc: i32,
    mut argv: *mut *mut i8,
    mut NotUsed: *mut *mut i8,
) -> i32 {
    let mut pData: *mut __SlateRecord196 = pInit as *mut __SlateRecord196;
    let mut db: *mut sqlite3 = unsafe { (*pData).db };
    let mut iDb: i32 = unsafe { (*pData).iDb };
    0 as i32;
    NotUsed;
    argc;
    0 as i32;
    let __v783: *mut sqlite3 = db;
    let __v784: u32 = unsafe { (*__v783).mDbFlags };
    let __v785: u32 = __v784 | ((64 as i32) as u32);
    unsafe {
        (*__v783).mDbFlags = __v785;
    }
    if argv == std::ptr::null_mut::<*mut i8>() {
        return 0 as i32;
    }
    // Might happen if EMPTY_RESULT_CALLBACKS are on
    let __v786: *mut __SlateRecord196 = pData;
    let __v787: u32 = unsafe { (*__v786).nInitRow };
    let __v788: u32 = __v787.wrapping_add((1 as i32) as u32);
    unsafe {
        (*__v786).nInitRow = __v788;
    }
    if (unsafe { (*db).mallocFailed }) != (0 as u8) {
        corruptSchema(pData, argv, std::ptr::null::<i8>());
        return 1 as i32;
    }
    0 as i32;
    0 as i32;
    if (unsafe { *unsafe { argv.offset((3 as i32) as isize) } }) == std::ptr::null_mut::<i8>() {
        corruptSchema(pData, argv, std::ptr::null::<i8>());
    } else {
        if (unsafe { *unsafe { argv.offset((4 as i32) as isize) } }) != std::ptr::null_mut::<i8>()
            && (99 as i32)
                == (((unsafe {
                    *unsafe {
                        unsafe { std::ptr::addr_of!(sqlite3UpperToLower) as *const u8 }.offset(
                            ((((unsafe {
                                *unsafe {
                                    unsafe { *unsafe { argv.offset((4 as i32) as isize) } }
                                        .offset((0 as i32) as isize)
                                }
                            }) as u8) as u32) as i32) as isize,
                        )
                    }
                }) as u32) as i32)
            && (114 as i32)
                == (((unsafe {
                    *unsafe {
                        unsafe { std::ptr::addr_of!(sqlite3UpperToLower) as *const u8 }.offset(
                            ((((unsafe {
                                *unsafe {
                                    unsafe { *unsafe { argv.offset((4 as i32) as isize) } }
                                        .offset((1 as i32) as isize)
                                }
                            }) as u8) as u32) as i32) as isize,
                        )
                    }
                }) as u32) as i32)
        {
            // Call the parser to process a CREATE TABLE, INDEX or VIEW.
            // But because db->init.busy is set to 1, no VDBE code is generated
            // or executed.  All the parser does is build the internal data
            // structures that describe the table, index, or view.
            //
            // No other valid SQL statement, other than the variable CREATE statements,
            // can begin with the letters "C" and "R".  Thus, it is not possible run
            // any other kind of statement while parsing the schema, even a corrupt
            // schema.
            let mut rc: i32 = 0 as i32;
            let mut saved_iDb: u8 = unsafe { (*db).init.iDb };
            let mut pStmt: *mut sqlite3_stmt = unsafe { std::mem::zeroed() };
            {}
            // Return code from sqlite3_prepare()
            0 as i32;
            unsafe {
                (*db).init.iDb = (iDb as i8) as u8;
            }
            if (unsafe {
                sqlite3GetUInt32(
                    (unsafe { *unsafe { argv.offset((3 as i32) as isize) } }) as *const i8,
                    unsafe { std::ptr::addr_of_mut!((*db).init.newTnum) },
                )
            }) == (0 as i32)
                || (unsafe { (*db).init.newTnum }) > unsafe { (*pData).mxPage }
                    && (unsafe { (*pData).mxPage }) > ((0 as i32) as u32)
            {
                if (unsafe { sqlite3Config.bExtraSchemaChecks }) != (0 as u8) {
                    corruptSchema(
                        pData,
                        argv,
                        (b"invalid rootpage\0".as_ptr() as *mut i8) as *const i8,
                    );
                }
            }
            unsafe {
                (*db)
                    .init
                    .__slate_bits_0
                    .__set_orphanTrigger((0 as i32) as u32);
            }
            unsafe {
                (*db).init.azInit = argv as *mut *const i8;
            }
            pStmt = std::ptr::null_mut::<sqlite3_stmt>();
            sqlite3Prepare(
                db,
                (unsafe { *unsafe { argv.offset((4 as i32) as isize) } }) as *const i8,
                -(1 as i32),
                (0 as i32) as u32,
                std::ptr::null_mut::<Vdbe>(),
                std::ptr::addr_of_mut!(pStmt),
                std::ptr::null_mut::<*const i8>(),
            );
            rc = unsafe { (*db).errCode };
            0 as i32;
            unsafe {
                (*db).init.iDb = saved_iDb;
            }
            // assert( saved_iDb==0 || (db->mDbFlags & DBFLAG_Vacuum)!=0 );
            if (0 as i32) != rc {
                if ((unsafe { (*db).init.__slate_bits_0.__get_orphanTrigger() }) as i32)
                    != (0 as i32)
                {
                    0 as i32;
                } else {
                    if rc > unsafe { (*pData).rc } {
                        unsafe {
                            (*pData).rc = rc;
                        }
                    }
                    if rc == (7 as i32) {
                        unsafe { sqlite3OomFault(db) };
                    } else {
                        if rc != (9 as i32) && rc & (255 as i32) != (6 as i32) {
                            corruptSchema(pData, argv, unsafe { sqlite3_errmsg(db) });
                        }
                    }
                }
            }
            unsafe {
                (*db).init.azInit =
                    unsafe { std::ptr::addr_of_mut!(sqlite3StdType) as *mut *const i8 };
            }
            // Any array of string ptrs will do
            unsafe { sqlite3_finalize(pStmt) };
        } else {
            if (unsafe { *unsafe { argv.offset((1 as i32) as isize) } })
                == std::ptr::null_mut::<i8>()
                || (unsafe { *unsafe { argv.offset((4 as i32) as isize) } })
                    != std::ptr::null_mut::<i8>()
                    && ((unsafe {
                        *unsafe {
                            unsafe { *unsafe { argv.offset((4 as i32) as isize) } }
                                .offset((0 as i32) as isize)
                        }
                    }) as i32)
                        != (0 as i32)
            {
                corruptSchema(pData, argv, std::ptr::null::<i8>());
            } else {
                // If the SQL column is blank it means this is an index that
                // was created to be the PRIMARY KEY or to fulfill a UNIQUE
                // constraint for a CREATE TABLE.  The index should have already
                // been created when we processed the CREATE TABLE.  All we have
                // to do here is record the root page number for that index.
                let mut pIndex: *mut Index = unsafe { std::mem::zeroed() };
                pIndex = unsafe {
                    sqlite3FindIndex(
                        db,
                        (unsafe { *unsafe { argv.offset((1 as i32) as isize) } }) as *const i8,
                        (unsafe {
                            (*unsafe { unsafe { (*db).aDb }.offset(iDb as isize) }).zDbSName
                        }) as *const i8,
                    )
                };
                if pIndex == std::ptr::null_mut::<Index>() {
                    corruptSchema(
                        pData,
                        argv,
                        (b"orphan index\0".as_ptr() as *mut i8) as *const i8,
                    );
                } else {
                    let __v789: bool;
                    if (unsafe {
                        sqlite3GetUInt32(
                            (unsafe { *unsafe { argv.offset((3 as i32) as isize) } }) as *const i8,
                            unsafe { std::ptr::addr_of_mut!((*pIndex).tnum) },
                        )
                    }) == (0 as i32)
                        || (unsafe { (*pIndex).tnum }) < ((2 as i32) as u32)
                        || (unsafe { (*pIndex).tnum }) > unsafe { (*pData).mxPage }
                    {
                        __v789 = true as bool;
                    } else {
                        __v789 = sqlite3IndexHasDuplicateRootPage(pIndex) != (0 as i32);
                    }
                    if __v789 {
                        if (unsafe { sqlite3Config.bExtraSchemaChecks }) != (0 as u8) {
                            corruptSchema(
                                pData,
                                argv,
                                (b"invalid rootpage\0".as_ptr() as *mut i8) as *const i8,
                            );
                        }
                    }
                }
            }
        }
    }
    if (unsafe { *unsafe { unsafe { (*pData).pzErrMsg }.offset((0 as i32) as isize) } })
        == std::ptr::null_mut::<i8>()
    {
        let mut pX: *mut Schema =
            unsafe { (*unsafe { unsafe { (*db).aDb }.offset(iDb as isize) }).pSchema };
        if unsafe { (*pX).tblHash.count }
            .wrapping_add(unsafe { (*pX).idxHash.count })
            .wrapping_add(unsafe { (*pX).trigHash.count })
            > ((unsafe {
                *unsafe {
                    unsafe { (*db).aLimit.as_mut_ptr() as *mut i32 }.offset((13 as i32) as isize)
                }
            }) as u32)
        {
            unsafe {
                *unsafe { (*pData).pzErrMsg } = unsafe {
                    sqlite3MPrintf(
                        db,
                        (b"too many schema objects\0".as_ptr() as *mut i8) as *const i8,
                    )
                };
            }
            unsafe {
                (*pData).rc = 1 as i32;
            }
            return 1 as i32;
        }
    }
    return 0 as i32;
}

/// Attempt to read the database schema and initialize internal
/// data structures for a single database file.  The index of the
/// database file is given by iDb.  iDb==0 is used for the main
/// database.  iDb==1 should never be used.  iDb>=2 is used for
/// auxiliary databases.  Return one of the SQLITE_ error codes to
/// indicate success or failure.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3InitOne(
    mut db: *mut sqlite3,
    mut iDb: i32,
    mut pzErrMsg: *mut *mut i8,
    mut mFlags: u32,
) -> i32 {
    let mut __slate_storage_804: std::mem::MaybeUninit<u16> = std::mem::MaybeUninit::uninit();
    let __slate_slot_804: *mut u16 = std::ptr::addr_of_mut!(__slate_storage_804) as *mut u16;
    let mut __slate_storage_803: std::mem::MaybeUninit<u16> = std::mem::MaybeUninit::uninit();
    let __slate_slot_803: *mut u16 = std::ptr::addr_of_mut!(__slate_storage_803) as *mut u16;
    // Hack: If the SQLITE_NoSchemaError flag is set, then consider
    // the schema loaded, even if errors (other than OOM) occurred. In
    // this situation the current sqlite3_prepare() operation will fail,
    // but the following one will attempt to compile the supplied statement
    // against whatever subset of the schema was loaded before the error
    // occurred.
    //
    // The primary purpose of this is to allow access to the sqlite_schema
    // table even when its contents have been corrupted.
    let mut __slate_storage_802: std::mem::MaybeUninit<*mut Schema> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_802: *mut *mut Schema =
        std::ptr::addr_of_mut!(__slate_storage_802) as *mut *mut Schema;
    let mut __slate_storage_471: std::mem::MaybeUninit<
        Option<
            unsafe extern "C-unwind" fn(
                *mut (),
                i32,
                *const i8,
                *const i8,
                *const i8,
                *const i8,
            ) -> i32,
        >,
    > = std::mem::MaybeUninit::uninit();
    let __slate_slot_471: *mut Option<
        unsafe extern "C-unwind" fn(
            *mut (),
            i32,
            *const i8,
            *const i8,
            *const i8,
            *const i8,
        ) -> i32,
    > = std::ptr::addr_of_mut!(__slate_storage_471)
        as *mut Option<
            unsafe extern "C-unwind" fn(
                *mut (),
                i32,
                *const i8,
                *const i8,
                *const i8,
                *const i8,
            ) -> i32,
        >;
    let mut __slate_storage_470: std::mem::MaybeUninit<*mut i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_470: *mut *mut i8 =
        std::ptr::addr_of_mut!(__slate_storage_470) as *mut *mut i8;
    let mut __slate_storage_801: std::mem::MaybeUninit<u64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_801: *mut u64 = std::ptr::addr_of_mut!(__slate_storage_801) as *mut u64;
    let mut __slate_storage_800: std::mem::MaybeUninit<u64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_800: *mut u64 = std::ptr::addr_of_mut!(__slate_storage_800) as *mut u64;
    let mut __slate_storage_799: std::mem::MaybeUninit<*mut sqlite3> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_799: *mut *mut sqlite3 =
        std::ptr::addr_of_mut!(__slate_storage_799) as *mut *mut sqlite3;
    let mut __slate_storage_469: std::mem::MaybeUninit<u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_469: *mut u8 = std::ptr::addr_of_mut!(__slate_storage_469) as *mut u8;
    let mut __slate_storage_798: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_798: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_798) as *mut i32;
    let mut __slate_storage_797: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_797: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_797) as *mut i32;
    let mut __slate_storage_796: std::mem::MaybeUninit<u16> = std::mem::MaybeUninit::uninit();
    let __slate_slot_796: *mut u16 = std::ptr::addr_of_mut!(__slate_storage_796) as *mut u16;
    let mut __slate_storage_795: std::mem::MaybeUninit<u16> = std::mem::MaybeUninit::uninit();
    let __slate_slot_795: *mut u16 = std::ptr::addr_of_mut!(__slate_storage_795) as *mut u16;
    let mut __slate_storage_794: std::mem::MaybeUninit<*mut Schema> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_794: *mut *mut Schema =
        std::ptr::addr_of_mut!(__slate_storage_794) as *mut *mut Schema;
    let mut __slate_storage_793: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_793: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_793) as *mut u32;
    let mut __slate_storage_792: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_792: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_792) as *mut u32;
    let mut __slate_storage_791: std::mem::MaybeUninit<*mut sqlite3> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_791: *mut *mut sqlite3 =
        std::ptr::addr_of_mut!(__slate_storage_791) as *mut *mut sqlite3;
    let mut __slate_storage_790: std::mem::MaybeUninit<*const i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_790: *mut *const i8 =
        std::ptr::addr_of_mut!(__slate_storage_790) as *mut *const i8;
    let mut __slate_storage_468: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_468: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_468) as *mut i32;
    let mut __slate_storage_467: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_467: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_467) as *mut i32;
    let mut __slate_storage_466: std::mem::MaybeUninit<*const i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_466: *mut *const i8 =
        std::ptr::addr_of_mut!(__slate_storage_466) as *mut *const i8;
    let mut __slate_storage_465: std::mem::MaybeUninit<__SlateRecord196> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_465: *mut __SlateRecord196 =
        std::ptr::addr_of_mut!(__slate_storage_465) as *mut __SlateRecord196;
    let mut __slate_storage_464: std::mem::MaybeUninit<__SlateAlign16<[i32; 5]>> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_464: *mut [i32; 5] =
        std::ptr::addr_of_mut!(__slate_storage_464) as *mut [i32; 5];
    let mut __slate_storage_463: std::mem::MaybeUninit<__SlateAlign16<[*const i8; 6]>> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_463: *mut [*const i8; 6] =
        std::ptr::addr_of_mut!(__slate_storage_463) as *mut [*const i8; 6];
    let mut __slate_storage_462: std::mem::MaybeUninit<*mut Db> = std::mem::MaybeUninit::uninit();
    let __slate_slot_462: *mut *mut Db =
        std::ptr::addr_of_mut!(__slate_storage_462) as *mut *mut Db;
    let mut __slate_storage_461: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_461: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_461) as *mut i32;
    let mut __slate_storage_460: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_460: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_460) as *mut i32;
    let mut __slate_storage_459: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_459: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_459) as *mut i32;
    unsafe {
        '__join_4: {
            std::ptr::write(__slate_slot_467, 0 as i32);
            std::ptr::write(
                __slate_slot_468,
                ((unsafe { (*db).mDbFlags }) & ((64 as i32) as u32) | (!(64 as i32) as u32)) as i32,
            );
            0 as i32;
            0 as i32;
            0 as i32;
            0 as i32;
            0 as i32;
            unsafe {
                (*db).init.busy = (((1 as i32)
                    + ((mFlags & ((3 as i32) as u32) != ((0 as i32) as u32)) as i32))
                    as i8) as u8;
            }
            // ^--- Any non-zero value for init.busy means that we are scanning
            // the sqlite_schema table to build the internal schema representation,
            // rather than running actual CREATE statements.  init.busy==2 has the
            // additional meaning that the scan is happening as part of
            // ALTER TABLE ADD COLUMN, which is stricter in its enforcement of
            // function name resolution.
            // Construct the in-memory representation schema tables (sqlite_schema or
            // sqlite_temp_schema) by invoking the parser directly.  The appropriate
            // table name will be inserted automatically by the parser so we can just
            // use the abbreviation "x" here.  The parser will also automatically tag
            // the schema table as read-only.
            unsafe {
                *unsafe {
                    ((*__slate_slot_463).as_mut_ptr() as *mut *const i8).offset((0 as i32) as isize)
                } = (b"table\0".as_ptr() as *mut i8) as *const i8;
            }
            std::ptr::write(
                __slate_slot_790,
                (if !((0 as i32) != (0 as i32)) && iDb == (1 as i32) {
                    b"sqlite_temp_master\0".as_ptr() as *mut i8
                } else {
                    b"sqlite_master\0".as_ptr() as *mut i8
                }) as *const i8,
            );
            *__slate_slot_466 = *__slate_slot_790;
            unsafe {
                *unsafe {
                    ((*__slate_slot_463).as_mut_ptr() as *mut *const i8).offset((1 as i32) as isize)
                } = *__slate_slot_790;
            }
            unsafe {
                *unsafe {
                    ((*__slate_slot_463).as_mut_ptr() as *mut *const i8).offset((2 as i32) as isize)
                } = unsafe {
                    *unsafe {
                        ((*__slate_slot_463).as_mut_ptr() as *mut *const i8)
                            .offset((1 as i32) as isize)
                    }
                };
            }
            unsafe {
                *unsafe {
                    ((*__slate_slot_463).as_mut_ptr() as *mut *const i8).offset((3 as i32) as isize)
                } = (b"1\0".as_ptr() as *mut i8) as *const i8;
            }
            unsafe {
                *unsafe {
                    ((*__slate_slot_463).as_mut_ptr() as *mut *const i8).offset((4 as i32) as isize)
                } = (b"CREATE TABLE x(type text,name text,tbl_name text,rootpage int,sql text)\0"
                    .as_ptr() as *mut i8) as *const i8;
            }
            unsafe {
                *unsafe {
                    ((*__slate_slot_463).as_mut_ptr() as *mut *const i8).offset((5 as i32) as isize)
                } = std::ptr::null::<i8>();
            }
            (*__slate_slot_465).db = db;
            (*__slate_slot_465).iDb = iDb;
            (*__slate_slot_465).rc = 0 as i32;
            (*__slate_slot_465).pzErrMsg = pzErrMsg;
            (*__slate_slot_465).mInitFlags = mFlags;
            (*__slate_slot_465).nInitRow = (0 as i32) as u32;
            (*__slate_slot_465).mxPage = (0 as i32) as u32;
            sqlite3InitCallback(
                std::ptr::addr_of_mut!(*__slate_slot_465) as *mut (),
                5 as i32,
                ((*__slate_slot_463).as_mut_ptr() as *mut *const i8) as *mut *mut i8,
                std::ptr::null_mut::<*mut i8>(),
            );
            std::ptr::write(__slate_slot_791, db);
            std::ptr::write(__slate_slot_792, unsafe { (*(*__slate_slot_791)).mDbFlags });
            std::ptr::write(
                __slate_slot_793,
                *__slate_slot_792 & (*__slate_slot_468 as u32),
            );
            unsafe {
                (*(*__slate_slot_791)).mDbFlags = *__slate_slot_793;
            }
            if (*__slate_slot_465).rc != (0 as i32) {
                *__slate_slot_459 = (*__slate_slot_465).rc;
            } else {
                // Create a cursor to hold the database open
                *__slate_slot_462 = unsafe { unsafe { (*db).aDb }.offset(iDb as isize) };
                if (unsafe { (*(*__slate_slot_462)).pBt }) == std::ptr::null_mut::<Btree>() {
                    0 as i32;
                    std::ptr::write(__slate_slot_794, unsafe {
                        (*unsafe { unsafe { (*db).aDb }.offset((1 as i32) as isize) }).pSchema
                    });
                    std::ptr::write(__slate_slot_795, unsafe {
                        (*(*__slate_slot_794)).schemaFlags
                    });
                    std::ptr::write(
                        __slate_slot_796,
                        ((((*__slate_slot_795 as u32) as i32) | (1 as i32)) as i16) as u16,
                    );
                    unsafe {
                        (*(*__slate_slot_794)).schemaFlags = *__slate_slot_796;
                    }
                    *__slate_slot_459 = 0 as i32;
                } else {
                    '__join_7: {
                        // If there is not already a read-only (or read-write) transaction opened
                        // on the b-tree database, open one now. If a transaction is opened, it
                        // will be closed before this function returns.
                        unsafe { sqlite3BtreeEnter(unsafe { (*(*__slate_slot_462)).pBt }) };
                        if (unsafe { sqlite3BtreeTxnState(unsafe { (*(*__slate_slot_462)).pBt }) })
                            == (0 as i32)
                        {
                            *__slate_slot_459 = unsafe {
                                sqlite3BtreeBeginTrans(
                                    unsafe { (*(*__slate_slot_462)).pBt },
                                    0 as i32,
                                    std::ptr::null_mut::<i32>(),
                                )
                            };
                            if *__slate_slot_459 != (0 as i32) {
                                unsafe {
                                    sqlite3SetString(pzErrMsg, db, unsafe {
                                        sqlite3ErrStr(*__slate_slot_459)
                                    })
                                };
                                break '__join_7;
                            } else {
                                *__slate_slot_467 = 1 as i32;
                            }
                        }
                        // Get the database meta information.
                        //
                        // Meta values are as follows:
                        //    meta[0]   Schema cookie.  Changes with each schema change.
                        //    meta[1]   File format of schema layer.
                        //    meta[2]   Size of the page cache.
                        //    meta[3]   Largest rootpage (auto/incr_vacuum mode)
                        //    meta[4]   Db text encoding. 1:UTF-8 2:UTF-16LE 3:UTF-16BE
                        //    meta[5]   User version
                        //    meta[6]   Incremental vacuum mode
                        //    meta[7]   unused
                        //    meta[8]   unused
                        //    meta[9]   unused
                        //
                        // Note: The #defined SQLITE_UTF* symbols in sqliteInt.h correspond to
                        // the possible values of meta[4].
                        *__slate_slot_460 = 0 as i32;
                        loop {
                            if *__slate_slot_460 < ((((20 as u64) / (4 as u64)) as u32) as i32) {
                                unsafe {
                                    sqlite3BtreeGetMeta(
                                        unsafe { (*(*__slate_slot_462)).pBt },
                                        *__slate_slot_460 + (1 as i32),
                                        (unsafe {
                                            ((*__slate_slot_464).as_mut_ptr() as *mut i32)
                                                .offset(*__slate_slot_460 as isize)
                                        }) as *mut u32,
                                    )
                                };
                                std::ptr::write(__slate_slot_797, *__slate_slot_460);
                                std::ptr::write(__slate_slot_798, *__slate_slot_797 + (1 as i32));
                                *__slate_slot_460 = *__slate_slot_798;
                            } else {
                                break;
                            }
                        }
                        if (unsafe { (*db).flags }) & (((33554432 as i32) as i64) as u64)
                            != (((0 as i32) as i64) as u64)
                        {
                            unsafe {
                                memset(
                                    ((*__slate_slot_464).as_mut_ptr() as *mut i32) as *mut (),
                                    0 as i32,
                                    20 as u64,
                                )
                            };
                        }
                        unsafe {
                            (*unsafe { (*(*__slate_slot_462)).pSchema }).schema_cookie = unsafe {
                                *unsafe {
                                    ((*__slate_slot_464).as_mut_ptr() as *mut i32)
                                        .offset(((1 as i32) - (1 as i32)) as isize)
                                }
                            };
                        }
                        // If opening a non-empty database, check the text encoding. For the
                        // main database, set sqlite3.enc to the encoding of the main database.
                        // For an attached db, it is an error if the encoding is not the same
                        // as sqlite3.enc.
                        if (unsafe {
                            *unsafe {
                                ((*__slate_slot_464).as_mut_ptr() as *mut i32)
                                    .offset(((5 as i32) - (1 as i32)) as isize)
                            }
                        }) != (0 as i32)
                        {
                            // text encoding
                            if iDb == (0 as i32)
                                && (unsafe { (*db).mDbFlags }) & ((64 as i32) as u32)
                                    == ((0 as i32) as u32)
                            {
                                // If opening the main database, set ENC(db).
                                *__slate_slot_469 =
                                    (((((((unsafe {
                                        *unsafe {
                                            ((*__slate_slot_464).as_mut_ptr() as *mut i32)
                                                .offset(((5 as i32) - (1 as i32)) as isize)
                                        }
                                    }) as i8) as u8) as u32)
                                        as i32)
                                        & (3 as i32)) as i8)
                                        as u8;
                                if ((*__slate_slot_469 as u32) as i32) == (0 as i32) {
                                    *__slate_slot_469 = ((1 as i32) as i8) as u8;
                                }
                                unsafe { sqlite3SetTextEncoding(db, *__slate_slot_469) };
                            } else {
                                // If opening an attached database, the encoding much match ENC(db)
                                if (unsafe {
                                    *unsafe {
                                        ((*__slate_slot_464).as_mut_ptr() as *mut i32)
                                            .offset(((5 as i32) - (1 as i32)) as isize)
                                    }
                                }) & (3 as i32)
                                    != (((unsafe { (*db).enc }) as u32) as i32)
                                {
                                    unsafe {
                                        sqlite3SetString(pzErrMsg, db, (b"attached databases must use the same text encoding as main database\0".as_ptr() as *mut i8) as *const i8)
                                    };
                                    *__slate_slot_459 = 1 as i32;
                                    break '__join_7;
                                }
                            }
                        }
                        unsafe {
                            (*unsafe { (*(*__slate_slot_462)).pSchema }).enc = unsafe { (*db).enc };
                        }
                        if (unsafe { (*unsafe { (*(*__slate_slot_462)).pSchema }).cache_size })
                            == (0 as i32)
                        {
                            '__join_23: {
                                *__slate_slot_461 = unsafe {
                                    sqlite3AbsInt32(unsafe {
                                        *unsafe {
                                            ((*__slate_slot_464).as_mut_ptr() as *mut i32)
                                                .offset(((3 as i32) - (1 as i32)) as isize)
                                        }
                                    })
                                };
                                if *__slate_slot_461 == (0 as i32) {
                                    *__slate_slot_461 = -(2000 as i32);
                                }
                            }
                            unsafe {
                                (*unsafe { (*(*__slate_slot_462)).pSchema }).cache_size =
                                    *__slate_slot_461;
                            }
                            unsafe {
                                sqlite3BtreeSetCacheSize(
                                    unsafe { (*(*__slate_slot_462)).pBt },
                                    unsafe {
                                        (*unsafe { (*(*__slate_slot_462)).pSchema }).cache_size
                                    },
                                )
                            };
                        }
                        // file_format==1    Version 3.0.0.
                        // file_format==2    Version 3.1.3.  // ALTER TABLE ADD COLUMN
                        // file_format==3    Version 3.1.4.  // ditto but with non-NULL defaults
                        // file_format==4    Version 3.3.0.  // DESC indices.  Boolean constants
                        unsafe {
                            (*unsafe { (*(*__slate_slot_462)).pSchema }).file_format =
                                ((unsafe {
                                    *unsafe {
                                        ((*__slate_slot_464).as_mut_ptr() as *mut i32)
                                            .offset(((2 as i32) - (1 as i32)) as isize)
                                    }
                                }) as i8) as u8;
                        }
                        if (((unsafe { (*unsafe { (*(*__slate_slot_462)).pSchema }).file_format })
                            as u32) as i32)
                            == (0 as i32)
                        {
                            unsafe {
                                (*unsafe { (*(*__slate_slot_462)).pSchema }).file_format =
                                    ((1 as i32) as i8) as u8;
                            }
                        }
                        if (((unsafe { (*unsafe { (*(*__slate_slot_462)).pSchema }).file_format })
                            as u32) as i32)
                            > (4 as i32)
                        {
                            unsafe {
                                sqlite3SetString(
                                    pzErrMsg,
                                    db,
                                    (b"unsupported file format\0".as_ptr() as *mut i8) as *const i8,
                                )
                            };
                            *__slate_slot_459 = 1 as i32;
                        } else {
                            // Ticket #2804:  When we open a database in the newer file format,
                            // clear the legacy_file_format pragma flag so that a VACUUM will
                            // not downgrade the database and thus invalidate any descending
                            // indices that the user might have created.
                            if iDb == (0 as i32)
                                && (unsafe {
                                    *unsafe {
                                        ((*__slate_slot_464).as_mut_ptr() as *mut i32)
                                            .offset(((2 as i32) - (1 as i32)) as isize)
                                    }
                                }) >= (4 as i32)
                            {
                                std::ptr::write(__slate_slot_799, db);
                                std::ptr::write(__slate_slot_800, unsafe {
                                    (*(*__slate_slot_799)).flags
                                });
                                std::ptr::write(
                                    __slate_slot_801,
                                    *__slate_slot_800 & !(((2 as i32) as i64) as u64),
                                );
                                unsafe {
                                    (*(*__slate_slot_799)).flags = *__slate_slot_801;
                                }
                            }
                            // Read the schema information out of the schema tables
                            0 as i32;
                            (*__slate_slot_465).mxPage = unsafe {
                                sqlite3BtreeLastPage(unsafe { (*(*__slate_slot_462)).pBt })
                            };
                            *__slate_slot_470 = unsafe {
                                sqlite3MPrintf(
                                    db,
                                    (b"SELECT*FROM\"%w\".%s ORDER BY rowid\0".as_ptr() as *mut i8)
                                        as *const i8,
                                    unsafe {
                                        (*unsafe { unsafe { (*db).aDb }.offset(iDb as isize) })
                                            .zDbSName
                                    },
                                    *__slate_slot_466,
                                )
                            };
                            *__slate_slot_471 = unsafe { (*db).xAuth };
                            unsafe {
                                (*db).xAuth = None;
                            }
                            *__slate_slot_459 = unsafe {
                                sqlite3_exec(
                                    db,
                                    *__slate_slot_470 as *const i8,
                                    Some(sqlite3InitCallback),
                                    std::ptr::addr_of_mut!(*__slate_slot_465) as *mut (),
                                    std::ptr::null_mut::<*mut i8>(),
                                )
                            };
                            unsafe {
                                (*db).xAuth = *__slate_slot_471;
                            }
                            if *__slate_slot_459 == (0 as i32) {
                                *__slate_slot_459 = (*__slate_slot_465).rc;
                            }
                            unsafe { sqlite3DbFree(db, *__slate_slot_470 as *mut ()) };
                            if *__slate_slot_459 == (0 as i32) {
                                unsafe { sqlite3AnalysisLoad(db, iDb) };
                            }
                            0 as i32;
                            if (unsafe { (*db).mallocFailed }) != (0 as u8) {
                                *__slate_slot_459 = 7 as i32;
                                unsafe { sqlite3ResetAllSchemasOfConnection(db) };
                                *__slate_slot_462 =
                                    unsafe { unsafe { (*db).aDb }.offset(iDb as isize) };
                            } else {
                                if *__slate_slot_459 == (0 as i32)
                                    || (unsafe { (*db).flags })
                                        & (((134217728 as i32) as i64) as u64)
                                        != (0 as u64)
                                        && *__slate_slot_459 != (7 as i32)
                                {
                                    std::ptr::write(__slate_slot_802, unsafe {
                                        (*unsafe { unsafe { (*db).aDb }.offset(iDb as isize) })
                                            .pSchema
                                    });
                                    std::ptr::write(__slate_slot_803, unsafe {
                                        (*(*__slate_slot_802)).schemaFlags
                                    });
                                    std::ptr::write(
                                        __slate_slot_804,
                                        ((((*__slate_slot_803 as u32) as i32) | (1 as i32)) as i16)
                                            as u16,
                                    );
                                    unsafe {
                                        (*(*__slate_slot_802)).schemaFlags = *__slate_slot_804;
                                    }
                                    *__slate_slot_459 = 0 as i32;
                                }
                            }
                            // Jump here for an error that occurs after successfully allocating
                            // curMain and calling sqlite3BtreeEnter(). For an error that occurs
                            // before that point, jump to error_out.
                        }
                    }
                    if *__slate_slot_467 != (0 as i32) {
                        unsafe { sqlite3BtreeCommit(unsafe { (*(*__slate_slot_462)).pBt }) };
                    }
                    unsafe { sqlite3BtreeLeave(unsafe { (*(*__slate_slot_462)).pBt }) };
                }
            }
        }
        if *__slate_slot_459 != (0 as i32) {
            if *__slate_slot_459 == (7 as i32)
                || *__slate_slot_459 == (10 as i32) | (12 as i32) << (8 as i32)
            {
                unsafe { sqlite3OomFault(db) };
            }
            unsafe { sqlite3ResetOneSchema(db, iDb) };
        }
        unsafe {
            (*db).init.busy = ((0 as i32) as i8) as u8;
        }
        return *__slate_slot_459;
    }
    return unsafe { std::mem::zeroed() };
}

/// Initialize all database files - the main database file, the file
/// used to store temporary tables, and any additional database files
/// created using ATTACH statements.  Return a success code.  If an
/// error occurs, write an error message into *pzErrMsg.
///
/// After a database is initialized, the DB_SchemaLoaded bit is set
/// bit is set in the flags field of the Db structure.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3Init(mut db: *mut sqlite3, mut pzErrMsg: *mut *mut i8) -> i32 {
    let mut i: i32 = 0 as i32;
    let mut rc: i32 = 0 as i32;
    let mut commit_internal: i32 =
        !((unsafe { (*db).mDbFlags }) & ((1 as i32) as u32) != (0 as u32)) as i32;
    0 as i32;
    0 as i32;
    0 as i32;
    unsafe {
        (*db).enc = unsafe {
            (*unsafe { (*unsafe { unsafe { (*db).aDb }.offset((0 as i32) as isize) }).pSchema }).enc
        };
    }
    0 as i32;
    // Do the main schema first
    if !((((unsafe {
        (*unsafe { (*unsafe { unsafe { (*db).aDb }.offset((0 as i32) as isize) }).pSchema })
            .schemaFlags
    }) as u32) as i32)
        & (1 as i32)
        == (1 as i32))
    {
        rc = sqlite3InitOne(db, 0 as i32, pzErrMsg, (0 as i32) as u32);
        if rc != (0 as i32) {
            return rc;
        }
    }
    // All other schemas after the main schema. The "temp" schema must be last
    i = (unsafe { (*db).nDb }) - (1 as i32);
    '__slate_break_767: loop {
        if !(i > (0 as i32)) {
            break;
        }
        0 as i32;
        if !((((unsafe {
            (*unsafe { (*unsafe { unsafe { (*db).aDb }.offset(i as isize) }).pSchema }).schemaFlags
        }) as u32) as i32)
            & (1 as i32)
            == (1 as i32))
        {
            rc = sqlite3InitOne(db, i, pzErrMsg, (0 as i32) as u32);
            if rc != (0 as i32) {
                return rc;
            }
        }
        let __v781: i32 = i;
        let __v782: i32 = __v781 - (1 as i32);
        i = __v782;
    }
    if commit_internal != (0 as i32) {
        unsafe { sqlite3CommitInternalChanges(db) };
    }
    return 0 as i32;
}

/// This routine is a no-op if the database schema is already initialized.
/// Otherwise, the schema is loaded. An error code is returned.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3ReadSchema(mut pParse: *mut Parse) -> i32 {
    let mut rc: i32 = 0 as i32;
    let mut db: *mut sqlite3 = unsafe { (*pParse).db };
    0 as i32;
    if !((unsafe { (*db).init.busy }) != (0 as u8)) {
        rc = sqlite3Init(db, unsafe { std::ptr::addr_of_mut!((*pParse).zErrMsg) });
        if rc != (0 as i32) {
            unsafe {
                (*pParse).rc = rc;
            }
            let __v805: *mut Parse = pParse;
            let __v806: i32 = unsafe { (*__v805).nErr };
            let __v807: i32 = __v806 + (1 as i32);
            unsafe {
                (*__v805).nErr = __v807;
            }
        } else {
            if (unsafe { (*db).noSharedCache }) != (0 as u8) {
                let __v808: *mut sqlite3 = db;
                let __v809: u32 = unsafe { (*__v808).mDbFlags };
                let __v810: u32 = __v809 | ((16 as i32) as u32);
                unsafe {
                    (*__v808).mDbFlags = __v810;
                }
            }
        }
    }
    return rc;
}

/// Check schema cookies in all databases.  If any cookie is out
/// of date set pParse->rc to SQLITE_SCHEMA.  If all schema cookies
/// make no changes to pParse->rc.
fn schemaIsValid(mut pParse: *mut Parse) {
    let mut db: *mut sqlite3 = unsafe { (*pParse).db };
    let mut iDb: i32 = 0 as i32;
    let mut rc: i32 = 0 as i32;
    let mut cookie: i32 = 0 as i32;
    0 as i32;
    0 as i32;
    iDb = 0 as i32;
    '__slate_break_768: loop {
        if !(iDb < unsafe { (*db).nDb }) {
            break;
        }
        let mut openedTransaction: i32 = 0 as i32; // True if a transaction is opened
        let mut pBt: *mut Btree =
            unsafe { (*unsafe { unsafe { (*db).aDb }.offset(iDb as isize) }).pBt }; // Btree database to read cookie from
        if pBt == std::ptr::null_mut::<Btree>() {
        } else {
            // If there is not already a read-only (or read-write) transaction opened
            // on the b-tree database, open one now. If a transaction is opened, it
            // will be closed immediately after reading the meta-value.
            if (unsafe { sqlite3BtreeTxnState(pBt) }) == (0 as i32) {
                rc = unsafe { sqlite3BtreeBeginTrans(pBt, 0 as i32, std::ptr::null_mut::<i32>()) };
                if rc == (7 as i32) || rc == (10 as i32) | (12 as i32) << (8 as i32) {
                    unsafe { sqlite3OomFault(db) };
                    unsafe {
                        (*pParse).rc = 7 as i32;
                    }
                }
                if rc != (0 as i32) {
                    return;
                }
                openedTransaction = 1 as i32;
            }
            // Read the schema cookie from the database. If it does not match the
            // value stored as part of the in-memory schema representation,
            // set Parse.rc to SQLITE_SCHEMA.
            unsafe {
                sqlite3BtreeGetMeta(pBt, 1 as i32, std::ptr::addr_of_mut!(cookie) as *mut u32)
            };
            0 as i32;
            if cookie
                != unsafe {
                    (*unsafe { (*unsafe { unsafe { (*db).aDb }.offset(iDb as isize) }).pSchema })
                        .schema_cookie
                }
            {
                if (((unsafe {
                    (*unsafe { (*unsafe { unsafe { (*db).aDb }.offset(iDb as isize) }).pSchema })
                        .schemaFlags
                }) as u32) as i32)
                    & (1 as i32)
                    == (1 as i32)
                {
                    unsafe {
                        (*pParse).rc = 17 as i32;
                    }
                }
                unsafe { sqlite3ResetOneSchema(db, iDb) };
            }
            // Close the transaction, if one was opened.
            if openedTransaction != (0 as i32) {
                unsafe { sqlite3BtreeCommit(pBt) };
            }
        }
        let __v823: i32 = iDb;
        let __v824: i32 = __v823 + (1 as i32);
        iDb = __v824;
    }
}

/// Convert a schema pointer into the iDb index that indicates
/// which database file in db->aDb[] the schema refers to.
///
/// If the same database is attached more than once, the first
/// attached database is returned.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3SchemaToIndex(mut db: *mut sqlite3, mut pSchema: *mut Schema) -> i32 {
    let mut i: i32 = -(32768 as i32);
    // If pSchema is NULL, then return -32768. This happens when code in
    // expr.c is trying to resolve a reference to a transient table (i.e. one
    // created by a sub-select). In this case the return value of this
    // function should never be used.
    //
    // We return -32768 instead of the more usual -1 simply because using
    // -32768 as the incorrect index into db->aDb[] is much
    // more likely to cause a segfault than -1 (of course there are assert()
    // statements too, but it never hurts to play the odds) and
    // -32768 will still fit into a 16-bit signed integer.
    0 as i32;
    if pSchema != std::ptr::null_mut::<Schema>() {
        i = 0 as i32;
        '__slate_break_769: loop {
            if !((1 as i32) != (0 as i32)) {
                break;
            }
            0 as i32;
            if (unsafe { (*unsafe { unsafe { (*db).aDb }.offset(i as isize) }).pSchema }) == pSchema
            {
                break '__slate_break_769;
            }
            let __v811: i32 = i;
            let __v812: i32 = __v811 + (1 as i32);
            i = __v812;
        }
        0 as i32;
    }
    return i;
}

/// Free all memory allocations in the pParse object
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3ParseObjectReset(mut pParse: *mut Parse) {
    let mut db: *mut sqlite3 = unsafe { (*pParse).db };
    0 as i32;
    0 as i32;
    0 as i32;
    if (unsafe { (*pParse).nTableLock }) != (0 as i32) {
        unsafe { sqlite3DbNNFreeNN(db, (unsafe { (*pParse).aTableLock }) as *mut ()) };
    }
    '__slate_break_770: while (unsafe { (*pParse).pCleanup })
        != std::ptr::null_mut::<ParseCleanup>()
    {
        let mut pCleanup: *mut ParseCleanup = unsafe { (*pParse).pCleanup };
        unsafe {
            (*pParse).pCleanup = unsafe { (*pCleanup).pNext };
        }
        unsafe { unsafe { (*pCleanup).xCleanup }.unwrap()(db, unsafe { (*pCleanup).pPtr }) };
        unsafe { sqlite3DbNNFreeNN(db, pCleanup as *mut ()) };
    }
    if (unsafe { (*pParse).aLabel }) != std::ptr::null_mut::<i32>() {
        unsafe { sqlite3DbNNFreeNN(db, (unsafe { (*pParse).aLabel }) as *mut ()) };
    }
    if (unsafe { (*pParse).pConstExpr }) != std::ptr::null_mut::<ExprList>() {
        unsafe { sqlite3ExprListDelete(db, unsafe { (*pParse).pConstExpr }) };
    }
    0 as i32;
    let __v813: *mut sqlite3 = db;
    let __v814: u32 = unsafe { (*__v813).lookaside.bDisable };
    let __v815: u32 =
        __v814.wrapping_sub((((unsafe { (*pParse).disableLookaside }) as u32) as i32) as u32);
    unsafe {
        (*__v813).lookaside.bDisable = __v815;
    }
    unsafe {
        (*db).lookaside.sz = ((if (unsafe { (*db).lookaside.bDisable }) != (0 as u32) {
            0 as i32
        } else {
            ((unsafe { (*db).lookaside.szTrue }) as u32) as i32
        }) as i16) as u16;
    }
    0 as i32;
    unsafe {
        (*db).pParse = unsafe { (*pParse).pOuterParse };
    }
}

/// Add a new cleanup operation to a Parser.  The cleanup should happen when
/// the parser object is destroyed.  But, beware: the cleanup might happen
/// immediately.
///
/// Use this mechanism for uncommon cleanups.  There is a higher setup
/// cost for this mechanism (an extra malloc), so it should not be used
/// for common cleanups that happen on most calls.  But for less
/// common cleanups, we save a single NULL-pointer comparison in
/// sqlite3ParseObjectReset(), which reduces the total CPU cycle count.
///
/// If a memory allocation error occurs, then the cleanup happens immediately.
/// When either SQLITE_DEBUG or SQLITE_COVERAGE_TEST are defined, the
/// pParse->earlyCleanup flag is set in that case.  Calling code show verify
/// that test cases exist for which this happens, to guard against possible
/// use-after-free errors following an OOM.  The preferred way to do this is
/// to immediately follow the call to this routine with:
///
///       testcase( pParse->earlyCleanup );
///
/// This routine returns a copy of its pPtr input (the third parameter)
/// except if an early cleanup occurs, in which case it returns NULL.  So
/// another way to check for early cleanup is to check the return value.
/// Or, stop using the pPtr parameter with this call and use only its
/// return value thereafter.  Something like this:
///
///       pObj = sqlite3ParserAddCleanup(pParse, destructor, pObj);
///
/// # Arguments
///
/// * `pParse` - Destroy when this Parser finishes
/// * `xCleanup` - The cleanup routine
/// * `pPtr` - Pointer to object to be cleaned up
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3ParserAddCleanup(
    mut pParse: *mut Parse,
    mut xCleanup: Option<unsafe extern "C-unwind" fn(*mut sqlite3, *mut ())>,
    mut pPtr: *mut (),
) -> *mut () {
    let mut pCleanup: *mut ParseCleanup = unsafe { std::mem::zeroed() };
    if (unsafe { sqlite3FaultSim(300 as i32) }) != (0 as i32) {
        pCleanup = std::ptr::null_mut::<ParseCleanup>();
        unsafe { sqlite3OomFault(unsafe { (*pParse).db }) };
    } else {
        pCleanup = (unsafe { sqlite3DbMallocRaw(unsafe { (*pParse).db }, 24 as u64) })
            as *mut ParseCleanup;
    }
    if pCleanup != std::ptr::null_mut::<ParseCleanup>() {
        unsafe {
            (*pCleanup).pNext = unsafe { (*pParse).pCleanup };
        }
        unsafe {
            (*pParse).pCleanup = pCleanup;
        }
        unsafe {
            (*pCleanup).pPtr = pPtr;
        }
        unsafe {
            (*pCleanup).xCleanup = xCleanup;
        }
    } else {
        unsafe { xCleanup.unwrap()(unsafe { (*pParse).db }, pPtr) };
        pPtr = std::ptr::null_mut::<()>();
    }
    return pPtr;
}

/// Turn bulk memory into a valid Parse object and link that Parse object
/// into database connection db.
///
/// Call sqlite3ParseObjectReset() to undo this operation.
///
/// Caution:  Do not confuse this routine with sqlite3ParseObjectInit() which
/// is generated by Lemon.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3ParseObjectInit(mut pParse: *mut Parse, mut db: *mut sqlite3) {
    0 as i32;
    unsafe {
        memset(
            (unsafe { (pParse as *mut i8).offset((8 as u64) as isize) }) as *mut (),
            0 as i32,
            (168 as u64).wrapping_sub(8 as u64),
        )
    };
    unsafe {
        memset(
            (unsafe { (pParse as *mut i8).offset((280 as u64) as isize) }) as *mut (),
            0 as i32,
            (432 as u64).wrapping_sub(280 as u64),
        )
    };
    0 as i32;
    unsafe {
        (*pParse).pOuterParse = unsafe { (*db).pParse };
    }
    unsafe {
        (*db).pParse = pParse;
    }
    unsafe {
        (*pParse).db = db;
    }
    if (unsafe { (*db).mallocFailed }) != (0 as u8) {
        unsafe {
            sqlite3ErrorMsg(
                pParse,
                (b"out of memory\0".as_ptr() as *mut i8) as *const i8,
            )
        };
    }
}

// Maximum number of times that we will try again to prepare a statement
// that returns SQLITE_ERROR_RETRY.
/// Compile the UTF-8 encoded SQL statement zSql into a statement handle.
///
/// # Arguments
///
/// * `db` - Database handle.
/// * `zSql` - UTF-8 encoded SQL statement.
/// * `nBytes` - Length of zSql in bytes.
/// * `prepFlags` - Zero or more SQLITE_PREPARE_* flags
/// * `pReprepare` - VM being reprepared
/// * `ppStmt` - OUT: A pointer to the prepared statement
/// * `pzTail` - OUT: End of parsed string
fn sqlite3Prepare(
    mut db: *mut sqlite3,
    mut zSql: *const i8,
    mut nBytes: i32,
    mut prepFlags: u32,
    mut pReprepare: *mut Vdbe,
    mut ppStmt: *mut *mut sqlite3_stmt,
    mut pzTail: *mut *const i8,
) -> i32 {
    let mut __slate_storage_515: std::mem::MaybeUninit<*mut TriggerPrg> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_515: *mut *mut TriggerPrg =
        std::ptr::addr_of_mut!(__slate_storage_515) as *mut *mut TriggerPrg;
    let mut __slate_storage_514: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_514: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_514) as *mut i32;
    let mut __slate_storage_513: std::mem::MaybeUninit<*mut i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_513: *mut *mut i8 =
        std::ptr::addr_of_mut!(__slate_storage_513) as *mut *mut i8;
    let mut __slate_storage_822: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_822: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_822) as *mut i32;
    let mut __slate_storage_821: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_821: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_821) as *mut i32;
    let mut __slate_storage_512: std::mem::MaybeUninit<*const i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_512: *mut *const i8 =
        std::ptr::addr_of_mut!(__slate_storage_512) as *mut *const i8;
    let mut __slate_storage_511: std::mem::MaybeUninit<*mut Btree> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_511: *mut *mut Btree =
        std::ptr::addr_of_mut!(__slate_storage_511) as *mut *mut Btree;
    let mut __slate_storage_820: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_820: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_820) as *mut u32;
    let mut __slate_storage_819: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_819: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_819) as *mut u32;
    let mut __slate_storage_818: std::mem::MaybeUninit<*mut sqlite3> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_818: *mut *mut sqlite3 =
        std::ptr::addr_of_mut!(__slate_storage_818) as *mut *mut sqlite3;
    let mut __slate_storage_817: std::mem::MaybeUninit<u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_817: *mut u8 = std::ptr::addr_of_mut!(__slate_storage_817) as *mut u8;
    let mut __slate_storage_816: std::mem::MaybeUninit<u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_816: *mut u8 = std::ptr::addr_of_mut!(__slate_storage_816) as *mut u8; // Parsing context
    let mut __slate_storage_510: std::mem::MaybeUninit<Parse> = std::mem::MaybeUninit::uninit();
    let __slate_slot_510: *mut Parse = std::ptr::addr_of_mut!(__slate_storage_510) as *mut Parse; // Loop counter
    let mut __slate_storage_509: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_509: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_509) as *mut i32; // Result code
    let mut __slate_storage_508: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_508: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_508) as *mut i32;
    unsafe {
        '__join_38: {
            std::ptr::write(__slate_slot_508, 0 as i32);
            // sqlite3ParseObjectInit(&sParse, db); // inlined for performance
            unsafe {
                memset(
                    (unsafe {
                        (std::ptr::addr_of_mut!(*__slate_slot_510) as *mut i8)
                            .offset((8 as u64) as isize)
                    }) as *mut (),
                    0 as i32,
                    (168 as u64).wrapping_sub(8 as u64),
                )
            };
            unsafe {
                memset(
                    (unsafe {
                        (std::ptr::addr_of_mut!(*__slate_slot_510) as *mut i8)
                            .offset((280 as u64) as isize)
                    }) as *mut (),
                    0 as i32,
                    (432 as u64).wrapping_sub(280 as u64),
                )
            };
            (*__slate_slot_510).pOuterParse = unsafe { (*db).pParse };
            unsafe {
                (*db).pParse = std::ptr::addr_of_mut!(*__slate_slot_510);
            }
            (*__slate_slot_510).db = db;
            if pReprepare != std::ptr::null_mut::<Vdbe>() {
                (*__slate_slot_510).pReprepare = pReprepare;
                (*__slate_slot_510).explain =
                    ((unsafe { sqlite3_stmt_isexplain(pReprepare as *mut sqlite3_stmt) }) as i8)
                        as u8;
            } else {
                0 as i32;
            }
        }
        '__join_0: {
            0 as i32;
            if (unsafe { (*db).mallocFailed }) != (0 as u8) {
                unsafe {
                    sqlite3ErrorMsg(
                        std::ptr::addr_of_mut!(*__slate_slot_510),
                        (b"out of memory\0".as_ptr() as *mut i8) as *const i8,
                    )
                };
                *__slate_slot_508 = 7 as i32;
                unsafe {
                    (*db).errCode = 7 as i32;
                }
            } else {
                0 as i32;
                // For a long-term use prepared statement avoid the use of
                // lookaside memory.
                if prepFlags & ((1 as i32) as u32) != (0 as u32) {
                    std::ptr::write(__slate_slot_816, (*__slate_slot_510).disableLookaside);
                    std::ptr::write(
                        __slate_slot_817,
                        ((((*__slate_slot_816 as u32) as i32) + (1 as i32)) as i8) as u8,
                    );
                    (*__slate_slot_510).disableLookaside = *__slate_slot_817;
                    std::ptr::write(__slate_slot_818, db);
                    std::ptr::write(__slate_slot_819, unsafe {
                        (*(*__slate_slot_818)).lookaside.bDisable
                    });
                    std::ptr::write(
                        __slate_slot_820,
                        (*__slate_slot_819).wrapping_add((1 as i32) as u32),
                    );
                    unsafe {
                        (*(*__slate_slot_818)).lookaside.bDisable = *__slate_slot_820;
                    }
                    unsafe {
                        (*db).lookaside.sz = ((0 as i32) as i16) as u16;
                    }
                }
                '__join_27: {
                    (*__slate_slot_510).prepFlags = (prepFlags & ((255 as i32) as u32)) as u8;
                    // Check to verify that it is possible to get a read lock on all
                    // database schemas.  The inability to get a read lock indicates that
                    // some other database connection is holding a write-lock, which in
                    // turn means that the other connection has made uncommitted changes
                    // to the schema.
                    //
                    // Were we to proceed and prepare the statement against the uncommitted
                    // schema changes and if those schema changes are subsequently rolled
                    // back and different changes are made in their place, then when this
                    // prepared statement goes to run the schema cookie would fail to detect
                    // the schema change.  Disaster would follow.
                    //
                    // This thread is currently holding mutexes on all Btrees (because
                    // of the sqlite3BtreeEnterAll() in sqlite3LockAndPrepare()) so it
                    // is not possible for another thread to start a new schema change
                    // while this routine is running.  Hence, we do not need to hold
                    // locks on the schema, we just need to make sure nobody else is
                    // holding them.
                    //
                    // Note that setting READ_UNCOMMITTED overrides most lock detection,
                    // but it does *not* override schema lock detection, so this all still
                    // works even if READ_UNCOMMITTED is set.
                    if !((unsafe { (*db).noSharedCache }) != (0 as u8)) {
                        *__slate_slot_509 = 0 as i32;
                        '__loop_28: loop {
                            if *__slate_slot_509 < unsafe { (*db).nDb } {
                                std::ptr::write(__slate_slot_511, unsafe {
                                    (*unsafe {
                                        unsafe { (*db).aDb }.offset(*__slate_slot_509 as isize)
                                    })
                                    .pBt
                                });
                                if *__slate_slot_511 != std::ptr::null_mut::<Btree>() {
                                    0 as i32;
                                    *__slate_slot_508 =
                                        unsafe { sqlite3BtreeSchemaLocked(*__slate_slot_511) };
                                    if *__slate_slot_508 != (0 as i32) {
                                        break '__loop_28;
                                    }
                                }
                                std::ptr::write(__slate_slot_821, *__slate_slot_509);
                                std::ptr::write(__slate_slot_822, *__slate_slot_821 + (1 as i32));
                                *__slate_slot_509 = *__slate_slot_822;
                            } else {
                                break '__join_27;
                            }
                        }
                        std::ptr::write(
                            __slate_slot_512,
                            (unsafe {
                                (*unsafe {
                                    unsafe { (*db).aDb }.offset(*__slate_slot_509 as isize)
                                })
                                .zDbSName
                            }) as *const i8,
                        );
                        unsafe {
                            sqlite3ErrorWithMsg(
                                db,
                                *__slate_slot_508,
                                (b"database schema is locked: %s\0".as_ptr() as *mut i8)
                                    as *const i8,
                                *__slate_slot_512,
                            )
                        };
                        {}
                        break '__join_0;
                    }
                }
                if (unsafe { (*db).pDisconnect }) != std::ptr::null_mut::<VTable>() {
                    unsafe { sqlite3VtabUnlockList(db) };
                }
                if nBytes >= (0 as i32)
                    && (nBytes == (0 as i32)
                        || ((unsafe { *unsafe { zSql.offset((nBytes - (1 as i32)) as isize) } })
                            as i32)
                            != (0 as i32))
                {
                    std::ptr::write(__slate_slot_514, unsafe {
                        *unsafe {
                            unsafe { (*db).aLimit.as_mut_ptr() as *mut i32 }
                                .offset((1 as i32) as isize)
                        }
                    });
                    {}
                    {}
                    if nBytes > *__slate_slot_514 {
                        unsafe {
                            sqlite3ErrorWithMsg(
                                db,
                                18 as i32,
                                (b"statement too long\0".as_ptr() as *mut i8) as *const i8,
                            )
                        };
                        *__slate_slot_508 = unsafe { sqlite3ApiExit(db, 18 as i32) };
                        break '__join_0;
                    } else {
                        *__slate_slot_513 =
                            unsafe { sqlite3DbStrNDup(db, zSql, (nBytes as i64) as u64) };
                        if *__slate_slot_513 != std::ptr::null_mut::<i8>() {
                            unsafe {
                                sqlite3RunParser(
                                    std::ptr::addr_of_mut!(*__slate_slot_510),
                                    *__slate_slot_513 as *const i8,
                                )
                            };
                            (*__slate_slot_510).zTail = unsafe {
                                zSql.offset(
                                    ((unsafe {
                                        (*__slate_slot_510)
                                            .zTail
                                            .offset_from(*__slate_slot_513 as *const i8)
                                    }) as i64) as isize,
                                )
                            };
                            unsafe { sqlite3DbFree(db, *__slate_slot_513 as *mut ()) };
                        } else {
                            (*__slate_slot_510).zTail = unsafe { zSql.offset(nBytes as isize) };
                        }
                    }
                } else {
                    unsafe { sqlite3RunParser(std::ptr::addr_of_mut!(*__slate_slot_510), zSql) };
                }
                0 as i32;
                if pzTail != std::ptr::null_mut::<*const i8>() {
                    unsafe {
                        *pzTail = (*__slate_slot_510).zTail;
                    }
                }
                if (((unsafe { (*db).init.busy }) as u32) as i32) == (0 as i32) {
                    unsafe {
                        sqlite3VdbeSetSql(
                            (*__slate_slot_510).pVdbe,
                            zSql,
                            ((unsafe { (*__slate_slot_510).zTail.offset_from(zSql as *const i8) })
                                as i64) as i32,
                            prepFlags as u8,
                        )
                    };
                }
                if (unsafe { (*db).mallocFailed }) != (0 as u8) {
                    (*__slate_slot_510).rc = 7 as i32;
                    (*__slate_slot_510)
                        .__slate_bits_0
                        .__set_checkSchema((0 as i32) as u32);
                }
                if (*__slate_slot_510).rc != (0 as i32) && (*__slate_slot_510).rc != (101 as i32) {
                    if ((*__slate_slot_510).__slate_bits_0.__get_checkSchema() as i32) != (0 as i32)
                        && (((unsafe { (*db).init.busy }) as u32) as i32) == (0 as i32)
                    {
                        schemaIsValid(std::ptr::addr_of_mut!(*__slate_slot_510));
                    }
                    if (*__slate_slot_510).pVdbe != std::ptr::null_mut::<Vdbe>() {
                        unsafe { sqlite3VdbeFinalize((*__slate_slot_510).pVdbe) };
                    }
                    0 as i32;
                    *__slate_slot_508 = (*__slate_slot_510).rc;
                    if (*__slate_slot_510).zErrMsg != std::ptr::null_mut::<i8>() {
                        unsafe {
                            sqlite3ErrorWithMsg(
                                db,
                                *__slate_slot_508,
                                (b"%s\0".as_ptr() as *mut i8) as *const i8,
                                (*__slate_slot_510).zErrMsg,
                            )
                        };
                        unsafe { sqlite3DbFree(db, (*__slate_slot_510).zErrMsg as *mut ()) };
                    } else {
                        unsafe { sqlite3Error(db, *__slate_slot_508) };
                    }
                } else {
                    0 as i32;
                    unsafe {
                        *ppStmt = (*__slate_slot_510).pVdbe as *mut sqlite3_stmt;
                    }
                    *__slate_slot_508 = 0 as i32;
                    unsafe { sqlite3ErrorClear(db) };
                }
                // Delete any TriggerPrg structures allocated while parsing this statement.
                loop {
                    if (*__slate_slot_510).pTriggerPrg != std::ptr::null_mut::<TriggerPrg>() {
                        std::ptr::write(__slate_slot_515, (*__slate_slot_510).pTriggerPrg);
                        (*__slate_slot_510).pTriggerPrg = unsafe { (*(*__slate_slot_515)).pNext };
                        unsafe { sqlite3DbFree(db, *__slate_slot_515 as *mut ()) };
                    } else {
                        break '__join_0;
                    }
                }
            }
        }
        sqlite3ParseObjectReset(std::ptr::addr_of_mut!(*__slate_slot_510));
        return *__slate_slot_508;
    }
    return unsafe { std::mem::zeroed() };
}

/// # Arguments
///
/// * `db` - Database handle.
/// * `zSql` - UTF-8 encoded SQL statement.
/// * `nBytes` - Length of zSql in bytes.
/// * `prepFlags` - Zero or more SQLITE_PREPARE_* flags
/// * `pOld` - VM being reprepared
/// * `ppStmt` - OUT: A pointer to the prepared statement
/// * `pzTail` - OUT: End of parsed string
fn sqlite3LockAndPrepare(
    mut db: *mut sqlite3,
    mut zSql: *const i8,
    mut nBytes: i32,
    mut prepFlags: u32,
    mut pOld: *mut Vdbe,
    mut ppStmt: *mut *mut sqlite3_stmt,
    mut pzTail: *mut *const i8,
) -> i32 {
    let mut rc: i32 = 0 as i32;
    let mut cnt: i32 = 0 as i32;
    unsafe {
        *ppStmt = std::ptr::null_mut::<sqlite3_stmt>();
    }
    if !((unsafe { sqlite3SafetyCheckOk(db) }) != (0 as i32)) || zSql == std::ptr::null::<i8>() {
        return unsafe { sqlite3MisuseError(871 as i32) };
    }
    unsafe { sqlite3_mutex_enter(unsafe { (*db).mutex }) };
    unsafe { sqlite3BtreeEnterAll(db) };
    '__slate_break_778: loop {
        // Make multiple attempts to compile the SQL, until it either succeeds
        // or encounters a permanent error.  A schema problem after one schema
        // reset is considered a permanent error.
        rc = sqlite3Prepare(db, zSql, nBytes, prepFlags, pOld, ppStmt, pzTail);
        0 as i32;
        if rc == (0 as i32) || (unsafe { (*db).mallocFailed }) != (0 as u8) {
            break '__slate_break_778;
        }
        let __v825: i32 = cnt;
        let __v826: i32 = __v825 + (1 as i32);
        cnt = __v826;
        let __v827: bool;
        if rc == (1 as i32) | (2 as i32) << (8 as i32) && cnt <= (25 as i32) {
            __v827 = true as bool;
        } else {
            let __v828: bool;
            if rc == (17 as i32) {
                unsafe { sqlite3ResetOneSchema(db, -(1 as i32)) };
                __v828 = cnt == (1 as i32);
            } else {
                __v828 = false as bool;
            }
            __v827 = __v828;
        }
        if !__v827 {
            break;
        }
    }
    unsafe { sqlite3BtreeLeaveAll(db) };
    0 as i32;
    rc = unsafe { sqlite3ApiExit(db, rc) };
    0 as i32;
    unsafe {
        (*db).busyHandler.nBusy = 0 as i32;
    }
    unsafe { sqlite3_mutex_leave(unsafe { (*db).mutex }) };
    0 as i32;
    return rc;
}

/// Rerun the compilation of a statement after a schema change.
///
/// If the statement is successfully recompiled, return SQLITE_OK. Otherwise,
/// if the statement cannot be recompiled because another connection has
/// locked the sqlite3_schema table, return SQLITE_LOCKED. If any other error
/// occurs, return SQLITE_SCHEMA.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3Reprepare(mut p: *mut Vdbe) -> i32 {
    let mut rc: i32 = 0 as i32;
    let mut pNew: *mut sqlite3_stmt = unsafe { std::mem::zeroed() };
    let mut zSql: *const i8 = unsafe { std::mem::zeroed() };
    let mut db: *mut sqlite3 = unsafe { std::mem::zeroed() };
    let mut prepFlags: u8 = 0 as u8;
    0 as i32;
    zSql = unsafe { sqlite3_sql(p as *mut sqlite3_stmt) };
    0 as i32; // Reprepare only called for prepare_v2() statements
    db = unsafe { sqlite3VdbeDb(p) };
    0 as i32;
    prepFlags = unsafe { sqlite3VdbePrepareFlags(p) };
    rc = sqlite3LockAndPrepare(
        db,
        zSql,
        -(1 as i32),
        prepFlags as u32,
        p,
        std::ptr::addr_of_mut!(pNew),
        std::ptr::null_mut::<*const i8>(),
    );
    if rc != (0 as i32) {
        if rc == (7 as i32) {
            unsafe { sqlite3OomFault(db) };
        }
        0 as i32;
        return rc;
    } else {
        0 as i32;
    }
    unsafe { sqlite3VdbeSwap(pNew as *mut Vdbe, p) };
    unsafe { sqlite3TransferBindings(pNew, p as *mut sqlite3_stmt) };
    unsafe { sqlite3VdbeResetStepResult(pNew as *mut Vdbe) };
    unsafe { sqlite3VdbeFinalize(pNew as *mut Vdbe) };
    return 0 as i32;
}

/// Two versions of the official API.  Legacy and new use.  In the legacy
/// version, the original SQL text is not saved in the prepared statement
/// and so if a schema change occurs, SQLITE_SCHEMA is returned by
/// sqlite3_step().  In the new version, the original SQL text is retained
/// and the statement is automatically recompiled if an schema change
/// occurs.
///
/// # Arguments
///
/// * `db` - Database handle.
/// * `zSql` - UTF-8 encoded SQL statement.
/// * `nBytes` - Length of zSql in bytes.
/// * `ppStmt` - OUT: A pointer to the prepared statement
/// * `pzTail` - OUT: End of parsed string
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.prepare.sqlite3_prepare")]
extern "C-unwind" fn sqlite3_prepare(
    mut db: *mut sqlite3,
    mut zSql: *const i8,
    mut nBytes: i32,
    mut ppStmt: *mut *mut sqlite3_stmt,
    mut pzTail: *mut *const i8,
) -> i32 {
    let mut rc: i32 = 0 as i32;
    rc = sqlite3LockAndPrepare(
        db,
        zSql,
        nBytes,
        (0 as i32) as u32,
        std::ptr::null_mut::<Vdbe>(),
        ppStmt,
        pzTail,
    );
    0 as i32; // VERIFY: F13021
    return rc;
}

/// # Arguments
///
/// * `db` - Database handle.
/// * `zSql` - UTF-8 encoded SQL statement.
/// * `nBytes` - Length of zSql in bytes.
/// * `ppStmt` - OUT: A pointer to the prepared statement
/// * `pzTail` - OUT: End of parsed string
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.prepare.sqlite3_prepare_v2")]
extern "C-unwind" fn sqlite3_prepare_v2(
    mut db: *mut sqlite3,
    mut zSql: *const i8,
    mut nBytes: i32,
    mut ppStmt: *mut *mut sqlite3_stmt,
    mut pzTail: *mut *const i8,
) -> i32 {
    let mut rc: i32 = 0 as i32;
    // EVIDENCE-OF: R-37923-12173 The sqlite3_prepare_v2() interface works
    // exactly the same as sqlite3_prepare_v3() with a zero prepFlags
    // parameter.
    //
    // Proof in that the 5th parameter to sqlite3LockAndPrepare is 0
    rc = sqlite3LockAndPrepare(
        db,
        zSql,
        nBytes,
        (128 as i32) as u32,
        std::ptr::null_mut::<Vdbe>(),
        ppStmt,
        pzTail,
    );
    0 as i32;
    return rc;
}

/// # Arguments
///
/// * `db` - Database handle.
/// * `zSql` - UTF-8 encoded SQL statement.
/// * `nBytes` - Length of zSql in bytes.
/// * `prepFlags` - Zero or more SQLITE_PREPARE_* flags
/// * `ppStmt` - OUT: A pointer to the prepared statement
/// * `pzTail` - OUT: End of parsed string
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.prepare.sqlite3_prepare_v3")]
extern "C-unwind" fn sqlite3_prepare_v3(
    mut db: *mut sqlite3,
    mut zSql: *const i8,
    mut nBytes: i32,
    mut prepFlags: u32,
    mut ppStmt: *mut *mut sqlite3_stmt,
    mut pzTail: *mut *const i8,
) -> i32 {
    let mut rc: i32 = 0 as i32;
    // EVIDENCE-OF: R-56861-42673 sqlite3_prepare_v3() differs from
    // sqlite3_prepare_v2() only in having the extra prepFlags parameter,
    // which is a bit array consisting of zero or more of the
    // SQLITE_PREPARE_* flags.
    //
    // Proof by comparison to the implementation of sqlite3_prepare_v2()
    // directly above.
    rc = sqlite3LockAndPrepare(
        db,
        zSql,
        nBytes,
        ((128 as i32) as u32) | prepFlags & ((63 as i32) as u32),
        std::ptr::null_mut::<Vdbe>(),
        ppStmt,
        pzTail,
    );
    0 as i32;
    return rc;
}

/// Compile the UTF-16 encoded SQL statement zSql into a statement handle.
///
/// # Arguments
///
/// * `db` - Database handle.
/// * `zSql` - UTF-16 encoded SQL statement.
/// * `nBytes` - Length of zSql in bytes.
/// * `prepFlags` - Zero or more SQLITE_PREPARE_* flags
/// * `ppStmt` - OUT: A pointer to the prepared statement
/// * `pzTail` - OUT: End of parsed string
fn sqlite3Prepare16(
    mut db: *mut sqlite3,
    mut zSql: *const (),
    mut nBytes: i32,
    mut prepFlags: u32,
    mut ppStmt: *mut *mut sqlite3_stmt,
    mut pzTail: *mut *const (),
) -> i32 {
    // This function currently works by first transforming the UTF-16
    // encoded string to UTF-8, then invoking sqlite3_prepare(). The
    // tricky bit is figuring out the pointer to return in *pzTail.
    let mut zSql8: *mut i8 = unsafe { std::mem::zeroed() };
    let mut zTail8: *const i8 = std::ptr::null::<i8>();
    let mut rc: i32 = 0 as i32;
    unsafe {
        *ppStmt = std::ptr::null_mut::<sqlite3_stmt>();
    }
    if !((unsafe { sqlite3SafetyCheckOk(db) }) != (0 as i32)) || zSql == std::ptr::null::<()>() {
        return unsafe { sqlite3MisuseError(1022 as i32) };
    }
    // Make sure nBytes is non-negative and correct.  It should be the
    // number of bytes until the end of the input buffer or until the first
    // U+0000 character.  If the input nBytes is odd, convert it into
    // an even number.  If the input nBytes is negative, then the input
    // must be terminated by at least one U+0000 character
    if nBytes >= (0 as i32) {
        let mut sz: i32 = 0 as i32;
        let mut z: *const i8 = zSql as *const i8;
        sz = 0 as i32;
        '__slate_break_779: loop {
            if !(sz < nBytes
                && (((unsafe { *unsafe { z.offset(sz as isize) } }) as i32) != (0 as i32)
                    || ((unsafe { *unsafe { z.offset((sz + (1 as i32)) as isize) } }) as i32)
                        != (0 as i32)))
            {
                break;
            }
            let __v829: i32 = sz;
            let __v830: i32 = __v829 + (2 as i32);
            sz = __v830;
        }
        nBytes = sz;
    } else {
        let mut sz: i32 = 0 as i32;
        let mut z: *const i8 = zSql as *const i8;
        sz = 0 as i32;
        '__slate_break_780: loop {
            if !(((unsafe { *unsafe { z.offset(sz as isize) } }) as i32) != (0 as i32)
                || ((unsafe { *unsafe { z.offset((sz + (1 as i32)) as isize) } }) as i32)
                    != (0 as i32))
            {
                break;
            }
            let __v831: i32 = sz;
            let __v832: i32 = __v831 + (2 as i32);
            sz = __v832;
        }
        nBytes = sz;
    }
    unsafe { sqlite3_mutex_enter(unsafe { (*db).mutex }) };
    zSql8 = unsafe { sqlite3Utf16to8(db, zSql, nBytes, ((2 as i32) as i8) as u8) };
    if zSql8 != std::ptr::null_mut::<i8>() {
        rc = sqlite3LockAndPrepare(
            db,
            zSql8 as *const i8,
            -(1 as i32),
            prepFlags,
            std::ptr::null_mut::<Vdbe>(),
            ppStmt,
            std::ptr::addr_of_mut!(zTail8),
        );
    }
    if zTail8 != std::ptr::null::<i8>() && pzTail != std::ptr::null_mut::<*const ()>() {
        // If sqlite3_prepare returns a tail pointer, we calculate the
        // equivalent pointer into the UTF-16 string by counting the unicode
        // characters between zSql8 and zTail8, and then returning a pointer
        // the same number of characters into the UTF-16 string.
        let mut chars_parsed: i32 = unsafe {
            sqlite3Utf8CharLen(
                zSql8 as *const i8,
                ((unsafe { zTail8.offset_from(zSql8 as *const i8) }) as i64) as i32,
            )
        };
        unsafe {
            *pzTail = (unsafe {
                (zSql as *mut u8)
                    .offset((unsafe { sqlite3Utf16ByteLen(zSql, nBytes, chars_parsed) }) as isize)
            }) as *const ();
        }
    }
    unsafe { sqlite3DbFree(db, zSql8 as *mut ()) };
    rc = unsafe { sqlite3ApiExit(db, rc) };
    unsafe { sqlite3_mutex_leave(unsafe { (*db).mutex }) };
    return rc;
}

/// Two versions of the official API.  Legacy and new use.  In the legacy
/// version, the original SQL text is not saved in the prepared statement
/// and so if a schema change occurs, SQLITE_SCHEMA is returned by
/// sqlite3_step().  In the new version, the original SQL text is retained
/// and the statement is automatically recompiled if an schema change
/// occurs.
///
/// # Arguments
///
/// * `db` - Database handle.
/// * `zSql` - UTF-16 encoded SQL statement.
/// * `nBytes` - Length of zSql in bytes.
/// * `ppStmt` - OUT: A pointer to the prepared statement
/// * `pzTail` - OUT: End of parsed string
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.prepare.sqlite3_prepare16")]
extern "C-unwind" fn sqlite3_prepare16(
    mut db: *mut sqlite3,
    mut zSql: *const (),
    mut nBytes: i32,
    mut ppStmt: *mut *mut sqlite3_stmt,
    mut pzTail: *mut *const (),
) -> i32 {
    let mut rc: i32 = 0 as i32;
    rc = sqlite3Prepare16(
        db,
        zSql,
        nBytes & !(1 as i32),
        (0 as i32) as u32,
        ppStmt,
        pzTail,
    );
    0 as i32; // VERIFY: F13021
    return rc;
}

/// # Arguments
///
/// * `db` - Database handle.
/// * `zSql` - UTF-16 encoded SQL statement.
/// * `nBytes` - Length of zSql in bytes.
/// * `ppStmt` - OUT: A pointer to the prepared statement
/// * `pzTail` - OUT: End of parsed string
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.prepare.sqlite3_prepare16_v2")]
extern "C-unwind" fn sqlite3_prepare16_v2(
    mut db: *mut sqlite3,
    mut zSql: *const (),
    mut nBytes: i32,
    mut ppStmt: *mut *mut sqlite3_stmt,
    mut pzTail: *mut *const (),
) -> i32 {
    let mut rc: i32 = 0 as i32;
    rc = sqlite3Prepare16(
        db,
        zSql,
        nBytes & !(1 as i32),
        (128 as i32) as u32,
        ppStmt,
        pzTail,
    );
    0 as i32; // VERIFY: F13021
    return rc;
}

/// # Arguments
///
/// * `db` - Database handle.
/// * `zSql` - UTF-16 encoded SQL statement.
/// * `nBytes` - Length of zSql in bytes.
/// * `prepFlags` - Zero or more SQLITE_PREPARE_* flags
/// * `ppStmt` - OUT: A pointer to the prepared statement
/// * `pzTail` - OUT: End of parsed string
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.prepare.sqlite3_prepare16_v3")]
extern "C-unwind" fn sqlite3_prepare16_v3(
    mut db: *mut sqlite3,
    mut zSql: *const (),
    mut nBytes: i32,
    mut prepFlags: u32,
    mut ppStmt: *mut *mut sqlite3_stmt,
    mut pzTail: *mut *const (),
) -> i32 {
    let mut rc: i32 = 0 as i32;
    rc = sqlite3Prepare16(
        db,
        zSql,
        nBytes & !(1 as i32),
        ((128 as i32) as u32) | prepFlags & ((63 as i32) as u32),
        ppStmt,
        pzTail,
    );
    0 as i32; // VERIFY: F13021
    return rc;
}
