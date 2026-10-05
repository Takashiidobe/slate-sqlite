unsafe extern "C" {
    fn sqlite3_value_double(__v862: *mut sqlite3_value) -> f64;
    fn sqlite3_value_int(__v863: *mut sqlite3_value) -> i32;
    fn sqlite3_value_int64(__v864: *mut sqlite3_value) -> i64;
    fn sqlite3_value_numeric_type(__v865: *mut sqlite3_value) -> i32;
    fn sqlite3_value_dup(__v866: *const sqlite3_value) -> *mut sqlite3_value;
    fn sqlite3_value_free(__v867: *mut sqlite3_value);
    fn sqlite3_aggregate_context(__v868: *mut sqlite3_context, nBytes: i32) -> *mut ();
    fn sqlite3_result_double(__v870: *mut sqlite3_context, __v871: f64);
    fn sqlite3_result_error(__v872: *mut sqlite3_context, __v873: *const i8, __v874: i32);
    fn sqlite3_result_error_nomem(__v875: *mut sqlite3_context);
    fn sqlite3_result_int64(__v876: *mut sqlite3_context, __v877: i64);
    fn sqlite3_result_value(__v878: *mut sqlite3_context, __v879: *mut sqlite3_value);
    fn memcpy(__dest: *mut (), __src: *const (), __n: u64) -> *mut ();
    fn memset(__s: *mut (), __c: i32, __n: u64) -> *mut ();
    fn sqlite3VdbeAddOp0(__v886: *mut Vdbe, __v887: i32) -> i32;
    fn sqlite3VdbeAddOp1(__v888: *mut Vdbe, __v889: i32, __v890: i32) -> i32;
    fn sqlite3VdbeAddOp2(__v891: *mut Vdbe, __v892: i32, __v893: i32, __v894: i32) -> i32;
    fn sqlite3VdbeAddOp3(
        __v895: *mut Vdbe,
        __v896: i32,
        __v897: i32,
        __v898: i32,
        __v899: i32,
    ) -> i32;
    fn sqlite3VdbeAddOp4(
        __v900: *mut Vdbe,
        __v901: i32,
        __v902: i32,
        __v903: i32,
        __v904: i32,
        zP4: *const i8,
        __v906: i32,
    ) -> i32;
    fn sqlite3VdbeAddOp4Int(
        __v907: *mut Vdbe,
        __v908: i32,
        __v909: i32,
        __v910: i32,
        __v911: i32,
        __v912: i32,
    ) -> i32;
    fn sqlite3VdbeChangeP1(__v913: *mut Vdbe, addr: i32, P1: i32);
    fn sqlite3VdbeChangeP5(__v916: *mut Vdbe, P5: u16);
    fn sqlite3VdbeJumpHere(__v918: *mut Vdbe, addr: i32);
    fn sqlite3VdbeAppendP4(__v920: *mut Vdbe, pP4: *mut (), p4type: i32);
    fn sqlite3VdbeGetOp(__v923: *mut Vdbe, __v924: i32) -> *mut VdbeOp;
    fn sqlite3VdbeMakeLabel(__v925: *mut Parse) -> i32;
    fn sqlite3VdbeResolveLabel(__v926: *mut Vdbe, __v927: i32);
    fn sqlite3VdbeCurrentAddr(__v928: *mut Vdbe) -> i32;
    fn sqlite3WalkExprList(__v929: *mut Walker, __v930: *mut ExprList) -> i32;
    fn sqlite3WalkSelect(__v931: *mut Walker, __v932: *mut Select) -> i32;
    fn sqlite3WalkerDepthIncrease(__v933: *mut Walker, __v934: *mut Select) -> i32;
    fn sqlite3WalkerDepthDecrease(__v935: *mut Walker, __v936: *mut Select);
    fn sqlite3StrICmp(__v984: *const i8, __v985: *const i8) -> i32;
    fn sqlite3DbMallocZero(__v986: *mut sqlite3, __v987: u64) -> *mut ();
    fn sqlite3DbStrDup(__v988: *mut sqlite3, __v989: *const i8) -> *mut i8;
    fn sqlite3DbStrNDup(__v990: *mut sqlite3, __v991: *const i8, __v992: u64) -> *mut i8;
    fn sqlite3DbFree(__v993: *mut sqlite3, __v994: *mut ());
    fn sqlite3ErrorMsg(__v995: *mut Parse, __v996: *const i8, ...);
    fn sqlite3ErrorToParser(__v997: *mut sqlite3, __v998: i32) -> i32;
    fn sqlite3GetTempReg(__v999: *mut Parse) -> i32;
    fn sqlite3ReleaseTempReg(__v1000: *mut Parse, __v1001: i32);
    fn sqlite3GetTempRange(__v1002: *mut Parse, __v1003: i32) -> i32;
    fn sqlite3ReleaseTempRange(__v1004: *mut Parse, __v1005: i32, __v1006: i32);
    fn sqlite3ExprAlloc(
        __v1007: *mut sqlite3,
        __v1008: i32,
        __v1009: *const Token,
        __v1010: i32,
    ) -> *mut Expr;
    fn sqlite3ExprInt32(__v1011: *mut sqlite3, __v1012: i32) -> *mut Expr;
    fn sqlite3ExprDelete(__v1013: *mut sqlite3, __v1014: *mut Expr);
    fn sqlite3ExprListAppend(
        __v1015: *mut Parse,
        __v1016: *mut ExprList,
        __v1017: *mut Expr,
    ) -> *mut ExprList;
    fn sqlite3ExprListDelete(__v1018: *mut sqlite3, __v1019: *mut ExprList);
    fn sqlite3ResultSetOfSelect(
        __v1020: *mut Parse,
        __v1021: *mut Select,
        __v1022: i8,
    ) -> *mut Table;
    fn sqlite3SrcListAppend(
        __v1023: *mut Parse,
        __v1024: *mut SrcList,
        __v1025: *mut Token,
        __v1026: *mut Token,
    ) -> *mut SrcList;
    fn sqlite3SrcItemAttachSubquery(
        __v1027: *mut Parse,
        __v1028: *mut SrcItem,
        __v1029: *mut Select,
        __v1030: i32,
    ) -> i32;
    fn sqlite3SrcListAssignCursors(__v1031: *mut Parse, __v1032: *mut SrcList);
    fn sqlite3SelectNew(
        __v1033: *mut Parse,
        __v1034: *mut ExprList,
        __v1035: *mut SrcList,
        __v1036: *mut Expr,
        __v1037: *mut ExprList,
        __v1038: *mut Expr,
        __v1039: *mut ExprList,
        __v1040: u32,
        __v1041: *mut Expr,
    ) -> *mut Select;
    fn sqlite3SelectDelete(__v1042: *mut sqlite3, __v1043: *mut Select);
    fn sqlite3WhereEnd(__v1044: *mut WhereInfo);
    fn sqlite3ExprCode(__v1045: *mut Parse, __v1046: *mut Expr, __v1047: i32);
    fn sqlite3ExprCodeExprList(
        __v1048: *mut Parse,
        __v1049: *mut ExprList,
        __v1050: i32,
        __v1051: i32,
        __v1052: u8,
    ) -> i32;
    fn sqlite3ExprCompare(
        __v1053: *const Parse,
        __v1054: *const Expr,
        __v1055: *const Expr,
        __v1056: i32,
    ) -> i32;
    fn sqlite3ExprListCompare(
        __v1057: *const ExprList,
        __v1058: *const ExprList,
        __v1059: i32,
    ) -> i32;
    fn sqlite3AggInfoPersistWalkerInit(__v1060: *mut Walker, __v1061: *mut Parse);
    fn sqlite3GetVdbe(__v1062: *mut Parse) -> *mut Vdbe;
    fn sqlite3ExprIsConstant(__v1063: *mut Parse, __v1064: *mut Expr) -> i32;
    fn sqlite3ExprIsInteger(
        __v1065: *const Expr,
        __v1066: *mut i32,
        __v1067: *mut Parse,
        __v1068: i32,
    ) -> i32;
    fn sqlite3MayAbort(__v1069: *mut Parse);
    fn sqlite3ExprDup(__v1070: *mut sqlite3, __v1071: *const Expr, __v1072: i32) -> *mut Expr;
    fn sqlite3ExprListDup(
        __v1073: *mut sqlite3,
        __v1074: *const ExprList,
        __v1075: i32,
    ) -> *mut ExprList;
    fn sqlite3InsertBuiltinFuncs(__v1076: *mut FuncDef, __v1077: i32);
    fn sqlite3RealToI64(__v1078: f64) -> i64;
    fn sqlite3ExprNNCollSeq(pParse: *mut Parse, pExpr: *const Expr) -> *mut CollSeq;
    fn sqlite3ExprSkipCollateAndLikely(__v1081: *mut Expr) -> *mut Expr;
    fn sqlite3ValueFree(__v1082: *mut sqlite3_value);
    fn sqlite3ValueFromExpr(
        __v1083: *mut sqlite3,
        __v1084: *const Expr,
        __v1085: u8,
        __v1086: u8,
        __v1087: *mut *mut sqlite3_value,
    ) -> i32;
    fn sqlite3RenameExprUnmap(__v1088: *mut Parse, __v1089: *mut Expr);
    fn sqlite3KeyInfoFromExprList(
        __v1090: *mut Parse,
        __v1091: *mut ExprList,
        __v1092: i32,
        __v1093: i32,
    ) -> *mut KeyInfo;
    fn sqlite3ParserAddCleanup(
        __v1094: *mut Parse,
        __v1095: Option<unsafe extern "C-unwind" fn(*mut sqlite3, *mut ())>,
        __v1096: *mut (),
    ) -> *mut ();
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
struct sqlite3_mutex {}

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
struct sqlite3_value {}

#[repr(C)]
#[derive(Clone, Copy)]
struct sqlite3_context {}

#[repr(C)]
#[derive(Clone, Copy)]
struct sqlite3_vtab {
    pModule: *const sqlite3_module,
    nRef: i32,
    zErrMsg: *mut i8,
}

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
struct sqlite3_vtab_cursor {
    pVtab: *mut sqlite3_vtab,
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
struct _ht {
    count: u32,
    chain: *mut HashElem,
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
struct AutoincInfo {
    pNext: *mut AutoincInfo,
    pTab: *mut Table,
    iDb: i32,
    regCtr: i32,
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
struct Db {
    zDbSName: *mut i8,
    pBt: *mut Btree,
    safety_level: u8,
    bSyncSet: u8,
    pSchema: *mut Schema,
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
struct FuncDestructor {
    nRef: i32,
    xDestroy: Option<unsafe extern "C-unwind" fn(*mut ())>,
    pUserData: *mut (),
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
struct IdList {
    nId: i32,
    a: [IdList_item; 0],
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
struct IndexedExpr {
    pExpr: *mut Expr,
    iDataCur: i32,
    iIdxCur: i32,
    iIdxCol: i32,
    bMaybeNullRow: u8,
    aff: u8,
    pIENext: *mut IndexedExpr,
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
struct NameContext {
    pParse: *mut Parse,
    pSrcList: *mut SrcList,
    uNC: __SlateRecord188,
    pNext: *mut NameContext,
    nRef: i32,
    nNcErr: i32,
    ncFlags: i32,
    nNestedSelect: u32,
    pWinSelect: *mut Select,
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
    u1: __SlateRecord190,
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
struct ParseCleanup {
    pNext: *mut ParseCleanup,
    pPtr: *mut (),
    xCleanup: Option<unsafe extern "C-unwind" fn(*mut sqlite3, *mut ())>,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct RenameToken {}

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
struct Savepoint {
    zName: *mut i8,
    nDeferredCons: i64,
    nDeferredImmCons: i64,
    pNext: *mut Savepoint,
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
struct TableLock {}

#[repr(C)]
#[derive(Clone, Copy)]
struct Token {
    z: *const i8,
    n: u32,
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
struct TriggerPrg {
    pTrigger: *mut Trigger,
    pNext: *mut TriggerPrg,
    pProgram: *mut SubProgram,
    orconf: i32,
    aColmask: [u32; 2],
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
struct VtabCtx {}

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
    u: __SlateRecord193,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct WhereInfo {}

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
union __SlateRecord188 {
    pEList: *mut ExprList,
    pAggInfo: *mut AggInfo,
    pUpsert: *mut Upsert,
    iBaseReg: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord190 {
    cr: __SlateRecord191,
    d: __SlateRecord192,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord191 {
    addrCrTab: i32,
    regRowid: i32,
    regRoot: i32,
    constraintName: Token,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord192 {
    pReturning: *mut Returning,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord193 {
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

// /*
// ** 2018 May 08
// **
// ** The author disclaims copyright to this source code.  In place of
// ** a legal notice, here is a blessing:
// **
// **    May you do good and not evil.
// **    May you find forgiveness for yourself and forgive others.
// **    May you share freely, never taking more than you give.
// **
// *************************************************************************
// */
#[repr(C)]
#[derive(Clone, Copy)]
struct CCurHint {}

#[repr(C)]
#[derive(Clone, Copy)]
struct RefSrcList {}

#[repr(C)]
#[derive(Clone, Copy)]
struct IdxCover {}

// /*
// ** Context object passed through sqlite3WalkExprList() to
// ** selectWindowRewriteExprCb() by selectWindowRewriteEList().
// */
#[repr(C)]
#[derive(Clone, Copy)]
struct WindowRewrite {
    pWin: *mut Window,
    pSrc: *mut SrcList,
    pSub: *mut ExprList,
    pTab: *mut Table,
    pSubSelect: *mut Select,
    // /* Current sub-select, if any */
}

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

// /*
// ** Context object type used by rank(), dense_rank(), percent_rank() and
// ** cume_dist().
// */
#[repr(C)]
#[derive(Clone, Copy)]
struct CallCount {
    nValue: i64,
    nStep: i64,
    nTotal: i64,
}

// /*
// ** Implementation of built-in window function nth_value(). This
// ** implementation is used in "slow mode" only - when the EXCLUDE clause
// ** is not set to the default value "NO OTHERS".
// */
#[repr(C)]
#[derive(Clone, Copy)]
struct NthValueCtx {
    nStep: i64,
    pValue: *mut sqlite3_value,
}

// /*
// ** Context object for ntile() window function.
// */
#[repr(C)]
#[derive(Clone, Copy)]
struct NtileCtx {
    nTotal: i64,
    // /* Total rows in partition */
    nParam: i64,
    // /* Parameter passed to ntile(N) */
    iRow: i64,
    // /* Current row */
}

// /*
// ** Context object for last_value() window function.
// */
#[repr(C)]
#[derive(Clone, Copy)]
struct LastValueCtx {
    pVal: *mut sqlite3_value,
    nVal: i32,
}

// /* List of named windows for this SELECT */
// /* Window frame to update */
// /* Window function definition */
#[repr(C)]
#[derive(Clone, Copy)]
struct WindowUpdate {
    zFunc: *const i8,
    eFrmType: i32,
    eStart: i32,
    eEnd: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct WindowCodeArg {
    // /*
    // ** See comments above struct WindowCodeArg.
    // */
    // /*
    // ** A single instance of this structure is allocated on the stack by
    // ** sqlite3WindowCodeStep() and a pointer to it passed to the various helper
    // ** routines. This is to reduce the number of arguments required by each
    // ** helper function.
    // **
    // ** regArg:
    // **   Each window function requires an accumulator register (just as an
    // **   ordinary aggregate function does). This variable is set to the first
    // **   in an array of accumulator registers - one for each window function
    // **   in the WindowCodeArg.pMWin list.
    // **
    // ** eDelete:
    // **   The window functions implementation sometimes caches the input rows
    // **   that it processes in a temporary table. If it is not zero, this
    // **   variable indicates when rows may be removed from the temp table (in
    // **   order to reduce memory requirements - it would always be safe just
    // **   to leave them there). Possible values for eDelete are:
    // **
    // **      WINDOW_RETURN_ROW:
    // **        An input row can be discarded after it is returned to the caller.
    // **
    // **      WINDOW_AGGINVERSE:
    // **        An input row can be discarded after the window functions xInverse()
    // **        callbacks have been invoked in it.
    // **
    // **      WINDOW_AGGSTEP:
    // **        An input row can be discarded after the window functions xStep()
    // **        callbacks have been invoked in it.
    // **
    // ** start,current,end
    // **   Consider a window-frame similar to the following:
    // **
    // **     (ORDER BY a, b GROUPS BETWEEN 2 PRECEDING AND 2 FOLLOWING)
    // **
    // **   The windows functions implementation caches the input rows in a temp
    // **   table, sorted by "a, b" (it actually populates the cache lazily, and
    // **   aggressively removes rows once they are no longer required, but that's
    // **   a mere detail). It keeps three cursors open on the temp table. One
    // **   (current) that points to the next row to return to the query engine
    // **   once its window function values have been calculated. Another (end)
    // **   points to the next row to call the xStep() method of each window function
    // **   on (so that it is 2 groups ahead of current). And a third (start) that
    // **   points to the next row to call the xInverse() method of each window
    // **   function on.
    // **
    // **   Each cursor (start, current and end) consists of a VDBE cursor
    // **   (WindowCsrAndReg.csr) and an array of registers (starting at
    // **   WindowCodeArg.reg) that always contains a copy of the peer values
    // **   read from the corresponding cursor.
    // **
    // **   Depending on the window-frame in question, all three cursors may not
    // **   be required. In this case both WindowCodeArg.csr and reg are set to
    // **   0.
    // */
    // /* Cursor number */
    // /* First in array of peer values */
    pParse: *mut Parse,
    // /* Parse context */
    pMWin: *mut Window,
    // /* First in list of functions being processed */
    pVdbe: *mut Vdbe,
    // /* VDBE object */
    addrGosub: i32,
    // /* OP_Gosub to this address to return one row */
    regGosub: i32,
    // /* Register used with OP_Gosub(addrGosub) */
    regArg: i32,
    // /* First in array of accumulator registers */
    eDelete: i32,
    // /* See above */
    regRowid: i32,
    start: WindowCsrAndReg,
    current: WindowCsrAndReg,
    end: WindowCsrAndReg,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct WindowCsrAndReg {
    csr: i32,
    reg: i32,
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

// /*
// ** Static names for the built-in window function names.  These static
// ** names are used, rather than string literals, so that FuncDef objects
// ** can be associated with a particular window function by direct
// ** comparison of the zName pointer.  Example:
// **
// **       if( pFuncDef->zName==row_valueName ){ ... }
// */
static mut row_numberName: [i8; 11] = [
    114 as i8, 111 as i8, 119 as i8, 95 as i8, 110 as i8, 117 as i8, 109 as i8, 98 as i8,
    101 as i8, 114 as i8, 0 as i8,
];

static mut dense_rankName: [i8; 11] = [
    100 as i8, 101 as i8, 110 as i8, 115 as i8, 101 as i8, 95 as i8, 114 as i8, 97 as i8,
    110 as i8, 107 as i8, 0 as i8,
];

static mut rankName: [i8; 5] = [114 as i8, 97 as i8, 110 as i8, 107 as i8, 0 as i8];

static mut percent_rankName: [i8; 13] = [
    112 as i8, 101 as i8, 114 as i8, 99 as i8, 101 as i8, 110 as i8, 116 as i8, 95 as i8,
    114 as i8, 97 as i8, 110 as i8, 107 as i8, 0 as i8,
];

static mut cume_distName: [i8; 10] = [
    99 as i8, 117 as i8, 109 as i8, 101 as i8, 95 as i8, 100 as i8, 105 as i8, 115 as i8,
    116 as i8, 0 as i8,
];

static mut ntileName: [i8; 6] = [
    110 as i8, 116 as i8, 105 as i8, 108 as i8, 101 as i8, 0 as i8,
];

static mut last_valueName: [i8; 11] = [
    108 as i8, 97 as i8, 115 as i8, 116 as i8, 95 as i8, 118 as i8, 97 as i8, 108 as i8, 117 as i8,
    101 as i8, 0 as i8,
];

static mut nth_valueName: [i8; 10] = [
    110 as i8, 116 as i8, 104 as i8, 95 as i8, 118 as i8, 97 as i8, 108 as i8, 117 as i8,
    101 as i8, 0 as i8,
];

static mut first_valueName: [i8; 12] = [
    102 as i8, 105 as i8, 114 as i8, 115 as i8, 116 as i8, 95 as i8, 118 as i8, 97 as i8,
    108 as i8, 117 as i8, 101 as i8, 0 as i8,
];

static mut leadName: [i8; 5] = [108 as i8, 101 as i8, 97 as i8, 100 as i8, 0 as i8];

static mut lagName: [i8; 4] = [108 as i8, 97 as i8, 103 as i8, 0 as i8];

static mut aWindowFuncs: __SlateAlign16<[FuncDef; 15]> = __SlateAlign16([
    FuncDef {
        nArg: (0 as i32) as i16,
        funcFlags: ((8388608 as i32) | (1 as i32) | (65536 as i32) | (0 as i32)) as u32,
        pUserData: std::ptr::null_mut::<()>(),
        pNext: std::ptr::null_mut::<FuncDef>(),
        xSFunc: Some(row_numberStepFunc),
        xFinalize: Some(row_numberValueFunc),
        xValue: Some(row_numberValueFunc),
        xInverse: Some(noopStepFunc),
        zName: unsafe { std::ptr::addr_of!(row_numberName) as *const i8 },
        u: {
            let mut __t0: __SlateRecord164 = unsafe { std::mem::zeroed() };
            __t0.pHash = std::ptr::null_mut::<FuncDef>();
            __t0
        },
    },
    FuncDef {
        nArg: (0 as i32) as i16,
        funcFlags: ((8388608 as i32) | (1 as i32) | (65536 as i32) | (0 as i32)) as u32,
        pUserData: std::ptr::null_mut::<()>(),
        pNext: std::ptr::null_mut::<FuncDef>(),
        xSFunc: Some(dense_rankStepFunc),
        xFinalize: Some(dense_rankValueFunc),
        xValue: Some(dense_rankValueFunc),
        xInverse: Some(noopStepFunc),
        zName: unsafe { std::ptr::addr_of!(dense_rankName) as *const i8 },
        u: {
            let mut __t1: __SlateRecord164 = unsafe { std::mem::zeroed() };
            __t1.pHash = std::ptr::null_mut::<FuncDef>();
            __t1
        },
    },
    FuncDef {
        nArg: (0 as i32) as i16,
        funcFlags: ((8388608 as i32) | (1 as i32) | (65536 as i32) | (0 as i32)) as u32,
        pUserData: std::ptr::null_mut::<()>(),
        pNext: std::ptr::null_mut::<FuncDef>(),
        xSFunc: Some(rankStepFunc),
        xFinalize: Some(rankValueFunc),
        xValue: Some(rankValueFunc),
        xInverse: Some(noopStepFunc),
        zName: unsafe { std::ptr::addr_of!(rankName) as *const i8 },
        u: {
            let mut __t2: __SlateRecord164 = unsafe { std::mem::zeroed() };
            __t2.pHash = std::ptr::null_mut::<FuncDef>();
            __t2
        },
    },
    FuncDef {
        nArg: (0 as i32) as i16,
        funcFlags: ((8388608 as i32) | (1 as i32) | (65536 as i32) | (0 as i32)) as u32,
        pUserData: std::ptr::null_mut::<()>(),
        pNext: std::ptr::null_mut::<FuncDef>(),
        xSFunc: Some(percent_rankStepFunc),
        xFinalize: Some(percent_rankValueFunc),
        xValue: Some(percent_rankValueFunc),
        xInverse: Some(percent_rankInvFunc),
        zName: unsafe { std::ptr::addr_of!(percent_rankName) as *const i8 },
        u: {
            let mut __t3: __SlateRecord164 = unsafe { std::mem::zeroed() };
            __t3.pHash = std::ptr::null_mut::<FuncDef>();
            __t3
        },
    },
    FuncDef {
        nArg: (0 as i32) as i16,
        funcFlags: ((8388608 as i32) | (1 as i32) | (65536 as i32) | (0 as i32)) as u32,
        pUserData: std::ptr::null_mut::<()>(),
        pNext: std::ptr::null_mut::<FuncDef>(),
        xSFunc: Some(cume_distStepFunc),
        xFinalize: Some(cume_distValueFunc),
        xValue: Some(cume_distValueFunc),
        xInverse: Some(cume_distInvFunc),
        zName: unsafe { std::ptr::addr_of!(cume_distName) as *const i8 },
        u: {
            let mut __t4: __SlateRecord164 = unsafe { std::mem::zeroed() };
            __t4.pHash = std::ptr::null_mut::<FuncDef>();
            __t4
        },
    },
    FuncDef {
        nArg: (1 as i32) as i16,
        funcFlags: ((8388608 as i32) | (1 as i32) | (65536 as i32) | (0 as i32)) as u32,
        pUserData: std::ptr::null_mut::<()>(),
        pNext: std::ptr::null_mut::<FuncDef>(),
        xSFunc: Some(ntileStepFunc),
        xFinalize: Some(ntileValueFunc),
        xValue: Some(ntileValueFunc),
        xInverse: Some(ntileInvFunc),
        zName: unsafe { std::ptr::addr_of!(ntileName) as *const i8 },
        u: {
            let mut __t5: __SlateRecord164 = unsafe { std::mem::zeroed() };
            __t5.pHash = std::ptr::null_mut::<FuncDef>();
            __t5
        },
    },
    FuncDef {
        nArg: (1 as i32) as i16,
        funcFlags: ((8388608 as i32) | (1 as i32) | (65536 as i32) | (0 as i32)) as u32,
        pUserData: std::ptr::null_mut::<()>(),
        pNext: std::ptr::null_mut::<FuncDef>(),
        xSFunc: Some(last_valueStepFunc),
        xFinalize: Some(last_valueFinalizeFunc),
        xValue: Some(last_valueValueFunc),
        xInverse: Some(last_valueInvFunc),
        zName: unsafe { std::ptr::addr_of!(last_valueName) as *const i8 },
        u: {
            let mut __t6: __SlateRecord164 = unsafe { std::mem::zeroed() };
            __t6.pHash = std::ptr::null_mut::<FuncDef>();
            __t6
        },
    },
    FuncDef {
        nArg: (2 as i32) as i16,
        funcFlags: ((8388608 as i32) | (1 as i32) | (65536 as i32) | (0 as i32)) as u32,
        pUserData: std::ptr::null_mut::<()>(),
        pNext: std::ptr::null_mut::<FuncDef>(),
        xSFunc: Some(nth_valueStepFunc),
        xFinalize: Some(nth_valueFinalizeFunc),
        xValue: Some(noopValueFunc),
        xInverse: Some(noopStepFunc),
        zName: unsafe { std::ptr::addr_of!(nth_valueName) as *const i8 },
        u: {
            let mut __t7: __SlateRecord164 = unsafe { std::mem::zeroed() };
            __t7.pHash = std::ptr::null_mut::<FuncDef>();
            __t7
        },
    },
    FuncDef {
        nArg: (1 as i32) as i16,
        funcFlags: ((8388608 as i32) | (1 as i32) | (65536 as i32) | (0 as i32)) as u32,
        pUserData: std::ptr::null_mut::<()>(),
        pNext: std::ptr::null_mut::<FuncDef>(),
        xSFunc: Some(first_valueStepFunc),
        xFinalize: Some(first_valueFinalizeFunc),
        xValue: Some(noopValueFunc),
        xInverse: Some(noopStepFunc),
        zName: unsafe { std::ptr::addr_of!(first_valueName) as *const i8 },
        u: {
            let mut __t8: __SlateRecord164 = unsafe { std::mem::zeroed() };
            __t8.pHash = std::ptr::null_mut::<FuncDef>();
            __t8
        },
    },
    FuncDef {
        nArg: (1 as i32) as i16,
        funcFlags: ((8388608 as i32) | (1 as i32) | (65536 as i32) | (0 as i32)) as u32,
        pUserData: std::ptr::null_mut::<()>(),
        pNext: std::ptr::null_mut::<FuncDef>(),
        xSFunc: Some(noopStepFunc),
        xFinalize: Some(noopValueFunc),
        xValue: Some(noopValueFunc),
        xInverse: Some(noopStepFunc),
        zName: unsafe { std::ptr::addr_of!(leadName) as *const i8 },
        u: {
            let mut __t9: __SlateRecord164 = unsafe { std::mem::zeroed() };
            __t9.pHash = std::ptr::null_mut::<FuncDef>();
            __t9
        },
    },
    FuncDef {
        nArg: (2 as i32) as i16,
        funcFlags: ((8388608 as i32) | (1 as i32) | (65536 as i32) | (0 as i32)) as u32,
        pUserData: std::ptr::null_mut::<()>(),
        pNext: std::ptr::null_mut::<FuncDef>(),
        xSFunc: Some(noopStepFunc),
        xFinalize: Some(noopValueFunc),
        xValue: Some(noopValueFunc),
        xInverse: Some(noopStepFunc),
        zName: unsafe { std::ptr::addr_of!(leadName) as *const i8 },
        u: {
            let mut __t10: __SlateRecord164 = unsafe { std::mem::zeroed() };
            __t10.pHash = std::ptr::null_mut::<FuncDef>();
            __t10
        },
    },
    FuncDef {
        nArg: (3 as i32) as i16,
        funcFlags: ((8388608 as i32) | (1 as i32) | (65536 as i32) | (0 as i32)) as u32,
        pUserData: std::ptr::null_mut::<()>(),
        pNext: std::ptr::null_mut::<FuncDef>(),
        xSFunc: Some(noopStepFunc),
        xFinalize: Some(noopValueFunc),
        xValue: Some(noopValueFunc),
        xInverse: Some(noopStepFunc),
        zName: unsafe { std::ptr::addr_of!(leadName) as *const i8 },
        u: {
            let mut __t11: __SlateRecord164 = unsafe { std::mem::zeroed() };
            __t11.pHash = std::ptr::null_mut::<FuncDef>();
            __t11
        },
    },
    FuncDef {
        nArg: (1 as i32) as i16,
        funcFlags: ((8388608 as i32) | (1 as i32) | (65536 as i32) | (0 as i32)) as u32,
        pUserData: std::ptr::null_mut::<()>(),
        pNext: std::ptr::null_mut::<FuncDef>(),
        xSFunc: Some(noopStepFunc),
        xFinalize: Some(noopValueFunc),
        xValue: Some(noopValueFunc),
        xInverse: Some(noopStepFunc),
        zName: unsafe { std::ptr::addr_of!(lagName) as *const i8 },
        u: {
            let mut __t12: __SlateRecord164 = unsafe { std::mem::zeroed() };
            __t12.pHash = std::ptr::null_mut::<FuncDef>();
            __t12
        },
    },
    FuncDef {
        nArg: (2 as i32) as i16,
        funcFlags: ((8388608 as i32) | (1 as i32) | (65536 as i32) | (0 as i32)) as u32,
        pUserData: std::ptr::null_mut::<()>(),
        pNext: std::ptr::null_mut::<FuncDef>(),
        xSFunc: Some(noopStepFunc),
        xFinalize: Some(noopValueFunc),
        xValue: Some(noopValueFunc),
        xInverse: Some(noopStepFunc),
        zName: unsafe { std::ptr::addr_of!(lagName) as *const i8 },
        u: {
            let mut __t13: __SlateRecord164 = unsafe { std::mem::zeroed() };
            __t13.pHash = std::ptr::null_mut::<FuncDef>();
            __t13
        },
    },
    FuncDef {
        nArg: (3 as i32) as i16,
        funcFlags: ((8388608 as i32) | (1 as i32) | (65536 as i32) | (0 as i32)) as u32,
        pUserData: std::ptr::null_mut::<()>(),
        pNext: std::ptr::null_mut::<FuncDef>(),
        xSFunc: Some(noopStepFunc),
        xFinalize: Some(noopValueFunc),
        xValue: Some(noopValueFunc),
        xInverse: Some(noopStepFunc),
        zName: unsafe { std::ptr::addr_of!(lagName) as *const i8 },
        u: {
            let mut __t14: __SlateRecord164 = unsafe { std::mem::zeroed() };
            __t14.pHash = std::ptr::null_mut::<FuncDef>();
            __t14
        },
    },
]);

static mut azErr: __SlateAlign16<[*const i8; 5]> = __SlateAlign16([
    (b"frame starting offset must be a non-negative integer\0".as_ptr() as *mut i8) as *const i8,
    (b"frame ending offset must be a non-negative integer\0".as_ptr() as *mut i8) as *const i8,
    (b"second argument to nth_value must be a positive integer\0".as_ptr() as *mut i8) as *const i8,
    (b"frame starting offset must be a non-negative number\0".as_ptr() as *mut i8) as *const i8,
    (b"frame ending offset must be a non-negative number\0".as_ptr() as *mut i8) as *const i8,
]);

static mut aOp: __SlateAlign16<[i32; 5]> =
    __SlateAlign16([58 as i32, 58 as i32, 55 as i32, 58 as i32, 58 as i32]);

// /*
// ** Free the Window object passed as the second argument.
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3WindowDelete(mut db: *mut sqlite3, mut p: *mut Window) {
    if p != std::ptr::null_mut::<Window>() {
        sqlite3WindowUnlinkFromSelect(p);
        unsafe { sqlite3ExprDelete(db, unsafe { (*p).pFilter }) };
        unsafe { sqlite3ExprListDelete(db, unsafe { (*p).pPartition }) };
        unsafe { sqlite3ExprListDelete(db, unsafe { (*p).pOrderBy }) };
        unsafe { sqlite3ExprDelete(db, unsafe { (*p).pEnd }) };
        unsafe { sqlite3ExprDelete(db, unsafe { (*p).pStart }) };
        unsafe { sqlite3DbFree(db, (unsafe { (*p).zName }) as *mut ()) };
        unsafe { sqlite3DbFree(db, (unsafe { (*p).zBase }) as *mut ()) };
        unsafe { sqlite3DbFree(db, p as *mut ()) };
    }
}

// /*
// ** Unlink the Window object from the Select to which it is attached,
// ** if it is attached.
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3WindowUnlinkFromSelect(mut p: *mut Window) {
    if (unsafe { (*p).ppThis }) != std::ptr::null_mut::<*mut Window>() {
        unsafe {
            *unsafe { (*p).ppThis } = unsafe { (*p).pNextWin };
        }
        if (unsafe { (*p).pNextWin }) != std::ptr::null_mut::<Window>() {
            unsafe {
                (*unsafe { (*p).pNextWin }).ppThis = unsafe { (*p).ppThis };
            }
        }
        unsafe {
            (*p).ppThis = std::ptr::null_mut::<*mut Window>();
        }
    }
}

// /*
// ** Free the linked list of Window objects starting at the second argument.
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3WindowListDelete(mut db: *mut sqlite3, mut p: *mut Window) {
    '__slate_break_1112: while p != std::ptr::null_mut::<Window>() {
        let mut pNext: *mut Window = unsafe { (*p).pNextWin };
        sqlite3WindowDelete(db, p);
        p = pNext;
    }
}

// /*
// ** Allocate and return a new Window object describing a Window Definition.
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3WindowAlloc(
    mut pParse: *mut Parse,
    mut eType: i32,
    mut eStart: i32,
    mut pStart: *mut Expr,
    mut eEnd: i32,
    mut pEnd: *mut Expr,
    mut eExclude: u8,
) -> *mut Window {
    let mut __slate_storage_622: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_622: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_622) as *mut i32;
    let mut __slate_storage_621: std::mem::MaybeUninit<*mut Window> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_621: *mut *mut Window =
        std::ptr::addr_of_mut!(__slate_storage_621) as *mut *mut Window;
    unsafe {
        '__join_6: {
            std::ptr::write(__slate_slot_621, std::ptr::null_mut::<Window>());
            std::ptr::write(__slate_slot_622, 0 as i32);
            // /* Parser assures the following: */
            0 as i32;
            0 as i32;
            0 as i32;
            0 as i32;
            0 as i32;
            if eType == (0 as i32) {
                *__slate_slot_622 = 1 as i32;
                eType = 90 as i32;
            }
        }
        // /* Additionally, the
        //   ** starting boundary type may not occur earlier in the following list than
        //   ** the ending boundary type:
        //   **
        //   **   UNBOUNDED PRECEDING
        //   **   <expr> PRECEDING
        //   **   CURRENT ROW
        //   **   <expr> FOLLOWING
        //   **   UNBOUNDED FOLLOWING
        //   **
        //   ** The parser ensures that "UNBOUNDED PRECEDING" cannot be used as an ending
        //   ** boundary, and than "UNBOUNDED FOLLOWING" cannot be used as a starting
        //   ** frame boundary.
        //   */
        if eStart == (86 as i32) && eEnd == (89 as i32)
            || eStart == (87 as i32) && (eEnd == (89 as i32) || eEnd == (86 as i32))
        {
            unsafe {
                sqlite3ErrorMsg(
                    pParse,
                    (b"unsupported frame specification\0".as_ptr() as *mut i8) as *const i8,
                )
            };
        } else {
            *__slate_slot_621 =
                (unsafe { sqlite3DbMallocZero(unsafe { (*pParse).db }, 144 as u64) })
                    as *mut Window;
            if *__slate_slot_621 == std::ptr::null_mut::<Window>() {
            } else {
                unsafe {
                    (*(*__slate_slot_621)).eFrmType = (eType as i8) as u8;
                }
                unsafe {
                    (*(*__slate_slot_621)).eStart = (eStart as i8) as u8;
                }
                unsafe {
                    (*(*__slate_slot_621)).eEnd = (eEnd as i8) as u8;
                }
                if ((eExclude as u32) as i32) == (0 as i32)
                    && (unsafe { (*unsafe { (*pParse).db }).dbOptFlags }) & ((2 as i32) as u32)
                        != ((0 as i32) as u32)
                {
                    eExclude = ((67 as i32) as i8) as u8;
                }
                unsafe {
                    (*(*__slate_slot_621)).eExclude = eExclude;
                }
                unsafe {
                    (*(*__slate_slot_621)).bImplicitFrame = (*__slate_slot_622 as i8) as u8;
                }
                unsafe {
                    (*(*__slate_slot_621)).pEnd = sqlite3WindowOffsetExpr(pParse, pEnd);
                }
                unsafe {
                    (*(*__slate_slot_621)).pStart = sqlite3WindowOffsetExpr(pParse, pStart);
                }
                return *__slate_slot_621;
            }
        }
        unsafe { sqlite3ExprDelete(unsafe { (*pParse).db }, pEnd) };
        unsafe { sqlite3ExprDelete(unsafe { (*pParse).db }, pStart) };
        return std::ptr::null_mut::<Window>();
    }
    return unsafe { std::mem::zeroed() };
}

// /*
// ** Attach window object pWin to expression p.
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3WindowAttach(
    mut pParse: *mut Parse,
    mut p: *mut Expr,
    mut pWin: *mut Window,
) {
    if p != std::ptr::null_mut::<Expr>() {
        0 as i32;
        0 as i32;
        0 as i32;
        unsafe {
            (*p).y.pWin = pWin;
        }
        let __v1142: *mut Expr = p;
        let __v1143: u32 = unsafe { (*__v1142).flags };
        let __v1144: u32 = __v1143 | (((16777216 as i32) | (131072 as i32)) as u32);
        unsafe {
            (*__v1142).flags = __v1144;
        }
        unsafe {
            (*pWin).pOwner = p;
        }
        if (unsafe { (*p).flags }) & ((4 as i32) as u32) != (0 as u32)
            && (((unsafe { (*pWin).eFrmType }) as u32) as i32) != (167 as i32)
        {
            unsafe {
                sqlite3ErrorMsg(
                    pParse,
                    (b"DISTINCT is not supported for window functions\0".as_ptr() as *mut i8)
                        as *const i8,
                )
            };
        }
    } else {
        sqlite3WindowDelete(unsafe { (*pParse).db }, pWin);
    }
}

// /*
// ** Possibly link window pWin into the list at pSel->pWin (window functions
// ** to be processed as part of SELECT statement pSel). The window is linked
// ** in if either (a) there are no other windows already linked to this
// ** SELECT, or (b) the windows already linked use a compatible window frame.
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3WindowLink(mut pSel: *mut Select, mut pWin: *mut Window) {
    if pSel != std::ptr::null_mut::<Select>() {
        let __v1145: bool;
        if std::ptr::null_mut::<Window>() == unsafe { (*pSel).pWin } {
            __v1145 = true as bool;
        } else {
            __v1145 = (0 as i32)
                == sqlite3WindowCompare(
                    std::ptr::null::<Parse>(),
                    (unsafe { (*pSel).pWin }) as *const Window,
                    pWin as *const Window,
                    0 as i32,
                );
        }
        if __v1145 {
            unsafe {
                (*pWin).pNextWin = unsafe { (*pSel).pWin };
            }
            if (unsafe { (*pSel).pWin }) != std::ptr::null_mut::<Window>() {
                unsafe {
                    (*unsafe { (*pSel).pWin }).ppThis =
                        unsafe { std::ptr::addr_of_mut!((*pWin).pNextWin) };
                }
            }
            unsafe {
                (*pSel).pWin = pWin;
            }
            unsafe {
                (*pWin).ppThis = unsafe { std::ptr::addr_of_mut!((*pSel).pWin) };
            }
        } else {
            if (unsafe {
                sqlite3ExprListCompare(
                    (unsafe { (*pWin).pPartition }) as *const ExprList,
                    (unsafe { (*unsafe { (*pSel).pWin }).pPartition }) as *const ExprList,
                    -(1 as i32),
                )
            }) != (0 as i32)
            {
                let __v1146: *mut Select = pSel;
                let __v1147: u32 = unsafe { (*__v1146).selFlags };
                let __v1148: u32 = __v1147 | ((33554432 as i32) as u32);
                unsafe {
                    (*__v1146).selFlags = __v1148;
                }
            }
        }
    }
}

// /*
// ** Return 0 if the two window objects are identical, 1 if they are
// ** different, or 2 if it cannot be determined if the objects are identical
// ** or not. Identical window objects can be processed in a single scan.
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3WindowCompare(
    mut pParse: *const Parse,
    mut p1: *const Window,
    mut p2: *const Window,
    mut bFilter: i32,
) -> i32 {
    let mut res: i32 = 0 as i32;
    if p1 == std::ptr::null::<Window>() || p2 == std::ptr::null::<Window>() {
        return 1 as i32;
    }
    if (((unsafe { (*p1).eFrmType }) as u32) as i32)
        != (((unsafe { (*p2).eFrmType }) as u32) as i32)
    {
        return 1 as i32;
    }
    if (((unsafe { (*p1).eStart }) as u32) as i32) != (((unsafe { (*p2).eStart }) as u32) as i32) {
        return 1 as i32;
    }
    if (((unsafe { (*p1).eEnd }) as u32) as i32) != (((unsafe { (*p2).eEnd }) as u32) as i32) {
        return 1 as i32;
    }
    if (((unsafe { (*p1).eExclude }) as u32) as i32)
        != (((unsafe { (*p2).eExclude }) as u32) as i32)
    {
        return 1 as i32;
    }
    if (unsafe {
        sqlite3ExprCompare(
            pParse,
            (unsafe { (*p1).pStart }) as *const Expr,
            (unsafe { (*p2).pStart }) as *const Expr,
            -(1 as i32),
        )
    }) != (0 as i32)
    {
        return 1 as i32;
    }
    if (unsafe {
        sqlite3ExprCompare(
            pParse,
            (unsafe { (*p1).pEnd }) as *const Expr,
            (unsafe { (*p2).pEnd }) as *const Expr,
            -(1 as i32),
        )
    }) != (0 as i32)
    {
        return 1 as i32;
    }
    let __v1149: i32 = unsafe {
        sqlite3ExprListCompare(
            (unsafe { (*p1).pPartition }) as *const ExprList,
            (unsafe { (*p2).pPartition }) as *const ExprList,
            -(1 as i32),
        )
    };
    res = __v1149;
    if __v1149 != (0 as i32) {
        return res;
    }
    let __v1150: i32 = unsafe {
        sqlite3ExprListCompare(
            (unsafe { (*p1).pOrderBy }) as *const ExprList,
            (unsafe { (*p2).pOrderBy }) as *const ExprList,
            -(1 as i32),
        )
    };
    res = __v1150;
    if __v1150 != (0 as i32) {
        return res;
    }
    if bFilter != (0 as i32) {
        let __v1151: i32 = unsafe {
            sqlite3ExprCompare(
                pParse,
                (unsafe { (*p1).pFilter }) as *const Expr,
                (unsafe { (*p2).pFilter }) as *const Expr,
                -(1 as i32),
            )
        };
        res = __v1151;
        if __v1151 != (0 as i32) {
            return res;
        }
    }
    return 0 as i32;
}

// /*
// ** This is called by code in select.c before it calls sqlite3WhereBegin()
// ** to begin iterating through the sub-query results. It is used to allocate
// ** and initialize registers and cursors used by sqlite3WindowCodeStep().
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3WindowCodeInit(mut pParse: *mut Parse, mut pSelect: *mut Select) {
    let mut pWin: *mut Window = unsafe { std::mem::zeroed() };
    let mut nEphExpr: i32 = 0 as i32;
    let mut pMWin: *mut Window = unsafe { std::mem::zeroed() };
    let mut v: *mut Vdbe = unsafe { std::mem::zeroed() };
    0 as i32;
    nEphExpr = unsafe {
        (*unsafe {
            (*unsafe {
                (*unsafe {
                    (*unsafe {
                        unsafe {
                            std::ptr::addr_of_mut!((*unsafe { (*pSelect).pSrc }).a) as *mut SrcItem
                        }
                        .offset((0 as i32) as isize)
                    })
                    .u4
                    .pSubq
                })
                .pSelect
            })
            .pEList
        })
        .nExpr
    };
    pMWin = unsafe { (*pSelect).pWin };
    v = unsafe { sqlite3GetVdbe(pParse) };
    unsafe { sqlite3VdbeAddOp2(v, 120 as i32, unsafe { (*pMWin).iEphCsr }, nEphExpr) };
    unsafe {
        sqlite3VdbeAddOp2(
            v,
            117 as i32,
            (unsafe { (*pMWin).iEphCsr }) + (1 as i32),
            unsafe { (*pMWin).iEphCsr },
        )
    };
    unsafe {
        sqlite3VdbeAddOp2(
            v,
            117 as i32,
            (unsafe { (*pMWin).iEphCsr }) + (2 as i32),
            unsafe { (*pMWin).iEphCsr },
        )
    };
    unsafe {
        sqlite3VdbeAddOp2(
            v,
            117 as i32,
            (unsafe { (*pMWin).iEphCsr }) + (3 as i32),
            unsafe { (*pMWin).iEphCsr },
        )
    };
    // /* Allocate registers to use for PARTITION BY values, if any. Initialize
    //   ** said registers to NULL.  */
    if (unsafe { (*pMWin).pPartition }) != std::ptr::null_mut::<ExprList>() {
        let mut nExpr: i32 = unsafe { (*unsafe { (*pMWin).pPartition }).nExpr };
        unsafe {
            (*pMWin).regPart = (unsafe { (*pParse).nMem }) + (1 as i32);
        }
        let __v1152: *mut Parse = pParse;
        let __v1153: i32 = unsafe { (*__v1152).nMem };
        let __v1154: i32 = __v1153 + nExpr;
        unsafe {
            (*__v1152).nMem = __v1154;
        }
        unsafe {
            sqlite3VdbeAddOp3(
                v,
                77 as i32,
                0 as i32,
                unsafe { (*pMWin).regPart },
                (unsafe { (*pMWin).regPart }) + nExpr - (1 as i32),
            )
        };
    }
    let __v1155: *mut Parse = pParse;
    let __v1156: i32 = unsafe { (*__v1155).nMem };
    let __v1157: i32 = __v1156 + (1 as i32);
    unsafe {
        (*__v1155).nMem = __v1157;
    }
    unsafe {
        (*pMWin).regOne = __v1157;
    }
    unsafe { sqlite3VdbeAddOp2(v, 73 as i32, 1 as i32, unsafe { (*pMWin).regOne }) };
    if (unsafe { (*pMWin).eExclude }) != (0 as u8) {
        let __v1158: *mut Parse = pParse;
        let __v1159: i32 = unsafe { (*__v1158).nMem };
        let __v1160: i32 = __v1159 + (1 as i32);
        unsafe {
            (*__v1158).nMem = __v1160;
        }
        unsafe {
            (*pMWin).regStartRowid = __v1160;
        }
        let __v1161: *mut Parse = pParse;
        let __v1162: i32 = unsafe { (*__v1161).nMem };
        let __v1163: i32 = __v1162 + (1 as i32);
        unsafe {
            (*__v1161).nMem = __v1163;
        }
        unsafe {
            (*pMWin).regEndRowid = __v1163;
        }
        let __v1164: *mut Parse = pParse;
        let __v1165: i32 = unsafe { (*__v1164).nTab };
        let __v1166: i32 = __v1165 + (1 as i32);
        unsafe {
            (*__v1164).nTab = __v1166;
        }
        unsafe {
            (*pMWin).csrApp = __v1165;
        }
        unsafe { sqlite3VdbeAddOp2(v, 73 as i32, 1 as i32, unsafe { (*pMWin).regStartRowid }) };
        unsafe { sqlite3VdbeAddOp2(v, 73 as i32, 0 as i32, unsafe { (*pMWin).regEndRowid }) };
        unsafe {
            sqlite3VdbeAddOp2(v, 117 as i32, unsafe { (*pMWin).csrApp }, unsafe {
                (*pMWin).iEphCsr
            })
        };
        return;
    }
    pWin = pMWin;
    '__slate_break_1119: while pWin != std::ptr::null_mut::<Window>() {
        let mut p: *mut FuncDef = unsafe { (*pWin).pWFunc };
        if (unsafe { (*p).funcFlags }) & ((4096 as i32) as u32) != (0 as u32)
            && (((unsafe { (*pWin).eStart }) as u32) as i32) != (91 as i32)
        {
            // /* The inline versions of min() and max() require a single ephemeral
            //       ** table and 3 registers. The registers are used as follows:
            //       **
            //       **   regApp+0: slot to copy min()/max() argument to for MakeRecord
            //       **   regApp+1: integer value used to ensure keys are unique
            //       **   regApp+2: output of MakeRecord
            //       */
            let mut pList: *mut ExprList = unsafe { std::mem::zeroed() };
            let mut pKeyInfo: *mut KeyInfo = unsafe { std::mem::zeroed() };
            0 as i32;
            pList = unsafe { (*unsafe { (*pWin).pOwner }).x.pList };
            pKeyInfo = unsafe { sqlite3KeyInfoFromExprList(pParse, pList, 0 as i32, 0 as i32) };
            let __v1167: *mut Parse = pParse;
            let __v1168: i32 = unsafe { (*__v1167).nTab };
            let __v1169: i32 = __v1168 + (1 as i32);
            unsafe {
                (*__v1167).nTab = __v1169;
            }
            unsafe {
                (*pWin).csrApp = __v1168;
            }
            unsafe {
                (*pWin).regApp = (unsafe { (*pParse).nMem }) + (1 as i32);
            }
            let __v1170: *mut Parse = pParse;
            let __v1171: i32 = unsafe { (*__v1170).nMem };
            let __v1172: i32 = __v1171 + (3 as i32);
            unsafe {
                (*__v1170).nMem = __v1172;
            }
            if pKeyInfo != std::ptr::null_mut::<KeyInfo>()
                && ((unsafe {
                    *unsafe {
                        unsafe { (*unsafe { (*pWin).pWFunc }).zName }.offset((1 as i32) as isize)
                    }
                }) as i32)
                    == (105 as i32)
            {
                0 as i32;
                unsafe {
                    *unsafe { unsafe { (*pKeyInfo).aSortFlags }.offset((0 as i32) as isize) } =
                        ((1 as i32) as i8) as u8;
                }
            }
            unsafe { sqlite3VdbeAddOp2(v, 120 as i32, unsafe { (*pWin).csrApp }, 2 as i32) };
            unsafe { sqlite3VdbeAppendP4(v, pKeyInfo as *mut (), -(9 as i32)) };
            unsafe {
                sqlite3VdbeAddOp2(
                    v,
                    73 as i32,
                    0 as i32,
                    (unsafe { (*pWin).regApp }) + (1 as i32),
                )
            };
        } else {
            if (unsafe { (*p).zName }) == unsafe { std::ptr::addr_of!(nth_valueName) as *const i8 }
                || (unsafe { (*p).zName })
                    == unsafe { std::ptr::addr_of!(first_valueName) as *const i8 }
            {
                // /* Allocate two registers at pWin->regApp. These will be used to
                //       ** store the start and end index of the current frame.  */
                unsafe {
                    (*pWin).regApp = (unsafe { (*pParse).nMem }) + (1 as i32);
                }
                let __v1173: *mut Parse = pParse;
                let __v1174: i32 = unsafe { (*__v1173).nTab };
                let __v1175: i32 = __v1174 + (1 as i32);
                unsafe {
                    (*__v1173).nTab = __v1175;
                }
                unsafe {
                    (*pWin).csrApp = __v1174;
                }
                let __v1176: *mut Parse = pParse;
                let __v1177: i32 = unsafe { (*__v1176).nMem };
                let __v1178: i32 = __v1177 + (2 as i32);
                unsafe {
                    (*__v1176).nMem = __v1178;
                }
                unsafe {
                    sqlite3VdbeAddOp2(v, 117 as i32, unsafe { (*pWin).csrApp }, unsafe {
                        (*pMWin).iEphCsr
                    })
                };
            } else {
                if (unsafe { (*p).zName }) == unsafe { std::ptr::addr_of!(leadName) as *const i8 }
                    || (unsafe { (*p).zName })
                        == unsafe { std::ptr::addr_of!(lagName) as *const i8 }
                {
                    let __v1179: *mut Parse = pParse;
                    let __v1180: i32 = unsafe { (*__v1179).nTab };
                    let __v1181: i32 = __v1180 + (1 as i32);
                    unsafe {
                        (*__v1179).nTab = __v1181;
                    }
                    unsafe {
                        (*pWin).csrApp = __v1180;
                    }
                    unsafe {
                        sqlite3VdbeAddOp2(v, 117 as i32, unsafe { (*pWin).csrApp }, unsafe {
                            (*pMWin).iEphCsr
                        })
                    };
                }
            }
        }
        pWin = unsafe { (*pWin).pNextWin };
    }
}

// /*
// ** sqlite3WhereBegin() has already been called for the SELECT statement
// ** passed as the second argument when this function is invoked. It generates
// ** code to populate the Window.regResult register for each window function
// ** and invoke the sub-routine at instruction addrGosub once for each row.
// ** sqlite3WhereEnd() is always called before returning.
// **
// ** This function handles several different types of window frames, which
// ** require slightly different processing. The following pseudo code is
// ** used to implement window frames of the form:
// **
// **   ROWS BETWEEN <expr1> PRECEDING AND <expr2> FOLLOWING
// **
// ** Other window frame types use variants of the following:
// **
// **     ... loop started by sqlite3WhereBegin() ...
// **       if( new partition ){
// **         Gosub flush
// **       }
// **       Insert new row into eph table.
// **
// **       if( first row of partition ){
// **         // Rewind three cursors, all open on the eph table.
// **         Rewind(csrEnd);
// **         Rewind(csrStart);
// **         Rewind(csrCurrent);
// **
// **         regEnd = <expr2>          // FOLLOWING expression
// **         regStart = <expr1>        // PRECEDING expression
// **       }else{
// **         // First time this branch is taken, the eph table contains two
// **         // rows. The first row in the partition, which all three cursors
// **         // currently point to, and the following row.
// **         AGGSTEP
// **         if( (regEnd--)<=0 ){
// **           RETURN_ROW
// **           if( (regStart--)<=0 ){
// **             AGGINVERSE
// **           }
// **         }
// **       }
// **     }
// **     flush:
// **       AGGSTEP
// **       while( 1 ){
// **         RETURN ROW
// **         if( csrCurrent is EOF ) break;
// **         if( (regStart--)<=0 ){
// **           AggInverse(csrStart)
// **           Next(csrStart)
// **         }
// **       }
// **
// ** The pseudo-code above uses the following shorthand:
// **
// **   AGGSTEP:    invoke the aggregate xStep() function for each window function
// **               with arguments read from the current row of cursor csrEnd, then
// **               step cursor csrEnd forward one row (i.e. sqlite3BtreeNext()).
// **
// **   RETURN_ROW: return a row to the caller based on the contents of the
// **               current row of csrCurrent and the current state of all
// **               aggregates. Then step cursor csrCurrent forward one row.
// **
// **   AGGINVERSE: invoke the aggregate xInverse() function for each window
// **               functions with arguments read from the current row of cursor
// **               csrStart. Then step csrStart forward one row.
// **
// ** There are two other ROWS window frames that are handled significantly
// ** differently from the above - "BETWEEN <expr> PRECEDING AND <expr> PRECEDING"
// ** and "BETWEEN <expr> FOLLOWING AND <expr> FOLLOWING". These are special
// ** cases because they change the order in which the three cursors (csrStart,
// ** csrCurrent and csrEnd) iterate through the ephemeral table. Cases that
// ** use UNBOUNDED or CURRENT ROW are much simpler variations on one of these
// ** three.
// **
// **   ROWS BETWEEN <expr1> PRECEDING AND <expr2> PRECEDING
// **
// **     ... loop started by sqlite3WhereBegin() ...
// **       if( new partition ){
// **         Gosub flush
// **       }
// **       Insert new row into eph table.
// **       if( first row of partition ){
// **         Rewind(csrEnd) ; Rewind(csrStart) ; Rewind(csrCurrent)
// **         regEnd = <expr2>
// **         regStart = <expr1>
// **       }else{
// **         if( (regEnd--)<=0 ){
// **           AGGSTEP
// **         }
// **         RETURN_ROW
// **         if( (regStart--)<=0 ){
// **           AGGINVERSE
// **         }
// **       }
// **     }
// **     flush:
// **       if( (regEnd--)<=0 ){
// **         AGGSTEP
// **       }
// **       RETURN_ROW
// **
// **
// **   ROWS BETWEEN <expr1> FOLLOWING AND <expr2> FOLLOWING
// **
// **   ... loop started by sqlite3WhereBegin() ...
// **     if( new partition ){
// **       Gosub flush
// **     }
// **     Insert new row into eph table.
// **     if( first row of partition ){
// **       Rewind(csrEnd) ; Rewind(csrStart) ; Rewind(csrCurrent)
// **       regEnd = <expr2>
// **       regStart = regEnd - <expr1>
// **     }else{
// **       AGGSTEP
// **       if( (regEnd--)<=0 ){
// **         RETURN_ROW
// **       }
// **       if( (regStart--)<=0 ){
// **         AGGINVERSE
// **       }
// **     }
// **   }
// **   flush:
// **     AGGSTEP
// **     while( 1 ){
// **       if( (regEnd--)<=0 ){
// **         RETURN_ROW
// **         if( eof ) break;
// **       }
// **       if( (regStart--)<=0 ){
// **         AGGINVERSE
// **         if( eof ) break
// **       }
// **     }
// **     while( !eof csrCurrent ){
// **       RETURN_ROW
// **     }
// **
// ** For the most part, the patterns above are adapted to support UNBOUNDED by
// ** assuming that it is equivalent to "infinity PRECEDING/FOLLOWING" and
// ** CURRENT ROW by assuming that it is equivalent to "0 PRECEDING/FOLLOWING".
// ** This is optimized of course - branches that will never be taken and
// ** conditions that are always true are omitted from the VM code. The only
// ** exceptional case is:
// **
// **   ROWS BETWEEN <expr1> FOLLOWING AND UNBOUNDED FOLLOWING
// **
// **     ... loop started by sqlite3WhereBegin() ...
// **     if( new partition ){
// **       Gosub flush
// **     }
// **     Insert new row into eph table.
// **     if( first row of partition ){
// **       Rewind(csrEnd) ; Rewind(csrStart) ; Rewind(csrCurrent)
// **       regStart = <expr1>
// **     }else{
// **       AGGSTEP
// **     }
// **   }
// **   flush:
// **     AGGSTEP
// **     while( 1 ){
// **       if( (regStart--)<=0 ){
// **         AGGINVERSE
// **         if( eof ) break
// **       }
// **       RETURN_ROW
// **     }
// **     while( !eof csrCurrent ){
// **       RETURN_ROW
// **     }
// **
// ** Also requiring special handling are the cases:
// **
// **   ROWS BETWEEN <expr1> PRECEDING AND <expr2> PRECEDING
// **   ROWS BETWEEN <expr1> FOLLOWING AND <expr2> FOLLOWING
// **
// ** when (expr1 < expr2). This is detected at runtime, not by this function.
// ** To handle this case, the pseudo-code programs depicted above are modified
// ** slightly to be:
// **
// **     ... loop started by sqlite3WhereBegin() ...
// **     if( new partition ){
// **       Gosub flush
// **     }
// **     Insert new row into eph table.
// **     if( first row of partition ){
// **       Rewind(csrEnd) ; Rewind(csrStart) ; Rewind(csrCurrent)
// **       regEnd = <expr2>
// **       regStart = <expr1>
// **       if( regEnd < regStart ){
// **         RETURN_ROW
// **         delete eph table contents
// **         continue
// **       }
// **     ...
// **
// ** The new "continue" statement in the above jumps to the next iteration
// ** of the outer loop - the one started by sqlite3WhereBegin().
// **
// ** The various GROUPS cases are implemented using the same patterns as
// ** ROWS. The VM code is modified slightly so that:
// **
// **   1. The else branch in the main loop is only taken if the row just
// **      added to the ephemeral table is the start of a new group. In
// **      other words, it becomes:
// **
// **         ... loop started by sqlite3WhereBegin() ...
// **         if( new partition ){
// **           Gosub flush
// **         }
// **         Insert new row into eph table.
// **         if( first row of partition ){
// **           Rewind(csrEnd) ; Rewind(csrStart) ; Rewind(csrCurrent)
// **           regEnd = <expr2>
// **           regStart = <expr1>
// **         }else if( new group ){
// **           ...
// **         }
// **       }
// **
// **   2. Instead of processing a single row, each RETURN_ROW, AGGSTEP or
// **      AGGINVERSE step processes the current row of the relevant cursor and
// **      all subsequent rows belonging to the same group.
// **
// ** RANGE window frames are a little different again. As for GROUPS, the
// ** main loop runs once per group only. And RETURN_ROW, AGGSTEP and AGGINVERSE
// ** deal in groups instead of rows. As for ROWS and GROUPS, there are three
// ** basic cases:
// **
// **   RANGE BETWEEN <expr1> PRECEDING AND <expr2> FOLLOWING
// **
// **     ... loop started by sqlite3WhereBegin() ...
// **       if( new partition ){
// **         Gosub flush
// **       }
// **       Insert new row into eph table.
// **       if( first row of partition ){
// **         Rewind(csrEnd) ; Rewind(csrStart) ; Rewind(csrCurrent)
// **         regEnd = <expr2>
// **         regStart = <expr1>
// **       }else{
// **         AGGSTEP
// **         while( (csrCurrent.key + regEnd) < csrEnd.key ){
// **           RETURN_ROW
// **           while( csrStart.key + regStart) < csrCurrent.key ){
// **             AGGINVERSE
// **           }
// **         }
// **       }
// **     }
// **     flush:
// **       AGGSTEP
// **       while( 1 ){
// **         RETURN ROW
// **         if( csrCurrent is EOF ) break;
// **           while( csrStart.key + regStart) < csrCurrent.key ){
// **             AGGINVERSE
// **           }
// **         }
// **       }
// **
// ** In the above notation, "csr.key" means the current value of the ORDER BY
// ** expression (there is only ever 1 for a RANGE that uses an <expr> FOLLOWING
// ** or <expr PRECEDING) read from cursor csr.
// **
// **   RANGE BETWEEN <expr1> PRECEDING AND <expr2> PRECEDING
// **
// **     ... loop started by sqlite3WhereBegin() ...
// **       if( new partition ){
// **         Gosub flush
// **       }
// **       Insert new row into eph table.
// **       if( first row of partition ){
// **         Rewind(csrEnd) ; Rewind(csrStart) ; Rewind(csrCurrent)
// **         regEnd = <expr2>
// **         regStart = <expr1>
// **       }else{
// **         while( (csrEnd.key + regEnd) <= csrCurrent.key ){
// **           AGGSTEP
// **         }
// **         while( (csrStart.key + regStart) < csrCurrent.key ){
// **           AGGINVERSE
// **         }
// **         RETURN_ROW
// **       }
// **     }
// **     flush:
// **       while( (csrEnd.key + regEnd) <= csrCurrent.key ){
// **         AGGSTEP
// **       }
// **       while( (csrStart.key + regStart) < csrCurrent.key ){
// **         AGGINVERSE
// **       }
// **       RETURN_ROW
// **
// **   RANGE BETWEEN <expr1> FOLLOWING AND <expr2> FOLLOWING
// **
// **     ... loop started by sqlite3WhereBegin() ...
// **       if( new partition ){
// **         Gosub flush
// **       }
// **       Insert new row into eph table.
// **       if( first row of partition ){
// **         Rewind(csrEnd) ; Rewind(csrStart) ; Rewind(csrCurrent)
// **         regEnd = <expr2>
// **         regStart = <expr1>
// **       }else{
// **         AGGSTEP
// **         while( (csrCurrent.key + regEnd) < csrEnd.key ){
// **           while( (csrCurrent.key + regStart) > csrStart.key ){
// **             AGGINVERSE
// **           }
// **           RETURN_ROW
// **         }
// **       }
// **     }
// **     flush:
// **       AGGSTEP
// **       while( 1 ){
// **         while( (csrCurrent.key + regStart) > csrStart.key ){
// **           AGGINVERSE
// **           if( eof ) break "while( 1 )" loop.
// **         }
// **         RETURN_ROW
// **       }
// **       while( !eof csrCurrent ){
// **         RETURN_ROW
// **       }
// **
// ** The text above leaves out many details. Refer to the code and comments
// ** below for a more complete picture.
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3WindowCodeStep(
    mut pParse: *mut Parse,
    mut p: *mut Select,
    mut pWInfo: *mut WhereInfo,
    mut regGosub: i32,
    mut addrGosub: i32,
) {
    let mut pMWin: *mut Window = unsafe { (*p).pWin };
    let mut pOrderBy: *mut ExprList = unsafe { (*pMWin).pOrderBy };
    let mut v: *mut Vdbe = unsafe { sqlite3GetVdbe(pParse) };
    // /* Cursor used to write to eph. table */
    let mut csrWrite: i32 = 0 as i32;
    // /* Cursor of sub-select */
    let mut csrInput: i32 = unsafe {
        (*unsafe {
            unsafe { std::ptr::addr_of_mut!((*unsafe { (*p).pSrc }).a) as *mut SrcItem }
                .offset((0 as i32) as isize)
        })
        .iCursor
    };
    // /* Number of cols returned by sub */
    let mut nInput: i32 = (unsafe {
        (*unsafe {
            (*unsafe {
                unsafe { std::ptr::addr_of_mut!((*unsafe { (*p).pSrc }).a) as *mut SrcItem }
                    .offset((0 as i32) as isize)
            })
            .pSTab
        })
        .nCol
    }) as i32;
    // /* To iterate through sub cols */
    let mut iInput: i32 = 0 as i32;
    // /* Address of OP_Ne */
    let mut addrNe: i32 = 0 as i32;
    // /* Address of OP_Gosub to flush: */
    let mut addrGosubFlush: i32 = 0 as i32;
    // /* Address of OP_Integer */
    let mut addrInteger: i32 = 0 as i32;
    // /* Address of OP_Rewind in flush: */
    let mut addrEmpty: i32 = 0 as i32;
    // /* Array of registers holding new input row */
    let mut regNew: i32 = 0 as i32;
    // /* regNew array in record form */
    let mut regRecord: i32 = 0 as i32;
    // /* Peer values for new row (part of regNew) */
    let mut regNewPeer: i32 = 0 as i32;
    // /* Peer values for current row */
    let mut regPeer: i32 = 0 as i32;
    // /* Register for "Gosub flush_partition" */
    let mut regFlushPart: i32 = 0 as i32;
    // /* Context object for sub-routines */
    let mut s: WindowCodeArg = unsafe { std::mem::zeroed() };
    // /* Label just before sqlite3WhereEnd() code */
    let mut lblWhereEnd: i32 = 0 as i32;
    // /* Value of <expr> PRECEDING */
    let mut regStart: i32 = 0 as i32;
    // /* Value of <expr> FOLLOWING */
    let mut regEnd: i32 = 0 as i32;
    0 as i32;
    0 as i32;
    0 as i32;
    lblWhereEnd = unsafe { sqlite3VdbeMakeLabel(pParse) };
    // /* Fill in the context object */
    unsafe { memset(std::ptr::addr_of_mut!(s) as *mut (), 0 as i32, 72 as u64) };
    s.pParse = pParse;
    s.pMWin = pMWin;
    s.pVdbe = v;
    s.regGosub = regGosub;
    s.addrGosub = addrGosub;
    s.current.csr = unsafe { (*pMWin).iEphCsr };
    csrWrite = s.current.csr + (1 as i32);
    s.start.csr = s.current.csr + (2 as i32);
    s.end.csr = s.current.csr + (3 as i32);
    // /* Figure out when rows may be deleted from the ephemeral table. There
    //   ** are four options - they may never be deleted (eDelete==0), they may
    //   ** be deleted as soon as they are no longer part of the window frame
    //   ** (eDelete==WINDOW_AGGINVERSE), they may be deleted as after the row
    //   ** has been returned to the caller (WINDOW_RETURN_ROW), or they may
    //   ** be deleted after they enter the frame (WINDOW_AGGSTEP). */
    match ((unsafe { (*pMWin).eStart }) as u32) as i32 {
        87 => {
            let __v1182: bool;
            if (((unsafe { (*pMWin).eFrmType }) as u32) as i32) != (90 as i32) {
                __v1182 = windowExprGtZero(pParse, unsafe { (*pMWin).pStart }) != (0 as i32);
            } else {
                __v1182 = false as bool;
            }
            if __v1182 {
                s.eDelete = 1 as i32;
            }
        }
        91 => {
            if windowCacheFrame(pMWin) == (0 as i32) {
                if (((unsafe { (*pMWin).eEnd }) as u32) as i32) == (89 as i32) {
                    let __v1183: bool;
                    if (((unsafe { (*pMWin).eFrmType }) as u32) as i32) != (90 as i32) {
                        __v1183 = windowExprGtZero(pParse, unsafe { (*pMWin).pEnd }) != (0 as i32);
                    } else {
                        __v1183 = false as bool;
                    }
                    if __v1183 {
                        s.eDelete = 3 as i32;
                    }
                } else {
                    s.eDelete = 1 as i32;
                }
            }
        }
        _ => {
            s.eDelete = 2 as i32;
        }
    }
    // /* Allocate registers for the array of values from the sub-query, the
    //   ** same values in record form, and the rowid used to insert said record
    //   ** into the ephemeral table.  */
    regNew = (unsafe { (*pParse).nMem }) + (1 as i32);
    let __v1184: *mut Parse = pParse;
    let __v1185: i32 = unsafe { (*__v1184).nMem };
    let __v1186: i32 = __v1185 + nInput;
    unsafe {
        (*__v1184).nMem = __v1186;
    }
    let __v1187: *mut Parse = pParse;
    let __v1188: i32 = unsafe { (*__v1187).nMem };
    let __v1189: i32 = __v1188 + (1 as i32);
    unsafe {
        (*__v1187).nMem = __v1189;
    }
    regRecord = __v1189;
    let __v1190: *mut Parse = pParse;
    let __v1191: i32 = unsafe { (*__v1190).nMem };
    let __v1192: i32 = __v1191 + (1 as i32);
    unsafe {
        (*__v1190).nMem = __v1192;
    }
    s.regRowid = __v1192;
    // /* If the window frame contains an "<expr> PRECEDING" or "<expr> FOLLOWING"
    //   ** clause, allocate registers to store the results of evaluating each
    //   ** <expr>.  */
    if (((unsafe { (*pMWin).eStart }) as u32) as i32) == (89 as i32)
        || (((unsafe { (*pMWin).eStart }) as u32) as i32) == (87 as i32)
    {
        let __v1193: *mut Parse = pParse;
        let __v1194: i32 = unsafe { (*__v1193).nMem };
        let __v1195: i32 = __v1194 + (1 as i32);
        unsafe {
            (*__v1193).nMem = __v1195;
        }
        regStart = __v1195;
    }
    if (((unsafe { (*pMWin).eEnd }) as u32) as i32) == (89 as i32)
        || (((unsafe { (*pMWin).eEnd }) as u32) as i32) == (87 as i32)
    {
        let __v1196: *mut Parse = pParse;
        let __v1197: i32 = unsafe { (*__v1196).nMem };
        let __v1198: i32 = __v1197 + (1 as i32);
        unsafe {
            (*__v1196).nMem = __v1198;
        }
        regEnd = __v1198;
    }
    // /* If this is not a "ROWS BETWEEN ..." frame, then allocate arrays of
    //   ** registers to store copies of the ORDER BY expressions (peer values)
    //   ** for the main loop, and for each cursor (start, current and end). */
    if (((unsafe { (*pMWin).eFrmType }) as u32) as i32) != (77 as i32) {
        let mut nPeer: i32 = if pOrderBy != std::ptr::null_mut::<ExprList>() {
            unsafe { (*pOrderBy).nExpr }
        } else {
            0 as i32
        };
        regNewPeer = regNew + unsafe { (*pMWin).nBufferCol };
        if (unsafe { (*pMWin).pPartition }) != std::ptr::null_mut::<ExprList>() {
            let __v1199: i32 = regNewPeer;
            let __v1200: i32 = __v1199 + unsafe { (*unsafe { (*pMWin).pPartition }).nExpr };
            regNewPeer = __v1200;
        }
        regPeer = (unsafe { (*pParse).nMem }) + (1 as i32);
        let __v1201: *mut Parse = pParse;
        let __v1202: i32 = unsafe { (*__v1201).nMem };
        let __v1203: i32 = __v1202 + nPeer;
        unsafe {
            (*__v1201).nMem = __v1203;
        }
        s.start.reg = (unsafe { (*pParse).nMem }) + (1 as i32);
        let __v1204: *mut Parse = pParse;
        let __v1205: i32 = unsafe { (*__v1204).nMem };
        let __v1206: i32 = __v1205 + nPeer;
        unsafe {
            (*__v1204).nMem = __v1206;
        }
        s.current.reg = (unsafe { (*pParse).nMem }) + (1 as i32);
        let __v1207: *mut Parse = pParse;
        let __v1208: i32 = unsafe { (*__v1207).nMem };
        let __v1209: i32 = __v1208 + nPeer;
        unsafe {
            (*__v1207).nMem = __v1209;
        }
        s.end.reg = (unsafe { (*pParse).nMem }) + (1 as i32);
        let __v1210: *mut Parse = pParse;
        let __v1211: i32 = unsafe { (*__v1210).nMem };
        let __v1212: i32 = __v1211 + nPeer;
        unsafe {
            (*__v1210).nMem = __v1212;
        }
    }
    // /* Load the column values for the row returned by the sub-select
    //   ** into an array of registers starting at regNew. Assemble them into
    //   ** a record in register regRecord. */
    iInput = 0 as i32;
    '__slate_break_1141: loop {
        if !(iInput < nInput) {
            break;
        }
        unsafe { sqlite3VdbeAddOp3(v, 96 as i32, csrInput, iInput, regNew + iInput) };
        let __v1213: i32 = iInput;
        let __v1214: i32 = __v1213 + (1 as i32);
        iInput = __v1214;
    }
    unsafe { sqlite3VdbeAddOp3(v, 99 as i32, regNew, nInput, regRecord) };
    // /* An input row has just been read into an array of registers starting
    //   ** at regNew. If the window has a PARTITION clause, this block generates
    //   ** VM code to check if the input row is the start of a new partition.
    //   ** If so, it does an OP_Gosub to an address to be filled in later. The
    //   ** address of the OP_Gosub is stored in local variable addrGosubFlush. */
    if (unsafe { (*pMWin).pPartition }) != std::ptr::null_mut::<ExprList>() {
        let mut addr: i32 = 0 as i32;
        let mut pPart: *mut ExprList = unsafe { (*pMWin).pPartition };
        let mut nPart: i32 = unsafe { (*pPart).nExpr };
        let mut regNewPart: i32 = regNew + unsafe { (*pMWin).nBufferCol };
        let mut pKeyInfo: *mut KeyInfo =
            unsafe { sqlite3KeyInfoFromExprList(pParse, pPart, 0 as i32, 0 as i32) };
        let __v1215: *mut Parse = pParse;
        let __v1216: i32 = unsafe { (*__v1215).nMem };
        let __v1217: i32 = __v1216 + (1 as i32);
        unsafe {
            (*__v1215).nMem = __v1217;
        }
        regFlushPart = __v1217;
        addr = unsafe {
            sqlite3VdbeAddOp3(v, 92 as i32, regNewPart, unsafe { (*pMWin).regPart }, nPart)
        };
        unsafe { sqlite3VdbeAppendP4(v, pKeyInfo as *mut (), -(9 as i32)) };
        unsafe {
            sqlite3VdbeAddOp3(
                v,
                14 as i32,
                addr + (2 as i32),
                addr + (4 as i32),
                addr + (2 as i32),
            )
        };
        {}
        addrGosubFlush = unsafe { sqlite3VdbeAddOp1(v, 10 as i32, regFlushPart) };
        {}
        unsafe {
            sqlite3VdbeAddOp3(
                v,
                82 as i32,
                regNewPart,
                unsafe { (*pMWin).regPart },
                nPart - (1 as i32),
            )
        };
    }
    // /* Insert the new row into the ephemeral table */
    unsafe { sqlite3VdbeAddOp2(v, 129 as i32, csrWrite, s.regRowid) };
    unsafe { sqlite3VdbeAddOp3(v, 130 as i32, csrWrite, regRecord, s.regRowid) };
    addrNe = unsafe {
        sqlite3VdbeAddOp3(
            v,
            53 as i32,
            unsafe { (*pMWin).regOne },
            0 as i32,
            s.regRowid,
        )
    };
    {}
    // /* This block is run for the first row of each partition */
    s.regArg = windowInitAccum(pParse, pMWin);
    if regStart != (0 as i32) {
        unsafe { sqlite3ExprCode(pParse, unsafe { (*pMWin).pStart }, regStart) };
        windowCheckValue(
            pParse,
            regStart,
            (0 as i32)
                + if (((unsafe { (*pMWin).eFrmType }) as u32) as i32) == (90 as i32) {
                    3 as i32
                } else {
                    0 as i32
                },
        );
    }
    if regEnd != (0 as i32) {
        unsafe { sqlite3ExprCode(pParse, unsafe { (*pMWin).pEnd }, regEnd) };
        windowCheckValue(
            pParse,
            regEnd,
            (1 as i32)
                + if (((unsafe { (*pMWin).eFrmType }) as u32) as i32) == (90 as i32) {
                    3 as i32
                } else {
                    0 as i32
                },
        );
    }
    if (((unsafe { (*pMWin).eFrmType }) as u32) as i32) != (90 as i32)
        && (((unsafe { (*pMWin).eStart }) as u32) as i32)
            == (((unsafe { (*pMWin).eEnd }) as u32) as i32)
        && regStart != (0 as i32)
    {
        let mut op: i32 = if (((unsafe { (*pMWin).eStart }) as u32) as i32) == (87 as i32) {
            58 as i32
        } else {
            56 as i32
        };
        let mut addrGe: i32 = unsafe { sqlite3VdbeAddOp3(v, op, regStart, 0 as i32, regEnd) };
        // /* NeverNull because bound <expr> */
        {}
        // /*   values previously checked */
        {}
        windowAggFinal(std::ptr::addr_of_mut!(s), 0 as i32);
        unsafe { sqlite3VdbeAddOp1(v, 36 as i32, s.current.csr) };
        windowReturnOneRow(std::ptr::addr_of_mut!(s));
        unsafe { sqlite3VdbeAddOp1(v, 148 as i32, s.current.csr) };
        unsafe { sqlite3VdbeAddOp2(v, 9 as i32, 0 as i32, lblWhereEnd) };
        unsafe { sqlite3VdbeJumpHere(v, addrGe) };
    }
    if (((unsafe { (*pMWin).eStart }) as u32) as i32) == (87 as i32)
        && (((unsafe { (*pMWin).eFrmType }) as u32) as i32) != (90 as i32)
        && regEnd != (0 as i32)
    {
        0 as i32;
        unsafe { sqlite3VdbeAddOp3(v, 108 as i32, regStart, regEnd, regStart) };
    }
    if (((unsafe { (*pMWin).eStart }) as u32) as i32) != (91 as i32) {
        unsafe { sqlite3VdbeAddOp1(v, 36 as i32, s.start.csr) };
    }
    unsafe { sqlite3VdbeAddOp1(v, 36 as i32, s.current.csr) };
    unsafe { sqlite3VdbeAddOp1(v, 36 as i32, s.end.csr) };
    if regPeer != (0 as i32) && pOrderBy != std::ptr::null_mut::<ExprList>() {
        unsafe {
            sqlite3VdbeAddOp3(
                v,
                82 as i32,
                regNewPeer,
                regPeer,
                (unsafe { (*pOrderBy).nExpr }) - (1 as i32),
            )
        };
        unsafe {
            sqlite3VdbeAddOp3(
                v,
                82 as i32,
                regPeer,
                s.start.reg,
                (unsafe { (*pOrderBy).nExpr }) - (1 as i32),
            )
        };
        unsafe {
            sqlite3VdbeAddOp3(
                v,
                82 as i32,
                regPeer,
                s.current.reg,
                (unsafe { (*pOrderBy).nExpr }) - (1 as i32),
            )
        };
        unsafe {
            sqlite3VdbeAddOp3(
                v,
                82 as i32,
                regPeer,
                s.end.reg,
                (unsafe { (*pOrderBy).nExpr }) - (1 as i32),
            )
        };
    }
    unsafe { sqlite3VdbeAddOp2(v, 9 as i32, 0 as i32, lblWhereEnd) };
    unsafe { sqlite3VdbeJumpHere(v, addrNe) };
    // /* Beginning of the block executed for the second and subsequent rows. */
    if regPeer != (0 as i32) {
        windowIfNewPeer(pParse, pOrderBy, regNewPeer, regPeer, lblWhereEnd);
    }
    if (((unsafe { (*pMWin).eStart }) as u32) as i32) == (87 as i32) {
        windowCodeOp(std::ptr::addr_of_mut!(s), 3 as i32, 0 as i32, 0 as i32);
        if (((unsafe { (*pMWin).eEnd }) as u32) as i32) != (91 as i32) {
            if (((unsafe { (*pMWin).eFrmType }) as u32) as i32) == (90 as i32) {
                let mut lbl: i32 = unsafe { sqlite3VdbeMakeLabel(pParse) };
                let mut addrNext: i32 = unsafe { sqlite3VdbeCurrentAddr(v) };
                windowCodeRangeTest(
                    std::ptr::addr_of_mut!(s),
                    58 as i32,
                    s.current.csr,
                    regEnd,
                    s.end.csr,
                    lbl,
                );
                windowCodeOp(std::ptr::addr_of_mut!(s), 2 as i32, regStart, 0 as i32);
                windowCodeOp(std::ptr::addr_of_mut!(s), 1 as i32, 0 as i32, 0 as i32);
                unsafe { sqlite3VdbeAddOp2(v, 9 as i32, 0 as i32, addrNext) };
                unsafe { sqlite3VdbeResolveLabel(v, lbl) };
            } else {
                windowCodeOp(std::ptr::addr_of_mut!(s), 1 as i32, regEnd, 0 as i32);
                windowCodeOp(std::ptr::addr_of_mut!(s), 2 as i32, regStart, 0 as i32);
            }
        }
    } else {
        if (((unsafe { (*pMWin).eEnd }) as u32) as i32) == (89 as i32) {
            let mut bRPS: i32 = ((((unsafe { (*pMWin).eStart }) as u32) as i32) == (89 as i32)
                && (((unsafe { (*pMWin).eFrmType }) as u32) as i32) == (90 as i32))
                as i32;
            windowCodeOp(std::ptr::addr_of_mut!(s), 3 as i32, regEnd, 0 as i32);
            if bRPS != (0 as i32) {
                windowCodeOp(std::ptr::addr_of_mut!(s), 2 as i32, regStart, 0 as i32);
            }
            windowCodeOp(std::ptr::addr_of_mut!(s), 1 as i32, 0 as i32, 0 as i32);
            if !(bRPS != (0 as i32)) {
                windowCodeOp(std::ptr::addr_of_mut!(s), 2 as i32, regStart, 0 as i32);
            }
        } else {
            let mut addr: i32 = 0 as i32;
            windowCodeOp(std::ptr::addr_of_mut!(s), 3 as i32, 0 as i32, 0 as i32);
            if (((unsafe { (*pMWin).eEnd }) as u32) as i32) != (91 as i32) {
                if (((unsafe { (*pMWin).eFrmType }) as u32) as i32) == (90 as i32) {
                    let mut lbl: i32 = 0 as i32;
                    addr = unsafe { sqlite3VdbeCurrentAddr(v) };
                    if regEnd != (0 as i32) {
                        lbl = unsafe { sqlite3VdbeMakeLabel(pParse) };
                        windowCodeRangeTest(
                            std::ptr::addr_of_mut!(s),
                            58 as i32,
                            s.current.csr,
                            regEnd,
                            s.end.csr,
                            lbl,
                        );
                    }
                    windowCodeOp(std::ptr::addr_of_mut!(s), 1 as i32, 0 as i32, 0 as i32);
                    windowCodeOp(std::ptr::addr_of_mut!(s), 2 as i32, regStart, 0 as i32);
                    if regEnd != (0 as i32) {
                        unsafe { sqlite3VdbeAddOp2(v, 9 as i32, 0 as i32, addr) };
                        unsafe { sqlite3VdbeResolveLabel(v, lbl) };
                    }
                } else {
                    if regEnd != (0 as i32) {
                        addr =
                            unsafe { sqlite3VdbeAddOp3(v, 61 as i32, regEnd, 0 as i32, 1 as i32) };
                        {}
                    }
                    windowCodeOp(std::ptr::addr_of_mut!(s), 1 as i32, 0 as i32, 0 as i32);
                    windowCodeOp(std::ptr::addr_of_mut!(s), 2 as i32, regStart, 0 as i32);
                    if regEnd != (0 as i32) {
                        unsafe { sqlite3VdbeJumpHere(v, addr) };
                    }
                }
            }
        }
    }
    // /* End of the main input loop */
    unsafe { sqlite3VdbeResolveLabel(v, lblWhereEnd) };
    unsafe { sqlite3WhereEnd(pWInfo) };
    // /* Fall through */
    if (unsafe { (*pMWin).pPartition }) != std::ptr::null_mut::<ExprList>() {
        addrInteger = unsafe { sqlite3VdbeAddOp2(v, 73 as i32, 0 as i32, regFlushPart) };
        unsafe { sqlite3VdbeJumpHere(v, addrGosubFlush) };
    }
    s.regRowid = 0 as i32;
    addrEmpty = unsafe { sqlite3VdbeAddOp1(v, 36 as i32, csrWrite) };
    {}
    if (((unsafe { (*pMWin).eEnd }) as u32) as i32) == (89 as i32) {
        let mut bRPS: i32 = ((((unsafe { (*pMWin).eStart }) as u32) as i32) == (89 as i32)
            && (((unsafe { (*pMWin).eFrmType }) as u32) as i32) == (90 as i32))
            as i32;
        windowCodeOp(std::ptr::addr_of_mut!(s), 3 as i32, regEnd, 0 as i32);
        if bRPS != (0 as i32) {
            windowCodeOp(std::ptr::addr_of_mut!(s), 2 as i32, regStart, 0 as i32);
        }
        windowCodeOp(std::ptr::addr_of_mut!(s), 1 as i32, 0 as i32, 0 as i32);
    } else {
        if (((unsafe { (*pMWin).eStart }) as u32) as i32) == (87 as i32) {
            let mut addrStart: i32 = 0 as i32;
            let mut addrBreak1: i32 = 0 as i32;
            let mut addrBreak2: i32 = 0 as i32;
            let mut addrBreak3: i32 = 0 as i32;
            windowCodeOp(std::ptr::addr_of_mut!(s), 3 as i32, 0 as i32, 0 as i32);
            if (((unsafe { (*pMWin).eFrmType }) as u32) as i32) == (90 as i32) {
                addrStart = unsafe { sqlite3VdbeCurrentAddr(v) };
                addrBreak2 = windowCodeOp(std::ptr::addr_of_mut!(s), 2 as i32, regStart, 1 as i32);
                addrBreak1 = windowCodeOp(std::ptr::addr_of_mut!(s), 1 as i32, 0 as i32, 1 as i32);
            } else {
                if (((unsafe { (*pMWin).eEnd }) as u32) as i32) == (91 as i32) {
                    addrStart = unsafe { sqlite3VdbeCurrentAddr(v) };
                    addrBreak1 =
                        windowCodeOp(std::ptr::addr_of_mut!(s), 1 as i32, regStart, 1 as i32);
                    addrBreak2 =
                        windowCodeOp(std::ptr::addr_of_mut!(s), 2 as i32, 0 as i32, 1 as i32);
                } else {
                    0 as i32;
                    // /* assert( regStart>=0 );
                    //       ** regEnd = regEnd - regStart;
                    //       ** regStart = 0;   */
                    unsafe { sqlite3VdbeAddOp3(v, 108 as i32, regStart, regEnd, regEnd) };
                    unsafe { sqlite3VdbeAddOp2(v, 73 as i32, 0 as i32, regStart) };
                    addrStart = unsafe { sqlite3VdbeCurrentAddr(v) };
                    addrBreak1 =
                        windowCodeOp(std::ptr::addr_of_mut!(s), 1 as i32, regEnd, 1 as i32);
                    addrBreak2 =
                        windowCodeOp(std::ptr::addr_of_mut!(s), 2 as i32, regStart, 1 as i32);
                }
            }
            unsafe { sqlite3VdbeAddOp2(v, 9 as i32, 0 as i32, addrStart) };
            unsafe { sqlite3VdbeJumpHere(v, addrBreak2) };
            addrStart = unsafe { sqlite3VdbeCurrentAddr(v) };
            addrBreak3 = windowCodeOp(std::ptr::addr_of_mut!(s), 1 as i32, 0 as i32, 1 as i32);
            unsafe { sqlite3VdbeAddOp2(v, 9 as i32, 0 as i32, addrStart) };
            unsafe { sqlite3VdbeJumpHere(v, addrBreak1) };
            unsafe { sqlite3VdbeJumpHere(v, addrBreak3) };
        } else {
            let mut addrBreak: i32 = 0 as i32;
            let mut addrStart: i32 = 0 as i32;
            windowCodeOp(std::ptr::addr_of_mut!(s), 3 as i32, 0 as i32, 0 as i32);
            addrStart = unsafe { sqlite3VdbeCurrentAddr(v) };
            addrBreak = windowCodeOp(std::ptr::addr_of_mut!(s), 1 as i32, 0 as i32, 1 as i32);
            windowCodeOp(std::ptr::addr_of_mut!(s), 2 as i32, regStart, 0 as i32);
            unsafe { sqlite3VdbeAddOp2(v, 9 as i32, 0 as i32, addrStart) };
            unsafe { sqlite3VdbeJumpHere(v, addrBreak) };
        }
    }
    unsafe { sqlite3VdbeJumpHere(v, addrEmpty) };
    unsafe { sqlite3VdbeAddOp1(v, 148 as i32, s.current.csr) };
    if (unsafe { (*pMWin).pPartition }) != std::ptr::null_mut::<ExprList>() {
        if (unsafe { (*pMWin).regStartRowid }) != (0 as i32) {
            unsafe { sqlite3VdbeAddOp2(v, 73 as i32, 1 as i32, unsafe { (*pMWin).regStartRowid }) };
            unsafe { sqlite3VdbeAddOp2(v, 73 as i32, 0 as i32, unsafe { (*pMWin).regEndRowid }) };
        }
        unsafe { sqlite3VdbeChangeP1(v, addrInteger, unsafe { sqlite3VdbeCurrentAddr(v) }) };
        unsafe { sqlite3VdbeAddOp1(v, 69 as i32, regFlushPart) };
    }
}

// /*
// ** If the SELECT statement passed as the second argument does not invoke
// ** any SQL window functions, this function is a no-op. Otherwise, it
// ** rewrites the SELECT statement so that window function xStep functions
// ** are invoked in the correct order as described under "SELECT REWRITING"
// ** at the top of this file.
// */
// /* Parse context */
// /* Rewritten SELECT statement */
// /* Context returned by sqlite3WhereBegin() */
// /* Register for OP_Gosub */
// /* OP_Gosub here to return each row */
// /* SQLITE_OMIT_WINDOWFUNC */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3WindowRewrite(mut pParse: *mut Parse, mut p: *mut Select) -> i32 {
    let mut rc: i32 = 0 as i32;
    if (unsafe { (*p).pWin }) != std::ptr::null_mut::<Window>()
        && (unsafe { (*p).pPrior }) == std::ptr::null_mut::<Select>()
        && (unsafe { (*p).selFlags }) & ((1048576 as i32) as u32) == ((0 as i32) as u32)
        && !((((unsafe { (*pParse).eParseMode }) as u32) as i32) >= (2 as i32))
    {
        let mut v: *mut Vdbe = unsafe { sqlite3GetVdbe(pParse) };
        let mut db: *mut sqlite3 = unsafe { (*pParse).db };
        // /* The subquery */
        let mut pSub: *mut Select = std::ptr::null_mut::<Select>();
        let mut pSrc: *mut SrcList = unsafe { (*p).pSrc };
        let mut pWhere: *mut Expr = unsafe { (*p).pWhere };
        let mut pGroupBy: *mut ExprList = unsafe { (*p).pGroupBy };
        let mut pHaving: *mut Expr = unsafe { (*p).pHaving };
        let mut pSort: *mut ExprList = std::ptr::null_mut::<ExprList>();
        // /* Expression list for sub-query */
        let mut pSublist: *mut ExprList = std::ptr::null_mut::<ExprList>();
        // /* Main window object */
        let mut pMWin: *mut Window = unsafe { (*p).pWin };
        // /* Window object iterator */
        let mut pWin: *mut Window = unsafe { std::mem::zeroed() };
        let mut pTab: *mut Table = unsafe { std::mem::zeroed() };
        let mut w: Walker = unsafe { std::mem::zeroed() };
        let mut selFlags: u32 = unsafe { (*p).selFlags };
        pTab = (unsafe { sqlite3DbMallocZero(db, 120 as u64) }) as *mut Table;
        if pTab == std::ptr::null_mut::<Table>() {
            return unsafe { sqlite3ErrorToParser(db, 7 as i32) };
        }
        unsafe { sqlite3AggInfoPersistWalkerInit(std::ptr::addr_of_mut!(w), pParse) };
        unsafe { sqlite3WalkSelect(std::ptr::addr_of_mut!(w), p) };
        if (unsafe { (*p).selFlags }) & ((8 as i32) as u32) == ((0 as i32) as u32) {
            w.xExprCallback = Some(disallowAggregatesInOrderByCb);
            w.xSelectCallback = None;
            unsafe { sqlite3WalkExprList(std::ptr::addr_of_mut!(w), unsafe { (*p).pOrderBy }) };
        }
        unsafe {
            (*p).pSrc = std::ptr::null_mut::<SrcList>();
        }
        unsafe {
            (*p).pWhere = std::ptr::null_mut::<Expr>();
        }
        unsafe {
            (*p).pGroupBy = std::ptr::null_mut::<ExprList>();
        }
        unsafe {
            (*p).pHaving = std::ptr::null_mut::<Expr>();
        }
        let __v1218: *mut Select = p;
        let __v1219: u32 = unsafe { (*__v1218).selFlags };
        let __v1220: u32 = __v1219 & !((8 as i32) as u32);
        unsafe {
            (*__v1218).selFlags = __v1220;
        }
        let __v1221: *mut Select = p;
        let __v1222: u32 = unsafe { (*__v1221).selFlags };
        let __v1223: u32 = __v1222 | ((1048576 as i32) as u32);
        unsafe {
            (*__v1221).selFlags = __v1223;
        }
        // /* Create the ORDER BY clause for the sub-select. This is the concatenation
        //     ** of the window PARTITION and ORDER BY clauses. Then, if this makes it
        //     ** redundant, remove the ORDER BY from the parent SELECT.  */
        pSort = exprListAppendList(
            pParse,
            std::ptr::null_mut::<ExprList>(),
            unsafe { (*pMWin).pPartition },
            1 as i32,
        );
        pSort = exprListAppendList(pParse, pSort, unsafe { (*pMWin).pOrderBy }, 1 as i32);
        if pSort != std::ptr::null_mut::<ExprList>()
            && (unsafe { (*p).pOrderBy }) != std::ptr::null_mut::<ExprList>()
            && (unsafe { (*unsafe { (*p).pOrderBy }).nExpr }) <= unsafe { (*pSort).nExpr }
        {
            let mut nSave: i32 = unsafe { (*pSort).nExpr };
            unsafe {
                (*pSort).nExpr = unsafe { (*unsafe { (*p).pOrderBy }).nExpr };
            }
            if (unsafe {
                sqlite3ExprListCompare(
                    pSort as *const ExprList,
                    (unsafe { (*p).pOrderBy }) as *const ExprList,
                    -(1 as i32),
                )
            }) == (0 as i32)
            {
                unsafe { sqlite3ExprListDelete(db, unsafe { (*p).pOrderBy }) };
                unsafe {
                    (*p).pOrderBy = std::ptr::null_mut::<ExprList>();
                }
            }
            unsafe {
                (*pSort).nExpr = nSave;
            }
        }
        // /* Assign a cursor number for the ephemeral table used to buffer rows.
        //     ** The OpenEphemeral instruction is coded later, after it is known how
        //     ** many columns the table will have.  */
        let __v1224: *mut Parse = pParse;
        let __v1225: i32 = unsafe { (*__v1224).nTab };
        let __v1226: i32 = __v1225 + (1 as i32);
        unsafe {
            (*__v1224).nTab = __v1226;
        }
        unsafe {
            (*pMWin).iEphCsr = __v1225;
        }
        let __v1227: *mut Parse = pParse;
        let __v1228: i32 = unsafe { (*__v1227).nTab };
        let __v1229: i32 = __v1228 + (3 as i32);
        unsafe {
            (*__v1227).nTab = __v1229;
        }
        selectWindowRewriteEList(
            pParse,
            pMWin,
            pSrc,
            unsafe { (*p).pEList },
            pTab,
            std::ptr::addr_of_mut!(pSublist),
        );
        selectWindowRewriteEList(
            pParse,
            pMWin,
            pSrc,
            unsafe { (*p).pOrderBy },
            pTab,
            std::ptr::addr_of_mut!(pSublist),
        );
        unsafe {
            (*pMWin).nBufferCol = if pSublist != std::ptr::null_mut::<ExprList>() {
                unsafe { (*pSublist).nExpr }
            } else {
                0 as i32
            };
        }
        // /* Append the PARTITION BY and ORDER BY expressions to the to the
        //     ** sub-select expression list. They are required to figure out where
        //     ** boundaries for partitions and sets of peer rows lie.  */
        pSublist = exprListAppendList(pParse, pSublist, unsafe { (*pMWin).pPartition }, 0 as i32);
        pSublist = exprListAppendList(pParse, pSublist, unsafe { (*pMWin).pOrderBy }, 0 as i32);
        // /* Append the arguments passed to each window function to the
        //     ** sub-select expression list. Also allocate two registers for each
        //     ** window function - one for the accumulator, another for interim
        //     ** results.  */
        pWin = pMWin;
        '__slate_break_1111: while pWin != std::ptr::null_mut::<Window>() {
            let mut pArgs: *mut ExprList = unsafe { std::mem::zeroed() };
            0 as i32;
            0 as i32;
            pArgs = unsafe { (*unsafe { (*pWin).pOwner }).x.pList };
            if (unsafe { (*unsafe { (*pWin).pWFunc }).funcFlags }) & ((1048576 as i32) as u32)
                != (0 as u32)
            {
                selectWindowRewriteEList(
                    pParse,
                    pMWin,
                    pSrc,
                    pArgs,
                    pTab,
                    std::ptr::addr_of_mut!(pSublist),
                );
                unsafe {
                    (*pWin).iArgCol = if pSublist != std::ptr::null_mut::<ExprList>() {
                        unsafe { (*pSublist).nExpr }
                    } else {
                        0 as i32
                    };
                }
                unsafe {
                    (*pWin).bExprArgs = ((1 as i32) as i8) as u8;
                }
            } else {
                unsafe {
                    (*pWin).iArgCol = if pSublist != std::ptr::null_mut::<ExprList>() {
                        unsafe { (*pSublist).nExpr }
                    } else {
                        0 as i32
                    };
                }
                pSublist = exprListAppendList(pParse, pSublist, pArgs, 0 as i32);
            }
            if (unsafe { (*pWin).pFilter }) != std::ptr::null_mut::<Expr>() {
                let mut pFilter: *mut Expr = unsafe {
                    sqlite3ExprDup(db, (unsafe { (*pWin).pFilter }) as *const Expr, 0 as i32)
                };
                pSublist = unsafe { sqlite3ExprListAppend(pParse, pSublist, pFilter) };
            }
            let __v1230: *mut Parse = pParse;
            let __v1231: i32 = unsafe { (*__v1230).nMem };
            let __v1232: i32 = __v1231 + (1 as i32);
            unsafe {
                (*__v1230).nMem = __v1232;
            }
            unsafe {
                (*pWin).regAccum = __v1232;
            }
            let __v1233: *mut Parse = pParse;
            let __v1234: i32 = unsafe { (*__v1233).nMem };
            let __v1235: i32 = __v1234 + (1 as i32);
            unsafe {
                (*__v1233).nMem = __v1235;
            }
            unsafe {
                (*pWin).regResult = __v1235;
            }
            unsafe { sqlite3VdbeAddOp2(v, 77 as i32, 0 as i32, unsafe { (*pWin).regAccum }) };
            pWin = unsafe { (*pWin).pNextWin };
        }
        // /* If there is no ORDER BY or PARTITION BY clause, and the window
        //     ** function accepts zero arguments, and there are no other columns
        //     ** selected (e.g. "SELECT row_number() OVER () FROM t1"), it is possible
        //     ** that pSublist is still NULL here. Add a constant expression here to
        //     ** keep everything legal in this case.
        //     */
        if pSublist == std::ptr::null_mut::<ExprList>() {
            pSublist = unsafe {
                sqlite3ExprListAppend(pParse, std::ptr::null_mut::<ExprList>(), unsafe {
                    sqlite3ExprInt32(db, 0 as i32)
                })
            };
        }
        pSub = unsafe {
            sqlite3SelectNew(
                pParse,
                pSublist,
                pSrc,
                pWhere,
                pGroupBy,
                pHaving,
                pSort,
                (0 as i32) as u32,
                std::ptr::null_mut::<Expr>(),
            )
        };
        {}
        unsafe {
            (*p).pSrc = unsafe {
                sqlite3SrcListAppend(
                    pParse,
                    std::ptr::null_mut::<SrcList>(),
                    std::ptr::null_mut::<Token>(),
                    std::ptr::null_mut::<Token>(),
                )
            };
        }
        // /* Due to db->mallocFailed test inside
        //                                      ** of sqlite3DbMallocRawNN() called from
        //                                      ** sqlite3SrcListAppend() */
        0 as i32;
        if (unsafe { (*p).pSrc }) == std::ptr::null_mut::<SrcList>() {
            unsafe { sqlite3SelectDelete(db, pSub) };
        } else {
            if (unsafe {
                sqlite3SrcItemAttachSubquery(
                    pParse,
                    unsafe {
                        unsafe { std::ptr::addr_of_mut!((*unsafe { (*p).pSrc }).a) as *mut SrcItem }
                            .offset((0 as i32) as isize)
                    },
                    pSub,
                    0 as i32,
                )
            }) != (0 as i32)
            {
                let mut pTab2: *mut Table = unsafe { std::mem::zeroed() };
                unsafe {
                    (*unsafe {
                        unsafe { std::ptr::addr_of_mut!((*unsafe { (*p).pSrc }).a) as *mut SrcItem }
                            .offset((0 as i32) as isize)
                    })
                    .fg
                    .__slate_bits_0
                    .__set_isCorrelated((1 as i32) as u32);
                }
                unsafe { sqlite3SrcListAssignCursors(pParse, unsafe { (*p).pSrc }) };
                let __v1236: *mut Select = pSub;
                let __v1237: u32 = unsafe { (*__v1236).selFlags };
                let __v1238: u32 = __v1237 | (((64 as i32) | (134217728 as i32)) as u32);
                unsafe {
                    (*__v1236).selFlags = __v1238;
                }
                pTab2 = unsafe { sqlite3ResultSetOfSelect(pParse, pSub, (64 as i32) as i8) };
                let __v1239: *mut Select = pSub;
                let __v1240: u32 = unsafe { (*__v1239).selFlags };
                let __v1241: u32 = __v1240 | selFlags & ((8 as i32) as u32);
                unsafe {
                    (*__v1239).selFlags = __v1241;
                }
                if pTab2 == std::ptr::null_mut::<Table>() {
                    // /* Might actually be some other kind of error, but in that case
                    //         ** pParse->nErr will be set, so if SQLITE_NOMEM is set, we will get
                    //         ** the correct error message regardless. */
                    rc = 7 as i32;
                } else {
                    unsafe { memcpy(pTab as *mut (), pTab2 as *const (), 120 as u64) };
                    let __v1242: *mut Table = pTab;
                    let __v1243: u32 = unsafe { (*__v1242).tabFlags };
                    let __v1244: u32 = __v1243 | ((16384 as i32) as u32);
                    unsafe {
                        (*__v1242).tabFlags = __v1244;
                    }
                    unsafe {
                        (*unsafe {
                            unsafe {
                                std::ptr::addr_of_mut!((*unsafe { (*p).pSrc }).a) as *mut SrcItem
                            }
                            .offset((0 as i32) as isize)
                        })
                        .pSTab = pTab;
                    }
                    pTab = pTab2;
                    unsafe { memset(std::ptr::addr_of_mut!(w) as *mut (), 0 as i32, 48 as u64) };
                    w.xExprCallback = Some(sqlite3WindowExtraAggFuncDepth);
                    w.xSelectCallback = unsafe {
                        std::mem::transmute::<
                            *const (),
                            Option<unsafe extern "C-unwind" fn(*mut Walker, *mut Select) -> i32>,
                        >(sqlite3WalkerDepthIncrease as *const ())
                    };
                    w.xSelectCallback2 = unsafe {
                        std::mem::transmute::<
                            *const (),
                            Option<unsafe extern "C-unwind" fn(*mut Walker, *mut Select)>,
                        >(sqlite3WalkerDepthDecrease as *const ())
                    };
                    unsafe { sqlite3WalkSelect(std::ptr::addr_of_mut!(w), pSub) };
                }
            }
        }
        if (unsafe { (*db).mallocFailed }) != (0 as u8) {
            rc = 7 as i32;
        }
        // /* Defer deleting the temporary table pTab because if an error occurred,
        //     ** there could still be references to that table embedded in the
        //     ** result-set or ORDER BY clause of the SELECT statement p.  */
        unsafe {
            sqlite3ParserAddCleanup(
                pParse,
                unsafe {
                    std::mem::transmute::<
                        *const (),
                        Option<unsafe extern "C-unwind" fn(*mut sqlite3, *mut ())>,
                    >(sqlite3DbFree as *const ())
                },
                pTab as *mut (),
            )
        };
    }
    0 as i32;
    return rc;
}

// /*
// ** This function is called immediately after resolving the function name
// ** for a window function within a SELECT statement. Argument pList is a
// ** linked list of WINDOW definitions for the current SELECT statement.
// ** Argument pFunc is the function definition just resolved and pWin
// ** is the Window object representing the associated OVER clause. This
// ** function updates the contents of pWin as follows:
// **
// **   * If the OVER clause referred to a named window (as in "max(x) OVER win"),
// **     search list pList for a matching WINDOW definition, and update pWin
// **     accordingly. If no such WINDOW clause can be found, leave an error
// **     in pParse.
// **
// **   * If the function is a built-in window function that requires the
// **     window to be coerced (see "BUILT-IN WINDOW FUNCTIONS" at the top
// **     of this file), pWin is updated here.
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3WindowUpdate(
    mut pParse: *mut Parse,
    mut pList: *mut Window,
    mut pWin: *mut Window,
    mut pFunc: *mut FuncDef,
) {
    if (unsafe { (*pWin).zName }) != std::ptr::null_mut::<i8>()
        && (((unsafe { (*pWin).eFrmType }) as u32) as i32) == (0 as i32)
    {
        let mut p: *mut Window = windowFind(pParse, pList, (unsafe { (*pWin).zName }) as *const i8);
        if p == std::ptr::null_mut::<Window>() {
            return;
        }
        unsafe {
            (*pWin).pPartition = unsafe {
                sqlite3ExprListDup(
                    unsafe { (*pParse).db },
                    (unsafe { (*p).pPartition }) as *const ExprList,
                    0 as i32,
                )
            };
        }
        unsafe {
            (*pWin).pOrderBy = unsafe {
                sqlite3ExprListDup(
                    unsafe { (*pParse).db },
                    (unsafe { (*p).pOrderBy }) as *const ExprList,
                    0 as i32,
                )
            };
        }
        unsafe {
            (*pWin).pStart = unsafe {
                sqlite3ExprDup(
                    unsafe { (*pParse).db },
                    (unsafe { (*p).pStart }) as *const Expr,
                    0 as i32,
                )
            };
        }
        unsafe {
            (*pWin).pEnd = unsafe {
                sqlite3ExprDup(
                    unsafe { (*pParse).db },
                    (unsafe { (*p).pEnd }) as *const Expr,
                    0 as i32,
                )
            };
        }
        unsafe {
            (*pWin).eStart = unsafe { (*p).eStart };
        }
        unsafe {
            (*pWin).eEnd = unsafe { (*p).eEnd };
        }
        unsafe {
            (*pWin).eFrmType = unsafe { (*p).eFrmType };
        }
        unsafe {
            (*pWin).eExclude = unsafe { (*p).eExclude };
        }
    } else {
        sqlite3WindowChain(pParse, pWin, pList);
    }
    if (((unsafe { (*pWin).eFrmType }) as u32) as i32) == (90 as i32)
        && ((unsafe { (*pWin).pStart }) != std::ptr::null_mut::<Expr>()
            || (unsafe { (*pWin).pEnd }) != std::ptr::null_mut::<Expr>())
        && ((unsafe { (*pWin).pOrderBy }) == std::ptr::null_mut::<ExprList>()
            || (unsafe { (*unsafe { (*pWin).pOrderBy }).nExpr }) != (1 as i32))
    {
        unsafe {
            sqlite3ErrorMsg(
                pParse,
                (b"RANGE with offset PRECEDING/FOLLOWING requires one ORDER BY expression\0"
                    .as_ptr() as *mut i8) as *const i8,
            )
        };
    } else {
        if (unsafe { (*pFunc).funcFlags }) & ((65536 as i32) as u32) != (0 as u32) {
            let mut db: *mut sqlite3 = unsafe { (*pParse).db };
            if (unsafe { (*pWin).pFilter }) != std::ptr::null_mut::<Expr>() {
                unsafe {
                    sqlite3ErrorMsg(
                        pParse,
                        (b"FILTER clause may only be used with aggregate window functions\0"
                            .as_ptr() as *mut i8) as *const i8,
                    )
                };
            } else {
                let mut aUp: __SlateAlign16<[WindowUpdate; 8]> = __SlateAlign16([
                    WindowUpdate {
                        zFunc: unsafe { std::ptr::addr_of!(row_numberName) as *const i8 },
                        eFrmType: 77 as i32,
                        eStart: 91 as i32,
                        eEnd: 86 as i32,
                    },
                    WindowUpdate {
                        zFunc: unsafe { std::ptr::addr_of!(dense_rankName) as *const i8 },
                        eFrmType: 90 as i32,
                        eStart: 91 as i32,
                        eEnd: 86 as i32,
                    },
                    WindowUpdate {
                        zFunc: unsafe { std::ptr::addr_of!(rankName) as *const i8 },
                        eFrmType: 90 as i32,
                        eStart: 91 as i32,
                        eEnd: 86 as i32,
                    },
                    WindowUpdate {
                        zFunc: unsafe { std::ptr::addr_of!(percent_rankName) as *const i8 },
                        eFrmType: 93 as i32,
                        eStart: 86 as i32,
                        eEnd: 91 as i32,
                    },
                    WindowUpdate {
                        zFunc: unsafe { std::ptr::addr_of!(cume_distName) as *const i8 },
                        eFrmType: 93 as i32,
                        eStart: 87 as i32,
                        eEnd: 91 as i32,
                    },
                    WindowUpdate {
                        zFunc: unsafe { std::ptr::addr_of!(ntileName) as *const i8 },
                        eFrmType: 77 as i32,
                        eStart: 86 as i32,
                        eEnd: 91 as i32,
                    },
                    WindowUpdate {
                        zFunc: unsafe { std::ptr::addr_of!(leadName) as *const i8 },
                        eFrmType: 77 as i32,
                        eStart: 91 as i32,
                        eEnd: 91 as i32,
                    },
                    WindowUpdate {
                        zFunc: unsafe { std::ptr::addr_of!(lagName) as *const i8 },
                        eFrmType: 77 as i32,
                        eStart: 91 as i32,
                        eEnd: 86 as i32,
                    },
                ]);
                let mut i: i32 = 0 as i32;
                i = 0 as i32;
                '__slate_break_1104: loop {
                    if !(i < ((((192 as u64) / (24 as u64)) as u32) as i32)) {
                        break;
                    }
                    if (unsafe { (*pFunc).zName })
                        == unsafe {
                            (*unsafe {
                                (aUp.0.as_mut_ptr() as *mut WindowUpdate).offset(i as isize)
                            })
                            .zFunc
                        }
                    {
                        unsafe { sqlite3ExprDelete(db, unsafe { (*pWin).pStart }) };
                        unsafe { sqlite3ExprDelete(db, unsafe { (*pWin).pEnd }) };
                        unsafe {
                            (*pWin).pStart = std::ptr::null_mut::<Expr>();
                        }
                        unsafe {
                            (*pWin).pEnd = std::ptr::null_mut::<Expr>();
                        }
                        unsafe {
                            (*pWin).eFrmType = ((unsafe {
                                (*unsafe {
                                    (aUp.0.as_mut_ptr() as *mut WindowUpdate).offset(i as isize)
                                })
                                .eFrmType
                            }) as i8) as u8;
                        }
                        unsafe {
                            (*pWin).eStart = ((unsafe {
                                (*unsafe {
                                    (aUp.0.as_mut_ptr() as *mut WindowUpdate).offset(i as isize)
                                })
                                .eStart
                            }) as i8) as u8;
                        }
                        unsafe {
                            (*pWin).eEnd = ((unsafe {
                                (*unsafe {
                                    (aUp.0.as_mut_ptr() as *mut WindowUpdate).offset(i as isize)
                                })
                                .eEnd
                            }) as i8) as u8;
                        }
                        unsafe {
                            (*pWin).eExclude = ((0 as i32) as i8) as u8;
                        }
                        if (((unsafe { (*pWin).eStart }) as u32) as i32) == (87 as i32) {
                            unsafe {
                                (*pWin).pStart = unsafe { sqlite3ExprInt32(db, 1 as i32) };
                            }
                        }
                        break '__slate_break_1104;
                    }
                    let __v1245: i32 = i;
                    let __v1246: i32 = __v1245 + (1 as i32);
                    i = __v1246;
                }
            }
        }
    }
    unsafe {
        (*pWin).pWFunc = pFunc;
    }
}

// /* Context object */
// /* WINDOW_RETURN_ROW, AGGSTEP or AGGINVERSE */
// /* Register for OP_IfPos countdown */
// /* Jump here if stepped cursor reaches EOF */
// /*
// ** Allocate and return a duplicate of the Window object indicated by the
// ** third argument. Set the Window.pOwner field of the new object to
// ** pOwner.
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3WindowDup(
    mut db: *mut sqlite3,
    mut pOwner: *mut Expr,
    mut p: *mut Window,
) -> *mut Window {
    let mut pNew: *mut Window = std::ptr::null_mut::<Window>();
    if p != std::ptr::null_mut::<Window>() {
        pNew = (unsafe { sqlite3DbMallocZero(db, 144 as u64) }) as *mut Window;
        if pNew != std::ptr::null_mut::<Window>() {
            unsafe {
                (*pNew).zName =
                    unsafe { sqlite3DbStrDup(db, (unsafe { (*p).zName }) as *const i8) };
            }
            unsafe {
                (*pNew).zBase =
                    unsafe { sqlite3DbStrDup(db, (unsafe { (*p).zBase }) as *const i8) };
            }
            unsafe {
                (*pNew).pFilter = unsafe {
                    sqlite3ExprDup(db, (unsafe { (*p).pFilter }) as *const Expr, 0 as i32)
                };
            }
            unsafe {
                (*pNew).pWFunc = unsafe { (*p).pWFunc };
            }
            unsafe {
                (*pNew).pPartition = unsafe {
                    sqlite3ExprListDup(
                        db,
                        (unsafe { (*p).pPartition }) as *const ExprList,
                        0 as i32,
                    )
                };
            }
            unsafe {
                (*pNew).pOrderBy = unsafe {
                    sqlite3ExprListDup(db, (unsafe { (*p).pOrderBy }) as *const ExprList, 0 as i32)
                };
            }
            unsafe {
                (*pNew).eFrmType = unsafe { (*p).eFrmType };
            }
            unsafe {
                (*pNew).eEnd = unsafe { (*p).eEnd };
            }
            unsafe {
                (*pNew).eStart = unsafe { (*p).eStart };
            }
            unsafe {
                (*pNew).eExclude = unsafe { (*p).eExclude };
            }
            unsafe {
                (*pNew).regResult = unsafe { (*p).regResult };
            }
            unsafe {
                (*pNew).regAccum = unsafe { (*p).regAccum };
            }
            unsafe {
                (*pNew).iArgCol = unsafe { (*p).iArgCol };
            }
            unsafe {
                (*pNew).iEphCsr = unsafe { (*p).iEphCsr };
            }
            unsafe {
                (*pNew).bExprArgs = unsafe { (*p).bExprArgs };
            }
            unsafe {
                (*pNew).pStart = unsafe {
                    sqlite3ExprDup(db, (unsafe { (*p).pStart }) as *const Expr, 0 as i32)
                };
            }
            unsafe {
                (*pNew).pEnd =
                    unsafe { sqlite3ExprDup(db, (unsafe { (*p).pEnd }) as *const Expr, 0 as i32) };
            }
            unsafe {
                (*pNew).pOwner = pOwner;
            }
            unsafe {
                (*pNew).bImplicitFrame = unsafe { (*p).bImplicitFrame };
            }
        }
    }
    return pNew;
}

// /*
// ** Return a copy of the linked list of Window objects passed as the
// ** second argument.
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3WindowListDup(mut db: *mut sqlite3, mut p: *mut Window) -> *mut Window {
    let mut pWin: *mut Window = unsafe { std::mem::zeroed() };
    let mut pRet: *mut Window = std::ptr::null_mut::<Window>();
    let mut pp: *mut *mut Window = std::ptr::addr_of_mut!(pRet);
    pWin = p;
    '__slate_break_1139: while pWin != std::ptr::null_mut::<Window>() {
        unsafe {
            *pp = sqlite3WindowDup(db, std::ptr::null_mut::<Expr>(), pWin);
        }
        if (unsafe { *pp }) == std::ptr::null_mut::<Window>() {
            break '__slate_break_1139;
        }
        pp = unsafe { std::ptr::addr_of_mut!((*unsafe { *pp }).pNextWin) };
        pWin = unsafe { (*pWin).pNextWin };
    }
    return pRet;
}

// /*no-op*/
// /* Window functions that use all window interfaces: xStep, xFinal,
// ** xValue, and xInverse */
// /* Window functions that are implemented using bytecode and thus have
// ** no-op routines for their methods */
// /* Window functions that use all window interfaces: xStep, the
// ** same routine for xFinalize and xValue and which never call
// ** xInverse. */
// /*
// ** Register those built-in window functions that are not also aggregates.
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3WindowFunctions() {
    unsafe {
        sqlite3InsertBuiltinFuncs(
            unsafe { std::ptr::addr_of_mut!(aWindowFuncs.0) as *mut FuncDef },
            (((1080 as u64) / (72 as u64)) as u32) as i32,
        )
    };
}

// /*
// ** Window *pWin has just been created from a WINDOW clause. Token pBase
// ** is the base window. Earlier windows from the same WINDOW clause are
// ** stored in the linked list starting at pWin->pNextWin. This function
// ** either updates *pWin according to the base specification, or else
// ** leaves an error in pParse.
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3WindowChain(
    mut pParse: *mut Parse,
    mut pWin: *mut Window,
    mut pList: *mut Window,
) {
    if (unsafe { (*pWin).zBase }) != std::ptr::null_mut::<i8>() {
        let mut db: *mut sqlite3 = unsafe { (*pParse).db };
        let mut pExist: *mut Window =
            windowFind(pParse, pList, (unsafe { (*pWin).zBase }) as *const i8);
        if pExist != std::ptr::null_mut::<Window>() {
            let mut zErr: *const i8 = std::ptr::null::<i8>();
            // /* Check for errors */
            if (unsafe { (*pWin).pPartition }) != std::ptr::null_mut::<ExprList>() {
                zErr = (b"PARTITION clause\0".as_ptr() as *mut i8) as *const i8;
            } else {
                if (unsafe { (*pExist).pOrderBy }) != std::ptr::null_mut::<ExprList>()
                    && (unsafe { (*pWin).pOrderBy }) != std::ptr::null_mut::<ExprList>()
                {
                    zErr = (b"ORDER BY clause\0".as_ptr() as *mut i8) as *const i8;
                } else {
                    if (((unsafe { (*pExist).bImplicitFrame }) as u32) as i32) == (0 as i32) {
                        zErr = (b"frame specification\0".as_ptr() as *mut i8) as *const i8;
                    }
                }
            }
            if zErr != std::ptr::null::<i8>() {
                unsafe {
                    sqlite3ErrorMsg(
                        pParse,
                        (b"cannot override %s of window: %s\0".as_ptr() as *mut i8) as *const i8,
                        zErr,
                        unsafe { (*pWin).zBase },
                    )
                };
            } else {
                unsafe {
                    (*pWin).pPartition = unsafe {
                        sqlite3ExprListDup(
                            db,
                            (unsafe { (*pExist).pPartition }) as *const ExprList,
                            0 as i32,
                        )
                    };
                }
                if (unsafe { (*pExist).pOrderBy }) != std::ptr::null_mut::<ExprList>() {
                    0 as i32;
                    unsafe {
                        (*pWin).pOrderBy = unsafe {
                            sqlite3ExprListDup(
                                db,
                                (unsafe { (*pExist).pOrderBy }) as *const ExprList,
                                0 as i32,
                            )
                        };
                    }
                }
                unsafe { sqlite3DbFree(db, (unsafe { (*pWin).zBase }) as *mut ()) };
                unsafe {
                    (*pWin).zBase = std::ptr::null_mut::<i8>();
                }
            }
        }
    }
}

// /* Parsing context */
// /* Frame type. TK_RANGE, TK_ROWS, TK_GROUPS, or 0 */
// /* Start type: CURRENT, PRECEDING, FOLLOWING, UNBOUNDED */
// /* Start window size if TK_PRECEDING or FOLLOWING */
// /* End type: CURRENT, FOLLOWING, TK_UNBOUNDED, PRECEDING */
// /* End window size if TK_FOLLOWING or PRECEDING */
// /* EXCLUDE clause */
// /*
// ** Attach PARTITION and ORDER BY clauses pPartition and pOrderBy to window
// ** pWin. Also, if parameter pBase is not NULL, set pWin->zBase to the
// ** equivalent nul-terminated string.
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3WindowAssemble(
    mut pParse: *mut Parse,
    mut pWin: *mut Window,
    mut pPartition: *mut ExprList,
    mut pOrderBy: *mut ExprList,
    mut pBase: *mut Token,
) -> *mut Window {
    if pWin != std::ptr::null_mut::<Window>() {
        unsafe {
            (*pWin).pPartition = pPartition;
        }
        unsafe {
            (*pWin).pOrderBy = pOrderBy;
        }
        if pBase != std::ptr::null_mut::<Token>() {
            unsafe {
                (*pWin).zBase = unsafe {
                    sqlite3DbStrNDup(
                        unsafe { (*pParse).db },
                        unsafe { (*pBase).z },
                        (unsafe { (*pBase).n }) as u64,
                    )
                };
            }
        }
    } else {
        unsafe { sqlite3ExprListDelete(unsafe { (*pParse).db }, pPartition) };
        unsafe { sqlite3ExprListDelete(unsafe { (*pParse).db }, pOrderBy) };
    }
    return pWin;
}

// /*
// ** SELECT REWRITING
// **
// **   Any SELECT statement that contains one or more window functions in
// **   either the select list or ORDER BY clause (the only two places window
// **   functions may be used) is transformed by function sqlite3WindowRewrite()
// **   in order to support window function processing. For example, with the
// **   schema:
// **
// **     CREATE TABLE t1(a, b, c, d, e, f, g);
// **
// **   the statement:
// **
// **     SELECT a+1, max(b) OVER (PARTITION BY c ORDER BY d) FROM t1 ORDER BY e;
// **
// **   is transformed to:
// **
// **     SELECT a+1, max(b) OVER (PARTITION BY c ORDER BY d) FROM (
// **         SELECT a, e, c, d, b FROM t1 ORDER BY c, d
// **     ) ORDER BY e;
// **
// **   The flattening optimization is disabled when processing this transformed
// **   SELECT statement. This allows the implementation of the window function
// **   (in this case max()) to process rows sorted in order of (c, d), which
// **   makes things easier for obvious reasons. More generally:
// **
// **     * FROM, WHERE, GROUP BY and HAVING clauses are all moved to
// **       the sub-query.
// **
// **     * ORDER BY, LIMIT and OFFSET remain part of the parent query.
// **
// **     * Terminals from each of the expression trees that make up the
// **       select-list and ORDER BY expressions in the parent query are
// **       selected by the sub-query. For the purposes of the transformation,
// **       terminals are column references and aggregate functions.
// **
// **   If there is more than one window function in the SELECT that uses
// **   the same window declaration (the OVER bit), then a single scan may
// **   be used to process more than one window function. For example:
// **
// **     SELECT max(b) OVER (PARTITION BY c ORDER BY d),
// **            min(e) OVER (PARTITION BY c ORDER BY d)
// **     FROM t1;
// **
// **   is transformed in the same way as the example above. However:
// **
// **     SELECT max(b) OVER (PARTITION BY c ORDER BY d),
// **            min(e) OVER (PARTITION BY a ORDER BY b)
// **     FROM t1;
// **
// **   Must be transformed to:
// **
// **     SELECT max(b) OVER (PARTITION BY c ORDER BY d) FROM (
// **         SELECT e, min(e) OVER (PARTITION BY a ORDER BY b), c, d, b FROM
// **           SELECT a, e, c, d, b FROM t1 ORDER BY a, b
// **         ) ORDER BY c, d
// **     ) ORDER BY e;
// **
// **   so that both min() and max() may process rows in the order defined by
// **   their respective window declarations.
// **
// ** INTERFACE WITH SELECT.C
// **
// **   When processing the rewritten SELECT statement, code in select.c calls
// **   sqlite3WhereBegin() to begin iterating through the results of the
// **   sub-query, which is always implemented as a co-routine. It then calls
// **   sqlite3WindowCodeStep() to process rows and finish the scan by calling
// **   sqlite3WhereEnd().
// **
// **   sqlite3WindowCodeStep() generates VM code so that, for each row returned
// **   by the sub-query a sub-routine (OP_Gosub) coded by select.c is invoked.
// **   When the sub-routine is invoked:
// **
// **     * The results of all window-functions for the row are stored
// **       in the associated Window.regResult registers.
// **
// **     * The required terminal values are stored in the current row of
// **       temp table Window.iEphCsr.
// **
// **   In some cases, depending on the window frame and the specific window
// **   functions invoked, sqlite3WindowCodeStep() caches each entire partition
// **   in a temp table before returning any rows. In other cases it does not.
// **   This detail is encapsulated within this file, the code generated by
// **   select.c is the same in either case.
// **
// ** BUILT-IN WINDOW FUNCTIONS
// **
// **   This implementation features the following built-in window functions:
// **
// **     row_number()
// **     rank()
// **     dense_rank()
// **     percent_rank()
// **     cume_dist()
// **     ntile(N)
// **     lead(expr [, offset [, default]])
// **     lag(expr [, offset [, default]])
// **     first_value(expr)
// **     last_value(expr)
// **     nth_value(expr, N)
// **
// **   These are the same built-in window functions supported by Postgres.
// **   Although the behaviour of aggregate window functions (functions that
// **   can be used as either aggregates or window functions) allows them to
// **   be implemented using an API, built-in window functions are much more
// **   esoteric. Additionally, some window functions (e.g. nth_value())
// **   may only be implemented by caching the entire partition in memory.
// **   As such, some built-in window functions use the same API as aggregate
// **   window functions and some are implemented directly using VDBE
// **   instructions. Additionally, for those functions that use the API, the
// **   window frame is sometimes modified before the SELECT statement is
// **   rewritten. For example, regardless of the specified window frame, the
// **   row_number() function always uses:
// **
// **     ROWS BETWEEN UNBOUNDED PRECEDING AND CURRENT ROW
// **
// **   See sqlite3WindowUpdate() for details.
// **
// **   As well as some of the built-in window functions, aggregate window
// **   functions min() and max() are implemented using VDBE instructions if
// **   the start of the window frame is declared as anything other than
// **   UNBOUNDED PRECEDING.
// */
// /*
// ** Implementation of built-in window function row_number(). Assumes that the
// ** window frame has been coerced to:
// **
// **   ROWS BETWEEN UNBOUNDED PRECEDING AND CURRENT ROW
// */
#[unsafe(link_section = ".text.slate_distinct.window.row_numberStepFunc")]
extern "C-unwind" fn row_numberStepFunc(
    mut pCtx: *mut sqlite3_context,
    mut nArg: i32,
    mut apArg: *mut *mut sqlite3_value,
) {
    let mut p: *mut i64 =
        (unsafe { sqlite3_aggregate_context(pCtx, ((8 as u64) as u32) as i32) }) as *mut i64;
    if p != std::ptr::null_mut::<i64>() {
        let __v1247: *mut i64 = p;
        let __v1248: i64 = unsafe { *__v1247 };
        let __v1249: i64 = __v1248 + ((1 as i32) as i64);
        unsafe {
            *__v1247 = __v1249;
        }
    }
    nArg;
    apArg;
}

#[unsafe(link_section = ".text.slate_distinct.window.row_numberValueFunc")]
extern "C-unwind" fn row_numberValueFunc(mut pCtx: *mut sqlite3_context) {
    let mut p: *mut i64 =
        (unsafe { sqlite3_aggregate_context(pCtx, ((8 as u64) as u32) as i32) }) as *mut i64;
    unsafe {
        sqlite3_result_int64(
            pCtx,
            if p != std::ptr::null_mut::<i64>() {
                unsafe { *p }
            } else {
                (0 as i32) as i64
            },
        )
    };
}

// /*
// ** Implementation of built-in window function dense_rank(). Assumes that
// ** the window frame has been set to:
// **
// **   RANGE BETWEEN UNBOUNDED PRECEDING AND CURRENT ROW
// */
#[unsafe(link_section = ".text.slate_distinct.window.dense_rankStepFunc")]
extern "C-unwind" fn dense_rankStepFunc(
    mut pCtx: *mut sqlite3_context,
    mut nArg: i32,
    mut apArg: *mut *mut sqlite3_value,
) {
    let mut p: *mut CallCount = unsafe { std::mem::zeroed() };
    p = (unsafe { sqlite3_aggregate_context(pCtx, ((24 as u64) as u32) as i32) }) as *mut CallCount;
    if p != std::ptr::null_mut::<CallCount>() {
        unsafe {
            (*p).nStep = (1 as i32) as i64;
        }
    }
    nArg;
    apArg;
}

#[unsafe(link_section = ".text.slate_distinct.window.dense_rankValueFunc")]
extern "C-unwind" fn dense_rankValueFunc(mut pCtx: *mut sqlite3_context) {
    let mut p: *mut CallCount = unsafe { std::mem::zeroed() };
    p = (unsafe { sqlite3_aggregate_context(pCtx, ((24 as u64) as u32) as i32) }) as *mut CallCount;
    if p != std::ptr::null_mut::<CallCount>() {
        if (unsafe { (*p).nStep }) != (0 as i64) {
            let __v1250: *mut CallCount = p;
            let __v1251: i64 = unsafe { (*__v1250).nValue };
            let __v1252: i64 = __v1251 + ((1 as i32) as i64);
            unsafe {
                (*__v1250).nValue = __v1252;
            }
            unsafe {
                (*p).nStep = (0 as i32) as i64;
            }
        }
        unsafe { sqlite3_result_int64(pCtx, unsafe { (*p).nValue }) };
    }
}

#[unsafe(link_section = ".text.slate_distinct.window.nth_valueStepFunc")]
extern "C-unwind" fn nth_valueStepFunc(
    mut pCtx: *mut sqlite3_context,
    mut nArg: i32,
    mut apArg: *mut *mut sqlite3_value,
) {
    let mut __slate_storage_1255: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1255: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_1255) as *mut i64;
    let mut __slate_storage_1254: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1254: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_1254) as *mut i64;
    let mut __slate_storage_1253: std::mem::MaybeUninit<*mut NthValueCtx> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1253: *mut *mut NthValueCtx =
        std::ptr::addr_of_mut!(__slate_storage_1253) as *mut *mut NthValueCtx;
    let mut __slate_storage_424: std::mem::MaybeUninit<f64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_424: *mut f64 = std::ptr::addr_of_mut!(__slate_storage_424) as *mut f64;
    let mut __slate_storage_423: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_423: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_423) as *mut i64;
    let mut __slate_storage_422: std::mem::MaybeUninit<*mut NthValueCtx> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_422: *mut *mut NthValueCtx =
        std::ptr::addr_of_mut!(__slate_storage_422) as *mut *mut NthValueCtx;
    unsafe {
        '__slate_dispatch: {
            '__join_1: {
                *__slate_slot_422 =
                    (unsafe { sqlite3_aggregate_context(pCtx, ((16 as u64) as u32) as i32) })
                        as *mut NthValueCtx;
                if *__slate_slot_422 != std::ptr::null_mut::<NthValueCtx>() {
                    '__join_0: {
                        let __t0: i32 = unsafe {
                            sqlite3_value_numeric_type(unsafe {
                                *unsafe { apArg.offset((1 as i32) as isize) }
                            })
                        };
                        if __t0 == (1 as i32) {
                            *__slate_slot_423 = unsafe {
                                sqlite3_value_int64(unsafe {
                                    *unsafe { apArg.offset((1 as i32) as isize) }
                                })
                            };
                        } else {
                            if __t0 == (2 as i32) {
                                std::ptr::write(__slate_slot_424, unsafe {
                                    sqlite3_value_double(unsafe {
                                        *unsafe { apArg.offset((1 as i32) as isize) }
                                    })
                                });
                                if ((unsafe { sqlite3RealToI64(*__slate_slot_424) }) as f64)
                                    != *__slate_slot_424
                                {
                                    break '__join_0;
                                } else {
                                    *__slate_slot_423 = *__slate_slot_424 as i64;
                                }
                            } else {
                                break '__join_0;
                            }
                        }
                        if *__slate_slot_423 <= ((0 as i32) as i64) {
                        } else {
                            std::ptr::write(__slate_slot_1253, *__slate_slot_422);
                            std::ptr::write(__slate_slot_1254, unsafe {
                                (*(*__slate_slot_1253)).nStep
                            });
                            std::ptr::write(
                                __slate_slot_1255,
                                *__slate_slot_1254 + ((1 as i32) as i64),
                            );
                            unsafe {
                                (*(*__slate_slot_1253)).nStep = *__slate_slot_1255;
                            }
                            if *__slate_slot_423 == unsafe { (*(*__slate_slot_422)).nStep } {
                                unsafe {
                                    (*(*__slate_slot_422)).pValue = unsafe {
                                        sqlite3_value_dup(
                                            (unsafe {
                                                *unsafe { apArg.offset((0 as i32) as isize) }
                                            })
                                                as *const sqlite3_value,
                                        )
                                    };
                                }
                                if !((unsafe { (*(*__slate_slot_422)).pValue })
                                    != std::ptr::null_mut::<sqlite3_value>())
                                {
                                    unsafe { sqlite3_result_error_nomem(pCtx) };
                                    break '__join_1;
                                } else {
                                    break '__join_1;
                                }
                            } else {
                                break '__join_1;
                            }
                        }
                    }
                    unsafe {
                        sqlite3_result_error(
                            pCtx,
                            (b"second argument to nth_value must be a positive integer\0".as_ptr()
                                as *mut i8) as *const i8,
                            -(1 as i32),
                        )
                    };
                    break '__slate_dispatch;
                }
            }
            nArg;
            apArg;
            return;
        }
    }
}

#[unsafe(link_section = ".text.slate_distinct.window.nth_valueFinalizeFunc")]
extern "C-unwind" fn nth_valueFinalizeFunc(mut pCtx: *mut sqlite3_context) {
    let mut p: *mut NthValueCtx = unsafe { std::mem::zeroed() };
    p = (unsafe { sqlite3_aggregate_context(pCtx, 0 as i32) }) as *mut NthValueCtx;
    if p != std::ptr::null_mut::<NthValueCtx>()
        && (unsafe { (*p).pValue }) != std::ptr::null_mut::<sqlite3_value>()
    {
        unsafe { sqlite3_result_value(pCtx, unsafe { (*p).pValue }) };
        unsafe { sqlite3_value_free(unsafe { (*p).pValue }) };
        unsafe {
            (*p).pValue = std::ptr::null_mut::<sqlite3_value>();
        }
    }
}

#[unsafe(link_section = ".text.slate_distinct.window.first_valueStepFunc")]
extern "C-unwind" fn first_valueStepFunc(
    mut pCtx: *mut sqlite3_context,
    mut nArg: i32,
    mut apArg: *mut *mut sqlite3_value,
) {
    let mut p: *mut NthValueCtx = unsafe { std::mem::zeroed() };
    p = (unsafe { sqlite3_aggregate_context(pCtx, ((16 as u64) as u32) as i32) })
        as *mut NthValueCtx;
    if p != std::ptr::null_mut::<NthValueCtx>()
        && (unsafe { (*p).pValue }) == std::ptr::null_mut::<sqlite3_value>()
    {
        unsafe {
            (*p).pValue = unsafe {
                sqlite3_value_dup(
                    (unsafe { *unsafe { apArg.offset((0 as i32) as isize) } })
                        as *const sqlite3_value,
                )
            };
        }
        if !((unsafe { (*p).pValue }) != std::ptr::null_mut::<sqlite3_value>()) {
            unsafe { sqlite3_result_error_nomem(pCtx) };
        }
    }
    nArg;
    apArg;
}

#[unsafe(link_section = ".text.slate_distinct.window.first_valueFinalizeFunc")]
extern "C-unwind" fn first_valueFinalizeFunc(mut pCtx: *mut sqlite3_context) {
    let mut p: *mut NthValueCtx = unsafe { std::mem::zeroed() };
    p = (unsafe { sqlite3_aggregate_context(pCtx, ((16 as u64) as u32) as i32) })
        as *mut NthValueCtx;
    if p != std::ptr::null_mut::<NthValueCtx>()
        && (unsafe { (*p).pValue }) != std::ptr::null_mut::<sqlite3_value>()
    {
        unsafe { sqlite3_result_value(pCtx, unsafe { (*p).pValue }) };
        unsafe { sqlite3_value_free(unsafe { (*p).pValue }) };
        unsafe {
            (*p).pValue = std::ptr::null_mut::<sqlite3_value>();
        }
    }
}

// /*
// ** Implementation of built-in window function rank(). Assumes that
// ** the window frame has been set to:
// **
// **   RANGE BETWEEN UNBOUNDED PRECEDING AND CURRENT ROW
// */
#[unsafe(link_section = ".text.slate_distinct.window.rankStepFunc")]
extern "C-unwind" fn rankStepFunc(
    mut pCtx: *mut sqlite3_context,
    mut nArg: i32,
    mut apArg: *mut *mut sqlite3_value,
) {
    let mut p: *mut CallCount = unsafe { std::mem::zeroed() };
    p = (unsafe { sqlite3_aggregate_context(pCtx, ((24 as u64) as u32) as i32) }) as *mut CallCount;
    if p != std::ptr::null_mut::<CallCount>() {
        let __v1256: *mut CallCount = p;
        let __v1257: i64 = unsafe { (*__v1256).nStep };
        let __v1258: i64 = __v1257 + ((1 as i32) as i64);
        unsafe {
            (*__v1256).nStep = __v1258;
        }
        if (unsafe { (*p).nValue }) == ((0 as i32) as i64) {
            unsafe {
                (*p).nValue = unsafe { (*p).nStep };
            }
        }
    }
    nArg;
    apArg;
}

#[unsafe(link_section = ".text.slate_distinct.window.rankValueFunc")]
extern "C-unwind" fn rankValueFunc(mut pCtx: *mut sqlite3_context) {
    let mut p: *mut CallCount = unsafe { std::mem::zeroed() };
    p = (unsafe { sqlite3_aggregate_context(pCtx, ((24 as u64) as u32) as i32) }) as *mut CallCount;
    if p != std::ptr::null_mut::<CallCount>() {
        unsafe { sqlite3_result_int64(pCtx, unsafe { (*p).nValue }) };
        unsafe {
            (*p).nValue = (0 as i32) as i64;
        }
    }
}

// /*
// ** Implementation of built-in window function percent_rank(). Assumes that
// ** the window frame has been set to:
// **
// **   GROUPS BETWEEN CURRENT ROW AND UNBOUNDED FOLLOWING
// */
#[unsafe(link_section = ".text.slate_distinct.window.percent_rankStepFunc")]
extern "C-unwind" fn percent_rankStepFunc(
    mut pCtx: *mut sqlite3_context,
    mut nArg: i32,
    mut apArg: *mut *mut sqlite3_value,
) {
    let mut p: *mut CallCount = unsafe { std::mem::zeroed() };
    nArg;
    0 as i32;
    apArg;
    p = (unsafe { sqlite3_aggregate_context(pCtx, ((24 as u64) as u32) as i32) }) as *mut CallCount;
    if p != std::ptr::null_mut::<CallCount>() {
        let __v1259: *mut CallCount = p;
        let __v1260: i64 = unsafe { (*__v1259).nTotal };
        let __v1261: i64 = __v1260 + ((1 as i32) as i64);
        unsafe {
            (*__v1259).nTotal = __v1261;
        }
    }
}

#[unsafe(link_section = ".text.slate_distinct.window.percent_rankInvFunc")]
extern "C-unwind" fn percent_rankInvFunc(
    mut pCtx: *mut sqlite3_context,
    mut nArg: i32,
    mut apArg: *mut *mut sqlite3_value,
) {
    let mut p: *mut CallCount = unsafe { std::mem::zeroed() };
    nArg;
    0 as i32;
    apArg;
    p = (unsafe { sqlite3_aggregate_context(pCtx, ((24 as u64) as u32) as i32) }) as *mut CallCount;
    let __v1262: *mut CallCount = p;
    let __v1263: i64 = unsafe { (*__v1262).nStep };
    let __v1264: i64 = __v1263 + ((1 as i32) as i64);
    unsafe {
        (*__v1262).nStep = __v1264;
    }
}

#[unsafe(link_section = ".text.slate_distinct.window.percent_rankValueFunc")]
extern "C-unwind" fn percent_rankValueFunc(mut pCtx: *mut sqlite3_context) {
    let mut p: *mut CallCount = unsafe { std::mem::zeroed() };
    p = (unsafe { sqlite3_aggregate_context(pCtx, ((24 as u64) as u32) as i32) }) as *mut CallCount;
    if p != std::ptr::null_mut::<CallCount>() {
        unsafe {
            (*p).nValue = unsafe { (*p).nStep };
        }
        if (unsafe { (*p).nTotal }) > ((1 as i32) as i64) {
            let mut r: f64 = ((unsafe { (*p).nValue }) as f64)
                / (((unsafe { (*p).nTotal }) - ((1 as i32) as i64)) as f64);
            unsafe { sqlite3_result_double(pCtx, r) };
        } else {
            unsafe { sqlite3_result_double(pCtx, 0.0f64) };
        }
    }
}

// /*
// ** Implementation of built-in window function cume_dist(). Assumes that
// ** the window frame has been set to:
// **
// **   GROUPS BETWEEN 1 FOLLOWING AND UNBOUNDED FOLLOWING
// */
#[unsafe(link_section = ".text.slate_distinct.window.cume_distStepFunc")]
extern "C-unwind" fn cume_distStepFunc(
    mut pCtx: *mut sqlite3_context,
    mut nArg: i32,
    mut apArg: *mut *mut sqlite3_value,
) {
    let mut p: *mut CallCount = unsafe { std::mem::zeroed() };
    nArg;
    0 as i32;
    apArg;
    p = (unsafe { sqlite3_aggregate_context(pCtx, ((24 as u64) as u32) as i32) }) as *mut CallCount;
    if p != std::ptr::null_mut::<CallCount>() {
        let __v1265: *mut CallCount = p;
        let __v1266: i64 = unsafe { (*__v1265).nTotal };
        let __v1267: i64 = __v1266 + ((1 as i32) as i64);
        unsafe {
            (*__v1265).nTotal = __v1267;
        }
    }
}

#[unsafe(link_section = ".text.slate_distinct.window.cume_distInvFunc")]
extern "C-unwind" fn cume_distInvFunc(
    mut pCtx: *mut sqlite3_context,
    mut nArg: i32,
    mut apArg: *mut *mut sqlite3_value,
) {
    let mut p: *mut CallCount = unsafe { std::mem::zeroed() };
    nArg;
    0 as i32;
    apArg;
    p = (unsafe { sqlite3_aggregate_context(pCtx, ((24 as u64) as u32) as i32) }) as *mut CallCount;
    let __v1268: *mut CallCount = p;
    let __v1269: i64 = unsafe { (*__v1268).nStep };
    let __v1270: i64 = __v1269 + ((1 as i32) as i64);
    unsafe {
        (*__v1268).nStep = __v1270;
    }
}

#[unsafe(link_section = ".text.slate_distinct.window.cume_distValueFunc")]
extern "C-unwind" fn cume_distValueFunc(mut pCtx: *mut sqlite3_context) {
    let mut p: *mut CallCount = unsafe { std::mem::zeroed() };
    p = (unsafe { sqlite3_aggregate_context(pCtx, 0 as i32) }) as *mut CallCount;
    if p != std::ptr::null_mut::<CallCount>() {
        let mut r: f64 = ((unsafe { (*p).nStep }) as f64) / ((unsafe { (*p).nTotal }) as f64);
        unsafe { sqlite3_result_double(pCtx, r) };
    }
}

// /*
// ** Implementation of ntile(). This assumes that the window frame has
// ** been coerced to:
// **
// **   ROWS CURRENT ROW AND UNBOUNDED FOLLOWING
// */
#[unsafe(link_section = ".text.slate_distinct.window.ntileStepFunc")]
extern "C-unwind" fn ntileStepFunc(
    mut pCtx: *mut sqlite3_context,
    mut nArg: i32,
    mut apArg: *mut *mut sqlite3_value,
) {
    let mut p: *mut NtileCtx = unsafe { std::mem::zeroed() };
    0 as i32;
    nArg;
    p = (unsafe { sqlite3_aggregate_context(pCtx, ((24 as u64) as u32) as i32) }) as *mut NtileCtx;
    if p != std::ptr::null_mut::<NtileCtx>() {
        if (unsafe { (*p).nTotal }) == ((0 as i32) as i64) {
            unsafe {
                (*p).nParam = unsafe {
                    sqlite3_value_int64(unsafe { *unsafe { apArg.offset((0 as i32) as isize) } })
                };
            }
            if (unsafe { (*p).nParam }) <= ((0 as i32) as i64) {
                unsafe {
                    sqlite3_result_error(
                        pCtx,
                        (b"argument of ntile must be a positive integer\0".as_ptr() as *mut i8)
                            as *const i8,
                        -(1 as i32),
                    )
                };
            }
        }
        let __v1271: *mut NtileCtx = p;
        let __v1272: i64 = unsafe { (*__v1271).nTotal };
        let __v1273: i64 = __v1272 + ((1 as i32) as i64);
        unsafe {
            (*__v1271).nTotal = __v1273;
        }
    }
}

#[unsafe(link_section = ".text.slate_distinct.window.ntileInvFunc")]
extern "C-unwind" fn ntileInvFunc(
    mut pCtx: *mut sqlite3_context,
    mut nArg: i32,
    mut apArg: *mut *mut sqlite3_value,
) {
    let mut p: *mut NtileCtx = unsafe { std::mem::zeroed() };
    0 as i32;
    nArg;
    apArg;
    p = (unsafe { sqlite3_aggregate_context(pCtx, ((24 as u64) as u32) as i32) }) as *mut NtileCtx;
    let __v1274: *mut NtileCtx = p;
    let __v1275: i64 = unsafe { (*__v1274).iRow };
    let __v1276: i64 = __v1275 + ((1 as i32) as i64);
    unsafe {
        (*__v1274).iRow = __v1276;
    }
}

#[unsafe(link_section = ".text.slate_distinct.window.ntileValueFunc")]
extern "C-unwind" fn ntileValueFunc(mut pCtx: *mut sqlite3_context) {
    let mut p: *mut NtileCtx = unsafe { std::mem::zeroed() };
    p = (unsafe { sqlite3_aggregate_context(pCtx, ((24 as u64) as u32) as i32) }) as *mut NtileCtx;
    if p != std::ptr::null_mut::<NtileCtx>() && (unsafe { (*p).nParam }) > ((0 as i32) as i64) {
        let mut nSize: i32 = ((unsafe { (*p).nTotal }) / unsafe { (*p).nParam }) as i32;
        if nSize == (0 as i32) {
            unsafe { sqlite3_result_int64(pCtx, (unsafe { (*p).iRow }) + ((1 as i32) as i64)) };
        } else {
            let mut nLarge: i64 =
                (unsafe { (*p).nTotal }) - (unsafe { (*p).nParam }) * (nSize as i64);
            let mut iSmall: i64 = nLarge * ((nSize + (1 as i32)) as i64);
            let mut iRow: i64 = unsafe { (*p).iRow };
            0 as i32;
            if iRow < iSmall {
                unsafe {
                    sqlite3_result_int64(
                        pCtx,
                        ((1 as i32) as i64) + iRow / ((nSize + (1 as i32)) as i64),
                    )
                };
            } else {
                unsafe {
                    sqlite3_result_int64(
                        pCtx,
                        ((1 as i32) as i64) + nLarge + (iRow - iSmall) / (nSize as i64),
                    )
                };
            }
        }
    }
}

// /*
// ** Implementation of last_value().
// */
#[unsafe(link_section = ".text.slate_distinct.window.last_valueStepFunc")]
extern "C-unwind" fn last_valueStepFunc(
    mut pCtx: *mut sqlite3_context,
    mut nArg: i32,
    mut apArg: *mut *mut sqlite3_value,
) {
    let mut p: *mut LastValueCtx = unsafe { std::mem::zeroed() };
    nArg;
    p = (unsafe { sqlite3_aggregate_context(pCtx, ((16 as u64) as u32) as i32) })
        as *mut LastValueCtx;
    if p != std::ptr::null_mut::<LastValueCtx>() {
        unsafe { sqlite3_value_free(unsafe { (*p).pVal }) };
        unsafe {
            (*p).pVal = unsafe {
                sqlite3_value_dup(
                    (unsafe { *unsafe { apArg.offset((0 as i32) as isize) } })
                        as *const sqlite3_value,
                )
            };
        }
        if (unsafe { (*p).pVal }) == std::ptr::null_mut::<sqlite3_value>() {
            unsafe { sqlite3_result_error_nomem(pCtx) };
        } else {
            let __v1277: *mut LastValueCtx = p;
            let __v1278: i32 = unsafe { (*__v1277).nVal };
            let __v1279: i32 = __v1278 + (1 as i32);
            unsafe {
                (*__v1277).nVal = __v1279;
            }
        }
    }
}

#[unsafe(link_section = ".text.slate_distinct.window.last_valueInvFunc")]
extern "C-unwind" fn last_valueInvFunc(
    mut pCtx: *mut sqlite3_context,
    mut nArg: i32,
    mut apArg: *mut *mut sqlite3_value,
) {
    let mut p: *mut LastValueCtx = unsafe { std::mem::zeroed() };
    nArg;
    apArg;
    p = (unsafe { sqlite3_aggregate_context(pCtx, ((16 as u64) as u32) as i32) })
        as *mut LastValueCtx;
    if p != std::ptr::null_mut::<LastValueCtx>() {
        let __v1280: *mut LastValueCtx = p;
        let __v1281: i32 = unsafe { (*__v1280).nVal };
        let __v1282: i32 = __v1281 - (1 as i32);
        unsafe {
            (*__v1280).nVal = __v1282;
        }
        if (unsafe { (*p).nVal }) == (0 as i32) {
            unsafe { sqlite3_value_free(unsafe { (*p).pVal }) };
            unsafe {
                (*p).pVal = std::ptr::null_mut::<sqlite3_value>();
            }
        }
    }
}

#[unsafe(link_section = ".text.slate_distinct.window.last_valueValueFunc")]
extern "C-unwind" fn last_valueValueFunc(mut pCtx: *mut sqlite3_context) {
    let mut p: *mut LastValueCtx = unsafe { std::mem::zeroed() };
    p = (unsafe { sqlite3_aggregate_context(pCtx, 0 as i32) }) as *mut LastValueCtx;
    if p != std::ptr::null_mut::<LastValueCtx>()
        && (unsafe { (*p).pVal }) != std::ptr::null_mut::<sqlite3_value>()
    {
        unsafe { sqlite3_result_value(pCtx, unsafe { (*p).pVal }) };
    }
}

#[unsafe(link_section = ".text.slate_distinct.window.last_valueFinalizeFunc")]
extern "C-unwind" fn last_valueFinalizeFunc(mut pCtx: *mut sqlite3_context) {
    let mut p: *mut LastValueCtx = unsafe { std::mem::zeroed() };
    p = (unsafe { sqlite3_aggregate_context(pCtx, ((16 as u64) as u32) as i32) })
        as *mut LastValueCtx;
    if p != std::ptr::null_mut::<LastValueCtx>()
        && (unsafe { (*p).pVal }) != std::ptr::null_mut::<sqlite3_value>()
    {
        unsafe { sqlite3_result_value(pCtx, unsafe { (*p).pVal }) };
        unsafe { sqlite3_value_free(unsafe { (*p).pVal }) };
        unsafe {
            (*p).pVal = std::ptr::null_mut::<sqlite3_value>();
        }
    }
}

// /*
// ** No-op implementations of xStep() and xFinalize().  Used as place-holders
// ** for built-in window functions that never call those interfaces.
// **
// ** The noopValueFunc() is called but is expected to do nothing.  The
// ** noopStepFunc() is never called, and so it is marked with NO_TEST to
// ** let the test coverage routine know not to expect this function to be
// ** invoked.
// */
// /*NO_TEST*/
#[unsafe(link_section = ".text.slate_distinct.window.noopStepFunc")]
extern "C-unwind" fn noopStepFunc(
    mut p: *mut sqlite3_context,
    mut n: i32,
    mut a: *mut *mut sqlite3_value,
) {
    // /*NO_TEST*/
    p;
    // /*NO_TEST*/
    n;
    // /*NO_TEST*/
    a;
    // /*NO_TEST*/
    0 as i32;
    // /*NO_TEST*/
}

// /*NO_TEST*/
// /*NO_TEST*/
// /*NO_TEST*/
// /*NO_TEST*/
#[unsafe(link_section = ".text.slate_distinct.window.noopValueFunc")]
extern "C-unwind" fn noopValueFunc(mut p: *mut sqlite3_context) {
    p;
}

fn windowFind(mut pParse: *mut Parse, mut pList: *mut Window, mut zName: *const i8) -> *mut Window {
    let mut p: *mut Window = unsafe { std::mem::zeroed() };
    p = pList;
    '__slate_break_1100: while p != std::ptr::null_mut::<Window>() {
        if (unsafe { sqlite3StrICmp((unsafe { (*p).zName }) as *const i8, zName) }) == (0 as i32) {
            break '__slate_break_1100;
        }
        p = unsafe { (*p).pNextWin };
    }
    if p == std::ptr::null_mut::<Window>() {
        unsafe {
            sqlite3ErrorMsg(
                pParse,
                (b"no such window: %s\0".as_ptr() as *mut i8) as *const i8,
                zName,
            )
        };
    }
    return p;
}

// /*
// ** Callback function used by selectWindowRewriteEList(). If necessary,
// ** this function appends to the output expression-list and updates
// ** expression (*ppExpr) in place.
// */
#[unsafe(link_section = ".text.slate_distinct.window.selectWindowRewriteExprCb")]
extern "C-unwind" fn selectWindowRewriteExprCb(
    mut pWalker: *mut Walker,
    mut pExpr: *mut Expr,
) -> i32 {
    let mut p: *mut WindowRewrite = unsafe { (*pWalker).u.pRewrite };
    let mut pParse: *mut Parse = unsafe { (*pWalker).pParse };
    0 as i32;
    0 as i32;
    // /* If this function is being called from within a scalar sub-select
    //   ** that used by the SELECT statement being processed, only process
    //   ** TK_COLUMN expressions that refer to it (the outer SELECT). Do
    //   ** not process aggregates or window functions at all, as they belong
    //   ** to the scalar sub-select.  */
    if (unsafe { (*p).pSubSelect }) != std::ptr::null_mut::<Select>() {
        if (((unsafe { (*pExpr).op }) as u32) as i32) != (168 as i32) {
            return 0 as i32;
        } else {
            let mut nSrc: i32 = unsafe { (*unsafe { (*p).pSrc }).nSrc };
            let mut i: i32 = 0 as i32;
            i = 0 as i32;
            '__slate_break_1105: loop {
                if !(i < nSrc) {
                    break;
                }
                if (unsafe { (*pExpr).iTable })
                    == unsafe {
                        (*unsafe {
                            unsafe {
                                std::ptr::addr_of_mut!((*unsafe { (*p).pSrc }).a) as *mut SrcItem
                            }
                            .offset(i as isize)
                        })
                        .iCursor
                    }
                {
                    break '__slate_break_1105;
                }
                let __v1283: i32 = i;
                let __v1284: i32 = __v1283 + (1 as i32);
                i = __v1284;
            }
            if i == nSrc {
                return 0 as i32;
            }
        }
    }
    '__slate_break_1106: {
        match ((unsafe { (*pExpr).op }) as u32) as i32 {
            172 => {
                if !((unsafe { (*pExpr).flags }) & ((16777216 as i32) as u32)
                    != ((0 as i32) as u32))
                {
                    break '__slate_break_1106;
                } else {
                    let mut pWin: *mut Window = unsafe { std::mem::zeroed() };
                    pWin = unsafe { (*p).pWin };
                    '__slate_break_1107: while pWin != std::ptr::null_mut::<Window>() {
                        if (unsafe { (*pExpr).y.pWin }) == pWin {
                            0 as i32;
                            return 1 as i32;
                        }
                        pWin = unsafe { (*pWin).pNextWin };
                    }
                }
                // /* no break */
                {}
                let mut iCol: i32 = -(1 as i32);
                if (unsafe { (*unsafe { (*pParse).db }).mallocFailed }) != (0 as u8) {
                    return 2 as i32;
                }
                if (unsafe { (*p).pSub }) != std::ptr::null_mut::<ExprList>() {
                    let mut i: i32 = 0 as i32;
                    i = 0 as i32;
                    '__slate_break_1108: loop {
                        if !(i < unsafe { (*unsafe { (*p).pSub }).nExpr }) {
                            break;
                        }
                        if (0 as i32)
                            == unsafe {
                                sqlite3ExprCompare(
                                    std::ptr::null::<Parse>(),
                                    (unsafe {
                                        (*unsafe {
                                            unsafe {
                                                std::ptr::addr_of_mut!((*unsafe { (*p).pSub }).a)
                                                    as *mut ExprList_item
                                            }
                                            .offset(i as isize)
                                        })
                                        .pExpr
                                    }) as *const Expr,
                                    pExpr as *const Expr,
                                    -(1 as i32),
                                )
                            }
                        {
                            iCol = i;
                            break '__slate_break_1108;
                        }
                        let _v1293: i32 = i;
                        let _v1294: i32 = _v1293 + (1 as i32);
                        i = _v1294;
                    }
                }
                if iCol < (0 as i32) {
                    let mut pDup: *mut Expr = unsafe {
                        sqlite3ExprDup(unsafe { (*pParse).db }, pExpr as *const Expr, 0 as i32)
                    };
                    if pDup != std::ptr::null_mut::<Expr>()
                        && (((unsafe { (*pDup).op }) as u32) as i32) == (169 as i32)
                    {
                        unsafe {
                            (*pDup).op = ((172 as i32) as i8) as u8;
                        }
                    }
                    unsafe {
                        (*p).pSub =
                            unsafe { sqlite3ExprListAppend(pParse, unsafe { (*p).pSub }, pDup) };
                    }
                }
                if (unsafe { (*p).pSub }) != std::ptr::null_mut::<ExprList>() {
                    let mut f: i32 = ((unsafe { (*pExpr).flags }) & ((512 as i32) as u32)) as i32;
                    0 as i32;
                    let _v1295: *mut Expr = pExpr;
                    let _v1296: u32 = unsafe { (*_v1295).flags };
                    let _v1297: u32 = _v1296 | ((134217728 as i32) as u32);
                    unsafe {
                        (*_v1295).flags = _v1297;
                    }
                    unsafe { sqlite3ExprDelete(unsafe { (*pParse).db }, pExpr) };
                    let _v1298: *mut Expr = pExpr;
                    let _v1299: u32 = unsafe { (*_v1298).flags };
                    let _v1300: u32 = _v1299 & !((134217728 as i32) as u32);
                    unsafe {
                        (*_v1298).flags = _v1300;
                    }
                    unsafe { memset(pExpr as *mut (), 0 as i32, 72 as u64) };
                    unsafe {
                        (*pExpr).op = ((168 as i32) as i8) as u8;
                    }
                    unsafe {
                        (*pExpr).iColumn = (if iCol < (0 as i32) {
                            (unsafe { (*unsafe { (*p).pSub }).nExpr }) - (1 as i32)
                        } else {
                            iCol
                        }) as i16;
                    }
                    unsafe {
                        (*pExpr).iTable = unsafe { (*unsafe { (*p).pWin }).iEphCsr };
                    }
                    unsafe {
                        (*pExpr).y.pTab = unsafe { (*p).pTab };
                    }
                    unsafe {
                        (*pExpr).flags = f as u32;
                    }
                }
                if (unsafe { (*unsafe { (*pParse).db }).mallocFailed }) != (0 as u8) {
                    return 2 as i32;
                }
                break '__slate_break_1106;
                // /* no-op */
            }
            179 | 169 | 168 => {
                let mut iCol: i32 = -(1 as i32);
                if (unsafe { (*unsafe { (*pParse).db }).mallocFailed }) != (0 as u8) {
                    return 2 as i32;
                }
                if (unsafe { (*p).pSub }) != std::ptr::null_mut::<ExprList>() {
                    {
                        let mut i: i32 = 0 as i32;
                        {
                            i = 0 as i32;
                            '__slate_break_1108: loop {
                                if !(i < unsafe { (*unsafe { (*p).pSub }).nExpr }) {
                                    break;
                                }
                                '__slate_continue_1108: {
                                    {
                                        if (0 as i32)
                                            == unsafe {
                                                sqlite3ExprCompare(
                                                    std::ptr::null::<Parse>(),
                                                    (unsafe {
                                                        (*unsafe {
                                                            unsafe {
                                                                std::ptr::addr_of_mut!(
                                                                    (*unsafe { (*p).pSub }).a
                                                                )
                                                                    as *mut ExprList_item
                                                            }
                                                            .offset(i as isize)
                                                        })
                                                        .pExpr
                                                    })
                                                        as *const Expr,
                                                    pExpr as *const Expr,
                                                    -(1 as i32),
                                                )
                                            }
                                        {
                                            {
                                                iCol = i;
                                                break '__slate_break_1108;
                                            }
                                        }
                                    }
                                }
                                let __v1285: i32 = i;
                                let __v1286: i32 = __v1285 + (1 as i32);
                                i = __v1286;
                            }
                        }
                    }
                }
                if iCol < (0 as i32) {
                    {
                        let mut pDup: *mut Expr = unsafe {
                            sqlite3ExprDup(unsafe { (*pParse).db }, pExpr as *const Expr, 0 as i32)
                        };
                        if pDup != std::ptr::null_mut::<Expr>()
                            && (((unsafe { (*pDup).op }) as u32) as i32) == (169 as i32)
                        {
                            unsafe {
                                (*pDup).op = ((172 as i32) as i8) as u8;
                            }
                        }
                        unsafe {
                            (*p).pSub = unsafe {
                                sqlite3ExprListAppend(pParse, unsafe { (*p).pSub }, pDup)
                            };
                        }
                    }
                }
                if (unsafe { (*p).pSub }) != std::ptr::null_mut::<ExprList>() {
                    {
                        let mut f: i32 =
                            ((unsafe { (*pExpr).flags }) & ((512 as i32) as u32)) as i32;
                        0 as i32;
                        let __v1287: *mut Expr = pExpr;
                        let __v1288: u32 = unsafe { (*__v1287).flags };
                        let __v1289: u32 = __v1288 | ((134217728 as i32) as u32);
                        unsafe {
                            (*__v1287).flags = __v1289;
                        }
                        unsafe { sqlite3ExprDelete(unsafe { (*pParse).db }, pExpr) };
                        let __v1290: *mut Expr = pExpr;
                        let __v1291: u32 = unsafe { (*__v1290).flags };
                        let __v1292: u32 = __v1291 & !((134217728 as i32) as u32);
                        unsafe {
                            (*__v1290).flags = __v1292;
                        }
                        unsafe { memset(pExpr as *mut (), 0 as i32, 72 as u64) };
                        unsafe {
                            (*pExpr).op = ((168 as i32) as i8) as u8;
                        }
                        unsafe {
                            (*pExpr).iColumn = (if iCol < (0 as i32) {
                                (unsafe { (*unsafe { (*p).pSub }).nExpr }) - (1 as i32)
                            } else {
                                iCol
                            }) as i16;
                        }
                        unsafe {
                            (*pExpr).iTable = unsafe { (*unsafe { (*p).pWin }).iEphCsr };
                        }
                        unsafe {
                            (*pExpr).y.pTab = unsafe { (*p).pTab };
                        }
                        unsafe {
                            (*pExpr).flags = f as u32;
                        }
                    }
                }
                if (unsafe { (*unsafe { (*pParse).db }).mallocFailed }) != (0 as u8) {
                    return 2 as i32;
                }
                break '__slate_break_1106;
                // /* no-op */
            }
            _ => {}
        }
    }
    return 0 as i32;
}

