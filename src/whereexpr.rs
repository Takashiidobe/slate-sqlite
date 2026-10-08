//! 2015-06-08
//!
//! The author disclaims copyright to this source code.  In place of
//! a legal notice, here is a blessing:
//!
//!    May you do good and not evil.
//!    May you find forgiveness for yourself and forgive others.
//!    May you share freely, never taking more than you give.
//!
//!
//! This module contains C code that generates VDBE code used to process
//! the WHERE clause of SQL statements.
//!
//! This file was originally part of where.c but was split out to improve
//! readability and editability.  This file contains utility routines for
//! analyzing Expr objects in the WHERE clause.
unsafe extern "C" {
    static mut sqlite3StrBINARY: [i8; 0];
    static mut sqlite3UpperToLower: [u8; 0];
    static mut sqlite3CtypeMap: [u8; 0];
    fn sqlite3_value_text(__v663: *mut sqlite3_value) -> *const u8;
    fn sqlite3_value_type(__v664: *mut sqlite3_value) -> i32;
    fn memcpy(__dest: *mut (), __src: *const (), __n: u64) -> *mut ();
    fn memset(__s: *mut (), __c: i32, __n: u64) -> *mut ();
    fn sqlite3VdbeChangeP3(__v671: *mut Vdbe, addr: i32, P3: i32);
    fn sqlite3VdbeCurrentAddr(__v674: *mut Vdbe) -> i32;
    fn sqlite3VdbeGetBoundValue(__v675: *mut Vdbe, __v676: i32, __v677: u8) -> *mut sqlite3_value;
    fn sqlite3VdbeReprepareOnBind(__v678: *mut Vdbe, __v679: i32, __v680: i32);
    fn sqlite3StrICmp(__v681: *const i8, __v682: *const i8) -> i32;
    fn sqlite3Strlen30(__v683: *const i8) -> i32;
    fn sqlite3DbMallocZero(__v684: *mut sqlite3, __v685: u64) -> *mut ();
    fn sqlite3DbMallocRawNN(__v686: *mut sqlite3, __v687: u64) -> *mut ();
    fn sqlite3DbFree(__v688: *mut sqlite3, __v689: *mut ());
    fn sqlite3ErrorMsg(__v690: *mut Parse, __v691: *const i8, ...);
    fn sqlite3GetTempReg(__v692: *mut Parse) -> i32;
    fn sqlite3ReleaseTempReg(__v693: *mut Parse, __v694: i32);
    fn sqlite3ExprAlloc(
        __v695: *mut sqlite3,
        __v696: i32,
        __v697: *const Token,
        __v698: i32,
    ) -> *mut Expr;
    fn sqlite3Expr(__v699: *mut sqlite3, __v700: i32, __v701: *const i8) -> *mut Expr;
    fn sqlite3ExprInt32(__v702: *mut sqlite3, __v703: i32) -> *mut Expr;
    fn sqlite3PExpr(
        __v704: *mut Parse,
        __v705: i32,
        __v706: *mut Expr,
        __v707: *mut Expr,
    ) -> *mut Expr;
    fn sqlite3ExprSimplifiedAndOr(__v708: *mut Expr) -> *mut Expr;
    fn sqlite3ExprDelete(__v709: *mut sqlite3, __v710: *mut Expr);
    fn sqlite3ExprListAppend(
        __v711: *mut Parse,
        __v712: *mut ExprList,
        __v713: *mut Expr,
    ) -> *mut ExprList;
    fn sqlite3ExprListDelete(__v714: *mut sqlite3, __v715: *mut ExprList);
    fn sqlite3IndexBloomable(__v716: *const Index, __v717: i32) -> i32;
    fn sqlite3ExprCodeTarget(__v718: *mut Parse, __v719: *mut Expr, __v720: i32) -> i32;
    fn sqlite3ExprCompare(
        __v721: *const Parse,
        __v722: *const Expr,
        __v723: *const Expr,
        __v724: i32,
    ) -> i32;
    fn sqlite3ExprCompareSkip(__v725: *mut Expr, __v726: *mut Expr, __v727: i32) -> i32;
    fn sqlite3ExprIsConstant(__v728: *mut Parse, __v729: *mut Expr) -> i32;
    fn sqlite3ExprIsInteger(
        __v730: *const Expr,
        __v731: *mut i32,
        __v732: *mut Parse,
        __v733: i32,
    ) -> i32;
    fn sqlite3ExprCanBeNull(__v734: *const Expr) -> i32;
    fn sqlite3ExprDup(__v736: *mut sqlite3, __v737: *const Expr, __v738: i32) -> *mut Expr;
    fn sqlite3SetJoinExpr(__v739: *mut Expr, __v740: i32, __v741: u32);
    fn sqlite3AtoF(z: *const i8, __v743: *mut f64) -> i32;
    fn sqlite3Utf8Read(__v744: *mut *const u8) -> u32;
    fn sqlite3LogEst(__v745: u64) -> i16;
    fn sqlite3ExprAffinity(pExpr: *const Expr) -> i8;
    fn sqlite3ExprCollSeq(pParse: *mut Parse, pExpr: *const Expr) -> *mut CollSeq;
    fn sqlite3ExprCollSeqMatch(__v749: *mut Parse, __v750: *const Expr, __v751: *const Expr)
    -> i32;
    fn sqlite3ExprAddCollateString(
        __v752: *const Parse,
        __v753: *mut Expr,
        __v754: *const i8,
    ) -> *mut Expr;
    fn sqlite3ExprSkipCollate(__v755: *mut Expr) -> *mut Expr;
    fn sqlite3ExprSkipCollateAndLikely(__v756: *mut Expr) -> *mut Expr;
    fn sqlite3ValueFree(__v757: *mut sqlite3_value);
    fn sqlite3ExprColUsed(__v758: *mut Expr) -> u64;
    fn sqlite3IsLikeFunction(
        __v759: *mut sqlite3,
        __v760: *mut Expr,
        __v761: *mut i32,
        __v762: *mut i8,
    ) -> i32;
    fn sqlite3ExprCheckIN(__v763: *mut Parse, __v764: *mut Expr) -> i32;
    fn sqlite3GetVTable(__v765: *mut sqlite3, __v766: *mut Table) -> *mut VTable;
    fn sqlite3ExprCompareCollSeq(__v767: *mut Parse, __v768: *const Expr) -> *mut CollSeq;
    fn sqlite3BinaryCompareCollSeq(
        __v769: *mut Parse,
        __v770: *const Expr,
        __v771: *const Expr,
    ) -> *mut CollSeq;
    fn sqlite3ExprVectorSize(pExpr: *const Expr) -> i32;
    fn sqlite3ExprForVectorField(
        __v773: *mut Parse,
        __v774: *mut Expr,
        __v775: i32,
        __v776: i32,
    ) -> *mut Expr;
    fn sqlite3WhereGetMask(__v777: *mut WhereMaskSet, __v778: i32) -> u64;
    fn sqlite3WhereMalloc(pWInfo: *mut WhereInfo, nByte: u64) -> *mut ();
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
    trace: __SlateRecord159,
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
    u1: __SlateRecord160,
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
    u: __SlateRecord161,
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
    u: __SlateRecord162,
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
    u: __SlateRecord170,
    pLeft: *mut Expr,
    pRight: *mut Expr,
    x: __SlateRecord171,
    nHeight: i32,
    iTable: i32,
    iColumn: i16,
    iAgg: i16,
    w: __SlateRecord172,
    pAggInfo: *mut AggInfo,
    y: __SlateRecord173,
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
    fg: __SlateRecord180,
    iCursor: i32,
    colUsed: u64,
    u1: __SlateRecord181,
    u2: __SlateRecord182,
    u3: __SlateRecord183,
    u4: __SlateRecord184,
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
    u1: __SlateRecord186,
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

#[repr(C)]
#[derive(Clone, Copy)]
struct WhereMemBlock {
    pNext: *mut WhereMemBlock,
    sz: u64,
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
    __slate_bits_0: __slate_bits::__SlateBits158U0,
    azInit: *mut *const i8,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord159 {
    xLegacy: Option<unsafe extern "C-unwind" fn(*mut (), *const i8)>,
    xV2: Option<unsafe extern "C-unwind" fn(u32, *mut (), *mut (), *mut ()) -> i32>,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord160 {
    isInterrupted: i32,
    notUsed1: f64,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord161 {
    pHash: *mut FuncDef,
    pDestructor: *mut FuncDestructor,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord162 {
    tab: __SlateRecord163,
    view: __SlateRecord164,
    vtab: __SlateRecord165,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord163 {
    addColOffset: i32,
    pFKey: *mut FKey,
    pDfltList: *mut ExprList,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord164 {
    pSelect: *mut Select,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord165 {
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
union __SlateRecord170 {
    zToken: *mut i8,
    iValue: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord171 {
    pList: *mut ExprList,
    pSelect: *mut Select,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord172 {
    iJoin: i32,
    iOfst: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord173 {
    pTab: *mut Table,
    pWin: *mut Window,
    nReg: i32,
    sub: __SlateRecord174,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord174 {
    iAddr: i32,
    regReturn: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct ExprList_item {
    pExpr: *mut Expr,
    zEName: *mut i8,
    fg: __SlateRecord176,
    u: __SlateRecord177,
}

#[repr(C, align(4))]
#[derive(Clone, Copy)]
struct __SlateRecord176 {
    sortFlags: u8,
    __slate_bits_0: __slate_bits::__SlateBits176U0,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord177 {
    x: __SlateRecord178,
    iConstExprReg: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord178 {
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
struct __SlateRecord180 {
    jointype: u8,
    __slate_bits_0: __slate_bits::__SlateBits180U0,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord181 {
    zIndexedBy: *mut i8,
    pFuncArg: *mut ExprList,
    nRow: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord182 {
    pIBIndex: *mut Index,
    pCteUse: *mut CteUse,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord183 {
    pOn: *mut Expr,
    pUsing: *mut IdList,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord184 {
    pSchema: *mut Schema,
    zDatabase: *mut i8,
    pSubq: *mut Subquery,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord186 {
    cr: __SlateRecord187,
    d: __SlateRecord188,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord187 {
    addrCrTab: i32,
    regRowid: i32,
    regRoot: i32,
    constraintName: Token,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord188 {
    pReturning: *mut Returning,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct WhereRightJoin {
    iMatch: i32,
    regBloom: i32,
    regReturn: i32,
    addrSubrtn: i32,
    endSubrtn: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct WhereLevel {
    iLeftJoin: i32,
    iTabCur: i32,
    iIdxCur: i32,
    addrBrk: i32,
    addrHalt: i32,
    addrNxt: i32,
    addrSkip: i32,
    addrCont: i32,
    addrFirst: i32,
    addrBody: i32,
    regBignull: i32,
    addrBignull: i32,
    iLikeRepCntr: u32,
    addrLikeRep: i32,
    regFilter: i32,
    pRJ: *mut WhereRightJoin,
    iFrom: u8,
    op: u8,
    p3: u8,
    p5: u8,
    p1: i32,
    p2: i32,
    u: __SlateRecord207,
    pWLoop: *mut WhereLoop,
    notReady: u64,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct WhereLoop {
    prereq: u64,
    maskSelf: u64,
    iTab: u8,
    iSortIdx: u8,
    rSetup: i16,
    rRun: i16,
    nOut: i16,
    u: __SlateRecord210,
    wsFlags: u32,
    nLTerm: u16,
    nSkip: u16,
    nLSlot: u16,
    aLTerm: *mut *mut WhereTerm,
    pNextLoop: *mut WhereLoop,
    aLTermSpace: [*mut WhereTerm; 3],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct WhereTerm {
    pExpr: *mut Expr,
    pWC: *mut WhereClause,
    truthProb: i16,
    wtFlags: u16,
    eOperator: u16,
    nChild: u8,
    eMatchOp: u8,
    iParent: i32,
    leftCursor: i32,
    u: __SlateRecord213,
    prereqRight: u64,
    prereqAll: u64,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct WhereClause {
    pWInfo: *mut WhereInfo,
    pOuter: *mut WhereClause,
    op: u8,
    hasOr: u8,
    nTerm: i32,
    nSlot: i32,
    nBase: i32,
    a: *mut WhereTerm,
    aStatic: [WhereTerm; 8],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct WhereOrInfo {
    wc: WhereClause,
    indexable: u64,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct WhereAndInfo {
    wc: WhereClause,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct WhereMaskSet {
    bVarSelect: i32,
    n: i32,
    ix: [i32; 64],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct WhereInfo {
    pParse: *mut Parse,
    pTabList: *mut SrcList,
    pOrderBy: *mut ExprList,
    pResultSet: *mut ExprList,
    pSelect: *mut Select,
    aiCurOnePass: [i32; 2],
    iContinue: i32,
    iBreak: i32,
    savedNQueryLoop: i32,
    wctrlFlags: u16,
    iLimit: i16,
    nLevel: u8,
    nOBSat: i8,
    eOnePass: u8,
    eDistinct: u8,
    __slate_bits_0: __slate_bits::__SlateBits136U0,
    nRowOut: i16,
    iTop: i32,
    iEndWhere: i32,
    pLoops: *mut WhereLoop,
    pMemToFree: *mut WhereMemBlock,
    revMask: u64,
    sWC: WhereClause,
    sMaskSet: WhereMaskSet,
    a: [WhereLevel; 0],
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord207 {
    r#in: __SlateRecord208,
    pCoveringIdx: *mut Index,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord208 {
    nIn: i32,
    aInLoop: *mut InLoop,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct InLoop {
    iCur: i32,
    addrInTop: i32,
    iBase: i32,
    nPrefix: i32,
    eEndLoopOp: u8,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord210 {
    btree: __SlateRecord211,
    vtab: __SlateRecord212,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord211 {
    nEq: u16,
    nBtm: u16,
    nTop: u16,
    nDistinctCol: u16,
    pIndex: *mut Index,
    pOrderBy: *mut ExprList,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord212 {
    idxNum: i32,
    __slate_bits_0: __slate_bits::__SlateBits212U0,
    isOrdered: i8,
    omitMask: u16,
    idxStr: *mut i8,
    mHandleIn: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord213 {
    x: __SlateRecord214,
    pOrInfo: *mut WhereOrInfo,
    pAndInfo: *mut WhereAndInfo,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord214 {
    leftColumn: i32,
    iField: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord215 {
    zOp: *const i8,
    eOp: u8,
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
    pub struct __SlateBits64U0 {
        #[bits(4)]
        pub notNull: u32,
        #[bits(4)]
        pub eCType: u32,
    }
    #[bitfields::bitfield([u8; 2], c_names = true, new = false, from_into_bits = false, from_traits = false, default = false, debug = false, builder = false, bit_ops = false)]
    pub struct __SlateBits176U0 {
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
    pub struct __SlateBits180U0 {
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
    pub struct __SlateBits158U0 {
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
    pub struct __SlateBits136U0 {
        #[bits(1)]
        pub bDeferredSeek: u32,
        #[bits(1)]
        pub untestedTerms: u32,
        #[bits(1)]
        pub bOrderedInnerLoop: u32,
        #[bits(1)]
        pub sorted: u32,
        #[bits(1)]
        pub bStarDone: u32,
        #[bits(1)]
        pub bStarUsed: u32,
        #[bits(2, access = na)]
        pub __slate_pad_6: u8,
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
    pub struct __SlateBits212U0 {
        #[bits(1)]
        pub needFree: u32,
        #[bits(1)]
        pub bOmitOffset: u32,
        #[bits(1)]
        pub bIdxNumHex: u32,
        #[bits(5, access = na)]
        pub __slate_pad_3: u8,
    }
}

/// Deallocate all memory associated with a WhereOrInfo object.
fn whereOrInfoDelete(mut db: *mut sqlite3, mut p: *mut WhereOrInfo) {
    sqlite3WhereClauseClear(unsafe { std::ptr::addr_of_mut!((*p).wc) });
    unsafe { sqlite3DbFree(db, p as *mut ()) };
}

/// Deallocate all memory associated with a WhereAndInfo object.
fn whereAndInfoDelete(mut db: *mut sqlite3, mut p: *mut WhereAndInfo) {
    sqlite3WhereClauseClear(unsafe { std::ptr::addr_of_mut!((*p).wc) });
    unsafe { sqlite3DbFree(db, p as *mut ()) };
}

/// Add a single new WhereTerm entry to the WhereClause object pWC.
/// The new WhereTerm object is constructed from Expr p and with wtFlags.
/// The index in pWC->a[] of the new WhereTerm is returned on success.
/// 0 is returned if the new WhereTerm could not be added due to a memory
/// allocation error.  The memory allocation failure will be recorded in
/// the db->mallocFailed flag so that higher-level functions can detect it.
///
/// This routine will increase the size of the pWC->a[] array as necessary.
///
/// If the wtFlags argument includes TERM_DYNAMIC, then responsibility
/// for freeing the expression p is assumed by the WhereClause object pWC.
/// This is true even if this routine fails to allocate a new WhereTerm.
///
/// WARNING:  This routine might reallocate the space used to store
/// WhereTerms.  All pointers to WhereTerms should be invalidated after
/// calling this routine.  Such pointers may be reinitialized by referencing
/// the pWC->a[] array.
fn whereClauseInsert(mut pWC: *mut WhereClause, mut p: *mut Expr, mut wtFlags: u16) -> i32 {
    let mut pTerm: *mut WhereTerm = unsafe { std::mem::zeroed() };
    let mut idx: i32 = 0 as i32;
    {}
    if (unsafe { (*pWC).nTerm }) >= unsafe { (*pWC).nSlot } {
        let mut pOld: *mut WhereTerm = unsafe { (*pWC).a };
        let mut db: *mut sqlite3 = unsafe { (*unsafe { (*unsafe { (*pWC).pWInfo }).pParse }).db };
        unsafe {
            (*pWC).a = (unsafe {
                sqlite3WhereMalloc(
                    unsafe { (*pWC).pWInfo },
                    (56 as u64)
                        .wrapping_mul(((unsafe { (*pWC).nSlot }) as i64) as u64)
                        .wrapping_mul(((2 as i32) as i64) as u64),
                )
            }) as *mut WhereTerm;
        }
        if (unsafe { (*pWC).a }) == std::ptr::null_mut::<WhereTerm>() {
            if ((wtFlags as u32) as i32) & (1 as i32) != (0 as i32) {
                unsafe { sqlite3ExprDelete(db, p) };
            }
            unsafe {
                (*pWC).a = pOld;
            }
            return 0 as i32;
        }
        unsafe {
            memcpy(
                (unsafe { (*pWC).a }) as *mut (),
                pOld as *const (),
                (56 as u64).wrapping_mul(((unsafe { (*pWC).nTerm }) as i64) as u64),
            )
        };
        unsafe {
            (*pWC).nSlot = (unsafe { (*pWC).nSlot }) * (2 as i32);
        }
    }
    let __v935: *mut WhereClause = pWC;
    let __v936: i32 = unsafe { (*__v935).nTerm };
    let __v937: i32 = __v936 + (1 as i32);
    unsafe {
        (*__v935).nTerm = __v937;
    }
    let __v938: i32 = __v936;
    idx = __v938;
    pTerm = unsafe { unsafe { (*pWC).a }.offset(__v938 as isize) };
    if ((wtFlags as u32) as i32) & (2 as i32) == (0 as i32) {
        unsafe {
            (*pWC).nBase = unsafe { (*pWC).nTerm };
        }
    }
    if p != std::ptr::null_mut::<Expr>()
        && (unsafe { (*p).flags }) & ((524288 as i32) as u32) != ((0 as i32) as u32)
    {
        unsafe {
            (*pTerm).truthProb =
                (((unsafe { sqlite3LogEst(((unsafe { (*p).iTable }) as i64) as u64) }) as i32)
                    - (270 as i32)) as i16;
        }
    } else {
        unsafe {
            (*pTerm).truthProb = (1 as i32) as i16;
        }
    }
    unsafe {
        (*pTerm).pExpr = unsafe { sqlite3ExprSkipCollateAndLikely(p) };
    }
    unsafe {
        (*pTerm).wtFlags = wtFlags;
    }
    unsafe {
        (*pTerm).pWC = pWC;
    }
    unsafe {
        (*pTerm).iParent = -(1 as i32);
    }
    unsafe {
        memset(
            (unsafe { std::ptr::addr_of_mut!((*pTerm).eOperator) }) as *mut (),
            0 as i32,
            (56 as u64).wrapping_sub(20 as u64),
        )
    };
    return idx;
}

/// Return TRUE if the given operator is one of the operators that is
/// allowed for an indexable WHERE clause term.  The allowed operators are
/// "=", "<", ">", "<=", ">=", "IN", "IS", and "IS NULL"
fn allowedOp(mut op: i32) -> i32 {
    0 as i32;
    0 as i32;
    0 as i32;
    0 as i32;
    0 as i32;
    0 as i32;
    0 as i32;
    if op > (58 as i32) {
        return 0 as i32;
    }
    if op >= (54 as i32) {
        return 1 as i32;
    }
    return (op == (50 as i32) || op == (51 as i32) || op == (45 as i32)) as i32;
}

/// Commute a comparison operator.  Expressions of the form "X op Y"
/// are converted into "Y op X".
fn exprCommute(mut pParse: *mut Parse, mut pExpr: *mut Expr) -> u16 {
    let __v939: bool;
    if (((unsafe { (*unsafe { (*pExpr).pLeft }).op }) as u32) as i32) == (177 as i32)
        || (((unsafe { (*unsafe { (*pExpr).pRight }).op }) as u32) as i32) == (177 as i32)
    {
        __v939 = true as bool;
    } else {
        __v939 = (unsafe {
            sqlite3BinaryCompareCollSeq(
                pParse,
                (unsafe { (*pExpr).pLeft }) as *const Expr,
                (unsafe { (*pExpr).pRight }) as *const Expr,
            )
        }) != unsafe {
            sqlite3BinaryCompareCollSeq(
                pParse,
                (unsafe { (*pExpr).pRight }) as *const Expr,
                (unsafe { (*pExpr).pLeft }) as *const Expr,
            )
        };
    }
    if __v939 {
        let __v940: *mut Expr = pExpr;
        let __v941: u32 = unsafe { (*__v940).flags };
        let __v942: u32 = __v941 ^ ((1024 as i32) as u32);
        unsafe {
            (*__v940).flags = __v942;
        }
    }
    let mut t: *mut Expr = unsafe { (*pExpr).pRight };
    unsafe {
        (*pExpr).pRight = unsafe { (*pExpr).pLeft };
    }
    unsafe {
        (*pExpr).pLeft = t;
    }
    {}
    if (((unsafe { (*pExpr).op }) as u32) as i32) >= (55 as i32) {
        0 as i32;
        0 as i32;
        0 as i32;
        0 as i32;
        0 as i32;
        unsafe {
            (*pExpr).op = ((((((unsafe { (*pExpr).op }) as u32) as i32) - (55 as i32) ^ (2 as i32))
                + (55 as i32)) as i8) as u8;
        }
    }
    return ((0 as i32) as i16) as u16;
}

/// Translate from TK_xx operator to WO_xx bitmask.
fn operatorMask(mut op: i32) -> u16 {
    let mut c: u16 = 0 as u16;
    0 as i32;
    if op >= (54 as i32) {
        0 as i32;
        c = (((2 as i32) << op - (54 as i32)) as i16) as u16;
    } else {
        if op == (50 as i32) {
            c = ((1 as i32) as i16) as u16;
        } else {
            if op == (51 as i32) {
                c = ((256 as i32) as i16) as u16;
            } else {
                0 as i32;
                c = ((128 as i32) as i16) as u16;
            }
        }
    }
    0 as i32;
    0 as i32;
    0 as i32;
    0 as i32;
    0 as i32;
    0 as i32;
    0 as i32;
    0 as i32;
    return c;
}

/// Check to see if the given expression is a LIKE or GLOB operator that
/// can be optimized using inequality constraints.  Return TRUE if it is
/// so and false if not.
///
/// In order for the operator to be optimizible, the RHS must be a string
/// literal that does not begin with a wildcard.  The LHS must be a column
/// that may only be NULL, a string, or a BLOB, never a number. (This means
/// that virtual tables cannot participate in the LIKE optimization.)  The
/// collating sequence for the column on the LHS must be appropriate for
/// the operator.
///
/// # Arguments
///
/// * `pParse` - Parsing and code generating context
/// * `pExpr` - Test this expression
/// * `ppPrefix` - Pointer to TK_STRING expression with pattern prefix
/// * `pisComplete` - True if the only wildcard is % in the last character
/// * `pnoCase` - True if uppercase is equivalent to lowercase
fn isLikeOrGlob(
    mut pParse: *mut Parse,
    mut pExpr: *mut Expr,
    mut ppPrefix: *mut *mut Expr,
    mut pisComplete: *mut i32,
    mut pnoCase: *mut i32,
) -> i32 {
    let mut z: *const u8 = std::ptr::null::<u8>(); // String on RHS of LIKE operator
    let mut pRight: *mut Expr = unsafe { std::mem::zeroed() };
    let mut pLeft: *mut Expr = unsafe { std::mem::zeroed() }; // Right and left size of LIKE operator
    let mut pList: *mut ExprList = unsafe { std::mem::zeroed() }; // List of operands to the LIKE operator
    let mut c: u8 = 0 as u8; // One character in z[]
    let mut cnt: i32 = 0 as i32; // Number of non-wildcard prefix characters
    let mut wc: [u8; 4] = [0 as u8; 4]; // Wildcard characters
    let mut db: *mut sqlite3 = unsafe { (*pParse).db }; // Database connection
    let mut pVal: *mut sqlite3_value = std::ptr::null_mut::<sqlite3_value>();
    let mut op: i32 = 0 as i32; // Opcode of pRight
    let mut rc: i32 = 0 as i32; // Result code to return
    if !((unsafe {
        sqlite3IsLikeFunction(db, pExpr, pnoCase, (wc.as_mut_ptr() as *mut u8) as *mut i8)
    }) != (0 as i32))
    {
        return 0 as i32;
    }
    0 as i32;
    pList = unsafe { (*pExpr).x.pList };
    pLeft = unsafe {
        (*unsafe {
            unsafe { std::ptr::addr_of_mut!((*pList).a) as *mut ExprList_item }
                .offset((1 as i32) as isize)
        })
        .pExpr
    };
    pRight = unsafe {
        sqlite3ExprSkipCollate(unsafe {
            (*unsafe {
                unsafe { std::ptr::addr_of_mut!((*pList).a) as *mut ExprList_item }
                    .offset((0 as i32) as isize)
            })
            .pExpr
        })
    };
    op = ((unsafe { (*pRight).op }) as u32) as i32;
    if op == (157 as i32)
        && (unsafe { (*db).flags }) & (((8388608 as i32) as i64) as u64)
            == (((0 as i32) as i64) as u64)
    {
        let mut pReprepare: *mut Vdbe = unsafe { (*pParse).pReprepare };
        let mut iCol: i32 = (unsafe { (*pRight).iColumn }) as i32;
        pVal = unsafe { sqlite3VdbeGetBoundValue(pReprepare, iCol, ((65 as i32) as i8) as u8) };
        let __v943: bool;
        if pVal != std::ptr::null_mut::<sqlite3_value>() {
            __v943 = (unsafe { sqlite3_value_type(pVal) }) == (3 as i32);
        } else {
            __v943 = false as bool;
        }
        if __v943 {
            z = unsafe { sqlite3_value_text(pVal) };
        }
        unsafe { sqlite3VdbeReprepareOnBind(unsafe { (*pParse).pVdbe }, iCol, 0 as i32) };
        0 as i32;
    } else {
        if op == (118 as i32) {
            0 as i32;
            z = ((unsafe { (*pRight).u.zToken }) as *mut u8) as *const u8;
        }
    }
    if z != std::ptr::null::<u8>() {
        // Count the number of prefix bytes prior to the first wildcard,
        // U+fffd character, or malformed utf-8. If the underlying database
        // has a UTF16LE encoding, then only consider ASCII characters.  Note that
        // the encoding of z[] is UTF8 - we are dealing with only UTF8 here in this
        // code, but the database engine itself might be processing content using a
        // different encoding.
        cnt = 0 as i32;
        '__slate_break_804: loop {
            let __v944: u8 = unsafe { *unsafe { z.offset(cnt as isize) } };
            c = __v944;
            if !(((__v944 as u32) as i32) != (0 as i32)
                && ((c as u32) as i32)
                    != (((unsafe {
                        *unsafe { (wc.as_mut_ptr() as *mut u8).offset((0 as i32) as isize) }
                    }) as u32) as i32)
                && ((c as u32) as i32)
                    != (((unsafe {
                        *unsafe { (wc.as_mut_ptr() as *mut u8).offset((1 as i32) as isize) }
                    }) as u32) as i32)
                && ((c as u32) as i32)
                    != (((unsafe {
                        *unsafe { (wc.as_mut_ptr() as *mut u8).offset((2 as i32) as isize) }
                    }) as u32) as i32))
            {
                break;
            }
            let __v945: i32 = cnt;
            let __v946: i32 = __v945 + (1 as i32);
            cnt = __v946;
            if ((c as u32) as i32)
                == (((unsafe {
                    *unsafe { (wc.as_mut_ptr() as *mut u8).offset((3 as i32) as isize) }
                }) as u32) as i32)
                && (((unsafe { *unsafe { z.offset(cnt as isize) } }) as u32) as i32) > (0 as i32)
                && (((unsafe { *unsafe { z.offset(cnt as isize) } }) as u32) as i32) < (128 as i32)
            {
                let __v947: i32 = cnt;
                let __v948: i32 = __v947 + (1 as i32);
                cnt = __v948;
            } else {
                if ((c as u32) as i32) >= (128 as i32) {
                    let mut z2: *const u8 =
                        unsafe { unsafe { z.offset(cnt as isize) }.offset(-((1 as i32) as isize)) };
                    let __v949: bool;
                    if ((c as u32) as i32) == (255 as i32) {
                        __v949 = true as bool;
                    } else {
                        __v949 = (unsafe { sqlite3Utf8Read(std::ptr::addr_of_mut!(z2)) })
                            == ((65533 as i32) as u32);
                    }
                    if __v949 || (((unsafe { (*db).enc }) as u32) as i32) == (2 as i32) {
                        let __v950: i32 = cnt;
                        let __v951: i32 = __v950 - (1 as i32);
                        cnt = __v951;
                        break '__slate_break_804;
                    } else {
                        cnt = ((unsafe { z2.offset_from(z as *const u8) }) as i64) as i32;
                    }
                    // bad utf-8
                }
            }
        }
        // The optimization is possible only if (1) the pattern does not begin
        // with a wildcard and if (2) the non-wildcard prefix does not end with
        // an (illegal 0xff) character, or (3) the pattern does not consist of
        // a single escape character. The second condition is necessary so
        // that we can increment the prefix key to find an upper bound for the
        // range search. The third is because the caller assumes that the pattern
        // consists of at least one character after all escapes have been
        // removed.
        if (cnt > (1 as i32)
            || cnt > (0 as i32)
                && (((unsafe { *unsafe { z.offset((0 as i32) as isize) } }) as u32) as i32)
                    != (((unsafe {
                        *unsafe { (wc.as_mut_ptr() as *mut u8).offset((3 as i32) as isize) }
                    }) as u32) as i32))
            && (255 as i32)
                != (((unsafe { *unsafe { z.offset((cnt - (1 as i32)) as isize) } }) as u32) as i32)
        {
            let mut pPrefix: *mut Expr = unsafe { std::mem::zeroed() };
            // A "complete" match if the pattern ends with "*" or "%"
            unsafe {
                *pisComplete = (((c as u32) as i32)
                    == (((unsafe {
                        *unsafe { (wc.as_mut_ptr() as *mut u8).offset((0 as i32) as isize) }
                    }) as u32) as i32)
                    && (((unsafe { *unsafe { z.offset((cnt + (1 as i32)) as isize) } }) as u32)
                        as i32)
                        == (0 as i32)
                    && (((unsafe { (*db).enc }) as u32) as i32) != (2 as i32))
                    as i32;
            }
            // Get the pattern prefix.  Remove all escapes from the prefix.
            pPrefix = unsafe { sqlite3Expr(db, 118 as i32, (z as *mut i8) as *const i8) };
            if pPrefix != std::ptr::null_mut::<Expr>() {
                let mut iFrom: i32 = 0 as i32;
                let mut iTo: i32 = 0 as i32;
                let mut zNew: *mut i8 = unsafe { std::mem::zeroed() };
                0 as i32;
                zNew = unsafe { (*pPrefix).u.zToken };
                unsafe {
                    *unsafe { zNew.offset(cnt as isize) } = (0 as i32) as i8;
                }
                iTo = 0 as i32;
                iFrom = 0 as i32;
                '__slate_break_805: loop {
                    if !(iFrom < cnt) {
                        break;
                    }
                    if ((unsafe { *unsafe { zNew.offset(iFrom as isize) } }) as i32)
                        == (((unsafe {
                            *unsafe { (wc.as_mut_ptr() as *mut u8).offset((3 as i32) as isize) }
                        }) as u32) as i32)
                    {
                        let __v954: i32 = iFrom;
                        let __v955: i32 = __v954 + (1 as i32);
                        iFrom = __v955;
                    }
                    let __v956: i32 = iTo;
                    let __v957: i32 = __v956 + (1 as i32);
                    iTo = __v957;
                    unsafe {
                        *unsafe { zNew.offset(__v956 as isize) } =
                            unsafe { *unsafe { zNew.offset(iFrom as isize) } };
                    }
                    let __v952: i32 = iFrom;
                    let __v953: i32 = __v952 + (1 as i32);
                    iFrom = __v953;
                }
                unsafe {
                    *unsafe { zNew.offset(iTo as isize) } = (0 as i32) as i8;
                }
                0 as i32;
                // If the LHS is not an ordinary column with TEXT affinity, then the
                // pattern prefix boundaries (both the start and end boundaries) must
                // not look like a number.  Otherwise the pattern might be treated as
                // a number, which will invalidate the LIKE optimization.
                //
                // Getting this right has been a persistent source of bugs in the
                // LIKE optimization.  See, for example:
                //    2018-09-10 https://sqlite.org/src/info/c94369cae9b561b1
                //    2019-05-02 https://sqlite.org/src/info/b043a54c3de54b28
                //    2019-06-10 https://sqlite.org/src/info/fd76310a5e843e07
                //    2019-06-14 https://sqlite.org/src/info/ce8717f0885af975
                //    2019-09-03 https://sqlite.org/src/info/0f0428096f17252a
                let __v958: bool;
                if (((unsafe { (*pLeft).op }) as u32) as i32) != (168 as i32) {
                    __v958 = true as bool;
                } else {
                    __v958 = ((unsafe { sqlite3ExprAffinity(pLeft as *const Expr) }) as i32)
                        != (66 as i32);
                }
                if __v958
                    || (unsafe { (*pLeft).flags })
                        & (((16777216 as i32) | (33554432 as i32)) as u32)
                        == ((0 as i32) as u32)
                        && (unsafe { (*pLeft).y.pTab }) != std::ptr::null_mut::<Table>()
                        && (((unsafe { (*unsafe { (*pLeft).y.pTab }).eTabType }) as u32) as i32)
                            == (1 as i32)
                {
                    let mut isNum: i32 = 0 as i32;
                    let mut rDummy: f64 = 0 as f64;
                    0 as i32;
                    isNum =
                        unsafe { sqlite3AtoF(zNew as *const i8, std::ptr::addr_of_mut!(rDummy)) };
                    if isNum <= (0 as i32) {
                        if iTo == (1 as i32)
                            && ((unsafe { *unsafe { zNew.offset((0 as i32) as isize) } }) as i32)
                                == (45 as i32)
                        {
                            isNum = 1 as i32;
                        } else {
                            let __v959: *mut i8 =
                                unsafe { zNew.offset((iTo - (1 as i32)) as isize) };
                            let __v960: i8 = unsafe { *__v959 };
                            let __v961: i8 = ((__v960 as i32) + (1 as i32)) as i8;
                            unsafe {
                                *__v959 = __v961;
                            }
                            isNum = unsafe {
                                sqlite3AtoF(zNew as *const i8, std::ptr::addr_of_mut!(rDummy))
                            };
                            let __v962: *mut i8 =
                                unsafe { zNew.offset((iTo - (1 as i32)) as isize) };
                            let __v963: i8 = unsafe { *__v962 };
                            let __v964: i8 = ((__v963 as i32) - (1 as i32)) as i8;
                            unsafe {
                                *__v962 = __v964;
                            }
                        }
                    }
                    if isNum > (0 as i32) {
                        unsafe { sqlite3ExprDelete(db, pPrefix) };
                        unsafe { sqlite3ValueFree(pVal) };
                        return 0 as i32;
                    }
                }
                // Might be numeric
            }
            unsafe {
                *ppPrefix = pPrefix;
            }
            // If the RHS pattern is a bound parameter, make arrangements to
            // reprepare the statement when that parameter is rebound
            if op == (157 as i32) {
                let mut v: *mut Vdbe = unsafe { (*pParse).pVdbe };
                unsafe {
                    sqlite3VdbeReprepareOnBind(v, (unsafe { (*pRight).iColumn }) as i32, 0 as i32)
                };
                0 as i32;
                if (unsafe { *pisComplete }) != (0 as i32)
                    && (unsafe {
                        *unsafe { unsafe { (*pRight).u.zToken }.offset((1 as i32) as isize) }
                    }) != (0 as i8)
                {
                    // If the rhs of the LIKE expression is a variable, and the current
                    // value of the variable means there is no need to invoke the LIKE
                    // function, then no OP_Variable will be added to the program.
                    // This causes problems for the sqlite3_bind_parameter_name()
                    // API. To work around them, add a dummy OP_Variable here.
                    let mut r1: i32 = unsafe { sqlite3GetTempReg(pParse) };
                    unsafe { sqlite3ExprCodeTarget(pParse, pRight, r1) };
                    unsafe {
                        sqlite3VdbeChangeP3(
                            v,
                            (unsafe { sqlite3VdbeCurrentAddr(v) }) - (1 as i32),
                            0 as i32,
                        )
                    };
                    unsafe { sqlite3ReleaseTempReg(pParse, r1) };
                }
            }
        } else {
            z = std::ptr::null::<u8>();
        }
    }
    rc = (z != std::ptr::null::<u8>()) as i32;
    unsafe { sqlite3ValueFree(pVal) };
    return rc;
}

/// If pExpr is one of "like", "glob", "match", or "regexp", then
/// return the corresponding SQLITE_INDEX_CONSTRAINT_xxxx value.
/// If not, return 0.
///
/// pExpr is guaranteed to be a TK_FUNCTION.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3ExprIsLikeOperator(mut pExpr: *const Expr) -> i32 {
    let mut i: i32 = 0 as i32;
    0 as i32;
    0 as i32;
    i = 0 as i32;
    '__slate_break_810: loop {
        if !(i < ((((64 as u64) / (16 as u64)) as u32) as i32)) {
            break;
        }
        if (unsafe {
            sqlite3StrICmp((unsafe { (*pExpr).u.zToken }) as *const i8, unsafe {
                (*unsafe {
                    unsafe { std::ptr::addr_of!(aOp.0) as *const __SlateRecord215 }
                        .offset(i as isize)
                })
                .zOp
            })
        }) == (0 as i32)
        {
            return ((unsafe {
                (*unsafe {
                    unsafe { std::ptr::addr_of!(aOp.0) as *const __SlateRecord215 }
                        .offset(i as isize)
                })
                .eOp
            }) as u32) as i32;
        }
        let __v844: i32 = i;
        let __v845: i32 = __v844 + (1 as i32);
        i = __v845;
    }
    return 0 as i32;
}

static mut aOp: __SlateAlign16<[__SlateRecord215; 4]> = __SlateAlign16([
    __SlateRecord215 {
        zOp: (b"match\0".as_ptr() as *mut i8) as *const i8,
        eOp: ((64 as i32) as i8) as u8,
    },
    __SlateRecord215 {
        zOp: (b"glob\0".as_ptr() as *mut i8) as *const i8,
        eOp: ((66 as i32) as i8) as u8,
    },
    __SlateRecord215 {
        zOp: (b"like\0".as_ptr() as *mut i8) as *const i8,
        eOp: ((65 as i32) as i8) as u8,
    },
    __SlateRecord215 {
        zOp: (b"regexp\0".as_ptr() as *mut i8) as *const i8,
        eOp: ((67 as i32) as i8) as u8,
    },
]);

/// Check to see if the pExpr expression is a form that needs to be passed
/// to the xBestIndex method of virtual tables.  Forms of interest include:
///
///          Expression                   Virtual Table Operator
///          -----------------------      ---------------------------------
///      1.  column MATCH expr            SQLITE_INDEX_CONSTRAINT_MATCH
///      2.  column GLOB expr             SQLITE_INDEX_CONSTRAINT_GLOB
///      3.  column LIKE expr             SQLITE_INDEX_CONSTRAINT_LIKE
///      4.  column REGEXP expr           SQLITE_INDEX_CONSTRAINT_REGEXP
///      5.  column != expr               SQLITE_INDEX_CONSTRAINT_NE
///      6.  expr != column               SQLITE_INDEX_CONSTRAINT_NE
///      7.  column IS NOT expr           SQLITE_INDEX_CONSTRAINT_ISNOT
///      8.  expr IS NOT column           SQLITE_INDEX_CONSTRAINT_ISNOT
///      9.  column IS NOT NULL           SQLITE_INDEX_CONSTRAINT_ISNOTNULL
///
/// In every case, "column" must be a column of a virtual table.  If there
/// is a match, set *ppLeft to the "column" expression, set *ppRight to the
/// "expr" expression (even though in forms (6) and (8) the column is on the
/// right and the expression is on the left).  Also set *peOp2 to the
/// appropriate virtual table operator.  The return value is 1 or 2 if there
/// is a match.  The usual return is 1, but if the RHS is also a column
/// of virtual table in forms (5) or (7) then return 2.
///
/// If the expression matches none of the patterns above, return 0.
///
/// # Arguments
///
/// * `db` - Parsing context
/// * `pExpr` - Test this expression
/// * `peOp2` - OUT: 0 for MATCH, or else an op2 value
/// * `ppLeft` - Column expression to left of MATCH/op2
/// * `ppRight` - Expression to left of MATCH/op2
fn isAuxiliaryVtabOperator(
    mut db: *mut sqlite3,
    mut pExpr: *mut Expr,
    mut peOp2: *mut u8,
    mut ppLeft: *mut *mut Expr,
    mut ppRight: *mut *mut Expr,
) -> i32 {
    if (((unsafe { (*pExpr).op }) as u32) as i32) == (172 as i32) {
        let mut pList: *mut ExprList = unsafe { std::mem::zeroed() };
        let mut pCol: *mut Expr = unsafe { std::mem::zeroed() }; // Column reference
        let mut i: i32 = 0 as i32;
        0 as i32;
        pList = unsafe { (*pExpr).x.pList };
        if pList == std::ptr::null_mut::<ExprList>() || (unsafe { (*pList).nExpr }) != (2 as i32) {
            return 0 as i32;
        }
        // Built-in operators MATCH, GLOB, LIKE, and REGEXP attach to a
        // virtual table on their second argument, which is the same as
        // the left-hand side operand in their in-fix form.
        //
        //       vtab_column MATCH expression
        //       MATCH(expression,vtab_column)
        pCol = unsafe {
            (*unsafe {
                unsafe { std::ptr::addr_of_mut!((*pList).a) as *mut ExprList_item }
                    .offset((1 as i32) as isize)
            })
            .pExpr
        };
        0 as i32;
        let __v965: bool;
        if (((unsafe { (*pCol).op }) as u32) as i32) == (168 as i32)
            && (((unsafe { (*unsafe { (*pCol).y.pTab }).eTabType }) as u32) as i32) == (1 as i32)
        {
            let __v966: i32 = sqlite3ExprIsLikeOperator(pExpr as *const Expr);
            i = __v966;
            __v965 = __v966 != (0 as i32);
        } else {
            __v965 = false as bool;
        }
        if __v965 {
            unsafe {
                *peOp2 = (i as i8) as u8;
            }
            unsafe {
                *ppRight = unsafe {
                    (*unsafe {
                        unsafe { std::ptr::addr_of_mut!((*pList).a) as *mut ExprList_item }
                            .offset((0 as i32) as isize)
                    })
                    .pExpr
                };
            }
            unsafe {
                *ppLeft = pCol;
            }
            return 1 as i32;
        }
        // We can also match against the first column of overloaded
        // functions where xFindFunction returns a value of at least
        // SQLITE_INDEX_CONSTRAINT_FUNCTION.
        //
        //      OVERLOADED(vtab_column,expression)
        //
        // Historically, xFindFunction expected to see lower-case function
        // names.  But for this use case, xFindFunction is expected to deal
        // with function names in an arbitrary case.
        pCol = unsafe {
            (*unsafe {
                unsafe { std::ptr::addr_of_mut!((*pList).a) as *mut ExprList_item }
                    .offset((0 as i32) as isize)
            })
            .pExpr
        };
        0 as i32;
        0 as i32;
        if (((unsafe { (*pCol).op }) as u32) as i32) == (168 as i32)
            && (((unsafe { (*unsafe { (*pCol).y.pTab }).eTabType }) as u32) as i32) == (1 as i32)
        {
            let mut pVtab: *mut sqlite3_vtab = unsafe { std::mem::zeroed() };
            let mut pMod: *mut sqlite3_module = unsafe { std::mem::zeroed() };
            let mut xNotUsed: Option<
                unsafe extern "C-unwind" fn(*mut sqlite3_context, i32, *mut *mut sqlite3_value),
            > = unsafe { std::mem::zeroed() };
            let mut pNotUsed: *mut () = unsafe { std::mem::zeroed() };
            pVtab = unsafe { (*unsafe { sqlite3GetVTable(db, unsafe { (*pCol).y.pTab }) }).pVtab };
            0 as i32;
            0 as i32;
            0 as i32;
            pMod = (unsafe { (*pVtab).pModule }) as *mut sqlite3_module;
            if (unsafe { (*pMod).xFindFunction }) != None {
                i = unsafe {
                    unsafe { (*pMod).xFindFunction }.unwrap()(
                        pVtab,
                        2 as i32,
                        (unsafe { (*pExpr).u.zToken }) as *const i8,
                        std::ptr::addr_of_mut!(xNotUsed),
                        std::ptr::addr_of_mut!(pNotUsed),
                    )
                };
                if i >= (150 as i32) {
                    unsafe {
                        *peOp2 = (i as i8) as u8;
                    }
                    unsafe {
                        *ppRight = unsafe {
                            (*unsafe {
                                unsafe { std::ptr::addr_of_mut!((*pList).a) as *mut ExprList_item }
                                    .offset((1 as i32) as isize)
                            })
                            .pExpr
                        };
                    }
                    unsafe {
                        *ppLeft = pCol;
                    }
                    return 1 as i32;
                }
            }
        }
    } else {
        if (((unsafe { (*pExpr).op }) as u32) as i32) >= (54 as i32) {
            // Comparison operators are a common case.  Save a few comparisons for
            // that common case by terminating early.
            0 as i32;
            0 as i32;
            0 as i32;
            return 0 as i32;
        } else {
            if (((unsafe { (*pExpr).op }) as u32) as i32) == (53 as i32)
                || (((unsafe { (*pExpr).op }) as u32) as i32) == (46 as i32)
                || (((unsafe { (*pExpr).op }) as u32) as i32) == (52 as i32)
            {
                let mut res: i32 = 0 as i32;
                let mut pLeft: *mut Expr = unsafe { (*pExpr).pLeft };
                let mut pRight: *mut Expr = unsafe { (*pExpr).pRight };
                0 as i32;
                if (((unsafe { (*pLeft).op }) as u32) as i32) == (168 as i32)
                    && (((unsafe { (*unsafe { (*pLeft).y.pTab }).eTabType }) as u32) as i32)
                        == (1 as i32)
                {
                    let __v967: i32 = res;
                    let __v968: i32 = __v967 + (1 as i32);
                    res = __v968;
                }
                0 as i32;
                if pRight != std::ptr::null_mut::<Expr>()
                    && ((((unsafe { (*pRight).op }) as u32) as i32) == (168 as i32)
                        && (((unsafe { (*unsafe { (*pRight).y.pTab }).eTabType }) as u32) as i32)
                            == (1 as i32))
                {
                    let __v969: i32 = res;
                    let __v970: i32 = __v969 + (1 as i32);
                    res = __v970;
                    let mut t: *mut Expr = pLeft;
                    pLeft = pRight;
                    pRight = t;
                    {}
                }
                unsafe {
                    *ppLeft = pLeft;
                }
                unsafe {
                    *ppRight = pRight;
                }
                if (((unsafe { (*pExpr).op }) as u32) as i32) == (53 as i32) {
                    unsafe {
                        *peOp2 = ((68 as i32) as i8) as u8;
                    }
                }
                if (((unsafe { (*pExpr).op }) as u32) as i32) == (46 as i32) {
                    unsafe {
                        *peOp2 = ((69 as i32) as i8) as u8;
                    }
                }
                if (((unsafe { (*pExpr).op }) as u32) as i32) == (52 as i32) {
                    unsafe {
                        *peOp2 = ((70 as i32) as i8) as u8;
                    }
                }
                return res;
            }
        }
    }
    return 0 as i32;
}

/// If the pBase expression originated in the ON or USING clause of
/// a join, then transfer the appropriate markings over to derived.
fn transferJoinMarkings(mut pDerived: *mut Expr, mut pBase: *mut Expr) {
    if pDerived != std::ptr::null_mut::<Expr>()
        && (unsafe { (*pBase).flags }) & (((1 as i32) | (2 as i32)) as u32) != ((0 as i32) as u32)
    {
        let __v971: *mut Expr = pDerived;
        let __v972: u32 = unsafe { (*__v971).flags };
        let __v973: u32 = __v972 | (unsafe { (*pBase).flags }) & (((1 as i32) | (2 as i32)) as u32);
        unsafe {
            (*__v971).flags = __v973;
        }
        unsafe {
            (*pDerived).w.iJoin = unsafe { (*pBase).w.iJoin };
        }
    }
}

/// Mark term iChild as being a child of term iParent
fn markTermAsChild(mut pWC: *mut WhereClause, mut iChild: i32, mut iParent: i32) {
    unsafe {
        (*unsafe { unsafe { (*pWC).a }.offset(iChild as isize) }).iParent = iParent;
    }
    unsafe {
        (*unsafe { unsafe { (*pWC).a }.offset(iChild as isize) }).truthProb =
            unsafe { (*unsafe { unsafe { (*pWC).a }.offset(iParent as isize) }).truthProb };
    }
    0 as i32;
    let __v974: *mut WhereTerm = unsafe { unsafe { (*pWC).a }.offset(iParent as isize) };
    let __v975: u8 = unsafe { (*__v974).nChild };
    let __v976: u8 = ((((__v975 as u32) as i32) + (1 as i32)) as i8) as u8;
    unsafe {
        (*__v974).nChild = __v976;
    }
    {}
}

/// Return the N-th AND-connected subterm of pTerm.  Or if pTerm is not
/// a conjunction, then return just pTerm when N==0.  If N is exceeds
/// the number of available subterms, return NULL.
fn whereNthSubterm(mut pTerm: *mut WhereTerm, mut N: i32) -> *mut WhereTerm {
    if (((unsafe { (*pTerm).eOperator }) as u32) as i32) != (1024 as i32) {
        return if N == (0 as i32) {
            pTerm
        } else {
            std::ptr::null_mut::<WhereTerm>()
        };
    }
    if N < unsafe { (*unsafe { (*pTerm).u.pAndInfo }).wc.nTerm } {
        return unsafe { unsafe { (*unsafe { (*pTerm).u.pAndInfo }).wc.a }.offset(N as isize) };
    }
    return std::ptr::null_mut::<WhereTerm>();
}

/// Subterms pOne and pTwo are contained within WHERE clause pWC.  The
/// two subterms are in disjunction - they are OR-ed together.
///
/// If these two terms are both of the form:  "A op B" with the same
/// A and B values but different operators and if the operators are
/// compatible (if one is = and the other is <, for example) then
/// add a new virtual AND term to pWC that is the combination of the
/// two.
///
/// Some examples:
///
///    x<y OR x=y    -->     x<=y
///    x=y OR x=y    -->     x=y
///    x<=y OR x<y   -->     x<=y
///
/// The following is NOT generated:
///
///    x<y OR x>y    -->     x!=y
///
/// # Arguments
///
/// * `pSrc` - the FROM clause
/// * `pWC` - The complete WHERE clause
/// * `pOne` - First disjunct
/// * `pTwo` - Second disjunct
fn whereCombineDisjuncts(
    mut pSrc: *mut SrcList,
    mut pWC: *mut WhereClause,
    mut pOne: *mut WhereTerm,
    mut pTwo: *mut WhereTerm,
) {
    let mut eOp: u16 = (((((unsafe { (*pOne).eOperator }) as u32) as i32)
        | (((unsafe { (*pTwo).eOperator }) as u32) as i32)) as i16) as u16;
    let mut db: *mut sqlite3 = unsafe { std::mem::zeroed() }; // Database connection (for malloc)
    let mut pNew: *mut Expr = unsafe { std::mem::zeroed() }; // New virtual expression
    let mut op: i32 = 0 as i32; // Operator for the combined expression
    let mut idxNew: i32 = 0 as i32; // Index in pWC of the next virtual term
    let mut pA: *mut Expr = unsafe { std::mem::zeroed() };
    let mut pB: *mut Expr = unsafe { std::mem::zeroed() }; // Expressions associated with pOne and pTwo
    if ((((unsafe { (*pOne).wtFlags }) as u32) as i32)
        | (((unsafe { (*pTwo).wtFlags }) as u32) as i32))
        & (128 as i32)
        != (0 as i32)
    {
        return;
    }
    if (((unsafe { (*pOne).eOperator }) as u32) as i32)
        & ((2 as i32)
            | (2 as i32) << (57 as i32) - (54 as i32)
            | (2 as i32) << (56 as i32) - (54 as i32)
            | (2 as i32) << (55 as i32) - (54 as i32)
            | (2 as i32) << (58 as i32) - (54 as i32))
        == (0 as i32)
    {
        return;
    }
    if (((unsafe { (*pTwo).eOperator }) as u32) as i32)
        & ((2 as i32)
            | (2 as i32) << (57 as i32) - (54 as i32)
            | (2 as i32) << (56 as i32) - (54 as i32)
            | (2 as i32) << (55 as i32) - (54 as i32)
            | (2 as i32) << (58 as i32) - (54 as i32))
        == (0 as i32)
    {
        return;
    }
    if ((eOp as u32) as i32)
        & ((2 as i32)
            | (2 as i32) << (57 as i32) - (54 as i32)
            | (2 as i32) << (56 as i32) - (54 as i32))
        != ((eOp as u32) as i32)
        && ((eOp as u32) as i32)
            & ((2 as i32)
                | (2 as i32) << (55 as i32) - (54 as i32)
                | (2 as i32) << (58 as i32) - (54 as i32))
            != ((eOp as u32) as i32)
    {
        return;
    }
    pA = unsafe { (*pOne).pExpr };
    pB = unsafe { (*pTwo).pExpr };
    0 as i32;
    0 as i32;
    if (unsafe {
        sqlite3ExprCompare(
            std::ptr::null::<Parse>(),
            (unsafe { (*pA).pLeft }) as *const Expr,
            (unsafe { (*pB).pLeft }) as *const Expr,
            -(1 as i32),
        )
    }) != (0 as i32)
    {
        return;
    }
    if (unsafe {
        sqlite3ExprCompare(
            std::ptr::null::<Parse>(),
            (unsafe { (*pA).pRight }) as *const Expr,
            (unsafe { (*pB).pRight }) as *const Expr,
            -(1 as i32),
        )
    }) != (0 as i32)
    {
        return;
    }
    if (((unsafe { (*pA).flags }) & ((1024 as i32) as u32) != ((0 as i32) as u32)) as i32)
        != (((unsafe { (*pB).flags }) & ((1024 as i32) as u32) != ((0 as i32) as u32)) as i32)
    {
        return;
    }
    // If we reach this point, it means the two subterms can be combined
    if ((eOp as u32) as i32) & ((eOp as u32) as i32) - (1 as i32) != (0 as i32) {
        if ((eOp as u32) as i32)
            & ((2 as i32) << (57 as i32) - (54 as i32) | (2 as i32) << (56 as i32) - (54 as i32))
            != (0 as i32)
        {
            eOp = (((2 as i32) << (56 as i32) - (54 as i32)) as i16) as u16;
        } else {
            0 as i32;
            eOp = (((2 as i32) << (58 as i32) - (54 as i32)) as i16) as u16;
        }
    }
    db = unsafe { (*unsafe { (*unsafe { (*pWC).pWInfo }).pParse }).db };
    pNew = unsafe { sqlite3ExprDup(db, pA as *const Expr, 0 as i32) };
    if pNew == std::ptr::null_mut::<Expr>() {
        return;
    }
    op = 54 as i32;
    '__slate_break_811: loop {
        if !(((eOp as u32) as i32) != (2 as i32) << op - (54 as i32)) {
            break;
        }
        0 as i32;
        let __v977: i32 = op;
        let __v978: i32 = __v977 + (1 as i32);
        op = __v978;
    }
    unsafe {
        (*pNew).op = (op as i8) as u8;
    }
    idxNew = whereClauseInsert(pWC, pNew, (((2 as i32) | (1 as i32)) as i16) as u16);
    exprAnalyze(pSrc, pWC, idxNew);
}

/// Analyze a term that consists of two or more OR-connected
/// subterms.  So in:
///
///     ... WHERE  (a=5) AND (b=7 OR c=9 OR d=13) AND (d=13)
///                          ^^^^^^^^^^^^^^^^^^^^
///
/// This routine analyzes terms such as the middle term in the above example.
/// A WhereOrTerm object is computed and attached to the term under
/// analysis, regardless of the outcome of the analysis.  Hence:
///
///     WhereTerm.wtFlags   |=  TERM_ORINFO
///     WhereTerm.u.pOrInfo  =  a dynamically allocated WhereOrTerm object
///
/// The term being analyzed must have two or more of OR-connected subterms.
/// A single subterm might be a set of AND-connected sub-subterms.
/// Examples of terms under analysis:
///
///     (A)     t1.x=t2.y OR t1.x=t2.z OR t1.y=15 OR t1.z=t3.a+5
///     (B)     x=expr1 OR expr2=x OR x=expr3
///     (C)     t1.x=t2.y OR (t1.x=t2.z AND t1.y=15)
///     (D)     x=expr1 OR (y>11 AND y<22 AND z LIKE '*hello*')
///     (E)     (p.a=1 AND q.b=2 AND r.c=3) OR (p.x=4 AND q.y=5 AND r.z=6)
///     (F)     x>A OR (x=A AND y>=B)
///
/// CASE 1:
///
/// If all subterms are of the form T.C=expr for some single column of C and
/// a single table T (as shown in example B above) then create a new virtual
/// term that is an equivalent IN expression.  In other words, if the term
/// being analyzed is:
///
///      x = expr1  OR  expr2 = x  OR  x = expr3
///
/// then create a new virtual term like this:
///
///      x IN (expr1,expr2,expr3)
///
/// CASE 2:
///
/// If there are exactly two disjuncts and one side has x>A and the other side
/// has x=A (for the same x and A) then add a new virtual conjunct term to the
/// WHERE clause of the form "x>=A".  Example:
///
///      x>A OR (x=A AND y>B)    adds:    x>=A
///
/// The added conjunct can sometimes be helpful in query planning.
///
/// CASE 3:
///
/// If all subterms are indexable by a single table T, then set
///
///     WhereTerm.eOperator              =  WO_OR
///     WhereTerm.u.pOrInfo->indexable  |=  the cursor number for table T
///
/// A subterm is "indexable" if it is of the form
/// "T.C <op> <expr>" where C is any column of table T and
/// <op> is one of "=", "<", "<=", ">", ">=", "IS NULL", or "IN".
/// A subterm is also indexable if it is an AND of two or more
/// subsubterms at least one of which is indexable.  Indexable AND
/// subterms have their eOperator set to WO_AND and they have
/// u.pAndInfo set to a dynamically allocated WhereAndTerm object.
///
/// From another point of view, "indexable" means that the subterm could
/// potentially be used with an index if an appropriate index exists.
/// This analysis does not consider whether or not the index exists; that
/// is decided elsewhere.  This analysis only looks at whether subterms
/// appropriate for indexing exist.
///
/// All examples A through E above satisfy case 3.  But if a term
/// also satisfies case 1 (such as B) we know that the optimizer will
/// always prefer case 1, so in that case we pretend that case 3 is not
/// satisfied.
///
/// It might be the case that multiple tables are indexable.  For example,
/// (E) above is indexable on tables P, Q, and R.
///
/// Terms that satisfy case 3 are candidates for lookup by using
/// separate indices to find rowids for each subterm and composing
/// the union of all rowids using a RowSet object.  This is similar
/// to "bitmap indices" in other database engines.
///
/// OTHERWISE:
///
/// If none of cases 1, 2, or 3 apply, then leave the eOperator set to
/// zero.  This term is not useful for search.
///
/// # Arguments
///
/// * `pSrc` - the FROM clause
/// * `pWC` - the complete WHERE clause
/// * `idxTerm` - Index of the OR-term to be analyzed
fn exprAnalyzeOrTerm(mut pSrc: *mut SrcList, mut pWC: *mut WhereClause, mut idxTerm: i32) {
    let mut pWInfo: *mut WhereInfo = unsafe { (*pWC).pWInfo }; // WHERE clause processing context
    let mut pParse: *mut Parse = unsafe { (*pWInfo).pParse }; // Parser context
    let mut db: *mut sqlite3 = unsafe { (*pParse).db }; // Database connection
    let mut pTerm: *mut WhereTerm = unsafe { unsafe { (*pWC).a }.offset(idxTerm as isize) }; // The term to be analyzed
    let mut pExpr: *mut Expr = unsafe { (*pTerm).pExpr }; // The expression of the term
    let mut i: i32 = 0 as i32; // Loop counters
    let mut pOrWc: *mut WhereClause = unsafe { std::mem::zeroed() }; // Breakup of pTerm into subterms
    let mut pOrTerm: *mut WhereTerm = unsafe { std::mem::zeroed() }; // A Sub-term within the pOrWc
    let mut pOrInfo: *mut WhereOrInfo = unsafe { std::mem::zeroed() }; // Additional information associated with pTerm
    let mut chngToIN: u64 = 0 as u64; // Tables that might satisfy case 1
    let mut indexable: u64 = 0 as u64; // Tables that are indexable, satisfying case 2
    // Break the OR clause into its separate subterms.  The subterms are
    // stored in a WhereClause structure containing within the WhereOrInfo
    // object that is attached to the original OR clause term.
    0 as i32;
    0 as i32;
    let __v979: *mut WhereOrInfo =
        (unsafe { sqlite3DbMallocZero(db, 496 as u64) }) as *mut WhereOrInfo;
    pOrInfo = __v979;
    unsafe {
        (*pTerm).u.pOrInfo = __v979;
    }
    if pOrInfo == std::ptr::null_mut::<WhereOrInfo>() {
        return;
    }
    let __v980: *mut WhereTerm = pTerm;
    let __v981: u16 = unsafe { (*__v980).wtFlags };
    let __v982: u16 = ((((__v981 as u32) as i32) | (16 as i32)) as i16) as u16;
    unsafe {
        (*__v980).wtFlags = __v982;
    }
    pOrWc = unsafe { std::ptr::addr_of_mut!((*pOrInfo).wc) };
    unsafe {
        memset(
            (unsafe { (*pOrWc).aStatic.as_mut_ptr() as *mut WhereTerm }) as *mut (),
            0 as i32,
            448 as u64,
        )
    };
    sqlite3WhereClauseInit(pOrWc, pWInfo);
    sqlite3WhereSplit(pOrWc, pExpr, ((43 as i32) as i8) as u8);
    sqlite3WhereExprAnalyze(pSrc, pOrWc);
    if (unsafe { (*db).mallocFailed }) != (0 as u8) {
        return;
    }
    0 as i32;
    // Compute the set of tables that might satisfy cases 1 or 3.
    indexable = !(((0 as i32) as i64) as u64);
    chngToIN = !(((0 as i32) as i64) as u64);
    i = (unsafe { (*pOrWc).nTerm }) - (1 as i32);
    let __v983: *mut WhereTerm = unsafe { (*pOrWc).a };
    pOrTerm = __v983;
    '__slate_break_812: while i >= (0 as i32) && indexable != (0 as u64) {
        if (((unsafe { (*pOrTerm).eOperator }) as u32) as i32) & (511 as i32) == (0 as i32) {
            let mut pAndInfo: *mut WhereAndInfo = unsafe { std::mem::zeroed() };
            0 as i32;
            chngToIN = ((0 as i32) as i64) as u64;
            pAndInfo = (unsafe { sqlite3DbMallocRawNN(db, 488 as u64) }) as *mut WhereAndInfo;
            if pAndInfo != std::ptr::null_mut::<WhereAndInfo>() {
                let mut pAndWC: *mut WhereClause = unsafe { std::mem::zeroed() };
                let mut pAndTerm: *mut WhereTerm = unsafe { std::mem::zeroed() };
                let mut j: i32 = 0 as i32;
                let mut b: u64 = ((0 as i32) as i64) as u64;
                unsafe {
                    (*pOrTerm).u.pAndInfo = pAndInfo;
                }
                let __v988: *mut WhereTerm = pOrTerm;
                let __v989: u16 = unsafe { (*__v988).wtFlags };
                let __v990: u16 = ((((__v989 as u32) as i32) | (32 as i32)) as i16) as u16;
                unsafe {
                    (*__v988).wtFlags = __v990;
                }
                unsafe {
                    (*pOrTerm).eOperator = ((1024 as i32) as i16) as u16;
                }
                unsafe {
                    (*pOrTerm).leftCursor = -(1 as i32);
                }
                pAndWC = unsafe { std::ptr::addr_of_mut!((*pAndInfo).wc) };
                unsafe {
                    memset(
                        (unsafe { (*pAndWC).aStatic.as_mut_ptr() as *mut WhereTerm }) as *mut (),
                        0 as i32,
                        448 as u64,
                    )
                };
                sqlite3WhereClauseInit(pAndWC, unsafe { (*pWC).pWInfo });
                sqlite3WhereSplit(
                    pAndWC,
                    unsafe { (*pOrTerm).pExpr },
                    ((44 as i32) as i8) as u8,
                );
                sqlite3WhereExprAnalyze(pSrc, pAndWC);
                unsafe {
                    (*pAndWC).pOuter = pWC;
                }
                if !((unsafe { (*db).mallocFailed }) != (0 as u8)) {
                    j = 0 as i32;
                    let __v991: *mut WhereTerm = unsafe { (*pAndWC).a };
                    pAndTerm = __v991;
                    '__slate_break_813: while j < unsafe { (*pAndWC).nTerm } {
                        0 as i32;
                        if allowedOp(
                            ((unsafe { (*unsafe { (*pAndTerm).pExpr }).op }) as u32) as i32,
                        ) != (0 as i32)
                            || (((unsafe { (*pAndTerm).eOperator }) as u32) as i32) == (64 as i32)
                        {
                            let __v996: u64 = b;
                            let __v997: u64 = __v996
                                | unsafe {
                                    sqlite3WhereGetMask(
                                        unsafe { std::ptr::addr_of_mut!((*pWInfo).sMaskSet) },
                                        unsafe { (*pAndTerm).leftCursor },
                                    )
                                };
                            b = __v997;
                        }
                        let __v992: i32 = j;
                        let __v993: i32 = __v992 + (1 as i32);
                        j = __v993;
                        let __v994: *mut WhereTerm = pAndTerm;
                        let __v995: *mut WhereTerm = unsafe { __v994.offset((1 as i32) as isize) };
                        pAndTerm = __v995;
                    }
                }
                let __v998: u64 = indexable;
                let __v999: u64 = __v998 & b;
                indexable = __v999;
            }
        } else {
            if (((unsafe { (*pOrTerm).wtFlags }) as u32) as i32) & (8 as i32) != (0 as i32) {
                // Skip this term for now.  We revisit it when we process the
                // corresponding TERM_VIRTUAL term
            } else {
                let mut b: u64 = 0 as u64;
                b = unsafe {
                    sqlite3WhereGetMask(
                        unsafe { std::ptr::addr_of_mut!((*pWInfo).sMaskSet) },
                        unsafe { (*pOrTerm).leftCursor },
                    )
                };
                if (((unsafe { (*pOrTerm).wtFlags }) as u32) as i32) & (2 as i32) != (0 as i32) {
                    let mut pOther: *mut WhereTerm = unsafe {
                        unsafe { (*pOrWc).a }.offset((unsafe { (*pOrTerm).iParent }) as isize)
                    };
                    let __v1000: u64 = b;
                    let __v1001: u64 = __v1000
                        | unsafe {
                            sqlite3WhereGetMask(
                                unsafe { std::ptr::addr_of_mut!((*pWInfo).sMaskSet) },
                                unsafe { (*pOther).leftCursor },
                            )
                        };
                    b = __v1001;
                }
                let __v1002: u64 = indexable;
                let __v1003: u64 = __v1002 & b;
                indexable = __v1003;
                if (((unsafe { (*pOrTerm).eOperator }) as u32) as i32) & (2 as i32) == (0 as i32) {
                    chngToIN = ((0 as i32) as i64) as u64;
                } else {
                    let __v1004: u64 = chngToIN;
                    let __v1005: u64 = __v1004 & b;
                    chngToIN = __v1005;
                }
            }
        }
        let __v984: i32 = i;
        let __v985: i32 = __v984 - (1 as i32);
        i = __v985;
        let __v986: *mut WhereTerm = pOrTerm;
        let __v987: *mut WhereTerm = unsafe { __v986.offset((1 as i32) as isize) };
        pOrTerm = __v987;
    }
    // Record the set of tables that satisfy case 3.  The set might be
    // empty.
    unsafe {
        (*pOrInfo).indexable = indexable;
    }
    unsafe {
        (*pTerm).eOperator = ((512 as i32) as i16) as u16;
    }
    unsafe {
        (*pTerm).leftCursor = -(1 as i32);
    }
    if indexable != (0 as u64) {
        unsafe {
            (*pWC).hasOr = ((1 as i32) as i8) as u8;
        }
    }
    // For a two-way OR, attempt to implementation case 2.
    if indexable != (0 as u64) && (unsafe { (*pOrWc).nTerm }) == (2 as i32) {
        let mut iOne: i32 = 0 as i32;
        let mut pOne: *mut WhereTerm = unsafe { std::mem::zeroed() };
        '__slate_break_814: loop {
            let __v1006: i32 = iOne;
            let __v1007: i32 = __v1006 + (1 as i32);
            iOne = __v1007;
            let __v1008: *mut WhereTerm = whereNthSubterm(
                unsafe { unsafe { (*pOrWc).a }.offset((0 as i32) as isize) },
                __v1006,
            );
            pOne = __v1008;
            if !(__v1008 != std::ptr::null_mut::<WhereTerm>()) {
                break;
            }
            let mut iTwo: i32 = 0 as i32;
            let mut pTwo: *mut WhereTerm = unsafe { std::mem::zeroed() };
            '__slate_break_815: loop {
                let __v1009: i32 = iTwo;
                let __v1010: i32 = __v1009 + (1 as i32);
                iTwo = __v1010;
                let __v1011: *mut WhereTerm = whereNthSubterm(
                    unsafe { unsafe { (*pOrWc).a }.offset((1 as i32) as isize) },
                    __v1009,
                );
                pTwo = __v1011;
                if !(__v1011 != std::ptr::null_mut::<WhereTerm>()) {
                    break;
                }
                whereCombineDisjuncts(pSrc, pWC, pOne, pTwo);
            }
        }
    }
    // chngToIN holds a set of tables that *might* satisfy case 1.  But
    // we have to do some additional checking to see if case 1 really
    // is satisfied.
    //
    // chngToIN will hold either 0, 1, or 2 bits.  The 0-bit case means
    // that there is no possibility of transforming the OR clause into an
    // IN operator because one or more terms in the OR clause contain
    // something other than == on a column in the single table.  The 1-bit
    // case means that every term of the OR clause is of the form
    // "table.column=expr" for some single table.  The one bit that is set
    // will correspond to the common table.  We still need to check to make
    // sure the same column is used on all terms.  The 2-bit case is when
    // the all terms are of the form "table1.column=table2.column".  It
    // might be possible to form an IN operator with either table1.column
    // or table2.column as the LHS if either is common to every term of
    // the OR clause.
    //
    // Note that terms of the form "table.column1=table.column2" (the
    // same table on both sizes of the ==) cannot be optimized.
    if chngToIN != (0 as u64) {
        let mut okToChngToIN: i32 = 0 as i32; // True if the conversion to IN is valid
        let mut iColumn: i32 = -(1 as i32); // Column index on lhs of IN operator
        let mut iCursor: i32 = -(1 as i32); // Table cursor common to all terms
        let mut j: i32 = 0 as i32; // Loop counter
        // Search for a table and column that appears on one side or the
        // other of the == operator in every subterm.  That table and column
        // will be recorded in iCursor and iColumn.  There might not be any
        // such table and column.  Set okToChngToIN if an appropriate table
        // and column is found but leave okToChngToIN false if not found.
        j = 0 as i32;
        '__slate_break_816: loop {
            if !(j < (2 as i32) && !(okToChngToIN != (0 as i32))) {
                break;
            }
            let mut pLeft: *mut Expr = std::ptr::null_mut::<Expr>();
            pOrTerm = unsafe { (*pOrWc).a };
            i = (unsafe { (*pOrWc).nTerm }) - (1 as i32);
            '__slate_break_817: while i >= (0 as i32) {
                0 as i32;
                let __v1018: *mut WhereTerm = pOrTerm;
                let __v1019: u16 = unsafe { (*__v1018).wtFlags };
                let __v1020: u16 = ((((__v1019 as u32) as i32) & !(64 as i32)) as i16) as u16;
                unsafe {
                    (*__v1018).wtFlags = __v1020;
                }
                if (unsafe { (*pOrTerm).leftCursor }) == iCursor {
                    // This is the 2-bit case and we are on the second iteration and
                    // current term is from the first iteration.  So skip this term.
                    0 as i32;
                } else {
                    if chngToIN
                        & unsafe {
                            sqlite3WhereGetMask(
                                unsafe { std::ptr::addr_of_mut!((*pWInfo).sMaskSet) },
                                unsafe { (*pOrTerm).leftCursor },
                            )
                        }
                        == (((0 as i32) as i64) as u64)
                    {
                        // This term must be of the form t1.a==t2.b where t2 is in the
                        // chngToIN set but t1 is not.  This term will be either preceded
                        // or followed by an inverted copy (t2.b==t1.a).  Skip this term
                        // and use its inversion.
                        {}
                        {}
                        0 as i32;
                    } else {
                        0 as i32;
                        iColumn = unsafe { (*pOrTerm).u.x.leftColumn };
                        iCursor = unsafe { (*pOrTerm).leftCursor };
                        pLeft = unsafe { (*unsafe { (*pOrTerm).pExpr }).pLeft };
                        break '__slate_break_817;
                    }
                }
                let __v1014: i32 = i;
                let __v1015: i32 = __v1014 - (1 as i32);
                i = __v1015;
                let __v1016: *mut WhereTerm = pOrTerm;
                let __v1017: *mut WhereTerm = unsafe { __v1016.offset((1 as i32) as isize) };
                pOrTerm = __v1017;
            }
            if i < (0 as i32) {
                // No candidate table+column was found.  This can only occur
                // on the second iteration
                0 as i32;
                0 as i32;
                0 as i32;
                break '__slate_break_816;
            }
            {}
            // We have found a candidate table and column.  Check to see if that
            // table and column is common to every term in the OR clause
            okToChngToIN = 1 as i32;
            '__slate_break_818: while i >= (0 as i32) && okToChngToIN != (0 as i32) {
                0 as i32;
                0 as i32;
                if (unsafe { (*pOrTerm).leftCursor }) != iCursor {
                    let __v1025: *mut WhereTerm = pOrTerm;
                    let __v1026: u16 = unsafe { (*__v1025).wtFlags };
                    let __v1027: u16 = ((((__v1026 as u32) as i32) & !(64 as i32)) as i16) as u16;
                    unsafe {
                        (*__v1025).wtFlags = __v1027;
                    }
                } else {
                    let __v1028: bool;
                    if (unsafe { (*pOrTerm).u.x.leftColumn }) != iColumn {
                        __v1028 = true as bool;
                    } else {
                        let __v1029: bool;
                        if iColumn == -(2 as i32) {
                            __v1029 = (unsafe {
                                sqlite3ExprCompare(
                                    pParse as *const Parse,
                                    (unsafe { (*unsafe { (*pOrTerm).pExpr }).pLeft })
                                        as *const Expr,
                                    pLeft as *const Expr,
                                    -(1 as i32),
                                )
                            }) != (0 as i32);
                        } else {
                            __v1029 = false as bool;
                        }
                        __v1028 = __v1029;
                    }
                    if __v1028 {
                        okToChngToIN = 0 as i32;
                    } else {
                        let mut affLeft: i32 = 0 as i32;
                        let mut affRight: i32 = 0 as i32;
                        // If the right-hand side is also a column, then the affinities
                        // of both right and left sides must be such that no type
                        // conversions are required on the right.  (Ticket #2249)
                        affRight = (unsafe {
                            sqlite3ExprAffinity(
                                (unsafe { (*unsafe { (*pOrTerm).pExpr }).pRight }) as *const Expr,
                            )
                        }) as i32;
                        affLeft = (unsafe {
                            sqlite3ExprAffinity(
                                (unsafe { (*unsafe { (*pOrTerm).pExpr }).pLeft }) as *const Expr,
                            )
                        }) as i32;
                        if affRight != (0 as i32) && affRight != affLeft {
                            okToChngToIN = 0 as i32;
                        } else {
                            let __v1030: *mut WhereTerm = pOrTerm;
                            let __v1031: u16 = unsafe { (*__v1030).wtFlags };
                            let __v1032: u16 =
                                ((((__v1031 as u32) as i32) | (64 as i32)) as i16) as u16;
                            unsafe {
                                (*__v1030).wtFlags = __v1032;
                            }
                        }
                    }
                }
                let __v1021: i32 = i;
                let __v1022: i32 = __v1021 - (1 as i32);
                i = __v1022;
                let __v1023: *mut WhereTerm = pOrTerm;
                let __v1024: *mut WhereTerm = unsafe { __v1023.offset((1 as i32) as isize) };
                pOrTerm = __v1024;
            }
            let __v1012: i32 = j;
            let __v1013: i32 = __v1012 + (1 as i32);
            j = __v1013;
        }
        // At this point, okToChngToIN is true if original pTerm is a
        // candidate to satisfy case 1, though we are not yet certain that
        // the collating sequences are all compatible.  Try to construct a
        // new virtual term that is pTerm converted from an OR operator
        // into an IN operator.
        //
        // During construction, verify that the collating sequences on all
        // subterms of the OR are compatible.  Omit the construction of the
        // new IN operator if there are any collating sequence mismatches.
        if okToChngToIN != (0 as i32) {
            let mut pDup: *mut Expr = unsafe { std::mem::zeroed() }; // A transient duplicate expression
            let mut pList: *mut ExprList = std::ptr::null_mut::<ExprList>(); // The RHS of the IN operator
            let mut pLeft: *mut Expr = std::ptr::null_mut::<Expr>(); // The LHS of the IN operator
            let mut pCollSeq: *mut CollSeq = std::ptr::null_mut::<CollSeq>(); // Collating sequence to use
            let mut pNew: *mut Expr = unsafe { std::mem::zeroed() }; // The complete IN operator
            i = (unsafe { (*pOrWc).nTerm }) - (1 as i32);
            let __v1033: *mut WhereTerm = unsafe { (*pOrWc).a };
            pOrTerm = __v1033;
            '__slate_break_819: while i >= (0 as i32) {
                let mut pThis: *mut Expr = unsafe { std::mem::zeroed() };
                if (((unsafe { (*pOrTerm).wtFlags }) as u32) as i32) & (64 as i32) == (0 as i32) {
                } else {
                    0 as i32;
                    0 as i32;
                    0 as i32;
                    0 as i32;
                    pThis = unsafe { (*pOrTerm).pExpr };
                    pDup = unsafe {
                        sqlite3ExprDup(db, (unsafe { (*pThis).pRight }) as *const Expr, 0 as i32)
                    };
                    pList =
                        unsafe { sqlite3ExprListAppend(unsafe { (*pWInfo).pParse }, pList, pDup) };
                    if pLeft == std::ptr::null_mut::<Expr>() {
                        pLeft = unsafe { (*pThis).pLeft };
                        pCollSeq =
                            unsafe { sqlite3ExprCompareCollSeq(pParse, pThis as *const Expr) };
                    } else {
                        0 as i32;
                        if pCollSeq
                            != unsafe { sqlite3ExprCompareCollSeq(pParse, pThis as *const Expr) }
                        {
                            pLeft = std::ptr::null_mut::<Expr>(); // Collating sequence mismatch
                            break '__slate_break_819;
                        }
                    }
                }
                let __v1034: i32 = i;
                let __v1035: i32 = __v1034 - (1 as i32);
                i = __v1035;
                let __v1036: *mut WhereTerm = pOrTerm;
                let __v1037: *mut WhereTerm = unsafe { __v1036.offset((1 as i32) as isize) };
                pOrTerm = __v1037;
            }
            if pLeft == std::ptr::null_mut::<Expr>() {
                pNew = std::ptr::null_mut::<Expr>(); // Collating sequence mismatch
            } else {
                pDup = unsafe { sqlite3ExprDup(db, pLeft as *const Expr, 0 as i32) };
                if (unsafe { sqlite3ExprCollSeq(pParse, pDup as *const Expr) }) != pCollSeq
                    && pCollSeq != std::ptr::null_mut::<CollSeq>()
                {
                    0 as i32;
                    pDup = unsafe {
                        sqlite3ExprAddCollateString(
                            pParse as *const Parse,
                            pDup,
                            (unsafe { (*pCollSeq).zName }) as *const i8,
                        )
                    };
                }
                pNew =
                    unsafe { sqlite3PExpr(pParse, 50 as i32, pDup, std::ptr::null_mut::<Expr>()) };
            }
            if pNew != std::ptr::null_mut::<Expr>() {
                let mut idxNew: i32 = 0 as i32;
                transferJoinMarkings(pNew, pExpr);
                0 as i32;
                unsafe {
                    (*pNew).x.pList = pList;
                }
                idxNew = whereClauseInsert(pWC, pNew, (((2 as i32) | (1 as i32)) as i16) as u16);
                {}
                exprAnalyze(pSrc, pWC, idxNew);
                // pTerm = &pWC->a[idxTerm]; // would be needed if pTerm where reused
                markTermAsChild(pWC, idxNew, idxTerm);
            } else {
                unsafe { sqlite3ExprListDelete(db, pList) };
            }
        }
    }
}

/// We already know that pExpr is a binary operator where both operands are
/// column references.  This routine checks to see if pExpr is an equivalence
/// relation:
///   1.  The SQLITE_Transitive optimization must be enabled
///   2.  Must be either an == or an IS operator
///   3.  Not originating in the ON clause of an OUTER JOIN
///   4.  The operator is not IS or else the query does not contain RIGHT JOIN
///   5.  The affinities of A and B must be compatible
///   6.  Both operands use the same collating sequence, and they must not
///       use explicit COLLATE clauses.
/// If this routine returns TRUE, that means that the RHS can be substituted
/// for the LHS anyplace else in the WHERE clause where the LHS column occurs.
/// This is an optimization.  No harm comes from returning 0.  But if 1 is
/// returned when it should not be, then incorrect answers might result.
fn termIsEquivalence(mut pParse: *mut Parse, mut pExpr: *mut Expr, mut pSrc: *mut SrcList) -> i32 {
    let mut aff1: i8 = 0 as i8;
    let mut aff2: i8 = 0 as i8;
    if !((unsafe { (*unsafe { (*pParse).db }).dbOptFlags }) & ((128 as i32) as u32)
        == ((0 as i32) as u32))
    {
        return 0 as i32;
    }
    // (1)
    if (((unsafe { (*pExpr).op }) as u32) as i32) != (54 as i32)
        && (((unsafe { (*pExpr).op }) as u32) as i32) != (45 as i32)
    {
        return 0 as i32;
    }
    // (2)
    if (unsafe { (*pExpr).flags }) & (((1 as i32) | (512 as i32)) as u32) != ((0 as i32) as u32) {
        return 0 as i32;
    }
    // (3)
    0 as i32;
    if (((unsafe { (*pExpr).op }) as u32) as i32) == (45 as i32)
        && (unsafe { (*pSrc).nSrc }) >= (2 as i32)
        && (((unsafe {
            (*unsafe {
                unsafe { std::ptr::addr_of_mut!((*pSrc).a) as *mut SrcItem }
                    .offset((0 as i32) as isize)
            })
            .fg
            .jointype
        }) as u32) as i32)
            & (64 as i32)
            != (0 as i32)
    {
        return 0 as i32; // (4)
    }
    aff1 = unsafe { sqlite3ExprAffinity((unsafe { (*pExpr).pLeft }) as *const Expr) };
    aff2 = unsafe { sqlite3ExprAffinity((unsafe { (*pExpr).pRight }) as *const Expr) };
    if (aff1 as i32) != (aff2 as i32)
        && (!((aff1 as i32) >= (67 as i32)) || !((aff2 as i32) >= (67 as i32)))
    {
        return 0 as i32; // (5)
    }
    if !((unsafe {
        sqlite3ExprCollSeqMatch(
            pParse,
            (unsafe { (*pExpr).pLeft }) as *const Expr,
            (unsafe { (*pExpr).pRight }) as *const Expr,
        )
    }) != (0 as i32))
    {
        return 0 as i32; // (6)
    }
    return 1 as i32;
}

/// Recursively walk the expressions of a SELECT statement and generate
/// a bitmask indicating which tables are used in that expression
/// tree.
fn exprSelectUsage(mut pMaskSet: *mut WhereMaskSet, mut pS: *mut Select) -> u64 {
    let mut mask: u64 = ((0 as i32) as i64) as u64;
    '__slate_break_820: while pS != std::ptr::null_mut::<Select>() {
        let mut pSrc: *mut SrcList = unsafe { (*pS).pSrc };
        let __v1038: u64 = mask;
        let __v1039: u64 = __v1038 | sqlite3WhereExprListUsage(pMaskSet, unsafe { (*pS).pEList });
        mask = __v1039;
        let __v1040: u64 = mask;
        let __v1041: u64 = __v1040 | sqlite3WhereExprListUsage(pMaskSet, unsafe { (*pS).pGroupBy });
        mask = __v1041;
        let __v1042: u64 = mask;
        let __v1043: u64 = __v1042 | sqlite3WhereExprListUsage(pMaskSet, unsafe { (*pS).pOrderBy });
        mask = __v1043;
        let __v1044: u64 = mask;
        let __v1045: u64 = __v1044 | sqlite3WhereExprUsage(pMaskSet, unsafe { (*pS).pWhere });
        mask = __v1045;
        let __v1046: u64 = mask;
        let __v1047: u64 = __v1046 | sqlite3WhereExprUsage(pMaskSet, unsafe { (*pS).pHaving });
        mask = __v1047;
        if pSrc != std::ptr::null_mut::<SrcList>() {
            let mut i: i32 = 0 as i32;
            i = 0 as i32;
            '__slate_break_821: loop {
                if !(i < unsafe { (*pSrc).nSrc }) {
                    break;
                }
                if ((unsafe {
                    (*unsafe {
                        unsafe { std::ptr::addr_of_mut!((*pSrc).a) as *mut SrcItem }
                            .offset(i as isize)
                    })
                    .fg
                    .__slate_bits_0
                    .__get_isSubquery()
                }) as i32)
                    != (0 as i32)
                {
                    let __v1050: u64 = mask;
                    let __v1051: u64 = __v1050
                        | exprSelectUsage(pMaskSet, unsafe {
                            (*unsafe {
                                (*unsafe {
                                    unsafe { std::ptr::addr_of_mut!((*pSrc).a) as *mut SrcItem }
                                        .offset(i as isize)
                                })
                                .u4
                                .pSubq
                            })
                            .pSelect
                        });
                    mask = __v1051;
                }
                if ((unsafe {
                    (*unsafe {
                        unsafe { std::ptr::addr_of_mut!((*pSrc).a) as *mut SrcItem }
                            .offset(i as isize)
                    })
                    .fg
                    .__slate_bits_0
                    .__get_isUsing()
                }) as i32)
                    == (0 as i32)
                {
                    let __v1052: u64 = mask;
                    let __v1053: u64 = __v1052
                        | sqlite3WhereExprUsage(pMaskSet, unsafe {
                            (*unsafe {
                                unsafe { std::ptr::addr_of_mut!((*pSrc).a) as *mut SrcItem }
                                    .offset(i as isize)
                            })
                            .u3
                            .pOn
                        });
                    mask = __v1053;
                }
                if ((unsafe {
                    (*unsafe {
                        unsafe { std::ptr::addr_of_mut!((*pSrc).a) as *mut SrcItem }
                            .offset(i as isize)
                    })
                    .fg
                    .__slate_bits_0
                    .__get_isTabFunc()
                }) as i32)
                    != (0 as i32)
                {
                    let __v1054: u64 = mask;
                    let __v1055: u64 = __v1054
                        | sqlite3WhereExprListUsage(pMaskSet, unsafe {
                            (*unsafe {
                                unsafe { std::ptr::addr_of_mut!((*pSrc).a) as *mut SrcItem }
                                    .offset(i as isize)
                            })
                            .u1
                            .pFuncArg
                        });
                    mask = __v1055;
                }
                let __v1048: i32 = i;
                let __v1049: i32 = __v1048 + (1 as i32);
                i = __v1049;
            }
        }
        pS = unsafe { (*pS).pPrior };
    }
    return mask;
}

/// Expression pExpr is one operand of a comparison operator that might
/// be useful for indexing.  This routine checks to see if pExpr appears
/// in any index.  Return TRUE (1) if pExpr is an indexed term and return
/// FALSE (0) if not.  If TRUE is returned, also set aiCurCol[0] to the cursor
/// number of the table that is indexed and aiCurCol[1] to the column number
/// of the column that is indexed, or XN_EXPR (-2) if an expression is being
/// indexed.
///
/// If pExpr is a TK_COLUMN column reference, then this routine always returns
/// true even if that particular column is not indexed, because the column
/// might be added to an automatic index later.
///
/// # Arguments
///
/// * `pFrom` - The FROM clause
/// * `aiCurCol` - Write the referenced table cursor and column here
/// * `pExpr` - An operand of a comparison operator
/// * `j` - Start looking with the j-th pFrom entry
fn exprMightBeIndexed2(
    mut pFrom: *mut SrcList,
    mut aiCurCol: *mut i32,
    mut pExpr: *mut Expr,
    mut j: i32,
) -> i32 {
    let mut pIdx: *mut Index = unsafe { std::mem::zeroed() };
    let mut i: i32 = 0 as i32;
    let mut iCur: i32 = 0 as i32;
    '__slate_break_822: loop {
        iCur = unsafe {
            (*unsafe {
                unsafe { std::ptr::addr_of_mut!((*pFrom).a) as *mut SrcItem }.offset(j as isize)
            })
            .iCursor
        };
        pIdx = unsafe {
            (*unsafe {
                (*unsafe {
                    unsafe { std::ptr::addr_of_mut!((*pFrom).a) as *mut SrcItem }.offset(j as isize)
                })
                .pSTab
            })
            .pIndex
        };
        '__slate_break_823: while pIdx != std::ptr::null_mut::<Index>() {
            if (unsafe { (*pIdx).aColExpr }) == std::ptr::null_mut::<ExprList>() {
            } else {
                i = 0 as i32;
                '__slate_break_824: loop {
                    if !(i < (((unsafe { (*pIdx).nKeyCol }) as u32) as i32)) {
                        break;
                    }
                    if ((unsafe { *unsafe { unsafe { (*pIdx).aiColumn }.offset(i as isize) } })
                        as i32)
                        != -(2 as i32)
                    {
                    } else {
                        0 as i32;
                        let __v1058: bool;
                        if (unsafe {
                            sqlite3ExprCompareSkip(
                                pExpr,
                                unsafe {
                                    (*unsafe {
                                        unsafe {
                                            std::ptr::addr_of_mut!((*unsafe { (*pIdx).aColExpr }).a)
                                                as *mut ExprList_item
                                        }
                                        .offset(i as isize)
                                    })
                                    .pExpr
                                },
                                iCur,
                            )
                        }) == (0 as i32)
                        {
                            __v1058 = !((unsafe {
                                sqlite3ExprIsConstant(std::ptr::null_mut::<Parse>(), unsafe {
                                    (*unsafe {
                                        unsafe {
                                            std::ptr::addr_of_mut!((*unsafe { (*pIdx).aColExpr }).a)
                                                as *mut ExprList_item
                                        }
                                        .offset(i as isize)
                                    })
                                    .pExpr
                                })
                            }) != (0 as i32));
                        } else {
                            __v1058 = false as bool;
                        }
                        if __v1058 {
                            unsafe {
                                *unsafe { aiCurCol.offset((0 as i32) as isize) } = iCur;
                            }
                            unsafe {
                                *unsafe { aiCurCol.offset((1 as i32) as isize) } = -(2 as i32);
                            }
                            return 1 as i32;
                        }
                    }
                    let __v1056: i32 = i;
                    let __v1057: i32 = __v1056 + (1 as i32);
                    i = __v1057;
                }
            }
            pIdx = unsafe { (*pIdx).pNext };
        }
        let __v1059: i32 = j;
        let __v1060: i32 = __v1059 + (1 as i32);
        j = __v1060;
        if !(__v1060 < unsafe { (*pFrom).nSrc }) {
            break;
        }
    }
    return 0 as i32;
}

/// # Arguments
///
/// * `pFrom` - The FROM clause
/// * `aiCurCol` - Write the referenced table cursor & column here
/// * `pExpr` - An operand of a comparison operator
/// * `op` - The specific comparison operator
fn exprMightBeIndexed(
    mut pFrom: *mut SrcList,
    mut aiCurCol: *mut i32,
    mut pExpr: *mut Expr,
    mut op: i32,
) -> i32 {
    let mut i: i32 = 0 as i32;
    // If this expression is a vector to the left or right of a
    // inequality constraint (>, <, >= or <=), perform the processing
    // on the first element of the vector.
    0 as i32;
    0 as i32;
    0 as i32;
    if (((unsafe { (*pExpr).op }) as u32) as i32) == (177 as i32)
        && (op >= (55 as i32) && op <= (58 as i32))
    {
        0 as i32;
        pExpr = unsafe {
            (*unsafe {
                unsafe {
                    std::ptr::addr_of_mut!((*unsafe { (*pExpr).x.pList }).a) as *mut ExprList_item
                }
                .offset((0 as i32) as isize)
            })
            .pExpr
        };
    }
    if (((unsafe { (*pExpr).op }) as u32) as i32) == (168 as i32) {
        unsafe {
            *unsafe { aiCurCol.offset((0 as i32) as isize) } = unsafe { (*pExpr).iTable };
        }
        unsafe {
            *unsafe { aiCurCol.offset((1 as i32) as isize) } = (unsafe { (*pExpr).iColumn }) as i32;
        }
        return 1 as i32;
    }
    i = 0 as i32;
    '__slate_break_825: loop {
        if !(i < unsafe { (*pFrom).nSrc }) {
            break;
        }
        let mut pIdx: *mut Index = unsafe { std::mem::zeroed() };
        pIdx = unsafe {
            (*unsafe {
                (*unsafe {
                    unsafe { std::ptr::addr_of_mut!((*pFrom).a) as *mut SrcItem }.offset(i as isize)
                })
                .pSTab
            })
            .pIndex
        };
        '__slate_break_826: while pIdx != std::ptr::null_mut::<Index>() {
            if (unsafe { (*pIdx).aColExpr }) != std::ptr::null_mut::<ExprList>() {
                return exprMightBeIndexed2(pFrom, aiCurCol, pExpr, i);
            }
            pIdx = unsafe { (*pIdx).pNext };
        }
        let __v1061: i32 = i;
        let __v1062: i32 = __v1061 + (1 as i32);
        i = __v1062;
    }
    return 0 as i32;
}

/// The input to this routine is an WhereTerm structure with only the
/// "pExpr" field filled in.  The job of this routine is to analyze the
/// subexpression and populate all the other fields of the WhereTerm
/// structure.
///
/// If the expression is of the form "<expr> <op> X" it gets commuted
/// to the standard form of "X <op> <expr>".
///
/// If the expression is of the form "X <op> Y" where both X and Y are
/// columns, then the original expression is unchanged and a new virtual
/// term of the form "Y <op> X" is added to the WHERE clause and
/// analyzed separately.  The original term is marked with TERM_COPIED
/// and the new term is marked with TERM_DYNAMIC (because it's pExpr
/// needs to be freed with the WhereClause) and TERM_VIRTUAL (because it
/// is a commuted copy of a prior term.)  The original term has nChild=1
/// and the copy has idxParent set to the index of the original term.
///
/// # Arguments
///
/// * `pSrc` - the FROM clause
/// * `pWC` - the WHERE clause
/// * `idxTerm` - Index of the term to be analyzed
fn exprAnalyze(mut pSrc: *mut SrcList, mut pWC: *mut WhereClause, mut idxTerm: i32) {
    let mut __slate_storage_934: std::mem::MaybeUninit<u64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_934: *mut u64 = std::ptr::addr_of_mut!(__slate_storage_934) as *mut u64;
    let mut __slate_storage_933: std::mem::MaybeUninit<u64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_933: *mut u64 = std::ptr::addr_of_mut!(__slate_storage_933) as *mut u64;
    let mut __slate_storage_932: std::mem::MaybeUninit<*mut WhereTerm> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_932: *mut *mut WhereTerm =
        std::ptr::addr_of_mut!(__slate_storage_932) as *mut *mut WhereTerm;
    let mut __slate_storage_921: std::mem::MaybeUninit<u16> = std::mem::MaybeUninit::uninit();
    let __slate_slot_921: *mut u16 = std::ptr::addr_of_mut!(__slate_storage_921) as *mut u16;
    let mut __slate_storage_920: std::mem::MaybeUninit<u16> = std::mem::MaybeUninit::uninit();
    let __slate_slot_920: *mut u16 = std::ptr::addr_of_mut!(__slate_storage_920) as *mut u16;
    let mut __slate_storage_919: std::mem::MaybeUninit<*mut WhereTerm> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_919: *mut *mut WhereTerm =
        std::ptr::addr_of_mut!(__slate_storage_919) as *mut *mut WhereTerm;
    let mut __slate_storage_918: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_918: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_918) as *mut i32;
    let mut __slate_storage_917: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_917: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_917) as *mut i32;
    let mut __slate_storage_593: std::mem::MaybeUninit<*mut Expr> = std::mem::MaybeUninit::uninit();
    let __slate_slot_593: *mut *mut Expr =
        std::ptr::addr_of_mut!(__slate_storage_593) as *mut *mut Expr;
    let mut __slate_storage_592: std::mem::MaybeUninit<*mut Expr> = std::mem::MaybeUninit::uninit();
    let __slate_slot_592: *mut *mut Expr =
        std::ptr::addr_of_mut!(__slate_storage_592) as *mut *mut Expr;
    let mut __slate_storage_591: std::mem::MaybeUninit<*mut Expr> = std::mem::MaybeUninit::uninit();
    let __slate_slot_591: *mut *mut Expr =
        std::ptr::addr_of_mut!(__slate_storage_591) as *mut *mut Expr;
    let mut __slate_storage_590: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_590: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_590) as *mut i32;
    let mut __slate_storage_589: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_589: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_589) as *mut i32;
    let mut __slate_storage_923: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_923: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_923) as *mut i32;
    let mut __slate_storage_922: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_922: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_922) as *mut i32;
    let mut __slate_storage_595: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_595: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_595) as *mut i32;
    let mut __slate_storage_594: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_594: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_594) as *mut i32;
    let mut __slate_storage_925: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_925: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_925) as *mut i32;
    let mut __slate_storage_924: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_924: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_924) as *mut i32;
    let mut __slate_storage_604: std::mem::MaybeUninit<*mut Expr> = std::mem::MaybeUninit::uninit();
    let __slate_slot_604: *mut *mut Expr =
        std::ptr::addr_of_mut!(__slate_storage_604) as *mut *mut Expr;
    let mut __slate_storage_931: std::mem::MaybeUninit<u16> = std::mem::MaybeUninit::uninit();
    let __slate_slot_931: *mut u16 = std::ptr::addr_of_mut!(__slate_storage_931) as *mut u16;
    let mut __slate_storage_930: std::mem::MaybeUninit<u16> = std::mem::MaybeUninit::uninit();
    let __slate_slot_930: *mut u16 = std::ptr::addr_of_mut!(__slate_storage_930) as *mut u16;
    let mut __slate_storage_929: std::mem::MaybeUninit<*mut WhereTerm> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_929: *mut *mut WhereTerm =
        std::ptr::addr_of_mut!(__slate_storage_929) as *mut *mut WhereTerm;
    let mut __slate_storage_928: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_928: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_928) as *mut u32;
    let mut __slate_storage_927: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_927: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_927) as *mut u32;
    let mut __slate_storage_926: std::mem::MaybeUninit<*mut Expr> = std::mem::MaybeUninit::uninit();
    let __slate_slot_926: *mut *mut Expr =
        std::ptr::addr_of_mut!(__slate_storage_926) as *mut *mut Expr;
    let mut __slate_storage_603: std::mem::MaybeUninit<*mut Expr> = std::mem::MaybeUninit::uninit();
    let __slate_slot_603: *mut *mut Expr =
        std::ptr::addr_of_mut!(__slate_storage_603) as *mut *mut Expr;
    let mut __slate_storage_602: std::mem::MaybeUninit<u64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_602: *mut u64 = std::ptr::addr_of_mut!(__slate_storage_602) as *mut u64;
    let mut __slate_storage_601: std::mem::MaybeUninit<u64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_601: *mut u64 = std::ptr::addr_of_mut!(__slate_storage_601) as *mut u64;
    let mut __slate_storage_600: std::mem::MaybeUninit<*mut WhereTerm> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_600: *mut *mut WhereTerm =
        std::ptr::addr_of_mut!(__slate_storage_600) as *mut *mut WhereTerm;
    let mut __slate_storage_599: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_599: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_599) as *mut i32;
    let mut __slate_storage_598: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_598: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_598) as *mut i32;
    let mut __slate_storage_597: std::mem::MaybeUninit<*mut Expr> = std::mem::MaybeUninit::uninit();
    let __slate_slot_597: *mut *mut Expr =
        std::ptr::addr_of_mut!(__slate_storage_597) as *mut *mut Expr;
    let mut __slate_storage_596: std::mem::MaybeUninit<*mut Expr> = std::mem::MaybeUninit::uninit();
    let __slate_slot_596: *mut *mut Expr =
        std::ptr::addr_of_mut!(__slate_storage_596) as *mut *mut Expr;
    let mut __slate_storage_916: std::mem::MaybeUninit<bool> = std::mem::MaybeUninit::uninit();
    let __slate_slot_916: *mut bool = std::ptr::addr_of_mut!(__slate_storage_916) as *mut bool;
    let mut __slate_storage_915: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_915: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_915) as *mut i32;
    // If there is a vector == or IS term - e.g. "(a, b) == (?, ?)" - create
    // new terms for each component comparison - "a = ?" and "b = ?".  The
    // new terms completely replace the original vector comparison, which is
    // no longer used.
    //
    // This is only required if at least one side of the comparison operation
    // is not a sub-select.
    //
    // tag-20220128a
    let mut __slate_storage_914: std::mem::MaybeUninit<bool> = std::mem::MaybeUninit::uninit();
    let __slate_slot_914: *mut bool = std::ptr::addr_of_mut!(__slate_storage_914) as *mut bool;
    let mut __slate_storage_891: std::mem::MaybeUninit<u16> = std::mem::MaybeUninit::uninit();
    let __slate_slot_891: *mut u16 = std::ptr::addr_of_mut!(__slate_storage_891) as *mut u16;
    let mut __slate_storage_890: std::mem::MaybeUninit<u16> = std::mem::MaybeUninit::uninit();
    let __slate_slot_890: *mut u16 = std::ptr::addr_of_mut!(__slate_storage_890) as *mut u16;
    let mut __slate_storage_889: std::mem::MaybeUninit<*mut WhereTerm> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_889: *mut *mut WhereTerm =
        std::ptr::addr_of_mut!(__slate_storage_889) as *mut *mut WhereTerm;
    let mut __slate_storage_888: std::mem::MaybeUninit<u16> = std::mem::MaybeUninit::uninit();
    let __slate_slot_888: *mut u16 = std::ptr::addr_of_mut!(__slate_storage_888) as *mut u16;
    let mut __slate_storage_887: std::mem::MaybeUninit<u16> = std::mem::MaybeUninit::uninit();
    let __slate_slot_887: *mut u16 = std::ptr::addr_of_mut!(__slate_storage_887) as *mut u16;
    let mut __slate_storage_886: std::mem::MaybeUninit<*mut WhereTerm> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_886: *mut *mut WhereTerm =
        std::ptr::addr_of_mut!(__slate_storage_886) as *mut *mut WhereTerm;
    let mut __slate_storage_885: std::mem::MaybeUninit<u16> = std::mem::MaybeUninit::uninit();
    let __slate_slot_885: *mut u16 = std::ptr::addr_of_mut!(__slate_storage_885) as *mut u16;
    let mut __slate_storage_884: std::mem::MaybeUninit<u16> = std::mem::MaybeUninit::uninit();
    let __slate_slot_884: *mut u16 = std::ptr::addr_of_mut!(__slate_storage_884) as *mut u16;
    let mut __slate_storage_883: std::mem::MaybeUninit<*mut WhereTerm> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_883: *mut *mut WhereTerm =
        std::ptr::addr_of_mut!(__slate_storage_883) as *mut *mut WhereTerm;
    let mut __slate_storage_882: std::mem::MaybeUninit<u16> = std::mem::MaybeUninit::uninit();
    let __slate_slot_882: *mut u16 = std::ptr::addr_of_mut!(__slate_storage_882) as *mut u16;
    let mut __slate_storage_881: std::mem::MaybeUninit<u16> = std::mem::MaybeUninit::uninit();
    let __slate_slot_881: *mut u16 = std::ptr::addr_of_mut!(__slate_storage_881) as *mut u16;
    let mut __slate_storage_880: std::mem::MaybeUninit<*mut WhereTerm> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_880: *mut *mut WhereTerm =
        std::ptr::addr_of_mut!(__slate_storage_880) as *mut *mut WhereTerm;
    let mut __slate_storage_567: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_567: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_567) as *mut i32; // Extra bits for pNew->eOperator
    let mut __slate_storage_566: std::mem::MaybeUninit<u16> = std::mem::MaybeUninit::uninit();
    let __slate_slot_566: *mut u16 = std::ptr::addr_of_mut!(__slate_storage_566) as *mut u16;
    let mut __slate_storage_565: std::mem::MaybeUninit<*mut Expr> = std::mem::MaybeUninit::uninit();
    let __slate_slot_565: *mut *mut Expr =
        std::ptr::addr_of_mut!(__slate_storage_565) as *mut *mut Expr;
    let mut __slate_storage_564: std::mem::MaybeUninit<*mut WhereTerm> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_564: *mut *mut WhereTerm =
        std::ptr::addr_of_mut!(__slate_storage_564) as *mut *mut WhereTerm;
    let mut __slate_storage_895: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_895: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_895) as *mut u32;
    let mut __slate_storage_894: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_894: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_894) as *mut u32;
    let mut __slate_storage_893: std::mem::MaybeUninit<*mut Expr> = std::mem::MaybeUninit::uninit();
    let __slate_slot_893: *mut *mut Expr =
        std::ptr::addr_of_mut!(__slate_storage_893) as *mut *mut Expr;
    let mut __slate_storage_892: std::mem::MaybeUninit<bool> = std::mem::MaybeUninit::uninit();
    let __slate_slot_892: *mut bool = std::ptr::addr_of_mut!(__slate_storage_892) as *mut bool;
    let mut __slate_storage_879: std::mem::MaybeUninit<bool> = std::mem::MaybeUninit::uninit();
    let __slate_slot_879: *mut bool = std::ptr::addr_of_mut!(__slate_storage_879) as *mut bool;
    let mut __slate_storage_878: std::mem::MaybeUninit<u16> = std::mem::MaybeUninit::uninit();
    let __slate_slot_878: *mut u16 = std::ptr::addr_of_mut!(__slate_storage_878) as *mut u16;
    let mut __slate_storage_877: std::mem::MaybeUninit<u16> = std::mem::MaybeUninit::uninit();
    let __slate_slot_877: *mut u16 = std::ptr::addr_of_mut!(__slate_storage_877) as *mut u16;
    let mut __slate_storage_876: std::mem::MaybeUninit<*mut WhereTerm> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_876: *mut *mut WhereTerm =
        std::ptr::addr_of_mut!(__slate_storage_876) as *mut *mut WhereTerm;
    let mut __slate_storage_563: std::mem::MaybeUninit<u16> = std::mem::MaybeUninit::uninit();
    let __slate_slot_563: *mut u16 = std::ptr::addr_of_mut!(__slate_storage_563) as *mut u16;
    let mut __slate_storage_562: std::mem::MaybeUninit<*mut Expr> = std::mem::MaybeUninit::uninit();
    let __slate_slot_562: *mut *mut Expr =
        std::ptr::addr_of_mut!(__slate_storage_562) as *mut *mut Expr;
    let mut __slate_storage_561: std::mem::MaybeUninit<*mut Expr> = std::mem::MaybeUninit::uninit();
    let __slate_slot_561: *mut *mut Expr =
        std::ptr::addr_of_mut!(__slate_storage_561) as *mut *mut Expr;
    let mut __slate_storage_560: std::mem::MaybeUninit<[i32; 2]> = std::mem::MaybeUninit::uninit();
    let __slate_slot_560: *mut [i32; 2] =
        std::ptr::addr_of_mut!(__slate_storage_560) as *mut [i32; 2];
    let mut __slate_storage_897: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_897: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_897) as *mut i32;
    let mut __slate_storage_896: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_896: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_896) as *mut i32;
    let mut __slate_storage_572: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_572: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_572) as *mut i32;
    let mut __slate_storage_571: std::mem::MaybeUninit<*mut Expr> = std::mem::MaybeUninit::uninit();
    let __slate_slot_571: *mut *mut Expr =
        std::ptr::addr_of_mut!(__slate_storage_571) as *mut *mut Expr;
    let mut __slate_storage_569: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_569: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_569) as *mut i32;
    let mut __slate_storage_568: std::mem::MaybeUninit<*mut ExprList> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_568: *mut *mut ExprList =
        std::ptr::addr_of_mut!(__slate_storage_568) as *mut *mut ExprList;
    let mut __slate_storage_898: std::mem::MaybeUninit<*mut Expr> = std::mem::MaybeUninit::uninit();
    let __slate_slot_898: *mut *mut Expr =
        std::ptr::addr_of_mut!(__slate_storage_898) as *mut *mut Expr;
    let mut __slate_storage_573: std::mem::MaybeUninit<*mut Expr> = std::mem::MaybeUninit::uninit();
    let __slate_slot_573: *mut *mut Expr =
        std::ptr::addr_of_mut!(__slate_storage_573) as *mut *mut Expr;
    let mut __slate_storage_901: std::mem::MaybeUninit<u16> = std::mem::MaybeUninit::uninit();
    let __slate_slot_901: *mut u16 = std::ptr::addr_of_mut!(__slate_storage_901) as *mut u16;
    let mut __slate_storage_900: std::mem::MaybeUninit<u16> = std::mem::MaybeUninit::uninit();
    let __slate_slot_900: *mut u16 = std::ptr::addr_of_mut!(__slate_storage_900) as *mut u16;
    let mut __slate_storage_899: std::mem::MaybeUninit<*mut WhereTerm> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_899: *mut *mut WhereTerm =
        std::ptr::addr_of_mut!(__slate_storage_899) as *mut *mut WhereTerm;
    let mut __slate_storage_577: std::mem::MaybeUninit<*mut WhereTerm> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_577: *mut *mut WhereTerm =
        std::ptr::addr_of_mut!(__slate_storage_577) as *mut *mut WhereTerm;
    let mut __slate_storage_576: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_576: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_576) as *mut i32;
    let mut __slate_storage_575: std::mem::MaybeUninit<*mut Expr> = std::mem::MaybeUninit::uninit();
    let __slate_slot_575: *mut *mut Expr =
        std::ptr::addr_of_mut!(__slate_storage_575) as *mut *mut Expr;
    let mut __slate_storage_574: std::mem::MaybeUninit<*mut Expr> = std::mem::MaybeUninit::uninit();
    let __slate_slot_574: *mut *mut Expr =
        std::ptr::addr_of_mut!(__slate_storage_574) as *mut *mut Expr;
    let mut __slate_storage_913: std::mem::MaybeUninit<u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_913: *mut u8 = std::ptr::addr_of_mut!(__slate_storage_913) as *mut u8;
    let mut __slate_storage_912: std::mem::MaybeUninit<u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_912: *mut u8 = std::ptr::addr_of_mut!(__slate_storage_912) as *mut u8;
    let mut __slate_storage_911: std::mem::MaybeUninit<*mut u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_911: *mut *mut u8 =
        std::ptr::addr_of_mut!(__slate_storage_911) as *mut *mut u8;
    let mut __slate_storage_910: std::mem::MaybeUninit<*mut u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_910: *mut *mut u8 =
        std::ptr::addr_of_mut!(__slate_storage_910) as *mut *mut u8;
    let mut __slate_storage_909: std::mem::MaybeUninit<*mut u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_909: *mut *mut u8 =
        std::ptr::addr_of_mut!(__slate_storage_909) as *mut *mut u8; // Last character before the first wildcard
    let mut __slate_storage_588: std::mem::MaybeUninit<*mut u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_588: *mut *mut u8 =
        std::ptr::addr_of_mut!(__slate_storage_588) as *mut *mut u8;
    let mut __slate_storage_906: std::mem::MaybeUninit<i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_906: *mut i8 = std::ptr::addr_of_mut!(__slate_storage_906) as *mut i8;
    let mut __slate_storage_908: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_908: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_908) as *mut i32;
    let mut __slate_storage_907: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_907: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_907) as *mut i32;
    let mut __slate_storage_905: std::mem::MaybeUninit<u16> = std::mem::MaybeUninit::uninit();
    let __slate_slot_905: *mut u16 = std::ptr::addr_of_mut!(__slate_storage_905) as *mut u16;
    let mut __slate_storage_904: std::mem::MaybeUninit<u16> = std::mem::MaybeUninit::uninit();
    let __slate_slot_904: *mut u16 = std::ptr::addr_of_mut!(__slate_storage_904) as *mut u16;
    let mut __slate_storage_903: std::mem::MaybeUninit<*mut WhereTerm> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_903: *mut *mut WhereTerm =
        std::ptr::addr_of_mut!(__slate_storage_903) as *mut *mut WhereTerm;
    let mut __slate_storage_587: std::mem::MaybeUninit<i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_587: *mut i8 = std::ptr::addr_of_mut!(__slate_storage_587) as *mut i8;
    let mut __slate_storage_586: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_586: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_586) as *mut i32;
    let mut __slate_storage_585: std::mem::MaybeUninit<u16> = std::mem::MaybeUninit::uninit();
    let __slate_slot_585: *mut u16 = std::ptr::addr_of_mut!(__slate_storage_585) as *mut u16; // Name of collating sequence
    let mut __slate_storage_584: std::mem::MaybeUninit<*const i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_584: *mut *const i8 =
        std::ptr::addr_of_mut!(__slate_storage_584) as *mut *const i8;
    let mut __slate_storage_583: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_583: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_583) as *mut i32;
    let mut __slate_storage_582: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_582: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_582) as *mut i32;
    let mut __slate_storage_581: std::mem::MaybeUninit<*mut Expr> = std::mem::MaybeUninit::uninit();
    let __slate_slot_581: *mut *mut Expr =
        std::ptr::addr_of_mut!(__slate_storage_581) as *mut *mut Expr;
    let mut __slate_storage_580: std::mem::MaybeUninit<*mut Expr> = std::mem::MaybeUninit::uninit();
    let __slate_slot_580: *mut *mut Expr =
        std::ptr::addr_of_mut!(__slate_storage_580) as *mut *mut Expr; // Copy of pStr1 - RHS of LIKE/GLOB operator
    let mut __slate_storage_579: std::mem::MaybeUninit<*mut Expr> = std::mem::MaybeUninit::uninit();
    let __slate_slot_579: *mut *mut Expr =
        std::ptr::addr_of_mut!(__slate_storage_579) as *mut *mut Expr; // LHS of LIKE/GLOB operator
    let mut __slate_storage_578: std::mem::MaybeUninit<*mut Expr> = std::mem::MaybeUninit::uninit();
    let __slate_slot_578: *mut *mut Expr =
        std::ptr::addr_of_mut!(__slate_storage_578) as *mut *mut Expr;
    let mut __slate_storage_902: std::mem::MaybeUninit<bool> = std::mem::MaybeUninit::uninit();
    let __slate_slot_902: *mut bool = std::ptr::addr_of_mut!(__slate_storage_902) as *mut bool;
    let mut __slate_storage_872: std::mem::MaybeUninit<u64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_872: *mut u64 = std::ptr::addr_of_mut!(__slate_storage_872) as *mut u64;
    let mut __slate_storage_871: std::mem::MaybeUninit<u64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_871: *mut u64 = std::ptr::addr_of_mut!(__slate_storage_871) as *mut u64;
    let mut __slate_storage_875: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_875: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_875) as *mut u32;
    let mut __slate_storage_874: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_874: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_874) as *mut u32;
    let mut __slate_storage_873: std::mem::MaybeUninit<*mut Expr> = std::mem::MaybeUninit::uninit();
    let __slate_slot_873: *mut *mut Expr =
        std::ptr::addr_of_mut!(__slate_storage_873) as *mut *mut Expr;
    let mut __slate_storage_559: std::mem::MaybeUninit<u64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_559: *mut u64 = std::ptr::addr_of_mut!(__slate_storage_559) as *mut u64;
    let mut __slate_storage_870: std::mem::MaybeUninit<u16> = std::mem::MaybeUninit::uninit();
    let __slate_slot_870: *mut u16 = std::ptr::addr_of_mut!(__slate_storage_870) as *mut u16;
    let mut __slate_storage_869: std::mem::MaybeUninit<u16> = std::mem::MaybeUninit::uninit();
    let __slate_slot_869: *mut u16 = std::ptr::addr_of_mut!(__slate_storage_869) as *mut u16;
    let mut __slate_storage_868: std::mem::MaybeUninit<*mut WhereTerm> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_868: *mut *mut WhereTerm =
        std::ptr::addr_of_mut!(__slate_storage_868) as *mut *mut WhereTerm; // Number of elements on left side vector
    let mut __slate_storage_558: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_558: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_558) as *mut i32; // op2 value for LIKE/REGEXP/GLOB
    let mut __slate_storage_557: std::mem::MaybeUninit<u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_557: *mut u8 = std::ptr::addr_of_mut!(__slate_storage_557) as *mut u8; // Database connection
    let mut __slate_storage_556: std::mem::MaybeUninit<*mut sqlite3> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_556: *mut *mut sqlite3 =
        std::ptr::addr_of_mut!(__slate_storage_556) as *mut *mut sqlite3; // Parsing context
    let mut __slate_storage_555: std::mem::MaybeUninit<*mut Parse> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_555: *mut *mut Parse =
        std::ptr::addr_of_mut!(__slate_storage_555) as *mut *mut Parse; // Top-level operator.  pExpr->op
    let mut __slate_storage_554: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_554: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_554) as *mut i32; // uppercase equivalent to lowercase
    let mut __slate_storage_553: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_553: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_553) as *mut i32; // RHS of LIKE/GLOB ends with wildcard
    let mut __slate_storage_552: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_552: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_552) as *mut i32; // RHS of LIKE/GLOB operator
    let mut __slate_storage_551: std::mem::MaybeUninit<*mut Expr> = std::mem::MaybeUninit::uninit();
    let __slate_slot_551: *mut *mut Expr =
        std::ptr::addr_of_mut!(__slate_storage_551) as *mut *mut Expr; // Extra dependencies on LEFT JOIN
    let mut __slate_storage_550: std::mem::MaybeUninit<u64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_550: *mut u64 = std::ptr::addr_of_mut!(__slate_storage_550) as *mut u64; // Prerequisites of pExpr
    let mut __slate_storage_549: std::mem::MaybeUninit<u64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_549: *mut u64 = std::ptr::addr_of_mut!(__slate_storage_549) as *mut u64; // Prerequisites of the pExpr->pLeft
    let mut __slate_storage_548: std::mem::MaybeUninit<u64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_548: *mut u64 = std::ptr::addr_of_mut!(__slate_storage_548) as *mut u64; // The expression to be analyzed
    let mut __slate_storage_547: std::mem::MaybeUninit<*mut Expr> = std::mem::MaybeUninit::uninit();
    let __slate_slot_547: *mut *mut Expr =
        std::ptr::addr_of_mut!(__slate_storage_547) as *mut *mut Expr; // Set of table index masks
    let mut __slate_storage_546: std::mem::MaybeUninit<*mut WhereMaskSet> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_546: *mut *mut WhereMaskSet =
        std::ptr::addr_of_mut!(__slate_storage_546) as *mut *mut WhereMaskSet; // The term to be analyzed
    let mut __slate_storage_545: std::mem::MaybeUninit<*mut WhereTerm> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_545: *mut *mut WhereTerm =
        std::ptr::addr_of_mut!(__slate_storage_545) as *mut *mut WhereTerm; // WHERE clause processing context
    let mut __slate_storage_544: std::mem::MaybeUninit<*mut WhereInfo> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_544: *mut *mut WhereInfo =
        std::ptr::addr_of_mut!(__slate_storage_544) as *mut *mut WhereInfo;
    unsafe {
        std::ptr::write(__slate_slot_544, unsafe { (*pWC).pWInfo });
        std::ptr::write(__slate_slot_550, ((0 as i32) as i64) as u64);
        std::ptr::write(__slate_slot_551, std::ptr::null_mut::<Expr>());
        std::ptr::write(__slate_slot_552, 0 as i32);
        std::ptr::write(__slate_slot_553, 0 as i32);
        std::ptr::write(__slate_slot_555, unsafe { (*(*__slate_slot_544)).pParse });
        std::ptr::write(__slate_slot_556, unsafe { (*(*__slate_slot_555)).db });
        std::ptr::write(__slate_slot_557, ((0 as i32) as i8) as u8);
        if (unsafe { (*(*__slate_slot_556)).mallocFailed }) != (0 as u8) {
            return;
        } else {
            0 as i32;
            *__slate_slot_545 = unsafe { unsafe { (*pWC).a }.offset(idxTerm as isize) };
            *__slate_slot_546 = unsafe { std::ptr::addr_of_mut!((*(*__slate_slot_544)).sMaskSet) };
            *__slate_slot_547 = unsafe { (*(*__slate_slot_545)).pExpr };
            0 as i32; // Because malloc() has not failed
            0 as i32;
            '__join_23: {
                '__join_49: {
                    '__join_52: {
                        '__join_53: {
                            loop {
                                unsafe {
                                    (*(*__slate_slot_546)).bVarSelect = 0 as i32;
                                }
                                *__slate_slot_548 =
                                    sqlite3WhereExprUsage(*__slate_slot_546, unsafe {
                                        (*(*__slate_slot_547)).pLeft
                                    });
                                *__slate_slot_554 =
                                    ((unsafe { (*(*__slate_slot_547)).op }) as u32) as i32;
                                if *__slate_slot_554 == (50 as i32) {
                                    0 as i32;
                                    if (unsafe {
                                        sqlite3ExprCheckIN(*__slate_slot_555, *__slate_slot_547)
                                    }) != (0 as i32)
                                    {
                                        return;
                                    } else {
                                        if (unsafe { (*(*__slate_slot_547)).flags })
                                            & ((4096 as i32) as u32)
                                            != ((0 as i32) as u32)
                                        {
                                            unsafe {
                                                (*(*__slate_slot_545)).prereqRight =
                                                    exprSelectUsage(*__slate_slot_546, unsafe {
                                                        (*(*__slate_slot_547)).x.pSelect
                                                    });
                                            }
                                        } else {
                                            unsafe {
                                                (*(*__slate_slot_545)).prereqRight =
                                                    sqlite3WhereExprListUsage(
                                                        *__slate_slot_546,
                                                        unsafe { (*(*__slate_slot_547)).x.pList },
                                                    );
                                            }
                                        }
                                        *__slate_slot_549 = *__slate_slot_548
                                            | unsafe { (*(*__slate_slot_545)).prereqRight };
                                    }
                                } else {
                                    unsafe {
                                        (*(*__slate_slot_545)).prereqRight =
                                            sqlite3WhereExprUsage(*__slate_slot_546, unsafe {
                                                (*(*__slate_slot_547)).pRight
                                            });
                                    }
                                    if (unsafe { (*(*__slate_slot_547)).pLeft })
                                        == std::ptr::null_mut::<Expr>()
                                        || (unsafe { (*(*__slate_slot_547)).flags })
                                            & (((4096 as i32) | (262144 as i32)) as u32)
                                            != ((0 as i32) as u32)
                                        || (unsafe { (*(*__slate_slot_547)).x.pList })
                                            != std::ptr::null_mut::<ExprList>()
                                    {
                                        *__slate_slot_549 = sqlite3WhereExprUsageNN(
                                            *__slate_slot_546,
                                            *__slate_slot_547,
                                        );
                                    } else {
                                        *__slate_slot_549 = *__slate_slot_548
                                            | unsafe { (*(*__slate_slot_545)).prereqRight };
                                    }
                                }
                                if (unsafe { (*(*__slate_slot_546)).bVarSelect }) != (0 as i32) {
                                    std::ptr::write(__slate_slot_868, *__slate_slot_545);
                                    std::ptr::write(__slate_slot_869, unsafe {
                                        (*(*__slate_slot_868)).wtFlags
                                    });
                                    std::ptr::write(
                                        __slate_slot_870,
                                        ((((*__slate_slot_869 as u32) as i32) | (4096 as i32))
                                            as i16) as u16,
                                    );
                                    unsafe {
                                        (*(*__slate_slot_868)).wtFlags = *__slate_slot_870;
                                    }
                                }
                                if (unsafe { (*(*__slate_slot_547)).flags })
                                    & (((1 as i32) | (2 as i32)) as u32)
                                    != ((0 as i32) as u32)
                                {
                                    std::ptr::write(__slate_slot_559, unsafe {
                                        sqlite3WhereGetMask(*__slate_slot_546, unsafe {
                                            (*(*__slate_slot_547)).w.iJoin
                                        })
                                    });
                                    if (unsafe { (*(*__slate_slot_547)).flags })
                                        & ((1 as i32) as u32)
                                        != ((0 as i32) as u32)
                                    {
                                        std::ptr::write(__slate_slot_871, *__slate_slot_549);
                                        std::ptr::write(
                                            __slate_slot_872,
                                            *__slate_slot_871 | *__slate_slot_559,
                                        );
                                        *__slate_slot_549 = *__slate_slot_872;
                                        *__slate_slot_550 = (*__slate_slot_559)
                                            .wrapping_sub(((1 as i32) as i64) as u64);
                                    // ON clause terms may not be used with an index
                                    // on left table of a LEFT JOIN.  Ticket #3015
                                    } else {
                                        if *__slate_slot_549 >> (1 as i32) >= *__slate_slot_559 {
                                            std::ptr::write(__slate_slot_873, *__slate_slot_547);
                                            std::ptr::write(__slate_slot_874, unsafe {
                                                (*(*__slate_slot_873)).flags
                                            });
                                            std::ptr::write(
                                                __slate_slot_875,
                                                *__slate_slot_874 & !((2 as i32) as u32),
                                            );
                                            unsafe {
                                                (*(*__slate_slot_873)).flags = *__slate_slot_875;
                                            }
                                        }
                                    }
                                }
                                unsafe {
                                    (*(*__slate_slot_545)).prereqAll = *__slate_slot_549;
                                }
                                unsafe {
                                    (*(*__slate_slot_545)).leftCursor = -(1 as i32);
                                }
                                unsafe {
                                    (*(*__slate_slot_545)).iParent = -(1 as i32);
                                }
                                unsafe {
                                    (*(*__slate_slot_545)).eOperator = ((0 as i32) as i16) as u16;
                                }
                                if allowedOp(*__slate_slot_554) != (0 as i32) {
                                    break '__join_49;
                                } else {
                                    if (((unsafe { (*(*__slate_slot_547)).op }) as u32) as i32)
                                        == (49 as i32)
                                        && (((unsafe { (*pWC).op }) as u32) as i32) == (44 as i32)
                                    {
                                        break '__join_52;
                                    } else {
                                        if (((unsafe { (*(*__slate_slot_547)).op }) as u32) as i32)
                                            == (43 as i32)
                                            && !((unsafe { (*(*__slate_slot_547)).flags })
                                                & ((512 as i32) as u32)
                                                != ((0 as i32) as u32))
                                        {
                                            std::ptr::write(__slate_slot_573, unsafe {
                                                sqlite3ExprSimplifiedAndOr(*__slate_slot_547)
                                            });
                                            if *__slate_slot_573 != *__slate_slot_547 {
                                                std::ptr::write(__slate_slot_898, unsafe {
                                                    sqlite3ExprSkipCollateAndLikely(
                                                        *__slate_slot_573,
                                                    )
                                                });
                                                *__slate_slot_547 = *__slate_slot_898;
                                                unsafe {
                                                    (*(*__slate_slot_545)).pExpr =
                                                        *__slate_slot_898;
                                                }
                                            } else {
                                                break '__join_53;
                                            }
                                        } else {
                                            break;
                                        }
                                    }
                                }
                            }
                            if (((unsafe { (*(*__slate_slot_547)).op }) as u32) as i32)
                                == (52 as i32)
                            {
                                if (((unsafe { (*unsafe { (*(*__slate_slot_547)).pLeft }).op })
                                    as u32) as i32)
                                    == (168 as i32)
                                    && ((unsafe {
                                        (*unsafe { (*(*__slate_slot_547)).pLeft }).iColumn
                                    }) as i32)
                                        >= (0 as i32)
                                    && !((unsafe { (*(*__slate_slot_547)).flags })
                                        & ((1 as i32) as u32)
                                        != ((0 as i32) as u32))
                                {
                                    std::ptr::write(__slate_slot_575, unsafe {
                                        (*(*__slate_slot_547)).pLeft
                                    });
                                    *__slate_slot_574 = unsafe {
                                        sqlite3PExpr(
                                            *__slate_slot_555,
                                            55 as i32,
                                            unsafe {
                                                sqlite3ExprDup(
                                                    *__slate_slot_556,
                                                    *__slate_slot_575 as *const Expr,
                                                    0 as i32,
                                                )
                                            },
                                            unsafe {
                                                sqlite3ExprAlloc(
                                                    *__slate_slot_556,
                                                    122 as i32,
                                                    std::ptr::null::<Token>(),
                                                    0 as i32,
                                                )
                                            },
                                        )
                                    };
                                    *__slate_slot_576 = whereClauseInsert(
                                        pWC,
                                        *__slate_slot_574,
                                        (((2 as i32) | (1 as i32) | (128 as i32)) as i16) as u16,
                                    );
                                    if *__slate_slot_576 != (0 as i32) {
                                        *__slate_slot_577 = unsafe {
                                            unsafe { (*pWC).a }.offset(*__slate_slot_576 as isize)
                                        };
                                        unsafe {
                                            (*(*__slate_slot_577)).prereqRight =
                                                ((0 as i32) as i64) as u64;
                                        }
                                        unsafe {
                                            (*(*__slate_slot_577)).leftCursor =
                                                unsafe { (*(*__slate_slot_575)).iTable };
                                        }
                                        unsafe {
                                            (*(*__slate_slot_577)).u.x.leftColumn =
                                                (unsafe { (*(*__slate_slot_575)).iColumn }) as i32;
                                        }
                                        unsafe {
                                            (*(*__slate_slot_577)).eOperator =
                                                (((2 as i32) << (55 as i32) - (54 as i32)) as i16)
                                                    as u16;
                                        }
                                        markTermAsChild(pWC, *__slate_slot_576, idxTerm);
                                        *__slate_slot_545 =
                                            unsafe { unsafe { (*pWC).a }.offset(idxTerm as isize) };
                                        std::ptr::write(__slate_slot_899, *__slate_slot_545);
                                        std::ptr::write(__slate_slot_900, unsafe {
                                            (*(*__slate_slot_899)).wtFlags
                                        });
                                        std::ptr::write(
                                            __slate_slot_901,
                                            ((((*__slate_slot_900 as u32) as i32) | (8 as i32))
                                                as i16)
                                                as u16,
                                        );
                                        unsafe {
                                            (*(*__slate_slot_899)).wtFlags = *__slate_slot_901;
                                        }
                                        unsafe {
                                            (*(*__slate_slot_577)).prereqAll =
                                                unsafe { (*(*__slate_slot_545)).prereqAll };
                                        }
                                        break '__join_23;
                                    } else {
                                        break '__join_23;
                                    }
                                } else {
                                    break '__join_23;
                                }
                            } else {
                                if (((unsafe { (*(*__slate_slot_547)).op }) as u32) as i32)
                                    == (172 as i32)
                                    && (((unsafe { (*pWC).op }) as u32) as i32) == (44 as i32)
                                {
                                    *__slate_slot_902 = isLikeOrGlob(
                                        *__slate_slot_555,
                                        *__slate_slot_547,
                                        std::ptr::addr_of_mut!(*__slate_slot_551),
                                        std::ptr::addr_of_mut!(*__slate_slot_552),
                                        std::ptr::addr_of_mut!(*__slate_slot_553),
                                    ) != (0 as i32);
                                } else {
                                    *__slate_slot_902 = false as bool;
                                }
                                if *__slate_slot_902 {
                                    '__join_69: {
                                        std::ptr::write(
                                            __slate_slot_585,
                                            (((256 as i32) | (2 as i32) | (1 as i32)) as i16)
                                                as u16,
                                        );
                                        0 as i32;
                                        *__slate_slot_578 = unsafe {
                                            (*unsafe {
                                                unsafe {
                                                    std::ptr::addr_of_mut!(
                                                        (*unsafe {
                                                            (*(*__slate_slot_547)).x.pList
                                                        })
                                                        .a
                                                    )
                                                        as *mut ExprList_item
                                                }
                                                .offset((1 as i32) as isize)
                                            })
                                            .pExpr
                                        };
                                        *__slate_slot_579 = unsafe {
                                            sqlite3ExprDup(
                                                *__slate_slot_556,
                                                *__slate_slot_551 as *const Expr,
                                                0 as i32,
                                            )
                                        };
                                        0 as i32;
                                        0 as i32;
                                        // Convert the lower bound to upper-case and the upper bound to
                                        // lower-case (upper-case is less than lower-case in ASCII) so that
                                        // the range constraints also work for BLOBs
                                        if *__slate_slot_553 != (0 as i32)
                                            && !((unsafe {
                                                (*unsafe { (*(*__slate_slot_555)).db }).mallocFailed
                                            }) != (0 as u8))
                                        {
                                            std::ptr::write(__slate_slot_903, *__slate_slot_545);
                                            std::ptr::write(__slate_slot_904, unsafe {
                                                (*(*__slate_slot_903)).wtFlags
                                            });
                                            std::ptr::write(
                                                __slate_slot_905,
                                                ((((*__slate_slot_904 as u32) as i32)
                                                    | (1024 as i32))
                                                    as i16)
                                                    as u16,
                                            );
                                            unsafe {
                                                (*(*__slate_slot_903)).wtFlags = *__slate_slot_905;
                                            }
                                            *__slate_slot_586 = 0 as i32;
                                            loop {
                                                std::ptr::write(__slate_slot_906, unsafe {
                                                    *unsafe {
                                                        unsafe { (*(*__slate_slot_551)).u.zToken }
                                                            .offset(*__slate_slot_586 as isize)
                                                    }
                                                });
                                                *__slate_slot_587 = *__slate_slot_906;
                                                if (*__slate_slot_906 as i32) != (0 as i32) {
                                                    unsafe {
                                                        *unsafe {
                                                            unsafe {
                                                                (*(*__slate_slot_551)).u.zToken
                                                            }
                                                            .offset(*__slate_slot_586 as isize)
                                                        } = ((*__slate_slot_587 as i32)
                                                            & !((((unsafe {
                                                                *unsafe {
                                                                    unsafe {
                                                                        std::ptr::addr_of!(
                                                                            sqlite3CtypeMap
                                                                        )
                                                                            as *const u8
                                                                    }
                                                                    .offset(
                                                                        (((*__slate_slot_587 as u8)
                                                                            as u32)
                                                                            as i32)
                                                                            as isize,
                                                                    )
                                                                }
                                                            })
                                                                as u32)
                                                                as i32)
                                                                & (32 as i32)))
                                                            as i8;
                                                    }
                                                    unsafe {
                                                        *unsafe {
                                                            unsafe {
                                                                (*(*__slate_slot_579)).u.zToken
                                                            }
                                                            .offset(*__slate_slot_586 as isize)
                                                        } = (unsafe {
                                                            *unsafe {
                                                                unsafe {
                                                                    std::ptr::addr_of!(
                                                                        sqlite3UpperToLower
                                                                    )
                                                                        as *const u8
                                                                }
                                                                .offset(
                                                                    (((*__slate_slot_587 as u8)
                                                                        as u32)
                                                                        as i32)
                                                                        as isize,
                                                                )
                                                            }
                                                        })
                                                            as i8;
                                                    }
                                                    std::ptr::write(
                                                        __slate_slot_907,
                                                        *__slate_slot_586,
                                                    );
                                                    std::ptr::write(
                                                        __slate_slot_908,
                                                        *__slate_slot_907 + (1 as i32),
                                                    );
                                                    *__slate_slot_586 = *__slate_slot_908;
                                                } else {
                                                    break '__join_69;
                                                }
                                            }
                                        }
                                    }
                                    if !((unsafe { (*(*__slate_slot_556)).mallocFailed })
                                        != (0 as u8))
                                    {
                                        *__slate_slot_588 = (unsafe {
                                            unsafe { (*(*__slate_slot_579)).u.zToken }.offset(
                                                ((unsafe {
                                                    sqlite3Strlen30(
                                                        (unsafe { (*(*__slate_slot_579)).u.zToken })
                                                            as *const i8,
                                                    )
                                                }) - (1 as i32))
                                                    as isize,
                                            )
                                        })
                                            as *mut u8;
                                        if *__slate_slot_553 != (0 as i32) {
                                            // The point is to increment the last character before the first
                                            // wildcard.  But if we increment '@', that will push it into the
                                            // alphabetic range where case conversions will mess up the
                                            // inequality.  To avoid this, make sure to also run the full
                                            // LIKE on all candidate expressions by clearing the isComplete flag
                                            if (((unsafe { *(*__slate_slot_588) }) as u32) as i32)
                                                == (65 as i32) - (1 as i32)
                                            {
                                                *__slate_slot_552 = 0 as i32;
                                            }
                                            unsafe {
                                                *(*__slate_slot_588) = unsafe {
                                                    *unsafe {
                                                        unsafe {
                                                            std::ptr::addr_of!(sqlite3UpperToLower)
                                                                as *const u8
                                                        }
                                                        .offset(
                                                            (((unsafe { *(*__slate_slot_588) })
                                                                as u32)
                                                                as i32)
                                                                as isize,
                                                        )
                                                    }
                                                };
                                            }
                                        }
                                        // Increment the value of the last utf8 character in the prefix.
                                        loop {
                                            if (((unsafe { *(*__slate_slot_588) }) as u32) as i32)
                                                == (191 as i32)
                                                && *__slate_slot_588
                                                    > ((unsafe { (*(*__slate_slot_579)).u.zToken })
                                                        as *mut u8)
                                            {
                                                unsafe {
                                                    *(*__slate_slot_588) =
                                                        ((128 as i32) as i8) as u8;
                                                }
                                                std::ptr::write(
                                                    __slate_slot_909,
                                                    *__slate_slot_588,
                                                );
                                                std::ptr::write(__slate_slot_910, unsafe {
                                                    (*__slate_slot_909)
                                                        .offset(-((1 as i32) as isize))
                                                });
                                                *__slate_slot_588 = *__slate_slot_910;
                                            } else {
                                                break;
                                            }
                                        }
                                        0 as i32; // isLikeOrGlob() guarantees this
                                        std::ptr::write(__slate_slot_911, *__slate_slot_588);
                                        std::ptr::write(__slate_slot_912, unsafe {
                                            *(*__slate_slot_911)
                                        });
                                        std::ptr::write(
                                            __slate_slot_913,
                                            ((((*__slate_slot_912 as u32) as i32) + (1 as i32))
                                                as i8)
                                                as u8,
                                        );
                                        unsafe {
                                            *(*__slate_slot_911) = *__slate_slot_913;
                                        }
                                    }
                                    *__slate_slot_584 = if *__slate_slot_553 != (0 as i32) {
                                        (b"NOCASE\0".as_ptr() as *mut i8) as *const i8
                                    } else {
                                        unsafe { std::ptr::addr_of!(sqlite3StrBINARY) as *const i8 }
                                    };
                                    *__slate_slot_580 = unsafe {
                                        sqlite3ExprDup(
                                            *__slate_slot_556,
                                            *__slate_slot_578 as *const Expr,
                                            0 as i32,
                                        )
                                    };
                                    *__slate_slot_580 = unsafe {
                                        sqlite3PExpr(
                                            *__slate_slot_555,
                                            58 as i32,
                                            unsafe {
                                                sqlite3ExprAddCollateString(
                                                    *__slate_slot_555 as *const Parse,
                                                    *__slate_slot_580,
                                                    *__slate_slot_584,
                                                )
                                            },
                                            *__slate_slot_551,
                                        )
                                    };
                                    transferJoinMarkings(*__slate_slot_580, *__slate_slot_547);
                                    *__slate_slot_582 = whereClauseInsert(
                                        pWC,
                                        *__slate_slot_580,
                                        *__slate_slot_585,
                                    );
                                    {}
                                    *__slate_slot_581 = unsafe {
                                        sqlite3ExprDup(
                                            *__slate_slot_556,
                                            *__slate_slot_578 as *const Expr,
                                            0 as i32,
                                        )
                                    };
                                    *__slate_slot_581 = unsafe {
                                        sqlite3PExpr(
                                            *__slate_slot_555,
                                            57 as i32,
                                            unsafe {
                                                sqlite3ExprAddCollateString(
                                                    *__slate_slot_555 as *const Parse,
                                                    *__slate_slot_581,
                                                    *__slate_slot_584,
                                                )
                                            },
                                            *__slate_slot_579,
                                        )
                                    };
                                    transferJoinMarkings(*__slate_slot_581, *__slate_slot_547);
                                    *__slate_slot_583 = whereClauseInsert(
                                        pWC,
                                        *__slate_slot_581,
                                        *__slate_slot_585,
                                    );
                                    {}
                                    exprAnalyze(pSrc, pWC, *__slate_slot_582);
                                    exprAnalyze(pSrc, pWC, *__slate_slot_583);
                                    *__slate_slot_545 =
                                        unsafe { unsafe { (*pWC).a }.offset(idxTerm as isize) };
                                    if *__slate_slot_552 != (0 as i32) {
                                        markTermAsChild(pWC, *__slate_slot_582, idxTerm);
                                        markTermAsChild(pWC, *__slate_slot_583, idxTerm);
                                        break '__join_23;
                                    } else {
                                        break '__join_23;
                                    }
                                } else {
                                    break '__join_23;
                                }
                            }
                        }
                        0 as i32;
                        exprAnalyzeOrTerm(pSrc, pWC, idxTerm);
                        *__slate_slot_545 = unsafe { unsafe { (*pWC).a }.offset(idxTerm as isize) };
                        break '__join_23;
                    }
                    0 as i32;
                    *__slate_slot_568 = unsafe { (*(*__slate_slot_547)).x.pList };
                    0 as i32;
                    0 as i32;
                    0 as i32;
                    *__slate_slot_569 = 0 as i32;
                    loop {
                        if *__slate_slot_569 < (2 as i32) {
                            *__slate_slot_571 = unsafe {
                                sqlite3PExpr(
                                    *__slate_slot_555,
                                    ((unsafe {
                                        *unsafe {
                                            unsafe { std::ptr::addr_of!(ops) as *const u8 }
                                                .offset(*__slate_slot_569 as isize)
                                        }
                                    }) as u32) as i32,
                                    unsafe {
                                        sqlite3ExprDup(
                                            *__slate_slot_556,
                                            (unsafe { (*(*__slate_slot_547)).pLeft })
                                                as *const Expr,
                                            0 as i32,
                                        )
                                    },
                                    unsafe {
                                        sqlite3ExprDup(
                                            *__slate_slot_556,
                                            (unsafe {
                                                (*unsafe {
                                                    unsafe {
                                                        std::ptr::addr_of_mut!(
                                                            (*(*__slate_slot_568)).a
                                                        )
                                                            as *mut ExprList_item
                                                    }
                                                    .offset(*__slate_slot_569 as isize)
                                                })
                                                .pExpr
                                            })
                                                as *const Expr,
                                            0 as i32,
                                        )
                                    },
                                )
                            };
                            transferJoinMarkings(*__slate_slot_571, *__slate_slot_547);
                            *__slate_slot_572 = whereClauseInsert(
                                pWC,
                                *__slate_slot_571,
                                (((2 as i32) | (1 as i32)) as i16) as u16,
                            );
                            {}
                            exprAnalyze(pSrc, pWC, *__slate_slot_572);
                            *__slate_slot_545 =
                                unsafe { unsafe { (*pWC).a }.offset(idxTerm as isize) };
                            markTermAsChild(pWC, *__slate_slot_572, idxTerm);
                            std::ptr::write(__slate_slot_896, *__slate_slot_569);
                            std::ptr::write(__slate_slot_897, *__slate_slot_896 + (1 as i32));
                            *__slate_slot_569 = *__slate_slot_897;
                        } else {
                            break '__join_23;
                        }
                    }
                }
                std::ptr::write(__slate_slot_561, unsafe {
                    sqlite3ExprSkipCollate(unsafe { (*(*__slate_slot_547)).pLeft })
                });
                std::ptr::write(__slate_slot_562, unsafe {
                    sqlite3ExprSkipCollate(unsafe { (*(*__slate_slot_547)).pRight })
                });
                std::ptr::write(
                    __slate_slot_563,
                    ((if (unsafe { (*(*__slate_slot_545)).prereqRight }) & *__slate_slot_548
                        == (((0 as i32) as i64) as u64)
                    {
                        16383 as i32
                    } else {
                        2048 as i32
                    }) as i16) as u16,
                );
                if (unsafe { (*(*__slate_slot_545)).u.x.iField }) > (0 as i32) {
                    0 as i32;
                    0 as i32;
                    0 as i32;
                    *__slate_slot_561 = unsafe {
                        (*unsafe {
                            unsafe {
                                std::ptr::addr_of_mut!(
                                    (*unsafe { (*(*__slate_slot_561)).x.pList }).a
                                ) as *mut ExprList_item
                            }
                            .offset(
                                ((unsafe { (*(*__slate_slot_545)).u.x.iField }) - (1 as i32))
                                    as isize,
                            )
                        })
                        .pExpr
                    };
                }
                if exprMightBeIndexed(
                    pSrc,
                    (*__slate_slot_560).as_mut_ptr() as *mut i32,
                    *__slate_slot_561,
                    *__slate_slot_554,
                ) != (0 as i32)
                {
                    unsafe {
                        (*(*__slate_slot_545)).leftCursor = unsafe {
                            *unsafe {
                                ((*__slate_slot_560).as_mut_ptr() as *mut i32)
                                    .offset((0 as i32) as isize)
                            }
                        };
                    }
                    0 as i32;
                    unsafe {
                        (*(*__slate_slot_545)).u.x.leftColumn = unsafe {
                            *unsafe {
                                ((*__slate_slot_560).as_mut_ptr() as *mut i32)
                                    .offset((1 as i32) as isize)
                            }
                        };
                    }
                    unsafe {
                        (*(*__slate_slot_545)).eOperator =
                            ((((operatorMask(*__slate_slot_554) as u32) as i32)
                                & ((*__slate_slot_563 as u32) as i32))
                                as i16) as u16;
                    }
                }
                if *__slate_slot_554 == (45 as i32) {
                    std::ptr::write(__slate_slot_876, *__slate_slot_545);
                    std::ptr::write(__slate_slot_877, unsafe { (*(*__slate_slot_876)).wtFlags });
                    std::ptr::write(
                        __slate_slot_878,
                        ((((*__slate_slot_877 as u32) as i32) | (2048 as i32)) as i16) as u16,
                    );
                    unsafe {
                        (*(*__slate_slot_876)).wtFlags = *__slate_slot_878;
                    }
                }
                if *__slate_slot_562 != std::ptr::null_mut::<Expr>() {
                    *__slate_slot_879 = exprMightBeIndexed(
                        pSrc,
                        (*__slate_slot_560).as_mut_ptr() as *mut i32,
                        *__slate_slot_562,
                        *__slate_slot_554,
                    ) != (0 as i32);
                } else {
                    *__slate_slot_879 = false as bool;
                }
                if *__slate_slot_879
                    && !((unsafe { (*(*__slate_slot_562)).flags }) & ((32 as i32) as u32)
                        != ((0 as i32) as u32))
                {
                    std::ptr::write(__slate_slot_566, ((0 as i32) as i16) as u16);
                    0 as i32;
                    if (unsafe { (*(*__slate_slot_545)).leftCursor }) >= (0 as i32) {
                        *__slate_slot_565 = unsafe {
                            sqlite3ExprDup(
                                *__slate_slot_556,
                                *__slate_slot_547 as *const Expr,
                                0 as i32,
                            )
                        };
                        if (unsafe { (*(*__slate_slot_556)).mallocFailed }) != (0 as u8) {
                            unsafe { sqlite3ExprDelete(*__slate_slot_556, *__slate_slot_565) };
                            return;
                        } else {
                            *__slate_slot_567 = whereClauseInsert(
                                pWC,
                                *__slate_slot_565,
                                (((2 as i32) | (1 as i32)) as i16) as u16,
                            );
                            if *__slate_slot_567 == (0 as i32) {
                                return;
                            } else {
                                *__slate_slot_564 = unsafe {
                                    unsafe { (*pWC).a }.offset(*__slate_slot_567 as isize)
                                };
                                markTermAsChild(pWC, *__slate_slot_567, idxTerm);
                                if *__slate_slot_554 == (45 as i32) {
                                    std::ptr::write(__slate_slot_880, *__slate_slot_564);
                                    std::ptr::write(__slate_slot_881, unsafe {
                                        (*(*__slate_slot_880)).wtFlags
                                    });
                                    std::ptr::write(
                                        __slate_slot_882,
                                        ((((*__slate_slot_881 as u32) as i32) | (2048 as i32))
                                            as i16) as u16,
                                    );
                                    unsafe {
                                        (*(*__slate_slot_880)).wtFlags = *__slate_slot_882;
                                    }
                                }
                                *__slate_slot_545 =
                                    unsafe { unsafe { (*pWC).a }.offset(idxTerm as isize) };
                                std::ptr::write(__slate_slot_883, *__slate_slot_545);
                                std::ptr::write(__slate_slot_884, unsafe {
                                    (*(*__slate_slot_883)).wtFlags
                                });
                                std::ptr::write(
                                    __slate_slot_885,
                                    ((((*__slate_slot_884 as u32) as i32) | (8 as i32)) as i16)
                                        as u16,
                                );
                                unsafe {
                                    (*(*__slate_slot_883)).wtFlags = *__slate_slot_885;
                                }
                                0 as i32;
                                if termIsEquivalence(*__slate_slot_555, *__slate_slot_565, unsafe {
                                    (*(*__slate_slot_544)).pTabList
                                }) != (0 as i32)
                                {
                                    std::ptr::write(__slate_slot_886, *__slate_slot_545);
                                    std::ptr::write(__slate_slot_887, unsafe {
                                        (*(*__slate_slot_886)).eOperator
                                    });
                                    std::ptr::write(
                                        __slate_slot_888,
                                        ((((*__slate_slot_887 as u32) as i32) | (2048 as i32))
                                            as i16) as u16,
                                    );
                                    unsafe {
                                        (*(*__slate_slot_886)).eOperator = *__slate_slot_888;
                                    }
                                    *__slate_slot_566 = ((2048 as i32) as i16) as u16;
                                }
                            }
                        }
                    } else {
                        *__slate_slot_565 = *__slate_slot_547;
                        *__slate_slot_564 = *__slate_slot_545;
                    }
                    std::ptr::write(__slate_slot_889, *__slate_slot_564);
                    std::ptr::write(__slate_slot_890, unsafe { (*(*__slate_slot_889)).wtFlags });
                    std::ptr::write(
                        __slate_slot_891,
                        ((((*__slate_slot_890 as u32) as i32)
                            | ((exprCommute(*__slate_slot_555, *__slate_slot_565) as u32) as i32))
                            as i16) as u16,
                    );
                    unsafe {
                        (*(*__slate_slot_889)).wtFlags = *__slate_slot_891;
                    }
                    unsafe {
                        (*(*__slate_slot_564)).leftCursor = unsafe {
                            *unsafe {
                                ((*__slate_slot_560).as_mut_ptr() as *mut i32)
                                    .offset((0 as i32) as isize)
                            }
                        };
                    }
                    0 as i32;
                    unsafe {
                        (*(*__slate_slot_564)).u.x.leftColumn = unsafe {
                            *unsafe {
                                ((*__slate_slot_560).as_mut_ptr() as *mut i32)
                                    .offset((1 as i32) as isize)
                            }
                        };
                    }
                    {}
                    unsafe {
                        (*(*__slate_slot_564)).prereqRight = *__slate_slot_548 | *__slate_slot_550;
                    }
                    unsafe {
                        (*(*__slate_slot_564)).prereqAll = *__slate_slot_549;
                    }
                    unsafe {
                        (*(*__slate_slot_564)).eOperator = ((((operatorMask(
                            ((unsafe { (*(*__slate_slot_565)).op }) as u32) as i32,
                        ) as u32)
                            as i32)
                            + ((*__slate_slot_566 as u32) as i32)
                            & ((*__slate_slot_563 as u32) as i32))
                            as i16)
                            as u16;
                    }
                } else {
                    if *__slate_slot_554 == (51 as i32)
                        && !((unsafe { (*(*__slate_slot_547)).flags }) & ((1 as i32) as u32)
                            != ((0 as i32) as u32))
                    {
                        *__slate_slot_892 = (0 as i32)
                            == unsafe { sqlite3ExprCanBeNull(*__slate_slot_561 as *const Expr) };
                    } else {
                        *__slate_slot_892 = false as bool;
                    }
                    if *__slate_slot_892 {
                        0 as i32;
                        unsafe {
                            (*(*__slate_slot_547)).op = ((171 as i32) as i8) as u8;
                        }
                        // See tag-20230504-1
                        unsafe {
                            (*(*__slate_slot_547)).u.zToken = b"false\0".as_ptr() as *mut i8;
                        }
                        std::ptr::write(__slate_slot_893, *__slate_slot_547);
                        std::ptr::write(__slate_slot_894, unsafe { (*(*__slate_slot_893)).flags });
                        std::ptr::write(
                            __slate_slot_895,
                            *__slate_slot_894 | ((536870912 as i32) as u32),
                        );
                        unsafe {
                            (*(*__slate_slot_893)).flags = *__slate_slot_895;
                        }
                        unsafe {
                            (*(*__slate_slot_545)).prereqAll = ((0 as i32) as i64) as u64;
                        }
                        unsafe {
                            (*(*__slate_slot_545)).eOperator = ((0 as i32) as i16) as u16;
                        }
                    }
                }
            }
            // If a term is the BETWEEN operator, create two new virtual terms
            // that define the range that the BETWEEN implements.  For example:
            //
            //      a BETWEEN b AND c
            //
            // is converted into:
            //
            //      (a BETWEEN b AND c) AND (a>=b) AND (a<=c)
            //
            // The two new terms are added onto the end of the WhereClause object.
            // The new terms are "dynamic" and are children of the original BETWEEN
            // term.  That means that if the BETWEEN term is coded, the children are
            // skipped.  Or, if the children are satisfied by an index, the original
            // BETWEEN term is skipped.
            // Analyze a term that is composed of two or more subterms connected by
            // an OR operator.
            // The form "x IS NOT NULL" can sometimes be evaluated more efficiently
            // as "x>NULL" if x is not an INTEGER PRIMARY KEY.  So construct a
            // virtual term of that form.
            //
            // The virtual term must be tagged with TERM_VNULL.
            // Add constraints to reduce the search space on a LIKE or GLOB
            // operator.
            //
            // A like pattern of the form "x LIKE 'aBc%'" is changed into constraints
            //
            //          x>='ABC' AND x<'abd' AND x LIKE 'aBc%'
            //
            // The last character of the prefix "abc" is incremented to form the
            // termination condition "abd".  If case is not significant (the default
            // for LIKE) then the lower-bound is made all uppercase and the upper-
            // bound is made all lowercase so that the bounds also work when comparing
            // BLOBs.
            if (((unsafe { (*(*__slate_slot_547)).op }) as u32) as i32) == (54 as i32)
                || (((unsafe { (*(*__slate_slot_547)).op }) as u32) as i32) == (45 as i32)
            {
                std::ptr::write(__slate_slot_915, unsafe {
                    sqlite3ExprVectorSize((unsafe { (*(*__slate_slot_547)).pLeft }) as *const Expr)
                });
                *__slate_slot_558 = *__slate_slot_915;
                *__slate_slot_914 = *__slate_slot_915 > (1 as i32);
            } else {
                *__slate_slot_914 = false as bool;
            }
            if *__slate_slot_914 {
                *__slate_slot_916 = (unsafe {
                    sqlite3ExprVectorSize((unsafe { (*(*__slate_slot_547)).pRight }) as *const Expr)
                }) == *__slate_slot_558;
            } else {
                *__slate_slot_916 = false as bool;
            }
            '__join_0: {
                if *__slate_slot_916
                    && ((unsafe { (*unsafe { (*(*__slate_slot_547)).pLeft }).flags })
                        & ((4096 as i32) as u32)
                        == ((0 as i32) as u32)
                        || (unsafe { (*unsafe { (*(*__slate_slot_547)).pRight }).flags })
                            & ((4096 as i32) as u32)
                            == ((0 as i32) as u32))
                    && (((unsafe { (*pWC).op }) as u32) as i32) == (44 as i32)
                {
                    *__slate_slot_589 = 0 as i32;
                    loop {
                        if *__slate_slot_589 < *__slate_slot_558 {
                            std::ptr::write(__slate_slot_592, unsafe {
                                sqlite3ExprForVectorField(
                                    *__slate_slot_555,
                                    unsafe { (*(*__slate_slot_547)).pLeft },
                                    *__slate_slot_589,
                                    *__slate_slot_558,
                                )
                            });
                            std::ptr::write(__slate_slot_593, unsafe {
                                sqlite3ExprForVectorField(
                                    *__slate_slot_555,
                                    unsafe { (*(*__slate_slot_547)).pRight },
                                    *__slate_slot_589,
                                    *__slate_slot_558,
                                )
                            });
                            *__slate_slot_591 = unsafe {
                                sqlite3PExpr(
                                    *__slate_slot_555,
                                    ((unsafe { (*(*__slate_slot_547)).op }) as u32) as i32,
                                    *__slate_slot_592,
                                    *__slate_slot_593,
                                )
                            };
                            transferJoinMarkings(*__slate_slot_591, *__slate_slot_547);
                            *__slate_slot_590 = whereClauseInsert(
                                pWC,
                                *__slate_slot_591,
                                (((1 as i32) | (32768 as i32)) as i16) as u16,
                            );
                            exprAnalyze(pSrc, pWC, *__slate_slot_590);
                            std::ptr::write(__slate_slot_917, *__slate_slot_589);
                            std::ptr::write(__slate_slot_918, *__slate_slot_917 + (1 as i32));
                            *__slate_slot_589 = *__slate_slot_918;
                        } else {
                            break;
                        }
                    }
                    *__slate_slot_545 = unsafe { unsafe { (*pWC).a }.offset(idxTerm as isize) };
                    std::ptr::write(__slate_slot_919, *__slate_slot_545);
                    std::ptr::write(__slate_slot_920, unsafe { (*(*__slate_slot_919)).wtFlags });
                    std::ptr::write(
                        __slate_slot_921,
                        ((((*__slate_slot_920 as u32) as i32) | ((4 as i32) | (2 as i32))) as i16)
                            as u16,
                    );
                    unsafe {
                        (*(*__slate_slot_919)).wtFlags = *__slate_slot_921;
                    }
                    // Disable the original
                    unsafe {
                        (*(*__slate_slot_545)).eOperator = ((8192 as i32) as i16) as u16;
                    }
                } else {
                    if (((unsafe { (*(*__slate_slot_547)).op }) as u32) as i32) == (50 as i32)
                        && (unsafe { (*(*__slate_slot_545)).u.x.iField }) == (0 as i32)
                        && (((unsafe { (*unsafe { (*(*__slate_slot_547)).pLeft }).op }) as u32)
                            as i32)
                            == (177 as i32)
                        && (unsafe { (*(*__slate_slot_547)).flags }) & ((4096 as i32) as u32)
                            != ((0 as i32) as u32)
                        && ((unsafe { (*unsafe { (*(*__slate_slot_547)).x.pSelect }).pPrior })
                            == std::ptr::null_mut::<Select>()
                            || (unsafe { (*unsafe { (*(*__slate_slot_547)).x.pSelect }).selFlags })
                                & ((512 as i32) as u32)
                                != (0 as u32))
                        && (unsafe { (*unsafe { (*(*__slate_slot_547)).x.pSelect }).pWin })
                            == std::ptr::null_mut::<Window>()
                        && (unsafe { (*unsafe { (*(*__slate_slot_547)).x.pSelect }).selFlags })
                            & ((4096 as i32) as u32)
                            == ((0 as i32) as u32)
                        && (((unsafe { (*pWC).op }) as u32) as i32) == (44 as i32)
                        && ((unsafe {
                            (*unsafe { (*unsafe { (*(*__slate_slot_547)).x.pSelect }).pEList })
                                .nExpr
                        }) as i64)
                            <= (((1 as i32) as i64)
                                << (1 as u64).wrapping_mul(((8 as i32) as i64) as u64))
                                - ((1 as i32) as i64)
                    {
                        0 as i32;
                        *__slate_slot_594 = 0 as i32;
                        loop {
                            if *__slate_slot_594
                                < unsafe {
                                    sqlite3ExprVectorSize(
                                        (unsafe { (*(*__slate_slot_547)).pLeft }) as *const Expr,
                                    )
                                }
                            {
                                *__slate_slot_595 = whereClauseInsert(
                                    pWC,
                                    *__slate_slot_547,
                                    (((2 as i32) | (32768 as i32)) as i16) as u16,
                                );
                                unsafe {
                                    (*unsafe {
                                        unsafe { (*pWC).a }.offset(*__slate_slot_595 as isize)
                                    })
                                    .u
                                    .x
                                    .iField = *__slate_slot_594 + (1 as i32);
                                }
                                exprAnalyze(pSrc, pWC, *__slate_slot_595);
                                markTermAsChild(pWC, *__slate_slot_595, idxTerm);
                                std::ptr::write(__slate_slot_922, *__slate_slot_594);
                                std::ptr::write(__slate_slot_923, *__slate_slot_922 + (1 as i32));
                                *__slate_slot_594 = *__slate_slot_923;
                            } else {
                                break '__join_0;
                            }
                        }
                    } else {
                        if (((unsafe { (*pWC).op }) as u32) as i32) == (44 as i32) {
                            std::ptr::write(__slate_slot_596, std::ptr::null_mut::<Expr>());
                            std::ptr::write(__slate_slot_597, std::ptr::null_mut::<Expr>());
                            std::ptr::write(
                                __slate_slot_598,
                                isAuxiliaryVtabOperator(
                                    *__slate_slot_556,
                                    *__slate_slot_547,
                                    std::ptr::addr_of_mut!(*__slate_slot_557),
                                    std::ptr::addr_of_mut!(*__slate_slot_597),
                                    std::ptr::addr_of_mut!(*__slate_slot_596),
                                ),
                            );
                            loop {
                                std::ptr::write(__slate_slot_924, *__slate_slot_598);
                                std::ptr::write(__slate_slot_925, *__slate_slot_924 - (1 as i32));
                                *__slate_slot_598 = *__slate_slot_925;
                                if *__slate_slot_924 > (0 as i32) {
                                    *__slate_slot_602 =
                                        sqlite3WhereExprUsage(*__slate_slot_546, *__slate_slot_596);
                                    *__slate_slot_601 =
                                        sqlite3WhereExprUsage(*__slate_slot_546, *__slate_slot_597);
                                    if *__slate_slot_602 & *__slate_slot_601
                                        == (((0 as i32) as i64) as u64)
                                    {
                                        *__slate_slot_603 = unsafe {
                                            sqlite3PExpr(
                                                *__slate_slot_555,
                                                47 as i32,
                                                std::ptr::null_mut::<Expr>(),
                                                unsafe {
                                                    sqlite3ExprDup(
                                                        *__slate_slot_556,
                                                        *__slate_slot_596 as *const Expr,
                                                        0 as i32,
                                                    )
                                                },
                                            )
                                        };
                                        if (unsafe { (*(*__slate_slot_547)).flags })
                                            & ((1 as i32) as u32)
                                            != ((0 as i32) as u32)
                                            && *__slate_slot_603 != std::ptr::null_mut::<Expr>()
                                        {
                                            std::ptr::write(__slate_slot_926, *__slate_slot_603);
                                            std::ptr::write(__slate_slot_927, unsafe {
                                                (*(*__slate_slot_926)).flags
                                            });
                                            std::ptr::write(
                                                __slate_slot_928,
                                                *__slate_slot_927 | ((1 as i32) as u32),
                                            );
                                            unsafe {
                                                (*(*__slate_slot_926)).flags = *__slate_slot_928;
                                            }
                                            unsafe {
                                                (*(*__slate_slot_603)).w.iJoin =
                                                    unsafe { (*(*__slate_slot_547)).w.iJoin };
                                            }
                                        }
                                        *__slate_slot_599 = whereClauseInsert(
                                            pWC,
                                            *__slate_slot_603,
                                            (((2 as i32) | (1 as i32)) as i16) as u16,
                                        );
                                        {}
                                        *__slate_slot_600 = unsafe {
                                            unsafe { (*pWC).a }.offset(*__slate_slot_599 as isize)
                                        };
                                        unsafe {
                                            (*(*__slate_slot_600)).prereqRight =
                                                *__slate_slot_602 | *__slate_slot_550;
                                        }
                                        unsafe {
                                            (*(*__slate_slot_600)).leftCursor =
                                                unsafe { (*(*__slate_slot_597)).iTable };
                                        }
                                        unsafe {
                                            (*(*__slate_slot_600)).u.x.leftColumn =
                                                (unsafe { (*(*__slate_slot_597)).iColumn }) as i32;
                                        }
                                        unsafe {
                                            (*(*__slate_slot_600)).eOperator =
                                                ((64 as i32) as i16) as u16;
                                        }
                                        unsafe {
                                            (*(*__slate_slot_600)).eMatchOp = *__slate_slot_557;
                                        }
                                        markTermAsChild(pWC, *__slate_slot_599, idxTerm);
                                        *__slate_slot_545 =
                                            unsafe { unsafe { (*pWC).a }.offset(idxTerm as isize) };
                                        std::ptr::write(__slate_slot_929, *__slate_slot_545);
                                        std::ptr::write(__slate_slot_930, unsafe {
                                            (*(*__slate_slot_929)).wtFlags
                                        });
                                        std::ptr::write(
                                            __slate_slot_931,
                                            ((((*__slate_slot_930 as u32) as i32) | (8 as i32))
                                                as i16)
                                                as u16,
                                        );
                                        unsafe {
                                            (*(*__slate_slot_929)).wtFlags = *__slate_slot_931;
                                        }
                                        unsafe {
                                            (*(*__slate_slot_600)).prereqAll =
                                                unsafe { (*(*__slate_slot_545)).prereqAll };
                                        }
                                    }
                                    std::ptr::write(__slate_slot_604, *__slate_slot_597);
                                    *__slate_slot_597 = *__slate_slot_596;
                                    *__slate_slot_596 = *__slate_slot_604;
                                    {}
                                } else {
                                    break '__join_0;
                                }
                            }
                        }
                    }
                }
            }
            // If there is a vector IN term - e.g. "(a, b) IN (SELECT ...)" - create
            // a virtual term for each vector component. The expression object
            // used by each such virtual term is pExpr (the full vector IN(...)
            // expression). The WhereTerm.u.x.iField variable identifies the index within
            // the vector on the LHS that the virtual term represents.
            //
            // This only works if the RHS is a simple SELECT (not a compound) that does
            // not use window functions.
            // ^-- See bug 2026-06-04T10:00:49Z
            // Add a WO_AUX auxiliary term to the constraint set if the
            // current expression is of the form "column OP expr" where OP
            // is an operator that gets passed into virtual tables but which is
            // not normally optimized for ordinary tables.  In other words, OP
            // is one of MATCH, LIKE, GLOB, REGEXP, !=, IS, IS NOT, or NOT NULL.
            // This information is used by the xBestIndex methods of
            // virtual tables.  The native query optimizer does not attempt
            // to do anything with MATCH functions.
            // Prevent ON clause terms of a LEFT JOIN from being used to drive
            // an index for tables to the left of the join.
            {}
            *__slate_slot_545 = unsafe { unsafe { (*pWC).a }.offset(idxTerm as isize) };
            std::ptr::write(__slate_slot_932, *__slate_slot_545);
            std::ptr::write(__slate_slot_933, unsafe {
                (*(*__slate_slot_932)).prereqRight
            });
            std::ptr::write(__slate_slot_934, *__slate_slot_933 | *__slate_slot_550);
            unsafe {
                (*(*__slate_slot_932)).prereqRight = *__slate_slot_934;
            }
        }
    }
}

