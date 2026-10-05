unsafe extern "C" {
    static mut sqlite3CtypeMap: [u8; 0];
    fn sqlite3_snprintf(__v949: i32, __v950: *mut i8, __v951: *const i8, ...) -> *mut i8;
    fn sqlite3_free(__v952: *mut ());
    fn sqlite3_value_int(__v953: *mut sqlite3_value) -> i32;
    fn sqlite3_value_text(__v954: *mut sqlite3_value) -> *const u8;
    fn sqlite3_value_type(__v955: *mut sqlite3_value) -> i32;
    fn sqlite3_context_db_handle(__v956: *mut sqlite3_context) -> *mut sqlite3;
    fn sqlite3_result_error(__v957: *mut sqlite3_context, __v958: *const i8, __v959: i32);
    fn sqlite3_result_error_nomem(__v960: *mut sqlite3_context);
    fn sqlite3_result_error_code(__v961: *mut sqlite3_context, __v962: i32);
    fn sqlite3_result_int(__v963: *mut sqlite3_context, __v964: i32);
    fn sqlite3_result_text(
        __v965: *mut sqlite3_context,
        __v966: *const i8,
        __v967: i32,
        __v968: Option<unsafe extern "C-unwind" fn(*mut ())>,
    );
    fn sqlite3_result_value(__v969: *mut sqlite3_context, __v970: *mut sqlite3_value);
    fn sqlite3_str_new(__v971: *mut sqlite3) -> *mut sqlite3_str;
    fn sqlite3_result_str(__v972: *mut sqlite3_context, __v973: *mut sqlite3_str, __v974: i32);
    fn sqlite3_str_appendf(__v975: *mut sqlite3_str, zFormat: *const i8, ...);
    fn sqlite3_str_append(__v977: *mut sqlite3_str, zIn: *const i8, N: i32);
    fn sqlite3_stricmp(__v980: *const i8, __v981: *const i8) -> i32;
    fn sqlite3_strnicmp(__v982: *const i8, __v983: *const i8, __v984: i32) -> i32;
    fn memcpy(__dest: *mut (), __src: *const (), __n: u64) -> *mut ();
    fn memmove(__dest: *mut (), __src: *const (), __n: u64) -> *mut ();
    fn memset(__s: *mut (), __c: i32, __n: u64) -> *mut ();
    fn sqlite3BtreeEnterAll(__v994: *mut sqlite3);
    fn sqlite3BtreeLeaveAll(__v995: *mut sqlite3);
    fn sqlite3VdbeAddOp1(__v996: *mut Vdbe, __v997: i32, __v998: i32) -> i32;
    fn sqlite3VdbeAddOp2(__v999: *mut Vdbe, __v1000: i32, __v1001: i32, __v1002: i32) -> i32;
    fn sqlite3VdbeLoadString(__v1003: *mut Vdbe, __v1004: i32, __v1005: *const i8) -> i32;
    fn sqlite3VdbeAddOp3(
        __v1006: *mut Vdbe,
        __v1007: i32,
        __v1008: i32,
        __v1009: i32,
        __v1010: i32,
    ) -> i32;
    fn sqlite3VdbeAddOp4(
        __v1011: *mut Vdbe,
        __v1012: i32,
        __v1013: i32,
        __v1014: i32,
        __v1015: i32,
        zP4: *const i8,
        __v1017: i32,
    ) -> i32;
    fn sqlite3VdbeAddOp4Int(
        __v1018: *mut Vdbe,
        __v1019: i32,
        __v1020: i32,
        __v1021: i32,
        __v1022: i32,
        __v1023: i32,
    ) -> i32;
    fn sqlite3VdbeAddParseSchemaOp(
        __v1024: *mut Vdbe,
        __v1025: i32,
        __v1026: *mut i8,
        __v1027: u16,
    );
    fn sqlite3VdbeChangeP5(__v1028: *mut Vdbe, P5: u16);
    fn sqlite3VdbeJumpHere(__v1030: *mut Vdbe, addr: i32);
    fn sqlite3VdbeUsesBtree(__v1032: *mut Vdbe, __v1033: i32);
    fn sqlite3VdbeFinalize(__v1034: *mut Vdbe) -> i32;
    fn sqlite3VdbeCurrentAddr(__v1035: *mut Vdbe) -> i32;
    fn sqlite3WalkExpr(__v1036: *mut Walker, __v1037: *mut Expr) -> i32;
    fn sqlite3WalkExprList(__v1038: *mut Walker, __v1039: *mut ExprList) -> i32;
    fn sqlite3WalkSelect(__v1040: *mut Walker, __v1041: *mut Select) -> i32;
    fn sqlite3CorruptError(__v1042: i32) -> i32;
    fn sqlite3IsIdChar(__v1043: u8) -> i32;
    fn sqlite3Strlen30(__v1044: *const i8) -> i32;
    fn sqlite3MallocZero(__v1045: u64) -> *mut ();
    fn sqlite3DbMallocZero(__v1046: *mut sqlite3, __v1047: u64) -> *mut ();
    fn sqlite3DbStrDup(__v1048: *mut sqlite3, __v1049: *const i8) -> *mut i8;
    fn sqlite3DbStrNDup(__v1050: *mut sqlite3, __v1051: *const i8, __v1052: u64) -> *mut i8;
    fn sqlite3DbFree(__v1053: *mut sqlite3, __v1054: *mut ());
    fn sqlite3MPrintf(__v1055: *mut sqlite3, __v1056: *const i8, ...) -> *mut i8;
    fn sqlite3VMPrintf(
        __v1057: *mut sqlite3,
        __v1058: *const i8,
        __v1059: core::ffi::VaList<'_>,
    ) -> *mut i8;
    fn sqlite3ErrorMsg(__v1060: *mut Parse, __v1061: *const i8, ...);
    fn sqlite3Dequote(__v1062: *mut i8);
    fn sqlite3RunParser(__v1063: *mut Parse, __v1064: *const i8) -> i32;
    fn sqlite3GetTempReg(__v1065: *mut Parse) -> i32;
    fn sqlite3ReleaseTempReg(__v1066: *mut Parse, __v1067: i32);
    fn sqlite3ExprDelete(__v1068: *mut sqlite3, __v1069: *mut Expr);
    fn sqlite3ColumnExpr(__v1070: *mut Table, __v1071: *mut Column) -> *mut Expr;
    fn sqlite3PrimaryKeyIndex(__v1072: *mut Table) -> *mut Index;
    fn sqlite3TableColumnToIndex(__v1073: *mut Index, __v1074: i32) -> i32;
    fn sqlite3RowSetClear(__v1075: *mut ());
    fn sqlite3ViewGetColumnNames(__v1076: *mut Parse, __v1077: *mut Table) -> i32;
    fn sqlite3DeleteTable(__v1078: *mut sqlite3, __v1079: *mut Table);
    fn sqlite3FreeIndex(__v1080: *mut sqlite3, __v1081: *mut Index);
    fn sqlite3SrcListDelete(__v1082: *mut sqlite3, __v1083: *mut SrcList);
    fn sqlite3SelectNew(
        __v1084: *mut Parse,
        __v1085: *mut ExprList,
        __v1086: *mut SrcList,
        __v1087: *mut Expr,
        __v1088: *mut ExprList,
        __v1089: *mut Expr,
        __v1090: *mut ExprList,
        __v1091: u32,
        __v1092: *mut Expr,
    ) -> *mut Select;
    fn sqlite3SelectDelete(__v1093: *mut sqlite3, __v1094: *mut Select);
    fn sqlite3OpenTable(
        __v1095: *mut Parse,
        iCur: i32,
        iDb: i32,
        __v1098: *mut Table,
        __v1099: i32,
    );
    fn sqlite3ExprCodeGetColumnOfTable(
        __v1100: *mut Vdbe,
        __v1101: *mut Table,
        __v1102: i32,
        __v1103: i32,
        __v1104: i32,
    );
    fn sqlite3FindTable(
        __v1105: *mut sqlite3,
        __v1106: *const i8,
        __v1107: *const i8,
    ) -> *mut Table;
    fn sqlite3LocateTableItem(__v1108: *mut Parse, flags: u32, __v1110: *mut SrcItem)
    -> *mut Table;
    fn sqlite3FindIndex(
        __v1111: *mut sqlite3,
        __v1112: *const i8,
        __v1113: *const i8,
    ) -> *mut Index;
    fn sqlite3NameFromToken(__v1114: *mut sqlite3, __v1115: *const Token) -> *mut i8;
    fn sqlite3GetVdbe(__v1116: *mut Parse) -> *mut Vdbe;
    fn sqlite3MayAbort(__v1117: *mut Parse);
    fn sqlite3ExprListDup(
        __v1118: *mut sqlite3,
        __v1119: *const ExprList,
        __v1120: i32,
    ) -> *mut ExprList;
    fn sqlite3SrcListDup(
        __v1121: *mut sqlite3,
        __v1122: *const SrcList,
        __v1123: i32,
    ) -> *mut SrcList;
    fn sqlite3InsertBuiltinFuncs(__v1124: *mut FuncDef, __v1125: i32);
    fn sqlite3ChangeCookie(__v1126: *mut Parse, __v1127: i32);
    fn sqlite3WithDup(db: *mut sqlite3, p: *mut With) -> *mut With;
    fn sqlite3DeleteTrigger(__v1130: *mut sqlite3, __v1131: *mut Trigger);
    fn sqlite3ColumnIndex(pTab: *mut Table, zCol: *const i8) -> i32;
    fn sqlite3AuthCheck(
        __v1134: *mut Parse,
        __v1135: i32,
        __v1136: *const i8,
        __v1137: *const i8,
        __v1138: *const i8,
    ) -> i32;
    fn sqlite3Utf8CharLen(pData: *const i8, nByte: i32) -> i32;
    fn sqlite3WritableSchema(__v1141: *mut sqlite3) -> i32;
    fn sqlite3CheckObjectName(
        __v1142: *mut Parse,
        __v1143: *const i8,
        __v1144: *const i8,
        __v1145: *const i8,
    ) -> i32;
    fn sqlite3ValueFree(__v1146: *mut sqlite3_value);
    fn sqlite3ValueFromExpr(
        __v1147: *mut sqlite3,
        __v1148: *const Expr,
        __v1149: u8,
        __v1150: u8,
        __v1151: *mut *mut sqlite3_value,
    ) -> i32;
    fn sqlite3GetToken(__v1174: *const u8, __v1175: *mut i32) -> i64;
    fn sqlite3NestedParse(__v1176: *mut Parse, __v1177: *const i8, ...);
    fn sqlite3SelectPrep(__v1178: *mut Parse, __v1179: *mut Select, __v1180: *mut NameContext);
    fn sqlite3StrIHash(__v1181: *const i8) -> u8;
    fn sqlite3ResolveExprNames(__v1182: *mut NameContext, __v1183: *mut Expr) -> i32;
    fn sqlite3ResolveExprListNames(__v1184: *mut NameContext, __v1185: *mut ExprList) -> i32;
    fn sqlite3ResolveSelfReference(
        __v1186: *mut Parse,
        __v1187: *mut Table,
        __v1188: i32,
        __v1189: *mut Expr,
        __v1190: *mut ExprList,
    ) -> i32;
    fn sqlite3FindDbName(__v1208: *mut sqlite3, __v1209: *const i8) -> i32;
    fn sqlite3SchemaToIndex(db: *mut sqlite3, __v1211: *mut Schema) -> i32;
    fn sqlite3GetVTable(__v1212: *mut sqlite3, __v1213: *mut Table) -> *mut VTable;
    fn sqlite3ReadOnlyShadowTables(db: *mut sqlite3) -> i32;
    fn sqlite3IsShadowTableOf(
        __v1215: *mut sqlite3,
        __v1216: *mut Table,
        __v1217: *const i8,
    ) -> i32;
    fn sqlite3ParseObjectInit(__v1218: *mut Parse, __v1219: *mut sqlite3);
    fn sqlite3ParseObjectReset(__v1220: *mut Parse);
    fn sqlite3WithPush(__v1221: *mut Parse, __v1222: *mut With, __v1223: u8) -> *mut With;
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
    __slate_bits_0: __slate_bits::__SlateBits67U0,
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
    u: __SlateRecord168,
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
    __slate_bits_0: __slate_bits::__SlateBits93U0,
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
    uNC: __SlateRecord192,
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
    __slate_bits_0: __slate_bits::__SlateBits107U0,
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
    u1: __SlateRecord194,
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
struct RenameToken {
    p: *const (),
    t: Token,
    pNext: *mut RenameToken,
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
    u: __SlateRecord197,
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
union __SlateRecord192 {
    pEList: *mut ExprList,
    pAggInfo: *mut AggInfo,
    pUpsert: *mut Upsert,
    iBaseReg: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord194 {
    cr: __SlateRecord195,
    d: __SlateRecord196,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord195 {
    addrCrTab: i32,
    regRowid: i32,
    regRoot: i32,
    constraintName: Token,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord196 {
    pReturning: *mut Returning,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord197 {
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
// ** 2005 February 15
// **
// ** The author disclaims copyright to this source code.  In place of
// ** a legal notice, here is a blessing:
// **
// **    May you do good and not evil.
// **    May you find forgiveness for yourself and forgive others.
// **    May you share freely, never taking more than you give.
// **
// *************************************************************************
// ** This file contains C code routines that used to generate VDBE code
// ** that implements the ALTER TABLE command.
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

#[repr(C)]
#[derive(Clone, Copy)]
struct WindowRewrite {}

#[repr(C)]
#[derive(Clone, Copy)]
struct WhereConst {}

// /* Parsing context */
// /* Table being altered.  pSrc->nSrc==1 */
// /* Name of column being changed */
// /* New column name */
// /*
// ** Each RenameToken object maps an element of the parse tree into
// ** the token that generated that element.  The parse tree element
// ** might be one of:
// **
// **     *  A pointer to an Expr that represents an ID
// **     *  The name of a table column in Column.zName
// **
// ** A list of RenameToken objects can be constructed during parsing.
// ** Each new object is created by sqlite3RenameTokenMap().
// ** As the parse tree is transformed, the sqlite3RenameTokenRemap()
// ** routine is used to keep the mapping current.
// **
// ** After the parse finishes, renameTokenFind() routine can be used
// ** to look up the actual token value that created some element in
// ** the parse tree.
// */
// /* Parse tree element created by token t */
// /* The token that created parse tree element p */
// /* Next is a list of all RenameToken objects */
// /*
// ** The context of an ALTER TABLE RENAME COLUMN operation that gets passed
// ** down into the Walker.
// */
#[repr(C)]
#[derive(Clone, Copy)]
struct RenameCtx {
    pList: *mut RenameToken,
    // /* List of tokens to overwrite */
    nList: i32,
    // /* Number of tokens in pList */
    iCol: i32,
    // /* Index of column being renamed */
    pTab: *mut Table,
    // /* Table being ALTERed */
    zOld: *const i8,
    // /* Old column name */
}

#[repr(C)]
#[derive(Clone, Copy)]
struct CoveringIndexCheck {}

#[repr(C)]
#[derive(Clone, Copy)]
struct CheckOnCtx {}

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
    pub struct __SlateBits67U0 {
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
    pub struct __SlateBits93U0 {
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
    pub struct __SlateBits107U0 {
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

static mut aAlterTableFuncs: __SlateAlign16<[FuncDef; 9]> = __SlateAlign16([
    FuncDef {
        nArg: (9 as i32) as i16,
        funcFlags: ((8388608 as i32) | (262144 as i32) | (1 as i32) | (2048 as i32)) as u32,
        pUserData: std::ptr::null_mut::<()>(),
        pNext: std::ptr::null_mut::<FuncDef>(),
        xSFunc: Some(renameColumnFunc),
        xFinalize: None,
        xValue: None,
        xInverse: None,
        zName: (b"sqlite_rename_column\0".as_ptr() as *mut i8) as *const i8,
        u: {
            let mut __t0: __SlateRecord168 = unsafe { std::mem::zeroed() };
            __t0.pHash = std::ptr::null_mut::<FuncDef>();
            __t0
        },
    },
    FuncDef {
        nArg: (7 as i32) as i16,
        funcFlags: ((8388608 as i32) | (262144 as i32) | (1 as i32) | (2048 as i32)) as u32,
        pUserData: std::ptr::null_mut::<()>(),
        pNext: std::ptr::null_mut::<FuncDef>(),
        xSFunc: Some(renameTableFunc),
        xFinalize: None,
        xValue: None,
        xInverse: None,
        zName: (b"sqlite_rename_table\0".as_ptr() as *mut i8) as *const i8,
        u: {
            let mut __t1: __SlateRecord168 = unsafe { std::mem::zeroed() };
            __t1.pHash = std::ptr::null_mut::<FuncDef>();
            __t1
        },
    },
    FuncDef {
        nArg: (7 as i32) as i16,
        funcFlags: ((8388608 as i32) | (262144 as i32) | (1 as i32) | (2048 as i32)) as u32,
        pUserData: std::ptr::null_mut::<()>(),
        pNext: std::ptr::null_mut::<FuncDef>(),
        xSFunc: Some(renameTableTest),
        xFinalize: None,
        xValue: None,
        xInverse: None,
        zName: (b"sqlite_rename_test\0".as_ptr() as *mut i8) as *const i8,
        u: {
            let mut __t2: __SlateRecord168 = unsafe { std::mem::zeroed() };
            __t2.pHash = std::ptr::null_mut::<FuncDef>();
            __t2
        },
    },
    FuncDef {
        nArg: (3 as i32) as i16,
        funcFlags: ((8388608 as i32) | (262144 as i32) | (1 as i32) | (2048 as i32)) as u32,
        pUserData: std::ptr::null_mut::<()>(),
        pNext: std::ptr::null_mut::<FuncDef>(),
        xSFunc: Some(dropColumnFunc),
        xFinalize: None,
        xValue: None,
        xInverse: None,
        zName: (b"sqlite_drop_column\0".as_ptr() as *mut i8) as *const i8,
        u: {
            let mut __t3: __SlateRecord168 = unsafe { std::mem::zeroed() };
            __t3.pHash = std::ptr::null_mut::<FuncDef>();
            __t3
        },
    },
    FuncDef {
        nArg: (2 as i32) as i16,
        funcFlags: ((8388608 as i32) | (262144 as i32) | (1 as i32) | (2048 as i32)) as u32,
        pUserData: std::ptr::null_mut::<()>(),
        pNext: std::ptr::null_mut::<FuncDef>(),
        xSFunc: Some(renameQuotefixFunc),
        xFinalize: None,
        xValue: None,
        xInverse: None,
        zName: (b"sqlite_rename_quotefix\0".as_ptr() as *mut i8) as *const i8,
        u: {
            let mut __t4: __SlateRecord168 = unsafe { std::mem::zeroed() };
            __t4.pHash = std::ptr::null_mut::<FuncDef>();
            __t4
        },
    },
    FuncDef {
        nArg: (2 as i32) as i16,
        funcFlags: ((8388608 as i32) | (262144 as i32) | (1 as i32) | (2048 as i32)) as u32,
        pUserData: std::ptr::null_mut::<()>(),
        pNext: std::ptr::null_mut::<FuncDef>(),
        xSFunc: Some(dropConstraintFunc),
        xFinalize: None,
        xValue: None,
        xInverse: None,
        zName: (b"sqlite_drop_constraint\0".as_ptr() as *mut i8) as *const i8,
        u: {
            let mut __t5: __SlateRecord168 = unsafe { std::mem::zeroed() };
            __t5.pHash = std::ptr::null_mut::<FuncDef>();
            __t5
        },
    },
    FuncDef {
        nArg: (2 as i32) as i16,
        funcFlags: ((8388608 as i32) | (262144 as i32) | (1 as i32) | (2048 as i32)) as u32,
        pUserData: std::ptr::null_mut::<()>(),
        pNext: std::ptr::null_mut::<FuncDef>(),
        xSFunc: Some(failConstraintFunc),
        xFinalize: None,
        xValue: None,
        xInverse: None,
        zName: (b"sqlite_fail\0".as_ptr() as *mut i8) as *const i8,
        u: {
            let mut __t6: __SlateRecord168 = unsafe { std::mem::zeroed() };
            __t6.pHash = std::ptr::null_mut::<FuncDef>();
            __t6
        },
    },
    FuncDef {
        nArg: (3 as i32) as i16,
        funcFlags: ((8388608 as i32) | (262144 as i32) | (1 as i32) | (2048 as i32)) as u32,
        pUserData: std::ptr::null_mut::<()>(),
        pNext: std::ptr::null_mut::<FuncDef>(),
        xSFunc: Some(addConstraintFunc),
        xFinalize: None,
        xValue: None,
        xInverse: None,
        zName: (b"sqlite_add_constraint\0".as_ptr() as *mut i8) as *const i8,
        u: {
            let mut __t7: __SlateRecord168 = unsafe { std::mem::zeroed() };
            __t7.pHash = std::ptr::null_mut::<FuncDef>();
            __t7
        },
    },
    FuncDef {
        nArg: (2 as i32) as i16,
        funcFlags: ((8388608 as i32) | (262144 as i32) | (1 as i32) | (2048 as i32)) as u32,
        pUserData: std::ptr::null_mut::<()>(),
        pNext: std::ptr::null_mut::<FuncDef>(),
        xSFunc: Some(findConstraintFunc),
        xFinalize: None,
        xValue: None,
        xInverse: None,
        zName: (b"sqlite_find_constraint\0".as_ptr() as *mut i8) as *const i8,
        u: {
            let mut __t8: __SlateRecord168 = unsafe { std::mem::zeroed() };
            __t8.pHash = std::ptr::null_mut::<FuncDef>();
            __t8
        },
    },
]);

// /* Parse context */
// /* Table to add constraint to */
// /* First token of new constraint */
// /* Name of new constraint. NULL if name omitted. */
// /* Text of CHECK expression */
// /* Size of pExpr in bytes */
// /* The parsed CHECK expression */
// /*
// ** Register built-in functions used to help implement ALTER TABLE
// */
// /* SQLITE_ALTER_TABLE */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3AlterFunctions() {
    unsafe {
        sqlite3InsertBuiltinFuncs(
            unsafe { std::ptr::addr_of_mut!(aAlterTableFuncs.0) as *mut FuncDef },
            (((648 as u64) / (72 as u64)) as u32) as i32,
        )
    };
}

// /*
// ** Generate code to implement the "ALTER TABLE xxx RENAME TO yyy"
// ** command.
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3AlterRenameTable(
    mut pParse: *mut Parse,
    mut pSrc: *mut SrcList,
    mut pName: *mut Token,
) {
    let mut __slate_storage_1358: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1358: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1358) as *mut i32;
    let mut __slate_storage_1357: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1357: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1357) as *mut i32;
    let mut __slate_storage_1356: std::mem::MaybeUninit<*mut Parse> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1356: *mut *mut Parse =
        std::ptr::addr_of_mut!(__slate_storage_1356) as *mut *mut Parse;
    let mut __slate_storage_466: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_466: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_466) as *mut i32;
    let mut __slate_storage_1355: std::mem::MaybeUninit<bool> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1355: *mut bool = std::ptr::addr_of_mut!(__slate_storage_1355) as *mut bool;
    let mut __slate_storage_1354: std::mem::MaybeUninit<bool> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1354: *mut bool = std::ptr::addr_of_mut!(__slate_storage_1354) as *mut bool;
    let mut __slate_storage_465: std::mem::MaybeUninit<*mut VTable> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_465: *mut *mut VTable =
        std::ptr::addr_of_mut!(__slate_storage_465) as *mut *mut VTable;
    let mut __slate_storage_464: std::mem::MaybeUninit<*mut Vdbe> = std::mem::MaybeUninit::uninit();
    let __slate_slot_464: *mut *mut Vdbe =
        std::ptr::addr_of_mut!(__slate_storage_464) as *mut *mut Vdbe;
    let mut __slate_storage_463: std::mem::MaybeUninit<*const i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_463: *mut *const i8 =
        std::ptr::addr_of_mut!(__slate_storage_463) as *mut *const i8;
    let mut __slate_storage_462: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_462: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_462) as *mut i32;
    let mut __slate_storage_461: std::mem::MaybeUninit<*mut sqlite3> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_461: *mut *mut sqlite3 =
        std::ptr::addr_of_mut!(__slate_storage_461) as *mut *mut sqlite3;
    let mut __slate_storage_460: std::mem::MaybeUninit<*mut i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_460: *mut *mut i8 =
        std::ptr::addr_of_mut!(__slate_storage_460) as *mut *mut i8;
    let mut __slate_storage_459: std::mem::MaybeUninit<*mut Table> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_459: *mut *mut Table =
        std::ptr::addr_of_mut!(__slate_storage_459) as *mut *mut Table;
    let mut __slate_storage_458: std::mem::MaybeUninit<*mut i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_458: *mut *mut i8 =
        std::ptr::addr_of_mut!(__slate_storage_458) as *mut *mut i8;
    let mut __slate_storage_457: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_457: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_457) as *mut i32;
    unsafe {
        '__join_0: {
            // /* Database that contains the table */
            // /* Name of database iDb */
            // /* Table being renamed */
            // /* NULL-terminated version of pName */
            std::ptr::write(__slate_slot_460, std::ptr::null_mut::<i8>());
            // /* Database connection */
            std::ptr::write(__slate_slot_461, unsafe { (*pParse).db });
            // /* Number of UTF-8 characters in zTabName */
            // /* Original name of the table */
            // /* Non-zero if this is a v-tab with an xRename() */
            std::ptr::write(__slate_slot_465, std::ptr::null_mut::<VTable>());
            if (unsafe { (*(*__slate_slot_461)).mallocFailed }) != (0 as u8) {
            } else {
                0 as i32;
                0 as i32;
                *__slate_slot_459 = unsafe {
                    sqlite3LocateTableItem(pParse, (0 as i32) as u32, unsafe {
                        unsafe { std::ptr::addr_of_mut!((*pSrc).a) as *mut SrcItem }
                            .offset((0 as i32) as isize)
                    })
                };
                if !(*__slate_slot_459 != std::ptr::null_mut::<Table>()) {
                } else {
                    *__slate_slot_457 = unsafe {
                        sqlite3SchemaToIndex(unsafe { (*pParse).db }, unsafe {
                            (*(*__slate_slot_459)).pSchema
                        })
                    };
                    *__slate_slot_458 = unsafe {
                        (*unsafe {
                            unsafe { (*(*__slate_slot_461)).aDb }.offset(*__slate_slot_457 as isize)
                        })
                        .zDbSName
                    };
                    // /* Get a NULL terminated version of the new table name. */
                    *__slate_slot_460 =
                        unsafe { sqlite3NameFromToken(*__slate_slot_461, pName as *const Token) };
                    if !(*__slate_slot_460 != std::ptr::null_mut::<i8>()) {
                    } else {
                        // /* Check that a table or index named 'zName' does not already exist
                        //   ** in database iDb. If so, this is an error.
                        //   */
                        if (unsafe {
                            sqlite3FindTable(
                                *__slate_slot_461,
                                *__slate_slot_460 as *const i8,
                                *__slate_slot_458 as *const i8,
                            )
                        }) != std::ptr::null_mut::<Table>()
                        {
                            *__slate_slot_1354 = true as bool;
                        } else {
                            *__slate_slot_1354 = (unsafe {
                                sqlite3FindIndex(
                                    *__slate_slot_461,
                                    *__slate_slot_460 as *const i8,
                                    *__slate_slot_458 as *const i8,
                                )
                            }) != std::ptr::null_mut::<Index>();
                        }
                        if *__slate_slot_1354 {
                            *__slate_slot_1355 = true as bool;
                        } else {
                            *__slate_slot_1355 = (unsafe {
                                sqlite3IsShadowTableOf(
                                    *__slate_slot_461,
                                    *__slate_slot_459,
                                    *__slate_slot_460 as *const i8,
                                )
                            }) != (0 as i32);
                        }
                        if *__slate_slot_1355 {
                            unsafe {
                                sqlite3ErrorMsg(
                                    pParse,
                                    (b"there is already another table or index with this name: %s\0"
                                        .as_ptr() as *mut i8)
                                        as *const i8,
                                    *__slate_slot_460,
                                )
                            };
                        } else {
                            // /* Make sure it is not a system table being altered, or a reserved name
                            //   ** that the table is being renamed to.
                            //   */
                            if (0 as i32) != isAlterableTable(pParse, *__slate_slot_459) {
                            } else {
                                if (0 as i32)
                                    != unsafe {
                                        sqlite3CheckObjectName(
                                            pParse,
                                            *__slate_slot_460 as *const i8,
                                            (b"table\0".as_ptr() as *mut i8) as *const i8,
                                            *__slate_slot_460 as *const i8,
                                        )
                                    }
                                {
                                } else {
                                    if (((unsafe { (*(*__slate_slot_459)).eTabType }) as u32)
                                        as i32)
                                        == (2 as i32)
                                    {
                                        unsafe {
                                            sqlite3ErrorMsg(
                                                pParse,
                                                (b"view %s may not be altered\0".as_ptr()
                                                    as *mut i8)
                                                    as *const i8,
                                                unsafe { (*(*__slate_slot_459)).zName },
                                            )
                                        };
                                    } else {
                                        // /* Invoke the authorization callback. */
                                        if (unsafe {
                                            sqlite3AuthCheck(
                                                pParse,
                                                26 as i32,
                                                *__slate_slot_458 as *const i8,
                                                (unsafe { (*(*__slate_slot_459)).zName })
                                                    as *const i8,
                                                std::ptr::null::<i8>(),
                                            )
                                        }) != (0 as i32)
                                        {
                                        } else {
                                            if (unsafe {
                                                sqlite3ViewGetColumnNames(pParse, *__slate_slot_459)
                                            }) != (0 as i32)
                                            {
                                            } else {
                                                if (((unsafe { (*(*__slate_slot_459)).eTabType })
                                                    as u32)
                                                    as i32)
                                                    == (1 as i32)
                                                {
                                                    *__slate_slot_465 = unsafe {
                                                        sqlite3GetVTable(
                                                            *__slate_slot_461,
                                                            *__slate_slot_459,
                                                        )
                                                    };
                                                    if (unsafe {
                                                        (*unsafe {
                                                            (*unsafe {
                                                                (*(*__slate_slot_465)).pVtab
                                                            })
                                                            .pModule
                                                        })
                                                        .xRename
                                                    }) == None
                                                    {
                                                        *__slate_slot_465 =
                                                            std::ptr::null_mut::<VTable>();
                                                    }
                                                }
                                                // /* Begin a transaction for database iDb. Then modify the schema cookie
                                                //   ** (since the ALTER TABLE modifies the schema). Call sqlite3MayAbort(),
                                                //   ** as the scalar functions (e.g. sqlite_rename_table()) invoked by the
                                                //   ** nested SQL may raise an exception.  */
                                                *__slate_slot_464 =
                                                    unsafe { sqlite3GetVdbe(pParse) };
                                                if *__slate_slot_464 == std::ptr::null_mut::<Vdbe>()
                                                {
                                                } else {
                                                    unsafe { sqlite3MayAbort(pParse) };
                                                    // /* figure out how many UTF-8 characters are in zName */
                                                    *__slate_slot_463 =
                                                        (unsafe { (*(*__slate_slot_459)).zName })
                                                            as *const i8;
                                                    *__slate_slot_462 = unsafe {
                                                        sqlite3Utf8CharLen(
                                                            *__slate_slot_463,
                                                            -(1 as i32),
                                                        )
                                                    };
                                                    // /* Rewrite all CREATE TABLE, INDEX, TRIGGER or VIEW statements in
                                                    //   ** the schema to use the new table name.  */
                                                    unsafe {
                                                        sqlite3NestedParse(pParse, (b"UPDATE \"%w\".sqlite_master SET sql = sqlite_rename_table(%Q, type, name, sql, %Q, %Q, %d) WHERE (type!='index' OR tbl_name=%Q COLLATE nocase)AND   name NOT LIKE 'sqliteX_%%' ESCAPE 'X'\0".as_ptr() as *mut i8) as *const i8, *__slate_slot_458, *__slate_slot_458, *__slate_slot_463, *__slate_slot_460, (*__slate_slot_457 == (1 as i32)) as i32, *__slate_slot_463)
                                                    };
                                                    // /* Update the tbl_name and name columns of the sqlite_schema table
                                                    //   ** as required.  */
                                                    unsafe {
                                                        sqlite3NestedParse(pParse, (b"UPDATE %Q.sqlite_master SET tbl_name = %Q, name = CASE WHEN type='table' THEN %Q WHEN name LIKE 'sqliteX_autoindex%%' ESCAPE 'X'      AND type='index' THEN 'sqlite_autoindex_' || %Q || substr(name,%d+18) ELSE name END WHERE tbl_name=%Q COLLATE nocase AND (type='table' OR type='index' OR type='trigger');\0".as_ptr() as *mut i8) as *const i8, *__slate_slot_458, *__slate_slot_460, *__slate_slot_460, *__slate_slot_460, *__slate_slot_462, *__slate_slot_463)
                                                    };
                                                    // /* If the sqlite_sequence table exists in this database, then update
                                                    //   ** it with the new table name.
                                                    //   */
                                                    if (unsafe {
                                                        sqlite3FindTable(
                                                            *__slate_slot_461,
                                                            (b"sqlite_sequence\0".as_ptr()
                                                                as *mut i8)
                                                                as *const i8,
                                                            *__slate_slot_458 as *const i8,
                                                        )
                                                    }) != std::ptr::null_mut::<Table>()
                                                    {
                                                        unsafe {
                                                            sqlite3NestedParse(pParse, (b"UPDATE \"%w\".sqlite_sequence set name = %Q WHERE name = %Q\0".as_ptr() as *mut i8) as *const i8, *__slate_slot_458, *__slate_slot_460, unsafe { (*(*__slate_slot_459)).zName })
                                                        };
                                                    }
                                                    // /* If the table being renamed is not itself part of the temp database,
                                                    //   ** edit view and trigger definitions within the temp database
                                                    //   ** as required.  */
                                                    if *__slate_slot_457 != (1 as i32) {
                                                        unsafe {
                                                            sqlite3NestedParse(pParse, (b"UPDATE sqlite_temp_schema SET sql = sqlite_rename_table(%Q, type, name, sql, %Q, %Q, 1), tbl_name = CASE WHEN tbl_name=%Q COLLATE nocase AND   sqlite_rename_test(%Q, sql, type, name, 1, 'after rename', 0) THEN %Q ELSE tbl_name END WHERE type IN ('view', 'trigger')\0".as_ptr() as *mut i8) as *const i8, *__slate_slot_458, *__slate_slot_463, *__slate_slot_460, *__slate_slot_463, *__slate_slot_458, *__slate_slot_460)
                                                        };
                                                    }
                                                    // /* If this is a virtual table, invoke the xRename() function if
                                                    //   ** one is defined. The xRename() callback will modify the names
                                                    //   ** of any resources used by the v-table implementation (including other
                                                    //   ** SQLite tables) that are identified by the name of the virtual table.
                                                    //   */
                                                    if *__slate_slot_465
                                                        != std::ptr::null_mut::<VTable>()
                                                    {
                                                        std::ptr::write(__slate_slot_1356, pParse);
                                                        std::ptr::write(
                                                            __slate_slot_1357,
                                                            unsafe { (*(*__slate_slot_1356)).nMem },
                                                        );
                                                        std::ptr::write(
                                                            __slate_slot_1358,
                                                            *__slate_slot_1357 + (1 as i32),
                                                        );
                                                        unsafe {
                                                            (*(*__slate_slot_1356)).nMem =
                                                                *__slate_slot_1358;
                                                        }
                                                        *__slate_slot_466 = *__slate_slot_1358;
                                                        unsafe {
                                                            sqlite3VdbeLoadString(
                                                                *__slate_slot_464,
                                                                *__slate_slot_466,
                                                                *__slate_slot_460 as *const i8,
                                                            )
                                                        };
                                                        unsafe {
                                                            sqlite3VdbeAddOp4(
                                                                *__slate_slot_464,
                                                                179 as i32,
                                                                *__slate_slot_466,
                                                                0 as i32,
                                                                0 as i32,
                                                                *__slate_slot_465 as *const i8,
                                                                -(12 as i32),
                                                            )
                                                        };
                                                    }
                                                    renameReloadSchema(
                                                        pParse,
                                                        *__slate_slot_457,
                                                        ((1 as i32) as i16) as u16,
                                                    );
                                                    renameTestSchema(
                                                        pParse,
                                                        *__slate_slot_458 as *const i8,
                                                        (*__slate_slot_457 == (1 as i32)) as i32,
                                                        (b"after rename\0".as_ptr() as *mut i8)
                                                            as *const i8,
                                                        0 as i32,
                                                    );
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
        }
        unsafe { sqlite3SrcListDelete(*__slate_slot_461, pSrc) };
        unsafe { sqlite3DbFree(*__slate_slot_461, *__slate_slot_460 as *mut ()) };
    }
}

// /* !defined(SQLITE_OMIT_VIEW) || !defined(SQLITE_OMIT_VIRTUALTABLE) */
// /*
// ** Handles the following parser reduction:
// **
// **  cmd ::= ALTER TABLE pSrc RENAME COLUMN pOld TO pNew
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3AlterRenameColumn(
    mut pParse: *mut Parse,
    mut pSrc: *mut SrcList,
    mut pOld: *mut Token,
    mut pNew: *mut Token,
) {
    let mut __slate_storage_516: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_516: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_516) as *mut i32;
    let mut __slate_storage_515: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_515: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_515) as *mut i32;
    let mut __slate_storage_514: std::mem::MaybeUninit<*const i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_514: *mut *const i8 =
        std::ptr::addr_of_mut!(__slate_storage_514) as *mut *const i8;
    let mut __slate_storage_513: std::mem::MaybeUninit<*mut i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_513: *mut *mut i8 =
        std::ptr::addr_of_mut!(__slate_storage_513) as *mut *mut i8;
    let mut __slate_storage_512: std::mem::MaybeUninit<*mut i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_512: *mut *mut i8 =
        std::ptr::addr_of_mut!(__slate_storage_512) as *mut *mut i8;
    let mut __slate_storage_511: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_511: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_511) as *mut i32;
    let mut __slate_storage_510: std::mem::MaybeUninit<*mut Table> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_510: *mut *mut Table =
        std::ptr::addr_of_mut!(__slate_storage_510) as *mut *mut Table;
    let mut __slate_storage_509: std::mem::MaybeUninit<*mut sqlite3> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_509: *mut *mut sqlite3 =
        std::ptr::addr_of_mut!(__slate_storage_509) as *mut *mut sqlite3;
    unsafe {
        '__join_0: {
            // /* Database connection */
            std::ptr::write(__slate_slot_509, unsafe { (*pParse).db });
            // /* Table being updated */
            // /* Index of column being renamed */
            // /* Old column name */
            std::ptr::write(__slate_slot_512, std::ptr::null_mut::<i8>());
            // /* New column name */
            std::ptr::write(__slate_slot_513, std::ptr::null_mut::<i8>());
            // /* Name of schema containing the table */
            // /* Index of the schema */
            // /* True to quote the new name */
            // /* Locate the table to be altered */
            *__slate_slot_510 = unsafe {
                sqlite3LocateTableItem(pParse, (0 as i32) as u32, unsafe {
                    unsafe { std::ptr::addr_of_mut!((*pSrc).a) as *mut SrcItem }
                        .offset((0 as i32) as isize)
                })
            };
            if !(*__slate_slot_510 != std::ptr::null_mut::<Table>()) {
            } else {
                // /* Cannot alter a system table */
                if (0 as i32) != isAlterableTable(pParse, *__slate_slot_510) {
                } else {
                    if (0 as i32) != isRealTable(pParse, *__slate_slot_510, 0 as i32) {
                    } else {
                        // /* Which schema holds the table to be altered */
                        *__slate_slot_515 = unsafe {
                            sqlite3SchemaToIndex(*__slate_slot_509, unsafe {
                                (*(*__slate_slot_510)).pSchema
                            })
                        };
                        0 as i32;
                        *__slate_slot_514 = (unsafe {
                            (*unsafe {
                                unsafe { (*(*__slate_slot_509)).aDb }
                                    .offset(*__slate_slot_515 as isize)
                            })
                            .zDbSName
                        }) as *const i8;
                        // /* Invoke the authorization callback. */
                        if (unsafe {
                            sqlite3AuthCheck(
                                pParse,
                                26 as i32,
                                *__slate_slot_514,
                                (unsafe { (*(*__slate_slot_510)).zName }) as *const i8,
                                std::ptr::null::<i8>(),
                            )
                        }) != (0 as i32)
                        {
                        } else {
                            // /* Make sure the old name really is a column name in the table to be
                            //   ** altered.  Set iCol to be the index of the column being renamed */
                            *__slate_slot_512 = unsafe {
                                sqlite3NameFromToken(*__slate_slot_509, pOld as *const Token)
                            };
                            if !(*__slate_slot_512 != std::ptr::null_mut::<i8>()) {
                            } else {
                                *__slate_slot_511 = unsafe {
                                    sqlite3ColumnIndex(
                                        *__slate_slot_510,
                                        *__slate_slot_512 as *const i8,
                                    )
                                };
                                if *__slate_slot_511 < (0 as i32) {
                                    unsafe {
                                        sqlite3ErrorMsg(
                                            pParse,
                                            (b"no such column: \"%T\"\0".as_ptr() as *mut i8)
                                                as *const i8,
                                            pOld,
                                        )
                                    };
                                } else {
                                    // /* Ensure the schema contains no double-quoted strings */
                                    renameTestSchema(
                                        pParse,
                                        *__slate_slot_514,
                                        (*__slate_slot_515 == (1 as i32)) as i32,
                                        (b"\0".as_ptr() as *mut i8) as *const i8,
                                        0 as i32,
                                    );
                                    renameFixQuotes(
                                        pParse,
                                        *__slate_slot_514,
                                        (*__slate_slot_515 == (1 as i32)) as i32,
                                    );
                                    // /* Do the rename operation using a recursive UPDATE statement that
                                    //   ** uses the sqlite_rename_column() SQL function to compute the new
                                    //   ** CREATE statement text for the sqlite_schema table.
                                    //   */
                                    unsafe { sqlite3MayAbort(pParse) };
                                    *__slate_slot_513 = unsafe {
                                        sqlite3NameFromToken(
                                            *__slate_slot_509,
                                            pNew as *const Token,
                                        )
                                    };
                                    if !(*__slate_slot_513 != std::ptr::null_mut::<i8>()) {
                                    } else {
                                        0 as i32;
                                        *__slate_slot_516 = (((unsafe {
                                            *unsafe {
                                                unsafe {
                                                    std::ptr::addr_of!(sqlite3CtypeMap) as *const u8
                                                }
                                                .offset(
                                                    ((((unsafe {
                                                        *unsafe {
                                                            unsafe { (*pNew).z }
                                                                .offset((0 as i32) as isize)
                                                        }
                                                    })
                                                        as u8)
                                                        as u32)
                                                        as i32)
                                                        as isize,
                                                )
                                            }
                                        })
                                            as u32)
                                            as i32)
                                            & (128 as i32);
                                        unsafe {
                                            sqlite3NestedParse(pParse, (b"UPDATE \"%w\".sqlite_master SET sql = sqlite_rename_column(sql, type, name, %Q, %Q, %d, %Q, %d, %d) WHERE name NOT LIKE 'sqliteX_%%' ESCAPE 'X'  AND (type != 'index' OR tbl_name = %Q)\0".as_ptr() as *mut i8) as *const i8, *__slate_slot_514, *__slate_slot_514, unsafe { (*(*__slate_slot_510)).zName }, *__slate_slot_511, *__slate_slot_513, *__slate_slot_516, (*__slate_slot_515 == (1 as i32)) as i32, unsafe { (*(*__slate_slot_510)).zName })
                                        };
                                        unsafe {
                                            sqlite3NestedParse(pParse, (b"UPDATE temp.sqlite_master SET sql = sqlite_rename_column(sql, type, name, %Q, %Q, %d, %Q, %d, 1) WHERE type IN ('trigger', 'view')\0".as_ptr() as *mut i8) as *const i8, *__slate_slot_514, unsafe { (*(*__slate_slot_510)).zName }, *__slate_slot_511, *__slate_slot_513, *__slate_slot_516)
                                        };
                                        // /* Drop and reload the database schema. */
                                        renameReloadSchema(
                                            pParse,
                                            *__slate_slot_515,
                                            ((1 as i32) as i16) as u16,
                                        );
                                        renameTestSchema(
                                            pParse,
                                            *__slate_slot_514,
                                            (*__slate_slot_515 == (1 as i32)) as i32,
                                            (b"after rename\0".as_ptr() as *mut i8) as *const i8,
                                            1 as i32,
                                        );
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        unsafe { sqlite3SrcListDelete(*__slate_slot_509, pSrc) };
        unsafe { sqlite3DbFree(*__slate_slot_509, *__slate_slot_512 as *mut ()) };
        unsafe { sqlite3DbFree(*__slate_slot_509, *__slate_slot_513 as *mut ()) };
        return;
    }
}

// /* Parsing context */
// /* Name of the table to look for */
// /* OUT: write the iDb here */
// /* OUT: write name of schema here */
// /* Do ALTER TABLE authorization checks if true */
// /*
// ** Generate bytecode for one of:
// **
// **  (1)   ALTER TABLE pSrc DROP CONSTRAINT pCons
// **  (2)   ALTER TABLE pSrc ALTER pCol DROP NOT NULL
// **
// ** One of pCons and pCol must be NULL and the other non-null.
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3AlterDropConstraint(
    mut pParse: *mut Parse,
    mut pSrc: *mut SrcList,
    mut pCons: *mut Token,
    mut pCol: *mut Token,
) {
    let mut db: *mut sqlite3 = unsafe { (*pParse).db };
    let mut pTab: *mut Table = std::ptr::null_mut::<Table>();
    let mut iDb: i32 = 0 as i32;
    let mut zDb: *const i8 = std::ptr::null::<i8>();
    let mut zArg: *mut i8 = std::ptr::null_mut::<i8>();
    0 as i32;
    0 as i32;
    pTab = alterFindTable(
        pParse,
        pSrc,
        std::ptr::addr_of_mut!(iDb),
        std::ptr::addr_of_mut!(zDb),
        (pCons != std::ptr::null_mut::<Token>()) as i32,
    );
    if !(pTab != std::ptr::null_mut::<Table>()) {
        return;
    }
    if pCons != std::ptr::null_mut::<Token>() {
        let mut z: *mut i8 = unsafe { sqlite3NameFromToken(db, pCons as *const Token) };
        zArg = unsafe { sqlite3MPrintf(db, (b"%Q\0".as_ptr() as *mut i8) as *const i8, z) };
        unsafe { sqlite3DbFree(db, z as *mut ()) };
    } else {
        let mut iCol: i32 = 0 as i32;
        if alterFindCol(pParse, pTab, pCol, std::ptr::addr_of_mut!(iCol)) != (0 as i32) {
            return;
        }
        zArg = unsafe { sqlite3MPrintf(db, (b"%d\0".as_ptr() as *mut i8) as *const i8, iCol) };
    }
    // /* Edit the SQL for the named table. */
    unsafe {
        sqlite3NestedParse(pParse, (b"UPDATE \"%w\".sqlite_master SET sql = sqlite_drop_constraint(sql, %s) WHERE type='table' AND tbl_name=%Q COLLATE nocase\0".as_ptr() as *mut i8) as *const i8, zDb, zArg, unsafe { (*pTab).zName })
    };
    unsafe { sqlite3DbFree(db, zArg as *mut ()) };
    // /* Finally, reload the database schema. */
    renameReloadSchema(pParse, iDb, ((4 as i32) as i16) as u16);
}

// /*
// ** Generate bytecode to implement:
// **
// **    ALTER TABLE pSrc ADD [CONSTRAINT pName] CHECK(pExpr)
// **
// ** Any "ON CONFLICT" text that occurs after the "CHECK(...)", up
// ** until pParse->sLastToken, is included as part of the new constraint.
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3AlterAddConstraint(
    mut pParse: *mut Parse,
    mut pSrc: *mut SrcList,
    mut pFirst: *mut Token,
    mut pName: *mut Token,
    mut zExpr: *const i8,
    mut nExpr: i32,
    mut pExpr: *mut Expr,
) {
    // /* Table identified by pSrc */
    let mut pTab: *mut Table = std::ptr::null_mut::<Table>();
    // /* Which schema does pTab live in */
    let mut iDb: i32 = 0 as i32;
    // /* Name of the schema in which pTab lives */
    let mut zDb: *const i8 = std::ptr::null::<i8>();
    // /* Text of the constraint */
    let mut pCons: *const i8 = std::ptr::null::<i8>();
    // /* Bytes of text to use from pCons[] */
    let mut nCons: i32 = 0 as i32;
    // /* Result from error checking pExpr */
    let mut rc: i32 = 0 as i32;
    // /* Look up the table being altered. */
    0 as i32;
    pTab = alterFindTable(
        pParse,
        pSrc,
        std::ptr::addr_of_mut!(iDb),
        std::ptr::addr_of_mut!(zDb),
        1 as i32,
    );
    if !(pTab != std::ptr::null_mut::<Table>()) {
        unsafe { sqlite3ExprDelete(unsafe { (*pParse).db }, pExpr) };
        return;
    }
    // /* Verify that the new CHECK constraint does not contain any
    //   ** internal-use-only function.  Forum post 2026-05-10T01:11:28Z
    //   */
    rc = unsafe {
        sqlite3ResolveSelfReference(
            pParse,
            pTab,
            4 as i32,
            pExpr,
            std::ptr::null_mut::<ExprList>(),
        )
    };
    unsafe { sqlite3ExprDelete(unsafe { (*pParse).db }, pExpr) };
    if rc != (0 as i32) {
        return;
    }
    // /* If this new constraint has a name, check that it is not a duplicate of
    //   ** an existing constraint. It is an error if it is.  */
    if pName != std::ptr::null_mut::<Token>() {
        let mut zName: *mut i8 =
            unsafe { sqlite3NameFromToken(unsafe { (*pParse).db }, pName as *const Token) };
        unsafe {
            sqlite3NestedParse(pParse, (b"SELECT sqlite_fail('constraint %q already exists', %d) FROM \"%w\".sqlite_master WHERE type='table' AND tbl_name=%Q COLLATE nocase AND sqlite_find_constraint(sql, %Q)\0".as_ptr() as *mut i8) as *const i8, zName, 1 as i32, zDb, unsafe { (*pTab).zName }, zName)
        };
        unsafe { sqlite3DbFree(unsafe { (*pParse).db }, zName as *mut ()) };
    }
    // /* Search for a constraint violation. Throw an exception if one is found. */
    unsafe {
        sqlite3NestedParse(
            pParse,
            (b"SELECT sqlite_fail('constraint failed', %d) FROM %Q.%Q WHERE (%.*s) IS NOT TRUE\0"
                .as_ptr() as *mut i8) as *const i8,
            19 as i32,
            zDb,
            unsafe { (*pTab).zName },
            nExpr,
            zExpr,
        )
    };
    // /* Edit the SQL for the named table. */
    pCons = unsafe { (*pFirst).z };
    nCons = alterRtrimConstraint(
        unsafe { (*pParse).db },
        pCons,
        ((unsafe { unsafe { (*pParse).sLastToken.z }.offset_from(pCons as *const i8) }) as i64)
            as i32,
    );
    unsafe {
        sqlite3NestedParse(pParse, (b"UPDATE \"%w\".sqlite_master SET sql = sqlite_add_constraint(sql, %.*Q, -1) WHERE type='table' AND tbl_name=%Q COLLATE nocase\0".as_ptr() as *mut i8) as *const i8, zDb, nCons, pCons, unsafe { (*pTab).zName })
    };
    // /* Finally, reload the database schema. */
    renameReloadSchema(pParse, iDb, ((4 as i32) as i16) as u16);
}

// /* used to record OOM error */
// /* Buffer containing constraint */
// /* Size of pCons in bytes */
// /*
// ** Prepare a statement of the form:
// **
// **   ALTER TABLE pSrc ALTER pCol SET NOT NULL
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3AlterSetNotNull(
    mut pParse: *mut Parse,
    mut pSrc: *mut SrcList,
    mut pCol: *mut Token,
    mut pFirst: *mut Token,
) {
    let mut pTab: *mut Table = std::ptr::null_mut::<Table>();
    let mut iCol: i32 = 0 as i32;
    let mut iDb: i32 = 0 as i32;
    let mut zDb: *const i8 = std::ptr::null::<i8>();
    let mut pCons: *const i8 = std::ptr::null::<i8>();
    let mut nCons: i32 = 0 as i32;
    // /* Look up the table being altered. */
    0 as i32;
    pTab = alterFindTable(
        pParse,
        pSrc,
        std::ptr::addr_of_mut!(iDb),
        std::ptr::addr_of_mut!(zDb),
        0 as i32,
    );
    if !(pTab != std::ptr::null_mut::<Table>()) {
        return;
    }
    // /* Find the column being altered. */
    if alterFindCol(pParse, pTab, pCol, std::ptr::addr_of_mut!(iCol)) != (0 as i32) {
        return;
    }
    // /* Find the length in bytes of the constraint definition */
    pCons = unsafe { (*pFirst).z };
    nCons = alterRtrimConstraint(
        unsafe { (*pParse).db },
        pCons,
        ((unsafe { unsafe { (*pParse).sLastToken.z }.offset_from(pCons as *const i8) }) as i64)
            as i32,
    );
    // /* Search for a constraint violation. Throw an exception if one is found. */
    unsafe {
        sqlite3NestedParse(
            pParse,
            (b"SELECT sqlite_fail('constraint failed', %d) FROM %Q.%Q AS x WHERE x.%.*s IS NULL\0"
                .as_ptr() as *mut i8) as *const i8,
            19 as i32,
            zDb,
            unsafe { (*pTab).zName },
            (unsafe { (*pCol).n }) as i32,
            unsafe { (*pCol).z },
        )
    };
    // /* Edit the SQL for the named table. */
    unsafe {
        sqlite3NestedParse(pParse, (b"UPDATE \"%w\".sqlite_master SET sql = sqlite_add_constraint(sqlite_drop_constraint(sql, %d), %.*Q, %d) WHERE type='table' AND tbl_name=%Q COLLATE nocase\0".as_ptr() as *mut i8) as *const i8, zDb, iCol, nCons, pCons, iCol, unsafe { (*pTab).zName })
    };
    // /* Finally, reload the database schema. */
    renameReloadSchema(pParse, iDb, ((4 as i32) as i16) as u16);
}

// /* Parsing context */
// /* Schema holding the table */
// /* Table to check for empty */
// /* Error message text */
// /*
// ** This function is called after an "ALTER TABLE ... ADD" statement
// ** has been parsed. Argument pColDef contains the text of the new
// ** column definition.
// **
// ** The Table structure pParse->pNewTable was extended to include
// ** the new column during parsing.
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3AlterFinishAddColumn(mut pParse: *mut Parse, mut pColDef: *mut Token) {
    // /* Copy of pParse->pNewTable */
    let mut pNew: *mut Table = unsafe { std::mem::zeroed() };
    // /* Table being altered */
    let mut pTab: *mut Table = unsafe { std::mem::zeroed() };
    // /* Database number */
    let mut iDb: i32 = 0 as i32;
    // /* Database name */
    let mut zDb: *const i8 = unsafe { std::mem::zeroed() };
    // /* Table name */
    let mut zTab: *const i8 = unsafe { std::mem::zeroed() };
    // /* Null-terminated column definition */
    let mut zCol: *mut i8 = unsafe { std::mem::zeroed() };
    // /* The new column */
    let mut pCol: *mut Column = unsafe { std::mem::zeroed() };
    // /* Default value for the new column */
    let mut pDflt: *mut Expr = unsafe { std::mem::zeroed() };
    // /* The database connection; */
    let mut db: *mut sqlite3 = unsafe { std::mem::zeroed() };
    // /* The prepared statement under construction */
    let mut v: *mut Vdbe = unsafe { std::mem::zeroed() };
    // /* Temporary registers */
    let mut r1: i32 = 0 as i32;
    db = unsafe { (*pParse).db };
    0 as i32;
    if (unsafe { (*pParse).nErr }) != (0 as i32) {
        return;
    }
    0 as i32;
    pNew = unsafe { (*pParse).pNewTable };
    0 as i32;
    0 as i32;
    iDb = unsafe { sqlite3SchemaToIndex(db, unsafe { (*pNew).pSchema }) };
    zDb =
        (unsafe { (*unsafe { unsafe { (*db).aDb }.offset(iDb as isize) }).zDbSName }) as *const i8;
    // /* Skip the "sqlite_altertab_" prefix on the name */
    zTab = (unsafe { unsafe { (*pNew).zName }.offset((16 as i32) as isize) }) as *const i8;
    pCol = unsafe {
        unsafe { (*pNew).aCol }.offset((((unsafe { (*pNew).nCol }) as i32) - (1 as i32)) as isize)
    };
    pDflt = unsafe { sqlite3ColumnExpr(pNew, pCol) };
    pTab = unsafe { sqlite3FindTable(db, zTab, zDb) };
    0 as i32;
    // /* Invoke the authorization callback. */
    if (unsafe {
        sqlite3AuthCheck(
            pParse,
            26 as i32,
            zDb,
            (unsafe { (*pTab).zName }) as *const i8,
            std::ptr::null::<i8>(),
        )
    }) != (0 as i32)
    {
        return;
    }
    // /* Check that the new column is not specified as PRIMARY KEY or UNIQUE.
    //   ** If there is a NOT NULL constraint, then the default value for the
    //   ** column must not be NULL.
    //   */
    if (((unsafe { (*pCol).colFlags }) as u32) as i32) & (1 as i32) != (0 as i32) {
        unsafe {
            sqlite3ErrorMsg(
                pParse,
                (b"Cannot add a PRIMARY KEY column\0".as_ptr() as *mut i8) as *const i8,
            )
        };
        return;
    }
    if (unsafe { (*pNew).pIndex }) != std::ptr::null_mut::<Index>() {
        unsafe {
            sqlite3ErrorMsg(
                pParse,
                (b"Cannot add a UNIQUE column\0".as_ptr() as *mut i8) as *const i8,
            )
        };
        return;
    }
    if (((unsafe { (*pCol).colFlags }) as u32) as i32) & (96 as i32) == (0 as i32) {
        // /* If the default value for the new column was specified with a
        //     ** literal NULL, then set pDflt to 0. This simplifies checking
        //     ** for an SQL NULL default below.
        //     */
        0 as i32;
        if pDflt != std::ptr::null_mut::<Expr>()
            && (((unsafe { (*unsafe { (*pDflt).pLeft }).op }) as u32) as i32) == (122 as i32)
        {
            pDflt = std::ptr::null_mut::<Expr>();
        }
        0 as i32;
        if (unsafe { (*db).flags }) & (((16384 as i32) as i64) as u64) != (0 as u64)
            && (unsafe { (*pNew).u.tab.pFKey }) != std::ptr::null_mut::<FKey>()
            && pDflt != std::ptr::null_mut::<Expr>()
        {
            sqlite3ErrorIfNotEmpty(
                pParse,
                zDb,
                zTab,
                (b"Cannot add a REFERENCES column with non-NULL default value\0".as_ptr()
                    as *mut i8) as *const i8,
            );
        }
        if ((unsafe { (*pCol).__slate_bits_0.__get_notNull() }) as i32) != (0 as i32)
            && !(pDflt != std::ptr::null_mut::<Expr>())
        {
            sqlite3ErrorIfNotEmpty(
                pParse,
                zDb,
                zTab,
                (b"Cannot add a NOT NULL column with default value NULL\0".as_ptr() as *mut i8)
                    as *const i8,
            );
        }
        // /* Ensure the default expression is something that sqlite3ValueFromExpr()
        //     ** can handle (i.e. not CURRENT_TIME etc.)
        //     */
        if pDflt != std::ptr::null_mut::<Expr>() {
            let mut pVal: *mut sqlite3_value = std::ptr::null_mut::<sqlite3_value>();
            let mut rc: i32 = 0 as i32;
            rc = unsafe {
                sqlite3ValueFromExpr(
                    db,
                    pDflt as *const Expr,
                    ((1 as i32) as i8) as u8,
                    ((65 as i32) as i8) as u8,
                    std::ptr::addr_of_mut!(pVal),
                )
            };
            0 as i32;
            if rc != (0 as i32) {
                0 as i32;
                return;
            }
            if !(pVal != std::ptr::null_mut::<sqlite3_value>()) {
                sqlite3ErrorIfNotEmpty(
                    pParse,
                    zDb,
                    zTab,
                    (b"Cannot add a column with non-constant default\0".as_ptr() as *mut i8)
                        as *const i8,
                );
            }
            unsafe { sqlite3ValueFree(pVal) };
        }
    } else {
        if (((unsafe { (*pCol).colFlags }) as u32) as i32) & (64 as i32) != (0 as i32) {
            sqlite3ErrorIfNotEmpty(
                pParse,
                zDb,
                zTab,
                (b"cannot add a STORED column\0".as_ptr() as *mut i8) as *const i8,
            );
        }
    }
    // /* Modify the CREATE TABLE statement. */
    zCol = unsafe {
        sqlite3DbStrNDup(
            db,
            ((unsafe { (*pColDef).z }) as *mut i8) as *const i8,
            (unsafe { (*pColDef).n }) as u64,
        )
    };
    if zCol != std::ptr::null_mut::<i8>() {
        let mut zEnd: *mut i8 = unsafe {
            zCol.offset(unsafe { (*pColDef).n }.wrapping_sub((1 as i32) as u32) as isize)
        };
        '__slate_break_1246: while zEnd > zCol
            && (((unsafe { *zEnd }) as i32) == (59 as i32)
                || (((unsafe {
                    *unsafe {
                        unsafe { std::ptr::addr_of!(sqlite3CtypeMap) as *const u8 }
                            .offset(((((unsafe { *zEnd }) as u8) as u32) as i32) as isize)
                    }
                }) as u32) as i32)
                    & (1 as i32)
                    != (0 as i32))
        {
            let __v1359: *mut i8 = zEnd;
            let __v1360: *mut i8 = unsafe { __v1359.offset(-((1 as i32) as isize)) };
            zEnd = __v1360;
            unsafe {
                *__v1359 = (0 as i32) as i8;
            }
        }
        // /* substr() operations on characters, but addColOffset is in bytes. So we
        //     ** have to use printf() to translate between these units: */
        0 as i32;
        0 as i32;
        unsafe {
            sqlite3NestedParse(pParse, (b"UPDATE \"%w\".sqlite_master SET sql = printf('%%.%ds, ',sql) || %Q || substr(sql,1+length(printf('%%.%ds',sql))) WHERE type = 'table' AND name = %Q\0".as_ptr() as *mut i8) as *const i8, zDb, unsafe { (*pNew).u.tab.addColOffset }, zCol, unsafe { (*pNew).u.tab.addColOffset }, zTab)
        };
        unsafe { sqlite3DbFree(db, zCol as *mut ()) };
    }
    v = unsafe { sqlite3GetVdbe(pParse) };
    if v != std::ptr::null_mut::<Vdbe>() {
        // /* Make sure the schema version is at least 3.  But do not upgrade
        //     ** from less than 3 to 4, as that will corrupt any preexisting DESC
        //     ** index.
        //     */
        r1 = unsafe { sqlite3GetTempReg(pParse) };
        unsafe { sqlite3VdbeAddOp3(v, 101 as i32, iDb, r1, 2 as i32) };
        unsafe { sqlite3VdbeUsesBtree(v, iDb) };
        unsafe { sqlite3VdbeAddOp2(v, 88 as i32, r1, -(2 as i32)) };
        unsafe {
            sqlite3VdbeAddOp2(
                v,
                61 as i32,
                r1,
                (unsafe { sqlite3VdbeCurrentAddr(v) }) + (2 as i32),
            )
        };
        {}
        unsafe { sqlite3VdbeAddOp3(v, 102 as i32, iDb, 2 as i32, 3 as i32) };
        unsafe { sqlite3ReleaseTempReg(pParse, r1) };
        // /* Reload the table definition */
        renameReloadSchema(pParse, iDb, ((3 as i32) as i16) as u16);
        // /* Verify that constraints are still satisfied */
        if (unsafe { (*pNew).pCheck }) != std::ptr::null_mut::<ExprList>()
            || ((unsafe { (*pCol).__slate_bits_0.__get_notNull() }) as i32) != (0 as i32)
                && (((unsafe { (*pCol).colFlags }) as u32) as i32) & (96 as i32) != (0 as i32)
            || (unsafe { (*pTab).tabFlags }) & ((65536 as i32) as u32) != ((0 as i32) as u32)
        {
            unsafe {
                sqlite3NestedParse(pParse, (b"SELECT CASE WHEN quick_check GLOB 'CHECK*' THEN raise(ABORT,'CHECK constraint failed') WHEN quick_check GLOB 'non-* value in*' THEN raise(ABORT,'type mismatch on DEFAULT') ELSE raise(ABORT,'NOT NULL constraint failed') END  FROM pragma_quick_check(%Q,%Q) WHERE quick_check GLOB 'CHECK*' OR quick_check GLOB 'NULL*' OR quick_check GLOB 'non-* value in*'\0".as_ptr() as *mut i8) as *const i8, zTab, zDb)
            };
        }
    }
}

// /*
// ** This function is called by the parser after the table-name in
// ** an "ALTER TABLE <table-name> ADD" statement is parsed. Argument
// ** pSrc is the full-name of the table being altered.
// **
// ** This routine makes a (partial) copy of the Table structure
// ** for the table being altered and sets Parse.pNewTable to point
// ** to it. Routines called by the parser as the column definition
// ** is parsed (i.e. sqlite3AddColumn()) add the new Column data to
// ** the copy. The copy of the Table structure is deleted by tokenize.c
// ** after parsing is finished.
// **
// ** Routine sqlite3AlterFinishAddColumn() will be called to complete
// ** coding the "ALTER TABLE ... ADD" statement.
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3AlterBeginAddColumn(mut pParse: *mut Parse, mut pSrc: *mut SrcList) {
    let mut __slate_storage_1362: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1362: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1362) as *mut i32;
    let mut __slate_storage_1361: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1361: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1361) as *mut i32;
    let mut __slate_storage_497: std::mem::MaybeUninit<*mut Column> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_497: *mut *mut Column =
        std::ptr::addr_of_mut!(__slate_storage_497) as *mut *mut Column;
    let mut __slate_storage_496: std::mem::MaybeUninit<*mut sqlite3> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_496: *mut *mut sqlite3 =
        std::ptr::addr_of_mut!(__slate_storage_496) as *mut *mut sqlite3;
    let mut __slate_storage_495: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_495: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_495) as *mut i32;
    let mut __slate_storage_494: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_494: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_494) as *mut i32;
    let mut __slate_storage_493: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_493: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_493) as *mut i32;
    let mut __slate_storage_492: std::mem::MaybeUninit<*mut Table> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_492: *mut *mut Table =
        std::ptr::addr_of_mut!(__slate_storage_492) as *mut *mut Table;
    let mut __slate_storage_491: std::mem::MaybeUninit<*mut Table> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_491: *mut *mut Table =
        std::ptr::addr_of_mut!(__slate_storage_491) as *mut *mut Table;
    unsafe {
        '__join_0: {
            std::ptr::write(__slate_slot_496, unsafe { (*pParse).db });
            // /* Look up the table being altered. */
            0 as i32;
            0 as i32;
            if (unsafe { (*(*__slate_slot_496)).mallocFailed }) != (0 as u8) {
            } else {
                *__slate_slot_492 = unsafe {
                    sqlite3LocateTableItem(pParse, (0 as i32) as u32, unsafe {
                        unsafe { std::ptr::addr_of_mut!((*pSrc).a) as *mut SrcItem }
                            .offset((0 as i32) as isize)
                    })
                };
                if !(*__slate_slot_492 != std::ptr::null_mut::<Table>()) {
                } else {
                    if (((unsafe { (*(*__slate_slot_492)).eTabType }) as u32) as i32) == (1 as i32)
                    {
                        unsafe {
                            sqlite3ErrorMsg(
                                pParse,
                                (b"virtual tables may not be altered\0".as_ptr() as *mut i8)
                                    as *const i8,
                            )
                        };
                    } else {
                        // /* Make sure this is not an attempt to ALTER a view. */
                        if (((unsafe { (*(*__slate_slot_492)).eTabType }) as u32) as i32)
                            == (2 as i32)
                        {
                            unsafe {
                                sqlite3ErrorMsg(
                                    pParse,
                                    (b"Cannot add a column to a view\0".as_ptr() as *mut i8)
                                        as *const i8,
                                )
                            };
                        } else {
                            if (0 as i32) != isAlterableTable(pParse, *__slate_slot_492) {
                            } else {
                                unsafe { sqlite3MayAbort(pParse) };
                                0 as i32;
                                0 as i32;
                                *__slate_slot_493 = unsafe {
                                    sqlite3SchemaToIndex(*__slate_slot_496, unsafe {
                                        (*(*__slate_slot_492)).pSchema
                                    })
                                };
                                // /* Put a copy of the Table struct in Parse.pNewTable for the
                                //   ** sqlite3AddColumn() function and friends to modify.  But modify
                                //   ** the name by adding an "sqlite_altertab_" prefix.  By adding this
                                //   ** prefix, we insure that the name will not collide with an existing
                                //   ** table because user table are not allowed to have the "sqlite_"
                                //   ** prefix on their name.
                                //   */
                                *__slate_slot_491 =
                                    (unsafe { sqlite3DbMallocZero(*__slate_slot_496, 120 as u64) })
                                        as *mut Table;
                                if !(*__slate_slot_491 != std::ptr::null_mut::<Table>()) {
                                } else {
                                    unsafe {
                                        (*pParse).pNewTable = *__slate_slot_491;
                                    }
                                    unsafe {
                                        (*(*__slate_slot_491)).nTabRef = (1 as i32) as u32;
                                    }
                                    unsafe {
                                        (*(*__slate_slot_491)).nCol =
                                            unsafe { (*(*__slate_slot_492)).nCol };
                                    }
                                    0 as i32;
                                    *__slate_slot_495 = (((unsafe { (*(*__slate_slot_491)).nCol })
                                        as i32)
                                        - (1 as i32))
                                        / (8 as i32)
                                        * (8 as i32)
                                        + (8 as i32);
                                    0 as i32;
                                    unsafe {
                                        (*(*__slate_slot_491)).aCol = (unsafe {
                                            sqlite3DbMallocZero(
                                                *__slate_slot_496,
                                                (16 as u64).wrapping_mul(
                                                    (*__slate_slot_495 as u32) as u64,
                                                ),
                                            )
                                        })
                                            as *mut Column;
                                    }
                                    unsafe {
                                        (*(*__slate_slot_491)).zName = unsafe {
                                            sqlite3MPrintf(
                                                *__slate_slot_496,
                                                (b"sqlite_altertab_%s\0".as_ptr() as *mut i8)
                                                    as *const i8,
                                                unsafe { (*(*__slate_slot_492)).zName },
                                            )
                                        };
                                    }
                                    if !((unsafe { (*(*__slate_slot_491)).aCol })
                                        != std::ptr::null_mut::<Column>())
                                        || !((unsafe { (*(*__slate_slot_491)).zName })
                                            != std::ptr::null_mut::<i8>())
                                    {
                                        0 as i32;
                                    } else {
                                        unsafe {
                                            memcpy(
                                                (unsafe { (*(*__slate_slot_491)).aCol }) as *mut (),
                                                (unsafe { (*(*__slate_slot_492)).aCol })
                                                    as *const (),
                                                (16 as u64).wrapping_mul(
                                                    ((unsafe { (*(*__slate_slot_491)).nCol })
                                                        as i64)
                                                        as u64,
                                                ),
                                            )
                                        };
                                        *__slate_slot_494 = 0 as i32;
                                        loop {
                                            if *__slate_slot_494
                                                < ((unsafe { (*(*__slate_slot_491)).nCol }) as i32)
                                            {
                                                std::ptr::write(__slate_slot_497, unsafe {
                                                    unsafe { (*(*__slate_slot_491)).aCol }
                                                        .offset(*__slate_slot_494 as isize)
                                                });
                                                unsafe {
                                                    (*(*__slate_slot_497)).zCnName = unsafe {
                                                        sqlite3DbStrDup(
                                                            *__slate_slot_496,
                                                            (unsafe {
                                                                (*(*__slate_slot_497)).zCnName
                                                            })
                                                                as *const i8,
                                                        )
                                                    };
                                                }
                                                unsafe {
                                                    (*(*__slate_slot_497)).hName = unsafe {
                                                        sqlite3StrIHash(
                                                            (unsafe {
                                                                (*(*__slate_slot_497)).zCnName
                                                            })
                                                                as *const i8,
                                                        )
                                                    };
                                                }
                                                std::ptr::write(
                                                    __slate_slot_1361,
                                                    *__slate_slot_494,
                                                );
                                                std::ptr::write(
                                                    __slate_slot_1362,
                                                    *__slate_slot_1361 + (1 as i32),
                                                );
                                                *__slate_slot_494 = *__slate_slot_1362;
                                            } else {
                                                break;
                                            }
                                        }
                                        0 as i32;
                                        unsafe {
                                            (*(*__slate_slot_491)).u.tab.pDfltList = unsafe {
                                                sqlite3ExprListDup(
                                                    *__slate_slot_496,
                                                    (unsafe {
                                                        (*(*__slate_slot_492)).u.tab.pDfltList
                                                    })
                                                        as *const ExprList,
                                                    0 as i32,
                                                )
                                            };
                                        }
                                        unsafe {
                                            (*(*__slate_slot_491)).pSchema = unsafe {
                                                (*unsafe {
                                                    unsafe { (*(*__slate_slot_496)).aDb }
                                                        .offset(*__slate_slot_493 as isize)
                                                })
                                                .pSchema
                                            };
                                        }
                                        unsafe {
                                            (*(*__slate_slot_491)).u.tab.addColOffset = unsafe {
                                                (*(*__slate_slot_492)).u.tab.addColOffset
                                            };
                                        }
                                        0 as i32;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        unsafe { sqlite3SrcListDelete(*__slate_slot_496, pSrc) };
        return;
    }
}

// /*
// ** This function is called by the parser upon parsing an
// **
// **     ALTER TABLE pSrc DROP COLUMN pName
// **
// ** statement. Argument pSrc contains the possibly qualified name of the
// ** table being edited, and token pName the name of the column to drop.
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3AlterDropColumn(
    mut pParse: *mut Parse,
    mut pSrc: *mut SrcList,
    mut pName: *const Token,
) {
    let mut __slate_storage_1386: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1386: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1386) as *mut i32;
    let mut __slate_storage_1385: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1385: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1385) as *mut i32;
    let mut __slate_storage_1384: std::mem::MaybeUninit<*mut Parse> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1384: *mut *mut Parse =
        std::ptr::addr_of_mut!(__slate_storage_1384) as *mut *mut Parse;
    let mut __slate_storage_1381: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1381: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1381) as *mut i32;
    let mut __slate_storage_1380: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1380: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1380) as *mut i32;
    let mut __slate_storage_1383: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1383: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1383) as *mut i32;
    let mut __slate_storage_1382: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1382: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1382) as *mut i32;
    let mut __slate_storage_813: std::mem::MaybeUninit<i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_813: *mut i8 = std::ptr::addr_of_mut!(__slate_storage_813) as *mut i8;
    let mut __slate_storage_812: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_812: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_812) as *mut i32;
    let mut __slate_storage_811: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_811: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_811) as *mut i32;
    let mut __slate_storage_810: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_810: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_810) as *mut i32;
    let mut __slate_storage_1379: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1379: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1379) as *mut i32;
    let mut __slate_storage_1378: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1378: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1378) as *mut i32;
    let mut __slate_storage_1377: std::mem::MaybeUninit<*mut Parse> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1377: *mut *mut Parse =
        std::ptr::addr_of_mut!(__slate_storage_1377) as *mut *mut Parse;
    let mut __slate_storage_1371: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1371: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1371) as *mut i32;
    let mut __slate_storage_1370: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1370: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1370) as *mut i32;
    let mut __slate_storage_1369: std::mem::MaybeUninit<*mut Parse> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1369: *mut *mut Parse =
        std::ptr::addr_of_mut!(__slate_storage_1369) as *mut *mut Parse;
    let mut __slate_storage_1376: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1376: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1376) as *mut i32;
    let mut __slate_storage_1375: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1375: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1375) as *mut i32;
    let mut __slate_storage_1374: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1374: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1374) as *mut i32;
    let mut __slate_storage_1373: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1373: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1373) as *mut i32;
    let mut __slate_storage_1372: std::mem::MaybeUninit<*mut Parse> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1372: *mut *mut Parse =
        std::ptr::addr_of_mut!(__slate_storage_1372) as *mut *mut Parse;
    let mut __slate_storage_1368: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1368: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1368) as *mut i32;
    let mut __slate_storage_1367: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1367: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1367) as *mut i32;
    let mut __slate_storage_1366: std::mem::MaybeUninit<*mut Parse> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1366: *mut *mut Parse =
        std::ptr::addr_of_mut!(__slate_storage_1366) as *mut *mut Parse;
    let mut __slate_storage_1365: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1365: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1365) as *mut i32;
    let mut __slate_storage_1364: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1364: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1364) as *mut i32;
    let mut __slate_storage_1363: std::mem::MaybeUninit<*mut Parse> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1363: *mut *mut Parse =
        std::ptr::addr_of_mut!(__slate_storage_1363) as *mut *mut Parse;
    let mut __slate_storage_809: std::mem::MaybeUninit<*mut Vdbe> = std::mem::MaybeUninit::uninit();
    let __slate_slot_809: *mut *mut Vdbe =
        std::ptr::addr_of_mut!(__slate_storage_809) as *mut *mut Vdbe;
    let mut __slate_storage_808: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_808: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_808) as *mut i32;
    let mut __slate_storage_807: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_807: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_807) as *mut i32;
    let mut __slate_storage_806: std::mem::MaybeUninit<*mut Index> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_806: *mut *mut Index =
        std::ptr::addr_of_mut!(__slate_storage_806) as *mut *mut Index;
    let mut __slate_storage_805: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_805: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_805) as *mut i32;
    let mut __slate_storage_804: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_804: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_804) as *mut i32;
    let mut __slate_storage_803: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_803: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_803) as *mut i32;
    let mut __slate_storage_802: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_802: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_802) as *mut i32;
    let mut __slate_storage_801: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_801: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_801) as *mut i32;
    let mut __slate_storage_800: std::mem::MaybeUninit<*mut i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_800: *mut *mut i8 =
        std::ptr::addr_of_mut!(__slate_storage_800) as *mut *mut i8;
    let mut __slate_storage_799: std::mem::MaybeUninit<*const i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_799: *mut *const i8 =
        std::ptr::addr_of_mut!(__slate_storage_799) as *mut *const i8;
    let mut __slate_storage_798: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_798: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_798) as *mut i32;
    let mut __slate_storage_797: std::mem::MaybeUninit<*mut Table> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_797: *mut *mut Table =
        std::ptr::addr_of_mut!(__slate_storage_797) as *mut *mut Table;
    let mut __slate_storage_796: std::mem::MaybeUninit<*mut sqlite3> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_796: *mut *mut sqlite3 =
        std::ptr::addr_of_mut!(__slate_storage_796) as *mut *mut sqlite3;
    unsafe {
        '__join_0: {
            // /* Database handle */
            std::ptr::write(__slate_slot_796, unsafe { (*pParse).db });
            // /* Table to modify */
            // /* Index of db containing pTab in aDb[] */
            // /* Database containing pTab ("main" etc.) */
            // /* Name of column to drop */
            std::ptr::write(__slate_slot_800, std::ptr::null_mut::<i8>());
            // /* Index of column zCol in pTab->aCol[] */
            // /* Look up the table being altered. */
            0 as i32;
            0 as i32;
            if (unsafe { (*(*__slate_slot_796)).mallocFailed }) != (0 as u8) {
            } else {
                *__slate_slot_797 = unsafe {
                    sqlite3LocateTableItem(pParse, (0 as i32) as u32, unsafe {
                        unsafe { std::ptr::addr_of_mut!((*pSrc).a) as *mut SrcItem }
                            .offset((0 as i32) as isize)
                    })
                };
                if !(*__slate_slot_797 != std::ptr::null_mut::<Table>()) {
                } else {
                    // /* Make sure this is not an attempt to ALTER a view, virtual table or
                    //   ** system table. */
                    if (0 as i32) != isAlterableTable(pParse, *__slate_slot_797) {
                    } else {
                        if (0 as i32) != isRealTable(pParse, *__slate_slot_797, 1 as i32) {
                        } else {
                            // /* Find the index of the column being dropped. */
                            *__slate_slot_800 =
                                unsafe { sqlite3NameFromToken(*__slate_slot_796, pName) };
                            if *__slate_slot_800 == std::ptr::null_mut::<i8>() {
                                0 as i32;
                            } else {
                                *__slate_slot_801 = unsafe {
                                    sqlite3ColumnIndex(
                                        *__slate_slot_797,
                                        *__slate_slot_800 as *const i8,
                                    )
                                };
                                if *__slate_slot_801 < (0 as i32) {
                                    unsafe {
                                        sqlite3ErrorMsg(
                                            pParse,
                                            (b"no such column: \"%T\"\0".as_ptr() as *mut i8)
                                                as *const i8,
                                            pName,
                                        )
                                    };
                                } else {
                                    // /* Do not allow the user to drop a PRIMARY KEY column or a column
                                    //   ** constrained by a UNIQUE constraint.  */
                                    if (((unsafe {
                                        (*unsafe {
                                            unsafe { (*(*__slate_slot_797)).aCol }
                                                .offset(*__slate_slot_801 as isize)
                                        })
                                        .colFlags
                                    }) as u32) as i32)
                                        & ((1 as i32) | (8 as i32))
                                        != (0 as i32)
                                    {
                                        unsafe {
                                            sqlite3ErrorMsg(
                                                pParse,
                                                (b"cannot drop %s column: \"%s\"\0".as_ptr()
                                                    as *mut i8)
                                                    as *const i8,
                                                if (((unsafe {
                                                    (*unsafe {
                                                        unsafe { (*(*__slate_slot_797)).aCol }
                                                            .offset(*__slate_slot_801 as isize)
                                                    })
                                                    .colFlags
                                                })
                                                    as u32)
                                                    as i32)
                                                    & (1 as i32)
                                                    != (0 as i32)
                                                {
                                                    b"PRIMARY KEY\0".as_ptr() as *mut i8
                                                } else {
                                                    b"UNIQUE\0".as_ptr() as *mut i8
                                                },
                                                *__slate_slot_800,
                                            )
                                        };
                                    } else {
                                        // /* Do not allow the number of columns to go to zero */
                                        if ((unsafe { (*(*__slate_slot_797)).nCol }) as i32)
                                            <= (1 as i32)
                                        {
                                            unsafe {
                                                sqlite3ErrorMsg(pParse, (b"cannot drop column \"%s\": no other columns exist\0".as_ptr() as *mut i8) as *const i8, *__slate_slot_800)
                                            };
                                        } else {
                                            // /* Edit the sqlite_schema table */
                                            *__slate_slot_798 = unsafe {
                                                sqlite3SchemaToIndex(*__slate_slot_796, unsafe {
                                                    (*(*__slate_slot_797)).pSchema
                                                })
                                            };
                                            0 as i32;
                                            *__slate_slot_799 = (unsafe {
                                                (*unsafe {
                                                    unsafe { (*(*__slate_slot_796)).aDb }
                                                        .offset(*__slate_slot_798 as isize)
                                                })
                                                .zDbSName
                                            })
                                                as *const i8;
                                            // /* Invoke the authorization callback. */
                                            if (unsafe {
                                                sqlite3AuthCheck(
                                                    pParse,
                                                    26 as i32,
                                                    *__slate_slot_799,
                                                    (unsafe { (*(*__slate_slot_797)).zName })
                                                        as *const i8,
                                                    *__slate_slot_800 as *const i8,
                                                )
                                            }) != (0 as i32)
                                            {
                                            } else {
                                                renameTestSchema(
                                                    pParse,
                                                    *__slate_slot_799,
                                                    (*__slate_slot_798 == (1 as i32)) as i32,
                                                    (b"\0".as_ptr() as *mut i8) as *const i8,
                                                    0 as i32,
                                                );
                                                renameFixQuotes(
                                                    pParse,
                                                    *__slate_slot_799,
                                                    (*__slate_slot_798 == (1 as i32)) as i32,
                                                );
                                                unsafe {
                                                    sqlite3NestedParse(pParse, (b"UPDATE \"%w\".sqlite_master SET sql = sqlite_drop_column(%d, sql, %d) WHERE (type=='table' AND tbl_name=%Q COLLATE nocase)\0".as_ptr() as *mut i8) as *const i8, *__slate_slot_799, *__slate_slot_798, *__slate_slot_801, unsafe { (*(*__slate_slot_797)).zName })
                                                };
                                                // /* Drop and reload the database schema. */
                                                renameReloadSchema(
                                                    pParse,
                                                    *__slate_slot_798,
                                                    ((2 as i32) as i16) as u16,
                                                );
                                                renameTestSchema(
                                                    pParse,
                                                    *__slate_slot_799,
                                                    (*__slate_slot_798 == (1 as i32)) as i32,
                                                    (b"after drop column\0".as_ptr() as *mut i8)
                                                        as *const i8,
                                                    1 as i32,
                                                );
                                                // /* Edit rows of table on disk */
                                                if (unsafe { (*pParse).nErr }) == (0 as i32)
                                                    && (((unsafe {
                                                        (*unsafe {
                                                            unsafe { (*(*__slate_slot_797)).aCol }
                                                                .offset(*__slate_slot_801 as isize)
                                                        })
                                                        .colFlags
                                                    })
                                                        as u32)
                                                        as i32)
                                                        & (32 as i32)
                                                        == (0 as i32)
                                                {
                                                    std::ptr::write(
                                                        __slate_slot_806,
                                                        std::ptr::null_mut::<Index>(),
                                                    );
                                                    // /* Number of non-virtual columns after drop */
                                                    std::ptr::write(__slate_slot_807, 0 as i32);
                                                    std::ptr::write(__slate_slot_809, unsafe {
                                                        sqlite3GetVdbe(pParse)
                                                    });
                                                    std::ptr::write(__slate_slot_1363, pParse);
                                                    std::ptr::write(__slate_slot_1364, unsafe {
                                                        (*(*__slate_slot_1363)).nTab
                                                    });
                                                    std::ptr::write(
                                                        __slate_slot_1365,
                                                        *__slate_slot_1364 + (1 as i32),
                                                    );
                                                    unsafe {
                                                        (*(*__slate_slot_1363)).nTab =
                                                            *__slate_slot_1365;
                                                    }
                                                    *__slate_slot_808 = *__slate_slot_1364;
                                                    unsafe {
                                                        sqlite3OpenTable(
                                                            pParse,
                                                            *__slate_slot_808,
                                                            *__slate_slot_798,
                                                            *__slate_slot_797,
                                                            116 as i32,
                                                        )
                                                    };
                                                    *__slate_slot_803 = unsafe {
                                                        sqlite3VdbeAddOp1(
                                                            *__slate_slot_809,
                                                            36 as i32,
                                                            *__slate_slot_808,
                                                        )
                                                    };
                                                    {}
                                                    std::ptr::write(__slate_slot_1366, pParse);
                                                    std::ptr::write(__slate_slot_1367, unsafe {
                                                        (*(*__slate_slot_1366)).nMem
                                                    });
                                                    std::ptr::write(
                                                        __slate_slot_1368,
                                                        *__slate_slot_1367 + (1 as i32),
                                                    );
                                                    unsafe {
                                                        (*(*__slate_slot_1366)).nMem =
                                                            *__slate_slot_1368;
                                                    }
                                                    *__slate_slot_804 = *__slate_slot_1368;
                                                    if (unsafe { (*(*__slate_slot_797)).tabFlags })
                                                        & ((128 as i32) as u32)
                                                        == ((0 as i32) as u32)
                                                    {
                                                        unsafe {
                                                            sqlite3VdbeAddOp2(
                                                                *__slate_slot_809,
                                                                137 as i32,
                                                                *__slate_slot_808,
                                                                *__slate_slot_804,
                                                            )
                                                        };
                                                        std::ptr::write(__slate_slot_1369, pParse);
                                                        std::ptr::write(
                                                            __slate_slot_1370,
                                                            unsafe { (*(*__slate_slot_1369)).nMem },
                                                        );
                                                        std::ptr::write(
                                                            __slate_slot_1371,
                                                            *__slate_slot_1370
                                                                + ((unsafe {
                                                                    (*(*__slate_slot_797)).nCol
                                                                })
                                                                    as i32),
                                                        );
                                                        unsafe {
                                                            (*(*__slate_slot_1369)).nMem =
                                                                *__slate_slot_1371;
                                                        }
                                                    } else {
                                                        *__slate_slot_806 = unsafe {
                                                            sqlite3PrimaryKeyIndex(
                                                                *__slate_slot_797,
                                                            )
                                                        };
                                                        std::ptr::write(__slate_slot_1372, pParse);
                                                        std::ptr::write(
                                                            __slate_slot_1373,
                                                            unsafe { (*(*__slate_slot_1372)).nMem },
                                                        );
                                                        std::ptr::write(
                                                            __slate_slot_1374,
                                                            *__slate_slot_1373
                                                                + (((unsafe {
                                                                    (*(*__slate_slot_806)).nColumn
                                                                })
                                                                    as u32)
                                                                    as i32),
                                                        );
                                                        unsafe {
                                                            (*(*__slate_slot_1372)).nMem =
                                                                *__slate_slot_1374;
                                                        }
                                                        *__slate_slot_802 = 0 as i32;
                                                        loop {
                                                            if *__slate_slot_802
                                                                < (((unsafe {
                                                                    (*(*__slate_slot_806)).nKeyCol
                                                                })
                                                                    as u32)
                                                                    as i32)
                                                            {
                                                                unsafe {
                                                                    sqlite3VdbeAddOp3(
                                                                        *__slate_slot_809,
                                                                        96 as i32,
                                                                        *__slate_slot_808,
                                                                        *__slate_slot_802,
                                                                        *__slate_slot_804
                                                                            + *__slate_slot_802
                                                                            + (1 as i32),
                                                                    )
                                                                };
                                                                std::ptr::write(
                                                                    __slate_slot_1375,
                                                                    *__slate_slot_802,
                                                                );
                                                                std::ptr::write(
                                                                    __slate_slot_1376,
                                                                    *__slate_slot_1375 + (1 as i32),
                                                                );
                                                                *__slate_slot_802 =
                                                                    *__slate_slot_1376;
                                                            } else {
                                                                break;
                                                            }
                                                        }
                                                        *__slate_slot_807 = ((unsafe {
                                                            (*(*__slate_slot_806)).nKeyCol
                                                        })
                                                            as u32)
                                                            as i32;
                                                    }
                                                    std::ptr::write(__slate_slot_1377, pParse);
                                                    std::ptr::write(__slate_slot_1378, unsafe {
                                                        (*(*__slate_slot_1377)).nMem
                                                    });
                                                    std::ptr::write(
                                                        __slate_slot_1379,
                                                        *__slate_slot_1378 + (1 as i32),
                                                    );
                                                    unsafe {
                                                        (*(*__slate_slot_1377)).nMem =
                                                            *__slate_slot_1379;
                                                    }
                                                    *__slate_slot_805 = *__slate_slot_1379;
                                                    *__slate_slot_802 = 0 as i32;
                                                    loop {
                                                        if *__slate_slot_802
                                                            < ((unsafe {
                                                                (*(*__slate_slot_797)).nCol
                                                            })
                                                                as i32)
                                                        {
                                                            '__join_8: {
                                                                if *__slate_slot_802
                                                                    != *__slate_slot_801
                                                                    && (((unsafe {
                                                                        (*unsafe { unsafe { (*(*__slate_slot_797)).aCol }.offset(*__slate_slot_802 as isize) }).colFlags
                                                                    })
                                                                        as u32)
                                                                        as i32)
                                                                        & (32 as i32)
                                                                        == (0 as i32)
                                                                {
                                                                    if *__slate_slot_806
                                                                        != std::ptr::null_mut::<Index>(
                                                                        )
                                                                    {
                                                                        std::ptr::write(
                                                                            __slate_slot_811,
                                                                            unsafe {
                                                                                sqlite3TableColumnToIndex(*__slate_slot_806, *__slate_slot_802)
                                                                            },
                                                                        );
                                                                        std::ptr::write(
                                                                            __slate_slot_812,
                                                                            unsafe {
                                                                                sqlite3TableColumnToIndex(*__slate_slot_806, *__slate_slot_801)
                                                                            },
                                                                        );
                                                                        if *__slate_slot_811
                                                                            < (((unsafe {
                                                                                (*(*__slate_slot_806)).nKeyCol
                                                                            })
                                                                                as u32)
                                                                                as i32)
                                                                        {
                                                                            break '__join_8;
                                                                        } else {
                                                                            *__slate_slot_810 = *__slate_slot_804 + (1 as i32) + *__slate_slot_811 - ((*__slate_slot_811 > *__slate_slot_812) as i32);
                                                                        }
                                                                    } else {
                                                                        *__slate_slot_810 =
                                                                            *__slate_slot_804
                                                                                + (1 as i32)
                                                                                + *__slate_slot_807;
                                                                    }
                                                                    if *__slate_slot_802
                                                                        == ((unsafe {
                                                                            (*(*__slate_slot_797))
                                                                                .iPKey
                                                                        })
                                                                            as i32)
                                                                    {
                                                                        unsafe {
                                                                            sqlite3VdbeAddOp2(
                                                                                *__slate_slot_809,
                                                                                77 as i32,
                                                                                0 as i32,
                                                                                *__slate_slot_810,
                                                                            )
                                                                        };
                                                                    } else {
                                                                        std::ptr::write(
                                                                            __slate_slot_813,
                                                                            unsafe {
                                                                                (*unsafe { unsafe { (*(*__slate_slot_797)).aCol }.offset(*__slate_slot_802 as isize) }).affinity
                                                                            },
                                                                        );
                                                                        if (*__slate_slot_813
                                                                            as i32)
                                                                            == (69 as i32)
                                                                        {
                                                                            unsafe {
                                                                                (*unsafe { unsafe { (*(*__slate_slot_797)).aCol }.offset(*__slate_slot_802 as isize) }).affinity = (67 as i32) as i8;
                                                                            }
                                                                        }
                                                                        unsafe {
                                                                            sqlite3ExprCodeGetColumnOfTable(*__slate_slot_809, *__slate_slot_797, *__slate_slot_808, *__slate_slot_802, *__slate_slot_810)
                                                                        };
                                                                        unsafe {
                                                                            (*unsafe { unsafe { (*(*__slate_slot_797)).aCol }.offset(*__slate_slot_802 as isize) }).affinity = *__slate_slot_813;
                                                                        }
                                                                    }
                                                                    std::ptr::write(
                                                                        __slate_slot_1382,
                                                                        *__slate_slot_807,
                                                                    );
                                                                    std::ptr::write(
                                                                        __slate_slot_1383,
                                                                        *__slate_slot_1382
                                                                            + (1 as i32),
                                                                    );
                                                                    *__slate_slot_807 =
                                                                        *__slate_slot_1383;
                                                                }
                                                            }
                                                            std::ptr::write(
                                                                __slate_slot_1380,
                                                                *__slate_slot_802,
                                                            );
                                                            std::ptr::write(
                                                                __slate_slot_1381,
                                                                *__slate_slot_1380 + (1 as i32),
                                                            );
                                                            *__slate_slot_802 = *__slate_slot_1381;
                                                        } else {
                                                            break;
                                                        }
                                                    }
                                                    if *__slate_slot_807 == (0 as i32) {
                                                        // /* dbsqlfuzz 5f09e7bcc78b4954d06bf9f2400d7715f48d1fef */
                                                        std::ptr::write(__slate_slot_1384, pParse);
                                                        std::ptr::write(
                                                            __slate_slot_1385,
                                                            unsafe { (*(*__slate_slot_1384)).nMem },
                                                        );
                                                        std::ptr::write(
                                                            __slate_slot_1386,
                                                            *__slate_slot_1385 + (1 as i32),
                                                        );
                                                        unsafe {
                                                            (*(*__slate_slot_1384)).nMem =
                                                                *__slate_slot_1386;
                                                        }
                                                        unsafe {
                                                            sqlite3VdbeAddOp2(
                                                                *__slate_slot_809,
                                                                77 as i32,
                                                                0 as i32,
                                                                *__slate_slot_804 + (1 as i32),
                                                            )
                                                        };
                                                        *__slate_slot_807 = 1 as i32;
                                                    }
                                                    unsafe {
                                                        sqlite3VdbeAddOp3(
                                                            *__slate_slot_809,
                                                            99 as i32,
                                                            *__slate_slot_804 + (1 as i32),
                                                            *__slate_slot_807,
                                                            *__slate_slot_805,
                                                        )
                                                    };
                                                    if *__slate_slot_806
                                                        != std::ptr::null_mut::<Index>()
                                                    {
                                                        unsafe {
                                                            sqlite3VdbeAddOp4Int(
                                                                *__slate_slot_809,
                                                                140 as i32,
                                                                *__slate_slot_808,
                                                                *__slate_slot_805,
                                                                *__slate_slot_804 + (1 as i32),
                                                                ((unsafe {
                                                                    (*(*__slate_slot_806)).nKeyCol
                                                                })
                                                                    as u32)
                                                                    as i32,
                                                            )
                                                        };
                                                    } else {
                                                        unsafe {
                                                            sqlite3VdbeAddOp3(
                                                                *__slate_slot_809,
                                                                130 as i32,
                                                                *__slate_slot_808,
                                                                *__slate_slot_805,
                                                                *__slate_slot_804,
                                                            )
                                                        };
                                                    }
                                                    unsafe {
                                                        sqlite3VdbeChangeP5(
                                                            *__slate_slot_809,
                                                            ((2 as i32) as i16) as u16,
                                                        )
                                                    };
                                                    unsafe {
                                                        sqlite3VdbeAddOp2(
                                                            *__slate_slot_809,
                                                            40 as i32,
                                                            *__slate_slot_808,
                                                            *__slate_slot_803 + (1 as i32),
                                                        )
                                                    };
                                                    {}
                                                    unsafe {
                                                        sqlite3VdbeJumpHere(
                                                            *__slate_slot_809,
                                                            *__slate_slot_803,
                                                        )
                                                    };
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
        }
        unsafe { sqlite3DbFree(*__slate_slot_796, *__slate_slot_800 as *mut ()) };
        unsafe { sqlite3SrcListDelete(*__slate_slot_796, pSrc) };
    }
}

// /*
// ** Remember that the parser tree element pPtr was created using
// ** the token pToken.
// **
// ** In other words, construct a new RenameToken object and add it
// ** to the list of RenameToken objects currently being built up
// ** in pParse->pRename.
// **
// ** The pPtr argument is returned so that this routine can be used
// ** with tail recursion in tokenExpr() routine, for a small performance
// ** improvement.
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3RenameTokenMap(
    mut pParse: *mut Parse,
    mut pPtr: *const (),
    mut pToken: *const Token,
) -> *const () {
    let mut pNew: *mut RenameToken = unsafe { std::mem::zeroed() };
    0 as i32;
    {}
    if (((unsafe { (*pParse).eParseMode }) as u32) as i32) != (3 as i32) {
        pNew = (unsafe { sqlite3DbMallocZero(unsafe { (*pParse).db }, 32 as u64) })
            as *mut RenameToken;
        if pNew != std::ptr::null_mut::<RenameToken>() {
            unsafe {
                (*pNew).p = pPtr;
            }
            unsafe {
                (*pNew).t = unsafe { *pToken };
            }
            unsafe {
                (*pNew).pNext = unsafe { (*pParse).pRename };
            }
            unsafe {
                (*pParse).pRename = pNew;
            }
        }
    }
    return pPtr;
}

// /*
// ** It is assumed that there is already a RenameToken object associated
// ** with parse tree element pFrom. This function remaps the associated token
// ** to parse tree element pTo.
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3RenameTokenRemap(
    mut pParse: *mut Parse,
    mut pTo: *const (),
    mut pFrom: *const (),
) {
    let mut p: *mut RenameToken = unsafe { std::mem::zeroed() };
    {}
    p = unsafe { (*pParse).pRename };
    '__slate_break_1264: while p != std::ptr::null_mut::<RenameToken>() {
        if (unsafe { (*p).p }) == pFrom {
            unsafe {
                (*p).p = pTo;
            }
            break '__slate_break_1264;
        }
        p = unsafe { (*p).pNext };
    }
}

// /*
// ** Remove all nodes that are part of expression pExpr from the rename list.
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3RenameExprUnmap(mut pParse: *mut Parse, mut pExpr: *mut Expr) {
    let mut eMode: u8 = unsafe { (*pParse).eParseMode };
    let mut sWalker: Walker = unsafe { std::mem::zeroed() };
    unsafe {
        memset(
            std::ptr::addr_of_mut!(sWalker) as *mut (),
            0 as i32,
            48 as u64,
        )
    };
    sWalker.pParse = pParse;
    sWalker.xExprCallback = Some(renameUnmapExprCb);
    sWalker.xSelectCallback = Some(renameUnmapSelectCb);
    unsafe {
        (*pParse).eParseMode = ((3 as i32) as i8) as u8;
    }
    unsafe { sqlite3WalkExpr(std::ptr::addr_of_mut!(sWalker), pExpr) };
    unsafe {
        (*pParse).eParseMode = eMode;
    }
}

// /*
// ** Remove all nodes that are part of expression-list pEList from the
// ** rename list.
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3RenameExprlistUnmap(mut pParse: *mut Parse, mut pEList: *mut ExprList) {
    if pEList != std::ptr::null_mut::<ExprList>() {
        let mut i: i32 = 0 as i32;
        let mut sWalker: Walker = unsafe { std::mem::zeroed() };
        unsafe {
            memset(
                std::ptr::addr_of_mut!(sWalker) as *mut (),
                0 as i32,
                48 as u64,
            )
        };
        sWalker.pParse = pParse;
        sWalker.xExprCallback = Some(renameUnmapExprCb);
        unsafe { sqlite3WalkExprList(std::ptr::addr_of_mut!(sWalker), pEList) };
        i = 0 as i32;
        '__slate_break_1269: loop {
            if !(i < unsafe { (*pEList).nExpr }) {
                break;
            }
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
                sqlite3RenameTokenRemap(
                    pParse,
                    std::ptr::null::<()>(),
                    ((unsafe {
                        (*unsafe {
                            unsafe { std::ptr::addr_of_mut!((*pEList).a) as *mut ExprList_item }
                                .offset(i as isize)
                        })
                        .zEName
                    }) as *mut ()) as *const (),
                );
            }
            let __v1387: i32 = i;
            let __v1388: i32 = __v1387 + (1 as i32);
            i = __v1388;
        }
    }
}

// /*
// ** The code in this file only exists if we are not omitting the
// ** ALTER TABLE logic from the build.
// */
// /*
// ** Parameter zName is the name of a table that is about to be altered
// ** (either with ALTER TABLE ... RENAME TO or ALTER TABLE ... ADD COLUMN).
// ** If the table is a system table, this function leaves an error message
// ** in pParse->zErr (system tables may not be altered) and returns non-zero.
// **
// ** Or, if zName is not a system table, zero is returned.
// */
fn isAlterableTable(mut pParse: *mut Parse, mut pTab: *mut Table) -> i32 {
    let __v1389: bool;
    if (0 as i32)
        == unsafe {
            sqlite3_strnicmp(
                (unsafe { (*pTab).zName }) as *const i8,
                (b"sqlite_\0".as_ptr() as *mut i8) as *const i8,
                7 as i32,
            )
        }
        || (unsafe { (*pTab).tabFlags }) & ((32768 as i32) as u32) != ((0 as i32) as u32)
    {
        __v1389 = true as bool;
    } else {
        let __v1390: bool;
        if (unsafe { (*pTab).tabFlags }) & ((4096 as i32) as u32) != ((0 as i32) as u32) {
            __v1390 =
                (unsafe { sqlite3ReadOnlyShadowTables(unsafe { (*pParse).db }) }) != (0 as i32);
        } else {
            __v1390 = false as bool;
        }
        __v1389 = __v1390;
    }
    if __v1389 {
        unsafe {
            sqlite3ErrorMsg(
                pParse,
                (b"table %s may not be altered\0".as_ptr() as *mut i8) as *const i8,
                unsafe { (*pTab).zName },
            )
        };
        return 1 as i32;
    }
    return 0 as i32;
}

// /*
// ** Generate code to verify that the schemas of database zDb and, if
// ** bTemp is not true, database "temp", can still be parsed. This is
// ** called at the end of the generation of an ALTER TABLE ... RENAME ...
// ** statement to ensure that the operation has not rendered any schema
// ** objects unusable.
// */
fn renameTestSchema(
    mut pParse: *mut Parse,
    mut zDb: *const i8,
    mut bTemp: i32,
    mut zWhen: *const i8,
    mut bNoDQS: i32,
) {
    unsafe {
        (*pParse)
            .__slate_bits_0
            .__set_colNamesSet((1 as i32) as u32);
    }
    unsafe {
        sqlite3NestedParse(pParse, (b"SELECT 1 FROM \"%w\".sqlite_master WHERE name NOT LIKE 'sqliteX_%%' ESCAPE 'X' AND sql NOT LIKE 'create virtual%%' AND sqlite_rename_test(%Q, sql, type, name, %d, %Q, %d)=NULL \0".as_ptr() as *mut i8) as *const i8, zDb, zDb, bTemp, zWhen, bNoDQS)
    };
    if bTemp == (0 as i32) {
        unsafe {
            sqlite3NestedParse(pParse, (b"SELECT 1 FROM temp.sqlite_master WHERE name NOT LIKE 'sqliteX_%%' ESCAPE 'X' AND sql NOT LIKE 'create virtual%%' AND sqlite_rename_test(%Q, sql, type, name, 1, %Q, %d)=NULL \0".as_ptr() as *mut i8) as *const i8, zDb, zWhen, bNoDQS)
        };
    }
}

// /* Parse context */
// /* Name of db to verify schema of */
// /* True if this is the temp db */
// /* "when" part of error message */
// /* Do not allow DQS in the schema */
// /*
// ** Generate VM code to replace any double-quoted strings (but not double-quoted
// ** identifiers) within the "sql" column of the sqlite_schema table in
// ** database zDb with their single-quoted equivalents. If argument bTemp is
// ** not true, similarly update all SQL statements in the sqlite_schema table
// ** of the temp db.
// */
fn renameFixQuotes(mut pParse: *mut Parse, mut zDb: *const i8, mut bTemp: i32) {
    unsafe {
        sqlite3NestedParse(pParse, (b"UPDATE \"%w\".sqlite_master SET sql = sqlite_rename_quotefix(%Q, sql)WHERE name NOT LIKE 'sqliteX_%%' ESCAPE 'X' AND sql NOT LIKE 'create virtual%%'\0".as_ptr() as *mut i8) as *const i8, zDb, zDb)
    };
    if bTemp == (0 as i32) {
        unsafe {
            sqlite3NestedParse(pParse, (b"UPDATE temp.sqlite_master SET sql = sqlite_rename_quotefix('temp', sql)WHERE name NOT LIKE 'sqliteX_%%' ESCAPE 'X' AND sql NOT LIKE 'create virtual%%'\0".as_ptr() as *mut i8) as *const i8)
        };
    }
}

// /*
// ** Generate code to reload the schema for database iDb. And, if iDb!=1, for
// ** the temp database as well.
// */
fn renameReloadSchema(mut pParse: *mut Parse, mut iDb: i32, mut p5: u16) {
    let mut v: *mut Vdbe = unsafe { (*pParse).pVdbe };
    if v != std::ptr::null_mut::<Vdbe>() {
        unsafe { sqlite3ChangeCookie(pParse, iDb) };
        unsafe {
            sqlite3VdbeAddParseSchemaOp(
                unsafe { (*pParse).pVdbe },
                iDb,
                std::ptr::null_mut::<i8>(),
                p5,
            )
        };
        if iDb != (1 as i32) {
            unsafe {
                sqlite3VdbeAddParseSchemaOp(
                    unsafe { (*pParse).pVdbe },
                    1 as i32,
                    std::ptr::null_mut::<i8>(),
                    p5,
                )
            };
        }
    }
}

// /* Parser context. */
// /* The table to rename. */
// /* The new table name. */
// /*
// ** Write code that will raise an error if the table described by
// ** zDb and zTab is not empty.
// */
fn sqlite3ErrorIfNotEmpty(
    mut pParse: *mut Parse,
    mut zDb: *const i8,
    mut zTab: *const i8,
    mut zErr: *const i8,
) {
    unsafe {
        sqlite3NestedParse(
            pParse,
            (b"SELECT raise(ABORT,%Q) FROM \"%w\".\"%w\"\0".as_ptr() as *mut i8) as *const i8,
            zErr,
            zDb,
            zTab,
        )
    };
}

// /*
// ** Parameter pTab is the subject of an ALTER TABLE ... RENAME COLUMN
// ** command. This function checks if the table is a view or virtual
// ** table (columns of views or virtual tables may not be renamed). If so,
// ** it loads an error message into pParse and returns non-zero.
// **
// ** Or, if pTab is not a view or virtual table, zero is returned.
// */
fn isRealTable(mut pParse: *mut Parse, mut pTab: *mut Table, mut iOp: i32) -> i32 {
    let mut zType: *const i8 = std::ptr::null::<i8>();
    if (((unsafe { (*pTab).eTabType }) as u32) as i32) == (2 as i32) {
        zType = (b"view\0".as_ptr() as *mut i8) as *const i8;
    }
    if (((unsafe { (*pTab).eTabType }) as u32) as i32) == (1 as i32) {
        zType = (b"virtual table\0".as_ptr() as *mut i8) as *const i8;
    }
    if zType != std::ptr::null::<i8>() {
        let mut azMsg: __SlateAlign16<[*const i8; 3]> = __SlateAlign16([
            (b"rename columns of\0".as_ptr() as *mut i8) as *const i8,
            (b"drop column from\0".as_ptr() as *mut i8) as *const i8,
            (b"edit constraints of\0".as_ptr() as *mut i8) as *const i8,
        ]);
        0 as i32;
        unsafe {
            sqlite3ErrorMsg(
                pParse,
                (b"cannot %s %s \"%s\"\0".as_ptr() as *mut i8) as *const i8,
                unsafe {
                    *unsafe { (azMsg.0.as_mut_ptr() as *mut *const i8).offset(iOp as isize) }
                },
                zType,
                unsafe { (*pTab).zName },
            )
        };
        return 1 as i32;
    }
    return 0 as i32;
}

// /*
// ** Walker callback used by sqlite3RenameExprUnmap().
// */
#[unsafe(link_section = ".text.slate_distinct.alter.renameUnmapExprCb")]
extern "C-unwind" fn renameUnmapExprCb(mut pWalker: *mut Walker, mut pExpr: *mut Expr) -> i32 {
    let mut pParse: *mut Parse = unsafe { (*pWalker).pParse };
    sqlite3RenameTokenRemap(pParse, std::ptr::null::<()>(), pExpr as *const ());
    if (unsafe { (*pExpr).flags }) & (((16777216 as i32) | (33554432 as i32)) as u32)
        == ((0 as i32) as u32)
    {
        sqlite3RenameTokenRemap(
            pParse,
            std::ptr::null::<()>(),
            (unsafe { std::ptr::addr_of_mut!((*pExpr).y.pTab) }) as *const (),
        );
    }
    return 0 as i32;
}

// /*
// ** Iterate through the Select objects that are part of WITH clauses attached
// ** to select statement pSelect.
// */
fn renameWalkWith(mut pWalker: *mut Walker, mut pSelect: *mut Select) {
    let mut pWith: *mut With = unsafe { (*pSelect).pWith };
    if pWith != std::ptr::null_mut::<With>() {
        let mut pParse: *mut Parse = unsafe { (*pWalker).pParse };
        let mut i: i32 = 0 as i32;
        let mut pCopy: *mut With = std::ptr::null_mut::<With>();
        0 as i32;
        if (unsafe {
            (*unsafe {
                (*unsafe {
                    unsafe { std::ptr::addr_of_mut!((*pWith).a) as *mut Cte }
                        .offset((0 as i32) as isize)
                })
                .pSelect
            })
            .selFlags
        }) & ((64 as i32) as u32)
            == ((0 as i32) as u32)
        {
            // /* Push a copy of the With object onto the with-stack. We use a copy
            //       ** here as the original will be expanded and resolved (flags SF_Expanded
            //       ** and SF_Resolved) below. And the parser code that uses the with-stack
            //       ** fails if the Select objects on it have already been expanded and
            //       ** resolved.  */
            pCopy = unsafe { sqlite3WithDup(unsafe { (*pParse).db }, pWith) };
            pCopy = unsafe { sqlite3WithPush(pParse, pCopy, ((1 as i32) as i8) as u8) };
        }
        i = 0 as i32;
        '__slate_break_1265: loop {
            if !(i < unsafe { (*pWith).nCte }) {
                break;
            }
            let mut p: *mut Select = unsafe {
                (*unsafe {
                    unsafe { std::ptr::addr_of_mut!((*pWith).a) as *mut Cte }.offset(i as isize)
                })
                .pSelect
            };
            let mut sNC: NameContext = unsafe { std::mem::zeroed() };
            unsafe { memset(std::ptr::addr_of_mut!(sNC) as *mut (), 0 as i32, 56 as u64) };
            sNC.pParse = pParse;
            if pCopy != std::ptr::null_mut::<With>() {
                unsafe { sqlite3SelectPrep(sNC.pParse, p, std::ptr::addr_of_mut!(sNC)) };
            }
            if (unsafe { (*unsafe { (*sNC.pParse).db }).mallocFailed }) != (0 as u8) {
                return;
            }
            unsafe { sqlite3WalkSelect(pWalker, p) };
            sqlite3RenameExprlistUnmap(pParse, unsafe {
                (*unsafe {
                    unsafe { std::ptr::addr_of_mut!((*pWith).a) as *mut Cte }.offset(i as isize)
                })
                .pCols
            });
            let __v1391: i32 = i;
            let __v1392: i32 = __v1391 + (1 as i32);
            i = __v1392;
        }
        if pCopy != std::ptr::null_mut::<With>() && (unsafe { (*pParse).pWith }) == pCopy {
            unsafe {
                (*pParse).pWith = unsafe { (*pCopy).pOuter };
            }
        }
    }
}

// /*
// ** Unmap all tokens in the IdList object passed as the second argument.
// */
fn unmapColumnIdlistNames(mut pParse: *mut Parse, mut pIdList: *const IdList) {
    let mut ii: i32 = 0 as i32;
    0 as i32;
    ii = 0 as i32;
    '__slate_break_1266: loop {
        if !(ii < unsafe { (*pIdList).nId }) {
            break;
        }
        sqlite3RenameTokenRemap(
            pParse,
            std::ptr::null::<()>(),
            (unsafe {
                (*unsafe {
                    unsafe { std::ptr::addr_of!((*pIdList).a) as *const IdList_item }
                        .offset(ii as isize)
                })
                .zName
            }) as *const (),
        );
        let __v1393: i32 = ii;
        let __v1394: i32 = __v1393 + (1 as i32);
        ii = __v1394;
    }
}

// /*
// ** Walker callback used by sqlite3RenameExprUnmap().
// */
#[unsafe(link_section = ".text.slate_distinct.alter.renameUnmapSelectCb")]
extern "C-unwind" fn renameUnmapSelectCb(mut pWalker: *mut Walker, mut p: *mut Select) -> i32 {
    let mut pParse: *mut Parse = unsafe { (*pWalker).pParse };
    let mut i: i32 = 0 as i32;
    if (unsafe { (*pParse).nErr }) != (0 as i32) {
        return 2 as i32;
    }
    {}
    {}
    if (unsafe { (*p).selFlags }) & (((2097152 as i32) | (67108864 as i32)) as u32) != (0 as u32) {
        return 1 as i32;
    }
    if (unsafe { (*p).pEList }) != std::ptr::null_mut::<ExprList>() {
        let mut pList: *mut ExprList = unsafe { (*p).pEList };
        i = 0 as i32;
        '__slate_break_1267: loop {
            if !(i < unsafe { (*pList).nExpr }) {
                break;
            }
            if (unsafe {
                (*unsafe {
                    unsafe { std::ptr::addr_of_mut!((*pList).a) as *mut ExprList_item }
                        .offset(i as isize)
                })
                .zEName
            }) != std::ptr::null_mut::<i8>()
                && ((unsafe {
                    (*unsafe {
                        unsafe { std::ptr::addr_of_mut!((*pList).a) as *mut ExprList_item }
                            .offset(i as isize)
                    })
                    .fg
                    .__slate_bits_0
                    .__get_eEName()
                }) as i32)
                    == (0 as i32)
            {
                sqlite3RenameTokenRemap(
                    pParse,
                    std::ptr::null::<()>(),
                    ((unsafe {
                        (*unsafe {
                            unsafe { std::ptr::addr_of_mut!((*pList).a) as *mut ExprList_item }
                                .offset(i as isize)
                        })
                        .zEName
                    }) as *mut ()) as *const (),
                );
            }
            let __v1395: i32 = i;
            let __v1396: i32 = __v1395 + (1 as i32);
            i = __v1396;
        }
    }
    // /* Every Select as a SrcList, even if it is empty */
    if (unsafe { (*p).pSrc }) != std::ptr::null_mut::<SrcList>() {
        let mut pSrc: *mut SrcList = unsafe { (*p).pSrc };
        i = 0 as i32;
        '__slate_break_1268: loop {
            if !(i < unsafe { (*pSrc).nSrc }) {
                break;
            }
            sqlite3RenameTokenRemap(
                pParse,
                std::ptr::null::<()>(),
                ((unsafe {
                    (*unsafe {
                        unsafe { std::ptr::addr_of_mut!((*pSrc).a) as *mut SrcItem }
                            .offset(i as isize)
                    })
                    .zName
                }) as *mut ()) as *const (),
            );
            if ((unsafe {
                (*unsafe {
                    unsafe { std::ptr::addr_of_mut!((*pSrc).a) as *mut SrcItem }.offset(i as isize)
                })
                .fg
                .__slate_bits_0
                .__get_isUsing()
            }) as i32)
                == (0 as i32)
            {
                unsafe {
                    sqlite3WalkExpr(pWalker, unsafe {
                        (*unsafe {
                            unsafe { std::ptr::addr_of_mut!((*pSrc).a) as *mut SrcItem }
                                .offset(i as isize)
                        })
                        .u3
                        .pOn
                    })
                };
            } else {
                unmapColumnIdlistNames(
                    pParse,
                    (unsafe {
                        (*unsafe {
                            unsafe { std::ptr::addr_of_mut!((*pSrc).a) as *mut SrcItem }
                                .offset(i as isize)
                        })
                        .u3
                        .pUsing
                    }) as *const IdList,
                );
            }
            let __v1397: i32 = i;
            let __v1398: i32 = __v1397 + (1 as i32);
            i = __v1398;
        }
    }
    renameWalkWith(pWalker, p);
    return 0 as i32;
}

