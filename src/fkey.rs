//! The author disclaims copyright to this source code.  In place of
//! a legal notice, here is a blessing:
//!
//!    May you do good and not evil.
//!    May you find forgiveness for yourself and forgive others.
//!    May you share freely, never taking more than you give.
//!
//!
//! This file contains code used by the compiler to add foreign key
//! support to compiled SQL statements.
unsafe extern "C" {
    static mut sqlite3StrBINARY: [i8; 0];
    fn sqlite3_stricmp(__v567: *const i8, __v568: *const i8) -> i32;
    fn sqlite3HashInsert(__v569: *mut Hash, pKey: *const i8, pData: *mut ()) -> *mut ();
    fn sqlite3HashFind(__v572: *const Hash, pKey: *const i8) -> *mut ();
    fn memset(__s: *mut (), __c: i32, __n: u64) -> *mut ();
    fn sqlite3VdbeAddOp1(__v577: *mut Vdbe, __v578: i32, __v579: i32) -> i32;
    fn sqlite3VdbeAddOp2(__v580: *mut Vdbe, __v581: i32, __v582: i32, __v583: i32) -> i32;
    fn sqlite3VdbeGoto(__v584: *mut Vdbe, __v585: i32) -> i32;
    fn sqlite3VdbeAddOp3(
        __v586: *mut Vdbe,
        __v587: i32,
        __v588: i32,
        __v589: i32,
        __v590: i32,
    ) -> i32;
    fn sqlite3VdbeAddOp4(
        __v591: *mut Vdbe,
        __v592: i32,
        __v593: i32,
        __v594: i32,
        __v595: i32,
        zP4: *const i8,
        __v597: i32,
    ) -> i32;
    fn sqlite3VdbeAddOp4Int(
        __v598: *mut Vdbe,
        __v599: i32,
        __v600: i32,
        __v601: i32,
        __v602: i32,
        __v603: i32,
    ) -> i32;
    fn sqlite3VdbeChangeP5(__v604: *mut Vdbe, P5: u16);
    fn sqlite3VdbeJumpHere(__v606: *mut Vdbe, addr: i32);
    fn sqlite3VdbeJumpHereOrPopInst(__v608: *mut Vdbe, addr: i32);
    fn sqlite3VdbeSetP4KeyInfo(__v610: *mut Parse, __v611: *mut Index);
    fn sqlite3VdbeMakeLabel(__v612: *mut Parse) -> i32;
    fn sqlite3VdbeResolveLabel(__v613: *mut Vdbe, __v614: i32);
    fn sqlite3VdbeCurrentAddr(__v615: *mut Vdbe) -> i32;
    fn sqlite3StrICmp(__v616: *const i8, __v617: *const i8) -> i32;
    fn sqlite3Strlen30(__v618: *const i8) -> i32;
    fn sqlite3DbMallocZero(__v619: *mut sqlite3, __v620: u64) -> *mut ();
    fn sqlite3DbMallocRawNN(__v621: *mut sqlite3, __v622: u64) -> *mut ();
    fn sqlite3DbStrDup(__v623: *mut sqlite3, __v624: *const i8) -> *mut i8;
    fn sqlite3DbStrNDup(__v625: *mut sqlite3, __v626: *const i8, __v627: u64) -> *mut i8;
    fn sqlite3DbFree(__v628: *mut sqlite3, __v629: *mut ());
    fn sqlite3ErrorMsg(__v630: *mut Parse, __v631: *const i8, ...);
    fn sqlite3TokenInit(__v632: *mut Token, __v633: *mut i8);
    fn sqlite3GetTempReg(__v634: *mut Parse) -> i32;
    fn sqlite3ReleaseTempReg(__v635: *mut Parse, __v636: i32);
    fn sqlite3GetTempRange(__v637: *mut Parse, __v638: i32) -> i32;
    fn sqlite3ReleaseTempRange(__v639: *mut Parse, __v640: i32, __v641: i32);
    fn sqlite3ExprAlloc(
        __v642: *mut sqlite3,
        __v643: i32,
        __v644: *const Token,
        __v645: i32,
    ) -> *mut Expr;
    fn sqlite3Expr(__v646: *mut sqlite3, __v647: i32, __v648: *const i8) -> *mut Expr;
    fn sqlite3PExpr(
        __v649: *mut Parse,
        __v650: i32,
        __v651: *mut Expr,
        __v652: *mut Expr,
    ) -> *mut Expr;
    fn sqlite3ExprAnd(__v653: *mut Parse, __v654: *mut Expr, __v655: *mut Expr) -> *mut Expr;
    fn sqlite3ExprDelete(__v656: *mut sqlite3, __v657: *mut Expr);
    fn sqlite3ExprListAppend(
        __v658: *mut Parse,
        __v659: *mut ExprList,
        __v660: *mut Expr,
    ) -> *mut ExprList;
    fn sqlite3ExprListSetName(
        __v661: *mut Parse,
        __v662: *mut ExprList,
        __v663: *const Token,
        __v664: i32,
    );
    fn sqlite3ExprListDelete(__v665: *mut sqlite3, __v666: *mut ExprList);
    fn sqlite3ColumnExpr(__v667: *mut Table, __v668: *mut Column) -> *mut Expr;
    fn sqlite3ColumnColl(__v669: *mut Column) -> *const i8;
    fn sqlite3TableColumnToStorage(__v670: *mut Table, __v671: i16) -> i16;
    fn sqlite3SrcListAppend(
        __v672: *mut Parse,
        __v673: *mut SrcList,
        __v674: *mut Token,
        __v675: *mut Token,
    ) -> *mut SrcList;
    fn sqlite3SrcListDelete(__v676: *mut sqlite3, __v677: *mut SrcList);
    fn sqlite3SelectNew(
        __v678: *mut Parse,
        __v679: *mut ExprList,
        __v680: *mut SrcList,
        __v681: *mut Expr,
        __v682: *mut ExprList,
        __v683: *mut Expr,
        __v684: *mut ExprList,
        __v685: u32,
        __v686: *mut Expr,
    ) -> *mut Select;
    fn sqlite3SelectDelete(__v687: *mut sqlite3, __v688: *mut Select);
    fn sqlite3OpenTable(__v689: *mut Parse, iCur: i32, iDb: i32, __v692: *mut Table, __v693: i32);
    fn sqlite3DeleteFrom(
        __v694: *mut Parse,
        __v695: *mut SrcList,
        __v696: *mut Expr,
        __v697: *mut ExprList,
        __v698: *mut Expr,
    );
    fn sqlite3WhereBegin(
        __v699: *mut Parse,
        __v700: *mut SrcList,
        __v701: *mut Expr,
        __v702: *mut ExprList,
        __v703: *mut ExprList,
        __v704: *mut Select,
        __v705: u16,
        __v706: i32,
    ) -> *mut WhereInfo;
    fn sqlite3WhereEnd(__v707: *mut WhereInfo);
    fn sqlite3FindTable(__v708: *mut sqlite3, __v709: *const i8, __v710: *const i8) -> *mut Table;
    fn sqlite3LocateTable(
        __v711: *mut Parse,
        flags: u32,
        __v713: *const i8,
        __v714: *const i8,
    ) -> *mut Table;
    fn sqlite3GetVdbe(__v715: *mut Parse) -> *mut Vdbe;
    fn sqlite3MayAbort(__v716: *mut Parse);
    fn sqlite3HaltConstraint(
        __v717: *mut Parse,
        __v718: i32,
        __v719: i32,
        __v720: *mut i8,
        __v721: i8,
        __v722: u8,
    );
    fn sqlite3ExprDup(__v723: *mut sqlite3, __v724: *const Expr, __v725: i32) -> *mut Expr;
    fn sqlite3ExprListDup(
        __v726: *mut sqlite3,
        __v727: *const ExprList,
        __v728: i32,
    ) -> *mut ExprList;
    fn sqlite3SrcListDup(__v729: *mut sqlite3, __v730: *const SrcList, __v731: i32)
    -> *mut SrcList;
    fn sqlite3SelectDup(__v732: *mut sqlite3, __v733: *const Select, __v734: i32) -> *mut Select;
    fn sqlite3CodeRowTriggerDirect(
        __v735: *mut Parse,
        __v736: *mut Trigger,
        __v737: *mut Table,
        __v738: i32,
        __v739: i32,
        __v740: i32,
    );
    fn sqlite3AuthReadCol(
        __v741: *mut Parse,
        __v742: *const i8,
        __v743: *const i8,
        __v744: i32,
    ) -> i32;
    fn sqlite3IndexAffinityStr(__v745: *mut sqlite3, __v746: *mut Index) -> *const i8;
    fn sqlite3ExprAddCollateString(
        __v747: *const Parse,
        __v748: *mut Expr,
        __v749: *const i8,
    ) -> *mut Expr;
    fn sqlite3ResolveExprNames(__v750: *mut NameContext, __v751: *mut Expr) -> i32;
    fn sqlite3SchemaToIndex(db: *mut sqlite3, __v753: *mut Schema) -> i32;
    fn sqlite3TableLock(
        __v754: *mut Parse,
        __v755: i32,
        __v756: u32,
        __v757: u8,
        __v758: *const i8,
    );
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
    trace: __SlateRecord161,
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
    u1: __SlateRecord162,
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
    u: __SlateRecord163,
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
    u: __SlateRecord164,
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
    u: __SlateRecord172,
    pLeft: *mut Expr,
    pRight: *mut Expr,
    x: __SlateRecord173,
    nHeight: i32,
    iTable: i32,
    iColumn: i16,
    iAgg: i16,
    w: __SlateRecord174,
    pAggInfo: *mut AggInfo,
    y: __SlateRecord175,
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
    fg: __SlateRecord182,
    iCursor: i32,
    colUsed: u64,
    u1: __SlateRecord183,
    u2: __SlateRecord184,
    u3: __SlateRecord185,
    u4: __SlateRecord186,
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
    uNC: __SlateRecord187,
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
struct VtabCtx {}

#[repr(C)]
#[derive(Clone, Copy)]
struct WhereInfo {}

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
    __slate_bits_0: __slate_bits::__SlateBits160U0,
    azInit: *mut *const i8,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord161 {
    xLegacy: Option<unsafe extern "C-unwind" fn(*mut (), *const i8)>,
    xV2: Option<unsafe extern "C-unwind" fn(u32, *mut (), *mut (), *mut ()) -> i32>,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord162 {
    isInterrupted: i32,
    notUsed1: f64,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord163 {
    pHash: *mut FuncDef,
    pDestructor: *mut FuncDestructor,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord164 {
    tab: __SlateRecord165,
    view: __SlateRecord166,
    vtab: __SlateRecord167,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord165 {
    addColOffset: i32,
    pFKey: *mut FKey,
    pDfltList: *mut ExprList,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord166 {
    pSelect: *mut Select,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord167 {
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
union __SlateRecord172 {
    zToken: *mut i8,
    iValue: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord173 {
    pList: *mut ExprList,
    pSelect: *mut Select,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord174 {
    iJoin: i32,
    iOfst: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord175 {
    pTab: *mut Table,
    pWin: *mut Window,
    nReg: i32,
    sub: __SlateRecord176,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord176 {
    iAddr: i32,
    regReturn: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct ExprList_item {
    pExpr: *mut Expr,
    zEName: *mut i8,
    fg: __SlateRecord178,
    u: __SlateRecord179,
}

#[repr(C, align(4))]
#[derive(Clone, Copy)]
struct __SlateRecord178 {
    sortFlags: u8,
    __slate_bits_0: __slate_bits::__SlateBits178U0,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord179 {
    x: __SlateRecord180,
    iConstExprReg: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord180 {
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
struct __SlateRecord182 {
    jointype: u8,
    __slate_bits_0: __slate_bits::__SlateBits182U0,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord183 {
    zIndexedBy: *mut i8,
    pFuncArg: *mut ExprList,
    nRow: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord184 {
    pIBIndex: *mut Index,
    pCteUse: *mut CteUse,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord185 {
    pOn: *mut Expr,
    pUsing: *mut IdList,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord186 {
    pSchema: *mut Schema,
    zDatabase: *mut i8,
    pSubq: *mut Subquery,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord187 {
    pEList: *mut ExprList,
    pAggInfo: *mut AggInfo,
    pUpsert: *mut Upsert,
    iBaseReg: i32,
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
    pub struct __SlateBits178U0 {
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
    pub struct __SlateBits182U0 {
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
    pub struct __SlateBits160U0 {
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
}

// Deferred and Immediate FKs
//
// Foreign keys in SQLite come in two flavours: deferred and immediate.
// If an immediate foreign key constraint is violated,
// SQLITE_CONSTRAINT_FOREIGNKEY is returned and the current
// statement transaction rolled back. If a
// deferred foreign key constraint is violated, no action is taken
// immediately. However if the application attempts to commit the
// transaction before fixing the constraint violation, the attempt fails.
//
// Deferred constraints are implemented using a simple counter associated
// with the database handle. The counter is set to zero each time a
// database transaction is opened. Each time a statement is executed
// that causes a foreign key violation, the counter is incremented. Each
// time a statement is executed that removes an existing violation from
// the database, the counter is decremented. When the transaction is
// committed, the commit fails if the current value of the counter is
// greater than zero. This scheme has two big drawbacks:
//
//   * When a commit fails due to a deferred foreign key constraint,
//     there is no way to tell which foreign constraint is not satisfied,
//     or which row it is not satisfied for.
//
//   * If the database contains foreign key violations when the
//     transaction is opened, this may cause the mechanism to malfunction.
//
// Despite these problems, this approach is adopted as it seems simpler
// than the alternatives.
//
// INSERT operations:
//
//   I.1) For each FK for which the table is the child table, search
//        the parent table for a match. If none is found increment the
//        constraint counter.
//
//   I.2) For each FK for which the table is the parent table,
//        search the child table for rows that correspond to the new
//        row in the parent table. Decrement the counter for each row
//        found (as the constraint is now satisfied).
//
// DELETE operations:
//
//   D.1) For each FK for which the table is the child table,
//        search the parent table for a row that corresponds to the
//        deleted row in the child table. If such a row is not found,
//        decrement the counter.
//
//   D.2) For each FK for which the table is the parent table, search
//        the child table for rows that correspond to the deleted row
//        in the parent table. For each found increment the counter.
//
// UPDATE operations:
//
//   An UPDATE command requires that all 4 steps above are taken, but only
//   for FK constraints for which the affected columns are actually
//   modified (values must be compared at runtime).
//
// Note that I.1 and D.1 are very similar operations, as are I.2 and D.2.
// This simplifies the implementation a bit.
//
// For the purposes of immediate FK constraints, the OR REPLACE conflict
// resolution is considered to delete rows before the new row is inserted.
// If a delete caused by OR REPLACE violates an FK constraint, an exception
// is thrown, even if the FK constraint would be satisfied after the new
// row is inserted.
//
// Immediate constraints are usually handled similarly. The only difference
// is that the counter used is stored as part of each individual statement
// object (struct Vdbe). If, after the statement has run, its immediate
// constraint counter is greater than zero,
// it returns SQLITE_CONSTRAINT_FOREIGNKEY
// and the statement transaction is rolled back. An exception is an INSERT
// statement that inserts a single row only (no triggers). In this case,
// instead of using a counter, an exception is thrown immediately if the
// INSERT violates a foreign key constraint. This is necessary as such
// an INSERT does not open a statement transaction.
//
// TODO: How should dropping a table be handled? How should renaming a
// table be handled?
//
//
// Query API Notes
//
// Before coding an UPDATE or DELETE row operation, the code-generator
// for those two operations needs to know whether or not the operation
// requires any FK processing and, if so, which columns of the original
// row are required by the FK processing VDBE code (i.e. if FKs were
// implemented using triggers, which of the old.* columns would be
// accessed). No information is required by the code-generator before
// coding an INSERT operation. The functions used by the UPDATE/DELETE
// generation code to query for this information are:
//
//   sqlite3FkRequired() - Test to see if FK processing is required.
//   sqlite3FkOldmask()  - Query for the set of required old.* columns.
//
//
// Externally accessible module functions
//
//   sqlite3FkCheck()    - Check for foreign key violations.
//   sqlite3FkActions()  - Code triggers for ON UPDATE/ON DELETE actions.
//   sqlite3FkDelete()   - Delete an FKey structure.
// VDBE Calling Convention
//
// Example:
//
//   For the following INSERT statement:
//
//     CREATE TABLE t1(a, b INTEGER PRIMARY KEY, c);
//     INSERT INTO t1 VALUES(1, 2, 3.1);
//
//   Register (x):        2    (type integer)
//   Register (x+1):      1    (type integer)
//   Register (x+2):      NULL (type NULL)
//   Register (x+3):      3.1  (type real)
/// A foreign key constraint requires that the key columns in the parent
/// table are collectively subject to a UNIQUE or PRIMARY KEY constraint.
/// Given that pParent is the parent table for foreign key constraint pFKey,
/// search the schema for a unique index on the parent key columns.
///
/// If successful, zero is returned. If the parent key is an INTEGER PRIMARY
/// KEY column, then output variable *ppIdx is set to NULL. Otherwise, *ppIdx
/// is set to point to the unique index.
///
/// If the parent key consists of a single column (the foreign key constraint
/// is not a composite foreign key), output variable *paiCol is set to NULL.
/// Otherwise, it is set to point to an allocated array of size N, where
/// N is the number of columns in the parent key. The first element of the
/// array is the index of the child table column that is mapped by the FK
/// constraint to the parent table column stored in the left-most column
/// of index *ppIdx. The second element of the array is the index of the
/// child table column that corresponds to the second left-most column of
/// *ppIdx, and so on.
///
/// If the required index cannot be found, either because:
///
///   1) The named parent key columns do not exist, or
///
///   2) The named parent key columns do exist, but are not subject to a
///      UNIQUE or PRIMARY KEY constraint, or
///
///   3) No parent key columns were provided explicitly as part of the
///      foreign key definition, and the parent table does not have a
///      PRIMARY KEY, or
///
///   4) No parent key columns were provided explicitly as part of the
///      foreign key definition, and the PRIMARY KEY of the parent table
///      consists of a different number of columns to the child key in
///      the child table.
///
/// then non-zero is returned, and a "foreign key mismatch" error loaded
/// into pParse. If an OOM error occurs, non-zero is returned and the
/// pParse->db->mallocFailed flag is set.
///
/// # Arguments
///
/// * `pParse` - Parse context to store any error in
/// * `pParent` - Parent table of FK constraint pFKey
/// * `pFKey` - Foreign key to find index for
/// * `ppIdx` - OUT: Unique index on parent table
/// * `paiCol` - OUT: Map of index columns in pFKey
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3FkLocateIndex(
    mut pParse: *mut Parse,
    mut pParent: *mut Table,
    mut pFKey: *mut FKey,
    mut ppIdx: *mut *mut Index,
    mut paiCol: *mut *mut i32,
) -> i32 {
    let mut pIdx: *mut Index = std::ptr::null_mut::<Index>(); // Value to return via *ppIdx
    let mut aiCol: *mut i32 = std::ptr::null_mut::<i32>(); // Value to return via *paiCol
    let mut nCol: i32 = unsafe { (*pFKey).nCol }; // Number of columns in parent key
    let mut zKey: *mut i8 = unsafe {
        (*unsafe {
            unsafe { std::ptr::addr_of_mut!((*pFKey).aCol) as *mut sColMap }
                .offset((0 as i32) as isize)
        })
        .zCol
    }; // Name of left-most parent key column
    // The caller is responsible for zeroing output parameters.
    0 as i32;
    0 as i32;
    0 as i32;
    // If this is a non-composite (single column) foreign key, check if it
    // maps to the INTEGER PRIMARY KEY of table pParent. If so, leave *ppIdx
    // and *paiCol set to zero and return early.
    //
    // Otherwise, for a composite foreign key (more than one column), allocate
    // space for the aiCol array (returned via output parameter *paiCol).
    // Non-composite foreign keys do not require the aiCol array.
    if nCol == (1 as i32) {
        // The FK maps to the IPK if any of the following are true:
        //
        // 1) There is an INTEGER PRIMARY KEY column and the FK is implicitly
        //    mapped to the primary key of table pParent, or
        // 2) The FK is explicitly mapped to a column declared as INTEGER
        //    PRIMARY KEY.
        if ((unsafe { (*pParent).iPKey }) as i32) >= (0 as i32) {
            if !(zKey != std::ptr::null_mut::<i8>()) {
                return 0 as i32;
            }
            if !((unsafe {
                sqlite3StrICmp(
                    (unsafe {
                        (*unsafe {
                            unsafe { (*pParent).aCol }
                                .offset(((unsafe { (*pParent).iPKey }) as i32) as isize)
                        })
                        .zCnName
                    }) as *const i8,
                    zKey as *const i8,
                )
            }) != (0 as i32))
            {
                return 0 as i32;
            }
        }
    } else {
        if paiCol != std::ptr::null_mut::<*mut i32>() {
            0 as i32;
            aiCol = (unsafe {
                sqlite3DbMallocRawNN(
                    unsafe { (*pParse).db },
                    ((nCol as i64) as u64).wrapping_mul(4 as u64),
                )
            }) as *mut i32;
            if !(aiCol != std::ptr::null_mut::<i32>()) {
                return 1 as i32;
            }
            unsafe {
                *paiCol = aiCol;
            }
        }
    }
    pIdx = unsafe { (*pParent).pIndex };
    '__slate_break_790: while pIdx != std::ptr::null_mut::<Index>() {
        if (((unsafe { (*pIdx).nKeyCol }) as u32) as i32) == nCol
            && (((unsafe { (*pIdx).onError }) as u32) as i32) != (0 as i32)
            && (unsafe { (*pIdx).pPartIdxWhere }) == std::ptr::null_mut::<Expr>()
        {
            // pIdx is a UNIQUE index (or a PRIMARY KEY) and has the right number
            // of columns. If each indexed column corresponds to a foreign key
            // column of pFKey, then this index is a winner.
            if zKey == std::ptr::null_mut::<i8>() {
                // If zKey is NULL, then this foreign key is implicitly mapped to
                // the PRIMARY KEY of table pParent. The PRIMARY KEY index may be
                // identified by the test.
                if ((unsafe { (*pIdx).__slate_bits_0.__get_idxType() }) as i32) == (2 as i32) {
                    if aiCol != std::ptr::null_mut::<i32>() {
                        let mut i: i32 = 0 as i32;
                        i = 0 as i32;
                        '__slate_break_791: loop {
                            if !(i < nCol) {
                                break;
                            }
                            unsafe {
                                *unsafe { aiCol.offset(i as isize) } = unsafe {
                                    (*unsafe {
                                        unsafe {
                                            std::ptr::addr_of_mut!((*pFKey).aCol) as *mut sColMap
                                        }
                                        .offset(i as isize)
                                    })
                                    .iFrom
                                };
                            }
                            let __v850: i32 = i;
                            let __v851: i32 = __v850 + (1 as i32);
                            i = __v851;
                        }
                    }
                    break '__slate_break_790;
                }
            } else {
                // If zKey is non-NULL, then this foreign key was declared to
                // map to an explicit list of columns in table pParent. Check if this
                // index matches those columns. Also, check that the index uses
                // the default collation sequences for each column.
                let mut i: i32 = 0 as i32;
                let mut j: i32 = 0 as i32;
                i = 0 as i32;
                '__slate_break_792: loop {
                    if !(i < nCol) {
                        break;
                    }
                    let mut iCol: i16 =
                        unsafe { *unsafe { unsafe { (*pIdx).aiColumn }.offset(i as isize) } }; // Index of column in parent tbl
                    let mut zDfltColl: *const i8 = unsafe { std::mem::zeroed() }; // Def. collation for column
                    let mut zIdxCol: *mut i8 = unsafe { std::mem::zeroed() }; // Name of indexed column
                    if (iCol as i32) < (0 as i32) {
                        break '__slate_break_792;
                    }
                    // No foreign keys against expression indexes
                    // If the index uses a collation sequence that is different from
                    // the default collation sequence for the column, this index is
                    // unusable. Bail out early in this case.
                    zDfltColl = unsafe {
                        sqlite3ColumnColl(unsafe {
                            unsafe { (*pParent).aCol }.offset((iCol as i32) as isize)
                        })
                    };
                    if !(zDfltColl != std::ptr::null::<i8>()) {
                        zDfltColl = unsafe { std::ptr::addr_of!(sqlite3StrBINARY) as *const i8 };
                    }
                    if (unsafe {
                        sqlite3StrICmp(
                            unsafe { *unsafe { unsafe { (*pIdx).azColl }.offset(i as isize) } },
                            zDfltColl,
                        )
                    }) != (0 as i32)
                    {
                        break '__slate_break_792;
                    }
                    zIdxCol = unsafe {
                        (*unsafe { unsafe { (*pParent).aCol }.offset((iCol as i32) as isize) })
                            .zCnName
                    };
                    j = 0 as i32;
                    '__slate_break_793: loop {
                        if !(j < nCol) {
                            break;
                        }
                        if (unsafe {
                            sqlite3StrICmp(
                                (unsafe {
                                    (*unsafe {
                                        unsafe {
                                            std::ptr::addr_of_mut!((*pFKey).aCol) as *mut sColMap
                                        }
                                        .offset(j as isize)
                                    })
                                    .zCol
                                }) as *const i8,
                                zIdxCol as *const i8,
                            )
                        }) == (0 as i32)
                        {
                            if aiCol != std::ptr::null_mut::<i32>() {
                                unsafe {
                                    *unsafe { aiCol.offset(i as isize) } = unsafe {
                                        (*unsafe {
                                            unsafe {
                                                std::ptr::addr_of_mut!((*pFKey).aCol)
                                                    as *mut sColMap
                                            }
                                            .offset(j as isize)
                                        })
                                        .iFrom
                                    };
                                }
                            }
                            break '__slate_break_793;
                        }
                        let __v854: i32 = j;
                        let __v855: i32 = __v854 + (1 as i32);
                        j = __v855;
                    }
                    if j == nCol {
                        break '__slate_break_792;
                    }
                    let __v852: i32 = i;
                    let __v853: i32 = __v852 + (1 as i32);
                    i = __v853;
                }
                if i == nCol {
                    break '__slate_break_790;
                }
                // pIdx is usable
            }
        }
        pIdx = unsafe { (*pIdx).pNext };
    }
    if !(pIdx != std::ptr::null_mut::<Index>()) {
        if !(((unsafe { (*pParse).__slate_bits_0.__get_disableTriggers() }) as i32) != (0 as i32)) {
            unsafe {
                sqlite3ErrorMsg(
                    pParse,
                    (b"foreign key mismatch - \"%w\" referencing \"%w\"\0".as_ptr() as *mut i8)
                        as *const i8,
                    unsafe { (*unsafe { (*pFKey).pFrom }).zName },
                    unsafe { (*pFKey).zTo },
                )
            };
        }
        unsafe { sqlite3DbFree(unsafe { (*pParse).db }, aiCol as *mut ()) };
        return 1 as i32;
    }
    unsafe {
        *ppIdx = pIdx;
    }
    return 0 as i32;
}

/// This function is called when a row is inserted into or deleted from the
/// child table of foreign key constraint pFKey. If an SQL UPDATE is executed
/// on the child table of pFKey, this function is invoked twice for each row
/// affected - once to "delete" the old row, and then again to "insert" the
/// new row.
///
/// Each time it is called, this function generates VDBE code to locate the
/// row in the parent table that corresponds to the row being inserted into
/// or deleted from the child table. If the parent row can be found, no
/// special action is taken. Otherwise, if the parent row can *not* be
/// found in the parent table:
///
///   Operation | FK type   | Action taken
///   INSERT      immediate   Increment the "immediate constraint counter".
///
///   DELETE      immediate   Decrement the "immediate constraint counter".
///
///   INSERT      deferred    Increment the "deferred constraint counter".
///
///   DELETE      deferred    Decrement the "deferred constraint counter".
///
/// These operations are identified in the comment at the top of this file
/// (fkey.c) as "I.1" and "D.1".
///
/// # Arguments
///
/// * `pParse` - Parse context
/// * `iDb` - Index of database housing pTab
/// * `pTab` - Parent table of FK pFKey
/// * `pIdx` - Unique index on parent key columns in pTab
/// * `pFKey` - Foreign key constraint
/// * `aiCol` - Map from parent key columns to child table columns
/// * `regData` - Address of array containing child table row
/// * `nIncr` - Increment constraint counter by this
/// * `isIgnore` - If true, pretend pTab contains all NULL values
fn fkLookupParent(
    mut pParse: *mut Parse,
    mut iDb: i32,
    mut pTab: *mut Table,
    mut pIdx: *mut Index,
    mut pFKey: *mut FKey,
    mut aiCol: *mut i32,
    mut regData: i32,
    mut nIncr: i32,
    mut isIgnore: i32,
) {
    let mut i: i32 = 0 as i32; // Iterator variable
    let mut v: *mut Vdbe = unsafe { sqlite3GetVdbe(pParse) }; // Vdbe to add code to
    let mut iCur: i32 = (unsafe { (*pParse).nTab }) - (1 as i32); // Cursor number to use
    let mut iOk: i32 = unsafe { sqlite3VdbeMakeLabel(pParse) }; // jump here if parent key found
    {}
    // If nIncr is less than zero, then check at runtime if there are any
    // outstanding constraints to resolve. If there are not, there is no need
    // to check if deleting this row resolves any outstanding violations.
    //
    // Check if any of the key columns in the child table row are NULL. If
    // any are, then the constraint is considered satisfied. No need to
    // search for a matching row in the parent table.
    if nIncr < (0 as i32) {
        unsafe {
            sqlite3VdbeAddOp2(
                v,
                60 as i32,
                ((unsafe { (*pFKey).isDeferred }) as u32) as i32,
                iOk,
            )
        };
        {}
    }
    i = 0 as i32;
    '__slate_break_795: loop {
        if !(i < unsafe { (*pFKey).nCol }) {
            break;
        }
        let mut iReg: i32 = ((unsafe {
            sqlite3TableColumnToStorage(
                unsafe { (*pFKey).pFrom },
                (unsafe { *unsafe { aiCol.offset(i as isize) } }) as i16,
            )
        }) as i32)
            + regData
            + (1 as i32);
        unsafe { sqlite3VdbeAddOp2(v, 51 as i32, iReg, iOk) };
        {}
        let __v856: i32 = i;
        let __v857: i32 = __v856 + (1 as i32);
        i = __v857;
    }
    if isIgnore == (0 as i32) {
        if pIdx == std::ptr::null_mut::<Index>() {
            // If pIdx is NULL, then the parent key is the INTEGER PRIMARY KEY
            // column of the parent table (table pTab).
            let mut iMustBeInt: i32 = 0 as i32; // Address of MustBeInt instruction
            let mut regTemp: i32 = unsafe { sqlite3GetTempReg(pParse) };
            // Invoke MustBeInt to coerce the child key value to an integer (i.e.
            // apply the affinity of the parent key). If this fails, then there
            // is no matching parent key. Before using MustBeInt, make a copy of
            // the value. Otherwise, the value inserted into the child key column
            // will have INTEGER affinity applied to it, which may not be correct.
            unsafe {
                sqlite3VdbeAddOp2(
                    v,
                    83 as i32,
                    ((unsafe {
                        sqlite3TableColumnToStorage(
                            unsafe { (*pFKey).pFrom },
                            (unsafe { *unsafe { aiCol.offset((0 as i32) as isize) } }) as i16,
                        )
                    }) as i32)
                        + (1 as i32)
                        + regData,
                    regTemp,
                )
            };
            iMustBeInt = unsafe { sqlite3VdbeAddOp2(v, 13 as i32, regTemp, 0 as i32) };
            {}
            // If the parent table is the same as the child table, and we are about
            // to increment the constraint-counter (i.e. this is an INSERT operation),
            // then check if the row being inserted matches itself. If so, do not
            // increment the constraint-counter.
            if pTab == unsafe { (*pFKey).pFrom } && nIncr == (1 as i32) {
                unsafe { sqlite3VdbeAddOp3(v, 54 as i32, regData, iOk, regTemp) };
                {}
                unsafe { sqlite3VdbeChangeP5(v, ((144 as i32) as i16) as u16) };
            }
            unsafe { sqlite3OpenTable(pParse, iCur, iDb, pTab, 114 as i32) };
            unsafe { sqlite3VdbeAddOp3(v, 31 as i32, iCur, 0 as i32, regTemp) };
            {}
            unsafe { sqlite3VdbeGoto(v, iOk) };
            unsafe { sqlite3VdbeJumpHere(v, (unsafe { sqlite3VdbeCurrentAddr(v) }) - (2 as i32)) };
            unsafe { sqlite3VdbeJumpHere(v, iMustBeInt) };
            unsafe { sqlite3ReleaseTempReg(pParse, regTemp) };
        } else {
            let mut nCol: i32 = unsafe { (*pFKey).nCol };
            let mut regTemp: i32 = unsafe { sqlite3GetTempRange(pParse, nCol) };
            unsafe {
                sqlite3VdbeAddOp3(v, 114 as i32, iCur, (unsafe { (*pIdx).tnum }) as i32, iDb)
            };
            unsafe { sqlite3VdbeSetP4KeyInfo(pParse, pIdx) };
            i = 0 as i32;
            '__slate_break_796: loop {
                if !(i < nCol) {
                    break;
                }
                unsafe {
                    sqlite3VdbeAddOp2(
                        v,
                        82 as i32,
                        ((unsafe {
                            sqlite3TableColumnToStorage(
                                unsafe { (*pFKey).pFrom },
                                (unsafe { *unsafe { aiCol.offset(i as isize) } }) as i16,
                            )
                        }) as i32)
                            + (1 as i32)
                            + regData,
                        regTemp + i,
                    )
                };
                let __v858: i32 = i;
                let __v859: i32 = __v858 + (1 as i32);
                i = __v859;
            }
            // If the parent table is the same as the child table, and we are about
            // to increment the constraint-counter (i.e. this is an INSERT operation),
            // then check if the row being inserted matches itself. If so, do not
            // increment the constraint-counter.
            //
            // If any of the parent-key values are NULL, then the row cannot match
            // itself. So set JUMPIFNULL to make sure we do the OP_Found if any
            // of the parent-key values are NULL (at this point it is known that
            // none of the child key values are).
            if pTab == unsafe { (*pFKey).pFrom } && nIncr == (1 as i32) {
                let mut iJump: i32 = (unsafe { sqlite3VdbeCurrentAddr(v) }) + nCol + (1 as i32);
                i = 0 as i32;
                '__slate_break_797: loop {
                    if !(i < nCol) {
                        break;
                    }
                    let mut iChild: i32 = ((unsafe {
                        sqlite3TableColumnToStorage(
                            unsafe { (*pFKey).pFrom },
                            (unsafe { *unsafe { aiCol.offset(i as isize) } }) as i16,
                        )
                    }) as i32)
                        + (1 as i32)
                        + regData;
                    let mut iParent: i32 = (1 as i32) + regData;
                    let __v862: i32 = iParent;
                    let __v863: i32 = __v862
                        + ((unsafe {
                            sqlite3TableColumnToStorage(unsafe { (*pIdx).pTable }, unsafe {
                                *unsafe { unsafe { (*pIdx).aiColumn }.offset(i as isize) }
                            })
                        }) as i32);
                    iParent = __v863;
                    0 as i32;
                    0 as i32;
                    if ((unsafe { *unsafe { unsafe { (*pIdx).aiColumn }.offset(i as isize) } })
                        as i32)
                        == ((unsafe { (*pTab).iPKey }) as i32)
                    {
                        // The parent key is a composite key that includes the IPK column
                        iParent = regData;
                    }
                    unsafe { sqlite3VdbeAddOp3(v, 53 as i32, iChild, iJump, iParent) };
                    {}
                    unsafe { sqlite3VdbeChangeP5(v, ((16 as i32) as i16) as u16) };
                    let __v860: i32 = i;
                    let __v861: i32 = __v860 + (1 as i32);
                    i = __v861;
                }
                unsafe { sqlite3VdbeGoto(v, iOk) };
            }
            unsafe {
                sqlite3VdbeAddOp4(
                    v,
                    98 as i32,
                    regTemp,
                    nCol,
                    0 as i32,
                    unsafe { sqlite3IndexAffinityStr(unsafe { (*pParse).db }, pIdx) },
                    nCol,
                )
            };
            unsafe { sqlite3VdbeAddOp4Int(v, 29 as i32, iCur, iOk, regTemp, nCol) };
            {}
            unsafe { sqlite3ReleaseTempRange(pParse, regTemp, nCol) };
        }
    }
    if !((unsafe { (*pFKey).isDeferred }) != (0 as u8))
        && !((unsafe { (*unsafe { (*pParse).db }).flags }) & (((524288 as i32) as i64) as u64)
            != (0 as u64))
        && !((unsafe { (*pParse).pToplevel }) != std::ptr::null_mut::<Parse>())
        && !((unsafe { (*pParse).isMultiWrite }) != (0 as u8))
    {
        // Special case: If this is an INSERT statement that will insert exactly
        // one row into the table, raise a constraint immediately instead of
        // incrementing a counter. This is necessary as the VM code is being
        // generated for will not open a statement transaction.
        0 as i32;
        unsafe {
            sqlite3HaltConstraint(
                pParse,
                (19 as i32) | (3 as i32) << (8 as i32),
                2 as i32,
                std::ptr::null_mut::<i8>(),
                -(1 as i32) as i8,
                ((4 as i32) as i8) as u8,
            )
        };
    } else {
        if nIncr > (0 as i32) && (((unsafe { (*pFKey).isDeferred }) as u32) as i32) == (0 as i32) {
            unsafe { sqlite3MayAbort(pParse) };
        }
        unsafe {
            sqlite3VdbeAddOp2(
                v,
                160 as i32,
                ((unsafe { (*pFKey).isDeferred }) as u32) as i32,
                nIncr,
            )
        };
    }
    unsafe { sqlite3VdbeResolveLabel(v, iOk) };
    unsafe { sqlite3VdbeAddOp1(v, 124 as i32, iCur) };
}

/// Return an Expr object that refers to a memory register corresponding
/// to column iCol of table pTab.
///
/// regBase is the first of an array of register that contains the data
/// for pTab.  regBase itself holds the rowid.  regBase+1 holds the first
/// column.  regBase+2 holds the second column, and so forth.
///
/// # Arguments
///
/// * `pParse` - Parsing and code generating context
/// * `pTab` - The table whose content is at r[regBase]...
/// * `regBase` - Contents of table pTab
/// * `iCol` - Which column of pTab is desired
fn exprTableRegister(
    mut pParse: *mut Parse,
    mut pTab: *mut Table,
    mut regBase: i32,
    mut iCol: i16,
) -> *mut Expr {
    let mut pExpr: *mut Expr = unsafe { std::mem::zeroed() };
    let mut pCol: *mut Column = unsafe { std::mem::zeroed() };
    let mut zColl: *const i8 = unsafe { std::mem::zeroed() };
    let mut db: *mut sqlite3 = unsafe { (*pParse).db };
    pExpr = unsafe { sqlite3Expr(db, 176 as i32, std::ptr::null::<i8>()) };
    if pExpr != std::ptr::null_mut::<Expr>() {
        if (iCol as i32) >= (0 as i32) && (iCol as i32) != ((unsafe { (*pTab).iPKey }) as i32) {
            pCol = unsafe { unsafe { (*pTab).aCol }.offset((iCol as i32) as isize) };
            unsafe {
                (*pExpr).iTable = regBase
                    + ((unsafe { sqlite3TableColumnToStorage(pTab, iCol) }) as i32)
                    + (1 as i32);
            }
            unsafe {
                (*pExpr).affExpr = unsafe { (*pCol).affinity };
            }
            zColl = unsafe { sqlite3ColumnColl(pCol) };
            if zColl == std::ptr::null::<i8>() {
                zColl = (unsafe { (*unsafe { (*db).pDfltColl }).zName }) as *const i8;
            }
            pExpr = unsafe { sqlite3ExprAddCollateString(pParse as *const Parse, pExpr, zColl) };
        } else {
            unsafe {
                (*pExpr).iTable = regBase;
            }
            unsafe {
                (*pExpr).affExpr = (68 as i32) as i8;
            }
        }
    }
    return pExpr;
}

/// Return an Expr object that refers to column iCol of table pTab which
/// has cursor iCur.
///
/// # Arguments
///
/// * `db` - The database connection
/// * `pTab` - The table whose column is desired
/// * `iCursor` - The open cursor on the table
/// * `iCol` - The column that is wanted
fn exprTableColumn(
    mut db: *mut sqlite3,
    mut pTab: *mut Table,
    mut iCursor: i32,
    mut iCol: i16,
) -> *mut Expr {
    let mut pExpr: *mut Expr = unsafe { sqlite3Expr(db, 168 as i32, std::ptr::null::<i8>()) };
    if pExpr != std::ptr::null_mut::<Expr>() {
        0 as i32;
        unsafe {
            (*pExpr).y.pTab = pTab;
        }
        unsafe {
            (*pExpr).iTable = iCursor;
        }
        unsafe {
            (*pExpr).iColumn = iCol;
        }
    }
    return pExpr;
}

/// This function is called to generate code executed when a row is deleted
/// from the parent table of foreign key constraint pFKey and, if pFKey is
/// deferred, when a row is inserted into the same table. When generating
/// code for an SQL UPDATE operation, this function may be called twice -
/// once to "delete" the old row and once to "insert" the new row.
///
/// Parameter nIncr is passed -1 when inserting a row (as this may decrease
/// the number of FK violations in the db) or +1 when deleting one (as this
/// may increase the number of FK constraint problems).
///
/// The code generated by this function scans through the rows in the child
/// table that correspond to the parent table row being deleted or inserted.
/// For each child row found, one of the following actions is taken:
///
///   Operation | FK type   | Action taken
///   DELETE      immediate   Increment the "immediate constraint counter".
///
///   INSERT      immediate   Decrement the "immediate constraint counter".
///
///   DELETE      deferred    Increment the "deferred constraint counter".
///
///   INSERT      deferred    Decrement the "deferred constraint counter".
///
/// These operations are identified in the comment at the top of this file
/// (fkey.c) as "I.2" and "D.2".
///
/// # Arguments
///
/// * `pParse` - Parse context
/// * `pSrc` - The child table to be scanned
/// * `pTab` - The parent table
/// * `pIdx` - Index on parent covering the foreign key
/// * `pFKey` - The foreign key linking pSrc to pTab
/// * `aiCol` - Map from pIdx cols to child table cols
/// * `regData` - Parent row data starts here
/// * `nIncr` - Amount to increment deferred counter by
fn fkScanChildren(
    mut pParse: *mut Parse,
    mut pSrc: *mut SrcList,
    mut pTab: *mut Table,
    mut pIdx: *mut Index,
    mut pFKey: *mut FKey,
    mut aiCol: *mut i32,
    mut regData: i32,
    mut nIncr: i32,
) {
    let mut db: *mut sqlite3 = unsafe { (*pParse).db }; // Database handle
    let mut i: i32 = 0 as i32; // Iterator variable
    let mut pWhere: *mut Expr = std::ptr::null_mut::<Expr>(); // WHERE clause to scan with
    let mut sNameContext: NameContext = unsafe { std::mem::zeroed() }; // Context used to resolve WHERE clause
    let mut pWInfo: *mut WhereInfo = unsafe { std::mem::zeroed() }; // Context used by sqlite3WhereXXX()
    let mut iFkIfZero: i32 = 0 as i32; // Address of OP_FkIfZero
    let mut v: *mut Vdbe = unsafe { sqlite3GetVdbe(pParse) };
    0 as i32;
    0 as i32;
    0 as i32;
    0 as i32;
    if nIncr < (0 as i32) {
        iFkIfZero = unsafe {
            sqlite3VdbeAddOp2(
                v,
                60 as i32,
                ((unsafe { (*pFKey).isDeferred }) as u32) as i32,
                0 as i32,
            )
        };
        {}
    }
    // Create an Expr object representing an SQL expression like:
    //
    //   <parent-key1> = <child-key1> AND <parent-key2> = <child-key2> ...
    //
    // The collation sequence used for the comparison should be that of
    // the parent key columns. The affinity of the parent key column should
    // be applied to each child key value before the comparison takes place.
    i = 0 as i32;
    '__slate_break_798: loop {
        if !(i < unsafe { (*pFKey).nCol }) {
            break;
        }
        let mut pLeft: *mut Expr = unsafe { std::mem::zeroed() }; // Value from parent table row
        let mut pRight: *mut Expr = unsafe { std::mem::zeroed() }; // Column ref to child table
        let mut pEq: *mut Expr = unsafe { std::mem::zeroed() }; // Expression (pLeft = pRight)
        let mut iCol: i16 = 0 as i16; // Index of column in child table
        let mut zCol: *const i8 = unsafe { std::mem::zeroed() }; // Name of column in child table
        iCol = (if pIdx != std::ptr::null_mut::<Index>() {
            (unsafe { *unsafe { unsafe { (*pIdx).aiColumn }.offset(i as isize) } }) as i32
        } else {
            -(1 as i32)
        }) as i16;
        pLeft = exprTableRegister(pParse, pTab, regData, iCol);
        iCol = (if aiCol != std::ptr::null_mut::<i32>() {
            unsafe { *unsafe { aiCol.offset(i as isize) } }
        } else {
            unsafe {
                (*unsafe {
                    unsafe { std::ptr::addr_of_mut!((*pFKey).aCol) as *mut sColMap }
                        .offset((0 as i32) as isize)
                })
                .iFrom
            }
        }) as i16;
        0 as i32;
        zCol = (unsafe {
            (*unsafe {
                unsafe { (*unsafe { (*pFKey).pFrom }).aCol }.offset((iCol as i32) as isize)
            })
            .zCnName
        }) as *const i8;
        pRight = unsafe { sqlite3Expr(db, 60 as i32, zCol) };
        pEq = unsafe { sqlite3PExpr(pParse, 54 as i32, pLeft, pRight) };
        pWhere = unsafe { sqlite3ExprAnd(pParse, pWhere, pEq) };
        let __v864: i32 = i;
        let __v865: i32 = __v864 + (1 as i32);
        i = __v865;
    }
    // If the child table is the same as the parent table, then add terms
    // to the WHERE clause that prevent this entry from being scanned.
    // The added WHERE clause terms are like this:
    //
    //     $current_rowid!=rowid
    //     NOT( $current_a==a AND $current_b==b AND ... )
    //
    // The first form is used for rowid tables.  The second form is used
    // for WITHOUT ROWID tables. In the second form, the *parent* key is
    // (a,b,...). Either the parent or primary key could be used to
    // uniquely identify the current row, but the parent key is more convenient
    // as the required values have already been loaded into registers
    // by the caller.
    if pTab == unsafe { (*pFKey).pFrom } && nIncr > (0 as i32) {
        let mut pNe: *mut Expr = unsafe { std::mem::zeroed() }; // Expression (pLeft != pRight)
        let mut pLeft: *mut Expr = unsafe { std::mem::zeroed() }; // Value from parent table row
        let mut pRight: *mut Expr = unsafe { std::mem::zeroed() }; // Column ref to child table
        if (unsafe { (*pTab).tabFlags }) & ((128 as i32) as u32) == ((0 as i32) as u32) {
            pLeft = exprTableRegister(pParse, pTab, regData, -(1 as i32) as i16);
            pRight = exprTableColumn(
                db,
                pTab,
                unsafe {
                    (*unsafe {
                        unsafe { std::ptr::addr_of_mut!((*pSrc).a) as *mut SrcItem }
                            .offset((0 as i32) as isize)
                    })
                    .iCursor
                },
                -(1 as i32) as i16,
            );
            pNe = unsafe { sqlite3PExpr(pParse, 53 as i32, pLeft, pRight) };
        } else {
            let mut pEq: *mut Expr = unsafe { std::mem::zeroed() };
            let mut pAll: *mut Expr = std::ptr::null_mut::<Expr>();
            0 as i32;
            i = 0 as i32;
            '__slate_break_799: loop {
                if !(i < (((unsafe { (*pIdx).nKeyCol }) as u32) as i32)) {
                    break;
                }
                let mut iCol: i16 =
                    unsafe { *unsafe { unsafe { (*pIdx).aiColumn }.offset(i as isize) } };
                0 as i32;
                pLeft = exprTableRegister(pParse, pTab, regData, iCol);
                pRight = unsafe {
                    sqlite3Expr(
                        db,
                        60 as i32,
                        (unsafe {
                            (*unsafe { unsafe { (*pTab).aCol }.offset((iCol as i32) as isize) })
                                .zCnName
                        }) as *const i8,
                    )
                };
                pEq = unsafe { sqlite3PExpr(pParse, 45 as i32, pLeft, pRight) };
                pAll = unsafe { sqlite3ExprAnd(pParse, pAll, pEq) };
                let __v866: i32 = i;
                let __v867: i32 = __v866 + (1 as i32);
                i = __v867;
            }
            pNe = unsafe { sqlite3PExpr(pParse, 19 as i32, pAll, std::ptr::null_mut::<Expr>()) };
        }
        pWhere = unsafe { sqlite3ExprAnd(pParse, pWhere, pNe) };
    }
    // Resolve the references in the WHERE clause.
    unsafe {
        memset(
            std::ptr::addr_of_mut!(sNameContext) as *mut (),
            0 as i32,
            56 as u64,
        )
    };
    sNameContext.pSrcList = pSrc;
    sNameContext.pParse = pParse;
    unsafe { sqlite3ResolveExprNames(std::ptr::addr_of_mut!(sNameContext), pWhere) };
    // Create VDBE to loop through the entries in pSrc that match the WHERE
    // clause. For each row found, increment either the deferred or immediate
    // foreign key constraint counter.
    if (unsafe { (*pParse).nErr }) == (0 as i32) {
        pWInfo = unsafe {
            sqlite3WhereBegin(
                pParse,
                pSrc,
                pWhere,
                std::ptr::null_mut::<ExprList>(),
                std::ptr::null_mut::<ExprList>(),
                std::ptr::null_mut::<Select>(),
                ((0 as i32) as i16) as u16,
                0 as i32,
            )
        };
        unsafe {
            sqlite3VdbeAddOp2(
                v,
                160 as i32,
                ((unsafe { (*pFKey).isDeferred }) as u32) as i32,
                nIncr,
            )
        };
        if pWInfo != std::ptr::null_mut::<WhereInfo>() {
            unsafe { sqlite3WhereEnd(pWInfo) };
        }
    }
    // Clean up the WHERE clause constructed above.
    unsafe { sqlite3ExprDelete(db, pWhere) };
    if iFkIfZero != (0 as i32) {
        unsafe { sqlite3VdbeJumpHereOrPopInst(v, iFkIfZero) };
    }
}

/// This function returns a linked list of FKey objects (connected by
/// FKey.pNextTo) holding all children of table pTab.  For example,
/// given the following schema:
///
///   CREATE TABLE t1(a PRIMARY KEY);
///   CREATE TABLE t2(b REFERENCES t1(a);
///
/// Calling this function with table "t1" as an argument returns a pointer
/// to the FKey structure representing the foreign key constraint on table
/// "t2". Calling this function with "t2" as the argument would return a
/// NULL pointer (as there are no FK constraints for which t2 is the parent
/// table).
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3FkReferences(mut pTab: *mut Table) -> *mut FKey {
    return (unsafe {
        sqlite3HashFind(
            (unsafe { std::ptr::addr_of_mut!((*unsafe { (*pTab).pSchema }).fkeyHash) })
                as *const Hash,
            (unsafe { (*pTab).zName }) as *const i8,
        )
    }) as *mut FKey;
}

/// The second argument is a Trigger structure allocated by the
/// fkActionTrigger() routine. This function deletes the Trigger structure
/// and all of its sub-components.
///
/// The Trigger structure or any of its sub-components may be allocated from
/// the lookaside buffer belonging to database handle dbMem.
fn fkTriggerDelete(mut dbMem: *mut sqlite3, mut p: *mut Trigger) {
    if p != std::ptr::null_mut::<Trigger>() {
        let mut pStep: *mut TriggerStep = unsafe { (*p).step_list };
        unsafe { sqlite3SrcListDelete(dbMem, unsafe { (*pStep).pSrc }) };
        unsafe { sqlite3ExprDelete(dbMem, unsafe { (*pStep).pWhere }) };
        unsafe { sqlite3ExprListDelete(dbMem, unsafe { (*pStep).pExprList }) };
        unsafe { sqlite3SelectDelete(dbMem, unsafe { (*pStep).pSelect }) };
        unsafe { sqlite3ExprDelete(dbMem, unsafe { (*p).pWhen }) };
        unsafe { sqlite3DbFree(dbMem, p as *mut ()) };
    }
}

/// Clear the apTrigger[] cache of CASCADE triggers for all foreign keys
/// in a particular database.  This needs to happen when the schema
/// changes.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3FkClearTriggerCache(mut db: *mut sqlite3, mut iDb: i32) {
    let mut k: *mut HashElem = unsafe { std::mem::zeroed() };
    let mut pHash: *mut Hash = unsafe {
        std::ptr::addr_of_mut!(
            (*unsafe { (*unsafe { unsafe { (*db).aDb }.offset(iDb as isize) }).pSchema }).tblHash
        )
    };
    k = unsafe { (*pHash).first };
    '__slate_break_800: while k != std::ptr::null_mut::<HashElem>() {
        let mut pTab: *mut Table = (unsafe { (*k).data }) as *mut Table;
        let mut pFKey: *mut FKey = unsafe { std::mem::zeroed() };
        if !((((unsafe { (*pTab).eTabType }) as u32) as i32) == (0 as i32)) {
        } else {
            pFKey = unsafe { (*pTab).u.tab.pFKey };
            '__slate_break_801: while pFKey != std::ptr::null_mut::<FKey>() {
                fkTriggerDelete(db, unsafe {
                    *unsafe {
                        unsafe { (*pFKey).apTrigger.as_mut_ptr() as *mut *mut Trigger }
                            .offset((0 as i32) as isize)
                    }
                });
                unsafe {
                    *unsafe {
                        unsafe { (*pFKey).apTrigger.as_mut_ptr() as *mut *mut Trigger }
                            .offset((0 as i32) as isize)
                    } = std::ptr::null_mut::<Trigger>();
                }
                fkTriggerDelete(db, unsafe {
                    *unsafe {
                        unsafe { (*pFKey).apTrigger.as_mut_ptr() as *mut *mut Trigger }
                            .offset((1 as i32) as isize)
                    }
                });
                unsafe {
                    *unsafe {
                        unsafe { (*pFKey).apTrigger.as_mut_ptr() as *mut *mut Trigger }
                            .offset((1 as i32) as isize)
                    } = std::ptr::null_mut::<Trigger>();
                }
                pFKey = unsafe { (*pFKey).pNextFrom };
            }
        }
        k = unsafe { (*k).next };
    }
}

/// This function is called to generate code that runs when table pTab is
/// being dropped from the database. The SrcList passed as the second argument
/// to this function contains a single entry guaranteed to resolve to
/// table pTab.
///
/// Normally, no code is required. However, if either
///
///   (a) The table is the parent table of a FK constraint, or
///   (b) The table is the child table of a deferred FK constraint and it is
///       determined at runtime that there are outstanding deferred FK
///       constraint violations in the database,
///
/// then the equivalent of "DELETE FROM <tbl>" is executed before dropping
/// the table from the database. Triggers are disabled while running this
/// DELETE, but foreign key actions are not.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3FkDropTable(
    mut pParse: *mut Parse,
    mut pName: *mut SrcList,
    mut pTab: *mut Table,
) {
    let mut db: *mut sqlite3 = unsafe { (*pParse).db };
    if (unsafe { (*db).flags }) & (((16384 as i32) as i64) as u64) != (0 as u64)
        && (((unsafe { (*pTab).eTabType }) as u32) as i32) == (0 as i32)
    {
        let mut iSkip: i32 = 0 as i32;
        let mut v: *mut Vdbe = unsafe { sqlite3GetVdbe(pParse) };
        0 as i32; // VDBE has already been allocated
        0 as i32;
        if sqlite3FkReferences(pTab) == std::ptr::null_mut::<FKey>() {
            // Search for a deferred foreign key constraint for which this table
            // is the child table. If one cannot be found, return without
            // generating any VDBE code. If one can be found, then jump over
            // the entire DELETE if there are no outstanding deferred constraints
            // when this statement is run.
            let mut p: *mut FKey = unsafe { std::mem::zeroed() };
            p = unsafe { (*pTab).u.tab.pFKey };
            '__slate_break_802: while p != std::ptr::null_mut::<FKey>() {
                if (unsafe { (*p).isDeferred }) != (0 as u8)
                    || (unsafe { (*db).flags }) & (((524288 as i32) as i64) as u64) != (0 as u64)
                {
                    break '__slate_break_802;
                }
                p = unsafe { (*p).pNextFrom };
            }
            if !(p != std::ptr::null_mut::<FKey>()) {
                return;
            }
            iSkip = unsafe { sqlite3VdbeMakeLabel(pParse) };
            unsafe { sqlite3VdbeAddOp2(v, 60 as i32, 1 as i32, iSkip) };
            {}
        }
        unsafe {
            (*pParse)
                .__slate_bits_0
                .__set_disableTriggers((1 as i32) as u32);
        }
        unsafe {
            sqlite3DeleteFrom(
                pParse,
                unsafe { sqlite3SrcListDup(db, pName as *const SrcList, 0 as i32) },
                std::ptr::null_mut::<Expr>(),
                std::ptr::null_mut::<ExprList>(),
                std::ptr::null_mut::<Expr>(),
            )
        };
        unsafe {
            (*pParse)
                .__slate_bits_0
                .__set_disableTriggers((0 as i32) as u32);
        }
        // If the DELETE has generated immediate foreign key constraint
        // violations, halt the VDBE and return an error at this point, before
        // any modifications to the schema are made. This is because statement
        // transactions are not able to rollback schema changes.
        //
        // If the SQLITE_DeferFKs flag is set, then this is not required, as
        // the statement transaction will not be rolled back even if FK
        // constraints are violated.
        if (unsafe { (*db).flags }) & (((524288 as i32) as i64) as u64)
            == (((0 as i32) as i64) as u64)
        {
            {}
            unsafe {
                sqlite3VdbeAddOp2(
                    v,
                    60 as i32,
                    0 as i32,
                    (unsafe { sqlite3VdbeCurrentAddr(v) }) + (2 as i32),
                )
            };
            {}
            unsafe {
                sqlite3HaltConstraint(
                    pParse,
                    (19 as i32) | (3 as i32) << (8 as i32),
                    2 as i32,
                    std::ptr::null_mut::<i8>(),
                    -(1 as i32) as i8,
                    ((4 as i32) as i8) as u8,
                )
            };
        }
        if iSkip != (0 as i32) {
            unsafe { sqlite3VdbeResolveLabel(v, iSkip) };
        }
    }
}

/// The second argument points to an FKey object representing a foreign key
/// for which pTab is the child table. An UPDATE statement against pTab
/// is currently being processed. For each column of the table that is
/// actually updated, the corresponding element in the aChange[] array
/// is zero or greater (if a column is unmodified the corresponding element
/// is set to -1). If the rowid column is modified by the UPDATE statement
/// the bChngRowid argument is non-zero.
///
/// This function returns true if any of the columns that are part of the
/// child key for FK constraint *p are modified.
///
/// # Arguments
///
/// * `pTab` - Table being updated
/// * `p` - Foreign key for which pTab is the child
/// * `aChange` - Array indicating modified columns
/// * `bChngRowid` - True if rowid is modified by this update
fn fkChildIsModified(
    mut pTab: *mut Table,
    mut p: *mut FKey,
    mut aChange: *mut i32,
    mut bChngRowid: i32,
) -> i32 {
    let mut i: i32 = 0 as i32;
    i = 0 as i32;
    '__slate_break_803: loop {
        if !(i < unsafe { (*p).nCol }) {
            break;
        }
        let mut iChildKey: i32 = unsafe {
            (*unsafe {
                unsafe { std::ptr::addr_of_mut!((*p).aCol) as *mut sColMap }.offset(i as isize)
            })
            .iFrom
        };
        if (unsafe { *unsafe { aChange.offset(iChildKey as isize) } }) >= (0 as i32) {
            return 1 as i32;
        }
        if iChildKey == ((unsafe { (*pTab).iPKey }) as i32) && bChngRowid != (0 as i32) {
            return 1 as i32;
        }
        let __v868: i32 = i;
        let __v869: i32 = __v868 + (1 as i32);
        i = __v869;
    }
    return 0 as i32;
}

/// The second argument points to an FKey object representing a foreign key
/// for which pTab is the parent table. An UPDATE statement against pTab
/// is currently being processed. For each column of the table that is
/// actually updated, the corresponding element in the aChange[] array
/// is zero or greater (if a column is unmodified the corresponding element
/// is set to -1). If the rowid column is modified by the UPDATE statement
/// the bChngRowid argument is non-zero.
///
/// This function returns true if any of the columns that are part of the
/// parent key for FK constraint *p are modified.
fn fkParentIsModified(
    mut pTab: *mut Table,
    mut p: *mut FKey,
    mut aChange: *mut i32,
    mut bChngRowid: i32,
) -> i32 {
    let mut i: i32 = 0 as i32;
    i = 0 as i32;
    '__slate_break_804: loop {
        if !(i < unsafe { (*p).nCol }) {
            break;
        }
        let mut zKey: *mut i8 = unsafe {
            (*unsafe {
                unsafe { std::ptr::addr_of_mut!((*p).aCol) as *mut sColMap }.offset(i as isize)
            })
            .zCol
        };
        let mut iKey: i32 = 0 as i32;
        iKey = 0 as i32;
        '__slate_break_805: loop {
            if !(iKey < ((unsafe { (*pTab).nCol }) as i32)) {
                break;
            }
            if (unsafe { *unsafe { aChange.offset(iKey as isize) } }) >= (0 as i32)
                || iKey == ((unsafe { (*pTab).iPKey }) as i32) && bChngRowid != (0 as i32)
            {
                let mut pCol: *mut Column =
                    unsafe { unsafe { (*pTab).aCol }.offset(iKey as isize) };
                if zKey != std::ptr::null_mut::<i8>() {
                    if (0 as i32)
                        == unsafe {
                            sqlite3StrICmp(
                                (unsafe { (*pCol).zCnName }) as *const i8,
                                zKey as *const i8,
                            )
                        }
                    {
                        return 1 as i32;
                    }
                } else {
                    if (((unsafe { (*pCol).colFlags }) as u32) as i32) & (1 as i32) != (0 as i32) {
                        return 1 as i32;
                    }
                }
            }
            let __v872: i32 = iKey;
            let __v873: i32 = __v872 + (1 as i32);
            iKey = __v873;
        }
        let __v870: i32 = i;
        let __v871: i32 = __v870 + (1 as i32);
        i = __v871;
    }
    return 0 as i32;
}

/// Return true if the parser passed as the first argument is being
/// used to code a trigger that is really a "SET NULL" action belonging
/// to trigger pFKey.
fn isSetNullAction(mut pParse: *mut Parse, mut pFKey: *mut FKey) -> i32 {
    let mut pTop: *mut Parse = if (unsafe { (*pParse).pToplevel }) != std::ptr::null_mut::<Parse>()
    {
        unsafe { (*pParse).pToplevel }
    } else {
        pParse
    };
    if (unsafe { (*pTop).pTriggerPrg }) != std::ptr::null_mut::<TriggerPrg>() {
        let mut p: *mut Trigger = unsafe { (*unsafe { (*pTop).pTriggerPrg }).pTrigger };
        if p == unsafe {
            *unsafe {
                unsafe { (*pFKey).apTrigger.as_mut_ptr() as *mut *mut Trigger }
                    .offset((0 as i32) as isize)
            }
        } && (((unsafe {
            *unsafe {
                unsafe { (*pFKey).aAction.as_mut_ptr() as *mut u8 }.offset((0 as i32) as isize)
            }
        }) as u32) as i32)
            == (8 as i32)
            || p == unsafe {
                *unsafe {
                    unsafe { (*pFKey).apTrigger.as_mut_ptr() as *mut *mut Trigger }
                        .offset((1 as i32) as isize)
                }
            } && (((unsafe {
                *unsafe {
                    unsafe { (*pFKey).aAction.as_mut_ptr() as *mut u8 }.offset((1 as i32) as isize)
                }
            }) as u32) as i32)
                == (8 as i32)
        {
            0 as i32;
            return 1 as i32;
        }
    }
    return 0 as i32;
}

/// This function is called when inserting, deleting or updating a row of
/// table pTab to generate VDBE code to perform foreign key constraint
/// processing for the operation.
///
/// For a DELETE operation, parameter regOld is passed the index of the
/// first register in an array of (pTab->nCol+1) registers containing the
/// rowid of the row being deleted, followed by each of the column values
/// of the row being deleted, from left to right. Parameter regNew is passed
/// zero in this case.
///
/// For an INSERT operation, regOld is passed zero and regNew is passed the
/// first register of an array of (pTab->nCol+1) registers containing the new
/// row data.
///
/// For an UPDATE operation, this function is called twice. Once before
/// the original record is deleted from the table using the calling convention
/// described for DELETE. Then again after the original record is deleted
/// but before the new record is inserted using the INSERT convention.
///
/// # Arguments
///
/// * `pParse` - Parse context
/// * `pTab` - Row is being deleted from this table
/// * `regOld` - Previous row data is stored here
/// * `regNew` - New row data is stored here
/// * `aChange` - Array indicating UPDATEd columns (or 0)
/// * `bChngRowid` - True if rowid is UPDATEd
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3FkCheck(
    mut pParse: *mut Parse,
    mut pTab: *mut Table,
    mut regOld: i32,
    mut regNew: i32,
    mut aChange: *mut i32,
    mut bChngRowid: i32,
) {
    let mut db: *mut sqlite3 = unsafe { (*pParse).db }; // Database handle
    let mut pFKey: *mut FKey = unsafe { std::mem::zeroed() }; // Used to iterate through FKs
    let mut iDb: i32 = 0 as i32; // Index of database containing pTab
    let mut zDb: *const i8 = unsafe { std::mem::zeroed() }; // Name of database containing pTab
    let mut isIgnoreErrors: i32 =
        (unsafe { (*pParse).__slate_bits_0.__get_disableTriggers() }) as i32;
    // Exactly one of regOld and regNew should be non-zero.
    0 as i32;
    // If foreign-keys are disabled, this function is a no-op.
    if (unsafe { (*db).flags }) & (((16384 as i32) as i64) as u64) == (((0 as i32) as i64) as u64) {
        return;
    }
    if !((((unsafe { (*pTab).eTabType }) as u32) as i32) == (0 as i32)) {
        return;
    }
    iDb = unsafe { sqlite3SchemaToIndex(db, unsafe { (*pTab).pSchema }) };
    0 as i32;
    zDb =
        (unsafe { (*unsafe { unsafe { (*db).aDb }.offset(iDb as isize) }).zDbSName }) as *const i8;
    // Loop through all the foreign key constraints for which pTab is the
    // child table (the table that the foreign key definition is part of).
    pFKey = unsafe { (*pTab).u.tab.pFKey };
    '__slate_break_806: while pFKey != std::ptr::null_mut::<FKey>() {
        let mut pTo: *mut Table = unsafe { std::mem::zeroed() }; // Parent table of foreign key pFKey
        let mut pIdx: *mut Index = std::ptr::null_mut::<Index>(); // Index on key columns in pTo
        let mut aiFree: *mut i32 = std::ptr::null_mut::<i32>();
        let mut aiCol: *mut i32 = unsafe { std::mem::zeroed() };
        let mut iCol: i32 = 0 as i32;
        let mut i: i32 = 0 as i32;
        let mut bIgnore: i32 = 0 as i32;
        let __v823: bool;
        if aChange != std::ptr::null_mut::<i32>() {
            __v823 = (unsafe {
                sqlite3_stricmp(
                    (unsafe { (*pTab).zName }) as *const i8,
                    (unsafe { (*pFKey).zTo }) as *const i8,
                )
            }) != (0 as i32);
        } else {
            __v823 = false as bool;
        }
        let __v824: bool;
        if __v823 {
            __v824 = fkChildIsModified(pTab, pFKey, aChange, bChngRowid) == (0 as i32);
        } else {
            __v824 = false as bool;
        }
        if __v824 {
        } else {
            // Find the parent table of this foreign key. Also find a unique index
            // on the parent key columns in the parent table. If either of these
            // schema items cannot be located, set an error in pParse and return
            // early.
            if ((unsafe { (*pParse).__slate_bits_0.__get_disableTriggers() }) as i32) != (0 as i32)
            {
                pTo = unsafe { sqlite3FindTable(db, (unsafe { (*pFKey).zTo }) as *const i8, zDb) };
            } else {
                pTo = unsafe {
                    sqlite3LocateTable(
                        pParse,
                        (0 as i32) as u32,
                        (unsafe { (*pFKey).zTo }) as *const i8,
                        zDb,
                    )
                };
            }
            let __v825: bool;
            if !(pTo != std::ptr::null_mut::<Table>()) {
                __v825 = true as bool;
            } else {
                __v825 = sqlite3FkLocateIndex(
                    pParse,
                    pTo,
                    pFKey,
                    std::ptr::addr_of_mut!(pIdx),
                    std::ptr::addr_of_mut!(aiFree),
                ) != (0 as i32);
            }
            if __v825 {
                0 as i32;
                if !(isIgnoreErrors != (0 as i32)) || (unsafe { (*db).mallocFailed }) != (0 as u8) {
                    return;
                }
                if pTo == std::ptr::null_mut::<Table>() {
                    // If isIgnoreErrors is true, then a table is being dropped. In this
                    // case SQLite runs a "DELETE FROM xxx" on the table being dropped
                    // before actually dropping it in order to check FK constraints.
                    // If the parent table of an FK constraint on the current table is
                    // missing, behave as if it is empty. i.e. decrement the relevant
                    // FK counter for each row of the current table with non-NULL keys.
                    let mut v: *mut Vdbe = unsafe { sqlite3GetVdbe(pParse) };
                    let mut iJump: i32 = (unsafe { sqlite3VdbeCurrentAddr(v) })
                        + unsafe { (*pFKey).nCol }
                        + (1 as i32);
                    i = 0 as i32;
                    '__slate_break_807: loop {
                        if !(i < unsafe { (*pFKey).nCol }) {
                            break;
                        }
                        let mut iFromCol: i32 = 0 as i32;
                        let mut iReg: i32 = 0 as i32;
                        iFromCol = unsafe {
                            (*unsafe {
                                unsafe { std::ptr::addr_of_mut!((*pFKey).aCol) as *mut sColMap }
                                    .offset(i as isize)
                            })
                            .iFrom
                        };
                        iReg = ((unsafe {
                            sqlite3TableColumnToStorage(unsafe { (*pFKey).pFrom }, iFromCol as i16)
                        }) as i32)
                            + regOld
                            + (1 as i32);
                        unsafe { sqlite3VdbeAddOp2(v, 51 as i32, iReg, iJump) };
                        {}
                        let __v826: i32 = i;
                        let __v827: i32 = __v826 + (1 as i32);
                        i = __v827;
                    }
                    unsafe {
                        sqlite3VdbeAddOp2(
                            v,
                            160 as i32,
                            ((unsafe { (*pFKey).isDeferred }) as u32) as i32,
                            -(1 as i32),
                        )
                    };
                }
            } else {
                0 as i32;
                if aiFree != std::ptr::null_mut::<i32>() {
                    aiCol = aiFree;
                } else {
                    iCol = unsafe {
                        (*unsafe {
                            unsafe { std::ptr::addr_of_mut!((*pFKey).aCol) as *mut sColMap }
                                .offset((0 as i32) as isize)
                        })
                        .iFrom
                    };
                    aiCol = std::ptr::addr_of_mut!(iCol);
                }
                i = 0 as i32;
                '__slate_break_808: loop {
                    if !(i < unsafe { (*pFKey).nCol }) {
                        break;
                    }
                    if (unsafe { *unsafe { aiCol.offset(i as isize) } })
                        == ((unsafe { (*pTab).iPKey }) as i32)
                    {
                        unsafe {
                            *unsafe { aiCol.offset(i as isize) } = -(1 as i32);
                        }
                    }
                    0 as i32;
                    // Request permission to read the parent key columns. If the
                    // authorization callback returns SQLITE_IGNORE, behave as if any
                    // values read from the parent table are NULL.
                    if (unsafe { (*db).xAuth }) != None {
                        let mut rcauth: i32 = 0 as i32;
                        let mut zCol: *mut i8 = unsafe {
                            (*unsafe {
                                unsafe { (*pTo).aCol }.offset(
                                    (if pIdx != std::ptr::null_mut::<Index>() {
                                        (unsafe {
                                            *unsafe {
                                                unsafe { (*pIdx).aiColumn }.offset(i as isize)
                                            }
                                        }) as i32
                                    } else {
                                        (unsafe { (*pTo).iPKey }) as i32
                                    }) as isize,
                                )
                            })
                            .zCnName
                        };
                        rcauth = unsafe {
                            sqlite3AuthReadCol(
                                pParse,
                                (unsafe { (*pTo).zName }) as *const i8,
                                zCol as *const i8,
                                iDb,
                            )
                        };
                        bIgnore = (rcauth == (2 as i32)) as i32;
                    }
                    let __v828: i32 = i;
                    let __v829: i32 = __v828 + (1 as i32);
                    i = __v829;
                }
                // Take a shared-cache advisory read-lock on the parent table. Allocate
                // a cursor to use to search the unique index on the parent key columns
                // in the parent table.
                unsafe {
                    sqlite3TableLock(
                        pParse,
                        iDb,
                        unsafe { (*pTo).tnum },
                        ((0 as i32) as i8) as u8,
                        (unsafe { (*pTo).zName }) as *const i8,
                    )
                };
                let __v830: *mut Parse = pParse;
                let __v831: i32 = unsafe { (*__v830).nTab };
                let __v832: i32 = __v831 + (1 as i32);
                unsafe {
                    (*__v830).nTab = __v832;
                }
                if regOld != (0 as i32) {
                    // A row is being removed from the child table. Search for the parent.
                    // If the parent does not exist, removing the child row resolves an
                    // outstanding foreign key constraint violation.
                    fkLookupParent(
                        pParse,
                        iDb,
                        pTo,
                        pIdx,
                        pFKey,
                        aiCol,
                        regOld,
                        -(1 as i32),
                        bIgnore,
                    );
                }
                let __v833: bool;
                if regNew != (0 as i32) {
                    __v833 = !(isSetNullAction(pParse, pFKey) != (0 as i32));
                } else {
                    __v833 = false as bool;
                }
                if __v833 {
                    // A row is being added to the child table. If a parent row cannot
                    // be found, adding the child row has violated the FK constraint.
                    //
                    // If this operation is being performed as part of a trigger program
                    // that is actually a "SET NULL" action belonging to this very
                    // foreign key, then omit this scan altogether. As all child key
                    // values are guaranteed to be NULL, it is not possible for adding
                    // this row to cause an FK violation.
                    fkLookupParent(
                        pParse, iDb, pTo, pIdx, pFKey, aiCol, regNew, 1 as i32, bIgnore,
                    );
                }
                unsafe { sqlite3DbFree(db, aiFree as *mut ()) };
            }
        }
        pFKey = unsafe { (*pFKey).pNextFrom };
    }
    // Loop through all the foreign key constraints that refer to this table.
    // (the "child" constraints)
    pFKey = sqlite3FkReferences(pTab);
    '__slate_break_809: while pFKey != std::ptr::null_mut::<FKey>() {
        let mut pIdx: *mut Index = std::ptr::null_mut::<Index>(); // Foreign key index for pFKey
        let mut pSrc: *mut SrcList = unsafe { std::mem::zeroed() };
        let mut aiCol: *mut i32 = std::ptr::null_mut::<i32>();
        let __v834: bool;
        if aChange != std::ptr::null_mut::<i32>() {
            __v834 = fkParentIsModified(pTab, pFKey, aChange, bChngRowid) == (0 as i32);
        } else {
            __v834 = false as bool;
        }
        if __v834 {
        } else {
            if !((unsafe { (*pFKey).isDeferred }) != (0 as u8))
                && !((unsafe { (*db).flags }) & (((524288 as i32) as i64) as u64) != (0 as u64))
                && !((unsafe { (*pParse).pToplevel }) != std::ptr::null_mut::<Parse>())
                && !((unsafe { (*pParse).isMultiWrite }) != (0 as u8))
            {
                0 as i32;
            // Inserting a single row into a parent table cannot cause (or fix)
            // an immediate foreign key violation. So do nothing in this case.
            } else {
                if sqlite3FkLocateIndex(
                    pParse,
                    pTab,
                    pFKey,
                    std::ptr::addr_of_mut!(pIdx),
                    std::ptr::addr_of_mut!(aiCol),
                ) != (0 as i32)
                {
                    if !(isIgnoreErrors != (0 as i32))
                        || (unsafe { (*db).mallocFailed }) != (0 as u8)
                    {
                        return;
                    }
                } else {
                    0 as i32;
                    // Create a SrcList structure containing the child table.  We need the
                    // child table as a SrcList for sqlite3WhereBegin()
                    pSrc = unsafe {
                        sqlite3SrcListAppend(
                            pParse,
                            std::ptr::null_mut::<SrcList>(),
                            std::ptr::null_mut::<Token>(),
                            std::ptr::null_mut::<Token>(),
                        )
                    };
                    if pSrc != std::ptr::null_mut::<SrcList>() {
                        let mut pItem: *mut SrcItem =
                            unsafe { std::ptr::addr_of_mut!((*pSrc).a) as *mut SrcItem };
                        unsafe {
                            (*pItem).pSTab = unsafe { (*pFKey).pFrom };
                        }
                        unsafe {
                            (*pItem).zName = unsafe { (*unsafe { (*pFKey).pFrom }).zName };
                        }
                        let __v835: *mut Table = unsafe { (*pItem).pSTab };
                        let __v836: u32 = unsafe { (*__v835).nTabRef };
                        let __v837: u32 = __v836.wrapping_add((1 as i32) as u32);
                        unsafe {
                            (*__v835).nTabRef = __v837;
                        }
                        let __v838: *mut Parse = pParse;
                        let __v839: i32 = unsafe { (*__v838).nTab };
                        let __v840: i32 = __v839 + (1 as i32);
                        unsafe {
                            (*__v838).nTab = __v840;
                        }
                        unsafe {
                            (*pItem).iCursor = __v839;
                        }
                        if regNew != (0 as i32) {
                            fkScanChildren(
                                pParse,
                                pSrc,
                                pTab,
                                pIdx,
                                pFKey,
                                aiCol,
                                regNew,
                                -(1 as i32),
                            );
                        }
                        if regOld != (0 as i32) {
                            let mut eAction: i32 = ((unsafe {
                                *unsafe {
                                    unsafe { (*pFKey).aAction.as_mut_ptr() as *mut u8 }.offset(
                                        ((aChange != std::ptr::null_mut::<i32>()) as i32) as isize,
                                    )
                                }
                            }) as u32) as i32;
                            if (unsafe { (*db).flags })
                                & (((8 as i32) as i64) as u64) << (32 as i32)
                                != (0 as u64)
                            {
                                eAction = 0 as i32;
                            }
                            fkScanChildren(
                                pParse, pSrc, pTab, pIdx, pFKey, aiCol, regOld, 1 as i32,
                            );
                            // If this is a deferred FK constraint, or a CASCADE or SET NULL
                            // action applies, then any foreign key violations caused by
                            // removing the parent key will be rectified by the action trigger.
                            // So do not set the "may-abort" flag in this case.
                            //
                            // Note 1: If the FK is declared "ON UPDATE CASCADE", then the
                            // may-abort flag will eventually be set on this statement anyway
                            // (when this function is called as part of processing the UPDATE
                            // within the action trigger).
                            //
                            // Note 2: At first glance it may seem like SQLite could simply omit
                            // all OP_FkCounter related scans when either CASCADE or SET NULL
                            // applies. The trouble starts if the CASCADE or SET NULL action
                            // trigger causes other triggers or action rules attached to the
                            // child table to fire. In these cases the fk constraint counters
                            // might be set incorrectly if any OP_FkCounter related scans are
                            // omitted.
                            if !((unsafe { (*pFKey).isDeferred }) != (0 as u8))
                                && eAction != (10 as i32)
                                && eAction != (8 as i32)
                            {
                                unsafe { sqlite3MayAbort(pParse) };
                            }
                        }
                        unsafe {
                            (*pItem).zName = std::ptr::null_mut::<i8>();
                        }
                        unsafe { sqlite3SrcListDelete(db, pSrc) };
                    }
                    unsafe { sqlite3DbFree(db, aiCol as *mut ()) };
                }
            }
        }
        pFKey = unsafe { (*pFKey).pNextTo };
    }
}

/// This function is called before generating code to update or delete a
/// row contained in table pTab.
///
/// # Arguments
///
/// * `pParse` - Parse context
/// * `pTab` - Table being modified
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3FkOldmask(mut pParse: *mut Parse, mut pTab: *mut Table) -> u32 {
    let mut mask: u32 = (0 as i32) as u32;
    if (unsafe { (*unsafe { (*pParse).db }).flags }) & (((16384 as i32) as i64) as u64)
        != (0 as u64)
        && (((unsafe { (*pTab).eTabType }) as u32) as i32) == (0 as i32)
    {
        let mut p: *mut FKey = unsafe { std::mem::zeroed() };
        let mut i: i32 = 0 as i32;
        p = unsafe { (*pTab).u.tab.pFKey };
        '__slate_break_810: while p != std::ptr::null_mut::<FKey>() {
            i = 0 as i32;
            '__slate_break_811: loop {
                if !(i < unsafe { (*p).nCol }) {
                    break;
                }
                let __v844: u32 = mask;
                let __v845: u32 = __v844
                    | if (unsafe {
                        (*unsafe {
                            unsafe { std::ptr::addr_of_mut!((*p).aCol) as *mut sColMap }
                                .offset(i as isize)
                        })
                        .iFrom
                    }) > (31 as i32)
                    {
                        4294967295 as u32
                    } else {
                        ((1 as i32) as u32)
                            << unsafe {
                                (*unsafe {
                                    unsafe { std::ptr::addr_of_mut!((*p).aCol) as *mut sColMap }
                                        .offset(i as isize)
                                })
                                .iFrom
                            }
                    };
                mask = __v845;
                let __v842: i32 = i;
                let __v843: i32 = __v842 + (1 as i32);
                i = __v843;
            }
            p = unsafe { (*p).pNextFrom };
        }
        p = sqlite3FkReferences(pTab);
        '__slate_break_812: while p != std::ptr::null_mut::<FKey>() {
            let mut pIdx: *mut Index = std::ptr::null_mut::<Index>();
            sqlite3FkLocateIndex(
                pParse,
                pTab,
                p,
                std::ptr::addr_of_mut!(pIdx),
                std::ptr::null_mut::<*mut i32>(),
            );
            if pIdx != std::ptr::null_mut::<Index>() {
                i = 0 as i32;
                '__slate_break_813: loop {
                    if !(i < (((unsafe { (*pIdx).nKeyCol }) as u32) as i32)) {
                        break;
                    }
                    0 as i32;
                    let __v848: u32 = mask;
                    let __v849: u32 = __v848
                        | if ((unsafe {
                            *unsafe { unsafe { (*pIdx).aiColumn }.offset(i as isize) }
                        }) as i32)
                            > (31 as i32)
                        {
                            4294967295 as u32
                        } else {
                            ((1 as i32) as u32)
                                << ((unsafe {
                                    *unsafe { unsafe { (*pIdx).aiColumn }.offset(i as isize) }
                                }) as i32)
                        };
                    mask = __v849;
                    let __v846: i32 = i;
                    let __v847: i32 = __v846 + (1 as i32);
                    i = __v847;
                }
            }
            p = unsafe { (*p).pNextTo };
        }
    }
    return mask;
}

