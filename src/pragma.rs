//! 2003 April 6
//!
//! The author disclaims copyright to this source code.  In place of
//! a legal notice, here is a blessing:
//!
//!    May you do good and not evil.
//!    May you find forgiveness for yourself and forgive others.
//!    May you share freely, never taking more than you give.
//!
//!
//! This file contains code used to implement the PRAGMA command.
unsafe extern "C" {
    static mut sqlite3_temp_directory: *mut i8;
    static mut sqlite3StrBINARY: [i8; 0];
    static mut sqlite3StdType: [*const i8; 0];
    static mut sqlite3UpperToLower: [u8; 0];
    static mut sqlite3CtypeMap: [u8; 0];
    static mut sqlite3Config: Sqlite3Config;
    static mut sqlite3BuiltinFunctions: FuncDefHash;
    fn sqlite3_compileoption_get(N: i32) -> *const i8;
    fn sqlite3_busy_timeout(__v866: *mut sqlite3, ms: i32) -> i32;
    fn sqlite3_mprintf(__v868: *const i8, ...) -> *mut i8;
    fn sqlite3_malloc(__v869: i32) -> *mut ();
    fn sqlite3_free(__v870: *mut ());
    fn sqlite3_errmsg(__v871: *mut sqlite3) -> *const i8;
    fn sqlite3_limit(__v872: *mut sqlite3, id: i32, newVal: i32) -> i32;
    fn sqlite3_prepare_v2(
        db: *mut sqlite3,
        zSql: *const i8,
        nByte: i32,
        ppStmt: *mut *mut sqlite3_stmt,
        pzTail: *mut *const i8,
    ) -> i32;
    fn sqlite3_prepare_v3(
        db: *mut sqlite3,
        zSql: *const i8,
        nByte: i32,
        prepFlags: u32,
        ppStmt: *mut *mut sqlite3_stmt,
        pzTail: *mut *const i8,
    ) -> i32;
    fn sqlite3_step(__v886: *mut sqlite3_stmt) -> i32;
    fn sqlite3_column_value(__v887: *mut sqlite3_stmt, iCol: i32) -> *mut sqlite3_value;
    fn sqlite3_finalize(pStmt: *mut sqlite3_stmt) -> i32;
    fn sqlite3_value_text(__v890: *mut sqlite3_value) -> *const u8;
    fn sqlite3_value_type(__v891: *mut sqlite3_value) -> i32;
    fn sqlite3_result_text(
        __v892: *mut sqlite3_context,
        __v893: *const i8,
        __v894: i32,
        __v895: Option<unsafe extern "C-unwind" fn(*mut ())>,
    );
    fn sqlite3_result_value(__v896: *mut sqlite3_context, __v897: *mut sqlite3_value);
    fn sqlite3_db_release_memory(__v898: *mut sqlite3) -> i32;
    fn sqlite3_soft_heap_limit64(N: i64) -> i64;
    fn sqlite3_hard_heap_limit64(N: i64) -> i64;
    fn sqlite3_declare_vtab(__v901: *mut sqlite3, zSQL: *const i8) -> i32;
    fn sqlite3_mutex_enter(__v903: *mut sqlite3_mutex);
    fn sqlite3_mutex_leave(__v904: *mut sqlite3_mutex);
    fn sqlite3_file_control(
        __v905: *mut sqlite3,
        zDbName: *const i8,
        op: i32,
        __v908: *mut (),
    ) -> i32;
    fn sqlite3_str_appendf(__v909: *mut sqlite3_str, zFormat: *const i8, ...);
    fn sqlite3_str_append(__v911: *mut sqlite3_str, zIn: *const i8, N: i32);
    fn sqlite3_str_appendall(__v914: *mut sqlite3_str, zIn: *const i8);
    fn sqlite3_stricmp(__v916: *const i8, __v917: *const i8) -> i32;
    fn sqlite3_strnicmp(__v918: *const i8, __v919: *const i8, __v920: i32) -> i32;
    fn sqlite3_wal_autocheckpoint(db: *mut sqlite3, N: i32) -> i32;
    fn sqlite3HashFind(__v923: *const Hash, pKey: *const i8) -> *mut ();
    fn memset(__s: *mut (), __c: i32, __n: u64) -> *mut ();
    fn sqlite3OsAccess(
        __v928: *mut sqlite3_vfs,
        __v929: *const i8,
        __v930: i32,
        pResOut: *mut i32,
    ) -> i32;
    fn sqlite3PagerLockingMode(__v932: *mut Pager, __v933: i32) -> i32;
    fn sqlite3PagerJournalSizeLimit(__v934: *mut Pager, __v935: i64) -> i64;
    fn sqlite3BtreeClose(__v936: *mut Btree) -> i32;
    fn sqlite3BtreeSetCacheSize(__v937: *mut Btree, __v938: i32) -> i32;
    fn sqlite3BtreeSetSpillSize(__v939: *mut Btree, __v940: i32) -> i32;
    fn sqlite3BtreeSetMmapLimit(__v941: *mut Btree, __v942: i64) -> i32;
    fn sqlite3BtreeSetPagerFlags(__v943: *mut Btree, __v944: u32) -> i32;
    fn sqlite3BtreeSetPageSize(p: *mut Btree, nPagesize: i32, nReserve: i32, eFix: i32) -> i32;
    fn sqlite3BtreeGetPageSize(__v949: *mut Btree) -> i32;
    fn sqlite3BtreeSecureDelete(__v950: *mut Btree, __v951: i32) -> i32;
    fn sqlite3BtreeSetAutoVacuum(__v952: *mut Btree, __v953: i32) -> i32;
    fn sqlite3BtreeGetAutoVacuum(__v954: *mut Btree) -> i32;
    fn sqlite3BtreeTxnState(__v955: *mut Btree) -> i32;
    fn sqlite3BtreeGetFilename(__v956: *mut Btree) -> *const i8;
    fn sqlite3BtreePager(__v957: *mut Btree) -> *mut Pager;
    fn sqlite3VdbeAddOp0(__v958: *mut Vdbe, __v959: i32) -> i32;
    fn sqlite3VdbeAddOp1(__v960: *mut Vdbe, __v961: i32, __v962: i32) -> i32;
    fn sqlite3VdbeAddOp2(__v963: *mut Vdbe, __v964: i32, __v965: i32, __v966: i32) -> i32;
    fn sqlite3VdbeGoto(__v967: *mut Vdbe, __v968: i32) -> i32;
    fn sqlite3VdbeLoadString(__v969: *mut Vdbe, __v970: i32, __v971: *const i8) -> i32;
    fn sqlite3VdbeMultiLoad(__v972: *mut Vdbe, __v973: i32, __v974: *const i8, ...);
    fn sqlite3VdbeAddOp3(
        __v975: *mut Vdbe,
        __v976: i32,
        __v977: i32,
        __v978: i32,
        __v979: i32,
    ) -> i32;
    fn sqlite3VdbeAddInt64(__v980: *mut Vdbe, __v981: i32, __v982: i64) -> i32;
    fn sqlite3VdbeAddOp4(
        __v983: *mut Vdbe,
        __v984: i32,
        __v985: i32,
        __v986: i32,
        __v987: i32,
        zP4: *const i8,
        __v989: i32,
    ) -> i32;
    fn sqlite3VdbeAddOp4Int(
        __v990: *mut Vdbe,
        __v991: i32,
        __v992: i32,
        __v993: i32,
        __v994: i32,
        __v995: i32,
    ) -> i32;
    fn sqlite3VdbeAddOpList(
        __v996: *mut Vdbe,
        nOp: i32,
        aOp: *const VdbeOpList,
        iLineno: i32,
    ) -> *mut VdbeOp;
    fn sqlite3VdbeChangeP3(__v1000: *mut Vdbe, addr: i32, P3: i32);
    fn sqlite3VdbeChangeP5(__v1003: *mut Vdbe, P5: u16);
    fn sqlite3VdbeTypeofColumn(__v1005: *mut Vdbe, __v1006: i32);
    fn sqlite3VdbeJumpHere(__v1007: *mut Vdbe, addr: i32);
    fn sqlite3VdbeChangeP4(__v1009: *mut Vdbe, addr: i32, zP4: *const i8, N: i32);
    fn sqlite3VdbeAppendP4(__v1013: *mut Vdbe, pP4: *mut (), p4type: i32);
    fn sqlite3VdbeSetP4KeyInfo(__v1016: *mut Parse, __v1017: *mut Index);
    fn sqlite3VdbeUsesBtree(__v1018: *mut Vdbe, __v1019: i32);
    fn sqlite3VdbeGetOp(__v1020: *mut Vdbe, __v1021: i32) -> *mut VdbeOp;
    fn sqlite3VdbeMakeLabel(__v1022: *mut Parse) -> i32;
    fn sqlite3VdbeRunOnlyOnce(__v1023: *mut Vdbe);
    fn sqlite3VdbeReusable(__v1024: *mut Vdbe);
    fn sqlite3VdbeResolveLabel(__v1025: *mut Vdbe, __v1026: i32);
    fn sqlite3VdbeCurrentAddr(__v1027: *mut Vdbe) -> i32;
    fn sqlite3VdbeSetNumCols(__v1028: *mut Vdbe, __v1029: i32);
    fn sqlite3VdbeSetColName(
        __v1030: *mut Vdbe,
        __v1031: i32,
        __v1032: i32,
        __v1033: *const i8,
        __v1034: Option<unsafe extern "C-unwind" fn(*mut ())>,
    ) -> i32;
    fn sqlite3StrICmp(__v1035: *const i8, __v1036: *const i8) -> i32;
    fn sqlite3Strlen30(__v1037: *const i8) -> i32;
    fn sqlite3ColumnType(__v1038: *mut Column, __v1039: *mut i8) -> *mut i8;
    fn sqlite3DbMallocRawNN(__v1040: *mut sqlite3, __v1041: u64) -> *mut ();
    fn sqlite3DbFree(__v1042: *mut sqlite3, __v1043: *mut ());
    fn sqlite3MutexAlloc(__v1044: i32) -> *mut sqlite3_mutex;
    fn sqlite3MPrintf(__v1045: *mut sqlite3, __v1046: *const i8, ...) -> *mut i8;
    fn sqlite3ErrorMsg(__v1047: *mut Parse, __v1048: *const i8, ...);
    fn sqlite3GetTempReg(__v1049: *mut Parse) -> i32;
    fn sqlite3GetTempRange(__v1050: *mut Parse, __v1051: i32) -> i32;
    fn sqlite3ReleaseTempRange(__v1052: *mut Parse, __v1053: i32, __v1054: i32);
    fn sqlite3ClearTempRegCache(__v1055: *mut Parse);
    fn sqlite3TouchRegister(__v1056: *mut Parse, __v1057: i32);
    fn sqlite3ExprListDelete(__v1058: *mut sqlite3, __v1059: *mut ExprList);
    fn sqlite3ResetAllSchemasOfConnection(__v1067: *mut sqlite3);
    fn sqlite3ColumnExpr(__v1068: *mut Table, __v1069: *mut Column) -> *mut Expr;
    fn sqlite3PrimaryKeyIndex(__v1070: *mut Table) -> *mut Index;
    fn sqlite3TableColumnToIndex(__v1071: *mut Index, __v1072: i32) -> i32;
    fn sqlite3TableColumnToStorage(__v1073: *mut Table, __v1074: i16) -> i16;
    fn sqlite3ViewGetColumnNames(__v1075: *mut Parse, __v1076: *mut Table) -> i32;
    fn sqlite3OpenTable(
        __v1077: *mut Parse,
        iCur: i32,
        iDb: i32,
        __v1080: *mut Table,
        __v1081: i32,
    );
    fn sqlite3ExprCodeLoadIndexColumn(
        __v1082: *mut Parse,
        __v1083: *mut Index,
        __v1084: i32,
        __v1085: i32,
        __v1086: i32,
    );
    fn sqlite3ExprCodeGetColumnOfTable(
        __v1087: *mut Vdbe,
        __v1088: *mut Table,
        __v1089: i32,
        __v1090: i32,
        __v1091: i32,
    );
    fn sqlite3ExprIfTrue(__v1092: *mut Parse, __v1093: *mut Expr, __v1094: i32, __v1095: i32);
    fn sqlite3ExprIfFalse(__v1096: *mut Parse, __v1097: *mut Expr, __v1098: i32, __v1099: i32);
    fn sqlite3FindTable(
        __v1100: *mut sqlite3,
        __v1101: *const i8,
        __v1102: *const i8,
    ) -> *mut Table;
    fn sqlite3LocateTable(
        __v1103: *mut Parse,
        flags: u32,
        __v1105: *const i8,
        __v1106: *const i8,
    ) -> *mut Table;
    fn sqlite3PreferredTableName(__v1107: *const i8) -> *const i8;
    fn sqlite3FindIndex(
        __v1108: *mut sqlite3,
        __v1109: *const i8,
        __v1110: *const i8,
    ) -> *mut Index;
    fn sqlite3NameFromToken(__v1111: *mut sqlite3, __v1112: *const Token) -> *mut i8;
    fn sqlite3GetVdbe(__v1113: *mut Parse) -> *mut Vdbe;
    fn sqlite3CodeVerifySchema(__v1114: *mut Parse, __v1115: i32);
    fn sqlite3CodeVerifyNamedSchema(__v1116: *mut Parse, zDb: *const i8);
    fn sqlite3GenerateIndexKey(
        __v1118: *mut Parse,
        __v1119: *mut Index,
        __v1120: i32,
        __v1121: i32,
        __v1122: i32,
        __v1123: *mut i32,
        __v1124: *mut Index,
        __v1125: i32,
    ) -> i32;
    fn sqlite3ResolvePartIdxLabel(__v1126: *mut Parse, __v1127: i32);
    fn sqlite3OpenTableAndIndices(
        __v1128: *mut Parse,
        __v1129: *mut Table,
        __v1130: i32,
        __v1131: u8,
        __v1132: i32,
        __v1133: *mut u8,
        __v1134: *mut i32,
        __v1135: *mut i32,
    ) -> i32;
    fn sqlite3BeginWriteOperation(__v1136: *mut Parse, __v1137: i32, __v1138: i32);
    fn sqlite3ExprListDup(
        __v1139: *mut sqlite3,
        __v1140: *const ExprList,
        __v1141: i32,
    ) -> *mut ExprList;
    fn sqlite3AuthCheck(
        __v1142: *mut Parse,
        __v1143: i32,
        __v1144: *const i8,
        __v1145: *const i8,
        __v1146: *const i8,
    ) -> i32;
    fn sqlite3GetInt32(__v1147: *const i8, __v1148: *mut i32) -> i32;
    fn sqlite3Atoi(__v1149: *const i8) -> i32;
    fn sqlite3IndexAffinityStr(__v1150: *mut sqlite3, __v1151: *mut Index) -> *const i8;
    fn sqlite3DecOrHexToI64(__v1152: *const i8, __v1153: *mut i64) -> i32;
    fn sqlite3TwoPartName(
        __v1154: *mut Parse,
        __v1155: *mut Token,
        __v1156: *mut Token,
        __v1157: *mut *mut Token,
    ) -> i32;
    fn sqlite3ErrStr(__v1158: i32) -> *const i8;
    fn sqlite3ReadSchema(pParse: *mut Parse) -> i32;
    fn sqlite3SetTextEncoding(db: *mut sqlite3, __v1161: u8);
    fn sqlite3AbsInt32(__v1162: i32) -> i32;
    fn sqlite3ValueFree(__v1165: *mut sqlite3_value);
    fn sqlite3ValueFromExpr(
        __v1166: *mut sqlite3,
        __v1167: *const Expr,
        __v1168: u8,
        __v1169: u8,
        __v1170: *mut *mut sqlite3_value,
    ) -> i32;
    fn sqlite3ColumnDefault(__v1171: *mut Vdbe, __v1172: *mut Table, __v1173: i32, __v1174: i32);
    fn sqlite3RegisterLikeFunctions(__v1175: *mut sqlite3, __v1176: i32);
    fn sqlite3SchemaToIndex(db: *mut sqlite3, __v1178: *mut Schema) -> i32;
    fn sqlite3OomFault(__v1179: *mut sqlite3) -> *mut ();
    fn sqlite3OpenTempDatabase(__v1180: *mut Parse) -> i32;
    fn sqlite3StrAccumInit(
        __v1181: *mut sqlite3_str,
        __v1182: *mut sqlite3,
        __v1183: *mut i8,
        __v1184: i32,
        __v1185: i32,
    );
    fn sqlite3StrAccumFinish(__v1186: *mut sqlite3_str) -> *mut i8;
    fn sqlite3TableLock(
        __v1187: *mut Parse,
        __v1188: i32,
        __v1189: u32,
        __v1190: u8,
        __v1191: *const i8,
    );
    fn sqlite3VtabCreateModule(
        __v1192: *mut sqlite3,
        __v1193: *const i8,
        __v1194: *const sqlite3_module,
        __v1195: *mut (),
        __v1196: Option<unsafe extern "C-unwind" fn(*mut ())>,
    ) -> *mut Module;
    fn sqlite3WalDefaultHook(
        __v1198: *mut (),
        __v1199: *mut sqlite3,
        __v1200: *const i8,
        __v1201: i32,
    ) -> i32;
    fn sqlite3FkLocateIndex(
        __v1202: *mut Parse,
        __v1203: *mut Table,
        __v1204: *mut FKey,
        __v1205: *mut *mut Index,
        __v1206: *mut *mut i32,
    ) -> i32;
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
struct sqlite3_pcache {}

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
struct VdbeOpList {
    opcode: u8,
    p1: i8,
    p2: i8,
    p3: i8,
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
struct FuncDefHash {
    a: [*mut FuncDef; 23],
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
    trace: __SlateRecord177,
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
    u1: __SlateRecord178,
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
    u: __SlateRecord179,
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
    __slate_bits_0: __slate_bits::__SlateBits77U0,
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
    u: __SlateRecord180,
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
    __slate_bits_0: __slate_bits::__SlateBits103U0,
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
    u: __SlateRecord188,
    pLeft: *mut Expr,
    pRight: *mut Expr,
    x: __SlateRecord189,
    nHeight: i32,
    iTable: i32,
    iColumn: i16,
    iAgg: i16,
    w: __SlateRecord190,
    pAggInfo: *mut AggInfo,
    y: __SlateRecord191,
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
    fg: __SlateRecord198,
    iCursor: i32,
    colUsed: u64,
    u1: __SlateRecord199,
    u2: __SlateRecord200,
    u3: __SlateRecord201,
    u4: __SlateRecord202,
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
    __slate_bits_0: __slate_bits::__SlateBits115U0,
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
    u1: __SlateRecord204,
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
struct Pager {}

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
struct DbClientData {
    pNext: *mut DbClientData,
    pData: *mut (),
    xDestructor: Option<unsafe extern "C-unwind" fn(*mut ())>,
    zName: [i8; 0],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct sqlite3InitInfo {
    newTnum: u32,
    iDb: u8,
    busy: u8,
    __slate_bits_0: __slate_bits::__SlateBits176U0,
    azInit: *mut *const i8,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord177 {
    xLegacy: Option<unsafe extern "C-unwind" fn(*mut (), *const i8)>,
    xV2: Option<unsafe extern "C-unwind" fn(u32, *mut (), *mut (), *mut ()) -> i32>,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord178 {
    isInterrupted: i32,
    notUsed1: f64,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord179 {
    pHash: *mut FuncDef,
    pDestructor: *mut FuncDestructor,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord180 {
    tab: __SlateRecord181,
    view: __SlateRecord182,
    vtab: __SlateRecord183,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord181 {
    addColOffset: i32,
    pFKey: *mut FKey,
    pDfltList: *mut ExprList,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord182 {
    pSelect: *mut Select,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord183 {
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
union __SlateRecord188 {
    zToken: *mut i8,
    iValue: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord189 {
    pList: *mut ExprList,
    pSelect: *mut Select,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord190 {
    iJoin: i32,
    iOfst: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord191 {
    pTab: *mut Table,
    pWin: *mut Window,
    nReg: i32,
    sub: __SlateRecord192,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord192 {
    iAddr: i32,
    regReturn: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct ExprList_item {
    pExpr: *mut Expr,
    zEName: *mut i8,
    fg: __SlateRecord194,
    u: __SlateRecord195,
}

#[repr(C, align(4))]
#[derive(Clone, Copy)]
struct __SlateRecord194 {
    sortFlags: u8,
    __slate_bits_0: __slate_bits::__SlateBits194U0,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord195 {
    x: __SlateRecord196,
    iConstExprReg: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord196 {
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
struct __SlateRecord198 {
    jointype: u8,
    __slate_bits_0: __slate_bits::__SlateBits198U0,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord199 {
    zIndexedBy: *mut i8,
    pFuncArg: *mut ExprList,
    nRow: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord200 {
    pIBIndex: *mut Index,
    pCteUse: *mut CteUse,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord201 {
    pOn: *mut Expr,
    pUsing: *mut IdList,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord202 {
    pSchema: *mut Schema,
    zDatabase: *mut i8,
    pSubq: *mut Subquery,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord204 {
    cr: __SlateRecord205,
    d: __SlateRecord206,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord205 {
    addrCrTab: i32,
    regRowid: i32,
    regRoot: i32,
    constraintName: Token,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord206 {
    pReturning: *mut Returning,
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

static mut pragCName: __SlateAlign16<[*const i8; 57]> = __SlateAlign16([
    (b"id\0".as_ptr() as *mut i8) as *const i8,
    (b"seq\0".as_ptr() as *mut i8) as *const i8,
    (b"table\0".as_ptr() as *mut i8) as *const i8,
    (b"from\0".as_ptr() as *mut i8) as *const i8,
    (b"to\0".as_ptr() as *mut i8) as *const i8,
    (b"on_update\0".as_ptr() as *mut i8) as *const i8,
    (b"on_delete\0".as_ptr() as *mut i8) as *const i8,
    (b"match\0".as_ptr() as *mut i8) as *const i8,
    (b"cid\0".as_ptr() as *mut i8) as *const i8,
    (b"name\0".as_ptr() as *mut i8) as *const i8,
    (b"type\0".as_ptr() as *mut i8) as *const i8,
    (b"notnull\0".as_ptr() as *mut i8) as *const i8,
    (b"dflt_value\0".as_ptr() as *mut i8) as *const i8,
    (b"pk\0".as_ptr() as *mut i8) as *const i8,
    (b"hidden\0".as_ptr() as *mut i8) as *const i8,
    (b"name\0".as_ptr() as *mut i8) as *const i8,
    (b"builtin\0".as_ptr() as *mut i8) as *const i8,
    (b"type\0".as_ptr() as *mut i8) as *const i8,
    (b"enc\0".as_ptr() as *mut i8) as *const i8,
    (b"narg\0".as_ptr() as *mut i8) as *const i8,
    (b"flags\0".as_ptr() as *mut i8) as *const i8,
    (b"schema\0".as_ptr() as *mut i8) as *const i8,
    (b"name\0".as_ptr() as *mut i8) as *const i8,
    (b"type\0".as_ptr() as *mut i8) as *const i8,
    (b"ncol\0".as_ptr() as *mut i8) as *const i8,
    (b"wr\0".as_ptr() as *mut i8) as *const i8,
    (b"strict\0".as_ptr() as *mut i8) as *const i8,
    (b"seqno\0".as_ptr() as *mut i8) as *const i8,
    (b"cid\0".as_ptr() as *mut i8) as *const i8,
    (b"name\0".as_ptr() as *mut i8) as *const i8,
    (b"desc\0".as_ptr() as *mut i8) as *const i8,
    (b"coll\0".as_ptr() as *mut i8) as *const i8,
    (b"key\0".as_ptr() as *mut i8) as *const i8,
    (b"seq\0".as_ptr() as *mut i8) as *const i8,
    (b"name\0".as_ptr() as *mut i8) as *const i8,
    (b"unique\0".as_ptr() as *mut i8) as *const i8,
    (b"origin\0".as_ptr() as *mut i8) as *const i8,
    (b"partial\0".as_ptr() as *mut i8) as *const i8,
    (b"tbl\0".as_ptr() as *mut i8) as *const i8,
    (b"idx\0".as_ptr() as *mut i8) as *const i8,
    (b"wdth\0".as_ptr() as *mut i8) as *const i8,
    (b"hght\0".as_ptr() as *mut i8) as *const i8,
    (b"flgs\0".as_ptr() as *mut i8) as *const i8,
    (b"table\0".as_ptr() as *mut i8) as *const i8,
    (b"rowid\0".as_ptr() as *mut i8) as *const i8,
    (b"parent\0".as_ptr() as *mut i8) as *const i8,
    (b"fkid\0".as_ptr() as *mut i8) as *const i8,
    (b"busy\0".as_ptr() as *mut i8) as *const i8,
    (b"log\0".as_ptr() as *mut i8) as *const i8,
    (b"checkpointed\0".as_ptr() as *mut i8) as *const i8,
    (b"seq\0".as_ptr() as *mut i8) as *const i8,
    (b"name\0".as_ptr() as *mut i8) as *const i8,
    (b"file\0".as_ptr() as *mut i8) as *const i8,
    (b"database\0".as_ptr() as *mut i8) as *const i8,
    (b"status\0".as_ptr() as *mut i8) as *const i8,
    (b"cache_size\0".as_ptr() as *mut i8) as *const i8,
    (b"timeout\0".as_ptr() as *mut i8) as *const i8,
]);

#[repr(C)]
#[derive(Clone, Copy)]
struct EncName {
    zName: *mut i8,
    enc: u8,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct PragmaName {
    zName: *const i8,
    ePragTyp: u8,
    mPragFlg: u8,
    iPragCName: u8,
    nPragCName: u8,
    iArg: u64,
}

static mut aPragmaName: __SlateAlign16<[PragmaName; 66]> = __SlateAlign16([
    PragmaName {
        zName: (b"analysis_limit\0".as_ptr() as *mut i8) as *const i8,
        ePragTyp: ((1 as i32) as i8) as u8,
        mPragFlg: ((16 as i32) as i8) as u8,
        iPragCName: ((0 as i32) as i8) as u8,
        nPragCName: ((0 as i32) as i8) as u8,
        iArg: ((0 as i32) as i64) as u64,
    },
    PragmaName {
        zName: (b"application_id\0".as_ptr() as *mut i8) as *const i8,
        ePragTyp: ((2 as i32) as i8) as u8,
        mPragFlg: (((4 as i32) | (16 as i32)) as i8) as u8,
        iPragCName: ((0 as i32) as i8) as u8,
        nPragCName: ((0 as i32) as i8) as u8,
        iArg: ((8 as i32) as i64) as u64,
    },
    PragmaName {
        zName: (b"auto_vacuum\0".as_ptr() as *mut i8) as *const i8,
        ePragTyp: ((3 as i32) as i8) as u8,
        mPragFlg: (((1 as i32) | (16 as i32) | (128 as i32) | (4 as i32)) as i8) as u8,
        iPragCName: ((0 as i32) as i8) as u8,
        nPragCName: ((0 as i32) as i8) as u8,
        iArg: ((0 as i32) as i64) as u64,
    },
    PragmaName {
        zName: (b"automatic_index\0".as_ptr() as *mut i8) as *const i8,
        ePragTyp: ((4 as i32) as i8) as u8,
        mPragFlg: (((16 as i32) | (4 as i32)) as i8) as u8,
        iPragCName: ((0 as i32) as i8) as u8,
        nPragCName: ((0 as i32) as i8) as u8,
        iArg: ((32768 as i32) as i64) as u64,
    },
    PragmaName {
        zName: (b"busy_timeout\0".as_ptr() as *mut i8) as *const i8,
        ePragTyp: ((5 as i32) as i8) as u8,
        mPragFlg: ((16 as i32) as i8) as u8,
        iPragCName: ((56 as i32) as i8) as u8,
        nPragCName: ((1 as i32) as i8) as u8,
        iArg: ((0 as i32) as i64) as u64,
    },
    PragmaName {
        zName: (b"cache_size\0".as_ptr() as *mut i8) as *const i8,
        ePragTyp: ((6 as i32) as i8) as u8,
        mPragFlg: (((1 as i32) | (16 as i32) | (128 as i32) | (4 as i32)) as i8) as u8,
        iPragCName: ((0 as i32) as i8) as u8,
        nPragCName: ((0 as i32) as i8) as u8,
        iArg: ((0 as i32) as i64) as u64,
    },
    PragmaName {
        zName: (b"cache_spill\0".as_ptr() as *mut i8) as *const i8,
        ePragTyp: ((7 as i32) as i8) as u8,
        mPragFlg: (((16 as i32) | (128 as i32) | (4 as i32)) as i8) as u8,
        iPragCName: ((0 as i32) as i8) as u8,
        nPragCName: ((0 as i32) as i8) as u8,
        iArg: ((0 as i32) as i64) as u64,
    },
    PragmaName {
        zName: (b"case_sensitive_like\0".as_ptr() as *mut i8) as *const i8,
        ePragTyp: ((8 as i32) as i8) as u8,
        mPragFlg: ((2 as i32) as i8) as u8,
        iPragCName: ((0 as i32) as i8) as u8,
        nPragCName: ((0 as i32) as i8) as u8,
        iArg: ((0 as i32) as i64) as u64,
    },
    PragmaName {
        zName: (b"cell_size_check\0".as_ptr() as *mut i8) as *const i8,
        ePragTyp: ((4 as i32) as i8) as u8,
        mPragFlg: (((16 as i32) | (4 as i32)) as i8) as u8,
        iPragCName: ((0 as i32) as i8) as u8,
        nPragCName: ((0 as i32) as i8) as u8,
        iArg: ((2097152 as i32) as i64) as u64,
    },
    PragmaName {
        zName: (b"checkpoint_fullfsync\0".as_ptr() as *mut i8) as *const i8,
        ePragTyp: ((4 as i32) as i8) as u8,
        mPragFlg: (((16 as i32) | (4 as i32)) as i8) as u8,
        iPragCName: ((0 as i32) as i8) as u8,
        nPragCName: ((0 as i32) as i8) as u8,
        iArg: ((16 as i32) as i64) as u64,
    },
    PragmaName {
        zName: (b"collation_list\0".as_ptr() as *mut i8) as *const i8,
        ePragTyp: ((9 as i32) as i8) as u8,
        mPragFlg: ((16 as i32) as i8) as u8,
        iPragCName: ((33 as i32) as i8) as u8,
        nPragCName: ((2 as i32) as i8) as u8,
        iArg: ((0 as i32) as i64) as u64,
    },
    PragmaName {
        zName: (b"compile_options\0".as_ptr() as *mut i8) as *const i8,
        ePragTyp: ((10 as i32) as i8) as u8,
        mPragFlg: ((16 as i32) as i8) as u8,
        iPragCName: ((0 as i32) as i8) as u8,
        nPragCName: ((0 as i32) as i8) as u8,
        iArg: ((0 as i32) as i64) as u64,
    },
    PragmaName {
        zName: (b"count_changes\0".as_ptr() as *mut i8) as *const i8,
        ePragTyp: ((4 as i32) as i8) as u8,
        mPragFlg: (((16 as i32) | (4 as i32)) as i8) as u8,
        iPragCName: ((0 as i32) as i8) as u8,
        nPragCName: ((0 as i32) as i8) as u8,
        iArg: (((1 as i32) as i64) as u64) << (32 as i32),
    },
    PragmaName {
        zName: (b"data_version\0".as_ptr() as *mut i8) as *const i8,
        ePragTyp: ((2 as i32) as i8) as u8,
        mPragFlg: (((8 as i32) | (16 as i32)) as i8) as u8,
        iPragCName: ((0 as i32) as i8) as u8,
        nPragCName: ((0 as i32) as i8) as u8,
        iArg: ((15 as i32) as i64) as u64,
    },
    PragmaName {
        zName: (b"database_list\0".as_ptr() as *mut i8) as *const i8,
        ePragTyp: ((12 as i32) as i8) as u8,
        mPragFlg: ((16 as i32) as i8) as u8,
        iPragCName: ((50 as i32) as i8) as u8,
        nPragCName: ((3 as i32) as i8) as u8,
        iArg: ((0 as i32) as i64) as u64,
    },
    PragmaName {
        zName: (b"default_cache_size\0".as_ptr() as *mut i8) as *const i8,
        ePragTyp: ((13 as i32) as i8) as u8,
        mPragFlg: (((1 as i32) | (16 as i32) | (128 as i32) | (4 as i32)) as i8) as u8,
        iPragCName: ((55 as i32) as i8) as u8,
        nPragCName: ((1 as i32) as i8) as u8,
        iArg: ((0 as i32) as i64) as u64,
    },
    PragmaName {
        zName: (b"defer_foreign_keys\0".as_ptr() as *mut i8) as *const i8,
        ePragTyp: ((4 as i32) as i8) as u8,
        mPragFlg: (((16 as i32) | (4 as i32)) as i8) as u8,
        iPragCName: ((0 as i32) as i8) as u8,
        nPragCName: ((0 as i32) as i8) as u8,
        iArg: ((524288 as i32) as i64) as u64,
    },
    PragmaName {
        zName: (b"empty_result_callbacks\0".as_ptr() as *mut i8) as *const i8,
        ePragTyp: ((4 as i32) as i8) as u8,
        mPragFlg: (((16 as i32) | (4 as i32)) as i8) as u8,
        iPragCName: ((0 as i32) as i8) as u8,
        nPragCName: ((0 as i32) as i8) as u8,
        iArg: ((256 as i32) as i64) as u64,
    },
    PragmaName {
        zName: (b"encoding\0".as_ptr() as *mut i8) as *const i8,
        ePragTyp: ((14 as i32) as i8) as u8,
        mPragFlg: (((16 as i32) | (4 as i32)) as i8) as u8,
        iPragCName: ((0 as i32) as i8) as u8,
        nPragCName: ((0 as i32) as i8) as u8,
        iArg: ((0 as i32) as i64) as u64,
    },
    PragmaName {
        zName: (b"foreign_key_check\0".as_ptr() as *mut i8) as *const i8,
        ePragTyp: ((15 as i32) as i8) as u8,
        mPragFlg: (((1 as i32) | (16 as i32) | (32 as i32) | (64 as i32)) as i8) as u8,
        iPragCName: ((43 as i32) as i8) as u8,
        nPragCName: ((4 as i32) as i8) as u8,
        iArg: ((0 as i32) as i64) as u64,
    },
    PragmaName {
        zName: (b"foreign_key_list\0".as_ptr() as *mut i8) as *const i8,
        ePragTyp: ((16 as i32) as i8) as u8,
        mPragFlg: (((1 as i32) | (32 as i32) | (64 as i32)) as i8) as u8,
        iPragCName: ((0 as i32) as i8) as u8,
        nPragCName: ((8 as i32) as i8) as u8,
        iArg: ((0 as i32) as i64) as u64,
    },
    PragmaName {
        zName: (b"foreign_keys\0".as_ptr() as *mut i8) as *const i8,
        ePragTyp: ((4 as i32) as i8) as u8,
        mPragFlg: (((16 as i32) | (4 as i32)) as i8) as u8,
        iPragCName: ((0 as i32) as i8) as u8,
        nPragCName: ((0 as i32) as i8) as u8,
        iArg: ((16384 as i32) as i64) as u64,
    },
    PragmaName {
        zName: (b"freelist_count\0".as_ptr() as *mut i8) as *const i8,
        ePragTyp: ((2 as i32) as i8) as u8,
        mPragFlg: (((8 as i32) | (16 as i32)) as i8) as u8,
        iPragCName: ((0 as i32) as i8) as u8,
        nPragCName: ((0 as i32) as i8) as u8,
        iArg: ((0 as i32) as i64) as u64,
    },
    PragmaName {
        zName: (b"full_column_names\0".as_ptr() as *mut i8) as *const i8,
        ePragTyp: ((4 as i32) as i8) as u8,
        mPragFlg: (((16 as i32) | (4 as i32)) as i8) as u8,
        iPragCName: ((0 as i32) as i8) as u8,
        nPragCName: ((0 as i32) as i8) as u8,
        iArg: ((4 as i32) as i64) as u64,
    },
    PragmaName {
        zName: (b"fullfsync\0".as_ptr() as *mut i8) as *const i8,
        ePragTyp: ((4 as i32) as i8) as u8,
        mPragFlg: (((16 as i32) | (4 as i32)) as i8) as u8,
        iPragCName: ((0 as i32) as i8) as u8,
        nPragCName: ((0 as i32) as i8) as u8,
        iArg: ((8 as i32) as i64) as u64,
    },
    PragmaName {
        zName: (b"function_list\0".as_ptr() as *mut i8) as *const i8,
        ePragTyp: ((17 as i32) as i8) as u8,
        mPragFlg: ((16 as i32) as i8) as u8,
        iPragCName: ((15 as i32) as i8) as u8,
        nPragCName: ((6 as i32) as i8) as u8,
        iArg: ((0 as i32) as i64) as u64,
    },
    PragmaName {
        zName: (b"hard_heap_limit\0".as_ptr() as *mut i8) as *const i8,
        ePragTyp: ((18 as i32) as i8) as u8,
        mPragFlg: ((16 as i32) as i8) as u8,
        iPragCName: ((0 as i32) as i8) as u8,
        nPragCName: ((0 as i32) as i8) as u8,
        iArg: ((0 as i32) as i64) as u64,
    },
    PragmaName {
        zName: (b"ignore_check_constraints\0".as_ptr() as *mut i8) as *const i8,
        ePragTyp: ((4 as i32) as i8) as u8,
        mPragFlg: (((16 as i32) | (4 as i32)) as i8) as u8,
        iPragCName: ((0 as i32) as i8) as u8,
        nPragCName: ((0 as i32) as i8) as u8,
        iArg: ((512 as i32) as i64) as u64,
    },
    PragmaName {
        zName: (b"incremental_vacuum\0".as_ptr() as *mut i8) as *const i8,
        ePragTyp: ((19 as i32) as i8) as u8,
        mPragFlg: (((1 as i32) | (2 as i32)) as i8) as u8,
        iPragCName: ((0 as i32) as i8) as u8,
        nPragCName: ((0 as i32) as i8) as u8,
        iArg: ((0 as i32) as i64) as u64,
    },
    PragmaName {
        zName: (b"index_info\0".as_ptr() as *mut i8) as *const i8,
        ePragTyp: ((20 as i32) as i8) as u8,
        mPragFlg: (((1 as i32) | (32 as i32) | (64 as i32)) as i8) as u8,
        iPragCName: ((27 as i32) as i8) as u8,
        nPragCName: ((3 as i32) as i8) as u8,
        iArg: ((0 as i32) as i64) as u64,
    },
    PragmaName {
        zName: (b"index_list\0".as_ptr() as *mut i8) as *const i8,
        ePragTyp: ((21 as i32) as i8) as u8,
        mPragFlg: (((1 as i32) | (32 as i32) | (64 as i32)) as i8) as u8,
        iPragCName: ((33 as i32) as i8) as u8,
        nPragCName: ((5 as i32) as i8) as u8,
        iArg: ((0 as i32) as i64) as u64,
    },
    PragmaName {
        zName: (b"index_xinfo\0".as_ptr() as *mut i8) as *const i8,
        ePragTyp: ((20 as i32) as i8) as u8,
        mPragFlg: (((1 as i32) | (32 as i32) | (64 as i32)) as i8) as u8,
        iPragCName: ((27 as i32) as i8) as u8,
        nPragCName: ((6 as i32) as i8) as u8,
        iArg: ((1 as i32) as i64) as u64,
    },
    PragmaName {
        zName: (b"integrity_check\0".as_ptr() as *mut i8) as *const i8,
        ePragTyp: ((22 as i32) as i8) as u8,
        mPragFlg: (((1 as i32) | (16 as i32) | (32 as i32) | (64 as i32)) as i8) as u8,
        iPragCName: ((0 as i32) as i8) as u8,
        nPragCName: ((0 as i32) as i8) as u8,
        iArg: ((0 as i32) as i64) as u64,
    },
    PragmaName {
        zName: (b"journal_mode\0".as_ptr() as *mut i8) as *const i8,
        ePragTyp: ((23 as i32) as i8) as u8,
        mPragFlg: (((1 as i32) | (16 as i32) | (128 as i32)) as i8) as u8,
        iPragCName: ((0 as i32) as i8) as u8,
        nPragCName: ((0 as i32) as i8) as u8,
        iArg: ((0 as i32) as i64) as u64,
    },
    PragmaName {
        zName: (b"journal_size_limit\0".as_ptr() as *mut i8) as *const i8,
        ePragTyp: ((24 as i32) as i8) as u8,
        mPragFlg: (((16 as i32) | (128 as i32)) as i8) as u8,
        iPragCName: ((0 as i32) as i8) as u8,
        nPragCName: ((0 as i32) as i8) as u8,
        iArg: ((0 as i32) as i64) as u64,
    },
    PragmaName {
        zName: (b"legacy_alter_table\0".as_ptr() as *mut i8) as *const i8,
        ePragTyp: ((4 as i32) as i8) as u8,
        mPragFlg: (((16 as i32) | (4 as i32)) as i8) as u8,
        iPragCName: ((0 as i32) as i8) as u8,
        nPragCName: ((0 as i32) as i8) as u8,
        iArg: ((67108864 as i32) as i64) as u64,
    },
    PragmaName {
        zName: (b"locking_mode\0".as_ptr() as *mut i8) as *const i8,
        ePragTyp: ((26 as i32) as i8) as u8,
        mPragFlg: (((16 as i32) | (128 as i32)) as i8) as u8,
        iPragCName: ((0 as i32) as i8) as u8,
        nPragCName: ((0 as i32) as i8) as u8,
        iArg: ((0 as i32) as i64) as u64,
    },
    PragmaName {
        zName: (b"max_page_count\0".as_ptr() as *mut i8) as *const i8,
        ePragTyp: ((27 as i32) as i8) as u8,
        mPragFlg: (((1 as i32) | (16 as i32) | (128 as i32)) as i8) as u8,
        iPragCName: ((0 as i32) as i8) as u8,
        nPragCName: ((0 as i32) as i8) as u8,
        iArg: ((0 as i32) as i64) as u64,
    },
    PragmaName {
        zName: (b"mmap_size\0".as_ptr() as *mut i8) as *const i8,
        ePragTyp: ((28 as i32) as i8) as u8,
        mPragFlg: ((0 as i32) as i8) as u8,
        iPragCName: ((0 as i32) as i8) as u8,
        nPragCName: ((0 as i32) as i8) as u8,
        iArg: ((0 as i32) as i64) as u64,
    },
    PragmaName {
        zName: (b"module_list\0".as_ptr() as *mut i8) as *const i8,
        ePragTyp: ((29 as i32) as i8) as u8,
        mPragFlg: ((16 as i32) as i8) as u8,
        iPragCName: ((9 as i32) as i8) as u8,
        nPragCName: ((1 as i32) as i8) as u8,
        iArg: ((0 as i32) as i64) as u64,
    },
    PragmaName {
        zName: (b"optimize\0".as_ptr() as *mut i8) as *const i8,
        ePragTyp: ((30 as i32) as i8) as u8,
        mPragFlg: (((32 as i32) | (1 as i32)) as i8) as u8,
        iPragCName: ((0 as i32) as i8) as u8,
        nPragCName: ((0 as i32) as i8) as u8,
        iArg: ((0 as i32) as i64) as u64,
    },
    PragmaName {
        zName: (b"page_count\0".as_ptr() as *mut i8) as *const i8,
        ePragTyp: ((27 as i32) as i8) as u8,
        mPragFlg: (((1 as i32) | (16 as i32) | (128 as i32)) as i8) as u8,
        iPragCName: ((0 as i32) as i8) as u8,
        nPragCName: ((0 as i32) as i8) as u8,
        iArg: ((0 as i32) as i64) as u64,
    },
    PragmaName {
        zName: (b"page_size\0".as_ptr() as *mut i8) as *const i8,
        ePragTyp: ((31 as i32) as i8) as u8,
        mPragFlg: (((16 as i32) | (128 as i32) | (4 as i32)) as i8) as u8,
        iPragCName: ((0 as i32) as i8) as u8,
        nPragCName: ((0 as i32) as i8) as u8,
        iArg: ((0 as i32) as i64) as u64,
    },
    PragmaName {
        zName: (b"pragma_list\0".as_ptr() as *mut i8) as *const i8,
        ePragTyp: ((32 as i32) as i8) as u8,
        mPragFlg: ((16 as i32) as i8) as u8,
        iPragCName: ((9 as i32) as i8) as u8,
        nPragCName: ((1 as i32) as i8) as u8,
        iArg: ((0 as i32) as i64) as u64,
    },
    PragmaName {
        zName: (b"query_only\0".as_ptr() as *mut i8) as *const i8,
        ePragTyp: ((4 as i32) as i8) as u8,
        mPragFlg: (((16 as i32) | (4 as i32)) as i8) as u8,
        iPragCName: ((0 as i32) as i8) as u8,
        nPragCName: ((0 as i32) as i8) as u8,
        iArg: ((1048576 as i32) as i64) as u64,
    },
    PragmaName {
        zName: (b"quick_check\0".as_ptr() as *mut i8) as *const i8,
        ePragTyp: ((22 as i32) as i8) as u8,
        mPragFlg: (((1 as i32) | (16 as i32) | (32 as i32) | (64 as i32)) as i8) as u8,
        iPragCName: ((0 as i32) as i8) as u8,
        nPragCName: ((0 as i32) as i8) as u8,
        iArg: ((0 as i32) as i64) as u64,
    },
    PragmaName {
        zName: (b"read_uncommitted\0".as_ptr() as *mut i8) as *const i8,
        ePragTyp: ((4 as i32) as i8) as u8,
        mPragFlg: (((16 as i32) | (4 as i32)) as i8) as u8,
        iPragCName: ((0 as i32) as i8) as u8,
        nPragCName: ((0 as i32) as i8) as u8,
        iArg: (((4 as i32) as i64) as u64) << (32 as i32),
    },
    PragmaName {
        zName: (b"recursive_triggers\0".as_ptr() as *mut i8) as *const i8,
        ePragTyp: ((4 as i32) as i8) as u8,
        mPragFlg: (((16 as i32) | (4 as i32)) as i8) as u8,
        iPragCName: ((0 as i32) as i8) as u8,
        nPragCName: ((0 as i32) as i8) as u8,
        iArg: ((8192 as i32) as i64) as u64,
    },
    PragmaName {
        zName: (b"reverse_unordered_selects\0".as_ptr() as *mut i8) as *const i8,
        ePragTyp: ((4 as i32) as i8) as u8,
        mPragFlg: (((16 as i32) | (4 as i32)) as i8) as u8,
        iPragCName: ((0 as i32) as i8) as u8,
        nPragCName: ((0 as i32) as i8) as u8,
        iArg: ((4096 as i32) as i64) as u64,
    },
    PragmaName {
        zName: (b"schema_version\0".as_ptr() as *mut i8) as *const i8,
        ePragTyp: ((2 as i32) as i8) as u8,
        mPragFlg: (((4 as i32) | (16 as i32)) as i8) as u8,
        iPragCName: ((0 as i32) as i8) as u8,
        nPragCName: ((0 as i32) as i8) as u8,
        iArg: ((1 as i32) as i64) as u64,
    },
    PragmaName {
        zName: (b"secure_delete\0".as_ptr() as *mut i8) as *const i8,
        ePragTyp: ((33 as i32) as i8) as u8,
        mPragFlg: ((16 as i32) as i8) as u8,
        iPragCName: ((0 as i32) as i8) as u8,
        nPragCName: ((0 as i32) as i8) as u8,
        iArg: ((0 as i32) as i64) as u64,
    },
    PragmaName {
        zName: (b"short_column_names\0".as_ptr() as *mut i8) as *const i8,
        ePragTyp: ((4 as i32) as i8) as u8,
        mPragFlg: (((16 as i32) | (4 as i32)) as i8) as u8,
        iPragCName: ((0 as i32) as i8) as u8,
        nPragCName: ((0 as i32) as i8) as u8,
        iArg: ((64 as i32) as i64) as u64,
    },
    PragmaName {
        zName: (b"shrink_memory\0".as_ptr() as *mut i8) as *const i8,
        ePragTyp: ((34 as i32) as i8) as u8,
        mPragFlg: ((2 as i32) as i8) as u8,
        iPragCName: ((0 as i32) as i8) as u8,
        nPragCName: ((0 as i32) as i8) as u8,
        iArg: ((0 as i32) as i64) as u64,
    },
    PragmaName {
        zName: (b"soft_heap_limit\0".as_ptr() as *mut i8) as *const i8,
        ePragTyp: ((35 as i32) as i8) as u8,
        mPragFlg: ((16 as i32) as i8) as u8,
        iPragCName: ((0 as i32) as i8) as u8,
        nPragCName: ((0 as i32) as i8) as u8,
        iArg: ((0 as i32) as i64) as u64,
    },
    PragmaName {
        zName: (b"synchronous\0".as_ptr() as *mut i8) as *const i8,
        ePragTyp: ((36 as i32) as i8) as u8,
        mPragFlg: (((1 as i32) | (16 as i32) | (128 as i32) | (4 as i32)) as i8) as u8,
        iPragCName: ((0 as i32) as i8) as u8,
        nPragCName: ((0 as i32) as i8) as u8,
        iArg: ((0 as i32) as i64) as u64,
    },
    PragmaName {
        zName: (b"table_info\0".as_ptr() as *mut i8) as *const i8,
        ePragTyp: ((37 as i32) as i8) as u8,
        mPragFlg: (((1 as i32) | (32 as i32) | (64 as i32)) as i8) as u8,
        iPragCName: ((8 as i32) as i8) as u8,
        nPragCName: ((6 as i32) as i8) as u8,
        iArg: ((0 as i32) as i64) as u64,
    },
    PragmaName {
        zName: (b"table_list\0".as_ptr() as *mut i8) as *const i8,
        ePragTyp: ((38 as i32) as i8) as u8,
        mPragFlg: (((1 as i32) | (32 as i32)) as i8) as u8,
        iPragCName: ((21 as i32) as i8) as u8,
        nPragCName: ((6 as i32) as i8) as u8,
        iArg: ((0 as i32) as i64) as u64,
    },
    PragmaName {
        zName: (b"table_xinfo\0".as_ptr() as *mut i8) as *const i8,
        ePragTyp: ((37 as i32) as i8) as u8,
        mPragFlg: (((1 as i32) | (32 as i32) | (64 as i32)) as i8) as u8,
        iPragCName: ((8 as i32) as i8) as u8,
        nPragCName: ((7 as i32) as i8) as u8,
        iArg: ((1 as i32) as i64) as u64,
    },
    PragmaName {
        zName: (b"temp_store\0".as_ptr() as *mut i8) as *const i8,
        ePragTyp: ((39 as i32) as i8) as u8,
        mPragFlg: (((16 as i32) | (4 as i32)) as i8) as u8,
        iPragCName: ((0 as i32) as i8) as u8,
        nPragCName: ((0 as i32) as i8) as u8,
        iArg: ((0 as i32) as i64) as u64,
    },
    PragmaName {
        zName: (b"temp_store_directory\0".as_ptr() as *mut i8) as *const i8,
        ePragTyp: ((40 as i32) as i8) as u8,
        mPragFlg: ((4 as i32) as i8) as u8,
        iPragCName: ((0 as i32) as i8) as u8,
        nPragCName: ((0 as i32) as i8) as u8,
        iArg: ((0 as i32) as i64) as u64,
    },
    PragmaName {
        zName: (b"threads\0".as_ptr() as *mut i8) as *const i8,
        ePragTyp: ((41 as i32) as i8) as u8,
        mPragFlg: ((16 as i32) as i8) as u8,
        iPragCName: ((0 as i32) as i8) as u8,
        nPragCName: ((0 as i32) as i8) as u8,
        iArg: ((0 as i32) as i64) as u64,
    },
    PragmaName {
        zName: (b"trusted_schema\0".as_ptr() as *mut i8) as *const i8,
        ePragTyp: ((4 as i32) as i8) as u8,
        mPragFlg: (((16 as i32) | (4 as i32)) as i8) as u8,
        iPragCName: ((0 as i32) as i8) as u8,
        nPragCName: ((0 as i32) as i8) as u8,
        iArg: ((128 as i32) as i64) as u64,
    },
    PragmaName {
        zName: (b"user_version\0".as_ptr() as *mut i8) as *const i8,
        ePragTyp: ((2 as i32) as i8) as u8,
        mPragFlg: (((4 as i32) | (16 as i32)) as i8) as u8,
        iPragCName: ((0 as i32) as i8) as u8,
        nPragCName: ((0 as i32) as i8) as u8,
        iArg: ((6 as i32) as i64) as u64,
    },
    PragmaName {
        zName: (b"wal_autocheckpoint\0".as_ptr() as *mut i8) as *const i8,
        ePragTyp: ((42 as i32) as i8) as u8,
        mPragFlg: ((0 as i32) as i8) as u8,
        iPragCName: ((0 as i32) as i8) as u8,
        nPragCName: ((0 as i32) as i8) as u8,
        iArg: ((0 as i32) as i64) as u64,
    },
    PragmaName {
        zName: (b"wal_checkpoint\0".as_ptr() as *mut i8) as *const i8,
        ePragTyp: ((43 as i32) as i8) as u8,
        mPragFlg: ((1 as i32) as i8) as u8,
        iPragCName: ((47 as i32) as i8) as u8,
        nPragCName: ((3 as i32) as i8) as u8,
        iArg: ((0 as i32) as i64) as u64,
    },
    PragmaName {
        zName: (b"writable_schema\0".as_ptr() as *mut i8) as *const i8,
        ePragTyp: ((4 as i32) as i8) as u8,
        mPragFlg: (((16 as i32) | (4 as i32)) as i8) as u8,
        iPragCName: ((0 as i32) as i8) as u8,
        nPragCName: ((0 as i32) as i8) as u8,
        iArg: (((1 as i32) | (134217728 as i32)) as i64) as u64,
    },
]);

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
    pub struct __SlateBits77U0 {
        #[bits(4)]
        pub notNull: u32,
        #[bits(4)]
        pub eCType: u32,
    }
    #[bitfields::bitfield([u8; 2], c_names = true, new = false, from_into_bits = false, from_traits = false, default = false, debug = false, builder = false, bit_ops = false)]
    pub struct __SlateBits194U0 {
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
    pub struct __SlateBits198U0 {
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
    pub struct __SlateBits103U0 {
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
    pub struct __SlateBits176U0 {
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
    pub struct __SlateBits115U0 {
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

// The "pragma.h" include file is an automatically generated file that
// that includes the PragType_XXXX macro definitions and the aPragmaName[]
// object.  This ensures that the aPragmaName[] table is arranged in
// lexicographical order to facility a binary search of the pragma name.
// Do not edit pragma.h directly.  Edit and rerun the script in at
// ../tool/mkpragmatab.tcl.
// When the 0x10 bit of PRAGMA optimize is set, any ANALYZE commands
// will be run with an analysis_limit set to the lessor of the value of
// the following macro or to the actual analysis_limit if it is non-zero,
// in order to prevent PRAGMA optimize from running for too long.
//
// The value of 2000 is chosen empirically so that the worst-case run-time
// for PRAGMA optimize does not exceed 100 milliseconds against a variety
// of test databases on a RaspberryPI-4 compiled using -Os and without
// -DSQLITE_DEBUG.  Of course, your mileage may vary.  For the purpose of
// this paragraph, "worst-case" means that ANALYZE ends up being
// run on every table in the database.  The worst case typically only
// happens if PRAGMA optimize is run on a database file for which ANALYZE
// has not been previously run and the 0x10000 flag is included so that
// all tables are analyzed.  The usual case for PRAGMA optimize is that
// no ANALYZE commands will be run at all, or if any ANALYZE happens it
// will be against a single table, so that expected timing for PRAGMA
// optimize on a PI-4 is more like 1 millisecond or less with the 0x10000
// flag or less than 100 microseconds without the 0x10000 flag.
//
// An analysis limit of 2000 is almost always sufficient for the query
// planner to fully characterize an index.  The additional accuracy from
// a larger analysis is not usually helpful.
/// Interpret the given string as a safety level.  Return 0 for OFF,
/// 1 for ON or NORMAL, 2 for FULL, and 3 for EXTRA.  Return 1 for an empty or
/// unrecognized string argument.  The FULL and EXTRA option is disallowed
/// if the omitFull parameter it 1.
///
/// Note that the values returned are one less that the values that
/// should be passed into sqlite3BtreeSetSafetyLevel().  The is done
/// to support legacy SQL code.  The safety level used to be boolean
/// and older scripts may have used numbers 0 for OFF and 1 for ON.
fn getSafetyLevel(mut z: *const i8, mut omitFull: i32, mut dflt: u8) -> u8 {
    // 123456789 123456789 123
    // on no off false yes true extra full
    let mut i: i32 = 0 as i32;
    let mut n: i32 = 0 as i32;
    if (((unsafe {
        *unsafe {
            unsafe { std::ptr::addr_of!(sqlite3CtypeMap) as *const u8 }
                .offset(((((unsafe { *z }) as u8) as u32) as i32) as isize)
        }
    }) as u32) as i32)
        & (4 as i32)
        != (0 as i32)
    {
        return ((unsafe { sqlite3Atoi(z) }) as i8) as u8;
    }
    n = unsafe { sqlite3Strlen30(z) };
    i = 0 as i32;
    '__slate_break_1330: loop {
        if !(i < ((((8 as u64) / (1 as u64)) as u32) as i32)) {
            break;
        }
        let __v1654: bool;
        if (((unsafe {
            *unsafe { unsafe { std::ptr::addr_of!(iLength) as *const u8 }.offset(i as isize) }
        }) as u32) as i32)
            == n
        {
            __v1654 = (unsafe {
                sqlite3_strnicmp(
                    unsafe {
                        unsafe { std::ptr::addr_of!(zText.0) as *const i8 }.offset(
                            (((unsafe {
                                *unsafe {
                                    unsafe { std::ptr::addr_of!(iOffset) as *const u8 }
                                        .offset(i as isize)
                                }
                            }) as u32) as i32) as isize,
                        )
                    },
                    z,
                    n,
                )
            }) == (0 as i32);
        } else {
            __v1654 = false as bool;
        }
        if __v1654
            && (!(omitFull != (0 as i32))
                || (((unsafe {
                    *unsafe {
                        unsafe { std::ptr::addr_of!(iValue) as *const u8 }.offset(i as isize)
                    }
                }) as u32) as i32)
                    <= (1 as i32))
        {
            return unsafe {
                *unsafe { unsafe { std::ptr::addr_of!(iValue) as *const u8 }.offset(i as isize) }
            };
        }
        let __v1652: i32 = i;
        let __v1653: i32 = __v1652 + (1 as i32);
        i = __v1653;
    }
    return dflt;
}

static mut zText: __SlateAlign16<[i8; 25]> = __SlateAlign16([
    111 as i8, 110 as i8, 111 as i8, 102 as i8, 102 as i8, 97 as i8, 108 as i8, 115 as i8,
    101 as i8, 121 as i8, 101 as i8, 115 as i8, 116 as i8, 114 as i8, 117 as i8, 101 as i8,
    120 as i8, 116 as i8, 114 as i8, 97 as i8, 102 as i8, 117 as i8, 108 as i8, 108 as i8, 0 as i8,
]);

static mut iOffset: [u8; 8] = [
    ((0 as i32) as i8) as u8,
    ((1 as i32) as i8) as u8,
    ((2 as i32) as i8) as u8,
    ((4 as i32) as i8) as u8,
    ((9 as i32) as i8) as u8,
    ((12 as i32) as i8) as u8,
    ((15 as i32) as i8) as u8,
    ((20 as i32) as i8) as u8,
];

static mut iLength: [u8; 8] = [
    ((2 as i32) as i8) as u8,
    ((2 as i32) as i8) as u8,
    ((3 as i32) as i8) as u8,
    ((5 as i32) as i8) as u8,
    ((3 as i32) as i8) as u8,
    ((4 as i32) as i8) as u8,
    ((5 as i32) as i8) as u8,
    ((4 as i32) as i8) as u8,
];

static mut iValue: [u8; 8] = [
    ((1 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((1 as i32) as i8) as u8,
    ((1 as i32) as i8) as u8,
    ((3 as i32) as i8) as u8,
    ((2 as i32) as i8) as u8,
];

/// Interpret the given string as a boolean value.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3GetBoolean(mut z: *const i8, mut dflt: u8) -> u8 {
    return (((getSafetyLevel(z, 1 as i32, dflt) as u32) as i32) != (0 as i32)) as u8;
}

// The sqlite3GetBoolean() function is used by other modules but the
// remainder of this file is specific to PRAGMA processing.  So omit
// the rest of the file if PRAGMAs are omitted from the build.
/// Interpret the given string as a locking mode value.
fn getLockingMode(mut z: *const i8) -> i32 {
    if z != std::ptr::null::<i8>() {
        if (0 as i32)
            == unsafe { sqlite3StrICmp(z, (b"exclusive\0".as_ptr() as *mut i8) as *const i8) }
        {
            return 1 as i32;
        }
        if (0 as i32)
            == unsafe { sqlite3StrICmp(z, (b"normal\0".as_ptr() as *mut i8) as *const i8) }
        {
            return 0 as i32;
        }
    }
    return -(1 as i32);
}

/// Interpret the given string as an auto-vacuum mode value.
///
/// The following strings, "none", "full" and "incremental" are
/// acceptable, as are their numeric equivalents: 0, 1 and 2 respectively.
fn getAutoVacuum(mut z: *const i8) -> i32 {
    let mut i: i32 = 0 as i32;
    if (0 as i32) == unsafe { sqlite3StrICmp(z, (b"none\0".as_ptr() as *mut i8) as *const i8) } {
        return 0 as i32;
    }
    if (0 as i32) == unsafe { sqlite3StrICmp(z, (b"full\0".as_ptr() as *mut i8) as *const i8) } {
        return 1 as i32;
    }
    if (0 as i32)
        == unsafe { sqlite3StrICmp(z, (b"incremental\0".as_ptr() as *mut i8) as *const i8) }
    {
        return 2 as i32;
    }
    i = unsafe { sqlite3Atoi(z) };
    return ((((if i >= (0 as i32) && i <= (2 as i32) {
        i
    } else {
        0 as i32
    }) as i8) as u8) as u32) as i32;
}

/// Interpret the given string as a temp db location. Return 1 for file
/// backed temporary databases, 2 for the Red-Black tree in memory database
/// and 0 to use the compile-time default.
fn getTempStore(mut z: *const i8) -> i32 {
    if ((unsafe { *unsafe { z.offset((0 as i32) as isize) } }) as i32) >= (48 as i32)
        && ((unsafe { *unsafe { z.offset((0 as i32) as isize) } }) as i32) <= (50 as i32)
    {
        return ((unsafe { *unsafe { z.offset((0 as i32) as isize) } }) as i32) - (48 as i32);
    } else {
        if (unsafe { sqlite3StrICmp(z, (b"file\0".as_ptr() as *mut i8) as *const i8) })
            == (0 as i32)
        {
            return 1 as i32;
        } else {
            if (unsafe { sqlite3StrICmp(z, (b"memory\0".as_ptr() as *mut i8) as *const i8) })
                == (0 as i32)
            {
                return 2 as i32;
            } else {
                return 0 as i32;
            }
        }
    }
    return unsafe { std::mem::zeroed() };
}

/// Invalidate temp storage, either when the temp storage is changed
/// from default, or when 'file' and the temp_store_directory has changed
fn invalidateTempStorage(mut pParse: *mut Parse) -> i32 {
    let mut db: *mut sqlite3 = unsafe { (*pParse).db };
    if (unsafe { (*unsafe { unsafe { (*db).aDb }.offset((1 as i32) as isize) }).pBt })
        != std::ptr::null_mut::<Btree>()
    {
        let __v1655: bool;
        if !((unsafe { (*db).autoCommit }) != (0 as u8)) {
            __v1655 = true as bool;
        } else {
            __v1655 = (unsafe {
                sqlite3BtreeTxnState(unsafe {
                    (*unsafe { unsafe { (*db).aDb }.offset((1 as i32) as isize) }).pBt
                })
            }) != (0 as i32);
        }
        if __v1655 {
            unsafe {
                sqlite3ErrorMsg(
                    pParse,
                    (b"temporary storage cannot be changed from within a transaction\0".as_ptr()
                        as *mut i8) as *const i8,
                )
            };
            return 1 as i32;
        }
        unsafe {
            sqlite3BtreeClose(unsafe {
                (*unsafe { unsafe { (*db).aDb }.offset((1 as i32) as isize) }).pBt
            })
        };
        unsafe {
            (*unsafe { unsafe { (*db).aDb }.offset((1 as i32) as isize) }).pBt =
                std::ptr::null_mut::<Btree>();
        }
        unsafe { sqlite3ResetAllSchemasOfConnection(db) };
    }
    return 0 as i32;
}

/// If the TEMP database is open, close it and mark the database schema
/// as needing reloading.  This must be done when using the SQLITE_TEMP_STORE
/// or DEFAULT_TEMP_STORE pragmas.
fn changeTempStorage(mut pParse: *mut Parse, mut zStorageType: *const i8) -> i32 {
    let mut ts: i32 = getTempStore(zStorageType);
    let mut db: *mut sqlite3 = unsafe { (*pParse).db };
    if (((unsafe { (*db).temp_store }) as u32) as i32) == ts {
        return 0 as i32;
    }
    if invalidateTempStorage(pParse) != (0 as i32) {
        return 1 as i32;
    }
    unsafe {
        (*db).temp_store = (ts as i8) as u8;
    }
    return 0 as i32;
}

/// Set result column names for a pragma.
///
/// # Arguments
///
/// * `v` - The query under construction
/// * `pPragma` - The pragma
fn setPragmaResultColumnNames(mut v: *mut Vdbe, mut pPragma: *const PragmaName) {
    let mut n: u8 = unsafe { (*pPragma).nPragCName };
    unsafe {
        sqlite3VdbeSetNumCols(
            v,
            if ((n as u32) as i32) == (0 as i32) {
                1 as i32
            } else {
                (n as u32) as i32
            },
        )
    };
    if ((n as u32) as i32) == (0 as i32) {
        unsafe { sqlite3VdbeSetColName(v, 0 as i32, 0 as i32, unsafe { (*pPragma).zName }, None) };
    } else {
        let mut i: i32 = 0 as i32;
        let mut j: i32 = 0 as i32;
        i = 0 as i32;
        let __v1656: i32 = ((unsafe { (*pPragma).iPragCName }) as u32) as i32;
        j = __v1656;
        '__slate_break_1339: loop {
            if !(i < ((n as u32) as i32)) {
                break;
            }
            unsafe {
                sqlite3VdbeSetColName(
                    v,
                    i,
                    0 as i32,
                    unsafe {
                        *unsafe {
                            unsafe { std::ptr::addr_of!(pragCName.0) as *const *const i8 }
                                .offset(j as isize)
                        }
                    },
                    None,
                )
            };
            let __v1657: i32 = i;
            let __v1658: i32 = __v1657 + (1 as i32);
            i = __v1658;
            let __v1659: i32 = j;
            let __v1660: i32 = __v1659 + (1 as i32);
            j = __v1660;
        }
    }
}

/// Generate code to return a single integer value.
fn returnSingleInt(mut v: *mut Vdbe, mut value: i64) {
    unsafe { sqlite3VdbeAddInt64(v, 1 as i32, value) };
    unsafe { sqlite3VdbeAddOp2(v, 86 as i32, 1 as i32, 1 as i32) };
}

/// Generate code to return a single text value.
///
/// # Arguments
///
/// * `v` - Prepared statement under construction
/// * `zValue` - Value to be returned
fn returnSingleText(mut v: *mut Vdbe, mut zValue: *const i8) {
    if zValue != std::ptr::null::<i8>() {
        unsafe { sqlite3VdbeLoadString(v, 1 as i32, zValue) };
        unsafe { sqlite3VdbeAddOp2(v, 86 as i32, 1 as i32, 1 as i32) };
    }
}

/// Set the safety_level and pager flags for pager iDb.  Or if iDb<0
/// set these values for all pagers.
fn setAllPagerFlags(mut db: *mut sqlite3) {
    if (unsafe { (*db).autoCommit }) != (0 as u8) {
        let mut pDb: *mut Db = unsafe { (*db).aDb };
        let mut n: i32 = unsafe { (*db).nDb };
        0 as i32;
        0 as i32;
        0 as i32;
        0 as i32;
        0 as i32;
        '__slate_break_1340: loop {
            let __v1661: i32 = n;
            let __v1662: i32 = __v1661 - (1 as i32);
            n = __v1662;
            if !(__v1661 > (0 as i32)) {
                break;
            }
            if (unsafe { (*pDb).pBt }) != std::ptr::null_mut::<Btree>() {
                unsafe {
                    sqlite3BtreeSetPagerFlags(
                        unsafe { (*pDb).pBt },
                        ((((((unsafe { (*pDb).safety_level }) as u32) as i32) as i64) as u64)
                            | (unsafe { (*db).flags }) & (((56 as i32) as i64) as u64))
                            as u32,
                    )
                };
            }
            let __v1663: *mut Db = pDb;
            let __v1664: *mut Db = unsafe { __v1663.offset((1 as i32) as isize) };
            pDb = __v1664;
        }
    }
}

/// Return a human-readable name for a constraint resolution action.
fn actionName(mut action: u8) -> *const i8 {
    let mut zName: *const i8 = unsafe { std::mem::zeroed() };
    '__slate_break_1341: {
        match (action as u32) as i32 {
            8 => {
                zName = (b"SET NULL\0".as_ptr() as *mut i8) as *const i8;
            }
            9 => {
                zName = (b"SET DEFAULT\0".as_ptr() as *mut i8) as *const i8;
            }
            10 => {
                zName = (b"CASCADE\0".as_ptr() as *mut i8) as *const i8;
            }
            7 => {
                zName = (b"RESTRICT\0".as_ptr() as *mut i8) as *const i8;
            }
            _ => {
                zName = (b"NO ACTION\0".as_ptr() as *mut i8) as *const i8;
                0 as i32;
            }
        }
    }
    return zName;
}

/// Parameter eMode must be one of the PAGER_JOURNALMODE_XXX constants
/// defined in pager.h. This function returns the associated lowercase
/// journal-mode name.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3JournalModename(mut eMode: i32) -> *const i8 {
    0 as i32;
    0 as i32;
    0 as i32;
    0 as i32;
    0 as i32;
    0 as i32;
    0 as i32;
    if eMode == ((((48 as u64) / (8 as u64)) as u32) as i32) {
        return std::ptr::null::<i8>();
    }
    return (unsafe {
        *unsafe {
            unsafe { std::ptr::addr_of!(azModeName.0) as *const *mut i8 }.offset(eMode as isize)
        }
    }) as *const i8;
}

static mut azModeName: __SlateAlign16<[*mut i8; 6]> = __SlateAlign16([
    b"delete\0".as_ptr() as *mut i8,
    b"persist\0".as_ptr() as *mut i8,
    b"off\0".as_ptr() as *mut i8,
    b"truncate\0".as_ptr() as *mut i8,
    b"memory\0".as_ptr() as *mut i8,
    b"wal\0".as_ptr() as *mut i8,
]);

/// Locate a pragma in the aPragmaName[] array.
fn pragmaLocate(mut zName: *const i8) -> *const PragmaName {
    let mut upr: i32 = 0 as i32;
    let mut lwr: i32 = 0 as i32;
    let mut mid: i32 = 0 as i32;
    let mut rc: i32 = 0 as i32;
    lwr = 0 as i32;
    upr = ((((1584 as u64) / (24 as u64)) as u32) as i32) - (1 as i32);
    '__slate_break_1353: while lwr <= upr {
        mid = (lwr + upr) / (2 as i32);
        rc = unsafe {
            sqlite3_stricmp(zName, unsafe {
                (*unsafe {
                    unsafe { std::ptr::addr_of!(aPragmaName.0) as *const PragmaName }
                        .offset(mid as isize)
                })
                .zName
            })
        };
        if rc == (0 as i32) {
            break '__slate_break_1353;
        }
        if rc < (0 as i32) {
            upr = mid - (1 as i32);
        } else {
            lwr = mid + (1 as i32);
        }
    }
    return if lwr > upr {
        std::ptr::null::<PragmaName>()
    } else {
        unsafe {
            unsafe { std::ptr::addr_of!(aPragmaName.0) as *const PragmaName }.offset(mid as isize)
        }
    };
}

/// Create zero or more entries in the output for the SQL functions
/// defined by FuncDef p.
///
/// # Arguments
///
/// * `v` - The prepared statement being created
/// * `p` - A particular function definition
/// * `isBuiltin` - True if this is a built-in function
/// * `showInternFuncs` - True if showing internal functions
fn pragmaFunclistLine(
    mut v: *mut Vdbe,
    mut p: *mut FuncDef,
    mut isBuiltin: i32,
    mut showInternFuncs: i32,
) {
    let mut mask: u32 =
        ((2048 as i32) | (524288 as i32) | (1048576 as i32) | (2097152 as i32) | (262144 as i32))
            as u32;
    if showInternFuncs != (0 as i32) {
        mask = 4294967295 as u32;
    }
    '__slate_break_1354: while p != std::ptr::null_mut::<FuncDef>() {
        let mut zType: *const i8 = unsafe { std::mem::zeroed() };
        0 as i32;
        0 as i32;
        0 as i32;
        0 as i32;
        if (unsafe { (*p).xSFunc }) == None {
        } else {
            if (unsafe { (*p).funcFlags }) & ((262144 as i32) as u32) != ((0 as i32) as u32)
                && showInternFuncs == (0 as i32)
            {
            } else {
                if (unsafe { (*p).xValue }) != None {
                    zType = (b"w\0".as_ptr() as *mut i8) as *const i8;
                } else {
                    if (unsafe { (*p).xFinalize }) != None {
                        zType = (b"a\0".as_ptr() as *mut i8) as *const i8;
                    } else {
                        zType = (b"s\0".as_ptr() as *mut i8) as *const i8;
                    }
                }
                unsafe {
                    sqlite3VdbeMultiLoad(
                        v,
                        1 as i32,
                        (b"sissii\0".as_ptr() as *mut i8) as *const i8,
                        unsafe { (*p).zName },
                        isBuiltin,
                        zType,
                        unsafe {
                            *unsafe {
                                unsafe { std::ptr::addr_of_mut!(azEnc.0) as *mut *const i8 }.offset(
                                    ((unsafe { (*p).funcFlags }) & ((3 as i32) as u32)) as isize,
                                )
                            }
                        },
                        (unsafe { (*p).nArg }) as i32,
                        (unsafe { (*p).funcFlags }) & mask ^ ((2097152 as i32) as u32),
                    )
                };
            }
        }
        p = unsafe { (*p).pNext };
    }
}

static mut azEnc: __SlateAlign16<[*const i8; 4]> = __SlateAlign16([
    std::ptr::null::<i8>(),
    (b"utf8\0".as_ptr() as *mut i8) as *const i8,
    (b"utf16le\0".as_ptr() as *mut i8) as *const i8,
    (b"utf16be\0".as_ptr() as *mut i8) as *const i8,
]);

/// Helper subroutine for PRAGMA integrity_check:
///
/// Generate code to output a single-column result row with a value of the
/// string held in register 3.  Decrement the result count in register 1
/// and halt if the maximum number of result rows have been issued.
fn integrityCheckResultRow(mut v: *mut Vdbe) -> i32 {
    let mut addr: i32 = 0 as i32;
    unsafe { sqlite3VdbeAddOp2(v, 86 as i32, 3 as i32, 1 as i32) };
    addr = unsafe {
        sqlite3VdbeAddOp3(
            v,
            61 as i32,
            1 as i32,
            (unsafe { sqlite3VdbeCurrentAddr(v) }) + (2 as i32),
            1 as i32,
        )
    };
    {}
    unsafe { sqlite3VdbeAddOp0(v, 72 as i32) };
    return addr;
}

/// Should table pTab be skipped when doing an integrity_check?
/// Return true or false.
///
/// If pObjTab is not null, the return true if pTab matches pObjTab.
///
/// If pObjTab is null, then return true only if pTab is an imposter table.
fn tableSkipIntegrityCheck(mut pTab: *const Table, mut pObjTab: *const Table) -> i32 {
    if pObjTab != std::ptr::null::<Table>() {
        return (pTab != pObjTab) as i32;
    } else {
        return ((unsafe { (*pTab).tabFlags }) & ((131072 as i32) as u32) != ((0 as i32) as u32))
            as i32;
    }
    return unsafe { std::mem::zeroed() };
}

/// Process a pragma statement.
///
/// Pragmas are of this form:
///
///      PRAGMA [schema.]id [= value]
///
/// The identifier might also be a string.  The value is a string, and
/// identifier, or a number.  If minusFlag is true, then the value is
/// a number that was preceded by a minus sign.
///
/// If the left side is "database.id" then pId1 is the database name
/// and pId2 is the id.  If the left side is just "id" then pId1 is the
/// id and pId2 is any empty string.
///
/// # Arguments
///
/// * `pId1` - First part of [schema.]id field
/// * `pId2` - Second part of [schema.]id field, or NULL
/// * `pValue` - Token for <value>, or NULL
/// * `minusFlag` - True if a '-' sign preceded <value>
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3Pragma(
    mut pParse: *mut Parse,
    mut pId1: *mut Token,
    mut pId2: *mut Token,
    mut pValue: *mut Token,
    mut minusFlag: i32,
) {
    let mut __slate_storage_1651: std::mem::MaybeUninit<bool> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1651: *mut bool = std::ptr::addr_of_mut!(__slate_storage_1651) as *mut bool;
    let mut __slate_storage_789: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_789: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_789) as *mut i64;
    let mut __slate_storage_1650: std::mem::MaybeUninit<bool> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1650: *mut bool = std::ptr::addr_of_mut!(__slate_storage_1650) as *mut bool;
    let mut __slate_storage_788: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_788: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_788) as *mut i64;
    let mut __slate_storage_787: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_787: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_787) as *mut i64;
    let mut __slate_storage_1649: std::mem::MaybeUninit<bool> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1649: *mut bool = std::ptr::addr_of_mut!(__slate_storage_1649) as *mut bool;
    let mut __slate_storage_786: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_786: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_786) as *mut i64;
    let mut __slate_storage_1648: std::mem::MaybeUninit<bool> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1648: *mut bool = std::ptr::addr_of_mut!(__slate_storage_1648) as *mut bool;
    let mut __slate_storage_785: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_785: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_785) as *mut i64;
    let mut __slate_storage_1647: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1647: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1647) as *mut i32;
    let mut __slate_storage_1646: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1646: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1646) as *mut i32;
    let mut __slate_storage_784: std::mem::MaybeUninit<*mut VdbeOp> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_784: *mut *mut VdbeOp =
        std::ptr::addr_of_mut!(__slate_storage_784) as *mut *mut VdbeOp;
    let mut __slate_storage_783: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_783: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_783) as *mut i32;
    let mut __slate_storage_782: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_782: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_782) as *mut i32;
    let mut __slate_storage_1639: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1639: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1639) as *mut i32;
    let mut __slate_storage_1638: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1638: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1638) as *mut i32;
    let mut __slate_storage_781: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_781: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_781) as *mut i32; // 10x size change
    let mut __slate_storage_780: std::mem::MaybeUninit<i16> = std::mem::MaybeUninit::uninit();
    let __slate_slot_780: *mut i16 = std::ptr::addr_of_mut!(__slate_storage_780) as *mut i16;
    let mut __slate_storage_1645: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1645: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1645) as *mut i32;
    let mut __slate_storage_1644: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1644: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1644) as *mut i32;
    let mut __slate_storage_1643: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1643: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1643) as *mut i32;
    let mut __slate_storage_1642: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1642: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1642) as *mut i32;
    let mut __slate_storage_1641: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1641: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1641) as *mut i32;
    let mut __slate_storage_1640: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1640: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1640) as *mut i32;
    let mut __slate_storage_1637: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1637: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1637) as *mut i32;
    let mut __slate_storage_1636: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1636: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1636) as *mut i32;
    let mut __slate_storage_1635: std::mem::MaybeUninit<*mut Parse> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1635: *mut *mut Parse =
        std::ptr::addr_of_mut!(__slate_storage_1635) as *mut *mut Parse; // Number of indexes on the current table
    let mut __slate_storage_779: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_779: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_779) as *mut i32; // Number of btrees to scan
    let mut __slate_storage_778: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_778: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_778) as *mut i32; // Number of tables to be optimized
    let mut __slate_storage_777: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_777: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_777) as *mut i32; // Analysis limit to use
    let mut __slate_storage_776: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_776: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_776) as *mut i32; // Mask of operations to perform
    let mut __slate_storage_775: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_775: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_775) as *mut u32; // SQL statement for the OP_SqlExec opcode
    let mut __slate_storage_774: std::mem::MaybeUninit<*mut i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_774: *mut *mut i8 =
        std::ptr::addr_of_mut!(__slate_storage_774) as *mut *mut i8; // Size threshold above which reanalysis needed
    let mut __slate_storage_773: std::mem::MaybeUninit<i16> = std::mem::MaybeUninit::uninit();
    let __slate_slot_773: *mut i16 = std::ptr::addr_of_mut!(__slate_storage_773) as *mut i16; // An index of the table
    let mut __slate_storage_772: std::mem::MaybeUninit<*mut Index> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_772: *mut *mut Index =
        std::ptr::addr_of_mut!(__slate_storage_772) as *mut *mut Index; // A table in the schema
    let mut __slate_storage_771: std::mem::MaybeUninit<*mut Table> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_771: *mut *mut Table =
        std::ptr::addr_of_mut!(__slate_storage_771) as *mut *mut Table; // The current schema
    let mut __slate_storage_770: std::mem::MaybeUninit<*mut Schema> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_770: *mut *mut Schema =
        std::ptr::addr_of_mut!(__slate_storage_770) as *mut *mut Schema; // Loop over tables of a schema
    let mut __slate_storage_769: std::mem::MaybeUninit<*mut HashElem> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_769: *mut *mut HashElem =
        std::ptr::addr_of_mut!(__slate_storage_769) as *mut *mut HashElem; // Cursor for a table whose size needs checking
    let mut __slate_storage_768: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_768: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_768) as *mut i32; // Loop termination point for the schema loop
    let mut __slate_storage_767: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_767: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_767) as *mut i32;
    let mut __slate_storage_766: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_766: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_766) as *mut i32;
    let mut __slate_storage_765: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_765: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_765) as *mut i32;
    let mut __slate_storage_1634: std::mem::MaybeUninit<*const i8> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1634: *mut *const i8 =
        std::ptr::addr_of_mut!(__slate_storage_1634) as *mut *const i8;
    let mut __slate_storage_1633: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1633: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1633) as *mut i32;
    let mut __slate_storage_1632: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1632: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1632) as *mut i32;
    let mut __slate_storage_764: std::mem::MaybeUninit<*const i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_764: *mut *const i8 =
        std::ptr::addr_of_mut!(__slate_storage_764) as *mut *const i8;
    let mut __slate_storage_763: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_763: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_763) as *mut i32;
    let mut __slate_storage_760: std::mem::MaybeUninit<*mut VdbeOp> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_760: *mut *mut VdbeOp =
        std::ptr::addr_of_mut!(__slate_storage_760) as *mut *mut VdbeOp;
    let mut __slate_storage_762: std::mem::MaybeUninit<*mut VdbeOp> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_762: *mut *mut VdbeOp =
        std::ptr::addr_of_mut!(__slate_storage_762) as *mut *mut VdbeOp; // Which cookie to read or write
    let mut __slate_storage_758: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_758: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_758) as *mut i32;
    let mut __slate_storage_1631: std::mem::MaybeUninit<*const EncName> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1631: *mut *const EncName =
        std::ptr::addr_of_mut!(__slate_storage_1631) as *mut *const EncName;
    let mut __slate_storage_1630: std::mem::MaybeUninit<*const EncName> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1630: *mut *const EncName =
        std::ptr::addr_of_mut!(__slate_storage_1630) as *mut *const EncName;
    let mut __slate_storage_757: std::mem::MaybeUninit<u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_757: *mut u8 = std::ptr::addr_of_mut!(__slate_storage_757) as *mut u8;
    let mut __slate_storage_756: std::mem::MaybeUninit<*const EncName> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_756: *mut *const EncName =
        std::ptr::addr_of_mut!(__slate_storage_756) as *mut *const EncName;
    let mut __slate_storage_753: std::mem::MaybeUninit<*mut VdbeOp> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_753: *mut *mut VdbeOp =
        std::ptr::addr_of_mut!(__slate_storage_753) as *mut *mut VdbeOp;
    let mut __slate_storage_1585: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1585: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1585) as *mut i32;
    let mut __slate_storage_1584: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1584: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1584) as *mut i32;
    let mut __slate_storage_1629: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1629: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_1629) as *mut u32;
    let mut __slate_storage_1628: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1628: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_1628) as *mut u32;
    let mut __slate_storage_1627: std::mem::MaybeUninit<*mut Table> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1627: *mut *mut Table =
        std::ptr::addr_of_mut!(__slate_storage_1627) as *mut *mut Table;
    let mut __slate_storage_750: std::mem::MaybeUninit<*const i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_750: *mut *const i8 =
        std::ptr::addr_of_mut!(__slate_storage_750) as *mut *const i8;
    let mut __slate_storage_749: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_749: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_749) as *mut i32;
    let mut __slate_storage_748: std::mem::MaybeUninit<*mut sqlite3_vtab> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_748: *mut *mut sqlite3_vtab =
        std::ptr::addr_of_mut!(__slate_storage_748) as *mut *mut sqlite3_vtab;
    let mut __slate_storage_747: std::mem::MaybeUninit<*mut Table> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_747: *mut *mut Table =
        std::ptr::addr_of_mut!(__slate_storage_747) as *mut *mut Table;
    let mut __slate_storage_1622: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1622: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1622) as *mut i32;
    let mut __slate_storage_1621: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1621: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1621) as *mut i32;
    let mut __slate_storage_1626: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1626: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1626) as *mut i32;
    let mut __slate_storage_1625: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1625: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1625) as *mut i32;
    let mut __slate_storage_746: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_746: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_746) as *mut i32;
    let mut __slate_storage_745: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_745: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_745) as *mut i32;
    let mut __slate_storage_744: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_744: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_744) as *mut i32;
    let mut __slate_storage_743: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_743: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_743) as *mut i32;
    let mut __slate_storage_1624: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1624: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1624) as *mut i32;
    let mut __slate_storage_1623: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1623: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1623) as *mut i32;
    let mut __slate_storage_742: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_742: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_742) as *mut i32;
    let mut __slate_storage_741: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_741: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_741) as *mut i32;
    let mut __slate_storage_740: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_740: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_740) as *mut i32;
    let mut __slate_storage_739: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_739: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_739) as *mut i32;
    let mut __slate_storage_738: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_738: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_738) as *mut i32;
    let mut __slate_storage_737: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_737: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_737) as *mut i32;
    let mut __slate_storage_736: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_736: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_736) as *mut i32;
    let mut __slate_storage_735: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_735: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_735) as *mut i32;
    let mut __slate_storage_1620: std::mem::MaybeUninit<*mut Index> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1620: *mut *mut Index =
        std::ptr::addr_of_mut!(__slate_storage_1620) as *mut *mut Index;
    let mut __slate_storage_1619: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1619: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1619) as *mut i32;
    let mut __slate_storage_1618: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1618: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1618) as *mut i32;
    let mut __slate_storage_734: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_734: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_734) as *mut i32;
    let mut __slate_storage_733: std::mem::MaybeUninit<*mut i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_733: *mut *mut i8 =
        std::ptr::addr_of_mut!(__slate_storage_733) as *mut *mut i8;
    let mut __slate_storage_732: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_732: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_732) as *mut i32;
    let mut __slate_storage_731: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_731: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_731) as *mut i32;
    let mut __slate_storage_730: std::mem::MaybeUninit<*mut ExprList> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_730: *mut *mut ExprList =
        std::ptr::addr_of_mut!(__slate_storage_730) as *mut *mut ExprList;
    let mut __slate_storage_1617: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1617: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1617) as *mut i32;
    let mut __slate_storage_1616: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1616: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1616) as *mut i32;
    let mut __slate_storage_728: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_728: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_728) as *mut i32;
    // (1) NOT NULL columns may not contain a NULL
    let mut __slate_storage_727: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_727: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_727) as *mut i32;
    let mut __slate_storage_726: std::mem::MaybeUninit<*mut sqlite3_value> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_726: *mut *mut sqlite3_value =
        std::ptr::addr_of_mut!(__slate_storage_726) as *mut *mut sqlite3_value; // Check datatypes (besides NOT NULL)
    let mut __slate_storage_725: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_725: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_725) as *mut i32; // Operands to the OP_IsType opcode
    let mut __slate_storage_724: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_724: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_724) as *mut i32;
    let mut __slate_storage_723: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_723: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_723) as *mut i32;
    let mut __slate_storage_722: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_722: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_722) as *mut i32; // Jump here if all looks ok
    let mut __slate_storage_721: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_721: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_721) as *mut i32; // Jump here to report an error
    let mut __slate_storage_720: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_720: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_720) as *mut i32; // The column to be checked
    let mut __slate_storage_719: std::mem::MaybeUninit<*mut Column> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_719: *mut *mut Column =
        std::ptr::addr_of_mut!(__slate_storage_719) as *mut *mut Column;
    let mut __slate_storage_718: std::mem::MaybeUninit<*mut i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_718: *mut *mut i8 =
        std::ptr::addr_of_mut!(__slate_storage_718) as *mut *mut i8;
    let mut __slate_storage_1615: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1615: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1615) as *mut i32;
    let mut __slate_storage_1614: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1614: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1614) as *mut i32;
    let mut __slate_storage_717: std::mem::MaybeUninit<*mut i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_717: *mut *mut i8 =
        std::ptr::addr_of_mut!(__slate_storage_717) as *mut *mut i8;
    // Verify WITHOUT ROWID keys are in ascending order
    let mut __slate_storage_716: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_716: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_716) as *mut i32;
    let mut __slate_storage_1613: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1613: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1613) as *mut i32;
    let mut __slate_storage_1612: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1612: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1612) as *mut i32;
    let mut __slate_storage_1609: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1609: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1609) as *mut i32;
    let mut __slate_storage_1608: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1608: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1608) as *mut i32;
    let mut __slate_storage_1611: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1611: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1611) as *mut i32;
    let mut __slate_storage_1610: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1610: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1610) as *mut i32;
    let mut __slate_storage_1607: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1607: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1607) as *mut i32;
    let mut __slate_storage_1606: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1606: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1606) as *mut i32;
    let mut __slate_storage_1605: std::mem::MaybeUninit<*mut Index> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1605: *mut *mut Index =
        std::ptr::addr_of_mut!(__slate_storage_1605) as *mut *mut Index; // Maximum non-virtual column number
    let mut __slate_storage_715: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_715: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_715) as *mut i32; // Previous key for WITHOUT ROWID tables
    let mut __slate_storage_714: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_714: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_714) as *mut i32; // True for a STRICT table
    let mut __slate_storage_713: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_713: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_713) as *mut i32;
    let mut __slate_storage_712: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_712: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_712) as *mut i32;
    let mut __slate_storage_711: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_711: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_711) as *mut i32;
    let mut __slate_storage_710: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_710: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_710) as *mut i32;
    let mut __slate_storage_709: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_709: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_709) as *mut i32; // Previous index
    let mut __slate_storage_708: std::mem::MaybeUninit<*mut Index> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_708: *mut *mut Index =
        std::ptr::addr_of_mut!(__slate_storage_708) as *mut *mut Index;
    let mut __slate_storage_707: std::mem::MaybeUninit<*mut Index> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_707: *mut *mut Index =
        std::ptr::addr_of_mut!(__slate_storage_707) as *mut *mut Index;
    let mut __slate_storage_706: std::mem::MaybeUninit<*mut Index> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_706: *mut *mut Index =
        std::ptr::addr_of_mut!(__slate_storage_706) as *mut *mut Index;
    let mut __slate_storage_705: std::mem::MaybeUninit<*mut Table> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_705: *mut *mut Table =
        std::ptr::addr_of_mut!(__slate_storage_705) as *mut *mut Table;
    let mut __slate_storage_1604: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1604: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1604) as *mut i32;
    let mut __slate_storage_1603: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1603: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1603) as *mut i32;
    let mut __slate_storage_1600: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1600: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1600) as *mut i32;
    let mut __slate_storage_1599: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1599: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1599) as *mut i32;
    let mut __slate_storage_1602: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1602: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1602) as *mut i32;
    let mut __slate_storage_1601: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1601: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1601) as *mut i32;
    let mut __slate_storage_704: std::mem::MaybeUninit<*mut Index> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_704: *mut *mut Index =
        std::ptr::addr_of_mut!(__slate_storage_704) as *mut *mut Index;
    let mut __slate_storage_703: std::mem::MaybeUninit<*mut Table> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_703: *mut *mut Table =
        std::ptr::addr_of_mut!(__slate_storage_703) as *mut *mut Table;
    let mut __slate_storage_702: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_702: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_702) as *mut i32;
    let mut __slate_storage_1598: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1598: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1598) as *mut i32;
    let mut __slate_storage_1597: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1597: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1597) as *mut i32;
    let mut __slate_storage_1596: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1596: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1596) as *mut i32;
    let mut __slate_storage_1595: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1595: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1595) as *mut i32;
    let mut __slate_storage_701: std::mem::MaybeUninit<*mut Index> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_701: *mut *mut Index =
        std::ptr::addr_of_mut!(__slate_storage_701) as *mut *mut Index;
    let mut __slate_storage_700: std::mem::MaybeUninit<*mut Table> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_700: *mut *mut Table =
        std::ptr::addr_of_mut!(__slate_storage_700) as *mut *mut Table;
    let mut __slate_storage_1594: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1594: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1594) as *mut i32;
    let mut __slate_storage_1593: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1593: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1593) as *mut i32;
    let mut __slate_storage_1592: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1592: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1592) as *mut i32;
    let mut __slate_storage_1591: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1591: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1591) as *mut i32;
    let mut __slate_storage_1590: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1590: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1590) as *mut i32;
    let mut __slate_storage_1589: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1589: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1589) as *mut i32;
    let mut __slate_storage_1588: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1588: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1588) as *mut i32;
    let mut __slate_storage_1587: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1587: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1587) as *mut i32; // An index on pTab
    let mut __slate_storage_699: std::mem::MaybeUninit<*mut Index> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_699: *mut *mut Index =
        std::ptr::addr_of_mut!(__slate_storage_699) as *mut *mut Index; // Current table
    let mut __slate_storage_698: std::mem::MaybeUninit<*mut Table> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_698: *mut *mut Table =
        std::ptr::addr_of_mut!(__slate_storage_698) as *mut *mut Table;
    let mut __slate_storage_1586: std::mem::MaybeUninit<*mut HashElem> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1586: *mut *mut HashElem =
        std::ptr::addr_of_mut!(__slate_storage_1586) as *mut *mut HashElem; // Number of entries in aRoot[]
    let mut __slate_storage_697: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_697: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_697) as *mut i32; // Array of root page numbers of all btrees
    let mut __slate_storage_696: std::mem::MaybeUninit<*mut i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_696: *mut *mut i32 =
        std::ptr::addr_of_mut!(__slate_storage_696) as *mut *mut i32; // Set of all tables in the schema
    let mut __slate_storage_695: std::mem::MaybeUninit<*mut Hash> = std::mem::MaybeUninit::uninit();
    let __slate_slot_695: *mut *mut Hash =
        std::ptr::addr_of_mut!(__slate_storage_695) as *mut *mut Hash; // For looping over tables in the schema
    let mut __slate_storage_694: std::mem::MaybeUninit<*mut HashElem> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_694: *mut *mut HashElem =
        std::ptr::addr_of_mut!(__slate_storage_694) as *mut *mut HashElem;
    let mut __slate_storage_693: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_693: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_693) as *mut i32; // Check only this one table, if not NULL
    let mut __slate_storage_692: std::mem::MaybeUninit<*mut Table> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_692: *mut *mut Table =
        std::ptr::addr_of_mut!(__slate_storage_692) as *mut *mut Table;
    let mut __slate_storage_691: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_691: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_691) as *mut i32;
    let mut __slate_storage_690: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_690: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_690) as *mut i32;
    let mut __slate_storage_689: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_689: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_689) as *mut i32;
    let mut __slate_storage_688: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_688: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_688) as *mut i32;
    let mut __slate_storage_1581: std::mem::MaybeUninit<*mut FKey> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1581: *mut *mut FKey =
        std::ptr::addr_of_mut!(__slate_storage_1581) as *mut *mut FKey;
    let mut __slate_storage_1580: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1580: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1580) as *mut i32;
    let mut __slate_storage_1579: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1579: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1579) as *mut i32;
    let mut __slate_storage_687: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_687: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_687) as *mut i32;
    let mut __slate_storage_1583: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1583: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1583) as *mut i32;
    let mut __slate_storage_1582: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1582: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1582) as *mut i32;
    let mut __slate_storage_686: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_686: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_686) as *mut i32;
    let mut __slate_storage_1578: std::mem::MaybeUninit<*mut FKey> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1578: *mut *mut FKey =
        std::ptr::addr_of_mut!(__slate_storage_1578) as *mut *mut FKey;
    let mut __slate_storage_1577: std::mem::MaybeUninit<*mut FKey> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1577: *mut *mut FKey =
        std::ptr::addr_of_mut!(__slate_storage_1577) as *mut *mut FKey;
    let mut __slate_storage_1576: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1576: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1576) as *mut i32;
    let mut __slate_storage_1575: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1575: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1575) as *mut i32;
    let mut __slate_storage_1574: std::mem::MaybeUninit<*mut FKey> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1574: *mut *mut FKey =
        std::ptr::addr_of_mut!(__slate_storage_1574) as *mut *mut FKey;
    let mut __slate_storage_1573: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1573: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1573) as *mut i32;
    let mut __slate_storage_1572: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1572: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1572) as *mut i32;
    let mut __slate_storage_1571: std::mem::MaybeUninit<*mut Parse> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1571: *mut *mut Parse =
        std::ptr::addr_of_mut!(__slate_storage_1571) as *mut *mut Parse;
    let mut __slate_storage_1570: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1570: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1570) as *mut i32;
    let mut __slate_storage_1569: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1569: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1569) as *mut i32;
    let mut __slate_storage_1568: std::mem::MaybeUninit<*mut Parse> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1568: *mut *mut Parse =
        std::ptr::addr_of_mut!(__slate_storage_1568) as *mut *mut Parse; // child to parent column mapping
    let mut __slate_storage_685: std::mem::MaybeUninit<*mut i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_685: *mut *mut i32 =
        std::ptr::addr_of_mut!(__slate_storage_685) as *mut *mut i32; // Jump here if the key is OK
    let mut __slate_storage_684: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_684: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_684) as *mut i32; // Top of a loop checking foreign keys
    let mut __slate_storage_683: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_683: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_683) as *mut i32; // Registers to hold a row from pTab
    let mut __slate_storage_682: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_682: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_682) as *mut i32; // 3 registers to hold a result row
    let mut __slate_storage_681: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_681: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_681) as *mut i32; // result variable
    let mut __slate_storage_680: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_680: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_680) as *mut i32; // Loop counter:  Next table in schema
    let mut __slate_storage_679: std::mem::MaybeUninit<*mut HashElem> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_679: *mut *mut HashElem =
        std::ptr::addr_of_mut!(__slate_storage_679) as *mut *mut HashElem; // Loop counter:  Field of the foreign key
    let mut __slate_storage_678: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_678: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_678) as *mut i32; // Loop counter:  Foreign key number for pTab
    let mut __slate_storage_677: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_677: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_677) as *mut i32; // Index in the parent table
    let mut __slate_storage_676: std::mem::MaybeUninit<*mut Index> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_676: *mut *mut Index =
        std::ptr::addr_of_mut!(__slate_storage_676) as *mut *mut Index; // Parent table that child points to
    let mut __slate_storage_675: std::mem::MaybeUninit<*mut Table> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_675: *mut *mut Table =
        std::ptr::addr_of_mut!(__slate_storage_675) as *mut *mut Table; // Child table contain "REFERENCES" keyword
    let mut __slate_storage_674: std::mem::MaybeUninit<*mut Table> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_674: *mut *mut Table =
        std::ptr::addr_of_mut!(__slate_storage_674) as *mut *mut Table; // A foreign key constraint
    let mut __slate_storage_673: std::mem::MaybeUninit<*mut FKey> = std::mem::MaybeUninit::uninit();
    let __slate_slot_673: *mut *mut FKey =
        std::ptr::addr_of_mut!(__slate_storage_673) as *mut *mut FKey;
    let mut __slate_storage_1567: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1567: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1567) as *mut i32;
    let mut __slate_storage_1566: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1566: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1566) as *mut i32;
    let mut __slate_storage_1565: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1565: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1565) as *mut i32;
    let mut __slate_storage_1564: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1564: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1564) as *mut i32;
    let mut __slate_storage_672: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_672: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_672) as *mut i32;
    let mut __slate_storage_671: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_671: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_671) as *mut i32;
    let mut __slate_storage_670: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_670: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_670) as *mut i32;
    let mut __slate_storage_669: std::mem::MaybeUninit<*mut Table> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_669: *mut *mut Table =
        std::ptr::addr_of_mut!(__slate_storage_669) as *mut *mut Table;
    let mut __slate_storage_668: std::mem::MaybeUninit<*mut FKey> = std::mem::MaybeUninit::uninit();
    let __slate_slot_668: *mut *mut FKey =
        std::ptr::addr_of_mut!(__slate_storage_668) as *mut *mut FKey;
    let mut __slate_storage_1563: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1563: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1563) as *mut i32;
    let mut __slate_storage_1562: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1562: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1562) as *mut i32;
    let mut __slate_storage_667: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_667: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_667) as *mut i32;
    let mut __slate_storage_666: std::mem::MaybeUninit<*mut Module> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_666: *mut *mut Module =
        std::ptr::addr_of_mut!(__slate_storage_666) as *mut *mut Module;
    let mut __slate_storage_665: std::mem::MaybeUninit<*mut HashElem> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_665: *mut *mut HashElem =
        std::ptr::addr_of_mut!(__slate_storage_665) as *mut *mut HashElem;
    let mut __slate_storage_1561: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1561: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1561) as *mut i32;
    let mut __slate_storage_1560: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1560: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1560) as *mut i32;
    let mut __slate_storage_664: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_664: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_664) as *mut i32;
    let mut __slate_storage_663: std::mem::MaybeUninit<*mut FuncDef> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_663: *mut *mut FuncDef =
        std::ptr::addr_of_mut!(__slate_storage_663) as *mut *mut FuncDef;
    let mut __slate_storage_662: std::mem::MaybeUninit<*mut HashElem> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_662: *mut *mut HashElem =
        std::ptr::addr_of_mut!(__slate_storage_662) as *mut *mut HashElem;
    let mut __slate_storage_661: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_661: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_661) as *mut i32;
    let mut __slate_storage_1559: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1559: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1559) as *mut i32;
    let mut __slate_storage_1558: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1558: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1558) as *mut i32;
    let mut __slate_storage_660: std::mem::MaybeUninit<*mut CollSeq> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_660: *mut *mut CollSeq =
        std::ptr::addr_of_mut!(__slate_storage_660) as *mut *mut CollSeq;
    let mut __slate_storage_659: std::mem::MaybeUninit<*mut HashElem> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_659: *mut *mut HashElem =
        std::ptr::addr_of_mut!(__slate_storage_659) as *mut *mut HashElem;
    let mut __slate_storage_658: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_658: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_658) as *mut i32;
    let mut __slate_storage_1557: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1557: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1557) as *mut i32;
    let mut __slate_storage_1556: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1556: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1556) as *mut i32;
    let mut __slate_storage_657: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_657: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_657) as *mut i32;
    let mut __slate_storage_1555: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1555: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1555) as *mut i32;
    let mut __slate_storage_1554: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1554: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1554) as *mut i32;
    let mut __slate_storage_656: std::mem::MaybeUninit<__SlateAlign16<[*const i8; 3]>> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_656: *mut [*const i8; 3] =
        std::ptr::addr_of_mut!(__slate_storage_656) as *mut [*const i8; 3];
    let mut __slate_storage_655: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_655: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_655) as *mut i32;
    let mut __slate_storage_654: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_654: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_654) as *mut i32;
    let mut __slate_storage_653: std::mem::MaybeUninit<*mut Table> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_653: *mut *mut Table =
        std::ptr::addr_of_mut!(__slate_storage_653) as *mut *mut Table;
    let mut __slate_storage_652: std::mem::MaybeUninit<*mut Index> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_652: *mut *mut Index =
        std::ptr::addr_of_mut!(__slate_storage_652) as *mut *mut Index;
    let mut __slate_storage_1553: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1553: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1553) as *mut i32;
    let mut __slate_storage_1552: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1552: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1552) as *mut i32;
    let mut __slate_storage_651: std::mem::MaybeUninit<i16> = std::mem::MaybeUninit::uninit();
    let __slate_slot_651: *mut i16 = std::ptr::addr_of_mut!(__slate_storage_651) as *mut i16;
    let mut __slate_storage_650: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_650: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_650) as *mut i32;
    let mut __slate_storage_649: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_649: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_649) as *mut i32;
    let mut __slate_storage_648: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_648: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_648) as *mut i32;
    let mut __slate_storage_647: std::mem::MaybeUninit<*mut Table> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_647: *mut *mut Table =
        std::ptr::addr_of_mut!(__slate_storage_647) as *mut *mut Table;
    let mut __slate_storage_646: std::mem::MaybeUninit<*mut Index> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_646: *mut *mut Index =
        std::ptr::addr_of_mut!(__slate_storage_646) as *mut *mut Index;
    let mut __slate_storage_1547: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1547: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1547) as *mut i32;
    let mut __slate_storage_1546: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1546: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1546) as *mut i32;
    let mut __slate_storage_1551: std::mem::MaybeUninit<bool> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1551: *mut bool = std::ptr::addr_of_mut!(__slate_storage_1551) as *mut bool;
    let mut __slate_storage_645: std::mem::MaybeUninit<*const i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_645: *mut *const i8 =
        std::ptr::addr_of_mut!(__slate_storage_645) as *mut *const i8;
    let mut __slate_storage_644: std::mem::MaybeUninit<*mut Table> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_644: *mut *mut Table =
        std::ptr::addr_of_mut!(__slate_storage_644) as *mut *mut Table;
    let mut __slate_storage_1550: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1550: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1550) as *mut i32;
    let mut __slate_storage_1549: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1549: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1549) as *mut i32;
    let mut __slate_storage_643: std::mem::MaybeUninit<*mut sqlite3_stmt> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_643: *mut *mut sqlite3_stmt =
        std::ptr::addr_of_mut!(__slate_storage_643) as *mut *mut sqlite3_stmt;
    let mut __slate_storage_642: std::mem::MaybeUninit<*mut i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_642: *mut *mut i8 =
        std::ptr::addr_of_mut!(__slate_storage_642) as *mut *mut i8;
    let mut __slate_storage_641: std::mem::MaybeUninit<*mut Table> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_641: *mut *mut Table =
        std::ptr::addr_of_mut!(__slate_storage_641) as *mut *mut Table;
    let mut __slate_storage_1548: std::mem::MaybeUninit<bool> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1548: *mut bool = std::ptr::addr_of_mut!(__slate_storage_1548) as *mut bool;
    let mut __slate_storage_640: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_640: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_640) as *mut i32;
    let mut __slate_storage_639: std::mem::MaybeUninit<*mut Hash> = std::mem::MaybeUninit::uninit();
    let __slate_slot_639: *mut *mut Hash =
        std::ptr::addr_of_mut!(__slate_storage_639) as *mut *mut Hash;
    let mut __slate_storage_638: std::mem::MaybeUninit<*mut HashElem> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_638: *mut *mut HashElem =
        std::ptr::addr_of_mut!(__slate_storage_638) as *mut *mut HashElem;
    let mut __slate_storage_637: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_637: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_637) as *mut i32;
    let mut __slate_storage_1541: std::mem::MaybeUninit<*mut Column> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1541: *mut *mut Column =
        std::ptr::addr_of_mut!(__slate_storage_1541) as *mut *mut Column;
    let mut __slate_storage_1540: std::mem::MaybeUninit<*mut Column> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1540: *mut *mut Column =
        std::ptr::addr_of_mut!(__slate_storage_1540) as *mut *mut Column;
    let mut __slate_storage_1539: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1539: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1539) as *mut i32;
    let mut __slate_storage_1538: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1538: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1538) as *mut i32;
    let mut __slate_storage_1545: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1545: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1545) as *mut i32;
    let mut __slate_storage_1544: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1544: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1544) as *mut i32;
    let mut __slate_storage_1543: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1543: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1543) as *mut i32;
    let mut __slate_storage_1542: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1542: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1542) as *mut i32;
    let mut __slate_storage_636: std::mem::MaybeUninit<*const Expr> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_636: *mut *const Expr =
        std::ptr::addr_of_mut!(__slate_storage_636) as *mut *const Expr;
    let mut __slate_storage_635: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_635: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_635) as *mut i32;
    let mut __slate_storage_1537: std::mem::MaybeUninit<*mut Column> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1537: *mut *mut Column =
        std::ptr::addr_of_mut!(__slate_storage_1537) as *mut *mut Column;
    let mut __slate_storage_634: std::mem::MaybeUninit<*mut Index> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_634: *mut *mut Index =
        std::ptr::addr_of_mut!(__slate_storage_634) as *mut *mut Index;
    let mut __slate_storage_633: std::mem::MaybeUninit<*mut Column> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_633: *mut *mut Column =
        std::ptr::addr_of_mut!(__slate_storage_633) as *mut *mut Column;
    let mut __slate_storage_632: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_632: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_632) as *mut i32;
    let mut __slate_storage_631: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_631: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_631) as *mut i32;
    let mut __slate_storage_630: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_630: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_630) as *mut i32;
    let mut __slate_storage_629: std::mem::MaybeUninit<*mut Table> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_629: *mut *mut Table =
        std::ptr::addr_of_mut!(__slate_storage_629) as *mut *mut Table;
    let mut __slate_storage_1532: std::mem::MaybeUninit<u64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1532: *mut u64 = std::ptr::addr_of_mut!(__slate_storage_1532) as *mut u64;
    let mut __slate_storage_1531: std::mem::MaybeUninit<u64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1531: *mut u64 = std::ptr::addr_of_mut!(__slate_storage_1531) as *mut u64;
    let mut __slate_storage_1530: std::mem::MaybeUninit<*mut sqlite3> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1530: *mut *mut sqlite3 =
        std::ptr::addr_of_mut!(__slate_storage_1530) as *mut *mut sqlite3;
    let mut __slate_storage_1536: std::mem::MaybeUninit<bool> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1536: *mut bool = std::ptr::addr_of_mut!(__slate_storage_1536) as *mut bool;
    let mut __slate_storage_1535: std::mem::MaybeUninit<u64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1535: *mut u64 = std::ptr::addr_of_mut!(__slate_storage_1535) as *mut u64;
    let mut __slate_storage_1534: std::mem::MaybeUninit<u64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1534: *mut u64 = std::ptr::addr_of_mut!(__slate_storage_1534) as *mut u64;
    let mut __slate_storage_1533: std::mem::MaybeUninit<*mut sqlite3> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1533: *mut *mut sqlite3 =
        std::ptr::addr_of_mut!(__slate_storage_1533) as *mut *mut sqlite3;
    let mut __slate_storage_1529: std::mem::MaybeUninit<u64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1529: *mut u64 = std::ptr::addr_of_mut!(__slate_storage_1529) as *mut u64;
    // Foreign key support may not be enabled or disabled while not
    // in auto-commit mode.
    let mut __slate_storage_1528: std::mem::MaybeUninit<u64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1528: *mut u64 = std::ptr::addr_of_mut!(__slate_storage_1528) as *mut u64; // Mask of bits to set or clear.
    let mut __slate_storage_628: std::mem::MaybeUninit<u64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_628: *mut u64 = std::ptr::addr_of_mut!(__slate_storage_628) as *mut u64;
    let mut __slate_storage_627: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_627: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_627) as *mut i32;
    let mut __slate_storage_626: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_626: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_626) as *mut i32;
    let mut __slate_storage_1527: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1527: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1527) as *mut i32;
    let mut __slate_storage_1526: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1526: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1526) as *mut i32;
    let mut __slate_storage_1525: std::mem::MaybeUninit<*mut Parse> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1525: *mut *mut Parse =
        std::ptr::addr_of_mut!(__slate_storage_1525) as *mut *mut Parse;
    let mut __slate_storage_1524: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1524: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1524) as *mut i32;
    let mut __slate_storage_1523: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1523: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1523) as *mut i32;
    let mut __slate_storage_625: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_625: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_625) as *mut i32;
    let mut __slate_storage_624: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_624: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_624) as *mut i64;
    let mut __slate_storage_1516: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1516: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1516) as *mut i32;
    let mut __slate_storage_1519: std::mem::MaybeUninit<u64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1519: *mut u64 = std::ptr::addr_of_mut!(__slate_storage_1519) as *mut u64;
    let mut __slate_storage_1518: std::mem::MaybeUninit<u64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1518: *mut u64 = std::ptr::addr_of_mut!(__slate_storage_1518) as *mut u64;
    let mut __slate_storage_1517: std::mem::MaybeUninit<*mut sqlite3> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1517: *mut *mut sqlite3 =
        std::ptr::addr_of_mut!(__slate_storage_1517) as *mut *mut sqlite3;
    let mut __slate_storage_1522: std::mem::MaybeUninit<u64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1522: *mut u64 = std::ptr::addr_of_mut!(__slate_storage_1522) as *mut u64;
    let mut __slate_storage_1521: std::mem::MaybeUninit<u64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1521: *mut u64 = std::ptr::addr_of_mut!(__slate_storage_1521) as *mut u64;
    let mut __slate_storage_1520: std::mem::MaybeUninit<*mut sqlite3> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1520: *mut *mut sqlite3 =
        std::ptr::addr_of_mut!(__slate_storage_1520) as *mut *mut sqlite3;
    let mut __slate_storage_623: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_623: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_623) as *mut i32;
    let mut __slate_storage_622: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_622: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_622) as *mut i32;
    let mut __slate_storage_1515: std::mem::MaybeUninit<bool> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1515: *mut bool = std::ptr::addr_of_mut!(__slate_storage_1515) as *mut bool;
    let mut __slate_storage_621: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_621: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_621) as *mut i32;
    let mut __slate_storage_620: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_620: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_620) as *mut i32;
    let mut __slate_storage_619: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_619: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_619) as *mut i32;
    let mut __slate_storage_618: std::mem::MaybeUninit<*mut VdbeOp> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_618: *mut *mut VdbeOp =
        std::ptr::addr_of_mut!(__slate_storage_618) as *mut *mut VdbeOp;
    let mut __slate_storage_615: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_615: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_615) as *mut i32;
    let mut __slate_storage_614: std::mem::MaybeUninit<*mut Btree> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_614: *mut *mut Btree =
        std::ptr::addr_of_mut!(__slate_storage_614) as *mut *mut Btree;
    let mut __slate_storage_613: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_613: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_613) as *mut i64;
    let mut __slate_storage_612: std::mem::MaybeUninit<*mut Pager> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_612: *mut *mut Pager =
        std::ptr::addr_of_mut!(__slate_storage_612) as *mut *mut Pager;
    let mut __slate_storage_1514: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1514: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1514) as *mut i32;
    let mut __slate_storage_1513: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1513: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1513) as *mut i32;
    let mut __slate_storage_1510: std::mem::MaybeUninit<*const i8> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1510: *mut *const i8 =
        std::ptr::addr_of_mut!(__slate_storage_1510) as *mut *const i8;
    let mut __slate_storage_1512: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1512: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1512) as *mut i32;
    let mut __slate_storage_1511: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1511: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1511) as *mut i32;
    let mut __slate_storage_611: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_611: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_611) as *mut i32;
    let mut __slate_storage_610: std::mem::MaybeUninit<*const i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_610: *mut *const i8 =
        std::ptr::addr_of_mut!(__slate_storage_610) as *mut *const i8; // Loop counter
    let mut __slate_storage_609: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_609: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_609) as *mut i32; // One of the PAGER_JOURNALMODE_XXX symbols
    let mut __slate_storage_608: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_608: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_608) as *mut i32;
    let mut __slate_storage_1509: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1509: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1509) as *mut i32;
    let mut __slate_storage_1508: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1508: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1508) as *mut i32;
    // This indicates that no database name was specified as part
    // of the PRAGMA command. In this case the locking-mode must be
    // set on all attached databases, as well as the main db file.
    //
    // Also, the sqlite3.dfltLockMode variable is set so that
    // any subsequently attached databases also use the specified
    // locking mode.
    let mut __slate_storage_607: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_607: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_607) as *mut i32;
    let mut __slate_storage_606: std::mem::MaybeUninit<*mut Pager> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_606: *mut *mut Pager =
        std::ptr::addr_of_mut!(__slate_storage_606) as *mut *mut Pager;
    let mut __slate_storage_605: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_605: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_605) as *mut i32;
    let mut __slate_storage_604: std::mem::MaybeUninit<*const i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_604: *mut *const i8 =
        std::ptr::addr_of_mut!(__slate_storage_604) as *mut *const i8;
    let mut __slate_storage_1507: std::mem::MaybeUninit<bool> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1507: *mut bool = std::ptr::addr_of_mut!(__slate_storage_1507) as *mut bool;
    let mut __slate_storage_1506: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1506: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1506) as *mut i32;
    let mut __slate_storage_1505: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1505: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1505) as *mut i32;
    let mut __slate_storage_1504: std::mem::MaybeUninit<*mut Parse> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1504: *mut *mut Parse =
        std::ptr::addr_of_mut!(__slate_storage_1504) as *mut *mut Parse;
    let mut __slate_storage_603: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_603: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_603) as *mut i64;
    let mut __slate_storage_602: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_602: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_602) as *mut i32;
    let mut __slate_storage_1503: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1503: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1503) as *mut i32;
    let mut __slate_storage_1502: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1502: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1502) as *mut i32;
    let mut __slate_storage_601: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_601: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_601) as *mut i32;
    let mut __slate_storage_600: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_600: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_600) as *mut i32;
    let mut __slate_storage_599: std::mem::MaybeUninit<*mut Btree> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_599: *mut *mut Btree =
        std::ptr::addr_of_mut!(__slate_storage_599) as *mut *mut Btree;
    let mut __slate_storage_1501: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1501: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1501) as *mut i32;
    let mut __slate_storage_598: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_598: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_598) as *mut i32;
    let mut __slate_storage_597: std::mem::MaybeUninit<*mut Btree> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_597: *mut *mut Btree =
        std::ptr::addr_of_mut!(__slate_storage_597) as *mut *mut Btree;
    let mut __slate_storage_1500: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1500: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1500) as *mut i32;
    let mut __slate_storage_1499: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1499: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1499) as *mut i32;
    let mut __slate_storage_1498: std::mem::MaybeUninit<*mut Parse> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1498: *mut *mut Parse =
        std::ptr::addr_of_mut!(__slate_storage_1498) as *mut *mut Parse;
    let mut __slate_storage_596: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_596: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_596) as *mut i32;
    let mut __slate_storage_595: std::mem::MaybeUninit<*mut VdbeOp> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_595: *mut *mut VdbeOp =
        std::ptr::addr_of_mut!(__slate_storage_595) as *mut *mut VdbeOp;
    let mut __slate_storage_1497: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1497: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1497) as *mut i32;
    let mut __slate_storage_1496: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1496: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1496) as *mut i32;
    let mut __slate_storage_1495: std::mem::MaybeUninit<*mut Parse> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1495: *mut *mut Parse =
        std::ptr::addr_of_mut!(__slate_storage_1495) as *mut *mut Parse;
    // If the temp database has been explicitly named as part of the
    // pragma, make sure it is open.
    let mut __slate_storage_1494: std::mem::MaybeUninit<bool> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1494: *mut bool = std::ptr::addr_of_mut!(__slate_storage_1494) as *mut bool; // The pragma
    let mut __slate_storage_592: std::mem::MaybeUninit<*const PragmaName> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_592: *mut *const PragmaName =
        std::ptr::addr_of_mut!(__slate_storage_592) as *mut *const PragmaName; // Prepared statement
    let mut __slate_storage_591: std::mem::MaybeUninit<*mut Vdbe> = std::mem::MaybeUninit::uninit();
    let __slate_slot_591: *mut *mut Vdbe =
        std::ptr::addr_of_mut!(__slate_storage_591) as *mut *mut Vdbe; // The specific database being pragmaed
    let mut __slate_storage_590: std::mem::MaybeUninit<*mut Db> = std::mem::MaybeUninit::uninit();
    let __slate_slot_590: *mut *mut Db =
        std::ptr::addr_of_mut!(__slate_storage_590) as *mut *mut Db; // The database connection
    let mut __slate_storage_589: std::mem::MaybeUninit<*mut sqlite3> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_589: *mut *mut sqlite3 =
        std::ptr::addr_of_mut!(__slate_storage_589) as *mut *mut sqlite3; // return value form SQLITE_FCNTL_PRAGMA
    let mut __slate_storage_588: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_588: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_588) as *mut i32; // Database index for <database>
    let mut __slate_storage_587: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_587: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_587) as *mut i32; // Argument to SQLITE_FCNTL_PRAGMA
    let mut __slate_storage_586: std::mem::MaybeUninit<__SlateAlign16<[*mut i8; 4]>> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_586: *mut [*mut i8; 4] =
        std::ptr::addr_of_mut!(__slate_storage_586) as *mut [*mut i8; 4]; // Pointer to <id> token
    let mut __slate_storage_585: std::mem::MaybeUninit<*mut Token> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_585: *mut *mut Token =
        std::ptr::addr_of_mut!(__slate_storage_585) as *mut *mut Token; // The database name
    let mut __slate_storage_584: std::mem::MaybeUninit<*const i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_584: *mut *const i8 =
        std::ptr::addr_of_mut!(__slate_storage_584) as *mut *const i8; // Nul-terminated UTF-8 string <value>, or NULL
    let mut __slate_storage_583: std::mem::MaybeUninit<*mut i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_583: *mut *mut i8 =
        std::ptr::addr_of_mut!(__slate_storage_583) as *mut *mut i8; // Nul-terminated UTF-8 string <id>
    let mut __slate_storage_582: std::mem::MaybeUninit<*mut i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_582: *mut *mut i8 =
        std::ptr::addr_of_mut!(__slate_storage_582) as *mut *mut i8;
    unsafe {
        std::ptr::write(__slate_slot_582, std::ptr::null_mut::<i8>());
        std::ptr::write(__slate_slot_583, std::ptr::null_mut::<i8>());
        std::ptr::write(__slate_slot_584, std::ptr::null::<i8>());
        std::ptr::write(__slate_slot_589, unsafe { (*pParse).db });
        std::ptr::write(__slate_slot_591, unsafe { sqlite3GetVdbe(pParse) });
        if *__slate_slot_591 == std::ptr::null_mut::<Vdbe>() {
            return;
        } else {
            unsafe { sqlite3VdbeRunOnlyOnce(*__slate_slot_591) };
            unsafe {
                (*pParse).nMem = 2 as i32;
            }
            // Interpret the [schema.] part of the pragma statement. iDb is the
            // index of the database this pragma is being applied to in db.aDb[].
            *__slate_slot_587 = unsafe {
                sqlite3TwoPartName(
                    pParse,
                    pId1,
                    pId2,
                    std::ptr::addr_of_mut!(*__slate_slot_585),
                )
            };
            if *__slate_slot_587 < (0 as i32) {
                return;
            } else {
                *__slate_slot_590 = unsafe {
                    unsafe { (*(*__slate_slot_589)).aDb }.offset(*__slate_slot_587 as isize)
                };
                if *__slate_slot_587 == (1 as i32) {
                    *__slate_slot_1494 = (unsafe { sqlite3OpenTempDatabase(pParse) }) != (0 as i32);
                } else {
                    *__slate_slot_1494 = false as bool;
                }
                if *__slate_slot_1494 {
                    return;
                } else {
                    *__slate_slot_582 = unsafe {
                        sqlite3NameFromToken(*__slate_slot_589, *__slate_slot_585 as *const Token)
                    };
                    if !(*__slate_slot_582 != std::ptr::null_mut::<i8>()) {
                        return;
                    } else {
                        if minusFlag != (0 as i32) {
                            *__slate_slot_583 = unsafe {
                                sqlite3MPrintf(
                                    *__slate_slot_589,
                                    (b"-%T\0".as_ptr() as *mut i8) as *const i8,
                                    pValue,
                                )
                            };
                        } else {
                            *__slate_slot_583 = unsafe {
                                sqlite3NameFromToken(*__slate_slot_589, pValue as *const Token)
                            };
                        }
                        '__join_0: {
                            0 as i32;
                            *__slate_slot_584 = (if (unsafe { (*pId2).n }) > ((0 as i32) as u32) {
                                unsafe { (*(*__slate_slot_590)).zDbSName }
                            } else {
                                std::ptr::null_mut::<i8>()
                            }) as *const i8;
                            if (unsafe {
                                sqlite3AuthCheck(
                                    pParse,
                                    19 as i32,
                                    *__slate_slot_582 as *const i8,
                                    *__slate_slot_583 as *const i8,
                                    *__slate_slot_584,
                                )
                            }) != (0 as i32)
                            {
                            } else {
                                // Send an SQLITE_FCNTL_PRAGMA file-control to the underlying VFS
                                // connection.  If it returns SQLITE_OK, then assume that the VFS
                                // handled the pragma and generate a no-op prepared statement.
                                //
                                // IMPLEMENTATION-OF: R-12238-55120 Whenever a PRAGMA statement is parsed,
                                // an SQLITE_FCNTL_PRAGMA file control is sent to the open sqlite3_file
                                // object corresponding to the database file to which the pragma
                                // statement refers.
                                //
                                // IMPLEMENTATION-OF: R-29875-31678 The argument to the SQLITE_FCNTL_PRAGMA
                                // file control is an array of pointers to strings (char**) in which the
                                // second element of the array is the name of the pragma and the third
                                // element is the argument to the pragma or NULL if the pragma has no
                                // argument.
                                unsafe {
                                    *unsafe {
                                        ((*__slate_slot_586).as_mut_ptr() as *mut *mut i8)
                                            .offset((0 as i32) as isize)
                                    } = std::ptr::null_mut::<i8>();
                                }
                                unsafe {
                                    *unsafe {
                                        ((*__slate_slot_586).as_mut_ptr() as *mut *mut i8)
                                            .offset((1 as i32) as isize)
                                    } = *__slate_slot_582;
                                }
                                unsafe {
                                    *unsafe {
                                        ((*__slate_slot_586).as_mut_ptr() as *mut *mut i8)
                                            .offset((2 as i32) as isize)
                                    } = *__slate_slot_583;
                                }
                                unsafe {
                                    *unsafe {
                                        ((*__slate_slot_586).as_mut_ptr() as *mut *mut i8)
                                            .offset((3 as i32) as isize)
                                    } = std::ptr::null_mut::<i8>();
                                }
                                unsafe {
                                    (*(*__slate_slot_589)).busyHandler.nBusy = 0 as i32;
                                }
                                *__slate_slot_588 = unsafe {
                                    sqlite3_file_control(
                                        *__slate_slot_589,
                                        *__slate_slot_584,
                                        14 as i32,
                                        ((*__slate_slot_586).as_mut_ptr() as *mut *mut i8)
                                            as *mut (),
                                    )
                                };
                                if *__slate_slot_588 == (0 as i32) {
                                    unsafe { sqlite3VdbeSetNumCols(*__slate_slot_591, 1 as i32) };
                                    unsafe {
                                        sqlite3VdbeSetColName(
                                            *__slate_slot_591,
                                            0 as i32,
                                            0 as i32,
                                            (unsafe {
                                                *unsafe {
                                                    ((*__slate_slot_586).as_mut_ptr()
                                                        as *mut *mut i8)
                                                        .offset((0 as i32) as isize)
                                                }
                                            })
                                                as *const i8,
                                            unsafe {
                                                std::mem::transmute::<
                                                    usize,
                                                    Option<unsafe extern "C-unwind" fn(*mut ())>,
                                                >(
                                                    -(1 as i32) as usize
                                                )
                                            },
                                        )
                                    };
                                    returnSingleText(
                                        *__slate_slot_591,
                                        (unsafe {
                                            *unsafe {
                                                ((*__slate_slot_586).as_mut_ptr() as *mut *mut i8)
                                                    .offset((0 as i32) as isize)
                                            }
                                        }) as *const i8,
                                    );
                                    unsafe {
                                        sqlite3_free(
                                            (unsafe {
                                                *unsafe {
                                                    ((*__slate_slot_586).as_mut_ptr()
                                                        as *mut *mut i8)
                                                        .offset((0 as i32) as isize)
                                                }
                                            })
                                                as *mut (),
                                        )
                                    };
                                } else {
                                    if *__slate_slot_588 != (12 as i32) {
                                        if (unsafe {
                                            *unsafe {
                                                ((*__slate_slot_586).as_mut_ptr() as *mut *mut i8)
                                                    .offset((0 as i32) as isize)
                                            }
                                        }) != std::ptr::null_mut::<i8>()
                                        {
                                            unsafe {
                                                sqlite3ErrorMsg(
                                                    pParse,
                                                    (b"%s\0".as_ptr() as *mut i8) as *const i8,
                                                    unsafe {
                                                        *unsafe {
                                                            ((*__slate_slot_586).as_mut_ptr()
                                                                as *mut *mut i8)
                                                                .offset((0 as i32) as isize)
                                                        }
                                                    },
                                                )
                                            };
                                            unsafe {
                                                sqlite3_free(
                                                    (unsafe {
                                                        *unsafe {
                                                            ((*__slate_slot_586).as_mut_ptr()
                                                                as *mut *mut i8)
                                                                .offset((0 as i32) as isize)
                                                        }
                                                    })
                                                        as *mut (),
                                                )
                                            };
                                        }
                                        std::ptr::write(__slate_slot_1495, pParse);
                                        std::ptr::write(__slate_slot_1496, unsafe {
                                            (*(*__slate_slot_1495)).nErr
                                        });
                                        std::ptr::write(
                                            __slate_slot_1497,
                                            *__slate_slot_1496 + (1 as i32),
                                        );
                                        unsafe {
                                            (*(*__slate_slot_1495)).nErr = *__slate_slot_1497;
                                        }
                                        unsafe {
                                            (*pParse).rc = *__slate_slot_588;
                                        }
                                    } else {
                                        // Locate the pragma in the lookup table
                                        *__slate_slot_592 =
                                            pragmaLocate(*__slate_slot_582 as *const i8);
                                        if *__slate_slot_592 == std::ptr::null::<PragmaName>() {
                                            // IMP: R-43042-22504 No error messages are generated if an
                                            // unknown pragma is issued.
                                        } else {
                                            // Make sure the database schema is loaded if the pragma requires that
                                            if (((unsafe { (*(*__slate_slot_592)).mPragFlg })
                                                as u32)
                                                as i32)
                                                & (1 as i32)
                                                != (0 as i32)
                                            {
                                                if (unsafe { sqlite3ReadSchema(pParse) })
                                                    != (0 as i32)
                                                {
                                                    break '__join_0;
                                                }
                                            }
                                            // Register the result column names for pragmas that return results
                                            if (((unsafe { (*(*__slate_slot_592)).mPragFlg })
                                                as u32)
                                                as i32)
                                                & (2 as i32)
                                                == (0 as i32)
                                                && ((((unsafe { (*(*__slate_slot_592)).mPragFlg })
                                                    as u32)
                                                    as i32)
                                                    & (4 as i32)
                                                    == (0 as i32)
                                                    || *__slate_slot_583
                                                        == std::ptr::null_mut::<i8>())
                                            {
                                                setPragmaResultColumnNames(
                                                    *__slate_slot_591,
                                                    *__slate_slot_592,
                                                );
                                            }
                                            '__join_2: {
                                                // Jump to the appropriate pragma handler
                                                let __t0: i32 =
                                                    ((unsafe { (*(*__slate_slot_592)).ePragTyp })
                                                        as u32)
                                                        as i32;
                                                if __t0 == (13 as i32) {
                                                    // 0
                                                    // 1
                                                    // 6
                                                    unsafe {
                                                        sqlite3VdbeUsesBtree(
                                                            *__slate_slot_591,
                                                            *__slate_slot_587,
                                                        )
                                                    };
                                                    if !(*__slate_slot_583
                                                        != std::ptr::null_mut::<i8>())
                                                    {
                                                        std::ptr::write(__slate_slot_1498, pParse);
                                                        std::ptr::write(
                                                            __slate_slot_1499,
                                                            unsafe { (*(*__slate_slot_1498)).nMem },
                                                        );
                                                        std::ptr::write(
                                                            __slate_slot_1500,
                                                            *__slate_slot_1499 + (2 as i32),
                                                        );
                                                        unsafe {
                                                            (*(*__slate_slot_1498)).nMem =
                                                                *__slate_slot_1500;
                                                        }
                                                        {}
                                                        *__slate_slot_595 = unsafe {
                                                            sqlite3VdbeAddOpList(
                                                                *__slate_slot_591,
                                                                (((36 as u64) / (4 as u64)) as u32)
                                                                    as i32,
                                                                unsafe {
                                                                    std::ptr::addr_of!(
                                                                        getCacheSize.0
                                                                    )
                                                                        as *const VdbeOpList
                                                                },
                                                                unsafe { iLn_593 },
                                                            )
                                                        };
                                                        if (0 as i32) != (0 as i32) {
                                                        } else {
                                                            unsafe {
                                                                (*unsafe {
                                                                    (*__slate_slot_595)
                                                                        .offset((0 as i32) as isize)
                                                                })
                                                                .p1 = *__slate_slot_587;
                                                            }
                                                            unsafe {
                                                                (*unsafe {
                                                                    (*__slate_slot_595)
                                                                        .offset((1 as i32) as isize)
                                                                })
                                                                .p1 = *__slate_slot_587;
                                                            }
                                                            unsafe {
                                                                (*unsafe {
                                                                    (*__slate_slot_595)
                                                                        .offset((6 as i32) as isize)
                                                                })
                                                                .p1 = -(2000 as i32);
                                                            }
                                                        }
                                                    } else {
                                                        std::ptr::write(__slate_slot_596, unsafe {
                                                            sqlite3AbsInt32(unsafe {
                                                                sqlite3Atoi(
                                                                    *__slate_slot_583 as *const i8,
                                                                )
                                                            })
                                                        });
                                                        unsafe {
                                                            sqlite3BeginWriteOperation(
                                                                pParse,
                                                                0 as i32,
                                                                *__slate_slot_587,
                                                            )
                                                        };
                                                        unsafe {
                                                            sqlite3VdbeAddOp3(
                                                                *__slate_slot_591,
                                                                102 as i32,
                                                                *__slate_slot_587,
                                                                3 as i32,
                                                                *__slate_slot_596,
                                                            )
                                                        };
                                                        0 as i32;
                                                        unsafe {
                                                            (*unsafe {
                                                                (*(*__slate_slot_590)).pSchema
                                                            })
                                                            .cache_size = *__slate_slot_596;
                                                        }
                                                        unsafe {
                                                            sqlite3BtreeSetCacheSize(
                                                                unsafe {
                                                                    (*(*__slate_slot_590)).pBt
                                                                },
                                                                unsafe {
                                                                    (*unsafe {
                                                                        (*(*__slate_slot_590))
                                                                            .pSchema
                                                                    })
                                                                    .cache_size
                                                                },
                                                            )
                                                        };
                                                    }
                                                } else {
                                                    if __t0 == (31 as i32) {
                                                        std::ptr::write(__slate_slot_597, unsafe {
                                                            (*(*__slate_slot_590)).pBt
                                                        });
                                                        0 as i32;
                                                        if !(*__slate_slot_583
                                                            != std::ptr::null_mut::<i8>())
                                                        {
                                                            if *__slate_slot_597
                                                                != std::ptr::null_mut::<Btree>()
                                                            {
                                                                *__slate_slot_1501 = unsafe {
                                                                    sqlite3BtreeGetPageSize(
                                                                        *__slate_slot_597,
                                                                    )
                                                                };
                                                            } else {
                                                                *__slate_slot_1501 = 0 as i32;
                                                            }
                                                            *__slate_slot_598 = *__slate_slot_1501;
                                                            returnSingleInt(
                                                                *__slate_slot_591,
                                                                *__slate_slot_598 as i64,
                                                            );
                                                        } else {
                                                            // Malloc may fail when setting the page-size, as there is an internal
                                                            // buffer that the pager module resizes using sqlite3_realloc().
                                                            unsafe {
                                                                (*(*__slate_slot_589))
                                                                    .nextPagesize = unsafe {
                                                                    sqlite3Atoi(
                                                                        *__slate_slot_583
                                                                            as *const i8,
                                                                    )
                                                                };
                                                            }
                                                            if (7 as i32)
                                                                == unsafe {
                                                                    sqlite3BtreeSetPageSize(
                                                                        *__slate_slot_597,
                                                                        unsafe {
                                                                            (*(*__slate_slot_589))
                                                                                .nextPagesize
                                                                        },
                                                                        0 as i32,
                                                                        0 as i32,
                                                                    )
                                                                }
                                                            {
                                                                unsafe {
                                                                    sqlite3OomFault(
                                                                        *__slate_slot_589,
                                                                    )
                                                                };
                                                            }
                                                        }
                                                    } else {
                                                        if __t0 == (33 as i32) {
                                                            std::ptr::write(
                                                                __slate_slot_599,
                                                                unsafe {
                                                                    (*(*__slate_slot_590)).pBt
                                                                },
                                                            );
                                                            std::ptr::write(
                                                                __slate_slot_600,
                                                                -(1 as i32),
                                                            );
                                                            0 as i32;
                                                            if *__slate_slot_583
                                                                != std::ptr::null_mut::<i8>()
                                                            {
                                                                if (unsafe {
                                                                    sqlite3_stricmp(
                                                                        *__slate_slot_583
                                                                            as *const i8,
                                                                        (b"fast\0".as_ptr()
                                                                            as *mut i8)
                                                                            as *const i8,
                                                                    )
                                                                }) == (0 as i32)
                                                                {
                                                                    *__slate_slot_600 = 2 as i32;
                                                                } else {
                                                                    *__slate_slot_600 =
                                                                        (sqlite3GetBoolean(
                                                                            *__slate_slot_583
                                                                                as *const i8,
                                                                            ((0 as i32) as i8)
                                                                                as u8,
                                                                        )
                                                                            as u32)
                                                                            as i32;
                                                                }
                                                            }
                                                            '__join_535: {
                                                                if (unsafe { (*pId2).n })
                                                                    == ((0 as i32) as u32)
                                                                    && *__slate_slot_600
                                                                        >= (0 as i32)
                                                                {
                                                                    *__slate_slot_601 = 0 as i32;
                                                                    loop {
                                                                        if *__slate_slot_601
                                                                            < unsafe {
                                                                                (*(*__slate_slot_589)).nDb
                                                                            }
                                                                        {
                                                                            unsafe {
                                                                                sqlite3BtreeSecureDelete(unsafe { (*unsafe { unsafe { (*(*__slate_slot_589)).aDb }.offset(*__slate_slot_601 as isize) }).pBt }, *__slate_slot_600)
                                                                            };
                                                                            std::ptr::write(
                                                                                __slate_slot_1502,
                                                                                *__slate_slot_601,
                                                                            );
                                                                            std::ptr::write(
                                                                                __slate_slot_1503,
                                                                                *__slate_slot_1502
                                                                                    + (1 as i32),
                                                                            );
                                                                            *__slate_slot_601 =
                                                                                *__slate_slot_1503;
                                                                        } else {
                                                                            break '__join_535;
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                            *__slate_slot_600 = unsafe {
                                                                sqlite3BtreeSecureDelete(
                                                                    *__slate_slot_599,
                                                                    *__slate_slot_600,
                                                                )
                                                            };
                                                            returnSingleInt(
                                                                *__slate_slot_591,
                                                                *__slate_slot_600 as i64,
                                                            );
                                                        } else {
                                                            if __t0 == (27 as i32) {
                                                                std::ptr::write(
                                                                    __slate_slot_603,
                                                                    (0 as i32) as i64,
                                                                );
                                                                unsafe {
                                                                    sqlite3CodeVerifySchema(
                                                                        pParse,
                                                                        *__slate_slot_587,
                                                                    )
                                                                };
                                                                std::ptr::write(
                                                                    __slate_slot_1504,
                                                                    pParse,
                                                                );
                                                                std::ptr::write(
                                                                    __slate_slot_1505,
                                                                    unsafe {
                                                                        (*(*__slate_slot_1504)).nMem
                                                                    },
                                                                );
                                                                std::ptr::write(
                                                                    __slate_slot_1506,
                                                                    *__slate_slot_1505 + (1 as i32),
                                                                );
                                                                unsafe {
                                                                    (*(*__slate_slot_1504)).nMem =
                                                                        *__slate_slot_1506;
                                                                }
                                                                *__slate_slot_602 =
                                                                    *__slate_slot_1506;
                                                                if (((unsafe {
                                                                    *unsafe {
                                                                        unsafe { std::ptr::addr_of!(sqlite3UpperToLower) as *const u8 }.offset(((((unsafe { *unsafe { (*__slate_slot_582).offset((0 as i32) as isize) } }) as u8) as u32) as i32) as isize)
                                                                    }
                                                                })
                                                                    as u32)
                                                                    as i32)
                                                                    == (112 as i32)
                                                                {
                                                                    unsafe {
                                                                        sqlite3VdbeAddOp2(
                                                                            *__slate_slot_591,
                                                                            180 as i32,
                                                                            *__slate_slot_587,
                                                                            *__slate_slot_602,
                                                                        )
                                                                    };
                                                                } else {
                                                                    '__join_530: {
                                                                        if *__slate_slot_583
                                                                            != std::ptr::null_mut::<
                                                                                i8,
                                                                            >(
                                                                            )
                                                                        {
                                                                            *__slate_slot_1507 = (unsafe {
                                                                                sqlite3DecOrHexToI64(*__slate_slot_583 as *const i8, std::ptr::addr_of_mut!(*__slate_slot_603))
                                                                            })
                                                                                == (0 as i32);
                                                                        } else {
                                                                            *__slate_slot_1507 =
                                                                                false as bool;
                                                                        }
                                                                    }
                                                                    if *__slate_slot_1507 {
                                                                        if *__slate_slot_603
                                                                            < ((0 as i32) as i64)
                                                                        {
                                                                            *__slate_slot_603 =
                                                                                (0 as i32) as i64;
                                                                        } else {
                                                                            if *__slate_slot_603
                                                                                > (((4294967294
                                                                                    as u32)
                                                                                    as u64)
                                                                                    as i64)
                                                                            {
                                                                                *__slate_slot_603 =
                                                                                    ((4294967294
                                                                                        as u32)
                                                                                        as u64)
                                                                                        as i64;
                                                                            }
                                                                        }
                                                                    } else {
                                                                        *__slate_slot_603 =
                                                                            (0 as i32) as i64;
                                                                    }
                                                                    unsafe {
                                                                        sqlite3VdbeAddOp3(
                                                                            *__slate_slot_591,
                                                                            181 as i32,
                                                                            *__slate_slot_587,
                                                                            *__slate_slot_602,
                                                                            *__slate_slot_603
                                                                                as i32,
                                                                        )
                                                                    };
                                                                }
                                                                unsafe {
                                                                    sqlite3VdbeAddOp2(
                                                                        *__slate_slot_591,
                                                                        86 as i32,
                                                                        *__slate_slot_602,
                                                                        1 as i32,
                                                                    )
                                                                };
                                                            } else {
                                                                if __t0 == (26 as i32) {
                                                                    std::ptr::write(
                                                                        __slate_slot_604,
                                                                        (b"normal\0".as_ptr()
                                                                            as *mut i8)
                                                                            as *const i8,
                                                                    );
                                                                    std::ptr::write(
                                                                        __slate_slot_605,
                                                                        getLockingMode(
                                                                            *__slate_slot_583
                                                                                as *const i8,
                                                                        ),
                                                                    );
                                                                    if (unsafe { (*pId2).n })
                                                                        == ((0 as i32) as u32)
                                                                        && *__slate_slot_605
                                                                            == -(1 as i32)
                                                                    {
                                                                        // Simple "PRAGMA locking_mode;" statement. This is a query for
                                                                        // the current default locking mode (which may be different to
                                                                        // the locking-mode of the main database).
                                                                        *__slate_slot_605 = ((unsafe {
                                                                            (*(*__slate_slot_589))
                                                                                .dfltLockMode
                                                                        })
                                                                            as u32)
                                                                            as i32;
                                                                    } else {
                                                                        if (unsafe { (*pId2).n })
                                                                            == ((0 as i32) as u32)
                                                                        {
                                                                            0 as i32;
                                                                            *__slate_slot_607 =
                                                                                2 as i32;
                                                                            loop {
                                                                                if *__slate_slot_607
                                                                                    < unsafe {
                                                                                        (*(*__slate_slot_589)).nDb
                                                                                    }
                                                                                {
                                                                                    *__slate_slot_606 = unsafe { sqlite3BtreePager(unsafe { (*unsafe { unsafe { (*(*__slate_slot_589)).aDb }.offset(*__slate_slot_607 as isize) }).pBt }) };
                                                                                    unsafe {
                                                                                        sqlite3PagerLockingMode(*__slate_slot_606, *__slate_slot_605)
                                                                                    };
                                                                                    std::ptr::write(__slate_slot_1508, *__slate_slot_607);
                                                                                    std::ptr::write(__slate_slot_1509, *__slate_slot_1508 + (1 as i32));
                                                                                    *__slate_slot_607 = *__slate_slot_1509;
                                                                                } else {
                                                                                    break;
                                                                                }
                                                                            }
                                                                            unsafe {
                                                                                (*(*__slate_slot_589)).dfltLockMode = (*__slate_slot_605 as i8) as u8;
                                                                            }
                                                                        }
                                                                        *__slate_slot_606 = unsafe {
                                                                            sqlite3BtreePager(
                                                                                unsafe {
                                                                                    (*(*__slate_slot_590)).pBt
                                                                                },
                                                                            )
                                                                        };
                                                                        *__slate_slot_605 = unsafe {
                                                                            sqlite3PagerLockingMode(
                                                                                *__slate_slot_606,
                                                                                *__slate_slot_605,
                                                                            )
                                                                        };
                                                                    }
                                                                    0 as i32;
                                                                    if *__slate_slot_605
                                                                        == (1 as i32)
                                                                    {
                                                                        *__slate_slot_604 =
                                                                            (b"exclusive\0".as_ptr()
                                                                                as *mut i8)
                                                                                as *const i8;
                                                                    }
                                                                    returnSingleText(
                                                                        *__slate_slot_591,
                                                                        *__slate_slot_604,
                                                                    );
                                                                } else {
                                                                    if __t0 == (23 as i32) {
                                                                        if *__slate_slot_583
                                                                            == std::ptr::null_mut::<
                                                                                i8,
                                                                            >(
                                                                            )
                                                                        {
                                                                            // If there is no "=MODE" part of the pragma, do a query for the
                                                                            // current mode
                                                                            *__slate_slot_608 =
                                                                                -(1 as i32);
                                                                        } else {
                                                                            std::ptr::write(
                                                                                __slate_slot_611,
                                                                                unsafe {
                                                                                    sqlite3Strlen30(*__slate_slot_583 as *const i8)
                                                                                },
                                                                            );
                                                                            *__slate_slot_608 =
                                                                                0 as i32;
                                                                            loop {
                                                                                std::ptr::write(__slate_slot_1510, sqlite3JournalModename(*__slate_slot_608));
                                                                                *__slate_slot_610 = *__slate_slot_1510;
                                                                                if *__slate_slot_1510 != std::ptr::null::<i8>() {
if (unsafe { sqlite3_strnicmp(*__slate_slot_583 as *const i8, *__slate_slot_610, *__slate_slot_611) }) == (0 as i32) {
break;
} else {
std::ptr::write(__slate_slot_1511, *__slate_slot_608);
std::ptr::write(__slate_slot_1512, *__slate_slot_1511 + (1 as i32));
*__slate_slot_608 = *__slate_slot_1512;
}
} else {
break;
}
                                                                            }
                                                                            if !(*__slate_slot_610
                                                                                != std::ptr::null::<
                                                                                    i8,
                                                                                >(
                                                                                ))
                                                                            {
                                                                                // If the "=MODE" part does not match any known journal mode,
                                                                                // then do a query
                                                                                *__slate_slot_608 =
                                                                                    -(1 as i32);
                                                                            }
                                                                            if *__slate_slot_608
                                                                                == (2 as i32)
                                                                                && (unsafe {
                                                                                    (*(*__slate_slot_589)).flags
                                                                                }) & (((268435456
                                                                                    as i32)
                                                                                    as i64)
                                                                                    as u64)
                                                                                    != (((0 as i32)
                                                                                        as i64)
                                                                                        as u64)
                                                                            {
                                                                                // Do not allow journal-mode "OFF" in defensive since the database
                                                                                // can become corrupted using ordinary SQL when the journal is off
                                                                                *__slate_slot_608 =
                                                                                    -(1 as i32);
                                                                            }
                                                                        }
                                                                        if *__slate_slot_608
                                                                            == -(1 as i32)
                                                                            && (unsafe {
                                                                                (*pId2).n
                                                                            }) == ((0 as i32)
                                                                                as u32)
                                                                        {
                                                                            // Convert "PRAGMA journal_mode" into "PRAGMA main.journal_mode"
                                                                            *__slate_slot_587 =
                                                                                0 as i32;
                                                                            unsafe {
                                                                                (*pId2).n = (1
                                                                                    as i32)
                                                                                    as u32;
                                                                            }
                                                                        }
                                                                        *__slate_slot_609 = (unsafe {
                                                                            (*(*__slate_slot_589))
                                                                                .nDb
                                                                        }) - (1
                                                                            as i32);
                                                                        loop {
                                                                            if *__slate_slot_609
                                                                                >= (0 as i32)
                                                                            {
                                                                                if (unsafe { (*unsafe { unsafe { (*(*__slate_slot_589)).aDb }.offset(*__slate_slot_609 as isize) }).pBt }) != std::ptr::null_mut::<Btree>() && (*__slate_slot_609 == *__slate_slot_587 || (unsafe { (*pId2).n }) == ((0 as i32) as u32)) {
unsafe { sqlite3VdbeUsesBtree(*__slate_slot_591, *__slate_slot_609) };
unsafe { sqlite3VdbeAddOp3(*__slate_slot_591, 4 as i32, *__slate_slot_609, 1 as i32, *__slate_slot_608) };
}
                                                                                std::ptr::write(__slate_slot_1513, *__slate_slot_609);
                                                                                std::ptr::write(__slate_slot_1514, *__slate_slot_1513 - (1 as i32));
                                                                                *__slate_slot_609 = *__slate_slot_1514;
                                                                            } else {
                                                                                break;
                                                                            }
                                                                        }
                                                                        unsafe {
                                                                            sqlite3VdbeAddOp2(
                                                                                *__slate_slot_591,
                                                                                86 as i32,
                                                                                1 as i32,
                                                                                1 as i32,
                                                                            )
                                                                        };
                                                                    } else {
                                                                        if __t0 == (24 as i32) {
                                                                            std::ptr::write(
                                                                                __slate_slot_612,
                                                                                unsafe {
                                                                                    sqlite3BtreePager(unsafe { (*(*__slate_slot_590)).pBt })
                                                                                },
                                                                            );
                                                                            std::ptr::write(
                                                                                __slate_slot_613,
                                                                                -(2 as i32) as i64,
                                                                            );
                                                                            if *__slate_slot_583 != std::ptr::null_mut::<i8>() {
unsafe { sqlite3DecOrHexToI64(*__slate_slot_583 as *const i8, std::ptr::addr_of_mut!(*__slate_slot_613)) };
if *__slate_slot_613 < (-(1 as i32) as i64) {
*__slate_slot_613 = -(1 as i32) as i64;
}
}
                                                                            *__slate_slot_613 = unsafe {
                                                                                sqlite3PagerJournalSizeLimit(*__slate_slot_612, *__slate_slot_613)
                                                                            };
                                                                            returnSingleInt(
                                                                                *__slate_slot_591,
                                                                                *__slate_slot_613,
                                                                            );
                                                                        } else {
                                                                            if __t0 == (3 as i32) {
                                                                                std::ptr::write(__slate_slot_614, unsafe { (*(*__slate_slot_590)).pBt });
                                                                                0 as i32;
                                                                                if !(*__slate_slot_583 != std::ptr::null_mut::<i8>()) {
returnSingleInt(*__slate_slot_591, (unsafe { sqlite3BtreeGetAutoVacuum(*__slate_slot_614) }) as i64);
} else {
std::ptr::write(__slate_slot_615, getAutoVacuum(*__slate_slot_583 as *const i8));
0 as i32;
unsafe {
(*(*__slate_slot_589)).nextAutovac = ((*__slate_slot_615 as i8) as u8) as i8;
}
// Call SetAutoVacuum() to set initialize the internal auto and
// incr-vacuum flags. This is required in case this connection
// creates the database file. It is important that it is created
// as an auto-vacuum capable db.
*__slate_slot_588 = unsafe { sqlite3BtreeSetAutoVacuum(*__slate_slot_614, *__slate_slot_615) };
if *__slate_slot_588 == (0 as i32) && (*__slate_slot_615 == (1 as i32) || *__slate_slot_615 == (2 as i32)) {
// When setting the auto_vacuum mode to either "full" or
// "incremental", write the value of meta[6] in the database
// file. Before writing to meta[6], check that meta[3] indicates
// that this really is an auto-vacuum capable database.
// 0
// 2
// 3
// 4
std::ptr::write(__slate_slot_619, unsafe { sqlite3VdbeCurrentAddr(*__slate_slot_591) });
{
}
*__slate_slot_618 = unsafe { sqlite3VdbeAddOpList(*__slate_slot_591, (((20 as u64) / (4 as u64)) as u32) as i32, unsafe { std::ptr::addr_of!(setMeta6.0) as *const VdbeOpList }, unsafe { iLn_616 }) };
if (0 as i32) != (0 as i32) {
} else {
unsafe {
(*unsafe { (*__slate_slot_618).offset((0 as i32) as isize) }).p1 = *__slate_slot_587;
}
unsafe {
(*unsafe { (*__slate_slot_618).offset((1 as i32) as isize) }).p1 = *__slate_slot_587;
}
unsafe {
(*unsafe { (*__slate_slot_618).offset((2 as i32) as isize) }).p2 = *__slate_slot_619 + (4 as i32);
}
unsafe {
(*unsafe { (*__slate_slot_618).offset((4 as i32) as isize) }).p1 = *__slate_slot_587;
}
unsafe {
(*unsafe { (*__slate_slot_618).offset((4 as i32) as isize) }).p3 = *__slate_slot_615 - (1 as i32);
}
unsafe { sqlite3VdbeUsesBtree(*__slate_slot_591, *__slate_slot_587) };
}
}
}
                                                                            } else {
                                                                                if __t0
                                                                                    == (19 as i32)
                                                                                {
                                                                                    std::ptr::write(__slate_slot_620, 0 as i32);
                                                                                    if *__slate_slot_583 == std::ptr::null_mut::<i8>() {
*__slate_slot_1515 = true as bool;
} else {
*__slate_slot_1515 = !((unsafe { sqlite3GetInt32(*__slate_slot_583 as *const i8, std::ptr::addr_of_mut!(*__slate_slot_620)) }) != (0 as i32));
}
                                                                                    if *__slate_slot_1515 || *__slate_slot_620 <= (0 as i32) {
*__slate_slot_620 = 2147483647 as i32;
}
                                                                                    unsafe {
                                                                                        sqlite3BeginWriteOperation(pParse, 0 as i32, *__slate_slot_587)
                                                                                    };
                                                                                    unsafe {
                                                                                        sqlite3VdbeAddOp2(*__slate_slot_591, 73 as i32, *__slate_slot_620, 1 as i32)
                                                                                    };
                                                                                    *__slate_slot_621 = unsafe { sqlite3VdbeAddOp1(*__slate_slot_591, 64 as i32, *__slate_slot_587) };
                                                                                    {}
                                                                                    unsafe {
                                                                                        sqlite3VdbeAddOp1(*__slate_slot_591, 86 as i32, 1 as i32)
                                                                                    };
                                                                                    unsafe {
                                                                                        sqlite3VdbeAddOp2(*__slate_slot_591, 88 as i32, 1 as i32, -(1 as i32))
                                                                                    };
                                                                                    unsafe {
                                                                                        sqlite3VdbeAddOp2(*__slate_slot_591, 61 as i32, 1 as i32, *__slate_slot_621)
                                                                                    };
                                                                                    {}
                                                                                    unsafe {
                                                                                        sqlite3VdbeJumpHere(*__slate_slot_591, *__slate_slot_621)
                                                                                    };
                                                                                } else {
                                                                                    if __t0
                                                                                        == (6
                                                                                            as i32)
                                                                                    {
                                                                                        0 as i32;
                                                                                        if !(*__slate_slot_583 != std::ptr::null_mut::<i8>()) {
returnSingleInt(*__slate_slot_591, (unsafe { (*unsafe { (*(*__slate_slot_590)).pSchema }).cache_size }) as i64);
} else {
std::ptr::write(__slate_slot_622, unsafe { sqlite3Atoi(*__slate_slot_583 as *const i8) });
unsafe {
(*unsafe { (*(*__slate_slot_590)).pSchema }).cache_size = *__slate_slot_622;
}
unsafe { sqlite3BtreeSetCacheSize(unsafe { (*(*__slate_slot_590)).pBt }, unsafe { (*unsafe { (*(*__slate_slot_590)).pSchema }).cache_size }) };
}
                                                                                    } else {
                                                                                        if __t0 == (7 as i32) {
0 as i32;
if !(*__slate_slot_583 != std::ptr::null_mut::<i8>()) {
if (unsafe { (*(*__slate_slot_589)).flags }) & (((32 as i32) as i64) as u64) == (((0 as i32) as i64) as u64) {
*__slate_slot_1516 = 0 as i32;
} else {
*__slate_slot_1516 = unsafe { sqlite3BtreeSetSpillSize(unsafe { (*(*__slate_slot_590)).pBt }, 0 as i32) };
}
returnSingleInt(*__slate_slot_591, *__slate_slot_1516 as i64);
} else {
std::ptr::write(__slate_slot_623, 1 as i32);
if (unsafe { sqlite3GetInt32(*__slate_slot_583 as *const i8, std::ptr::addr_of_mut!(*__slate_slot_623)) }) != (0 as i32) {
unsafe { sqlite3BtreeSetSpillSize(unsafe { (*(*__slate_slot_590)).pBt }, *__slate_slot_623) };
}
if sqlite3GetBoolean(*__slate_slot_583 as *const i8, (*__slate_slot_623 != (0 as i32)) as u8) != (0 as u8) {
std::ptr::write(__slate_slot_1517, *__slate_slot_589);
std::ptr::write(__slate_slot_1518, unsafe { (*(*__slate_slot_1517)).flags });
std::ptr::write(__slate_slot_1519, *__slate_slot_1518 | (((32 as i32) as i64) as u64));
unsafe {
(*(*__slate_slot_1517)).flags = *__slate_slot_1519;
}
} else {
std::ptr::write(__slate_slot_1520, *__slate_slot_589);
std::ptr::write(__slate_slot_1521, unsafe { (*(*__slate_slot_1520)).flags });
std::ptr::write(__slate_slot_1522, *__slate_slot_1521 & !(((32 as i32) as i64) as u64));
unsafe {
(*(*__slate_slot_1520)).flags = *__slate_slot_1522;
}
}
setAllPagerFlags(*__slate_slot_589);
}
} else {
if __t0 == (28 as i32) {
'__join_453: {
0 as i32;
if *__slate_slot_583 != std::ptr::null_mut::<i8>() {
unsafe { sqlite3DecOrHexToI64(*__slate_slot_583 as *const i8, std::ptr::addr_of_mut!(*__slate_slot_624)) };
if *__slate_slot_624 < ((0 as i32) as i64) {
*__slate_slot_624 = unsafe { sqlite3Config.szMmap };
}
if (unsafe { (*pId2).n }) == ((0 as i32) as u32) {
unsafe {
(*(*__slate_slot_589)).szMmap = *__slate_slot_624;
}
}
*__slate_slot_625 = (unsafe { (*(*__slate_slot_589)).nDb }) - (1 as i32);
loop {
if *__slate_slot_625 >= (0 as i32) {
if (unsafe { (*unsafe { unsafe { (*(*__slate_slot_589)).aDb }.offset(*__slate_slot_625 as isize) }).pBt }) != std::ptr::null_mut::<Btree>() && (*__slate_slot_625 == *__slate_slot_587 || (unsafe { (*pId2).n }) == ((0 as i32) as u32)) {
unsafe { sqlite3BtreeSetMmapLimit(unsafe { (*unsafe { unsafe { (*(*__slate_slot_589)).aDb }.offset(*__slate_slot_625 as isize) }).pBt }, *__slate_slot_624) };
}
std::ptr::write(__slate_slot_1523, *__slate_slot_625);
std::ptr::write(__slate_slot_1524, *__slate_slot_1523 - (1 as i32));
*__slate_slot_625 = *__slate_slot_1524;
} else {
break '__join_453;
}
}
}
}
*__slate_slot_624 = -(1 as i32) as i64;
*__slate_slot_588 = unsafe { sqlite3_file_control(*__slate_slot_589, *__slate_slot_584, 18 as i32, std::ptr::addr_of_mut!(*__slate_slot_624) as *mut ()) };
if *__slate_slot_588 == (0 as i32) {
returnSingleInt(*__slate_slot_591, *__slate_slot_624);
} else {
if *__slate_slot_588 != (12 as i32) {
std::ptr::write(__slate_slot_1525, pParse);
std::ptr::write(__slate_slot_1526, unsafe { (*(*__slate_slot_1525)).nErr });
std::ptr::write(__slate_slot_1527, *__slate_slot_1526 + (1 as i32));
unsafe {
(*(*__slate_slot_1525)).nErr = *__slate_slot_1527;
}
unsafe {
(*pParse).rc = *__slate_slot_588;
}
}
}
} else {
if __t0 == (39 as i32) {
if !(*__slate_slot_583 != std::ptr::null_mut::<i8>()) {
returnSingleInt(*__slate_slot_591, ((unsafe { (*(*__slate_slot_589)).temp_store }) as u64) as i64);
} else {
changeTempStorage(pParse, *__slate_slot_583 as *const i8);
}
} else {
if __t0 == (40 as i32) {
unsafe { sqlite3_mutex_enter(unsafe { sqlite3MutexAlloc(11 as i32) }) };
if !(*__slate_slot_583 != std::ptr::null_mut::<i8>()) {
returnSingleText(*__slate_slot_591, (unsafe { sqlite3_temp_directory }) as *const i8);
} else {
if (unsafe { *unsafe { (*__slate_slot_583).offset((0 as i32) as isize) } }) != (0 as i8) {
*__slate_slot_588 = unsafe { sqlite3OsAccess(unsafe { (*(*__slate_slot_589)).pVfs }, *__slate_slot_583 as *const i8, 1 as i32, std::ptr::addr_of_mut!(*__slate_slot_626)) };
if *__slate_slot_588 != (0 as i32) || *__slate_slot_626 == (0 as i32) {
unsafe { sqlite3ErrorMsg(pParse, (b"not a writable directory\0".as_ptr() as *mut i8) as *const i8) };
unsafe { sqlite3_mutex_leave(unsafe { sqlite3MutexAlloc(11 as i32) }) };
break '__join_0;
}
}
if (1 as i32) == (0 as i32) || (1 as i32) == (1 as i32) && (((unsafe { (*(*__slate_slot_589)).temp_store }) as u32) as i32) <= (1 as i32) || (1 as i32) == (2 as i32) && (((unsafe { (*(*__slate_slot_589)).temp_store }) as u32) as i32) == (1 as i32) {
invalidateTempStorage(pParse);
}
unsafe { sqlite3_free((unsafe { sqlite3_temp_directory }) as *mut ()) };
if (unsafe { *unsafe { (*__slate_slot_583).offset((0 as i32) as isize) } }) != (0 as i8) {
unsafe {
sqlite3_temp_directory = unsafe { sqlite3_mprintf((b"%s\0".as_ptr() as *mut i8) as *const i8, *__slate_slot_583) };
}
} else {
unsafe {
sqlite3_temp_directory = std::ptr::null_mut::<i8>();
}
}
}
unsafe { sqlite3_mutex_leave(unsafe { sqlite3MutexAlloc(11 as i32) }) };
} else {
if __t0 == (36 as i32) {
if !(*__slate_slot_583 != std::ptr::null_mut::<i8>()) {
returnSingleInt(*__slate_slot_591, ((((unsafe { (*(*__slate_slot_590)).safety_level }) as u32) as i32) - (1 as i32)) as i64);
} else {
if !((unsafe { (*(*__slate_slot_589)).autoCommit }) != (0 as u8)) {
unsafe { sqlite3ErrorMsg(pParse, (b"Safety level may not be changed inside a transaction\0".as_ptr() as *mut i8) as *const i8) };
} else {
if *__slate_slot_587 != (1 as i32) {
std::ptr::write(__slate_slot_627, ((getSafetyLevel(*__slate_slot_583 as *const i8, 0 as i32, ((1 as i32) as i8) as u8) as u32) as i32) + (1 as i32) & (7 as i32));
if *__slate_slot_627 == (0 as i32) {
*__slate_slot_627 = 1 as i32;
}
unsafe {
(*(*__slate_slot_590)).safety_level = (*__slate_slot_627 as i8) as u8;
}
unsafe {
(*(*__slate_slot_590)).bSyncSet = ((1 as i32) as i8) as u8;
}
setAllPagerFlags(*__slate_slot_589);
}
}
}
} else {
if __t0 == (4 as i32) {
if *__slate_slot_583 == std::ptr::null_mut::<i8>() {
setPragmaResultColumnNames(*__slate_slot_591, *__slate_slot_592);
returnSingleInt(*__slate_slot_591, ((unsafe { (*(*__slate_slot_589)).flags }) & unsafe { (*(*__slate_slot_592)).iArg } != (((0 as i32) as i64) as u64)) as i64);
} else {
std::ptr::write(__slate_slot_628, unsafe { (*(*__slate_slot_592)).iArg });
if (((unsafe { (*(*__slate_slot_589)).autoCommit }) as u32) as i32) == (0 as i32) {
std::ptr::write(__slate_slot_1528, *__slate_slot_628);
std::ptr::write(__slate_slot_1529, *__slate_slot_1528 & ((!(16384 as i32) as i64) as u64));
*__slate_slot_628 = *__slate_slot_1529;
}
if sqlite3GetBoolean(*__slate_slot_583 as *const i8, ((0 as i32) as i8) as u8) != (0 as u8) {
if *__slate_slot_628 & (((1 as i32) as i64) as u64) == (((0 as i32) as i64) as u64) || (unsafe { (*(*__slate_slot_589)).flags }) & (((268435456 as i32) as i64) as u64) == (((0 as i32) as i64) as u64) {
std::ptr::write(__slate_slot_1530, *__slate_slot_589);
std::ptr::write(__slate_slot_1531, unsafe { (*(*__slate_slot_1530)).flags });
std::ptr::write(__slate_slot_1532, *__slate_slot_1531 | *__slate_slot_628);
unsafe {
(*(*__slate_slot_1530)).flags = *__slate_slot_1532;
}
}
} else {
std::ptr::write(__slate_slot_1533, *__slate_slot_589);
std::ptr::write(__slate_slot_1534, unsafe { (*(*__slate_slot_1533)).flags });
std::ptr::write(__slate_slot_1535, *__slate_slot_1534 & !(*__slate_slot_628));
unsafe {
(*(*__slate_slot_1533)).flags = *__slate_slot_1535;
}
if *__slate_slot_628 == (((524288 as i32) as i64) as u64) {
unsafe {
(*(*__slate_slot_589)).nDeferredImmCons = (0 as i32) as i64;
}
unsafe {
(*(*__slate_slot_589)).nDeferredCons = (0 as i32) as i64;
}
}
if *__slate_slot_628 & (((1 as i32) as i64) as u64) != (((0 as i32) as i64) as u64) {
*__slate_slot_1536 = (unsafe { sqlite3_stricmp(*__slate_slot_583 as *const i8, (b"reset\0".as_ptr() as *mut i8) as *const i8) }) == (0 as i32);
} else {
*__slate_slot_1536 = false as bool;
}
if *__slate_slot_1536 {
// IMP: R-60817-01178 If the argument is "RESET" then schema
// writing is disabled (as with "PRAGMA writable_schema=OFF") and,
// in addition, the schema is reloaded.
unsafe { sqlite3ResetAllSchemasOfConnection(*__slate_slot_589) };
}
}
// Many of the flag-pragmas modify the code generated by the SQL
// compiler (eg. count_changes). So add an opcode to expire all
// compiled SQL statements after modifying a pragma value.
unsafe { sqlite3VdbeAddOp0(*__slate_slot_591, 168 as i32) };
setAllPagerFlags(*__slate_slot_589);
}
} else {
if __t0 == (37 as i32) {
if *__slate_slot_583 != std::ptr::null_mut::<i8>() {
unsafe { sqlite3CodeVerifyNamedSchema(pParse, *__slate_slot_584) };
*__slate_slot_629 = unsafe { sqlite3LocateTable(pParse, (2 as i32) as u32, *__slate_slot_583 as *const i8, *__slate_slot_584) };
if *__slate_slot_629 != std::ptr::null_mut::<Table>() {
std::ptr::write(__slate_slot_632, 0 as i32);
std::ptr::write(__slate_slot_634, unsafe { sqlite3PrimaryKeyIndex(*__slate_slot_629) });
unsafe {
(*pParse).nMem = 7 as i32;
}
unsafe { sqlite3ViewGetColumnNames(pParse, *__slate_slot_629) };
*__slate_slot_630 = 0 as i32;
std::ptr::write(__slate_slot_1537, unsafe { (*(*__slate_slot_629)).aCol });
*__slate_slot_633 = *__slate_slot_1537;
loop {
if *__slate_slot_630 < ((unsafe { (*(*__slate_slot_629)).nCol }) as i32) {
'__join_393: {
std::ptr::write(__slate_slot_635, 0 as i32);
if (((unsafe { (*(*__slate_slot_633)).colFlags }) as u32) as i32) & (98 as i32) != (0 as i32) {
if (unsafe { (*(*__slate_slot_592)).iArg }) == (((0 as i32) as i64) as u64) {
std::ptr::write(__slate_slot_1542, *__slate_slot_632);
std::ptr::write(__slate_slot_1543, *__slate_slot_1542 + (1 as i32));
*__slate_slot_632 = *__slate_slot_1543;
break '__join_393;
} else {
if (((unsafe { (*(*__slate_slot_633)).colFlags }) as u32) as i32) & (32 as i32) != (0 as i32) {
*__slate_slot_635 = 2 as i32; // GENERATED ALWAYS AS ... VIRTUAL
} else {
if (((unsafe { (*(*__slate_slot_633)).colFlags }) as u32) as i32) & (64 as i32) != (0 as i32) {
*__slate_slot_635 = 3 as i32; // GENERATED ALWAYS AS ... STORED
} else {
0 as i32;
*__slate_slot_635 = 1 as i32; // HIDDEN
}
}
}
}
'__join_394: {
if (((unsafe { (*(*__slate_slot_633)).colFlags }) as u32) as i32) & (1 as i32) == (0 as i32) {
*__slate_slot_631 = 0 as i32;
} else {
if *__slate_slot_634 == std::ptr::null_mut::<Index>() {
*__slate_slot_631 = 1 as i32;
} else {
*__slate_slot_631 = 1 as i32;
loop {
if *__slate_slot_631 <= ((unsafe { (*(*__slate_slot_629)).nCol }) as i32) && ((unsafe { *unsafe { unsafe { (*(*__slate_slot_634)).aiColumn }.offset((*__slate_slot_631 - (1 as i32)) as isize) } }) as i32) != *__slate_slot_630 {
std::ptr::write(__slate_slot_1544, *__slate_slot_631);
std::ptr::write(__slate_slot_1545, *__slate_slot_1544 + (1 as i32));
*__slate_slot_631 = *__slate_slot_1545;
} else {
break '__join_394;
}
}
}
}
}
*__slate_slot_636 = (unsafe { sqlite3ColumnExpr(*__slate_slot_629, *__slate_slot_633) }) as *const Expr;
0 as i32;
0 as i32;
unsafe { sqlite3VdbeMultiLoad(*__slate_slot_591, 1 as i32, (if (unsafe { (*(*__slate_slot_592)).iArg }) != (0 as u64) { b"issisii\0".as_ptr() as *mut i8 } else { b"issisi\0".as_ptr() as *mut i8 }) as *const i8, *__slate_slot_630 - *__slate_slot_632, unsafe { (*(*__slate_slot_633)).zCnName }, unsafe { sqlite3ColumnType(*__slate_slot_633, b"\0".as_ptr() as *mut i8) }, if ((unsafe { (*(*__slate_slot_633)).__slate_bits_0.__get_notNull() }) as i32) != (0 as i32) { 1 as i32 } else { 0 as i32 }, if *__slate_slot_635 >= (2 as i32) || *__slate_slot_636 == std::ptr::null::<Expr>() { std::ptr::null_mut::<i8>() } else { unsafe { (*(*__slate_slot_636)).u.zToken } }, *__slate_slot_631, *__slate_slot_635) };
}
std::ptr::write(__slate_slot_1538, *__slate_slot_630);
std::ptr::write(__slate_slot_1539, *__slate_slot_1538 + (1 as i32));
*__slate_slot_630 = *__slate_slot_1539;
std::ptr::write(__slate_slot_1540, *__slate_slot_633);
std::ptr::write(__slate_slot_1541, unsafe { (*__slate_slot_1540).offset((1 as i32) as isize) });
*__slate_slot_633 = *__slate_slot_1541;
} else {
break '__join_2;
}
}
}
}
} else {
if __t0 == (38 as i32) {
unsafe {
(*pParse).nMem = 6 as i32;
}
unsafe { sqlite3CodeVerifyNamedSchema(pParse, *__slate_slot_584) };
*__slate_slot_637 = 0 as i32;
loop {
if *__slate_slot_637 < unsafe { (*(*__slate_slot_589)).nDb } {
if *__slate_slot_584 != std::ptr::null::<i8>() {
*__slate_slot_1548 = (unsafe { sqlite3_stricmp(*__slate_slot_584, (unsafe { (*unsafe { unsafe { (*(*__slate_slot_589)).aDb }.offset(*__slate_slot_637 as isize) }).zDbSName }) as *const i8) }) != (0 as i32);
} else {
*__slate_slot_1548 = false as bool;
}
'__join_358: {
if *__slate_slot_1548 {
} else {
// Ensure that the Table.nCol field is initialized for all views
// and virtual tables.  Each time we initialize a Table.nCol value
// for a table, that can potentially disrupt the hash table, so restart
// the initialization scan.
*__slate_slot_639 = unsafe { std::ptr::addr_of_mut!((*unsafe { (*unsafe { unsafe { (*(*__slate_slot_589)).aDb }.offset(*__slate_slot_637 as isize) }).pSchema }).tblHash) };
*__slate_slot_640 = (unsafe { (*(*__slate_slot_639)).count }) as i32;
'__loop_374: loop {
std::ptr::write(__slate_slot_1549, *__slate_slot_640);
std::ptr::write(__slate_slot_1550, *__slate_slot_1549 - (1 as i32));
*__slate_slot_640 = *__slate_slot_1550;
if *__slate_slot_1549 != (0 as i32) {
*__slate_slot_638 = unsafe { (*(*__slate_slot_639)).first };
'__join_383: {
loop {
if (1 as i32) != (0 as i32) {
if *__slate_slot_638 == std::ptr::null_mut::<HashElem>() {
break '__join_383;
} else {
*__slate_slot_641 = (unsafe { (*(*__slate_slot_638)).data }) as *mut Table;
if ((unsafe { (*(*__slate_slot_641)).nCol }) as i32) == (0 as i32) {
break;
} else {
*__slate_slot_638 = unsafe { (*(*__slate_slot_638)).next };
}
}
} else {
continue '__loop_374;
}
}
std::ptr::write(__slate_slot_642, unsafe { sqlite3MPrintf(*__slate_slot_589, (b"SELECT*FROM\"%w\"\0".as_ptr() as *mut i8) as *const i8, unsafe { (*(*__slate_slot_641)).zName }) });
if *__slate_slot_642 != std::ptr::null_mut::<i8>() {
std::ptr::write(__slate_slot_643, std::ptr::null_mut::<sqlite3_stmt>());
unsafe { sqlite3_prepare_v3(*__slate_slot_589, *__slate_slot_642 as *const i8, -(1 as i32), (16 as i32) as u32, std::ptr::addr_of_mut!(*__slate_slot_643), std::ptr::null_mut::<*const i8>()) };
unsafe { sqlite3_finalize(*__slate_slot_643) };
unsafe { sqlite3DbFree(*__slate_slot_589, *__slate_slot_642 as *mut ()) };
}
if (unsafe { (*(*__slate_slot_589)).mallocFailed }) != (0 as u8) {
unsafe { sqlite3ErrorMsg(unsafe { (*(*__slate_slot_589)).pParse }, (b"out of memory\0".as_ptr() as *mut i8) as *const i8) };
unsafe {
(*unsafe { (*(*__slate_slot_589)).pParse }).rc = 7 as i32;
}
}
*__slate_slot_639 = unsafe { std::ptr::addr_of_mut!((*unsafe { (*unsafe { unsafe { (*(*__slate_slot_589)).aDb }.offset(*__slate_slot_637 as isize) }).pSchema }).tblHash) };
continue '__loop_374;
}
*__slate_slot_640 = 0 as i32;
} else {
break;
}
}
*__slate_slot_638 = unsafe { (*(*__slate_slot_639)).first };
loop {
if *__slate_slot_638 != std::ptr::null_mut::<HashElem>() {
std::ptr::write(__slate_slot_644, (unsafe { (*(*__slate_slot_638)).data }) as *mut Table);
if *__slate_slot_583 != std::ptr::null_mut::<i8>() {
*__slate_slot_1551 = (unsafe { sqlite3_stricmp(*__slate_slot_583 as *const i8, (unsafe { (*(*__slate_slot_644)).zName }) as *const i8) }) != (0 as i32);
} else {
*__slate_slot_1551 = false as bool;
}
if *__slate_slot_1551 {
} else {
if (((unsafe { (*(*__slate_slot_644)).eTabType }) as u32) as i32) == (2 as i32) {
*__slate_slot_645 = (b"view\0".as_ptr() as *mut i8) as *const i8;
} else {
if (((unsafe { (*(*__slate_slot_644)).eTabType }) as u32) as i32) == (1 as i32) {
*__slate_slot_645 = (b"virtual\0".as_ptr() as *mut i8) as *const i8;
} else {
if (unsafe { (*(*__slate_slot_644)).tabFlags }) & ((4096 as i32) as u32) != (0 as u32) {
*__slate_slot_645 = (b"shadow\0".as_ptr() as *mut i8) as *const i8;
} else {
*__slate_slot_645 = (b"table\0".as_ptr() as *mut i8) as *const i8;
}
}
}
unsafe { sqlite3VdbeMultiLoad(*__slate_slot_591, 1 as i32, (b"sssiii\0".as_ptr() as *mut i8) as *const i8, unsafe { (*unsafe { unsafe { (*(*__slate_slot_589)).aDb }.offset(*__slate_slot_637 as isize) }).zDbSName }, unsafe { sqlite3PreferredTableName((unsafe { (*(*__slate_slot_644)).zName }) as *const i8) }, *__slate_slot_645, (unsafe { (*(*__slate_slot_644)).nCol }) as i32, ((unsafe { (*(*__slate_slot_644)).tabFlags }) & ((128 as i32) as u32) != ((0 as i32) as u32)) as i32, ((unsafe { (*(*__slate_slot_644)).tabFlags }) & ((65536 as i32) as u32) != ((0 as i32) as u32)) as i32) };
}
*__slate_slot_638 = unsafe { (*(*__slate_slot_638)).next };
} else {
break '__join_358;
}
}
}
}
std::ptr::write(__slate_slot_1546, *__slate_slot_637);
std::ptr::write(__slate_slot_1547, *__slate_slot_1546 + (1 as i32));
*__slate_slot_637 = *__slate_slot_1547;
} else {
break '__join_2;
}
}
} else {
if __t0 == (20 as i32) {
if *__slate_slot_583 != std::ptr::null_mut::<i8>() {
*__slate_slot_646 = unsafe { sqlite3FindIndex(*__slate_slot_589, *__slate_slot_583 as *const i8, *__slate_slot_584) };
if *__slate_slot_646 == std::ptr::null_mut::<Index>() {
// If there is no index named zRight, check to see if there is a
// WITHOUT ROWID table named zRight, and if there is, show the
// structure of the PRIMARY KEY index for that table.
*__slate_slot_647 = unsafe { sqlite3LocateTable(pParse, (2 as i32) as u32, *__slate_slot_583 as *const i8, *__slate_slot_584) };
if *__slate_slot_647 != std::ptr::null_mut::<Table>() && !((unsafe { (*(*__slate_slot_647)).tabFlags }) & ((128 as i32) as u32) == ((0 as i32) as u32)) {
*__slate_slot_646 = unsafe { sqlite3PrimaryKeyIndex(*__slate_slot_647) };
}
}
if *__slate_slot_646 != std::ptr::null_mut::<Index>() {
std::ptr::write(__slate_slot_648, unsafe { sqlite3SchemaToIndex(*__slate_slot_589, unsafe { (*(*__slate_slot_646)).pSchema }) });
if (unsafe { (*(*__slate_slot_592)).iArg }) != (0 as u64) {
// PRAGMA index_xinfo (newer version with more rows and columns)
*__slate_slot_650 = ((unsafe { (*(*__slate_slot_646)).nColumn }) as u32) as i32;
unsafe {
(*pParse).nMem = 6 as i32;
}
} else {
// PRAGMA index_info (legacy version)
*__slate_slot_650 = ((unsafe { (*(*__slate_slot_646)).nKeyCol }) as u32) as i32;
unsafe {
(*pParse).nMem = 3 as i32;
}
}
*__slate_slot_647 = unsafe { (*(*__slate_slot_646)).pTable };
unsafe { sqlite3CodeVerifySchema(pParse, *__slate_slot_648) };
0 as i32;
*__slate_slot_649 = 0 as i32;
loop {
if *__slate_slot_649 < *__slate_slot_650 {
std::ptr::write(__slate_slot_651, unsafe { *unsafe { unsafe { (*(*__slate_slot_646)).aiColumn }.offset(*__slate_slot_649 as isize) } });
unsafe { sqlite3VdbeMultiLoad(*__slate_slot_591, 1 as i32, (b"iisX\0".as_ptr() as *mut i8) as *const i8, *__slate_slot_649, *__slate_slot_651 as i32, if (*__slate_slot_651 as i32) < (0 as i32) { std::ptr::null_mut::<i8>() } else { unsafe { (*unsafe { unsafe { (*(*__slate_slot_647)).aCol }.offset((*__slate_slot_651 as i32) as isize) }).zCnName } }) };
if (unsafe { (*(*__slate_slot_592)).iArg }) != (0 as u64) {
unsafe { sqlite3VdbeMultiLoad(*__slate_slot_591, 4 as i32, (b"isiX\0".as_ptr() as *mut i8) as *const i8, ((unsafe { *unsafe { unsafe { (*(*__slate_slot_646)).aSortOrder }.offset(*__slate_slot_649 as isize) } }) as u32) as i32, unsafe { *unsafe { unsafe { (*(*__slate_slot_646)).azColl }.offset(*__slate_slot_649 as isize) } }, (*__slate_slot_649 < (((unsafe { (*(*__slate_slot_646)).nKeyCol }) as u32) as i32)) as i32) };
}
unsafe { sqlite3VdbeAddOp2(*__slate_slot_591, 86 as i32, 1 as i32, unsafe { (*pParse).nMem }) };
std::ptr::write(__slate_slot_1552, *__slate_slot_649);
std::ptr::write(__slate_slot_1553, *__slate_slot_1552 + (1 as i32));
*__slate_slot_649 = *__slate_slot_1553;
} else {
break '__join_2;
}
}
}
}
} else {
if __t0 == (21 as i32) {
if *__slate_slot_583 != std::ptr::null_mut::<i8>() {
*__slate_slot_653 = unsafe { sqlite3FindTable(*__slate_slot_589, *__slate_slot_583 as *const i8, *__slate_slot_584) };
if *__slate_slot_653 != std::ptr::null_mut::<Table>() {
std::ptr::write(__slate_slot_655, unsafe { sqlite3SchemaToIndex(*__slate_slot_589, unsafe { (*(*__slate_slot_653)).pSchema }) });
unsafe {
(*pParse).nMem = 5 as i32;
}
unsafe { sqlite3CodeVerifySchema(pParse, *__slate_slot_655) };
*__slate_slot_652 = unsafe { (*(*__slate_slot_653)).pIndex };
*__slate_slot_654 = 0 as i32;
loop {
if *__slate_slot_652 != std::ptr::null_mut::<Index>() {
std::ptr::write(__slate_slot_656, [(b"c\0".as_ptr() as *mut i8) as *const i8, (b"u\0".as_ptr() as *mut i8) as *const i8, (b"pk\0".as_ptr() as *mut i8) as *const i8]);
unsafe { sqlite3VdbeMultiLoad(*__slate_slot_591, 1 as i32, (b"isisi\0".as_ptr() as *mut i8) as *const i8, *__slate_slot_654, unsafe { (*(*__slate_slot_652)).zName }, ((((unsafe { (*(*__slate_slot_652)).onError }) as u32) as i32) != (0 as i32)) as i32, unsafe { *unsafe { ((*__slate_slot_656).as_mut_ptr() as *mut *const i8).offset(((unsafe { (*(*__slate_slot_652)).__slate_bits_0.__get_idxType() }) as i32) as isize) } }, ((unsafe { (*(*__slate_slot_652)).pPartIdxWhere }) != std::ptr::null_mut::<Expr>()) as i32) };
*__slate_slot_652 = unsafe { (*(*__slate_slot_652)).pNext };
std::ptr::write(__slate_slot_1554, *__slate_slot_654);
std::ptr::write(__slate_slot_1555, *__slate_slot_1554 + (1 as i32));
*__slate_slot_654 = *__slate_slot_1555;
} else {
break '__join_2;
}
}
}
}
} else {
if __t0 == (12 as i32) {
unsafe {
(*pParse).nMem = 3 as i32;
}
*__slate_slot_657 = 0 as i32;
loop {
if *__slate_slot_657 < unsafe { (*(*__slate_slot_589)).nDb } {
'__join_335: {
if (unsafe { (*unsafe { unsafe { (*(*__slate_slot_589)).aDb }.offset(*__slate_slot_657 as isize) }).pBt }) == std::ptr::null_mut::<Btree>() {
} else {
0 as i32;
unsafe { sqlite3VdbeMultiLoad(*__slate_slot_591, 1 as i32, (b"iss\0".as_ptr() as *mut i8) as *const i8, *__slate_slot_657, unsafe { (*unsafe { unsafe { (*(*__slate_slot_589)).aDb }.offset(*__slate_slot_657 as isize) }).zDbSName }, unsafe { sqlite3BtreeGetFilename(unsafe { (*unsafe { unsafe { (*(*__slate_slot_589)).aDb }.offset(*__slate_slot_657 as isize) }).pBt }) }) };
}
}
std::ptr::write(__slate_slot_1556, *__slate_slot_657);
std::ptr::write(__slate_slot_1557, *__slate_slot_1556 + (1 as i32));
*__slate_slot_657 = *__slate_slot_1557;
} else {
break '__join_2;
}
}
} else {
if __t0 == (9 as i32) {
std::ptr::write(__slate_slot_658, 0 as i32);
unsafe {
(*pParse).nMem = 2 as i32;
}
*__slate_slot_659 = unsafe { (*unsafe { std::ptr::addr_of_mut!((*(*__slate_slot_589)).aCollSeq) }).first };
loop {
if *__slate_slot_659 != std::ptr::null_mut::<HashElem>() {
std::ptr::write(__slate_slot_660, (unsafe { (*(*__slate_slot_659)).data }) as *mut CollSeq);
std::ptr::write(__slate_slot_1558, *__slate_slot_658);
std::ptr::write(__slate_slot_1559, *__slate_slot_1558 + (1 as i32));
*__slate_slot_658 = *__slate_slot_1559;
unsafe { sqlite3VdbeMultiLoad(*__slate_slot_591, 1 as i32, (b"is\0".as_ptr() as *mut i8) as *const i8, *__slate_slot_1558, unsafe { (*(*__slate_slot_660)).zName }) };
*__slate_slot_659 = unsafe { (*(*__slate_slot_659)).next };
} else {
break '__join_2;
}
}
} else {
if __t0 == (17 as i32) {
std::ptr::write(__slate_slot_664, ((unsafe { (*(*__slate_slot_589)).mDbFlags }) & ((32 as i32) as u32) != ((0 as i32) as u32)) as i32);
unsafe {
(*pParse).nMem = 6 as i32;
}
*__slate_slot_661 = 0 as i32;
loop {
if *__slate_slot_661 < (23 as i32) {
*__slate_slot_663 = unsafe { *unsafe { unsafe { std::ptr::addr_of_mut!(sqlite3BuiltinFunctions.a) as *mut *mut FuncDef }.offset(*__slate_slot_661 as isize) } };
loop {
if *__slate_slot_663 != std::ptr::null_mut::<FuncDef>() {
0 as i32;
pragmaFunclistLine(*__slate_slot_591, *__slate_slot_663, 1 as i32, *__slate_slot_664);
*__slate_slot_663 = unsafe { (*(*__slate_slot_663)).u.pHash };
} else {
break;
}
}
std::ptr::write(__slate_slot_1560, *__slate_slot_661);
std::ptr::write(__slate_slot_1561, *__slate_slot_1560 + (1 as i32));
*__slate_slot_661 = *__slate_slot_1561;
} else {
break;
}
}
*__slate_slot_662 = unsafe { (*unsafe { std::ptr::addr_of_mut!((*(*__slate_slot_589)).aFunc) }).first };
loop {
if *__slate_slot_662 != std::ptr::null_mut::<HashElem>() {
*__slate_slot_663 = (unsafe { (*(*__slate_slot_662)).data }) as *mut FuncDef;
0 as i32;
pragmaFunclistLine(*__slate_slot_591, *__slate_slot_663, 0 as i32, *__slate_slot_664);
*__slate_slot_662 = unsafe { (*(*__slate_slot_662)).next };
} else {
break '__join_2;
}
}
} else {
if __t0 == (29 as i32) {
unsafe {
(*pParse).nMem = 1 as i32;
}
*__slate_slot_665 = unsafe { (*unsafe { std::ptr::addr_of_mut!((*(*__slate_slot_589)).aModule) }).first };
loop {
if *__slate_slot_665 != std::ptr::null_mut::<HashElem>() {
std::ptr::write(__slate_slot_666, (unsafe { (*(*__slate_slot_665)).data }) as *mut Module);
unsafe { sqlite3VdbeMultiLoad(*__slate_slot_591, 1 as i32, (b"s\0".as_ptr() as *mut i8) as *const i8, unsafe { (*(*__slate_slot_666)).zName }) };
*__slate_slot_665 = unsafe { (*(*__slate_slot_665)).next };
} else {
break '__join_2;
}
}
} else {
if __t0 == (32 as i32) {
*__slate_slot_667 = 0 as i32;
loop {
if *__slate_slot_667 < ((((1584 as u64) / (24 as u64)) as u32) as i32) {
unsafe { sqlite3VdbeMultiLoad(*__slate_slot_591, 1 as i32, (b"s\0".as_ptr() as *mut i8) as *const i8, unsafe { (*unsafe { unsafe { std::ptr::addr_of!(aPragmaName.0) as *const PragmaName }.offset(*__slate_slot_667 as isize) }).zName }) };
std::ptr::write(__slate_slot_1562, *__slate_slot_667);
std::ptr::write(__slate_slot_1563, *__slate_slot_1562 + (1 as i32));
*__slate_slot_667 = *__slate_slot_1563;
} else {
break '__join_2;
}
}
} else {
if __t0 == (16 as i32) {
if *__slate_slot_583 != std::ptr::null_mut::<i8>() {
*__slate_slot_669 = unsafe { sqlite3FindTable(*__slate_slot_589, *__slate_slot_583 as *const i8, *__slate_slot_584) };
if *__slate_slot_669 != std::ptr::null_mut::<Table>() && (((unsafe { (*(*__slate_slot_669)).eTabType }) as u32) as i32) == (0 as i32) {
*__slate_slot_668 = unsafe { (*(*__slate_slot_669)).u.tab.pFKey };
if *__slate_slot_668 != std::ptr::null_mut::<FKey>() {
std::ptr::write(__slate_slot_670, unsafe { sqlite3SchemaToIndex(*__slate_slot_589, unsafe { (*(*__slate_slot_669)).pSchema }) });
std::ptr::write(__slate_slot_671, 0 as i32);
unsafe {
(*pParse).nMem = 8 as i32;
}
unsafe { sqlite3CodeVerifySchema(pParse, *__slate_slot_670) };
loop {
if *__slate_slot_668 != std::ptr::null_mut::<FKey>() {
*__slate_slot_672 = 0 as i32;
loop {
if *__slate_slot_672 < unsafe { (*(*__slate_slot_668)).nCol } {
unsafe { sqlite3VdbeMultiLoad(*__slate_slot_591, 1 as i32, (b"iissssss\0".as_ptr() as *mut i8) as *const i8, *__slate_slot_671, *__slate_slot_672, unsafe { (*(*__slate_slot_668)).zTo }, unsafe { (*unsafe { unsafe { (*(*__slate_slot_669)).aCol }.offset((unsafe { (*unsafe { unsafe { std::ptr::addr_of_mut!((*(*__slate_slot_668)).aCol) as *mut sColMap }.offset(*__slate_slot_672 as isize) }).iFrom }) as isize) }).zCnName }, unsafe { (*unsafe { unsafe { std::ptr::addr_of_mut!((*(*__slate_slot_668)).aCol) as *mut sColMap }.offset(*__slate_slot_672 as isize) }).zCol }, actionName(unsafe { *unsafe { unsafe { (*(*__slate_slot_668)).aAction.as_mut_ptr() as *mut u8 }.offset((1 as i32) as isize) } }), actionName(unsafe { *unsafe { unsafe { (*(*__slate_slot_668)).aAction.as_mut_ptr() as *mut u8 }.offset((0 as i32) as isize) } }), b"NONE\0".as_ptr() as *mut i8) }; // ON UPDATE
// ON DELETE
std::ptr::write(__slate_slot_1564, *__slate_slot_672);
std::ptr::write(__slate_slot_1565, *__slate_slot_1564 + (1 as i32));
*__slate_slot_672 = *__slate_slot_1565;
} else {
break;
}
}
std::ptr::write(__slate_slot_1566, *__slate_slot_671);
std::ptr::write(__slate_slot_1567, *__slate_slot_1566 + (1 as i32));
*__slate_slot_671 = *__slate_slot_1567;
*__slate_slot_668 = unsafe { (*(*__slate_slot_668)).pNextFrom };
} else {
break '__join_2;
}
}
}
}
}
} else {
if __t0 == (15 as i32) {
*__slate_slot_681 = (unsafe { (*pParse).nMem }) + (1 as i32);
std::ptr::write(__slate_slot_1568, pParse);
std::ptr::write(__slate_slot_1569, unsafe { (*(*__slate_slot_1568)).nMem });
std::ptr::write(__slate_slot_1570, *__slate_slot_1569 + (4 as i32));
unsafe {
(*(*__slate_slot_1568)).nMem = *__slate_slot_1570;
}
std::ptr::write(__slate_slot_1571, pParse);
std::ptr::write(__slate_slot_1572, unsafe { (*(*__slate_slot_1571)).nMem });
std::ptr::write(__slate_slot_1573, *__slate_slot_1572 + (1 as i32));
unsafe {
(*(*__slate_slot_1571)).nMem = *__slate_slot_1573;
}
*__slate_slot_682 = *__slate_slot_1573;
*__slate_slot_679 = unsafe { (*unsafe { std::ptr::addr_of_mut!((*unsafe { (*unsafe { unsafe { (*(*__slate_slot_589)).aDb }.offset(*__slate_slot_587 as isize) }).pSchema }).tblHash) }).first };
loop {
if *__slate_slot_679 != std::ptr::null_mut::<HashElem>() {
if *__slate_slot_583 != std::ptr::null_mut::<i8>() {
*__slate_slot_674 = unsafe { sqlite3LocateTable(pParse, (0 as i32) as u32, *__slate_slot_583 as *const i8, *__slate_slot_584) };
*__slate_slot_679 = std::ptr::null_mut::<HashElem>();
} else {
*__slate_slot_674 = (unsafe { (*(*__slate_slot_679)).data }) as *mut Table;
*__slate_slot_679 = unsafe { (*(*__slate_slot_679)).next };
}
if !(*__slate_slot_674 == std::ptr::null_mut::<Table>() || !((((unsafe { (*(*__slate_slot_674)).eTabType }) as u32) as i32) == (0 as i32)) || (unsafe { (*(*__slate_slot_674)).u.tab.pFKey }) == std::ptr::null_mut::<FKey>()) {
*__slate_slot_587 = unsafe { sqlite3SchemaToIndex(*__slate_slot_589, unsafe { (*(*__slate_slot_674)).pSchema }) };
*__slate_slot_584 = (unsafe { (*unsafe { unsafe { (*(*__slate_slot_589)).aDb }.offset(*__slate_slot_587 as isize) }).zDbSName }) as *const i8;
unsafe { sqlite3CodeVerifySchema(pParse, *__slate_slot_587) };
unsafe { sqlite3TableLock(pParse, *__slate_slot_587, unsafe { (*(*__slate_slot_674)).tnum }, ((0 as i32) as i8) as u8, (unsafe { (*(*__slate_slot_674)).zName }) as *const i8) };
unsafe { sqlite3TouchRegister(pParse, ((unsafe { (*(*__slate_slot_674)).nCol }) as i32) + *__slate_slot_682) };
unsafe { sqlite3OpenTable(pParse, 0 as i32, *__slate_slot_587, *__slate_slot_674, 114 as i32) };
unsafe { sqlite3VdbeLoadString(*__slate_slot_591, *__slate_slot_681, (unsafe { (*(*__slate_slot_674)).zName }) as *const i8) };
0 as i32;
*__slate_slot_677 = 1 as i32;
std::ptr::write(__slate_slot_1574, unsafe { (*(*__slate_slot_674)).u.tab.pFKey });
*__slate_slot_673 = *__slate_slot_1574;
'__join_292: {
'__loop_293: loop {
if *__slate_slot_673 != std::ptr::null_mut::<FKey>() {
*__slate_slot_675 = unsafe { sqlite3FindTable(*__slate_slot_589, (unsafe { (*(*__slate_slot_673)).zTo }) as *const i8, *__slate_slot_584) };
if *__slate_slot_675 == std::ptr::null_mut::<Table>() {
} else {
*__slate_slot_676 = std::ptr::null_mut::<Index>();
unsafe { sqlite3TableLock(pParse, *__slate_slot_587, unsafe { (*(*__slate_slot_675)).tnum }, ((0 as i32) as i8) as u8, (unsafe { (*(*__slate_slot_675)).zName }) as *const i8) };
*__slate_slot_680 = unsafe { sqlite3FkLocateIndex(pParse, *__slate_slot_675, *__slate_slot_673, std::ptr::addr_of_mut!(*__slate_slot_676), std::ptr::null_mut::<*mut i32>()) };
if *__slate_slot_680 == (0 as i32) {
if *__slate_slot_676 == std::ptr::null_mut::<Index>() {
unsafe { sqlite3OpenTable(pParse, *__slate_slot_677, *__slate_slot_587, *__slate_slot_675, 114 as i32) };
} else {
unsafe { sqlite3VdbeAddOp3(*__slate_slot_591, 114 as i32, *__slate_slot_677, (unsafe { (*(*__slate_slot_676)).tnum }) as i32, *__slate_slot_587) };
unsafe { sqlite3VdbeSetP4KeyInfo(pParse, *__slate_slot_676) };
}
} else {
break '__loop_293;
}
}
std::ptr::write(__slate_slot_1575, *__slate_slot_677);
std::ptr::write(__slate_slot_1576, *__slate_slot_1575 + (1 as i32));
*__slate_slot_677 = *__slate_slot_1576;
std::ptr::write(__slate_slot_1577, unsafe { (*(*__slate_slot_673)).pNextFrom });
*__slate_slot_673 = *__slate_slot_1577;
} else {
break '__join_292;
}
}
*__slate_slot_679 = std::ptr::null_mut::<HashElem>();
}
0 as i32;
if *__slate_slot_673 != std::ptr::null_mut::<FKey>() {
break '__join_2;
} else {
if (unsafe { (*pParse).nTab }) < *__slate_slot_677 {
unsafe {
(*pParse).nTab = *__slate_slot_677;
}
}
*__slate_slot_683 = unsafe { sqlite3VdbeAddOp1(*__slate_slot_591, 36 as i32, 0 as i32) };
{
}
0 as i32;
*__slate_slot_677 = 1 as i32;
std::ptr::write(__slate_slot_1578, unsafe { (*(*__slate_slot_674)).u.tab.pFKey });
*__slate_slot_673 = *__slate_slot_1578;
loop {
if *__slate_slot_673 != std::ptr::null_mut::<FKey>() {
*__slate_slot_675 = unsafe { sqlite3FindTable(*__slate_slot_589, (unsafe { (*(*__slate_slot_673)).zTo }) as *const i8, *__slate_slot_584) };
*__slate_slot_676 = std::ptr::null_mut::<Index>();
*__slate_slot_685 = std::ptr::null_mut::<i32>();
if *__slate_slot_675 != std::ptr::null_mut::<Table>() {
*__slate_slot_680 = unsafe { sqlite3FkLocateIndex(pParse, *__slate_slot_675, *__slate_slot_673, std::ptr::addr_of_mut!(*__slate_slot_676), std::ptr::addr_of_mut!(*__slate_slot_685)) };
0 as i32;
}
*__slate_slot_684 = unsafe { sqlite3VdbeMakeLabel(pParse) };
// Generate code to read the child key values into registers
// regRow..regRow+n. If any of the child key values are NULL, this
// row cannot cause an FK violation. Jump directly to addrOk in
// this case.
unsafe { sqlite3TouchRegister(pParse, *__slate_slot_682 + unsafe { (*(*__slate_slot_673)).nCol }) };
*__slate_slot_678 = 0 as i32;
loop {
if *__slate_slot_678 < unsafe { (*(*__slate_slot_673)).nCol } {
std::ptr::write(__slate_slot_686, if *__slate_slot_685 != std::ptr::null_mut::<i32>() { unsafe { *unsafe { (*__slate_slot_685).offset(*__slate_slot_678 as isize) } } } else { unsafe { (*unsafe { unsafe { std::ptr::addr_of_mut!((*(*__slate_slot_673)).aCol) as *mut sColMap }.offset(*__slate_slot_678 as isize) }).iFrom } });
unsafe { sqlite3ExprCodeGetColumnOfTable(*__slate_slot_591, *__slate_slot_674, 0 as i32, *__slate_slot_686, *__slate_slot_682 + *__slate_slot_678) };
unsafe { sqlite3VdbeAddOp2(*__slate_slot_591, 51 as i32, *__slate_slot_682 + *__slate_slot_678, *__slate_slot_684) };
{
}
std::ptr::write(__slate_slot_1582, *__slate_slot_678);
std::ptr::write(__slate_slot_1583, *__slate_slot_1582 + (1 as i32));
*__slate_slot_678 = *__slate_slot_1583;
} else {
break;
}
}
// Generate code to query the parent index for a matching parent
// key. If a match is found, jump to addrOk.
if *__slate_slot_676 != std::ptr::null_mut::<Index>() {
unsafe { sqlite3VdbeAddOp4(*__slate_slot_591, 98 as i32, *__slate_slot_682, unsafe { (*(*__slate_slot_673)).nCol }, 0 as i32, unsafe { sqlite3IndexAffinityStr(*__slate_slot_589, *__slate_slot_676) }, unsafe { (*(*__slate_slot_673)).nCol }) };
unsafe { sqlite3VdbeAddOp4Int(*__slate_slot_591, 29 as i32, *__slate_slot_677, *__slate_slot_684, *__slate_slot_682, unsafe { (*(*__slate_slot_673)).nCol }) };
{
}
} else {
if *__slate_slot_675 != std::ptr::null_mut::<Table>() {
std::ptr::write(__slate_slot_687, (unsafe { sqlite3VdbeCurrentAddr(*__slate_slot_591) }) + (2 as i32));
unsafe { sqlite3VdbeAddOp3(*__slate_slot_591, 30 as i32, *__slate_slot_677, *__slate_slot_687, *__slate_slot_682) };
{
}
unsafe { sqlite3VdbeGoto(*__slate_slot_591, *__slate_slot_684) };
0 as i32;
}
}
// Generate code to report an FK violation to the caller.
if (unsafe { (*(*__slate_slot_674)).tabFlags }) & ((128 as i32) as u32) == ((0 as i32) as u32) {
unsafe { sqlite3VdbeAddOp2(*__slate_slot_591, 137 as i32, 0 as i32, *__slate_slot_681 + (1 as i32)) };
} else {
unsafe { sqlite3VdbeAddOp2(*__slate_slot_591, 77 as i32, 0 as i32, *__slate_slot_681 + (1 as i32)) };
}
unsafe { sqlite3VdbeMultiLoad(*__slate_slot_591, *__slate_slot_681 + (2 as i32), (b"siX\0".as_ptr() as *mut i8) as *const i8, unsafe { (*(*__slate_slot_673)).zTo }, *__slate_slot_677 - (1 as i32)) };
unsafe { sqlite3VdbeAddOp2(*__slate_slot_591, 86 as i32, *__slate_slot_681, 4 as i32) };
unsafe { sqlite3VdbeResolveLabel(*__slate_slot_591, *__slate_slot_684) };
unsafe { sqlite3DbFree(*__slate_slot_589, *__slate_slot_685 as *mut ()) };
std::ptr::write(__slate_slot_1579, *__slate_slot_677);
std::ptr::write(__slate_slot_1580, *__slate_slot_1579 + (1 as i32));
*__slate_slot_677 = *__slate_slot_1580;
std::ptr::write(__slate_slot_1581, unsafe { (*(*__slate_slot_673)).pNextFrom });
*__slate_slot_673 = *__slate_slot_1581;
} else {
break;
}
}
unsafe { sqlite3VdbeAddOp2(*__slate_slot_591, 40 as i32, 0 as i32, *__slate_slot_683 + (1 as i32)) };
{
}
unsafe { sqlite3VdbeJumpHere(*__slate_slot_591, *__slate_slot_683) };
}
}
} else {
break '__join_2;
}
}
} else {
if __t0 == (8 as i32) {
if *__slate_slot_583 != std::ptr::null_mut::<i8>() {
unsafe { sqlite3RegisterLikeFunctions(*__slate_slot_589, (sqlite3GetBoolean(*__slate_slot_583 as *const i8, ((0 as i32) as i8) as u8) as u32) as i32) };
}
} else {
if __t0 == (22 as i32) {
std::ptr::write(__slate_slot_692, std::ptr::null_mut::<Table>());
std::ptr::write(__slate_slot_693, ((((unsafe { *unsafe { unsafe { std::ptr::addr_of!(sqlite3UpperToLower) as *const u8 }.offset(((((unsafe { *unsafe { (*__slate_slot_582).offset((0 as i32) as isize) } }) as u8) as u32) as i32) as isize) } }) as u32) as i32) == (113 as i32)) as i32);
// If the PRAGMA command was of the form "PRAGMA <db>.integrity_check",
// then iDb is set to the index of the database identified by <db>.
// In this case, the integrity of database iDb only is verified by
// the VDBE created below.
//
// Otherwise, if the command was simply "PRAGMA integrity_check" (or
// "PRAGMA quick_check"), then iDb is set to 0. In this case, set iDb
// to -1 here, to indicate that the VDBE should verify the integrity
// of all attached databases.
0 as i32;
0 as i32;
if (unsafe { (*pId2).z }) == std::ptr::null::<i8>() {
*__slate_slot_587 = -(1 as i32);
}
// Initialize the VDBE program
unsafe {
(*pParse).nMem = 6 as i32;
}
// Set the maximum error count
*__slate_slot_691 = 100 as i32;
if *__slate_slot_583 != std::ptr::null_mut::<i8>() {
if (unsafe { sqlite3GetInt32(unsafe { (*pValue).z }, std::ptr::addr_of_mut!(*__slate_slot_691)) }) != (0 as i32) {
if *__slate_slot_691 <= (0 as i32) {
*__slate_slot_691 = 100 as i32;
}
} else {
*__slate_slot_692 = unsafe { sqlite3LocateTable(pParse, (0 as i32) as u32, *__slate_slot_583 as *const i8, (if *__slate_slot_587 >= (0 as i32) { unsafe { (*unsafe { unsafe { (*(*__slate_slot_589)).aDb }.offset(*__slate_slot_587 as isize) }).zDbSName } } else { std::ptr::null_mut::<i8>() }) as *const i8) };
}
}
unsafe { sqlite3VdbeAddOp2(*__slate_slot_591, 73 as i32, *__slate_slot_691 - (1 as i32), 1 as i32) }; // reg[1] holds errors left
// Do an integrity check on each database file
*__slate_slot_688 = 0 as i32;
'__loop_114: loop {
if *__slate_slot_688 < unsafe { (*(*__slate_slot_589)).nDb } {
'__join_115: {
std::ptr::write(__slate_slot_697, 0 as i32);
if (0 as i32) != (0 as i32) && *__slate_slot_688 == (1 as i32) {
} else {
if *__slate_slot_587 >= (0 as i32) && *__slate_slot_688 != *__slate_slot_587 {
} else {
unsafe { sqlite3CodeVerifySchema(pParse, *__slate_slot_688) };
unsafe {
(*pParse).__slate_bits_0.__set_okConstFactor((0 as i32) as u32);
}
// tag-20230327-1
// Do an integrity check of the B-Tree
//
// Begin by finding the root pages numbers
// for all tables and indices in the database.
0 as i32;
*__slate_slot_695 = unsafe { std::ptr::addr_of_mut!((*unsafe { (*unsafe { unsafe { (*(*__slate_slot_589)).aDb }.offset(*__slate_slot_688 as isize) }).pSchema }).tblHash) };
*__slate_slot_697 = 0 as i32;
std::ptr::write(__slate_slot_1586, unsafe { (*(*__slate_slot_695)).first });
*__slate_slot_694 = *__slate_slot_1586;
loop {
if *__slate_slot_694 != std::ptr::null_mut::<HashElem>() {
'__join_253: {
std::ptr::write(__slate_slot_698, (unsafe { (*(*__slate_slot_694)).data }) as *mut Table);
if tableSkipIntegrityCheck(*__slate_slot_698 as *const Table, *__slate_slot_692 as *const Table) != (0 as i32) {
} else {
if (unsafe { (*(*__slate_slot_698)).tabFlags }) & ((128 as i32) as u32) == ((0 as i32) as u32) {
std::ptr::write(__slate_slot_1587, *__slate_slot_697);
std::ptr::write(__slate_slot_1588, *__slate_slot_1587 + (1 as i32));
*__slate_slot_697 = *__slate_slot_1588;
}
*__slate_slot_699 = unsafe { (*(*__slate_slot_698)).pIndex };
loop {
if *__slate_slot_699 != std::ptr::null_mut::<Index>() {
std::ptr::write(__slate_slot_1589, *__slate_slot_697);
std::ptr::write(__slate_slot_1590, *__slate_slot_1589 + (1 as i32));
*__slate_slot_697 = *__slate_slot_1590;
*__slate_slot_699 = unsafe { (*(*__slate_slot_699)).pNext };
} else {
break '__join_253;
}
}
}
}
*__slate_slot_694 = unsafe { (*(*__slate_slot_694)).next };
} else {
break;
}
}
if *__slate_slot_697 == (0 as i32) {
} else {
if *__slate_slot_692 != std::ptr::null_mut::<Table>() {
std::ptr::write(__slate_slot_1591, *__slate_slot_697);
std::ptr::write(__slate_slot_1592, *__slate_slot_1591 + (1 as i32));
*__slate_slot_697 = *__slate_slot_1592;
}
*__slate_slot_696 = (unsafe { sqlite3DbMallocRawNN(*__slate_slot_589, (4 as u64).wrapping_mul(((*__slate_slot_697 + (1 as i32)) as i64) as u64)) }) as *mut i32;
if *__slate_slot_696 == std::ptr::null_mut::<i32>() {
break '__loop_114;
} else {
*__slate_slot_697 = 0 as i32;
if *__slate_slot_692 != std::ptr::null_mut::<Table>() {
std::ptr::write(__slate_slot_1593, *__slate_slot_697);
std::ptr::write(__slate_slot_1594, *__slate_slot_1593 + (1 as i32));
*__slate_slot_697 = *__slate_slot_1594;
unsafe {
*unsafe { (*__slate_slot_696).offset(*__slate_slot_1594 as isize) } = 0 as i32;
}
}
*__slate_slot_694 = unsafe { (*(*__slate_slot_695)).first };
loop {
if *__slate_slot_694 != std::ptr::null_mut::<HashElem>() {
'__join_238: {
std::ptr::write(__slate_slot_700, (unsafe { (*(*__slate_slot_694)).data }) as *mut Table);
if tableSkipIntegrityCheck(*__slate_slot_700 as *const Table, *__slate_slot_692 as *const Table) != (0 as i32) {
} else {
if (unsafe { (*(*__slate_slot_700)).tabFlags }) & ((128 as i32) as u32) == ((0 as i32) as u32) {
std::ptr::write(__slate_slot_1595, *__slate_slot_697);
std::ptr::write(__slate_slot_1596, *__slate_slot_1595 + (1 as i32));
*__slate_slot_697 = *__slate_slot_1596;
unsafe {
*unsafe { (*__slate_slot_696).offset(*__slate_slot_1596 as isize) } = (unsafe { (*(*__slate_slot_700)).tnum }) as i32;
}
}
*__slate_slot_701 = unsafe { (*(*__slate_slot_700)).pIndex };
loop {
if *__slate_slot_701 != std::ptr::null_mut::<Index>() {
std::ptr::write(__slate_slot_1597, *__slate_slot_697);
std::ptr::write(__slate_slot_1598, *__slate_slot_1597 + (1 as i32));
*__slate_slot_697 = *__slate_slot_1598;
unsafe {
*unsafe { (*__slate_slot_696).offset(*__slate_slot_1598 as isize) } = (unsafe { (*(*__slate_slot_701)).tnum }) as i32;
}
*__slate_slot_701 = unsafe { (*(*__slate_slot_701)).pNext };
} else {
break '__join_238;
}
}
}
}
*__slate_slot_694 = unsafe { (*(*__slate_slot_694)).next };
} else {
break;
}
}
unsafe {
*unsafe { (*__slate_slot_696).offset((0 as i32) as isize) } = *__slate_slot_697;
}
// Make sure sufficient number of registers have been allocated
unsafe { sqlite3TouchRegister(pParse, (8 as i32) + *__slate_slot_697) };
unsafe { sqlite3VdbeAddOp3(*__slate_slot_591, 77 as i32, 0 as i32, 8 as i32, (8 as i32) + *__slate_slot_697) };
unsafe { sqlite3ClearTempRegCache(pParse) };
// Do the b-tree integrity checks
unsafe { sqlite3VdbeAddOp4(*__slate_slot_591, 157 as i32, 1 as i32, *__slate_slot_697, 8 as i32, (*__slate_slot_696 as *mut i8) as *const i8, -(13 as i32)) };
unsafe { sqlite3VdbeChangeP5(*__slate_slot_591, (*__slate_slot_688 as i16) as u16) };
*__slate_slot_690 = unsafe { sqlite3VdbeAddOp1(*__slate_slot_591, 51 as i32, 2 as i32) };
{
}
unsafe { sqlite3VdbeAddOp4(*__slate_slot_591, 118 as i32, 0 as i32, 3 as i32, 0 as i32, (unsafe { sqlite3MPrintf(*__slate_slot_589, (b"*** in database %s ***\n\0".as_ptr() as *mut i8) as *const i8, unsafe { (*unsafe { unsafe { (*(*__slate_slot_589)).aDb }.offset(*__slate_slot_688 as isize) }).zDbSName }) }) as *const i8, -(7 as i32)) };
unsafe { sqlite3VdbeAddOp3(*__slate_slot_591, 112 as i32, 2 as i32, 3 as i32, 3 as i32) };
integrityCheckResultRow(*__slate_slot_591);
unsafe { sqlite3VdbeJumpHere(*__slate_slot_591, *__slate_slot_690) };
// Check that the indexes all have the right number of rows
*__slate_slot_697 = if *__slate_slot_692 != std::ptr::null_mut::<Table>() { 1 as i32 } else { 0 as i32 };
unsafe { sqlite3VdbeLoadString(*__slate_slot_591, 2 as i32, (b"wrong # of entries in index \0".as_ptr() as *mut i8) as *const i8) };
*__slate_slot_694 = unsafe { (*(*__slate_slot_695)).first };
loop {
if *__slate_slot_694 != std::ptr::null_mut::<HashElem>() {
'__join_223: {
std::ptr::write(__slate_slot_702, 0 as i32);
std::ptr::write(__slate_slot_703, (unsafe { (*(*__slate_slot_694)).data }) as *mut Table);
if tableSkipIntegrityCheck(*__slate_slot_703 as *const Table, *__slate_slot_692 as *const Table) != (0 as i32) {
} else {
'__join_228: {
if (unsafe { (*(*__slate_slot_703)).tabFlags }) & ((128 as i32) as u32) == ((0 as i32) as u32) {
std::ptr::write(__slate_slot_1599, *__slate_slot_697);
std::ptr::write(__slate_slot_1600, *__slate_slot_1599 + (1 as i32));
*__slate_slot_697 = *__slate_slot_1600;
*__slate_slot_702 = *__slate_slot_1599;
} else {
*__slate_slot_702 = *__slate_slot_697;
*__slate_slot_704 = unsafe { (*(*__slate_slot_703)).pIndex };
loop {
if *__slate_slot_704 != std::ptr::null_mut::<Index>() {
if ((unsafe { (*(*__slate_slot_704)).__slate_bits_0.__get_idxType() }) as i32) == (2 as i32) {
break '__join_228;
} else {
std::ptr::write(__slate_slot_1601, *__slate_slot_702);
std::ptr::write(__slate_slot_1602, *__slate_slot_1601 + (1 as i32));
*__slate_slot_702 = *__slate_slot_1602;
*__slate_slot_704 = unsafe { (*(*__slate_slot_704)).pNext };
}
} else {
break '__join_228;
}
}
}
}
*__slate_slot_704 = unsafe { (*(*__slate_slot_703)).pIndex };
loop {
if *__slate_slot_704 != std::ptr::null_mut::<Index>() {
if (unsafe { (*(*__slate_slot_704)).pPartIdxWhere }) == std::ptr::null_mut::<Expr>() {
*__slate_slot_690 = unsafe { sqlite3VdbeAddOp3(*__slate_slot_591, 54 as i32, (8 as i32) + *__slate_slot_697, 0 as i32, (8 as i32) + *__slate_slot_702) };
{
}
unsafe { sqlite3VdbeLoadString(*__slate_slot_591, 4 as i32, (unsafe { (*(*__slate_slot_704)).zName }) as *const i8) };
unsafe { sqlite3VdbeAddOp3(*__slate_slot_591, 112 as i32, 4 as i32, 2 as i32, 3 as i32) };
integrityCheckResultRow(*__slate_slot_591);
unsafe { sqlite3VdbeJumpHere(*__slate_slot_591, *__slate_slot_690) };
}
std::ptr::write(__slate_slot_1603, *__slate_slot_697);
std::ptr::write(__slate_slot_1604, *__slate_slot_1603 + (1 as i32));
*__slate_slot_697 = *__slate_slot_1604;
*__slate_slot_704 = unsafe { (*(*__slate_slot_704)).pNext };
} else {
break '__join_223;
}
}
}
}
*__slate_slot_694 = unsafe { (*(*__slate_slot_694)).next };
} else {
break;
}
}
// Make sure all the indices are constructed correctly.
*__slate_slot_694 = unsafe { (*(*__slate_slot_695)).first };
loop {
if *__slate_slot_694 != std::ptr::null_mut::<HashElem>() {
std::ptr::write(__slate_slot_705, (unsafe { (*(*__slate_slot_694)).data }) as *mut Table);
std::ptr::write(__slate_slot_708, std::ptr::null_mut::<Index>());
std::ptr::write(__slate_slot_712, -(1 as i32));
if tableSkipIntegrityCheck(*__slate_slot_705 as *const Table, *__slate_slot_692 as *const Table) != (0 as i32) {
} else {
if !((((unsafe { (*(*__slate_slot_705)).eTabType }) as u32) as i32) == (0 as i32)) {
} else {
if *__slate_slot_693 != (0 as i32) || (unsafe { (*(*__slate_slot_705)).tabFlags }) & ((128 as i32) as u32) == ((0 as i32) as u32) {
*__slate_slot_707 = std::ptr::null_mut::<Index>();
*__slate_slot_714 = 0 as i32;
} else {
*__slate_slot_707 = unsafe { sqlite3PrimaryKeyIndex(*__slate_slot_705) };
*__slate_slot_714 = unsafe { sqlite3GetTempRange(pParse, ((unsafe { (*(*__slate_slot_707)).nKeyCol }) as u32) as i32) };
unsafe { sqlite3VdbeAddOp3(*__slate_slot_591, 77 as i32, 1 as i32, *__slate_slot_714, *__slate_slot_714 + (((unsafe { (*(*__slate_slot_707)).nKeyCol }) as u32) as i32) - (1 as i32)) };
}
unsafe { sqlite3OpenTableAndIndices(pParse, *__slate_slot_705, 114 as i32, ((0 as i32) as i8) as u8, 1 as i32, std::ptr::null_mut::<u8>(), std::ptr::addr_of_mut!(*__slate_slot_710), std::ptr::addr_of_mut!(*__slate_slot_711)) };
// reg[7] counts the number of entries in the table.
// reg[8+i] counts the number of entries in the i-th index
unsafe { sqlite3VdbeAddOp2(*__slate_slot_591, 73 as i32, 0 as i32, 7 as i32) };
*__slate_slot_689 = 0 as i32;
std::ptr::write(__slate_slot_1605, unsafe { (*(*__slate_slot_705)).pIndex });
*__slate_slot_706 = *__slate_slot_1605;
loop {
if *__slate_slot_706 != std::ptr::null_mut::<Index>() {
unsafe { sqlite3VdbeAddOp2(*__slate_slot_591, 73 as i32, 0 as i32, (8 as i32) + *__slate_slot_689) }; // index entries counter
*__slate_slot_706 = unsafe { (*(*__slate_slot_706)).pNext };
std::ptr::write(__slate_slot_1606, *__slate_slot_689);
std::ptr::write(__slate_slot_1607, *__slate_slot_1606 + (1 as i32));
*__slate_slot_689 = *__slate_slot_1607;
} else {
break;
}
}
0 as i32;
0 as i32;
unsafe { sqlite3VdbeAddOp2(*__slate_slot_591, 36 as i32, *__slate_slot_710, 0 as i32) };
{
}
*__slate_slot_709 = unsafe { sqlite3VdbeAddOp2(*__slate_slot_591, 88 as i32, 7 as i32, 1 as i32) };
// Fetch the right-most column from the table.  This will cause
// the entire record header to be parsed and sanity checked.  It
// will also prepopulate the cursor column cache that is used
// by the OP_IsType code, so it is a required step.
0 as i32;
if (unsafe { (*(*__slate_slot_705)).tabFlags }) & ((128 as i32) as u32) == ((0 as i32) as u32) {
*__slate_slot_715 = -(1 as i32);
*__slate_slot_689 = 0 as i32;
loop {
if *__slate_slot_689 < ((unsafe { (*(*__slate_slot_705)).nCol }) as i32) {
if (((unsafe { (*unsafe { unsafe { (*(*__slate_slot_705)).aCol }.offset(*__slate_slot_689 as isize) }).colFlags }) as u32) as i32) & (32 as i32) == (0 as i32) {
std::ptr::write(__slate_slot_1610, *__slate_slot_715);
std::ptr::write(__slate_slot_1611, *__slate_slot_1610 + (1 as i32));
*__slate_slot_715 = *__slate_slot_1611;
}
std::ptr::write(__slate_slot_1608, *__slate_slot_689);
std::ptr::write(__slate_slot_1609, *__slate_slot_1608 + (1 as i32));
*__slate_slot_689 = *__slate_slot_1609;
} else {
break;
}
}
if *__slate_slot_715 == ((unsafe { (*(*__slate_slot_705)).iPKey }) as i32) {
std::ptr::write(__slate_slot_1612, *__slate_slot_715);
std::ptr::write(__slate_slot_1613, *__slate_slot_1612 - (1 as i32));
*__slate_slot_715 = *__slate_slot_1613;
}
} else {
// COLFLAG_VIRTUAL columns are not included in the WITHOUT ROWID
// PK index column-count, so there is no need to account for them
// in this case.
*__slate_slot_715 = (((unsafe { (*unsafe { sqlite3PrimaryKeyIndex(*__slate_slot_705) }).nColumn }) as u32) as i32) - (1 as i32);
}
if *__slate_slot_715 >= (0 as i32) {
unsafe { sqlite3VdbeAddOp3(*__slate_slot_591, 96 as i32, *__slate_slot_710, *__slate_slot_715, 3 as i32) };
unsafe { sqlite3VdbeTypeofColumn(*__slate_slot_591, 3 as i32) };
}
'__join_196: {
if !(*__slate_slot_693 != (0 as i32)) {
if *__slate_slot_707 != std::ptr::null_mut::<Index>() {
*__slate_slot_716 = unsafe { sqlite3VdbeAddOp4Int(*__slate_slot_591, 42 as i32, *__slate_slot_710, 0 as i32, *__slate_slot_714, ((unsafe { (*(*__slate_slot_707)).nKeyCol }) as u32) as i32) };
{
}
unsafe { sqlite3VdbeAddOp1(*__slate_slot_591, 51 as i32, *__slate_slot_714) };
{
}
*__slate_slot_717 = unsafe { sqlite3MPrintf(*__slate_slot_589, (b"row not in PRIMARY KEY order for %s\0".as_ptr() as *mut i8) as *const i8, unsafe { (*(*__slate_slot_705)).zName }) };
unsafe { sqlite3VdbeAddOp4(*__slate_slot_591, 118 as i32, 0 as i32, 3 as i32, 0 as i32, *__slate_slot_717 as *const i8, -(7 as i32)) };
integrityCheckResultRow(*__slate_slot_591);
unsafe { sqlite3VdbeJumpHere(*__slate_slot_591, *__slate_slot_716) };
unsafe { sqlite3VdbeJumpHere(*__slate_slot_591, *__slate_slot_716 + (1 as i32)) };
*__slate_slot_689 = 0 as i32;
loop {
if *__slate_slot_689 < (((unsafe { (*(*__slate_slot_707)).nKeyCol }) as u32) as i32) {
unsafe { sqlite3ExprCodeLoadIndexColumn(pParse, *__slate_slot_707, *__slate_slot_710, *__slate_slot_689, *__slate_slot_714 + *__slate_slot_689) };
std::ptr::write(__slate_slot_1614, *__slate_slot_689);
std::ptr::write(__slate_slot_1615, *__slate_slot_1614 + (1 as i32));
*__slate_slot_689 = *__slate_slot_1615;
} else {
break '__join_196;
}
}
}
}
}
// Verify datatypes for all columns:
//
// (1) NOT NULL columns may not contain a NULL
// (2) Datatype must be exact for non-ANY columns in STRICT tables
// (3) Datatype for TEXT columns in non-STRICT tables must be
//     NULL, TEXT, or BLOB.
// (4) Datatype for numeric columns in non-STRICT tables must not
//     be a TEXT value that can be losslessly converted to numeric.
*__slate_slot_713 = ((unsafe { (*(*__slate_slot_705)).tabFlags }) & ((65536 as i32) as u32) != ((0 as i32) as u32)) as i32;
*__slate_slot_689 = 0 as i32;
loop {
if *__slate_slot_689 < ((unsafe { (*(*__slate_slot_705)).nCol }) as i32) {
std::ptr::write(__slate_slot_719, unsafe { unsafe { (*(*__slate_slot_705)).aCol }.offset(*__slate_slot_689 as isize) });
if *__slate_slot_689 == ((unsafe { (*(*__slate_slot_705)).iPKey }) as i32) {
} else {
if *__slate_slot_713 != (0 as i32) {
*__slate_slot_725 = (((unsafe { (*(*__slate_slot_719)).__slate_bits_0.__get_eCType() }) as i32) > (1 as i32)) as i32;
} else {
*__slate_slot_725 = (((unsafe { (*(*__slate_slot_719)).affinity }) as i32) > (65 as i32)) as i32;
}
if ((unsafe { (*(*__slate_slot_719)).__slate_bits_0.__get_notNull() }) as i32) == (0 as i32) && !(*__slate_slot_725 != (0 as i32)) {
} else {
// Compute the operands that will be needed for OP_IsType
*__slate_slot_724 = 5 as i32;
if (((unsafe { (*(*__slate_slot_719)).colFlags }) as u32) as i32) & (32 as i32) != (0 as i32) {
unsafe { sqlite3ExprCodeGetColumnOfTable(*__slate_slot_591, *__slate_slot_705, *__slate_slot_710, *__slate_slot_689, 3 as i32) };
*__slate_slot_722 = -(1 as i32);
*__slate_slot_723 = 3 as i32;
} else {
if (unsafe { (*(*__slate_slot_719)).iDflt }) != (0 as u16) {
std::ptr::write(__slate_slot_726, std::ptr::null_mut::<sqlite3_value>());
unsafe { sqlite3ValueFromExpr(*__slate_slot_589, (unsafe { sqlite3ColumnExpr(*__slate_slot_705, *__slate_slot_719) }) as *const Expr, unsafe { (*(*__slate_slot_589)).enc }, (unsafe { (*(*__slate_slot_719)).affinity }) as u8, std::ptr::addr_of_mut!(*__slate_slot_726)) };
if *__slate_slot_726 != std::ptr::null_mut::<sqlite3_value>() {
*__slate_slot_724 = unsafe { sqlite3_value_type(*__slate_slot_726) };
unsafe { sqlite3ValueFree(*__slate_slot_726) };
}
}
*__slate_slot_722 = *__slate_slot_710;
if !((unsafe { (*(*__slate_slot_705)).tabFlags }) & ((128 as i32) as u32) == ((0 as i32) as u32)) {
{
}
*__slate_slot_723 = unsafe { sqlite3TableColumnToIndex(unsafe { sqlite3PrimaryKeyIndex(*__slate_slot_705) }, *__slate_slot_689) };
} else {
*__slate_slot_723 = (unsafe { sqlite3TableColumnToStorage(*__slate_slot_705, *__slate_slot_689 as i16) }) as i32;
{
}
}
}
*__slate_slot_720 = unsafe { sqlite3VdbeMakeLabel(pParse) };
*__slate_slot_721 = unsafe { sqlite3VdbeMakeLabel(pParse) };
if ((unsafe { (*(*__slate_slot_719)).__slate_bits_0.__get_notNull() }) as i32) != (0 as i32) {
std::ptr::write(__slate_slot_728, unsafe { sqlite3VdbeAddOp4Int(*__slate_slot_591, 18 as i32, *__slate_slot_722, *__slate_slot_721, *__slate_slot_723, *__slate_slot_724) });
{
}
if *__slate_slot_722 < (0 as i32) {
unsafe { sqlite3VdbeChangeP5(*__slate_slot_591, ((15 as i32) as i16) as u16) }; // INT, REAL, TEXT, or BLOB
*__slate_slot_727 = *__slate_slot_728;
} else {
unsafe { sqlite3VdbeChangeP5(*__slate_slot_591, ((13 as i32) as i16) as u16) }; // INT, TEXT, or BLOB
// OP_IsType does not detect NaN values in the database file
// which should be treated as a NULL.  So if the header type
// is REAL, we have to load the actual data using OP_Column
// to reliably determine if the value is a NULL.
unsafe { sqlite3VdbeAddOp3(*__slate_slot_591, 96 as i32, *__slate_slot_722, *__slate_slot_723, 3 as i32) };
unsafe { sqlite3ColumnDefault(*__slate_slot_591, *__slate_slot_705, *__slate_slot_689, 3 as i32) };
*__slate_slot_727 = unsafe { sqlite3VdbeAddOp2(*__slate_slot_591, 52 as i32, 3 as i32, *__slate_slot_721) };
{
}
}
*__slate_slot_718 = unsafe { sqlite3MPrintf(*__slate_slot_589, (b"NULL value in %s.%s\0".as_ptr() as *mut i8) as *const i8, unsafe { (*(*__slate_slot_705)).zName }, unsafe { (*(*__slate_slot_719)).zCnName }) };
unsafe { sqlite3VdbeAddOp4(*__slate_slot_591, 118 as i32, 0 as i32, 3 as i32, 0 as i32, *__slate_slot_718 as *const i8, -(7 as i32)) };
if *__slate_slot_725 != (0 as i32) {
unsafe { sqlite3VdbeGoto(*__slate_slot_591, *__slate_slot_720) };
unsafe { sqlite3VdbeJumpHere(*__slate_slot_591, *__slate_slot_728) };
unsafe { sqlite3VdbeJumpHere(*__slate_slot_591, *__slate_slot_727) };
} else {
// VDBE byte code will fall thru
}
}
if *__slate_slot_713 != (0 as i32) && *__slate_slot_725 != (0 as i32) {
// (2) Datatype must be exact for non-ANY columns in STRICT tables
// ANY
// BLOB
// INT
// INTEGER
// REAL
// TEXT
unsafe { sqlite3VdbeAddOp4Int(*__slate_slot_591, 18 as i32, *__slate_slot_722, *__slate_slot_721, *__slate_slot_723, *__slate_slot_724) };
0 as i32;
unsafe { sqlite3VdbeChangeP5(*__slate_slot_591, (unsafe { *unsafe { unsafe { std::ptr::addr_of_mut!(aStdTypeMask) as *mut u8 }.offset((((unsafe { (*(*__slate_slot_719)).__slate_bits_0.__get_eCType() }) as i32) - (1 as i32)) as isize) } }) as u16) };
{
}
*__slate_slot_718 = unsafe { sqlite3MPrintf(*__slate_slot_589, (b"non-%s value in %s.%s\0".as_ptr() as *mut i8) as *const i8, unsafe { *unsafe { unsafe { std::ptr::addr_of_mut!(sqlite3StdType) as *mut *const i8 }.offset((((unsafe { (*(*__slate_slot_719)).__slate_bits_0.__get_eCType() }) as i32) - (1 as i32)) as isize) } }, unsafe { (*(*__slate_slot_705)).zName }, unsafe { (*unsafe { unsafe { (*(*__slate_slot_705)).aCol }.offset(*__slate_slot_689 as isize) }).zCnName }) };
unsafe { sqlite3VdbeAddOp4(*__slate_slot_591, 118 as i32, 0 as i32, 3 as i32, 0 as i32, *__slate_slot_718 as *const i8, -(7 as i32)) };
} else {
if !(*__slate_slot_713 != (0 as i32)) && ((unsafe { (*(*__slate_slot_719)).affinity }) as i32) == (66 as i32) {
// (3) Datatype for TEXT columns in non-STRICT tables must be
// NULL, TEXT, or BLOB.
unsafe { sqlite3VdbeAddOp4Int(*__slate_slot_591, 18 as i32, *__slate_slot_722, *__slate_slot_721, *__slate_slot_723, *__slate_slot_724) };
unsafe { sqlite3VdbeChangeP5(*__slate_slot_591, ((28 as i32) as i16) as u16) }; // NULL, TEXT, or BLOB
{
}
*__slate_slot_718 = unsafe { sqlite3MPrintf(*__slate_slot_589, (b"NUMERIC value in %s.%s\0".as_ptr() as *mut i8) as *const i8, unsafe { (*(*__slate_slot_705)).zName }, unsafe { (*unsafe { unsafe { (*(*__slate_slot_705)).aCol }.offset(*__slate_slot_689 as isize) }).zCnName }) };
unsafe { sqlite3VdbeAddOp4(*__slate_slot_591, 118 as i32, 0 as i32, 3 as i32, 0 as i32, *__slate_slot_718 as *const i8, -(7 as i32)) };
} else {
if !(*__slate_slot_713 != (0 as i32)) && ((unsafe { (*(*__slate_slot_719)).affinity }) as i32) >= (67 as i32) {
// (4) Datatype for numeric columns in non-STRICT tables must not
// be a TEXT value that can be converted to numeric.
unsafe { sqlite3VdbeAddOp4Int(*__slate_slot_591, 18 as i32, *__slate_slot_722, *__slate_slot_721, *__slate_slot_723, *__slate_slot_724) };
unsafe { sqlite3VdbeChangeP5(*__slate_slot_591, ((27 as i32) as i16) as u16) }; // NULL, INT, FLOAT, or BLOB
{
}
if *__slate_slot_722 >= (0 as i32) {
unsafe { sqlite3ExprCodeGetColumnOfTable(*__slate_slot_591, *__slate_slot_705, *__slate_slot_710, *__slate_slot_689, 3 as i32) };
}
unsafe { sqlite3VdbeAddOp4(*__slate_slot_591, 98 as i32, 3 as i32, 1 as i32, 0 as i32, (b"C\0".as_ptr() as *mut i8) as *const i8, -(1 as i32)) };
unsafe { sqlite3VdbeAddOp4Int(*__slate_slot_591, 18 as i32, -(1 as i32), *__slate_slot_721, 3 as i32, *__slate_slot_724) };
unsafe { sqlite3VdbeChangeP5(*__slate_slot_591, ((28 as i32) as i16) as u16) }; // NULL, TEXT, or BLOB
{
}
*__slate_slot_718 = unsafe { sqlite3MPrintf(*__slate_slot_589, (b"TEXT value in %s.%s\0".as_ptr() as *mut i8) as *const i8, unsafe { (*(*__slate_slot_705)).zName }, unsafe { (*unsafe { unsafe { (*(*__slate_slot_705)).aCol }.offset(*__slate_slot_689 as isize) }).zCnName }) };
unsafe { sqlite3VdbeAddOp4(*__slate_slot_591, 118 as i32, 0 as i32, 3 as i32, 0 as i32, *__slate_slot_718 as *const i8, -(7 as i32)) };
}
}
}
unsafe { sqlite3VdbeResolveLabel(*__slate_slot_591, *__slate_slot_720) };
integrityCheckResultRow(*__slate_slot_591);
unsafe { sqlite3VdbeResolveLabel(*__slate_slot_591, *__slate_slot_721) };
}
}
std::ptr::write(__slate_slot_1616, *__slate_slot_689);
std::ptr::write(__slate_slot_1617, *__slate_slot_1616 + (1 as i32));
*__slate_slot_689 = *__slate_slot_1617;
} else {
break;
}
}
// Verify CHECK constraints
if (unsafe { (*(*__slate_slot_705)).pCheck }) != std::ptr::null_mut::<ExprList>() && (unsafe { (*(*__slate_slot_589)).flags }) & (((512 as i32) as i64) as u64) == (((0 as i32) as i64) as u64) {
std::ptr::write(__slate_slot_730, unsafe { sqlite3ExprListDup(*__slate_slot_589, (unsafe { (*(*__slate_slot_705)).pCheck }) as *const ExprList, 0 as i32) });
if (((unsafe { (*(*__slate_slot_589)).mallocFailed }) as u32) as i32) == (0 as i32) {
std::ptr::write(__slate_slot_731, unsafe { sqlite3VdbeMakeLabel(pParse) });
std::ptr::write(__slate_slot_732, unsafe { sqlite3VdbeMakeLabel(pParse) });
unsafe {
(*pParse).iSelfTab = *__slate_slot_710 + (1 as i32);
}
*__slate_slot_734 = (unsafe { (*(*__slate_slot_730)).nExpr }) - (1 as i32);
loop {
if *__slate_slot_734 > (0 as i32) {
unsafe { sqlite3ExprIfFalse(pParse, unsafe { (*unsafe { unsafe { std::ptr::addr_of_mut!((*(*__slate_slot_730)).a) as *mut ExprList_item }.offset(*__slate_slot_734 as isize) }).pExpr }, *__slate_slot_731, 0 as i32) };
std::ptr::write(__slate_slot_1618, *__slate_slot_734);
std::ptr::write(__slate_slot_1619, *__slate_slot_1618 - (1 as i32));
*__slate_slot_734 = *__slate_slot_1619;
} else {
break;
}
}
unsafe { sqlite3ExprIfTrue(pParse, unsafe { (*unsafe { unsafe { std::ptr::addr_of_mut!((*(*__slate_slot_730)).a) as *mut ExprList_item }.offset((0 as i32) as isize) }).pExpr }, *__slate_slot_732, 16 as i32) };
unsafe { sqlite3VdbeResolveLabel(*__slate_slot_591, *__slate_slot_731) };
unsafe {
(*pParse).iSelfTab = 0 as i32;
}
*__slate_slot_733 = unsafe { sqlite3MPrintf(*__slate_slot_589, (b"CHECK constraint failed in %s\0".as_ptr() as *mut i8) as *const i8, unsafe { (*(*__slate_slot_705)).zName }) };
unsafe { sqlite3VdbeAddOp4(*__slate_slot_591, 118 as i32, 0 as i32, 3 as i32, 0 as i32, *__slate_slot_733 as *const i8, -(7 as i32)) };
integrityCheckResultRow(*__slate_slot_591);
unsafe { sqlite3VdbeResolveLabel(*__slate_slot_591, *__slate_slot_732) };
}
unsafe { sqlite3ExprListDelete(*__slate_slot_589, *__slate_slot_730) };
}
'__join_133: {
if !(*__slate_slot_693 != (0 as i32)) {
// Omit the remaining tests for quick_check
// Validate index entries for the current row
*__slate_slot_689 = 0 as i32;
std::ptr::write(__slate_slot_1620, unsafe { (*(*__slate_slot_705)).pIndex });
*__slate_slot_706 = *__slate_slot_1620;
loop {
if *__slate_slot_706 != std::ptr::null_mut::<Index>() {
std::ptr::write(__slate_slot_741, unsafe { sqlite3VdbeMakeLabel(pParse) });
if *__slate_slot_707 == *__slate_slot_706 {
} else {
*__slate_slot_712 = unsafe { sqlite3GenerateIndexKey(pParse, *__slate_slot_706, *__slate_slot_710, 0 as i32, 0 as i32, std::ptr::addr_of_mut!(*__slate_slot_736), *__slate_slot_708, *__slate_slot_712) };
*__slate_slot_708 = *__slate_slot_706;
unsafe { sqlite3VdbeAddOp2(*__slate_slot_591, 88 as i32, (8 as i32) + *__slate_slot_689, 1 as i32) }; // increment entry count
// Verify that an index entry exists for the current table row
unsafe { sqlite3VdbeAddOp4Int(*__slate_slot_591, 29 as i32, *__slate_slot_711 + *__slate_slot_689, *__slate_slot_741, *__slate_slot_712, ((unsafe { (*(*__slate_slot_706)).nColumn }) as u32) as i32) };
{
}
*__slate_slot_735 = unsafe { sqlite3VdbeAddOp3(*__slate_slot_591, 47 as i32, *__slate_slot_711 + *__slate_slot_689, *__slate_slot_741, *__slate_slot_712) };
{
}
unsafe { sqlite3VdbeChangeP4(*__slate_slot_591, -(1 as i32), *__slate_slot_706 as *const i8, -(6 as i32)) };
unsafe { sqlite3VdbeAddOp4(*__slate_slot_591, 118 as i32, 0 as i32, 3 as i32, 0 as i32, (unsafe { sqlite3MPrintf(*__slate_slot_589, (b"index %s stores an imprecise floating-point value for row \0".as_ptr() as *mut i8) as *const i8, unsafe { (*(*__slate_slot_706)).zName }) }) as *const i8, -(7 as i32)) };
unsafe { sqlite3VdbeAddOp3(*__slate_slot_591, 112 as i32, 7 as i32, 3 as i32, 3 as i32) };
integrityCheckResultRow(*__slate_slot_591);
unsafe { sqlite3VdbeAddOp2(*__slate_slot_591, 9 as i32, 0 as i32, *__slate_slot_741) };
unsafe { sqlite3VdbeJumpHere(*__slate_slot_591, *__slate_slot_735) };
unsafe { sqlite3VdbeLoadString(*__slate_slot_591, 3 as i32, (b"row \0".as_ptr() as *mut i8) as *const i8) };
unsafe { sqlite3VdbeAddOp3(*__slate_slot_591, 112 as i32, 7 as i32, 3 as i32, 3 as i32) };
unsafe { sqlite3VdbeLoadString(*__slate_slot_591, 4 as i32, (b" missing from index \0".as_ptr() as *mut i8) as *const i8) };
unsafe { sqlite3VdbeAddOp3(*__slate_slot_591, 112 as i32, 4 as i32, 3 as i32, 3 as i32) };
*__slate_slot_738 = unsafe { sqlite3VdbeLoadString(*__slate_slot_591, 4 as i32, (unsafe { (*(*__slate_slot_706)).zName }) as *const i8) };
unsafe { sqlite3VdbeAddOp3(*__slate_slot_591, 112 as i32, 4 as i32, 3 as i32, 3 as i32) };
*__slate_slot_737 = integrityCheckResultRow(*__slate_slot_591);
unsafe { sqlite3VdbeResolveLabel(*__slate_slot_591, *__slate_slot_741) };
// The OP_IdxRowid opcode is an optimized version of OP_Column
// that extracts the rowid off the end of the index record.
// But it only works correctly if index record does not have
// any extra bytes at the end.  Verify that this is the case.
if (unsafe { (*(*__slate_slot_705)).tabFlags }) & ((128 as i32) as u32) == ((0 as i32) as u32) {
unsafe { sqlite3VdbeAddOp2(*__slate_slot_591, 144 as i32, *__slate_slot_711 + *__slate_slot_689, 3 as i32) };
*__slate_slot_742 = unsafe { sqlite3VdbeAddOp3(*__slate_slot_591, 54 as i32, 3 as i32, 0 as i32, *__slate_slot_712 + (((unsafe { (*(*__slate_slot_706)).nColumn }) as u32) as i32) - (1 as i32)) };
{
}
unsafe { sqlite3VdbeLoadString(*__slate_slot_591, 3 as i32, (b"rowid not at end-of-record for row \0".as_ptr() as *mut i8) as *const i8) };
unsafe { sqlite3VdbeAddOp3(*__slate_slot_591, 112 as i32, 7 as i32, 3 as i32, 3 as i32) };
unsafe { sqlite3VdbeLoadString(*__slate_slot_591, 4 as i32, (b" of index \0".as_ptr() as *mut i8) as *const i8) };
unsafe { sqlite3VdbeGoto(*__slate_slot_591, *__slate_slot_738 - (1 as i32)) };
unsafe { sqlite3VdbeJumpHere(*__slate_slot_591, *__slate_slot_742) };
}
// Any indexed columns with non-BINARY collations must still hold
// the exact same text value as the table.
*__slate_slot_739 = 0 as i32;
*__slate_slot_740 = 0 as i32;
loop {
if *__slate_slot_740 < (((unsafe { (*(*__slate_slot_706)).nKeyCol }) as u32) as i32) {
if (unsafe { *unsafe { unsafe { (*(*__slate_slot_706)).azColl }.offset(*__slate_slot_740 as isize) } }) == unsafe { std::ptr::addr_of!(sqlite3StrBINARY) as *const i8 } {
} else {
if *__slate_slot_739 == (0 as i32) {
*__slate_slot_739 = unsafe { sqlite3VdbeMakeLabel(pParse) };
}
unsafe { sqlite3VdbeAddOp3(*__slate_slot_591, 96 as i32, *__slate_slot_711 + *__slate_slot_689, *__slate_slot_740, 3 as i32) };
unsafe { sqlite3VdbeAddOp3(*__slate_slot_591, 53 as i32, 3 as i32, *__slate_slot_739, *__slate_slot_712 + *__slate_slot_740) };
{
}
}
std::ptr::write(__slate_slot_1623, *__slate_slot_740);
std::ptr::write(__slate_slot_1624, *__slate_slot_1623 + (1 as i32));
*__slate_slot_740 = *__slate_slot_1624;
} else {
break;
}
}
if *__slate_slot_739 != (0 as i32) {
std::ptr::write(__slate_slot_743, unsafe { sqlite3VdbeAddOp0(*__slate_slot_591, 9 as i32) });
unsafe { sqlite3VdbeResolveLabel(*__slate_slot_591, *__slate_slot_739) };
unsafe { sqlite3VdbeLoadString(*__slate_slot_591, 3 as i32, (b"row \0".as_ptr() as *mut i8) as *const i8) };
unsafe { sqlite3VdbeAddOp3(*__slate_slot_591, 112 as i32, 7 as i32, 3 as i32, 3 as i32) };
unsafe { sqlite3VdbeLoadString(*__slate_slot_591, 4 as i32, (b" values differ from index \0".as_ptr() as *mut i8) as *const i8) };
unsafe { sqlite3VdbeGoto(*__slate_slot_591, *__slate_slot_738 - (1 as i32)) };
unsafe { sqlite3VdbeJumpHere(*__slate_slot_591, *__slate_slot_743) };
}
// For UNIQUE indexes, verify that only one entry exists with the
// current key.  The entry is unique if (1) any column is NULL
// or (2) the next entry has a different key
if (((unsafe { (*(*__slate_slot_706)).onError }) as u32) as i32) != (0 as i32) {
std::ptr::write(__slate_slot_744, unsafe { sqlite3VdbeMakeLabel(pParse) });
*__slate_slot_740 = 0 as i32;
loop {
if *__slate_slot_740 < (((unsafe { (*(*__slate_slot_706)).nKeyCol }) as u32) as i32) {
std::ptr::write(__slate_slot_746, (unsafe { *unsafe { unsafe { (*(*__slate_slot_706)).aiColumn }.offset(*__slate_slot_740 as isize) } }) as i32);
0 as i32;
if *__slate_slot_746 >= (0 as i32) && ((unsafe { (*unsafe { unsafe { (*(*__slate_slot_705)).aCol }.offset(*__slate_slot_746 as isize) }).__slate_bits_0.__get_notNull() }) as i32) != (0 as i32) {
} else {
unsafe { sqlite3VdbeAddOp2(*__slate_slot_591, 51 as i32, *__slate_slot_712 + *__slate_slot_740, *__slate_slot_744) };
{
}
}
std::ptr::write(__slate_slot_1625, *__slate_slot_740);
std::ptr::write(__slate_slot_1626, *__slate_slot_1625 + (1 as i32));
*__slate_slot_740 = *__slate_slot_1626;
} else {
break;
}
}
*__slate_slot_745 = unsafe { sqlite3VdbeAddOp1(*__slate_slot_591, 40 as i32, *__slate_slot_711 + *__slate_slot_689) };
{
}
unsafe { sqlite3VdbeGoto(*__slate_slot_591, *__slate_slot_744) };
unsafe { sqlite3VdbeJumpHere(*__slate_slot_591, *__slate_slot_745) };
unsafe { sqlite3VdbeAddOp4Int(*__slate_slot_591, 42 as i32, *__slate_slot_711 + *__slate_slot_689, *__slate_slot_744, *__slate_slot_712, ((unsafe { (*(*__slate_slot_706)).nKeyCol }) as u32) as i32) };
{
}
unsafe { sqlite3VdbeLoadString(*__slate_slot_591, 3 as i32, (b"non-unique entry in index \0".as_ptr() as *mut i8) as *const i8) };
unsafe { sqlite3VdbeGoto(*__slate_slot_591, *__slate_slot_738) };
unsafe { sqlite3VdbeResolveLabel(*__slate_slot_591, *__slate_slot_744) };
}
unsafe { sqlite3VdbeJumpHere(*__slate_slot_591, *__slate_slot_737) };
unsafe { sqlite3ResolvePartIdxLabel(pParse, *__slate_slot_736) };
}
*__slate_slot_706 = unsafe { (*(*__slate_slot_706)).pNext };
std::ptr::write(__slate_slot_1621, *__slate_slot_689);
std::ptr::write(__slate_slot_1622, *__slate_slot_1621 + (1 as i32));
*__slate_slot_689 = *__slate_slot_1622;
} else {
break '__join_133;
}
}
}
}
unsafe { sqlite3VdbeAddOp2(*__slate_slot_591, 40 as i32, *__slate_slot_710, *__slate_slot_709) };
{
}
unsafe { sqlite3VdbeJumpHere(*__slate_slot_591, *__slate_slot_709 - (1 as i32)) };
if *__slate_slot_707 != std::ptr::null_mut::<Index>() {
0 as i32;
unsafe { sqlite3ReleaseTempRange(pParse, *__slate_slot_714, ((unsafe { (*(*__slate_slot_707)).nKeyCol }) as u32) as i32) };
}
}
}
*__slate_slot_694 = unsafe { (*(*__slate_slot_694)).next };
} else {
break;
}
}
// Second pass to invoke the xIntegrity method on all virtual
// tables.
*__slate_slot_694 = unsafe { (*(*__slate_slot_695)).first };
loop {
if *__slate_slot_694 != std::ptr::null_mut::<HashElem>() {
'__join_117: {
std::ptr::write(__slate_slot_747, (unsafe { (*(*__slate_slot_694)).data }) as *mut Table);
if tableSkipIntegrityCheck(*__slate_slot_747 as *const Table, *__slate_slot_692 as *const Table) != (0 as i32) {
} else {
if (((unsafe { (*(*__slate_slot_747)).eTabType }) as u32) as i32) == (0 as i32) {
} else {
if !((((unsafe { (*(*__slate_slot_747)).eTabType }) as u32) as i32) == (1 as i32)) {
} else {
if ((unsafe { (*(*__slate_slot_747)).nCol }) as i32) <= (0 as i32) {
std::ptr::write(__slate_slot_750, (unsafe { *unsafe { unsafe { (*(*__slate_slot_747)).u.vtab.azArg }.offset((0 as i32) as isize) } }) as *const i8);
if (unsafe { sqlite3HashFind((unsafe { std::ptr::addr_of_mut!((*(*__slate_slot_589)).aModule) }) as *const Hash, *__slate_slot_750) }) == std::ptr::null_mut::<()>() {
break '__join_117;
}
}
unsafe { sqlite3ViewGetColumnNames(pParse, *__slate_slot_747) };
if (unsafe { (*(*__slate_slot_747)).u.vtab.p }) == std::ptr::null_mut::<VTable>() {
} else {
*__slate_slot_748 = unsafe { (*unsafe { (*(*__slate_slot_747)).u.vtab.p }).pVtab };
if *__slate_slot_748 == std::ptr::null_mut::<sqlite3_vtab>() {
} else {
if (unsafe { (*(*__slate_slot_748)).pModule }) == std::ptr::null::<sqlite3_module>() {
} else {
if (unsafe { (*unsafe { (*(*__slate_slot_748)).pModule }).iVersion }) < (4 as i32) {
} else {
if (unsafe { (*unsafe { (*(*__slate_slot_748)).pModule }).xIntegrity }) == None {
} else {
unsafe { sqlite3VdbeAddOp3(*__slate_slot_591, 176 as i32, *__slate_slot_688, 3 as i32, *__slate_slot_693) };
std::ptr::write(__slate_slot_1627, *__slate_slot_747);
std::ptr::write(__slate_slot_1628, unsafe { (*(*__slate_slot_1627)).nTabRef });
std::ptr::write(__slate_slot_1629, (*__slate_slot_1628).wrapping_add((1 as i32) as u32));
unsafe {
(*(*__slate_slot_1627)).nTabRef = *__slate_slot_1629;
}
unsafe { sqlite3VdbeAppendP4(*__slate_slot_591, *__slate_slot_747 as *mut (), -(15 as i32)) };
*__slate_slot_749 = unsafe { sqlite3VdbeAddOp1(*__slate_slot_591, 51 as i32, 3 as i32) };
{
}
integrityCheckResultRow(*__slate_slot_591);
unsafe { sqlite3VdbeJumpHere(*__slate_slot_591, *__slate_slot_749) };
}
}
}
}
}
}
}
}
}
*__slate_slot_694 = unsafe { (*(*__slate_slot_694)).next };
} else {
break '__join_115;
}
}
}
}
}
}
}
std::ptr::write(__slate_slot_1584, *__slate_slot_688);
std::ptr::write(__slate_slot_1585, *__slate_slot_1584 + (1 as i32));
*__slate_slot_688 = *__slate_slot_1585;
} else {
break;
}
}
// 0
// 1
// 2
// 3
// 4
// 5
// 6
*__slate_slot_753 = unsafe { sqlite3VdbeAddOpList(*__slate_slot_591, (((28 as u64) / (4 as u64)) as u32) as i32, unsafe { std::ptr::addr_of!(endCode.0) as *const VdbeOpList }, unsafe { iLn_751 }) };
if *__slate_slot_753 != std::ptr::null_mut::<VdbeOp>() {
unsafe {
(*unsafe { (*__slate_slot_753).offset((0 as i32) as isize) }).p2 = (1 as i32) - *__slate_slot_691;
}
unsafe {
(*unsafe { (*__slate_slot_753).offset((2 as i32) as isize) }).p4type = -(1 as i32) as i8;
}
unsafe {
(*unsafe { (*__slate_slot_753).offset((2 as i32) as isize) }).p4.z = b"ok\0".as_ptr() as *mut i8;
}
unsafe {
(*unsafe { (*__slate_slot_753).offset((5 as i32) as isize) }).p4type = -(1 as i32) as i8;
}
unsafe {
(*unsafe { (*__slate_slot_753).offset((5 as i32) as isize) }).p4.z = (unsafe { sqlite3ErrStr(11 as i32) }) as *mut i8;
}
}
unsafe { sqlite3VdbeChangeP3(*__slate_slot_591, 0 as i32, (unsafe { sqlite3VdbeCurrentAddr(*__slate_slot_591) }) - (2 as i32)) };
} else {
if __t0 == (14 as i32) {
// Must be element [1]
// Must be element [2]
// Must be element [3]
// SQLITE_UTF16NATIVE
// SQLITE_UTF16NATIVE
if !(*__slate_slot_583 != std::ptr::null_mut::<i8>()) {
// "PRAGMA encoding"
if (unsafe { sqlite3ReadSchema(pParse) }) != (0 as i32) {
break '__join_0;
} else {
0 as i32;
0 as i32;
0 as i32;
returnSingleText(*__slate_slot_591, (unsafe { (*unsafe { unsafe { std::ptr::addr_of!(encnames.0) as *const EncName }.offset((((unsafe { (*unsafe { (*pParse).db }).enc }) as u32) as i32) as isize) }).zName }) as *const i8);
}
} else {
// "PRAGMA encoding = XXX"
// Only change the value of sqlite.enc if the database handle is not
// initialized. If the main database exists, the new sqlite.enc value
// will be overwritten when the schema is next loaded. If it does not
// already exists, it will be created to use the new encoding value.
if (unsafe { (*(*__slate_slot_589)).mDbFlags }) & ((64 as i32) as u32) == ((0 as i32) as u32) {
*__slate_slot_756 = unsafe { unsafe { std::ptr::addr_of!(encnames.0) as *const EncName }.offset((0 as i32) as isize) };
'__join_103: {
loop {
if (unsafe { (*(*__slate_slot_756)).zName }) != std::ptr::null_mut::<i8>() {
if (0 as i32) == unsafe { sqlite3StrICmp(*__slate_slot_583 as *const i8, (unsafe { (*(*__slate_slot_756)).zName }) as *const i8) } {
break;
} else {
std::ptr::write(__slate_slot_1630, *__slate_slot_756);
std::ptr::write(__slate_slot_1631, unsafe { (*__slate_slot_1630).offset((1 as i32) as isize) });
*__slate_slot_756 = *__slate_slot_1631;
}
} else {
break '__join_103;
}
}
std::ptr::write(__slate_slot_757, ((if (unsafe { (*(*__slate_slot_756)).enc }) != (0 as u8) { ((unsafe { (*(*__slate_slot_756)).enc }) as u32) as i32 } else { 2 as i32 }) as i8) as u8);
unsafe {
(*unsafe { (*unsafe { unsafe { (*(*__slate_slot_589)).aDb }.offset((0 as i32) as isize) }).pSchema }).enc = *__slate_slot_757;
}
unsafe { sqlite3SetTextEncoding(*__slate_slot_589, *__slate_slot_757) };
}
if !((unsafe { (*(*__slate_slot_756)).zName }) != std::ptr::null_mut::<i8>()) {
unsafe { sqlite3ErrorMsg(pParse, (b"unsupported encoding: %s\0".as_ptr() as *mut i8) as *const i8, *__slate_slot_583) };
}
}
}
} else {
if __t0 == (2 as i32) {
std::ptr::write(__slate_slot_758, ((unsafe { (*(*__slate_slot_592)).iArg }) as u32) as i32);
unsafe { sqlite3VdbeUsesBtree(*__slate_slot_591, *__slate_slot_587) };
if *__slate_slot_583 != std::ptr::null_mut::<i8>() && (((unsafe { (*(*__slate_slot_592)).mPragFlg }) as u32) as i32) & (8 as i32) == (0 as i32) {
// Write the specified cookie value
// 0
// 1
{
}
*__slate_slot_760 = unsafe { sqlite3VdbeAddOpList(*__slate_slot_591, (((8 as u64) / (4 as u64)) as u32) as i32, unsafe { std::ptr::addr_of!(setCookie) as *const VdbeOpList }, 0 as i32) };
if (0 as i32) != (0 as i32) {
} else {
unsafe {
(*unsafe { (*__slate_slot_760).offset((0 as i32) as isize) }).p1 = *__slate_slot_587;
}
unsafe {
(*unsafe { (*__slate_slot_760).offset((1 as i32) as isize) }).p1 = *__slate_slot_587;
}
unsafe {
(*unsafe { (*__slate_slot_760).offset((1 as i32) as isize) }).p2 = *__slate_slot_758;
}
unsafe {
(*unsafe { (*__slate_slot_760).offset((1 as i32) as isize) }).p3 = unsafe { sqlite3Atoi(*__slate_slot_583 as *const i8) };
}
unsafe {
(*unsafe { (*__slate_slot_760).offset((1 as i32) as isize) }).p5 = ((1 as i32) as i16) as u16;
}
if *__slate_slot_758 == (1 as i32) && (unsafe { (*(*__slate_slot_589)).flags }) & (((268435456 as i32) as i64) as u64) != (((0 as i32) as i64) as u64) {
// Do not allow the use of PRAGMA schema_version=VALUE in defensive
// mode.  Change the OP_SetCookie opcode into a no-op.
unsafe {
(*unsafe { (*__slate_slot_760).offset((1 as i32) as isize) }).opcode = ((189 as i32) as i8) as u8;
}
}
}
} else {
// Read the specified cookie value
// 0
// 1
{
}
*__slate_slot_762 = unsafe { sqlite3VdbeAddOpList(*__slate_slot_591, (((12 as u64) / (4 as u64)) as u32) as i32, unsafe { std::ptr::addr_of!(readCookie) as *const VdbeOpList }, 0 as i32) };
if (0 as i32) != (0 as i32) {
} else {
unsafe {
(*unsafe { (*__slate_slot_762).offset((0 as i32) as isize) }).p1 = *__slate_slot_587;
}
unsafe {
(*unsafe { (*__slate_slot_762).offset((1 as i32) as isize) }).p1 = *__slate_slot_587;
}
unsafe {
(*unsafe { (*__slate_slot_762).offset((1 as i32) as isize) }).p3 = *__slate_slot_758;
}
unsafe { sqlite3VdbeReusable(*__slate_slot_591) };
}
}
} else {
if __t0 == (10 as i32) {
std::ptr::write(__slate_slot_763, 0 as i32);
unsafe {
(*pParse).nMem = 1 as i32;
}
loop {
std::ptr::write(__slate_slot_1632, *__slate_slot_763);
std::ptr::write(__slate_slot_1633, *__slate_slot_1632 + (1 as i32));
*__slate_slot_763 = *__slate_slot_1633;
std::ptr::write(__slate_slot_1634, unsafe { sqlite3_compileoption_get(*__slate_slot_1632) });
*__slate_slot_764 = *__slate_slot_1634;
if *__slate_slot_1634 != std::ptr::null::<i8>() {
unsafe { sqlite3VdbeLoadString(*__slate_slot_591, 1 as i32, *__slate_slot_764) };
unsafe { sqlite3VdbeAddOp2(*__slate_slot_591, 86 as i32, 1 as i32, 1 as i32) };
} else {
break;
}
}
unsafe { sqlite3VdbeReusable(*__slate_slot_591) };
} else {
if __t0 == (43 as i32) {
std::ptr::write(__slate_slot_765, if (unsafe { (*pId2).z }) != std::ptr::null::<i8>() { *__slate_slot_587 } else { (10 as i32) + (2 as i32) });
std::ptr::write(__slate_slot_766, 0 as i32);
if *__slate_slot_583 != std::ptr::null_mut::<i8>() {
if (unsafe { sqlite3StrICmp(*__slate_slot_583 as *const i8, (b"full\0".as_ptr() as *mut i8) as *const i8) }) == (0 as i32) {
*__slate_slot_766 = 1 as i32;
} else {
if (unsafe { sqlite3StrICmp(*__slate_slot_583 as *const i8, (b"restart\0".as_ptr() as *mut i8) as *const i8) }) == (0 as i32) {
*__slate_slot_766 = 2 as i32;
} else {
if (unsafe { sqlite3StrICmp(*__slate_slot_583 as *const i8, (b"truncate\0".as_ptr() as *mut i8) as *const i8) }) == (0 as i32) {
*__slate_slot_766 = 3 as i32;
} else {
if (unsafe { sqlite3StrICmp(*__slate_slot_583 as *const i8, (b"noop\0".as_ptr() as *mut i8) as *const i8) }) == (0 as i32) {
*__slate_slot_766 = -(1 as i32);
}
}
}
}
}
unsafe {
(*pParse).nMem = 3 as i32;
}
unsafe { sqlite3VdbeAddOp3(*__slate_slot_591, 3 as i32, *__slate_slot_765, *__slate_slot_766, 1 as i32) };
unsafe { sqlite3VdbeAddOp2(*__slate_slot_591, 86 as i32, 1 as i32, 3 as i32) };
} else {
if __t0 == (42 as i32) {
if *__slate_slot_583 != std::ptr::null_mut::<i8>() {
unsafe { sqlite3_wal_autocheckpoint(*__slate_slot_589, unsafe { sqlite3Atoi(*__slate_slot_583 as *const i8) }) };
}
returnSingleInt(*__slate_slot_591, (if (unsafe { (*(*__slate_slot_589)).xWalCallback }) == unsafe { std::mem::transmute::<*const (), Option<unsafe extern "C-unwind" fn(*mut (), *mut sqlite3, *const i8, i32) -> i32>>(sqlite3WalDefaultHook as *const ()) } { ((unsafe { (*(*__slate_slot_589)).pWalArg }) as i64) as i32 } else { 0 as i32 }) as i64);
} else {
if __t0 == (34 as i32) {
unsafe { sqlite3_db_release_memory(*__slate_slot_589) };
} else {
if __t0 == (30 as i32) {
std::ptr::write(__slate_slot_777, 0 as i32);
std::ptr::write(__slate_slot_778, 0 as i32);
if *__slate_slot_583 != std::ptr::null_mut::<i8>() {
*__slate_slot_775 = (unsafe { sqlite3Atoi(*__slate_slot_583 as *const i8) }) as u32;
if *__slate_slot_775 & ((2 as i32) as u32) == ((0 as i32) as u32) {
break '__join_2;
}
} else {
*__slate_slot_775 = (65534 as i32) as u32;
}
if *__slate_slot_775 & ((16 as i32) as u32) == ((0 as i32) as u32) {
*__slate_slot_776 = 0 as i32;
} else {
if (unsafe { (*(*__slate_slot_589)).nAnalysisLimit }) > (0 as i32) && (unsafe { (*(*__slate_slot_589)).nAnalysisLimit }) < (2000 as i32) {
*__slate_slot_776 = 0 as i32;
} else {
*__slate_slot_776 = 2000 as i32;
}
}
std::ptr::write(__slate_slot_1635, pParse);
std::ptr::write(__slate_slot_1636, unsafe { (*(*__slate_slot_1635)).nTab });
std::ptr::write(__slate_slot_1637, *__slate_slot_1636 + (1 as i32));
unsafe {
(*(*__slate_slot_1635)).nTab = *__slate_slot_1637;
}
*__slate_slot_768 = *__slate_slot_1636;
*__slate_slot_767 = if *__slate_slot_584 != std::ptr::null::<i8>() { *__slate_slot_587 } else { (unsafe { (*(*__slate_slot_589)).nDb }) - (1 as i32) };
loop {
if *__slate_slot_587 <= *__slate_slot_767 {
'__join_40: {
if *__slate_slot_587 == (1 as i32) {
} else {
unsafe { sqlite3CodeVerifySchema(pParse, *__slate_slot_587) };
*__slate_slot_770 = unsafe { (*unsafe { unsafe { (*(*__slate_slot_589)).aDb }.offset(*__slate_slot_587 as isize) }).pSchema };
*__slate_slot_769 = unsafe { (*unsafe { std::ptr::addr_of_mut!((*(*__slate_slot_770)).tblHash) }).first };
loop {
if *__slate_slot_769 != std::ptr::null_mut::<HashElem>() {
'__join_42: {
*__slate_slot_771 = (unsafe { (*(*__slate_slot_769)).data }) as *mut Table;
// This only works for ordinary tables
if !((((unsafe { (*(*__slate_slot_771)).eTabType }) as u32) as i32) == (0 as i32)) {
} else {
// Do not scan system tables
if (0 as i32) == unsafe { sqlite3_strnicmp((unsafe { (*(*__slate_slot_771)).zName }) as *const i8, (b"sqlite_\0".as_ptr() as *mut i8) as *const i8, 7 as i32) } {
} else {
// Find the size of the table as last recorded in sqlite_stat1.
// If any index is unanalyzed, then the threshold is -1 to
// indicate a new, unanalyzed index
*__slate_slot_773 = unsafe { (*(*__slate_slot_771)).nRowLogEst };
*__slate_slot_779 = 0 as i32;
*__slate_slot_772 = unsafe { (*(*__slate_slot_771)).pIndex };
loop {
if *__slate_slot_772 != std::ptr::null_mut::<Index>() {
std::ptr::write(__slate_slot_1640, *__slate_slot_779);
std::ptr::write(__slate_slot_1641, *__slate_slot_1640 + (1 as i32));
*__slate_slot_779 = *__slate_slot_1641;
if !(((unsafe { (*(*__slate_slot_772)).__slate_bits_0.__get_hasStat1() }) as i32) != (0 as i32)) {
*__slate_slot_773 = -(1 as i32) as i16; // Always analyze if any index lacks statistics
}
*__slate_slot_772 = unsafe { (*(*__slate_slot_772)).pNext };
} else {
break;
}
}
// If table pTab has not been used in a way that would benefit from
// having analysis statistics during the current session, then skip it,
// unless the 0x10000 MASK bit is set.
if (unsafe { (*(*__slate_slot_771)).tabFlags }) & ((256 as i32) as u32) != ((0 as i32) as u32) {
// Check for size change if stat1 has been used for a query
} else {
if *__slate_slot_775 & ((65536 as i32) as u32) != (0 as u32) {
// Check for size change if 0x10000 is set
} else {
if (unsafe { (*(*__slate_slot_771)).pIndex }) != std::ptr::null_mut::<Index>() && (*__slate_slot_773 as i32) < (0 as i32) {
// Do analysis if unanalyzed indexes exists
} else {
// Otherwise, we can skip this table
break '__join_42;
}
}
}
std::ptr::write(__slate_slot_1642, *__slate_slot_777);
std::ptr::write(__slate_slot_1643, *__slate_slot_1642 + (1 as i32));
*__slate_slot_777 = *__slate_slot_1643;
if *__slate_slot_777 == (2 as i32) {
// If ANALYZE might be invoked two or more times, hold a write
// transaction for efficiency
unsafe { sqlite3BeginWriteOperation(pParse, 0 as i32, *__slate_slot_587) };
}
std::ptr::write(__slate_slot_1644, *__slate_slot_778);
std::ptr::write(__slate_slot_1645, *__slate_slot_1644 + (*__slate_slot_779 + (1 as i32)));
*__slate_slot_778 = *__slate_slot_1645;
// Reanalyze if the table is 10 times larger or smaller than
// the last analysis.  Unconditional reanalysis if there are
// unanalyzed indexes.
unsafe { sqlite3OpenTable(pParse, *__slate_slot_768, *__slate_slot_587, *__slate_slot_771, 114 as i32) };
if (*__slate_slot_773 as i32) >= (0 as i32) {
std::ptr::write(__slate_slot_780, (33 as i32) as i16);
unsafe { sqlite3VdbeAddOp4Int(*__slate_slot_591, 33 as i32, *__slate_slot_768, (((unsafe { sqlite3VdbeCurrentAddr(*__slate_slot_591) }) + (2 as i32)) as u32).wrapping_add(*__slate_slot_775 & ((1 as i32) as u32)) as i32, if (*__slate_slot_773 as i32) >= (*__slate_slot_780 as i32) { (*__slate_slot_773 as i32) - (*__slate_slot_780 as i32) } else { -(1 as i32) }, (*__slate_slot_773 as i32) + (*__slate_slot_780 as i32)) };
{
}
} else {
unsafe { sqlite3VdbeAddOp2(*__slate_slot_591, 36 as i32, *__slate_slot_768, (((unsafe { sqlite3VdbeCurrentAddr(*__slate_slot_591) }) + (2 as i32)) as u32).wrapping_add(*__slate_slot_775 & ((1 as i32) as u32)) as i32) };
{
}
}
*__slate_slot_774 = unsafe { sqlite3MPrintf(*__slate_slot_589, (b"ANALYZE \"%w\".\"%w\"\0".as_ptr() as *mut i8) as *const i8, unsafe { (*unsafe { unsafe { (*(*__slate_slot_589)).aDb }.offset(*__slate_slot_587 as isize) }).zDbSName }, unsafe { (*(*__slate_slot_771)).zName }) };
if *__slate_slot_775 & ((1 as i32) as u32) != (0 as u32) {
std::ptr::write(__slate_slot_781, unsafe { sqlite3GetTempReg(pParse) });
unsafe { sqlite3VdbeAddOp4(*__slate_slot_591, 118 as i32, 0 as i32, *__slate_slot_781, 0 as i32, *__slate_slot_774 as *const i8, -(7 as i32)) };
unsafe { sqlite3VdbeAddOp2(*__slate_slot_591, 86 as i32, *__slate_slot_781, 1 as i32) };
} else {
unsafe { sqlite3VdbeAddOp4(*__slate_slot_591, 150 as i32, if *__slate_slot_776 != (0 as i32) { 2 as i32 } else { 0 as i32 }, *__slate_slot_776, 0 as i32, *__slate_slot_774 as *const i8, -(7 as i32)) };
}
}
}
}
*__slate_slot_769 = unsafe { (*(*__slate_slot_769)).next };
} else {
break '__join_40;
}
}
}
}
std::ptr::write(__slate_slot_1638, *__slate_slot_587);
std::ptr::write(__slate_slot_1639, *__slate_slot_1638 + (1 as i32));
*__slate_slot_587 = *__slate_slot_1639;
} else {
break;
}
}
unsafe { sqlite3VdbeAddOp0(*__slate_slot_591, 168 as i32) };
// In a schema with a large number of tables and indexes, scale back
// the analysis_limit to avoid excess run-time in the worst case.
if !((unsafe { (*(*__slate_slot_589)).mallocFailed }) != (0 as u8)) && *__slate_slot_776 > (0 as i32) && *__slate_slot_778 > (100 as i32) {
*__slate_slot_776 = (100 as i32) * *__slate_slot_776 / *__slate_slot_778;
if *__slate_slot_776 < (100 as i32) {
*__slate_slot_776 = 100 as i32;
}
*__slate_slot_784 = unsafe { sqlite3VdbeGetOp(*__slate_slot_591, 0 as i32) };
*__slate_slot_783 = unsafe { sqlite3VdbeCurrentAddr(*__slate_slot_591) };
*__slate_slot_782 = 0 as i32;
loop {
if *__slate_slot_782 < *__slate_slot_783 {
if (((unsafe { (*unsafe { (*__slate_slot_784).offset(*__slate_slot_782 as isize) }).opcode }) as u32) as i32) == (150 as i32) {
unsafe {
(*unsafe { (*__slate_slot_784).offset(*__slate_slot_782 as isize) }).p2 = *__slate_slot_776;
}
}
std::ptr::write(__slate_slot_1646, *__slate_slot_782);
std::ptr::write(__slate_slot_1647, *__slate_slot_1646 + (1 as i32));
*__slate_slot_782 = *__slate_slot_1647;
} else {
break '__join_2;
}
}
}
} else {
if __t0 == (35 as i32) {
'__join_24: {
if *__slate_slot_583 != std::ptr::null_mut::<i8>() {
*__slate_slot_1648 = (unsafe { sqlite3DecOrHexToI64(*__slate_slot_583 as *const i8, std::ptr::addr_of_mut!(*__slate_slot_785)) }) == (0 as i32);
} else {
*__slate_slot_1648 = false as bool;
}
}
if *__slate_slot_1648 {
unsafe { sqlite3_soft_heap_limit64(*__slate_slot_785) };
}
returnSingleInt(*__slate_slot_591, unsafe { sqlite3_soft_heap_limit64(-(1 as i32) as i64) });
} else {
if __t0 == (18 as i32) {
if *__slate_slot_583 != std::ptr::null_mut::<i8>() {
*__slate_slot_1649 = (unsafe { sqlite3DecOrHexToI64(*__slate_slot_583 as *const i8, std::ptr::addr_of_mut!(*__slate_slot_786)) }) == (0 as i32);
} else {
*__slate_slot_1649 = false as bool;
}
if *__slate_slot_1649 {
std::ptr::write(__slate_slot_787, unsafe { sqlite3_hard_heap_limit64(-(1 as i32) as i64) });
if *__slate_slot_786 > ((0 as i32) as i64) && (*__slate_slot_787 == ((0 as i32) as i64) || *__slate_slot_787 > *__slate_slot_786) {
unsafe { sqlite3_hard_heap_limit64(*__slate_slot_786) };
}
}
returnSingleInt(*__slate_slot_591, unsafe { sqlite3_hard_heap_limit64(-(1 as i32) as i64) });
} else {
if __t0 == (41 as i32) {
if *__slate_slot_583 != std::ptr::null_mut::<i8>() {
*__slate_slot_1650 = (unsafe { sqlite3DecOrHexToI64(*__slate_slot_583 as *const i8, std::ptr::addr_of_mut!(*__slate_slot_788)) }) == (0 as i32);
} else {
*__slate_slot_1650 = false as bool;
}
if *__slate_slot_1650 && *__slate_slot_788 >= ((0 as i32) as i64) {
unsafe { sqlite3_limit(*__slate_slot_589, 11 as i32, (*__slate_slot_788 & ((2147483647 as i32) as i64)) as i32) };
}
returnSingleInt(*__slate_slot_591, (unsafe { sqlite3_limit(*__slate_slot_589, 11 as i32, -(1 as i32)) }) as i64);
} else {
if __t0 == (1 as i32) {
if *__slate_slot_583 != std::ptr::null_mut::<i8>() {
*__slate_slot_1651 = (unsafe { sqlite3DecOrHexToI64(*__slate_slot_583 as *const i8, std::ptr::addr_of_mut!(*__slate_slot_789)) }) == (0 as i32);
} else {
*__slate_slot_1651 = false as bool;
}
if *__slate_slot_1651 && *__slate_slot_789 >= ((0 as i32) as i64) {
unsafe {
(*(*__slate_slot_589)).nAnalysisLimit = (*__slate_slot_789 & ((2147483647 as i32) as i64)) as i32;
}
}
// IMP: R-40975-20399
returnSingleInt(*__slate_slot_591, (unsafe { (*(*__slate_slot_589)).nAnalysisLimit }) as i64); // IMP: R-57594-65522
} else {
0 as i32;
if *__slate_slot_583 != std::ptr::null_mut::<i8>() {
unsafe { sqlite3_busy_timeout(*__slate_slot_589, unsafe { sqlite3Atoi(*__slate_slot_583 as *const i8) }) };
}
returnSingleInt(*__slate_slot_591, (unsafe { (*(*__slate_slot_589)).busyTimeout }) as i64);
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
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                            // End of the PRAGMA switch
                                            // The following block is a no-op unless SQLITE_DEBUG is defined. Its only
                                            // purpose is to execute assert() statements to verify that if the
                                            // PragFlg_NoColumns1 flag is set and the caller specified an argument
                                            // to the PRAGMA, the implementation has not added any OP_ResultRow
                                            // instructions to the VM.
                                            if (((unsafe { (*(*__slate_slot_592)).mPragFlg })
                                                as u32)
                                                as i32)
                                                & (4 as i32)
                                                != (0 as i32)
                                                && *__slate_slot_583 != std::ptr::null_mut::<i8>()
                                            {
                                                {}
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        unsafe { sqlite3DbFree(*__slate_slot_589, *__slate_slot_582 as *mut ()) };
                        unsafe { sqlite3DbFree(*__slate_slot_589, *__slate_slot_583 as *mut ()) };
                    }
                }
            }
        }
    }
    //  PRAGMA [schema.]default_cache_size
    //  PRAGMA [schema.]default_cache_size=N
    //
    // The first form reports the current persistent setting for the
    // page cache size.  The value returned is the maximum number of
    // pages in the page cache.  The second form sets both the current
    // page cache size value and the persistent page cache size value
    // stored in the database file.
    //
    // Older versions of SQLite would set the default cache size to a
    // negative number to indicate synchronous=OFF.  These days, synchronous
    // is always on by default regardless of the sign of the default cache
    // size.  But continue to take the absolute value of the default cache
    // size of historical compatibility.
    //  PRAGMA [schema.]page_size
    //  PRAGMA [schema.]page_size=N
    //
    // The first form reports the current setting for the
    // database page size in bytes.  The second form sets the
    // database page size value.  The value can only be set if
    // the database has not yet been created.
    //  PRAGMA [schema.]secure_delete
    //  PRAGMA [schema.]secure_delete=ON/OFF/FAST
    //
    // The first form reports the current setting for the
    // secure_delete flag.  The second form changes the secure_delete
    // flag setting and reports the new value.
    //  PRAGMA [schema.]max_page_count
    //  PRAGMA [schema.]max_page_count=N
    //
    // The first form reports the current setting for the
    // maximum number of pages in the database file.  The
    // second form attempts to change this setting.  Both
    // forms return the current setting.
    //
    // The absolute value of N is used.  This is undocumented and might
    // change.  The only purpose is to provide an easy way to test
    // the sqlite3AbsInt32() function.
    //
    //  PRAGMA [schema.]page_count
    //
    // Return the number of pages in the specified database.
    // PRAGMA [schema.]locking_mode
    // PRAGMA [schema.]locking_mode = (normal|exclusive)
    // PRAGMA [schema.]journal_mode
    // PRAGMA [schema.]journal_mode =
    //                     (delete|persist|off|truncate|memory|wal|off)
    //  PRAGMA [schema.]journal_size_limit
    //  PRAGMA [schema.]journal_size_limit=N
    //
    // Get or set the size limit on rollback journal files.
    //  PRAGMA [schema.]auto_vacuum
    //  PRAGMA [schema.]auto_vacuum=N
    //
    // Get or set the value of the database 'auto-vacuum' parameter.
    // The value is one of:  0 NONE 1 FULL 2 INCREMENTAL
    //  PRAGMA [schema.]incremental_vacuum(N)
    //
    // Do N steps of incremental vacuuming on a database.
    //  PRAGMA [schema.]cache_size
    //  PRAGMA [schema.]cache_size=N
    //
    // The first form reports the current local setting for the
    // page cache size. The second form sets the local
    // page cache size value.  If N is positive then that is the
    // number of pages in the cache.  If N is negative, then the
    // number of pages is adjusted so that the cache uses -N kibibytes
    // of memory.
    //  PRAGMA [schema.]cache_spill
    //  PRAGMA cache_spill=BOOLEAN
    //  PRAGMA [schema.]cache_spill=N
    //
    // The first form reports the current local setting for the
    // page cache spill size. The second form turns cache spill on
    // or off.  When turning cache spill on, the size is set to the
    // current cache_size.  The third form sets a spill size that
    // may be different form the cache size.
    // If N is positive then that is the
    // number of pages in the cache.  If N is negative, then the
    // number of pages is adjusted so that the cache uses -N kibibytes
    // of memory.
    //
    // If the number of cache_spill pages is less then the number of
    // cache_size pages, no spilling occurs until the page count exceeds
    // the number of cache_size pages.
    //
    // The cache_spill=BOOLEAN setting applies to all attached schemas,
    // not just the schema specified.
    //  PRAGMA [schema.]mmap_size(N)
    //
    // Used to set mapping size limit. The mapping size limit is
    // used to limit the aggregate size of all memory mapped regions of the
    // database file. If this parameter is set to zero, then memory mapping
    // is not used at all.  If N is negative, then the default memory map
    // limit determined by sqlite3_config(SQLITE_CONFIG_MMAP_SIZE) is set.
    // The parameter N is measured in bytes.
    //
    // This value is advisory.  The underlying VFS is free to memory map
    // as little or as much as it wants.  Except, if N is set to 0 then the
    // upper layers will never invoke the xFetch interfaces to the VFS.
    //   PRAGMA temp_store
    //   PRAGMA temp_store = "default"|"memory"|"file"
    //
    // Return or set the local value of the temp_store flag.  Changing
    // the local value does not make changes to the disk file and the default
    // value will be restored the next time the database is opened.
    //
    // Note that it is possible for the library compile-time options to
    // override this setting
    //   PRAGMA temp_store_directory
    //   PRAGMA temp_store_directory = ""|"directory_name"
    //
    // Return or set the local value of the temp_store_directory flag.  Changing
    // the value sets a specific directory to be used for temporary files.
    // Setting to a null string reverts to the default temporary directory search.
    // If temporary directory is changed, then invalidateTempStorage.
    //   PRAGMA [schema.]synchronous
    //   PRAGMA [schema.]synchronous=OFF|ON|NORMAL|FULL|EXTRA
    //
    // Return or set the local value of the synchronous flag.  Changing
    // the local value does not make changes to the disk file and the
    // default value will be restored the next time the database is
    // opened.
    //   PRAGMA table_info(<table>)
    //
    // Return a single row for each column of the named table. The columns of
    // the returned data set are:
    //
    // cid:        Column id (numbered from left to right, starting at 0)
    // name:       Column name
    // type:       Column declaration type.
    // notnull:    True if 'NOT NULL' is part of column declaration
    // dflt_value: The default value for the column, if any.
    // pk:         Non-zero for PK fields.
    //   PRAGMA table_list
    //
    // Return a single row for each table, virtual table, or view in the
    // entire schema.
    //
    // schema:     Name of attached database hold this table
    // name:       Name of the table itself
    // type:       "table", "view", "virtual", "shadow"
    // ncol:       Number of columns
    // wr:         True for a WITHOUT ROWID table
    // strict:     True for a STRICT table
    // Reinstall the LIKE and GLOB functions.  The variant of LIKE
    // used will be case sensitive or not depending on the RHS.
    //    PRAGMA integrity_check
    //    PRAGMA integrity_check(N)
    //    PRAGMA quick_check
    //    PRAGMA quick_check(N)
    //
    // Verify the integrity of the database.
    //
    // The "quick_check" is reduced version of
    // integrity_check designed to detect most database corruption
    // without the overhead of cross-checking indexes.  Quick_check
    // is linear time whereas integrity_check is O(NlogN).
    //
    // The maximum number of errors is 100 by default.  A different default
    // can be specified using a numeric parameter N.
    //
    // Or, the parameter N can be the name of a table.  In that case, only
    // the one table named is verified.  The freelist is only verified if
    // the named table is "sqlite_schema" (or one of its aliases).
    //
    // All schemas are checked by default.  To check just a single
    // schema, use the form:
    //
    //      PRAGMA schema.integrity_check;
    //   PRAGMA encoding
    //   PRAGMA encoding = "utf-8"|"utf-16"|"utf-16le"|"utf-16be"
    //
    // In its first form, this pragma returns the encoding of the main
    // database. If the database is not initialized, it is initialized now.
    //
    // The second form of this pragma is a no-op if the main database file
    // has not already been initialized. In this case it sets the default
    // encoding that will be used for the main database file if a new file
    // is created. If an existing main database file is opened, then the
    // default text encoding for the existing database is used.
    //
    // In all cases new databases created using the ATTACH command are
    // created to use the same default text encoding as the main database. If
    // the main database has not been initialized and/or created when ATTACH
    // is executed, this is done before the ATTACH operation.
    //
    // In the second form this pragma sets the text encoding to be used in
    // new database files created using this database handle. It is only
    // useful if invoked immediately after the main database i
    //   PRAGMA [schema.]schema_version
    //   PRAGMA [schema.]schema_version = <integer>
    //
    //   PRAGMA [schema.]user_version
    //   PRAGMA [schema.]user_version = <integer>
    //
    //   PRAGMA [schema.]freelist_count
    //
    //   PRAGMA [schema.]data_version
    //
    //   PRAGMA [schema.]application_id
    //   PRAGMA [schema.]application_id = <integer>
    //
    // The pragma's schema_version and user_version are used to set or get
    // the value of the schema-version and user-version, respectively. Both
    // the schema-version and the user-version are 32-bit signed integers
    // stored in the database header.
    //
    // The schema-cookie is usually only manipulated internally by SQLite. It
    // is incremented by SQLite whenever the database schema is modified (by
    // creating or dropping a table or index). The schema version is used by
    // SQLite each time a query is executed to ensure that the internal cache
    // of the schema used when compiling the SQL query matches the schema of
    // the database against which the compiled query is actually executed.
    // Subverting this mechanism by using "PRAGMA schema_version" to modify
    // the schema-version is potentially dangerous and may lead to program
    // crashes or database corruption. Use with caution!
    //
    // The user-version is not used internally by SQLite. It may be used by
    // applications for any purpose.
    //   PRAGMA compile_options
    //
    // Return the names of all compile-time options used in this build,
    // one option per row.
    //   PRAGMA [schema.]wal_checkpoint = passive|full|restart|truncate
    //
    // Checkpoint the database.
    //   PRAGMA wal_autocheckpoint
    //   PRAGMA wal_autocheckpoint = N
    //
    // Configure a database connection to automatically checkpoint a database
    // after accumulating N frames in the log. Or query for the current value
    // of N.
    //  PRAGMA shrink_memory
    //
    // IMPLEMENTATION-OF: R-23445-46109 This pragma causes the database
    // connection on which it is invoked to free up as much memory as it
    // can, by calling sqlite3_db_release_memory().
    //  PRAGMA optimize
    //  PRAGMA optimize(MASK)
    //  PRAGMA schema.optimize
    //  PRAGMA schema.optimize(MASK)
    //
    // Attempt to optimize the database.  All schemas are optimized in the first
    // two forms, and only the specified schema is optimized in the latter two.
    //
    // The details of optimizations performed by this pragma are expected
    // to change and improve over time.  Applications should anticipate that
    // this pragma will perform new optimizations in future releases.
    //
    // The optional argument is a bitmask of optimizations to perform:
    //
    //    0x00001    Debugging mode.  Do not actually perform any optimizations
    //               but instead return one line of text for each optimization
    //               that would have been done.  Off by default.
    //
    //    0x00002    Run ANALYZE on tables that might benefit.  On by default.
    //               See below for additional information.
    //
    //    0x00010    Run all ANALYZE operations using an analysis_limit that
    //               is the lessor of the current analysis_limit and the
    //               SQLITE_DEFAULT_OPTIMIZE_LIMIT compile-time option.
    //               The default value of SQLITE_DEFAULT_OPTIMIZE_LIMIT is
    //               currently (2024-02-19) set to 2000, which is such that
    //               the worst case run-time for PRAGMA optimize on a 100MB
    //               database will usually be less than 100 milliseconds on
    //               a RaspberryPI-4 class machine.  On by default.
    //
    //    0x10000    Look at tables to see if they need to be reanalyzed
    //               due to growth or shrinkage even if they have not been
    //               queried during the current connection.  Off by default.
    //
    // The default MASK is and always shall be 0x0fffe.  In the current
    // implementation, the default mask only covers the 0x00002 optimization,
    // though additional optimizations that are covered by 0x0fffe might be
    // added in the future.  Optimizations that are off by default and must
    // be explicitly requested have masks of 0x10000 or greater.
    //
    // DETERMINATION OF WHEN TO RUN ANALYZE
    //
    // In the current implementation, a table is analyzed if only if all of
    // the following are true:
    //
    // (1) MASK bit 0x00002 is set.
    //
    // (2) The table is an ordinary table, not a virtual table or view.
    //
    // (3) The table name does not begin with "sqlite_".
    //
    // (4) One or more of the following is true:
    //      (4a) The 0x10000 MASK bit is set.
    //      (4b) One or more indexes on the table lacks an entry
    //           in the sqlite_stat1 table.
    //      (4c) The query planner used sqlite_stat1-style statistics for one
    //           or more indexes of the table at some point during the lifetime
    //           of the current connection.
    //
    // (5) One or more of the following is true:
    //      (5a) One or more indexes on the table lacks an entry
    //           in the sqlite_stat1 table.  (Same as 4a)
    //      (5b) The number of rows in the table has increased or decreased by
    //           10-fold.  In other words, the current size of the table is
    //           10 times larger than the size in sqlite_stat1 or else the
    //           current size is less than 1/10th the size in sqlite_stat1.
    //
    // The rules for when tables are analyzed are likely to change in
    // future releases.  Future versions of SQLite might accept a string
    // literal argument to this pragma that contains a mnemonic description
    // of the options rather than a bitmap.
    //   PRAGMA busy_timeout
    //   PRAGMA busy_timeout = N
    //
    // Call sqlite3_busy_timeout(db, N).  Return the current timeout value
    // if one is set.  If no busy handler or a different busy handler is set
    // then 0 is returned.  Setting the busy_timeout to 0 or negative
    // disables the timeout.
    //
    // case PragTyp_BUSY_TIMEOUT
    //   PRAGMA soft_heap_limit
    //   PRAGMA soft_heap_limit = N
    //
    // IMPLEMENTATION-OF: R-26343-45930 This pragma invokes the
    // sqlite3_soft_heap_limit64() interface with the argument N, if N is
    // specified and is a non-negative integer.
    // IMPLEMENTATION-OF: R-64451-07163 The soft_heap_limit pragma always
    // returns the same integer that would be returned by the
    // sqlite3_soft_heap_limit64(-1) C-language function.
    //   PRAGMA hard_heap_limit
    //   PRAGMA hard_heap_limit = N
    //
    // Invoke sqlite3_hard_heap_limit64() to query or set the hard heap
    // limit.  The hard heap limit can be activated or lowered by this
    // pragma, but not raised or deactivated.  Only the
    // sqlite3_hard_heap_limit64() C-language API can raise or deactivate
    // the hard heap limit.  This allows an application to set a heap limit
    // constraint that cannot be relaxed by an untrusted SQL script.
    //   PRAGMA threads
    //   PRAGMA threads = N
    //
    // Configure the maximum number of worker threads.  Return the new
    // maximum, which might be less than requested.
    //   PRAGMA analysis_limit
    //   PRAGMA analysis_limit = N
    //
    // Configure the maximum number of rows that ANALYZE will examine
    // in each index that it looks at.  Return the new limit.
}

static mut iLn_593: i32 = 0 as i32;

static mut getCacheSize: __SlateAlign16<[VdbeOpList; 9]> = __SlateAlign16([
    VdbeOpList {
        opcode: ((2 as i32) as i8) as u8,
        p1: (0 as i32) as i8,
        p2: (0 as i32) as i8,
        p3: (0 as i32) as i8,
    },
    VdbeOpList {
        opcode: ((101 as i32) as i8) as u8,
        p1: (0 as i32) as i8,
        p2: (1 as i32) as i8,
        p3: (3 as i32) as i8,
    },
    VdbeOpList {
        opcode: ((61 as i32) as i8) as u8,
        p1: (1 as i32) as i8,
        p2: (8 as i32) as i8,
        p3: (0 as i32) as i8,
    },
    VdbeOpList {
        opcode: ((73 as i32) as i8) as u8,
        p1: (0 as i32) as i8,
        p2: (2 as i32) as i8,
        p3: (0 as i32) as i8,
    },
    VdbeOpList {
        opcode: ((108 as i32) as i8) as u8,
        p1: (1 as i32) as i8,
        p2: (2 as i32) as i8,
        p3: (1 as i32) as i8,
    },
    VdbeOpList {
        opcode: ((61 as i32) as i8) as u8,
        p1: (1 as i32) as i8,
        p2: (8 as i32) as i8,
        p3: (0 as i32) as i8,
    },
    VdbeOpList {
        opcode: ((73 as i32) as i8) as u8,
        p1: (0 as i32) as i8,
        p2: (1 as i32) as i8,
        p3: (0 as i32) as i8,
    },
    VdbeOpList {
        opcode: ((189 as i32) as i8) as u8,
        p1: (0 as i32) as i8,
        p2: (0 as i32) as i8,
        p3: (0 as i32) as i8,
    },
    VdbeOpList {
        opcode: ((86 as i32) as i8) as u8,
        p1: (1 as i32) as i8,
        p2: (1 as i32) as i8,
        p3: (0 as i32) as i8,
    },
]);

static mut iLn_616: i32 = 0 as i32;

static mut setMeta6: __SlateAlign16<[VdbeOpList; 5]> = __SlateAlign16([
    VdbeOpList {
        opcode: ((2 as i32) as i8) as u8,
        p1: (0 as i32) as i8,
        p2: (1 as i32) as i8,
        p3: (0 as i32) as i8,
    },
    VdbeOpList {
        opcode: ((101 as i32) as i8) as u8,
        p1: (0 as i32) as i8,
        p2: (1 as i32) as i8,
        p3: (4 as i32) as i8,
    },
    VdbeOpList {
        opcode: ((16 as i32) as i8) as u8,
        p1: (1 as i32) as i8,
        p2: (0 as i32) as i8,
        p3: (0 as i32) as i8,
    },
    VdbeOpList {
        opcode: ((72 as i32) as i8) as u8,
        p1: (0 as i32) as i8,
        p2: (2 as i32) as i8,
        p3: (0 as i32) as i8,
    },
    VdbeOpList {
        opcode: ((102 as i32) as i8) as u8,
        p1: (0 as i32) as i8,
        p2: (7 as i32) as i8,
        p3: (0 as i32) as i8,
    },
]);

static mut aStdTypeMask: [u8; 6] = [
    ((31 as i32) as i8) as u8,
    ((24 as i32) as i8) as u8,
    ((17 as i32) as i8) as u8,
    ((17 as i32) as i8) as u8,
    ((19 as i32) as i8) as u8,
    ((20 as i32) as i8) as u8,
];

static mut iLn_751: i32 = 0 as i32;

static mut endCode: __SlateAlign16<[VdbeOpList; 7]> = __SlateAlign16([
    VdbeOpList {
        opcode: ((88 as i32) as i8) as u8,
        p1: (1 as i32) as i8,
        p2: (0 as i32) as i8,
        p3: (0 as i32) as i8,
    },
    VdbeOpList {
        opcode: ((62 as i32) as i8) as u8,
        p1: (1 as i32) as i8,
        p2: (4 as i32) as i8,
        p3: (0 as i32) as i8,
    },
    VdbeOpList {
        opcode: ((118 as i32) as i8) as u8,
        p1: (0 as i32) as i8,
        p2: (3 as i32) as i8,
        p3: (0 as i32) as i8,
    },
    VdbeOpList {
        opcode: ((86 as i32) as i8) as u8,
        p1: (3 as i32) as i8,
        p2: (1 as i32) as i8,
        p3: (0 as i32) as i8,
    },
    VdbeOpList {
        opcode: ((72 as i32) as i8) as u8,
        p1: (0 as i32) as i8,
        p2: (0 as i32) as i8,
        p3: (0 as i32) as i8,
    },
    VdbeOpList {
        opcode: ((118 as i32) as i8) as u8,
        p1: (0 as i32) as i8,
        p2: (3 as i32) as i8,
        p3: (0 as i32) as i8,
    },
    VdbeOpList {
        opcode: ((9 as i32) as i8) as u8,
        p1: (0 as i32) as i8,
        p2: (3 as i32) as i8,
        p3: (0 as i32) as i8,
    },
]);

static mut encnames: __SlateAlign16<[EncName; 9]> = __SlateAlign16([
    EncName {
        zName: b"UTF8\0".as_ptr() as *mut i8,
        enc: ((1 as i32) as i8) as u8,
    },
    EncName {
        zName: b"UTF-8\0".as_ptr() as *mut i8,
        enc: ((1 as i32) as i8) as u8,
    },
    EncName {
        zName: b"UTF-16le\0".as_ptr() as *mut i8,
        enc: ((2 as i32) as i8) as u8,
    },
    EncName {
        zName: b"UTF-16be\0".as_ptr() as *mut i8,
        enc: ((3 as i32) as i8) as u8,
    },
    EncName {
        zName: b"UTF16le\0".as_ptr() as *mut i8,
        enc: ((2 as i32) as i8) as u8,
    },
    EncName {
        zName: b"UTF16be\0".as_ptr() as *mut i8,
        enc: ((3 as i32) as i8) as u8,
    },
    EncName {
        zName: b"UTF-16\0".as_ptr() as *mut i8,
        enc: ((0 as i32) as i8) as u8,
    },
    EncName {
        zName: b"UTF16\0".as_ptr() as *mut i8,
        enc: ((0 as i32) as i8) as u8,
    },
    EncName {
        zName: std::ptr::null_mut::<i8>(),
        enc: ((0 as i32) as i8) as u8,
    },
]);

static mut setCookie: [VdbeOpList; 2] = [
    VdbeOpList {
        opcode: ((2 as i32) as i8) as u8,
        p1: (0 as i32) as i8,
        p2: (1 as i32) as i8,
        p3: (0 as i32) as i8,
    },
    VdbeOpList {
        opcode: ((102 as i32) as i8) as u8,
        p1: (0 as i32) as i8,
        p2: (0 as i32) as i8,
        p3: (0 as i32) as i8,
    },
];

static mut readCookie: [VdbeOpList; 3] = [
    VdbeOpList {
        opcode: ((2 as i32) as i8) as u8,
        p1: (0 as i32) as i8,
        p2: (0 as i32) as i8,
        p3: (0 as i32) as i8,
    },
    VdbeOpList {
        opcode: ((101 as i32) as i8) as u8,
        p1: (0 as i32) as i8,
        p2: (1 as i32) as i8,
        p3: (0 as i32) as i8,
    },
    VdbeOpList {
        opcode: ((86 as i32) as i8) as u8,
        p1: (1 as i32) as i8,
        p2: (1 as i32) as i8,
        p3: (0 as i32) as i8,
    },
];

/// Implementation of an eponymous virtual table that runs a pragma.
#[repr(C)]
#[derive(Clone, Copy)]
struct PragmaVtab {
    /// Base class.  Must be first
    base: sqlite3_vtab,
    /// The database connection to which it belongs
    db: *mut sqlite3,
    /// Name of the pragma
    pName: *const PragmaName,
    /// Number of hidden columns
    nHidden: u8,
    /// Index of the first hidden column
    iHidden: u8,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct PragmaVtabCursor {
    /// Base class.  Must be first
    base: sqlite3_vtab_cursor,
    /// The pragma statement to run
    pPragma: *mut sqlite3_stmt,
    /// Current rowid
    iRowid: i64,
    /// Value of the argument and schema
    azArg: [*mut i8; 2],
}

/// Pragma virtual table module xConnect method.
#[unsafe(link_section = ".text.slate_distinct.pragma.pragmaVtabConnect")]
extern "C-unwind" fn pragmaVtabConnect(
    mut db: *mut sqlite3,
    mut pAux: *mut (),
    mut argc: i32,
    mut argv: *const *const i8,
    mut ppVtab: *mut *mut sqlite3_vtab,
    mut pzErr: *mut *mut i8,
) -> i32 {
    let mut pPragma: *const PragmaName = pAux as *const PragmaName;
    let mut pTab: *mut PragmaVtab = std::ptr::null_mut::<PragmaVtab>();
    let mut rc: i32 = 0 as i32;
    let mut i: i32 = 0 as i32;
    let mut j: i32 = 0 as i32;
    let mut cSep: i8 = (40 as i32) as i8;
    let mut acc: sqlite3_str = unsafe { std::mem::zeroed() };
    let mut zBuf: __SlateAlign16<[i8; 200]> = __SlateAlign16([0 as i8; 200]);
    argc;
    argv;
    unsafe {
        sqlite3StrAccumInit(
            std::ptr::addr_of_mut!(acc),
            std::ptr::null_mut::<sqlite3>(),
            zBuf.0.as_mut_ptr() as *mut i8,
            ((200 as u64) as u32) as i32,
            0 as i32,
        )
    };
    unsafe {
        sqlite3_str_appendall(
            std::ptr::addr_of_mut!(acc),
            (b"CREATE TABLE x\0".as_ptr() as *mut i8) as *const i8,
        )
    };
    i = 0 as i32;
    let __v1665: i32 = ((unsafe { (*pPragma).iPragCName }) as u32) as i32;
    j = __v1665;
    '__slate_break_1479: loop {
        if !(i < (((unsafe { (*pPragma).nPragCName }) as u32) as i32)) {
            break;
        }
        unsafe {
            sqlite3_str_appendf(
                std::ptr::addr_of_mut!(acc),
                (b"%c\"%s\"\0".as_ptr() as *mut i8) as *const i8,
                cSep as i32,
                unsafe {
                    *unsafe {
                        unsafe { std::ptr::addr_of!(pragCName.0) as *const *const i8 }
                            .offset(j as isize)
                    }
                },
            )
        };
        cSep = (44 as i32) as i8;
        let __v1666: i32 = i;
        let __v1667: i32 = __v1666 + (1 as i32);
        i = __v1667;
        let __v1668: i32 = j;
        let __v1669: i32 = __v1668 + (1 as i32);
        j = __v1669;
    }
    if i == (0 as i32) {
        unsafe {
            sqlite3_str_appendf(
                std::ptr::addr_of_mut!(acc),
                (b"(\"%s\"\0".as_ptr() as *mut i8) as *const i8,
                unsafe { (*pPragma).zName },
            )
        };
        let __v1670: i32 = i;
        let __v1671: i32 = __v1670 + (1 as i32);
        i = __v1671;
    }
    j = 0 as i32;
    if (((unsafe { (*pPragma).mPragFlg }) as u32) as i32) & (32 as i32) != (0 as i32) {
        unsafe {
            sqlite3_str_appendall(
                std::ptr::addr_of_mut!(acc),
                (b",arg HIDDEN\0".as_ptr() as *mut i8) as *const i8,
            )
        };
        let __v1672: i32 = j;
        let __v1673: i32 = __v1672 + (1 as i32);
        j = __v1673;
    }
    if (((unsafe { (*pPragma).mPragFlg }) as u32) as i32) & ((64 as i32) | (128 as i32))
        != (0 as i32)
    {
        unsafe {
            sqlite3_str_appendall(
                std::ptr::addr_of_mut!(acc),
                (b",schema HIDDEN\0".as_ptr() as *mut i8) as *const i8,
            )
        };
        let __v1674: i32 = j;
        let __v1675: i32 = __v1674 + (1 as i32);
        j = __v1675;
    }
    unsafe {
        sqlite3_str_append(
            std::ptr::addr_of_mut!(acc),
            (b")\0".as_ptr() as *mut i8) as *const i8,
            1 as i32,
        )
    };
    unsafe { sqlite3StrAccumFinish(std::ptr::addr_of_mut!(acc)) };
    0 as i32;
    rc = unsafe { sqlite3_declare_vtab(db, (zBuf.0.as_mut_ptr() as *mut i8) as *const i8) };
    if rc == (0 as i32) {
        pTab = (unsafe { sqlite3_malloc(((48 as u64) as u32) as i32) }) as *mut PragmaVtab;
        if pTab == std::ptr::null_mut::<PragmaVtab>() {
            rc = 7 as i32;
        } else {
            unsafe { memset(pTab as *mut (), 0 as i32, 48 as u64) };
            unsafe {
                (*pTab).pName = pPragma;
            }
            unsafe {
                (*pTab).db = db;
            }
            unsafe {
                (*pTab).iHidden = (i as i8) as u8;
            }
            unsafe {
                (*pTab).nHidden = (j as i8) as u8;
            }
        }
    } else {
        unsafe {
            *pzErr = unsafe {
                sqlite3_mprintf((b"%s\0".as_ptr() as *mut i8) as *const i8, unsafe {
                    sqlite3_errmsg(db)
                })
            };
        }
    }
    unsafe {
        *ppVtab = pTab as *mut sqlite3_vtab;
    }
    return rc;
}

/// Pragma virtual table module xDisconnect method.
#[unsafe(link_section = ".text.slate_distinct.pragma.pragmaVtabDisconnect")]
extern "C-unwind" fn pragmaVtabDisconnect(mut pVtab: *mut sqlite3_vtab) -> i32 {
    let mut pTab: *mut PragmaVtab = pVtab as *mut PragmaVtab;
    unsafe { sqlite3_free(pTab as *mut ()) };
    return 0 as i32;
}

/// Figure out the best index to use to search a pragma virtual table.
///
/// There are not really any index choices.  But we want to encourage the
/// query planner to give == constraints on as many hidden parameters as
/// possible, and especially on the first hidden parameter.  So return a
/// high cost if hidden parameters are unconstrained.
#[unsafe(link_section = ".text.slate_distinct.pragma.pragmaVtabBestIndex")]
extern "C-unwind" fn pragmaVtabBestIndex(
    mut tab: *mut sqlite3_vtab,
    mut pIdxInfo: *mut sqlite3_index_info,
) -> i32 {
    let mut pTab: *mut PragmaVtab = tab as *mut PragmaVtab;
    let mut pConstraint: *const sqlite3_index_constraint = unsafe { std::mem::zeroed() };
    let mut i: i32 = 0 as i32;
    let mut j: i32 = 0 as i32;
    let mut seen: [i32; 2] = [0 as i32; 2];
    unsafe {
        (*pIdxInfo).estimatedCost = (1 as i32) as f64;
    }
    if (((unsafe { (*pTab).nHidden }) as u32) as i32) == (0 as i32) {
        return 0 as i32;
    }
    pConstraint = (unsafe { (*pIdxInfo).aConstraint }) as *const sqlite3_index_constraint;
    unsafe {
        *unsafe { (seen.as_mut_ptr() as *mut i32).offset((0 as i32) as isize) } = 0 as i32;
    }
    unsafe {
        *unsafe { (seen.as_mut_ptr() as *mut i32).offset((1 as i32) as isize) } = 0 as i32;
    }
    i = 0 as i32;
    '__slate_break_1486: while i < unsafe { (*pIdxInfo).nConstraint } {
        if (unsafe { (*pConstraint).iColumn }) < (((unsafe { (*pTab).iHidden }) as u32) as i32) {
        } else {
            if (((unsafe { (*pConstraint).op }) as u32) as i32) != (2 as i32) {
            } else {
                if (((unsafe { (*pConstraint).usable }) as u32) as i32) == (0 as i32) {
                    return 19 as i32;
                }
                j = (unsafe { (*pConstraint).iColumn })
                    - (((unsafe { (*pTab).iHidden }) as u32) as i32);
                0 as i32;
                unsafe {
                    *unsafe { (seen.as_mut_ptr() as *mut i32).offset(j as isize) } = i + (1 as i32);
                }
            }
        }
        let __v1676: i32 = i;
        let __v1677: i32 = __v1676 + (1 as i32);
        i = __v1677;
        let __v1678: *const sqlite3_index_constraint = pConstraint;
        let __v1679: *const sqlite3_index_constraint =
            unsafe { __v1678.offset((1 as i32) as isize) };
        pConstraint = __v1679;
    }
    if (unsafe { *unsafe { (seen.as_mut_ptr() as *mut i32).offset((0 as i32) as isize) } })
        == (0 as i32)
    {
        unsafe {
            (*pIdxInfo).estimatedCost = (2147483647 as i32) as f64;
        }
        unsafe {
            (*pIdxInfo).estimatedRows = (2147483647 as i32) as i64;
        }
        return 0 as i32;
    }
    j = (unsafe { *unsafe { (seen.as_mut_ptr() as *mut i32).offset((0 as i32) as isize) } })
        - (1 as i32);
    unsafe {
        (*unsafe { unsafe { (*pIdxInfo).aConstraintUsage }.offset(j as isize) }).argvIndex =
            1 as i32;
    }
    unsafe {
        (*unsafe { unsafe { (*pIdxInfo).aConstraintUsage }.offset(j as isize) }).omit =
            ((1 as i32) as i8) as u8;
    }
    unsafe {
        (*pIdxInfo).estimatedCost = (20 as i32) as f64;
    }
    unsafe {
        (*pIdxInfo).estimatedRows = (20 as i32) as i64;
    }
    if (unsafe { *unsafe { (seen.as_mut_ptr() as *mut i32).offset((1 as i32) as isize) } })
        != (0 as i32)
    {
        j = (unsafe { *unsafe { (seen.as_mut_ptr() as *mut i32).offset((1 as i32) as isize) } })
            - (1 as i32);
        unsafe {
            (*unsafe { unsafe { (*pIdxInfo).aConstraintUsage }.offset(j as isize) }).argvIndex =
                2 as i32;
        }
        unsafe {
            (*unsafe { unsafe { (*pIdxInfo).aConstraintUsage }.offset(j as isize) }).omit =
                ((1 as i32) as i8) as u8;
        }
    }
    return 0 as i32;
}

/// Create a new cursor for the pragma virtual table
#[unsafe(link_section = ".text.slate_distinct.pragma.pragmaVtabOpen")]
extern "C-unwind" fn pragmaVtabOpen(
    mut pVtab: *mut sqlite3_vtab,
    mut ppCursor: *mut *mut sqlite3_vtab_cursor,
) -> i32 {
    let mut pCsr: *mut PragmaVtabCursor = unsafe { std::mem::zeroed() };
    pCsr = (unsafe { sqlite3_malloc(((40 as u64) as u32) as i32) }) as *mut PragmaVtabCursor;
    if pCsr == std::ptr::null_mut::<PragmaVtabCursor>() {
        return 7 as i32;
    }
    unsafe { memset(pCsr as *mut (), 0 as i32, 40 as u64) };
    unsafe {
        (*pCsr).base.pVtab = pVtab;
    }
    unsafe {
        *ppCursor = unsafe { std::ptr::addr_of_mut!((*pCsr).base) };
    }
    return 0 as i32;
}

/// Clear all content from pragma virtual table cursor.
fn pragmaVtabCursorClear(mut pCsr: *mut PragmaVtabCursor) {
    let mut i: i32 = 0 as i32;
    unsafe { sqlite3_finalize(unsafe { (*pCsr).pPragma }) };
    unsafe {
        (*pCsr).pPragma = std::ptr::null_mut::<sqlite3_stmt>();
    }
    unsafe {
        (*pCsr).iRowid = (0 as i32) as i64;
    }
    i = 0 as i32;
    '__slate_break_1487: loop {
        if !(i < ((((16 as u64) / (8 as u64)) as u32) as i32)) {
            break;
        }
        unsafe {
            sqlite3_free(
                (unsafe {
                    *unsafe {
                        unsafe { (*pCsr).azArg.as_mut_ptr() as *mut *mut i8 }.offset(i as isize)
                    }
                }) as *mut (),
            )
        };
        unsafe {
            *unsafe { unsafe { (*pCsr).azArg.as_mut_ptr() as *mut *mut i8 }.offset(i as isize) } =
                std::ptr::null_mut::<i8>();
        }
        let __v1680: i32 = i;
        let __v1681: i32 = __v1680 + (1 as i32);
        i = __v1681;
    }
}

/// Close a pragma virtual table cursor
#[unsafe(link_section = ".text.slate_distinct.pragma.pragmaVtabClose")]
extern "C-unwind" fn pragmaVtabClose(mut cur: *mut sqlite3_vtab_cursor) -> i32 {
    let mut pCsr: *mut PragmaVtabCursor = cur as *mut PragmaVtabCursor;
    pragmaVtabCursorClear(pCsr);
    unsafe { sqlite3_free(pCsr as *mut ()) };
    return 0 as i32;
}

/// Advance the pragma virtual table cursor to the next row
#[unsafe(link_section = ".text.slate_distinct.pragma.pragmaVtabNext")]
extern "C-unwind" fn pragmaVtabNext(mut pVtabCursor: *mut sqlite3_vtab_cursor) -> i32 {
    let mut pCsr: *mut PragmaVtabCursor = pVtabCursor as *mut PragmaVtabCursor;
    let mut rc: i32 = 0 as i32;
    // Increment the xRowid value
    let __v1682: *mut PragmaVtabCursor = pCsr;
    let __v1683: i64 = unsafe { (*__v1682).iRowid };
    let __v1684: i64 = __v1683 + ((1 as i32) as i64);
    unsafe {
        (*__v1682).iRowid = __v1684;
    }
    0 as i32;
    if (100 as i32) != unsafe { sqlite3_step(unsafe { (*pCsr).pPragma }) } {
        rc = unsafe { sqlite3_finalize(unsafe { (*pCsr).pPragma }) };
        unsafe {
            (*pCsr).pPragma = std::ptr::null_mut::<sqlite3_stmt>();
        }
        pragmaVtabCursorClear(pCsr);
    }
    return rc;
}

/// Pragma virtual table module xFilter method.
#[unsafe(link_section = ".text.slate_distinct.pragma.pragmaVtabFilter")]
extern "C-unwind" fn pragmaVtabFilter(
    mut pVtabCursor: *mut sqlite3_vtab_cursor,
    mut idxNum: i32,
    mut idxStr: *const i8,
    mut argc: i32,
    mut argv: *mut *mut sqlite3_value,
) -> i32 {
    let mut pCsr: *mut PragmaVtabCursor = pVtabCursor as *mut PragmaVtabCursor;
    let mut pTab: *mut PragmaVtab = (unsafe { (*pVtabCursor).pVtab }) as *mut PragmaVtab;
    let mut rc: i32 = 0 as i32;
    let mut i: i32 = 0 as i32;
    let mut j: i32 = 0 as i32;
    let mut acc: sqlite3_str = unsafe { std::mem::zeroed() };
    let mut zSql: *mut i8 = unsafe { std::mem::zeroed() };
    idxNum;
    idxStr;
    pragmaVtabCursorClear(pCsr);
    j = if (((unsafe { (*unsafe { (*pTab).pName }).mPragFlg }) as u32) as i32) & (32 as i32)
        != (0 as i32)
    {
        0 as i32
    } else {
        1 as i32
    };
    i = 0 as i32;
    '__slate_break_1488: loop {
        if !(i < argc) {
            break;
        }
        let mut zText_847: *const i8 =
            (unsafe { sqlite3_value_text(unsafe { *unsafe { argv.offset(i as isize) } }) })
                as *const i8;
        0 as i32;
        0 as i32;
        if zText_847 != std::ptr::null::<i8>() {
            unsafe {
                *unsafe {
                    unsafe { (*pCsr).azArg.as_mut_ptr() as *mut *mut i8 }.offset(j as isize)
                } = unsafe {
                    sqlite3_mprintf((b"%s\0".as_ptr() as *mut i8) as *const i8, zText_847)
                };
            }
            if (unsafe {
                *unsafe { unsafe { (*pCsr).azArg.as_mut_ptr() as *mut *mut i8 }.offset(j as isize) }
            }) == std::ptr::null_mut::<i8>()
            {
                return 7 as i32;
            }
        }
        let __v1685: i32 = i;
        let __v1686: i32 = __v1685 + (1 as i32);
        i = __v1686;
        let __v1687: i32 = j;
        let __v1688: i32 = __v1687 + (1 as i32);
        j = __v1688;
    }
    unsafe {
        sqlite3StrAccumInit(
            std::ptr::addr_of_mut!(acc),
            std::ptr::null_mut::<sqlite3>(),
            std::ptr::null_mut::<i8>(),
            0 as i32,
            unsafe {
                *unsafe {
                    unsafe { (*unsafe { (*pTab).db }).aLimit.as_mut_ptr() as *mut i32 }
                        .offset((1 as i32) as isize)
                }
            },
        )
    };
    unsafe {
        sqlite3_str_appendall(
            std::ptr::addr_of_mut!(acc),
            (b"PRAGMA \0".as_ptr() as *mut i8) as *const i8,
        )
    };
    if (unsafe {
        *unsafe {
            unsafe { (*pCsr).azArg.as_mut_ptr() as *mut *mut i8 }.offset((1 as i32) as isize)
        }
    }) != std::ptr::null_mut::<i8>()
    {
        unsafe {
            sqlite3_str_appendf(
                std::ptr::addr_of_mut!(acc),
                (b"%Q.\0".as_ptr() as *mut i8) as *const i8,
                unsafe {
                    *unsafe {
                        unsafe { (*pCsr).azArg.as_mut_ptr() as *mut *mut i8 }
                            .offset((1 as i32) as isize)
                    }
                },
            )
        };
    }
    unsafe {
        sqlite3_str_appendall(std::ptr::addr_of_mut!(acc), unsafe {
            (*unsafe { (*pTab).pName }).zName
        })
    };
    if (unsafe {
        *unsafe {
            unsafe { (*pCsr).azArg.as_mut_ptr() as *mut *mut i8 }.offset((0 as i32) as isize)
        }
    }) != std::ptr::null_mut::<i8>()
    {
        unsafe {
            sqlite3_str_appendf(
                std::ptr::addr_of_mut!(acc),
                (b"=%Q\0".as_ptr() as *mut i8) as *const i8,
                unsafe {
                    *unsafe {
                        unsafe { (*pCsr).azArg.as_mut_ptr() as *mut *mut i8 }
                            .offset((0 as i32) as isize)
                    }
                },
            )
        };
    }
    zSql = unsafe { sqlite3StrAccumFinish(std::ptr::addr_of_mut!(acc)) };
    if zSql == std::ptr::null_mut::<i8>() {
        return 7 as i32;
    }
    rc = unsafe {
        sqlite3_prepare_v2(
            unsafe { (*pTab).db },
            zSql as *const i8,
            -(1 as i32),
            unsafe { std::ptr::addr_of_mut!((*pCsr).pPragma) },
            std::ptr::null_mut::<*const i8>(),
        )
    };
    unsafe { sqlite3_free(zSql as *mut ()) };
    if rc != (0 as i32) {
        unsafe {
            (*pTab).base.zErrMsg = unsafe {
                sqlite3_mprintf((b"%s\0".as_ptr() as *mut i8) as *const i8, unsafe {
                    sqlite3_errmsg(unsafe { (*pTab).db })
                })
            };
        }
        return rc;
    }
    return pragmaVtabNext(pVtabCursor);
}

/// Pragma virtual table module xEof method.
#[unsafe(link_section = ".text.slate_distinct.pragma.pragmaVtabEof")]
extern "C-unwind" fn pragmaVtabEof(mut pVtabCursor: *mut sqlite3_vtab_cursor) -> i32 {
    let mut pCsr: *mut PragmaVtabCursor = pVtabCursor as *mut PragmaVtabCursor;
    return ((unsafe { (*pCsr).pPragma }) == std::ptr::null_mut::<sqlite3_stmt>()) as i32;
}

/// The xColumn method simply returns the corresponding column from
/// the PRAGMA.
#[unsafe(link_section = ".text.slate_distinct.pragma.pragmaVtabColumn")]
extern "C-unwind" fn pragmaVtabColumn(
    mut pVtabCursor: *mut sqlite3_vtab_cursor,
    mut ctx: *mut sqlite3_context,
    mut i: i32,
) -> i32 {
    let mut pCsr: *mut PragmaVtabCursor = pVtabCursor as *mut PragmaVtabCursor;
    let mut pTab: *mut PragmaVtab = (unsafe { (*pVtabCursor).pVtab }) as *mut PragmaVtab;
    if i < (((unsafe { (*pTab).iHidden }) as u32) as i32) {
        unsafe {
            sqlite3_result_value(ctx, unsafe {
                sqlite3_column_value(unsafe { (*pCsr).pPragma }, i)
            })
        };
    } else {
        unsafe {
            sqlite3_result_text(
                ctx,
                (unsafe {
                    *unsafe {
                        unsafe { (*pCsr).azArg.as_mut_ptr() as *mut *mut i8 }
                            .offset((i - (((unsafe { (*pTab).iHidden }) as u32) as i32)) as isize)
                    }
                }) as *const i8,
                -(1 as i32),
                unsafe {
                    std::mem::transmute::<usize, Option<unsafe extern "C-unwind" fn(*mut ())>>(
                        -(1 as i32) as usize,
                    )
                },
            )
        };
    }
    return 0 as i32;
}

/// Pragma virtual table module xRowid method.
#[unsafe(link_section = ".text.slate_distinct.pragma.pragmaVtabRowid")]
extern "C-unwind" fn pragmaVtabRowid(
    mut pVtabCursor: *mut sqlite3_vtab_cursor,
    mut p: *mut i64,
) -> i32 {
    let mut pCsr: *mut PragmaVtabCursor = pVtabCursor as *mut PragmaVtabCursor;
    unsafe {
        *p = unsafe { (*pCsr).iRowid };
    }
    return 0 as i32;
}

/// The pragma virtual table object
/// iVersion
/// xCreate - create a table
/// xConnect - connect to an existing table
/// xBestIndex - Determine search strategy
/// xDisconnect - Disconnect from a table
/// xDestroy - Drop a table
/// xOpen - open a cursor
/// xClose - close a cursor
/// xFilter - configure scan constraints
/// xNext - advance a cursor
/// xEof
/// xColumn - read data
/// xRowid - read data
/// xUpdate - write data
/// xBegin - begin transaction
/// xSync - sync transaction
/// xCommit - commit transaction
/// xRollback - rollback transaction
/// xFindFunction - function overloading
/// xRename - rename the table
/// xSavepoint
/// xRelease
/// xRollbackTo
/// xShadowName
/// xIntegrity
static mut pragmaVtabModule: sqlite3_module = sqlite3_module {
    iVersion: 0 as i32,
    xCreate: None,
    xConnect: Some(pragmaVtabConnect),
    xBestIndex: Some(pragmaVtabBestIndex),
    xDisconnect: Some(pragmaVtabDisconnect),
    xDestroy: None,
    xOpen: Some(pragmaVtabOpen),
    xClose: Some(pragmaVtabClose),
    xFilter: Some(pragmaVtabFilter),
    xNext: Some(pragmaVtabNext),
    xEof: Some(pragmaVtabEof),
    xColumn: Some(pragmaVtabColumn),
    xRowid: Some(pragmaVtabRowid),
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

/// Check to see if zTabName is really the name of a pragma.  If it is,
/// then register an eponymous virtual table for that pragma and return
/// a pointer to the Module object for the new virtual table.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3PragmaVtabRegister(
    mut db: *mut sqlite3,
    mut zName: *const i8,
) -> *mut Module {
    let mut pName: *const PragmaName = unsafe { std::mem::zeroed() };
    0 as i32;
    pName = pragmaLocate(unsafe { zName.offset((7 as i32) as isize) });
    if pName == std::ptr::null::<PragmaName>() {
        return std::ptr::null_mut::<Module>();
    }
    if (((unsafe { (*pName).mPragFlg }) as u32) as i32) & ((16 as i32) | (32 as i32)) == (0 as i32)
    {
        return std::ptr::null_mut::<Module>();
    }
    0 as i32;
    return unsafe {
        sqlite3VtabCreateModule(
            db,
            zName,
            unsafe { std::ptr::addr_of!(pragmaVtabModule) },
            pName as *mut (),
            None,
        )
    };
}