// /*
// ** Free the list of RenameToken objects given in the second argument
// */
fn renameTokenFree(mut db: *mut sqlite3, mut pToken: *mut RenameToken) {
    let mut pNext: *mut RenameToken = unsafe { std::mem::zeroed() };
    let mut p: *mut RenameToken = unsafe { std::mem::zeroed() };
    p = pToken;
    '__slate_break_1270: while p != std::ptr::null_mut::<RenameToken>() {
        pNext = unsafe { (*p).pNext };
        unsafe { sqlite3DbFree(db, p as *mut ()) };
        p = pNext;
    }
}

// /*
// ** Search the Parse object passed as the first argument for a RenameToken
// ** object associated with parse tree element pPtr. If found, return a pointer
// ** to it. Otherwise, return NULL.
// **
// ** If the second argument passed to this function is not NULL and a matching
// ** RenameToken object is found, remove it from the Parse object and add it to
// ** the list maintained by the RenameCtx object.
// */
fn renameTokenFind(
    mut pParse: *mut Parse,
    mut pCtx: *mut RenameCtx,
    mut pPtr: *const (),
) -> *mut RenameToken {
    let mut pp: *mut *mut RenameToken = unsafe { std::mem::zeroed() };
    if pPtr == std::ptr::null::<()>() {
        return std::ptr::null_mut::<RenameToken>();
    }
    pp = unsafe { std::ptr::addr_of_mut!((*pParse).pRename) };
    '__slate_break_1271: while (unsafe { *pp }) != std::ptr::null_mut::<RenameToken>() {
        if (unsafe { (*unsafe { *pp }).p }) == pPtr {
            let mut pToken: *mut RenameToken = unsafe { *pp };
            if pCtx != std::ptr::null_mut::<RenameCtx>() {
                unsafe {
                    *pp = unsafe { (*pToken).pNext };
                }
                unsafe {
                    (*pToken).pNext = unsafe { (*pCtx).pList };
                }
                unsafe {
                    (*pCtx).pList = pToken;
                }
                let __v1399: *mut RenameCtx = pCtx;
                let __v1400: i32 = unsafe { (*__v1399).nList };
                let __v1401: i32 = __v1400 + (1 as i32);
                unsafe {
                    (*__v1399).nList = __v1401;
                }
            }
            return pToken;
        }
        pp = unsafe { std::ptr::addr_of_mut!((*unsafe { *pp }).pNext) };
    }
    return std::ptr::null_mut::<RenameToken>();
}