/// This function is called before generating code to update or delete a
/// row contained in table pTab. If the operation is a DELETE, then
/// parameter aChange is passed a NULL value. For an UPDATE, aChange points
/// to an array of size N, where N is the number of columns in table pTab.
/// If the i'th column is not modified by the UPDATE, then the corresponding
/// entry in the aChange[] array is set to -1. If the column is modified,
/// the value is 0 or greater. Parameter chngRowid is set to true if the
/// UPDATE statement modifies the rowid fields of the table.
///
/// If any foreign key processing will be required, this function returns
/// non-zero. If there is no foreign key related processing, this function
/// returns zero.
///
/// For an UPDATE, this function returns 2 if:
///
///   * There are any FKs for which pTab is the child and the parent table
///     and any FK processing at all is required (even of a different FK), or
///
///   * the UPDATE modifies one or more parent keys for which the action is
///     not "NO ACTION" (i.e. is CASCADE, SET DEFAULT or SET NULL).
///
/// Or, assuming some other foreign key processing is required, 1.
///
/// # Arguments
///
/// * `pParse` - Parse context
/// * `pTab` - Table being modified
/// * `aChange` - Non-NULL for UPDATE operations
/// * `chngRowid` - True for UPDATE that affects rowid
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3FkRequired(
    mut pParse: *mut Parse,
    mut pTab: *mut Table,
    mut aChange: *mut i32,
    mut chngRowid: i32,
) -> i32 {
    let mut eRet: i32 = 1 as i32; // Value to return if bHaveFK is true
    let mut bHaveFK: i32 = 0 as i32; // If FK processing is required
    if (unsafe { (*unsafe { (*pParse).db }).flags }) & (((16384 as i32) as i64) as u64)
        != (0 as u64)
        && (((unsafe { (*pTab).eTabType }) as u32) as i32) == (0 as i32)
    {
        if !(aChange != std::ptr::null_mut::<i32>()) {
            // A DELETE operation. Foreign key processing is required if the
            // table in question is either the child or parent table for any
            // foreign key constraint.
            bHaveFK = (sqlite3FkReferences(pTab) != std::ptr::null_mut::<FKey>()
                || (unsafe { (*pTab).u.tab.pFKey }) != std::ptr::null_mut::<FKey>())
                as i32;
        } else {
            // This is an UPDATE. Foreign key processing is only required if the
            // operation modifies one or more child or parent key columns.
            let mut p: *mut FKey = unsafe { std::mem::zeroed() };
            // Check if any child key columns are being modified.
            p = unsafe { (*pTab).u.tab.pFKey };
            '__slate_break_814: while p != std::ptr::null_mut::<FKey>() {
                if fkChildIsModified(pTab, p, aChange, chngRowid) != (0 as i32) {
                    if (0 as i32)
                        == unsafe {
                            sqlite3_stricmp(
                                (unsafe { (*pTab).zName }) as *const i8,
                                (unsafe { (*p).zTo }) as *const i8,
                            )
                        }
                    {
                        eRet = 2 as i32;
                    }
                    bHaveFK = 1 as i32;
                }
                p = unsafe { (*p).pNextFrom };
            }
            // Check if any parent key columns are being modified.
            p = sqlite3FkReferences(pTab);
            '__slate_break_815: while p != std::ptr::null_mut::<FKey>() {
                if fkParentIsModified(pTab, p, aChange, chngRowid) != (0 as i32) {
                    if (unsafe { (*unsafe { (*pParse).db }).flags })
                        & (((8 as i32) as i64) as u64) << (32 as i32)
                        == (((0 as i32) as i64) as u64)
                        && (((unsafe {
                            *unsafe {
                                unsafe { (*p).aAction.as_mut_ptr() as *mut u8 }
                                    .offset((1 as i32) as isize)
                            }
                        }) as u32) as i32)
                            != (0 as i32)
                    {
                        return 2 as i32;
                    }
                    bHaveFK = 1 as i32;
                }
                p = unsafe { (*p).pNextTo };
            }
        }
    }
    return if bHaveFK != (0 as i32) {
        eRet
    } else {
        0 as i32
    };
}