static mut ops: [u8; 2] = [((58 as i32) as i8) as u8, ((56 as i32) as i8) as u8];

// Routines with file scope above.  Interface to the rest of the where.c
// subsystem follows.
/// This routine identifies subexpressions in the WHERE clause where
/// each subexpression is separated by the AND operator or some other
/// operator specified in the op parameter.  The WhereClause structure
/// is filled with pointers to subexpressions.  For example:
///
///    WHERE  a=='hello' AND coalesce(b,11)<10 AND (c+12!=d OR c==22)
///           \________/     \_______________/     \________________/
///            slot[0]            slot[1]               slot[2]
///
/// The original WHERE clause in pExpr is unaltered.  All this routine
/// does is make slot[] entries point to substructure within pExpr.
///
/// In the previous sentence and in the diagram, "slot[]" refers to
/// the WhereClause.a[] array.  The slot[] array grows as needed to contain
/// all terms of the WHERE clause.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3WhereSplit(
    mut pWC: *mut WhereClause,
    mut pExpr: *mut Expr,
    mut op: u8,
) {
    let mut pE2: *mut Expr = unsafe { sqlite3ExprSkipCollateAndLikely(pExpr) };
    unsafe {
        (*pWC).op = op;
    }
    0 as i32;
    if pE2 == std::ptr::null_mut::<Expr>() {
        return;
    }
    if (((unsafe { (*pE2).op }) as u32) as i32) != ((op as u32) as i32) {
        whereClauseInsert(pWC, pExpr, ((0 as i32) as i16) as u16);
    } else {
        sqlite3WhereSplit(pWC, unsafe { (*pE2).pLeft }, op);
        sqlite3WhereSplit(pWC, unsafe { (*pE2).pRight }, op);
    }
}