// /*
// ** This is a Walker select callback. It does nothing. It is only required
// ** because without a dummy callback, sqlite3WalkExpr() and similar do not
// ** descend into sub-select statements.
// */
#[unsafe(link_section = ".text.slate_distinct.alter.renameColumnSelectCb")]
extern "C-unwind" fn renameColumnSelectCb(mut pWalker: *mut Walker, mut p: *mut Select) -> i32 {
    if (unsafe { (*p).selFlags }) & (((2097152 as i32) | (67108864 as i32)) as u32) != (0 as u32) {
        {}
        {}
        return 1 as i32;
    }
    renameWalkWith(pWalker, p);
    return 0 as i32;
}

// /*
// ** This is a Walker expression callback.
// **
// ** For every TK_COLUMN node in the expression tree, search to see
// ** if the column being references is the column being renamed by an
// ** ALTER TABLE statement.  If it is, then attach its associated
// ** RenameToken object to the list of RenameToken objects being
// ** constructed in RenameCtx object at pWalker->u.pRename.
// */
#[unsafe(link_section = ".text.slate_distinct.alter.renameColumnExprCb")]
extern "C-unwind" fn renameColumnExprCb(mut pWalker: *mut Walker, mut pExpr: *mut Expr) -> i32 {
    let mut p: *mut RenameCtx = unsafe { (*pWalker).u.pRename };
    if (((unsafe { (*pExpr).op }) as u32) as i32) == (78 as i32)
        && ((unsafe { (*pExpr).iColumn }) as i32) == unsafe { (*p).iCol }
        && (unsafe { (*unsafe { (*pWalker).pParse }).pTriggerTab }) == unsafe { (*p).pTab }
    {
        renameTokenFind(
            unsafe { (*pWalker).pParse },
            p,
            (pExpr as *mut ()) as *const (),
        );
    } else {
        if (((unsafe { (*pExpr).op }) as u32) as i32) == (168 as i32)
            && ((unsafe { (*pExpr).iColumn }) as i32) == unsafe { (*p).iCol }
            && (unsafe { (*pExpr).flags }) & (((16777216 as i32) | (33554432 as i32)) as u32)
                == ((0 as i32) as u32)
            && (unsafe { (*p).pTab }) == unsafe { (*pExpr).y.pTab }
        {
            renameTokenFind(
                unsafe { (*pWalker).pParse },
                p,
                (pExpr as *mut ()) as *const (),
            );
        }
    }
    return 0 as i32;
}

