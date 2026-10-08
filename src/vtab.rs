//! 2006 June 10
//!
//! The author disclaims copyright to this source code.  In place of
//! a legal notice, here is a blessing:
//!
//!    May you do good and not evil.
//!    May you find forgiveness for yourself and forgive others.
//!    May you share freely, never taking more than you give.
//!
//!
//! This file contains code used to help implement virtual tables.
unsafe extern "C" {
    fn sqlite3_free(__v606: *mut ());
    fn sqlite3_mutex_enter(__v620: *mut sqlite3_mutex);
    fn sqlite3_mutex_leave(__v621: *mut sqlite3_mutex);
    fn sqlite3_strnicmp(__v622: *const i8, __v623: *const i8, __v624: i32) -> i32;
    fn sqlite3HashInsert(__v628: *mut Hash, pKey: *const i8, pData: *mut ()) -> *mut ();
    fn sqlite3HashFind(__v631: *const Hash, pKey: *const i8) -> *mut ();
    fn memcpy(__dest: *mut (), __src: *const (), __n: u64) -> *mut ();
    fn memset(__s: *mut (), __c: i32, __n: u64) -> *mut ();
    fn strcmp(__s1: *const i8, __s2: *const i8) -> i32;
    fn sqlite3VdbeAddOp0(__v641: *mut Vdbe, __v642: i32) -> i32;
    fn sqlite3VdbeAddOp2(__v643: *mut Vdbe, __v644: i32, __v645: i32, __v646: i32) -> i32;
    fn sqlite3VdbeLoadString(__v647: *mut Vdbe, __v648: i32, __v649: *const i8) -> i32;
    fn sqlite3VdbeAddParseSchemaOp(__v650: *mut Vdbe, __v651: i32, __v652: *mut i8, __v653: u16);
    fn sqlite3VdbeFinalize(__v654: *mut Vdbe) -> i32;
    fn sqlite3MisuseError(__v655: i32) -> i32;
    fn sqlite3Strlen30(__v656: *const i8) -> i32;
    fn sqlite3ColumnType(__v657: *mut Column, __v658: *mut i8) -> *mut i8;
    fn sqlite3Malloc(__v659: u64) -> *mut ();
    fn sqlite3MallocZero(__v660: u64) -> *mut ();
    fn sqlite3DbMallocZero(__v661: *mut sqlite3, __v662: u64) -> *mut ();
    fn sqlite3DbStrDup(__v663: *mut sqlite3, __v664: *const i8) -> *mut i8;
    fn sqlite3DbStrNDup(__v665: *mut sqlite3, __v666: *const i8, __v667: u64) -> *mut i8;
    fn sqlite3Realloc(__v668: *mut (), __v669: u64) -> *mut ();
    fn sqlite3DbRealloc(__v670: *mut sqlite3, __v671: *mut (), __v672: u64) -> *mut ();
    fn sqlite3DbFree(__v673: *mut sqlite3, __v674: *mut ());
    fn sqlite3MPrintf(__v675: *mut sqlite3, __v676: *const i8, ...) -> *mut i8;
    fn sqlite3ErrorMsg(__v677: *mut Parse, __v678: *const i8, ...);
    fn sqlite3RunParser(__v679: *mut Parse, __v680: *const i8) -> i32;
    fn sqlite3ExprListDelete(__v681: *mut sqlite3, __v682: *mut ExprList);
    fn sqlite3PrimaryKeyIndex(__v683: *mut Table) -> *mut Index;
    fn sqlite3StartTable(
        __v684: *mut Parse,
        __v685: *mut Token,
        __v686: *mut Token,
        __v687: i32,
        __v688: i32,
        __v689: i32,
        __v690: i32,
    );
    fn sqlite3DeleteTable(__v691: *mut sqlite3, __v692: *mut Table);
    fn sqlite3FindTable(__v693: *mut sqlite3, __v694: *const i8, __v695: *const i8) -> *mut Table;
    fn sqlite3NameFromToken(__v696: *mut sqlite3, __v697: *const Token) -> *mut i8;
    fn sqlite3GetVdbe(__v698: *mut Parse) -> *mut Vdbe;
    fn sqlite3MayAbort(__v699: *mut Parse);
    fn sqlite3ChangeCookie(__v700: *mut Parse, __v701: i32);
    fn sqlite3AuthCheck(
        __v702: *mut Parse,
        __v703: i32,
        __v704: *const i8,
        __v705: *const i8,
        __v706: *const i8,
    ) -> i32;
    fn sqlite3ErrorWithMsg(__v707: *mut sqlite3, __v708: i32, __v709: *const i8, ...);
    fn sqlite3Error(__v710: *mut sqlite3, __v711: i32);
    fn sqlite3GetToken(__v712: *const u8, __v713: *mut i32) -> i64;
    fn sqlite3NestedParse(__v714: *mut Parse, __v715: *const i8, ...);
    fn sqlite3SchemaToIndex(db: *mut sqlite3, __v717: *mut Schema) -> i32;
    fn sqlite3OomFault(__v718: *mut sqlite3) -> *mut ();
    fn sqlite3ApiExit(db: *mut sqlite3, __v720: i32) -> i32;
    fn sqlite3VtabImportErrmsg(__v737: *mut Vdbe, __v738: *mut sqlite3_vtab);
    fn sqlite3MarkAllShadowTablesOf(__v746: *mut sqlite3, __v747: *mut Table);
    fn sqlite3ParseObjectInit(__v779: *mut Parse, __v780: *mut sqlite3);
    fn sqlite3ParseObjectReset(__v781: *mut Parse);
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
    trace: __SlateRecord157,
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
    u1: __SlateRecord158,
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
    u: __SlateRecord159,
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
    u: __SlateRecord160,
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
    u: __SlateRecord168,
    pLeft: *mut Expr,
    pRight: *mut Expr,
    x: __SlateRecord169,
    nHeight: i32,
    iTable: i32,
    iColumn: i16,
    iAgg: i16,
    w: __SlateRecord170,
    pAggInfo: *mut AggInfo,
    y: __SlateRecord171,
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
    fg: __SlateRecord178,
    iCursor: i32,
    colUsed: u64,
    u1: __SlateRecord179,
    u2: __SlateRecord180,
    u3: __SlateRecord181,
    u4: __SlateRecord182,
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

/// Before a virtual table xCreate() or xConnect() method is invoked, the
/// sqlite3.pVtabCtx member variable is set to point to an instance of
/// this struct allocated on the stack. It is used by the implementation of
/// the sqlite3_declare_vtab() and sqlite3_vtab_config() APIs, both of which
/// are invoked only from within xCreate and xConnect methods.
#[repr(C)]
#[derive(Clone, Copy)]
struct VtabCtx {
    /// The virtual table being constructed
    pVTable: *mut VTable,
    /// The Table object to which the virtual table belongs
    pTab: *mut Table,
    /// Parent context (if any)
    pPrior: *mut VtabCtx,
    /// True after sqlite3_declare_vtab() is called
    bDeclared: i32,
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
    __slate_bits_0: __slate_bits::__SlateBits156U0,
    azInit: *mut *const i8,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord157 {
    xLegacy: Option<unsafe extern "C-unwind" fn(*mut (), *const i8)>,
    xV2: Option<unsafe extern "C-unwind" fn(u32, *mut (), *mut (), *mut ()) -> i32>,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord158 {
    isInterrupted: i32,
    notUsed1: f64,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord159 {
    pHash: *mut FuncDef,
    pDestructor: *mut FuncDestructor,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord160 {
    tab: __SlateRecord161,
    view: __SlateRecord162,
    vtab: __SlateRecord163,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord161 {
    addColOffset: i32,
    pFKey: *mut FKey,
    pDfltList: *mut ExprList,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord162 {
    pSelect: *mut Select,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord163 {
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
union __SlateRecord168 {
    zToken: *mut i8,
    iValue: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord169 {
    pList: *mut ExprList,
    pSelect: *mut Select,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord170 {
    iJoin: i32,
    iOfst: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord171 {
    pTab: *mut Table,
    pWin: *mut Window,
    nReg: i32,
    sub: __SlateRecord172,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord172 {
    iAddr: i32,
    regReturn: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct ExprList_item {
    pExpr: *mut Expr,
    zEName: *mut i8,
    fg: __SlateRecord174,
    u: __SlateRecord175,
}

#[repr(C, align(4))]
#[derive(Clone, Copy)]
struct __SlateRecord174 {
    sortFlags: u8,
    __slate_bits_0: __slate_bits::__SlateBits174U0,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord175 {
    x: __SlateRecord176,
    iConstExprReg: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord176 {
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
struct __SlateRecord178 {
    jointype: u8,
    __slate_bits_0: __slate_bits::__SlateBits178U0,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord179 {
    zIndexedBy: *mut i8,
    pFuncArg: *mut ExprList,
    nRow: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord180 {
    pIBIndex: *mut Index,
    pCteUse: *mut CteUse,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord181 {
    pOn: *mut Expr,
    pUsing: *mut IdList,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord182 {
    pSchema: *mut Schema,
    zDatabase: *mut i8,
    pSubq: *mut Subquery,
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
    pub struct __SlateBits174U0 {
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
    pub struct __SlateBits178U0 {
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
    pub struct __SlateBits156U0 {
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

/// Construct and install a Module object for a virtual table.  When this
/// routine is called, it is guaranteed that all appropriate locks are held
/// and the module is not already part of the connection.
///
/// If there already exists a module with zName, replace it with the new one.
/// If pModule==0, then delete the module zName if it exists.
///
/// # Arguments
///
/// * `db` - Database in which module is registered
/// * `zName` - Name assigned to this module
/// * `pModule` - The definition of the module
/// * `pAux` - Context pointer for xCreate/xConnect
/// * `xDestroy` - Module destructor function
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3VtabCreateModule(
    mut db: *mut sqlite3,
    mut zName: *const i8,
    mut pModule: *const sqlite3_module,
    mut pAux: *mut (),
    mut xDestroy: Option<unsafe extern "C-unwind" fn(*mut ())>,
) -> *mut Module {
    let mut pMod: *mut Module = unsafe { std::mem::zeroed() };
    let mut pDel: *mut Module = unsafe { std::mem::zeroed() };
    let mut zCopy: *mut i8 = unsafe { std::mem::zeroed() };
    if pModule == std::ptr::null::<sqlite3_module>() {
        zCopy = zName as *mut i8;
        pMod = std::ptr::null_mut::<Module>();
    } else {
        let mut nName: i32 = unsafe { sqlite3Strlen30(zName) };
        pMod = (unsafe {
            sqlite3Malloc(
                (48 as u64)
                    .wrapping_add((nName as i64) as u64)
                    .wrapping_add(((1 as i32) as i64) as u64),
            )
        }) as *mut Module;
        if pMod == std::ptr::null_mut::<Module>() {
            unsafe { sqlite3OomFault(db) };
            return std::ptr::null_mut::<Module>();
        }
        zCopy = (unsafe { pMod.offset((1 as i32) as isize) }) as *mut i8;
        unsafe {
            memcpy(
                zCopy as *mut (),
                zName as *const (),
                ((nName + (1 as i32)) as i64) as u64,
            )
        };
        unsafe {
            (*pMod).zName = zCopy as *const i8;
        }
        unsafe {
            (*pMod).pModule = pModule;
        }
        unsafe {
            (*pMod).pAux = pAux;
        }
        unsafe {
            (*pMod).xDestroy = xDestroy;
        }
        unsafe {
            (*pMod).pEpoTab = std::ptr::null_mut::<Table>();
        }
        unsafe {
            (*pMod).nRefModule = 1 as i32;
        }
    }
    pDel = (unsafe {
        sqlite3HashInsert(
            unsafe { std::ptr::addr_of_mut!((*db).aModule) },
            zCopy as *const i8,
            pMod as *mut (),
        )
    }) as *mut Module;
    if pDel != std::ptr::null_mut::<Module>() {
        if pDel == pMod {
            unsafe { sqlite3OomFault(db) };
            unsafe { sqlite3DbFree(db, pDel as *mut ()) };
            pMod = std::ptr::null_mut::<Module>();
        } else {
            sqlite3VtabEponymousTableClear(db, pDel);
            sqlite3VtabModuleUnref(db, pDel);
        }
    }
    return pMod;
}

/// The actual function that does the work of creating a new module.
/// This function implements the sqlite3_create_module() and
/// sqlite3_create_module_v2() interfaces.
///
/// # Arguments
///
/// * `db` - Database in which module is registered
/// * `zName` - Name assigned to this module
/// * `pModule` - The definition of the module
/// * `pAux` - Context pointer for xCreate/xConnect
/// * `xDestroy` - Module destructor function
fn createModule(
    mut db: *mut sqlite3,
    mut zName: *const i8,
    mut pModule: *const sqlite3_module,
    mut pAux: *mut (),
    mut xDestroy: Option<unsafe extern "C-unwind" fn(*mut ())>,
) -> i32 {
    let mut rc: i32 = 0 as i32;
    unsafe { sqlite3_mutex_enter(unsafe { (*db).mutex }) };
    sqlite3VtabCreateModule(db, zName, pModule, pAux, xDestroy);
    rc = unsafe { sqlite3ApiExit(db, rc) };
    if rc != (0 as i32) && xDestroy != None {
        unsafe { xDestroy.unwrap()(pAux) };
    }
    unsafe { sqlite3_mutex_leave(unsafe { (*db).mutex }) };
    return rc;
}

/// External API function used to create a new virtual-table module.
///
/// # Arguments
///
/// * `db` - Database in which module is registered
/// * `zName` - Name assigned to this module
/// * `pModule` - The definition of the module
/// * `pAux` - Context pointer for xCreate/xConnect
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.vtab.sqlite3_create_module")]
extern "C-unwind" fn sqlite3_create_module(
    mut db: *mut sqlite3,
    mut zName: *const i8,
    mut pModule: *const sqlite3_module,
    mut pAux: *mut (),
) -> i32 {
    return createModule(db, zName, pModule, pAux, None);
}

/// External API function used to create a new virtual-table module.
///
/// # Arguments
///
/// * `db` - Database in which module is registered
/// * `zName` - Name assigned to this module
/// * `pModule` - The definition of the module
/// * `pAux` - Context pointer for xCreate/xConnect
/// * `xDestroy` - Module destructor function
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.vtab.sqlite3_create_module_v2")]
extern "C-unwind" fn sqlite3_create_module_v2(
    mut db: *mut sqlite3,
    mut zName: *const i8,
    mut pModule: *const sqlite3_module,
    mut pAux: *mut (),
    mut xDestroy: Option<unsafe extern "C-unwind" fn(*mut ())>,
) -> i32 {
    return createModule(db, zName, pModule, pAux, xDestroy);
}

/// External API to drop all virtual-table modules, except those named
/// on the azNames list.
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.vtab.sqlite3_drop_modules")]
extern "C-unwind" fn sqlite3_drop_modules(
    mut db: *mut sqlite3,
    mut azNames: *mut *const i8,
) -> i32 {
    let mut pThis: *mut HashElem = unsafe { std::mem::zeroed() };
    let mut pNext: *mut HashElem = unsafe { std::mem::zeroed() };
    unsafe { sqlite3_mutex_enter(unsafe { (*db).mutex }) };
    pThis = unsafe { (*unsafe { std::ptr::addr_of_mut!((*db).aModule) }).first };
    '__slate_break_782: while pThis != std::ptr::null_mut::<HashElem>() {
        '__slate_continue_782: {
            let mut pMod: *mut Module = (unsafe { (*pThis).data }) as *mut Module;
            pNext = unsafe { (*pThis).next };
            if azNames != std::ptr::null_mut::<*const i8>() {
                let mut ii: i32 = 0 as i32;
                ii = 0 as i32;
                '__slate_break_783: loop {
                    if !((unsafe { *unsafe { azNames.offset(ii as isize) } })
                        != std::ptr::null::<i8>()
                        && (unsafe {
                            strcmp(unsafe { *unsafe { azNames.offset(ii as isize) } }, unsafe {
                                (*pMod).zName
                            })
                        }) != (0 as i32))
                    {
                        break;
                    }
                    let __v819: i32 = ii;
                    let __v820: i32 = __v819 + (1 as i32);
                    ii = __v820;
                }
                if (unsafe { *unsafe { azNames.offset(ii as isize) } }) != std::ptr::null::<i8>() {
                    break '__slate_continue_782;
                }
            }
            createModule(
                db,
                unsafe { (*pMod).zName },
                std::ptr::null::<sqlite3_module>(),
                std::ptr::null_mut::<()>(),
                None,
            );
        }
        pThis = pNext;
    }
    unsafe { sqlite3_mutex_leave(unsafe { (*db).mutex }) };
    return 0 as i32;
}

/// Decrement the reference count on a Module object.  Destroy the
/// module when the reference count reaches zero.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3VtabModuleUnref(mut db: *mut sqlite3, mut pMod: *mut Module) {
    0 as i32;
    let __v842: *mut Module = pMod;
    let __v843: i32 = unsafe { (*__v842).nRefModule };
    let __v844: i32 = __v843 - (1 as i32);
    unsafe {
        (*__v842).nRefModule = __v844;
    }
    if (unsafe { (*pMod).nRefModule }) == (0 as i32) {
        if (unsafe { (*pMod).xDestroy }) != None {
            unsafe { unsafe { (*pMod).xDestroy }.unwrap()(unsafe { (*pMod).pAux }) };
        }
        0 as i32;
        unsafe { sqlite3DbFree(db, pMod as *mut ()) };
    }
}

/// Lock the virtual table so that it cannot be disconnected.
/// Locks nest.  Every lock should have a corresponding unlock.
/// If an unlock is omitted, resources leaks will occur.
///
/// If a disconnect is attempted while a virtual table is locked,
/// the disconnect is deferred until all locks have been removed.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3VtabLock(mut pVTab: *mut VTable) {
    let __v836: *mut VTable = pVTab;
    let __v837: i32 = unsafe { (*__v836).nRef };
    let __v838: i32 = __v837 + (1 as i32);
    unsafe {
        (*__v836).nRef = __v838;
    }
}

/// pTab is a pointer to a Table structure representing a virtual-table.
/// Return a pointer to the VTable object used by connection db to access
/// this virtual-table, if one has been created, or NULL otherwise.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3GetVTable(mut db: *mut sqlite3, mut pTab: *mut Table) -> *mut VTable {
    let mut pVtab: *mut VTable = unsafe { std::mem::zeroed() };
    0 as i32;
    pVtab = unsafe { (*pTab).u.vtab.p };
    '__slate_break_784: while pVtab != std::ptr::null_mut::<VTable>()
        && (unsafe { (*pVtab).db }) != db
    {
        {}
        pVtab = unsafe { (*pVtab).pNext };
    }
    return pVtab;
}

/// Decrement the ref-count on a virtual table object. When the ref-count
/// reaches zero, call the xDisconnect() method to delete the object.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3VtabUnlock(mut pVTab: *mut VTable) {
    let mut db: *mut sqlite3 = unsafe { (*pVTab).db };
    0 as i32;
    0 as i32;
    0 as i32;
    let __v839: *mut VTable = pVTab;
    let __v840: i32 = unsafe { (*__v839).nRef };
    let __v841: i32 = __v840 - (1 as i32);
    unsafe {
        (*__v839).nRef = __v841;
    }
    if (unsafe { (*pVTab).nRef }) == (0 as i32) {
        let mut p: *mut sqlite3_vtab = unsafe { (*pVTab).pVtab };
        if p != std::ptr::null_mut::<sqlite3_vtab>() {
            unsafe { unsafe { (*unsafe { (*p).pModule }).xDisconnect }.unwrap()(p) };
        }
        sqlite3VtabModuleUnref(unsafe { (*pVTab).db }, unsafe { (*pVTab).pMod });
        unsafe { sqlite3DbFree(db, pVTab as *mut ()) };
    }
}

/// Table p is a virtual table. This function moves all elements in the
/// p->u.vtab.p list to the sqlite3.pDisconnect lists of their associated
/// database connections to be disconnected at the next opportunity.
/// Except, if argument db is not NULL, then the entry associated with
/// connection db is left in the p->u.vtab.p list.
fn vtabDisconnectAll(mut db: *mut sqlite3, mut p: *mut Table) -> *mut VTable {
    let mut pRet: *mut VTable = std::ptr::null_mut::<VTable>();
    let mut pVTable: *mut VTable = unsafe { std::mem::zeroed() };
    0 as i32;
    pVTable = unsafe { (*p).u.vtab.p };
    unsafe {
        (*p).u.vtab.p = std::ptr::null_mut::<VTable>();
    }
    // Assert that the mutex (if any) associated with the BtShared database
    // that contains table p is held by the caller. See header comments
    // above function sqlite3VtabUnlockList() for an explanation of why
    // this makes it safe to access the sqlite3.pDisconnect list of any
    // database connection that may have an entry in the p->u.vtab.p list.
    0 as i32;
    '__slate_break_785: while pVTable != std::ptr::null_mut::<VTable>() {
        let mut db2: *mut sqlite3 = unsafe { (*pVTable).db };
        let mut pNext: *mut VTable = unsafe { (*pVTable).pNext };
        0 as i32;
        if db2 == db {
            pRet = pVTable;
            unsafe {
                (*p).u.vtab.p = pRet;
            }
            unsafe {
                (*pRet).pNext = std::ptr::null_mut::<VTable>();
            }
        } else {
            unsafe {
                (*pVTable).pNext = unsafe { (*db2).pDisconnect };
            }
            unsafe {
                (*db2).pDisconnect = pVTable;
            }
        }
        pVTable = pNext;
    }
    0 as i32;
    return pRet;
}

/// Table *p is a virtual table. This function removes the VTable object
/// for table *p associated with database connection db from the linked
/// list in p->pVTab. It also decrements the VTable ref count. This is
/// used when closing database connection db to free all of its VTable
/// objects without disturbing the rest of the Schema object (which may
/// be being used by other shared-cache connections).
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3VtabDisconnect(mut db: *mut sqlite3, mut p: *mut Table) {
    let mut ppVTab: *mut *mut VTable = unsafe { std::mem::zeroed() };
    0 as i32;
    0 as i32;
    0 as i32;
    ppVTab = unsafe { std::ptr::addr_of_mut!((*p).u.vtab.p) };
    '__slate_break_786: while (unsafe { *ppVTab }) != std::ptr::null_mut::<VTable>() {
        if (unsafe { (*unsafe { *ppVTab }).db }) == db {
            let mut pVTab: *mut VTable = unsafe { *ppVTab };
            unsafe {
                *ppVTab = unsafe { (*pVTab).pNext };
            }
            sqlite3VtabUnlock(pVTab);
            break '__slate_break_786;
        }
        ppVTab = unsafe { std::ptr::addr_of_mut!((*unsafe { *ppVTab }).pNext) };
    }
}

/// Disconnect all the virtual table objects in the sqlite3.pDisconnect list.
///
/// This function may only be called when the mutexes associated with all
/// shared b-tree databases opened using connection db are held by the
/// caller. This is done to protect the sqlite3.pDisconnect list. The
/// sqlite3.pDisconnect list is accessed only as follows:
///
///   1) By this function. In this case, all BtShared mutexes and the mutex
///      associated with the database handle itself must be held.
///
///   2) By function vtabDisconnectAll(), when it adds a VTable entry to
///      the sqlite3.pDisconnect list. In this case either the BtShared mutex
///      associated with the database the virtual table is stored in is held
///      or, if the virtual table is stored in a non-sharable database, then
///      the database handle mutex is held.
///
/// As a result, a sqlite3.pDisconnect cannot be accessed simultaneously
/// by multiple threads. It is thread-safe.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3VtabUnlockList(mut db: *mut sqlite3) {
    let mut p: *mut VTable = unsafe { (*db).pDisconnect };
    0 as i32;
    0 as i32;
    if p != std::ptr::null_mut::<VTable>() {
        unsafe {
            (*db).pDisconnect = std::ptr::null_mut::<VTable>();
        }
        '__slate_break_787: loop {
            let mut pNext: *mut VTable = unsafe { (*p).pNext };
            sqlite3VtabUnlock(p);
            p = pNext;
            if !(p != std::ptr::null_mut::<VTable>()) {
                break;
            }
        }
    }
}

/// Clear any and all virtual-table information from the Table record.
/// This routine is called, for example, just before deleting the Table
/// record.
///
/// Since it is a virtual-table, the Table structure contains a pointer
/// to the head of a linked list of VTable structures. Each VTable
/// structure is associated with a single sqlite3* user of the schema.
/// The reference count of the VTable structure associated with database
/// connection db is decremented immediately (which may lead to the
/// structure being xDisconnected and free). Any other VTable structures
/// in the list are moved to the sqlite3.pDisconnect list of the associated
/// database connection.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3VtabClear(mut db: *mut sqlite3, mut p: *mut Table) {
    0 as i32;
    0 as i32;
    if (unsafe { (*db).pnBytesFreed }) == std::ptr::null_mut::<i32>() {
        vtabDisconnectAll(std::ptr::null_mut::<sqlite3>(), p);
    }
    if (unsafe { (*p).u.vtab.azArg }) != std::ptr::null_mut::<*mut i8>() {
        let mut i: i32 = 0 as i32;
        i = 0 as i32;
        '__slate_break_788: loop {
            if !(i < unsafe { (*p).u.vtab.nArg }) {
                break;
            }
            if i != (1 as i32) {
                unsafe {
                    sqlite3DbFree(
                        db,
                        (unsafe { *unsafe { unsafe { (*p).u.vtab.azArg }.offset(i as isize) } })
                            as *mut (),
                    )
                };
            }
            let __v830: i32 = i;
            let __v831: i32 = __v830 + (1 as i32);
            i = __v831;
        }
        unsafe { sqlite3DbFree(db, (unsafe { (*p).u.vtab.azArg }) as *mut ()) };
    }
}

/// Add a new module argument to pTable->u.vtab.azArg[].
/// The string is not copied - the pointer is stored.  The
/// string will be freed automatically when the table is
/// deleted.
fn addModuleArgument(mut pParse: *mut Parse, mut pTable: *mut Table, mut zArg: *mut i8) {
    let mut nBytes: i64 = 0 as i64;
    let mut azModuleArg: *mut *mut i8 = unsafe { std::mem::zeroed() };
    let mut db: *mut sqlite3 = unsafe { (*pParse).db };
    0 as i32;
    nBytes = (8 as u64)
        .wrapping_mul((((2 as i32) + unsafe { (*pTable).u.vtab.nArg }) as i64) as u64)
        as i64;
    if (unsafe { (*pTable).u.vtab.nArg }) + (3 as i32)
        >= unsafe {
            *unsafe { unsafe { (*db).aLimit.as_mut_ptr() as *mut i32 }.offset((2 as i32) as isize) }
        }
    {
        unsafe {
            sqlite3ErrorMsg(
                pParse,
                (b"too many columns on %s\0".as_ptr() as *mut i8) as *const i8,
                unsafe { (*pTable).zName },
            )
        };
    }
    azModuleArg = (unsafe {
        sqlite3DbRealloc(
            db,
            (unsafe { (*pTable).u.vtab.azArg }) as *mut (),
            nBytes as u64,
        )
    }) as *mut *mut i8;
    if azModuleArg == std::ptr::null_mut::<*mut i8>() {
        unsafe { sqlite3DbFree(db, zArg as *mut ()) };
    } else {
        let mut i: i32 = 0 as i32;
        let __v882: *mut Table = pTable;
        let __v883: i32 = unsafe { (*__v882).u.vtab.nArg };
        let __v884: i32 = __v883 + (1 as i32);
        unsafe {
            (*__v882).u.vtab.nArg = __v884;
        }
        i = __v883;
        unsafe {
            *unsafe { azModuleArg.offset(i as isize) } = zArg;
        }
        unsafe {
            *unsafe { azModuleArg.offset((i + (1 as i32)) as isize) } = std::ptr::null_mut::<i8>();
        }
        unsafe {
            (*pTable).u.vtab.azArg = azModuleArg;
        }
    }
}

/// The parser calls this routine when it first sees a CREATE VIRTUAL TABLE
/// statement.  The module name has been parsed, but the optional list
/// of parameters that follow the module name are still pending.
///
/// # Arguments
///
/// * `pParse` - Parsing context
/// * `pName1` - Name of new table, or database name
/// * `pName2` - Name of new table or NULL
/// * `pModuleName` - Name of the module for the virtual table
/// * `ifNotExists` - No error if the table already exists
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3VtabBeginParse(
    mut pParse: *mut Parse,
    mut pName1: *mut Token,
    mut pName2: *mut Token,
    mut pModuleName: *mut Token,
    mut ifNotExists: i32,
) {
    let mut pTable: *mut Table = unsafe { std::mem::zeroed() }; // The new virtual table
    let mut db: *mut sqlite3 = unsafe { std::mem::zeroed() }; // Database connection
    unsafe {
        sqlite3StartTable(
            pParse,
            pName1,
            pName2,
            0 as i32,
            0 as i32,
            1 as i32,
            ifNotExists,
        )
    };
    pTable = unsafe { (*pParse).pNewTable };
    if pTable == std::ptr::null_mut::<Table>() {
        return;
    }
    0 as i32;
    unsafe {
        (*pTable).eTabType = ((1 as i32) as i8) as u8;
    }
    db = unsafe { (*pParse).db };
    0 as i32;
    addModuleArgument(pParse, pTable, unsafe {
        sqlite3NameFromToken(db, pModuleName as *const Token)
    });
    addModuleArgument(pParse, pTable, std::ptr::null_mut::<i8>());
    addModuleArgument(pParse, pTable, unsafe {
        sqlite3DbStrDup(db, (unsafe { (*pTable).zName }) as *const i8)
    });
    0 as i32;
    unsafe {
        (*pParse).sNameToken.n = (((unsafe {
            unsafe { unsafe { (*pModuleName).z }.offset((unsafe { (*pModuleName).n }) as isize) }
                .offset_from((unsafe { (*pParse).sNameToken.z }) as *const i8)
        }) as i64) as i32) as u32;
    }
    // Creating a virtual table invokes the authorization callback twice.
    // The first invocation, to obtain permission to INSERT a row into the
    // sqlite_schema table, has already been made by sqlite3StartTable().
    // The second call, to obtain permission to create the table, is made now.
    if (unsafe { (*pTable).u.vtab.azArg }) != std::ptr::null_mut::<*mut i8>() {
        let mut iDb: i32 = unsafe { sqlite3SchemaToIndex(db, unsafe { (*pTable).pSchema }) };
        0 as i32; // The database the table is being created in
        unsafe {
            sqlite3AuthCheck(
                pParse,
                29 as i32,
                (unsafe { (*pTable).zName }) as *const i8,
                (unsafe {
                    *unsafe { unsafe { (*pTable).u.vtab.azArg }.offset((0 as i32) as isize) }
                }) as *const i8,
                (unsafe {
                    (*unsafe { unsafe { (*unsafe { (*pParse).db }).aDb }.offset(iDb as isize) })
                        .zDbSName
                }) as *const i8,
            )
        };
    }
}

/// This routine takes the module argument that has been accumulating
/// in pParse->zArg[] and appends it to the list of arguments on the
/// virtual table currently under construction in pParse->pTable.
fn addArgumentToVtab(mut pParse: *mut Parse) {
    if (unsafe { (*pParse).sArg.z }) != std::ptr::null::<i8>()
        && (unsafe { (*pParse).pNewTable }) != std::ptr::null_mut::<Table>()
    {
        let mut z: *const i8 = unsafe { (*pParse).sArg.z };
        let mut n: i32 = (unsafe { (*pParse).sArg.n }) as i32;
        let mut db: *mut sqlite3 = unsafe { (*pParse).db };
        addModuleArgument(pParse, unsafe { (*pParse).pNewTable }, unsafe {
            sqlite3DbStrNDup(db, z, (n as i64) as u64)
        });
    }
}

/// The parser calls this routine after the CREATE VIRTUAL TABLE statement
/// has been completely parsed.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3VtabFinishParse(mut pParse: *mut Parse, mut pEnd: *mut Token) {
    let mut pTab: *mut Table = unsafe { (*pParse).pNewTable }; // The table being constructed
    let mut db: *mut sqlite3 = unsafe { (*pParse).db }; // The database connection
    if pTab == std::ptr::null_mut::<Table>() {
        return;
    }
    0 as i32;
    addArgumentToVtab(pParse);
    unsafe {
        (*pParse).sArg.z = std::ptr::null::<i8>();
    }
    if (unsafe { (*pTab).u.vtab.nArg }) < (1 as i32) {
        return;
    }
    // If the CREATE VIRTUAL TABLE statement is being entered for the
    // first time (in other words if the virtual table is actually being
    // created now instead of just being read out of sqlite_schema) then
    // do additional initialization work and store the statement text
    // in the sqlite_schema table.
    if !((unsafe { (*db).init.busy }) != (0 as u8)) {
        let mut zStmt: *mut i8 = unsafe { std::mem::zeroed() };
        let mut zWhere: *mut i8 = unsafe { std::mem::zeroed() };
        let mut iDb: i32 = 0 as i32;
        let mut iReg: i32 = 0 as i32;
        let mut v: *mut Vdbe = unsafe { std::mem::zeroed() };
        unsafe { sqlite3MayAbort(pParse) };
        // Compute the complete text of the CREATE VIRTUAL TABLE statement
        if pEnd != std::ptr::null_mut::<Token>() {
            unsafe {
                (*pParse).sNameToken.n = ((((unsafe {
                    unsafe { (*pEnd).z }
                        .offset_from((unsafe { (*pParse).sNameToken.z }) as *const i8)
                }) as i64) as i32) as u32)
                    .wrapping_add(unsafe { (*pEnd).n });
            }
        }
        zStmt = unsafe {
            sqlite3MPrintf(
                db,
                (b"CREATE VIRTUAL TABLE %T\0".as_ptr() as *mut i8) as *const i8,
                unsafe { std::ptr::addr_of_mut!((*pParse).sNameToken) },
            )
        };
        // A slot for the record has already been allocated in the
        // schema table.  We just need to update that slot with all
        // the information we've collected.
        //
        // The VM register number pParse->u1.cr.regRowid holds the rowid of an
        // entry in the sqlite_schema table that was created for this vtab
        // by sqlite3StartTable().
        iDb = unsafe { sqlite3SchemaToIndex(db, unsafe { (*pTab).pSchema }) };
        0 as i32;
        unsafe {
            sqlite3NestedParse(pParse, (b"UPDATE %Q.sqlite_master SET type='table', name=%Q, tbl_name=%Q, rootpage=0, sql=%Q WHERE rowid=#%d\0".as_ptr() as *mut i8) as *const i8, unsafe { (*unsafe { unsafe { (*db).aDb }.offset(iDb as isize) }).zDbSName }, unsafe { (*pTab).zName }, unsafe { (*pTab).zName }, zStmt, unsafe { (*pParse).u1.cr.regRowid })
        };
        v = unsafe { sqlite3GetVdbe(pParse) };
        unsafe { sqlite3ChangeCookie(pParse, iDb) };
        unsafe { sqlite3VdbeAddOp0(v, 168 as i32) };
        zWhere = unsafe {
            sqlite3MPrintf(
                db,
                (b"name=%Q AND sql=%Q\0".as_ptr() as *mut i8) as *const i8,
                unsafe { (*pTab).zName },
                zStmt,
            )
        };
        unsafe { sqlite3VdbeAddParseSchemaOp(v, iDb, zWhere, ((0 as i32) as i16) as u16) };
        unsafe { sqlite3DbFree(db, zStmt as *mut ()) };
        let __v870: *mut Parse = pParse;
        let __v871: i32 = unsafe { (*__v870).nMem };
        let __v872: i32 = __v871 + (1 as i32);
        unsafe {
            (*__v870).nMem = __v872;
        }
        iReg = __v872;
        unsafe { sqlite3VdbeLoadString(v, iReg, (unsafe { (*pTab).zName }) as *const i8) };
        unsafe { sqlite3VdbeAddOp2(v, 173 as i32, iDb, iReg) };
    } else {
        // If we are rereading the sqlite_schema table create the in-memory
        // record of the table.
        let mut pOld: *mut Table = unsafe { std::mem::zeroed() };
        let mut pSchema: *mut Schema = unsafe { (*pTab).pSchema };
        let mut zName: *const i8 = (unsafe { (*pTab).zName }) as *const i8;
        0 as i32;
        unsafe { sqlite3MarkAllShadowTablesOf(db, pTab) };
        pOld = (unsafe {
            sqlite3HashInsert(
                unsafe { std::ptr::addr_of_mut!((*pSchema).tblHash) },
                zName,
                pTab as *mut (),
            )
        }) as *mut Table;
        if pOld != std::ptr::null_mut::<Table>() {
            unsafe { sqlite3OomFault(db) };
            0 as i32; // Malloc must have failed inside HashInsert()
            return;
        }
        unsafe {
            (*pParse).pNewTable = std::ptr::null_mut::<Table>();
        }
    }
}

/// The parser calls this routine when it sees the first token
/// of an argument to the module name in a CREATE VIRTUAL TABLE statement.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3VtabArgInit(mut pParse: *mut Parse) {
    addArgumentToVtab(pParse);
    unsafe {
        (*pParse).sArg.z = std::ptr::null::<i8>();
    }
    unsafe {
        (*pParse).sArg.n = (0 as i32) as u32;
    }
}

/// The parser calls this routine for each token after the first token
/// in an argument to the module name in a CREATE VIRTUAL TABLE statement.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3VtabArgExtend(mut pParse: *mut Parse, mut p: *mut Token) {
    let mut pArg: *mut Token = unsafe { std::ptr::addr_of_mut!((*pParse).sArg) };
    if (unsafe { (*pArg).z }) == std::ptr::null::<i8>() {
        unsafe {
            (*pArg).z = unsafe { (*p).z };
        }
        unsafe {
            (*pArg).n = unsafe { (*p).n };
        }
    } else {
        0 as i32;
        unsafe {
            (*pArg).n = (((unsafe {
                unsafe { unsafe { (*p).z }.offset((unsafe { (*p).n }) as isize) }
                    .offset_from((unsafe { (*pArg).z }) as *const i8)
            }) as i64) as i32) as u32;
        }
    }
}

/// Invoke a virtual table constructor (either xCreate or xConnect). The
/// pointer to the function to invoke is passed as the fourth parameter
/// to this procedure.
fn vtabCallConstructor(
    mut db: *mut sqlite3,
    mut pTab: *mut Table,
    mut pMod: *mut Module,
    mut xConstruct: Option<
        unsafe extern "C-unwind" fn(
            *mut sqlite3,
            *mut (),
            i32,
            *const *const i8,
            *mut *mut sqlite3_vtab,
            *mut *mut i8,
        ) -> i32,
    >,
    mut pzErr: *mut *mut i8,
) -> i32 {
    let mut sCtx: VtabCtx = unsafe { std::mem::zeroed() };
    let mut pVTable: *mut VTable = unsafe { std::mem::zeroed() };
    let mut rc: i32 = 0 as i32;
    let mut azArg: *const *const i8 = unsafe { std::mem::zeroed() };
    let mut nArg: i32 = unsafe { (*pTab).u.vtab.nArg };
    let mut zErr: *mut i8 = std::ptr::null_mut::<i8>();
    let mut zModuleName: *mut i8 = unsafe { std::mem::zeroed() };
    let mut iDb: i32 = 0 as i32;
    let mut pCtx: *mut VtabCtx = unsafe { std::mem::zeroed() };
    0 as i32;
    azArg = (unsafe { (*pTab).u.vtab.azArg }) as *const *const i8;
    // Check that the virtual-table is not already being initialized
    pCtx = unsafe { (*db).pVtabCtx };
    '__slate_break_793: while pCtx != std::ptr::null_mut::<VtabCtx>() {
        if (unsafe { (*pCtx).pTab }) == pTab {
            unsafe {
                *pzErr = unsafe {
                    sqlite3MPrintf(
                        db,
                        (b"vtable constructor called recursively: %s\0".as_ptr() as *mut i8)
                            as *const i8,
                        unsafe { (*pTab).zName },
                    )
                };
            }
            return 6 as i32;
        }
        pCtx = unsafe { (*pCtx).pPrior };
    }
    zModuleName = unsafe { sqlite3DbStrDup(db, (unsafe { (*pTab).zName }) as *const i8) };
    if !(zModuleName != std::ptr::null_mut::<i8>()) {
        return 7 as i32;
    }
    pVTable = (unsafe { sqlite3MallocZero(48 as u64) }) as *mut VTable;
    if !(pVTable != std::ptr::null_mut::<VTable>()) {
        unsafe { sqlite3OomFault(db) };
        unsafe { sqlite3DbFree(db, zModuleName as *mut ()) };
        return 7 as i32;
    }
    unsafe {
        (*pVTable).db = db;
    }
    unsafe {
        (*pVTable).pMod = pMod;
    }
    unsafe {
        (*pVTable).eVtabRisk = ((1 as i32) as i8) as u8;
    }
    iDb = unsafe { sqlite3SchemaToIndex(db, unsafe { (*pTab).pSchema }) };
    unsafe {
        *unsafe { unsafe { (*pTab).u.vtab.azArg }.offset((1 as i32) as isize) } =
            unsafe { (*unsafe { unsafe { (*db).aDb }.offset(iDb as isize) }).zDbSName };
    }
    // Invoke the virtual table constructor
    0 as i32;
    0 as i32;
    sCtx.pTab = pTab;
    sCtx.pVTable = pVTable;
    sCtx.pPrior = unsafe { (*db).pVtabCtx };
    sCtx.bDeclared = 0 as i32;
    unsafe {
        (*db).pVtabCtx = std::ptr::addr_of_mut!(sCtx);
    }
    let __v885: *mut Table = pTab;
    let __v886: u32 = unsafe { (*__v885).nTabRef };
    let __v887: u32 = __v886.wrapping_add((1 as i32) as u32);
    unsafe {
        (*__v885).nTabRef = __v887;
    }
    rc = unsafe {
        xConstruct.unwrap()(
            db,
            unsafe { (*pMod).pAux },
            nArg,
            azArg,
            unsafe { std::ptr::addr_of_mut!((*pVTable).pVtab) },
            std::ptr::addr_of_mut!(zErr),
        )
    };
    0 as i32;
    0 as i32;
    unsafe { sqlite3DeleteTable(db, pTab) };
    unsafe {
        (*db).pVtabCtx = sCtx.pPrior;
    }
    if rc == (7 as i32) {
        unsafe { sqlite3OomFault(db) };
    }
    0 as i32;
    if (0 as i32) != rc {
        if zErr == std::ptr::null_mut::<i8>() {
            unsafe {
                *pzErr = unsafe {
                    sqlite3MPrintf(
                        db,
                        (b"vtable constructor failed: %s\0".as_ptr() as *mut i8) as *const i8,
                        zModuleName,
                    )
                };
            }
        } else {
            unsafe {
                *pzErr =
                    unsafe { sqlite3MPrintf(db, (b"%s\0".as_ptr() as *mut i8) as *const i8, zErr) };
            }
            unsafe { sqlite3_free(zErr as *mut ()) };
        }
        unsafe { sqlite3DbFree(db, pVTable as *mut ()) };
    } else {
        if (unsafe { (*pVTable).pVtab }) != std::ptr::null_mut::<sqlite3_vtab>() {
            // Justification of ALWAYS():  A correct vtab constructor must allocate
            // the sqlite3_vtab object if successful.
            unsafe {
                memset(
                    (unsafe { (*pVTable).pVtab }) as *mut (),
                    0 as i32,
                    24 as u64,
                )
            };
            unsafe {
                (*unsafe { (*pVTable).pVtab }).pModule = unsafe { (*pMod).pModule };
            }
            let __v888: *mut Module = pMod;
            let __v889: i32 = unsafe { (*__v888).nRefModule };
            let __v890: i32 = __v889 + (1 as i32);
            unsafe {
                (*__v888).nRefModule = __v890;
            }
            unsafe {
                (*pVTable).nRef = 1 as i32;
            }
            if sCtx.bDeclared == (0 as i32) {
                let mut zFormat: *const i8 = (b"vtable constructor did not declare schema: %s\0"
                    .as_ptr() as *mut i8) as *const i8;
                unsafe {
                    *pzErr = unsafe { sqlite3MPrintf(db, zFormat, zModuleName) };
                }
                sqlite3VtabUnlock(pVTable);
                rc = 1 as i32;
            } else {
                let mut iCol: i32 = 0 as i32;
                let mut oooHidden: u16 = ((0 as i32) as i16) as u16;
                // If everything went according to plan, link the new VTable structure
                // into the linked list headed by pTab->u.vtab.p. Then loop through the
                // columns of the table to see if any of them contain the token "hidden".
                // If so, set the Column COLFLAG_HIDDEN flag and remove the token from
                // the type string.
                unsafe {
                    (*pVTable).pNext = unsafe { (*pTab).u.vtab.p };
                }
                unsafe {
                    (*pTab).u.vtab.p = pVTable;
                }
                iCol = 0 as i32;
                '__slate_break_798: loop {
                    if !(iCol < ((unsafe { (*pTab).nCol }) as i32)) {
                        break;
                    }
                    let mut zType: *mut i8 = unsafe {
                        sqlite3ColumnType(
                            unsafe { unsafe { (*pTab).aCol }.offset(iCol as isize) },
                            b"\0".as_ptr() as *mut i8,
                        )
                    };
                    let mut nType: i32 = 0 as i32;
                    let mut i: i32 = 0 as i32;
                    nType = unsafe { sqlite3Strlen30(zType as *const i8) };
                    i = 0 as i32;
                    '__slate_break_800: loop {
                        if !(i < nType) {
                            break;
                        }
                        if (0 as i32)
                            == unsafe {
                                sqlite3_strnicmp(
                                    (b"hidden\0".as_ptr() as *mut i8) as *const i8,
                                    (unsafe { zType.offset(i as isize) }) as *const i8,
                                    6 as i32,
                                )
                            }
                            && (i == (0 as i32)
                                || ((unsafe { *unsafe { zType.offset((i - (1 as i32)) as isize) } })
                                    as i32)
                                    == (32 as i32))
                            && (((unsafe { *unsafe { zType.offset((i + (6 as i32)) as isize) } })
                                as i32)
                                == (0 as i32)
                                || ((unsafe { *unsafe { zType.offset((i + (6 as i32)) as isize) } })
                                    as i32)
                                    == (32 as i32))
                        {
                            break '__slate_break_800;
                        }
                        let __v893: i32 = i;
                        let __v894: i32 = __v893 + (1 as i32);
                        i = __v894;
                    }
                    if i < nType {
                        let mut j: i32 = 0 as i32;
                        let mut nDel: i32 = (6 as i32)
                            + if (unsafe { *unsafe { zType.offset((i + (6 as i32)) as isize) } })
                                != (0 as i8)
                            {
                                1 as i32
                            } else {
                                0 as i32
                            };
                        j = i;
                        '__slate_break_802: loop {
                            if !(j + nDel <= nType) {
                                break;
                            }
                            unsafe {
                                *unsafe { zType.offset(j as isize) } =
                                    unsafe { *unsafe { zType.offset((j + nDel) as isize) } };
                            }
                            let __v895: i32 = j;
                            let __v896: i32 = __v895 + (1 as i32);
                            j = __v896;
                        }
                        if ((unsafe { *unsafe { zType.offset(i as isize) } }) as i32) == (0 as i32)
                            && i > (0 as i32)
                        {
                            0 as i32;
                            unsafe {
                                *unsafe { zType.offset((i - (1 as i32)) as isize) } =
                                    (0 as i32) as i8;
                            }
                        }
                        let __v897: *mut Column =
                            unsafe { unsafe { (*pTab).aCol }.offset(iCol as isize) };
                        let __v898: u16 = unsafe { (*__v897).colFlags };
                        let __v899: u16 = ((((__v898 as u32) as i32) | (2 as i32)) as i16) as u16;
                        unsafe {
                            (*__v897).colFlags = __v899;
                        }
                        let __v900: *mut Table = pTab;
                        let __v901: u32 = unsafe { (*__v900).tabFlags };
                        let __v902: u32 = __v901 | ((2 as i32) as u32);
                        unsafe {
                            (*__v900).tabFlags = __v902;
                        }
                        oooHidden = ((1024 as i32) as i16) as u16;
                    } else {
                        let __v903: *mut Table = pTab;
                        let __v904: u32 = unsafe { (*__v903).tabFlags };
                        let __v905: u32 = __v904 | (((oooHidden as u32) as i32) as u32);
                        unsafe {
                            (*__v903).tabFlags = __v905;
                        }
                    }
                    let __v891: i32 = iCol;
                    let __v892: i32 = __v891 + (1 as i32);
                    iCol = __v892;
                }
            }
        }
    }
    unsafe { sqlite3DbFree(db, zModuleName as *mut ()) };
    return rc;
}

/// This function is invoked by the parser to call the xConnect() method
/// of the virtual table pTab. If an error occurs, an error code is returned
/// and an error left in pParse.
///
/// This call is a no-op if table pTab is not a virtual table.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3VtabCallConnect(mut pParse: *mut Parse, mut pTab: *mut Table) -> i32 {
    let mut db: *mut sqlite3 = unsafe { (*pParse).db };
    let mut zMod: *const i8 = unsafe { std::mem::zeroed() };
    let mut pMod: *mut Module = unsafe { std::mem::zeroed() };
    let mut rc: i32 = 0 as i32;
    0 as i32;
    0 as i32;
    if sqlite3GetVTable(db, pTab) != std::ptr::null_mut::<VTable>() {
        return 0 as i32;
    }
    // Locate the required virtual table module
    zMod = (unsafe { *unsafe { unsafe { (*pTab).u.vtab.azArg }.offset((0 as i32) as isize) } })
        as *const i8;
    pMod = (unsafe {
        sqlite3HashFind(
            (unsafe { std::ptr::addr_of_mut!((*db).aModule) }) as *const Hash,
            zMod,
        )
    }) as *mut Module;
    if !(pMod != std::ptr::null_mut::<Module>()) {
        let mut zModule: *const i8 =
            (unsafe { *unsafe { unsafe { (*pTab).u.vtab.azArg }.offset((0 as i32) as isize) } })
                as *const i8;
        unsafe {
            sqlite3ErrorMsg(
                pParse,
                (b"no such module: %s\0".as_ptr() as *mut i8) as *const i8,
                zModule,
            )
        };
        rc = 1 as i32;
    } else {
        let mut zErr: *mut i8 = std::ptr::null_mut::<i8>();
        rc = vtabCallConstructor(
            db,
            pTab,
            pMod,
            unsafe { (*unsafe { (*pMod).pModule }).xConnect },
            std::ptr::addr_of_mut!(zErr),
        );
        if rc != (0 as i32) {
            unsafe { sqlite3ErrorMsg(pParse, (b"%s\0".as_ptr() as *mut i8) as *const i8, zErr) };
            unsafe {
                (*pParse).rc = rc;
            }
        } else {
            unsafe { sqlite3MarkAllShadowTablesOf(db, pTab) };
        }
        unsafe { sqlite3DbFree(db, zErr as *mut ()) };
    }
    return rc;
}

/// Grow the db->aVTrans[] array so that there is room for at least one
/// more v-table. Return SQLITE_NOMEM if a malloc fails, or SQLITE_OK otherwise.
fn growVTrans(mut db: *mut sqlite3) -> i32 {
    let mut ARRAY_INCR: i32 = 5 as i32;
    // Grow the sqlite3.aVTrans array if required
    if (unsafe { (*db).nVTrans }) % ARRAY_INCR == (0 as i32) {
        let mut aVTrans: *mut *mut VTable = unsafe { std::mem::zeroed() };
        let mut nBytes: i64 = (8 as u64)
            .wrapping_mul((((unsafe { (*db).nVTrans }) as i64) + (ARRAY_INCR as i64)) as u64)
            as i64;
        aVTrans =
            (unsafe { sqlite3DbRealloc(db, (unsafe { (*db).aVTrans }) as *mut (), nBytes as u64) })
                as *mut *mut VTable;
        if !(aVTrans != std::ptr::null_mut::<*mut VTable>()) {
            return 7 as i32;
        }
        unsafe {
            memset(
                (unsafe { aVTrans.offset((unsafe { (*db).nVTrans }) as isize) }) as *mut (),
                0 as i32,
                (8 as u64).wrapping_mul((ARRAY_INCR as i64) as u64),
            )
        };
        unsafe {
            (*db).aVTrans = aVTrans;
        }
    }
    return 0 as i32;
}

/// Add the virtual table pVTab to the array sqlite3.aVTrans[]. Space should
/// have already been reserved using growVTrans().
fn addToVTrans(mut db: *mut sqlite3, mut pVTab: *mut VTable) {
    // Add pVtab to the end of sqlite3.aVTrans
    let __v906: *mut sqlite3 = db;
    let __v907: i32 = unsafe { (*__v906).nVTrans };
    let __v908: i32 = __v907 + (1 as i32);
    unsafe {
        (*__v906).nVTrans = __v908;
    }
    unsafe {
        *unsafe { unsafe { (*db).aVTrans }.offset(__v907 as isize) } = pVTab;
    }
    sqlite3VtabLock(pVTab);
}

/// This function is invoked by the vdbe to call the xCreate method
/// of the virtual table named zTab in database iDb.
///
/// If an error occurs, *pzErr is set to point to an English language
/// description of the error and an SQLITE_XXX error code is returned.
/// In this case the caller must call sqlite3DbFree(db, ) on *pzErr.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3VtabCallCreate(
    mut db: *mut sqlite3,
    mut iDb: i32,
    mut zTab: *const i8,
    mut pzErr: *mut *mut i8,
) -> i32 {
    let mut rc: i32 = 0 as i32;
    let mut pTab: *mut Table = unsafe { std::mem::zeroed() };
    let mut pMod: *mut Module = unsafe { std::mem::zeroed() };
    let mut zMod: *const i8 = unsafe { std::mem::zeroed() };
    pTab = unsafe {
        sqlite3FindTable(
            db,
            zTab,
            (unsafe { (*unsafe { unsafe { (*db).aDb }.offset(iDb as isize) }).zDbSName })
                as *const i8,
        )
    };
    0 as i32;
    // Locate the required virtual table module
    zMod = (unsafe { *unsafe { unsafe { (*pTab).u.vtab.azArg }.offset((0 as i32) as isize) } })
        as *const i8;
    pMod = (unsafe {
        sqlite3HashFind(
            (unsafe { std::ptr::addr_of_mut!((*db).aModule) }) as *const Hash,
            zMod,
        )
    }) as *mut Module;
    // If the module has been registered and includes a Create method,
    // invoke it now. If the module has not been registered, return an
    // error. Otherwise, do nothing.
    if pMod == std::ptr::null_mut::<Module>()
        || (unsafe { (*unsafe { (*pMod).pModule }).xCreate }) == None
        || (unsafe { (*unsafe { (*pMod).pModule }).xDestroy }) == None
    {
        unsafe {
            *pzErr = unsafe {
                sqlite3MPrintf(
                    db,
                    (b"no such module: %s\0".as_ptr() as *mut i8) as *const i8,
                    zMod,
                )
            };
        }
        rc = 1 as i32;
    } else {
        rc = vtabCallConstructor(
            db,
            pTab,
            pMod,
            unsafe { (*unsafe { (*pMod).pModule }).xCreate },
            pzErr,
        );
    }
    // Justification of ALWAYS():  The xConstructor method is required to
    // create a valid sqlite3_vtab if it returns SQLITE_OK.
    let __v873: bool;
    if rc == (0 as i32) {
        __v873 = sqlite3GetVTable(db, pTab) != std::ptr::null_mut::<VTable>();
    } else {
        __v873 = false as bool;
    }
    if __v873 {
        rc = growVTrans(db);
        if rc == (0 as i32) {
            addToVTrans(db, sqlite3GetVTable(db, pTab));
        }
    }
    return rc;
}

/// This function is used to set the schema of a virtual table.  It is only
/// valid to call this function from within the xCreate() or xConnect() of a
/// virtual table module.
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.vtab.sqlite3_declare_vtab")]
extern "C-unwind" fn sqlite3_declare_vtab(
    mut db: *mut sqlite3,
    mut zCreateTable: *const i8,
) -> i32 {
    let mut pCtx: *mut VtabCtx = unsafe { std::mem::zeroed() };
    let mut rc: i32 = 0 as i32;
    let mut pTab: *mut Table = unsafe { std::mem::zeroed() };
    let mut sParse: Parse = unsafe { std::mem::zeroed() };
    let mut initBusy: i32 = 0 as i32;
    let mut i: i32 = 0 as i32;
    let mut z: *const u8 = unsafe { std::mem::zeroed() };
    // Verify that the first two keywords in the CREATE TABLE statement
    // really are "CREATE" and "TABLE".  If this is not the case, then
    // sqlite3_declare_vtab() is being misused.
    z = zCreateTable as *const u8;
    i = 0 as i32;
    '__slate_break_806: loop {
        if !((unsafe {
            *unsafe { unsafe { std::ptr::addr_of!(aKeyword) as *const u8 }.offset(i as isize) }
        }) != (0 as u8))
        {
            break;
        }
        let mut tokenType: i32 = 0 as i32;
        '__slate_break_807: loop {
            let __v823: *const u8 = z;
            let __v824: *const u8 = unsafe {
                __v823.offset(
                    (unsafe { sqlite3GetToken(z, std::ptr::addr_of_mut!(tokenType)) }) as isize,
                )
            };
            z = __v824;
            if !(tokenType == (184 as i32) || tokenType == (185 as i32)) {
                break;
            }
        }
        if tokenType
            != (((unsafe {
                *unsafe { unsafe { std::ptr::addr_of!(aKeyword) as *const u8 }.offset(i as isize) }
            }) as u32) as i32)
        {
            unsafe {
                sqlite3ErrorWithMsg(
                    db,
                    1 as i32,
                    (b"syntax error\0".as_ptr() as *mut i8) as *const i8,
                )
            };
            return 1 as i32;
        }
        let __v821: i32 = i;
        let __v822: i32 = __v821 + (1 as i32);
        i = __v822;
    }
    unsafe { sqlite3_mutex_enter(unsafe { (*db).mutex }) };
    pCtx = unsafe { (*db).pVtabCtx };
    if !(pCtx != std::ptr::null_mut::<VtabCtx>()) || (unsafe { (*pCtx).bDeclared }) != (0 as i32) {
        unsafe { sqlite3Error(db, unsafe { sqlite3MisuseError(850 as i32) }) };
        unsafe { sqlite3_mutex_leave(unsafe { (*db).mutex }) };
        return unsafe { sqlite3MisuseError(852 as i32) };
    }
    pTab = unsafe { (*pCtx).pTab };
    0 as i32;
    unsafe { sqlite3ParseObjectInit(std::ptr::addr_of_mut!(sParse), db) };
    sParse.eParseMode = ((1 as i32) as i8) as u8;
    sParse
        .__slate_bits_0
        .__set_disableTriggers((1 as i32) as u32);
    // We should never be able to reach this point while loading the
    // schema.  Nevertheless, defend against that (turn off db->init.busy)
    // in case a bug arises.
    0 as i32;
    initBusy = ((unsafe { (*db).init.busy }) as u32) as i32;
    unsafe {
        (*db).init.busy = ((0 as i32) as i8) as u8;
    }
    sParse.nQueryLoop = (1 as i32) as i16;
    if (0 as i32) == unsafe { sqlite3RunParser(std::ptr::addr_of_mut!(sParse), zCreateTable) } {
        0 as i32;
        0 as i32;
        0 as i32;
        0 as i32;
        if !((unsafe { (*pTab).aCol }) != std::ptr::null_mut::<Column>()) {
            let mut pNew: *mut Table = sParse.pNewTable;
            let mut pIdx: *mut Index = unsafe { std::mem::zeroed() };
            unsafe {
                (*pTab).aCol = unsafe { (*pNew).aCol };
            }
            0 as i32;
            unsafe { sqlite3ExprListDelete(db, unsafe { (*pNew).u.tab.pDfltList }) };
            let __v825: i16 = unsafe { (*pNew).nCol };
            unsafe {
                (*pTab).nCol = __v825;
            }
            unsafe {
                (*pTab).nNVCol = __v825;
            }
            let __v826: *mut Table = pTab;
            let __v827: u32 = unsafe { (*__v826).tabFlags };
            let __v828: u32 =
                __v827 | (unsafe { (*pNew).tabFlags }) & (((128 as i32) | (512 as i32)) as u32);
            unsafe {
                (*__v826).tabFlags = __v828;
            }
            unsafe {
                (*pNew).nCol = (0 as i32) as i16;
            }
            unsafe {
                (*pNew).aCol = std::ptr::null_mut::<Column>();
            }
            0 as i32;
            0 as i32;
            let __v829: bool;
            if !((unsafe { (*pNew).tabFlags }) & ((128 as i32) as u32) == ((0 as i32) as u32))
                && (unsafe {
                    (*unsafe { (*unsafe { (*unsafe { (*pCtx).pVTable }).pMod }).pModule }).xUpdate
                }) != None
            {
                __v829 = (((unsafe { (*unsafe { sqlite3PrimaryKeyIndex(pNew) }).nKeyCol }) as u32)
                    as i32)
                    != (1 as i32);
            } else {
                __v829 = false as bool;
            }
            if __v829 {
                // WITHOUT ROWID virtual tables must either be read-only (xUpdate==0)
                // or else must have a single-column PRIMARY KEY
                rc = 1 as i32;
            }
            pIdx = unsafe { (*pNew).pIndex };
            if pIdx != std::ptr::null_mut::<Index>() {
                0 as i32;
                unsafe {
                    (*pTab).pIndex = pIdx;
                }
                unsafe {
                    (*pNew).pIndex = std::ptr::null_mut::<Index>();
                }
                unsafe {
                    (*pIdx).pTable = pTab;
                }
            }
        }
        unsafe {
            (*pCtx).bDeclared = 1 as i32;
        }
    } else {
        unsafe {
            sqlite3ErrorWithMsg(
                db,
                1 as i32,
                (if sParse.zErrMsg != std::ptr::null_mut::<i8>() {
                    b"%s\0".as_ptr() as *mut i8
                } else {
                    std::ptr::null_mut::<i8>()
                }) as *const i8,
                sParse.zErrMsg,
            )
        };
        unsafe { sqlite3DbFree(db, sParse.zErrMsg as *mut ()) };
        rc = 1 as i32;
    }
    sParse.eParseMode = ((0 as i32) as i8) as u8;
    if sParse.pVdbe != std::ptr::null_mut::<Vdbe>() {
        unsafe { sqlite3VdbeFinalize(sParse.pVdbe) };
    }
    unsafe { sqlite3DeleteTable(db, sParse.pNewTable) };
    unsafe { sqlite3ParseObjectReset(std::ptr::addr_of_mut!(sParse)) };
    unsafe {
        (*db).init.busy = (initBusy as i8) as u8;
    }
    0 as i32;
    rc = unsafe { sqlite3ApiExit(db, rc) };
    unsafe { sqlite3_mutex_leave(unsafe { (*db).mutex }) };
    return rc;
}

static mut aKeyword: [u8; 3] = [
    ((17 as i32) as i8) as u8,
    ((16 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
];

/// This function is invoked by the vdbe to call the xDestroy method
/// of the virtual table named zTab in database iDb. This occurs
/// when a DROP TABLE is mentioned.
///
/// This call is a no-op if zTab is not a virtual table.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3VtabCallDestroy(
    mut db: *mut sqlite3,
    mut iDb: i32,
    mut zTab: *const i8,
) -> i32 {
    let mut rc: i32 = 0 as i32;
    let mut pTab: *mut Table = unsafe { std::mem::zeroed() };
    pTab = unsafe {
        sqlite3FindTable(
            db,
            zTab,
            (unsafe { (*unsafe { unsafe { (*db).aDb }.offset(iDb as isize) }).zDbSName })
                as *const i8,
        )
    };
    if pTab != std::ptr::null_mut::<Table>()
        && (((unsafe { (*pTab).eTabType }) as u32) as i32) == (1 as i32)
        && (unsafe { (*pTab).u.vtab.p }) != std::ptr::null_mut::<VTable>()
    {
        let mut p: *mut VTable = unsafe { std::mem::zeroed() };
        let mut xDestroy: Option<unsafe extern "C-unwind" fn(*mut sqlite3_vtab) -> i32> =
            unsafe { std::mem::zeroed() };
        p = unsafe { (*pTab).u.vtab.p };
        '__slate_break_810: while p != std::ptr::null_mut::<VTable>() {
            0 as i32;
            if (unsafe { (*unsafe { (*p).pVtab }).nRef }) > (0 as i32) {
                return 6 as i32;
            }
            p = unsafe { (*p).pNext };
        }
        p = vtabDisconnectAll(db, pTab);
        xDestroy = unsafe { (*unsafe { (*unsafe { (*p).pMod }).pModule }).xDestroy };
        if xDestroy == None {
            xDestroy = unsafe { (*unsafe { (*unsafe { (*p).pMod }).pModule }).xDisconnect };
        }
        0 as i32;
        let __v874: *mut Table = pTab;
        let __v875: u32 = unsafe { (*__v874).nTabRef };
        let __v876: u32 = __v875.wrapping_add((1 as i32) as u32);
        unsafe {
            (*__v874).nTabRef = __v876;
        }
        rc = unsafe { xDestroy.unwrap()(unsafe { (*p).pVtab }) };
        // Remove the sqlite3_vtab* from the aVTrans[] array, if applicable
        if rc == (0 as i32) {
            0 as i32;
            unsafe {
                (*p).pVtab = std::ptr::null_mut::<sqlite3_vtab>();
            }
            unsafe {
                (*pTab).u.vtab.p = std::ptr::null_mut::<VTable>();
            }
            sqlite3VtabUnlock(p);
        }
        unsafe { sqlite3DeleteTable(db, pTab) };
    }
    return rc;
}

/// This function invokes either the xRollback or xCommit method
/// of each of the virtual tables in the sqlite3.aVTrans array. The method
/// called is identified by the second argument, "offset", which is
/// the offset of the method to call in the sqlite3_module structure.
///
/// The array is cleared after invoking the callbacks.
fn callFinaliser(mut db: *mut sqlite3, mut offset: i32) {
    let mut i: i32 = 0 as i32;
    if (unsafe { (*db).aVTrans }) != std::ptr::null_mut::<*mut VTable>() {
        let mut aVTrans: *mut *mut VTable = unsafe { (*db).aVTrans };
        unsafe {
            (*db).aVTrans = std::ptr::null_mut::<*mut VTable>();
        }
        i = 0 as i32;
        '__slate_break_811: loop {
            if !(i < unsafe { (*db).nVTrans }) {
                break;
            }
            let mut pVTab: *mut VTable = unsafe { *unsafe { aVTrans.offset(i as isize) } };
            let mut p: *mut sqlite3_vtab = unsafe { (*pVTab).pVtab };
            if p != std::ptr::null_mut::<sqlite3_vtab>() {
                let mut x: Option<unsafe extern "C-unwind" fn(*mut sqlite3_vtab) -> i32> =
                    unsafe { std::mem::zeroed() };
                x = unsafe {
                    *((unsafe { ((unsafe { (*p).pModule }) as *mut i8).offset(offset as isize) })
                        as *mut Option<unsafe extern "C-unwind" fn(*mut sqlite3_vtab) -> i32>)
                };
                if x != None {
                    unsafe { x.unwrap()(p) };
                }
            }
            unsafe {
                (*pVTab).iSavepoint = 0 as i32;
            }
            sqlite3VtabUnlock(pVTab);
            let __v909: i32 = i;
            let __v910: i32 = __v909 + (1 as i32);
            i = __v910;
        }
        unsafe { sqlite3DbFree(db, aVTrans as *mut ()) };
        unsafe {
            (*db).nVTrans = 0 as i32;
        }
    }
}