#[unsafe(link_section = ".text.slate_distinct.window.selectWindowRewriteSelectCb")]
extern "C-unwind" fn selectWindowRewriteSelectCb(
    mut pWalker: *mut Walker,
    mut pSelect: *mut Select,
) -> i32 {
    let mut p: *mut WindowRewrite = unsafe { (*pWalker).u.pRewrite };
    let mut pSave: *mut Select = unsafe { (*p).pSubSelect };
    if pSave == pSelect {
        return 0 as i32;
    } else {
        unsafe {
            (*p).pSubSelect = pSelect;
        }
        unsafe { sqlite3WalkSelect(pWalker, pSelect) };
        unsafe {
            (*p).pSubSelect = pSave;
        }
    }
    return 1 as i32;
}

// /*
// ** Iterate through each expression in expression-list pEList. For each:
// **
// **   * TK_COLUMN,
// **   * aggregate function, or
// **   * window function with a Window object that is not a member of the
// **     Window list passed as the second argument (pWin).
// **
// ** Append the node to output expression-list (*ppSub). And replace it
// ** with a TK_COLUMN that reads the (N-1)th element of table
// ** pWin->iEphCsr, where N is the number of elements in (*ppSub) after
// ** appending the new one.
// */
fn selectWindowRewriteEList(
    mut pParse: *mut Parse,
    mut pWin: *mut Window,
    mut pSrc: *mut SrcList,
    mut pEList: *mut ExprList,
    mut pTab: *mut Table,
    mut ppSub: *mut *mut ExprList,
) {
    let mut sWalker: Walker = unsafe { std::mem::zeroed() };
    let mut sRewrite: WindowRewrite = unsafe { std::mem::zeroed() };
    0 as i32;
    unsafe {
        memset(
            std::ptr::addr_of_mut!(sWalker) as *mut (),
            0 as i32,
            48 as u64,
        )
    };
    unsafe {
        memset(
            std::ptr::addr_of_mut!(sRewrite) as *mut (),
            0 as i32,
            40 as u64,
        )
    };
    sRewrite.pSub = unsafe { *ppSub };
    sRewrite.pWin = pWin;
    sRewrite.pSrc = pSrc;
    sRewrite.pTab = pTab;
    sWalker.pParse = pParse;
    sWalker.xExprCallback = Some(selectWindowRewriteExprCb);
    sWalker.xSelectCallback = Some(selectWindowRewriteSelectCb);
    unsafe {
        sWalker.u.pRewrite = std::ptr::addr_of_mut!(sRewrite);
    }
    unsafe { sqlite3WalkExprList(std::ptr::addr_of_mut!(sWalker), pEList) };
    unsafe {
        *ppSub = sRewrite.pSub;
    }
}