// /*
// ** The RenameCtx contains a list of tokens that reference a column that
// ** is being renamed by an ALTER TABLE statement.  Return the "last"
// ** RenameToken in the RenameCtx and remove that RenameToken from the
// ** RenameContext.  "Last" means the last RenameToken encountered when
// ** the input SQL is parsed from left to right.  Repeated calls to this routine
// ** return all column name tokens in the order that they are encountered
// ** in the SQL statement.
// */
fn renameColumnTokenNext(mut pCtx: *mut RenameCtx) -> *mut RenameToken {
    let mut pBest: *mut RenameToken = unsafe { (*pCtx).pList };
    let mut pToken: *mut RenameToken = unsafe { std::mem::zeroed() };
    let mut pp: *mut *mut RenameToken = unsafe { std::mem::zeroed() };
    pToken = unsafe { (*pBest).pNext };
    '__slate_break_1272: while pToken != std::ptr::null_mut::<RenameToken>() {
        if (unsafe { (*pToken).t.z }) > unsafe { (*pBest).t.z } {
            pBest = pToken;
        }
        pToken = unsafe { (*pToken).pNext };
    }
    pp = unsafe { std::ptr::addr_of_mut!((*pCtx).pList) };
    '__slate_break_1273: while (unsafe { *pp }) != pBest {
        {}
        pp = unsafe { std::ptr::addr_of_mut!((*unsafe { *pp }).pNext) };
    }
    unsafe {
        *pp = unsafe { (*pBest).pNext };
    }
    return pBest;
}

// /*
// ** Set the error message of the context passed as the first argument to
// ** the result of formatting zFmt using printf() style formatting.
// */
unsafe extern "C-unwind" fn errorMPrintf(
    mut pCtx: *mut sqlite3_context,
    mut zFmt: *const i8,
    mut __va_args: ...
) {
    let mut db: *mut sqlite3 = unsafe { sqlite3_context_db_handle(pCtx) };
    let mut zErr: *mut i8 = std::ptr::null_mut::<i8>();
    let mut ap: core::ffi::VaList<'_> = unsafe { std::mem::zeroed() };
    ap = __va_args.clone();
    zErr = unsafe { sqlite3VMPrintf(db, zFmt, ap.clone()) };
    {}
    if zErr != std::ptr::null_mut::<i8>() {
        unsafe { sqlite3_result_error(pCtx, zErr as *const i8, -(1 as i32)) };
        unsafe { sqlite3DbFree(db, zErr as *mut ()) };
    } else {
        unsafe { sqlite3_result_error_nomem(pCtx) };
    }
}

// /*
// ** An error occurred while parsing or otherwise processing a database
// ** object (either pParse->pNewTable, pNewIndex or pNewTrigger) as part of an
// ** ALTER TABLE RENAME COLUMN program. The error message emitted by the
// ** sub-routine is currently stored in pParse->zErrMsg. This function
// ** adds context to the error message and then stores it in pCtx.
// */
fn renameColumnParseError(
    mut pCtx: *mut sqlite3_context,
    mut zWhen: *const i8,
    mut pType: *mut sqlite3_value,
    mut pObject: *mut sqlite3_value,
    mut pParse: *mut Parse,
) {
    let mut zT: *const i8 = (unsafe { sqlite3_value_text(pType) }) as *const i8;
    let mut zN: *const i8 = (unsafe { sqlite3_value_text(pObject) }) as *const i8;
    let mut zErr: *mut i8 = unsafe { std::mem::zeroed() };
    zErr = unsafe {
        sqlite3MPrintf(
            unsafe { (*pParse).db },
            (b"error in %s %s%s%s: %s\0".as_ptr() as *mut i8) as *const i8,
            zT,
            zN,
            if (unsafe { *unsafe { zWhen.offset((0 as i32) as isize) } }) != (0 as i8) {
                b" \0".as_ptr() as *mut i8
            } else {
                b"\0".as_ptr() as *mut i8
            },
            zWhen,
            unsafe { (*pParse).zErrMsg },
        )
    };
    unsafe { sqlite3_result_error(pCtx, zErr as *const i8, -(1 as i32)) };
    unsafe { sqlite3DbFree(unsafe { (*pParse).db }, zErr as *mut ()) };
}