/// Add either a LIMIT (if eMatchOp==SQLITE_INDEX_CONSTRAINT_LIMIT) or
/// OFFSET (if eMatchOp==SQLITE_INDEX_CONSTRAINT_OFFSET) term to the
/// where-clause passed as the first argument. The value for the term
/// is found in register iReg.
///
/// In the common case where the value is a simple integer
/// (example: "LIMIT 5 OFFSET 10") then the expression codes as a
/// TK_INTEGER so that it will be available to sqlite3_vtab_rhs_value().
/// If not, then it codes as a TK_REGISTER expression.
///
/// # Arguments
///
/// * `pWC` - Add the constraint to this WHERE clause
/// * `iReg` - Register that will hold value of the limit/offset
/// * `pExpr` - Expression that defines the limit/offset
/// * `iCsr` - Cursor to which the constraint applies
/// * `eMatchOp` - SQLITE_INDEX_CONSTRAINT_LIMIT or _OFFSET
fn whereAddLimitExpr(
    mut pWC: *mut WhereClause,
    mut iReg: i32,
    mut pExpr: *mut Expr,
    mut iCsr: i32,
    mut eMatchOp: i32,
) {
    let mut pParse: *mut Parse = unsafe { (*unsafe { (*pWC).pWInfo }).pParse };
    let mut db: *mut sqlite3 = unsafe { (*pParse).db };
    let mut pNew: *mut Expr = unsafe { std::mem::zeroed() };
    let mut iVal: i32 = 0 as i32;
    if (unsafe {
        sqlite3ExprIsInteger(
            pExpr as *const Expr,
            std::ptr::addr_of_mut!(iVal),
            pParse,
            0 as i32,
        )
    }) != (0 as i32)
        && iVal >= (0 as i32)
    {
        let mut pVal: *mut Expr = unsafe { sqlite3ExprInt32(db, iVal) };
        if pVal == std::ptr::null_mut::<Expr>() {
            return;
        }
        pNew = unsafe { sqlite3PExpr(pParse, 47 as i32, std::ptr::null_mut::<Expr>(), pVal) };
    } else {
        let mut pVal: *mut Expr =
            unsafe { sqlite3ExprAlloc(db, 176 as i32, std::ptr::null::<Token>(), 0 as i32) };
        if pVal == std::ptr::null_mut::<Expr>() {
            return;
        }
        unsafe {
            (*pVal).iTable = iReg;
        }
        pNew = unsafe { sqlite3PExpr(pParse, 47 as i32, std::ptr::null_mut::<Expr>(), pVal) };
    }
    if pNew != std::ptr::null_mut::<Expr>() {
        let mut pTerm: *mut WhereTerm = unsafe { std::mem::zeroed() };
        let mut idx: i32 = 0 as i32;
        idx = whereClauseInsert(pWC, pNew, (((1 as i32) | (2 as i32)) as i16) as u16);
        pTerm = unsafe { unsafe { (*pWC).a }.offset(idx as isize) };
        unsafe {
            (*pTerm).leftCursor = iCsr;
        }
        unsafe {
            (*pTerm).eOperator = ((64 as i32) as i16) as u16;
        }
        unsafe {
            (*pTerm).eMatchOp = (eMatchOp as i8) as u8;
        }
    }
}

