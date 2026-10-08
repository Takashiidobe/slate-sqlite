//! 2008 August 18
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
//! This file contains routines used for walking the parser tree and
//! resolve all identifiers by associating them with a particular
//! table and column.
unsafe extern "C" {
    fn sqlite3_stricmp(__v598: *const i8, __v599: *const i8) -> i32;
    fn sqlite3_strnicmp(__v600: *const i8, __v601: *const i8, __v602: i32) -> i32;
    fn sqlite3_log(iErrCode: i32, zFormat: *const i8, ...);
    fn memcpy(__dest: *mut (), __src: *const (), __n: u64) -> *mut ();
    fn memset(__s: *mut (), __c: i32, __n: u64) -> *mut ();
    fn strcmp(__s1: *const i8, __s2: *const i8) -> i32;
    fn sqlite3WalkExpr(__v613: *mut Walker, __v614: *mut Expr) -> i32;
    fn sqlite3WalkExprNN(__v615: *mut Walker, __v616: *mut Expr) -> i32;
    fn sqlite3WalkExprList(__v617: *mut Walker, __v618: *mut ExprList) -> i32;
    fn sqlite3WalkSelect(__v619: *mut Walker, __v620: *mut Select) -> i32;
    fn sqlite3WindowUnlinkFromSelect(__v621: *mut Window);
    fn sqlite3WindowLink(pSel: *mut Select, pWin: *mut Window);
    fn sqlite3WindowUpdate(
        __v624: *mut Parse,
        __v625: *mut Window,
        __v626: *mut Window,
        __v627: *mut FuncDef,
    );
    fn sqlite3StrICmp(__v628: *const i8, __v629: *const i8) -> i32;
    fn sqlite3ErrorMsg(__v630: *mut Parse, __v631: *const i8, ...);
    fn sqlite3ExprAlloc(
        __v632: *mut sqlite3,
        __v633: i32,
        __v634: *const Token,
        __v635: i32,
    ) -> *mut Expr;
    fn sqlite3ExprInt32(__v636: *mut sqlite3, __v637: i32) -> *mut Expr;
    fn sqlite3ExprOrderByAggregateError(__v638: *mut Parse, __v639: *mut Expr);
    fn sqlite3ExprFunctionUsable(__v640: *mut Parse, __v641: *const Expr, __v642: *const FuncDef);
    fn sqlite3ExprDelete(__v643: *mut sqlite3, __v644: *mut Expr);
    fn sqlite3ExprDeferredDelete(__v645: *mut Parse, __v646: *mut Expr) -> i32;
    fn sqlite3ExprListAppend(
        __v647: *mut Parse,
        __v648: *mut ExprList,
        __v649: *mut Expr,
    ) -> *mut ExprList;
    fn sqlite3ExprListDelete(__v650: *mut sqlite3, __v651: *mut ExprList);
    fn sqlite3TableColumnToStorage(__v652: *mut Table, __v653: i16) -> i16;
    fn sqlite3IdListIndex(__v654: *mut IdList, __v655: *const i8) -> i32;
    fn sqlite3SelectCheckOnClauses(pParse: *mut Parse, pSelect: *mut Select);
    fn sqlite3ExprCompare(
        __v658: *const Parse,
        __v659: *const Expr,
        __v660: *const Expr,
        __v661: i32,
    ) -> i32;
    fn sqlite3ReferencesSrcList(__v662: *mut Parse, __v663: *mut Expr, __v664: *mut SrcList)
    -> i32;
    fn sqlite3ExprIdToTrueFalse(__v665: *mut Expr) -> i32;
    fn sqlite3ExprIsInteger(
        __v666: *const Expr,
        __v667: *mut i32,
        __v668: *mut Parse,
        __v669: i32,
    ) -> i32;
    fn sqlite3ExprCanBeNull(__v670: *const Expr) -> i32;
    fn sqlite3IsRowid(__v671: *const i8) -> i32;
    fn sqlite3ExprDup(__v672: *mut sqlite3, __v673: *const Expr, __v674: i32) -> *mut Expr;
    fn sqlite3FindFunction(
        __v675: *mut sqlite3,
        __v676: *const i8,
        __v677: i32,
        __v678: u8,
        __v679: u8,
    ) -> *mut FuncDef;
    fn sqlite3ColumnIndex(pTab: *mut Table, zCol: *const i8) -> i32;
    fn sqlite3SrcItemColumnUsed(__v682: *mut SrcItem, __v683: i32);
    fn sqlite3AuthRead(
        __v684: *mut Parse,
        __v685: *mut Expr,
        __v686: *mut Schema,
        __v687: *mut SrcList,
    );
    fn sqlite3AuthCheck(
        __v688: *mut Parse,
        __v689: i32,
        __v690: *const i8,
        __v691: *const i8,
        __v692: *const i8,
    ) -> i32;
    fn sqlite3AtoF(z: *const i8, __v694: *mut f64) -> i32;
    fn sqlite3ExprAddCollateString(
        __v695: *const Parse,
        __v696: *mut Expr,
        __v697: *const i8,
    ) -> *mut Expr;
    fn sqlite3ExprSkipCollateAndLikely(__v698: *mut Expr) -> *mut Expr;
    fn sqlite3WritableSchema(__v699: *mut sqlite3) -> i32;
    fn sqlite3SelectPrep(__v700: *mut Parse, __v701: *mut Select, __v702: *mut NameContext);
    fn sqlite3SelectWrongNumTermsError(pParse: *mut Parse, p: *mut Select);
    fn sqlite3RenameTokenRemap(__v727: *mut Parse, pTo: *const (), pFrom: *const ());
    fn sqlite3RecordErrorOffsetOfExpr(__v734: *mut sqlite3, __v735: *const Expr);
    fn sqlite3ExprCheckHeight(__v736: *mut Parse, __v737: i32) -> i32;
    fn sqlite3ExprVectorSize(pExpr: *const Expr) -> i32;
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
    trace: __SlateRecord160,
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
    u1: __SlateRecord161,
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
    u: __SlateRecord162,
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
    u: __SlateRecord163,
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
    u: __SlateRecord171,
    pLeft: *mut Expr,
    pRight: *mut Expr,
    x: __SlateRecord172,
    nHeight: i32,
    iTable: i32,
    iColumn: i16,
    iAgg: i16,
    w: __SlateRecord173,
    pAggInfo: *mut AggInfo,
    y: __SlateRecord174,
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
    fg: __SlateRecord181,
    iCursor: i32,
    colUsed: u64,
    u1: __SlateRecord182,
    u2: __SlateRecord183,
    u3: __SlateRecord184,
    u4: __SlateRecord185,
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
    uNC: __SlateRecord186,
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
    __slate_bits_0: __slate_bits::__SlateBits101U0,
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
    u1: __SlateRecord188,
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
    u: __SlateRecord191,
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
    __slate_bits_0: __slate_bits::__SlateBits159U0,
    azInit: *mut *const i8,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord160 {
    xLegacy: Option<unsafe extern "C-unwind" fn(*mut (), *const i8)>,
    xV2: Option<unsafe extern "C-unwind" fn(u32, *mut (), *mut (), *mut ()) -> i32>,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord161 {
    isInterrupted: i32,
    notUsed1: f64,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord162 {
    pHash: *mut FuncDef,
    pDestructor: *mut FuncDestructor,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord163 {
    tab: __SlateRecord164,
    view: __SlateRecord165,
    vtab: __SlateRecord166,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord164 {
    addColOffset: i32,
    pFKey: *mut FKey,
    pDfltList: *mut ExprList,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord165 {
    pSelect: *mut Select,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord166 {
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
union __SlateRecord171 {
    zToken: *mut i8,
    iValue: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord172 {
    pList: *mut ExprList,
    pSelect: *mut Select,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord173 {
    iJoin: i32,
    iOfst: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord174 {
    pTab: *mut Table,
    pWin: *mut Window,
    nReg: i32,
    sub: __SlateRecord175,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord175 {
    iAddr: i32,
    regReturn: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct ExprList_item {
    pExpr: *mut Expr,
    zEName: *mut i8,
    fg: __SlateRecord177,
    u: __SlateRecord178,
}

#[repr(C, align(4))]
#[derive(Clone, Copy)]
struct __SlateRecord177 {
    sortFlags: u8,
    __slate_bits_0: __slate_bits::__SlateBits177U0,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord178 {
    x: __SlateRecord179,
    iConstExprReg: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord179 {
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
struct __SlateRecord181 {
    jointype: u8,
    __slate_bits_0: __slate_bits::__SlateBits181U0,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord182 {
    zIndexedBy: *mut i8,
    pFuncArg: *mut ExprList,
    nRow: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord183 {
    pIBIndex: *mut Index,
    pCteUse: *mut CteUse,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord184 {
    pOn: *mut Expr,
    pUsing: *mut IdList,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord185 {
    pSchema: *mut Schema,
    zDatabase: *mut i8,
    pSubq: *mut Subquery,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord186 {
    pEList: *mut ExprList,
    pAggInfo: *mut AggInfo,
    pUpsert: *mut Upsert,
    iBaseReg: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord188 {
    cr: __SlateRecord189,
    d: __SlateRecord190,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord189 {
    addrCrTab: i32,
    regRowid: i32,
    regRoot: i32,
    constraintName: Token,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord190 {
    pReturning: *mut Returning,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord191 {
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

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord200 {
    sSrc: SrcList,
    /// Memory space for the fake SrcList
    srcSpace: [u8; 80],
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
    pub struct __SlateBits177U0 {
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
    pub struct __SlateBits181U0 {
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
    pub struct __SlateBits159U0 {
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
    pub struct __SlateBits101U0 {
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

// Magic table number to mean the EXCLUDED table in an UPSERT statement.
/// Walk the expression tree pExpr and increase the aggregate function
/// depth (the Expr.op2 field) by N on every TK_AGG_FUNCTION node.
/// This needs to occur when copying a TK_AGG_FUNCTION node from an
/// outer query into an inner subquery.
///
/// incrAggFunctionDepth(pExpr,n) is the main routine.  incrAggDepth(..)
/// is a helper function - a callback for the tree walker.
///
/// See also the sqlite3WindowExtraAggFuncDepth() routine in window.c
#[unsafe(link_section = ".text.slate_distinct.resolve.incrAggDepth")]
extern "C-unwind" fn incrAggDepth(mut pWalker: *mut Walker, mut pExpr: *mut Expr) -> i32 {
    if (((unsafe { (*pExpr).op }) as u32) as i32) == (169 as i32) {
        let __v883: *mut Expr = pExpr;
        let __v884: u8 = unsafe { (*__v883).op2 };
        let __v885: u8 = ((((__v884 as u32) as i32) + unsafe { (*pWalker).u.n }) as i8) as u8;
        unsafe {
            (*__v883).op2 = __v885;
        }
    }
    return 0 as i32;
}

fn incrAggFunctionDepth(mut pExpr: *mut Expr, mut N: i32) {
    if N > (0 as i32) {
        let mut w: Walker = unsafe { std::mem::zeroed() };
        unsafe { memset(std::ptr::addr_of_mut!(w) as *mut (), 0 as i32, 48 as u64) };
        w.xExprCallback = Some(incrAggDepth);
        unsafe {
            w.u.n = N;
        }
        unsafe { sqlite3WalkExpr(std::ptr::addr_of_mut!(w), pExpr) };
    }
}

/// Turn the pExpr expression into an alias for the iCol-th column of the
/// result set in pEList.
///
/// If the reference is followed by a COLLATE operator, then make sure
/// the COLLATE operator is preserved.  For example:
///
///     SELECT a+b, c+d FROM t1 ORDER BY 1 COLLATE nocase;
///
/// Should be transformed into:
///
///     SELECT a+b, c+d FROM t1 ORDER BY (a+b) COLLATE nocase;
///
/// The nSubquery parameter specifies how many levels of subquery the
/// alias is removed from the original expression.  The usual value is
/// zero but it might be more if the alias is contained within a subquery
/// of the original expression.  The Expr.op2 field of TK_AGG_FUNCTION
/// structures must be increased by the nSubquery amount.
///
/// # Arguments
///
/// * `pParse` - Parsing context
/// * `pEList` - A result set
/// * `iCol` - A column in the result set.  0..pEList->nExpr-1
/// * `pExpr` - Transform this into an alias to the result set
/// * `nSubquery` - Number of subqueries that the label is moving
fn resolveAlias(
    mut pParse: *mut Parse,
    mut pEList: *mut ExprList,
    mut iCol: i32,
    mut pExpr: *mut Expr,
    mut nSubquery: i32,
) {
    let mut pOrig: *mut Expr = unsafe { std::mem::zeroed() }; // The iCol-th column of the result set
    let mut pDup: *mut Expr = unsafe { std::mem::zeroed() }; // Copy of pOrig
    let mut db: *mut sqlite3 = unsafe { std::mem::zeroed() }; // The database connection
    0 as i32;
    pOrig = unsafe {
        (*unsafe {
            unsafe { std::ptr::addr_of_mut!((*pEList).a) as *mut ExprList_item }
                .offset(iCol as isize)
        })
        .pExpr
    };
    0 as i32;
    0 as i32;
    if (unsafe { (*pExpr).pAggInfo }) != std::ptr::null_mut::<AggInfo>() {
        return;
    }
    db = unsafe { (*pParse).db };
    pDup = unsafe { sqlite3ExprDup(db, pOrig as *const Expr, 0 as i32) };
    if (unsafe { (*db).mallocFailed }) != (0 as u8) {
        unsafe { sqlite3ExprDelete(db, pDup) };
        pDup = std::ptr::null_mut::<Expr>();
    } else {
        let mut temp: Expr = unsafe { std::mem::zeroed() };
        incrAggFunctionDepth(pDup, nSubquery);
        if (((unsafe { (*pExpr).op }) as u32) as i32) == (114 as i32) {
            0 as i32;
            pDup = unsafe {
                sqlite3ExprAddCollateString(
                    pParse as *const Parse,
                    pDup,
                    (unsafe { (*pExpr).u.zToken }) as *const i8,
                )
            };
        }
        unsafe {
            memcpy(
                std::ptr::addr_of_mut!(temp) as *mut (),
                pDup as *const (),
                72 as u64,
            )
        };
        unsafe { memcpy(pDup as *mut (), pExpr as *const (), 72 as u64) };
        unsafe {
            memcpy(
                pExpr as *mut (),
                std::ptr::addr_of_mut!(temp) as *const (),
                72 as u64,
            )
        };
        if (unsafe { (*pExpr).flags }) & ((16777216 as i32) as u32) != ((0 as i32) as u32) {
            if (unsafe { (*pExpr).y.pWin }) != std::ptr::null_mut::<Window>() {
                unsafe {
                    (*unsafe { (*pExpr).y.pWin }).pOwner = pExpr;
                }
            }
        }
        unsafe { sqlite3ExprDeferredDelete(pParse, pDup) };
    }
}

/// Subqueries store the original database, table and column names for their
/// result sets in ExprList.a[].zSpan, in the form "DATABASE.TABLE.COLUMN",
/// and mark the expression-list item by setting ExprList.a[].fg.eEName
/// to ENAME_TAB.
///
/// Check to see if the zSpan/eEName of the expression-list item passed to this
/// routine matches the zDb, zTab, and zCol.  If any of zDb, zTab, and zCol are
/// NULL then those fields will match anything. Return true if there is a match,
/// or false otherwise.
///
/// SF_NestedFrom subqueries also store an entry for the implicit rowid (or
/// _rowid_, or oid) column by setting ExprList.a[].fg.eEName to ENAME_ROWID,
/// and setting zSpan to "DATABASE.TABLE.<rowid-alias>". This type of pItem
/// argument matches if zCol is a rowid alias. If it is not NULL, (*pbRowid)
/// is set to 1 if there is this kind of match.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3MatchEName(
    mut pItem: *const ExprList_item,
    mut zCol: *const i8,
    mut zTab: *const i8,
    mut zDb: *const i8,
    mut pbRowid: *mut i32,
) -> i32 {
    let mut n: i32 = 0 as i32;
    let mut zSpan: *const i8 = unsafe { std::mem::zeroed() };
    let mut eEName: i32 = (unsafe { (*pItem).fg.__slate_bits_0.__get_eEName() }) as i32;
    if eEName != (2 as i32) && (eEName != (3 as i32) || pbRowid == std::ptr::null_mut::<i32>()) {
        return 0 as i32;
    }
    0 as i32;
    zSpan = (unsafe { (*pItem).zEName }) as *const i8;
    n = 0 as i32;
    '__slate_break_739: loop {
        if !((unsafe { *unsafe { zSpan.offset(n as isize) } }) != (0 as i8)
            && ((unsafe { *unsafe { zSpan.offset(n as isize) } }) as i32) != (46 as i32))
        {
            break;
        }
        let __v822: i32 = n;
        let __v823: i32 = __v822 + (1 as i32);
        n = __v823;
    }
    let __v824: bool;
    if zDb != std::ptr::null::<i8>() {
        __v824 = (unsafe { sqlite3_strnicmp(zSpan, zDb, n) }) != (0 as i32)
            || ((unsafe { *unsafe { zDb.offset(n as isize) } }) as i32) != (0 as i32);
    } else {
        __v824 = false as bool;
    }
    if __v824 {
        return 0 as i32;
    }
    let __v825: *const i8 = zSpan;
    let __v826: *const i8 = unsafe { __v825.offset((n + (1 as i32)) as isize) };
    zSpan = __v826;
    n = 0 as i32;
    '__slate_break_740: loop {
        if !((unsafe { *unsafe { zSpan.offset(n as isize) } }) != (0 as i8)
            && ((unsafe { *unsafe { zSpan.offset(n as isize) } }) as i32) != (46 as i32))
        {
            break;
        }
        let __v827: i32 = n;
        let __v828: i32 = __v827 + (1 as i32);
        n = __v828;
    }
    let __v829: bool;
    if zTab != std::ptr::null::<i8>() {
        __v829 = (unsafe { sqlite3_strnicmp(zSpan, zTab, n) }) != (0 as i32)
            || ((unsafe { *unsafe { zTab.offset(n as isize) } }) as i32) != (0 as i32);
    } else {
        __v829 = false as bool;
    }
    if __v829 {
        return 0 as i32;
    }
    let __v830: *const i8 = zSpan;
    let __v831: *const i8 = unsafe { __v830.offset((n + (1 as i32)) as isize) };
    zSpan = __v831;
    if zCol != std::ptr::null::<i8>() {
        let __v832: bool;
        if eEName == (2 as i32) {
            __v832 = (unsafe { sqlite3StrICmp(zSpan, zCol) }) != (0 as i32);
        } else {
            __v832 = false as bool;
        }
        if __v832 {
            return 0 as i32;
        }
        let __v833: bool;
        if eEName == (3 as i32) {
            __v833 = (unsafe { sqlite3IsRowid(zCol) }) == (0 as i32);
        } else {
            __v833 = false as bool;
        }
        if __v833 {
            return 0 as i32;
        }
    }
    if eEName == (3 as i32) {
        unsafe {
            *pbRowid = 1 as i32;
        }
    }
    return 1 as i32;
}

/// Return TRUE if the double-quoted string  mis-feature should be supported.
fn areDoubleQuotedStringsEnabled(mut db: *mut sqlite3, mut pTopNC: *mut NameContext) -> i32 {
    if (unsafe { (*db).init.busy }) != (0 as u8) {
        return 1 as i32;
    }
    // Always support for legacy schemas
    if (unsafe { (*pTopNC).ncFlags }) & (65536 as i32) != (0 as i32) {
        // Currently parsing a DDL statement
        if (unsafe { sqlite3WritableSchema(db) }) != (0 as i32)
            && (unsafe { (*db).flags }) & (((1073741824 as i32) as i64) as u64)
                != (((0 as i32) as i64) as u64)
        {
            return 1 as i32;
        }
        return ((unsafe { (*db).flags }) & (((536870912 as i32) as i64) as u64)
            != (((0 as i32) as i64) as u64)) as i32;
    } else {
        // Currently parsing a DML statement
        return ((unsafe { (*db).flags }) & (((1073741824 as i32) as i64) as u64)
            != (((0 as i32) as i64) as u64)) as i32;
    }
    return unsafe { std::mem::zeroed() };
}

/// The argument is guaranteed to be a non-NULL Expr node of type TK_COLUMN.
/// return the appropriate colUsed mask.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3ExprColUsed(mut pExpr: *mut Expr) -> u64 {
    let mut n: i32 = 0 as i32;
    let mut pExTab: *mut Table = unsafe { std::mem::zeroed() };
    n = (unsafe { (*pExpr).iColumn }) as i32;
    0 as i32;
    pExTab = unsafe { (*pExpr).y.pTab };
    0 as i32;
    0 as i32;
    if (unsafe { (*pExTab).tabFlags }) & ((96 as i32) as u32) != ((0 as i32) as u32)
        && (((unsafe { (*unsafe { unsafe { (*pExTab).aCol }.offset(n as isize) }).colFlags })
            as u32) as i32)
            & (96 as i32)
            != (0 as i32)
    {
        {}
        {}
        return if ((unsafe { (*pExTab).nCol }) as i32)
            >= (((8 as u64).wrapping_mul(((8 as i32) as i64) as u64) as u32) as i32)
        {
            (-(1 as i32) as i64) as u64
        } else {
            ((((1 as i32) as i64) as u64) << ((unsafe { (*pExTab).nCol }) as i32))
                .wrapping_sub(((1 as i32) as i64) as u64)
        };
    } else {
        {}
        {}
        if n >= (((8 as u64).wrapping_mul(((8 as i32) as i64) as u64) as u32) as i32) {
            n = (((8 as u64).wrapping_mul(((8 as i32) as i64) as u64) as u32) as i32) - (1 as i32);
        }
        return (((1 as i32) as i64) as u64) << n;
    }
    return unsafe { std::mem::zeroed() };
}

/// Create a new expression term for the column specified by pMatch and
/// iColumn.  Append this new expression term to the FULL JOIN Match set
/// in *ppList.  Create a new *ppList if this is the first term in the
/// set.
///
/// # Arguments
///
/// * `pParse` - Parsing context
/// * `ppList` - ExprList to extend
/// * `pMatch` - Source table containing the column
/// * `iColumn` - The column number
fn extendFJMatch(
    mut pParse: *mut Parse,
    mut ppList: *mut *mut ExprList,
    mut pMatch: *mut SrcItem,
    mut iColumn: i16,
) {
    let mut pNew: *mut Expr = unsafe {
        sqlite3ExprAlloc(
            unsafe { (*pParse).db },
            168 as i32,
            std::ptr::null::<Token>(),
            0 as i32,
        )
    };
    if pNew != std::ptr::null_mut::<Expr>() {
        unsafe {
            (*pNew).iTable = unsafe { (*pMatch).iCursor };
        }
        unsafe {
            (*pNew).iColumn = iColumn;
        }
        unsafe {
            (*pNew).y.pTab = unsafe { (*pMatch).pSTab };
        }
        0 as i32;
        let __v886: *mut Expr = pNew;
        let __v887: u32 = unsafe { (*__v886).flags };
        let __v888: u32 = __v887 | ((2097152 as i32) as u32);
        unsafe {
            (*__v886).flags = __v888;
        }
        unsafe {
            *ppList = unsafe { sqlite3ExprListAppend(pParse, unsafe { *ppList }, pNew) };
        }
    }
}

/// Return TRUE (non-zero) if zTab is a valid name for the schema table pTab.
///
/// # Arguments
///
/// * `zTab` - Name as it appears in the SQL
/// * `pTab` - The schema table we are trying to match
/// * `zDb` - non-NULL if a database qualifier is present
fn isValidSchemaTableName(mut zTab: *const i8, mut pTab: *mut Table, mut zDb: *const i8) -> i32 {
    let mut zLegacy: *const i8 = unsafe { std::mem::zeroed() };
    0 as i32;
    0 as i32;
    if (unsafe {
        sqlite3_strnicmp(
            zTab,
            (b"sqlite_\0".as_ptr() as *mut i8) as *const i8,
            7 as i32,
        )
    }) != (0 as i32)
    {
        return 0 as i32;
    }
    zLegacy = (unsafe { (*pTab).zName }) as *const i8;
    if (unsafe {
        strcmp(
            unsafe { zLegacy.offset((7 as i32) as isize) },
            (unsafe { (b"sqlite_temp_master\0".as_ptr() as *mut i8).offset((7 as i32) as isize) })
                as *const i8,
        )
    }) == (0 as i32)
    {
        if (unsafe {
            sqlite3StrICmp(
                unsafe { zTab.offset((7 as i32) as isize) },
                (unsafe {
                    (b"sqlite_temp_schema\0".as_ptr() as *mut i8).offset((7 as i32) as isize)
                }) as *const i8,
            )
        }) == (0 as i32)
        {
            return 1 as i32;
        }
        if zDb == std::ptr::null::<i8>() {
            return 0 as i32;
        }
        if (unsafe {
            sqlite3StrICmp(
                unsafe { zTab.offset((7 as i32) as isize) },
                (unsafe { (b"sqlite_master\0".as_ptr() as *mut i8).offset((7 as i32) as isize) })
                    as *const i8,
            )
        }) == (0 as i32)
        {
            return 1 as i32;
        }
        if (unsafe {
            sqlite3StrICmp(
                unsafe { zTab.offset((7 as i32) as isize) },
                (unsafe { (b"sqlite_schema\0".as_ptr() as *mut i8).offset((7 as i32) as isize) })
                    as *const i8,
            )
        }) == (0 as i32)
        {
            return 1 as i32;
        }
    } else {
        if (unsafe {
            sqlite3StrICmp(
                unsafe { zTab.offset((7 as i32) as isize) },
                (unsafe { (b"sqlite_schema\0".as_ptr() as *mut i8).offset((7 as i32) as isize) })
                    as *const i8,
            )
        }) == (0 as i32)
        {
            return 1 as i32;
        }
    }
    return 0 as i32;
}

/// Given the name of a column of the form X.Y.Z or Y.Z or just Z, look up
/// that name in the set of source tables in pSrcList and make the pExpr
/// expression node refer back to that source column.  The following changes
/// are made to pExpr:
///
///    pExpr->iDb           Set the index in db->aDb[] of the database X
///                         (even if X is implied).
///    pExpr->iTable        Set to the cursor number for the table obtained
///                         from pSrcList.
///    pExpr->y.pTab        Points to the Table structure of X.Y (even if
///                         X and/or Y are implied.)
///    pExpr->iColumn       Set to the column number within the table.
///    pExpr->op            Set to TK_COLUMN.
///    pExpr->pLeft         Any expression this points to is deleted
///    pExpr->pRight        Any expression this points to is deleted.
///
/// The zDb variable is the name of the database (the "X").  This value may be
/// NULL meaning that name is of the form Y.Z or Z.  Any available database
/// can be used.  The zTable variable is the name of the table (the "Y").  This
/// value can be NULL if zDb is also NULL.  If zTable is NULL it
/// means that the form of the name is Z and that columns from any table
/// can be used.
///
/// If the name cannot be resolved unambiguously, leave an error message
/// in pParse and return WRC_Abort.  Return WRC_Prune on success.
///
/// # Arguments
///
/// * `pParse` - The parsing context
/// * `zDb` - Name of the database containing table, or NULL
/// * `zTab` - Name of table containing column, or NULL
/// * `pRight` - Name of the column.
/// * `pNC` - The name context used to resolve the name
/// * `pExpr` - Make this EXPR node point to the selected column
fn lookupName(
    mut pParse: *mut Parse,
    mut zDb: *const i8,
    mut zTab: *const i8,
    mut pRight: *const Expr,
    mut pNC: *mut NameContext,
    mut pExpr: *mut Expr,
) -> i32 {
    let mut __slate_storage_951: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_951: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_951) as *mut i32;
    let mut __slate_storage_950: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_950: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_950) as *mut i32;
    let mut __slate_storage_949: std::mem::MaybeUninit<*mut NameContext> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_949: *mut *mut NameContext =
        std::ptr::addr_of_mut!(__slate_storage_949) as *mut *mut NameContext;
    let mut __slate_storage_948: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_948: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_948) as *mut i32;
    let mut __slate_storage_947: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_947: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_947) as *mut i32;
    let mut __slate_storage_946: std::mem::MaybeUninit<u64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_946: *mut u64 = std::ptr::addr_of_mut!(__slate_storage_946) as *mut u64;
    let mut __slate_storage_945: std::mem::MaybeUninit<u64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_945: *mut u64 = std::ptr::addr_of_mut!(__slate_storage_945) as *mut u64;
    let mut __slate_storage_944: std::mem::MaybeUninit<*mut SrcItem> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_944: *mut *mut SrcItem =
        std::ptr::addr_of_mut!(__slate_storage_944) as *mut *mut SrcItem;
    let mut __slate_storage_943: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_943: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_943) as *mut u32;
    let mut __slate_storage_942: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_942: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_942) as *mut u32;
    let mut __slate_storage_941: std::mem::MaybeUninit<*mut Expr> = std::mem::MaybeUninit::uninit();
    let __slate_slot_941: *mut *mut Expr =
        std::ptr::addr_of_mut!(__slate_storage_941) as *mut *mut Expr;
    let mut __slate_storage_940: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_940: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_940) as *mut i32;
    let mut __slate_storage_939: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_939: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_939) as *mut i32;
    let mut __slate_storage_938: std::mem::MaybeUninit<*mut NameContext> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_938: *mut *mut NameContext =
        std::ptr::addr_of_mut!(__slate_storage_938) as *mut *mut NameContext;
    let mut __slate_storage_937: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_937: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_937) as *mut u32;
    let mut __slate_storage_936: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_936: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_936) as *mut u32;
    let mut __slate_storage_935: std::mem::MaybeUninit<*mut Expr> = std::mem::MaybeUninit::uninit();
    let __slate_slot_935: *mut *mut Expr =
        std::ptr::addr_of_mut!(__slate_storage_935) as *mut *mut Expr;
    let mut __slate_storage_433: std::mem::MaybeUninit<*const i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_433: *mut *const i8 =
        std::ptr::addr_of_mut!(__slate_storage_433) as *mut *const i8;
    let mut __slate_storage_934: std::mem::MaybeUninit<bool> = std::mem::MaybeUninit::uninit();
    let __slate_slot_934: *mut bool = std::ptr::addr_of_mut!(__slate_storage_934) as *mut bool;
    let mut __slate_storage_933: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_933: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_933) as *mut i32;
    let mut __slate_storage_932: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_932: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_932) as *mut i32;
    let mut __slate_storage_930: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_930: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_930) as *mut i32;
    let mut __slate_storage_929: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_929: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_929) as *mut i32;
    let mut __slate_storage_432: std::mem::MaybeUninit<*mut Expr> = std::mem::MaybeUninit::uninit();
    let __slate_slot_432: *mut *mut Expr =
        std::ptr::addr_of_mut!(__slate_storage_432) as *mut *mut Expr;
    let mut __slate_storage_931: std::mem::MaybeUninit<bool> = std::mem::MaybeUninit::uninit();
    let __slate_slot_931: *mut bool = std::ptr::addr_of_mut!(__slate_storage_931) as *mut bool;
    let mut __slate_storage_431: std::mem::MaybeUninit<*mut i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_431: *mut *mut i8 =
        std::ptr::addr_of_mut!(__slate_storage_431) as *mut *mut i8;
    // Perhaps the name is a reference to the ROWID
    let mut __slate_storage_928: std::mem::MaybeUninit<bool> = std::mem::MaybeUninit::uninit();
    let __slate_slot_928: *mut bool = std::ptr::addr_of_mut!(__slate_storage_928) as *mut bool;
    let mut __slate_storage_924: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_924: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_924) as *mut u32;
    let mut __slate_storage_923: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_923: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_923) as *mut u32;
    let mut __slate_storage_922: std::mem::MaybeUninit<*mut Parse> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_922: *mut *mut Parse =
        std::ptr::addr_of_mut!(__slate_storage_922) as *mut *mut Parse;
    let mut __slate_storage_927: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_927: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_927) as *mut u32;
    let mut __slate_storage_926: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_926: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_926) as *mut u32;
    let mut __slate_storage_925: std::mem::MaybeUninit<*mut Parse> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_925: *mut *mut Parse =
        std::ptr::addr_of_mut!(__slate_storage_925) as *mut *mut Parse;
    let mut __slate_storage_921: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_921: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_921) as *mut i32;
    let mut __slate_storage_920: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_920: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_920) as *mut i32;
    let mut __slate_storage_919: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_919: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_919) as *mut i32;
    let mut __slate_storage_918: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_918: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_918) as *mut i32;
    let mut __slate_storage_430: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_430: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_430) as *mut i32;
    let mut __slate_storage_917: std::mem::MaybeUninit<bool> = std::mem::MaybeUninit::uninit();
    let __slate_slot_917: *mut bool = std::ptr::addr_of_mut!(__slate_storage_917) as *mut bool;
    let mut __slate_storage_429: std::mem::MaybeUninit<*mut Upsert> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_429: *mut *mut Upsert =
        std::ptr::addr_of_mut!(__slate_storage_429) as *mut *mut Upsert;
    let mut __slate_storage_914: std::mem::MaybeUninit<bool> = std::mem::MaybeUninit::uninit();
    let __slate_slot_914: *mut bool = std::ptr::addr_of_mut!(__slate_storage_914) as *mut bool;
    let mut __slate_storage_913: std::mem::MaybeUninit<bool> = std::mem::MaybeUninit::uninit();
    let __slate_slot_913: *mut bool = std::ptr::addr_of_mut!(__slate_storage_913) as *mut bool;
    let mut __slate_storage_912: std::mem::MaybeUninit<bool> = std::mem::MaybeUninit::uninit();
    let __slate_slot_912: *mut bool = std::ptr::addr_of_mut!(__slate_storage_912) as *mut bool;
    let mut __slate_storage_916: std::mem::MaybeUninit<bool> = std::mem::MaybeUninit::uninit();
    let __slate_slot_916: *mut bool = std::ptr::addr_of_mut!(__slate_storage_916) as *mut bool;
    let mut __slate_storage_915: std::mem::MaybeUninit<bool> = std::mem::MaybeUninit::uninit();
    let __slate_slot_915: *mut bool = std::ptr::addr_of_mut!(__slate_storage_915) as *mut bool;
    let mut __slate_storage_428: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_428: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_428) as *mut i32;
    let mut __slate_storage_911: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_911: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_911) as *mut u32;
    let mut __slate_storage_910: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_910: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_910) as *mut u32;
    let mut __slate_storage_909: std::mem::MaybeUninit<*mut Expr> = std::mem::MaybeUninit::uninit();
    let __slate_slot_909: *mut *mut Expr =
        std::ptr::addr_of_mut!(__slate_storage_909) as *mut *mut Expr;
    let mut __slate_storage_896: std::mem::MaybeUninit<*mut SrcItem> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_896: *mut *mut SrcItem =
        std::ptr::addr_of_mut!(__slate_storage_896) as *mut *mut SrcItem;
    let mut __slate_storage_895: std::mem::MaybeUninit<*mut SrcItem> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_895: *mut *mut SrcItem =
        std::ptr::addr_of_mut!(__slate_storage_895) as *mut *mut SrcItem;
    let mut __slate_storage_894: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_894: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_894) as *mut i32;
    let mut __slate_storage_893: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_893: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_893) as *mut i32;
    let mut __slate_storage_908: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_908: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_908) as *mut i32;
    // pTab is a potential ROWID match.  Keep track of it and match
    // the ROWID later if that seems appropriate.  (Search for "cntTab"
    // to find related code.)  Only allow a ROWID match if there is
    // a single ROWID match candidate.
    //
    // The (much more common) non-SQLITE_ALLOW_ROWID_IN_VIEW case is
    // simpler since we require exactly one candidate, which will
    // always be a non-VIEW
    let mut __slate_storage_907: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_907: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_907) as *mut i32;
    let mut __slate_storage_906: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_906: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_906) as *mut i32;
    let mut __slate_storage_905: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_905: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_905) as *mut i32;
    let mut __slate_storage_904: std::mem::MaybeUninit<bool> = std::mem::MaybeUninit::uninit();
    let __slate_slot_904: *mut bool = std::ptr::addr_of_mut!(__slate_storage_904) as *mut bool;
    let mut __slate_storage_898: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_898: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_898) as *mut i32;
    let mut __slate_storage_897: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_897: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_897) as *mut i32;
    let mut __slate_storage_903: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_903: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_903) as *mut i32;
    let mut __slate_storage_902: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_902: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_902) as *mut i32;
    let mut __slate_storage_901: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_901: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_901) as *mut i32;
    let mut __slate_storage_900: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_900: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_900) as *mut i32;
    let mut __slate_storage_899: std::mem::MaybeUninit<bool> = std::mem::MaybeUninit::uninit();
    let __slate_slot_899: *mut bool = std::ptr::addr_of_mut!(__slate_storage_899) as *mut bool; // True if possible rowid match
    let mut __slate_storage_427: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_427: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_427) as *mut i32;
    let mut __slate_storage_426: std::mem::MaybeUninit<*mut Select> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_426: *mut *mut Select =
        std::ptr::addr_of_mut!(__slate_storage_426) as *mut *mut Select;
    // In this case, pItem is a subquery that has been formed from a
    // parenthesized subset of the FROM clause terms.  Example:
    //   .... FROM t1 LEFT JOIN (t2 RIGHT JOIN t3 USING(x)) USING(y) ...
    //                          \_________________________/
    //             This pItem -------------^
    let mut __slate_storage_425: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_425: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_425) as *mut i32;
    let mut __slate_storage_892: std::mem::MaybeUninit<*mut SrcItem> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_892: *mut *mut SrcItem =
        std::ptr::addr_of_mut!(__slate_storage_892) as *mut *mut SrcItem;
    let mut __slate_storage_424: std::mem::MaybeUninit<*mut SrcList> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_424: *mut *mut SrcList =
        std::ptr::addr_of_mut!(__slate_storage_424) as *mut *mut SrcList;
    let mut __slate_storage_423: std::mem::MaybeUninit<*mut ExprList> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_423: *mut *mut ExprList =
        std::ptr::addr_of_mut!(__slate_storage_423) as *mut *mut ExprList;
    let mut __slate_storage_891: std::mem::MaybeUninit<bool> = std::mem::MaybeUninit::uninit();
    let __slate_slot_891: *mut bool = std::ptr::addr_of_mut!(__slate_storage_891) as *mut bool;
    let mut __slate_storage_890: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_890: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_890) as *mut i32;
    let mut __slate_storage_889: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_889: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_889) as *mut i32;
    let mut __slate_storage_422: std::mem::MaybeUninit<*const i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_422: *mut *const i8 =
        std::ptr::addr_of_mut!(__slate_storage_422) as *mut *const i8; // Matches for FULL JOIN .. USING
    let mut __slate_storage_421: std::mem::MaybeUninit<*mut ExprList> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_421: *mut *mut ExprList =
        std::ptr::addr_of_mut!(__slate_storage_421) as *mut *mut ExprList; // Table holding the row
    let mut __slate_storage_420: std::mem::MaybeUninit<*mut Table> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_420: *mut *mut Table =
        std::ptr::addr_of_mut!(__slate_storage_420) as *mut *mut Table; // New value for pExpr->op on success
    let mut __slate_storage_419: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_419: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_419) as *mut i32; // Schema of the expression
    let mut __slate_storage_418: std::mem::MaybeUninit<*mut Schema> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_418: *mut *mut Schema =
        std::ptr::addr_of_mut!(__slate_storage_418) as *mut *mut Schema; // First namecontext in the list
    let mut __slate_storage_417: std::mem::MaybeUninit<*mut NameContext> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_417: *mut *mut NameContext =
        std::ptr::addr_of_mut!(__slate_storage_417) as *mut *mut NameContext; // The matching pSrcList item
    let mut __slate_storage_416: std::mem::MaybeUninit<*mut SrcItem> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_416: *mut *mut SrcItem =
        std::ptr::addr_of_mut!(__slate_storage_416) as *mut *mut SrcItem; // Use for looping over pSrcList items
    let mut __slate_storage_415: std::mem::MaybeUninit<*mut SrcItem> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_415: *mut *mut SrcItem =
        std::ptr::addr_of_mut!(__slate_storage_415) as *mut *mut SrcItem; // The database connection
    let mut __slate_storage_414: std::mem::MaybeUninit<*mut sqlite3> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_414: *mut *mut sqlite3 =
        std::ptr::addr_of_mut!(__slate_storage_414) as *mut *mut sqlite3; // How many levels of subquery
    let mut __slate_storage_413: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_413: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_413) as *mut i32; // Number of potential "rowid" matches
    let mut __slate_storage_412: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_412: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_412) as *mut i32; // Number of matching column names
    let mut __slate_storage_411: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_411: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_411) as *mut i32; // Loop counters
    let mut __slate_storage_410: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_410: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_410) as *mut i32;
    let mut __slate_storage_409: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_409: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_409) as *mut i32;
    unsafe {
        '__join_175: {
            std::ptr::write(__slate_slot_411, 0 as i32);
            std::ptr::write(__slate_slot_412, 0 as i32);
            std::ptr::write(__slate_slot_413, 0 as i32);
            std::ptr::write(__slate_slot_414, unsafe { (*pParse).db });
            std::ptr::write(__slate_slot_416, std::ptr::null_mut::<SrcItem>());
            std::ptr::write(__slate_slot_417, pNC);
            std::ptr::write(__slate_slot_418, std::ptr::null_mut::<Schema>());
            std::ptr::write(__slate_slot_419, 168 as i32);
            std::ptr::write(__slate_slot_420, std::ptr::null_mut::<Table>());
            std::ptr::write(__slate_slot_421, std::ptr::null_mut::<ExprList>());
            std::ptr::write(
                __slate_slot_422,
                (unsafe { (*pRight).u.zToken }) as *const i8,
            );
            0 as i32; // the name context cannot be NULL.
            0 as i32; // The Z in X.Y.Z cannot be NULL
            0 as i32;
            0 as i32;
            // Initialize the node to no-match
            unsafe {
                (*pExpr).iTable = -(1 as i32);
            }
            {}
            // Translate the schema name in zDb into a pointer to the corresponding
            // schema.  If not found, pSchema will remain NULL and nothing will match
            // resulting in an appropriate error message toward the end of this routine
            if zDb != std::ptr::null::<i8>() {
                {}
                {}
                if (unsafe { (*pNC).ncFlags }) & ((2 as i32) | (4 as i32)) != (0 as i32) {
                    // Silently ignore database qualifiers inside CHECK constraints and
                    // partial indices.  Do not raise errors because that might break
                    // legacy and because it does not hurt anything to just ignore the
                    // database name.
                    zDb = std::ptr::null::<i8>();
                } else {
                    *__slate_slot_409 = 0 as i32;
                    '__join_181: {
                        loop {
                            if *__slate_slot_409 < unsafe { (*(*__slate_slot_414)).nDb } {
                                0 as i32;
                                if (unsafe {
                                    sqlite3StrICmp(
                                        (unsafe {
                                            (*unsafe {
                                                unsafe { (*(*__slate_slot_414)).aDb }
                                                    .offset(*__slate_slot_409 as isize)
                                            })
                                            .zDbSName
                                        }) as *const i8,
                                        zDb,
                                    )
                                }) == (0 as i32)
                                {
                                    break;
                                } else {
                                    std::ptr::write(__slate_slot_889, *__slate_slot_409);
                                    std::ptr::write(
                                        __slate_slot_890,
                                        *__slate_slot_889 + (1 as i32),
                                    );
                                    *__slate_slot_409 = *__slate_slot_890;
                                }
                            } else {
                                break '__join_181;
                            }
                        }
                        *__slate_slot_418 = unsafe {
                            (*unsafe {
                                unsafe { (*(*__slate_slot_414)).aDb }
                                    .offset(*__slate_slot_409 as isize)
                            })
                            .pSchema
                        };
                    }
                    if *__slate_slot_409 == unsafe { (*(*__slate_slot_414)).nDb } {
                        *__slate_slot_891 = (unsafe {
                            sqlite3StrICmp((b"main\0".as_ptr() as *mut i8) as *const i8, zDb)
                        }) == (0 as i32);
                    } else {
                        *__slate_slot_891 = false as bool;
                    }
                    if *__slate_slot_891 {
                        // This branch is taken when the main database has been renamed
                        // using SQLITE_DBCONFIG_MAINDBNAME.
                        *__slate_slot_418 = unsafe {
                            (*unsafe {
                                unsafe { (*(*__slate_slot_414)).aDb }.offset((0 as i32) as isize)
                            })
                            .pSchema
                        };
                        zDb = (unsafe {
                            (*unsafe {
                                unsafe { (*(*__slate_slot_414)).aDb }.offset((0 as i32) as isize)
                            })
                            .zDbSName
                        }) as *const i8;
                    }
                }
            }
        }
        // Start at the inner-most context and move outward until a match is found
        0 as i32;
        '__join_12: {
            '__join_55: {
                loop {
                    std::ptr::write(__slate_slot_424, unsafe { (*pNC).pSrcList });
                    if *__slate_slot_424 != std::ptr::null_mut::<SrcList>() {
                        *__slate_slot_409 = 0 as i32;
                        std::ptr::write(__slate_slot_892, unsafe {
                            std::ptr::addr_of_mut!((*(*__slate_slot_424)).a) as *mut SrcItem
                        });
                        *__slate_slot_415 = *__slate_slot_892;
                        loop {
                            if *__slate_slot_409 < unsafe { (*(*__slate_slot_424)).nSrc } {
                                '__join_123: {
                                    *__slate_slot_420 = unsafe { (*(*__slate_slot_415)).pSTab };
                                    0 as i32;
                                    0 as i32;
                                    0 as i32;
                                    if ((unsafe {
                                        (*(*__slate_slot_415))
                                            .fg
                                            .__slate_bits_0
                                            .__get_isNestedFrom()
                                    }) as i32)
                                        != (0 as i32)
                                    {
                                        std::ptr::write(__slate_slot_425, 0 as i32);
                                        0 as i32;
                                        0 as i32;
                                        *__slate_slot_426 = unsafe {
                                            (*unsafe { (*(*__slate_slot_415)).u4.pSubq }).pSelect
                                        };
                                        0 as i32;
                                        *__slate_slot_423 =
                                            unsafe { (*(*__slate_slot_426)).pEList };
                                        0 as i32;
                                        0 as i32;
                                        *__slate_slot_410 = 0 as i32;
                                        '__loop_152: loop {
                                            if *__slate_slot_410
                                                < unsafe { (*(*__slate_slot_423)).nExpr }
                                            {
                                                '__join_153: {
                                                    std::ptr::write(__slate_slot_427, 0 as i32);
                                                    if !(sqlite3MatchEName(
                                                        (unsafe {
                                                            unsafe {
                                                                std::ptr::addr_of_mut!(
                                                                    (*(*__slate_slot_423)).a
                                                                )
                                                                    as *mut ExprList_item
                                                            }
                                                            .offset(*__slate_slot_410 as isize)
                                                        })
                                                            as *const ExprList_item,
                                                        *__slate_slot_422,
                                                        zTab,
                                                        zDb,
                                                        std::ptr::addr_of_mut!(*__slate_slot_427),
                                                    ) != (0 as i32))
                                                    {
                                                    } else {
                                                        if *__slate_slot_427 == (0 as i32) {
                                                            if *__slate_slot_411 > (0 as i32) {
                                                                if ((unsafe {
                                                                    (*(*__slate_slot_415))
                                                                        .fg
                                                                        .__slate_bits_0
                                                                        .__get_isUsing()
                                                                })
                                                                    as i32)
                                                                    == (0 as i32)
                                                                {
                                                                    *__slate_slot_899 =
                                                                        true as bool;
                                                                } else {
                                                                    *__slate_slot_899 = (unsafe {
                                                                        sqlite3IdListIndex(
                                                                            unsafe {
                                                                                (*(*__slate_slot_415)).u3.pUsing
                                                                            },
                                                                            *__slate_slot_422,
                                                                        )
                                                                    }) < (0
                                                                        as i32);
                                                                }
                                                                if *__slate_slot_899
                                                                    || *__slate_slot_416
                                                                        == *__slate_slot_415
                                                                {
                                                                    // Two or more tables have the same column name which is
                                                                    // not joined by USING. Or, a single table has two columns
                                                                    // that match a USING term (if pMatch==pItem). These are both
                                                                    // "ambiguous column name" errors. Signal as much by clearing
                                                                    // pFJMatch and letting cnt go above 1.
                                                                    unsafe {
                                                                        sqlite3ExprListDelete(
                                                                            *__slate_slot_414,
                                                                            *__slate_slot_421,
                                                                        )
                                                                    };
                                                                    *__slate_slot_421 =
                                                                        std::ptr::null_mut::<
                                                                            ExprList,
                                                                        >(
                                                                        );
                                                                } else {
                                                                    if (((unsafe {
                                                                        (*(*__slate_slot_415))
                                                                            .fg
                                                                            .jointype
                                                                    })
                                                                        as u32)
                                                                        as i32)
                                                                        & (16 as i32)
                                                                        == (0 as i32)
                                                                    {
                                                                        // An INNER or LEFT JOIN.  Use the left-most table
                                                                        break '__join_153;
                                                                    } else {
                                                                        if (((unsafe {
                                                                            (*(*__slate_slot_415))
                                                                                .fg
                                                                                .jointype
                                                                        })
                                                                            as u32)
                                                                            as i32)
                                                                            & (8 as i32)
                                                                            == (0 as i32)
                                                                        {
                                                                            // A RIGHT JOIN.  Use the right-most table
                                                                            *__slate_slot_411 =
                                                                                0 as i32;
                                                                            unsafe {
                                                                                sqlite3ExprListDelete(*__slate_slot_414, *__slate_slot_421)
                                                                            };
                                                                            *__slate_slot_421 =
                                                                                std::ptr::null_mut::<
                                                                                    ExprList,
                                                                                >(
                                                                                );
                                                                        } else {
                                                                            // For a FULL JOIN, we must construct a coalesce() func
                                                                            extendFJMatch(pParse, std::ptr::addr_of_mut!(*__slate_slot_421), *__slate_slot_416, unsafe { (*pExpr).iColumn });
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                            std::ptr::write(
                                                                __slate_slot_900,
                                                                *__slate_slot_411,
                                                            );
                                                            std::ptr::write(
                                                                __slate_slot_901,
                                                                *__slate_slot_900 + (1 as i32),
                                                            );
                                                            *__slate_slot_411 = *__slate_slot_901;
                                                            *__slate_slot_425 = 1 as i32;
                                                        } else {
                                                            if *__slate_slot_411 > (0 as i32) {
                                                                // This is a potential rowid match, but there has already been
                                                                // a real match found. So this can be ignored.
                                                                break '__join_153;
                                                            }
                                                        }
                                                        std::ptr::write(
                                                            __slate_slot_902,
                                                            *__slate_slot_412,
                                                        );
                                                        std::ptr::write(
                                                            __slate_slot_903,
                                                            *__slate_slot_902 + (1 as i32),
                                                        );
                                                        *__slate_slot_412 = *__slate_slot_903;
                                                        *__slate_slot_416 = *__slate_slot_415;
                                                        unsafe {
                                                            (*pExpr).iColumn =
                                                                *__slate_slot_410 as i16;
                                                        }
                                                        unsafe {
                                                            (*unsafe {
                                                                unsafe {
                                                                    std::ptr::addr_of_mut!(
                                                                        (*(*__slate_slot_423)).a
                                                                    )
                                                                        as *mut ExprList_item
                                                                }
                                                                .offset(*__slate_slot_410 as isize)
                                                            })
                                                            .fg
                                                            .__slate_bits_0
                                                            .__set_bUsed((1 as i32) as u32);
                                                        }
                                                        // rowid cannot be part of a USING clause - assert() this.
                                                        0 as i32;
                                                        if ((unsafe {
                                                            (*unsafe {
                                                                unsafe {
                                                                    std::ptr::addr_of_mut!(
                                                                        (*(*__slate_slot_423)).a
                                                                    )
                                                                        as *mut ExprList_item
                                                                }
                                                                .offset(*__slate_slot_410 as isize)
                                                            })
                                                            .fg
                                                            .__slate_bits_0
                                                            .__get_bUsingTerm()
                                                        })
                                                            as i32)
                                                            != (0 as i32)
                                                        {
                                                            break '__loop_152;
                                                        }
                                                    }
                                                }
                                                std::ptr::write(
                                                    __slate_slot_897,
                                                    *__slate_slot_410,
                                                );
                                                std::ptr::write(
                                                    __slate_slot_898,
                                                    *__slate_slot_897 + (1 as i32),
                                                );
                                                *__slate_slot_410 = *__slate_slot_898;
                                            } else {
                                                break;
                                            }
                                        }
                                        if *__slate_slot_425 != (0 as i32)
                                            || zTab == std::ptr::null::<i8>()
                                        {
                                            break '__join_123;
                                        }
                                    }
                                    0 as i32;
                                    if zTab != std::ptr::null::<i8>() {
                                        if zDb != std::ptr::null::<i8>() {
                                            if (unsafe { (*(*__slate_slot_420)).pSchema })
                                                != *__slate_slot_418
                                            {
                                                break '__join_123;
                                            } else {
                                                if *__slate_slot_418
                                                    == std::ptr::null_mut::<Schema>()
                                                    && (unsafe {
                                                        strcmp(
                                                            zDb,
                                                            (b"*\0".as_ptr() as *mut i8)
                                                                as *const i8,
                                                        )
                                                    }) != (0 as i32)
                                                {
                                                    break '__join_123;
                                                }
                                            }
                                        }
                                        if (unsafe { (*(*__slate_slot_415)).zAlias })
                                            != std::ptr::null_mut::<i8>()
                                        {
                                            if (unsafe {
                                                sqlite3StrICmp(
                                                    zTab,
                                                    (unsafe { (*(*__slate_slot_415)).zAlias })
                                                        as *const i8,
                                                )
                                            }) != (0 as i32)
                                            {
                                                break '__join_123;
                                            }
                                        } else {
                                            if (unsafe {
                                                sqlite3StrICmp(
                                                    zTab,
                                                    (unsafe { (*(*__slate_slot_420)).zName })
                                                        as *const i8,
                                                )
                                            }) != (0 as i32)
                                            {
                                                if (unsafe { (*(*__slate_slot_420)).tnum })
                                                    != ((1 as i32) as u32)
                                                {
                                                    break '__join_123;
                                                } else {
                                                    if !(isValidSchemaTableName(
                                                        zTab,
                                                        *__slate_slot_420,
                                                        zDb,
                                                    ) != (0 as i32))
                                                    {
                                                        break '__join_123;
                                                    }
                                                }
                                            }
                                        }
                                        0 as i32;
                                        if (((unsafe { (*pParse).eParseMode }) as u32) as i32)
                                            >= (2 as i32)
                                            && (unsafe { (*(*__slate_slot_415)).zAlias })
                                                != std::ptr::null_mut::<i8>()
                                        {
                                            unsafe {
                                                sqlite3RenameTokenRemap(
                                                    pParse,
                                                    std::ptr::null::<()>(),
                                                    ((unsafe {
                                                        std::ptr::addr_of_mut!((*pExpr).y.pTab)
                                                    })
                                                        as *mut ())
                                                        as *const (),
                                                )
                                            };
                                        }
                                    }
                                    *__slate_slot_410 = unsafe {
                                        sqlite3ColumnIndex(*__slate_slot_420, *__slate_slot_422)
                                    };
                                    if *__slate_slot_410 >= (0 as i32) {
                                        if *__slate_slot_411 > (0 as i32) {
                                            if ((unsafe {
                                                (*(*__slate_slot_415))
                                                    .fg
                                                    .__slate_bits_0
                                                    .__get_isUsing()
                                            })
                                                as i32)
                                                == (0 as i32)
                                            {
                                                *__slate_slot_904 = true as bool;
                                            } else {
                                                *__slate_slot_904 = (unsafe {
                                                    sqlite3IdListIndex(
                                                        unsafe { (*(*__slate_slot_415)).u3.pUsing },
                                                        *__slate_slot_422,
                                                    )
                                                }) < (0 as i32);
                                            }
                                            if *__slate_slot_904 {
                                                // Two or more tables have the same column name which is
                                                // not joined by USING.  This is an error.  Signal as much
                                                // by clearing pFJMatch and letting cnt go above 1.
                                                unsafe {
                                                    sqlite3ExprListDelete(
                                                        *__slate_slot_414,
                                                        *__slate_slot_421,
                                                    )
                                                };
                                                *__slate_slot_421 =
                                                    std::ptr::null_mut::<ExprList>();
                                            } else {
                                                if (((unsafe { (*(*__slate_slot_415)).fg.jointype })
                                                    as u32)
                                                    as i32)
                                                    & (16 as i32)
                                                    == (0 as i32)
                                                {
                                                    // An INNER or LEFT JOIN.  Use the left-most table
                                                    break '__join_123;
                                                } else {
                                                    if (((unsafe {
                                                        (*(*__slate_slot_415)).fg.jointype
                                                    })
                                                        as u32)
                                                        as i32)
                                                        & (8 as i32)
                                                        == (0 as i32)
                                                    {
                                                        // A RIGHT JOIN.  Use the right-most table
                                                        *__slate_slot_411 = 0 as i32;
                                                        unsafe {
                                                            sqlite3ExprListDelete(
                                                                *__slate_slot_414,
                                                                *__slate_slot_421,
                                                            )
                                                        };
                                                        *__slate_slot_421 =
                                                            std::ptr::null_mut::<ExprList>();
                                                    } else {
                                                        // For a FULL JOIN, we must construct a coalesce() func
                                                        extendFJMatch(
                                                            pParse,
                                                            std::ptr::addr_of_mut!(
                                                                *__slate_slot_421
                                                            ),
                                                            *__slate_slot_416,
                                                            unsafe { (*pExpr).iColumn },
                                                        );
                                                    }
                                                }
                                            }
                                        }
                                        std::ptr::write(__slate_slot_905, *__slate_slot_411);
                                        std::ptr::write(
                                            __slate_slot_906,
                                            *__slate_slot_905 + (1 as i32),
                                        );
                                        *__slate_slot_411 = *__slate_slot_906;
                                        *__slate_slot_416 = *__slate_slot_415;
                                        // Substitute the rowid (column -1) for the INTEGER PRIMARY KEY
                                        unsafe {
                                            (*pExpr).iColumn = (if *__slate_slot_410
                                                == ((unsafe { (*(*__slate_slot_420)).iPKey })
                                                    as i32)
                                            {
                                                -(1 as i32)
                                            } else {
                                                (*__slate_slot_410 as i16) as i32
                                            })
                                                as i16;
                                        }
                                        if ((unsafe {
                                            (*(*__slate_slot_415))
                                                .fg
                                                .__slate_bits_0
                                                .__get_isNestedFrom()
                                        }) as i32)
                                            != (0 as i32)
                                        {
                                            unsafe {
                                                sqlite3SrcItemColumnUsed(
                                                    *__slate_slot_415,
                                                    *__slate_slot_410,
                                                )
                                            };
                                        }
                                    }
                                    if (0 as i32) == *__slate_slot_411
                                        && (unsafe { (*(*__slate_slot_420)).tabFlags })
                                            & ((512 as i32) as u32)
                                            == ((0 as i32) as u32)
                                    {
                                        std::ptr::write(__slate_slot_907, *__slate_slot_412);
                                        std::ptr::write(
                                            __slate_slot_908,
                                            *__slate_slot_907 + (1 as i32),
                                        );
                                        *__slate_slot_412 = *__slate_slot_908;
                                        *__slate_slot_416 = *__slate_slot_415;
                                    }
                                }
                                std::ptr::write(__slate_slot_893, *__slate_slot_409);
                                std::ptr::write(__slate_slot_894, *__slate_slot_893 + (1 as i32));
                                *__slate_slot_409 = *__slate_slot_894;
                                std::ptr::write(__slate_slot_895, *__slate_slot_415);
                                std::ptr::write(__slate_slot_896, unsafe {
                                    (*__slate_slot_895).offset((1 as i32) as isize)
                                });
                                *__slate_slot_415 = *__slate_slot_896;
                            } else {
                                break;
                            }
                        }
                        if *__slate_slot_416 != std::ptr::null_mut::<SrcItem>() {
                            unsafe {
                                (*pExpr).iTable = unsafe { (*(*__slate_slot_416)).iCursor };
                            }
                            0 as i32;
                            unsafe {
                                (*pExpr).y.pTab = unsafe { (*(*__slate_slot_416)).pSTab };
                            }
                            if (((unsafe { (*(*__slate_slot_416)).fg.jointype }) as u32) as i32)
                                & ((8 as i32) | (64 as i32))
                                != (0 as i32)
                            {
                                std::ptr::write(__slate_slot_909, pExpr);
                                std::ptr::write(__slate_slot_910, unsafe {
                                    (*(*__slate_slot_909)).flags
                                });
                                std::ptr::write(
                                    __slate_slot_911,
                                    *__slate_slot_910 | ((2097152 as i32) as u32),
                                );
                                unsafe {
                                    (*(*__slate_slot_909)).flags = *__slate_slot_911;
                                }
                            }
                            *__slate_slot_418 = unsafe { (*unsafe { (*pExpr).y.pTab }).pSchema };
                        }
                    }
                    // if( pSrcList )
                    // If we have not already resolved the name, then maybe
                    // it is a new.* or old.* trigger argument reference.  Or
                    // maybe it is an excluded.* from an upsert.  Or maybe it is
                    // a reference in the RETURNING clause to a table being modified.
                    if *__slate_slot_411 == (0 as i32) && zDb == std::ptr::null::<i8>() {
                        *__slate_slot_420 = std::ptr::null_mut::<Table>();
                        if (unsafe { (*pParse).pTriggerTab }) != std::ptr::null_mut::<Table>() {
                            std::ptr::write(
                                __slate_slot_428,
                                ((unsafe { (*pParse).eTriggerOp }) as u32) as i32,
                            );
                            0 as i32;
                            if ((unsafe { (*pParse).__slate_bits_0.__get_bReturning() }) as i32)
                                != (0 as i32)
                            {
                                if (unsafe { (*pNC).ncFlags }) & (1024 as i32) != (0 as i32) {
                                    if zTab == std::ptr::null::<i8>() {
                                        *__slate_slot_913 = true as bool;
                                    } else {
                                        *__slate_slot_913 = (unsafe {
                                            sqlite3StrICmp(
                                                zTab,
                                                (unsafe {
                                                    (*unsafe { (*pParse).pTriggerTab }).zName
                                                })
                                                    as *const i8,
                                            )
                                        }) == (0 as i32);
                                    }
                                    if *__slate_slot_913 {
                                        *__slate_slot_914 = true as bool;
                                    } else {
                                        *__slate_slot_914 = isValidSchemaTableName(
                                            zTab,
                                            unsafe { (*pParse).pTriggerTab },
                                            std::ptr::null::<i8>(),
                                        ) != (0 as i32);
                                    }
                                    *__slate_slot_912 = *__slate_slot_914;
                                } else {
                                    *__slate_slot_912 = false as bool;
                                }
                                if *__slate_slot_912 {
                                    unsafe {
                                        (*pExpr).iTable =
                                            (*__slate_slot_428 != (129 as i32)) as i32;
                                    }
                                    *__slate_slot_420 = unsafe { (*pParse).pTriggerTab };
                                }
                            } else {
                                if *__slate_slot_428 != (129 as i32)
                                    && zTab != std::ptr::null::<i8>()
                                {
                                    *__slate_slot_915 = (unsafe {
                                        sqlite3StrICmp(
                                            (b"new\0".as_ptr() as *mut i8) as *const i8,
                                            zTab,
                                        )
                                    }) == (0 as i32);
                                } else {
                                    *__slate_slot_915 = false as bool;
                                }
                                if *__slate_slot_915 {
                                    unsafe {
                                        (*pExpr).iTable = 1 as i32;
                                    }
                                    *__slate_slot_420 = unsafe { (*pParse).pTriggerTab };
                                } else {
                                    if *__slate_slot_428 != (128 as i32)
                                        && zTab != std::ptr::null::<i8>()
                                    {
                                        *__slate_slot_916 = (unsafe {
                                            sqlite3StrICmp(
                                                (b"old\0".as_ptr() as *mut i8) as *const i8,
                                                zTab,
                                            )
                                        }) == (0 as i32);
                                    } else {
                                        *__slate_slot_916 = false as bool;
                                    }
                                    if *__slate_slot_916 {
                                        unsafe {
                                            (*pExpr).iTable = 0 as i32;
                                        }
                                        *__slate_slot_420 = unsafe { (*pParse).pTriggerTab };
                                    }
                                }
                            }
                        }
                        if (unsafe { (*pNC).ncFlags }) & (512 as i32) != (0 as i32)
                            && zTab != std::ptr::null::<i8>()
                        {
                            std::ptr::write(__slate_slot_429, unsafe { (*pNC).uNC.pUpsert });
                            if *__slate_slot_429 != std::ptr::null_mut::<Upsert>() {
                                *__slate_slot_917 = (unsafe {
                                    sqlite3StrICmp(
                                        (b"excluded\0".as_ptr() as *mut i8) as *const i8,
                                        zTab,
                                    )
                                }) == (0 as i32);
                            } else {
                                *__slate_slot_917 = false as bool;
                            }
                            if *__slate_slot_917 {
                                *__slate_slot_420 = unsafe {
                                    (*unsafe {
                                        unsafe {
                                            std::ptr::addr_of_mut!(
                                                (*unsafe { (*(*__slate_slot_429)).pUpsertSrc }).a
                                            )
                                                as *mut SrcItem
                                        }
                                        .offset((0 as i32) as isize)
                                    })
                                    .pSTab
                                };
                                unsafe {
                                    (*pExpr).iTable = 2 as i32;
                                }
                            }
                        }
                        if *__slate_slot_420 != std::ptr::null_mut::<Table>() {
                            *__slate_slot_418 = unsafe { (*(*__slate_slot_420)).pSchema };
                            std::ptr::write(__slate_slot_918, *__slate_slot_412);
                            std::ptr::write(__slate_slot_919, *__slate_slot_918 + (1 as i32));
                            *__slate_slot_412 = *__slate_slot_919;
                            *__slate_slot_430 =
                                unsafe { sqlite3ColumnIndex(*__slate_slot_420, *__slate_slot_422) };
                            if *__slate_slot_430 >= (0 as i32) {
                                if ((unsafe { (*(*__slate_slot_420)).iPKey }) as i32)
                                    == *__slate_slot_430
                                {
                                    *__slate_slot_430 = -(1 as i32);
                                }
                            } else {
                                if (unsafe { sqlite3IsRowid(*__slate_slot_422) }) != (0 as i32)
                                    && (unsafe { (*(*__slate_slot_420)).tabFlags })
                                        & ((512 as i32) as u32)
                                        == ((0 as i32) as u32)
                                {
                                    *__slate_slot_430 = -(1 as i32);
                                } else {
                                    *__slate_slot_430 =
                                        (unsafe { (*(*__slate_slot_420)).nCol }) as i32;
                                }
                            }
                            if *__slate_slot_430 < ((unsafe { (*(*__slate_slot_420)).nCol }) as i32)
                            {
                                std::ptr::write(__slate_slot_920, *__slate_slot_411);
                                std::ptr::write(__slate_slot_921, *__slate_slot_920 + (1 as i32));
                                *__slate_slot_411 = *__slate_slot_921;
                                *__slate_slot_416 = std::ptr::null_mut::<SrcItem>();
                                if (unsafe { (*pExpr).iTable }) == (2 as i32) {
                                    {}
                                    0 as i32;
                                    if (((unsafe { (*pParse).eParseMode }) as u32) as i32)
                                        >= (2 as i32)
                                    {
                                        unsafe {
                                            (*pExpr).iColumn = *__slate_slot_430 as i16;
                                        }
                                        unsafe {
                                            (*pExpr).y.pTab = *__slate_slot_420;
                                        }
                                        *__slate_slot_419 = 168 as i32;
                                    } else {
                                        unsafe {
                                            (*pExpr).iTable = (unsafe {
                                                (*unsafe { (*pNC).uNC.pUpsert }).regData
                                            }) + ((unsafe {
                                                sqlite3TableColumnToStorage(
                                                    *__slate_slot_420,
                                                    *__slate_slot_430 as i16,
                                                )
                                            })
                                                as i32);
                                        }
                                        *__slate_slot_419 = 176 as i32;
                                    }
                                } else {
                                    0 as i32;
                                    unsafe {
                                        (*pExpr).y.pTab = *__slate_slot_420;
                                    }
                                    if ((unsafe { (*pParse).__slate_bits_0.__get_bReturning() })
                                        as i32)
                                        != (0 as i32)
                                    {
                                        *__slate_slot_419 = 176 as i32;
                                        unsafe {
                                            (*pExpr).op2 = ((168 as i32) as i8) as u8;
                                        }
                                        unsafe {
                                            (*pExpr).iColumn = *__slate_slot_430 as i16;
                                        }
                                        unsafe {
                                            (*pExpr).iTable = (unsafe { (*pNC).uNC.iBaseReg })
                                                + (((unsafe { (*(*__slate_slot_420)).nCol })
                                                    as i32)
                                                    + (1 as i32))
                                                    * unsafe { (*pExpr).iTable }
                                                + ((unsafe {
                                                    sqlite3TableColumnToStorage(
                                                        *__slate_slot_420,
                                                        *__slate_slot_430 as i16,
                                                    )
                                                })
                                                    as i32)
                                                + (1 as i32);
                                        }
                                    } else {
                                        unsafe {
                                            (*pExpr).iColumn = *__slate_slot_430 as i16;
                                        }
                                        *__slate_slot_419 = 78 as i32;
                                        if *__slate_slot_430 < (0 as i32) {
                                            unsafe {
                                                (*pExpr).affExpr = (68 as i32) as i8;
                                            }
                                        } else {
                                            if (unsafe { (*pExpr).iTable }) == (0 as i32) {
                                                {}
                                                {}
                                                std::ptr::write(__slate_slot_922, pParse);
                                                std::ptr::write(__slate_slot_923, unsafe {
                                                    (*(*__slate_slot_922)).oldmask
                                                });
                                                std::ptr::write(
                                                    __slate_slot_924,
                                                    *__slate_slot_923
                                                        | if *__slate_slot_430 >= (32 as i32) {
                                                            4294967295 as u32
                                                        } else {
                                                            ((1 as i32) as u32) << *__slate_slot_430
                                                        },
                                                );
                                                unsafe {
                                                    (*(*__slate_slot_922)).oldmask =
                                                        *__slate_slot_924;
                                                }
                                            } else {
                                                {}
                                                {}
                                                std::ptr::write(__slate_slot_925, pParse);
                                                std::ptr::write(__slate_slot_926, unsafe {
                                                    (*(*__slate_slot_925)).newmask
                                                });
                                                std::ptr::write(
                                                    __slate_slot_927,
                                                    *__slate_slot_926
                                                        | if *__slate_slot_430 >= (32 as i32) {
                                                            4294967295 as u32
                                                        } else {
                                                            ((1 as i32) as u32) << *__slate_slot_430
                                                        },
                                                );
                                                unsafe {
                                                    (*(*__slate_slot_925)).newmask =
                                                        *__slate_slot_927;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    if *__slate_slot_411 == (0 as i32)
                        && *__slate_slot_412 >= (1 as i32)
                        && *__slate_slot_416 != std::ptr::null_mut::<SrcItem>()
                        && (unsafe { (*pNC).ncFlags }) & ((32 as i32) | (8 as i32)) == (0 as i32)
                    {
                        *__slate_slot_928 =
                            (unsafe { sqlite3IsRowid(*__slate_slot_422) }) != (0 as i32);
                    } else {
                        *__slate_slot_928 = false as bool;
                    }
                    if *__slate_slot_928
                        && ((unsafe { (*unsafe { (*(*__slate_slot_416)).pSTab }).tabFlags })
                            & ((512 as i32) as u32)
                            == ((0 as i32) as u32)
                            || ((unsafe {
                                (*(*__slate_slot_416))
                                    .fg
                                    .__slate_bits_0
                                    .__get_isNestedFrom()
                            }) as i32)
                                != (0 as i32))
                    {
                        *__slate_slot_411 = *__slate_slot_412;
                        if ((unsafe {
                            (*(*__slate_slot_416))
                                .fg
                                .__slate_bits_0
                                .__get_isNestedFrom()
                        }) as i32)
                            == (0 as i32)
                        {
                            unsafe {
                                (*pExpr).iColumn = -(1 as i32) as i16;
                            }
                        }
                        unsafe {
                            (*pExpr).affExpr = (68 as i32) as i8;
                        }
                    }
                    '__join_45: {
                        // If the input is of the form Z (not Y.Z or X.Y.Z) then the name Z
                        // might refer to an result-set alias.  This happens, for example, when
                        // we are resolving names in the WHERE clause of the following command:
                        //
                        //     SELECT a+b AS x FROM table WHERE x<10;
                        //
                        // In cases like this, replace pExpr with a copy of the expression that
                        // forms the result set entry ("a+b" in the example) and return immediately.
                        // Note that the expression in the result set should have already been
                        // resolved by the time the WHERE clause is resolved.
                        //
                        // The ability to use an output result-set column in the WHERE, GROUP BY,
                        // or HAVING clauses, or as part of a larger expression in the ORDER BY
                        // clause is not standard SQL.  This is a (goofy) SQLite extension, that
                        // is supported for backwards compatibility only. Hence, we issue a warning
                        // on sqlite3_log() whenever the capability is used.
                        if *__slate_slot_411 == (0 as i32)
                            && (unsafe { (*pNC).ncFlags }) & (128 as i32) != (0 as i32)
                            && zTab == std::ptr::null::<i8>()
                        {
                            *__slate_slot_423 = unsafe { (*pNC).uNC.pEList };
                            0 as i32;
                            *__slate_slot_410 = 0 as i32;
                            loop {
                                if *__slate_slot_410 < unsafe { (*(*__slate_slot_423)).nExpr } {
                                    std::ptr::write(__slate_slot_431, unsafe {
                                        (*unsafe {
                                            unsafe {
                                                std::ptr::addr_of_mut!((*(*__slate_slot_423)).a)
                                                    as *mut ExprList_item
                                            }
                                            .offset(*__slate_slot_410 as isize)
                                        })
                                        .zEName
                                    });
                                    if ((unsafe {
                                        (*unsafe {
                                            unsafe {
                                                std::ptr::addr_of_mut!((*(*__slate_slot_423)).a)
                                                    as *mut ExprList_item
                                            }
                                            .offset(*__slate_slot_410 as isize)
                                        })
                                        .fg
                                        .__slate_bits_0
                                        .__get_eEName()
                                    }) as i32)
                                        == (0 as i32)
                                    {
                                        *__slate_slot_931 = (unsafe {
                                            sqlite3_stricmp(
                                                *__slate_slot_431 as *const i8,
                                                *__slate_slot_422,
                                            )
                                        }) == (0 as i32);
                                    } else {
                                        *__slate_slot_931 = false as bool;
                                    }
                                    if *__slate_slot_931 {
                                        break '__join_55;
                                    } else {
                                        std::ptr::write(__slate_slot_929, *__slate_slot_410);
                                        std::ptr::write(
                                            __slate_slot_930,
                                            *__slate_slot_929 + (1 as i32),
                                        );
                                        *__slate_slot_410 = *__slate_slot_930;
                                    }
                                } else {
                                    break '__join_45;
                                }
                            }
                        }
                    }
                    // Advance to the next name context.  The loop will exit when either
                    // we have a match (cnt>0) or when we run out of name contexts.
                    if *__slate_slot_411 != (0 as i32) {
                        break;
                    } else {
                        pNC = unsafe { (*pNC).pNext };
                        std::ptr::write(__slate_slot_932, *__slate_slot_413);
                        std::ptr::write(__slate_slot_933, *__slate_slot_932 + (1 as i32));
                        *__slate_slot_413 = *__slate_slot_933;
                        if !(pNC != std::ptr::null_mut::<NameContext>()) {
                            break;
                        }
                    }
                }
                // If X and Y are NULL (in other words if only the column name Z is
                // supplied) and the value of Z is enclosed in double-quotes, then
                // Z is a string literal if it doesn't match any column names.  In that
                // case, we need to return right away and not make any changes to
                // pExpr.
                //
                // Because no reference was made to outer contexts, the pNC->nRef
                // fields are not changed in any context.
                if *__slate_slot_411 == (0 as i32) && zTab == std::ptr::null::<i8>() {
                    0 as i32;
                    if (unsafe { (*pExpr).flags }) & ((128 as i32) as u32) != ((0 as i32) as u32) {
                        *__slate_slot_934 =
                            areDoubleQuotedStringsEnabled(*__slate_slot_414, *__slate_slot_417)
                                != (0 as i32);
                    } else {
                        *__slate_slot_934 = false as bool;
                    }
                    if *__slate_slot_934 {
                        // If a double-quoted identifier does not match any known column name,
                        // then treat it as a string.
                        //
                        // This hack was added in the early days of SQLite in a misguided attempt
                        // to be compatible with MySQL 3.x, which used double-quotes for strings.
                        // I now sorely regret putting in this hack. The effect of this hack is
                        // that misspelled identifier names are silently converted into strings
                        // rather than causing an error, to the frustration of countless
                        // programmers. To all those frustrated programmers, my apologies.
                        //
                        // Someday, I hope to get rid of this hack. Unfortunately there is
                        // a huge amount of legacy SQL that uses it. So for now, we just
                        // issue a warning.
                        unsafe {
                            sqlite3_log(
                                28 as i32,
                                (b"double-quoted string literal: \"%w\"\0".as_ptr() as *mut i8)
                                    as *const i8,
                                *__slate_slot_422,
                            )
                        };
                        unsafe {
                            (*pExpr).op = ((118 as i32) as i8) as u8;
                        }
                        unsafe {
                            memset(
                                (unsafe { std::ptr::addr_of_mut!((*pExpr).y) }) as *mut (),
                                0 as i32,
                                8 as u64,
                            )
                        };
                        return 1 as i32;
                    } else {
                        if (unsafe { sqlite3ExprIdToTrueFalse(pExpr) }) != (0 as i32) {
                            return 1 as i32;
                        }
                    }
                }
                // cnt==0 means there was not match.
                // cnt>1 means there were two or more matches.
                //
                // cnt==0 is always an error.  cnt>1 is often an error, but might
                // be multiple matches for a NATURAL OUTER JOIN or a OUTER JOIN USING.
                0 as i32;
                0 as i32;
                if *__slate_slot_411 != (1 as i32) {
                    if *__slate_slot_421 != std::ptr::null_mut::<ExprList>() {
                        if (unsafe { (*(*__slate_slot_421)).nExpr })
                            == *__slate_slot_411 - (1 as i32)
                        {
                            if (unsafe { (*pExpr).flags }) & ((8388608 as i32) as u32)
                                != ((0 as i32) as u32)
                            {
                                std::ptr::write(__slate_slot_935, pExpr);
                                std::ptr::write(__slate_slot_936, unsafe {
                                    (*(*__slate_slot_935)).flags
                                });
                                std::ptr::write(
                                    __slate_slot_937,
                                    *__slate_slot_936 & !((8388608 as i32) as u32),
                                );
                                unsafe {
                                    (*(*__slate_slot_935)).flags = *__slate_slot_937;
                                }
                            } else {
                                unsafe {
                                    sqlite3ExprDelete(*__slate_slot_414, unsafe { (*pExpr).pLeft })
                                };
                                unsafe {
                                    (*pExpr).pLeft = std::ptr::null_mut::<Expr>();
                                }
                                unsafe {
                                    sqlite3ExprDelete(*__slate_slot_414, unsafe { (*pExpr).pRight })
                                };
                                unsafe {
                                    (*pExpr).pRight = std::ptr::null_mut::<Expr>();
                                }
                            }
                            extendFJMatch(
                                pParse,
                                std::ptr::addr_of_mut!(*__slate_slot_421),
                                *__slate_slot_416,
                                unsafe { (*pExpr).iColumn },
                            );
                            unsafe {
                                (*pExpr).op = ((172 as i32) as i8) as u8;
                            }
                            unsafe {
                                (*pExpr).u.zToken = b"coalesce\0".as_ptr() as *mut i8;
                            }
                            unsafe {
                                (*pExpr).x.pList = *__slate_slot_421;
                            }
                            unsafe {
                                (*pExpr).affExpr = (88 as i32) as i8;
                            }
                            *__slate_slot_411 = 1 as i32;
                            break '__join_12;
                        } else {
                            unsafe { sqlite3ExprListDelete(*__slate_slot_414, *__slate_slot_421) };
                            *__slate_slot_421 = std::ptr::null_mut::<ExprList>();
                        }
                    }
                    *__slate_slot_433 = (if *__slate_slot_411 == (0 as i32) {
                        b"no such column\0".as_ptr() as *mut i8
                    } else {
                        b"ambiguous column name\0".as_ptr() as *mut i8
                    }) as *const i8;
                    if zDb != std::ptr::null::<i8>() {
                        unsafe {
                            sqlite3ErrorMsg(
                                pParse,
                                (b"%s: %s.%s.%s\0".as_ptr() as *mut i8) as *const i8,
                                *__slate_slot_433,
                                zDb,
                                zTab,
                                *__slate_slot_422,
                            )
                        };
                    } else {
                        if zTab != std::ptr::null::<i8>() {
                            unsafe {
                                sqlite3ErrorMsg(
                                    pParse,
                                    (b"%s: %s.%s\0".as_ptr() as *mut i8) as *const i8,
                                    *__slate_slot_433,
                                    zTab,
                                    *__slate_slot_422,
                                )
                            };
                        } else {
                            if *__slate_slot_411 == (0 as i32)
                                && (unsafe { (*pRight).flags }) & ((128 as i32) as u32)
                                    != ((0 as i32) as u32)
                            {
                                unsafe {
                                    sqlite3ErrorMsg(pParse, (b"%s: \"%s\" - should this be a string literal in single-quotes?\0".as_ptr() as *mut i8) as *const i8, *__slate_slot_433, *__slate_slot_422)
                                };
                            } else {
                                unsafe {
                                    sqlite3ErrorMsg(
                                        pParse,
                                        (b"%s: %s\0".as_ptr() as *mut i8) as *const i8,
                                        *__slate_slot_433,
                                        *__slate_slot_422,
                                    )
                                };
                            }
                        }
                    }
                    unsafe {
                        sqlite3RecordErrorOffsetOfExpr(
                            unsafe { (*pParse).db },
                            pExpr as *const Expr,
                        )
                    };
                    unsafe {
                        (*pParse)
                            .__slate_bits_0
                            .__set_checkSchema((1 as i32) as u32);
                    }
                    std::ptr::write(__slate_slot_938, *__slate_slot_417);
                    std::ptr::write(__slate_slot_939, unsafe { (*(*__slate_slot_938)).nNcErr });
                    std::ptr::write(__slate_slot_940, *__slate_slot_939 + (1 as i32));
                    unsafe {
                        (*(*__slate_slot_938)).nNcErr = *__slate_slot_940;
                    }
                    *__slate_slot_419 = 122 as i32;
                }
                0 as i32;
                // Remove all substructure from pExpr
                if !((unsafe { (*pExpr).flags }) & (((65536 as i32) | (8388608 as i32)) as u32)
                    != ((0 as i32) as u32))
                {
                    unsafe { sqlite3ExprDelete(*__slate_slot_414, unsafe { (*pExpr).pLeft }) };
                    unsafe {
                        (*pExpr).pLeft = std::ptr::null_mut::<Expr>();
                    }
                    unsafe { sqlite3ExprDelete(*__slate_slot_414, unsafe { (*pExpr).pRight }) };
                    unsafe {
                        (*pExpr).pRight = std::ptr::null_mut::<Expr>();
                    }
                    std::ptr::write(__slate_slot_941, pExpr);
                    std::ptr::write(__slate_slot_942, unsafe { (*(*__slate_slot_941)).flags });
                    std::ptr::write(
                        __slate_slot_943,
                        *__slate_slot_942 | ((8388608 as i32) as u32),
                    );
                    unsafe {
                        (*(*__slate_slot_941)).flags = *__slate_slot_943;
                    }
                }
                // If a column from a table in pSrcList is referenced, then record
                // this fact in the pSrcList.a[].colUsed bitmask.  Column 0 causes
                // bit 0 to be set.  Column 1 sets bit 1.  And so forth.  Bit 63 is
                // set if the 63rd or any subsequent column is used.
                //
                // The colUsed mask is an optimization used to help determine if an
                // index is a covering index.  The correct answer is still obtained
                // if the mask contains extra set bits.  However, it is important to
                // avoid setting bits beyond the maximum column number of the table.
                // (See ticket [b92e5e8ec2cdbaa1]).
                //
                // If a generated column is referenced, set bits for every column
                // of the table.
                if *__slate_slot_416 != std::ptr::null_mut::<SrcItem>() {
                    if ((unsafe { (*pExpr).iColumn }) as i32) >= (0 as i32) {
                        std::ptr::write(__slate_slot_944, *__slate_slot_416);
                        std::ptr::write(__slate_slot_945, unsafe {
                            (*(*__slate_slot_944)).colUsed
                        });
                        std::ptr::write(
                            __slate_slot_946,
                            *__slate_slot_945 | sqlite3ExprColUsed(pExpr),
                        );
                        unsafe {
                            (*(*__slate_slot_944)).colUsed = *__slate_slot_946;
                        }
                    } else {
                        unsafe {
                            (*(*__slate_slot_416))
                                .fg
                                .__slate_bits_0
                                .__set_rowidUsed((1 as i32) as u32);
                        }
                    }
                }
                unsafe {
                    (*pExpr).op = (*__slate_slot_419 as i8) as u8;
                }
                break '__join_12;
            }
            0 as i32;
            0 as i32;
            0 as i32;
            *__slate_slot_432 = unsafe {
                (*unsafe {
                    unsafe {
                        std::ptr::addr_of_mut!((*(*__slate_slot_423)).a) as *mut ExprList_item
                    }
                    .offset(*__slate_slot_410 as isize)
                })
                .pExpr
            };
            if (unsafe { (*pNC).ncFlags }) & (1 as i32) == (0 as i32)
                && (unsafe { (*(*__slate_slot_432)).flags }) & ((16 as i32) as u32)
                    != ((0 as i32) as u32)
            {
                unsafe {
                    sqlite3ErrorMsg(
                        pParse,
                        (b"misuse of aliased aggregate %s\0".as_ptr() as *mut i8) as *const i8,
                        *__slate_slot_431,
                    )
                };
                return 2 as i32;
            } else {
                if (unsafe { (*(*__slate_slot_432)).flags }) & ((32768 as i32) as u32)
                    != ((0 as i32) as u32)
                    && ((unsafe { (*pNC).ncFlags }) & (16384 as i32) == (0 as i32)
                        || pNC != *__slate_slot_417)
                {
                    unsafe {
                        sqlite3ErrorMsg(
                            pParse,
                            (b"misuse of aliased window function %s\0".as_ptr() as *mut i8)
                                as *const i8,
                            *__slate_slot_431,
                        )
                    };
                    return 2 as i32;
                } else {
                    if (unsafe { sqlite3ExprVectorSize(*__slate_slot_432 as *const Expr) })
                        != (1 as i32)
                    {
                        unsafe {
                            sqlite3ErrorMsg(
                                pParse,
                                (b"row value misused\0".as_ptr() as *mut i8) as *const i8,
                            )
                        };
                        return 2 as i32;
                    } else {
                        resolveAlias(
                            pParse,
                            *__slate_slot_423,
                            *__slate_slot_410,
                            pExpr,
                            *__slate_slot_413,
                        );
                        *__slate_slot_411 = 1 as i32;
                        *__slate_slot_416 = std::ptr::null_mut::<SrcItem>();
                        0 as i32;
                        if (((unsafe { (*pParse).eParseMode }) as u32) as i32) >= (2 as i32) {
                            unsafe {
                                sqlite3RenameTokenRemap(
                                    pParse,
                                    std::ptr::null::<()>(),
                                    (pExpr as *mut ()) as *const (),
                                )
                            };
                        }
                    }
                }
            }
        }
        if *__slate_slot_411 == (1 as i32) {
            '__join_3: {
                0 as i32;
                if (unsafe { (*(*__slate_slot_414)).xAuth }) != None {
                    if *__slate_slot_421 != std::ptr::null_mut::<ExprList>() {
                        0 as i32;
                        0 as i32;
                        0 as i32;
                        0 as i32;
                        *__slate_slot_409 = 0 as i32;
                        loop {
                            if *__slate_slot_409 < unsafe { (*(*__slate_slot_421)).nExpr } {
                                0 as i32;
                                unsafe {
                                    sqlite3AuthRead(
                                        pParse,
                                        unsafe {
                                            (*unsafe {
                                                unsafe {
                                                    std::ptr::addr_of_mut!((*(*__slate_slot_421)).a)
                                                        as *mut ExprList_item
                                                }
                                                .offset(*__slate_slot_409 as isize)
                                            })
                                            .pExpr
                                        },
                                        *__slate_slot_418,
                                        unsafe { (*pNC).pSrcList },
                                    )
                                };
                                std::ptr::write(__slate_slot_947, *__slate_slot_409);
                                std::ptr::write(__slate_slot_948, *__slate_slot_947 + (1 as i32));
                                *__slate_slot_409 = *__slate_slot_948;
                            } else {
                                break '__join_3;
                            }
                        }
                    } else {
                        if (((unsafe { (*pExpr).op }) as u32) as i32) == (168 as i32)
                            || (((unsafe { (*pExpr).op }) as u32) as i32) == (78 as i32)
                        {
                            unsafe {
                                sqlite3AuthRead(pParse, pExpr, *__slate_slot_418, unsafe {
                                    (*pNC).pSrcList
                                })
                            };
                        }
                    }
                }
            }
            // Increment the nRef value on all name contexts from TopNC up to
            // the point where the name matched.
            loop {
                0 as i32;
                std::ptr::write(__slate_slot_949, *__slate_slot_417);
                std::ptr::write(__slate_slot_950, unsafe { (*(*__slate_slot_949)).nRef });
                std::ptr::write(__slate_slot_951, *__slate_slot_950 + (1 as i32));
                unsafe {
                    (*(*__slate_slot_949)).nRef = *__slate_slot_951;
                }
                if *__slate_slot_417 == pNC {
                    break;
                } else {
                    *__slate_slot_417 = unsafe { (*(*__slate_slot_417)).pNext };
                }
            }
            return 1 as i32;
        } else {
            return 2 as i32;
        }
    }
    return unsafe { std::mem::zeroed() };
}

/// Allocate and return a pointer to an expression to load the column iCol
/// from datasource iSrc in SrcList pSrc.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3CreateColumnExpr(
    mut db: *mut sqlite3,
    mut pSrc: *mut SrcList,
    mut iSrc: i32,
    mut iCol: i32,
) -> *mut Expr {
    let mut p: *mut Expr =
        unsafe { sqlite3ExprAlloc(db, 168 as i32, std::ptr::null::<Token>(), 0 as i32) };
    if p != std::ptr::null_mut::<Expr>() {
        let mut pItem: *mut SrcItem = unsafe {
            unsafe { std::ptr::addr_of_mut!((*pSrc).a) as *mut SrcItem }.offset(iSrc as isize)
        };
        let mut pTab: *mut Table = unsafe { std::mem::zeroed() };
        0 as i32;
        let __v879: *mut Table = unsafe { (*pItem).pSTab };
        unsafe {
            (*p).y.pTab = __v879;
        }
        pTab = __v879;
        unsafe {
            (*p).iTable = unsafe { (*pItem).iCursor };
        }
        if ((unsafe { (*unsafe { (*p).y.pTab }).iPKey }) as i32) == iCol {
            unsafe {
                (*p).iColumn = -(1 as i32) as i16;
            }
        } else {
            unsafe {
                (*p).iColumn = iCol as i16;
            }
            if (unsafe { (*pTab).tabFlags }) & ((96 as i32) as u32) != ((0 as i32) as u32)
                && (((unsafe {
                    (*unsafe { unsafe { (*pTab).aCol }.offset(iCol as isize) }).colFlags
                }) as u32) as i32)
                    & (96 as i32)
                    != (0 as i32)
            {
                {}
                {}
                unsafe {
                    (*pItem).colUsed = if ((unsafe { (*pTab).nCol }) as i32) >= (64 as i32) {
                        (-(1 as i32) as i64) as u64
                    } else {
                        ((((1 as i32) as i64) as u64) << ((unsafe { (*pTab).nCol }) as i32))
                            .wrapping_sub(((1 as i32) as i64) as u64)
                    };
                }
            } else {
                {}
                {}
                let __v880: *mut SrcItem = pItem;
                let __v881: u64 = unsafe { (*__v880).colUsed };
                let __v882: u64 = __v881
                    | (((1 as i32) as i64) as u64)
                        << if iCol
                            >= (((8 as u64).wrapping_mul(((8 as i32) as i64) as u64) as u32) as i32)
                        {
                            (((8 as u64).wrapping_mul(((8 as i32) as i64) as u64) as u32) as i32)
                                - (1 as i32)
                        } else {
                            iCol
                        };
                unsafe {
                    (*__v880).colUsed = __v882;
                }
            }
        }
    }
    return p;
}

/// Report an error that an expression is not valid for some set of
/// pNC->ncFlags values determined by validMask.
///
/// static void notValid(
///   Parse *pParse,       // Leave error message here
///   NameContext *pNC,    // The name context
///   const char *zMsg,    // Type of error
///   int validMask,       // Set of contexts for which prohibited
///   Expr *pExpr          // Invalidate this expression on error
/// ){...}
///
/// As an optimization, since the conditional is almost always false
/// (because errors are rare), the conditional is moved outside of the
/// function call using a macro.
///
/// # Arguments
///
/// * `pParse` - Leave error message here
/// * `pNC` - The name context
/// * `zMsg` - Type of error
/// * `pExpr` - Invalidate this expression on error
/// * `pError` - Associate error with this expression
fn notValidImpl(
    mut pParse: *mut Parse,
    mut pNC: *mut NameContext,
    mut zMsg: *const i8,
    mut pExpr: *mut Expr,
    mut pError: *mut Expr,
) {
    let mut zIn: *const i8 = (b"partial index WHERE clauses\0".as_ptr() as *mut i8) as *const i8;
    if (unsafe { (*pNC).ncFlags }) & (32 as i32) != (0 as i32) {
        zIn = (b"index expressions\0".as_ptr() as *mut i8) as *const i8;
    } else {
        if (unsafe { (*pNC).ncFlags }) & (4 as i32) != (0 as i32) {
            zIn = (b"CHECK constraints\0".as_ptr() as *mut i8) as *const i8;
        } else {
            if (unsafe { (*pNC).ncFlags }) & (8 as i32) != (0 as i32) {
                zIn = (b"generated columns\0".as_ptr() as *mut i8) as *const i8;
            }
        }
    }
    unsafe {
        sqlite3ErrorMsg(
            pParse,
            (b"%s prohibited in %s\0".as_ptr() as *mut i8) as *const i8,
            zMsg,
            zIn,
        )
    };
    if pExpr != std::ptr::null_mut::<Expr>() {
        unsafe {
            (*pExpr).op = ((122 as i32) as i8) as u8;
        }
    }
    unsafe { sqlite3RecordErrorOffsetOfExpr(unsafe { (*pParse).db }, pError as *const Expr) };
}

/// Expression p should encode a floating point value between 1.0 and 0.0.
/// Return 134,217,728 (2^27) times this value.  Or return -1 if p is not
/// a floating point value between 1.0 and 0.0.
fn exprProbability(mut p: *mut Expr) -> i32 {
    let mut r: f64 = -1.0f64;
    if (((unsafe { (*p).op }) as u32) as i32) != (154 as i32) {
        return -(1 as i32);
    }
    0 as i32;
    unsafe {
        sqlite3AtoF(
            (unsafe { (*p).u.zToken }) as *const i8,
            std::ptr::addr_of_mut!(r),
        )
    };
    0 as i32;
    if r > 1.0f64 {
        return -(1 as i32);
    }
    return (r * 134217728.0f64) as i32;
}

/// Set the EP_SubtArg property on every expression inside of
/// pList.  If any subexpression is actually a subquery, then
/// also set the EP_SubtArg property on the first result-set
/// column of that subquery.
fn resolveSetExprSubtypeArg(mut pList: *mut ExprList) {
    let mut nn: i32 = 0 as i32;
    let mut ii: i32 = 0 as i32;
    nn = if pList != std::ptr::null_mut::<ExprList>() {
        unsafe { (*pList).nExpr }
    } else {
        0 as i32
    };
    ii = 0 as i32;
    '__slate_break_775: loop {
        if !(ii < nn) {
            break;
        }
        let mut pExpr: *mut Expr = unsafe {
            (*unsafe {
                unsafe { std::ptr::addr_of_mut!((*pList).a) as *mut ExprList_item }
                    .offset(ii as isize)
            })
            .pExpr
        };
        '__slate_break_776: while (1 as i32) != (0 as i32) {
            // exit-by-break
            let __v954: *mut Expr = pExpr;
            let __v955: u32 = unsafe { (*__v954).flags };
            let __v956: u32 = __v955 | (2147483648 as u32);
            unsafe {
                (*__v954).flags = __v956;
            }
            if (((unsafe { (*pExpr).op }) as u32) as i32) == (139 as i32) {
                0 as i32;
                0 as i32;
                resolveSetExprSubtypeArg(unsafe { (*unsafe { (*pExpr).x.pSelect }).pEList });
                break '__slate_break_776;
            }
            if (((unsafe { (*pExpr).op }) as u32) as i32) == (173 as i32) {
                pExpr = unsafe { (*pExpr).pLeft };
                0 as i32;
            } else {
                break '__slate_break_776;
            }
        }
        let __v952: i32 = ii;
        let __v953: i32 = __v952 + (1 as i32);
        ii = __v953;
    }
}