// /*
// ** For each name in the the expression-list pEList (i.e. each
// ** pEList->a[i].zName) that matches the string in zOld, extract the
// ** corresponding rename-token from Parse object pParse and add it
// ** to the RenameCtx pCtx.
// */
fn renameColumnElistNames(
    mut pParse: *mut Parse,
    mut pCtx: *mut RenameCtx,
    mut pEList: *const ExprList,
    mut zOld: *const i8,
) {
    if pEList != std::ptr::null::<ExprList>() {
        let mut i: i32 = 0 as i32;
        i = 0 as i32;
        '__slate_break_1277: loop {
            if !(i < unsafe { (*pEList).nExpr }) {
                break;
            }
            let mut zName: *const i8 = (unsafe {
                (*unsafe {
                    unsafe { std::ptr::addr_of!((*pEList).a) as *const ExprList_item }
                        .offset(i as isize)
                })
                .zEName
            }) as *const i8;
            let __v1404: bool;
            if ((unsafe {
                (*unsafe {
                    unsafe { std::ptr::addr_of!((*pEList).a) as *const ExprList_item }
                        .offset(i as isize)
                })
                .fg
                .__slate_bits_0
                .__get_eEName()
            }) as i32)
                == (0 as i32)
                && zName != std::ptr::null::<i8>()
            {
                __v1404 = (0 as i32) == unsafe { sqlite3_stricmp(zName, zOld) };
            } else {
                __v1404 = false as bool;
            }
            if __v1404 {
                renameTokenFind(pParse, pCtx, zName as *const ());
            }
            let __v1402: i32 = i;
            let __v1403: i32 = __v1402 + (1 as i32);
            i = __v1403;
        }
    }
}

// /*
// ** For each name in the the id-list pIdList (i.e. each pIdList->a[i].zName)
// ** that matches the string in zOld, extract the corresponding rename-token
// ** from Parse object pParse and add it to the RenameCtx pCtx.
// */
fn renameColumnIdlistNames(
    mut pParse: *mut Parse,
    mut pCtx: *mut RenameCtx,
    mut pIdList: *const IdList,
    mut zOld: *const i8,
) {
    if pIdList != std::ptr::null::<IdList>() {
        let mut i: i32 = 0 as i32;
        i = 0 as i32;
        '__slate_break_1278: loop {
            if !(i < unsafe { (*pIdList).nId }) {
                break;
            }
            let mut zName: *const i8 = (unsafe {
                (*unsafe {
                    unsafe { std::ptr::addr_of!((*pIdList).a) as *const IdList_item }
                        .offset(i as isize)
                })
                .zName
            }) as *const i8;
            if (0 as i32) == unsafe { sqlite3_stricmp(zName, zOld) } {
                renameTokenFind(pParse, pCtx, zName as *const ());
            }
            let __v1405: i32 = i;
            let __v1406: i32 = __v1405 + (1 as i32);
            i = __v1406;
        }
    }
}

// /*
// ** Parse the SQL statement zSql using Parse object (*p). The Parse object
// ** is initialized by this function before it is used.
// */
fn renameParseSql(
    mut p: *mut Parse,
    mut zDb: *const i8,
    mut db: *mut sqlite3,
    mut zSql: *const i8,
    mut bTemp: i32,
) -> i32 {
    let mut rc: i32 = 0 as i32;
    let mut flags: u64 = 0 as u64;
    unsafe { sqlite3ParseObjectInit(p, db) };
    if zSql == std::ptr::null::<i8>() {
        return 7 as i32;
    }
    if (unsafe {
        sqlite3_strnicmp(
            zSql,
            (b"CREATE \0".as_ptr() as *mut i8) as *const i8,
            7 as i32,
        )
    }) != (0 as i32)
    {
        return unsafe { sqlite3CorruptError(1168 as i32) };
    }
    if bTemp != (0 as i32) {
        unsafe {
            (*db).init.iDb = ((1 as i32) as i8) as u8;
        }
    } else {
        let mut iDb: i32 = unsafe { sqlite3FindDbName(db, zDb) };
        0 as i32;
        unsafe {
            (*db).init.iDb = (iDb as i8) as u8;
        }
    }
    unsafe {
        (*p).eParseMode = ((2 as i32) as i8) as u8;
    }
    unsafe {
        (*p).db = db;
    }
    unsafe {
        (*p).nQueryLoop = (1 as i32) as i16;
    }
    flags = unsafe { (*db).flags };
    {}
    let __v1407: *mut sqlite3 = db;
    let __v1408: u64 = unsafe { (*__v1407).flags };
    let __v1409: u64 = __v1408 | (((64 as i32) as i64) as u64) << (32 as i32);
    unsafe {
        (*__v1407).flags = __v1409;
    }
    rc = unsafe { sqlite3RunParser(p, zSql) };
    unsafe {
        (*db).flags = flags;
    }
    if (unsafe { (*db).mallocFailed }) != (0 as u8) {
        rc = 7 as i32;
    }
    if rc == (0 as i32)
        && ((unsafe { (*p).pNewTable }) == std::ptr::null_mut::<Table>()
            && (unsafe { (*p).pNewIndex }) == std::ptr::null_mut::<Index>()
            && (unsafe { (*p).pNewTrigger }) == std::ptr::null_mut::<Trigger>())
    {
        rc = unsafe { sqlite3CorruptError(1189 as i32) };
    }
    unsafe {
        (*db).init.iDb = ((0 as i32) as i8) as u8;
    }
    return rc;
}

// /* Memory to use for Parse object */
// /* Name of schema SQL belongs to */
// /* Database handle */
// /* SQL to parse */
// /* True if SQL is from temp schema */
// /*
// ** This function edits SQL statement zSql, replacing each token identified
// ** by the linked list pRename with the text of zNew. If argument bQuote is
// ** true, then zNew is always quoted first. If no error occurs, the result
// ** is loaded into context object pCtx as the result.
// **
// ** Or, if an error occurs (i.e. an OOM condition), an error is left in
// ** pCtx and an SQLite error code returned.
// */
fn renameEditSql(
    mut pCtx: *mut sqlite3_context,
    mut pRename: *mut RenameCtx,
    mut zSql: *const i8,
    mut zNew: *const i8,
    mut bQuote: i32,
) -> i32 {
    let mut nNew: i64 = (unsafe { sqlite3Strlen30(zNew) }) as i64;
    let mut nSql: i64 = (unsafe { sqlite3Strlen30(zSql) }) as i64;
    let mut db: *mut sqlite3 = unsafe { sqlite3_context_db_handle(pCtx) };
    let mut rc: i32 = 0 as i32;
    let mut zQuot: *mut i8 = std::ptr::null_mut::<i8>();
    let mut zOut: *mut i8 = unsafe { std::mem::zeroed() };
    let mut nQuot: i64 = (0 as i32) as i64;
    let mut zBuf1: *mut i8 = std::ptr::null_mut::<i8>();
    let mut zBuf2: *mut i8 = std::ptr::null_mut::<i8>();
    if zNew != std::ptr::null::<i8>() {
        // /* Set zQuot to point to a buffer containing a quoted copy of the
        //     ** identifier zNew. If the corresponding identifier in the original
        //     ** ALTER TABLE statement was quoted (bQuote==1), then set zNew to
        //     ** point to zQuot so that all substitutions are made using the
        //     ** quoted version of the new column name.  */
        zQuot =
            unsafe { sqlite3MPrintf(db, (b"\"%w\" \0".as_ptr() as *mut i8) as *const i8, zNew) };
        if zQuot == std::ptr::null_mut::<i8>() {
            return 7 as i32;
        } else {
            nQuot = ((unsafe { sqlite3Strlen30(zQuot as *const i8) }) - (1 as i32)) as i64;
        }
        0 as i32;
        zOut = (unsafe {
            sqlite3DbMallocZero(
                db,
                (nSql as u64)
                    .wrapping_add(
                        (((unsafe { (*pRename).nList }) as i64) as u64).wrapping_mul(nQuot as u64),
                    )
                    .wrapping_add(((1 as i32) as i64) as u64),
            )
        }) as *mut i8;
    } else {
        0 as i32;
        zOut = (unsafe {
            sqlite3DbMallocZero(
                db,
                (((2 as i32) as i64) as u64)
                    .wrapping_mul(nSql as u64)
                    .wrapping_add(((1 as i32) as i64) as u64)
                    .wrapping_mul(((3 as i32) as i64) as u64),
            )
        }) as *mut i8;
        if zOut != std::ptr::null_mut::<i8>() {
            zBuf1 =
                unsafe { zOut.offset((nSql * ((2 as i32) as i64) + ((1 as i32) as i64)) as isize) };
            zBuf2 =
                unsafe { zOut.offset((nSql * ((4 as i32) as i64) + ((2 as i32) as i64)) as isize) };
        }
    }
    // /* At this point pRename->pList contains a list of RenameToken objects
    //   ** corresponding to all tokens in the input SQL that must be replaced
    //   ** with the new column name, or with single-quoted versions of themselves.
    //   ** All that remains is to construct and return the edited SQL string. */
    if zOut != std::ptr::null_mut::<i8>() {
        let mut nOut: i64 = nSql;
        0 as i32;
        unsafe { memcpy(zOut as *mut (), zSql as *const (), nSql as u64) };
        '__slate_break_1281: while (unsafe { (*pRename).pList })
            != std::ptr::null_mut::<RenameToken>()
        {
            // /* Offset of token to replace in zOut */
            let mut iOff: i32 = 0 as i32;
            let mut nReplace: i64 = 0 as i64;
            let mut zReplace: *const i8 = unsafe { std::mem::zeroed() };
            let mut pBest: *mut RenameToken = renameColumnTokenNext(pRename);
            if zNew != std::ptr::null::<i8>() {
                let __v1410: bool;
                if bQuote == (0 as i32) {
                    __v1410 = (unsafe {
                        sqlite3IsIdChar(unsafe { *((unsafe { (*pBest).t.z }) as *mut u8) })
                    }) != (0 as i32);
                } else {
                    __v1410 = false as bool;
                }
                if __v1410 {
                    nReplace = nNew;
                    zReplace = zNew;
                } else {
                    nReplace = nQuot;
                    zReplace = zQuot as *const i8;
                    if ((unsafe {
                        *unsafe {
                            unsafe { (*pBest).t.z }.offset((unsafe { (*pBest).t.n }) as isize)
                        }
                    }) as i32)
                        == (34 as i32)
                    {
                        let __v1411: i64 = nReplace;
                        let __v1412: i64 = __v1411 + ((1 as i32) as i64);
                        nReplace = __v1412;
                    }
                }
            } else {
                // /* Dequote the double-quoted token. Then requote it again, this time
                //         ** using single quotes. If the character immediately following the
                //         ** original token within the input SQL was a single quote ('), then
                //         ** add another space after the new, single-quoted version of the
                //         ** token. This is so that (SELECT "string"'alias') maps to
                //         ** (SELECT 'string' 'alias'), and not (SELECT 'string''alias').  */
                unsafe {
                    memcpy(
                        zBuf1 as *mut (),
                        (unsafe { (*pBest).t.z }) as *const (),
                        (unsafe { (*pBest).t.n }) as u64,
                    )
                };
                unsafe {
                    *unsafe { zBuf1.offset((unsafe { (*pBest).t.n }) as isize) } = (0 as i32) as i8;
                }
                unsafe { sqlite3Dequote(zBuf1) };
                // /* otherwise malloc would have failed */
                0 as i32;
                unsafe {
                    sqlite3_snprintf(
                        (nSql * ((2 as i32) as i64)) as i32,
                        zBuf2,
                        (b"%Q%s\0".as_ptr() as *mut i8) as *const i8,
                        zBuf1,
                        if ((unsafe {
                            *unsafe {
                                unsafe { (*pBest).t.z }.offset((unsafe { (*pBest).t.n }) as isize)
                            }
                        }) as i32)
                            == (39 as i32)
                        {
                            b" \0".as_ptr() as *mut i8
                        } else {
                            b"\0".as_ptr() as *mut i8
                        },
                    )
                };
                zReplace = zBuf2 as *const i8;
                nReplace = (unsafe { sqlite3Strlen30(zReplace) }) as i64;
            }
            iOff =
                ((unsafe { unsafe { (*pBest).t.z }.offset_from(zSql as *const i8) }) as i64) as i32;
            if (((unsafe { (*pBest).t.n }) as u64) as i64) != nReplace {
                unsafe {
                    memmove(
                        (unsafe { zOut.offset(((iOff as i64) + nReplace) as isize) }) as *mut (),
                        (unsafe {
                            zOut.offset((iOff as u32).wrapping_add(unsafe { (*pBest).t.n }) as isize)
                        }) as *const (),
                        (nOut
                            - (((iOff as u32).wrapping_add(unsafe { (*pBest).t.n }) as u64) as i64))
                            as u64,
                    )
                };
                let __v1413: i64 = nOut;
                let __v1414: i64 =
                    __v1413 + (nReplace - (((unsafe { (*pBest).t.n }) as u64) as i64));
                nOut = __v1414;
                unsafe {
                    *unsafe { zOut.offset(nOut as isize) } = (0 as i32) as i8;
                }
            }
            unsafe {
                memcpy(
                    (unsafe { zOut.offset(iOff as isize) }) as *mut (),
                    zReplace as *const (),
                    nReplace as u64,
                )
            };
            unsafe { sqlite3DbFree(db, pBest as *mut ()) };
        }
        unsafe {
            sqlite3_result_text(pCtx, zOut as *const i8, -(1 as i32), unsafe {
                std::mem::transmute::<usize, Option<unsafe extern "C-unwind" fn(*mut ())>>(
                    -(1 as i32) as usize,
                )
            })
        };
        unsafe { sqlite3DbFree(db, zOut as *mut ()) };
    } else {
        rc = 7 as i32;
    }
    unsafe { sqlite3_free(zQuot as *mut ()) };
    return rc;
}

// /* Return result here */
// /* Rename context */
// /* SQL statement to edit */
// /* New token text */
// /* True to always quote token */
// /*
// ** Set all pEList->a[].fg.eEName fields in the expression-list to val.
// */
fn renameSetENames(mut pEList: *mut ExprList, mut val: i32) {
    0 as i32;
    if pEList != std::ptr::null_mut::<ExprList>() {
        let mut i: i32 = 0 as i32;
        i = 0 as i32;
        '__slate_break_1285: loop {
            if !(i < unsafe { (*pEList).nExpr }) {
                break;
            }
            0 as i32;
            unsafe {
                (*unsafe {
                    unsafe { std::ptr::addr_of_mut!((*pEList).a) as *mut ExprList_item }
                        .offset(i as isize)
                })
                .fg
                .__slate_bits_0
                .__set_eEName((val & (3 as i32)) as u32);
            }
            let __v1415: i32 = i;
            let __v1416: i32 = __v1415 + (1 as i32);
            i = __v1416;
        }
    }
}

// /*
// ** Resolve all symbols in the trigger at pParse->pNewTrigger, assuming
// ** it was read from the schema of database zDb. Return SQLITE_OK if
// ** successful. Otherwise, return an SQLite error code and leave an error
// ** message in the Parse object.
// */
fn renameResolveTrigger(mut pParse: *mut Parse) -> i32 {
    let mut db: *mut sqlite3 = unsafe { (*pParse).db };
    let mut pNew: *mut Trigger = unsafe { (*pParse).pNewTrigger };
    let mut pStep: *mut TriggerStep = unsafe { std::mem::zeroed() };
    let mut sNC: NameContext = unsafe { std::mem::zeroed() };
    let mut rc: i32 = 0 as i32;
    unsafe { memset(std::ptr::addr_of_mut!(sNC) as *mut (), 0 as i32, 56 as u64) };
    sNC.pParse = pParse;
    0 as i32;
    unsafe {
        (*pParse).pTriggerTab = unsafe {
            sqlite3FindTable(
                db,
                (unsafe { (*pNew).table }) as *const i8,
                (unsafe {
                    (*unsafe {
                        unsafe { (*db).aDb }.offset(
                            (unsafe { sqlite3SchemaToIndex(db, unsafe { (*pNew).pTabSchema }) })
                                as isize,
                        )
                    })
                    .zDbSName
                }) as *const i8,
            )
        };
    }
    unsafe {
        (*pParse).eTriggerOp = unsafe { (*pNew).op };
    }
    // /* ALWAYS() because if the table of the trigger does not exist, the
    //   ** error would have been hit before this point */
    if (unsafe { (*pParse).pTriggerTab }) != std::ptr::null_mut::<Table>() {
        rc = ((unsafe { sqlite3ViewGetColumnNames(pParse, unsafe { (*pParse).pTriggerTab }) })
            != (0 as i32)) as i32;
    }
    // /* Resolve symbols in WHEN clause */
    if rc == (0 as i32) && (unsafe { (*pNew).pWhen }) != std::ptr::null_mut::<Expr>() {
        rc = unsafe {
            sqlite3ResolveExprNames(std::ptr::addr_of_mut!(sNC), unsafe { (*pNew).pWhen })
        };
    }
    pStep = unsafe { (*pNew).step_list };
    '__slate_break_1286: while rc == (0 as i32) && pStep != std::ptr::null_mut::<TriggerStep>() {
        if (unsafe { (*pStep).pSelect }) != std::ptr::null_mut::<Select>() {
            unsafe {
                sqlite3SelectPrep(
                    pParse,
                    unsafe { (*pStep).pSelect },
                    std::ptr::addr_of_mut!(sNC),
                )
            };
            if (unsafe { (*pParse).nErr }) != (0 as i32) {
                rc = unsafe { (*pParse).rc };
            }
        }
        if rc == (0 as i32) && (unsafe { (*pStep).pSrc }) != std::ptr::null_mut::<SrcList>() {
            let mut pSrc: *mut SrcList = unsafe {
                sqlite3SrcListDup(db, (unsafe { (*pStep).pSrc }) as *const SrcList, 0 as i32)
            };
            if pSrc != std::ptr::null_mut::<SrcList>() {
                let mut pSel: *mut Select = unsafe {
                    sqlite3SelectNew(
                        pParse,
                        unsafe { (*pStep).pExprList },
                        pSrc,
                        std::ptr::null_mut::<Expr>(),
                        std::ptr::null_mut::<ExprList>(),
                        std::ptr::null_mut::<Expr>(),
                        std::ptr::null_mut::<ExprList>(),
                        (0 as i32) as u32,
                        std::ptr::null_mut::<Expr>(),
                    )
                };
                if pSel == std::ptr::null_mut::<Select>() {
                    unsafe {
                        (*pStep).pExprList = std::ptr::null_mut::<ExprList>();
                    }
                    pSrc = std::ptr::null_mut::<SrcList>();
                    rc = 7 as i32;
                } else {
                    // /* pStep->pExprList contains an expression-list used for an UPDATE
                    //           ** statement. So the a[].zEName values are the RHS of the
                    //           ** "<col> = <expr>" clauses of the UPDATE statement. So, before
                    //           ** running SelectPrep(), change all the eEName values in
                    //           ** pStep->pExprList to ENAME_SPAN (from their current value of
                    //           ** ENAME_NAME). This is to prevent any ids in ON() clauses that are
                    //           ** part of pSrc from being incorrectly resolved against the
                    //           ** a[].zEName values as if they were column aliases.  */
                    renameSetENames(unsafe { (*pStep).pExprList }, 1 as i32);
                    unsafe { sqlite3SelectPrep(pParse, pSel, std::ptr::null_mut::<NameContext>()) };
                    renameSetENames(unsafe { (*pStep).pExprList }, 0 as i32);
                    rc = if (unsafe { (*pParse).nErr }) != (0 as i32) {
                        1 as i32
                    } else {
                        0 as i32
                    };
                    0 as i32;
                    0 as i32;
                    if (unsafe { (*pStep).pExprList }) != std::ptr::null_mut::<ExprList>() {
                        unsafe {
                            (*pSel).pEList = std::ptr::null_mut::<ExprList>();
                        }
                    }
                    unsafe {
                        (*pSel).pSrc = std::ptr::null_mut::<SrcList>();
                    }
                    unsafe { sqlite3SelectDelete(db, pSel) };
                }
                if (unsafe { (*pStep).pSrc }) != std::ptr::null_mut::<SrcList>() {
                    let mut i: i32 = 0 as i32;
                    i = 0 as i32;
                    '__slate_break_1287: loop {
                        if !(i < unsafe { (*unsafe { (*pStep).pSrc }).nSrc } && rc == (0 as i32)) {
                            break;
                        }
                        let mut p: *mut SrcItem = unsafe {
                            unsafe {
                                std::ptr::addr_of_mut!((*unsafe { (*pStep).pSrc }).a)
                                    as *mut SrcItem
                            }
                            .offset(i as isize)
                        };
                        if ((unsafe { (*p).fg.__slate_bits_0.__get_isSubquery() }) as i32)
                            != (0 as i32)
                        {
                            0 as i32;
                            unsafe {
                                sqlite3SelectPrep(
                                    pParse,
                                    unsafe { (*unsafe { (*p).u4.pSubq }).pSelect },
                                    std::ptr::null_mut::<NameContext>(),
                                )
                            };
                        }
                        let __v1417: i32 = i;
                        let __v1418: i32 = __v1417 + (1 as i32);
                        i = __v1418;
                    }
                }
                if (unsafe { (*db).mallocFailed }) != (0 as u8) {
                    rc = 7 as i32;
                }
                sNC.pSrcList = pSrc;
                if rc == (0 as i32) && (unsafe { (*pStep).pWhere }) != std::ptr::null_mut::<Expr>()
                {
                    rc = unsafe {
                        sqlite3ResolveExprNames(std::ptr::addr_of_mut!(sNC), unsafe {
                            (*pStep).pWhere
                        })
                    };
                }
                if rc == (0 as i32) {
                    rc = unsafe {
                        sqlite3ResolveExprListNames(std::ptr::addr_of_mut!(sNC), unsafe {
                            (*pStep).pExprList
                        })
                    };
                }
                0 as i32;
                if (unsafe { (*pStep).pUpsert }) != std::ptr::null_mut::<Upsert>()
                    && rc == (0 as i32)
                {
                    let mut pUpsert: *mut Upsert = unsafe { (*pStep).pUpsert };
                    unsafe {
                        (*pUpsert).pUpsertSrc = pSrc;
                    }
                    unsafe {
                        sNC.uNC.pUpsert = pUpsert;
                    }
                    sNC.ncFlags = 512 as i32;
                    rc = unsafe {
                        sqlite3ResolveExprListNames(std::ptr::addr_of_mut!(sNC), unsafe {
                            (*pUpsert).pUpsertTarget
                        })
                    };
                    if rc == (0 as i32) {
                        let mut pUpsertSet: *mut ExprList = unsafe { (*pUpsert).pUpsertSet };
                        rc = unsafe {
                            sqlite3ResolveExprListNames(std::ptr::addr_of_mut!(sNC), pUpsertSet)
                        };
                    }
                    if rc == (0 as i32) {
                        rc = unsafe {
                            sqlite3ResolveExprNames(std::ptr::addr_of_mut!(sNC), unsafe {
                                (*pUpsert).pUpsertWhere
                            })
                        };
                    }
                    if rc == (0 as i32) {
                        rc = unsafe {
                            sqlite3ResolveExprNames(std::ptr::addr_of_mut!(sNC), unsafe {
                                (*pUpsert).pUpsertTargetWhere
                            })
                        };
                    }
                    sNC.ncFlags = 0 as i32;
                }
                sNC.pSrcList = std::ptr::null_mut::<SrcList>();
                unsafe { sqlite3SrcListDelete(db, pSrc) };
            } else {
                rc = 7 as i32;
            }
        }
        pStep = unsafe { (*pStep).pNext };
    }
    return rc;
}

// /*
// ** Invoke sqlite3WalkExpr() or sqlite3WalkSelect() on all Select or Expr
// ** objects that are part of the trigger passed as the second argument.
// */
fn renameWalkTrigger(mut pWalker: *mut Walker, mut pTrigger: *mut Trigger) {
    let mut pStep: *mut TriggerStep = unsafe { std::mem::zeroed() };
    // /* Find tokens to edit in WHEN clause */
    unsafe { sqlite3WalkExpr(pWalker, unsafe { (*pTrigger).pWhen }) };
    // /* Find tokens to edit in trigger steps */
    pStep = unsafe { (*pTrigger).step_list };
    '__slate_break_1288: while pStep != std::ptr::null_mut::<TriggerStep>() {
        unsafe { sqlite3WalkSelect(pWalker, unsafe { (*pStep).pSelect }) };
        unsafe { sqlite3WalkExpr(pWalker, unsafe { (*pStep).pWhere }) };
        unsafe { sqlite3WalkExprList(pWalker, unsafe { (*pStep).pExprList }) };
        if (unsafe { (*pStep).pUpsert }) != std::ptr::null_mut::<Upsert>() {
            let mut pUpsert: *mut Upsert = unsafe { (*pStep).pUpsert };
            unsafe { sqlite3WalkExprList(pWalker, unsafe { (*pUpsert).pUpsertTarget }) };
            unsafe { sqlite3WalkExprList(pWalker, unsafe { (*pUpsert).pUpsertSet }) };
            unsafe { sqlite3WalkExpr(pWalker, unsafe { (*pUpsert).pUpsertWhere }) };
            unsafe { sqlite3WalkExpr(pWalker, unsafe { (*pUpsert).pUpsertTargetWhere }) };
        }
        if (unsafe { (*pStep).pSrc }) != std::ptr::null_mut::<SrcList>() {
            let mut i: i32 = 0 as i32;
            let mut pSrc: *mut SrcList = unsafe { (*pStep).pSrc };
            i = 0 as i32;
            '__slate_break_1289: loop {
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
                    0 as i32;
                    unsafe {
                        sqlite3WalkSelect(pWalker, unsafe {
                            (*unsafe {
                                (*unsafe {
                                    unsafe { std::ptr::addr_of_mut!((*pSrc).a) as *mut SrcItem }
                                        .offset(i as isize)
                                })
                                .u4
                                .pSubq
                            })
                            .pSelect
                        })
                    };
                }
                let __v1419: i32 = i;
                let __v1420: i32 = __v1419 + (1 as i32);
                i = __v1420;
            }
        }
        pStep = unsafe { (*pStep).pNext };
    }
}

// /*
// ** Free the contents of Parse object (*pParse). Do not free the memory
// ** occupied by the Parse object itself.
// */
fn renameParseCleanup(mut pParse: *mut Parse) {
    let mut db: *mut sqlite3 = unsafe { (*pParse).db };
    let mut pIdx: *mut Index = unsafe { std::mem::zeroed() };
    if (unsafe { (*pParse).pVdbe }) != std::ptr::null_mut::<Vdbe>() {
        unsafe { sqlite3VdbeFinalize(unsafe { (*pParse).pVdbe }) };
    }
    unsafe { sqlite3DeleteTable(db, unsafe { (*pParse).pNewTable }) };
    '__slate_break_1290: loop {
        let __v1421: *mut Index = unsafe { (*pParse).pNewIndex };
        pIdx = __v1421;
        if !(__v1421 != std::ptr::null_mut::<Index>()) {
            break;
        }
        unsafe {
            (*pParse).pNewIndex = unsafe { (*pIdx).pNext };
        }
        unsafe { sqlite3FreeIndex(db, pIdx) };
    }
    unsafe { sqlite3DeleteTrigger(db, unsafe { (*pParse).pNewTrigger }) };
    unsafe { sqlite3DbFree(db, (unsafe { (*pParse).zErrMsg }) as *mut ()) };
    renameTokenFree(db, unsafe { (*pParse).pRename });
    unsafe { sqlite3ParseObjectReset(pParse) };
}