/// Possibly add terms corresponding to the LIMIT and OFFSET clauses of the
/// SELECT statement passed as the second argument. These terms are only
/// added if:
///
///   1. The SELECT statement has a LIMIT clause, and
///   2. The SELECT statement is not an aggregate or DISTINCT query, and
///   3. The SELECT statement has exactly one object in its FROM clause, and
///      that object is a virtual table, and
///   4. There are no terms in the WHERE clause that will not be passed
///      to the virtual table xBestIndex method.
///   5. The ORDER BY clause, if any, will be made available to the xBestIndex
///      method.
///
/// LIMIT and OFFSET terms are ignored by most of the planner code. They
/// exist only so that they may be passed to the xBestIndex method of the
/// single virtual table in the FROM clause of the SELECT.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3WhereAddLimit(mut pWC: *mut WhereClause, mut p: *mut Select) {
    0 as i32; // 1 -- checked by caller
    if (unsafe { (*p).pGroupBy }) == std::ptr::null_mut::<ExprList>()
        && (unsafe { (*p).selFlags }) & (((1 as i32) | (8 as i32)) as u32) == ((0 as i32) as u32)
        && ((unsafe { (*unsafe { (*p).pSrc }).nSrc }) == (1 as i32)
            && (((unsafe {
                (*unsafe {
                    (*unsafe {
                        unsafe { std::ptr::addr_of_mut!((*unsafe { (*p).pSrc }).a) as *mut SrcItem }
                            .offset((0 as i32) as isize)
                    })
                    .pSTab
                })
                .eTabType
            }) as u32) as i32)
                == (1 as i32))
    {
        let mut pOrderBy: *mut ExprList = unsafe { (*p).pOrderBy };
        let mut iCsr: i32 = unsafe {
            (*unsafe {
                unsafe { std::ptr::addr_of_mut!((*unsafe { (*p).pSrc }).a) as *mut SrcItem }
                    .offset((0 as i32) as isize)
            })
            .iCursor
        };
        let mut ii: i32 = 0 as i32;
        // Check condition (4). Return early if it is not met.
        ii = 0 as i32;
        '__slate_break_835: loop {
            if !(ii < unsafe { (*pWC).nTerm }) {
                break;
            }
            '__slate_continue_835: {
                if (((unsafe { (*unsafe { unsafe { (*pWC).a }.offset(ii as isize) }).wtFlags })
                    as u32) as i32)
                    & (4 as i32)
                    != (0 as i32)
                {
                    // This term is a vector operation that has been decomposed into
                    // other, subsequent terms.  It can be ignored. See tag-20220128a
                    0 as i32;
                    0 as i32;
                } else {
                    if (unsafe { (*unsafe { unsafe { (*pWC).a }.offset(ii as isize) }).nChild })
                        != (0 as u8)
                    {
                        // If this term has child terms, then they are also part of the
                        // pWC->a[] array. So this term can be ignored, as a LIMIT clause
                        // will only be added if each of the child terms passes the
                        // (leftCursor==iCsr) test below.
                    } else {
                        if (unsafe {
                            (*unsafe { unsafe { (*pWC).a }.offset(ii as isize) }).leftCursor
                        }) == iCsr
                            && (unsafe {
                                (*unsafe { unsafe { (*pWC).a }.offset(ii as isize) }).prereqRight
                            }) == (((0 as i32) as i64) as u64)
                        {
                        } else {
                            // If this term has a parent with exactly one child, and the parent will
                            // be passed through to xBestIndex, then this term can be ignored.
                            if (unsafe {
                                (*unsafe { unsafe { (*pWC).a }.offset(ii as isize) }).iParent
                            }) >= (0 as i32)
                            {
                                let mut pParent: *mut WhereTerm = unsafe {
                                    unsafe { (*pWC).a }.offset(
                                        (unsafe {
                                            (*unsafe { unsafe { (*pWC).a }.offset(ii as isize) })
                                                .iParent
                                        }) as isize,
                                    )
                                };
                                if (unsafe { (*pParent).leftCursor }) == iCsr
                                    && (unsafe { (*pParent).prereqRight })
                                        == (((0 as i32) as i64) as u64)
                                    && (((unsafe { (*pParent).nChild }) as u32) as i32)
                                        == (1 as i32)
                                {
                                    break '__slate_continue_835;
                                }
                            }
                            // This term will not be passed through. Do not add a LIMIT clause.
                            return;
                        }
                    }
                }
            }
            let __v848: i32 = ii;
            let __v849: i32 = __v848 + (1 as i32);
            ii = __v849;
        }
        // Check condition (5). Return early if it is not met.
        if pOrderBy != std::ptr::null_mut::<ExprList>() {
            ii = 0 as i32;
            '__slate_break_836: loop {
                if !(ii < unsafe { (*pOrderBy).nExpr }) {
                    break;
                }
                let mut pExpr: *mut Expr = unsafe {
                    (*unsafe {
                        unsafe { std::ptr::addr_of_mut!((*pOrderBy).a) as *mut ExprList_item }
                            .offset(ii as isize)
                    })
                    .pExpr
                };
                if (((unsafe { (*pExpr).op }) as u32) as i32) != (168 as i32) {
                    return;
                }
                if (unsafe { (*pExpr).iTable }) != iCsr {
                    return;
                }
                if (((unsafe {
                    (*unsafe {
                        unsafe { std::ptr::addr_of_mut!((*pOrderBy).a) as *mut ExprList_item }
                            .offset(ii as isize)
                    })
                    .fg
                    .sortFlags
                }) as u32) as i32)
                    & (2 as i32)
                    != (0 as i32)
                {
                    return;
                }
                let __v850: i32 = ii;
                let __v851: i32 = __v850 + (1 as i32);
                ii = __v851;
            }
        }
        // All conditions are met. Add the terms to the where-clause object.
        0 as i32;
        if (unsafe { (*p).iOffset }) != (0 as i32)
            && (unsafe { (*p).selFlags }) & ((256 as i32) as u32) == ((0 as i32) as u32)
        {
            whereAddLimitExpr(
                pWC,
                unsafe { (*p).iOffset },
                unsafe { (*unsafe { (*p).pLimit }).pRight },
                iCsr,
                74 as i32,
            );
        }
        if (unsafe { (*p).iOffset }) == (0 as i32)
            || (unsafe { (*p).selFlags }) & ((256 as i32) as u32) == ((0 as i32) as u32)
        {
            whereAddLimitExpr(
                pWC,
                unsafe { (*p).iLimit },
                unsafe { (*unsafe { (*p).pLimit }).pLeft },
                iCsr,
                73 as i32,
            );
        }
    }
    // 2
    // 3
}