// /* Rewrite expressions in this list */
// /* IN/OUT: Sub-select expression-list */
// /*
// ** Append a copy of each expression in expression-list pAppend to
// ** expression list pList. Return a pointer to the result list.
// */
fn exprListAppendList(
    mut pParse: *mut Parse,
    mut pList: *mut ExprList,
    mut pAppend: *mut ExprList,
    mut bIntToNull: i32,
) -> *mut ExprList {
    if pAppend != std::ptr::null_mut::<ExprList>() {
        let mut i: i32 = 0 as i32;
        let mut nInit: i32 = if pList != std::ptr::null_mut::<ExprList>() {
            unsafe { (*pList).nExpr }
        } else {
            0 as i32
        };
        i = 0 as i32;
        '__slate_break_1109: loop {
            if !(i < unsafe { (*pAppend).nExpr }) {
                break;
            }
            let mut db: *mut sqlite3 = unsafe { (*pParse).db };
            let mut pDup: *mut Expr = unsafe {
                sqlite3ExprDup(
                    db,
                    (unsafe {
                        (*unsafe {
                            unsafe { std::ptr::addr_of_mut!((*pAppend).a) as *mut ExprList_item }
                                .offset(i as isize)
                        })
                        .pExpr
                    }) as *const Expr,
                    0 as i32,
                )
            };
            if (unsafe { (*db).mallocFailed }) != (0 as u8) {
                unsafe { sqlite3ExprDelete(db, pDup) };
                break '__slate_break_1109;
            }
            if bIntToNull != (0 as i32) {
                let mut iDummy: i32 = 0 as i32;
                let mut pSub: *mut Expr = unsafe { std::mem::zeroed() };
                pSub = unsafe { sqlite3ExprSkipCollateAndLikely(pDup) };
                if (unsafe {
                    sqlite3ExprIsInteger(
                        pSub as *const Expr,
                        std::ptr::addr_of_mut!(iDummy),
                        std::ptr::null_mut::<Parse>(),
                        0 as i32,
                    )
                }) != (0 as i32)
                {
                    unsafe {
                        (*pSub).op = ((122 as i32) as i8) as u8;
                    }
                    let __v1295: *mut Expr = pSub;
                    let __v1296: u32 = unsafe { (*__v1295).flags };
                    let __v1297: u32 = __v1296
                        & (!((2048 as i32) | (268435456 as i32) | (536870912 as i32)) as u32);
                    unsafe {
                        (*__v1295).flags = __v1297;
                    }
                    unsafe {
                        (*pSub).u.zToken = std::ptr::null_mut::<i8>();
                    }
                }
            }
            pList = unsafe { sqlite3ExprListAppend(pParse, pList, pDup) };
            if pList != std::ptr::null_mut::<ExprList>() {
                unsafe {
                    (*unsafe {
                        unsafe { std::ptr::addr_of_mut!((*pList).a) as *mut ExprList_item }
                            .offset((nInit + i) as isize)
                    })
                    .fg
                    .sortFlags = unsafe {
                        (*unsafe {
                            unsafe { std::ptr::addr_of_mut!((*pAppend).a) as *mut ExprList_item }
                                .offset(i as isize)
                        })
                        .fg
                        .sortFlags
                    };
                }
            }
            let __v1293: i32 = i;
            let __v1294: i32 = __v1293 + (1 as i32);
            i = __v1294;
        }
    }
    return pList;
}