// /*
// ** SQL function:
// **
// **     sqlite_rename_column(SQL,TYPE,OBJ,DB,TABLE,COL,NEWNAME,QUOTE,TEMP)
// **
// **   0. zSql:     SQL statement to rewrite
// **   1. type:     Type of object ("table", "view" etc.)
// **   2. object:   Name of object
// **   3. Database: Database name (e.g. "main")
// **   4. Table:    Table name
// **   5. iCol:     Index of column to rename
// **   6. zNew:     New column name
// **   7. bQuote:   Non-zero if the new column name should be quoted.
// **   8. bTemp:    True if zSql comes from temp schema
// **
// ** Do a column rename operation on the CREATE statement given in zSql.
// ** The iCol-th column (left-most is 0) of table zTable is renamed from zCol
// ** into zNew.  The name should be quoted if bQuote is true.
// **
// ** This function is used internally by the ALTER TABLE RENAME COLUMN command.
// ** It is only accessible to SQL created using sqlite3NestedParse().  It is
// ** not reachable from ordinary SQL passed into sqlite3_prepare() unless the
// ** SQLITE_TESTCTRL_INTERNAL_FUNCTIONS test setting is enabled.
// */
#[unsafe(link_section = ".text.slate_distinct.alter.renameColumnFunc")]
extern "C-unwind" fn renameColumnFunc(
    mut context: *mut sqlite3_context,
    mut NotUsed: i32,
    mut argv: *mut *mut sqlite3_value,
) {
    let mut __slate_storage_1430: std::mem::MaybeUninit<bool> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1430: *mut bool = std::ptr::addr_of_mut!(__slate_storage_1430) as *mut bool;
    let mut __slate_storage_1424: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1424: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_1424) as *mut u32;
    let mut __slate_storage_1423: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1423: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_1423) as *mut u32;
    let mut __slate_storage_1422: std::mem::MaybeUninit<*mut Select> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1422: *mut *mut Select =
        std::ptr::addr_of_mut!(__slate_storage_1422) as *mut *mut Select;
    let mut __slate_storage_689: std::mem::MaybeUninit<*mut Select> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_689: *mut *mut Select =
        std::ptr::addr_of_mut!(__slate_storage_689) as *mut *mut Select;
    let mut __slate_storage_1428: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1428: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1428) as *mut i32;
    let mut __slate_storage_1427: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1427: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1427) as *mut i32;
    let mut __slate_storage_1429: std::mem::MaybeUninit<bool> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1429: *mut bool = std::ptr::addr_of_mut!(__slate_storage_1429) as *mut bool;
    let mut __slate_storage_1426: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1426: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1426) as *mut i32;
    let mut __slate_storage_1425: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1425: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1425) as *mut i32;
    let mut __slate_storage_692: std::mem::MaybeUninit<*mut Expr> = std::mem::MaybeUninit::uninit();
    let __slate_slot_692: *mut *mut Expr =
        std::ptr::addr_of_mut!(__slate_storage_692) as *mut *mut Expr;
    let mut __slate_storage_691: std::mem::MaybeUninit<*mut FKey> = std::mem::MaybeUninit::uninit();
    let __slate_slot_691: *mut *mut FKey =
        std::ptr::addr_of_mut!(__slate_storage_691) as *mut *mut FKey;
    let mut __slate_storage_690: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_690: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_690) as *mut i32;
    let mut __slate_storage_695: std::mem::MaybeUninit<*mut ExprList> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_695: *mut *mut ExprList =
        std::ptr::addr_of_mut!(__slate_storage_695) as *mut *mut ExprList;
    let mut __slate_storage_694: std::mem::MaybeUninit<*mut Table> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_694: *mut *mut Table =
        std::ptr::addr_of_mut!(__slate_storage_694) as *mut *mut Table;
    let mut __slate_storage_693: std::mem::MaybeUninit<*mut TriggerStep> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_693: *mut *mut TriggerStep =
        std::ptr::addr_of_mut!(__slate_storage_693) as *mut *mut TriggerStep;
    let mut __slate_storage_688: std::mem::MaybeUninit<
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
    let __slate_slot_688: *mut Option<
        unsafe extern "C-unwind" fn(
            *mut (),
            i32,
            *const i8,
            *const i8,
            *const i8,
            *const i8,
        ) -> i32,
    > = std::ptr::addr_of_mut!(__slate_storage_688)
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
    let mut __slate_storage_687: std::mem::MaybeUninit<*mut Table> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_687: *mut *mut Table =
        std::ptr::addr_of_mut!(__slate_storage_687) as *mut *mut Table;
    let mut __slate_storage_686: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_686: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_686) as *mut i32;
    let mut __slate_storage_685: std::mem::MaybeUninit<*mut Index> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_685: *mut *mut Index =
        std::ptr::addr_of_mut!(__slate_storage_685) as *mut *mut Index;
    let mut __slate_storage_684: std::mem::MaybeUninit<Walker> = std::mem::MaybeUninit::uninit();
    let __slate_slot_684: *mut Walker = std::ptr::addr_of_mut!(__slate_storage_684) as *mut Walker;
    let mut __slate_storage_683: std::mem::MaybeUninit<Parse> = std::mem::MaybeUninit::uninit();
    let __slate_slot_683: *mut Parse = std::ptr::addr_of_mut!(__slate_storage_683) as *mut Parse;
    let mut __slate_storage_682: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_682: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_682) as *mut i32;
    let mut __slate_storage_681: std::mem::MaybeUninit<*const i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_681: *mut *const i8 =
        std::ptr::addr_of_mut!(__slate_storage_681) as *mut *const i8;
    let mut __slate_storage_680: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_680: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_680) as *mut i32;
    let mut __slate_storage_679: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_679: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_679) as *mut i32;
    let mut __slate_storage_678: std::mem::MaybeUninit<*const i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_678: *mut *const i8 =
        std::ptr::addr_of_mut!(__slate_storage_678) as *mut *const i8;
    let mut __slate_storage_677: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_677: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_677) as *mut i32;
    let mut __slate_storage_676: std::mem::MaybeUninit<*const i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_676: *mut *const i8 =
        std::ptr::addr_of_mut!(__slate_storage_676) as *mut *const i8;
    let mut __slate_storage_675: std::mem::MaybeUninit<*const i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_675: *mut *const i8 =
        std::ptr::addr_of_mut!(__slate_storage_675) as *mut *const i8;
    let mut __slate_storage_674: std::mem::MaybeUninit<*const i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_674: *mut *const i8 =
        std::ptr::addr_of_mut!(__slate_storage_674) as *mut *const i8;
    let mut __slate_storage_673: std::mem::MaybeUninit<RenameCtx> = std::mem::MaybeUninit::uninit();
    let __slate_slot_673: *mut RenameCtx =
        std::ptr::addr_of_mut!(__slate_storage_673) as *mut RenameCtx;
    let mut __slate_storage_672: std::mem::MaybeUninit<*mut sqlite3> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_672: *mut *mut sqlite3 =
        std::ptr::addr_of_mut!(__slate_storage_672) as *mut *mut sqlite3;
    unsafe {
        std::ptr::write(__slate_slot_672, unsafe {
            sqlite3_context_db_handle(context)
        });
        std::ptr::write(
            __slate_slot_674,
            (unsafe { sqlite3_value_text(unsafe { *unsafe { argv.offset((0 as i32) as isize) } }) })
                as *const i8,
        );
        std::ptr::write(
            __slate_slot_675,
            (unsafe { sqlite3_value_text(unsafe { *unsafe { argv.offset((3 as i32) as isize) } }) })
                as *const i8,
        );
        std::ptr::write(
            __slate_slot_676,
            (unsafe { sqlite3_value_text(unsafe { *unsafe { argv.offset((4 as i32) as isize) } }) })
                as *const i8,
        );
        std::ptr::write(__slate_slot_677, unsafe {
            sqlite3_value_int(unsafe { *unsafe { argv.offset((5 as i32) as isize) } })
        });
        std::ptr::write(
            __slate_slot_678,
            (unsafe { sqlite3_value_text(unsafe { *unsafe { argv.offset((6 as i32) as isize) } }) })
                as *const i8,
        );
        std::ptr::write(__slate_slot_679, unsafe {
            sqlite3_value_int(unsafe { *unsafe { argv.offset((7 as i32) as isize) } })
        });
        std::ptr::write(__slate_slot_680, unsafe {
            sqlite3_value_int(unsafe { *unsafe { argv.offset((8 as i32) as isize) } })
        });
        std::ptr::write(__slate_slot_688, unsafe { (*(*__slate_slot_672)).xAuth });
        NotUsed;
        if *__slate_slot_674 == std::ptr::null::<i8>() {
            return;
        } else {
            if *__slate_slot_676 == std::ptr::null::<i8>() {
                return;
            } else {
                if *__slate_slot_678 == std::ptr::null::<i8>() {
                    return;
                } else {
                    if *__slate_slot_677 < (0 as i32) {
                        return;
                    } else {
                        unsafe { sqlite3BtreeEnterAll(*__slate_slot_672) };
                        *__slate_slot_687 = unsafe {
                            sqlite3FindTable(
                                *__slate_slot_672,
                                *__slate_slot_676,
                                *__slate_slot_675,
                            )
                        };
                        if *__slate_slot_687 == std::ptr::null_mut::<Table>()
                            || *__slate_slot_677
                                >= ((unsafe { (*(*__slate_slot_687)).nCol }) as i32)
                        {
                            unsafe { sqlite3BtreeLeaveAll(*__slate_slot_672) };
                            return;
                        } else {
                            '__join_9: {
                                *__slate_slot_681 = (unsafe {
                                    (*unsafe {
                                        unsafe { (*(*__slate_slot_687)).aCol }
                                            .offset(*__slate_slot_677 as isize)
                                    })
                                    .zCnName
                                }) as *const i8;
                                unsafe {
                                    memset(
                                        std::ptr::addr_of_mut!(*__slate_slot_673) as *mut (),
                                        0 as i32,
                                        32 as u64,
                                    )
                                };
                                (*__slate_slot_673).iCol = if *__slate_slot_677
                                    == ((unsafe { (*(*__slate_slot_687)).iPKey }) as i32)
                                {
                                    -(1 as i32)
                                } else {
                                    *__slate_slot_677
                                };
                                unsafe {
                                    (*(*__slate_slot_672)).xAuth = None;
                                }
                                *__slate_slot_682 = renameParseSql(
                                    std::ptr::addr_of_mut!(*__slate_slot_683),
                                    *__slate_slot_675,
                                    *__slate_slot_672,
                                    *__slate_slot_674,
                                    *__slate_slot_680,
                                );
                                // /* Find tokens that need to be replaced. */
                                unsafe {
                                    memset(
                                        std::ptr::addr_of_mut!(*__slate_slot_684) as *mut (),
                                        0 as i32,
                                        48 as u64,
                                    )
                                };
                                (*__slate_slot_684).pParse =
                                    std::ptr::addr_of_mut!(*__slate_slot_683);
                                (*__slate_slot_684).xExprCallback = Some(renameColumnExprCb);
                                (*__slate_slot_684).xSelectCallback = Some(renameColumnSelectCb);
                                unsafe {
                                    (*__slate_slot_684).u.pRename =
                                        std::ptr::addr_of_mut!(*__slate_slot_673);
                                }
                                (*__slate_slot_673).pTab = *__slate_slot_687;
                                if *__slate_slot_682 != (0 as i32) {
                                } else {
                                    '__join_10: {
                                        if (*__slate_slot_683).pNewTable
                                            != std::ptr::null_mut::<Table>()
                                        {
                                            if (((unsafe {
                                                (*(*__slate_slot_683).pNewTable).eTabType
                                            })
                                                as u32)
                                                as i32)
                                                == (2 as i32)
                                            {
                                                std::ptr::write(__slate_slot_689, unsafe {
                                                    (*(*__slate_slot_683).pNewTable).u.view.pSelect
                                                });
                                                std::ptr::write(
                                                    __slate_slot_1422,
                                                    *__slate_slot_689,
                                                );
                                                std::ptr::write(__slate_slot_1423, unsafe {
                                                    (*(*__slate_slot_1422)).selFlags
                                                });
                                                std::ptr::write(
                                                    __slate_slot_1424,
                                                    *__slate_slot_1423 & !((2097152 as i32) as u32),
                                                );
                                                unsafe {
                                                    (*(*__slate_slot_1422)).selFlags =
                                                        *__slate_slot_1424;
                                                }
                                                (*__slate_slot_683).rc = 0 as i32;
                                                unsafe {
                                                    sqlite3SelectPrep(
                                                        std::ptr::addr_of_mut!(*__slate_slot_683),
                                                        *__slate_slot_689,
                                                        std::ptr::null_mut::<NameContext>(),
                                                    )
                                                };
                                                *__slate_slot_682 = if (unsafe {
                                                    (*(*__slate_slot_672)).mallocFailed
                                                }) != (0 as u8)
                                                {
                                                    7 as i32
                                                } else {
                                                    (*__slate_slot_683).rc
                                                };
                                                if *__slate_slot_682 == (0 as i32) {
                                                    unsafe {
                                                        sqlite3WalkSelect(
                                                            std::ptr::addr_of_mut!(
                                                                *__slate_slot_684
                                                            ),
                                                            *__slate_slot_689,
                                                        )
                                                    };
                                                }
                                                if *__slate_slot_682 != (0 as i32) {
                                                    break '__join_9;
                                                }
                                            } else {
                                                if (((unsafe {
                                                    (*(*__slate_slot_683).pNewTable).eTabType
                                                })
                                                    as u32)
                                                    as i32)
                                                    == (0 as i32)
                                                {
                                                    '__join_26: {
                                                        // /* A regular table */
                                                        std::ptr::write(__slate_slot_690, unsafe {
                                                            sqlite3_stricmp(
                                                                *__slate_slot_676,
                                                                (unsafe {
                                                                    (*(*__slate_slot_683).pNewTable)
                                                                        .zName
                                                                })
                                                                    as *const i8,
                                                            )
                                                        });
                                                        (*__slate_slot_673).pTab =
                                                            (*__slate_slot_683).pNewTable;
                                                        if *__slate_slot_690 == (0 as i32) {
                                                            if *__slate_slot_677
                                                                < ((unsafe {
                                                                    (*(*__slate_slot_683).pNewTable)
                                                                        .nCol
                                                                })
                                                                    as i32)
                                                            {
                                                                renameTokenFind(
                                                                    std::ptr::addr_of_mut!(
                                                                        *__slate_slot_683
                                                                    ),
                                                                    std::ptr::addr_of_mut!(
                                                                        *__slate_slot_673
                                                                    ),
                                                                    ((unsafe {
                                                                        (*unsafe { unsafe { (*(*__slate_slot_683).pNewTable).aCol }.offset(*__slate_slot_677 as isize) }).zCnName
                                                                    })
                                                                        as *mut ())
                                                                        as *const (),
                                                                );
                                                            }
                                                            if (*__slate_slot_673).iCol < (0 as i32)
                                                            {
                                                                renameTokenFind(
                                                                    std::ptr::addr_of_mut!(
                                                                        *__slate_slot_683
                                                                    ),
                                                                    std::ptr::addr_of_mut!(
                                                                        *__slate_slot_673
                                                                    ),
                                                                    ((unsafe {
                                                                        std::ptr::addr_of_mut!(
                                                                            (*(*__slate_slot_683)
                                                                                .pNewTable)
                                                                                .iPKey
                                                                        )
                                                                    })
                                                                        as *mut ())
                                                                        as *const (),
                                                                );
                                                            }
                                                            unsafe {
                                                                sqlite3WalkExprList(
                                                                    std::ptr::addr_of_mut!(
                                                                        *__slate_slot_684
                                                                    ),
                                                                    unsafe {
                                                                        (*(*__slate_slot_683)
                                                                            .pNewTable)
                                                                            .pCheck
                                                                    },
                                                                )
                                                            };
                                                            *__slate_slot_685 = unsafe {
                                                                (*(*__slate_slot_683).pNewTable)
                                                                    .pIndex
                                                            };
                                                            loop {
                                                                if *__slate_slot_685
                                                                    != std::ptr::null_mut::<Index>()
                                                                {
                                                                    unsafe {
                                                                        sqlite3WalkExprList(
                                                                            std::ptr::addr_of_mut!(
                                                                                *__slate_slot_684
                                                                            ),
                                                                            unsafe {
                                                                                (*(*__slate_slot_685)).aColExpr
                                                                            },
                                                                        )
                                                                    };
                                                                    *__slate_slot_685 = unsafe {
                                                                        (*(*__slate_slot_685)).pNext
                                                                    };
                                                                } else {
                                                                    break;
                                                                }
                                                            }
                                                            *__slate_slot_685 =
                                                                (*__slate_slot_683).pNewIndex;
                                                            loop {
                                                                if *__slate_slot_685
                                                                    != std::ptr::null_mut::<Index>()
                                                                {
                                                                    unsafe {
                                                                        sqlite3WalkExprList(
                                                                            std::ptr::addr_of_mut!(
                                                                                *__slate_slot_684
                                                                            ),
                                                                            unsafe {
                                                                                (*(*__slate_slot_685)).aColExpr
                                                                            },
                                                                        )
                                                                    };
                                                                    *__slate_slot_685 = unsafe {
                                                                        (*(*__slate_slot_685)).pNext
                                                                    };
                                                                } else {
                                                                    break;
                                                                }
                                                            }
                                                            *__slate_slot_686 = 0 as i32;
                                                            loop {
                                                                if *__slate_slot_686
                                                                    < ((unsafe {
                                                                        (*(*__slate_slot_683)
                                                                            .pNewTable)
                                                                            .nCol
                                                                    })
                                                                        as i32)
                                                                {
                                                                    std::ptr::write(
                                                                        __slate_slot_692,
                                                                        unsafe {
                                                                            sqlite3ColumnExpr(
                                                                                (*__slate_slot_683)
                                                                                    .pNewTable,
                                                                                unsafe {
                                                                                    unsafe { (*(*__slate_slot_683).pNewTable).aCol }.offset(*__slate_slot_686 as isize)
                                                                                },
                                                                            )
                                                                        },
                                                                    );
                                                                    unsafe {
                                                                        sqlite3WalkExpr(
                                                                            std::ptr::addr_of_mut!(
                                                                                *__slate_slot_684
                                                                            ),
                                                                            *__slate_slot_692,
                                                                        )
                                                                    };
                                                                    std::ptr::write(
                                                                        __slate_slot_1425,
                                                                        *__slate_slot_686,
                                                                    );
                                                                    std::ptr::write(
                                                                        __slate_slot_1426,
                                                                        *__slate_slot_1425
                                                                            + (1 as i32),
                                                                    );
                                                                    *__slate_slot_686 =
                                                                        *__slate_slot_1426;
                                                                } else {
                                                                    break '__join_26;
                                                                }
                                                            }
                                                        }
                                                    }
                                                    0 as i32;
                                                    *__slate_slot_691 = unsafe {
                                                        (*(*__slate_slot_683).pNewTable).u.tab.pFKey
                                                    };
                                                    loop {
                                                        if *__slate_slot_691
                                                            != std::ptr::null_mut::<FKey>()
                                                        {
                                                            *__slate_slot_686 = 0 as i32;
                                                            loop {
                                                                if *__slate_slot_686
                                                                    < unsafe {
                                                                        (*(*__slate_slot_691)).nCol
                                                                    }
                                                                {
                                                                    if *__slate_slot_690
                                                                        == (0 as i32)
                                                                        && (unsafe {
                                                                            (*unsafe { unsafe { std::ptr::addr_of_mut!((*(*__slate_slot_691)).aCol) as *mut sColMap }.offset(*__slate_slot_686 as isize) }).iFrom
                                                                        }) == *__slate_slot_677
                                                                    {
                                                                        renameTokenFind(
                                                                            std::ptr::addr_of_mut!(
                                                                                *__slate_slot_683
                                                                            ),
                                                                            std::ptr::addr_of_mut!(
                                                                                *__slate_slot_673
                                                                            ),
                                                                            ((unsafe {
                                                                                unsafe { std::ptr::addr_of_mut!((*(*__slate_slot_691)).aCol) as *mut sColMap }.offset(*__slate_slot_686 as isize)
                                                                            })
                                                                                as *mut ())
                                                                                as *const (),
                                                                        );
                                                                    }
                                                                    if (0 as i32)
                                                                        == unsafe {
                                                                            sqlite3_stricmp(
                                                                                (unsafe {
                                                                                    (*(*__slate_slot_691)).zTo
                                                                                })
                                                                                    as *const i8,
                                                                                *__slate_slot_676,
                                                                            )
                                                                        }
                                                                    {
                                                                        *__slate_slot_1429 = (0
                                                                            as i32)
                                                                            == unsafe {
                                                                                sqlite3_stricmp((unsafe { (*unsafe { unsafe { std::ptr::addr_of_mut!((*(*__slate_slot_691)).aCol) as *mut sColMap }.offset(*__slate_slot_686 as isize) }).zCol }) as *const i8, *__slate_slot_681)
                                                                            };
                                                                    } else {
                                                                        *__slate_slot_1429 =
                                                                            false as bool;
                                                                    }
                                                                    if *__slate_slot_1429 {
                                                                        renameTokenFind(
                                                                            std::ptr::addr_of_mut!(
                                                                                *__slate_slot_683
                                                                            ),
                                                                            std::ptr::addr_of_mut!(
                                                                                *__slate_slot_673
                                                                            ),
                                                                            ((unsafe {
                                                                                (*unsafe { unsafe { std::ptr::addr_of_mut!((*(*__slate_slot_691)).aCol) as *mut sColMap }.offset(*__slate_slot_686 as isize) }).zCol
                                                                            })
                                                                                as *mut ())
                                                                                as *const (),
                                                                        );
                                                                    }
                                                                    std::ptr::write(
                                                                        __slate_slot_1427,
                                                                        *__slate_slot_686,
                                                                    );
                                                                    std::ptr::write(
                                                                        __slate_slot_1428,
                                                                        *__slate_slot_1427
                                                                            + (1 as i32),
                                                                    );
                                                                    *__slate_slot_686 =
                                                                        *__slate_slot_1428;
                                                                } else {
                                                                    break;
                                                                }
                                                            }
                                                            *__slate_slot_691 = unsafe {
                                                                (*(*__slate_slot_691)).pNextFrom
                                                            };
                                                        } else {
                                                            break '__join_10;
                                                        }
                                                    }
                                                }
                                            }
                                        } else {
                                            if (*__slate_slot_683).pNewIndex
                                                != std::ptr::null_mut::<Index>()
                                            {
                                                unsafe {
                                                    sqlite3WalkExprList(
                                                        std::ptr::addr_of_mut!(*__slate_slot_684),
                                                        unsafe {
                                                            (*(*__slate_slot_683).pNewIndex)
                                                                .aColExpr
                                                        },
                                                    )
                                                };
                                                unsafe {
                                                    sqlite3WalkExpr(
                                                        std::ptr::addr_of_mut!(*__slate_slot_684),
                                                        unsafe {
                                                            (*(*__slate_slot_683).pNewIndex)
                                                                .pPartIdxWhere
                                                        },
                                                    )
                                                };
                                            } else {
                                                // /* A trigger */
                                                *__slate_slot_682 = renameResolveTrigger(
                                                    std::ptr::addr_of_mut!(*__slate_slot_683),
                                                );
                                                if *__slate_slot_682 != (0 as i32) {
                                                    break '__join_9;
                                                } else {
                                                    *__slate_slot_693 = unsafe {
                                                        (*(*__slate_slot_683).pNewTrigger).step_list
                                                    };
                                                    loop {
                                                        if *__slate_slot_693
                                                            != std::ptr::null_mut::<TriggerStep>()
                                                        {
                                                            if (unsafe {
                                                                (*(*__slate_slot_693)).pSrc
                                                            }) != std::ptr::null_mut::<SrcList>()
                                                            {
                                                                std::ptr::write(
                                                                    __slate_slot_694,
                                                                    unsafe {
                                                                        sqlite3LocateTableItem(
                                                                            std::ptr::addr_of_mut!(
                                                                                *__slate_slot_683
                                                                            ),
                                                                            (0 as i32) as u32,
                                                                            unsafe {
                                                                                unsafe { std::ptr::addr_of_mut!((*unsafe { (*(*__slate_slot_693)).pSrc }).a) as *mut SrcItem }.offset((0 as i32) as isize)
                                                                            },
                                                                        )
                                                                    },
                                                                );
                                                                if *__slate_slot_694
                                                                    == *__slate_slot_687
                                                                {
                                                                    if (unsafe {
                                                                        (*(*__slate_slot_693))
                                                                            .pUpsert
                                                                    }) != std::ptr::null_mut::<
                                                                        Upsert,
                                                                    >(
                                                                    ) {
                                                                        std::ptr::write(
                                                                            __slate_slot_695,
                                                                            unsafe {
                                                                                (*unsafe { (*(*__slate_slot_693)).pUpsert }).pUpsertSet
                                                                            },
                                                                        );
                                                                        renameColumnElistNames(
                                                                            std::ptr::addr_of_mut!(
                                                                                *__slate_slot_683
                                                                            ),
                                                                            std::ptr::addr_of_mut!(
                                                                                *__slate_slot_673
                                                                            ),
                                                                            *__slate_slot_695
                                                                                as *const ExprList,
                                                                            *__slate_slot_681,
                                                                        );
                                                                    }
                                                                    renameColumnIdlistNames(
                                                                        std::ptr::addr_of_mut!(
                                                                            *__slate_slot_683
                                                                        ),
                                                                        std::ptr::addr_of_mut!(
                                                                            *__slate_slot_673
                                                                        ),
                                                                        (unsafe {
                                                                            (*(*__slate_slot_693))
                                                                                .pIdList
                                                                        })
                                                                            as *const IdList,
                                                                        *__slate_slot_681,
                                                                    );
                                                                    renameColumnElistNames(
                                                                        std::ptr::addr_of_mut!(
                                                                            *__slate_slot_683
                                                                        ),
                                                                        std::ptr::addr_of_mut!(
                                                                            *__slate_slot_673
                                                                        ),
                                                                        (unsafe {
                                                                            (*(*__slate_slot_693))
                                                                                .pExprList
                                                                        })
                                                                            as *const ExprList,
                                                                        *__slate_slot_681,
                                                                    );
                                                                }
                                                            }
                                                            *__slate_slot_693 = unsafe {
                                                                (*(*__slate_slot_693)).pNext
                                                            };
                                                        } else {
                                                            break;
                                                        }
                                                    }
                                                    // /* Find tokens to edit in UPDATE OF clause */
                                                    if (*__slate_slot_683).pTriggerTab
                                                        == *__slate_slot_687
                                                    {
                                                        renameColumnIdlistNames(
                                                            std::ptr::addr_of_mut!(
                                                                *__slate_slot_683
                                                            ),
                                                            std::ptr::addr_of_mut!(
                                                                *__slate_slot_673
                                                            ),
                                                            (unsafe {
                                                                (*(*__slate_slot_683).pNewTrigger)
                                                                    .pColumns
                                                            })
                                                                as *const IdList,
                                                            *__slate_slot_681,
                                                        );
                                                    }
                                                    // /* Find tokens to edit in various expressions and selects */
                                                    renameWalkTrigger(
                                                        std::ptr::addr_of_mut!(*__slate_slot_684),
                                                        (*__slate_slot_683).pNewTrigger,
                                                    );
                                                }
                                            }
                                        }
                                    }
                                    0 as i32;
                                    *__slate_slot_682 = renameEditSql(
                                        context,
                                        std::ptr::addr_of_mut!(*__slate_slot_673),
                                        *__slate_slot_674,
                                        *__slate_slot_678,
                                        *__slate_slot_679,
                                    );
                                }
                            }
                            if *__slate_slot_682 != (0 as i32) {
                                if *__slate_slot_682 == (1 as i32) {
                                    *__slate_slot_1430 =
                                        (unsafe { sqlite3WritableSchema(*__slate_slot_672) })
                                            != (0 as i32);
                                } else {
                                    *__slate_slot_1430 = false as bool;
                                }
                                if *__slate_slot_1430 {
                                    unsafe {
                                        sqlite3_result_value(context, unsafe {
                                            *unsafe { argv.offset((0 as i32) as isize) }
                                        })
                                    };
                                } else {
                                    if (*__slate_slot_683).zErrMsg != std::ptr::null_mut::<i8>() {
                                        renameColumnParseError(
                                            context,
                                            (b"\0".as_ptr() as *mut i8) as *const i8,
                                            unsafe { *unsafe { argv.offset((1 as i32) as isize) } },
                                            unsafe { *unsafe { argv.offset((2 as i32) as isize) } },
                                            std::ptr::addr_of_mut!(*__slate_slot_683),
                                        );
                                    } else {
                                        unsafe {
                                            sqlite3_result_error_code(context, *__slate_slot_682)
                                        };
                                    }
                                }
                            }
                            renameParseCleanup(std::ptr::addr_of_mut!(*__slate_slot_683));
                            renameTokenFree(*__slate_slot_672, (*__slate_slot_673).pList);
                            unsafe {
                                (*(*__slate_slot_672)).xAuth = *__slate_slot_688;
                            }
                            unsafe { sqlite3BtreeLeaveAll(*__slate_slot_672) };
                        }
                    }
                }
            }
        }
    }
}

// /*
// ** Walker expression callback used by "RENAME TABLE".
// */
#[unsafe(link_section = ".text.slate_distinct.alter.renameTableExprCb")]
extern "C-unwind" fn renameTableExprCb(mut pWalker: *mut Walker, mut pExpr: *mut Expr) -> i32 {
    let mut p: *mut RenameCtx = unsafe { (*pWalker).u.pRename };
    if (((unsafe { (*pExpr).op }) as u32) as i32) == (168 as i32)
        && (unsafe { (*pExpr).flags }) & (((16777216 as i32) | (33554432 as i32)) as u32)
            == ((0 as i32) as u32)
        && (unsafe { (*p).pTab }) == unsafe { (*pExpr).y.pTab }
    {
        renameTokenFind(
            unsafe { (*pWalker).pParse },
            p,
            ((unsafe { std::ptr::addr_of_mut!((*pExpr).y.pTab) }) as *mut ()) as *const (),
        );
    }
    return 0 as i32;
}

// /*
// ** Walker select callback used by "RENAME TABLE".
// */
#[unsafe(link_section = ".text.slate_distinct.alter.renameTableSelectCb")]
extern "C-unwind" fn renameTableSelectCb(
    mut pWalker: *mut Walker,
    mut pSelect: *mut Select,
) -> i32 {
    let mut i: i32 = 0 as i32;
    let mut p: *mut RenameCtx = unsafe { (*pWalker).u.pRename };
    let mut pSrc: *mut SrcList = unsafe { (*pSelect).pSrc };
    if (unsafe { (*pSelect).selFlags }) & (((2097152 as i32) | (67108864 as i32)) as u32)
        != (0 as u32)
    {
        {}
        {}
        return 1 as i32;
    }
    if pSrc == std::ptr::null_mut::<SrcList>() {
        0 as i32;
        return 2 as i32;
    }
    i = 0 as i32;
    '__slate_break_1298: loop {
        if !(i < unsafe { (*pSrc).nSrc }) {
            break;
        }
        let mut pItem: *mut SrcItem = unsafe {
            unsafe { std::ptr::addr_of_mut!((*pSrc).a) as *mut SrcItem }.offset(i as isize)
        };
        if (unsafe { (*pItem).pSTab }) == unsafe { (*p).pTab } {
            renameTokenFind(
                unsafe { (*pWalker).pParse },
                p,
                (unsafe { (*pItem).zName }) as *const (),
            );
        }
        let __v1431: i32 = i;
        let __v1432: i32 = __v1431 + (1 as i32);
        i = __v1432;
    }
    renameWalkWith(pWalker, pSelect);
    return 0 as i32;
}