/// Initialize a preallocated WhereClause structure.
///
/// # Arguments
///
/// * `pWC` - The WhereClause to be initialized
/// * `pWInfo` - The WHERE processing context
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3WhereClauseInit(mut pWC: *mut WhereClause, mut pWInfo: *mut WhereInfo) {
    unsafe {
        (*pWC).pWInfo = pWInfo;
    }
    unsafe {
        (*pWC).hasOr = ((0 as i32) as i8) as u8;
    }
    unsafe {
        (*pWC).pOuter = std::ptr::null_mut::<WhereClause>();
    }
    unsafe {
        (*pWC).nTerm = 0 as i32;
    }
    unsafe {
        (*pWC).nBase = 0 as i32;
    }
    unsafe {
        (*pWC).nSlot = (((448 as u64) / (56 as u64)) as u32) as i32;
    }
    unsafe {
        (*pWC).a = unsafe { (*pWC).aStatic.as_mut_ptr() as *mut WhereTerm };
    }
}

/// Deallocate a WhereClause structure.  The WhereClause structure
/// itself is not freed.  This routine is the inverse of
/// sqlite3WhereClauseInit().
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3WhereClauseClear(mut pWC: *mut WhereClause) {
    let mut db: *mut sqlite3 = unsafe { (*unsafe { (*unsafe { (*pWC).pWInfo }).pParse }).db };
    0 as i32;
    if (unsafe { (*pWC).nTerm }) > (0 as i32) {
        let mut a: *mut WhereTerm = unsafe { (*pWC).a };
        let mut aLast: *mut WhereTerm = unsafe {
            unsafe { (*pWC).a }.offset(((unsafe { (*pWC).nTerm }) - (1 as i32)) as isize)
        };
        '__slate_break_837: while (1 as i32) != (0 as i32) {
            0 as i32;
            if (((unsafe { (*a).wtFlags }) as u32) as i32) & (1 as i32) != (0 as i32) {
                unsafe { sqlite3ExprDelete(db, unsafe { (*a).pExpr }) };
            }
            if (((unsafe { (*a).wtFlags }) as u32) as i32) & ((16 as i32) | (32 as i32))
                != (0 as i32)
            {
                if (((unsafe { (*a).wtFlags }) as u32) as i32) & (16 as i32) != (0 as i32) {
                    0 as i32;
                    whereOrInfoDelete(db, unsafe { (*a).u.pOrInfo });
                } else {
                    0 as i32;
                    whereAndInfoDelete(db, unsafe { (*a).u.pAndInfo });
                }
            }
            if a == aLast {
                break '__slate_break_837;
            }
            let __v846: *mut WhereTerm = a;
            let __v847: *mut WhereTerm = unsafe { __v846.offset((1 as i32) as isize) };
            a = __v847;
        }
    }
}

