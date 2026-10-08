//! 2015-06-06
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
//! This file was split off from where.c on 2015-06-06 in order to reduce the
//! size of where.c and make it easier to edit.  This file contains the routines
//! that actually generate the bulk of the WHERE loop code.  The original where.c
//! file retains the code that does query planning and analysis.
unsafe extern "C" {
    fn sqlite3_str_appendf(__v777: *mut sqlite3_str, zFormat: *const i8, ...);
    fn sqlite3_str_append(__v779: *mut sqlite3_str, zIn: *const i8, N: i32);
    fn sqlite3_str_appendall(__v782: *mut sqlite3_str, zIn: *const i8);
    fn memcpy(__dest: *mut (), __src: *const (), __n: u64) -> *mut ();
    fn sqlite3VdbeAddOp0(__v787: *mut Vdbe, __v788: i32) -> i32;
    fn sqlite3VdbeAddOp1(__v789: *mut Vdbe, __v790: i32, __v791: i32) -> i32;
    fn sqlite3VdbeAddOp2(__v792: *mut Vdbe, __v793: i32, __v794: i32, __v795: i32) -> i32;
    fn sqlite3VdbeGoto(__v796: *mut Vdbe, __v797: i32) -> i32;
    fn sqlite3VdbeAddOp3(
        __v798: *mut Vdbe,
        __v799: i32,
        __v800: i32,
        __v801: i32,
        __v802: i32,
    ) -> i32;
    fn sqlite3VdbeAddOp4(
        __v803: *mut Vdbe,
        __v804: i32,
        __v805: i32,
        __v806: i32,
        __v807: i32,
        zP4: *const i8,
        __v809: i32,
    ) -> i32;
    fn sqlite3VdbeAddOp4Int(
        __v810: *mut Vdbe,
        __v811: i32,
        __v812: i32,
        __v813: i32,
        __v814: i32,
        __v815: i32,
    ) -> i32;
    fn sqlite3VdbeExplain(__v816: *mut Parse, __v817: u8, __v818: *const i8, ...) -> i32;
    fn sqlite3VdbeExplainPop(__v819: *mut Parse);
    fn sqlite3VdbeChangeP1(__v820: *mut Vdbe, addr: i32, P1: i32);
    fn sqlite3VdbeChangeP2(__v823: *mut Vdbe, addr: i32, P2: i32);
    fn sqlite3VdbeChangeP5(__v826: *mut Vdbe, P5: u16);
    fn sqlite3VdbeJumpHere(__v828: *mut Vdbe, addr: i32);
    fn sqlite3VdbeChangeP4(__v830: *mut Vdbe, addr: i32, zP4: *const i8, N: i32);
    fn sqlite3VdbeSetP4KeyInfo(__v834: *mut Parse, __v835: *mut Index);
    fn sqlite3VdbeGetOp(__v836: *mut Vdbe, __v837: i32) -> *mut VdbeOp;
    fn sqlite3VdbeGetLastOp(__v838: *mut Vdbe) -> *mut VdbeOp;
    fn sqlite3VdbeMakeLabel(__v839: *mut Parse) -> i32;
    fn sqlite3VdbeResolveLabel(__v840: *mut Vdbe, __v841: i32);
    fn sqlite3VdbeCurrentAddr(__v842: *mut Vdbe) -> i32;
    fn sqlite3VdbeComment(__v843: *mut Vdbe, __v844: *const i8, ...);
    fn sqlite3DbMallocZero(__v845: *mut sqlite3, __v846: u64) -> *mut ();
    fn sqlite3DbMallocRawNN(__v847: *mut sqlite3, __v848: u64) -> *mut ();
    fn sqlite3DbStrDup(__v849: *mut sqlite3, __v850: *const i8) -> *mut i8;
    fn sqlite3DbFree(__v851: *mut sqlite3, __v852: *mut ());
    fn sqlite3DbFreeNN(__v853: *mut sqlite3, __v854: *mut ());
    fn sqlite3DbNNFreeNN(__v855: *mut sqlite3, __v856: *mut ());
    fn sqlite3ErrorMsg(__v857: *mut Parse, __v858: *const i8, ...);
    fn sqlite3GetTempReg(__v859: *mut Parse) -> i32;
    fn sqlite3ReleaseTempReg(__v860: *mut Parse, __v861: i32);
    fn sqlite3GetTempRange(__v862: *mut Parse, __v863: i32) -> i32;
    fn sqlite3ReleaseTempRange(__v864: *mut Parse, __v865: i32, __v866: i32);
    fn sqlite3Expr(__v867: *mut sqlite3, __v868: i32, __v869: *const i8) -> *mut Expr;
    fn sqlite3PExpr(
        __v870: *mut Parse,
        __v871: i32,
        __v872: *mut Expr,
        __v873: *mut Expr,
    ) -> *mut Expr;
    fn sqlite3ExprAnd(__v874: *mut Parse, __v875: *mut Expr, __v876: *mut Expr) -> *mut Expr;
    fn sqlite3ExprDelete(__v877: *mut sqlite3, __v878: *mut Expr);
    fn sqlite3ExprListAppend(
        __v879: *mut Parse,
        __v880: *mut ExprList,
        __v881: *mut Expr,
    ) -> *mut ExprList;
    fn sqlite3ExprListDelete(__v882: *mut sqlite3, __v883: *mut ExprList);
    fn sqlite3PrimaryKeyIndex(__v884: *mut Table) -> *mut Index;
    fn sqlite3TableColumnToIndex(__v885: *mut Index, __v886: i32) -> i32;
    fn sqlite3TableColumnToStorage(__v887: *mut Table, __v888: i16) -> i16;
    fn sqlite3WhereBegin(
        __v889: *mut Parse,
        __v890: *mut SrcList,
        __v891: *mut Expr,
        __v892: *mut ExprList,
        __v893: *mut ExprList,
        __v894: *mut Select,
        __v895: u16,
        __v896: i32,
    ) -> *mut WhereInfo;
    fn sqlite3WhereEnd(__v897: *mut WhereInfo);
    fn sqlite3WhereContinueLabel(__v898: *mut WhereInfo) -> i32;
    fn sqlite3WhereUsesDeferredSeek(__v899: *mut WhereInfo) -> i32;
    fn sqlite3ExprCodeGetColumnOfTable(
        __v900: *mut Vdbe,
        __v901: *mut Table,
        __v902: i32,
        __v903: i32,
        __v904: i32,
    );
    fn sqlite3ExprCode(__v905: *mut Parse, __v906: *mut Expr, __v907: i32);
    fn sqlite3ExprCodeTemp(__v908: *mut Parse, __v909: *mut Expr, __v910: *mut i32) -> i32;
    fn sqlite3ExprCodeTarget(__v911: *mut Parse, __v912: *mut Expr, __v913: i32) -> i32;
    fn sqlite3ExprIfFalse(__v914: *mut Parse, __v915: *mut Expr, __v916: i32, __v917: i32);
    fn sqlite3ExprCompare(
        __v918: *const Parse,
        __v919: *const Expr,
        __v920: *const Expr,
        __v921: i32,
    ) -> i32;
    fn sqlite3ExprCoveredByIndex(__v922: *mut Expr, iCur: i32, pIdx: *mut Index) -> i32;
    fn sqlite3ExprCanBeNull(__v925: *const Expr) -> i32;
    fn sqlite3ExprNeedsNoAffinityChange(__v926: *const Expr, __v927: i8) -> i32;
    fn sqlite3ExprDup(__v928: *mut sqlite3, __v929: *const Expr, __v930: i32) -> *mut Expr;
    fn sqlite3IndexAffinityStr(__v931: *mut sqlite3, __v932: *mut Index) -> *const i8;
    fn sqlite3CompareAffinity(pExpr: *const Expr, aff2: i8) -> i8;
    fn sqlite3CodeRhsOfIN(__v935: *mut Parse, __v936: *mut Expr, __v937: i32, __v938: i32);
    fn sqlite3CodeSubselect(__v939: *mut Parse, __v940: *mut Expr) -> i32;
    fn sqlite3StrAccumInit(
        __v941: *mut sqlite3_str,
        __v942: *mut sqlite3,
        __v943: *mut i8,
        __v944: i32,
        __v945: i32,
    );
    fn sqlite3StrAccumFinish(__v946: *mut sqlite3_str) -> *mut i8;
    fn sqlite3FindInIndex(
        __v947: *mut Parse,
        __v948: *mut Expr,
        __v949: u32,
        __v950: *mut i32,
        __v951: *mut i32,
        __v952: *mut i32,
    ) -> i32;
    fn sqlite3ExprIsVector(pExpr: *const Expr) -> i32;
    fn sqlite3VectorFieldSubexpr(__v954: *mut Expr, __v955: i32) -> *mut Expr;
    fn sqlite3WhereGetMask(__v956: *mut WhereMaskSet, __v957: i32) -> u64;
    fn sqlite3WhereFindTerm(
        pWC: *mut WhereClause,
        iCur: i32,
        iColumn: i32,
        notReady: u64,
        op: u32,
        pIdx: *mut Index,
    ) -> *mut WhereTerm;
    fn sqlite3WhereRealloc(pWInfo: *mut WhereInfo, pOld: *mut (), nByte: u64) -> *mut ();
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
    __slate_bits_0: __slate_bits::__SlateBits66U0,
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
    __slate_bits_0: __slate_bits::__SlateBits90U0,
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
struct RenameToken {}

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
    __slate_bits_0: __slate_bits::__SlateBits102U0,
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
    u: __SlateRecord210,
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
    u: __SlateRecord213,
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
    u: __SlateRecord216,
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
    __slate_bits_0: __slate_bits::__SlateBits139U0,
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
union __SlateRecord210 {
    r#in: __SlateRecord211,
    pCoveringIdx: *mut Index,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord211 {
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
union __SlateRecord213 {
    btree: __SlateRecord214,
    vtab: __SlateRecord215,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord214 {
    nEq: u16,
    nBtm: u16,
    nTop: u16,
    nDistinctCol: u16,
    pIndex: *mut Index,
    pOrderBy: *mut ExprList,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord215 {
    idxNum: i32,
    __slate_bits_0: __slate_bits::__SlateBits215U0,
    isOrdered: i8,
    omitMask: u16,
    idxStr: *mut i8,
    mHandleIn: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord216 {
    x: __SlateRecord217,
    pOrInfo: *mut WhereOrInfo,
    pAndInfo: *mut WhereAndInfo,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord217 {
    leftColumn: i32,
    iField: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord218 {
    sSrc: SrcList,
    fromSpace: [u8; 80],
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
    pub struct __SlateBits66U0 {
        #[bits(4)]
        pub notNull: u32,
        #[bits(4)]
        pub eCType: u32,
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
    pub struct __SlateBits90U0 {
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
    #[bitfields::bitfield([u8; 2], c_names = true, new = false, from_into_bits = false, from_traits = false, default = false, debug = false, builder = false, bit_ops = false)]
    pub struct __SlateBits102U0 {
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
    pub struct __SlateBits139U0 {
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
    pub struct __SlateBits215U0 {
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

/// Return the name of the i-th column of the pIdx index.
fn explainIndexColumnName(mut pIdx: *mut Index, mut i: i32) -> *const i8 {
    i = (unsafe { *unsafe { unsafe { (*pIdx).aiColumn }.offset(i as isize) } }) as i32;
    if i == -(2 as i32) {
        return (b"<expr>\0".as_ptr() as *mut i8) as *const i8;
    }
    if i == -(1 as i32) {
        return (b"rowid\0".as_ptr() as *mut i8) as *const i8;
    }
    return (unsafe {
        (*unsafe { unsafe { (*unsafe { (*pIdx).pTable }).aCol }.offset(i as isize) }).zCnName
    }) as *const i8;
}

/// This routine is a helper for explainIndexRange() below
///
/// pStr holds the text of an expression that we are building up one term
/// at a time.  This routine adds a new term to the end of the expression.
/// Terms are separated by AND so add the "AND" text for second and subsequent
/// terms only.
///
/// # Arguments
///
/// * `pStr` - The text expression being built
/// * `pIdx` - Index to read column names from
/// * `nTerm` - Number of terms
/// * `iTerm` - Zero-based index of first term.
/// * `bAnd` - Non-zero to append " AND "
/// * `zOp` - Name of the operator
fn explainAppendTerm(
    mut pStr: *mut sqlite3_str,
    mut pIdx: *mut Index,
    mut nTerm: i32,
    mut iTerm: i32,
    mut bAnd: i32,
    mut zOp: *const i8,
) {
    let mut i: i32 = 0 as i32;
    0 as i32;
    if bAnd != (0 as i32) {
        unsafe {
            sqlite3_str_append(
                pStr,
                (b" AND \0".as_ptr() as *mut i8) as *const i8,
                5 as i32,
            )
        };
    }
    if nTerm > (1 as i32) {
        unsafe { sqlite3_str_append(pStr, (b"(\0".as_ptr() as *mut i8) as *const i8, 1 as i32) };
    }
    i = 0 as i32;
    '__slate_break_992: loop {
        if !(i < nTerm) {
            break;
        }
        if i != (0 as i32) {
            unsafe {
                sqlite3_str_append(pStr, (b",\0".as_ptr() as *mut i8) as *const i8, 1 as i32)
            };
        }
        unsafe { sqlite3_str_appendall(pStr, explainIndexColumnName(pIdx, iTerm + i)) };
        let __v1228: i32 = i;
        let __v1229: i32 = __v1228 + (1 as i32);
        i = __v1229;
    }
    if nTerm > (1 as i32) {
        unsafe { sqlite3_str_append(pStr, (b")\0".as_ptr() as *mut i8) as *const i8, 1 as i32) };
    }
    unsafe { sqlite3_str_append(pStr, zOp, 1 as i32) };
    if nTerm > (1 as i32) {
        unsafe { sqlite3_str_append(pStr, (b"(\0".as_ptr() as *mut i8) as *const i8, 1 as i32) };
    }
    i = 0 as i32;
    '__slate_break_996: loop {
        if !(i < nTerm) {
            break;
        }
        if i != (0 as i32) {
            unsafe {
                sqlite3_str_append(pStr, (b",\0".as_ptr() as *mut i8) as *const i8, 1 as i32)
            };
        }
        unsafe { sqlite3_str_append(pStr, (b"?\0".as_ptr() as *mut i8) as *const i8, 1 as i32) };
        let __v1230: i32 = i;
        let __v1231: i32 = __v1230 + (1 as i32);
        i = __v1231;
    }
    if nTerm > (1 as i32) {
        unsafe { sqlite3_str_append(pStr, (b")\0".as_ptr() as *mut i8) as *const i8, 1 as i32) };
    }
}

/// Argument pLevel describes a strategy for scanning table pTab. This
/// function appends text to pStr that describes the subset of table
/// rows scanned by the strategy in the form of an SQL expression.
///
/// For example, if the query:
///
///   SELECT * FROM t1 WHERE a=1 AND b>2;
///
/// is run and there is an index on (a, b), then this function returns a
/// string similar to:
///
///   "a=? AND b>?"
fn explainIndexRange(mut pStr: *mut sqlite3_str, mut pLoop: *mut WhereLoop) {
    let mut pIndex: *mut Index = unsafe { (*pLoop).u.btree.pIndex };
    let mut nEq: u16 = unsafe { (*pLoop).u.btree.nEq };
    let mut nSkip: u16 = unsafe { (*pLoop).nSkip };
    let mut i: i32 = 0 as i32;
    let mut j: i32 = 0 as i32;
    if ((nEq as u32) as i32) == (0 as i32)
        && (unsafe { (*pLoop).wsFlags }) & (((32 as i32) | (16 as i32)) as u32)
            == ((0 as i32) as u32)
    {
        return;
    }
    unsafe { sqlite3_str_append(pStr, (b" (\0".as_ptr() as *mut i8) as *const i8, 2 as i32) };
    i = 0 as i32;
    '__slate_break_1001: loop {
        if !(i < ((nEq as u32) as i32)) {
            break;
        }
        let mut z: *const i8 = explainIndexColumnName(pIndex, i);
        if i != (0 as i32) {
            unsafe {
                sqlite3_str_append(
                    pStr,
                    (b" AND \0".as_ptr() as *mut i8) as *const i8,
                    5 as i32,
                )
            };
        }
        unsafe {
            sqlite3_str_appendf(
                pStr,
                (if i >= ((nSkip as u32) as i32) {
                    b"%s=?\0".as_ptr() as *mut i8
                } else {
                    b"ANY(%s)\0".as_ptr() as *mut i8
                }) as *const i8,
                z,
            )
        };
        let __v1232: i32 = i;
        let __v1233: i32 = __v1232 + (1 as i32);
        i = __v1233;
    }
    j = i;
    if (unsafe { (*pLoop).wsFlags }) & ((32 as i32) as u32) != (0 as u32) {
        explainAppendTerm(
            pStr,
            pIndex,
            ((unsafe { (*pLoop).u.btree.nBtm }) as u32) as i32,
            j,
            i,
            (b">\0".as_ptr() as *mut i8) as *const i8,
        );
        i = 1 as i32;
    }
    if (unsafe { (*pLoop).wsFlags }) & ((16 as i32) as u32) != (0 as u32) {
        explainAppendTerm(
            pStr,
            pIndex,
            ((unsafe { (*pLoop).u.btree.nTop }) as u32) as i32,
            j,
            i,
            (b"<\0".as_ptr() as *mut i8) as *const i8,
        );
    }
    unsafe { sqlite3_str_append(pStr, (b")\0".as_ptr() as *mut i8) as *const i8, 1 as i32) };
}

/// This function sets the P4 value of an existing OP_Explain opcode to
/// text describing the loop in pLevel. If the OP_Explain opcode already has
/// a P4 value, it is freed before it is overwritten.
///
/// # Arguments
///
/// * `pParse` - Parse context
/// * `addr` - Address of OP_Explain opcode
/// * `pTabList` - Table list this loop refers to
/// * `pLevel` - Scan to write OP_Explain opcode for
/// * `wctrlFlags` - Flags passed to sqlite3WhereBegin()
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3WhereAddExplainText(
    mut pParse: *mut Parse,
    mut addr: i32,
    mut pTabList: *mut SrcList,
    mut pLevel: *mut WhereLevel,
    mut wctrlFlags: u16,
) {
    if (((unsafe {
        (*if (unsafe { (*pParse).pToplevel }) != std::ptr::null_mut::<Parse>() {
            unsafe { (*pParse).pToplevel }
        } else {
            pParse
        })
        .explain
    }) as u32) as i32)
        == (2 as i32)
        || (0 as i32) != (0 as i32)
    {
        let mut pOp: *mut VdbeOp = unsafe { sqlite3VdbeGetOp(unsafe { (*pParse).pVdbe }, addr) };
        let mut pItem: *mut SrcItem = unsafe {
            unsafe { std::ptr::addr_of_mut!((*pTabList).a) as *mut SrcItem }
                .offset((((unsafe { (*pLevel).iFrom }) as u32) as i32) as isize)
        };
        let mut db: *mut sqlite3 = unsafe { (*pParse).db }; // Database handle
        let mut isSearch: i32 = 0 as i32; // True for a SEARCH. False for SCAN.
        let mut pLoop: *mut WhereLoop = unsafe { std::mem::zeroed() }; // The controlling WhereLoop object
        let mut flags: u32 = 0 as u32; // Flags that describe this loop
        let mut str: sqlite3_str = unsafe { std::mem::zeroed() }; // EQP output string
        let mut zBuf: __SlateAlign16<[i8; 100]> = __SlateAlign16([0 as i8; 100]); // Initial space for EQP output string
        if (unsafe { (*db).mallocFailed }) != (0 as u8) {
            return;
        }
        pLoop = unsafe { (*pLevel).pWLoop };
        flags = unsafe { (*pLoop).wsFlags };
        isSearch = (flags & (((32 as i32) | (16 as i32)) as u32) != ((0 as i32) as u32)
            || flags & ((1024 as i32) as u32) == ((0 as i32) as u32)
                && (((unsafe { (*pLoop).u.btree.nEq }) as u32) as i32) > (0 as i32)
            || ((wctrlFlags as u32) as i32) & ((1 as i32) | (2 as i32)) != (0 as i32))
            as i32;
        unsafe {
            sqlite3StrAccumInit(
                std::ptr::addr_of_mut!(str),
                db,
                zBuf.0.as_mut_ptr() as *mut i8,
                ((100 as u64) as u32) as i32,
                1000000000 as i32,
            )
        };
        str.printfFlags = ((1 as i32) as i8) as u8;
        unsafe {
            sqlite3_str_appendf(
                std::ptr::addr_of_mut!(str),
                (b"%s %S%s\0".as_ptr() as *mut i8) as *const i8,
                if isSearch != (0 as i32) {
                    b"SEARCH\0".as_ptr() as *mut i8
                } else {
                    b"SCAN\0".as_ptr() as *mut i8
                },
                pItem,
                if ((unsafe { (*pItem).fg.__slate_bits_0.__get_fromExists() }) as i32) != (0 as i32)
                {
                    b" EXISTS\0".as_ptr() as *mut i8
                } else {
                    b"\0".as_ptr() as *mut i8
                },
            )
        };
        if flags & (((256 as i32) | (1024 as i32)) as u32) == ((0 as i32) as u32) {
            let mut zFmt: *const i8 = std::ptr::null::<i8>();
            let mut pIdx: *mut Index = unsafe { std::mem::zeroed() };
            0 as i32;
            pIdx = unsafe { (*pLoop).u.btree.pIndex };
            0 as i32;
            if !((unsafe { (*unsafe { (*pItem).pSTab }).tabFlags }) & ((128 as i32) as u32)
                == ((0 as i32) as u32))
                && ((unsafe { (*pIdx).__slate_bits_0.__get_idxType() }) as i32) == (2 as i32)
            {
                if isSearch != (0 as i32) {
                    zFmt = (b"PRIMARY KEY\0".as_ptr() as *mut i8) as *const i8;
                }
            } else {
                if flags & ((131072 as i32) as u32) != (0 as u32) {
                    zFmt = (b"AUTOMATIC PARTIAL COVERING INDEX\0".as_ptr() as *mut i8) as *const i8;
                } else {
                    if flags & ((16384 as i32) as u32) != (0 as u32) {
                        zFmt = (b"AUTOMATIC COVERING INDEX\0".as_ptr() as *mut i8) as *const i8;
                    } else {
                        if flags & (((64 as i32) | (67108864 as i32)) as u32) != (0 as u32) {
                            zFmt = (b"COVERING INDEX %s\0".as_ptr() as *mut i8) as *const i8;
                        } else {
                            zFmt = (b"INDEX %s\0".as_ptr() as *mut i8) as *const i8;
                        }
                    }
                }
            }
            if zFmt != std::ptr::null::<i8>() {
                unsafe {
                    sqlite3_str_append(
                        std::ptr::addr_of_mut!(str),
                        (b" USING \0".as_ptr() as *mut i8) as *const i8,
                        7 as i32,
                    )
                };
                unsafe {
                    sqlite3_str_appendf(std::ptr::addr_of_mut!(str), zFmt, unsafe { (*pIdx).zName })
                };
                explainIndexRange(std::ptr::addr_of_mut!(str), pLoop);
            }
        } else {
            if flags & ((256 as i32) as u32) != ((0 as i32) as u32)
                && flags & ((15 as i32) as u32) != ((0 as i32) as u32)
            {
                let mut cRangeOp: i8 = 0 as i8;
                let mut zRowid: *const i8 = (b"rowid\0".as_ptr() as *mut i8) as *const i8;
                unsafe {
                    sqlite3_str_appendf(
                        std::ptr::addr_of_mut!(str),
                        (b" USING INTEGER PRIMARY KEY (%s\0".as_ptr() as *mut i8) as *const i8,
                        zRowid,
                    )
                };
                if flags & (((1 as i32) | (4 as i32)) as u32) != (0 as u32) {
                    cRangeOp = (61 as i32) as i8;
                } else {
                    if flags & ((48 as i32) as u32) == ((48 as i32) as u32) {
                        unsafe {
                            sqlite3_str_appendf(
                                std::ptr::addr_of_mut!(str),
                                (b">? AND %s\0".as_ptr() as *mut i8) as *const i8,
                                zRowid,
                            )
                        };
                        cRangeOp = (60 as i32) as i8;
                    } else {
                        if flags & ((32 as i32) as u32) != (0 as u32) {
                            cRangeOp = (62 as i32) as i8;
                        } else {
                            0 as i32;
                            cRangeOp = (60 as i32) as i8;
                        }
                    }
                }
                unsafe {
                    sqlite3_str_appendf(
                        std::ptr::addr_of_mut!(str),
                        (b"%c?)\0".as_ptr() as *mut i8) as *const i8,
                        cRangeOp as i32,
                    )
                };
            } else {
                if flags & ((1024 as i32) as u32) != ((0 as i32) as u32) {
                    unsafe {
                        sqlite3_str_appendall(
                            std::ptr::addr_of_mut!(str),
                            (b" VIRTUAL TABLE INDEX \0".as_ptr() as *mut i8) as *const i8,
                        )
                    };
                    unsafe {
                        sqlite3_str_appendf(
                            std::ptr::addr_of_mut!(str),
                            (if ((unsafe { (*pLoop).u.vtab.__slate_bits_0.__get_bIdxNumHex() })
                                as i32)
                                != (0 as i32)
                            {
                                b"0x%x:%s\0".as_ptr() as *mut i8
                            } else {
                                b"%d:%s\0".as_ptr() as *mut i8
                            }) as *const i8,
                            unsafe { (*pLoop).u.vtab.idxNum },
                            unsafe { (*pLoop).u.vtab.idxStr },
                        )
                    };
                }
            }
        }
        if (((unsafe { (*pItem).fg.jointype }) as u32) as i32) & (8 as i32) != (0 as i32) {
            unsafe {
                sqlite3_str_appendf(
                    std::ptr::addr_of_mut!(str),
                    (b" LEFT-JOIN\0".as_ptr() as *mut i8) as *const i8,
                )
            };
        }
        0 as i32;
        0 as i32;
        unsafe { sqlite3DbFree(db, (unsafe { (*pOp).p4.z }) as *mut ()) };
        unsafe {
            (*pOp).p4type = -(7 as i32) as i8;
        }
        unsafe {
            (*pOp).p4.z = unsafe { sqlite3StrAccumFinish(std::ptr::addr_of_mut!(str)) };
        }
    }
}

/// This function is a no-op unless currently processing an EXPLAIN QUERY PLAN
/// command, or if stmt_scanstatus_v2() stats are enabled, or if SQLITE_DEBUG
/// was defined at compile-time. If it is not a no-op, a single OP_Explain
/// opcode is added to the output to describe the table scan strategy in pLevel.
///
/// If an OP_Explain opcode is added to the VM, its address is returned.
/// Otherwise, if no OP_Explain is coded, zero is returned.
///
/// # Arguments
///
/// * `pParse` - Parse context
/// * `pTabList` - Table list this loop refers to
/// * `pLevel` - Scan to write OP_Explain opcode for
/// * `wctrlFlags` - Flags passed to sqlite3WhereBegin()
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3WhereExplainOneScan(
    mut pParse: *mut Parse,
    mut pTabList: *mut SrcList,
    mut pLevel: *mut WhereLevel,
    mut wctrlFlags: u16,
) -> i32 {
    let mut ret: i32 = 0 as i32;
    if (((unsafe {
        (*if (unsafe { (*pParse).pToplevel }) != std::ptr::null_mut::<Parse>() {
            unsafe { (*pParse).pToplevel }
        } else {
            pParse
        })
        .explain
    }) as u32) as i32)
        == (2 as i32)
        || (0 as i32) != (0 as i32)
    {
        if (unsafe { (*unsafe { (*pLevel).pWLoop }).wsFlags }) & ((8192 as i32) as u32)
            == ((0 as i32) as u32)
            && ((wctrlFlags as u32) as i32) & (32 as i32) == (0 as i32)
        {
            let mut v: *mut Vdbe = unsafe { (*pParse).pVdbe };
            let mut addr: i32 = unsafe { sqlite3VdbeCurrentAddr(v) };
            ret = unsafe {
                sqlite3VdbeAddOp3(
                    v,
                    190 as i32,
                    addr,
                    unsafe { (*pParse).addrExplain },
                    (unsafe { (*unsafe { (*pLevel).pWLoop }).rRun }) as i32,
                )
            };
            sqlite3WhereAddExplainText(pParse, addr, pTabList, pLevel, wctrlFlags);
        }
    }
    return ret;
}

/// Add a single OP_Explain opcode that describes a Bloom filter.
///
/// Or if not processing EXPLAIN QUERY PLAN and not in a SQLITE_DEBUG and/or
/// SQLITE_ENABLE_STMT_SCANSTATUS build, then OP_Explain opcodes are not
/// required and this routine is a no-op.
///
/// If an OP_Explain opcode is added to the VM, its address is returned.
/// Otherwise, if no OP_Explain is coded, zero is returned.
///
/// # Arguments
///
/// * `pParse` - Parse context
/// * `pWInfo` - WHERE clause
/// * `pLevel` - Bloom filter on this level
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3WhereExplainBloomFilter(
    mut pParse: *const Parse,
    mut pWInfo: *const WhereInfo,
    mut pLevel: *const WhereLevel,
) -> i32 {
    let mut ret: i32 = 0 as i32;
    let mut pItem: *mut SrcItem = unsafe {
        unsafe { std::ptr::addr_of_mut!((*unsafe { (*pWInfo).pTabList }).a) as *mut SrcItem }
            .offset((((unsafe { (*pLevel).iFrom }) as u32) as i32) as isize)
    };
    let mut v: *mut Vdbe = unsafe { (*pParse).pVdbe }; // VM being constructed
    let mut db: *mut sqlite3 = unsafe { (*pParse).db }; // Database handle
    let mut zMsg: *mut i8 = unsafe { std::mem::zeroed() }; // Text to add to EQP output
    let mut i: i32 = 0 as i32; // Loop counter
    let mut pLoop: *mut WhereLoop = unsafe { std::mem::zeroed() }; // The where loop
    let mut str: sqlite3_str = unsafe { std::mem::zeroed() }; // EQP output string
    let mut zBuf: __SlateAlign16<[i8; 100]> = __SlateAlign16([0 as i8; 100]); // Initial space for EQP output string
    unsafe {
        sqlite3StrAccumInit(
            std::ptr::addr_of_mut!(str),
            db,
            zBuf.0.as_mut_ptr() as *mut i8,
            ((100 as u64) as u32) as i32,
            1000000000 as i32,
        )
    };
    str.printfFlags = ((1 as i32) as i8) as u8;
    unsafe {
        sqlite3_str_appendf(
            std::ptr::addr_of_mut!(str),
            (b"BLOOM FILTER ON %S (\0".as_ptr() as *mut i8) as *const i8,
            pItem,
        )
    };
    pLoop = unsafe { (*pLevel).pWLoop };
    if (unsafe { (*pLoop).wsFlags }) & ((256 as i32) as u32) != (0 as u32) {
        let mut pTab: *const Table = (unsafe { (*pItem).pSTab }) as *const Table;
        if ((unsafe { (*pTab).iPKey }) as i32) >= (0 as i32) {
            unsafe {
                sqlite3_str_appendf(
                    std::ptr::addr_of_mut!(str),
                    (b"%s=?\0".as_ptr() as *mut i8) as *const i8,
                    unsafe {
                        (*unsafe {
                            unsafe { (*pTab).aCol }
                                .offset(((unsafe { (*pTab).iPKey }) as i32) as isize)
                        })
                        .zCnName
                    },
                )
            };
        } else {
            unsafe {
                sqlite3_str_appendf(
                    std::ptr::addr_of_mut!(str),
                    (b"rowid=?\0".as_ptr() as *mut i8) as *const i8,
                )
            };
        }
    } else {
        i = ((unsafe { (*pLoop).nSkip }) as u32) as i32;
        '__slate_break_1030: loop {
            if !(i < (((unsafe { (*pLoop).u.btree.nEq }) as u32) as i32)) {
                break;
            }
            let mut z: *const i8 = explainIndexColumnName(unsafe { (*pLoop).u.btree.pIndex }, i);
            if i > (((unsafe { (*pLoop).nSkip }) as u32) as i32) {
                unsafe {
                    sqlite3_str_append(
                        std::ptr::addr_of_mut!(str),
                        (b" AND \0".as_ptr() as *mut i8) as *const i8,
                        5 as i32,
                    )
                };
            }
            unsafe {
                sqlite3_str_appendf(
                    std::ptr::addr_of_mut!(str),
                    (b"%s=?\0".as_ptr() as *mut i8) as *const i8,
                    z,
                )
            };
            let __v1086: i32 = i;
            let __v1087: i32 = __v1086 + (1 as i32);
            i = __v1087;
        }
    }
    unsafe {
        sqlite3_str_append(
            std::ptr::addr_of_mut!(str),
            (b")\0".as_ptr() as *mut i8) as *const i8,
            1 as i32,
        )
    };
    zMsg = unsafe { sqlite3StrAccumFinish(std::ptr::addr_of_mut!(str)) };
    ret = unsafe {
        sqlite3VdbeAddOp4(
            v,
            190 as i32,
            unsafe { sqlite3VdbeCurrentAddr(v) },
            unsafe { (*pParse).addrExplain },
            0 as i32,
            zMsg as *const i8,
            -(7 as i32),
        )
    };
    {}
    return ret;
}

/// Disable a term in the WHERE clause.  Except, do not disable the term
/// if it controls a LEFT OUTER JOIN and it did not originate in the ON
/// or USING clause of that join.
///
/// Consider the term t2.z='ok' in the following queries:
///
///   (1)  SELECT * FROM t1 LEFT JOIN t2 ON t1.a=t2.x WHERE t2.z='ok'
///   (2)  SELECT * FROM t1 LEFT JOIN t2 ON t1.a=t2.x AND t2.z='ok'
///   (3)  SELECT * FROM t1, t2 WHERE t1.a=t2.x AND t2.z='ok'
///
/// The t2.z='ok' is disabled in the in (2) because it originates
/// in the ON clause.  The term is disabled in (3) because it is not part
/// of a LEFT OUTER JOIN.  In (1), the term is not disabled.
///
/// Disabling a term causes that term to not be tested in the inner loop
/// of the join.  Disabling is an optimization.  When terms are satisfied
/// by indices, we disable them to prevent redundant tests in the inner
/// loop.  We would get the correct results if nothing were ever disabled,
/// but joins might run a little slower.  The trick is to disable as much
/// as we can without disabling too much.  If we disabled in (1), we'd get
/// the wrong answer.  See ticket #813.
///
/// If all the children of a term are disabled, then that term is also
/// automatically disabled.  In this way, terms get disabled if derived
/// virtual terms are tested first.  For example:
///
///      x GLOB 'abc*' AND x>='abc' AND x<'acd'
///      \___________/     \______/     \_____/
///         parent          child1       child2
///
/// Only the parent term was in the original WHERE clause.  The child1
/// and child2 terms were added by the LIKE optimization.  If both of
/// the virtual child terms are valid, then testing of the parent can be
/// skipped.
///
/// Usually the parent term is marked as TERM_CODED.  But if the parent
/// term was originally TERM_LIKE, then the parent gets TERM_LIKECOND instead.
/// The TERM_LIKECOND marking indicates that the term should be coded inside
/// a conditional such that is only evaluated on the second pass of a
/// LIKE-optimization loop, when scanning BLOBs instead of strings.
fn disableTerm(mut pLevel: *mut WhereLevel, mut pTerm: *mut WhereTerm) {
    let mut nLoop: i32 = 0 as i32;
    0 as i32;
    '__slate_break_1034: while (((unsafe { (*pTerm).wtFlags }) as u32) as i32) & (4 as i32)
        == (0 as i32)
        && ((unsafe { (*pLevel).iLeftJoin }) == (0 as i32)
            || (unsafe { (*unsafe { (*pTerm).pExpr }).flags }) & ((1 as i32) as u32)
                != ((0 as i32) as u32))
        && (unsafe { (*pLevel).notReady }) & unsafe { (*pTerm).prereqAll }
            == (((0 as i32) as i64) as u64)
    {
        if nLoop != (0 as i32)
            && (((unsafe { (*pTerm).wtFlags }) as u32) as i32) & (1024 as i32) != (0 as i32)
        {
            let __v1234: *mut WhereTerm = pTerm;
            let __v1235: u16 = unsafe { (*__v1234).wtFlags };
            let __v1236: u16 = ((((__v1235 as u32) as i32) | (512 as i32)) as i16) as u16;
            unsafe {
                (*__v1234).wtFlags = __v1236;
            }
        } else {
            let __v1237: *mut WhereTerm = pTerm;
            let __v1238: u16 = unsafe { (*__v1237).wtFlags };
            let __v1239: u16 = ((((__v1238 as u32) as i32) | (4 as i32)) as i16) as u16;
            unsafe {
                (*__v1237).wtFlags = __v1239;
            }
        }
        if (unsafe { (*pTerm).iParent }) < (0 as i32) {
            break '__slate_break_1034;
        }
        pTerm = unsafe {
            unsafe { (*unsafe { (*pTerm).pWC }).a }.offset((unsafe { (*pTerm).iParent }) as isize)
        };
        0 as i32;
        let __v1240: *mut WhereTerm = pTerm;
        let __v1241: u8 = unsafe { (*__v1240).nChild };
        let __v1242: u8 = ((((__v1241 as u32) as i32) - (1 as i32)) as i8) as u8;
        unsafe {
            (*__v1240).nChild = __v1242;
        }
        if (((unsafe { (*pTerm).nChild }) as u32) as i32) != (0 as i32) {
            break '__slate_break_1034;
        }
        let __v1243: i32 = nLoop;
        let __v1244: i32 = __v1243 + (1 as i32);
        nLoop = __v1244;
    }
}

/// Code an OP_Affinity opcode to apply the column affinity string zAff
/// to the n registers starting at base.
///
/// As an optimization, SQLITE_AFF_BLOB and SQLITE_AFF_NONE entries (which
/// are no-ops) at the beginning and end of zAff are ignored.  If all entries
/// in zAff are SQLITE_AFF_BLOB or SQLITE_AFF_NONE, then no code gets generated.
///
/// This routine makes its own copy of zAff so that the caller is free
/// to modify zAff after this routine returns.
fn codeApplyAffinity(mut pParse: *mut Parse, mut base: i32, mut n: i32, mut zAff: *mut i8) {
    let mut v: *mut Vdbe = unsafe { (*pParse).pVdbe };
    if zAff == std::ptr::null_mut::<i8>() {
        0 as i32;
        return;
    }
    0 as i32;
    // Adjust base and n to skip over SQLITE_AFF_BLOB and SQLITE_AFF_NONE
    // entries at the beginning and end of the affinity string.
    0 as i32;
    '__slate_break_1035: while n > (0 as i32)
        && ((unsafe { *unsafe { zAff.offset((0 as i32) as isize) } }) as i32) <= (65 as i32)
    {
        let __v1245: i32 = n;
        let __v1246: i32 = __v1245 - (1 as i32);
        n = __v1246;
        let __v1247: i32 = base;
        let __v1248: i32 = __v1247 + (1 as i32);
        base = __v1248;
        let __v1249: *mut i8 = zAff;
        let __v1250: *mut i8 = unsafe { __v1249.offset((1 as i32) as isize) };
        zAff = __v1250;
    }
    '__slate_break_1036: while n > (1 as i32)
        && ((unsafe { *unsafe { zAff.offset((n - (1 as i32)) as isize) } }) as i32) <= (65 as i32)
    {
        let __v1251: i32 = n;
        let __v1252: i32 = __v1251 - (1 as i32);
        n = __v1252;
    }
    // Code the OP_Affinity opcode if there is anything left to do.
    if n > (0 as i32) {
        unsafe { sqlite3VdbeAddOp4(v, 98 as i32, base, n, 0 as i32, zAff as *const i8, n) };
    }
}

/// Expression pRight, which is the RHS of a comparison operation, is
/// either a vector of n elements or, if n==1, a scalar expression.
/// Before the comparison operation, affinity zAff is to be applied
/// to the pRight values. This function modifies characters within the
/// affinity string to SQLITE_AFF_BLOB if either:
///
///   * the comparison will be performed with no affinity, or
///   * the affinity change in zAff is guaranteed not to change the value.
///
/// # Arguments
///
/// * `pRight` - RHS of comparison
/// * `n` - Number of vector elements in comparison
/// * `zAff` - Affinity string to modify
fn updateRangeAffinityStr(mut pRight: *mut Expr, mut n: i32, mut zAff: *mut i8) {
    let mut i: i32 = 0 as i32;
    i = 0 as i32;
    '__slate_break_1037: loop {
        if !(i < n) {
            break;
        }
        let mut p: *mut Expr = unsafe { sqlite3VectorFieldSubexpr(pRight, i) };
        let __v1255: bool;
        if ((unsafe {
            sqlite3CompareAffinity(p as *const Expr, unsafe {
                *unsafe { zAff.offset(i as isize) }
            })
        }) as i32)
            == (65 as i32)
        {
            __v1255 = true as bool;
        } else {
            __v1255 = (unsafe {
                sqlite3ExprNeedsNoAffinityChange(p as *const Expr, unsafe {
                    *unsafe { zAff.offset(i as isize) }
                })
            }) != (0 as i32);
        }
        if __v1255 {
            unsafe {
                *unsafe { zAff.offset(i as isize) } = (65 as i32) as i8;
            }
        }
        let __v1253: i32 = i;
        let __v1254: i32 = __v1253 + (1 as i32);
        i = __v1254;
    }
}

/// The pOrderBy->a[].u.x.iOrderByCol values might be incorrect because
/// columns might have been rearranged in the result set.  This routine
/// fixes them up.
///
/// pEList is the new result set.  The pEList->a[].u.x.iOrderByCol values
/// contain the *old* locations of each expression.  This is a temporary
/// use of u.x.iOrderByCol, not its intended use.  The caller must reset
/// u.x.iOrderByCol back to zero for all entries in pEList before the
/// caller returns.
///
/// This routine changes pOrderBy->a[].u.x.iOrderByCol values from
/// pEList->a[N].u.x.iOrderByCol into N+1.  (The "+1" is because of the 1-based
/// indexing used by iOrderByCol.)  Or if no match, iOrderByCol is set to zero.
fn adjustOrderByCol(mut pOrderBy: *mut ExprList, mut pEList: *mut ExprList) {
    let mut i: i32 = 0 as i32;
    let mut j: i32 = 0 as i32;
    if pOrderBy == std::ptr::null_mut::<ExprList>() {
        return;
    }
    i = 0 as i32;
    '__slate_break_1038: loop {
        if !(i < unsafe { (*pOrderBy).nExpr }) {
            break;
        }
        let mut t: i32 = ((unsafe {
            (*unsafe {
                unsafe { std::ptr::addr_of_mut!((*pOrderBy).a) as *mut ExprList_item }
                    .offset(i as isize)
            })
            .u
            .x
            .iOrderByCol
        }) as u32) as i32;
        if t == (0 as i32) {
        } else {
            j = 0 as i32;
            '__slate_break_1039: loop {
                if !(j < unsafe { (*pEList).nExpr }) {
                    break;
                }
                if (((unsafe {
                    (*unsafe {
                        unsafe { std::ptr::addr_of_mut!((*pEList).a) as *mut ExprList_item }
                            .offset(j as isize)
                    })
                    .u
                    .x
                    .iOrderByCol
                }) as u32) as i32)
                    == t
                {
                    unsafe {
                        (*unsafe {
                            unsafe { std::ptr::addr_of_mut!((*pOrderBy).a) as *mut ExprList_item }
                                .offset(i as isize)
                        })
                        .u
                        .x
                        .iOrderByCol = ((j + (1 as i32)) as i16) as u16;
                    }
                    break '__slate_break_1039;
                }
                let __v1258: i32 = j;
                let __v1259: i32 = __v1258 + (1 as i32);
                j = __v1259;
            }
            if j >= unsafe { (*pEList).nExpr } {
                unsafe {
                    (*unsafe {
                        unsafe { std::ptr::addr_of_mut!((*pOrderBy).a) as *mut ExprList_item }
                            .offset(i as isize)
                    })
                    .u
                    .x
                    .iOrderByCol = ((0 as i32) as i16) as u16;
                }
            }
        }
        let __v1256: i32 = i;
        let __v1257: i32 = __v1256 + (1 as i32);
        i = __v1257;
    }
}

/// pX is an expression of the form:  (vector) IN (SELECT ...)
/// In other words, it is a vector IN operator with a SELECT clause on the
/// RHS.  But not all terms in the vector are indexable and the terms might
/// not be in the correct order for indexing.
///
/// This routine makes a copy of the input pX expression and then adjusts
/// the vector on the LHS with corresponding changes to the SELECT so that
/// the vector contains only index terms and those terms are in the correct
/// order.  The modified IN expression is returned.  The caller is responsible
/// for deleting the returned expression.
///
/// Example:
///
///    CREATE TABLE t1(a,b,c,d,e,f);
///    CREATE INDEX t1x1 ON t1(e,c);
///    SELECT * FROM t1 WHERE (a,b,c,d,e) IN (SELECT v,w,x,y,z FROM t2)
///                           \_______________________________________/
///                                     The pX expression
///
/// Since only columns e and c can be used with the index, in that order,
/// the modified IN expression that is returned will be:
///
///        (e,c) IN (SELECT z,x FROM t2)
///
/// The reduced pX is different from the original (obviously) and thus is
/// only used for indexing, to improve performance.  The original unaltered
/// IN expression must also be run on each output row for correctness.
///
/// # Arguments
///
/// * `pParse` - The parsing context
/// * `iEq` - Look at loop terms starting here
/// * `pLoop` - The current loop
/// * `pX` - The IN expression to be reduced
fn removeUnindexableInClauseTerms(
    mut pParse: *mut Parse,
    mut iEq: i32,
    mut pLoop: *mut WhereLoop,
    mut pX: *mut Expr,
) -> *mut Expr {
    let mut db: *mut sqlite3 = unsafe { (*pParse).db };
    let mut pSelect: *mut Select = unsafe { std::mem::zeroed() }; // Pointer to the SELECT on the RHS
    let mut pNew: *mut Expr = unsafe { std::mem::zeroed() };
    pNew = unsafe { sqlite3ExprDup(db, pX as *const Expr, 0 as i32) };
    if (((unsafe { (*db).mallocFailed }) as u32) as i32) == (0 as i32) {
        pSelect = unsafe { (*pNew).x.pSelect };
        '__slate_break_1040: while pSelect != std::ptr::null_mut::<Select>() {
            let mut pOrigRhs: *mut ExprList = unsafe { std::mem::zeroed() }; // Original unmodified RHS
            let mut pOrigLhs: *mut ExprList = std::ptr::null_mut::<ExprList>(); // Original unmodified LHS
            let mut pRhs: *mut ExprList = std::ptr::null_mut::<ExprList>(); // New RHS after modifications
            let mut pLhs: *mut ExprList = std::ptr::null_mut::<ExprList>(); // New LHS after mods
            let mut i: i32 = 0 as i32; // Loop counter
            0 as i32;
            pOrigRhs = unsafe { (*pSelect).pEList };
            0 as i32;
            0 as i32;
            if pSelect == unsafe { (*pNew).x.pSelect } {
                pOrigLhs = unsafe { (*unsafe { (*pNew).pLeft }).x.pList };
            }
            i = iEq;
            '__slate_break_1041: loop {
                if !(i < (((unsafe { (*pLoop).nLTerm }) as u32) as i32)) {
                    break;
                }
                '__slate_continue_1041: {
                    if (unsafe {
                        (*unsafe { *unsafe { unsafe { (*pLoop).aLTerm }.offset(i as isize) } })
                            .pExpr
                    }) == pX
                    {
                        let mut iField: i32 = 0 as i32;
                        0 as i32;
                        iField = (unsafe {
                            (*unsafe { *unsafe { unsafe { (*pLoop).aLTerm }.offset(i as isize) } })
                                .u
                                .x
                                .iField
                        }) - (1 as i32);
                        if (unsafe {
                            (*unsafe {
                                unsafe {
                                    std::ptr::addr_of_mut!((*pOrigRhs).a) as *mut ExprList_item
                                }
                                .offset(iField as isize)
                            })
                            .pExpr
                        }) == std::ptr::null_mut::<Expr>()
                        {
                            break '__slate_continue_1041; // Duplicate PK column
                        }
                        pRhs = unsafe {
                            sqlite3ExprListAppend(pParse, pRhs, unsafe {
                                (*unsafe {
                                    unsafe {
                                        std::ptr::addr_of_mut!((*pOrigRhs).a) as *mut ExprList_item
                                    }
                                    .offset(iField as isize)
                                })
                                .pExpr
                            })
                        };
                        unsafe {
                            (*unsafe {
                                unsafe {
                                    std::ptr::addr_of_mut!((*pOrigRhs).a) as *mut ExprList_item
                                }
                                .offset(iField as isize)
                            })
                            .pExpr = std::ptr::null_mut::<Expr>();
                        }
                        if pRhs != std::ptr::null_mut::<ExprList>() {
                            unsafe {
                                (*unsafe {
                                    unsafe {
                                        std::ptr::addr_of_mut!((*pRhs).a) as *mut ExprList_item
                                    }
                                    .offset(((unsafe { (*pRhs).nExpr }) - (1 as i32)) as isize)
                                })
                                .u
                                .x
                                .iOrderByCol = ((iField + (1 as i32)) as i16) as u16;
                            }
                        }
                        if pOrigLhs != std::ptr::null_mut::<ExprList>() {
                            0 as i32;
                            pLhs = unsafe {
                                sqlite3ExprListAppend(pParse, pLhs, unsafe {
                                    (*unsafe {
                                        unsafe {
                                            std::ptr::addr_of_mut!((*pOrigLhs).a)
                                                as *mut ExprList_item
                                        }
                                        .offset(iField as isize)
                                    })
                                    .pExpr
                                })
                            };
                            unsafe {
                                (*unsafe {
                                    unsafe {
                                        std::ptr::addr_of_mut!((*pOrigLhs).a) as *mut ExprList_item
                                    }
                                    .offset(iField as isize)
                                })
                                .pExpr = std::ptr::null_mut::<Expr>();
                            }
                        }
                    }
                }
                let __v1260: i32 = i;
                let __v1261: i32 = __v1260 + (1 as i32);
                i = __v1261;
            }
            unsafe { sqlite3ExprListDelete(db, pOrigRhs) };
            if pOrigLhs != std::ptr::null_mut::<ExprList>() {
                unsafe { sqlite3ExprListDelete(db, pOrigLhs) };
                unsafe {
                    (*unsafe { (*pNew).pLeft }).x.pList = pLhs;
                }
            }
            unsafe {
                (*pSelect).pEList = pRhs;
            }
            let __v1262: *mut Parse = pParse;
            let __v1263: i32 = unsafe { (*__v1262).nSelect };
            let __v1264: i32 = __v1263 + (1 as i32);
            unsafe {
                (*__v1262).nSelect = __v1264;
            }
            unsafe {
                (*pSelect).selId = __v1264 as u32;
            }
            // Req'd for SubrtnSig validity
            if pLhs != std::ptr::null_mut::<ExprList>() && (unsafe { (*pLhs).nExpr }) == (1 as i32)
            {
                // Take care here not to generate a TK_VECTOR containing only a
                // single value. Since the parser never creates such a vector, some
                // of the subroutines do not handle this case.
                let mut p: *mut Expr = unsafe {
                    (*unsafe {
                        unsafe { std::ptr::addr_of_mut!((*pLhs).a) as *mut ExprList_item }
                            .offset((0 as i32) as isize)
                    })
                    .pExpr
                };
                unsafe {
                    (*unsafe {
                        unsafe { std::ptr::addr_of_mut!((*pLhs).a) as *mut ExprList_item }
                            .offset((0 as i32) as isize)
                    })
                    .pExpr = std::ptr::null_mut::<Expr>();
                }
                unsafe { sqlite3ExprDelete(db, unsafe { (*pNew).pLeft }) };
                unsafe {
                    (*pNew).pLeft = p;
                }
            }
            // If either the ORDER BY clause or the GROUP BY clause contains
            // references to result-set columns, those references might now be
            // obsolete.  So fix them up.
            0 as i32;
            if pRhs != std::ptr::null_mut::<ExprList>() {
                adjustOrderByCol(unsafe { (*pSelect).pOrderBy }, pRhs);
                adjustOrderByCol(unsafe { (*pSelect).pGroupBy }, pRhs);
                i = 0 as i32;
                '__slate_break_1042: loop {
                    if !(i < unsafe { (*pRhs).nExpr }) {
                        break;
                    }
                    unsafe {
                        (*unsafe {
                            unsafe { std::ptr::addr_of_mut!((*pRhs).a) as *mut ExprList_item }
                                .offset(i as isize)
                        })
                        .u
                        .x
                        .iOrderByCol = ((0 as i32) as i16) as u16;
                    }
                    let __v1265: i32 = i;
                    let __v1266: i32 = __v1265 + (1 as i32);
                    i = __v1266;
                }
            }
            pSelect = unsafe { (*pSelect).pPrior };
        }
    }
    return pNew;
}

/// Generate code for a single X IN (....) term of the WHERE clause.
///
/// This is a special-case of codeEqualityTerm() that works for IN operators
/// only.  It is broken out into a subroutine because this case is
/// uncommon and by splitting it off into a subroutine, the common case
/// runs faster.
///
/// The current value for the constraint is left in  register iTarget.
/// This routine sets up a loop that will iterate over all values of X.
///
/// # Arguments
///
/// * `pParse` - The parsing context
/// * `pTerm` - The term of the WHERE clause to be coded
/// * `pLevel` - The level of the FROM clause we are working on
/// * `iEq` - Index of the equality term within this level
/// * `bRev` - True for reverse-order IN operations
/// * `iTarget` - Attempt to leave results in this register
fn codeINTerm(
    mut pParse: *mut Parse,
    mut pTerm: *mut WhereTerm,
    mut pLevel: *mut WhereLevel,
    mut iEq: i32,
    mut bRev: i32,
    mut iTarget: i32,
) {
    let mut pX: *mut Expr = unsafe { (*pTerm).pExpr };
    let mut eType: i32 = 5 as i32;
    let mut iTab: i32 = 0 as i32;
    let mut pIn: *mut InLoop = unsafe { std::mem::zeroed() };
    let mut pLoop: *mut WhereLoop = unsafe { (*pLevel).pWLoop };
    let mut v: *mut Vdbe = unsafe { (*pParse).pVdbe };
    let mut i: i32 = 0 as i32;
    let mut nEq: i32 = 0 as i32;
    let mut aiMap: *mut i32 = std::ptr::null_mut::<i32>();
    if (unsafe { (*pLoop).wsFlags }) & ((1024 as i32) as u32) == ((0 as i32) as u32)
        && (unsafe { (*pLoop).u.btree.pIndex }) != std::ptr::null_mut::<Index>()
        && (unsafe {
            *unsafe {
                unsafe { (*unsafe { (*pLoop).u.btree.pIndex }).aSortOrder }.offset(iEq as isize)
            }
        }) != (0 as u8)
    {
        {}
        {}
        bRev = !(bRev != (0 as i32)) as i32;
    }
    0 as i32;
    i = 0 as i32;
    '__slate_break_1043: loop {
        if !(i < iEq) {
            break;
        }
        if (unsafe { *unsafe { unsafe { (*pLoop).aLTerm }.offset(i as isize) } })
            != std::ptr::null_mut::<WhereTerm>()
            && (unsafe {
                (*unsafe { *unsafe { unsafe { (*pLoop).aLTerm }.offset(i as isize) } }).pExpr
            }) == pX
        {
            disableTerm(pLevel, pTerm);
            return;
        }
        let __v1267: i32 = i;
        let __v1268: i32 = __v1267 + (1 as i32);
        i = __v1268;
    }
    i = iEq;
    '__slate_break_1044: loop {
        if !(i < (((unsafe { (*pLoop).nLTerm }) as u32) as i32)) {
            break;
        }
        0 as i32;
        if (unsafe {
            (*unsafe { *unsafe { unsafe { (*pLoop).aLTerm }.offset(i as isize) } }).pExpr
        }) == pX
        {
            let __v1271: i32 = nEq;
            let __v1272: i32 = __v1271 + (1 as i32);
            nEq = __v1272;
        }
        let __v1269: i32 = i;
        let __v1270: i32 = __v1269 + (1 as i32);
        i = __v1270;
    }
    iTab = 0 as i32;
    if !((unsafe { (*pX).flags }) & ((4096 as i32) as u32) != ((0 as i32) as u32))
        || (unsafe { (*unsafe { (*unsafe { (*pX).x.pSelect }).pEList }).nExpr }) == (1 as i32)
    {
        eType = unsafe {
            sqlite3FindInIndex(
                pParse,
                pX,
                (4 as i32) as u32,
                std::ptr::null_mut::<i32>(),
                std::ptr::null_mut::<i32>(),
                std::ptr::addr_of_mut!(iTab),
            )
        };
    } else {
        let mut db: *mut sqlite3 = unsafe { (*pParse).db };
        let mut pXMod: *mut Expr = removeUnindexableInClauseTerms(pParse, iEq, pLoop, pX);
        if nEq > (1 as i32) {
            // If this IN(SELECT ...) expression drives more than one column of
            // the index, disable the seek-scan optimization. The reasons for this
            // are that (a) it is only possible to make this happen by populating
            // the sqlite_stat1 table with inconsistent information, and (b) it
            // would require sqlite3FindInIndex() to find an index that is not
            // only unique for the columns in question, but also delivers them
            // in sorted order (requires checking asc/desc, and rejecting cases
            // where the indexed columns are not in the right order).
            let __v1273: *mut WhereLoop = pLoop;
            let __v1274: u32 = unsafe { (*__v1273).wsFlags };
            let __v1275: u32 = __v1274 & (!(1048576 as i32) as u32);
            unsafe {
                (*__v1273).wsFlags = __v1275;
            }
        }
        if !((unsafe { (*db).mallocFailed }) != (0 as u8)) {
            aiMap =
                (unsafe { sqlite3DbMallocZero(db, (4 as u64).wrapping_mul((nEq as i64) as u64)) })
                    as *mut i32;
            eType = unsafe {
                sqlite3FindInIndex(
                    pParse,
                    pXMod,
                    (4 as i32) as u32,
                    std::ptr::null_mut::<i32>(),
                    aiMap,
                    std::ptr::addr_of_mut!(iTab),
                )
            };
        }
        unsafe { sqlite3ExprDelete(db, pXMod) };
    }
    if eType == (4 as i32) {
        {}
        bRev = !(bRev != (0 as i32)) as i32;
    }
    unsafe {
        sqlite3VdbeAddOp2(
            v,
            if bRev != (0 as i32) {
                32 as i32
            } else {
                36 as i32
            },
            iTab,
            0 as i32,
        )
    };
    {}
    {}
    0 as i32;
    let __v1276: *mut WhereLoop = pLoop;
    let __v1277: u32 = unsafe { (*__v1276).wsFlags };
    let __v1278: u32 = __v1277 | ((2048 as i32) as u32);
    unsafe {
        (*__v1276).wsFlags = __v1278;
    }
    if (unsafe { (*pLevel).u.r#in.nIn }) == (0 as i32) {
        unsafe {
            (*pLevel).addrNxt = unsafe { sqlite3VdbeMakeLabel(pParse) };
        }
    }
    if iEq > (0 as i32)
        && (unsafe { (*pLoop).wsFlags }) & ((1048576 as i32) as u32) == ((0 as i32) as u32)
    {
        let __v1279: *mut WhereLoop = pLoop;
        let __v1280: u32 = unsafe { (*__v1279).wsFlags };
        let __v1281: u32 = __v1280 | ((262144 as i32) as u32);
        unsafe {
            (*__v1279).wsFlags = __v1281;
        }
    }
    i = unsafe { (*pLevel).u.r#in.nIn };
    let __v1282: *mut WhereLevel = pLevel;
    let __v1283: i32 = unsafe { (*__v1282).u.r#in.nIn };
    let __v1284: i32 = __v1283 + nEq;
    unsafe {
        (*__v1282).u.r#in.nIn = __v1284;
    }
    unsafe {
        (*pLevel).u.r#in.aInLoop = (unsafe {
            sqlite3WhereRealloc(
                unsafe { (*unsafe { (*pTerm).pWC }).pWInfo },
                (unsafe { (*pLevel).u.r#in.aInLoop }) as *mut (),
                (20 as u64).wrapping_mul(((unsafe { (*pLevel).u.r#in.nIn }) as i64) as u64),
            )
        }) as *mut InLoop;
    }
    pIn = unsafe { (*pLevel).u.r#in.aInLoop };
    if pIn != std::ptr::null_mut::<InLoop>() {
        let mut iMap: i32 = 0 as i32; // Index in aiMap[]
        let __v1285: *mut InLoop = pIn;
        let __v1286: *mut InLoop = unsafe { __v1285.offset(i as isize) };
        pIn = __v1286;
        i = iEq;
        '__slate_break_1045: loop {
            if !(i < (((unsafe { (*pLoop).nLTerm }) as u32) as i32)) {
                break;
            }
            if (unsafe {
                (*unsafe { *unsafe { unsafe { (*pLoop).aLTerm }.offset(i as isize) } }).pExpr
            }) == pX
            {
                let mut iOut: i32 = iTarget + i - iEq;
                if eType == (1 as i32) {
                    unsafe {
                        (*pIn).addrInTop = unsafe { sqlite3VdbeAddOp2(v, 137 as i32, iTab, iOut) };
                    }
                } else {
                    let mut iCol: i32 = 0 as i32;
                    let __v1289: i32;
                    if aiMap != std::ptr::null_mut::<i32>() {
                        let __v1290: i32 = iMap;
                        let __v1291: i32 = __v1290 + (1 as i32);
                        iMap = __v1291;
                        __v1289 = unsafe { *unsafe { aiMap.offset(__v1290 as isize) } };
                    } else {
                        __v1289 = 0 as i32;
                    }
                    iCol = __v1289;
                    unsafe {
                        (*pIn).addrInTop =
                            unsafe { sqlite3VdbeAddOp3(v, 96 as i32, iTab, iCol, iOut) };
                    }
                }
                unsafe { sqlite3VdbeAddOp1(v, 51 as i32, iOut) };
                {}
                if i == iEq {
                    unsafe {
                        (*pIn).iCur = iTab;
                    }
                    unsafe {
                        (*pIn).eEndLoopOp = ((if bRev != (0 as i32) {
                            39 as i32
                        } else {
                            40 as i32
                        }) as i8) as u8;
                    }
                    if iEq > (0 as i32) {
                        unsafe {
                            (*pIn).iBase = iTarget - i;
                        }
                        unsafe {
                            (*pIn).nPrefix = i;
                        }
                    } else {
                        unsafe {
                            (*pIn).nPrefix = 0 as i32;
                        }
                    }
                } else {
                    unsafe {
                        (*pIn).eEndLoopOp = ((189 as i32) as i8) as u8;
                    }
                }
                let __v1292: *mut InLoop = pIn;
                let __v1293: *mut InLoop = unsafe { __v1292.offset((1 as i32) as isize) };
                pIn = __v1293;
            }
            let __v1287: i32 = i;
            let __v1288: i32 = __v1287 + (1 as i32);
            i = __v1288;
        }
        {}
        if iEq > (0 as i32)
            && (unsafe { (*pLoop).wsFlags }) & (((1048576 as i32) | (1024 as i32)) as u32)
                == ((0 as i32) as u32)
        {
            unsafe {
                sqlite3VdbeAddOp3(v, 127 as i32, unsafe { (*pLevel).iIdxCur }, 0 as i32, iEq)
            };
        }
    } else {
        unsafe {
            (*pLevel).u.r#in.nIn = 0 as i32;
        }
    }
    unsafe { sqlite3DbFree(unsafe { (*pParse).db }, aiMap as *mut ()) };
}

/// Generate code for a single equality term of the WHERE clause.  An equality
/// term can be either X=expr or X IN (...).   pTerm is the term to be
/// coded.
///
/// The current value for the constraint is left in a register, the index
/// of which is returned.  An attempt is made store the result in iTarget but
/// this is only guaranteed for TK_ISNULL and TK_IN constraints.  If the
/// constraint is a TK_EQ or TK_IS, then the current value might be left in
/// some other register and it is the caller's responsibility to compensate.
///
/// For a constraint of the form X=expr, the expression is evaluated in
/// straight-line code.  For constraints of the form X IN (...)
/// this routine sets up a loop that will iterate over all values of X.
///
/// # Arguments
///
/// * `pParse` - The parsing context
/// * `pTerm` - The term of the WHERE clause to be coded
/// * `pLevel` - The level of the FROM clause we are working on
/// * `iEq` - Index of the equality term within this level
/// * `bRev` - True for reverse-order IN operations
/// * `iTarget` - Attempt to leave results in this register
fn codeEqualityTerm(
    mut pParse: *mut Parse,
    mut pTerm: *mut WhereTerm,
    mut pLevel: *mut WhereLevel,
    mut iEq: i32,
    mut bRev: i32,
    mut iTarget: i32,
) -> i32 {
    let mut pX: *mut Expr = unsafe { (*pTerm).pExpr };
    let mut iReg: i32 = 0 as i32; // Register holding results
    0 as i32;
    0 as i32;
    if (((unsafe { (*pX).op }) as u32) as i32) == (54 as i32)
        || (((unsafe { (*pX).op }) as u32) as i32) == (45 as i32)
    {
        iReg = unsafe { sqlite3ExprCodeTarget(pParse, unsafe { (*pX).pRight }, iTarget) };
    } else {
        if (((unsafe { (*pX).op }) as u32) as i32) == (51 as i32) {
            iReg = iTarget;
            unsafe { sqlite3VdbeAddOp2(unsafe { (*pParse).pVdbe }, 77 as i32, 0 as i32, iReg) };
        } else {
            0 as i32;
            iReg = iTarget;
            codeINTerm(pParse, pTerm, pLevel, iEq, bRev, iTarget);
        }
    }
    // As an optimization, try to disable the WHERE clause term that is
    // driving the index as it will always be true.  The correct answer is
    // obtained regardless, but we might get the answer with fewer CPU cycles
    // by omitting the term.
    //
    // But do not disable the term unless we are certain that the term is
    // not a transitive constraint.  For an example of where that does not
    // work, see https://sqlite.org/forum/forumpost/eb8613976a (2021-05-04)
    if (unsafe { (*unsafe { (*pLevel).pWLoop }).wsFlags }) & ((2097152 as i32) as u32)
        == ((0 as i32) as u32)
        || (((unsafe { (*pTerm).eOperator }) as u32) as i32) & (2048 as i32) == (0 as i32)
    {
        disableTerm(pLevel, pTerm);
    }
    return iReg;
}

/// Generate code that will evaluate all == and IN constraints for an
/// index scan.
///
/// For example, consider table t1(a,b,c,d,e,f) with index i1(a,b,c).
/// Suppose the WHERE clause is this:  a==5 AND b IN (1,2,3) AND c>5 AND c<10
/// The index has as many as three equality constraints, but in this
/// example, the third "c" value is an inequality.  So only two
/// constraints are coded.  This routine will generate code to evaluate
/// a==5 and b IN (1,2,3).  The current values for a and b will be stored
/// in consecutive registers and the index of the first register is returned.
///
/// In the example above nEq==2.  But this subroutine works for any value
/// of nEq including 0.  If nEq==0, this routine is nearly a no-op.
/// The only thing it does is allocate the pLevel->iMem memory cell and
/// compute the affinity string.
///
/// The nExtraReg parameter is 0 or 1.  It is 0 if all WHERE clause constraints
/// are == or IN and are covered by the nEq.  nExtraReg is 1 if there is
/// an inequality constraint (such as the "c>=5 AND c<10" in the example) that
/// occurs after the nEq quality constraints.
///
/// This routine allocates a range of nEq+nExtraReg memory cells and returns
/// the index of the first memory cell in that range. The code that
/// calls this routine will use that memory range to store keys for
/// start and termination conditions of the loop.
/// key value of the loop.  If one or more IN operators appear, then
/// this routine allocates an additional nEq memory cells for internal
/// use.
///
/// Before returning, *pzAff is set to point to a buffer containing a
/// copy of the column affinity string of the index allocated using
/// sqlite3DbMalloc(). Except, entries in the copy of the string associated
/// with equality constraints that use BLOB or NONE affinity are set to
/// SQLITE_AFF_BLOB. This is to deal with SQL such as the following:
///
///   CREATE TABLE t1(a TEXT PRIMARY KEY, b);
///   SELECT ... FROM t1 AS t2, t1 WHERE t1.a = t2.b;
///
/// In the example above, the index on t1(a) has TEXT affinity. But since
/// the right hand side of the equality constraint (t2.b) has BLOB/NONE affinity,
/// no conversion should be attempted before using a t2.b value as part of
/// a key to search the index. Hence the first byte in the returned affinity
/// string in this example would be set to SQLITE_AFF_BLOB.
///
/// # Arguments
///
/// * `pParse` - Parsing context
/// * `pLevel` - Which nested loop of the FROM we are coding
/// * `bRev` - Reverse the order of IN operators
/// * `nExtraReg` - Number of extra registers to allocate
/// * `pzAff` - OUT: Set to point to affinity string
fn codeAllEqualityTerms(
    mut pParse: *mut Parse,
    mut pLevel: *mut WhereLevel,
    mut bRev: i32,
    mut nExtraReg: i32,
    mut pzAff: *mut *mut i8,
) -> i32 {
    let mut nEq: u16 = 0 as u16; // The number of == or IN constraints to code
    let mut nSkip: u16 = 0 as u16; // Number of left-most columns to skip
    let mut v: *mut Vdbe = unsafe { (*pParse).pVdbe }; // The vm under construction
    let mut pIdx: *mut Index = unsafe { std::mem::zeroed() }; // The index being used for this loop
    let mut pTerm: *mut WhereTerm = unsafe { std::mem::zeroed() }; // A single constraint term
    let mut pLoop: *mut WhereLoop = unsafe { std::mem::zeroed() }; // The WhereLoop object
    let mut j: i32 = 0 as i32; // Loop counter
    let mut regBase: i32 = 0 as i32; // Base register
    let mut nReg: i32 = 0 as i32; // Number of registers to allocate
    let mut zAff: *mut i8 = unsafe { std::mem::zeroed() }; // Affinity string to return
    // This module is only called on query plans that use an index.
    pLoop = unsafe { (*pLevel).pWLoop };
    0 as i32;
    nEq = unsafe { (*pLoop).u.btree.nEq };
    nSkip = unsafe { (*pLoop).nSkip };
    pIdx = unsafe { (*pLoop).u.btree.pIndex };
    0 as i32;
    // Figure out how many memory cells we will need then allocate them.
    regBase = (unsafe { (*pParse).nMem }) + (1 as i32);
    nReg = ((nEq as u32) as i32) + nExtraReg;
    let __v1294: *mut Parse = pParse;
    let __v1295: i32 = unsafe { (*__v1294).nMem };
    let __v1296: i32 = __v1295 + nReg;
    unsafe {
        (*__v1294).nMem = __v1296;
    }
    zAff = unsafe {
        sqlite3DbStrDup(unsafe { (*pParse).db }, unsafe {
            sqlite3IndexAffinityStr(unsafe { (*pParse).db }, pIdx)
        })
    };
    0 as i32;
    if nSkip != (0 as u16) {
        let mut iIdxCur: i32 = unsafe { (*pLevel).iIdxCur };
        unsafe {
            sqlite3VdbeAddOp3(
                v,
                77 as i32,
                0 as i32,
                regBase,
                regBase + ((nSkip as u32) as i32) - (1 as i32),
            )
        };
        unsafe {
            sqlite3VdbeAddOp1(
                v,
                if bRev != (0 as i32) {
                    32 as i32
                } else {
                    36 as i32
                },
                iIdxCur,
            )
        };
        {}
        {}
        unsafe {
            sqlite3VdbeComment(
                v,
                (b"begin skip-scan on %s\0".as_ptr() as *mut i8) as *const i8,
                unsafe { (*pIdx).zName },
            )
        };
        j = unsafe { sqlite3VdbeAddOp0(v, 9 as i32) };
        0 as i32;
        unsafe {
            (*pLevel).addrSkip = unsafe {
                sqlite3VdbeAddOp4Int(
                    v,
                    if bRev != (0 as i32) {
                        21 as i32
                    } else {
                        24 as i32
                    },
                    iIdxCur,
                    0 as i32,
                    regBase,
                    (nSkip as u32) as i32,
                )
            };
        }
        {}
        {}
        unsafe { sqlite3VdbeJumpHere(v, j) };
        j = 0 as i32;
        '__slate_break_1047: loop {
            if !(j < ((nSkip as u32) as i32)) {
                break;
            }
            unsafe { sqlite3VdbeAddOp3(v, 96 as i32, iIdxCur, j, regBase + j) };
            {}
            unsafe {
                sqlite3VdbeComment(
                    v,
                    (b"%s\0".as_ptr() as *mut i8) as *const i8,
                    explainIndexColumnName(pIdx, j),
                )
            };
            let __v1297: i32 = j;
            let __v1298: i32 = __v1297 + (1 as i32);
            j = __v1298;
        }
    }
    // Evaluate the equality constraints
    0 as i32;
    j = (nSkip as u32) as i32;
    '__slate_break_1049: loop {
        if !(j < ((nEq as u32) as i32)) {
            break;
        }
        let mut r1: i32 = 0 as i32;
        pTerm = unsafe { *unsafe { unsafe { (*pLoop).aLTerm }.offset(j as isize) } };
        0 as i32;
        // The following testcase is true for indices with redundant columns.
        // Ex: CREATE INDEX i1 ON t1(a,b,a); SELECT * FROM t1 WHERE a=0 AND b=0;
        {}
        {}
        r1 = codeEqualityTerm(pParse, pTerm, pLevel, j, bRev, regBase + j);
        if r1 != regBase + j {
            // If this routine is being called as part of a RIGHT JOIN loop, then
            // register r1 may be used by the body of the loop that the RIGHT JOIN
            // will jump back into (e.g. if pTerm is a sub-query). This can cause
            // problems if (say) the affinity of r1 is modified by the caller of
            // this routine. So, always take a copy of the value in this case.
            if nReg == (1 as i32)
                && (((unsafe { (*pParse).withinRJSubrtn }) as u32) as i32) == (0 as i32)
            {
                unsafe { sqlite3ReleaseTempReg(pParse, regBase) };
                regBase = r1;
            } else {
                unsafe { sqlite3VdbeAddOp2(v, 82 as i32, r1, regBase + j) };
            }
        }
        if (((unsafe { (*pTerm).eOperator }) as u32) as i32) & (1 as i32) != (0 as i32) {
            if (unsafe { (*unsafe { (*pTerm).pExpr }).flags }) & ((4096 as i32) as u32)
                != (0 as u32)
            {
                // No affinity ever needs to be (or should be) applied to a value
                // from the RHS of an "? IN (SELECT ...)" expression. The
                // sqlite3FindInIndex() routine has already ensured that the
                // affinity of the comparison has been applied to the value.
                if zAff != std::ptr::null_mut::<i8>() {
                    unsafe {
                        *unsafe { zAff.offset(j as isize) } = (65 as i32) as i8;
                    }
                }
            }
        } else {
            if (((unsafe { (*pTerm).eOperator }) as u32) as i32) & (256 as i32) == (0 as i32) {
                let mut pRight: *mut Expr = unsafe { (*unsafe { (*pTerm).pExpr }).pRight };
                let __v1301: bool;
                if (((unsafe { (*pTerm).wtFlags }) as u32) as i32) & (2048 as i32) == (0 as i32) {
                    __v1301 =
                        (unsafe { sqlite3ExprCanBeNull(pRight as *const Expr) }) != (0 as i32);
                } else {
                    __v1301 = false as bool;
                }
                if __v1301 {
                    unsafe {
                        sqlite3VdbeAddOp2(v, 51 as i32, regBase + j, unsafe { (*pLevel).addrBrk })
                    };
                    {}
                }
                if (unsafe { (*pParse).nErr }) == (0 as i32) {
                    0 as i32;
                    if ((unsafe {
                        sqlite3CompareAffinity(pRight as *const Expr, unsafe {
                            *unsafe { zAff.offset(j as isize) }
                        })
                    }) as i32)
                        == (65 as i32)
                    {
                        unsafe {
                            *unsafe { zAff.offset(j as isize) } = (65 as i32) as i8;
                        }
                    }
                    if (unsafe {
                        sqlite3ExprNeedsNoAffinityChange(pRight as *const Expr, unsafe {
                            *unsafe { zAff.offset(j as isize) }
                        })
                    }) != (0 as i32)
                    {
                        unsafe {
                            *unsafe { zAff.offset(j as isize) } = (65 as i32) as i8;
                        }
                    }
                }
            }
        }
        let __v1299: i32 = j;
        let __v1300: i32 = __v1299 + (1 as i32);
        j = __v1300;
    }
    unsafe {
        *pzAff = zAff;
    }
    return regBase;
}

/// If the most recently coded instruction is a constant range constraint
/// (a string literal) that originated from the LIKE optimization, then
/// set P3 and P5 on the OP_String opcode so that the string will be cast
/// to a BLOB at appropriate times.
///
/// The LIKE optimization trys to evaluate "x LIKE 'abc%'" as a range
/// expression: "x>='ABC' AND x<'abd'".  But this requires that the range
/// scan loop run twice, once for strings and a second time for BLOBs.
/// The OP_String opcodes on the second pass convert the upper and lower
/// bound string constants to blobs.  This routine makes the necessary changes
/// to the OP_String opcodes for that to happen.
///
/// Except, of course, if SQLITE_LIKE_DOESNT_MATCH_BLOBS is defined, then
/// only the one pass through the string space is required, so this routine
/// becomes a no-op.
///
/// # Arguments
///
/// * `v` - prepared statement under construction
/// * `pLevel` - The loop that contains the LIKE operator
/// * `pTerm` - The upper or lower bound just coded
fn whereLikeOptimizationStringFixup(
    mut v: *mut Vdbe,
    mut pLevel: *mut WhereLevel,
    mut pTerm: *mut WhereTerm,
) {
    if (((unsafe { (*pTerm).wtFlags }) as u32) as i32) & (256 as i32) != (0 as i32) {
        let mut pOp: *mut VdbeOp = unsafe { std::mem::zeroed() };
        0 as i32;
        pOp = unsafe { sqlite3VdbeGetLastOp(v) };
        0 as i32;
        0 as i32;
        unsafe {
            (*pOp).p3 = ((unsafe { (*pLevel).iLikeRepCntr }) >> (1 as i32)) as i32;
        }
        // Register holding counter
        unsafe {
            (*pOp).p5 = (((unsafe { (*pLevel).iLikeRepCntr }) & ((1 as i32) as u32)) as u8) as u16;
        }
        // ASC or DESC
    }
}

// No-op
/// Cursor iCur is open on an intkey b-tree (a table). Register iRowid contains
/// a rowid value just read from cursor iIdxCur, open on index pIdx. This
/// function generates code to do a deferred seek of cursor iCur to the
/// rowid stored in register iRowid.
///
/// Normally, this is just:
///
///   OP_DeferredSeek $iCur $iRowid
///
/// Which causes a seek on $iCur to the row with rowid $iRowid.
///
/// However, if the scan currently being coded is a branch of an OR-loop and
/// the statement currently being coded is a SELECT, then additional information
/// is added that might allow OP_Column to omit the seek and instead do its
/// lookup on the index, thus avoiding an expensive seek operation.  To
/// enable this optimization, the P3 of OP_DeferredSeek is set to iIdxCur
/// and P4 is set to an array of integers containing one entry for each column
/// in the table.  For each table column, if the column is the i'th
/// column of the index, then the corresponding array entry is set to (i+1).
/// If the column does not appear in the index at all, the array entry is set
/// to 0.  The OP_Column opcode can check this array to see if the column it
/// wants is in the index and if it is, it will substitute the index cursor
/// and column number and continue with those new values, rather than seeking
/// the table cursor.
///
/// # Arguments
///
/// * `pWInfo` - Where clause context
/// * `pIdx` - Index scan is using
/// * `iCur` - Cursor for IPK b-tree
/// * `iIdxCur` - Index cursor
fn codeDeferredSeek(
    mut pWInfo: *mut WhereInfo,
    mut pIdx: *mut Index,
    mut iCur: i32,
    mut iIdxCur: i32,
) {
    let mut pParse: *mut Parse = unsafe { (*pWInfo).pParse }; // Parse context
    let mut v: *mut Vdbe = unsafe { (*pParse).pVdbe }; // Vdbe to generate code within
    0 as i32;
    0 as i32;
    unsafe {
        (*pWInfo)
            .__slate_bits_0
            .__set_bDeferredSeek((1 as i32) as u32);
    }
    unsafe { sqlite3VdbeAddOp3(v, 143 as i32, iIdxCur, 0 as i32, iCur) };
    if (((unsafe { (*pWInfo).wctrlFlags }) as u32) as i32) & ((32 as i32) | (4096 as i32))
        != (0 as i32)
        && (unsafe {
            (*if (unsafe { (*pParse).pToplevel }) != std::ptr::null_mut::<Parse>() {
                unsafe { (*pParse).pToplevel }
            } else {
                pParse
            })
            .writeMask
        }) == ((0 as i32) as u32)
    {
        let mut i: i32 = 0 as i32;
        let mut pTab: *mut Table = unsafe { (*pIdx).pTable };
        let mut ai: *mut u32 = (unsafe {
            sqlite3DbMallocZero(
                unsafe { (*pParse).db },
                (4 as u64).wrapping_mul(
                    ((((unsafe { (*pTab).nCol }) as i32) + (1 as i32)) as i64) as u64,
                ),
            )
        }) as *mut u32;
        if ai != std::ptr::null_mut::<u32>() {
            unsafe {
                *unsafe { ai.offset((0 as i32) as isize) } =
                    ((unsafe { (*pTab).nCol }) as i32) as u32;
            }
            i = 0 as i32;
            '__slate_break_1050: loop {
                if !(i < (((unsafe { (*pIdx).nColumn }) as u32) as i32) - (1 as i32)) {
                    break;
                }
                let mut x1: i32 = 0 as i32;
                let mut x2: i32 = 0 as i32;
                0 as i32;
                x1 = (unsafe { *unsafe { unsafe { (*pIdx).aiColumn }.offset(i as isize) } }) as i32;
                x2 = (unsafe { sqlite3TableColumnToStorage(pTab, x1 as i16) }) as i32;
                {}
                if x1 >= (0 as i32) {
                    unsafe {
                        *unsafe { ai.offset((x2 + (1 as i32)) as isize) } = (i + (1 as i32)) as u32;
                    }
                }
                let __v1302: i32 = i;
                let __v1303: i32 = __v1302 + (1 as i32);
                i = __v1303;
            }
            unsafe {
                sqlite3VdbeChangeP4(v, -(1 as i32), (ai as *mut i8) as *const i8, -(13 as i32))
            };
        }
    }
}

/// If the expression passed as the second argument is a vector, generate
/// code to write the first nReg elements of the vector into an array
/// of registers starting with iReg.
///
/// If the expression is not a vector, then nReg must be passed 1. In
/// this case, generate code to evaluate the expression and leave the
/// result in register iReg.
fn codeExprOrVector(mut pParse: *mut Parse, mut p: *mut Expr, mut iReg: i32, mut nReg: i32) {
    0 as i32;
    let __v1304: bool;
    if p != std::ptr::null_mut::<Expr>() {
        __v1304 = (unsafe { sqlite3ExprIsVector(p as *const Expr) }) != (0 as i32);
    } else {
        __v1304 = false as bool;
    }
    if __v1304 {
        if (unsafe { (*p).flags }) & ((4096 as i32) as u32) != ((0 as i32) as u32) {
            let mut v: *mut Vdbe = unsafe { (*pParse).pVdbe };
            let mut iSelect: i32 = 0 as i32;
            0 as i32;
            iSelect = unsafe { sqlite3CodeSubselect(pParse, p) };
            unsafe { sqlite3VdbeAddOp3(v, 82 as i32, iSelect, iReg, nReg - (1 as i32)) };
        } else {
            let mut i: i32 = 0 as i32;
            let mut pList: *const ExprList = unsafe { std::mem::zeroed() };
            0 as i32;
            pList = (unsafe { (*p).x.pList }) as *const ExprList;
            0 as i32;
            i = 0 as i32;
            '__slate_break_1051: loop {
                if !(i < nReg) {
                    break;
                }
                unsafe {
                    sqlite3ExprCode(
                        pParse,
                        unsafe {
                            (*unsafe {
                                unsafe { std::ptr::addr_of!((*pList).a) as *const ExprList_item }
                                    .offset(i as isize)
                            })
                            .pExpr
                        },
                        iReg + i,
                    )
                };
                let __v1305: i32 = i;
                let __v1306: i32 = __v1305 + (1 as i32);
                i = __v1306;
            }
        }
    } else {
        0 as i32;
        unsafe { sqlite3ExprCode(pParse, p, iReg) };
    }
}

/// The pTruth expression is always true because it is the WHERE clause
/// a partial index that is driving a query loop.  Look through all of the
/// WHERE clause terms on the query, and if any of those terms must be
/// true because pTruth is true, then mark those WHERE clause terms as
/// coded.
fn whereApplyPartialIndexConstraints(
    mut pTruth: *mut Expr,
    mut iTabCur: i32,
    mut pWC: *mut WhereClause,
) {
    let mut i: i32 = 0 as i32;
    let mut pTerm: *mut WhereTerm = unsafe { std::mem::zeroed() };
    '__slate_break_1052: while (((unsafe { (*pTruth).op }) as u32) as i32) == (44 as i32) {
        whereApplyPartialIndexConstraints(unsafe { (*pTruth).pLeft }, iTabCur, pWC);
        pTruth = unsafe { (*pTruth).pRight };
    }
    i = 0 as i32;
    let __v1307: *mut WhereTerm = unsafe { (*pWC).a };
    pTerm = __v1307;
    '__slate_break_1053: while i < unsafe { (*pWC).nTerm } {
        let mut pExpr: *mut Expr = unsafe { std::mem::zeroed() };
        if (((unsafe { (*pTerm).wtFlags }) as u32) as i32) & (4 as i32) != (0 as i32) {
        } else {
            pExpr = unsafe { (*pTerm).pExpr };
            if (unsafe {
                sqlite3ExprCompare(
                    std::ptr::null::<Parse>(),
                    pExpr as *const Expr,
                    pTruth as *const Expr,
                    iTabCur,
                )
            }) == (0 as i32)
            {
                let __v1312: *mut WhereTerm = pTerm;
                let __v1313: u16 = unsafe { (*__v1312).wtFlags };
                let __v1314: u16 = ((((__v1313 as u32) as i32) | (4 as i32)) as i16) as u16;
                unsafe {
                    (*__v1312).wtFlags = __v1314;
                }
            }
        }
        let __v1308: i32 = i;
        let __v1309: i32 = __v1308 + (1 as i32);
        i = __v1309;
        let __v1310: *mut WhereTerm = pTerm;
        let __v1311: *mut WhereTerm = unsafe { __v1310.offset((1 as i32) as isize) };
        pTerm = __v1311;
    }
}

/// This routine is called right after An OP_Filter has been generated and
/// before the corresponding index search has been performed.  This routine
/// checks to see if there are additional Bloom filters in inner loops that
/// can be checked prior to doing the index lookup.  If there are available
/// inner-loop Bloom filters, then evaluate those filters now, before the
/// index lookup.  The idea is that a Bloom filter check is way faster than
/// an index lookup, and the Bloom filter might return false, meaning that
/// the index lookup can be skipped.
///
/// We know that an inner loop uses a Bloom filter because it has the
/// WhereLevel.regFilter set.  If an inner-loop Bloom filter is checked,
/// then clear the WhereLevel.regFilter value to prevent the Bloom filter
/// from being checked a second time when the inner loop is evaluated.
///
/// # Arguments
///
/// * `pParse` - Parsing context
/// * `pWInfo` - Complete information about the WHERE clause
/// * `iLevel` - Which level of pWInfo->a[] should be coded
/// * `addrNxt` - Jump here to bypass inner loops
/// * `notReady` - Loops that are not ready
fn filterPullDown(
    mut pParse: *mut Parse,
    mut pWInfo: *mut WhereInfo,
    mut iLevel: i32,
    mut addrNxt: i32,
    mut notReady: u64,
) {
    let mut saved_addrBrk: i32 = 0 as i32;
    '__slate_break_1054: loop {
        let __v1315: i32 = iLevel;
        let __v1316: i32 = __v1315 + (1 as i32);
        iLevel = __v1316;
        if !(__v1316 < (((unsafe { (*pWInfo).nLevel }) as u32) as i32)) {
            break;
        }
        let mut pLevel: *mut WhereLevel = unsafe {
            unsafe { std::ptr::addr_of_mut!((*pWInfo).a) as *mut WhereLevel }
                .offset(iLevel as isize)
        };
        let mut pLoop: *mut WhereLoop = unsafe { (*pLevel).pWLoop };
        if (unsafe { (*pLevel).regFilter }) == (0 as i32) {
        } else {
            if (unsafe { (*unsafe { (*pLevel).pWLoop }).nSkip }) != (0 as u16) {
            } else {
                //         ,--- Because sqlite3ConstructBloomFilter() has will not have set
                // vvvvv--'    pLevel->regFilter if this were true.
                if (unsafe { (*pLoop).prereq }) & notReady != (0 as u64) {
                } else {
                    saved_addrBrk = unsafe { (*pLevel).addrBrk };
                    unsafe {
                        (*pLevel).addrBrk = addrNxt;
                    }
                    if (unsafe { (*pLoop).wsFlags }) & ((256 as i32) as u32) != (0 as u32) {
                        let mut pTerm: *mut WhereTerm = unsafe {
                            *unsafe { unsafe { (*pLoop).aLTerm }.offset((0 as i32) as isize) }
                        };
                        let mut regRowid: i32 = 0 as i32;
                        0 as i32;
                        0 as i32;
                        {}
                        regRowid = unsafe { sqlite3GetTempReg(pParse) };
                        regRowid =
                            codeEqualityTerm(pParse, pTerm, pLevel, 0 as i32, 0 as i32, regRowid);
                        unsafe {
                            sqlite3VdbeAddOp2(
                                unsafe { (*pParse).pVdbe },
                                13 as i32,
                                regRowid,
                                addrNxt,
                            )
                        };
                        {}
                        0 as i32;
                        unsafe {
                            sqlite3VdbeAddOp4Int(
                                unsafe { (*pParse).pVdbe },
                                66 as i32,
                                unsafe { (*pLevel).regFilter },
                                addrNxt,
                                regRowid,
                                1 as i32,
                            )
                        };
                        {}
                    } else {
                        let mut nEq: u16 = unsafe { (*pLoop).u.btree.nEq };
                        let mut r1: i32 = 0 as i32;
                        let mut zStartAff: *mut i8 = unsafe { std::mem::zeroed() };
                        0 as i32;
                        0 as i32;
                        r1 = codeAllEqualityTerms(
                            pParse,
                            pLevel,
                            0 as i32,
                            0 as i32,
                            std::ptr::addr_of_mut!(zStartAff),
                        );
                        codeApplyAffinity(pParse, r1, (nEq as u32) as i32, zStartAff);
                        unsafe { sqlite3DbFree(unsafe { (*pParse).db }, zStartAff as *mut ()) };
                        0 as i32;
                        unsafe {
                            sqlite3VdbeAddOp4Int(
                                unsafe { (*pParse).pVdbe },
                                66 as i32,
                                unsafe { (*pLevel).regFilter },
                                addrNxt,
                                r1,
                                (nEq as u32) as i32,
                            )
                        };
                        {}
                    }
                    unsafe {
                        (*pLevel).regFilter = 0 as i32;
                    }
                    unsafe {
                        (*pLevel).addrBrk = saved_addrBrk;
                    }
                }
            }
        }
    }
}

/// Loop pLoop is a WHERE_INDEXED level that uses at least one IN(...)
/// operator. Return true if level pLoop is guaranteed to visit only one
/// row for each key generated for the index.
fn whereLoopIsOneRow(mut pLoop: *mut WhereLoop) -> i32 {
    if (unsafe { (*unsafe { (*pLoop).u.btree.pIndex }).onError }) != (0 as u8)
        && (((unsafe { (*pLoop).nSkip }) as u32) as i32) == (0 as i32)
        && (((unsafe { (*pLoop).u.btree.nEq }) as u32) as i32)
            == (((unsafe { (*unsafe { (*pLoop).u.btree.pIndex }).nKeyCol }) as u32) as i32)
    {
        let mut ii: i32 = 0 as i32;
        ii = 0 as i32;
        '__slate_break_1055: loop {
            if !(ii < (((unsafe { (*pLoop).u.btree.nEq }) as u32) as i32)) {
                break;
            }
            if (((unsafe {
                (*unsafe { *unsafe { unsafe { (*pLoop).aLTerm }.offset(ii as isize) } }).eOperator
            }) as u32) as i32)
                & ((128 as i32) | (256 as i32))
                != (0 as i32)
            {
                return 0 as i32;
            }
            let __v1317: i32 = ii;
            let __v1318: i32 = __v1317 + (1 as i32);
            ii = __v1318;
        }
        return 1 as i32;
    }
    return 0 as i32;
}