// /*
// ** This C function implements an SQL user function that is used by SQL code
// ** generated by the ALTER TABLE ... RENAME command to modify the definition
// ** of any foreign key constraints that use the table being renamed as the
// ** parent table. It is passed three arguments:
// **
// **   0: The database containing the table being renamed.
// **   1. type:     Type of object ("table", "view" etc.)
// **   2. object:   Name of object
// **   3: The complete text of the schema statement being modified,
// **   4: The old name of the table being renamed, and
// **   5: The new name of the table being renamed.
// **   6: True if the schema statement comes from the temp db.
// **
// ** It returns the new schema statement. For example:
// **
// ** sqlite_rename_table('main', 'CREATE TABLE t1(a REFERENCES t2)','t2','t3',0)
// **       -> 'CREATE TABLE t1(a REFERENCES t3)'
// */
#[unsafe(link_section = ".text.slate_distinct.alter.renameTableFunc")]
extern "C-unwind" fn renameTableFunc(
    mut context: *mut sqlite3_context,
    mut NotUsed: i32,
    mut argv: *mut *mut sqlite3_value,
) {
    let mut db: *mut sqlite3 = unsafe { sqlite3_context_db_handle(context) };
    let mut zDb: *const i8 =
        (unsafe { sqlite3_value_text(unsafe { *unsafe { argv.offset((0 as i32) as isize) } }) })
            as *const i8;
    let mut zInput: *const i8 =
        (unsafe { sqlite3_value_text(unsafe { *unsafe { argv.offset((3 as i32) as isize) } }) })
            as *const i8;
    let mut zOld: *const i8 =
        (unsafe { sqlite3_value_text(unsafe { *unsafe { argv.offset((4 as i32) as isize) } }) })
            as *const i8;
    let mut zNew: *const i8 =
        (unsafe { sqlite3_value_text(unsafe { *unsafe { argv.offset((5 as i32) as isize) } }) })
            as *const i8;
    let mut bTemp: i32 =
        unsafe { sqlite3_value_int(unsafe { *unsafe { argv.offset((6 as i32) as isize) } }) };
    NotUsed;
    if zInput != std::ptr::null::<i8>()
        && zOld != std::ptr::null::<i8>()
        && zNew != std::ptr::null::<i8>()
    {
        let mut sParse: Parse = unsafe { std::mem::zeroed() };
        let mut rc: i32 = 0 as i32;
        let mut bQuote: i32 = 1 as i32;
        let mut sCtx: RenameCtx = unsafe { std::mem::zeroed() };
        let mut sWalker: Walker = unsafe { std::mem::zeroed() };
        let mut xAuth: Option<
            unsafe extern "C-unwind" fn(
                *mut (),
                i32,
                *const i8,
                *const i8,
                *const i8,
                *const i8,
            ) -> i32,
        > = unsafe { (*db).xAuth };
        unsafe {
            (*db).xAuth = None;
        }
        unsafe { sqlite3BtreeEnterAll(db) };
        unsafe { memset(std::ptr::addr_of_mut!(sCtx) as *mut (), 0 as i32, 32 as u64) };
        sCtx.pTab = unsafe { sqlite3FindTable(db, zOld, zDb) };
        unsafe {
            memset(
                std::ptr::addr_of_mut!(sWalker) as *mut (),
                0 as i32,
                48 as u64,
            )
        };
        sWalker.pParse = std::ptr::addr_of_mut!(sParse);
        sWalker.xExprCallback = Some(renameTableExprCb);
        sWalker.xSelectCallback = Some(renameTableSelectCb);
        unsafe {
            sWalker.u.pRename = std::ptr::addr_of_mut!(sCtx);
        }
        rc = renameParseSql(std::ptr::addr_of_mut!(sParse), zDb, db, zInput, bTemp);
        if rc == (0 as i32) {
            let mut isLegacy: i32 =
                (((unsafe { (*db).flags }) & (((67108864 as i32) as i64) as u64)) as u32) as i32;
            if sParse.pNewTable != std::ptr::null_mut::<Table>() {
                let mut pTab: *mut Table = sParse.pNewTable;
                if (((unsafe { (*pTab).eTabType }) as u32) as i32) == (2 as i32) {
                    if isLegacy == (0 as i32) {
                        let mut pSelect: *mut Select = unsafe { (*pTab).u.view.pSelect };
                        let mut sNC: NameContext = unsafe { std::mem::zeroed() };
                        unsafe {
                            memset(std::ptr::addr_of_mut!(sNC) as *mut (), 0 as i32, 56 as u64)
                        };
                        sNC.pParse = std::ptr::addr_of_mut!(sParse);
                        0 as i32;
                        let __v1433: *mut Select = pSelect;
                        let __v1434: u32 = unsafe { (*__v1433).selFlags };
                        let __v1435: u32 = __v1434 & !((2097152 as i32) as u32);
                        unsafe {
                            (*__v1433).selFlags = __v1435;
                        }
                        unsafe {
                            sqlite3SelectPrep(
                                std::ptr::addr_of_mut!(sParse),
                                unsafe { (*pTab).u.view.pSelect },
                                std::ptr::addr_of_mut!(sNC),
                            )
                        };
                        if sParse.nErr != (0 as i32) {
                            rc = sParse.rc;
                        } else {
                            unsafe {
                                sqlite3WalkSelect(std::ptr::addr_of_mut!(sWalker), unsafe {
                                    (*pTab).u.view.pSelect
                                })
                            };
                        }
                    }
                } else {
                    // /* Modify any FK definitions to point to the new table. */
                    if (isLegacy == (0 as i32)
                        || (unsafe { (*db).flags }) & (((16384 as i32) as i64) as u64)
                            != (0 as u64))
                        && !((((unsafe { (*pTab).eTabType }) as u32) as i32) == (1 as i32))
                    {
                        let mut pFKey: *mut FKey = unsafe { std::mem::zeroed() };
                        0 as i32;
                        pFKey = unsafe { (*pTab).u.tab.pFKey };
                        '__slate_break_1299: while pFKey != std::ptr::null_mut::<FKey>() {
                            if (unsafe {
                                sqlite3_stricmp((unsafe { (*pFKey).zTo }) as *const i8, zOld)
                            }) == (0 as i32)
                            {
                                renameTokenFind(
                                    std::ptr::addr_of_mut!(sParse),
                                    std::ptr::addr_of_mut!(sCtx),
                                    ((unsafe { (*pFKey).zTo }) as *mut ()) as *const (),
                                );
                            }
                            pFKey = unsafe { (*pFKey).pNextFrom };
                        }
                    }
                    // /* If this is the table being altered, fix any table refs in CHECK
                    //           ** expressions. Also update the name that appears right after the
                    //           ** "CREATE [VIRTUAL] TABLE" bit. */
                    if (unsafe { sqlite3_stricmp(zOld, (unsafe { (*pTab).zName }) as *const i8) })
                        == (0 as i32)
                    {
                        sCtx.pTab = pTab;
                        if isLegacy == (0 as i32) {
                            unsafe {
                                sqlite3WalkExprList(std::ptr::addr_of_mut!(sWalker), unsafe {
                                    (*pTab).pCheck
                                })
                            };
                        }
                        renameTokenFind(
                            std::ptr::addr_of_mut!(sParse),
                            std::ptr::addr_of_mut!(sCtx),
                            (unsafe { (*pTab).zName }) as *const (),
                        );
                    }
                }
            } else {
                if sParse.pNewIndex != std::ptr::null_mut::<Index>() {
                    renameTokenFind(
                        std::ptr::addr_of_mut!(sParse),
                        std::ptr::addr_of_mut!(sCtx),
                        (unsafe { (*sParse.pNewIndex).zName }) as *const (),
                    );
                    if isLegacy == (0 as i32) {
                        unsafe {
                            sqlite3WalkExpr(std::ptr::addr_of_mut!(sWalker), unsafe {
                                (*sParse.pNewIndex).pPartIdxWhere
                            })
                        };
                    }
                } else {
                    let mut pTrigger: *mut Trigger = sParse.pNewTrigger;
                    let mut pStep: *mut TriggerStep = unsafe { std::mem::zeroed() };
                    if (0 as i32)
                        == unsafe {
                            sqlite3_stricmp(
                                (unsafe { (*sParse.pNewTrigger).table }) as *const i8,
                                zOld,
                            )
                        }
                        && (unsafe { (*sCtx.pTab).pSchema }) == unsafe { (*pTrigger).pTabSchema }
                    {
                        renameTokenFind(
                            std::ptr::addr_of_mut!(sParse),
                            std::ptr::addr_of_mut!(sCtx),
                            (unsafe { (*sParse.pNewTrigger).table }) as *const (),
                        );
                    }
                    if isLegacy == (0 as i32) {
                        rc = renameResolveTrigger(std::ptr::addr_of_mut!(sParse));
                        if rc == (0 as i32) {
                            renameWalkTrigger(std::ptr::addr_of_mut!(sWalker), pTrigger);
                            pStep = unsafe { (*pTrigger).step_list };
                            '__slate_break_1300: while pStep != std::ptr::null_mut::<TriggerStep>()
                            {
                                if (unsafe { (*pStep).pSrc }) != std::ptr::null_mut::<SrcList>() {
                                    let mut i: i32 = 0 as i32;
                                    i = 0 as i32;
                                    '__slate_break_1301: loop {
                                        if !(i < unsafe { (*unsafe { (*pStep).pSrc }).nSrc }) {
                                            break;
                                        }
                                        let mut pItem: *mut SrcItem = unsafe {
                                            unsafe {
                                                std::ptr::addr_of_mut!(
                                                    (*unsafe { (*pStep).pSrc }).a
                                                )
                                                    as *mut SrcItem
                                            }
                                            .offset(i as isize)
                                        };
                                        if (0 as i32)
                                            == unsafe {
                                                sqlite3_stricmp(
                                                    (unsafe { (*pItem).zName }) as *const i8,
                                                    zOld,
                                                )
                                            }
                                        {
                                            renameTokenFind(
                                                std::ptr::addr_of_mut!(sParse),
                                                std::ptr::addr_of_mut!(sCtx),
                                                (unsafe { (*pItem).zName }) as *const (),
                                            );
                                        }
                                        let __v1436: i32 = i;
                                        let __v1437: i32 = __v1436 + (1 as i32);
                                        i = __v1437;
                                    }
                                }
                                pStep = unsafe { (*pStep).pNext };
                            }
                        }
                    }
                }
            }
        }
        if rc == (0 as i32) {
            rc = renameEditSql(context, std::ptr::addr_of_mut!(sCtx), zInput, zNew, bQuote);
        }
        if rc != (0 as i32) {
            let __v1438: bool;
            if rc == (1 as i32) {
                __v1438 = (unsafe { sqlite3WritableSchema(db) }) != (0 as i32);
            } else {
                __v1438 = false as bool;
            }
            if __v1438 {
                unsafe {
                    sqlite3_result_value(context, unsafe {
                        *unsafe { argv.offset((3 as i32) as isize) }
                    })
                };
            } else {
                if sParse.zErrMsg != std::ptr::null_mut::<i8>() {
                    renameColumnParseError(
                        context,
                        (b"\0".as_ptr() as *mut i8) as *const i8,
                        unsafe { *unsafe { argv.offset((1 as i32) as isize) } },
                        unsafe { *unsafe { argv.offset((2 as i32) as isize) } },
                        std::ptr::addr_of_mut!(sParse),
                    );
                } else {
                    unsafe { sqlite3_result_error_code(context, rc) };
                }
            }
        }
        renameParseCleanup(std::ptr::addr_of_mut!(sParse));
        renameTokenFree(db, sCtx.pList);
        unsafe { sqlite3BtreeLeaveAll(db) };
        unsafe {
            (*db).xAuth = xAuth;
        }
    }
    return;
}

#[unsafe(link_section = ".text.slate_distinct.alter.renameQuotefixExprCb")]
extern "C-unwind" fn renameQuotefixExprCb(mut pWalker: *mut Walker, mut pExpr: *mut Expr) -> i32 {
    if (((unsafe { (*pExpr).op }) as u32) as i32) == (118 as i32)
        && (unsafe { (*pExpr).flags }) & ((128 as i32) as u32) != (0 as u32)
    {
        renameTokenFind(
            unsafe { (*pWalker).pParse },
            unsafe { (*pWalker).u.pRename },
            pExpr as *const (),
        );
    }
    return 0 as i32;
}

// /* SQL function: sqlite_rename_quotefix(DB,SQL)
// **
// ** Rewrite the DDL statement "SQL" so that any string literals that use
// ** double-quotes use single quotes instead.
// **
// ** Two arguments must be passed:
// **
// **   0: Database name ("main", "temp" etc.).
// **   1: SQL statement to edit.
// **
// ** The returned value is the modified SQL statement. For example, given
// ** the database schema:
// **
// **   CREATE TABLE t1(a, b, c);
// **
// **   SELECT sqlite_rename_quotefix('main',
// **       'CREATE VIEW v1 AS SELECT "a", "string" FROM t1'
// **   );
// **
// ** returns the string:
// **
// **   CREATE VIEW v1 AS SELECT "a", 'string' FROM t1
// **
// ** If there is a error in the input SQL, then raise an error, except
// ** if PRAGMA writable_schema=ON, then just return the input string
// ** unmodified following an error.
// */
#[unsafe(link_section = ".text.slate_distinct.alter.renameQuotefixFunc")]
extern "C-unwind" fn renameQuotefixFunc(
    mut context: *mut sqlite3_context,
    mut NotUsed: i32,
    mut argv: *mut *mut sqlite3_value,
) {
    let mut db: *mut sqlite3 = unsafe { sqlite3_context_db_handle(context) };
    let mut zDb: *const i8 =
        (unsafe { sqlite3_value_text(unsafe { *unsafe { argv.offset((0 as i32) as isize) } }) })
            as *const i8;
    let mut zInput: *const i8 =
        (unsafe { sqlite3_value_text(unsafe { *unsafe { argv.offset((1 as i32) as isize) } }) })
            as *const i8;
    let mut xAuth: Option<
        unsafe extern "C-unwind" fn(
            *mut (),
            i32,
            *const i8,
            *const i8,
            *const i8,
            *const i8,
        ) -> i32,
    > = unsafe { (*db).xAuth };
    unsafe {
        (*db).xAuth = None;
    }
    unsafe { sqlite3BtreeEnterAll(db) };
    NotUsed;
    if zDb != std::ptr::null::<i8>() && zInput != std::ptr::null::<i8>() {
        let mut rc: i32 = 0 as i32;
        let mut sParse: Parse = unsafe { std::mem::zeroed() };
        rc = renameParseSql(std::ptr::addr_of_mut!(sParse), zDb, db, zInput, 0 as i32);
        if rc == (0 as i32) {
            let mut sCtx: RenameCtx = unsafe { std::mem::zeroed() };
            let mut sWalker: Walker = unsafe { std::mem::zeroed() };
            // /* Walker to find tokens that need to be replaced. */
            unsafe { memset(std::ptr::addr_of_mut!(sCtx) as *mut (), 0 as i32, 32 as u64) };
            unsafe {
                memset(
                    std::ptr::addr_of_mut!(sWalker) as *mut (),
                    0 as i32,
                    48 as u64,
                )
            };
            sWalker.pParse = std::ptr::addr_of_mut!(sParse);
            sWalker.xExprCallback = Some(renameQuotefixExprCb);
            sWalker.xSelectCallback = Some(renameColumnSelectCb);
            unsafe {
                sWalker.u.pRename = std::ptr::addr_of_mut!(sCtx);
            }
            if sParse.pNewTable != std::ptr::null_mut::<Table>() {
                if (((unsafe { (*sParse.pNewTable).eTabType }) as u32) as i32) == (2 as i32) {
                    let mut pSelect: *mut Select = unsafe { (*sParse.pNewTable).u.view.pSelect };
                    let __v1439: *mut Select = pSelect;
                    let __v1440: u32 = unsafe { (*__v1439).selFlags };
                    let __v1441: u32 = __v1440 & !((2097152 as i32) as u32);
                    unsafe {
                        (*__v1439).selFlags = __v1441;
                    }
                    sParse.rc = 0 as i32;
                    unsafe {
                        sqlite3SelectPrep(
                            std::ptr::addr_of_mut!(sParse),
                            pSelect,
                            std::ptr::null_mut::<NameContext>(),
                        )
                    };
                    rc = if (unsafe { (*db).mallocFailed }) != (0 as u8) {
                        7 as i32
                    } else {
                        sParse.rc
                    };
                    if rc == (0 as i32) {
                        unsafe { sqlite3WalkSelect(std::ptr::addr_of_mut!(sWalker), pSelect) };
                    }
                } else {
                    let mut i: i32 = 0 as i32;
                    unsafe {
                        sqlite3WalkExprList(std::ptr::addr_of_mut!(sWalker), unsafe {
                            (*sParse.pNewTable).pCheck
                        })
                    };
                    i = 0 as i32;
                    '__slate_break_1303: loop {
                        if !(i < ((unsafe { (*sParse.pNewTable).nCol }) as i32)) {
                            break;
                        }
                        unsafe {
                            sqlite3WalkExpr(std::ptr::addr_of_mut!(sWalker), unsafe {
                                sqlite3ColumnExpr(sParse.pNewTable, unsafe {
                                    unsafe { (*sParse.pNewTable).aCol }.offset(i as isize)
                                })
                            })
                        };
                        let __v1442: i32 = i;
                        let __v1443: i32 = __v1442 + (1 as i32);
                        i = __v1443;
                    }
                    // /* SQLITE_OMIT_GENERATED_COLUMNS */
                }
            } else {
                if sParse.pNewIndex != std::ptr::null_mut::<Index>() {
                    unsafe {
                        sqlite3WalkExprList(std::ptr::addr_of_mut!(sWalker), unsafe {
                            (*sParse.pNewIndex).aColExpr
                        })
                    };
                    unsafe {
                        sqlite3WalkExpr(std::ptr::addr_of_mut!(sWalker), unsafe {
                            (*sParse.pNewIndex).pPartIdxWhere
                        })
                    };
                } else {
                    rc = renameResolveTrigger(std::ptr::addr_of_mut!(sParse));
                    if rc == (0 as i32) {
                        renameWalkTrigger(std::ptr::addr_of_mut!(sWalker), sParse.pNewTrigger);
                    }
                    // /* SQLITE_OMIT_TRIGGER */
                }
            }
            if rc == (0 as i32) {
                rc = renameEditSql(
                    context,
                    std::ptr::addr_of_mut!(sCtx),
                    zInput,
                    std::ptr::null::<i8>(),
                    0 as i32,
                );
            }
            renameTokenFree(db, sCtx.pList);
        }
        if rc != (0 as i32) {
            if (unsafe { sqlite3WritableSchema(db) }) != (0 as i32) && rc == (1 as i32) {
                unsafe {
                    sqlite3_result_value(context, unsafe {
                        *unsafe { argv.offset((1 as i32) as isize) }
                    })
                };
            } else {
                unsafe { sqlite3_result_error_code(context, rc) };
            }
        }
        renameParseCleanup(std::ptr::addr_of_mut!(sParse));
    }
    unsafe {
        (*db).xAuth = xAuth;
    }
    unsafe { sqlite3BtreeLeaveAll(db) };
}

// /* Function:  sqlite_rename_test(DB,SQL,TYPE,NAME,ISTEMP,WHEN,DQS)
// **
// ** An SQL user function that checks that there are no parse or symbol
// ** resolution problems in a CREATE TRIGGER|TABLE|VIEW|INDEX statement.
// ** After an ALTER TABLE .. RENAME operation is performed and the schema
// ** reloaded, this function is called on each SQL statement in the schema
// ** to ensure that it is still usable.
// **
// **   0: Database name ("main", "temp" etc.).
// **   1: SQL statement.
// **   2: Object type ("view", "table", "trigger" or "index").
// **   3: Object name.
// **   4: True if object is from temp schema.
// **   5: "when" part of error message.
// **   6: True to disable the DQS quirk when parsing SQL.
// **
// ** The return value is computed as follows:
// **
// **   A. If an error is seen and not in PRAGMA writable_schema=ON mode,
// **      then raise the error.
// **   B. Else if a trigger is created and the the table that the trigger is
// **      attached to is in database zDb, then return 1.
// **   C. Otherwise return NULL.
// */
#[unsafe(link_section = ".text.slate_distinct.alter.renameTableTest")]
extern "C-unwind" fn renameTableTest(
    mut context: *mut sqlite3_context,
    mut NotUsed: i32,
    mut argv: *mut *mut sqlite3_value,
) {
    let mut db: *mut sqlite3 = unsafe { sqlite3_context_db_handle(context) };
    let mut zDb: *const i8 =
        (unsafe { sqlite3_value_text(unsafe { *unsafe { argv.offset((0 as i32) as isize) } }) })
            as *const i8;
    let mut zInput: *const i8 =
        (unsafe { sqlite3_value_text(unsafe { *unsafe { argv.offset((1 as i32) as isize) } }) })
            as *const i8;
    let mut bTemp: i32 =
        unsafe { sqlite3_value_int(unsafe { *unsafe { argv.offset((4 as i32) as isize) } }) };
    let mut isLegacy: i32 =
        (((unsafe { (*db).flags }) & (((67108864 as i32) as i64) as u64)) as u32) as i32;
    let mut zWhen: *const i8 =
        (unsafe { sqlite3_value_text(unsafe { *unsafe { argv.offset((5 as i32) as isize) } }) })
            as *const i8;
    let mut bNoDQS: i32 =
        unsafe { sqlite3_value_int(unsafe { *unsafe { argv.offset((6 as i32) as isize) } }) };
    let mut xAuth: Option<
        unsafe extern "C-unwind" fn(
            *mut (),
            i32,
            *const i8,
            *const i8,
            *const i8,
            *const i8,
        ) -> i32,
    > = unsafe { (*db).xAuth };
    unsafe {
        (*db).xAuth = None;
    }
    NotUsed;
    if zDb != std::ptr::null::<i8>() && zInput != std::ptr::null::<i8>() {
        let mut rc: i32 = 0 as i32;
        let mut sParse: Parse = unsafe { std::mem::zeroed() };
        let mut flags: u64 = unsafe { (*db).flags };
        if bNoDQS != (0 as i32) {
            let __v1444: *mut sqlite3 = db;
            let __v1445: u64 = unsafe { (*__v1444).flags };
            let __v1446: u64 =
                __v1445 & ((!((1073741824 as i32) | (536870912 as i32)) as i64) as u64);
            unsafe {
                (*__v1444).flags = __v1446;
            }
        }
        rc = renameParseSql(std::ptr::addr_of_mut!(sParse), zDb, db, zInput, bTemp);
        unsafe {
            (*db).flags = flags;
        }
        if rc == (0 as i32) {
            if isLegacy == (0 as i32)
                && sParse.pNewTable != std::ptr::null_mut::<Table>()
                && (((unsafe { (*sParse.pNewTable).eTabType }) as u32) as i32) == (2 as i32)
            {
                let mut sNC: NameContext = unsafe { std::mem::zeroed() };
                unsafe { memset(std::ptr::addr_of_mut!(sNC) as *mut (), 0 as i32, 56 as u64) };
                sNC.pParse = std::ptr::addr_of_mut!(sParse);
                unsafe {
                    sqlite3SelectPrep(
                        std::ptr::addr_of_mut!(sParse),
                        unsafe { (*sParse.pNewTable).u.view.pSelect },
                        std::ptr::addr_of_mut!(sNC),
                    )
                };
                if sParse.nErr != (0 as i32) {
                    rc = sParse.rc;
                }
            } else {
                if sParse.pNewTrigger != std::ptr::null_mut::<Trigger>() {
                    if isLegacy == (0 as i32) {
                        rc = renameResolveTrigger(std::ptr::addr_of_mut!(sParse));
                    }
                    if rc == (0 as i32) {
                        let mut i1: i32 = unsafe {
                            sqlite3SchemaToIndex(db, unsafe { (*sParse.pNewTrigger).pTabSchema })
                        };
                        let mut i2: i32 = unsafe { sqlite3FindDbName(db, zDb) };
                        if i1 == i2 {
                            // /* Handle output case B */
                            unsafe { sqlite3_result_int(context, 1 as i32) };
                        }
                    }
                }
            }
        }
        let __v1447: bool;
        if rc != (0 as i32) && zWhen != std::ptr::null::<i8>() {
            __v1447 = !((unsafe { sqlite3WritableSchema(db) }) != (0 as i32));
        } else {
            __v1447 = false as bool;
        }
        if __v1447 {
            // /* Output case A */
            renameColumnParseError(
                context,
                zWhen,
                unsafe { *unsafe { argv.offset((2 as i32) as isize) } },
                unsafe { *unsafe { argv.offset((3 as i32) as isize) } },
                std::ptr::addr_of_mut!(sParse),
            );
        }
        renameParseCleanup(std::ptr::addr_of_mut!(sParse));
    }
    unsafe {
        (*db).xAuth = xAuth;
    }
}

// /*
// ** Return the number of bytes until the end of the next non-whitespace and
// ** non-comment token.  For the purpose of this function, a "(" token includes
// ** all of the bytes through and including the matching ")", or until the
// ** first illegal token, whichever comes first.
// **
// ** Write the token type into *piToken.
// **
// ** The value returned is the number of bytes in the token itself plus
// ** the number of bytes of leading whitespace and comments skipped plus
// ** all bytes through the next matching ")" if the token is TK_LP.
// **
// ** Example:    (Note: '.' used in place of '*' in the example z[] text)
// **
// **                                    ,--------- *piToken := TK_RP
// **                                    v
// **    z[] = " /.comment./ --comment\n (two three four) five"
// **          |                                        |
// **          |<-------------------------------------->|
// **                              |
// **                              `--- return value
// */
fn getConstraintToken(mut z: *const u8, mut piToken: *mut i32) -> i32 {
    let mut iOff: i32 = 0 as i32;
    let mut t: i32 = 0 as i32;
    '__slate_break_1304: loop {
        let __v1448: i32 = iOff;
        let __v1449: i32 = ((__v1448 as i64)
            + unsafe {
                sqlite3GetToken(
                    unsafe { z.offset(iOff as isize) },
                    std::ptr::addr_of_mut!(t),
                )
            }) as i32;
        iOff = __v1449;
        if !(t == (184 as i32) || t == (185 as i32)) {
            break;
        }
    }
    unsafe {
        *piToken = t;
    }
    if t == (22 as i32) {
        let mut nNest: i32 = 1 as i32;
        '__slate_break_1305: while nNest > (0 as i32) {
            let __v1450: i32 = iOff;
            let __v1451: i32 = ((__v1450 as i64)
                + unsafe {
                    sqlite3GetToken(
                        unsafe { z.offset(iOff as isize) },
                        std::ptr::addr_of_mut!(t),
                    )
                }) as i32;
            iOff = __v1451;
            if t == (22 as i32) {
                let __v1452: i32 = nNest;
                let __v1453: i32 = __v1452 + (1 as i32);
                nNest = __v1453;
            } else {
                if t == (23 as i32) {
                    t = 22 as i32;
                    let __v1454: i32 = nNest;
                    let __v1455: i32 = __v1454 - (1 as i32);
                    nNest = __v1455;
                } else {
                    if t == (186 as i32) {
                        break '__slate_break_1305;
                    }
                }
            }
        }
    }
    unsafe {
        *piToken = t;
    }
    return iOff;
}

// /*
// ** The implementation of internal UDF sqlite_drop_column().
// **
// ** Arguments:
// **
// **  argv[0]: An integer - the index of the schema containing the table
// **  argv[1]: CREATE TABLE statement to modify.
// **  argv[2]: An integer - the index of the column to remove.
// **
// ** The value returned is a string containing the CREATE TABLE statement
// ** with column argv[2] removed.
// */
#[unsafe(link_section = ".text.slate_distinct.alter.dropColumnFunc")]
extern "C-unwind" fn dropColumnFunc(
    mut context: *mut sqlite3_context,
    mut NotUsed: i32,
    mut argv: *mut *mut sqlite3_value,
) {
    let mut __slate_storage_790: std::mem::MaybeUninit<*mut RenameToken> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_790: *mut *mut RenameToken =
        std::ptr::addr_of_mut!(__slate_storage_790) as *mut *mut RenameToken;
    let mut __slate_storage_1461: std::mem::MaybeUninit<*const i8> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1461: *mut *const i8 =
        std::ptr::addr_of_mut!(__slate_storage_1461) as *mut *const i8;
    let mut __slate_storage_1460: std::mem::MaybeUninit<*const i8> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1460: *mut *const i8 =
        std::ptr::addr_of_mut!(__slate_storage_1460) as *mut *const i8;
    let mut __slate_storage_1459: std::mem::MaybeUninit<*mut RenameToken> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1459: *mut *mut RenameToken =
        std::ptr::addr_of_mut!(__slate_storage_1459) as *mut *mut RenameToken;
    let mut __slate_storage_1458: std::mem::MaybeUninit<*const i8> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1458: *mut *const i8 =
        std::ptr::addr_of_mut!(__slate_storage_1458) as *mut *const i8;
    let mut __slate_storage_1457: std::mem::MaybeUninit<*const i8> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1457: *mut *const i8 =
        std::ptr::addr_of_mut!(__slate_storage_1457) as *mut *const i8;
    let mut __slate_storage_1456: std::mem::MaybeUninit<*mut RenameToken> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1456: *mut *mut RenameToken =
        std::ptr::addr_of_mut!(__slate_storage_1456) as *mut *mut RenameToken;
    let mut __slate_storage_791: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_791: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_791) as *mut i32;
    let mut __slate_storage_789: std::mem::MaybeUninit<
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
    let __slate_slot_789: *mut Option<
        unsafe extern "C-unwind" fn(
            *mut (),
            i32,
            *const i8,
            *const i8,
            *const i8,
            *const i8,
        ) -> i32,
    > = std::ptr::addr_of_mut!(__slate_storage_789)
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
    let mut __slate_storage_788: std::mem::MaybeUninit<*mut i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_788: *mut *mut i8 =
        std::ptr::addr_of_mut!(__slate_storage_788) as *mut *mut i8;
    let mut __slate_storage_787: std::mem::MaybeUninit<*const i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_787: *mut *const i8 =
        std::ptr::addr_of_mut!(__slate_storage_787) as *mut *const i8;
    let mut __slate_storage_786: std::mem::MaybeUninit<*mut Table> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_786: *mut *mut Table =
        std::ptr::addr_of_mut!(__slate_storage_786) as *mut *mut Table;
    let mut __slate_storage_785: std::mem::MaybeUninit<*mut RenameToken> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_785: *mut *mut RenameToken =
        std::ptr::addr_of_mut!(__slate_storage_785) as *mut *mut RenameToken;
    let mut __slate_storage_784: std::mem::MaybeUninit<Parse> = std::mem::MaybeUninit::uninit();
    let __slate_slot_784: *mut Parse = std::ptr::addr_of_mut!(__slate_storage_784) as *mut Parse;
    let mut __slate_storage_783: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_783: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_783) as *mut i32;
    let mut __slate_storage_782: std::mem::MaybeUninit<*const i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_782: *mut *const i8 =
        std::ptr::addr_of_mut!(__slate_storage_782) as *mut *const i8;
    let mut __slate_storage_781: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_781: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_781) as *mut i32;
    let mut __slate_storage_780: std::mem::MaybeUninit<*const i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_780: *mut *const i8 =
        std::ptr::addr_of_mut!(__slate_storage_780) as *mut *const i8;
    let mut __slate_storage_779: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_779: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_779) as *mut i32;
    let mut __slate_storage_778: std::mem::MaybeUninit<*mut sqlite3> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_778: *mut *mut sqlite3 =
        std::ptr::addr_of_mut!(__slate_storage_778) as *mut *mut sqlite3;
    unsafe {
        '__join_2: {
            std::ptr::write(__slate_slot_778, unsafe {
                sqlite3_context_db_handle(context)
            });
            std::ptr::write(__slate_slot_779, unsafe {
                sqlite3_value_int(unsafe { *unsafe { argv.offset((0 as i32) as isize) } })
            });
            std::ptr::write(
                __slate_slot_780,
                (unsafe {
                    sqlite3_value_text(unsafe { *unsafe { argv.offset((1 as i32) as isize) } })
                }) as *const i8,
            );
            std::ptr::write(__slate_slot_781, unsafe {
                sqlite3_value_int(unsafe { *unsafe { argv.offset((2 as i32) as isize) } })
            });
            std::ptr::write(
                __slate_slot_782,
                (unsafe {
                    (*unsafe {
                        unsafe { (*(*__slate_slot_778)).aDb }.offset(*__slate_slot_779 as isize)
                    })
                    .zDbSName
                }) as *const i8,
            );
            std::ptr::write(__slate_slot_788, std::ptr::null_mut::<i8>());
            std::ptr::write(__slate_slot_789, unsafe { (*(*__slate_slot_778)).xAuth });
            unsafe {
                (*(*__slate_slot_778)).xAuth = None;
            }
            NotUsed;
            *__slate_slot_783 = renameParseSql(
                std::ptr::addr_of_mut!(*__slate_slot_784),
                *__slate_slot_782,
                *__slate_slot_778,
                *__slate_slot_780,
                (*__slate_slot_779 == (1 as i32)) as i32,
            );
            if *__slate_slot_783 != (0 as i32) {
            } else {
                *__slate_slot_786 = (*__slate_slot_784).pNewTable;
                if *__slate_slot_786 == std::ptr::null_mut::<Table>()
                    || ((unsafe { (*(*__slate_slot_786)).nCol }) as i32) == (1 as i32)
                    || *__slate_slot_781 >= ((unsafe { (*(*__slate_slot_786)).nCol }) as i32)
                {
                    // /* This can happen if the sqlite_schema table is corrupt */
                    *__slate_slot_783 = unsafe { sqlite3CorruptError(2204 as i32) };
                } else {
                    if *__slate_slot_781
                        < ((unsafe { (*(*__slate_slot_786)).nCol }) as i32) - (1 as i32)
                    {
                        *__slate_slot_785 = renameTokenFind(
                            std::ptr::addr_of_mut!(*__slate_slot_784),
                            std::ptr::null_mut::<RenameCtx>(),
                            ((unsafe {
                                (*unsafe {
                                    unsafe { (*(*__slate_slot_786)).aCol }
                                        .offset(*__slate_slot_781 as isize)
                                })
                                .zCnName
                            }) as *mut ()) as *const (),
                        );
                        *__slate_slot_790 = renameTokenFind(
                            std::ptr::addr_of_mut!(*__slate_slot_784),
                            std::ptr::null_mut::<RenameCtx>(),
                            ((unsafe {
                                (*unsafe {
                                    unsafe { (*(*__slate_slot_786)).aCol }
                                        .offset((*__slate_slot_781 + (1 as i32)) as isize)
                                })
                                .zCnName
                            }) as *mut ()) as *const (),
                        );
                        *__slate_slot_787 = unsafe { (*(*__slate_slot_790)).t.z };
                    } else {
                        0 as i32;
                        0 as i32;
                        // /* Point pCol->t.z at the "," immediately preceding the definition of
                        //     ** the column being dropped. To do this, start at the name of the
                        //     ** previous column, and tokenize until the next ",".  */
                        *__slate_slot_785 = renameTokenFind(
                            std::ptr::addr_of_mut!(*__slate_slot_784),
                            std::ptr::null_mut::<RenameCtx>(),
                            ((unsafe {
                                (*unsafe {
                                    unsafe { (*(*__slate_slot_786)).aCol }
                                        .offset((*__slate_slot_781 - (1 as i32)) as isize)
                                })
                                .zCnName
                            }) as *mut ()) as *const (),
                        );
                        loop {
                            std::ptr::write(__slate_slot_1456, *__slate_slot_785);
                            std::ptr::write(__slate_slot_1457, unsafe {
                                (*(*__slate_slot_1456)).t.z
                            });
                            std::ptr::write(__slate_slot_1458, unsafe {
                                (*__slate_slot_1457).offset(getConstraintToken(
                                    (unsafe { (*(*__slate_slot_785)).t.z }) as *const u8,
                                    std::ptr::addr_of_mut!(*__slate_slot_791),
                                )
                                    as isize)
                            });
                            unsafe {
                                (*(*__slate_slot_1456)).t.z = *__slate_slot_1458;
                            }
                            if !(*__slate_slot_791 != (25 as i32)) {
                                break;
                            }
                        }
                        std::ptr::write(__slate_slot_1459, *__slate_slot_785);
                        std::ptr::write(__slate_slot_1460, unsafe { (*(*__slate_slot_1459)).t.z });
                        std::ptr::write(__slate_slot_1461, unsafe {
                            (*__slate_slot_1460).offset(-((1 as i32) as isize))
                        });
                        unsafe {
                            (*(*__slate_slot_1459)).t.z = *__slate_slot_1461;
                        }
                        *__slate_slot_787 = unsafe {
                            (*__slate_slot_780).offset(
                                (unsafe { (*(*__slate_slot_786)).u.tab.addColOffset }) as isize,
                            )
                        };
                    }
                    *__slate_slot_788 = unsafe {
                        sqlite3MPrintf(
                            *__slate_slot_778,
                            (b"%.*s%s\0".as_ptr() as *mut i8) as *const i8,
                            (unsafe {
                                unsafe { (*(*__slate_slot_785)).t.z }
                                    .offset_from(*__slate_slot_780 as *const i8)
                            }) as i64,
                            *__slate_slot_780,
                            *__slate_slot_787,
                        )
                    };
                    unsafe {
                        sqlite3_result_text(
                            context,
                            *__slate_slot_788 as *const i8,
                            -(1 as i32),
                            unsafe {
                                std::mem::transmute::<
                                    usize,
                                    Option<unsafe extern "C-unwind" fn(*mut ())>,
                                >(-(1 as i32) as usize)
                            },
                        )
                    };
                    unsafe { sqlite3_free(*__slate_slot_788 as *mut ()) };
                }
            }
        }
        renameParseCleanup(std::ptr::addr_of_mut!(*__slate_slot_784));
        unsafe {
            (*(*__slate_slot_778)).xAuth = *__slate_slot_789;
        }
        if *__slate_slot_783 != (0 as i32) {
            unsafe { sqlite3_result_error_code(context, *__slate_slot_783) };
        }
    }
}