/// Invoke the xSync method of all virtual tables in the sqlite3.aVTrans
/// array. Return the error code for the first error that occurs, or
/// SQLITE_OK if all xSync operations are successful.
///
/// If an error message is available, leave it in p->zErrMsg.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3VtabSync(mut db: *mut sqlite3, mut p: *mut Vdbe) -> i32 {
    let mut i: i32 = 0 as i32;
    let mut rc: i32 = 0 as i32;
    let mut aVTrans: *mut *mut VTable = unsafe { (*db).aVTrans };
    unsafe {
        (*db).aVTrans = std::ptr::null_mut::<*mut VTable>();
    }
    i = 0 as i32;
    '__slate_break_812: loop {
        if !(rc == (0 as i32) && i < unsafe { (*db).nVTrans }) {
            break;
        }
        let mut x: Option<unsafe extern "C-unwind" fn(*mut sqlite3_vtab) -> i32> =
            unsafe { std::mem::zeroed() };
        let mut pVtab: *mut sqlite3_vtab =
            unsafe { (*unsafe { *unsafe { aVTrans.offset(i as isize) } }).pVtab };
        let __v834: bool;
        if pVtab != std::ptr::null_mut::<sqlite3_vtab>() {
            let __v835: Option<unsafe extern "C-unwind" fn(*mut sqlite3_vtab) -> i32> =
                unsafe { (*unsafe { (*pVtab).pModule }).xSync };
            x = __v835;
            __v834 = __v835 != None;
        } else {
            __v834 = false as bool;
        }
        if __v834 {
            rc = unsafe { x.unwrap()(pVtab) };
            unsafe { sqlite3VtabImportErrmsg(p, pVtab) };
        }
        let __v832: i32 = i;
        let __v833: i32 = __v832 + (1 as i32);
        i = __v833;
    }
    unsafe {
        (*db).aVTrans = aVTrans;
    }
    return rc;
}