/// This routine is callback for sqlite3WalkExpr().
///
/// Resolve symbolic names into TK_COLUMN operators for the current
/// node in the expression tree.  Return 0 to continue the search down
/// the tree or 2 to abort the tree walk.
///
/// This routine also does error checking and name resolution for
/// function names.  The operator for aggregate functions is changed
/// to TK_AGG_FUNCTION.
#[unsafe(link_section = ".text.slate_distinct.resolve.resolveExprStep")]
extern "C-unwind" fn resolveExprStep(mut pWalker: *mut Walker, mut pExpr: *mut Expr) -> i32 {
    let mut pNC: *mut NameContext = unsafe { std::mem::zeroed() };
    let mut pParse: *mut Parse = unsafe { std::mem::zeroed() };
    pNC = unsafe { (*pWalker).u.pNC };
    0 as i32;
    pParse = unsafe { (*pNC).pParse };
    0 as i32;
    // The special operator TK_ROW means use the rowid for the first
    // column in the FROM clause.  This is used by the LIMIT and ORDER BY
    // clause processing on UPDATE and DELETE statements, and by
    // UPDATE ... FROM statement processing.
    // An optimization:  Attempt to convert
    //
    //      "expr IS NOT NULL"  -->  "TRUE"
    //      "expr IS NULL"      -->  "FALSE"
    //
    // if we can prove that "expr" is never NULL.  Call this the
    // "NOT NULL strength reduction optimization".
    //
    // If this optimization occurs, also restore the NameContext ref-counts
    // to the state they where in before the "column" LHS expression was
    // resolved.  This prevents "column" from being counted as having been
    // referenced, which might prevent a SELECT from being erroneously
    // marked as correlated.
    //
    // 2024-03-28: Beware of aggregates.  A bare column of aggregated table
    // can still evaluate to NULL even though it is marked as NOT NULL.
    // Example:
    //
    //       CREATE TABLE t1(a INT NOT NULL);
    //       SELECT a, a IS NULL, a IS NOT NULL, count(*) FROM t1;
    //
    // The "a IS NULL" and "a IS NOT NULL" expressions cannot be optimized
    // here because at the time this case is hit, we do not yet know whether
    // or not t1 is being aggregated.  We have to assume the worst and omit
    // the optimization.  The only time it is safe to apply this optimization
    // is within the WHERE clause.
    // The expression can be NULL.  So the optimization does not apply
    // Not in a WHERE clause.  Unsafe to optimize.
    // A column name:                    ID
    // Or table name and column name:    ID.ID
    // Or a database, table and column:  ID.ID.ID
    //
    // The TK_ID and TK_OUT cases are combined so that there will only
    // be one call to lookupName().  Then the compiler will in-line
    // lookupName() for a size reduction and performance increase.
    // Resolve function names
    // The argument list
    // Number of arguments
    // True if no such function exists
    // True if wrong number of arguments
    // True if is an aggregate function
    // The function name.
    // Information about the function
    // The database encoding
    // EVIDENCE-OF: R-61304-29449 The unlikely(X) function is
    // equivalent to likelihood(X, 0.0625).
    // EVIDENCE-OF: R-01283-11636 The unlikely(X) function is
    // short-hand for likelihood(X,0.0625).
    // EVIDENCE-OF: R-36850-34127 The likely(X) function is short-hand
    // for likelihood(X,0.9375).
    // EVIDENCE-OF: R-53436-40973 The likely(X) function is equivalent
    // to likelihood(X,0.9375).
    //
    // TUNING: unlikely() probability is 0.0625.  likely() is 0.9375
    // If the function may call sqlite3_value_subtype(), then set the
    // EP_SubtArg flag on all of its argument expressions. This prevents
    // where.c from replacing the expression with a value read from an
    // index on the same expression, which will not have the correct
    // subtype. Also set the flag if the function expression itself is
    // an EP_SubtArg expression. In this case subtypes are required as
    // the function may return a value with a subtype back to its
    // caller using sqlite3_result_value().
    // For the purposes of the EP_ConstFunc flag, date and time
    // functions and other functions that change slowly are considered
    // constant because they are constant for the duration of one query.
    // This allows them to be factored out of inner loops.
    // Clearly non-deterministic functions like random(), but also
    // date/time functions that use 'now', and other functions like
    // sqlite_version() that might change over time cannot be used
    // in an index or generated column.  Curiously, they can be used
    // in a CHECK constraint.  SQLServer, MySQL, and PostgreSQL all
    // allow this.
    // Must fit in 8 bits
    // Internal-use-only functions are disallowed unless the
    // SQL is being compiled using sqlite3NestedParse() or
    // the SQLITE_TESTCTRL_INTERNAL_FUNCTIONS test-control has be
    // used to activate internal functions for testing purposes.
    //
    // The 2 value for no_such_func means that the function is
    // an internal-use-only function which should be treated as a
    // non-existant function for name resolution purposes.
    // Suppress "no such function" errors when reading
    // the sqlite_schema table.  Except, do raise the error
    // if init.busy is 2, meaning the schema parse is due
    // to an ALTER TABLE ADD COLUMN statement, and the function
    // is an internal-use-only function (no_such_func==2).
    // Window functions may not be arguments of aggregate functions.
    // Or arguments of other window functions. But aggregate functions
    // may be arguments for window functions.
    // For looping up thru outer contexts
    // FIX ME:  Compute pExpr->affinity based on the expected return
    // type of the function
    // Handle special cases of "x IS TRUE", "x IS FALSE", "x IS NOT TRUE",
    // and "x IS NOT FALSE".
    // no break
    '__slate_break_777: {
        match ((unsafe { (*pExpr).op }) as u32) as i32 {
            76 => {
                // The special operator TK_ROW means use the rowid for the first
                // column in the FROM clause.  This is used by the LIMIT and ORDER BY
                // clause processing on UPDATE and DELETE statements, and by
                // UPDATE ... FROM statement processing.
                let mut pSrcList: *mut SrcList = unsafe { (*pNC).pSrcList };
                let mut pItem: *mut SrcItem = unsafe { std::mem::zeroed() };
                0 as i32;
                pItem = unsafe { std::ptr::addr_of_mut!((*pSrcList).a) as *mut SrcItem };
                unsafe {
                    (*pExpr).op = ((168 as i32) as i8) as u8;
                }
                0 as i32;
                unsafe {
                    (*pExpr).y.pTab = unsafe { (*pItem).pSTab };
                }
                unsafe {
                    (*pExpr).iTable = unsafe { (*pItem).iCursor };
                }
                let __v957: *mut Expr = pExpr;
                let __v958: i16 = unsafe { (*__v957).iColumn };
                let __v959: i16 = ((__v958 as i32) - (1 as i32)) as i16;
                unsafe {
                    (*__v957).iColumn = __v959;
                }
                unsafe {
                    (*pExpr).affExpr = (68 as i32) as i8;
                }
                break '__slate_break_777;
                // An optimization:  Attempt to convert
                //
                //      "expr IS NOT NULL"  -->  "TRUE"
                //      "expr IS NULL"      -->  "FALSE"
                //
                // if we can prove that "expr" is never NULL.  Call this the
                // "NOT NULL strength reduction optimization".
                //
                // If this optimization occurs, also restore the NameContext ref-counts
                // to the state they where in before the "column" LHS expression was
                // resolved.  This prevents "column" from being counted as having been
                // referenced, which might prevent a SELECT from being erroneously
                // marked as correlated.
                //
                // 2024-03-28: Beware of aggregates.  A bare column of aggregated table
                // can still evaluate to NULL even though it is marked as NOT NULL.
                // Example:
                //
                //       CREATE TABLE t1(a INT NOT NULL);
                //       SELECT a, a IS NULL, a IS NOT NULL, count(*) FROM t1;
                //
                // The "a IS NULL" and "a IS NOT NULL" expressions cannot be optimized
                // here because at the time this case is hit, we do not yet know whether
                // or not t1 is being aggregated.  We have to assume the worst and omit
                // the optimization.  The only time it is safe to apply this optimization
                // is within the WHERE clause.
            }
            52 | 51 => {
                let mut anRef: __SlateAlign16<[i32; 8]> = __SlateAlign16([0 as i32; 8]);
                let mut p: *mut NameContext = unsafe { std::mem::zeroed() };
                let mut i: i32 = 0 as i32;
                i = 0 as i32;
                let __v960: *mut NameContext = pNC;
                p = __v960;
                '__slate_break_778: loop {
                    if !(p != std::ptr::null_mut::<NameContext>()
                        && i < ((((32 as u64) / (4 as u64)) as u32) as i32))
                    {
                        break;
                    }
                    unsafe {
                        *unsafe { (anRef.0.as_mut_ptr() as *mut i32).offset(i as isize) } =
                            unsafe { (*p).nRef };
                    }
                    p = unsafe { (*p).pNext };
                    let __v961: i32 = i;
                    let __v962: i32 = __v961 + (1 as i32);
                    i = __v962;
                }
                unsafe { sqlite3WalkExpr(pWalker, unsafe { (*pExpr).pLeft }) };
                if (((unsafe { (*pParse).eParseMode }) as u32) as i32) >= (2 as i32) {
                    return 1 as i32;
                }
                if (unsafe { sqlite3ExprCanBeNull((unsafe { (*pExpr).pLeft }) as *const Expr) })
                    != (0 as i32)
                {
                    // The expression can be NULL.  So the optimization does not apply
                    return 1 as i32;
                }
                i = 0 as i32;
                let __v963: *mut NameContext = pNC;
                p = __v963;
                '__slate_break_779: loop {
                    if !(p != std::ptr::null_mut::<NameContext>()) {
                        break;
                    }
                    if (unsafe { (*p).ncFlags }) & (1048576 as i32) == (0 as i32) {
                        return 1 as i32; // Not in a WHERE clause.  Unsafe to optimize.
                    }
                    p = unsafe { (*p).pNext };
                    let __v964: i32 = i;
                    let __v965: i32 = __v964 + (1 as i32);
                    i = __v965;
                }
                {}
                0 as i32;
                unsafe {
                    (*pExpr).u.iValue =
                        ((((unsafe { (*pExpr).op }) as u32) as i32) == (52 as i32)) as i32;
                }
                let __v966: *mut Expr = pExpr;
                let __v967: u32 = unsafe { (*__v966).flags };
                let __v968: u32 = __v967 | ((2048 as i32) as u32);
                unsafe {
                    (*__v966).flags = __v968;
                }
                unsafe {
                    (*pExpr).op = ((156 as i32) as i8) as u8;
                }
                i = 0 as i32;
                let __v969: *mut NameContext = pNC;
                p = __v969;
                '__slate_break_780: loop {
                    if !(p != std::ptr::null_mut::<NameContext>()
                        && i < ((((32 as u64) / (4 as u64)) as u32) as i32))
                    {
                        break;
                    }
                    unsafe {
                        (*p).nRef = unsafe {
                            *unsafe { (anRef.0.as_mut_ptr() as *mut i32).offset(i as isize) }
                        };
                    }
                    p = unsafe { (*p).pNext };
                    let __v970: i32 = i;
                    let __v971: i32 = __v970 + (1 as i32);
                    i = __v971;
                }
                unsafe { sqlite3ExprDelete(unsafe { (*pParse).db }, unsafe { (*pExpr).pLeft }) };
                unsafe {
                    (*pExpr).pLeft = std::ptr::null_mut::<Expr>();
                }
                return 1 as i32;
                // A column name:                    ID
                // Or table name and column name:    ID.ID
                // Or a database, table and column:  ID.ID.ID
                //
                // The TK_ID and TK_OUT cases are combined so that there will only
                // be one call to lookupName().  Then the compiler will in-line
                // lookupName() for a size reduction and performance increase.
            }
            60 | 142 => {
                let mut zTable: *const i8 = unsafe { std::mem::zeroed() };
                let mut zDb: *const i8 = unsafe { std::mem::zeroed() };
                let mut pRight: *mut Expr = unsafe { std::mem::zeroed() };
                if (((unsafe { (*pExpr).op }) as u32) as i32) == (60 as i32) {
                    zDb = std::ptr::null::<i8>();
                    zTable = std::ptr::null::<i8>();
                    0 as i32;
                    pRight = pExpr;
                } else {
                    let mut pLeft: *mut Expr = unsafe { (*pExpr).pLeft };
                    {}
                    {}
                    0 as i32;
                    if (unsafe { (*pNC).ncFlags }) & ((32 as i32) | (8 as i32)) != (0 as i32) {
                        notValidImpl(
                            pParse,
                            pNC,
                            (b"the \".\" operator\0".as_ptr() as *mut i8) as *const i8,
                            std::ptr::null_mut::<Expr>(),
                            pExpr,
                        );
                    }
                    {}
                    pRight = unsafe { (*pExpr).pRight };
                    if (((unsafe { (*pRight).op }) as u32) as i32) == (60 as i32) {
                        zDb = std::ptr::null::<i8>();
                    } else {
                        0 as i32;
                        0 as i32;
                        zDb = (unsafe { (*pLeft).u.zToken }) as *const i8;
                        pLeft = unsafe { (*pRight).pLeft };
                        pRight = unsafe { (*pRight).pRight };
                    }
                    0 as i32;
                    zTable = (unsafe { (*pLeft).u.zToken }) as *const i8;
                    0 as i32;
                    if (((unsafe { (*pParse).eParseMode }) as u32) as i32) >= (2 as i32) {
                        unsafe {
                            sqlite3RenameTokenRemap(
                                pParse,
                                (pExpr as *mut ()) as *const (),
                                (pRight as *mut ()) as *const (),
                            )
                        };
                        unsafe {
                            sqlite3RenameTokenRemap(
                                pParse,
                                ((unsafe { std::ptr::addr_of_mut!((*pExpr).y.pTab) }) as *mut ())
                                    as *const (),
                                (pLeft as *mut ()) as *const (),
                            )
                        };
                    }
                }
                return lookupName(pParse, zDb, zTable, pRight as *const Expr, pNC, pExpr);
                // Resolve function names
            }
            172 => {
                let mut pList: *mut ExprList = unsafe { std::mem::zeroed() }; // The argument list
                let mut n: i32 = 0 as i32; // Number of arguments
                let mut no_such_func: i32 = 0 as i32; // True if no such function exists
                let mut wrong_num_args: i32 = 0 as i32; // True if wrong number of arguments
                let mut is_agg: i32 = 0 as i32; // True if is an aggregate function
                let mut zId: *const i8 = unsafe { std::mem::zeroed() }; // The function name.
                let mut pDef: *mut FuncDef = unsafe { std::mem::zeroed() }; // Information about the function
                let mut enc: u8 = unsafe { (*unsafe { (*pParse).db }).enc }; // The database encoding
                let mut savedAllowFlags: i32 =
                    (unsafe { (*pNC).ncFlags }) & ((1 as i32) | (16384 as i32));
                let mut pWin: *mut Window = if (unsafe { (*pExpr).flags })
                    & ((16777216 as i32) as u32)
                    != ((0 as i32) as u32)
                    && (((unsafe { (*unsafe { (*pExpr).y.pWin }).eFrmType }) as u32) as i32)
                        != (167 as i32)
                {
                    unsafe { (*pExpr).y.pWin }
                } else {
                    std::ptr::null_mut::<Window>()
                };
                0 as i32;
                0 as i32;
                pList = unsafe { (*pExpr).x.pList };
                n = if pList != std::ptr::null_mut::<ExprList>() {
                    unsafe { (*pList).nExpr }
                } else {
                    0 as i32
                };
                zId = (unsafe { (*pExpr).u.zToken }) as *const i8;
                pDef = unsafe {
                    sqlite3FindFunction(
                        unsafe { (*pParse).db },
                        zId,
                        n,
                        enc,
                        ((0 as i32) as i8) as u8,
                    )
                };
                if pDef == std::ptr::null_mut::<FuncDef>() {
                    pDef = unsafe {
                        sqlite3FindFunction(
                            unsafe { (*pParse).db },
                            zId,
                            -(2 as i32),
                            enc,
                            ((0 as i32) as i8) as u8,
                        )
                    };
                    if pDef == std::ptr::null_mut::<FuncDef>() {
                        no_such_func = 1 as i32;
                    } else {
                        wrong_num_args = 1 as i32;
                    }
                } else {
                    is_agg = ((unsafe { (*pDef).xFinalize }) != None) as i32;
                    if (unsafe { (*pDef).funcFlags }) & ((1024 as i32) as u32) != (0 as u32) {
                        let __v972: *mut Expr = pExpr;
                        let __v973: u32 = unsafe { (*__v972).flags };
                        let __v974: u32 = __v973 | ((524288 as i32) as u32);
                        unsafe {
                            (*__v972).flags = __v974;
                        }
                        if n == (2 as i32) {
                            unsafe {
                                (*pExpr).iTable = exprProbability(unsafe {
                                    (*unsafe {
                                        unsafe {
                                            std::ptr::addr_of_mut!((*pList).a) as *mut ExprList_item
                                        }
                                        .offset((1 as i32) as isize)
                                    })
                                    .pExpr
                                });
                            }
                            if (unsafe { (*pExpr).iTable }) < (0 as i32) {
                                unsafe {
                                    sqlite3ErrorMsg(pParse, (b"second argument to %#T() must be a constant between 0.0 and 1.0\0".as_ptr() as *mut i8) as *const i8, pExpr)
                                };
                                let __v975: *mut NameContext = pNC;
                                let __v976: i32 = unsafe { (*__v975).nNcErr };
                                let __v977: i32 = __v976 + (1 as i32);
                                unsafe {
                                    (*__v975).nNcErr = __v977;
                                }
                            }
                        } else {
                            // EVIDENCE-OF: R-61304-29449 The unlikely(X) function is
                            // equivalent to likelihood(X, 0.0625).
                            // EVIDENCE-OF: R-01283-11636 The unlikely(X) function is
                            // short-hand for likelihood(X,0.0625).
                            // EVIDENCE-OF: R-36850-34127 The likely(X) function is short-hand
                            // for likelihood(X,0.9375).
                            // EVIDENCE-OF: R-53436-40973 The likely(X) function is equivalent
                            // to likelihood(X,0.9375).
                            //
                            // TUNING: unlikely() probability is 0.0625.  likely() is 0.9375
                            unsafe {
                                (*pExpr).iTable = if ((unsafe {
                                    *unsafe { unsafe { (*pDef).zName }.offset((0 as i32) as isize) }
                                }) as i32)
                                    == (117 as i32)
                                {
                                    8388608 as i32
                                } else {
                                    125829120 as i32
                                };
                            }
                        }
                    }
                    let mut auth: i32 = unsafe {
                        sqlite3AuthCheck(
                            pParse,
                            31 as i32,
                            std::ptr::null::<i8>(),
                            unsafe { (*pDef).zName },
                            std::ptr::null::<i8>(),
                        )
                    };
                    if auth != (0 as i32) {
                        if auth == (1 as i32) {
                            unsafe {
                                sqlite3ErrorMsg(
                                    pParse,
                                    (b"not authorized to use function: %#T\0".as_ptr() as *mut i8)
                                        as *const i8,
                                    pExpr,
                                )
                            };
                            let __v978: *mut NameContext = pNC;
                            let __v979: i32 = unsafe { (*__v978).nNcErr };
                            let __v980: i32 = __v979 + (1 as i32);
                            unsafe {
                                (*__v978).nNcErr = __v980;
                            }
                        }
                        unsafe {
                            (*pExpr).op = ((122 as i32) as i8) as u8;
                        }
                        return 1 as i32;
                    }
                    // If the function may call sqlite3_value_subtype(), then set the
                    // EP_SubtArg flag on all of its argument expressions. This prevents
                    // where.c from replacing the expression with a value read from an
                    // index on the same expression, which will not have the correct
                    // subtype. Also set the flag if the function expression itself is
                    // an EP_SubtArg expression. In this case subtypes are required as
                    // the function may return a value with a subtype back to its
                    // caller using sqlite3_result_value().
                    if (unsafe { (*pDef).funcFlags }) & ((1048576 as i32) as u32) != (0 as u32)
                        || (unsafe { (*pExpr).flags }) & (2147483648 as u32) != ((0 as i32) as u32)
                    {
                        resolveSetExprSubtypeArg(pList);
                    }
                    if (unsafe { (*pDef).funcFlags }) & (((2048 as i32) | (8192 as i32)) as u32)
                        != (0 as u32)
                    {
                        // For the purposes of the EP_ConstFunc flag, date and time
                        // functions and other functions that change slowly are considered
                        // constant because they are constant for the duration of one query.
                        // This allows them to be factored out of inner loops.
                        let __v981: *mut Expr = pExpr;
                        let __v982: u32 = unsafe { (*__v981).flags };
                        let __v983: u32 = __v982 | ((1048576 as i32) as u32);
                        unsafe {
                            (*__v981).flags = __v983;
                        }
                    }
                    if (unsafe { (*pDef).funcFlags }) & ((2048 as i32) as u32)
                        == ((0 as i32) as u32)
                    {
                        // Clearly non-deterministic functions like random(), but also
                        // date/time functions that use 'now', and other functions like
                        // sqlite_version() that might change over time cannot be used
                        // in an index or generated column.  Curiously, they can be used
                        // in a CHECK constraint.  SQLServer, MySQL, and PostgreSQL all
                        // allow this.
                        0 as i32;
                        if (unsafe { (*pNC).ncFlags }) & ((32 as i32) | (2 as i32) | (8 as i32))
                            != (0 as i32)
                        {
                            notValidImpl(
                                pParse,
                                pNC,
                                (b"non-deterministic functions\0".as_ptr() as *mut i8) as *const i8,
                                std::ptr::null_mut::<Expr>(),
                                pExpr,
                            );
                        }
                        {}
                    } else {
                        0 as i32; // Must fit in 8 bits
                        unsafe {
                            (*pExpr).op2 =
                                (((unsafe { (*pNC).ncFlags }) & (46 as i32)) as i8) as u8;
                        }
                    }
                    if (unsafe { (*pDef).funcFlags }) & ((262144 as i32) as u32)
                        != ((0 as i32) as u32)
                        && (((unsafe { (*pParse).nested }) as u32) as i32) == (0 as i32)
                        && (unsafe { (*unsafe { (*pParse).db }).mDbFlags }) & ((32 as i32) as u32)
                            == ((0 as i32) as u32)
                    {
                        // Internal-use-only functions are disallowed unless the
                        // SQL is being compiled using sqlite3NestedParse() or
                        // the SQLITE_TESTCTRL_INTERNAL_FUNCTIONS test-control has be
                        // used to activate internal functions for testing purposes.
                        //
                        // The 2 value for no_such_func means that the function is
                        // an internal-use-only function which should be treated as a
                        // non-existant function for name resolution purposes.
                        no_such_func = 2 as i32;
                        pDef = std::ptr::null_mut::<FuncDef>();
                    } else {
                        if (unsafe { (*pDef).funcFlags })
                            & (((524288 as i32) | (2097152 as i32)) as u32)
                            != ((0 as i32) as u32)
                            && !((((unsafe { (*pParse).eParseMode }) as u32) as i32) >= (2 as i32))
                        {
                            if (unsafe { (*pNC).ncFlags }) & (262144 as i32) != (0 as i32) {
                                let __v984: *mut Expr = pExpr;
                                let __v985: u32 = unsafe { (*__v984).flags };
                                let __v986: u32 = __v985 | ((1073741824 as i32) as u32);
                                unsafe {
                                    (*__v984).flags = __v986;
                                }
                            }
                            unsafe {
                                sqlite3ExprFunctionUsable(
                                    pParse,
                                    pExpr as *const Expr,
                                    pDef as *const FuncDef,
                                )
                            };
                        }
                    }
                }
                if (0 as i32)
                    == (((((unsafe { (*pParse).eParseMode }) as u32) as i32) >= (2 as i32)) as i32)
                {
                    0 as i32;
                    if pDef != std::ptr::null_mut::<FuncDef>()
                        && (unsafe { (*pDef).xValue }) == None
                        && pWin != std::ptr::null_mut::<Window>()
                    {
                        unsafe {
                            sqlite3ErrorMsg(
                                pParse,
                                (b"%#T() may not be used as a window function\0".as_ptr()
                                    as *mut i8) as *const i8,
                                pExpr,
                            )
                        };
                        let __v987: *mut NameContext = pNC;
                        let __v988: i32 = unsafe { (*__v987).nNcErr };
                        let __v989: i32 = __v988 + (1 as i32);
                        unsafe {
                            (*__v987).nNcErr = __v989;
                        }
                    } else {
                        if is_agg != (0 as i32)
                            && (unsafe { (*pNC).ncFlags }) & (1 as i32) == (0 as i32)
                            || is_agg != (0 as i32)
                                && (unsafe { (*pDef).funcFlags }) & ((65536 as i32) as u32)
                                    != (0 as u32)
                                && !(pWin != std::ptr::null_mut::<Window>())
                            || is_agg != (0 as i32)
                                && pWin != std::ptr::null_mut::<Window>()
                                && (unsafe { (*pNC).ncFlags }) & (16384 as i32) == (0 as i32)
                        {
                            let mut zType: *const i8 = unsafe { std::mem::zeroed() };
                            if (unsafe { (*pDef).funcFlags }) & ((65536 as i32) as u32)
                                != (0 as u32)
                                || pWin != std::ptr::null_mut::<Window>()
                            {
                                zType = (b"window\0".as_ptr() as *mut i8) as *const i8;
                            } else {
                                zType = (b"aggregate\0".as_ptr() as *mut i8) as *const i8;
                            }
                            unsafe {
                                sqlite3ErrorMsg(
                                    pParse,
                                    (b"misuse of %s function %#T()\0".as_ptr() as *mut i8)
                                        as *const i8,
                                    zType,
                                    pExpr,
                                )
                            };
                            let __v990: *mut NameContext = pNC;
                            let __v991: i32 = unsafe { (*__v990).nNcErr };
                            let __v992: i32 = __v991 + (1 as i32);
                            unsafe {
                                (*__v990).nNcErr = __v992;
                            }
                            is_agg = 0 as i32;
                        } else {
                            if no_such_func != (0 as i32)
                                && ((((unsafe { (*unsafe { (*pParse).db }).init.busy }) as u32)
                                    as i32)
                                    == (0 as i32)
                                    || no_such_func == (2 as i32)
                                        && (((unsafe { (*unsafe { (*pParse).db }).init.busy })
                                            as u32)
                                            as i32)
                                            == (2 as i32))
                                && (((unsafe { (*pParse).explain }) as u32) as i32) == (0 as i32)
                            {
                                unsafe {
                                    sqlite3ErrorMsg(
                                        pParse,
                                        (b"no such function: %#T\0".as_ptr() as *mut i8)
                                            as *const i8,
                                        pExpr,
                                    )
                                };
                                let __v993: *mut NameContext = pNC;
                                let __v994: i32 = unsafe { (*__v993).nNcErr };
                                let __v995: i32 = __v994 + (1 as i32);
                                unsafe {
                                    (*__v993).nNcErr = __v995;
                                }
                            } else {
                                if wrong_num_args != (0 as i32) {
                                    unsafe {
                                        sqlite3ErrorMsg(
                                            pParse,
                                            (b"wrong number of arguments to function %#T()\0"
                                                .as_ptr()
                                                as *mut i8)
                                                as *const i8,
                                            pExpr,
                                        )
                                    };
                                    let __v996: *mut NameContext = pNC;
                                    let __v997: i32 = unsafe { (*__v996).nNcErr };
                                    let __v998: i32 = __v997 + (1 as i32);
                                    unsafe {
                                        (*__v996).nNcErr = __v998;
                                    }
                                } else {
                                    if is_agg == (0 as i32)
                                        && (unsafe { (*pExpr).flags }) & ((16777216 as i32) as u32)
                                            != ((0 as i32) as u32)
                                    {
                                        unsafe {
                                            sqlite3ErrorMsg(pParse, (b"FILTER may not be used with non-aggregate %#T()\0".as_ptr() as *mut i8) as *const i8, pExpr)
                                        };
                                        let __v999: *mut NameContext = pNC;
                                        let __v1000: i32 = unsafe { (*__v999).nNcErr };
                                        let __v1001: i32 = __v1000 + (1 as i32);
                                        unsafe {
                                            (*__v999).nNcErr = __v1001;
                                        }
                                    } else {
                                        if is_agg == (0 as i32)
                                            && (unsafe { (*pExpr).pLeft })
                                                != std::ptr::null_mut::<Expr>()
                                        {
                                            unsafe {
                                                sqlite3ExprOrderByAggregateError(pParse, pExpr)
                                            };
                                            let __v1002: *mut NameContext = pNC;
                                            let __v1003: i32 = unsafe { (*__v1002).nNcErr };
                                            let __v1004: i32 = __v1003 + (1 as i32);
                                            unsafe {
                                                (*__v1002).nNcErr = __v1004;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    // Suppress "no such function" errors when reading
                    // the sqlite_schema table.  Except, do raise the error
                    // if init.busy is 2, meaning the schema parse is due
                    // to an ALTER TABLE ADD COLUMN statement, and the function
                    // is an internal-use-only function (no_such_func==2).
                    if is_agg != (0 as i32) {
                        // Window functions may not be arguments of aggregate functions.
                        // Or arguments of other window functions. But aggregate functions
                        // may be arguments for window functions.
                        let __v1005: *mut NameContext = pNC;
                        let __v1006: i32 = unsafe { (*__v1005).ncFlags };
                        let __v1007: i32 = __v1006
                            & !((16384 as i32)
                                | if !(pWin != std::ptr::null_mut::<Window>()) {
                                    1 as i32
                                } else {
                                    0 as i32
                                });
                        unsafe {
                            (*__v1005).ncFlags = __v1007;
                        }
                    }
                } else {
                    if (unsafe { (*pExpr).flags }) & ((16777216 as i32) as u32)
                        != ((0 as i32) as u32)
                        || (unsafe { (*pExpr).pLeft }) != std::ptr::null_mut::<Expr>()
                    {
                        is_agg = 1 as i32;
                    }
                }
                unsafe { sqlite3WalkExprList(pWalker, pList) };
                if is_agg != (0 as i32) {
                    if (unsafe { (*pExpr).pLeft }) != std::ptr::null_mut::<Expr>() {
                        0 as i32;
                        0 as i32;
                        unsafe {
                            sqlite3WalkExprList(pWalker, unsafe {
                                (*unsafe { (*pExpr).pLeft }).x.pList
                            })
                        };
                    }
                    if pWin != std::ptr::null_mut::<Window>()
                        && (unsafe { (*pParse).nErr }) == (0 as i32)
                    {
                        let mut pSel: *mut Select = unsafe { (*pNC).pWinSelect };
                        0 as i32;
                        if (((((unsafe { (*pParse).eParseMode }) as u32) as i32) >= (2 as i32))
                            as i32)
                            == (0 as i32)
                        {
                            unsafe {
                                sqlite3WindowUpdate(
                                    pParse,
                                    if pSel != std::ptr::null_mut::<Select>() {
                                        unsafe { (*pSel).pWinDefn }
                                    } else {
                                        std::ptr::null_mut::<Window>()
                                    },
                                    pWin,
                                    pDef,
                                )
                            };
                            if (unsafe { (*unsafe { (*pParse).db }).mallocFailed }) != (0 as u8) {
                                break '__slate_break_777;
                            }
                        }
                        unsafe { sqlite3WalkExprList(pWalker, unsafe { (*pWin).pPartition }) };
                        unsafe { sqlite3WalkExprList(pWalker, unsafe { (*pWin).pOrderBy }) };
                        unsafe { sqlite3WalkExpr(pWalker, unsafe { (*pWin).pFilter }) };
                        unsafe { sqlite3WindowLink(pSel, pWin) };
                        let __v1008: *mut NameContext = pNC;
                        let __v1009: i32 = unsafe { (*__v1008).ncFlags };
                        let __v1010: i32 = __v1009 | (32768 as i32);
                        unsafe {
                            (*__v1008).ncFlags = __v1010;
                        }
                    } else {
                        let mut pNC2: *mut NameContext = unsafe { std::mem::zeroed() }; // For looping up thru outer contexts
                        unsafe {
                            (*pExpr).op = ((169 as i32) as i8) as u8;
                        }
                        unsafe {
                            (*pExpr).op2 = ((0 as i32) as i8) as u8;
                        }
                        if (unsafe { (*pExpr).flags }) & ((16777216 as i32) as u32)
                            != ((0 as i32) as u32)
                        {
                            unsafe {
                                sqlite3WalkExpr(pWalker, unsafe {
                                    (*unsafe { (*pExpr).y.pWin }).pFilter
                                })
                            };
                        }
                        pNC2 = pNC;
                        '__slate_break_792: loop {
                            let __v1011: bool;
                            if pNC2 != std::ptr::null_mut::<NameContext>() {
                                __v1011 = (unsafe {
                                    sqlite3ReferencesSrcList(pParse, pExpr, unsafe {
                                        (*pNC2).pSrcList
                                    })
                                }) == (0 as i32);
                            } else {
                                __v1011 = false as bool;
                            }
                            if !__v1011 {
                                break;
                            }
                            let __v1012: *mut Expr = pExpr;
                            let __v1013: u8 = unsafe { (*__v1012).op2 };
                            let __v1014: u8 = (((__v1013 as u32) as i32) as u32).wrapping_add(
                                ((1 as i32) as u32).wrapping_add(unsafe { (*pNC2).nNestedSelect }),
                            ) as u8;
                            unsafe {
                                (*__v1012).op2 = __v1014;
                            }
                            pNC2 = unsafe { (*pNC2).pNext };
                        }
                        0 as i32;
                        if pNC2 != std::ptr::null_mut::<NameContext>()
                            && pDef != std::ptr::null_mut::<FuncDef>()
                        {
                            let __v1015: *mut Expr = pExpr;
                            let __v1016: u8 = unsafe { (*__v1015).op2 };
                            let __v1017: u8 = (((__v1016 as u32) as i32) as u32)
                                .wrapping_add(unsafe { (*pNC2).nNestedSelect })
                                as u8;
                            unsafe {
                                (*__v1015).op2 = __v1017;
                            }
                            0 as i32;
                            0 as i32;
                            {}
                            {}
                            let __v1018: *mut NameContext = pNC2;
                            let __v1019: i32 = unsafe { (*__v1018).ncFlags };
                            let __v1020: i32 = ((__v1019 as u32)
                                | (((16 as i32) as u32)
                                    | ((unsafe { (*pDef).funcFlags })
                                        ^ ((134217728 as i32) as u32))
                                        & (((4096 as i32) | (134217728 as i32)) as u32)))
                                as i32;
                            unsafe {
                                (*__v1018).ncFlags = __v1020;
                            }
                        }
                    }
                    let __v1021: *mut NameContext = pNC;
                    let __v1022: i32 = unsafe { (*__v1021).ncFlags };
                    let __v1023: i32 = __v1022 | savedAllowFlags;
                    unsafe {
                        (*__v1021).ncFlags = __v1023;
                    }
                }
                // FIX ME:  Compute pExpr->affinity based on the expected return
                // type of the function
                return 1 as i32;
            }
            20 | 139 | 50 => {
                {}
                {}
                {}
                if (unsafe { (*pExpr).flags }) & ((4096 as i32) as u32) != ((0 as i32) as u32) {
                    let mut nRef: i32 = unsafe { (*pNC).nRef };
                    {}
                    {}
                    {}
                    {}
                    0 as i32;
                    if (((unsafe { (*pExpr).op }) as u32) as i32) == (20 as i32) {
                        unsafe {
                            (*pParse).__slate_bits_0.__set_bHasExists((1 as i32) as u32);
                        }
                    }
                    if (unsafe { (*pNC).ncFlags }) & (46 as i32) != (0 as i32) {
                        notValidImpl(
                            pParse,
                            pNC,
                            (b"subqueries\0".as_ptr() as *mut i8) as *const i8,
                            pExpr,
                            pExpr,
                        );
                    } else {
                        unsafe { sqlite3WalkSelect(pWalker, unsafe { (*pExpr).x.pSelect }) };
                    }
                    0 as i32;
                    if nRef != unsafe { (*pNC).nRef } {
                        let __v1024: *mut Expr = pExpr;
                        let __v1025: u32 = unsafe { (*__v1024).flags };
                        let __v1026: u32 = __v1025 | ((64 as i32) as u32);
                        unsafe {
                            (*__v1024).flags = __v1026;
                        }
                        let __v1027: *mut Select = unsafe { (*pExpr).x.pSelect };
                        let __v1028: u32 = unsafe { (*__v1027).selFlags };
                        let __v1029: u32 = __v1028 | ((536870912 as i32) as u32);
                        unsafe {
                            (*__v1027).selFlags = __v1029;
                        }
                    }
                    let __v1030: *mut NameContext = pNC;
                    let __v1031: i32 = unsafe { (*__v1030).ncFlags };
                    let __v1032: i32 = __v1031 | (64 as i32);
                    unsafe {
                        (*__v1030).ncFlags = __v1032;
                    }
                }
            }
            157 => {
                {}
                {}
                {}
                {}
                0 as i32;
                if (unsafe { (*pNC).ncFlags })
                    & ((4 as i32) | (2 as i32) | (32 as i32) | (8 as i32))
                    != (0 as i32)
                {
                    notValidImpl(
                        pParse,
                        pNC,
                        (b"parameters\0".as_ptr() as *mut i8) as *const i8,
                        pExpr,
                        pExpr,
                    );
                }
                {}
            }
            45 | 46 => {
                let mut pRight: *mut Expr =
                    unsafe { sqlite3ExprSkipCollateAndLikely(unsafe { (*pExpr).pRight }) };
                0 as i32;
                // Handle special cases of "x IS TRUE", "x IS FALSE", "x IS NOT TRUE",
                // and "x IS NOT FALSE".
                if pRight != std::ptr::null_mut::<Expr>()
                    && ((((unsafe { (*pRight).op }) as u32) as i32) == (60 as i32)
                        || (((unsafe { (*pRight).op }) as u32) as i32) == (171 as i32))
                {
                    let mut rc: i32 = resolveExprStep(pWalker, pRight);
                    if rc == (2 as i32) {
                        return 2 as i32;
                    }
                    if (((unsafe { (*pRight).op }) as u32) as i32) == (171 as i32) {
                        unsafe {
                            (*pExpr).op2 = unsafe { (*pExpr).op };
                        }
                        unsafe {
                            (*pExpr).op = ((175 as i32) as i8) as u8;
                        }
                        return 0 as i32;
                    }
                }
                // no break
                {}
                let mut nLeft: i32 = 0 as i32;
                let mut nRight: i32 = 0 as i32;
                if (unsafe { (*unsafe { (*pParse).db }).mallocFailed }) != (0 as u8) {
                } else {
                    0 as i32;
                    nLeft = unsafe {
                        sqlite3ExprVectorSize((unsafe { (*pExpr).pLeft }) as *const Expr)
                    };
                    if (((unsafe { (*pExpr).op }) as u32) as i32) == (49 as i32) {
                        0 as i32;
                        nRight = unsafe {
                            sqlite3ExprVectorSize(
                                (unsafe {
                                    (*unsafe {
                                        unsafe {
                                            std::ptr::addr_of_mut!((*unsafe { (*pExpr).x.pList }).a)
                                                as *mut ExprList_item
                                        }
                                        .offset((0 as i32) as isize)
                                    })
                                    .pExpr
                                }) as *const Expr,
                            )
                        };
                        if nRight == nLeft {
                            nRight = unsafe {
                                sqlite3ExprVectorSize(
                                    (unsafe {
                                        (*unsafe {
                                            unsafe {
                                                std::ptr::addr_of_mut!(
                                                    (*unsafe { (*pExpr).x.pList }).a
                                                )
                                                    as *mut ExprList_item
                                            }
                                            .offset((1 as i32) as isize)
                                        })
                                        .pExpr
                                    }) as *const Expr,
                                )
                            };
                        }
                    } else {
                        0 as i32;
                        nRight = unsafe {
                            sqlite3ExprVectorSize((unsafe { (*pExpr).pRight }) as *const Expr)
                        };
                    }
                    if nLeft != nRight {
                        {}
                        {}
                        {}
                        {}
                        {}
                        {}
                        {}
                        {}
                        {}
                        unsafe {
                            sqlite3ErrorMsg(
                                pParse,
                                (b"row value misused\0".as_ptr() as *mut i8) as *const i8,
                            )
                        };
                        unsafe {
                            sqlite3RecordErrorOffsetOfExpr(
                                unsafe { (*pParse).db },
                                pExpr as *const Expr,
                            )
                        };
                    }
                }
            }
            49 | 54 | 53 | 57 | 56 | 55 | 58 => {
                let mut nLeft: i32 = 0 as i32;
                let mut nRight: i32 = 0 as i32;
                if (unsafe { (*unsafe { (*pParse).db }).mallocFailed }) != (0 as u8) {
                } else {
                    0 as i32;
                    nLeft = unsafe {
                        sqlite3ExprVectorSize((unsafe { (*pExpr).pLeft }) as *const Expr)
                    };
                    if (((unsafe { (*pExpr).op }) as u32) as i32) == (49 as i32) {
                        0 as i32;
                        nRight = unsafe {
                            sqlite3ExprVectorSize(
                                (unsafe {
                                    (*unsafe {
                                        unsafe {
                                            std::ptr::addr_of_mut!((*unsafe { (*pExpr).x.pList }).a)
                                                as *mut ExprList_item
                                        }
                                        .offset((0 as i32) as isize)
                                    })
                                    .pExpr
                                }) as *const Expr,
                            )
                        };
                        if nRight == nLeft {
                            nRight = unsafe {
                                sqlite3ExprVectorSize(
                                    (unsafe {
                                        (*unsafe {
                                            unsafe {
                                                std::ptr::addr_of_mut!(
                                                    (*unsafe { (*pExpr).x.pList }).a
                                                )
                                                    as *mut ExprList_item
                                            }
                                            .offset((1 as i32) as isize)
                                        })
                                        .pExpr
                                    }) as *const Expr,
                                )
                            };
                        }
                    } else {
                        0 as i32;
                        nRight = unsafe {
                            sqlite3ExprVectorSize((unsafe { (*pExpr).pRight }) as *const Expr)
                        };
                    }
                    if nLeft != nRight {
                        {}
                        {}
                        {}
                        {}
                        {}
                        {}
                        {}
                        {}
                        {}
                        unsafe {
                            sqlite3ErrorMsg(
                                pParse,
                                (b"row value misused\0".as_ptr() as *mut i8) as *const i8,
                            )
                        };
                        unsafe {
                            sqlite3RecordErrorOffsetOfExpr(
                                unsafe { (*pParse).db },
                                pExpr as *const Expr,
                            )
                        };
                    }
                }
            }
            _ => {}
        }
    }
    0 as i32;
    return if (unsafe { (*pParse).nErr }) != (0 as i32) {
        2 as i32
    } else {
        0 as i32
    };
}

/// pEList is a list of expressions which are really the result set of the
/// a SELECT statement.  pE is a term in an ORDER BY or GROUP BY clause.
/// This routine checks to see if pE is a simple identifier which corresponds
/// to the AS-name of one of the terms of the expression list.  If it is,
/// this routine return an integer between 1 and N where N is the number of
/// elements in pEList, corresponding to the matching entry.  If there is
/// no match, or if pE is not a simple identifier, then this routine
/// return 0.
///
/// pEList has been resolved.  pE has not.
///
/// # Arguments
///
/// * `pParse` - Parsing context for error messages
/// * `pEList` - List of expressions to scan
/// * `pE` - Expression we are trying to match
fn resolveAsName(mut pParse: *mut Parse, mut pEList: *mut ExprList, mut pE: *mut Expr) -> i32 {
    let mut i: i32 = 0 as i32; // Loop counter
    pParse;
    if (((unsafe { (*pE).op }) as u32) as i32) == (60 as i32) {
        let mut zCol: *const i8 = unsafe { std::mem::zeroed() };
        0 as i32;
        zCol = (unsafe { (*pE).u.zToken }) as *const i8;
        i = 0 as i32;
        '__slate_break_796: loop {
            if !(i < unsafe { (*pEList).nExpr }) {
                break;
            }
            let __v1035: bool;
            if ((unsafe {
                (*unsafe {
                    unsafe { std::ptr::addr_of_mut!((*pEList).a) as *mut ExprList_item }
                        .offset(i as isize)
                })
                .fg
                .__slate_bits_0
                .__get_eEName()
            }) as i32)
                == (0 as i32)
            {
                __v1035 = (unsafe {
                    sqlite3_stricmp(
                        (unsafe {
                            (*unsafe {
                                unsafe { std::ptr::addr_of_mut!((*pEList).a) as *mut ExprList_item }
                                    .offset(i as isize)
                            })
                            .zEName
                        }) as *const i8,
                        zCol,
                    )
                }) == (0 as i32);
            } else {
                __v1035 = false as bool;
            }
            if __v1035 {
                return i + (1 as i32);
            }
            let __v1033: i32 = i;
            let __v1034: i32 = __v1033 + (1 as i32);
            i = __v1034;
        }
    }
    return 0 as i32;
}

/// pE is a pointer to an expression which is a single term in the
/// ORDER BY of a compound SELECT.  The expression has not been
/// name resolved.
///
/// At the point this routine is called, we already know that the
/// ORDER BY term is not an integer index into the result set.  That
/// case is handled by the calling routine.
///
/// Attempt to match pE against result set columns in the left-most
/// SELECT statement.  Return the index i of the matching column,
/// as an indication to the caller that it should sort by the i-th column.
/// The left-most column is 1.  In other words, the value returned is the
/// same integer value that would be used in the SQL statement to indicate
/// the column.
///
/// If there is no match, return 0.  Return -1 if an error occurs.
///
/// # Arguments
///
/// * `pParse` - Parsing context for error messages
/// * `pSelect` - The SELECT statement with the ORDER BY clause
/// * `pE` - The specific ORDER BY term
fn resolveOrderByTermToExprList(
    mut pParse: *mut Parse,
    mut pSelect: *mut Select,
    mut pE: *mut Expr,
) -> i32 {
    let mut i: i32 = 0 as i32; // Loop counter
    let mut pEList: *mut ExprList = unsafe { std::mem::zeroed() }; // The columns of the result set
    let mut nc: NameContext = unsafe { std::mem::zeroed() }; // Name context for resolving pE
    let mut db: *mut sqlite3 = unsafe { std::mem::zeroed() }; // Database connection
    let mut rc: i32 = 0 as i32; // Return code from subprocedures
    let mut savedSuppErr: u8 = 0 as u8; // Saved value of db->suppressErr
    0 as i32;
    pEList = unsafe { (*pSelect).pEList };
    // Resolve all names in the ORDER BY term expression
    unsafe { memset(std::ptr::addr_of_mut!(nc) as *mut (), 0 as i32, 56 as u64) };
    nc.pParse = pParse;
    nc.pSrcList = unsafe { (*pSelect).pSrc };
    unsafe {
        nc.uNC.pEList = pEList;
    }
    nc.ncFlags = (1 as i32) | (128 as i32) | (524288 as i32);
    nc.nNcErr = 0 as i32;
    db = unsafe { (*pParse).db };
    savedSuppErr = unsafe { (*db).suppressErr };
    unsafe {
        (*db).suppressErr = ((1 as i32) as i8) as u8;
    }
    rc = sqlite3ResolveExprNames(std::ptr::addr_of_mut!(nc), pE);
    unsafe {
        (*db).suppressErr = savedSuppErr;
    }
    if rc != (0 as i32) {
        return 0 as i32;
    }
    // Try to match the ORDER BY expression against an expression
    // in the result set.  Return an 1-based index of the matching
    // result-set entry.
    i = 0 as i32;
    '__slate_break_797: loop {
        if !(i < unsafe { (*pEList).nExpr }) {
            break;
        }
        if (unsafe {
            sqlite3ExprCompare(
                std::ptr::null::<Parse>(),
                (unsafe {
                    (*unsafe {
                        unsafe { std::ptr::addr_of_mut!((*pEList).a) as *mut ExprList_item }
                            .offset(i as isize)
                    })
                    .pExpr
                }) as *const Expr,
                pE as *const Expr,
                -(1 as i32),
            )
        }) < (2 as i32)
        {
            return i + (1 as i32);
        }
        let __v1036: i32 = i;
        let __v1037: i32 = __v1036 + (1 as i32);
        i = __v1037;
    }
    // If no match, return 0.
    return 0 as i32;
}

/// Generate an ORDER BY or GROUP BY term out-of-range error.
///
/// # Arguments
///
/// * `pParse` - The error context into which to write the error
/// * `zType` - "ORDER" or "GROUP"
/// * `i` - The index (1-based) of the term out of range
/// * `mx` - Largest permissible value of i
/// * `pError` - Associate the error with the expression
fn resolveOutOfRangeError(
    mut pParse: *mut Parse,
    mut zType: *const i8,
    mut i: i32,
    mut mx: i32,
    mut pError: *mut Expr,
) {
    unsafe {
        sqlite3ErrorMsg(
            pParse,
            (b"%r %s BY term out of range - should be between 1 and %d\0".as_ptr() as *mut i8)
                as *const i8,
            i,
            zType,
            mx,
        )
    };
    unsafe { sqlite3RecordErrorOffsetOfExpr(unsafe { (*pParse).db }, pError as *const Expr) };
}

/// Analyze the ORDER BY clause in a compound SELECT statement.   Modify
/// each term of the ORDER BY clause is a constant integer between 1
/// and N where N is the number of columns in the compound SELECT.
///
/// ORDER BY terms that are already an integer between 1 and N are
/// unmodified.  ORDER BY terms that are integers outside the range of
/// 1 through N generate an error.  ORDER BY terms that are expressions
/// are matched against result set expressions of compound SELECT
/// beginning with the left-most SELECT and working toward the right.
/// At the first match, the ORDER BY expression is transformed into
/// the integer column number.
///
/// Return the number of errors seen.
///
/// # Arguments
///
/// * `pParse` - Parsing context.  Leave error messages here
/// * `pSelect` - The SELECT statement containing the ORDER BY
fn resolveCompoundOrderBy(mut pParse: *mut Parse, mut pSelect: *mut Select) -> i32 {
    let mut i: i32 = 0 as i32;
    let mut pOrderBy: *mut ExprList = unsafe { std::mem::zeroed() };
    let mut pEList: *mut ExprList = unsafe { std::mem::zeroed() };
    let mut db: *mut sqlite3 = unsafe { std::mem::zeroed() };
    let mut moreToDo: i32 = 1 as i32;
    pOrderBy = unsafe { (*pSelect).pOrderBy };
    if pOrderBy == std::ptr::null_mut::<ExprList>() {
        return 0 as i32;
    }
    db = unsafe { (*pParse).db };
    if (unsafe { (*pOrderBy).nExpr })
        > unsafe {
            *unsafe { unsafe { (*db).aLimit.as_mut_ptr() as *mut i32 }.offset((2 as i32) as isize) }
        }
    {
        unsafe {
            sqlite3ErrorMsg(
                pParse,
                (b"too many terms in ORDER BY clause\0".as_ptr() as *mut i8) as *const i8,
            )
        };
        return 1 as i32;
    }
    i = 0 as i32;
    '__slate_break_800: loop {
        if !(i < unsafe { (*pOrderBy).nExpr }) {
            break;
        }
        unsafe {
            (*unsafe {
                unsafe { std::ptr::addr_of_mut!((*pOrderBy).a) as *mut ExprList_item }
                    .offset(i as isize)
            })
            .fg
            .__slate_bits_0
            .__set_done((0 as i32) as u32);
        }
        let __v1038: i32 = i;
        let __v1039: i32 = __v1038 + (1 as i32);
        i = __v1039;
    }
    unsafe {
        (*pSelect).pNext = std::ptr::null_mut::<Select>();
    }
    '__slate_break_801: while (unsafe { (*pSelect).pPrior }) != std::ptr::null_mut::<Select>() {
        unsafe {
            (*unsafe { (*pSelect).pPrior }).pNext = pSelect;
        }
        pSelect = unsafe { (*pSelect).pPrior };
    }
    '__slate_break_802: while pSelect != std::ptr::null_mut::<Select>() && moreToDo != (0 as i32) {
        let mut pItem: *mut ExprList_item = unsafe { std::mem::zeroed() };
        moreToDo = 0 as i32;
        pEList = unsafe { (*pSelect).pEList };
        0 as i32;
        i = 0 as i32;
        let __v1040: *mut ExprList_item =
            unsafe { std::ptr::addr_of_mut!((*pOrderBy).a) as *mut ExprList_item };
        pItem = __v1040;
        '__slate_break_803: while i < unsafe { (*pOrderBy).nExpr } {
            let mut iCol: i32 = -(1 as i32);
            let mut pE: *mut Expr = unsafe { std::mem::zeroed() };
            let mut pDup: *mut Expr = unsafe { std::mem::zeroed() };
            if ((unsafe { (*pItem).fg.__slate_bits_0.__get_done() }) as i32) != (0 as i32) {
            } else {
                pE = unsafe { sqlite3ExprSkipCollateAndLikely(unsafe { (*pItem).pExpr }) };
                if pE == std::ptr::null_mut::<Expr>() {
                } else {
                    if (unsafe {
                        sqlite3ExprIsInteger(
                            pE as *const Expr,
                            std::ptr::addr_of_mut!(iCol),
                            std::ptr::null_mut::<Parse>(),
                            0 as i32,
                        )
                    }) != (0 as i32)
                    {
                        if iCol <= (0 as i32) || iCol > unsafe { (*pEList).nExpr } {
                            resolveOutOfRangeError(
                                pParse,
                                (b"ORDER\0".as_ptr() as *mut i8) as *const i8,
                                i + (1 as i32),
                                unsafe { (*pEList).nExpr },
                                pE,
                            );
                            return 1 as i32;
                        }
                    } else {
                        iCol = resolveAsName(pParse, pEList, pE);
                        if iCol == (0 as i32) {
                            // Now test if expression pE matches one of the values returned
                            // by pSelect. In the usual case this is done by duplicating the
                            // expression, resolving any symbols in it, and then comparing
                            // it against each expression returned by the SELECT statement.
                            // Once the comparisons are finished, the duplicate expression
                            // is deleted.
                            //
                            // If this is running as part of an ALTER TABLE operation and
                            // the symbols resolve successfully, also resolve the symbols in the
                            // actual expression. This allows the code in alter.c to modify
                            // column references within the ORDER BY expression as required.
                            pDup = unsafe { sqlite3ExprDup(db, pE as *const Expr, 0 as i32) };
                            if !((unsafe { (*db).mallocFailed }) != (0 as u8)) {
                                0 as i32;
                                iCol = resolveOrderByTermToExprList(pParse, pSelect, pDup);
                                if (((unsafe { (*pParse).eParseMode }) as u32) as i32) >= (2 as i32)
                                    && iCol > (0 as i32)
                                {
                                    resolveOrderByTermToExprList(pParse, pSelect, pE);
                                }
                            }
                            unsafe { sqlite3ExprDelete(db, pDup) };
                        }
                    }
                    if iCol > (0 as i32) {
                        // Convert the ORDER BY term into an integer column number iCol,
                        // taking care to preserve the COLLATE clause if it exists.
                        if !((((unsafe { (*pParse).eParseMode }) as u32) as i32) >= (2 as i32)) {
                            let mut pNew: *mut Expr = unsafe { sqlite3ExprInt32(db, iCol) };
                            if pNew == std::ptr::null_mut::<Expr>() {
                                return 1 as i32;
                            }
                            if (unsafe { (*pItem).pExpr }) == pE {
                                unsafe {
                                    (*pItem).pExpr = pNew;
                                }
                            } else {
                                let mut pParent: *mut Expr = unsafe { (*pItem).pExpr };
                                0 as i32;
                                '__slate_break_805: while (((unsafe {
                                    (*unsafe { (*pParent).pLeft }).op
                                })
                                    as u32)
                                    as i32)
                                    == (114 as i32)
                                {
                                    pParent = unsafe { (*pParent).pLeft };
                                }
                                0 as i32;
                                unsafe {
                                    (*pParent).pLeft = pNew;
                                }
                            }
                            unsafe { sqlite3ExprDelete(db, pE) };
                            unsafe {
                                (*pItem).u.x.iOrderByCol = (iCol as i16) as u16;
                            }
                        }
                        unsafe {
                            (*pItem).fg.__slate_bits_0.__set_done((1 as i32) as u32);
                        }
                    } else {
                        moreToDo = 1 as i32;
                    }
                }
            }
            let __v1041: i32 = i;
            let __v1042: i32 = __v1041 + (1 as i32);
            i = __v1042;
            let __v1043: *mut ExprList_item = pItem;
            let __v1044: *mut ExprList_item = unsafe { __v1043.offset((1 as i32) as isize) };
            pItem = __v1044;
        }
        pSelect = unsafe { (*pSelect).pNext };
    }
    i = 0 as i32;
    '__slate_break_806: loop {
        if !(i < unsafe { (*pOrderBy).nExpr }) {
            break;
        }
        if ((unsafe {
            (*unsafe {
                unsafe { std::ptr::addr_of_mut!((*pOrderBy).a) as *mut ExprList_item }
                    .offset(i as isize)
            })
            .fg
            .__slate_bits_0
            .__get_done()
        }) as i32)
            == (0 as i32)
        {
            unsafe {
                sqlite3ErrorMsg(
                    pParse,
                    (b"%r ORDER BY term does not match any column in the result set\0".as_ptr()
                        as *mut i8) as *const i8,
                    i + (1 as i32),
                )
            };
            return 1 as i32;
        }
        let __v1045: i32 = i;
        let __v1046: i32 = __v1045 + (1 as i32);
        i = __v1046;
    }
    return 0 as i32;
}