/// These routines walk (recursively) an expression tree and generate
/// a bitmask indicating which tables are used in that expression
/// tree.
///
/// sqlite3WhereExprUsage(MaskSet, Expr) ->
///
///       Return a Bitmask of all tables referenced by Expr.  Expr can be
///       be NULL, in which case 0 is returned.
///
/// sqlite3WhereExprUsageNN(MaskSet, Expr) ->
///
///       Same as sqlite3WhereExprUsage() except that Expr must not be
///       NULL.  The "NN" suffix on the name stands for "Not Null".
///
/// sqlite3WhereExprListUsage(MaskSet, ExprList) ->
///
///       Return a Bitmask of all tables referenced by every expression
///       in the expression list ExprList.  ExprList can be NULL, in which
///       case 0 is returned.
///
/// sqlite3WhereExprUsageFull(MaskSet, ExprList) ->
///
///       Internal use only.  Called only by sqlite3WhereExprUsageNN() for
///       complex expressions that require pushing register values onto
///       the stack.  Many calls to sqlite3WhereExprUsageNN() do not need
///       the more complex analysis done by this routine.  Hence, the
///       computations done by this routine are broken out into a separate
///       "no-inline" function to avoid the stack push overhead in the
///       common case where it is not needed.
fn sqlite3WhereExprUsageFull(mut pMaskSet: *mut WhereMaskSet, mut p: *mut Expr) -> u64 {
    let mut mask: u64 = 0 as u64;
    let __v1063: u64;
    if (((unsafe { (*p).op }) as u32) as i32) == (179 as i32) {
        __v1063 = unsafe { sqlite3WhereGetMask(pMaskSet, unsafe { (*p).iTable }) };
    } else {
        __v1063 = ((0 as i32) as i64) as u64;
    }
    mask = __v1063;
    if (unsafe { (*p).pLeft }) != std::ptr::null_mut::<Expr>() {
        let __v1064: u64 = mask;
        let __v1065: u64 = __v1064 | sqlite3WhereExprUsageNN(pMaskSet, unsafe { (*p).pLeft });
        mask = __v1065;
    }
    if (unsafe { (*p).pRight }) != std::ptr::null_mut::<Expr>() {
        let __v1066: u64 = mask;
        let __v1067: u64 = __v1066 | sqlite3WhereExprUsageNN(pMaskSet, unsafe { (*p).pRight });
        mask = __v1067;
        0 as i32;
    } else {
        if (unsafe { (*p).flags }) & ((4096 as i32) as u32) != ((0 as i32) as u32) {
            if (unsafe { (*p).flags }) & ((64 as i32) as u32) != ((0 as i32) as u32) {
                unsafe {
                    (*pMaskSet).bVarSelect = 1 as i32;
                }
            }
            let __v1068: u64 = mask;
            let __v1069: u64 = __v1068 | exprSelectUsage(pMaskSet, unsafe { (*p).x.pSelect });
            mask = __v1069;
        } else {
            if (unsafe { (*p).x.pList }) != std::ptr::null_mut::<ExprList>() {
                let __v1070: u64 = mask;
                let __v1071: u64 =
                    __v1070 | sqlite3WhereExprListUsage(pMaskSet, unsafe { (*p).x.pList });
                mask = __v1071;
            }
        }
    }
    if ((((unsafe { (*p).op }) as u32) as i32) == (172 as i32)
        || (((unsafe { (*p).op }) as u32) as i32) == (169 as i32))
        && (unsafe { (*p).flags }) & ((16777216 as i32) as u32) != ((0 as i32) as u32)
    {
        0 as i32;
        let __v1072: u64 = mask;
        let __v1073: u64 = __v1072
            | sqlite3WhereExprListUsage(pMaskSet, unsafe { (*unsafe { (*p).y.pWin }).pPartition });
        mask = __v1073;
        let __v1074: u64 = mask;
        let __v1075: u64 = __v1074
            | sqlite3WhereExprListUsage(pMaskSet, unsafe { (*unsafe { (*p).y.pWin }).pOrderBy });
        mask = __v1075;
        let __v1076: u64 = mask;
        let __v1077: u64 =
            __v1076 | sqlite3WhereExprUsage(pMaskSet, unsafe { (*unsafe { (*p).y.pWin }).pFilter });
        mask = __v1077;
    }
    return mask;
}