/// Invoke the xRollback method of all virtual tables in the
/// sqlite3.aVTrans array. Then clear the array itself.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3VtabRollback(mut db: *mut sqlite3) -> i32 {
    callFinaliser(db, ((136 as u64) as u32) as i32);
    return 0 as i32;
}

/// Invoke the xCommit method of all virtual tables in the
/// sqlite3.aVTrans array. Then clear the array itself.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3VtabCommit(mut db: *mut sqlite3) -> i32 {
    callFinaliser(db, ((128 as u64) as u32) as i32);
    return 0 as i32;
}

/// If the virtual table pVtab supports the transaction interface
/// (xBegin/xRollback/xCommit and optionally xSync) and a transaction is
/// not currently open, invoke the xBegin method now.
///
/// If the xBegin call is successful, place the sqlite3_vtab pointer
/// in the sqlite3.aVTrans array.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3VtabBegin(mut db: *mut sqlite3, mut pVTab: *mut VTable) -> i32 {
    let mut rc: i32 = 0 as i32;
    let mut pModule: *const sqlite3_module = unsafe { std::mem::zeroed() };
    // Special case: If db->aVTrans is NULL and db->nVTrans is greater
    // than zero, then this function is being called from within a
    // virtual module xSync() callback. It is illegal to write to
    // virtual module tables in this case, so return SQLITE_LOCKED.
    if (unsafe { (*db).nVTrans }) > (0 as i32)
        && (unsafe { (*db).aVTrans }) == std::ptr::null_mut::<*mut VTable>()
    {
        return 6 as i32;
    }
    if !(pVTab != std::ptr::null_mut::<VTable>()) {
        return 0 as i32;
    }
    pModule = unsafe { (*unsafe { (*pVTab).pVtab }).pModule };
    if (unsafe { (*pModule).xBegin }) != None {
        let mut i: i32 = 0 as i32;
        // If pVtab is already in the aVTrans array, return early
        i = 0 as i32;
        '__slate_break_813: loop {
            if !(i < unsafe { (*db).nVTrans }) {
                break;
            }
            if (unsafe { *unsafe { unsafe { (*db).aVTrans }.offset(i as isize) } }) == pVTab {
                return 0 as i32;
            }
            let __v877: i32 = i;
            let __v878: i32 = __v877 + (1 as i32);
            i = __v878;
        }
        // Invoke the xBegin method. If successful, add the vtab to the
        // sqlite3.aVTrans[] array.
        rc = growVTrans(db);
        if rc == (0 as i32) {
            rc = unsafe { unsafe { (*pModule).xBegin }.unwrap()(unsafe { (*pVTab).pVtab }) };
            if rc == (0 as i32) {
                let mut iSvpt: i32 = (unsafe { (*db).nStatement }) + unsafe { (*db).nSavepoint };
                addToVTrans(db, pVTab);
                if iSvpt != (0 as i32) && (unsafe { (*pModule).xSavepoint }) != None {
                    unsafe {
                        (*pVTab).iSavepoint = iSvpt;
                    }
                    rc = unsafe {
                        unsafe { (*pModule).xSavepoint }.unwrap()(
                            unsafe { (*pVTab).pVtab },
                            iSvpt - (1 as i32),
                        )
                    };
                }
            }
        }
    }
    return rc;
}