/// This function is called when an UPDATE or DELETE operation is being
/// compiled on table pTab, which is the parent table of foreign-key pFKey.
/// If the current operation is an UPDATE, then the pChanges parameter is
/// passed a pointer to the list of columns being modified. If it is a
/// DELETE, pChanges is passed a NULL pointer.
///
/// It returns a pointer to a Trigger structure containing a trigger
/// equivalent to the ON UPDATE or ON DELETE action specified by pFKey.
/// If the action is "NO ACTION" then a NULL pointer is returned (these actions
/// require no special handling by the triggers sub-system, code for them is
/// created by fkScanChildren()).
///
/// For example, if pFKey is the foreign key and pTab is table "p" in
/// the following schema:
///
///   CREATE TABLE p(pk PRIMARY KEY);
///   CREATE TABLE c(ck REFERENCES p ON DELETE CASCADE);
///
/// then the returned trigger structure is equivalent to:
///
///   CREATE TRIGGER ... DELETE ON p BEGIN
///     DELETE FROM c WHERE ck = old.pk;
///   END;
///
/// The returned pointer is cached as part of the foreign key object. It
/// is eventually freed along with the rest of the foreign key object by
/// sqlite3FkDelete().
///
/// # Arguments
///
/// * `pParse` - Parse context
/// * `pTab` - Table being updated or deleted from
/// * `pFKey` - Foreign key to get action for
/// * `pChanges` - Change-list for UPDATE, NULL for DELETE
fn fkActionTrigger(
    mut pParse: *mut Parse,
    mut pTab: *mut Table,
    mut pFKey: *mut FKey,
    mut pChanges: *mut ExprList,
) -> *mut Trigger {
    let mut db: *mut sqlite3 = unsafe { (*pParse).db }; // Database handle
    let mut action: i32 = 0 as i32; // One of OE_None, OE_Cascade etc.
    let mut pTrigger: *mut Trigger = unsafe { std::mem::zeroed() }; // Trigger definition to return
    let mut iAction: i32 = (pChanges != std::ptr::null_mut::<ExprList>()) as i32; // 1 for UPDATE, 0 for DELETE
    action = ((unsafe {
        *unsafe { unsafe { (*pFKey).aAction.as_mut_ptr() as *mut u8 }.offset(iAction as isize) }
    }) as u32) as i32;
    if (unsafe { (*db).flags }) & (((8 as i32) as i64) as u64) << (32 as i32) != (0 as u64) {
        action = 0 as i32;
    }
    if action == (7 as i32)
        && (unsafe { (*db).flags }) & (((524288 as i32) as i64) as u64) != (0 as u64)
    {
        return std::ptr::null_mut::<Trigger>();
    }
    pTrigger = unsafe {
        *unsafe {
            unsafe { (*pFKey).apTrigger.as_mut_ptr() as *mut *mut Trigger }.offset(iAction as isize)
        }
    };
    if action != (0 as i32) && !(pTrigger != std::ptr::null_mut::<Trigger>()) {
        let mut zFrom: *const i8 = unsafe { std::mem::zeroed() }; // Name of child table
        let mut nFrom: i32 = 0 as i32; // Length in bytes of zFrom
        let mut pIdx: *mut Index = std::ptr::null_mut::<Index>(); // Parent key index for this FK
        let mut aiCol: *mut i32 = std::ptr::null_mut::<i32>(); // child table cols -> parent key cols
        let mut pStep: *mut TriggerStep = std::ptr::null_mut::<TriggerStep>(); // First (only) step of trigger program
        let mut pWhere: *mut Expr = std::ptr::null_mut::<Expr>(); // WHERE clause of trigger step
        let mut pList: *mut ExprList = std::ptr::null_mut::<ExprList>(); // Changes list if ON UPDATE CASCADE
        let mut pSelect: *mut Select = std::ptr::null_mut::<Select>(); // If RESTRICT, "SELECT RAISE(...)"
        let mut i: i32 = 0 as i32; // Iterator variable
        let mut pWhen: *mut Expr = std::ptr::null_mut::<Expr>(); // WHEN clause for the trigger
        if sqlite3FkLocateIndex(
            pParse,
            pTab,
            pFKey,
            std::ptr::addr_of_mut!(pIdx),
            std::ptr::addr_of_mut!(aiCol),
        ) != (0 as i32)
        {
            return std::ptr::null_mut::<Trigger>();
        }
        0 as i32;
        i = 0 as i32;
        '__slate_break_816: loop {
            if !(i < unsafe { (*pFKey).nCol }) {
                break;
            }
            let mut tOld: Token = Token {
                z: (b"old\0".as_ptr() as *mut i8) as *const i8,
                n: (3 as i32) as u32,
            }; // Literal "old" token
            let mut tNew: Token = Token {
                z: (b"new\0".as_ptr() as *mut i8) as *const i8,
                n: (3 as i32) as u32,
            }; // Literal "new" token
            let mut tFromCol: Token = unsafe { std::mem::zeroed() }; // Name of column in child table
            let mut tToCol: Token = unsafe { std::mem::zeroed() }; // Name of column in parent table
            let mut iFromCol: i32 = 0 as i32; // Idx of column in child table
            let mut pEq: *mut Expr = unsafe { std::mem::zeroed() }; // tFromCol = OLD.tToCol
            iFromCol = if aiCol != std::ptr::null_mut::<i32>() {
                unsafe { *unsafe { aiCol.offset(i as isize) } }
            } else {
                unsafe {
                    (*unsafe {
                        unsafe { std::ptr::addr_of_mut!((*pFKey).aCol) as *mut sColMap }
                            .offset((0 as i32) as isize)
                    })
                    .iFrom
                }
            };
            0 as i32;
            0 as i32;
            0 as i32;
            unsafe {
                sqlite3TokenInit(std::ptr::addr_of_mut!(tToCol), unsafe {
                    (*unsafe {
                        unsafe { (*pTab).aCol }.offset(
                            (if pIdx != std::ptr::null_mut::<Index>() {
                                (unsafe {
                                    *unsafe { unsafe { (*pIdx).aiColumn }.offset(i as isize) }
                                }) as i32
                            } else {
                                (unsafe { (*pTab).iPKey }) as i32
                            }) as isize,
                        )
                    })
                    .zCnName
                })
            };
            unsafe {
                sqlite3TokenInit(std::ptr::addr_of_mut!(tFromCol), unsafe {
                    (*unsafe {
                        unsafe { (*unsafe { (*pFKey).pFrom }).aCol }.offset(iFromCol as isize)
                    })
                    .zCnName
                })
            };
            // Create the expression "OLD.zToCol = zFromCol". It is important
            // that the "OLD.zToCol" term is on the LHS of the = operator, so
            // that the affinity and collation sequence associated with the
            // parent table are used for the comparison.
            pEq = unsafe {
                sqlite3PExpr(
                    pParse,
                    54 as i32,
                    unsafe {
                        sqlite3PExpr(
                            pParse,
                            142 as i32,
                            unsafe {
                                sqlite3ExprAlloc(
                                    db,
                                    60 as i32,
                                    std::ptr::addr_of_mut!(tOld) as *const Token,
                                    0 as i32,
                                )
                            },
                            unsafe {
                                sqlite3ExprAlloc(
                                    db,
                                    60 as i32,
                                    std::ptr::addr_of_mut!(tToCol) as *const Token,
                                    0 as i32,
                                )
                            },
                        )
                    },
                    unsafe {
                        sqlite3ExprAlloc(
                            db,
                            60 as i32,
                            std::ptr::addr_of_mut!(tFromCol) as *const Token,
                            0 as i32,
                        )
                    },
                )
            };
            pWhere = unsafe { sqlite3ExprAnd(pParse, pWhere, pEq) };
            // For ON UPDATE, construct the next term of the WHEN clause.
            // The final WHEN clause will be like this:
            //
            //    WHEN NOT(old.col1 IS new.col1 AND ... AND old.colN IS new.colN)
            if pChanges != std::ptr::null_mut::<ExprList>() {
                pEq = unsafe {
                    sqlite3PExpr(
                        pParse,
                        45 as i32,
                        unsafe {
                            sqlite3PExpr(
                                pParse,
                                142 as i32,
                                unsafe {
                                    sqlite3ExprAlloc(
                                        db,
                                        60 as i32,
                                        std::ptr::addr_of_mut!(tOld) as *const Token,
                                        0 as i32,
                                    )
                                },
                                unsafe {
                                    sqlite3ExprAlloc(
                                        db,
                                        60 as i32,
                                        std::ptr::addr_of_mut!(tToCol) as *const Token,
                                        0 as i32,
                                    )
                                },
                            )
                        },
                        unsafe {
                            sqlite3PExpr(
                                pParse,
                                142 as i32,
                                unsafe {
                                    sqlite3ExprAlloc(
                                        db,
                                        60 as i32,
                                        std::ptr::addr_of_mut!(tNew) as *const Token,
                                        0 as i32,
                                    )
                                },
                                unsafe {
                                    sqlite3ExprAlloc(
                                        db,
                                        60 as i32,
                                        std::ptr::addr_of_mut!(tToCol) as *const Token,
                                        0 as i32,
                                    )
                                },
                            )
                        },
                    )
                };
                pWhen = unsafe { sqlite3ExprAnd(pParse, pWhen, pEq) };
            }
            if action != (7 as i32)
                && (action != (10 as i32) || pChanges != std::ptr::null_mut::<ExprList>())
            {
                let mut pNew: *mut Expr = unsafe { std::mem::zeroed() };
                if action == (10 as i32) {
                    pNew = unsafe {
                        sqlite3PExpr(
                            pParse,
                            142 as i32,
                            unsafe {
                                sqlite3ExprAlloc(
                                    db,
                                    60 as i32,
                                    std::ptr::addr_of_mut!(tNew) as *const Token,
                                    0 as i32,
                                )
                            },
                            unsafe {
                                sqlite3ExprAlloc(
                                    db,
                                    60 as i32,
                                    std::ptr::addr_of_mut!(tToCol) as *const Token,
                                    0 as i32,
                                )
                            },
                        )
                    };
                } else {
                    if action == (9 as i32) {
                        let mut pCol: *mut Column = unsafe {
                            unsafe { (*unsafe { (*pFKey).pFrom }).aCol }.offset(iFromCol as isize)
                        };
                        let mut pDflt: *mut Expr = unsafe { std::mem::zeroed() };
                        if (((unsafe { (*pCol).colFlags }) as u32) as i32) & (96 as i32)
                            != (0 as i32)
                        {
                            {}
                            {}
                            pDflt = std::ptr::null_mut::<Expr>();
                        } else {
                            pDflt = unsafe { sqlite3ColumnExpr(unsafe { (*pFKey).pFrom }, pCol) };
                        }
                        if pDflt != std::ptr::null_mut::<Expr>() {
                            pNew = unsafe { sqlite3ExprDup(db, pDflt as *const Expr, 0 as i32) };
                        } else {
                            pNew = unsafe {
                                sqlite3ExprAlloc(
                                    db,
                                    122 as i32,
                                    std::ptr::null::<Token>(),
                                    0 as i32,
                                )
                            };
                        }
                    } else {
                        pNew = unsafe {
                            sqlite3ExprAlloc(db, 122 as i32, std::ptr::null::<Token>(), 0 as i32)
                        };
                    }
                }
                pList = unsafe { sqlite3ExprListAppend(pParse, pList, pNew) };
                unsafe {
                    sqlite3ExprListSetName(
                        pParse,
                        pList,
                        std::ptr::addr_of_mut!(tFromCol) as *const Token,
                        0 as i32,
                    )
                };
            }
            let __v874: i32 = i;
            let __v875: i32 = __v874 + (1 as i32);
            i = __v875;
        }
        unsafe { sqlite3DbFree(db, aiCol as *mut ()) };
        zFrom = (unsafe { (*unsafe { (*pFKey).pFrom }).zName }) as *const i8;
        nFrom = unsafe { sqlite3Strlen30(zFrom) };
        if action == (7 as i32) {
            let mut pSrc: *mut SrcList = unsafe { std::mem::zeroed() };
            let mut pRaise: *mut Expr = unsafe { std::mem::zeroed() };
            pRaise = unsafe {
                sqlite3Expr(
                    db,
                    118 as i32,
                    (b"FOREIGN KEY constraint failed\0".as_ptr() as *mut i8) as *const i8,
                )
            };
            let __v876: *mut Expr =
                unsafe { sqlite3PExpr(pParse, 72 as i32, pRaise, std::ptr::null_mut::<Expr>()) };
            pRaise = __v876;
            if pRaise != std::ptr::null_mut::<Expr>() {
                unsafe {
                    (*pRaise).affExpr = (2 as i32) as i8;
                }
            }
            pSrc = unsafe {
                sqlite3SrcListAppend(
                    pParse,
                    std::ptr::null_mut::<SrcList>(),
                    std::ptr::null_mut::<Token>(),
                    std::ptr::null_mut::<Token>(),
                )
            };
            if pSrc != std::ptr::null_mut::<SrcList>() {
                let mut pItem: *mut SrcItem = unsafe {
                    unsafe { std::ptr::addr_of_mut!((*pSrc).a) as *mut SrcItem }
                        .offset((0 as i32) as isize)
                };
                unsafe {
                    (*pItem).zName = unsafe { sqlite3DbStrDup(db, zFrom) };
                }
                unsafe {
                    (*pItem)
                        .fg
                        .__slate_bits_0
                        .__set_fixedSchema((1 as i32) as u32);
                }
                unsafe {
                    (*pItem).u4.pSchema = unsafe { (*pTab).pSchema };
                }
            }
            pSelect = unsafe {
                sqlite3SelectNew(
                    pParse,
                    unsafe {
                        sqlite3ExprListAppend(pParse, std::ptr::null_mut::<ExprList>(), pRaise)
                    },
                    pSrc,
                    pWhere,
                    std::ptr::null_mut::<ExprList>(),
                    std::ptr::null_mut::<Expr>(),
                    std::ptr::null_mut::<ExprList>(),
                    (0 as i32) as u32,
                    std::ptr::null_mut::<Expr>(),
                )
            };
            pWhere = std::ptr::null_mut::<Expr>();
        }
        // Disable lookaside memory allocation
        let __v877: *mut sqlite3 = db;
        let __v878: u32 = unsafe { (*__v877).lookaside.bDisable };
        let __v879: u32 = __v878.wrapping_add((1 as i32) as u32);
        unsafe {
            (*__v877).lookaside.bDisable = __v879;
        }
        unsafe {
            (*db).lookaside.sz = ((0 as i32) as i16) as u16;
        }
        pTrigger = (unsafe { sqlite3DbMallocZero(db, (72 as u64).wrapping_add(88 as u64)) })
            as *mut Trigger; // struct Trigger
        // Single step in trigger program
        if pTrigger != std::ptr::null_mut::<Trigger>() {
            let __v880: *mut TriggerStep =
                (unsafe { pTrigger.offset((1 as i32) as isize) }) as *mut TriggerStep;
            unsafe {
                (*pTrigger).step_list = __v880;
            }
            pStep = __v880;
            unsafe {
                (*pStep).pSrc = unsafe {
                    sqlite3SrcListAppend(
                        pParse,
                        std::ptr::null_mut::<SrcList>(),
                        std::ptr::null_mut::<Token>(),
                        std::ptr::null_mut::<Token>(),
                    )
                };
            }
            if (unsafe { (*pStep).pSrc }) != std::ptr::null_mut::<SrcList>() {
                let mut pItem: *mut SrcItem = unsafe {
                    unsafe { std::ptr::addr_of_mut!((*unsafe { (*pStep).pSrc }).a) as *mut SrcItem }
                        .offset((0 as i32) as isize)
                };
                unsafe {
                    (*pItem).zName = unsafe { sqlite3DbStrNDup(db, zFrom, (nFrom as i64) as u64) };
                }
                unsafe {
                    (*pItem).u4.pSchema = unsafe { (*pTab).pSchema };
                }
                unsafe {
                    (*pItem)
                        .fg
                        .__slate_bits_0
                        .__set_fixedSchema((1 as i32) as u32);
                }
            }
            unsafe {
                (*pStep).pWhere = unsafe { sqlite3ExprDup(db, pWhere as *const Expr, 1 as i32) };
            }
            unsafe {
                (*pStep).pExprList =
                    unsafe { sqlite3ExprListDup(db, pList as *const ExprList, 1 as i32) };
            }
            unsafe {
                (*pStep).pSelect =
                    unsafe { sqlite3SelectDup(db, pSelect as *const Select, 1 as i32) };
            }
            if pWhen != std::ptr::null_mut::<Expr>() {
                pWhen =
                    unsafe { sqlite3PExpr(pParse, 19 as i32, pWhen, std::ptr::null_mut::<Expr>()) };
                unsafe {
                    (*pTrigger).pWhen =
                        unsafe { sqlite3ExprDup(db, pWhen as *const Expr, 1 as i32) };
                }
            }
        }
        // Re-enable the lookaside buffer, if it was disabled earlier.
        let __v881: *mut sqlite3 = db;
        let __v882: u32 = unsafe { (*__v881).lookaside.bDisable };
        let __v883: u32 = __v882.wrapping_sub((1 as i32) as u32);
        unsafe {
            (*__v881).lookaside.bDisable = __v883;
        }
        unsafe {
            (*db).lookaside.sz = ((if (unsafe { (*db).lookaside.bDisable }) != (0 as u32) {
                0 as i32
            } else {
                ((unsafe { (*db).lookaside.szTrue }) as u32) as i32
            }) as i16) as u16;
        }
        unsafe { sqlite3ExprDelete(db, pWhere) };
        unsafe { sqlite3ExprDelete(db, pWhen) };
        unsafe { sqlite3ExprListDelete(db, pList) };
        unsafe { sqlite3SelectDelete(db, pSelect) };
        if (((unsafe { (*db).mallocFailed }) as u32) as i32) == (1 as i32) {
            fkTriggerDelete(db, pTrigger);
            return std::ptr::null_mut::<Trigger>();
        }
        0 as i32;
        0 as i32;
        // no break
        match action {
            7 => unsafe {
                (*pStep).op = ((139 as i32) as i8) as u8;
            },
            10 => {
                if !(pChanges != std::ptr::null_mut::<ExprList>()) {
                    unsafe {
                        (*pStep).op = ((129 as i32) as i8) as u8;
                    }
                } else {
                    // no break
                    {}
                    unsafe {
                        (*pStep).op = ((130 as i32) as i8) as u8;
                    }
                }
            }
            _ => unsafe {
                (*pStep).op = ((130 as i32) as i8) as u8;
            },
        }
        unsafe {
            (*pStep).pTrig = pTrigger;
        }
        unsafe {
            (*pTrigger).pSchema = unsafe { (*pTab).pSchema };
        }
        unsafe {
            (*pTrigger).pTabSchema = unsafe { (*pTab).pSchema };
        }
        unsafe {
            *unsafe {
                unsafe { (*pFKey).apTrigger.as_mut_ptr() as *mut *mut Trigger }
                    .offset(iAction as isize)
            } = pTrigger;
        }
        unsafe {
            (*pTrigger).op = ((if pChanges != std::ptr::null_mut::<ExprList>() {
                130 as i32
            } else {
                129 as i32
            }) as i8) as u8;
        }
    }
    return pTrigger;
}