#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3WhereExprUsageNN(
    mut pMaskSet: *mut WhereMaskSet,
    mut p: *mut Expr,
) -> u64 {
    if (((unsafe { (*p).op }) as u32) as i32) == (168 as i32)
        && !((unsafe { (*p).flags }) & ((32 as i32) as u32) != ((0 as i32) as u32))
    {
        return unsafe { sqlite3WhereGetMask(pMaskSet, unsafe { (*p).iTable }) };
    } else {
        if (unsafe { (*p).flags }) & (((65536 as i32) | (8388608 as i32)) as u32)
            != ((0 as i32) as u32)
        {
            0 as i32;
            return ((0 as i32) as i64) as u64;
        }
    }
    return sqlite3WhereExprUsageFull(pMaskSet, p);
}

#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3WhereExprUsage(
    mut pMaskSet: *mut WhereMaskSet,
    mut p: *mut Expr,
) -> u64 {
    let __v852: u64;
    if p != std::ptr::null_mut::<Expr>() {
        __v852 = sqlite3WhereExprUsageNN(pMaskSet, p);
    } else {
        __v852 = ((0 as i32) as i64) as u64;
    }
    return __v852;
}

#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3WhereExprListUsage(
    mut pMaskSet: *mut WhereMaskSet,
    mut pList: *mut ExprList,
) -> u64 {
    let mut i: i32 = 0 as i32;
    let mut mask: u64 = ((0 as i32) as i64) as u64;
    if pList != std::ptr::null_mut::<ExprList>() {
        i = 0 as i32;
        '__slate_break_838: loop {
            if !(i < unsafe { (*pList).nExpr }) {
                break;
            }
            let __v855: u64 = mask;
            let __v856: u64 = __v855
                | sqlite3WhereExprUsage(pMaskSet, unsafe {
                    (*unsafe {
                        unsafe { std::ptr::addr_of_mut!((*pList).a) as *mut ExprList_item }
                            .offset(i as isize)
                    })
                    .pExpr
                });
            mask = __v856;
            let __v853: i32 = i;
            let __v854: i32 = __v853 + (1 as i32);
            i = __v854;
        }
    }
    return mask;
}