/// Invoke either the xSavepoint, xRollbackTo or xRelease method of all
/// virtual tables that currently have an open transaction. Pass iSavepoint
/// as the second argument to the virtual table method invoked.
///
/// If op is SAVEPOINT_BEGIN, the xSavepoint method is invoked. If it is
/// SAVEPOINT_ROLLBACK, the xRollbackTo method. Otherwise, if op is
/// SAVEPOINT_RELEASE, then the xRelease method of each virtual table with
/// an open transaction is invoked.
///
/// If any virtual table method returns an error code other than SQLITE_OK,
/// processing is abandoned and the error returned to the caller of this
/// function immediately. If all calls to virtual table methods are successful,
/// SQLITE_OK is returned.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3VtabSavepoint(
    mut db: *mut sqlite3,
    mut op: i32,
    mut iSavepoint: i32,
) -> i32 {
    let mut rc: i32 = 0 as i32;
    0 as i32;
    0 as i32;
    if (unsafe { (*db).aVTrans }) != std::ptr::null_mut::<*mut VTable>() {
        let mut i: i32 = 0 as i32;
        i = 0 as i32;
        '__slate_break_814: loop {
            if !(rc == (0 as i32) && i < unsafe { (*db).nVTrans }) {
                break;
            }
            let mut pVTab: *mut VTable =
                unsafe { *unsafe { unsafe { (*db).aVTrans }.offset(i as isize) } };
            let mut pMod: *const sqlite3_module = unsafe { (*unsafe { (*pVTab).pMod }).pModule };
            if (unsafe { (*pVTab).pVtab }) != std::ptr::null_mut::<sqlite3_vtab>()
                && (unsafe { (*pMod).iVersion }) >= (2 as i32)
            {
                let mut xMethod: Option<
                    unsafe extern "C-unwind" fn(*mut sqlite3_vtab, i32) -> i32,
                > = unsafe { std::mem::zeroed() };
                sqlite3VtabLock(pVTab);
                '__slate_break_815: {
                    match op {
                        0 => {
                            xMethod = unsafe { (*pMod).xSavepoint };
                            unsafe {
                                (*pVTab).iSavepoint = iSavepoint + (1 as i32);
                            }
                        }
                        2 => {
                            xMethod = unsafe { (*pMod).xRollbackTo };
                        }
                        _ => {
                            xMethod = unsafe { (*pMod).xRelease };
                        }
                    }
                }
                if xMethod != None && (unsafe { (*pVTab).iSavepoint }) > iSavepoint {
                    let mut savedFlags: u64 =
                        (unsafe { (*db).flags }) & (((268435456 as i32) as i64) as u64);
                    let __v847: *mut sqlite3 = db;
                    let __v848: u64 = unsafe { (*__v847).flags };
                    let __v849: u64 = __v848 & !(((268435456 as i32) as i64) as u64);
                    unsafe {
                        (*__v847).flags = __v849;
                    }
                    rc = unsafe { xMethod.unwrap()(unsafe { (*pVTab).pVtab }, iSavepoint) };
                    let __v850: *mut sqlite3 = db;
                    let __v851: u64 = unsafe { (*__v850).flags };
                    let __v852: u64 = __v851 | savedFlags;
                    unsafe {
                        (*__v850).flags = __v852;
                    }
                }
                sqlite3VtabUnlock(pVTab);
            }
            let __v845: i32 = i;
            let __v846: i32 = __v845 + (1 as i32);
            i = __v846;
        }
    }
    return rc;
}