// /* Parsing context */
// /* List to which to append. Might be NULL */
// /* List of values to append. Might be NULL */
// /*
// ** When rewriting a query, if the new subquery in the FROM clause
// ** contains TK_AGG_FUNCTION nodes that refer to an outer query,
// ** then we have to increase the Expr->op2 values of those nodes
// ** due to the extra subquery layer that was added.
// **
// ** See also the incrAggDepth() routine in resolve.c
// */
#[unsafe(link_section = ".text.slate_distinct.window.sqlite3WindowExtraAggFuncDepth")]
extern "C-unwind" fn sqlite3WindowExtraAggFuncDepth(
    mut pWalker: *mut Walker,
    mut pExpr: *mut Expr,
) -> i32 {
    if (((unsafe { (*pExpr).op }) as u32) as i32) == (169 as i32)
        && (((unsafe { (*pExpr).op2 }) as u32) as i32) >= unsafe { (*pWalker).walkerDepth }
    {
        let __v1298: *mut Expr = pExpr;
        let __v1299: u8 = unsafe { (*__v1298).op2 };
        let __v1300: u8 = ((((__v1299 as u32) as i32) + (1 as i32)) as i8) as u8;
        unsafe {
            (*__v1298).op2 = __v1300;
        }
    }
    return 0 as i32;
}