/// Check every term in the ORDER BY or GROUP BY clause pOrderBy of
/// the SELECT statement pSelect.  If any term is reference to a
/// result set expression (as determined by the ExprList.a.u.x.iOrderByCol
/// field) then convert that term into a copy of the corresponding result set
/// column.
///
/// If any errors are detected, add an error message to pParse and
/// return non-zero.  Return zero if no errors are seen.
///
/// # Arguments
///
/// * `pParse` - Parsing context.  Leave error messages here
/// * `pSelect` - The SELECT statement containing the clause
/// * `pOrderBy` - The ORDER BY or GROUP BY clause to be processed
/// * `zType` - "ORDER" or "GROUP"
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3ResolveOrderGroupBy(
    mut pParse: *mut Parse,
    mut pSelect: *mut Select,
    mut pOrderBy: *mut ExprList,
    mut zType: *const i8,
) -> i32 {
    let mut i: i32 = 0 as i32;
    let mut db: *mut sqlite3 = unsafe { (*pParse).db };
    let mut pEList: *mut ExprList = unsafe { std::mem::zeroed() };
    let mut pItem: *mut ExprList_item = unsafe { std::mem::zeroed() };
    if pOrderBy == std::ptr::null_mut::<ExprList>()
        || (unsafe { (*unsafe { (*pParse).db }).mallocFailed }) != (0 as u8)
        || (((unsafe { (*pParse).eParseMode }) as u32) as i32) >= (2 as i32)
    {
        return 0 as i32;
    }
    if (unsafe { (*pOrderBy).nExpr })
        > unsafe {
            *unsafe { unsafe { (*db).aLimit.as_mut_ptr() as *mut i32 }.offset((2 as i32) as isize) }
        }
    {
        unsafe {
            sqlite3ErrorMsg(
                pParse,
                (b"too many terms in %s BY clause\0".as_ptr() as *mut i8) as *const i8,
                zType,
            )
        };
        return 1 as i32;
    }
    pEList = unsafe { (*pSelect).pEList };
    0 as i32; // sqlite3SelectNew() guarantees this
    i = 0 as i32;
    let __v874: *mut ExprList_item =
        unsafe { std::ptr::addr_of_mut!((*pOrderBy).a) as *mut ExprList_item };
    pItem = __v874;
    '__slate_break_809: while i < unsafe { (*pOrderBy).nExpr } {
        if (unsafe { (*pItem).u.x.iOrderByCol }) != (0 as u16) {
            if (((unsafe { (*pItem).u.x.iOrderByCol }) as u32) as i32) > unsafe { (*pEList).nExpr }
            {
                resolveOutOfRangeError(
                    pParse,
                    zType,
                    i + (1 as i32),
                    unsafe { (*pEList).nExpr },
                    std::ptr::null_mut::<Expr>(),
                );
                return 1 as i32;
            }
            resolveAlias(
                pParse,
                pEList,
                (((unsafe { (*pItem).u.x.iOrderByCol }) as u32) as i32) - (1 as i32),
                unsafe { (*pItem).pExpr },
                0 as i32,
            );
        }
        let __v875: i32 = i;
        let __v876: i32 = __v875 + (1 as i32);
        i = __v876;
        let __v877: *mut ExprList_item = pItem;
        let __v878: *mut ExprList_item = unsafe { __v877.offset((1 as i32) as isize) };
        pItem = __v878;
    }
    return 0 as i32;
}