/// Call exprAnalyze on all terms in a WHERE clause.
///
/// Note that exprAnalyze() might add new virtual terms onto the
/// end of the WHERE clause.  We do not want to analyze these new
/// virtual terms, so start analyzing at the end and work forward
/// so that the added virtual terms are never processed.
///
/// # Arguments
///
/// * `pTabList` - the FROM clause
/// * `pWC` - the WHERE clause to be analyzed
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3WhereExprAnalyze(
    mut pTabList: *mut SrcList,
    mut pWC: *mut WhereClause,
) {
    let mut i: i32 = 0 as i32;
    i = (unsafe { (*pWC).nTerm }) - (1 as i32);
    '__slate_break_839: loop {
        if !(i >= (0 as i32)) {
            break;
        }
        exprAnalyze(pTabList, pWC, i);
        let __v857: i32 = i;
        let __v858: i32 = __v857 - (1 as i32);
        i = __v858;
    }
}

/// For table-valued-functions, transform the function arguments into
/// new WHERE clause terms.
///
/// Each function argument translates into an equality constraint against
/// a HIDDEN column in the table.
///
/// # Arguments
///
/// * `pParse` - Parsing context
/// * `pItem` - The FROM clause term to process
/// * `pWC` - Xfer function arguments to here
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3WhereTabFuncArgs(
    mut pParse: *mut Parse,
    mut pItem: *mut SrcItem,
    mut pWC: *mut WhereClause,
) {
    let mut pTab: *mut Table = unsafe { std::mem::zeroed() };
    let mut j: i32 = 0 as i32;
    let mut k: i32 = 0 as i32;
    let mut pArgs: *mut ExprList = unsafe { std::mem::zeroed() };
    let mut pColRef: *mut Expr = unsafe { std::mem::zeroed() };
    let mut pTerm: *mut Expr = unsafe { std::mem::zeroed() };
    if ((unsafe { (*pItem).fg.__slate_bits_0.__get_isTabFunc() }) as i32) == (0 as i32) {
        return;
    }
    pTab = unsafe { (*pItem).pSTab };
    0 as i32;
    if !((((unsafe { (*pTab).eTabType }) as u32) as i32) == (1 as i32)) {
        unsafe {
            sqlite3ErrorMsg(
                pParse,
                (b"'%s' is not a function\0".as_ptr() as *mut i8) as *const i8,
                unsafe { (*pItem).zName },
            )
        };
        return;
    }
    pArgs = unsafe { (*pItem).u1.pFuncArg };
    if pArgs == std::ptr::null_mut::<ExprList>() {
        return;
    }
    k = 0 as i32;
    j = 0 as i32;
    '__slate_break_841: loop {
        if !(j < unsafe { (*pArgs).nExpr }) {
            break;
        }
        let mut pRhs: *mut Expr = unsafe { std::mem::zeroed() };
        let mut joinType: u32 = 0 as u32;
        '__slate_break_842: while k < ((unsafe { (*pTab).nCol }) as i32)
            && (((unsafe { (*unsafe { unsafe { (*pTab).aCol }.offset(k as isize) }).colFlags })
                as u32) as i32)
                & (2 as i32)
                == (0 as i32)
        {
            let __v861: i32 = k;
            let __v862: i32 = __v861 + (1 as i32);
            k = __v862;
        }
        if k >= ((unsafe { (*pTab).nCol }) as i32) {
            unsafe {
                sqlite3ErrorMsg(
                    pParse,
                    (b"too many arguments on %s() - max %d\0".as_ptr() as *mut i8) as *const i8,
                    unsafe { (*pTab).zName },
                    j,
                )
            };
            return;
        }
        pColRef = unsafe {
            sqlite3ExprAlloc(
                unsafe { (*pParse).db },
                168 as i32,
                std::ptr::null::<Token>(),
                0 as i32,
            )
        };
        if pColRef == std::ptr::null_mut::<Expr>() {
            return;
        }
        unsafe {
            (*pColRef).iTable = unsafe { (*pItem).iCursor };
        }
        let __v863: i32 = k;
        let __v864: i32 = __v863 + (1 as i32);
        k = __v864;
        unsafe {
            (*pColRef).iColumn = __v863 as i16;
        }
        0 as i32;
        unsafe {
            (*pColRef).y.pTab = pTab;
        }
        let __v865: *mut SrcItem = pItem;
        let __v866: u64 = unsafe { (*__v865).colUsed };
        let __v867: u64 = __v866 | unsafe { sqlite3ExprColUsed(pColRef) };
        unsafe {
            (*__v865).colUsed = __v867;
        }
        pRhs = unsafe {
            sqlite3PExpr(
                pParse,
                173 as i32,
                unsafe {
                    sqlite3ExprDup(
                        unsafe { (*pParse).db },
                        (unsafe {
                            (*unsafe {
                                unsafe { std::ptr::addr_of_mut!((*pArgs).a) as *mut ExprList_item }
                                    .offset(j as isize)
                            })
                            .pExpr
                        }) as *const Expr,
                        0 as i32,
                    )
                },
                std::ptr::null_mut::<Expr>(),
            )
        };
        pTerm = unsafe { sqlite3PExpr(pParse, 54 as i32, pColRef, pRhs) };
        if (((unsafe { (*pItem).fg.jointype }) as u32) as i32) & ((8 as i32) | (16 as i32))
            != (0 as i32)
        {
            {}
            // testtag-20230227a
            {}
            // testtag-20230227b
            joinType = (1 as i32) as u32;
        } else {
            {}
            // testtag-20230227c
            joinType = (2 as i32) as u32;
        }
        unsafe { sqlite3SetJoinExpr(pTerm, unsafe { (*pItem).iCursor }, joinType) };
        whereClauseInsert(pWC, pTerm, ((1 as i32) as i16) as u16);
        let __v859: i32 = j;
        let __v860: i32 = __v859 + (1 as i32);
        j = __v860;
    }
}

/// Return true if the WhereLoop pLoop can be use a Bloom filter.
/// tag-202607231411
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3WhereLoopBloomable(mut pLoop: *const WhereLoop) -> i32 {
    if (unsafe { (*pLoop).wsFlags }) & ((256 as i32) as u32) != (0 as u32) {
        return 1 as i32;
    }
    if (unsafe { (*pLoop).wsFlags }) & ((512 as i32) as u32) == ((0 as i32) as u32) {
        return 0 as i32;
    }
    return unsafe {
        sqlite3IndexBloomable(
            (unsafe { (*pLoop).u.btree.pIndex }) as *const Index,
            ((unsafe { (*pLoop).u.btree.nEq }) as u32) as i32,
        )
    };
}