#[unsafe(link_section = ".text.slate_distinct.window.disallowAggregatesInOrderByCb")]
extern "C-unwind" fn disallowAggregatesInOrderByCb(
    mut pWalker: *mut Walker,
    mut pExpr: *mut Expr,
) -> i32 {
    if (((unsafe { (*pExpr).op }) as u32) as i32) == (169 as i32)
        && (unsafe { (*pExpr).pAggInfo }) == std::ptr::null_mut::<AggInfo>()
    {
        0 as i32;
        unsafe {
            sqlite3ErrorMsg(
                unsafe { (*pWalker).pParse },
                (b"misuse of aggregate: %s()\0".as_ptr() as *mut i8) as *const i8,
                unsafe { (*pExpr).u.zToken },
            )
        };
    }
    return 0 as i32;
}

// /*
// ** The argument expression is an PRECEDING or FOLLOWING offset.  The
// ** value should be a non-negative integer.  If the value is not a
// ** constant, change it to NULL.  The fact that it is then a non-negative
// ** integer will be caught later.  But it is important not to leave
// ** variable values in the expression tree.
// */
fn sqlite3WindowOffsetExpr(mut pParse: *mut Parse, mut pExpr: *mut Expr) -> *mut Expr {
    if (0 as i32) == unsafe { sqlite3ExprIsConstant(std::ptr::null_mut::<Parse>(), pExpr) } {
        if (((unsafe { (*pParse).eParseMode }) as u32) as i32) >= (2 as i32) {
            unsafe { sqlite3RenameExprUnmap(pParse, pExpr) };
        }
        unsafe { sqlite3ExprDelete(unsafe { (*pParse).db }, pExpr) };
        pExpr = unsafe {
            sqlite3ExprAlloc(
                unsafe { (*pParse).db },
                122 as i32,
                std::ptr::null::<Token>(),
                0 as i32,
            )
        };
    }
    return pExpr;
}