/// Walker callback for windowRemoveExprFromSelect().
#[unsafe(link_section = ".text.slate_distinct.resolve.resolveRemoveWindowsCb")]
extern "C-unwind" fn resolveRemoveWindowsCb(mut pWalker: *mut Walker, mut pExpr: *mut Expr) -> i32 {
    pWalker;
    if (unsafe { (*pExpr).flags }) & ((16777216 as i32) as u32) != ((0 as i32) as u32) {
        let mut pWin: *mut Window = unsafe { (*pExpr).y.pWin };
        unsafe { sqlite3WindowUnlinkFromSelect(pWin) };
    }
    return 0 as i32;
}

/// Remove any Window objects owned by the expression pExpr from the
/// Select.pWin list of Select object pSelect.
fn windowRemoveExprFromSelect(mut pSelect: *mut Select, mut pExpr: *mut Expr) {
    if (unsafe { (*pSelect).pWin }) != std::ptr::null_mut::<Window>() {
        let mut sWalker: Walker = unsafe { std::mem::zeroed() };
        unsafe {
            memset(
                std::ptr::addr_of_mut!(sWalker) as *mut (),
                0 as i32,
                48 as u64,
            )
        };
        sWalker.xExprCallback = Some(resolveRemoveWindowsCb);
        unsafe {
            sWalker.u.pSelect = pSelect;
        }
        unsafe { sqlite3WalkExpr(std::ptr::addr_of_mut!(sWalker), pExpr) };
    }
}