/// The first parameter (pDef) is a function implementation.  The
/// second parameter (pExpr) is the first argument to this function.
/// If pExpr is a column in a virtual table, then let the virtual
/// table implementation have an opportunity to overload the function.
///
/// This routine is used to allow virtual table implementations to
/// overload MATCH, LIKE, GLOB, and REGEXP operators.
///
/// Return either the pDef argument (indicating no change) or a
/// new FuncDef structure that is marked as ephemeral using the
/// SQLITE_FUNC_EPHEM flag.
///
/// # Arguments
///
/// * `db` - Database connection for reporting malloc problems
/// * `pDef` - Function to possibly overload
/// * `nArg` - Number of arguments to the function
/// * `pExpr` - First argument to the function
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3VtabOverloadFunction(
    mut db: *mut sqlite3,
    mut pDef: *mut FuncDef,
    mut nArg: i32,
    mut pExpr: *mut Expr,
) -> *mut FuncDef {
    let mut pTab: *mut Table = unsafe { std::mem::zeroed() };
    let mut pVtab: *mut sqlite3_vtab = unsafe { std::mem::zeroed() };
    let mut pMod: *mut sqlite3_module = unsafe { std::mem::zeroed() };
    let mut xSFunc: Option<
        unsafe extern "C-unwind" fn(*mut sqlite3_context, i32, *mut *mut sqlite3_value),
    > = None;
    let mut pArg: *mut () = std::ptr::null_mut::<()>();
    let mut pNew: *mut FuncDef = unsafe { std::mem::zeroed() };
    let mut rc: i32 = 0 as i32;
    // Check to see the left operand is a column in a virtual table
    if pExpr == std::ptr::null_mut::<Expr>() {
        return pDef;
    }
    if (((unsafe { (*pExpr).op }) as u32) as i32) != (168 as i32) {
        return pDef;
    }
    0 as i32;
    pTab = unsafe { (*pExpr).y.pTab };
    if pTab == std::ptr::null_mut::<Table>() {
        return pDef;
    }
    if !((((unsafe { (*pTab).eTabType }) as u32) as i32) == (1 as i32)) {
        return pDef;
    }
    pVtab = unsafe { (*sqlite3GetVTable(db, pTab)).pVtab };
    0 as i32;
    0 as i32;
    pMod = (unsafe { (*pVtab).pModule }) as *mut sqlite3_module;
    if (unsafe { (*pMod).xFindFunction }) == None {
        return pDef;
    }
    // Call the xFindFunction method on the virtual table implementation
    // to see if the implementation wants to overload this function.
    //
    // Though undocumented, we have historically always invoked xFindFunction
    // with an all lower-case function name.  Continue in this tradition to
    // avoid any chance of an incompatibility.
    rc = unsafe {
        unsafe { (*pMod).xFindFunction }.unwrap()(
            pVtab,
            nArg,
            unsafe { (*pDef).zName },
            std::ptr::addr_of_mut!(xSFunc),
            std::ptr::addr_of_mut!(pArg),
        )
    };
    if rc == (0 as i32) {
        return pDef;
    }
    // Create a new ephemeral function definition for the overloaded
    // function
    pNew = (unsafe {
        sqlite3DbMallocZero(
            db,
            (72 as u64)
                .wrapping_add(
                    ((unsafe { sqlite3Strlen30(unsafe { (*pDef).zName }) }) as i64) as u64,
                )
                .wrapping_add(((1 as i32) as i64) as u64),
        )
    }) as *mut FuncDef;
    if pNew == std::ptr::null_mut::<FuncDef>() {
        return pDef;
    }
    unsafe {
        *pNew = unsafe { *pDef };
    }
    unsafe {
        (*pNew).zName = (unsafe { pNew.offset((1 as i32) as isize) }) as *const i8;
    }
    unsafe {
        memcpy(
            ((unsafe { pNew.offset((1 as i32) as isize) }) as *mut i8) as *mut (),
            (unsafe { (*pDef).zName }) as *const (),
            (((unsafe { sqlite3Strlen30(unsafe { (*pDef).zName }) }) + (1 as i32)) as i64) as u64,
        )
    };
    unsafe {
        (*pNew).xSFunc = xSFunc;
    }
    unsafe {
        (*pNew).pUserData = pArg;
    }
    let __v879: *mut FuncDef = pNew;
    let __v880: u32 = unsafe { (*__v879).funcFlags };
    let __v881: u32 = __v880 | ((16 as i32) as u32);
    unsafe {
        (*__v879).funcFlags = __v881;
    }
    return pNew;
}