// /*
// ** A "PRECEDING <expr>" (eCond==0) or "FOLLOWING <expr>" (eCond==1) or the
// ** value of the second argument to nth_value() (eCond==2) has just been
// ** evaluated and the result left in register reg. This function generates VM
// ** code to check that the value is a non-negative integer and throws an
// ** exception if it is not.
// */
fn windowCheckValue(mut pParse: *mut Parse, mut reg: i32, mut eCond: i32) {
    let mut v: *mut Vdbe = unsafe { sqlite3GetVdbe(pParse) };
    let mut regZero: i32 = unsafe { sqlite3GetTempReg(pParse) };
    0 as i32;
    unsafe { sqlite3VdbeAddOp2(v, 73 as i32, 0 as i32, regZero) };
    if eCond >= (3 as i32) {
        let mut regString: i32 = unsafe { sqlite3GetTempReg(pParse) };
        unsafe {
            sqlite3VdbeAddOp4(
                v,
                118 as i32,
                0 as i32,
                regString,
                0 as i32,
                (b"\0".as_ptr() as *mut i8) as *const i8,
                -(1 as i32),
            )
        };
        unsafe {
            sqlite3VdbeAddOp3(
                v,
                58 as i32,
                regString,
                (unsafe { sqlite3VdbeCurrentAddr(v) }) + (2 as i32),
                reg,
            )
        };
        unsafe { sqlite3VdbeChangeP5(v, (((67 as i32) | (16 as i32)) as i16) as u16) };
        {}
        0 as i32;
        {}
        {}
    } else {
        unsafe {
            sqlite3VdbeAddOp2(
                v,
                13 as i32,
                reg,
                (unsafe { sqlite3VdbeCurrentAddr(v) }) + (2 as i32),
            )
        };
        {}
        0 as i32;
        {}
        {}
        {}
    }
    unsafe {
        sqlite3VdbeAddOp3(
            v,
            unsafe {
                *unsafe {
                    unsafe { std::ptr::addr_of_mut!(aOp.0) as *mut i32 }.offset(eCond as isize)
                }
            },
            regZero,
            (unsafe { sqlite3VdbeCurrentAddr(v) }) + (2 as i32),
            reg,
        )
    };
    unsafe { sqlite3VdbeChangeP5(v, ((67 as i32) as i16) as u16) };
    // /* NULL case captured by */
    {}
    // /*   the OP_MustBeInt */
    {}
    {}
    // /* NULL case caught by */
    {}
    // /*   the OP_Ge */
    {}
    unsafe { sqlite3MayAbort(pParse) };
    unsafe { sqlite3VdbeAddOp2(v, 72 as i32, 1 as i32, 2 as i32) };
    unsafe {
        sqlite3VdbeAppendP4(
            v,
            (unsafe {
                *unsafe {
                    unsafe { std::ptr::addr_of_mut!(azErr.0) as *mut *const i8 }
                        .offset(eCond as isize)
                }
            }) as *mut (),
            -(1 as i32),
        )
    };
    unsafe { sqlite3ReleaseTempReg(pParse, regZero) };
}