/// pOrderBy is an ORDER BY or GROUP BY clause in SELECT statement pSelect.
/// The Name context of the SELECT statement is pNC.  zType is either
/// "ORDER" or "GROUP" depending on which type of clause pOrderBy is.
///
/// This routine resolves each term of the clause into an expression.
/// If the order-by term is an integer I between 1 and N (where N is the
/// number of columns in the result set of the SELECT) then the expression
/// in the resolution is a copy of the I-th result-set expression.  If
/// the order-by term is an identifier that corresponds to the AS-name of
/// a result-set expression, then the term resolves to a copy of the
/// result-set expression.  Otherwise, the expression is resolved in
/// the usual way - using sqlite3ResolveExprNames().
///
/// This routine returns the number of errors.  If errors occur, then
/// an appropriate error message might be left in pParse.  (OOM errors
/// excepted.)
///
/// # Arguments
///
/// * `pNC` - The name context of the SELECT statement
/// * `pSelect` - The SELECT statement holding pOrderBy
/// * `pOrderBy` - An ORDER BY or GROUP BY clause to resolve
/// * `zType` - Either "ORDER" or "GROUP", as appropriate
fn resolveOrderGroupBy(
    mut pNC: *mut NameContext,
    mut pSelect: *mut Select,
    mut pOrderBy: *mut ExprList,
    mut zType: *const i8,
) -> i32 {
    let mut i: i32 = 0 as i32;
    let mut j: i32 = 0 as i32; // Loop counters
    let mut iCol: i32 = 0 as i32; // Column number
    let mut pItem: *mut ExprList_item = unsafe { std::mem::zeroed() }; // A term of the ORDER BY clause
    let mut pParse: *mut Parse = unsafe { std::mem::zeroed() }; // Parsing context
    let mut nResult: i32 = 0 as i32; // Number of terms in the result set
    0 as i32;
    nResult = unsafe { (*unsafe { (*pSelect).pEList }).nExpr };
    pParse = unsafe { (*pNC).pParse };
    i = 0 as i32;
    let __v1047: *mut ExprList_item =
        unsafe { std::ptr::addr_of_mut!((*pOrderBy).a) as *mut ExprList_item };
    pItem = __v1047;
    '__slate_break_810: while i < unsafe { (*pOrderBy).nExpr } {
        '__slate_continue_810: {
            let mut pE: *mut Expr = unsafe { (*pItem).pExpr };
            let mut pE2: *mut Expr = unsafe { sqlite3ExprSkipCollateAndLikely(pE) };
            if pE2 == std::ptr::null_mut::<Expr>() {
            } else {
                if ((unsafe { *unsafe { zType.offset((0 as i32) as isize) } }) as i32)
                    != (71 as i32)
                {
                    iCol = resolveAsName(pParse, unsafe { (*pSelect).pEList }, pE2);
                    if iCol > (0 as i32) {
                        // If an AS-name match is found, mark this ORDER BY column as being
                        // a copy of the iCol-th result-set column.  The subsequent call to
                        // sqlite3ResolveOrderGroupBy() will convert the expression to a
                        // copy of the iCol-th result-set expression.
                        unsafe {
                            (*pItem).u.x.iOrderByCol = (iCol as i16) as u16;
                        }
                        break '__slate_continue_810;
                    }
                }
                if (unsafe {
                    sqlite3ExprIsInteger(
                        pE2 as *const Expr,
                        std::ptr::addr_of_mut!(iCol),
                        std::ptr::null_mut::<Parse>(),
                        0 as i32,
                    )
                }) != (0 as i32)
                {
                    // The ORDER BY term is an integer constant.  Again, set the column
                    // number so that sqlite3ResolveOrderGroupBy() will convert the
                    // order-by term to a copy of the result-set expression
                    if iCol < (1 as i32) || iCol > (65535 as i32) {
                        resolveOutOfRangeError(pParse, zType, i + (1 as i32), nResult, pE2);
                        return 1 as i32;
                    }
                    unsafe {
                        (*pItem).u.x.iOrderByCol = (iCol as i16) as u16;
                    }
                } else {
                    // Otherwise, treat the ORDER BY term as an ordinary expression
                    unsafe {
                        (*pItem).u.x.iOrderByCol = ((0 as i32) as i16) as u16;
                    }
                    if sqlite3ResolveExprNames(pNC, pE) != (0 as i32) {
                        return 1 as i32;
                    }
                    j = 0 as i32;
                    '__slate_break_811: loop {
                        if !(j < unsafe { (*unsafe { (*pSelect).pEList }).nExpr }) {
                            break;
                        }
                        if (unsafe {
                            sqlite3ExprCompare(
                                std::ptr::null::<Parse>(),
                                pE as *const Expr,
                                (unsafe {
                                    (*unsafe {
                                        unsafe {
                                            std::ptr::addr_of_mut!(
                                                (*unsafe { (*pSelect).pEList }).a
                                            )
                                                as *mut ExprList_item
                                        }
                                        .offset(j as isize)
                                    })
                                    .pExpr
                                }) as *const Expr,
                                -(1 as i32),
                            )
                        }) == (0 as i32)
                        {
                            // Since this expression is being changed into a reference
                            // to an identical expression in the result set, remove all Window
                            // objects belonging to the expression from the Select.pWin list.
                            windowRemoveExprFromSelect(pSelect, pE);
                            unsafe {
                                (*pItem).u.x.iOrderByCol = ((j + (1 as i32)) as i16) as u16;
                            }
                        }
                        let __v1052: i32 = j;
                        let __v1053: i32 = __v1052 + (1 as i32);
                        j = __v1053;
                    }
                }
            }
        }
        let __v1048: i32 = i;
        let __v1049: i32 = __v1048 + (1 as i32);
        i = __v1049;
        let __v1050: *mut ExprList_item = pItem;
        let __v1051: *mut ExprList_item = unsafe { __v1050.offset((1 as i32) as isize) };
        pItem = __v1051;
    }
    return sqlite3ResolveOrderGroupBy(pParse, pSelect, pOrderBy, zType);
}