/// Make sure virtual table pTab is contained in the pParse->apVirtualLock[]
/// array so that an OP_VBegin will get generated for it.  Add pTab to the
/// array if it is missing.  If pTab is already in the array, this routine
/// is a no-op.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3VtabMakeWritable(mut pParse: *mut Parse, mut pTab: *mut Table) {
    let mut pToplevel: *mut Parse =
        if (unsafe { (*pParse).pToplevel }) != std::ptr::null_mut::<Parse>() {
            unsafe { (*pParse).pToplevel }
        } else {
            pParse
        };
    let mut i: i32 = 0 as i32;
    let mut n: i32 = 0 as i32;
    let mut apVtabLock: *mut *mut Table = unsafe { std::mem::zeroed() };
    0 as i32;
    i = 0 as i32;
    '__slate_break_816: loop {
        if !(i < unsafe { (*pToplevel).nVtabLock }) {
            break;
        }
        if pTab == unsafe { *unsafe { unsafe { (*pToplevel).apVtabLock }.offset(i as isize) } } {
            return;
        }
        let __v865: i32 = i;
        let __v866: i32 = __v865 + (1 as i32);
        i = __v866;
    }
    n = (((((unsafe { (*pToplevel).nVtabLock }) + (1 as i32)) as i64) as u64).wrapping_mul(8 as u64)
        as u32) as i32;
    apVtabLock = (unsafe {
        sqlite3Realloc(
            (unsafe { (*pToplevel).apVtabLock }) as *mut (),
            (n as i64) as u64,
        )
    }) as *mut *mut Table;
    if apVtabLock != std::ptr::null_mut::<*mut Table>() {
        unsafe {
            (*pToplevel).apVtabLock = apVtabLock;
        }
        let __v867: *mut Parse = pToplevel;
        let __v868: i32 = unsafe { (*__v867).nVtabLock };
        let __v869: i32 = __v868 + (1 as i32);
        unsafe {
            (*__v867).nVtabLock = __v869;
        }
        unsafe {
            *unsafe { unsafe { (*pToplevel).apVtabLock }.offset(__v868 as isize) } = pTab;
        }
    } else {
        unsafe { sqlite3OomFault(unsafe { (*pToplevel).db }) };
    }
}