// /*
// ** Return the number of bytes of leading whitespace/comments in string z[].
// */
fn getWhitespace(mut z: *const u8) -> i32 {
    let mut nRet: i32 = 0 as i32;
    '__slate_break_1318: while (1 as i32) != (0 as i32) {
        let mut t: i32 = 0 as i32;
        let mut n: i32 = (unsafe {
            sqlite3GetToken(
                unsafe { z.offset(nRet as isize) },
                std::ptr::addr_of_mut!(t),
            )
        }) as i32;
        if t != (184 as i32) && t != (185 as i32) {
            break '__slate_break_1318;
        }
        let __v1462: i32 = nRet;
        let __v1463: i32 = __v1462 + n;
        nRet = __v1463;
    }
    return nRet;
}

// /*
// ** Argument z points into the body of a constraint - specifically the
// ** second token of the constraint definition.  For a named constraint,
// ** z points to the second token of the constraint definition. For an
// ** unnamed NOT NULL constraint, z points to the first byte past the NOT
// ** keyword.
// **
// ** Argument eTok may be the token value of the first token of the constraint
// ** (e.g. TK_CHECK or TK_REFERENCES) or zero. If it is either TK_REFERENCES
// ** or TK_FOREIGN, special parsing is enabled to find the end of the foreign-key
// ** constraint definition.
// **
// ** Return the number of bytes until the end of the constraint.
// */
fn getConstraint(mut z: *const u8, mut eTok: i32) -> i32 {
    let mut iOff: i32 = 0 as i32;
    let mut t: i32 = 0 as i32;
    if eTok == (133 as i32) {
        // /* For a FOREIGN KEY constraint, use getConstraint() to parse everything
        //     ** up to the REFERENCES keyword. Then getConstraintToken() to consume
        //     ** the TK_REFERENCES token itself. Then fall through to the special
        //     ** handling for TK_REFERENCES below.  */
        iOff = getConstraint(z, 0 as i32);
        let __v1464: i32 = iOff;
        let __v1465: i32 = __v1464
            + getConstraintToken(
                unsafe { z.offset(iOff as isize) },
                std::ptr::addr_of_mut!(eTok),
            );
        iOff = __v1465;
    }
    if eTok == (126 as i32) {
        // /* REFERENCES is followed by a table name. Gobble this up here in
        //     ** case the table name is a fallback token like TK_GENERATED. */
        let __v1466: i32 = iOff;
        let __v1467: i32 = __v1466
            + getConstraintToken(
                unsafe { z.offset(iOff as isize) },
                std::ptr::addr_of_mut!(t),
            );
        iOff = __v1467;
    }
    // /* Now, the current constraint proceeds until the next occurence of one
    //   ** of the following tokens:
    //   **
    //   **   CONSTRAINT, PRIMARY, NOT, UNIQUE, CHECK, DEFAULT,
    //   **   COLLATE, REFERENCES, FOREIGN, GENERATED, AS, RP, or COMMA
    //   **
    //   ** Also exit the loop if ILLEGAL turns up.
    //   */
    '__slate_break_1319: while (1 as i32) != (0 as i32) {
        let mut n: i32 = getConstraintToken(
            unsafe { z.offset(iOff as isize) },
            std::ptr::addr_of_mut!(t),
        );
        if t == (120 as i32)
            || t == (123 as i32)
            || t == (19 as i32)
            || t == (124 as i32)
            || t == (125 as i32)
            || t == (121 as i32)
            || t == (114 as i32)
            || t == (126 as i32)
            || t == (133 as i32)
            || t == (23 as i32)
            || t == (25 as i32)
            || t == (186 as i32)
            || t == (24 as i32)
            || t == (96 as i32)
        {
            break '__slate_break_1319;
        }
        let __v1468: i32 = iOff;
        let __v1469: i32 = __v1468 + n;
        iOff = __v1469;
    }
    return iOff;
}

// /*
// ** Compare two constraint names.
// **
// ** Summary:   *pRes := zQuote != zCmp
// **
// ** Details:
// ** Compare the (possibly quoted) constraint name zQuote[0..nQuote-1]
// ** against zCmp[].  Write zero into *pRes if they are the same and
// ** non-zero if they differ.  Normally return SQLITE_OK, except if there
// ** is an OOM, set the OOM error condition on ctx and return SQLITE_NOMEM.
// */
fn quotedCompare(
    mut ctx: *mut sqlite3_context,
    mut t: i32,
    mut zQuote: *const u8,
    mut nQuote: i32,
    mut zCmp: *const u8,
    mut pRes: *mut i32,
) -> i32 {
    // /* De-quoted, zero-terminated copy of zQuote[] */
    let mut zCopy: *mut i8 = std::ptr::null_mut::<i8>();
    if t == (186 as i32) {
        unsafe {
            *pRes = 1 as i32;
        }
        return 0 as i32;
    }
    zCopy = (unsafe { sqlite3MallocZero(((nQuote + (1 as i32)) as i64) as u64) }) as *mut i8;
    if zCopy == std::ptr::null_mut::<i8>() {
        unsafe { sqlite3_result_error_nomem(ctx) };
        return 7 as i32;
    }
    unsafe {
        memcpy(
            zCopy as *mut (),
            zQuote as *const (),
            (nQuote as i64) as u64,
        )
    };
    unsafe { sqlite3Dequote(zCopy) };
    unsafe {
        *pRes = unsafe { sqlite3_stricmp(zCopy as *const i8, zCmp as *const i8) };
    }
    unsafe { sqlite3_free(zCopy as *mut ()) };
    return 0 as i32;
}

// /* Function context on which to report errors */
// /* Token type */
// /* Possibly quoted text.  Not zero-terminated. */
// /* Length of zQuote in bytes */
// /* Zero-terminated, unquoted name to compare against */
// /* OUT: Set to 0 if equal, non-zero if unequal */
// /*
// ** zSql[] is a CREATE TABLE statement, supposedly.  Find the offset
// ** into zSql[] of the first character past the first "(" and write
// ** that offset into *piOff and return SQLITE_OK.  Or, if not found,
// ** set the SQLITE_CORRUPT error code and return SQLITE_ERROR.
// */
fn skipCreateTable(mut ctx: *mut sqlite3_context, mut zSql: *const u8, mut piOff: *mut i32) -> i32 {
    let mut iOff: i32 = 0 as i32;
    if zSql == std::ptr::null::<u8>() {
        return 1 as i32;
    }
    // /* Jump past the "CREATE TABLE" bit. */
    '__slate_break_1320: while (1 as i32) != (0 as i32) {
        let mut t: i32 = 0 as i32;
        let __v1470: i32 = iOff;
        let __v1471: i32 = ((__v1470 as i64)
            + unsafe {
                sqlite3GetToken(
                    unsafe { zSql.offset(iOff as isize) },
                    std::ptr::addr_of_mut!(t),
                )
            }) as i32;
        iOff = __v1471;
        if t == (22 as i32) {
            break '__slate_break_1320;
        }
        if t == (186 as i32) {
            unsafe { sqlite3_result_error_code(ctx, unsafe { sqlite3CorruptError(2521 as i32) }) };
            return 1 as i32;
        }
    }
    unsafe {
        *piOff = iOff;
    }
    return 0 as i32;
}

// /*
// ** Internal SQL function sqlite3_drop_constraint():  Given an input
// ** CREATE TABLE statement, return a revised CREATE TABLE statement
// ** with a constraint removed.  Two forms, depending on the datatype
// ** of argv[2]:
// **
// **   sqlite_drop_constraint(SQL, INT)  -- Omit NOT NULL from the INT-th column
// **   sqlite_drop_constraint(SQL, TEXT) -- OMIT constraint with name TEXT
// **
// ** In the first case, the left-most column is 0.
// */
#[unsafe(link_section = ".text.slate_distinct.alter.dropConstraintFunc")]
extern "C-unwind" fn dropConstraintFunc(
    mut ctx: *mut sqlite3_context,
    mut NotUsed: i32,
    mut argv: *mut *mut sqlite3_value,
) {
    let mut zSql: *const u8 =
        unsafe { sqlite3_value_text(unsafe { *unsafe { argv.offset((0 as i32) as isize) } }) };
    let mut zCons: *const u8 = std::ptr::null::<u8>();
    let mut iNotNull: i32 = -(1 as i32);
    let mut ii: i32 = 0 as i32;
    let mut iOff: i32 = 0 as i32;
    let mut iStart: i32 = 0 as i32;
    let mut iEnd: i32 = 0 as i32;
    let mut zNew: *mut i8 = std::ptr::null_mut::<i8>();
    let mut t: i32 = 0 as i32;
    let mut db: *mut sqlite3 = unsafe { std::mem::zeroed() };
    NotUsed;
    if zSql == std::ptr::null::<u8>() {
        return;
    }
    // /* Jump past the "CREATE TABLE" bit. */
    if skipCreateTable(ctx, zSql, std::ptr::addr_of_mut!(iOff)) != (0 as i32) {
        return;
    }
    if (unsafe { sqlite3_value_type(unsafe { *unsafe { argv.offset((1 as i32) as isize) } }) })
        == (1 as i32)
    {
        iNotNull =
            unsafe { sqlite3_value_int(unsafe { *unsafe { argv.offset((1 as i32) as isize) } }) };
    } else {
        zCons =
            unsafe { sqlite3_value_text(unsafe { *unsafe { argv.offset((1 as i32) as isize) } }) };
    }
    // /* Search for the named constraint within column definitions. */
    ii = 0 as i32;
    '__slate_break_1321: loop {
        if !(iEnd == (0 as i32)) {
            break;
        }
        // /* Now parse the column or table constraint definition. Search
        //     ** for the token CONSTRAINT if this is a DROP CONSTRAINT command, or
        //     ** NOT in the right column if this is a DROP NOT NULL. */
        '__slate_break_1322: while (1 as i32) != (0 as i32) {
            iStart = iOff;
            let __v1474: i32 = iOff;
            let __v1475: i32 = __v1474
                + getConstraintToken(
                    unsafe { zSql.offset(iOff as isize) },
                    std::ptr::addr_of_mut!(t),
                );
            iOff = __v1475;
            if t == (120 as i32) && (zCons != std::ptr::null::<u8>() || iNotNull == ii) {
                // /* Check if this is the constraint we are searching for. */
                let mut nTok: i32 = 0 as i32;
                let mut cmp: i32 = 1 as i32;
                // /* Skip past any whitespace. */
                let __v1476: i32 = iOff;
                let __v1477: i32 = __v1476 + getWhitespace(unsafe { zSql.offset(iOff as isize) });
                iOff = __v1477;
                // /* Compare the next token - which may be quoted - with the name of
                //         ** the constraint being dropped.  */
                nTok = getConstraintToken(
                    unsafe { zSql.offset(iOff as isize) },
                    std::ptr::addr_of_mut!(t),
                );
                if zCons != std::ptr::null::<u8>() {
                    if quotedCompare(
                        ctx,
                        t,
                        unsafe { zSql.offset(iOff as isize) },
                        nTok,
                        zCons,
                        std::ptr::addr_of_mut!(cmp),
                    ) != (0 as i32)
                    {
                        return;
                    }
                }
                let __v1478: i32 = iOff;
                let __v1479: i32 = __v1478 + nTok;
                iOff = __v1479;
                // /* The next token is usually the first token of the constraint
                //         ** definition. This is enough to tell the type of the constraint -
                //         ** TK_NOT means it is a NOT NULL, TK_CHECK a CHECK constraint etc.
                //         **
                //         ** There is also the chance that the next token is TK_CONSTRAINT
                //         ** (or TK_DEFAULT or TK_COLLATE), for example if a table has been
                //         ** created as follows:
                //         **
                //         **    CREATE TABLE t1(cols, CONSTRAINT one CONSTRAINT two NOT NULL);
                //         **
                //         ** In this case, allow the "CONSTRAINT one" bit to be dropped by
                //         ** this command if that is what is requested, or to advance to
                //         ** the next iteration of the loop with &zSql[iOff] still pointing
                //         ** to the CONSTRAINT keyword.  */
                nTok = getConstraintToken(
                    unsafe { zSql.offset(iOff as isize) },
                    std::ptr::addr_of_mut!(t),
                );
                if t == (120 as i32)
                    || t == (121 as i32)
                    || t == (114 as i32)
                    || t == (25 as i32)
                    || t == (23 as i32)
                    || t == (96 as i32)
                    || t == (24 as i32)
                {
                    t = 125 as i32;
                } else {
                    let __v1480: i32 = iOff;
                    let __v1481: i32 = __v1480 + nTok;
                    iOff = __v1481;
                    let __v1482: i32 = iOff;
                    let __v1483: i32 =
                        __v1482 + getConstraint(unsafe { zSql.offset(iOff as isize) }, t);
                    iOff = __v1483;
                }
                if cmp == (0 as i32) || iNotNull >= (0 as i32) && t == (19 as i32) {
                    if t != (19 as i32)
                        && t != (125 as i32)
                        && t != (126 as i32)
                        && t != (133 as i32)
                    {
                        unsafe {
                            errorMPrintf(
                                ctx,
                                (b"constraint may not be dropped: %s\0".as_ptr() as *mut i8)
                                    as *const i8,
                                zCons,
                            )
                        };
                        return;
                    }
                    iEnd = iOff;
                    break '__slate_break_1322;
                }
            } else {
                if t == (19 as i32) && iNotNull == ii {
                    iEnd = iOff + getConstraint(unsafe { zSql.offset(iOff as isize) }, 0 as i32);
                    break '__slate_break_1322;
                } else {
                    if t == (23 as i32) || t == (186 as i32) {
                        iEnd = -(1 as i32);
                        break '__slate_break_1322;
                    } else {
                        if t == (25 as i32) {
                            break '__slate_break_1322;
                        }
                    }
                }
            }
        }
        let __v1472: i32 = ii;
        let __v1473: i32 = __v1472 + (1 as i32);
        ii = __v1473;
    }
    // /* If the constraint has not been found it is an error. */
    if iEnd <= (0 as i32) {
        if zCons != std::ptr::null::<u8>() {
            unsafe {
                errorMPrintf(
                    ctx,
                    (b"no such constraint: %s\0".as_ptr() as *mut i8) as *const i8,
                    zCons,
                )
            };
        } else {
            // /* SQLite follows postgres in that a DROP NOT NULL on a column that is
            //       ** not NOT NULL is not an error. So just return the original SQL here. */
            unsafe {
                sqlite3_result_text(ctx, zSql as *const i8, -(1 as i32), unsafe {
                    std::mem::transmute::<usize, Option<unsafe extern "C-unwind" fn(*mut ())>>(
                        -(1 as i32) as usize,
                    )
                })
            };
        }
    } else {
        // /* Figure out if an extra space should be inserted after the constraint
        //     ** is removed. And if an additional comma preceding the constraint
        //     ** should be removed. */
        let mut zSpace: *const i8 = (b" \0".as_ptr() as *mut i8) as *const i8;
        let __v1484: i32 = iEnd;
        let __v1485: i32 = __v1484 + getWhitespace(unsafe { zSql.offset(iEnd as isize) });
        iEnd = __v1485;
        unsafe {
            sqlite3GetToken(
                unsafe { zSql.offset(iEnd as isize) },
                std::ptr::addr_of_mut!(t),
            )
        };
        if t == (23 as i32) || t == (25 as i32) {
            zSpace = (b"\0".as_ptr() as *mut i8) as *const i8;
            if (((unsafe { *unsafe { zSql.offset((iStart - (1 as i32)) as isize) } }) as u32)
                as i32)
                == (44 as i32)
            {
                let __v1486: i32 = iStart;
                let __v1487: i32 = __v1486 - (1 as i32);
                iStart = __v1487;
            }
        }
        db = unsafe { sqlite3_context_db_handle(ctx) };
        zNew = unsafe {
            sqlite3MPrintf(
                db,
                (b"%.*s%s%s\0".as_ptr() as *mut i8) as *const i8,
                iStart,
                zSql,
                zSpace,
                unsafe { zSql.offset(iEnd as isize) },
            )
        };
        unsafe {
            sqlite3_result_text(ctx, zNew as *const i8, -(1 as i32), unsafe {
                std::mem::transmute::<*const (), Option<unsafe extern "C-unwind" fn(*mut ())>>(
                    sqlite3RowSetClear as *const (),
                )
            })
        };
    }
}

// /*
// ** Internal SQL function:
// **
// **     sqlite_add_constraint(SQL, CONSTRAINT-TEXT, ICOL)
// **
// ** SQL is a CREATE TABLE statement.  Return a modified version of
// ** SQL that adds CONSTRAINT-TEXT at the end of the ICOL-th column
// ** definition.  (The left-most column defintion is 0.)
// */
#[unsafe(link_section = ".text.slate_distinct.alter.addConstraintFunc")]
extern "C-unwind" fn addConstraintFunc(
    mut ctx: *mut sqlite3_context,
    mut NotUsed: i32,
    mut argv: *mut *mut sqlite3_value,
) {
    let mut zSql: *const u8 =
        unsafe { sqlite3_value_text(unsafe { *unsafe { argv.offset((0 as i32) as isize) } }) };
    let mut zCons: *const i8 =
        (unsafe { sqlite3_value_text(unsafe { *unsafe { argv.offset((1 as i32) as isize) } }) })
            as *const i8;
    let mut iCol: i32 =
        unsafe { sqlite3_value_int(unsafe { *unsafe { argv.offset((2 as i32) as isize) } }) };
    let mut iOff: i32 = 0 as i32;
    let mut ii: i32 = 0 as i32;
    let mut pNew: *mut sqlite3_str = unsafe { std::mem::zeroed() };
    let mut t: i32 = 0 as i32;
    NotUsed;
    if skipCreateTable(ctx, zSql, std::ptr::addr_of_mut!(iOff)) != (0 as i32) {
        return;
    }
    ii = 0 as i32;
    '__slate_break_1328: loop {
        if !(ii <= iCol || iCol < (0 as i32) && t != (23 as i32)) {
            break;
        }
        let __v1490: i32 = iOff;
        let __v1491: i32 = __v1490
            + getConstraintToken(
                unsafe { zSql.offset(iOff as isize) },
                std::ptr::addr_of_mut!(t),
            );
        iOff = __v1491;
        '__slate_break_1329: while (1 as i32) != (0 as i32) {
            let mut nTok: i32 = getConstraintToken(
                unsafe { zSql.offset(iOff as isize) },
                std::ptr::addr_of_mut!(t),
            );
            if t == (25 as i32) || t == (23 as i32) {
                break '__slate_break_1329;
            }
            if t == (186 as i32) {
                unsafe {
                    sqlite3_result_error_code(ctx, unsafe { sqlite3CorruptError(2698 as i32) })
                };
                return;
            }
            let __v1492: i32 = iOff;
            let __v1493: i32 = __v1492 + nTok;
            iOff = __v1493;
        }
        let __v1488: i32 = ii;
        let __v1489: i32 = __v1488 + (1 as i32);
        ii = __v1489;
    }
    let __v1494: i32 = iOff;
    let __v1495: i32 = __v1494 + getWhitespace(unsafe { zSql.offset(iOff as isize) });
    iOff = __v1495;
    pNew = unsafe { sqlite3_str_new(unsafe { sqlite3_context_db_handle(ctx) }) };
    unsafe { sqlite3_str_append(pNew, zSql as *const i8, iOff) };
    if iCol < (0 as i32) {
        unsafe { sqlite3_str_append(pNew, (b",\0".as_ptr() as *mut i8) as *const i8, 1 as i32) };
    }
    unsafe {
        sqlite3_str_appendf(
            pNew,
            (b" %s%s\0".as_ptr() as *mut i8) as *const i8,
            zCons,
            unsafe { zSql.offset(iOff as isize) },
        )
    };
    unsafe { sqlite3_result_str(ctx, pNew, 2 as i32) };
}

// /*
// ** Find a column named pCol in table pTab. If successful, set output
// ** parameter *piCol to the index of the column in the table and return
// ** SQLITE_OK. Otherwise, set *piCol to -1 and return an SQLite error
// ** code.
// */
fn alterFindCol(
    mut pParse: *mut Parse,
    mut pTab: *mut Table,
    mut pCol: *mut Token,
    mut piCol: *mut i32,
) -> i32 {
    let mut db: *mut sqlite3 = unsafe { (*pParse).db };
    let mut zName: *mut i8 = unsafe { sqlite3NameFromToken(db, pCol as *const Token) };
    let mut rc: i32 = 7 as i32;
    let mut iCol: i32 = -(1 as i32);
    if zName != std::ptr::null_mut::<i8>() {
        iCol = unsafe { sqlite3ColumnIndex(pTab, zName as *const i8) };
        if iCol < (0 as i32) {
            unsafe {
                sqlite3ErrorMsg(
                    pParse,
                    (b"no such column: %s\0".as_ptr() as *mut i8) as *const i8,
                    zName,
                )
            };
            rc = 1 as i32;
        } else {
            rc = 0 as i32;
        }
    }
    if rc == (0 as i32) {
        let mut zDb: *const i8 = (unsafe {
            (*unsafe {
                unsafe { (*db).aDb }.offset(
                    (unsafe { sqlite3SchemaToIndex(db, unsafe { (*pTab).pSchema }) }) as isize,
                )
            })
            .zDbSName
        }) as *const i8;
        let mut zCol: *const i8 =
            (unsafe { (*unsafe { unsafe { (*pTab).aCol }.offset(iCol as isize) }).zCnName })
                as *const i8;
        if (unsafe {
            sqlite3AuthCheck(
                pParse,
                26 as i32,
                zDb,
                (unsafe { (*pTab).zName }) as *const i8,
                zCol,
            )
        }) != (0 as i32)
        {
            pTab = std::ptr::null_mut::<Table>();
        }
    }
    unsafe { sqlite3DbFree(db, zName as *mut ()) };
    unsafe {
        *piCol = iCol;
    }
    return rc;
}

// /*
// ** Find the table named by the first entry in source list pSrc. If successful,
// ** return a pointer to the Table structure and set output variable (*pzDb)
// ** to point to the name of the database containin the table (i.e. "main",
// ** "temp" or the name of an attached database).
// **
// ** If the table cannot be located, return NULL. The value of the two output
// ** parameters is undefined in this case.
// */
fn alterFindTable(
    mut pParse: *mut Parse,
    mut pSrc: *mut SrcList,
    mut piDb: *mut i32,
    mut pzDb: *mut *const i8,
    mut bAuth: i32,
) -> *mut Table {
    let mut db: *mut sqlite3 = unsafe { (*pParse).db };
    let mut pTab: *mut Table = std::ptr::null_mut::<Table>();
    0 as i32;
    pTab = unsafe {
        sqlite3LocateTableItem(pParse, (0 as i32) as u32, unsafe {
            unsafe { std::ptr::addr_of_mut!((*pSrc).a) as *mut SrcItem }.offset((0 as i32) as isize)
        })
    };
    if pTab != std::ptr::null_mut::<Table>() {
        let mut iDb: i32 = unsafe { sqlite3SchemaToIndex(db, unsafe { (*pTab).pSchema }) };
        unsafe {
            *pzDb = (unsafe { (*unsafe { unsafe { (*db).aDb }.offset(iDb as isize) }).zDbSName })
                as *const i8;
        }
        unsafe {
            *piDb = iDb;
        }
        let __v1496: bool;
        if (0 as i32) != isRealTable(pParse, pTab, 2 as i32) {
            __v1496 = true as bool;
        } else {
            __v1496 = (0 as i32) != isAlterableTable(pParse, pTab);
        }
        if __v1496 {
            pTab = std::ptr::null_mut::<Table>();
        }
    }
    if pTab != std::ptr::null_mut::<Table>() && bAuth != (0 as i32) {
        if (unsafe {
            sqlite3AuthCheck(
                pParse,
                26 as i32,
                unsafe { *pzDb },
                (unsafe { (*pTab).zName }) as *const i8,
                std::ptr::null::<i8>(),
            )
        }) != (0 as i32)
        {
            pTab = std::ptr::null_mut::<Table>();
        }
    }
    unsafe { sqlite3SrcListDelete(db, pSrc) };
    return pTab;
}

// /* Parsing context */
// /* The table being altered */
// /* Name of the constraint to drop */
// /* Name of the column from which to remove the NOT NULL */
// /*
// ** The implementation of SQL function sqlite_fail(MSG). This takes a single
// ** argument, and returns it as an error message with the error code set to
// ** SQLITE_CONSTRAINT.
// */
#[unsafe(link_section = ".text.slate_distinct.alter.failConstraintFunc")]
extern "C-unwind" fn failConstraintFunc(
    mut ctx: *mut sqlite3_context,
    mut NotUsed: i32,
    mut argv: *mut *mut sqlite3_value,
) {
    let mut zText: *const i8 =
        (unsafe { sqlite3_value_text(unsafe { *unsafe { argv.offset((0 as i32) as isize) } }) })
            as *const i8;
    let mut err: i32 =
        unsafe { sqlite3_value_int(unsafe { *unsafe { argv.offset((1 as i32) as isize) } }) };
    NotUsed;
    unsafe { sqlite3_result_error(ctx, zText, -(1 as i32)) };
    unsafe { sqlite3_result_error_code(ctx, err) };
}

// /*
// ** Buffer pCons, which is nCons bytes in size, contains the text of a
// ** NOT NULL or CHECK constraint that will be inserted into a CREATE TABLE
// ** statement. If successful, this function returns the size of the buffer in
// ** bytes not including any trailing whitespace or "--" style comments. Or,
// ** if an OOM occurs, it returns 0 and sets db->mallocFailed to true.
// **
// ** C-style comments at the end are preserved.  "--" style comments are
// ** removed because the comment terminator might be \000, and we are about
// ** to insert the pCons[] text into the middle of a larger string, and that
// ** will have the effect of removing the comment terminator and messing up
// ** the syntax.
// */
fn alterRtrimConstraint(mut db: *mut sqlite3, mut pCons: *const i8, mut nCons: i32) -> i32 {
    let mut zTmp: *mut u8 = (unsafe {
        sqlite3MPrintf(
            db,
            (b"%.*s\0".as_ptr() as *mut i8) as *const i8,
            nCons,
            pCons,
        )
    }) as *mut u8;
    let mut iOff: i32 = 0 as i32;
    let mut iEnd: i32 = 0 as i32;
    if zTmp == std::ptr::null_mut::<u8>() {
        return 0 as i32;
    }
    '__slate_break_1337: while (1 as i32) != (0 as i32) {
        let mut t: i32 = 0 as i32;
        let mut nToken: i32 = (unsafe {
            sqlite3GetToken(
                (unsafe { zTmp.offset(iOff as isize) }) as *const u8,
                std::ptr::addr_of_mut!(t),
            )
        }) as i32;
        if t == (186 as i32) {
            break '__slate_break_1337;
        }
        if t != (184 as i32)
            && (t != (185 as i32)
                || (((unsafe { *unsafe { zTmp.offset(iOff as isize) } }) as u32) as i32)
                    != (45 as i32))
        {
            iEnd = iOff + nToken;
        }
        let __v1497: i32 = iOff;
        let __v1498: i32 = __v1497 + nToken;
        iOff = __v1498;
    }
    unsafe { sqlite3DbFree(db, zTmp as *mut ()) };
    return iEnd;
}

// /* Parsing context */
// /* Name of the table being altered */
// /* Name of the column to add a NOT NULL constraint to */
// /* The NOT token of the NOT NULL constraint text */
// /*
// ** Implementation of internal SQL function:
// **
// **     sqlite_find_constraint(SQL, CONSTRAINT-NAME)
// **
// ** This function returns true if the SQL passed as the first argument is a
// ** CREATE TABLE that contains a constraint with the name CONSTRAINT-NAME,
// ** or false otherwise.
// */
#[unsafe(link_section = ".text.slate_distinct.alter.findConstraintFunc")]
extern "C-unwind" fn findConstraintFunc(
    mut ctx: *mut sqlite3_context,
    mut NotUsed: i32,
    mut argv: *mut *mut sqlite3_value,
) {
    let mut zSql: *const u8 = std::ptr::null::<u8>();
    let mut zCons: *const u8 = std::ptr::null::<u8>();
    let mut iOff: i32 = 0 as i32;
    let mut t: i32 = 0 as i32;
    NotUsed;
    zSql = unsafe { sqlite3_value_text(unsafe { *unsafe { argv.offset((0 as i32) as isize) } }) };
    zCons = unsafe { sqlite3_value_text(unsafe { *unsafe { argv.offset((1 as i32) as isize) } }) };
    if zSql == std::ptr::null::<u8>() || zCons == std::ptr::null::<u8>() {
        return;
    }
    '__slate_break_1340: while t != (22 as i32) && t != (186 as i32) {
        let __v1499: i32 = iOff;
        let __v1500: i32 = ((__v1499 as i64)
            + unsafe {
                sqlite3GetToken(
                    unsafe { zSql.offset(iOff as isize) },
                    std::ptr::addr_of_mut!(t),
                )
            }) as i32;
        iOff = __v1500;
    }
    '__slate_break_1341: while (1 as i32) != (0 as i32) {
        let __v1501: i32 = iOff;
        let __v1502: i32 = __v1501
            + getConstraintToken(
                unsafe { zSql.offset(iOff as isize) },
                std::ptr::addr_of_mut!(t),
            );
        iOff = __v1502;
        if t == (120 as i32) {
            let mut nTok: i32 = 0 as i32;
            let mut cmp: i32 = 0 as i32;
            let __v1503: i32 = iOff;
            let __v1504: i32 = __v1503 + getWhitespace(unsafe { zSql.offset(iOff as isize) });
            iOff = __v1504;
            nTok = getConstraintToken(
                unsafe { zSql.offset(iOff as isize) },
                std::ptr::addr_of_mut!(t),
            );
            if quotedCompare(
                ctx,
                t,
                unsafe { zSql.offset(iOff as isize) },
                nTok,
                zCons,
                std::ptr::addr_of_mut!(cmp),
            ) != (0 as i32)
            {
                return;
            }
            if cmp == (0 as i32) {
                unsafe { sqlite3_result_int(ctx, 1 as i32) };
                return;
            }
        } else {
            if t == (186 as i32) {
                break '__slate_break_1341;
            }
        }
    }
    unsafe { sqlite3_result_int(ctx, 0 as i32) };
}