/// Generate code for the start of the iLevel-th loop in the WHERE clause
/// implementation described by pWInfo.
///
/// # Arguments
///
/// * `pParse` - Parsing context
/// * `v` - Prepared statement under construction
/// * `pWInfo` - Complete information about the WHERE clause
/// * `iLevel` - Which level of pWInfo->a[] should be coded
/// * `pLevel` - The current level pointer
/// * `notReady` - Which tables are currently available
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3WhereCodeOneLoopStart(
    mut pParse: *mut Parse,
    mut v: *mut Vdbe,
    mut pWInfo: *mut WhereInfo,
    mut iLevel: i32,
    mut pLevel: *mut WhereLevel,
    mut notReady: u64,
) -> u64 {
    let mut __slate_storage_1202: std::mem::MaybeUninit<*mut WhereTerm> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1202: *mut *mut WhereTerm =
        std::ptr::addr_of_mut!(__slate_storage_1202) as *mut *mut WhereTerm;
    let mut __slate_storage_1201: std::mem::MaybeUninit<*mut WhereTerm> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1201: *mut *mut WhereTerm =
        std::ptr::addr_of_mut!(__slate_storage_1201) as *mut *mut WhereTerm;
    let mut __slate_storage_1200: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1200: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1200) as *mut i32;
    let mut __slate_storage_1199: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1199: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1199) as *mut i32;
    let mut __slate_storage_1205: std::mem::MaybeUninit<u16> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1205: *mut u16 = std::ptr::addr_of_mut!(__slate_storage_1205) as *mut u16;
    let mut __slate_storage_1204: std::mem::MaybeUninit<u16> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1204: *mut u16 = std::ptr::addr_of_mut!(__slate_storage_1204) as *mut u16;
    let mut __slate_storage_1203: std::mem::MaybeUninit<*mut WhereTerm> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1203: *mut *mut WhereTerm =
        std::ptr::addr_of_mut!(__slate_storage_1203) as *mut *mut WhereTerm;
    let mut __slate_storage_1198: std::mem::MaybeUninit<u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1198: *mut u8 = std::ptr::addr_of_mut!(__slate_storage_1198) as *mut u8;
    let mut __slate_storage_1197: std::mem::MaybeUninit<u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1197: *mut u8 = std::ptr::addr_of_mut!(__slate_storage_1197) as *mut u8;
    let mut __slate_storage_1196: std::mem::MaybeUninit<*mut Parse> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1196: *mut *mut Parse =
        std::ptr::addr_of_mut!(__slate_storage_1196) as *mut *mut Parse;
    // Create a subroutine used to process all interior loops and code
    // of the RIGHT JOIN.  During normal operation, the subroutine will
    // be in-line with the rest of the code.  But at the end, a separate
    // loop will run that invokes this subroutine for unmatched rows
    // of pTab, with all tables to left begin set to NULL.
    let mut __slate_storage_747: std::mem::MaybeUninit<*mut WhereRightJoin> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_747: *mut *mut WhereRightJoin =
        std::ptr::addr_of_mut!(__slate_storage_747) as *mut *mut WhereRightJoin;
    let mut __slate_storage_1195: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1195: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1195) as *mut i32;
    let mut __slate_storage_1194: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1194: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1194) as *mut i32;
    let mut __slate_storage_746: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_746: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_746) as *mut i32;
    let mut __slate_storage_745: std::mem::MaybeUninit<*mut Index> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_745: *mut *mut Index =
        std::ptr::addr_of_mut!(__slate_storage_745) as *mut *mut Index;
    let mut __slate_storage_744: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_744: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_744) as *mut i32;
    let mut __slate_storage_743: std::mem::MaybeUninit<*mut WhereRightJoin> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_743: *mut *mut WhereRightJoin =
        std::ptr::addr_of_mut!(__slate_storage_743) as *mut *mut WhereRightJoin;
    let mut __slate_storage_742: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_742: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_742) as *mut i32;
    let mut __slate_storage_741: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_741: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_741) as *mut i32;
    let mut __slate_storage_740: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_740: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_740) as *mut i32;
    let mut __slate_storage_739: std::mem::MaybeUninit<*mut Table> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_739: *mut *mut Table =
        std::ptr::addr_of_mut!(__slate_storage_739) as *mut *mut Table;
    let mut __slate_storage_1190: std::mem::MaybeUninit<*mut WhereTerm> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1190: *mut *mut WhereTerm =
        std::ptr::addr_of_mut!(__slate_storage_1190) as *mut *mut WhereTerm;
    let mut __slate_storage_1189: std::mem::MaybeUninit<*mut WhereTerm> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1189: *mut *mut WhereTerm =
        std::ptr::addr_of_mut!(__slate_storage_1189) as *mut *mut WhereTerm;
    let mut __slate_storage_1188: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1188: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1188) as *mut i32;
    let mut __slate_storage_1187: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1187: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1187) as *mut i32;
    let mut __slate_storage_1193: std::mem::MaybeUninit<u16> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1193: *mut u16 = std::ptr::addr_of_mut!(__slate_storage_1193) as *mut u16;
    let mut __slate_storage_1192: std::mem::MaybeUninit<u16> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1192: *mut u16 = std::ptr::addr_of_mut!(__slate_storage_1192) as *mut u16;
    let mut __slate_storage_1191: std::mem::MaybeUninit<*mut WhereTerm> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1191: *mut *mut WhereTerm =
        std::ptr::addr_of_mut!(__slate_storage_1191) as *mut *mut WhereTerm;
    let mut __slate_storage_738: std::mem::MaybeUninit<*mut WhereTerm> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_738: *mut *mut WhereTerm =
        std::ptr::addr_of_mut!(__slate_storage_738) as *mut *mut WhereTerm;
    let mut __slate_storage_737: std::mem::MaybeUninit<Expr> = std::mem::MaybeUninit::uninit();
    let __slate_slot_737: *mut Expr = std::ptr::addr_of_mut!(__slate_storage_737) as *mut Expr;
    let mut __slate_storage_736: std::mem::MaybeUninit<*mut Expr> = std::mem::MaybeUninit::uninit();
    let __slate_slot_736: *mut *mut Expr =
        std::ptr::addr_of_mut!(__slate_storage_736) as *mut *mut Expr;
    let mut __slate_storage_1186: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1186: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1186) as *mut i32;
    let mut __slate_storage_1181: std::mem::MaybeUninit<*mut WhereTerm> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1181: *mut *mut WhereTerm =
        std::ptr::addr_of_mut!(__slate_storage_1181) as *mut *mut WhereTerm;
    let mut __slate_storage_1180: std::mem::MaybeUninit<*mut WhereTerm> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1180: *mut *mut WhereTerm =
        std::ptr::addr_of_mut!(__slate_storage_1180) as *mut *mut WhereTerm;
    let mut __slate_storage_1179: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1179: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1179) as *mut i32;
    let mut __slate_storage_1178: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1178: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1178) as *mut i32;
    let mut __slate_storage_1185: std::mem::MaybeUninit<u16> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1185: *mut u16 = std::ptr::addr_of_mut!(__slate_storage_1185) as *mut u16;
    let mut __slate_storage_1184: std::mem::MaybeUninit<u16> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1184: *mut u16 = std::ptr::addr_of_mut!(__slate_storage_1184) as *mut u16;
    let mut __slate_storage_1183: std::mem::MaybeUninit<*mut WhereTerm> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1183: *mut *mut WhereTerm =
        std::ptr::addr_of_mut!(__slate_storage_1183) as *mut *mut WhereTerm;
    // If the TERM_LIKECOND flag is set, that means that the range search
    // is sufficient to guarantee that the LIKE operator is true, so we
    // can skip the call to the like(A,B) function.  But this only works
    // for strings.  So do not skip the call to the function on the pass
    // that compares BLOBs.
    let mut __slate_storage_735: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_735: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_735) as *mut u32;
    let mut __slate_storage_1182: std::mem::MaybeUninit<bool> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1182: *mut bool = std::ptr::addr_of_mut!(__slate_storage_1182) as *mut bool;
    let mut __slate_storage_734: std::mem::MaybeUninit<u64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_734: *mut u64 = std::ptr::addr_of_mut!(__slate_storage_734) as *mut u64;
    let mut __slate_storage_733: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_733: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_733) as *mut i32;
    let mut __slate_storage_732: std::mem::MaybeUninit<*mut Expr> = std::mem::MaybeUninit::uninit();
    let __slate_slot_732: *mut *mut Expr =
        std::ptr::addr_of_mut!(__slate_storage_732) as *mut *mut Expr;
    let mut __slate_storage_1177: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1177: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1177) as *mut i32; // Next value for iLoop
    let mut __slate_storage_731: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_731: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_731) as *mut i32;
    let mut __slate_storage_645: std::mem::MaybeUninit<*mut Subquery> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_645: *mut *mut Subquery =
        std::ptr::addr_of_mut!(__slate_storage_645) as *mut *mut Subquery;
    let mut __slate_storage_644: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_644: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_644) as *mut i32;
    let mut __slate_storage_1102: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1102: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1102) as *mut i32;
    let mut __slate_storage_1101: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1101: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1101) as *mut i32;
    let mut __slate_storage_1105: std::mem::MaybeUninit<*mut Expr> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1105: *mut *mut Expr =
        std::ptr::addr_of_mut!(__slate_storage_1105) as *mut *mut Expr;
    let mut __slate_storage_658: std::mem::MaybeUninit<*mut Expr> = std::mem::MaybeUninit::uninit();
    let __slate_slot_658: *mut *mut Expr =
        std::ptr::addr_of_mut!(__slate_storage_658) as *mut *mut Expr;
    let mut __slate_storage_657: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_657: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_657) as *mut i32;
    let mut __slate_storage_1104: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1104: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1104) as *mut i32;
    let mut __slate_storage_1103: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1103: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1103) as *mut i32; // IN loop corresponding to the j-th constraint
    let mut __slate_storage_656: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_656: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_656) as *mut i32; // Opcode to access the value of the IN constraint
    let mut __slate_storage_655: std::mem::MaybeUninit<*mut VdbeOp> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_655: *mut *mut VdbeOp =
        std::ptr::addr_of_mut!(__slate_storage_655) as *mut *mut VdbeOp; // RHS of the comparison
    let mut __slate_storage_654: std::mem::MaybeUninit<*mut Expr> = std::mem::MaybeUninit::uninit();
    let __slate_slot_654: *mut *mut Expr =
        std::ptr::addr_of_mut!(__slate_storage_654) as *mut *mut Expr; // The comparison operator
    let mut __slate_storage_653: std::mem::MaybeUninit<*mut Expr> = std::mem::MaybeUninit::uninit();
    let __slate_slot_653: *mut *mut Expr =
        std::ptr::addr_of_mut!(__slate_storage_653) as *mut *mut Expr;
    let mut __slate_storage_1094: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1094: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1094) as *mut i32;
    let mut __slate_storage_1093: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1093: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1093) as *mut i32;
    let mut __slate_storage_1100: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1100: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1100) as *mut i32;
    let mut __slate_storage_1099: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1099: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1099) as *mut i32;
    let mut __slate_storage_1098: std::mem::MaybeUninit<*mut Parse> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1098: *mut *mut Parse =
        std::ptr::addr_of_mut!(__slate_storage_1098) as *mut *mut Parse;
    let mut __slate_storage_651: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_651: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_651) as *mut i32;
    let mut __slate_storage_1097: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1097: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1097) as *mut i32;
    let mut __slate_storage_1096: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1096: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1096) as *mut i32;
    let mut __slate_storage_1095: std::mem::MaybeUninit<*mut Parse> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1095: *mut *mut Parse =
        std::ptr::addr_of_mut!(__slate_storage_1095) as *mut *mut Parse;
    let mut __slate_storage_650: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_650: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_650) as *mut i32;
    let mut __slate_storage_652: std::mem::MaybeUninit<*mut Expr> = std::mem::MaybeUninit::uninit();
    let __slate_slot_652: *mut *mut Expr =
        std::ptr::addr_of_mut!(__slate_storage_652) as *mut *mut Expr;
    let mut __slate_storage_649: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_649: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_649) as *mut i32;
    let mut __slate_storage_648: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_648: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_648) as *mut i32;
    let mut __slate_storage_647: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_647: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_647) as *mut i32;
    // Case 1:  The table is a virtual-table.  Use the VFilter and VNext
    // to access the data.
    // P3 Value for OP_VFilter
    let mut __slate_storage_646: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_646: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_646) as *mut i32;
    let mut __slate_storage_1108: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1108: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1108) as *mut i32;
    let mut __slate_storage_1107: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1107: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1107) as *mut i32;
    let mut __slate_storage_1106: std::mem::MaybeUninit<*mut Parse> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1106: *mut *mut Parse =
        std::ptr::addr_of_mut!(__slate_storage_1106) as *mut *mut Parse;
    let mut __slate_storage_1119: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1119: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1119) as *mut i32;
    let mut __slate_storage_1118: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1118: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1118) as *mut i32;
    let mut __slate_storage_1117: std::mem::MaybeUninit<*mut Parse> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1117: *mut *mut Parse =
        std::ptr::addr_of_mut!(__slate_storage_1117) as *mut *mut Parse;
    let mut __slate_storage_1116: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1116: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1116) as *mut i32;
    let mut __slate_storage_1115: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1115: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1115) as *mut i32;
    let mut __slate_storage_1114: std::mem::MaybeUninit<*mut Parse> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1114: *mut *mut Parse =
        std::ptr::addr_of_mut!(__slate_storage_1114) as *mut *mut Parse;
    let mut __slate_storage_669: std::mem::MaybeUninit<*mut Expr> = std::mem::MaybeUninit::uninit();
    let __slate_slot_669: *mut *mut Expr =
        std::ptr::addr_of_mut!(__slate_storage_669) as *mut *mut Expr;
    let mut __slate_storage_1113: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1113: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1113) as *mut i32;
    // The following constant maps TK_xx codes into corresponding
    // seek opcodes.  It depends on a particular ordering of TK_xx
    // TK_GT
    // TK_LE
    // TK_LT
    // TK_GE
    let mut __slate_storage_668: std::mem::MaybeUninit<[u8; 4]> = std::mem::MaybeUninit::uninit();
    let __slate_slot_668: *mut [u8; 4] =
        std::ptr::addr_of_mut!(__slate_storage_668) as *mut [u8; 4]; // Cursor seek operation
    let mut __slate_storage_667: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_667: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_667) as *mut i32; // Registers for holding the start boundary
    let mut __slate_storage_666: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_666: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_666) as *mut i32;
    let mut __slate_storage_665: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_665: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_665) as *mut i32; // The expression that defines the start bound
    let mut __slate_storage_664: std::mem::MaybeUninit<*mut Expr> = std::mem::MaybeUninit::uninit();
    let __slate_slot_664: *mut *mut Expr =
        std::ptr::addr_of_mut!(__slate_storage_664) as *mut *mut Expr;
    let mut __slate_storage_1112: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1112: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1112) as *mut i32;
    let mut __slate_storage_1111: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1111: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1111) as *mut i32;
    let mut __slate_storage_1110: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1110: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1110) as *mut i32;
    let mut __slate_storage_1109: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1109: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1109) as *mut i32;
    let mut __slate_storage_663: std::mem::MaybeUninit<*mut WhereTerm> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_663: *mut *mut WhereTerm =
        std::ptr::addr_of_mut!(__slate_storage_663) as *mut *mut WhereTerm;
    let mut __slate_storage_662: std::mem::MaybeUninit<*mut WhereTerm> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_662: *mut *mut WhereTerm =
        std::ptr::addr_of_mut!(__slate_storage_662) as *mut *mut WhereTerm;
    let mut __slate_storage_661: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_661: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_661) as *mut i32;
    let mut __slate_storage_660: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_660: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_660) as *mut i32;
    // Case 3:  We have an inequality comparison against the ROWID field.
    let mut __slate_storage_659: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_659: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_659) as *mut i32;
    let mut __slate_storage_1152: std::mem::MaybeUninit<bool> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1152: *mut bool = std::ptr::addr_of_mut!(__slate_storage_1152) as *mut bool;
    // Record the instruction used to terminate the loop.
    let mut __slate_storage_1151: std::mem::MaybeUninit<bool> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1151: *mut bool = std::ptr::addr_of_mut!(__slate_storage_1151) as *mut bool;
    let mut __slate_storage_1150: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1150: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1150) as *mut i32;
    let mut __slate_storage_1149: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1149: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1149) as *mut i32;
    let mut __slate_storage_697: std::mem::MaybeUninit<*mut Index> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_697: *mut *mut Index =
        std::ptr::addr_of_mut!(__slate_storage_697) as *mut *mut Index;
    let mut __slate_storage_1146: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1146: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1146) as *mut i32;
    let mut __slate_storage_1145: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1145: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1145) as *mut i32;
    let mut __slate_storage_1144: std::mem::MaybeUninit<bool> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1144: *mut bool = std::ptr::addr_of_mut!(__slate_storage_1144) as *mut bool;
    let mut __slate_storage_696: std::mem::MaybeUninit<*mut Expr> = std::mem::MaybeUninit::uninit();
    let __slate_slot_696: *mut *mut Expr =
        std::ptr::addr_of_mut!(__slate_storage_696) as *mut *mut Expr;
    let mut __slate_storage_1148: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1148: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1148) as *mut i32;
    let mut __slate_storage_1147: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1147: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1147) as *mut i32;
    let mut __slate_storage_1139: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1139: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1139) as *mut i32;
    let mut __slate_storage_1138: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1138: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1138) as *mut i32;
    let mut __slate_storage_1137: std::mem::MaybeUninit<bool> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1137: *mut bool = std::ptr::addr_of_mut!(__slate_storage_1137) as *mut bool;
    let mut __slate_storage_695: std::mem::MaybeUninit<*mut Expr> = std::mem::MaybeUninit::uninit();
    let __slate_slot_695: *mut *mut Expr =
        std::ptr::addr_of_mut!(__slate_storage_695) as *mut *mut Expr;
    let mut __slate_storage_1141: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1141: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1141) as *mut i32;
    let mut __slate_storage_1140: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1140: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1140) as *mut i32;
    let mut __slate_storage_1143: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1143: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1143) as *mut i32;
    let mut __slate_storage_1142: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1142: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1142) as *mut i32;
    let mut __slate_storage_694: std::mem::MaybeUninit<u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_694: *mut u8 = std::ptr::addr_of_mut!(__slate_storage_694) as *mut u8;
    let mut __slate_storage_693: std::mem::MaybeUninit<u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_693: *mut u8 = std::ptr::addr_of_mut!(__slate_storage_693) as *mut u8;
    let mut __slate_storage_692: std::mem::MaybeUninit<*mut WhereTerm> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_692: *mut *mut WhereTerm =
        std::ptr::addr_of_mut!(__slate_storage_692) as *mut *mut WhereTerm;
    let mut __slate_storage_1136: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1136: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1136) as *mut i32;
    let mut __slate_storage_1135: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1135: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1135) as *mut i32;
    let mut __slate_storage_1134: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1134: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1134) as *mut i32;
    let mut __slate_storage_1133: std::mem::MaybeUninit<*mut Parse> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1133: *mut *mut Parse =
        std::ptr::addr_of_mut!(__slate_storage_1133) as *mut *mut Parse;
    let mut __slate_storage_1132: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1132: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_1132) as *mut u32;
    let mut __slate_storage_1131: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1131: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_1131) as *mut u32;
    let mut __slate_storage_1130: std::mem::MaybeUninit<*mut WhereLevel> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1130: *mut *mut WhereLevel =
        std::ptr::addr_of_mut!(__slate_storage_1130) as *mut *mut WhereLevel;
    let mut __slate_storage_1129: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1129: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_1129) as *mut u32;
    let mut __slate_storage_1128: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1128: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_1128) as *mut u32;
    let mut __slate_storage_1127: std::mem::MaybeUninit<*mut WhereLevel> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1127: *mut *mut WhereLevel =
        std::ptr::addr_of_mut!(__slate_storage_1127) as *mut *mut WhereLevel;
    let mut __slate_storage_1126: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1126: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1126) as *mut i32;
    let mut __slate_storage_1125: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1125: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1125) as *mut i32;
    let mut __slate_storage_1124: std::mem::MaybeUninit<*mut Parse> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1124: *mut *mut Parse =
        std::ptr::addr_of_mut!(__slate_storage_1124) as *mut *mut Parse;
    let mut __slate_storage_1123: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1123: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1123) as *mut i32;
    let mut __slate_storage_1122: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1122: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1122) as *mut i32;
    let mut __slate_storage_1121: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1121: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1121) as *mut i32;
    let mut __slate_storage_1120: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1120: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1120) as *mut i32; // Opcode of the OP_SeekScan, if any
    let mut __slate_storage_691: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_691: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_691) as *mut i32; // big-null flag register
    let mut __slate_storage_690: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_690: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_690) as *mut i32; // True if we use the index only
    let mut __slate_storage_689: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_689: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_689) as *mut i32; // Add condition to terminate at NULLs
    let mut __slate_storage_688: std::mem::MaybeUninit<u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_688: *mut u8 = std::ptr::addr_of_mut!(__slate_storage_688) as *mut u8; // True to seek past initial nulls
    let mut __slate_storage_687: std::mem::MaybeUninit<u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_687: *mut u8 = std::ptr::addr_of_mut!(__slate_storage_687) as *mut u8; // Affinity for end of range constraint
    let mut __slate_storage_686: std::mem::MaybeUninit<*mut i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_686: *mut *mut i8 =
        std::ptr::addr_of_mut!(__slate_storage_686) as *mut *mut i8; // Affinity for start of range constraint
    let mut __slate_storage_685: std::mem::MaybeUninit<*mut i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_685: *mut *mut i8 =
        std::ptr::addr_of_mut!(__slate_storage_685) as *mut *mut i8; // Instruction opcode
    let mut __slate_storage_684: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_684: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_684) as *mut i32; // Number of extra registers needed
    let mut __slate_storage_683: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_683: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_683) as *mut i32; // The VDBE cursor for the index
    let mut __slate_storage_682: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_682: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_682) as *mut i32; // Number of constraint terms
    let mut __slate_storage_681: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_681: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_681) as *mut i32; // Start of range is constrained
    let mut __slate_storage_680: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_680: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_680) as *mut i32; // True if range end uses ==, >= or <=
    let mut __slate_storage_679: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_679: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_679) as *mut i32; // True if range start uses ==, >= or <=
    let mut __slate_storage_678: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_678: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_678) as *mut i32; // Inequality constraint at range end
    let mut __slate_storage_677: std::mem::MaybeUninit<*mut WhereTerm> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_677: *mut *mut WhereTerm =
        std::ptr::addr_of_mut!(__slate_storage_677) as *mut *mut WhereTerm; // Inequality constraint at range start
    let mut __slate_storage_676: std::mem::MaybeUninit<*mut WhereTerm> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_676: *mut *mut WhereTerm =
        std::ptr::addr_of_mut!(__slate_storage_676) as *mut *mut WhereTerm; // Base register holding constraint values
    let mut __slate_storage_675: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_675: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_675) as *mut i32; // Length of TOP vector
    let mut __slate_storage_674: std::mem::MaybeUninit<u16> = std::mem::MaybeUninit::uninit();
    let __slate_slot_674: *mut u16 = std::ptr::addr_of_mut!(__slate_storage_674) as *mut u16; // Length of BTM vector
    let mut __slate_storage_673: std::mem::MaybeUninit<u16> = std::mem::MaybeUninit::uninit();
    let __slate_slot_673: *mut u16 = std::ptr::addr_of_mut!(__slate_storage_673) as *mut u16; // Number of == or IN terms
    let mut __slate_storage_672: std::mem::MaybeUninit<u16> = std::mem::MaybeUninit::uninit();
    let __slate_slot_672: *mut u16 = std::ptr::addr_of_mut!(__slate_storage_672) as *mut u16;
    let mut __slate_storage_1173: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1173: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1173) as *mut i32;
    let mut __slate_storage_1172: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1172: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1172) as *mut i32;
    let mut __slate_storage_1176: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1176: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1176) as *mut i32;
    let mut __slate_storage_1175: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1175: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1175) as *mut i32;
    let mut __slate_storage_728: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_728: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_728) as *mut i32;
    let mut __slate_storage_727: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_727: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_727) as *mut i32;
    let mut __slate_storage_726: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_726: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_726) as *mut i32;
    let mut __slate_storage_725: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_725: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_725) as *mut i32;
    let mut __slate_storage_724: std::mem::MaybeUninit<*mut Index> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_724: *mut *mut Index =
        std::ptr::addr_of_mut!(__slate_storage_724) as *mut *mut Index;
    let mut __slate_storage_723: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_723: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_723) as *mut i32;
    let mut __slate_storage_722: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_722: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_722) as *mut i32;
    let mut __slate_storage_721: std::mem::MaybeUninit<*mut WhereLoop> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_721: *mut *mut WhereLoop =
        std::ptr::addr_of_mut!(__slate_storage_721) as *mut *mut WhereLoop;
    let mut __slate_storage_1174: std::mem::MaybeUninit<*mut Expr> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1174: *mut *mut Expr =
        std::ptr::addr_of_mut!(__slate_storage_1174) as *mut *mut Expr; // Address of jump operation
    let mut __slate_storage_720: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_720: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_720) as *mut i32; // Local copy of OR clause term
    let mut __slate_storage_719: std::mem::MaybeUninit<*mut Expr> = std::mem::MaybeUninit::uninit();
    let __slate_slot_719: *mut *mut Expr =
        std::ptr::addr_of_mut!(__slate_storage_719) as *mut *mut Expr; // Current OR clause term
    let mut __slate_storage_718: std::mem::MaybeUninit<*mut Expr> = std::mem::MaybeUninit::uninit();
    let __slate_slot_718: *mut *mut Expr =
        std::ptr::addr_of_mut!(__slate_storage_718) as *mut *mut Expr; // Info for single OR-term scan
    let mut __slate_storage_717: std::mem::MaybeUninit<*mut WhereInfo> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_717: *mut *mut WhereInfo =
        std::ptr::addr_of_mut!(__slate_storage_717) as *mut *mut WhereInfo;
    let mut __slate_storage_716: std::mem::MaybeUninit<*mut WhereTerm> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_716: *mut *mut WhereTerm =
        std::ptr::addr_of_mut!(__slate_storage_716) as *mut *mut WhereTerm;
    let mut __slate_storage_1171: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1171: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1171) as *mut i32;
    let mut __slate_storage_1170: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1170: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1170) as *mut i32;
    let mut __slate_storage_715: std::mem::MaybeUninit<*mut Expr> = std::mem::MaybeUninit::uninit();
    let __slate_slot_715: *mut *mut Expr =
        std::ptr::addr_of_mut!(__slate_storage_715) as *mut *mut Expr;
    let mut __slate_storage_714: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_714: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_714) as *mut i32;
    let mut __slate_storage_1169: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1169: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1169) as *mut i32;
    let mut __slate_storage_1168: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1168: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1168) as *mut i32;
    let mut __slate_storage_1167: std::mem::MaybeUninit<*mut Parse> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1167: *mut *mut Parse =
        std::ptr::addr_of_mut!(__slate_storage_1167) as *mut *mut Parse;
    let mut __slate_storage_1163: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1163: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1163) as *mut i32;
    let mut __slate_storage_1162: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1162: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1162) as *mut i32;
    let mut __slate_storage_1161: std::mem::MaybeUninit<*mut Parse> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1161: *mut *mut Parse =
        std::ptr::addr_of_mut!(__slate_storage_1161) as *mut *mut Parse;
    let mut __slate_storage_1166: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1166: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1166) as *mut i32;
    let mut __slate_storage_1165: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1165: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1165) as *mut i32;
    let mut __slate_storage_1164: std::mem::MaybeUninit<*mut Parse> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1164: *mut *mut Parse =
        std::ptr::addr_of_mut!(__slate_storage_1164) as *mut *mut Parse;
    let mut __slate_storage_713: std::mem::MaybeUninit<*mut Index> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_713: *mut *mut Index =
        std::ptr::addr_of_mut!(__slate_storage_713) as *mut *mut Index;
    let mut __slate_storage_1160: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1160: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1160) as *mut i32;
    let mut __slate_storage_1159: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1159: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1159) as *mut i32; // Original list of tables
    let mut __slate_storage_712: std::mem::MaybeUninit<*mut SrcItem> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_712: *mut *mut SrcItem =
        std::ptr::addr_of_mut!(__slate_storage_712) as *mut *mut SrcItem; // The number of notReady tables
    let mut __slate_storage_711: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_711: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_711) as *mut i32;
    let mut __slate_storage_710: std::mem::MaybeUninit<*mut Table> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_710: *mut *mut Table =
        std::ptr::addr_of_mut!(__slate_storage_710) as *mut *mut Table; // An ".. AND (...)" expression
    let mut __slate_storage_709: std::mem::MaybeUninit<*mut Expr> = std::mem::MaybeUninit::uninit();
    let __slate_slot_709: *mut *mut Expr =
        std::ptr::addr_of_mut!(__slate_storage_709) as *mut *mut Expr; // Loop counter
    let mut __slate_storage_708: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_708: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_708) as *mut i32; // Some terms not completely tested
    let mut __slate_storage_707: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_707: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_707) as *mut i32; // Address of regReturn init
    let mut __slate_storage_706: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_706: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_706) as *mut i32; // Start of loop body
    let mut __slate_storage_705: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_705: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_705) as *mut i32; // Register holding rowid
    let mut __slate_storage_704: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_704: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_704) as *mut i32; // Register for RowSet object
    let mut __slate_storage_703: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_703: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_703) as *mut i32;
    let mut __slate_storage_1158: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1158: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1158) as *mut i32;
    let mut __slate_storage_1157: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1157: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1157) as *mut i32;
    let mut __slate_storage_1156: std::mem::MaybeUninit<*mut Parse> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1156: *mut *mut Parse =
        std::ptr::addr_of_mut!(__slate_storage_1156) as *mut *mut Parse;
    let mut __slate_storage_702: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_702: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_702) as *mut i32;
    let mut __slate_storage_1155: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1155: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1155) as *mut i32;
    let mut __slate_storage_1154: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1154: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1154) as *mut i32;
    let mut __slate_storage_1153: std::mem::MaybeUninit<*mut Parse> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1153: *mut *mut Parse =
        std::ptr::addr_of_mut!(__slate_storage_1153) as *mut *mut Parse;
    let mut __slate_storage_701: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_701: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_701) as *mut i32; // Potential covering index (or NULL)
    let mut __slate_storage_700: std::mem::MaybeUninit<*mut Index> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_700: *mut *mut Index =
        std::ptr::addr_of_mut!(__slate_storage_700) as *mut *mut Index; // Shortened table list or OR-clause generation
    let mut __slate_storage_699: std::mem::MaybeUninit<*mut SrcList> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_699: *mut *mut SrcList =
        std::ptr::addr_of_mut!(__slate_storage_699) as *mut *mut SrcList;
    // Case 5:  Two or more separately indexed terms connected by OR
    //
    // Example:
    //
    //   CREATE TABLE t1(a,b,c,d);
    //   CREATE INDEX i1 ON t1(a);
    //   CREATE INDEX i2 ON t1(b);
    //   CREATE INDEX i3 ON t1(c);
    //
    //   SELECT * FROM t1 WHERE a=5 OR b=7 OR (c=11 AND d=13)
    //
    // In the example, there are three indexed terms connected by OR.
    // The top of the loop looks like this:
    //
    //          Null       1                # Zero the rowset in reg 1
    //
    // Then, for each indexed term, the following. The arguments to
    // RowSetTest are such that the rowid of the current row is inserted
    // into the RowSet. If it is already present, control skips the
    // Gosub opcode and jumps straight to the code generated by WhereEnd().
    //
    //        sqlite3WhereBegin(<term>)
    //          RowSetTest                  # Insert rowid into rowset
    //          Gosub      2 A
    //        sqlite3WhereEnd()
    //
    // Following the above, code to terminate the loop. Label A, the target
    // of the Gosub above, jumps to the instruction right after the Goto.
    //
    //          Null       1                # Zero the rowset in reg 1
    //          Goto       B                # The loop is finished.
    //
    //       A: <loop body>                 # Return data, whatever.
    //
    //          Return     2                # Jump back to the Gosub
    //
    //       B: <after the loop>
    //
    // Added 2014-05-26: If the table is a WITHOUT ROWID table, then
    // use an ephemeral index instead of a RowSet to record the primary
    // keys of the rows we have already seen.
    // The OR-clause broken out into subterms
    let mut __slate_storage_698: std::mem::MaybeUninit<*mut WhereClause> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_698: *mut *mut WhereClause =
        std::ptr::addr_of_mut!(__slate_storage_698) as *mut *mut WhereClause;
    let mut __slate_storage_1092: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1092: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1092) as *mut i32;
    let mut __slate_storage_1091: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1091: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1091) as *mut i32;
    let mut __slate_storage_1090: std::mem::MaybeUninit<*mut Parse> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1090: *mut *mut Parse =
        std::ptr::addr_of_mut!(__slate_storage_1090) as *mut *mut Parse;
    let mut __slate_storage_1089: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1089: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1089) as *mut i32;
    // Create labels for the "break" and "continue" instructions
    // for the current loop.  Jump to addrBrk to break out of a loop.
    // Jump to cont to go immediately to the next iteration of the
    // loop.
    //
    // When there is an IN operator, we also have a "addrNxt" label that
    // means to continue with the next IN value combination.  When
    // there are no IN operators in the constraints, the "addrNxt" label
    // is the same as "addrBrk".
    let mut __slate_storage_1088: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1088: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1088) as *mut i32; // Iteration of constraint generator loop
    let mut __slate_storage_643: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_643: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_643) as *mut i32; // Index used by loop (if any)
    let mut __slate_storage_642: std::mem::MaybeUninit<*mut Index> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_642: *mut *mut Index =
        std::ptr::addr_of_mut!(__slate_storage_642) as *mut *mut Index; // Temp register to free before returning
    let mut __slate_storage_641: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_641: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_641) as *mut i32; // Rowid is stored in this register, if not zero
    let mut __slate_storage_640: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_640: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_640) as *mut i32; // Jump here to continue with next cycle
    let mut __slate_storage_639: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_639: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_639) as *mut i32; // Jump here to break out of the loop
    let mut __slate_storage_638: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_638: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_638) as *mut i32; // FROM clause term being coded
    let mut __slate_storage_637: std::mem::MaybeUninit<*mut SrcItem> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_637: *mut *mut SrcItem =
        std::ptr::addr_of_mut!(__slate_storage_637) as *mut *mut SrcItem; // Database connection
    let mut __slate_storage_636: std::mem::MaybeUninit<*mut sqlite3> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_636: *mut *mut sqlite3 =
        std::ptr::addr_of_mut!(__slate_storage_636) as *mut *mut sqlite3; // A WHERE clause term
    let mut __slate_storage_635: std::mem::MaybeUninit<*mut WhereTerm> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_635: *mut *mut WhereTerm =
        std::ptr::addr_of_mut!(__slate_storage_635) as *mut *mut WhereTerm; // Decomposition of the entire WHERE clause
    let mut __slate_storage_634: std::mem::MaybeUninit<*mut WhereClause> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_634: *mut *mut WhereClause =
        std::ptr::addr_of_mut!(__slate_storage_634) as *mut *mut WhereClause; // The WhereLoop object being coded
    let mut __slate_storage_633: std::mem::MaybeUninit<*mut WhereLoop> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_633: *mut *mut WhereLoop =
        std::ptr::addr_of_mut!(__slate_storage_633) as *mut *mut WhereLoop; // True if we need to scan in reverse order
    let mut __slate_storage_632: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_632: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_632) as *mut i32; // Where to jump to continue with the next IN case
    let mut __slate_storage_631: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_631: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_631) as *mut i32; // The VDBE cursor for the table
    let mut __slate_storage_630: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_630: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_630) as *mut i32; // Loop counters
    let mut __slate_storage_629: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_629: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_629) as *mut i32;
    let mut __slate_storage_628: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_628: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_628) as *mut i32;
    unsafe {
        '__join_297: {
            std::ptr::write(__slate_slot_640, 0 as i32);
            std::ptr::write(__slate_slot_641, 0 as i32);
            std::ptr::write(__slate_slot_642, std::ptr::null_mut::<Index>());
            *__slate_slot_634 = unsafe { std::ptr::addr_of_mut!((*pWInfo).sWC) };
            *__slate_slot_636 = unsafe { (*pParse).db };
            *__slate_slot_633 = unsafe { (*pLevel).pWLoop };
            *__slate_slot_637 = unsafe {
                unsafe {
                    std::ptr::addr_of_mut!((*unsafe { (*pWInfo).pTabList }).a) as *mut SrcItem
                }
                .offset((((unsafe { (*pLevel).iFrom }) as u32) as i32) as isize)
            };
            *__slate_slot_630 = unsafe { (*(*__slate_slot_637)).iCursor };
            unsafe {
                (*pLevel).notReady = notReady
                    & !unsafe {
                        sqlite3WhereGetMask(
                            unsafe { std::ptr::addr_of_mut!((*pWInfo).sMaskSet) },
                            *__slate_slot_630,
                        )
                    };
            }
            *__slate_slot_632 = (((unsafe { (*pWInfo).revMask }) >> iLevel
                & (((1 as i32) as i64) as u64)) as u32) as i32;
            {}
            std::ptr::write(__slate_slot_1088, unsafe { (*pLevel).addrBrk });
            unsafe {
                (*pLevel).addrNxt = *__slate_slot_1088;
            }
            *__slate_slot_638 = *__slate_slot_1088;
            std::ptr::write(__slate_slot_1089, unsafe { sqlite3VdbeMakeLabel(pParse) });
            unsafe {
                (*pLevel).addrCont = *__slate_slot_1089;
            }
            *__slate_slot_639 = *__slate_slot_1089;
            // If this is the right table of a LEFT OUTER JOIN, allocate and
            // initialize a memory cell that records if this table matches any
            // row of the left table of the join.
            0 as i32;
            if (((unsafe { (*pLevel).iFrom }) as u32) as i32) > (0 as i32)
                && (((unsafe {
                    (*unsafe { (*__slate_slot_637).offset((0 as i32) as isize) })
                        .fg
                        .jointype
                }) as u32) as i32)
                    & (8 as i32)
                    != (0 as i32)
            {
                std::ptr::write(__slate_slot_1090, pParse);
                std::ptr::write(__slate_slot_1091, unsafe { (*(*__slate_slot_1090)).nMem });
                std::ptr::write(__slate_slot_1092, *__slate_slot_1091 + (1 as i32));
                unsafe {
                    (*(*__slate_slot_1090)).nMem = *__slate_slot_1092;
                }
                unsafe {
                    (*pLevel).iLeftJoin = *__slate_slot_1092;
                }
                unsafe {
                    sqlite3VdbeAddOp2(v, 73 as i32, 0 as i32, unsafe { (*pLevel).iLeftJoin })
                };
                unsafe {
                    sqlite3VdbeComment(
                        v,
                        (b"init LEFT JOIN match flag\0".as_ptr() as *mut i8) as *const i8,
                    )
                };
            }
        }
        // Special case of a FROM clause subquery implemented as a co-routine
        if ((unsafe {
            (*(*__slate_slot_637))
                .fg
                .__slate_bits_0
                .__get_viaCoroutine()
        }) as i32)
            != (0 as i32)
        {
            0 as i32;
            *__slate_slot_645 = unsafe { (*(*__slate_slot_637)).u4.pSubq };
            *__slate_slot_644 = unsafe { (*(*__slate_slot_645)).regReturn };
            unsafe {
                sqlite3VdbeAddOp3(v, 11 as i32, *__slate_slot_644, 0 as i32, unsafe {
                    (*(*__slate_slot_645)).addrFillSub
                })
            };
            unsafe {
                (*pLevel).p2 = unsafe {
                    sqlite3VdbeAddOp2(v, 12 as i32, *__slate_slot_644, *__slate_slot_638)
                };
            }
            {}
            unsafe {
                sqlite3VdbeComment(
                    v,
                    (b"next row of %s\0".as_ptr() as *mut i8) as *const i8,
                    unsafe { (*unsafe { (*(*__slate_slot_637)).pSTab }).zName },
                )
            };
            unsafe {
                (*pLevel).op = ((9 as i32) as i8) as u8;
            }
        } else {
            if (unsafe { (*(*__slate_slot_633)).wsFlags }) & ((1024 as i32) as u32)
                != ((0 as i32) as u32)
            {
                std::ptr::write(
                    __slate_slot_648,
                    ((unsafe { (*(*__slate_slot_633)).nLTerm }) as u32) as i32,
                );
                *__slate_slot_646 =
                    unsafe { sqlite3GetTempRange(pParse, *__slate_slot_648 + (2 as i32)) };
                *__slate_slot_647 = unsafe { (*pLevel).addrBrk };
                *__slate_slot_628 = 0 as i32;
                loop {
                    if *__slate_slot_628 < *__slate_slot_648 {
                        '__join_87: {
                            std::ptr::write(
                                __slate_slot_649,
                                *__slate_slot_646 + *__slate_slot_628 + (2 as i32),
                            );
                            *__slate_slot_635 = unsafe {
                                *unsafe {
                                    unsafe { (*(*__slate_slot_633)).aLTerm }
                                        .offset(*__slate_slot_628 as isize)
                                }
                            };
                            if *__slate_slot_635 == std::ptr::null_mut::<WhereTerm>() {
                            } else {
                                if (((unsafe { (*(*__slate_slot_635)).eOperator }) as u32) as i32)
                                    & (1 as i32)
                                    != (0 as i32)
                                {
                                    if (if *__slate_slot_628 <= (31 as i32) {
                                        ((1 as i32) as u32) << *__slate_slot_628
                                    } else {
                                        (0 as i32) as u32
                                    }) & unsafe { (*(*__slate_slot_633)).u.vtab.mHandleIn }
                                        != (0 as u32)
                                    {
                                        std::ptr::write(__slate_slot_1095, pParse);
                                        std::ptr::write(__slate_slot_1096, unsafe {
                                            (*(*__slate_slot_1095)).nTab
                                        });
                                        std::ptr::write(
                                            __slate_slot_1097,
                                            *__slate_slot_1096 + (1 as i32),
                                        );
                                        unsafe {
                                            (*(*__slate_slot_1095)).nTab = *__slate_slot_1097;
                                        }
                                        *__slate_slot_650 = *__slate_slot_1096;
                                        std::ptr::write(__slate_slot_1098, pParse);
                                        std::ptr::write(__slate_slot_1099, unsafe {
                                            (*(*__slate_slot_1098)).nMem
                                        });
                                        std::ptr::write(
                                            __slate_slot_1100,
                                            *__slate_slot_1099 + (1 as i32),
                                        );
                                        unsafe {
                                            (*(*__slate_slot_1098)).nMem = *__slate_slot_1100;
                                        }
                                        *__slate_slot_651 = *__slate_slot_1100;
                                        unsafe {
                                            sqlite3CodeRhsOfIN(
                                                pParse,
                                                unsafe { (*(*__slate_slot_635)).pExpr },
                                                *__slate_slot_650,
                                                0 as i32,
                                            )
                                        };
                                        unsafe {
                                            sqlite3VdbeAddOp3(
                                                v,
                                                177 as i32,
                                                *__slate_slot_650,
                                                *__slate_slot_649,
                                                *__slate_slot_651,
                                            )
                                        };
                                    } else {
                                        codeEqualityTerm(
                                            pParse,
                                            *__slate_slot_635,
                                            pLevel,
                                            *__slate_slot_628,
                                            *__slate_slot_632,
                                            *__slate_slot_649,
                                        );
                                        *__slate_slot_647 = unsafe { (*pLevel).addrNxt };
                                    }
                                } else {
                                    std::ptr::write(__slate_slot_652, unsafe {
                                        (*unsafe { (*(*__slate_slot_635)).pExpr }).pRight
                                    });
                                    codeExprOrVector(
                                        pParse,
                                        *__slate_slot_652,
                                        *__slate_slot_649,
                                        1 as i32,
                                    );
                                    if (((unsafe { (*(*__slate_slot_635)).eMatchOp }) as u32)
                                        as i32)
                                        == (74 as i32)
                                        && ((unsafe {
                                            (*(*__slate_slot_633))
                                                .u
                                                .vtab
                                                .__slate_bits_0
                                                .__get_bOmitOffset()
                                        }) as i32)
                                            != (0 as i32)
                                    {
                                        0 as i32;
                                        0 as i32;
                                        0 as i32;
                                        unsafe {
                                            sqlite3VdbeAddOp2(v, 73 as i32, 0 as i32, unsafe {
                                                (*unsafe { (*pWInfo).pSelect }).iOffset
                                            })
                                        };
                                        unsafe {
                                            sqlite3VdbeComment(
                                                v,
                                                (b"Zero OFFSET counter\0".as_ptr() as *mut i8)
                                                    as *const i8,
                                            )
                                        };
                                    }
                                }
                            }
                        }
                        std::ptr::write(__slate_slot_1093, *__slate_slot_628);
                        std::ptr::write(__slate_slot_1094, *__slate_slot_1093 + (1 as i32));
                        *__slate_slot_628 = *__slate_slot_1094;
                    } else {
                        break;
                    }
                }
                unsafe {
                    sqlite3VdbeAddOp2(
                        v,
                        73 as i32,
                        unsafe { (*(*__slate_slot_633)).u.vtab.idxNum },
                        *__slate_slot_646,
                    )
                };
                unsafe {
                    sqlite3VdbeAddOp2(
                        v,
                        73 as i32,
                        *__slate_slot_648,
                        *__slate_slot_646 + (1 as i32),
                    )
                };
                // The instruction immediately prior to OP_VFilter must be an OP_Integer
                // that sets the "argc" value for xVFilter.  This is necessary for
                // resolveP2() to work correctly.  See tag-20250207a.
                unsafe {
                    sqlite3VdbeAddOp4(
                        v,
                        6 as i32,
                        *__slate_slot_630,
                        *__slate_slot_647,
                        *__slate_slot_646,
                        (unsafe { (*(*__slate_slot_633)).u.vtab.idxStr }) as *const i8,
                        if ((unsafe {
                            (*(*__slate_slot_633))
                                .u
                                .vtab
                                .__slate_bits_0
                                .__get_needFree()
                        }) as i32)
                            != (0 as i32)
                        {
                            -(7 as i32)
                        } else {
                            -(1 as i32)
                        },
                    )
                };
                {}
                unsafe {
                    (*(*__slate_slot_633))
                        .u
                        .vtab
                        .__slate_bits_0
                        .__set_needFree((0 as i32) as u32);
                }
                // An OOM inside of AddOp4(OP_VFilter) instruction above might have freed
                // the u.vtab.idxStr.  NULL it out to prevent a use-after-free
                if (unsafe { (*(*__slate_slot_636)).mallocFailed }) != (0 as u8) {
                    unsafe {
                        (*(*__slate_slot_633)).u.vtab.idxStr = std::ptr::null_mut::<i8>();
                    }
                }
                unsafe {
                    (*pLevel).p1 = *__slate_slot_630;
                }
                unsafe {
                    (*pLevel).op = ((if (unsafe { (*pWInfo).eOnePass }) != (0 as u8) {
                        189 as i32
                    } else {
                        65 as i32
                    }) as i8) as u8;
                }
                unsafe {
                    (*pLevel).p2 = unsafe { sqlite3VdbeCurrentAddr(v) };
                }
                0 as i32;
                *__slate_slot_628 = 0 as i32;
                loop {
                    if *__slate_slot_628 < *__slate_slot_648 {
                        *__slate_slot_635 = unsafe {
                            *unsafe {
                                unsafe { (*(*__slate_slot_633)).aLTerm }
                                    .offset(*__slate_slot_628 as isize)
                            }
                        };
                        if *__slate_slot_628 < (16 as i32)
                            && (((unsafe { (*(*__slate_slot_633)).u.vtab.omitMask }) as u32) as i32)
                                >> *__slate_slot_628
                                & (1 as i32)
                                != (0 as i32)
                        {
                            disableTerm(pLevel, *__slate_slot_635);
                        } else {
                            if (((unsafe { (*(*__slate_slot_635)).eOperator }) as u32) as i32)
                                & (1 as i32)
                                != (0 as i32)
                                && (if *__slate_slot_628 <= (31 as i32) {
                                    ((1 as i32) as u32) << *__slate_slot_628
                                } else {
                                    (0 as i32) as u32
                                }) & unsafe { (*(*__slate_slot_633)).u.vtab.mHandleIn }
                                    == ((0 as i32) as u32)
                                && !((unsafe { (*(*__slate_slot_636)).mallocFailed }) != (0 as u8))
                            {
                                // Reload the constraint value into reg[iReg+j+2].  The same value
                                // was loaded into the same register prior to the OP_VFilter, but
                                // the xFilter implementation might have changed the datatype or
                                // encoding of the value in the register, so it *must* be reloaded.
                                *__slate_slot_656 = 0 as i32;
                                '__join_74: {
                                    loop {
                                        if *__slate_slot_656 < unsafe { (*pLevel).u.r#in.nIn } {
                                            *__slate_slot_655 = unsafe {
                                                sqlite3VdbeGetOp(v, unsafe {
                                                    (*unsafe {
                                                        unsafe { (*pLevel).u.r#in.aInLoop }
                                                            .offset(*__slate_slot_656 as isize)
                                                    })
                                                    .addrInTop
                                                })
                                            };
                                            if (((unsafe { (*(*__slate_slot_655)).opcode }) as u32)
                                                as i32)
                                                == (96 as i32)
                                                && (unsafe { (*(*__slate_slot_655)).p3 })
                                                    == *__slate_slot_646
                                                        + *__slate_slot_628
                                                        + (2 as i32)
                                                || (((unsafe { (*(*__slate_slot_655)).opcode })
                                                    as u32)
                                                    as i32)
                                                    == (137 as i32)
                                                    && (unsafe { (*(*__slate_slot_655)).p2 })
                                                        == *__slate_slot_646
                                                            + *__slate_slot_628
                                                            + (2 as i32)
                                            {
                                                break;
                                            } else {
                                                std::ptr::write(
                                                    __slate_slot_1103,
                                                    *__slate_slot_656,
                                                );
                                                std::ptr::write(
                                                    __slate_slot_1104,
                                                    *__slate_slot_1103 + (1 as i32),
                                                );
                                                *__slate_slot_656 = *__slate_slot_1104;
                                            }
                                        } else {
                                            break '__join_74;
                                        }
                                    }
                                    {}
                                    unsafe {
                                        sqlite3VdbeAddOp3(
                                            v,
                                            ((unsafe { (*(*__slate_slot_655)).opcode }) as u32)
                                                as i32,
                                            unsafe { (*(*__slate_slot_655)).p1 },
                                            unsafe { (*(*__slate_slot_655)).p2 },
                                            unsafe { (*(*__slate_slot_655)).p3 },
                                        )
                                    };
                                }
                                // Generate code that will continue to the next row if
                                // the IN constraint is not satisfied
                                *__slate_slot_653 = unsafe {
                                    sqlite3PExpr(
                                        pParse,
                                        54 as i32,
                                        std::ptr::null_mut::<Expr>(),
                                        std::ptr::null_mut::<Expr>(),
                                    )
                                };
                                if !((unsafe { (*(*__slate_slot_636)).mallocFailed }) != (0 as u8))
                                {
                                    std::ptr::write(__slate_slot_657, unsafe {
                                        (*(*__slate_slot_635)).u.x.iField
                                    });
                                    std::ptr::write(__slate_slot_658, unsafe {
                                        (*unsafe { (*(*__slate_slot_635)).pExpr }).pLeft
                                    });
                                    0 as i32;
                                    if *__slate_slot_657 > (0 as i32) {
                                        0 as i32;
                                        0 as i32;
                                        0 as i32;
                                        unsafe {
                                            (*(*__slate_slot_653)).pLeft = unsafe {
                                                (*unsafe {
                                                    unsafe {
                                                        std::ptr::addr_of_mut!(
                                                            (*unsafe {
                                                                (*(*__slate_slot_658)).x.pList
                                                            })
                                                            .a
                                                        )
                                                            as *mut ExprList_item
                                                    }
                                                    .offset(
                                                        (*__slate_slot_657 - (1 as i32)) as isize,
                                                    )
                                                })
                                                .pExpr
                                            };
                                        }
                                    } else {
                                        unsafe {
                                            (*(*__slate_slot_653)).pLeft = *__slate_slot_658;
                                        }
                                    }
                                    std::ptr::write(__slate_slot_1105, unsafe {
                                        sqlite3Expr(
                                            *__slate_slot_636,
                                            176 as i32,
                                            std::ptr::null::<i8>(),
                                        )
                                    });
                                    *__slate_slot_654 = *__slate_slot_1105;
                                    unsafe {
                                        (*(*__slate_slot_653)).pRight = *__slate_slot_1105;
                                    }
                                    if *__slate_slot_654 != std::ptr::null_mut::<Expr>() {
                                        unsafe {
                                            (*(*__slate_slot_654)).iTable =
                                                *__slate_slot_646 + *__slate_slot_628 + (2 as i32);
                                        }
                                        unsafe {
                                            sqlite3ExprIfFalse(
                                                pParse,
                                                *__slate_slot_653,
                                                unsafe { (*pLevel).addrCont },
                                                16 as i32,
                                            )
                                        };
                                    }
                                    unsafe {
                                        (*(*__slate_slot_653)).pLeft = std::ptr::null_mut::<Expr>();
                                    }
                                }
                                unsafe { sqlite3ExprDelete(*__slate_slot_636, *__slate_slot_653) };
                            }
                        }
                        std::ptr::write(__slate_slot_1101, *__slate_slot_628);
                        std::ptr::write(__slate_slot_1102, *__slate_slot_1101 + (1 as i32));
                        *__slate_slot_628 = *__slate_slot_1102;
                    } else {
                        break;
                    }
                }
            // These registers need to be preserved in case there is an IN operator
            // loop.  So we could deallocate the registers here (and potentially
            // reuse them later) if (pLoop->wsFlags & WHERE_IN_ABLE)==0.  But it seems
            // simpler and safer to simply not reuse the registers.
            //
            //    sqlite3ReleaseTempRange(pParse, iReg, nConstraint+2);
            } else {
                if (unsafe { (*(*__slate_slot_633)).wsFlags }) & ((256 as i32) as u32)
                    != ((0 as i32) as u32)
                    && (unsafe { (*(*__slate_slot_633)).wsFlags })
                        & (((4 as i32) | (1 as i32)) as u32)
                        != ((0 as i32) as u32)
                {
                    // Case 2:  We can directly reference a single row using an
                    // equality comparison against the ROWID field.  Or
                    // we reference multiple rows using a "rowid IN (...)"
                    // construct.
                    0 as i32;
                    *__slate_slot_635 = unsafe {
                        *unsafe {
                            unsafe { (*(*__slate_slot_633)).aLTerm }.offset((0 as i32) as isize)
                        }
                    };
                    0 as i32;
                    0 as i32;
                    {}
                    std::ptr::write(__slate_slot_1106, pParse);
                    std::ptr::write(__slate_slot_1107, unsafe { (*(*__slate_slot_1106)).nMem });
                    std::ptr::write(__slate_slot_1108, *__slate_slot_1107 + (1 as i32));
                    unsafe {
                        (*(*__slate_slot_1106)).nMem = *__slate_slot_1108;
                    }
                    *__slate_slot_641 = *__slate_slot_1108;
                    *__slate_slot_640 = codeEqualityTerm(
                        pParse,
                        *__slate_slot_635,
                        pLevel,
                        0 as i32,
                        *__slate_slot_632,
                        *__slate_slot_641,
                    );
                    if *__slate_slot_640 != *__slate_slot_641 {
                        unsafe { sqlite3ReleaseTempReg(pParse, *__slate_slot_641) };
                    }
                    *__slate_slot_631 = unsafe { (*pLevel).addrNxt };
                    if (unsafe { (*pLevel).regFilter }) != (0 as i32) {
                        unsafe {
                            sqlite3VdbeAddOp2(v, 13 as i32, *__slate_slot_640, *__slate_slot_631)
                        };
                        {}
                        unsafe {
                            sqlite3VdbeAddOp4Int(
                                v,
                                66 as i32,
                                unsafe { (*pLevel).regFilter },
                                *__slate_slot_631,
                                *__slate_slot_640,
                                1 as i32,
                            )
                        };
                        {}
                        filterPullDown(pParse, pWInfo, iLevel, *__slate_slot_631, notReady);
                    }
                    unsafe {
                        sqlite3VdbeAddOp3(
                            v,
                            30 as i32,
                            *__slate_slot_630,
                            *__slate_slot_631,
                            *__slate_slot_640,
                        )
                    };
                    {}
                    unsafe {
                        (*pLevel).op = ((189 as i32) as i8) as u8;
                    }
                } else {
                    if (unsafe { (*(*__slate_slot_633)).wsFlags }) & ((256 as i32) as u32)
                        != ((0 as i32) as u32)
                        && (unsafe { (*(*__slate_slot_633)).wsFlags }) & ((2 as i32) as u32)
                            != ((0 as i32) as u32)
                    {
                        std::ptr::write(__slate_slot_659, 189 as i32);
                        std::ptr::write(__slate_slot_661, 0 as i32);
                        *__slate_slot_628 = 0 as i32;
                        *__slate_slot_663 = std::ptr::null_mut::<WhereTerm>();
                        *__slate_slot_662 = std::ptr::null_mut::<WhereTerm>();
                        if (unsafe { (*(*__slate_slot_633)).wsFlags }) & ((32 as i32) as u32)
                            != (0 as u32)
                        {
                            std::ptr::write(__slate_slot_1109, *__slate_slot_628);
                            std::ptr::write(__slate_slot_1110, *__slate_slot_1109 + (1 as i32));
                            *__slate_slot_628 = *__slate_slot_1110;
                            *__slate_slot_662 = unsafe {
                                *unsafe {
                                    unsafe { (*(*__slate_slot_633)).aLTerm }
                                        .offset(*__slate_slot_1109 as isize)
                                }
                            };
                        }
                        if (unsafe { (*(*__slate_slot_633)).wsFlags }) & ((16 as i32) as u32)
                            != (0 as u32)
                        {
                            std::ptr::write(__slate_slot_1111, *__slate_slot_628);
                            std::ptr::write(__slate_slot_1112, *__slate_slot_1111 + (1 as i32));
                            *__slate_slot_628 = *__slate_slot_1112;
                            *__slate_slot_663 = unsafe {
                                *unsafe {
                                    unsafe { (*(*__slate_slot_633)).aLTerm }
                                        .offset(*__slate_slot_1111 as isize)
                                }
                            };
                        }
                        0 as i32;
                        if *__slate_slot_632 != (0 as i32) {
                            *__slate_slot_635 = *__slate_slot_662;
                            *__slate_slot_662 = *__slate_slot_663;
                            *__slate_slot_663 = *__slate_slot_635;
                        }
                        {}
                        if *__slate_slot_662 != std::ptr::null_mut::<WhereTerm>() {
                            std::ptr::write(
                                __slate_slot_668,
                                [
                                    ((24 as i32) as i8) as u8,
                                    ((22 as i32) as i8) as u8,
                                    ((21 as i32) as i8) as u8,
                                    ((23 as i32) as i8) as u8,
                                ],
                            );
                            0 as i32; // Make sure the ordering..
                            0 as i32; //  ... of the TK_xx values...
                            0 as i32; //  ... is correct.
                            0 as i32;
                            {}
                            *__slate_slot_664 = unsafe { (*(*__slate_slot_662)).pExpr };
                            0 as i32;
                            {}
                            // transitive constraints
                            if (unsafe {
                                sqlite3ExprIsVector(
                                    (unsafe { (*(*__slate_slot_664)).pRight }) as *const Expr,
                                )
                            }) != (0 as i32)
                            {
                                std::ptr::write(__slate_slot_1113, unsafe {
                                    sqlite3GetTempReg(pParse)
                                });
                                *__slate_slot_666 = *__slate_slot_1113;
                                *__slate_slot_665 = *__slate_slot_1113;
                                codeExprOrVector(
                                    pParse,
                                    unsafe { (*(*__slate_slot_664)).pRight },
                                    *__slate_slot_665,
                                    1 as i32,
                                );
                                {}
                                {}
                                {}
                                {}
                                *__slate_slot_667 = ((unsafe {
                                    *unsafe {
                                        ((*__slate_slot_668).as_ptr() as *const u8).offset(
                                            ((((unsafe { (*(*__slate_slot_664)).op }) as u32)
                                                as i32)
                                                - (55 as i32)
                                                - (1 as i32)
                                                & (3 as i32)
                                                | (1 as i32))
                                                as isize,
                                        )
                                    }
                                }) as u32)
                                    as i32;
                                0 as i32;
                                0 as i32;
                                0 as i32;
                                0 as i32;
                            } else {
                                *__slate_slot_665 = unsafe {
                                    sqlite3ExprCodeTemp(
                                        pParse,
                                        unsafe { (*(*__slate_slot_664)).pRight },
                                        std::ptr::addr_of_mut!(*__slate_slot_666),
                                    )
                                };
                                disableTerm(pLevel, *__slate_slot_662);
                                *__slate_slot_667 = ((unsafe {
                                    *unsafe {
                                        ((*__slate_slot_668).as_ptr() as *const u8).offset(
                                            ((((unsafe { (*(*__slate_slot_664)).op }) as u32)
                                                as i32)
                                                - (55 as i32))
                                                as isize,
                                        )
                                    }
                                }) as u32)
                                    as i32;
                            }
                            unsafe {
                                sqlite3VdbeAddOp3(
                                    v,
                                    *__slate_slot_667,
                                    *__slate_slot_630,
                                    *__slate_slot_638,
                                    *__slate_slot_665,
                                )
                            };
                            unsafe {
                                sqlite3VdbeComment(v, (b"pk\0".as_ptr() as *mut i8) as *const i8)
                            };
                            {}
                            {}
                            {}
                            {}
                            unsafe { sqlite3ReleaseTempReg(pParse, *__slate_slot_666) };
                        } else {
                            unsafe {
                                sqlite3VdbeAddOp2(
                                    v,
                                    if *__slate_slot_632 != (0 as i32) {
                                        32 as i32
                                    } else {
                                        36 as i32
                                    },
                                    *__slate_slot_630,
                                    unsafe { (*pLevel).addrHalt },
                                )
                            };
                            {}
                            {}
                        }
                        if *__slate_slot_663 != std::ptr::null_mut::<WhereTerm>() {
                            *__slate_slot_669 = unsafe { (*(*__slate_slot_663)).pExpr };
                            0 as i32;
                            0 as i32;
                            {}
                            // Transitive constraints
                            {}
                            std::ptr::write(__slate_slot_1114, pParse);
                            std::ptr::write(__slate_slot_1115, unsafe {
                                (*(*__slate_slot_1114)).nMem
                            });
                            std::ptr::write(__slate_slot_1116, *__slate_slot_1115 + (1 as i32));
                            unsafe {
                                (*(*__slate_slot_1114)).nMem = *__slate_slot_1116;
                            }
                            *__slate_slot_661 = *__slate_slot_1116;
                            codeExprOrVector(
                                pParse,
                                unsafe { (*(*__slate_slot_669)).pRight },
                                *__slate_slot_661,
                                1 as i32,
                            );
                            if (0 as i32)
                                == unsafe {
                                    sqlite3ExprIsVector(
                                        (unsafe { (*(*__slate_slot_669)).pRight }) as *const Expr,
                                    )
                                }
                                && ((((unsafe { (*(*__slate_slot_669)).op }) as u32) as i32)
                                    == (57 as i32)
                                    || (((unsafe { (*(*__slate_slot_669)).op }) as u32) as i32)
                                        == (55 as i32))
                            {
                                *__slate_slot_659 = if *__slate_slot_632 != (0 as i32) {
                                    56 as i32
                                } else {
                                    58 as i32
                                };
                            } else {
                                *__slate_slot_659 = if *__slate_slot_632 != (0 as i32) {
                                    57 as i32
                                } else {
                                    55 as i32
                                };
                            }
                            if (0 as i32)
                                == unsafe {
                                    sqlite3ExprIsVector(
                                        (unsafe { (*(*__slate_slot_669)).pRight }) as *const Expr,
                                    )
                                }
                            {
                                disableTerm(pLevel, *__slate_slot_663);
                            }
                        }
                        *__slate_slot_660 = unsafe { sqlite3VdbeCurrentAddr(v) };
                        unsafe {
                            (*pLevel).op = ((if *__slate_slot_632 != (0 as i32) {
                                39 as i32
                            } else {
                                40 as i32
                            }) as i8) as u8;
                        }
                        unsafe {
                            (*pLevel).p1 = *__slate_slot_630;
                        }
                        unsafe {
                            (*pLevel).p2 = *__slate_slot_660;
                        }
                        0 as i32;
                        if *__slate_slot_659 != (189 as i32) {
                            std::ptr::write(__slate_slot_1117, pParse);
                            std::ptr::write(__slate_slot_1118, unsafe {
                                (*(*__slate_slot_1117)).nMem
                            });
                            std::ptr::write(__slate_slot_1119, *__slate_slot_1118 + (1 as i32));
                            unsafe {
                                (*(*__slate_slot_1117)).nMem = *__slate_slot_1119;
                            }
                            *__slate_slot_640 = *__slate_slot_1119;
                            unsafe {
                                sqlite3VdbeAddOp2(
                                    v,
                                    137 as i32,
                                    *__slate_slot_630,
                                    *__slate_slot_640,
                                )
                            };
                            unsafe {
                                sqlite3VdbeAddOp3(
                                    v,
                                    *__slate_slot_659,
                                    *__slate_slot_661,
                                    *__slate_slot_638,
                                    *__slate_slot_640,
                                )
                            };
                            {}
                            {}
                            {}
                            {}
                            unsafe {
                                sqlite3VdbeChangeP5(v, (((67 as i32) | (16 as i32)) as i16) as u16)
                            };
                        }
                    } else {
                        if (unsafe { (*(*__slate_slot_633)).wsFlags }) & ((512 as i32) as u32)
                            != (0 as u32)
                        {
                            // Case 4: Search using an index.
                            //
                            // The WHERE clause may contain zero or more equality
                            // terms ("==" or "IN" or "IS" operators) that refer to the N
                            // left-most columns of the index. It may also contain
                            // inequality constraints (>, <, >= or <=) on the indexed
                            // column that immediately follows the N equalities. Only
                            // the right-most column can be an inequality - the rest must
                            // use the "==", "IN", or "IS" operators. For example, if the
                            // index is on (x,y,z), then the following clauses are all
                            // optimized:
                            //
                            //    x=5
                            //    x=5 AND y=10
                            //    x=5 AND y<10
                            //    x=5 AND y>5 AND y<10
                            //    x=5 AND y=5 AND z<=10
                            //
                            // The z<10 term of the following cannot be used, only
                            // the x=5 term:
                            //
                            //    x=5 AND z<10
                            //
                            // N may be zero if there are inequality constraints.
                            // If there are no inequality constraints, then N is at
                            // least one.
                            //
                            // This case is also used when there are no WHERE clause
                            // constraints but an index is selected anyway, in order
                            // to force the output order to conform to an ORDER BY.
                            // 2: (!start_constraints && startEq &&  !bRev)
                            // 3: (!start_constraints && startEq &&   bRev)
                            // 4: (start_constraints  && !startEq && !bRev)
                            // 5: (start_constraints  && !startEq &&  bRev)
                            // 6: (start_constraints  &&  startEq && !bRev)
                            // 7: (start_constraints  &&  startEq &&  bRev)
                            // 0: (end_constraints && !bRev && !endEq)
                            // 1: (end_constraints && !bRev &&  endEq)
                            // 2: (end_constraints &&  bRev && !endEq)
                            // 3: (end_constraints &&  bRev &&  endEq)
                            std::ptr::write(__slate_slot_672, unsafe {
                                (*(*__slate_slot_633)).u.btree.nEq
                            });
                            std::ptr::write(__slate_slot_673, unsafe {
                                (*(*__slate_slot_633)).u.btree.nBtm
                            });
                            std::ptr::write(__slate_slot_674, unsafe {
                                (*(*__slate_slot_633)).u.btree.nTop
                            });
                            std::ptr::write(__slate_slot_676, std::ptr::null_mut::<WhereTerm>());
                            std::ptr::write(__slate_slot_677, std::ptr::null_mut::<WhereTerm>());
                            std::ptr::write(__slate_slot_683, 0 as i32);
                            std::ptr::write(__slate_slot_686, std::ptr::null_mut::<i8>());
                            std::ptr::write(__slate_slot_687, ((0 as i32) as i8) as u8);
                            std::ptr::write(__slate_slot_688, ((0 as i32) as i8) as u8);
                            std::ptr::write(__slate_slot_690, 0 as i32);
                            std::ptr::write(__slate_slot_691, 0 as i32);
                            *__slate_slot_642 = unsafe { (*(*__slate_slot_633)).u.btree.pIndex };
                            *__slate_slot_682 = unsafe { (*pLevel).iIdxCur };
                            0 as i32;
                            // Find any inequality constraint terms for the start and end
                            // of the range.
                            *__slate_slot_628 = (*__slate_slot_672 as u32) as i32;
                            if (unsafe { (*(*__slate_slot_633)).wsFlags }) & ((32 as i32) as u32)
                                != (0 as u32)
                            {
                                std::ptr::write(__slate_slot_1120, *__slate_slot_628);
                                std::ptr::write(__slate_slot_1121, *__slate_slot_1120 + (1 as i32));
                                *__slate_slot_628 = *__slate_slot_1121;
                                *__slate_slot_676 = unsafe {
                                    *unsafe {
                                        unsafe { (*(*__slate_slot_633)).aLTerm }
                                            .offset(*__slate_slot_1120 as isize)
                                    }
                                };
                                *__slate_slot_683 = if *__slate_slot_683
                                    > (((unsafe { (*(*__slate_slot_633)).u.btree.nBtm }) as u32)
                                        as i32)
                                {
                                    *__slate_slot_683
                                } else {
                                    ((unsafe { (*(*__slate_slot_633)).u.btree.nBtm }) as u32) as i32
                                };
                                // Like optimization range constraints always occur in pairs
                                0 as i32;
                            }
                            if (unsafe { (*(*__slate_slot_633)).wsFlags }) & ((16 as i32) as u32)
                                != (0 as u32)
                            {
                                std::ptr::write(__slate_slot_1122, *__slate_slot_628);
                                std::ptr::write(__slate_slot_1123, *__slate_slot_1122 + (1 as i32));
                                *__slate_slot_628 = *__slate_slot_1123;
                                *__slate_slot_677 = unsafe {
                                    *unsafe {
                                        unsafe { (*(*__slate_slot_633)).aLTerm }
                                            .offset(*__slate_slot_1122 as isize)
                                    }
                                };
                                *__slate_slot_683 = if *__slate_slot_683
                                    > (((unsafe { (*(*__slate_slot_633)).u.btree.nTop }) as u32)
                                        as i32)
                                {
                                    *__slate_slot_683
                                } else {
                                    ((unsafe { (*(*__slate_slot_633)).u.btree.nTop }) as u32) as i32
                                };
                                if (((unsafe { (*(*__slate_slot_677)).wtFlags }) as u32) as i32)
                                    & (256 as i32)
                                    != (0 as i32)
                                {
                                    0 as i32; // LIKE opt constraints
                                    0 as i32; // occur in pairs
                                    std::ptr::write(__slate_slot_1124, pParse);
                                    std::ptr::write(__slate_slot_1125, unsafe {
                                        (*(*__slate_slot_1124)).nMem
                                    });
                                    std::ptr::write(
                                        __slate_slot_1126,
                                        *__slate_slot_1125 + (1 as i32),
                                    );
                                    unsafe {
                                        (*(*__slate_slot_1124)).nMem = *__slate_slot_1126;
                                    }
                                    unsafe {
                                        (*pLevel).iLikeRepCntr = *__slate_slot_1126 as u32;
                                    }
                                    unsafe {
                                        sqlite3VdbeAddOp2(
                                            v,
                                            73 as i32,
                                            1 as i32,
                                            (unsafe { (*pLevel).iLikeRepCntr }) as i32,
                                        )
                                    };
                                    unsafe {
                                        sqlite3VdbeComment(
                                            v,
                                            (b"LIKE loop counter\0".as_ptr() as *mut i8)
                                                as *const i8,
                                        )
                                    };
                                    unsafe {
                                        (*pLevel).addrLikeRep =
                                            unsafe { sqlite3VdbeCurrentAddr(v) };
                                    }
                                    // iLikeRepCntr actually stores 2x the counter register number.  The
                                    // bottom bit indicates whether the search order is ASC or DESC.
                                    {}
                                    {}
                                    0 as i32;
                                    std::ptr::write(__slate_slot_1127, pLevel);
                                    std::ptr::write(__slate_slot_1128, unsafe {
                                        (*(*__slate_slot_1127)).iLikeRepCntr
                                    });
                                    std::ptr::write(
                                        __slate_slot_1129,
                                        *__slate_slot_1128 << (1 as i32),
                                    );
                                    unsafe {
                                        (*(*__slate_slot_1127)).iLikeRepCntr = *__slate_slot_1129;
                                    }
                                    std::ptr::write(__slate_slot_1130, pLevel);
                                    std::ptr::write(__slate_slot_1131, unsafe {
                                        (*(*__slate_slot_1130)).iLikeRepCntr
                                    });
                                    std::ptr::write(
                                        __slate_slot_1132,
                                        *__slate_slot_1131
                                            | ((*__slate_slot_632
                                                ^ (((((unsafe {
                                                    *unsafe {
                                                        unsafe { (*(*__slate_slot_642)).aSortOrder }
                                                            .offset(
                                                                ((*__slate_slot_672 as u32) as i32)
                                                                    as isize,
                                                            )
                                                    }
                                                })
                                                    as u32)
                                                    as i32)
                                                    == (1 as i32))
                                                    as i32))
                                                as u32),
                                    );
                                    unsafe {
                                        (*(*__slate_slot_1130)).iLikeRepCntr = *__slate_slot_1132;
                                    }
                                }
                                if *__slate_slot_676 == std::ptr::null_mut::<WhereTerm>() {
                                    *__slate_slot_628 = (unsafe {
                                        *unsafe {
                                            unsafe { (*(*__slate_slot_642)).aiColumn }.offset(
                                                ((*__slate_slot_672 as u32) as i32) as isize,
                                            )
                                        }
                                    })
                                        as i32;
                                    if *__slate_slot_628 >= (0 as i32)
                                        && ((unsafe {
                                            (*unsafe {
                                                unsafe {
                                                    (*unsafe { (*(*__slate_slot_642)).pTable }).aCol
                                                }
                                                .offset(*__slate_slot_628 as isize)
                                            })
                                            .__slate_bits_0
                                            .__get_notNull()
                                        }) as i32)
                                            == (0 as i32)
                                        || *__slate_slot_628 == -(2 as i32)
                                    {
                                        *__slate_slot_687 = ((1 as i32) as i8) as u8;
                                    }
                                }
                            }
                            0 as i32;
                            // If the WHERE_BIGNULL_SORT flag is set, then index column nEq uses
                            // a non-default "big-null" sort (either ASC NULLS LAST or DESC NULLS
                            // FIRST). In both cases separate ordered scans are made of those
                            // index entries for which the column is null and for those for which
                            // it is not. For an ASC sort, the non-NULL entries are scanned first.
                            // For DESC, NULL entries are scanned first.
                            if (unsafe { (*(*__slate_slot_633)).wsFlags })
                                & (((16 as i32) | (32 as i32)) as u32)
                                == ((0 as i32) as u32)
                                && (unsafe { (*(*__slate_slot_633)).wsFlags })
                                    & ((524288 as i32) as u32)
                                    != ((0 as i32) as u32)
                            {
                                0 as i32;
                                0 as i32;
                                {}
                                *__slate_slot_683 = 1 as i32;
                                *__slate_slot_687 = ((1 as i32) as i8) as u8;
                                std::ptr::write(__slate_slot_1133, pParse);
                                std::ptr::write(__slate_slot_1134, unsafe {
                                    (*(*__slate_slot_1133)).nMem
                                });
                                std::ptr::write(__slate_slot_1135, *__slate_slot_1134 + (1 as i32));
                                unsafe {
                                    (*(*__slate_slot_1133)).nMem = *__slate_slot_1135;
                                }
                                std::ptr::write(__slate_slot_1136, *__slate_slot_1135);
                                *__slate_slot_690 = *__slate_slot_1136;
                                unsafe {
                                    (*pLevel).regBignull = *__slate_slot_1136;
                                }
                                if (unsafe { (*pLevel).iLeftJoin }) != (0 as i32) {
                                    unsafe {
                                        sqlite3VdbeAddOp2(v, 73 as i32, 0 as i32, *__slate_slot_690)
                                    };
                                }
                                unsafe {
                                    (*pLevel).addrBignull = unsafe { sqlite3VdbeMakeLabel(pParse) };
                                }
                            }
                            // If we are doing a reverse order scan on an ascending index, or
                            // a forward order scan on a descending index, interchange the
                            // start and end terms (pRangeStart and pRangeEnd).
                            if ((*__slate_slot_672 as u32) as i32)
                                < (((unsafe { (*(*__slate_slot_642)).nColumn }) as u32) as i32)
                                && *__slate_slot_632
                                    == (((((unsafe {
                                        *unsafe {
                                            unsafe { (*(*__slate_slot_642)).aSortOrder }.offset(
                                                ((*__slate_slot_672 as u32) as i32) as isize,
                                            )
                                        }
                                    }) as u32) as i32)
                                        == (0 as i32))
                                        as i32)
                            {
                                std::ptr::write(__slate_slot_692, *__slate_slot_677);
                                *__slate_slot_677 = *__slate_slot_676;
                                *__slate_slot_676 = *__slate_slot_692;
                                {}
                                std::ptr::write(__slate_slot_693, *__slate_slot_687);
                                *__slate_slot_687 = *__slate_slot_688;
                                *__slate_slot_688 = *__slate_slot_693;
                                {}
                                std::ptr::write(__slate_slot_694, *__slate_slot_673 as u8);
                                *__slate_slot_673 = *__slate_slot_674;
                                *__slate_slot_674 = *__slate_slot_694 as u16;
                                {}
                            }
                            if iLevel > (0 as i32)
                                && (unsafe { (*(*__slate_slot_633)).wsFlags })
                                    & ((1048576 as i32) as u32)
                                    != ((0 as i32) as u32)
                            {
                                // In case OP_SeekScan is used, ensure that the index cursor does not
                                // point to a valid row for the first iteration of this loop.
                                unsafe { sqlite3VdbeAddOp1(v, 138 as i32, *__slate_slot_682) };
                            }
                            // Generate code to evaluate all constraint terms using == or IN
                            // and store the values of those terms in an array of registers
                            // starting at regBase.
                            {}
                            *__slate_slot_675 = codeAllEqualityTerms(
                                pParse,
                                pLevel,
                                *__slate_slot_632,
                                *__slate_slot_683,
                                std::ptr::addr_of_mut!(*__slate_slot_685),
                            );
                            0 as i32;
                            if *__slate_slot_685 != std::ptr::null_mut::<i8>()
                                && *__slate_slot_674 != (0 as u16)
                            {
                                *__slate_slot_686 = unsafe {
                                    sqlite3DbStrDup(
                                        *__slate_slot_636,
                                        (unsafe {
                                            (*__slate_slot_685).offset(
                                                ((*__slate_slot_672 as u32) as i32) as isize,
                                            )
                                        }) as *const i8,
                                    )
                                };
                            }
                            *__slate_slot_631 = if *__slate_slot_690 != (0 as i32) {
                                unsafe { (*pLevel).addrBignull }
                            } else {
                                unsafe { (*pLevel).addrNxt }
                            };
                            {}
                            {}
                            {}
                            {}
                            *__slate_slot_678 = (!(*__slate_slot_676
                                != std::ptr::null_mut::<WhereTerm>())
                                || (((unsafe { (*(*__slate_slot_676)).eOperator }) as u32) as i32)
                                    & ((2 as i32) << (56 as i32) - (54 as i32)
                                        | (2 as i32) << (58 as i32) - (54 as i32))
                                    != (0 as i32))
                                as i32;
                            *__slate_slot_679 = (!(*__slate_slot_677
                                != std::ptr::null_mut::<WhereTerm>())
                                || (((unsafe { (*(*__slate_slot_677)).eOperator }) as u32) as i32)
                                    & ((2 as i32) << (56 as i32) - (54 as i32)
                                        | (2 as i32) << (58 as i32) - (54 as i32))
                                    != (0 as i32))
                                as i32;
                            *__slate_slot_680 = (*__slate_slot_676
                                != std::ptr::null_mut::<WhereTerm>()
                                || ((*__slate_slot_672 as u32) as i32) > (0 as i32))
                                as i32;
                            // Seek the index cursor to the start of the range.
                            *__slate_slot_681 = (*__slate_slot_672 as u32) as i32;
                            if *__slate_slot_676 != std::ptr::null_mut::<WhereTerm>() {
                                std::ptr::write(__slate_slot_695, unsafe {
                                    (*unsafe { (*(*__slate_slot_676)).pExpr }).pRight
                                });
                                codeExprOrVector(
                                    pParse,
                                    *__slate_slot_695,
                                    *__slate_slot_675 + ((*__slate_slot_672 as u32) as i32),
                                    (*__slate_slot_673 as u32) as i32,
                                );
                                whereLikeOptimizationStringFixup(v, pLevel, *__slate_slot_676);
                                if (((unsafe { (*(*__slate_slot_676)).wtFlags }) as u32) as i32)
                                    & (128 as i32)
                                    == (0 as i32)
                                {
                                    *__slate_slot_1137 = (unsafe {
                                        sqlite3ExprCanBeNull(*__slate_slot_695 as *const Expr)
                                    }) != (0 as i32);
                                } else {
                                    *__slate_slot_1137 = false as bool;
                                }
                                if *__slate_slot_1137 {
                                    unsafe {
                                        sqlite3VdbeAddOp2(
                                            v,
                                            51 as i32,
                                            *__slate_slot_675 + ((*__slate_slot_672 as u32) as i32),
                                            *__slate_slot_631,
                                        )
                                    };
                                    {}
                                }
                                if *__slate_slot_685 != std::ptr::null_mut::<i8>() {
                                    updateRangeAffinityStr(
                                        *__slate_slot_695,
                                        (*__slate_slot_673 as u32) as i32,
                                        unsafe {
                                            (*__slate_slot_685).offset(
                                                ((*__slate_slot_672 as u32) as i32) as isize,
                                            )
                                        },
                                    );
                                }
                                std::ptr::write(__slate_slot_1138, *__slate_slot_681);
                                std::ptr::write(
                                    __slate_slot_1139,
                                    *__slate_slot_1138 + ((*__slate_slot_673 as u32) as i32),
                                );
                                *__slate_slot_681 = *__slate_slot_1139;
                                {}
                                if (unsafe {
                                    sqlite3ExprIsVector(*__slate_slot_695 as *const Expr)
                                }) == (0 as i32)
                                {
                                    disableTerm(pLevel, *__slate_slot_676);
                                } else {
                                    *__slate_slot_678 = 1 as i32;
                                }
                                *__slate_slot_687 = ((0 as i32) as i8) as u8;
                            } else {
                                if *__slate_slot_687 != (0 as u8) {
                                    *__slate_slot_678 = 0 as i32;
                                    unsafe {
                                        sqlite3VdbeAddOp2(
                                            v,
                                            77 as i32,
                                            0 as i32,
                                            *__slate_slot_675 + ((*__slate_slot_672 as u32) as i32),
                                        )
                                    };
                                    *__slate_slot_680 = 1 as i32;
                                    std::ptr::write(__slate_slot_1140, *__slate_slot_681);
                                    std::ptr::write(
                                        __slate_slot_1141,
                                        *__slate_slot_1140 + (1 as i32),
                                    );
                                    *__slate_slot_681 = *__slate_slot_1141;
                                } else {
                                    if *__slate_slot_690 != (0 as i32) {
                                        unsafe {
                                            sqlite3VdbeAddOp2(
                                                v,
                                                77 as i32,
                                                0 as i32,
                                                *__slate_slot_675
                                                    + ((*__slate_slot_672 as u32) as i32),
                                            )
                                        };
                                        *__slate_slot_680 = 1 as i32;
                                        std::ptr::write(__slate_slot_1142, *__slate_slot_681);
                                        std::ptr::write(
                                            __slate_slot_1143,
                                            *__slate_slot_1142 + (1 as i32),
                                        );
                                        *__slate_slot_681 = *__slate_slot_1143;
                                    }
                                }
                            }
                            codeApplyAffinity(
                                pParse,
                                *__slate_slot_675,
                                *__slate_slot_681 - ((*__slate_slot_687 as u32) as i32),
                                *__slate_slot_685,
                            );
                            if (((unsafe { (*(*__slate_slot_633)).nSkip }) as u32) as i32)
                                > (0 as i32)
                                && *__slate_slot_681
                                    == (((unsafe { (*(*__slate_slot_633)).nSkip }) as u32) as i32)
                            {
                                // The skip-scan logic inside the call to codeAllEqualityConstraints()
                                // above has already left the cursor sitting on the correct row,
                                // so no further seeking is needed
                            } else {
                                if *__slate_slot_690 != (0 as i32) {
                                    unsafe {
                                        sqlite3VdbeAddOp2(v, 73 as i32, 1 as i32, *__slate_slot_690)
                                    };
                                    unsafe {
                                        sqlite3VdbeComment(
                                            v,
                                            (b"NULL-scan pass ctr\0".as_ptr() as *mut i8)
                                                as *const i8,
                                        )
                                    };
                                }
                                if (unsafe { (*pLevel).regFilter }) != (0 as i32) {
                                    0 as i32;
                                    unsafe {
                                        sqlite3VdbeAddOp4Int(
                                            v,
                                            66 as i32,
                                            unsafe { (*pLevel).regFilter },
                                            *__slate_slot_631,
                                            *__slate_slot_675,
                                            (*__slate_slot_672 as u32) as i32,
                                        )
                                    };
                                    {}
                                    filterPullDown(
                                        pParse,
                                        pWInfo,
                                        iLevel,
                                        *__slate_slot_631,
                                        notReady,
                                    );
                                }
                                *__slate_slot_684 = ((unsafe {
                                    *unsafe {
                                        unsafe { std::ptr::addr_of!(aStartOp) as *const u8 }.offset(
                                            ((*__slate_slot_680 << (2 as i32))
                                                + (*__slate_slot_678 << (1 as i32))
                                                + *__slate_slot_632)
                                                as isize,
                                        )
                                    }
                                }) as u32)
                                    as i32;
                                0 as i32;
                                if (unsafe { (*(*__slate_slot_633)).wsFlags })
                                    & ((1048576 as i32) as u32)
                                    != ((0 as i32) as u32)
                                    && *__slate_slot_684 == (23 as i32)
                                {
                                    0 as i32;
                                    // TUNING:  The OP_SeekScan opcode seeks to reduce the number
                                    // of expensive seek operations by replacing a single seek with
                                    // 1 or more step operations.  The question is, how many steps
                                    // should we try before giving up and going with a seek.  The cost
                                    // of a seek is proportional to the logarithm of the of the number
                                    // of entries in the tree, so basing the number of steps to try
                                    // on the estimated number of rows in the btree seems like a good
                                    // guess.
                                    *__slate_slot_691 = unsafe {
                                        sqlite3VdbeAddOp1(
                                            v,
                                            126 as i32,
                                            (((unsafe {
                                                *unsafe {
                                                    unsafe { (*(*__slate_slot_642)).aiRowLogEst }
                                                        .offset((0 as i32) as isize)
                                                }
                                            })
                                                as i32)
                                                + (9 as i32))
                                                / (10 as i32),
                                        )
                                    };
                                    if *__slate_slot_676 != std::ptr::null_mut::<WhereTerm>()
                                        || *__slate_slot_677 != std::ptr::null_mut::<WhereTerm>()
                                    {
                                        unsafe {
                                            sqlite3VdbeChangeP5(v, ((1 as i32) as i16) as u16)
                                        };
                                        unsafe {
                                            sqlite3VdbeChangeP2(
                                                v,
                                                *__slate_slot_691,
                                                (unsafe { sqlite3VdbeCurrentAddr(v) }) + (1 as i32),
                                            )
                                        };
                                        *__slate_slot_691 = 0 as i32;
                                    }
                                    {}
                                }
                                unsafe {
                                    sqlite3VdbeAddOp4Int(
                                        v,
                                        *__slate_slot_684,
                                        *__slate_slot_682,
                                        *__slate_slot_631,
                                        *__slate_slot_675,
                                        *__slate_slot_681,
                                    )
                                };
                                {}
                                {}
                                {}
                                {}
                                {}
                                {}
                                {}
                                {}
                                {}
                                {}
                                {}
                                {}
                                {}
                                0 as i32;
                                if *__slate_slot_690 != (0 as i32) {
                                    0 as i32;
                                    0 as i32;
                                    0 as i32;
                                    unsafe {
                                        sqlite3VdbeAddOp2(
                                            v,
                                            9 as i32,
                                            0 as i32,
                                            (unsafe { sqlite3VdbeCurrentAddr(v) }) + (2 as i32),
                                        )
                                    };
                                    *__slate_slot_684 =
                                        ((unsafe {
                                            *unsafe {
                                                unsafe { std::ptr::addr_of!(aStartOp) as *const u8 }
                                                    .offset(
                                                        (((*__slate_slot_681 > (1 as i32)) as i32)
                                                            * (4 as i32)
                                                            + (2 as i32)
                                                            + *__slate_slot_632)
                                                            as isize,
                                                    )
                                            }
                                        }) as u32) as i32;
                                    unsafe {
                                        sqlite3VdbeAddOp4Int(
                                            v,
                                            *__slate_slot_684,
                                            *__slate_slot_682,
                                            *__slate_slot_631,
                                            *__slate_slot_675,
                                            *__slate_slot_681 - *__slate_slot_678,
                                        )
                                    };
                                    {}
                                    {}
                                    {}
                                    {}
                                    {}
                                    {}
                                    {}
                                    {}
                                    {}
                                    0 as i32;
                                }
                            }
                            // Load the value for the inequality constraint at the end of the
                            // range (if any).
                            *__slate_slot_681 = (*__slate_slot_672 as u32) as i32;
                            0 as i32;
                            if *__slate_slot_677 != std::ptr::null_mut::<WhereTerm>() {
                                std::ptr::write(__slate_slot_696, unsafe {
                                    (*unsafe { (*(*__slate_slot_677)).pExpr }).pRight
                                });
                                0 as i32;
                                codeExprOrVector(
                                    pParse,
                                    *__slate_slot_696,
                                    *__slate_slot_675 + ((*__slate_slot_672 as u32) as i32),
                                    (*__slate_slot_674 as u32) as i32,
                                );
                                whereLikeOptimizationStringFixup(v, pLevel, *__slate_slot_677);
                                if (((unsafe { (*(*__slate_slot_677)).wtFlags }) as u32) as i32)
                                    & (128 as i32)
                                    == (0 as i32)
                                {
                                    *__slate_slot_1144 = (unsafe {
                                        sqlite3ExprCanBeNull(*__slate_slot_696 as *const Expr)
                                    }) != (0 as i32);
                                } else {
                                    *__slate_slot_1144 = false as bool;
                                }
                                if *__slate_slot_1144 {
                                    unsafe {
                                        sqlite3VdbeAddOp2(
                                            v,
                                            51 as i32,
                                            *__slate_slot_675 + ((*__slate_slot_672 as u32) as i32),
                                            *__slate_slot_631,
                                        )
                                    };
                                    {}
                                }
                                if *__slate_slot_686 != std::ptr::null_mut::<i8>() {
                                    updateRangeAffinityStr(
                                        *__slate_slot_696,
                                        (*__slate_slot_674 as u32) as i32,
                                        *__slate_slot_686,
                                    );
                                    codeApplyAffinity(
                                        pParse,
                                        *__slate_slot_675 + ((*__slate_slot_672 as u32) as i32),
                                        (*__slate_slot_674 as u32) as i32,
                                        *__slate_slot_686,
                                    );
                                } else {
                                    0 as i32;
                                }
                                std::ptr::write(__slate_slot_1145, *__slate_slot_681);
                                std::ptr::write(
                                    __slate_slot_1146,
                                    *__slate_slot_1145 + ((*__slate_slot_674 as u32) as i32),
                                );
                                *__slate_slot_681 = *__slate_slot_1146;
                                {}
                                if (unsafe {
                                    sqlite3ExprIsVector(*__slate_slot_696 as *const Expr)
                                }) == (0 as i32)
                                {
                                    disableTerm(pLevel, *__slate_slot_677);
                                } else {
                                    *__slate_slot_679 = 1 as i32;
                                }
                            } else {
                                if *__slate_slot_688 != (0 as u8) {
                                    if *__slate_slot_690 == (0 as i32) {
                                        unsafe {
                                            sqlite3VdbeAddOp2(
                                                v,
                                                77 as i32,
                                                0 as i32,
                                                *__slate_slot_675
                                                    + ((*__slate_slot_672 as u32) as i32),
                                            )
                                        };
                                        *__slate_slot_679 = 0 as i32;
                                    }
                                    std::ptr::write(__slate_slot_1147, *__slate_slot_681);
                                    std::ptr::write(
                                        __slate_slot_1148,
                                        *__slate_slot_1147 + (1 as i32),
                                    );
                                    *__slate_slot_681 = *__slate_slot_1148;
                                }
                            }
                            if *__slate_slot_685 != std::ptr::null_mut::<i8>() {
                                unsafe {
                                    sqlite3DbNNFreeNN(
                                        *__slate_slot_636,
                                        *__slate_slot_685 as *mut (),
                                    )
                                };
                            }
                            if *__slate_slot_686 != std::ptr::null_mut::<i8>() {
                                unsafe {
                                    sqlite3DbNNFreeNN(
                                        *__slate_slot_636,
                                        *__slate_slot_686 as *mut (),
                                    )
                                };
                            }
                            // Top of the loop body
                            unsafe {
                                (*pLevel).p2 = unsafe { sqlite3VdbeCurrentAddr(v) };
                            }
                            // Check if the index cursor is past the end of the range.
                            if *__slate_slot_681 != (0 as i32) {
                                if *__slate_slot_690 != (0 as i32) {
                                    // Except, skip the end-of-range check while doing the NULL-scan
                                    unsafe {
                                        sqlite3VdbeAddOp2(
                                            v,
                                            17 as i32,
                                            *__slate_slot_690,
                                            (unsafe { sqlite3VdbeCurrentAddr(v) }) + (3 as i32),
                                        )
                                    };
                                    unsafe {
                                        sqlite3VdbeComment(
                                            v,
                                            (b"If NULL-scan 2nd pass\0".as_ptr() as *mut i8)
                                                as *const i8,
                                        )
                                    };
                                    {}
                                }
                                *__slate_slot_684 = ((unsafe {
                                    *unsafe {
                                        unsafe { std::ptr::addr_of!(aEndOp) as *const u8 }.offset(
                                            (*__slate_slot_632 * (2 as i32) + *__slate_slot_679)
                                                as isize,
                                        )
                                    }
                                }) as u32)
                                    as i32;
                                unsafe {
                                    sqlite3VdbeAddOp4Int(
                                        v,
                                        *__slate_slot_684,
                                        *__slate_slot_682,
                                        *__slate_slot_631,
                                        *__slate_slot_675,
                                        *__slate_slot_681,
                                    )
                                };
                                {}
                                {}
                                {}
                                {}
                                {}
                                {}
                                {}
                                {}
                                if *__slate_slot_691 != (0 as i32) {
                                    unsafe { sqlite3VdbeJumpHere(v, *__slate_slot_691) };
                                }
                            }
                            if *__slate_slot_690 != (0 as i32) {
                                // During a NULL-scan, check to see if we have reached the end of
                                // the NULLs
                                0 as i32;
                                0 as i32;
                                0 as i32;
                                unsafe {
                                    sqlite3VdbeAddOp2(
                                        v,
                                        16 as i32,
                                        *__slate_slot_690,
                                        (unsafe { sqlite3VdbeCurrentAddr(v) }) + (2 as i32),
                                    )
                                };
                                unsafe {
                                    sqlite3VdbeComment(
                                        v,
                                        (b"If NULL-scan 1st pass\0".as_ptr() as *mut i8)
                                            as *const i8,
                                    )
                                };
                                {}
                                *__slate_slot_684 = ((unsafe {
                                    *unsafe {
                                        unsafe { std::ptr::addr_of!(aEndOp) as *const u8 }.offset(
                                            (*__slate_slot_632 * (2 as i32)
                                                + ((*__slate_slot_687 as u32) as i32))
                                                as isize,
                                        )
                                    }
                                }) as u32)
                                    as i32;
                                unsafe {
                                    sqlite3VdbeAddOp4Int(
                                        v,
                                        *__slate_slot_684,
                                        *__slate_slot_682,
                                        *__slate_slot_631,
                                        *__slate_slot_675,
                                        *__slate_slot_681 + ((*__slate_slot_687 as u32) as i32),
                                    )
                                };
                                {}
                                {}
                                {}
                                {}
                                {}
                                {}
                                {}
                                {}
                            }
                            if (unsafe { (*(*__slate_slot_633)).wsFlags })
                                & ((262144 as i32) as u32)
                                != ((0 as i32) as u32)
                            {
                                unsafe {
                                    sqlite3VdbeAddOp3(
                                        v,
                                        127 as i32,
                                        *__slate_slot_682,
                                        (*__slate_slot_672 as u32) as i32,
                                        (*__slate_slot_672 as u32) as i32,
                                    )
                                };
                            }
                            // Seek the table cursor, if required
                            *__slate_slot_689 =
                                ((unsafe { (*(*__slate_slot_633)).wsFlags }) & ((64 as i32) as u32)
                                    != ((0 as i32) as u32)
                                    && (((unsafe { (*pWInfo).wctrlFlags }) as u32) as i32)
                                        & ((32 as i32) | (4096 as i32))
                                        == (0 as i32)) as i32;
                            if *__slate_slot_689 != (0 as i32) {
                                // pIdx is a covering index.  No need to access the main table.
                            } else {
                                if (unsafe { (*unsafe { (*(*__slate_slot_642)).pTable }).tabFlags })
                                    & ((128 as i32) as u32)
                                    == ((0 as i32) as u32)
                                {
                                    codeDeferredSeek(
                                        pWInfo,
                                        *__slate_slot_642,
                                        *__slate_slot_630,
                                        *__slate_slot_682,
                                    );
                                } else {
                                    if *__slate_slot_630 != *__slate_slot_682 {
                                        std::ptr::write(__slate_slot_697, unsafe {
                                            sqlite3PrimaryKeyIndex(unsafe {
                                                (*(*__slate_slot_642)).pTable
                                            })
                                        });
                                        *__slate_slot_640 = unsafe {
                                            sqlite3GetTempRange(
                                                pParse,
                                                ((unsafe { (*(*__slate_slot_697)).nKeyCol }) as u32)
                                                    as i32,
                                            )
                                        };
                                        *__slate_slot_628 = 0 as i32;
                                        loop {
                                            if *__slate_slot_628
                                                < (((unsafe { (*(*__slate_slot_697)).nKeyCol })
                                                    as u32)
                                                    as i32)
                                            {
                                                *__slate_slot_629 = unsafe {
                                                    sqlite3TableColumnToIndex(
                                                        *__slate_slot_642,
                                                        (unsafe {
                                                            *unsafe {
                                                                unsafe {
                                                                    (*(*__slate_slot_697)).aiColumn
                                                                }
                                                                .offset(*__slate_slot_628 as isize)
                                                            }
                                                        })
                                                            as i32,
                                                    )
                                                };
                                                unsafe {
                                                    sqlite3VdbeAddOp3(
                                                        v,
                                                        96 as i32,
                                                        *__slate_slot_682,
                                                        *__slate_slot_629,
                                                        *__slate_slot_640 + *__slate_slot_628,
                                                    )
                                                };
                                                std::ptr::write(
                                                    __slate_slot_1149,
                                                    *__slate_slot_628,
                                                );
                                                std::ptr::write(
                                                    __slate_slot_1150,
                                                    *__slate_slot_1149 + (1 as i32),
                                                );
                                                *__slate_slot_628 = *__slate_slot_1150;
                                            } else {
                                                break;
                                            }
                                        }
                                        unsafe {
                                            sqlite3VdbeAddOp4Int(
                                                v,
                                                28 as i32,
                                                *__slate_slot_630,
                                                *__slate_slot_639,
                                                *__slate_slot_640,
                                                ((unsafe { (*(*__slate_slot_697)).nKeyCol }) as u32)
                                                    as i32,
                                            )
                                        };
                                        {}
                                    }
                                }
                            }
                            if (unsafe { (*pLevel).iLeftJoin }) == (0 as i32) {
                                // If a partial index is driving the loop, try to eliminate WHERE clause
                                // terms from the query that must be true due to the WHERE clause of
                                // the partial index.  This optimization does not work on an outer join,
                                // as shown by:
                                //
                                // 2019-11-02 ticket 623eff57e76d45f6      (LEFT JOIN)
                                // 2025-05-29 forum post 7dee41d32506c4ae  (RIGHT JOIN)
                                if (unsafe { (*(*__slate_slot_642)).pPartIdxWhere })
                                    != std::ptr::null_mut::<Expr>()
                                    && (unsafe { (*pLevel).pRJ })
                                        == std::ptr::null_mut::<WhereRightJoin>()
                                {
                                    whereApplyPartialIndexConstraints(
                                        unsafe { (*(*__slate_slot_642)).pPartIdxWhere },
                                        *__slate_slot_630,
                                        *__slate_slot_634,
                                    );
                                }
                            } else {
                                {}
                                // The following assert() is not a requirement, merely an observation:
                                // The OR-optimization doesn't work for the right hand table of
                                // a LEFT JOIN:
                                0 as i32;
                            }
                            if (unsafe { (*(*__slate_slot_633)).wsFlags }) & ((4096 as i32) as u32)
                                != (0 as u32)
                            {
                                *__slate_slot_1151 = true as bool;
                            } else {
                                if (unsafe { (*pLevel).u.r#in.nIn }) != (0 as i32)
                                    && *__slate_slot_690 == (0 as i32)
                                {
                                    *__slate_slot_1152 =
                                        whereLoopIsOneRow(*__slate_slot_633) != (0 as i32);
                                } else {
                                    *__slate_slot_1152 = false as bool;
                                }
                                *__slate_slot_1151 = *__slate_slot_1152;
                            }
                            if *__slate_slot_1151 {
                                unsafe {
                                    (*pLevel).op = ((189 as i32) as i8) as u8;
                                }
                            } else {
                                if *__slate_slot_632 != (0 as i32) {
                                    unsafe {
                                        (*pLevel).op = ((39 as i32) as i8) as u8;
                                    }
                                } else {
                                    unsafe {
                                        (*pLevel).op = ((40 as i32) as i8) as u8;
                                    }
                                }
                            }
                            unsafe {
                                (*pLevel).p1 = *__slate_slot_682;
                            }
                            unsafe {
                                (*pLevel).p3 = ((if (unsafe { (*(*__slate_slot_633)).wsFlags })
                                    & ((65536 as i32) as u32)
                                    != ((0 as i32) as u32)
                                {
                                    1 as i32
                                } else {
                                    0 as i32
                                }) as i8) as u8;
                            }
                            if (unsafe { (*(*__slate_slot_633)).wsFlags }) & ((15 as i32) as u32)
                                == ((0 as i32) as u32)
                            {
                                unsafe {
                                    (*pLevel).p5 = ((1 as i32) as i8) as u8;
                                }
                            } else {
                                0 as i32;
                            }
                            if *__slate_slot_689 != (0 as i32) {
                                *__slate_slot_642 = std::ptr::null_mut::<Index>();
                            }
                        } else {
                            if (unsafe { (*(*__slate_slot_633)).wsFlags }) & ((8192 as i32) as u32)
                                != (0 as u32)
                            {
                                std::ptr::write(__slate_slot_700, std::ptr::null_mut::<Index>());
                                std::ptr::write(__slate_slot_1153, pParse);
                                std::ptr::write(__slate_slot_1154, unsafe {
                                    (*(*__slate_slot_1153)).nTab
                                });
                                std::ptr::write(__slate_slot_1155, *__slate_slot_1154 + (1 as i32));
                                unsafe {
                                    (*(*__slate_slot_1153)).nTab = *__slate_slot_1155;
                                }
                                *__slate_slot_701 = *__slate_slot_1154; // Cursor used for index scans (if any)
                                std::ptr::write(__slate_slot_1156, pParse);
                                std::ptr::write(__slate_slot_1157, unsafe {
                                    (*(*__slate_slot_1156)).nMem
                                });
                                std::ptr::write(__slate_slot_1158, *__slate_slot_1157 + (1 as i32));
                                unsafe {
                                    (*(*__slate_slot_1156)).nMem = *__slate_slot_1158;
                                }
                                *__slate_slot_702 = *__slate_slot_1158; // Register used with OP_Gosub
                                std::ptr::write(__slate_slot_703, 0 as i32);
                                std::ptr::write(__slate_slot_704, 0 as i32);
                                std::ptr::write(__slate_slot_705, unsafe {
                                    sqlite3VdbeMakeLabel(pParse)
                                });
                                std::ptr::write(__slate_slot_707, 0 as i32);
                                std::ptr::write(__slate_slot_709, std::ptr::null_mut::<Expr>());
                                std::ptr::write(__slate_slot_710, unsafe {
                                    (*(*__slate_slot_637)).pSTab
                                });
                                *__slate_slot_635 = unsafe {
                                    *unsafe {
                                        unsafe { (*(*__slate_slot_633)).aLTerm }
                                            .offset((0 as i32) as isize)
                                    }
                                };
                                0 as i32;
                                0 as i32;
                                0 as i32;
                                *__slate_slot_698 = unsafe {
                                    std::ptr::addr_of_mut!(
                                        (*unsafe { (*(*__slate_slot_635)).u.pOrInfo }).wc
                                    )
                                };
                                unsafe {
                                    (*pLevel).op = ((69 as i32) as i8) as u8;
                                }
                                unsafe {
                                    (*pLevel).p1 = *__slate_slot_702;
                                }
                                // Set up a new SrcList in pOrTab containing the table being scanned
                                // by this loop in the a[0] slot and all notReady tables in a[1..] slots.
                                // This becomes the SrcList in the recursive call to sqlite3WhereBegin().
                                if (((unsafe { (*pWInfo).nLevel }) as u32) as i32) > (1 as i32)
                                    || ((unsafe {
                                        (*(*__slate_slot_637)).fg.__slate_bits_0.__get_fromExists()
                                    }) as i32)
                                        != (0 as i32)
                                {
                                    *__slate_slot_711 = (((unsafe { (*pWInfo).nLevel }) as u32)
                                        as i32)
                                        - iLevel
                                        - (1 as i32);
                                    *__slate_slot_699 = (unsafe {
                                        sqlite3DbMallocRawNN(
                                            *__slate_slot_636,
                                            (8 as u64).wrapping_add(
                                                (((*__slate_slot_711 + (1 as i32)) as i64) as u64)
                                                    .wrapping_mul(72 as u64),
                                            ),
                                        )
                                    })
                                        as *mut SrcList;
                                    if *__slate_slot_699 == std::ptr::null_mut::<SrcList>() {
                                        return notReady;
                                    } else {
                                        unsafe {
                                            (*(*__slate_slot_699)).nAlloc =
                                                (((*__slate_slot_711 + (1 as i32)) as i8) as u8)
                                                    as u32;
                                        }
                                        unsafe {
                                            (*(*__slate_slot_699)).nSrc =
                                                (unsafe { (*(*__slate_slot_699)).nAlloc }) as i32;
                                        }
                                        unsafe {
                                            memcpy(
                                                (unsafe {
                                                    std::ptr::addr_of_mut!((*(*__slate_slot_699)).a)
                                                        as *mut SrcItem
                                                })
                                                    as *mut (),
                                                *__slate_slot_637 as *const (),
                                                72 as u64,
                                            )
                                        };
                                        *__slate_slot_712 = unsafe {
                                            std::ptr::addr_of_mut!(
                                                (*unsafe { (*pWInfo).pTabList }).a
                                            )
                                                as *mut SrcItem
                                        };
                                        *__slate_slot_629 = 1 as i32;
                                        loop {
                                            if *__slate_slot_629 <= *__slate_slot_711 {
                                                unsafe {
                                                    memcpy(
                                                        (unsafe {
                                                            unsafe {
                                                                std::ptr::addr_of_mut!(
                                                                    (*(*__slate_slot_699)).a
                                                                )
                                                                    as *mut SrcItem
                                                            }
                                                            .offset(*__slate_slot_629 as isize)
                                                        })
                                                            as *mut (),
                                                        (unsafe {
                                                            (*__slate_slot_712).offset(
                                                                (((unsafe {
                                                                    (*unsafe {
                                                                        pLevel.offset(
                                                                            *__slate_slot_629
                                                                                as isize,
                                                                        )
                                                                    })
                                                                    .iFrom
                                                                })
                                                                    as u32)
                                                                    as i32)
                                                                    as isize,
                                                            )
                                                        })
                                                            as *const (),
                                                        72 as u64,
                                                    )
                                                };
                                                std::ptr::write(
                                                    __slate_slot_1159,
                                                    *__slate_slot_629,
                                                );
                                                std::ptr::write(
                                                    __slate_slot_1160,
                                                    *__slate_slot_1159 + (1 as i32),
                                                );
                                                *__slate_slot_629 = *__slate_slot_1160;
                                            } else {
                                                break;
                                            }
                                        }
                                        // Clear the fromExists flag on the OR-optimized table entry so that
                                        // the calls to sqlite3WhereEnd() do not code early-exits after the
                                        // first row is visited. The early exit applies to this table's
                                        // overall loop - including the multiple OR branches and any WHERE
                                        // conditions not passed to the sub-loops - not to the sub-loops.
                                        unsafe {
                                            (*unsafe {
                                                unsafe {
                                                    std::ptr::addr_of_mut!((*(*__slate_slot_699)).a)
                                                        as *mut SrcItem
                                                }
                                                .offset((0 as i32) as isize)
                                            })
                                            .fg
                                            .__slate_bits_0
                                            .__set_fromExists((0 as i32) as u32);
                                        }
                                    }
                                } else {
                                    *__slate_slot_699 = unsafe { (*pWInfo).pTabList };
                                }
                                // Initialize the rowset register to contain NULL. An SQL NULL is
                                // equivalent to an empty rowset.  Or, create an ephemeral index
                                // capable of holding primary keys in the case of a WITHOUT ROWID.
                                //
                                // Also initialize regReturn to contain the address of the instruction
                                // immediately following the OP_Return at the bottom of the loop. This
                                // is required in a few obscure LEFT JOIN cases where control jumps
                                // over the top of the loop into the body of it. In this case the
                                // correct response for the end-of-loop code (the OP_Return) is to
                                // fall through to the next instruction, just as an OP_Next does if
                                // called on an uninitialized cursor.
                                if (((unsafe { (*pWInfo).wctrlFlags }) as u32) as i32) & (16 as i32)
                                    == (0 as i32)
                                {
                                    if (unsafe { (*(*__slate_slot_710)).tabFlags })
                                        & ((128 as i32) as u32)
                                        == ((0 as i32) as u32)
                                    {
                                        std::ptr::write(__slate_slot_1161, pParse);
                                        std::ptr::write(__slate_slot_1162, unsafe {
                                            (*(*__slate_slot_1161)).nMem
                                        });
                                        std::ptr::write(
                                            __slate_slot_1163,
                                            *__slate_slot_1162 + (1 as i32),
                                        );
                                        unsafe {
                                            (*(*__slate_slot_1161)).nMem = *__slate_slot_1163;
                                        }
                                        *__slate_slot_703 = *__slate_slot_1163;
                                        unsafe {
                                            sqlite3VdbeAddOp2(
                                                v,
                                                77 as i32,
                                                0 as i32,
                                                *__slate_slot_703,
                                            )
                                        };
                                    } else {
                                        std::ptr::write(__slate_slot_713, unsafe {
                                            sqlite3PrimaryKeyIndex(*__slate_slot_710)
                                        });
                                        std::ptr::write(__slate_slot_1164, pParse);
                                        std::ptr::write(__slate_slot_1165, unsafe {
                                            (*(*__slate_slot_1164)).nTab
                                        });
                                        std::ptr::write(
                                            __slate_slot_1166,
                                            *__slate_slot_1165 + (1 as i32),
                                        );
                                        unsafe {
                                            (*(*__slate_slot_1164)).nTab = *__slate_slot_1166;
                                        }
                                        *__slate_slot_703 = *__slate_slot_1165;
                                        unsafe {
                                            sqlite3VdbeAddOp2(
                                                v,
                                                120 as i32,
                                                *__slate_slot_703,
                                                ((unsafe { (*(*__slate_slot_713)).nKeyCol }) as u32)
                                                    as i32,
                                            )
                                        };
                                        unsafe {
                                            sqlite3VdbeSetP4KeyInfo(pParse, *__slate_slot_713)
                                        };
                                    }
                                    std::ptr::write(__slate_slot_1167, pParse);
                                    std::ptr::write(__slate_slot_1168, unsafe {
                                        (*(*__slate_slot_1167)).nMem
                                    });
                                    std::ptr::write(
                                        __slate_slot_1169,
                                        *__slate_slot_1168 + (1 as i32),
                                    );
                                    unsafe {
                                        (*(*__slate_slot_1167)).nMem = *__slate_slot_1169;
                                    }
                                    *__slate_slot_704 = *__slate_slot_1169;
                                }
                                *__slate_slot_706 = unsafe {
                                    sqlite3VdbeAddOp2(v, 73 as i32, 0 as i32, *__slate_slot_702)
                                };
                                // If the original WHERE clause is z of the form:  (x1 OR x2 OR ...) AND y
                                // Then for every term xN, evaluate as the subexpression: xN AND y
                                // That way, terms in y that are factored into the disjunction will
                                // be picked up by the recursive calls to sqlite3WhereBegin() below.
                                //
                                // Actually, each subexpression is converted to "xN AND w" where w is
                                // the "interesting" terms of z - terms that did not originate in the
                                // ON or USING clause of a LEFT JOIN, and terms that are usable as
                                // indices.
                                //
                                // This optimization also only applies if the (x1 OR x2 OR ...) term
                                // is not contained in the ON clause of a LEFT JOIN.
                                // See ticket http://sqlite.org/src/info/f2369304e4
                                //
                                // 2022-02-04:  Do not push down slices of a row-value comparison.
                                // In other words, "w" or "y" may not be a slice of a vector.  Otherwise,
                                // the initialization of the right-hand operand of the vector comparison
                                // might not occur, or might occur only in an OR branch that is not
                                // taken.  dbsqlfuzz 80a9fade844b4fb43564efc972bcb2c68270f5d1.
                                //
                                // 2022-03-03:  Do not push down expressions that involve subqueries.
                                // The subquery might get coded as a subroutine.  Any table-references
                                // in the subquery might be resolved to index-references for the index on
                                // the OR branch in which the subroutine is coded.  But if the subroutine
                                // is invoked from a different OR branch that uses a different index, such
                                // index-references will not work.  tag-20220303a
                                // https://sqlite.org/forum/forumpost/36937b197273d403
                                if (unsafe { (*(*__slate_slot_634)).nTerm }) > (1 as i32) {
                                    *__slate_slot_714 = 0 as i32;
                                    loop {
                                        if *__slate_slot_714
                                            < unsafe { (*(*__slate_slot_634)).nTerm }
                                        {
                                            '__join_268: {
                                                std::ptr::write(__slate_slot_715, unsafe {
                                                    (*unsafe {
                                                        unsafe { (*(*__slate_slot_634)).a }
                                                            .offset(*__slate_slot_714 as isize)
                                                    })
                                                    .pExpr
                                                });
                                                if (unsafe {
                                                    unsafe { (*(*__slate_slot_634)).a }
                                                        .offset(*__slate_slot_714 as isize)
                                                }) == *__slate_slot_635
                                                {
                                                } else {
                                                    {}
                                                    {}
                                                    {}
                                                    if (((unsafe {
                                                        (*unsafe {
                                                            unsafe { (*(*__slate_slot_634)).a }
                                                                .offset(*__slate_slot_714 as isize)
                                                        })
                                                        .wtFlags
                                                    })
                                                        as u32)
                                                        as i32)
                                                        & ((2 as i32) | (4 as i32) | (32768 as i32))
                                                        != (0 as i32)
                                                    {
                                                    } else {
                                                        if (((unsafe {
                                                            (*unsafe {
                                                                unsafe { (*(*__slate_slot_634)).a }
                                                                    .offset(
                                                                        *__slate_slot_714 as isize,
                                                                    )
                                                            })
                                                            .eOperator
                                                        })
                                                            as u32)
                                                            as i32)
                                                            & (16383 as i32)
                                                            == (0 as i32)
                                                        {
                                                        } else {
                                                            if (unsafe {
                                                                (*(*__slate_slot_715)).flags
                                                            }) & ((4194304 as i32) as u32)
                                                                != ((0 as i32) as u32)
                                                            {
                                                            } else {
                                                                // tag-20220303a
                                                                *__slate_slot_715 = unsafe {
                                                                    sqlite3ExprDup(
                                                                        *__slate_slot_636,
                                                                        *__slate_slot_715
                                                                            as *const Expr,
                                                                        0 as i32,
                                                                    )
                                                                };
                                                                *__slate_slot_709 = unsafe {
                                                                    sqlite3ExprAnd(
                                                                        pParse,
                                                                        *__slate_slot_709,
                                                                        *__slate_slot_715,
                                                                    )
                                                                };
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                            std::ptr::write(__slate_slot_1170, *__slate_slot_714);
                                            std::ptr::write(
                                                __slate_slot_1171,
                                                *__slate_slot_1170 + (1 as i32),
                                            );
                                            *__slate_slot_714 = *__slate_slot_1171;
                                        } else {
                                            break;
                                        }
                                    }
                                    if *__slate_slot_709 != std::ptr::null_mut::<Expr>() {
                                        // The extra 0x10000 bit on the opcode is masked off and does not
                                        // become part of the new Expr.op.  However, it does make the
                                        // op==TK_AND comparison inside of sqlite3PExpr() false, and this
                                        // prevents sqlite3PExpr() from applying the AND short-circuit
                                        // optimization, which we do not want here.
                                        *__slate_slot_709 = unsafe {
                                            sqlite3PExpr(
                                                pParse,
                                                (44 as i32) | (65536 as i32),
                                                std::ptr::null_mut::<Expr>(),
                                                *__slate_slot_709,
                                            )
                                        };
                                    }
                                }
                                // Run a separate WHERE clause for each term of the OR clause.  After
                                // eliminating duplicates from other WHERE clauses, the action for each
                                // sub-WHERE clause is to to invoke the main loop body as a subroutine.
                                unsafe {
                                    sqlite3VdbeExplain(
                                        pParse,
                                        ((1 as i32) as i8) as u8,
                                        (b"MULTI-INDEX OR\0".as_ptr() as *mut i8) as *const i8,
                                    )
                                };
                                *__slate_slot_708 = 0 as i32;
                                loop {
                                    if *__slate_slot_708 < unsafe { (*(*__slate_slot_698)).nTerm } {
                                        std::ptr::write(__slate_slot_716, unsafe {
                                            unsafe { (*(*__slate_slot_698)).a }
                                                .offset(*__slate_slot_708 as isize)
                                        });
                                        if (unsafe { (*(*__slate_slot_716)).leftCursor })
                                            == *__slate_slot_630
                                            || (((unsafe { (*(*__slate_slot_716)).eOperator })
                                                as u32)
                                                as i32)
                                                & (1024 as i32)
                                                != (0 as i32)
                                        {
                                            std::ptr::write(__slate_slot_718, unsafe {
                                                (*(*__slate_slot_716)).pExpr
                                            });
                                            std::ptr::write(__slate_slot_720, 0 as i32);
                                            {}
                                            // See TH3 vtab25.400 and ticket 614b25314c766238
                                            std::ptr::write(__slate_slot_1174, unsafe {
                                                sqlite3ExprDup(
                                                    *__slate_slot_636,
                                                    *__slate_slot_718 as *const Expr,
                                                    0 as i32,
                                                )
                                            });
                                            *__slate_slot_718 = *__slate_slot_1174;
                                            *__slate_slot_719 = *__slate_slot_1174;
                                            if (unsafe { (*(*__slate_slot_636)).mallocFailed })
                                                != (0 as u8)
                                            {
                                                unsafe {
                                                    sqlite3ExprDelete(
                                                        *__slate_slot_636,
                                                        *__slate_slot_719,
                                                    )
                                                };
                                            } else {
                                                if *__slate_slot_709 != std::ptr::null_mut::<Expr>()
                                                {
                                                    unsafe {
                                                        (*(*__slate_slot_709)).pLeft =
                                                            *__slate_slot_718;
                                                    }
                                                    *__slate_slot_718 = *__slate_slot_709;
                                                }
                                                // Loop through table entries that match term pOrTerm.
                                                unsafe {
                                                    sqlite3VdbeExplain(
                                                        pParse,
                                                        ((1 as i32) as i8) as u8,
                                                        (b"INDEX %d\0".as_ptr() as *mut i8)
                                                            as *const i8,
                                                        *__slate_slot_708 + (1 as i32),
                                                    )
                                                };
                                                {}
                                                *__slate_slot_717 = unsafe {
                                                    sqlite3WhereBegin(
                                                        pParse,
                                                        *__slate_slot_699,
                                                        *__slate_slot_718,
                                                        std::ptr::null_mut::<ExprList>(),
                                                        std::ptr::null_mut::<ExprList>(),
                                                        std::ptr::null_mut::<Select>(),
                                                        ((32 as i32) as i16) as u16,
                                                        *__slate_slot_701,
                                                    )
                                                };
                                                0 as i32;
                                                if *__slate_slot_717
                                                    != std::ptr::null_mut::<WhereInfo>()
                                                {
                                                    std::ptr::write(
                                                        __slate_slot_722,
                                                        sqlite3WhereExplainOneScan(
                                                            pParse,
                                                            *__slate_slot_699,
                                                            unsafe {
                                                                unsafe {
                                                                    std::ptr::addr_of_mut!(
                                                                        (*(*__slate_slot_717)).a
                                                                    )
                                                                        as *mut WhereLevel
                                                                }
                                                                .offset((0 as i32) as isize)
                                                            },
                                                            ((0 as i32) as i16) as u16,
                                                        ),
                                                    );
                                                    *__slate_slot_722;
                                                    // This is the sub-WHERE clause body.  First skip over
                                                    // duplicate rows from prior sub-WHERE clauses, and record the
                                                    // rowid (or PRIMARY KEY) for the current row so that the same
                                                    // row will be skipped in subsequent sub-WHERE clauses.
                                                    if (((unsafe { (*pWInfo).wctrlFlags }) as u32)
                                                        as i32)
                                                        & (16 as i32)
                                                        == (0 as i32)
                                                    {
                                                        std::ptr::write(
                                                            __slate_slot_723,
                                                            if *__slate_slot_708
                                                                == (unsafe {
                                                                    (*(*__slate_slot_698)).nTerm
                                                                }) - (1 as i32)
                                                            {
                                                                -(1 as i32)
                                                            } else {
                                                                *__slate_slot_708
                                                            },
                                                        );
                                                        if (unsafe {
                                                            (*(*__slate_slot_710)).tabFlags
                                                        }) & ((128 as i32) as u32)
                                                            == ((0 as i32) as u32)
                                                        {
                                                            unsafe {
                                                                sqlite3ExprCodeGetColumnOfTable(
                                                                    v,
                                                                    *__slate_slot_710,
                                                                    *__slate_slot_630,
                                                                    -(1 as i32),
                                                                    *__slate_slot_704,
                                                                )
                                                            };
                                                            *__slate_slot_720 = unsafe {
                                                                sqlite3VdbeAddOp4Int(
                                                                    v,
                                                                    49 as i32,
                                                                    *__slate_slot_703,
                                                                    0 as i32,
                                                                    *__slate_slot_704,
                                                                    *__slate_slot_723,
                                                                )
                                                            };
                                                            {}
                                                        } else {
                                                            std::ptr::write(
                                                                __slate_slot_724,
                                                                unsafe {
                                                                    sqlite3PrimaryKeyIndex(
                                                                        *__slate_slot_710,
                                                                    )
                                                                },
                                                            );
                                                            std::ptr::write(
                                                                __slate_slot_725,
                                                                ((unsafe {
                                                                    (*(*__slate_slot_724)).nKeyCol
                                                                })
                                                                    as u32)
                                                                    as i32,
                                                            );
                                                            // Read the PK into an array of temp registers.
                                                            *__slate_slot_727 = unsafe {
                                                                sqlite3GetTempRange(
                                                                    pParse,
                                                                    *__slate_slot_725,
                                                                )
                                                            };
                                                            *__slate_slot_726 = 0 as i32;
                                                            loop {
                                                                if *__slate_slot_726
                                                                    < *__slate_slot_725
                                                                {
                                                                    std::ptr::write(
                                                                        __slate_slot_728,
                                                                        (unsafe {
                                                                            *unsafe {
                                                                                unsafe { (*(*__slate_slot_724)).aiColumn }.offset(*__slate_slot_726 as isize)
                                                                            }
                                                                        })
                                                                            as i32,
                                                                    );
                                                                    unsafe {
                                                                        sqlite3ExprCodeGetColumnOfTable(v, *__slate_slot_710, *__slate_slot_630, *__slate_slot_728, *__slate_slot_727 + *__slate_slot_726)
                                                                    };
                                                                    std::ptr::write(
                                                                        __slate_slot_1175,
                                                                        *__slate_slot_726,
                                                                    );
                                                                    std::ptr::write(
                                                                        __slate_slot_1176,
                                                                        *__slate_slot_1175
                                                                            + (1 as i32),
                                                                    );
                                                                    *__slate_slot_726 =
                                                                        *__slate_slot_1176;
                                                                } else {
                                                                    break;
                                                                }
                                                            }
                                                            // Check if the temp table already contains this key. If so,
                                                            // the row has already been included in the result set and
                                                            // can be ignored (by jumping past the Gosub below). Otherwise,
                                                            // insert the key into the temp table and proceed with processing
                                                            // the row.
                                                            //
                                                            // Use some of the same optimizations as OP_RowSetTest: If iSet
                                                            // is zero, assume that the key cannot already be present in
                                                            // the temp table. And if iSet is -1, assume that there is no
                                                            // need to insert the key into the temp table, as it will never
                                                            // be tested for.
                                                            if *__slate_slot_723 != (0 as i32) {
                                                                *__slate_slot_720 = unsafe {
                                                                    sqlite3VdbeAddOp4Int(
                                                                        v,
                                                                        29 as i32,
                                                                        *__slate_slot_703,
                                                                        0 as i32,
                                                                        *__slate_slot_727,
                                                                        *__slate_slot_725,
                                                                    )
                                                                };
                                                                {}
                                                            }
                                                            if *__slate_slot_723 >= (0 as i32) {
                                                                unsafe {
                                                                    sqlite3VdbeAddOp3(
                                                                        v,
                                                                        99 as i32,
                                                                        *__slate_slot_727,
                                                                        *__slate_slot_725,
                                                                        *__slate_slot_704,
                                                                    )
                                                                };
                                                                unsafe {
                                                                    sqlite3VdbeAddOp4Int(
                                                                        v,
                                                                        140 as i32,
                                                                        *__slate_slot_703,
                                                                        *__slate_slot_704,
                                                                        *__slate_slot_727,
                                                                        *__slate_slot_725,
                                                                    )
                                                                };
                                                                if *__slate_slot_723 != (0 as i32) {
                                                                    unsafe {
                                                                        sqlite3VdbeChangeP5(
                                                                            v,
                                                                            ((16 as i32) as i16)
                                                                                as u16,
                                                                        )
                                                                    };
                                                                }
                                                            }
                                                            // Release the array of temp registers
                                                            unsafe {
                                                                sqlite3ReleaseTempRange(
                                                                    pParse,
                                                                    *__slate_slot_727,
                                                                    *__slate_slot_725,
                                                                )
                                                            };
                                                        }
                                                    }
                                                    // Invoke the main loop body as a subroutine
                                                    unsafe {
                                                        sqlite3VdbeAddOp2(
                                                            v,
                                                            10 as i32,
                                                            *__slate_slot_702,
                                                            *__slate_slot_705,
                                                        )
                                                    };
                                                    // Jump here (skipping the main loop body subroutine) if the
                                                    // current sub-WHERE row is a duplicate from prior sub-WHEREs.
                                                    if *__slate_slot_720 != (0 as i32) {
                                                        unsafe {
                                                            sqlite3VdbeJumpHere(
                                                                v,
                                                                *__slate_slot_720,
                                                            )
                                                        };
                                                    }
                                                    // The pSubWInfo->untestedTerms flag means that this OR term
                                                    // contained one or more AND term from a notReady table.  The
                                                    // terms from the notReady table could not be tested and will
                                                    // need to be tested later.
                                                    if ((unsafe {
                                                        (*(*__slate_slot_717))
                                                            .__slate_bits_0
                                                            .__get_untestedTerms()
                                                    })
                                                        as i32)
                                                        != (0 as i32)
                                                    {
                                                        *__slate_slot_707 = 1 as i32;
                                                    }
                                                    // If all of the OR-connected terms are optimized using the same
                                                    // index, and the index is opened using the same cursor number
                                                    // by each call to sqlite3WhereBegin() made by this loop, it may
                                                    // be possible to use that index as a covering index.
                                                    //
                                                    // If the call to sqlite3WhereBegin() above resulted in a scan that
                                                    // uses an index, and this is either the first OR-connected term
                                                    // processed or the index is the same as that used by all previous
                                                    // terms, set pCov to the candidate covering index. Otherwise, set
                                                    // pCov to NULL to indicate that no candidate covering index will
                                                    // be available.
                                                    *__slate_slot_721 = unsafe {
                                                        (*unsafe {
                                                            unsafe {
                                                                std::ptr::addr_of_mut!(
                                                                    (*(*__slate_slot_717)).a
                                                                )
                                                                    as *mut WhereLevel
                                                            }
                                                            .offset((0 as i32) as isize)
                                                        })
                                                        .pWLoop
                                                    };
                                                    0 as i32;
                                                    if (unsafe { (*(*__slate_slot_721)).wsFlags })
                                                        & ((512 as i32) as u32)
                                                        != ((0 as i32) as u32)
                                                        && (*__slate_slot_708 == (0 as i32)
                                                            || (unsafe {
                                                                (*(*__slate_slot_721))
                                                                    .u
                                                                    .btree
                                                                    .pIndex
                                                            }) == *__slate_slot_700)
                                                        && ((unsafe {
                                                            (*(*__slate_slot_710)).tabFlags
                                                        }) & ((128 as i32) as u32)
                                                            == ((0 as i32) as u32)
                                                            || !(((unsafe {
                                                                (*unsafe {
                                                                    (*(*__slate_slot_721))
                                                                        .u
                                                                        .btree
                                                                        .pIndex
                                                                })
                                                                .__slate_bits_0
                                                                .__get_idxType()
                                                            })
                                                                as i32)
                                                                == (2 as i32)))
                                                    {
                                                        0 as i32;
                                                        *__slate_slot_700 = unsafe {
                                                            (*(*__slate_slot_721)).u.btree.pIndex
                                                        };
                                                    } else {
                                                        *__slate_slot_700 =
                                                            std::ptr::null_mut::<Index>();
                                                    }
                                                    if (unsafe {
                                                        sqlite3WhereUsesDeferredSeek(
                                                            *__slate_slot_717,
                                                        )
                                                    }) != (0 as i32)
                                                    {
                                                        unsafe {
                                                            (*pWInfo)
                                                                .__slate_bits_0
                                                                .__set_bDeferredSeek(
                                                                    (1 as i32) as u32,
                                                                );
                                                        }
                                                    }
                                                    // Finish the loop through table entries that match term pOrTerm.
                                                    unsafe { sqlite3WhereEnd(*__slate_slot_717) };
                                                    unsafe { sqlite3VdbeExplainPop(pParse) };
                                                }
                                                unsafe {
                                                    sqlite3ExprDelete(
                                                        *__slate_slot_636,
                                                        *__slate_slot_719,
                                                    )
                                                };
                                            }
                                        }
                                        std::ptr::write(__slate_slot_1172, *__slate_slot_708);
                                        std::ptr::write(
                                            __slate_slot_1173,
                                            *__slate_slot_1172 + (1 as i32),
                                        );
                                        *__slate_slot_708 = *__slate_slot_1173;
                                    } else {
                                        break;
                                    }
                                }
                                unsafe { sqlite3VdbeExplainPop(pParse) };
                                0 as i32;
                                0 as i32;
                                0 as i32;
                                unsafe {
                                    (*pLevel).u.pCoveringIdx = *__slate_slot_700;
                                }
                                if *__slate_slot_700 != std::ptr::null_mut::<Index>() {
                                    unsafe {
                                        (*pLevel).iIdxCur = *__slate_slot_701;
                                    }
                                }
                                if *__slate_slot_709 != std::ptr::null_mut::<Expr>() {
                                    unsafe {
                                        (*(*__slate_slot_709)).pLeft = std::ptr::null_mut::<Expr>();
                                    }
                                    unsafe {
                                        sqlite3ExprDelete(*__slate_slot_636, *__slate_slot_709)
                                    };
                                }
                                unsafe {
                                    sqlite3VdbeChangeP1(v, *__slate_slot_706, unsafe {
                                        sqlite3VdbeCurrentAddr(v)
                                    })
                                };
                                unsafe { sqlite3VdbeGoto(v, unsafe { (*pLevel).addrBrk }) };
                                unsafe { sqlite3VdbeResolveLabel(v, *__slate_slot_705) };
                                // Set the P2 operand of the OP_Return opcode that will end the current
                                // loop to point to this spot, which is the top of the next containing
                                // loop.  The byte-code formatter will use that P2 value as a hint to
                                // indent everything in between the this point and the final OP_Return.
                                // See tag-20220407a in vdbe.c and shell.c
                                0 as i32;
                                unsafe {
                                    (*pLevel).p2 = unsafe { sqlite3VdbeCurrentAddr(v) };
                                }
                                if (unsafe { (*pWInfo).pTabList }) != *__slate_slot_699 {
                                    unsafe {
                                        sqlite3DbFreeNN(
                                            *__slate_slot_636,
                                            *__slate_slot_699 as *mut (),
                                        )
                                    };
                                }
                                if !(*__slate_slot_707 != (0 as i32)) {
                                    disableTerm(pLevel, *__slate_slot_635);
                                }
                            } else {
                                // Case 6:  There is no usable index.  We must do a complete
                                // scan of the entire table.
                                0 as i32;
                                if ((unsafe {
                                    (*(*__slate_slot_637)).fg.__slate_bits_0.__get_isRecursive()
                                }) as i32)
                                    != (0 as i32)
                                {
                                    // Tables marked isRecursive have only a single row that is stored in
                                    // a pseudo-cursor.  No need to Rewind or Next such cursors.
                                    unsafe {
                                        (*pLevel).op = ((189 as i32) as i8) as u8;
                                    }
                                } else {
                                    {}
                                    unsafe {
                                        (*pLevel).op = unsafe {
                                            *unsafe {
                                                unsafe { std::ptr::addr_of!(aStep) as *const u8 }
                                                    .offset(*__slate_slot_632 as isize)
                                            }
                                        };
                                    }
                                    unsafe {
                                        (*pLevel).p1 = *__slate_slot_630;
                                    }
                                    unsafe {
                                        (*pLevel).p2 = (1 as i32)
                                            + unsafe {
                                                sqlite3VdbeAddOp2(
                                                    v,
                                                    ((unsafe {
                                                        *unsafe {
                                                            unsafe {
                                                                std::ptr::addr_of!(aStart)
                                                                    as *const u8
                                                            }
                                                            .offset(*__slate_slot_632 as isize)
                                                        }
                                                    })
                                                        as u32)
                                                        as i32,
                                                    *__slate_slot_630,
                                                    unsafe { (*pLevel).addrHalt },
                                                )
                                            };
                                    }
                                    {}
                                    {}
                                    unsafe {
                                        (*pLevel).p5 = ((1 as i32) as i8) as u8;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        // Insert code to test every subexpression that can be completely
        // computed using the current set of tables.
        //
        // This loop may run between one and three times, depending on the
        // constraints to be generated. The value of stack variable iLoop
        // determines the constraints coded by each iteration, as follows:
        //
        // iLoop==1: Code only expressions that are entirely covered by pIdx.
        // iLoop==2: Code remaining expressions that do not contain correlated
        //           sub-queries.
        // iLoop==3: Code all remaining expressions.
        //
        // An effort is made to skip unnecessary iterations of the loop.
        //
        // This optimization of causing simple query restrictions to occur before
        // more complex one is call the "push-down" optimization in MySQL.  Here
        // in SQLite, the name is "MySQL push-down", since there is also another
        // totally unrelated optimization called "WHERE-clause push-down".
        // Sometimes the qualifier is omitted, resulting in an ambiguity, so beware.
        *__slate_slot_643 = if *__slate_slot_642 != std::ptr::null_mut::<Index>() {
            1 as i32
        } else {
            2 as i32
        };
        loop {
            std::ptr::write(__slate_slot_731, 0 as i32);
            *__slate_slot_635 = unsafe { (*(*__slate_slot_634)).a };
            std::ptr::write(__slate_slot_1177, unsafe { (*(*__slate_slot_634)).nTerm });
            *__slate_slot_628 = *__slate_slot_1177;
            loop {
                if *__slate_slot_628 > (0 as i32) {
                    '__join_37: {
                        std::ptr::write(__slate_slot_733, 0 as i32);
                        {}
                        {}
                        if (((unsafe { (*(*__slate_slot_635)).wtFlags }) as u32) as i32)
                            & ((2 as i32) | (4 as i32))
                            != (0 as i32)
                        {
                        } else {
                            if (unsafe { (*(*__slate_slot_635)).prereqAll })
                                & unsafe { (*pLevel).notReady }
                                != (((0 as i32) as i64) as u64)
                            {
                                {}
                                unsafe {
                                    (*pWInfo)
                                        .__slate_bits_0
                                        .__set_untestedTerms((1 as i32) as u32);
                                }
                            } else {
                                *__slate_slot_732 = unsafe { (*(*__slate_slot_635)).pExpr };
                                0 as i32;
                                if (((unsafe { (*(*__slate_slot_637)).fg.jointype }) as u32) as i32)
                                    & ((8 as i32) | (64 as i32) | (16 as i32))
                                    != (0 as i32)
                                {
                                    if !((unsafe { (*(*__slate_slot_732)).flags })
                                        & (((1 as i32) | (2 as i32)) as u32)
                                        != ((0 as i32) as u32))
                                    {
                                        // Defer processing WHERE clause constraints until after outer
                                        // join processing.  tag-20220513a
                                        break '__join_37;
                                    } else {
                                        if (((unsafe { (*(*__slate_slot_637)).fg.jointype }) as u32)
                                            as i32)
                                            & (8 as i32)
                                            == (8 as i32)
                                            && !((unsafe { (*(*__slate_slot_732)).flags })
                                                & ((1 as i32) as u32)
                                                != ((0 as i32) as u32))
                                        {
                                            break '__join_37;
                                        } else {
                                            std::ptr::write(__slate_slot_734, unsafe {
                                                sqlite3WhereGetMask(
                                                    unsafe {
                                                        std::ptr::addr_of_mut!((*pWInfo).sMaskSet)
                                                    },
                                                    unsafe { (*(*__slate_slot_732)).w.iJoin },
                                                )
                                            });
                                            if *__slate_slot_734 & unsafe { (*pLevel).notReady }
                                                != (0 as u64)
                                            {
                                                // An ON clause that is not ripe
                                                break '__join_37;
                                            }
                                        }
                                    }
                                }
                                if *__slate_slot_643 == (1 as i32) {
                                    *__slate_slot_1182 = !((unsafe {
                                        sqlite3ExprCoveredByIndex(
                                            *__slate_slot_732,
                                            unsafe { (*pLevel).iTabCur },
                                            *__slate_slot_642,
                                        )
                                    }) != (0 as i32));
                                } else {
                                    *__slate_slot_1182 = false as bool;
                                }
                                if *__slate_slot_1182 {
                                    *__slate_slot_731 = 2 as i32;
                                } else {
                                    if *__slate_slot_643 < (3 as i32)
                                        && (((unsafe { (*(*__slate_slot_635)).wtFlags }) as u32)
                                            as i32)
                                            & (4096 as i32)
                                            != (0 as i32)
                                    {
                                        if *__slate_slot_731 == (0 as i32) {
                                            *__slate_slot_731 = 3 as i32;
                                        }
                                    } else {
                                        if (((unsafe { (*(*__slate_slot_635)).wtFlags }) as u32)
                                            as i32)
                                            & (512 as i32)
                                            != (0 as i32)
                                        {
                                            std::ptr::write(__slate_slot_735, unsafe {
                                                (*pLevel).iLikeRepCntr
                                            });
                                            if *__slate_slot_735 > ((0 as i32) as u32) {
                                                *__slate_slot_733 = unsafe {
                                                    sqlite3VdbeAddOp1(
                                                        v,
                                                        if *__slate_slot_735 & ((1 as i32) as u32)
                                                            != (0 as u32)
                                                        {
                                                            17 as i32
                                                        } else {
                                                            16 as i32
                                                        },
                                                        (*__slate_slot_735 >> (1 as i32)) as i32,
                                                    )
                                                };
                                                {}
                                                {}
                                            }
                                        }
                                        unsafe {
                                            sqlite3ExprIfFalse(
                                                pParse,
                                                *__slate_slot_732,
                                                *__slate_slot_639,
                                                16 as i32,
                                            )
                                        };
                                        if *__slate_slot_733 != (0 as i32) {
                                            unsafe { sqlite3VdbeJumpHere(v, *__slate_slot_733) };
                                        }
                                        std::ptr::write(__slate_slot_1183, *__slate_slot_635);
                                        std::ptr::write(__slate_slot_1184, unsafe {
                                            (*(*__slate_slot_1183)).wtFlags
                                        });
                                        std::ptr::write(
                                            __slate_slot_1185,
                                            ((((*__slate_slot_1184 as u32) as i32) | (4 as i32))
                                                as i16)
                                                as u16,
                                        );
                                        unsafe {
                                            (*(*__slate_slot_1183)).wtFlags = *__slate_slot_1185;
                                        }
                                    }
                                }
                            }
                        }
                    }
                    std::ptr::write(__slate_slot_1178, *__slate_slot_628);
                    std::ptr::write(__slate_slot_1179, *__slate_slot_1178 - (1 as i32));
                    *__slate_slot_628 = *__slate_slot_1179;
                    std::ptr::write(__slate_slot_1180, *__slate_slot_635);
                    std::ptr::write(__slate_slot_1181, unsafe {
                        (*__slate_slot_1180).offset((1 as i32) as isize)
                    });
                    *__slate_slot_635 = *__slate_slot_1181;
                } else {
                    break;
                }
            }
            *__slate_slot_643 = *__slate_slot_731;
            if !(*__slate_slot_643 > (0 as i32)) {
                break;
            }
        }
        // Insert code to test for implied constraints based on transitivity
        // of the "==" operator.
        //
        // Example: If the WHERE clause contains "t1.a=t2.b" and "t2.b=123"
        // and we are coding the t1 loop and the t2 loop has not yet coded,
        // then we cannot use the "t1.a=t2.b" constraint, but we can code
        // the implied "t1.a=123" constraint.
        *__slate_slot_635 = unsafe { (*(*__slate_slot_634)).a };
        std::ptr::write(__slate_slot_1186, unsafe { (*(*__slate_slot_634)).nBase });
        *__slate_slot_628 = *__slate_slot_1186;
        loop {
            if *__slate_slot_628 > (0 as i32) {
                if (((unsafe { (*(*__slate_slot_635)).wtFlags }) as u32) as i32)
                    & ((2 as i32) | (4 as i32))
                    != (0 as i32)
                {
                } else {
                    if (((unsafe { (*(*__slate_slot_635)).eOperator }) as u32) as i32)
                        & ((2 as i32) | (128 as i32))
                        == (0 as i32)
                    {
                    } else {
                        if (((unsafe { (*(*__slate_slot_635)).eOperator }) as u32) as i32)
                            & (2048 as i32)
                            == (0 as i32)
                        {
                        } else {
                            if (unsafe { (*(*__slate_slot_635)).leftCursor }) != *__slate_slot_630 {
                            } else {
                                if (((unsafe { (*(*__slate_slot_637)).fg.jointype }) as u32) as i32)
                                    & ((8 as i32) | (64 as i32) | (16 as i32))
                                    != (0 as i32)
                                {
                                } else {
                                    *__slate_slot_736 = unsafe { (*(*__slate_slot_635)).pExpr };
                                    0 as i32;
                                    0 as i32;
                                    0 as i32;
                                    *__slate_slot_738 = unsafe {
                                        sqlite3WhereFindTerm(
                                            *__slate_slot_634,
                                            *__slate_slot_630,
                                            unsafe { (*(*__slate_slot_635)).u.x.leftColumn },
                                            notReady,
                                            ((2 as i32) | (1 as i32) | (128 as i32)) as u32,
                                            std::ptr::null_mut::<Index>(),
                                        )
                                    };
                                    if *__slate_slot_738 == std::ptr::null_mut::<WhereTerm>() {
                                    } else {
                                        if (((unsafe { (*(*__slate_slot_738)).wtFlags }) as u32)
                                            as i32)
                                            & (4 as i32)
                                            != (0 as i32)
                                        {
                                        } else {
                                            if (unsafe {
                                                (*unsafe { (*(*__slate_slot_738)).pExpr }).flags
                                            }) & ((512 as i32) as u32)
                                                != ((0 as i32) as u32)
                                            {
                                            } else {
                                                if (((unsafe { (*(*__slate_slot_738)).eOperator })
                                                    as u32)
                                                    as i32)
                                                    & (1 as i32)
                                                    != (0 as i32)
                                                    && (unsafe {
                                                        (*unsafe { (*(*__slate_slot_738)).pExpr })
                                                            .flags
                                                    }) & ((4096 as i32) as u32)
                                                        != ((0 as i32) as u32)
                                                    && (unsafe {
                                                        (*unsafe {
                                                            (*unsafe {
                                                                (*unsafe {
                                                                    (*(*__slate_slot_738)).pExpr
                                                                })
                                                                .x
                                                                .pSelect
                                                            })
                                                            .pEList
                                                        })
                                                        .nExpr
                                                    }) > (1 as i32)
                                                {
                                                } else {
                                                    {}
                                                    {}
                                                    {}
                                                    {}
                                                    *__slate_slot_737 = unsafe {
                                                        *unsafe { (*(*__slate_slot_738)).pExpr }
                                                    };
                                                    (*__slate_slot_737).pLeft =
                                                        unsafe { (*(*__slate_slot_736)).pLeft };
                                                    unsafe {
                                                        sqlite3ExprIfFalse(
                                                            pParse,
                                                            std::ptr::addr_of_mut!(
                                                                *__slate_slot_737
                                                            ),
                                                            *__slate_slot_639,
                                                            16 as i32,
                                                        )
                                                    };
                                                    std::ptr::write(
                                                        __slate_slot_1191,
                                                        *__slate_slot_738,
                                                    );
                                                    std::ptr::write(__slate_slot_1192, unsafe {
                                                        (*(*__slate_slot_1191)).wtFlags
                                                    });
                                                    std::ptr::write(
                                                        __slate_slot_1193,
                                                        ((((*__slate_slot_1192 as u32) as i32)
                                                            | (4 as i32))
                                                            as i16)
                                                            as u16,
                                                    );
                                                    unsafe {
                                                        (*(*__slate_slot_1191)).wtFlags =
                                                            *__slate_slot_1193;
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                std::ptr::write(__slate_slot_1187, *__slate_slot_628);
                std::ptr::write(__slate_slot_1188, *__slate_slot_1187 - (1 as i32));
                *__slate_slot_628 = *__slate_slot_1188;
                std::ptr::write(__slate_slot_1189, *__slate_slot_635);
                std::ptr::write(__slate_slot_1190, unsafe {
                    (*__slate_slot_1189).offset((1 as i32) as isize)
                });
                *__slate_slot_635 = *__slate_slot_1190;
            } else {
                break;
            }
        }
        // For a RIGHT OUTER JOIN, record the fact that the current row has
        // been matched at least once.
        if (unsafe { (*pLevel).pRJ }) != std::ptr::null_mut::<WhereRightJoin>() {
            '__join_15: {
                std::ptr::write(__slate_slot_742, 0 as i32);
                std::ptr::write(__slate_slot_743, unsafe { (*pLevel).pRJ });
                // pTab is the right-hand table of the RIGHT JOIN.  Generate code that
                // will record that the current row of that table has been matched at
                // least once.  This is accomplished by storing the PK for the row in
                // both the iMatch index and the regBloom Bloom filter.
                *__slate_slot_739 = unsafe {
                    (*unsafe {
                        unsafe {
                            std::ptr::addr_of_mut!((*unsafe { (*pWInfo).pTabList }).a)
                                as *mut SrcItem
                        }
                        .offset((((unsafe { (*pLevel).iFrom }) as u32) as i32) as isize)
                    })
                    .pSTab
                };
                if (unsafe { (*(*__slate_slot_739)).tabFlags }) & ((128 as i32) as u32)
                    == ((0 as i32) as u32)
                {
                    *__slate_slot_741 = unsafe { sqlite3GetTempRange(pParse, 2 as i32) };
                    unsafe {
                        sqlite3ExprCodeGetColumnOfTable(
                            v,
                            *__slate_slot_739,
                            unsafe { (*pLevel).iTabCur },
                            -(1 as i32),
                            *__slate_slot_741 + (1 as i32),
                        )
                    };
                    *__slate_slot_740 = 1 as i32;
                } else {
                    std::ptr::write(__slate_slot_745, unsafe {
                        sqlite3PrimaryKeyIndex(*__slate_slot_739)
                    });
                    *__slate_slot_740 = ((unsafe { (*(*__slate_slot_745)).nKeyCol }) as u32) as i32;
                    *__slate_slot_741 =
                        unsafe { sqlite3GetTempRange(pParse, *__slate_slot_740 + (1 as i32)) };
                    *__slate_slot_744 = 0 as i32;
                    loop {
                        if *__slate_slot_744 < *__slate_slot_740 {
                            std::ptr::write(
                                __slate_slot_746,
                                (unsafe {
                                    *unsafe {
                                        unsafe { (*(*__slate_slot_745)).aiColumn }
                                            .offset(*__slate_slot_744 as isize)
                                    }
                                }) as i32,
                            );
                            unsafe {
                                sqlite3ExprCodeGetColumnOfTable(
                                    v,
                                    *__slate_slot_739,
                                    *__slate_slot_630,
                                    *__slate_slot_746,
                                    *__slate_slot_741 + (1 as i32) + *__slate_slot_744,
                                )
                            };
                            std::ptr::write(__slate_slot_1194, *__slate_slot_744);
                            std::ptr::write(__slate_slot_1195, *__slate_slot_1194 + (1 as i32));
                            *__slate_slot_744 = *__slate_slot_1195;
                        } else {
                            break '__join_15;
                        }
                    }
                }
            }
            *__slate_slot_742 = unsafe {
                sqlite3VdbeAddOp4Int(
                    v,
                    29 as i32,
                    unsafe { (*(*__slate_slot_743)).iMatch },
                    0 as i32,
                    *__slate_slot_741 + (1 as i32),
                    *__slate_slot_740,
                )
            };
            {}
            unsafe {
                sqlite3VdbeComment(
                    v,
                    (b"match against %s\0".as_ptr() as *mut i8) as *const i8,
                    unsafe { (*(*__slate_slot_739)).zName },
                )
            };
            unsafe {
                sqlite3VdbeAddOp3(
                    v,
                    99 as i32,
                    *__slate_slot_741 + (1 as i32),
                    *__slate_slot_740,
                    *__slate_slot_741,
                )
            };
            unsafe {
                sqlite3VdbeAddOp4Int(
                    v,
                    140 as i32,
                    unsafe { (*(*__slate_slot_743)).iMatch },
                    *__slate_slot_741,
                    *__slate_slot_741 + (1 as i32),
                    *__slate_slot_740,
                )
            };
            if (unsafe { (*(*__slate_slot_743)).regBloom }) != (0 as i32) {
                unsafe {
                    sqlite3VdbeAddOp4Int(
                        v,
                        185 as i32,
                        unsafe { (*(*__slate_slot_743)).regBloom },
                        0 as i32,
                        *__slate_slot_741 + (1 as i32),
                        *__slate_slot_740,
                    )
                };
                unsafe { sqlite3VdbeChangeP5(v, ((16 as i32) as i16) as u16) };
            }
            unsafe { sqlite3VdbeJumpHere(v, *__slate_slot_742) };
            unsafe {
                sqlite3ReleaseTempRange(pParse, *__slate_slot_741, *__slate_slot_740 + (1 as i32))
            };
        }
        '__join_0: {
            '__join_8: {
                // For a LEFT OUTER JOIN, generate code that will record the fact that
                // at least one row of the right table has matched the left table.
                if (unsafe { (*pLevel).iLeftJoin }) != (0 as i32) {
                    unsafe {
                        (*pLevel).addrFirst = unsafe { sqlite3VdbeCurrentAddr(v) };
                    }
                    unsafe {
                        sqlite3VdbeAddOp2(v, 73 as i32, 1 as i32, unsafe { (*pLevel).iLeftJoin })
                    };
                    unsafe {
                        sqlite3VdbeComment(
                            v,
                            (b"record LEFT JOIN hit\0".as_ptr() as *mut i8) as *const i8,
                        )
                    };
                    if (unsafe { (*pLevel).pRJ }) == std::ptr::null_mut::<WhereRightJoin>() {
                        break '__join_8;
                    }
                }
                if (unsafe { (*pLevel).pRJ }) != std::ptr::null_mut::<WhereRightJoin>() {
                    std::ptr::write(__slate_slot_747, unsafe { (*pLevel).pRJ });
                    unsafe {
                        sqlite3VdbeAddOp2(v, 76 as i32, 0 as i32, unsafe {
                            (*(*__slate_slot_747)).regReturn
                        })
                    };
                    unsafe {
                        (*(*__slate_slot_747)).addrSubrtn = unsafe { sqlite3VdbeCurrentAddr(v) };
                    }
                    0 as i32;
                    std::ptr::write(__slate_slot_1196, pParse);
                    std::ptr::write(__slate_slot_1197, unsafe {
                        (*(*__slate_slot_1196)).withinRJSubrtn
                    });
                    std::ptr::write(
                        __slate_slot_1198,
                        ((((*__slate_slot_1197 as u32) as i32) + (1 as i32)) as i8) as u8,
                    );
                    unsafe {
                        (*(*__slate_slot_1196)).withinRJSubrtn = *__slate_slot_1198;
                    }
                // WHERE clause constraints must be deferred until after outer join
                // row elimination has completed, since WHERE clause constraints apply
                // to the results of the OUTER JOIN.  The following loop generates the
                // appropriate WHERE clause constraint checks.  tag-20220513a.
                } else {
                    break '__join_0;
                }
            }
            *__slate_slot_635 = unsafe { (*(*__slate_slot_634)).a };
            *__slate_slot_628 = 0 as i32;
            loop {
                if *__slate_slot_628 < unsafe { (*(*__slate_slot_634)).nBase } {
                    {}
                    {}
                    if (((unsafe { (*(*__slate_slot_635)).wtFlags }) as u32) as i32)
                        & ((2 as i32) | (4 as i32))
                        != (0 as i32)
                    {
                    } else {
                        if (unsafe { (*(*__slate_slot_635)).prereqAll })
                            & unsafe { (*pLevel).notReady }
                            != (((0 as i32) as i64) as u64)
                        {
                            0 as i32;
                        } else {
                            if (((unsafe { (*(*__slate_slot_637)).fg.jointype }) as u32) as i32)
                                & (64 as i32)
                                != (0 as i32)
                            {
                            } else {
                                0 as i32;
                                unsafe {
                                    sqlite3ExprIfFalse(
                                        pParse,
                                        unsafe { (*(*__slate_slot_635)).pExpr },
                                        *__slate_slot_639,
                                        16 as i32,
                                    )
                                };
                                std::ptr::write(__slate_slot_1203, *__slate_slot_635);
                                std::ptr::write(__slate_slot_1204, unsafe {
                                    (*(*__slate_slot_1203)).wtFlags
                                });
                                std::ptr::write(
                                    __slate_slot_1205,
                                    ((((*__slate_slot_1204 as u32) as i32) | (4 as i32)) as i16)
                                        as u16,
                                );
                                unsafe {
                                    (*(*__slate_slot_1203)).wtFlags = *__slate_slot_1205;
                                }
                            }
                        }
                    }
                    std::ptr::write(__slate_slot_1199, *__slate_slot_628);
                    std::ptr::write(__slate_slot_1200, *__slate_slot_1199 + (1 as i32));
                    *__slate_slot_628 = *__slate_slot_1200;
                    std::ptr::write(__slate_slot_1201, *__slate_slot_635);
                    std::ptr::write(__slate_slot_1202, unsafe {
                        (*__slate_slot_1201).offset((1 as i32) as isize)
                    });
                    *__slate_slot_635 = *__slate_slot_1202;
                } else {
                    break '__join_0;
                }
            }
        }
        return unsafe { (*pLevel).notReady };
    }
    // WHERE clause constraints
    return unsafe { std::mem::zeroed() };
}

static mut aStartOp: [u8; 8] = [
    ((0 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((36 as i32) as i8) as u8,
    ((32 as i32) as i8) as u8,
    ((24 as i32) as i8) as u8,
    ((21 as i32) as i8) as u8,
    ((23 as i32) as i8) as u8,
    ((22 as i32) as i8) as u8,
];

static mut aEndOp: [u8; 4] = [
    ((46 as i32) as i8) as u8,
    ((42 as i32) as i8) as u8,
    ((41 as i32) as i8) as u8,
    ((45 as i32) as i8) as u8,
];

static mut aStep: [u8; 2] = [((40 as i32) as i8) as u8, ((39 as i32) as i8) as u8];

static mut aStart: [u8; 2] = [((36 as i32) as i8) as u8, ((32 as i32) as i8) as u8];

/// Generate the code for the loop that finds all non-matched terms
/// for a RIGHT JOIN.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3WhereRightJoinLoop(
    mut pWInfo: *mut WhereInfo,
    mut iLevel: i32,
    mut pLevel: *mut WhereLevel,
) {
    let mut pParse: *mut Parse = unsafe { (*pWInfo).pParse };
    let mut v: *mut Vdbe = unsafe { (*pParse).pVdbe };
    let mut pRJ: *mut WhereRightJoin = unsafe { (*pLevel).pRJ };
    let mut pSubWhere: *mut Expr = std::ptr::null_mut::<Expr>();
    let mut pWC: *mut WhereClause = unsafe { std::ptr::addr_of_mut!((*pWInfo).sWC) };
    let mut pSubWInfo: *mut WhereInfo = unsafe { std::mem::zeroed() };
    let mut pLoop: *mut WhereLoop = unsafe { (*pLevel).pWLoop };
    let mut pTabItem: *mut SrcItem = unsafe {
        unsafe { std::ptr::addr_of_mut!((*unsafe { (*pWInfo).pTabList }).a) as *mut SrcItem }
            .offset((((unsafe { (*pLevel).iFrom }) as u32) as i32) as isize)
    };
    let mut pFrom: *mut SrcList = unsafe { std::mem::zeroed() };
    let mut uSrc: __SlateRecord218 = unsafe { std::mem::zeroed() };
    let mut mAll: u64 = ((0 as i32) as i64) as u64;
    let mut k: i32 = 0 as i32;
    unsafe {
        sqlite3VdbeExplain(
            pParse,
            ((1 as i32) as i8) as u8,
            (b"RIGHT-JOIN %s\0".as_ptr() as *mut i8) as *const i8,
            unsafe { (*unsafe { (*pTabItem).pSTab }).zName },
        )
    };
    {}
    k = 0 as i32;
    '__slate_break_1082: loop {
        if !(k < iLevel) {
            break;
        }
        let mut iIdxCur: i32 = 0 as i32;
        let mut pRight: *mut SrcItem = unsafe { std::mem::zeroed() };
        0 as i32;
        pRight = unsafe {
            unsafe { std::ptr::addr_of_mut!((*unsafe { (*pWInfo).pTabList }).a) as *mut SrcItem }
                .offset(
                    (((unsafe {
                        (*unsafe {
                            unsafe { std::ptr::addr_of_mut!((*pWInfo).a) as *mut WhereLevel }
                                .offset(k as isize)
                        })
                        .iFrom
                    }) as u32) as i32) as isize,
                )
        };
        let __v1208: u64 = mAll;
        let __v1209: u64 = __v1208
            | unsafe {
                (*unsafe {
                    (*unsafe {
                        unsafe { std::ptr::addr_of_mut!((*pWInfo).a) as *mut WhereLevel }
                            .offset(k as isize)
                    })
                    .pWLoop
                })
                .maskSelf
            };
        mAll = __v1209;
        if ((unsafe { (*pRight).fg.__slate_bits_0.__get_viaCoroutine() }) as i32) != (0 as i32) {
            let mut pSubq: *mut Subquery = unsafe { std::mem::zeroed() };
            0 as i32;
            pSubq = unsafe { (*pRight).u4.pSubq };
            0 as i32;
            unsafe {
                sqlite3VdbeAddOp3(
                    v,
                    77 as i32,
                    0 as i32,
                    unsafe { (*pSubq).regResult },
                    (unsafe { (*pSubq).regResult })
                        + unsafe { (*unsafe { (*unsafe { (*pSubq).pSelect }).pEList }).nExpr }
                        - (1 as i32),
                )
            };
        }
        unsafe {
            sqlite3VdbeAddOp1(v, 138 as i32, unsafe {
                (*unsafe {
                    unsafe { std::ptr::addr_of_mut!((*pWInfo).a) as *mut WhereLevel }
                        .offset(k as isize)
                })
                .iTabCur
            })
        };
        iIdxCur = unsafe {
            (*unsafe {
                unsafe { std::ptr::addr_of_mut!((*pWInfo).a) as *mut WhereLevel }.offset(k as isize)
            })
            .iIdxCur
        };
        if iIdxCur != (0 as i32) {
            unsafe { sqlite3VdbeAddOp1(v, 138 as i32, iIdxCur) };
        }
        let __v1206: i32 = k;
        let __v1207: i32 = __v1206 + (1 as i32);
        k = __v1207;
    }
    if (((unsafe { (*pTabItem).fg.jointype }) as u32) as i32) & (64 as i32) == (0 as i32) {
        let __v1210: u64 = mAll;
        let __v1211: u64 = __v1210 | unsafe { (*pLoop).maskSelf };
        mAll = __v1211;
        k = 0 as i32;
        '__slate_break_1083: loop {
            if !(k < unsafe { (*pWC).nTerm }) {
                break;
            }
            let mut pTerm: *mut WhereTerm = unsafe { unsafe { (*pWC).a }.offset(k as isize) };
            if (((unsafe { (*pTerm).wtFlags }) as u32) as i32) & ((2 as i32) | (32768 as i32))
                != (0 as i32)
                && (((unsafe { (*pTerm).eOperator }) as u32) as i32) != (8192 as i32)
            {
                break '__slate_break_1083;
            }
            if (unsafe { (*pTerm).prereqAll }) & !mAll != (0 as u64) {
            } else {
                if (unsafe { (*unsafe { (*pTerm).pExpr }).flags })
                    & (((1 as i32) | (2 as i32)) as u32)
                    != ((0 as i32) as u32)
                {
                } else {
                    pSubWhere = unsafe {
                        sqlite3ExprAnd(pParse, pSubWhere, unsafe {
                            sqlite3ExprDup(
                                unsafe { (*pParse).db },
                                (unsafe { (*pTerm).pExpr }) as *const Expr,
                                0 as i32,
                            )
                        })
                    };
                }
            }
            let __v1212: i32 = k;
            let __v1213: i32 = __v1212 + (1 as i32);
            k = __v1213;
        }
    }
    if (unsafe { (*pLevel).iIdxCur }) != (0 as i32) {
        // pSubWhere may contain expressions that read from an index on the
        // table on the RHS of the right join. All such expressions first test
        // if the index is pointing at a NULL row, and if so, read from the
        // table cursor instead. So ensure that the index cursor really is
        // pointing at a NULL row here, so that no values are read from it during
        // the scan of the RHS of the RIGHT join below.
        unsafe { sqlite3VdbeAddOp1(v, 138 as i32, unsafe { (*pLevel).iIdxCur }) };
    }
    pFrom = unsafe { std::ptr::addr_of_mut!(uSrc.sSrc) };
    unsafe {
        (*pFrom).nSrc = 1 as i32;
    }
    unsafe {
        (*pFrom).nAlloc = (1 as i32) as u32;
    }
    unsafe {
        memcpy(
            (unsafe {
                unsafe { std::ptr::addr_of_mut!((*pFrom).a) as *mut SrcItem }
                    .offset((0 as i32) as isize)
            }) as *mut (),
            pTabItem as *const (),
            72 as u64,
        )
    };
    unsafe {
        (*unsafe {
            unsafe { std::ptr::addr_of_mut!((*pFrom).a) as *mut SrcItem }
                .offset((0 as i32) as isize)
        })
        .fg
        .jointype = ((0 as i32) as i8) as u8;
    }
    if (((unsafe { (*pParse).withinRJSubrtn }) as u32) as i32) >= (100 as i32) {
        // This limit --------------^^^
        // is based on an historical assert().  It is not compile-time or
        // run-time configurable.  It could perhaps be raised as high as 254,
        // but only an attack robot would ever do even 100 RIGHT JOINS within
        // a single query, so we'll just leave it as it is.
        unsafe {
            sqlite3ErrorMsg(
                pParse,
                (b"too many RIGHT JOINs\0".as_ptr() as *mut i8) as *const i8,
            )
        };
        return;
    }
    let __v1214: *mut Parse = pParse;
    let __v1215: u8 = unsafe { (*__v1214).withinRJSubrtn };
    let __v1216: u8 = ((((__v1215 as u32) as i32) + (1 as i32)) as i8) as u8;
    unsafe {
        (*__v1214).withinRJSubrtn = __v1216;
    }
    pSubWInfo = unsafe {
        sqlite3WhereBegin(
            pParse,
            pFrom,
            pSubWhere,
            std::ptr::null_mut::<ExprList>(),
            std::ptr::null_mut::<ExprList>(),
            std::ptr::null_mut::<Select>(),
            ((4096 as i32) as i16) as u16,
            0 as i32,
        )
    };
    if pSubWInfo != std::ptr::null_mut::<WhereInfo>() {
        let mut iCur: i32 = unsafe { (*pLevel).iTabCur }; // Table on RHS of RIGHT JOIN &
        let mut r: i32 = 0 as i32;
        let __v1217: *mut Parse = pParse;
        let __v1218: i32 = unsafe { (*__v1217).nMem };
        let __v1219: i32 = __v1218 + (1 as i32);
        unsafe {
            (*__v1217).nMem = __v1219;
        }
        r = __v1219; // Register range to hold primary key
        let mut nPk: i32 = 0 as i32; // Number of values in the primary key
        let mut r2: i32 = 0 as i32; // Register holding record for primary key
        let mut addrCont: i32 = unsafe { sqlite3WhereContinueLabel(pSubWInfo) };
        let mut pTab: *mut Table = unsafe { (*pTabItem).pSTab }; // Table on RHS of RIGHT JOIN
        if (unsafe { (*pTab).tabFlags }) & ((128 as i32) as u32) == ((0 as i32) as u32) {
            unsafe { sqlite3ExprCodeGetColumnOfTable(v, pTab, iCur, -(1 as i32), r) };
            nPk = 1 as i32;
        } else {
            let mut iPk: i32 = 0 as i32;
            let mut pPk: *mut Index = unsafe { sqlite3PrimaryKeyIndex(pTab) };
            nPk = ((unsafe { (*pPk).nKeyCol }) as u32) as i32;
            let __v1220: *mut Parse = pParse;
            let __v1221: i32 = unsafe { (*__v1220).nMem };
            let __v1222: i32 = __v1221 + (nPk - (1 as i32));
            unsafe {
                (*__v1220).nMem = __v1222;
            }
            iPk = 0 as i32;
            '__slate_break_1085: loop {
                if !(iPk < nPk) {
                    break;
                }
                let mut iCol: i32 =
                    (unsafe { *unsafe { unsafe { (*pPk).aiColumn }.offset(iPk as isize) } }) as i32;
                unsafe { sqlite3ExprCodeGetColumnOfTable(v, pTab, iCur, iCol, r + iPk) };
                let __v1223: i32 = iPk;
                let __v1224: i32 = __v1223 + (1 as i32);
                iPk = __v1224;
            }
        }
        // Generate code that checks to see if the current row of the RHS table
        // has appeared in any prior output row.
        if (unsafe { (*pRJ).regBloom }) != (0 as i32) {
            unsafe {
                sqlite3VdbeAddOp4Int(
                    v,
                    66 as i32,
                    unsafe { (*pRJ).regBloom },
                    (unsafe { sqlite3VdbeCurrentAddr(v) }) + (2 as i32),
                    r,
                    nPk,
                )
            };
            {}
        }
        unsafe { sqlite3VdbeAddOp4Int(v, 29 as i32, unsafe { (*pRJ).iMatch }, addrCont, r, nPk) };
        {}
        r2 = unsafe { sqlite3GetTempReg(pParse) };
        // Generate code that inserts the PK of the RHS table into the
        // pRH->iMatch index to indicate that the current row has appeared
        // in the output set.
        unsafe { sqlite3VdbeAddOp3(v, 99 as i32, r, nPk, r2) };
        unsafe { sqlite3VdbeAddOp4Int(v, 140 as i32, unsafe { (*pRJ).iMatch }, r2, r, nPk) };
        unsafe { sqlite3ReleaseTempReg(pParse, r2) };
        if (unsafe { (*pRJ).regBloom }) != (0 as i32) {
            unsafe {
                sqlite3VdbeAddOp4Int(v, 185 as i32, unsafe { (*pRJ).regBloom }, 0 as i32, r, nPk)
            };
            unsafe { sqlite3VdbeChangeP5(v, ((16 as i32) as i16) as u16) };
        }
        // Invoke the subroutine that actually puts the current RHS table row
        // into the output set, with NULLs for the LHS.
        unsafe {
            sqlite3VdbeAddOp2(v, 10 as i32, unsafe { (*pRJ).regReturn }, unsafe {
                (*pRJ).addrSubrtn
            })
        };
        unsafe { sqlite3WhereEnd(pSubWInfo) };
    }
    unsafe { sqlite3ExprDelete(unsafe { (*pParse).db }, pSubWhere) };
    unsafe { sqlite3VdbeExplainPop(pParse) };
    0 as i32;
    let __v1225: *mut Parse = pParse;
    let __v1226: u8 = unsafe { (*__v1225).withinRJSubrtn };
    let __v1227: u8 = ((((__v1226 as u32) as i32) - (1 as i32)) as i8) as u8;
    unsafe {
        (*__v1225).withinRJSubrtn = __v1227;
    }
}