/// Check to see if virtual table module pMod can be have an eponymous
/// virtual table instance.  If it can, create one if one does not already
/// exist. Return non-zero if either the eponymous virtual table instance
/// exists when this routine returns or if an attempt to create it failed
/// and an error message was left in pParse.
///
/// An eponymous virtual table instance is one that is named after its
/// module, and more importantly, does not require a CREATE VIRTUAL TABLE
/// statement in order to come into existence.  Eponymous virtual table
/// instances always exist.  They cannot be DROP-ed.
///
/// Any virtual table module for which xConnect and xCreate are the same
/// method can have an eponymous virtual table instance.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3VtabEponymousTableInit(
    mut pParse: *mut Parse,
    mut pMod: *mut Module,
) -> i32 {
    let mut pModule: *const sqlite3_module = unsafe { (*pMod).pModule };
    let mut pTab: *mut Table = unsafe { std::mem::zeroed() };
    let mut zErr: *mut i8 = std::ptr::null_mut::<i8>();
    let mut rc: i32 = 0 as i32;
    let mut db: *mut sqlite3 = unsafe { (*pParse).db };
    if (unsafe { (*pMod).pEpoTab }) != std::ptr::null_mut::<Table>() {
        return 1 as i32;
    }
    if (unsafe { (*pModule).xCreate }) != None
        && (unsafe { (*pModule).xCreate }) != unsafe { (*pModule).xConnect }
    {
        return 0 as i32;
    }
    pTab = (unsafe { sqlite3DbMallocZero(db, 120 as u64) }) as *mut Table;
    if pTab == std::ptr::null_mut::<Table>() {
        return 0 as i32;
    }
    unsafe {
        (*pTab).zName = unsafe { sqlite3DbStrDup(db, unsafe { (*pMod).zName }) };
    }
    if (unsafe { (*pTab).zName }) == std::ptr::null_mut::<i8>() {
        unsafe { sqlite3DbFree(db, pTab as *mut ()) };
        return 0 as i32;
    }
    unsafe {
        (*pMod).pEpoTab = pTab;
    }
    unsafe {
        (*pTab).nTabRef = (1 as i32) as u32;
    }
    unsafe {
        (*pTab).eTabType = ((1 as i32) as i8) as u8;
    }
    unsafe {
        (*pTab).pSchema =
            unsafe { (*unsafe { unsafe { (*db).aDb }.offset((0 as i32) as isize) }).pSchema };
    }
    0 as i32;
    unsafe {
        (*pTab).iPKey = -(1 as i32) as i16;
    }
    let __v853: *mut Table = pTab;
    let __v854: u32 = unsafe { (*__v853).tabFlags };
    let __v855: u32 = __v854 | ((32768 as i32) as u32);
    unsafe {
        (*__v853).tabFlags = __v855;
    }
    addModuleArgument(pParse, pTab, unsafe {
        sqlite3DbStrDup(db, (unsafe { (*pTab).zName }) as *const i8)
    });
    addModuleArgument(pParse, pTab, std::ptr::null_mut::<i8>());
    addModuleArgument(pParse, pTab, unsafe {
        sqlite3DbStrDup(db, (unsafe { (*pTab).zName }) as *const i8)
    });
    let __v856: *mut sqlite3 = db;
    let __v857: u32 = unsafe { (*__v856).nSchemaLock };
    let __v858: u32 = __v857.wrapping_add((1 as i32) as u32);
    unsafe {
        (*__v856).nSchemaLock = __v858;
    }
    rc = vtabCallConstructor(
        db,
        pTab,
        pMod,
        unsafe { (*pModule).xConnect },
        std::ptr::addr_of_mut!(zErr),
    );
    let __v859: *mut sqlite3 = db;
    let __v860: u32 = unsafe { (*__v859).nSchemaLock };
    let __v861: u32 = __v860.wrapping_sub((1 as i32) as u32);
    unsafe {
        (*__v859).nSchemaLock = __v861;
    }
    if rc != (0 as i32) {
        unsafe { sqlite3ErrorMsg(pParse, (b"%s\0".as_ptr() as *mut i8) as *const i8, zErr) };
        unsafe {
            (*pParse).rc = rc;
        }
        unsafe { sqlite3DbFree(db, zErr as *mut ()) };
        sqlite3VtabEponymousTableClear(db, pMod);
    }
    return 1 as i32;
}