/// Resolve names in the SELECT statement p and all of its descendants.
#[unsafe(link_section = ".text.slate_distinct.resolve.resolveSelectStep")]
extern "C-unwind" fn resolveSelectStep(mut pWalker: *mut Walker, mut p: *mut Select) -> i32 {
    let mut pOuterNC: *mut NameContext = unsafe { std::mem::zeroed() }; // Context that contains this SELECT
    let mut sNC: NameContext = unsafe { std::mem::zeroed() }; // Name context of this SELECT
    let mut isCompound: i32 = 0 as i32; // True if p is a compound select
    let mut nCompound: i32 = 0 as i32; // Number of compound terms processed so far
    let mut pParse: *mut Parse = unsafe { std::mem::zeroed() }; // Parsing context
    let mut i: i32 = 0 as i32; // Loop counter
    let mut pGroupBy: *mut ExprList = unsafe { std::mem::zeroed() }; // The GROUP BY clause
    let mut pLeftmost: *mut Select = unsafe { std::mem::zeroed() }; // Left-most of SELECT of a compound
    let mut db: *mut sqlite3 = unsafe { std::mem::zeroed() }; // Database connection
    0 as i32;
    if (unsafe { (*p).selFlags }) & ((4 as i32) as u32) != (0 as u32) {
        return 1 as i32;
    }
    pOuterNC = unsafe { (*pWalker).u.pNC };
    pParse = unsafe { (*pWalker).pParse };
    db = unsafe { (*pParse).db };
    // Normally sqlite3SelectExpand() will be called first and will have
    // already expanded this SELECT.  However, if this is a subquery within
    // an expression, sqlite3ResolveExprNames() will be called without a
    // prior call to sqlite3SelectExpand().  When that happens, let
    // sqlite3SelectPrep() do all of the processing for this SELECT.
    // sqlite3SelectPrep() will invoke both sqlite3SelectExpand() and
    // this routine in the correct order.
    if (unsafe { (*p).selFlags }) & ((64 as i32) as u32) == ((0 as i32) as u32) {
        unsafe { sqlite3SelectPrep(pParse, p, pOuterNC) };
        return if (unsafe { (*pParse).nErr }) != (0 as i32) {
            2 as i32
        } else {
            1 as i32
        };
    }
    isCompound = ((unsafe { (*p).pPrior }) != std::ptr::null_mut::<Select>()) as i32;
    nCompound = 0 as i32;
    pLeftmost = p;
    '__slate_break_812: while p != std::ptr::null_mut::<Select>() {
        0 as i32;
        0 as i32;
        let __v1054: *mut Select = p;
        let __v1055: u32 = unsafe { (*__v1054).selFlags };
        let __v1056: u32 = __v1055 | ((4 as i32) as u32);
        unsafe {
            (*__v1054).selFlags = __v1056;
        }
        // Resolve the expressions in the LIMIT and OFFSET clauses. These
        // are not allowed to refer to any names, so pass an empty NameContext.
        unsafe { memset(std::ptr::addr_of_mut!(sNC) as *mut (), 0 as i32, 56 as u64) };
        sNC.pParse = pParse;
        sNC.pWinSelect = p;
        if sqlite3ResolveExprNames(std::ptr::addr_of_mut!(sNC), unsafe { (*p).pLimit })
            != (0 as i32)
        {
            return 2 as i32;
        }
        // If the SF_Converted flags is set, then this Select object was
        // was created by the convertCompoundSelectToSubquery() function.
        // In this case the ORDER BY clause (p->pOrderBy) should be resolved
        // as if it were part of the sub-query, not the parent. This block
        // moves the pOrderBy down to the sub-query. It will be moved back
        // after the names have been resolved.
        if (unsafe { (*p).selFlags }) & ((65536 as i32) as u32) != (0 as u32) {
            let mut pSub: *mut Select = unsafe { std::mem::zeroed() };
            0 as i32;
            0 as i32;
            pSub = unsafe {
                (*unsafe {
                    (*unsafe {
                        unsafe { std::ptr::addr_of_mut!((*unsafe { (*p).pSrc }).a) as *mut SrcItem }
                            .offset((0 as i32) as isize)
                    })
                    .u4
                    .pSubq
                })
                .pSelect
            };
            0 as i32;
            0 as i32;
            0 as i32;
            unsafe {
                (*pSub).pOrderBy = unsafe { (*p).pOrderBy };
            }
            unsafe {
                (*p).pOrderBy = std::ptr::null_mut::<ExprList>();
            }
        }
        // Recursively resolve names in all subqueries in the FROM clause
        if pOuterNC != std::ptr::null_mut::<NameContext>() {
            let __v1057: *mut NameContext = pOuterNC;
            let __v1058: u32 = unsafe { (*__v1057).nNestedSelect };
            let __v1059: u32 = __v1058.wrapping_add((1 as i32) as u32);
            unsafe {
                (*__v1057).nNestedSelect = __v1059;
            }
        }
        i = 0 as i32;
        '__slate_break_813: loop {
            if !(i < unsafe { (*unsafe { (*p).pSrc }).nSrc }) {
                break;
            }
            let mut pItem: *mut SrcItem = unsafe {
                unsafe { std::ptr::addr_of_mut!((*unsafe { (*p).pSrc }).a) as *mut SrcItem }
                    .offset(i as isize)
            };
            0 as i32; // Test of tag-20240424-1
            if ((unsafe { (*pItem).fg.__slate_bits_0.__get_isSubquery() }) as i32) != (0 as i32)
                && (unsafe { (*unsafe { (*unsafe { (*pItem).u4.pSubq }).pSelect }).selFlags })
                    & ((4 as i32) as u32)
                    == ((0 as i32) as u32)
            {
                let mut nRef: i32 = if pOuterNC != std::ptr::null_mut::<NameContext>() {
                    unsafe { (*pOuterNC).nRef }
                } else {
                    0 as i32
                };
                let mut zSavedContext: *const i8 = unsafe { (*pParse).zAuthContext };
                if (unsafe { (*pItem).zName }) != std::ptr::null_mut::<i8>() {
                    unsafe {
                        (*pParse).zAuthContext = (unsafe { (*pItem).zName }) as *const i8;
                    }
                }
                sqlite3ResolveSelectNames(
                    pParse,
                    unsafe { (*unsafe { (*pItem).u4.pSubq }).pSelect },
                    pOuterNC,
                );
                unsafe {
                    (*pParse).zAuthContext = zSavedContext;
                }
                if (unsafe { (*pParse).nErr }) != (0 as i32) {
                    return 2 as i32;
                }
                0 as i32;
                // If the number of references to the outer context changed when
                // expressions in the sub-select were resolved, the sub-select
                // is correlated. It is not required to check the refcount on any
                // but the innermost outer context object, as lookupName() increments
                // the refcount on all contexts between the current one and the
                // context containing the column when it resolves a name.
                if pOuterNC != std::ptr::null_mut::<NameContext>() {
                    0 as i32;
                    unsafe {
                        (*pItem)
                            .fg
                            .__slate_bits_0
                            .__set_isCorrelated(((unsafe { (*pOuterNC).nRef }) > nRef) as u32);
                    }
                }
            }
            let __v1060: i32 = i;
            let __v1061: i32 = __v1060 + (1 as i32);
            i = __v1061;
        }
        if pOuterNC != std::ptr::null_mut::<NameContext>()
            && (unsafe { (*pOuterNC).nNestedSelect }) > ((0 as i32) as u32)
        {
            let __v1062: *mut NameContext = pOuterNC;
            let __v1063: u32 = unsafe { (*__v1062).nNestedSelect };
            let __v1064: u32 = __v1063.wrapping_sub((1 as i32) as u32);
            unsafe {
                (*__v1062).nNestedSelect = __v1064;
            }
        }
        // Set up the local name-context to pass to sqlite3ResolveExprNames() to
        // resolve the result-set expression list.
        sNC.ncFlags = (1 as i32) | (16384 as i32);
        sNC.pSrcList = unsafe { (*p).pSrc };
        sNC.pNext = pOuterNC;
        // Resolve names in the result set.
        if sqlite3ResolveExprListNames(std::ptr::addr_of_mut!(sNC), unsafe { (*p).pEList })
            != (0 as i32)
        {
            return 2 as i32;
        }
        let __v1065: i32 = sNC.ncFlags;
        let __v1066: i32 = __v1065 & !(16384 as i32);
        sNC.ncFlags = __v1066;
        // If there are no aggregate functions in the result-set, and no GROUP BY
        // expression, do not allow aggregates in any of the other expressions.
        0 as i32;
        pGroupBy = unsafe { (*p).pGroupBy };
        if pGroupBy != std::ptr::null_mut::<ExprList>() || sNC.ncFlags & (16 as i32) != (0 as i32) {
            0 as i32;
            0 as i32;
            let __v1067: *mut Select = p;
            let __v1068: u32 = unsafe { (*__v1067).selFlags };
            let __v1069: u32 = __v1068
                | (((8 as i32) | sNC.ncFlags & ((4096 as i32) | (134217728 as i32))) as u32);
            unsafe {
                (*__v1067).selFlags = __v1069;
            }
        } else {
            let __v1070: i32 = sNC.ncFlags;
            let __v1071: i32 = __v1070 & !(1 as i32);
            sNC.ncFlags = __v1071;
        }
        // Add the output column list to the name-context before parsing the
        // other expressions in the SELECT statement. This is so that
        // expressions in the WHERE clause (etc.) can refer to expressions by
        // aliases in the result set.
        //
        // Minor point: If this is the case, then the expression will be
        // re-evaluated for each reference to it.
        0 as i32;
        unsafe {
            sNC.uNC.pEList = unsafe { (*p).pEList };
        }
        let __v1072: i32 = sNC.ncFlags;
        let __v1073: i32 = __v1072 | (128 as i32);
        sNC.ncFlags = __v1073;
        if (unsafe { (*p).pHaving }) != std::ptr::null_mut::<Expr>() {
            if (unsafe { (*p).selFlags }) & ((8 as i32) as u32) == ((0 as i32) as u32) {
                unsafe {
                    sqlite3ErrorMsg(
                        pParse,
                        (b"HAVING clause on a non-aggregate query\0".as_ptr() as *mut i8)
                            as *const i8,
                    )
                };
                return 2 as i32;
            }
            if sqlite3ResolveExprNames(std::ptr::addr_of_mut!(sNC), unsafe { (*p).pHaving })
                != (0 as i32)
            {
                return 2 as i32;
            }
        }
        let __v1074: i32 = sNC.ncFlags;
        let __v1075: i32 = __v1074 | (1048576 as i32);
        sNC.ncFlags = __v1075;
        if sqlite3ResolveExprNames(std::ptr::addr_of_mut!(sNC), unsafe { (*p).pWhere })
            != (0 as i32)
        {
            return 2 as i32;
        }
        let __v1076: i32 = sNC.ncFlags;
        let __v1077: i32 = __v1076 & !(1048576 as i32);
        sNC.ncFlags = __v1077;
        // Resolve names in table-valued-function arguments
        i = 0 as i32;
        '__slate_break_815: loop {
            if !(i < unsafe { (*unsafe { (*p).pSrc }).nSrc }) {
                break;
            }
            let mut pItem: *mut SrcItem = unsafe {
                unsafe { std::ptr::addr_of_mut!((*unsafe { (*p).pSrc }).a) as *mut SrcItem }
                    .offset(i as isize)
            };
            let __v1080: bool;
            if ((unsafe { (*pItem).fg.__slate_bits_0.__get_isTabFunc() }) as i32) != (0 as i32) {
                __v1080 = sqlite3ResolveExprListNames(std::ptr::addr_of_mut!(sNC), unsafe {
                    (*pItem).u1.pFuncArg
                }) != (0 as i32);
            } else {
                __v1080 = false as bool;
            }
            if __v1080 {
                return 2 as i32;
            }
            let __v1078: i32 = i;
            let __v1079: i32 = __v1078 + (1 as i32);
            i = __v1079;
        }
        if (((unsafe { (*pParse).eParseMode }) as u32) as i32) >= (2 as i32) {
            let mut pWin: *mut Window = unsafe { std::mem::zeroed() };
            pWin = unsafe { (*p).pWinDefn };
            '__slate_break_816: while pWin != std::ptr::null_mut::<Window>() {
                let __v1081: bool;
                if sqlite3ResolveExprListNames(std::ptr::addr_of_mut!(sNC), unsafe {
                    (*pWin).pOrderBy
                }) != (0 as i32)
                {
                    __v1081 = true as bool;
                } else {
                    __v1081 = sqlite3ResolveExprListNames(std::ptr::addr_of_mut!(sNC), unsafe {
                        (*pWin).pPartition
                    }) != (0 as i32);
                }
                if __v1081 {
                    return 2 as i32;
                }
                pWin = unsafe { (*pWin).pNextWin };
            }
        }
        let __v1082: i32 = sNC.ncFlags;
        let __v1083: i32 = __v1082 | ((1 as i32) | (16384 as i32));
        sNC.ncFlags = __v1083;
        // If this is a converted compound query, move the ORDER BY clause from
        // the sub-query back to the parent query. At this point each term
        // within the ORDER BY clause has been transformed to an integer value.
        // These integers will be replaced by copies of the corresponding result
        // set expressions by the call to resolveOrderGroupBy() below.
        if (unsafe { (*p).selFlags }) & ((65536 as i32) as u32) != (0 as u32) {
            let mut pSub: *mut Select = unsafe { std::mem::zeroed() };
            0 as i32;
            pSub = unsafe {
                (*unsafe {
                    (*unsafe {
                        unsafe { std::ptr::addr_of_mut!((*unsafe { (*p).pSrc }).a) as *mut SrcItem }
                            .offset((0 as i32) as isize)
                    })
                    .u4
                    .pSubq
                })
                .pSelect
            };
            0 as i32;
            unsafe {
                (*p).pOrderBy = unsafe { (*pSub).pOrderBy };
            }
            unsafe {
                (*pSub).pOrderBy = std::ptr::null_mut::<ExprList>();
            }
        }
        // Process the ORDER BY clause for singleton SELECT statements.
        // The ORDER BY clause for compounds SELECT statements is handled
        // below, after all of the result-sets for all of the elements of
        // the compound have been resolved.
        //
        // If there is an ORDER BY clause on a term of a compound-select other
        // than the right-most term, then that is a syntax error.  But the error
        // is not detected until much later, and so we need to go ahead and
        // resolve those symbols on the incorrect ORDER BY for consistency.
        let __v1084: bool;
        if (unsafe { (*p).pOrderBy }) != std::ptr::null_mut::<ExprList>() && isCompound <= nCompound
        {
            __v1084 = resolveOrderGroupBy(
                std::ptr::addr_of_mut!(sNC),
                p,
                unsafe { (*p).pOrderBy },
                (b"ORDER\0".as_ptr() as *mut i8) as *const i8,
            ) != (0 as i32);
        } else {
            __v1084 = false as bool;
        }
        if __v1084 {
            return 2 as i32;
        }
        // Defer right-most ORDER BY of a compound
        if (unsafe { (*db).mallocFailed }) != (0 as u8) {
            return 2 as i32;
        }
        let __v1085: i32 = sNC.ncFlags;
        let __v1086: i32 = __v1085 & !(16384 as i32);
        sNC.ncFlags = __v1086;
        // Resolve the GROUP BY clause.  At the same time, make sure
        // the GROUP BY clause does not contain aggregate functions.
        if pGroupBy != std::ptr::null_mut::<ExprList>() {
            let mut pItem: *mut ExprList_item = unsafe { std::mem::zeroed() };
            if resolveOrderGroupBy(
                std::ptr::addr_of_mut!(sNC),
                p,
                pGroupBy,
                (b"GROUP\0".as_ptr() as *mut i8) as *const i8,
            ) != (0 as i32)
                || (unsafe { (*db).mallocFailed }) != (0 as u8)
            {
                return 2 as i32;
            }
            i = 0 as i32;
            let __v1087: *mut ExprList_item =
                unsafe { std::ptr::addr_of_mut!((*pGroupBy).a) as *mut ExprList_item };
            pItem = __v1087;
            '__slate_break_819: while i < unsafe { (*pGroupBy).nExpr } {
                if (unsafe { (*unsafe { (*pItem).pExpr }).flags }) & ((16 as i32) as u32)
                    != ((0 as i32) as u32)
                {
                    unsafe {
                        sqlite3ErrorMsg(
                            pParse,
                            (b"aggregate functions are not allowed in the GROUP BY clause\0"
                                .as_ptr() as *mut i8) as *const i8,
                        )
                    };
                    return 2 as i32;
                }
                let __v1088: i32 = i;
                let __v1089: i32 = __v1088 + (1 as i32);
                i = __v1089;
                let __v1090: *mut ExprList_item = pItem;
                let __v1091: *mut ExprList_item = unsafe { __v1090.offset((1 as i32) as isize) };
                pItem = __v1091;
            }
        }
        // If this is part of a compound SELECT, check that it has the right
        // number of expressions in the select list.
        if (unsafe { (*p).pNext }) != std::ptr::null_mut::<Select>()
            && (unsafe { (*unsafe { (*p).pEList }).nExpr })
                != unsafe { (*unsafe { (*unsafe { (*p).pNext }).pEList }).nExpr }
        {
            unsafe { sqlite3SelectWrongNumTermsError(pParse, unsafe { (*p).pNext }) };
            return 2 as i32;
        }
        // If the SELECT statement contains ON clauses that were moved into
        // the WHERE clause, go through and verify that none of the terms
        // in the ON clauses reference tables to the right of the ON clause.
        if (unsafe { (*p).selFlags }) & ((1073741824 as i32) as u32) != (0 as u32) {
            unsafe { sqlite3SelectCheckOnClauses(pParse, p) };
            if (unsafe { (*pParse).nErr }) != (0 as i32) {
                return 2 as i32;
            }
        }
        // Advance to the next term of the compound
        p = unsafe { (*p).pPrior };
        let __v1092: i32 = nCompound;
        let __v1093: i32 = __v1092 + (1 as i32);
        nCompound = __v1093;
    }
    // Resolve the ORDER BY on a compound SELECT after all terms of
    // the compound have been resolved.
    let __v1094: bool;
    if isCompound != (0 as i32) {
        __v1094 = resolveCompoundOrderBy(pParse, pLeftmost) != (0 as i32);
    } else {
        __v1094 = false as bool;
    }
    if __v1094 {
        return 2 as i32;
    }
    return 1 as i32;
}

