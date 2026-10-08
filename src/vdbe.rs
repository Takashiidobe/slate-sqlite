//! 2001 September 15
//!
//! The author disclaims copyright to this source code.  In place of
//! a legal notice, here is a blessing:
//!
//!    May you do good and not evil.
//!    May you find forgiveness for yourself and forgive others.
//!    May you share freely, never taking more than you give.
//!
//!
//! The code in this file implements the function that runs the
//! bytecode of a prepared statement.
//!
//! Various scripts scan this source file in order to generate HTML
//! documentation, headers files, or other derived files.  The formatting
//! of the code in this file is, therefore, important.  See other comments
//! in this file for details.  If in doubt, do not deviate from existing
//! commenting and indentation practices when changing or adding code.
unsafe extern "C" {
    static mut sqlite3StdType: [*const i8; 0];
    static mut sqlite3aLTb: *const u8;
    static mut sqlite3aEQb: *const u8;
    static mut sqlite3aGTb: *const u8;
    static mut sqlite3CtypeMap: [u8; 0];
    static mut sqlite3Config: Sqlite3Config;
    static mut sqlite3SmallTypeSizes: [u8; 0];
    fn sqlite3_exec(
        __v1059: *mut sqlite3,
        sql: *const i8,
        callback: Option<
            unsafe extern "C-unwind" fn(*mut (), i32, *mut *mut i8, *mut *mut i8) -> i32,
        >,
        __v1062: *mut (),
        errmsg: *mut *mut i8,
    ) -> i32;
    fn sqlite3_snprintf(__v1064: i32, __v1065: *mut i8, __v1066: *const i8, ...) -> *mut i8;
    fn sqlite3_malloc64(__v1067: u64) -> *mut ();
    fn sqlite3_free(__v1068: *mut ());
    fn sqlite3_randomness(N: i32, P: *mut ());
    fn sqlite3_value_text(__v1071: *mut sqlite3_value) -> *const u8;
    fn sqlite3_value_type(__v1072: *mut sqlite3_value) -> i32;
    fn sqlite3_mutex_enter(__v1074: *mut sqlite3_mutex);
    fn sqlite3_mutex_leave(__v1075: *mut sqlite3_mutex);
    fn sqlite3_log(iErrCode: i32, zFormat: *const i8, ...);
    fn memcpy(__dest: *mut (), __src: *const (), __n: u64) -> *mut ();
    fn memset(__s: *mut (), __c: i32, __n: u64) -> *mut ();
    fn memcmp(__s1: *const (), __s2: *const (), __n: u64) -> i32;
    fn sqlite3PagerSetJournalMode(__v1087: *mut Pager, __v1088: i32) -> i32;
    fn sqlite3PagerGetJournalMode(__v1089: *mut Pager) -> i32;
    fn sqlite3PagerOkToChangeJournalMode(__v1090: *mut Pager) -> i32;
    fn sqlite3PagerWalSupported(pPager: *mut Pager) -> i32;
    fn sqlite3PagerCloseWal(pPager: *mut Pager, __v1093: *mut sqlite3) -> i32;
    fn sqlite3PagerFilename(__v1094: *const Pager, __v1095: i32) -> *const i8;
    fn sqlite3BtreeOpen(
        pVfs: *mut sqlite3_vfs,
        zFilename: *const i8,
        db: *mut sqlite3,
        ppBtree: *mut *mut Btree,
        flags: i32,
        vfsFlags_1101: i32,
    ) -> i32;
    fn sqlite3BtreeClose(__v1102: *mut Btree) -> i32;
    fn sqlite3BtreeMaxPageCount(__v1103: *mut Btree, __v1104: u32) -> u32;
    fn sqlite3BtreeLastPage(__v1105: *mut Btree) -> u32;
    fn sqlite3BtreeBeginTrans(__v1106: *mut Btree, __v1107: i32, __v1108: *mut i32) -> i32;
    fn sqlite3BtreeBeginStmt(__v1109: *mut Btree, __v1110: i32) -> i32;
    fn sqlite3BtreeCreateTable(__v1111: *mut Btree, __v1112: *mut u32, flags: i32) -> i32;
    fn sqlite3BtreeLockTable(pBtree: *mut Btree, iTab: i32, isWriteLock: u8) -> i32;
    fn sqlite3BtreeSavepoint(__v1117: *mut Btree, __v1118: i32, __v1119: i32) -> i32;
    fn sqlite3BtreeIncrVacuum(__v1120: *mut Btree) -> i32;
    fn sqlite3BtreeDropTable(__v1121: *mut Btree, __v1122: i32, __v1123: *mut i32) -> i32;
    fn sqlite3BtreeClearTable(__v1124: *mut Btree, __v1125: i32, __v1126: *mut i64) -> i32;
    fn sqlite3BtreeClearTableOfCursor(__v1127: *mut BtCursor) -> i32;
    fn sqlite3BtreeTripAllCursors(__v1128: *mut Btree, __v1129: i32, __v1130: i32) -> i32;
    fn sqlite3BtreeGetMeta(pBtree: *mut Btree, idx: i32, pValue: *mut u32);
    fn sqlite3BtreeUpdateMeta(__v1134: *mut Btree, idx: i32, value: u32) -> i32;
    fn sqlite3BtreeCursor(
        __v1137: *mut Btree,
        iTable: u32,
        wrFlag: i32,
        __v1140: *mut KeyInfo,
        pCursor: *mut BtCursor,
    ) -> i32;
    fn sqlite3BtreeFakeValidCursor() -> *mut BtCursor;
    fn sqlite3BtreeCursorSize() -> i32;
    fn sqlite3BtreeCursorZero(__v1142: *mut BtCursor);
    fn sqlite3BtreeCursorHintFlags(__v1143: *mut BtCursor, __v1144: u32);
    fn sqlite3BtreeTableMoveto(
        __v1145: *mut BtCursor,
        intKey: i64,
        bias: i32,
        pRes: *mut i32,
    ) -> i32;
    fn sqlite3BtreeIndexMoveto(
        __v1149: *mut BtCursor,
        pUnKey: *mut UnpackedRecord,
        pRes: *mut i32,
    ) -> i32;
    fn sqlite3BtreeCursorHasMoved(__v1152: *mut BtCursor) -> i32;
    fn sqlite3BtreeDelete(__v1153: *mut BtCursor, flags: u8) -> i32;
    fn sqlite3BtreeInsert(
        __v1155: *mut BtCursor,
        pPayload: *const BtreePayload,
        flags: i32,
        seekResult: i32,
    ) -> i32;
    fn sqlite3BtreeFirst(__v1159: *mut BtCursor, pRes: *mut i32) -> i32;
    fn sqlite3BtreeIsEmpty(pCur: *mut BtCursor, pRes: *mut i32) -> i32;
    fn sqlite3BtreeLast(__v1163: *mut BtCursor, pRes: *mut i32) -> i32;
    fn sqlite3BtreeNext(__v1165: *mut BtCursor, flags: i32) -> i32;
    fn sqlite3BtreeEof(__v1167: *mut BtCursor) -> i32;
    fn sqlite3BtreePrevious(__v1168: *mut BtCursor, flags: i32) -> i32;
    fn sqlite3BtreeIntegerKey(__v1170: *mut BtCursor) -> i64;
    fn sqlite3BtreeCursorPin(__v1171: *mut BtCursor);
    fn sqlite3BtreeCursorUnpin(__v1172: *mut BtCursor);
    fn sqlite3BtreeOffset(__v1173: *mut BtCursor) -> i64;
    fn sqlite3BtreePayload(__v1174: *mut BtCursor, offset: u32, amt: u32, __v1177: *mut ()) -> i32;
    fn sqlite3BtreePayloadFetch(__v1178: *mut BtCursor, pAmt: *mut u32) -> *const ();
    fn sqlite3BtreePayloadSize(__v1180: *mut BtCursor) -> u32;
    fn sqlite3BtreeIntegrityCheck(
        db: *mut sqlite3,
        p: *mut Btree,
        aRoot: *mut u32,
        aCnt: *mut sqlite3_value,
        nRoot: i32,
        mxErr: i32,
        pnErr: *mut i32,
        pzOut: *mut *mut i8,
    ) -> i32;
    fn sqlite3BtreePager(__v1189: *mut Btree) -> *mut Pager;
    fn sqlite3BtreeRowCountEst(__v1190: *mut BtCursor) -> i64;
    fn sqlite3BtreeClearCursor(__v1191: *mut BtCursor);
    fn sqlite3BtreeSetVersion(pBt: *mut Btree, iVersion: i32) -> i32;
    fn sqlite3BtreeCursorHasHint(__v1194: *mut BtCursor, mask: u32) -> i32;
    fn sqlite3BtreeCursorIsValidNN(__v1196: *mut BtCursor) -> i32;
    fn sqlite3BtreeCount(__v1197: *mut sqlite3, __v1198: *mut BtCursor, __v1199: *mut i64) -> i32;
    fn sqlite3BtreeTransferRow(__v1200: *mut BtCursor, __v1201: *mut BtCursor, __v1202: i64)
    -> i32;
    fn sqlite3VdbeExpandSql(__v1203: *mut Vdbe, __v1204: *const i8) -> *mut i8;
    fn sqlite3MemCompare(
        __v1205: *const sqlite3_value,
        __v1206: *const sqlite3_value,
        __v1207: *const CollSeq,
    ) -> i32;
    fn sqlite3VdbeRecordUnpack(__v1208: i32, __v1209: *const (), __v1210: *mut UnpackedRecord);
    fn sqlite3VdbeRecordCompareWithSkip(
        __v1211: i32,
        __v1212: *const (),
        __v1213: *mut UnpackedRecord,
        __v1214: i32,
    ) -> i32;
    fn sqlite3VdbeAllocUnpackedRecord(__v1215: *mut KeyInfo) -> *mut UnpackedRecord;
    fn sqlite3ReportError(iErr: i32, lineno: i32, zType: *const i8) -> i32;
    fn sqlite3CorruptError(__v1219: i32) -> i32;
    fn sqlite3StrICmp(__v1220: *const i8, __v1221: *const i8) -> i32;
    fn sqlite3Strlen30(__v1222: *const i8) -> i32;
    fn sqlite3DbMallocZero(__v1223: *mut sqlite3, __v1224: u64) -> *mut ();
    fn sqlite3DbMallocRaw(__v1225: *mut sqlite3, __v1226: u64) -> *mut ();
    fn sqlite3DbMallocRawNN(__v1227: *mut sqlite3, __v1228: u64) -> *mut ();
    fn sqlite3DbStrDup(__v1229: *mut sqlite3, __v1230: *const i8) -> *mut i8;
    fn sqlite3DbFree(__v1231: *mut sqlite3, __v1232: *mut ());
    fn sqlite3DbFreeNN(__v1233: *mut sqlite3, __v1234: *mut ());
    fn sqlite3IsNaN(__v1235: f64) -> i32;
    fn sqlite3MPrintf(__v1236: *mut sqlite3, __v1237: *const i8, ...) -> *mut i8;
    fn sqlite3InitCallback(
        __v1238: *mut (),
        __v1239: i32,
        __v1240: *mut *mut i8,
        __v1241: *mut *mut i8,
    ) -> i32;
    fn sqlite3InitOne(
        __v1242: *mut sqlite3,
        __v1243: i32,
        __v1244: *mut *mut i8,
        __v1245: u32,
    ) -> i32;
    fn sqlite3ResetAllSchemasOfConnection(__v1246: *mut sqlite3);
    fn sqlite3ResetOneSchema(__v1247: *mut sqlite3, __v1248: i32);
    fn sqlite3RowSetInsert(__v1249: *mut RowSet, __v1250: i64);
    fn sqlite3RowSetTest(__v1251: *mut RowSet, iBatch: i32, __v1253: i64) -> i32;
    fn sqlite3RowSetNext(__v1254: *mut RowSet, __v1255: *mut i64) -> i32;
    fn sqlite3UnlinkAndDeleteTable(__v1256: *mut sqlite3, __v1257: i32, __v1258: *const i8);
    fn sqlite3UnlinkAndDeleteIndex(__v1259: *mut sqlite3, __v1260: i32, __v1261: *const i8);
    fn sqlite3RunVacuum(
        __v1262: *mut *mut i8,
        __v1263: *mut sqlite3,
        __v1264: i32,
        __v1265: *mut sqlite3_value,
    ) -> i32;
    fn sqlite3RollbackAll(__v1266: *mut sqlite3, __v1267: i32);
    fn sqlite3CloseSavepoints(__v1268: *mut sqlite3);
    fn sqlite3UnlinkAndDeleteTrigger(__v1269: *mut sqlite3, __v1270: i32, __v1271: *const i8);
    fn sqlite3RealSameAsInt(__v1272: f64, __v1273: i64) -> i32;
    fn sqlite3RealToI64(__v1274: f64) -> i64;
    fn sqlite3LogEst(__v1275: u64) -> i16;
    fn sqlite3PutVarint(__v1276: *mut u8, __v1277: u64) -> i32;
    fn sqlite3GetVarint32(__v1278: *const u8, __v1279: *mut u32) -> u8;
    fn sqlite3VarintLen(v: u64) -> i32;
    fn sqlite3Atoi64(__v1281: *const i8, __v1282: *mut i64, __v1283: i32, __v1284: u8) -> i32;
    fn sqlite3SystemError(__v1285: *mut sqlite3, __v1286: i32);
    fn sqlite3ErrStr(__v1287: i32) -> *const i8;
    fn sqlite3WritableSchema(__v1288: *mut sqlite3) -> i32;
    fn sqlite3VdbeSetChanges(__v1289: *mut sqlite3, __v1290: i64);
    fn sqlite3AddInt64(__v1291: *mut i64, __v1292: i64) -> i32;
    fn sqlite3SubInt64(__v1293: *mut i64, __v1294: i64) -> i32;
    fn sqlite3MulInt64(__v1295: *mut i64, __v1296: i64) -> i32;
    fn sqlite3ValueText(__v1297: *mut sqlite3_value, __v1298: u8) -> *const ();
    fn sqlite3RootPageMoved(__v1302: *mut sqlite3, __v1303: i32, __v1304: u32, __v1305: u32);
    fn sqlite3ExpirePreparedStatements(__v1306: *mut sqlite3, __v1307: i32);
    fn sqlite3AnalysisLoad(__v1308: *mut sqlite3, iDB: i32) -> i32;
    fn sqlite3SchemaClear(__v1310: *mut ());
    fn sqlite3OomFault(__v1311: *mut sqlite3) -> *mut ();
    fn sqlite3RCStrRef(__v1312: *mut i8) -> *mut i8;
    fn sqlite3RCStrUnref(__v1313: *mut ());
    fn sqlite3RCStrNew(__v1314: u64) -> *mut i8;
    fn sqlite3VtabLock(__v1315: *mut VTable);
    fn sqlite3VtabUnlock(__v1316: *mut VTable);
    fn sqlite3VtabSavepoint(__v1317: *mut sqlite3, __v1318: i32, __v1319: i32) -> i32;
    fn sqlite3VtabImportErrmsg(__v1320: *mut Vdbe, __v1321: *mut sqlite3_vtab);
    fn sqlite3VtabCallCreate(
        __v1322: *mut sqlite3,
        __v1323: i32,
        __v1324: *const i8,
        __v1325: *mut *mut i8,
    ) -> i32;
    fn sqlite3VtabCallDestroy(__v1326: *mut sqlite3, __v1327: i32, __v1328: *const i8) -> i32;
    fn sqlite3VtabBegin(__v1329: *mut sqlite3, __v1330: *mut VTable) -> i32;
    fn sqlite3JournalModename(__v1331: i32) -> *const i8;
    fn sqlite3Checkpoint(
        __v1332: *mut sqlite3,
        __v1333: i32,
        __v1334: i32,
        __v1335: *mut i32,
        __v1336: *mut i32,
    ) -> i32;
    fn sqlite3FkClearTriggerCache(__v1337: *mut sqlite3, __v1338: i32);
    fn sqlite3Get8byte(__v1339: *const u8) -> u64;
    fn sqlite3BSwap64(__v1340: u64) -> u64;
    fn sqlite3VdbeError(__v1341: *mut Vdbe, __v1342: *const i8, ...);
    fn sqlite3VdbeFreeCursor(__v1343: *mut Vdbe, __v1344: *mut VdbeCursor);
    fn sqlite3VdbeFreeCursorNN(__v1345: *mut Vdbe, __v1346: *mut VdbeCursor);
    fn sqlite3VdbeHandleMovedCursor(p: *mut VdbeCursor) -> i32;
    fn sqlite3VdbeFinishMoveto(__v1348: *mut VdbeCursor) -> i32;
    fn sqlite3VdbeCursorRestore(__v1349: *mut VdbeCursor) -> i32;
    fn sqlite3VdbeSerialTypeLen(__v1350: u32) -> u32;
    fn sqlite3VdbeOneByteSerialTypeLen(__v1351: u8) -> u8;
    fn sqlite3VdbeSerialGet(__v1352: *const u8, __v1353: u32, __v1354: *mut sqlite3_value);
    fn sqlite3VdbeDeleteAuxData(
        __v1355: *mut sqlite3,
        __v1356: *mut *mut AuxData,
        __v1357: i32,
        __v1358: i32,
    );
    fn sqlite3VdbeIdxKeyCompare(
        __v1359: *mut sqlite3,
        __v1360: *mut VdbeCursor,
        __v1361: *mut UnpackedRecord,
        __v1362: *mut i32,
    ) -> i32;
    fn sqlite3VdbeIdxRowid(__v1363: *mut sqlite3, __v1364: *mut BtCursor, __v1365: *mut i64)
    -> i32;
    fn sqlite3VdbeHalt(__v1367: *mut Vdbe) -> i32;
    fn sqlite3VdbeChangeEncoding(__v1368: *mut sqlite3_value, __v1369: i32) -> i32;
    fn sqlite3VdbeMemTooBig(__v1370: *mut sqlite3_value) -> i32;
    fn sqlite3VdbeMemCopy(__v1371: *mut sqlite3_value, __v1372: *const sqlite3_value) -> i32;
    fn sqlite3VdbeMemShallowCopy(
        __v1373: *mut sqlite3_value,
        __v1374: *const sqlite3_value,
        __v1375: i32,
    );
    fn sqlite3VdbeMemMove(__v1376: *mut sqlite3_value, __v1377: *mut sqlite3_value);
    fn sqlite3VdbeMemSetStr(
        __v1378: *mut sqlite3_value,
        __v1379: *const i8,
        __v1380: i64,
        __v1381: u8,
        __v1382: Option<unsafe extern "C-unwind" fn(*mut ())>,
    ) -> i32;
    fn sqlite3VdbeMemSetInt64(__v1383: *mut sqlite3_value, __v1384: i64);
    fn sqlite3VdbeMemSetPointer(
        __v1385: *mut sqlite3_value,
        __v1386: *mut (),
        __v1387: *const i8,
        __v1388: Option<unsafe extern "C-unwind" fn(*mut ())>,
    );
    fn sqlite3VdbeMemInit(__v1389: *mut sqlite3_value, __v1390: *mut sqlite3, __v1391: u16);
    fn sqlite3VdbeMemSetNull(__v1392: *mut sqlite3_value);
    fn sqlite3VdbeMemSetZeroBlob(__v1393: *mut sqlite3_value, __v1394: i32);
    fn sqlite3VdbeMemSetRowSet(__v1395: *mut sqlite3_value) -> i32;
    fn sqlite3VdbeMemMakeWriteable(__v1396: *mut sqlite3_value) -> i32;
    fn sqlite3VdbeMemStringify(__v1397: *mut sqlite3_value, __v1398: u8, __v1399: u8) -> i32;
    fn sqlite3IntFloatCompare(__v1400: i64, __v1401: f64) -> i32;
    fn sqlite3VdbeIntValue(__v1402: *const sqlite3_value) -> i64;
    fn sqlite3VdbeMemIntegerify(__v1403: *mut sqlite3_value) -> i32;
    fn sqlite3VdbeRealValue(__v1404: *mut sqlite3_value) -> f64;
    fn sqlite3MemRealValueRC(__v1405: *mut sqlite3_value, __v1406: *mut f64) -> i32;
    fn sqlite3VdbeBooleanValue(__v1407: *mut sqlite3_value, ifNull: i32) -> i32;
    fn sqlite3VdbeIntegerAffinity(__v1409: *mut sqlite3_value);
    fn sqlite3VdbeMemRealify(__v1410: *mut sqlite3_value) -> i32;
    fn sqlite3VdbeMemCast(__v1411: *mut sqlite3_value, __v1412: u8, __v1413: u8) -> i32;
    fn sqlite3VdbeMemFromBtree(
        __v1414: *mut BtCursor,
        __v1415: u32,
        __v1416: u32,
        __v1417: *mut sqlite3_value,
    ) -> i32;
    fn sqlite3VdbeMemFromBtreeZeroOffset(
        __v1418: *mut BtCursor,
        __v1419: u32,
        __v1420: *mut sqlite3_value,
    ) -> i32;
    fn sqlite3VdbeMemRelease(p: *mut sqlite3_value);
    fn sqlite3VdbeMemReleaseMalloc(p: *mut sqlite3_value);
    fn sqlite3VdbeMemFinalize(__v1423: *mut sqlite3_value, __v1424: *mut FuncDef) -> i32;
    fn sqlite3VdbeMemAggValue(
        __v1425: *mut sqlite3_value,
        __v1426: *mut sqlite3_value,
        __v1427: *mut FuncDef,
    ) -> i32;
    fn sqlite3VdbeMemGrow(pMem: *mut sqlite3_value, n: i32, preserve: i32) -> i32;
    fn sqlite3VdbeMemClearAndResize(pMem: *mut sqlite3_value, n: i32) -> i32;
    fn sqlite3VdbeFrameMemDel(__v1433: *mut ());
    fn sqlite3VdbeFrameRestore(__v1434: *mut VdbeFrame) -> i32;
    fn sqlite3VdbeFindIndexKey(
        __v1435: *mut BtCursor,
        __v1436: *mut Index,
        __v1437: *mut UnpackedRecord,
        __v1438: *mut i32,
        __v1439: i32,
    ) -> i32;
    fn sqlite3VdbeSorterInit(__v1440: *mut sqlite3, __v1441: i32, __v1442: *mut VdbeCursor) -> i32;
    fn sqlite3VdbeSorterReset(__v1443: *mut sqlite3, __v1444: *mut VdbeSorter);
    fn sqlite3VdbeSorterRowkey(__v1445: *const VdbeCursor, __v1446: *mut sqlite3_value) -> i32;
    fn sqlite3VdbeSorterNext(__v1447: *mut sqlite3, __v1448: *const VdbeCursor) -> i32;
    fn sqlite3VdbeSorterRewind(__v1449: *const VdbeCursor, __v1450: *mut i32) -> i32;
    fn sqlite3VdbeSorterWrite(__v1451: *const VdbeCursor, __v1452: *mut sqlite3_value) -> i32;
    fn sqlite3VdbeSorterCompare(
        __v1453: *const VdbeCursor,
        __v1454: *mut sqlite3_value,
        __v1455: i32,
        __v1456: *mut i32,
    ) -> i32;
    fn sqlite3VdbeValueListFree(__v1457: *mut ());
    fn sqlite3VdbeEnter(__v1458: *mut Vdbe);
    fn sqlite3VdbeLeave(__v1459: *mut Vdbe);
    fn sqlite3VdbeCheckFkImmediate(__v1460: *mut Vdbe) -> i32;
    fn sqlite3VdbeCheckFkDeferred(__v1461: *mut Vdbe) -> i32;
    fn sqlite3VdbeMemExpandBlob(__v1462: *mut sqlite3_value) -> i32;
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
struct sqlite3_pcache {}

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
struct BtreePayload {
    pKey: *const (),
    nKey: i64,
    pData: *const (),
    aMem: *mut sqlite3_value,
    nMem: u16,
    nData: i32,
    nZero: i32,
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
    trace: __SlateRecord178,
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
    u1: __SlateRecord179,
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
    u: __SlateRecord180,
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
    __slate_bits_0: __slate_bits::__SlateBits75U0,
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
    u: __SlateRecord181,
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
struct UnpackedRecord {
    pKeyInfo: *mut KeyInfo,
    aMem: *mut sqlite3_value,
    u: __SlateRecord186,
    n: i32,
    nField: u16,
    default_rc: i8,
    errCode: u8,
    r1: i8,
    r2: i8,
    eqSeen: u8,
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
    __slate_bits_0: __slate_bits::__SlateBits99U0,
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
    u: __SlateRecord190,
    pLeft: *mut Expr,
    pRight: *mut Expr,
    x: __SlateRecord191,
    nHeight: i32,
    iTable: i32,
    iColumn: i16,
    iAgg: i16,
    w: __SlateRecord192,
    pAggInfo: *mut AggInfo,
    y: __SlateRecord193,
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
struct RowSet {}

#[repr(C)]
#[derive(Clone, Copy)]
struct SrcItem {
    zName: *mut i8,
    zAlias: *mut i8,
    pSTab: *mut Table,
    fg: __SlateRecord200,
    iCursor: i32,
    colUsed: u64,
    u1: __SlateRecord201,
    u2: __SlateRecord202,
    u3: __SlateRecord203,
    u4: __SlateRecord204,
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
    __slate_bits_0: __slate_bits::__SlateBits111U0,
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
    u1: __SlateRecord206,
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
struct __SlateRecord209 {
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
struct VtabCtx {}

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
struct Pager {}

#[repr(C)]
#[derive(Clone, Copy)]
struct Btree {}

#[repr(C)]
#[derive(Clone, Copy)]
struct BtCursor {}

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
struct VdbeCursor {
    eCurType: u8,
    iDb: i8,
    nullRow: u8,
    deferredMoveto: u8,
    isTable: u8,
    __slate_bits_0: __slate_bits::__SlateBits220U0,
    seekHit: u16,
    ub: __SlateRecord222,
    seqCount: i64,
    cacheStatus: u32,
    seekResult: i32,
    pAltCursor: *mut VdbeCursor,
    uc: __SlateRecord223,
    pKeyInfo: *mut KeyInfo,
    iHdrOffset: u32,
    pgnoRoot: u32,
    nField: i16,
    nHdrParsed: u16,
    movetoTarget: i64,
    aOffset: *mut u32,
    aRow: *const u8,
    payloadSize: u32,
    szRow: u32,
    pCache: *mut VdbeTxtBlbCache,
    aType: [u32; 0],
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
    __slate_bits_0: __slate_bits::__SlateBits177U0,
    azInit: *mut *const i8,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord178 {
    xLegacy: Option<unsafe extern "C-unwind" fn(*mut (), *const i8)>,
    xV2: Option<unsafe extern "C-unwind" fn(u32, *mut (), *mut (), *mut ()) -> i32>,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord179 {
    isInterrupted: i32,
    notUsed1: f64,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord180 {
    pHash: *mut FuncDef,
    pDestructor: *mut FuncDestructor,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord181 {
    tab: __SlateRecord182,
    view: __SlateRecord183,
    vtab: __SlateRecord184,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord182 {
    addColOffset: i32,
    pFKey: *mut FKey,
    pDfltList: *mut ExprList,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord183 {
    pSelect: *mut Select,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord184 {
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
union __SlateRecord186 {
    z: *mut i8,
    i: i64,
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
union __SlateRecord190 {
    zToken: *mut i8,
    iValue: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord191 {
    pList: *mut ExprList,
    pSelect: *mut Select,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord192 {
    iJoin: i32,
    iOfst: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord193 {
    pTab: *mut Table,
    pWin: *mut Window,
    nReg: i32,
    sub: __SlateRecord194,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord194 {
    iAddr: i32,
    regReturn: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct ExprList_item {
    pExpr: *mut Expr,
    zEName: *mut i8,
    fg: __SlateRecord196,
    u: __SlateRecord197,
}

#[repr(C, align(4))]
#[derive(Clone, Copy)]
struct __SlateRecord196 {
    sortFlags: u8,
    __slate_bits_0: __slate_bits::__SlateBits196U0,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord197 {
    x: __SlateRecord198,
    iConstExprReg: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord198 {
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
struct __SlateRecord200 {
    jointype: u8,
    __slate_bits_0: __slate_bits::__SlateBits200U0,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord201 {
    zIndexedBy: *mut i8,
    pFuncArg: *mut ExprList,
    nRow: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord202 {
    pIBIndex: *mut Index,
    pCteUse: *mut CteUse,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord203 {
    pOn: *mut Expr,
    pUsing: *mut IdList,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord204 {
    pSchema: *mut Schema,
    zDatabase: *mut i8,
    pSubq: *mut Subquery,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord206 {
    cr: __SlateRecord207,
    d: __SlateRecord208,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord207 {
    addrCrTab: i32,
    regRowid: i32,
    regRoot: i32,
    constraintName: Token,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord208 {
    pReturning: *mut Returning,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct VdbeTxtBlbCache {
    pCValue: *mut i8,
    iOffset: i64,
    iCol: i32,
    cacheStatus: u32,
    colCacheCtr: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct VdbeFrame {
    v: *mut Vdbe,
    pParent: *mut VdbeFrame,
    aOp: *mut VdbeOp,
    aMem: *mut sqlite3_value,
    apCsr: *mut *mut VdbeCursor,
    aOnce: *mut u8,
    token: *mut (),
    lastRowid: i64,
    pAuxData: *mut AuxData,
    nCursor: i32,
    pc: i32,
    nOp: i32,
    nMem: i32,
    nChildMem: i32,
    nChildCsr: i32,
    nChange: i64,
    nDbChange: i64,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct VdbeSorter {}

#[repr(C)]
#[derive(Clone, Copy)]
struct sqlite3_value {
    u: MemValue,
    z: *mut i8,
    n: i32,
    flags: u16,
    enc: u8,
    eSubtype: u8,
    db: *mut sqlite3,
    szMalloc: i32,
    uTemp: u32,
    zMalloc: *mut i8,
    xDel: Option<unsafe extern "C-unwind" fn(*mut ())>,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct AuxData {
    iAuxOp: i32,
    iAuxArg: i32,
    pAux: *mut (),
    xDeleteAux: Option<unsafe extern "C-unwind" fn(*mut ())>,
    pNextAux: *mut AuxData,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct sqlite3_context {
    pOut: *mut sqlite3_value,
    pFunc: *mut FuncDef,
    pMem: *mut sqlite3_value,
    pVdbe: *mut Vdbe,
    iOp: i32,
    isError: i32,
    enc: u8,
    skipFlag: u8,
    argc: u16,
    argv: [*mut sqlite3_value; 0],
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord222 {
    pBtx: *mut Btree,
    aAltMap: *mut u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord223 {
    pCursor: *mut BtCursor,
    pVCur: *mut sqlite3_vtab_cursor,
    pSorter: *mut VdbeSorter,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct Vdbe {
    db: *mut sqlite3,
    ppVPrev: *mut *mut Vdbe,
    pVNext: *mut Vdbe,
    pParse: *mut Parse,
    nVar: i16,
    nMem: i32,
    nCursor: i32,
    cacheCtr: u32,
    pc: i32,
    rc: i32,
    nChange: i64,
    iStatement: i32,
    iCurrentTime: i64,
    nFkConstraint: i64,
    nStmtDefCons: i64,
    nStmtDefImmCons: i64,
    aMem: *mut sqlite3_value,
    apArg: *mut *mut sqlite3_value,
    apCsr: *mut *mut VdbeCursor,
    aVar: *mut sqlite3_value,
    aOp: *mut VdbeOp,
    nOp: i32,
    nOpAlloc: i32,
    aColName: *mut sqlite3_value,
    pResultRow: *mut sqlite3_value,
    zErrMsg: *mut i8,
    pVList: *mut i32,
    startTime: i64,
    nResColumn: u16,
    nResAlloc: u16,
    errorAction: u8,
    minWriteFileFormat: u8,
    prepFlags: u8,
    eVdbeState: u8,
    __slate_bits_0: __slate_bits::__SlateBits166U0,
    btreeMask: u32,
    lockMask: u32,
    aCounter: [u32; 9],
    zSql: *mut i8,
    pFree: *mut (),
    pFrame: *mut VdbeFrame,
    pDelFrame: *mut VdbeFrame,
    nFrame: i32,
    expmask: u32,
    smimask: u32,
    pProgram: *mut SubProgram,
    pAuxData: *mut AuxData,
}

#[repr(C)]
#[derive(Clone, Copy)]
union MemValue {
    r: f64,
    i: i64,
    nZero: i32,
    zPType: *const i8,
    pDef: *mut FuncDef,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct ValueList {
    pCsr: *mut BtCursor,
    pOut: *mut sqlite3_value,
}

#[repr(C, align(16))]
struct __SlateAlign16<T>(T);

mod __slate_bits {
    #[bitfields::bitfield([u8; 2], c_names = true, new = false, from_into_bits = false, from_traits = false, default = false, debug = false, builder = false, bit_ops = false)]
    pub struct __SlateBits111U0 {
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
    pub struct __SlateBits75U0 {
        #[bits(4)]
        pub notNull: u32,
        #[bits(4)]
        pub eCType: u32,
    }
    #[bitfields::bitfield([u8; 2], c_names = true, new = false, from_into_bits = false, from_traits = false, default = false, debug = false, builder = false, bit_ops = false)]
    pub struct __SlateBits99U0 {
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
    #[bitfields::bitfield([u8; 3], c_names = true, new = false, from_into_bits = false, from_traits = false, default = false, debug = false, builder = false, bit_ops = false)]
    pub struct __SlateBits200U0 {
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
    pub struct __SlateBits196U0 {
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
    pub struct __SlateBits220U0 {
        #[bits(1)]
        pub isEphemeral: u32,
        #[bits(1)]
        pub useRandomRowid: u32,
        #[bits(1)]
        pub isOrdered: u32,
        #[bits(1)]
        pub noReuse: u32,
        #[bits(1)]
        pub colCache: u32,
        #[bits(3, access = na)]
        pub __slate_pad_5: u8,
    }
    #[bitfields::bitfield(
        u16,
        c_names = true,
        new = false,
        from_into_bits = false,
        from_traits = false,
        default = false,
        debug = false,
        builder = false,
        bit_ops = false
    )]
    pub struct __SlateBits166U0 {
        #[bits(2)]
        pub expired: u32,
        #[bits(2)]
        pub explain: u32,
        #[bits(1)]
        pub changeCntOn: u32,
        #[bits(1)]
        pub usesStmtJournal: u32,
        #[bits(1)]
        pub readOnly: u32,
        #[bits(1)]
        pub bIsReader: u32,
        #[bits(1)]
        pub haveEqpOps: u32,
        #[bits(7, access = na)]
        pub __slate_pad_7: u8,
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
    pub struct __SlateBits177U0 {
        #[bits(1)]
        pub orphanTrigger: u32,
        #[bits(2)]
        pub imposterTable: u32,
        #[bits(1)]
        pub reopenMemdb: u32,
        #[bits(4, access = na)]
        pub __slate_pad_3: u8,
    }
}

// Invoke this macro on memory cells just prior to changing the
// value of the cell.  This macro verifies that shallow copies are
// not misused.  A shallow copy of a string or blob just copies a
// pointer to the string or blob, not the content.  If the original
// is changed while the copy is still in use, the string or blob might
// be changed out from under the copy.  This macro verifies that nothing
// like that ever happens.
// This macro evaluates to true if either the update hook or the preupdate
// hook are enabled for database connect DB.
// Test a register to see if it exceeds the current maximum blob size.
// If it does, record the new maximum blob size.
// Invoke the VDBE coverage callback, if that callback is defined.  This
// feature is used for test suite validation only and does not appear an
// production builds.
//
// M is the type of branch.  I is the direction taken for this instance of
// the branch.
//
//   M: 2 - two-way branch (I=0: fall-thru   1: jump                )
//      3 - two-way + NULL (I=0: fall-thru   1: jump      2: NULL   )
//      4 - OP_Jump        (I=0: jump p1     1: jump p2   2: jump p3)
//
// In other words, if M is 2, then I is either 0 (for fall-through) or
// 1 (for when the branch is taken).  If M is 3, the I is 0 for an
// ordinary fall-through, I is 1 if the branch was taken, and I is 2
// if the result of comparison is NULL.  For M=3, I=2 the jump may or
// may not be taken, depending on the SQLITE_JUMPIFNULL flags in p5.
// When M is 4, that means that an OP_Jump is being run.  I is 0, 1, or 2
// depending on if the operands are less than, equal, or greater than.
//
// iSrcLine is the source code line (from the __LINE__ macro) that
// generated the VDBE instruction combined with flag bits.  The source
// code line number is in the lower 24 bits of iSrcLine and the upper
// 8 bytes are flags.  The lower three bits of the flags indicate
// values for I that should never occur.  For example, if the branch is
// always taken, the flags should be 0x05 since the fall-through and
// alternate branch are never taken.  If a branch is never taken then
// flags should be 0x06 since only the fall-through approach is allowed.
//
// Bit 0x08 of the flags indicates an OP_Jump opcode that is only
// interested in equal or not-equal.  In other words, I==0 and I==2
// should be treated as equivalent
//
// Since only a line number is retained, not the filename, this macro
// only works for amalgamation builds.  But that is ok, since these macros
// should be no-ops except for special builds used to measure test coverage.
// An ephemeral string value (signified by the MEM_Ephem flag) contains
// a pointer to a dynamically allocated string where some other entity
// is responsible for deallocating that string.  Because the register
// does not control the string, it might be deleted without the register
// knowing it.
//
// This routine converts an ephemeral string into a dynamically allocated
// string that the register itself controls.  In other words, it
// converts an MEM_Ephem string into a string with P.z==P.zMalloc.
// Return true if the cursor was opened using the OP_OpenSorter opcode.
/// High-resolution hardware timer used for debugging and testing only.
/// The following global variable is incremented every time a cursor
/// moves, either by the OP_SeekXX, OP_Next, or OP_Prev opcodes.  The test
/// procedures use this information to make sure that indices are
/// working correctly.  This variable has no function other than to
/// help verify the correct operation of the library.
/// When this global variable is positive, it gets decremented once before
/// each instruction in the VDBE.  When it reaches zero, the u1.isInterrupted
/// field of the sqlite3 structure is set in order to simulate an interrupt.
///
/// This facility is used for testing purposes only.  It does not function
/// in an ordinary build.
/// The next global variable is incremented each type the OP_Sort opcode
/// is executed.  The test procedures use this information to make sure that
/// sorting is occurring or not occurring at appropriate times.   This variable
/// has no function other than to help verify the correct operation of the
/// library.
/// The next global variable records the size of the largest MEM_Blob
/// or MEM_Str that has been used by a VDBE opcode.  The test procedures
/// use this information to make sure that the zero-blob functionality
/// is working correctly.   This variable has no function other than to
/// help verify the correct operation of the library.
/// The next global variable is incremented each time the OP_Found opcode
/// is executed. This is used to test whether or not the foreign key
/// operation implemented using OP_FkIsZero is working. This variable
/// has no function other than to help verify the correct operation of the
/// library.
/// Allocate VdbeCursor number iCur.  Return a pointer to it.  Return NULL
/// if we run out of memory.
///
/// # Arguments
///
/// * `p` - The virtual machine
/// * `iCur` - Index of the new VdbeCursor
/// * `nField` - Number of fields in the table or index
/// * `eCurType` - Type of the new cursor
fn allocateCursor(
    mut p: *mut Vdbe,
    mut iCur: i32,
    mut nField: i32,
    mut eCurType: u8,
) -> *mut VdbeCursor {
    // Find the memory cell that will be used to store the blob of memory
    // required for this VdbeCursor structure. It is convenient to use a
    // vdbe memory cell to manage the memory allocation required for a
    // VdbeCursor structure for the following reasons:
    //
    //   * Sometimes cursor numbers are used for a couple of different
    //     purposes in a vdbe program. The different uses might require
    //     different sized allocations. Memory cells provide growable
    //     allocations.
    //
    //   * When using ENABLE_MEMORY_MANAGEMENT, memory cell buffers can
    //     be freed lazily via the sqlite3_release_memory() API. This
    //     minimizes the number of malloc calls made by the system.
    //
    // The memory cell for cursor 0 is aMem[0]. The rest are allocated from
    // the top of the register space.  Cursor 1 is at Mem[p->nMem-1].
    // Cursor 2 is at Mem[p->nMem-2]. And so forth.
    let mut pMem: *mut sqlite3_value = if iCur > (0 as i32) {
        unsafe { unsafe { (*p).aMem }.offset(((unsafe { (*p).nMem }) - iCur) as isize) }
    } else {
        unsafe { (*p).aMem }
    };
    let mut nByte: i64 = 0 as i64;
    let mut pCx: *mut VdbeCursor = std::ptr::null_mut::<VdbeCursor>();
    nByte = ((112 as u64).wrapping_add(((7 as i32) as i64) as u64) & ((!(7 as i32) as i64) as u64))
        .wrapping_add((((nField + (1 as i32)) as i64) as u64).wrapping_mul(8 as u64))
        as i64;
    0 as i32;
    if ((eCurType as u32) as i32) == (0 as i32) {
        let __v1967: i64 = nByte;
        let __v1968: i64 = __v1967 + ((unsafe { sqlite3BtreeCursorSize() }) as i64);
        nByte = __v1968;
    }
    0 as i32;
    if (unsafe { *unsafe { unsafe { (*p).apCsr }.offset(iCur as isize) } })
        != std::ptr::null_mut::<VdbeCursor>()
    {
        unsafe {
            sqlite3VdbeFreeCursorNN(p, unsafe {
                *unsafe { unsafe { (*p).apCsr }.offset(iCur as isize) }
            })
        };
        unsafe {
            *unsafe { unsafe { (*p).apCsr }.offset(iCur as isize) } =
                std::ptr::null_mut::<VdbeCursor>();
        }
    }
    // There used to be a call to sqlite3VdbeMemClearAndResize() to make sure
    // the pMem used to hold space for the cursor has enough storage available
    // in pMem->zMalloc.  But for the special case of the aMem[] entries used
    // to hold cursors, it is faster to in-line the logic.
    0 as i32;
    0 as i32;
    0 as i32;
    if ((unsafe { (*pMem).szMalloc }) as i64) < nByte {
        if (unsafe { (*pMem).szMalloc }) > (0 as i32) {
            unsafe {
                sqlite3DbFreeNN(
                    unsafe { (*pMem).db },
                    (unsafe { (*pMem).zMalloc }) as *mut (),
                )
            };
        }
        let __v1969: *mut i8 =
            (unsafe { sqlite3DbMallocRaw(unsafe { (*pMem).db }, nByte as u64) }) as *mut i8;
        unsafe {
            (*pMem).zMalloc = __v1969;
        }
        unsafe {
            (*pMem).z = __v1969;
        }
        if (unsafe { (*pMem).zMalloc }) == std::ptr::null_mut::<i8>() {
            unsafe {
                (*pMem).szMalloc = 0 as i32;
            }
            return std::ptr::null_mut::<VdbeCursor>();
        }
        unsafe {
            (*pMem).szMalloc = nByte as i32;
        }
    }
    let __v1970: *mut VdbeCursor = (unsafe { (*pMem).zMalloc }) as *mut VdbeCursor;
    pCx = __v1970;
    unsafe {
        *unsafe { unsafe { (*p).apCsr }.offset(iCur as isize) } = __v1970;
    }
    unsafe { memset(pCx as *mut (), 0 as i32, 32 as u64) };
    unsafe {
        (*pCx).eCurType = eCurType;
    }
    unsafe {
        (*pCx).nField = nField as i16;
    }
    unsafe {
        (*pCx).aOffset = unsafe {
            unsafe { std::ptr::addr_of_mut!((*pCx).aType) as *mut u32 }.offset(nField as isize)
        };
    }
    if ((eCurType as u32) as i32) == (0 as i32) {
        0 as i32;
        unsafe {
            (*pCx).uc.pCursor = (unsafe {
                unsafe { (*pMem).z }.offset(
                    ((112 as u64).wrapping_add(((7 as i32) as i64) as u64)
                        & ((!(7 as i32) as i64) as u64))
                        .wrapping_add(
                            (((nField + (1 as i32)) as i64) as u64).wrapping_mul(8 as u64),
                        ) as isize,
                )
            }) as *mut BtCursor;
        }
        unsafe { sqlite3BtreeCursorZero(unsafe { (*pCx).uc.pCursor }) };
    }
    return pCx;
}

/// The string in pRec is known to look like an integer and to have a
/// floating point value of rValue.  Return true and set *piValue to the
/// integer value if the string is in range to be an integer.  Otherwise,
/// return false.
fn alsoAnInt(mut pRec: *mut sqlite3_value, mut rValue: f64, mut piValue: *mut i64) -> i32 {
    let mut iValue: i64 = 0 as i64;
    iValue = unsafe { sqlite3RealToI64(rValue) };
    if (unsafe { sqlite3RealSameAsInt(rValue, iValue) }) != (0 as i32) {
        unsafe {
            *piValue = iValue;
        }
        return 1 as i32;
    }
    return ((0 as i32)
        == unsafe {
            sqlite3Atoi64(
                (unsafe { (*pRec).z }) as *const i8,
                piValue,
                unsafe { (*pRec).n },
                unsafe { (*pRec).enc },
            )
        }) as i32;
}

/// Try to convert a value into a numeric representation if we can
/// do so without loss of information.  In other words, if the string
/// looks like a number, convert it into a number.  If it does not
/// look like a number, leave it alone.
///
/// If the bTryForInt flag is true, then extra effort is made to give
/// an integer representation.  Strings that look like floating point
/// values but which have no fractional component (example: '48.00')
/// will have a MEM_Int representation when bTryForInt is true.
///
/// If bTryForInt is false, then if the input string contains a decimal
/// point or exponential notation, the result is only MEM_Real, even
/// if there is an exact integer representation of the quantity.
fn applyNumericAffinity(mut pRec: *mut sqlite3_value, mut bTryForInt: i32) {
    let mut rValue: f64 = 0 as f64;
    let mut rc: i32 = 0 as i32;
    0 as i32;
    rc = unsafe { sqlite3MemRealValueRC(pRec, std::ptr::addr_of_mut!(rValue)) };
    if rc <= (0 as i32) {
        return;
    }
    let __v1971: bool;
    if rc & (2 as i32) == (0 as i32) {
        __v1971 =
            alsoAnInt(pRec, rValue, unsafe { std::ptr::addr_of_mut!((*pRec).u.i) }) != (0 as i32);
    } else {
        __v1971 = false as bool;
    }
    if __v1971 {
        let __v1972: *mut sqlite3_value = pRec;
        let __v1973: u16 = unsafe { (*__v1972).flags };
        let __v1974: u16 = ((((__v1973 as u32) as i32) | (4 as i32)) as i16) as u16;
        unsafe {
            (*__v1972).flags = __v1974;
        }
    } else {
        unsafe {
            (*pRec).u.r = rValue;
        }
        let __v1975: *mut sqlite3_value = pRec;
        let __v1976: u16 = unsafe { (*__v1975).flags };
        let __v1977: u16 = ((((__v1976 as u32) as i32) | (8 as i32)) as i16) as u16;
        unsafe {
            (*__v1975).flags = __v1977;
        }
        if bTryForInt != (0 as i32) {
            unsafe { sqlite3VdbeIntegerAffinity(pRec) };
        }
    }
    // TEXT->NUMERIC is many->one.  Hence, it is important to invalidate the
    // string representation after computing a numeric equivalent, because the
    // string representation might not be the canonical representation for the
    // numeric value.  Ticket [343634942dd54ab57b7024] 2018-01-31.
    let __v1978: *mut sqlite3_value = pRec;
    let __v1979: u16 = unsafe { (*__v1978).flags };
    let __v1980: u16 = ((((__v1979 as u32) as i32) & !(2 as i32)) as i16) as u16;
    unsafe {
        (*__v1978).flags = __v1980;
    }
}

/// Processing is determine by the affinity parameter:
///
/// SQLITE_AFF_INTEGER:
/// SQLITE_AFF_REAL:
/// SQLITE_AFF_NUMERIC:
///    Try to convert pRec to an integer representation or a
///    floating-point representation if an integer representation
///    is not possible.  Note that the integer representation is
///    always preferred, even if the affinity is REAL, because
///    an integer representation is more space efficient on disk.
///
/// SQLITE_AFF_FLEXNUM:
///    If the value is text, then try to convert it into a number of
///    some kind (integer or real) but do not make any other changes.
///
/// SQLITE_AFF_TEXT:
///    Convert pRec to a text representation.
///
/// SQLITE_AFF_BLOB:
/// SQLITE_AFF_NONE:
///    No-op.  pRec is unchanged.
///
/// # Arguments
///
/// * `pRec` - The value to apply affinity to
/// * `affinity` - The affinity to be applied
/// * `enc` - Use this text encoding
fn applyAffinity(mut pRec: *mut sqlite3_value, mut affinity: i8, mut enc: u8) {
    if (affinity as i32) >= (67 as i32) {
        0 as i32;
        if (((unsafe { (*pRec).flags }) as u32) as i32) & (4 as i32) == (0 as i32) {
            if (((unsafe { (*pRec).flags }) as u32) as i32) & ((8 as i32) | (32 as i32))
                == (0 as i32)
            {
                if (((unsafe { (*pRec).flags }) as u32) as i32) & (2 as i32) != (0 as i32) {
                    applyNumericAffinity(pRec, 1 as i32);
                }
            } else {
                if (affinity as i32) <= (69 as i32) {
                    unsafe { sqlite3VdbeIntegerAffinity(pRec) };
                }
            }
        }
    } else {
        if (affinity as i32) == (66 as i32) {
            // Only attempt the conversion to TEXT if there is an integer or real
            // representation (blob and NULL do not get converted) but no string
            // representation.  It would be harmless to repeat the conversion if
            // there is already a string rep, but it is pointless to waste those
            // CPU cycles.
            if (0 as i32) == (((unsafe { (*pRec).flags }) as u32) as i32) & (2 as i32) {
                if (((unsafe { (*pRec).flags }) as u32) as i32)
                    & ((8 as i32) | (4 as i32) | (32 as i32))
                    != (0 as i32)
                {
                    {}
                    {}
                    {}
                    unsafe { sqlite3VdbeMemStringify(pRec, enc, ((1 as i32) as i8) as u8) };
                }
            }
            let __v1981: *mut sqlite3_value = pRec;
            let __v1982: u16 = unsafe { (*__v1981).flags };
            let __v1983: u16 = ((((__v1982 as u32) as i32)
                & !((8 as i32) | (4 as i32) | (32 as i32))) as i16)
                as u16;
            unsafe {
                (*__v1981).flags = __v1983;
            }
        }
    }
}

/// Try to convert the type of a function argument or a result column
/// into a numeric representation.  Use either INTEGER or REAL whichever
/// is appropriate.  But only do the conversion if it is possible without
/// loss of information and return the revised type of the argument.
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.vdbe.sqlite3_value_numeric_type")]
extern "C-unwind" fn sqlite3_value_numeric_type(mut pVal: *mut sqlite3_value) -> i32 {
    let mut eType: i32 = unsafe { sqlite3_value_type(pVal) };
    if eType == (3 as i32) {
        let mut pMem: *mut sqlite3_value = pVal;
        let mut pMutex: *mut sqlite3_mutex =
            if (unsafe { (*pMem).db }) != std::ptr::null_mut::<sqlite3>() {
                unsafe { (*unsafe { (*pMem).db }).mutex }
            } else {
                std::ptr::null_mut::<sqlite3_mutex>()
            };
        unsafe { sqlite3_mutex_enter(pMutex) };
        applyNumericAffinity(pMem, 0 as i32);
        unsafe { sqlite3_mutex_leave(pMutex) };
        eType = unsafe { sqlite3_value_type(pVal) };
    }
    return eType;
}

/// Exported version of applyAffinity(). This one works on sqlite3_value*,
/// not the internal Mem* type.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3ValueApplyAffinity(
    mut pVal: *mut sqlite3_value,
    mut affinity: u8,
    mut enc: u8,
) {
    applyAffinity(pVal, affinity as i8, enc);
}

/// pMem currently only holds a string type (or maybe a BLOB that we can
/// interpret as a string if we want to).  Compute its corresponding
/// numeric type, if has one.  Set the pMem->u.r and pMem->u.i fields
/// accordingly.
fn computeNumericType(mut pMem: *mut sqlite3_value) -> u16 {
    let mut rc: i32 = 0 as i32;
    let mut ix: i64 = 0 as i64;
    0 as i32;
    0 as i32;
    let __v1984: i32;
    if (((unsafe { (*pMem).flags }) as u32) as i32) & (1024 as i32) != (0 as i32) {
        __v1984 = unsafe { sqlite3VdbeMemExpandBlob(pMem) };
    } else {
        __v1984 = 0 as i32;
    }
    if __v1984 != (0 as i32) {
        unsafe {
            (*pMem).u.i = (0 as i32) as i64;
        }
        return ((4 as i32) as i16) as u16;
    }
    rc = unsafe { sqlite3MemRealValueRC(pMem, unsafe { std::ptr::addr_of_mut!((*pMem).u.r) }) };
    if rc <= (0 as i32) {
        let __v1985: bool;
        if rc & (2 as i32) == (0 as i32) {
            __v1985 = (unsafe {
                sqlite3Atoi64(
                    (unsafe { (*pMem).z }) as *const i8,
                    std::ptr::addr_of_mut!(ix),
                    unsafe { (*pMem).n },
                    unsafe { (*pMem).enc },
                )
            }) <= (1 as i32);
        } else {
            __v1985 = false as bool;
        }
        if __v1985 {
            unsafe {
                (*pMem).u.i = ix;
            }
            return ((4 as i32) as i16) as u16;
        } else {
            return ((8 as i32) as i16) as u16;
        }
    } else {
        let __v1986: bool;
        if rc & (2 as i32) == (0 as i32) {
            __v1986 = (unsafe {
                sqlite3Atoi64(
                    (unsafe { (*pMem).z }) as *const i8,
                    std::ptr::addr_of_mut!(ix),
                    unsafe { (*pMem).n },
                    unsafe { (*pMem).enc },
                )
            }) == (0 as i32);
        } else {
            __v1986 = false as bool;
        }
        if __v1986 {
            unsafe {
                (*pMem).u.i = ix;
            }
            return ((4 as i32) as i16) as u16;
        }
    }
    return ((8 as i32) as i16) as u16;
}

/// Return the numeric type for pMem, either MEM_Int or MEM_Real or both or
/// none.
///
/// Unlike applyNumericAffinity(), this routine does not modify pMem->flags.
/// But it does set pMem->u.r and pMem->u.i appropriately.
fn numericType(mut pMem: *mut sqlite3_value) -> u16 {
    0 as i32;
    if (((unsafe { (*pMem).flags }) as u32) as i32)
        & ((4 as i32) | (8 as i32) | (32 as i32) | (1 as i32))
        != (0 as i32)
    {
        {}
        {}
        {}
        return (((((unsafe { (*pMem).flags }) as u32) as i32)
            & ((4 as i32) | (8 as i32) | (32 as i32) | (1 as i32))) as i16) as u16;
    }
    0 as i32;
    {}
    {}
    return computeNumericType(pMem);
    return ((0 as i32) as i16) as u16;
}

/// Return the register of pOp->p2 after first preparing it to be
/// overwritten with an integer value.
fn out2PrereleaseWithClear(mut pOut: *mut sqlite3_value) -> *mut sqlite3_value {
    unsafe { sqlite3VdbeMemSetNull(pOut) };
    unsafe {
        (*pOut).flags = ((4 as i32) as i16) as u16;
    }
    return pOut;
}

fn out2Prerelease(mut p: *mut Vdbe, mut pOp: *mut VdbeOp) -> *mut sqlite3_value {
    let mut pOut: *mut sqlite3_value = unsafe { std::mem::zeroed() };
    0 as i32;
    0 as i32;
    pOut = unsafe { unsafe { (*p).aMem }.offset((unsafe { (*pOp).p2 }) as isize) };
    {}
    if (((unsafe { (*pOut).flags }) as u32) as i32) & ((32768 as i32) | (4096 as i32)) != (0 as i32)
    {
        return out2PrereleaseWithClear(pOut);
    } else {
        unsafe {
            (*pOut).flags = ((4 as i32) as i16) as u16;
        }
        return pOut;
    }
    return unsafe { std::mem::zeroed() };
}

/// Compute a bloom filter hash using pOp->p4.i registers from aMem[] beginning
/// with pOp->p3.  Return the hash.
///
/// IMPORTANT RESTRICTION (tag-202607231411):  This hash is only valid if the
/// collating sequence for TEXT is BINARY. Hence, Bloom filters that use this
/// hash will not work for look-ups that use any other collating sequence.
fn filterHash(mut aMem: *const sqlite3_value, mut pOp: *const VdbeOp) -> u64 {
    let mut i: i32 = 0 as i32;
    let mut mx: i32 = 0 as i32;
    let mut h: u64 = ((0 as i32) as i64) as u64;
    0 as i32;
    i = unsafe { (*pOp).p3 };
    let __v1987: i32 = i + unsafe { (*pOp).p4.i };
    mx = __v1987;
    '__slate_break_1463: loop {
        if !(i < mx) {
            break;
        }
        let mut p: *const sqlite3_value = unsafe { aMem.offset(i as isize) };
        if (((unsafe { (*p).flags }) as u32) as i32) & ((4 as i32) | (32 as i32)) != (0 as i32) {
            let __v1990: u64 = h;
            let __v1991: u64 = __v1990.wrapping_add((unsafe { (*p).u.i }) as u64);
            h = __v1991;
        } else {
            if (((unsafe { (*p).flags }) as u32) as i32) & (8 as i32) != (0 as i32) {
                let __v1992: u64 = h;
                let __v1993: u64 = __v1992.wrapping_add((unsafe { sqlite3VdbeIntValue(p) }) as u64);
                h = __v1993;
            } else {
                if (((unsafe { (*p).flags }) as u32) as i32) & (2 as i32) != (0 as i32) {
                    let mut x: u64 = 0 as u64;
                    let __v1994: u64 = h;
                    let __v1995: u64 = __v1994.wrapping_add(((unsafe { (*p).n }) as i64) as u64);
                    h = __v1995;
                    if (unsafe { (*p).n }) >= (((8 as u64) as u32) as i32) {
                        unsafe {
                            memcpy(
                                std::ptr::addr_of_mut!(x) as *mut (),
                                (unsafe { (*p).z }) as *const (),
                                8 as u64,
                            )
                        };
                        let __v1996: u64 = h;
                        let __v1997: u64 = __v1996.wrapping_add(x);
                        h = __v1997;
                        unsafe {
                            memcpy(
                                std::ptr::addr_of_mut!(x) as *mut (),
                                (unsafe {
                                    unsafe {
                                        unsafe { (*p).z }.offset((unsafe { (*p).n }) as isize)
                                    }
                                    .offset(-((8 as u64) as isize))
                                }) as *const (),
                                8 as u64,
                            )
                        };
                        let __v1998: u64 = h;
                        let __v1999: u64 = __v1998.wrapping_add(x);
                        h = __v1999;
                    } else {
                        x = ((0 as i32) as i64) as u64;
                        unsafe {
                            memcpy(
                                std::ptr::addr_of_mut!(x) as *mut (),
                                (unsafe { (*p).z }) as *const (),
                                ((unsafe { (*p).n }) as i64) as u64,
                            )
                        };
                        let __v2000: u64 = h;
                        let __v2001: u64 = __v2000.wrapping_add(x);
                        h = __v2001;
                    }
                } else {
                    if (((unsafe { (*p).flags }) as u32) as i32) & (16 as i32) != (0 as i32) {
                        let mut n: i32 = unsafe { (*p).n };
                        let mut x: u64 = ((0 as i32) as i64) as u64;
                        if n != (0 as i32) {
                            unsafe {
                                memcpy(
                                    std::ptr::addr_of_mut!(x) as *mut (),
                                    (unsafe { (*p).z }) as *const (),
                                    ((if n < (((8 as u64) as u32) as i32) {
                                        n
                                    } else {
                                        ((8 as u64) as u32) as i32
                                    }) as i64) as u64,
                                )
                            };
                            let __v2002: u64 = h;
                            let __v2003: u64 = __v2002.wrapping_add(x);
                            h = __v2003;
                        }
                        let __v2004: u64 = h;
                        let __v2005: u64 = __v2004.wrapping_add((n as i64) as u64);
                        h = __v2005;
                        if (((unsafe { (*p).flags }) as u32) as i32) & (1024 as i32) != (0 as i32) {
                            let __v2006: u64 = h;
                            let __v2007: u64 =
                                __v2006.wrapping_add(((unsafe { (*p).u.nZero }) as i64) as u64);
                            h = __v2007;
                        }
                    }
                }
            }
        }
        let __v1988: i32 = i;
        let __v1989: i32 = __v1988 + (1 as i32);
        i = __v1989;
    }
    return h;
}

/// For OP_Column, factor out the case where content is loaded from
/// overflow pages, so that the code to implement this case is separate
/// the common case where all content fits on the page.  Factoring out
/// the code reduces register pressure and helps the common case
/// to run faster.
///
/// # Arguments
///
/// * `pC` - The BTree cursor from which we are reading
/// * `iCol` - The column to read
/// * `t` - The serial-type code for the column value
/// * `iOffset` - Offset to the start of the content value
/// * `cacheStatus` - Current Vdbe.cacheCtr value
/// * `colCacheCtr` - Current value of the column cache counter
/// * `pDest` - Store the value into this register.
fn vdbeColumnFromOverflow(
    mut pC: *mut VdbeCursor,
    mut iCol: i32,
    mut t: u32,
    mut iOffset: i64,
    mut cacheStatus: u32,
    mut colCacheCtr: u32,
    mut pDest: *mut sqlite3_value,
) -> i32 {
    let mut rc: i32 = 0 as i32;
    let mut db: *mut sqlite3 = unsafe { (*pDest).db };
    let mut encoding: i32 = ((unsafe { (*pDest).enc }) as u32) as i32;
    let mut len: i32 = (unsafe { sqlite3VdbeSerialTypeLen(t) }) as i32;
    0 as i32;
    if len
        > unsafe {
            *unsafe { unsafe { (*db).aLimit.as_mut_ptr() as *mut i32 }.offset((0 as i32) as isize) }
        }
    {
        return 18 as i32;
    }
    if len > (4000 as i32) && (unsafe { (*pC).pKeyInfo }) == std::ptr::null_mut::<KeyInfo>() {
        // Cache large column values that are on overflow pages using
        // an RCStr (reference counted string) so that if they are reloaded,
        // that do not have to be copied a second time.  The overhead of
        // creating and managing the cache is such that this is only
        // profitable for larger TEXT and BLOB values.
        //
        // Only do this on table-btrees so that writes to index-btrees do not
        // need to clear the cache.  This buys performance in the common case
        // in exchange for generality.
        let mut pCache: *mut VdbeTxtBlbCache = unsafe { std::mem::zeroed() };
        let mut pBuf: *mut i8 = unsafe { std::mem::zeroed() };
        if ((unsafe { (*pC).__slate_bits_0.__get_colCache() }) as i32) == (0 as i32) {
            unsafe {
                (*pC).pCache =
                    (unsafe { sqlite3DbMallocZero(db, 32 as u64) }) as *mut VdbeTxtBlbCache;
            }
            if (unsafe { (*pC).pCache }) == std::ptr::null_mut::<VdbeTxtBlbCache>() {
                return 7 as i32;
            }
            unsafe {
                (*pC).__slate_bits_0.__set_colCache((1 as i32) as u32);
            }
        }
        pCache = unsafe { (*pC).pCache };
        let __v2008: bool;
        if (unsafe { (*pCache).pCValue }) == std::ptr::null_mut::<i8>()
            || (unsafe { (*pCache).iCol }) != iCol
            || (unsafe { (*pCache).cacheStatus }) != cacheStatus
            || (unsafe { (*pCache).colCacheCtr }) != colCacheCtr
        {
            __v2008 = true as bool;
        } else {
            __v2008 = (unsafe { (*pCache).iOffset })
                != unsafe { sqlite3BtreeOffset(unsafe { (*pC).uc.pCursor }) };
        }
        if __v2008 {
            if (unsafe { (*pCache).pCValue }) != std::ptr::null_mut::<i8>() {
                unsafe { sqlite3RCStrUnref((unsafe { (*pCache).pCValue }) as *mut ()) };
            }
            let __v2009: *mut i8 = unsafe { sqlite3RCStrNew(((len + (3 as i32)) as i64) as u64) };
            unsafe {
                (*pCache).pCValue = __v2009;
            }
            pBuf = __v2009;
            if pBuf == std::ptr::null_mut::<i8>() {
                return 7 as i32;
            }
            rc = unsafe {
                sqlite3BtreePayload(
                    unsafe { (*pC).uc.pCursor },
                    (iOffset as i32) as u32,
                    len as u32,
                    pBuf as *mut (),
                )
            };
            if rc != (0 as i32) {
                return rc;
            }
            unsafe {
                *unsafe { pBuf.offset(len as isize) } = (0 as i32) as i8;
            }
            unsafe {
                *unsafe { pBuf.offset((len + (1 as i32)) as isize) } = (0 as i32) as i8;
            }
            unsafe {
                *unsafe { pBuf.offset((len + (2 as i32)) as isize) } = (0 as i32) as i8;
            }
            unsafe {
                (*pCache).iCol = iCol;
            }
            unsafe {
                (*pCache).cacheStatus = cacheStatus;
            }
            unsafe {
                (*pCache).colCacheCtr = colCacheCtr;
            }
            unsafe {
                (*pCache).iOffset = unsafe { sqlite3BtreeOffset(unsafe { (*pC).uc.pCursor }) };
            }
        } else {
            pBuf = unsafe { (*pCache).pCValue };
        }
        0 as i32;
        unsafe { sqlite3RCStrRef(pBuf) };
        if t & ((1 as i32) as u32) != (0 as u32) {
            rc = unsafe {
                sqlite3VdbeMemSetStr(
                    pDest,
                    pBuf as *const i8,
                    len as i64,
                    (encoding as i8) as u8,
                    unsafe {
                        std::mem::transmute::<*const (), Option<unsafe extern "C-unwind" fn(*mut ())>>(
                            sqlite3RCStrUnref as *const (),
                        )
                    },
                )
            };
            let __v2010: *mut sqlite3_value = pDest;
            let __v2011: u16 = unsafe { (*__v2010).flags };
            let __v2012: u16 = ((((__v2011 as u32) as i32) | (512 as i32)) as i16) as u16;
            unsafe {
                (*__v2010).flags = __v2012;
            }
        } else {
            rc = unsafe {
                sqlite3VdbeMemSetStr(
                    pDest,
                    pBuf as *const i8,
                    len as i64,
                    ((0 as i32) as i8) as u8,
                    unsafe {
                        std::mem::transmute::<*const (), Option<unsafe extern "C-unwind" fn(*mut ())>>(
                            sqlite3RCStrUnref as *const (),
                        )
                    },
                )
            };
        }
    } else {
        rc = unsafe {
            sqlite3VdbeMemFromBtree(
                unsafe { (*pC).uc.pCursor },
                (iOffset as i32) as u32,
                len as u32,
                pDest,
            )
        };
        if rc != (0 as i32) {
            return rc;
        }
        unsafe { sqlite3VdbeSerialGet((unsafe { (*pDest).z }) as *const u8, t, pDest) };
        if t & ((1 as i32) as u32) != ((0 as i32) as u32) && encoding == (1 as i32) {
            unsafe {
                *unsafe { unsafe { (*pDest).z }.offset(len as isize) } = (0 as i32) as i8;
            }
            let __v2013: *mut sqlite3_value = pDest;
            let __v2014: u16 = unsafe { (*__v2013).flags };
            let __v2015: u16 = ((((__v2014 as u32) as i32) | (512 as i32)) as i16) as u16;
            unsafe {
                (*__v2013).flags = __v2015;
            }
        }
    }
    let __v2016: *mut sqlite3_value = pDest;
    let __v2017: u16 = unsafe { (*__v2016).flags };
    let __v2018: u16 = ((((__v2017 as u32) as i32) & !(16384 as i32)) as i16) as u16;
    unsafe {
        (*__v2016).flags = __v2018;
    }
    return rc;
}

/// Memory cell pMem may contain a blob or a NULL value. Cursor pCsr is
/// open on an index. If the current index entry matches the blob value in
/// pMem byte-for-byte, set pMem to NULL and return 1. Otherwise, return 0.
///
/// If an error occurs, set (*pRc) to an SQLite error code. Return 1 in this
/// case as well.
///
/// # Arguments
///
/// * `pCsr` - Cursor to compare key to
fn vdbeIndexKeyCompare(
    mut pCsr: *mut BtCursor,
    mut pMem: *mut sqlite3_value,
    mut pRc: *mut i32,
) -> i32 {
    let mut ret: i32 = 0 as i32;
    let mut nKey: u32 = (0 as i32) as u32;
    0 as i32;
    nKey = unsafe { sqlite3BtreePayloadSize(pCsr) };
    if nKey == ((unsafe { (*pMem).n }) as u32)
        && (((unsafe { (*pMem).flags }) as u32) as i32) & (16 as i32) != (0 as i32)
    {
        // This code could just use sqlite3BtreePayloadFetch(). But calling that
        // function here apparently prevents compilers from inlining it in other,
        // more performance critical, places. So this code uses
        // MemFromBtreeZeroOffset(), which is just as fast in most cases, but also
        // handles the case where the index record uses overflow pages.
        let mut m: sqlite3_value = unsafe { std::mem::zeroed() };
        unsafe { memset(std::ptr::addr_of_mut!(m) as *mut (), 0 as i32, 56 as u64) };
        unsafe {
            *pRc =
                unsafe { sqlite3VdbeMemFromBtreeZeroOffset(pCsr, nKey, std::ptr::addr_of_mut!(m)) };
        }
        ret = ((unsafe { *pRc }) != (0 as i32)
            || (0 as i32)
                == unsafe {
                    memcmp(
                        (unsafe { (*pMem).z }) as *const (),
                        m.z as *const (),
                        nKey as u64,
                    )
                }) as i32;
        unsafe { sqlite3VdbeMemReleaseMalloc(std::ptr::addr_of_mut!(m)) };
    }
    return ret;
}

/// Send a "statement aborts" message to the error log.
///
/// # Arguments
///
/// * `p` - The statement that is running at the time of failure
/// * `rc` - Error code
/// * `pOp` - Opcode that filed
/// * `aOp` - All opcodes
fn sqlite3VdbeLogAbort(mut p: *mut Vdbe, mut rc: i32, mut pOp: *mut VdbeOp, mut aOp: *mut VdbeOp) {
    let mut zSql: *const i8 = (unsafe { (*p).zSql }) as *const i8; // Original SQL text
    let mut zPrefix: *const i8 = (b"\0".as_ptr() as *mut i8) as *const i8; // Prefix added to SQL text
    let mut pc: i32 = 0 as i32; // Opcode address
    let mut zXtra: __SlateAlign16<[i8; 100]> = __SlateAlign16([0 as i8; 100]); // Buffer space to store zPrefix
    if (unsafe { (*p).pFrame }) != std::ptr::null_mut::<VdbeFrame>() {
        0 as i32;
        if (unsafe { (*unsafe { aOp.offset((0 as i32) as isize) }).p4.z })
            != std::ptr::null_mut::<i8>()
        {
            0 as i32;
            unsafe {
                sqlite3_snprintf(
                    ((100 as u64) as u32) as i32,
                    zXtra.0.as_mut_ptr() as *mut i8,
                    (b"/* %s */ \0".as_ptr() as *mut i8) as *const i8,
                    unsafe {
                        unsafe { (*unsafe { aOp.offset((0 as i32) as isize) }).p4.z }
                            .offset((3 as i32) as isize)
                    },
                )
            };
            zPrefix = (zXtra.0.as_mut_ptr() as *mut i8) as *const i8;
        } else {
            zPrefix = (b"/* unknown trigger */ \0".as_ptr() as *mut i8) as *const i8;
        }
    }
    pc = ((unsafe { pOp.offset_from(aOp as *mut VdbeOp) }) as i64) as i32;
    unsafe {
        sqlite3_log(
            rc,
            (b"statement aborts at %d: %s; [%s%s]\0".as_ptr() as *mut i8) as *const i8,
            pc,
            unsafe { (*p).zErrMsg },
            zPrefix,
            zSql,
        )
    };
}

/// Return the symbolic name for the data type of a pMem
fn vdbeMemTypeName(mut pMem: *mut sqlite3_value) -> *const i8 {
    // SQLITE_INTEGER
    // SQLITE_FLOAT
    // SQLITE_TEXT
    // SQLITE_BLOB
    // SQLITE_NULL
    return unsafe {
        *unsafe {
            unsafe { std::ptr::addr_of_mut!(azTypes.0) as *mut *const i8 }
                .offset(((unsafe { sqlite3_value_type(pMem) }) - (1 as i32)) as isize)
        }
    };
}

static mut azTypes: __SlateAlign16<[*const i8; 5]> = __SlateAlign16([
    (b"INT\0".as_ptr() as *mut i8) as *const i8,
    (b"REAL\0".as_ptr() as *mut i8) as *const i8,
    (b"TEXT\0".as_ptr() as *mut i8) as *const i8,
    (b"BLOB\0".as_ptr() as *mut i8) as *const i8,
    (b"NULL\0".as_ptr() as *mut i8) as *const i8,
]);

/// Execute as much of a VDBE program as we can.
/// This is the core of sqlite3_step().
///
/// # Arguments
///
/// * `p` - The VDBE
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3VdbeExec(mut p: *mut Vdbe) -> i32 {
    let mut __slate_storage_1966: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1966: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_1966) as *mut u32;
    let mut __slate_storage_1965: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1965: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_1965) as *mut u32;
    let mut __slate_storage_1964: std::mem::MaybeUninit<*mut u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1964: *mut *mut u32 =
        std::ptr::addr_of_mut!(__slate_storage_1964) as *mut *mut u32;
    let mut __slate_storage_1963: std::mem::MaybeUninit<u64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1963: *mut u64 = std::ptr::addr_of_mut!(__slate_storage_1963) as *mut u64;
    let mut __slate_storage_1962: std::mem::MaybeUninit<u64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1962: *mut u64 = std::ptr::addr_of_mut!(__slate_storage_1962) as *mut u64;
    let mut __slate_storage_1961: std::mem::MaybeUninit<u64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1961: *mut u64 = std::ptr::addr_of_mut!(__slate_storage_1961) as *mut u64;
    let mut __slate_storage_1960: std::mem::MaybeUninit<u64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1960: *mut u64 = std::ptr::addr_of_mut!(__slate_storage_1960) as *mut u64;
    let mut __slate_storage_1959: std::mem::MaybeUninit<*mut sqlite3> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1959: *mut *mut sqlite3 =
        std::ptr::addr_of_mut!(__slate_storage_1959) as *mut *mut sqlite3;
    let mut __slate_storage_1545: std::mem::MaybeUninit<*mut VdbeOp> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1545: *mut *mut VdbeOp =
        std::ptr::addr_of_mut!(__slate_storage_1545) as *mut *mut VdbeOp;
    // The following code adds nothing to the actual functionality
    // of the program.  It is only here for testing and debugging.
    // On the other hand, it does burn CPU cycles every time through
    // the evaluator loop.  So we can leave it out when NDEBUG is defined.
    let mut __slate_storage_1544: std::mem::MaybeUninit<*mut VdbeOp> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1544: *mut *mut VdbeOp =
        std::ptr::addr_of_mut!(__slate_storage_1544) as *mut *mut VdbeOp;
    let mut __slate_storage_1958: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1958: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_1958) as *mut u32;
    let mut __slate_storage_1957: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1957: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_1957) as *mut u32;
    let mut __slate_storage_1956: std::mem::MaybeUninit<*mut u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1956: *mut *mut u32 =
        std::ptr::addr_of_mut!(__slate_storage_1956) as *mut *mut u32;
    let mut __slate_storage_1955: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1955: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1955) as *mut i32;
    let mut __slate_storage_1954: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1954: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1954) as *mut i32;
    let mut __slate_storage_1953: std::mem::MaybeUninit<*mut VdbeOp> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1953: *mut *mut VdbeOp =
        std::ptr::addr_of_mut!(__slate_storage_1953) as *mut *mut VdbeOp;
    let mut __slate_storage_1952: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1952: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1952) as *mut i32;
    let mut __slate_storage_1951: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1951: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1951) as *mut i32;
    let mut __slate_storage_1057: std::mem::MaybeUninit<*mut i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1057: *mut *mut i8 =
        std::ptr::addr_of_mut!(__slate_storage_1057) as *mut *mut i8;
    let mut __slate_storage_1058: std::mem::MaybeUninit<*mut i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1058: *mut *mut i8 =
        std::ptr::addr_of_mut!(__slate_storage_1058) as *mut *mut i8;
    let mut __slate_storage_1950: std::mem::MaybeUninit<*mut i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1950: *mut *mut i8 =
        std::ptr::addr_of_mut!(__slate_storage_1950) as *mut *mut i8;
    let mut __slate_storage_1949: std::mem::MaybeUninit<bool> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1949: *mut bool = std::ptr::addr_of_mut!(__slate_storage_1949) as *mut bool;
    let mut __slate_storage_1056: std::mem::MaybeUninit<*mut i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1056: *mut *mut i8 =
        std::ptr::addr_of_mut!(__slate_storage_1056) as *mut *mut i8;
    let mut __slate_storage_1055: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1055: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1055) as *mut i32;
    let mut __slate_storage_1945: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1945: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_1945) as *mut u32;
    let mut __slate_storage_1944: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1944: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_1944) as *mut u32;
    let mut __slate_storage_1943: std::mem::MaybeUninit<*mut u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1943: *mut *mut u32 =
        std::ptr::addr_of_mut!(__slate_storage_1943) as *mut *mut u32;
    let mut __slate_storage_1948: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1948: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_1948) as *mut u32;
    let mut __slate_storage_1947: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1947: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_1947) as *mut u32;
    let mut __slate_storage_1946: std::mem::MaybeUninit<*mut u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1946: *mut *mut u32 =
        std::ptr::addr_of_mut!(__slate_storage_1946) as *mut *mut u32;
    let mut __slate_storage_1942: std::mem::MaybeUninit<u64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1942: *mut u64 = std::ptr::addr_of_mut!(__slate_storage_1942) as *mut u64;
    let mut __slate_storage_1941: std::mem::MaybeUninit<u64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1941: *mut u64 = std::ptr::addr_of_mut!(__slate_storage_1941) as *mut u64;
    let mut __slate_storage_1054: std::mem::MaybeUninit<u64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1054: *mut u64 = std::ptr::addr_of_mut!(__slate_storage_1054) as *mut u64;
    let mut __slate_storage_1940: std::mem::MaybeUninit<i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1940: *mut i8 = std::ptr::addr_of_mut!(__slate_storage_1940) as *mut i8;
    let mut __slate_storage_1939: std::mem::MaybeUninit<i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1939: *mut i8 = std::ptr::addr_of_mut!(__slate_storage_1939) as *mut i8;
    let mut __slate_storage_1938: std::mem::MaybeUninit<*mut i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1938: *mut *mut i8 =
        std::ptr::addr_of_mut!(__slate_storage_1938) as *mut *mut i8;
    let mut __slate_storage_1937: std::mem::MaybeUninit<u64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1937: *mut u64 = std::ptr::addr_of_mut!(__slate_storage_1937) as *mut u64;
    let mut __slate_storage_1936: std::mem::MaybeUninit<u64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1936: *mut u64 = std::ptr::addr_of_mut!(__slate_storage_1936) as *mut u64;
    let mut __slate_storage_1053: std::mem::MaybeUninit<u64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1053: *mut u64 = std::ptr::addr_of_mut!(__slate_storage_1053) as *mut u64;
    let mut __slate_storage_1932: std::mem::MaybeUninit<u16> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1932: *mut u16 = std::ptr::addr_of_mut!(__slate_storage_1932) as *mut u16;
    let mut __slate_storage_1931: std::mem::MaybeUninit<u16> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1931: *mut u16 = std::ptr::addr_of_mut!(__slate_storage_1931) as *mut u16;
    let mut __slate_storage_1930: std::mem::MaybeUninit<*mut sqlite3_value> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1930: *mut *mut sqlite3_value =
        std::ptr::addr_of_mut!(__slate_storage_1930) as *mut *mut sqlite3_value;
    let mut __slate_storage_1935: std::mem::MaybeUninit<u16> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1935: *mut u16 = std::ptr::addr_of_mut!(__slate_storage_1935) as *mut u16;
    let mut __slate_storage_1934: std::mem::MaybeUninit<u16> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1934: *mut u16 = std::ptr::addr_of_mut!(__slate_storage_1934) as *mut u16;
    let mut __slate_storage_1933: std::mem::MaybeUninit<*mut sqlite3_value> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1933: *mut *mut sqlite3_value =
        std::ptr::addr_of_mut!(__slate_storage_1933) as *mut *mut sqlite3_value;
    let mut __slate_storage_1929: std::mem::MaybeUninit<u16> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1929: *mut u16 = std::ptr::addr_of_mut!(__slate_storage_1929) as *mut u16;
    let mut __slate_storage_1928: std::mem::MaybeUninit<u16> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1928: *mut u16 = std::ptr::addr_of_mut!(__slate_storage_1928) as *mut u16;
    let mut __slate_storage_1927: std::mem::MaybeUninit<*mut sqlite3_value> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1927: *mut *mut sqlite3_value =
        std::ptr::addr_of_mut!(__slate_storage_1927) as *mut *mut sqlite3_value;
    let mut __slate_storage_1926: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1926: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1926) as *mut i32;
    let mut __slate_storage_1925: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1925: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1925) as *mut i32;
    let mut __slate_storage_1052: std::mem::MaybeUninit<*mut sqlite3_context> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1052: *mut *mut sqlite3_context =
        std::ptr::addr_of_mut!(__slate_storage_1052) as *mut *mut sqlite3_context;
    let mut __slate_storage_1051: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1051: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1051) as *mut i32;
    let mut __slate_storage_1050: std::mem::MaybeUninit<*mut Btree> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1050: *mut *mut Btree =
        std::ptr::addr_of_mut!(__slate_storage_1050) as *mut *mut Btree;
    let mut __slate_storage_1049: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1049: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_1049) as *mut u32;
    let mut __slate_storage_1924: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1924: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_1924) as *mut i64;
    let mut __slate_storage_1923: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1923: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_1923) as *mut i64;
    let mut __slate_storage_1922: std::mem::MaybeUninit<*mut Vdbe> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1922: *mut *mut Vdbe =
        std::ptr::addr_of_mut!(__slate_storage_1922) as *mut *mut Vdbe;
    let mut __slate_storage_1919: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1919: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1919) as *mut i32;
    let mut __slate_storage_1918: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1918: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1918) as *mut i32;
    let mut __slate_storage_1921: std::mem::MaybeUninit<*mut sqlite3_value> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1921: *mut *mut sqlite3_value =
        std::ptr::addr_of_mut!(__slate_storage_1921) as *mut *mut sqlite3_value;
    let mut __slate_storage_1920: std::mem::MaybeUninit<*mut sqlite3_value> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1920: *mut *mut sqlite3_value =
        std::ptr::addr_of_mut!(__slate_storage_1920) as *mut *mut sqlite3_value;
    let mut __slate_storage_1048: std::mem::MaybeUninit<u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1048: *mut u8 = std::ptr::addr_of_mut!(__slate_storage_1048) as *mut u8;
    let mut __slate_storage_1047: std::mem::MaybeUninit<*mut sqlite3_value> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1047: *mut *mut sqlite3_value =
        std::ptr::addr_of_mut!(__slate_storage_1047) as *mut *mut sqlite3_value;
    let mut __slate_storage_1046: std::mem::MaybeUninit<*mut *mut sqlite3_value> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1046: *mut *mut *mut sqlite3_value =
        std::ptr::addr_of_mut!(__slate_storage_1046) as *mut *mut *mut sqlite3_value;
    let mut __slate_storage_1045: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1045: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_1045) as *mut i64;
    let mut __slate_storage_1044: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1044: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1044) as *mut i32;
    let mut __slate_storage_1043: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1043: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1043) as *mut i32;
    let mut __slate_storage_1042: std::mem::MaybeUninit<*const sqlite3_module> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1042: *mut *const sqlite3_module =
        std::ptr::addr_of_mut!(__slate_storage_1042) as *mut *const sqlite3_module;
    let mut __slate_storage_1041: std::mem::MaybeUninit<*mut sqlite3_vtab> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1041: *mut *mut sqlite3_vtab =
        std::ptr::addr_of_mut!(__slate_storage_1041) as *mut *mut sqlite3_vtab;
    let mut __slate_storage_1917: std::mem::MaybeUninit<u64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1917: *mut u64 = std::ptr::addr_of_mut!(__slate_storage_1917) as *mut u64;
    let mut __slate_storage_1916: std::mem::MaybeUninit<u64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1916: *mut u64 = std::ptr::addr_of_mut!(__slate_storage_1916) as *mut u64;
    let mut __slate_storage_1915: std::mem::MaybeUninit<*mut sqlite3> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1915: *mut *mut sqlite3 =
        std::ptr::addr_of_mut!(__slate_storage_1915) as *mut *mut sqlite3;
    let mut __slate_storage_1914: std::mem::MaybeUninit<u64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1914: *mut u64 = std::ptr::addr_of_mut!(__slate_storage_1914) as *mut u64;
    let mut __slate_storage_1913: std::mem::MaybeUninit<u64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1913: *mut u64 = std::ptr::addr_of_mut!(__slate_storage_1913) as *mut u64;
    let mut __slate_storage_1912: std::mem::MaybeUninit<*mut sqlite3> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1912: *mut *mut sqlite3 =
        std::ptr::addr_of_mut!(__slate_storage_1912) as *mut *mut sqlite3;
    let mut __slate_storage_1040: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1040: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1040) as *mut i32;
    let mut __slate_storage_1039: std::mem::MaybeUninit<*mut sqlite3_value> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1039: *mut *mut sqlite3_value =
        std::ptr::addr_of_mut!(__slate_storage_1039) as *mut *mut sqlite3_value;
    let mut __slate_storage_1038: std::mem::MaybeUninit<*mut sqlite3_vtab> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1038: *mut *mut sqlite3_vtab =
        std::ptr::addr_of_mut!(__slate_storage_1038) as *mut *mut sqlite3_vtab;
    let mut __slate_storage_1037: std::mem::MaybeUninit<*mut VdbeCursor> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1037: *mut *mut VdbeCursor =
        std::ptr::addr_of_mut!(__slate_storage_1037) as *mut *mut VdbeCursor;
    let mut __slate_storage_1036: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1036: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1036) as *mut i32;
    let mut __slate_storage_1035: std::mem::MaybeUninit<*const sqlite3_module> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1035: *mut *const sqlite3_module =
        std::ptr::addr_of_mut!(__slate_storage_1035) as *mut *const sqlite3_module;
    let mut __slate_storage_1034: std::mem::MaybeUninit<*mut sqlite3_vtab> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1034: *mut *mut sqlite3_vtab =
        std::ptr::addr_of_mut!(__slate_storage_1034) as *mut *mut sqlite3_vtab;
    let mut __slate_storage_1033: std::mem::MaybeUninit<*mut VdbeCursor> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1033: *mut *mut VdbeCursor =
        std::ptr::addr_of_mut!(__slate_storage_1033) as *mut *mut VdbeCursor;
    let mut __slate_storage_1032: std::mem::MaybeUninit<FuncDef> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1032: *mut FuncDef =
        std::ptr::addr_of_mut!(__slate_storage_1032) as *mut FuncDef;
    let mut __slate_storage_1031: std::mem::MaybeUninit<sqlite3_context> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1031: *mut sqlite3_context =
        std::ptr::addr_of_mut!(__slate_storage_1031) as *mut sqlite3_context;
    let mut __slate_storage_1030: std::mem::MaybeUninit<*mut sqlite3_value> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1030: *mut *mut sqlite3_value =
        std::ptr::addr_of_mut!(__slate_storage_1030) as *mut *mut sqlite3_value;
    let mut __slate_storage_1029: std::mem::MaybeUninit<*const sqlite3_module> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1029: *mut *const sqlite3_module =
        std::ptr::addr_of_mut!(__slate_storage_1029) as *mut *const sqlite3_module;
    let mut __slate_storage_1028: std::mem::MaybeUninit<*mut sqlite3_vtab> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1028: *mut *mut sqlite3_vtab =
        std::ptr::addr_of_mut!(__slate_storage_1028) as *mut *mut sqlite3_vtab;
    let mut __slate_storage_1911: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1911: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1911) as *mut i32;
    let mut __slate_storage_1910: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1910: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1910) as *mut i32;
    let mut __slate_storage_1027: std::mem::MaybeUninit<*mut *mut sqlite3_value> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1027: *mut *mut *mut sqlite3_value =
        std::ptr::addr_of_mut!(__slate_storage_1027) as *mut *mut *mut sqlite3_value;
    let mut __slate_storage_1026: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1026: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1026) as *mut i32;
    let mut __slate_storage_1025: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1025: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1025) as *mut i32;
    let mut __slate_storage_1024: std::mem::MaybeUninit<*mut VdbeCursor> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1024: *mut *mut VdbeCursor =
        std::ptr::addr_of_mut!(__slate_storage_1024) as *mut *mut VdbeCursor;
    let mut __slate_storage_1023: std::mem::MaybeUninit<*mut sqlite3_vtab> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1023: *mut *mut sqlite3_vtab =
        std::ptr::addr_of_mut!(__slate_storage_1023) as *mut *mut sqlite3_vtab;
    let mut __slate_storage_1022: std::mem::MaybeUninit<*mut sqlite3_vtab_cursor> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1022: *mut *mut sqlite3_vtab_cursor =
        std::ptr::addr_of_mut!(__slate_storage_1022) as *mut *mut sqlite3_vtab_cursor;
    let mut __slate_storage_1021: std::mem::MaybeUninit<*mut sqlite3_value> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1021: *mut *mut sqlite3_value =
        std::ptr::addr_of_mut!(__slate_storage_1021) as *mut *mut sqlite3_value;
    let mut __slate_storage_1020: std::mem::MaybeUninit<*mut sqlite3_value> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1020: *mut *mut sqlite3_value =
        std::ptr::addr_of_mut!(__slate_storage_1020) as *mut *mut sqlite3_value;
    let mut __slate_storage_1019: std::mem::MaybeUninit<*const sqlite3_module> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1019: *mut *const sqlite3_module =
        std::ptr::addr_of_mut!(__slate_storage_1019) as *mut *const sqlite3_module;
    let mut __slate_storage_1018: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1018: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1018) as *mut i32;
    let mut __slate_storage_1017: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1017: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1017) as *mut i32; // New ValueList object to put in reg[P2]
    let mut __slate_storage_1016: std::mem::MaybeUninit<*mut ValueList> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1016: *mut *mut ValueList =
        std::ptr::addr_of_mut!(__slate_storage_1016) as *mut *mut ValueList; // The cursor containing the RHS values
    let mut __slate_storage_1015: std::mem::MaybeUninit<*mut VdbeCursor> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1015: *mut *mut VdbeCursor =
        std::ptr::addr_of_mut!(__slate_storage_1015) as *mut *mut VdbeCursor;
    let mut __slate_storage_1014: std::mem::MaybeUninit<*mut i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1014: *mut *mut i8 =
        std::ptr::addr_of_mut!(__slate_storage_1014) as *mut *mut i8;
    let mut __slate_storage_1013: std::mem::MaybeUninit<*const sqlite3_module> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1013: *mut *const sqlite3_module =
        std::ptr::addr_of_mut!(__slate_storage_1013) as *mut *const sqlite3_module;
    let mut __slate_storage_1012: std::mem::MaybeUninit<*mut sqlite3_vtab> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1012: *mut *mut sqlite3_vtab =
        std::ptr::addr_of_mut!(__slate_storage_1012) as *mut *mut sqlite3_vtab;
    let mut __slate_storage_1011: std::mem::MaybeUninit<*mut Table> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1011: *mut *mut Table =
        std::ptr::addr_of_mut!(__slate_storage_1011) as *mut *mut Table;
    let mut __slate_storage_1909: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1909: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1909) as *mut i32;
    let mut __slate_storage_1908: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1908: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1908) as *mut i32;
    let mut __slate_storage_1907: std::mem::MaybeUninit<*mut sqlite3_vtab> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1907: *mut *mut sqlite3_vtab =
        std::ptr::addr_of_mut!(__slate_storage_1907) as *mut *mut sqlite3_vtab;
    let mut __slate_storage_1010: std::mem::MaybeUninit<*const sqlite3_module> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1010: *mut *const sqlite3_module =
        std::ptr::addr_of_mut!(__slate_storage_1010) as *mut *const sqlite3_module;
    let mut __slate_storage_1009: std::mem::MaybeUninit<*mut sqlite3_vtab> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1009: *mut *mut sqlite3_vtab =
        std::ptr::addr_of_mut!(__slate_storage_1009) as *mut *mut sqlite3_vtab;
    let mut __slate_storage_1008: std::mem::MaybeUninit<*mut sqlite3_vtab_cursor> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1008: *mut *mut sqlite3_vtab_cursor =
        std::ptr::addr_of_mut!(__slate_storage_1008) as *mut *mut sqlite3_vtab_cursor;
    let mut __slate_storage_1007: std::mem::MaybeUninit<*mut VdbeCursor> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1007: *mut *mut VdbeCursor =
        std::ptr::addr_of_mut!(__slate_storage_1007) as *mut *mut VdbeCursor;
    let mut __slate_storage_1906: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1906: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1906) as *mut i32;
    let mut __slate_storage_1905: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1905: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1905) as *mut i32;
    let mut __slate_storage_1904: std::mem::MaybeUninit<*mut sqlite3> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1904: *mut *mut sqlite3 =
        std::ptr::addr_of_mut!(__slate_storage_1904) as *mut *mut sqlite3;
    let mut __slate_storage_1903: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1903: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1903) as *mut i32;
    let mut __slate_storage_1902: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1902: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1902) as *mut i32;
    let mut __slate_storage_1901: std::mem::MaybeUninit<*mut sqlite3> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1901: *mut *mut sqlite3 =
        std::ptr::addr_of_mut!(__slate_storage_1901) as *mut *mut sqlite3; // Name of the virtual table
    let mut __slate_storage_1006: std::mem::MaybeUninit<*const i8> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1006: *mut *const i8 =
        std::ptr::addr_of_mut!(__slate_storage_1006) as *mut *const i8; // For storing the record being decoded
    let mut __slate_storage_1005: std::mem::MaybeUninit<sqlite3_value> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1005: *mut sqlite3_value =
        std::ptr::addr_of_mut!(__slate_storage_1005) as *mut sqlite3_value;
    let mut __slate_storage_1004: std::mem::MaybeUninit<*mut VTable> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1004: *mut *mut VTable =
        std::ptr::addr_of_mut!(__slate_storage_1004) as *mut *mut VTable;
    let mut __slate_storage_1003: std::mem::MaybeUninit<*const i8> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1003: *mut *const i8 =
        std::ptr::addr_of_mut!(__slate_storage_1003) as *mut *const i8;
    let mut __slate_storage_1002: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1002: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1002) as *mut i32;
    let mut __slate_storage_1001: std::mem::MaybeUninit<u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1001: *mut u8 = std::ptr::addr_of_mut!(__slate_storage_1001) as *mut u8;
    let mut __slate_storage_1000: std::mem::MaybeUninit<*mut VdbeCursor> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1000: *mut *mut VdbeCursor =
        std::ptr::addr_of_mut!(__slate_storage_1000) as *mut *mut VdbeCursor;
    let mut __slate_storage_999: std::mem::MaybeUninit<*mut VdbeCursor> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_999: *mut *mut VdbeCursor =
        std::ptr::addr_of_mut!(__slate_storage_999) as *mut *mut VdbeCursor;
    let mut __slate_storage_998: std::mem::MaybeUninit<*mut Btree> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_998: *mut *mut Btree =
        std::ptr::addr_of_mut!(__slate_storage_998) as *mut *mut Btree;
    let mut __slate_storage_1900: std::mem::MaybeUninit<bool> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1900: *mut bool = std::ptr::addr_of_mut!(__slate_storage_1900) as *mut bool;
    // Do not allow a transition to journal_mode=WAL for a database
    // in temporary storage or if the VFS does not support shared memory
    let mut __slate_storage_1899: std::mem::MaybeUninit<bool> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1899: *mut bool = std::ptr::addr_of_mut!(__slate_storage_1899) as *mut bool; // Name of database file for pPager
    let mut __slate_storage_997: std::mem::MaybeUninit<*const i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_997: *mut *const i8 =
        std::ptr::addr_of_mut!(__slate_storage_997) as *mut *const i8; // The old journal mode
    let mut __slate_storage_996: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_996: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_996) as *mut i32; // New journal mode
    let mut __slate_storage_995: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_995: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_995) as *mut i32; // Pager associated with pBt
    let mut __slate_storage_994: std::mem::MaybeUninit<*mut Pager> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_994: *mut *mut Pager =
        std::ptr::addr_of_mut!(__slate_storage_994) as *mut *mut Pager; // Btree to change journal mode of
    let mut __slate_storage_993: std::mem::MaybeUninit<*mut Btree> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_993: *mut *mut Btree =
        std::ptr::addr_of_mut!(__slate_storage_993) as *mut *mut Btree;
    let mut __slate_storage_1898: std::mem::MaybeUninit<*mut sqlite3_value> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1898: *mut *mut sqlite3_value =
        std::ptr::addr_of_mut!(__slate_storage_1898) as *mut *mut sqlite3_value;
    let mut __slate_storage_1897: std::mem::MaybeUninit<*mut sqlite3_value> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1897: *mut *mut sqlite3_value =
        std::ptr::addr_of_mut!(__slate_storage_1897) as *mut *mut sqlite3_value;
    let mut __slate_storage_1896: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1896: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1896) as *mut i32;
    let mut __slate_storage_1895: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1895: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1895) as *mut i32;
    let mut __slate_storage_1894: std::mem::MaybeUninit<*mut sqlite3_value> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1894: *mut *mut sqlite3_value =
        std::ptr::addr_of_mut!(__slate_storage_1894) as *mut *mut sqlite3_value;
    let mut __slate_storage_1893: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1893: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1893) as *mut i32; // Write results here
    let mut __slate_storage_992: std::mem::MaybeUninit<*mut sqlite3_value> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_992: *mut *mut sqlite3_value =
        std::ptr::addr_of_mut!(__slate_storage_992) as *mut *mut sqlite3_value; // Results
    let mut __slate_storage_991: std::mem::MaybeUninit<[i32; 3]> = std::mem::MaybeUninit::uninit();
    let __slate_slot_991: *mut [i32; 3] =
        std::ptr::addr_of_mut!(__slate_storage_991) as *mut [i32; 3]; // Loop counter
    let mut __slate_storage_990: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_990: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_990) as *mut i32;
    let mut __slate_storage_989: std::mem::MaybeUninit<*mut sqlite3_value> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_989: *mut *mut sqlite3_value =
        std::ptr::addr_of_mut!(__slate_storage_989) as *mut *mut sqlite3_value;
    let mut __slate_storage_1892: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1892: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1892) as *mut i32;
    let mut __slate_storage_1891: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1891: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1891) as *mut i32;
    let mut __slate_storage_1890: std::mem::MaybeUninit<*mut sqlite3_value> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1890: *mut *mut sqlite3_value =
        std::ptr::addr_of_mut!(__slate_storage_1890) as *mut *mut sqlite3_value;
    let mut __slate_storage_1889: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1889: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1889) as *mut i32;
    let mut __slate_storage_1888: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1888: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1888) as *mut i32;
    let mut __slate_storage_988: std::mem::MaybeUninit<*mut sqlite3_value> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_988: *mut *mut sqlite3_value =
        std::ptr::addr_of_mut!(__slate_storage_988) as *mut *mut sqlite3_value;
    let mut __slate_storage_987: std::mem::MaybeUninit<*mut sqlite3_context> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_987: *mut *mut sqlite3_context =
        std::ptr::addr_of_mut!(__slate_storage_987) as *mut *mut sqlite3_context;
    let mut __slate_storage_986: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_986: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_986) as *mut i32;
    let mut __slate_storage_985: std::mem::MaybeUninit<u64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_985: *mut u64 = std::ptr::addr_of_mut!(__slate_storage_985) as *mut u64;
    let mut __slate_storage_984: std::mem::MaybeUninit<*mut sqlite3_context> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_984: *mut *mut sqlite3_context =
        std::ptr::addr_of_mut!(__slate_storage_984) as *mut *mut sqlite3_context;
    let mut __slate_storage_983: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_983: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_983) as *mut i32;
    let mut __slate_storage_1887: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1887: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_1887) as *mut i64;
    let mut __slate_storage_1886: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1886: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_1886) as *mut i64;
    let mut __slate_storage_1885: std::mem::MaybeUninit<*mut sqlite3_value> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1885: *mut *mut sqlite3_value =
        std::ptr::addr_of_mut!(__slate_storage_1885) as *mut *mut sqlite3_value;
    let mut __slate_storage_1884: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1884: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_1884) as *mut i64;
    let mut __slate_storage_1883: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1883: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_1883) as *mut i64;
    let mut __slate_storage_1882: std::mem::MaybeUninit<*mut sqlite3_value> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1882: *mut *mut sqlite3_value =
        std::ptr::addr_of_mut!(__slate_storage_1882) as *mut *mut sqlite3_value;
    let mut __slate_storage_1881: std::mem::MaybeUninit<bool> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1881: *mut bool = std::ptr::addr_of_mut!(__slate_storage_1881) as *mut bool;
    let mut __slate_storage_982: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_982: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_982) as *mut i64;
    let mut __slate_storage_1880: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1880: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_1880) as *mut i64;
    let mut __slate_storage_1879: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1879: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_1879) as *mut i64;
    let mut __slate_storage_1878: std::mem::MaybeUninit<*mut sqlite3_value> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1878: *mut *mut sqlite3_value =
        std::ptr::addr_of_mut!(__slate_storage_1878) as *mut *mut sqlite3_value;
    let mut __slate_storage_981: std::mem::MaybeUninit<*mut VdbeFrame> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_981: *mut *mut VdbeFrame =
        std::ptr::addr_of_mut!(__slate_storage_981) as *mut *mut VdbeFrame;
    let mut __slate_storage_1871: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1871: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_1871) as *mut i64;
    let mut __slate_storage_1870: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1870: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_1870) as *mut i64;
    let mut __slate_storage_1869: std::mem::MaybeUninit<*mut sqlite3> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1869: *mut *mut sqlite3 =
        std::ptr::addr_of_mut!(__slate_storage_1869) as *mut *mut sqlite3;
    let mut __slate_storage_1874: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1874: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_1874) as *mut i64;
    let mut __slate_storage_1873: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1873: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_1873) as *mut i64;
    let mut __slate_storage_1872: std::mem::MaybeUninit<*mut sqlite3> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1872: *mut *mut sqlite3 =
        std::ptr::addr_of_mut!(__slate_storage_1872) as *mut *mut sqlite3;
    let mut __slate_storage_1877: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1877: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_1877) as *mut i64;
    let mut __slate_storage_1876: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1876: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_1876) as *mut i64;
    let mut __slate_storage_1875: std::mem::MaybeUninit<*mut Vdbe> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1875: *mut *mut Vdbe =
        std::ptr::addr_of_mut!(__slate_storage_1875) as *mut *mut Vdbe;
    let mut __slate_storage_980: std::mem::MaybeUninit<*mut sqlite3_value> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_980: *mut *mut sqlite3_value =
        std::ptr::addr_of_mut!(__slate_storage_980) as *mut *mut sqlite3_value;
    let mut __slate_storage_979: std::mem::MaybeUninit<*mut VdbeFrame> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_979: *mut *mut VdbeFrame =
        std::ptr::addr_of_mut!(__slate_storage_979) as *mut *mut VdbeFrame;
    let mut __slate_storage_1868: std::mem::MaybeUninit<*mut VdbeOp> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1868: *mut *mut VdbeOp =
        std::ptr::addr_of_mut!(__slate_storage_1868) as *mut *mut VdbeOp;
    let mut __slate_storage_1867: std::mem::MaybeUninit<*mut sqlite3_value> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1867: *mut *mut sqlite3_value =
        std::ptr::addr_of_mut!(__slate_storage_1867) as *mut *mut sqlite3_value;
    let mut __slate_storage_1866: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1866: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1866) as *mut i32;
    let mut __slate_storage_1865: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1865: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1865) as *mut i32;
    let mut __slate_storage_1864: std::mem::MaybeUninit<*mut Vdbe> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1864: *mut *mut Vdbe =
        std::ptr::addr_of_mut!(__slate_storage_1864) as *mut *mut Vdbe;
    let mut __slate_storage_1863: std::mem::MaybeUninit<*mut sqlite3_value> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1863: *mut *mut sqlite3_value =
        std::ptr::addr_of_mut!(__slate_storage_1863) as *mut *mut sqlite3_value;
    let mut __slate_storage_1862: std::mem::MaybeUninit<*mut sqlite3_value> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1862: *mut *mut sqlite3_value =
        std::ptr::addr_of_mut!(__slate_storage_1862) as *mut *mut sqlite3_value;
    let mut __slate_storage_1861: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1861: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1861) as *mut i32;
    let mut __slate_storage_1860: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1860: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1860) as *mut i32; // Token identifying trigger
    let mut __slate_storage_978: std::mem::MaybeUninit<*mut ()> = std::mem::MaybeUninit::uninit();
    let __slate_slot_978: *mut *mut () =
        std::ptr::addr_of_mut!(__slate_storage_978) as *mut *mut (); // Sub-program to execute
    let mut __slate_storage_977: std::mem::MaybeUninit<*mut SubProgram> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_977: *mut *mut SubProgram =
        std::ptr::addr_of_mut!(__slate_storage_977) as *mut *mut SubProgram; // New vdbe frame to execute in
    let mut __slate_storage_976: std::mem::MaybeUninit<*mut VdbeFrame> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_976: *mut *mut VdbeFrame =
        std::ptr::addr_of_mut!(__slate_storage_976) as *mut *mut VdbeFrame; // Last memory cell in new array
    let mut __slate_storage_975: std::mem::MaybeUninit<*mut sqlite3_value> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_975: *mut *mut sqlite3_value =
        std::ptr::addr_of_mut!(__slate_storage_975) as *mut *mut sqlite3_value; // Used to iterate through memory cells
    let mut __slate_storage_974: std::mem::MaybeUninit<*mut sqlite3_value> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_974: *mut *mut sqlite3_value =
        std::ptr::addr_of_mut!(__slate_storage_974) as *mut *mut sqlite3_value; // Register to allocate runtime space
    let mut __slate_storage_973: std::mem::MaybeUninit<*mut sqlite3_value> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_973: *mut *mut sqlite3_value =
        std::ptr::addr_of_mut!(__slate_storage_973) as *mut *mut sqlite3_value; // Bytes of runtime space required for sub-program
    let mut __slate_storage_972: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_972: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_972) as *mut i64; // Number of memory registers for sub-program
    let mut __slate_storage_971: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_971: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_971) as *mut i32;
    let mut __slate_storage_970: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_970: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_970) as *mut i32;
    let mut __slate_storage_969: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_969: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_969) as *mut i32;
    let mut __slate_storage_1859: std::mem::MaybeUninit<bool> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1859: *mut bool = std::ptr::addr_of_mut!(__slate_storage_1859) as *mut bool;
    let mut __slate_storage_968: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_968: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_968) as *mut i64;
    let mut __slate_storage_967: std::mem::MaybeUninit<UnpackedRecord> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_967: *mut UnpackedRecord =
        std::ptr::addr_of_mut!(__slate_storage_967) as *mut UnpackedRecord;
    let mut __slate_storage_966: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_966: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_966) as *mut i32;
    let mut __slate_storage_965: std::mem::MaybeUninit<*mut VdbeCursor> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_965: *mut *mut VdbeCursor =
        std::ptr::addr_of_mut!(__slate_storage_965) as *mut *mut VdbeCursor;
    let mut __slate_storage_1858: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1858: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_1858) as *mut i64;
    let mut __slate_storage_1857: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1857: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_1857) as *mut i64;
    let mut __slate_storage_1856: std::mem::MaybeUninit<*mut sqlite3_value> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1856: *mut *mut sqlite3_value =
        std::ptr::addr_of_mut!(__slate_storage_1856) as *mut *mut sqlite3_value; // Register keeping track of errors remaining
    let mut __slate_storage_964: std::mem::MaybeUninit<*mut sqlite3_value> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_964: *mut *mut sqlite3_value =
        std::ptr::addr_of_mut!(__slate_storage_964) as *mut *mut sqlite3_value; // Text of the error report
    let mut __slate_storage_963: std::mem::MaybeUninit<*mut i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_963: *mut *mut i8 =
        std::ptr::addr_of_mut!(__slate_storage_963) as *mut *mut i8; // Number of errors reported
    let mut __slate_storage_962: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_962: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_962) as *mut i32; // Array of rootpage numbers for tables to be checked
    let mut __slate_storage_961: std::mem::MaybeUninit<*mut u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_961: *mut *mut u32 =
        std::ptr::addr_of_mut!(__slate_storage_961) as *mut *mut u32; // Number of tables to check.  (Number of root pages.)
    let mut __slate_storage_960: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_960: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_960) as *mut i32;
    let mut __slate_storage_1855: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1855: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_1855) as *mut u32;
    let mut __slate_storage_1854: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1854: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_1854) as *mut u32;
    let mut __slate_storage_1853: std::mem::MaybeUninit<*mut sqlite3> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1853: *mut *mut sqlite3 =
        std::ptr::addr_of_mut!(__slate_storage_1853) as *mut *mut sqlite3;
    let mut __slate_storage_1852: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1852: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_1852) as *mut u32;
    let mut __slate_storage_1851: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1851: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_1851) as *mut u32;
    let mut __slate_storage_1850: std::mem::MaybeUninit<*mut sqlite3> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1850: *mut *mut sqlite3 =
        std::ptr::addr_of_mut!(__slate_storage_1850) as *mut *mut sqlite3;
    let mut __slate_storage_959: std::mem::MaybeUninit<__SlateRecord209> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_959: *mut __SlateRecord209 =
        std::ptr::addr_of_mut!(__slate_storage_959) as *mut __SlateRecord209;
    let mut __slate_storage_958: std::mem::MaybeUninit<*mut i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_958: *mut *mut i8 =
        std::ptr::addr_of_mut!(__slate_storage_958) as *mut *mut i8;
    let mut __slate_storage_957: std::mem::MaybeUninit<*const i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_957: *mut *const i8 =
        std::ptr::addr_of_mut!(__slate_storage_957) as *mut *const i8;
    let mut __slate_storage_956: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_956: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_956) as *mut i32;
    let mut __slate_storage_1849: std::mem::MaybeUninit<u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1849: *mut u8 = std::ptr::addr_of_mut!(__slate_storage_1849) as *mut u8;
    let mut __slate_storage_1848: std::mem::MaybeUninit<u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1848: *mut u8 = std::ptr::addr_of_mut!(__slate_storage_1848) as *mut u8;
    let mut __slate_storage_1847: std::mem::MaybeUninit<*mut sqlite3> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1847: *mut *mut sqlite3 =
        std::ptr::addr_of_mut!(__slate_storage_1847) as *mut *mut sqlite3;
    let mut __slate_storage_1846: std::mem::MaybeUninit<u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1846: *mut u8 = std::ptr::addr_of_mut!(__slate_storage_1846) as *mut u8;
    let mut __slate_storage_1845: std::mem::MaybeUninit<u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1845: *mut u8 = std::ptr::addr_of_mut!(__slate_storage_1845) as *mut u8;
    let mut __slate_storage_1844: std::mem::MaybeUninit<*mut sqlite3> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1844: *mut *mut sqlite3 =
        std::ptr::addr_of_mut!(__slate_storage_1844) as *mut *mut sqlite3;
    let mut __slate_storage_955: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_955: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_955) as *mut i32;
    let mut __slate_storage_954: std::mem::MaybeUninit<u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_954: *mut u8 = std::ptr::addr_of_mut!(__slate_storage_954) as *mut u8;
    let mut __slate_storage_953: std::mem::MaybeUninit<
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
    let __slate_slot_953: *mut Option<
        unsafe extern "C-unwind" fn(
            *mut (),
            i32,
            *const i8,
            *const i8,
            *const i8,
            *const i8,
        ) -> i32,
    > = std::ptr::addr_of_mut!(__slate_storage_953)
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
    let mut __slate_storage_952: std::mem::MaybeUninit<*mut i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_952: *mut *mut i8 =
        std::ptr::addr_of_mut!(__slate_storage_952) as *mut *mut i8;
    let mut __slate_storage_951: std::mem::MaybeUninit<*mut Db> = std::mem::MaybeUninit::uninit();
    let __slate_slot_951: *mut *mut Db =
        std::ptr::addr_of_mut!(__slate_storage_951) as *mut *mut Db;
    let mut __slate_storage_950: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_950: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_950) as *mut u32;
    let mut __slate_storage_949: std::mem::MaybeUninit<*mut VdbeCursor> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_949: *mut *mut VdbeCursor =
        std::ptr::addr_of_mut!(__slate_storage_949) as *mut *mut VdbeCursor;
    let mut __slate_storage_1843: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1843: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_1843) as *mut i64;
    let mut __slate_storage_1842: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1842: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_1842) as *mut i64;
    let mut __slate_storage_1841: std::mem::MaybeUninit<*mut sqlite3_value> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1841: *mut *mut sqlite3_value =
        std::ptr::addr_of_mut!(__slate_storage_1841) as *mut *mut sqlite3_value;
    let mut __slate_storage_1840: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1840: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_1840) as *mut i64;
    let mut __slate_storage_1839: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1839: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_1839) as *mut i64;
    let mut __slate_storage_1838: std::mem::MaybeUninit<*mut Vdbe> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1838: *mut *mut Vdbe =
        std::ptr::addr_of_mut!(__slate_storage_1838) as *mut *mut Vdbe;
    let mut __slate_storage_948: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_948: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_948) as *mut i64;
    let mut __slate_storage_947: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_947: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_947) as *mut i32;
    let mut __slate_storage_946: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_946: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_946) as *mut i32;
    let mut __slate_storage_1837: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1837: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1837) as *mut i32;
    let mut __slate_storage_1836: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1836: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1836) as *mut i32;
    let mut __slate_storage_945: std::mem::MaybeUninit<sqlite3_value> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_945: *mut sqlite3_value =
        std::ptr::addr_of_mut!(__slate_storage_945) as *mut sqlite3_value;
    let mut __slate_storage_944: std::mem::MaybeUninit<*mut BtCursor> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_944: *mut *mut BtCursor =
        std::ptr::addr_of_mut!(__slate_storage_944) as *mut *mut BtCursor;
    // Inlined version of sqlite3VdbeIdxKeyCompare()
    let mut __slate_storage_943: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_943: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_943) as *mut i64;
    let mut __slate_storage_942: std::mem::MaybeUninit<UnpackedRecord> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_942: *mut UnpackedRecord =
        std::ptr::addr_of_mut!(__slate_storage_942) as *mut UnpackedRecord;
    let mut __slate_storage_941: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_941: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_941) as *mut i32;
    let mut __slate_storage_940: std::mem::MaybeUninit<*mut VdbeCursor> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_940: *mut *mut VdbeCursor =
        std::ptr::addr_of_mut!(__slate_storage_940) as *mut *mut VdbeCursor; // The P1 index cursor
    let mut __slate_storage_939: std::mem::MaybeUninit<*mut VdbeCursor> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_939: *mut *mut VdbeCursor =
        std::ptr::addr_of_mut!(__slate_storage_939) as *mut *mut VdbeCursor; // Rowid that P1 current points to
    let mut __slate_storage_938: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_938: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_938) as *mut i64; // The P2 table cursor (OP_DeferredSeek only)
    let mut __slate_storage_937: std::mem::MaybeUninit<*mut VdbeCursor> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_937: *mut *mut VdbeCursor =
        std::ptr::addr_of_mut!(__slate_storage_937) as *mut *mut VdbeCursor; // The P1 index cursor
    let mut __slate_storage_936: std::mem::MaybeUninit<*mut VdbeCursor> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_936: *mut *mut VdbeCursor =
        std::ptr::addr_of_mut!(__slate_storage_936) as *mut *mut VdbeCursor;
    let mut __slate_storage_1835: std::mem::MaybeUninit<bool> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1835: *mut bool = std::ptr::addr_of_mut!(__slate_storage_1835) as *mut bool;
    let mut __slate_storage_935: std::mem::MaybeUninit<UnpackedRecord> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_935: *mut UnpackedRecord =
        std::ptr::addr_of_mut!(__slate_storage_935) as *mut UnpackedRecord;
    let mut __slate_storage_934: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_934: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_934) as *mut i32;
    let mut __slate_storage_933: std::mem::MaybeUninit<*mut BtCursor> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_933: *mut *mut BtCursor =
        std::ptr::addr_of_mut!(__slate_storage_933) as *mut *mut BtCursor;
    let mut __slate_storage_932: std::mem::MaybeUninit<*mut VdbeCursor> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_932: *mut *mut VdbeCursor =
        std::ptr::addr_of_mut!(__slate_storage_932) as *mut *mut VdbeCursor;
    let mut __slate_storage_1834: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1834: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1834) as *mut i32;
    let mut __slate_storage_931: std::mem::MaybeUninit<*mut VdbeCursor> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_931: *mut *mut VdbeCursor =
        std::ptr::addr_of_mut!(__slate_storage_931) as *mut *mut VdbeCursor;
    let mut __slate_storage_1833: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1833: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1833) as *mut i32;
    let mut __slate_storage_1832: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1832: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_1832) as *mut i64;
    let mut __slate_storage_1831: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1831: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_1831) as *mut i64;
    let mut __slate_storage_1830: std::mem::MaybeUninit<*mut Vdbe> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1830: *mut *mut Vdbe =
        std::ptr::addr_of_mut!(__slate_storage_1830) as *mut *mut Vdbe;
    let mut __slate_storage_930: std::mem::MaybeUninit<BtreePayload> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_930: *mut BtreePayload =
        std::ptr::addr_of_mut!(__slate_storage_930) as *mut BtreePayload;
    let mut __slate_storage_929: std::mem::MaybeUninit<*mut VdbeCursor> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_929: *mut *mut VdbeCursor =
        std::ptr::addr_of_mut!(__slate_storage_929) as *mut *mut VdbeCursor;
    let mut __slate_storage_1829: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1829: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_1829) as *mut u32;
    let mut __slate_storage_1828: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1828: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_1828) as *mut u32;
    let mut __slate_storage_1827: std::mem::MaybeUninit<*mut u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1827: *mut *mut u32 =
        std::ptr::addr_of_mut!(__slate_storage_1827) as *mut *mut u32;
    let mut __slate_storage_928: std::mem::MaybeUninit<*mut VdbeCursor> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_928: *mut *mut VdbeCursor =
        std::ptr::addr_of_mut!(__slate_storage_928) as *mut *mut VdbeCursor;
    let mut __slate_storage_927: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_927: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_927) as *mut i32;
    let mut __slate_storage_926: std::mem::MaybeUninit<*mut BtCursor> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_926: *mut *mut BtCursor =
        std::ptr::addr_of_mut!(__slate_storage_926) as *mut *mut BtCursor;
    let mut __slate_storage_925: std::mem::MaybeUninit<*mut VdbeCursor> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_925: *mut *mut VdbeCursor =
        std::ptr::addr_of_mut!(__slate_storage_925) as *mut *mut VdbeCursor;
    let mut __slate_storage_924: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_924: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_924) as *mut i32;
    let mut __slate_storage_923: std::mem::MaybeUninit<*mut BtCursor> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_923: *mut *mut BtCursor =
        std::ptr::addr_of_mut!(__slate_storage_923) as *mut *mut BtCursor;
    let mut __slate_storage_922: std::mem::MaybeUninit<*mut VdbeCursor> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_922: *mut *mut VdbeCursor =
        std::ptr::addr_of_mut!(__slate_storage_922) as *mut *mut VdbeCursor;
    let mut __slate_storage_1826: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1826: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_1826) as *mut u32;
    let mut __slate_storage_1825: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1825: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_1825) as *mut u32;
    let mut __slate_storage_1824: std::mem::MaybeUninit<*mut u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1824: *mut *mut u32 =
        std::ptr::addr_of_mut!(__slate_storage_1824) as *mut *mut u32;
    let mut __slate_storage_921: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_921: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_921) as *mut i64;
    let mut __slate_storage_920: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_920: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_920) as *mut i32;
    let mut __slate_storage_919: std::mem::MaybeUninit<*mut BtCursor> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_919: *mut *mut BtCursor =
        std::ptr::addr_of_mut!(__slate_storage_919) as *mut *mut BtCursor;
    let mut __slate_storage_918: std::mem::MaybeUninit<*mut VdbeCursor> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_918: *mut *mut VdbeCursor =
        std::ptr::addr_of_mut!(__slate_storage_918) as *mut *mut VdbeCursor;
    let mut __slate_storage_917: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_917: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_917) as *mut i32;
    let mut __slate_storage_916: std::mem::MaybeUninit<*mut BtCursor> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_916: *mut *mut BtCursor =
        std::ptr::addr_of_mut!(__slate_storage_916) as *mut *mut BtCursor;
    let mut __slate_storage_915: std::mem::MaybeUninit<*mut VdbeCursor> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_915: *mut *mut VdbeCursor =
        std::ptr::addr_of_mut!(__slate_storage_915) as *mut *mut VdbeCursor;
    let mut __slate_storage_914: std::mem::MaybeUninit<*mut VdbeCursor> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_914: *mut *mut VdbeCursor =
        std::ptr::addr_of_mut!(__slate_storage_914) as *mut *mut VdbeCursor;
    let mut __slate_storage_913: std::mem::MaybeUninit<*const sqlite3_module> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_913: *mut *const sqlite3_module =
        std::ptr::addr_of_mut!(__slate_storage_913) as *mut *const sqlite3_module;
    let mut __slate_storage_912: std::mem::MaybeUninit<*mut sqlite3_vtab> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_912: *mut *mut sqlite3_vtab =
        std::ptr::addr_of_mut!(__slate_storage_912) as *mut *mut sqlite3_vtab;
    let mut __slate_storage_911: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_911: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_911) as *mut i64;
    let mut __slate_storage_910: std::mem::MaybeUninit<*mut VdbeCursor> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_910: *mut *mut VdbeCursor =
        std::ptr::addr_of_mut!(__slate_storage_910) as *mut *mut VdbeCursor;
    let mut __slate_storage_1823: std::mem::MaybeUninit<bool> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1823: *mut bool = std::ptr::addr_of_mut!(__slate_storage_1823) as *mut bool;
    let mut __slate_storage_909: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_909: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_909) as *mut u32;
    let mut __slate_storage_908: std::mem::MaybeUninit<*mut BtCursor> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_908: *mut *mut BtCursor =
        std::ptr::addr_of_mut!(__slate_storage_908) as *mut *mut BtCursor;
    let mut __slate_storage_907: std::mem::MaybeUninit<*mut VdbeCursor> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_907: *mut *mut VdbeCursor =
        std::ptr::addr_of_mut!(__slate_storage_907) as *mut *mut VdbeCursor;
    let mut __slate_storage_906: std::mem::MaybeUninit<*mut VdbeCursor> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_906: *mut *mut VdbeCursor =
        std::ptr::addr_of_mut!(__slate_storage_906) as *mut *mut VdbeCursor;
    let mut __slate_storage_905: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_905: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_905) as *mut i32;
    let mut __slate_storage_904: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_904: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_904) as *mut i32;
    let mut __slate_storage_903: std::mem::MaybeUninit<*mut VdbeCursor> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_903: *mut *mut VdbeCursor =
        std::ptr::addr_of_mut!(__slate_storage_903) as *mut *mut VdbeCursor;
    let mut __slate_storage_1822: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1822: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_1822) as *mut i64;
    let mut __slate_storage_1821: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1821: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_1821) as *mut i64;
    let mut __slate_storage_1820: std::mem::MaybeUninit<*mut Vdbe> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1820: *mut *mut Vdbe =
        std::ptr::addr_of_mut!(__slate_storage_1820) as *mut *mut Vdbe;
    let mut __slate_storage_1819: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1819: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_1819) as *mut u32;
    let mut __slate_storage_1818: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1818: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_1818) as *mut u32;
    let mut __slate_storage_902: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_902: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_902) as *mut i32;
    let mut __slate_storage_901: std::mem::MaybeUninit<*mut Table> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_901: *mut *mut Table =
        std::ptr::addr_of_mut!(__slate_storage_901) as *mut *mut Table;
    let mut __slate_storage_900: std::mem::MaybeUninit<*const i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_900: *mut *const i8 =
        std::ptr::addr_of_mut!(__slate_storage_900) as *mut *const i8;
    let mut __slate_storage_899: std::mem::MaybeUninit<*mut VdbeCursor> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_899: *mut *mut VdbeCursor =
        std::ptr::addr_of_mut!(__slate_storage_899) as *mut *mut VdbeCursor; // Rowid value to insert with
    let mut __slate_storage_898: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_898: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_898) as *mut i64; // Cursor to read from
    let mut __slate_storage_897: std::mem::MaybeUninit<*mut VdbeCursor> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_897: *mut *mut VdbeCursor =
        std::ptr::addr_of_mut!(__slate_storage_897) as *mut *mut VdbeCursor; // Cursor to write to
    let mut __slate_storage_896: std::mem::MaybeUninit<*mut VdbeCursor> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_896: *mut *mut VdbeCursor =
        std::ptr::addr_of_mut!(__slate_storage_896) as *mut *mut VdbeCursor;
    let mut __slate_storage_1817: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1817: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_1817) as *mut u32;
    let mut __slate_storage_1816: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1816: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_1816) as *mut u32;
    let mut __slate_storage_1815: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1815: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_1815) as *mut i64;
    let mut __slate_storage_1814: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1814: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_1814) as *mut i64;
    let mut __slate_storage_1813: std::mem::MaybeUninit<*mut Vdbe> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1813: *mut *mut Vdbe =
        std::ptr::addr_of_mut!(__slate_storage_1813) as *mut *mut Vdbe; // Payload to be inserted
    let mut __slate_storage_895: std::mem::MaybeUninit<BtreePayload> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_895: *mut BtreePayload =
        std::ptr::addr_of_mut!(__slate_storage_895) as *mut BtreePayload; // Table structure - used by update and pre-update hooks
    let mut __slate_storage_894: std::mem::MaybeUninit<*mut Table> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_894: *mut *mut Table =
        std::ptr::addr_of_mut!(__slate_storage_894) as *mut *mut Table; // database name - used by the update hook
    let mut __slate_storage_893: std::mem::MaybeUninit<*const i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_893: *mut *const i8 =
        std::ptr::addr_of_mut!(__slate_storage_893) as *mut *const i8; // Result of prior seek or 0 if no USESEEKRESULT flag
    let mut __slate_storage_892: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_892: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_892) as *mut i32; // Cursor to table into which insert is written
    let mut __slate_storage_891: std::mem::MaybeUninit<*mut VdbeCursor> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_891: *mut *mut VdbeCursor =
        std::ptr::addr_of_mut!(__slate_storage_891) as *mut *mut VdbeCursor; // MEM cell holding key  for the record
    let mut __slate_storage_890: std::mem::MaybeUninit<*mut sqlite3_value> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_890: *mut *mut sqlite3_value =
        std::ptr::addr_of_mut!(__slate_storage_890) as *mut *mut sqlite3_value; // MEM cell holding data for the record to be inserted
    let mut __slate_storage_889: std::mem::MaybeUninit<*mut sqlite3_value> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_889: *mut *mut sqlite3_value =
        std::ptr::addr_of_mut!(__slate_storage_889) as *mut *mut sqlite3_value;
    let mut __slate_storage_1812: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1812: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1812) as *mut i32;
    let mut __slate_storage_1811: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1811: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1811) as *mut i32;
    let mut __slate_storage_1810: std::mem::MaybeUninit<bool> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1810: *mut bool = std::ptr::addr_of_mut!(__slate_storage_1810) as *mut bool;
    let mut __slate_storage_1809: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1809: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1809) as *mut i32;
    let mut __slate_storage_1808: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1808: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_1808) as *mut i64;
    let mut __slate_storage_1807: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1807: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_1807) as *mut i64;
    let mut __slate_storage_1806: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1806: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_1806) as *mut i64;
    let mut __slate_storage_1805: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1805: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_1805) as *mut i64;
    let mut __slate_storage_1804: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1804: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_1804) as *mut i64;
    let mut __slate_storage_1803: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1803: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_1803) as *mut i64; // Root frame of VDBE
    let mut __slate_storage_888: std::mem::MaybeUninit<*mut VdbeFrame> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_888: *mut *mut VdbeFrame =
        std::ptr::addr_of_mut!(__slate_storage_888) as *mut *mut VdbeFrame; // Register holding largest rowid for AUTOINCREMENT
    let mut __slate_storage_887: std::mem::MaybeUninit<*mut sqlite3_value> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_887: *mut *mut sqlite3_value =
        std::ptr::addr_of_mut!(__slate_storage_887) as *mut *mut sqlite3_value; // Counter to limit the number of searches
    let mut __slate_storage_886: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_886: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_886) as *mut i32; // Result of an sqlite3BtreeLast()
    let mut __slate_storage_885: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_885: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_885) as *mut i32; // Cursor of table to get the new rowid
    let mut __slate_storage_884: std::mem::MaybeUninit<*mut VdbeCursor> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_884: *mut *mut VdbeCursor =
        std::ptr::addr_of_mut!(__slate_storage_884) as *mut *mut VdbeCursor; // The new rowid
    let mut __slate_storage_883: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_883: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_883) as *mut i64;
    let mut __slate_storage_1802: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1802: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_1802) as *mut i64;
    let mut __slate_storage_1801: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1801: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_1801) as *mut i64;
    let mut __slate_storage_1800: std::mem::MaybeUninit<*mut VdbeCursor> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1800: *mut *mut VdbeCursor =
        std::ptr::addr_of_mut!(__slate_storage_1800) as *mut *mut VdbeCursor;
    let mut __slate_storage_1799: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1799: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_1799) as *mut i64;
    let mut __slate_storage_1798: std::mem::MaybeUninit<bool> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1798: *mut bool = std::ptr::addr_of_mut!(__slate_storage_1798) as *mut bool;
    // If pIn3->u.i does not contain an integer, compute iKey as the
    // integer value of pIn3.  Jump to P2 if pIn3 cannot be converted
    // into an integer without loss of information.  Take care to avoid
    // changing the datatype of pIn3, however, as it is used by other
    // parts of the prepared statement.
    let mut __slate_storage_882: std::mem::MaybeUninit<sqlite3_value> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_882: *mut sqlite3_value =
        std::ptr::addr_of_mut!(__slate_storage_882) as *mut sqlite3_value;
    let mut __slate_storage_881: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_881: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_881) as *mut i64;
    let mut __slate_storage_880: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_880: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_880) as *mut i32;
    let mut __slate_storage_879: std::mem::MaybeUninit<*mut BtCursor> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_879: *mut *mut BtCursor =
        std::ptr::addr_of_mut!(__slate_storage_879) as *mut *mut BtCursor;
    let mut __slate_storage_878: std::mem::MaybeUninit<*mut VdbeCursor> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_878: *mut *mut VdbeCursor =
        std::ptr::addr_of_mut!(__slate_storage_878) as *mut *mut VdbeCursor;
    let mut __slate_storage_1797: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1797: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1797) as *mut i32;
    let mut __slate_storage_1796: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1796: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1796) as *mut i32;
    let mut __slate_storage_1795: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1795: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1795) as *mut i32;
    let mut __slate_storage_877: std::mem::MaybeUninit<UnpackedRecord> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_877: *mut UnpackedRecord =
        std::ptr::addr_of_mut!(__slate_storage_877) as *mut UnpackedRecord;
    let mut __slate_storage_876: std::mem::MaybeUninit<*mut UnpackedRecord> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_876: *mut *mut UnpackedRecord =
        std::ptr::addr_of_mut!(__slate_storage_876) as *mut *mut UnpackedRecord;
    let mut __slate_storage_875: std::mem::MaybeUninit<*mut VdbeCursor> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_875: *mut *mut VdbeCursor =
        std::ptr::addr_of_mut!(__slate_storage_875) as *mut *mut VdbeCursor;
    let mut __slate_storage_874: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_874: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_874) as *mut i32;
    let mut __slate_storage_873: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_873: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_873) as *mut i32;
    let mut __slate_storage_872: std::mem::MaybeUninit<*mut VdbeCursor> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_872: *mut *mut VdbeCursor =
        std::ptr::addr_of_mut!(__slate_storage_872) as *mut *mut VdbeCursor;
    let mut __slate_storage_871: std::mem::MaybeUninit<*mut VdbeCursor> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_871: *mut *mut VdbeCursor =
        std::ptr::addr_of_mut!(__slate_storage_871) as *mut *mut VdbeCursor;
    let mut __slate_storage_870: std::mem::MaybeUninit<*mut VdbeCursor> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_870: *mut *mut VdbeCursor =
        std::ptr::addr_of_mut!(__slate_storage_870) as *mut *mut VdbeCursor;
    let mut __slate_storage_1794: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1794: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1794) as *mut i32;
    let mut __slate_storage_1793: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1793: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1793) as *mut i32;
    let mut __slate_storage_1792: std::mem::MaybeUninit<*mut VdbeOp> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1792: *mut *mut VdbeOp =
        std::ptr::addr_of_mut!(__slate_storage_1792) as *mut *mut VdbeOp;
    let mut __slate_storage_1791: std::mem::MaybeUninit<*mut VdbeOp> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1791: *mut *mut VdbeOp =
        std::ptr::addr_of_mut!(__slate_storage_1791) as *mut *mut VdbeOp;
    let mut __slate_storage_869: std::mem::MaybeUninit<UnpackedRecord> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_869: *mut UnpackedRecord =
        std::ptr::addr_of_mut!(__slate_storage_869) as *mut UnpackedRecord;
    let mut __slate_storage_868: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_868: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_868) as *mut i32;
    let mut __slate_storage_867: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_867: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_867) as *mut i32;
    let mut __slate_storage_866: std::mem::MaybeUninit<*mut VdbeCursor> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_866: *mut *mut VdbeCursor =
        std::ptr::addr_of_mut!(__slate_storage_866) as *mut *mut VdbeCursor;
    let mut __slate_storage_1790: std::mem::MaybeUninit<*mut VdbeOp> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1790: *mut *mut VdbeOp =
        std::ptr::addr_of_mut!(__slate_storage_1790) as *mut *mut VdbeOp;
    let mut __slate_storage_1789: std::mem::MaybeUninit<*mut VdbeOp> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1789: *mut *mut VdbeOp =
        std::ptr::addr_of_mut!(__slate_storage_1789) as *mut *mut VdbeOp;
    let mut __slate_storage_1786: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1786: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1786) as *mut i32;
    let mut __slate_storage_1785: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1785: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1785) as *mut i32;
    let mut __slate_storage_1788: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1788: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1788) as *mut i32;
    let mut __slate_storage_1787: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1787: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1787) as *mut i32;
    let mut __slate_storage_865: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_865: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_865) as *mut i32;
    let mut __slate_storage_864: std::mem::MaybeUninit<u16> = std::mem::MaybeUninit::uninit();
    let __slate_slot_864: *mut u16 = std::ptr::addr_of_mut!(__slate_storage_864) as *mut u16;
    let mut __slate_storage_863: std::mem::MaybeUninit<u16> = std::mem::MaybeUninit::uninit();
    let __slate_slot_863: *mut u16 = std::ptr::addr_of_mut!(__slate_storage_863) as *mut u16; // Only interested in == results
    let mut __slate_storage_862: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_862: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_862) as *mut i32; // The rowid we are to seek to
    let mut __slate_storage_861: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_861: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_861) as *mut i64; // Number of columns or fields in the key
    let mut __slate_storage_860: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_860: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_860) as *mut i32; // The key to seek for
    let mut __slate_storage_859: std::mem::MaybeUninit<UnpackedRecord> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_859: *mut UnpackedRecord =
        std::ptr::addr_of_mut!(__slate_storage_859) as *mut UnpackedRecord; // The cursor to seek
    let mut __slate_storage_858: std::mem::MaybeUninit<*mut VdbeCursor> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_858: *mut *mut VdbeCursor =
        std::ptr::addr_of_mut!(__slate_storage_858) as *mut *mut VdbeCursor; // Opcode
    let mut __slate_storage_857: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_857: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_857) as *mut i32; // Comparison result
    let mut __slate_storage_856: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_856: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_856) as *mut i32;
    let mut __slate_storage_855: std::mem::MaybeUninit<*mut VdbeCursor> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_855: *mut *mut VdbeCursor =
        std::ptr::addr_of_mut!(__slate_storage_855) as *mut *mut VdbeCursor;
    let mut __slate_storage_1784: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1784: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_1784) as *mut i64;
    let mut __slate_storage_1783: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1783: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_1783) as *mut i64;
    let mut __slate_storage_1782: std::mem::MaybeUninit<*mut VdbeCursor> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1782: *mut *mut VdbeCursor =
        std::ptr::addr_of_mut!(__slate_storage_1782) as *mut *mut VdbeCursor;
    let mut __slate_storage_854: std::mem::MaybeUninit<*mut VdbeCursor> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_854: *mut *mut VdbeCursor =
        std::ptr::addr_of_mut!(__slate_storage_854) as *mut *mut VdbeCursor;
    let mut __slate_storage_853: std::mem::MaybeUninit<*mut VdbeCursor> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_853: *mut *mut VdbeCursor =
        std::ptr::addr_of_mut!(__slate_storage_853) as *mut *mut VdbeCursor;
    let mut __slate_storage_1781: std::mem::MaybeUninit<*mut KeyInfo> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1781: *mut *mut KeyInfo =
        std::ptr::addr_of_mut!(__slate_storage_1781) as *mut *mut KeyInfo;
    // If a transient index is required, create it by calling
    // sqlite3BtreeCreateTable() with the BTREE_BLOBKEY flag before
    // opening it. If a transient table is required, just use the
    // automatically created table with root-page 1 (an BLOB_INTKEY table).
    let mut __slate_storage_1780: std::mem::MaybeUninit<*mut KeyInfo> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1780: *mut *mut KeyInfo =
        std::ptr::addr_of_mut!(__slate_storage_1780) as *mut *mut KeyInfo;
    let mut __slate_storage_851: std::mem::MaybeUninit<*mut KeyInfo> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_851: *mut *mut KeyInfo =
        std::ptr::addr_of_mut!(__slate_storage_851) as *mut *mut KeyInfo;
    let mut __slate_storage_850: std::mem::MaybeUninit<*mut VdbeCursor> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_850: *mut *mut VdbeCursor =
        std::ptr::addr_of_mut!(__slate_storage_850) as *mut *mut VdbeCursor; // The new cursor
    let mut __slate_storage_849: std::mem::MaybeUninit<*mut VdbeCursor> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_849: *mut *mut VdbeCursor =
        std::ptr::addr_of_mut!(__slate_storage_849) as *mut *mut VdbeCursor; // The original cursor to be duplicated
    let mut __slate_storage_848: std::mem::MaybeUninit<*mut VdbeCursor> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_848: *mut *mut VdbeCursor =
        std::ptr::addr_of_mut!(__slate_storage_848) as *mut *mut VdbeCursor;
    let mut __slate_storage_847: std::mem::MaybeUninit<*mut Db> = std::mem::MaybeUninit::uninit();
    let __slate_slot_847: *mut *mut Db =
        std::ptr::addr_of_mut!(__slate_storage_847) as *mut *mut Db;
    let mut __slate_storage_846: std::mem::MaybeUninit<*mut VdbeCursor> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_846: *mut *mut VdbeCursor =
        std::ptr::addr_of_mut!(__slate_storage_846) as *mut *mut VdbeCursor;
    let mut __slate_storage_845: std::mem::MaybeUninit<*mut Btree> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_845: *mut *mut Btree =
        std::ptr::addr_of_mut!(__slate_storage_845) as *mut *mut Btree;
    let mut __slate_storage_844: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_844: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_844) as *mut i32;
    let mut __slate_storage_843: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_843: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_843) as *mut i32;
    let mut __slate_storage_842: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_842: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_842) as *mut u32;
    let mut __slate_storage_841: std::mem::MaybeUninit<*mut KeyInfo> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_841: *mut *mut KeyInfo =
        std::ptr::addr_of_mut!(__slate_storage_841) as *mut *mut KeyInfo;
    let mut __slate_storage_840: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_840: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_840) as *mut i32;
    let mut __slate_storage_1779: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1779: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_1779) as *mut u32;
    let mut __slate_storage_1778: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1778: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_1778) as *mut u32;
    let mut __slate_storage_1777: std::mem::MaybeUninit<*mut sqlite3> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1777: *mut *mut sqlite3 =
        std::ptr::addr_of_mut!(__slate_storage_1777) as *mut *mut sqlite3;
    let mut __slate_storage_839: std::mem::MaybeUninit<*mut Db> = std::mem::MaybeUninit::uninit();
    let __slate_slot_839: *mut *mut Db =
        std::ptr::addr_of_mut!(__slate_storage_839) as *mut *mut Db;
    let mut __slate_storage_838: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_838: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_838) as *mut i32;
    let mut __slate_storage_837: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_837: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_837) as *mut i32;
    let mut __slate_storage_836: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_836: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_836) as *mut i32;
    let mut __slate_storage_1776: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1776: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1776) as *mut i32;
    let mut __slate_storage_1775: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1775: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1775) as *mut i32;
    let mut __slate_storage_1774: std::mem::MaybeUninit<*mut sqlite3> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1774: *mut *mut sqlite3 =
        std::ptr::addr_of_mut!(__slate_storage_1774) as *mut *mut sqlite3;
    let mut __slate_storage_835: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_835: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_835) as *mut i32;
    let mut __slate_storage_834: std::mem::MaybeUninit<*mut Db> = std::mem::MaybeUninit::uninit();
    let __slate_slot_834: *mut *mut Db =
        std::ptr::addr_of_mut!(__slate_storage_834) as *mut *mut Db;
    let mut __slate_storage_833: std::mem::MaybeUninit<*mut Btree> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_833: *mut *mut Btree =
        std::ptr::addr_of_mut!(__slate_storage_833) as *mut *mut Btree;
    let mut __slate_storage_1773: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1773: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1773) as *mut i32;
    let mut __slate_storage_832: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_832: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_832) as *mut i32;
    let mut __slate_storage_831: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_831: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_831) as *mut i32;
    let mut __slate_storage_1755: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1755: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1755) as *mut i32;
    let mut __slate_storage_1754: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1754: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1754) as *mut i32;
    let mut __slate_storage_1753: std::mem::MaybeUninit<*mut sqlite3> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1753: *mut *mut sqlite3 =
        std::ptr::addr_of_mut!(__slate_storage_1753) as *mut *mut sqlite3;
    let mut __slate_storage_1772: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1772: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1772) as *mut i32;
    let mut __slate_storage_1771: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1771: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1771) as *mut i32;
    let mut __slate_storage_1770: std::mem::MaybeUninit<*mut sqlite3> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1770: *mut *mut sqlite3 =
        std::ptr::addr_of_mut!(__slate_storage_1770) as *mut *mut sqlite3;
    let mut __slate_storage_1769: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1769: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1769) as *mut i32;
    let mut __slate_storage_1768: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1768: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1768) as *mut i32;
    let mut __slate_storage_1767: std::mem::MaybeUninit<*mut sqlite3> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1767: *mut *mut sqlite3 =
        std::ptr::addr_of_mut!(__slate_storage_1767) as *mut *mut sqlite3;
    let mut __slate_storage_1759: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1759: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1759) as *mut i32;
    let mut __slate_storage_1766: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1766: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_1766) as *mut u32;
    let mut __slate_storage_1765: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1765: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_1765) as *mut u32;
    let mut __slate_storage_1764: std::mem::MaybeUninit<*mut sqlite3> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1764: *mut *mut sqlite3 =
        std::ptr::addr_of_mut!(__slate_storage_1764) as *mut *mut sqlite3;
    let mut __slate_storage_1763: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1763: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1763) as *mut i32;
    let mut __slate_storage_1762: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1762: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1762) as *mut i32;
    let mut __slate_storage_1761: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1761: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1761) as *mut i32;
    let mut __slate_storage_1760: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1760: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1760) as *mut i32;
    let mut __slate_storage_830: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_830: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_830) as *mut i32;
    // Determine whether or not this is a transaction savepoint. If so,
    // and this is a RELEASE command, then the current transaction
    // is committed.
    let mut __slate_storage_829: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_829: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_829) as *mut i32;
    let mut __slate_storage_1756: std::mem::MaybeUninit<bool> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1756: *mut bool = std::ptr::addr_of_mut!(__slate_storage_1756) as *mut bool;
    let mut __slate_storage_1758: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1758: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1758) as *mut i32;
    let mut __slate_storage_1757: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1757: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1757) as *mut i32;
    let mut __slate_storage_828: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_828: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_828) as *mut i32;
    let mut __slate_storage_827: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_827: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_827) as *mut i32;
    let mut __slate_storage_826: std::mem::MaybeUninit<*mut Savepoint> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_826: *mut *mut Savepoint =
        std::ptr::addr_of_mut!(__slate_storage_826) as *mut *mut Savepoint;
    let mut __slate_storage_825: std::mem::MaybeUninit<*mut Savepoint> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_825: *mut *mut Savepoint =
        std::ptr::addr_of_mut!(__slate_storage_825) as *mut *mut Savepoint;
    let mut __slate_storage_824: std::mem::MaybeUninit<*mut Savepoint> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_824: *mut *mut Savepoint =
        std::ptr::addr_of_mut!(__slate_storage_824) as *mut *mut Savepoint;
    let mut __slate_storage_823: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_823: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_823) as *mut i32; // Name of savepoint
    let mut __slate_storage_822: std::mem::MaybeUninit<*mut i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_822: *mut *mut i8 =
        std::ptr::addr_of_mut!(__slate_storage_822) as *mut *mut i8; // Value of P1 operand
    let mut __slate_storage_821: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_821: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_821) as *mut i32;
    let mut __slate_storage_820: std::mem::MaybeUninit<*mut BtCursor> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_820: *mut *mut BtCursor =
        std::ptr::addr_of_mut!(__slate_storage_820) as *mut *mut BtCursor;
    let mut __slate_storage_819: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_819: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_819) as *mut i64;
    let mut __slate_storage_1752: std::mem::MaybeUninit<*mut sqlite3_value> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1752: *mut *mut sqlite3_value =
        std::ptr::addr_of_mut!(__slate_storage_1752) as *mut *mut sqlite3_value;
    let mut __slate_storage_1751: std::mem::MaybeUninit<*mut sqlite3_value> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1751: *mut *mut sqlite3_value =
        std::ptr::addr_of_mut!(__slate_storage_1751) as *mut *mut sqlite3_value;
    let mut __slate_storage_1742: std::mem::MaybeUninit<*mut u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1742: *mut *mut u8 =
        std::ptr::addr_of_mut!(__slate_storage_1742) as *mut *mut u8;
    let mut __slate_storage_1741: std::mem::MaybeUninit<*mut u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1741: *mut *mut u8 =
        std::ptr::addr_of_mut!(__slate_storage_1741) as *mut *mut u8;
    let mut __slate_storage_1740: std::mem::MaybeUninit<u64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1740: *mut u64 = std::ptr::addr_of_mut!(__slate_storage_1740) as *mut u64;
    let mut __slate_storage_1739: std::mem::MaybeUninit<u64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1739: *mut u64 = std::ptr::addr_of_mut!(__slate_storage_1739) as *mut u64;
    let mut __slate_storage_817: std::mem::MaybeUninit<u64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_817: *mut u64 = std::ptr::addr_of_mut!(__slate_storage_817) as *mut u64;
    let mut __slate_storage_1738: std::mem::MaybeUninit<*mut u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1738: *mut *mut u8 =
        std::ptr::addr_of_mut!(__slate_storage_1738) as *mut *mut u8;
    let mut __slate_storage_1737: std::mem::MaybeUninit<*mut u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1737: *mut *mut u8 =
        std::ptr::addr_of_mut!(__slate_storage_1737) as *mut *mut u8;
    let mut __slate_storage_1746: std::mem::MaybeUninit<*mut u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1746: *mut *mut u8 =
        std::ptr::addr_of_mut!(__slate_storage_1746) as *mut *mut u8;
    let mut __slate_storage_1745: std::mem::MaybeUninit<*mut u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1745: *mut *mut u8 =
        std::ptr::addr_of_mut!(__slate_storage_1745) as *mut *mut u8;
    let mut __slate_storage_1744: std::mem::MaybeUninit<*mut u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1744: *mut *mut u8 =
        std::ptr::addr_of_mut!(__slate_storage_1744) as *mut *mut u8;
    let mut __slate_storage_1743: std::mem::MaybeUninit<*mut u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1743: *mut *mut u8 =
        std::ptr::addr_of_mut!(__slate_storage_1743) as *mut *mut u8;
    let mut __slate_storage_1750: std::mem::MaybeUninit<*mut u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1750: *mut *mut u8 =
        std::ptr::addr_of_mut!(__slate_storage_1750) as *mut *mut u8;
    let mut __slate_storage_1749: std::mem::MaybeUninit<*mut u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1749: *mut *mut u8 =
        std::ptr::addr_of_mut!(__slate_storage_1749) as *mut *mut u8;
    let mut __slate_storage_1748: std::mem::MaybeUninit<*mut u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1748: *mut *mut u8 =
        std::ptr::addr_of_mut!(__slate_storage_1748) as *mut *mut u8;
    let mut __slate_storage_1747: std::mem::MaybeUninit<*mut u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1747: *mut *mut u8 =
        std::ptr::addr_of_mut!(__slate_storage_1747) as *mut *mut u8;
    let mut __slate_storage_1734: std::mem::MaybeUninit<*mut u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1734: *mut *mut u8 =
        std::ptr::addr_of_mut!(__slate_storage_1734) as *mut *mut u8;
    let mut __slate_storage_1733: std::mem::MaybeUninit<*mut u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1733: *mut *mut u8 =
        std::ptr::addr_of_mut!(__slate_storage_1733) as *mut *mut u8;
    let mut __slate_storage_1736: std::mem::MaybeUninit<*mut u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1736: *mut *mut u8 =
        std::ptr::addr_of_mut!(__slate_storage_1736) as *mut *mut u8;
    let mut __slate_storage_1735: std::mem::MaybeUninit<*mut u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1735: *mut *mut u8 =
        std::ptr::addr_of_mut!(__slate_storage_1735) as *mut *mut u8;
    let mut __slate_storage_1732: std::mem::MaybeUninit<u16> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1732: *mut u16 = std::ptr::addr_of_mut!(__slate_storage_1732) as *mut u16;
    let mut __slate_storage_1731: std::mem::MaybeUninit<u16> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1731: *mut u16 = std::ptr::addr_of_mut!(__slate_storage_1731) as *mut u16;
    let mut __slate_storage_1730: std::mem::MaybeUninit<*mut sqlite3_value> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1730: *mut *mut sqlite3_value =
        std::ptr::addr_of_mut!(__slate_storage_1730) as *mut *mut sqlite3_value;
    let mut __slate_storage_1725: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1725: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1725) as *mut i32;
    // The common case
    let mut __slate_storage_1724: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1724: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1724) as *mut i32;
    let mut __slate_storage_1729: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1729: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1729) as *mut i32;
    let mut __slate_storage_1728: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1728: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1728) as *mut i32;
    let mut __slate_storage_1727: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1727: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1727) as *mut i32;
    let mut __slate_storage_1726: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1726: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1726) as *mut i32;
    let mut __slate_storage_1723: std::mem::MaybeUninit<*mut sqlite3_value> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1723: *mut *mut sqlite3_value =
        std::ptr::addr_of_mut!(__slate_storage_1723) as *mut *mut sqlite3_value;
    let mut __slate_storage_1722: std::mem::MaybeUninit<*mut sqlite3_value> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1722: *mut *mut sqlite3_value =
        std::ptr::addr_of_mut!(__slate_storage_1722) as *mut *mut sqlite3_value;
    let mut __slate_storage_1687: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1687: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1687) as *mut i32;
    let mut __slate_storage_1686: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1686: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1686) as *mut i32;
    let mut __slate_storage_1691: std::mem::MaybeUninit<u64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1691: *mut u64 = std::ptr::addr_of_mut!(__slate_storage_1691) as *mut u64;
    let mut __slate_storage_1690: std::mem::MaybeUninit<u64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1690: *mut u64 = std::ptr::addr_of_mut!(__slate_storage_1690) as *mut u64;
    let mut __slate_storage_1693: std::mem::MaybeUninit<u64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1693: *mut u64 = std::ptr::addr_of_mut!(__slate_storage_1693) as *mut u64;
    let mut __slate_storage_1692: std::mem::MaybeUninit<u64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1692: *mut u64 = std::ptr::addr_of_mut!(__slate_storage_1692) as *mut u64;
    let mut __slate_storage_1695: std::mem::MaybeUninit<u64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1695: *mut u64 = std::ptr::addr_of_mut!(__slate_storage_1695) as *mut u64;
    let mut __slate_storage_1694: std::mem::MaybeUninit<u64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1694: *mut u64 = std::ptr::addr_of_mut!(__slate_storage_1694) as *mut u64;
    let mut __slate_storage_1697: std::mem::MaybeUninit<u64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1697: *mut u64 = std::ptr::addr_of_mut!(__slate_storage_1697) as *mut u64;
    let mut __slate_storage_1696: std::mem::MaybeUninit<u64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1696: *mut u64 = std::ptr::addr_of_mut!(__slate_storage_1696) as *mut u64;
    let mut __slate_storage_1699: std::mem::MaybeUninit<u64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1699: *mut u64 = std::ptr::addr_of_mut!(__slate_storage_1699) as *mut u64;
    let mut __slate_storage_1698: std::mem::MaybeUninit<u64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1698: *mut u64 = std::ptr::addr_of_mut!(__slate_storage_1698) as *mut u64;
    let mut __slate_storage_1707: std::mem::MaybeUninit<u16> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1707: *mut u16 = std::ptr::addr_of_mut!(__slate_storage_1707) as *mut u16;
    let mut __slate_storage_1706: std::mem::MaybeUninit<u16> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1706: *mut u16 = std::ptr::addr_of_mut!(__slate_storage_1706) as *mut u16;
    let mut __slate_storage_1705: std::mem::MaybeUninit<*mut sqlite3_value> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1705: *mut *mut sqlite3_value =
        std::ptr::addr_of_mut!(__slate_storage_1705) as *mut *mut sqlite3_value;
    let mut __slate_storage_1704: std::mem::MaybeUninit<u16> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1704: *mut u16 = std::ptr::addr_of_mut!(__slate_storage_1704) as *mut u16;
    let mut __slate_storage_1703: std::mem::MaybeUninit<u16> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1703: *mut u16 = std::ptr::addr_of_mut!(__slate_storage_1703) as *mut u16;
    let mut __slate_storage_1702: std::mem::MaybeUninit<*mut sqlite3_value> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1702: *mut *mut sqlite3_value =
        std::ptr::addr_of_mut!(__slate_storage_1702) as *mut *mut sqlite3_value;
    let mut __slate_storage_1701: std::mem::MaybeUninit<u64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1701: *mut u64 = std::ptr::addr_of_mut!(__slate_storage_1701) as *mut u64;
    let mut __slate_storage_1700: std::mem::MaybeUninit<u64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1700: *mut u64 = std::ptr::addr_of_mut!(__slate_storage_1700) as *mut u64;
    let mut __slate_storage_1689: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1689: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1689) as *mut i32;
    let mut __slate_storage_1688: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1688: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1688) as *mut i32;
    let mut __slate_storage_816: std::mem::MaybeUninit<u64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_816: *mut u64 = std::ptr::addr_of_mut!(__slate_storage_816) as *mut u64;
    // Figure out whether to use 1, 2, 4, 6 or 8 bytes.
    let mut __slate_storage_815: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_815: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_815) as *mut i64;
    let mut __slate_storage_1711: std::mem::MaybeUninit<u64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1711: *mut u64 = std::ptr::addr_of_mut!(__slate_storage_1711) as *mut u64;
    let mut __slate_storage_1710: std::mem::MaybeUninit<u64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1710: *mut u64 = std::ptr::addr_of_mut!(__slate_storage_1710) as *mut u64;
    let mut __slate_storage_1709: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1709: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1709) as *mut i32;
    let mut __slate_storage_1708: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1708: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1708) as *mut i32;
    let mut __slate_storage_1721: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1721: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1721) as *mut i32;
    let mut __slate_storage_1720: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1720: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1720) as *mut i32;
    let mut __slate_storage_1719: std::mem::MaybeUninit<u64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1719: *mut u64 = std::ptr::addr_of_mut!(__slate_storage_1719) as *mut u64;
    let mut __slate_storage_1718: std::mem::MaybeUninit<u64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1718: *mut u64 = std::ptr::addr_of_mut!(__slate_storage_1718) as *mut u64;
    let mut __slate_storage_1715: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1715: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_1715) as *mut u32;
    let mut __slate_storage_1714: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1714: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_1714) as *mut u32;
    let mut __slate_storage_1717: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1717: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_1717) as *mut i64;
    let mut __slate_storage_1716: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1716: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_1716) as *mut i64;
    let mut __slate_storage_1713: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1713: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_1713) as *mut u32;
    let mut __slate_storage_1712: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1712: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_1712) as *mut u32;
    let mut __slate_storage_1685: std::mem::MaybeUninit<*mut sqlite3_value> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1685: *mut *mut sqlite3_value =
        std::ptr::addr_of_mut!(__slate_storage_1685) as *mut *mut sqlite3_value;
    let mut __slate_storage_1684: std::mem::MaybeUninit<*mut sqlite3_value> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1684: *mut *mut sqlite3_value =
        std::ptr::addr_of_mut!(__slate_storage_1684) as *mut *mut sqlite3_value;
    let mut __slate_storage_1683: std::mem::MaybeUninit<*mut i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1683: *mut *mut i8 =
        std::ptr::addr_of_mut!(__slate_storage_1683) as *mut *mut i8;
    let mut __slate_storage_1682: std::mem::MaybeUninit<*mut i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1682: *mut *mut i8 =
        std::ptr::addr_of_mut!(__slate_storage_1682) as *mut *mut i8;
    let mut __slate_storage_1681: std::mem::MaybeUninit<u16> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1681: *mut u16 = std::ptr::addr_of_mut!(__slate_storage_1681) as *mut u16;
    let mut __slate_storage_1680: std::mem::MaybeUninit<u16> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1680: *mut u16 = std::ptr::addr_of_mut!(__slate_storage_1680) as *mut u16;
    let mut __slate_storage_1679: std::mem::MaybeUninit<*mut sqlite3_value> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1679: *mut *mut sqlite3_value =
        std::ptr::addr_of_mut!(__slate_storage_1679) as *mut *mut sqlite3_value;
    let mut __slate_storage_1678: std::mem::MaybeUninit<u16> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1678: *mut u16 = std::ptr::addr_of_mut!(__slate_storage_1678) as *mut u16;
    let mut __slate_storage_1677: std::mem::MaybeUninit<u16> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1677: *mut u16 = std::ptr::addr_of_mut!(__slate_storage_1677) as *mut u16;
    let mut __slate_storage_1676: std::mem::MaybeUninit<*mut sqlite3_value> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1676: *mut *mut sqlite3_value =
        std::ptr::addr_of_mut!(__slate_storage_1676) as *mut *mut sqlite3_value; // Where to write next byte of the payload
    let mut __slate_storage_814: std::mem::MaybeUninit<*mut u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_814: *mut *mut u8 =
        std::ptr::addr_of_mut!(__slate_storage_814) as *mut *mut u8; // Where to write next byte of the header
    let mut __slate_storage_813: std::mem::MaybeUninit<*mut u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_813: *mut *mut u8 =
        std::ptr::addr_of_mut!(__slate_storage_813) as *mut *mut u8; // Length of a field
    let mut __slate_storage_812: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_812: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_812) as *mut u32; // The affinity string for the record
    let mut __slate_storage_811: std::mem::MaybeUninit<*mut i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_811: *mut *mut i8 =
        std::ptr::addr_of_mut!(__slate_storage_811) as *mut *mut i8; // Number of fields in the record
    let mut __slate_storage_810: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_810: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_810) as *mut i32; // Last field of the record
    let mut __slate_storage_809: std::mem::MaybeUninit<*mut sqlite3_value> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_809: *mut *mut sqlite3_value =
        std::ptr::addr_of_mut!(__slate_storage_809) as *mut *mut sqlite3_value; // First field to be combined into the record
    let mut __slate_storage_808: std::mem::MaybeUninit<*mut sqlite3_value> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_808: *mut *mut sqlite3_value =
        std::ptr::addr_of_mut!(__slate_storage_808) as *mut *mut sqlite3_value; // Type field
    let mut __slate_storage_807: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_807: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_807) as *mut u32; // Number of bytes in a varint
    let mut __slate_storage_806: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_806: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_806) as *mut i32; // Number of zero bytes at the end of the record
    let mut __slate_storage_805: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_805: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_805) as *mut i64; // Data space required for this record
    let mut __slate_storage_804: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_804: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_804) as *mut i64; // Number of bytes of header space
    let mut __slate_storage_803: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_803: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_803) as *mut i32; // Number of bytes of data space
    let mut __slate_storage_802: std::mem::MaybeUninit<u64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_802: *mut u64 = std::ptr::addr_of_mut!(__slate_storage_802) as *mut u64; // The new record
    let mut __slate_storage_801: std::mem::MaybeUninit<*mut sqlite3_value> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_801: *mut *mut sqlite3_value =
        std::ptr::addr_of_mut!(__slate_storage_801) as *mut *mut sqlite3_value;
    let mut __slate_storage_1675: std::mem::MaybeUninit<*mut sqlite3_value> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1675: *mut *mut sqlite3_value =
        std::ptr::addr_of_mut!(__slate_storage_1675) as *mut *mut sqlite3_value;
    let mut __slate_storage_1674: std::mem::MaybeUninit<*mut sqlite3_value> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1674: *mut *mut sqlite3_value =
        std::ptr::addr_of_mut!(__slate_storage_1674) as *mut *mut sqlite3_value;
    let mut __slate_storage_1673: std::mem::MaybeUninit<*const i8> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1673: *mut *const i8 =
        std::ptr::addr_of_mut!(__slate_storage_1673) as *mut *const i8;
    let mut __slate_storage_1672: std::mem::MaybeUninit<*const i8> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1672: *mut *const i8 =
        std::ptr::addr_of_mut!(__slate_storage_1672) as *mut *const i8;
    let mut __slate_storage_1671: std::mem::MaybeUninit<u16> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1671: *mut u16 = std::ptr::addr_of_mut!(__slate_storage_1671) as *mut u16;
    let mut __slate_storage_1670: std::mem::MaybeUninit<u16> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1670: *mut u16 = std::ptr::addr_of_mut!(__slate_storage_1670) as *mut u16;
    let mut __slate_storage_1669: std::mem::MaybeUninit<*mut sqlite3_value> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1669: *mut *mut sqlite3_value =
        std::ptr::addr_of_mut!(__slate_storage_1669) as *mut *mut sqlite3_value;
    let mut __slate_storage_1668: std::mem::MaybeUninit<u16> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1668: *mut u16 = std::ptr::addr_of_mut!(__slate_storage_1668) as *mut u16;
    let mut __slate_storage_1667: std::mem::MaybeUninit<u16> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1667: *mut u16 = std::ptr::addr_of_mut!(__slate_storage_1667) as *mut u16;
    let mut __slate_storage_1666: std::mem::MaybeUninit<*mut sqlite3_value> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1666: *mut *mut sqlite3_value =
        std::ptr::addr_of_mut!(__slate_storage_1666) as *mut *mut sqlite3_value; // The affinity to be applied
    let mut __slate_storage_800: std::mem::MaybeUninit<*const i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_800: *mut *const i8 =
        std::ptr::addr_of_mut!(__slate_storage_800) as *mut *const i8;
    let mut __slate_storage_1649: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1649: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1649) as *mut i32;
    let mut __slate_storage_1648: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1648: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1648) as *mut i32;
    let mut __slate_storage_1665: std::mem::MaybeUninit<*mut sqlite3_value> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1665: *mut *mut sqlite3_value =
        std::ptr::addr_of_mut!(__slate_storage_1665) as *mut *mut sqlite3_value;
    let mut __slate_storage_1664: std::mem::MaybeUninit<*mut sqlite3_value> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1664: *mut *mut sqlite3_value =
        std::ptr::addr_of_mut!(__slate_storage_1664) as *mut *mut sqlite3_value;
    let mut __slate_storage_1657: std::mem::MaybeUninit<u16> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1657: *mut u16 = std::ptr::addr_of_mut!(__slate_storage_1657) as *mut u16;
    let mut __slate_storage_1656: std::mem::MaybeUninit<u16> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1656: *mut u16 = std::ptr::addr_of_mut!(__slate_storage_1656) as *mut u16;
    let mut __slate_storage_1655: std::mem::MaybeUninit<*mut sqlite3_value> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1655: *mut *mut sqlite3_value =
        std::ptr::addr_of_mut!(__slate_storage_1655) as *mut *mut sqlite3_value;
    let mut __slate_storage_1654: std::mem::MaybeUninit<u16> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1654: *mut u16 = std::ptr::addr_of_mut!(__slate_storage_1654) as *mut u16;
    let mut __slate_storage_1653: std::mem::MaybeUninit<u16> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1653: *mut u16 = std::ptr::addr_of_mut!(__slate_storage_1653) as *mut u16;
    let mut __slate_storage_1652: std::mem::MaybeUninit<*mut sqlite3_value> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1652: *mut *mut sqlite3_value =
        std::ptr::addr_of_mut!(__slate_storage_1652) as *mut *mut sqlite3_value;
    let mut __slate_storage_1663: std::mem::MaybeUninit<u16> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1663: *mut u16 = std::ptr::addr_of_mut!(__slate_storage_1663) as *mut u16;
    let mut __slate_storage_1662: std::mem::MaybeUninit<u16> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1662: *mut u16 = std::ptr::addr_of_mut!(__slate_storage_1662) as *mut u16;
    let mut __slate_storage_1661: std::mem::MaybeUninit<*mut sqlite3_value> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1661: *mut *mut sqlite3_value =
        std::ptr::addr_of_mut!(__slate_storage_1661) as *mut *mut sqlite3_value;
    let mut __slate_storage_1660: std::mem::MaybeUninit<u16> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1660: *mut u16 = std::ptr::addr_of_mut!(__slate_storage_1660) as *mut u16;
    let mut __slate_storage_1659: std::mem::MaybeUninit<u16> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1659: *mut u16 = std::ptr::addr_of_mut!(__slate_storage_1659) as *mut u16;
    let mut __slate_storage_1658: std::mem::MaybeUninit<*mut sqlite3_value> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1658: *mut *mut sqlite3_value =
        std::ptr::addr_of_mut!(__slate_storage_1658) as *mut *mut sqlite3_value;
    let mut __slate_storage_1651: std::mem::MaybeUninit<*mut sqlite3_value> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1651: *mut *mut sqlite3_value =
        std::ptr::addr_of_mut!(__slate_storage_1651) as *mut *mut sqlite3_value;
    let mut __slate_storage_1650: std::mem::MaybeUninit<*mut sqlite3_value> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1650: *mut *mut sqlite3_value =
        std::ptr::addr_of_mut!(__slate_storage_1650) as *mut *mut sqlite3_value;
    let mut __slate_storage_799: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_799: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_799) as *mut i32;
    let mut __slate_storage_798: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_798: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_798) as *mut i32;
    let mut __slate_storage_797: std::mem::MaybeUninit<*mut Column> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_797: *mut *mut Column =
        std::ptr::addr_of_mut!(__slate_storage_797) as *mut *mut Column;
    let mut __slate_storage_796: std::mem::MaybeUninit<*mut Table> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_796: *mut *mut Table =
        std::ptr::addr_of_mut!(__slate_storage_796) as *mut *mut Table;
    // If the column value is a string or blob, we need a persistent
    // value, not a MEM_Ephem value.  This case is a fast short-cut
    // that is equivalent to calling sqlite3VdbeSerialGet() and
    // sqlite3VdbeDeephemeralize().
    let mut __slate_storage_1645: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1645: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1645) as *mut i32;
    let mut __slate_storage_793: std::mem::MaybeUninit<u64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_793: *mut u64 = std::ptr::addr_of_mut!(__slate_storage_793) as *mut u64;
    let mut __slate_storage_1647: std::mem::MaybeUninit<bool> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1647: *mut bool = std::ptr::addr_of_mut!(__slate_storage_1647) as *mut bool;
    // This branch happens only when content is on overflow pages
    let mut __slate_storage_1646: std::mem::MaybeUninit<u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1646: *mut u8 = std::ptr::addr_of_mut!(__slate_storage_1646) as *mut u8;
    let mut __slate_storage_795: std::mem::MaybeUninit<u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_795: *mut u8 = std::ptr::addr_of_mut!(__slate_storage_795) as *mut u8;
    let mut __slate_storage_1644: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1644: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1644) as *mut i32;
    let mut __slate_storage_1643: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1643: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1643) as *mut i32;
    let mut __slate_storage_1638: std::mem::MaybeUninit<u64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1638: *mut u64 = std::ptr::addr_of_mut!(__slate_storage_1638) as *mut u64;
    let mut __slate_storage_1637: std::mem::MaybeUninit<u64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1637: *mut u64 = std::ptr::addr_of_mut!(__slate_storage_1637) as *mut u64;
    let mut __slate_storage_1636: std::mem::MaybeUninit<*const u8> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1636: *mut *const u8 =
        std::ptr::addr_of_mut!(__slate_storage_1636) as *mut *const u8;
    let mut __slate_storage_1635: std::mem::MaybeUninit<*const u8> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1635: *mut *const u8 =
        std::ptr::addr_of_mut!(__slate_storage_1635) as *mut *const u8;
    let mut __slate_storage_1642: std::mem::MaybeUninit<u64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1642: *mut u64 = std::ptr::addr_of_mut!(__slate_storage_1642) as *mut u64;
    let mut __slate_storage_1641: std::mem::MaybeUninit<u64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1641: *mut u64 = std::ptr::addr_of_mut!(__slate_storage_1641) as *mut u64;
    let mut __slate_storage_1640: std::mem::MaybeUninit<*const u8> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1640: *mut *const u8 =
        std::ptr::addr_of_mut!(__slate_storage_1640) as *mut *const u8;
    let mut __slate_storage_1639: std::mem::MaybeUninit<*const u8> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1639: *mut *const u8 =
        std::ptr::addr_of_mut!(__slate_storage_1639) as *mut *const u8;
    let mut __slate_storage_1634: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1634: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_1634) as *mut u32;
    let mut __slate_storage_1633: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1633: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_1633) as *mut u32;
    let mut __slate_storage_1632: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1632: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_1632) as *mut u32;
    let mut __slate_storage_1629: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1629: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_1629) as *mut u32;
    let mut __slate_storage_1631: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1631: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_1631) as *mut u32;
    let mut __slate_storage_1630: std::mem::MaybeUninit<bool> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1630: *mut bool = std::ptr::addr_of_mut!(__slate_storage_1630) as *mut bool;
    let mut __slate_storage_792: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_792: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_792) as *mut u32; // PseudoTable input register
    let mut __slate_storage_791: std::mem::MaybeUninit<*mut sqlite3_value> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_791: *mut *mut sqlite3_value =
        std::ptr::addr_of_mut!(__slate_storage_791) as *mut *mut sqlite3_value; // A type code from the record header
    let mut __slate_storage_790: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_790: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_790) as *mut u32; // 64-bit offset
    let mut __slate_storage_789: std::mem::MaybeUninit<u64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_789: *mut u64 = std::ptr::addr_of_mut!(__slate_storage_789) as *mut u64; // Pointer to first byte after the header
    let mut __slate_storage_788: std::mem::MaybeUninit<*const u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_788: *mut *const u8 =
        std::ptr::addr_of_mut!(__slate_storage_788) as *mut *const u8; // Next unparsed byte of the header
    let mut __slate_storage_787: std::mem::MaybeUninit<*const u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_787: *mut *const u8 =
        std::ptr::addr_of_mut!(__slate_storage_787) as *mut *const u8; // Part of the record being decoded
    let mut __slate_storage_786: std::mem::MaybeUninit<*const u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_786: *mut *const u8 =
        std::ptr::addr_of_mut!(__slate_storage_786) as *mut *const u8; // For storing the record being decoded
    let mut __slate_storage_785: std::mem::MaybeUninit<sqlite3_value> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_785: *mut sqlite3_value =
        std::ptr::addr_of_mut!(__slate_storage_785) as *mut sqlite3_value; // Where to write the extracted value
    let mut __slate_storage_784: std::mem::MaybeUninit<*mut sqlite3_value> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_784: *mut *mut sqlite3_value =
        std::ptr::addr_of_mut!(__slate_storage_784) as *mut *mut sqlite3_value; // Loop counter
    let mut __slate_storage_783: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_783: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_783) as *mut i32; // The length of the serialized data for the column
    let mut __slate_storage_782: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_782: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_782) as *mut i32; // aOffset[i] is offset to start of data for i-th column
    let mut __slate_storage_781: std::mem::MaybeUninit<*mut u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_781: *mut *mut u32 =
        std::ptr::addr_of_mut!(__slate_storage_781) as *mut *mut u32; // The B-Tree cursor corresponding to pC
    let mut __slate_storage_780: std::mem::MaybeUninit<*mut BtCursor> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_780: *mut *mut BtCursor =
        std::ptr::addr_of_mut!(__slate_storage_780) as *mut *mut BtCursor; // The VDBE cursor
    let mut __slate_storage_779: std::mem::MaybeUninit<*mut VdbeCursor> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_779: *mut *mut VdbeCursor =
        std::ptr::addr_of_mut!(__slate_storage_779) as *mut *mut VdbeCursor; // column number to retrieve
    let mut __slate_storage_778: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_778: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_778) as *mut u32; // The VDBE cursor
    let mut __slate_storage_777: std::mem::MaybeUninit<*mut VdbeCursor> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_777: *mut *mut VdbeCursor =
        std::ptr::addr_of_mut!(__slate_storage_777) as *mut *mut VdbeCursor;
    let mut __slate_storage_776: std::mem::MaybeUninit<*mut VdbeCursor> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_776: *mut *mut VdbeCursor =
        std::ptr::addr_of_mut!(__slate_storage_776) as *mut *mut VdbeCursor;
    let mut __slate_storage_774: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_774: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_774) as *mut u32;
    let mut __slate_storage_773: std::mem::MaybeUninit<u16> = std::mem::MaybeUninit::uninit();
    let __slate_slot_773: *mut u16 = std::ptr::addr_of_mut!(__slate_storage_773) as *mut u16;
    let mut __slate_storage_772: std::mem::MaybeUninit<*mut VdbeCursor> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_772: *mut *mut VdbeCursor =
        std::ptr::addr_of_mut!(__slate_storage_772) as *mut *mut VdbeCursor;
    let mut __slate_storage_771: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_771: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_771) as *mut i32;
    let mut __slate_storage_770: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_770: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_770) as *mut i32;
    let mut __slate_storage_1628: std::mem::MaybeUninit<u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1628: *mut u8 = std::ptr::addr_of_mut!(__slate_storage_1628) as *mut u8;
    let mut __slate_storage_1627: std::mem::MaybeUninit<u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1627: *mut u8 = std::ptr::addr_of_mut!(__slate_storage_1627) as *mut u8;
    let mut __slate_storage_1626: std::mem::MaybeUninit<*mut u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1626: *mut *mut u8 =
        std::ptr::addr_of_mut!(__slate_storage_1626) as *mut *mut u8; // Address of this instruction
    let mut __slate_storage_769: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_769: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_769) as *mut u32; // Right operand: 0==FALSE, 1==TRUE, 2==UNKNOWN or NULL
    let mut __slate_storage_766: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_766: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_766) as *mut i32; // Left operand:  0==FALSE, 1==TRUE, 2==UNKNOWN or NULL
    let mut __slate_storage_765: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_765: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_765) as *mut i32;
    let mut __slate_storage_1625: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1625: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1625) as *mut i32;
    let mut __slate_storage_1624: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1624: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1624) as *mut i32; // The permutation
    let mut __slate_storage_764: std::mem::MaybeUninit<*mut u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_764: *mut *mut u32 =
        std::ptr::addr_of_mut!(__slate_storage_764) as *mut *mut u32; // True for DESCENDING sort order
    let mut __slate_storage_763: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_763: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_763) as *mut i32; // Collating sequence to use on this term
    let mut __slate_storage_762: std::mem::MaybeUninit<*mut CollSeq> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_762: *mut *mut CollSeq =
        std::ptr::addr_of_mut!(__slate_storage_762) as *mut *mut CollSeq;
    let mut __slate_storage_761: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_761: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_761) as *mut u32;
    let mut __slate_storage_760: std::mem::MaybeUninit<*const KeyInfo> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_760: *mut *const KeyInfo =
        std::ptr::addr_of_mut!(__slate_storage_760) as *mut *const KeyInfo;
    let mut __slate_storage_759: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_759: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_759) as *mut i32;
    let mut __slate_storage_758: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_758: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_758) as *mut i32;
    let mut __slate_storage_757: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_757: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_757) as *mut i32;
    let mut __slate_storage_756: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_756: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_756) as *mut i32;
    let mut __slate_storage_1623: std::mem::MaybeUninit<u16> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1623: *mut u16 = std::ptr::addr_of_mut!(__slate_storage_1623) as *mut u16;
    let mut __slate_storage_1622: std::mem::MaybeUninit<u16> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1622: *mut u16 = std::ptr::addr_of_mut!(__slate_storage_1622) as *mut u16;
    let mut __slate_storage_1621: std::mem::MaybeUninit<*mut sqlite3_value> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1621: *mut *mut sqlite3_value =
        std::ptr::addr_of_mut!(__slate_storage_1621) as *mut *mut sqlite3_value;
    let mut __slate_storage_1620: std::mem::MaybeUninit<u16> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1620: *mut u16 = std::ptr::addr_of_mut!(__slate_storage_1620) as *mut u16;
    let mut __slate_storage_1619: std::mem::MaybeUninit<u16> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1619: *mut u16 = std::ptr::addr_of_mut!(__slate_storage_1619) as *mut u16;
    let mut __slate_storage_1618: std::mem::MaybeUninit<*mut sqlite3_value> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1618: *mut *mut sqlite3_value =
        std::ptr::addr_of_mut!(__slate_storage_1618) as *mut *mut sqlite3_value; // Copy of initial value of pIn3->flags
    let mut __slate_storage_755: std::mem::MaybeUninit<u16> = std::mem::MaybeUninit::uninit();
    let __slate_slot_755: *mut u16 = std::ptr::addr_of_mut!(__slate_storage_755) as *mut u16; // Copy of initial value of pIn1->flags
    let mut __slate_storage_754: std::mem::MaybeUninit<u16> = std::mem::MaybeUninit::uninit();
    let __slate_slot_754: *mut u16 = std::ptr::addr_of_mut!(__slate_storage_754) as *mut u16; // Affinity to use for comparison
    let mut __slate_storage_753: std::mem::MaybeUninit<i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_753: *mut i8 = std::ptr::addr_of_mut!(__slate_storage_753) as *mut i8; // Result of the comparison of pIn1 against pIn3
    let mut __slate_storage_752: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_752: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_752) as *mut i32;
    let mut __slate_storage_751: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_751: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_751) as *mut i32;
    let mut __slate_storage_1617: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1617: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1617) as *mut i32;
    let mut __slate_storage_1616: std::mem::MaybeUninit<u64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1616: *mut u64 = std::ptr::addr_of_mut!(__slate_storage_1616) as *mut u64;
    let mut __slate_storage_1615: std::mem::MaybeUninit<u64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1615: *mut u64 = std::ptr::addr_of_mut!(__slate_storage_1615) as *mut u64;
    let mut __slate_storage_1614: std::mem::MaybeUninit<*mut u64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1614: *mut *mut u64 =
        std::ptr::addr_of_mut!(__slate_storage_1614) as *mut *mut u64;
    let mut __slate_storage_1605: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1605: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_1605) as *mut i64;
    let mut __slate_storage_1604: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1604: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_1604) as *mut i64;
    let mut __slate_storage_1607: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1607: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_1607) as *mut i64;
    let mut __slate_storage_1606: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1606: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_1606) as *mut i64;
    let mut __slate_storage_1609: std::mem::MaybeUninit<u64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1609: *mut u64 = std::ptr::addr_of_mut!(__slate_storage_1609) as *mut u64;
    let mut __slate_storage_1608: std::mem::MaybeUninit<u64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1608: *mut u64 = std::ptr::addr_of_mut!(__slate_storage_1608) as *mut u64;
    let mut __slate_storage_1613: std::mem::MaybeUninit<u64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1613: *mut u64 = std::ptr::addr_of_mut!(__slate_storage_1613) as *mut u64;
    let mut __slate_storage_1612: std::mem::MaybeUninit<u64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1612: *mut u64 = std::ptr::addr_of_mut!(__slate_storage_1612) as *mut u64;
    let mut __slate_storage_1611: std::mem::MaybeUninit<u64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1611: *mut u64 = std::ptr::addr_of_mut!(__slate_storage_1611) as *mut u64;
    let mut __slate_storage_1610: std::mem::MaybeUninit<u64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1610: *mut u64 = std::ptr::addr_of_mut!(__slate_storage_1610) as *mut u64;
    let mut __slate_storage_750: std::mem::MaybeUninit<u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_750: *mut u8 = std::ptr::addr_of_mut!(__slate_storage_750) as *mut u8;
    let mut __slate_storage_749: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_749: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_749) as *mut i64;
    let mut __slate_storage_748: std::mem::MaybeUninit<u64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_748: *mut u64 = std::ptr::addr_of_mut!(__slate_storage_748) as *mut u64;
    let mut __slate_storage_747: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_747: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_747) as *mut i64;
    let mut __slate_storage_1595: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1595: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_1595) as *mut i64;
    let mut __slate_storage_1594: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1594: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_1594) as *mut i64;
    let mut __slate_storage_1593: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1593: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_1593) as *mut i64;
    let mut __slate_storage_1592: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1592: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_1592) as *mut i64;
    let mut __slate_storage_1603: std::mem::MaybeUninit<f64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1603: *mut f64 = std::ptr::addr_of_mut!(__slate_storage_1603) as *mut f64;
    let mut __slate_storage_1602: std::mem::MaybeUninit<f64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1602: *mut f64 = std::ptr::addr_of_mut!(__slate_storage_1602) as *mut f64;
    let mut __slate_storage_1601: std::mem::MaybeUninit<f64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1601: *mut f64 = std::ptr::addr_of_mut!(__slate_storage_1601) as *mut f64;
    let mut __slate_storage_1600: std::mem::MaybeUninit<f64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1600: *mut f64 = std::ptr::addr_of_mut!(__slate_storage_1600) as *mut f64;
    let mut __slate_storage_1599: std::mem::MaybeUninit<f64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1599: *mut f64 = std::ptr::addr_of_mut!(__slate_storage_1599) as *mut f64;
    let mut __slate_storage_1598: std::mem::MaybeUninit<f64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1598: *mut f64 = std::ptr::addr_of_mut!(__slate_storage_1598) as *mut f64;
    let mut __slate_storage_1597: std::mem::MaybeUninit<f64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1597: *mut f64 = std::ptr::addr_of_mut!(__slate_storage_1597) as *mut f64;
    let mut __slate_storage_1596: std::mem::MaybeUninit<f64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1596: *mut f64 = std::ptr::addr_of_mut!(__slate_storage_1596) as *mut f64; // Real value of right operand
    let mut __slate_storage_746: std::mem::MaybeUninit<f64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_746: *mut f64 = std::ptr::addr_of_mut!(__slate_storage_746) as *mut f64; // Real value of left operand
    let mut __slate_storage_745: std::mem::MaybeUninit<f64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_745: *mut f64 = std::ptr::addr_of_mut!(__slate_storage_745) as *mut f64; // Integer value of right operand
    let mut __slate_storage_744: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_744: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_744) as *mut i64; // Integer value of left operand
    let mut __slate_storage_743: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_743: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_743) as *mut i64; // Numeric type of right operand
    let mut __slate_storage_742: std::mem::MaybeUninit<u16> = std::mem::MaybeUninit::uninit();
    let __slate_slot_742: *mut u16 = std::ptr::addr_of_mut!(__slate_storage_742) as *mut u16; // Numeric type of left operand
    let mut __slate_storage_741: std::mem::MaybeUninit<u16> = std::mem::MaybeUninit::uninit();
    let __slate_slot_741: *mut u16 = std::ptr::addr_of_mut!(__slate_storage_741) as *mut u16;
    let mut __slate_storage_1591: std::mem::MaybeUninit<u16> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1591: *mut u16 = std::ptr::addr_of_mut!(__slate_storage_1591) as *mut u16;
    let mut __slate_storage_1590: std::mem::MaybeUninit<u16> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1590: *mut u16 = std::ptr::addr_of_mut!(__slate_storage_1590) as *mut u16;
    let mut __slate_storage_1589: std::mem::MaybeUninit<*mut sqlite3_value> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1589: *mut *mut sqlite3_value =
        std::ptr::addr_of_mut!(__slate_storage_1589) as *mut *mut sqlite3_value;
    let mut __slate_storage_1588: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1588: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_1588) as *mut i64;
    let mut __slate_storage_1587: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1587: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_1587) as *mut i64;
    let mut __slate_storage_1586: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1586: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_1586) as *mut i64;
    let mut __slate_storage_1585: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1585: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_1585) as *mut i64; // Initial flags for P2
    let mut __slate_storage_740: std::mem::MaybeUninit<u16> = std::mem::MaybeUninit::uninit();
    let __slate_slot_740: *mut u16 = std::ptr::addr_of_mut!(__slate_storage_740) as *mut u16; // Initial flags for P1
    let mut __slate_storage_739: std::mem::MaybeUninit<u16> = std::mem::MaybeUninit::uninit();
    let __slate_slot_739: *mut u16 = std::ptr::addr_of_mut!(__slate_storage_739) as *mut u16; // Total size of the output string or blob
    let mut __slate_storage_738: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_738: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_738) as *mut i64;
    let mut __slate_storage_1584: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1584: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1584) as *mut i32;
    let mut __slate_storage_1583: std::mem::MaybeUninit<*mut sqlite3_value> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1583: *mut *mut sqlite3_value =
        std::ptr::addr_of_mut!(__slate_storage_1583) as *mut *mut sqlite3_value;
    let mut __slate_storage_1582: std::mem::MaybeUninit<*mut sqlite3_value> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1582: *mut *mut sqlite3_value =
        std::ptr::addr_of_mut!(__slate_storage_1582) as *mut *mut sqlite3_value;
    let mut __slate_storage_1581: std::mem::MaybeUninit<*mut sqlite3_value> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1581: *mut *mut sqlite3_value =
        std::ptr::addr_of_mut!(__slate_storage_1581) as *mut *mut sqlite3_value;
    let mut __slate_storage_1580: std::mem::MaybeUninit<*mut sqlite3_value> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1580: *mut *mut sqlite3_value =
        std::ptr::addr_of_mut!(__slate_storage_1580) as *mut *mut sqlite3_value;
    let mut __slate_storage_1579: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1579: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1579) as *mut i32;
    let mut __slate_storage_1578: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1578: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1578) as *mut i32;
    let mut __slate_storage_1577: std::mem::MaybeUninit<u16> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1577: *mut u16 = std::ptr::addr_of_mut!(__slate_storage_1577) as *mut u16;
    let mut __slate_storage_1576: std::mem::MaybeUninit<u16> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1576: *mut u16 = std::ptr::addr_of_mut!(__slate_storage_1576) as *mut u16;
    let mut __slate_storage_1575: std::mem::MaybeUninit<*mut sqlite3_value> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1575: *mut *mut sqlite3_value =
        std::ptr::addr_of_mut!(__slate_storage_1575) as *mut *mut sqlite3_value;
    let mut __slate_storage_1574: std::mem::MaybeUninit<bool> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1574: *mut bool = std::ptr::addr_of_mut!(__slate_storage_1574) as *mut bool;
    let mut __slate_storage_737: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_737: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_737) as *mut i32;
    let mut __slate_storage_1573: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1573: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1573) as *mut i32;
    let mut __slate_storage_1572: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1572: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1572) as *mut i32;
    let mut __slate_storage_1571: std::mem::MaybeUninit<*mut sqlite3_value> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1571: *mut *mut sqlite3_value =
        std::ptr::addr_of_mut!(__slate_storage_1571) as *mut *mut sqlite3_value;
    let mut __slate_storage_1570: std::mem::MaybeUninit<*mut sqlite3_value> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1570: *mut *mut sqlite3_value =
        std::ptr::addr_of_mut!(__slate_storage_1570) as *mut *mut sqlite3_value;
    let mut __slate_storage_1569: std::mem::MaybeUninit<*mut sqlite3_value> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1569: *mut *mut sqlite3_value =
        std::ptr::addr_of_mut!(__slate_storage_1569) as *mut *mut sqlite3_value;
    let mut __slate_storage_1568: std::mem::MaybeUninit<*mut sqlite3_value> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1568: *mut *mut sqlite3_value =
        std::ptr::addr_of_mut!(__slate_storage_1568) as *mut *mut sqlite3_value;
    let mut __slate_storage_1567: std::mem::MaybeUninit<bool> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1567: *mut bool = std::ptr::addr_of_mut!(__slate_storage_1567) as *mut bool; // Register to copy to
    let mut __slate_storage_736: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_736: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_736) as *mut i32; // Register to copy from
    let mut __slate_storage_735: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_735: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_735) as *mut i32; // Number of registers left to copy
    let mut __slate_storage_734: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_734: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_734) as *mut i32;
    let mut __slate_storage_1566: std::mem::MaybeUninit<u16> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1566: *mut u16 = std::ptr::addr_of_mut!(__slate_storage_1566) as *mut u16;
    let mut __slate_storage_1565: std::mem::MaybeUninit<u16> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1565: *mut u16 = std::ptr::addr_of_mut!(__slate_storage_1565) as *mut u16;
    let mut __slate_storage_1564: std::mem::MaybeUninit<*mut sqlite3_value> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1564: *mut *mut sqlite3_value =
        std::ptr::addr_of_mut!(__slate_storage_1564) as *mut *mut sqlite3_value;
    let mut __slate_storage_1563: std::mem::MaybeUninit<u16> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1563: *mut u16 = std::ptr::addr_of_mut!(__slate_storage_1563) as *mut u16;
    let mut __slate_storage_1562: std::mem::MaybeUninit<u16> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1562: *mut u16 = std::ptr::addr_of_mut!(__slate_storage_1562) as *mut u16;
    let mut __slate_storage_1561: std::mem::MaybeUninit<*mut sqlite3_value> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1561: *mut *mut sqlite3_value =
        std::ptr::addr_of_mut!(__slate_storage_1561) as *mut *mut sqlite3_value; // Value being transferred
    let mut __slate_storage_733: std::mem::MaybeUninit<*mut sqlite3_value> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_733: *mut *mut sqlite3_value =
        std::ptr::addr_of_mut!(__slate_storage_733) as *mut *mut sqlite3_value;
    let mut __slate_storage_1560: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1560: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1560) as *mut i32;
    let mut __slate_storage_1559: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1559: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1559) as *mut i32;
    let mut __slate_storage_1558: std::mem::MaybeUninit<*mut sqlite3_value> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1558: *mut *mut sqlite3_value =
        std::ptr::addr_of_mut!(__slate_storage_1558) as *mut *mut sqlite3_value;
    let mut __slate_storage_1557: std::mem::MaybeUninit<*mut sqlite3_value> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1557: *mut *mut sqlite3_value =
        std::ptr::addr_of_mut!(__slate_storage_1557) as *mut *mut sqlite3_value;
    let mut __slate_storage_1556: std::mem::MaybeUninit<u16> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1556: *mut u16 = std::ptr::addr_of_mut!(__slate_storage_1556) as *mut u16;
    let mut __slate_storage_732: std::mem::MaybeUninit<u16> = std::mem::MaybeUninit::uninit();
    let __slate_slot_732: *mut u16 = std::ptr::addr_of_mut!(__slate_storage_732) as *mut u16;
    let mut __slate_storage_731: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_731: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_731) as *mut i32;
    let mut __slate_storage_1555: std::mem::MaybeUninit<u16> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1555: *mut u16 = std::ptr::addr_of_mut!(__slate_storage_1555) as *mut u16;
    let mut __slate_storage_1554: std::mem::MaybeUninit<u16> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1554: *mut u16 = std::ptr::addr_of_mut!(__slate_storage_1554) as *mut u16;
    let mut __slate_storage_1553: std::mem::MaybeUninit<*mut sqlite3_value> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1553: *mut *mut sqlite3_value =
        std::ptr::addr_of_mut!(__slate_storage_1553) as *mut *mut sqlite3_value;
    let mut __slate_storage_730: std::mem::MaybeUninit<u64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_730: *mut u64 = std::ptr::addr_of_mut!(__slate_storage_730) as *mut u64;
    let mut __slate_storage_728: std::mem::MaybeUninit<*const i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_728: *mut *const i8 =
        std::ptr::addr_of_mut!(__slate_storage_728) as *mut *const i8;
    let mut __slate_storage_1552: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1552: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1552) as *mut i32;
    let mut __slate_storage_1551: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1551: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1551) as *mut i32;
    let mut __slate_storage_1550: std::mem::MaybeUninit<*mut Vdbe> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1550: *mut *mut Vdbe =
        std::ptr::addr_of_mut!(__slate_storage_1550) as *mut *mut Vdbe;
    let mut __slate_storage_727: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_727: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_727) as *mut i32;
    let mut __slate_storage_726: std::mem::MaybeUninit<*mut VdbeFrame> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_726: *mut *mut VdbeFrame =
        std::ptr::addr_of_mut!(__slate_storage_726) as *mut *mut VdbeFrame;
    let mut __slate_storage_725: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_725: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_725) as *mut i32;
    let mut __slate_storage_724: std::mem::MaybeUninit<*mut VdbeOp> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_724: *mut *mut VdbeOp =
        std::ptr::addr_of_mut!(__slate_storage_724) as *mut *mut VdbeOp;
    let mut __slate_storage_1549: std::mem::MaybeUninit<u64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1549: *mut u64 = std::ptr::addr_of_mut!(__slate_storage_1549) as *mut u64;
    let mut __slate_storage_1548: std::mem::MaybeUninit<u64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1548: *mut u64 = std::ptr::addr_of_mut!(__slate_storage_1548) as *mut u64;
    let mut __slate_storage_1547: std::mem::MaybeUninit<u64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1547: *mut u64 = std::ptr::addr_of_mut!(__slate_storage_1547) as *mut u64;
    let mut __slate_storage_1546: std::mem::MaybeUninit<u64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1546: *mut u64 = std::ptr::addr_of_mut!(__slate_storage_1546) as *mut u64;
    let mut __slate_storage_723: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_723: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_723) as *mut u32; // Column cache counter
    let mut __slate_storage_722: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_722: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_722) as *mut u32; // Output operand
    let mut __slate_storage_721: std::mem::MaybeUninit<*mut sqlite3_value> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_721: *mut *mut sqlite3_value =
        std::ptr::addr_of_mut!(__slate_storage_721) as *mut *mut sqlite3_value; // 3rd input operand
    let mut __slate_storage_720: std::mem::MaybeUninit<*mut sqlite3_value> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_720: *mut *mut sqlite3_value =
        std::ptr::addr_of_mut!(__slate_storage_720) as *mut *mut sqlite3_value; // 2nd input operand
    let mut __slate_storage_719: std::mem::MaybeUninit<*mut sqlite3_value> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_719: *mut *mut sqlite3_value =
        std::ptr::addr_of_mut!(__slate_storage_719) as *mut *mut sqlite3_value; // 1st input operand
    let mut __slate_storage_718: std::mem::MaybeUninit<*mut sqlite3_value> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_718: *mut *mut sqlite3_value =
        std::ptr::addr_of_mut!(__slate_storage_718) as *mut *mut sqlite3_value; // Copy of p->aMem
    let mut __slate_storage_717: std::mem::MaybeUninit<*mut sqlite3_value> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_717: *mut *mut sqlite3_value =
        std::ptr::addr_of_mut!(__slate_storage_717) as *mut *mut sqlite3_value; // Invoke xProgress() when nVmStep reaches this
    let mut __slate_storage_716: std::mem::MaybeUninit<u64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_716: *mut u64 = std::ptr::addr_of_mut!(__slate_storage_716) as *mut u64; // Number of virtual machine steps
    let mut __slate_storage_715: std::mem::MaybeUninit<u64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_715: *mut u64 = std::ptr::addr_of_mut!(__slate_storage_715) as *mut u64; // Result of last comparison
    let mut __slate_storage_714: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_714: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_714) as *mut i32; // The database encoding
    let mut __slate_storage_713: std::mem::MaybeUninit<u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_713: *mut u8 = std::ptr::addr_of_mut!(__slate_storage_713) as *mut u8; // Reset schema after an error if positive
    let mut __slate_storage_712: std::mem::MaybeUninit<u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_712: *mut u8 = std::ptr::addr_of_mut!(__slate_storage_712) as *mut u8; // The database
    let mut __slate_storage_711: std::mem::MaybeUninit<*mut sqlite3> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_711: *mut *mut sqlite3 =
        std::ptr::addr_of_mut!(__slate_storage_711) as *mut *mut sqlite3; // Value to return
    let mut __slate_storage_710: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_710: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_710) as *mut i32; // Current operation
    let mut __slate_storage_709: std::mem::MaybeUninit<*mut VdbeOp> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_709: *mut *mut VdbeOp =
        std::ptr::addr_of_mut!(__slate_storage_709) as *mut *mut VdbeOp; // Copy of p->aOp
    let mut __slate_storage_708: std::mem::MaybeUninit<*mut VdbeOp> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_708: *mut *mut VdbeOp =
        std::ptr::addr_of_mut!(__slate_storage_708) as *mut *mut VdbeOp;
    let mut __slate_state: usize = 1151usize;
    unsafe {
        '__join_1149: {
            std::ptr::write(__slate_slot_708, unsafe { (*p).aOp });
            std::ptr::write(__slate_slot_709, *__slate_slot_708);
            std::ptr::write(__slate_slot_710, 0 as i32);
            std::ptr::write(__slate_slot_711, unsafe { (*p).db });
            std::ptr::write(__slate_slot_712, ((0 as i32) as i8) as u8);
            std::ptr::write(__slate_slot_713, unsafe { (*(*__slate_slot_711)).enc });
            std::ptr::write(__slate_slot_714, 0 as i32);
            std::ptr::write(__slate_slot_715, ((0 as i32) as i64) as u64);
            std::ptr::write(__slate_slot_717, unsafe { (*p).aMem });
            std::ptr::write(__slate_slot_718, std::ptr::null_mut::<sqlite3_value>());
            std::ptr::write(__slate_slot_719, std::ptr::null_mut::<sqlite3_value>());
            std::ptr::write(__slate_slot_720, std::ptr::null_mut::<sqlite3_value>());
            std::ptr::write(__slate_slot_721, std::ptr::null_mut::<sqlite3_value>());
            std::ptr::write(__slate_slot_722, (0 as i32) as u32);
            // INSERT STACK UNION HERE
            0 as i32; // sqlite3_step() verifies this
            if (unsafe { (*p).lockMask }) != ((0 as i32) as u32) {
                unsafe { sqlite3VdbeEnter(p) };
            }
        }
        if (unsafe { (*(*__slate_slot_711)).xProgress }) != None {
            std::ptr::write(__slate_slot_723, unsafe {
                *unsafe {
                    unsafe { (*p).aCounter.as_mut_ptr() as *mut u32 }.offset((4 as i32) as isize)
                }
            });
            0 as i32;
            *__slate_slot_716 = unsafe { (*(*__slate_slot_711)).nProgressOps }
                .wrapping_sub(*__slate_slot_723 % unsafe { (*(*__slate_slot_711)).nProgressOps })
                as u64;
        } else {
            *__slate_slot_716 =
                ((4294967295 as u32) as u64) | ((4294967295 as u32) as u64) << (32 as i32);
        }
        '__join_1152: {
            '__join_1: {
                if (unsafe { (*p).rc }) == (7 as i32) {
                    // This happens if a malloc() inside a call to sqlite3_column_text() or
                    // sqlite3_column_text16() failed.
                } else {
                    '__join_0: {
                        0 as i32;
                        {}
                        unsafe {
                            (*p).rc = 0 as i32;
                        }
                        0 as i32;
                        unsafe {
                            (*p).iCurrentTime = (0 as i32) as i64;
                        }
                        0 as i32;
                        unsafe {
                            (*(*__slate_slot_711)).busyHandler.nBusy = 0 as i32;
                        }
                        if (unsafe {
                            std::sync::atomic::AtomicI32::load_volatile(
                                std::sync::atomic::AtomicI32::from_ptr_raw(
                                    (unsafe {
                                        std::ptr::addr_of_mut!(
                                            (*(*__slate_slot_711)).u1.isInterrupted
                                        )
                                    }) as *mut i32,
                                ),
                                std::sync::atomic::Ordering::Relaxed,
                            )
                        }) != (0 as i32)
                        {
                        } else {
                            {}
                            *__slate_slot_709 = unsafe {
                                (*__slate_slot_708).offset((unsafe { (*p).pc }) as isize)
                            };
                            '__join_1136: {
                                '__join_1116: {
                                    '__join_2: {
                                        '__join_1271: {
                                            '__join_1060: {
                                                '__join_984: {
                                                    '__join_1268: {
                                                        '__join_1267: {
                                                            '__join_1266: {
                                                                '__join_788: {
                                                                    '__join_1262: {
                                                                        '__join_810: {
                                                                            '__join_1263: {
                                                                                '__join_1264: {
                                                                                    '__join_1265: {
                                                                                        '__join_764: {
                                                                                            '__join_1258: {
                                                                                                '__join_1250: {
                                                                                                    '__join_630: {
                                                                                                        '__join_1251: {
                                                                                                            '__join_1255: {
                                                                                                                '__join_656: {
                                                                                                                    '__join_1253: {
                                                                                                                        '__join_1252: {
                                                                                                                            '__join_1257: {
                                                                                                                                '__join_1256: {
                                                                                                                                    '__join_629: {
                                                                                                                                        '__join_615: {
                                                                                                                                            '__join_610: {
                                                                                                                                                '__join_1238: {
                                                                                                                                                    '__join_1237: {
                                                                                                                                                        '__join_1235: {
                                                                                                                                                            '__join_585: {
                                                                                                                                                                '__join_1234: {
                                                                                                                                                                    '__join_1233: {
                                                                                                                                                                        '__join_1231: {
                                                                                                                                                                            '__join_1230: {
                                                                                                                                                                                '__join_1228: {
                                                                                                                                                                                    '__join_1229: {
                                                                                                                                                                                        '__join_1232: {
                                                                                                                                                                                            '__join_1227: {
                                                                                                                                                                                                '__join_1226: {
                                                                                                                                                                                                    '__join_1225: {
                                                                                                                                                                                                        '__join_1224: {
                                                                                                                                                                                                            '__join_1223: {
                                                                                                                                                                                                                '__join_439: {
                                                                                                                                                                                                                    '__join_1221: {
                                                                                                                                                                                                                        '__join_427: {
                                                                                                                                                                                                                            '__join_1219: {
                                                                                                                                                                                                                                '__join_1218: {
                                                                                                                                                                                                                                    '__join_1217: {
                                                                                                                                                                                                                                        '__join_1216: {
                                                                                                                                                                                                                                            '__join_1215: {
                                                                                                                                                                                                                                                '__join_1214: {
                                                                                                                                                                                                                                                    '__join_1212: {
                                                                                                                                                                                                                                                        '__join_1213: {
                                                                                                                                                                                                                                                            '__join_1211: {
                                                                                                                                                                                                                                                                '__join_1210: {
                                                                                                                                                                                                                                                                    '__join_1209: {
                                                                                                                                                                                                                                                                        '__join_1208: {
                                                                                                                                                                                                                                                                            '__join_1207: {
                                                                                                                                                                                                                                                                                '__join_1206: {
                                                                                                                                                                                                                                                                                    '__join_1205: {
                                                                                                                                                                                                                                                                                        '__join_1204: {
                                                                                                                                                                                                                                                                                            '__join_1203: {
                                                                                                                                                                                                                                                                                                '__join_1202: {
                                                                                                                                                                                                                                                                                                    '__join_1201: {
                                                                                                                                                                                                                                                                                                        '__join_331: {
                                                                                                                                                                                                                                                                                                            '__join_1199: {
                                                                                                                                                                                                                                                                                                                '__join_1198: {
                                                                                                                                                                                                                                                                                                                    '__join_1197: {
                                                                                                                                                                                                                                                                                                                        '__join_1196: {
                                                                                                                                                                                                                                                                                                                            '__join_1195: {
                                                                                                                                                                                                                                                                                                                                '__join_308: {
                                                                                                                                                                                                                                                                                                                                    '__join_1193: {
                                                                                                                                                                                                                                                                                                                                        '__join_298: {
                                                                                                                                                                                                                                                                                                                                            '__join_1192: {
                                                                                                                                                                                                                                                                                                                                                '__join_1190: {
                                                                                                                                                                                                                                                                                                                                                    '__join_1189: {
                                                                                                                                                                                                                                                                                                                                                        '__join_1188: {
                                                                                                                                                                                                                                                                                                                                                            '__join_283: {
                                                                                                                                                                                                                                                                                                                                                                '__join_272: {
                                                                                                                                                                                                                                                                                                                                                                    '__join_1185: {
                                                                                                                                                                                                                                                                                                                                                                        '__join_264: {
                                                                                                                                                                                                                                                                                                                                                                            '__join_237: {
                                                                                                                                                                                                                                                                                                                                                                                '__join_1182: {
                                                                                                                                                                                                                                                                                                                                                                                    '__join_174: {
                                                                                                                                                                                                                                                                                                                                                                                        '__join_1180: {
                                                                                                                                                                                                                                                                                                                                                                                            '__join_145: {
                                                                                                                                                                                                                                                                                                                                                                                                '__join_1178: {
                                                                                                                                                                                                                                                                                                                                                                                                    '__join_1177: {
                                                                                                                                                                                                                                                                                                                                                                                                        '__join_1176: {
                                                                                                                                                                                                                                                                                                                                                                                                            '__join_130: {
                                                                                                                                                                                                                                                                                                                                                                                                                '__join_1173: {
                                                                                                                                                                                                                                                                                                                                                                                                                    '__join_1172: {
                                                                                                                                                                                                                                                                                                                                                                                                                        '__join_1171: {
                                                                                                                                                                                                                                                                                                                                                                                                                            '__join_118: {
                                                                                                                                                                                                                                                                                                                                                                                                                                '__join_1169: {
                                                                                                                                                                                                                                                                                                                                                                                                                                    '__join_115: {
                                                                                                                                                                                                                                                                                                                                                                                                                                        '__join_111: {
                                                                                                                                                                                                                                                                                                                                                                                                                                            '__join_1167: {
                                                                                                                                                                                                                                                                                                                                                                                                                                                '__join_1166: {
                                                                                                                                                                                                                                                                                                                                                                                                                                                    '__join_1165: {
                                                                                                                                                                                                                                                                                                                                                                                                                                                        '__join_1164: {
                                                                                                                                                                                                                                                                                                                                                                                                                                                            '__join_1163: {
                                                                                                                                                                                                                                                                                                                                                                                                                                                                '__join_83: {
                                                                                                                                                                                                                                                                                                                                                                                                                                                                    '__join_1161: {
                                                                                                                                                                                                                                                                                                                                                                                                                                                                        '__join_1160: {
                                                                                                                                                                                                                                                                                                                                                                                                                                                                            loop {
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                if (1 as i32) != (0 as i32) {
'__join_26: {
'__join_1139: {
'__join_1140: {
'__join_1125: {
'__join_1120: {
'__join_1094: {
'__join_1091: {
'__join_1035: {
'__join_1005: {
'__join_974: {
'__join_912: {
'__join_572: {
'__join_586: {
'__join_569: {
'__join_546: {
'__join_491: {
'__join_459: {
'__join_460: {
'__join_374: {
'__join_363: {
'__join_364: {
'__join_352: {
'__join_321: {
'__join_312: {
'__join_194: {
'__join_196: {
'__join_178: {
'__join_65: {
// Errors are detected by individual opcodes, with an immediate
// jumps to abort_due_to_error.
0 as i32;
0 as i32;
std::ptr::write(__slate_slot_1546, *__slate_slot_715);
std::ptr::write(__slate_slot_1547, (*__slate_slot_1546).wrapping_add(((1 as i32) as i64) as u64));
*__slate_slot_715 = *__slate_slot_1547;
// Only allow tracing if SQLITE_DEBUG is defined.
// Check to see if we need to simulate an interrupt.  This only happens
// if we have a special test build.
// Sanity checking on other operands
let __t4: i32 = ((unsafe { (*(*__slate_slot_709)).opcode }) as u32) as i32;
if __t4 == (9 as i32) {
// jump
break '__join_1140;
} else {
if __t4 == (10 as i32) {
// jump
0 as i32;
*__slate_slot_718 = unsafe { (*__slate_slot_717).offset((unsafe { (*(*__slate_slot_709)).p1 }) as isize) };
0 as i32;
{
}
unsafe {
(*(*__slate_slot_718)).flags = ((4 as i32) as i16) as u16;
}
unsafe {
(*(*__slate_slot_718)).u.i = (((unsafe { (*__slate_slot_709).offset_from(*__slate_slot_708 as *mut VdbeOp) }) as i64) as i32) as i64;
}
{
}
break '__join_1140;
} else {
if __t4 == (69 as i32) {
// in1
*__slate_slot_718 = unsafe { (*__slate_slot_717).offset((unsafe { (*(*__slate_slot_709)).p1 }) as isize) };
if (((unsafe { (*(*__slate_slot_718)).flags }) as u32) as i32) & (4 as i32) != (0 as i32) {
'__join_1128: {
if (unsafe { (*(*__slate_slot_709)).p3 }) != (0 as i32) {
{
}
}
}
*__slate_slot_709 = unsafe { (*__slate_slot_708).offset((unsafe { (*(*__slate_slot_718)).u.i }) as isize) };
break '__join_26;
} else {
if (unsafe { (*(*__slate_slot_709)).p3 }) != (0 as i32) {
{
}
break '__join_26;
} else {
break '__join_26;
}
}
} else {
if __t4 == (11 as i32) {
// jump0
0 as i32;
0 as i32;
0 as i32;
*__slate_slot_721 = unsafe { (*__slate_slot_717).offset((unsafe { (*(*__slate_slot_709)).p1 }) as isize) };
0 as i32;
unsafe {
(*(*__slate_slot_721)).u.i = ((unsafe { (*(*__slate_slot_709)).p3 }) - (1 as i32)) as i64;
}
unsafe {
(*(*__slate_slot_721)).flags = ((4 as i32) as i16) as u16;
}
if (unsafe { (*(*__slate_slot_709)).p2 }) == (0 as i32) {
break '__join_26;
} else {
// Most jump operations do a goto to this spot in order to update
// the pOp pointer.
break '__join_1125;
}
} else {
if __t4 == (70 as i32) {
// in1
*__slate_slot_718 = unsafe { (*__slate_slot_717).offset((unsafe { (*(*__slate_slot_709)).p1 }) as isize) };
0 as i32;
0 as i32;
*__slate_slot_724 = unsafe { (*__slate_slot_708).offset((unsafe { (*(*__slate_slot_718)).u.i }) as isize) };
0 as i32;
0 as i32;
unsafe {
(*(*__slate_slot_718)).u.i = ((((unsafe { (*__slate_slot_709).offset_from((unsafe { (*p).aOp }) as *mut VdbeOp) }) as i64) as i32) - (1 as i32)) as i64;
}
*__slate_slot_709 = unsafe { (*__slate_slot_708).offset(((unsafe { (*(*__slate_slot_724)).p2 }) - (1 as i32)) as isize) };
break '__join_26;
} else {
if __t4 == (12 as i32) {
// in1, jump0
*__slate_slot_718 = unsafe { (*__slate_slot_717).offset((unsafe { (*(*__slate_slot_709)).p1 }) as isize) };
0 as i32;
unsafe {
(*(*__slate_slot_718)).flags = ((4 as i32) as i16) as u16;
}
*__slate_slot_725 = (unsafe { (*(*__slate_slot_718)).u.i }) as i32;
unsafe {
(*(*__slate_slot_718)).u.i = (((unsafe { (*__slate_slot_709).offset_from(*__slate_slot_708 as *mut VdbeOp) }) as i64) as i32) as i64;
}
{
}
*__slate_slot_709 = unsafe { (*__slate_slot_708).offset(*__slate_slot_725 as isize) };
break '__join_26;
} else {
if __t4 == (71 as i32) {
// in3
*__slate_slot_720 = unsafe { (*__slate_slot_717).offset((unsafe { (*(*__slate_slot_709)).p3 }) as isize) };
if (((unsafe { (*(*__slate_slot_720)).flags }) as u32) as i32) & (1 as i32) == (0 as i32) {
break '__join_26;
} else {
// Fall through into OP_Halt
//
// no break
{
}
// Opcode:  Halt P1 P2 P3 P4 P5
//
// Exit immediately.  All open cursors, etc are closed
// automatically.
//
// P1 is the result code returned by sqlite3_exec(), sqlite3_reset(),
// or sqlite3_finalize().  For a normal halt, this should be SQLITE_OK (0).
// For errors, it can be some other value.  If P1!=0 then P2 will determine
// whether or not to rollback the current transaction.  Do not rollback
// if P2==OE_Fail. Do the rollback if P2==OE_Rollback.  If P2==OE_Abort,
// then back out all changes that have occurred during this execution of the
// VDBE, but do not rollback the transaction.
//
// If P3 is not zero and P4 is NULL, then P3 is a register that holds the
// text of an error message.
//
// If P3 is zero and P4 is not null then the error message string is held
// in P4.
//
// P5 is a value between 1 and 4, inclusive, then the P4 error message
// string is modified as follows:
//
//    1:  NOT NULL constraint failed: P4
//    2:  UNIQUE constraint failed: P4
//    3:  CHECK constraint failed: P4
//    4:  FOREIGN KEY constraint failed: P4
//
// If P3 is zero and P5 is not zero and P4 is NULL, then everything after
// the ":" is omitted.
//
// There is an implied "Halt 0 0 0" instruction inserted at the very end of
// every program.  So a jump past the last instruction of the program
// is the same as executing Halt.
break '__join_1120;
}
} else {
if __t4 == (72 as i32) {
break '__join_1120;
} else {
if __t4 == (73 as i32) {
// out2
*__slate_slot_721 = out2Prerelease(p, *__slate_slot_709);
unsafe {
(*(*__slate_slot_721)).u.i = (unsafe { (*(*__slate_slot_709)).p1 }) as i64;
}
break '__join_26;
} else {
if __t4 == (74 as i32) {
// out2
*__slate_slot_721 = out2Prerelease(p, *__slate_slot_709);
unsafe {
(*(*__slate_slot_721)).u.i = ((((unsafe { (*(*__slate_slot_709)).p3 }) as u32) as u64) << (32 as i32) | (((unsafe { (*(*__slate_slot_709)).p1 }) as u32) as u64)) as i64;
}
break '__join_26;
} else {
if __t4 == (154 as i32) {
// same as TK_FLOAT, out2
*__slate_slot_721 = out2Prerelease(p, *__slate_slot_709);
unsafe {
(*(*__slate_slot_721)).flags = ((8 as i32) as i16) as u16;
}
*__slate_slot_730 = (((((unsafe { (*(*__slate_slot_709)).p3 }) as u32) as u64) << (32 as i32) | (((unsafe { (*(*__slate_slot_709)).p1 }) as u32) as u64)) as i64) as u64;
{
}
unsafe { memcpy((unsafe { std::ptr::addr_of_mut!((*(*__slate_slot_721)).u.r) }) as *mut (), std::ptr::addr_of_mut!(*__slate_slot_730) as *const (), ((8 as i32) as i64) as u64) };
0 as i32;
break '__join_26;
} else {
if __t4 == (118 as i32) {
// same as TK_STRING, out2
0 as i32;
*__slate_slot_721 = out2Prerelease(p, *__slate_slot_709);
unsafe {
(*(*__slate_slot_709)).p1 = unsafe { sqlite3Strlen30((unsafe { (*(*__slate_slot_709)).p4.z }) as *const i8) };
}
if ((*__slate_slot_713 as u32) as i32) != (1 as i32) {
*__slate_slot_710 = unsafe { sqlite3VdbeMemSetStr(*__slate_slot_721, (unsafe { (*(*__slate_slot_709)).p4.z }) as *const i8, -(1 as i32) as i64, ((1 as i32) as i8) as u8, None) };
0 as i32;
if *__slate_slot_710 != (0 as i32) {
break '__join_2;
} else {
if (0 as i32) != unsafe { sqlite3VdbeChangeEncoding(*__slate_slot_721, (*__slate_slot_713 as u32) as i32) } {
break '__join_1;
} else {
0 as i32;
0 as i32;
unsafe {
(*(*__slate_slot_721)).szMalloc = 0 as i32;
}
std::ptr::write(__slate_slot_1553, *__slate_slot_721);
std::ptr::write(__slate_slot_1554, unsafe { (*(*__slate_slot_1553)).flags });
std::ptr::write(__slate_slot_1555, ((((*__slate_slot_1554 as u32) as i32) | (8192 as i32)) as i16) as u16);
unsafe {
(*(*__slate_slot_1553)).flags = *__slate_slot_1555;
}
if ((unsafe { (*(*__slate_slot_709)).p4type }) as i32) == -(7 as i32) {
unsafe { sqlite3DbFree(*__slate_slot_711, (unsafe { (*(*__slate_slot_709)).p4.z }) as *mut ()) };
}
unsafe {
(*(*__slate_slot_709)).p4type = -(7 as i32) as i8;
}
unsafe {
(*(*__slate_slot_709)).p4.z = unsafe { (*(*__slate_slot_721)).z };
}
unsafe {
(*(*__slate_slot_709)).p1 = unsafe { (*(*__slate_slot_721)).n };
}
}
}
}
if (unsafe { (*(*__slate_slot_709)).p1 }) > unsafe { *unsafe { unsafe { (*(*__slate_slot_711)).aLimit.as_mut_ptr() as *mut i32 }.offset((0 as i32) as isize) } } {
break '__join_2;
} else {
unsafe {
(*(*__slate_slot_709)).opcode = ((75 as i32) as i8) as u8;
}
0 as i32;
// Fall through to the next case, OP_String
//
// no break
{
}
// Opcode: String P1 P2 P3 P4 P5
// Synopsis: r[P2]='P4' (len=P1)
//
// The string value P4 of length P1 (bytes) is stored in register P2.
//
// If P3 is not zero and the content of register P3 is equal to P5, then
// the datatype of the register P2 is converted to BLOB.  The content is
// the same sequence of bytes, it is merely interpreted as a BLOB instead
// of a string, as if it had been CAST.  In other words:
//
// if( P3!=0 and reg[P3]==P5 ) reg[P2] := CAST(reg[P2] as BLOB)
break '__join_1094;
}
} else {
if __t4 == (75 as i32) {
break '__join_1094;
} else {
if __t4 == (76 as i32) {
break '__join_1091;
} else {
if __t4 == (77 as i32) {
break '__join_1091;
} else {
if __t4 == (78 as i32) {
0 as i32;
*__slate_slot_721 = unsafe { (*__slate_slot_717).offset((unsafe { (*(*__slate_slot_709)).p1 }) as isize) };
unsafe {
(*(*__slate_slot_721)).flags = (((((unsafe { (*(*__slate_slot_721)).flags }) as u32) as i32) & !((0 as i32) | (63 as i32)) | (1 as i32)) as i16) as u16;
}
break '__join_26;
} else {
if __t4 == (79 as i32) {
// out2
0 as i32;
*__slate_slot_721 = out2Prerelease(p, *__slate_slot_709);
if (unsafe { (*(*__slate_slot_709)).p4.z }) == std::ptr::null_mut::<i8>() {
unsafe { sqlite3VdbeMemSetZeroBlob(*__slate_slot_721, unsafe { (*(*__slate_slot_709)).p1 }) };
if (unsafe { sqlite3VdbeMemExpandBlob(*__slate_slot_721) }) != (0 as i32) {
break '__join_1;
}
} else {
unsafe { sqlite3VdbeMemSetStr(*__slate_slot_721, (unsafe { (*(*__slate_slot_709)).p4.z }) as *const i8, (unsafe { (*(*__slate_slot_709)).p1 }) as i64, ((0 as i32) as i8) as u8, None) };
}
unsafe {
(*(*__slate_slot_721)).enc = *__slate_slot_713;
}
{
}
break '__join_26;
} else {
if __t4 == (80 as i32) {
// out2
0 as i32;
*__slate_slot_733 = unsafe { unsafe { (*p).aVar }.offset(((unsafe { (*(*__slate_slot_709)).p1 }) - (1 as i32)) as isize) };
if (unsafe { sqlite3VdbeMemTooBig(*__slate_slot_733) }) != (0 as i32) {
break '__join_2;
} else {
*__slate_slot_721 = unsafe { (*__slate_slot_717).offset((unsafe { (*(*__slate_slot_709)).p2 }) as isize) };
if (((unsafe { (*(*__slate_slot_721)).flags }) as u32) as i32) & ((32768 as i32) | (4096 as i32)) != (0 as i32) {
unsafe { sqlite3VdbeMemSetNull(*__slate_slot_721) };
}
unsafe { memcpy(*__slate_slot_721 as *mut (), *__slate_slot_733 as *const (), 24 as u64) };
std::ptr::write(__slate_slot_1561, *__slate_slot_721);
std::ptr::write(__slate_slot_1562, unsafe { (*(*__slate_slot_1561)).flags });
std::ptr::write(__slate_slot_1563, ((((*__slate_slot_1562 as u32) as i32) & !((4096 as i32) | (16384 as i32))) as i16) as u16);
unsafe {
(*(*__slate_slot_1561)).flags = *__slate_slot_1563;
}
std::ptr::write(__slate_slot_1564, *__slate_slot_721);
std::ptr::write(__slate_slot_1565, unsafe { (*(*__slate_slot_1564)).flags });
std::ptr::write(__slate_slot_1566, ((((*__slate_slot_1565 as u32) as i32) | ((8192 as i32) | (64 as i32))) as i16) as u16);
unsafe {
(*(*__slate_slot_1564)).flags = *__slate_slot_1566;
}
{
}
break '__join_26;
}
} else {
if __t4 == (81 as i32) {
*__slate_slot_734 = unsafe { (*(*__slate_slot_709)).p3 };
*__slate_slot_735 = unsafe { (*(*__slate_slot_709)).p1 };
*__slate_slot_736 = unsafe { (*(*__slate_slot_709)).p2 };
0 as i32;
0 as i32;
*__slate_slot_718 = unsafe { (*__slate_slot_717).offset(*__slate_slot_735 as isize) };
*__slate_slot_721 = unsafe { (*__slate_slot_717).offset(*__slate_slot_736 as isize) };
loop {
0 as i32;
0 as i32;
0 as i32;
{
}
unsafe { sqlite3VdbeMemMove(*__slate_slot_721, *__slate_slot_718) };
if (((unsafe { (*(*__slate_slot_721)).flags }) as u32) as i32) & (16384 as i32) != (0 as i32) {
*__slate_slot_1567 = (unsafe { sqlite3VdbeMemMakeWriteable(*__slate_slot_721) }) != (0 as i32);
} else {
*__slate_slot_1567 = false as bool;
}
if *__slate_slot_1567 {
break '__join_1;
} else {
{
}
{
}
std::ptr::write(__slate_slot_1568, *__slate_slot_718);
std::ptr::write(__slate_slot_1569, unsafe { (*__slate_slot_1568).offset((1 as i32) as isize) });
*__slate_slot_718 = *__slate_slot_1569;
std::ptr::write(__slate_slot_1570, *__slate_slot_721);
std::ptr::write(__slate_slot_1571, unsafe { (*__slate_slot_1570).offset((1 as i32) as isize) });
*__slate_slot_721 = *__slate_slot_1571;
std::ptr::write(__slate_slot_1572, *__slate_slot_734);
std::ptr::write(__slate_slot_1573, *__slate_slot_1572 - (1 as i32));
*__slate_slot_734 = *__slate_slot_1573;
if !(*__slate_slot_1573 != (0 as i32)) {
break '__join_26;
}
}
}
} else {
if __t4 == (82 as i32) {
*__slate_slot_737 = unsafe { (*(*__slate_slot_709)).p3 };
*__slate_slot_718 = unsafe { (*__slate_slot_717).offset((unsafe { (*(*__slate_slot_709)).p1 }) as isize) };
*__slate_slot_721 = unsafe { (*__slate_slot_717).offset((unsafe { (*(*__slate_slot_709)).p2 }) as isize) };
0 as i32;
loop {
if (1 as i32) != (0 as i32) {
{
}
unsafe { sqlite3VdbeMemShallowCopy(*__slate_slot_721, *__slate_slot_718 as *const sqlite3_value, 16384 as i32) };
if (((unsafe { (*(*__slate_slot_721)).flags }) as u32) as i32) & (16384 as i32) != (0 as i32) {
*__slate_slot_1574 = (unsafe { sqlite3VdbeMemMakeWriteable(*__slate_slot_721) }) != (0 as i32);
} else {
*__slate_slot_1574 = false as bool;
}
if *__slate_slot_1574 {
break '__join_1;
} else {
{
}
if (((unsafe { (*(*__slate_slot_721)).flags }) as u32) as i32) & (2048 as i32) != (0 as i32) && (((unsafe { (*(*__slate_slot_709)).p5 }) as u32) as i32) & (2 as i32) != (0 as i32) {
std::ptr::write(__slate_slot_1575, *__slate_slot_721);
std::ptr::write(__slate_slot_1576, unsafe { (*(*__slate_slot_1575)).flags });
std::ptr::write(__slate_slot_1577, ((((*__slate_slot_1576 as u32) as i32) & !(2048 as i32)) as i16) as u16);
unsafe {
(*(*__slate_slot_1575)).flags = *__slate_slot_1577;
}
}
{
}
std::ptr::write(__slate_slot_1578, *__slate_slot_737);
std::ptr::write(__slate_slot_1579, *__slate_slot_1578 - (1 as i32));
*__slate_slot_737 = *__slate_slot_1579;
if *__slate_slot_1578 == (0 as i32) {
break '__join_26;
} else {
std::ptr::write(__slate_slot_1580, *__slate_slot_721);
std::ptr::write(__slate_slot_1581, unsafe { (*__slate_slot_1580).offset((1 as i32) as isize) });
*__slate_slot_721 = *__slate_slot_1581;
std::ptr::write(__slate_slot_1582, *__slate_slot_718);
std::ptr::write(__slate_slot_1583, unsafe { (*__slate_slot_1582).offset((1 as i32) as isize) });
*__slate_slot_718 = *__slate_slot_1583;
}
}
} else {
break '__join_26;
}
}
} else {
if __t4 == (83 as i32) {
// out2
*__slate_slot_718 = unsafe { (*__slate_slot_717).offset((unsafe { (*(*__slate_slot_709)).p1 }) as isize) };
*__slate_slot_721 = unsafe { (*__slate_slot_717).offset((unsafe { (*(*__slate_slot_709)).p2 }) as isize) };
0 as i32;
unsafe { sqlite3VdbeMemShallowCopy(*__slate_slot_721, *__slate_slot_718 as *const sqlite3_value, 16384 as i32) };
break '__join_26;
} else {
if __t4 == (84 as i32) {
// out2
*__slate_slot_718 = unsafe { (*__slate_slot_717).offset((unsafe { (*(*__slate_slot_709)).p1 }) as isize) };
0 as i32;
*__slate_slot_721 = unsafe { (*__slate_slot_717).offset((unsafe { (*(*__slate_slot_709)).p2 }) as isize) };
unsafe { sqlite3VdbeMemSetInt64(*__slate_slot_721, unsafe { (*(*__slate_slot_718)).u.i }) };
break '__join_26;
} else {
if __t4 == (85 as i32) {
std::ptr::write(__slate_slot_1584, unsafe { sqlite3VdbeCheckFkImmediate(p) });
*__slate_slot_710 = *__slate_slot_1584;
if *__slate_slot_1584 != (0 as i32) {
break '__join_1271;
} else {
break '__join_26;
}
} else {
if __t4 == (86 as i32) {
break '__join_1060;
} else {
if __t4 == (112 as i32) {
// same as TK_CONCAT, in1, in2, out3
*__slate_slot_718 = unsafe { (*__slate_slot_717).offset((unsafe { (*(*__slate_slot_709)).p1 }) as isize) };
*__slate_slot_719 = unsafe { (*__slate_slot_717).offset((unsafe { (*(*__slate_slot_709)).p2 }) as isize) };
*__slate_slot_721 = unsafe { (*__slate_slot_717).offset((unsafe { (*(*__slate_slot_709)).p3 }) as isize) };
{
}
0 as i32;
*__slate_slot_739 = unsafe { (*(*__slate_slot_718)).flags };
{
}
{
}
if (((*__slate_slot_739 as u32) as i32) | (((unsafe { (*(*__slate_slot_719)).flags }) as u32) as i32)) & (1 as i32) != (0 as i32) {
unsafe { sqlite3VdbeMemSetNull(*__slate_slot_721) };
break '__join_26;
} else {
if ((*__slate_slot_739 as u32) as i32) & ((2 as i32) | (16 as i32)) == (0 as i32) {
if (unsafe { sqlite3VdbeMemStringify(*__slate_slot_718, *__slate_slot_713, ((0 as i32) as i8) as u8) }) != (0 as i32) {
break '__join_1;
} else {
*__slate_slot_739 = (((((unsafe { (*(*__slate_slot_718)).flags }) as u32) as i32) & !(2 as i32)) as i16) as u16;
}
} else {
if ((*__slate_slot_739 as u32) as i32) & (1024 as i32) != (0 as i32) {
if (unsafe { sqlite3VdbeMemExpandBlob(*__slate_slot_718) }) != (0 as i32) {
break '__join_1;
} else {
*__slate_slot_739 = (((((unsafe { (*(*__slate_slot_718)).flags }) as u32) as i32) & !(2 as i32)) as i16) as u16;
}
}
}
*__slate_slot_740 = unsafe { (*(*__slate_slot_719)).flags };
if ((*__slate_slot_740 as u32) as i32) & ((2 as i32) | (16 as i32)) == (0 as i32) {
if (unsafe { sqlite3VdbeMemStringify(*__slate_slot_719, *__slate_slot_713, ((0 as i32) as i8) as u8) }) != (0 as i32) {
break '__join_1;
} else {
*__slate_slot_740 = (((((unsafe { (*(*__slate_slot_719)).flags }) as u32) as i32) & !(2 as i32)) as i16) as u16;
}
} else {
if ((*__slate_slot_740 as u32) as i32) & (1024 as i32) != (0 as i32) {
if (unsafe { sqlite3VdbeMemExpandBlob(*__slate_slot_719) }) != (0 as i32) {
break '__join_1;
} else {
*__slate_slot_740 = (((((unsafe { (*(*__slate_slot_719)).flags }) as u32) as i32) & !(2 as i32)) as i16) as u16;
}
}
}
*__slate_slot_738 = (unsafe { (*(*__slate_slot_718)).n }) as i64;
std::ptr::write(__slate_slot_1585, *__slate_slot_738);
std::ptr::write(__slate_slot_1586, *__slate_slot_1585 + ((unsafe { (*(*__slate_slot_719)).n }) as i64));
*__slate_slot_738 = *__slate_slot_1586;
if *__slate_slot_738 > ((unsafe { *unsafe { unsafe { (*(*__slate_slot_711)).aLimit.as_mut_ptr() as *mut i32 }.offset((0 as i32) as isize) } }) as i64) {
break '__join_2;
} else {
if (unsafe { sqlite3VdbeMemGrow(*__slate_slot_721, (*__slate_slot_738 as i32) + (2 as i32), (*__slate_slot_721 == *__slate_slot_719) as i32) }) != (0 as i32) {
break '__join_1;
} else {
unsafe {
(*(*__slate_slot_721)).flags = (((((unsafe { (*(*__slate_slot_721)).flags }) as u32) as i32) & !((3519 as i32) | (1024 as i32)) | (2 as i32)) as i16) as u16;
}
if *__slate_slot_721 != *__slate_slot_719 {
unsafe { memcpy((unsafe { (*(*__slate_slot_721)).z }) as *mut (), (unsafe { (*(*__slate_slot_719)).z }) as *const (), ((unsafe { (*(*__slate_slot_719)).n }) as i64) as u64) };
0 as i32;
unsafe {
(*(*__slate_slot_719)).flags = *__slate_slot_740;
}
}
unsafe { memcpy((unsafe { unsafe { (*(*__slate_slot_721)).z }.offset((unsafe { (*(*__slate_slot_719)).n }) as isize) }) as *mut (), (unsafe { (*(*__slate_slot_718)).z }) as *const (), ((unsafe { (*(*__slate_slot_718)).n }) as i64) as u64) };
0 as i32;
unsafe {
(*(*__slate_slot_718)).flags = *__slate_slot_739;
}
if ((*__slate_slot_713 as u32) as i32) > (1 as i32) {
std::ptr::write(__slate_slot_1587, *__slate_slot_738);
std::ptr::write(__slate_slot_1588, *__slate_slot_1587 & (!(1 as i32) as i64));
*__slate_slot_738 = *__slate_slot_1588;
}
unsafe {
*unsafe { unsafe { (*(*__slate_slot_721)).z }.offset(*__slate_slot_738 as isize) } = (0 as i32) as i8;
}
unsafe {
*unsafe { unsafe { (*(*__slate_slot_721)).z }.offset((*__slate_slot_738 + ((1 as i32) as i64)) as isize) } = (0 as i32) as i8;
}
std::ptr::write(__slate_slot_1589, *__slate_slot_721);
std::ptr::write(__slate_slot_1590, unsafe { (*(*__slate_slot_1589)).flags });
std::ptr::write(__slate_slot_1591, ((((*__slate_slot_1590 as u32) as i32) | (512 as i32)) as i16) as u16);
unsafe {
(*(*__slate_slot_1589)).flags = *__slate_slot_1591;
}
unsafe {
(*(*__slate_slot_721)).n = *__slate_slot_738 as i32;
}
unsafe {
(*(*__slate_slot_721)).enc = *__slate_slot_713;
}
{
}
break '__join_26;
}
}
}
} else {
if __t4 == (107 as i32) {
break '__join_1035;
} else {
if __t4 == (108 as i32) {
break '__join_1035;
} else {
if __t4 == (109 as i32) {
break '__join_1035;
} else {
if __t4 == (110 as i32) {
break '__join_1035;
} else {
if __t4 == (111 as i32) {
break '__join_1035;
} else {
if __t4 == (87 as i32) {
0 as i32;
if (unsafe { (*(*__slate_slot_709)).p1 }) != (0 as i32) {
unsafe { sqlite3VdbeMemSetInt64(unsafe { (*__slate_slot_717).offset((unsafe { (*(*__slate_slot_709)).p1 }) as isize) }, (0 as i32) as i64) };
break '__join_26;
} else {
break '__join_26;
}
} else {
if __t4 == (103 as i32) {
break '__join_1005;
} else {
if __t4 == (104 as i32) {
break '__join_1005;
} else {
if __t4 == (105 as i32) {
break '__join_1005;
} else {
if __t4 == (106 as i32) {
break '__join_1005;
} else {
if __t4 == (88 as i32) {
// in1
*__slate_slot_718 = unsafe { (*__slate_slot_717).offset((unsafe { (*(*__slate_slot_709)).p1 }) as isize) };
{
}
unsafe { sqlite3VdbeMemIntegerify(*__slate_slot_718) };
std::ptr::write(__slate_slot_1614, (unsafe { std::ptr::addr_of_mut!((*(*__slate_slot_718)).u.i) }) as *mut u64);
std::ptr::write(__slate_slot_1615, unsafe { *(*__slate_slot_1614) });
std::ptr::write(__slate_slot_1616, (*__slate_slot_1615).wrapping_add(((unsafe { (*(*__slate_slot_709)).p2 }) as i64) as u64));
unsafe {
*(*__slate_slot_1614) = *__slate_slot_1616;
}
break '__join_26;
} else {
if __t4 == (13 as i32) {
// jump0, in1
*__slate_slot_718 = unsafe { (*__slate_slot_717).offset((unsafe { (*(*__slate_slot_709)).p1 }) as isize) };
if (((unsafe { (*(*__slate_slot_718)).flags }) as u32) as i32) & (4 as i32) == (0 as i32) {
applyAffinity(*__slate_slot_718, (67 as i32) as i8, *__slate_slot_713);
if (((unsafe { (*(*__slate_slot_718)).flags }) as u32) as i32) & (4 as i32) == (0 as i32) {
{
}
if (unsafe { (*(*__slate_slot_709)).p2 }) == (0 as i32) {
break '__join_984;
} else {
break '__join_1125;
}
}
}
{
}
unsafe {
(*(*__slate_slot_718)).flags = (((((unsafe { (*(*__slate_slot_718)).flags }) as u32) as i32) & !((3519 as i32) | (1024 as i32)) | (4 as i32)) as i16) as u16;
}
break '__join_26;
} else {
if __t4 == (89 as i32) {
// in1
*__slate_slot_718 = unsafe { (*__slate_slot_717).offset((unsafe { (*(*__slate_slot_709)).p1 }) as isize) };
if (((unsafe { (*(*__slate_slot_718)).flags }) as u32) as i32) & ((4 as i32) | (32 as i32)) != (0 as i32) {
{
}
{
}
unsafe { sqlite3VdbeMemRealify(*__slate_slot_718) };
{
}
break '__join_26;
} else {
break '__join_26;
}
} else {
if __t4 == (90 as i32) {
// in1
0 as i32;
{
}
{
}
{
}
{
}
{
}
*__slate_slot_718 = unsafe { (*__slate_slot_717).offset((unsafe { (*(*__slate_slot_709)).p1 }) as isize) };
{
}
if (((unsafe { (*(*__slate_slot_718)).flags }) as u32) as i32) & (1024 as i32) != (0 as i32) {
*__slate_slot_1617 = unsafe { sqlite3VdbeMemExpandBlob(*__slate_slot_718) };
} else {
*__slate_slot_1617 = 0 as i32;
}
*__slate_slot_710 = *__slate_slot_1617;
if *__slate_slot_710 != (0 as i32) {
break '__join_1268;
} else {
*__slate_slot_710 = unsafe { sqlite3VdbeMemCast(*__slate_slot_718, ((unsafe { (*(*__slate_slot_709)).p2 }) as i8) as u8, *__slate_slot_713) };
if *__slate_slot_710 != (0 as i32) {
break '__join_1267;
} else {
{
}
{
}
break '__join_26;
}
}
} else {
if __t4 == (54 as i32) {
break '__join_974;
} else {
if __t4 == (53 as i32) {
break '__join_974;
} else {
if __t4 == (57 as i32) {
break '__join_974;
} else {
if __t4 == (56 as i32) {
break '__join_974;
} else {
if __t4 == (55 as i32) {
break '__join_974;
} else {
if __t4 == (58 as i32) {
break '__join_974;
} else {
if __t4 == (59 as i32) {
// same as TK_ESCAPE, jump
0 as i32;
{
}
if *__slate_slot_714 == (0 as i32) {
break '__join_1125;
} else {
break '__join_26;
}
} else {
if __t4 == (91 as i32) {
0 as i32;
0 as i32;
0 as i32;
0 as i32;
break '__join_26;
} else {
if __t4 == (92 as i32) {
if (((unsafe { (*(*__slate_slot_709)).p5 }) as u32) as i32) & (1 as i32) == (0 as i32) {
*__slate_slot_764 = std::ptr::null_mut::<u32>();
} else {
0 as i32;
0 as i32;
0 as i32;
*__slate_slot_764 = unsafe { unsafe { (*unsafe { (*__slate_slot_709).offset(-(1 as i32) as isize) }).p4.ai }.offset((1 as i32) as isize) };
0 as i32;
}
*__slate_slot_756 = unsafe { (*(*__slate_slot_709)).p3 };
*__slate_slot_760 = (unsafe { (*(*__slate_slot_709)).p4.pKeyInfo }) as *const KeyInfo;
0 as i32;
0 as i32;
0 as i32;
*__slate_slot_758 = unsafe { (*(*__slate_slot_709)).p1 };
*__slate_slot_759 = unsafe { (*(*__slate_slot_709)).p2 };
*__slate_slot_757 = 0 as i32;
'__join_918: {
loop {
if *__slate_slot_757 < *__slate_slot_756 {
*__slate_slot_761 = if *__slate_slot_764 != std::ptr::null_mut::<u32>() { unsafe { *unsafe { (*__slate_slot_764).offset(*__slate_slot_757 as isize) } } } else { *__slate_slot_757 as u32 };
0 as i32;
0 as i32;
{
}
{
}
0 as i32;
*__slate_slot_762 = unsafe { *unsafe { unsafe { std::ptr::addr_of!((*(*__slate_slot_760)).aColl) as *const *mut CollSeq }.offset(*__slate_slot_757 as isize) } };
*__slate_slot_763 = (((unsafe { *unsafe { unsafe { (*(*__slate_slot_760)).aSortFlags }.offset(*__slate_slot_757 as isize) } }) as u32) as i32) & (1 as i32);
*__slate_slot_714 = unsafe { sqlite3MemCompare((unsafe { (*__slate_slot_717).offset((*__slate_slot_758 as u32).wrapping_add(*__slate_slot_761) as isize) }) as *const sqlite3_value, (unsafe { (*__slate_slot_717).offset((*__slate_slot_759 as u32).wrapping_add(*__slate_slot_761) as isize) }) as *const sqlite3_value, *__slate_slot_762 as *const CollSeq) };
if *__slate_slot_714 != (0 as i32) {
break;
} else {
std::ptr::write(__slate_slot_1624, *__slate_slot_757);
std::ptr::write(__slate_slot_1625, *__slate_slot_1624 + (1 as i32));
*__slate_slot_757 = *__slate_slot_1625;
}
} else {
break '__join_918;
}
}
if (((unsafe { *unsafe { unsafe { (*(*__slate_slot_760)).aSortFlags }.offset(*__slate_slot_757 as isize) } }) as u32) as i32) & (2 as i32) != (0 as i32) && ((((unsafe { (*unsafe { (*__slate_slot_717).offset((*__slate_slot_758 as u32).wrapping_add(*__slate_slot_761) as isize) }).flags }) as u32) as i32) & (1 as i32) != (0 as i32) || (((unsafe { (*unsafe { (*__slate_slot_717).offset((*__slate_slot_759 as u32).wrapping_add(*__slate_slot_761) as isize) }).flags }) as u32) as i32) & (1 as i32) != (0 as i32)) {
*__slate_slot_714 = -(*__slate_slot_714);
}
if *__slate_slot_763 != (0 as i32) {
*__slate_slot_714 = -(*__slate_slot_714);
}
}
0 as i32;
break '__join_26;
} else {
if __t4 == (14 as i32) {
// jump
0 as i32;
0 as i32;
if *__slate_slot_714 < (0 as i32) {
{
}
*__slate_slot_709 = unsafe { (*__slate_slot_708).offset(((unsafe { (*(*__slate_slot_709)).p1 }) - (1 as i32)) as isize) };
break '__join_26;
} else {
if *__slate_slot_714 == (0 as i32) {
{
}
*__slate_slot_709 = unsafe { (*__slate_slot_708).offset(((unsafe { (*(*__slate_slot_709)).p2 }) - (1 as i32)) as isize) };
break '__join_26;
} else {
{
}
*__slate_slot_709 = unsafe { (*__slate_slot_708).offset(((unsafe { (*(*__slate_slot_709)).p3 }) - (1 as i32)) as isize) };
break '__join_26;
}
}
} else {
if __t4 == (44 as i32) {
break '__join_912;
} else {
if __t4 == (43 as i32) {
break '__join_912;
} else {
if __t4 == (93 as i32) {
// in1, out2
0 as i32;
0 as i32;
0 as i32;
unsafe { sqlite3VdbeMemSetInt64(unsafe { (*__slate_slot_717).offset((unsafe { (*(*__slate_slot_709)).p2 }) as isize) }, ((unsafe { sqlite3VdbeBooleanValue(unsafe { (*__slate_slot_717).offset((unsafe { (*(*__slate_slot_709)).p1 }) as isize) }, unsafe { (*(*__slate_slot_709)).p3 }) }) ^ unsafe { (*(*__slate_slot_709)).p4.i }) as i64) };
break '__join_26;
} else {
if __t4 == (19 as i32) {
// same as TK_NOT, in1, out2
*__slate_slot_718 = unsafe { (*__slate_slot_717).offset((unsafe { (*(*__slate_slot_709)).p1 }) as isize) };
*__slate_slot_721 = unsafe { (*__slate_slot_717).offset((unsafe { (*(*__slate_slot_709)).p2 }) as isize) };
if (((unsafe { (*(*__slate_slot_718)).flags }) as u32) as i32) & (1 as i32) == (0 as i32) {
unsafe { sqlite3VdbeMemSetInt64(*__slate_slot_721, !((unsafe { sqlite3VdbeBooleanValue(*__slate_slot_718, 0 as i32) }) != (0 as i32)) as i64) };
break '__join_26;
} else {
unsafe { sqlite3VdbeMemSetNull(*__slate_slot_721) };
break '__join_26;
}
} else {
if __t4 == (115 as i32) {
// same as TK_BITNOT, in1, out2
*__slate_slot_718 = unsafe { (*__slate_slot_717).offset((unsafe { (*(*__slate_slot_709)).p1 }) as isize) };
*__slate_slot_721 = unsafe { (*__slate_slot_717).offset((unsafe { (*(*__slate_slot_709)).p2 }) as isize) };
unsafe { sqlite3VdbeMemSetNull(*__slate_slot_721) };
if (((unsafe { (*(*__slate_slot_718)).flags }) as u32) as i32) & (1 as i32) == (0 as i32) {
unsafe {
(*(*__slate_slot_721)).flags = ((4 as i32) as i16) as u16;
}
unsafe {
(*(*__slate_slot_721)).u.i = !unsafe { sqlite3VdbeIntValue(*__slate_slot_718 as *const sqlite3_value) };
}
break '__join_26;
} else {
break '__join_26;
}
} else {
if __t4 == (15 as i32) {
// jump
0 as i32;
if (unsafe { (*p).pFrame }) != std::ptr::null_mut::<VdbeFrame>() {
*__slate_slot_769 = (((unsafe { (*__slate_slot_709).offset_from((unsafe { (*p).aOp }) as *mut VdbeOp) }) as i64) as i32) as u32;
if (((unsafe { *unsafe { unsafe { (*unsafe { (*p).pFrame }).aOnce }.offset((*__slate_slot_769 / ((8 as i32) as u32)) as isize) } }) as u32) as i32) & (1 as i32) << (*__slate_slot_769 & ((7 as i32) as u32)) != (0 as i32) {
{
}
break '__join_1125;
} else {
std::ptr::write(__slate_slot_1626, unsafe { unsafe { (*unsafe { (*p).pFrame }).aOnce }.offset((*__slate_slot_769 / ((8 as i32) as u32)) as isize) });
std::ptr::write(__slate_slot_1627, unsafe { *(*__slate_slot_1626) });
std::ptr::write(__slate_slot_1628, ((((*__slate_slot_1627 as u32) as i32) | (1 as i32) << (*__slate_slot_769 & ((7 as i32) as u32))) as i8) as u8);
unsafe {
*(*__slate_slot_1626) = *__slate_slot_1628;
}
}
} else {
if (unsafe { (*unsafe { unsafe { (*p).aOp }.offset((0 as i32) as isize) }).p1 }) == unsafe { (*(*__slate_slot_709)).p1 } {
{
}
break '__join_1125;
}
}
{
}
unsafe {
(*(*__slate_slot_709)).p1 = unsafe { (*unsafe { unsafe { (*p).aOp }.offset((0 as i32) as isize) }).p1 };
}
break '__join_26;
} else {
if __t4 == (16 as i32) {
// jump, in1
*__slate_slot_770 = unsafe { sqlite3VdbeBooleanValue(unsafe { (*__slate_slot_717).offset((unsafe { (*(*__slate_slot_709)).p1 }) as isize) }, unsafe { (*(*__slate_slot_709)).p3 }) };
{
}
if *__slate_slot_770 != (0 as i32) {
break '__join_1125;
} else {
break '__join_26;
}
} else {
if __t4 == (17 as i32) {
// jump, in1
*__slate_slot_771 = !((unsafe { sqlite3VdbeBooleanValue(unsafe { (*__slate_slot_717).offset((unsafe { (*(*__slate_slot_709)).p1 }) as isize) }, !((unsafe { (*(*__slate_slot_709)).p3 }) != (0 as i32)) as i32) }) != (0 as i32)) as i32;
{
}
if *__slate_slot_771 != (0 as i32) {
break '__join_1125;
} else {
break '__join_26;
}
} else {
if __t4 == (51 as i32) {
// same as TK_ISNULL, jump, in1
*__slate_slot_718 = unsafe { (*__slate_slot_717).offset((unsafe { (*(*__slate_slot_709)).p1 }) as isize) };
{
}
if (((unsafe { (*(*__slate_slot_718)).flags }) as u32) as i32) & (1 as i32) != (0 as i32) {
break '__join_1125;
} else {
break '__join_26;
}
} else {
if __t4 == (18 as i32) {
// jump
0 as i32;
0 as i32;
if (unsafe { (*(*__slate_slot_709)).p1 }) >= (0 as i32) {
*__slate_slot_772 = unsafe { *unsafe { unsafe { (*p).apCsr }.offset((unsafe { (*(*__slate_slot_709)).p1 }) as isize) } };
0 as i32;
0 as i32;
if (unsafe { (*(*__slate_slot_709)).p3 }) < (((unsafe { (*(*__slate_slot_772)).nHdrParsed }) as u32) as i32) {
*__slate_slot_774 = unsafe { *unsafe { unsafe { std::ptr::addr_of_mut!((*(*__slate_slot_772)).aType) as *mut u32 }.offset((unsafe { (*(*__slate_slot_709)).p3 }) as isize) } };
if *__slate_slot_774 >= ((12 as i32) as u32) {
if *__slate_slot_774 & ((1 as i32) as u32) != (0 as u32) {
*__slate_slot_773 = ((4 as i32) as i16) as u16; // SQLITE_TEXT
} else {
*__slate_slot_773 = ((8 as i32) as i16) as u16; // SQLITE_BLOB
}
} else {
{
}
{
}
{
}
{
}
{
}
{
}
{
}
{
}
{
}
{
}
{
}
{
}
*__slate_slot_773 = (unsafe { *unsafe { unsafe { std::ptr::addr_of!(aMask) as *const u8 }.offset(*__slate_slot_774 as isize) } }) as u16;
}
} else {
*__slate_slot_773 = (((1 as i32) << (unsafe { (*(*__slate_slot_709)).p4.i }) - (1 as i32)) as i16) as u16;
{
}
{
}
{
}
{
}
{
}
}
} else {
0 as i32;
*__slate_slot_773 = (((1 as i32) << (unsafe { sqlite3_value_type(unsafe { (*__slate_slot_717).offset((unsafe { (*(*__slate_slot_709)).p3 }) as isize) }) }) - (1 as i32)) as i16) as u16;
{
}
{
}
{
}
{
}
{
}
}
{
}
if ((*__slate_slot_773 as u32) as i32) & (((unsafe { (*(*__slate_slot_709)).p5 }) as u32) as i32) != (0 as i32) {
break '__join_1125;
} else {
break '__join_26;
}
} else {
if __t4 == (94 as i32) {
// in1, in2, out2, in3
if (((unsafe { (*unsafe { (*__slate_slot_717).offset((unsafe { (*(*__slate_slot_709)).p1 }) as isize) }).flags }) as u32) as i32) & (1 as i32) != (0 as i32) || (((unsafe { (*unsafe { (*__slate_slot_717).offset((unsafe { (*(*__slate_slot_709)).p3 }) as isize) }).flags }) as u32) as i32) & (1 as i32) != (0 as i32) {
unsafe { sqlite3VdbeMemSetNull(unsafe { (*__slate_slot_717).offset((unsafe { (*(*__slate_slot_709)).p2 }) as isize) }) };
break '__join_26;
} else {
unsafe { sqlite3VdbeMemSetInt64(unsafe { (*__slate_slot_717).offset((unsafe { (*(*__slate_slot_709)).p2 }) as isize) }, (0 as i32) as i64) };
break '__join_26;
}
} else {
if __t4 == (52 as i32) {
// same as TK_NOTNULL, jump, in1
*__slate_slot_718 = unsafe { (*__slate_slot_717).offset((unsafe { (*(*__slate_slot_709)).p1 }) as isize) };
{
}
if (((unsafe { (*(*__slate_slot_718)).flags }) as u32) as i32) & (1 as i32) == (0 as i32) {
break '__join_1125;
} else {
break '__join_26;
}
} else {
if __t4 == (20 as i32) {
// jump
0 as i32;
*__slate_slot_776 = unsafe { *unsafe { unsafe { (*p).apCsr }.offset((unsafe { (*(*__slate_slot_709)).p1 }) as isize) } };
if *__slate_slot_776 != std::ptr::null_mut::<VdbeCursor>() && (unsafe { (*(*__slate_slot_776)).nullRow }) != (0 as u8) {
unsafe { sqlite3VdbeMemSetNull(unsafe { (*__slate_slot_717).offset((unsafe { (*(*__slate_slot_709)).p3 }) as isize) }) };
break '__join_1125;
} else {
break '__join_26;
}
} else {
if __t4 == (95 as i32) {
// out3
0 as i32;
*__slate_slot_777 = unsafe { *unsafe { unsafe { (*p).apCsr }.offset((unsafe { (*(*__slate_slot_709)).p1 }) as isize) } };
*__slate_slot_721 = unsafe { unsafe { (*p).aMem }.offset((unsafe { (*(*__slate_slot_709)).p3 }) as isize) };
if *__slate_slot_777 == std::ptr::null_mut::<VdbeCursor>() || (((unsafe { (*(*__slate_slot_777)).eCurType }) as u32) as i32) != (0 as i32) {
unsafe { sqlite3VdbeMemSetNull(*__slate_slot_721) };
break '__join_26;
} else {
if (unsafe { (*(*__slate_slot_777)).deferredMoveto }) != (0 as u8) {
*__slate_slot_710 = unsafe { sqlite3VdbeFinishMoveto(*__slate_slot_777) };
if *__slate_slot_710 != (0 as i32) {
break '__join_1266;
}
}
if (unsafe { sqlite3BtreeEof(unsafe { (*(*__slate_slot_777)).uc.pCursor }) }) != (0 as i32) {
unsafe { sqlite3VdbeMemSetNull(*__slate_slot_721) };
break '__join_26;
} else {
unsafe { sqlite3VdbeMemSetInt64(*__slate_slot_721, unsafe { sqlite3BtreeOffset(unsafe { (*(*__slate_slot_777)).uc.pCursor }) }) };
break '__join_26;
}
}
} else {
if __t4 == (96 as i32) {
// ncycle
0 as i32;
0 as i32;
*__slate_slot_779 = unsafe { *unsafe { unsafe { (*p).apCsr }.offset((unsafe { (*(*__slate_slot_709)).p1 }) as isize) } };
*__slate_slot_778 = (unsafe { (*(*__slate_slot_709)).p2 }) as u32;
'__join_789: {
'__join_790: {
'__join_818: {
'__join_822: {
'__join_834: {
'__join_843: {
'__join_849: {
'__join_852: {
'__join_853: {
loop {
0 as i32;
0 as i32;
*__slate_slot_781 = unsafe { (*(*__slate_slot_779)).aOffset };
0 as i32;
0 as i32;
0 as i32;
0 as i32;
if (unsafe { (*(*__slate_slot_779)).cacheStatus }) != unsafe { (*p).cacheCtr } {
if (unsafe { (*(*__slate_slot_779)).nullRow }) != (0 as u8) {
break '__join_852;
} else {
*__slate_slot_780 = unsafe { (*(*__slate_slot_779)).uc.pCursor };
if (unsafe { (*(*__slate_slot_779)).deferredMoveto }) != (0 as u8) {
0 as i32;
if (unsafe { (*(*__slate_slot_779)).ub.aAltMap }) != std::ptr::null_mut::<u32>() {
std::ptr::write(__slate_slot_1631, unsafe { *unsafe { unsafe { (*(*__slate_slot_779)).ub.aAltMap }.offset(((1 as i32) as u32).wrapping_add(*__slate_slot_778) as isize) } });
*__slate_slot_792 = *__slate_slot_1631;
*__slate_slot_1630 = *__slate_slot_1631 > ((0 as i32) as u32);
} else {
*__slate_slot_1630 = false as bool;
}
if *__slate_slot_1630 {
*__slate_slot_779 = unsafe { (*(*__slate_slot_779)).pAltCursor };
*__slate_slot_778 = (*__slate_slot_792).wrapping_sub((1 as i32) as u32);
} else {
break;
}
} else {
if (unsafe { sqlite3BtreeCursorHasMoved(*__slate_slot_780) }) != (0 as i32) {
*__slate_slot_710 = unsafe { sqlite3VdbeHandleMovedCursor(*__slate_slot_779) };
if *__slate_slot_710 != (0 as i32) {
break '__join_1264;
}
} else {
break '__join_853;
}
}
}
} else {
if (unsafe { sqlite3BtreeCursorHasMoved(unsafe { (*(*__slate_slot_779)).uc.pCursor }) }) != (0 as i32) {
*__slate_slot_710 = unsafe { sqlite3VdbeHandleMovedCursor(*__slate_slot_779) };
if *__slate_slot_710 != (0 as i32) {
break '__join_1265;
}
} else {
break '__join_843;
}
}
}
*__slate_slot_710 = unsafe { sqlite3VdbeFinishMoveto(*__slate_slot_779) };
if *__slate_slot_710 != (0 as i32) {
break '__join_1263;
}
}
0 as i32;
0 as i32;
0 as i32;
unsafe {
(*(*__slate_slot_779)).payloadSize = unsafe { sqlite3BtreePayloadSize(*__slate_slot_780) };
}
unsafe {
(*(*__slate_slot_779)).aRow = (unsafe { sqlite3BtreePayloadFetch(*__slate_slot_780, unsafe { std::ptr::addr_of_mut!((*(*__slate_slot_779)).szRow) }) }) as *const u8;
}
0 as i32;
0 as i32; // Maximum page size is 64KiB
break '__join_849;
}
if (((unsafe { (*(*__slate_slot_779)).eCurType }) as u32) as i32) == (3 as i32) && (unsafe { (*(*__slate_slot_779)).seekResult }) > (0 as i32) {
// For the special case of as pseudo-cursor, the seekResult field
// identifies the register that holds the record
*__slate_slot_791 = unsafe { (*__slate_slot_717).offset((unsafe { (*(*__slate_slot_779)).seekResult }) as isize) };
0 as i32;
0 as i32;
std::ptr::write(__slate_slot_1629, (unsafe { (*(*__slate_slot_791)).n }) as u32);
unsafe {
(*(*__slate_slot_779)).szRow = *__slate_slot_1629;
}
unsafe {
(*(*__slate_slot_779)).payloadSize = *__slate_slot_1629;
}
unsafe {
(*(*__slate_slot_779)).aRow = ((unsafe { (*(*__slate_slot_791)).z }) as *mut u8) as *const u8;
}
} else {
*__slate_slot_784 = unsafe { (*__slate_slot_717).offset((unsafe { (*(*__slate_slot_709)).p3 }) as isize) };
{
}
unsafe { sqlite3VdbeMemSetNull(*__slate_slot_784) };
break '__join_790;
}
}
unsafe {
(*(*__slate_slot_779)).cacheStatus = unsafe { (*p).cacheCtr };
}
std::ptr::write(__slate_slot_1632, (unsafe { *unsafe { unsafe { (*(*__slate_slot_779)).aRow }.offset((0 as i32) as isize) } }) as u32);
unsafe {
*unsafe { (*__slate_slot_781).offset((0 as i32) as isize) } = *__slate_slot_1632;
}
if *__slate_slot_1632 < ((128 as i32) as u32) {
unsafe {
(*(*__slate_slot_779)).iHdrOffset = (1 as i32) as u32;
}
} else {
unsafe {
(*(*__slate_slot_779)).iHdrOffset = (unsafe { sqlite3GetVarint32(unsafe { (*(*__slate_slot_779)).aRow }, *__slate_slot_781) }) as u32;
}
}
unsafe {
(*(*__slate_slot_779)).nHdrParsed = ((0 as i32) as i16) as u16;
}
if (unsafe { (*(*__slate_slot_779)).szRow }) < unsafe { *unsafe { (*__slate_slot_781).offset((0 as i32) as isize) } } {
// pC->aRow does not have to hold the entire row, but it does at least
// need to cover the header of the record.  If pC->aRow does not contain
// the complete header, then set it to zero, forcing the header to be
// dynamically allocated.
unsafe {
(*(*__slate_slot_779)).aRow = std::ptr::null::<u8>();
}
unsafe {
(*(*__slate_slot_779)).szRow = (0 as i32) as u32;
}
// Make sure a corrupt database has not given us an oversize header.
// Do this now to avoid an oversize memory allocation.
//
// Type entries can be between 1 and 5 bytes each.  But 4 and 5 byte
// types use so much data space that there can only be 4096 and 32 of
// them, respectively.  So the maximum header length results from a
// 3-byte type for each of the maximum of 32768 columns plus three
// extra bytes for the header length itself.  32768*3 + 3 = 98307.
if (unsafe { *unsafe { (*__slate_slot_781).offset((0 as i32) as isize) } }) > ((98307 as i32) as u32) || (unsafe { *unsafe { (*__slate_slot_781).offset((0 as i32) as isize) } }) > unsafe { (*(*__slate_slot_779)).payloadSize } {
break '__join_789;
}
} else {
// This is an optimization.  By skipping over the first few tests
// (ex: pC->nHdrParsed<=p2) in the next section, we achieve a
// measurable performance gain.
//
// This branch is taken even if aOffset[0]==0.  Such a record is never
// generated by SQLite, and could be considered corruption, but we
// accept it for historical reasons.  When aOffset[0]==0, the code this
// branch jumps to reads past the end of the record, but never more
// than a few bytes.  Even if the record occurs at the end of the page
// content area, the "page header" comes after the page content and so
// this overread is harmless.  Similar overreads can occur for a corrupt
// database file.
*__slate_slot_786 = unsafe { (*(*__slate_slot_779)).aRow };
0 as i32; // Conditional skipped
{
}
break '__join_834;
}
}
// Make sure at least the first p2+1 entries of the header have been
// parsed and valid information is in aOffset[] and pC->aType[].
if ((((unsafe { (*(*__slate_slot_779)).nHdrParsed }) as u32) as i32) as u32) <= *__slate_slot_778 {
// If there is more header available for parsing in the record, try
// to extract additional fields up through the p2+1-th field
if (unsafe { (*(*__slate_slot_779)).iHdrOffset }) < unsafe { *unsafe { (*__slate_slot_781).offset((0 as i32) as isize) } } {
// Make sure zData points to enough of the record to cover the header.
if (unsafe { (*(*__slate_slot_779)).aRow }) == std::ptr::null::<u8>() {
unsafe { memset(std::ptr::addr_of_mut!(*__slate_slot_785) as *mut (), 0 as i32, 56 as u64) };
*__slate_slot_710 = unsafe { sqlite3VdbeMemFromBtreeZeroOffset(unsafe { (*(*__slate_slot_779)).uc.pCursor }, unsafe { *unsafe { (*__slate_slot_781).offset((0 as i32) as isize) } }, std::ptr::addr_of_mut!(*__slate_slot_785)) };
if *__slate_slot_710 != (0 as i32) {
break '__join_1262;
} else {
*__slate_slot_786 = ((*__slate_slot_785).z as *mut u8) as *const u8;
}
} else {
*__slate_slot_786 = unsafe { (*(*__slate_slot_779)).aRow };
}
// Fill in pC->aType[i] and aOffset[i] values through the p2-th field.
} else {
*__slate_slot_790 = (0 as i32) as u32;
break '__join_822;
}
} else {
*__slate_slot_790 = unsafe { *unsafe { unsafe { std::ptr::addr_of_mut!((*(*__slate_slot_779)).aType) as *mut u32 }.offset(*__slate_slot_778 as isize) } };
break '__join_818;
}
}
*__slate_slot_783 = ((unsafe { (*(*__slate_slot_779)).nHdrParsed }) as u32) as i32;
*__slate_slot_789 = (unsafe { *unsafe { (*__slate_slot_781).offset(*__slate_slot_783 as isize) } }) as u64;
*__slate_slot_787 = unsafe { (*__slate_slot_786).offset((unsafe { (*(*__slate_slot_779)).iHdrOffset }) as isize) };
*__slate_slot_788 = unsafe { (*__slate_slot_786).offset((unsafe { *unsafe { (*__slate_slot_781).offset((0 as i32) as isize) } }) as isize) };
{
}
loop {
std::ptr::write(__slate_slot_1633, (unsafe { *unsafe { (*__slate_slot_787).offset((0 as i32) as isize) } }) as u32);
*__slate_slot_790 = *__slate_slot_1633;
std::ptr::write(__slate_slot_1634, *__slate_slot_1633);
unsafe {
*unsafe { unsafe { std::ptr::addr_of_mut!((*(*__slate_slot_779)).aType) as *mut u32 }.offset(*__slate_slot_783 as isize) } = *__slate_slot_1634;
}
if *__slate_slot_1634 < ((128 as i32) as u32) {
std::ptr::write(__slate_slot_1635, *__slate_slot_787);
std::ptr::write(__slate_slot_1636, unsafe { (*__slate_slot_1635).offset((1 as i32) as isize) });
*__slate_slot_787 = *__slate_slot_1636;
std::ptr::write(__slate_slot_1637, *__slate_slot_789);
std::ptr::write(__slate_slot_1638, (*__slate_slot_1637).wrapping_add(((((unsafe { sqlite3VdbeOneByteSerialTypeLen(*__slate_slot_790 as u8) }) as u32) as i32) as i64) as u64));
*__slate_slot_789 = *__slate_slot_1638;
} else {
std::ptr::write(__slate_slot_1639, *__slate_slot_787);
std::ptr::write(__slate_slot_1640, unsafe { (*__slate_slot_1639).offset((((unsafe { sqlite3GetVarint32(*__slate_slot_787, std::ptr::addr_of_mut!(*__slate_slot_790)) }) as u32) as i32) as isize) });
*__slate_slot_787 = *__slate_slot_1640;
unsafe {
*unsafe { unsafe { std::ptr::addr_of_mut!((*(*__slate_slot_779)).aType) as *mut u32 }.offset(*__slate_slot_783 as isize) } = *__slate_slot_790;
}
std::ptr::write(__slate_slot_1641, *__slate_slot_789);
std::ptr::write(__slate_slot_1642, (*__slate_slot_1641).wrapping_add((unsafe { sqlite3VdbeSerialTypeLen(*__slate_slot_790) }) as u64));
*__slate_slot_789 = *__slate_slot_1642;
}
std::ptr::write(__slate_slot_1643, *__slate_slot_783);
std::ptr::write(__slate_slot_1644, *__slate_slot_1643 + (1 as i32));
*__slate_slot_783 = *__slate_slot_1644;
unsafe {
*unsafe { (*__slate_slot_781).offset(*__slate_slot_1644 as isize) } = (*__slate_slot_789 & ((4294967295 as u32) as u64)) as u32;
}
if !((*__slate_slot_783 as u32) <= *__slate_slot_778 && *__slate_slot_787 < *__slate_slot_788) {
break;
}
}
// The record is corrupt if any of the following are true:
// (1) the bytes of the header extend past the declared header size
// (2) the entire header was used but not all data was used
// (3) the end of the data extends beyond the end of the record.
if *__slate_slot_787 >= *__slate_slot_788 && (*__slate_slot_787 > *__slate_slot_788 || *__slate_slot_789 != ((unsafe { (*(*__slate_slot_779)).payloadSize }) as u64)) || *__slate_slot_789 > ((unsafe { (*(*__slate_slot_779)).payloadSize }) as u64) {
if (unsafe { *unsafe { (*__slate_slot_781).offset((0 as i32) as isize) } }) == ((0 as i32) as u32) {
*__slate_slot_783 = 0 as i32;
*__slate_slot_787 = *__slate_slot_788;
} else {
if (unsafe { (*(*__slate_slot_779)).aRow }) == std::ptr::null::<u8>() {
unsafe { sqlite3VdbeMemRelease(std::ptr::addr_of_mut!(*__slate_slot_785)) };
break '__join_789;
} else {
break '__join_789;
}
}
}
unsafe {
(*(*__slate_slot_779)).nHdrParsed = (*__slate_slot_783 as i16) as u16;
}
unsafe {
(*(*__slate_slot_779)).iHdrOffset = (((unsafe { (*__slate_slot_787).offset_from(*__slate_slot_786 as *const u8) }) as i64) as i32) as u32;
}
if (unsafe { (*(*__slate_slot_779)).aRow }) == std::ptr::null::<u8>() {
unsafe { sqlite3VdbeMemRelease(std::ptr::addr_of_mut!(*__slate_slot_785)) };
}
}
// If after trying to extract new entries from the header, nHdrParsed is
// still not up to p2, that means that the record has fewer than p2
// columns.  So the result will be either the default value or a NULL.
if ((((unsafe { (*(*__slate_slot_779)).nHdrParsed }) as u32) as i32) as u32) <= *__slate_slot_778 {
*__slate_slot_784 = unsafe { (*__slate_slot_717).offset((unsafe { (*(*__slate_slot_709)).p3 }) as isize) };
{
}
if ((unsafe { (*(*__slate_slot_709)).p4type }) as i32) == -(11 as i32) {
unsafe { sqlite3VdbeMemShallowCopy(*__slate_slot_784, (unsafe { (*(*__slate_slot_709)).p4.pMem }) as *const sqlite3_value, 8192 as i32) };
break '__join_790;
} else {
unsafe { sqlite3VdbeMemSetNull(*__slate_slot_784) };
break '__join_790;
}
}
}
// Extract the content for the p2+1-th column.  Control can only
// reach this point if aOffset[p2], aOffset[p2+1], and pC->aType[p2] are
// all valid.
0 as i32;
0 as i32;
*__slate_slot_784 = unsafe { (*__slate_slot_717).offset((unsafe { (*(*__slate_slot_709)).p3 }) as isize) };
{
}
0 as i32;
if (((unsafe { (*(*__slate_slot_784)).flags }) as u32) as i32) & ((32768 as i32) | (4096 as i32)) != (0 as i32) {
unsafe { sqlite3VdbeMemSetNull(*__slate_slot_784) };
}
0 as i32;
if (unsafe { (*(*__slate_slot_779)).szRow }) >= unsafe { *unsafe { (*__slate_slot_781).offset((*__slate_slot_778).wrapping_add((1 as i32) as u32) as isize) } } {
'__join_791: {
'__join_806: {
// This is the common case where the desired content fits on the original
// page - where the content is not on an overflow page.
//
// The big switch() is an in-line variant of sqlite3VdbeSerialGet() that
// has been optimized for the OP_Column opcode.
*__slate_slot_786 = unsafe { unsafe { (*(*__slate_slot_779)).aRow }.offset((unsafe { *unsafe { (*__slate_slot_781).offset(*__slate_slot_778 as isize) } }) as isize) };
let __t1: u32 = *__slate_slot_790;
if __t1 == (0 as u32) {
break '__join_806;
} else {
if __t1 == (11 as u32) {
break '__join_806;
} else {
if __t1 == (1 as u32) {
unsafe {
(*(*__slate_slot_784)).u.i = ((unsafe { *unsafe { (*__slate_slot_786).offset((0 as i32) as isize) } }) as i8) as i64;
}
unsafe {
(*(*__slate_slot_784)).flags = ((4 as i32) as i16) as u16;
}
{
}
break '__join_791;
} else {
if __t1 == (2 as u32) {
unsafe {
(*(*__slate_slot_784)).u.i = ((256 as i32) * (((unsafe { *unsafe { (*__slate_slot_786).offset((0 as i32) as isize) } }) as i8) as i32) | (((unsafe { *unsafe { (*__slate_slot_786).offset((1 as i32) as isize) } }) as u32) as i32)) as i64;
}
unsafe {
(*(*__slate_slot_784)).flags = ((4 as i32) as i16) as u16;
}
{
}
break '__join_791;
} else {
if __t1 == (3 as u32) {
unsafe {
(*(*__slate_slot_784)).u.i = ((65536 as i32) * (((unsafe { *unsafe { (*__slate_slot_786).offset((0 as i32) as isize) } }) as i8) as i32) | (((unsafe { *unsafe { (*__slate_slot_786).offset((1 as i32) as isize) } }) as u32) as i32) << (8 as i32) | (((unsafe { *unsafe { (*__slate_slot_786).offset((2 as i32) as isize) } }) as u32) as i32)) as i64;
}
unsafe {
(*(*__slate_slot_784)).flags = ((4 as i32) as i16) as u16;
}
{
}
break '__join_791;
} else {
if __t1 == (4 as u32) {
unsafe {
(*(*__slate_slot_784)).u.i = ((((unsafe { *unsafe { (*__slate_slot_786).offset((0 as i32) as isize) } }) as u32) << (24 as i32) | (((((unsafe { *unsafe { (*__slate_slot_786).offset((1 as i32) as isize) } }) as u32) as i32) << (16 as i32)) as u32) | (((((unsafe { *unsafe { (*__slate_slot_786).offset((2 as i32) as isize) } }) as u32) as i32) << (8 as i32)) as u32) | ((((unsafe { *unsafe { (*__slate_slot_786).offset((3 as i32) as isize) } }) as u32) as i32) as u32)) as i32) as i64;
}
unsafe {
(*(*__slate_slot_784)).flags = ((4 as i32) as i16) as u16;
}
{
}
break '__join_791;
} else {
if __t1 == (5 as u32) {
unsafe {
(*(*__slate_slot_784)).u.i = (((((unsafe { *unsafe { unsafe { (*__slate_slot_786).offset((2 as i32) as isize) }.offset((0 as i32) as isize) } }) as u32) << (24 as i32) | (((((unsafe { *unsafe { unsafe { (*__slate_slot_786).offset((2 as i32) as isize) }.offset((1 as i32) as isize) } }) as u32) as i32) << (16 as i32)) as u32) | (((((unsafe { *unsafe { unsafe { (*__slate_slot_786).offset((2 as i32) as isize) }.offset((2 as i32) as isize) } }) as u32) as i32) << (8 as i32)) as u32) | ((((unsafe { *unsafe { unsafe { (*__slate_slot_786).offset((2 as i32) as isize) }.offset((3 as i32) as isize) } }) as u32) as i32) as u32)) as u64) as i64) + (4294967296 as i64) * (((256 as i32) * (((unsafe { *unsafe { (*__slate_slot_786).offset((0 as i32) as isize) } }) as i8) as i32) | (((unsafe { *unsafe { (*__slate_slot_786).offset((1 as i32) as isize) } }) as u32) as i32)) as i64);
}
unsafe {
(*(*__slate_slot_784)).flags = ((4 as i32) as i16) as u16;
}
{
}
break '__join_791;
} else {
if __t1 == (6 as u32) {
unsafe {
(*(*__slate_slot_784)).u.i = (unsafe { sqlite3Get8byte(*__slate_slot_786) }) as i64;
}
unsafe {
(*(*__slate_slot_784)).flags = ((4 as i32) as i16) as u16;
}
{
}
break '__join_791;
} else {
if __t1 == (7 as u32) {
std::ptr::write(__slate_slot_793, unsafe { sqlite3Get8byte(*__slate_slot_786) });
{
}
unsafe {
(*(*__slate_slot_784)).flags = ((if *__slate_slot_793 & (((2047 as i32) as i64) as u64) << (52 as i32) == (((2047 as i32) as i64) as u64) << (52 as i32) && *__slate_slot_793 & ((((1 as i32) as i64) as u64) << (52 as i32)).wrapping_sub(((1 as i32) as i64) as u64) != (((0 as i32) as i64) as u64) { 1 as i32 } else { 8 as i32 }) as i16) as u16;
}
unsafe { memcpy((unsafe { std::ptr::addr_of_mut!((*(*__slate_slot_784)).u.r) }) as *mut (), std::ptr::addr_of_mut!(*__slate_slot_793) as *const (), 8 as u64) };
{
}
break '__join_791;
} else {
if __t1 == (8 as u32) {
} else {
if __t1 == (9 as u32) {
} else {
if __t1 == (10 as u32) {
unsafe {
(*(*__slate_slot_784)).flags = (((1 as i32) | (1024 as i32)) as i16) as u16;
}
// Internal use only: NULL with virtual table
// UPDATE no-change flag set
unsafe {
(*(*__slate_slot_784)).u.nZero = 0 as i32;
}
unsafe {
(*(*__slate_slot_784)).n = 0 as i32;
}
break '__join_791;
} else {
std::ptr::write(__slate_slot_1645, ((*__slate_slot_790).wrapping_sub((12 as i32) as u32) / ((2 as i32) as u32)) as i32);
*__slate_slot_782 = *__slate_slot_1645;
unsafe {
(*(*__slate_slot_784)).n = *__slate_slot_1645;
}
unsafe {
(*(*__slate_slot_784)).enc = *__slate_slot_713;
}
if (unsafe { (*(*__slate_slot_784)).szMalloc }) < *__slate_slot_782 + (2 as i32) {
if *__slate_slot_782 > unsafe { *unsafe { unsafe { (*(*__slate_slot_711)).aLimit.as_mut_ptr() as *mut i32 }.offset((0 as i32) as isize) } } {
break '__join_2;
} else {
unsafe {
(*(*__slate_slot_784)).flags = ((1 as i32) as i16) as u16;
}
if (unsafe { sqlite3VdbeMemGrow(*__slate_slot_784, *__slate_slot_782 + (2 as i32), 0 as i32) }) != (0 as i32) {
break '__join_1;
}
}
} else {
unsafe {
(*(*__slate_slot_784)).z = unsafe { (*(*__slate_slot_784)).zMalloc };
}
}
unsafe { memcpy((unsafe { (*(*__slate_slot_784)).z }) as *mut (), *__slate_slot_786 as *const (), (*__slate_slot_782 as i64) as u64) };
unsafe {
*unsafe { unsafe { (*(*__slate_slot_784)).z }.offset(*__slate_slot_782 as isize) } = (0 as i32) as i8;
}
unsafe {
*unsafe { unsafe { (*(*__slate_slot_784)).z }.offset((*__slate_slot_782 + (1 as i32)) as isize) } = (0 as i32) as i8;
}
unsafe {
(*(*__slate_slot_784)).flags = unsafe { *unsafe { unsafe { std::ptr::addr_of!(aFlag) as *const u16 }.offset((*__slate_slot_790 & ((1 as i32) as u32)) as isize) } };
}
break '__join_791;
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
unsafe {
(*(*__slate_slot_784)).u.i = ((*__slate_slot_790).wrapping_sub((8 as i32) as u32) as u64) as i64;
}
unsafe {
(*(*__slate_slot_784)).flags = ((4 as i32) as i16) as u16;
}
break '__join_791;
}
unsafe {
(*(*__slate_slot_784)).flags = ((1 as i32) as i16) as u16;
}
}
// End of switch
} else {
unsafe {
(*(*__slate_slot_784)).enc = *__slate_slot_713;
}
0 as i32;
std::ptr::write(__slate_slot_1646, (((((unsafe { (*(*__slate_slot_709)).p5 }) as u32) as i32) & (192 as i32)) as i8) as u8);
*__slate_slot_795 = *__slate_slot_1646;
if ((*__slate_slot_1646 as u32) as i32) != (0 as i32) && (((*__slate_slot_795 as u32) as i32) == (128 as i32) || *__slate_slot_790 >= ((12 as i32) as u32) && (*__slate_slot_790 & ((1 as i32) as u32) == ((0 as i32) as u32) || ((*__slate_slot_795 as u32) as i32) == (192 as i32))) {
*__slate_slot_1647 = true as bool;
} else {
*__slate_slot_1647 = (unsafe { sqlite3VdbeSerialTypeLen(*__slate_slot_790) }) == ((0 as i32) as u32);
}
if *__slate_slot_1647 {
// Content is irrelevant for
//    1. the typeof() function,
//    2. the length(X) function if X is a blob, and
//    3. if the content length is zero.
// So we might as well use bogus content rather than reading
// content from disk.
//
// Although sqlite3VdbeSerialGet() may read at most 8 bytes from the
// buffer passed to it, debugging function VdbeMemPrettyPrint() may
// read more.  Use the global constant sqlite3CtypeMap[] as the array,
// as that array is 256 bytes long (plenty for VdbeMemPrettyPrint())
// and it begins with a bunch of zeros.
unsafe { sqlite3VdbeSerialGet(((unsafe { std::ptr::addr_of!(sqlite3CtypeMap) as *const u8 }) as *mut u8) as *const u8, *__slate_slot_790, *__slate_slot_784) };
} else {
*__slate_slot_710 = vdbeColumnFromOverflow(*__slate_slot_779, *__slate_slot_778 as i32, *__slate_slot_790, ((unsafe { *unsafe { (*__slate_slot_781).offset(*__slate_slot_778 as isize) } }) as u64) as i64, unsafe { (*p).cacheCtr }, *__slate_slot_722, *__slate_slot_784);
if *__slate_slot_710 != (0 as i32) {
break '__join_810;
}
}
}
}
{
}
{
}
break '__join_26;
}
if (unsafe { (*unsafe { (*__slate_slot_708).offset((0 as i32) as isize) }).p3 }) > (0 as i32) {
*__slate_slot_709 = unsafe { (*__slate_slot_708).offset(((unsafe { (*unsafe { (*__slate_slot_708).offset((0 as i32) as isize) }).p3 }) - (1 as i32)) as isize) };
break '__join_26;
} else {
break '__join_788;
}
} else {
if __t4 == (97 as i32) {
0 as i32;
*__slate_slot_796 = unsafe { (*(*__slate_slot_709)).p4.pTab };
0 as i32;
0 as i32;
*__slate_slot_797 = unsafe { (*(*__slate_slot_796)).aCol };
*__slate_slot_718 = unsafe { (*__slate_slot_717).offset((unsafe { (*(*__slate_slot_709)).p1 }) as isize) };
if (unsafe { (*(*__slate_slot_709)).p3 }) < (2 as i32) {
0 as i32;
*__slate_slot_798 = 0 as i32;
*__slate_slot_799 = (unsafe { (*(*__slate_slot_796)).nCol }) as i32;
} else {
*__slate_slot_798 = (unsafe { (*(*__slate_slot_709)).p3 }) - (2 as i32);
*__slate_slot_799 = *__slate_slot_798 + (1 as i32);
0 as i32;
0 as i32;
0 as i32;
}
loop {
if *__slate_slot_798 < *__slate_slot_799 {
'__join_767: {
if (((unsafe { (*unsafe { (*__slate_slot_797).offset(*__slate_slot_798 as isize) }).colFlags }) as u32) as i32) & (96 as i32) != (0 as i32) && (unsafe { (*(*__slate_slot_709)).p3 }) < (2 as i32) {
if (((unsafe { (*unsafe { (*__slate_slot_797).offset(*__slate_slot_798 as isize) }).colFlags }) as u32) as i32) & (32 as i32) != (0 as i32) {
break '__join_767;
} else {
if (unsafe { (*(*__slate_slot_709)).p3 }) != (0 as i32) {
std::ptr::write(__slate_slot_1650, *__slate_slot_718);
std::ptr::write(__slate_slot_1651, unsafe { (*__slate_slot_1650).offset((1 as i32) as isize) });
*__slate_slot_718 = *__slate_slot_1651;
break '__join_767;
}
}
}
'__join_768: {
0 as i32;
applyAffinity(*__slate_slot_718, unsafe { (*unsafe { (*__slate_slot_797).offset(*__slate_slot_798 as isize) }).affinity }, *__slate_slot_713);
if (((unsafe { (*(*__slate_slot_718)).flags }) as u32) as i32) & (1 as i32) == (0 as i32) {
let __t0: i32 = (unsafe { (*unsafe { (*__slate_slot_797).offset(*__slate_slot_798 as isize) }).__slate_bits_0.__get_eCType() }) as i32;
if __t0 == (2 as i32) {
if (((unsafe { (*(*__slate_slot_718)).flags }) as u32) as i32) & (16 as i32) == (0 as i32) {
break '__join_764;
} else {
break '__join_768;
}
} else {
if __t0 == (4 as i32) {
} else {
if __t0 == (3 as i32) {
} else {
if __t0 == (6 as i32) {
if (((unsafe { (*(*__slate_slot_718)).flags }) as u32) as i32) & (2 as i32) == (0 as i32) {
break '__join_764;
} else {
break '__join_768;
}
} else {
if __t0 == (5 as i32) {
{
}
0 as i32;
if (((unsafe { (*(*__slate_slot_718)).flags }) as u32) as i32) & (4 as i32) != (0 as i32) {
// When applying REAL affinity, if the result is still an MEM_Int
// that will fit in 6 bytes, then change the type to MEM_IntReal
// so that we keep the high-resolution integer value but know that
// the type really wants to be REAL.
{
}
{
}
{
}
{
}
if (unsafe { (*(*__slate_slot_718)).u.i }) <= (140737488355327 as i64) && (unsafe { (*(*__slate_slot_718)).u.i }) >= -(140737488355328 as i64) {
std::ptr::write(__slate_slot_1652, *__slate_slot_718);
std::ptr::write(__slate_slot_1653, unsafe { (*(*__slate_slot_1652)).flags });
std::ptr::write(__slate_slot_1654, ((((*__slate_slot_1653 as u32) as i32) | (32 as i32)) as i16) as u16);
unsafe {
(*(*__slate_slot_1652)).flags = *__slate_slot_1654;
}
std::ptr::write(__slate_slot_1655, *__slate_slot_718);
std::ptr::write(__slate_slot_1656, unsafe { (*(*__slate_slot_1655)).flags });
std::ptr::write(__slate_slot_1657, ((((*__slate_slot_1656 as u32) as i32) & !(4 as i32)) as i16) as u16);
unsafe {
(*(*__slate_slot_1655)).flags = *__slate_slot_1657;
}
break '__join_768;
} else {
unsafe {
(*(*__slate_slot_718)).u.r = (unsafe { (*(*__slate_slot_718)).u.i }) as f64;
}
std::ptr::write(__slate_slot_1658, *__slate_slot_718);
std::ptr::write(__slate_slot_1659, unsafe { (*(*__slate_slot_1658)).flags });
std::ptr::write(__slate_slot_1660, ((((*__slate_slot_1659 as u32) as i32) | (8 as i32)) as i16) as u16);
unsafe {
(*(*__slate_slot_1658)).flags = *__slate_slot_1660;
}
std::ptr::write(__slate_slot_1661, *__slate_slot_718);
std::ptr::write(__slate_slot_1662, unsafe { (*(*__slate_slot_1661)).flags });
std::ptr::write(__slate_slot_1663, ((((*__slate_slot_1662 as u32) as i32) & !(4 as i32)) as i16) as u16);
unsafe {
(*(*__slate_slot_1661)).flags = *__slate_slot_1663;
}
break '__join_768;
}
} else {
if (((unsafe { (*(*__slate_slot_718)).flags }) as u32) as i32) & ((8 as i32) | (32 as i32)) == (0 as i32) {
break '__join_764;
} else {
break '__join_768;
}
}
} else {
// COLTYPE_ANY.  Accept anything.
break '__join_768;
}
}
}
}
}
if (((unsafe { (*(*__slate_slot_718)).flags }) as u32) as i32) & (4 as i32) == (0 as i32) {
break '__join_764;
}
}
}
{
}
std::ptr::write(__slate_slot_1664, *__slate_slot_718);
std::ptr::write(__slate_slot_1665, unsafe { (*__slate_slot_1664).offset((1 as i32) as isize) });
*__slate_slot_718 = *__slate_slot_1665;
}
std::ptr::write(__slate_slot_1648, *__slate_slot_798);
std::ptr::write(__slate_slot_1649, *__slate_slot_1648 + (1 as i32));
*__slate_slot_798 = *__slate_slot_1649;
} else {
break;
}
}
0 as i32;
break '__join_26;
} else {
if __t4 == (98 as i32) {
*__slate_slot_800 = (unsafe { (*(*__slate_slot_709)).p4.z }) as *const i8;
0 as i32;
0 as i32;
0 as i32;
*__slate_slot_718 = unsafe { (*__slate_slot_717).offset((unsafe { (*(*__slate_slot_709)).p1 }) as isize) };
loop {
if (1 as i32) != (0 as i32) {
// exit-by-break
0 as i32;
0 as i32;
applyAffinity(*__slate_slot_718, unsafe { *unsafe { (*__slate_slot_800).offset((0 as i32) as isize) } }, *__slate_slot_713);
if ((unsafe { *unsafe { (*__slate_slot_800).offset((0 as i32) as isize) } }) as i32) == (69 as i32) && (((unsafe { (*(*__slate_slot_718)).flags }) as u32) as i32) & (4 as i32) != (0 as i32) {
unsafe {
(*(*__slate_slot_718)).u.r = (unsafe { (*(*__slate_slot_718)).u.i }) as f64;
}
std::ptr::write(__slate_slot_1666, *__slate_slot_718);
std::ptr::write(__slate_slot_1667, unsafe { (*(*__slate_slot_1666)).flags });
std::ptr::write(__slate_slot_1668, ((((*__slate_slot_1667 as u32) as i32) | (8 as i32)) as i16) as u16);
unsafe {
(*(*__slate_slot_1666)).flags = *__slate_slot_1668;
}
std::ptr::write(__slate_slot_1669, *__slate_slot_718);
std::ptr::write(__slate_slot_1670, unsafe { (*(*__slate_slot_1669)).flags });
std::ptr::write(__slate_slot_1671, ((((*__slate_slot_1670 as u32) as i32) & !((4 as i32) | (2 as i32))) as i16) as u16);
unsafe {
(*(*__slate_slot_1669)).flags = *__slate_slot_1671;
}
}
{
}
std::ptr::write(__slate_slot_1672, *__slate_slot_800);
std::ptr::write(__slate_slot_1673, unsafe { (*__slate_slot_1672).offset((1 as i32) as isize) });
*__slate_slot_800 = *__slate_slot_1673;
if ((unsafe { *unsafe { (*__slate_slot_800).offset((0 as i32) as isize) } }) as i32) == (0 as i32) {
break '__join_26;
} else {
std::ptr::write(__slate_slot_1674, *__slate_slot_718);
std::ptr::write(__slate_slot_1675, unsafe { (*__slate_slot_1674).offset((1 as i32) as isize) });
*__slate_slot_718 = *__slate_slot_1675;
}
} else {
break '__join_26;
}
}
} else {
if __t4 == (99 as i32) {
'__join_752: {
// Assuming the record contains N fields, the record format looks
// like this:
//
// | hdr-size | type 0 | type 1 | ... | type N-1 | data0 | ... | data N-1 |
//
// Data(0) is taken from register P1.  Data(1) comes from register P1+1
// and so forth.
//
// Each type field is a varint representing the serial type of the
// corresponding data element (see sqlite3VdbeSerialType()). The
// hdr-size field is also a varint which is the offset from the beginning
// of the record to data0.
*__slate_slot_802 = ((0 as i32) as i64) as u64; // Number of bytes of data space
*__slate_slot_803 = 0 as i32; // Number of bytes of header space
*__slate_slot_805 = (0 as i32) as i64; // Number of zero bytes at the end of the record
*__slate_slot_810 = unsafe { (*(*__slate_slot_709)).p1 };
*__slate_slot_811 = unsafe { (*(*__slate_slot_709)).p4.z };
0 as i32;
*__slate_slot_808 = unsafe { (*__slate_slot_717).offset(*__slate_slot_810 as isize) };
*__slate_slot_810 = unsafe { (*(*__slate_slot_709)).p2 };
*__slate_slot_809 = unsafe { (*__slate_slot_808).offset((*__slate_slot_810 - (1 as i32)) as isize) };
// Identify the output register
0 as i32;
*__slate_slot_721 = unsafe { (*__slate_slot_717).offset((unsafe { (*(*__slate_slot_709)).p3 }) as isize) };
{
}
// Apply the requested affinity to all inputs
0 as i32;
if *__slate_slot_811 != std::ptr::null_mut::<i8>() {
*__slate_slot_801 = *__slate_slot_808;
loop {
applyAffinity(*__slate_slot_801, unsafe { *unsafe { (*__slate_slot_811).offset((0 as i32) as isize) } }, *__slate_slot_713);
if ((unsafe { *unsafe { (*__slate_slot_811).offset((0 as i32) as isize) } }) as i32) == (69 as i32) && (((unsafe { (*(*__slate_slot_801)).flags }) as u32) as i32) & (4 as i32) != (0 as i32) {
std::ptr::write(__slate_slot_1676, *__slate_slot_801);
std::ptr::write(__slate_slot_1677, unsafe { (*(*__slate_slot_1676)).flags });
std::ptr::write(__slate_slot_1678, ((((*__slate_slot_1677 as u32) as i32) | (32 as i32)) as i16) as u16);
unsafe {
(*(*__slate_slot_1676)).flags = *__slate_slot_1678;
}
std::ptr::write(__slate_slot_1679, *__slate_slot_801);
std::ptr::write(__slate_slot_1680, unsafe { (*(*__slate_slot_1679)).flags });
std::ptr::write(__slate_slot_1681, ((((*__slate_slot_1680 as u32) as i32) & !(4 as i32)) as i16) as u16);
unsafe {
(*(*__slate_slot_1679)).flags = *__slate_slot_1681;
}
}
{
}
std::ptr::write(__slate_slot_1682, *__slate_slot_811);
std::ptr::write(__slate_slot_1683, unsafe { (*__slate_slot_1682).offset((1 as i32) as isize) });
*__slate_slot_811 = *__slate_slot_1683;
std::ptr::write(__slate_slot_1684, *__slate_slot_801);
std::ptr::write(__slate_slot_1685, unsafe { (*__slate_slot_1684).offset((1 as i32) as isize) });
*__slate_slot_801 = *__slate_slot_1685;
0 as i32;
if !((unsafe { *unsafe { (*__slate_slot_811).offset((0 as i32) as isize) } }) != (0 as i8)) {
break '__join_752;
}
}
}
}
// Loop through the elements that will make up the record to figure
// out how much space is required for the new record.  After this loop,
// the Mem.uTemp field of each term should hold the serial-type that will
// be used for that term in the generated record:
//
//   Mem.uTemp value    type
//   ---------------    ---------------
//      0               NULL
//      1               1-byte signed integer
//      2               2-byte signed integer
//      3               3-byte signed integer
//      4               4-byte signed integer
//      5               6-byte signed integer
//      6               8-byte signed integer
//      7               IEEE float
//      8               Integer constant 0
//      9               Integer constant 1
//     10,11            reserved for expansion
//    N>=12 and even    BLOB
//    N>=13 and odd     text
//
// The following additional values are computed:
//     nHdr        Number of bytes needed for the record header
//     nData       Number of bytes of data space needed for the record
//     nZero       Zero bytes at the end of the record
*__slate_slot_801 = *__slate_slot_809;
loop {
0 as i32;
if (((unsafe { (*(*__slate_slot_801)).flags }) as u32) as i32) & (1 as i32) != (0 as i32) {
if (((unsafe { (*(*__slate_slot_801)).flags }) as u32) as i32) & (1024 as i32) != (0 as i32) {
// Values with MEM_Null and MEM_Zero are created by xColumn virtual
// table methods that never invoke sqlite3_result_xxxxx() while
// computing an unchanging column value in an UPDATE statement.
// Give such values a special internal-use-only serial-type of 10
// so that they can be passed through to xUpdate and have
// a true sqlite3_value_nochange().
0 as i32;
unsafe {
(*(*__slate_slot_801)).uTemp = (10 as i32) as u32;
}
} else {
unsafe {
(*(*__slate_slot_801)).uTemp = (0 as i32) as u32;
}
}
std::ptr::write(__slate_slot_1686, *__slate_slot_803);
std::ptr::write(__slate_slot_1687, *__slate_slot_1686 + (1 as i32));
*__slate_slot_803 = *__slate_slot_1687;
} else {
if (((unsafe { (*(*__slate_slot_801)).flags }) as u32) as i32) & ((4 as i32) | (32 as i32)) != (0 as i32) {
std::ptr::write(__slate_slot_815, unsafe { (*(*__slate_slot_801)).u.i });
{
}
{
}
if *__slate_slot_815 < ((0 as i32) as i64) {
*__slate_slot_816 = !(*__slate_slot_815) as u64;
} else {
*__slate_slot_816 = *__slate_slot_815 as u64;
}
std::ptr::write(__slate_slot_1688, *__slate_slot_803);
std::ptr::write(__slate_slot_1689, *__slate_slot_1688 + (1 as i32));
*__slate_slot_803 = *__slate_slot_1689;
{
}
{
}
{
}
{
}
{
}
{
}
{
}
{
}
{
}
{
}
if *__slate_slot_816 <= (((127 as i32) as i64) as u64) {
if *__slate_slot_815 & ((1 as i32) as i64) == *__slate_slot_815 && (((unsafe { (*p).minWriteFileFormat }) as u32) as i32) >= (4 as i32) {
unsafe {
(*(*__slate_slot_801)).uTemp = ((8 as i32) as u32).wrapping_add(*__slate_slot_816 as u32);
}
} else {
std::ptr::write(__slate_slot_1690, *__slate_slot_802);
std::ptr::write(__slate_slot_1691, (*__slate_slot_1690).wrapping_add(((1 as i32) as i64) as u64));
*__slate_slot_802 = *__slate_slot_1691;
unsafe {
(*(*__slate_slot_801)).uTemp = (1 as i32) as u32;
}
}
} else {
if *__slate_slot_816 <= (((32767 as i32) as i64) as u64) {
std::ptr::write(__slate_slot_1692, *__slate_slot_802);
std::ptr::write(__slate_slot_1693, (*__slate_slot_1692).wrapping_add(((2 as i32) as i64) as u64));
*__slate_slot_802 = *__slate_slot_1693;
unsafe {
(*(*__slate_slot_801)).uTemp = (2 as i32) as u32;
}
} else {
if *__slate_slot_816 <= (((8388607 as i32) as i64) as u64) {
std::ptr::write(__slate_slot_1694, *__slate_slot_802);
std::ptr::write(__slate_slot_1695, (*__slate_slot_1694).wrapping_add(((3 as i32) as i64) as u64));
*__slate_slot_802 = *__slate_slot_1695;
unsafe {
(*(*__slate_slot_801)).uTemp = (3 as i32) as u32;
}
} else {
if *__slate_slot_816 <= (((2147483647 as i32) as i64) as u64) {
std::ptr::write(__slate_slot_1696, *__slate_slot_802);
std::ptr::write(__slate_slot_1697, (*__slate_slot_1696).wrapping_add(((4 as i32) as i64) as u64));
*__slate_slot_802 = *__slate_slot_1697;
unsafe {
(*(*__slate_slot_801)).uTemp = (4 as i32) as u32;
}
} else {
if *__slate_slot_816 <= ((140737488355327 as i64) as u64) {
std::ptr::write(__slate_slot_1698, *__slate_slot_802);
std::ptr::write(__slate_slot_1699, (*__slate_slot_1698).wrapping_add(((6 as i32) as i64) as u64));
*__slate_slot_802 = *__slate_slot_1699;
unsafe {
(*(*__slate_slot_801)).uTemp = (5 as i32) as u32;
}
} else {
std::ptr::write(__slate_slot_1700, *__slate_slot_802);
std::ptr::write(__slate_slot_1701, (*__slate_slot_1700).wrapping_add(((8 as i32) as i64) as u64));
*__slate_slot_802 = *__slate_slot_1701;
if (((unsafe { (*(*__slate_slot_801)).flags }) as u32) as i32) & (32 as i32) != (0 as i32) {
// If the value is IntReal and is going to take up 8 bytes to store
// as an integer, then we might as well make it an 8-byte floating
// point value
unsafe {
(*(*__slate_slot_801)).u.r = (unsafe { (*(*__slate_slot_801)).u.i }) as f64;
}
std::ptr::write(__slate_slot_1702, *__slate_slot_801);
std::ptr::write(__slate_slot_1703, unsafe { (*(*__slate_slot_1702)).flags });
std::ptr::write(__slate_slot_1704, ((((*__slate_slot_1703 as u32) as i32) & !(32 as i32)) as i16) as u16);
unsafe {
(*(*__slate_slot_1702)).flags = *__slate_slot_1704;
}
std::ptr::write(__slate_slot_1705, *__slate_slot_801);
std::ptr::write(__slate_slot_1706, unsafe { (*(*__slate_slot_1705)).flags });
std::ptr::write(__slate_slot_1707, ((((*__slate_slot_1706 as u32) as i32) | (8 as i32)) as i16) as u16);
unsafe {
(*(*__slate_slot_1705)).flags = *__slate_slot_1707;
}
unsafe {
(*(*__slate_slot_801)).uTemp = (7 as i32) as u32;
}
} else {
unsafe {
(*(*__slate_slot_801)).uTemp = (6 as i32) as u32;
}
}
}
}
}
}
}
} else {
if (((unsafe { (*(*__slate_slot_801)).flags }) as u32) as i32) & (8 as i32) != (0 as i32) {
std::ptr::write(__slate_slot_1708, *__slate_slot_803);
std::ptr::write(__slate_slot_1709, *__slate_slot_1708 + (1 as i32));
*__slate_slot_803 = *__slate_slot_1709;
std::ptr::write(__slate_slot_1710, *__slate_slot_802);
std::ptr::write(__slate_slot_1711, (*__slate_slot_1710).wrapping_add(((8 as i32) as i64) as u64));
*__slate_slot_802 = *__slate_slot_1711;
unsafe {
(*(*__slate_slot_801)).uTemp = (7 as i32) as u32;
}
} else {
0 as i32;
0 as i32;
*__slate_slot_812 = (unsafe { (*(*__slate_slot_801)).n }) as u32;
*__slate_slot_807 = (*__slate_slot_812).wrapping_mul((2 as i32) as u32).wrapping_add((12 as i32) as u32).wrapping_add((((((unsafe { (*(*__slate_slot_801)).flags }) as u32) as i32) & (2 as i32) != (0 as i32)) as i32) as u32);
if (((unsafe { (*(*__slate_slot_801)).flags }) as u32) as i32) & (1024 as i32) != (0 as i32) {
std::ptr::write(__slate_slot_1712, *__slate_slot_807);
std::ptr::write(__slate_slot_1713, (*__slate_slot_1712).wrapping_add(((unsafe { (*(*__slate_slot_801)).u.nZero }) as u32).wrapping_mul((2 as i32) as u32)));
*__slate_slot_807 = *__slate_slot_1713;
if *__slate_slot_802 != (0 as u64) {
if (unsafe { sqlite3VdbeMemExpandBlob(*__slate_slot_801) }) != (0 as i32) {
break '__join_1;
} else {
std::ptr::write(__slate_slot_1714, *__slate_slot_812);
std::ptr::write(__slate_slot_1715, (*__slate_slot_1714).wrapping_add((unsafe { (*(*__slate_slot_801)).u.nZero }) as u32));
*__slate_slot_812 = *__slate_slot_1715;
}
} else {
std::ptr::write(__slate_slot_1716, *__slate_slot_805);
std::ptr::write(__slate_slot_1717, *__slate_slot_1716 + ((unsafe { (*(*__slate_slot_801)).u.nZero }) as i64));
*__slate_slot_805 = *__slate_slot_1717;
}
}
std::ptr::write(__slate_slot_1718, *__slate_slot_802);
std::ptr::write(__slate_slot_1719, (*__slate_slot_1718).wrapping_add(*__slate_slot_812 as u64));
*__slate_slot_802 = *__slate_slot_1719;
std::ptr::write(__slate_slot_1720, *__slate_slot_803);
std::ptr::write(__slate_slot_1721, *__slate_slot_1720 + unsafe { sqlite3VarintLen(*__slate_slot_807 as u64) });
*__slate_slot_803 = *__slate_slot_1721;
unsafe {
(*(*__slate_slot_801)).uTemp = *__slate_slot_807;
}
}
}
}
if *__slate_slot_801 == *__slate_slot_808 {
break;
} else {
std::ptr::write(__slate_slot_1722, *__slate_slot_801);
std::ptr::write(__slate_slot_1723, unsafe { (*__slate_slot_1722).offset(-((1 as i32) as isize)) });
*__slate_slot_801 = *__slate_slot_1723;
if !((1 as i32) != (0 as i32)) {
break;
}
}
}
// EVIDENCE-OF: R-22564-11647 The header begins with a single varint
// which determines the total number of bytes in the header. The varint
// value is the size of the header in bytes including the size varint
// itself.
{
}
{
}
if *__slate_slot_803 <= (126 as i32) {
std::ptr::write(__slate_slot_1724, *__slate_slot_803);
std::ptr::write(__slate_slot_1725, *__slate_slot_1724 + (1 as i32));
*__slate_slot_803 = *__slate_slot_1725;
} else {
// Rare case of a really large header
*__slate_slot_806 = unsafe { sqlite3VarintLen((*__slate_slot_803 as i64) as u64) };
std::ptr::write(__slate_slot_1726, *__slate_slot_803);
std::ptr::write(__slate_slot_1727, *__slate_slot_1726 + *__slate_slot_806);
*__slate_slot_803 = *__slate_slot_1727;
if *__slate_slot_806 < unsafe { sqlite3VarintLen((*__slate_slot_803 as i64) as u64) } {
std::ptr::write(__slate_slot_1728, *__slate_slot_803);
std::ptr::write(__slate_slot_1729, *__slate_slot_1728 + (1 as i32));
*__slate_slot_803 = *__slate_slot_1729;
}
}
*__slate_slot_804 = ((*__slate_slot_803 as i64) as u64).wrapping_add(*__slate_slot_802) as i64;
// If we are able to put an over-run area of 7 bytes on the end of the
// memory allocation into which the record is being constructed, then
// the encoding of integer values can go faster. This is only possible
// if SQLITE_MAX_LENGTH is no with 7 of INT32_MAX and if the host CPU
// byte-order is known at compile-time.
// We are able to allocate an overrun of 7 bytes
// Make sure the output register has a buffer large enough to store
// the new record. The output register (pOp->p3) is not allowed to
// be one of the input registers (because the following call to
// sqlite3VdbeMemClearAndResize() could clobber the value before it is used).
if *__slate_slot_804 + *__slate_slot_805 <= (((unsafe { (*(*__slate_slot_721)).szMalloc }) - (7 as i32)) as i64) {
// The output register is already large enough to hold the record.
// No error checks or buffer enlargement is required
unsafe {
(*(*__slate_slot_721)).z = unsafe { (*(*__slate_slot_721)).zMalloc };
}
} else {
// Need to make sure that the output is not too big and then enlarge
// the output register to hold the full result
if *__slate_slot_804 + *__slate_slot_805 > ((unsafe { *unsafe { unsafe { (*(*__slate_slot_711)).aLimit.as_mut_ptr() as *mut i32 }.offset((0 as i32) as isize) } }) as i64) {
break '__join_2;
} else {
if (unsafe { sqlite3VdbeMemClearAndResize(*__slate_slot_721, (*__slate_slot_804 as i32) + (7 as i32)) }) != (0 as i32) {
break '__join_1;
}
}
}
unsafe {
(*(*__slate_slot_721)).n = *__slate_slot_804 as i32;
}
unsafe {
(*(*__slate_slot_721)).flags = ((16 as i32) as i16) as u16;
}
if *__slate_slot_805 != (0 as i64) {
unsafe {
(*(*__slate_slot_721)).u.nZero = *__slate_slot_805 as i32;
}
std::ptr::write(__slate_slot_1730, *__slate_slot_721);
std::ptr::write(__slate_slot_1731, unsafe { (*(*__slate_slot_1730)).flags });
std::ptr::write(__slate_slot_1732, ((((*__slate_slot_1731 as u32) as i32) | (1024 as i32)) as i16) as u16);
unsafe {
(*(*__slate_slot_1730)).flags = *__slate_slot_1732;
}
}
{
}
*__slate_slot_813 = (unsafe { (*(*__slate_slot_721)).z }) as *mut u8;
*__slate_slot_814 = unsafe { (*__slate_slot_813).offset(*__slate_slot_803 as isize) };
// Write the record
if *__slate_slot_803 < (128 as i32) {
std::ptr::write(__slate_slot_1733, *__slate_slot_813);
std::ptr::write(__slate_slot_1734, unsafe { (*__slate_slot_1733).offset((1 as i32) as isize) });
*__slate_slot_813 = *__slate_slot_1734;
unsafe {
*(*__slate_slot_1733) = (*__slate_slot_803 as i8) as u8;
}
} else {
std::ptr::write(__slate_slot_1735, *__slate_slot_813);
std::ptr::write(__slate_slot_1736, unsafe { (*__slate_slot_1735).offset((unsafe { sqlite3PutVarint(*__slate_slot_813, (*__slate_slot_803 as i64) as u64) }) as isize) });
*__slate_slot_813 = *__slate_slot_1736;
}
0 as i32;
*__slate_slot_801 = *__slate_slot_808;
loop {
if (1 as i32) != (0 as i32) {
// exit-by-break
*__slate_slot_807 = unsafe { (*(*__slate_slot_801)).uTemp };
// EVIDENCE-OF: R-06529-47362 Following the size varint are one or more
// additional varints, one per column.
// EVIDENCE-OF: R-64536-51728 The values for each column in the record
// immediately follow the header.
if *__slate_slot_807 <= ((7 as i32) as u32) {
std::ptr::write(__slate_slot_1737, *__slate_slot_813);
std::ptr::write(__slate_slot_1738, unsafe { (*__slate_slot_1737).offset((1 as i32) as isize) });
*__slate_slot_813 = *__slate_slot_1738;
unsafe {
*(*__slate_slot_1737) = *__slate_slot_807 as u8;
}
if *__slate_slot_807 == ((0 as i32) as u32) {
// NULL value.  No change in zPayload
} else {
if *__slate_slot_807 == ((7 as i32) as u32) {
0 as i32;
unsafe { memcpy(std::ptr::addr_of_mut!(*__slate_slot_817) as *mut (), (unsafe { std::ptr::addr_of_mut!((*(*__slate_slot_801)).u.r) }) as *const (), 8 as u64) };
{
}
} else {
*__slate_slot_817 = (unsafe { (*(*__slate_slot_801)).u.i }) as u64;
}
*__slate_slot_812 = (unsafe { *unsafe { unsafe { std::ptr::addr_of!(sqlite3SmallTypeSizes) as *const u8 }.offset(*__slate_slot_807 as isize) } }) as u32;
0 as i32;
*__slate_slot_817 = unsafe { sqlite3BSwap64(*__slate_slot_817) };
if (7 as i32) != (0 as i32) {
std::ptr::write(__slate_slot_1739, *__slate_slot_817);
std::ptr::write(__slate_slot_1740, *__slate_slot_1739 >> (((unsafe { *unsafe { unsafe { std::ptr::addr_of!(aShift) as *const u8 }.offset(*__slate_slot_807 as isize) } }) as u32) as i32));
*__slate_slot_817 = *__slate_slot_1740;
unsafe { memcpy(*__slate_slot_814 as *mut (), std::ptr::addr_of_mut!(*__slate_slot_817) as *const (), ((8 as i32) as i64) as u64) };
} else {
// Test this limb by compiling with -DSQLITE_MAX_LENGTH=2147483647
unsafe { memcpy(*__slate_slot_814 as *mut (), (unsafe { unsafe { (std::ptr::addr_of_mut!(*__slate_slot_817) as *mut u8).offset((8 as i32) as isize) }.offset(-(*__slate_slot_812 as isize)) }) as *const (), *__slate_slot_812 as u64) };
}
// We are done with the OVERRUN macro now
std::ptr::write(__slate_slot_1741, *__slate_slot_814);
std::ptr::write(__slate_slot_1742, unsafe { (*__slate_slot_1741).offset(*__slate_slot_812 as isize) });
*__slate_slot_814 = *__slate_slot_1742;
}
} else {
if *__slate_slot_807 < ((128 as i32) as u32) {
std::ptr::write(__slate_slot_1743, *__slate_slot_813);
std::ptr::write(__slate_slot_1744, unsafe { (*__slate_slot_1743).offset((1 as i32) as isize) });
*__slate_slot_813 = *__slate_slot_1744;
unsafe {
*(*__slate_slot_1743) = *__slate_slot_807 as u8;
}
if *__slate_slot_807 >= ((14 as i32) as u32) && (unsafe { (*(*__slate_slot_801)).n }) > (0 as i32) {
0 as i32;
unsafe { memcpy(*__slate_slot_814 as *mut (), (unsafe { (*(*__slate_slot_801)).z }) as *const (), ((unsafe { (*(*__slate_slot_801)).n }) as i64) as u64) };
std::ptr::write(__slate_slot_1745, *__slate_slot_814);
std::ptr::write(__slate_slot_1746, unsafe { (*__slate_slot_1745).offset((unsafe { (*(*__slate_slot_801)).n }) as isize) });
*__slate_slot_814 = *__slate_slot_1746;
}
} else {
std::ptr::write(__slate_slot_1747, *__slate_slot_813);
std::ptr::write(__slate_slot_1748, unsafe { (*__slate_slot_1747).offset((unsafe { sqlite3PutVarint(*__slate_slot_813, *__slate_slot_807 as u64) }) as isize) });
*__slate_slot_813 = *__slate_slot_1748;
if (unsafe { (*(*__slate_slot_801)).n }) != (0 as i32) {
0 as i32;
0 as i32;
unsafe { memcpy(*__slate_slot_814 as *mut (), (unsafe { (*(*__slate_slot_801)).z }) as *const (), ((unsafe { (*(*__slate_slot_801)).n }) as i64) as u64) };
std::ptr::write(__slate_slot_1749, *__slate_slot_814);
std::ptr::write(__slate_slot_1750, unsafe { (*__slate_slot_1749).offset((unsafe { (*(*__slate_slot_801)).n }) as isize) });
*__slate_slot_814 = *__slate_slot_1750;
}
}
}
if *__slate_slot_801 == *__slate_slot_809 {
break;
} else {
std::ptr::write(__slate_slot_1751, *__slate_slot_801);
std::ptr::write(__slate_slot_1752, unsafe { (*__slate_slot_1751).offset((1 as i32) as isize) });
*__slate_slot_801 = *__slate_slot_1752;
}
} else {
break;
}
}
0 as i32;
0 as i32;
0 as i32;
{
}
break '__join_26;
} else {
if __t4 == (100 as i32) {
// out2
0 as i32;
*__slate_slot_820 = unsafe { (*unsafe { *unsafe { unsafe { (*p).apCsr }.offset((unsafe { (*(*__slate_slot_709)).p1 }) as isize) } }).uc.pCursor };
0 as i32;
if (unsafe { (*(*__slate_slot_709)).p3 }) != (0 as i32) {
*__slate_slot_819 = unsafe { sqlite3BtreeRowCountEst(*__slate_slot_820) };
} else {
*__slate_slot_819 = (0 as i32) as i64; // Not needed.  Only used to silence a warning.
*__slate_slot_710 = unsafe { sqlite3BtreeCount(*__slate_slot_711, *__slate_slot_820, std::ptr::addr_of_mut!(*__slate_slot_819)) };
if *__slate_slot_710 != (0 as i32) {
break '__join_1258;
}
}
*__slate_slot_721 = out2Prerelease(p, *__slate_slot_709);
unsafe {
(*(*__slate_slot_721)).u.i = *__slate_slot_819;
}
break '__join_1139;
} else {
if __t4 == (0 as i32) {
*__slate_slot_821 = unsafe { (*(*__slate_slot_709)).p1 };
*__slate_slot_822 = unsafe { (*(*__slate_slot_709)).p4.z };
// Assert that the p1 parameter is valid. Also that if there is no open
// transaction, then there cannot be any savepoints.
0 as i32;
0 as i32;
0 as i32;
0 as i32;
0 as i32;
if *__slate_slot_821 == (0 as i32) {
if (unsafe { (*(*__slate_slot_711)).nVdbeWrite }) > (0 as i32) {
// A new savepoint cannot be created if there are active write
// statements (i.e. open read/write incremental blob handles).
unsafe { sqlite3VdbeError(p, (b"cannot open savepoint - SQL statements in progress\0".as_ptr() as *mut i8) as *const i8) };
*__slate_slot_710 = 5 as i32;
} else {
*__slate_slot_823 = unsafe { sqlite3Strlen30(*__slate_slot_822 as *const i8) };
// This call is Ok even if this savepoint is actually a transaction
// savepoint (and therefore should not prompt xSavepoint()) callbacks.
// If this is a transaction savepoint being opened, it is guaranteed
// that the db->aVTrans[] array is empty.
0 as i32;
*__slate_slot_710 = unsafe { sqlite3VtabSavepoint(*__slate_slot_711, 0 as i32, (unsafe { (*(*__slate_slot_711)).nStatement }) + unsafe { (*(*__slate_slot_711)).nSavepoint }) };
if *__slate_slot_710 != (0 as i32) {
break '__join_1251;
} else {
// Create a new savepoint structure.
*__slate_slot_824 = (unsafe { sqlite3DbMallocRawNN(*__slate_slot_711, (32 as u64).wrapping_add((*__slate_slot_823 as i64) as u64).wrapping_add(((1 as i32) as i64) as u64)) }) as *mut Savepoint;
if *__slate_slot_824 != std::ptr::null_mut::<Savepoint>() {
unsafe {
(*(*__slate_slot_824)).zName = (unsafe { (*__slate_slot_824).offset((1 as i32) as isize) }) as *mut i8;
}
unsafe { memcpy((unsafe { (*(*__slate_slot_824)).zName }) as *mut (), *__slate_slot_822 as *const (), ((*__slate_slot_823 + (1 as i32)) as i64) as u64) };
// If there is no open transaction, then mark this as a special
// "transaction savepoint".
if (unsafe { (*(*__slate_slot_711)).autoCommit }) != (0 as u8) {
unsafe {
(*(*__slate_slot_711)).autoCommit = ((0 as i32) as i8) as u8;
}
unsafe {
(*(*__slate_slot_711)).isTransactionSavepoint = ((1 as i32) as i8) as u8;
}
} else {
std::ptr::write(__slate_slot_1753, *__slate_slot_711);
std::ptr::write(__slate_slot_1754, unsafe { (*(*__slate_slot_1753)).nSavepoint });
std::ptr::write(__slate_slot_1755, *__slate_slot_1754 + (1 as i32));
unsafe {
(*(*__slate_slot_1753)).nSavepoint = *__slate_slot_1755;
}
}
// Link the new savepoint into the database handle's list.
unsafe {
(*(*__slate_slot_824)).pNext = unsafe { (*(*__slate_slot_711)).pSavepoint };
}
unsafe {
(*(*__slate_slot_711)).pSavepoint = *__slate_slot_824;
}
unsafe {
(*(*__slate_slot_824)).nDeferredCons = unsafe { (*(*__slate_slot_711)).nDeferredCons };
}
unsafe {
(*(*__slate_slot_824)).nDeferredImmCons = unsafe { (*(*__slate_slot_711)).nDeferredImmCons };
}
}
}
}
} else {
0 as i32;
*__slate_slot_827 = 0 as i32;
// Find the named savepoint. If there is no such savepoint, then an
// an error is returned to the user.
*__slate_slot_825 = unsafe { (*(*__slate_slot_711)).pSavepoint };
loop {
if *__slate_slot_825 != std::ptr::null_mut::<Savepoint>() {
*__slate_slot_1756 = (unsafe { sqlite3StrICmp((unsafe { (*(*__slate_slot_825)).zName }) as *const i8, *__slate_slot_822 as *const i8) }) != (0 as i32);
} else {
*__slate_slot_1756 = false as bool;
}
if *__slate_slot_1756 {
std::ptr::write(__slate_slot_1757, *__slate_slot_827);
std::ptr::write(__slate_slot_1758, *__slate_slot_1757 + (1 as i32));
*__slate_slot_827 = *__slate_slot_1758;
*__slate_slot_825 = unsafe { (*(*__slate_slot_825)).pNext };
} else {
break;
}
}
if !(*__slate_slot_825 != std::ptr::null_mut::<Savepoint>()) {
unsafe { sqlite3VdbeError(p, (b"no such savepoint: %s\0".as_ptr() as *mut i8) as *const i8, *__slate_slot_822) };
*__slate_slot_710 = 1 as i32;
} else {
if (unsafe { (*(*__slate_slot_711)).nVdbeWrite }) > (0 as i32) && *__slate_slot_821 == (1 as i32) {
// It is not possible to release (commit) a savepoint if there are
// active write statements.
unsafe { sqlite3VdbeError(p, (b"cannot release savepoint - SQL statements in progress\0".as_ptr() as *mut i8) as *const i8) };
*__slate_slot_710 = 5 as i32;
} else {
std::ptr::write(__slate_slot_829, ((unsafe { (*(*__slate_slot_825)).pNext }) == std::ptr::null_mut::<Savepoint>() && (unsafe { (*(*__slate_slot_711)).isTransactionSavepoint }) != (0 as u8)) as i32);
if *__slate_slot_829 != (0 as i32) && *__slate_slot_821 == (1 as i32) {
std::ptr::write(__slate_slot_1759, unsafe { sqlite3VdbeCheckFkDeferred(p) });
*__slate_slot_710 = *__slate_slot_1759;
if *__slate_slot_1759 != (0 as i32) {
break '__join_1255;
} else {
unsafe {
(*(*__slate_slot_711)).autoCommit = ((1 as i32) as i8) as u8;
}
if (unsafe { sqlite3VdbeHalt(p) }) == (5 as i32) {
break '__join_656;
} else {
*__slate_slot_710 = unsafe { (*p).rc };
if *__slate_slot_710 != (0 as i32) {
unsafe {
(*(*__slate_slot_711)).autoCommit = ((0 as i32) as i8) as u8;
}
} else {
unsafe {
(*(*__slate_slot_711)).isTransactionSavepoint = ((0 as i32) as i8) as u8;
}
}
}
}
} else {
'__join_664: {
*__slate_slot_827 = (unsafe { (*(*__slate_slot_711)).nSavepoint }) - *__slate_slot_827 - (1 as i32);
if *__slate_slot_821 == (2 as i32) {
*__slate_slot_830 = ((unsafe { (*(*__slate_slot_711)).mDbFlags }) & ((1 as i32) as u32) != ((0 as i32) as u32)) as i32;
*__slate_slot_828 = 0 as i32;
loop {
if *__slate_slot_828 < unsafe { (*(*__slate_slot_711)).nDb } {
*__slate_slot_710 = unsafe { sqlite3BtreeTripAllCursors(unsafe { (*unsafe { unsafe { (*(*__slate_slot_711)).aDb }.offset(*__slate_slot_828 as isize) }).pBt }, (4 as i32) | (2 as i32) << (8 as i32), (*__slate_slot_830 == (0 as i32)) as i32) };
if *__slate_slot_710 != (0 as i32) {
break '__join_1257;
} else {
std::ptr::write(__slate_slot_1760, *__slate_slot_828);
std::ptr::write(__slate_slot_1761, *__slate_slot_1760 + (1 as i32));
*__slate_slot_828 = *__slate_slot_1761;
}
} else {
break '__join_664;
}
}
} else {
0 as i32;
*__slate_slot_830 = 0 as i32;
}
}
*__slate_slot_828 = 0 as i32;
loop {
if *__slate_slot_828 < unsafe { (*(*__slate_slot_711)).nDb } {
*__slate_slot_710 = unsafe { sqlite3BtreeSavepoint(unsafe { (*unsafe { unsafe { (*(*__slate_slot_711)).aDb }.offset(*__slate_slot_828 as isize) }).pBt }, *__slate_slot_821, *__slate_slot_827) };
if *__slate_slot_710 != (0 as i32) {
break '__join_1256;
} else {
std::ptr::write(__slate_slot_1762, *__slate_slot_828);
std::ptr::write(__slate_slot_1763, *__slate_slot_1762 + (1 as i32));
*__slate_slot_828 = *__slate_slot_1763;
}
} else {
break;
}
}
if *__slate_slot_830 != (0 as i32) {
unsafe { sqlite3ExpirePreparedStatements(*__slate_slot_711, 0 as i32) };
unsafe { sqlite3ResetAllSchemasOfConnection(*__slate_slot_711) };
std::ptr::write(__slate_slot_1764, *__slate_slot_711);
std::ptr::write(__slate_slot_1765, unsafe { (*(*__slate_slot_1764)).mDbFlags });
std::ptr::write(__slate_slot_1766, *__slate_slot_1765 | ((1 as i32) as u32));
unsafe {
(*(*__slate_slot_1764)).mDbFlags = *__slate_slot_1766;
}
}
}
if *__slate_slot_710 != (0 as i32) {
break '__join_1253;
} else {
// Regardless of whether this is a RELEASE or ROLLBACK, destroy all
// savepoints nested inside of the savepoint being operated on.
loop {
if (unsafe { (*(*__slate_slot_711)).pSavepoint }) != *__slate_slot_825 {
*__slate_slot_826 = unsafe { (*(*__slate_slot_711)).pSavepoint };
unsafe {
(*(*__slate_slot_711)).pSavepoint = unsafe { (*(*__slate_slot_826)).pNext };
}
unsafe { sqlite3DbFree(*__slate_slot_711, *__slate_slot_826 as *mut ()) };
std::ptr::write(__slate_slot_1767, *__slate_slot_711);
std::ptr::write(__slate_slot_1768, unsafe { (*(*__slate_slot_1767)).nSavepoint });
std::ptr::write(__slate_slot_1769, *__slate_slot_1768 - (1 as i32));
unsafe {
(*(*__slate_slot_1767)).nSavepoint = *__slate_slot_1769;
}
} else {
break;
}
}
// If it is a RELEASE, then destroy the savepoint being operated on
// too. If it is a ROLLBACK TO, then set the number of deferred
// constraint violations present in the database to the value stored
// when the savepoint was created.
if *__slate_slot_821 == (1 as i32) {
0 as i32;
unsafe {
(*(*__slate_slot_711)).pSavepoint = unsafe { (*(*__slate_slot_825)).pNext };
}
unsafe { sqlite3DbFree(*__slate_slot_711, *__slate_slot_825 as *mut ()) };
if !(*__slate_slot_829 != (0 as i32)) {
std::ptr::write(__slate_slot_1770, *__slate_slot_711);
std::ptr::write(__slate_slot_1771, unsafe { (*(*__slate_slot_1770)).nSavepoint });
std::ptr::write(__slate_slot_1772, *__slate_slot_1771 - (1 as i32));
unsafe {
(*(*__slate_slot_1770)).nSavepoint = *__slate_slot_1772;
}
}
} else {
0 as i32;
unsafe {
(*(*__slate_slot_711)).nDeferredCons = unsafe { (*(*__slate_slot_825)).nDeferredCons };
}
unsafe {
(*(*__slate_slot_711)).nDeferredImmCons = unsafe { (*(*__slate_slot_825)).nDeferredImmCons };
}
}
if !(*__slate_slot_829 != (0 as i32)) || *__slate_slot_821 == (2 as i32) {
*__slate_slot_710 = unsafe { sqlite3VtabSavepoint(*__slate_slot_711, *__slate_slot_821, *__slate_slot_827) };
if *__slate_slot_710 != (0 as i32) {
break '__join_1252;
}
}
}
}
}
}
if *__slate_slot_710 != (0 as i32) {
break '__join_1250;
} else {
if (((unsafe { (*p).eVdbeState }) as u32) as i32) == (3 as i32) {
break '__join_630;
} else {
break '__join_26;
}
}
} else {
if __t4 == (1 as i32) {
break '__join_629;
} else {
if __t4 == (2 as i32) {
std::ptr::write(__slate_slot_835, 0 as i32);
0 as i32;
0 as i32;
0 as i32;
0 as i32;
0 as i32;
0 as i32;
if (unsafe { (*(*__slate_slot_709)).p2 }) != (0 as i32) && (unsafe { (*(*__slate_slot_711)).flags }) & ((((1048576 as i32) as i64) as u64) | (((2 as i32) as i64) as u64) << (32 as i32)) != (((0 as i32) as i64) as u64) {
break '__join_615;
} else {
*__slate_slot_834 = unsafe { unsafe { (*(*__slate_slot_711)).aDb }.offset((unsafe { (*(*__slate_slot_709)).p1 }) as isize) };
*__slate_slot_833 = unsafe { (*(*__slate_slot_834)).pBt };
if *__slate_slot_833 != std::ptr::null_mut::<Btree>() {
*__slate_slot_710 = unsafe { sqlite3BtreeBeginTrans(*__slate_slot_833, unsafe { (*(*__slate_slot_709)).p2 }, std::ptr::addr_of_mut!(*__slate_slot_835)) };
{
}
{
}
if *__slate_slot_710 != (0 as i32) {
break '__join_610;
} else {
if ((unsafe { (*p).__slate_bits_0.__get_usesStmtJournal() }) as i32) != (0 as i32) && (unsafe { (*(*__slate_slot_709)).p2 }) != (0 as i32) && ((((unsafe { (*(*__slate_slot_711)).autoCommit }) as u32) as i32) == (0 as i32) || (unsafe { (*(*__slate_slot_711)).nVdbeRead }) > (1 as i32)) {
0 as i32;
if (unsafe { (*p).iStatement }) == (0 as i32) {
0 as i32;
std::ptr::write(__slate_slot_1774, *__slate_slot_711);
std::ptr::write(__slate_slot_1775, unsafe { (*(*__slate_slot_1774)).nStatement });
std::ptr::write(__slate_slot_1776, *__slate_slot_1775 + (1 as i32));
unsafe {
(*(*__slate_slot_1774)).nStatement = *__slate_slot_1776;
}
unsafe {
(*p).iStatement = (unsafe { (*(*__slate_slot_711)).nSavepoint }) + unsafe { (*(*__slate_slot_711)).nStatement };
}
}
*__slate_slot_710 = unsafe { sqlite3VtabSavepoint(*__slate_slot_711, 0 as i32, (unsafe { (*p).iStatement }) - (1 as i32)) };
if *__slate_slot_710 == (0 as i32) {
*__slate_slot_710 = unsafe { sqlite3BtreeBeginStmt(*__slate_slot_833, unsafe { (*p).iStatement }) };
}
// Store the current value of the database handles deferred constraint
// counter. If the statement transaction needs to be rolled back,
// the value of this counter needs to be restored too.
unsafe {
(*p).nStmtDefCons = unsafe { (*(*__slate_slot_711)).nDeferredCons };
}
unsafe {
(*p).nStmtDefImmCons = unsafe { (*(*__slate_slot_711)).nDeferredImmCons };
}
}
}
}
0 as i32;
if *__slate_slot_710 == (0 as i32) && (unsafe { (*(*__slate_slot_709)).p5 }) != (0 as u16) && (*__slate_slot_835 != unsafe { (*(*__slate_slot_709)).p3 } || (unsafe { (*unsafe { (*(*__slate_slot_834)).pSchema }).iGeneration }) != unsafe { (*(*__slate_slot_709)).p4.i }) {
// IMPLEMENTATION-OF: R-03189-51135 As each SQL statement runs, the schema
// version is checked to ensure that the schema has not changed since the
// SQL statement was prepared.
unsafe { sqlite3DbFree(*__slate_slot_711, (unsafe { (*p).zErrMsg }) as *mut ()) };
unsafe {
(*p).zErrMsg = unsafe { sqlite3DbStrDup(*__slate_slot_711, (b"database schema has changed\0".as_ptr() as *mut i8) as *const i8) };
}
// If the schema-cookie from the database file matches the cookie
// stored with the in-memory representation of the schema, do
// not reload the schema from the database file.
//
// If virtual-tables are in use, this is not just an optimization.
// Often, v-tables store their data in other SQLite tables, which
// are queried from within xNext() and other v-table methods using
// prepared queries. If such a query is out-of-date, we do not want to
// discard the database schema, as the user code implementing the
// v-table would have to be ready for the sqlite3_vtab structure itself
// to be invalidated whenever sqlite3_step() is called from within
// a v-table method.
if (unsafe { (*unsafe { (*unsafe { unsafe { (*(*__slate_slot_711)).aDb }.offset((unsafe { (*(*__slate_slot_709)).p1 }) as isize) }).pSchema }).schema_cookie }) != *__slate_slot_835 {
unsafe { sqlite3ResetOneSchema(*__slate_slot_711, unsafe { (*(*__slate_slot_709)).p1 }) };
}
unsafe {
(*p).__slate_bits_0.__set_expired((1 as i32) as u32);
}
*__slate_slot_710 = 17 as i32;
// Set changeCntOn to 0 to prevent the value returned by sqlite3_changes()
// from being modified in sqlite3VdbeHalt(). If this statement is
// reprepared, changeCntOn will be set again.
unsafe {
(*p).__slate_bits_0.__set_changeCntOn((0 as i32) as u32);
}
}
if *__slate_slot_710 != (0 as i32) {
break '__join_1238;
} else {
break '__join_26;
}
}
} else {
if __t4 == (101 as i32) {
// out2
0 as i32;
*__slate_slot_837 = unsafe { (*(*__slate_slot_709)).p1 };
*__slate_slot_838 = unsafe { (*(*__slate_slot_709)).p3 };
0 as i32;
0 as i32;
0 as i32;
0 as i32;
unsafe { sqlite3BtreeGetMeta(unsafe { (*unsafe { unsafe { (*(*__slate_slot_711)).aDb }.offset(*__slate_slot_837 as isize) }).pBt }, *__slate_slot_838, std::ptr::addr_of_mut!(*__slate_slot_836) as *mut u32) };
*__slate_slot_721 = out2Prerelease(p, *__slate_slot_709);
unsafe {
(*(*__slate_slot_721)).u.i = *__slate_slot_836 as i64;
}
break '__join_26;
} else {
if __t4 == (102 as i32) {
{
}
0 as i32;
0 as i32;
0 as i32;
0 as i32;
*__slate_slot_839 = unsafe { unsafe { (*(*__slate_slot_711)).aDb }.offset((unsafe { (*(*__slate_slot_709)).p1 }) as isize) };
0 as i32;
0 as i32;
// See note about index shifting on OP_ReadCookie
*__slate_slot_710 = unsafe { sqlite3BtreeUpdateMeta(unsafe { (*(*__slate_slot_839)).pBt }, unsafe { (*(*__slate_slot_709)).p2 }, (unsafe { (*(*__slate_slot_709)).p3 }) as u32) };
if (unsafe { (*(*__slate_slot_709)).p2 }) == (1 as i32) {
// When the schema cookie changes, record the new cookie internally
unsafe {
*((unsafe { std::ptr::addr_of_mut!((*unsafe { (*(*__slate_slot_839)).pSchema }).schema_cookie) }) as *mut u32) = unsafe { *((unsafe { std::ptr::addr_of_mut!((*(*__slate_slot_709)).p3) }) as *mut u32) }.wrapping_sub((((unsafe { (*(*__slate_slot_709)).p5 }) as u32) as i32) as u32);
}
std::ptr::write(__slate_slot_1777, *__slate_slot_711);
std::ptr::write(__slate_slot_1778, unsafe { (*(*__slate_slot_1777)).mDbFlags });
std::ptr::write(__slate_slot_1779, *__slate_slot_1778 | ((1 as i32) as u32));
unsafe {
(*(*__slate_slot_1777)).mDbFlags = *__slate_slot_1779;
}
unsafe { sqlite3FkClearTriggerCache(*__slate_slot_711, unsafe { (*(*__slate_slot_709)).p1 }) };
} else {
if (unsafe { (*(*__slate_slot_709)).p2 }) == (2 as i32) {
// Record changes in the file format
unsafe {
(*unsafe { (*(*__slate_slot_839)).pSchema }).file_format = ((unsafe { (*(*__slate_slot_709)).p3 }) as i8) as u8;
}
}
}
if (unsafe { (*(*__slate_slot_709)).p1 }) == (1 as i32) {
// Invalidate all prepared statements whenever the TEMP database
// schema is changed.  Ticket #1644
unsafe { sqlite3ExpirePreparedStatements(*__slate_slot_711, 0 as i32) };
unsafe {
(*p).__slate_bits_0.__set_expired((0 as i32) as u32);
}
}
if *__slate_slot_710 != (0 as i32) {
break '__join_1237;
} else {
break '__join_26;
}
} else {
if __t4 == (113 as i32) {
// ncycle
0 as i32;
0 as i32;
*__slate_slot_846 = unsafe { *unsafe { unsafe { (*p).apCsr }.offset((unsafe { (*(*__slate_slot_709)).p1 }) as isize) } };
if *__slate_slot_846 != std::ptr::null_mut::<VdbeCursor>() && (unsafe { (*(*__slate_slot_846)).pgnoRoot }) == ((unsafe { (*(*__slate_slot_709)).p2 }) as u32) {
0 as i32; // Guaranteed by the code generator
0 as i32;
unsafe { sqlite3BtreeClearCursor(unsafe { (*(*__slate_slot_846)).uc.pCursor }) };
break '__join_572;
} else {
// If the cursor is not currently open or is open on a different
// index, then fall through into OP_OpenRead to force a reopen
break '__join_586;
}
} else {
if __t4 == (114 as i32) {
break '__join_586;
} else {
if __t4 == (116 as i32) {
break '__join_586;
} else {
if __t4 == (117 as i32) {
// ncycle
*__slate_slot_848 = unsafe { *unsafe { unsafe { (*p).apCsr }.offset((unsafe { (*(*__slate_slot_709)).p2 }) as isize) } };
0 as i32;
0 as i32; // Only ephemeral cursors can be duplicated
*__slate_slot_849 = allocateCursor(p, unsafe { (*(*__slate_slot_709)).p1 }, (unsafe { (*(*__slate_slot_848)).nField }) as i32, ((0 as i32) as i8) as u8);
if *__slate_slot_849 == std::ptr::null_mut::<VdbeCursor>() {
break '__join_1;
} else {
unsafe {
(*(*__slate_slot_849)).nullRow = ((1 as i32) as i8) as u8;
}
unsafe {
(*(*__slate_slot_849)).__slate_bits_0.__set_isEphemeral((1 as i32) as u32);
}
unsafe {
(*(*__slate_slot_849)).pKeyInfo = unsafe { (*(*__slate_slot_848)).pKeyInfo };
}
unsafe {
(*(*__slate_slot_849)).isTable = unsafe { (*(*__slate_slot_848)).isTable };
}
unsafe {
(*(*__slate_slot_849)).pgnoRoot = unsafe { (*(*__slate_slot_848)).pgnoRoot };
}
unsafe {
(*(*__slate_slot_849)).__slate_bits_0.__set_isOrdered(((unsafe { (*(*__slate_slot_848)).__slate_bits_0.__get_isOrdered() }) as i32) as u32);
}
unsafe {
(*(*__slate_slot_849)).ub.pBtx = unsafe { (*(*__slate_slot_848)).ub.pBtx };
}
unsafe {
(*(*__slate_slot_849)).__slate_bits_0.__set_noReuse((1 as i32) as u32);
}
unsafe {
(*(*__slate_slot_848)).__slate_bits_0.__set_noReuse((1 as i32) as u32);
}
*__slate_slot_710 = unsafe { sqlite3BtreeCursor(unsafe { (*(*__slate_slot_849)).ub.pBtx }, unsafe { (*(*__slate_slot_849)).pgnoRoot }, 4 as i32, unsafe { (*(*__slate_slot_849)).pKeyInfo }, unsafe { (*(*__slate_slot_849)).uc.pCursor }) };
// The sqlite3BtreeCursor() routine can only fail for the first cursor
// opened for a database.  Since there is already an open cursor when this
// opcode is run, the sqlite3BtreeCursor() cannot fail
0 as i32;
break '__join_26;
}
} else {
if __t4 == (119 as i32) {
break '__join_569;
} else {
if __t4 == (120 as i32) {
break '__join_569;
} else {
if __t4 == (121 as i32) {
0 as i32;
0 as i32;
*__slate_slot_853 = allocateCursor(p, unsafe { (*(*__slate_slot_709)).p1 }, unsafe { (*(*__slate_slot_709)).p2 }, ((1 as i32) as i8) as u8);
if *__slate_slot_853 == std::ptr::null_mut::<VdbeCursor>() {
break '__join_1;
} else {
unsafe {
(*(*__slate_slot_853)).pKeyInfo = unsafe { (*(*__slate_slot_709)).p4.pKeyInfo };
}
0 as i32;
0 as i32;
*__slate_slot_710 = unsafe { sqlite3VdbeSorterInit(*__slate_slot_711, unsafe { (*(*__slate_slot_709)).p3 }, *__slate_slot_853) };
if *__slate_slot_710 != (0 as i32) {
break '__join_1233;
} else {
break '__join_26;
}
}
} else {
if __t4 == (122 as i32) {
0 as i32;
*__slate_slot_854 = unsafe { *unsafe { unsafe { (*p).apCsr }.offset((unsafe { (*(*__slate_slot_709)).p1 }) as isize) } };
0 as i32;
std::ptr::write(__slate_slot_1782, *__slate_slot_854);
std::ptr::write(__slate_slot_1783, unsafe { (*(*__slate_slot_1782)).seqCount });
std::ptr::write(__slate_slot_1784, *__slate_slot_1783 + ((1 as i32) as i64));
unsafe {
(*(*__slate_slot_1782)).seqCount = *__slate_slot_1784;
}
if *__slate_slot_1783 == ((0 as i32) as i64) {
break '__join_1125;
} else {
break '__join_26;
}
} else {
if __t4 == (123 as i32) {
0 as i32;
0 as i32;
*__slate_slot_855 = allocateCursor(p, unsafe { (*(*__slate_slot_709)).p1 }, unsafe { (*(*__slate_slot_709)).p3 }, ((3 as i32) as i8) as u8);
if *__slate_slot_855 == std::ptr::null_mut::<VdbeCursor>() {
break '__join_1;
} else {
unsafe {
(*(*__slate_slot_855)).nullRow = ((1 as i32) as i8) as u8;
}
unsafe {
(*(*__slate_slot_855)).seekResult = unsafe { (*(*__slate_slot_709)).p2 };
}
unsafe {
(*(*__slate_slot_855)).isTable = ((1 as i32) as i8) as u8;
}
// Give this pseudo-cursor a fake BtCursor pointer so that pCx
// can be safely passed to sqlite3VdbeCursorMoveto().  This avoids a test
// for pCx->eCurType==CURTYPE_BTREE inside of sqlite3VdbeCursorMoveto()
// which is a performance optimization
unsafe {
(*(*__slate_slot_855)).uc.pCursor = unsafe { sqlite3BtreeFakeValidCursor() };
}
0 as i32;
break '__join_26;
}
} else {
if __t4 == (124 as i32) {
// ncycle
0 as i32;
unsafe { sqlite3VdbeFreeCursor(p, unsafe { *unsafe { unsafe { (*p).apCsr }.offset((unsafe { (*(*__slate_slot_709)).p1 }) as isize) } }) };
unsafe {
*unsafe { unsafe { (*p).apCsr }.offset((unsafe { (*(*__slate_slot_709)).p1 }) as isize) } = std::ptr::null_mut::<VdbeCursor>();
}
break '__join_26;
} else {
if __t4 == (21 as i32) {
break '__join_546;
} else {
if __t4 == (22 as i32) {
break '__join_546;
} else {
if __t4 == (23 as i32) {
break '__join_546;
} else {
if __t4 == (24 as i32) {
break '__join_546;
} else {
if __t4 == (126 as i32) {
// ncycle
0 as i32;
// If pOp->p5 is clear, then pOp->p2 points to the first instruction past the
// OP_IdxGT that follows the OP_SeekGE. Otherwise, it points to the first
// opcode past the OP_SeekGE itself.
0 as i32;
0 as i32;
*__slate_slot_866 = unsafe { *unsafe { unsafe { (*p).apCsr }.offset((unsafe { (*unsafe { (*__slate_slot_709).offset((1 as i32) as isize) }).p1 }) as isize) } };
0 as i32;
0 as i32;
0 as i32;
if !((unsafe { sqlite3BtreeCursorIsValidNN(unsafe { (*(*__slate_slot_866)).uc.pCursor }) }) != (0 as i32)) {
break '__join_26;
} else {
*__slate_slot_868 = unsafe { (*(*__slate_slot_709)).p1 };
0 as i32;
(*__slate_slot_869).pKeyInfo = unsafe { (*(*__slate_slot_866)).pKeyInfo };
(*__slate_slot_869).nField = ((unsafe { (*unsafe { (*__slate_slot_709).offset((1 as i32) as isize) }).p4.i }) as i16) as u16;
(*__slate_slot_869).default_rc = (0 as i32) as i8;
(*__slate_slot_869).aMem = unsafe { (*__slate_slot_717).offset((unsafe { (*unsafe { (*__slate_slot_709).offset((1 as i32) as isize) }).p3 }) as isize) };
*__slate_slot_867 = 0 as i32; // Not needed.  Only used to silence a warning.
'__join_507: {
'__join_505: {
'__join_503: {
loop {
if (1 as i32) != (0 as i32) {
*__slate_slot_710 = unsafe { sqlite3VdbeIdxKeyCompare(*__slate_slot_711, *__slate_slot_866, std::ptr::addr_of_mut!(*__slate_slot_869), std::ptr::addr_of_mut!(*__slate_slot_867)) };
if *__slate_slot_710 != (0 as i32) {
break '__join_1227;
} else {
if *__slate_slot_867 > (0 as i32) && (((unsafe { (*(*__slate_slot_709)).p5 }) as u32) as i32) == (0 as i32) {
break '__join_507;
} else {
if *__slate_slot_867 >= (0 as i32) {
break '__join_505;
} else {
if *__slate_slot_868 <= (0 as i32) {
break '__join_503;
} else {
std::ptr::write(__slate_slot_1793, *__slate_slot_868);
std::ptr::write(__slate_slot_1794, *__slate_slot_1793 - (1 as i32));
*__slate_slot_868 = *__slate_slot_1794;
unsafe {
(*(*__slate_slot_866)).cacheStatus = (0 as i32) as u32;
}
*__slate_slot_710 = unsafe { sqlite3BtreeNext(unsafe { (*(*__slate_slot_866)).uc.pCursor }, 0 as i32) };
if *__slate_slot_710 != (0 as i32) {
break;
}
}
}
}
}
} else {
break '__join_26;
}
}
if *__slate_slot_710 == (101 as i32) {
*__slate_slot_710 = 0 as i32;
break '__join_507;
} else {
break '__join_1226;
}
}
{
}
break '__join_26;
}
// Jump to This.P2, bypassing the OP_SeekGE opcode
{
}
break '__join_1125;
}
{
}
// Jump to SeekGE.P2, ending the loop
std::ptr::write(__slate_slot_1791, *__slate_slot_709);
std::ptr::write(__slate_slot_1792, unsafe { (*__slate_slot_1791).offset((1 as i32) as isize) });
*__slate_slot_709 = *__slate_slot_1792;
break '__join_1125;
}
} else {
if __t4 == (127 as i32) {
// ncycle
0 as i32;
*__slate_slot_870 = unsafe { *unsafe { unsafe { (*p).apCsr }.offset((unsafe { (*(*__slate_slot_709)).p1 }) as isize) } };
0 as i32;
0 as i32;
if (((unsafe { (*(*__slate_slot_870)).seekHit }) as u32) as i32) < unsafe { (*(*__slate_slot_709)).p2 } {
unsafe {
(*(*__slate_slot_870)).seekHit = ((unsafe { (*(*__slate_slot_709)).p2 }) as i16) as u16;
}
break '__join_26;
} else {
if (((unsafe { (*(*__slate_slot_870)).seekHit }) as u32) as i32) > unsafe { (*(*__slate_slot_709)).p3 } {
unsafe {
(*(*__slate_slot_870)).seekHit = ((unsafe { (*(*__slate_slot_709)).p3 }) as i16) as u16;
}
break '__join_26;
} else {
break '__join_26;
}
}
} else {
if __t4 == (25 as i32) {
// jump
0 as i32;
*__slate_slot_871 = unsafe { *unsafe { unsafe { (*p).apCsr }.offset((unsafe { (*(*__slate_slot_709)).p1 }) as isize) } };
{
}
if *__slate_slot_871 == std::ptr::null_mut::<VdbeCursor>() || (unsafe { (*(*__slate_slot_871)).nullRow }) != (0 as u8) {
break '__join_1140;
} else {
break '__join_26;
}
} else {
if __t4 == (26 as i32) {
// jump, in3, ncycle
0 as i32;
*__slate_slot_872 = unsafe { *unsafe { unsafe { (*p).apCsr }.offset((unsafe { (*(*__slate_slot_709)).p1 }) as isize) } };
0 as i32;
if (((unsafe { (*(*__slate_slot_872)).seekHit }) as u32) as i32) >= unsafe { (*(*__slate_slot_709)).p4.i } {
break '__join_26;
} else {
// Fall through into OP_NotFound
//
// no break
{
}
break '__join_491;
}
} else {
if __t4 == (27 as i32) {
break '__join_491;
} else {
if __t4 == (28 as i32) {
break '__join_491;
} else {
if __t4 == (29 as i32) {
break '__join_491;
} else {
if __t4 == (30 as i32) {
// jump0, in3, ncycle
*__slate_slot_720 = unsafe { (*__slate_slot_717).offset((unsafe { (*(*__slate_slot_709)).p3 }) as isize) };
{
}
{
}
{
}
{
}
if (((unsafe { (*(*__slate_slot_720)).flags }) as u32) as i32) & ((4 as i32) | (32 as i32)) == (0 as i32) {
'__join_467: {
std::ptr::write(__slate_slot_882, unsafe { *unsafe { (*__slate_slot_720).offset((0 as i32) as isize) } });
if (((*__slate_slot_882).flags as u32) as i32) & (2 as i32) != (0 as i32) {
applyNumericAffinity(std::ptr::addr_of_mut!(*__slate_slot_882), 1 as i32);
}
}
if (((*__slate_slot_882).flags as u32) as i32) & (4 as i32) != (0 as i32) {
*__slate_slot_881 = unsafe { (*__slate_slot_882).u.i };
break '__join_459;
} else {
if (((*__slate_slot_882).flags as u32) as i32) & (8 as i32) == (0 as i32) || (unsafe { (*__slate_slot_882).u.r }) < -9.223372036854776e18f64 || (unsafe { (*__slate_slot_882).u.r }) > 9.223372036854776e18f64 {
*__slate_slot_1798 = true as bool;
} else {
std::ptr::write(__slate_slot_1799, unsafe { sqlite3RealToI64(unsafe { (*__slate_slot_882).u.r }) });
*__slate_slot_881 = *__slate_slot_1799;
*__slate_slot_1798 = (*__slate_slot_1799 as f64) != unsafe { (*__slate_slot_882).u.r };
}
if *__slate_slot_1798 {
break '__join_1125;
} else {
break '__join_459;
}
}
} else {
// Fall through into OP_NotExists
//
// no break
{
}
break '__join_460;
}
} else {
if __t4 == (31 as i32) {
break '__join_460;
} else {
if __t4 == (128 as i32) {
// out2
0 as i32;
0 as i32;
0 as i32;
*__slate_slot_721 = out2Prerelease(p, *__slate_slot_709);
std::ptr::write(__slate_slot_1800, unsafe { *unsafe { unsafe { (*p).apCsr }.offset((unsafe { (*(*__slate_slot_709)).p1 }) as isize) } });
std::ptr::write(__slate_slot_1801, unsafe { (*(*__slate_slot_1800)).seqCount });
std::ptr::write(__slate_slot_1802, *__slate_slot_1801 + ((1 as i32) as i64));
unsafe {
(*(*__slate_slot_1800)).seqCount = *__slate_slot_1802;
}
unsafe {
(*(*__slate_slot_721)).u.i = *__slate_slot_1801;
}
break '__join_26;
} else {
if __t4 == (129 as i32) {
// out2
*__slate_slot_883 = (0 as i32) as i64;
*__slate_slot_885 = 0 as i32;
*__slate_slot_721 = out2Prerelease(p, *__slate_slot_709);
0 as i32;
*__slate_slot_884 = unsafe { *unsafe { unsafe { (*p).apCsr }.offset((unsafe { (*(*__slate_slot_709)).p1 }) as isize) } };
0 as i32;
0 as i32;
0 as i32;
0 as i32;
// The next rowid or record number (different terms for the same
// thing) is obtained in a two-step algorithm.
//
// First we attempt to find the largest existing rowid and add one
// to that.  But if the largest existing rowid is already the maximum
// positive integer, we have to fall through to the second
// probabilistic algorithm
//
// The second algorithm is to select a rowid at random and see if
// it already exists in the table.  If it does not exist, we have
// succeeded.  If the random rowid does exist, we select a new one
// and try again, up to 100 times.
0 as i32;
// Some compilers complain about constants of the form 0x7fffffffffffffff.
// Others complain about 0x7ffffffffffffffffLL.  The following macro seems
// to provide the constant while making all compilers happy.
if !(((unsafe { (*(*__slate_slot_884)).__slate_bits_0.__get_useRandomRowid() }) as i32) != (0 as i32)) {
*__slate_slot_710 = unsafe { sqlite3BtreeLast(unsafe { (*(*__slate_slot_884)).uc.pCursor }, std::ptr::addr_of_mut!(*__slate_slot_885)) };
if *__slate_slot_710 != (0 as i32) {
break '__join_1223;
} else {
if *__slate_slot_885 != (0 as i32) {
*__slate_slot_883 = (1 as i32) as i64; // IMP: R-61914-48074
} else {
0 as i32;
*__slate_slot_883 = unsafe { sqlite3BtreeIntegerKey(unsafe { (*(*__slate_slot_884)).uc.pCursor }) };
if *__slate_slot_883 >= (((((2147483647 as i32) as i64) as u64) << (32 as i32) | ((4294967295 as u32) as u64)) as i64) {
unsafe {
(*(*__slate_slot_884)).__slate_bits_0.__set_useRandomRowid((1 as i32) as u32);
}
} else {
std::ptr::write(__slate_slot_1803, *__slate_slot_883);
std::ptr::write(__slate_slot_1804, *__slate_slot_1803 + ((1 as i32) as i64));
*__slate_slot_883 = *__slate_slot_1804; // IMP: R-29538-34987
}
}
}
}
if (unsafe { (*(*__slate_slot_709)).p3 }) != (0 as i32) {
// Assert that P3 is a valid memory cell.
0 as i32;
if (unsafe { (*p).pFrame }) != std::ptr::null_mut::<VdbeFrame>() {
*__slate_slot_888 = unsafe { (*p).pFrame };
loop {
if (unsafe { (*(*__slate_slot_888)).pParent }) != std::ptr::null_mut::<VdbeFrame>() {
{
}
*__slate_slot_888 = unsafe { (*(*__slate_slot_888)).pParent };
} else {
break;
}
}
// Assert that P3 is a valid memory cell.
0 as i32;
*__slate_slot_887 = unsafe { unsafe { (*(*__slate_slot_888)).aMem }.offset((unsafe { (*(*__slate_slot_709)).p3 }) as isize) };
} else {
// Assert that P3 is a valid memory cell.
0 as i32;
*__slate_slot_887 = unsafe { (*__slate_slot_717).offset((unsafe { (*(*__slate_slot_709)).p3 }) as isize) };
{
}
}
0 as i32;
{
}
unsafe { sqlite3VdbeMemIntegerify(*__slate_slot_887) };
0 as i32; // mem(P3) holds an integer
if (unsafe { (*(*__slate_slot_887)).u.i }) == (((((2147483647 as i32) as i64) as u64) << (32 as i32) | ((4294967295 as u32) as u64)) as i64) || ((unsafe { (*(*__slate_slot_884)).__slate_bits_0.__get_useRandomRowid() }) as i32) != (0 as i32) {
break '__join_439;
} else {
if *__slate_slot_883 < (unsafe { (*(*__slate_slot_887)).u.i }) + ((1 as i32) as i64) {
*__slate_slot_883 = (unsafe { (*(*__slate_slot_887)).u.i }) + ((1 as i32) as i64);
}
unsafe {
(*(*__slate_slot_887)).u.i = *__slate_slot_883;
}
}
}
if ((unsafe { (*(*__slate_slot_884)).__slate_bits_0.__get_useRandomRowid() }) as i32) != (0 as i32) {
// IMPLEMENTATION-OF: R-07677-41881 If the largest ROWID is equal to the
// largest possible integer (9223372036854775807) then the database
// engine starts picking positive candidate ROWIDs at random until
// it finds one that is not previously used.
0 as i32;
// We cannot be in random rowid mode if this is
// an AUTOINCREMENT table.
*__slate_slot_886 = 0 as i32;
loop {
unsafe { sqlite3_randomness(((8 as u64) as u32) as i32, std::ptr::addr_of_mut!(*__slate_slot_883) as *mut ()) };
std::ptr::write(__slate_slot_1805, *__slate_slot_883);
std::ptr::write(__slate_slot_1806, *__slate_slot_1805 & (((((2147483647 as i32) as i64) as u64) << (32 as i32) | ((4294967295 as u32) as u64)) as i64) >> (1 as i32));
*__slate_slot_883 = *__slate_slot_1806;
std::ptr::write(__slate_slot_1807, *__slate_slot_883);
std::ptr::write(__slate_slot_1808, *__slate_slot_1807 + ((1 as i32) as i64));
*__slate_slot_883 = *__slate_slot_1808; // Ensure that v is greater than zero
std::ptr::write(__slate_slot_1809, unsafe { sqlite3BtreeTableMoveto(unsafe { (*(*__slate_slot_884)).uc.pCursor }, (*__slate_slot_883 as u64) as i64, 0 as i32, std::ptr::addr_of_mut!(*__slate_slot_885)) });
*__slate_slot_710 = *__slate_slot_1809;
if *__slate_slot_1809 == (0 as i32) && *__slate_slot_885 == (0 as i32) {
std::ptr::write(__slate_slot_1811, *__slate_slot_886);
std::ptr::write(__slate_slot_1812, *__slate_slot_1811 + (1 as i32));
*__slate_slot_886 = *__slate_slot_1812;
*__slate_slot_1810 = *__slate_slot_1812 < (100 as i32);
} else {
*__slate_slot_1810 = false as bool;
}
if !(*__slate_slot_1810) {
break;
}
}
if *__slate_slot_710 != (0 as i32) {
break '__join_1221;
} else {
if *__slate_slot_885 == (0 as i32) {
break '__join_427;
} else {
0 as i32; // EV: R-40812-03570
}
}
}
unsafe {
(*(*__slate_slot_884)).deferredMoveto = ((0 as i32) as i8) as u8;
}
unsafe {
(*(*__slate_slot_884)).cacheStatus = (0 as i32) as u32;
}
unsafe {
(*(*__slate_slot_721)).u.i = *__slate_slot_883;
}
break '__join_26;
} else {
if __t4 == (130 as i32) {
*__slate_slot_889 = unsafe { (*__slate_slot_717).offset((unsafe { (*(*__slate_slot_709)).p2 }) as isize) };
0 as i32;
0 as i32;
*__slate_slot_891 = unsafe { *unsafe { unsafe { (*p).apCsr }.offset((unsafe { (*(*__slate_slot_709)).p1 }) as isize) } };
0 as i32;
0 as i32;
0 as i32;
0 as i32;
0 as i32;
0 as i32;
{
}
{
}
*__slate_slot_890 = unsafe { (*__slate_slot_717).offset((unsafe { (*(*__slate_slot_709)).p3 }) as isize) };
0 as i32;
0 as i32;
{
}
(*__slate_slot_895).nKey = unsafe { (*(*__slate_slot_890)).u.i };
if ((unsafe { (*(*__slate_slot_709)).p4type }) as i32) == -(5 as i32) && (unsafe { (*(*__slate_slot_711)).xUpdateCallback }) != None {
0 as i32;
*__slate_slot_893 = (unsafe { (*unsafe { unsafe { (*(*__slate_slot_711)).aDb }.offset(((unsafe { (*(*__slate_slot_891)).iDb }) as i32) as isize) }).zDbSName }) as *const i8;
*__slate_slot_894 = unsafe { (*(*__slate_slot_709)).p4.pTab };
0 as i32;
} else {
*__slate_slot_894 = std::ptr::null_mut::<Table>();
*__slate_slot_893 = std::ptr::null::<i8>();
}
0 as i32;
if (((unsafe { (*(*__slate_slot_709)).p5 }) as u32) as i32) & (1 as i32) != (0 as i32) {
std::ptr::write(__slate_slot_1813, p);
std::ptr::write(__slate_slot_1814, unsafe { (*(*__slate_slot_1813)).nChange });
std::ptr::write(__slate_slot_1815, *__slate_slot_1814 + ((1 as i32) as i64));
unsafe {
(*(*__slate_slot_1813)).nChange = *__slate_slot_1815;
}
if (((unsafe { (*(*__slate_slot_709)).p5 }) as u32) as i32) & (32 as i32) != (0 as i32) {
unsafe {
(*(*__slate_slot_711)).lastRowid = (*__slate_slot_895).nKey;
}
}
}
0 as i32;
(*__slate_slot_895).pData = (unsafe { (*(*__slate_slot_889)).z }) as *const ();
(*__slate_slot_895).nData = unsafe { (*(*__slate_slot_889)).n };
*__slate_slot_892 = if (((unsafe { (*(*__slate_slot_709)).p5 }) as u32) as i32) & (16 as i32) != (0 as i32) { unsafe { (*(*__slate_slot_891)).seekResult } } else { 0 as i32 };
if (((unsafe { (*(*__slate_slot_889)).flags }) as u32) as i32) & (1024 as i32) != (0 as i32) {
(*__slate_slot_895).nZero = unsafe { (*(*__slate_slot_889)).u.nZero };
} else {
(*__slate_slot_895).nZero = 0 as i32;
}
(*__slate_slot_895).pKey = std::ptr::null::<()>();
0 as i32;
*__slate_slot_710 = unsafe { sqlite3BtreeInsert(unsafe { (*(*__slate_slot_891)).uc.pCursor }, std::ptr::addr_of_mut!(*__slate_slot_895) as *const BtreePayload, (((unsafe { (*(*__slate_slot_709)).p5 }) as u32) as i32) & ((8 as i32) | (2 as i32) | (128 as i32)), *__slate_slot_892) };
unsafe {
(*(*__slate_slot_891)).deferredMoveto = ((0 as i32) as i8) as u8;
}
unsafe {
(*(*__slate_slot_891)).cacheStatus = (0 as i32) as u32;
}
std::ptr::write(__slate_slot_1816, *__slate_slot_722);
std::ptr::write(__slate_slot_1817, (*__slate_slot_1816).wrapping_add((1 as i32) as u32));
*__slate_slot_722 = *__slate_slot_1817;
// Invoke the update-hook if required.
if *__slate_slot_710 != (0 as i32) {
break '__join_1219;
} else {
if *__slate_slot_894 != std::ptr::null_mut::<Table>() {
0 as i32;
0 as i32;
unsafe { unsafe { (*(*__slate_slot_711)).xUpdateCallback }.unwrap()(unsafe { (*(*__slate_slot_711)).pUpdateArg }, if (((unsafe { (*(*__slate_slot_709)).p5 }) as u32) as i32) & (4 as i32) != (0 as i32) { 23 as i32 } else { 18 as i32 }, *__slate_slot_893, (unsafe { (*(*__slate_slot_894)).zName }) as *const i8, (*__slate_slot_895).nKey) };
break '__join_26;
} else {
break '__join_26;
}
}
} else {
if __t4 == (131 as i32) {
0 as i32;
0 as i32;
0 as i32;
0 as i32;
*__slate_slot_896 = unsafe { *unsafe { unsafe { (*p).apCsr }.offset((unsafe { (*(*__slate_slot_709)).p1 }) as isize) } };
*__slate_slot_897 = unsafe { *unsafe { unsafe { (*p).apCsr }.offset((unsafe { (*(*__slate_slot_709)).p2 }) as isize) } };
*__slate_slot_898 = if (unsafe { (*(*__slate_slot_709)).p3 }) != (0 as i32) { unsafe { (*unsafe { (*__slate_slot_717).offset((unsafe { (*(*__slate_slot_709)).p3 }) as isize) }).u.i } } else { (0 as i32) as i64 };
*__slate_slot_710 = unsafe { sqlite3BtreeTransferRow(unsafe { (*(*__slate_slot_896)).uc.pCursor }, unsafe { (*(*__slate_slot_897)).uc.pCursor }, *__slate_slot_898) };
if *__slate_slot_710 != (0 as i32) {
break '__join_1218;
} else {
break '__join_26;
}
} else {
if __t4 == (132 as i32) {
*__slate_slot_902 = unsafe { (*(*__slate_slot_709)).p2 };
0 as i32;
*__slate_slot_899 = unsafe { *unsafe { unsafe { (*p).apCsr }.offset((unsafe { (*(*__slate_slot_709)).p1 }) as isize) } };
0 as i32;
0 as i32;
0 as i32;
0 as i32;
{
}
// If the update-hook or pre-update-hook will be invoked, set zDb to
// the name of the db to pass as to it. Also set local pTab to a copy
// of p4.pTab. Finally, if p5 is true, indicating that this cursor was
// last moved with OP_Next or OP_Prev, not Seek or NotFound, set
// VdbeCursor.movetoTarget to the current rowid.
if ((unsafe { (*(*__slate_slot_709)).p4type }) as i32) == -(5 as i32) && (unsafe { (*(*__slate_slot_711)).xUpdateCallback }) != None {
0 as i32;
0 as i32;
*__slate_slot_900 = (unsafe { (*unsafe { unsafe { (*(*__slate_slot_711)).aDb }.offset(((unsafe { (*(*__slate_slot_899)).iDb }) as i32) as isize) }).zDbSName }) as *const i8;
*__slate_slot_901 = unsafe { (*(*__slate_slot_709)).p4.pTab };
if (((unsafe { (*(*__slate_slot_709)).p5 }) as u32) as i32) & (2 as i32) != (0 as i32) && (unsafe { (*(*__slate_slot_899)).isTable }) != (0 as u8) {
unsafe {
(*(*__slate_slot_899)).movetoTarget = unsafe { sqlite3BtreeIntegerKey(unsafe { (*(*__slate_slot_899)).uc.pCursor }) };
}
}
} else {
*__slate_slot_900 = std::ptr::null::<i8>();
*__slate_slot_901 = std::ptr::null_mut::<Table>();
}
// Only flags that can be set are SAVEPOISTION and AUXDELETE
0 as i32;
0 as i32;
0 as i32;
*__slate_slot_710 = unsafe { sqlite3BtreeDelete(unsafe { (*(*__slate_slot_899)).uc.pCursor }, (unsafe { (*(*__slate_slot_709)).p5 }) as u8) };
unsafe {
(*(*__slate_slot_899)).cacheStatus = (0 as i32) as u32;
}
std::ptr::write(__slate_slot_1818, *__slate_slot_722);
std::ptr::write(__slate_slot_1819, (*__slate_slot_1818).wrapping_add((1 as i32) as u32));
*__slate_slot_722 = *__slate_slot_1819;
unsafe {
(*(*__slate_slot_899)).seekResult = 0 as i32;
}
if *__slate_slot_710 != (0 as i32) {
break '__join_1217;
} else {
// Invoke the update-hook if required.
if *__slate_slot_902 & (1 as i32) != (0 as i32) {
std::ptr::write(__slate_slot_1820, p);
std::ptr::write(__slate_slot_1821, unsafe { (*(*__slate_slot_1820)).nChange });
std::ptr::write(__slate_slot_1822, *__slate_slot_1821 + ((1 as i32) as i64));
unsafe {
(*(*__slate_slot_1820)).nChange = *__slate_slot_1822;
}
if (unsafe { (*(*__slate_slot_711)).xUpdateCallback }) != None && *__slate_slot_901 != std::ptr::null_mut::<Table>() && (unsafe { (*(*__slate_slot_901)).tabFlags }) & ((128 as i32) as u32) == ((0 as i32) as u32) {
unsafe { unsafe { (*(*__slate_slot_711)).xUpdateCallback }.unwrap()(unsafe { (*(*__slate_slot_711)).pUpdateArg }, 9 as i32, *__slate_slot_900, (unsafe { (*(*__slate_slot_901)).zName }) as *const i8, unsafe { (*(*__slate_slot_899)).movetoTarget }) };
0 as i32;
break '__join_26;
} else {
break '__join_26;
}
} else {
break '__join_26;
}
}
} else {
if __t4 == (133 as i32) {
unsafe { sqlite3VdbeSetChanges(*__slate_slot_711, unsafe { (*p).nChange }) };
unsafe {
(*p).nChange = (0 as i32) as i64;
}
break '__join_26;
} else {
if __t4 == (134 as i32) {
*__slate_slot_903 = unsafe { *unsafe { unsafe { (*p).apCsr }.offset((unsafe { (*(*__slate_slot_709)).p1 }) as isize) } };
0 as i32;
0 as i32;
*__slate_slot_720 = unsafe { (*__slate_slot_717).offset((unsafe { (*(*__slate_slot_709)).p3 }) as isize) };
*__slate_slot_905 = unsafe { (*(*__slate_slot_709)).p4.i };
*__slate_slot_904 = 0 as i32;
*__slate_slot_710 = unsafe { sqlite3VdbeSorterCompare(*__slate_slot_903 as *const VdbeCursor, *__slate_slot_720, *__slate_slot_905, std::ptr::addr_of_mut!(*__slate_slot_904)) };
{
}
if *__slate_slot_710 != (0 as i32) {
break '__join_1216;
} else {
if *__slate_slot_904 != (0 as i32) {
break '__join_1125;
} else {
break '__join_26;
}
}
} else {
if __t4 == (135 as i32) {
// ncycle
*__slate_slot_721 = unsafe { (*__slate_slot_717).offset((unsafe { (*(*__slate_slot_709)).p2 }) as isize) };
*__slate_slot_906 = unsafe { *unsafe { unsafe { (*p).apCsr }.offset((unsafe { (*(*__slate_slot_709)).p1 }) as isize) } };
0 as i32;
*__slate_slot_710 = unsafe { sqlite3VdbeSorterRowkey(*__slate_slot_906 as *const VdbeCursor, *__slate_slot_721) };
0 as i32;
0 as i32;
if *__slate_slot_710 != (0 as i32) {
break '__join_1215;
} else {
unsafe {
(*unsafe { *unsafe { unsafe { (*p).apCsr }.offset((unsafe { (*(*__slate_slot_709)).p3 }) as isize) } }).cacheStatus = (0 as i32) as u32;
}
break '__join_26;
}
} else {
if __t4 == (136 as i32) {
*__slate_slot_721 = out2Prerelease(p, *__slate_slot_709);
0 as i32;
*__slate_slot_907 = unsafe { *unsafe { unsafe { (*p).apCsr }.offset((unsafe { (*(*__slate_slot_709)).p1 }) as isize) } };
0 as i32;
0 as i32;
0 as i32;
0 as i32;
0 as i32;
*__slate_slot_908 = unsafe { (*(*__slate_slot_907)).uc.pCursor };
// The OP_RowData opcodes always follow OP_NotExists or
// OP_SeekRowid or OP_Rewind/Op_Next with no intervening instructions
// that might invalidate the cursor.
// If this were not the case, one of the following assert()s
// would fail.  Should this ever change (because of changes in the code
// generator) then the fix would be to insert a call to
// sqlite3VdbeCursorMoveto().
0 as i32;
0 as i32;
*__slate_slot_909 = unsafe { sqlite3BtreePayloadSize(*__slate_slot_908) };
if *__slate_slot_909 > ((unsafe { *unsafe { unsafe { (*(*__slate_slot_711)).aLimit.as_mut_ptr() as *mut i32 }.offset((0 as i32) as isize) } }) as u32) {
break '__join_2;
} else {
{
}
*__slate_slot_710 = unsafe { sqlite3VdbeMemFromBtreeZeroOffset(*__slate_slot_908, *__slate_slot_909, *__slate_slot_721) };
if *__slate_slot_710 != (0 as i32) {
break '__join_1214;
} else {
if !((unsafe { (*(*__slate_slot_709)).p3 }) != (0 as i32)) {
if (((unsafe { (*(*__slate_slot_721)).flags }) as u32) as i32) & (16384 as i32) != (0 as i32) {
*__slate_slot_1823 = (unsafe { sqlite3VdbeMemMakeWriteable(*__slate_slot_721) }) != (0 as i32);
} else {
*__slate_slot_1823 = false as bool;
}
if *__slate_slot_1823 {
break '__join_1;
}
}
{
}
{
}
{
}
break '__join_26;
}
}
} else {
if __t4 == (137 as i32) {
// out2, ncycle
*__slate_slot_721 = out2Prerelease(p, *__slate_slot_709);
0 as i32;
*__slate_slot_910 = unsafe { *unsafe { unsafe { (*p).apCsr }.offset((unsafe { (*(*__slate_slot_709)).p1 }) as isize) } };
0 as i32;
0 as i32;
if (unsafe { (*(*__slate_slot_910)).nullRow }) != (0 as u8) {
unsafe {
(*(*__slate_slot_721)).flags = ((1 as i32) as i16) as u16;
}
break '__join_26;
} else {
if (unsafe { (*(*__slate_slot_910)).deferredMoveto }) != (0 as u8) {
*__slate_slot_911 = unsafe { (*(*__slate_slot_910)).movetoTarget };
} else {
if (((unsafe { (*(*__slate_slot_910)).eCurType }) as u32) as i32) == (2 as i32) {
0 as i32;
*__slate_slot_912 = unsafe { (*unsafe { (*(*__slate_slot_910)).uc.pVCur }).pVtab };
*__slate_slot_913 = unsafe { (*(*__slate_slot_912)).pModule };
0 as i32;
*__slate_slot_710 = unsafe { unsafe { (*(*__slate_slot_913)).xRowid }.unwrap()(unsafe { (*(*__slate_slot_910)).uc.pVCur }, std::ptr::addr_of_mut!(*__slate_slot_911)) };
unsafe { sqlite3VtabImportErrmsg(p, *__slate_slot_912) };
if *__slate_slot_710 != (0 as i32) {
break '__join_1212;
}
} else {
0 as i32;
0 as i32;
*__slate_slot_710 = unsafe { sqlite3VdbeCursorRestore(*__slate_slot_910) };
if *__slate_slot_710 != (0 as i32) {
break '__join_1213;
} else {
if (unsafe { (*(*__slate_slot_910)).nullRow }) != (0 as u8) {
unsafe {
(*(*__slate_slot_721)).flags = ((1 as i32) as i16) as u16;
}
break '__join_26;
} else {
*__slate_slot_911 = unsafe { sqlite3BtreeIntegerKey(unsafe { (*(*__slate_slot_910)).uc.pCursor }) };
}
}
}
}
unsafe {
(*(*__slate_slot_721)).u.i = *__slate_slot_911;
}
break '__join_26;
}
} else {
if __t4 == (138 as i32) {
0 as i32;
*__slate_slot_914 = unsafe { *unsafe { unsafe { (*p).apCsr }.offset((unsafe { (*(*__slate_slot_709)).p1 }) as isize) } };
if *__slate_slot_914 == std::ptr::null_mut::<VdbeCursor>() {
// If the cursor is not already open, create a special kind of
// pseudo-cursor that always gives null rows.
*__slate_slot_914 = allocateCursor(p, unsafe { (*(*__slate_slot_709)).p1 }, 1 as i32, ((3 as i32) as i8) as u8);
if *__slate_slot_914 == std::ptr::null_mut::<VdbeCursor>() {
break '__join_1;
} else {
unsafe {
(*(*__slate_slot_914)).seekResult = 0 as i32;
}
unsafe {
(*(*__slate_slot_914)).isTable = ((1 as i32) as i8) as u8;
}
unsafe {
(*(*__slate_slot_914)).__slate_bits_0.__set_noReuse((1 as i32) as u32);
}
unsafe {
(*(*__slate_slot_914)).uc.pCursor = unsafe { sqlite3BtreeFakeValidCursor() };
}
}
}
unsafe {
(*(*__slate_slot_914)).nullRow = ((1 as i32) as i8) as u8;
}
unsafe {
(*(*__slate_slot_914)).cacheStatus = (0 as i32) as u32;
}
if (((unsafe { (*(*__slate_slot_914)).eCurType }) as u32) as i32) == (0 as i32) {
0 as i32;
unsafe { sqlite3BtreeClearCursor(unsafe { (*(*__slate_slot_914)).uc.pCursor }) };
break '__join_26;
} else {
break '__join_26;
}
} else {
if __t4 == (139 as i32) {
break '__join_374;
} else {
if __t4 == (32 as i32) {
break '__join_374;
} else {
if __t4 == (33 as i32) {
// jump
0 as i32;
0 as i32;
0 as i32;
0 as i32;
*__slate_slot_918 = unsafe { *unsafe { unsafe { (*p).apCsr }.offset((unsafe { (*(*__slate_slot_709)).p1 }) as isize) } };
0 as i32;
*__slate_slot_919 = unsafe { (*(*__slate_slot_918)).uc.pCursor };
0 as i32;
*__slate_slot_710 = unsafe { sqlite3BtreeFirst(*__slate_slot_919, std::ptr::addr_of_mut!(*__slate_slot_920)) };
if *__slate_slot_710 != (0 as i32) {
break '__join_1210;
} else {
if *__slate_slot_920 != (0 as i32) {
*__slate_slot_921 = -(1 as i32) as i64; // -Infinity encoding
} else {
*__slate_slot_921 = unsafe { sqlite3BtreeRowCountEst(*__slate_slot_919) };
0 as i32;
*__slate_slot_921 = (unsafe { sqlite3LogEst(*__slate_slot_921 as u64) }) as i64;
}
*__slate_slot_920 = (*__slate_slot_921 >= ((unsafe { (*(*__slate_slot_709)).p3 }) as i64) && *__slate_slot_921 <= ((unsafe { (*(*__slate_slot_709)).p4.i }) as i64)) as i32;
{
}
if *__slate_slot_920 != (0 as i32) {
break '__join_1125;
} else {
break '__join_26;
}
}
} else {
if __t4 == (34 as i32) {
break '__join_364;
} else {
if __t4 == (35 as i32) {
break '__join_364;
} else {
if __t4 == (36 as i32) {
break '__join_363;
} else {
if __t4 == (37 as i32) {
// jump
0 as i32;
0 as i32;
*__slate_slot_925 = unsafe { *unsafe { unsafe { (*p).apCsr }.offset((unsafe { (*(*__slate_slot_709)).p1 }) as isize) } };
0 as i32;
0 as i32;
*__slate_slot_926 = unsafe { (*(*__slate_slot_925)).uc.pCursor };
0 as i32;
*__slate_slot_710 = unsafe { sqlite3BtreeIsEmpty(*__slate_slot_926, std::ptr::addr_of_mut!(*__slate_slot_927)) };
if *__slate_slot_710 != (0 as i32) {
break '__join_1208;
} else {
{
}
if *__slate_slot_927 != (0 as i32) {
break '__join_1125;
} else {
break '__join_26;
}
}
} else {
if __t4 == (38 as i32) {
// jump
*__slate_slot_928 = unsafe { *unsafe { unsafe { (*p).apCsr }.offset((unsafe { (*(*__slate_slot_709)).p1 }) as isize) } };
0 as i32;
*__slate_slot_710 = unsafe { sqlite3VdbeSorterNext(*__slate_slot_711, *__slate_slot_928 as *const VdbeCursor) };
break '__join_352;
} else {
if __t4 == (39 as i32) {
0 as i32; // jump, ncycle
0 as i32;
*__slate_slot_928 = unsafe { *unsafe { unsafe { (*p).apCsr }.offset((unsafe { (*(*__slate_slot_709)).p1 }) as isize) } };
0 as i32;
0 as i32;
0 as i32;
0 as i32;
*__slate_slot_710 = unsafe { sqlite3BtreePrevious(unsafe { (*(*__slate_slot_928)).uc.pCursor }, unsafe { (*(*__slate_slot_709)).p3 }) };
break '__join_352;
} else {
if __t4 == (40 as i32) {
0 as i32; // jump, ncycle
0 as i32;
*__slate_slot_928 = unsafe { *unsafe { unsafe { (*p).apCsr }.offset((unsafe { (*(*__slate_slot_709)).p1 }) as isize) } };
0 as i32;
0 as i32;
0 as i32;
0 as i32;
*__slate_slot_710 = unsafe { sqlite3BtreeNext(unsafe { (*(*__slate_slot_928)).uc.pCursor }, unsafe { (*(*__slate_slot_709)).p3 }) };
break '__join_352;
} else {
if __t4 == (140 as i32) {
// in2
0 as i32;
*__slate_slot_929 = unsafe { *unsafe { unsafe { (*p).apCsr }.offset((unsafe { (*(*__slate_slot_709)).p1 }) as isize) } };
{
}
0 as i32;
0 as i32;
*__slate_slot_719 = unsafe { (*__slate_slot_717).offset((unsafe { (*(*__slate_slot_709)).p2 }) as isize) };
0 as i32;
if (((unsafe { (*(*__slate_slot_709)).p5 }) as u32) as i32) & (1 as i32) != (0 as i32) {
std::ptr::write(__slate_slot_1830, p);
std::ptr::write(__slate_slot_1831, unsafe { (*(*__slate_slot_1830)).nChange });
std::ptr::write(__slate_slot_1832, *__slate_slot_1831 + ((1 as i32) as i64));
unsafe {
(*(*__slate_slot_1830)).nChange = *__slate_slot_1832;
}
}
0 as i32;
0 as i32;
if (((unsafe { (*(*__slate_slot_719)).flags }) as u32) as i32) & (1024 as i32) != (0 as i32) {
*__slate_slot_1833 = unsafe { sqlite3VdbeMemExpandBlob(*__slate_slot_719) };
} else {
*__slate_slot_1833 = 0 as i32;
}
*__slate_slot_710 = *__slate_slot_1833;
if *__slate_slot_710 != (0 as i32) {
break '__join_1206;
} else {
(*__slate_slot_930).nKey = (unsafe { (*(*__slate_slot_719)).n }) as i64;
(*__slate_slot_930).pKey = (unsafe { (*(*__slate_slot_719)).z }) as *const ();
(*__slate_slot_930).aMem = unsafe { (*__slate_slot_717).offset((unsafe { (*(*__slate_slot_709)).p3 }) as isize) };
(*__slate_slot_930).nMem = ((unsafe { (*(*__slate_slot_709)).p4.i }) as i16) as u16;
*__slate_slot_710 = unsafe { sqlite3BtreeInsert(unsafe { (*(*__slate_slot_929)).uc.pCursor }, std::ptr::addr_of_mut!(*__slate_slot_930) as *const BtreePayload, (((unsafe { (*(*__slate_slot_709)).p5 }) as u32) as i32) & ((8 as i32) | (2 as i32) | (128 as i32)), if (((unsafe { (*(*__slate_slot_709)).p5 }) as u32) as i32) & (16 as i32) != (0 as i32) { unsafe { (*(*__slate_slot_929)).seekResult } } else { 0 as i32 }) };
0 as i32;
unsafe {
(*(*__slate_slot_929)).cacheStatus = (0 as i32) as u32;
}
if *__slate_slot_710 != (0 as i32) {
break '__join_1205;
} else {
break '__join_26;
}
}
} else {
if __t4 == (141 as i32) {
// in2
0 as i32;
*__slate_slot_931 = unsafe { *unsafe { unsafe { (*p).apCsr }.offset((unsafe { (*(*__slate_slot_709)).p1 }) as isize) } };
{
}
0 as i32;
0 as i32;
*__slate_slot_719 = unsafe { (*__slate_slot_717).offset((unsafe { (*(*__slate_slot_709)).p2 }) as isize) };
0 as i32;
0 as i32;
if (((unsafe { (*(*__slate_slot_719)).flags }) as u32) as i32) & (1024 as i32) != (0 as i32) {
*__slate_slot_1834 = unsafe { sqlite3VdbeMemExpandBlob(*__slate_slot_719) };
} else {
*__slate_slot_1834 = 0 as i32;
}
*__slate_slot_710 = *__slate_slot_1834;
if *__slate_slot_710 != (0 as i32) {
break '__join_1204;
} else {
*__slate_slot_710 = unsafe { sqlite3VdbeSorterWrite(*__slate_slot_931 as *const VdbeCursor, *__slate_slot_719) };
if *__slate_slot_710 != (0 as i32) {
break '__join_1203;
} else {
break '__join_26;
}
}
} else {
if __t4 == (142 as i32) {
0 as i32;
0 as i32;
0 as i32;
*__slate_slot_932 = unsafe { *unsafe { unsafe { (*p).apCsr }.offset((unsafe { (*(*__slate_slot_709)).p1 }) as isize) } };
0 as i32;
0 as i32;
{
}
*__slate_slot_933 = unsafe { (*(*__slate_slot_932)).uc.pCursor };
0 as i32;
(*__slate_slot_935).pKeyInfo = unsafe { (*(*__slate_slot_932)).pKeyInfo };
(*__slate_slot_935).nField = unsafe { (*(*__slate_slot_709)).p5 };
(*__slate_slot_935).default_rc = (0 as i32) as i8;
(*__slate_slot_935).aMem = unsafe { (*__slate_slot_717).offset((unsafe { (*(*__slate_slot_709)).p2 }) as isize) };
*__slate_slot_710 = unsafe { sqlite3BtreeIndexMoveto(*__slate_slot_933, std::ptr::addr_of_mut!(*__slate_slot_935), std::ptr::addr_of_mut!(*__slate_slot_934)) };
if *__slate_slot_710 != (0 as i32) {
break '__join_1202;
} else {
if *__slate_slot_934 != (0 as i32) {
*__slate_slot_710 = unsafe { sqlite3VdbeFindIndexKey(*__slate_slot_933, unsafe { (*(*__slate_slot_709)).p4.pIdx }, std::ptr::addr_of_mut!(*__slate_slot_935), std::ptr::addr_of_mut!(*__slate_slot_934), 0 as i32) };
if *__slate_slot_710 != (0 as i32) {
break '__join_1201;
} else {
if *__slate_slot_934 != (0 as i32) {
if !((unsafe { sqlite3WritableSchema(*__slate_slot_711) }) != (0 as i32)) {
break '__join_331;
} else {
unsafe {
(*(*__slate_slot_932)).cacheStatus = (0 as i32) as u32;
}
unsafe {
(*(*__slate_slot_932)).seekResult = 0 as i32;
}
break '__join_26;
}
}
}
}
if (unsafe { (*(*__slate_slot_709)).p3 }) != (0 as i32) {
*__slate_slot_1835 = vdbeIndexKeyCompare(*__slate_slot_933, unsafe { (*__slate_slot_717).offset((unsafe { (*(*__slate_slot_709)).p3 }) as isize) }, std::ptr::addr_of_mut!(*__slate_slot_710)) != (0 as i32);
} else {
*__slate_slot_1835 = false as bool;
}
if *__slate_slot_1835 {
if *__slate_slot_710 != (0 as i32) {
break '__join_1199;
} else {
unsafe { sqlite3VdbeMemSetNull(unsafe { (*__slate_slot_717).offset((unsafe { (*(*__slate_slot_709)).p3 }) as isize) }) };
break '__join_26;
}
} else {
*__slate_slot_710 = unsafe { sqlite3BtreeDelete(*__slate_slot_933, ((4 as i32) as i8) as u8) };
if *__slate_slot_710 != (0 as i32) {
break '__join_1198;
} else {
0 as i32;
unsafe {
(*(*__slate_slot_932)).cacheStatus = (0 as i32) as u32;
}
unsafe {
(*(*__slate_slot_932)).seekResult = 0 as i32;
}
break '__join_26;
}
}
}
} else {
if __t4 == (143 as i32) {
break '__join_321;
} else {
if __t4 == (144 as i32) {
break '__join_321;
} else {
if __t4 == (145 as i32) {
// ncycle
0 as i32;
*__slate_slot_939 = unsafe { *unsafe { unsafe { (*p).apCsr }.offset((unsafe { (*(*__slate_slot_709)).p1 }) as isize) } };
if (unsafe { (*(*__slate_slot_939)).deferredMoveto }) != (0 as u8) {
*__slate_slot_710 = unsafe { sqlite3VdbeFinishMoveto(*__slate_slot_939) };
if *__slate_slot_710 != (0 as i32) {
break '__join_1195;
} else {
break '__join_26;
}
} else {
break '__join_26;
}
} else {
if __t4 == (41 as i32) {
break '__join_312;
} else {
if __t4 == (42 as i32) {
break '__join_312;
} else {
if __t4 == (45 as i32) {
break '__join_312;
} else {
if __t4 == (46 as i32) {
break '__join_312;
} else {
if __t4 == (146 as i32) {
// out2
{
}
0 as i32;
0 as i32;
*__slate_slot_721 = out2Prerelease(p, *__slate_slot_709);
unsafe {
(*(*__slate_slot_721)).flags = ((1 as i32) as i16) as u16;
}
if (unsafe { (*(*__slate_slot_711)).nVdbeRead }) > (unsafe { (*(*__slate_slot_711)).nVDestroy }) + (1 as i32) {
break '__join_298;
} else {
*__slate_slot_947 = unsafe { (*(*__slate_slot_709)).p3 };
0 as i32;
*__slate_slot_946 = 0 as i32; // Not needed.  Only to silence a warning.
*__slate_slot_710 = unsafe { sqlite3BtreeDropTable(unsafe { (*unsafe { unsafe { (*(*__slate_slot_711)).aDb }.offset(*__slate_slot_947 as isize) }).pBt }, unsafe { (*(*__slate_slot_709)).p1 }, std::ptr::addr_of_mut!(*__slate_slot_946)) };
unsafe {
(*(*__slate_slot_721)).flags = ((4 as i32) as i16) as u16;
}
unsafe {
(*(*__slate_slot_721)).u.i = *__slate_slot_946 as i64;
}
if *__slate_slot_710 != (0 as i32) {
break '__join_1192;
} else {
if *__slate_slot_946 != (0 as i32) {
unsafe { sqlite3RootPageMoved(*__slate_slot_711, *__slate_slot_947, *__slate_slot_946 as u32, (unsafe { (*(*__slate_slot_709)).p1 }) as u32) };
// All OP_Destroy operations occur on the same btree
0 as i32;
*__slate_slot_712 = ((*__slate_slot_947 + (1 as i32)) as i8) as u8;
break '__join_26;
} else {
break '__join_26;
}
}
}
} else {
if __t4 == (147 as i32) {
{
}
*__slate_slot_948 = (0 as i32) as i64;
0 as i32;
0 as i32;
*__slate_slot_710 = unsafe { sqlite3BtreeClearTable(unsafe { (*unsafe { unsafe { (*(*__slate_slot_711)).aDb }.offset((unsafe { (*(*__slate_slot_709)).p2 }) as isize) }).pBt }, ((unsafe { (*(*__slate_slot_709)).p1 }) as u32) as i32, std::ptr::addr_of_mut!(*__slate_slot_948)) };
if (unsafe { (*(*__slate_slot_709)).p3 }) != (0 as i32) {
std::ptr::write(__slate_slot_1838, p);
std::ptr::write(__slate_slot_1839, unsafe { (*(*__slate_slot_1838)).nChange });
std::ptr::write(__slate_slot_1840, *__slate_slot_1839 + *__slate_slot_948);
unsafe {
(*(*__slate_slot_1838)).nChange = *__slate_slot_1840;
}
if (unsafe { (*(*__slate_slot_709)).p3 }) > (0 as i32) {
0 as i32;
{
}
std::ptr::write(__slate_slot_1841, unsafe { (*__slate_slot_717).offset((unsafe { (*(*__slate_slot_709)).p3 }) as isize) });
std::ptr::write(__slate_slot_1842, unsafe { (*(*__slate_slot_1841)).u.i });
std::ptr::write(__slate_slot_1843, *__slate_slot_1842 + *__slate_slot_948);
unsafe {
(*(*__slate_slot_1841)).u.i = *__slate_slot_1843;
}
}
}
if *__slate_slot_710 != (0 as i32) {
break '__join_1190;
} else {
break '__join_26;
}
} else {
if __t4 == (148 as i32) {
0 as i32;
*__slate_slot_949 = unsafe { *unsafe { unsafe { (*p).apCsr }.offset((unsafe { (*(*__slate_slot_709)).p1 }) as isize) } };
0 as i32;
if (((unsafe { (*(*__slate_slot_949)).eCurType }) as u32) as i32) == (1 as i32) {
unsafe { sqlite3VdbeSorterReset(*__slate_slot_711, unsafe { (*(*__slate_slot_949)).uc.pSorter }) };
break '__join_26;
} else {
0 as i32;
0 as i32;
*__slate_slot_710 = unsafe { sqlite3BtreeClearTableOfCursor(unsafe { (*(*__slate_slot_949)).uc.pCursor }) };
if *__slate_slot_710 != (0 as i32) {
break '__join_1189;
} else {
break '__join_26;
}
}
} else {
if __t4 == (149 as i32) {
// out2
{
}
*__slate_slot_721 = out2Prerelease(p, *__slate_slot_709);
*__slate_slot_950 = (0 as i32) as u32;
0 as i32;
0 as i32;
0 as i32;
0 as i32;
*__slate_slot_951 = unsafe { unsafe { (*(*__slate_slot_711)).aDb }.offset((unsafe { (*(*__slate_slot_709)).p1 }) as isize) };
0 as i32;
*__slate_slot_710 = unsafe { sqlite3BtreeCreateTable(unsafe { (*(*__slate_slot_951)).pBt }, std::ptr::addr_of_mut!(*__slate_slot_950), unsafe { (*(*__slate_slot_709)).p3 }) };
if *__slate_slot_710 != (0 as i32) {
break '__join_1188;
} else {
unsafe {
(*(*__slate_slot_721)).u.i = (*__slate_slot_950 as u64) as i64;
}
break '__join_26;
}
} else {
if __t4 == (150 as i32) {
{
}
std::ptr::write(__slate_slot_1844, *__slate_slot_711);
std::ptr::write(__slate_slot_1845, unsafe { (*(*__slate_slot_1844)).nSqlExec });
std::ptr::write(__slate_slot_1846, ((((*__slate_slot_1845 as u32) as i32) + (1 as i32)) as i8) as u8);
unsafe {
(*(*__slate_slot_1844)).nSqlExec = *__slate_slot_1846;
}
*__slate_slot_952 = std::ptr::null_mut::<i8>();
*__slate_slot_953 = unsafe { (*(*__slate_slot_711)).xAuth };
*__slate_slot_954 = unsafe { (*(*__slate_slot_711)).mTrace };
*__slate_slot_955 = unsafe { (*(*__slate_slot_711)).nAnalysisLimit };
if (unsafe { (*(*__slate_slot_709)).p1 }) & (1 as i32) != (0 as i32) {
unsafe {
(*(*__slate_slot_711)).xAuth = None;
}
unsafe {
(*(*__slate_slot_711)).mTrace = ((0 as i32) as i8) as u8;
}
}
if (unsafe { (*(*__slate_slot_709)).p1 }) & (2 as i32) != (0 as i32) {
unsafe {
(*(*__slate_slot_711)).nAnalysisLimit = unsafe { (*(*__slate_slot_709)).p2 };
}
}
*__slate_slot_710 = unsafe { sqlite3_exec(*__slate_slot_711, (unsafe { (*(*__slate_slot_709)).p4.z }) as *const i8, None, std::ptr::null_mut::<()>(), std::ptr::addr_of_mut!(*__slate_slot_952)) };
std::ptr::write(__slate_slot_1847, *__slate_slot_711);
std::ptr::write(__slate_slot_1848, unsafe { (*(*__slate_slot_1847)).nSqlExec });
std::ptr::write(__slate_slot_1849, ((((*__slate_slot_1848 as u32) as i32) - (1 as i32)) as i8) as u8);
unsafe {
(*(*__slate_slot_1847)).nSqlExec = *__slate_slot_1849;
}
unsafe {
(*(*__slate_slot_711)).xAuth = *__slate_slot_953;
}
unsafe {
(*(*__slate_slot_711)).mTrace = *__slate_slot_954;
}
unsafe {
(*(*__slate_slot_711)).nAnalysisLimit = *__slate_slot_955;
}
if *__slate_slot_952 != std::ptr::null_mut::<i8>() || *__slate_slot_710 != (0 as i32) {
break '__join_283;
} else {
break '__join_26;
}
} else {
if __t4 == (151 as i32) {
// Any prepared statement that invokes this opcode will hold mutexes
// on every btree.  This is a prerequisite for invoking
// sqlite3InitCallback().
*__slate_slot_956 = unsafe { (*(*__slate_slot_709)).p1 };
0 as i32;
0 as i32;
if (unsafe { (*(*__slate_slot_709)).p4.z }) == std::ptr::null_mut::<i8>() {
0 as i32;
unsafe { sqlite3SchemaClear((unsafe { (*unsafe { unsafe { (*(*__slate_slot_711)).aDb }.offset(*__slate_slot_956 as isize) }).pSchema }) as *mut ()) };
std::ptr::write(__slate_slot_1850, *__slate_slot_711);
std::ptr::write(__slate_slot_1851, unsafe { (*(*__slate_slot_1850)).mDbFlags });
std::ptr::write(__slate_slot_1852, *__slate_slot_1851 & (!(16 as i32) as u32));
unsafe {
(*(*__slate_slot_1850)).mDbFlags = *__slate_slot_1852;
}
*__slate_slot_710 = unsafe { sqlite3InitOne(*__slate_slot_711, *__slate_slot_956, unsafe { std::ptr::addr_of_mut!((*p).zErrMsg) }, (unsafe { (*(*__slate_slot_709)).p5 }) as u32) };
std::ptr::write(__slate_slot_1853, *__slate_slot_711);
std::ptr::write(__slate_slot_1854, unsafe { (*(*__slate_slot_1853)).mDbFlags });
std::ptr::write(__slate_slot_1855, *__slate_slot_1854 | ((1 as i32) as u32));
unsafe {
(*(*__slate_slot_1853)).mDbFlags = *__slate_slot_1855;
}
unsafe {
(*p).__slate_bits_0.__set_expired((0 as i32) as u32);
}
} else {
0 as i32;
*__slate_slot_957 = (b"sqlite_master\0".as_ptr() as *mut i8) as *const i8;
(*__slate_slot_959).db = *__slate_slot_711;
(*__slate_slot_959).iDb = *__slate_slot_956;
(*__slate_slot_959).pzErrMsg = unsafe { std::ptr::addr_of_mut!((*p).zErrMsg) };
(*__slate_slot_959).mInitFlags = (0 as i32) as u32;
(*__slate_slot_959).mxPage = unsafe { sqlite3BtreeLastPage(unsafe { (*unsafe { unsafe { (*(*__slate_slot_711)).aDb }.offset(*__slate_slot_956 as isize) }).pBt }) };
*__slate_slot_958 = unsafe { sqlite3MPrintf(*__slate_slot_711, (b"SELECT*FROM\"%w\".%s WHERE %s ORDER BY rowid\0".as_ptr() as *mut i8) as *const i8, unsafe { (*unsafe { unsafe { (*(*__slate_slot_711)).aDb }.offset(*__slate_slot_956 as isize) }).zDbSName }, *__slate_slot_957, unsafe { (*(*__slate_slot_709)).p4.z }) };
if *__slate_slot_958 == std::ptr::null_mut::<i8>() {
*__slate_slot_710 = 7 as i32;
} else {
0 as i32;
unsafe {
(*(*__slate_slot_711)).init.busy = ((1 as i32) as i8) as u8;
}
(*__slate_slot_959).rc = 0 as i32;
(*__slate_slot_959).nInitRow = (0 as i32) as u32;
0 as i32;
*__slate_slot_710 = unsafe { sqlite3_exec(*__slate_slot_711, *__slate_slot_958 as *const i8, unsafe { std::mem::transmute::<*const (), Option<unsafe extern "C-unwind" fn(*mut (), i32, *mut *mut i8, *mut *mut i8) -> i32>>(sqlite3InitCallback as *const ()) }, std::ptr::addr_of_mut!(*__slate_slot_959) as *mut (), std::ptr::null_mut::<*mut i8>()) };
if *__slate_slot_710 == (0 as i32) {
*__slate_slot_710 = (*__slate_slot_959).rc;
}
if *__slate_slot_710 == (0 as i32) && (*__slate_slot_959).nInitRow == ((0 as i32) as u32) {
// The OP_ParseSchema opcode with a non-NULL P4 argument should parse
// at least one SQL statement. Any less than that indicates that
// the sqlite_schema table is corrupt.
*__slate_slot_710 = unsafe { sqlite3CorruptError(7348 as i32) };
}
unsafe { sqlite3DbFreeNN(*__slate_slot_711, *__slate_slot_958 as *mut ()) };
unsafe {
(*(*__slate_slot_711)).init.busy = ((0 as i32) as i8) as u8;
}
}
}
if *__slate_slot_710 != (0 as i32) {
break '__join_272;
} else {
break '__join_26;
}
} else {
if __t4 == (152 as i32) {
0 as i32;
*__slate_slot_710 = unsafe { sqlite3AnalysisLoad(*__slate_slot_711, unsafe { (*(*__slate_slot_709)).p1 }) };
if *__slate_slot_710 != (0 as i32) {
break '__join_1185;
} else {
break '__join_26;
}
} else {
if __t4 == (153 as i32) {
{
}
unsafe { sqlite3UnlinkAndDeleteTable(*__slate_slot_711, unsafe { (*(*__slate_slot_709)).p1 }, (unsafe { (*(*__slate_slot_709)).p4.z }) as *const i8) };
break '__join_26;
} else {
if __t4 == (155 as i32) {
{
}
unsafe { sqlite3UnlinkAndDeleteIndex(*__slate_slot_711, unsafe { (*(*__slate_slot_709)).p1 }, (unsafe { (*(*__slate_slot_709)).p4.z }) as *const i8) };
break '__join_26;
} else {
if __t4 == (156 as i32) {
{
}
unsafe { sqlite3UnlinkAndDeleteTrigger(*__slate_slot_711, unsafe { (*(*__slate_slot_709)).p1 }, (unsafe { (*(*__slate_slot_709)).p4.z }) as *const i8) };
break '__join_26;
} else {
if __t4 == (157 as i32) {
0 as i32;
0 as i32;
*__slate_slot_960 = unsafe { (*(*__slate_slot_709)).p2 };
*__slate_slot_961 = unsafe { (*(*__slate_slot_709)).p4.ai };
0 as i32;
0 as i32;
0 as i32;
0 as i32;
*__slate_slot_964 = unsafe { (*__slate_slot_717).offset((unsafe { (*(*__slate_slot_709)).p1 }) as isize) };
0 as i32;
0 as i32;
*__slate_slot_718 = unsafe { (*__slate_slot_717).offset(((unsafe { (*(*__slate_slot_709)).p1 }) + (1 as i32)) as isize) };
0 as i32;
0 as i32;
*__slate_slot_710 = unsafe { sqlite3BtreeIntegrityCheck(*__slate_slot_711, unsafe { (*unsafe { unsafe { (*(*__slate_slot_711)).aDb }.offset((((unsafe { (*(*__slate_slot_709)).p5 }) as u32) as i32) as isize) }).pBt }, unsafe { (*__slate_slot_961).offset((1 as i32) as isize) }, unsafe { (*__slate_slot_717).offset((unsafe { (*(*__slate_slot_709)).p3 }) as isize) }, *__slate_slot_960, ((unsafe { (*(*__slate_slot_964)).u.i }) as i32) + (1 as i32), std::ptr::addr_of_mut!(*__slate_slot_962), std::ptr::addr_of_mut!(*__slate_slot_963)) };
unsafe { sqlite3VdbeMemSetNull(*__slate_slot_718) };
if *__slate_slot_962 == (0 as i32) {
0 as i32;
} else {
if *__slate_slot_710 != (0 as i32) {
break '__join_264;
} else {
std::ptr::write(__slate_slot_1856, *__slate_slot_964);
std::ptr::write(__slate_slot_1857, unsafe { (*(*__slate_slot_1856)).u.i });
std::ptr::write(__slate_slot_1858, *__slate_slot_1857 - ((*__slate_slot_962 - (1 as i32)) as i64));
unsafe {
(*(*__slate_slot_1856)).u.i = *__slate_slot_1858;
}
unsafe { sqlite3VdbeMemSetStr(*__slate_slot_718, *__slate_slot_963 as *const i8, -(1 as i32) as i64, ((1 as i32) as i8) as u8, unsafe { std::mem::transmute::<*const (), Option<unsafe extern "C-unwind" fn(*mut ())>>(sqlite3_free as *const ()) }) };
}
}
{
}
unsafe { sqlite3VdbeChangeEncoding(*__slate_slot_718, (*__slate_slot_713 as u32) as i32) };
break '__join_1139;
} else {
if __t4 == (47 as i32) {
// jump, in3
0 as i32;
0 as i32;
0 as i32;
*__slate_slot_965 = unsafe { *unsafe { unsafe { (*p).apCsr }.offset((unsafe { (*(*__slate_slot_709)).p1 }) as isize) } };
0 as i32;
0 as i32;
0 as i32;
0 as i32;
unsafe { memset(std::ptr::addr_of_mut!(*__slate_slot_967) as *mut (), 0 as i32, 40 as u64) };
(*__slate_slot_967).aMem = unsafe { (*__slate_slot_717).offset((unsafe { (*(*__slate_slot_709)).p3 }) as isize) };
(*__slate_slot_967).nField = unsafe { (*unsafe { (*(*__slate_slot_709)).p4.pIdx }).nColumn };
(*__slate_slot_967).pKeyInfo = unsafe { (*(*__slate_slot_965)).pKeyInfo };
*__slate_slot_710 = unsafe { sqlite3VdbeFindIndexKey(unsafe { (*(*__slate_slot_965)).uc.pCursor }, unsafe { (*(*__slate_slot_709)).p4.pIdx }, std::ptr::addr_of_mut!(*__slate_slot_967), std::ptr::addr_of_mut!(*__slate_slot_966), 1 as i32) };
if *__slate_slot_710 != (0 as i32) || *__slate_slot_966 != (0 as i32) {
*__slate_slot_710 = 0 as i32;
break '__join_1125;
} else {
unsafe {
(*(*__slate_slot_965)).nullRow = ((0 as i32) as i8) as u8;
}
break '__join_26;
}
} else {
if __t4 == (158 as i32) {
// in1, in2
*__slate_slot_718 = unsafe { (*__slate_slot_717).offset((unsafe { (*(*__slate_slot_709)).p1 }) as isize) };
*__slate_slot_719 = unsafe { (*__slate_slot_717).offset((unsafe { (*(*__slate_slot_709)).p2 }) as isize) };
0 as i32;
if (((unsafe { (*(*__slate_slot_718)).flags }) as u32) as i32) & (16 as i32) == (0 as i32) {
if (unsafe { sqlite3VdbeMemSetRowSet(*__slate_slot_718) }) != (0 as i32) {
break '__join_1;
}
}
0 as i32;
unsafe { sqlite3RowSetInsert((unsafe { (*(*__slate_slot_718)).z }) as *mut RowSet, unsafe { (*(*__slate_slot_719)).u.i }) };
break '__join_26;
} else {
if __t4 == (48 as i32) {
// jump, in1, out3
*__slate_slot_718 = unsafe { (*__slate_slot_717).offset((unsafe { (*(*__slate_slot_709)).p1 }) as isize) };
0 as i32;
if (((unsafe { (*(*__slate_slot_718)).flags }) as u32) as i32) & (16 as i32) == (0 as i32) {
*__slate_slot_1859 = true as bool;
} else {
*__slate_slot_1859 = (unsafe { sqlite3RowSetNext((unsafe { (*(*__slate_slot_718)).z }) as *mut RowSet, std::ptr::addr_of_mut!(*__slate_slot_968)) }) == (0 as i32);
}
if *__slate_slot_1859 {
// The boolean index is empty
unsafe { sqlite3VdbeMemSetNull(*__slate_slot_718) };
{
}
break '__join_1140;
} else {
// A value was pulled from the index
{
}
unsafe { sqlite3VdbeMemSetInt64(unsafe { (*__slate_slot_717).offset((unsafe { (*(*__slate_slot_709)).p3 }) as isize) }, *__slate_slot_968) };
break '__join_1139;
}
} else {
if __t4 == (49 as i32) {
// jump, in1, in3
*__slate_slot_718 = unsafe { (*__slate_slot_717).offset((unsafe { (*(*__slate_slot_709)).p1 }) as isize) };
*__slate_slot_720 = unsafe { (*__slate_slot_717).offset((unsafe { (*(*__slate_slot_709)).p3 }) as isize) };
*__slate_slot_969 = unsafe { (*(*__slate_slot_709)).p4.i };
0 as i32;
// If there is anything other than a rowset object in memory cell P1,
// delete it now and initialize P1 with an empty rowset
if (((unsafe { (*(*__slate_slot_718)).flags }) as u32) as i32) & (16 as i32) == (0 as i32) {
if (unsafe { sqlite3VdbeMemSetRowSet(*__slate_slot_718) }) != (0 as i32) {
break '__join_1;
}
}
0 as i32;
0 as i32;
0 as i32;
if *__slate_slot_969 != (0 as i32) {
*__slate_slot_970 = unsafe { sqlite3RowSetTest((unsafe { (*(*__slate_slot_718)).z }) as *mut RowSet, *__slate_slot_969, unsafe { (*(*__slate_slot_720)).u.i }) };
{
}
if *__slate_slot_970 != (0 as i32) {
break '__join_1125;
}
}
if *__slate_slot_969 >= (0 as i32) {
unsafe { sqlite3RowSetInsert((unsafe { (*(*__slate_slot_718)).z }) as *mut RowSet, unsafe { (*(*__slate_slot_720)).u.i }) };
break '__join_26;
} else {
break '__join_26;
}
} else {
if __t4 == (50 as i32) {
// jump0
*__slate_slot_977 = unsafe { (*(*__slate_slot_709)).p4.pProgram };
*__slate_slot_973 = unsafe { (*__slate_slot_717).offset((unsafe { (*(*__slate_slot_709)).p3 }) as isize) };
0 as i32;
// If the p5 flag is clear, then recursive invocation of triggers is
// disabled for backwards compatibility (p5 is set if this sub-program
// is really a trigger, not a foreign key action, and the flag set
// and cleared by the "PRAGMA recursive_triggers" command is clear).
//
// It is recursive invocation of triggers, at the SQL level, that is
// disabled. In some cases a single trigger may generate more than one
// SubProgram (if the trigger may be executed with more than one different
// ON CONFLICT algorithm). SubProgram structures associated with a
// single trigger all have the same value for the SubProgram.token
// variable.
if (unsafe { (*(*__slate_slot_709)).p5 }) != (0 as u16) {
*__slate_slot_978 = unsafe { (*(*__slate_slot_977)).token };
*__slate_slot_976 = unsafe { (*p).pFrame };
loop {
if *__slate_slot_976 != std::ptr::null_mut::<VdbeFrame>() && (unsafe { (*(*__slate_slot_976)).token }) != *__slate_slot_978 {
{
}
*__slate_slot_976 = unsafe { (*(*__slate_slot_976)).pParent };
} else {
break;
}
}
if *__slate_slot_976 != std::ptr::null_mut::<VdbeFrame>() {
break '__join_26;
}
}
if (unsafe { (*p).nFrame }) >= unsafe { *unsafe { unsafe { (*(*__slate_slot_711)).aLimit.as_mut_ptr() as *mut i32 }.offset((10 as i32) as isize) } } {
break '__join_237;
} else {
'__join_228: {
// Register pRt is used to store the memory required to save the state
// of the current program, and the memory required at runtime to execute
// the trigger program. If this trigger has been fired before, then pRt
// is already allocated. Otherwise, it must be initialized.
if (((unsafe { (*(*__slate_slot_973)).flags }) as u32) as i32) & (16 as i32) == (0 as i32) {
// SubProgram.nMem is set to the number of memory cells used by the
// program stored in SubProgram.aOp. As well as these, one memory
// cell is required for each cursor used by the program. Set local
// variable nMem (and later, VdbeFrame.nChildMem) to this value.
*__slate_slot_971 = (unsafe { (*(*__slate_slot_977)).nMem }) + unsafe { (*(*__slate_slot_977)).nCsr };
0 as i32;
if (unsafe { (*(*__slate_slot_977)).nCsr }) == (0 as i32) {
std::ptr::write(__slate_slot_1860, *__slate_slot_971);
std::ptr::write(__slate_slot_1861, *__slate_slot_1860 + (1 as i32));
*__slate_slot_971 = *__slate_slot_1861;
}
*__slate_slot_972 = ((112 as u64).wrapping_add(((7 as i32) as i64) as u64) & ((!(7 as i32) as i64) as u64)).wrapping_add(((*__slate_slot_971 as i64) as u64).wrapping_mul(56 as u64)).wrapping_add((((unsafe { (*(*__slate_slot_977)).nCsr }) as i64) as u64).wrapping_mul(8 as u64)).wrapping_add(((((7 as i32) as i64) + ((unsafe { (*(*__slate_slot_977)).nOp }) as i64)) / ((8 as i32) as i64)) as u64) as i64;
*__slate_slot_976 = (unsafe { sqlite3DbMallocZero(*__slate_slot_711, *__slate_slot_972 as u64) }) as *mut VdbeFrame;
if !(*__slate_slot_976 != std::ptr::null_mut::<VdbeFrame>()) {
break '__join_1;
} else {
unsafe { sqlite3VdbeMemRelease(*__slate_slot_973) };
unsafe {
(*(*__slate_slot_973)).flags = (((16 as i32) | (4096 as i32)) as i16) as u16;
}
unsafe {
(*(*__slate_slot_973)).z = *__slate_slot_976 as *mut i8;
}
unsafe {
(*(*__slate_slot_973)).n = *__slate_slot_972 as i32;
}
unsafe {
(*(*__slate_slot_973)).xDel = unsafe { std::mem::transmute::<*const (), Option<unsafe extern "C-unwind" fn(*mut ())>>(sqlite3VdbeFrameMemDel as *const ()) };
}
unsafe {
(*(*__slate_slot_976)).v = p;
}
unsafe {
(*(*__slate_slot_976)).nChildMem = *__slate_slot_971;
}
unsafe {
(*(*__slate_slot_976)).nChildCsr = unsafe { (*(*__slate_slot_977)).nCsr };
}
unsafe {
(*(*__slate_slot_976)).pc = ((unsafe { (*__slate_slot_709).offset_from(*__slate_slot_708 as *mut VdbeOp) }) as i64) as i32;
}
unsafe {
(*(*__slate_slot_976)).aMem = unsafe { (*p).aMem };
}
unsafe {
(*(*__slate_slot_976)).nMem = unsafe { (*p).nMem };
}
unsafe {
(*(*__slate_slot_976)).apCsr = unsafe { (*p).apCsr };
}
unsafe {
(*(*__slate_slot_976)).nCursor = unsafe { (*p).nCursor };
}
unsafe {
(*(*__slate_slot_976)).aOp = unsafe { (*p).aOp };
}
unsafe {
(*(*__slate_slot_976)).nOp = unsafe { (*p).nOp };
}
unsafe {
(*(*__slate_slot_976)).token = unsafe { (*(*__slate_slot_977)).token };
}
*__slate_slot_975 = unsafe { ((unsafe { (*__slate_slot_976 as *mut u8).offset(((112 as u64).wrapping_add(((7 as i32) as i64) as u64) & ((!(7 as i32) as i64) as u64)) as isize) }) as *mut sqlite3_value).offset((unsafe { (*(*__slate_slot_976)).nChildMem }) as isize) };
*__slate_slot_974 = (unsafe { (*__slate_slot_976 as *mut u8).offset(((112 as u64).wrapping_add(((7 as i32) as i64) as u64) & ((!(7 as i32) as i64) as u64)) as isize) }) as *mut sqlite3_value;
loop {
if *__slate_slot_974 != *__slate_slot_975 {
unsafe {
(*(*__slate_slot_974)).flags = ((0 as i32) as i16) as u16;
}
unsafe {
(*(*__slate_slot_974)).db = *__slate_slot_711;
}
std::ptr::write(__slate_slot_1862, *__slate_slot_974);
std::ptr::write(__slate_slot_1863, unsafe { (*__slate_slot_1862).offset((1 as i32) as isize) });
*__slate_slot_974 = *__slate_slot_1863;
} else {
break '__join_228;
}
}
}
} else {
*__slate_slot_976 = (unsafe { (*(*__slate_slot_973)).z }) as *mut VdbeFrame;
0 as i32;
0 as i32;
0 as i32;
0 as i32;
}
}
std::ptr::write(__slate_slot_1864, p);
std::ptr::write(__slate_slot_1865, unsafe { (*(*__slate_slot_1864)).nFrame });
std::ptr::write(__slate_slot_1866, *__slate_slot_1865 + (1 as i32));
unsafe {
(*(*__slate_slot_1864)).nFrame = *__slate_slot_1866;
}
unsafe {
(*(*__slate_slot_976)).pParent = unsafe { (*p).pFrame };
}
unsafe {
(*(*__slate_slot_976)).lastRowid = unsafe { (*(*__slate_slot_711)).lastRowid };
}
unsafe {
(*(*__slate_slot_976)).nChange = unsafe { (*p).nChange };
}
unsafe {
(*(*__slate_slot_976)).nDbChange = unsafe { (*unsafe { (*p).db }).nChange };
}
0 as i32;
unsafe {
(*(*__slate_slot_976)).pAuxData = unsafe { (*p).pAuxData };
}
unsafe {
(*p).pAuxData = std::ptr::null_mut::<AuxData>();
}
unsafe {
(*p).nChange = (0 as i32) as i64;
}
unsafe {
(*p).pFrame = *__slate_slot_976;
}
std::ptr::write(__slate_slot_1867, (unsafe { (*__slate_slot_976 as *mut u8).offset(((112 as u64).wrapping_add(((7 as i32) as i64) as u64) & ((!(7 as i32) as i64) as u64)) as isize) }) as *mut sqlite3_value);
*__slate_slot_717 = *__slate_slot_1867;
unsafe {
(*p).aMem = *__slate_slot_1867;
}
unsafe {
(*p).nMem = unsafe { (*(*__slate_slot_976)).nChildMem };
}
unsafe {
(*p).nCursor = ((((unsafe { (*(*__slate_slot_976)).nChildCsr }) as i16) as u16) as u32) as i32;
}
unsafe {
(*p).apCsr = (unsafe { (*__slate_slot_717).offset((unsafe { (*p).nMem }) as isize) }) as *mut *mut VdbeCursor;
}
unsafe {
(*(*__slate_slot_976)).aOnce = (unsafe { unsafe { (*p).apCsr }.offset((unsafe { (*(*__slate_slot_977)).nCsr }) as isize) }) as *mut u8;
}
unsafe { memset((unsafe { (*(*__slate_slot_976)).aOnce }) as *mut (), 0 as i32, ((((unsafe { (*(*__slate_slot_977)).nOp }) + (7 as i32)) / (8 as i32)) as i64) as u64) };
std::ptr::write(__slate_slot_1868, unsafe { (*(*__slate_slot_977)).aOp });
*__slate_slot_708 = *__slate_slot_1868;
unsafe {
(*p).aOp = *__slate_slot_1868;
}
unsafe {
(*p).nOp = unsafe { (*(*__slate_slot_977)).nOp };
}
*__slate_slot_709 = unsafe { (*__slate_slot_708).offset(-(1 as i32) as isize) };
break '__join_1139;
}
} else {
if __t4 == (159 as i32) {
// out2
*__slate_slot_721 = out2Prerelease(p, *__slate_slot_709);
*__slate_slot_979 = unsafe { (*p).pFrame };
*__slate_slot_980 = unsafe { unsafe { (*(*__slate_slot_979)).aMem }.offset(((unsafe { (*(*__slate_slot_709)).p1 }) + unsafe { (*unsafe { unsafe { (*(*__slate_slot_979)).aOp }.offset((unsafe { (*(*__slate_slot_979)).pc }) as isize) }).p1 }) as isize) };
unsafe { sqlite3VdbeMemShallowCopy(*__slate_slot_721, *__slate_slot_980 as *const sqlite3_value, 16384 as i32) };
break '__join_26;
} else {
if __t4 == (160 as i32) {
if (unsafe { (*(*__slate_slot_709)).p1 }) != (0 as i32) {
std::ptr::write(__slate_slot_1869, *__slate_slot_711);
std::ptr::write(__slate_slot_1870, unsafe { (*(*__slate_slot_1869)).nDeferredCons });
std::ptr::write(__slate_slot_1871, *__slate_slot_1870 + ((unsafe { (*(*__slate_slot_709)).p2 }) as i64));
unsafe {
(*(*__slate_slot_1869)).nDeferredCons = *__slate_slot_1871;
}
break '__join_26;
} else {
if (unsafe { (*(*__slate_slot_711)).flags }) & (((524288 as i32) as i64) as u64) != (0 as u64) {
std::ptr::write(__slate_slot_1872, *__slate_slot_711);
std::ptr::write(__slate_slot_1873, unsafe { (*(*__slate_slot_1872)).nDeferredImmCons });
std::ptr::write(__slate_slot_1874, *__slate_slot_1873 + ((unsafe { (*(*__slate_slot_709)).p2 }) as i64));
unsafe {
(*(*__slate_slot_1872)).nDeferredImmCons = *__slate_slot_1874;
}
break '__join_26;
} else {
std::ptr::write(__slate_slot_1875, p);
std::ptr::write(__slate_slot_1876, unsafe { (*(*__slate_slot_1875)).nFkConstraint });
std::ptr::write(__slate_slot_1877, *__slate_slot_1876 + ((unsafe { (*(*__slate_slot_709)).p2 }) as i64));
unsafe {
(*(*__slate_slot_1875)).nFkConstraint = *__slate_slot_1877;
}
break '__join_26;
}
}
} else {
if __t4 == (60 as i32) {
// jump
if (unsafe { (*(*__slate_slot_709)).p1 }) != (0 as i32) {
{
}
if (unsafe { (*(*__slate_slot_711)).nDeferredCons }) == ((0 as i32) as i64) && (unsafe { (*(*__slate_slot_711)).nDeferredImmCons }) == ((0 as i32) as i64) {
break '__join_1125;
} else {
break '__join_26;
}
} else {
{
}
if (unsafe { (*p).nFkConstraint }) == ((0 as i32) as i64) && (unsafe { (*(*__slate_slot_711)).nDeferredImmCons }) == ((0 as i32) as i64) {
break '__join_1125;
} else {
break '__join_26;
}
}
} else {
if __t4 == (161 as i32) {
// in2
if (unsafe { (*p).pFrame }) != std::ptr::null_mut::<VdbeFrame>() {
*__slate_slot_981 = unsafe { (*p).pFrame };
loop {
if (unsafe { (*(*__slate_slot_981)).pParent }) != std::ptr::null_mut::<VdbeFrame>() {
{
}
*__slate_slot_981 = unsafe { (*(*__slate_slot_981)).pParent };
} else {
break;
}
}
*__slate_slot_718 = unsafe { unsafe { (*(*__slate_slot_981)).aMem }.offset((unsafe { (*(*__slate_slot_709)).p1 }) as isize) };
} else {
*__slate_slot_718 = unsafe { (*__slate_slot_717).offset((unsafe { (*(*__slate_slot_709)).p1 }) as isize) };
}
0 as i32;
unsafe { sqlite3VdbeMemIntegerify(*__slate_slot_718) };
*__slate_slot_719 = unsafe { (*__slate_slot_717).offset((unsafe { (*(*__slate_slot_709)).p2 }) as isize) };
unsafe { sqlite3VdbeMemIntegerify(*__slate_slot_719) };
if (unsafe { (*(*__slate_slot_718)).u.i }) < unsafe { (*(*__slate_slot_719)).u.i } {
unsafe {
(*(*__slate_slot_718)).u.i = unsafe { (*(*__slate_slot_719)).u.i };
}
break '__join_26;
} else {
break '__join_26;
}
} else {
if __t4 == (61 as i32) {
// jump, in1
*__slate_slot_718 = unsafe { (*__slate_slot_717).offset((unsafe { (*(*__slate_slot_709)).p1 }) as isize) };
0 as i32;
{
}
if (unsafe { (*(*__slate_slot_718)).u.i }) > ((0 as i32) as i64) {
std::ptr::write(__slate_slot_1878, *__slate_slot_718);
std::ptr::write(__slate_slot_1879, unsafe { (*(*__slate_slot_1878)).u.i });
std::ptr::write(__slate_slot_1880, *__slate_slot_1879 - ((unsafe { (*(*__slate_slot_709)).p3 }) as i64));
unsafe {
(*(*__slate_slot_1878)).u.i = *__slate_slot_1880;
}
break '__join_1125;
} else {
break '__join_26;
}
} else {
if __t4 == (162 as i32) {
// in1, out2, in3
*__slate_slot_718 = unsafe { (*__slate_slot_717).offset((unsafe { (*(*__slate_slot_709)).p1 }) as isize) };
*__slate_slot_720 = unsafe { (*__slate_slot_717).offset((unsafe { (*(*__slate_slot_709)).p3 }) as isize) };
*__slate_slot_721 = out2Prerelease(p, *__slate_slot_709);
0 as i32;
0 as i32;
*__slate_slot_982 = unsafe { (*(*__slate_slot_718)).u.i };
if *__slate_slot_982 <= ((0 as i32) as i64) {
*__slate_slot_1881 = true as bool;
} else {
*__slate_slot_1881 = (unsafe { sqlite3AddInt64(std::ptr::addr_of_mut!(*__slate_slot_982), if (unsafe { (*(*__slate_slot_720)).u.i }) > ((0 as i32) as i64) { unsafe { (*(*__slate_slot_720)).u.i } } else { (0 as i32) as i64 }) }) != (0 as i32);
}
if *__slate_slot_1881 {
// If the LIMIT is less than or equal to zero, loop forever.  This
// is documented.  But also, if the LIMIT+OFFSET exceeds 2^63 then
// also loop forever.  This is undocumented.  In fact, one could argue
// that the loop should terminate.  But assuming 1 billion iterations
// per second (far exceeding the capabilities of any current hardware)
// it would take nearly 300 years to actually reach the limit.  So
// looping forever is a reasonable approximation.
unsafe {
(*(*__slate_slot_721)).u.i = -(1 as i32) as i64;
}
break '__join_26;
} else {
unsafe {
(*(*__slate_slot_721)).u.i = *__slate_slot_982;
}
break '__join_26;
}
} else {
if __t4 == (62 as i32) {
// jump, in1
*__slate_slot_718 = unsafe { (*__slate_slot_717).offset((unsafe { (*(*__slate_slot_709)).p1 }) as isize) };
0 as i32;
{
}
if (unsafe { (*(*__slate_slot_718)).u.i }) != (0 as i64) {
if (unsafe { (*(*__slate_slot_718)).u.i }) > ((0 as i32) as i64) {
std::ptr::write(__slate_slot_1882, *__slate_slot_718);
std::ptr::write(__slate_slot_1883, unsafe { (*(*__slate_slot_1882)).u.i });
std::ptr::write(__slate_slot_1884, *__slate_slot_1883 - ((1 as i32) as i64));
unsafe {
(*(*__slate_slot_1882)).u.i = *__slate_slot_1884;
}
break '__join_1125;
} else {
break '__join_1125;
}
} else {
break '__join_26;
}
} else {
if __t4 == (63 as i32) {
// jump, in1
*__slate_slot_718 = unsafe { (*__slate_slot_717).offset((unsafe { (*(*__slate_slot_709)).p1 }) as isize) };
0 as i32;
if (unsafe { (*(*__slate_slot_718)).u.i }) > (-(1 as i32) as i64) - ((((4294967295 as u32) as u64) as i64) | ((2147483647 as i32) as i64) << (32 as i32)) {
std::ptr::write(__slate_slot_1885, *__slate_slot_718);
std::ptr::write(__slate_slot_1886, unsafe { (*(*__slate_slot_1885)).u.i });
std::ptr::write(__slate_slot_1887, *__slate_slot_1886 - ((1 as i32) as i64));
unsafe {
(*(*__slate_slot_1885)).u.i = *__slate_slot_1887;
}
}
{
}
if (unsafe { (*(*__slate_slot_718)).u.i }) == ((0 as i32) as i64) {
break '__join_1125;
} else {
break '__join_26;
}
} else {
if __t4 == (163 as i32) {
break '__join_196;
} else {
if __t4 == (164 as i32) {
break '__join_196;
} else {
if __t4 == (165 as i32) {
break '__join_194;
} else {
if __t4 == (166 as i32) {
break '__join_178;
} else {
if __t4 == (167 as i32) {
break '__join_178;
} else {
if __t4 == (3 as i32) {
0 as i32;
unsafe {
*unsafe { ((*__slate_slot_991).as_mut_ptr() as *mut i32).offset((0 as i32) as isize) } = 0 as i32;
}
std::ptr::write(__slate_slot_1893, -(1 as i32));
unsafe {
*unsafe { ((*__slate_slot_991).as_mut_ptr() as *mut i32).offset((2 as i32) as isize) } = *__slate_slot_1893;
}
unsafe {
*unsafe { ((*__slate_slot_991).as_mut_ptr() as *mut i32).offset((1 as i32) as isize) } = *__slate_slot_1893;
}
0 as i32;
*__slate_slot_710 = unsafe { sqlite3Checkpoint(*__slate_slot_711, unsafe { (*(*__slate_slot_709)).p1 }, unsafe { (*(*__slate_slot_709)).p2 }, unsafe { ((*__slate_slot_991).as_mut_ptr() as *mut i32).offset((1 as i32) as isize) }, unsafe { ((*__slate_slot_991).as_mut_ptr() as *mut i32).offset((2 as i32) as isize) }) };
if *__slate_slot_710 != (0 as i32) {
if *__slate_slot_710 != (5 as i32) {
break '__join_1180;
} else {
*__slate_slot_710 = 0 as i32;
unsafe {
*unsafe { ((*__slate_slot_991).as_mut_ptr() as *mut i32).offset((0 as i32) as isize) } = 1 as i32;
}
}
}
*__slate_slot_990 = 0 as i32;
std::ptr::write(__slate_slot_1894, unsafe { (*__slate_slot_717).offset((unsafe { (*(*__slate_slot_709)).p3 }) as isize) });
*__slate_slot_992 = *__slate_slot_1894;
loop {
if *__slate_slot_990 < (3 as i32) {
unsafe { sqlite3VdbeMemSetInt64(*__slate_slot_992, (unsafe { *unsafe { ((*__slate_slot_991).as_mut_ptr() as *mut i32).offset(*__slate_slot_990 as isize) } }) as i64) };
std::ptr::write(__slate_slot_1895, *__slate_slot_990);
std::ptr::write(__slate_slot_1896, *__slate_slot_1895 + (1 as i32));
*__slate_slot_990 = *__slate_slot_1896;
std::ptr::write(__slate_slot_1897, *__slate_slot_992);
std::ptr::write(__slate_slot_1898, unsafe { (*__slate_slot_1897).offset((1 as i32) as isize) });
*__slate_slot_992 = *__slate_slot_1898;
} else {
break '__join_26;
}
}
} else {
if __t4 == (4 as i32) {
// out2
*__slate_slot_721 = out2Prerelease(p, *__slate_slot_709);
*__slate_slot_995 = unsafe { (*(*__slate_slot_709)).p3 };
0 as i32;
0 as i32;
0 as i32;
*__slate_slot_993 = unsafe { (*unsafe { unsafe { (*(*__slate_slot_711)).aDb }.offset((unsafe { (*(*__slate_slot_709)).p1 }) as isize) }).pBt };
*__slate_slot_994 = unsafe { sqlite3BtreePager(*__slate_slot_993) };
*__slate_slot_996 = unsafe { sqlite3PagerGetJournalMode(*__slate_slot_994) };
if *__slate_slot_995 == -(1 as i32) {
*__slate_slot_995 = *__slate_slot_996;
}
0 as i32;
if !((unsafe { sqlite3PagerOkToChangeJournalMode(*__slate_slot_994) }) != (0 as i32)) {
*__slate_slot_995 = *__slate_slot_996;
}
*__slate_slot_997 = unsafe { sqlite3PagerFilename(*__slate_slot_994 as *const Pager, 1 as i32) };
if *__slate_slot_995 == (5 as i32) {
if (unsafe { sqlite3Strlen30(*__slate_slot_997) }) == (0 as i32) {
*__slate_slot_1900 = true as bool;
} else {
*__slate_slot_1900 = !((unsafe { sqlite3PagerWalSupported(*__slate_slot_994) }) != (0 as i32));
}
*__slate_slot_1899 = *__slate_slot_1900;
} else {
*__slate_slot_1899 = false as bool;
}
if *__slate_slot_1899 {
*__slate_slot_995 = *__slate_slot_996;
}
// Temp file
// No shared-memory support
if *__slate_slot_995 != *__slate_slot_996 && (*__slate_slot_996 == (5 as i32) || *__slate_slot_995 == (5 as i32)) {
if !((unsafe { (*(*__slate_slot_711)).autoCommit }) != (0 as u8)) || (unsafe { (*(*__slate_slot_711)).nVdbeRead }) > (1 as i32) {
break '__join_145;
} else {
if *__slate_slot_996 == (5 as i32) {
// If leaving WAL mode, close the log file. If successful, the call
// to PagerCloseWal() checkpoints and deletes the write-ahead-log
// file. An EXCLUSIVE lock may still be held on the database file
// after a successful return.
*__slate_slot_710 = unsafe { sqlite3PagerCloseWal(*__slate_slot_994, *__slate_slot_711) };
if *__slate_slot_710 == (0 as i32) {
unsafe { sqlite3PagerSetJournalMode(*__slate_slot_994, *__slate_slot_995) };
}
} else {
if *__slate_slot_996 == (4 as i32) {
// Cannot transition directly from MEMORY to WAL.  Use mode OFF
// as an intermediate
unsafe { sqlite3PagerSetJournalMode(*__slate_slot_994, 2 as i32) };
}
}
// Open a transaction on the database file. Regardless of the journal
// mode, this transaction always uses a rollback journal.
0 as i32;
if *__slate_slot_710 == (0 as i32) {
*__slate_slot_710 = unsafe { sqlite3BtreeSetVersion(*__slate_slot_993, if *__slate_slot_995 == (5 as i32) { 2 as i32 } else { 1 as i32 }) };
}
}
}
if *__slate_slot_710 != (0 as i32) {
*__slate_slot_995 = *__slate_slot_996;
}
*__slate_slot_995 = unsafe { sqlite3PagerSetJournalMode(*__slate_slot_994, *__slate_slot_995) };
unsafe {
(*(*__slate_slot_721)).flags = (((2 as i32) | (8192 as i32) | (512 as i32)) as i16) as u16;
}
unsafe {
(*(*__slate_slot_721)).z = (unsafe { sqlite3JournalModename(*__slate_slot_995) }) as *mut i8;
}
unsafe {
(*(*__slate_slot_721)).n = unsafe { sqlite3Strlen30((unsafe { (*(*__slate_slot_721)).z }) as *const i8) };
}
unsafe {
(*(*__slate_slot_721)).enc = ((1 as i32) as i8) as u8;
}
unsafe { sqlite3VdbeChangeEncoding(*__slate_slot_721, (*__slate_slot_713 as u32) as i32) };
if *__slate_slot_710 != (0 as i32) {
break '__join_1178;
} else {
break '__join_26;
}
} else {
if __t4 == (5 as i32) {
0 as i32;
*__slate_slot_710 = unsafe { sqlite3RunVacuum(unsafe { std::ptr::addr_of_mut!((*p).zErrMsg) }, *__slate_slot_711, unsafe { (*(*__slate_slot_709)).p1 }, if (unsafe { (*(*__slate_slot_709)).p2 }) != (0 as i32) { unsafe { (*__slate_slot_717).offset((unsafe { (*(*__slate_slot_709)).p2 }) as isize) } } else { std::ptr::null_mut::<sqlite3_value>() }) };
if *__slate_slot_710 != (0 as i32) {
break '__join_1177;
} else {
break '__join_26;
}
} else {
if __t4 == (64 as i32) {
// jump
0 as i32;
0 as i32;
0 as i32;
*__slate_slot_998 = unsafe { (*unsafe { unsafe { (*(*__slate_slot_711)).aDb }.offset((unsafe { (*(*__slate_slot_709)).p1 }) as isize) }).pBt };
*__slate_slot_710 = unsafe { sqlite3BtreeIncrVacuum(*__slate_slot_998) };
{
}
if *__slate_slot_710 != (0 as i32) {
if *__slate_slot_710 != (101 as i32) {
break '__join_1176;
} else {
*__slate_slot_710 = 0 as i32;
break '__join_1125;
}
} else {
break '__join_26;
}
} else {
if __t4 == (168 as i32) {
0 as i32;
if !((unsafe { (*(*__slate_slot_709)).p1 }) != (0 as i32)) {
unsafe { sqlite3ExpirePreparedStatements(*__slate_slot_711, unsafe { (*(*__slate_slot_709)).p2 }) };
break '__join_26;
} else {
unsafe {
(*p).__slate_bits_0.__set_expired(((unsafe { (*(*__slate_slot_709)).p2 }) + (1 as i32)) as u32);
}
break '__join_26;
}
} else {
if __t4 == (169 as i32) {
0 as i32;
*__slate_slot_999 = unsafe { *unsafe { unsafe { (*p).apCsr }.offset((unsafe { (*(*__slate_slot_709)).p1 }) as isize) } };
0 as i32;
0 as i32;
unsafe { sqlite3BtreeCursorPin(unsafe { (*(*__slate_slot_999)).uc.pCursor }) };
break '__join_26;
} else {
if __t4 == (170 as i32) {
0 as i32;
*__slate_slot_1000 = unsafe { *unsafe { unsafe { (*p).apCsr }.offset((unsafe { (*(*__slate_slot_709)).p1 }) as isize) } };
0 as i32;
0 as i32;
unsafe { sqlite3BtreeCursorUnpin(unsafe { (*(*__slate_slot_1000)).uc.pCursor }) };
break '__join_26;
} else {
if __t4 == (171 as i32) {
std::ptr::write(__slate_slot_1001, ((unsafe { (*(*__slate_slot_709)).p3 }) as i8) as u8);
if *__slate_slot_1001 != (0 as u8) || (((0 as i32) as i64) as u64) == (unsafe { (*(*__slate_slot_711)).flags }) & (((4 as i32) as i64) as u64) << (32 as i32) {
std::ptr::write(__slate_slot_1002, unsafe { (*(*__slate_slot_709)).p1 });
0 as i32;
0 as i32;
0 as i32;
*__slate_slot_710 = unsafe { sqlite3BtreeLockTable(unsafe { (*unsafe { unsafe { (*(*__slate_slot_711)).aDb }.offset(*__slate_slot_1002 as isize) }).pBt }, unsafe { (*(*__slate_slot_709)).p2 }, *__slate_slot_1001) };
if *__slate_slot_710 != (0 as i32) {
break '__join_130;
} else {
break '__join_26;
}
} else {
break '__join_26;
}
} else {
if __t4 == (172 as i32) {
*__slate_slot_1004 = unsafe { (*(*__slate_slot_709)).p4.pVtab };
*__slate_slot_710 = unsafe { sqlite3VtabBegin(*__slate_slot_711, *__slate_slot_1004) };
if *__slate_slot_1004 != std::ptr::null_mut::<VTable>() {
unsafe { sqlite3VtabImportErrmsg(p, unsafe { (*(*__slate_slot_1004)).pVtab }) };
}
if *__slate_slot_710 != (0 as i32) {
break '__join_1173;
} else {
break '__join_26;
}
} else {
if __t4 == (173 as i32) {
unsafe { memset(std::ptr::addr_of_mut!(*__slate_slot_1005) as *mut (), 0 as i32, 56 as u64) };
(*__slate_slot_1005).db = *__slate_slot_711;
// Because P2 is always a static string, it is impossible for the
// sqlite3VdbeMemCopy() to fail
0 as i32;
0 as i32;
*__slate_slot_710 = unsafe { sqlite3VdbeMemCopy(std::ptr::addr_of_mut!(*__slate_slot_1005), (unsafe { (*__slate_slot_717).offset((unsafe { (*(*__slate_slot_709)).p2 }) as isize) }) as *const sqlite3_value) };
0 as i32;
*__slate_slot_1006 = (unsafe { sqlite3_value_text(std::ptr::addr_of_mut!(*__slate_slot_1005)) }) as *const i8;
0 as i32;
if *__slate_slot_1006 != std::ptr::null::<i8>() {
*__slate_slot_710 = unsafe { sqlite3VtabCallCreate(*__slate_slot_711, unsafe { (*(*__slate_slot_709)).p1 }, *__slate_slot_1006, unsafe { std::ptr::addr_of_mut!((*p).zErrMsg) }) };
}
unsafe { sqlite3VdbeMemRelease(std::ptr::addr_of_mut!(*__slate_slot_1005)) };
if *__slate_slot_710 != (0 as i32) {
break '__join_1172;
} else {
break '__join_26;
}
} else {
if __t4 == (174 as i32) {
std::ptr::write(__slate_slot_1901, *__slate_slot_711);
std::ptr::write(__slate_slot_1902, unsafe { (*(*__slate_slot_1901)).nVDestroy });
std::ptr::write(__slate_slot_1903, *__slate_slot_1902 + (1 as i32));
unsafe {
(*(*__slate_slot_1901)).nVDestroy = *__slate_slot_1903;
}
*__slate_slot_710 = unsafe { sqlite3VtabCallDestroy(*__slate_slot_711, unsafe { (*(*__slate_slot_709)).p1 }, (unsafe { (*(*__slate_slot_709)).p4.z }) as *const i8) };
std::ptr::write(__slate_slot_1904, *__slate_slot_711);
std::ptr::write(__slate_slot_1905, unsafe { (*(*__slate_slot_1904)).nVDestroy });
std::ptr::write(__slate_slot_1906, *__slate_slot_1905 - (1 as i32));
unsafe {
(*(*__slate_slot_1904)).nVDestroy = *__slate_slot_1906;
}
0 as i32;
if *__slate_slot_710 != (0 as i32) {
break '__join_1171;
} else {
break '__join_26;
}
} else {
if __t4 == (175 as i32) {
// ncycle
0 as i32;
*__slate_slot_1007 = unsafe { *unsafe { unsafe { (*p).apCsr }.offset((unsafe { (*(*__slate_slot_709)).p1 }) as isize) } };
if *__slate_slot_1007 != std::ptr::null_mut::<VdbeCursor>() && (((unsafe { (*(*__slate_slot_1007)).eCurType }) as u32) as i32) == (2 as i32) && (unsafe { (*unsafe { (*(*__slate_slot_1007)).uc.pVCur }).pVtab }) == unsafe { (*unsafe { (*(*__slate_slot_709)).p4.pVtab }).pVtab } {
// This opcode is a no-op if the cursor is already open
break '__join_26;
} else {
*__slate_slot_1008 = std::ptr::null_mut::<sqlite3_vtab_cursor>();
*__slate_slot_1009 = unsafe { (*unsafe { (*(*__slate_slot_709)).p4.pVtab }).pVtab };
if *__slate_slot_1009 == std::ptr::null_mut::<sqlite3_vtab>() || (unsafe { (*(*__slate_slot_1009)).pModule }) == std::ptr::null::<sqlite3_module>() {
break '__join_118;
} else {
*__slate_slot_1010 = unsafe { (*(*__slate_slot_1009)).pModule };
*__slate_slot_710 = unsafe { unsafe { (*(*__slate_slot_1010)).xOpen }.unwrap()(*__slate_slot_1009, std::ptr::addr_of_mut!(*__slate_slot_1008)) };
unsafe { sqlite3VtabImportErrmsg(p, *__slate_slot_1009) };
if *__slate_slot_710 != (0 as i32) {
break '__join_1169;
} else {
// Initialize sqlite3_vtab_cursor base class
unsafe {
(*(*__slate_slot_1008)).pVtab = *__slate_slot_1009;
}
// Initialize vdbe cursor object
*__slate_slot_1007 = allocateCursor(p, unsafe { (*(*__slate_slot_709)).p1 }, 0 as i32, ((2 as i32) as i8) as u8);
if *__slate_slot_1007 != std::ptr::null_mut::<VdbeCursor>() {
unsafe {
(*(*__slate_slot_1007)).uc.pVCur = *__slate_slot_1008;
}
std::ptr::write(__slate_slot_1907, *__slate_slot_1009);
std::ptr::write(__slate_slot_1908, unsafe { (*(*__slate_slot_1907)).nRef });
std::ptr::write(__slate_slot_1909, *__slate_slot_1908 + (1 as i32));
unsafe {
(*(*__slate_slot_1907)).nRef = *__slate_slot_1909;
}
break '__join_26;
} else {
break '__join_115;
}
}
}
}
} else {
if __t4 == (176 as i32) {
// out2
std::ptr::write(__slate_slot_1014, std::ptr::null_mut::<i8>());
*__slate_slot_721 = unsafe { (*__slate_slot_717).offset((unsafe { (*(*__slate_slot_709)).p2 }) as isize) };
unsafe { sqlite3VdbeMemSetNull(*__slate_slot_721) }; // Innocent until proven guilty
0 as i32;
*__slate_slot_1011 = unsafe { (*(*__slate_slot_709)).p4.pTab };
0 as i32;
0 as i32;
0 as i32;
if (unsafe { (*(*__slate_slot_1011)).u.vtab.p }) == std::ptr::null_mut::<VTable>() {
break '__join_26;
} else {
*__slate_slot_1012 = unsafe { (*unsafe { (*(*__slate_slot_1011)).u.vtab.p }).pVtab };
0 as i32;
*__slate_slot_1013 = unsafe { (*(*__slate_slot_1012)).pModule };
0 as i32;
0 as i32;
0 as i32;
unsafe { sqlite3VtabLock(unsafe { (*(*__slate_slot_1011)).u.vtab.p }) };
0 as i32;
*__slate_slot_710 = unsafe { unsafe { (*(*__slate_slot_1013)).xIntegrity }.unwrap()(*__slate_slot_1012, (unsafe { (*unsafe { unsafe { (*(*__slate_slot_711)).aDb }.offset((unsafe { (*(*__slate_slot_709)).p1 }) as isize) }).zDbSName }) as *const i8, (unsafe { (*(*__slate_slot_1011)).zName }) as *const i8, unsafe { (*(*__slate_slot_709)).p3 }, std::ptr::addr_of_mut!(*__slate_slot_1014)) };
unsafe { sqlite3VtabUnlock(unsafe { (*(*__slate_slot_1011)).u.vtab.p }) };
if *__slate_slot_710 != (0 as i32) {
break '__join_111;
} else {
if *__slate_slot_1014 != std::ptr::null_mut::<i8>() {
unsafe { sqlite3VdbeMemSetStr(*__slate_slot_721, *__slate_slot_1014 as *const i8, -(1 as i32) as i64, ((1 as i32) as i8) as u8, unsafe { std::mem::transmute::<*const (), Option<unsafe extern "C-unwind" fn(*mut ())>>(sqlite3_free as *const ()) }) };
break '__join_26;
} else {
break '__join_26;
}
}
}
} else {
if __t4 == (177 as i32) {
// out2, ncycle
*__slate_slot_1015 = unsafe { *unsafe { unsafe { (*p).apCsr }.offset((unsafe { (*(*__slate_slot_709)).p1 }) as isize) } };
*__slate_slot_1016 = (unsafe { sqlite3_malloc64(16 as u64) }) as *mut ValueList;
if *__slate_slot_1016 == std::ptr::null_mut::<ValueList>() {
break '__join_1;
} else {
unsafe {
(*(*__slate_slot_1016)).pCsr = unsafe { (*(*__slate_slot_1015)).uc.pCursor };
}
unsafe {
(*(*__slate_slot_1016)).pOut = unsafe { (*__slate_slot_717).offset((unsafe { (*(*__slate_slot_709)).p3 }) as isize) };
}
*__slate_slot_721 = out2Prerelease(p, *__slate_slot_709);
unsafe {
(*(*__slate_slot_721)).flags = ((1 as i32) as i16) as u16;
}
unsafe { sqlite3VdbeMemSetPointer(*__slate_slot_721, *__slate_slot_1016 as *mut (), (b"ValueList\0".as_ptr() as *mut i8) as *const i8, unsafe { std::mem::transmute::<*const (), Option<unsafe extern "C-unwind" fn(*mut ())>>(sqlite3VdbeValueListFree as *const ()) }) };
break '__join_26;
}
} else {
if __t4 == (6 as i32) {
// jump, ncycle
*__slate_slot_1020 = unsafe { (*__slate_slot_717).offset((unsafe { (*(*__slate_slot_709)).p3 }) as isize) };
*__slate_slot_1021 = unsafe { (*__slate_slot_1020).offset((1 as i32) as isize) };
*__slate_slot_1024 = unsafe { *unsafe { unsafe { (*p).apCsr }.offset((unsafe { (*(*__slate_slot_709)).p1 }) as isize) } };
0 as i32;
{
}
0 as i32;
0 as i32;
*__slate_slot_1022 = unsafe { (*(*__slate_slot_1024)).uc.pVCur };
*__slate_slot_1023 = unsafe { (*(*__slate_slot_1022)).pVtab };
*__slate_slot_1019 = unsafe { (*(*__slate_slot_1023)).pModule };
// Grab the index number and argc parameters
0 as i32;
*__slate_slot_1017 = (unsafe { (*(*__slate_slot_1021)).u.i }) as i32;
*__slate_slot_1018 = (unsafe { (*(*__slate_slot_1020)).u.i }) as i32;
// Invoke the xFilter method
*__slate_slot_1027 = unsafe { (*p).apArg };
0 as i32;
*__slate_slot_1026 = 0 as i32;
loop {
if *__slate_slot_1026 < *__slate_slot_1017 {
unsafe {
*unsafe { (*__slate_slot_1027).offset(*__slate_slot_1026 as isize) } = unsafe { (*__slate_slot_1021).offset((*__slate_slot_1026 + (1 as i32)) as isize) };
}
std::ptr::write(__slate_slot_1910, *__slate_slot_1026);
std::ptr::write(__slate_slot_1911, *__slate_slot_1910 + (1 as i32));
*__slate_slot_1026 = *__slate_slot_1911;
} else {
break;
}
}
*__slate_slot_710 = unsafe { unsafe { (*(*__slate_slot_1019)).xFilter }.unwrap()(*__slate_slot_1022, *__slate_slot_1018, (unsafe { (*(*__slate_slot_709)).p4.z }) as *const i8, *__slate_slot_1017, *__slate_slot_1027) };
unsafe { sqlite3VtabImportErrmsg(p, *__slate_slot_1023) };
if *__slate_slot_710 != (0 as i32) {
break '__join_1167;
} else {
*__slate_slot_1025 = unsafe { unsafe { (*(*__slate_slot_1019)).xEof }.unwrap()(*__slate_slot_1022) };
unsafe {
(*(*__slate_slot_1024)).nullRow = ((0 as i32) as i8) as u8;
}
{
}
if *__slate_slot_1025 != (0 as i32) {
break '__join_1125;
} else {
break '__join_26;
}
}
} else {
if __t4 == (178 as i32) {
// ncycle
std::ptr::write(__slate_slot_1033, unsafe { *unsafe { unsafe { (*p).apCsr }.offset((unsafe { (*(*__slate_slot_709)).p1 }) as isize) } });
0 as i32;
0 as i32;
*__slate_slot_1030 = unsafe { (*__slate_slot_717).offset((unsafe { (*(*__slate_slot_709)).p3 }) as isize) };
{
}
if (unsafe { (*(*__slate_slot_1033)).nullRow }) != (0 as u8) {
unsafe { sqlite3VdbeMemSetNull(*__slate_slot_1030) };
break '__join_26;
} else {
0 as i32;
*__slate_slot_1028 = unsafe { (*unsafe { (*(*__slate_slot_1033)).uc.pVCur }).pVtab };
*__slate_slot_1029 = unsafe { (*(*__slate_slot_1028)).pModule };
0 as i32;
unsafe { memset(std::ptr::addr_of_mut!(*__slate_slot_1031) as *mut (), 0 as i32, 48 as u64) };
(*__slate_slot_1031).pOut = *__slate_slot_1030;
(*__slate_slot_1031).enc = *__slate_slot_713;
(*__slate_slot_1032).pUserData = std::ptr::null_mut::<()>();
(*__slate_slot_1032).funcFlags = (16777216 as i32) as u32;
(*__slate_slot_1031).pFunc = std::ptr::addr_of_mut!(*__slate_slot_1032);
0 as i32;
if (((unsafe { (*(*__slate_slot_709)).p5 }) as u32) as i32) & (1 as i32) != (0 as i32) {
unsafe { sqlite3VdbeMemSetNull(*__slate_slot_1030) };
unsafe {
(*(*__slate_slot_1030)).flags = (((1 as i32) | (1024 as i32)) as i16) as u16;
}
unsafe {
(*(*__slate_slot_1030)).u.nZero = 0 as i32;
}
} else {
unsafe {
(*(*__slate_slot_1030)).flags = (((((unsafe { (*(*__slate_slot_1030)).flags }) as u32) as i32) & !((3519 as i32) | (1024 as i32)) | (1 as i32)) as i16) as u16;
}
}
*__slate_slot_710 = unsafe { unsafe { (*(*__slate_slot_1029)).xColumn }.unwrap()(unsafe { (*(*__slate_slot_1033)).uc.pVCur }, std::ptr::addr_of_mut!(*__slate_slot_1031), unsafe { (*(*__slate_slot_709)).p2 }) };
unsafe { sqlite3VtabImportErrmsg(p, *__slate_slot_1028) };
if (*__slate_slot_1031).isError > (0 as i32) {
unsafe { sqlite3VdbeError(p, (b"%s\0".as_ptr() as *mut i8) as *const i8, unsafe { sqlite3_value_text(*__slate_slot_1030) }) };
*__slate_slot_710 = (*__slate_slot_1031).isError;
}
unsafe { sqlite3VdbeChangeEncoding(*__slate_slot_1030, (*__slate_slot_713 as u32) as i32) };
{
}
{
}
if *__slate_slot_710 != (0 as i32) {
break '__join_1166;
} else {
break '__join_26;
}
}
} else {
if __t4 == (65 as i32) {
// jump, ncycle
*__slate_slot_1037 = unsafe { *unsafe { unsafe { (*p).apCsr }.offset((unsafe { (*(*__slate_slot_709)).p1 }) as isize) } };
0 as i32;
0 as i32;
if (unsafe { (*(*__slate_slot_1037)).nullRow }) != (0 as u8) {
break '__join_26;
} else {
*__slate_slot_1034 = unsafe { (*unsafe { (*(*__slate_slot_1037)).uc.pVCur }).pVtab };
*__slate_slot_1035 = unsafe { (*(*__slate_slot_1034)).pModule };
0 as i32;
// Invoke the xNext() method of the module. There is no way for the
// underlying implementation to return an error if one occurs during
// xNext(). Instead, if an error occurs, true is returned (indicating that
// data is available) and the error code returned when xColumn or
// some other method is next invoked on the save virtual table cursor.
*__slate_slot_710 = unsafe { unsafe { (*(*__slate_slot_1035)).xNext }.unwrap()(unsafe { (*(*__slate_slot_1037)).uc.pVCur }) };
unsafe { sqlite3VtabImportErrmsg(p, *__slate_slot_1034) };
if *__slate_slot_710 != (0 as i32) {
break '__join_1165;
} else {
*__slate_slot_1036 = unsafe { unsafe { (*(*__slate_slot_1035)).xEof }.unwrap()(unsafe { (*(*__slate_slot_1037)).uc.pVCur }) };
{
}
if !(*__slate_slot_1036 != (0 as i32)) {
// If there is data, jump to P2
break '__join_1140;
} else {
break '__join_1139;
}
}
}
} else {
if __t4 == (179 as i32) {
*__slate_slot_1040 = (((unsafe { (*(*__slate_slot_711)).flags }) & (((67108864 as i32) as i64) as u64)) as u32) as i32;
std::ptr::write(__slate_slot_1912, *__slate_slot_711);
std::ptr::write(__slate_slot_1913, unsafe { (*(*__slate_slot_1912)).flags });
std::ptr::write(__slate_slot_1914, *__slate_slot_1913 | (((67108864 as i32) as i64) as u64));
unsafe {
(*(*__slate_slot_1912)).flags = *__slate_slot_1914;
}
*__slate_slot_1038 = unsafe { (*unsafe { (*(*__slate_slot_709)).p4.pVtab }).pVtab };
*__slate_slot_1039 = unsafe { (*__slate_slot_717).offset((unsafe { (*(*__slate_slot_709)).p1 }) as isize) };
0 as i32;
0 as i32;
0 as i32;
{
}
0 as i32;
{
}
{
}
{
}
*__slate_slot_710 = unsafe { sqlite3VdbeChangeEncoding(*__slate_slot_1039, 1 as i32) };
if *__slate_slot_710 != (0 as i32) {
break '__join_1164;
} else {
*__slate_slot_710 = unsafe { unsafe { (*unsafe { (*(*__slate_slot_1038)).pModule }).xRename }.unwrap()(*__slate_slot_1038, (unsafe { (*(*__slate_slot_1039)).z }) as *const i8) };
if *__slate_slot_1040 == (0 as i32) {
std::ptr::write(__slate_slot_1915, *__slate_slot_711);
std::ptr::write(__slate_slot_1916, unsafe { (*(*__slate_slot_1915)).flags });
std::ptr::write(__slate_slot_1917, *__slate_slot_1916 & !(((67108864 as i32) as i64) as u64));
unsafe {
(*(*__slate_slot_1915)).flags = *__slate_slot_1917;
}
}
unsafe { sqlite3VtabImportErrmsg(p, *__slate_slot_1038) };
unsafe {
(*p).__slate_bits_0.__set_expired((0 as i32) as u32);
}
if *__slate_slot_710 != (0 as i32) {
break '__join_1163;
} else {
break '__join_26;
}
}
} else {
if __t4 == (7 as i32) {
std::ptr::write(__slate_slot_1045, (0 as i32) as i64);
0 as i32;
0 as i32;
if (unsafe { (*(*__slate_slot_711)).mallocFailed }) != (0 as u8) {
break '__join_1;
} else {
{
}
*__slate_slot_1041 = unsafe { (*unsafe { (*(*__slate_slot_709)).p4.pVtab }).pVtab };
if *__slate_slot_1041 == std::ptr::null_mut::<sqlite3_vtab>() || (unsafe { (*(*__slate_slot_1041)).pModule }) == std::ptr::null::<sqlite3_module>() {
break '__join_83;
} else {
*__slate_slot_1042 = unsafe { (*(*__slate_slot_1041)).pModule };
*__slate_slot_1043 = unsafe { (*(*__slate_slot_709)).p2 };
0 as i32;
if (unsafe { (*(*__slate_slot_1042)).xUpdate }) != None {
std::ptr::write(__slate_slot_1048, unsafe { (*(*__slate_slot_711)).vtabOnConflict });
*__slate_slot_1046 = unsafe { (*p).apArg };
*__slate_slot_1047 = unsafe { (*__slate_slot_717).offset((unsafe { (*(*__slate_slot_709)).p3 }) as isize) };
0 as i32;
*__slate_slot_1044 = 0 as i32;
loop {
if *__slate_slot_1044 < *__slate_slot_1043 {
0 as i32;
{
}
unsafe {
*unsafe { (*__slate_slot_1046).offset(*__slate_slot_1044 as isize) } = *__slate_slot_1047;
}
std::ptr::write(__slate_slot_1920, *__slate_slot_1047);
std::ptr::write(__slate_slot_1921, unsafe { (*__slate_slot_1920).offset((1 as i32) as isize) });
*__slate_slot_1047 = *__slate_slot_1921;
std::ptr::write(__slate_slot_1918, *__slate_slot_1044);
std::ptr::write(__slate_slot_1919, *__slate_slot_1918 + (1 as i32));
*__slate_slot_1044 = *__slate_slot_1919;
} else {
break;
}
}
unsafe {
(*(*__slate_slot_711)).vtabOnConflict = (unsafe { (*(*__slate_slot_709)).p5 }) as u8;
}
*__slate_slot_710 = unsafe { unsafe { (*(*__slate_slot_1042)).xUpdate }.unwrap()(*__slate_slot_1041, *__slate_slot_1043, *__slate_slot_1046, std::ptr::addr_of_mut!(*__slate_slot_1045)) };
unsafe {
(*(*__slate_slot_711)).vtabOnConflict = *__slate_slot_1048;
}
unsafe { sqlite3VtabImportErrmsg(p, *__slate_slot_1041) };
if *__slate_slot_710 == (0 as i32) && (unsafe { (*(*__slate_slot_709)).p1 }) != (0 as i32) {
0 as i32;
unsafe {
(*(*__slate_slot_711)).lastRowid = *__slate_slot_1045;
}
}
if *__slate_slot_710 & (255 as i32) == (19 as i32) && (unsafe { (*unsafe { (*(*__slate_slot_709)).p4.pVtab }).bConstraint }) != (0 as u8) {
if (((unsafe { (*(*__slate_slot_709)).p5 }) as u32) as i32) == (4 as i32) {
*__slate_slot_710 = 0 as i32;
} else {
unsafe {
(*p).errorAction = ((if (((unsafe { (*(*__slate_slot_709)).p5 }) as u32) as i32) == (5 as i32) { 2 as i32 } else { ((unsafe { (*(*__slate_slot_709)).p5 }) as u32) as i32 }) as i8) as u8;
}
}
} else {
std::ptr::write(__slate_slot_1922, p);
std::ptr::write(__slate_slot_1923, unsafe { (*(*__slate_slot_1922)).nChange });
std::ptr::write(__slate_slot_1924, *__slate_slot_1923 + ((1 as i32) as i64));
unsafe {
(*(*__slate_slot_1922)).nChange = *__slate_slot_1924;
}
}
if *__slate_slot_710 != (0 as i32) {
break '__join_1161;
} else {
break '__join_26;
}
} else {
break '__join_26;
}
}
}
} else {
if __t4 == (180 as i32) {
// out2
*__slate_slot_721 = out2Prerelease(p, *__slate_slot_709);
unsafe {
(*(*__slate_slot_721)).u.i = ((unsafe { sqlite3BtreeLastPage(unsafe { (*unsafe { unsafe { (*(*__slate_slot_711)).aDb }.offset((unsafe { (*(*__slate_slot_709)).p1 }) as isize) }).pBt }) }) as u64) as i64;
}
break '__join_26;
} else {
if __t4 == (181 as i32) {
// out2
*__slate_slot_721 = out2Prerelease(p, *__slate_slot_709);
*__slate_slot_1050 = unsafe { (*unsafe { unsafe { (*(*__slate_slot_711)).aDb }.offset((unsafe { (*(*__slate_slot_709)).p1 }) as isize) }).pBt };
*__slate_slot_1049 = (0 as i32) as u32;
if (unsafe { (*(*__slate_slot_709)).p3 }) != (0 as i32) {
*__slate_slot_1049 = unsafe { sqlite3BtreeLastPage(*__slate_slot_1050) };
if *__slate_slot_1049 < ((unsafe { (*(*__slate_slot_709)).p3 }) as u32) {
*__slate_slot_1049 = (unsafe { (*(*__slate_slot_709)).p3 }) as u32;
}
}
unsafe {
(*(*__slate_slot_721)).u.i = ((unsafe { sqlite3BtreeMaxPageCount(*__slate_slot_1050, *__slate_slot_1049) }) as u64) as i64;
}
break '__join_26;
} else {
if __t4 == (67 as i32) {
break '__join_65;
} else {
if __t4 == (68 as i32) {
break '__join_65;
} else {
if __t4 == (182 as i32) {
// in1
*__slate_slot_718 = unsafe { (*__slate_slot_717).offset((unsafe { (*(*__slate_slot_709)).p1 }) as isize) };
std::ptr::write(__slate_slot_1927, *__slate_slot_718);
std::ptr::write(__slate_slot_1928, unsafe { (*(*__slate_slot_1927)).flags });
std::ptr::write(__slate_slot_1929, ((((*__slate_slot_1928 as u32) as i32) & !(2048 as i32)) as i16) as u16);
unsafe {
(*(*__slate_slot_1927)).flags = *__slate_slot_1929;
}
break '__join_26;
} else {
if __t4 == (183 as i32) {
// in1 out2
*__slate_slot_718 = unsafe { (*__slate_slot_717).offset((unsafe { (*(*__slate_slot_709)).p1 }) as isize) };
*__slate_slot_721 = unsafe { (*__slate_slot_717).offset((unsafe { (*(*__slate_slot_709)).p2 }) as isize) };
if (((unsafe { (*(*__slate_slot_718)).flags }) as u32) as i32) & (2048 as i32) != (0 as i32) {
unsafe { sqlite3VdbeMemSetInt64(*__slate_slot_721, ((unsafe { (*(*__slate_slot_718)).eSubtype }) as u64) as i64) };
break '__join_26;
} else {
unsafe { sqlite3VdbeMemSetNull(*__slate_slot_721) };
break '__join_26;
}
} else {
if __t4 == (184 as i32) {
// in1 out2
*__slate_slot_718 = unsafe { (*__slate_slot_717).offset((unsafe { (*(*__slate_slot_709)).p1 }) as isize) };
*__slate_slot_721 = unsafe { (*__slate_slot_717).offset((unsafe { (*(*__slate_slot_709)).p2 }) as isize) };
if (((unsafe { (*(*__slate_slot_718)).flags }) as u32) as i32) & (1 as i32) != (0 as i32) {
std::ptr::write(__slate_slot_1930, *__slate_slot_721);
std::ptr::write(__slate_slot_1931, unsafe { (*(*__slate_slot_1930)).flags });
std::ptr::write(__slate_slot_1932, ((((*__slate_slot_1931 as u32) as i32) & !(2048 as i32)) as i16) as u16);
unsafe {
(*(*__slate_slot_1930)).flags = *__slate_slot_1932;
}
break '__join_26;
} else {
0 as i32;
std::ptr::write(__slate_slot_1933, *__slate_slot_721);
std::ptr::write(__slate_slot_1934, unsafe { (*(*__slate_slot_1933)).flags });
std::ptr::write(__slate_slot_1935, ((((*__slate_slot_1934 as u32) as i32) | (2048 as i32)) as i16) as u16);
unsafe {
(*(*__slate_slot_1933)).flags = *__slate_slot_1935;
}
unsafe {
(*(*__slate_slot_721)).eSubtype = (((unsafe { (*(*__slate_slot_718)).u.i }) & ((255 as i32) as i64)) as i8) as u8;
}
break '__join_26;
}
} else {
if __t4 == (185 as i32) {
0 as i32;
*__slate_slot_718 = unsafe { (*__slate_slot_717).offset((unsafe { (*(*__slate_slot_709)).p1 }) as isize) };
0 as i32;
0 as i32;
*__slate_slot_1053 = filterHash(*__slate_slot_717 as *const sqlite3_value, *__slate_slot_709 as *const VdbeOp);
std::ptr::write(__slate_slot_1936, *__slate_slot_1053);
std::ptr::write(__slate_slot_1937, *__slate_slot_1936 % ((((unsafe { (*(*__slate_slot_718)).n }) * (8 as i32)) as i64) as u64));
*__slate_slot_1053 = *__slate_slot_1937;
std::ptr::write(__slate_slot_1938, unsafe { unsafe { (*(*__slate_slot_718)).z }.offset((*__slate_slot_1053 / (((8 as i32) as i64) as u64)) as isize) });
std::ptr::write(__slate_slot_1939, unsafe { *(*__slate_slot_1938) });
std::ptr::write(__slate_slot_1940, ((*__slate_slot_1939 as i32) | (1 as i32) << (*__slate_slot_1053 & (((7 as i32) as i64) as u64))) as i8);
unsafe {
*(*__slate_slot_1938) = *__slate_slot_1940;
}
break '__join_26;
} else {
if __t4 == (66 as i32) {
// jump
0 as i32;
*__slate_slot_718 = unsafe { (*__slate_slot_717).offset((unsafe { (*(*__slate_slot_709)).p1 }) as isize) };
0 as i32;
0 as i32;
*__slate_slot_1054 = filterHash(*__slate_slot_717 as *const sqlite3_value, *__slate_slot_709 as *const VdbeOp);
std::ptr::write(__slate_slot_1941, *__slate_slot_1054);
std::ptr::write(__slate_slot_1942, *__slate_slot_1941 % ((((unsafe { (*(*__slate_slot_718)).n }) * (8 as i32)) as i64) as u64));
*__slate_slot_1054 = *__slate_slot_1942;
if ((unsafe { *unsafe { unsafe { (*(*__slate_slot_718)).z }.offset((*__slate_slot_1054 / (((8 as i32) as i64) as u64)) as isize) } }) as i32) & (1 as i32) << (*__slate_slot_1054 & (((7 as i32) as i64) as u64)) == (0 as i32) {
{
}
std::ptr::write(__slate_slot_1943, unsafe { unsafe { (*p).aCounter.as_mut_ptr() as *mut u32 }.offset((8 as i32) as isize) });
std::ptr::write(__slate_slot_1944, unsafe { *(*__slate_slot_1943) });
std::ptr::write(__slate_slot_1945, (*__slate_slot_1944).wrapping_add((1 as i32) as u32));
unsafe {
*(*__slate_slot_1943) = *__slate_slot_1945;
}
break '__join_1125;
} else {
std::ptr::write(__slate_slot_1946, unsafe { unsafe { (*p).aCounter.as_mut_ptr() as *mut u32 }.offset((7 as i32) as isize) });
std::ptr::write(__slate_slot_1947, unsafe { *(*__slate_slot_1946) });
std::ptr::write(__slate_slot_1948, (*__slate_slot_1947).wrapping_add((1 as i32) as u32));
unsafe {
*(*__slate_slot_1946) = *__slate_slot_1948;
}
{
}
break '__join_26;
}
} else {
if __t4 == (186 as i32) {
} else {
if __t4 == (8 as i32) {
} else {
// This is really OP_Noop, OP_Explain
0 as i32;
break '__join_26;
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
// jump0
// If the P4 argument is not NULL, then it must be an SQL comment string.
// The "--" string is broken up to prevent false-positives with srcck1.c.
//
// This assert() provides evidence for:
// EVIDENCE-OF: R-50676-09860 The callback can compute the same text that
// would have been returned by the legacy sqlite3_trace() interface by
// using the X argument when X begins with "--" and invoking
// sqlite3_expanded_sql(P) otherwise.
0 as i32;
// OP_Init is always instruction 0
0 as i32;
if (((unsafe { (*(*__slate_slot_711)).mTrace }) as u32) as i32) & ((1 as i32) | (64 as i32)) != (0 as i32) && (((unsafe { (*p).minWriteFileFormat }) as u32) as i32) != (254 as i32) {
std::ptr::write(__slate_slot_1950, if (unsafe { (*(*__slate_slot_709)).p4.z }) != std::ptr::null_mut::<i8>() { unsafe { (*(*__slate_slot_709)).p4.z } } else { unsafe { (*p).zSql } });
*__slate_slot_1056 = *__slate_slot_1950;
*__slate_slot_1949 = *__slate_slot_1950 != std::ptr::null_mut::<i8>();
} else {
*__slate_slot_1949 = false as bool;
}
if *__slate_slot_1949 {
if (((unsafe { (*(*__slate_slot_711)).mTrace }) as u32) as i32) & (64 as i32) != (0 as i32) {
std::ptr::write(__slate_slot_1057, unsafe { sqlite3VdbeExpandSql(p, *__slate_slot_1056 as *const i8) });
unsafe { unsafe { (*(*__slate_slot_711)).trace.xLegacy }.unwrap()(unsafe { (*(*__slate_slot_711)).pTraceArg }, *__slate_slot_1057 as *const i8) };
unsafe { sqlite3_free(*__slate_slot_1057 as *mut ()) };
} else {
if (unsafe { (*(*__slate_slot_711)).nVdbeExec }) > (1 as i32) {
std::ptr::write(__slate_slot_1058, unsafe { sqlite3MPrintf(*__slate_slot_711, (b"-- %s\0".as_ptr() as *mut i8) as *const i8, *__slate_slot_1056) });
unsafe { unsafe { (*(*__slate_slot_711)).trace.xV2 }.unwrap()((1 as i32) as u32, unsafe { (*(*__slate_slot_711)).pTraceArg }, p as *mut (), *__slate_slot_1058 as *mut ()) };
unsafe { sqlite3DbFree(*__slate_slot_711, *__slate_slot_1058 as *mut ()) };
} else {
unsafe { unsafe { (*(*__slate_slot_711)).trace.xV2 }.unwrap()((1 as i32) as u32, unsafe { (*(*__slate_slot_711)).pTraceArg }, p as *mut (), *__slate_slot_1056 as *mut ()) };
}
}
}
// tag-20220401a
0 as i32;
if (unsafe { (*(*__slate_slot_709)).p1 }) >= unsafe { sqlite3Config.iOnceResetThreshold } {
if (((unsafe { (*(*__slate_slot_709)).opcode }) as u32) as i32) == (186 as i32) {
break '__join_26;
} else {
*__slate_slot_1055 = 1 as i32;
loop {
if *__slate_slot_1055 < unsafe { (*p).nOp } {
if (((unsafe { (*unsafe { unsafe { (*p).aOp }.offset(*__slate_slot_1055 as isize) }).opcode }) as u32) as i32) == (15 as i32) {
unsafe {
(*unsafe { unsafe { (*p).aOp }.offset(*__slate_slot_1055 as isize) }).p1 = 0 as i32;
}
}
std::ptr::write(__slate_slot_1951, *__slate_slot_1055);
std::ptr::write(__slate_slot_1952, *__slate_slot_1951 + (1 as i32));
*__slate_slot_1055 = *__slate_slot_1952;
} else {
break;
}
}
unsafe {
(*(*__slate_slot_709)).p1 = 0 as i32;
}
}
}
std::ptr::write(__slate_slot_1953, *__slate_slot_709);
std::ptr::write(__slate_slot_1954, unsafe { (*(*__slate_slot_1953)).p1 });
std::ptr::write(__slate_slot_1955, *__slate_slot_1954 + (1 as i32));
unsafe {
(*(*__slate_slot_1953)).p1 = *__slate_slot_1955;
}
std::ptr::write(__slate_slot_1956, unsafe { unsafe { (*p).aCounter.as_mut_ptr() as *mut u32 }.offset((6 as i32) as isize) });
std::ptr::write(__slate_slot_1957, unsafe { *(*__slate_slot_1956) });
std::ptr::write(__slate_slot_1958, (*__slate_slot_1957).wrapping_add((1 as i32) as u32));
unsafe {
*(*__slate_slot_1956) = *__slate_slot_1958;
}
break '__join_1125;
}
'__join_61: {
// group
0 as i32;
*__slate_slot_1052 = unsafe { (*(*__slate_slot_709)).p4.pCtx };
// If this function is inside of a trigger, the register array in aMem[]
// might change from one evaluation to the next.  The next block of code
// checks to see if the register array has changed, and if so it
// reinitializes the relevant parts of the sqlite3_context object
*__slate_slot_721 = unsafe { (*__slate_slot_717).offset((unsafe { (*(*__slate_slot_709)).p3 }) as isize) };
if (unsafe { (*(*__slate_slot_1052)).pOut }) != *__slate_slot_721 {
unsafe {
(*(*__slate_slot_1052)).pVdbe = p;
}
unsafe {
(*(*__slate_slot_1052)).pOut = *__slate_slot_721;
}
unsafe {
(*(*__slate_slot_1052)).enc = *__slate_slot_713;
}
*__slate_slot_1051 = (((unsafe { (*(*__slate_slot_1052)).argc }) as u32) as i32) - (1 as i32);
loop {
if *__slate_slot_1051 >= (0 as i32) {
unsafe {
*unsafe { unsafe { std::ptr::addr_of_mut!((*(*__slate_slot_1052)).argv) as *mut *mut sqlite3_value }.offset(*__slate_slot_1051 as isize) } = unsafe { (*__slate_slot_717).offset(((unsafe { (*(*__slate_slot_709)).p2 }) + *__slate_slot_1051) as isize) };
}
std::ptr::write(__slate_slot_1925, *__slate_slot_1051);
std::ptr::write(__slate_slot_1926, *__slate_slot_1925 - (1 as i32));
*__slate_slot_1051 = *__slate_slot_1926;
} else {
break '__join_61;
}
}
}
}
0 as i32;
{
}
unsafe {
(*(*__slate_slot_721)).flags = (((((unsafe { (*(*__slate_slot_721)).flags }) as u32) as i32) & !((3519 as i32) | (1024 as i32)) | (1 as i32)) as i16) as u16;
}
0 as i32;
unsafe { unsafe { (*unsafe { (*(*__slate_slot_1052)).pFunc }).xSFunc }.unwrap()(*__slate_slot_1052, ((unsafe { (*(*__slate_slot_1052)).argc }) as u32) as i32, unsafe { std::ptr::addr_of_mut!((*(*__slate_slot_1052)).argv) as *mut *mut sqlite3_value }) }; // IMP: R-24505-23230
// If the function returned an error, throw an exception
if (unsafe { (*(*__slate_slot_1052)).isError }) != (0 as i32) {
if (unsafe { (*(*__slate_slot_1052)).isError }) > (0 as i32) {
unsafe { sqlite3VdbeError(p, (b"%s\0".as_ptr() as *mut i8) as *const i8, unsafe { sqlite3_value_text(*__slate_slot_721) }) };
*__slate_slot_710 = unsafe { (*(*__slate_slot_1052)).isError };
}
unsafe { sqlite3VdbeDeleteAuxData(*__slate_slot_711, unsafe { std::ptr::addr_of_mut!((*p).pAuxData) }, unsafe { (*(*__slate_slot_1052)).iOp }, unsafe { (*(*__slate_slot_709)).p1 }) };
unsafe {
(*(*__slate_slot_1052)).isError = 0 as i32;
}
if *__slate_slot_710 != (0 as i32) {
break '__join_1160;
}
}
0 as i32;
0 as i32;
{
}
{
}
break '__join_26;
}
0 as i32;
0 as i32;
*__slate_slot_989 = unsafe { (*__slate_slot_717).offset((unsafe { (*(*__slate_slot_709)).p1 }) as isize) };
0 as i32;
if (unsafe { (*(*__slate_slot_709)).p3 }) != (0 as i32) {
{
}
*__slate_slot_710 = unsafe { sqlite3VdbeMemAggValue(*__slate_slot_989, unsafe { (*__slate_slot_717).offset((unsafe { (*(*__slate_slot_709)).p3 }) as isize) }, unsafe { (*(*__slate_slot_709)).p4.pFunc }) };
*__slate_slot_989 = unsafe { (*__slate_slot_717).offset((unsafe { (*(*__slate_slot_709)).p3 }) as isize) };
} else {
*__slate_slot_710 = unsafe { sqlite3VdbeMemFinalize(*__slate_slot_989, unsafe { (*(*__slate_slot_709)).p4.pFunc }) };
}
if *__slate_slot_710 != (0 as i32) {
break '__join_174;
} else {
unsafe { sqlite3VdbeChangeEncoding(*__slate_slot_989, (*__slate_slot_713 as u32) as i32) };
{
}
{
}
break '__join_26;
}
}
0 as i32;
*__slate_slot_983 = ((unsafe { (*(*__slate_slot_709)).p5 }) as u32) as i32;
0 as i32;
0 as i32;
0 as i32;
// Allocate space for (a) the context object and (n-1) extra pointers
// to append to the sqlite3_context.argv[1] array, and (b) a memory
// cell in which to store the accumulation. Be careful that the memory
// cell is 8-byte aligned, even on platforms where a pointer is 32-bits.
//
// Note: We could avoid this by using a regular memory cell from aMem[] for
// the accumulator, instead of allocating one here.
*__slate_slot_985 = (48 as u64).wrapping_add(((*__slate_slot_983 as i64) as u64).wrapping_mul(8 as u64));
*__slate_slot_984 = (unsafe { sqlite3DbMallocRawNN(*__slate_slot_711, (*__slate_slot_985).wrapping_add(56 as u64)) }) as *mut sqlite3_context;
if *__slate_slot_984 == std::ptr::null_mut::<sqlite3_context>() {
break '__join_1;
} else {
unsafe {
(*(*__slate_slot_984)).pOut = (unsafe { (*__slate_slot_984 as *mut u8).offset(*__slate_slot_985 as isize) }) as *mut sqlite3_value;
}
0 as i32;
unsafe { sqlite3VdbeMemInit(unsafe { (*(*__slate_slot_984)).pOut }, *__slate_slot_711, ((1 as i32) as i16) as u16) };
unsafe {
(*(*__slate_slot_984)).pMem = std::ptr::null_mut::<sqlite3_value>();
}
unsafe {
(*(*__slate_slot_984)).pFunc = unsafe { (*(*__slate_slot_709)).p4.pFunc };
}
unsafe {
(*(*__slate_slot_984)).iOp = ((unsafe { (*__slate_slot_709).offset_from(*__slate_slot_708 as *mut VdbeOp) }) as i64) as i32;
}
unsafe {
(*(*__slate_slot_984)).pVdbe = p;
}
unsafe {
(*(*__slate_slot_984)).skipFlag = ((0 as i32) as i8) as u8;
}
unsafe {
(*(*__slate_slot_984)).isError = 0 as i32;
}
unsafe {
(*(*__slate_slot_984)).enc = *__slate_slot_713;
}
unsafe {
(*(*__slate_slot_984)).argc = (*__slate_slot_983 as i16) as u16;
}
unsafe {
(*(*__slate_slot_709)).p4type = -(14 as i32) as i8;
}
unsafe {
(*(*__slate_slot_709)).p4.pCtx = *__slate_slot_984;
}
// OP_AggInverse must have P1==1 and OP_AggStep must have P1==0
0 as i32;
unsafe {
(*(*__slate_slot_709)).opcode = ((165 as i32) as i8) as u8;
}
// Fall through into OP_AggStep
//
// no break
{
}
}
}
'__join_190: {
0 as i32;
*__slate_slot_987 = unsafe { (*(*__slate_slot_709)).p4.pCtx };
*__slate_slot_988 = unsafe { (*__slate_slot_717).offset((unsafe { (*(*__slate_slot_709)).p3 }) as isize) };
// If this function is inside of a trigger, the register array in aMem[]
// might change from one evaluation to the next.  The next block of code
// checks to see if the register array has changed, and if so it
// reinitializes the relevant parts of the sqlite3_context object
if (unsafe { (*(*__slate_slot_987)).pMem }) != *__slate_slot_988 {
unsafe {
(*(*__slate_slot_987)).pMem = *__slate_slot_988;
}
*__slate_slot_986 = (((unsafe { (*(*__slate_slot_987)).argc }) as u32) as i32) - (1 as i32);
loop {
if *__slate_slot_986 >= (0 as i32) {
unsafe {
*unsafe { unsafe { std::ptr::addr_of_mut!((*(*__slate_slot_987)).argv) as *mut *mut sqlite3_value }.offset(*__slate_slot_986 as isize) } = unsafe { (*__slate_slot_717).offset(((unsafe { (*(*__slate_slot_709)).p2 }) + *__slate_slot_986) as isize) };
}
std::ptr::write(__slate_slot_1888, *__slate_slot_986);
std::ptr::write(__slate_slot_1889, *__slate_slot_1888 - (1 as i32));
*__slate_slot_986 = *__slate_slot_1889;
} else {
break '__join_190;
}
}
}
}
std::ptr::write(__slate_slot_1890, *__slate_slot_988);
std::ptr::write(__slate_slot_1891, unsafe { (*(*__slate_slot_1890)).n });
std::ptr::write(__slate_slot_1892, *__slate_slot_1891 + (1 as i32));
unsafe {
(*(*__slate_slot_1890)).n = *__slate_slot_1892;
}
0 as i32;
0 as i32;
0 as i32;
if (unsafe { (*(*__slate_slot_709)).p1 }) != (0 as i32) {
unsafe { unsafe { (*unsafe { (*(*__slate_slot_987)).pFunc }).xInverse }.unwrap()(*__slate_slot_987, ((unsafe { (*(*__slate_slot_987)).argc }) as u32) as i32, unsafe { std::ptr::addr_of_mut!((*(*__slate_slot_987)).argv) as *mut *mut sqlite3_value }) };
} else {
unsafe { unsafe { (*unsafe { (*(*__slate_slot_987)).pFunc }).xSFunc }.unwrap()(*__slate_slot_987, ((unsafe { (*(*__slate_slot_987)).argc }) as u32) as i32, unsafe { std::ptr::addr_of_mut!((*(*__slate_slot_987)).argv) as *mut *mut sqlite3_value }) };
}
// IMP: R-24505-23230
if (unsafe { (*(*__slate_slot_987)).isError }) != (0 as i32) {
if (unsafe { (*(*__slate_slot_987)).isError }) > (0 as i32) {
unsafe { sqlite3VdbeError(p, (b"%s\0".as_ptr() as *mut i8) as *const i8, unsafe { sqlite3_value_text(unsafe { (*(*__slate_slot_987)).pOut }) }) };
*__slate_slot_710 = unsafe { (*(*__slate_slot_987)).isError };
}
if (unsafe { (*(*__slate_slot_987)).skipFlag }) != (0 as u8) {
0 as i32;
*__slate_slot_986 = unsafe { (*unsafe { (*__slate_slot_709).offset(-(1 as i32) as isize) }).p1 };
if *__slate_slot_986 != (0 as i32) {
unsafe { sqlite3VdbeMemSetInt64(unsafe { (*__slate_slot_717).offset(*__slate_slot_986 as isize) }, (1 as i32) as i64) };
}
unsafe {
(*(*__slate_slot_987)).skipFlag = ((0 as i32) as i8) as u8;
}
}
unsafe { sqlite3VdbeMemRelease(unsafe { (*(*__slate_slot_987)).pOut }) };
unsafe {
(*unsafe { (*(*__slate_slot_987)).pOut }).flags = ((1 as i32) as i16) as u16;
}
unsafe {
(*(*__slate_slot_987)).isError = 0 as i32;
}
if *__slate_slot_710 != (0 as i32) {
break '__join_1182;
}
}
0 as i32;
0 as i32;
break '__join_26;
}
// jump, ncycle
0 as i32;
*__slate_slot_940 = unsafe { *unsafe { unsafe { (*p).apCsr }.offset((unsafe { (*(*__slate_slot_709)).p1 }) as isize) } };
0 as i32;
0 as i32;
0 as i32;
0 as i32;
0 as i32;
0 as i32;
(*__slate_slot_942).pKeyInfo = unsafe { (*(*__slate_slot_940)).pKeyInfo };
(*__slate_slot_942).nField = ((unsafe { (*(*__slate_slot_709)).p4.i }) as i16) as u16;
if (((unsafe { (*(*__slate_slot_709)).opcode }) as u32) as i32) < (45 as i32) {
0 as i32;
(*__slate_slot_942).default_rc = -(1 as i32) as i8;
} else {
0 as i32;
(*__slate_slot_942).default_rc = (0 as i32) as i8;
}
(*__slate_slot_942).aMem = unsafe { (*__slate_slot_717).offset((unsafe { (*(*__slate_slot_709)).p3 }) as isize) };
std::ptr::write(__slate_slot_943, (0 as i32) as i64);
0 as i32;
*__slate_slot_944 = unsafe { (*(*__slate_slot_940)).uc.pCursor };
0 as i32;
*__slate_slot_943 = ((unsafe { sqlite3BtreePayloadSize(*__slate_slot_944) }) as u64) as i64;
// nCellKey will always be between 0 and 0xffffffff because of the way
// that btreeParseCellPtr() and sqlite3GetVarint32() are implemented
if *__slate_slot_943 <= ((0 as i32) as i64) || *__slate_slot_943 > ((2147483647 as i32) as i64) {
break '__join_308;
} else {
unsafe { sqlite3VdbeMemInit(std::ptr::addr_of_mut!(*__slate_slot_945), *__slate_slot_711, ((0 as i32) as i16) as u16) };
*__slate_slot_710 = unsafe { sqlite3VdbeMemFromBtreeZeroOffset(*__slate_slot_944, (*__slate_slot_943 as i32) as u32, std::ptr::addr_of_mut!(*__slate_slot_945)) };
if *__slate_slot_710 != (0 as i32) {
break '__join_1193;
} else {
*__slate_slot_941 = unsafe { sqlite3VdbeRecordCompareWithSkip((*__slate_slot_945).n, (*__slate_slot_945).z as *const (), std::ptr::addr_of_mut!(*__slate_slot_942), 0 as i32) };
unsafe { sqlite3VdbeMemReleaseMalloc(std::ptr::addr_of_mut!(*__slate_slot_945)) };
// End of inlined sqlite3VdbeIdxKeyCompare()
0 as i32;
if (((unsafe { (*(*__slate_slot_709)).opcode }) as u32) as i32) & (1 as i32) == (45 as i32) & (1 as i32) {
0 as i32;
*__slate_slot_941 = -(*__slate_slot_941);
} else {
0 as i32;
std::ptr::write(__slate_slot_1836, *__slate_slot_941);
std::ptr::write(__slate_slot_1837, *__slate_slot_1836 + (1 as i32));
*__slate_slot_941 = *__slate_slot_1837;
}
{
}
0 as i32;
if *__slate_slot_941 > (0 as i32) {
break '__join_1125;
} else {
break '__join_26;
}
}
}
}
// out2, ncycle
0 as i32;
*__slate_slot_936 = unsafe { *unsafe { unsafe { (*p).apCsr }.offset((unsafe { (*(*__slate_slot_709)).p1 }) as isize) } };
0 as i32;
0 as i32;
0 as i32;
0 as i32;
0 as i32;
0 as i32;
// The IdxRowid and Seek opcodes are combined because of the commonality
// of sqlite3VdbeCursorRestore() and sqlite3VdbeIdxRowid().
*__slate_slot_710 = unsafe { sqlite3VdbeCursorRestore(*__slate_slot_936) };
// sqlite3VdbeCursorRestore() may fail if the cursor has been disturbed
// since it was last positioned and an error (e.g. OOM or an IO error)
// occurs while trying to reposition it.
if *__slate_slot_710 != (0 as i32) {
break '__join_1197;
} else {
if !((unsafe { (*(*__slate_slot_936)).nullRow }) != (0 as u8)) {
*__slate_slot_938 = (0 as i32) as i64; // Not needed.  Only used to silence a warning.
*__slate_slot_710 = unsafe { sqlite3VdbeIdxRowid(*__slate_slot_711, unsafe { (*(*__slate_slot_936)).uc.pCursor }, std::ptr::addr_of_mut!(*__slate_slot_938)) };
if *__slate_slot_710 != (0 as i32) {
break '__join_1196;
} else {
if (((unsafe { (*(*__slate_slot_709)).opcode }) as u32) as i32) == (143 as i32) {
0 as i32;
*__slate_slot_937 = unsafe { *unsafe { unsafe { (*p).apCsr }.offset((unsafe { (*(*__slate_slot_709)).p3 }) as isize) } };
0 as i32;
0 as i32;
0 as i32;
0 as i32;
unsafe {
(*(*__slate_slot_937)).nullRow = ((0 as i32) as i8) as u8;
}
unsafe {
(*(*__slate_slot_937)).movetoTarget = *__slate_slot_938;
}
unsafe {
(*(*__slate_slot_937)).deferredMoveto = ((1 as i32) as i8) as u8;
}
unsafe {
(*(*__slate_slot_937)).cacheStatus = (0 as i32) as u32;
}
0 as i32;
0 as i32;
unsafe {
(*(*__slate_slot_937)).ub.aAltMap = unsafe { (*(*__slate_slot_709)).p4.ai };
}
0 as i32;
unsafe {
(*(*__slate_slot_937)).pAltCursor = *__slate_slot_936;
}
break '__join_26;
} else {
*__slate_slot_721 = out2Prerelease(p, *__slate_slot_709);
unsafe {
(*(*__slate_slot_721)).u.i = *__slate_slot_938;
}
break '__join_26;
}
}
} else {
0 as i32;
unsafe { sqlite3VdbeMemSetNull(unsafe { (*__slate_slot_717).offset((unsafe { (*(*__slate_slot_709)).p2 }) as isize) }) };
break '__join_26;
}
}
}
unsafe {
(*(*__slate_slot_928)).cacheStatus = (0 as i32) as u32;
}
{
}
if *__slate_slot_710 == (0 as i32) {
unsafe {
(*(*__slate_slot_928)).nullRow = ((0 as i32) as i8) as u8;
}
std::ptr::write(__slate_slot_1827, unsafe { unsafe { (*p).aCounter.as_mut_ptr() as *mut u32 }.offset((((unsafe { (*(*__slate_slot_709)).p5 }) as u32) as i32) as isize) });
std::ptr::write(__slate_slot_1828, unsafe { *(*__slate_slot_1827) });
std::ptr::write(__slate_slot_1829, (*__slate_slot_1828).wrapping_add((1 as i32) as u32));
unsafe {
*(*__slate_slot_1827) = *__slate_slot_1829;
}
break '__join_1140;
} else {
if *__slate_slot_710 != (101 as i32) {
break '__join_1207;
} else {
*__slate_slot_710 = 0 as i32;
unsafe {
(*(*__slate_slot_928)).nullRow = ((1 as i32) as i8) as u8;
}
break '__join_1139;
}
}
}
// jump ncycle
std::ptr::write(__slate_slot_1824, unsafe { unsafe { (*p).aCounter.as_mut_ptr() as *mut u32 }.offset((2 as i32) as isize) });
std::ptr::write(__slate_slot_1825, unsafe { *(*__slate_slot_1824) });
std::ptr::write(__slate_slot_1826, (*__slate_slot_1825).wrapping_add((1 as i32) as u32));
unsafe {
*(*__slate_slot_1824) = *__slate_slot_1826;
}
// Fall through into OP_Rewind
//
// no break
{
}
// jump ncycle
// Opcode: Rewind P1 P2 * * *
//
// The next use of the Rowid or Column or Next instruction for P1
// will refer to the first entry in the database table or index.
// If the table or index is empty, jump immediately to P2.
// If the table or index is not empty, fall through to the following
// instruction.
//
// If P2 is zero, that is an assertion that the P1 table is never
// empty and hence the jump will never be taken.
//
// This opcode leaves the cursor configured to move in forward order,
// from the beginning toward the end.  In other words, the cursor is
// configured to use Next, not Prev.
}
// jump0, ncycle
0 as i32;
0 as i32;
0 as i32;
*__slate_slot_922 = unsafe { *unsafe { unsafe { (*p).apCsr }.offset((unsafe { (*(*__slate_slot_709)).p1 }) as isize) } };
0 as i32;
0 as i32;
*__slate_slot_924 = 1 as i32;
if (((unsafe { (*(*__slate_slot_922)).eCurType }) as u32) as i32) == (1 as i32) {
*__slate_slot_710 = unsafe { sqlite3VdbeSorterRewind(*__slate_slot_922 as *const VdbeCursor, std::ptr::addr_of_mut!(*__slate_slot_924)) };
} else {
0 as i32;
*__slate_slot_923 = unsafe { (*(*__slate_slot_922)).uc.pCursor };
0 as i32;
*__slate_slot_710 = unsafe { sqlite3BtreeFirst(*__slate_slot_923, std::ptr::addr_of_mut!(*__slate_slot_924)) };
unsafe {
(*(*__slate_slot_922)).deferredMoveto = ((0 as i32) as i8) as u8;
}
unsafe {
(*(*__slate_slot_922)).cacheStatus = (0 as i32) as u32;
}
}
if *__slate_slot_710 != (0 as i32) {
break '__join_1209;
} else {
unsafe {
(*(*__slate_slot_922)).nullRow = (*__slate_slot_924 as i8) as u8;
}
if (unsafe { (*(*__slate_slot_709)).p2 }) > (0 as i32) {
{
}
if *__slate_slot_924 != (0 as i32) {
break '__join_1125;
} else {
break '__join_26;
}
} else {
break '__join_26;
}
}
}
// jump0, ncycle
0 as i32;
*__slate_slot_915 = unsafe { *unsafe { unsafe { (*p).apCsr }.offset((unsafe { (*(*__slate_slot_709)).p1 }) as isize) } };
0 as i32;
0 as i32;
*__slate_slot_916 = unsafe { (*(*__slate_slot_915)).uc.pCursor };
*__slate_slot_917 = 0 as i32;
0 as i32;
if (((unsafe { (*(*__slate_slot_709)).opcode }) as u32) as i32) == (139 as i32) {
0 as i32;
unsafe {
(*(*__slate_slot_915)).seekResult = -(1 as i32);
}
if (unsafe { sqlite3BtreeCursorIsValidNN(*__slate_slot_916) }) != (0 as i32) {
break '__join_26;
}
}
*__slate_slot_710 = unsafe { sqlite3BtreeLast(*__slate_slot_916, std::ptr::addr_of_mut!(*__slate_slot_917)) };
unsafe {
(*(*__slate_slot_915)).nullRow = (*__slate_slot_917 as i8) as u8;
}
unsafe {
(*(*__slate_slot_915)).deferredMoveto = ((0 as i32) as i8) as u8;
}
unsafe {
(*(*__slate_slot_915)).cacheStatus = (0 as i32) as u32;
}
if *__slate_slot_710 != (0 as i32) {
break '__join_1211;
} else {
if (unsafe { (*(*__slate_slot_709)).p2 }) > (0 as i32) {
{
}
if *__slate_slot_917 != (0 as i32) {
break '__join_1125;
} else {
break '__join_26;
}
} else {
break '__join_26;
}
}
}
*__slate_slot_720 = unsafe { (*__slate_slot_717).offset((unsafe { (*(*__slate_slot_709)).p3 }) as isize) }; // jump, in3, ncycle
0 as i32;
0 as i32;
*__slate_slot_881 = unsafe { (*(*__slate_slot_720)).u.i };
}
*__slate_slot_878 = unsafe { *unsafe { unsafe { (*p).apCsr }.offset((unsafe { (*(*__slate_slot_709)).p1 }) as isize) } };
0 as i32;
0 as i32;
0 as i32;
*__slate_slot_879 = unsafe { (*(*__slate_slot_878)).uc.pCursor };
0 as i32;
*__slate_slot_880 = 0 as i32;
*__slate_slot_710 = unsafe { sqlite3BtreeTableMoveto(*__slate_slot_879, *__slate_slot_881, 0 as i32, std::ptr::addr_of_mut!(*__slate_slot_880)) };
0 as i32;
unsafe {
(*(*__slate_slot_878)).movetoTarget = *__slate_slot_881;
}
// Used by OP_Delete
unsafe {
(*(*__slate_slot_878)).nullRow = ((0 as i32) as i8) as u8;
}
unsafe {
(*(*__slate_slot_878)).cacheStatus = (0 as i32) as u32;
}
unsafe {
(*(*__slate_slot_878)).deferredMoveto = ((0 as i32) as i8) as u8;
}
{
}
unsafe {
(*(*__slate_slot_878)).seekResult = *__slate_slot_880;
}
if *__slate_slot_880 != (0 as i32) {
0 as i32;
if (unsafe { (*(*__slate_slot_709)).p2 }) == (0 as i32) {
*__slate_slot_710 = unsafe { sqlite3CorruptError(5700 as i32) };
} else {
break '__join_1125;
}
}
if *__slate_slot_710 != (0 as i32) {
break '__join_1224;
} else {
break '__join_26;
}
}
// jump, in3, ncycle
0 as i32;
0 as i32;
*__slate_slot_875 = unsafe { *unsafe { unsafe { (*p).apCsr }.offset((unsafe { (*(*__slate_slot_709)).p1 }) as isize) } };
0 as i32;
(*__slate_slot_877).aMem = unsafe { (*__slate_slot_717).offset((unsafe { (*(*__slate_slot_709)).p3 }) as isize) };
0 as i32;
0 as i32;
0 as i32;
(*__slate_slot_877).nField = ((unsafe { (*(*__slate_slot_709)).p4.i }) as i16) as u16;
if (((*__slate_slot_877).nField as u32) as i32) > (0 as i32) {
// Key values in an array of registers
(*__slate_slot_877).pKeyInfo = unsafe { (*(*__slate_slot_875)).pKeyInfo };
(*__slate_slot_877).default_rc = (0 as i32) as i8;
*__slate_slot_710 = unsafe { sqlite3BtreeIndexMoveto(unsafe { (*(*__slate_slot_875)).uc.pCursor }, std::ptr::addr_of_mut!(*__slate_slot_877), unsafe { std::ptr::addr_of_mut!((*(*__slate_slot_875)).seekResult) }) };
} else {
// Composite key generated by OP_MakeRecord
0 as i32;
0 as i32;
if (((unsafe { (*(*__slate_slot_877).aMem).flags }) as u32) as i32) & (1024 as i32) != (0 as i32) {
*__slate_slot_1795 = unsafe { sqlite3VdbeMemExpandBlob((*__slate_slot_877).aMem) };
} else {
*__slate_slot_1795 = 0 as i32;
}
*__slate_slot_710 = *__slate_slot_1795;
0 as i32;
if *__slate_slot_710 != (0 as i32) {
break '__join_1;
} else {
*__slate_slot_876 = unsafe { sqlite3VdbeAllocUnpackedRecord(unsafe { (*(*__slate_slot_875)).pKeyInfo }) };
if *__slate_slot_876 == std::ptr::null_mut::<UnpackedRecord>() {
break '__join_1;
} else {
unsafe { sqlite3VdbeRecordUnpack(unsafe { (*(*__slate_slot_877).aMem).n }, (unsafe { (*(*__slate_slot_877).aMem).z }) as *const (), *__slate_slot_876) };
unsafe {
(*(*__slate_slot_876)).default_rc = (0 as i32) as i8;
}
*__slate_slot_710 = unsafe { sqlite3BtreeIndexMoveto(unsafe { (*(*__slate_slot_875)).uc.pCursor }, *__slate_slot_876, unsafe { std::ptr::addr_of_mut!((*(*__slate_slot_875)).seekResult) }) };
unsafe { sqlite3DbFreeNN(*__slate_slot_711, *__slate_slot_876 as *mut ()) };
}
}
}
if *__slate_slot_710 != (0 as i32) {
break '__join_1225;
} else {
*__slate_slot_873 = ((unsafe { (*(*__slate_slot_875)).seekResult }) == (0 as i32)) as i32;
unsafe {
(*(*__slate_slot_875)).nullRow = (((1 as i32) - *__slate_slot_873) as i8) as u8;
}
unsafe {
(*(*__slate_slot_875)).deferredMoveto = ((0 as i32) as i8) as u8;
}
unsafe {
(*(*__slate_slot_875)).cacheStatus = (0 as i32) as u32;
}
if (((unsafe { (*(*__slate_slot_709)).opcode }) as u32) as i32) == (29 as i32) {
{
}
if *__slate_slot_873 != (0 as i32) {
break '__join_1125;
} else {
break '__join_26;
}
} else {
if !(*__slate_slot_873 != (0 as i32)) {
{
}
break '__join_1125;
} else {
'__join_473: {
if (((unsafe { (*(*__slate_slot_709)).opcode }) as u32) as i32) == (27 as i32) {
// For the OP_NoConflict opcode, take the jump if any of the
// input fields are NULL, since any key with a NULL will not
// conflict
*__slate_slot_874 = 0 as i32;
loop {
if *__slate_slot_874 < (((*__slate_slot_877).nField as u32) as i32) {
if (((unsafe { (*unsafe { (*__slate_slot_877).aMem.offset(*__slate_slot_874 as isize) }).flags }) as u32) as i32) & (1 as i32) != (0 as i32) {
break;
} else {
std::ptr::write(__slate_slot_1796, *__slate_slot_874);
std::ptr::write(__slate_slot_1797, *__slate_slot_1796 + (1 as i32));
*__slate_slot_874 = *__slate_slot_1797;
}
} else {
break '__join_473;
}
}
{
}
break '__join_1125;
}
}
{
}
if (((unsafe { (*(*__slate_slot_709)).opcode }) as u32) as i32) == (26 as i32) {
unsafe {
(*(*__slate_slot_875)).seekHit = ((unsafe { (*(*__slate_slot_709)).p4.i }) as i16) as u16;
}
break '__join_26;
} else {
break '__join_26;
}
}
}
}
}
'__join_514: {
// jump0, in3, group, ncycle
0 as i32;
0 as i32;
*__slate_slot_858 = unsafe { *unsafe { unsafe { (*p).apCsr }.offset((unsafe { (*(*__slate_slot_709)).p1 }) as isize) } };
0 as i32;
0 as i32;
0 as i32;
0 as i32;
0 as i32;
0 as i32;
0 as i32;
*__slate_slot_857 = ((unsafe { (*(*__slate_slot_709)).opcode }) as u32) as i32;
*__slate_slot_862 = 0 as i32;
unsafe {
(*(*__slate_slot_858)).nullRow = ((0 as i32) as i8) as u8;
}
unsafe {
(*(*__slate_slot_858)).deferredMoveto = ((0 as i32) as i8) as u8;
}
unsafe {
(*(*__slate_slot_858)).cacheStatus = (0 as i32) as u32;
}
if (unsafe { (*(*__slate_slot_858)).isTable }) != (0 as u8) {
// The OPFLAG_SEEKEQ/BTREE_SEEK_EQ flag is only set on index cursors
0 as i32;
// The input value in P3 might be of any type: integer, real, string,
// blob, or NULL.  But it needs to be an integer before we can do
// the seek, so convert it.
*__slate_slot_720 = unsafe { (*__slate_slot_717).offset((unsafe { (*(*__slate_slot_709)).p3 }) as isize) };
*__slate_slot_863 = unsafe { (*(*__slate_slot_720)).flags };
if ((*__slate_slot_863 as u32) as i32) & ((4 as i32) | (8 as i32) | (32 as i32) | (2 as i32)) == (2 as i32) {
applyNumericAffinity(*__slate_slot_720, 0 as i32);
}
*__slate_slot_861 = unsafe { sqlite3VdbeIntValue(*__slate_slot_720 as *const sqlite3_value) }; // Get the integer key value
*__slate_slot_864 = unsafe { (*(*__slate_slot_720)).flags }; // Record the type after applying numeric affinity
unsafe {
(*(*__slate_slot_720)).flags = *__slate_slot_863;
}
// But convert the type back to its original
// If the P3 value could not be converted into an integer without
// loss of information, then special processing is required...
if ((*__slate_slot_864 as u32) as i32) & ((4 as i32) | (32 as i32)) == (0 as i32) {
if ((*__slate_slot_864 as u32) as i32) & (8 as i32) == (0 as i32) {
if ((*__slate_slot_864 as u32) as i32) & (1 as i32) != (0 as i32) || *__slate_slot_857 >= (23 as i32) {
{
}
break '__join_1125;
} else {
*__slate_slot_710 = unsafe { sqlite3BtreeLast(unsafe { (*(*__slate_slot_858)).uc.pCursor }, std::ptr::addr_of_mut!(*__slate_slot_856)) };
if *__slate_slot_710 != (0 as i32) {
break '__join_1231;
} else {
break '__join_514;
}
}
} else {
*__slate_slot_865 = unsafe { sqlite3IntFloatCompare(*__slate_slot_861, unsafe { (*(*__slate_slot_720)).u.r }) };
// If the approximation iKey is larger than the actual real search
// term, substitute >= for > and < for <=. e.g. if the search term
// is 4.9 and the integer approximation 5:
//
//        (x >  4.9)    ->     (x >= 5)
//        (x <= 4.9)    ->     (x <  5)
if *__slate_slot_865 > (0 as i32) {
0 as i32;
0 as i32;
0 as i32;
if *__slate_slot_857 & (1 as i32) == (24 as i32) & (1 as i32) {
std::ptr::write(__slate_slot_1785, *__slate_slot_857);
std::ptr::write(__slate_slot_1786, *__slate_slot_1785 - (1 as i32));
*__slate_slot_857 = *__slate_slot_1786;
}
} else {
if *__slate_slot_865 < (0 as i32) {
0 as i32;
0 as i32;
0 as i32;
if *__slate_slot_857 & (1 as i32) == (21 as i32) & (1 as i32) {
std::ptr::write(__slate_slot_1787, *__slate_slot_857);
std::ptr::write(__slate_slot_1788, *__slate_slot_1787 + (1 as i32));
*__slate_slot_857 = *__slate_slot_1788;
}
}
}
// If the approximation iKey is smaller than the actual real search
// term, substitute <= for < and > for >=.
}
}
*__slate_slot_710 = unsafe { sqlite3BtreeTableMoveto(unsafe { (*(*__slate_slot_858)).uc.pCursor }, (*__slate_slot_861 as u64) as i64, 0 as i32, std::ptr::addr_of_mut!(*__slate_slot_856)) };
unsafe {
(*(*__slate_slot_858)).movetoTarget = *__slate_slot_861;
}
// Used by OP_Delete
if *__slate_slot_710 != (0 as i32) {
break '__join_1230;
}
} else {
// For a cursor with the OPFLAG_SEEKEQ/BTREE_SEEK_EQ hint, only the
// OP_SeekGE and OP_SeekLE opcodes are allowed, and these must be
// immediately followed by an OP_IdxGT or OP_IdxLT opcode, respectively,
// with the same key.
if (unsafe { sqlite3BtreeCursorHasHint(unsafe { (*(*__slate_slot_858)).uc.pCursor }, (2 as i32) as u32) }) != (0 as i32) {
*__slate_slot_862 = 1 as i32;
0 as i32;
0 as i32;
0 as i32;
0 as i32;
0 as i32;
0 as i32;
0 as i32;
0 as i32;
}
*__slate_slot_860 = unsafe { (*(*__slate_slot_709)).p4.i };
0 as i32;
0 as i32;
(*__slate_slot_859).pKeyInfo = unsafe { (*(*__slate_slot_858)).pKeyInfo };
(*__slate_slot_859).nField = (*__slate_slot_860 as i16) as u16;
// The next line of code computes as follows, only faster:
// if( oc==OP_SeekGT || oc==OP_SeekLE ){
//   r.default_rc = -1;
// }else{
//   r.default_rc = +1;
// }
(*__slate_slot_859).default_rc = (if (1 as i32) & *__slate_slot_857 - (21 as i32) != (0 as i32) { -(1 as i32) } else { 1 as i32 }) as i8;
0 as i32;
0 as i32;
0 as i32;
0 as i32;
(*__slate_slot_859).aMem = unsafe { (*__slate_slot_717).offset((unsafe { (*(*__slate_slot_709)).p3 }) as isize) };
(*__slate_slot_859).eqSeen = ((0 as i32) as i8) as u8;
*__slate_slot_710 = unsafe { sqlite3BtreeIndexMoveto(unsafe { (*(*__slate_slot_858)).uc.pCursor }, std::ptr::addr_of_mut!(*__slate_slot_859), std::ptr::addr_of_mut!(*__slate_slot_856)) };
if *__slate_slot_710 != (0 as i32) {
break '__join_1232;
} else {
if *__slate_slot_862 != (0 as i32) && (((*__slate_slot_859).eqSeen as u32) as i32) == (0 as i32) {
0 as i32;
break '__join_514;
}
}
}
if *__slate_slot_857 >= (23 as i32) {
0 as i32;
if *__slate_slot_856 < (0 as i32) || *__slate_slot_856 == (0 as i32) && *__slate_slot_857 == (24 as i32) {
*__slate_slot_856 = 0 as i32;
*__slate_slot_710 = unsafe { sqlite3BtreeNext(unsafe { (*(*__slate_slot_858)).uc.pCursor }, 0 as i32) };
if *__slate_slot_710 != (0 as i32) {
if *__slate_slot_710 == (101 as i32) {
*__slate_slot_710 = 0 as i32;
*__slate_slot_856 = 1 as i32;
} else {
break '__join_1228;
}
}
} else {
*__slate_slot_856 = 0 as i32;
}
} else {
0 as i32;
if *__slate_slot_856 > (0 as i32) || *__slate_slot_856 == (0 as i32) && *__slate_slot_857 == (21 as i32) {
*__slate_slot_856 = 0 as i32;
*__slate_slot_710 = unsafe { sqlite3BtreePrevious(unsafe { (*(*__slate_slot_858)).uc.pCursor }, 0 as i32) };
if *__slate_slot_710 != (0 as i32) {
if *__slate_slot_710 == (101 as i32) {
*__slate_slot_710 = 0 as i32;
*__slate_slot_856 = 1 as i32;
} else {
break '__join_1229;
}
}
} else {
// res might be negative because the table is empty.  Check to
// see if this is the case.
*__slate_slot_856 = unsafe { sqlite3BtreeEof(unsafe { (*(*__slate_slot_858)).uc.pCursor }) };
}
}
}
0 as i32;
{
}
if *__slate_slot_856 != (0 as i32) {
break '__join_1125;
} else {
if *__slate_slot_862 != (0 as i32) {
0 as i32;
std::ptr::write(__slate_slot_1789, *__slate_slot_709);
std::ptr::write(__slate_slot_1790, unsafe { (*__slate_slot_1789).offset((1 as i32) as isize) });
*__slate_slot_709 = *__slate_slot_1790; // Skip the OP_IdxLt or OP_IdxGT that follows
break '__join_26;
} else {
break '__join_26;
}
}
}
// ncycle
0 as i32;
0 as i32;
if (unsafe { (*(*__slate_slot_709)).p3 }) > (0 as i32) {
// Make register reg[P3] into a value that can be used as the data
// form sqlite3BtreeInsert() where the length of the data is zero.
0 as i32; // Only used when number of columns is zero
0 as i32;
0 as i32;
unsafe {
(*unsafe { (*__slate_slot_717).offset((unsafe { (*(*__slate_slot_709)).p3 }) as isize) }).n = 0 as i32;
}
unsafe {
(*unsafe { (*__slate_slot_717).offset((unsafe { (*(*__slate_slot_709)).p3 }) as isize) }).z = b"\0".as_ptr() as *mut i8;
}
}
*__slate_slot_850 = unsafe { *unsafe { unsafe { (*p).apCsr }.offset((unsafe { (*(*__slate_slot_709)).p1 }) as isize) } };
if *__slate_slot_850 != std::ptr::null_mut::<VdbeCursor>() && !(((unsafe { (*(*__slate_slot_850)).__slate_bits_0.__get_noReuse() }) as i32) != (0 as i32)) && (unsafe { (*(*__slate_slot_709)).p2 }) <= ((unsafe { (*(*__slate_slot_850)).nField }) as i32) {
// If the ephemeral table is already open and has no duplicates from
// OP_OpenDup, then erase all existing content so that the table is
// empty again, rather than creating a new table.
0 as i32;
unsafe {
(*(*__slate_slot_850)).seqCount = (0 as i32) as i64;
}
unsafe {
(*(*__slate_slot_850)).cacheStatus = (0 as i32) as u32;
}
*__slate_slot_710 = unsafe { sqlite3BtreeClearTable(unsafe { (*(*__slate_slot_850)).ub.pBtx }, (unsafe { (*(*__slate_slot_850)).pgnoRoot }) as i32, std::ptr::null_mut::<i64>()) };
} else {
*__slate_slot_850 = allocateCursor(p, unsafe { (*(*__slate_slot_709)).p1 }, unsafe { (*(*__slate_slot_709)).p2 }, ((0 as i32) as i8) as u8);
if *__slate_slot_850 == std::ptr::null_mut::<VdbeCursor>() {
break '__join_1;
} else {
unsafe {
(*(*__slate_slot_850)).__slate_bits_0.__set_isEphemeral((1 as i32) as u32);
}
*__slate_slot_710 = unsafe { sqlite3BtreeOpen(unsafe { (*(*__slate_slot_711)).pVfs }, std::ptr::null::<i8>(), *__slate_slot_711, unsafe { std::ptr::addr_of_mut!((*(*__slate_slot_850)).ub.pBtx) }, (1 as i32) | (4 as i32) | (((unsafe { (*(*__slate_slot_709)).p5 }) as u32) as i32), unsafe { vfsFlags }) };
if *__slate_slot_710 == (0 as i32) {
*__slate_slot_710 = unsafe { sqlite3BtreeBeginTrans(unsafe { (*(*__slate_slot_850)).ub.pBtx }, 1 as i32, std::ptr::null_mut::<i32>()) };
if *__slate_slot_710 == (0 as i32) {
std::ptr::write(__slate_slot_1780, unsafe { (*(*__slate_slot_709)).p4.pKeyInfo });
*__slate_slot_851 = *__slate_slot_1780;
std::ptr::write(__slate_slot_1781, *__slate_slot_1780);
unsafe {
(*(*__slate_slot_850)).pKeyInfo = *__slate_slot_1781;
}
if *__slate_slot_1781 != std::ptr::null_mut::<KeyInfo>() {
0 as i32;
*__slate_slot_710 = unsafe { sqlite3BtreeCreateTable(unsafe { (*(*__slate_slot_850)).ub.pBtx }, unsafe { std::ptr::addr_of_mut!((*(*__slate_slot_850)).pgnoRoot) }, (2 as i32) | (((unsafe { (*(*__slate_slot_709)).p5 }) as u32) as i32)) };
if *__slate_slot_710 == (0 as i32) {
0 as i32;
0 as i32;
0 as i32;
*__slate_slot_710 = unsafe { sqlite3BtreeCursor(unsafe { (*(*__slate_slot_850)).ub.pBtx }, unsafe { (*(*__slate_slot_850)).pgnoRoot }, 4 as i32, *__slate_slot_851, unsafe { (*(*__slate_slot_850)).uc.pCursor }) };
}
unsafe {
(*(*__slate_slot_850)).isTable = ((0 as i32) as i8) as u8;
}
} else {
unsafe {
(*(*__slate_slot_850)).pgnoRoot = (1 as i32) as u32;
}
*__slate_slot_710 = unsafe { sqlite3BtreeCursor(unsafe { (*(*__slate_slot_850)).ub.pBtx }, (1 as i32) as u32, 4 as i32, std::ptr::null_mut::<KeyInfo>(), unsafe { (*(*__slate_slot_850)).uc.pCursor }) };
unsafe {
(*(*__slate_slot_850)).isTable = ((1 as i32) as i8) as u8;
}
}
}
unsafe {
(*(*__slate_slot_850)).__slate_bits_0.__set_isOrdered(((((unsafe { (*(*__slate_slot_709)).p5 }) as u32) as i32) != (8 as i32)) as u32);
}
0 as i32;
if *__slate_slot_710 != (0 as i32) {
0 as i32;
unsafe { sqlite3BtreeClose(unsafe { (*(*__slate_slot_850)).ub.pBtx }) };
unsafe {
*unsafe { unsafe { (*p).apCsr }.offset((unsafe { (*(*__slate_slot_709)).p1 }) as isize) } = std::ptr::null_mut::<VdbeCursor>();
}
// Not required; helps with static analysis
} else {
0 as i32;
}
}
}
}
if *__slate_slot_710 != (0 as i32) {
break '__join_1234;
} else {
unsafe {
(*(*__slate_slot_850)).nullRow = ((1 as i32) as i8) as u8;
}
break '__join_26;
}
}
0 as i32; // ncycle
0 as i32;
0 as i32;
if ((unsafe { (*p).__slate_bits_0.__get_expired() }) as i32) == (1 as i32) {
break '__join_585;
} else {
*__slate_slot_840 = 0 as i32;
*__slate_slot_841 = std::ptr::null_mut::<KeyInfo>();
*__slate_slot_842 = (unsafe { (*(*__slate_slot_709)).p2 }) as u32;
*__slate_slot_843 = unsafe { (*(*__slate_slot_709)).p3 };
0 as i32;
0 as i32;
*__slate_slot_847 = unsafe { unsafe { (*(*__slate_slot_711)).aDb }.offset(*__slate_slot_843 as isize) };
*__slate_slot_845 = unsafe { (*(*__slate_slot_847)).pBt };
0 as i32;
if (((unsafe { (*(*__slate_slot_709)).opcode }) as u32) as i32) == (116 as i32) {
0 as i32;
*__slate_slot_844 = (4 as i32) | (((unsafe { (*(*__slate_slot_709)).p5 }) as u32) as i32) & (8 as i32);
0 as i32;
if (((unsafe { (*unsafe { (*(*__slate_slot_847)).pSchema }).file_format }) as u32) as i32) < (((unsafe { (*p).minWriteFileFormat }) as u32) as i32) {
unsafe {
(*p).minWriteFileFormat = unsafe { (*unsafe { (*(*__slate_slot_847)).pSchema }).file_format };
}
}
if (((unsafe { (*(*__slate_slot_709)).p5 }) as u32) as i32) & (16 as i32) != (0 as i32) {
0 as i32;
0 as i32;
*__slate_slot_719 = unsafe { (*__slate_slot_717).offset(*__slate_slot_842 as isize) };
0 as i32;
0 as i32;
unsafe { sqlite3VdbeMemIntegerify(*__slate_slot_719) };
*__slate_slot_842 = ((unsafe { (*(*__slate_slot_719)).u.i }) as i32) as u32;
// The p2 value always comes from a prior OP_CreateBtree opcode and
// that opcode will always set the p2 value to 2 or more or else fail.
// If there were a failure, the prepared statement would have halted
// before reaching this instruction.
0 as i32;
}
} else {
*__slate_slot_844 = 0 as i32;
0 as i32;
}
if ((unsafe { (*(*__slate_slot_709)).p4type }) as i32) == -(9 as i32) {
*__slate_slot_841 = unsafe { (*(*__slate_slot_709)).p4.pKeyInfo };
0 as i32;
0 as i32;
*__slate_slot_840 = ((unsafe { (*(*__slate_slot_841)).nAllField }) as u32) as i32;
} else {
if ((unsafe { (*(*__slate_slot_709)).p4type }) as i32) == -(3 as i32) {
*__slate_slot_840 = unsafe { (*(*__slate_slot_709)).p4.i };
}
}
0 as i32;
0 as i32;
{
}
// Table with INTEGER PRIMARY KEY and nothing else
*__slate_slot_846 = allocateCursor(p, unsafe { (*(*__slate_slot_709)).p1 }, *__slate_slot_840, ((0 as i32) as i8) as u8);
if *__slate_slot_846 == std::ptr::null_mut::<VdbeCursor>() {
break '__join_1;
} else {
unsafe {
(*(*__slate_slot_846)).iDb = *__slate_slot_843 as i8;
}
unsafe {
(*(*__slate_slot_846)).nullRow = ((1 as i32) as i8) as u8;
}
unsafe {
(*(*__slate_slot_846)).__slate_bits_0.__set_isOrdered((1 as i32) as u32);
}
unsafe {
(*(*__slate_slot_846)).pgnoRoot = *__slate_slot_842;
}
*__slate_slot_710 = unsafe { sqlite3BtreeCursor(*__slate_slot_845, *__slate_slot_842, *__slate_slot_844, *__slate_slot_841, unsafe { (*(*__slate_slot_846)).uc.pCursor }) };
unsafe {
(*(*__slate_slot_846)).pKeyInfo = *__slate_slot_841;
}
// Set the VdbeCursor.isTable variable. Previous versions of
// SQLite used to check if the root-page flags were sane at this point
// and report database corruption if they were not, but this check has
// since moved into the btree layer.
unsafe {
(*(*__slate_slot_846)).isTable = (((unsafe { (*(*__slate_slot_709)).p4type }) as i32) != -(9 as i32)) as u8;
}
}
}
}
0 as i32;
0 as i32;
{
}
{
}
unsafe { sqlite3BtreeCursorHintFlags(unsafe { (*(*__slate_slot_846)).uc.pCursor }, ((((unsafe { (*(*__slate_slot_709)).p5 }) as u32) as i32) & ((1 as i32) | (2 as i32))) as u32) };
if *__slate_slot_710 != (0 as i32) {
break '__join_1235;
} else {
break '__join_26;
}
}
// same as TK_OR, in1, in2, out3
*__slate_slot_765 = unsafe { sqlite3VdbeBooleanValue(unsafe { (*__slate_slot_717).offset((unsafe { (*(*__slate_slot_709)).p1 }) as isize) }, 2 as i32) };
*__slate_slot_766 = unsafe { sqlite3VdbeBooleanValue(unsafe { (*__slate_slot_717).offset((unsafe { (*(*__slate_slot_709)).p2 }) as isize) }, 2 as i32) };
if (((unsafe { (*(*__slate_slot_709)).opcode }) as u32) as i32) == (44 as i32) {
*__slate_slot_765 = ((unsafe { *unsafe { unsafe { std::ptr::addr_of!(and_logic) as *const u8 }.offset((*__slate_slot_765 * (3 as i32) + *__slate_slot_766) as isize) } }) as u32) as i32;
} else {
*__slate_slot_765 = ((unsafe { *unsafe { unsafe { std::ptr::addr_of!(or_logic) as *const u8 }.offset((*__slate_slot_765 * (3 as i32) + *__slate_slot_766) as isize) } }) as u32) as i32;
}
*__slate_slot_721 = unsafe { (*__slate_slot_717).offset((unsafe { (*(*__slate_slot_709)).p3 }) as isize) };
if *__slate_slot_765 == (2 as i32) {
unsafe {
(*(*__slate_slot_721)).flags = (((((unsafe { (*(*__slate_slot_721)).flags }) as u32) as i32) & !((3519 as i32) | (1024 as i32)) | (1 as i32)) as i16) as u16;
}
break '__join_26;
} else {
unsafe {
(*(*__slate_slot_721)).u.i = *__slate_slot_765 as i64;
}
unsafe {
(*(*__slate_slot_721)).flags = (((((unsafe { (*(*__slate_slot_721)).flags }) as u32) as i32) & !((3519 as i32) | (1024 as i32)) | (4 as i32)) as i16) as u16;
}
break '__join_26;
}
}
// same as TK_GE, jump, in1, in3
*__slate_slot_718 = unsafe { (*__slate_slot_717).offset((unsafe { (*(*__slate_slot_709)).p1 }) as isize) };
*__slate_slot_720 = unsafe { (*__slate_slot_717).offset((unsafe { (*(*__slate_slot_709)).p3 }) as isize) };
*__slate_slot_754 = unsafe { (*(*__slate_slot_718)).flags };
*__slate_slot_755 = unsafe { (*(*__slate_slot_720)).flags };
if ((*__slate_slot_754 as u32) as i32) & ((*__slate_slot_755 as u32) as i32) & (4 as i32) != (0 as i32) {
// Common case of comparison of two integers
if (unsafe { (*(*__slate_slot_720)).u.i }) > unsafe { (*(*__slate_slot_718)).u.i } {
if (unsafe { *unsafe { unsafe { sqlite3aGTb }.offset((((unsafe { (*(*__slate_slot_709)).opcode }) as u32) as i32) as isize) } }) != (0 as u8) {
{
}
break '__join_1125;
} else {
*__slate_slot_714 = 1 as i32;
}
} else {
if (unsafe { (*(*__slate_slot_720)).u.i }) < unsafe { (*(*__slate_slot_718)).u.i } {
if (unsafe { *unsafe { unsafe { sqlite3aLTb }.offset((((unsafe { (*(*__slate_slot_709)).opcode }) as u32) as i32) as isize) } }) != (0 as u8) {
{
}
break '__join_1125;
} else {
*__slate_slot_714 = -(1 as i32);
}
} else {
if (unsafe { *unsafe { unsafe { sqlite3aEQb }.offset((((unsafe { (*(*__slate_slot_709)).opcode }) as u32) as i32) as isize) } }) != (0 as u8) {
{
}
break '__join_1125;
} else {
*__slate_slot_714 = 0 as i32;
}
}
}
{
}
break '__join_26;
} else {
if (((*__slate_slot_754 as u32) as i32) | ((*__slate_slot_755 as u32) as i32)) & (1 as i32) != (0 as i32) {
// One or both operands are NULL
if (((unsafe { (*(*__slate_slot_709)).p5 }) as u32) as i32) & (128 as i32) != (0 as i32) {
// If SQLITE_NULLEQ is set (which will only happen if the operator is
// OP_Eq or OP_Ne) then take the jump or not depending on whether
// or not both operands are null.
0 as i32;
0 as i32;
{
}
if ((*__slate_slot_754 as u32) as i32) & ((*__slate_slot_755 as u32) as i32) & (1 as i32) != (0 as i32) && ((*__slate_slot_755 as u32) as i32) & (256 as i32) == (0 as i32) {
*__slate_slot_751 = 0 as i32; // Operands are equal
} else {
*__slate_slot_751 = if ((*__slate_slot_755 as u32) as i32) & (1 as i32) != (0 as i32) { -(1 as i32) } else { 1 as i32 }; // Operands are not equal
}
} else {
// SQLITE_NULLEQ is clear and at least one operand is NULL,
// then the result is always NULL.
// The jump is taken if the SQLITE_JUMPIFNULL bit is set.
{
}
if (((unsafe { (*(*__slate_slot_709)).p5 }) as u32) as i32) & (16 as i32) != (0 as i32) {
break '__join_1125;
} else {
*__slate_slot_714 = 1 as i32; // Operands are not equal
break '__join_26;
}
}
} else {
// Neither operand is NULL and we couldn't do the special high-speed
// integer comparison case.  So do a general-case comparison.
*__slate_slot_753 = ((((unsafe { (*(*__slate_slot_709)).p5 }) as u32) as i32) & (71 as i32)) as i8;
if (*__slate_slot_753 as i32) >= (67 as i32) {
if (((*__slate_slot_754 as u32) as i32) | ((*__slate_slot_755 as u32) as i32)) & (2 as i32) != (0 as i32) {
if ((*__slate_slot_754 as u32) as i32) & ((4 as i32) | (32 as i32) | (8 as i32) | (2 as i32)) == (2 as i32) {
applyNumericAffinity(*__slate_slot_718, 0 as i32);
0 as i32;
*__slate_slot_755 = unsafe { (*(*__slate_slot_720)).flags };
}
if ((*__slate_slot_755 as u32) as i32) & ((4 as i32) | (32 as i32) | (8 as i32) | (2 as i32)) == (2 as i32) {
applyNumericAffinity(*__slate_slot_720, 0 as i32);
}
}
} else {
if (*__slate_slot_753 as i32) == (66 as i32) && (((*__slate_slot_754 as u32) as i32) | ((*__slate_slot_755 as u32) as i32)) & (2 as i32) != (0 as i32) {
if ((*__slate_slot_754 as u32) as i32) & (2 as i32) != (0 as i32) {
std::ptr::write(__slate_slot_1618, *__slate_slot_718);
std::ptr::write(__slate_slot_1619, unsafe { (*(*__slate_slot_1618)).flags });
std::ptr::write(__slate_slot_1620, ((((*__slate_slot_1619 as u32) as i32) & !((4 as i32) | (8 as i32) | (32 as i32))) as i16) as u16);
unsafe {
(*(*__slate_slot_1618)).flags = *__slate_slot_1620;
}
} else {
if ((*__slate_slot_754 as u32) as i32) & ((4 as i32) | (8 as i32) | (32 as i32)) != (0 as i32) {
{
}
{
}
{
}
unsafe { sqlite3VdbeMemStringify(*__slate_slot_718, *__slate_slot_713, ((1 as i32) as i8) as u8) };
{
}
*__slate_slot_754 = (((((unsafe { (*(*__slate_slot_718)).flags }) as u32) as i32) & !(3519 as i32) | ((*__slate_slot_754 as u32) as i32) & (3519 as i32)) as i16) as u16;
if *__slate_slot_718 == *__slate_slot_720 {
*__slate_slot_755 = ((((*__slate_slot_754 as u32) as i32) | (2 as i32)) as i16) as u16;
}
}
}
if ((*__slate_slot_755 as u32) as i32) & (2 as i32) != (0 as i32) {
std::ptr::write(__slate_slot_1621, *__slate_slot_720);
std::ptr::write(__slate_slot_1622, unsafe { (*(*__slate_slot_1621)).flags });
std::ptr::write(__slate_slot_1623, ((((*__slate_slot_1622 as u32) as i32) & !((4 as i32) | (8 as i32) | (32 as i32))) as i16) as u16);
unsafe {
(*(*__slate_slot_1621)).flags = *__slate_slot_1623;
}
} else {
if ((*__slate_slot_755 as u32) as i32) & ((4 as i32) | (8 as i32) | (32 as i32)) != (0 as i32) {
{
}
{
}
{
}
unsafe { sqlite3VdbeMemStringify(*__slate_slot_720, *__slate_slot_713, ((1 as i32) as i8) as u8) };
{
}
*__slate_slot_755 = (((((unsafe { (*(*__slate_slot_720)).flags }) as u32) as i32) & !(3519 as i32) | ((*__slate_slot_755 as u32) as i32) & (3519 as i32)) as i16) as u16;
}
}
}
}
0 as i32;
*__slate_slot_751 = unsafe { sqlite3MemCompare(*__slate_slot_720 as *const sqlite3_value, *__slate_slot_718 as *const sqlite3_value, (unsafe { (*(*__slate_slot_709)).p4.pColl }) as *const CollSeq) };
}
// At this point, res is negative, zero, or positive if reg[P1] is
// less than, equal to, or greater than reg[P3], respectively.  Compute
// the answer to this operator in res2, depending on what the comparison
// operator actually is.  The next block of code depends on the fact
// that the 6 comparison operators are consecutive integers in this
// order:  NE, EQ, GT, LE, LT, GE
0 as i32;
0 as i32;
0 as i32;
0 as i32;
0 as i32;
if *__slate_slot_751 < (0 as i32) {
*__slate_slot_752 = ((unsafe { *unsafe { unsafe { sqlite3aLTb }.offset((((unsafe { (*(*__slate_slot_709)).opcode }) as u32) as i32) as isize) } }) as u32) as i32;
} else {
if *__slate_slot_751 == (0 as i32) {
*__slate_slot_752 = ((unsafe { *unsafe { unsafe { sqlite3aEQb }.offset((((unsafe { (*(*__slate_slot_709)).opcode }) as u32) as i32) as isize) } }) as u32) as i32;
} else {
*__slate_slot_752 = ((unsafe { *unsafe { unsafe { sqlite3aGTb }.offset((((unsafe { (*(*__slate_slot_709)).opcode }) as u32) as i32) as isize) } }) as u32) as i32;
}
}
*__slate_slot_714 = *__slate_slot_751;
// Undo any changes made by applyAffinity() to the input registers.
0 as i32;
unsafe {
(*(*__slate_slot_720)).flags = *__slate_slot_755;
}
0 as i32;
unsafe {
(*(*__slate_slot_718)).flags = *__slate_slot_754;
}
{
}
if *__slate_slot_752 != (0 as i32) {
break '__join_1125;
} else {
break '__join_26;
}
}
}
// same as TK_RSHIFT, in1, in2, out3
*__slate_slot_718 = unsafe { (*__slate_slot_717).offset((unsafe { (*(*__slate_slot_709)).p1 }) as isize) };
*__slate_slot_719 = unsafe { (*__slate_slot_717).offset((unsafe { (*(*__slate_slot_709)).p2 }) as isize) };
*__slate_slot_721 = unsafe { (*__slate_slot_717).offset((unsafe { (*(*__slate_slot_709)).p3 }) as isize) };
if ((((unsafe { (*(*__slate_slot_718)).flags }) as u32) as i32) | (((unsafe { (*(*__slate_slot_719)).flags }) as u32) as i32)) & (1 as i32) != (0 as i32) {
unsafe { sqlite3VdbeMemSetNull(*__slate_slot_721) };
break '__join_26;
} else {
*__slate_slot_747 = unsafe { sqlite3VdbeIntValue(*__slate_slot_719 as *const sqlite3_value) };
*__slate_slot_749 = unsafe { sqlite3VdbeIntValue(*__slate_slot_718 as *const sqlite3_value) };
*__slate_slot_750 = unsafe { (*(*__slate_slot_709)).opcode };
if ((*__slate_slot_750 as u32) as i32) == (103 as i32) {
std::ptr::write(__slate_slot_1604, *__slate_slot_747);
std::ptr::write(__slate_slot_1605, *__slate_slot_1604 & *__slate_slot_749);
*__slate_slot_747 = *__slate_slot_1605;
} else {
if ((*__slate_slot_750 as u32) as i32) == (104 as i32) {
std::ptr::write(__slate_slot_1606, *__slate_slot_747);
std::ptr::write(__slate_slot_1607, *__slate_slot_1606 | *__slate_slot_749);
*__slate_slot_747 = *__slate_slot_1607;
} else {
if *__slate_slot_749 != ((0 as i32) as i64) {
0 as i32;
// If shifting by a negative amount, shift in the other direction
if *__slate_slot_749 < ((0 as i32) as i64) {
0 as i32;
*__slate_slot_750 = (((2 as i32) * (105 as i32) + (1 as i32) - ((*__slate_slot_750 as u32) as i32)) as i8) as u8;
*__slate_slot_749 = if *__slate_slot_749 > (-(64 as i32) as i64) { -(*__slate_slot_749) } else { (64 as i32) as i64 };
}
if *__slate_slot_749 >= ((64 as i32) as i64) {
*__slate_slot_747 = (if *__slate_slot_747 >= ((0 as i32) as i64) || ((*__slate_slot_750 as u32) as i32) == (105 as i32) { 0 as i32 } else { -(1 as i32) }) as i64;
} else {
unsafe { memcpy(std::ptr::addr_of_mut!(*__slate_slot_748) as *mut (), std::ptr::addr_of_mut!(*__slate_slot_747) as *const (), 8 as u64) };
if ((*__slate_slot_750 as u32) as i32) == (105 as i32) {
std::ptr::write(__slate_slot_1608, *__slate_slot_748);
std::ptr::write(__slate_slot_1609, *__slate_slot_1608 << *__slate_slot_749);
*__slate_slot_748 = *__slate_slot_1609;
} else {
std::ptr::write(__slate_slot_1610, *__slate_slot_748);
std::ptr::write(__slate_slot_1611, *__slate_slot_1610 >> *__slate_slot_749);
*__slate_slot_748 = *__slate_slot_1611;
// Sign-extend on a right shift of a negative number
if *__slate_slot_747 < ((0 as i32) as i64) {
std::ptr::write(__slate_slot_1612, *__slate_slot_748);
std::ptr::write(__slate_slot_1613, *__slate_slot_1612 | (((4294967295 as u32) as u64) << (32 as i32) | ((4294967295 as u32) as u64)) << ((64 as i32) as i64) - *__slate_slot_749);
*__slate_slot_748 = *__slate_slot_1613;
}
}
unsafe { memcpy(std::ptr::addr_of_mut!(*__slate_slot_747) as *mut (), std::ptr::addr_of_mut!(*__slate_slot_748) as *const (), 8 as u64) };
}
}
}
}
unsafe {
(*(*__slate_slot_721)).u.i = *__slate_slot_747;
}
unsafe {
(*(*__slate_slot_721)).flags = (((((unsafe { (*(*__slate_slot_721)).flags }) as u32) as i32) & !((3519 as i32) | (1024 as i32)) | (4 as i32)) as i16) as u16;
}
break '__join_26;
}
}
'__join_1008: {
'__join_1032: {
// same as TK_REM, in1, in2, out3
*__slate_slot_718 = unsafe { (*__slate_slot_717).offset((unsafe { (*(*__slate_slot_709)).p1 }) as isize) };
*__slate_slot_741 = unsafe { (*(*__slate_slot_718)).flags };
*__slate_slot_719 = unsafe { (*__slate_slot_717).offset((unsafe { (*(*__slate_slot_709)).p2 }) as isize) };
*__slate_slot_742 = unsafe { (*(*__slate_slot_719)).flags };
*__slate_slot_721 = unsafe { (*__slate_slot_717).offset((unsafe { (*(*__slate_slot_709)).p3 }) as isize) };
if ((*__slate_slot_741 as u32) as i32) & ((*__slate_slot_742 as u32) as i32) & (4 as i32) != (0 as i32) {
} else {
if (((*__slate_slot_741 as u32) as i32) | ((*__slate_slot_742 as u32) as i32)) & (1 as i32) != (0 as i32) {
break '__join_1008;
} else {
*__slate_slot_741 = numericType(*__slate_slot_718);
*__slate_slot_742 = numericType(*__slate_slot_719);
if ((*__slate_slot_741 as u32) as i32) & ((*__slate_slot_742 as u32) as i32) & (4 as i32) != (0 as i32) {
} else {
break '__join_1032;
}
}
}
*__slate_slot_743 = unsafe { (*(*__slate_slot_718)).u.i };
*__slate_slot_744 = unsafe { (*(*__slate_slot_719)).u.i };
let __t2: i32 = ((unsafe { (*(*__slate_slot_709)).opcode }) as u32) as i32;
if __t2 == (107 as i32) {
if (unsafe { sqlite3AddInt64(std::ptr::addr_of_mut!(*__slate_slot_744), *__slate_slot_743) }) != (0 as i32) {
break '__join_1032;
}
} else {
if __t2 == (108 as i32) {
if (unsafe { sqlite3SubInt64(std::ptr::addr_of_mut!(*__slate_slot_744), *__slate_slot_743) }) != (0 as i32) {
break '__join_1032;
}
} else {
if __t2 == (109 as i32) {
if (unsafe { sqlite3MulInt64(std::ptr::addr_of_mut!(*__slate_slot_744), *__slate_slot_743) }) != (0 as i32) {
break '__join_1032;
}
} else {
if __t2 == (110 as i32) {
if *__slate_slot_743 == ((0 as i32) as i64) {
break '__join_1008;
} else {
if *__slate_slot_743 == (-(1 as i32) as i64) && *__slate_slot_744 == (-(1 as i32) as i64) - ((((4294967295 as u32) as u64) as i64) | ((2147483647 as i32) as i64) << (32 as i32)) {
break '__join_1032;
} else {
std::ptr::write(__slate_slot_1592, *__slate_slot_744);
std::ptr::write(__slate_slot_1593, *__slate_slot_1592 / *__slate_slot_743);
*__slate_slot_744 = *__slate_slot_1593;
}
}
} else {
if *__slate_slot_743 == ((0 as i32) as i64) {
break '__join_1008;
} else {
if *__slate_slot_743 == (-(1 as i32) as i64) {
*__slate_slot_743 = (1 as i32) as i64;
}
std::ptr::write(__slate_slot_1594, *__slate_slot_744);
std::ptr::write(__slate_slot_1595, *__slate_slot_1594 % *__slate_slot_743);
*__slate_slot_744 = *__slate_slot_1595;
}
}
}
}
}
unsafe {
(*(*__slate_slot_721)).u.i = *__slate_slot_744;
}
unsafe {
(*(*__slate_slot_721)).flags = (((((unsafe { (*(*__slate_slot_721)).flags }) as u32) as i32) & !((3519 as i32) | (1024 as i32)) | (4 as i32)) as i16) as u16;
}
break '__join_26;
}
*__slate_slot_745 = unsafe { sqlite3VdbeRealValue(*__slate_slot_718) };
*__slate_slot_746 = unsafe { sqlite3VdbeRealValue(*__slate_slot_719) };
let __t3: i32 = ((unsafe { (*(*__slate_slot_709)).opcode }) as u32) as i32;
if __t3 == (107 as i32) {
std::ptr::write(__slate_slot_1596, *__slate_slot_746);
std::ptr::write(__slate_slot_1597, *__slate_slot_1596 + *__slate_slot_745);
*__slate_slot_746 = *__slate_slot_1597;
} else {
if __t3 == (108 as i32) {
std::ptr::write(__slate_slot_1598, *__slate_slot_746);
std::ptr::write(__slate_slot_1599, *__slate_slot_1598 - *__slate_slot_745);
*__slate_slot_746 = *__slate_slot_1599;
} else {
if __t3 == (109 as i32) {
std::ptr::write(__slate_slot_1600, *__slate_slot_746);
std::ptr::write(__slate_slot_1601, *__slate_slot_1600 * *__slate_slot_745);
*__slate_slot_746 = *__slate_slot_1601;
} else {
if __t3 == (110 as i32) {
// (double)0 In case of SQLITE_OMIT_FLOATING_POINT...
if *__slate_slot_745 == ((0 as i32) as f64) {
break '__join_1008;
} else {
std::ptr::write(__slate_slot_1602, *__slate_slot_746);
std::ptr::write(__slate_slot_1603, *__slate_slot_1602 / *__slate_slot_745);
*__slate_slot_746 = *__slate_slot_1603;
}
} else {
*__slate_slot_743 = unsafe { sqlite3VdbeIntValue(*__slate_slot_718 as *const sqlite3_value) };
*__slate_slot_744 = unsafe { sqlite3VdbeIntValue(*__slate_slot_719 as *const sqlite3_value) };
if *__slate_slot_743 == ((0 as i32) as i64) {
break '__join_1008;
} else {
if *__slate_slot_743 == (-(1 as i32) as i64) {
*__slate_slot_743 = (1 as i32) as i64;
}
*__slate_slot_746 = (*__slate_slot_744 % *__slate_slot_743) as f64;
}
}
}
}
}
if (unsafe { sqlite3IsNaN(*__slate_slot_746) }) != (0 as i32) {
} else {
unsafe {
(*(*__slate_slot_721)).u.r = *__slate_slot_746;
}
unsafe {
(*(*__slate_slot_721)).flags = (((((unsafe { (*(*__slate_slot_721)).flags }) as u32) as i32) & !((3519 as i32) | (1024 as i32)) | (8 as i32)) as i16) as u16;
}
break '__join_26;
}
}
unsafe { sqlite3VdbeMemSetNull(*__slate_slot_721) };
break '__join_26;
}
// out2
*__slate_slot_721 = out2Prerelease(p, *__slate_slot_709);
*__slate_slot_731 = (unsafe { (*(*__slate_slot_709)).p3 }) - unsafe { (*(*__slate_slot_709)).p2 };
0 as i32;
std::ptr::write(__slate_slot_1556, ((if (unsafe { (*(*__slate_slot_709)).p1 }) != (0 as i32) { (1 as i32) | (256 as i32) } else { 1 as i32 }) as i16) as u16);
*__slate_slot_732 = *__slate_slot_1556;
unsafe {
(*(*__slate_slot_721)).flags = *__slate_slot_1556;
}
unsafe {
(*(*__slate_slot_721)).n = 0 as i32;
}
loop {
if *__slate_slot_731 > (0 as i32) {
std::ptr::write(__slate_slot_1557, *__slate_slot_721);
std::ptr::write(__slate_slot_1558, unsafe { (*__slate_slot_1557).offset((1 as i32) as isize) });
*__slate_slot_721 = *__slate_slot_1558;
{
}
unsafe { sqlite3VdbeMemSetNull(*__slate_slot_721) };
unsafe {
(*(*__slate_slot_721)).flags = *__slate_slot_732;
}
unsafe {
(*(*__slate_slot_721)).n = 0 as i32;
}
std::ptr::write(__slate_slot_1559, *__slate_slot_731);
std::ptr::write(__slate_slot_1560, *__slate_slot_1559 - (1 as i32));
*__slate_slot_731 = *__slate_slot_1560;
} else {
break '__join_26;
}
}
}
// out2
0 as i32;
*__slate_slot_721 = out2Prerelease(p, *__slate_slot_709);
unsafe {
(*(*__slate_slot_721)).flags = (((2 as i32) | (8192 as i32) | (512 as i32)) as i16) as u16;
}
unsafe {
(*(*__slate_slot_721)).z = unsafe { (*(*__slate_slot_709)).p4.z };
}
unsafe {
(*(*__slate_slot_721)).n = unsafe { (*(*__slate_slot_709)).p1 };
}
unsafe {
(*(*__slate_slot_721)).enc = *__slate_slot_713;
}
{
}
if (unsafe { (*(*__slate_slot_709)).p3 }) > (0 as i32) {
0 as i32;
*__slate_slot_720 = unsafe { (*__slate_slot_717).offset((unsafe { (*(*__slate_slot_709)).p3 }) as isize) };
0 as i32;
if (unsafe { (*(*__slate_slot_720)).u.i }) == ((((unsafe { (*(*__slate_slot_709)).p5 }) as u32) as i32) as i64) {
unsafe {
(*(*__slate_slot_721)).flags = (((16 as i32) | (8192 as i32) | (512 as i32)) as i16) as u16;
}
break '__join_26;
} else {
break '__join_26;
}
} else {
break '__join_26;
}
}
0 as i32;
// A deliberately coded "OP_Halt SQLITE_INTERNAL * * * *" opcode indicates
// something is wrong with the code generator.  Raise an assertion in order
// to bring this to the attention of fuzzers and other testing tools.
0 as i32;
if (unsafe { (*p).pFrame }) != std::ptr::null_mut::<VdbeFrame>() && (unsafe { (*(*__slate_slot_709)).p1 }) == (0 as i32) {
// Halt the sub-program. Return control to the parent frame.
*__slate_slot_726 = unsafe { (*p).pFrame };
unsafe {
(*p).pFrame = unsafe { (*(*__slate_slot_726)).pParent };
}
std::ptr::write(__slate_slot_1550, p);
std::ptr::write(__slate_slot_1551, unsafe { (*(*__slate_slot_1550)).nFrame });
std::ptr::write(__slate_slot_1552, *__slate_slot_1551 - (1 as i32));
unsafe {
(*(*__slate_slot_1550)).nFrame = *__slate_slot_1552;
}
unsafe { sqlite3VdbeSetChanges(*__slate_slot_711, unsafe { (*p).nChange }) };
*__slate_slot_727 = unsafe { sqlite3VdbeFrameRestore(*__slate_slot_726) };
if (unsafe { (*(*__slate_slot_709)).p2 }) == (4 as i32) {
// Instruction pcx is the OP_Program that invoked the sub-program
// currently being halted. If the p2 instruction of this OP_Halt
// instruction is set to OE_Ignore, then the sub-program is throwing
// an IGNORE exception. In this case jump to the address specified
// as the p2 of the calling OP_Program.
*__slate_slot_727 = (unsafe { (*unsafe { unsafe { (*p).aOp }.offset(*__slate_slot_727 as isize) }).p2 }) - (1 as i32);
}
*__slate_slot_708 = unsafe { (*p).aOp };
*__slate_slot_717 = unsafe { (*p).aMem };
*__slate_slot_709 = unsafe { (*__slate_slot_708).offset(*__slate_slot_727 as isize) };
break '__join_26;
} else {
break '__join_1116;
}
}
0 as i32; // There are never any jumps to instruction 0
0 as i32; // Jumps must be in range
*__slate_slot_709 = unsafe { (*__slate_slot_708).offset(((unsafe { (*(*__slate_slot_709)).p2 }) - (1 as i32)) as isize) };
break '__join_26;
}
*__slate_slot_709 = unsafe { (*__slate_slot_708).offset(((unsafe { (*(*__slate_slot_709)).p2 }) - (1 as i32)) as isize) };
// Opcodes that are used as the bottom of a loop (OP_Next, OP_Prev,
// OP_VNext, or OP_SorterNext) all jump here upon
// completion.  Check to see if sqlite3_interrupt() has been called
// or if the progress callback needs to be invoked.
//
// This code uses unstructured "goto" statements and does not look clean.
// But that is not due to sloppy coding habits. The code is written this
// way for performance, to avoid having to run the interrupt and progress
// checks on every opcode.  This helps sqlite3_step() to run about 1.5%
// faster according to "valgrind --tool=cachegrind"
}
if (unsafe { std::sync::atomic::AtomicI32::load_volatile(std::sync::atomic::AtomicI32::from_ptr_raw((unsafe { std::ptr::addr_of_mut!((*(*__slate_slot_711)).u1.isInterrupted) }) as *mut i32), std::sync::atomic::Ordering::Relaxed) }) != (0 as i32) {
break '__join_0;
} else {
// Call the progress callback if it is configured and the required number
// of VDBE ops have been executed (either since this invocation of
// sqlite3VdbeExec() or since last time the progress callback was called).
// If the progress callback returns non-zero, exit the virtual machine with
// a return code SQLITE_ABORT.
loop {
if *__slate_slot_715 >= *__slate_slot_716 && (unsafe { (*(*__slate_slot_711)).xProgress }) != None {
0 as i32;
std::ptr::write(__slate_slot_1548, *__slate_slot_716);
std::ptr::write(__slate_slot_1549, (*__slate_slot_1548).wrapping_add((unsafe { (*(*__slate_slot_711)).nProgressOps }) as u64));
*__slate_slot_716 = *__slate_slot_1549;
if (unsafe { unsafe { (*(*__slate_slot_711)).xProgress }.unwrap()(unsafe { (*(*__slate_slot_711)).pProgressArg }) }) != (0 as i32) {
break '__join_1136;
}
} else {
break '__join_26;
}
}
}
}
std::ptr::write(__slate_slot_1544, *__slate_slot_709);
std::ptr::write(__slate_slot_1545, unsafe { (*__slate_slot_1544).offset((1 as i32) as isize) });
*__slate_slot_709 = *__slate_slot_1545;
} else {
break;
}
                                                                                                                                                                                                                                                                                                                                                                                                                                                                            }
                                                                                                                                                                                                                                                                                                                                                                                                                                                                            // The end of the for(;;) loop the loops through opcodes
                                                                                                                                                                                                                                                                                                                                                                                                                                                                            // If we reach this point, it means that execution is finished with
                                                                                                                                                                                                                                                                                                                                                                                                                                                                            // an error of some kind.
                                                                                                                                                                                                                                                                                                                                                                                                                                                                            __slate_state = 23;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                            break '__join_1152;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                        }
                                                                                                                                                                                                                                                                                                                                                                                                                                                                        __slate_state = 23;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                        break '__join_1152;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                    }
                                                                                                                                                                                                                                                                                                                                                                                                                                                                    __slate_state = 23;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                    break '__join_1152;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                }
                                                                                                                                                                                                                                                                                                                                                                                                                                                                *__slate_slot_710 = 6 as i32;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                __slate_state = 23;
                                                                                                                                                                                                                                                                                                                                                                                                                                                                break '__join_1152;
                                                                                                                                                                                                                                                                                                                                                                                                                                                            }
                                                                                                                                                                                                                                                                                                                                                                                                                                                            __slate_state = 23;
                                                                                                                                                                                                                                                                                                                                                                                                                                                            break '__join_1152;
                                                                                                                                                                                                                                                                                                                                                                                                                                                        }
                                                                                                                                                                                                                                                                                                                                                                                                                                                        __slate_state = 23;
                                                                                                                                                                                                                                                                                                                                                                                                                                                        break '__join_1152;
                                                                                                                                                                                                                                                                                                                                                                                                                                                    }
                                                                                                                                                                                                                                                                                                                                                                                                                                                    __slate_state = 23;
                                                                                                                                                                                                                                                                                                                                                                                                                                                    break '__join_1152;
                                                                                                                                                                                                                                                                                                                                                                                                                                                }
                                                                                                                                                                                                                                                                                                                                                                                                                                                __slate_state = 23;
                                                                                                                                                                                                                                                                                                                                                                                                                                                break '__join_1152;
                                                                                                                                                                                                                                                                                                                                                                                                                                            }
                                                                                                                                                                                                                                                                                                                                                                                                                                            __slate_state = 23;
                                                                                                                                                                                                                                                                                                                                                                                                                                            break '__join_1152;
                                                                                                                                                                                                                                                                                                                                                                                                                                        }
                                                                                                                                                                                                                                                                                                                                                                                                                                        unsafe { sqlite3_free(*__slate_slot_1014 as *mut ()) };
                                                                                                                                                                                                                                                                                                                                                                                                                                        __slate_state = 23;
                                                                                                                                                                                                                                                                                                                                                                                                                                        break '__join_1152;
                                                                                                                                                                                                                                                                                                                                                                                                                                    }
                                                                                                                                                                                                                                                                                                                                                                                                                                    0 as i32;
                                                                                                                                                                                                                                                                                                                                                                                                                                    unsafe { unsafe { (*(*__slate_slot_1010)).xClose }.unwrap()(*__slate_slot_1008) };
                                                                                                                                                                                                                                                                                                                                                                                                                                    break '__join_1;
                                                                                                                                                                                                                                                                                                                                                                                                                                }
                                                                                                                                                                                                                                                                                                                                                                                                                                __slate_state = 23;
                                                                                                                                                                                                                                                                                                                                                                                                                                break '__join_1152;
                                                                                                                                                                                                                                                                                                                                                                                                                            }
                                                                                                                                                                                                                                                                                                                                                                                                                            *__slate_slot_710 = 6 as i32;
                                                                                                                                                                                                                                                                                                                                                                                                                            __slate_state = 23;
                                                                                                                                                                                                                                                                                                                                                                                                                            break '__join_1152;
                                                                                                                                                                                                                                                                                                                                                                                                                        }
                                                                                                                                                                                                                                                                                                                                                                                                                        __slate_state = 23;
                                                                                                                                                                                                                                                                                                                                                                                                                        break '__join_1152;
                                                                                                                                                                                                                                                                                                                                                                                                                    }
                                                                                                                                                                                                                                                                                                                                                                                                                    __slate_state = 23;
                                                                                                                                                                                                                                                                                                                                                                                                                    break '__join_1152;
                                                                                                                                                                                                                                                                                                                                                                                                                }
                                                                                                                                                                                                                                                                                                                                                                                                                __slate_state = 23;
                                                                                                                                                                                                                                                                                                                                                                                                                break '__join_1152;
                                                                                                                                                                                                                                                                                                                                                                                                            }
                                                                                                                                                                                                                                                                                                                                                                                                            if *__slate_slot_710 & (255 as i32) == (6 as i32) {
std::ptr::write(__slate_slot_1003, (unsafe { (*(*__slate_slot_709)).p4.z }) as *const i8);
unsafe { sqlite3VdbeError(p, (b"database table is locked: %s\0".as_ptr() as *mut i8) as *const i8, *__slate_slot_1003) };
__slate_state = 23;
break '__join_1152;
} else {
__slate_state = 23;
break '__join_1152;
}
                                                                                                                                                                                                                                                                                                                                                                                                        }
                                                                                                                                                                                                                                                                                                                                                                                                        __slate_state = 23;
                                                                                                                                                                                                                                                                                                                                                                                                        break '__join_1152;
                                                                                                                                                                                                                                                                                                                                                                                                    }
                                                                                                                                                                                                                                                                                                                                                                                                    __slate_state = 23;
                                                                                                                                                                                                                                                                                                                                                                                                    break '__join_1152;
                                                                                                                                                                                                                                                                                                                                                                                                }
                                                                                                                                                                                                                                                                                                                                                                                                __slate_state = 23;
                                                                                                                                                                                                                                                                                                                                                                                                break '__join_1152;
                                                                                                                                                                                                                                                                                                                                                                                            }
                                                                                                                                                                                                                                                                                                                                                                                            *__slate_slot_710 = 1 as i32;
                                                                                                                                                                                                                                                                                                                                                                                            unsafe { sqlite3VdbeError(p, (b"cannot change %s wal mode from within a transaction\0".as_ptr() as *mut i8) as *const i8, if *__slate_slot_995 == (5 as i32) { b"into\0".as_ptr() as *mut i8 } else { b"out of\0".as_ptr() as *mut i8 }) };
                                                                                                                                                                                                                                                                                                                                                                                            __slate_state = 23;
                                                                                                                                                                                                                                                                                                                                                                                            break '__join_1152;
                                                                                                                                                                                                                                                                                                                                                                                        }
                                                                                                                                                                                                                                                                                                                                                                                        __slate_state = 23;
                                                                                                                                                                                                                                                                                                                                                                                        break '__join_1152;
                                                                                                                                                                                                                                                                                                                                                                                    }
                                                                                                                                                                                                                                                                                                                                                                                    unsafe { sqlite3VdbeError(p, (b"%s\0".as_ptr() as *mut i8) as *const i8, unsafe { sqlite3_value_text(*__slate_slot_989) }) };
                                                                                                                                                                                                                                                                                                                                                                                    __slate_state = 23;
                                                                                                                                                                                                                                                                                                                                                                                    break '__join_1152;
                                                                                                                                                                                                                                                                                                                                                                                }
                                                                                                                                                                                                                                                                                                                                                                                __slate_state = 23;
                                                                                                                                                                                                                                                                                                                                                                                break '__join_1152;
                                                                                                                                                                                                                                                                                                                                                                            }
                                                                                                                                                                                                                                                                                                                                                                            *__slate_slot_710 = 1 as i32;
                                                                                                                                                                                                                                                                                                                                                                            unsafe { sqlite3VdbeError(p, (b"triggers nested too deep\0".as_ptr() as *mut i8) as *const i8) };
                                                                                                                                                                                                                                                                                                                                                                            __slate_state = 23;
                                                                                                                                                                                                                                                                                                                                                                            break '__join_1152;
                                                                                                                                                                                                                                                                                                                                                                        }
                                                                                                                                                                                                                                                                                                                                                                        unsafe { sqlite3_free(*__slate_slot_963 as *mut ()) };
                                                                                                                                                                                                                                                                                                                                                                        __slate_state = 23;
                                                                                                                                                                                                                                                                                                                                                                        break '__join_1152;
                                                                                                                                                                                                                                                                                                                                                                    }
                                                                                                                                                                                                                                                                                                                                                                    __slate_state = 23;
                                                                                                                                                                                                                                                                                                                                                                    break '__join_1152;
                                                                                                                                                                                                                                                                                                                                                                }
                                                                                                                                                                                                                                                                                                                                                                unsafe { sqlite3ResetAllSchemasOfConnection(*__slate_slot_711) };
                                                                                                                                                                                                                                                                                                                                                                if *__slate_slot_710 == (7 as i32) {
break '__join_1;
} else {
__slate_state = 23;
break '__join_1152;
}
                                                                                                                                                                                                                                                                                                                                                            }
                                                                                                                                                                                                                                                                                                                                                            unsafe { sqlite3VdbeError(p, (b"%s\0".as_ptr() as *mut i8) as *const i8, *__slate_slot_952) };
                                                                                                                                                                                                                                                                                                                                                            unsafe { sqlite3_free(*__slate_slot_952 as *mut ()) };
                                                                                                                                                                                                                                                                                                                                                            if *__slate_slot_710 == (7 as i32) {
break '__join_1;
} else {
__slate_state = 23;
break '__join_1152;
}
                                                                                                                                                                                                                                                                                                                                                        }
                                                                                                                                                                                                                                                                                                                                                        __slate_state = 23;
                                                                                                                                                                                                                                                                                                                                                        break '__join_1152;
                                                                                                                                                                                                                                                                                                                                                    }
                                                                                                                                                                                                                                                                                                                                                    __slate_state = 23;
                                                                                                                                                                                                                                                                                                                                                    break '__join_1152;
                                                                                                                                                                                                                                                                                                                                                }
                                                                                                                                                                                                                                                                                                                                                __slate_state = 23;
                                                                                                                                                                                                                                                                                                                                                break '__join_1152;
                                                                                                                                                                                                                                                                                                                                            }
                                                                                                                                                                                                                                                                                                                                            __slate_state = 23;
                                                                                                                                                                                                                                                                                                                                            break '__join_1152;
                                                                                                                                                                                                                                                                                                                                        }
                                                                                                                                                                                                                                                                                                                                        *__slate_slot_710 = 6 as i32;
                                                                                                                                                                                                                                                                                                                                        unsafe {
                                                                                                                                                                                                                                                                                                                                            (*p).errorAction = ((2 as i32) as i8) as u8;
                                                                                                                                                                                                                                                                                                                                        }
                                                                                                                                                                                                                                                                                                                                        __slate_state = 23;
                                                                                                                                                                                                                                                                                                                                        break '__join_1152;
                                                                                                                                                                                                                                                                                                                                    }
                                                                                                                                                                                                                                                                                                                                    __slate_state = 23;
                                                                                                                                                                                                                                                                                                                                    break '__join_1152;
                                                                                                                                                                                                                                                                                                                                }
                                                                                                                                                                                                                                                                                                                                *__slate_slot_710 = unsafe { sqlite3CorruptError(7046 as i32) };
                                                                                                                                                                                                                                                                                                                                __slate_state = 23;
                                                                                                                                                                                                                                                                                                                                break '__join_1152;
                                                                                                                                                                                                                                                                                                                            }
                                                                                                                                                                                                                                                                                                                            __slate_state = 23;
                                                                                                                                                                                                                                                                                                                            break '__join_1152;
                                                                                                                                                                                                                                                                                                                        }
                                                                                                                                                                                                                                                                                                                        __slate_state = 23;
                                                                                                                                                                                                                                                                                                                        break '__join_1152;
                                                                                                                                                                                                                                                                                                                    }
                                                                                                                                                                                                                                                                                                                    __slate_state = 23;
                                                                                                                                                                                                                                                                                                                    break '__join_1152;
                                                                                                                                                                                                                                                                                                                }
                                                                                                                                                                                                                                                                                                                __slate_state = 23;
                                                                                                                                                                                                                                                                                                                break '__join_1152;
                                                                                                                                                                                                                                                                                                            }
                                                                                                                                                                                                                                                                                                            __slate_state = 23;
                                                                                                                                                                                                                                                                                                            break '__join_1152;
                                                                                                                                                                                                                                                                                                        }
                                                                                                                                                                                                                                                                                                        *__slate_slot_710 = unsafe { sqlite3ReportError((11 as i32) | (3 as i32) << (8 as i32), 6822 as i32, (b"index corruption\0".as_ptr() as *mut i8) as *const i8) };
                                                                                                                                                                                                                                                                                                        __slate_state = 23;
                                                                                                                                                                                                                                                                                                        break '__join_1152;
                                                                                                                                                                                                                                                                                                    }
                                                                                                                                                                                                                                                                                                    __slate_state = 23;
                                                                                                                                                                                                                                                                                                    break '__join_1152;
                                                                                                                                                                                                                                                                                                }
                                                                                                                                                                                                                                                                                                __slate_state = 23;
                                                                                                                                                                                                                                                                                                break '__join_1152;
                                                                                                                                                                                                                                                                                            }
                                                                                                                                                                                                                                                                                            __slate_state = 23;
                                                                                                                                                                                                                                                                                            break '__join_1152;
                                                                                                                                                                                                                                                                                        }
                                                                                                                                                                                                                                                                                        __slate_state = 23;
                                                                                                                                                                                                                                                                                        break '__join_1152;
                                                                                                                                                                                                                                                                                    }
                                                                                                                                                                                                                                                                                    __slate_state = 23;
                                                                                                                                                                                                                                                                                    break '__join_1152;
                                                                                                                                                                                                                                                                                }
                                                                                                                                                                                                                                                                                __slate_state = 23;
                                                                                                                                                                                                                                                                                break '__join_1152;
                                                                                                                                                                                                                                                                            }
                                                                                                                                                                                                                                                                            __slate_state = 23;
                                                                                                                                                                                                                                                                            break '__join_1152;
                                                                                                                                                                                                                                                                        }
                                                                                                                                                                                                                                                                        __slate_state = 23;
                                                                                                                                                                                                                                                                        break '__join_1152;
                                                                                                                                                                                                                                                                    }
                                                                                                                                                                                                                                                                    __slate_state = 23;
                                                                                                                                                                                                                                                                    break '__join_1152;
                                                                                                                                                                                                                                                                }
                                                                                                                                                                                                                                                                __slate_state = 23;
                                                                                                                                                                                                                                                                break '__join_1152;
                                                                                                                                                                                                                                                            }
                                                                                                                                                                                                                                                            __slate_state = 23;
                                                                                                                                                                                                                                                            break '__join_1152;
                                                                                                                                                                                                                                                        }
                                                                                                                                                                                                                                                        __slate_state = 23;
                                                                                                                                                                                                                                                        break '__join_1152;
                                                                                                                                                                                                                                                    }
                                                                                                                                                                                                                                                    __slate_state = 23;
                                                                                                                                                                                                                                                    break '__join_1152;
                                                                                                                                                                                                                                                }
                                                                                                                                                                                                                                                __slate_state = 23;
                                                                                                                                                                                                                                                break '__join_1152;
                                                                                                                                                                                                                                            }
                                                                                                                                                                                                                                            __slate_state = 23;
                                                                                                                                                                                                                                            break '__join_1152;
                                                                                                                                                                                                                                        }
                                                                                                                                                                                                                                        __slate_state = 23;
                                                                                                                                                                                                                                        break '__join_1152;
                                                                                                                                                                                                                                    }
                                                                                                                                                                                                                                    __slate_state = 23;
                                                                                                                                                                                                                                    break '__join_1152;
                                                                                                                                                                                                                                }
                                                                                                                                                                                                                                __slate_state = 23;
                                                                                                                                                                                                                                break '__join_1152;
                                                                                                                                                                                                                            }
                                                                                                                                                                                                                            __slate_state = 23;
                                                                                                                                                                                                                            break '__join_1152;
                                                                                                                                                                                                                        }
                                                                                                                                                                                                                        *__slate_slot_710 = 13 as i32; // IMP: R-38219-53002
                                                                                                                                                                                                                        __slate_state = 23;
                                                                                                                                                                                                                        break '__join_1152;
                                                                                                                                                                                                                    }
                                                                                                                                                                                                                    __slate_state = 23;
                                                                                                                                                                                                                    break '__join_1152;
                                                                                                                                                                                                                }
                                                                                                                                                                                                                *__slate_slot_710 = 13 as i32; // IMP: R-17817-00630
                                                                                                                                                                                                                __slate_state = 23;
                                                                                                                                                                                                                break '__join_1152;
                                                                                                                                                                                                            }
                                                                                                                                                                                                            __slate_state = 23;
                                                                                                                                                                                                            break '__join_1152;
                                                                                                                                                                                                        }
                                                                                                                                                                                                        __slate_state = 23;
                                                                                                                                                                                                        break '__join_1152;
                                                                                                                                                                                                    }
                                                                                                                                                                                                    __slate_state = 23;
                                                                                                                                                                                                    break '__join_1152;
                                                                                                                                                                                                }
                                                                                                                                                                                                __slate_state = 23;
                                                                                                                                                                                                break '__join_1152;
                                                                                                                                                                                            }
                                                                                                                                                                                            __slate_state = 23;
                                                                                                                                                                                            break '__join_1152;
                                                                                                                                                                                        }
                                                                                                                                                                                        __slate_state = 23;
                                                                                                                                                                                        break '__join_1152;
                                                                                                                                                                                    }
                                                                                                                                                                                    __slate_state = 23;
                                                                                                                                                                                    break '__join_1152;
                                                                                                                                                                                }
                                                                                                                                                                                __slate_state = 23;
                                                                                                                                                                                break '__join_1152;
                                                                                                                                                                            }
                                                                                                                                                                            __slate_state = 23;
                                                                                                                                                                            break '__join_1152;
                                                                                                                                                                        }
                                                                                                                                                                        __slate_state = 23;
                                                                                                                                                                        break '__join_1152;
                                                                                                                                                                    }
                                                                                                                                                                    __slate_state = 23;
                                                                                                                                                                    break '__join_1152;
                                                                                                                                                                }
                                                                                                                                                                __slate_state = 23;
                                                                                                                                                                break '__join_1152;
                                                                                                                                                            }
                                                                                                                                                            *__slate_slot_710 = (4 as i32) | (2 as i32) << (8 as i32);
                                                                                                                                                            __slate_state = 23;
                                                                                                                                                            break '__join_1152;
                                                                                                                                                        }
                                                                                                                                                        __slate_state = 23;
                                                                                                                                                        break '__join_1152;
                                                                                                                                                    }
                                                                                                                                                    __slate_state = 23;
                                                                                                                                                    break '__join_1152;
                                                                                                                                                }
                                                                                                                                                __slate_state = 23;
                                                                                                                                                break '__join_1152;
                                                                                                                                            }
                                                                                                                                            if *__slate_slot_710 & (255 as i32) == (5 as i32) {
unsafe {
(*p).pc = ((unsafe { (*__slate_slot_709).offset_from(*__slate_slot_708 as *mut VdbeOp) }) as i64) as i32;
}
unsafe {
(*p).rc = *__slate_slot_710;
}
__slate_state = 6;
break '__join_1152;
} else {
__slate_state = 23;
break '__join_1152;
}
                                                                                                                                        }
                                                                                                                                        if (unsafe { (*(*__slate_slot_711)).flags }) & (((1048576 as i32) as i64) as u64) != (0 as u64) {
// Writes prohibited by the "PRAGMA query_only=TRUE" statement
*__slate_slot_710 = 8 as i32;
__slate_state = 23;
break '__join_1152;
} else {
// Writes prohibited due to a prior SQLITE_CORRUPT in the current
// transaction
*__slate_slot_710 = 11 as i32;
__slate_state = 23;
break '__join_1152;
}
                                                                                                                                    }
                                                                                                                                    *__slate_slot_831 = unsafe { (*(*__slate_slot_709)).p1 };
                                                                                                                                    *__slate_slot_832 = unsafe { (*(*__slate_slot_709)).p2 };
                                                                                                                                    0 as i32;
                                                                                                                                    0 as i32;
                                                                                                                                    0 as i32; // At least this one VM is active
                                                                                                                                    0 as i32;
                                                                                                                                    if *__slate_slot_831 != (((unsafe { (*(*__slate_slot_711)).autoCommit }) as u32) as i32) {
if *__slate_slot_832 != (0 as i32) {
0 as i32;
unsafe { sqlite3RollbackAll(*__slate_slot_711, (4 as i32) | (2 as i32) << (8 as i32)) };
unsafe {
(*(*__slate_slot_711)).autoCommit = ((1 as i32) as i8) as u8;
}
} else {
if *__slate_slot_831 != (0 as i32) && (unsafe { (*(*__slate_slot_711)).nVdbeWrite }) > (0 as i32) {
// If this instruction implements a COMMIT and other VMs are writing
// return an error indicating that the other VMs must complete first.
unsafe { sqlite3VdbeError(p, (b"cannot commit transaction - SQL statements in progress\0".as_ptr() as *mut i8) as *const i8) };
*__slate_slot_710 = 5 as i32;
__slate_state = 23;
break '__join_1152;
} else {
std::ptr::write(__slate_slot_1773, unsafe { sqlite3VdbeCheckFkDeferred(p) });
*__slate_slot_710 = *__slate_slot_1773;
if *__slate_slot_1773 != (0 as i32) {
__slate_state = 6;
break '__join_1152;
} else {
unsafe {
(*(*__slate_slot_711)).autoCommit = (*__slate_slot_831 as i8) as u8;
}
}
}
}
if (unsafe { sqlite3VdbeHalt(p) }) == (5 as i32) {
unsafe {
(*p).pc = ((unsafe { (*__slate_slot_709).offset_from(*__slate_slot_708 as *mut VdbeOp) }) as i64) as i32;
}
unsafe {
(*(*__slate_slot_711)).autoCommit = (((1 as i32) - *__slate_slot_831) as i8) as u8;
}
*__slate_slot_710 = 5 as i32;
unsafe {
(*p).rc = 5 as i32;
}
__slate_state = 6;
break '__join_1152;
} else {
unsafe { sqlite3CloseSavepoints(*__slate_slot_711) };
if (unsafe { (*p).rc }) == (0 as i32) {
*__slate_slot_710 = 101 as i32;
__slate_state = 6;
break '__join_1152;
} else {
*__slate_slot_710 = 1 as i32;
__slate_state = 6;
break '__join_1152;
}
}
} else {
unsafe { sqlite3VdbeError(p, (if !(*__slate_slot_831 != (0 as i32)) { b"cannot start a transaction within a transaction\0".as_ptr() as *mut i8 } else { if *__slate_slot_832 != (0 as i32) { b"cannot rollback - no transaction is active\0".as_ptr() as *mut i8 } else { b"cannot commit - no transaction is active\0".as_ptr() as *mut i8 } }) as *const i8) };
*__slate_slot_710 = 1 as i32;
__slate_state = 23;
break '__join_1152;
}
                                                                                                                                }
                                                                                                                                __slate_state = 23;
                                                                                                                                break '__join_1152;
                                                                                                                            }
                                                                                                                            __slate_state = 23;
                                                                                                                            break '__join_1152;
                                                                                                                        }
                                                                                                                        __slate_state = 23;
                                                                                                                        break '__join_1152;
                                                                                                                    }
                                                                                                                    __slate_state = 23;
                                                                                                                    break '__join_1152;
                                                                                                                }
                                                                                                                unsafe {
                                                                                                                    (*p).pc = ((unsafe { (*__slate_slot_709).offset_from(*__slate_slot_708 as *mut VdbeOp) }) as i64) as i32;
                                                                                                                }
                                                                                                                unsafe {
                                                                                                                    (*(*__slate_slot_711)).autoCommit = ((0 as i32) as i8) as u8;
                                                                                                                }
                                                                                                                *__slate_slot_710 = 5 as i32;
                                                                                                                unsafe {
                                                                                                                    (*p).rc = 5 as i32;
                                                                                                                }
                                                                                                                __slate_state = 6;
                                                                                                                break '__join_1152;
                                                                                                            }
                                                                                                            __slate_state = 6;
                                                                                                            break '__join_1152;
                                                                                                        }
                                                                                                        __slate_state = 23;
                                                                                                        break '__join_1152;
                                                                                                    }
                                                                                                    *__slate_slot_710 = 101 as i32;
                                                                                                    __slate_state = 6;
                                                                                                    break '__join_1152;
                                                                                                }
                                                                                                __slate_state = 23;
                                                                                                break '__join_1152;
                                                                                            }
                                                                                            __slate_state = 23;
                                                                                            break '__join_1152;
                                                                                        }
                                                                                        unsafe {
                                                                                            sqlite3VdbeError(p, (b"cannot store %s value in %s column %s.%s\0".as_ptr() as *mut i8) as *const i8, vdbeMemTypeName(*__slate_slot_718), unsafe { *unsafe { unsafe { std::ptr::addr_of_mut!(sqlite3StdType) as *mut *const i8 }.offset((((unsafe { (*unsafe { (*__slate_slot_797).offset(*__slate_slot_798 as isize) }).__slate_bits_0.__get_eCType() }) as i32) - (1 as i32)) as isize) } }, unsafe { (*(*__slate_slot_796)).zName }, unsafe { (*unsafe { (*__slate_slot_797).offset(*__slate_slot_798 as isize) }).zCnName })
                                                                                        };
                                                                                        *__slate_slot_710 = (19 as i32) | (12 as i32) << (8 as i32);
                                                                                        __slate_state = 23;
                                                                                        break '__join_1152;
                                                                                    }
                                                                                    __slate_state =
                                                                                        23;
                                                                                    break '__join_1152;
                                                                                }
                                                                                __slate_state = 23;
                                                                                break '__join_1152;
                                                                            }
                                                                            __slate_state = 23;
                                                                            break '__join_1152;
                                                                        }
                                                                        if *__slate_slot_710
                                                                            == (7 as i32)
                                                                        {
                                                                            break '__join_1;
                                                                        } else {
                                                                            if *__slate_slot_710
                                                                                == (18 as i32)
                                                                            {
                                                                                break '__join_2;
                                                                            } else {
                                                                                __slate_state = 23;
                                                                                break '__join_1152;
                                                                            }
                                                                        }
                                                                    }
                                                                    __slate_state = 23;
                                                                    break '__join_1152;
                                                                }
                                                                *__slate_slot_710 = unsafe {
                                                                    sqlite3CorruptError(3386 as i32)
                                                                };
                                                                __slate_state = 23;
                                                                break '__join_1152;
                                                            }
                                                            __slate_state = 23;
                                                            break '__join_1152;
                                                        }
                                                        __slate_state = 23;
                                                        break '__join_1152;
                                                    }
                                                    __slate_state = 23;
                                                    break '__join_1152;
                                                }
                                                *__slate_slot_710 = 20 as i32;
                                                __slate_state = 23;
                                                break '__join_1152;
                                            }
                                            0 as i32;
                                            0 as i32;
                                            0 as i32;
                                            unsafe {
                                                (*p).cacheCtr = unsafe { (*p).cacheCtr }
                                                    .wrapping_add((2 as i32) as u32)
                                                    | ((1 as i32) as u32);
                                            }
                                            unsafe {
                                                (*p).pResultRow = unsafe {
                                                    (*__slate_slot_717).offset(
                                                        (unsafe { (*(*__slate_slot_709)).p1 })
                                                            as isize,
                                                    )
                                                };
                                            }
                                            if (unsafe { (*(*__slate_slot_711)).mallocFailed })
                                                != (0 as u8)
                                            {
                                                break '__join_1;
                                            } else {
                                                if (((unsafe { (*(*__slate_slot_711)).mTrace })
                                                    as u32)
                                                    as i32)
                                                    & (4 as i32)
                                                    != (0 as i32)
                                                {
                                                    unsafe {
                                                        unsafe { (*(*__slate_slot_711)).trace.xV2 }
                                                            .unwrap()(
                                                            (4 as i32) as u32,
                                                            unsafe {
                                                                (*(*__slate_slot_711)).pTraceArg
                                                            },
                                                            p as *mut (),
                                                            std::ptr::null_mut::<()>(),
                                                        )
                                                    };
                                                }
                                                unsafe {
                                                    (*p).pc = (((unsafe {
                                                        (*__slate_slot_709).offset_from(
                                                            *__slate_slot_708 as *mut VdbeOp,
                                                        )
                                                    })
                                                        as i64)
                                                        as i32)
                                                        + (1 as i32);
                                                }
                                                *__slate_slot_710 = 100 as i32;
                                                __slate_state = 6;
                                                break '__join_1152;
                                            }
                                        }
                                        __slate_state = 23;
                                        break '__join_1152;
                                    }
                                    unsafe {
                                        sqlite3VdbeError(
                                            p,
                                            (b"string or blob too big\0".as_ptr() as *mut i8)
                                                as *const i8,
                                        )
                                    };
                                    *__slate_slot_710 = 18 as i32;
                                    __slate_state = 23;
                                    break '__join_1152;
                                }
                                unsafe {
                                    (*p).rc = unsafe { (*(*__slate_slot_709)).p1 };
                                }
                                unsafe {
                                    (*p).errorAction =
                                        ((unsafe { (*(*__slate_slot_709)).p2 }) as i8) as u8;
                                }
                                0 as i32;
                                if (unsafe { (*p).rc }) != (0 as i32) {
                                    if (unsafe { (*(*__slate_slot_709)).p3 }) > (0 as i32)
                                        && ((unsafe { (*(*__slate_slot_709)).p4type }) as i32)
                                            == (0 as i32)
                                    {
                                        0 as i32;
                                        *__slate_slot_728 = (unsafe {
                                            sqlite3ValueText(
                                                unsafe {
                                                    (*__slate_slot_717).offset(
                                                        (unsafe { (*(*__slate_slot_709)).p3 })
                                                            as isize,
                                                    )
                                                },
                                                ((1 as i32) as i8) as u8,
                                            )
                                        })
                                            as *const i8;
                                        unsafe {
                                            sqlite3VdbeError(
                                                p,
                                                (b"%s\0".as_ptr() as *mut i8) as *const i8,
                                                *__slate_slot_728,
                                            )
                                        };
                                    } else {
                                        if (unsafe { (*(*__slate_slot_709)).p5 }) != (0 as u16) {
                                            {}
                                            {}
                                            {}
                                            {}
                                            unsafe {
                                                sqlite3VdbeError(
                                                    p,
                                                    (b"%s constraint failed\0".as_ptr() as *mut i8)
                                                        as *const i8,
                                                    unsafe {
                                                        *unsafe {
                                                            unsafe {
                                                                std::ptr::addr_of!(azType.0)
                                                                    as *const *const i8
                                                            }
                                                            .offset(
                                                                ((((unsafe {
                                                                    (*(*__slate_slot_709)).p5
                                                                })
                                                                    as u32)
                                                                    as i32)
                                                                    - (1 as i32))
                                                                    as isize,
                                                            )
                                                        }
                                                    },
                                                )
                                            };
                                            if (unsafe { (*(*__slate_slot_709)).p4.z })
                                                != std::ptr::null_mut::<i8>()
                                            {
                                                unsafe {
                                                    (*p).zErrMsg = unsafe {
                                                        sqlite3MPrintf(
                                                            *__slate_slot_711,
                                                            (b"%z: %s\0".as_ptr() as *mut i8)
                                                                as *const i8,
                                                            unsafe { (*p).zErrMsg },
                                                            unsafe { (*(*__slate_slot_709)).p4.z },
                                                        )
                                                    };
                                                }
                                            }
                                        } else {
                                            unsafe {
                                                sqlite3VdbeError(
                                                    p,
                                                    (b"%s\0".as_ptr() as *mut i8) as *const i8,
                                                    unsafe { (*(*__slate_slot_709)).p4.z },
                                                )
                                            };
                                        }
                                    }
                                    sqlite3VdbeLogAbort(
                                        p,
                                        unsafe { (*(*__slate_slot_709)).p1 },
                                        *__slate_slot_709,
                                        *__slate_slot_708,
                                    );
                                }
                                *__slate_slot_710 = unsafe { sqlite3VdbeHalt(p) };
                                0 as i32;
                                if *__slate_slot_710 == (5 as i32) {
                                    unsafe {
                                        (*p).rc = 5 as i32;
                                    }
                                    __slate_state = 6;
                                    break '__join_1152;
                                } else {
                                    0 as i32;
                                    0 as i32;
                                    *__slate_slot_710 = if (unsafe { (*p).rc }) != (0 as i32) {
                                        1 as i32
                                    } else {
                                        101 as i32
                                    };
                                    __slate_state = 6;
                                    break '__join_1152;
                                }
                            }
                            *__slate_slot_716 = ((4294967295 as u32) as u64)
                                | ((4294967295 as u32) as u64) << (32 as i32);
                            *__slate_slot_710 = 9 as i32;
                            __slate_state = 23;
                            break '__join_1152;
                        }
                    }
                    0 as i32;
                    *__slate_slot_710 = 9 as i32;
                    __slate_state = 23;
                    break '__join_1152;
                }
            }
            unsafe { sqlite3OomFault(*__slate_slot_711) };
            unsafe { sqlite3VdbeError(p, (b"out of memory\0".as_ptr() as *mut i8) as *const i8) };
            *__slate_slot_710 = 7 as i32;
            __slate_state = 23;
        }
        loop {
            match __slate_state {
                6 => {
                    if *__slate_slot_715 >= *__slate_slot_716
                        && (unsafe { (*(*__slate_slot_711)).xProgress }) != None
                    {
                        std::ptr::write(__slate_slot_1962, *__slate_slot_716);
                        std::ptr::write(
                            __slate_slot_1963,
                            (*__slate_slot_1962).wrapping_add(
                                (unsafe { (*(*__slate_slot_711)).nProgressOps }) as u64,
                            ),
                        );
                        *__slate_slot_716 = *__slate_slot_1963;
                        if (unsafe {
                            unsafe { (*(*__slate_slot_711)).xProgress }.unwrap()(unsafe {
                                (*(*__slate_slot_711)).pProgressArg
                            })
                        }) != (0 as i32)
                        {
                            *__slate_slot_716 = ((4294967295 as u32) as u64)
                                | ((4294967295 as u32) as u64) << (32 as i32);
                            *__slate_slot_710 = 9 as i32;
                            __slate_state = 23;
                        } else {
                            __slate_state = 6;
                        }
                    } else {
                        break;
                    }
                }
                _ => {
                    if (unsafe { (*(*__slate_slot_711)).mallocFailed }) != (0 as u8) {
                        *__slate_slot_710 = 7 as i32;
                    } else {
                        if *__slate_slot_710 == (10 as i32) | (33 as i32) << (8 as i32) {
                            *__slate_slot_710 = unsafe { sqlite3CorruptError(9484 as i32) };
                        }
                    }
                    0 as i32;
                    if (unsafe { (*p).zErrMsg }) == std::ptr::null_mut::<i8>()
                        && *__slate_slot_710 != (10 as i32) | (12 as i32) << (8 as i32)
                    {
                        unsafe {
                            sqlite3VdbeError(
                                p,
                                (b"%s\0".as_ptr() as *mut i8) as *const i8,
                                unsafe { sqlite3ErrStr(*__slate_slot_710) },
                            )
                        };
                    }
                    unsafe {
                        (*p).rc = *__slate_slot_710;
                    }
                    unsafe { sqlite3SystemError(*__slate_slot_711, *__slate_slot_710) };
                    {}
                    sqlite3VdbeLogAbort(p, *__slate_slot_710, *__slate_slot_709, *__slate_slot_708);
                    if (((unsafe { (*p).eVdbeState }) as u32) as i32) == (2 as i32) {
                        unsafe { sqlite3VdbeHalt(p) };
                    }
                    if *__slate_slot_710 == (10 as i32) | (12 as i32) << (8 as i32) {
                        unsafe { sqlite3OomFault(*__slate_slot_711) };
                    }
                    if *__slate_slot_710 == (11 as i32)
                        && (((unsafe { (*(*__slate_slot_711)).autoCommit }) as u32) as i32)
                            == (0 as i32)
                    {
                        std::ptr::write(__slate_slot_1959, *__slate_slot_711);
                        std::ptr::write(__slate_slot_1960, unsafe {
                            (*(*__slate_slot_1959)).flags
                        });
                        std::ptr::write(
                            __slate_slot_1961,
                            *__slate_slot_1960 | (((2 as i32) as i64) as u64) << (32 as i32),
                        );
                        unsafe {
                            (*(*__slate_slot_1959)).flags = *__slate_slot_1961;
                        }
                    }
                    *__slate_slot_710 = 1 as i32;
                    if ((*__slate_slot_712 as u32) as i32) > (0 as i32) {
                        unsafe {
                            sqlite3ResetOneSchema(
                                *__slate_slot_711,
                                ((*__slate_slot_712 as u32) as i32) - (1 as i32),
                            )
                        };
                    }
                    // This is the only way out of this procedure.  We have to
                    // release the mutexes on btrees that were acquired at the
                    // top.
                    __slate_state = 6;
                }
            }
        }
        std::ptr::write(__slate_slot_1964, unsafe {
            unsafe { (*p).aCounter.as_mut_ptr() as *mut u32 }.offset((4 as i32) as isize)
        });
        std::ptr::write(__slate_slot_1965, unsafe { *(*__slate_slot_1964) });
        std::ptr::write(
            __slate_slot_1966,
            (*__slate_slot_1965).wrapping_add(((*__slate_slot_715 as u32) as i32) as u32),
        );
        unsafe {
            *(*__slate_slot_1964) = *__slate_slot_1966;
        }
        if (unsafe { (*p).lockMask }) != ((0 as i32) as u32) {
            unsafe { sqlite3VdbeLeave(p) };
        }
        0 as i32;
        return *__slate_slot_710;
    }
    // What follows is a massive switch statement where each case implements a
    // separate instruction in the virtual machine.  If we follow the usual
    // indentation conventions, each case should be indented by 6 spaces.  But
    // that is a lot of wasted space on the left margin.  So the code within
    // the switch statement will break with convention and be flush-left. Another
    // big comment (similar to this one) will mark the point in the code where
    // we transition back to normal indentation.
    //
    // The formatting of each case is important.  The makefile for SQLite
    // generates two C files "opcodes.h" and "opcodes.c" by scanning this
    // file looking for lines that begin with "case OP_".  The opcodes.h files
    // will be filled with #defines that give unique integer values to each
    // opcode and the opcodes.c file is filled with an array of strings where
    // each string is the symbolic name for the corresponding opcode.  If the
    // case statement is followed by a comment of the form "/# same as ... #/"
    // that comment is used to determine the particular value of the opcode.
    //
    // Other keywords in the comment that follows each case are used to
    // construct the OPFLG_INITIALIZER value that initializes opcodeProperty[].
    // Keywords include: in1, in2, in3, out2, out3.  See
    // the mkopcodeh.awk script for additional information.
    //
    // Documentation about VDBE opcodes is generated by scanning this file
    // for lines of that contain "Opcode:".  That line and all subsequent
    // comment lines are used in the generation of the opcode.html documentation
    // file.
    //
    // SUMMARY:
    //
    //     Formatting is important to scripts that scan this file.
    //     Do not deviate from the formatting style currently in use.
    // Opcode:  Goto * P2 * * *
    //
    // An unconditional jump to address P2.
    // The next instruction executed will be
    // the one at index P2 from the beginning of
    // the program.
    //
    // The P1 parameter is not actually used by this opcode.  However, it
    // is sometimes set to 1 instead of 0 as a hint to the command-line shell
    // that this Goto is the bottom of a loop and that the lines from P2 down
    // to the current line should be indented for EXPLAIN output.
    // Opcode:  Gosub P1 P2 * * *
    //
    // Write the current address onto register P1
    // and then jump to address P2.
    // Opcode:  Return P1 P2 P3 * *
    //
    // Jump to the address stored in register P1.  If P1 is a return address
    // register, then this accomplishes a return from a subroutine.
    //
    // If P3 is 1, then the jump is only taken if register P1 holds an integer
    // values, otherwise execution falls through to the next opcode, and the
    // OP_Return becomes a no-op. If P3 is 0, then register P1 must hold an
    // integer or else an assert() is raised.  P3 should be set to 1 when
    // this opcode is used in combination with OP_BeginSubrtn, and set to 0
    // otherwise.
    //
    // The value in register P1 is unchanged by this opcode.
    //
    // P2 is not used by the byte-code engine.  However, if P2 is positive
    // and also less than the current address, then the "EXPLAIN" output
    // formatter in the CLI will indent all opcodes from the P2 opcode up
    // to be not including the current Return.   P2 should be the first opcode
    // in the subroutine from which this opcode is returning.  Thus the P2
    // value is a byte-code indentation hint.  See tag-20220407a in
    // wherecode.c and shell.c.
    // Opcode: InitCoroutine P1 P2 P3 * *
    //
    // Set up register P1 so that it will Yield to the coroutine
    // located at address P3.
    //
    // If P2!=0 then the coroutine implementation immediately follows
    // this opcode.  So jump over the coroutine implementation to
    // address P2.
    //
    // See also: EndCoroutine
    // Opcode:  EndCoroutine P1 * * * *
    //
    // The instruction at the address in register P1 is a Yield.
    // Jump to the P2 parameter of that Yield.
    // After the jump, the value register P1 is left with a value
    // such that subsequent OP_Yields go back to the this same
    // OP_EndCoroutine instruction.
    //
    // See also: InitCoroutine
    // Opcode:  Yield P1 P2 * * *
    //
    // Swap the program counter with the value in register P1.  This
    // has the effect of yielding to a coroutine.
    //
    // If the coroutine that is launched by this instruction ends with
    // Yield or Return then continue to the next instruction.  But if
    // the coroutine launched by this instruction ends with
    // EndCoroutine, then jump to P2 rather than continuing with the
    // next instruction.
    //
    // See also: InitCoroutine
    // Opcode:  HaltIfNull  P1 P2 P3 P4 P5
    // Synopsis: if r[P3]=null halt
    //
    // Check the value in register P3.  If it is NULL then Halt using
    // parameter P1, P2, and P4 as if this were a Halt instruction.  If the
    // value in register P3 is not NULL, then this routine is a no-op.
    // The P5 parameter should be 1.
    // Opcode: Integer P1 P2 * * *
    // Synopsis: r[P2]=P1
    //
    // The 32-bit integer value P1 is written into register P2.
    // Opcode: Int64 P1 P2 P3 * *
    // Synopsis: r[P2]=PINT13
    //
    // Combine P1 and P3 into a signed 64-bit integer.  P1 is the least
    // significant 32 bits and P3 is the most significant.  Store the
    // result in register P2.
    // Opcode: Real P1 P2 P3 * *
    // Synopsis: r[P2]=PDBL13
    //
    // P1 and P3 are combined to 64 bits with P1 being the lower the P3
    // the upper.  The result is interpreted as a 64-bit floating-point
    // and stored in register P2.
    // Opcode: String8 * P2 * P4 *
    // Synopsis: r[P2]='P4'
    //
    // P4 points to a nul terminated UTF-8 string. This opcode is transformed
    // into a String opcode before it is executed for the first time.  During
    // this transformation, the length of string P4 is computed and stored
    // as the P1 parameter.
    // Opcode: BeginSubrtn * P2 * * *
    // Synopsis: r[P2]=NULL
    //
    // Mark the beginning of a subroutine that can be entered in-line
    // or that can be called using OP_Gosub.  The subroutine should
    // be terminated by an OP_Return instruction that has a P1 operand that
    // is the same as the P2 operand to this opcode and that has P3 set to 1.
    // If the subroutine is entered in-line, then the OP_Return will simply
    // fall through.  But if the subroutine is entered using OP_Gosub, then
    // the OP_Return will jump back to the first instruction after the OP_Gosub.
    //
    // This routine works by loading a NULL into the P2 register.  When the
    // return address register contains a NULL, the OP_Return instruction is
    // a no-op that simply falls through to the next instruction (assuming that
    // the OP_Return opcode has a P3 value of 1).  Thus if the subroutine is
    // entered in-line, then the OP_Return will cause in-line execution to
    // continue.  But if the subroutine is entered via OP_Gosub, then the
    // OP_Return will cause a return to the address following the OP_Gosub.
    //
    // This opcode is identical to OP_Null.  It has a different name
    // only to make the byte code easier to read and verify.
    //
    // Opcode: Null P1 P2 P3 * *
    // Synopsis: r[P2..P3]=NULL
    //
    // Write a NULL into registers P2.  If P3 greater than P2, then also write
    // NULL into register P3 and every register in between P2 and P3.  If P3
    // is less than P2 (typically P3 is zero) then only register P2 is
    // set to NULL.
    //
    // If the P1 value is non-zero, then also set the MEM_Cleared flag so that
    // NULL values will not compare equal even if SQLITE_NULLEQ is set on
    // OP_Ne or OP_Eq.
    // Opcode: SoftNull P1 * * * *
    // Synopsis: r[P1]=NULL
    //
    // Set register P1 to have the value NULL as seen by the OP_MakeRecord
    // instruction, but do not free any string or blob memory associated with
    // the register, so that if the value was a string or blob that was
    // previously copied using OP_SCopy, the copies will continue to be valid.
    // Opcode: Blob P1 P2 * P4 *
    // Synopsis: r[P2]=P4 (len=P1)
    //
    // P4 points to a blob of data P1 bytes long.  Store this
    // blob in register P2.  If P4 is a NULL pointer, then construct
    // a zero-filled blob that is P1 bytes long in P2.
    // Opcode: Variable P1 P2 * * *
    // Synopsis: r[P2]=parameter(P1)
    //
    // Transfer the values of bound parameter P1 into register P2
    // Opcode: Move P1 P2 P3 * *
    // Synopsis: r[P2@P3]=r[P1@P3]
    //
    // Move the P3 values in register P1..P1+P3-1 over into
    // registers P2..P2+P3-1.  Registers P1..P1+P3-1 are
    // left holding a NULL.  It is an error for register ranges
    // P1..P1+P3-1 and P2..P2+P3-1 to overlap.  It is an error
    // for P3 to be less than 1.
    // Opcode: Copy P1 P2 P3 * P5
    // Synopsis: r[P2@P3+1]=r[P1@P3+1]
    //
    // Make a copy of registers P1..P1+P3 into registers P2..P2+P3.
    //
    // If the 0x0002 bit of P5 is set then also clear the MEM_Subtype flag in the
    // destination.  The 0x0001 bit of P5 indicates that this Copy opcode cannot
    // be merged.  The 0x0001 bit is used by the query planner and does not
    // come into play during query execution.
    //
    // This instruction makes a deep copy of the value.  A duplicate
    // is made of any string or blob constant.  See also OP_SCopy.
    // Opcode: SCopy P1 P2 * * *
    // Synopsis: r[P2]=r[P1]
    //
    // Make a shallow copy of register P1 into register P2.
    //
    // This instruction makes a shallow copy of the value.  If the value
    // is a string or blob, then the copy is only a pointer to the
    // original and hence if the original changes so will the copy.
    // Worse, if the original is deallocated, the copy becomes invalid.
    // Thus the program must guarantee that the original will not change
    // during the lifetime of the copy.  Use OP_Copy to make a complete
    // copy.
    // Opcode: IntCopy P1 P2 * * *
    // Synopsis: r[P2]=r[P1]
    //
    // Transfer the integer value held in register P1 into register P2.
    //
    // This is an optimized version of SCopy that works only for integer
    // values.
    // Opcode: FkCheck * * * * *
    //
    // Halt with an SQLITE_CONSTRAINT error if there are any unresolved
    // foreign key constraint violations.  If there are no foreign key
    // constraint violations, this is a no-op.
    //
    // FK constraint violations are also checked when the prepared statement
    // exits.  This opcode is used to raise foreign key constraint errors prior
    // to returning results such as a row change count or the result of a
    // RETURNING clause.
    // Opcode: ResultRow P1 P2 * * *
    // Synopsis: output=r[P1@P2]
    //
    // The registers P1 through P1+P2-1 contain a single row of
    // results. This opcode causes the sqlite3_step() call to terminate
    // with an SQLITE_ROW return code and it sets up the sqlite3_stmt
    // structure to provide access to the r(P1)..r(P1+P2-1) values as
    // the result row.
    // Opcode: Concat P1 P2 P3 * *
    // Synopsis: r[P3]=r[P2]+r[P1]
    //
    // Add the text in register P1 onto the end of the text in
    // register P2 and store the result in register P3.
    // If either the P1 or P2 text are NULL then store NULL in P3.
    //
    //   P3 = P2 || P1
    //
    // It is illegal for P1 and P3 to be the same register. Sometimes,
    // if P3 is the same register as P2, the implementation is able
    // to avoid a memcpy().
    // Opcode: Add P1 P2 P3 * *
    // Synopsis: r[P3]=r[P1]+r[P2]
    //
    // Add the value in register P1 to the value in register P2
    // and store the result in register P3.
    // If either input is NULL, the result is NULL.
    //
    // Opcode: Multiply P1 P2 P3 * *
    // Synopsis: r[P3]=r[P1]*r[P2]
    //
    //
    // Multiply the value in register P1 by the value in register P2
    // and store the result in register P3.
    // If either input is NULL, the result is NULL.
    //
    // Opcode: Subtract P1 P2 P3 * *
    // Synopsis: r[P3]=r[P2]-r[P1]
    //
    // Subtract the value in register P1 from the value in register P2
    // and store the result in register P3.
    // If either input is NULL, the result is NULL.
    //
    // Opcode: Divide P1 P2 P3 * *
    // Synopsis: r[P3]=r[P2]/r[P1]
    //
    // Divide the value in register P1 by the value in register P2
    // and store the result in register P3 (P3=P2/P1). If the value in
    // register P1 is zero, then the result is NULL. If either input is
    // NULL, the result is NULL.
    //
    // Opcode: Remainder P1 P2 P3 * *
    // Synopsis: r[P3]=r[P2]%r[P1]
    //
    // Compute the remainder after integer register P2 is divided by
    // register P1 and store the result in register P3.
    // If the value in register P1 is zero the result is NULL.
    // If either operand is NULL, the result is NULL.
    // same as TK_PLUS, in1, in2, out3
    // same as TK_MINUS, in1, in2, out3
    // same as TK_STAR, in1, in2, out3
    // same as TK_SLASH, in1, in2, out3
    // Opcode: CollSeq P1 * * P4
    //
    // P4 is a pointer to a CollSeq object. If the next call to a user function
    // or aggregate calls sqlite3GetFuncCollSeq(), this collation sequence will
    // be returned. This is used by the built-in min(), max() and nullif()
    // functions.
    //
    // If P1 is not zero, then it is a register that a subsequent min() or
    // max() aggregate will set to 1 if the current row is not the minimum or
    // maximum.  The P1 register is initialized to 0 by this instruction.
    //
    // The interface used by the implementation of the aforementioned functions
    // to retrieve the collation sequence set by this opcode is not available
    // publicly.  Only built-in functions have access to this feature.
    // Opcode: BitAnd P1 P2 P3 * *
    // Synopsis: r[P3]=r[P1]&r[P2]
    //
    // Take the bit-wise AND of the values in register P1 and P2 and
    // store the result in register P3.
    // If either input is NULL, the result is NULL.
    //
    // Opcode: BitOr P1 P2 P3 * *
    // Synopsis: r[P3]=r[P1]|r[P2]
    //
    // Take the bit-wise OR of the values in register P1 and P2 and
    // store the result in register P3.
    // If either input is NULL, the result is NULL.
    //
    // Opcode: ShiftLeft P1 P2 P3 * *
    // Synopsis: r[P3]=r[P2]<<r[P1]
    //
    // Shift the integer value in register P2 to the left by the
    // number of bits specified by the integer in register P1.
    // Store the result in register P3.
    // If either input is NULL, the result is NULL.
    //
    // Opcode: ShiftRight P1 P2 P3 * *
    // Synopsis: r[P3]=r[P2]>>r[P1]
    //
    // Shift the integer value in register P2 to the right by the
    // number of bits specified by the integer in register P1.
    // Store the result in register P3.
    // If either input is NULL, the result is NULL.
    // same as TK_BITAND, in1, in2, out3
    // same as TK_BITOR, in1, in2, out3
    // same as TK_LSHIFT, in1, in2, out3
    // Opcode: AddImm  P1 P2 * * *
    // Synopsis: r[P1]=r[P1]+P2
    //
    // Add the constant P2 to the value in register P1.
    // The result is always an integer.
    //
    // To force any register to be an integer, just add 0.
    // Opcode: MustBeInt P1 P2 * * *
    //
    // Force the value in register P1 to be an integer.  If the value
    // in P1 is not an integer and cannot be converted into an integer
    // without data loss, then jump immediately to P2, or if P2==0
    // raise an SQLITE_MISMATCH exception.
    // Opcode: RealAffinity P1 * * * *
    //
    // If register P1 holds an integer convert it to a real value.
    //
    // This opcode is used when extracting information from a column that
    // has REAL affinity.  Such column values may still be stored as
    // integers, for space efficiency, but after extraction we want them
    // to have only a real value.
    // Opcode: Cast P1 P2 * * *
    // Synopsis: affinity(r[P1])
    //
    // Force the value in register P1 to be the type defined by P2.
    //
    // <ul>
    // <li> P2=='A' &rarr; BLOB
    // <li> P2=='B' &rarr; TEXT
    // <li> P2=='C' &rarr; NUMERIC
    // <li> P2=='D' &rarr; INTEGER
    // <li> P2=='E' &rarr; REAL
    // </ul>
    //
    // A NULL value is not changed by this routine.  It remains NULL.
    // Opcode: Eq P1 P2 P3 P4 P5
    // Synopsis: IF r[P3]==r[P1]
    //
    // Compare the values in register P1 and P3.  If reg(P3)==reg(P1) then
    // jump to address P2.
    //
    // The SQLITE_AFF_MASK portion of P5 must be an affinity character -
    // SQLITE_AFF_TEXT, SQLITE_AFF_INTEGER, and so forth. An attempt is made
    // to coerce both inputs according to this affinity before the
    // comparison is made. If the SQLITE_AFF_MASK is 0x00, then numeric
    // affinity is used. Note that the affinity conversions are stored
    // back into the input registers P1 and P3.  So this opcode can cause
    // persistent changes to registers P1 and P3.
    //
    // Once any conversions have taken place, and neither value is NULL,
    // the values are compared. If both values are blobs then memcmp() is
    // used to determine the results of the comparison.  If both values
    // are text, then the appropriate collating function specified in
    // P4 is used to do the comparison.  If P4 is not specified then
    // memcmp() is used to compare text string.  If both values are
    // numeric, then a numeric comparison is used. If the two values
    // are of different types, then numbers are considered less than
    // strings and strings are considered less than blobs.
    //
    // If SQLITE_NULLEQ is set in P5 then the result of comparison is always either
    // true or false and is never NULL.  If both operands are NULL then the result
    // of comparison is true.  If either operand is NULL then the result is false.
    // If neither operand is NULL the result is the same as it would be if
    // the SQLITE_NULLEQ flag were omitted from P5.
    //
    // This opcode saves the result of comparison for use by the new
    // OP_Jump opcode.
    //
    // Opcode: Ne P1 P2 P3 P4 P5
    // Synopsis: IF r[P3]!=r[P1]
    //
    // This works just like the Eq opcode except that the jump is taken if
    // the operands in registers P1 and P3 are not equal.  See the Eq opcode for
    // additional information.
    //
    // Opcode: Lt P1 P2 P3 P4 P5
    // Synopsis: IF r[P3]<r[P1]
    //
    // Compare the values in register P1 and P3.  If reg(P3)<reg(P1) then
    // jump to address P2.
    //
    // If the SQLITE_JUMPIFNULL bit of P5 is set and either reg(P1) or
    // reg(P3) is NULL then the take the jump.  If the SQLITE_JUMPIFNULL
    // bit is clear then fall through if either operand is NULL.
    //
    // The SQLITE_AFF_MASK portion of P5 must be an affinity character -
    // SQLITE_AFF_TEXT, SQLITE_AFF_INTEGER, and so forth. An attempt is made
    // to coerce both inputs according to this affinity before the
    // comparison is made. If the SQLITE_AFF_MASK is 0x00, then numeric
    // affinity is used. Note that the affinity conversions are stored
    // back into the input registers P1 and P3.  So this opcode can cause
    // persistent changes to registers P1 and P3.
    //
    // Once any conversions have taken place, and neither value is NULL,
    // the values are compared. If both values are blobs then memcmp() is
    // used to determine the results of the comparison.  If both values
    // are text, then the appropriate collating function specified in
    // P4 is  used to do the comparison.  If P4 is not specified then
    // memcmp() is used to compare text string.  If both values are
    // numeric, then a numeric comparison is used. If the two values
    // are of different types, then numbers are considered less than
    // strings and strings are considered less than blobs.
    //
    // This opcode saves the result of comparison for use by the new
    // OP_Jump opcode.
    //
    // Opcode: Le P1 P2 P3 P4 P5
    // Synopsis: IF r[P3]<=r[P1]
    //
    // This works just like the Lt opcode except that the jump is taken if
    // the content of register P3 is less than or equal to the content of
    // register P1.  See the Lt opcode for additional information.
    //
    // Opcode: Gt P1 P2 P3 P4 P5
    // Synopsis: IF r[P3]>r[P1]
    //
    // This works just like the Lt opcode except that the jump is taken if
    // the content of register P3 is greater than the content of
    // register P1.  See the Lt opcode for additional information.
    //
    // Opcode: Ge P1 P2 P3 P4 P5
    // Synopsis: IF r[P3]>=r[P1]
    //
    // This works just like the Lt opcode except that the jump is taken if
    // the content of register P3 is greater than or equal to the content of
    // register P1.  See the Lt opcode for additional information.
    // same as TK_EQ, jump, in1, in3
    // same as TK_NE, jump, in1, in3
    // same as TK_LT, jump, in1, in3
    // same as TK_LE, jump, in1, in3
    // same as TK_GT, jump, in1, in3
    // Opcode: ElseEq * P2 * * *
    //
    // This opcode must follow an OP_Lt or OP_Gt comparison operator.  There
    // can be zero or more OP_ReleaseReg opcodes intervening, but no other
    // opcodes are allowed to occur between this instruction and the previous
    // OP_Lt or OP_Gt.
    //
    // If the result of an OP_Eq comparison on the same two operands as
    // the prior OP_Lt or OP_Gt would have been true, then jump to P2.  If
    // the result of an OP_Eq comparison on the two previous operands
    // would have been false or NULL, then fall through.
    // Opcode: Permutation * * * P4 *
    //
    // Set the permutation used by the OP_Compare operator in the next
    // instruction.  The permutation is stored in the P4 operand.
    //
    // The permutation is only valid for the next opcode which must be
    // an OP_Compare that has the OPFLAG_PERMUTE bit set in P5.
    //
    // The first integer in the P4 integer array is the length of the array
    // and does not become part of the permutation.
    // Opcode: Compare P1 P2 P3 P4 P5
    // Synopsis: r[P1@P3] <-> r[P2@P3]
    //
    // Compare two vectors of registers in reg(P1)..reg(P1+P3-1) (call this
    // vector "A") and in reg(P2)..reg(P2+P3-1) ("B").  Save the result of
    // the comparison for use by the next OP_Jump instruct.
    //
    // If P5 has the OPFLAG_PERMUTE bit set, then the order of comparison is
    // determined by the most recent OP_Permutation operator.  If the
    // OPFLAG_PERMUTE bit is clear, then register are compared in sequential
    // order.
    //
    // P4 is a KeyInfo structure that defines collating sequences and sort
    // orders for the comparison.  The permutation applies to registers
    // only.  The KeyInfo elements are used sequentially.
    //
    // The comparison is a sort comparison, so NULLs compare equal,
    // NULLs are less than numbers, numbers are less than strings,
    // and strings are less than blobs.
    //
    // This opcode must be immediately followed by an OP_Jump opcode.
    // Opcode: Jump P1 P2 P3 * *
    //
    // Jump to the instruction at address P1, P2, or P3 depending on whether
    // in the most recent OP_Compare instruction the P1 vector was less than,
    // equal to, or greater than the P2 vector, respectively.
    //
    // This opcode must immediately follow an OP_Compare opcode.
    // Opcode: And P1 P2 P3 * *
    // Synopsis: r[P3]=(r[P1] && r[P2])
    //
    // Take the logical AND of the values in registers P1 and P2 and
    // write the result into register P3.
    //
    // If either P1 or P2 is 0 (false) then the result is 0 even if
    // the other input is NULL.  A NULL and true or two NULLs give
    // a NULL output.
    //
    // Opcode: Or P1 P2 P3 * *
    // Synopsis: r[P3]=(r[P1] || r[P2])
    //
    // Take the logical OR of the values in register P1 and P2 and
    // store the answer in register P3.
    //
    // If either P1 or P2 is nonzero (true) then the result is 1 (true)
    // even if the other input is NULL.  A NULL and false or two NULLs
    // give a NULL output.
    // same as TK_AND, in1, in2, out3
    // Opcode: IsTrue P1 P2 P3 P4 *
    // Synopsis: r[P2] = coalesce(r[P1]==TRUE,P3) ^ P4
    //
    // This opcode implements the IS TRUE, IS FALSE, IS NOT TRUE, and
    // IS NOT FALSE operators.
    //
    // Interpret the value in register P1 as a boolean value.  Store that
    // boolean (a 0 or 1) in register P2.  Or if the value in register P1 is
    // NULL, then the P3 is stored in register P2.  Invert the answer if P4
    // is 1.
    //
    // The logic is summarized like this:
    //
    // <ul>
    // <li> If P3==0 and P4==0  then  r[P2] := r[P1] IS TRUE
    // <li> If P3==1 and P4==1  then  r[P2] := r[P1] IS FALSE
    // <li> If P3==0 and P4==1  then  r[P2] := r[P1] IS NOT TRUE
    // <li> If P3==1 and P4==0  then  r[P2] := r[P1] IS NOT FALSE
    // </ul>
    // Opcode: Not P1 P2 * * *
    // Synopsis: r[P2]= !r[P1]
    //
    // Interpret the value in register P1 as a boolean value.  Store the
    // boolean complement in register P2.  If the value in register P1 is
    // NULL, then a NULL is stored in P2.
    // Opcode: BitNot P1 P2 * * *
    // Synopsis: r[P2]= ~r[P1]
    //
    // Interpret the content of register P1 as an integer.  Store the
    // ones-complement of the P1 value into register P2.  If P1 holds
    // a NULL then store a NULL in P2.
    // Opcode: Once P1 P2 P3 * *
    //
    // Fall through to the next instruction the first time this opcode is
    // encountered on each invocation of the byte-code program.  Jump to P2
    // on the second and all subsequent encounters during the same invocation.
    //
    // Top-level programs determine first invocation by comparing the P1
    // operand against the P1 operand on the OP_Init opcode at the beginning
    // of the program.  If the P1 values differ, then fall through and make
    // the P1 of this opcode equal to the P1 of OP_Init.  If P1 values are
    // the same then take the jump.
    //
    // For subprograms, there is a bitmask in the VdbeFrame that determines
    // whether or not the jump should be taken.  The bitmask is necessary
    // because the self-altering code trick does not work for recursive
    // triggers.
    //
    // The P3 operand is not used directly by this opcode.  However P3 is
    // used by the code generator as follows:  If this opcode is the start
    // of a subroutine and that subroutine uses a Bloom filter, then P3 will
    // be the register that holds that Bloom filter.  See tag-202407032019
    // in the source code for implementation details.
    // Opcode: If P1 P2 P3 * *
    //
    // Jump to P2 if the value in register P1 is true.  The value
    // is considered true if it is numeric and non-zero.  If the value
    // in P1 is NULL then take the jump if and only if P3 is non-zero.
    // Opcode: IfNot P1 P2 P3 * *
    //
    // Jump to P2 if the value in register P1 is False.  The value
    // is considered false if it has a numeric value of zero.  If the value
    // in P1 is NULL then take the jump if and only if P3 is non-zero.
    // Opcode: IsNull P1 P2 * * *
    // Synopsis: if r[P1]==NULL goto P2
    //
    // Jump to P2 if the value in register P1 is NULL.
    // Opcode: IsType P1 P2 P3 P4 P5
    // Synopsis: if typeof(P1.P3) in P5 goto P2
    //
    // Jump to P2 if the type of a column in a btree is one of the types specified
    // by the P5 bitmask.
    //
    // P1 is normally a cursor on a btree for which the row decode cache is
    // valid through at least column P3.  In other words, there should have been
    // a prior OP_Column for column P3 or greater.  If the cursor is not valid,
    // then this opcode might give spurious results.
    // The the btree row has fewer than P3 columns, then use P4 as the
    // datatype.
    //
    // If P1 is -1, then P3 is a register number and the datatype is taken
    // from the value in that register.
    //
    // P5 is a bitmask of data types.  SQLITE_INTEGER is the least significant
    // (0x01) bit. SQLITE_FLOAT is the 0x02 bit. SQLITE_TEXT is 0x04.
    // SQLITE_BLOB is 0x08.  SQLITE_NULL is 0x10.
    //
    // WARNING: This opcode does not reliably distinguish between NULL and REAL
    // when P1>=0.  If the database contains a NaN value, this opcode will think
    // that the datatype is REAL when it should be NULL.  When P1<0 and the value
    // is already stored in register P3, then this opcode does reliably
    // distinguish between NULL and REAL.  The problem only arises then P1>=0.
    //
    // Take the jump to address P2 if and only if the datatype of the
    // value determined by P1 and P3 corresponds to one of the bits in the
    // P5 bitmask.
    // Opcode: ZeroOrNull P1 P2 P3 * *
    // Synopsis: r[P2] = 0 OR NULL
    //
    // If both registers P1 and P3 are NOT NULL, then store a zero in
    // register P2.  If either registers P1 or P3 are NULL then put
    // a NULL in register P2.
    // Opcode: NotNull P1 P2 * * *
    // Synopsis: if r[P1]!=NULL goto P2
    //
    // Jump to P2 if the value in register P1 is not NULL.
    // Opcode: IfNullRow P1 P2 P3 * *
    // Synopsis: if P1.nullRow then r[P3]=NULL, goto P2
    //
    // Check the cursor P1 to see if it is currently pointing at a NULL row.
    // If it is, then set register P3 to NULL and jump immediately to P2.
    // If P1 is not on a NULL row, then fall through without making any
    // changes.
    //
    // If P1 is not an open cursor, then this opcode is a no-op.
    // Opcode: Offset P1 P2 P3 * *
    // Synopsis: r[P3] = sqlite_offset(P1)
    //
    // Store in register r[P3] the byte offset into the database file that is the
    // start of the payload for the record at which that cursor P1 is currently
    // pointing.
    //
    // P2 is the column number for the argument to the sqlite_offset() function.
    // This opcode does not use P2 itself, but the P2 value is used by the
    // code generator.  The P1, P2, and P3 operands to this opcode are the
    // same as for OP_Column.
    //
    // This opcode is only available if SQLite is compiled with the
    // -DSQLITE_ENABLE_OFFSET_SQL_FUNC option.
    // Opcode: Column P1 P2 P3 P4 P5
    // Synopsis: r[P3]=PX cursor P1 column P2
    //
    // Interpret the data that cursor P1 points to as a structure built using
    // the MakeRecord instruction.  (See the MakeRecord opcode for additional
    // information about the format of the data.)  Extract the P2-th column
    // from this record.  If there are less than (P2+1)
    // values in the record, extract a NULL.
    //
    // The value extracted is stored in register P3.
    //
    // If the record contains fewer than P2 fields, then extract a NULL.  Or,
    // if the P4 argument is a P4_MEM use the value of the P4 argument as
    // the result.
    //
    // If the OPFLAG_LENGTHARG bit is set in P5 then the result is guaranteed
    // to only be used by the length() function or the equivalent.  The content
    // of large blobs is not loaded, thus saving CPU cycles.  If the
    // OPFLAG_TYPEOFARG bit is set then the result will only be used by the
    // typeof() function or the IS NULL or IS NOT NULL operators or the
    // equivalent.  In this case, all content loading can be omitted.
    // Opcode: TypeCheck P1 P2 P3 P4 *
    // Synopsis: typecheck(r[P1@P2])
    //
    // Apply affinities to the range of P2 registers beginning with P1.
    // Take the affinities from the Table object in P4.  If any value
    // cannot be coerced into the correct type, then raise an error.
    //
    // If P3==0, then omit checking of VIRTUAL columns.
    //
    // If P3==1, then omit checking of all generated column, both VIRTUAL
    // and STORED.
    //
    // If P3>=2, then only check column number P3-2 in the table (which will
    // be a VIRTUAL column) against the value in reg[P1].  In this case,
    // P2 will be 1.
    //
    // This opcode is similar to OP_Affinity except that this opcode
    // forces the register type to the Table column type.  This is used
    // to implement "strict affinity".
    //
    // GENERATED ALWAYS AS ... STATIC columns are only checked if P3
    // is zero.  When P3 is non-zero, no type checking occurs for
    // static generated columns.  Virtual columns are computed at query time
    // and so they are never checked.
    //
    // Preconditions:
    //
    // <ul>
    // <li> P2 should be the number of non-virtual columns in the
    //      table of P4 unless P3>1, in which case P2 will be 1.
    // <li> Table P4 is a STRICT table.
    // </ul>
    //
    // If any precondition is false, an assertion fault occurs.
    // Opcode: Affinity P1 P2 * P4 *
    // Synopsis: affinity(r[P1@P2])
    //
    // Apply affinities to a range of P2 registers starting with P1.
    //
    // P4 is a string that is P2 characters long. The N-th character of the
    // string indicates the column affinity that should be used for the N-th
    // memory cell in the range.
    // Opcode: MakeRecord P1 P2 P3 P4 *
    // Synopsis: r[P3]=mkrec(r[P1@P2])
    //
    // Convert P2 registers beginning with P1 into the [record format]
    // use as a data record in a database table or as a key
    // in an index.  The OP_Column opcode can decode the record later.
    //
    // P4 may be a string that is P2 characters long.  The N-th character of the
    // string indicates the column affinity that should be used for the N-th
    // field of the index key.
    //
    // The mapping from character to affinity is given by the SQLITE_AFF_
    // macros defined in sqliteInt.h.
    //
    // If P4 is NULL then all index fields have the affinity BLOB.
    //
    // The meaning of P5 depends on whether or not the SQLITE_ENABLE_NULL_TRIM
    // compile-time option is enabled:
    //
    //   * If SQLITE_ENABLE_NULL_TRIM is enabled, then the P5 is the index
    //     of the right-most table that can be null-trimmed.
    //
    //   * If SQLITE_ENABLE_NULL_TRIM is omitted, then P5 has the value
    //     OPFLAG_NOCHNG_MAGIC if the OP_MakeRecord opcode is allowed to
    //     accept no-change records with serial_type 10.  This value is
    //     only used inside an assert() and does not affect the end result.
    // Opcode: Count P1 P2 P3 * *
    // Synopsis: r[P2]=count()
    //
    // Store the number of entries (an integer value) in the table or index
    // opened by cursor P1 in register P2.
    //
    // If P3==0, then an exact count is obtained, which involves visiting
    // every btree page of the table.  But if P3 is non-zero, an estimate
    // is returned based on the current cursor position.
    // Opcode: Savepoint P1 * * P4 *
    //
    // Open, release or rollback the savepoint named by parameter P4, depending
    // on the value of P1. To open a new savepoint set P1==0 (SAVEPOINT_BEGIN).
    // To release (commit) an existing savepoint set P1==1 (SAVEPOINT_RELEASE).
    // To rollback an existing savepoint set P1==2 (SAVEPOINT_ROLLBACK).
    // Opcode: AutoCommit P1 P2 * * *
    //
    // Set the database auto-commit flag to P1 (1 or 0). If P2 is true, roll
    // back any currently active btree transactions. If there are any active
    // VMs (apart from this one), then a ROLLBACK fails.  A COMMIT fails if
    // there are active writing VMs or active VMs that use shared cache.
    //
    // This instruction causes the VM to halt.
    // Opcode: Transaction P1 P2 P3 P4 P5
    //
    // Begin a transaction on database P1 if a transaction is not already
    // active.
    // If P2 is non-zero, then a write-transaction is started, or if a
    // read-transaction is already active, it is upgraded to a write-transaction.
    // If P2 is zero, then a read-transaction is started.  If P2 is 2 or more
    // then an exclusive transaction is started.
    //
    // P1 is the index of the database file on which the transaction is
    // started.  Index 0 is the main database file and index 1 is the
    // file used for temporary tables.  Indices of 2 or more are used for
    // attached databases.
    //
    // If a write-transaction is started and the Vdbe.usesStmtJournal flag is
    // true (this flag is set if the Vdbe may modify more than one row and may
    // throw an ABORT exception), a statement transaction may also be opened.
    // More specifically, a statement transaction is opened iff the database
    // connection is currently not in autocommit mode, or if there are other
    // active statements. A statement transaction allows the changes made by this
    // VDBE to be rolled back after an error without having to roll back the
    // entire transaction. If no error is encountered, the statement transaction
    // will automatically commit when the VDBE halts.
    //
    // If P5!=0 then this opcode also checks the schema cookie against P3
    // and the schema generation counter against P4.
    // The cookie changes its value whenever the database schema changes.
    // This operation is used to detect when that the cookie has changed
    // and that the current process needs to reread the schema.  If the schema
    // cookie in P3 differs from the schema cookie in the database header or
    // if the schema generation counter in P4 differs from the current
    // generation counter, then an SQLITE_SCHEMA error is raised and execution
    // halts.  The sqlite3_step() wrapper function might then reprepare the
    // statement and rerun it from the beginning.
    // Opcode: ReadCookie P1 P2 P3 * *
    //
    // Read cookie number P3 from database P1 and write it into register P2.
    // P3==1 is the schema version.  P3==2 is the database format.
    // P3==3 is the recommended pager cache size, and so forth.  P1==0 is
    // the main database file and P1==1 is the database file used to store
    // temporary tables.
    //
    // There must be a read-lock on the database (either a transaction
    // must be started or there must be an open cursor) before
    // executing this instruction.
    // Opcode: SetCookie P1 P2 P3 * P5
    //
    // Write the integer value P3 into cookie number P2 of database P1.
    // P2==1 is the schema version.  P2==2 is the database format.
    // P2==3 is the recommended pager cache
    // size, and so forth.  P1==0 is the main database file and P1==1 is the
    // database file used to store temporary tables.
    //
    // A transaction must be started before executing this opcode.
    //
    // If P2 is the SCHEMA_VERSION cookie (cookie number 1) then the internal
    // schema version is set to P3-P5.  The "PRAGMA schema_version=N" statement
    // has P5 set to 1, so that the internal schema version will be different
    // from the database schema version, resulting in a schema reset.
    // Opcode: OpenRead P1 P2 P3 P4 P5
    // Synopsis: root=P2 iDb=P3
    //
    // Open a read-only cursor for the database table whose root page is
    // P2 in a database file.  The database file is determined by P3.
    // P3==0 means the main database, P3==1 means the database used for
    // temporary tables, and P3>1 means used the corresponding attached
    // database.  Give the new cursor an identifier of P1.  The P1
    // values need not be contiguous but all P1 values should be small integers.
    // It is an error for P1 to be negative.
    //
    // Allowed P5 bits:
    // <ul>
    // <li>  <b>0x02 OPFLAG_SEEKEQ</b>: This cursor will only be used for
    //       equality lookups (implemented as a pair of opcodes OP_SeekGE/OP_IdxGT
    //       of OP_SeekLE/OP_IdxLT)
    // </ul>
    //
    // The P4 value may be either an integer (P4_INT32) or a pointer to
    // a KeyInfo structure (P4_KEYINFO). If it is a pointer to a KeyInfo
    // object, then table being opened must be an [index b-tree] where the
    // KeyInfo object defines the content and collating
    // sequence of that index b-tree. Otherwise, if P4 is an integer
    // value, then the table being opened must be a [table b-tree] with a
    // number of columns no less than the value of P4.
    //
    // See also: OpenWrite, ReopenIdx
    //
    // Opcode: ReopenIdx P1 P2 P3 P4 P5
    // Synopsis: root=P2 iDb=P3
    //
    // The ReopenIdx opcode works like OP_OpenRead except that it first
    // checks to see if the cursor on P1 is already open on the same
    // b-tree and if it is this opcode becomes a no-op.  In other words,
    // if the cursor is already open, do not reopen it.
    //
    // The ReopenIdx opcode may only be used with P5==0 or P5==OPFLAG_SEEKEQ
    // and with P4 being a P4_KEYINFO object.  Furthermore, the P3 value must
    // be the same as every other ReopenIdx or OpenRead for the same cursor
    // number.
    //
    // Allowed P5 bits:
    // <ul>
    // <li>  <b>0x02 OPFLAG_SEEKEQ</b>: This cursor will only be used for
    //       equality lookups (implemented as a pair of opcodes OP_SeekGE/OP_IdxGT
    //       of OP_SeekLE/OP_IdxLT)
    // </ul>
    //
    // See also: OP_OpenRead, OP_OpenWrite
    //
    // Opcode: OpenWrite P1 P2 P3 P4 P5
    // Synopsis: root=P2 iDb=P3
    //
    // Open a read/write cursor named P1 on the table or index whose root
    // page is P2 (or whose root page is held in register P2 if the
    // OPFLAG_P2ISREG bit is set in P5 - see below).
    //
    // The P4 value may be either an integer (P4_INT32) or a pointer to
    // a KeyInfo structure (P4_KEYINFO). If it is a pointer to a KeyInfo
    // object, then table being opened must be an [index b-tree] where the
    // KeyInfo object defines the content and collating
    // sequence of that index b-tree. Otherwise, if P4 is an integer
    // value, then the table being opened must be a [table b-tree] with a
    // number of columns no less than the value of P4.
    //
    // Allowed P5 bits:
    // <ul>
    // <li>  <b>0x02 OPFLAG_SEEKEQ</b>: This cursor will only be used for
    //       equality lookups (implemented as a pair of opcodes OP_SeekGE/OP_IdxGT
    //       of OP_SeekLE/OP_IdxLT)
    // <li>  <b>0x08 OPFLAG_FORDELETE</b>: This cursor is used only to seek
    //       and subsequently delete entries in an index btree.  This is a
    //       hint to the storage engine that the storage engine is allowed to
    //       ignore.  The hint is not used by the official SQLite b*tree storage
    //       engine, but is used by COMDB2.
    // <li>  <b>0x10 OPFLAG_P2ISREG</b>: Use the content of register P2
    //       as the root page, not the value of P2 itself.
    // </ul>
    //
    // This instruction works like OpenRead except that it opens the cursor
    // in read/write mode.
    //
    // See also: OP_OpenRead, OP_ReopenIdx
    // Opcode: OpenDup P1 P2 * * *
    //
    // Open a new cursor P1 that points to the same ephemeral table as
    // cursor P2.  The P2 cursor must have been opened by a prior OP_OpenEphemeral
    // opcode.  Only ephemeral cursors may be duplicated.
    //
    // Duplicate ephemeral cursors are used for self-joins of materialized views.
    // Opcode: OpenEphemeral P1 P2 P3 P4 P5
    // Synopsis: nColumn=P2
    //
    // Open a new cursor P1 to a transient table.
    // The cursor is always opened read/write even if
    // the main database is read-only.  The ephemeral
    // table is deleted automatically when the cursor is closed.
    //
    // If the cursor P1 is already opened on an ephemeral table, the table
    // is cleared (all content is erased).
    //
    // P2 is the number of columns in the ephemeral table.
    // The cursor points to a BTree table if P4==0 and to a BTree index
    // if P4 is not 0.  If P4 is not NULL, it points to a KeyInfo structure
    // that defines the format of keys in the index.
    //
    // The P5 parameter can be a mask of the BTREE_* flags defined
    // in btree.h.  These flags control aspects of the operation of
    // the btree.  The BTREE_OMIT_JOURNAL and BTREE_SINGLE flags are
    // added automatically.
    //
    // If P3 is positive, then reg[P3] is modified slightly so that it
    // can be used as zero-length data for OP_Insert.  This is an optimization
    // that avoids an extra OP_Blob opcode to initialize that register.
    //
    // Opcode: OpenAutoindex P1 P2 * P4 *
    // Synopsis: nColumn=P2
    //
    // This opcode works the same as OP_OpenEphemeral.  It has a
    // different name to distinguish its use.  Tables created using
    // by this opcode will be used for automatically created transient
    // indices in joins.
    // ncycle
    // Opcode: SorterOpen P1 P2 P3 P4 *
    //
    // This opcode works like OP_OpenEphemeral except that it opens
    // a transient index that is specifically designed to sort large
    // tables using an external merge-sort algorithm.
    //
    // If argument P3 is non-zero, then it indicates that the sorter may
    // assume that a stable sort considering the first P3 fields of each
    // key is sufficient to produce the required results.
    // Opcode: SequenceTest P1 P2 * * *
    // Synopsis: if( cursor[P1].ctr++ ) pc = P2
    //
    // P1 is a sorter cursor. If the sequence counter is currently zero, jump
    // to P2. Regardless of whether or not the jump is taken, increment the
    // the sequence value.
    // Opcode: OpenPseudo P1 P2 P3 * *
    // Synopsis: P3 columns in r[P2]
    //
    // Open a new cursor that points to a fake table that contains a single
    // row of data.  The content of that one row is the content of memory
    // register P2.  In other words, cursor P1 becomes an alias for the
    // MEM_Blob content contained in register P2.
    //
    // A pseudo-table created by this opcode is used to hold a single
    // row output from the sorter so that the row can be decomposed into
    // individual columns using the OP_Column opcode.  The OP_Column opcode
    // is the only cursor opcode that works with a pseudo-table.
    //
    // P3 is the number of fields in the records that will be stored by
    // the pseudo-table.  If P2 is 0 or negative then the pseudo-cursor
    // will return NULL for every column.
    // Opcode: Close P1 * * * *
    //
    // Close a cursor previously opened as P1.  If P1 is not
    // currently open, this instruction is a no-op.
    // Opcode: SeekGE P1 P2 P3 P4 *
    // Synopsis: key=r[P3@P4]
    //
    // If cursor P1 refers to an SQL table (B-Tree that uses integer keys),
    // use the value in register P3 as the key.  If cursor P1 refers
    // to an SQL index, then P3 is the first in an array of P4 registers
    // that are used as an unpacked index key.
    //
    // Reposition cursor P1 so that  it points to the smallest entry that
    // is greater than or equal to the key value. If there are no records
    // greater than or equal to the key and P2 is not zero, then jump to P2.
    //
    // If the cursor P1 was opened using the OPFLAG_SEEKEQ flag, then this
    // opcode will either land on a record that exactly matches the key, or
    // else it will cause a jump to P2.  When the cursor is OPFLAG_SEEKEQ,
    // this opcode must be followed by an IdxLE opcode with the same arguments.
    // The IdxGT opcode will be skipped if this opcode succeeds, but the
    // IdxGT opcode will be used on subsequent loop iterations.  The
    // OPFLAG_SEEKEQ flags is a hint to the btree layer to say that this
    // is an equality search.
    //
    // This opcode leaves the cursor configured to move in forward order,
    // from the beginning toward the end.  In other words, the cursor is
    // configured to use Next, not Prev.
    //
    // See also: Found, NotFound, SeekLt, SeekGt, SeekLe
    //
    // Opcode: SeekGT P1 P2 P3 P4 *
    // Synopsis: key=r[P3@P4]
    //
    // If cursor P1 refers to an SQL table (B-Tree that uses integer keys),
    // use the value in register P3 as a key. If cursor P1 refers
    // to an SQL index, then P3 is the first in an array of P4 registers
    // that are used as an unpacked index key.
    //
    // Reposition cursor P1 so that it points to the smallest entry that
    // is greater than the key value. If there are no records greater than
    // the key and P2 is not zero, then jump to P2.
    //
    // This opcode leaves the cursor configured to move in forward order,
    // from the beginning toward the end.  In other words, the cursor is
    // configured to use Next, not Prev.
    //
    // See also: Found, NotFound, SeekLt, SeekGe, SeekLe
    //
    // Opcode: SeekLT P1 P2 P3 P4 *
    // Synopsis: key=r[P3@P4]
    //
    // If cursor P1 refers to an SQL table (B-Tree that uses integer keys),
    // use the value in register P3 as a key. If cursor P1 refers
    // to an SQL index, then P3 is the first in an array of P4 registers
    // that are used as an unpacked index key.
    //
    // Reposition cursor P1 so that  it points to the largest entry that
    // is less than the key value. If there are no records less than
    // the key and P2 is not zero, then jump to P2.
    //
    // This opcode leaves the cursor configured to move in reverse order,
    // from the end toward the beginning.  In other words, the cursor is
    // configured to use Prev, not Next.
    //
    // See also: Found, NotFound, SeekGt, SeekGe, SeekLe
    //
    // Opcode: SeekLE P1 P2 P3 P4 *
    // Synopsis: key=r[P3@P4]
    //
    // If cursor P1 refers to an SQL table (B-Tree that uses integer keys),
    // use the value in register P3 as a key. If cursor P1 refers
    // to an SQL index, then P3 is the first in an array of P4 registers
    // that are used as an unpacked index key.
    //
    // Reposition cursor P1 so that it points to the largest entry that
    // is less than or equal to the key value. If there are no records
    // less than or equal to the key and P2 is not zero, then jump to P2.
    //
    // This opcode leaves the cursor configured to move in reverse order,
    // from the end toward the beginning.  In other words, the cursor is
    // configured to use Prev, not Next.
    //
    // If the cursor P1 was opened using the OPFLAG_SEEKEQ flag, then this
    // opcode will either land on a record that exactly matches the key, or
    // else it will cause a jump to P2.  When the cursor is OPFLAG_SEEKEQ,
    // this opcode must be followed by an IdxLE opcode with the same arguments.
    // The IdxGE opcode will be skipped if this opcode succeeds, but the
    // IdxGE opcode will be used on subsequent loop iterations.  The
    // OPFLAG_SEEKEQ flags is a hint to the btree layer to say that this
    // is an equality search.
    //
    // See also: Found, NotFound, SeekGt, SeekGe, SeekLt
    // jump0, in3, group, ncycle
    // jump0, in3, group, ncycle
    // jump0, in3, group, ncycle
    // Opcode: SeekScan  P1 P2 * * P5
    // Synopsis: Scan-ahead up to P1 rows
    //
    // This opcode is a prefix opcode to OP_SeekGE.  In other words, this
    // opcode must be immediately followed by OP_SeekGE. This constraint is
    // checked by assert() statements.
    //
    // This opcode uses the P1 through P4 operands of the subsequent
    // OP_SeekGE.  In the text that follows, the operands of the subsequent
    // OP_SeekGE opcode are denoted as SeekOP.P1 through SeekOP.P4.   Only
    // the P1, P2 and P5 operands of this opcode are also used, and  are called
    // This.P1, This.P2 and This.P5.
    //
    // This opcode helps to optimize IN operators on a multi-column index
    // where the IN operator is on the later terms of the index by avoiding
    // unnecessary seeks on the btree, substituting steps to the next row
    // of the b-tree instead.  A correct answer is obtained if this opcode
    // is omitted or is a no-op.
    //
    // The SeekGE.P3 and SeekGE.P4 operands identify an unpacked key which
    // is the desired entry that we want the cursor SeekGE.P1 to be pointing
    // to.  Call this SeekGE.P3/P4 row the "target".
    //
    // If the SeekGE.P1 cursor is not currently pointing to a valid row,
    // then this opcode is a no-op and control passes through into the OP_SeekGE.
    //
    // If the SeekGE.P1 cursor is pointing to a valid row, then that row
    // might be the target row, or it might be near and slightly before the
    // target row, or it might be after the target row.  If the cursor is
    // currently before the target row, then this opcode attempts to position
    // the cursor on or after the target row by invoking sqlite3BtreeStep()
    // on the cursor between 1 and This.P1 times.
    //
    // The This.P5 parameter is a flag that indicates what to do if the
    // cursor ends up pointing at a valid row that is past the target
    // row.  If This.P5 is false (0) then a jump is made to SeekGE.P2.  If
    // This.P5 is true (non-zero) then a jump is made to This.P2.  The P5==0
    // case occurs when there are no inequality constraints to the right of
    // the IN constraint.  The jump to SeekGE.P2 ends the loop.  The P5!=0 case
    // occurs when there are inequality constraints to the right of the IN
    // operator.  In that case, the This.P2 will point either directly to or
    // to setup code prior to the OP_IdxGT or OP_IdxGE opcode that checks for
    // loop terminate.
    //
    // Possible outcomes from this opcode:<ol>
    //
    // <li> If the cursor is initially not pointed to any valid row, then
    //      fall through into the subsequent OP_SeekGE opcode.
    //
    // <li> If the cursor is left pointing to a row that is before the target
    //      row, even after making as many as This.P1 calls to
    //      sqlite3BtreeNext(), then also fall through into OP_SeekGE.
    //
    // <li> If the cursor is left pointing at the target row, either because it
    //      was at the target row to begin with or because one or more
    //      sqlite3BtreeNext() calls moved the cursor to the target row,
    //      then jump to This.P2..,
    //
    // <li> If the cursor started out before the target row and a call to
    //      to sqlite3BtreeNext() moved the cursor off the end of the index
    //      (indicating that the target row definitely does not exist in the
    //      btree) then jump to SeekGE.P2, ending the loop.
    //
    // <li> If the cursor ends up on a valid row that is past the target row
    //      (indicating that the target row does not exist in the btree) then
    //      jump to SeekOP.P2 if This.P5==0 or to This.P2 if This.P5>0.
    // </ol>
    // Opcode: SeekHit P1 P2 P3 * *
    // Synopsis: set P2<=seekHit<=P3
    //
    // Increase or decrease the seekHit value for cursor P1, if necessary,
    // so that it is no less than P2 and no greater than P3.
    //
    // The seekHit integer represents the maximum of terms in an index for which
    // there is known to be at least one match.  If the seekHit value is smaller
    // than the total number of equality terms in an index lookup, then the
    // OP_IfNoHope opcode might run to see if the IN loop can be abandoned
    // early, thus saving work.  This is part of the IN-early-out optimization.
    //
    // P1 must be a valid b-tree cursor.
    // Opcode: IfNotOpen P1 P2 * * *
    // Synopsis: if( !csr[P1] ) goto P2
    //
    // If cursor P1 is not open or if P1 is set to a NULL row using the
    // OP_NullRow opcode, then jump to instruction P2. Otherwise, fall through.
    // Opcode: Found P1 P2 P3 P4 *
    // Synopsis: key=r[P3@P4]
    //
    // If P4==0 then register P3 holds a blob constructed by MakeRecord.  If
    // P4>0 then register P3 is the first of P4 registers that form an unpacked
    // record.
    //
    // Cursor P1 is on an index btree.  If the record identified by P3 and P4
    // is a prefix of any entry in P1 then a jump is made to P2 and
    // P1 is left pointing at the matching entry.
    //
    // This operation leaves the cursor in a state where it can be
    // advanced in the forward direction.  The Next instruction will work,
    // but not the Prev instruction.
    //
    // See also: NotFound, NoConflict, NotExists. SeekGe
    //
    // Opcode: NotFound P1 P2 P3 P4 *
    // Synopsis: key=r[P3@P4]
    //
    // If P4==0 then register P3 holds a blob constructed by MakeRecord.  If
    // P4>0 then register P3 is the first of P4 registers that form an unpacked
    // record.
    //
    // Cursor P1 is on an index btree.  If the record identified by P3 and P4
    // is not the prefix of any entry in P1 then a jump is made to P2.  If P1
    // does contain an entry whose prefix matches the P3/P4 record then control
    // falls through to the next instruction and P1 is left pointing at the
    // matching entry.
    //
    // This operation leaves the cursor in a state where it cannot be
    // advanced in either direction.  In other words, the Next and Prev
    // opcodes do not work after this operation.
    //
    // See also: Found, NotExists, NoConflict, IfNoHope
    //
    // Opcode: IfNoHope P1 P2 P3 P4 *
    // Synopsis: key=r[P3@P4]
    //
    // Register P3 is the first of P4 registers that form an unpacked
    // record.  Cursor P1 is an index btree.  P2 is a jump destination.
    // In other words, the operands to this opcode are the same as the
    // operands to OP_NotFound and OP_IdxGT.
    //
    // This opcode is an optimization attempt only.  If this opcode always
    // falls through, the correct answer is still obtained, but extra work
    // is performed.
    //
    // A value of N in the seekHit flag of cursor P1 means that there exists
    // a key P3:N that will match some record in the index.  We want to know
    // if it is possible for a record P3:P4 to match some record in the
    // index.  If it is not possible, we can skip some work.  So if seekHit
    // is less than P4, attempt to find out if a match is possible by running
    // OP_NotFound.
    //
    // This opcode is used in IN clause processing for a multi-column key.
    // If an IN clause is attached to an element of the key other than the
    // left-most element, and if there are no matches on the most recent
    // seek over the whole key, then it might be that one of the key element
    // to the left is prohibiting a match, and hence there is "no hope" of
    // any match regardless of how many IN clause elements are checked.
    // In such a case, we abandon the IN clause search early, using this
    // opcode.  The opcode name comes from the fact that the
    // jump is taken if there is "no hope" of achieving a match.
    //
    // See also: NotFound, SeekHit
    //
    // Opcode: NoConflict P1 P2 P3 P4 *
    // Synopsis: key=r[P3@P4]
    //
    // If P4==0 then register P3 holds a blob constructed by MakeRecord.  If
    // P4>0 then register P3 is the first of P4 registers that form an unpacked
    // record.
    //
    // Cursor P1 is on an index btree.  If the record identified by P3 and P4
    // contains any NULL value, jump immediately to P2.  If all terms of the
    // record are not-NULL then a check is done to determine if any row in the
    // P1 index btree has a matching key prefix.  If there are no matches, jump
    // immediately to P2.  If there is a match, fall through and leave the P1
    // cursor pointing to the matching row.
    //
    // This opcode is similar to OP_NotFound with the exceptions that the
    // branch is always taken if any part of the search key input is NULL.
    //
    // This operation leaves the cursor in a state where it cannot be
    // advanced in either direction.  In other words, the Next and Prev
    // opcodes do not work after this operation.
    //
    // See also: NotFound, Found, NotExists
    // jump, in3, ncycle
    // jump, in3, ncycle
    // Opcode: SeekRowid P1 P2 P3 * *
    // Synopsis: intkey=r[P3]
    //
    // P1 is the index of a cursor open on an SQL table btree (with integer
    // keys).  If register P3 does not contain an integer or if P1 does not
    // contain a record with rowid P3 then jump immediately to P2.
    // Or, if P2 is 0, raise an SQLITE_CORRUPT error. If P1 does contain
    // a record with rowid P3 then
    // leave the cursor pointing at that record and fall through to the next
    // instruction.
    //
    // The OP_NotExists opcode performs the same operation, but with OP_NotExists
    // the P3 register must be guaranteed to contain an integer value.  With this
    // opcode, register P3 might not contain an integer.
    //
    // The OP_NotFound opcode performs the same operation on index btrees
    // (with arbitrary multi-value keys).
    //
    // This opcode leaves the cursor in a state where it cannot be advanced
    // in either direction.  In other words, the Next and Prev opcodes will
    // not work following this opcode.
    //
    // See also: Found, NotFound, NoConflict, SeekRowid
    //
    // Opcode: NotExists P1 P2 P3 * *
    // Synopsis: intkey=r[P3]
    //
    // P1 is the index of a cursor open on an SQL table btree (with integer
    // keys).  P3 is an integer rowid.  If P1 does not contain a record with
    // rowid P3 then jump immediately to P2.  Or, if P2 is 0, raise an
    // SQLITE_CORRUPT error. If P1 does contain a record with rowid P3 then
    // leave the cursor pointing at that record and fall through to the next
    // instruction.
    //
    // The OP_SeekRowid opcode performs the same operation but also allows the
    // P3 register to contain a non-integer value, in which case the jump is
    // always taken.  This opcode requires that P3 always contain an integer.
    //
    // The OP_NotFound opcode performs the same operation on index btrees
    // (with arbitrary multi-value keys).
    //
    // This opcode leaves the cursor in a state where it cannot be advanced
    // in either direction.  In other words, the Next and Prev opcodes will
    // not work following this opcode.
    //
    // See also: Found, NotFound, NoConflict, SeekRowid
    // Opcode: Sequence P1 P2 * * *
    // Synopsis: r[P2]=cursor[P1].ctr++
    //
    // Find the next available sequence number for cursor P1.
    // Write the sequence number into register P2.
    // The sequence number on the cursor is incremented after this
    // instruction.
    // Opcode: NewRowid P1 P2 P3 * *
    // Synopsis: r[P2]=rowid
    //
    // Get a new integer record number (a.k.a "rowid") used as the key to a table.
    // The record number is not previously used as a key in the database
    // table that cursor P1 points to.  The new record number is written
    // written to register P2.
    //
    // If P3>0 then P3 is a register in the root frame of this VDBE that holds
    // the largest previously generated record number. No new record numbers are
    // allowed to be less than this value. When this value reaches its maximum,
    // an SQLITE_FULL error is generated. The P3 register is updated with the '
    // generated record number. This P3 mechanism is used to help implement the
    // AUTOINCREMENT feature.
    // Opcode: Insert P1 P2 P3 P4 P5
    // Synopsis: intkey=r[P3] data=r[P2]
    //
    // Write an entry into the table of cursor P1.  A new entry is
    // created if it doesn't already exist or the data for an existing
    // entry is overwritten.  The data is the value MEM_Blob stored in register
    // number P2. The key is stored in register P3. The key must
    // be a MEM_Int.
    //
    // If the OPFLAG_NCHANGE flag of P5 is set, then the row change count is
    // incremented (otherwise not).  If the OPFLAG_LASTROWID flag of P5 is set,
    // then rowid is stored for subsequent return by the
    // sqlite3_last_insert_rowid() function (otherwise it is unmodified).
    //
    // If the OPFLAG_USESEEKRESULT flag of P5 is set, the implementation might
    // run faster by avoiding an unnecessary seek on cursor P1.  However,
    // the OPFLAG_USESEEKRESULT flag must only be set if there have been no prior
    // seeks on the cursor or if the most recent seek used a key equal to P3.
    //
    // If the OPFLAG_ISUPDATE flag is set, then this opcode is part of an
    // UPDATE operation.  Otherwise (if the flag is clear) then this opcode
    // is part of an INSERT operation.  The difference is only important to
    // the update hook.
    //
    // Parameter P4 may point to a Table structure, or may be NULL. If it is
    // not NULL, then the update-hook (sqlite3.xUpdateCallback) is invoked
    // following a successful insert.
    //
    // (WARNING/TODO: If P1 is a pseudo-cursor and P2 is dynamically
    // allocated, then ownership of P2 is transferred to the pseudo-cursor
    // and register P2 becomes ephemeral.  If the cursor is changed, the
    // value of register P2 will then change.  Make sure this does not
    // cause any problems.)
    //
    // This instruction only works on tables.  The equivalent instruction
    // for indices is OP_IdxInsert.
    // Opcode: RowCell P1 P2 P3 * *
    //
    // P1 and P2 are both open cursors. Both must be opened on the same type
    // of table - intkey or index. This opcode is used as part of copying
    // the current row from P2 into P1. If the cursors are opened on intkey
    // tables, register P3 contains the rowid to use with the new record in
    // P1. If they are opened on index tables, P3 is not used.
    //
    // This opcode must be followed by either an Insert or InsertIdx opcode
    // with the OPFLAG_PREFORMAT flag set to complete the insert operation.
    // Opcode: Delete P1 P2 P3 P4 P5
    //
    // Delete the record at which the P1 cursor is currently pointing.
    //
    // If the OPFLAG_SAVEPOSITION bit of the P5 parameter is set, then
    // the cursor will be left pointing at  either the next or the previous
    // record in the table. If it is left pointing at the next record, then
    // the next Next instruction will be a no-op. As a result, in this case
    // it is ok to delete a record from within a Next loop. If
    // OPFLAG_SAVEPOSITION bit of P5 is clear, then the cursor will be
    // left in an undefined state.
    //
    // If the OPFLAG_AUXDELETE bit is set on P5, that indicates that this
    // delete is one of several associated with deleting a table row and
    // all its associated index entries.  Exactly one of those deletes is
    // the "primary" delete.  The others are all on OPFLAG_FORDELETE
    // cursors or else are marked with the AUXDELETE flag.
    //
    // If the OPFLAG_NCHANGE (0x01) flag of P2 (NB: P2 not P5) is set, then
    // the row change count is incremented (otherwise not).
    //
    // If the OPFLAG_ISNOOP (0x40) flag of P2 (not P5!) is set, then the
    // pre-update-hook for deletes is run, but the btree is otherwise unchanged.
    // This happens when the OP_Delete is to be shortly followed by an OP_Insert
    // with the same key, causing the btree entry to be overwritten.
    //
    // P1 must not be pseudo-table.  It has to be a real table with
    // multiple rows.
    //
    // If P4 is not NULL then it points to a Table object. In this case either
    // the update or pre-update hook, or both, may be invoked. The P1 cursor must
    // have been positioned using OP_NotFound prior to invoking this opcode in
    // this case. Specifically, if one is configured, the pre-update hook is
    // invoked if P4 is not NULL. The update-hook is invoked if one is configured,
    // P4 is not NULL, and the OPFLAG_NCHANGE flag is set in P2.
    //
    // If the OPFLAG_ISUPDATE flag is set in P2, then P3 contains the address
    // of the memory cell that contains the value that the rowid of the row will
    // be set to by the update.
    // Opcode: ResetCount * * * * *
    //
    // The value of the change counter is copied to the database handle
    // change counter (returned by subsequent calls to sqlite3_changes()).
    // Then the VMs internal change counter resets to 0.
    // This is used by trigger programs.
    // Opcode: SorterCompare P1 P2 P3 P4
    // Synopsis: if key(P1)!=trim(r[P3],P4) goto P2
    //
    // P1 is a sorter cursor. This instruction compares a prefix of the
    // record blob in register P3 against a prefix of the entry that
    // the sorter cursor currently points to.  Only the first P4 fields
    // of r[P3] and the sorter record are compared.
    //
    // If either P3 or the sorter contains a NULL in one of their significant
    // fields (not counting the P4 fields at the end which are ignored) then
    // the comparison is assumed to be equal.
    //
    // Fall through to next instruction if the two records compare equal to
    // each other.  Jump to P2 if they are different.
    // Opcode: SorterData P1 P2 P3 * *
    // Synopsis: r[P2]=data
    //
    // Write into register P2 the current sorter data for sorter cursor P1.
    // Then clear the column header cache on cursor P3.
    //
    // This opcode is normally used to move a record out of the sorter and into
    // a register that is the source for a pseudo-table cursor created using
    // OpenPseudo.  That pseudo-table cursor is the one that is identified by
    // parameter P3.  Clearing the P3 column cache as part of this opcode saves
    // us from having to issue a separate NullRow instruction to clear that cache.
    // Opcode: RowData P1 P2 P3 * *
    // Synopsis: r[P2]=data
    //
    // Write into register P2 the complete row content for the row at
    // which cursor P1 is currently pointing.
    // There is no interpretation of the data.
    // It is just copied onto the P2 register exactly as
    // it is found in the database file.
    //
    // If cursor P1 is an index, then the content is the key of the row.
    // If cursor P2 is a table, then the content extracted is the data.
    //
    // If the P1 cursor must be pointing to a valid row (not a NULL row)
    // of a real table, not a pseudo-table.
    //
    // If P3!=0 then this opcode is allowed to make an ephemeral pointer
    // into the database page.  That means that the content of the output
    // register will be invalidated as soon as the cursor moves - including
    // moves caused by other cursors that "save" the current cursors
    // position in order that they can write to the same table.  If P3==0
    // then a copy of the data is made into memory.  P3!=0 is faster, but
    // P3==0 is safer.
    //
    // If P3!=0 then the content of the P2 register is unsuitable for use
    // in OP_Result and any OP_Result will invalidate the P2 register content.
    // The P2 register content is invalidated by opcodes like OP_Function or
    // by any use of another cursor pointing to the same table.
    // Opcode: Rowid P1 P2 * * *
    // Synopsis: r[P2]=PX rowid of P1
    //
    // Store in register P2 an integer which is the key of the table entry that
    // P1 is currently point to.
    //
    // P1 can be either an ordinary table or a virtual table.  There used to
    // be a separate OP_VRowid opcode for use with virtual tables, but this
    // one opcode now works for both table types.
    // Opcode: NullRow P1 * * * *
    //
    // Move the cursor P1 to a null row.  Any OP_Column operations
    // that occur while the cursor is on the null row will always
    // write a NULL.
    //
    // If cursor P1 is not previously opened, open it now to a special
    // pseudo-cursor that always returns NULL for every column.
    // Opcode: SeekEnd P1 * * * *
    //
    // Position cursor P1 at the end of the btree for the purpose of
    // appending a new entry onto the btree.
    //
    // It is assumed that the cursor is used only for appending and so
    // if the cursor is valid, then the cursor must already be pointing
    // at the end of the btree and so no changes are made to
    // the cursor.
    //
    // Opcode: Last P1 P2 * * *
    //
    // The next use of the Rowid or Column or Prev instruction for P1
    // will refer to the last entry in the database table or index.
    // If the table or index is empty and P2>0, then jump immediately to P2.
    // If P2 is 0 or if the table or index is not empty, fall through
    // to the following instruction.
    //
    // This opcode leaves the cursor configured to move in reverse order,
    // from the end toward the beginning.  In other words, the cursor is
    // configured to use Prev, not Next.
    // ncycle
    // Opcode: IfSizeBetween P1 P2 P3 P4 *
    //
    // Let N be the approximate number of rows in the table or index
    // with cursor P1 and let X be 10*log2(N) if N is positive or -1
    // if N is zero.
    //
    // Jump to P2 if X is in between P3 and P4, inclusive.
    // Opcode: SorterSort P1 P2 * * *
    //
    // After all records have been inserted into the Sorter object
    // identified by P1, invoke this opcode to actually do the sorting.
    // Jump to P2 if there are no records to be sorted.
    //
    // This opcode is an alias for OP_Sort and OP_Rewind that is used
    // for Sorter objects.
    //
    // Opcode: Sort P1 P2 * * *
    //
    // This opcode does exactly the same thing as OP_Rewind except that
    // it increments an undocumented global variable used for testing.
    //
    // Sorting is accomplished by writing records into a sorting index,
    // then rewinding that index and playing it back from beginning to
    // end.  We use the OP_Sort opcode instead of OP_Rewind to do the
    // rewinding so that the global variable will be incremented and
    // regression tests can determine whether or not the optimizer is
    // correctly optimizing out sorts.
    // Opcode: IfEmpty P1 P2 * * *
    // Synopsis: if( empty(P1) ) goto P2
    //
    // Check to see if the b-tree table that cursor P1 references is empty
    // and jump to P2 if it is.
    // Opcode: Next P1 P2 P3 * P5
    //
    // Advance cursor P1 so that it points to the next key/data pair in its
    // table or index.  If there are no more key/value pairs then fall through
    // to the following instruction.  But if the cursor advance was successful,
    // jump immediately to P2.
    //
    // The Next opcode is only valid following an SeekGT, SeekGE, or
    // OP_Rewind opcode used to position the cursor.  Next is not allowed
    // to follow SeekLT, SeekLE, or OP_Last.
    //
    // The P1 cursor must be for a real table, not a pseudo-table.  P1 must have
    // been opened prior to this opcode or the program will segfault.
    //
    // The P3 value is a hint to the btree implementation. If P3==1, that
    // means P1 is an SQL index and that this instruction could have been
    // omitted if that index had been unique.  P3 is usually 0.  P3 is
    // always either 0 or 1.
    //
    // If P5 is positive and the jump is taken, then event counter
    // number P5-1 in the prepared statement is incremented.
    //
    // See also: Prev
    //
    // Opcode: Prev P1 P2 P3 * P5
    //
    // Back up cursor P1 so that it points to the previous key/data pair in its
    // table or index.  If there is no previous key/value pairs then fall through
    // to the following instruction.  But if the cursor backup was successful,
    // jump immediately to P2.
    //
    //
    // The Prev opcode is only valid following an SeekLT, SeekLE, or
    // OP_Last opcode used to position the cursor.  Prev is not allowed
    // to follow SeekGT, SeekGE, or OP_Rewind.
    //
    // The P1 cursor must be for a real table, not a pseudo-table.  If P1 is
    // not open then the behavior is undefined.
    //
    // The P3 value is a hint to the btree implementation. If P3==1, that
    // means P1 is an SQL index and that this instruction could have been
    // omitted if that index had been unique.  P3 is usually 0.  P3 is
    // always either 0 or 1.
    //
    // If P5 is positive and the jump is taken, then event counter
    // number P5-1 in the prepared statement is incremented.
    //
    // Opcode: SorterNext P1 P2 * * P5
    //
    // This opcode works just like OP_Next except that P1 must be a
    // sorter object for which the OP_SorterSort opcode has been
    // invoked.  This opcode advances the cursor to the next sorted
    // record, or jumps to P2 if there are no more sorted records.
    // Opcode: IdxInsert P1 P2 P3 P4 P5
    // Synopsis: key=r[P2]
    //
    // Register P2 holds an SQL index key made using the
    // MakeRecord instructions.  This opcode writes that key
    // into the index P1.  Data for the entry is nil.
    //
    // If P4 is not zero, then it is the number of values in the unpacked
    // key of reg(P2).  In that case, P3 is the index of the first register
    // for the unpacked key.  The availability of the unpacked key can sometimes
    // be an optimization.
    //
    // If P5 has the OPFLAG_APPEND bit set, that is a hint to the b-tree layer
    // that this insert is likely to be an append.
    //
    // If P5 has the OPFLAG_NCHANGE bit set, then the change counter is
    // incremented by this instruction.  If the OPFLAG_NCHANGE bit is clear,
    // then the change counter is unchanged.
    //
    // If the OPFLAG_USESEEKRESULT flag of P5 is set, the implementation might
    // run faster by avoiding an unnecessary seek on cursor P1.  However,
    // the OPFLAG_USESEEKRESULT flag must only be set if there have been no prior
    // seeks on the cursor or if the most recent seek used a key equivalent
    // to P2.
    //
    // This instruction only works for indices.  The equivalent instruction
    // for tables is OP_Insert.
    // Opcode: SorterInsert P1 P2 * * *
    // Synopsis: key=r[P2]
    //
    // Register P2 holds an SQL index key made using the
    // MakeRecord instructions.  This opcode writes that key
    // into the sorter P1.  Data for the entry is nil.
    // Opcode: IdxDelete P1 P2 P3 P4 P5
    // Synopsis: key=r[P2@P5]
    //
    // The content of P5 registers starting at register P2 form
    // an unpacked index key. This opcode removes that entry from the
    // index opened by cursor P1.
    //
    // P4 is a pointer to an Index structure.
    //
    // If P3 is non-zero, it is the register number of a register holding
    // a record that will be inserted into this index. If that record is
    // identical to the one that would be deleted by this instruction,
    // skip the delete and set register P3 to NULL.
    //
    // Raise an SQLITE_CORRUPT_INDEX error if no matching index entry is found
    // and not in writable_schema mode.
    // Opcode: DeferredSeek P1 * P3 P4 *
    // Synopsis: Move P3 to P1.rowid if needed
    //
    // P1 is an open index cursor and P3 is a cursor on the corresponding
    // table.  This opcode does a deferred seek of the P3 table cursor
    // to the row that corresponds to the current row of P1.
    //
    // This is a deferred seek.  Nothing actually happens until
    // the cursor is used to read a record.  That way, if no reads
    // occur, no unnecessary I/O happens.
    //
    // P4 may be an array of integers (type P4_INTARRAY) containing
    // one entry for each column in the P3 table.  If array entry a(i)
    // is non-zero, then reading column a(i)-1 from cursor P3 is
    // equivalent to performing the deferred seek and then reading column i
    // from P1.  This information is stored in P3 and used to redirect
    // reads against P3 over to P1, thus possibly avoiding the need to
    // seek and read cursor P3.
    //
    // Opcode: IdxRowid P1 P2 * * *
    // Synopsis: r[P2]=rowid
    //
    // Write into register P2 an integer which is the last entry in the record at
    // the end of the index key pointed to by cursor P1.  This integer should be
    // the rowid of the table entry to which this index entry points.
    //
    // See also: Rowid, MakeRecord.
    // ncycle
    // Opcode: FinishSeek P1 * * * *
    //
    // If cursor P1 was previously moved via OP_DeferredSeek, complete that
    // seek operation now, without further delay.  If the cursor seek has
    // already occurred, this instruction is a no-op.
    // Opcode: IdxGE P1 P2 P3 P4 *
    // Synopsis: key=r[P3@P4]
    //
    // The P4 register values beginning with P3 form an unpacked index
    // key that omits the PRIMARY KEY.  Compare this key value against the index
    // that P1 is currently pointing to, ignoring the PRIMARY KEY or ROWID
    // fields at the end.
    //
    // If the P1 index entry is greater than or equal to the key value
    // then jump to P2.  Otherwise fall through to the next instruction.
    //
    // Opcode: IdxGT P1 P2 P3 P4 *
    // Synopsis: key=r[P3@P4]
    //
    // The P4 register values beginning with P3 form an unpacked index
    // key that omits the PRIMARY KEY.  Compare this key value against the index
    // that P1 is currently pointing to, ignoring the PRIMARY KEY or ROWID
    // fields at the end.
    //
    // If the P1 index entry is greater than the key value
    // then jump to P2.  Otherwise fall through to the next instruction.
    //
    // Opcode: IdxLT P1 P2 P3 P4 *
    // Synopsis: key=r[P3@P4]
    //
    // The P4 register values beginning with P3 form an unpacked index
    // key that omits the PRIMARY KEY or ROWID.  Compare this key value against
    // the index that P1 is currently pointing to, ignoring the PRIMARY KEY or
    // ROWID on the P1 index.
    //
    // If the P1 index entry is less than the key value then jump to P2.
    // Otherwise fall through to the next instruction.
    //
    // Opcode: IdxLE P1 P2 P3 P4 *
    // Synopsis: key=r[P3@P4]
    //
    // The P4 register values beginning with P3 form an unpacked index
    // key that omits the PRIMARY KEY or ROWID.  Compare this key value against
    // the index that P1 is currently pointing to, ignoring the PRIMARY KEY or
    // ROWID on the P1 index.
    //
    // If the P1 index entry is less than or equal to the key value then jump
    // to P2. Otherwise fall through to the next instruction.
    // jump, ncycle
    // jump, ncycle
    // jump, ncycle
    // Opcode: Destroy P1 P2 P3 * *
    //
    // Delete an entire database table or index whose root page in the database
    // file is given by P1.
    //
    // The table being destroyed is in the main database file if P3==0.  If
    // P3==1 then the table to be destroyed is in the auxiliary database file
    // that is used to store tables create using CREATE TEMPORARY TABLE.
    //
    // If AUTOVACUUM is enabled then it is possible that another root page
    // might be moved into the newly deleted root page in order to keep all
    // root pages contiguous at the beginning of the database.  The former
    // value of the root page that moved - its value before the move occurred -
    // is stored in register P2. If no page movement was required (because the
    // table being dropped was already the last one in the database) then a
    // zero is stored in register P2.  If AUTOVACUUM is disabled then a zero
    // is stored in register P2.
    //
    // This opcode throws an error if there are any active reader VMs when
    // it is invoked. This is done to avoid the difficulty associated with
    // updating existing cursors when a root page is moved in an AUTOVACUUM
    // database. This error is thrown even if the database is not an AUTOVACUUM
    // db in order to avoid introducing an incompatibility between autovacuum
    // and non-autovacuum modes.
    //
    // See also: Clear
    // Opcode: Clear P1 P2 P3
    //
    // Delete all contents of the database table or index whose root page
    // in the database file is given by P1.  But, unlike Destroy, do not
    // remove the table or index from the database file.
    //
    // The table being cleared is in the main database file if P2==0.  If
    // P2==1 then the table to be cleared is in the auxiliary database file
    // that is used to store tables create using CREATE TEMPORARY TABLE.
    //
    // If the P3 value is non-zero, then the row change count is incremented
    // by the number of rows in the table being cleared. If P3 is greater
    // than zero, then the value stored in register P3 is also incremented
    // by the number of rows in the table being cleared.
    //
    // See also: Destroy
    // Opcode: ResetSorter P1 * * * *
    //
    // Delete all contents from the ephemeral table or sorter
    // that is open on cursor P1.
    //
    // This opcode only works for cursors used for sorting and
    // opened with OP_OpenEphemeral or OP_SorterOpen.
    // Opcode: CreateBtree P1 P2 P3 * *
    // Synopsis: r[P2]=root iDb=P1 flags=P3
    //
    // Allocate a new b-tree in the main database file if P1==0 or in the
    // TEMP database file if P1==1 or in an attached database if
    // P1>1.  The P3 argument must be 1 (BTREE_INTKEY) for a rowid table
    // it must be 2 (BTREE_BLOBKEY) for an index or WITHOUT ROWID table.
    // The root page number of the new b-tree is stored in register P2.
    // Opcode: SqlExec P1 P2 * P4 *
    //
    // Run the SQL statement or statements specified in the P4 string.
    //
    // The P1 parameter is a bitmask of options:
    //
    //    0x0001     Disable Auth and Trace callbacks while the statements
    //               in P4 are running.
    //
    //    0x0002     Set db->nAnalysisLimit to P2 while the statements in
    //               P4 are running.
    // Opcode: ParseSchema P1 * * P4 P5
    //
    // Read and parse all entries from the schema table of database P1
    // that match the WHERE clause P4.  If P4 is a NULL pointer, then the
    // entire schema for P1 is reparsed.
    //
    // When P4 is NULL, the P5 value is used as the mFlags argument
    // to sqlite3InitOne().  In other words, P5 should be a mask composed
    // of INITFLAG_* values.
    //
    // The P4==0 case is only used by ALTER TABLE and P5!=0 for all such
    // cases.  For uses other than ALTER TABLE, P4<>0 and P5==0.
    //
    // This opcode invokes the parser to create a new virtual machine,
    // then runs the new virtual machine.  It is thus a re-entrant opcode.
    // Opcode: LoadAnalysis P1 * * * *
    //
    // Read the sqlite_stat1 table for database P1 and load the content
    // of that table into the internal index hash table.  This will cause
    // the analysis to be used when preparing all subsequent queries.
    // Opcode: DropTable P1 * * P4 *
    //
    // Remove the internal (in-memory) data structures that describe
    // the table named P4 in database P1.  This is called after a table
    // is dropped from disk (using the Destroy opcode) in order to keep
    // the internal representation of the
    // schema consistent with what is on disk.
    // Opcode: DropIndex P1 * * P4 *
    //
    // Remove the internal (in-memory) data structures that describe
    // the index named P4 in database P1.  This is called after an index
    // is dropped from disk (using the Destroy opcode)
    // in order to keep the internal representation of the
    // schema consistent with what is on disk.
    // Opcode: DropTrigger P1 * * P4 *
    //
    // Remove the internal (in-memory) data structures that describe
    // the trigger named P4 in database P1.  This is called after a trigger
    // is dropped from disk (using the Destroy opcode) in order to keep
    // the internal representation of the
    // schema consistent with what is on disk.
    // Opcode: IntegrityCk P1 P2 P3 P4 P5
    //
    // Do an analysis of the currently open database.  Store in
    // register (P1+1) the text of an error message describing any problems.
    // If no problems are found, store a NULL in register (P1+1).
    //
    // The register (P1) contains one less than the maximum number of allowed
    // errors.  At most reg(P1) errors will be reported.
    // In other words, the analysis stops as soon as reg(P1) errors are
    // seen.  Reg(P1) is updated with the number of errors remaining.
    //
    // The root page numbers of all tables in the database are integers
    // stored in P4_INTARRAY argument.
    //
    // If P5 is not zero, the check is done on the auxiliary database
    // file, not the main database file.
    //
    // This opcode is used to implement the integrity_check pragma.
    // Opcode: IFindKey P1 P2 P3 P4 *
    //
    // This instruction always follows an OP_Found with the same P1, P2 and P3
    // values as this instruction and a non-zero P4 value. The P4 value to
    // this opcode is of type P4_INDEX and contains a pointer to the Index
    // object of for the index being searched.
    //
    // This opcode uses sqlite3VdbeFindIndexKey() to search around the current
    // cursor location for an index key that exactly matches all fields that
    // are not indexed expressions or references to VIRTUAL generated columns,
    // and either exactly match or are real numbers that are within 2 ULPs of
    // each other if the don't match.
    //
    // To put it another way, this opcode looks for nearby index entries that
    // are very close to the search key, but which might have small differences
    // in floating-point values that come via an expression.
    //
    // If no nearby alternative entry is found in cursor P1, then jump to P2.
    // But if a close match is found, fall through.
    //
    // This opcode is used by PRAGMA integrity_check to help distinguish
    // between truely corrupt indexes and expression indexes that are holding
    // floating-point values that are off by one or two ULPs.
    // Opcode: RowSetAdd P1 P2 * * *
    // Synopsis: rowset(P1)=r[P2]
    //
    // Insert the integer value held by register P2 into a RowSet object
    // held in register P1.
    //
    // An assertion fails if P2 is not an integer.
    // Opcode: RowSetRead P1 P2 P3 * *
    // Synopsis: r[P3]=rowset(P1)
    //
    // Extract the smallest value from the RowSet object in P1
    // and put that value into register P3.
    // Or, if RowSet object P1 is initially empty, leave P3
    // unchanged and jump to instruction P2.
    // Opcode: RowSetTest P1 P2 P3 P4
    // Synopsis: if r[P3] in rowset(P1) goto P2
    //
    // Register P3 is assumed to hold a 64-bit integer value. If register P1
    // contains a RowSet object and that RowSet object contains
    // the value held in P3, jump to register P2. Otherwise, insert the
    // integer in P3 into the RowSet and continue on to the
    // next opcode.
    //
    // The RowSet object is optimized for the case where sets of integers
    // are inserted in distinct phases, which each set contains no duplicates.
    // Each set is identified by a unique P4 value. The first set
    // must have P4==0, the final set must have P4==-1, and for all other sets
    // must have P4>0.
    //
    // This allows optimizations: (a) when P4==0 there is no need to test
    // the RowSet object for P3, as it is guaranteed not to contain it,
    // (b) when P4==-1 there is no need to insert the value, as it will
    // never be tested for, and (c) when a value that is part of set X is
    // inserted, there is no need to search to see if the same value was
    // previously inserted as part of set X (only if it was previously
    // inserted as part of some other set).
    // Opcode: Program P1 P2 P3 P4 P5
    //
    // Execute the trigger program passed as P4 (type P4_SUBPROGRAM).
    //
    // P1 contains the address of the memory cell that contains the first memory
    // cell in an array of values used as arguments to the sub-program. P2
    // contains the address to jump to if the sub-program throws an IGNORE
    // exception using the RAISE() function. P2 might be zero, if there is
    // no possibility that an IGNORE exception will be raised.
    // Register P3 contains the address
    // of a memory cell in this (the parent) VM that is used to allocate the
    // memory required by the sub-vdbe at runtime.
    //
    // P4 is a pointer to the VM containing the trigger program.
    //
    // If P5 is non-zero, then recursive program invocation is enabled.
    // Opcode: Param P1 P2 * * *
    //
    // This opcode is only ever present in sub-programs called via the
    // OP_Program instruction. Copy a value currently stored in a memory
    // cell of the calling (parent) frame to cell P2 in the current frames
    // address space. This is used by trigger programs to access the new.*
    // and old.* values.
    //
    // The address of the cell in the parent frame is determined by adding
    // the value of the P1 argument to the value of the P1 argument to the
    // calling OP_Program instruction.
    // Opcode: FkCounter P1 P2 * * *
    // Synopsis: fkctr[P1]+=P2
    //
    // Increment a "constraint counter" by P2 (P2 may be negative or positive).
    // If P1 is non-zero, the database constraint counter is incremented
    // (deferred foreign key constraints). Otherwise, if P1 is zero, the
    // statement counter is incremented (immediate foreign key constraints).
    // Opcode: FkIfZero P1 P2 * * *
    // Synopsis: if fkctr[P1]==0 goto P2
    //
    // This opcode tests if a foreign key constraint-counter is currently zero.
    // If so, jump to instruction P2. Otherwise, fall through to the next
    // instruction.
    //
    // If P1 is non-zero, then the jump is taken if the database constraint-counter
    // is zero (the one that counts deferred constraint violations). If P1 is
    // zero, the jump is taken if the statement constraint-counter is zero
    // (immediate foreign key constraint violations).
    // Opcode: MemMax P1 P2 * * *
    // Synopsis: r[P1]=max(r[P1],r[P2])
    //
    // P1 is a register in the root frame of this VM (the root frame is
    // different from the current frame if this instruction is being executed
    // within a sub-program). Set the value of register P1 to the maximum of
    // its current value and the value in register P2.
    //
    // This instruction throws an error if the memory cell is not initially
    // an integer.
    // Opcode: IfPos P1 P2 P3 * *
    // Synopsis: if r[P1]>0 then r[P1]-=P3, goto P2
    //
    // Register P1 must contain an integer.
    // If the value of register P1 is 1 or greater, subtract P3 from the
    // value in P1 and jump to P2.
    //
    // If the initial value of register P1 is less than 1, then the
    // value is unchanged and control passes through to the next instruction.
    // Opcode: OffsetLimit P1 P2 P3 * *
    // Synopsis: if r[P1]>0 then r[P2]=r[P1]+max(0,r[P3]) else r[P2]=(-1)
    //
    // This opcode performs a commonly used computation associated with
    // LIMIT and OFFSET processing.  r[P1] holds the limit counter.  r[P3]
    // holds the offset counter.  The opcode computes the combined value
    // of the LIMIT and OFFSET and stores that value in r[P2].  The r[P2]
    // value computed is the total number of rows that will need to be
    // visited in order to complete the query.
    //
    // If r[P3] is zero or negative, that means there is no OFFSET
    // and r[P2] is set to be the value of the LIMIT, r[P1].
    //
    // if r[P1] is zero or negative, that means there is no LIMIT
    // and r[P2] is set to -1.
    //
    // Otherwise, r[P2] is set to the sum of r[P1] and r[P3].
    // Opcode: IfNotZero P1 P2 * * *
    // Synopsis: if r[P1]!=0 then r[P1]--, goto P2
    //
    // Register P1 must contain an integer.  If the content of register P1 is
    // initially greater than zero, then decrement the value in register P1.
    // If it is non-zero (negative or positive) and then also jump to P2.
    // If register P1 is initially zero, leave it unchanged and fall through.
    // Opcode: DecrJumpZero P1 P2 * * *
    // Synopsis: if (--r[P1])==0 goto P2
    //
    // Register P1 must hold an integer.  Decrement the value in P1
    // and jump to P2 if the new value is exactly zero.
    // Opcode: AggStep * P2 P3 P4 P5
    // Synopsis: accum=r[P3] step(r[P2@P5])
    //
    // Execute the xStep function for an aggregate.
    // The function has P5 arguments.  P4 is a pointer to the
    // FuncDef structure that specifies the function.  Register P3 is the
    // accumulator.
    //
    // The P5 arguments are taken from register P2 and its
    // successors.
    //
    // Opcode: AggInverse * P2 P3 P4 P5
    // Synopsis: accum=r[P3] inverse(r[P2@P5])
    //
    // Execute the xInverse function for an aggregate.
    // The function has P5 arguments.  P4 is a pointer to the
    // FuncDef structure that specifies the function.  Register P3 is the
    // accumulator.
    //
    // The P5 arguments are taken from register P2 and its
    // successors.
    //
    // Opcode: AggStep1 P1 P2 P3 P4 P5
    // Synopsis: accum=r[P3] step(r[P2@P5])
    //
    // Execute the xStep (if P1==0) or xInverse (if P1!=0) function for an
    // aggregate.  The function has P5 arguments.  P4 is a pointer to the
    // FuncDef structure that specifies the function.  Register P3 is the
    // accumulator.
    //
    // The P5 arguments are taken from register P2 and its
    // successors.
    //
    // This opcode is initially coded as OP_AggStep0.  On first evaluation,
    // the FuncDef stored in P4 is converted into an sqlite3_context and
    // the opcode is changed.  In this way, the initialization of the
    // sqlite3_context only happens once, instead of on each call to the
    // step function.
    // Opcode: AggFinal P1 P2 * P4 *
    // Synopsis: accum=r[P1] N=P2
    //
    // P1 is the memory location that is the accumulator for an aggregate
    // or window function.  Execute the finalizer function
    // for an aggregate and store the result in P1.
    //
    // P2 is the number of arguments that the step function takes and
    // P4 is a pointer to the FuncDef for this function.  The P2
    // argument is not used by this opcode.  It is only there to disambiguate
    // functions that can take varying numbers of arguments.  The
    // P4 argument is only needed for the case where
    // the step function was not previously called.
    //
    // Opcode: AggValue * P2 P3 P4 *
    // Synopsis: r[P3]=value N=P2
    //
    // Invoke the xValue() function and store the result in register P3.
    //
    // P2 is the number of arguments that the step function takes and
    // P4 is a pointer to the FuncDef for this function.  The P2
    // argument is not used by this opcode.  It is only there to disambiguate
    // functions that can take varying numbers of arguments.  The
    // P4 argument is only needed for the case where
    // the step function was not previously called.
    // Opcode: Checkpoint P1 P2 P3 * *
    //
    // Checkpoint database P1. This is a no-op if P1 is not currently in
    // WAL mode. Parameter P2 is one of SQLITE_CHECKPOINT_PASSIVE, FULL,
    // RESTART, or TRUNCATE.  Write 1 or 0 into mem[P3] if the checkpoint returns
    // SQLITE_BUSY or not, respectively.  Write the number of pages in the
    // WAL after the checkpoint into mem[P3+1] and the number of pages
    // in the WAL that have been checkpointed after the checkpoint
    // completes into mem[P3+2].  However on an error, mem[P3+1] and
    // mem[P3+2] are initialized to -1.
    // Opcode: JournalMode P1 P2 P3 * *
    //
    // Change the journal mode of database P1 to P3. P3 must be one of the
    // PAGER_JOURNALMODE_XXX values. If changing between the various rollback
    // modes (delete, truncate, persist, off and memory), this is a simple
    // operation. No IO is required.
    //
    // If changing into or out of WAL mode the procedure is more complicated.
    //
    // Write a string containing the final journal-mode to register P2.
    // Opcode: Vacuum P1 P2 * * *
    //
    // Vacuum the entire database P1.  P1 is 0 for "main", and 2 or more
    // for an attached database.  The "temp" database may not be vacuumed.
    //
    // If P2 is not zero, then it is a register holding a string which is
    // the file into which the result of vacuum should be written.  When
    // P2 is zero, the vacuum overwrites the original database.
    // Opcode: IncrVacuum P1 P2 * * *
    //
    // Perform a single step of the incremental vacuum procedure on
    // the P1 database. If the vacuum has finished, jump to instruction
    // P2. Otherwise, fall through to the next instruction.
    // Opcode: Expire P1 P2 * * *
    //
    // Cause precompiled statements to expire.  When an expired statement
    // is executed using sqlite3_step() it will either automatically
    // reprepare itself (if it was originally created using sqlite3_prepare_v2())
    // or it will fail with SQLITE_SCHEMA.
    //
    // If P1 is 0, then all SQL statements become expired. If P1 is non-zero,
    // then only the currently executing statement is expired.
    //
    // If P2 is 0, then SQL statements are expired immediately.  If P2 is 1,
    // then running SQL statements are allowed to continue to run to completion.
    // The P2==1 case occurs when a CREATE INDEX or similar schema change happens
    // that might help the statement run faster but which does not affect the
    // correctness of operation.
    // Opcode: CursorLock P1 * * * *
    //
    // Lock the btree to which cursor P1 is pointing so that the btree cannot be
    // written by an other cursor.
    // Opcode: CursorUnlock P1 * * * *
    //
    // Unlock the btree to which cursor P1 is pointing so that it can be
    // written by other cursors.
    // Opcode: TableLock P1 P2 P3 P4 *
    // Synopsis: iDb=P1 root=P2 write=P3
    //
    // Obtain a lock on a particular table. This instruction is only used when
    // the shared-cache feature is enabled.
    //
    // P1 is the index of the database in sqlite3.aDb[] of the database
    // on which the lock is acquired.  A readlock is obtained if P3==0 or
    // a write lock if P3==1.
    //
    // P2 contains the root-page of the table to lock.
    //
    // P4 contains a pointer to the name of the table being locked. This is only
    // used to generate an error message if the lock cannot be obtained.
    // Opcode: VBegin * * * P4 *
    //
    // P4 may be a pointer to an sqlite3_vtab structure. If so, call the
    // xBegin method for that table.
    //
    // Also, whether or not P4 is set, check that this is not being called from
    // within a callback to a virtual table xSync() method. If it is, the error
    // code will be set to SQLITE_LOCKED.
    // Opcode: VCreate P1 P2 * * *
    //
    // P2 is a register that holds the name of a virtual table in database
    // P1. Call the xCreate method for that table.
    // Opcode: VDestroy P1 * * P4 *
    //
    // P4 is the name of a virtual table in database P1.  Call the xDestroy method
    // of that table.
    // Opcode: VOpen P1 * * P4 *
    //
    // P4 is a pointer to a virtual table object, an sqlite3_vtab structure.
    // P1 is a cursor number.  This opcode opens a cursor to the virtual
    // table and stores that cursor in P1.
    // Opcode: VCheck P1 P2 P3 P4 *
    //
    // P4 is a pointer to a Table object that is a virtual table in schema P1
    // that supports the xIntegrity() method.  This opcode runs the xIntegrity()
    // method for that virtual table, using P3 as the integer argument.  If
    // an error is reported back, the table name is prepended to the error
    // message and that message is stored in P2.  If no errors are seen,
    // register P2 is set to NULL.
    // Opcode: VInitIn P1 P2 P3 * *
    // Synopsis: r[P2]=ValueList(P1,P3)
    //
    // Set register P2 to be a pointer to a ValueList object for cursor P1
    // with cache register P3 and output register P3+1.  This ValueList object
    // can be used as the first argument to sqlite3_vtab_in_first() and
    // sqlite3_vtab_in_next() to extract all of the values stored in the P1
    // cursor.  Register P3 is used to hold the values returned by
    // sqlite3_vtab_in_first() and sqlite3_vtab_in_next().
    // Opcode: VFilter P1 P2 P3 P4 *
    // Synopsis: iplan=r[P3] zplan='P4'
    //
    // P1 is a cursor opened using VOpen.  P2 is an address to jump to if
    // the filtered result set is empty.
    //
    // P4 is either NULL or a string that was generated by the xBestIndex
    // method of the module.  The interpretation of the P4 string is left
    // to the module implementation.
    //
    // This opcode invokes the xFilter method on the virtual table specified
    // by P1.  The integer query plan parameter to xFilter is stored in register
    // P3. Register P3+1 stores the argc parameter to be passed to the
    // xFilter method. Registers P3+2..P3+1+argc are the argc
    // additional parameters which are passed to
    // xFilter as argv. Register P3+2 becomes argv[0] when passed to xFilter.
    //
    // A jump is made to P2 if the result set after filtering would be empty.
    // Opcode: VColumn P1 P2 P3 * P5
    // Synopsis: r[P3]=vcolumn(P2)
    //
    // Store in register P3 the value of the P2-th column of
    // the current row of the virtual-table of cursor P1.
    //
    // If the VColumn opcode is being used to fetch the value of
    // an unchanging column during an UPDATE operation, then the P5
    // value is OPFLAG_NOCHNG.  This will cause the sqlite3_vtab_nochange()
    // function to return true inside the xColumn method of the virtual
    // table implementation.  The P5 column might also contain other
    // bits (OPFLAG_LENGTHARG or OPFLAG_TYPEOFARG) but those bits are
    // unused by OP_VColumn.
    // Opcode: VNext P1 P2 * * *
    //
    // Advance virtual table P1 to the next row in its result set and
    // jump to instruction P2.  Or, if the virtual table has reached
    // the end of its result set, then fall through to the next instruction.
    // Opcode: VRename P1 * * P4 *
    //
    // P4 is a pointer to a virtual table object, an sqlite3_vtab structure.
    // This opcode invokes the corresponding xRename method. The value
    // in register P1 is passed as the zName argument to the xRename method.
    // Opcode: VUpdate P1 P2 P3 P4 P5
    // Synopsis: data=r[P3@P2]
    //
    // P4 is a pointer to a virtual table object, an sqlite3_vtab structure.
    // This opcode invokes the corresponding xUpdate method. P2 values
    // are contiguous memory cells starting at P3 to pass to the xUpdate
    // invocation. The value in register (P3+P2-1) corresponds to the
    // p2th element of the argv array passed to xUpdate.
    //
    // The xUpdate method will do a DELETE or an INSERT or both.
    // The argv[0] element (which corresponds to memory cell P3)
    // is the rowid of a row to delete.  If argv[0] is NULL then no
    // deletion occurs.  The argv[1] element is the rowid of the new
    // row.  This can be NULL to have the virtual table select the new
    // rowid for itself.  The subsequent elements in the array are
    // the values of columns in the new row.
    //
    // If P2==1 then no insert is performed.  argv[0] is the rowid of
    // a row to delete.
    //
    // P1 is a boolean flag. If it is set to true and the xUpdate call
    // is successful, then the value returned by sqlite3_last_insert_rowid()
    // is set to the value of the rowid for the row just inserted.
    //
    // P5 is the error actions (OE_Replace, OE_Fail, OE_Ignore, etc) to
    // apply in the case of a constraint failure on an insert or update.
    // Opcode: Pagecount P1 P2 * * *
    //
    // Write the current number of pages in database P1 to memory cell P2.
    // Opcode: MaxPgcnt P1 P2 P3 * *
    //
    // Try to set the maximum page count for database P1 to the value in P3.
    // Do not let the maximum page count fall below the current page count and
    // do not change the maximum page count value if P3==0.
    //
    // Store the maximum page count after the change in register P2.
    // Opcode: Function P1 P2 P3 P4 *
    // Synopsis: r[P3]=func(r[P2@NP])
    //
    // Invoke a user function (P4 is a pointer to an sqlite3_context object that
    // contains a pointer to the function to be run) with arguments taken
    // from register P2 and successors.  The number of arguments is in
    // the sqlite3_context object that P4 points to.
    // The result of the function is stored
    // in register P3.  Register P3 must not be one of the function inputs.
    //
    // P1 is a 32-bit bitmask indicating whether or not each argument to the
    // function was determined to be constant at compile time. If the first
    // argument was constant then bit 0 of P1 is set. This is used to determine
    // whether meta data associated with a user function argument using the
    // sqlite3_set_auxdata() API may be safely retained until the next
    // invocation of this opcode.
    //
    // See also: AggStep, AggFinal, PureFunc
    //
    // Opcode: PureFunc P1 P2 P3 P4 *
    // Synopsis: r[P3]=func(r[P2@NP])
    //
    // Invoke a user function (P4 is a pointer to an sqlite3_context object that
    // contains a pointer to the function to be run) with arguments taken
    // from register P2 and successors.  The number of arguments is in
    // the sqlite3_context object that P4 points to.
    // The result of the function is stored
    // in register P3.  Register P3 must not be one of the function inputs.
    //
    // P1 is a 32-bit bitmask indicating whether or not each argument to the
    // function was determined to be constant at compile time. If the first
    // argument was constant then bit 0 of P1 is set. This is used to determine
    // whether meta data associated with a user function argument using the
    // sqlite3_set_auxdata() API may be safely retained until the next
    // invocation of this opcode.
    //
    // This opcode works exactly like OP_Function.  The only difference is in
    // its name.  This opcode is used in places where the function must be
    // purely non-deterministic.  Some built-in date/time functions can be
    // either deterministic of non-deterministic, depending on their arguments.
    // When those function are used in a non-deterministic way, they will check
    // to see if they were called using OP_PureFunc instead of OP_Function, and
    // if they were, they throw an error.
    //
    // See also: AggStep, AggFinal, Function
    // group
    // Opcode: ClrSubtype P1 * * * *
    // Synopsis:  r[P1].subtype = 0
    //
    // Clear the subtype from register P1.
    // Opcode: GetSubtype P1 P2 * * *
    // Synopsis:  r[P2] = r[P1].subtype
    //
    // Extract the subtype value from register P1 and write that subtype
    // into register P2.  If P1 has no subtype, then P1 gets a NULL.
    // Opcode: SetSubtype P1 P2 * * *
    // Synopsis:  r[P2].subtype = r[P1]
    //
    // Set the subtype value of register P2 to the integer from register P1.
    // If P1 is NULL, clear the subtype from p2.
    // Opcode: FilterAdd P1 * P3 P4 *
    // Synopsis: filter(P1) += key(P3@P4)
    //
    // Compute a hash on the P4 registers starting with r[P3] and
    // add that hash to the bloom filter contained in r[P1].
    // Opcode: Filter P1 P2 P3 P4 *
    // Synopsis: if key(P3@P4) not in filter(P1) goto P2
    //
    // Compute a hash on the key contained in the P4 registers starting
    // with r[P3].  Check to see if that hash is found in the
    // bloom filter hosted by register P1.  If it is not present then
    // maybe jump to P2.  Otherwise fall through.
    //
    // False negatives are harmless.  It is always safe to fall through,
    // even if the value is in the bloom filter.  A false negative causes
    // more CPU cycles to be used, but it should still yield the correct
    // answer.  However, an incorrect answer may well arise from a
    // false positive - if the jump is taken when it should fall through.
    // Opcode: Trace P1 P2 * P4 *
    //
    // Write P4 on the statement trace output if statement tracing is
    // enabled.
    //
    // Operand P1 must be 0x7fffffff and P2 must positive.
    //
    // Opcode: Init P1 P2 P3 P4 *
    // Synopsis: Start at P2
    //
    // Programs contain a single instance of this opcode as the very first
    // opcode.
    //
    // If tracing is enabled (by the sqlite3_trace()) interface, then
    // the UTF-8 string contained in P4 is emitted on the trace callback.
    // Or if P4 is blank, use the string returned by sqlite3_sql().
    //
    // If P2 is not zero, jump to instruction P2.
    //
    // Increment the value of P1 so that OP_Once opcodes will jump the
    // first time they are evaluated for this run.
    //
    // If P3 is not zero, then it is an address to jump to if an SQLITE_CORRUPT
    // error is encountered.
    // Opcode: Noop * * * * *
    //
    // Do nothing.  Continue downward to the next opcode.
    //
    // Opcode: Explain P1 P2 P3 P4 *
    //
    // This is the same as OP_Noop during normal query execution.  The
    // purpose of this opcode is to hold information about the query
    // plan for the purpose of EXPLAIN QUERY PLAN output.
    //
    // The P4 value is human-readable text that describes the query plan
    // element.  Something like "SCAN t1" or "SEARCH t2 USING INDEX t2x1".
    //
    // The P1 value is the ID of the current element and P2 is the parent
    // element for the case of nested query plan elements.  If P2 is zero
    // then this element is a top-level element.
    //
    // For loop elements, P3 is the estimated code of each invocation of this
    // element.
    //
    // As with all opcodes, the meanings of the parameters for OP_Explain
    // are subject to change from one release to the next.  Applications
    // should not attempt to interpret or use any of the information
    // contained in the OP_Explain opcode.  The information provided by this
    // opcode is intended for testing and debugging use only.
    // The cases of the switch statement above this line should all be indented
    // by 6 spaces.  But the left-most 6 spaces have been removed to improve the
    // readability.  From this point on down, the normal indentation rules are
    // restored.
    // Jump to here if a string or blob larger than SQLITE_MAX_LENGTH
    // is encountered.
    // Jump to here if a malloc() fails.
    // Jump to here if the sqlite3_interrupt() API sets the interrupt
    // flag.
    return unsafe { std::mem::zeroed() };
}

static mut azType: __SlateAlign16<[*const i8; 4]> = __SlateAlign16([
    (b"NOT NULL\0".as_ptr() as *mut i8) as *const i8,
    (b"UNIQUE\0".as_ptr() as *mut i8) as *const i8,
    (b"CHECK\0".as_ptr() as *mut i8) as *const i8,
    (b"FOREIGN KEY\0".as_ptr() as *mut i8) as *const i8,
]);

static mut and_logic: [u8; 9] = [
    ((0 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((1 as i32) as i8) as u8,
    ((2 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((2 as i32) as i8) as u8,
    ((2 as i32) as i8) as u8,
];

static mut or_logic: [u8; 9] = [
    ((0 as i32) as i8) as u8,
    ((1 as i32) as i8) as u8,
    ((2 as i32) as i8) as u8,
    ((1 as i32) as i8) as u8,
    ((1 as i32) as i8) as u8,
    ((1 as i32) as i8) as u8,
    ((2 as i32) as i8) as u8,
    ((1 as i32) as i8) as u8,
    ((2 as i32) as i8) as u8,
];

static mut aMask: [u8; 12] = [
    ((16 as i32) as i8) as u8,
    ((1 as i32) as i8) as u8,
    ((1 as i32) as i8) as u8,
    ((1 as i32) as i8) as u8,
    ((1 as i32) as i8) as u8,
    ((1 as i32) as i8) as u8,
    ((1 as i32) as i8) as u8,
    ((2 as i32) as i8) as u8,
    ((1 as i32) as i8) as u8,
    ((1 as i32) as i8) as u8,
    ((16 as i32) as i8) as u8,
    ((16 as i32) as i8) as u8,
];

static mut aFlag: [u16; 2] = [
    ((16 as i32) as i16) as u16,
    (((2 as i32) | (512 as i32)) as i16) as u16,
];

static mut aShift: [u8; 8] = [
    ((0 as i32) as i8) as u8,
    ((56 as i32) as i8) as u8,
    ((48 as i32) as i8) as u8,
    ((40 as i32) as i8) as u8,
    ((32 as i32) as i8) as u8,
    ((16 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
];

static mut vfsFlags: i32 = (2 as i32) | (4 as i32) | (16 as i32) | (8 as i32) | (1024 as i32);