/// Erase the eponymous virtual table instance associated with
/// virtual table module pMod, if it exists.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3VtabEponymousTableClear(mut db: *mut sqlite3, mut pMod: *mut Module) {
    let mut pTab: *mut Table = unsafe { (*pMod).pEpoTab };
    if pTab != std::ptr::null_mut::<Table>() {
        // Mark the table as Ephemeral prior to deleting it, so that the
        // sqlite3DeleteTable() routine will know that it is not stored in
        // the schema.
        let __v862: *mut Table = pTab;
        let __v863: u32 = unsafe { (*__v862).tabFlags };
        let __v864: u32 = __v863 | ((16384 as i32) as u32);
        unsafe {
            (*__v862).tabFlags = __v864;
        }
        unsafe { sqlite3DeleteTable(db, pTab) };
        unsafe {
            (*pMod).pEpoTab = std::ptr::null_mut::<Table>();
        }
    }
}

/// Return the ON CONFLICT resolution mode in effect for the virtual
/// table update operation currently in progress.
///
/// The results of this routine are undefined unless it is called from
/// within an xUpdate method.
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.vtab.sqlite3_vtab_on_conflict")]
extern "C-unwind" fn sqlite3_vtab_on_conflict(mut db: *mut sqlite3) -> i32 {
    0 as i32;
    0 as i32;
    0 as i32;
    return ((unsafe {
        *unsafe {
            unsafe { std::ptr::addr_of!(aMap) as *const u8 }
                .offset(((((unsafe { (*db).vtabOnConflict }) as u32) as i32) - (1 as i32)) as isize)
        }
    }) as u32) as i32;
}

static mut aMap: [u8; 5] = [
    ((1 as i32) as i8) as u8,
    ((4 as i32) as i8) as u8,
    ((3 as i32) as i8) as u8,
    ((2 as i32) as i8) as u8,
    ((5 as i32) as i8) as u8,
];

/// Call from within the xCreate() or xConnect() methods to provide
/// the SQLite core with additional information about the behavior
/// of the virtual table being implemented.
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.vtab.sqlite3_vtab_config")]
unsafe extern "C-unwind" fn sqlite3_vtab_config(
    mut db: *mut sqlite3,
    mut op: i32,
    mut __va_args: ...
) -> i32 {
    let mut ap: core::ffi::VaList<'_> = unsafe { std::mem::zeroed() };
    let mut rc: i32 = 0 as i32;
    let mut p: *mut VtabCtx = unsafe { std::mem::zeroed() };
    unsafe { sqlite3_mutex_enter(unsafe { (*db).mutex }) };
    p = unsafe { (*db).pVtabCtx };
    if !(p != std::ptr::null_mut::<VtabCtx>()) {
        rc = unsafe { sqlite3MisuseError(1350 as i32) };
    } else {
        0 as i32;
        ap = __va_args.clone();
        '__slate_break_818: {
            match op {
                1 => unsafe {
                    (*unsafe { (*p).pVTable }).bConstraint =
                        ((unsafe { ap.next_arg::<i32>() }) as i8) as u8;
                },
                2 => unsafe {
                    (*unsafe { (*p).pVTable }).eVtabRisk = ((0 as i32) as i8) as u8;
                },
                3 => unsafe {
                    (*unsafe { (*p).pVTable }).eVtabRisk = ((2 as i32) as i8) as u8;
                },
                4 => unsafe {
                    (*unsafe { (*p).pVTable }).bAllSchemas = ((1 as i32) as i8) as u8;
                },
                _ => {
                    rc = unsafe { sqlite3MisuseError(1372 as i32) };
                }
            }
        }
        {}
    }
    if rc != (0 as i32) {
        unsafe { sqlite3Error(db, rc) };
    }
    unsafe { sqlite3_mutex_leave(unsafe { (*db).mutex }) };
    return rc;
}