// /*
// ** Return the number of arguments passed to the window-function associated
// ** with the object passed as the only argument to this function.
// */
fn windowArgCount(mut pWin: *mut Window) -> i32 {
    let mut pList: *const ExprList = unsafe { std::mem::zeroed() };
    0 as i32;
    pList = (unsafe { (*unsafe { (*pWin).pOwner }).x.pList }) as *const ExprList;
    return if pList != std::ptr::null::<ExprList>() {
        unsafe { (*pList).nExpr }
    } else {
        0 as i32
    };
}

// /*
// ** Generate VM code to read the window frames peer values from cursor csr into
// ** an array of registers starting at reg.
// */
fn windowReadPeerValues(mut p: *mut WindowCodeArg, mut csr: i32, mut reg: i32) {
    let mut pMWin: *mut Window = unsafe { (*p).pMWin };
    let mut pOrderBy: *mut ExprList = unsafe { (*pMWin).pOrderBy };
    if pOrderBy != std::ptr::null_mut::<ExprList>() {
        let mut v: *mut Vdbe = unsafe { sqlite3GetVdbe(unsafe { (*p).pParse }) };
        let mut pPart: *mut ExprList = unsafe { (*pMWin).pPartition };
        let mut iColOff: i32 = (unsafe { (*pMWin).nBufferCol })
            + if pPart != std::ptr::null_mut::<ExprList>() {
                unsafe { (*pPart).nExpr }
            } else {
                0 as i32
            };
        let mut i: i32 = 0 as i32;
        i = 0 as i32;
        '__slate_break_1126: loop {
            if !(i < unsafe { (*pOrderBy).nExpr }) {
                break;
            }
            unsafe { sqlite3VdbeAddOp3(v, 96 as i32, csr, iColOff + i, reg + i) };
            let __v1301: i32 = i;
            let __v1302: i32 = __v1301 + (1 as i32);
            i = __v1302;
        }
    }
}

// /*
// ** Generate VM code to invoke either xStep() (if bInverse is 0) or
// ** xInverse (if bInverse is non-zero) for each window function in the
// ** linked list starting at pMWin. Or, for built-in window functions
// ** that do not use the standard function API, generate the required
// ** inline VM code.
// **
// ** If argument csr is greater than or equal to 0, then argument reg is
// ** the first register in an array of registers guaranteed to be large
// ** enough to hold the array of arguments for each function. In this case
// ** the arguments are extracted from the current row of csr into the
// ** array of registers before invoking OP_AggStep or OP_AggInverse
// **
// ** Or, if csr is less than zero, then the array of registers at reg is
// ** already populated with all columns from the current row of the sub-query.
// **
// ** If argument regPartSize is non-zero, then it is a register containing the
// ** number of rows in the current partition.
// */
fn windowAggStep(
    mut p: *mut WindowCodeArg,
    mut pMWin: *mut Window,
    mut csr: i32,
    mut bInverse: i32,
    mut reg: i32,
) {
    let mut pParse: *mut Parse = unsafe { (*p).pParse };
    let mut v: *mut Vdbe = unsafe { sqlite3GetVdbe(pParse) };
    let mut pWin: *mut Window = unsafe { std::mem::zeroed() };
    pWin = pMWin;
    '__slate_break_1127: while pWin != std::ptr::null_mut::<Window>() {
        let mut pFunc: *mut FuncDef = unsafe { (*pWin).pWFunc };
        let mut regArg: i32 = 0 as i32;
        let mut nArg: i32 = 0 as i32;
        let __v1303: i32;
        if (unsafe { (*pWin).bExprArgs }) != (0 as u8) {
            __v1303 = 0 as i32;
        } else {
            __v1303 = windowArgCount(pWin);
        }
        nArg = __v1303;
        let mut i: i32 = 0 as i32;
        let mut addrIf: i32 = 0 as i32;
        0 as i32;
        // /* All OVER clauses in the same window function aggregate step must
        //     ** be the same. */
        0 as i32;
        i = 0 as i32;
        '__slate_break_1128: loop {
            if !(i < nArg) {
                break;
            }
            if i != (1 as i32)
                || (unsafe { (*pFunc).zName })
                    != unsafe { std::ptr::addr_of!(nth_valueName) as *const i8 }
            {
                unsafe {
                    sqlite3VdbeAddOp3(v, 96 as i32, csr, (unsafe { (*pWin).iArgCol }) + i, reg + i)
                };
            } else {
                unsafe {
                    sqlite3VdbeAddOp3(
                        v,
                        96 as i32,
                        unsafe { (*pMWin).iEphCsr },
                        (unsafe { (*pWin).iArgCol }) + i,
                        reg + i,
                    )
                };
            }
            let __v1304: i32 = i;
            let __v1305: i32 = __v1304 + (1 as i32);
            i = __v1305;
        }
        regArg = reg;
        if (unsafe { (*pWin).pFilter }) != std::ptr::null_mut::<Expr>() {
            let mut regTmp: i32 = 0 as i32;
            0 as i32;
            0 as i32;
            0 as i32;
            regTmp = unsafe { sqlite3GetTempReg(pParse) };
            unsafe {
                sqlite3VdbeAddOp3(
                    v,
                    96 as i32,
                    csr,
                    (unsafe { (*pWin).iArgCol }) + nArg,
                    regTmp,
                )
            };
            addrIf = unsafe { sqlite3VdbeAddOp3(v, 17 as i32, regTmp, 0 as i32, 1 as i32) };
            {}
            unsafe { sqlite3ReleaseTempReg(pParse, regTmp) };
        }
        if (unsafe { (*pMWin).regStartRowid }) == (0 as i32)
            && (unsafe { (*pFunc).funcFlags }) & ((4096 as i32) as u32) != (0 as u32)
            && (((unsafe { (*pWin).eStart }) as u32) as i32) != (91 as i32)
        {
            let mut addrIsNull: i32 = unsafe { sqlite3VdbeAddOp1(v, 51 as i32, regArg) };
            {}
            if bInverse == (0 as i32) {
                unsafe {
                    sqlite3VdbeAddOp2(
                        v,
                        88 as i32,
                        (unsafe { (*pWin).regApp }) + (1 as i32),
                        1 as i32,
                    )
                };
                unsafe { sqlite3VdbeAddOp2(v, 83 as i32, regArg, unsafe { (*pWin).regApp }) };
                unsafe {
                    sqlite3VdbeAddOp3(
                        v,
                        99 as i32,
                        unsafe { (*pWin).regApp },
                        2 as i32,
                        (unsafe { (*pWin).regApp }) + (2 as i32),
                    )
                };
                unsafe {
                    sqlite3VdbeAddOp2(
                        v,
                        140 as i32,
                        unsafe { (*pWin).csrApp },
                        (unsafe { (*pWin).regApp }) + (2 as i32),
                    )
                };
            } else {
                unsafe {
                    sqlite3VdbeAddOp4Int(
                        v,
                        23 as i32,
                        unsafe { (*pWin).csrApp },
                        0 as i32,
                        regArg,
                        1 as i32,
                    )
                };
                {}
                unsafe { sqlite3VdbeAddOp1(v, 132 as i32, unsafe { (*pWin).csrApp }) };
                unsafe {
                    sqlite3VdbeJumpHere(v, (unsafe { sqlite3VdbeCurrentAddr(v) }) - (2 as i32))
                };
            }
            unsafe { sqlite3VdbeJumpHere(v, addrIsNull) };
        } else {
            if (unsafe { (*pWin).regApp }) != (0 as i32) {
                0 as i32;
                0 as i32;
                0 as i32;
                unsafe {
                    sqlite3VdbeAddOp2(
                        v,
                        88 as i32,
                        (unsafe { (*pWin).regApp }) + (1 as i32) - bInverse,
                        1 as i32,
                    )
                };
            } else {
                if (unsafe { (*pFunc).xSFunc }) != Some(noopStepFunc) {
                    if (unsafe { (*pWin).bExprArgs }) != (0 as u8) {
                        let mut iOp: i32 = unsafe { sqlite3VdbeCurrentAddr(v) };
                        let mut iEnd: i32 = 0 as i32;
                        0 as i32;
                        nArg = unsafe { (*unsafe { (*unsafe { (*pWin).pOwner }).x.pList }).nExpr };
                        regArg = unsafe { sqlite3GetTempRange(pParse, nArg) };
                        unsafe {
                            sqlite3ExprCodeExprList(
                                pParse,
                                unsafe { (*unsafe { (*pWin).pOwner }).x.pList },
                                regArg,
                                0 as i32,
                                ((0 as i32) as i8) as u8,
                            )
                        };
                        iEnd = unsafe { sqlite3VdbeCurrentAddr(v) };
                        '__slate_break_1129: loop {
                            if !(iOp < iEnd) {
                                break;
                            }
                            let mut pOp: *mut VdbeOp = unsafe { sqlite3VdbeGetOp(v, iOp) };
                            if (((unsafe { (*pOp).opcode }) as u32) as i32) == (96 as i32)
                                && (unsafe { (*pOp).p1 }) == unsafe { (*pMWin).iEphCsr }
                            {
                                unsafe {
                                    (*pOp).p1 = csr;
                                }
                            }
                            let __v1306: i32 = iOp;
                            let __v1307: i32 = __v1306 + (1 as i32);
                            iOp = __v1307;
                        }
                    }
                    if (unsafe { (*pFunc).funcFlags }) & ((32 as i32) as u32) != (0 as u32) {
                        let mut pColl: *mut CollSeq = unsafe { std::mem::zeroed() };
                        0 as i32;
                        0 as i32;
                        pColl = unsafe {
                            sqlite3ExprNNCollSeq(
                                pParse,
                                (unsafe {
                                    (*unsafe {
                                        unsafe {
                                            std::ptr::addr_of_mut!(
                                                (*unsafe { (*unsafe { (*pWin).pOwner }).x.pList })
                                                    .a
                                            )
                                                as *mut ExprList_item
                                        }
                                        .offset((0 as i32) as isize)
                                    })
                                    .pExpr
                                }) as *const Expr,
                            )
                        };
                        unsafe {
                            sqlite3VdbeAddOp4(
                                v,
                                87 as i32,
                                0 as i32,
                                0 as i32,
                                0 as i32,
                                pColl as *const i8,
                                -(2 as i32),
                            )
                        };
                    }
                    unsafe {
                        sqlite3VdbeAddOp3(
                            v,
                            if bInverse != (0 as i32) {
                                163 as i32
                            } else {
                                164 as i32
                            },
                            bInverse,
                            regArg,
                            unsafe { (*pWin).regAccum },
                        )
                    };
                    unsafe { sqlite3VdbeAppendP4(v, pFunc as *mut (), -(8 as i32)) };
                    unsafe { sqlite3VdbeChangeP5(v, (nArg as i16) as u16) };
                    if (unsafe { (*pWin).bExprArgs }) != (0 as u8) {
                        unsafe { sqlite3ReleaseTempRange(pParse, regArg, nArg) };
                    }
                }
            }
        }
        if addrIf != (0 as i32) {
            unsafe { sqlite3VdbeJumpHere(v, addrIf) };
        }
        pWin = unsafe { (*pWin).pNextWin };
    }
}

// /* Linked list of window functions */
// /* Read arguments from this cursor */
// /* True to invoke xInverse instead of xStep */
// /* Array of registers */
// /*
// ** Values that may be passed as the second argument to windowCodeOp().
// */
// /*
// ** Generate VM code to invoke either xValue() (bFin==0) or xFinalize()
// ** (bFin==1) for each window function in the linked list starting at
// ** pMWin. Or, for built-in window-functions that do not use the standard
// ** API, generate the equivalent VM code.
// */
fn windowAggFinal(mut p: *mut WindowCodeArg, mut bFin: i32) {
    let mut pParse: *mut Parse = unsafe { (*p).pParse };
    let mut pMWin: *mut Window = unsafe { (*p).pMWin };
    let mut v: *mut Vdbe = unsafe { sqlite3GetVdbe(pParse) };
    let mut pWin: *mut Window = unsafe { std::mem::zeroed() };
    pWin = pMWin;
    '__slate_break_1130: while pWin != std::ptr::null_mut::<Window>() {
        if (unsafe { (*pMWin).regStartRowid }) == (0 as i32)
            && (unsafe { (*unsafe { (*pWin).pWFunc }).funcFlags }) & ((4096 as i32) as u32)
                != (0 as u32)
            && (((unsafe { (*pWin).eStart }) as u32) as i32) != (91 as i32)
        {
            unsafe { sqlite3VdbeAddOp2(v, 77 as i32, 0 as i32, unsafe { (*pWin).regResult }) };
            unsafe { sqlite3VdbeAddOp1(v, 32 as i32, unsafe { (*pWin).csrApp }) };
            {}
            unsafe {
                sqlite3VdbeAddOp3(v, 96 as i32, unsafe { (*pWin).csrApp }, 0 as i32, unsafe {
                    (*pWin).regResult
                })
            };
            unsafe { sqlite3VdbeJumpHere(v, (unsafe { sqlite3VdbeCurrentAddr(v) }) - (2 as i32)) };
        } else {
            if (unsafe { (*pWin).regApp }) != (0 as i32) {
                0 as i32;
            } else {
                let mut nArg: i32 = windowArgCount(pWin);
                if bFin != (0 as i32) {
                    unsafe { sqlite3VdbeAddOp2(v, 167 as i32, unsafe { (*pWin).regAccum }, nArg) };
                    unsafe {
                        sqlite3VdbeAppendP4(v, (unsafe { (*pWin).pWFunc }) as *mut (), -(8 as i32))
                    };
                    unsafe {
                        sqlite3VdbeAddOp2(v, 82 as i32, unsafe { (*pWin).regAccum }, unsafe {
                            (*pWin).regResult
                        })
                    };
                    unsafe {
                        sqlite3VdbeAddOp2(v, 77 as i32, 0 as i32, unsafe { (*pWin).regAccum })
                    };
                } else {
                    unsafe {
                        sqlite3VdbeAddOp3(
                            v,
                            166 as i32,
                            unsafe { (*pWin).regAccum },
                            nArg,
                            unsafe { (*pWin).regResult },
                        )
                    };
                    unsafe {
                        sqlite3VdbeAppendP4(v, (unsafe { (*pWin).pWFunc }) as *mut (), -(8 as i32))
                    };
                }
            }
        }
        pWin = unsafe { (*pWin).pNextWin };
    }
}

// /*
// ** Generate code to calculate the current values of all window functions in the
// ** p->pMWin list by doing a full scan of the current window frame. Store the
// ** results in the Window.regResult registers, ready to return the upper
// ** layer.
// */
fn windowFullScan(mut p: *mut WindowCodeArg) {
    let mut pWin: *mut Window = unsafe { std::mem::zeroed() };
    let mut pParse: *mut Parse = unsafe { (*p).pParse };
    let mut pMWin: *mut Window = unsafe { (*p).pMWin };
    let mut v: *mut Vdbe = unsafe { (*p).pVdbe };
    // /* Current rowid value */
    let mut regCRowid: i32 = 0 as i32;
    // /* Current peer values */
    let mut regCPeer: i32 = 0 as i32;
    // /* AggStep rowid value */
    let mut regRowid: i32 = 0 as i32;
    // /* AggStep peer values */
    let mut regPeer: i32 = 0 as i32;
    let mut nPeer: i32 = 0 as i32;
    let mut lblNext: i32 = 0 as i32;
    let mut lblBrk: i32 = 0 as i32;
    let mut addrNext: i32 = 0 as i32;
    let mut csr: i32 = 0 as i32;
    {}
    0 as i32;
    csr = unsafe { (*pMWin).csrApp };
    nPeer = if (unsafe { (*pMWin).pOrderBy }) != std::ptr::null_mut::<ExprList>() {
        unsafe { (*unsafe { (*pMWin).pOrderBy }).nExpr }
    } else {
        0 as i32
    };
    lblNext = unsafe { sqlite3VdbeMakeLabel(pParse) };
    lblBrk = unsafe { sqlite3VdbeMakeLabel(pParse) };
    regCRowid = unsafe { sqlite3GetTempReg(pParse) };
    regRowid = unsafe { sqlite3GetTempReg(pParse) };
    if nPeer != (0 as i32) {
        regCPeer = unsafe { sqlite3GetTempRange(pParse, nPeer) };
        regPeer = unsafe { sqlite3GetTempRange(pParse, nPeer) };
    }
    unsafe { sqlite3VdbeAddOp2(v, 137 as i32, unsafe { (*pMWin).iEphCsr }, regCRowid) };
    windowReadPeerValues(p, unsafe { (*pMWin).iEphCsr }, regCPeer);
    pWin = pMWin;
    '__slate_break_1131: while pWin != std::ptr::null_mut::<Window>() {
        unsafe { sqlite3VdbeAddOp2(v, 77 as i32, 0 as i32, unsafe { (*pWin).regAccum }) };
        pWin = unsafe { (*pWin).pNextWin };
    }
    unsafe { sqlite3VdbeAddOp3(v, 23 as i32, csr, lblBrk, unsafe { (*pMWin).regStartRowid }) };
    {}
    addrNext = unsafe { sqlite3VdbeCurrentAddr(v) };
    unsafe { sqlite3VdbeAddOp2(v, 137 as i32, csr, regRowid) };
    unsafe {
        sqlite3VdbeAddOp3(
            v,
            55 as i32,
            unsafe { (*pMWin).regEndRowid },
            lblBrk,
            regRowid,
        )
    };
    {}
    if (((unsafe { (*pMWin).eExclude }) as u32) as i32) == (86 as i32) {
        unsafe { sqlite3VdbeAddOp3(v, 54 as i32, regCRowid, lblNext, regRowid) };
        {}
    } else {
        if (((unsafe { (*pMWin).eExclude }) as u32) as i32) != (67 as i32) {
            let mut addr: i32 = 0 as i32;
            let mut addrEq: i32 = 0 as i32;
            let mut pKeyInfo: *mut KeyInfo = std::ptr::null_mut::<KeyInfo>();
            if (unsafe { (*pMWin).pOrderBy }) != std::ptr::null_mut::<ExprList>() {
                pKeyInfo = unsafe {
                    sqlite3KeyInfoFromExprList(
                        pParse,
                        unsafe { (*pMWin).pOrderBy },
                        0 as i32,
                        0 as i32,
                    )
                };
            }
            if (((unsafe { (*pMWin).eExclude }) as u32) as i32) == (95 as i32) {
                addrEq = unsafe { sqlite3VdbeAddOp3(v, 54 as i32, regCRowid, 0 as i32, regRowid) };
                {}
            }
            if pKeyInfo != std::ptr::null_mut::<KeyInfo>() {
                windowReadPeerValues(p, csr, regPeer);
                unsafe { sqlite3VdbeAddOp3(v, 92 as i32, regPeer, regCPeer, nPeer) };
                unsafe { sqlite3VdbeAppendP4(v, pKeyInfo as *mut (), -(9 as i32)) };
                addr = (unsafe { sqlite3VdbeCurrentAddr(v) }) + (1 as i32);
                unsafe { sqlite3VdbeAddOp3(v, 14 as i32, addr, lblNext, addr) };
                {}
            } else {
                unsafe { sqlite3VdbeAddOp2(v, 9 as i32, 0 as i32, lblNext) };
            }
            if addrEq != (0 as i32) {
                unsafe { sqlite3VdbeJumpHere(v, addrEq) };
            }
        }
    }
    windowAggStep(p, pMWin, csr, 0 as i32, unsafe { (*p).regArg });
    unsafe { sqlite3VdbeResolveLabel(v, lblNext) };
    unsafe { sqlite3VdbeAddOp2(v, 40 as i32, csr, addrNext) };
    {}
    unsafe { sqlite3VdbeJumpHere(v, addrNext - (1 as i32)) };
    unsafe { sqlite3VdbeJumpHere(v, addrNext + (1 as i32)) };
    unsafe { sqlite3ReleaseTempReg(pParse, regRowid) };
    unsafe { sqlite3ReleaseTempReg(pParse, regCRowid) };
    if nPeer != (0 as i32) {
        unsafe { sqlite3ReleaseTempRange(pParse, regPeer, nPeer) };
        unsafe { sqlite3ReleaseTempRange(pParse, regCPeer, nPeer) };
    }
    windowAggFinal(p, 1 as i32);
    {}
}