/// This function is called when deleting or updating a row to implement
/// any required CASCADE, SET NULL or SET DEFAULT actions.
///
/// # Arguments
///
/// * `pParse` - Parse context
/// * `pTab` - Table being updated or deleted from
/// * `pChanges` - Change-list for UPDATE, NULL for DELETE
/// * `regOld` - Address of array containing old row
/// * `aChange` - Array indicating UPDATEd columns (or 0)
/// * `bChngRowid` - True if rowid is UPDATEd
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3FkActions(
    mut pParse: *mut Parse,
    mut pTab: *mut Table,
    mut pChanges: *mut ExprList,
    mut regOld: i32,
    mut aChange: *mut i32,
    mut bChngRowid: i32,
) {
    // If foreign-key support is enabled, iterate through all FKs that
    // refer to table pTab. If there is an action associated with the FK
    // for this operation (either update or delete), invoke the associated
    // trigger sub-program.
    if (unsafe { (*unsafe { (*pParse).db }).flags }) & (((16384 as i32) as i64) as u64)
        != (0 as u64)
    {
        let mut pFKey: *mut FKey = unsafe { std::mem::zeroed() }; // Iterator variable
        pFKey = sqlite3FkReferences(pTab);
        '__slate_break_821: while pFKey != std::ptr::null_mut::<FKey>() {
            let __v841: bool;
            if aChange == std::ptr::null_mut::<i32>() {
                __v841 = true as bool;
            } else {
                __v841 = fkParentIsModified(pTab, pFKey, aChange, bChngRowid) != (0 as i32);
            }
            if __v841 {
                let mut pAct: *mut Trigger = fkActionTrigger(pParse, pTab, pFKey, pChanges);
                if pAct != std::ptr::null_mut::<Trigger>() {
                    unsafe {
                        sqlite3CodeRowTriggerDirect(pParse, pAct, pTab, regOld, 2 as i32, 0 as i32)
                    };
                }
            }
            pFKey = unsafe { (*pFKey).pNextTo };
        }
    }
}

