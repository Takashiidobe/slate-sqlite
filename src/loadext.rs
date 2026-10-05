unsafe extern "C" {
    static mut sqlite3UpperToLower: [u8; 0];
    static mut sqlite3CtypeMap: [u8; 0];
    fn sqlite3_libversion() -> *const i8;
    fn sqlite3_sourceid() -> *const i8;
    fn sqlite3_libversion_number() -> i32;
    fn sqlite3_compileoption_used(zOptName: *const i8) -> i32;
    fn sqlite3_compileoption_get(N: i32) -> *const i8;
    fn sqlite3_threadsafe() -> i32;
    fn sqlite3_close(__v983: *mut sqlite3) -> i32;
    fn sqlite3_close_v2(__v984: *mut sqlite3) -> i32;
    fn sqlite3_exec(
        __v985: *mut sqlite3,
        sql: *const i8,
        callback: Option<
            unsafe extern "C-unwind" fn(*mut (), i32, *mut *mut i8, *mut *mut i8) -> i32,
        >,
        __v988: *mut (),
        errmsg: *mut *mut i8,
    ) -> i32;
    fn sqlite3_initialize() -> i32;
    fn sqlite3_db_config(__v990: *mut sqlite3, op: i32, ...) -> i32;
    fn sqlite3_extended_result_codes(__v992: *mut sqlite3, onoff: i32) -> i32;
    fn sqlite3_last_insert_rowid(__v994: *mut sqlite3) -> i64;
    fn sqlite3_set_last_insert_rowid(__v995: *mut sqlite3, __v996: i64);
    fn sqlite3_changes(__v997: *mut sqlite3) -> i32;
    fn sqlite3_changes64(__v998: *mut sqlite3) -> i64;
    fn sqlite3_total_changes(__v999: *mut sqlite3) -> i32;
    fn sqlite3_total_changes64(__v1000: *mut sqlite3) -> i64;
    fn sqlite3_interrupt(__v1001: *mut sqlite3);
    fn sqlite3_is_interrupted(__v1002: *mut sqlite3) -> i32;
    fn sqlite3_complete(sql: *const i8) -> i32;
    fn sqlite3_complete16(sql: *const ()) -> i32;
    fn sqlite3_incomplete(sql: *const i8) -> i64;
    fn sqlite3_busy_handler(
        __v1006: *mut sqlite3,
        __v1007: Option<unsafe extern "C-unwind" fn(*mut (), i32) -> i32>,
        __v1008: *mut (),
    ) -> i32;
    fn sqlite3_busy_timeout(__v1009: *mut sqlite3, ms: i32) -> i32;
    fn sqlite3_setlk_timeout(__v1011: *mut sqlite3, ms: i32, flags: i32) -> i32;
    fn sqlite3_get_table(
        db: *mut sqlite3,
        zSql: *const i8,
        pazResult: *mut *mut *mut i8,
        pnRow: *mut i32,
        pnColumn: *mut i32,
        pzErrmsg: *mut *mut i8,
    ) -> i32;
    fn sqlite3_free_table(result: *mut *mut i8);
    fn sqlite3_mprintf(__v1021: *const i8, ...) -> *mut i8;
    fn sqlite3_vmprintf(__v1022: *const i8, __v1023: core::ffi::VaList<'_>) -> *mut i8;
    fn sqlite3_snprintf(__v1024: i32, __v1025: *mut i8, __v1026: *const i8, ...) -> *mut i8;
    fn sqlite3_vsnprintf(
        __v1027: i32,
        __v1028: *mut i8,
        __v1029: *const i8,
        __v1030: core::ffi::VaList<'_>,
    ) -> *mut i8;
    fn sqlite3_malloc(__v1031: i32) -> *mut ();
    fn sqlite3_malloc64(__v1032: u64) -> *mut ();
    fn sqlite3_realloc(__v1033: *mut (), __v1034: i32) -> *mut ();
    fn sqlite3_realloc64(__v1035: *mut (), __v1036: u64) -> *mut ();
    fn sqlite3_free(__v1037: *mut ());
    fn sqlite3_msize(__v1038: *mut ()) -> u64;
    fn sqlite3_memory_used() -> i64;
    fn sqlite3_memory_highwater(resetFlag: i32) -> i64;
    fn sqlite3_randomness(N: i32, P: *mut ());
    fn sqlite3_set_authorizer(
        __v1042: *mut sqlite3,
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
        pUserData: *mut (),
    ) -> i32;
    fn sqlite3_trace(
        __v1045: *mut sqlite3,
        xTrace: Option<unsafe extern "C-unwind" fn(*mut (), *const i8)>,
        __v1047: *mut (),
    ) -> *mut ();
    fn sqlite3_profile(
        __v1048: *mut sqlite3,
        xProfile: Option<unsafe extern "C-unwind" fn(*mut (), *const i8, u64)>,
        __v1050: *mut (),
    ) -> *mut ();
    fn sqlite3_trace_v2(
        __v1051: *mut sqlite3,
        uMask: u32,
        xCallback: Option<unsafe extern "C-unwind" fn(u32, *mut (), *mut (), *mut ()) -> i32>,
        pCtx: *mut (),
    ) -> i32;
    fn sqlite3_progress_handler(
        __v1055: *mut sqlite3,
        __v1056: i32,
        __v1057: Option<unsafe extern "C-unwind" fn(*mut ()) -> i32>,
        __v1058: *mut (),
    );
    fn sqlite3_open(filename: *const i8, ppDb: *mut *mut sqlite3) -> i32;
    fn sqlite3_open16(filename: *const (), ppDb: *mut *mut sqlite3) -> i32;
    fn sqlite3_open_v2(
        filename: *const i8,
        ppDb: *mut *mut sqlite3,
        flags: i32,
        zVfs: *const i8,
    ) -> i32;
    fn sqlite3_uri_parameter(z: *const i8, zParam: *const i8) -> *const i8;
    fn sqlite3_uri_boolean(z: *const i8, zParam: *const i8, bDefault: i32) -> i32;
    fn sqlite3_uri_int64(__v1072: *const i8, __v1073: *const i8, __v1074: i64) -> i64;
    fn sqlite3_uri_key(z: *const i8, N: i32) -> *const i8;
    fn sqlite3_filename_database(__v1077: *const i8) -> *const i8;
    fn sqlite3_filename_journal(__v1078: *const i8) -> *const i8;
    fn sqlite3_filename_wal(__v1079: *const i8) -> *const i8;
    fn sqlite3_database_file_object(__v1080: *const i8) -> *mut sqlite3_file;
    fn sqlite3_create_filename(
        zDatabase: *const i8,
        zJournal: *const i8,
        zWal: *const i8,
        nParam: i32,
        azParam: *mut *const i8,
    ) -> *const i8;
    fn sqlite3_free_filename(__v1086: *const i8);
    fn sqlite3_errcode(db: *mut sqlite3) -> i32;
    fn sqlite3_extended_errcode(db: *mut sqlite3) -> i32;
    fn sqlite3_errmsg(__v1089: *mut sqlite3) -> *const i8;
    fn sqlite3_errmsg16(__v1090: *mut sqlite3) -> *const ();
    fn sqlite3_errstr(__v1091: i32) -> *const i8;
    fn sqlite3_error_offset(db: *mut sqlite3) -> i32;
    fn sqlite3_set_errmsg(db: *mut sqlite3, errcode: i32, zErrMsg: *const i8) -> i32;
    fn sqlite3_limit(__v1096: *mut sqlite3, id: i32, newVal: i32) -> i32;
    fn sqlite3_prepare(
        db: *mut sqlite3,
        zSql: *const i8,
        nByte: i32,
        ppStmt: *mut *mut sqlite3_stmt,
        pzTail: *mut *const i8,
    ) -> i32;
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
    fn sqlite3_prepare16(
        db: *mut sqlite3,
        zSql: *const (),
        nByte: i32,
        ppStmt: *mut *mut sqlite3_stmt,
        pzTail: *mut *const (),
    ) -> i32;
    fn sqlite3_prepare16_v2(
        db: *mut sqlite3,
        zSql: *const (),
        nByte: i32,
        ppStmt: *mut *mut sqlite3_stmt,
        pzTail: *mut *const (),
    ) -> i32;
    fn sqlite3_prepare16_v3(
        db: *mut sqlite3,
        zSql: *const (),
        nByte: i32,
        prepFlags: u32,
        ppStmt: *mut *mut sqlite3_stmt,
        pzTail: *mut *const (),
    ) -> i32;
    fn sqlite3_sql(pStmt: *mut sqlite3_stmt) -> *const i8;
    fn sqlite3_expanded_sql(pStmt: *mut sqlite3_stmt) -> *mut i8;
    fn sqlite3_stmt_readonly(pStmt: *mut sqlite3_stmt) -> i32;
    fn sqlite3_stmt_isexplain(pStmt: *mut sqlite3_stmt) -> i32;
    fn sqlite3_stmt_explain(pStmt: *mut sqlite3_stmt, eMode: i32) -> i32;
    fn sqlite3_stmt_busy(__v1137: *mut sqlite3_stmt) -> i32;
    fn sqlite3_bind_blob(
        __v1138: *mut sqlite3_stmt,
        __v1139: i32,
        __v1140: *const (),
        n: i32,
        __v1142: Option<unsafe extern "C-unwind" fn(*mut ())>,
    ) -> i32;
    fn sqlite3_bind_blob64(
        __v1143: *mut sqlite3_stmt,
        __v1144: i32,
        __v1145: *const (),
        __v1146: u64,
        __v1147: Option<unsafe extern "C-unwind" fn(*mut ())>,
    ) -> i32;
    fn sqlite3_bind_double(__v1148: *mut sqlite3_stmt, __v1149: i32, __v1150: f64) -> i32;
    fn sqlite3_bind_int(__v1151: *mut sqlite3_stmt, __v1152: i32, __v1153: i32) -> i32;
    fn sqlite3_bind_int64(__v1154: *mut sqlite3_stmt, __v1155: i32, __v1156: i64) -> i32;
    fn sqlite3_bind_null(__v1157: *mut sqlite3_stmt, __v1158: i32) -> i32;
    fn sqlite3_bind_text(
        __v1159: *mut sqlite3_stmt,
        __v1160: i32,
        __v1161: *const i8,
        __v1162: i32,
        __v1163: Option<unsafe extern "C-unwind" fn(*mut ())>,
    ) -> i32;
    fn sqlite3_bind_text16(
        __v1164: *mut sqlite3_stmt,
        __v1165: i32,
        __v1166: *const (),
        __v1167: i32,
        __v1168: Option<unsafe extern "C-unwind" fn(*mut ())>,
    ) -> i32;
    fn sqlite3_bind_text64(
        __v1169: *mut sqlite3_stmt,
        __v1170: i32,
        __v1171: *const i8,
        __v1172: u64,
        __v1173: Option<unsafe extern "C-unwind" fn(*mut ())>,
        encoding: u8,
    ) -> i32;
    fn sqlite3_bind_value(
        __v1175: *mut sqlite3_stmt,
        __v1176: i32,
        __v1177: *const sqlite3_value,
    ) -> i32;
    fn sqlite3_bind_pointer(
        __v1178: *mut sqlite3_stmt,
        __v1179: i32,
        __v1180: *mut (),
        __v1181: *const i8,
        __v1182: Option<unsafe extern "C-unwind" fn(*mut ())>,
    ) -> i32;
    fn sqlite3_bind_zeroblob(__v1183: *mut sqlite3_stmt, __v1184: i32, n: i32) -> i32;
    fn sqlite3_bind_zeroblob64(__v1186: *mut sqlite3_stmt, __v1187: i32, __v1188: u64) -> i32;
    fn sqlite3_bind_parameter_count(__v1189: *mut sqlite3_stmt) -> i32;
    fn sqlite3_bind_parameter_name(__v1190: *mut sqlite3_stmt, __v1191: i32) -> *const i8;
    fn sqlite3_bind_parameter_index(__v1192: *mut sqlite3_stmt, zName: *const i8) -> i32;
    fn sqlite3_clear_bindings(__v1194: *mut sqlite3_stmt) -> i32;
    fn sqlite3_column_count(pStmt: *mut sqlite3_stmt) -> i32;
    fn sqlite3_column_name(__v1196: *mut sqlite3_stmt, N: i32) -> *const i8;
    fn sqlite3_column_name16(__v1198: *mut sqlite3_stmt, N: i32) -> *const ();
    fn sqlite3_column_decltype(__v1200: *mut sqlite3_stmt, __v1201: i32) -> *const i8;
    fn sqlite3_column_decltype16(__v1202: *mut sqlite3_stmt, __v1203: i32) -> *const ();
    fn sqlite3_step(__v1204: *mut sqlite3_stmt) -> i32;
    fn sqlite3_data_count(pStmt: *mut sqlite3_stmt) -> i32;
    fn sqlite3_column_blob(__v1206: *mut sqlite3_stmt, iCol: i32) -> *const ();
    fn sqlite3_column_double(__v1208: *mut sqlite3_stmt, iCol: i32) -> f64;
    fn sqlite3_column_int(__v1210: *mut sqlite3_stmt, iCol: i32) -> i32;
    fn sqlite3_column_int64(__v1212: *mut sqlite3_stmt, iCol: i32) -> i64;
    fn sqlite3_column_text(__v1214: *mut sqlite3_stmt, iCol: i32) -> *const u8;
    fn sqlite3_column_text16(__v1216: *mut sqlite3_stmt, iCol: i32) -> *const ();
    fn sqlite3_column_value(__v1218: *mut sqlite3_stmt, iCol: i32) -> *mut sqlite3_value;
    fn sqlite3_column_bytes(__v1220: *mut sqlite3_stmt, iCol: i32) -> i32;
    fn sqlite3_column_bytes16(__v1222: *mut sqlite3_stmt, iCol: i32) -> i32;
    fn sqlite3_column_type(__v1224: *mut sqlite3_stmt, iCol: i32) -> i32;
    fn sqlite3_finalize(pStmt: *mut sqlite3_stmt) -> i32;
    fn sqlite3_reset(pStmt: *mut sqlite3_stmt) -> i32;
    fn sqlite3_create_function(
        db: *mut sqlite3,
        zFunctionName: *const i8,
        nArg: i32,
        eTextRep: i32,
        pApp: *mut (),
        xFunc: Option<
            unsafe extern "C-unwind" fn(*mut sqlite3_context, i32, *mut *mut sqlite3_value),
        >,
        xStep: Option<
            unsafe extern "C-unwind" fn(*mut sqlite3_context, i32, *mut *mut sqlite3_value),
        >,
        xFinal: Option<unsafe extern "C-unwind" fn(*mut sqlite3_context)>,
    ) -> i32;
    fn sqlite3_create_function16(
        db: *mut sqlite3,
        zFunctionName: *const (),
        nArg: i32,
        eTextRep: i32,
        pApp: *mut (),
        xFunc: Option<
            unsafe extern "C-unwind" fn(*mut sqlite3_context, i32, *mut *mut sqlite3_value),
        >,
        xStep: Option<
            unsafe extern "C-unwind" fn(*mut sqlite3_context, i32, *mut *mut sqlite3_value),
        >,
        xFinal: Option<unsafe extern "C-unwind" fn(*mut sqlite3_context)>,
    ) -> i32;
    fn sqlite3_create_function_v2(
        db: *mut sqlite3,
        zFunctionName: *const i8,
        nArg: i32,
        eTextRep: i32,
        pApp: *mut (),
        xFunc: Option<
            unsafe extern "C-unwind" fn(*mut sqlite3_context, i32, *mut *mut sqlite3_value),
        >,
        xStep: Option<
            unsafe extern "C-unwind" fn(*mut sqlite3_context, i32, *mut *mut sqlite3_value),
        >,
        xFinal: Option<unsafe extern "C-unwind" fn(*mut sqlite3_context)>,
        xDestroy: Option<unsafe extern "C-unwind" fn(*mut ())>,
    ) -> i32;
    fn sqlite3_create_window_function(
        db: *mut sqlite3,
        zFunctionName: *const i8,
        nArg: i32,
        eTextRep: i32,
        pApp: *mut (),
        xStep: Option<
            unsafe extern "C-unwind" fn(*mut sqlite3_context, i32, *mut *mut sqlite3_value),
        >,
        xFinal: Option<unsafe extern "C-unwind" fn(*mut sqlite3_context)>,
        xValue: Option<unsafe extern "C-unwind" fn(*mut sqlite3_context)>,
        xInverse: Option<
            unsafe extern "C-unwind" fn(*mut sqlite3_context, i32, *mut *mut sqlite3_value),
        >,
        xDestroy: Option<unsafe extern "C-unwind" fn(*mut ())>,
    ) -> i32;
    fn sqlite3_aggregate_count(__v1263: *mut sqlite3_context) -> i32;
    fn sqlite3_expired(__v1264: *mut sqlite3_stmt) -> i32;
    fn sqlite3_transfer_bindings(__v1265: *mut sqlite3_stmt, __v1266: *mut sqlite3_stmt) -> i32;
    fn sqlite3_thread_cleanup();
    fn sqlite3_value_blob(__v1267: *mut sqlite3_value) -> *const ();
    fn sqlite3_value_double(__v1268: *mut sqlite3_value) -> f64;
    fn sqlite3_value_int(__v1269: *mut sqlite3_value) -> i32;
    fn sqlite3_value_int64(__v1270: *mut sqlite3_value) -> i64;
    fn sqlite3_value_pointer(__v1271: *mut sqlite3_value, __v1272: *const i8) -> *mut ();
    fn sqlite3_value_text(__v1273: *mut sqlite3_value) -> *const u8;
    fn sqlite3_value_text16(__v1274: *mut sqlite3_value) -> *const ();
    fn sqlite3_value_text16le(__v1275: *mut sqlite3_value) -> *const ();
    fn sqlite3_value_text16be(__v1276: *mut sqlite3_value) -> *const ();
    fn sqlite3_value_bytes(__v1277: *mut sqlite3_value) -> i32;
    fn sqlite3_value_bytes16(__v1278: *mut sqlite3_value) -> i32;
    fn sqlite3_value_type(__v1279: *mut sqlite3_value) -> i32;
    fn sqlite3_value_numeric_type(__v1280: *mut sqlite3_value) -> i32;
    fn sqlite3_value_nochange(__v1281: *mut sqlite3_value) -> i32;
    fn sqlite3_value_frombind(__v1282: *mut sqlite3_value) -> i32;
    fn sqlite3_value_encoding(__v1283: *mut sqlite3_value) -> i32;
    fn sqlite3_value_subtype(__v1284: *mut sqlite3_value) -> u32;
    fn sqlite3_value_dup(__v1285: *const sqlite3_value) -> *mut sqlite3_value;
    fn sqlite3_value_free(__v1286: *mut sqlite3_value);
    fn sqlite3_aggregate_context(__v1287: *mut sqlite3_context, nBytes: i32) -> *mut ();
    fn sqlite3_user_data(__v1289: *mut sqlite3_context) -> *mut ();
    fn sqlite3_context_db_handle(__v1290: *mut sqlite3_context) -> *mut sqlite3;
    fn sqlite3_get_auxdata(__v1291: *mut sqlite3_context, N: i32) -> *mut ();
    fn sqlite3_set_auxdata(
        __v1293: *mut sqlite3_context,
        N: i32,
        __v1295: *mut (),
        __v1296: Option<unsafe extern "C-unwind" fn(*mut ())>,
    );
    fn sqlite3_get_clientdata(__v1297: *mut sqlite3, __v1298: *const i8) -> *mut ();
    fn sqlite3_set_clientdata(
        __v1299: *mut sqlite3,
        __v1300: *const i8,
        __v1301: *mut (),
        __v1302: Option<unsafe extern "C-unwind" fn(*mut ())>,
    ) -> i32;
    fn sqlite3_result_blob(
        __v1303: *mut sqlite3_context,
        __v1304: *const (),
        __v1305: i32,
        __v1306: Option<unsafe extern "C-unwind" fn(*mut ())>,
    );
    fn sqlite3_result_blob64(
        __v1307: *mut sqlite3_context,
        __v1308: *const (),
        __v1309: u64,
        __v1310: Option<unsafe extern "C-unwind" fn(*mut ())>,
    );
    fn sqlite3_result_double(__v1311: *mut sqlite3_context, __v1312: f64);
    fn sqlite3_result_error(__v1313: *mut sqlite3_context, __v1314: *const i8, __v1315: i32);
    fn sqlite3_result_error16(__v1316: *mut sqlite3_context, __v1317: *const (), __v1318: i32);
    fn sqlite3_result_error_toobig(__v1319: *mut sqlite3_context);
    fn sqlite3_result_error_nomem(__v1320: *mut sqlite3_context);
    fn sqlite3_result_error_code(__v1321: *mut sqlite3_context, __v1322: i32);
    fn sqlite3_result_int(__v1323: *mut sqlite3_context, __v1324: i32);
    fn sqlite3_result_int64(__v1325: *mut sqlite3_context, __v1326: i64);
    fn sqlite3_result_null(__v1327: *mut sqlite3_context);
    fn sqlite3_result_text(
        __v1328: *mut sqlite3_context,
        __v1329: *const i8,
        __v1330: i32,
        __v1331: Option<unsafe extern "C-unwind" fn(*mut ())>,
    );
    fn sqlite3_result_text64(
        __v1332: *mut sqlite3_context,
        z: *const i8,
        n: u64,
        __v1335: Option<unsafe extern "C-unwind" fn(*mut ())>,
        encoding: u8,
    );
    fn sqlite3_result_text16(
        __v1337: *mut sqlite3_context,
        __v1338: *const (),
        __v1339: i32,
        __v1340: Option<unsafe extern "C-unwind" fn(*mut ())>,
    );
    fn sqlite3_result_text16le(
        __v1341: *mut sqlite3_context,
        __v1342: *const (),
        __v1343: i32,
        __v1344: Option<unsafe extern "C-unwind" fn(*mut ())>,
    );
    fn sqlite3_result_text16be(
        __v1345: *mut sqlite3_context,
        __v1346: *const (),
        __v1347: i32,
        __v1348: Option<unsafe extern "C-unwind" fn(*mut ())>,
    );
    fn sqlite3_result_value(__v1349: *mut sqlite3_context, __v1350: *mut sqlite3_value);
    fn sqlite3_result_pointer(
        __v1351: *mut sqlite3_context,
        __v1352: *mut (),
        __v1353: *const i8,
        __v1354: Option<unsafe extern "C-unwind" fn(*mut ())>,
    );
    fn sqlite3_result_zeroblob(__v1355: *mut sqlite3_context, n: i32);
    fn sqlite3_result_zeroblob64(__v1357: *mut sqlite3_context, n: u64) -> i32;
    fn sqlite3_result_subtype(__v1359: *mut sqlite3_context, __v1360: u32);
    fn sqlite3_create_collation(
        __v1361: *mut sqlite3,
        zName: *const i8,
        eTextRep: i32,
        pArg: *mut (),
        xCompare: Option<
            unsafe extern "C-unwind" fn(*mut (), i32, *const (), i32, *const ()) -> i32,
        >,
    ) -> i32;
    fn sqlite3_create_collation_v2(
        __v1366: *mut sqlite3,
        zName: *const i8,
        eTextRep: i32,
        pArg: *mut (),
        xCompare: Option<
            unsafe extern "C-unwind" fn(*mut (), i32, *const (), i32, *const ()) -> i32,
        >,
        xDestroy: Option<unsafe extern "C-unwind" fn(*mut ())>,
    ) -> i32;
    fn sqlite3_create_collation16(
        __v1372: *mut sqlite3,
        zName: *const (),
        eTextRep: i32,
        pArg: *mut (),
        xCompare: Option<
            unsafe extern "C-unwind" fn(*mut (), i32, *const (), i32, *const ()) -> i32,
        >,
    ) -> i32;
    fn sqlite3_collation_needed(
        __v1377: *mut sqlite3,
        __v1378: *mut (),
        __v1379: Option<unsafe extern "C-unwind" fn(*mut (), *mut sqlite3, i32, *const i8)>,
    ) -> i32;
    fn sqlite3_collation_needed16(
        __v1380: *mut sqlite3,
        __v1381: *mut (),
        __v1382: Option<unsafe extern "C-unwind" fn(*mut (), *mut sqlite3, i32, *const ())>,
    ) -> i32;
    fn sqlite3_sleep(__v1383: i32) -> i32;
    fn sqlite3_get_autocommit(__v1384: *mut sqlite3) -> i32;
    fn sqlite3_db_handle(__v1385: *mut sqlite3_stmt) -> *mut sqlite3;
    fn sqlite3_db_name(db: *mut sqlite3, N: i32) -> *const i8;
    fn sqlite3_db_filename(db: *mut sqlite3, zDbName: *const i8) -> *const i8;
    fn sqlite3_db_readonly(db: *mut sqlite3, zDbName: *const i8) -> i32;
    fn sqlite3_txn_state(__v1392: *mut sqlite3, zSchema: *const i8) -> i32;
    fn sqlite3_next_stmt(pDb: *mut sqlite3, pStmt: *mut sqlite3_stmt) -> *mut sqlite3_stmt;
    fn sqlite3_commit_hook(
        __v1396: *mut sqlite3,
        __v1397: Option<unsafe extern "C-unwind" fn(*mut ()) -> i32>,
        __v1398: *mut (),
    ) -> *mut ();
    fn sqlite3_rollback_hook(
        __v1399: *mut sqlite3,
        __v1400: Option<unsafe extern "C-unwind" fn(*mut ())>,
        __v1401: *mut (),
    ) -> *mut ();
    fn sqlite3_autovacuum_pages(
        db: *mut sqlite3,
        __v1403: Option<unsafe extern "C-unwind" fn(*mut (), *const i8, u32, u32, u32) -> u32>,
        __v1404: *mut (),
        __v1405: Option<unsafe extern "C-unwind" fn(*mut ())>,
    ) -> i32;
    fn sqlite3_update_hook(
        __v1406: *mut sqlite3,
        __v1407: Option<unsafe extern "C-unwind" fn(*mut (), i32, *const i8, *const i8, i64)>,
        __v1408: *mut (),
    ) -> *mut ();
    fn sqlite3_enable_shared_cache(__v1409: i32) -> i32;
    fn sqlite3_release_memory(__v1410: i32) -> i32;
    fn sqlite3_db_release_memory(__v1411: *mut sqlite3) -> i32;
    fn sqlite3_soft_heap_limit64(N: i64) -> i64;
    fn sqlite3_hard_heap_limit64(N: i64) -> i64;
    fn sqlite3_soft_heap_limit(N: i32);
    fn sqlite3_table_column_metadata(
        db: *mut sqlite3,
        zDbName: *const i8,
        zTableName: *const i8,
        zColumnName: *const i8,
        pzDataType: *mut *const i8,
        pzCollSeq: *mut *const i8,
        pNotNull: *mut i32,
        pPrimaryKey: *mut i32,
        pAutoinc: *mut i32,
    ) -> i32;
    fn sqlite3_create_module(
        db: *mut sqlite3,
        zName: *const i8,
        p: *const sqlite3_module,
        pClientData: *mut (),
    ) -> i32;
    fn sqlite3_create_module_v2(
        db: *mut sqlite3,
        zName: *const i8,
        p: *const sqlite3_module,
        pClientData: *mut (),
        xDestroy: Option<unsafe extern "C-unwind" fn(*mut ())>,
    ) -> i32;
    fn sqlite3_drop_modules(db: *mut sqlite3, azKeep: *mut *const i8) -> i32;
    fn sqlite3_declare_vtab(__v1443: *mut sqlite3, zSQL: *const i8) -> i32;
    fn sqlite3_overload_function(__v1445: *mut sqlite3, zFuncName: *const i8, nArg: i32) -> i32;
    fn sqlite3_blob_open(
        __v1448: *mut sqlite3,
        zDb: *const i8,
        zTable: *const i8,
        zColumn: *const i8,
        iRow: i64,
        flags: i32,
        ppBlob: *mut *mut sqlite3_blob,
    ) -> i32;
    fn sqlite3_blob_reopen(__v1455: *mut sqlite3_blob, __v1456: i64) -> i32;
    fn sqlite3_blob_close(__v1457: *mut sqlite3_blob) -> i32;
    fn sqlite3_blob_bytes(__v1458: *mut sqlite3_blob) -> i32;
    fn sqlite3_blob_read(__v1459: *mut sqlite3_blob, Z: *mut (), N: i32, iOffset: i32) -> i32;
    fn sqlite3_blob_write(__v1463: *mut sqlite3_blob, z: *const (), n: i32, iOffset: i32) -> i32;
    fn sqlite3_vfs_find(zVfsName: *const i8) -> *mut sqlite3_vfs;
    fn sqlite3_vfs_register(__v1468: *mut sqlite3_vfs, makeDflt: i32) -> i32;
    fn sqlite3_vfs_unregister(__v1470: *mut sqlite3_vfs) -> i32;
    fn sqlite3_mutex_alloc(__v1471: i32) -> *mut sqlite3_mutex;
    fn sqlite3_mutex_free(__v1472: *mut sqlite3_mutex);
    fn sqlite3_mutex_enter(__v1473: *mut sqlite3_mutex);
    fn sqlite3_mutex_try(__v1474: *mut sqlite3_mutex) -> i32;
    fn sqlite3_mutex_leave(__v1475: *mut sqlite3_mutex);
    fn sqlite3_db_mutex(__v1476: *mut sqlite3) -> *mut sqlite3_mutex;
    fn sqlite3_file_control(
        __v1477: *mut sqlite3,
        zDbName: *const i8,
        op: i32,
        __v1480: *mut (),
    ) -> i32;
    fn sqlite3_test_control(op: i32, ...) -> i32;
    fn sqlite3_keyword_count() -> i32;
    fn sqlite3_keyword_name(__v1482: i32, __v1483: *mut *const i8, __v1484: *mut i32) -> i32;
    fn sqlite3_keyword_check(__v1485: *const i8, __v1486: i32) -> i32;
    fn sqlite3_str_new(__v1487: *mut sqlite3) -> *mut sqlite3_str;
    fn sqlite3_str_finish(__v1488: *mut sqlite3_str) -> *mut i8;
    fn sqlite3_str_free(__v1489: *mut sqlite3_str);
    fn sqlite3_result_str(__v1490: *mut sqlite3_context, __v1491: *mut sqlite3_str, __v1492: i32);
    fn sqlite3_str_appendf(__v1493: *mut sqlite3_str, zFormat: *const i8, ...);
    fn sqlite3_str_vappendf(
        __v1495: *mut sqlite3_str,
        zFormat: *const i8,
        __v1497: core::ffi::VaList<'_>,
    );
    fn sqlite3_str_append(__v1498: *mut sqlite3_str, zIn: *const i8, N: i32);
    fn sqlite3_str_appendall(__v1501: *mut sqlite3_str, zIn: *const i8);
    fn sqlite3_str_appendchar(__v1503: *mut sqlite3_str, N: i32, C: i8);
    fn sqlite3_str_reset(__v1506: *mut sqlite3_str);
    fn sqlite3_str_truncate(__v1507: *mut sqlite3_str, N: i32);
    fn sqlite3_str_errcode(__v1509: *mut sqlite3_str) -> i32;
    fn sqlite3_str_length(__v1510: *mut sqlite3_str) -> i32;
    fn sqlite3_str_value(__v1511: *mut sqlite3_str) -> *mut i8;
    fn sqlite3_status(op: i32, pCurrent: *mut i32, pHighwater: *mut i32, resetFlag: i32) -> i32;
    fn sqlite3_status64(op: i32, pCurrent: *mut i64, pHighwater: *mut i64, resetFlag: i32) -> i32;
    fn sqlite3_db_status(
        __v1520: *mut sqlite3,
        op: i32,
        pCur: *mut i32,
        pHiwtr: *mut i32,
        resetFlg: i32,
    ) -> i32;
    fn sqlite3_db_status64(
        __v1525: *mut sqlite3,
        __v1526: i32,
        __v1527: *mut i64,
        __v1528: *mut i64,
        __v1529: i32,
    ) -> i32;
    fn sqlite3_stmt_status(__v1530: *mut sqlite3_stmt, op: i32, resetFlg: i32) -> i32;
    fn sqlite3_backup_init(
        pDest: *mut sqlite3,
        zDestName: *const i8,
        pSource: *mut sqlite3,
        zSourceName: *const i8,
    ) -> *mut sqlite3_backup;
    fn sqlite3_backup_step(p: *mut sqlite3_backup, nPage: i32) -> i32;
    fn sqlite3_backup_finish(p: *mut sqlite3_backup) -> i32;
    fn sqlite3_backup_remaining(p: *mut sqlite3_backup) -> i32;
    fn sqlite3_backup_pagecount(p: *mut sqlite3_backup) -> i32;
    fn sqlite3_stricmp(__v1542: *const i8, __v1543: *const i8) -> i32;
    fn sqlite3_strnicmp(__v1544: *const i8, __v1545: *const i8, __v1546: i32) -> i32;
    fn sqlite3_strglob(zGlob: *const i8, zStr: *const i8) -> i32;
    fn sqlite3_strlike(zGlob: *const i8, zStr: *const i8, cEsc: u32) -> i32;
    fn sqlite3_log(iErrCode: i32, zFormat: *const i8, ...);
    fn sqlite3_wal_hook(
        __v1554: *mut sqlite3,
        __v1555: Option<unsafe extern "C-unwind" fn(*mut (), *mut sqlite3, *const i8, i32) -> i32>,
        __v1556: *mut (),
    ) -> *mut ();
    fn sqlite3_wal_autocheckpoint(db: *mut sqlite3, N: i32) -> i32;
    fn sqlite3_wal_checkpoint(db: *mut sqlite3, zDb: *const i8) -> i32;
    fn sqlite3_wal_checkpoint_v2(
        db: *mut sqlite3,
        zDb: *const i8,
        eMode: i32,
        pnLog: *mut i32,
        pnCkpt: *mut i32,
    ) -> i32;
    fn sqlite3_vtab_config(__v1566: *mut sqlite3, op: i32, ...) -> i32;
    fn sqlite3_vtab_on_conflict(__v1568: *mut sqlite3) -> i32;
    fn sqlite3_vtab_nochange(__v1569: *mut sqlite3_context) -> i32;
    fn sqlite3_vtab_collation(__v1570: *mut sqlite3_index_info, __v1571: i32) -> *const i8;
    fn sqlite3_vtab_distinct(__v1572: *mut sqlite3_index_info) -> i32;
    fn sqlite3_vtab_in(__v1573: *mut sqlite3_index_info, iCons: i32, bHandle: i32) -> i32;
    fn sqlite3_vtab_in_first(pVal: *mut sqlite3_value, ppOut: *mut *mut sqlite3_value) -> i32;
    fn sqlite3_vtab_in_next(pVal: *mut sqlite3_value, ppOut: *mut *mut sqlite3_value) -> i32;
    fn sqlite3_vtab_rhs_value(
        __v1580: *mut sqlite3_index_info,
        __v1581: i32,
        ppVal: *mut *mut sqlite3_value,
    ) -> i32;
    fn sqlite3_db_cacheflush(__v1583: *mut sqlite3) -> i32;
    fn sqlite3_system_errno(__v1584: *mut sqlite3) -> i32;
    fn sqlite3_serialize(
        db: *mut sqlite3,
        zSchema: *const i8,
        piSize: *mut i64,
        mFlags: u32,
    ) -> *mut u8;
    fn sqlite3_deserialize(
        db: *mut sqlite3,
        zSchema: *const i8,
        pData: *mut u8,
        szDb: i64,
        szBuf: i64,
        mFlags: u32,
    ) -> i32;
    fn memcpy(__dest: *mut (), __src: *const (), __n: u64) -> *mut ();
    fn strlen(__s: *const i8) -> u64;
    fn sqlite3OsDlOpen(__v1599: *mut sqlite3_vfs, __v1600: *const i8) -> *mut ();
    fn sqlite3OsDlError(__v1601: *mut sqlite3_vfs, __v1602: i32, __v1603: *mut i8);
    fn sqlite3OsDlSym(
        __v1604: *mut sqlite3_vfs,
        __v1605: *mut (),
        __v1606: *const i8,
    ) -> Option<unsafe extern "C-unwind" fn()>;
    fn sqlite3OsDlClose(__v1607: *mut sqlite3_vfs, __v1608: *mut ());
    fn sqlite3Strlen30(__v1609: *const i8) -> i32;
    fn sqlite3DbMallocZero(__v1610: *mut sqlite3, __v1611: u64) -> *mut ();
    fn sqlite3DbFree(__v1612: *mut sqlite3, __v1613: *mut ());
    fn sqlite3MutexAlloc(__v1614: i32) -> *mut sqlite3_mutex;
    fn sqlite3ErrorWithMsg(__v1615: *mut sqlite3, __v1616: i32, __v1617: *const i8, ...);
    fn sqlite3ApiExit(db: *mut sqlite3, __v1619: i32) -> i32;
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
    trace: __SlateRecord169,
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
    u1: __SlateRecord170,
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
struct sqlite3_api_routines {
    aggregate_context: Option<unsafe extern "C-unwind" fn(*mut sqlite3_context, i32) -> *mut ()>,
    aggregate_count: Option<unsafe extern "C-unwind" fn(*mut sqlite3_context) -> i32>,
    bind_blob: Option<
        unsafe extern "C-unwind" fn(
            *mut sqlite3_stmt,
            i32,
            *const (),
            i32,
            Option<unsafe extern "C-unwind" fn(*mut ())>,
        ) -> i32,
    >,
    bind_double: Option<unsafe extern "C-unwind" fn(*mut sqlite3_stmt, i32, f64) -> i32>,
    bind_int: Option<unsafe extern "C-unwind" fn(*mut sqlite3_stmt, i32, i32) -> i32>,
    bind_int64: Option<unsafe extern "C-unwind" fn(*mut sqlite3_stmt, i32, i64) -> i32>,
    bind_null: Option<unsafe extern "C-unwind" fn(*mut sqlite3_stmt, i32) -> i32>,
    bind_parameter_count: Option<unsafe extern "C-unwind" fn(*mut sqlite3_stmt) -> i32>,
    bind_parameter_index: Option<unsafe extern "C-unwind" fn(*mut sqlite3_stmt, *const i8) -> i32>,
    bind_parameter_name: Option<unsafe extern "C-unwind" fn(*mut sqlite3_stmt, i32) -> *const i8>,
    bind_text: Option<
        unsafe extern "C-unwind" fn(
            *mut sqlite3_stmt,
            i32,
            *const i8,
            i32,
            Option<unsafe extern "C-unwind" fn(*mut ())>,
        ) -> i32,
    >,
    bind_text16: Option<
        unsafe extern "C-unwind" fn(
            *mut sqlite3_stmt,
            i32,
            *const (),
            i32,
            Option<unsafe extern "C-unwind" fn(*mut ())>,
        ) -> i32,
    >,
    bind_value:
        Option<unsafe extern "C-unwind" fn(*mut sqlite3_stmt, i32, *const sqlite3_value) -> i32>,
    busy_handler: Option<
        unsafe extern "C-unwind" fn(
            *mut sqlite3,
            Option<unsafe extern "C-unwind" fn(*mut (), i32) -> i32>,
            *mut (),
        ) -> i32,
    >,
    busy_timeout: Option<unsafe extern "C-unwind" fn(*mut sqlite3, i32) -> i32>,
    changes: Option<unsafe extern "C-unwind" fn(*mut sqlite3) -> i32>,
    close: Option<unsafe extern "C-unwind" fn(*mut sqlite3) -> i32>,
    collation_needed: Option<
        unsafe extern "C-unwind" fn(
            *mut sqlite3,
            *mut (),
            Option<unsafe extern "C-unwind" fn(*mut (), *mut sqlite3, i32, *const i8)>,
        ) -> i32,
    >,
    collation_needed16: Option<
        unsafe extern "C-unwind" fn(
            *mut sqlite3,
            *mut (),
            Option<unsafe extern "C-unwind" fn(*mut (), *mut sqlite3, i32, *const ())>,
        ) -> i32,
    >,
    column_blob: Option<unsafe extern "C-unwind" fn(*mut sqlite3_stmt, i32) -> *const ()>,
    column_bytes: Option<unsafe extern "C-unwind" fn(*mut sqlite3_stmt, i32) -> i32>,
    column_bytes16: Option<unsafe extern "C-unwind" fn(*mut sqlite3_stmt, i32) -> i32>,
    column_count: Option<unsafe extern "C-unwind" fn(*mut sqlite3_stmt) -> i32>,
    column_database_name: Option<unsafe extern "C-unwind" fn(*mut sqlite3_stmt, i32) -> *const i8>,
    column_database_name16:
        Option<unsafe extern "C-unwind" fn(*mut sqlite3_stmt, i32) -> *const ()>,
    column_decltype: Option<unsafe extern "C-unwind" fn(*mut sqlite3_stmt, i32) -> *const i8>,
    column_decltype16: Option<unsafe extern "C-unwind" fn(*mut sqlite3_stmt, i32) -> *const ()>,
    column_double: Option<unsafe extern "C-unwind" fn(*mut sqlite3_stmt, i32) -> f64>,
    column_int: Option<unsafe extern "C-unwind" fn(*mut sqlite3_stmt, i32) -> i32>,
    column_int64: Option<unsafe extern "C-unwind" fn(*mut sqlite3_stmt, i32) -> i64>,
    column_name: Option<unsafe extern "C-unwind" fn(*mut sqlite3_stmt, i32) -> *const i8>,
    column_name16: Option<unsafe extern "C-unwind" fn(*mut sqlite3_stmt, i32) -> *const ()>,
    column_origin_name: Option<unsafe extern "C-unwind" fn(*mut sqlite3_stmt, i32) -> *const i8>,
    column_origin_name16: Option<unsafe extern "C-unwind" fn(*mut sqlite3_stmt, i32) -> *const ()>,
    column_table_name: Option<unsafe extern "C-unwind" fn(*mut sqlite3_stmt, i32) -> *const i8>,
    column_table_name16: Option<unsafe extern "C-unwind" fn(*mut sqlite3_stmt, i32) -> *const ()>,
    column_text: Option<unsafe extern "C-unwind" fn(*mut sqlite3_stmt, i32) -> *const u8>,
    column_text16: Option<unsafe extern "C-unwind" fn(*mut sqlite3_stmt, i32) -> *const ()>,
    column_type: Option<unsafe extern "C-unwind" fn(*mut sqlite3_stmt, i32) -> i32>,
    column_value: Option<unsafe extern "C-unwind" fn(*mut sqlite3_stmt, i32) -> *mut sqlite3_value>,
    commit_hook: Option<
        unsafe extern "C-unwind" fn(
            *mut sqlite3,
            Option<unsafe extern "C-unwind" fn(*mut ()) -> i32>,
            *mut (),
        ) -> *mut (),
    >,
    complete: Option<unsafe extern "C-unwind" fn(*const i8) -> i32>,
    complete16: Option<unsafe extern "C-unwind" fn(*const ()) -> i32>,
    create_collation: Option<
        unsafe extern "C-unwind" fn(
            *mut sqlite3,
            *const i8,
            i32,
            *mut (),
            Option<unsafe extern "C-unwind" fn(*mut (), i32, *const (), i32, *const ()) -> i32>,
        ) -> i32,
    >,
    create_collation16: Option<
        unsafe extern "C-unwind" fn(
            *mut sqlite3,
            *const (),
            i32,
            *mut (),
            Option<unsafe extern "C-unwind" fn(*mut (), i32, *const (), i32, *const ()) -> i32>,
        ) -> i32,
    >,
    create_function: Option<
        unsafe extern "C-unwind" fn(
            *mut sqlite3,
            *const i8,
            i32,
            i32,
            *mut (),
            Option<unsafe extern "C-unwind" fn(*mut sqlite3_context, i32, *mut *mut sqlite3_value)>,
            Option<unsafe extern "C-unwind" fn(*mut sqlite3_context, i32, *mut *mut sqlite3_value)>,
            Option<unsafe extern "C-unwind" fn(*mut sqlite3_context)>,
        ) -> i32,
    >,
    create_function16: Option<
        unsafe extern "C-unwind" fn(
            *mut sqlite3,
            *const (),
            i32,
            i32,
            *mut (),
            Option<unsafe extern "C-unwind" fn(*mut sqlite3_context, i32, *mut *mut sqlite3_value)>,
            Option<unsafe extern "C-unwind" fn(*mut sqlite3_context, i32, *mut *mut sqlite3_value)>,
            Option<unsafe extern "C-unwind" fn(*mut sqlite3_context)>,
        ) -> i32,
    >,
    create_module: Option<
        unsafe extern "C-unwind" fn(*mut sqlite3, *const i8, *const sqlite3_module, *mut ()) -> i32,
    >,
    data_count: Option<unsafe extern "C-unwind" fn(*mut sqlite3_stmt) -> i32>,
    db_handle: Option<unsafe extern "C-unwind" fn(*mut sqlite3_stmt) -> *mut sqlite3>,
    declare_vtab: Option<unsafe extern "C-unwind" fn(*mut sqlite3, *const i8) -> i32>,
    enable_shared_cache: Option<unsafe extern "C-unwind" fn(i32) -> i32>,
    errcode: Option<unsafe extern "C-unwind" fn(*mut sqlite3) -> i32>,
    errmsg: Option<unsafe extern "C-unwind" fn(*mut sqlite3) -> *const i8>,
    errmsg16: Option<unsafe extern "C-unwind" fn(*mut sqlite3) -> *const ()>,
    exec: Option<
        unsafe extern "C-unwind" fn(
            *mut sqlite3,
            *const i8,
            Option<unsafe extern "C-unwind" fn(*mut (), i32, *mut *mut i8, *mut *mut i8) -> i32>,
            *mut (),
            *mut *mut i8,
        ) -> i32,
    >,
    expired: Option<unsafe extern "C-unwind" fn(*mut sqlite3_stmt) -> i32>,
    finalize: Option<unsafe extern "C-unwind" fn(*mut sqlite3_stmt) -> i32>,
    free: Option<unsafe extern "C-unwind" fn(*mut ())>,
    free_table: Option<unsafe extern "C-unwind" fn(*mut *mut i8)>,
    get_autocommit: Option<unsafe extern "C-unwind" fn(*mut sqlite3) -> i32>,
    get_auxdata: Option<unsafe extern "C-unwind" fn(*mut sqlite3_context, i32) -> *mut ()>,
    get_table: Option<
        unsafe extern "C-unwind" fn(
            *mut sqlite3,
            *const i8,
            *mut *mut *mut i8,
            *mut i32,
            *mut i32,
            *mut *mut i8,
        ) -> i32,
    >,
    global_recover: Option<unsafe extern "C-unwind" fn() -> i32>,
    interruptx: Option<unsafe extern "C-unwind" fn(*mut sqlite3)>,
    last_insert_rowid: Option<unsafe extern "C-unwind" fn(*mut sqlite3) -> i64>,
    libversion: Option<unsafe extern "C-unwind" fn() -> *const i8>,
    libversion_number: Option<unsafe extern "C-unwind" fn() -> i32>,
    malloc: Option<unsafe extern "C-unwind" fn(i32) -> *mut ()>,
    mprintf: Option<unsafe extern "C-unwind" fn(*const i8, ...) -> *mut i8>,
    open: Option<unsafe extern "C-unwind" fn(*const i8, *mut *mut sqlite3) -> i32>,
    open16: Option<unsafe extern "C-unwind" fn(*const (), *mut *mut sqlite3) -> i32>,
    prepare: Option<
        unsafe extern "C-unwind" fn(
            *mut sqlite3,
            *const i8,
            i32,
            *mut *mut sqlite3_stmt,
            *mut *const i8,
        ) -> i32,
    >,
    prepare16: Option<
        unsafe extern "C-unwind" fn(
            *mut sqlite3,
            *const (),
            i32,
            *mut *mut sqlite3_stmt,
            *mut *const (),
        ) -> i32,
    >,
    profile: Option<
        unsafe extern "C-unwind" fn(
            *mut sqlite3,
            Option<unsafe extern "C-unwind" fn(*mut (), *const i8, u64)>,
            *mut (),
        ) -> *mut (),
    >,
    progress_handler: Option<
        unsafe extern "C-unwind" fn(
            *mut sqlite3,
            i32,
            Option<unsafe extern "C-unwind" fn(*mut ()) -> i32>,
            *mut (),
        ),
    >,
    realloc: Option<unsafe extern "C-unwind" fn(*mut (), i32) -> *mut ()>,
    reset: Option<unsafe extern "C-unwind" fn(*mut sqlite3_stmt) -> i32>,
    result_blob: Option<
        unsafe extern "C-unwind" fn(
            *mut sqlite3_context,
            *const (),
            i32,
            Option<unsafe extern "C-unwind" fn(*mut ())>,
        ),
    >,
    result_double: Option<unsafe extern "C-unwind" fn(*mut sqlite3_context, f64)>,
    result_error: Option<unsafe extern "C-unwind" fn(*mut sqlite3_context, *const i8, i32)>,
    result_error16: Option<unsafe extern "C-unwind" fn(*mut sqlite3_context, *const (), i32)>,
    result_int: Option<unsafe extern "C-unwind" fn(*mut sqlite3_context, i32)>,
    result_int64: Option<unsafe extern "C-unwind" fn(*mut sqlite3_context, i64)>,
    result_null: Option<unsafe extern "C-unwind" fn(*mut sqlite3_context)>,
    result_text: Option<
        unsafe extern "C-unwind" fn(
            *mut sqlite3_context,
            *const i8,
            i32,
            Option<unsafe extern "C-unwind" fn(*mut ())>,
        ),
    >,
    result_text16: Option<
        unsafe extern "C-unwind" fn(
            *mut sqlite3_context,
            *const (),
            i32,
            Option<unsafe extern "C-unwind" fn(*mut ())>,
        ),
    >,
    result_text16be: Option<
        unsafe extern "C-unwind" fn(
            *mut sqlite3_context,
            *const (),
            i32,
            Option<unsafe extern "C-unwind" fn(*mut ())>,
        ),
    >,
    result_text16le: Option<
        unsafe extern "C-unwind" fn(
            *mut sqlite3_context,
            *const (),
            i32,
            Option<unsafe extern "C-unwind" fn(*mut ())>,
        ),
    >,
    result_value: Option<unsafe extern "C-unwind" fn(*mut sqlite3_context, *mut sqlite3_value)>,
    rollback_hook: Option<
        unsafe extern "C-unwind" fn(
            *mut sqlite3,
            Option<unsafe extern "C-unwind" fn(*mut ())>,
            *mut (),
        ) -> *mut (),
    >,
    set_authorizer: Option<
        unsafe extern "C-unwind" fn(
            *mut sqlite3,
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
            *mut (),
        ) -> i32,
    >,
    set_auxdata: Option<
        unsafe extern "C-unwind" fn(
            *mut sqlite3_context,
            i32,
            *mut (),
            Option<unsafe extern "C-unwind" fn(*mut ())>,
        ),
    >,
    xsnprintf: Option<unsafe extern "C-unwind" fn(i32, *mut i8, *const i8, ...) -> *mut i8>,
    step: Option<unsafe extern "C-unwind" fn(*mut sqlite3_stmt) -> i32>,
    table_column_metadata: Option<
        unsafe extern "C-unwind" fn(
            *mut sqlite3,
            *const i8,
            *const i8,
            *const i8,
            *mut *const i8,
            *mut *const i8,
            *mut i32,
            *mut i32,
            *mut i32,
        ) -> i32,
    >,
    thread_cleanup: Option<unsafe extern "C-unwind" fn()>,
    total_changes: Option<unsafe extern "C-unwind" fn(*mut sqlite3) -> i32>,
    trace: Option<
        unsafe extern "C-unwind" fn(
            *mut sqlite3,
            Option<unsafe extern "C-unwind" fn(*mut (), *const i8)>,
            *mut (),
        ) -> *mut (),
    >,
    transfer_bindings:
        Option<unsafe extern "C-unwind" fn(*mut sqlite3_stmt, *mut sqlite3_stmt) -> i32>,
    update_hook: Option<
        unsafe extern "C-unwind" fn(
            *mut sqlite3,
            Option<unsafe extern "C-unwind" fn(*mut (), i32, *const i8, *const i8, i64)>,
            *mut (),
        ) -> *mut (),
    >,
    user_data: Option<unsafe extern "C-unwind" fn(*mut sqlite3_context) -> *mut ()>,
    value_blob: Option<unsafe extern "C-unwind" fn(*mut sqlite3_value) -> *const ()>,
    value_bytes: Option<unsafe extern "C-unwind" fn(*mut sqlite3_value) -> i32>,
    value_bytes16: Option<unsafe extern "C-unwind" fn(*mut sqlite3_value) -> i32>,
    value_double: Option<unsafe extern "C-unwind" fn(*mut sqlite3_value) -> f64>,
    value_int: Option<unsafe extern "C-unwind" fn(*mut sqlite3_value) -> i32>,
    value_int64: Option<unsafe extern "C-unwind" fn(*mut sqlite3_value) -> i64>,
    value_numeric_type: Option<unsafe extern "C-unwind" fn(*mut sqlite3_value) -> i32>,
    value_text: Option<unsafe extern "C-unwind" fn(*mut sqlite3_value) -> *const u8>,
    value_text16: Option<unsafe extern "C-unwind" fn(*mut sqlite3_value) -> *const ()>,
    value_text16be: Option<unsafe extern "C-unwind" fn(*mut sqlite3_value) -> *const ()>,
    value_text16le: Option<unsafe extern "C-unwind" fn(*mut sqlite3_value) -> *const ()>,
    value_type: Option<unsafe extern "C-unwind" fn(*mut sqlite3_value) -> i32>,
    vmprintf: Option<unsafe extern "C-unwind" fn(*const i8, core::ffi::VaList<'_>) -> *mut i8>,
    overload_function: Option<unsafe extern "C-unwind" fn(*mut sqlite3, *const i8, i32) -> i32>,
    prepare_v2: Option<
        unsafe extern "C-unwind" fn(
            *mut sqlite3,
            *const i8,
            i32,
            *mut *mut sqlite3_stmt,
            *mut *const i8,
        ) -> i32,
    >,
    prepare16_v2: Option<
        unsafe extern "C-unwind" fn(
            *mut sqlite3,
            *const (),
            i32,
            *mut *mut sqlite3_stmt,
            *mut *const (),
        ) -> i32,
    >,
    clear_bindings: Option<unsafe extern "C-unwind" fn(*mut sqlite3_stmt) -> i32>,
    create_module_v2: Option<
        unsafe extern "C-unwind" fn(
            *mut sqlite3,
            *const i8,
            *const sqlite3_module,
            *mut (),
            Option<unsafe extern "C-unwind" fn(*mut ())>,
        ) -> i32,
    >,
    bind_zeroblob: Option<unsafe extern "C-unwind" fn(*mut sqlite3_stmt, i32, i32) -> i32>,
    blob_bytes: Option<unsafe extern "C-unwind" fn(*mut sqlite3_blob) -> i32>,
    blob_close: Option<unsafe extern "C-unwind" fn(*mut sqlite3_blob) -> i32>,
    blob_open: Option<
        unsafe extern "C-unwind" fn(
            *mut sqlite3,
            *const i8,
            *const i8,
            *const i8,
            i64,
            i32,
            *mut *mut sqlite3_blob,
        ) -> i32,
    >,
    blob_read: Option<unsafe extern "C-unwind" fn(*mut sqlite3_blob, *mut (), i32, i32) -> i32>,
    blob_write: Option<unsafe extern "C-unwind" fn(*mut sqlite3_blob, *const (), i32, i32) -> i32>,
    create_collation_v2: Option<
        unsafe extern "C-unwind" fn(
            *mut sqlite3,
            *const i8,
            i32,
            *mut (),
            Option<unsafe extern "C-unwind" fn(*mut (), i32, *const (), i32, *const ()) -> i32>,
            Option<unsafe extern "C-unwind" fn(*mut ())>,
        ) -> i32,
    >,
    file_control: Option<unsafe extern "C-unwind" fn(*mut sqlite3, *const i8, i32, *mut ()) -> i32>,
    memory_highwater: Option<unsafe extern "C-unwind" fn(i32) -> i64>,
    memory_used: Option<unsafe extern "C-unwind" fn() -> i64>,
    mutex_alloc: Option<unsafe extern "C-unwind" fn(i32) -> *mut sqlite3_mutex>,
    mutex_enter: Option<unsafe extern "C-unwind" fn(*mut sqlite3_mutex)>,
    mutex_free: Option<unsafe extern "C-unwind" fn(*mut sqlite3_mutex)>,
    mutex_leave: Option<unsafe extern "C-unwind" fn(*mut sqlite3_mutex)>,
    mutex_try: Option<unsafe extern "C-unwind" fn(*mut sqlite3_mutex) -> i32>,
    open_v2:
        Option<unsafe extern "C-unwind" fn(*const i8, *mut *mut sqlite3, i32, *const i8) -> i32>,
    release_memory: Option<unsafe extern "C-unwind" fn(i32) -> i32>,
    result_error_nomem: Option<unsafe extern "C-unwind" fn(*mut sqlite3_context)>,
    result_error_toobig: Option<unsafe extern "C-unwind" fn(*mut sqlite3_context)>,
    sleep: Option<unsafe extern "C-unwind" fn(i32) -> i32>,
    soft_heap_limit: Option<unsafe extern "C-unwind" fn(i32)>,
    vfs_find: Option<unsafe extern "C-unwind" fn(*const i8) -> *mut sqlite3_vfs>,
    vfs_register: Option<unsafe extern "C-unwind" fn(*mut sqlite3_vfs, i32) -> i32>,
    vfs_unregister: Option<unsafe extern "C-unwind" fn(*mut sqlite3_vfs) -> i32>,
    xthreadsafe: Option<unsafe extern "C-unwind" fn() -> i32>,
    result_zeroblob: Option<unsafe extern "C-unwind" fn(*mut sqlite3_context, i32)>,
    result_error_code: Option<unsafe extern "C-unwind" fn(*mut sqlite3_context, i32)>,
    test_control: Option<unsafe extern "C-unwind" fn(i32, ...) -> i32>,
    randomness: Option<unsafe extern "C-unwind" fn(i32, *mut ())>,
    context_db_handle: Option<unsafe extern "C-unwind" fn(*mut sqlite3_context) -> *mut sqlite3>,
    extended_result_codes: Option<unsafe extern "C-unwind" fn(*mut sqlite3, i32) -> i32>,
    limit: Option<unsafe extern "C-unwind" fn(*mut sqlite3, i32, i32) -> i32>,
    next_stmt:
        Option<unsafe extern "C-unwind" fn(*mut sqlite3, *mut sqlite3_stmt) -> *mut sqlite3_stmt>,
    sql: Option<unsafe extern "C-unwind" fn(*mut sqlite3_stmt) -> *const i8>,
    status: Option<unsafe extern "C-unwind" fn(i32, *mut i32, *mut i32, i32) -> i32>,
    backup_finish: Option<unsafe extern "C-unwind" fn(*mut sqlite3_backup) -> i32>,
    backup_init: Option<
        unsafe extern "C-unwind" fn(
            *mut sqlite3,
            *const i8,
            *mut sqlite3,
            *const i8,
        ) -> *mut sqlite3_backup,
    >,
    backup_pagecount: Option<unsafe extern "C-unwind" fn(*mut sqlite3_backup) -> i32>,
    backup_remaining: Option<unsafe extern "C-unwind" fn(*mut sqlite3_backup) -> i32>,
    backup_step: Option<unsafe extern "C-unwind" fn(*mut sqlite3_backup, i32) -> i32>,
    compileoption_get: Option<unsafe extern "C-unwind" fn(i32) -> *const i8>,
    compileoption_used: Option<unsafe extern "C-unwind" fn(*const i8) -> i32>,
    create_function_v2: Option<
        unsafe extern "C-unwind" fn(
            *mut sqlite3,
            *const i8,
            i32,
            i32,
            *mut (),
            Option<unsafe extern "C-unwind" fn(*mut sqlite3_context, i32, *mut *mut sqlite3_value)>,
            Option<unsafe extern "C-unwind" fn(*mut sqlite3_context, i32, *mut *mut sqlite3_value)>,
            Option<unsafe extern "C-unwind" fn(*mut sqlite3_context)>,
            Option<unsafe extern "C-unwind" fn(*mut ())>,
        ) -> i32,
    >,
    db_config: Option<unsafe extern "C-unwind" fn(*mut sqlite3, i32, ...) -> i32>,
    db_mutex: Option<unsafe extern "C-unwind" fn(*mut sqlite3) -> *mut sqlite3_mutex>,
    db_status:
        Option<unsafe extern "C-unwind" fn(*mut sqlite3, i32, *mut i32, *mut i32, i32) -> i32>,
    extended_errcode: Option<unsafe extern "C-unwind" fn(*mut sqlite3) -> i32>,
    log: Option<unsafe extern "C-unwind" fn(i32, *const i8, ...)>,
    soft_heap_limit64: Option<unsafe extern "C-unwind" fn(i64) -> i64>,
    sourceid: Option<unsafe extern "C-unwind" fn() -> *const i8>,
    stmt_status: Option<unsafe extern "C-unwind" fn(*mut sqlite3_stmt, i32, i32) -> i32>,
    strnicmp: Option<unsafe extern "C-unwind" fn(*const i8, *const i8, i32) -> i32>,
    unlock_notify: Option<
        unsafe extern "C-unwind" fn(
            *mut sqlite3,
            Option<unsafe extern "C-unwind" fn(*mut *mut (), i32)>,
            *mut (),
        ) -> i32,
    >,
    wal_autocheckpoint: Option<unsafe extern "C-unwind" fn(*mut sqlite3, i32) -> i32>,
    wal_checkpoint: Option<unsafe extern "C-unwind" fn(*mut sqlite3, *const i8) -> i32>,
    wal_hook: Option<
        unsafe extern "C-unwind" fn(
            *mut sqlite3,
            Option<unsafe extern "C-unwind" fn(*mut (), *mut sqlite3, *const i8, i32) -> i32>,
            *mut (),
        ) -> *mut (),
    >,
    blob_reopen: Option<unsafe extern "C-unwind" fn(*mut sqlite3_blob, i64) -> i32>,
    vtab_config: Option<unsafe extern "C-unwind" fn(*mut sqlite3, i32, ...) -> i32>,
    vtab_on_conflict: Option<unsafe extern "C-unwind" fn(*mut sqlite3) -> i32>,
    close_v2: Option<unsafe extern "C-unwind" fn(*mut sqlite3) -> i32>,
    db_filename: Option<unsafe extern "C-unwind" fn(*mut sqlite3, *const i8) -> *const i8>,
    db_readonly: Option<unsafe extern "C-unwind" fn(*mut sqlite3, *const i8) -> i32>,
    db_release_memory: Option<unsafe extern "C-unwind" fn(*mut sqlite3) -> i32>,
    errstr: Option<unsafe extern "C-unwind" fn(i32) -> *const i8>,
    stmt_busy: Option<unsafe extern "C-unwind" fn(*mut sqlite3_stmt) -> i32>,
    stmt_readonly: Option<unsafe extern "C-unwind" fn(*mut sqlite3_stmt) -> i32>,
    stricmp: Option<unsafe extern "C-unwind" fn(*const i8, *const i8) -> i32>,
    uri_boolean: Option<unsafe extern "C-unwind" fn(*const i8, *const i8, i32) -> i32>,
    uri_int64: Option<unsafe extern "C-unwind" fn(*const i8, *const i8, i64) -> i64>,
    uri_parameter: Option<unsafe extern "C-unwind" fn(*const i8, *const i8) -> *const i8>,
    xvsnprintf: Option<
        unsafe extern "C-unwind" fn(i32, *mut i8, *const i8, core::ffi::VaList<'_>) -> *mut i8,
    >,
    wal_checkpoint_v2: Option<
        unsafe extern "C-unwind" fn(*mut sqlite3, *const i8, i32, *mut i32, *mut i32) -> i32,
    >,
    auto_extension:
        Option<unsafe extern "C-unwind" fn(Option<unsafe extern "C-unwind" fn()>) -> i32>,
    bind_blob64: Option<
        unsafe extern "C-unwind" fn(
            *mut sqlite3_stmt,
            i32,
            *const (),
            u64,
            Option<unsafe extern "C-unwind" fn(*mut ())>,
        ) -> i32,
    >,
    bind_text64: Option<
        unsafe extern "C-unwind" fn(
            *mut sqlite3_stmt,
            i32,
            *const i8,
            u64,
            Option<unsafe extern "C-unwind" fn(*mut ())>,
            u8,
        ) -> i32,
    >,
    cancel_auto_extension:
        Option<unsafe extern "C-unwind" fn(Option<unsafe extern "C-unwind" fn()>) -> i32>,
    load_extension: Option<
        unsafe extern "C-unwind" fn(*mut sqlite3, *const i8, *const i8, *mut *mut i8) -> i32,
    >,
    malloc64: Option<unsafe extern "C-unwind" fn(u64) -> *mut ()>,
    msize: Option<unsafe extern "C-unwind" fn(*mut ()) -> u64>,
    realloc64: Option<unsafe extern "C-unwind" fn(*mut (), u64) -> *mut ()>,
    reset_auto_extension: Option<unsafe extern "C-unwind" fn()>,
    result_blob64: Option<
        unsafe extern "C-unwind" fn(
            *mut sqlite3_context,
            *const (),
            u64,
            Option<unsafe extern "C-unwind" fn(*mut ())>,
        ),
    >,
    result_text64: Option<
        unsafe extern "C-unwind" fn(
            *mut sqlite3_context,
            *const i8,
            u64,
            Option<unsafe extern "C-unwind" fn(*mut ())>,
            u8,
        ),
    >,
    strglob: Option<unsafe extern "C-unwind" fn(*const i8, *const i8) -> i32>,
    value_dup: Option<unsafe extern "C-unwind" fn(*const sqlite3_value) -> *mut sqlite3_value>,
    value_free: Option<unsafe extern "C-unwind" fn(*mut sqlite3_value)>,
    result_zeroblob64: Option<unsafe extern "C-unwind" fn(*mut sqlite3_context, u64) -> i32>,
    bind_zeroblob64: Option<unsafe extern "C-unwind" fn(*mut sqlite3_stmt, i32, u64) -> i32>,
    value_subtype: Option<unsafe extern "C-unwind" fn(*mut sqlite3_value) -> u32>,
    result_subtype: Option<unsafe extern "C-unwind" fn(*mut sqlite3_context, u32)>,
    status64: Option<unsafe extern "C-unwind" fn(i32, *mut i64, *mut i64, i32) -> i32>,
    strlike: Option<unsafe extern "C-unwind" fn(*const i8, *const i8, u32) -> i32>,
    db_cacheflush: Option<unsafe extern "C-unwind" fn(*mut sqlite3) -> i32>,
    system_errno: Option<unsafe extern "C-unwind" fn(*mut sqlite3) -> i32>,
    trace_v2: Option<
        unsafe extern "C-unwind" fn(
            *mut sqlite3,
            u32,
            Option<unsafe extern "C-unwind" fn(u32, *mut (), *mut (), *mut ()) -> i32>,
            *mut (),
        ) -> i32,
    >,
    expanded_sql: Option<unsafe extern "C-unwind" fn(*mut sqlite3_stmt) -> *mut i8>,
    set_last_insert_rowid: Option<unsafe extern "C-unwind" fn(*mut sqlite3, i64)>,
    prepare_v3: Option<
        unsafe extern "C-unwind" fn(
            *mut sqlite3,
            *const i8,
            i32,
            u32,
            *mut *mut sqlite3_stmt,
            *mut *const i8,
        ) -> i32,
    >,
    prepare16_v3: Option<
        unsafe extern "C-unwind" fn(
            *mut sqlite3,
            *const (),
            i32,
            u32,
            *mut *mut sqlite3_stmt,
            *mut *const (),
        ) -> i32,
    >,
    bind_pointer: Option<
        unsafe extern "C-unwind" fn(
            *mut sqlite3_stmt,
            i32,
            *mut (),
            *const i8,
            Option<unsafe extern "C-unwind" fn(*mut ())>,
        ) -> i32,
    >,
    result_pointer: Option<
        unsafe extern "C-unwind" fn(
            *mut sqlite3_context,
            *mut (),
            *const i8,
            Option<unsafe extern "C-unwind" fn(*mut ())>,
        ),
    >,
    value_pointer: Option<unsafe extern "C-unwind" fn(*mut sqlite3_value, *const i8) -> *mut ()>,
    vtab_nochange: Option<unsafe extern "C-unwind" fn(*mut sqlite3_context) -> i32>,
    value_nochange: Option<unsafe extern "C-unwind" fn(*mut sqlite3_value) -> i32>,
    vtab_collation: Option<unsafe extern "C-unwind" fn(*mut sqlite3_index_info, i32) -> *const i8>,
    keyword_count: Option<unsafe extern "C-unwind" fn() -> i32>,
    keyword_name: Option<unsafe extern "C-unwind" fn(i32, *mut *const i8, *mut i32) -> i32>,
    keyword_check: Option<unsafe extern "C-unwind" fn(*const i8, i32) -> i32>,
    str_new: Option<unsafe extern "C-unwind" fn(*mut sqlite3) -> *mut sqlite3_str>,
    str_finish: Option<unsafe extern "C-unwind" fn(*mut sqlite3_str) -> *mut i8>,
    str_appendf: Option<unsafe extern "C-unwind" fn(*mut sqlite3_str, *const i8, ...)>,
    str_vappendf:
        Option<unsafe extern "C-unwind" fn(*mut sqlite3_str, *const i8, core::ffi::VaList<'_>)>,
    str_append: Option<unsafe extern "C-unwind" fn(*mut sqlite3_str, *const i8, i32)>,
    str_appendall: Option<unsafe extern "C-unwind" fn(*mut sqlite3_str, *const i8)>,
    str_appendchar: Option<unsafe extern "C-unwind" fn(*mut sqlite3_str, i32, i8)>,
    str_reset: Option<unsafe extern "C-unwind" fn(*mut sqlite3_str)>,
    str_errcode: Option<unsafe extern "C-unwind" fn(*mut sqlite3_str) -> i32>,
    str_length: Option<unsafe extern "C-unwind" fn(*mut sqlite3_str) -> i32>,
    str_value: Option<unsafe extern "C-unwind" fn(*mut sqlite3_str) -> *mut i8>,
    create_window_function: Option<
        unsafe extern "C-unwind" fn(
            *mut sqlite3,
            *const i8,
            i32,
            i32,
            *mut (),
            Option<unsafe extern "C-unwind" fn(*mut sqlite3_context, i32, *mut *mut sqlite3_value)>,
            Option<unsafe extern "C-unwind" fn(*mut sqlite3_context)>,
            Option<unsafe extern "C-unwind" fn(*mut sqlite3_context)>,
            Option<unsafe extern "C-unwind" fn(*mut sqlite3_context, i32, *mut *mut sqlite3_value)>,
            Option<unsafe extern "C-unwind" fn(*mut ())>,
        ) -> i32,
    >,
    normalized_sql: Option<unsafe extern "C-unwind" fn(*mut sqlite3_stmt) -> *const i8>,
    stmt_isexplain: Option<unsafe extern "C-unwind" fn(*mut sqlite3_stmt) -> i32>,
    value_frombind: Option<unsafe extern "C-unwind" fn(*mut sqlite3_value) -> i32>,
    drop_modules: Option<unsafe extern "C-unwind" fn(*mut sqlite3, *mut *const i8) -> i32>,
    hard_heap_limit64: Option<unsafe extern "C-unwind" fn(i64) -> i64>,
    uri_key: Option<unsafe extern "C-unwind" fn(*const i8, i32) -> *const i8>,
    filename_database: Option<unsafe extern "C-unwind" fn(*const i8) -> *const i8>,
    filename_journal: Option<unsafe extern "C-unwind" fn(*const i8) -> *const i8>,
    filename_wal: Option<unsafe extern "C-unwind" fn(*const i8) -> *const i8>,
    create_filename: Option<
        unsafe extern "C-unwind" fn(
            *const i8,
            *const i8,
            *const i8,
            i32,
            *mut *const i8,
        ) -> *const i8,
    >,
    free_filename: Option<unsafe extern "C-unwind" fn(*const i8)>,
    database_file_object: Option<unsafe extern "C-unwind" fn(*const i8) -> *mut sqlite3_file>,
    txn_state: Option<unsafe extern "C-unwind" fn(*mut sqlite3, *const i8) -> i32>,
    changes64: Option<unsafe extern "C-unwind" fn(*mut sqlite3) -> i64>,
    total_changes64: Option<unsafe extern "C-unwind" fn(*mut sqlite3) -> i64>,
    autovacuum_pages: Option<
        unsafe extern "C-unwind" fn(
            *mut sqlite3,
            Option<unsafe extern "C-unwind" fn(*mut (), *const i8, u32, u32, u32) -> u32>,
            *mut (),
            Option<unsafe extern "C-unwind" fn(*mut ())>,
        ) -> i32,
    >,
    error_offset: Option<unsafe extern "C-unwind" fn(*mut sqlite3) -> i32>,
    vtab_rhs_value: Option<
        unsafe extern "C-unwind" fn(*mut sqlite3_index_info, i32, *mut *mut sqlite3_value) -> i32,
    >,
    vtab_distinct: Option<unsafe extern "C-unwind" fn(*mut sqlite3_index_info) -> i32>,
    vtab_in: Option<unsafe extern "C-unwind" fn(*mut sqlite3_index_info, i32, i32) -> i32>,
    vtab_in_first:
        Option<unsafe extern "C-unwind" fn(*mut sqlite3_value, *mut *mut sqlite3_value) -> i32>,
    vtab_in_next:
        Option<unsafe extern "C-unwind" fn(*mut sqlite3_value, *mut *mut sqlite3_value) -> i32>,
    deserialize:
        Option<unsafe extern "C-unwind" fn(*mut sqlite3, *const i8, *mut u8, i64, i64, u32) -> i32>,
    serialize:
        Option<unsafe extern "C-unwind" fn(*mut sqlite3, *const i8, *mut i64, u32) -> *mut u8>,
    db_name: Option<unsafe extern "C-unwind" fn(*mut sqlite3, i32) -> *const i8>,
    value_encoding: Option<unsafe extern "C-unwind" fn(*mut sqlite3_value) -> i32>,
    is_interrupted: Option<unsafe extern "C-unwind" fn(*mut sqlite3) -> i32>,
    stmt_explain: Option<unsafe extern "C-unwind" fn(*mut sqlite3_stmt, i32) -> i32>,
    get_clientdata: Option<unsafe extern "C-unwind" fn(*mut sqlite3, *const i8) -> *mut ()>,
    set_clientdata: Option<
        unsafe extern "C-unwind" fn(
            *mut sqlite3,
            *const i8,
            *mut (),
            Option<unsafe extern "C-unwind" fn(*mut ())>,
        ) -> i32,
    >,
    setlk_timeout: Option<unsafe extern "C-unwind" fn(*mut sqlite3, i32, i32) -> i32>,
    set_errmsg: Option<unsafe extern "C-unwind" fn(*mut sqlite3, i32, *const i8) -> i32>,
    db_status64:
        Option<unsafe extern "C-unwind" fn(*mut sqlite3, i32, *mut i64, *mut i64, i32) -> i32>,
    str_truncate: Option<unsafe extern "C-unwind" fn(*mut sqlite3_str, i32)>,
    str_free: Option<unsafe extern "C-unwind" fn(*mut sqlite3_str)>,
    carray_bind: Option<
        unsafe extern "C-unwind" fn(
            *mut sqlite3_stmt,
            i32,
            *mut (),
            i32,
            i32,
            Option<unsafe extern "C-unwind" fn(*mut ())>,
        ) -> i32,
    >,
    carray_bind_v2: Option<
        unsafe extern "C-unwind" fn(
            *mut sqlite3_stmt,
            i32,
            *mut (),
            i32,
            i32,
            Option<unsafe extern "C-unwind" fn(*mut ())>,
            *mut (),
        ) -> i32,
    >,
    incomplete: Option<unsafe extern "C-unwind" fn(*const i8) -> i64>,
    result_str: Option<unsafe extern "C-unwind" fn(*mut sqlite3_context, *mut sqlite3_str, i32)>,
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
struct sqlite3_stmt {}

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
struct sqlite3_blob {}

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
struct sqlite3_backup {}

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
    __slate_bits_0: __slate_bits::__SlateBits76U0,
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
    u: __SlateRecord180,
    pLeft: *mut Expr,
    pRight: *mut Expr,
    x: __SlateRecord181,
    nHeight: i32,
    iTable: i32,
    iColumn: i16,
    iAgg: i16,
    w: __SlateRecord182,
    pAggInfo: *mut AggInfo,
    y: __SlateRecord183,
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
    u: __SlateRecord171,
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
    __slate_bits_0: __slate_bits::__SlateBits100U0,
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
    __slate_bits_0: __slate_bits::__SlateBits112U0,
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
    u1: __SlateRecord196,
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
    fg: __SlateRecord190,
    iCursor: i32,
    colUsed: u64,
    u1: __SlateRecord191,
    u2: __SlateRecord192,
    u3: __SlateRecord193,
    u4: __SlateRecord194,
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
    u: __SlateRecord172,
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
    __slate_bits_0: __slate_bits::__SlateBits168U0,
    azInit: *mut *const i8,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord169 {
    xLegacy: Option<unsafe extern "C-unwind" fn(*mut (), *const i8)>,
    xV2: Option<unsafe extern "C-unwind" fn(u32, *mut (), *mut (), *mut ()) -> i32>,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord170 {
    isInterrupted: i32,
    notUsed1: f64,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord171 {
    pHash: *mut FuncDef,
    pDestructor: *mut FuncDestructor,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord172 {
    tab: __SlateRecord173,
    view: __SlateRecord174,
    vtab: __SlateRecord175,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord173 {
    addColOffset: i32,
    pFKey: *mut FKey,
    pDfltList: *mut ExprList,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord174 {
    pSelect: *mut Select,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord175 {
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
union __SlateRecord180 {
    zToken: *mut i8,
    iValue: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord181 {
    pList: *mut ExprList,
    pSelect: *mut Select,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord182 {
    iJoin: i32,
    iOfst: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord183 {
    pTab: *mut Table,
    pWin: *mut Window,
    nReg: i32,
    sub: __SlateRecord184,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord184 {
    iAddr: i32,
    regReturn: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct ExprList_item {
    pExpr: *mut Expr,
    zEName: *mut i8,
    fg: __SlateRecord186,
    u: __SlateRecord187,
}

#[repr(C, align(4))]
#[derive(Clone, Copy)]
struct __SlateRecord186 {
    sortFlags: u8,
    __slate_bits_0: __slate_bits::__SlateBits186U0,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord187 {
    x: __SlateRecord188,
    iConstExprReg: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord188 {
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
struct __SlateRecord190 {
    jointype: u8,
    __slate_bits_0: __slate_bits::__SlateBits190U0,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord191 {
    zIndexedBy: *mut i8,
    pFuncArg: *mut ExprList,
    nRow: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord192 {
    pIBIndex: *mut Index,
    pCteUse: *mut CteUse,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord193 {
    pOn: *mut Expr,
    pUsing: *mut IdList,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord194 {
    pSchema: *mut Schema,
    zDatabase: *mut i8,
    pSubq: *mut Subquery,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord196 {
    cr: __SlateRecord197,
    d: __SlateRecord198,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord197 {
    addrCrTab: i32,
    regRowid: i32,
    regRoot: i32,
    constraintName: Token,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord198 {
    pReturning: *mut Returning,
}

// /* !defined(SQLITE_OMIT_LOAD_EXTENSION) */
// /*
// ** The following object holds the list of automatically loaded
// ** extensions.
// **
// ** This list is shared across threads.  The SQLITE_MUTEX_STATIC_MAIN
// ** mutex must be held while accessing this list.
// */
#[repr(C)]
#[derive(Clone, Copy)]
struct sqlite3AutoExtList {
    nExt: u32,
    // /* Number of entries in aExt[] */
    aExt: *mut Option<unsafe extern "C-unwind" fn()>,
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
    pub struct __SlateBits76U0 {
        #[bits(4)]
        pub notNull: u32,
        #[bits(4)]
        pub eCType: u32,
    }
    #[bitfields::bitfield([u8; 2], c_names = true, new = false, from_into_bits = false, from_traits = false, default = false, debug = false, builder = false, bit_ops = false)]
    pub struct __SlateBits186U0 {
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
    pub struct __SlateBits190U0 {
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
    pub struct __SlateBits100U0 {
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
    pub struct __SlateBits168U0 {
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
    pub struct __SlateBits112U0 {
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
// ** 2006 June 7
// **
// ** The author disclaims copyright to this source code.  In place of
// ** a legal notice, here is a blessing:
// **
// **    May you do good and not evil.
// **    May you find forgiveness for yourself and forgive others.
// **    May you share freely, never taking more than you give.
// **
// *************************************************************************
// ** This file contains code used to dynamically load extensions into
// ** the SQLite library.
// */
// /* Disable the API redefinition in sqlite3ext.h */
// /*
// ** Some API routines are omitted when various features are
// ** excluded from a build of SQLite.  Substitute a NULL pointer
// ** for any missing APIs.
// */
// /*
// ** The following structure contains pointers to all SQLite API routines.
// ** A pointer to this structure is passed into extensions when they are
// ** loaded so that the extension can make calls back into the SQLite
// ** library.
// **
// ** When adding new APIs, add them to the bottom of this structure
// ** in order to preserve backwards compatibility.
// **
// ** Extensions that use newer APIs should first call the
// ** sqlite3_libversion_number() to make sure that the API they
// ** intend to use is supported by the library.  Extensions should
// ** also check to make sure that the pointer to the function is
// ** not NULL before calling it.
// */
static mut sqlite3Apis: sqlite3_api_routines = sqlite3_api_routines {
    aggregate_context: unsafe {
        std::mem::transmute::<
            *const (),
            Option<unsafe extern "C-unwind" fn(*mut sqlite3_context, i32) -> *mut ()>,
        >(sqlite3_aggregate_context as *const ())
    },
    aggregate_count: unsafe {
        std::mem::transmute::<
            *const (),
            Option<unsafe extern "C-unwind" fn(*mut sqlite3_context) -> i32>,
        >(sqlite3_aggregate_count as *const ())
    },
    bind_blob: unsafe {
        std::mem::transmute::<
            *const (),
            Option<
                unsafe extern "C-unwind" fn(
                    *mut sqlite3_stmt,
                    i32,
                    *const (),
                    i32,
                    Option<unsafe extern "C-unwind" fn(*mut ())>,
                ) -> i32,
            >,
        >(sqlite3_bind_blob as *const ())
    },
    bind_double: unsafe {
        std::mem::transmute::<
            *const (),
            Option<unsafe extern "C-unwind" fn(*mut sqlite3_stmt, i32, f64) -> i32>,
        >(sqlite3_bind_double as *const ())
    },
    bind_int: unsafe {
        std::mem::transmute::<
            *const (),
            Option<unsafe extern "C-unwind" fn(*mut sqlite3_stmt, i32, i32) -> i32>,
        >(sqlite3_bind_int as *const ())
    },
    bind_int64: unsafe {
        std::mem::transmute::<
            *const (),
            Option<unsafe extern "C-unwind" fn(*mut sqlite3_stmt, i32, i64) -> i32>,
        >(sqlite3_bind_int64 as *const ())
    },
    bind_null: unsafe {
        std::mem::transmute::<
            *const (),
            Option<unsafe extern "C-unwind" fn(*mut sqlite3_stmt, i32) -> i32>,
        >(sqlite3_bind_null as *const ())
    },
    bind_parameter_count: unsafe {
        std::mem::transmute::<
            *const (),
            Option<unsafe extern "C-unwind" fn(*mut sqlite3_stmt) -> i32>,
        >(sqlite3_bind_parameter_count as *const ())
    },
    bind_parameter_index: unsafe {
        std::mem::transmute::<
            *const (),
            Option<unsafe extern "C-unwind" fn(*mut sqlite3_stmt, *const i8) -> i32>,
        >(sqlite3_bind_parameter_index as *const ())
    },
    bind_parameter_name: unsafe {
        std::mem::transmute::<
            *const (),
            Option<unsafe extern "C-unwind" fn(*mut sqlite3_stmt, i32) -> *const i8>,
        >(sqlite3_bind_parameter_name as *const ())
    },
    bind_text: unsafe {
        std::mem::transmute::<
            *const (),
            Option<
                unsafe extern "C-unwind" fn(
                    *mut sqlite3_stmt,
                    i32,
                    *const i8,
                    i32,
                    Option<unsafe extern "C-unwind" fn(*mut ())>,
                ) -> i32,
            >,
        >(sqlite3_bind_text as *const ())
    },
    bind_text16: unsafe {
        std::mem::transmute::<
            *const (),
            Option<
                unsafe extern "C-unwind" fn(
                    *mut sqlite3_stmt,
                    i32,
                    *const (),
                    i32,
                    Option<unsafe extern "C-unwind" fn(*mut ())>,
                ) -> i32,
            >,
        >(sqlite3_bind_text16 as *const ())
    },
    bind_value: unsafe {
        std::mem::transmute::<
            *const (),
            Option<
                unsafe extern "C-unwind" fn(*mut sqlite3_stmt, i32, *const sqlite3_value) -> i32,
            >,
        >(sqlite3_bind_value as *const ())
    },
    busy_handler: unsafe {
        std::mem::transmute::<
            *const (),
            Option<
                unsafe extern "C-unwind" fn(
                    *mut sqlite3,
                    Option<unsafe extern "C-unwind" fn(*mut (), i32) -> i32>,
                    *mut (),
                ) -> i32,
            >,
        >(sqlite3_busy_handler as *const ())
    },
    busy_timeout: unsafe {
        std::mem::transmute::<
            *const (),
            Option<unsafe extern "C-unwind" fn(*mut sqlite3, i32) -> i32>,
        >(sqlite3_busy_timeout as *const ())
    },
    changes: unsafe {
        std::mem::transmute::<*const (), Option<unsafe extern "C-unwind" fn(*mut sqlite3) -> i32>>(
            sqlite3_changes as *const (),
        )
    },
    close: unsafe {
        std::mem::transmute::<*const (), Option<unsafe extern "C-unwind" fn(*mut sqlite3) -> i32>>(
            sqlite3_close as *const (),
        )
    },
    collation_needed: unsafe {
        std::mem::transmute::<
            *const (),
            Option<
                unsafe extern "C-unwind" fn(
                    *mut sqlite3,
                    *mut (),
                    Option<unsafe extern "C-unwind" fn(*mut (), *mut sqlite3, i32, *const i8)>,
                ) -> i32,
            >,
        >(sqlite3_collation_needed as *const ())
    },
    collation_needed16: unsafe {
        std::mem::transmute::<
            *const (),
            Option<
                unsafe extern "C-unwind" fn(
                    *mut sqlite3,
                    *mut (),
                    Option<unsafe extern "C-unwind" fn(*mut (), *mut sqlite3, i32, *const ())>,
                ) -> i32,
            >,
        >(sqlite3_collation_needed16 as *const ())
    },
    column_blob: unsafe {
        std::mem::transmute::<
            *const (),
            Option<unsafe extern "C-unwind" fn(*mut sqlite3_stmt, i32) -> *const ()>,
        >(sqlite3_column_blob as *const ())
    },
    column_bytes: unsafe {
        std::mem::transmute::<
            *const (),
            Option<unsafe extern "C-unwind" fn(*mut sqlite3_stmt, i32) -> i32>,
        >(sqlite3_column_bytes as *const ())
    },
    column_bytes16: unsafe {
        std::mem::transmute::<
            *const (),
            Option<unsafe extern "C-unwind" fn(*mut sqlite3_stmt, i32) -> i32>,
        >(sqlite3_column_bytes16 as *const ())
    },
    column_count: unsafe {
        std::mem::transmute::<
            *const (),
            Option<unsafe extern "C-unwind" fn(*mut sqlite3_stmt) -> i32>,
        >(sqlite3_column_count as *const ())
    },
    column_database_name: None,
    column_database_name16: None,
    column_decltype: unsafe {
        std::mem::transmute::<
            *const (),
            Option<unsafe extern "C-unwind" fn(*mut sqlite3_stmt, i32) -> *const i8>,
        >(sqlite3_column_decltype as *const ())
    },
    column_decltype16: unsafe {
        std::mem::transmute::<
            *const (),
            Option<unsafe extern "C-unwind" fn(*mut sqlite3_stmt, i32) -> *const ()>,
        >(sqlite3_column_decltype16 as *const ())
    },
    column_double: unsafe {
        std::mem::transmute::<
            *const (),
            Option<unsafe extern "C-unwind" fn(*mut sqlite3_stmt, i32) -> f64>,
        >(sqlite3_column_double as *const ())
    },
    column_int: unsafe {
        std::mem::transmute::<
            *const (),
            Option<unsafe extern "C-unwind" fn(*mut sqlite3_stmt, i32) -> i32>,
        >(sqlite3_column_int as *const ())
    },
    column_int64: unsafe {
        std::mem::transmute::<
            *const (),
            Option<unsafe extern "C-unwind" fn(*mut sqlite3_stmt, i32) -> i64>,
        >(sqlite3_column_int64 as *const ())
    },
    column_name: unsafe {
        std::mem::transmute::<
            *const (),
            Option<unsafe extern "C-unwind" fn(*mut sqlite3_stmt, i32) -> *const i8>,
        >(sqlite3_column_name as *const ())
    },
    column_name16: unsafe {
        std::mem::transmute::<
            *const (),
            Option<unsafe extern "C-unwind" fn(*mut sqlite3_stmt, i32) -> *const ()>,
        >(sqlite3_column_name16 as *const ())
    },
    column_origin_name: None,
    column_origin_name16: None,
    column_table_name: None,
    column_table_name16: None,
    column_text: unsafe {
        std::mem::transmute::<
            *const (),
            Option<unsafe extern "C-unwind" fn(*mut sqlite3_stmt, i32) -> *const u8>,
        >(sqlite3_column_text as *const ())
    },
    column_text16: unsafe {
        std::mem::transmute::<
            *const (),
            Option<unsafe extern "C-unwind" fn(*mut sqlite3_stmt, i32) -> *const ()>,
        >(sqlite3_column_text16 as *const ())
    },
    column_type: unsafe {
        std::mem::transmute::<
            *const (),
            Option<unsafe extern "C-unwind" fn(*mut sqlite3_stmt, i32) -> i32>,
        >(sqlite3_column_type as *const ())
    },
    column_value: unsafe {
        std::mem::transmute::<
            *const (),
            Option<unsafe extern "C-unwind" fn(*mut sqlite3_stmt, i32) -> *mut sqlite3_value>,
        >(sqlite3_column_value as *const ())
    },
    commit_hook: unsafe {
        std::mem::transmute::<
            *const (),
            Option<
                unsafe extern "C-unwind" fn(
                    *mut sqlite3,
                    Option<unsafe extern "C-unwind" fn(*mut ()) -> i32>,
                    *mut (),
                ) -> *mut (),
            >,
        >(sqlite3_commit_hook as *const ())
    },
    complete: unsafe {
        std::mem::transmute::<*const (), Option<unsafe extern "C-unwind" fn(*const i8) -> i32>>(
            sqlite3_complete as *const (),
        )
    },
    complete16: unsafe {
        std::mem::transmute::<*const (), Option<unsafe extern "C-unwind" fn(*const ()) -> i32>>(
            sqlite3_complete16 as *const (),
        )
    },
    create_collation: unsafe {
        std::mem::transmute::<
            *const (),
            Option<
                unsafe extern "C-unwind" fn(
                    *mut sqlite3,
                    *const i8,
                    i32,
                    *mut (),
                    Option<
                        unsafe extern "C-unwind" fn(*mut (), i32, *const (), i32, *const ()) -> i32,
                    >,
                ) -> i32,
            >,
        >(sqlite3_create_collation as *const ())
    },
    create_collation16: unsafe {
        std::mem::transmute::<
            *const (),
            Option<
                unsafe extern "C-unwind" fn(
                    *mut sqlite3,
                    *const (),
                    i32,
                    *mut (),
                    Option<
                        unsafe extern "C-unwind" fn(*mut (), i32, *const (), i32, *const ()) -> i32,
                    >,
                ) -> i32,
            >,
        >(sqlite3_create_collation16 as *const ())
    },
    create_function: unsafe {
        std::mem::transmute::<
            *const (),
            Option<
                unsafe extern "C-unwind" fn(
                    *mut sqlite3,
                    *const i8,
                    i32,
                    i32,
                    *mut (),
                    Option<
                        unsafe extern "C-unwind" fn(
                            *mut sqlite3_context,
                            i32,
                            *mut *mut sqlite3_value,
                        ),
                    >,
                    Option<
                        unsafe extern "C-unwind" fn(
                            *mut sqlite3_context,
                            i32,
                            *mut *mut sqlite3_value,
                        ),
                    >,
                    Option<unsafe extern "C-unwind" fn(*mut sqlite3_context)>,
                ) -> i32,
            >,
        >(sqlite3_create_function as *const ())
    },
    create_function16: unsafe {
        std::mem::transmute::<
            *const (),
            Option<
                unsafe extern "C-unwind" fn(
                    *mut sqlite3,
                    *const (),
                    i32,
                    i32,
                    *mut (),
                    Option<
                        unsafe extern "C-unwind" fn(
                            *mut sqlite3_context,
                            i32,
                            *mut *mut sqlite3_value,
                        ),
                    >,
                    Option<
                        unsafe extern "C-unwind" fn(
                            *mut sqlite3_context,
                            i32,
                            *mut *mut sqlite3_value,
                        ),
                    >,
                    Option<unsafe extern "C-unwind" fn(*mut sqlite3_context)>,
                ) -> i32,
            >,
        >(sqlite3_create_function16 as *const ())
    },
    create_module: unsafe {
        std::mem::transmute::<
            *const (),
            Option<
                unsafe extern "C-unwind" fn(
                    *mut sqlite3,
                    *const i8,
                    *const sqlite3_module,
                    *mut (),
                ) -> i32,
            >,
        >(sqlite3_create_module as *const ())
    },
    data_count: unsafe {
        std::mem::transmute::<
            *const (),
            Option<unsafe extern "C-unwind" fn(*mut sqlite3_stmt) -> i32>,
        >(sqlite3_data_count as *const ())
    },
    db_handle: unsafe {
        std::mem::transmute::<
            *const (),
            Option<unsafe extern "C-unwind" fn(*mut sqlite3_stmt) -> *mut sqlite3>,
        >(sqlite3_db_handle as *const ())
    },
    declare_vtab: unsafe {
        std::mem::transmute::<
            *const (),
            Option<unsafe extern "C-unwind" fn(*mut sqlite3, *const i8) -> i32>,
        >(sqlite3_declare_vtab as *const ())
    },
    enable_shared_cache: unsafe {
        std::mem::transmute::<*const (), Option<unsafe extern "C-unwind" fn(i32) -> i32>>(
            sqlite3_enable_shared_cache as *const (),
        )
    },
    errcode: unsafe {
        std::mem::transmute::<*const (), Option<unsafe extern "C-unwind" fn(*mut sqlite3) -> i32>>(
            sqlite3_errcode as *const (),
        )
    },
    errmsg: unsafe {
        std::mem::transmute::<
            *const (),
            Option<unsafe extern "C-unwind" fn(*mut sqlite3) -> *const i8>,
        >(sqlite3_errmsg as *const ())
    },
    errmsg16: unsafe {
        std::mem::transmute::<
            *const (),
            Option<unsafe extern "C-unwind" fn(*mut sqlite3) -> *const ()>,
        >(sqlite3_errmsg16 as *const ())
    },
    exec: unsafe {
        std::mem::transmute::<
            *const (),
            Option<
                unsafe extern "C-unwind" fn(
                    *mut sqlite3,
                    *const i8,
                    Option<
                        unsafe extern "C-unwind" fn(
                            *mut (),
                            i32,
                            *mut *mut i8,
                            *mut *mut i8,
                        ) -> i32,
                    >,
                    *mut (),
                    *mut *mut i8,
                ) -> i32,
            >,
        >(sqlite3_exec as *const ())
    },
    expired: unsafe {
        std::mem::transmute::<
            *const (),
            Option<unsafe extern "C-unwind" fn(*mut sqlite3_stmt) -> i32>,
        >(sqlite3_expired as *const ())
    },
    finalize: unsafe {
        std::mem::transmute::<
            *const (),
            Option<unsafe extern "C-unwind" fn(*mut sqlite3_stmt) -> i32>,
        >(sqlite3_finalize as *const ())
    },
    free: unsafe {
        std::mem::transmute::<*const (), Option<unsafe extern "C-unwind" fn(*mut ())>>(
            sqlite3_free as *const (),
        )
    },
    free_table: unsafe {
        std::mem::transmute::<*const (), Option<unsafe extern "C-unwind" fn(*mut *mut i8)>>(
            sqlite3_free_table as *const (),
        )
    },
    get_autocommit: unsafe {
        std::mem::transmute::<*const (), Option<unsafe extern "C-unwind" fn(*mut sqlite3) -> i32>>(
            sqlite3_get_autocommit as *const (),
        )
    },
    get_auxdata: unsafe {
        std::mem::transmute::<
            *const (),
            Option<unsafe extern "C-unwind" fn(*mut sqlite3_context, i32) -> *mut ()>,
        >(sqlite3_get_auxdata as *const ())
    },
    get_table: unsafe {
        std::mem::transmute::<
            *const (),
            Option<
                unsafe extern "C-unwind" fn(
                    *mut sqlite3,
                    *const i8,
                    *mut *mut *mut i8,
                    *mut i32,
                    *mut i32,
                    *mut *mut i8,
                ) -> i32,
            >,
        >(sqlite3_get_table as *const ())
    },
    global_recover: None,
    interruptx: unsafe {
        std::mem::transmute::<*const (), Option<unsafe extern "C-unwind" fn(*mut sqlite3)>>(
            sqlite3_interrupt as *const (),
        )
    },
    last_insert_rowid: unsafe {
        std::mem::transmute::<*const (), Option<unsafe extern "C-unwind" fn(*mut sqlite3) -> i64>>(
            sqlite3_last_insert_rowid as *const (),
        )
    },
    libversion: unsafe {
        std::mem::transmute::<*const (), Option<unsafe extern "C-unwind" fn() -> *const i8>>(
            sqlite3_libversion as *const (),
        )
    },
    libversion_number: unsafe {
        std::mem::transmute::<*const (), Option<unsafe extern "C-unwind" fn() -> i32>>(
            sqlite3_libversion_number as *const (),
        )
    },
    malloc: unsafe {
        std::mem::transmute::<*const (), Option<unsafe extern "C-unwind" fn(i32) -> *mut ()>>(
            sqlite3_malloc as *const (),
        )
    },
    mprintf: unsafe {
        std::mem::transmute::<
            *const (),
            Option<unsafe extern "C-unwind" fn(*const i8, ...) -> *mut i8>,
        >(sqlite3_mprintf as *const ())
    },
    open: unsafe {
        std::mem::transmute::<
            *const (),
            Option<unsafe extern "C-unwind" fn(*const i8, *mut *mut sqlite3) -> i32>,
        >(sqlite3_open as *const ())
    },
    open16: unsafe {
        std::mem::transmute::<
            *const (),
            Option<unsafe extern "C-unwind" fn(*const (), *mut *mut sqlite3) -> i32>,
        >(sqlite3_open16 as *const ())
    },
    prepare: unsafe {
        std::mem::transmute::<
            *const (),
            Option<
                unsafe extern "C-unwind" fn(
                    *mut sqlite3,
                    *const i8,
                    i32,
                    *mut *mut sqlite3_stmt,
                    *mut *const i8,
                ) -> i32,
            >,
        >(sqlite3_prepare as *const ())
    },
    prepare16: unsafe {
        std::mem::transmute::<
            *const (),
            Option<
                unsafe extern "C-unwind" fn(
                    *mut sqlite3,
                    *const (),
                    i32,
                    *mut *mut sqlite3_stmt,
                    *mut *const (),
                ) -> i32,
            >,
        >(sqlite3_prepare16 as *const ())
    },
    profile: unsafe {
        std::mem::transmute::<
            *const (),
            Option<
                unsafe extern "C-unwind" fn(
                    *mut sqlite3,
                    Option<unsafe extern "C-unwind" fn(*mut (), *const i8, u64)>,
                    *mut (),
                ) -> *mut (),
            >,
        >(sqlite3_profile as *const ())
    },
    progress_handler: unsafe {
        std::mem::transmute::<
            *const (),
            Option<
                unsafe extern "C-unwind" fn(
                    *mut sqlite3,
                    i32,
                    Option<unsafe extern "C-unwind" fn(*mut ()) -> i32>,
                    *mut (),
                ),
            >,
        >(sqlite3_progress_handler as *const ())
    },
    realloc: unsafe {
        std::mem::transmute::<*const (), Option<unsafe extern "C-unwind" fn(*mut (), i32) -> *mut ()>>(
            sqlite3_realloc as *const (),
        )
    },
    reset: unsafe {
        std::mem::transmute::<
            *const (),
            Option<unsafe extern "C-unwind" fn(*mut sqlite3_stmt) -> i32>,
        >(sqlite3_reset as *const ())
    },
    result_blob: unsafe {
        std::mem::transmute::<
            *const (),
            Option<
                unsafe extern "C-unwind" fn(
                    *mut sqlite3_context,
                    *const (),
                    i32,
                    Option<unsafe extern "C-unwind" fn(*mut ())>,
                ),
            >,
        >(sqlite3_result_blob as *const ())
    },
    result_double: unsafe {
        std::mem::transmute::<
            *const (),
            Option<unsafe extern "C-unwind" fn(*mut sqlite3_context, f64)>,
        >(sqlite3_result_double as *const ())
    },
    result_error: unsafe {
        std::mem::transmute::<
            *const (),
            Option<unsafe extern "C-unwind" fn(*mut sqlite3_context, *const i8, i32)>,
        >(sqlite3_result_error as *const ())
    },
    result_error16: unsafe {
        std::mem::transmute::<
            *const (),
            Option<unsafe extern "C-unwind" fn(*mut sqlite3_context, *const (), i32)>,
        >(sqlite3_result_error16 as *const ())
    },
    result_int: unsafe {
        std::mem::transmute::<
            *const (),
            Option<unsafe extern "C-unwind" fn(*mut sqlite3_context, i32)>,
        >(sqlite3_result_int as *const ())
    },
    result_int64: unsafe {
        std::mem::transmute::<
            *const (),
            Option<unsafe extern "C-unwind" fn(*mut sqlite3_context, i64)>,
        >(sqlite3_result_int64 as *const ())
    },
    result_null: unsafe {
        std::mem::transmute::<*const (), Option<unsafe extern "C-unwind" fn(*mut sqlite3_context)>>(
            sqlite3_result_null as *const (),
        )
    },
    result_text: unsafe {
        std::mem::transmute::<
            *const (),
            Option<
                unsafe extern "C-unwind" fn(
                    *mut sqlite3_context,
                    *const i8,
                    i32,
                    Option<unsafe extern "C-unwind" fn(*mut ())>,
                ),
            >,
        >(sqlite3_result_text as *const ())
    },
    result_text16: unsafe {
        std::mem::transmute::<
            *const (),
            Option<
                unsafe extern "C-unwind" fn(
                    *mut sqlite3_context,
                    *const (),
                    i32,
                    Option<unsafe extern "C-unwind" fn(*mut ())>,
                ),
            >,
        >(sqlite3_result_text16 as *const ())
    },
    result_text16be: unsafe {
        std::mem::transmute::<
            *const (),
            Option<
                unsafe extern "C-unwind" fn(
                    *mut sqlite3_context,
                    *const (),
                    i32,
                    Option<unsafe extern "C-unwind" fn(*mut ())>,
                ),
            >,
        >(sqlite3_result_text16be as *const ())
    },
    result_text16le: unsafe {
        std::mem::transmute::<
            *const (),
            Option<
                unsafe extern "C-unwind" fn(
                    *mut sqlite3_context,
                    *const (),
                    i32,
                    Option<unsafe extern "C-unwind" fn(*mut ())>,
                ),
            >,
        >(sqlite3_result_text16le as *const ())
    },
    result_value: unsafe {
        std::mem::transmute::<
            *const (),
            Option<unsafe extern "C-unwind" fn(*mut sqlite3_context, *mut sqlite3_value)>,
        >(sqlite3_result_value as *const ())
    },
    rollback_hook: unsafe {
        std::mem::transmute::<
            *const (),
            Option<
                unsafe extern "C-unwind" fn(
                    *mut sqlite3,
                    Option<unsafe extern "C-unwind" fn(*mut ())>,
                    *mut (),
                ) -> *mut (),
            >,
        >(sqlite3_rollback_hook as *const ())
    },
    set_authorizer: unsafe {
        std::mem::transmute::<
            *const (),
            Option<
                unsafe extern "C-unwind" fn(
                    *mut sqlite3,
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
                    *mut (),
                ) -> i32,
            >,
        >(sqlite3_set_authorizer as *const ())
    },
    set_auxdata: unsafe {
        std::mem::transmute::<
            *const (),
            Option<
                unsafe extern "C-unwind" fn(
                    *mut sqlite3_context,
                    i32,
                    *mut (),
                    Option<unsafe extern "C-unwind" fn(*mut ())>,
                ),
            >,
        >(sqlite3_set_auxdata as *const ())
    },
    xsnprintf: unsafe {
        std::mem::transmute::<
            *const (),
            Option<unsafe extern "C-unwind" fn(i32, *mut i8, *const i8, ...) -> *mut i8>,
        >(sqlite3_snprintf as *const ())
    },
    step: unsafe {
        std::mem::transmute::<
            *const (),
            Option<unsafe extern "C-unwind" fn(*mut sqlite3_stmt) -> i32>,
        >(sqlite3_step as *const ())
    },
    table_column_metadata: unsafe {
        std::mem::transmute::<
            *const (),
            Option<
                unsafe extern "C-unwind" fn(
                    *mut sqlite3,
                    *const i8,
                    *const i8,
                    *const i8,
                    *mut *const i8,
                    *mut *const i8,
                    *mut i32,
                    *mut i32,
                    *mut i32,
                ) -> i32,
            >,
        >(sqlite3_table_column_metadata as *const ())
    },
    thread_cleanup: unsafe {
        std::mem::transmute::<*const (), Option<unsafe extern "C-unwind" fn()>>(
            sqlite3_thread_cleanup as *const (),
        )
    },
    total_changes: unsafe {
        std::mem::transmute::<*const (), Option<unsafe extern "C-unwind" fn(*mut sqlite3) -> i32>>(
            sqlite3_total_changes as *const (),
        )
    },
    trace: unsafe {
        std::mem::transmute::<
            *const (),
            Option<
                unsafe extern "C-unwind" fn(
                    *mut sqlite3,
                    Option<unsafe extern "C-unwind" fn(*mut (), *const i8)>,
                    *mut (),
                ) -> *mut (),
            >,
        >(sqlite3_trace as *const ())
    },
    transfer_bindings: unsafe {
        std::mem::transmute::<
            *const (),
            Option<unsafe extern "C-unwind" fn(*mut sqlite3_stmt, *mut sqlite3_stmt) -> i32>,
        >(sqlite3_transfer_bindings as *const ())
    },
    update_hook: unsafe {
        std::mem::transmute::<
            *const (),
            Option<
                unsafe extern "C-unwind" fn(
                    *mut sqlite3,
                    Option<unsafe extern "C-unwind" fn(*mut (), i32, *const i8, *const i8, i64)>,
                    *mut (),
                ) -> *mut (),
            >,
        >(sqlite3_update_hook as *const ())
    },
    user_data: unsafe {
        std::mem::transmute::<
            *const (),
            Option<unsafe extern "C-unwind" fn(*mut sqlite3_context) -> *mut ()>,
        >(sqlite3_user_data as *const ())
    },
    value_blob: unsafe {
        std::mem::transmute::<
            *const (),
            Option<unsafe extern "C-unwind" fn(*mut sqlite3_value) -> *const ()>,
        >(sqlite3_value_blob as *const ())
    },
    value_bytes: unsafe {
        std::mem::transmute::<
            *const (),
            Option<unsafe extern "C-unwind" fn(*mut sqlite3_value) -> i32>,
        >(sqlite3_value_bytes as *const ())
    },
    value_bytes16: unsafe {
        std::mem::transmute::<
            *const (),
            Option<unsafe extern "C-unwind" fn(*mut sqlite3_value) -> i32>,
        >(sqlite3_value_bytes16 as *const ())
    },
    value_double: unsafe {
        std::mem::transmute::<
            *const (),
            Option<unsafe extern "C-unwind" fn(*mut sqlite3_value) -> f64>,
        >(sqlite3_value_double as *const ())
    },
    value_int: unsafe {
        std::mem::transmute::<
            *const (),
            Option<unsafe extern "C-unwind" fn(*mut sqlite3_value) -> i32>,
        >(sqlite3_value_int as *const ())
    },
    value_int64: unsafe {
        std::mem::transmute::<
            *const (),
            Option<unsafe extern "C-unwind" fn(*mut sqlite3_value) -> i64>,
        >(sqlite3_value_int64 as *const ())
    },
    value_numeric_type: unsafe {
        std::mem::transmute::<
            *const (),
            Option<unsafe extern "C-unwind" fn(*mut sqlite3_value) -> i32>,
        >(sqlite3_value_numeric_type as *const ())
    },
    value_text: unsafe {
        std::mem::transmute::<
            *const (),
            Option<unsafe extern "C-unwind" fn(*mut sqlite3_value) -> *const u8>,
        >(sqlite3_value_text as *const ())
    },
    value_text16: unsafe {
        std::mem::transmute::<
            *const (),
            Option<unsafe extern "C-unwind" fn(*mut sqlite3_value) -> *const ()>,
        >(sqlite3_value_text16 as *const ())
    },
    value_text16be: unsafe {
        std::mem::transmute::<
            *const (),
            Option<unsafe extern "C-unwind" fn(*mut sqlite3_value) -> *const ()>,
        >(sqlite3_value_text16be as *const ())
    },
    value_text16le: unsafe {
        std::mem::transmute::<
            *const (),
            Option<unsafe extern "C-unwind" fn(*mut sqlite3_value) -> *const ()>,
        >(sqlite3_value_text16le as *const ())
    },
    value_type: unsafe {
        std::mem::transmute::<
            *const (),
            Option<unsafe extern "C-unwind" fn(*mut sqlite3_value) -> i32>,
        >(sqlite3_value_type as *const ())
    },
    vmprintf: unsafe {
        std::mem::transmute::<
            *const (),
            Option<unsafe extern "C-unwind" fn(*const i8, core::ffi::VaList<'_>) -> *mut i8>,
        >(sqlite3_vmprintf as *const ())
    },
    overload_function: unsafe {
        std::mem::transmute::<
            *const (),
            Option<unsafe extern "C-unwind" fn(*mut sqlite3, *const i8, i32) -> i32>,
        >(sqlite3_overload_function as *const ())
    },
    prepare_v2: unsafe {
        std::mem::transmute::<
            *const (),
            Option<
                unsafe extern "C-unwind" fn(
                    *mut sqlite3,
                    *const i8,
                    i32,
                    *mut *mut sqlite3_stmt,
                    *mut *const i8,
                ) -> i32,
            >,
        >(sqlite3_prepare_v2 as *const ())
    },
    prepare16_v2: unsafe {
        std::mem::transmute::<
            *const (),
            Option<
                unsafe extern "C-unwind" fn(
                    *mut sqlite3,
                    *const (),
                    i32,
                    *mut *mut sqlite3_stmt,
                    *mut *const (),
                ) -> i32,
            >,
        >(sqlite3_prepare16_v2 as *const ())
    },
    clear_bindings: unsafe {
        std::mem::transmute::<
            *const (),
            Option<unsafe extern "C-unwind" fn(*mut sqlite3_stmt) -> i32>,
        >(sqlite3_clear_bindings as *const ())
    },
    create_module_v2: unsafe {
        std::mem::transmute::<
            *const (),
            Option<
                unsafe extern "C-unwind" fn(
                    *mut sqlite3,
                    *const i8,
                    *const sqlite3_module,
                    *mut (),
                    Option<unsafe extern "C-unwind" fn(*mut ())>,
                ) -> i32,
            >,
        >(sqlite3_create_module_v2 as *const ())
    },
    bind_zeroblob: unsafe {
        std::mem::transmute::<
            *const (),
            Option<unsafe extern "C-unwind" fn(*mut sqlite3_stmt, i32, i32) -> i32>,
        >(sqlite3_bind_zeroblob as *const ())
    },
    blob_bytes: unsafe {
        std::mem::transmute::<
            *const (),
            Option<unsafe extern "C-unwind" fn(*mut sqlite3_blob) -> i32>,
        >(sqlite3_blob_bytes as *const ())
    },
    blob_close: unsafe {
        std::mem::transmute::<
            *const (),
            Option<unsafe extern "C-unwind" fn(*mut sqlite3_blob) -> i32>,
        >(sqlite3_blob_close as *const ())
    },
    blob_open: unsafe {
        std::mem::transmute::<
            *const (),
            Option<
                unsafe extern "C-unwind" fn(
                    *mut sqlite3,
                    *const i8,
                    *const i8,
                    *const i8,
                    i64,
                    i32,
                    *mut *mut sqlite3_blob,
                ) -> i32,
            >,
        >(sqlite3_blob_open as *const ())
    },
    blob_read: unsafe {
        std::mem::transmute::<
            *const (),
            Option<unsafe extern "C-unwind" fn(*mut sqlite3_blob, *mut (), i32, i32) -> i32>,
        >(sqlite3_blob_read as *const ())
    },
    blob_write: unsafe {
        std::mem::transmute::<
            *const (),
            Option<unsafe extern "C-unwind" fn(*mut sqlite3_blob, *const (), i32, i32) -> i32>,
        >(sqlite3_blob_write as *const ())
    },
    create_collation_v2: unsafe {
        std::mem::transmute::<
            *const (),
            Option<
                unsafe extern "C-unwind" fn(
                    *mut sqlite3,
                    *const i8,
                    i32,
                    *mut (),
                    Option<
                        unsafe extern "C-unwind" fn(*mut (), i32, *const (), i32, *const ()) -> i32,
                    >,
                    Option<unsafe extern "C-unwind" fn(*mut ())>,
                ) -> i32,
            >,
        >(sqlite3_create_collation_v2 as *const ())
    },
    file_control: unsafe {
        std::mem::transmute::<
            *const (),
            Option<unsafe extern "C-unwind" fn(*mut sqlite3, *const i8, i32, *mut ()) -> i32>,
        >(sqlite3_file_control as *const ())
    },
    memory_highwater: unsafe {
        std::mem::transmute::<*const (), Option<unsafe extern "C-unwind" fn(i32) -> i64>>(
            sqlite3_memory_highwater as *const (),
        )
    },
    memory_used: unsafe {
        std::mem::transmute::<*const (), Option<unsafe extern "C-unwind" fn() -> i64>>(
            sqlite3_memory_used as *const (),
        )
    },
    mutex_alloc: unsafe {
        std::mem::transmute::<
            *const (),
            Option<unsafe extern "C-unwind" fn(i32) -> *mut sqlite3_mutex>,
        >(sqlite3_mutex_alloc as *const ())
    },
    mutex_enter: unsafe {
        std::mem::transmute::<*const (), Option<unsafe extern "C-unwind" fn(*mut sqlite3_mutex)>>(
            sqlite3_mutex_enter as *const (),
        )
    },
    mutex_free: unsafe {
        std::mem::transmute::<*const (), Option<unsafe extern "C-unwind" fn(*mut sqlite3_mutex)>>(
            sqlite3_mutex_free as *const (),
        )
    },
    mutex_leave: unsafe {
        std::mem::transmute::<*const (), Option<unsafe extern "C-unwind" fn(*mut sqlite3_mutex)>>(
            sqlite3_mutex_leave as *const (),
        )
    },
    mutex_try: unsafe {
        std::mem::transmute::<
            *const (),
            Option<unsafe extern "C-unwind" fn(*mut sqlite3_mutex) -> i32>,
        >(sqlite3_mutex_try as *const ())
    },
    open_v2: unsafe {
        std::mem::transmute::<
            *const (),
            Option<
                unsafe extern "C-unwind" fn(*const i8, *mut *mut sqlite3, i32, *const i8) -> i32,
            >,
        >(sqlite3_open_v2 as *const ())
    },
    release_memory: unsafe {
        std::mem::transmute::<*const (), Option<unsafe extern "C-unwind" fn(i32) -> i32>>(
            sqlite3_release_memory as *const (),
        )
    },
    result_error_nomem: unsafe {
        std::mem::transmute::<*const (), Option<unsafe extern "C-unwind" fn(*mut sqlite3_context)>>(
            sqlite3_result_error_nomem as *const (),
        )
    },
    result_error_toobig: unsafe {
        std::mem::transmute::<*const (), Option<unsafe extern "C-unwind" fn(*mut sqlite3_context)>>(
            sqlite3_result_error_toobig as *const (),
        )
    },
    sleep: unsafe {
        std::mem::transmute::<*const (), Option<unsafe extern "C-unwind" fn(i32) -> i32>>(
            sqlite3_sleep as *const (),
        )
    },
    soft_heap_limit: unsafe {
        std::mem::transmute::<*const (), Option<unsafe extern "C-unwind" fn(i32)>>(
            sqlite3_soft_heap_limit as *const (),
        )
    },
    vfs_find: unsafe {
        std::mem::transmute::<
            *const (),
            Option<unsafe extern "C-unwind" fn(*const i8) -> *mut sqlite3_vfs>,
        >(sqlite3_vfs_find as *const ())
    },
    vfs_register: unsafe {
        std::mem::transmute::<
            *const (),
            Option<unsafe extern "C-unwind" fn(*mut sqlite3_vfs, i32) -> i32>,
        >(sqlite3_vfs_register as *const ())
    },
    vfs_unregister: unsafe {
        std::mem::transmute::<*const (), Option<unsafe extern "C-unwind" fn(*mut sqlite3_vfs) -> i32>>(
            sqlite3_vfs_unregister as *const (),
        )
    },
    xthreadsafe: unsafe {
        std::mem::transmute::<*const (), Option<unsafe extern "C-unwind" fn() -> i32>>(
            sqlite3_threadsafe as *const (),
        )
    },
    result_zeroblob: unsafe {
        std::mem::transmute::<
            *const (),
            Option<unsafe extern "C-unwind" fn(*mut sqlite3_context, i32)>,
        >(sqlite3_result_zeroblob as *const ())
    },
    result_error_code: unsafe {
        std::mem::transmute::<
            *const (),
            Option<unsafe extern "C-unwind" fn(*mut sqlite3_context, i32)>,
        >(sqlite3_result_error_code as *const ())
    },
    test_control: unsafe {
        std::mem::transmute::<*const (), Option<unsafe extern "C-unwind" fn(i32, ...) -> i32>>(
            sqlite3_test_control as *const (),
        )
    },
    randomness: unsafe {
        std::mem::transmute::<*const (), Option<unsafe extern "C-unwind" fn(i32, *mut ())>>(
            sqlite3_randomness as *const (),
        )
    },
    context_db_handle: unsafe {
        std::mem::transmute::<
            *const (),
            Option<unsafe extern "C-unwind" fn(*mut sqlite3_context) -> *mut sqlite3>,
        >(sqlite3_context_db_handle as *const ())
    },
    extended_result_codes: unsafe {
        std::mem::transmute::<
            *const (),
            Option<unsafe extern "C-unwind" fn(*mut sqlite3, i32) -> i32>,
        >(sqlite3_extended_result_codes as *const ())
    },
    limit: unsafe {
        std::mem::transmute::<
            *const (),
            Option<unsafe extern "C-unwind" fn(*mut sqlite3, i32, i32) -> i32>,
        >(sqlite3_limit as *const ())
    },
    next_stmt: unsafe {
        std::mem::transmute::<
            *const (),
            Option<
                unsafe extern "C-unwind" fn(*mut sqlite3, *mut sqlite3_stmt) -> *mut sqlite3_stmt,
            >,
        >(sqlite3_next_stmt as *const ())
    },
    sql: unsafe {
        std::mem::transmute::<
            *const (),
            Option<unsafe extern "C-unwind" fn(*mut sqlite3_stmt) -> *const i8>,
        >(sqlite3_sql as *const ())
    },
    status: unsafe {
        std::mem::transmute::<
            *const (),
            Option<unsafe extern "C-unwind" fn(i32, *mut i32, *mut i32, i32) -> i32>,
        >(sqlite3_status as *const ())
    },
    backup_finish: unsafe {
        std::mem::transmute::<
            *const (),
            Option<unsafe extern "C-unwind" fn(*mut sqlite3_backup) -> i32>,
        >(sqlite3_backup_finish as *const ())
    },
    backup_init: unsafe {
        std::mem::transmute::<
            *const (),
            Option<
                unsafe extern "C-unwind" fn(
                    *mut sqlite3,
                    *const i8,
                    *mut sqlite3,
                    *const i8,
                ) -> *mut sqlite3_backup,
            >,
        >(sqlite3_backup_init as *const ())
    },
    backup_pagecount: unsafe {
        std::mem::transmute::<
            *const (),
            Option<unsafe extern "C-unwind" fn(*mut sqlite3_backup) -> i32>,
        >(sqlite3_backup_pagecount as *const ())
    },
    backup_remaining: unsafe {
        std::mem::transmute::<
            *const (),
            Option<unsafe extern "C-unwind" fn(*mut sqlite3_backup) -> i32>,
        >(sqlite3_backup_remaining as *const ())
    },
    backup_step: unsafe {
        std::mem::transmute::<
            *const (),
            Option<unsafe extern "C-unwind" fn(*mut sqlite3_backup, i32) -> i32>,
        >(sqlite3_backup_step as *const ())
    },
    compileoption_get: unsafe {
        std::mem::transmute::<*const (), Option<unsafe extern "C-unwind" fn(i32) -> *const i8>>(
            sqlite3_compileoption_get as *const (),
        )
    },
    compileoption_used: unsafe {
        std::mem::transmute::<*const (), Option<unsafe extern "C-unwind" fn(*const i8) -> i32>>(
            sqlite3_compileoption_used as *const (),
        )
    },
    create_function_v2: unsafe {
        std::mem::transmute::<
            *const (),
            Option<
                unsafe extern "C-unwind" fn(
                    *mut sqlite3,
                    *const i8,
                    i32,
                    i32,
                    *mut (),
                    Option<
                        unsafe extern "C-unwind" fn(
                            *mut sqlite3_context,
                            i32,
                            *mut *mut sqlite3_value,
                        ),
                    >,
                    Option<
                        unsafe extern "C-unwind" fn(
                            *mut sqlite3_context,
                            i32,
                            *mut *mut sqlite3_value,
                        ),
                    >,
                    Option<unsafe extern "C-unwind" fn(*mut sqlite3_context)>,
                    Option<unsafe extern "C-unwind" fn(*mut ())>,
                ) -> i32,
            >,
        >(sqlite3_create_function_v2 as *const ())
    },
    db_config: unsafe {
        std::mem::transmute::<
            *const (),
            Option<unsafe extern "C-unwind" fn(*mut sqlite3, i32, ...) -> i32>,
        >(sqlite3_db_config as *const ())
    },
    db_mutex: unsafe {
        std::mem::transmute::<
            *const (),
            Option<unsafe extern "C-unwind" fn(*mut sqlite3) -> *mut sqlite3_mutex>,
        >(sqlite3_db_mutex as *const ())
    },
    db_status: unsafe {
        std::mem::transmute::<
            *const (),
            Option<unsafe extern "C-unwind" fn(*mut sqlite3, i32, *mut i32, *mut i32, i32) -> i32>,
        >(sqlite3_db_status as *const ())
    },
    extended_errcode: unsafe {
        std::mem::transmute::<*const (), Option<unsafe extern "C-unwind" fn(*mut sqlite3) -> i32>>(
            sqlite3_extended_errcode as *const (),
        )
    },
    log: unsafe {
        std::mem::transmute::<*const (), Option<unsafe extern "C-unwind" fn(i32, *const i8, ...)>>(
            sqlite3_log as *const (),
        )
    },
    soft_heap_limit64: unsafe {
        std::mem::transmute::<*const (), Option<unsafe extern "C-unwind" fn(i64) -> i64>>(
            sqlite3_soft_heap_limit64 as *const (),
        )
    },
    sourceid: unsafe {
        std::mem::transmute::<*const (), Option<unsafe extern "C-unwind" fn() -> *const i8>>(
            sqlite3_sourceid as *const (),
        )
    },
    stmt_status: unsafe {
        std::mem::transmute::<
            *const (),
            Option<unsafe extern "C-unwind" fn(*mut sqlite3_stmt, i32, i32) -> i32>,
        >(sqlite3_stmt_status as *const ())
    },
    strnicmp: unsafe {
        std::mem::transmute::<
            *const (),
            Option<unsafe extern "C-unwind" fn(*const i8, *const i8, i32) -> i32>,
        >(sqlite3_strnicmp as *const ())
    },
    unlock_notify: None,
    wal_autocheckpoint: unsafe {
        std::mem::transmute::<
            *const (),
            Option<unsafe extern "C-unwind" fn(*mut sqlite3, i32) -> i32>,
        >(sqlite3_wal_autocheckpoint as *const ())
    },
    wal_checkpoint: unsafe {
        std::mem::transmute::<
            *const (),
            Option<unsafe extern "C-unwind" fn(*mut sqlite3, *const i8) -> i32>,
        >(sqlite3_wal_checkpoint as *const ())
    },
    wal_hook: unsafe {
        std::mem::transmute::<
            *const (),
            Option<
                unsafe extern "C-unwind" fn(
                    *mut sqlite3,
                    Option<
                        unsafe extern "C-unwind" fn(*mut (), *mut sqlite3, *const i8, i32) -> i32,
                    >,
                    *mut (),
                ) -> *mut (),
            >,
        >(sqlite3_wal_hook as *const ())
    },
    blob_reopen: unsafe {
        std::mem::transmute::<
            *const (),
            Option<unsafe extern "C-unwind" fn(*mut sqlite3_blob, i64) -> i32>,
        >(sqlite3_blob_reopen as *const ())
    },
    vtab_config: unsafe {
        std::mem::transmute::<
            *const (),
            Option<unsafe extern "C-unwind" fn(*mut sqlite3, i32, ...) -> i32>,
        >(sqlite3_vtab_config as *const ())
    },
    vtab_on_conflict: unsafe {
        std::mem::transmute::<*const (), Option<unsafe extern "C-unwind" fn(*mut sqlite3) -> i32>>(
            sqlite3_vtab_on_conflict as *const (),
        )
    },
    close_v2: unsafe {
        std::mem::transmute::<*const (), Option<unsafe extern "C-unwind" fn(*mut sqlite3) -> i32>>(
            sqlite3_close_v2 as *const (),
        )
    },
    db_filename: unsafe {
        std::mem::transmute::<
            *const (),
            Option<unsafe extern "C-unwind" fn(*mut sqlite3, *const i8) -> *const i8>,
        >(sqlite3_db_filename as *const ())
    },
    db_readonly: unsafe {
        std::mem::transmute::<
            *const (),
            Option<unsafe extern "C-unwind" fn(*mut sqlite3, *const i8) -> i32>,
        >(sqlite3_db_readonly as *const ())
    },
    db_release_memory: unsafe {
        std::mem::transmute::<*const (), Option<unsafe extern "C-unwind" fn(*mut sqlite3) -> i32>>(
            sqlite3_db_release_memory as *const (),
        )
    },
    errstr: unsafe {
        std::mem::transmute::<*const (), Option<unsafe extern "C-unwind" fn(i32) -> *const i8>>(
            sqlite3_errstr as *const (),
        )
    },
    stmt_busy: unsafe {
        std::mem::transmute::<
            *const (),
            Option<unsafe extern "C-unwind" fn(*mut sqlite3_stmt) -> i32>,
        >(sqlite3_stmt_busy as *const ())
    },
    stmt_readonly: unsafe {
        std::mem::transmute::<
            *const (),
            Option<unsafe extern "C-unwind" fn(*mut sqlite3_stmt) -> i32>,
        >(sqlite3_stmt_readonly as *const ())
    },
    stricmp: unsafe {
        std::mem::transmute::<
            *const (),
            Option<unsafe extern "C-unwind" fn(*const i8, *const i8) -> i32>,
        >(sqlite3_stricmp as *const ())
    },
    uri_boolean: unsafe {
        std::mem::transmute::<
            *const (),
            Option<unsafe extern "C-unwind" fn(*const i8, *const i8, i32) -> i32>,
        >(sqlite3_uri_boolean as *const ())
    },
    uri_int64: unsafe {
        std::mem::transmute::<
            *const (),
            Option<unsafe extern "C-unwind" fn(*const i8, *const i8, i64) -> i64>,
        >(sqlite3_uri_int64 as *const ())
    },
    uri_parameter: unsafe {
        std::mem::transmute::<
            *const (),
            Option<unsafe extern "C-unwind" fn(*const i8, *const i8) -> *const i8>,
        >(sqlite3_uri_parameter as *const ())
    },
    xvsnprintf: unsafe {
        std::mem::transmute::<
            *const (),
            Option<
                unsafe extern "C-unwind" fn(
                    i32,
                    *mut i8,
                    *const i8,
                    core::ffi::VaList<'_>,
                ) -> *mut i8,
            >,
        >(sqlite3_vsnprintf as *const ())
    },
    wal_checkpoint_v2: unsafe {
        std::mem::transmute::<
            *const (),
            Option<
                unsafe extern "C-unwind" fn(
                    *mut sqlite3,
                    *const i8,
                    i32,
                    *mut i32,
                    *mut i32,
                ) -> i32,
            >,
        >(sqlite3_wal_checkpoint_v2 as *const ())
    },
    auto_extension: Some(sqlite3_auto_extension),
    bind_blob64: unsafe {
        std::mem::transmute::<
            *const (),
            Option<
                unsafe extern "C-unwind" fn(
                    *mut sqlite3_stmt,
                    i32,
                    *const (),
                    u64,
                    Option<unsafe extern "C-unwind" fn(*mut ())>,
                ) -> i32,
            >,
        >(sqlite3_bind_blob64 as *const ())
    },
    bind_text64: unsafe {
        std::mem::transmute::<
            *const (),
            Option<
                unsafe extern "C-unwind" fn(
                    *mut sqlite3_stmt,
                    i32,
                    *const i8,
                    u64,
                    Option<unsafe extern "C-unwind" fn(*mut ())>,
                    u8,
                ) -> i32,
            >,
        >(sqlite3_bind_text64 as *const ())
    },
    cancel_auto_extension: Some(sqlite3_cancel_auto_extension),
    load_extension: Some(sqlite3_load_extension),
    malloc64: unsafe {
        std::mem::transmute::<*const (), Option<unsafe extern "C-unwind" fn(u64) -> *mut ()>>(
            sqlite3_malloc64 as *const (),
        )
    },
    msize: unsafe {
        std::mem::transmute::<*const (), Option<unsafe extern "C-unwind" fn(*mut ()) -> u64>>(
            sqlite3_msize as *const (),
        )
    },
    realloc64: unsafe {
        std::mem::transmute::<*const (), Option<unsafe extern "C-unwind" fn(*mut (), u64) -> *mut ()>>(
            sqlite3_realloc64 as *const (),
        )
    },
    reset_auto_extension: Some(sqlite3_reset_auto_extension),
    result_blob64: unsafe {
        std::mem::transmute::<
            *const (),
            Option<
                unsafe extern "C-unwind" fn(
                    *mut sqlite3_context,
                    *const (),
                    u64,
                    Option<unsafe extern "C-unwind" fn(*mut ())>,
                ),
            >,
        >(sqlite3_result_blob64 as *const ())
    },
    result_text64: unsafe {
        std::mem::transmute::<
            *const (),
            Option<
                unsafe extern "C-unwind" fn(
                    *mut sqlite3_context,
                    *const i8,
                    u64,
                    Option<unsafe extern "C-unwind" fn(*mut ())>,
                    u8,
                ),
            >,
        >(sqlite3_result_text64 as *const ())
    },
    strglob: unsafe {
        std::mem::transmute::<
            *const (),
            Option<unsafe extern "C-unwind" fn(*const i8, *const i8) -> i32>,
        >(sqlite3_strglob as *const ())
    },
    value_dup: unsafe {
        std::mem::transmute::<
            *const (),
            Option<unsafe extern "C-unwind" fn(*const sqlite3_value) -> *mut sqlite3_value>,
        >(sqlite3_value_dup as *const ())
    },
    value_free: unsafe {
        std::mem::transmute::<*const (), Option<unsafe extern "C-unwind" fn(*mut sqlite3_value)>>(
            sqlite3_value_free as *const (),
        )
    },
    result_zeroblob64: unsafe {
        std::mem::transmute::<
            *const (),
            Option<unsafe extern "C-unwind" fn(*mut sqlite3_context, u64) -> i32>,
        >(sqlite3_result_zeroblob64 as *const ())
    },
    bind_zeroblob64: unsafe {
        std::mem::transmute::<
            *const (),
            Option<unsafe extern "C-unwind" fn(*mut sqlite3_stmt, i32, u64) -> i32>,
        >(sqlite3_bind_zeroblob64 as *const ())
    },
    value_subtype: unsafe {
        std::mem::transmute::<
            *const (),
            Option<unsafe extern "C-unwind" fn(*mut sqlite3_value) -> u32>,
        >(sqlite3_value_subtype as *const ())
    },
    result_subtype: unsafe {
        std::mem::transmute::<
            *const (),
            Option<unsafe extern "C-unwind" fn(*mut sqlite3_context, u32)>,
        >(sqlite3_result_subtype as *const ())
    },
    status64: unsafe {
        std::mem::transmute::<
            *const (),
            Option<unsafe extern "C-unwind" fn(i32, *mut i64, *mut i64, i32) -> i32>,
        >(sqlite3_status64 as *const ())
    },
    strlike: unsafe {
        std::mem::transmute::<
            *const (),
            Option<unsafe extern "C-unwind" fn(*const i8, *const i8, u32) -> i32>,
        >(sqlite3_strlike as *const ())
    },
    db_cacheflush: unsafe {
        std::mem::transmute::<*const (), Option<unsafe extern "C-unwind" fn(*mut sqlite3) -> i32>>(
            sqlite3_db_cacheflush as *const (),
        )
    },
    system_errno: unsafe {
        std::mem::transmute::<*const (), Option<unsafe extern "C-unwind" fn(*mut sqlite3) -> i32>>(
            sqlite3_system_errno as *const (),
        )
    },
    trace_v2: unsafe {
        std::mem::transmute::<
            *const (),
            Option<
                unsafe extern "C-unwind" fn(
                    *mut sqlite3,
                    u32,
                    Option<unsafe extern "C-unwind" fn(u32, *mut (), *mut (), *mut ()) -> i32>,
                    *mut (),
                ) -> i32,
            >,
        >(sqlite3_trace_v2 as *const ())
    },
    expanded_sql: unsafe {
        std::mem::transmute::<
            *const (),
            Option<unsafe extern "C-unwind" fn(*mut sqlite3_stmt) -> *mut i8>,
        >(sqlite3_expanded_sql as *const ())
    },
    set_last_insert_rowid: unsafe {
        std::mem::transmute::<*const (), Option<unsafe extern "C-unwind" fn(*mut sqlite3, i64)>>(
            sqlite3_set_last_insert_rowid as *const (),
        )
    },
    prepare_v3: unsafe {
        std::mem::transmute::<
            *const (),
            Option<
                unsafe extern "C-unwind" fn(
                    *mut sqlite3,
                    *const i8,
                    i32,
                    u32,
                    *mut *mut sqlite3_stmt,
                    *mut *const i8,
                ) -> i32,
            >,
        >(sqlite3_prepare_v3 as *const ())
    },
    prepare16_v3: unsafe {
        std::mem::transmute::<
            *const (),
            Option<
                unsafe extern "C-unwind" fn(
                    *mut sqlite3,
                    *const (),
                    i32,
                    u32,
                    *mut *mut sqlite3_stmt,
                    *mut *const (),
                ) -> i32,
            >,
        >(sqlite3_prepare16_v3 as *const ())
    },
    bind_pointer: unsafe {
        std::mem::transmute::<
            *const (),
            Option<
                unsafe extern "C-unwind" fn(
                    *mut sqlite3_stmt,
                    i32,
                    *mut (),
                    *const i8,
                    Option<unsafe extern "C-unwind" fn(*mut ())>,
                ) -> i32,
            >,
        >(sqlite3_bind_pointer as *const ())
    },
    result_pointer: unsafe {
        std::mem::transmute::<
            *const (),
            Option<
                unsafe extern "C-unwind" fn(
                    *mut sqlite3_context,
                    *mut (),
                    *const i8,
                    Option<unsafe extern "C-unwind" fn(*mut ())>,
                ),
            >,
        >(sqlite3_result_pointer as *const ())
    },
    value_pointer: unsafe {
        std::mem::transmute::<
            *const (),
            Option<unsafe extern "C-unwind" fn(*mut sqlite3_value, *const i8) -> *mut ()>,
        >(sqlite3_value_pointer as *const ())
    },
    vtab_nochange: unsafe {
        std::mem::transmute::<
            *const (),
            Option<unsafe extern "C-unwind" fn(*mut sqlite3_context) -> i32>,
        >(sqlite3_vtab_nochange as *const ())
    },
    value_nochange: unsafe {
        std::mem::transmute::<
            *const (),
            Option<unsafe extern "C-unwind" fn(*mut sqlite3_value) -> i32>,
        >(sqlite3_value_nochange as *const ())
    },
    vtab_collation: unsafe {
        std::mem::transmute::<
            *const (),
            Option<unsafe extern "C-unwind" fn(*mut sqlite3_index_info, i32) -> *const i8>,
        >(sqlite3_vtab_collation as *const ())
    },
    keyword_count: unsafe {
        std::mem::transmute::<*const (), Option<unsafe extern "C-unwind" fn() -> i32>>(
            sqlite3_keyword_count as *const (),
        )
    },
    keyword_name: unsafe {
        std::mem::transmute::<
            *const (),
            Option<unsafe extern "C-unwind" fn(i32, *mut *const i8, *mut i32) -> i32>,
        >(sqlite3_keyword_name as *const ())
    },
    keyword_check: unsafe {
        std::mem::transmute::<*const (), Option<unsafe extern "C-unwind" fn(*const i8, i32) -> i32>>(
            sqlite3_keyword_check as *const (),
        )
    },
    str_new: unsafe {
        std::mem::transmute::<
            *const (),
            Option<unsafe extern "C-unwind" fn(*mut sqlite3) -> *mut sqlite3_str>,
        >(sqlite3_str_new as *const ())
    },
    str_finish: unsafe {
        std::mem::transmute::<
            *const (),
            Option<unsafe extern "C-unwind" fn(*mut sqlite3_str) -> *mut i8>,
        >(sqlite3_str_finish as *const ())
    },
    str_appendf: unsafe {
        std::mem::transmute::<
            *const (),
            Option<unsafe extern "C-unwind" fn(*mut sqlite3_str, *const i8, ...)>,
        >(sqlite3_str_appendf as *const ())
    },
    str_vappendf: unsafe {
        std::mem::transmute::<
            *const (),
            Option<unsafe extern "C-unwind" fn(*mut sqlite3_str, *const i8, core::ffi::VaList<'_>)>,
        >(sqlite3_str_vappendf as *const ())
    },
    str_append: unsafe {
        std::mem::transmute::<
            *const (),
            Option<unsafe extern "C-unwind" fn(*mut sqlite3_str, *const i8, i32)>,
        >(sqlite3_str_append as *const ())
    },
    str_appendall: unsafe {
        std::mem::transmute::<
            *const (),
            Option<unsafe extern "C-unwind" fn(*mut sqlite3_str, *const i8)>,
        >(sqlite3_str_appendall as *const ())
    },
    str_appendchar: unsafe {
        std::mem::transmute::<
            *const (),
            Option<unsafe extern "C-unwind" fn(*mut sqlite3_str, i32, i8)>,
        >(sqlite3_str_appendchar as *const ())
    },
    str_reset: unsafe {
        std::mem::transmute::<*const (), Option<unsafe extern "C-unwind" fn(*mut sqlite3_str)>>(
            sqlite3_str_reset as *const (),
        )
    },
    str_errcode: unsafe {
        std::mem::transmute::<*const (), Option<unsafe extern "C-unwind" fn(*mut sqlite3_str) -> i32>>(
            sqlite3_str_errcode as *const (),
        )
    },
    str_length: unsafe {
        std::mem::transmute::<*const (), Option<unsafe extern "C-unwind" fn(*mut sqlite3_str) -> i32>>(
            sqlite3_str_length as *const (),
        )
    },
    str_value: unsafe {
        std::mem::transmute::<
            *const (),
            Option<unsafe extern "C-unwind" fn(*mut sqlite3_str) -> *mut i8>,
        >(sqlite3_str_value as *const ())
    },
    create_window_function: unsafe {
        std::mem::transmute::<
            *const (),
            Option<
                unsafe extern "C-unwind" fn(
                    *mut sqlite3,
                    *const i8,
                    i32,
                    i32,
                    *mut (),
                    Option<
                        unsafe extern "C-unwind" fn(
                            *mut sqlite3_context,
                            i32,
                            *mut *mut sqlite3_value,
                        ),
                    >,
                    Option<unsafe extern "C-unwind" fn(*mut sqlite3_context)>,
                    Option<unsafe extern "C-unwind" fn(*mut sqlite3_context)>,
                    Option<
                        unsafe extern "C-unwind" fn(
                            *mut sqlite3_context,
                            i32,
                            *mut *mut sqlite3_value,
                        ),
                    >,
                    Option<unsafe extern "C-unwind" fn(*mut ())>,
                ) -> i32,
            >,
        >(sqlite3_create_window_function as *const ())
    },
    normalized_sql: None,
    stmt_isexplain: unsafe {
        std::mem::transmute::<
            *const (),
            Option<unsafe extern "C-unwind" fn(*mut sqlite3_stmt) -> i32>,
        >(sqlite3_stmt_isexplain as *const ())
    },
    value_frombind: unsafe {
        std::mem::transmute::<
            *const (),
            Option<unsafe extern "C-unwind" fn(*mut sqlite3_value) -> i32>,
        >(sqlite3_value_frombind as *const ())
    },
    drop_modules: unsafe {
        std::mem::transmute::<
            *const (),
            Option<unsafe extern "C-unwind" fn(*mut sqlite3, *mut *const i8) -> i32>,
        >(sqlite3_drop_modules as *const ())
    },
    hard_heap_limit64: unsafe {
        std::mem::transmute::<*const (), Option<unsafe extern "C-unwind" fn(i64) -> i64>>(
            sqlite3_hard_heap_limit64 as *const (),
        )
    },
    uri_key: unsafe {
        std::mem::transmute::<
            *const (),
            Option<unsafe extern "C-unwind" fn(*const i8, i32) -> *const i8>,
        >(sqlite3_uri_key as *const ())
    },
    filename_database: unsafe {
        std::mem::transmute::<*const (), Option<unsafe extern "C-unwind" fn(*const i8) -> *const i8>>(
            sqlite3_filename_database as *const (),
        )
    },
    filename_journal: unsafe {
        std::mem::transmute::<*const (), Option<unsafe extern "C-unwind" fn(*const i8) -> *const i8>>(
            sqlite3_filename_journal as *const (),
        )
    },
    filename_wal: unsafe {
        std::mem::transmute::<*const (), Option<unsafe extern "C-unwind" fn(*const i8) -> *const i8>>(
            sqlite3_filename_wal as *const (),
        )
    },
    create_filename: unsafe {
        std::mem::transmute::<
            *const (),
            Option<
                unsafe extern "C-unwind" fn(
                    *const i8,
                    *const i8,
                    *const i8,
                    i32,
                    *mut *const i8,
                ) -> *const i8,
            >,
        >(sqlite3_create_filename as *const ())
    },
    free_filename: unsafe {
        std::mem::transmute::<*const (), Option<unsafe extern "C-unwind" fn(*const i8)>>(
            sqlite3_free_filename as *const (),
        )
    },
    database_file_object: unsafe {
        std::mem::transmute::<
            *const (),
            Option<unsafe extern "C-unwind" fn(*const i8) -> *mut sqlite3_file>,
        >(sqlite3_database_file_object as *const ())
    },
    txn_state: unsafe {
        std::mem::transmute::<
            *const (),
            Option<unsafe extern "C-unwind" fn(*mut sqlite3, *const i8) -> i32>,
        >(sqlite3_txn_state as *const ())
    },
    changes64: unsafe {
        std::mem::transmute::<*const (), Option<unsafe extern "C-unwind" fn(*mut sqlite3) -> i64>>(
            sqlite3_changes64 as *const (),
        )
    },
    total_changes64: unsafe {
        std::mem::transmute::<*const (), Option<unsafe extern "C-unwind" fn(*mut sqlite3) -> i64>>(
            sqlite3_total_changes64 as *const (),
        )
    },
    autovacuum_pages: unsafe {
        std::mem::transmute::<
            *const (),
            Option<
                unsafe extern "C-unwind" fn(
                    *mut sqlite3,
                    Option<unsafe extern "C-unwind" fn(*mut (), *const i8, u32, u32, u32) -> u32>,
                    *mut (),
                    Option<unsafe extern "C-unwind" fn(*mut ())>,
                ) -> i32,
            >,
        >(sqlite3_autovacuum_pages as *const ())
    },
    error_offset: unsafe {
        std::mem::transmute::<*const (), Option<unsafe extern "C-unwind" fn(*mut sqlite3) -> i32>>(
            sqlite3_error_offset as *const (),
        )
    },
    vtab_rhs_value: unsafe {
        std::mem::transmute::<
            *const (),
            Option<
                unsafe extern "C-unwind" fn(
                    *mut sqlite3_index_info,
                    i32,
                    *mut *mut sqlite3_value,
                ) -> i32,
            >,
        >(sqlite3_vtab_rhs_value as *const ())
    },
    vtab_distinct: unsafe {
        std::mem::transmute::<
            *const (),
            Option<unsafe extern "C-unwind" fn(*mut sqlite3_index_info) -> i32>,
        >(sqlite3_vtab_distinct as *const ())
    },
    vtab_in: unsafe {
        std::mem::transmute::<
            *const (),
            Option<unsafe extern "C-unwind" fn(*mut sqlite3_index_info, i32, i32) -> i32>,
        >(sqlite3_vtab_in as *const ())
    },
    vtab_in_first: unsafe {
        std::mem::transmute::<
            *const (),
            Option<unsafe extern "C-unwind" fn(*mut sqlite3_value, *mut *mut sqlite3_value) -> i32>,
        >(sqlite3_vtab_in_first as *const ())
    },
    vtab_in_next: unsafe {
        std::mem::transmute::<
            *const (),
            Option<unsafe extern "C-unwind" fn(*mut sqlite3_value, *mut *mut sqlite3_value) -> i32>,
        >(sqlite3_vtab_in_next as *const ())
    },
    deserialize: unsafe {
        std::mem::transmute::<
            *const (),
            Option<
                unsafe extern "C-unwind" fn(*mut sqlite3, *const i8, *mut u8, i64, i64, u32) -> i32,
            >,
        >(sqlite3_deserialize as *const ())
    },
    serialize: unsafe {
        std::mem::transmute::<
            *const (),
            Option<unsafe extern "C-unwind" fn(*mut sqlite3, *const i8, *mut i64, u32) -> *mut u8>,
        >(sqlite3_serialize as *const ())
    },
    db_name: unsafe {
        std::mem::transmute::<
            *const (),
            Option<unsafe extern "C-unwind" fn(*mut sqlite3, i32) -> *const i8>,
        >(sqlite3_db_name as *const ())
    },
    value_encoding: unsafe {
        std::mem::transmute::<
            *const (),
            Option<unsafe extern "C-unwind" fn(*mut sqlite3_value) -> i32>,
        >(sqlite3_value_encoding as *const ())
    },
    is_interrupted: unsafe {
        std::mem::transmute::<*const (), Option<unsafe extern "C-unwind" fn(*mut sqlite3) -> i32>>(
            sqlite3_is_interrupted as *const (),
        )
    },
    stmt_explain: unsafe {
        std::mem::transmute::<
            *const (),
            Option<unsafe extern "C-unwind" fn(*mut sqlite3_stmt, i32) -> i32>,
        >(sqlite3_stmt_explain as *const ())
    },
    get_clientdata: unsafe {
        std::mem::transmute::<
            *const (),
            Option<unsafe extern "C-unwind" fn(*mut sqlite3, *const i8) -> *mut ()>,
        >(sqlite3_get_clientdata as *const ())
    },
    set_clientdata: unsafe {
        std::mem::transmute::<
            *const (),
            Option<
                unsafe extern "C-unwind" fn(
                    *mut sqlite3,
                    *const i8,
                    *mut (),
                    Option<unsafe extern "C-unwind" fn(*mut ())>,
                ) -> i32,
            >,
        >(sqlite3_set_clientdata as *const ())
    },
    setlk_timeout: unsafe {
        std::mem::transmute::<
            *const (),
            Option<unsafe extern "C-unwind" fn(*mut sqlite3, i32, i32) -> i32>,
        >(sqlite3_setlk_timeout as *const ())
    },
    set_errmsg: unsafe {
        std::mem::transmute::<
            *const (),
            Option<unsafe extern "C-unwind" fn(*mut sqlite3, i32, *const i8) -> i32>,
        >(sqlite3_set_errmsg as *const ())
    },
    db_status64: unsafe {
        std::mem::transmute::<
            *const (),
            Option<unsafe extern "C-unwind" fn(*mut sqlite3, i32, *mut i64, *mut i64, i32) -> i32>,
        >(sqlite3_db_status64 as *const ())
    },
    str_truncate: unsafe {
        std::mem::transmute::<*const (), Option<unsafe extern "C-unwind" fn(*mut sqlite3_str, i32)>>(
            sqlite3_str_truncate as *const (),
        )
    },
    str_free: unsafe {
        std::mem::transmute::<*const (), Option<unsafe extern "C-unwind" fn(*mut sqlite3_str)>>(
            sqlite3_str_free as *const (),
        )
    },
    carray_bind: None,
    carray_bind_v2: None,
    incomplete: unsafe {
        std::mem::transmute::<*const (), Option<unsafe extern "C-unwind" fn(*const i8) -> i64>>(
            sqlite3_incomplete as *const (),
        )
    },
    result_str: unsafe {
        std::mem::transmute::<
            *const (),
            Option<unsafe extern "C-unwind" fn(*mut sqlite3_context, *mut sqlite3_str, i32)>,
        >(sqlite3_result_str as *const ())
    },
};

// /* Load the extension into this database connection */
// /* Name of the shared library containing extension */
// /* Entry point.  Use "sqlite3_extension_init" if 0 */
// /* Put error message here if not 0 */
static mut azEndings: [*const i8; 1] = [(b"so\0".as_ptr() as *mut i8) as *const i8];

// /* Pointers to the extension init functions */
static mut sqlite3Autoext: sqlite3AutoExtList = sqlite3AutoExtList {
    nExt: (0 as i32) as u32,
    aExt: std::ptr::null_mut::<Option<unsafe extern "C-unwind" fn()>>(),
};

#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.loadext.sqlite3_load_extension")]
extern "C-unwind" fn sqlite3_load_extension(
    mut db: *mut sqlite3,
    mut zFile: *const i8,
    mut zProc: *const i8,
    mut pzErrMsg: *mut *mut i8,
) -> i32 {
    let mut rc: i32 = 0 as i32;
    unsafe { sqlite3_mutex_enter(unsafe { (*db).mutex }) };
    rc = sqlite3LoadExtension(db, zFile, zProc, pzErrMsg);
    rc = unsafe { sqlite3ApiExit(db, rc) };
    unsafe { sqlite3_mutex_leave(unsafe { (*db).mutex }) };
    return rc;
}

// /*
// ** Enable or disable extension loading.  Extension loading is disabled by
// ** default so as not to open security holes in older applications.
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3_enable_load_extension(mut db: *mut sqlite3, mut onoff: i32) -> i32 {
    unsafe { sqlite3_mutex_enter(unsafe { (*db).mutex }) };
    if onoff != (0 as i32) {
        let __v1641: *mut sqlite3 = db;
        let __v1642: u64 = unsafe { (*__v1641).flags };
        let __v1643: u64 = __v1642 | ((((65536 as i32) | (131072 as i32)) as i64) as u64);
        unsafe {
            (*__v1641).flags = __v1643;
        }
    } else {
        let __v1644: *mut sqlite3 = db;
        let __v1645: u64 = unsafe { (*__v1644).flags };
        let __v1646: u64 = __v1645 & !((((65536 as i32) | (131072 as i32)) as i64) as u64);
        unsafe {
            (*__v1644).flags = __v1646;
        }
    }
    unsafe { sqlite3_mutex_leave(unsafe { (*db).mutex }) };
    return 0 as i32;
}

// /* The "wsdAutoext" macro will resolve to the autoextension
// ** state vector.  If writable static data is unsupported on the target,
// ** we have to locate the state vector at run-time.  In the more common
// ** case where writable static data is supported, wsdStat can refer directly
// ** to the "sqlite3Autoext" state vector declared above.
// */
// /*
// ** Register a statically linked extension that is automatically
// ** loaded by every new database connection.
// */
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.loadext.sqlite3_auto_extension")]
extern "C-unwind" fn sqlite3_auto_extension(
    mut xInit: Option<unsafe extern "C-unwind" fn()>,
) -> i32 {
    let mut rc: i32 = 0 as i32;
    rc = unsafe { sqlite3_initialize() };
    if rc != (0 as i32) {
        return rc;
    } else {
        let mut i: u32 = 0 as u32;
        let mut mutex: *mut sqlite3_mutex = unsafe { sqlite3MutexAlloc(2 as i32) };
        {}
        unsafe { sqlite3_mutex_enter(mutex) };
        i = (0 as i32) as u32;
        '__slate_break_1637: while i < unsafe { sqlite3Autoext.nExt } {
            if (unsafe { *unsafe { unsafe { sqlite3Autoext.aExt }.offset(i as isize) } }) == xInit {
                break '__slate_break_1637;
            }
            let __v1647: u32 = i;
            let __v1648: u32 = __v1647.wrapping_add((1 as i32) as u32);
            i = __v1648;
        }
        if i == unsafe { sqlite3Autoext.nExt } {
            let mut nByte: u64 = (unsafe { sqlite3Autoext.nExt }.wrapping_add((1 as i32) as u32)
                as u64)
                .wrapping_mul(8 as u64);
            let mut aNew: *mut Option<unsafe extern "C-unwind" fn()> =
                unsafe { std::mem::zeroed() };
            aNew =
                (unsafe { sqlite3_realloc64((unsafe { sqlite3Autoext.aExt }) as *mut (), nByte) })
                    as *mut Option<unsafe extern "C-unwind" fn()>;
            if aNew == std::ptr::null_mut::<Option<unsafe extern "C-unwind" fn()>>() {
                rc = 7 as i32;
            } else {
                unsafe {
                    sqlite3Autoext.aExt = aNew;
                }
                unsafe {
                    *unsafe {
                        unsafe { sqlite3Autoext.aExt }
                            .offset((unsafe { sqlite3Autoext.nExt }) as isize)
                    } = xInit;
                }
                let __v1649: u32 = unsafe { sqlite3Autoext.nExt };
                let __v1650: u32 = __v1649.wrapping_add((1 as i32) as u32);
                unsafe {
                    sqlite3Autoext.nExt = __v1650;
                }
            }
        }
        unsafe { sqlite3_mutex_leave(mutex) };
        0 as i32;
        return rc;
    }
    return unsafe { std::mem::zeroed() };
}

// /*
// ** Cancel a prior call to sqlite3_auto_extension.  Remove xInit from the
// ** set of routines that is invoked for each new database connection, if it
// ** is currently on the list.  If xInit is not on the list, then this
// ** routine is a no-op.
// **
// ** Return 1 if xInit was found on the list and removed.  Return 0 if xInit
// ** was not on the list.
// */
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.loadext.sqlite3_cancel_auto_extension")]
extern "C-unwind" fn sqlite3_cancel_auto_extension(
    mut xInit: Option<unsafe extern "C-unwind" fn()>,
) -> i32 {
    let mut mutex: *mut sqlite3_mutex = unsafe { sqlite3MutexAlloc(2 as i32) };
    let mut i: i32 = 0 as i32;
    let mut n: i32 = 0 as i32;
    {}
    unsafe { sqlite3_mutex_enter(mutex) };
    i = ((unsafe { sqlite3Autoext.nExt }) as i32) - (1 as i32);
    '__slate_break_1638: loop {
        if !(i >= (0 as i32)) {
            break;
        }
        if (unsafe { *unsafe { unsafe { sqlite3Autoext.aExt }.offset(i as isize) } }) == xInit {
            let __v1653: u32 = unsafe { sqlite3Autoext.nExt };
            let __v1654: u32 = __v1653.wrapping_sub((1 as i32) as u32);
            unsafe {
                sqlite3Autoext.nExt = __v1654;
            }
            unsafe {
                *unsafe { unsafe { sqlite3Autoext.aExt }.offset(i as isize) } = unsafe {
                    *unsafe {
                        unsafe { sqlite3Autoext.aExt }
                            .offset((unsafe { sqlite3Autoext.nExt }) as isize)
                    }
                };
            }
            let __v1655: i32 = n;
            let __v1656: i32 = __v1655 + (1 as i32);
            n = __v1656;
            break '__slate_break_1638;
        }
        let __v1651: i32 = i;
        let __v1652: i32 = __v1651 - (1 as i32);
        i = __v1652;
    }
    unsafe { sqlite3_mutex_leave(mutex) };
    return n;
}

// /*
// ** Reset the automatic extension loading mechanism.
// */
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.loadext.sqlite3_reset_auto_extension")]
extern "C-unwind" fn sqlite3_reset_auto_extension() {
    if (unsafe { sqlite3_initialize() }) == (0 as i32) {
        let mut mutex: *mut sqlite3_mutex = unsafe { sqlite3MutexAlloc(2 as i32) };
        {}
        unsafe { sqlite3_mutex_enter(mutex) };
        unsafe { sqlite3_free((unsafe { sqlite3Autoext.aExt }) as *mut ()) };
        unsafe {
            sqlite3Autoext.aExt = std::ptr::null_mut::<Option<unsafe extern "C-unwind" fn()>>();
        }
        unsafe {
            sqlite3Autoext.nExt = (0 as i32) as u32;
        }
        unsafe { sqlite3_mutex_leave(mutex) };
    }
}

// /*
// ** Load all automatic extensions.
// **
// ** If anything goes wrong, set an error in the database connection.
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3AutoLoadExtensions(mut db: *mut sqlite3) {
    let mut i: u32 = 0 as u32;
    let mut go: i32 = 1 as i32;
    let mut rc: i32 = 0 as i32;
    let mut xInit: Option<
        unsafe extern "C-unwind" fn(*mut sqlite3, *mut *mut i8, *const sqlite3_api_routines) -> i32,
    > = unsafe { std::mem::zeroed() };
    {}
    if (unsafe { sqlite3Autoext.nExt }) == ((0 as i32) as u32) {
        // /* Common case: early out without every having to acquire a mutex */
        return;
    }
    i = (0 as i32) as u32;
    '__slate_break_1639: while go != (0 as i32) {
        let mut zErrmsg: *mut i8 = unsafe { std::mem::zeroed() };
        let mut mutex: *mut sqlite3_mutex = unsafe { sqlite3MutexAlloc(2 as i32) };
        let mut pThunk: *const sqlite3_api_routines = unsafe { std::ptr::addr_of!(sqlite3Apis) };
        unsafe { sqlite3_mutex_enter(mutex) };
        if i >= unsafe { sqlite3Autoext.nExt } {
            xInit = None;
            go = 0 as i32;
        } else {
            xInit = unsafe {
                std::mem::transmute::<
                    Option<unsafe extern "C-unwind" fn()>,
                    Option<
                        unsafe extern "C-unwind" fn(
                            *mut sqlite3,
                            *mut *mut i8,
                            *const sqlite3_api_routines,
                        ) -> i32,
                    >,
                >(unsafe {
                    *unsafe { unsafe { sqlite3Autoext.aExt }.offset(i as isize) }
                })
            };
        }
        unsafe { sqlite3_mutex_leave(mutex) };
        zErrmsg = std::ptr::null_mut::<i8>();
        let __v1659: bool;
        if xInit != None {
            let __v1660: i32 =
                unsafe { xInit.unwrap()(db, std::ptr::addr_of_mut!(zErrmsg), pThunk) };
            rc = __v1660;
            __v1659 = __v1660 != (0 as i32);
        } else {
            __v1659 = false as bool;
        }
        if __v1659 {
            unsafe {
                sqlite3ErrorWithMsg(
                    db,
                    rc,
                    (b"automatic extension loading failed: %s\0".as_ptr() as *mut i8) as *const i8,
                    zErrmsg,
                )
            };
            go = 0 as i32;
        }
        unsafe { sqlite3_free(zErrmsg as *mut ()) };
        let __v1657: u32 = i;
        let __v1658: u32 = __v1657.wrapping_add((1 as i32) as u32);
        i = __v1658;
    }
}

// /* Load the extension into this database connection */
// /* Name of the shared library containing extension */
// /* Entry point.  Use "sqlite3_extension_init" if 0 */
// /* Put error message here if not 0 */
// /*
// ** Call this routine when the database connection is closing in order
// ** to clean up loaded extensions
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3CloseExtensions(mut db: *mut sqlite3) {
    let mut i: i32 = 0 as i32;
    0 as i32;
    i = 0 as i32;
    '__slate_break_1636: loop {
        if !(i < unsafe { (*db).nExtension }) {
            break;
        }
        unsafe {
            sqlite3OsDlClose(unsafe { (*db).pVfs }, unsafe {
                *unsafe { unsafe { (*db).aExtension }.offset(i as isize) }
            })
        };
        let __v1661: i32 = i;
        let __v1662: i32 = __v1661 + (1 as i32);
        i = __v1662;
    }
    unsafe { sqlite3DbFree(db, (unsafe { (*db).aExtension }) as *mut ()) };
}

// /* Was sqlite3_global_recover(), but that function is deprecated */
// /*
//   ** The original API set ends here.  All extensions can call any
//   ** of the APIs above provided that the pointer is not NULL.  But
//   ** before calling APIs that follow, extension should check the
//   ** sqlite3_libversion_number() to make sure they are dealing with
//   ** a library that is new enough to support that API.
//   *************************************************************************
//   */
// /*
//   ** Added after 3.3.13
//   */
// /*
//   ** Added for 3.4.1
//   */
// /*
//   ** Added for 3.5.0
//   */
// /*
//   ** Added for 3.5.8
//   */
// /*
//   ** Added for 3.6.0
//   */
// /*
//   ** Added for 3.7.4
//   */
// /* Version 3.8.7 and later */
// /* Version 3.8.11 and later */
// /* Version 3.9.0 and later */
// /* Version 3.10.0 and later */
// /* Version 3.12.0 and later */
// /* Version 3.14.0 and later */
// /* Version 3.18.0 and later */
// /* Version 3.20.0 and later */
// /* Version 3.22.0 and later */
// /* Version 3.24.0 and later */
// /* Version 3.25.0 and later */
// /* Version 3.26.0 and later */
// /* Version 3.28.0 and later */
// /* Version 3.30.0 and later */
// /* Version 3.31.0 and later */
// /* Version 3.32.0 and later */
// /* Version 3.34.0 and later */
// /* Version 3.36.1 and later */
// /* Version 3.37.0 and later */
// /* Version 3.38.0 and later */
// /* Version 3.39.0 and later */
// /* Version 3.40.0 and later */
// /* Version 3.41.0 and later */
// /* Version 3.43.0 and later */
// /* Version 3.44.0 and later */
// /* Version 3.50.0 and later */
// /* Version 3.51.0 and later */
// /* Version 3.52.0 and later */
// /* Version 3.54.0 and later */
// /* True if x is the directory separator character
// */
// /*
// ** Attempt to load an SQLite extension library contained in the file
// ** zFile.  The entry point is zProc.  zProc may be 0 in which case a
// ** default entry point name (sqlite3_extension_init) is used.  Use
// ** of the default name is recommended.
// **
// ** Return SQLITE_OK on success and SQLITE_ERROR if something goes wrong.
// **
// ** If an error occurs and pzErrMsg is not 0, then fill *pzErrMsg with
// ** error message text.  The calling function should free this memory
// ** by calling sqlite3DbFree(db, ).
// */
fn sqlite3LoadExtension(
    mut db: *mut sqlite3,
    mut zFile: *const i8,
    mut zProc: *const i8,
    mut pzErrMsg: *mut *mut i8,
) -> i32 {
    let mut __slate_storage_1687: std::mem::MaybeUninit<*mut i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1687: *mut *mut i8 =
        std::ptr::addr_of_mut!(__slate_storage_1687) as *mut *mut i8;
    let mut __slate_storage_1686: std::mem::MaybeUninit<u64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1686: *mut u64 = std::ptr::addr_of_mut!(__slate_storage_1686) as *mut u64;
    let mut __slate_storage_1685: std::mem::MaybeUninit<u64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1685: *mut u64 = std::ptr::addr_of_mut!(__slate_storage_1685) as *mut u64;
    let mut __slate_storage_1684: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1684: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1684) as *mut i32;
    let mut __slate_storage_1683: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1683: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1683) as *mut i32;
    let mut __slate_storage_1682: std::mem::MaybeUninit<*mut sqlite3> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1682: *mut *mut sqlite3 =
        std::ptr::addr_of_mut!(__slate_storage_1682) as *mut *mut sqlite3;
    let mut __slate_storage_1681: std::mem::MaybeUninit<*mut i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1681: *mut *mut i8 =
        std::ptr::addr_of_mut!(__slate_storage_1681) as *mut *mut i8;
    let mut __slate_storage_1680: std::mem::MaybeUninit<u64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1680: *mut u64 = std::ptr::addr_of_mut!(__slate_storage_1680) as *mut u64;
    let mut __slate_storage_1679: std::mem::MaybeUninit<u64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1679: *mut u64 = std::ptr::addr_of_mut!(__slate_storage_1679) as *mut u64;
    let mut __slate_storage_1678: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1678: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1678) as *mut i32;
    let mut __slate_storage_1677: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1677: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1677) as *mut i32;
    let mut __slate_storage_1676: std::mem::MaybeUninit<bool> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1676: *mut bool = std::ptr::addr_of_mut!(__slate_storage_1676) as *mut bool;
    let mut __slate_storage_1671: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1671: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1671) as *mut i32;
    let mut __slate_storage_1673: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1673: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1673) as *mut i32;
    let mut __slate_storage_1672: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1672: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1672) as *mut i32;
    let mut __slate_storage_1675: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1675: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1675) as *mut i32;
    let mut __slate_storage_1674: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1674: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1674) as *mut i32;
    let mut __slate_storage_1670: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1670: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1670) as *mut i32;
    let mut __slate_storage_1669: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1669: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1669) as *mut i32;
    let mut __slate_storage_1668: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1668: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1668) as *mut i32;
    let mut __slate_storage_1667: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1667: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1667) as *mut i32;
    let mut __slate_storage_1666: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1666: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1666) as *mut i32;
    let mut __slate_storage_1665: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1665: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1665) as *mut i32;
    let mut __slate_storage_949: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_949: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_949) as *mut i32;
    let mut __slate_storage_948: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_948: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_948) as *mut i32;
    let mut __slate_storage_947: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_947: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_947) as *mut i32;
    let mut __slate_storage_946: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_946: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_946) as *mut i32;
    let mut __slate_storage_945: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_945: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_945) as *mut i32;
    let mut __slate_storage_1664: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1664: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1664) as *mut i32;
    let mut __slate_storage_1663: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1663: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1663) as *mut i32;
    let mut __slate_storage_944: std::mem::MaybeUninit<*mut i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_944: *mut *mut i8 =
        std::ptr::addr_of_mut!(__slate_storage_944) as *mut *mut i8;
    let mut __slate_storage_942: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_942: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_942) as *mut i32;
    let mut __slate_storage_941: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_941: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_941) as *mut i32;
    let mut __slate_storage_940: std::mem::MaybeUninit<u64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_940: *mut u64 = std::ptr::addr_of_mut!(__slate_storage_940) as *mut u64;
    let mut __slate_storage_939: std::mem::MaybeUninit<*mut *mut ()> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_939: *mut *mut *mut () =
        std::ptr::addr_of_mut!(__slate_storage_939) as *mut *mut *mut ();
    let mut __slate_storage_938: std::mem::MaybeUninit<*mut i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_938: *mut *mut i8 =
        std::ptr::addr_of_mut!(__slate_storage_938) as *mut *mut i8;
    let mut __slate_storage_937: std::mem::MaybeUninit<*const i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_937: *mut *const i8 =
        std::ptr::addr_of_mut!(__slate_storage_937) as *mut *const i8;
    let mut __slate_storage_936: std::mem::MaybeUninit<*mut i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_936: *mut *mut i8 =
        std::ptr::addr_of_mut!(__slate_storage_936) as *mut *mut i8;
    let mut __slate_storage_935: std::mem::MaybeUninit<
        Option<
            unsafe extern "C-unwind" fn(
                *mut sqlite3,
                *mut *mut i8,
                *const sqlite3_api_routines,
            ) -> i32,
        >,
    > = std::mem::MaybeUninit::uninit();
    let __slate_slot_935: *mut Option<
        unsafe extern "C-unwind" fn(*mut sqlite3, *mut *mut i8, *const sqlite3_api_routines) -> i32,
    > = std::ptr::addr_of_mut!(__slate_storage_935)
        as *mut Option<
            unsafe extern "C-unwind" fn(
                *mut sqlite3,
                *mut *mut i8,
                *const sqlite3_api_routines,
            ) -> i32,
        >;
    let mut __slate_storage_934: std::mem::MaybeUninit<*mut ()> = std::mem::MaybeUninit::uninit();
    let __slate_slot_934: *mut *mut () =
        std::ptr::addr_of_mut!(__slate_storage_934) as *mut *mut ();
    let mut __slate_storage_933: std::mem::MaybeUninit<*mut sqlite3_vfs> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_933: *mut *mut sqlite3_vfs =
        std::ptr::addr_of_mut!(__slate_storage_933) as *mut *mut sqlite3_vfs;
    unsafe {
        '__join_50: {
            std::ptr::write(__slate_slot_933, unsafe { (*db).pVfs });
            std::ptr::write(__slate_slot_936, std::ptr::null_mut::<i8>());
            std::ptr::write(__slate_slot_938, std::ptr::null_mut::<i8>());
            std::ptr::write(__slate_slot_940, unsafe { strlen(zFile) });
            // /* Shared library endings to try if zFile cannot be loaded as written */
            if pzErrMsg != std::ptr::null_mut::<*mut i8>() {
                unsafe {
                    *pzErrMsg = std::ptr::null_mut::<i8>();
                }
            }
        }
        // /* Ticket #1863.  To avoid a creating security problems for older
        //   ** applications that relink against newer versions of SQLite, the
        //   ** ability to run load_extension is turned off by default.  One
        //   ** must call either sqlite3_enable_load_extension(db) or
        //   ** sqlite3_db_config(db, SQLITE_DBCONFIG_ENABLE_LOAD_EXTENSION, 1, 0)
        //   ** to turn on extension loading.
        //   */
        if (unsafe { (*db).flags }) & (((65536 as i32) as i64) as u64)
            == (((0 as i32) as i64) as u64)
        {
            if pzErrMsg != std::ptr::null_mut::<*mut i8>() {
                unsafe {
                    *pzErrMsg = unsafe {
                        sqlite3_mprintf((b"not authorized\0".as_ptr() as *mut i8) as *const i8)
                    };
                }
            }
            return 1 as i32;
        } else {
            *__slate_slot_937 = if zProc != std::ptr::null::<i8>() {
                zProc
            } else {
                (b"sqlite3_extension_init\0".as_ptr() as *mut i8) as *const i8
            };
            // /* tag-20210611-1.  Some dlopen() implementations will segfault if given
            //   ** an oversize filename.  Most filesystems have a pathname limit of 4K,
            //   ** so limit the extension filename length to about twice that.
            //   ** https://sqlite.org/forum/forumpost/08a0d6d9bf
            //   **
            //   ** Later (2023-03-25): Save an extra 6 bytes for the filename suffix.
            //   ** See https://sqlite.org/forum/forumpost/24083b579d.
            //   */
            if *__slate_slot_940 > (((4096 as i32) as i64) as u64) {
            } else {
                // /* Do not allow sqlite3_load_extension() to link to a copy of the
                //   ** running application, by passing in an empty filename. */
                if *__slate_slot_940 == (((0 as i32) as i64) as u64) {
                } else {
                    *__slate_slot_934 = unsafe { sqlite3OsDlOpen(*__slate_slot_933, zFile) };
                    *__slate_slot_941 = 0 as i32;
                    loop {
                        if *__slate_slot_941 < ((((8 as u64) / (8 as u64)) as u32) as i32)
                            && *__slate_slot_934 == std::ptr::null_mut::<()>()
                        {
                            std::ptr::write(__slate_slot_944, unsafe {
                                sqlite3_mprintf(
                                    (b"%s.%s\0".as_ptr() as *mut i8) as *const i8,
                                    zFile,
                                    unsafe {
                                        *unsafe {
                                            unsafe {
                                                std::ptr::addr_of_mut!(azEndings) as *mut *const i8
                                            }
                                            .offset(*__slate_slot_941 as isize)
                                        }
                                    },
                                )
                            });
                            if *__slate_slot_944 == std::ptr::null_mut::<i8>() {
                                return 7 as i32;
                            } else {
                                if (*__slate_slot_940)
                                    .wrapping_add(unsafe {
                                        strlen(unsafe {
                                            *unsafe {
                                                unsafe {
                                                    std::ptr::addr_of_mut!(azEndings)
                                                        as *mut *const i8
                                                }
                                                .offset(*__slate_slot_941 as isize)
                                            }
                                        })
                                    })
                                    .wrapping_add(((1 as i32) as i64) as u64)
                                    <= (((4096 as i32) as i64) as u64)
                                {
                                    *__slate_slot_934 = unsafe {
                                        sqlite3OsDlOpen(
                                            *__slate_slot_933,
                                            *__slate_slot_944 as *const i8,
                                        )
                                    };
                                }
                                unsafe { sqlite3_free(*__slate_slot_944 as *mut ()) };
                                std::ptr::write(__slate_slot_1663, *__slate_slot_941);
                                std::ptr::write(__slate_slot_1664, *__slate_slot_1663 + (1 as i32));
                                *__slate_slot_941 = *__slate_slot_1664;
                            }
                        } else {
                            break;
                        }
                    }
                    if *__slate_slot_934 == std::ptr::null_mut::<()>() {
                    } else {
                        '__join_19: {
                            *__slate_slot_935 = unsafe {
                                std::mem::transmute::<
                                    Option<unsafe extern "C-unwind" fn()>,
                                    Option<
                                        unsafe extern "C-unwind" fn(
                                            *mut sqlite3,
                                            *mut *mut i8,
                                            *const sqlite3_api_routines,
                                        )
                                            -> i32,
                                    >,
                                >(unsafe {
                                    sqlite3OsDlSym(
                                        *__slate_slot_933,
                                        *__slate_slot_934,
                                        *__slate_slot_937,
                                    )
                                })
                            };
                            // /* If no entry point was specified and the default legacy
                            //   ** entry point name "sqlite3_extension_init" was not found, then
                            //   ** construct an entry point name "sqlite3_X_init" where the X is
                            //   ** replaced by the lowercase value of every ASCII alphabetic
                            //   ** character in the filename after the last "/" up to the first ".",
                            //   ** and skipping the first three characters if they are "lib".
                            //   ** Examples:
                            //   **
                            //   **    /usr/local/lib/libExample5.4.3.so ==>  sqlite3_example_init
                            //   **    C:/lib/mathfuncs.dll              ==>  sqlite3_mathfuncs_init
                            //   **
                            //   ** If that still finds no entry point, repeat a second time but this
                            //   ** time include both alphabetic and numeric characters up to the first
                            //   ** ".".  Example:
                            //   **
                            //   **    /usr/local/lib/libExample5.4.3.so ==>  sqlite3_example5_init
                            //   */
                            if *__slate_slot_935 == None && zProc == std::ptr::null::<i8>() {
                                std::ptr::write(__slate_slot_948, unsafe {
                                    sqlite3Strlen30(zFile)
                                });
                                std::ptr::write(__slate_slot_949, 0 as i32);
                                *__slate_slot_938 = (unsafe {
                                    sqlite3_malloc64(
                                        ((*__slate_slot_948 + (30 as i32)) as i64) as u64,
                                    )
                                }) as *mut i8;
                                if *__slate_slot_938 == std::ptr::null_mut::<i8>() {
                                    unsafe {
                                        sqlite3OsDlClose(*__slate_slot_933, *__slate_slot_934)
                                    };
                                    return 7 as i32;
                                } else {
                                    loop {
                                        unsafe {
                                            memcpy(
                                                *__slate_slot_938 as *mut (),
                                                (b"sqlite3_\0".as_ptr() as *mut i8) as *const (),
                                                ((8 as i32) as i64) as u64,
                                            )
                                        };
                                        *__slate_slot_945 = *__slate_slot_948 - (1 as i32);
                                        loop {
                                            if *__slate_slot_945 >= (0 as i32)
                                                && !(((unsafe {
                                                    *unsafe {
                                                        zFile.offset(*__slate_slot_945 as isize)
                                                    }
                                                })
                                                    as i32)
                                                    == (47 as i32))
                                            {
                                                std::ptr::write(
                                                    __slate_slot_1665,
                                                    *__slate_slot_945,
                                                );
                                                std::ptr::write(
                                                    __slate_slot_1666,
                                                    *__slate_slot_1665 - (1 as i32),
                                                );
                                                *__slate_slot_945 = *__slate_slot_1666;
                                            } else {
                                                break;
                                            }
                                        }
                                        std::ptr::write(__slate_slot_1667, *__slate_slot_945);
                                        std::ptr::write(
                                            __slate_slot_1668,
                                            *__slate_slot_1667 + (1 as i32),
                                        );
                                        *__slate_slot_945 = *__slate_slot_1668;
                                        if (unsafe {
                                            sqlite3_strnicmp(
                                                unsafe { zFile.offset(*__slate_slot_945 as isize) },
                                                (b"lib\0".as_ptr() as *mut i8) as *const i8,
                                                3 as i32,
                                            )
                                        }) == (0 as i32)
                                        {
                                            std::ptr::write(__slate_slot_1669, *__slate_slot_945);
                                            std::ptr::write(
                                                __slate_slot_1670,
                                                *__slate_slot_1669 + (3 as i32),
                                            );
                                            *__slate_slot_945 = *__slate_slot_1670;
                                        }
                                        *__slate_slot_946 = 8 as i32;
                                        loop {
                                            std::ptr::write(
                                                __slate_slot_1671,
                                                (unsafe {
                                                    *unsafe {
                                                        zFile.offset(*__slate_slot_945 as isize)
                                                    }
                                                })
                                                    as i32,
                                            );
                                            *__slate_slot_947 = *__slate_slot_1671;
                                            if *__slate_slot_1671 != (0 as i32)
                                                && *__slate_slot_947 != (46 as i32)
                                            {
                                                if (((unsafe {
                                                    *unsafe {
                                                        unsafe {
                                                            std::ptr::addr_of!(sqlite3CtypeMap)
                                                                as *const u8
                                                        }
                                                        .offset(
                                                            ((((*__slate_slot_947 as i8) as u8)
                                                                as u32)
                                                                as i32)
                                                                as isize,
                                                        )
                                                    }
                                                })
                                                    as u32)
                                                    as i32)
                                                    & (2 as i32)
                                                    != (0 as i32)
                                                    || *__slate_slot_949 != (0 as i32)
                                                        && (((unsafe {
                                                            *unsafe {
                                                                unsafe {
                                                                    std::ptr::addr_of!(
                                                                        sqlite3CtypeMap
                                                                    )
                                                                        as *const u8
                                                                }
                                                                .offset(
                                                                    ((((*__slate_slot_947 as i8)
                                                                        as u8)
                                                                        as u32)
                                                                        as i32)
                                                                        as isize,
                                                                )
                                                            }
                                                        })
                                                            as u32)
                                                            as i32)
                                                            & (4 as i32)
                                                            != (0 as i32)
                                                {
                                                    std::ptr::write(
                                                        __slate_slot_1674,
                                                        *__slate_slot_946,
                                                    );
                                                    std::ptr::write(
                                                        __slate_slot_1675,
                                                        *__slate_slot_1674 + (1 as i32),
                                                    );
                                                    *__slate_slot_946 = *__slate_slot_1675;
                                                    unsafe {
                                                        *unsafe {
                                                            (*__slate_slot_938)
                                                                .offset(*__slate_slot_1674 as isize)
                                                        } = (unsafe {
                                                            *unsafe {
                                                                unsafe {
                                                                    std::ptr::addr_of!(
                                                                        sqlite3UpperToLower
                                                                    )
                                                                        as *const u8
                                                                }
                                                                .offset(
                                                                    (*__slate_slot_947 as u32)
                                                                        as isize,
                                                                )
                                                            }
                                                        })
                                                            as i8;
                                                    }
                                                }
                                                std::ptr::write(
                                                    __slate_slot_1672,
                                                    *__slate_slot_945,
                                                );
                                                std::ptr::write(
                                                    __slate_slot_1673,
                                                    *__slate_slot_1672 + (1 as i32),
                                                );
                                                *__slate_slot_945 = *__slate_slot_1673;
                                            } else {
                                                break;
                                            }
                                        }
                                        unsafe {
                                            memcpy(
                                                (unsafe {
                                                    (*__slate_slot_938)
                                                        .offset(*__slate_slot_946 as isize)
                                                })
                                                    as *mut (),
                                                (b"_init\0".as_ptr() as *mut i8) as *const (),
                                                ((6 as i32) as i64) as u64,
                                            )
                                        };
                                        *__slate_slot_937 = *__slate_slot_938 as *const i8;
                                        *__slate_slot_935 = unsafe {
                                            std::mem::transmute::<
                                                Option<unsafe extern "C-unwind" fn()>,
                                                Option<
                                                    unsafe extern "C-unwind" fn(
                                                        *mut sqlite3,
                                                        *mut *mut i8,
                                                        *const sqlite3_api_routines,
                                                    )
                                                        -> i32,
                                                >,
                                            >(unsafe {
                                                sqlite3OsDlSym(
                                                    *__slate_slot_933,
                                                    *__slate_slot_934,
                                                    *__slate_slot_937,
                                                )
                                            })
                                        };
                                        if *__slate_slot_935 == None {
                                            std::ptr::write(__slate_slot_1677, *__slate_slot_949);
                                            std::ptr::write(
                                                __slate_slot_1678,
                                                *__slate_slot_1677 + (1 as i32),
                                            );
                                            *__slate_slot_949 = *__slate_slot_1678;
                                            *__slate_slot_1676 = *__slate_slot_1678 < (2 as i32);
                                        } else {
                                            *__slate_slot_1676 = false as bool;
                                        }
                                        if !(*__slate_slot_1676) {
                                            break '__join_19;
                                        }
                                    }
                                }
                            }
                        }
                        if *__slate_slot_935 == None {
                            if pzErrMsg != std::ptr::null_mut::<*mut i8>() {
                                std::ptr::write(__slate_slot_1679, *__slate_slot_940);
                                std::ptr::write(
                                    __slate_slot_1680,
                                    (*__slate_slot_1679).wrapping_add(
                                        unsafe { strlen(*__slate_slot_937) }
                                            .wrapping_add(((300 as i32) as i64) as u64),
                                    ),
                                );
                                *__slate_slot_940 = *__slate_slot_1680;
                                std::ptr::write(
                                    __slate_slot_1681,
                                    (unsafe { sqlite3_malloc64(*__slate_slot_940) }) as *mut i8,
                                );
                                *__slate_slot_936 = *__slate_slot_1681;
                                unsafe {
                                    *pzErrMsg = *__slate_slot_1681;
                                }
                                if *__slate_slot_936 != std::ptr::null_mut::<i8>() {
                                    // /* zErrmsg would be NULL if not so */
                                    0 as i32;
                                    unsafe {
                                        sqlite3_snprintf(
                                            (*__slate_slot_940 as u32) as i32,
                                            *__slate_slot_936,
                                            (b"no entry point [%s] in shared library [%s]\0"
                                                .as_ptr()
                                                as *mut i8)
                                                as *const i8,
                                            *__slate_slot_937,
                                            zFile,
                                        )
                                    };
                                    unsafe {
                                        sqlite3OsDlError(
                                            *__slate_slot_933,
                                            ((*__slate_slot_940)
                                                .wrapping_sub(((1 as i32) as i64) as u64)
                                                as u32)
                                                as i32,
                                            *__slate_slot_936,
                                        )
                                    };
                                }
                            }
                            unsafe { sqlite3OsDlClose(*__slate_slot_933, *__slate_slot_934) };
                            unsafe { sqlite3_free(*__slate_slot_938 as *mut ()) };
                            return 1 as i32;
                        } else {
                            unsafe { sqlite3_free(*__slate_slot_938 as *mut ()) };
                            *__slate_slot_942 = unsafe {
                                (*__slate_slot_935).unwrap()(
                                    db,
                                    std::ptr::addr_of_mut!(*__slate_slot_936),
                                    unsafe { std::ptr::addr_of!(sqlite3Apis) },
                                )
                            };
                            if *__slate_slot_942 != (0 as i32) {
                                if *__slate_slot_942 == (0 as i32) | (1 as i32) << (8 as i32) {
                                    return 0 as i32;
                                } else {
                                    if pzErrMsg != std::ptr::null_mut::<*mut i8>() {
                                        unsafe {
                                            *pzErrMsg = unsafe {
                                                sqlite3_mprintf(
                                                    (b"error during initialization: %s\0".as_ptr()
                                                        as *mut i8)
                                                        as *const i8,
                                                    *__slate_slot_936,
                                                )
                                            };
                                        }
                                    }
                                    unsafe { sqlite3_free(*__slate_slot_936 as *mut ()) };
                                    unsafe {
                                        sqlite3OsDlClose(*__slate_slot_933, *__slate_slot_934)
                                    };
                                    return 1 as i32;
                                }
                            } else {
                                // /* Append the new shared library handle to the db->aExtension array. */
                                *__slate_slot_939 = (unsafe {
                                    sqlite3DbMallocZero(
                                        db,
                                        (8 as u64).wrapping_mul(
                                            (((unsafe { (*db).nExtension }) + (1 as i32)) as i64)
                                                as u64,
                                        ),
                                    )
                                })
                                    as *mut *mut ();
                                if *__slate_slot_939 == std::ptr::null_mut::<*mut ()>() {
                                    return 7 as i32;
                                } else {
                                    if (unsafe { (*db).nExtension }) > (0 as i32) {
                                        unsafe {
                                            memcpy(
                                                *__slate_slot_939 as *mut (),
                                                (unsafe { (*db).aExtension }) as *const (),
                                                (8 as u64).wrapping_mul(
                                                    ((unsafe { (*db).nExtension }) as i64) as u64,
                                                ),
                                            )
                                        };
                                    }
                                    unsafe {
                                        sqlite3DbFree(db, (unsafe { (*db).aExtension }) as *mut ())
                                    };
                                    unsafe {
                                        (*db).aExtension = *__slate_slot_939;
                                    }
                                    std::ptr::write(__slate_slot_1682, db);
                                    std::ptr::write(__slate_slot_1683, unsafe {
                                        (*(*__slate_slot_1682)).nExtension
                                    });
                                    std::ptr::write(
                                        __slate_slot_1684,
                                        *__slate_slot_1683 + (1 as i32),
                                    );
                                    unsafe {
                                        (*(*__slate_slot_1682)).nExtension = *__slate_slot_1684;
                                    }
                                    unsafe {
                                        *unsafe {
                                            unsafe { (*db).aExtension }
                                                .offset(*__slate_slot_1683 as isize)
                                        } = *__slate_slot_934;
                                    }
                                    return 0 as i32;
                                }
                            }
                        }
                    }
                }
            }
            if pzErrMsg != std::ptr::null_mut::<*mut i8>() {
                std::ptr::write(__slate_slot_1685, *__slate_slot_940);
                std::ptr::write(
                    __slate_slot_1686,
                    (*__slate_slot_1685).wrapping_add(((300 as i32) as i64) as u64),
                );
                *__slate_slot_940 = *__slate_slot_1686;
                std::ptr::write(
                    __slate_slot_1687,
                    (unsafe { sqlite3_malloc64(*__slate_slot_940) }) as *mut i8,
                );
                *__slate_slot_936 = *__slate_slot_1687;
                unsafe {
                    *pzErrMsg = *__slate_slot_1687;
                }
                if *__slate_slot_936 != std::ptr::null_mut::<i8>() {
                    // /* zErrmsg would be NULL if not so */
                    0 as i32;
                    unsafe {
                        sqlite3_snprintf(
                            (*__slate_slot_940 as u32) as i32,
                            *__slate_slot_936,
                            (b"unable to open shared library [%.*s]\0".as_ptr() as *mut i8)
                                as *const i8,
                            4096 as i32,
                            zFile,
                        )
                    };
                    unsafe {
                        sqlite3OsDlError(
                            *__slate_slot_933,
                            ((*__slate_slot_940).wrapping_sub(((1 as i32) as i64) as u64) as u32)
                                as i32,
                            *__slate_slot_936,
                        )
                    };
                }
            }
            return 1 as i32;
        }
    }
    return unsafe { std::mem::zeroed() };
}