/// This routine walks an expression tree and resolves references to
/// table columns and result-set columns.  At the same time, do error
/// checking on function usage and set a flag if any aggregate functions
/// are seen.
///
/// To resolve table columns references we look for nodes (or subtrees) of the
/// form X.Y.Z or Y.Z or just Z where
///
///      X:   The name of a database.  Ex:  "main" or "temp" or
///           the symbolic name assigned to an ATTACH-ed database.
///
///      Y:   The name of a table in a FROM clause.  Or in a trigger
///           one of the special names "old" or "new".
///
///      Z:   The name of a column in table Y.
///
/// The node at the root of the subtree is modified as follows:
///
///    Expr.op        Changed to TK_COLUMN
///    Expr.pTab      Points to the Table object for X.Y
///    Expr.iColumn   The column index in X.Y.  -1 for the rowid.
///    Expr.iTable    The VDBE cursor number for X.Y
///
///
/// To resolve result-set references, look for expression nodes of the
/// form Z (with no X and Y prefix) where the Z matches the right-hand
/// size of an AS clause in the result-set of a SELECT.  The Z expression
/// is replaced by a copy of the left-hand side of the result-set expression.
/// Table-name and function resolution occurs on the substituted expression
/// tree.  For example, in:
///
///      SELECT a+b AS x, c+d AS y FROM t1 ORDER BY x;
///
/// The "x" term of the order by is replaced by "a+b" to render:
///
///      SELECT a+b AS x, c+d AS y FROM t1 ORDER BY a+b;
///
/// Function calls are checked to make sure that the function is
/// defined and that the correct number of arguments are specified.
/// If the function is an aggregate function, then the NC_HasAgg flag is
/// set and the opcode is changed from TK_FUNCTION to TK_AGG_FUNCTION.
/// If an expression contains aggregate functions then the EP_Agg
/// property on the expression is set.
///
/// An error message is left in pParse if anything is amiss.  The number
/// if errors is returned.
///
/// # Arguments
///
/// * `pNC` - Namespace to resolve expressions in.
/// * `pExpr` - The expression to be analyzed.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3ResolveExprNames(
    mut pNC: *mut NameContext,
    mut pExpr: *mut Expr,
) -> i32 {
    let mut savedHasAgg: i32 = 0 as i32;
    let mut w: Walker = unsafe { std::mem::zeroed() };
    if pExpr == std::ptr::null_mut::<Expr>() {
        return 0 as i32;
    }
    savedHasAgg = (unsafe { (*pNC).ncFlags })
        & ((16 as i32) | (4096 as i32) | (32768 as i32) | (134217728 as i32));
    let __v834: *mut NameContext = pNC;
    let __v835: i32 = unsafe { (*__v834).ncFlags };
    let __v836: i32 = __v835 & !((16 as i32) | (4096 as i32) | (32768 as i32) | (134217728 as i32));
    unsafe {
        (*__v834).ncFlags = __v836;
    }
    w.pParse = unsafe { (*pNC).pParse };
    w.xExprCallback = Some(resolveExprStep);
    w.xSelectCallback = {
        let __t0: Option<unsafe extern "C-unwind" fn(*mut Walker, *mut Select) -> i32> =
            if (unsafe { (*pNC).ncFlags }) & (524288 as i32) != (0 as i32) {
                None
            } else {
                Some(resolveSelectStep)
            };
        __t0
    };
    w.xSelectCallback2 = None;
    unsafe {
        w.u.pNC = pNC;
    }
    let __v837: *mut Parse = w.pParse;
    let __v838: i32 = unsafe { (*__v837).nHeight };
    let __v839: i32 = __v838 + unsafe { (*pExpr).nHeight };
    unsafe {
        (*__v837).nHeight = __v839;
    }
    if (unsafe { sqlite3ExprCheckHeight(w.pParse, unsafe { (*w.pParse).nHeight }) }) != (0 as i32) {
        return 1 as i32;
    }
    0 as i32;
    unsafe { sqlite3WalkExprNN(std::ptr::addr_of_mut!(w), pExpr) };
    let __v840: *mut Parse = w.pParse;
    let __v841: i32 = unsafe { (*__v840).nHeight };
    let __v842: i32 = __v841 - unsafe { (*pExpr).nHeight };
    unsafe {
        (*__v840).nHeight = __v842;
    }
    0 as i32;
    0 as i32;
    {}
    {}
    let __v843: *mut Expr = pExpr;
    let __v844: u32 = unsafe { (*__v843).flags };
    let __v845: u32 =
        __v844 | (((unsafe { (*pNC).ncFlags }) & ((16 as i32) | (32768 as i32))) as u32);
    unsafe {
        (*__v843).flags = __v845;
    }
    let __v846: *mut NameContext = pNC;
    let __v847: i32 = unsafe { (*__v846).ncFlags };
    let __v848: i32 = __v847 | savedHasAgg;
    unsafe {
        (*__v846).ncFlags = __v848;
    }
    return ((unsafe { (*pNC).nNcErr }) > (0 as i32) || (unsafe { (*w.pParse).nErr }) > (0 as i32))
        as i32;
}