// /*
// ** Invoke the sub-routine at regGosub (generated by code in select.c) to
// ** return the current row of Window.iEphCsr. If all window functions are
// ** aggregate window functions that use the standard API, a single
// ** OP_Gosub instruction is all that this routine generates. Extra VM code
// ** for per-row processing is only generated for the following built-in window
// ** functions:
// **
// **   nth_value()
// **   first_value()
// **   lag()
// **   lead()
// */
fn windowReturnOneRow(mut p: *mut WindowCodeArg) {
    let mut pMWin: *mut Window = unsafe { (*p).pMWin };
    let mut v: *mut Vdbe = unsafe { (*p).pVdbe };
    if (unsafe { (*pMWin).regStartRowid }) != (0 as i32) {
        windowFullScan(p);
    } else {
        let mut pParse: *mut Parse = unsafe { (*p).pParse };
        let mut pWin: *mut Window = unsafe { std::mem::zeroed() };
        pWin = pMWin;
        '__slate_break_1132: while pWin != std::ptr::null_mut::<Window>() {
            let mut pFunc: *mut FuncDef = unsafe { (*pWin).pWFunc };
            0 as i32;
            if (unsafe { (*pFunc).zName })
                == unsafe { std::ptr::addr_of!(nth_valueName) as *const i8 }
                || (unsafe { (*pFunc).zName })
                    == unsafe { std::ptr::addr_of!(first_valueName) as *const i8 }
            {
                let mut csr: i32 = unsafe { (*pWin).csrApp };
                let mut lbl: i32 = unsafe { sqlite3VdbeMakeLabel(pParse) };
                let mut tmpReg: i32 = unsafe { sqlite3GetTempReg(pParse) };
                unsafe { sqlite3VdbeAddOp2(v, 77 as i32, 0 as i32, unsafe { (*pWin).regResult }) };
                if (unsafe { (*pFunc).zName })
                    == unsafe { std::ptr::addr_of!(nth_valueName) as *const i8 }
                {
                    unsafe {
                        sqlite3VdbeAddOp3(
                            v,
                            96 as i32,
                            unsafe { (*pMWin).iEphCsr },
                            (unsafe { (*pWin).iArgCol }) + (1 as i32),
                            tmpReg,
                        )
                    };
                    windowCheckValue(pParse, tmpReg, 2 as i32);
                } else {
                    unsafe { sqlite3VdbeAddOp2(v, 73 as i32, 1 as i32, tmpReg) };
                }
                unsafe {
                    sqlite3VdbeAddOp3(v, 107 as i32, tmpReg, unsafe { (*pWin).regApp }, tmpReg)
                };
                unsafe {
                    sqlite3VdbeAddOp3(
                        v,
                        55 as i32,
                        (unsafe { (*pWin).regApp }) + (1 as i32),
                        lbl,
                        tmpReg,
                    )
                };
                {}
                unsafe { sqlite3VdbeAddOp3(v, 30 as i32, csr, 0 as i32, tmpReg) };
                {}
                unsafe {
                    sqlite3VdbeAddOp3(v, 96 as i32, csr, unsafe { (*pWin).iArgCol }, unsafe {
                        (*pWin).regResult
                    })
                };
                unsafe { sqlite3VdbeResolveLabel(v, lbl) };
                unsafe { sqlite3ReleaseTempReg(pParse, tmpReg) };
            } else {
                if (unsafe { (*pFunc).zName })
                    == unsafe { std::ptr::addr_of!(leadName) as *const i8 }
                    || (unsafe { (*pFunc).zName })
                        == unsafe { std::ptr::addr_of!(lagName) as *const i8 }
                {
                    let mut nArg: i32 =
                        unsafe { (*unsafe { (*unsafe { (*pWin).pOwner }).x.pList }).nExpr };
                    let mut csr: i32 = unsafe { (*pWin).csrApp };
                    let mut lbl: i32 = unsafe { sqlite3VdbeMakeLabel(pParse) };
                    let mut tmpReg: i32 = unsafe { sqlite3GetTempReg(pParse) };
                    let mut iEph: i32 = unsafe { (*pMWin).iEphCsr };
                    if nArg < (3 as i32) {
                        unsafe {
                            sqlite3VdbeAddOp2(v, 77 as i32, 0 as i32, unsafe { (*pWin).regResult })
                        };
                    } else {
                        unsafe {
                            sqlite3VdbeAddOp3(
                                v,
                                96 as i32,
                                iEph,
                                (unsafe { (*pWin).iArgCol }) + (2 as i32),
                                unsafe { (*pWin).regResult },
                            )
                        };
                    }
                    unsafe { sqlite3VdbeAddOp2(v, 137 as i32, iEph, tmpReg) };
                    if nArg < (2 as i32) {
                        let mut val: i32 = if (unsafe { (*pFunc).zName })
                            == unsafe { std::ptr::addr_of!(leadName) as *const i8 }
                        {
                            1 as i32
                        } else {
                            -(1 as i32)
                        };
                        unsafe { sqlite3VdbeAddOp2(v, 88 as i32, tmpReg, val) };
                    } else {
                        let mut op: i32 = if (unsafe { (*pFunc).zName })
                            == unsafe { std::ptr::addr_of!(leadName) as *const i8 }
                        {
                            107 as i32
                        } else {
                            108 as i32
                        };
                        let mut tmpReg2: i32 = unsafe { sqlite3GetTempReg(pParse) };
                        unsafe {
                            sqlite3VdbeAddOp3(
                                v,
                                96 as i32,
                                iEph,
                                (unsafe { (*pWin).iArgCol }) + (1 as i32),
                                tmpReg2,
                            )
                        };
                        unsafe { sqlite3VdbeAddOp3(v, op, tmpReg2, tmpReg, tmpReg) };
                        unsafe { sqlite3ReleaseTempReg(pParse, tmpReg2) };
                    }
                    unsafe { sqlite3VdbeAddOp3(v, 30 as i32, csr, lbl, tmpReg) };
                    {}
                    unsafe {
                        sqlite3VdbeAddOp3(v, 96 as i32, csr, unsafe { (*pWin).iArgCol }, unsafe {
                            (*pWin).regResult
                        })
                    };
                    unsafe { sqlite3VdbeResolveLabel(v, lbl) };
                    unsafe { sqlite3ReleaseTempReg(pParse, tmpReg) };
                }
            }
            pWin = unsafe { (*pWin).pNextWin };
        }
    }
    unsafe {
        sqlite3VdbeAddOp2(v, 10 as i32, unsafe { (*p).regGosub }, unsafe {
            (*p).addrGosub
        })
    };
}

// /*
// ** Generate code to set the accumulator register for each window function
// ** in the linked list passed as the second argument to NULL. And perform
// ** any equivalent initialization required by any built-in window functions
// ** in the list.
// */
fn windowInitAccum(mut pParse: *mut Parse, mut pMWin: *mut Window) -> i32 {
    let mut v: *mut Vdbe = unsafe { sqlite3GetVdbe(pParse) };
    let mut regArg: i32 = 0 as i32;
    let mut nArg: i32 = 0 as i32;
    let mut pWin: *mut Window = unsafe { std::mem::zeroed() };
    pWin = pMWin;
    '__slate_break_1133: while pWin != std::ptr::null_mut::<Window>() {
        let mut pFunc: *mut FuncDef = unsafe { (*pWin).pWFunc };
        0 as i32;
        unsafe { sqlite3VdbeAddOp2(v, 77 as i32, 0 as i32, unsafe { (*pWin).regAccum }) };
        let __v1308: i32;
        if nArg > windowArgCount(pWin) {
            __v1308 = nArg;
        } else {
            __v1308 = windowArgCount(pWin);
        }
        nArg = __v1308;
        if (unsafe { (*pMWin).regStartRowid }) == (0 as i32) {
            if (unsafe { (*pFunc).zName })
                == unsafe { std::ptr::addr_of!(nth_valueName) as *const i8 }
                || (unsafe { (*pFunc).zName })
                    == unsafe { std::ptr::addr_of!(first_valueName) as *const i8 }
            {
                unsafe { sqlite3VdbeAddOp2(v, 73 as i32, 0 as i32, unsafe { (*pWin).regApp }) };
                unsafe {
                    sqlite3VdbeAddOp2(
                        v,
                        73 as i32,
                        0 as i32,
                        (unsafe { (*pWin).regApp }) + (1 as i32),
                    )
                };
            }
            if (unsafe { (*pFunc).funcFlags }) & ((4096 as i32) as u32) != (0 as u32)
                && (unsafe { (*pWin).csrApp }) != (0 as i32)
            {
                0 as i32;
                unsafe { sqlite3VdbeAddOp1(v, 148 as i32, unsafe { (*pWin).csrApp }) };
                unsafe {
                    sqlite3VdbeAddOp2(
                        v,
                        73 as i32,
                        0 as i32,
                        (unsafe { (*pWin).regApp }) + (1 as i32),
                    )
                };
            }
        }
        pWin = unsafe { (*pWin).pNextWin };
    }
    regArg = (unsafe { (*pParse).nMem }) + (1 as i32);
    let __v1309: *mut Parse = pParse;
    let __v1310: i32 = unsafe { (*__v1309).nMem };
    let __v1311: i32 = __v1310 + nArg;
    unsafe {
        (*__v1309).nMem = __v1311;
    }
    return regArg;
}

// /*
// ** Return true if the current frame should be cached in the ephemeral table,
// ** even if there are no xInverse() calls required.
// */
fn windowCacheFrame(mut pMWin: *mut Window) -> i32 {
    let mut pWin: *mut Window = unsafe { std::mem::zeroed() };
    if (unsafe { (*pMWin).regStartRowid }) != (0 as i32) {
        return 1 as i32;
    }
    pWin = pMWin;
    '__slate_break_1134: while pWin != std::ptr::null_mut::<Window>() {
        let mut pFunc: *mut FuncDef = unsafe { (*pWin).pWFunc };
        if (unsafe { (*pFunc).zName }) == unsafe { std::ptr::addr_of!(nth_valueName) as *const i8 }
            || (unsafe { (*pFunc).zName })
                == unsafe { std::ptr::addr_of!(first_valueName) as *const i8 }
            || (unsafe { (*pFunc).zName }) == unsafe { std::ptr::addr_of!(leadName) as *const i8 }
            || (unsafe { (*pFunc).zName }) == unsafe { std::ptr::addr_of!(lagName) as *const i8 }
        {
            return 1 as i32;
        }
        pWin = unsafe { (*pWin).pNextWin };
    }
    return 0 as i32;
}

// /*
// ** regOld and regNew are each the first register in an array of size
// ** pOrderBy->nExpr. This function generates code to compare the two
// ** arrays of registers using the collation sequences and other comparison
// ** parameters specified by pOrderBy.
// **
// ** If the two arrays are not equal, the contents of regNew is copied to
// ** regOld and control falls through. Otherwise, if the contents of the arrays
// ** are equal, an OP_Goto is executed. The address of the OP_Goto is returned.
// */
fn windowIfNewPeer(
    mut pParse: *mut Parse,
    mut pOrderBy: *mut ExprList,
    mut regNew: i32,
    mut regOld: i32,
    mut addr: i32,
) {
    let mut v: *mut Vdbe = unsafe { sqlite3GetVdbe(pParse) };
    if pOrderBy != std::ptr::null_mut::<ExprList>() {
        let mut nVal: i32 = unsafe { (*pOrderBy).nExpr };
        let mut pKeyInfo: *mut KeyInfo =
            unsafe { sqlite3KeyInfoFromExprList(pParse, pOrderBy, 0 as i32, 0 as i32) };
        unsafe { sqlite3VdbeAddOp3(v, 92 as i32, regOld, regNew, nVal) };
        unsafe { sqlite3VdbeAppendP4(v, pKeyInfo as *mut (), -(9 as i32)) };
        unsafe {
            sqlite3VdbeAddOp3(
                v,
                14 as i32,
                (unsafe { sqlite3VdbeCurrentAddr(v) }) + (1 as i32),
                addr,
                (unsafe { sqlite3VdbeCurrentAddr(v) }) + (1 as i32),
            )
        };
        {}
        unsafe { sqlite3VdbeAddOp3(v, 82 as i32, regNew, regOld, nVal - (1 as i32)) };
    } else {
        unsafe { sqlite3VdbeAddOp2(v, 9 as i32, 0 as i32, addr) };
    }
}

// /* First in array of new values */
// /* First in array of old values */
// /* Jump here */
// /*
// ** This function is called as part of generating VM programs for RANGE
// ** offset PRECEDING/FOLLOWING frame boundaries. Assuming "ASC" order for
// ** the ORDER BY term in the window, and that argument op is OP_Ge, it generates
// ** code equivalent to:
// **
// **   if( csr1.peerVal + regVal >= csr2.peerVal ) goto lbl;
// **
// ** The value of parameter op may also be OP_Gt or OP_Le. In these cases the
// ** operator in the above pseudo-code is replaced with ">" or "<=", respectively.
// **
// ** If the sort-order for the ORDER BY term in the window is DESC, then the
// ** comparison is reversed. Instead of adding regVal to csr1.peerVal, it is
// ** subtracted. And the comparison operator is inverted to - ">=" becomes "<=",
// ** ">" becomes "<", and so on. So, with DESC sort order, if the argument op
// ** is OP_Ge, the generated code is equivalent to:
// **
// **   if( csr1.peerVal - regVal <= csr2.peerVal ) goto lbl;
// **
// ** A special type of arithmetic is used such that if csr1.peerVal is not
// ** a numeric type (real or integer), then the result of the addition
// ** or subtraction is a a copy of csr1.peerVal.
// */
fn windowCodeRangeTest(
    mut p: *mut WindowCodeArg,
    mut op: i32,
    mut csr1: i32,
    mut regVal: i32,
    mut csr2: i32,
    mut lbl: i32,
) {
    let mut pParse: *mut Parse = unsafe { (*p).pParse };
    let mut v: *mut Vdbe = unsafe { sqlite3GetVdbe(pParse) };
    // /* ORDER BY clause for window */
    let mut pOrderBy: *mut ExprList = unsafe { (*unsafe { (*p).pMWin }).pOrderBy };
    // /* Reg. for csr1.peerVal+regVal */
    let mut reg1: i32 = unsafe { sqlite3GetTempReg(pParse) };
    // /* Reg. for csr2.peerVal */
    let mut reg2: i32 = unsafe { sqlite3GetTempReg(pParse) };
    // /* Reg. for constant value '' */
    let mut regString: i32 = 0 as i32;
    let __v1312: *mut Parse = pParse;
    let __v1313: i32 = unsafe { (*__v1312).nMem };
    let __v1314: i32 = __v1313 + (1 as i32);
    unsafe {
        (*__v1312).nMem = __v1314;
    }
    regString = __v1314;
    // /* OP_Add or OP_Subtract */
    let mut arith: i32 = 107 as i32;
    // /* Jump destination */
    let mut addrGe: i32 = 0 as i32;
    // /* Address past OP_Ge */
    let mut addrDone: i32 = unsafe { sqlite3VdbeMakeLabel(pParse) };
    let mut pColl: *mut CollSeq = unsafe { std::mem::zeroed() };
    // /* Read the peer-value from each cursor into a register */
    windowReadPeerValues(p, csr1, reg1);
    windowReadPeerValues(p, csr2, reg2);
    0 as i32;
    0 as i32;
    if (((unsafe {
        (*unsafe {
            unsafe { std::ptr::addr_of_mut!((*pOrderBy).a) as *mut ExprList_item }
                .offset((0 as i32) as isize)
        })
        .fg
        .sortFlags
    }) as u32) as i32)
        & (1 as i32)
        != (0 as i32)
    {
        '__slate_break_1135: {
            match op {
                58 => {
                    op = 56 as i32;
                }
                55 => {
                    op = 57 as i32;
                }
                _ => {
                    0 as i32;
                    op = 58 as i32;
                }
            }
        }
        arith = 108 as i32;
    }
    {}
    // /* If the BIGNULL flag is set for the ORDER BY, then it is required to
    //   ** consider NULL values to be larger than all other values, instead of
    //   ** the usual smaller. The VDBE opcodes OP_Ge and so on do not handle this
    //   ** (and adding that capability causes a performance regression), so
    //   ** instead if the BIGNULL flag is set then cases where either reg1 or
    //   ** reg2 are NULL are handled separately in the following block. The code
    //   ** generated is equivalent to:
    //   **
    //   **   if( reg1 IS NULL ){
    //   **     if( op==OP_Ge ) goto lbl;
    //   **     if( op==OP_Gt && reg2 IS NOT NULL ) goto lbl;
    //   **     if( op==OP_Le && reg2 IS NULL ) goto lbl;
    //   **   }else if( reg2 IS NULL ){
    //   **     if( op==OP_Le ) goto lbl;
    //   **   }
    //   **
    //   ** Additionally, if either reg1 or reg2 are NULL but the jump to lbl is
    //   ** not taken, control jumps over the comparison operator coded below this
    //   ** block.  */
    if (((unsafe {
        (*unsafe {
            unsafe { std::ptr::addr_of_mut!((*pOrderBy).a) as *mut ExprList_item }
                .offset((0 as i32) as isize)
        })
        .fg
        .sortFlags
    }) as u32) as i32)
        & (2 as i32)
        != (0 as i32)
    {
        // /* This block runs if reg1 contains a NULL. */
        let mut addr: i32 = unsafe { sqlite3VdbeAddOp1(v, 52 as i32, reg1) };
        {}
        '__slate_break_1136: {
            match op {
                58 => {
                    unsafe { sqlite3VdbeAddOp2(v, 9 as i32, 0 as i32, lbl) };
                }
                55 => {
                    unsafe { sqlite3VdbeAddOp2(v, 52 as i32, reg2, lbl) };
                    {}
                }
                56 => {
                    unsafe { sqlite3VdbeAddOp2(v, 51 as i32, reg2, lbl) };
                    {}
                    break '__slate_break_1136;
                    // /* no-op */
                }
                _ => {
                    0 as i32;
                }
            }
        }
        unsafe { sqlite3VdbeAddOp2(v, 9 as i32, 0 as i32, addrDone) };
        // /* This block runs if reg1 is not NULL, but reg2 is. */
        unsafe { sqlite3VdbeJumpHere(v, addr) };
        unsafe {
            sqlite3VdbeAddOp2(
                v,
                51 as i32,
                reg2,
                if op == (55 as i32) || op == (58 as i32) {
                    addrDone
                } else {
                    lbl
                },
            )
        };
        {}
    }
    // /* Register reg1 currently contains csr1.peerVal (the peer-value from csr1).
    //   ** This block adds (or subtracts for DESC) the numeric value in regVal
    //   ** from it. Or, if reg1 is not numeric (it is a NULL, a text value or a blob),
    //   ** then leave reg1 as it is. In pseudo-code, this is implemented as:
    //   **
    //   **   if( reg1>='' ) goto addrGe;
    //   **   reg1 = reg1 +/- regVal
    //   **   addrGe:
    //   **
    //   ** Since all strings and blobs are greater-than-or-equal-to an empty string,
    //   ** the add/subtract is skipped for these, as required. If reg1 is a NULL,
    //   ** then the arithmetic is performed, but since adding or subtracting from
    //   ** NULL is always NULL anyway, this case is handled as required too.  */
    unsafe {
        sqlite3VdbeAddOp4(
            v,
            118 as i32,
            0 as i32,
            regString,
            0 as i32,
            (b"\0".as_ptr() as *mut i8) as *const i8,
            -(1 as i32),
        )
    };
    addrGe = unsafe { sqlite3VdbeAddOp3(v, 58 as i32, regString, 0 as i32, reg1) };
    {}
    if op == (58 as i32) && arith == (107 as i32) || op == (56 as i32) && arith == (108 as i32) {
        unsafe { sqlite3VdbeAddOp3(v, op, reg2, lbl, reg1) };
        {}
    }
    unsafe { sqlite3VdbeAddOp3(v, arith, regVal, reg1, reg1) };
    unsafe { sqlite3VdbeJumpHere(v, addrGe) };
    // /* Compare registers reg2 and reg1, taking the jump if required. Note that
    //   ** control skips over this test if the BIGNULL flag is set and either
    //   ** reg1 or reg2 contain a NULL value.  */
    unsafe { sqlite3VdbeAddOp3(v, op, reg2, lbl, reg1) };
    {}
    pColl = unsafe {
        sqlite3ExprNNCollSeq(
            pParse,
            (unsafe {
                (*unsafe {
                    unsafe { std::ptr::addr_of_mut!((*pOrderBy).a) as *mut ExprList_item }
                        .offset((0 as i32) as isize)
                })
                .pExpr
            }) as *const Expr,
        )
    };
    unsafe { sqlite3VdbeAppendP4(v, pColl as *mut (), -(2 as i32)) };
    unsafe { sqlite3VdbeChangeP5(v, ((128 as i32) as i16) as u16) };
    unsafe { sqlite3VdbeResolveLabel(v, addrDone) };
    0 as i32;
    {}
    {}
    {}
    {}
    {}
    {}
    {}
    {}
    unsafe { sqlite3ReleaseTempReg(pParse, reg1) };
    unsafe { sqlite3ReleaseTempReg(pParse, reg2) };
    {}
}

// /* OP_Ge, OP_Gt, or OP_Le */
// /* Cursor number for cursor 1 */
// /* Register containing non-negative number */
// /* Cursor number for cursor 2 */
// /* Jump destination if condition is true */
// /*
// ** Helper function for sqlite3WindowCodeStep(). Each call to this function
// ** generates VM code for a single RETURN_ROW, AGGSTEP or AGGINVERSE
// ** operation. Refer to the header comment for sqlite3WindowCodeStep() for
// ** details.
// */
fn windowCodeOp(
    mut p: *mut WindowCodeArg,
    mut op: i32,
    mut regCountdown: i32,
    mut jumpOnEof: i32,
) -> i32 {
    let mut csr: i32 = 0 as i32;
    let mut reg: i32 = 0 as i32;
    let mut pParse: *mut Parse = unsafe { (*p).pParse };
    let mut pMWin: *mut Window = unsafe { (*p).pMWin };
    let mut ret: i32 = 0 as i32;
    let mut v: *mut Vdbe = unsafe { (*p).pVdbe };
    let mut addrContinue: i32 = 0 as i32;
    let mut bPeer: i32 = ((((unsafe { (*pMWin).eFrmType }) as u32) as i32) != (77 as i32)) as i32;
    let mut lblDone: i32 = unsafe { sqlite3VdbeMakeLabel(pParse) };
    let mut addrNextRange: i32 = 0 as i32;
    // /* Special case - WINDOW_AGGINVERSE is always a no-op if the frame
    //   ** starts with UNBOUNDED PRECEDING. */
    if op == (2 as i32) && (((unsafe { (*pMWin).eStart }) as u32) as i32) == (91 as i32) {
        0 as i32;
        return 0 as i32;
    }
    if regCountdown > (0 as i32) {
        if (((unsafe { (*pMWin).eFrmType }) as u32) as i32) == (90 as i32) {
            addrNextRange = unsafe { sqlite3VdbeCurrentAddr(v) };
            0 as i32;
            if op == (2 as i32) {
                if (((unsafe { (*pMWin).eStart }) as u32) as i32) == (87 as i32) {
                    windowCodeRangeTest(
                        p,
                        56 as i32,
                        unsafe { (*p).current.csr },
                        regCountdown,
                        unsafe { (*p).start.csr },
                        lblDone,
                    );
                } else {
                    windowCodeRangeTest(
                        p,
                        58 as i32,
                        unsafe { (*p).start.csr },
                        regCountdown,
                        unsafe { (*p).current.csr },
                        lblDone,
                    );
                }
            } else {
                windowCodeRangeTest(
                    p,
                    55 as i32,
                    unsafe { (*p).end.csr },
                    regCountdown,
                    unsafe { (*p).current.csr },
                    lblDone,
                );
            }
        } else {
            unsafe { sqlite3VdbeAddOp3(v, 61 as i32, regCountdown, lblDone, 1 as i32) };
            {}
        }
    }
    if op == (1 as i32) && (unsafe { (*pMWin).regStartRowid }) == (0 as i32) {
        windowAggFinal(p, 0 as i32);
    }
    addrContinue = unsafe { sqlite3VdbeCurrentAddr(v) };
    // /* If this is a (RANGE BETWEEN a FOLLOWING AND b FOLLOWING) or
    //   ** (RANGE BETWEEN b PRECEDING AND a PRECEDING) frame, ensure the
    //   ** start cursor does not advance past the end cursor within the
    //   ** temporary table. It otherwise might, if (a>b). Also ensure that,
    //   ** if the input cursor is still finding new rows, that the end
    //   ** cursor does not go past it to EOF. */
    if (((unsafe { (*pMWin).eStart }) as u32) as i32)
        == (((unsafe { (*pMWin).eEnd }) as u32) as i32)
        && regCountdown != (0 as i32)
        && (((unsafe { (*pMWin).eFrmType }) as u32) as i32) == (90 as i32)
    {
        let mut regRowid1: i32 = unsafe { sqlite3GetTempReg(pParse) };
        let mut regRowid2: i32 = unsafe { sqlite3GetTempReg(pParse) };
        if op == (2 as i32) {
            unsafe { sqlite3VdbeAddOp2(v, 137 as i32, unsafe { (*p).start.csr }, regRowid1) };
            unsafe { sqlite3VdbeAddOp2(v, 137 as i32, unsafe { (*p).end.csr }, regRowid2) };
            unsafe { sqlite3VdbeAddOp3(v, 58 as i32, regRowid2, lblDone, regRowid1) };
            {}
        } else {
            if (unsafe { (*p).regRowid }) != (0 as i32) {
                unsafe { sqlite3VdbeAddOp2(v, 137 as i32, unsafe { (*p).end.csr }, regRowid1) };
                unsafe {
                    sqlite3VdbeAddOp3(v, 58 as i32, unsafe { (*p).regRowid }, lblDone, regRowid1)
                };
                {}
            }
        }
        unsafe { sqlite3ReleaseTempReg(pParse, regRowid1) };
        unsafe { sqlite3ReleaseTempReg(pParse, regRowid2) };
        0 as i32;
    }
    '__slate_break_1138: {
        match op {
            1 => {
                csr = unsafe { (*p).current.csr };
                reg = unsafe { (*p).current.reg };
                windowReturnOneRow(p);
            }
            2 => {
                csr = unsafe { (*p).start.csr };
                reg = unsafe { (*p).start.reg };
                if (unsafe { (*pMWin).regStartRowid }) != (0 as i32) {
                    0 as i32;
                    unsafe {
                        sqlite3VdbeAddOp2(v, 88 as i32, unsafe { (*pMWin).regStartRowid }, 1 as i32)
                    };
                } else {
                    windowAggStep(p, pMWin, csr, 1 as i32, unsafe { (*p).regArg });
                }
            }
            _ => {
                0 as i32;
                csr = unsafe { (*p).end.csr };
                reg = unsafe { (*p).end.reg };
                if (unsafe { (*pMWin).regStartRowid }) != (0 as i32) {
                    0 as i32;
                    unsafe {
                        sqlite3VdbeAddOp2(v, 88 as i32, unsafe { (*pMWin).regEndRowid }, 1 as i32)
                    };
                } else {
                    windowAggStep(p, pMWin, csr, 0 as i32, unsafe { (*p).regArg });
                }
            }
        }
    }
    if op == unsafe { (*p).eDelete } {
        unsafe { sqlite3VdbeAddOp1(v, 132 as i32, csr) };
        unsafe { sqlite3VdbeChangeP5(v, ((2 as i32) as i16) as u16) };
    }
    if jumpOnEof != (0 as i32) {
        unsafe {
            sqlite3VdbeAddOp2(
                v,
                40 as i32,
                csr,
                (unsafe { sqlite3VdbeCurrentAddr(v) }) + (2 as i32),
            )
        };
        {}
        ret = unsafe { sqlite3VdbeAddOp0(v, 9 as i32) };
    } else {
        unsafe {
            sqlite3VdbeAddOp2(
                v,
                40 as i32,
                csr,
                (unsafe { sqlite3VdbeCurrentAddr(v) }) + (1 as i32) + bPeer,
            )
        };
        {}
        if bPeer != (0 as i32) {
            unsafe { sqlite3VdbeAddOp2(v, 9 as i32, 0 as i32, lblDone) };
        }
    }
    if bPeer != (0 as i32) {
        let mut nReg: i32 = if (unsafe { (*pMWin).pOrderBy }) != std::ptr::null_mut::<ExprList>() {
            unsafe { (*unsafe { (*pMWin).pOrderBy }).nExpr }
        } else {
            0 as i32
        };
        let mut regTmp: i32 = 0 as i32;
        let __v1315: i32;
        if nReg != (0 as i32) {
            __v1315 = unsafe { sqlite3GetTempRange(pParse, nReg) };
        } else {
            __v1315 = 0 as i32;
        }
        regTmp = __v1315;
        windowReadPeerValues(p, csr, regTmp);
        windowIfNewPeer(
            pParse,
            unsafe { (*pMWin).pOrderBy },
            regTmp,
            reg,
            addrContinue,
        );
        unsafe { sqlite3ReleaseTempRange(pParse, regTmp, nReg) };
    }
    if addrNextRange != (0 as i32) {
        unsafe { sqlite3VdbeAddOp2(v, 9 as i32, 0 as i32, addrNextRange) };
    }
    unsafe { sqlite3VdbeResolveLabel(v, lblDone) };
    return ret;
}

// /*
// ** Return true if it can be determined at compile time that expression
// ** pExpr evaluates to a value that, when cast to an integer, is greater
// ** than zero. False otherwise.
// **
// ** If an OOM error occurs, this function sets the Parse.db.mallocFailed
// ** flag and returns zero.
// */
fn windowExprGtZero(mut pParse: *mut Parse, mut pExpr: *mut Expr) -> i32 {
    let mut ret: i32 = 0 as i32;
    let mut db: *mut sqlite3 = unsafe { (*pParse).db };
    let mut pVal: *mut sqlite3_value = std::ptr::null_mut::<sqlite3_value>();
    unsafe {
        sqlite3ValueFromExpr(
            db,
            pExpr as *const Expr,
            unsafe { (*db).enc },
            ((67 as i32) as i8) as u8,
            std::ptr::addr_of_mut!(pVal),
        )
    };
    let __v1316: bool;
    if pVal != std::ptr::null_mut::<sqlite3_value>() {
        __v1316 = (unsafe { sqlite3_value_int(pVal) }) > (0 as i32);
    } else {
        __v1316 = false as bool;
    }
    if __v1316 {
        ret = 1 as i32;
    }
    unsafe { sqlite3ValueFree(pVal) };
    return ret;
}