/// Free all memory associated with foreign key definitions attached to
/// table pTab. Remove the deleted foreign keys from the Schema.fkeyHash
/// hash table.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3FkDelete(mut db: *mut sqlite3, mut pTab: *mut Table) {
    let mut pFKey: *mut FKey = unsafe { std::mem::zeroed() }; // Iterator variable
    let mut pNext: *mut FKey = unsafe { std::mem::zeroed() }; // Copy of pFKey->pNextFrom
    0 as i32;
    0 as i32;
    pFKey = unsafe { (*pTab).u.tab.pFKey };
    '__slate_break_822: while pFKey != std::ptr::null_mut::<FKey>() {
        0 as i32;
        // Remove the FK from the fkeyHash hash table.
        if (unsafe { (*db).pnBytesFreed }) == std::ptr::null_mut::<i32>() {
            if (unsafe { (*pFKey).pPrevTo }) != std::ptr::null_mut::<FKey>() {
                unsafe {
                    (*unsafe { (*pFKey).pPrevTo }).pNextTo = unsafe { (*pFKey).pNextTo };
                }
            } else {
                let mut z: *const i8 =
                    (if (unsafe { (*pFKey).pNextTo }) != std::ptr::null_mut::<FKey>() {
                        unsafe { (*unsafe { (*pFKey).pNextTo }).zTo }
                    } else {
                        unsafe { (*pFKey).zTo }
                    }) as *const i8;
                unsafe {
                    sqlite3HashInsert(
                        unsafe { std::ptr::addr_of_mut!((*unsafe { (*pTab).pSchema }).fkeyHash) },
                        z,
                        (unsafe { (*pFKey).pNextTo }) as *mut (),
                    )
                };
            }
            if (unsafe { (*pFKey).pNextTo }) != std::ptr::null_mut::<FKey>() {
                unsafe {
                    (*unsafe { (*pFKey).pNextTo }).pPrevTo = unsafe { (*pFKey).pPrevTo };
                }
            }
        }
        // EV: R-30323-21917 Each foreign key constraint in SQLite is
        // classified as either immediate or deferred.
        0 as i32;
        // Delete any triggers created to implement actions for this FK.
        fkTriggerDelete(db, unsafe {
            *unsafe {
                unsafe { (*pFKey).apTrigger.as_mut_ptr() as *mut *mut Trigger }
                    .offset((0 as i32) as isize)
            }
        });
        fkTriggerDelete(db, unsafe {
            *unsafe {
                unsafe { (*pFKey).apTrigger.as_mut_ptr() as *mut *mut Trigger }
                    .offset((1 as i32) as isize)
            }
        });
        pNext = unsafe { (*pFKey).pNextFrom };
        unsafe { sqlite3DbFree(db, pFKey as *mut ()) };
        pFKey = pNext;
    }
}