/// Resolve all names for all expression in an expression list.  This is
/// just like sqlite3ResolveExprNames() except that it works for an expression
/// list rather than a single expression.
///
/// The return value is SQLITE_OK (0) for success or SQLITE_ERROR (1) for a
/// failure.
///
/// # Arguments
///
/// * `pNC` - Namespace to resolve expressions in.
/// * `pList` - The expression list to be analyzed.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3ResolveExprListNames(
    mut pNC: *mut NameContext,
    mut pList: *mut ExprList,
) -> i32 {
    let mut i: i32 = 0 as i32;
    let mut savedHasAgg: i32 = 0 as i32;
    let mut w: Walker = unsafe { std::mem::zeroed() };
    if pList == std::ptr::null_mut::<ExprList>() {
        return 0 as i32;
    }
    w.pParse = unsafe { (*pNC).pParse };
    w.xExprCallback = Some(resolveExprStep);
    w.xSelectCallback = Some(resolveSelectStep);
    w.xSelectCallback2 = None;
    unsafe {
        w.u.pNC = pNC;
    }
    savedHasAgg = (unsafe { (*pNC).ncFlags })
        & ((16 as i32) | (4096 as i32) | (32768 as i32) | (134217728 as i32));
    let __v849: *mut NameContext = pNC;
    let __v850: i32 = unsafe { (*__v849).ncFlags };
    let __v851: i32 = __v850 & !((16 as i32) | (4096 as i32) | (32768 as i32) | (134217728 as i32));
    unsafe {
        (*__v849).ncFlags = __v851;
    }
    i = 0 as i32;
    '__slate_break_821: loop {
        if !(i < unsafe { (*pList).nExpr }) {
            break;
        }
        let mut pExpr: *mut Expr = unsafe {
            (*unsafe {
                unsafe { std::ptr::addr_of_mut!((*pList).a) as *mut ExprList_item }
                    .offset(i as isize)
            })
            .pExpr
        };
        if pExpr == std::ptr::null_mut::<Expr>() {
        } else {
            let __v854: *mut Parse = w.pParse;
            let __v855: i32 = unsafe { (*__v854).nHeight };
            let __v856: i32 = __v855 + unsafe { (*pExpr).nHeight };
            unsafe {
                (*__v854).nHeight = __v856;
            }
            if (unsafe { sqlite3ExprCheckHeight(w.pParse, unsafe { (*w.pParse).nHeight }) })
                != (0 as i32)
            {
                return 1 as i32;
            }
            unsafe { sqlite3WalkExprNN(std::ptr::addr_of_mut!(w), pExpr) };
            let __v857: *mut Parse = w.pParse;
            let __v858: i32 = unsafe { (*__v857).nHeight };
            let __v859: i32 = __v858 - unsafe { (*pExpr).nHeight };
            unsafe {
                (*__v857).nHeight = __v859;
            }
            0 as i32;
            0 as i32;
            {}
            {}
            if (unsafe { (*pNC).ncFlags })
                & ((16 as i32) | (4096 as i32) | (32768 as i32) | (134217728 as i32))
                != (0 as i32)
            {
                let __v860: *mut Expr = pExpr;
                let __v861: u32 = unsafe { (*__v860).flags };
                let __v862: u32 = __v861
                    | (((unsafe { (*pNC).ncFlags }) & ((16 as i32) | (32768 as i32))) as u32);
                unsafe {
                    (*__v860).flags = __v862;
                }
                let __v863: i32 = savedHasAgg;
                let __v864: i32 = __v863
                    | (unsafe { (*pNC).ncFlags })
                        & ((16 as i32) | (4096 as i32) | (32768 as i32) | (134217728 as i32));
                savedHasAgg = __v864;
                let __v865: *mut NameContext = pNC;
                let __v866: i32 = unsafe { (*__v865).ncFlags };
                let __v867: i32 =
                    __v866 & !((16 as i32) | (4096 as i32) | (32768 as i32) | (134217728 as i32));
                unsafe {
                    (*__v865).ncFlags = __v867;
                }
            }
            if (unsafe { (*w.pParse).nErr }) > (0 as i32) {
                return 1 as i32;
            }
        }
        let __v852: i32 = i;
        let __v853: i32 = __v852 + (1 as i32);
        i = __v853;
    }
    let __v868: *mut NameContext = pNC;
    let __v869: i32 = unsafe { (*__v868).ncFlags };
    let __v870: i32 = __v869 | savedHasAgg;
    unsafe {
        (*__v868).ncFlags = __v870;
    }
    return 0 as i32;
}

/// Resolve all names in all expressions of a SELECT and in all
/// descendants of the SELECT, including compounds off of p->pPrior,
/// subqueries in expressions, and subqueries used as FROM clause
/// terms.
///
/// See sqlite3ResolveExprNames() for a description of the kinds of
/// transformations that occur.
///
/// All SELECT statements should have been expanded using
/// sqlite3SelectExpand() prior to invoking this routine.
///
/// # Arguments
///
/// * `pParse` - The parser context
/// * `p` - The SELECT statement being coded.
/// * `pOuterNC` - Name context for parent SELECT statement
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3ResolveSelectNames(
    mut pParse: *mut Parse,
    mut p: *mut Select,
    mut pOuterNC: *mut NameContext,
) {
    let mut w: Walker = unsafe { std::mem::zeroed() };
    0 as i32;
    w.xExprCallback = Some(resolveExprStep);
    w.xSelectCallback = Some(resolveSelectStep);
    w.xSelectCallback2 = None;
    w.pParse = pParse;
    unsafe {
        w.u.pNC = pOuterNC;
    }
    unsafe { sqlite3WalkSelect(std::ptr::addr_of_mut!(w), p) };
}

/// Resolve names in expressions that can only reference a single table
/// or which cannot reference any tables at all.  Examples:
///
///                                                    "type" flag
///    (1)   CHECK constraints                         NC_IsCheck
///    (2)   WHERE clauses on partial indices          NC_PartIdx
///    (3)   Expressions in indexes on expressions     NC_IdxExpr
///    (4)   Expression arguments to VACUUM INTO.      0
///    (5)   GENERATED ALWAYS as expressions           NC_GenCol
///
/// In all cases except (4), the Expr.iTable value for Expr.op==TK_COLUMN
/// nodes of the expression is set to -1 and the Expr.iColumn value is
/// set to the column number.  In case (4), TK_COLUMN nodes cause an error.
///
/// Any errors cause an error message to be set in pParse.
///
/// # Arguments
///
/// * `pParse` - Parsing context
/// * `pTab` - The table being referenced, or NULL
/// * `r#type` - NC_IsCheck, NC_PartIdx, NC_IdxExpr, NC_GenCol, or 0
/// * `pExpr` - Expression to resolve.  May be NULL.
/// * `pList` - Expression list to resolve.  May be NULL.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3ResolveSelfReference(
    mut pParse: *mut Parse,
    mut pTab: *mut Table,
    mut r#type: i32,
    mut pExpr: *mut Expr,
    mut pList: *mut ExprList,
) -> i32 {
    let mut pSrc: *mut SrcList = unsafe { std::mem::zeroed() }; // Fake SrcList for pParse->pNewTable
    let mut sNC: NameContext = unsafe { std::mem::zeroed() }; // Name context for pParse->pNewTable
    let mut rc: i32 = 0 as i32;
    let mut uSrc: __SlateRecord200 = unsafe { std::mem::zeroed() };
    0 as i32;
    0 as i32;
    unsafe { memset(std::ptr::addr_of_mut!(sNC) as *mut (), 0 as i32, 56 as u64) };
    unsafe { memset(std::ptr::addr_of_mut!(uSrc) as *mut (), 0 as i32, 80 as u64) };
    pSrc = unsafe { std::ptr::addr_of_mut!(uSrc.sSrc) };
    if pTab != std::ptr::null_mut::<Table>() {
        unsafe {
            (*pSrc).nSrc = 1 as i32;
        }
        unsafe {
            (*unsafe {
                unsafe { std::ptr::addr_of_mut!((*pSrc).a) as *mut SrcItem }
                    .offset((0 as i32) as isize)
            })
            .zName = unsafe { (*pTab).zName };
        }
        unsafe {
            (*unsafe {
                unsafe { std::ptr::addr_of_mut!((*pSrc).a) as *mut SrcItem }
                    .offset((0 as i32) as isize)
            })
            .pSTab = pTab;
        }
        unsafe {
            (*unsafe {
                unsafe { std::ptr::addr_of_mut!((*pSrc).a) as *mut SrcItem }
                    .offset((0 as i32) as isize)
            })
            .iCursor = -(1 as i32);
        }
        if (unsafe { (*pTab).pSchema })
            != unsafe {
                (*unsafe { unsafe { (*unsafe { (*pParse).db }).aDb }.offset((1 as i32) as isize) })
                    .pSchema
            }
        {
            // Cause EP_FromDDL to be set on TK_FUNCTION nodes of non-TEMP
            // schema elements
            let __v871: i32 = r#type;
            let __v872: i32 = __v871 | (262144 as i32);
            r#type = __v872;
        }
    }
    sNC.pParse = pParse;
    sNC.pSrcList = pSrc;
    sNC.ncFlags = r#type | (65536 as i32);
    let __v873: i32 = sqlite3ResolveExprNames(std::ptr::addr_of_mut!(sNC), pExpr);
    rc = __v873;
    if __v873 != (0 as i32) {
        return rc;
    }
    if pList != std::ptr::null_mut::<ExprList>() {
        rc = sqlite3ResolveExprListNames(std::ptr::addr_of_mut!(sNC), pList);
    }
    return rc;
}
