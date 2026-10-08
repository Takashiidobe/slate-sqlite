//! 2006 Oct 10
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
//! This is an SQLite module implementing full-text search.
unsafe extern "C" {
    fn memcpy(__dest: *mut (), __src: *const (), __n: u64) -> *mut ();
    fn memset(__s: *mut (), __c: i32, __n: u64) -> *mut ();
    fn memcmp(__s1: *const (), __s2: *const (), __n: u64) -> i32;
    fn strcmp(__s1: *const i8, __s2: *const i8) -> i32;
    fn strlen(__s: *const i8) -> u64;
    fn sqlite3_libversion_number() -> i32;
    fn sqlite3_exec(
        __v1336: *mut sqlite3,
        sql: *const i8,
        callback: Option<
            unsafe extern "C-unwind" fn(*mut (), i32, *mut *mut i8, *mut *mut i8) -> i32,
        >,
        __v1339: *mut (),
        errmsg: *mut *mut i8,
    ) -> i32;
    fn sqlite3_last_insert_rowid(__v1341: *mut sqlite3) -> i64;
    fn sqlite3_set_last_insert_rowid(__v1342: *mut sqlite3, __v1343: i64);
    fn sqlite3_mprintf(__v1344: *const i8, ...) -> *mut i8;
    fn sqlite3_vmprintf(__v1345: *const i8, __v1346: core::ffi::VaList<'_>) -> *mut i8;
    fn sqlite3_malloc(__v1347: i32) -> *mut ();
    fn sqlite3_malloc64(__v1348: u64) -> *mut ();
    fn sqlite3_realloc64(__v1349: *mut (), __v1350: u64) -> *mut ();
    fn sqlite3_free(__v1351: *mut ());
    fn sqlite3_errmsg(__v1352: *mut sqlite3) -> *const i8;
    fn sqlite3_errstr(__v1353: i32) -> *const i8;
    fn sqlite3_prepare(
        db: *mut sqlite3,
        zSql: *const i8,
        nByte: i32,
        ppStmt: *mut *mut sqlite3_stmt,
        pzTail: *mut *const i8,
    ) -> i32;
    fn sqlite3_bind_int64(__v1359: *mut sqlite3_stmt, __v1360: i32, __v1361: i64) -> i32;
    fn sqlite3_bind_value(
        __v1362: *mut sqlite3_stmt,
        __v1363: i32,
        __v1364: *const sqlite3_value,
    ) -> i32;
    fn sqlite3_column_count(pStmt: *mut sqlite3_stmt) -> i32;
    fn sqlite3_column_name(__v1366: *mut sqlite3_stmt, N: i32) -> *const i8;
    fn sqlite3_step(__v1368: *mut sqlite3_stmt) -> i32;
    fn sqlite3_data_count(pStmt: *mut sqlite3_stmt) -> i32;
    fn sqlite3_column_blob(__v1370: *mut sqlite3_stmt, iCol: i32) -> *const ();
    fn sqlite3_column_int(__v1372: *mut sqlite3_stmt, iCol: i32) -> i32;
    fn sqlite3_column_int64(__v1374: *mut sqlite3_stmt, iCol: i32) -> i64;
    fn sqlite3_column_value(__v1376: *mut sqlite3_stmt, iCol: i32) -> *mut sqlite3_value;
    fn sqlite3_column_bytes(__v1378: *mut sqlite3_stmt, iCol: i32) -> i32;
    fn sqlite3_finalize(pStmt: *mut sqlite3_stmt) -> i32;
    fn sqlite3_reset(pStmt: *mut sqlite3_stmt) -> i32;
    fn sqlite3_value_int(__v1382: *mut sqlite3_value) -> i32;
    fn sqlite3_value_int64(__v1383: *mut sqlite3_value) -> i64;
    fn sqlite3_value_pointer(__v1384: *mut sqlite3_value, __v1385: *const i8) -> *mut ();
    fn sqlite3_value_text(__v1386: *mut sqlite3_value) -> *const u8;
    fn sqlite3_value_type(__v1387: *mut sqlite3_value) -> i32;
    fn sqlite3_value_numeric_type(__v1388: *mut sqlite3_value) -> i32;
    fn sqlite3_result_error(__v1389: *mut sqlite3_context, __v1390: *const i8, __v1391: i32);
    fn sqlite3_result_error_nomem(__v1392: *mut sqlite3_context);
    fn sqlite3_result_error_code(__v1393: *mut sqlite3_context, __v1394: i32);
    fn sqlite3_result_int(__v1395: *mut sqlite3_context, __v1396: i32);
    fn sqlite3_result_int64(__v1397: *mut sqlite3_context, __v1398: i64);
    fn sqlite3_result_text(
        __v1399: *mut sqlite3_context,
        __v1400: *const i8,
        __v1401: i32,
        __v1402: Option<unsafe extern "C-unwind" fn(*mut ())>,
    );
    fn sqlite3_result_value(__v1403: *mut sqlite3_context, __v1404: *mut sqlite3_value);
    fn sqlite3_result_pointer(
        __v1405: *mut sqlite3_context,
        __v1406: *mut (),
        __v1407: *const i8,
        __v1408: Option<unsafe extern "C-unwind" fn(*mut ())>,
    );
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
    fn sqlite3_create_module_v2(
        db: *mut sqlite3,
        zName: *const i8,
        p: *const sqlite3_module,
        pClientData: *mut (),
        xDestroy: Option<unsafe extern "C-unwind" fn(*mut ())>,
    ) -> i32;
    fn sqlite3_declare_vtab(__v1423: *mut sqlite3, zSQL: *const i8) -> i32;
    fn sqlite3_overload_function(__v1425: *mut sqlite3, zFuncName: *const i8, nArg: i32) -> i32;
    fn sqlite3_stricmp(__v1428: *const i8, __v1429: *const i8) -> i32;
    fn sqlite3_strnicmp(__v1430: *const i8, __v1431: *const i8, __v1432: i32) -> i32;
    fn sqlite3_vtab_config(__v1433: *mut sqlite3, op: i32, ...) -> i32;
    fn sqlite3Fts3HashInit(pNew: *mut Fts3Hash, keyClass: i8, copyKey: i8);
    fn sqlite3Fts3HashInsert(
        __v1438: *mut Fts3Hash,
        pKey: *const (),
        nKey: i32,
        pData: *mut (),
    ) -> *mut ();
    fn sqlite3Fts3HashClear(__v1442: *mut Fts3Hash);
    fn sqlite3Fts3UpdateMethod(
        __v1443: *mut sqlite3_vtab,
        __v1444: i32,
        __v1445: *mut *mut sqlite3_value,
        __v1446: *mut i64,
    ) -> i32;
    fn sqlite3Fts3PendingTermsFlush(__v1447: *mut Fts3Table) -> i32;
    fn sqlite3Fts3PendingTermsClear(__v1448: *mut Fts3Table);
    fn sqlite3Fts3Optimize(__v1449: *mut Fts3Table) -> i32;
    fn sqlite3Fts3SegReaderNew(
        __v1450: i32,
        __v1451: i32,
        __v1452: i64,
        __v1453: i64,
        __v1454: i64,
        __v1455: *const i8,
        __v1456: i32,
        __v1457: *mut *mut Fts3SegReader,
    ) -> i32;
    fn sqlite3Fts3SegReaderPending(
        __v1458: *mut Fts3Table,
        __v1459: i32,
        __v1460: *const i8,
        __v1461: i32,
        __v1462: i32,
        __v1463: *mut *mut Fts3SegReader,
    ) -> i32;
    fn sqlite3Fts3SegReaderFree(__v1464: *mut Fts3SegReader);
    fn sqlite3Fts3AllSegdirs(
        __v1465: *mut Fts3Table,
        __v1466: i32,
        __v1467: i32,
        __v1468: i32,
        __v1469: *mut *mut sqlite3_stmt,
    ) -> i32;
    fn sqlite3Fts3ReadBlock(
        __v1470: *mut Fts3Table,
        __v1471: i64,
        __v1472: *mut *mut i8,
        __v1473: *mut i32,
        __v1474: *mut i32,
    ) -> i32;
    fn sqlite3Fts3SelectDoctotal(__v1475: *mut Fts3Table, __v1476: *mut *mut sqlite3_stmt) -> i32;
    fn sqlite3Fts3FreeDeferredTokens(__v1477: *mut Fts3Cursor);
    fn sqlite3Fts3DeferToken(
        __v1478: *mut Fts3Cursor,
        __v1479: *mut Fts3PhraseToken,
        __v1480: i32,
    ) -> i32;
    fn sqlite3Fts3CacheDeferredDoclists(__v1481: *mut Fts3Cursor) -> i32;
    fn sqlite3Fts3FreeDeferredDoclists(__v1482: *mut Fts3Cursor);
    fn sqlite3Fts3DeferredTokenList(
        __v1483: *mut Fts3DeferredToken,
        __v1484: *mut *mut i8,
        __v1485: *mut i32,
    ) -> i32;
    fn sqlite3Fts3SegmentsClose(__v1486: *mut Fts3Table);
    fn sqlite3Fts3MaxLevel(__v1487: *mut Fts3Table, __v1488: *mut i32) -> i32;
    fn sqlite3Fts3SegReaderStart(
        __v1489: *mut Fts3Table,
        __v1490: *mut Fts3MultiSegReader,
        __v1491: *mut Fts3SegFilter,
    ) -> i32;
    fn sqlite3Fts3SegReaderStep(__v1492: *mut Fts3Table, __v1493: *mut Fts3MultiSegReader) -> i32;
    fn sqlite3Fts3SegReaderFinish(__v1494: *mut Fts3MultiSegReader);
    fn sqlite3Fts3Incrmerge(__v1504: *mut Fts3Table, __v1505: i32, __v1506: i32) -> i32;
    fn sqlite3Fts3PrepareStmt(
        p: *mut Fts3Table,
        zSql: *const i8,
        bPersist: i32,
        bAllowVtab: i32,
        pp: *mut *mut sqlite3_stmt,
    ) -> i32;
    fn sqlite3Fts3NextToken(__v1547: *const i8, __v1548: *mut i32) -> *const i8;
    fn sqlite3Fts3InitHashTable(
        __v1549: *mut sqlite3,
        __v1550: *mut Fts3Hash,
        __v1551: *const i8,
    ) -> i32;
    fn sqlite3Fts3InitTokenizer(
        pHash: *mut Fts3Hash,
        __v1553: *const i8,
        __v1554: *mut *mut sqlite3_tokenizer,
        __v1555: *mut *mut i8,
    ) -> i32;
    fn sqlite3Fts3IsIdChar(__v1556: i8) -> i32;
    fn sqlite3Fts3Offsets(__v1557: *mut sqlite3_context, __v1558: *mut Fts3Cursor);
    fn sqlite3Fts3Snippet(
        __v1559: *mut sqlite3_context,
        __v1560: *mut Fts3Cursor,
        __v1561: *const i8,
        __v1562: *const i8,
        __v1563: *const i8,
        __v1564: i32,
        __v1565: i32,
    );
    fn sqlite3Fts3Matchinfo(
        __v1566: *mut sqlite3_context,
        __v1567: *mut Fts3Cursor,
        __v1568: *const i8,
    );
    fn sqlite3Fts3MIBufferFree(p: *mut MatchinfoBuffer);
    fn sqlite3Fts3ExprParse(
        __v1570: *mut sqlite3_tokenizer,
        __v1571: i32,
        __v1572: *mut *mut i8,
        __v1573: i32,
        __v1574: i32,
        __v1575: i32,
        __v1576: *const i8,
        __v1577: i32,
        __v1578: *mut *mut Fts3Expr,
        __v1579: *mut *mut i8,
    ) -> i32;
    fn sqlite3Fts3ExprFree(__v1580: *mut Fts3Expr);
    fn sqlite3Fts3MallocZero(nByte: i64) -> *mut ();
    fn sqlite3Fts3InitAux(db: *mut sqlite3) -> i32;
    fn sqlite3Fts3MsrIncrStart(
        __v1584: *mut Fts3Table,
        __v1585: *mut Fts3MultiSegReader,
        __v1586: i32,
        __v1587: *const i8,
        __v1588: i32,
    ) -> i32;
    fn sqlite3Fts3MsrIncrNext(
        __v1589: *mut Fts3Table,
        __v1590: *mut Fts3MultiSegReader,
        __v1591: *mut i64,
        __v1592: *mut *mut i8,
        __v1593: *mut i32,
    ) -> i32;
    fn sqlite3Fts3MsrOvfl(
        __v1598: *mut Fts3Cursor,
        __v1599: *mut Fts3MultiSegReader,
        __v1600: *mut i32,
    ) -> i32;
    fn sqlite3Fts3MsrIncrRestart(pCsr: *mut Fts3MultiSegReader) -> i32;
    fn sqlite3Fts3InitTok(
        __v1604: *mut sqlite3,
        __v1605: *mut Fts3Hash,
        xDestroy: Option<unsafe extern "C-unwind" fn(*mut ())>,
    ) -> i32;
    fn sqlite3Fts3ExprIterate(
        __v1607: *mut Fts3Expr,
        x: Option<unsafe extern "C-unwind" fn(*mut Fts3Expr, i32, *mut ()) -> i32>,
        __v1609: *mut (),
    ) -> i32;
    fn sqlite3Fts3IntegrityCheck(p: *mut Fts3Table, pbOk: *mut i32) -> i32;
    fn sqlite3Fts3SimpleTokenizerModule(ppModule: *mut *const sqlite3_tokenizer_module);
    fn sqlite3Fts3PorterTokenizerModule(ppModule: *mut *const sqlite3_tokenizer_module);
    fn sqlite3Fts3UnicodeTokenizer(ppModule: *mut *const sqlite3_tokenizer_module);
}

#[repr(C)]
#[derive(Clone, Copy)]
struct sqlite3 {}

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
struct sqlite3_tokenizer_module {
    iVersion: i32,
    xCreate: Option<
        unsafe extern "C-unwind" fn(i32, *const *const i8, *mut *mut sqlite3_tokenizer) -> i32,
    >,
    xDestroy: Option<unsafe extern "C-unwind" fn(*mut sqlite3_tokenizer) -> i32>,
    xOpen: Option<
        unsafe extern "C-unwind" fn(
            *mut sqlite3_tokenizer,
            *const i8,
            i32,
            *mut *mut sqlite3_tokenizer_cursor,
        ) -> i32,
    >,
    xClose: Option<unsafe extern "C-unwind" fn(*mut sqlite3_tokenizer_cursor) -> i32>,
    xNext: Option<
        unsafe extern "C-unwind" fn(
            *mut sqlite3_tokenizer_cursor,
            *mut *const i8,
            *mut i32,
            *mut i32,
            *mut i32,
            *mut i32,
        ) -> i32,
    >,
    xLanguageid: Option<unsafe extern "C-unwind" fn(*mut sqlite3_tokenizer_cursor, i32) -> i32>,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct sqlite3_tokenizer {
    pModule: *const sqlite3_tokenizer_module,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct sqlite3_tokenizer_cursor {
    pTokenizer: *mut sqlite3_tokenizer,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct Fts3Hash {
    keyClass: i8,
    copyKey: i8,
    count: i32,
    first: *mut Fts3HashElem,
    htsize: i32,
    ht: *mut _fts3ht,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct Fts3HashElem {
    next: *mut Fts3HashElem,
    prev: *mut Fts3HashElem,
    data: *mut (),
    pKey: *mut (),
    nKey: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct _fts3ht {
    count: i32,
    chain: *mut Fts3HashElem,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct Fts3Table {
    base: sqlite3_vtab,
    db: *mut sqlite3,
    zDb: *const i8,
    zName: *const i8,
    nColumn: i32,
    azColumn: *mut *mut i8,
    abNotindexed: *mut u8,
    pTokenizer: *mut sqlite3_tokenizer,
    zContentTbl: *mut i8,
    zLanguageid: *mut i8,
    nAutoincrmerge: i32,
    nLeafAdd: u32,
    bLock: i32,
    aStmt: [*mut sqlite3_stmt; 40],
    pSeekStmt: *mut sqlite3_stmt,
    zReadExprlist: *mut i8,
    zWriteExprlist: *mut i8,
    nNodeSize: i32,
    bFts4: u8,
    bHasStat: u8,
    bHasDocsize: u8,
    bDescIdx: u8,
    bIgnoreSavepoint: u8,
    nPgsz: i32,
    zSegmentsTbl: *mut i8,
    pSegments: *mut sqlite3_blob,
    iSavepoint: i32,
    nIndex: i32,
    aIndex: *mut Fts3Index,
    nMaxPendingData: i32,
    nPendingData: i32,
    iPrevDocid: i64,
    iPrevLangid: i32,
    bPrevDelete: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct Fts3Cursor {
    base: sqlite3_vtab_cursor,
    eSearch: i16,
    isEof: u8,
    isRequireSeek: u8,
    bSeekStmt: u8,
    pStmt: *mut sqlite3_stmt,
    pExpr: *mut Fts3Expr,
    iLangid: i32,
    nPhrase: i32,
    pDeferred: *mut Fts3DeferredToken,
    iPrevId: i64,
    pNextId: *mut i8,
    aDoclist: *mut i8,
    nDoclist: i32,
    bDesc: u8,
    eEvalmode: i32,
    nRowAvg: i32,
    nDoc: i64,
    iMinDocid: i64,
    iMaxDocid: i64,
    isMatchinfoNeeded: i32,
    pMIBuffer: *mut MatchinfoBuffer,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct Fts3Doclist {
    aAll: *mut i8,
    nAll: i32,
    pNextDocid: *mut i8,
    iDocid: i64,
    bFreeList: i32,
    pList: *mut i8,
    nList: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct Fts3PhraseToken {
    z: *mut i8,
    n: i32,
    isPrefix: i32,
    bFirst: i32,
    pDeferred: *mut Fts3DeferredToken,
    pSegcsr: *mut Fts3MultiSegReader,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct Fts3Phrase {
    doclist: Fts3Doclist,
    bIncr: i32,
    iDoclistToken: i32,
    pOrPoslist: *mut i8,
    iOrDocid: i64,
    nToken: i32,
    iColumn: i32,
    aToken: [Fts3PhraseToken; 0],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct Fts3Expr {
    eType: i32,
    nNear: i32,
    pParent: *mut Fts3Expr,
    pLeft: *mut Fts3Expr,
    pRight: *mut Fts3Expr,
    pPhrase: *mut Fts3Phrase,
    iDocid: i64,
    bEof: u8,
    bStart: u8,
    bDeferred: u8,
    iPhrase: i32,
    aMI: *mut u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct Fts3SegFilter {
    zTerm: *const i8,
    nTerm: i32,
    iCol: i32,
    flags: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct Fts3DeferredToken {}

#[repr(C)]
#[derive(Clone, Copy)]
struct Fts3SegReader {}

#[repr(C)]
#[derive(Clone, Copy)]
struct Fts3MultiSegReader {
    apSegment: *mut *mut Fts3SegReader,
    nSegment: i32,
    nAdvance: i32,
    pFilter: *mut Fts3SegFilter,
    aBuffer: *mut i8,
    nBuffer: i64,
    iColFilter: i32,
    bRestart: i32,
    nCost: i32,
    bLookup: i32,
    zTerm: *mut i8,
    nTerm: i32,
    aDoclist: *mut i8,
    nDoclist: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct MatchinfoBuffer {}

#[repr(C)]
#[derive(Clone, Copy)]
struct Fts3Index {
    nPrefix: i32,
    hPending: Fts3Hash,
}

// The code in this file is only compiled if:
//
//     * The FTS3 module is being built as an extension
//       (in which case SQLITE_CORE is not defined), or
//
//     * The FTS3 module is being built into the core of
//       SQLite (in which case SQLITE_ENABLE_FTS3 is defined).
// The full-text index is stored in a series of b+tree (-like)
// structures called segments which map terms to doclists.  The
// structures are like b+trees in layout, but are constructed from the
// bottom up in optimal fashion and are not updatable.  Since trees
// are built from the bottom up, things will be described from the
// bottom up.
//
//
// Varints ****
// The basic unit of encoding is a variable-length integer called a
// varint.  We encode variable-length integers in little-endian order
// using seven bits * per byte as follows:
//
// KEY:
//         A = 0xxxxxxx    7 bits of data and one flag bit
//         B = 1xxxxxxx    7 bits of data and one flag bit
//
//  7 bits - A
// 14 bits - BA
// 21 bits - BBA
// and so on.
//
// This is similar in concept to how sqlite encodes "varints" but
// the encoding is not the same.  SQLite varints are big-endian
// are are limited to 9 bytes in length whereas FTS3 varints are
// little-endian and can be up to 10 bytes in length (in theory).
//
// Example encodings:
//
//     1:    0x01
//   127:    0x7f
//   128:    0x81 0x00
//
//
// Document lists ****
// A doclist (document list) holds a docid-sorted list of hits for a
// given term.  Doclists hold docids and associated token positions.
// A docid is the unique integer identifier for a single document.
// A position is the index of a word within the document.  The first
// word of the document has a position of 0.
//
// FTS3 used to optionally store character offsets using a compile-time
// option.  But that functionality is no longer supported.
//
// A doclist is stored like this:
//
// array {
//   varint docid;          (delta from previous doclist)
//   array {                (position list for column 0)
//     varint position;     (2 more than the delta from previous position)
//   }
//   array {
//     varint POS_COLUMN;   (marks start of position list for new column)
//     varint column;       (index of new column)
//     array {
//       varint position;   (2 more than the delta from previous position)
//     }
//   }
//   varint POS_END;        (marks end of positions for this document.
// }
//
// Here, array { X } means zero or more occurrences of X, adjacent in
// memory.  A "position" is an index of a token in the token stream
// generated by the tokenizer. Note that POS_END and POS_COLUMN occur
// in the same logical place as the position element, and act as sentinels
// ending a position list array.  POS_END is 0.  POS_COLUMN is 1.
// The positions numbers are not stored literally but rather as two more
// than the difference from the prior position, or the just the position plus
// 2 for the first position.  Example:
//
//   label:       A B C D E  F  G H   I  J K
//   value:     123 5 9 1 1 14 35 0 234 72 0
//
// The 123 value is the first docid.  For column zero in this document
// there are two matches at positions 3 and 10 (5-2 and 9-2+3).  The 1
// at D signals the start of a new column; the 1 at E indicates that the
// new column is column number 1.  There are two positions at 12 and 45
// (14-2 and 35-2+12).  The 0 at H indicate the end-of-document.  The
// 234 at I is the delta to next docid (357).  It has one position 70
// (72-2) and then terminates with the 0 at K.
//
// A "position-list" is the list of positions for multiple columns for
// a single docid.  A "column-list" is the set of positions for a single
// column.  Hence, a position-list consists of one or more column-lists,
// a document record consists of a docid followed by a position-list and
// a doclist consists of one or more document records.
//
// A bare doclist omits the position information, becoming an
// array of varint-encoded docids.
//
// Segment leaf nodes ****
// Segment leaf nodes store terms and doclists, ordered by term.  Leaf
// nodes are written using LeafWriter, and read using LeafReader (to
// iterate through a single leaf node's data) and LeavesReader (to
// iterate through a segment's entire leaf layer).  Leaf nodes have
// the format:
//
// varint iHeight;             (height from leaf level, always 0)
// varint nTerm;               (length of first term)
// char pTerm[nTerm];          (content of first term)
// varint nDoclist;            (length of term's associated doclist)
// char pDoclist[nDoclist];    (content of doclist)
// array {
//                             (further terms are delta-encoded)
//   varint nPrefix;           (length of prefix shared with previous term)
//   varint nSuffix;           (length of unshared suffix)
//   char pTermSuffix[nSuffix];(unshared suffix of next term)
//   varint nDoclist;          (length of term's associated doclist)
//   char pDoclist[nDoclist];  (content of doclist)
// }
//
// Here, array { X } means zero or more occurrences of X, adjacent in
// memory.
//
// Leaf nodes are broken into blocks which are stored contiguously in
// the %_segments table in sorted order.  This means that when the end
// of a node is reached, the next term is in the node with the next
// greater node id.
//
// New data is spilled to a new leaf node when the current node
// exceeds LEAF_MAX bytes (default 2048).  New data which itself is
// larger than STANDALONE_MIN (default 1024) is placed in a standalone
// node (a leaf node with a single term and doclist).  The goal of
// these settings is to pack together groups of small doclists while
// making it efficient to directly access large doclists.  The
// assumption is that large doclists represent terms which are more
// likely to be query targets.
//
// TODO(shess) It may be useful for blocking decisions to be more
// dynamic.  For instance, it may make more sense to have a 2.5k leaf
// node rather than splitting into 2k and .5k nodes.  My intuition is
// that this might extend through 2x or 4x the pagesize.
//
//
// Segment interior nodes ****
// Segment interior nodes store blockids for subtree nodes and terms
// to describe what data is stored by the each subtree.  Interior
// nodes are written using InteriorWriter, and read using
// InteriorReader.  InteriorWriters are created as needed when
// SegmentWriter creates new leaf nodes, or when an interior node
// itself grows too big and must be split.  The format of interior
// nodes:
//
// varint iHeight;           (height from leaf level, always >0)
// varint iBlockid;          (block id of node's leftmost subtree)
// optional {
//   varint nTerm;           (length of first term)
//   char pTerm[nTerm];      (content of first term)
//   array {
//                                (further terms are delta-encoded)
//     varint nPrefix;            (length of shared prefix with previous term)
//     varint nSuffix;            (length of unshared suffix)
//     char pTermSuffix[nSuffix]; (unshared suffix of next term)
//   }
// }
//
// Here, optional { X } means an optional element, while array { X }
// means zero or more occurrences of X, adjacent in memory.
//
// An interior node encodes n terms separating n+1 subtrees.  The
// subtree blocks are contiguous, so only the first subtree's blockid
// is encoded.  The subtree at iBlockid will contain all terms less
// than the first term encoded (or all terms if no term is encoded).
// Otherwise, for terms greater than or equal to pTerm[i] but less
// than pTerm[i+1], the subtree for that term will be rooted at
// iBlockid+i.  Interior nodes only store enough term data to
// distinguish adjacent children (if the rightmost term of the left
// child is "something", and the leftmost term of the right child is
// "wicked", only "w" is stored).
//
// New data is spilled to a new interior node at the same height when
// the current node exceeds INTERIOR_MAX bytes (default 2048).
// INTERIOR_MIN_TERMS (default 7) keeps large terms from monopolizing
// interior nodes and making the tree too skinny.  The interior nodes
// at a given height are naturally tracked by interior nodes at
// height+1, and so on.
//
//
// Segment directory ****
// The segment directory in table %_segdir stores meta-information for
// merging and deleting segments, and also the root node of the
// segment's tree.
//
// The root node is the top node of the segment's tree after encoding
// the entire segment, restricted to ROOT_MAX bytes (default 1024).
// This could be either a leaf node or an interior node.  If the top
// node requires more than ROOT_MAX bytes, it is flushed to %_segments
// and a new root interior node is generated (which should always fit
// within ROOT_MAX because it only needs space for 2 varints, the
// height and the blockid of the previous root).
//
// The meta-information in the segment directory is:
//   level               - segment level (see below)
//   idx                 - index within level
//                       - (level,idx uniquely identify a segment)
//   start_block         - first leaf node
//   leaves_end_block    - last leaf node
//   end_block           - last block (including interior nodes)
//   root                - contents of root node
//
// If the root node is a leaf node, then start_block,
// leaves_end_block, and end_block are all 0.
//
//
// Segment merging ****
// To amortize update costs, segments are grouped into levels and
// merged in batches.  Each increase in level represents exponentially
// more documents.
//
// New documents (actually, document updates) are tokenized and
// written individually (using LeafWriter) to a level 0 segment, with
// incrementing idx.  When idx reaches MERGE_COUNT (default 16), all
// level 0 segments are merged into a single level 1 segment.  Level 1
// is populated like level 0, and eventually MERGE_COUNT level 1
// segments are merged to a single level 2 segment (representing
// MERGE_COUNT^2 updates), and so on.
//
// A segment merge traverses all segments at a given level in
// parallel, performing a straightforward sorted merge.  Since segment
// leaf nodes are written in to the %_segments table in order, this
// merge traverses the underlying sqlite disk structures efficiently.
// After the merge, all segment blocks from the merged level are
// deleted.
//
// MERGE_COUNT controls how often we merge segments.  16 seems to be
// somewhat of a sweet spot for insertion performance.  32 and 64 show
// very similar performance numbers to 16 on insertion, though they're
// a tiny bit slower (perhaps due to more overhead in merge-time
// sorting).  8 is about 20% slower than 16, 4 about 50% slower than
// 16, 2 about 66% slower than 16.
//
// At query time, high MERGE_COUNT increases the number of segments
// which need to be scanned and merged.  For instance, with 100k docs
// inserted:
//
//    MERGE_COUNT   segments
//       16           25
//        8           12
//        4           10
//        2            6
//
// This appears to have only a moderate impact on queries for very
// frequent terms (which are somewhat dominated by segment merge
// costs), and infrequent and non-existent terms still seem to be fast
// even with many segments.
//
// TODO(shess) That said, it would be nice to have a better query-side
// argument for MERGE_COUNT of 16.  Also, it is possible/likely that
// optimizations to things like doclist merging will swing the sweet
// spot around.
//
//
//
// Handling of deletions and updates ****
// Since we're using a segmented structure, with no docid-oriented
// index into the term index, we clearly cannot simply update the term
// index when a document is deleted or updated.  For deletions, we
// write an empty doclist (varint(docid) varint(POS_END)), for updates
// we simply write the new doclist.  Segment merges overwrite older
// data for a particular docid with newer data, so deletes or updates
// will eventually overtake the earlier data and knock it out.  The
// query logic likewise merges doclists so that newer data knocks out
// older data.
// Assume any b-tree layer with more levels than this is corrupt.
#[repr(C)]
#[derive(Clone, Copy)]
struct Fts3HashWrapper {
    /// Hash table
    hash: Fts3Hash,
    /// Number of pointers to this object
    nRef: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct Fts4Option {
    zOpt: *const i8,
    nOpt: i32,
}

/// This variable is set to false when running tests for which the on disk
/// structures should not be corrupt. Otherwise, true. If it is false, extra
/// assert() conditions in the fts3 code are activated - conditions that are
/// only true if it is guaranteed that the fts3 database is not corrupt.
/// Write a 64-bit variable-length integer to memory starting at p[0].
/// The length of data written will be between 1 and FTS3_VARINT_MAX bytes.
/// The number of bytes written is returned.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3Fts3PutVarint(mut p: *mut i8, mut v: i64) -> i32 {
    let mut q: *mut u8 = p as *mut u8;
    let mut vu: u64 = v as u64;
    '__slate_break_1620: loop {
        let __v1842: *mut u8 = q;
        let __v1843: *mut u8 = unsafe { __v1842.offset((1 as i32) as isize) };
        q = __v1843;
        unsafe {
            *__v1842 = (vu & (((127 as i32) as i64) as u64) | (((128 as i32) as i64) as u64)) as u8;
        }
        let __v1844: u64 = vu;
        let __v1845: u64 = __v1844 >> (7 as i32);
        vu = __v1845;
        if !(vu != (((0 as i32) as i64) as u64)) {
            break;
        }
    }
    let __v1846: *mut u8 = unsafe { q.offset(-(1 as i32) as isize) };
    let __v1847: u8 = unsafe { *__v1846 };
    let __v1848: u8 = ((((__v1847 as u32) as i32) & (127 as i32)) as i8) as u8;
    unsafe {
        *__v1846 = __v1848;
    }
    // turn off high bit in final byte
    0 as i32;
    return ((unsafe { q.offset_from((p as *mut u8) as *mut u8) }) as i64) as i32;
}

#[repr(C)]
#[derive(Clone, Copy)]
struct Overloaded {
    zName: *const i8,
    xFunc: Option<unsafe extern "C-unwind" fn(*mut sqlite3_context, i32, *mut *mut sqlite3_value)>,
}

#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3Fts3GetVarintU(mut pBuf: *const i8, mut v: *mut u64) -> i32 {
    let mut p: *const u8 = pBuf as *const u8;
    let mut pStart: *const u8 = p;
    let mut a: u32 = 0 as u32;
    let mut b: u64 = 0 as u64;
    let mut shift: i32 = 0 as i32;
    let __v1849: *const u8 = p;
    let __v1850: *const u8 = unsafe { __v1849.offset((1 as i32) as isize) };
    p = __v1850;
    a = (unsafe { *__v1849 }) as u32;
    if a & ((128 as i32) as u32) == ((0 as i32) as u32) {
        unsafe {
            *v = a as u64;
        }
        return 1 as i32;
    }
    {}
    let __v1851: *const u8 = p;
    let __v1852: *const u8 = unsafe { __v1851.offset((1 as i32) as isize) };
    p = __v1852;
    a = a & ((127 as i32) as u32)
        | (((((unsafe { *__v1851 }) as u32) as i32) << (7 as i32)) as u32);
    if a & ((16384 as i32) as u32) == ((0 as i32) as u32) {
        unsafe {
            *v = a as u64;
        }
        return 2 as i32;
    }
    {}
    let __v1853: *const u8 = p;
    let __v1854: *const u8 = unsafe { __v1853.offset((1 as i32) as isize) };
    p = __v1854;
    a = a & ((16383 as i32) as u32)
        | (((((unsafe { *__v1853 }) as u32) as i32) << (14 as i32)) as u32);
    if a & ((2097152 as i32) as u32) == ((0 as i32) as u32) {
        unsafe {
            *v = a as u64;
        }
        return 3 as i32;
    }
    {}
    let __v1855: *const u8 = p;
    let __v1856: *const u8 = unsafe { __v1855.offset((1 as i32) as isize) };
    p = __v1856;
    a = a & ((2097151 as i32) as u32)
        | (((((unsafe { *__v1855 }) as u32) as i32) << (21 as i32)) as u32);
    if a & ((268435456 as i32) as u32) == ((0 as i32) as u32) {
        unsafe {
            *v = a as u64;
        }
        return 4 as i32;
    }
    {}
    b = (a & ((268435455 as i32) as u32)) as u64;
    shift = 28 as i32;
    '__slate_break_1621: loop {
        if !(shift <= (63 as i32)) {
            break;
        }
        let mut c: u64 = 0 as u64;
        let __v1859: *const u8 = p;
        let __v1860: *const u8 = unsafe { __v1859.offset((1 as i32) as isize) };
        p = __v1860;
        c = (unsafe { *__v1859 }) as u64;
        let __v1861: u64 = b;
        let __v1862: u64 = __v1861.wrapping_add((c & (((127 as i32) as i64) as u64)) << shift);
        b = __v1862;
        if c & (((128 as i32) as i64) as u64) == (((0 as i32) as i64) as u64) {
            break '__slate_break_1621;
        }
        let __v1857: i32 = shift;
        let __v1858: i32 = __v1857 + (7 as i32);
        shift = __v1858;
    }
    unsafe {
        *v = b;
    }
    return ((unsafe { p.offset_from(pStart as *const u8) }) as i64) as i32;
}

/// Read a 64-bit variable-length integer from memory starting at p[0].
/// Return the number of bytes read, or 0 on error.
/// The value is stored in *v.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3Fts3GetVarint(mut pBuf: *const i8, mut v: *mut i64) -> i32 {
    return sqlite3Fts3GetVarintU(pBuf, v as *mut u64);
}

#[repr(C, align(16))]
struct __SlateAlign16<T>(T);

/// Read a 64-bit variable-length integer from memory starting at p[0] and
/// not extending past pEnd[-1].
/// Return the number of bytes read, or 0 on error.
/// The value is stored in *v.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3Fts3GetVarintBounded(
    mut pBuf: *const i8,
    mut pEnd: *const i8,
    mut v: *mut i64,
) -> i32 {
    let mut p: *const u8 = pBuf as *const u8;
    let mut pStart: *const u8 = p;
    let mut pX: *const u8 = pEnd as *const u8;
    let mut b: u64 = ((0 as i32) as i64) as u64;
    let mut shift: i32 = 0 as i32;
    shift = 0 as i32;
    '__slate_break_1622: loop {
        if !(shift <= (63 as i32)) {
            break;
        }
        let mut c: u64 = ((if p < pX {
            ((unsafe { *p }) as u32) as i32
        } else {
            0 as i32
        }) as i64) as u64;
        let __v1865: *const u8 = p;
        let __v1866: *const u8 = unsafe { __v1865.offset((1 as i32) as isize) };
        p = __v1866;
        let __v1867: u64 = b;
        let __v1868: u64 = __v1867.wrapping_add((c & (((127 as i32) as i64) as u64)) << shift);
        b = __v1868;
        if c & (((128 as i32) as i64) as u64) == (((0 as i32) as i64) as u64) {
            break '__slate_break_1622;
        }
        let __v1863: i32 = shift;
        let __v1864: i32 = __v1863 + (7 as i32);
        shift = __v1864;
    }
    unsafe {
        *v = b as i64;
    }
    return ((unsafe { p.offset_from(pStart as *const u8) }) as i64) as i32;
}

/// Similar to sqlite3Fts3GetVarint(), except that the output is truncated to
/// a non-negative 32-bit integer before it is returned.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3Fts3GetVarint32(mut p: *const i8, mut pi: *mut i32) -> i32 {
    let mut ptr: *const u8 = p as *const u8;
    let mut a: u32 = 0 as u32;
    let __v1869: *const u8 = ptr;
    let __v1870: *const u8 = unsafe { __v1869.offset((1 as i32) as isize) };
    ptr = __v1870;
    a = (unsafe { *__v1869 }) as u32;
    0 as i32;
    let __v1871: *const u8 = ptr;
    let __v1872: *const u8 = unsafe { __v1871.offset((1 as i32) as isize) };
    ptr = __v1872;
    a = a & ((127 as i32) as u32)
        | (((((unsafe { *__v1871 }) as u32) as i32) << (7 as i32)) as u32);
    if a & ((16384 as i32) as u32) == ((0 as i32) as u32) {
        unsafe {
            *pi = a as i32;
        }
        return 2 as i32;
    }
    {}
    let __v1873: *const u8 = ptr;
    let __v1874: *const u8 = unsafe { __v1873.offset((1 as i32) as isize) };
    ptr = __v1874;
    a = a & ((16383 as i32) as u32)
        | (((((unsafe { *__v1873 }) as u32) as i32) << (14 as i32)) as u32);
    if a & ((2097152 as i32) as u32) == ((0 as i32) as u32) {
        unsafe {
            *pi = a as i32;
        }
        return 3 as i32;
    }
    {}
    let __v1875: *const u8 = ptr;
    let __v1876: *const u8 = unsafe { __v1875.offset((1 as i32) as isize) };
    ptr = __v1876;
    a = a & ((2097151 as i32) as u32)
        | (((((unsafe { *__v1875 }) as u32) as i32) << (21 as i32)) as u32);
    if a & ((268435456 as i32) as u32) == ((0 as i32) as u32) {
        unsafe {
            *pi = a as i32;
        }
        return 4 as i32;
    }
    {}
    a = a & ((268435455 as i32) as u32);
    unsafe {
        *pi =
            (a | (((((unsafe { *ptr }) as u32) as i32) & (7 as i32)) as u32) << (28 as i32)) as i32;
    }
    0 as i32;
    0 as i32;
    return 5 as i32;
}

/// Return the number of bytes required to encode v as a varint
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3Fts3VarintLen(mut v: u64) -> i32 {
    let mut i: i32 = 0 as i32;
    '__slate_break_1623: loop {
        let __v1877: i32 = i;
        let __v1878: i32 = __v1877 + (1 as i32);
        i = __v1878;
        let __v1879: u64 = v;
        let __v1880: u64 = __v1879 >> (7 as i32);
        v = __v1880;
        if !(v != (((0 as i32) as i64) as u64)) {
            break;
        }
    }
    return i;
}

/// Convert an SQL-style quoted string into a normal string by removing
/// the quote characters.  The conversion is done in-place.  If the
/// input does not begin with a quote character, then this routine
/// is a no-op.
///
/// Examples:
///
///     "abc"   becomes   abc
///     'xyz'   becomes   xyz
///     [pqr]   becomes   pqr
///     `mno`   becomes   mno
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3Fts3Dequote(mut z: *mut i8) {
    let mut quote: i8 = 0 as i8; // Quote character (if any )
    quote = unsafe { *unsafe { z.offset((0 as i32) as isize) } };
    if (quote as i32) == (91 as i32)
        || (quote as i32) == (39 as i32)
        || (quote as i32) == (34 as i32)
        || (quote as i32) == (96 as i32)
    {
        let mut iIn: i32 = 1 as i32; // Index of next byte to read from input
        let mut iOut: i32 = 0 as i32; // Index of next byte to write to output
        // If the first byte was a '[', then the close-quote character is a ']'
        if (quote as i32) == (91 as i32) {
            quote = (93 as i32) as i8;
        }
        '__slate_break_1624: while (unsafe { *unsafe { z.offset(iIn as isize) } }) != (0 as i8) {
            if ((unsafe { *unsafe { z.offset(iIn as isize) } }) as i32) == (quote as i32) {
                if ((unsafe { *unsafe { z.offset((iIn + (1 as i32)) as isize) } }) as i32)
                    != (quote as i32)
                {
                    break '__slate_break_1624;
                }
                let __v1881: i32 = iOut;
                let __v1882: i32 = __v1881 + (1 as i32);
                iOut = __v1882;
                unsafe {
                    *unsafe { z.offset(__v1881 as isize) } = quote;
                }
                let __v1883: i32 = iIn;
                let __v1884: i32 = __v1883 + (2 as i32);
                iIn = __v1884;
            } else {
                let __v1885: i32 = iIn;
                let __v1886: i32 = __v1885 + (1 as i32);
                iIn = __v1886;
                let __v1887: i32 = iOut;
                let __v1888: i32 = __v1887 + (1 as i32);
                iOut = __v1888;
                unsafe {
                    *unsafe { z.offset(__v1887 as isize) } =
                        unsafe { *unsafe { z.offset(__v1885 as isize) } };
                }
            }
        }
        unsafe {
            *unsafe { z.offset(iOut as isize) } = (0 as i32) as i8;
        }
    }
}

/// Read a single varint from the doclist at *pp and advance *pp to point
/// to the first byte past the end of the varint.  Add the value of the varint
/// to *pVal.
fn fts3GetDeltaVarint(mut pp: *mut *mut i8, mut pVal: *mut i64) {
    let mut iVal: i64 = 0 as i64;
    let __v1964: *mut *mut i8 = pp;
    let __v1965: *mut i8 = unsafe { *__v1964 };
    let __v1966: *mut i8 = unsafe {
        __v1965.offset(sqlite3Fts3GetVarint(
            (unsafe { *pp }) as *const i8,
            std::ptr::addr_of_mut!(iVal),
        ) as isize)
    };
    unsafe {
        *__v1964 = __v1966;
    }
    let __v1967: *mut i64 = pVal;
    let __v1968: i64 = unsafe { *__v1967 };
    let __v1969: i64 = __v1968 + iVal;
    unsafe {
        *__v1967 = __v1969;
    }
}

/// When this function is called, *pp points to the first byte following a
/// varint that is part of a doclist (or position-list, or any other list
/// of varints). This function moves *pp to point to the start of that varint,
/// and sets *pVal by the varint value.
///
/// Argument pStart points to the first byte of the doclist that the
/// varint is part of.
fn fts3GetReverseVarint(mut pp: *mut *mut i8, mut pStart: *mut i8, mut pVal: *mut i64) {
    let mut iVal: i64 = 0 as i64;
    let mut p: *mut i8 = unsafe { std::mem::zeroed() };
    // Pointer p now points at the first byte past the varint we are
    // interested in. So, unless the doclist is corrupt, the 0x80 bit is
    // clear on character p[-1].
    p = unsafe { unsafe { *pp }.offset(-((2 as i32) as isize)) };
    '__slate_break_1625: while p >= pStart && ((unsafe { *p }) as i32) & (128 as i32) != (0 as i32)
    {
        {}
        let __v1970: *mut i8 = p;
        let __v1971: *mut i8 = unsafe { __v1970.offset(-((1 as i32) as isize)) };
        p = __v1971;
    }
    let __v1972: *mut i8 = p;
    let __v1973: *mut i8 = unsafe { __v1972.offset((1 as i32) as isize) };
    p = __v1973;
    unsafe {
        *pp = p;
    }
    sqlite3Fts3GetVarint(p as *const i8, std::ptr::addr_of_mut!(iVal));
    unsafe {
        *pVal = iVal;
    }
}

/// The xDisconnect() virtual table method.
#[unsafe(link_section = ".text.slate_distinct.fts3.fts3DisconnectMethod")]
extern "C-unwind" fn fts3DisconnectMethod(mut pVtab: *mut sqlite3_vtab) -> i32 {
    let mut p: *mut Fts3Table = pVtab as *mut Fts3Table;
    let mut i: i32 = 0 as i32;
    0 as i32;
    0 as i32;
    // Free any prepared statements held
    unsafe { sqlite3_finalize(unsafe { (*p).pSeekStmt }) };
    i = 0 as i32;
    '__slate_break_1626: loop {
        if !(i < ((((320 as u64) / (8 as u64)) as u32) as i32)) {
            break;
        }
        unsafe {
            sqlite3_finalize(unsafe {
                *unsafe {
                    unsafe { (*p).aStmt.as_mut_ptr() as *mut *mut sqlite3_stmt }.offset(i as isize)
                }
            })
        };
        let __v1974: i32 = i;
        let __v1975: i32 = __v1974 + (1 as i32);
        i = __v1975;
    }
    unsafe { sqlite3_free((unsafe { (*p).zSegmentsTbl }) as *mut ()) };
    unsafe { sqlite3_free((unsafe { (*p).zReadExprlist }) as *mut ()) };
    unsafe { sqlite3_free((unsafe { (*p).zWriteExprlist }) as *mut ()) };
    unsafe { sqlite3_free((unsafe { (*p).zContentTbl }) as *mut ()) };
    unsafe { sqlite3_free((unsafe { (*p).zLanguageid }) as *mut ()) };
    // Invoke the tokenizer destructor to free the tokenizer.
    unsafe {
        unsafe { (*unsafe { (*unsafe { (*p).pTokenizer }).pModule }).xDestroy }.unwrap()(unsafe {
            (*p).pTokenizer
        })
    };
    unsafe { sqlite3_free(p as *mut ()) };
    return 0 as i32;
}

/// Write an error message into *pzErr
#[unsafe(no_mangle)]
unsafe extern "C-unwind" fn sqlite3Fts3ErrMsg(
    mut pzErr: *mut *mut i8,
    mut zFormat: *const i8,
    mut __va_args: ...
) {
    let mut ap: core::ffi::VaList<'_> = unsafe { std::mem::zeroed() };
    unsafe { sqlite3_free((unsafe { *pzErr }) as *mut ()) };
    ap = __va_args.clone();
    unsafe {
        *pzErr = unsafe { sqlite3_vmprintf(zFormat, ap.clone()) };
    }
    {}
}

/// Construct one or more SQL statements from the format string given
/// and then evaluate those statements. The success code is written
/// into *pRc.
///
/// If *pRc is initially non-zero then this routine is a no-op.
/// Format string for SQL
/// Arguments to the format string
///
/// # Arguments
///
/// * `pRc` - Success code
/// * `db` - Database in which to run SQL
unsafe extern "C-unwind" fn fts3DbExec(
    mut pRc: *mut i32,
    mut db: *mut sqlite3,
    mut zFormat: *const i8,
    mut __va_args: ...
) {
    let mut ap: core::ffi::VaList<'_> = unsafe { std::mem::zeroed() };
    let mut zSql: *mut i8 = unsafe { std::mem::zeroed() };
    if (unsafe { *pRc }) != (0 as i32) {
        return;
    }
    ap = __va_args.clone();
    zSql = unsafe { sqlite3_vmprintf(zFormat, ap.clone()) };
    {}
    if zSql == std::ptr::null_mut::<i8>() {
        unsafe {
            *pRc = 7 as i32;
        }
    } else {
        unsafe {
            *pRc = unsafe {
                sqlite3_exec(
                    db,
                    zSql as *const i8,
                    None,
                    std::ptr::null_mut::<()>(),
                    std::ptr::null_mut::<*mut i8>(),
                )
            };
        }
        unsafe { sqlite3_free(zSql as *mut ()) };
    }
}

/// The xDestroy() virtual table method.
#[unsafe(link_section = ".text.slate_distinct.fts3.fts3DestroyMethod")]
extern "C-unwind" fn fts3DestroyMethod(mut pVtab: *mut sqlite3_vtab) -> i32 {
    let mut p: *mut Fts3Table = pVtab as *mut Fts3Table;
    let mut rc: i32 = 0 as i32; // Return code
    let mut zDb: *const i8 = unsafe { (*p).zDb }; // Name of database (e.g. "main", "temp")
    let mut db: *mut sqlite3 = unsafe { (*p).db }; // Database handle
    // Drop the shadow tables
    unsafe {
        fts3DbExec(std::ptr::addr_of_mut!(rc), db, (b"DROP TABLE IF EXISTS %Q.'%q_segments';DROP TABLE IF EXISTS %Q.'%q_segdir';DROP TABLE IF EXISTS %Q.'%q_docsize';DROP TABLE IF EXISTS %Q.'%q_stat';%s DROP TABLE IF EXISTS %Q.'%q_content';\0".as_ptr() as *mut i8) as *const i8, zDb, unsafe { (*p).zName }, zDb, unsafe { (*p).zName }, zDb, unsafe { (*p).zName }, zDb, unsafe { (*p).zName }, if (unsafe { (*p).zContentTbl }) != std::ptr::null_mut::<i8>() { b"--\0".as_ptr() as *mut i8 } else { b"\0".as_ptr() as *mut i8 }, zDb, unsafe { (*p).zName })
    };
    // If everything has worked, invoke fts3DisconnectMethod() to free the
    // memory associated with the Fts3Table structure and return SQLITE_OK.
    // Otherwise, return an SQLite error code.
    let __v1976: i32;
    if rc == (0 as i32) {
        __v1976 = fts3DisconnectMethod(pVtab);
    } else {
        __v1976 = rc;
    }
    return __v1976;
}

/// Invoke sqlite3_declare_vtab() to declare the schema for the FTS3 table
/// passed as the first argument. This is done as part of the xConnect()
/// and xCreate() methods.
///
/// If *pRc is non-zero when this function is called, it is a no-op.
/// Otherwise, if an error occurs, an SQLite error code is stored in *pRc
/// before returning.
fn fts3DeclareVtab(mut pRc: *mut i32, mut p: *mut Fts3Table) {
    if (unsafe { *pRc }) == (0 as i32) {
        let mut i: i32 = 0 as i32; // Iterator variable
        let mut rc: i32 = 0 as i32; // Return code
        let mut zSql: *mut i8 = unsafe { std::mem::zeroed() }; // SQL statement passed to declare_vtab()
        let mut zCols: *mut i8 = unsafe { std::mem::zeroed() }; // List of user defined columns
        let mut zLanguageid: *const i8 = unsafe { std::mem::zeroed() };
        zLanguageid = (if (unsafe { (*p).zLanguageid }) != std::ptr::null_mut::<i8>() {
            unsafe { (*p).zLanguageid }
        } else {
            b"__langid\0".as_ptr() as *mut i8
        }) as *const i8;
        unsafe { sqlite3_vtab_config(unsafe { (*p).db }, 1 as i32, 1 as i32) };
        unsafe { sqlite3_vtab_config(unsafe { (*p).db }, 2 as i32) };
        // Create a list of user columns for the virtual table
        zCols = unsafe {
            sqlite3_mprintf((b"%Q, \0".as_ptr() as *mut i8) as *const i8, unsafe {
                *unsafe { unsafe { (*p).azColumn }.offset((0 as i32) as isize) }
            })
        };
        i = 1 as i32;
        '__slate_break_1632: loop {
            if !(zCols != std::ptr::null_mut::<i8>() && i < unsafe { (*p).nColumn }) {
                break;
            }
            zCols = unsafe {
                sqlite3_mprintf(
                    (b"%z%Q, \0".as_ptr() as *mut i8) as *const i8,
                    zCols,
                    unsafe { *unsafe { unsafe { (*p).azColumn }.offset(i as isize) } },
                )
            };
            let __v1977: i32 = i;
            let __v1978: i32 = __v1977 + (1 as i32);
            i = __v1978;
        }
        // Create the whole "CREATE TABLE" statement to pass to SQLite
        zSql = unsafe {
            sqlite3_mprintf(
                (b"CREATE TABLE x(%s %Q HIDDEN, docid HIDDEN, %Q HIDDEN)\0".as_ptr() as *mut i8)
                    as *const i8,
                zCols,
                unsafe { (*p).zName },
                zLanguageid,
            )
        };
        if !(zCols != std::ptr::null_mut::<i8>()) || !(zSql != std::ptr::null_mut::<i8>()) {
            rc = 7 as i32;
        } else {
            rc = unsafe { sqlite3_declare_vtab(unsafe { (*p).db }, zSql as *const i8) };
        }
        unsafe { sqlite3_free(zSql as *mut ()) };
        unsafe { sqlite3_free(zCols as *mut ()) };
        unsafe {
            *pRc = rc;
        }
    }
}

/// Create the %_stat table if it does not already exist.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3Fts3CreateStatTable(mut pRc: *mut i32, mut p: *mut Fts3Table) {
    unsafe {
        fts3DbExec(
            pRc,
            unsafe { (*p).db },
            (b"CREATE TABLE IF NOT EXISTS %Q.'%q_stat'(id INTEGER PRIMARY KEY, value BLOB);\0"
                .as_ptr() as *mut i8) as *const i8,
            unsafe { (*p).zDb },
            unsafe { (*p).zName },
        )
    };
    if (unsafe { *pRc }) == (0 as i32) {
        unsafe {
            (*p).bHasStat = ((1 as i32) as i8) as u8;
        }
    }
}

/// Create the backing store tables (%_content, %_segments and %_segdir)
/// required by the FTS3 table passed as the only argument. This is done
/// as part of the vtab xCreate() method.
///
/// If the p->bHasDocsize boolean is true (indicating that this is an
/// FTS4 table, not an FTS3 table) then also create the %_docsize and
/// %_stat tables required by FTS4.
fn fts3CreateTables(mut p: *mut Fts3Table) -> i32 {
    let mut rc: i32 = 0 as i32; // Return code
    let mut i: i32 = 0 as i32; // Iterator variable
    let mut db: *mut sqlite3 = unsafe { (*p).db }; // The database connection
    if (unsafe { (*p).zContentTbl }) == std::ptr::null_mut::<i8>() {
        let mut zLanguageid: *const i8 = (unsafe { (*p).zLanguageid }) as *const i8;
        let mut zContentCols: *mut i8 = unsafe { std::mem::zeroed() }; // Columns of %_content table
        // Create a list of user columns for the content table
        zContentCols = unsafe {
            sqlite3_mprintf((b"docid INTEGER PRIMARY KEY\0".as_ptr() as *mut i8) as *const i8)
        };
        i = 0 as i32;
        '__slate_break_1637: loop {
            if !(zContentCols != std::ptr::null_mut::<i8>() && i < unsafe { (*p).nColumn }) {
                break;
            }
            let mut z: *mut i8 = unsafe { *unsafe { unsafe { (*p).azColumn }.offset(i as isize) } };
            zContentCols = unsafe {
                sqlite3_mprintf(
                    (b"%z, 'c%d%q'\0".as_ptr() as *mut i8) as *const i8,
                    zContentCols,
                    i,
                    z,
                )
            };
            let __v1979: i32 = i;
            let __v1980: i32 = __v1979 + (1 as i32);
            i = __v1980;
        }
        if zLanguageid != std::ptr::null::<i8>() && zContentCols != std::ptr::null_mut::<i8>() {
            zContentCols = unsafe {
                sqlite3_mprintf(
                    (b"%z, langid\0".as_ptr() as *mut i8) as *const i8,
                    zContentCols,
                    zLanguageid,
                )
            };
        }
        if zContentCols == std::ptr::null_mut::<i8>() {
            rc = 7 as i32;
        }
        // Create the content table
        unsafe {
            fts3DbExec(
                std::ptr::addr_of_mut!(rc),
                db,
                (b"CREATE TABLE %Q.'%q_content'(%s)\0".as_ptr() as *mut i8) as *const i8,
                unsafe { (*p).zDb },
                unsafe { (*p).zName },
                zContentCols,
            )
        };
        unsafe { sqlite3_free(zContentCols as *mut ()) };
    }
    // Create other tables
    unsafe {
        fts3DbExec(
            std::ptr::addr_of_mut!(rc),
            db,
            (b"CREATE TABLE %Q.'%q_segments'(blockid INTEGER PRIMARY KEY, block BLOB);\0".as_ptr()
                as *mut i8) as *const i8,
            unsafe { (*p).zDb },
            unsafe { (*p).zName },
        )
    };
    unsafe {
        fts3DbExec(std::ptr::addr_of_mut!(rc), db, (b"CREATE TABLE %Q.'%q_segdir'(level INTEGER,idx INTEGER,start_block INTEGER,leaves_end_block INTEGER,end_block INTEGER,root BLOB,PRIMARY KEY(level, idx));\0".as_ptr() as *mut i8) as *const i8, unsafe { (*p).zDb }, unsafe { (*p).zName })
    };
    if (unsafe { (*p).bHasDocsize }) != (0 as u8) {
        unsafe {
            fts3DbExec(
                std::ptr::addr_of_mut!(rc),
                db,
                (b"CREATE TABLE %Q.'%q_docsize'(docid INTEGER PRIMARY KEY, size BLOB);\0".as_ptr()
                    as *mut i8) as *const i8,
                unsafe { (*p).zDb },
                unsafe { (*p).zName },
            )
        };
    }
    0 as i32;
    if (unsafe { (*p).bHasStat }) != (0 as u8) {
        sqlite3Fts3CreateStatTable(std::ptr::addr_of_mut!(rc), p);
    }
    return rc;
}

/// Store the current database page-size in bytes in p->nPgsz.
///
/// If *pRc is non-zero when this function is called, it is a no-op.
/// Otherwise, if an error occurs, an SQLite error code is stored in *pRc
/// before returning.
fn fts3DatabasePageSize(mut pRc: *mut i32, mut p: *mut Fts3Table) {
    if (unsafe { *pRc }) == (0 as i32) {
        let mut rc: i32 = 0 as i32; // Return code
        let mut zSql: *mut i8 = unsafe { std::mem::zeroed() }; // SQL text "PRAGMA %Q.page_size"
        let mut pStmt: *mut sqlite3_stmt = unsafe { std::mem::zeroed() }; // Compiled "PRAGMA %Q.page_size" statement
        zSql = unsafe {
            sqlite3_mprintf(
                (b"PRAGMA %Q.page_size\0".as_ptr() as *mut i8) as *const i8,
                unsafe { (*p).zDb },
            )
        };
        if !(zSql != std::ptr::null_mut::<i8>()) {
            rc = 7 as i32;
        } else {
            rc = unsafe {
                sqlite3_prepare(
                    unsafe { (*p).db },
                    zSql as *const i8,
                    -(1 as i32),
                    std::ptr::addr_of_mut!(pStmt),
                    std::ptr::null_mut::<*const i8>(),
                )
            };
            if rc == (0 as i32) {
                unsafe { sqlite3_step(pStmt) };
                unsafe {
                    (*p).nPgsz = unsafe { sqlite3_column_int(pStmt, 0 as i32) };
                }
                rc = unsafe { sqlite3_finalize(pStmt) };
            } else {
                if rc == (23 as i32) {
                    unsafe {
                        (*p).nPgsz = 1024 as i32;
                    }
                    rc = 0 as i32;
                }
            }
        }
        0 as i32;
        unsafe { sqlite3_free(zSql as *mut ()) };
        unsafe {
            *pRc = rc;
        }
    }
}

/// "Special" FTS4 arguments are column specifications of the following form:
///
///   <key> = <value>
///
/// There may not be whitespace surrounding the "=" character. The <value>
/// term may be quoted, but the <key> may not.
fn fts3IsSpecialColumn(mut z: *const i8, mut pnKey: *mut i32, mut pzValue: *mut *mut i8) -> i32 {
    let mut zValue: *mut i8 = unsafe { std::mem::zeroed() };
    let mut zCsr: *const i8 = z;
    '__slate_break_1645: while ((unsafe { *zCsr }) as i32) != (61 as i32) {
        if ((unsafe { *zCsr }) as i32) == (0 as i32) {
            return 0 as i32;
        }
        let __v1981: *const i8 = zCsr;
        let __v1982: *const i8 = unsafe { __v1981.offset((1 as i32) as isize) };
        zCsr = __v1982;
    }
    unsafe {
        *pnKey = ((unsafe { zCsr.offset_from(z as *const i8) }) as i64) as i32;
    }
    zValue = unsafe {
        sqlite3_mprintf((b"%s\0".as_ptr() as *mut i8) as *const i8, unsafe {
            zCsr.offset((1 as i32) as isize)
        })
    };
    if zValue != std::ptr::null_mut::<i8>() {
        sqlite3Fts3Dequote(zValue);
    }
    unsafe {
        *pzValue = zValue;
    }
    return 1 as i32;
}

/// Append the output of a printf() style formatting to an existing string.
///
/// # Arguments
///
/// * `pRc` - IN/OUT: Error code
/// * `pz` - IN/OUT: Pointer to string buffer
unsafe extern "C-unwind" fn fts3Appendf(
    mut pRc: *mut i32,
    mut pz: *mut *mut i8,
    mut zFormat: *const i8,
    mut __va_args: ...
) {
    if (unsafe { *pRc }) == (0 as i32) {
        let mut ap: core::ffi::VaList<'_> = unsafe { std::mem::zeroed() };
        let mut z: *mut i8 = unsafe { std::mem::zeroed() };
        ap = __va_args.clone();
        z = unsafe { sqlite3_vmprintf(zFormat, ap.clone()) };
        {}
        if z != std::ptr::null_mut::<i8>() && (unsafe { *pz }) != std::ptr::null_mut::<i8>() {
            let mut z2: *mut i8 = unsafe {
                sqlite3_mprintf(
                    (b"%s%s\0".as_ptr() as *mut i8) as *const i8,
                    unsafe { *pz },
                    z,
                )
            };
            unsafe { sqlite3_free(z as *mut ()) };
            z = z2;
        }
        if z == std::ptr::null_mut::<i8>() {
            unsafe {
                *pRc = 7 as i32;
            }
        }
        unsafe { sqlite3_free((unsafe { *pz }) as *mut ()) };
        unsafe {
            *pz = z;
        }
    }
}

// Printf format string to append
// Arguments for printf format string
/// Return a copy of input string zInput enclosed in double-quotes (") and
/// with all double quote characters escaped. For example:
///
///     fts3QuoteId("un \"zip\"")   ->    "un \"\"zip\"\""
///
/// The pointer returned points to memory obtained from sqlite3_malloc(). It
/// is the callers responsibility to call sqlite3_free() to release this
/// memory.
fn fts3QuoteId(mut zInput: *const i8) -> *mut i8 {
    let mut nRet: i64 = 0 as i64;
    let mut zRet: *mut i8 = unsafe { std::mem::zeroed() };
    nRet = ((2 as i32) + (((unsafe { strlen(zInput) }) as u32) as i32) * (2 as i32) + (1 as i32))
        as i64;
    zRet = (unsafe { sqlite3_malloc64(nRet as u64) }) as *mut i8;
    if zRet != std::ptr::null_mut::<i8>() {
        let mut i: i32 = 0 as i32;
        let mut z: *mut i8 = zRet;
        let __v1983: *mut i8 = z;
        let __v1984: *mut i8 = unsafe { __v1983.offset((1 as i32) as isize) };
        z = __v1984;
        unsafe {
            *__v1983 = (34 as i32) as i8;
        }
        i = 0 as i32;
        '__slate_break_1648: loop {
            if !((unsafe { *unsafe { zInput.offset(i as isize) } }) != (0 as i8)) {
                break;
            }
            if ((unsafe { *unsafe { zInput.offset(i as isize) } }) as i32) == (34 as i32) {
                let __v1987: *mut i8 = z;
                let __v1988: *mut i8 = unsafe { __v1987.offset((1 as i32) as isize) };
                z = __v1988;
                unsafe {
                    *__v1987 = (34 as i32) as i8;
                }
            }
            let __v1989: *mut i8 = z;
            let __v1990: *mut i8 = unsafe { __v1989.offset((1 as i32) as isize) };
            z = __v1990;
            unsafe {
                *__v1989 = unsafe { *unsafe { zInput.offset(i as isize) } };
            }
            let __v1985: i32 = i;
            let __v1986: i32 = __v1985 + (1 as i32);
            i = __v1986;
        }
        let __v1991: *mut i8 = z;
        let __v1992: *mut i8 = unsafe { __v1991.offset((1 as i32) as isize) };
        z = __v1992;
        unsafe {
            *__v1991 = (34 as i32) as i8;
        }
        let __v1993: *mut i8 = z;
        let __v1994: *mut i8 = unsafe { __v1993.offset((1 as i32) as isize) };
        z = __v1994;
        unsafe {
            *__v1993 = (0 as i32) as i8;
        }
    }
    return zRet;
}

/// Return a list of comma separated SQL expressions and a FROM clause that
/// could be used in a SELECT statement such as the following:
///
///     SELECT <list of expressions> FROM %_content AS x ...
///
/// to return the docid, followed by each column of text data in order
/// from left to write. If parameter zFunc is not NULL, then instead of
/// being returned directly each column of text data is passed to an SQL
/// function named zFunc first. For example, if zFunc is "unzip" and the
/// table has the three user-defined columns "a", "b", and "c", the following
/// string is returned:
///
///     "docid, unzip(x.'a'), unzip(x.'b'), unzip(x.'c') FROM %_content AS x"
///
/// The pointer returned points to a buffer allocated by sqlite3_malloc(). It
/// is the responsibility of the caller to eventually free it.
///
/// If *pRc is not SQLITE_OK when this function is called, it is a no-op (and
/// a NULL pointer is returned). Otherwise, if an OOM error is encountered
/// by this function, NULL is returned and *pRc is set to SQLITE_NOMEM. If
/// no error occurs, *pRc is left unmodified.
fn fts3ReadExprList(mut p: *mut Fts3Table, mut zFunc: *const i8, mut pRc: *mut i32) -> *mut i8 {
    let mut zRet: *mut i8 = std::ptr::null_mut::<i8>();
    let mut zFree: *mut i8 = std::ptr::null_mut::<i8>();
    let mut zFunction: *mut i8 = unsafe { std::mem::zeroed() };
    let mut i: i32 = 0 as i32;
    if (unsafe { (*p).zContentTbl }) == std::ptr::null_mut::<i8>() {
        if !(zFunc != std::ptr::null::<i8>()) {
            zFunction = b"\0".as_ptr() as *mut i8;
        } else {
            let __v1995: *mut i8 = fts3QuoteId(zFunc);
            zFunction = __v1995;
            zFree = __v1995;
        }
        unsafe {
            fts3Appendf(
                pRc,
                std::ptr::addr_of_mut!(zRet),
                (b"docid\0".as_ptr() as *mut i8) as *const i8,
            )
        };
        i = 0 as i32;
        '__slate_break_1651: loop {
            if !(i < unsafe { (*p).nColumn }) {
                break;
            }
            unsafe {
                fts3Appendf(
                    pRc,
                    std::ptr::addr_of_mut!(zRet),
                    (b",%s(x.'c%d%q')\0".as_ptr() as *mut i8) as *const i8,
                    zFunction,
                    i,
                    unsafe { *unsafe { unsafe { (*p).azColumn }.offset(i as isize) } },
                )
            };
            let __v1996: i32 = i;
            let __v1997: i32 = __v1996 + (1 as i32);
            i = __v1997;
        }
        if (unsafe { (*p).zLanguageid }) != std::ptr::null_mut::<i8>() {
            unsafe {
                fts3Appendf(
                    pRc,
                    std::ptr::addr_of_mut!(zRet),
                    (b", x.%Q\0".as_ptr() as *mut i8) as *const i8,
                    b"langid\0".as_ptr() as *mut i8,
                )
            };
        }
        unsafe { sqlite3_free(zFree as *mut ()) };
    } else {
        unsafe {
            fts3Appendf(
                pRc,
                std::ptr::addr_of_mut!(zRet),
                (b"rowid\0".as_ptr() as *mut i8) as *const i8,
            )
        };
        i = 0 as i32;
        '__slate_break_1656: loop {
            if !(i < unsafe { (*p).nColumn }) {
                break;
            }
            unsafe {
                fts3Appendf(
                    pRc,
                    std::ptr::addr_of_mut!(zRet),
                    (b", x.'%q'\0".as_ptr() as *mut i8) as *const i8,
                    unsafe { *unsafe { unsafe { (*p).azColumn }.offset(i as isize) } },
                )
            };
            let __v1998: i32 = i;
            let __v1999: i32 = __v1998 + (1 as i32);
            i = __v1999;
        }
        if (unsafe { (*p).zLanguageid }) != std::ptr::null_mut::<i8>() {
            unsafe {
                fts3Appendf(
                    pRc,
                    std::ptr::addr_of_mut!(zRet),
                    (b", x.%Q\0".as_ptr() as *mut i8) as *const i8,
                    unsafe { (*p).zLanguageid },
                )
            };
        }
    }
    unsafe {
        fts3Appendf(
            pRc,
            std::ptr::addr_of_mut!(zRet),
            (b" FROM '%q'.'%q%s' AS x\0".as_ptr() as *mut i8) as *const i8,
            unsafe { (*p).zDb },
            if (unsafe { (*p).zContentTbl }) != std::ptr::null_mut::<i8>() {
                (unsafe { (*p).zContentTbl }) as *const i8
            } else {
                unsafe { (*p).zName }
            },
            if (unsafe { (*p).zContentTbl }) != std::ptr::null_mut::<i8>() {
                b"\0".as_ptr() as *mut i8
            } else {
                b"_content\0".as_ptr() as *mut i8
            },
        )
    };
    return zRet;
}

/// Return a list of N comma separated question marks, where N is the number
/// of columns in the %_content table (one for the docid plus one for each
/// user-defined text column).
///
/// If argument zFunc is not NULL, then all but the first question mark
/// is preceded by zFunc and an open bracket, and followed by a closed
/// bracket. For example, if zFunc is "zip" and the FTS3 table has three
/// user-defined text columns, the following string is returned:
///
///     "?, zip(?), zip(?), zip(?)"
///
/// The pointer returned points to a buffer allocated by sqlite3_malloc(). It
/// is the responsibility of the caller to eventually free it.
///
/// If *pRc is not SQLITE_OK when this function is called, it is a no-op (and
/// a NULL pointer is returned). Otherwise, if an OOM error is encountered
/// by this function, NULL is returned and *pRc is set to SQLITE_NOMEM. If
/// no error occurs, *pRc is left unmodified.
fn fts3WriteExprList(mut p: *mut Fts3Table, mut zFunc: *const i8, mut pRc: *mut i32) -> *mut i8 {
    let mut zRet: *mut i8 = std::ptr::null_mut::<i8>();
    let mut zFree: *mut i8 = std::ptr::null_mut::<i8>();
    let mut zFunction: *mut i8 = unsafe { std::mem::zeroed() };
    let mut i: i32 = 0 as i32;
    if !(zFunc != std::ptr::null::<i8>()) {
        zFunction = b"\0".as_ptr() as *mut i8;
    } else {
        let __v2000: *mut i8 = fts3QuoteId(zFunc);
        zFunction = __v2000;
        zFree = __v2000;
    }
    unsafe {
        fts3Appendf(
            pRc,
            std::ptr::addr_of_mut!(zRet),
            (b"?\0".as_ptr() as *mut i8) as *const i8,
        )
    };
    i = 0 as i32;
    '__slate_break_1664: loop {
        if !(i < unsafe { (*p).nColumn }) {
            break;
        }
        unsafe {
            fts3Appendf(
                pRc,
                std::ptr::addr_of_mut!(zRet),
                (b",%s(?)\0".as_ptr() as *mut i8) as *const i8,
                zFunction,
            )
        };
        let __v2001: i32 = i;
        let __v2002: i32 = __v2001 + (1 as i32);
        i = __v2002;
    }
    if (unsafe { (*p).zLanguageid }) != std::ptr::null_mut::<i8>() {
        unsafe {
            fts3Appendf(
                pRc,
                std::ptr::addr_of_mut!(zRet),
                (b", ?\0".as_ptr() as *mut i8) as *const i8,
            )
        };
    }
    unsafe { sqlite3_free(zFree as *mut ()) };
    return zRet;
}

/// Buffer z contains a positive integer value encoded as utf-8 text.
/// Decode this value and store it in *pnOut, returning the number of bytes
/// consumed. If an overflow error occurs return a negative value.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3Fts3ReadInt(mut z: *const i8, mut pnOut: *mut i32) -> i32 {
    let mut iVal: u64 = ((0 as i32) as i64) as u64;
    let mut i: i32 = 0 as i32;
    i = 0 as i32;
    '__slate_break_1667: loop {
        if !(((unsafe { *unsafe { z.offset(i as isize) } }) as i32) >= (48 as i32)
            && ((unsafe { *unsafe { z.offset(i as isize) } }) as i32) <= (57 as i32))
        {
            break;
        }
        iVal = iVal.wrapping_mul(((10 as i32) as i64) as u64).wrapping_add(
            ((((unsafe { *unsafe { z.offset(i as isize) } }) as i32) - (48 as i32)) as i64) as u64,
        );
        if iVal > (((2147483647 as i32) as i64) as u64) {
            return -(1 as i32);
        }
        let __v1920: i32 = i;
        let __v1921: i32 = __v1920 + (1 as i32);
        i = __v1921;
    }
    unsafe {
        *pnOut = (iVal as u32) as i32;
    }
    return i;
}

/// This function interprets the string at (*pp) as a non-negative integer
/// value. It reads the integer and sets *pnOut to the value read, then
/// sets *pp to point to the byte immediately following the last byte of
/// the integer value.
///
/// Only decimal digits ('0'..'9') may be part of an integer value.
///
/// If *pp does not being with a decimal digit SQLITE_ERROR is returned and
/// the output value undefined. Otherwise SQLITE_OK is returned.
///
/// This function is used when parsing the "prefix=" FTS4 parameter.
fn fts3GobbleInt(mut pp: *mut *const i8, mut pnOut: *mut i32) -> i32 {
    let mut MAX_NPREFIX: i32 = 10000000 as i32;
    let mut nInt: i32 = 0 as i32; // Output value
    let mut nByte: i32 = 0 as i32;
    nByte = sqlite3Fts3ReadInt(unsafe { *pp }, std::ptr::addr_of_mut!(nInt));
    if nInt > MAX_NPREFIX {
        nInt = 0 as i32;
    }
    if nByte == (0 as i32) {
        return 1 as i32;
    }
    unsafe {
        *pnOut = nInt;
    }
    let __v2003: *mut *const i8 = pp;
    let __v2004: *const i8 = unsafe { *__v2003 };
    let __v2005: *const i8 = unsafe { __v2004.offset(nByte as isize) };
    unsafe {
        *__v2003 = __v2005;
    }
    return 0 as i32;
}

/// This function is called to allocate an array of Fts3Index structures
/// representing the indexes maintained by the current FTS table. FTS tables
/// always maintain the main "terms" index, but may also maintain one or
/// more "prefix" indexes, depending on the value of the "prefix=" parameter
/// (if any) specified as part of the CREATE VIRTUAL TABLE statement.
///
/// Argument zParam is passed the value of the "prefix=" option if one was
/// specified, or NULL otherwise.
///
/// If no error occurs, SQLITE_OK is returned and *apIndex set to point to
/// the allocated array. *pnIndex is set to the number of elements in the
/// array. If an error does occur, an SQLite error code is returned.
///
/// Regardless of whether or not an error is returned, it is the responsibility
/// of the caller to call sqlite3_free() on the output array to free it.
///
/// # Arguments
///
/// * `zParam` - ABC in prefix=ABC parameter to parse
/// * `pnIndex` - OUT: size of *apIndex[] array
/// * `apIndex` - OUT: Array of indexes for this table
fn fts3PrefixParameter(
    mut zParam: *const i8,
    mut pnIndex: *mut i32,
    mut apIndex: *mut *mut Fts3Index,
) -> i32 {
    let mut aIndex: *mut Fts3Index = unsafe { std::mem::zeroed() }; // Allocated array
    let mut nIndex: i32 = 1 as i32; // Number of entries in array
    if zParam != std::ptr::null::<i8>()
        && (unsafe { *unsafe { zParam.offset((0 as i32) as isize) } }) != (0 as i8)
    {
        let mut p: *const i8 = unsafe { std::mem::zeroed() };
        let __v2006: i32 = nIndex;
        let __v2007: i32 = __v2006 + (1 as i32);
        nIndex = __v2007;
        p = zParam;
        '__slate_break_1668: while (unsafe { *p }) != (0 as i8) {
            if ((unsafe { *p }) as i32) == (44 as i32) {
                let __v2010: i32 = nIndex;
                let __v2011: i32 = __v2010 + (1 as i32);
                nIndex = __v2011;
            }
            let __v2008: *const i8 = p;
            let __v2009: *const i8 = unsafe { __v2008.offset((1 as i32) as isize) };
            p = __v2009;
        }
    }
    aIndex = (unsafe { sqlite3_malloc64((40 as u64).wrapping_mul((nIndex as i64) as u64)) })
        as *mut Fts3Index;
    unsafe {
        *apIndex = aIndex;
    }
    if !(aIndex != std::ptr::null_mut::<Fts3Index>()) {
        return 7 as i32;
    }
    unsafe {
        memset(
            aIndex as *mut (),
            0 as i32,
            (40 as u64).wrapping_mul((nIndex as i64) as u64),
        )
    };
    if zParam != std::ptr::null::<i8>() {
        let mut p: *const i8 = zParam;
        let mut i: i32 = 0 as i32;
        i = 1 as i32;
        '__slate_break_1669: loop {
            if !(i < nIndex) {
                break;
            }
            let mut nPrefix: i32 = 0 as i32;
            if fts3GobbleInt(std::ptr::addr_of_mut!(p), std::ptr::addr_of_mut!(nPrefix))
                != (0 as i32)
            {
                return 1 as i32;
            }
            0 as i32;
            if nPrefix == (0 as i32) {
                let __v2014: i32 = nIndex;
                let __v2015: i32 = __v2014 - (1 as i32);
                nIndex = __v2015;
                let __v2016: i32 = i;
                let __v2017: i32 = __v2016 - (1 as i32);
                i = __v2017;
            } else {
                unsafe {
                    (*unsafe { aIndex.offset(i as isize) }).nPrefix = nPrefix;
                }
            }
            let __v2018: *const i8 = p;
            let __v2019: *const i8 = unsafe { __v2018.offset((1 as i32) as isize) };
            p = __v2019;
            let __v2012: i32 = i;
            let __v2013: i32 = __v2012 + (1 as i32);
            i = __v2013;
        }
    }
    unsafe {
        *pnIndex = nIndex;
    }
    return 0 as i32;
}

/// This function is called when initializing an FTS4 table that uses the
/// content=xxx option. It determines the number of and names of the columns
/// of the new FTS4 table.
///
/// The third argument passed to this function is the value passed to the
/// config=xxx option (i.e. "xxx"). This function queries the database for
/// a table of that name. If found, the output variables are populated
/// as follows:
///
///   *pnCol:   Set to the number of columns table xxx has,
///
///   *pnStr:   Set to the total amount of space required to store a copy
///             of each columns name, including the nul-terminator.
///
///   *pazCol:  Set to point to an array of *pnCol strings. Each string is
///             the name of the corresponding column in table xxx. The array
///             and its contents are allocated using a single allocation. It
///             is the responsibility of the caller to free this allocation
///             by eventually passing the *pazCol value to sqlite3_free().
///
/// If the table cannot be found, an error code is returned and the output
/// variables are undefined. Or, if an OOM is encountered, SQLITE_NOMEM is
/// returned (and the output variables are undefined).
///
/// # Arguments
///
/// * `db` - Database handle
/// * `zDb` - Name of db (i.e. "main", "temp" etc.)
/// * `zTbl` - Name of content table
/// * `pazCol` - OUT: Malloc'd array of column names
/// * `pnCol` - OUT: Size of array *pazCol
/// * `pnStr` - OUT: Bytes of string content
/// * `pzErr` - OUT: error message
fn fts3ContentColumns(
    mut db: *mut sqlite3,
    mut zDb: *const i8,
    mut zTbl: *const i8,
    mut pazCol: *mut *mut *const i8,
    mut pnCol: *mut i32,
    mut pnStr: *mut i32,
    mut pzErr: *mut *mut i8,
) -> i32 {
    let mut rc: i32 = 0 as i32; // Return code
    let mut zSql: *mut i8 = unsafe { std::mem::zeroed() }; // "SELECT *" statement on zTbl
    let mut pStmt: *mut sqlite3_stmt = std::ptr::null_mut::<sqlite3_stmt>(); // Compiled version of zSql
    zSql = unsafe {
        sqlite3_mprintf(
            (b"SELECT * FROM %Q.%Q\0".as_ptr() as *mut i8) as *const i8,
            zDb,
            zTbl,
        )
    };
    if !(zSql != std::ptr::null_mut::<i8>()) {
        rc = 7 as i32;
    } else {
        rc = unsafe {
            sqlite3_prepare(
                db,
                zSql as *const i8,
                -(1 as i32),
                std::ptr::addr_of_mut!(pStmt),
                std::ptr::null_mut::<*const i8>(),
            )
        };
        if rc != (0 as i32) {
            unsafe {
                sqlite3Fts3ErrMsg(pzErr, (b"%s\0".as_ptr() as *mut i8) as *const i8, unsafe {
                    sqlite3_errmsg(db)
                })
            };
        }
    }
    unsafe { sqlite3_free(zSql as *mut ()) };
    if rc == (0 as i32) {
        let mut azCol: *mut *const i8 = unsafe { std::mem::zeroed() }; // Output array
        let mut nStr: i64 = (0 as i32) as i64; // Size of all column names (incl. 0x00)
        let mut nCol: i32 = 0 as i32; // Number of table columns
        let mut i: i32 = 0 as i32; // Used to iterate through columns
        // Loop through the returned columns. Set nStr to the number of bytes of
        // space required to store a copy of each column name, including the
        // nul-terminator byte.
        nCol = unsafe { sqlite3_column_count(pStmt) };
        i = 0 as i32;
        '__slate_break_1672: loop {
            if !(i < nCol) {
                break;
            }
            let mut zCol: *const i8 = unsafe { sqlite3_column_name(pStmt, i) };
            let __v2022: i64 = nStr;
            let __v2023: i64 = (__v2022 as u64)
                .wrapping_add(unsafe { strlen(zCol) }.wrapping_add(((1 as i32) as i64) as u64))
                as i64;
            nStr = __v2023;
            let __v2020: i32 = i;
            let __v2021: i32 = __v2020 + (1 as i32);
            i = __v2021;
        }
        // Allocate and populate the array to return.
        azCol = (unsafe {
            sqlite3_malloc64(
                (8 as u64)
                    .wrapping_mul((nCol as i64) as u64)
                    .wrapping_add(nStr as u64),
            )
        }) as *mut *const i8;
        if azCol == std::ptr::null_mut::<*const i8>() {
            rc = 7 as i32;
        } else {
            let mut p: *mut i8 = (unsafe { azCol.offset(nCol as isize) }) as *mut i8;
            i = 0 as i32;
            '__slate_break_1673: loop {
                if !(i < nCol) {
                    break;
                }
                let mut zCol: *const i8 = unsafe { sqlite3_column_name(pStmt, i) };
                let mut n: i32 = (((unsafe { strlen(zCol) }) as u32) as i32) + (1 as i32);
                unsafe { memcpy(p as *mut (), zCol as *const (), (n as i64) as u64) };
                unsafe {
                    *unsafe { azCol.offset(i as isize) } = p as *const i8;
                }
                let __v2026: *mut i8 = p;
                let __v2027: *mut i8 = unsafe { __v2026.offset(n as isize) };
                p = __v2027;
                let __v2024: i32 = i;
                let __v2025: i32 = __v2024 + (1 as i32);
                i = __v2025;
            }
        }
        unsafe { sqlite3_finalize(pStmt) };
        // Set the output variables.
        unsafe {
            *pnCol = nCol;
        }
        unsafe {
            *pnStr = nStr as i32;
        }
        unsafe {
            *pazCol = azCol;
        }
    }
    return rc;
}

/// This function is the implementation of both the xConnect and xCreate
/// methods of the FTS3 virtual table.
///
/// The argv[] array contains the following:
///
///   argv[0]   -> module name  ("fts3" or "fts4")
///   argv[1]   -> database name
///   argv[2]   -> table name
///   argv[...] -> "column name" and other module argument fields.
///
/// # Arguments
///
/// * `isCreate` - True for xCreate, false for xConnect
/// * `db` - The SQLite database connection
/// * `pAux` - Hash table containing tokenizers
/// * `argc` - Number of elements in argv array
/// * `argv` - xCreate/xConnect argument array
/// * `ppVTab` - Write the resulting vtab structure here
/// * `pzErr` - Write any error message here
fn fts3InitVtab(
    mut isCreate: i32,
    mut db: *mut sqlite3,
    mut pAux: *mut (),
    mut argc: i32,
    mut argv: *const *const i8,
    mut ppVTab: *mut *mut sqlite3_vtab,
    mut pzErr: *mut *mut i8,
) -> i32 {
    let mut __slate_storage_2070: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2070: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_2070) as *mut i32;
    let mut __slate_storage_2069: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2069: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_2069) as *mut i32;
    let mut __slate_storage_525: std::mem::MaybeUninit<*const i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_525: *mut *const i8 =
        std::ptr::addr_of_mut!(__slate_storage_525) as *mut *const i8;
    let mut __slate_storage_2068: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2068: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_2068) as *mut i32;
    let mut __slate_storage_2067: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2067: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_2067) as *mut i32;
    let mut __slate_storage_2063: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2063: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_2063) as *mut i32;
    let mut __slate_storage_2062: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2062: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_2062) as *mut i32;
    let mut __slate_storage_2065: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2065: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_2065) as *mut i32;
    let mut __slate_storage_2064: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2064: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_2064) as *mut i32;
    let mut __slate_storage_2066: std::mem::MaybeUninit<bool> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2066: *mut bool = std::ptr::addr_of_mut!(__slate_storage_2066) as *mut bool;
    let mut __slate_storage_524: std::mem::MaybeUninit<*mut i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_524: *mut *mut i8 =
        std::ptr::addr_of_mut!(__slate_storage_524) as *mut *mut i8;
    let mut __slate_storage_523: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_523: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_523) as *mut i32;
    let mut __slate_storage_2059: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2059: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_2059) as *mut i32;
    let mut __slate_storage_2058: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2058: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_2058) as *mut i32;
    let mut __slate_storage_2061: std::mem::MaybeUninit<*mut i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2061: *mut *mut i8 =
        std::ptr::addr_of_mut!(__slate_storage_2061) as *mut *mut i8;
    let mut __slate_storage_2060: std::mem::MaybeUninit<*mut i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2060: *mut *mut i8 =
        std::ptr::addr_of_mut!(__slate_storage_2060) as *mut *mut i8;
    let mut __slate_storage_522: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_522: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_522) as *mut i32;
    let mut __slate_storage_521: std::mem::MaybeUninit<*mut i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_521: *mut *mut i8 =
        std::ptr::addr_of_mut!(__slate_storage_521) as *mut *mut i8;
    let mut __slate_storage_2057: std::mem::MaybeUninit<*mut i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2057: *mut *mut i8 =
        std::ptr::addr_of_mut!(__slate_storage_2057) as *mut *mut i8;
    let mut __slate_storage_2056: std::mem::MaybeUninit<*mut i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2056: *mut *mut i8 =
        std::ptr::addr_of_mut!(__slate_storage_2056) as *mut *mut i8;
    let mut __slate_storage_2055: std::mem::MaybeUninit<*mut i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2055: *mut *mut i8 =
        std::ptr::addr_of_mut!(__slate_storage_2055) as *mut *mut i8;
    let mut __slate_storage_2054: std::mem::MaybeUninit<*mut i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2054: *mut *mut i8 =
        std::ptr::addr_of_mut!(__slate_storage_2054) as *mut *mut i8;
    let mut __slate_storage_2053: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2053: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_2053) as *mut i32;
    let mut __slate_storage_2052: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2052: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_2052) as *mut i32;
    let mut __slate_storage_2047: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2047: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_2047) as *mut i32;
    let mut __slate_storage_2046: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2046: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_2046) as *mut i32;
    let mut __slate_storage_2051: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2051: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_2051) as *mut i32;
    let mut __slate_storage_2050: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2050: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_2050) as *mut i32;
    let mut __slate_storage_2049: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2049: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_2049) as *mut i32;
    let mut __slate_storage_2048: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2048: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_2048) as *mut i32;
    let mut __slate_storage_520: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_520: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_520) as *mut i32;
    let mut __slate_storage_519: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_519: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_519) as *mut i32;
    let mut __slate_storage_2029: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2029: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_2029) as *mut i32;
    let mut __slate_storage_2028: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2028: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_2028) as *mut i32;
    let mut __slate_storage_2041: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2041: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_2041) as *mut i32;
    let mut __slate_storage_2040: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2040: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_2040) as *mut i32;
    let mut __slate_storage_2039: std::mem::MaybeUninit<bool> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2039: *mut bool = std::ptr::addr_of_mut!(__slate_storage_2039) as *mut bool;
    let mut __slate_storage_2038: std::mem::MaybeUninit<bool> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2038: *mut bool = std::ptr::addr_of_mut!(__slate_storage_2038) as *mut bool;
    let mut __slate_storage_2037: std::mem::MaybeUninit<bool> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2037: *mut bool = std::ptr::addr_of_mut!(__slate_storage_2037) as *mut bool;
    let mut __slate_storage_2036: std::mem::MaybeUninit<bool> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2036: *mut bool = std::ptr::addr_of_mut!(__slate_storage_2036) as *mut bool;
    let mut __slate_storage_2034: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2034: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_2034) as *mut i32;
    let mut __slate_storage_2033: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2033: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_2033) as *mut i32;
    let mut __slate_storage_2035: std::mem::MaybeUninit<bool> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2035: *mut bool = std::ptr::addr_of_mut!(__slate_storage_2035) as *mut bool;
    let mut __slate_storage_518: std::mem::MaybeUninit<*mut Fts4Option> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_518: *mut *mut Fts4Option =
        std::ptr::addr_of_mut!(__slate_storage_518) as *mut *mut Fts4Option;
    let mut __slate_storage_517: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_517: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_517) as *mut i32; // 0 -> MATCHINFO
    // 1 -> PREFIX
    // 2 -> COMPRESS
    // 3 -> UNCOMPRESS
    // 4 -> ORDER
    // 5 -> CONTENT
    // 6 -> LANGUAGEID
    // 7 -> NOTINDEXED
    let mut __slate_storage_516: std::mem::MaybeUninit<__SlateAlign16<[Fts4Option; 8]>> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_516: *mut [Fts4Option; 8] =
        std::ptr::addr_of_mut!(__slate_storage_516) as *mut [Fts4Option; 8];
    let mut __slate_storage_2045: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2045: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_2045) as *mut i32;
    let mut __slate_storage_2044: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2044: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_2044) as *mut i32;
    let mut __slate_storage_2043: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2043: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_2043) as *mut i32;
    let mut __slate_storage_2042: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2042: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_2042) as *mut i32;
    let mut __slate_storage_2032: std::mem::MaybeUninit<bool> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2032: *mut bool = std::ptr::addr_of_mut!(__slate_storage_2032) as *mut bool;
    let mut __slate_storage_2031: std::mem::MaybeUninit<bool> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2031: *mut bool = std::ptr::addr_of_mut!(__slate_storage_2031) as *mut bool;
    // Check if this is a tokenizer specification
    let mut __slate_storage_2030: std::mem::MaybeUninit<bool> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2030: *mut bool = std::ptr::addr_of_mut!(__slate_storage_2030) as *mut bool;
    let mut __slate_storage_514: std::mem::MaybeUninit<*mut i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_514: *mut *mut i8 =
        std::ptr::addr_of_mut!(__slate_storage_514) as *mut *mut i8;
    let mut __slate_storage_513: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_513: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_513) as *mut i32;
    let mut __slate_storage_512: std::mem::MaybeUninit<*const i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_512: *mut *const i8 =
        std::ptr::addr_of_mut!(__slate_storage_512) as *mut *const i8; // Size of azNotindexed[] array
    let mut __slate_storage_511: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_511: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_511) as *mut i32; // The set of notindexed= columns
    let mut __slate_storage_510: std::mem::MaybeUninit<*mut *mut i8> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_510: *mut *mut *mut i8 =
        std::ptr::addr_of_mut!(__slate_storage_510) as *mut *mut *mut i8; // languageid=? parameter (or NULL)
    let mut __slate_storage_509: std::mem::MaybeUninit<*mut i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_509: *mut *mut i8 =
        std::ptr::addr_of_mut!(__slate_storage_509) as *mut *mut i8; // content=? parameter (or NULL)
    let mut __slate_storage_508: std::mem::MaybeUninit<*mut i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_508: *mut *mut i8 =
        std::ptr::addr_of_mut!(__slate_storage_508) as *mut *mut i8; // uncompress=? parameter (or NULL)
    let mut __slate_storage_507: std::mem::MaybeUninit<*mut i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_507: *mut *mut i8 =
        std::ptr::addr_of_mut!(__slate_storage_507) as *mut *mut i8; // compress=? parameter (or NULL)
    let mut __slate_storage_506: std::mem::MaybeUninit<*mut i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_506: *mut *mut i8 =
        std::ptr::addr_of_mut!(__slate_storage_506) as *mut *mut i8; // Prefix parameter value (or NULL)
    let mut __slate_storage_505: std::mem::MaybeUninit<*mut i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_505: *mut *mut i8 =
        std::ptr::addr_of_mut!(__slate_storage_505) as *mut *mut i8; // True to store descending indexes
    let mut __slate_storage_504: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_504: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_504) as *mut i32;
    // The results of parsing supported FTS4 key=value options:
    // True to omit %_docsize table
    let mut __slate_storage_503: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_503: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_503) as *mut i32; // Array of indexes for this table
    let mut __slate_storage_502: std::mem::MaybeUninit<*mut Fts3Index> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_502: *mut *mut Fts3Index =
        std::ptr::addr_of_mut!(__slate_storage_502) as *mut *mut Fts3Index; // Size of aIndex[] array
    let mut __slate_storage_501: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_501: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_501) as *mut i32; // Tokenizer for this table
    let mut __slate_storage_500: std::mem::MaybeUninit<*mut sqlite3_tokenizer> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_500: *mut *mut sqlite3_tokenizer =
        std::ptr::addr_of_mut!(__slate_storage_500) as *mut *mut sqlite3_tokenizer; // Array of column names
    let mut __slate_storage_499: std::mem::MaybeUninit<*mut *const i8> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_499: *mut *mut *const i8 =
        std::ptr::addr_of_mut!(__slate_storage_499) as *mut *mut *const i8; // True for FTS4, false for FTS3
    let mut __slate_storage_498: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_498: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_498) as *mut i32; // Bytes required to hold table name
    let mut __slate_storage_497: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_497: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_497) as *mut i32; // Bytes required to hold database name
    let mut __slate_storage_496: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_496: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_496) as *mut i32; // Space for holding column names
    let mut __slate_storage_495: std::mem::MaybeUninit<*mut i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_495: *mut *mut i8 =
        std::ptr::addr_of_mut!(__slate_storage_495) as *mut *mut i8; // Number of columns in the FTS table
    let mut __slate_storage_494: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_494: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_494) as *mut i32; // Bytes required to hold all column names
    let mut __slate_storage_493: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_493: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_493) as *mut i32; // Column index
    let mut __slate_storage_492: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_492: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_492) as *mut i32; // Size of allocation used for *p
    let mut __slate_storage_491: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_491: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_491) as *mut i64; // Iterator variable
    let mut __slate_storage_490: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_490: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_490) as *mut i32; // Return code
    let mut __slate_storage_489: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_489: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_489) as *mut i32; // Pointer to allocated vtab
    let mut __slate_storage_488: std::mem::MaybeUninit<*mut Fts3Table> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_488: *mut *mut Fts3Table =
        std::ptr::addr_of_mut!(__slate_storage_488) as *mut *mut Fts3Table;
    let mut __slate_storage_487: std::mem::MaybeUninit<*mut Fts3Hash> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_487: *mut *mut Fts3Hash =
        std::ptr::addr_of_mut!(__slate_storage_487) as *mut *mut Fts3Hash;
    unsafe {
        '__join_118: {
            std::ptr::write(__slate_slot_487, unsafe {
                std::ptr::addr_of_mut!((*(pAux as *mut Fts3HashWrapper)).hash)
            });
            std::ptr::write(__slate_slot_488, std::ptr::null_mut::<Fts3Table>());
            std::ptr::write(__slate_slot_489, 0 as i32);
            std::ptr::write(__slate_slot_493, 0 as i32);
            std::ptr::write(__slate_slot_494, 0 as i32);
            std::ptr::write(
                __slate_slot_498,
                (((unsafe {
                    *unsafe {
                        unsafe { *unsafe { argv.offset((0 as i32) as isize) } }
                            .offset((3 as i32) as isize)
                    }
                }) as i32)
                    == (52 as i32)) as i32,
            );
            std::ptr::write(__slate_slot_500, std::ptr::null_mut::<sqlite3_tokenizer>());
            std::ptr::write(__slate_slot_501, 0 as i32);
            std::ptr::write(__slate_slot_502, std::ptr::null_mut::<Fts3Index>());
            std::ptr::write(__slate_slot_503, 0 as i32);
            std::ptr::write(__slate_slot_504, 0 as i32);
            std::ptr::write(__slate_slot_505, std::ptr::null_mut::<i8>());
            std::ptr::write(__slate_slot_506, std::ptr::null_mut::<i8>());
            std::ptr::write(__slate_slot_507, std::ptr::null_mut::<i8>());
            std::ptr::write(__slate_slot_508, std::ptr::null_mut::<i8>());
            std::ptr::write(__slate_slot_509, std::ptr::null_mut::<i8>());
            std::ptr::write(__slate_slot_510, std::ptr::null_mut::<*mut i8>());
            std::ptr::write(__slate_slot_511, 0 as i32);
            0 as i32;
            0 as i32;
            *__slate_slot_496 =
                (((unsafe { strlen(unsafe { *unsafe { argv.offset((1 as i32) as isize) } }) })
                    as u32) as i32)
                    + (1 as i32);
            *__slate_slot_497 =
                (((unsafe { strlen(unsafe { *unsafe { argv.offset((2 as i32) as isize) } }) })
                    as u32) as i32)
                    + (1 as i32);
            *__slate_slot_491 = (8 as u64).wrapping_mul(((argc - (2 as i32)) as i64) as u64) as i64;
            *__slate_slot_499 =
                (unsafe { sqlite3_malloc64(*__slate_slot_491 as u64) }) as *mut *const i8;
            if *__slate_slot_499 != std::ptr::null_mut::<*const i8>() {
                unsafe {
                    memset(
                        *__slate_slot_499 as *mut (),
                        0 as i32,
                        *__slate_slot_491 as u64,
                    )
                };
                *__slate_slot_510 =
                    (unsafe { sqlite3_malloc64(*__slate_slot_491 as u64) }) as *mut *mut i8;
            }
        }
        if *__slate_slot_510 != std::ptr::null_mut::<*mut i8>() {
            unsafe {
                memset(
                    *__slate_slot_510 as *mut (),
                    0 as i32,
                    *__slate_slot_491 as u64,
                )
            };
        }
        '__join_9: {
            if !(*__slate_slot_499 != std::ptr::null_mut::<*const i8>())
                || !(*__slate_slot_510 != std::ptr::null_mut::<*mut i8>())
            {
                *__slate_slot_489 = 7 as i32;
            } else {
                // Loop through all of the arguments passed by the user to the FTS3/4
                // module (i.e. all the column names and special arguments). This loop
                // does the following:
                //
                //   + Figures out the number of columns the FTSX table will have, and
                //     the number of bytes of space that must be allocated to store copies
                //     of the column names.
                //
                //   + If there is a tokenizer specification included in the arguments,
                //     initializes the tokenizer pTokenizer.
                *__slate_slot_490 = 3 as i32;
                loop {
                    if *__slate_slot_489 == (0 as i32) && *__slate_slot_490 < argc {
                        '__join_110: {
                            std::ptr::write(__slate_slot_512, unsafe {
                                *unsafe { argv.offset(*__slate_slot_490 as isize) }
                            });
                            if !(*__slate_slot_500 != std::ptr::null_mut::<sqlite3_tokenizer>())
                                && (unsafe { strlen(*__slate_slot_512) })
                                    > (((8 as i32) as i64) as u64)
                            {
                                *__slate_slot_2030 = (0 as i32)
                                    == unsafe {
                                        sqlite3_strnicmp(
                                            *__slate_slot_512,
                                            (b"tokenize\0".as_ptr() as *mut i8) as *const i8,
                                            8 as i32,
                                        )
                                    };
                            } else {
                                *__slate_slot_2030 = false as bool;
                            }
                        }
                        if *__slate_slot_2030 {
                            *__slate_slot_2031 = (0 as i32)
                                == unsafe {
                                    sqlite3Fts3IsIdChar(unsafe {
                                        *unsafe { (*__slate_slot_512).offset((8 as i32) as isize) }
                                    })
                                };
                        } else {
                            *__slate_slot_2031 = false as bool;
                        }
                        if *__slate_slot_2031 {
                            *__slate_slot_489 = unsafe {
                                sqlite3Fts3InitTokenizer(
                                    *__slate_slot_487,
                                    unsafe { (*__slate_slot_512).offset((9 as i32) as isize) },
                                    std::ptr::addr_of_mut!(*__slate_slot_500),
                                    pzErr,
                                )
                            };
                        } else {
                            if *__slate_slot_498 != (0 as i32) {
                                *__slate_slot_2032 = fts3IsSpecialColumn(
                                    *__slate_slot_512,
                                    std::ptr::addr_of_mut!(*__slate_slot_513),
                                    std::ptr::addr_of_mut!(*__slate_slot_514),
                                ) != (0 as i32);
                            } else {
                                *__slate_slot_2032 = false as bool;
                            }
                            if *__slate_slot_2032 {
                                std::ptr::write(
                                    __slate_slot_516,
                                    [
                                        Fts4Option {
                                            zOpt: (b"matchinfo\0".as_ptr() as *mut i8) as *const i8,
                                            nOpt: 9 as i32,
                                        },
                                        Fts4Option {
                                            zOpt: (b"prefix\0".as_ptr() as *mut i8) as *const i8,
                                            nOpt: 6 as i32,
                                        },
                                        Fts4Option {
                                            zOpt: (b"compress\0".as_ptr() as *mut i8) as *const i8,
                                            nOpt: 8 as i32,
                                        },
                                        Fts4Option {
                                            zOpt: (b"uncompress\0".as_ptr() as *mut i8)
                                                as *const i8,
                                            nOpt: 10 as i32,
                                        },
                                        Fts4Option {
                                            zOpt: (b"order\0".as_ptr() as *mut i8) as *const i8,
                                            nOpt: 5 as i32,
                                        },
                                        Fts4Option {
                                            zOpt: (b"content\0".as_ptr() as *mut i8) as *const i8,
                                            nOpt: 7 as i32,
                                        },
                                        Fts4Option {
                                            zOpt: (b"languageid\0".as_ptr() as *mut i8)
                                                as *const i8,
                                            nOpt: 10 as i32,
                                        },
                                        Fts4Option {
                                            zOpt: (b"notindexed\0".as_ptr() as *mut i8)
                                                as *const i8,
                                            nOpt: 10 as i32,
                                        },
                                    ],
                                );
                                if !(*__slate_slot_514 != std::ptr::null_mut::<i8>()) {
                                    *__slate_slot_489 = 7 as i32;
                                } else {
                                    *__slate_slot_517 = 0 as i32;
                                    loop {
                                        if *__slate_slot_517
                                            < ((((128 as u64) / (16 as u64)) as u32) as i32)
                                        {
                                            std::ptr::write(__slate_slot_518, unsafe {
                                                ((*__slate_slot_516).as_mut_ptr()
                                                    as *mut Fts4Option)
                                                    .offset(*__slate_slot_517 as isize)
                                            });
                                            if *__slate_slot_513
                                                == unsafe { (*(*__slate_slot_518)).nOpt }
                                            {
                                                *__slate_slot_2035 = !((unsafe {
                                                    sqlite3_strnicmp(
                                                        *__slate_slot_512,
                                                        unsafe { (*(*__slate_slot_518)).zOpt },
                                                        unsafe { (*(*__slate_slot_518)).nOpt },
                                                    )
                                                }) != (0 as i32));
                                            } else {
                                                *__slate_slot_2035 = false as bool;
                                            }
                                            if *__slate_slot_2035 {
                                                break;
                                            } else {
                                                std::ptr::write(
                                                    __slate_slot_2033,
                                                    *__slate_slot_517,
                                                );
                                                std::ptr::write(
                                                    __slate_slot_2034,
                                                    *__slate_slot_2033 + (1 as i32),
                                                );
                                                *__slate_slot_517 = *__slate_slot_2034;
                                            }
                                        } else {
                                            break;
                                        }
                                    }
                                    let __t0: i32 = *__slate_slot_517;
                                    if __t0 == (0 as i32) {
                                        if (unsafe { strlen(*__slate_slot_514 as *const i8) })
                                            != (((4 as i32) as i64) as u64)
                                        {
                                            *__slate_slot_2036 = true as bool;
                                        } else {
                                            *__slate_slot_2036 = (unsafe {
                                                sqlite3_strnicmp(
                                                    *__slate_slot_514 as *const i8,
                                                    (b"fts3\0".as_ptr() as *mut i8) as *const i8,
                                                    4 as i32,
                                                )
                                            }) != (0 as i32);
                                        }
                                        if *__slate_slot_2036 {
                                            unsafe {
                                                sqlite3Fts3ErrMsg(
                                                    pzErr,
                                                    (b"unrecognized matchinfo: %s\0".as_ptr()
                                                        as *mut i8)
                                                        as *const i8,
                                                    *__slate_slot_514,
                                                )
                                            };
                                            *__slate_slot_489 = 1 as i32;
                                        }
                                        // MATCHINFO
                                        *__slate_slot_503 = 1 as i32;
                                    } else {
                                        if __t0 == (1 as i32) {
                                            unsafe { sqlite3_free(*__slate_slot_505 as *mut ()) }; // PREFIX
                                            *__slate_slot_505 = *__slate_slot_514;
                                            *__slate_slot_514 = std::ptr::null_mut::<i8>();
                                        } else {
                                            if __t0 == (2 as i32) {
                                                unsafe {
                                                    sqlite3_free(*__slate_slot_506 as *mut ())
                                                }; // COMPRESS
                                                *__slate_slot_506 = *__slate_slot_514;
                                                *__slate_slot_514 = std::ptr::null_mut::<i8>();
                                            } else {
                                                if __t0 == (3 as i32) {
                                                    unsafe {
                                                        sqlite3_free(*__slate_slot_507 as *mut ())
                                                    }; // UNCOMPRESS
                                                    *__slate_slot_507 = *__slate_slot_514;
                                                    *__slate_slot_514 = std::ptr::null_mut::<i8>();
                                                } else {
                                                    if __t0 == (4 as i32) {
                                                        if (unsafe {
                                                            strlen(*__slate_slot_514 as *const i8)
                                                        }) != (((3 as i32) as i64) as u64)
                                                        {
                                                            *__slate_slot_2037 = true as bool;
                                                        } else {
                                                            *__slate_slot_2037 = (unsafe {
                                                                sqlite3_strnicmp(
                                                                    *__slate_slot_514 as *const i8,
                                                                    (b"asc\0".as_ptr() as *mut i8)
                                                                        as *const i8,
                                                                    3 as i32,
                                                                )
                                                            }) != (0 as i32);
                                                        }
                                                        if *__slate_slot_2037 {
                                                            if (unsafe {
                                                                strlen(
                                                                    *__slate_slot_514 as *const i8,
                                                                )
                                                            }) != (((4 as i32) as i64) as u64)
                                                            {
                                                                *__slate_slot_2039 = true as bool;
                                                            } else {
                                                                *__slate_slot_2039 = (unsafe {
                                                                    sqlite3_strnicmp(
                                                                        *__slate_slot_514
                                                                            as *const i8,
                                                                        (b"desc\0".as_ptr()
                                                                            as *mut i8)
                                                                            as *const i8,
                                                                        4 as i32,
                                                                    )
                                                                }) != (0
                                                                    as i32);
                                                            }
                                                            *__slate_slot_2038 = *__slate_slot_2039;
                                                        } else {
                                                            *__slate_slot_2038 = false as bool;
                                                        }
                                                        if *__slate_slot_2038 {
                                                            unsafe {
                                                                sqlite3Fts3ErrMsg(
                                                                    pzErr,
                                                                    (b"unrecognized order: %s\0"
                                                                        .as_ptr()
                                                                        as *mut i8)
                                                                        as *const i8,
                                                                    *__slate_slot_514,
                                                                )
                                                            };
                                                            *__slate_slot_489 = 1 as i32;
                                                        }
                                                        // ORDER
                                                        *__slate_slot_504 = (((unsafe {
                                                            *unsafe {
                                                                (*__slate_slot_514)
                                                                    .offset((0 as i32) as isize)
                                                            }
                                                        })
                                                            as i32)
                                                            == (100 as i32)
                                                            || ((unsafe {
                                                                *unsafe {
                                                                    (*__slate_slot_514)
                                                                        .offset((0 as i32) as isize)
                                                                }
                                                            })
                                                                as i32)
                                                                == (68 as i32))
                                                            as i32;
                                                    } else {
                                                        if __t0 == (5 as i32) {
                                                            unsafe {
                                                                sqlite3_free(
                                                                    *__slate_slot_508 as *mut (),
                                                                )
                                                            }; // CONTENT
                                                            *__slate_slot_508 = *__slate_slot_514;
                                                            *__slate_slot_514 =
                                                                std::ptr::null_mut::<i8>();
                                                        } else {
                                                            if __t0 == (6 as i32) {
                                                                0 as i32; // LANGUAGEID
                                                                unsafe {
                                                                    sqlite3_free(
                                                                        *__slate_slot_509
                                                                            as *mut (),
                                                                    )
                                                                };
                                                                *__slate_slot_509 =
                                                                    *__slate_slot_514;
                                                                *__slate_slot_514 =
                                                                    std::ptr::null_mut::<i8>();
                                                            } else {
                                                                if __t0 == (7 as i32) {
                                                                    std::ptr::write(
                                                                        __slate_slot_2040,
                                                                        *__slate_slot_511,
                                                                    );
                                                                    std::ptr::write(
                                                                        __slate_slot_2041,
                                                                        *__slate_slot_2040
                                                                            + (1 as i32),
                                                                    );
                                                                    *__slate_slot_511 =
                                                                        *__slate_slot_2041;
                                                                    unsafe {
                                                                        *unsafe {
                                                                            (*__slate_slot_510)
                                                                                .offset(
                                                                                *__slate_slot_2040
                                                                                    as isize,
                                                                            )
                                                                        } = *__slate_slot_514;
                                                                    }
                                                                    // NOTINDEXED
                                                                    *__slate_slot_514 =
                                                                        std::ptr::null_mut::<i8>();
                                                                } else {
                                                                    0 as i32;
                                                                    unsafe {
                                                                        sqlite3Fts3ErrMsg(pzErr, (b"unrecognized parameter: %s\0".as_ptr() as *mut i8) as *const i8, *__slate_slot_512)
                                                                    };
                                                                    *__slate_slot_489 = 1 as i32;
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    unsafe { sqlite3_free(*__slate_slot_514 as *mut ()) };
                                }
                            } else {
                                std::ptr::write(__slate_slot_2042, *__slate_slot_493);
                                std::ptr::write(
                                    __slate_slot_2043,
                                    *__slate_slot_2042
                                        + ((unsafe { strlen(*__slate_slot_512) }
                                            .wrapping_add(((1 as i32) as i64) as u64)
                                            as u32)
                                            as i32),
                                );
                                *__slate_slot_493 = *__slate_slot_2043;
                                std::ptr::write(__slate_slot_2044, *__slate_slot_494);
                                std::ptr::write(__slate_slot_2045, *__slate_slot_2044 + (1 as i32));
                                *__slate_slot_494 = *__slate_slot_2045;
                                unsafe {
                                    *unsafe {
                                        (*__slate_slot_499).offset(*__slate_slot_2044 as isize)
                                    } = *__slate_slot_512;
                                }
                            }
                        }
                        // Check if it is an FTS4 special argument.
                        // Otherwise, the argument is a column name.
                        std::ptr::write(__slate_slot_2028, *__slate_slot_490);
                        std::ptr::write(__slate_slot_2029, *__slate_slot_2028 + (1 as i32));
                        *__slate_slot_490 = *__slate_slot_2029;
                    } else {
                        break;
                    }
                }
                '__join_51: {
                    // If a content=xxx option was specified, the following:
                    //
                    // 1. Ignore any compress= and uncompress= options.
                    //
                    // 2. If no column names were specified as part of the CREATE VIRTUAL
                    //    TABLE statement, use all columns from the content table.
                    if *__slate_slot_489 == (0 as i32)
                        && *__slate_slot_508 != std::ptr::null_mut::<i8>()
                    {
                        unsafe { sqlite3_free(*__slate_slot_506 as *mut ()) };
                        unsafe { sqlite3_free(*__slate_slot_507 as *mut ()) };
                        *__slate_slot_506 = std::ptr::null_mut::<i8>();
                        *__slate_slot_507 = std::ptr::null_mut::<i8>();
                        if *__slate_slot_494 == (0 as i32) {
                            unsafe { sqlite3_free(*__slate_slot_499 as *mut ()) };
                            *__slate_slot_499 = std::ptr::null_mut::<*const i8>();
                            *__slate_slot_489 = fts3ContentColumns(
                                db,
                                unsafe { *unsafe { argv.offset((1 as i32) as isize) } },
                                *__slate_slot_508 as *const i8,
                                std::ptr::addr_of_mut!(*__slate_slot_499),
                                std::ptr::addr_of_mut!(*__slate_slot_494),
                                std::ptr::addr_of_mut!(*__slate_slot_493),
                                pzErr,
                            );
                            // If a languageid= option was specified, remove the language id
                            // column from the aCol[] array.
                            if *__slate_slot_489 == (0 as i32)
                                && *__slate_slot_509 != std::ptr::null_mut::<i8>()
                            {
                                *__slate_slot_519 = 0 as i32;
                                loop {
                                    if *__slate_slot_519 < *__slate_slot_494 {
                                        if (unsafe {
                                            sqlite3_stricmp(
                                                *__slate_slot_509 as *const i8,
                                                unsafe {
                                                    *unsafe {
                                                        (*__slate_slot_499)
                                                            .offset(*__slate_slot_519 as isize)
                                                    }
                                                },
                                            )
                                        }) == (0 as i32)
                                        {
                                            break;
                                        } else {
                                            std::ptr::write(__slate_slot_2046, *__slate_slot_519);
                                            std::ptr::write(
                                                __slate_slot_2047,
                                                *__slate_slot_2046 + (1 as i32),
                                            );
                                            *__slate_slot_519 = *__slate_slot_2047;
                                        }
                                    } else {
                                        break '__join_51;
                                    }
                                }
                                *__slate_slot_520 = *__slate_slot_519;
                                loop {
                                    if *__slate_slot_520 < *__slate_slot_494 {
                                        unsafe {
                                            *unsafe {
                                                (*__slate_slot_499)
                                                    .offset(*__slate_slot_520 as isize)
                                            } = unsafe {
                                                *unsafe {
                                                    (*__slate_slot_499).offset(
                                                        (*__slate_slot_520 + (1 as i32)) as isize,
                                                    )
                                                }
                                            };
                                        }
                                        std::ptr::write(__slate_slot_2048, *__slate_slot_520);
                                        std::ptr::write(
                                            __slate_slot_2049,
                                            *__slate_slot_2048 + (1 as i32),
                                        );
                                        *__slate_slot_520 = *__slate_slot_2049;
                                    } else {
                                        break;
                                    }
                                }
                                std::ptr::write(__slate_slot_2050, *__slate_slot_494);
                                std::ptr::write(__slate_slot_2051, *__slate_slot_2050 - (1 as i32));
                                *__slate_slot_494 = *__slate_slot_2051;
                            }
                        }
                    }
                }
                if *__slate_slot_489 != (0 as i32) {
                } else {
                    if *__slate_slot_494 == (0 as i32) {
                        0 as i32;
                        unsafe {
                            *unsafe { (*__slate_slot_499).offset((0 as i32) as isize) } =
                                (b"content\0".as_ptr() as *mut i8) as *const i8;
                        }
                        *__slate_slot_493 = 8 as i32;
                        *__slate_slot_494 = 1 as i32;
                    }
                    if *__slate_slot_500 == std::ptr::null_mut::<sqlite3_tokenizer>() {
                        *__slate_slot_489 = unsafe {
                            sqlite3Fts3InitTokenizer(
                                *__slate_slot_487,
                                (b"simple\0".as_ptr() as *mut i8) as *const i8,
                                std::ptr::addr_of_mut!(*__slate_slot_500),
                                pzErr,
                            )
                        };
                        if *__slate_slot_489 != (0 as i32) {
                            break '__join_9;
                        }
                    }
                    0 as i32;
                    *__slate_slot_489 = fts3PrefixParameter(
                        *__slate_slot_505 as *const i8,
                        std::ptr::addr_of_mut!(*__slate_slot_501),
                        std::ptr::addr_of_mut!(*__slate_slot_502),
                    );
                    if *__slate_slot_489 == (1 as i32) {
                        0 as i32;
                        unsafe {
                            sqlite3Fts3ErrMsg(
                                pzErr,
                                (b"error parsing prefix parameter: %s\0".as_ptr() as *mut i8)
                                    as *const i8,
                                *__slate_slot_505,
                            )
                        };
                    }
                    if *__slate_slot_489 != (0 as i32) {
                    } else {
                        // Allocate and populate the Fts3Table structure.
                        *__slate_slot_491 = (528 as u64)
                            .wrapping_add(
                                ((*__slate_slot_494 as i64) as u64).wrapping_mul(8 as u64),
                            )
                            .wrapping_add(
                                ((*__slate_slot_501 as i64) as u64).wrapping_mul(40 as u64),
                            )
                            .wrapping_add(
                                ((*__slate_slot_494 as i64) as u64).wrapping_mul(1 as u64),
                            )
                            .wrapping_add((*__slate_slot_497 as i64) as u64)
                            .wrapping_add((*__slate_slot_496 as i64) as u64)
                            .wrapping_add((*__slate_slot_493 as i64) as u64)
                            as i64; // Fts3Table
                        // azColumn
                        // aIndex
                        // abNotindexed
                        // zName
                        // zDb
                        // Space for azColumn strings
                        *__slate_slot_488 = (unsafe { sqlite3_malloc64(*__slate_slot_491 as u64) })
                            as *mut Fts3Table;
                        if *__slate_slot_488 == std::ptr::null_mut::<Fts3Table>() {
                            *__slate_slot_489 = 7 as i32;
                        } else {
                            unsafe {
                                memset(
                                    *__slate_slot_488 as *mut (),
                                    0 as i32,
                                    *__slate_slot_491 as u64,
                                )
                            };
                            unsafe {
                                (*(*__slate_slot_488)).db = db;
                            }
                            unsafe {
                                (*(*__slate_slot_488)).nColumn = *__slate_slot_494;
                            }
                            unsafe {
                                (*(*__slate_slot_488)).nPendingData = 0 as i32;
                            }
                            unsafe {
                                (*(*__slate_slot_488)).azColumn =
                                    (unsafe { (*__slate_slot_488).offset((1 as i32) as isize) })
                                        as *mut *mut i8;
                            }
                            unsafe {
                                (*(*__slate_slot_488)).pTokenizer = *__slate_slot_500;
                            }
                            unsafe {
                                (*(*__slate_slot_488)).nMaxPendingData =
                                    (1 as i32) * (1024 as i32) * (1024 as i32);
                            }
                            unsafe {
                                (*(*__slate_slot_488)).bHasDocsize = (*__slate_slot_498
                                    != (0 as i32)
                                    && *__slate_slot_503 == (0 as i32))
                                    as u8;
                            }
                            unsafe {
                                (*(*__slate_slot_488)).bHasStat = (*__slate_slot_498 as i8) as u8;
                            }
                            unsafe {
                                (*(*__slate_slot_488)).bFts4 = (*__slate_slot_498 as i8) as u8;
                            }
                            unsafe {
                                (*(*__slate_slot_488)).bDescIdx = (*__slate_slot_504 as i8) as u8;
                            }
                            unsafe {
                                (*(*__slate_slot_488)).nAutoincrmerge = 255 as i32;
                            }
                            // 0xff means setting unknown
                            unsafe {
                                (*(*__slate_slot_488)).zContentTbl = *__slate_slot_508;
                            }
                            unsafe {
                                (*(*__slate_slot_488)).zLanguageid = *__slate_slot_509;
                            }
                            *__slate_slot_508 = std::ptr::null_mut::<i8>();
                            *__slate_slot_509 = std::ptr::null_mut::<i8>();
                            {}
                            {}
                            unsafe {
                                (*(*__slate_slot_488)).aIndex = (unsafe {
                                    unsafe { (*(*__slate_slot_488)).azColumn }
                                        .offset(*__slate_slot_494 as isize)
                                })
                                    as *mut Fts3Index;
                            }
                            unsafe {
                                memcpy(
                                    (unsafe { (*(*__slate_slot_488)).aIndex }) as *mut (),
                                    *__slate_slot_502 as *const (),
                                    (40 as u64).wrapping_mul((*__slate_slot_501 as i64) as u64),
                                )
                            };
                            unsafe {
                                (*(*__slate_slot_488)).nIndex = *__slate_slot_501;
                            }
                            *__slate_slot_490 = 0 as i32;
                            loop {
                                if *__slate_slot_490 < *__slate_slot_501 {
                                    unsafe {
                                        sqlite3Fts3HashInit(
                                            unsafe {
                                                std::ptr::addr_of_mut!(
                                                    (*unsafe {
                                                        unsafe { (*(*__slate_slot_488)).aIndex }
                                                            .offset(*__slate_slot_490 as isize)
                                                    })
                                                    .hPending
                                                )
                                            },
                                            (1 as i32) as i8,
                                            (1 as i32) as i8,
                                        )
                                    };
                                    std::ptr::write(__slate_slot_2052, *__slate_slot_490);
                                    std::ptr::write(
                                        __slate_slot_2053,
                                        *__slate_slot_2052 + (1 as i32),
                                    );
                                    *__slate_slot_490 = *__slate_slot_2053;
                                } else {
                                    break;
                                }
                            }
                            unsafe {
                                (*(*__slate_slot_488)).abNotindexed = (unsafe {
                                    unsafe { (*(*__slate_slot_488)).aIndex }
                                        .offset(*__slate_slot_501 as isize)
                                })
                                    as *mut u8;
                            }
                            // Fill in the zName and zDb fields of the vtab structure.
                            *__slate_slot_495 = (unsafe {
                                unsafe { (*(*__slate_slot_488)).abNotindexed }
                                    .offset(*__slate_slot_494 as isize)
                            }) as *mut i8;
                            unsafe {
                                (*(*__slate_slot_488)).zName = *__slate_slot_495 as *const i8;
                            }
                            unsafe {
                                memcpy(
                                    *__slate_slot_495 as *mut (),
                                    (unsafe { *unsafe { argv.offset((2 as i32) as isize) } })
                                        as *const (),
                                    (*__slate_slot_497 as i64) as u64,
                                )
                            };
                            std::ptr::write(__slate_slot_2054, *__slate_slot_495);
                            std::ptr::write(__slate_slot_2055, unsafe {
                                (*__slate_slot_2054).offset(*__slate_slot_497 as isize)
                            });
                            *__slate_slot_495 = *__slate_slot_2055;
                            unsafe {
                                (*(*__slate_slot_488)).zDb = *__slate_slot_495 as *const i8;
                            }
                            unsafe {
                                memcpy(
                                    *__slate_slot_495 as *mut (),
                                    (unsafe { *unsafe { argv.offset((1 as i32) as isize) } })
                                        as *const (),
                                    (*__slate_slot_496 as i64) as u64,
                                )
                            };
                            std::ptr::write(__slate_slot_2056, *__slate_slot_495);
                            std::ptr::write(__slate_slot_2057, unsafe {
                                (*__slate_slot_2056).offset(*__slate_slot_496 as isize)
                            });
                            *__slate_slot_495 = *__slate_slot_2057;
                            // Fill in the azColumn array
                            *__slate_slot_492 = 0 as i32;
                            loop {
                                if *__slate_slot_492 < *__slate_slot_494 {
                                    std::ptr::write(__slate_slot_522, 0 as i32);
                                    *__slate_slot_521 = (unsafe {
                                        sqlite3Fts3NextToken(
                                            unsafe {
                                                *unsafe {
                                                    (*__slate_slot_499)
                                                        .offset(*__slate_slot_492 as isize)
                                                }
                                            },
                                            std::ptr::addr_of_mut!(*__slate_slot_522),
                                        )
                                    })
                                        as *mut i8;
                                    if *__slate_slot_522 > (0 as i32) {
                                        unsafe {
                                            memcpy(
                                                *__slate_slot_495 as *mut (),
                                                *__slate_slot_521 as *const (),
                                                (*__slate_slot_522 as i64) as u64,
                                            )
                                        };
                                    }
                                    unsafe {
                                        *unsafe {
                                            (*__slate_slot_495).offset(*__slate_slot_522 as isize)
                                        } = (0 as i32) as i8;
                                    }
                                    sqlite3Fts3Dequote(*__slate_slot_495);
                                    unsafe {
                                        *unsafe {
                                            unsafe { (*(*__slate_slot_488)).azColumn }
                                                .offset(*__slate_slot_492 as isize)
                                        } = *__slate_slot_495;
                                    }
                                    std::ptr::write(__slate_slot_2060, *__slate_slot_495);
                                    std::ptr::write(__slate_slot_2061, unsafe {
                                        (*__slate_slot_2060)
                                            .offset((*__slate_slot_522 + (1 as i32)) as isize)
                                    });
                                    *__slate_slot_495 = *__slate_slot_2061;
                                    0 as i32;
                                    std::ptr::write(__slate_slot_2058, *__slate_slot_492);
                                    std::ptr::write(
                                        __slate_slot_2059,
                                        *__slate_slot_2058 + (1 as i32),
                                    );
                                    *__slate_slot_492 = *__slate_slot_2059;
                                } else {
                                    break;
                                }
                            }
                            // Fill in the abNotindexed array
                            *__slate_slot_492 = 0 as i32;
                            loop {
                                if *__slate_slot_492 < *__slate_slot_494 {
                                    std::ptr::write(
                                        __slate_slot_523,
                                        ((unsafe {
                                            strlen(
                                                (unsafe {
                                                    *unsafe {
                                                        unsafe { (*(*__slate_slot_488)).azColumn }
                                                            .offset(*__slate_slot_492 as isize)
                                                    }
                                                })
                                                    as *const i8,
                                            )
                                        }) as u32) as i32,
                                    );
                                    *__slate_slot_490 = 0 as i32;
                                    loop {
                                        if *__slate_slot_490 < *__slate_slot_511 {
                                            std::ptr::write(__slate_slot_524, unsafe {
                                                *unsafe {
                                                    (*__slate_slot_510)
                                                        .offset(*__slate_slot_490 as isize)
                                                }
                                            });
                                            if *__slate_slot_524 != std::ptr::null_mut::<i8>()
                                                && *__slate_slot_523
                                                    == (((unsafe {
                                                        strlen(*__slate_slot_524 as *const i8)
                                                    })
                                                        as u32)
                                                        as i32)
                                            {
                                                *__slate_slot_2066 = (0 as i32)
                                                    == unsafe {
                                                        sqlite3_strnicmp(
                                                            (unsafe {
                                                                *unsafe {
                                                                    unsafe {
                                                                        (*(*__slate_slot_488))
                                                                            .azColumn
                                                                    }
                                                                    .offset(
                                                                        *__slate_slot_492 as isize,
                                                                    )
                                                                }
                                                            })
                                                                as *const i8,
                                                            *__slate_slot_524 as *const i8,
                                                            *__slate_slot_523,
                                                        )
                                                    };
                                            } else {
                                                *__slate_slot_2066 = false as bool;
                                            }
                                            if *__slate_slot_2066 {
                                                unsafe {
                                                    *unsafe {
                                                        unsafe {
                                                            (*(*__slate_slot_488)).abNotindexed
                                                        }
                                                        .offset(*__slate_slot_492 as isize)
                                                    } = ((1 as i32) as i8) as u8;
                                                }
                                                unsafe {
                                                    sqlite3_free(*__slate_slot_524 as *mut ())
                                                };
                                                unsafe {
                                                    *unsafe {
                                                        (*__slate_slot_510)
                                                            .offset(*__slate_slot_490 as isize)
                                                    } = std::ptr::null_mut::<i8>();
                                                }
                                            }
                                            std::ptr::write(__slate_slot_2064, *__slate_slot_490);
                                            std::ptr::write(
                                                __slate_slot_2065,
                                                *__slate_slot_2064 + (1 as i32),
                                            );
                                            *__slate_slot_490 = *__slate_slot_2065;
                                        } else {
                                            break;
                                        }
                                    }
                                    std::ptr::write(__slate_slot_2062, *__slate_slot_492);
                                    std::ptr::write(
                                        __slate_slot_2063,
                                        *__slate_slot_2062 + (1 as i32),
                                    );
                                    *__slate_slot_492 = *__slate_slot_2063;
                                } else {
                                    break;
                                }
                            }
                            *__slate_slot_490 = 0 as i32;
                            loop {
                                if *__slate_slot_490 < *__slate_slot_511 {
                                    if (unsafe {
                                        *unsafe {
                                            (*__slate_slot_510).offset(*__slate_slot_490 as isize)
                                        }
                                    }) != std::ptr::null_mut::<i8>()
                                    {
                                        unsafe {
                                            sqlite3Fts3ErrMsg(
                                                pzErr,
                                                (b"no such column: %s\0".as_ptr() as *mut i8)
                                                    as *const i8,
                                                unsafe {
                                                    *unsafe {
                                                        (*__slate_slot_510)
                                                            .offset(*__slate_slot_490 as isize)
                                                    }
                                                },
                                            )
                                        };
                                        *__slate_slot_489 = 1 as i32;
                                    }
                                    std::ptr::write(__slate_slot_2067, *__slate_slot_490);
                                    std::ptr::write(
                                        __slate_slot_2068,
                                        *__slate_slot_2067 + (1 as i32),
                                    );
                                    *__slate_slot_490 = *__slate_slot_2068;
                                } else {
                                    break;
                                }
                            }
                            if *__slate_slot_489 == (0 as i32)
                                && ((*__slate_slot_506 == std::ptr::null_mut::<i8>()) as i32)
                                    != ((*__slate_slot_507 == std::ptr::null_mut::<i8>()) as i32)
                            {
                                std::ptr::write(
                                    __slate_slot_525,
                                    (if *__slate_slot_506 == std::ptr::null_mut::<i8>() {
                                        b"compress\0".as_ptr() as *mut i8
                                    } else {
                                        b"uncompress\0".as_ptr() as *mut i8
                                    }) as *const i8,
                                );
                                *__slate_slot_489 = 1 as i32;
                                unsafe {
                                    sqlite3Fts3ErrMsg(
                                        pzErr,
                                        (b"missing %s parameter in fts4 constructor\0".as_ptr()
                                            as *mut i8)
                                            as *const i8,
                                        *__slate_slot_525,
                                    )
                                };
                            }
                            unsafe {
                                (*(*__slate_slot_488)).zReadExprlist = fts3ReadExprList(
                                    *__slate_slot_488,
                                    *__slate_slot_507 as *const i8,
                                    std::ptr::addr_of_mut!(*__slate_slot_489),
                                );
                            }
                            unsafe {
                                (*(*__slate_slot_488)).zWriteExprlist = fts3WriteExprList(
                                    *__slate_slot_488,
                                    *__slate_slot_506 as *const i8,
                                    std::ptr::addr_of_mut!(*__slate_slot_489),
                                );
                            }
                            if *__slate_slot_489 != (0 as i32) {
                            } else {
                                // If this is an xCreate call, create the underlying tables in the
                                // database. TODO: For xConnect(), it could verify that said tables exist.
                                if isCreate != (0 as i32) {
                                    *__slate_slot_489 = fts3CreateTables(*__slate_slot_488);
                                }
                                // Check to see if a legacy fts3 table has been "upgraded" by the
                                // addition of a %_stat table so that it can use incremental merge.
                                if !(*__slate_slot_498 != (0 as i32)) && !(isCreate != (0 as i32)) {
                                    unsafe {
                                        (*(*__slate_slot_488)).bHasStat = ((2 as i32) as i8) as u8;
                                    }
                                }
                                // Figure out the page-size for the database. This is required in order to
                                // estimate the cost of loading large doclists from the database.
                                fts3DatabasePageSize(
                                    std::ptr::addr_of_mut!(*__slate_slot_489),
                                    *__slate_slot_488,
                                );
                                unsafe {
                                    (*(*__slate_slot_488)).nNodeSize =
                                        (unsafe { (*(*__slate_slot_488)).nPgsz }) - (35 as i32);
                                }
                                // Declare the table schema to SQLite.
                                fts3DeclareVtab(
                                    std::ptr::addr_of_mut!(*__slate_slot_489),
                                    *__slate_slot_488,
                                );
                            }
                        }
                    }
                }
            }
        }
        unsafe { sqlite3_free(*__slate_slot_505 as *mut ()) };
        unsafe { sqlite3_free(*__slate_slot_502 as *mut ()) };
        unsafe { sqlite3_free(*__slate_slot_506 as *mut ()) };
        unsafe { sqlite3_free(*__slate_slot_507 as *mut ()) };
        unsafe { sqlite3_free(*__slate_slot_508 as *mut ()) };
        unsafe { sqlite3_free(*__slate_slot_509 as *mut ()) };
        *__slate_slot_490 = 0 as i32;
        loop {
            if *__slate_slot_490 < *__slate_slot_511 {
                unsafe {
                    sqlite3_free(
                        (unsafe {
                            *unsafe { (*__slate_slot_510).offset(*__slate_slot_490 as isize) }
                        }) as *mut (),
                    )
                };
                std::ptr::write(__slate_slot_2069, *__slate_slot_490);
                std::ptr::write(__slate_slot_2070, *__slate_slot_2069 + (1 as i32));
                *__slate_slot_490 = *__slate_slot_2070;
            } else {
                break;
            }
        }
        unsafe { sqlite3_free(*__slate_slot_499 as *mut ()) };
        unsafe { sqlite3_free(*__slate_slot_510 as *mut ()) };
        if *__slate_slot_489 != (0 as i32) {
            if *__slate_slot_488 != std::ptr::null_mut::<Fts3Table>() {
                fts3DisconnectMethod(*__slate_slot_488 as *mut sqlite3_vtab);
            } else {
                if *__slate_slot_500 != std::ptr::null_mut::<sqlite3_tokenizer>() {
                    unsafe {
                        unsafe { (*unsafe { (*(*__slate_slot_500)).pModule }).xDestroy }.unwrap()(
                            *__slate_slot_500,
                        )
                    };
                }
            }
        } else {
            0 as i32;
            unsafe {
                *ppVTab = unsafe { std::ptr::addr_of_mut!((*(*__slate_slot_488)).base) };
            }
        }
        return *__slate_slot_489;
    }
    return unsafe { std::mem::zeroed() };
}

/// The xConnect() and xCreate() methods for the virtual table. All the
/// work is done in function fts3InitVtab().
///
/// # Arguments
///
/// * `db` - Database connection
/// * `pAux` - Pointer to tokenizer hash table
/// * `argc` - Number of elements in argv array
/// * `argv` - xCreate/xConnect argument array
/// * `ppVtab` - OUT: New sqlite3_vtab object
/// * `pzErr` - OUT: sqlite3_malloc'd error message
#[unsafe(link_section = ".text.slate_distinct.fts3.fts3ConnectMethod")]
extern "C-unwind" fn fts3ConnectMethod(
    mut db: *mut sqlite3,
    mut pAux: *mut (),
    mut argc: i32,
    mut argv: *const *const i8,
    mut ppVtab: *mut *mut sqlite3_vtab,
    mut pzErr: *mut *mut i8,
) -> i32 {
    return fts3InitVtab(0 as i32, db, pAux, argc, argv, ppVtab, pzErr);
}

/// # Arguments
///
/// * `db` - Database connection
/// * `pAux` - Pointer to tokenizer hash table
/// * `argc` - Number of elements in argv array
/// * `argv` - xCreate/xConnect argument array
/// * `ppVtab` - OUT: New sqlite3_vtab object
/// * `pzErr` - OUT: sqlite3_malloc'd error message
#[unsafe(link_section = ".text.slate_distinct.fts3.fts3CreateMethod")]
extern "C-unwind" fn fts3CreateMethod(
    mut db: *mut sqlite3,
    mut pAux: *mut (),
    mut argc: i32,
    mut argv: *const *const i8,
    mut ppVtab: *mut *mut sqlite3_vtab,
    mut pzErr: *mut *mut i8,
) -> i32 {
    return fts3InitVtab(1 as i32, db, pAux, argc, argv, ppVtab, pzErr);
}

/// Set the pIdxInfo->estimatedRows variable to nRow. Unless this
/// extension is currently being used by a version of SQLite too old to
/// support estimatedRows. In that case this function is a no-op.
fn fts3SetEstimatedRows(mut pIdxInfo: *mut sqlite3_index_info, mut nRow: i64) {
    if (unsafe { sqlite3_libversion_number() }) >= (3008002 as i32) {
        unsafe {
            (*pIdxInfo).estimatedRows = nRow;
        }
    }
}

/// Set the SQLITE_INDEX_SCAN_UNIQUE flag in pIdxInfo->flags. Unless this
/// extension is currently being used by a version of SQLite too old to
/// support index-info flags. In that case this function is a no-op.
fn fts3SetUniqueFlag(mut pIdxInfo: *mut sqlite3_index_info) {
    if (unsafe { sqlite3_libversion_number() }) >= (3008012 as i32) {
        let __v2071: *mut sqlite3_index_info = pIdxInfo;
        let __v2072: i32 = unsafe { (*__v2071).idxFlags };
        let __v2073: i32 = __v2072 | (1 as i32);
        unsafe {
            (*__v2071).idxFlags = __v2073;
        }
    }
}

/// Implementation of the xBestIndex method for FTS3 tables. There
/// are three possible strategies, in order of preference:
///
///   1. Direct lookup by rowid or docid.
///   2. Full-text search using a MATCH operator on a non-docid column.
///   3. Linear scan of %_content table.
#[unsafe(link_section = ".text.slate_distinct.fts3.fts3BestIndexMethod")]
extern "C-unwind" fn fts3BestIndexMethod(
    mut pVTab: *mut sqlite3_vtab,
    mut pInfo: *mut sqlite3_index_info,
) -> i32 {
    let mut p: *mut Fts3Table = pVTab as *mut Fts3Table;
    let mut i: i32 = 0 as i32; // Iterator variable
    let mut iCons: i32 = -(1 as i32); // Index of constraint to use
    let mut iLangidCons: i32 = -(1 as i32); // Index of langid=x constraint, if present
    let mut iDocidGe: i32 = -(1 as i32); // Index of docid>=x constraint, if present
    let mut iDocidLe: i32 = -(1 as i32); // Index of docid<=x constraint, if present
    let mut iIdx: i32 = 0 as i32;
    if (unsafe { (*p).bLock }) != (0 as i32) {
        return 1 as i32;
    }
    // By default use a full table scan. This is an expensive option,
    // so search through the constraints to see if a more efficient
    // strategy is possible.
    unsafe {
        (*pInfo).idxNum = 0 as i32;
    }
    unsafe {
        (*pInfo).estimatedCost = (5000000 as i32) as f64;
    }
    i = 0 as i32;
    '__slate_break_1707: loop {
        if !(i < unsafe { (*pInfo).nConstraint }) {
            break;
        }
        let mut bDocid: i32 = 0 as i32; // True if this constraint is on docid
        let mut pCons: *mut sqlite3_index_constraint =
            unsafe { unsafe { (*pInfo).aConstraint }.offset(i as isize) };
        if (((unsafe { (*pCons).usable }) as u32) as i32) == (0 as i32) {
            if (((unsafe { (*pCons).op }) as u32) as i32) == (64 as i32) {
                // There exists an unusable MATCH constraint. This means that if
                // the planner does elect to use the results of this call as part
                // of the overall query plan the user will see an "unable to use
                // function MATCH in the requested context" error. To discourage
                // this, return a very high cost here.
                unsafe {
                    (*pInfo).idxNum = 0 as i32;
                }
                unsafe {
                    (*pInfo).estimatedCost = 1e50f64;
                }
                fts3SetEstimatedRows(pInfo, ((1 as i32) as i64) << (50 as i32));
                return 0 as i32;
            }
        } else {
            bDocid = ((unsafe { (*pCons).iColumn }) < (0 as i32)
                || (unsafe { (*pCons).iColumn }) == (unsafe { (*p).nColumn }) + (1 as i32))
                as i32;
            // A direct lookup on the rowid or docid column. Assign a cost of 1.0.
            if iCons < (0 as i32)
                && (((unsafe { (*pCons).op }) as u32) as i32) == (2 as i32)
                && bDocid != (0 as i32)
            {
                unsafe {
                    (*pInfo).idxNum = 1 as i32;
                }
                unsafe {
                    (*pInfo).estimatedCost = 1.0f64;
                }
                iCons = i;
            }
            // A MATCH constraint. Use a full-text search.
            //
            // If there is more than one MATCH constraint available, use the first
            // one encountered. If there is both a MATCH constraint and a direct
            // rowid/docid lookup, prefer the MATCH strategy. This is done even
            // though the rowid/docid lookup is faster than a MATCH query, selecting
            // it would lead to an "unable to use function MATCH in the requested
            // context" error.
            if (((unsafe { (*pCons).op }) as u32) as i32) == (64 as i32)
                && (unsafe { (*pCons).iColumn }) >= (0 as i32)
                && (unsafe { (*pCons).iColumn }) <= unsafe { (*p).nColumn }
            {
                unsafe {
                    (*pInfo).idxNum = (2 as i32) + unsafe { (*pCons).iColumn };
                }
                unsafe {
                    (*pInfo).estimatedCost = 2.0f64;
                }
                iCons = i;
            }
            // Equality constraint on the langid column
            if (((unsafe { (*pCons).op }) as u32) as i32) == (2 as i32)
                && (unsafe { (*pCons).iColumn }) == (unsafe { (*p).nColumn }) + (2 as i32)
            {
                iLangidCons = i;
            }
            if bDocid != (0 as i32) {
                '__slate_break_1708: {
                    match ((unsafe { (*pCons).op }) as u32) as i32 {
                        32 | 4 => {
                            iDocidGe = i;
                        }
                        8 | 16 => {
                            iDocidLe = i;
                        }
                        _ => {}
                    }
                }
            }
        }
        let __v2074: i32 = i;
        let __v2075: i32 = __v2074 + (1 as i32);
        i = __v2075;
    }
    // If using a docid=? or rowid=? strategy, set the UNIQUE flag.
    if (unsafe { (*pInfo).idxNum }) == (1 as i32) {
        fts3SetUniqueFlag(pInfo);
    }
    iIdx = 1 as i32;
    if iCons >= (0 as i32) {
        let __v2076: i32 = iIdx;
        let __v2077: i32 = __v2076 + (1 as i32);
        iIdx = __v2077;
        unsafe {
            (*unsafe { unsafe { (*pInfo).aConstraintUsage }.offset(iCons as isize) }).argvIndex =
                __v2076;
        }
        unsafe {
            (*unsafe { unsafe { (*pInfo).aConstraintUsage }.offset(iCons as isize) }).omit =
                ((1 as i32) as i8) as u8;
        }
    }
    if iLangidCons >= (0 as i32) {
        let __v2078: *mut sqlite3_index_info = pInfo;
        let __v2079: i32 = unsafe { (*__v2078).idxNum };
        let __v2080: i32 = __v2079 | (65536 as i32);
        unsafe {
            (*__v2078).idxNum = __v2080;
        }
        let __v2081: i32 = iIdx;
        let __v2082: i32 = __v2081 + (1 as i32);
        iIdx = __v2082;
        unsafe {
            (*unsafe { unsafe { (*pInfo).aConstraintUsage }.offset(iLangidCons as isize) })
                .argvIndex = __v2081;
        }
    }
    if iDocidGe >= (0 as i32) {
        let __v2083: *mut sqlite3_index_info = pInfo;
        let __v2084: i32 = unsafe { (*__v2083).idxNum };
        let __v2085: i32 = __v2084 | (131072 as i32);
        unsafe {
            (*__v2083).idxNum = __v2085;
        }
        let __v2086: i32 = iIdx;
        let __v2087: i32 = __v2086 + (1 as i32);
        iIdx = __v2087;
        unsafe {
            (*unsafe { unsafe { (*pInfo).aConstraintUsage }.offset(iDocidGe as isize) })
                .argvIndex = __v2086;
        }
    }
    if iDocidLe >= (0 as i32) {
        let __v2088: *mut sqlite3_index_info = pInfo;
        let __v2089: i32 = unsafe { (*__v2088).idxNum };
        let __v2090: i32 = __v2089 | (262144 as i32);
        unsafe {
            (*__v2088).idxNum = __v2090;
        }
        let __v2091: i32 = iIdx;
        let __v2092: i32 = __v2091 + (1 as i32);
        iIdx = __v2092;
        unsafe {
            (*unsafe { unsafe { (*pInfo).aConstraintUsage }.offset(iDocidLe as isize) })
                .argvIndex = __v2091;
        }
    }
    // Regardless of the strategy selected, FTS can deliver rows in rowid (or
    // docid) order. Both ascending and descending are possible.
    if (unsafe { (*pInfo).nOrderBy }) == (1 as i32) {
        let mut pOrder: *mut sqlite3_index_orderby =
            unsafe { unsafe { (*pInfo).aOrderBy }.offset((0 as i32) as isize) };
        if (unsafe { (*pOrder).iColumn }) < (0 as i32)
            || (unsafe { (*pOrder).iColumn }) == (unsafe { (*p).nColumn }) + (1 as i32)
        {
            if (unsafe { (*pOrder).desc }) != (0 as u8) {
                unsafe {
                    (*pInfo).idxStr = b"DESC\0".as_ptr() as *mut i8;
                }
            } else {
                unsafe {
                    (*pInfo).idxStr = b"ASC\0".as_ptr() as *mut i8;
                }
            }
            unsafe {
                (*pInfo).orderByConsumed = 1 as i32;
            }
        }
    }
    0 as i32;
    return 0 as i32;
}

/// Implementation of xOpen method.
#[unsafe(link_section = ".text.slate_distinct.fts3.fts3OpenMethod")]
extern "C-unwind" fn fts3OpenMethod(
    mut pVTab: *mut sqlite3_vtab,
    mut ppCsr: *mut *mut sqlite3_vtab_cursor,
) -> i32 {
    let mut pCsr: *mut sqlite3_vtab_cursor = unsafe { std::mem::zeroed() }; // Allocated cursor
    pVTab;
    // Allocate a buffer large enough for an Fts3Cursor structure. If the
    // allocation succeeds, zero it and return SQLITE_OK. Otherwise,
    // if the allocation fails, return SQLITE_NOMEM.
    let __v2093: *mut sqlite3_vtab_cursor =
        (unsafe { sqlite3_malloc(((128 as u64) as u32) as i32) }) as *mut sqlite3_vtab_cursor;
    pCsr = __v2093;
    unsafe {
        *ppCsr = __v2093;
    }
    if !(pCsr != std::ptr::null_mut::<sqlite3_vtab_cursor>()) {
        return 7 as i32;
    }
    unsafe { memset(pCsr as *mut (), 0 as i32, 128 as u64) };
    return 0 as i32;
}

/// Finalize the statement handle at pCsr->pStmt.
///
/// Or, if that statement handle is one created by fts3CursorSeekStmt(),
/// and the Fts3Table.pSeekStmt slot is currently NULL, save the statement
/// pointer there instead of finalizing it.
fn fts3CursorFinalizeStmt(mut pCsr: *mut Fts3Cursor) {
    if (unsafe { (*pCsr).bSeekStmt }) != (0 as u8) {
        let mut p: *mut Fts3Table = (unsafe { (*pCsr).base.pVtab }) as *mut Fts3Table;
        if (unsafe { (*p).pSeekStmt }) == std::ptr::null_mut::<sqlite3_stmt>() {
            unsafe {
                (*p).pSeekStmt = unsafe { (*pCsr).pStmt };
            }
            unsafe { sqlite3_reset(unsafe { (*pCsr).pStmt }) };
            unsafe {
                (*pCsr).pStmt = std::ptr::null_mut::<sqlite3_stmt>();
            }
        }
        unsafe {
            (*pCsr).bSeekStmt = ((0 as i32) as i8) as u8;
        }
    }
    unsafe { sqlite3_finalize(unsafe { (*pCsr).pStmt }) };
}

/// Free all resources currently held by the cursor passed as the only
/// argument.
fn fts3ClearCursor(mut pCsr: *mut Fts3Cursor) {
    fts3CursorFinalizeStmt(pCsr);
    unsafe { sqlite3Fts3FreeDeferredTokens(pCsr) };
    unsafe { sqlite3_free((unsafe { (*pCsr).aDoclist }) as *mut ()) };
    unsafe { sqlite3Fts3MIBufferFree(unsafe { (*pCsr).pMIBuffer }) };
    unsafe { sqlite3Fts3ExprFree(unsafe { (*pCsr).pExpr }) };
    unsafe {
        memset(
            (unsafe { unsafe { std::ptr::addr_of_mut!((*pCsr).base) }.offset((1 as i32) as isize) })
                as *mut (),
            0 as i32,
            (128 as u64).wrapping_sub(8 as u64),
        )
    };
}

/// Close the cursor.  For additional information see the documentation
/// on the xClose method of the virtual table interface.
#[unsafe(link_section = ".text.slate_distinct.fts3.fts3CloseMethod")]
extern "C-unwind" fn fts3CloseMethod(mut pCursor: *mut sqlite3_vtab_cursor) -> i32 {
    let mut pCsr: *mut Fts3Cursor = pCursor as *mut Fts3Cursor;
    0 as i32;
    fts3ClearCursor(pCsr);
    0 as i32;
    unsafe { sqlite3_free(pCsr as *mut ()) };
    return 0 as i32;
}

/// If pCsr->pStmt has not been prepared (i.e. if pCsr->pStmt==0), then
/// compose and prepare an SQL statement of the form:
///
///    "SELECT <columns> FROM %_content WHERE rowid = ?"
///
/// (or the equivalent for a content=xxx table) and set pCsr->pStmt to
/// it. If an error occurs, return an SQLite error code.
fn fts3CursorSeekStmt(mut pCsr: *mut Fts3Cursor) -> i32 {
    let mut rc: i32 = 0 as i32;
    if (unsafe { (*pCsr).pStmt }) == std::ptr::null_mut::<sqlite3_stmt>() {
        let mut p: *mut Fts3Table = (unsafe { (*pCsr).base.pVtab }) as *mut Fts3Table;
        let mut zSql: *mut i8 = unsafe { std::mem::zeroed() };
        if (unsafe { (*p).pSeekStmt }) != std::ptr::null_mut::<sqlite3_stmt>() {
            unsafe {
                (*pCsr).pStmt = unsafe { (*p).pSeekStmt };
            }
            unsafe {
                (*p).pSeekStmt = std::ptr::null_mut::<sqlite3_stmt>();
            }
        } else {
            zSql = unsafe {
                sqlite3_mprintf(
                    (b"SELECT %s WHERE rowid = ?\0".as_ptr() as *mut i8) as *const i8,
                    unsafe { (*p).zReadExprlist },
                )
            };
            if !(zSql != std::ptr::null_mut::<i8>()) {
                return 7 as i32;
            }
            let __v2094: *mut Fts3Table = p;
            let __v2095: i32 = unsafe { (*__v2094).bLock };
            let __v2096: i32 = __v2095 + (1 as i32);
            unsafe {
                (*__v2094).bLock = __v2096;
            }
            rc = unsafe {
                sqlite3Fts3PrepareStmt(p, zSql as *const i8, 1 as i32, 1 as i32, unsafe {
                    std::ptr::addr_of_mut!((*pCsr).pStmt)
                })
            };
            let __v2097: *mut Fts3Table = p;
            let __v2098: i32 = unsafe { (*__v2097).bLock };
            let __v2099: i32 = __v2098 - (1 as i32);
            unsafe {
                (*__v2097).bLock = __v2099;
            }
            unsafe { sqlite3_free(zSql as *mut ()) };
        }
        if rc == (0 as i32) {
            unsafe {
                (*pCsr).bSeekStmt = ((1 as i32) as i8) as u8;
            }
        }
    }
    return rc;
}

/// Position the pCsr->pStmt statement so that it is on the row
/// of the %_content table that contains the last match.  Return
/// SQLITE_OK on success.
fn fts3CursorSeek(mut pContext: *mut sqlite3_context, mut pCsr: *mut Fts3Cursor) -> i32 {
    let mut rc: i32 = 0 as i32;
    if (unsafe { (*pCsr).isRequireSeek }) != (0 as u8) {
        rc = fts3CursorSeekStmt(pCsr);
        if rc == (0 as i32) {
            let mut pTab: *mut Fts3Table = (unsafe { (*pCsr).base.pVtab }) as *mut Fts3Table;
            let __v2100: *mut Fts3Table = pTab;
            let __v2101: i32 = unsafe { (*__v2100).bLock };
            let __v2102: i32 = __v2101 + (1 as i32);
            unsafe {
                (*__v2100).bLock = __v2102;
            }
            unsafe {
                sqlite3_bind_int64(unsafe { (*pCsr).pStmt }, 1 as i32, unsafe {
                    (*pCsr).iPrevId
                })
            };
            unsafe {
                (*pCsr).isRequireSeek = ((0 as i32) as i8) as u8;
            }
            if (100 as i32) == unsafe { sqlite3_step(unsafe { (*pCsr).pStmt }) } {
                let __v2103: *mut Fts3Table = pTab;
                let __v2104: i32 = unsafe { (*__v2103).bLock };
                let __v2105: i32 = __v2104 - (1 as i32);
                unsafe {
                    (*__v2103).bLock = __v2105;
                }
                return 0 as i32;
            } else {
                let __v2106: *mut Fts3Table = pTab;
                let __v2107: i32 = unsafe { (*__v2106).bLock };
                let __v2108: i32 = __v2107 - (1 as i32);
                unsafe {
                    (*__v2106).bLock = __v2108;
                }
                rc = unsafe { sqlite3_reset(unsafe { (*pCsr).pStmt }) };
                if rc == (0 as i32)
                    && (unsafe {
                        (*((unsafe { (*pCsr).base.pVtab }) as *mut Fts3Table)).zContentTbl
                    }) == std::ptr::null_mut::<i8>()
                {
                    // If no row was found and no error has occurred, then the %_content
                    // table is missing a row that is present in the full-text index.
                    // The data structures are corrupt.
                    rc = (11 as i32) | (1 as i32) << (8 as i32);
                    unsafe {
                        (*pCsr).isEof = ((1 as i32) as i8) as u8;
                    }
                }
            }
        }
    }
    if rc != (0 as i32) && pContext != std::ptr::null_mut::<sqlite3_context>() {
        unsafe { sqlite3_result_error_code(pContext, rc) };
    }
    return rc;
}

/// This function is used to process a single interior node when searching
/// a b-tree for a term or term prefix. The node data is passed to this
/// function via the zNode/nNode parameters. The term to search for is
/// passed in zTerm/nTerm.
///
/// If piFirst is not NULL, then this function sets *piFirst to the blockid
/// of the child node that heads the sub-tree that may contain the term.
///
/// If piLast is not NULL, then *piLast is set to the right-most child node
/// that heads a sub-tree that may contain a term for which zTerm/nTerm is
/// a prefix.
///
/// If an OOM error occurs, SQLITE_NOMEM is returned. Otherwise, SQLITE_OK.
///
/// # Arguments
///
/// * `zTerm` - Term to select leaves for
/// * `nTerm` - Size of term zTerm in bytes
/// * `zNode` - Buffer containing segment interior node
/// * `nNode` - Size of buffer at zNode
/// * `piFirst` - OUT: Selected child node
/// * `piLast` - OUT: Selected child node
fn fts3ScanInteriorNode(
    mut zTerm: *const i8,
    mut nTerm: i32,
    mut zNode: *const i8,
    mut nNode: i32,
    mut piFirst: *mut i64,
    mut piLast: *mut i64,
) -> i32 {
    let mut __slate_storage_2122: std::mem::MaybeUninit<u64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2122: *mut u64 = std::ptr::addr_of_mut!(__slate_storage_2122) as *mut u64;
    let mut __slate_storage_2121: std::mem::MaybeUninit<u64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2121: *mut u64 = std::ptr::addr_of_mut!(__slate_storage_2121) as *mut u64;
    let mut __slate_storage_2120: std::mem::MaybeUninit<*const i8> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_2120: *mut *const i8 =
        std::ptr::addr_of_mut!(__slate_storage_2120) as *mut *const i8;
    let mut __slate_storage_2119: std::mem::MaybeUninit<*const i8> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_2119: *mut *const i8 =
        std::ptr::addr_of_mut!(__slate_storage_2119) as *mut *const i8;
    let mut __slate_storage_599: std::mem::MaybeUninit<*mut i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_599: *mut *mut i8 =
        std::ptr::addr_of_mut!(__slate_storage_599) as *mut *mut i8;
    let mut __slate_storage_2118: std::mem::MaybeUninit<*const i8> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_2118: *mut *const i8 =
        std::ptr::addr_of_mut!(__slate_storage_2118) as *mut *const i8;
    let mut __slate_storage_2117: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2117: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_2117) as *mut i32;
    let mut __slate_storage_2116: std::mem::MaybeUninit<*const i8> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_2116: *mut *const i8 =
        std::ptr::addr_of_mut!(__slate_storage_2116) as *mut *const i8;
    let mut __slate_storage_2115: std::mem::MaybeUninit<*const i8> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_2115: *mut *const i8 =
        std::ptr::addr_of_mut!(__slate_storage_2115) as *mut *const i8;
    let mut __slate_storage_2114: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2114: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_2114) as *mut i32;
    let mut __slate_storage_2113: std::mem::MaybeUninit<*const i8> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_2113: *mut *const i8 =
        std::ptr::addr_of_mut!(__slate_storage_2113) as *mut *const i8; // Size of term prefix
    let mut __slate_storage_598: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_598: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_598) as *mut i32; // Size of term suffix
    let mut __slate_storage_597: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_597: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_597) as *mut i32; // memcmp() result
    let mut __slate_storage_596: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_596: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_596) as *mut i32;
    let mut __slate_storage_2112: std::mem::MaybeUninit<*const i8> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_2112: *mut *const i8 =
        std::ptr::addr_of_mut!(__slate_storage_2112) as *mut *const i8;
    let mut __slate_storage_2111: std::mem::MaybeUninit<*const i8> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_2111: *mut *const i8 =
        std::ptr::addr_of_mut!(__slate_storage_2111) as *mut *const i8;
    let mut __slate_storage_2110: std::mem::MaybeUninit<*const i8> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_2110: *mut *const i8 =
        std::ptr::addr_of_mut!(__slate_storage_2110) as *mut *const i8;
    // Skip over the 'height' varint that occurs at the start of every
    // interior node. Then load the blockid of the left-child of the b-tree
    // node into variable iChild.
    //
    // Even if the data structure on disk is corrupted, this (reading two
    // varints from the buffer) does not risk an overread. If zNode is a
    // root node, then the buffer comes from a SELECT statement. SQLite does
    // not make this guarantee explicitly, but in practice there are always
    // either more than 20 bytes of allocated space following the nNode bytes of
    // contents, or two zero bytes. Or, if the node is read from the %_segments
    // table, then there are always 20 bytes of zeroed padding following the
    // nNode bytes of content (see sqlite3Fts3ReadBlock() for details).
    let mut __slate_storage_2109: std::mem::MaybeUninit<*const i8> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_2109: *mut *const i8 =
        std::ptr::addr_of_mut!(__slate_storage_2109) as *mut *const i8; // Total term size
    let mut __slate_storage_595: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_595: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_595) as *mut i32; // Block id of child node to descend to
    let mut __slate_storage_594: std::mem::MaybeUninit<u64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_594: *mut u64 = std::ptr::addr_of_mut!(__slate_storage_594) as *mut u64; // True when processing first term on page
    let mut __slate_storage_593: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_593: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_593) as *mut i32; // Size of allocated buffer
    let mut __slate_storage_592: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_592: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_592) as *mut i64; // Buffer to load terms into
    let mut __slate_storage_591: std::mem::MaybeUninit<*mut i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_591: *mut *mut i8 =
        std::ptr::addr_of_mut!(__slate_storage_591) as *mut *mut i8; // End of interior node buffer
    let mut __slate_storage_590: std::mem::MaybeUninit<*const i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_590: *mut *const i8 =
        std::ptr::addr_of_mut!(__slate_storage_590) as *mut *const i8; // Cursor to iterate through node
    let mut __slate_storage_589: std::mem::MaybeUninit<*const i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_589: *mut *const i8 =
        std::ptr::addr_of_mut!(__slate_storage_589) as *mut *const i8; // Return code
    let mut __slate_storage_588: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_588: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_588) as *mut i32;
    unsafe {
        std::ptr::write(__slate_slot_588, 0 as i32);
        std::ptr::write(__slate_slot_589, zNode);
        std::ptr::write(__slate_slot_590, unsafe {
            (*__slate_slot_589).offset(nNode as isize)
        });
        std::ptr::write(__slate_slot_591, std::ptr::null_mut::<i8>());
        std::ptr::write(__slate_slot_592, (0 as i32) as i64);
        std::ptr::write(__slate_slot_593, 1 as i32);
        std::ptr::write(__slate_slot_595, 0 as i32);
        std::ptr::write(__slate_slot_2109, *__slate_slot_589);
        std::ptr::write(__slate_slot_2110, unsafe {
            (*__slate_slot_2109).offset(sqlite3Fts3GetVarintU(
                *__slate_slot_589,
                std::ptr::addr_of_mut!(*__slate_slot_594),
            ) as isize)
        });
        *__slate_slot_589 = *__slate_slot_2110;
        std::ptr::write(__slate_slot_2111, *__slate_slot_589);
        std::ptr::write(__slate_slot_2112, unsafe {
            (*__slate_slot_2111).offset(sqlite3Fts3GetVarintU(
                *__slate_slot_589,
                std::ptr::addr_of_mut!(*__slate_slot_594),
            ) as isize)
        });
        *__slate_slot_589 = *__slate_slot_2112;
        if *__slate_slot_589 > *__slate_slot_590 {
            return (11 as i32) | (1 as i32) << (8 as i32);
        } else {
            '__join_0: {
                '__join_20: {
                    '__join_15: {
                        '__join_12: {
                            loop {
                                if *__slate_slot_589 < *__slate_slot_590
                                    && (piFirst != std::ptr::null_mut::<i64>()
                                        || piLast != std::ptr::null_mut::<i64>())
                                {
                                    std::ptr::write(__slate_slot_598, 0 as i32);
                                    // Load the next term on the node into zBuffer. Use realloc() to expand
                                    // the size of zBuffer if required.
                                    if !(*__slate_slot_593 != (0 as i32)) {
                                        std::ptr::write(__slate_slot_2113, *__slate_slot_589);
                                        if (((unsafe { *(*__slate_slot_589 as *mut u8) }) as u32)
                                            as i32)
                                            & (128 as i32)
                                            != (0 as i32)
                                        {
                                            *__slate_slot_2114 = sqlite3Fts3GetVarint32(
                                                *__slate_slot_589,
                                                std::ptr::addr_of_mut!(*__slate_slot_598),
                                            );
                                        } else {
                                            unsafe {
                                                *std::ptr::addr_of_mut!(*__slate_slot_598) =
                                                    ((unsafe { *(*__slate_slot_589 as *mut u8) })
                                                        as u32)
                                                        as i32;
                                            }
                                            *__slate_slot_2114 = 1 as i32;
                                        }
                                        std::ptr::write(__slate_slot_2115, unsafe {
                                            (*__slate_slot_2113).offset(*__slate_slot_2114 as isize)
                                        });
                                        *__slate_slot_589 = *__slate_slot_2115;
                                        if *__slate_slot_598 > *__slate_slot_595 {
                                            break '__join_20;
                                        }
                                    }
                                    *__slate_slot_593 = 0 as i32;
                                    std::ptr::write(__slate_slot_2116, *__slate_slot_589);
                                    if (((unsafe { *(*__slate_slot_589 as *mut u8) }) as u32)
                                        as i32)
                                        & (128 as i32)
                                        != (0 as i32)
                                    {
                                        *__slate_slot_2117 = sqlite3Fts3GetVarint32(
                                            *__slate_slot_589,
                                            std::ptr::addr_of_mut!(*__slate_slot_597),
                                        );
                                    } else {
                                        unsafe {
                                            *std::ptr::addr_of_mut!(*__slate_slot_597) =
                                                ((unsafe { *(*__slate_slot_589 as *mut u8) })
                                                    as u32)
                                                    as i32;
                                        }
                                        *__slate_slot_2117 = 1 as i32;
                                    }
                                    std::ptr::write(__slate_slot_2118, unsafe {
                                        (*__slate_slot_2116).offset(*__slate_slot_2117 as isize)
                                    });
                                    *__slate_slot_589 = *__slate_slot_2118;
                                    0 as i32;
                                    if (*__slate_slot_598 as i64)
                                        > ((unsafe {
                                            (*__slate_slot_589).offset_from(zNode as *const i8)
                                        }) as i64)
                                        || (*__slate_slot_597 as i64)
                                            > ((unsafe {
                                                (*__slate_slot_590)
                                                    .offset_from(*__slate_slot_589 as *const i8)
                                            })
                                                as i64)
                                        || *__slate_slot_597 == (0 as i32)
                                    {
                                        break '__join_15;
                                    } else {
                                        if (*__slate_slot_598 as i64) + (*__slate_slot_597 as i64)
                                            > *__slate_slot_592
                                        {
                                            *__slate_slot_592 = ((*__slate_slot_598 as i64)
                                                + (*__slate_slot_597 as i64))
                                                * ((2 as i32) as i64);
                                            *__slate_slot_599 = (unsafe {
                                                sqlite3_realloc64(
                                                    *__slate_slot_591 as *mut (),
                                                    *__slate_slot_592 as u64,
                                                )
                                            })
                                                as *mut i8;
                                            if !(*__slate_slot_599 != std::ptr::null_mut::<i8>()) {
                                                break '__join_12;
                                            } else {
                                                *__slate_slot_591 = *__slate_slot_599;
                                            }
                                        }
                                        0 as i32;
                                        unsafe {
                                            memcpy(
                                                (unsafe {
                                                    (*__slate_slot_591)
                                                        .offset(*__slate_slot_598 as isize)
                                                })
                                                    as *mut (),
                                                *__slate_slot_589 as *const (),
                                                (*__slate_slot_597 as i64) as u64,
                                            )
                                        };
                                        *__slate_slot_595 = *__slate_slot_598 + *__slate_slot_597;
                                        std::ptr::write(__slate_slot_2119, *__slate_slot_589);
                                        std::ptr::write(__slate_slot_2120, unsafe {
                                            (*__slate_slot_2119).offset(*__slate_slot_597 as isize)
                                        });
                                        *__slate_slot_589 = *__slate_slot_2120;
                                        // Compare the term we are searching for with the term just loaded from
                                        // the interior node. If the specified term is greater than or equal
                                        // to the term from the interior node, then all terms on the sub-tree
                                        // headed by node iChild are smaller than zTerm. No need to search
                                        // iChild.
                                        //
                                        // If the interior node term is larger than the specified term, then
                                        // the tree headed by iChild may contain the specified term.
                                        *__slate_slot_596 = unsafe {
                                            memcmp(
                                                zTerm as *const (),
                                                *__slate_slot_591 as *const (),
                                                ((if *__slate_slot_595 > nTerm {
                                                    nTerm
                                                } else {
                                                    *__slate_slot_595
                                                })
                                                    as i64)
                                                    as u64,
                                            )
                                        };
                                        if piFirst != std::ptr::null_mut::<i64>()
                                            && (*__slate_slot_596 < (0 as i32)
                                                || *__slate_slot_596 == (0 as i32)
                                                    && *__slate_slot_595 > nTerm)
                                        {
                                            unsafe {
                                                *piFirst = *__slate_slot_594 as i64;
                                            }
                                            piFirst = std::ptr::null_mut::<i64>();
                                        }
                                        if piLast != std::ptr::null_mut::<i64>()
                                            && *__slate_slot_596 < (0 as i32)
                                        {
                                            unsafe {
                                                *piLast = *__slate_slot_594 as i64;
                                            }
                                            piLast = std::ptr::null_mut::<i64>();
                                        }
                                        std::ptr::write(__slate_slot_2121, *__slate_slot_594);
                                        std::ptr::write(
                                            __slate_slot_2122,
                                            (*__slate_slot_2121)
                                                .wrapping_add(((1 as i32) as i64) as u64),
                                        );
                                        *__slate_slot_594 = *__slate_slot_2122;
                                    }
                                } else {
                                    break;
                                }
                            }
                            {}
                            if piFirst != std::ptr::null_mut::<i64>() {
                                unsafe {
                                    *piFirst = *__slate_slot_594 as i64;
                                }
                            }
                            if piLast != std::ptr::null_mut::<i64>() {
                                unsafe {
                                    *piLast = *__slate_slot_594 as i64;
                                }
                                break '__join_0;
                            } else {
                                break '__join_0;
                            }
                        }
                        *__slate_slot_588 = 7 as i32;
                        break '__join_0;
                    }
                    *__slate_slot_588 = (11 as i32) | (1 as i32) << (8 as i32);
                    break '__join_0;
                }
                *__slate_slot_588 = (11 as i32) | (1 as i32) << (8 as i32);
            }
            unsafe { sqlite3_free(*__slate_slot_591 as *mut ()) };
            return *__slate_slot_588;
        }
    }
    return unsafe { std::mem::zeroed() };
}

/// The buffer pointed to by argument zNode (size nNode bytes) contains an
/// interior node of a b-tree segment. The zTerm buffer (size nTerm bytes)
/// contains a term. This function searches the sub-tree headed by the zNode
/// node for the range of leaf nodes that may contain the specified term
/// or terms for which the specified term is a prefix.
///
/// If piLeaf is not NULL, then *piLeaf is set to the blockid of the
/// left-most leaf node in the tree that may contain the specified term.
/// If piLeaf2 is not NULL, then *piLeaf2 is set to the blockid of the
/// right-most leaf node that may contain a term for which the specified
/// term is a prefix.
///
/// It is possible that the range of returned leaf nodes does not contain
/// the specified term or any terms for which it is a prefix. However, if the
/// segment does contain any such terms, they are stored within the identified
/// range. Because this function only inspects interior segment nodes (and
/// never loads leaf nodes into memory), it is not possible to be sure.
///
/// If an error occurs, an error code other than SQLITE_OK is returned.
///
/// # Arguments
///
/// * `p` - Virtual table handle
/// * `zTerm` - Term to select leaves for
/// * `nTerm` - Size of term zTerm in bytes
/// * `zNode` - Buffer containing segment interior node
/// * `nNode` - Size of buffer at zNode
/// * `piLeaf` - Selected leaf node
/// * `piLeaf2` - Selected leaf node
fn fts3SelectLeaf(
    mut p: *mut Fts3Table,
    mut zTerm: *const i8,
    mut nTerm: i32,
    mut zNode: *const i8,
    mut nNode: i32,
    mut piLeaf: *mut i64,
    mut piLeaf2: *mut i64,
) -> i32 {
    let mut rc: i32 = 0 as i32; // Return code
    let mut iHeight: i32 = 0 as i32; // Height of this node in tree
    0 as i32;
    let __v2123: i32;
    if (((unsafe { *(zNode as *mut u8) }) as u32) as i32) & (128 as i32) != (0 as i32) {
        __v2123 = sqlite3Fts3GetVarint32(zNode, std::ptr::addr_of_mut!(iHeight));
    } else {
        unsafe {
            *std::ptr::addr_of_mut!(iHeight) = ((unsafe { *(zNode as *mut u8) }) as u32) as i32;
        }
        __v2123 = 1 as i32;
    }
    if iHeight > (48 as i32) {
        rc = (11 as i32) | (1 as i32) << (8 as i32);
    } else {
        rc = fts3ScanInteriorNode(zTerm, nTerm, zNode, nNode, piLeaf, piLeaf2);
    }
    0 as i32;
    if rc == (0 as i32) && iHeight > (1 as i32) {
        let mut zBlob: *mut i8 = std::ptr::null_mut::<i8>(); // Blob read from %_segments table
        let mut nBlob: i32 = 0 as i32; // Size of zBlob in bytes
        if piLeaf != std::ptr::null_mut::<i64>()
            && piLeaf2 != std::ptr::null_mut::<i64>()
            && (unsafe { *piLeaf }) != unsafe { *piLeaf2 }
        {
            rc = unsafe {
                sqlite3Fts3ReadBlock(
                    p,
                    unsafe { *piLeaf },
                    std::ptr::addr_of_mut!(zBlob),
                    std::ptr::addr_of_mut!(nBlob),
                    std::ptr::null_mut::<i32>(),
                )
            };
            if rc == (0 as i32) {
                rc = fts3SelectLeaf(
                    p,
                    zTerm,
                    nTerm,
                    zBlob as *const i8,
                    nBlob,
                    piLeaf,
                    std::ptr::null_mut::<i64>(),
                );
            }
            unsafe { sqlite3_free(zBlob as *mut ()) };
            piLeaf = std::ptr::null_mut::<i64>();
            zBlob = std::ptr::null_mut::<i8>();
        }
        if rc == (0 as i32) {
            rc = unsafe {
                sqlite3Fts3ReadBlock(
                    p,
                    if piLeaf != std::ptr::null_mut::<i64>() {
                        unsafe { *piLeaf }
                    } else {
                        unsafe { *piLeaf2 }
                    },
                    std::ptr::addr_of_mut!(zBlob),
                    std::ptr::addr_of_mut!(nBlob),
                    std::ptr::null_mut::<i32>(),
                )
            };
        }
        if rc == (0 as i32) {
            let mut iNewHeight: i32 = 0 as i32;
            let __v2124: i32;
            if (((unsafe { *(zBlob as *mut u8) }) as u32) as i32) & (128 as i32) != (0 as i32) {
                __v2124 =
                    sqlite3Fts3GetVarint32(zBlob as *const i8, std::ptr::addr_of_mut!(iNewHeight));
            } else {
                unsafe {
                    *std::ptr::addr_of_mut!(iNewHeight) =
                        ((unsafe { *(zBlob as *mut u8) }) as u32) as i32;
                }
                __v2124 = 1 as i32;
            }
            if iNewHeight >= iHeight {
                rc = (11 as i32) | (1 as i32) << (8 as i32);
            } else {
                rc = fts3SelectLeaf(p, zTerm, nTerm, zBlob as *const i8, nBlob, piLeaf, piLeaf2);
            }
        }
        unsafe { sqlite3_free(zBlob as *mut ()) };
    }
    return rc;
}

/// This function is used to create delta-encoded serialized lists of FTS3
/// varints. Each call to this function appends a single varint to a list.
///
/// # Arguments
///
/// * `pp` - IN/OUT: Output pointer
/// * `piPrev` - IN/OUT: Previous value written to list
/// * `iVal` - Write this value to the list
fn fts3PutDeltaVarint(mut pp: *mut *mut i8, mut piPrev: *mut i64, mut iVal: i64) {
    0 as i32;
    if iVal - unsafe { *piPrev } >= ((0 as i32) as i64) {
        // Refuse to write a negative delta integer. This only happens with a
        // corrupt db (see the assert above) and can cause buffer overwrites
        // in some cases.
        let __v2125: *mut *mut i8 = pp;
        let __v2126: *mut i8 = unsafe { *__v2125 };
        let __v2127: *mut i8 = unsafe {
            __v2126.offset(sqlite3Fts3PutVarint(unsafe { *pp }, iVal - unsafe { *piPrev }) as isize)
        };
        unsafe {
            *__v2125 = __v2127;
        }
        unsafe {
            *piPrev = iVal;
        }
    }
}

/// When this function is called, *ppPoslist is assumed to point to the
/// start of a position-list. After it returns, *ppPoslist points to the
/// first byte after the position-list.
///
/// A position list is list of positions (delta encoded) and columns for
/// a single document record of a doclist.  So, in other words, this
/// routine advances *ppPoslist so that it points to the next docid in
/// the doclist, or to the first byte past the end of the doclist.
///
/// If pp is not NULL, then the contents of the position list are copied
/// to *pp. *pp is set to point to the first byte past the last byte copied
/// before this function returns.
fn fts3PoslistCopy(mut pp: *mut *mut i8, mut ppPoslist: *mut *mut i8) {
    let mut pEnd: *mut i8 = unsafe { *ppPoslist };
    let mut c: i8 = (0 as i32) as i8;
    // The end of a position list is marked by a zero encoded as an FTS3
    // varint. A single POS_END (0) byte. Except, if the 0 byte is preceded by
    // a byte with the 0x80 bit set, then it is not a varint 0, but the tail
    // of some other, multi-byte, value.
    //
    // The following while-loop moves pEnd to point to the first byte that is not
    // immediately preceded by a byte with the 0x80 bit set. Then increments
    // pEnd once more so that it points to the byte immediately following the
    // last byte in the position-list.
    '__slate_break_1713: while ((unsafe { *pEnd }) as i32) | (c as i32) != (0 as i32) {
        let __v2128: *mut i8 = pEnd;
        let __v2129: *mut i8 = unsafe { __v2128.offset((1 as i32) as isize) };
        pEnd = __v2129;
        c = (((unsafe { *__v2128 }) as i32) & (128 as i32)) as i8;
        {}
    }
    let __v2130: *mut i8 = pEnd;
    let __v2131: *mut i8 = unsafe { __v2130.offset((1 as i32) as isize) };
    pEnd = __v2131; // Advance past the POS_END terminator byte
    if pp != std::ptr::null_mut::<*mut i8>() {
        let mut n: i32 =
            ((unsafe { pEnd.offset_from((unsafe { *ppPoslist }) as *mut i8) }) as i64) as i32;
        let mut p: *mut i8 = unsafe { *pp };
        unsafe {
            memcpy(
                p as *mut (),
                (unsafe { *ppPoslist }) as *const (),
                (n as i64) as u64,
            )
        };
        let __v2132: *mut i8 = p;
        let __v2133: *mut i8 = unsafe { __v2132.offset(n as isize) };
        p = __v2133;
        unsafe {
            *pp = p;
        }
    }
    unsafe {
        *ppPoslist = pEnd;
    }
}

/// When this function is called, *ppPoslist is assumed to point to the
/// start of a column-list. After it returns, *ppPoslist points to the
/// to the terminator (POS_COLUMN or POS_END) byte of the column-list.
///
/// A column-list is list of delta-encoded positions for a single column
/// within a single document within a doclist.
///
/// The column-list is terminated either by a POS_COLUMN varint (1) or
/// a POS_END varint (0).  This routine leaves *ppPoslist pointing to
/// the POS_COLUMN or POS_END that terminates the column-list.
///
/// If pp is not NULL, then the contents of the column-list are copied
/// to *pp. *pp is set to point to the first byte past the last byte copied
/// before this function returns.  The POS_COLUMN or POS_END terminator
/// is not copied into *pp.
fn fts3ColumnlistCopy(mut pp: *mut *mut i8, mut ppPoslist: *mut *mut i8) {
    let mut pEnd: *mut i8 = unsafe { *ppPoslist };
    let mut c: i8 = (0 as i32) as i8;
    // A column-list is terminated by either a 0x01 or 0x00 byte that is
    // not part of a multi-byte varint.
    '__slate_break_1714: while (254 as i32) & (((unsafe { *pEnd }) as i32) | (c as i32))
        != (0 as i32)
    {
        let __v2134: *mut i8 = pEnd;
        let __v2135: *mut i8 = unsafe { __v2134.offset((1 as i32) as isize) };
        pEnd = __v2135;
        c = (((unsafe { *__v2134 }) as i32) & (128 as i32)) as i8;
        {}
    }
    if pp != std::ptr::null_mut::<*mut i8>() {
        let mut n: i32 =
            ((unsafe { pEnd.offset_from((unsafe { *ppPoslist }) as *mut i8) }) as i64) as i32;
        let mut p: *mut i8 = unsafe { *pp };
        unsafe {
            memcpy(
                p as *mut (),
                (unsafe { *ppPoslist }) as *const (),
                (n as i64) as u64,
            )
        };
        let __v2136: *mut i8 = p;
        let __v2137: *mut i8 = unsafe { __v2136.offset(n as isize) };
        p = __v2137;
        unsafe {
            *pp = p;
        }
    }
    unsafe {
        *ppPoslist = pEnd;
    }
}

// Value used to signify the end of an position-list. This must be
// as large or larger than any value that might appear on the
// position-list, even a position list that has been corrupted.
/// This function is used to help parse position-lists. When this function is
/// called, *pp may point to the start of the next varint in the position-list
/// being parsed, or it may point to 1 byte past the end of the position-list
/// (in which case **pp will be a terminator bytes POS_END (0) or
/// (1)).
///
/// If *pp points past the end of the current position-list, set *pi to
/// POSITION_LIST_END and return. Otherwise, read the next varint from *pp,
/// increment the current value of *pi by the value read, and set *pp to
/// point to the next value before returning.
///
/// Before calling this routine *pi must be initialized to the value of
/// the previous position, or zero if we are reading the first position
/// in the position-list.  Because positions are delta-encoded, the value
/// of the previous position is needed in order to compute the value of
/// the next position.
///
/// # Arguments
///
/// * `pp` - IN/OUT: Pointer into position-list buffer
/// * `pi` - IN/OUT: Value read from position-list
fn fts3ReadNextPos(mut pp: *mut *mut i8, mut pi: *mut i64) {
    if ((unsafe { *unsafe { *pp } }) as i32) & (254 as i32) != (0 as i32) {
        let mut iVal: i32 = 0 as i32;
        let __v2138: *mut *mut i8 = pp;
        let __v2139: *mut i8 = unsafe { *__v2138 };
        let __v2140: i32;
        if (((unsafe { *((unsafe { *pp }) as *mut u8) }) as u32) as i32) & (128 as i32)
            != (0 as i32)
        {
            __v2140 =
                sqlite3Fts3GetVarint32((unsafe { *pp }) as *const i8, std::ptr::addr_of_mut!(iVal));
        } else {
            unsafe {
                *std::ptr::addr_of_mut!(iVal) =
                    ((unsafe { *((unsafe { *pp }) as *mut u8) }) as u32) as i32;
            }
            __v2140 = 1 as i32;
        }
        let __v2141: *mut i8 = unsafe { __v2139.offset(__v2140 as isize) };
        unsafe {
            *__v2138 = __v2141;
        }
        let __v2142: *mut i64 = pi;
        let __v2143: i64 = unsafe { *__v2142 };
        let __v2144: i64 = __v2143 + (iVal as i64);
        unsafe {
            *__v2142 = __v2144;
        }
        let __v2145: *mut i64 = pi;
        let __v2146: i64 = unsafe { *__v2145 };
        let __v2147: i64 = __v2146 - ((2 as i32) as i64);
        unsafe {
            *__v2145 = __v2147;
        }
    } else {
        unsafe {
            *pi =
                (((4294967295 as u32) as u64) as i64) | ((2147483647 as i32) as i64) << (32 as i32);
        }
    }
}

/// If parameter iCol is not 0, write an POS_COLUMN (1) byte followed by
/// the value of iCol encoded as a varint to *pp.   This will start a new
/// column list.
///
/// Set *pp to point to the byte just after the last byte written before
/// returning (do not modify it if iCol==0). Return the total number of bytes
/// written (0 if iCol==0).
fn fts3PutColNumber(mut pp: *mut *mut i8, mut iCol: i32) -> i32 {
    let mut n: i32 = 0 as i32; // Number of bytes written
    if iCol != (0 as i32) {
        let mut p: *mut i8 = unsafe { *pp }; // Output pointer
        n = (1 as i32)
            + sqlite3Fts3PutVarint(unsafe { p.offset((1 as i32) as isize) }, iCol as i64);
        unsafe {
            *p = (1 as i32) as i8;
        }
        unsafe {
            *pp = unsafe { p.offset(n as isize) };
        }
    }
    return n;
}

/// Compute the union of two position lists.  The output written
/// into *pp contains all positions of both *pp1 and *pp2 in sorted
/// order and with any duplicates removed.  All pointers are
/// updated appropriately.   The caller is responsible for insuring
/// that there is enough space in *pp to hold the complete output.
///
/// # Arguments
///
/// * `pp` - Output buffer
/// * `pp1` - Left input list
/// * `pp2` - Right input list
fn fts3PoslistMerge(mut pp: *mut *mut i8, mut pp1: *mut *mut i8, mut pp2: *mut *mut i8) -> i32 {
    let mut p: *mut i8 = unsafe { *pp };
    let mut p1: *mut i8 = unsafe { *pp1 };
    let mut p2: *mut i8 = unsafe { *pp2 };
    '__slate_break_1715: while (unsafe { *p1 }) != (0 as i8) || (unsafe { *p2 }) != (0 as i8) {
        let mut iCol1: i32 = 0 as i32; // The current column index in pp1
        let mut iCol2: i32 = 0 as i32; // The current column index in pp2
        if ((unsafe { *p1 }) as i32) == (1 as i32) {
            let __v2148: i32;
            if (((unsafe { *((unsafe { p1.offset((1 as i32) as isize) }) as *mut u8) }) as u32)
                as i32)
                & (128 as i32)
                != (0 as i32)
            {
                __v2148 = sqlite3Fts3GetVarint32(
                    (unsafe { p1.offset((1 as i32) as isize) }) as *const i8,
                    std::ptr::addr_of_mut!(iCol1),
                );
            } else {
                unsafe {
                    *std::ptr::addr_of_mut!(iCol1) =
                        ((unsafe { *((unsafe { p1.offset((1 as i32) as isize) }) as *mut u8) })
                            as u32) as i32;
                }
                __v2148 = 1 as i32;
            }
            if iCol1 == (0 as i32) {
                return (11 as i32) | (1 as i32) << (8 as i32);
            }
        } else {
            if ((unsafe { *p1 }) as i32) == (0 as i32) {
                iCol1 = 2147483647 as i32;
            } else {
                iCol1 = 0 as i32;
            }
        }
        if ((unsafe { *p2 }) as i32) == (1 as i32) {
            let __v2149: i32;
            if (((unsafe { *((unsafe { p2.offset((1 as i32) as isize) }) as *mut u8) }) as u32)
                as i32)
                & (128 as i32)
                != (0 as i32)
            {
                __v2149 = sqlite3Fts3GetVarint32(
                    (unsafe { p2.offset((1 as i32) as isize) }) as *const i8,
                    std::ptr::addr_of_mut!(iCol2),
                );
            } else {
                unsafe {
                    *std::ptr::addr_of_mut!(iCol2) =
                        ((unsafe { *((unsafe { p2.offset((1 as i32) as isize) }) as *mut u8) })
                            as u32) as i32;
                }
                __v2149 = 1 as i32;
            }
            if iCol2 == (0 as i32) {
                return (11 as i32) | (1 as i32) << (8 as i32);
            }
        } else {
            if ((unsafe { *p2 }) as i32) == (0 as i32) {
                iCol2 = 2147483647 as i32;
            } else {
                iCol2 = 0 as i32;
            }
        }
        if iCol1 == iCol2 {
            let mut i1: i64 = (0 as i32) as i64; // Last position from pp1
            let mut i2: i64 = (0 as i32) as i64; // Last position from pp2
            let mut iPrev: i64 = (0 as i32) as i64;
            let mut n: i32 = fts3PutColNumber(std::ptr::addr_of_mut!(p), iCol1);
            let __v2150: *mut i8 = p1;
            let __v2151: *mut i8 = unsafe { __v2150.offset(n as isize) };
            p1 = __v2151;
            let __v2152: *mut i8 = p2;
            let __v2153: *mut i8 = unsafe { __v2152.offset(n as isize) };
            p2 = __v2153;
            // At this point, both p1 and p2 point to the start of column-lists
            // for the same column (the column with index iCol1 and iCol2).
            // A column-list is a list of non-negative delta-encoded varints, each
            // incremented by 2 before being stored. Each list is terminated by a
            // POS_END (0) or POS_COLUMN (1). The following block merges the two lists
            // and writes the results to buffer p. p is left pointing to the byte
            // after the list written. No terminator (POS_END or POS_COLUMN) is
            // written to the output.
            fts3GetDeltaVarint(std::ptr::addr_of_mut!(p1), std::ptr::addr_of_mut!(i1));
            fts3GetDeltaVarint(std::ptr::addr_of_mut!(p2), std::ptr::addr_of_mut!(i2));
            if i1 < ((2 as i32) as i64) || i2 < ((2 as i32) as i64) {
                break '__slate_break_1715;
            }
            '__slate_break_1716: loop {
                fts3PutDeltaVarint(
                    std::ptr::addr_of_mut!(p),
                    std::ptr::addr_of_mut!(iPrev),
                    if i1 < i2 { i1 } else { i2 },
                );
                let __v2154: i64 = iPrev;
                let __v2155: i64 = __v2154 - ((2 as i32) as i64);
                iPrev = __v2155;
                if i1 == i2 {
                    fts3ReadNextPos(std::ptr::addr_of_mut!(p1), std::ptr::addr_of_mut!(i1));
                    fts3ReadNextPos(std::ptr::addr_of_mut!(p2), std::ptr::addr_of_mut!(i2));
                } else {
                    if i1 < i2 {
                        fts3ReadNextPos(std::ptr::addr_of_mut!(p1), std::ptr::addr_of_mut!(i1));
                    } else {
                        fts3ReadNextPos(std::ptr::addr_of_mut!(p2), std::ptr::addr_of_mut!(i2));
                    }
                }
                if !(i1
                    != (((4294967295 as u32) as u64) as i64)
                        | ((2147483647 as i32) as i64) << (32 as i32)
                    || i2
                        != (((4294967295 as u32) as u64) as i64)
                            | ((2147483647 as i32) as i64) << (32 as i32))
                {
                    break;
                }
            }
        } else {
            if iCol1 < iCol2 {
                let __v2156: *mut i8 = p1;
                let __v2157: *mut i8 = unsafe {
                    __v2156.offset(fts3PutColNumber(std::ptr::addr_of_mut!(p), iCol1) as isize)
                };
                p1 = __v2157;
                fts3ColumnlistCopy(std::ptr::addr_of_mut!(p), std::ptr::addr_of_mut!(p1));
            } else {
                let __v2158: *mut i8 = p2;
                let __v2159: *mut i8 = unsafe {
                    __v2158.offset(fts3PutColNumber(std::ptr::addr_of_mut!(p), iCol2) as isize)
                };
                p2 = __v2159;
                fts3ColumnlistCopy(std::ptr::addr_of_mut!(p), std::ptr::addr_of_mut!(p2));
            }
        }
    }
    let __v2160: *mut i8 = p;
    let __v2161: *mut i8 = unsafe { __v2160.offset((1 as i32) as isize) };
    p = __v2161;
    unsafe {
        *__v2160 = (0 as i32) as i8;
    }
    unsafe {
        *pp = p;
    }
    unsafe {
        *pp1 = unsafe { p1.offset((1 as i32) as isize) };
    }
    unsafe {
        *pp2 = unsafe { p2.offset((1 as i32) as isize) };
    }
    return 0 as i32;
}

/// This function is used to merge two position lists into one. When it is
/// called, *pp1 and *pp2 must both point to position lists. A position-list is
/// the part of a doclist that follows each document id. For example, if a row
/// contains:
///
///     'a b c'|'x y z'|'a b b a'
///
/// Then the position list for this row for token 'b' would consist of:
///
///     0x02 0x01 0x02 0x03 0x03 0x00
///
/// When this function returns, both *pp1 and *pp2 are left pointing to the
/// byte following the 0x00 terminator of their respective position lists.
///
/// If isSaveLeft is 0, an entry is added to the output position list for
/// each position in *pp2 for which there exists one or more positions in
/// *pp1 so that (pos(*pp2)>pos(*pp1) && pos(*pp2)-pos(*pp1)<=nToken). i.e.
/// when the *pp1 token appears before the *pp2 token, but not more than nToken
/// slots before it.
///
/// e.g. nToken==1 searches for adjacent positions.
///
/// # Arguments
///
/// * `pp` - IN/OUT: Preallocated output buffer
/// * `nToken` - Maximum difference in token positions
/// * `isSaveLeft` - Save the left position
/// * `isExact` - If *pp1 is exactly nTokens before *pp2
/// * `pp1` - IN/OUT: Left input list
/// * `pp2` - IN/OUT: Right input list
fn fts3PoslistPhraseMerge(
    mut pp: *mut *mut i8,
    mut nToken: i32,
    mut isSaveLeft: i32,
    mut isExact: i32,
    mut pp1: *mut *mut i8,
    mut pp2: *mut *mut i8,
) -> i32 {
    let mut p: *mut i8 = unsafe { *pp };
    let mut p1: *mut i8 = unsafe { *pp1 };
    let mut p2: *mut i8 = unsafe { *pp2 };
    let mut iCol1: i32 = 0 as i32;
    let mut iCol2: i32 = 0 as i32;
    // Never set both isSaveLeft and isExact for the same invocation.
    0 as i32;
    0 as i32;
    if ((unsafe { *p1 }) as i32) == (1 as i32) {
        let __v2162: *mut i8 = p1;
        let __v2163: *mut i8 = unsafe { __v2162.offset((1 as i32) as isize) };
        p1 = __v2163;
        let __v2164: *mut i8 = p1;
        let __v2165: i32;
        if (((unsafe { *(p1 as *mut u8) }) as u32) as i32) & (128 as i32) != (0 as i32) {
            __v2165 = sqlite3Fts3GetVarint32(p1 as *const i8, std::ptr::addr_of_mut!(iCol1));
        } else {
            unsafe {
                *std::ptr::addr_of_mut!(iCol1) = ((unsafe { *(p1 as *mut u8) }) as u32) as i32;
            }
            __v2165 = 1 as i32;
        }
        let __v2166: *mut i8 = unsafe { __v2164.offset(__v2165 as isize) };
        p1 = __v2166;
        // iCol1==0 indicates corruption. Column 0 does not have a POS_COLUMN
        // entry, so this is actually end-of-doclist.
        if iCol1 == (0 as i32) {
            return 0 as i32;
        }
    }
    if ((unsafe { *p2 }) as i32) == (1 as i32) {
        let __v2167: *mut i8 = p2;
        let __v2168: *mut i8 = unsafe { __v2167.offset((1 as i32) as isize) };
        p2 = __v2168;
        let __v2169: *mut i8 = p2;
        let __v2170: i32;
        if (((unsafe { *(p2 as *mut u8) }) as u32) as i32) & (128 as i32) != (0 as i32) {
            __v2170 = sqlite3Fts3GetVarint32(p2 as *const i8, std::ptr::addr_of_mut!(iCol2));
        } else {
            unsafe {
                *std::ptr::addr_of_mut!(iCol2) = ((unsafe { *(p2 as *mut u8) }) as u32) as i32;
            }
            __v2170 = 1 as i32;
        }
        let __v2171: *mut i8 = unsafe { __v2169.offset(__v2170 as isize) };
        p2 = __v2171;
        // As above, iCol2==0 indicates corruption.
        if iCol2 == (0 as i32) {
            return 0 as i32;
        }
    }
    '__slate_break_1717: while (1 as i32) != (0 as i32) {
        if iCol1 == iCol2 {
            let mut pSave: *mut i8 = p;
            let mut iPrev: i64 = (0 as i32) as i64;
            let mut iPos1: i64 = (0 as i32) as i64;
            let mut iPos2: i64 = (0 as i32) as i64;
            if iCol1 != (0 as i32) {
                let __v2172: *mut i8 = p;
                let __v2173: *mut i8 = unsafe { __v2172.offset((1 as i32) as isize) };
                p = __v2173;
                unsafe {
                    *__v2172 = (1 as i32) as i8;
                }
                let __v2174: *mut i8 = p;
                let __v2175: *mut i8 =
                    unsafe { __v2174.offset(sqlite3Fts3PutVarint(p, iCol1 as i64) as isize) };
                p = __v2175;
            }
            fts3GetDeltaVarint(std::ptr::addr_of_mut!(p1), std::ptr::addr_of_mut!(iPos1));
            let __v2176: i64 = iPos1;
            let __v2177: i64 = __v2176 - ((2 as i32) as i64);
            iPos1 = __v2177;
            fts3GetDeltaVarint(std::ptr::addr_of_mut!(p2), std::ptr::addr_of_mut!(iPos2));
            let __v2178: i64 = iPos2;
            let __v2179: i64 = __v2178 - ((2 as i32) as i64);
            iPos2 = __v2179;
            if iPos1 < ((0 as i32) as i64) || iPos2 < ((0 as i32) as i64) {
                break '__slate_break_1717;
            }
            '__slate_break_1718: while (1 as i32) != (0 as i32) {
                if iPos2 == iPos1 + (nToken as i64)
                    || isExact == (0 as i32) && iPos2 > iPos1 && iPos2 <= iPos1 + (nToken as i64)
                {
                    let mut iSave: i64 = 0 as i64;
                    iSave = if isSaveLeft != (0 as i32) {
                        iPos1
                    } else {
                        iPos2
                    };
                    fts3PutDeltaVarint(
                        std::ptr::addr_of_mut!(p),
                        std::ptr::addr_of_mut!(iPrev),
                        iSave + ((2 as i32) as i64),
                    );
                    let __v2180: i64 = iPrev;
                    let __v2181: i64 = __v2180 - ((2 as i32) as i64);
                    iPrev = __v2181;
                    pSave = std::ptr::null_mut::<i8>();
                    0 as i32;
                }
                if !(isSaveLeft != (0 as i32)) && iPos2 <= iPos1 + (nToken as i64) || iPos2 <= iPos1
                {
                    if ((unsafe { *p2 }) as i32) & (254 as i32) == (0 as i32) {
                        break '__slate_break_1718;
                    }
                    fts3GetDeltaVarint(std::ptr::addr_of_mut!(p2), std::ptr::addr_of_mut!(iPos2));
                    let __v2182: i64 = iPos2;
                    let __v2183: i64 = __v2182 - ((2 as i32) as i64);
                    iPos2 = __v2183;
                } else {
                    if ((unsafe { *p1 }) as i32) & (254 as i32) == (0 as i32) {
                        break '__slate_break_1718;
                    }
                    fts3GetDeltaVarint(std::ptr::addr_of_mut!(p1), std::ptr::addr_of_mut!(iPos1));
                    let __v2184: i64 = iPos1;
                    let __v2185: i64 = __v2184 - ((2 as i32) as i64);
                    iPos1 = __v2185;
                }
            }
            if pSave != std::ptr::null_mut::<i8>() {
                0 as i32;
                p = pSave;
            }
            fts3ColumnlistCopy(std::ptr::null_mut::<*mut i8>(), std::ptr::addr_of_mut!(p1));
            fts3ColumnlistCopy(std::ptr::null_mut::<*mut i8>(), std::ptr::addr_of_mut!(p2));
            0 as i32;
            if (0 as i32) == ((unsafe { *p1 }) as i32) || (0 as i32) == ((unsafe { *p2 }) as i32) {
                break '__slate_break_1717;
            }
            let __v2186: *mut i8 = p1;
            let __v2187: *mut i8 = unsafe { __v2186.offset((1 as i32) as isize) };
            p1 = __v2187;
            let __v2188: *mut i8 = p1;
            let __v2189: i32;
            if (((unsafe { *(p1 as *mut u8) }) as u32) as i32) & (128 as i32) != (0 as i32) {
                __v2189 = sqlite3Fts3GetVarint32(p1 as *const i8, std::ptr::addr_of_mut!(iCol1));
            } else {
                unsafe {
                    *std::ptr::addr_of_mut!(iCol1) = ((unsafe { *(p1 as *mut u8) }) as u32) as i32;
                }
                __v2189 = 1 as i32;
            }
            let __v2190: *mut i8 = unsafe { __v2188.offset(__v2189 as isize) };
            p1 = __v2190;
            let __v2191: *mut i8 = p2;
            let __v2192: *mut i8 = unsafe { __v2191.offset((1 as i32) as isize) };
            p2 = __v2192;
            let __v2193: *mut i8 = p2;
            let __v2194: i32;
            if (((unsafe { *(p2 as *mut u8) }) as u32) as i32) & (128 as i32) != (0 as i32) {
                __v2194 = sqlite3Fts3GetVarint32(p2 as *const i8, std::ptr::addr_of_mut!(iCol2));
            } else {
                unsafe {
                    *std::ptr::addr_of_mut!(iCol2) = ((unsafe { *(p2 as *mut u8) }) as u32) as i32;
                }
                __v2194 = 1 as i32;
            }
            let __v2195: *mut i8 = unsafe { __v2193.offset(__v2194 as isize) };
            p2 = __v2195;
        } else {
            if iCol1 < iCol2 {
                fts3ColumnlistCopy(std::ptr::null_mut::<*mut i8>(), std::ptr::addr_of_mut!(p1));
                if (0 as i32) == ((unsafe { *p1 }) as i32) {
                    break '__slate_break_1717;
                }
                let __v2196: *mut i8 = p1;
                let __v2197: *mut i8 = unsafe { __v2196.offset((1 as i32) as isize) };
                p1 = __v2197;
                let __v2198: *mut i8 = p1;
                let __v2199: i32;
                if (((unsafe { *(p1 as *mut u8) }) as u32) as i32) & (128 as i32) != (0 as i32) {
                    __v2199 =
                        sqlite3Fts3GetVarint32(p1 as *const i8, std::ptr::addr_of_mut!(iCol1));
                } else {
                    unsafe {
                        *std::ptr::addr_of_mut!(iCol1) =
                            ((unsafe { *(p1 as *mut u8) }) as u32) as i32;
                    }
                    __v2199 = 1 as i32;
                }
                let __v2200: *mut i8 = unsafe { __v2198.offset(__v2199 as isize) };
                p1 = __v2200;
            } else {
                fts3ColumnlistCopy(std::ptr::null_mut::<*mut i8>(), std::ptr::addr_of_mut!(p2));
                if (0 as i32) == ((unsafe { *p2 }) as i32) {
                    break '__slate_break_1717;
                }
                let __v2201: *mut i8 = p2;
                let __v2202: *mut i8 = unsafe { __v2201.offset((1 as i32) as isize) };
                p2 = __v2202;
                let __v2203: *mut i8 = p2;
                let __v2204: i32;
                if (((unsafe { *(p2 as *mut u8) }) as u32) as i32) & (128 as i32) != (0 as i32) {
                    __v2204 =
                        sqlite3Fts3GetVarint32(p2 as *const i8, std::ptr::addr_of_mut!(iCol2));
                } else {
                    unsafe {
                        *std::ptr::addr_of_mut!(iCol2) =
                            ((unsafe { *(p2 as *mut u8) }) as u32) as i32;
                    }
                    __v2204 = 1 as i32;
                }
                let __v2205: *mut i8 = unsafe { __v2203.offset(__v2204 as isize) };
                p2 = __v2205;
            }
        }
        // Advance pointer p1 or p2 (whichever corresponds to the smaller of
        // iCol1 and iCol2) so that it points to either the 0x00 that marks the
        // end of the position list, or the 0x01 that precedes the next
        // column-number in the position list.
    }
    fts3PoslistCopy(std::ptr::null_mut::<*mut i8>(), std::ptr::addr_of_mut!(p2));
    fts3PoslistCopy(std::ptr::null_mut::<*mut i8>(), std::ptr::addr_of_mut!(p1));
    unsafe {
        *pp1 = p1;
    }
    unsafe {
        *pp2 = p2;
    }
    if (unsafe { *pp }) == p {
        return 0 as i32;
    }
    let __v2206: *mut i8 = p;
    let __v2207: *mut i8 = unsafe { __v2206.offset((1 as i32) as isize) };
    p = __v2207;
    unsafe {
        *__v2206 = (0 as i32) as i8;
    }
    unsafe {
        *pp = p;
    }
    return 1 as i32;
}

/// Merge two position-lists as required by the NEAR operator. The argument
/// position lists correspond to the left and right phrases of an expression
/// like:
///
///     "phrase 1" NEAR "phrase number 2"
///
/// Position list *pp1 corresponds to the left-hand side of the NEAR
/// expression and *pp2 to the right. As usual, the indexes in the position
/// lists are the offsets of the last token in each phrase (tokens "1" and "2"
/// in the example above).
///
/// The output position list - written to *pp - is a copy of *pp2 with those
/// entries that are not sufficiently NEAR entries in *pp1 removed.
///
/// # Arguments
///
/// * `pp` - Output buffer
/// * `aTmp` - Temporary buffer space
/// * `nRight` - Maximum difference in token positions
/// * `nLeft` - Maximum difference in token positions
/// * `pp1` - IN/OUT: Left input list
/// * `pp2` - IN/OUT: Right input list
fn fts3PoslistNearMerge(
    mut pp: *mut *mut i8,
    mut aTmp: *mut i8,
    mut nRight: i32,
    mut nLeft: i32,
    mut pp1: *mut *mut i8,
    mut pp2: *mut *mut i8,
) -> i32 {
    let mut p1: *mut i8 = unsafe { *pp1 };
    let mut p2: *mut i8 = unsafe { *pp2 };
    let mut pTmp1: *mut i8 = aTmp;
    let mut pTmp2: *mut i8 = unsafe { std::mem::zeroed() };
    let mut aTmp2: *mut i8 = unsafe { std::mem::zeroed() };
    let mut res: i32 = 1 as i32;
    fts3PoslistPhraseMerge(
        std::ptr::addr_of_mut!(pTmp1),
        nRight,
        0 as i32,
        0 as i32,
        pp1,
        pp2,
    );
    let __v2208: *mut i8 = pTmp1;
    pTmp2 = __v2208;
    aTmp2 = __v2208;
    unsafe {
        *pp1 = p1;
    }
    unsafe {
        *pp2 = p2;
    }
    fts3PoslistPhraseMerge(
        std::ptr::addr_of_mut!(pTmp2),
        nLeft,
        1 as i32,
        0 as i32,
        pp2,
        pp1,
    );
    if pTmp1 != aTmp && pTmp2 != aTmp2 {
        fts3PoslistMerge(
            pp,
            std::ptr::addr_of_mut!(aTmp),
            std::ptr::addr_of_mut!(aTmp2),
        );
    } else {
        if pTmp1 != aTmp {
            fts3PoslistCopy(pp, std::ptr::addr_of_mut!(aTmp));
        } else {
            if pTmp2 != aTmp2 {
                fts3PoslistCopy(pp, std::ptr::addr_of_mut!(aTmp2));
            } else {
                res = 0 as i32;
            }
        }
    }
    return res;
}

/// An instance of this function is used to merge together the (potentially
/// large number of) doclists for each term that matches a prefix query.
/// See function fts3TermSelectMerge() for details.
#[repr(C)]
#[derive(Clone, Copy)]
struct TermSelect {
    /// Malloc'd output buffers
    aaOutput: [*mut i8; 16],
    /// Size each output buffer in bytes
    anOutput: [i32; 16],
}

/// This function is used to read a single varint from a buffer. Parameter
/// pEnd points 1 byte past the end of the buffer. When this function is
/// called, if *pp points to pEnd or greater, then the end of the buffer
/// has been reached. In this case *pp is set to 0 and the function returns.
///
/// If *pp does not point to or past pEnd, then a single varint is read
/// from *pp. *pp is then set to point 1 byte past the end of the read varint.
///
/// If bDescIdx is false, the value read is added to *pVal before returning.
/// If it is true, the value read is subtracted from *pVal before this
/// function returns.
///
/// # Arguments
///
/// * `pp` - IN/OUT: Point to read varint from
/// * `pEnd` - End of buffer
/// * `bDescIdx` - True if docids are descending
/// * `pVal` - IN/OUT: Integer value
fn fts3GetDeltaVarint3(
    mut pp: *mut *mut i8,
    mut pEnd: *mut i8,
    mut bDescIdx: i32,
    mut pVal: *mut i64,
) {
    if (unsafe { *pp }) >= pEnd {
        unsafe {
            *pp = std::ptr::null_mut::<i8>();
        }
    } else {
        let mut iVal: u64 = 0 as u64;
        let __v2209: *mut *mut i8 = pp;
        let __v2210: *mut i8 = unsafe { *__v2209 };
        let __v2211: *mut i8 = unsafe {
            __v2210.offset(sqlite3Fts3GetVarintU(
                (unsafe { *pp }) as *const i8,
                std::ptr::addr_of_mut!(iVal),
            ) as isize)
        };
        unsafe {
            *__v2209 = __v2211;
        }
        if bDescIdx != (0 as i32) {
            unsafe {
                *pVal = ((unsafe { *pVal }) as u64).wrapping_sub(iVal) as i64;
            }
        } else {
            unsafe {
                *pVal = ((unsafe { *pVal }) as u64).wrapping_add(iVal) as i64;
            }
        }
    }
}

/// This function is used to write a single varint to a buffer. The varint
/// is written to *pp. Before returning, *pp is set to point 1 byte past the
/// end of the value written.
///
/// If *pbFirst is zero when this function is called, the value written to
/// the buffer is that of parameter iVal.
///
/// If *pbFirst is non-zero when this function is called, then the value
/// written is either (iVal-*piPrev) (if bDescIdx is zero) or (*piPrev-iVal)
/// (if bDescIdx is non-zero).
///
/// Before returning, this function always sets *pbFirst to 1 and *piPrev
/// to the value of parameter iVal.
///
/// # Arguments
///
/// * `pp` - IN/OUT: Output pointer
/// * `bDescIdx` - True for descending docids
/// * `piPrev` - IN/OUT: Previous value written to list
/// * `pbFirst` - IN/OUT: True after first int written
/// * `iVal` - Write this value to the list
fn fts3PutDeltaVarint3(
    mut pp: *mut *mut i8,
    mut bDescIdx: i32,
    mut piPrev: *mut i64,
    mut pbFirst: *mut i32,
    mut iVal: i64,
) {
    let mut iWrite: u64 = 0 as u64;
    if bDescIdx == (0 as i32) || (unsafe { *pbFirst }) == (0 as i32) {
        0 as i32;
        iWrite = (iVal as u64).wrapping_sub((unsafe { *piPrev }) as u64);
    } else {
        0 as i32;
        iWrite = ((unsafe { *piPrev }) as u64).wrapping_sub(iVal as u64);
    }
    0 as i32;
    0 as i32;
    let __v2212: *mut *mut i8 = pp;
    let __v2213: *mut i8 = unsafe { *__v2212 };
    let __v2214: *mut i8 =
        unsafe { __v2213.offset(sqlite3Fts3PutVarint(unsafe { *pp }, iWrite as i64) as isize) };
    unsafe {
        *__v2212 = __v2214;
    }
    unsafe {
        *piPrev = iVal;
    }
    unsafe {
        *pbFirst = 1 as i32;
    }
}

// This macro is used by various functions that merge doclists. The two
// arguments are 64-bit docid values. If the value of the stack variable
// bDescDoclist is 0 when this macro is invoked, then it returns (i1-i2).
// Otherwise, (i2-i1).
//
// Using this makes it easier to write code that can merge doclists that are
// sorted in either ascending or descending order.
//
// #define DOCID_CMP(i1, i2) ((bDescDoclist?-1:1) * (i64)((u64)i1-i2))
/// This function does an "OR" merge of two doclists (output contains all
/// positions contained in either argument doclist). If the docids in the
/// input doclists are sorted in ascending order, parameter bDescDoclist
/// should be false. If they are sorted in ascending order, it should be
/// passed a non-zero value.
///
/// If no error occurs, *paOut is set to point at an sqlite3_malloc'd buffer
/// containing the output doclist and SQLITE_OK is returned. In this case
/// *pnOut is set to the number of bytes in the output doclist.
///
/// If an error occurs, an SQLite error code is returned. The output values
/// are undefined in this case.
///
/// # Arguments
///
/// * `bDescDoclist` - True if arguments are desc
/// * `n1` - First doclist
/// * `n2` - Second doclist
/// * `pnOut` - OUT: Malloc'd doclist
fn fts3DoclistOrMerge(
    mut bDescDoclist: i32,
    mut a1: *mut i8,
    mut n1: i32,
    mut a2: *mut i8,
    mut n2: i32,
    mut paOut: *mut *mut i8,
    mut pnOut: *mut i32,
) -> i32 {
    let mut rc: i32 = 0 as i32;
    let mut i1: i64 = (0 as i32) as i64;
    let mut i2: i64 = (0 as i32) as i64;
    let mut iPrev: i64 = (0 as i32) as i64;
    let mut pEnd1: *mut i8 = unsafe { a1.offset(n1 as isize) };
    let mut pEnd2: *mut i8 = unsafe { a2.offset(n2 as isize) };
    let mut p1: *mut i8 = a1;
    let mut p2: *mut i8 = a2;
    let mut p: *mut i8 = unsafe { std::mem::zeroed() };
    let mut aOut: *mut i8 = unsafe { std::mem::zeroed() };
    let mut bFirstOut: i32 = 0 as i32;
    unsafe {
        *paOut = std::ptr::null_mut::<i8>();
    }
    unsafe {
        *pnOut = 0 as i32;
    }
    // Allocate space for the output. Both the input and output doclists
    // are delta encoded. If they are in ascending order (bDescDoclist==0),
    // then the first docid in each list is simply encoded as a varint. For
    // each subsequent docid, the varint stored is the difference between the
    // current and previous docid (a positive number - since the list is in
    // ascending order).
    //
    // The first docid written to the output is therefore encoded using the
    // same number of bytes as it is in whichever of the input lists it is
    // read from. And each subsequent docid read from the same input list
    // consumes either the same or less bytes as it did in the input (since
    // the difference between it and the previous value in the output must
    // be a positive value less than or equal to the delta value read from
    // the input list). The same argument applies to all but the first docid
    // read from the 'other' list. And to the contents of all position lists
    // that will be copied and merged from the input to the output.
    //
    // However, if the first docid copied to the output is a negative number,
    // then the encoding of the first docid from the 'other' input list may
    // be larger in the output than it was in the input (since the delta value
    // may be a larger positive integer than the actual docid).
    //
    // The space required to store the output is therefore the sum of the
    // sizes of the two inputs, plus enough space for exactly one of the input
    // docids to grow.
    //
    // A symmetric argument may be made if the doclists are in descending
    // order.
    aOut = (unsafe {
        sqlite3_malloc64(
            ((n1 as i64) + (n2 as i64) + ((10 as i32) as i64) - ((1 as i32) as i64)
                + ((8 as i32) as i64)) as u64,
        )
    }) as *mut i8;
    if !(aOut != std::ptr::null_mut::<i8>()) {
        return 7 as i32;
    }
    p = aOut;
    fts3GetDeltaVarint3(
        std::ptr::addr_of_mut!(p1),
        pEnd1,
        0 as i32,
        std::ptr::addr_of_mut!(i1),
    );
    fts3GetDeltaVarint3(
        std::ptr::addr_of_mut!(p2),
        pEnd2,
        0 as i32,
        std::ptr::addr_of_mut!(i2),
    );
    '__slate_break_1719: while p1 != std::ptr::null_mut::<i8>() || p2 != std::ptr::null_mut::<i8>()
    {
        let mut iDiff: i64 = ((if bDescDoclist != (0 as i32) {
            -(1 as i32)
        } else {
            1 as i32
        }) * if i1 > i2 {
            1 as i32
        } else {
            if i1 == i2 { 0 as i32 } else { -(1 as i32) }
        }) as i64;
        if p2 != std::ptr::null_mut::<i8>()
            && p1 != std::ptr::null_mut::<i8>()
            && iDiff == ((0 as i32) as i64)
        {
            fts3PutDeltaVarint3(
                std::ptr::addr_of_mut!(p),
                bDescDoclist,
                std::ptr::addr_of_mut!(iPrev),
                std::ptr::addr_of_mut!(bFirstOut),
                i1,
            );
            rc = fts3PoslistMerge(
                std::ptr::addr_of_mut!(p),
                std::ptr::addr_of_mut!(p1),
                std::ptr::addr_of_mut!(p2),
            );
            if rc != (0 as i32) {
                break '__slate_break_1719;
            }
            fts3GetDeltaVarint3(
                std::ptr::addr_of_mut!(p1),
                pEnd1,
                bDescDoclist,
                std::ptr::addr_of_mut!(i1),
            );
            fts3GetDeltaVarint3(
                std::ptr::addr_of_mut!(p2),
                pEnd2,
                bDescDoclist,
                std::ptr::addr_of_mut!(i2),
            );
        } else {
            if !(p2 != std::ptr::null_mut::<i8>())
                || p1 != std::ptr::null_mut::<i8>() && iDiff < ((0 as i32) as i64)
            {
                fts3PutDeltaVarint3(
                    std::ptr::addr_of_mut!(p),
                    bDescDoclist,
                    std::ptr::addr_of_mut!(iPrev),
                    std::ptr::addr_of_mut!(bFirstOut),
                    i1,
                );
                fts3PoslistCopy(std::ptr::addr_of_mut!(p), std::ptr::addr_of_mut!(p1));
                fts3GetDeltaVarint3(
                    std::ptr::addr_of_mut!(p1),
                    pEnd1,
                    bDescDoclist,
                    std::ptr::addr_of_mut!(i1),
                );
            } else {
                fts3PutDeltaVarint3(
                    std::ptr::addr_of_mut!(p),
                    bDescDoclist,
                    std::ptr::addr_of_mut!(iPrev),
                    std::ptr::addr_of_mut!(bFirstOut),
                    i2,
                );
                fts3PoslistCopy(std::ptr::addr_of_mut!(p), std::ptr::addr_of_mut!(p2));
                fts3GetDeltaVarint3(
                    std::ptr::addr_of_mut!(p2),
                    pEnd2,
                    bDescDoclist,
                    std::ptr::addr_of_mut!(i2),
                );
            }
        }
        0 as i32;
    }
    if rc != (0 as i32) {
        unsafe { sqlite3_free(aOut as *mut ()) };
        aOut = std::ptr::null_mut::<i8>();
        p = std::ptr::null_mut::<i8>();
    } else {
        0 as i32;
        unsafe {
            memset(
                (unsafe {
                    aOut.offset(((unsafe { p.offset_from(aOut as *mut i8) }) as i64) as isize)
                }) as *mut (),
                0 as i32,
                ((8 as i32) as i64) as u64,
            )
        };
    }
    unsafe {
        *paOut = aOut;
    }
    unsafe {
        *pnOut = ((unsafe { p.offset_from(aOut as *mut i8) }) as i64) as i32;
    }
    return rc;
}

/// This function does a "phrase" merge of two doclists. In a phrase merge,
/// the output contains a copy of each position from the right-hand input
/// doclist for which there is a position in the left-hand input doclist
/// exactly nDist tokens before it.
///
/// If the docids in the input doclists are sorted in ascending order,
/// parameter bDescDoclist should be false. If they are sorted in ascending
/// order, it should be passed a non-zero value.
///
/// The right-hand input doclist is overwritten by this function.
///
/// # Arguments
///
/// * `bDescDoclist` - True if arguments are desc
/// * `nDist` - Distance from left to right (1=adjacent)
/// * `nLeft` - Left doclist
/// * `pnRight` - IN/OUT: Right/output doclist
fn fts3DoclistPhraseMerge(
    mut bDescDoclist: i32,
    mut nDist: i32,
    mut aLeft: *mut i8,
    mut nLeft: i32,
    mut paRight: *mut *mut i8,
    mut pnRight: *mut i32,
) -> i32 {
    let mut i1: i64 = (0 as i32) as i64;
    let mut i2: i64 = (0 as i32) as i64;
    let mut iPrev: i64 = (0 as i32) as i64;
    let mut aRight: *mut i8 = unsafe { *paRight };
    let mut pEnd1: *mut i8 = unsafe { aLeft.offset(nLeft as isize) };
    let mut pEnd2: *mut i8 = unsafe { aRight.offset((unsafe { *pnRight }) as isize) };
    let mut p1: *mut i8 = aLeft;
    let mut p2: *mut i8 = aRight;
    let mut p: *mut i8 = unsafe { std::mem::zeroed() };
    let mut bFirstOut: i32 = 0 as i32;
    let mut aOut: *mut i8 = unsafe { std::mem::zeroed() };
    0 as i32;
    if bDescDoclist != (0 as i32) {
        aOut = (unsafe {
            sqlite3_malloc64((((unsafe { *pnRight }) as i64) + ((10 as i32) as i64)) as u64)
        }) as *mut i8;
        if aOut == std::ptr::null_mut::<i8>() {
            return 7 as i32;
        }
    } else {
        aOut = aRight;
    }
    p = aOut;
    fts3GetDeltaVarint3(
        std::ptr::addr_of_mut!(p1),
        pEnd1,
        0 as i32,
        std::ptr::addr_of_mut!(i1),
    );
    fts3GetDeltaVarint3(
        std::ptr::addr_of_mut!(p2),
        pEnd2,
        0 as i32,
        std::ptr::addr_of_mut!(i2),
    );
    '__slate_break_1720: while p1 != std::ptr::null_mut::<i8>() && p2 != std::ptr::null_mut::<i8>()
    {
        let mut iDiff: i64 = ((if bDescDoclist != (0 as i32) {
            -(1 as i32)
        } else {
            1 as i32
        }) * if i1 > i2 {
            1 as i32
        } else {
            if i1 == i2 { 0 as i32 } else { -(1 as i32) }
        }) as i64;
        if iDiff == ((0 as i32) as i64) {
            let mut pSave: *mut i8 = p;
            let mut iPrevSave: i64 = iPrev;
            let mut bFirstOutSave: i32 = bFirstOut;
            fts3PutDeltaVarint3(
                std::ptr::addr_of_mut!(p),
                bDescDoclist,
                std::ptr::addr_of_mut!(iPrev),
                std::ptr::addr_of_mut!(bFirstOut),
                i1,
            );
            if (0 as i32)
                == fts3PoslistPhraseMerge(
                    std::ptr::addr_of_mut!(p),
                    nDist,
                    0 as i32,
                    1 as i32,
                    std::ptr::addr_of_mut!(p1),
                    std::ptr::addr_of_mut!(p2),
                )
            {
                p = pSave;
                iPrev = iPrevSave;
                bFirstOut = bFirstOutSave;
            }
            fts3GetDeltaVarint3(
                std::ptr::addr_of_mut!(p1),
                pEnd1,
                bDescDoclist,
                std::ptr::addr_of_mut!(i1),
            );
            fts3GetDeltaVarint3(
                std::ptr::addr_of_mut!(p2),
                pEnd2,
                bDescDoclist,
                std::ptr::addr_of_mut!(i2),
            );
        } else {
            if iDiff < ((0 as i32) as i64) {
                fts3PoslistCopy(std::ptr::null_mut::<*mut i8>(), std::ptr::addr_of_mut!(p1));
                fts3GetDeltaVarint3(
                    std::ptr::addr_of_mut!(p1),
                    pEnd1,
                    bDescDoclist,
                    std::ptr::addr_of_mut!(i1),
                );
            } else {
                fts3PoslistCopy(std::ptr::null_mut::<*mut i8>(), std::ptr::addr_of_mut!(p2));
                fts3GetDeltaVarint3(
                    std::ptr::addr_of_mut!(p2),
                    pEnd2,
                    bDescDoclist,
                    std::ptr::addr_of_mut!(i2),
                );
            }
        }
    }
    unsafe {
        *pnRight = ((unsafe { p.offset_from(aOut as *mut i8) }) as i64) as i32;
    }
    if bDescDoclist != (0 as i32) {
        unsafe { sqlite3_free(aRight as *mut ()) };
        unsafe {
            *paRight = aOut;
        }
    }
    return 0 as i32;
}

/// Argument pList points to a position list nList bytes in size. This
/// function checks to see if the position list contains any entries for
/// a token in position 0 (of any column). If so, it writes argument iDelta
/// to the output buffer pOut, followed by a position list consisting only
/// of the entries from pList at position 0, and terminated by an 0x00 byte.
/// The value returned is the number of bytes written to pOut (if any).
///
/// # Arguments
///
/// * `iDelta` - Varint that may be written to pOut
/// * `pList` - Position list (no 0x00 term)
/// * `nList` - Size of pList in bytes
/// * `pOut` - Write output here
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3Fts3FirstFilter(
    mut iDelta: i64,
    mut pList: *mut i8,
    mut nList: i32,
    mut pOut: *mut i8,
) -> i32 {
    let mut nOut: i32 = 0 as i32;
    let mut bWritten: i32 = 0 as i32; // True once iDelta has been written
    let mut p: *mut i8 = pList;
    let mut pEnd: *mut i8 = unsafe { pList.offset(nList as isize) };
    if ((unsafe { *p }) as i32) != (1 as i32) {
        if ((unsafe { *p }) as i32) == (2 as i32) {
            let __v1902: i32 = nOut;
            let __v1903: i32 =
                __v1902 + sqlite3Fts3PutVarint(unsafe { pOut.offset(nOut as isize) }, iDelta);
            nOut = __v1903;
            let __v1904: i32 = nOut;
            let __v1905: i32 = __v1904 + (1 as i32);
            nOut = __v1905;
            unsafe {
                *unsafe { pOut.offset(__v1904 as isize) } = (2 as i32) as i8;
            }
            bWritten = 1 as i32;
        }
        fts3ColumnlistCopy(std::ptr::null_mut::<*mut i8>(), std::ptr::addr_of_mut!(p));
    }
    '__slate_break_1721: while p < pEnd {
        let mut iCol: i64 = 0 as i64;
        let __v1906: *mut i8 = p;
        let __v1907: *mut i8 = unsafe { __v1906.offset((1 as i32) as isize) };
        p = __v1907;
        let __v1908: *mut i8 = p;
        let __v1909: *mut i8 = unsafe {
            __v1908
                .offset(sqlite3Fts3GetVarint(p as *const i8, std::ptr::addr_of_mut!(iCol)) as isize)
        };
        p = __v1909;
        if ((unsafe { *p }) as i32) == (2 as i32) {
            if bWritten == (0 as i32) {
                let __v1910: i32 = nOut;
                let __v1911: i32 =
                    __v1910 + sqlite3Fts3PutVarint(unsafe { pOut.offset(nOut as isize) }, iDelta);
                nOut = __v1911;
                bWritten = 1 as i32;
            }
            let __v1912: i32 = nOut;
            let __v1913: i32 = __v1912 + (1 as i32);
            nOut = __v1913;
            unsafe {
                *unsafe { pOut.offset(__v1912 as isize) } = (1 as i32) as i8;
            }
            let __v1914: i32 = nOut;
            let __v1915: i32 =
                __v1914 + sqlite3Fts3PutVarint(unsafe { pOut.offset(nOut as isize) }, iCol);
            nOut = __v1915;
            let __v1916: i32 = nOut;
            let __v1917: i32 = __v1916 + (1 as i32);
            nOut = __v1917;
            unsafe {
                *unsafe { pOut.offset(__v1916 as isize) } = (2 as i32) as i8;
            }
        }
        fts3ColumnlistCopy(std::ptr::null_mut::<*mut i8>(), std::ptr::addr_of_mut!(p));
    }
    if bWritten != (0 as i32) {
        let __v1918: i32 = nOut;
        let __v1919: i32 = __v1918 + (1 as i32);
        nOut = __v1919;
        unsafe {
            *unsafe { pOut.offset(__v1918 as isize) } = (0 as i32) as i8;
        }
    }
    return nOut;
}

/// Merge all doclists in the TermSelect.aaOutput[] array into a single
/// doclist stored in TermSelect.aaOutput[0]. If successful, delete all
/// other doclists (except the aaOutput[0] one) and return SQLITE_OK.
///
/// If an OOM error occurs, return SQLITE_NOMEM. In this case it is
/// the responsibility of the caller to free any doclists left in the
/// TermSelect.aaOutput[] array.
fn fts3TermSelectFinishMerge(mut p: *mut Fts3Table, mut pTS: *mut TermSelect) -> i32 {
    let mut aOut: *mut i8 = std::ptr::null_mut::<i8>();
    let mut nOut: i32 = 0 as i32;
    let mut i: i32 = 0 as i32;
    // Loop through the doclists in the aaOutput[] array. Merge them all
    // into a single doclist.
    i = 0 as i32;
    '__slate_break_1722: loop {
        if !(i < ((((128 as u64) / (8 as u64)) as u32) as i32)) {
            break;
        }
        if (unsafe {
            *unsafe { unsafe { (*pTS).aaOutput.as_mut_ptr() as *mut *mut i8 }.offset(i as isize) }
        }) != std::ptr::null_mut::<i8>()
        {
            if !(aOut != std::ptr::null_mut::<i8>()) {
                aOut = unsafe {
                    *unsafe {
                        unsafe { (*pTS).aaOutput.as_mut_ptr() as *mut *mut i8 }.offset(i as isize)
                    }
                };
                nOut = unsafe {
                    *unsafe {
                        unsafe { (*pTS).anOutput.as_mut_ptr() as *mut i32 }.offset(i as isize)
                    }
                };
                unsafe {
                    *unsafe {
                        unsafe { (*pTS).aaOutput.as_mut_ptr() as *mut *mut i8 }.offset(i as isize)
                    } = std::ptr::null_mut::<i8>();
                }
            } else {
                let mut nNew: i32 = 0 as i32;
                let mut aNew: *mut i8 = unsafe { std::mem::zeroed() };
                let mut rc: i32 = fts3DoclistOrMerge(
                    ((unsafe { (*p).bDescIdx }) as u32) as i32,
                    unsafe {
                        *unsafe {
                            unsafe { (*pTS).aaOutput.as_mut_ptr() as *mut *mut i8 }
                                .offset(i as isize)
                        }
                    },
                    unsafe {
                        *unsafe {
                            unsafe { (*pTS).anOutput.as_mut_ptr() as *mut i32 }.offset(i as isize)
                        }
                    },
                    aOut,
                    nOut,
                    std::ptr::addr_of_mut!(aNew),
                    std::ptr::addr_of_mut!(nNew),
                );
                if rc != (0 as i32) {
                    unsafe { sqlite3_free(aOut as *mut ()) };
                    return rc;
                }
                unsafe {
                    sqlite3_free(
                        (unsafe {
                            *unsafe {
                                unsafe { (*pTS).aaOutput.as_mut_ptr() as *mut *mut i8 }
                                    .offset(i as isize)
                            }
                        }) as *mut (),
                    )
                };
                unsafe { sqlite3_free(aOut as *mut ()) };
                unsafe {
                    *unsafe {
                        unsafe { (*pTS).aaOutput.as_mut_ptr() as *mut *mut i8 }.offset(i as isize)
                    } = std::ptr::null_mut::<i8>();
                }
                aOut = aNew;
                nOut = nNew;
            }
        }
        let __v2215: i32 = i;
        let __v2216: i32 = __v2215 + (1 as i32);
        i = __v2216;
    }
    unsafe {
        *unsafe {
            unsafe { (*pTS).aaOutput.as_mut_ptr() as *mut *mut i8 }.offset((0 as i32) as isize)
        } = aOut;
    }
    unsafe {
        *unsafe {
            unsafe { (*pTS).anOutput.as_mut_ptr() as *mut i32 }.offset((0 as i32) as isize)
        } = nOut;
    }
    return 0 as i32;
}

/// Merge the doclist aDoclist/nDoclist into the TermSelect object passed
/// as the first argument. The merge is an "OR" merge (see function
/// fts3DoclistOrMerge() for details).
///
/// This function is called with the doclist for each term that matches
/// a queried prefix. It merges all these doclists into one, the doclist
/// for the specified prefix. Since there can be a very large number of
/// doclists to merge, the merging is done pair-wise using the TermSelect
/// object.
///
/// This function returns SQLITE_OK if the merge is successful, or an
/// SQLite error code (SQLITE_NOMEM) if an error occurs.
///
/// # Arguments
///
/// * `p` - FTS table handle
/// * `pTS` - TermSelect object to merge into
/// * `aDoclist` - Pointer to doclist
/// * `nDoclist` - Size of aDoclist in bytes
fn fts3TermSelectMerge(
    mut p: *mut Fts3Table,
    mut pTS: *mut TermSelect,
    mut aDoclist: *mut i8,
    mut nDoclist: i32,
) -> i32 {
    if (unsafe {
        *unsafe {
            unsafe { (*pTS).aaOutput.as_mut_ptr() as *mut *mut i8 }.offset((0 as i32) as isize)
        }
    }) == std::ptr::null_mut::<i8>()
    {
        // If this is the first term selected, copy the doclist to the output
        // buffer using memcpy().
        //
        // Add FTS3_VARINT_MAX bytes of unused space to the end of the
        // allocation. This is so as to ensure that the buffer is big enough
        // to hold the current doclist AND'd with any other doclist. If the
        // doclists are stored in order=ASC order, this padding would not be
        // required (since the size of [doclistA AND doclistB] is always less
        // than or equal to the size of [doclistA] in that case). But this is
        // not true for order=DESC. For example, a doclist containing (1, -1)
        // may be smaller than (-1), as in the first example the -1 may be stored
        // as a single-byte delta, whereas in the second it must be stored as a
        // FTS3_VARINT_MAX byte varint.
        //
        // Similar padding is added in the fts3DoclistOrMerge() function.
        unsafe {
            *unsafe {
                unsafe { (*pTS).aaOutput.as_mut_ptr() as *mut *mut i8 }.offset((0 as i32) as isize)
            } = (unsafe {
                sqlite3_malloc64(
                    ((nDoclist as i64) + ((10 as i32) as i64) + ((1 as i32) as i64)) as u64,
                )
            }) as *mut i8;
        }
        unsafe {
            *unsafe {
                unsafe { (*pTS).anOutput.as_mut_ptr() as *mut i32 }.offset((0 as i32) as isize)
            } = nDoclist;
        }
        if (unsafe {
            *unsafe {
                unsafe { (*pTS).aaOutput.as_mut_ptr() as *mut *mut i8 }.offset((0 as i32) as isize)
            }
        }) != std::ptr::null_mut::<i8>()
        {
            unsafe {
                memcpy(
                    (unsafe {
                        *unsafe {
                            unsafe { (*pTS).aaOutput.as_mut_ptr() as *mut *mut i8 }
                                .offset((0 as i32) as isize)
                        }
                    }) as *mut (),
                    aDoclist as *const (),
                    (nDoclist as i64) as u64,
                )
            };
            unsafe {
                memset(
                    (unsafe {
                        unsafe {
                            *unsafe {
                                unsafe { (*pTS).aaOutput.as_mut_ptr() as *mut *mut i8 }
                                    .offset((0 as i32) as isize)
                            }
                        }
                        .offset(nDoclist as isize)
                    }) as *mut (),
                    0 as i32,
                    ((10 as i32) as i64) as u64,
                )
            };
        } else {
            return 7 as i32;
        }
    } else {
        let mut aMerge: *mut i8 = aDoclist;
        let mut nMerge: i32 = nDoclist;
        let mut iOut: i32 = 0 as i32;
        iOut = 0 as i32;
        '__slate_break_1723: loop {
            if !(iOut < ((((128 as u64) / (8 as u64)) as u32) as i32)) {
                break;
            }
            if (unsafe {
                *unsafe {
                    unsafe { (*pTS).aaOutput.as_mut_ptr() as *mut *mut i8 }.offset(iOut as isize)
                }
            }) == std::ptr::null_mut::<i8>()
            {
                0 as i32;
                unsafe {
                    *unsafe {
                        unsafe { (*pTS).aaOutput.as_mut_ptr() as *mut *mut i8 }
                            .offset(iOut as isize)
                    } = aMerge;
                }
                unsafe {
                    *unsafe {
                        unsafe { (*pTS).anOutput.as_mut_ptr() as *mut i32 }.offset(iOut as isize)
                    } = nMerge;
                }
                break '__slate_break_1723;
            } else {
                let mut aNew: *mut i8 = unsafe { std::mem::zeroed() };
                let mut nNew: i32 = 0 as i32;
                let mut rc: i32 = fts3DoclistOrMerge(
                    ((unsafe { (*p).bDescIdx }) as u32) as i32,
                    aMerge,
                    nMerge,
                    unsafe {
                        *unsafe {
                            unsafe { (*pTS).aaOutput.as_mut_ptr() as *mut *mut i8 }
                                .offset(iOut as isize)
                        }
                    },
                    unsafe {
                        *unsafe {
                            unsafe { (*pTS).anOutput.as_mut_ptr() as *mut i32 }
                                .offset(iOut as isize)
                        }
                    },
                    std::ptr::addr_of_mut!(aNew),
                    std::ptr::addr_of_mut!(nNew),
                );
                if rc != (0 as i32) {
                    if aMerge != aDoclist {
                        unsafe { sqlite3_free(aMerge as *mut ()) };
                    }
                    return rc;
                }
                if aMerge != aDoclist {
                    unsafe { sqlite3_free(aMerge as *mut ()) };
                }
                unsafe {
                    sqlite3_free(
                        (unsafe {
                            *unsafe {
                                unsafe { (*pTS).aaOutput.as_mut_ptr() as *mut *mut i8 }
                                    .offset(iOut as isize)
                            }
                        }) as *mut (),
                    )
                };
                unsafe {
                    *unsafe {
                        unsafe { (*pTS).aaOutput.as_mut_ptr() as *mut *mut i8 }
                            .offset(iOut as isize)
                    } = std::ptr::null_mut::<i8>();
                }
                aMerge = aNew;
                nMerge = nNew;
                if iOut + (1 as i32) == ((((128 as u64) / (8 as u64)) as u32) as i32) {
                    unsafe {
                        *unsafe {
                            unsafe { (*pTS).aaOutput.as_mut_ptr() as *mut *mut i8 }
                                .offset(iOut as isize)
                        } = aMerge;
                    }
                    unsafe {
                        *unsafe {
                            unsafe { (*pTS).anOutput.as_mut_ptr() as *mut i32 }
                                .offset(iOut as isize)
                        } = nMerge;
                    }
                }
            }
            let __v2217: i32 = iOut;
            let __v2218: i32 = __v2217 + (1 as i32);
            iOut = __v2218;
        }
    }
    return 0 as i32;
}

/// Append SegReader object pNew to the end of the pCsr->apSegment[] array.
fn fts3SegReaderCursorAppend(
    mut pCsr: *mut Fts3MultiSegReader,
    mut pNew: *mut Fts3SegReader,
) -> i32 {
    if (unsafe { (*pCsr).nSegment }) % (16 as i32) == (0 as i32) {
        let mut apNew: *mut *mut Fts3SegReader = unsafe { std::mem::zeroed() };
        let mut nByte: i64 = ((((unsafe { (*pCsr).nSegment }) + (16 as i32)) as i64) as u64)
            .wrapping_mul(8 as u64) as i64;
        apNew =
            (unsafe { sqlite3_realloc64((unsafe { (*pCsr).apSegment }) as *mut (), nByte as u64) })
                as *mut *mut Fts3SegReader;
        if !(apNew != std::ptr::null_mut::<*mut Fts3SegReader>()) {
            unsafe { sqlite3Fts3SegReaderFree(pNew) };
            return 7 as i32;
        }
        unsafe {
            (*pCsr).apSegment = apNew;
        }
    }
    let __v2219: *mut Fts3MultiSegReader = pCsr;
    let __v2220: i32 = unsafe { (*__v2219).nSegment };
    let __v2221: i32 = __v2220 + (1 as i32);
    unsafe {
        (*__v2219).nSegment = __v2221;
    }
    unsafe {
        *unsafe { unsafe { (*pCsr).apSegment }.offset(__v2220 as isize) } = pNew;
    }
    return 0 as i32;
}

/// Add seg-reader objects to the Fts3MultiSegReader object passed as the
/// 8th argument.
///
/// This function returns SQLITE_OK if successful, or an SQLite error code
/// otherwise.
///
/// # Arguments
///
/// * `p` - FTS3 table handle
/// * `iLangid` - Language id
/// * `iIndex` - Index to search (from 0 to p->nIndex-1)
/// * `iLevel` - Level of segments to scan
/// * `zTerm` - Term to query for
/// * `nTerm` - Size of zTerm in bytes
/// * `isPrefix` - True for a prefix search
/// * `isScan` - True to scan from zTerm to EOF
/// * `pCsr` - Cursor object to populate
fn fts3SegReaderCursor(
    mut p: *mut Fts3Table,
    mut iLangid: i32,
    mut iIndex: i32,
    mut iLevel: i32,
    mut zTerm: *const i8,
    mut nTerm: i32,
    mut isPrefix: i32,
    mut isScan: i32,
    mut pCsr: *mut Fts3MultiSegReader,
) -> i32 {
    let mut __slate_storage_2223: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2223: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_2223) as *mut i32;
    let mut __slate_storage_2222: std::mem::MaybeUninit<bool> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2222: *mut bool = std::ptr::addr_of_mut!(__slate_storage_2222) as *mut bool;
    let mut __slate_storage_795: std::mem::MaybeUninit<*mut i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_795: *mut *mut i64 =
        std::ptr::addr_of_mut!(__slate_storage_795) as *mut *mut i64;
    let mut __slate_storage_794: std::mem::MaybeUninit<*const i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_794: *mut *const i8 =
        std::ptr::addr_of_mut!(__slate_storage_794) as *mut *const i8;
    let mut __slate_storage_793: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_793: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_793) as *mut i32;
    let mut __slate_storage_792: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_792: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_792) as *mut i64;
    let mut __slate_storage_791: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_791: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_791) as *mut i64;
    // Read the values returned by the SELECT into local variables.
    let mut __slate_storage_790: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_790: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_790) as *mut i64;
    let mut __slate_storage_789: std::mem::MaybeUninit<*mut Fts3SegReader> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_789: *mut *mut Fts3SegReader =
        std::ptr::addr_of_mut!(__slate_storage_789) as *mut *mut Fts3SegReader;
    let mut __slate_storage_788: std::mem::MaybeUninit<*mut Fts3SegReader> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_788: *mut *mut Fts3SegReader =
        std::ptr::addr_of_mut!(__slate_storage_788) as *mut *mut Fts3SegReader; // Result of sqlite3_reset()
    let mut __slate_storage_787: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_787: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_787) as *mut i32; // Statement to iterate through segments
    let mut __slate_storage_786: std::mem::MaybeUninit<*mut sqlite3_stmt> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_786: *mut *mut sqlite3_stmt =
        std::ptr::addr_of_mut!(__slate_storage_786) as *mut *mut sqlite3_stmt; // Error code
    let mut __slate_storage_785: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_785: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_785) as *mut i32;
    unsafe {
        '__join_15: {
            std::ptr::write(__slate_slot_785, 0 as i32);
            std::ptr::write(__slate_slot_786, std::ptr::null_mut::<sqlite3_stmt>());
            // If iLevel is less than 0 and this is not a scan, include a seg-reader
            // for the pending-terms. If this is a scan, then this call must be being
            // made by an fts4aux module, not an FTS table. In this case calling
            // Fts3SegReaderPending might segfault, as the data structures used by
            // fts4aux are not completely populated. So it's easiest to filter these
            // calls out here.
            if iLevel < (0 as i32)
                && (unsafe { (*p).aIndex }) != std::ptr::null_mut::<Fts3Index>()
                && (unsafe { (*p).iPrevLangid }) == iLangid
            {
                std::ptr::write(__slate_slot_788, std::ptr::null_mut::<Fts3SegReader>());
                *__slate_slot_785 = unsafe {
                    sqlite3Fts3SegReaderPending(
                        p,
                        iIndex,
                        zTerm,
                        nTerm,
                        (isPrefix != (0 as i32) || isScan != (0 as i32)) as i32,
                        std::ptr::addr_of_mut!(*__slate_slot_788),
                    )
                };
                if *__slate_slot_785 == (0 as i32)
                    && *__slate_slot_788 != std::ptr::null_mut::<Fts3SegReader>()
                {
                    *__slate_slot_785 = fts3SegReaderCursorAppend(pCsr, *__slate_slot_788);
                }
            }
        }
        '__join_2: {
            if iLevel != -(1 as i32) {
                if *__slate_slot_785 == (0 as i32) {
                    *__slate_slot_785 = unsafe {
                        sqlite3Fts3AllSegdirs(
                            p,
                            iLangid,
                            iIndex,
                            iLevel,
                            std::ptr::addr_of_mut!(*__slate_slot_786),
                        )
                    };
                }
                loop {
                    if *__slate_slot_785 == (0 as i32) {
                        std::ptr::write(__slate_slot_2223, unsafe {
                            sqlite3_step(*__slate_slot_786)
                        });
                        *__slate_slot_785 = *__slate_slot_2223;
                        *__slate_slot_2222 = (100 as i32) == *__slate_slot_2223;
                    } else {
                        *__slate_slot_2222 = false as bool;
                    }
                    if *__slate_slot_2222 {
                        std::ptr::write(__slate_slot_789, std::ptr::null_mut::<Fts3SegReader>());
                        std::ptr::write(__slate_slot_790, unsafe {
                            sqlite3_column_int64(*__slate_slot_786, 1 as i32)
                        });
                        std::ptr::write(__slate_slot_791, unsafe {
                            sqlite3_column_int64(*__slate_slot_786, 2 as i32)
                        });
                        std::ptr::write(__slate_slot_792, unsafe {
                            sqlite3_column_int64(*__slate_slot_786, 3 as i32)
                        });
                        std::ptr::write(__slate_slot_793, unsafe {
                            sqlite3_column_bytes(*__slate_slot_786, 4 as i32)
                        });
                        std::ptr::write(
                            __slate_slot_794,
                            (unsafe { sqlite3_column_blob(*__slate_slot_786, 4 as i32) })
                                as *const i8,
                        );
                        // If zTerm is not NULL, and this segment is not stored entirely on its
                        // root node, the range of leaves scanned can be reduced. Do this.
                        if *__slate_slot_790 != (0 as i64)
                            && zTerm != std::ptr::null::<i8>()
                            && *__slate_slot_794 != std::ptr::null::<i8>()
                        {
                            std::ptr::write(
                                __slate_slot_795,
                                if isPrefix != (0 as i32) {
                                    std::ptr::addr_of_mut!(*__slate_slot_791)
                                } else {
                                    std::ptr::null_mut::<i64>()
                                },
                            );
                            *__slate_slot_785 = fts3SelectLeaf(
                                p,
                                zTerm,
                                nTerm,
                                *__slate_slot_794,
                                *__slate_slot_793,
                                std::ptr::addr_of_mut!(*__slate_slot_790),
                                *__slate_slot_795,
                            );
                            if *__slate_slot_785 != (0 as i32) {
                                break '__join_2;
                            } else {
                                if isPrefix == (0 as i32) && isScan == (0 as i32) {
                                    *__slate_slot_791 = *__slate_slot_790;
                                }
                            }
                        }
                        *__slate_slot_785 = unsafe {
                            sqlite3Fts3SegReaderNew(
                                (unsafe { (*pCsr).nSegment }) + (1 as i32),
                                (isPrefix == (0 as i32) && isScan == (0 as i32)) as i32,
                                *__slate_slot_790,
                                *__slate_slot_791,
                                *__slate_slot_792,
                                *__slate_slot_794,
                                *__slate_slot_793,
                                std::ptr::addr_of_mut!(*__slate_slot_789),
                            )
                        };
                        if *__slate_slot_785 != (0 as i32) {
                            break '__join_2;
                        } else {
                            *__slate_slot_785 = fts3SegReaderCursorAppend(pCsr, *__slate_slot_789);
                        }
                    } else {
                        break '__join_2;
                    }
                }
            }
        }
        *__slate_slot_787 = unsafe { sqlite3_reset(*__slate_slot_786) };
        if *__slate_slot_785 == (101 as i32) {
            *__slate_slot_785 = *__slate_slot_787;
        }
        return *__slate_slot_785;
    }
    return unsafe { std::mem::zeroed() };
}

/// Set up a cursor object for iterating through a full-text index or a
/// single level therein.
///
/// # Arguments
///
/// * `p` - FTS3 table handle
/// * `iLangid` - Language-id to search
/// * `iIndex` - Index to search (from 0 to p->nIndex-1)
/// * `iLevel` - Level of segments to scan
/// * `zTerm` - Term to query for
/// * `nTerm` - Size of zTerm in bytes
/// * `isPrefix` - True for a prefix search
/// * `isScan` - True to scan from zTerm to EOF
/// * `pCsr` - Cursor object to populate
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3Fts3SegReaderCursor(
    mut p: *mut Fts3Table,
    mut iLangid: i32,
    mut iIndex: i32,
    mut iLevel: i32,
    mut zTerm: *const i8,
    mut nTerm: i32,
    mut isPrefix: i32,
    mut isScan: i32,
    mut pCsr: *mut Fts3MultiSegReader,
) -> i32 {
    0 as i32;
    0 as i32;
    0 as i32;
    0 as i32;
    0 as i32;
    unsafe { memset(pCsr as *mut (), 0 as i32, 88 as u64) };
    return fts3SegReaderCursor(
        p, iLangid, iIndex, iLevel, zTerm, nTerm, isPrefix, isScan, pCsr,
    );
}

/// In addition to its current configuration, have the Fts3MultiSegReader
/// passed as the 4th argument also scan the doclist for term zTerm/nTerm.
///
/// SQLITE_OK is returned if no error occurs, otherwise an SQLite error code.
///
/// # Arguments
///
/// * `p` - FTS virtual table handle
/// * `zTerm` - Term to scan doclist of
/// * `nTerm` - Number of bytes in zTerm
/// * `pCsr` - Fts3MultiSegReader to modify
fn fts3SegReaderCursorAddZero(
    mut p: *mut Fts3Table,
    mut iLangid: i32,
    mut zTerm: *const i8,
    mut nTerm: i32,
    mut pCsr: *mut Fts3MultiSegReader,
) -> i32 {
    return fts3SegReaderCursor(
        p,
        iLangid,
        0 as i32,
        -(2 as i32),
        zTerm,
        nTerm,
        0 as i32,
        0 as i32,
        pCsr,
    );
}

/// Open an Fts3MultiSegReader to scan the doclist for term zTerm/nTerm. Or,
/// if isPrefix is true, to scan the doclist for all terms for which
/// zTerm/nTerm is a prefix. If successful, return SQLITE_OK and write
/// a pointer to the new Fts3MultiSegReader to *ppSegcsr. Otherwise, return
/// an SQLite error code.
///
/// It is the responsibility of the caller to free this object by eventually
/// passing it to fts3SegReaderCursorFree()
///
/// SQLITE_OK is returned if no error occurs, otherwise an SQLite error code.
/// Output parameter *ppSegcsr is set to 0 if an error occurs.
///
/// # Arguments
///
/// * `pCsr` - Virtual table cursor handle
/// * `zTerm` - Term to query for
/// * `nTerm` - Size of zTerm in bytes
/// * `isPrefix` - True for a prefix search
/// * `ppSegcsr` - OUT: Allocated seg-reader cursor
fn fts3TermSegReaderCursor(
    mut pCsr: *mut Fts3Cursor,
    mut zTerm: *const i8,
    mut nTerm: i32,
    mut isPrefix: i32,
    mut ppSegcsr: *mut *mut Fts3MultiSegReader,
) -> i32 {
    let mut pSegcsr: *mut Fts3MultiSegReader = unsafe { std::mem::zeroed() }; // Object to allocate and return
    let mut rc: i32 = 7 as i32; // Return code
    pSegcsr = (unsafe { sqlite3_malloc(((88 as u64) as u32) as i32) }) as *mut Fts3MultiSegReader;
    if pSegcsr != std::ptr::null_mut::<Fts3MultiSegReader>() {
        let mut i: i32 = 0 as i32;
        let mut bFound: i32 = 0 as i32; // True once an index has been found
        let mut p: *mut Fts3Table = (unsafe { (*pCsr).base.pVtab }) as *mut Fts3Table;
        if isPrefix != (0 as i32) {
            i = 1 as i32;
            '__slate_break_1725: loop {
                if !(bFound == (0 as i32) && i < unsafe { (*p).nIndex }) {
                    break;
                }
                if (unsafe { (*unsafe { unsafe { (*p).aIndex }.offset(i as isize) }).nPrefix })
                    == nTerm
                {
                    bFound = 1 as i32;
                    rc = sqlite3Fts3SegReaderCursor(
                        p,
                        unsafe { (*pCsr).iLangid },
                        i,
                        -(2 as i32),
                        zTerm,
                        nTerm,
                        0 as i32,
                        0 as i32,
                        pSegcsr,
                    );
                    unsafe {
                        (*pSegcsr).bLookup = 1 as i32;
                    }
                }
                let __v1960: i32 = i;
                let __v1961: i32 = __v1960 + (1 as i32);
                i = __v1961;
            }
            i = 1 as i32;
            '__slate_break_1726: loop {
                if !(bFound == (0 as i32) && i < unsafe { (*p).nIndex }) {
                    break;
                }
                if (unsafe { (*unsafe { unsafe { (*p).aIndex }.offset(i as isize) }).nPrefix })
                    == nTerm + (1 as i32)
                {
                    bFound = 1 as i32;
                    rc = sqlite3Fts3SegReaderCursor(
                        p,
                        unsafe { (*pCsr).iLangid },
                        i,
                        -(2 as i32),
                        zTerm,
                        nTerm,
                        1 as i32,
                        0 as i32,
                        pSegcsr,
                    );
                    if rc == (0 as i32) {
                        rc = fts3SegReaderCursorAddZero(
                            p,
                            unsafe { (*pCsr).iLangid },
                            zTerm,
                            nTerm,
                            pSegcsr,
                        );
                    }
                }
                let __v1962: i32 = i;
                let __v1963: i32 = __v1962 + (1 as i32);
                i = __v1963;
            }
        }
        if bFound == (0 as i32) {
            rc = sqlite3Fts3SegReaderCursor(
                p,
                unsafe { (*pCsr).iLangid },
                0 as i32,
                -(2 as i32),
                zTerm,
                nTerm,
                isPrefix,
                0 as i32,
                pSegcsr,
            );
            unsafe {
                (*pSegcsr).bLookup = !(isPrefix != (0 as i32)) as i32;
            }
        }
    }
    unsafe {
        *ppSegcsr = pSegcsr;
    }
    return rc;
}

/// Free an Fts3MultiSegReader allocated by fts3TermSegReaderCursor().
fn fts3SegReaderCursorFree(mut pSegcsr: *mut Fts3MultiSegReader) {
    unsafe { sqlite3Fts3SegReaderFinish(pSegcsr) };
    unsafe { sqlite3_free(pSegcsr as *mut ()) };
}

/// This function retrieves the doclist for the specified term (or term
/// prefix) from the database.
///
/// # Arguments
///
/// * `p` - Virtual table handle
/// * `pTok` - Token to query for
/// * `iColumn` - Column to query (or -ve for all columns)
/// * `pnOut` - OUT: Size of buffer at *ppOut
/// * `ppOut` - OUT: Malloced result buffer
fn fts3TermSelect(
    mut p: *mut Fts3Table,
    mut pTok: *mut Fts3PhraseToken,
    mut iColumn: i32,
    mut pnOut: *mut i32,
    mut ppOut: *mut *mut i8,
) -> i32 {
    let mut rc: i32 = 0 as i32; // Return code
    let mut pSegcsr: *mut Fts3MultiSegReader = unsafe { std::mem::zeroed() }; // Seg-reader cursor for this term
    let mut tsc: TermSelect = unsafe { std::mem::zeroed() }; // Object for pair-wise doclist merging
    let mut filter: Fts3SegFilter = unsafe { std::mem::zeroed() }; // Segment term filter configuration
    pSegcsr = unsafe { (*pTok).pSegcsr };
    unsafe { memset(std::ptr::addr_of_mut!(tsc) as *mut (), 0 as i32, 192 as u64) };
    filter.flags = (2 as i32)
        | (1 as i32)
        | if (unsafe { (*pTok).isPrefix }) != (0 as i32) {
            8 as i32
        } else {
            0 as i32
        }
        | if (unsafe { (*pTok).bFirst }) != (0 as i32) {
            32 as i32
        } else {
            0 as i32
        }
        | if iColumn < unsafe { (*p).nColumn } {
            4 as i32
        } else {
            0 as i32
        };
    filter.iCol = iColumn;
    filter.zTerm = (unsafe { (*pTok).z }) as *const i8;
    filter.nTerm = unsafe { (*pTok).n };
    rc = unsafe { sqlite3Fts3SegReaderStart(p, pSegcsr, std::ptr::addr_of_mut!(filter)) };
    '__slate_break_1727: loop {
        let __v2224: bool;
        if (0 as i32) == rc {
            let __v2225: i32 = unsafe { sqlite3Fts3SegReaderStep(p, pSegcsr) };
            rc = __v2225;
            __v2224 = (100 as i32) == __v2225;
        } else {
            __v2224 = false as bool;
        }
        if !__v2224 {
            break;
        }
        rc = fts3TermSelectMerge(
            p,
            std::ptr::addr_of_mut!(tsc),
            unsafe { (*pSegcsr).aDoclist },
            unsafe { (*pSegcsr).nDoclist },
        );
    }
    if rc == (0 as i32) {
        rc = fts3TermSelectFinishMerge(p, std::ptr::addr_of_mut!(tsc));
    }
    if rc == (0 as i32) {
        unsafe {
            *ppOut = unsafe {
                *unsafe { (tsc.aaOutput.as_mut_ptr() as *mut *mut i8).offset((0 as i32) as isize) }
            };
        }
        unsafe {
            *pnOut = unsafe {
                *unsafe { (tsc.anOutput.as_mut_ptr() as *mut i32).offset((0 as i32) as isize) }
            };
        }
    } else {
        let mut i: i32 = 0 as i32;
        i = 0 as i32;
        '__slate_break_1728: loop {
            if !(i < ((((128 as u64) / (8 as u64)) as u32) as i32)) {
                break;
            }
            unsafe {
                sqlite3_free(
                    (unsafe {
                        *unsafe { (tsc.aaOutput.as_mut_ptr() as *mut *mut i8).offset(i as isize) }
                    }) as *mut (),
                )
            };
            let __v2226: i32 = i;
            let __v2227: i32 = __v2226 + (1 as i32);
            i = __v2227;
        }
    }
    fts3SegReaderCursorFree(pSegcsr);
    unsafe {
        (*pTok).pSegcsr = std::ptr::null_mut::<Fts3MultiSegReader>();
    }
    return rc;
}

/// This function counts the total number of docids in the doclist stored
/// in buffer aList[], size nList bytes.
///
/// If the isPoslist argument is true, then it is assumed that the doclist
/// contains a position-list following each docid. Otherwise, it is assumed
/// that the doclist is simply a list of docids stored as delta encoded
/// varints.
fn fts3DoclistCountDocids(mut aList: *mut i8, mut nList: i32) -> i32 {
    let mut nDoc: i32 = 0 as i32; // Return value
    if aList != std::ptr::null_mut::<i8>() {
        let mut aEnd: *mut i8 = unsafe { aList.offset(nList as isize) }; // Pointer to one byte after EOF
        let mut p: *mut i8 = aList; // Cursor
        '__slate_break_1729: while p < aEnd {
            let __v2228: i32 = nDoc;
            let __v2229: i32 = __v2228 + (1 as i32);
            nDoc = __v2229;
            '__slate_break_1730: loop {
                let __v2230: *mut i8 = p;
                let __v2231: *mut i8 = unsafe { __v2230.offset((1 as i32) as isize) };
                p = __v2231;
                if !(((unsafe { *__v2230 }) as i32) & (128 as i32) != (0 as i32)) {
                    break;
                }
                {}
            }
            // Skip docid varint
            fts3PoslistCopy(std::ptr::null_mut::<*mut i8>(), std::ptr::addr_of_mut!(p)); // Skip over position list
        }
    }
    return nDoc;
}

/// Advance the cursor to the next row in the %_content table that
/// matches the search criteria.  For a MATCH search, this will be
/// the next row that matches. For a full-table scan, this will be
/// simply the next row in the %_content table.  For a docid lookup,
/// this routine simply sets the EOF flag.
///
/// Return SQLITE_OK if nothing goes wrong.  SQLITE_OK is returned
/// even if we reach end-of-file.  The fts3EofMethod() will be called
/// subsequently to determine whether or not an EOF was hit.
#[unsafe(link_section = ".text.slate_distinct.fts3.fts3NextMethod")]
extern "C-unwind" fn fts3NextMethod(mut pCursor: *mut sqlite3_vtab_cursor) -> i32 {
    let mut rc: i32 = 0 as i32;
    let mut pCsr: *mut Fts3Cursor = pCursor as *mut Fts3Cursor;
    if ((unsafe { (*pCsr).eSearch }) as i32) == (1 as i32)
        || ((unsafe { (*pCsr).eSearch }) as i32) == (0 as i32)
    {
        let mut pTab: *mut Fts3Table = (unsafe { (*pCursor).pVtab }) as *mut Fts3Table;
        let __v2232: *mut Fts3Table = pTab;
        let __v2233: i32 = unsafe { (*__v2232).bLock };
        let __v2234: i32 = __v2233 + (1 as i32);
        unsafe {
            (*__v2232).bLock = __v2234;
        }
        if (100 as i32) != unsafe { sqlite3_step(unsafe { (*pCsr).pStmt }) } {
            unsafe {
                (*pCsr).isEof = ((1 as i32) as i8) as u8;
            }
            rc = unsafe { sqlite3_reset(unsafe { (*pCsr).pStmt }) };
        } else {
            unsafe {
                (*pCsr).iPrevId =
                    unsafe { sqlite3_column_int64(unsafe { (*pCsr).pStmt }, 0 as i32) };
            }
            rc = 0 as i32;
        }
        let __v2235: *mut Fts3Table = pTab;
        let __v2236: i32 = unsafe { (*__v2235).bLock };
        let __v2237: i32 = __v2236 - (1 as i32);
        unsafe {
            (*__v2235).bLock = __v2237;
        }
    } else {
        rc = fts3EvalNext(pCursor as *mut Fts3Cursor);
    }
    0 as i32;
    return rc;
}

/// If the numeric type of argument pVal is "integer", then return it
/// converted to a 64-bit signed integer. Otherwise, return a copy of
/// the second parameter, iDefault.
fn fts3DocidRange(mut pVal: *mut sqlite3_value, mut iDefault: i64) -> i64 {
    if pVal != std::ptr::null_mut::<sqlite3_value>() {
        let mut eType: i32 = unsafe { sqlite3_value_numeric_type(pVal) };
        if eType == (1 as i32) {
            return unsafe { sqlite3_value_int64(pVal) };
        }
    }
    return iDefault;
}

/// This is the xFilter interface for the virtual table.  See
/// the virtual table xFilter method documentation for additional
/// information.
///
/// If idxNum==FTS3_FULLSCAN_SEARCH then do a full table scan against
/// the %_content table.
///
/// If idxNum==FTS3_DOCID_SEARCH then do a docid lookup for a single entry
/// in the %_content table.
///
/// If idxNum>=FTS3_FULLTEXT_SEARCH then use the full text index.  The
/// column on the left-hand side of the MATCH operator is column
/// number idxNum-FTS3_FULLTEXT_SEARCH, 0 indexed.  argv[0] is the right-hand
/// side of the MATCH operator.
///
/// # Arguments
///
/// * `pCursor` - The cursor used for this query
/// * `idxNum` - Strategy index
/// * `idxStr` - Unused
/// * `nVal` - Number of elements in apVal
/// * `apVal` - Arguments for the indexing scheme
#[unsafe(link_section = ".text.slate_distinct.fts3.fts3FilterMethod")]
extern "C-unwind" fn fts3FilterMethod(
    mut pCursor: *mut sqlite3_vtab_cursor,
    mut idxNum: i32,
    mut idxStr: *const i8,
    mut nVal: i32,
    mut apVal: *mut *mut sqlite3_value,
) -> i32 {
    let mut rc: i32 = 0 as i32;
    let mut zSql: *mut i8 = unsafe { std::mem::zeroed() }; // SQL statement used to access %_content
    let mut eSearch: i32 = 0 as i32;
    let mut p: *mut Fts3Table = (unsafe { (*pCursor).pVtab }) as *mut Fts3Table;
    let mut pCsr: *mut Fts3Cursor = pCursor as *mut Fts3Cursor;
    let mut pCons: *mut sqlite3_value = std::ptr::null_mut::<sqlite3_value>(); // The MATCH or rowid constraint, if any
    let mut pLangid: *mut sqlite3_value = std::ptr::null_mut::<sqlite3_value>(); // The "langid = ?" constraint, if any
    let mut pDocidGe: *mut sqlite3_value = std::ptr::null_mut::<sqlite3_value>(); // The "docid >= ?" constraint, if any
    let mut pDocidLe: *mut sqlite3_value = std::ptr::null_mut::<sqlite3_value>(); // The "docid <= ?" constraint, if any
    let mut iIdx: i32 = 0 as i32;
    idxStr;
    nVal;
    if (unsafe { (*p).bLock }) != (0 as i32) {
        return 1 as i32;
    }
    eSearch = idxNum & (65535 as i32);
    0 as i32;
    0 as i32;
    // Collect arguments into local variables
    iIdx = 0 as i32;
    if eSearch != (0 as i32) {
        let __v2238: i32 = iIdx;
        let __v2239: i32 = __v2238 + (1 as i32);
        iIdx = __v2239;
        pCons = unsafe { *unsafe { apVal.offset(__v2238 as isize) } };
    }
    if idxNum & (65536 as i32) != (0 as i32) {
        let __v2240: i32 = iIdx;
        let __v2241: i32 = __v2240 + (1 as i32);
        iIdx = __v2241;
        pLangid = unsafe { *unsafe { apVal.offset(__v2240 as isize) } };
    }
    if idxNum & (131072 as i32) != (0 as i32) {
        let __v2242: i32 = iIdx;
        let __v2243: i32 = __v2242 + (1 as i32);
        iIdx = __v2243;
        pDocidGe = unsafe { *unsafe { apVal.offset(__v2242 as isize) } };
    }
    if idxNum & (262144 as i32) != (0 as i32) {
        let __v2244: i32 = iIdx;
        let __v2245: i32 = __v2244 + (1 as i32);
        iIdx = __v2245;
        pDocidLe = unsafe { *unsafe { apVal.offset(__v2244 as isize) } };
    }
    0 as i32;
    // In case the cursor has been used before, clear it now.
    fts3ClearCursor(pCsr);
    // Set the lower and upper bounds on docids to return
    unsafe {
        (*pCsr).iMinDocid = fts3DocidRange(
            pDocidGe,
            (-(1 as i32) as i64)
                - ((((4294967295 as u32) as u64) as i64)
                    | ((2147483647 as i32) as i64) << (32 as i32)),
        );
    }
    unsafe {
        (*pCsr).iMaxDocid = fts3DocidRange(
            pDocidLe,
            (((4294967295 as u32) as u64) as i64) | ((2147483647 as i32) as i64) << (32 as i32),
        );
    }
    if idxStr != std::ptr::null::<i8>() {
        unsafe {
            (*pCsr).bDesc = (((unsafe { *unsafe { idxStr.offset((0 as i32) as isize) } }) as i32)
                == (68 as i32)) as u8;
        }
    } else {
        unsafe {
            (*pCsr).bDesc = unsafe { (*p).bDescIdx };
        }
    }
    unsafe {
        (*pCsr).eSearch = eSearch as i16;
    }
    if eSearch != (1 as i32) && eSearch != (0 as i32) {
        let mut iCol: i32 = eSearch - (2 as i32);
        let mut zQuery: *const i8 = (unsafe { sqlite3_value_text(pCons) }) as *const i8;
        let __v2246: bool;
        if zQuery == std::ptr::null::<i8>() {
            __v2246 = (unsafe { sqlite3_value_type(pCons) }) != (5 as i32);
        } else {
            __v2246 = false as bool;
        }
        if __v2246 {
            return 7 as i32;
        }
        unsafe {
            (*pCsr).iLangid = 0 as i32;
        }
        if pLangid != std::ptr::null_mut::<sqlite3_value>() {
            unsafe {
                (*pCsr).iLangid = unsafe { sqlite3_value_int(pLangid) };
            }
        }
        0 as i32;
        rc = unsafe {
            sqlite3Fts3ExprParse(
                unsafe { (*p).pTokenizer },
                unsafe { (*pCsr).iLangid },
                unsafe { (*p).azColumn },
                ((unsafe { (*p).bFts4 }) as u32) as i32,
                unsafe { (*p).nColumn },
                iCol,
                zQuery,
                -(1 as i32),
                unsafe { std::ptr::addr_of_mut!((*pCsr).pExpr) },
                unsafe { std::ptr::addr_of_mut!((*p).base.zErrMsg) },
            )
        };
        if rc != (0 as i32) {
            return rc;
        }
        rc = fts3EvalStart(pCsr);
        unsafe { sqlite3Fts3SegmentsClose(p) };
        if rc != (0 as i32) {
            return rc;
        }
        unsafe {
            (*pCsr).pNextId = unsafe { (*pCsr).aDoclist };
        }
        unsafe {
            (*pCsr).iPrevId = (0 as i32) as i64;
        }
    }
    // Compile a SELECT statement for this cursor. For a full-table-scan, the
    // statement loops through all rows of the %_content table. For a
    // full-text query or docid lookup, the statement retrieves a single
    // row by docid.
    if eSearch == (0 as i32) {
        if pDocidGe != std::ptr::null_mut::<sqlite3_value>()
            || pDocidLe != std::ptr::null_mut::<sqlite3_value>()
        {
            zSql = unsafe {
                sqlite3_mprintf(
                    (b"SELECT %s WHERE rowid BETWEEN %lld AND %lld ORDER BY rowid %s\0".as_ptr()
                        as *mut i8) as *const i8,
                    unsafe { (*p).zReadExprlist },
                    unsafe { (*pCsr).iMinDocid },
                    unsafe { (*pCsr).iMaxDocid },
                    if (unsafe { (*pCsr).bDesc }) != (0 as u8) {
                        b"DESC\0".as_ptr() as *mut i8
                    } else {
                        b"ASC\0".as_ptr() as *mut i8
                    },
                )
            };
        } else {
            zSql = unsafe {
                sqlite3_mprintf(
                    (b"SELECT %s ORDER BY rowid %s\0".as_ptr() as *mut i8) as *const i8,
                    unsafe { (*p).zReadExprlist },
                    if (unsafe { (*pCsr).bDesc }) != (0 as u8) {
                        b"DESC\0".as_ptr() as *mut i8
                    } else {
                        b"ASC\0".as_ptr() as *mut i8
                    },
                )
            };
        }
        if zSql != std::ptr::null_mut::<i8>() {
            let __v2247: *mut Fts3Table = p;
            let __v2248: i32 = unsafe { (*__v2247).bLock };
            let __v2249: i32 = __v2248 + (1 as i32);
            unsafe {
                (*__v2247).bLock = __v2249;
            }
            rc = unsafe {
                sqlite3Fts3PrepareStmt(p, zSql as *const i8, 1 as i32, 1 as i32, unsafe {
                    std::ptr::addr_of_mut!((*pCsr).pStmt)
                })
            };
            let __v2250: *mut Fts3Table = p;
            let __v2251: i32 = unsafe { (*__v2250).bLock };
            let __v2252: i32 = __v2251 - (1 as i32);
            unsafe {
                (*__v2250).bLock = __v2252;
            }
            unsafe { sqlite3_free(zSql as *mut ()) };
        } else {
            rc = 7 as i32;
        }
    } else {
        if eSearch == (1 as i32) {
            rc = fts3CursorSeekStmt(pCsr);
            if rc == (0 as i32) {
                rc = unsafe {
                    sqlite3_bind_value(
                        unsafe { (*pCsr).pStmt },
                        1 as i32,
                        pCons as *const sqlite3_value,
                    )
                };
            }
        }
    }
    if rc != (0 as i32) {
        return rc;
    }
    return fts3NextMethod(pCursor);
}

/// This is the xEof method of the virtual table. SQLite calls this
/// routine to find out if it has reached the end of a result set.
#[unsafe(link_section = ".text.slate_distinct.fts3.fts3EofMethod")]
extern "C-unwind" fn fts3EofMethod(mut pCursor: *mut sqlite3_vtab_cursor) -> i32 {
    let mut pCsr: *mut Fts3Cursor = pCursor as *mut Fts3Cursor;
    if (unsafe { (*pCsr).isEof }) != (0 as u8) {
        fts3ClearCursor(pCsr);
        unsafe {
            (*pCsr).isEof = ((1 as i32) as i8) as u8;
        }
    }
    return ((unsafe { (*pCsr).isEof }) as u32) as i32;
}

/// This is the xRowid method. The SQLite core calls this routine to
/// retrieve the rowid for the current row of the result set. fts3
/// exposes %_content.docid as the rowid for the virtual table. The
/// rowid should be written to *pRowid.
#[unsafe(link_section = ".text.slate_distinct.fts3.fts3RowidMethod")]
extern "C-unwind" fn fts3RowidMethod(
    mut pCursor: *mut sqlite3_vtab_cursor,
    mut pRowid: *mut i64,
) -> i32 {
    let mut pCsr: *mut Fts3Cursor = pCursor as *mut Fts3Cursor;
    unsafe {
        *pRowid = unsafe { (*pCsr).iPrevId };
    }
    return 0 as i32;
}

/// This is the xColumn method, called by SQLite to request a value from
/// the row that the supplied cursor currently points to.
///
/// If:
///
///   (iCol <  p->nColumn)   -> The value of the iCol'th user column.
///   (iCol == p->nColumn)   -> Magic column with the same name as the table.
///   (iCol == p->nColumn+1) -> Docid column
///   (iCol == p->nColumn+2) -> Langid column
///
/// # Arguments
///
/// * `pCursor` - Cursor to retrieve value from
/// * `pCtx` - Context for sqlite3_result_xxx() calls
/// * `iCol` - Index of column to read value from
#[unsafe(link_section = ".text.slate_distinct.fts3.fts3ColumnMethod")]
extern "C-unwind" fn fts3ColumnMethod(
    mut pCursor: *mut sqlite3_vtab_cursor,
    mut pCtx: *mut sqlite3_context,
    mut iCol: i32,
) -> i32 {
    let mut rc: i32 = 0 as i32; // Return Code
    let mut pCsr: *mut Fts3Cursor = pCursor as *mut Fts3Cursor;
    let mut p: *mut Fts3Table = (unsafe { (*pCursor).pVtab }) as *mut Fts3Table;
    // The column value supplied by SQLite must be in range.
    0 as i32;
    // The special 'table-name' column
    // The docid column
    // no break
    // A user column. Or, if this is a full-table scan, possibly the
    // language-id column. Seek the cursor.
    '__slate_break_1737: {
        match iCol - unsafe { (*p).nColumn } {
            0 => {
                unsafe {
                    sqlite3_result_pointer(
                        pCtx,
                        pCsr as *mut (),
                        (b"fts3cursor\0".as_ptr() as *mut i8) as *const i8,
                        None,
                    )
                }; // The special 'table-name' column
            }
            1 => {
                unsafe { sqlite3_result_int64(pCtx, unsafe { (*pCsr).iPrevId }) }; // The docid column
            }
            2 => {
                if (unsafe { (*pCsr).pExpr }) != std::ptr::null_mut::<Fts3Expr>() {
                    unsafe { sqlite3_result_int64(pCtx, (unsafe { (*pCsr).iLangid }) as i64) };
                    break '__slate_break_1737;
                } else {
                    if (unsafe { (*p).zLanguageid }) == std::ptr::null_mut::<i8>() {
                        unsafe { sqlite3_result_int(pCtx, 0 as i32) };
                        break '__slate_break_1737;
                    } else {
                        iCol = unsafe { (*p).nColumn };
                        // no break
                        {}
                    }
                }
                rc = fts3CursorSeek(std::ptr::null_mut::<sqlite3_context>(), pCsr);
                // A user column. Or, if this is a full-table scan, possibly the
                // language-id column. Seek the cursor.
                let _v2254: bool;
                if rc == (0 as i32) {
                    _v2254 = (unsafe { sqlite3_data_count(unsafe { (*pCsr).pStmt }) }) - (1 as i32)
                        > iCol;
                } else {
                    _v2254 = false as bool;
                }
                if _v2254 {
                    unsafe {
                        sqlite3_result_value(pCtx, unsafe {
                            sqlite3_column_value(unsafe { (*pCsr).pStmt }, iCol + (1 as i32))
                        })
                    };
                }
            }
            _ => {
                rc = fts3CursorSeek(std::ptr::null_mut::<sqlite3_context>(), pCsr);
                // A user column. Or, if this is a full-table scan, possibly the
                // language-id column. Seek the cursor.
                let __v2253: bool;
                if rc == (0 as i32) {
                    __v2253 = (unsafe { sqlite3_data_count(unsafe { (*pCsr).pStmt }) })
                        - (1 as i32)
                        > iCol;
                } else {
                    __v2253 = false as bool;
                }
                if __v2253 {
                    unsafe {
                        sqlite3_result_value(pCtx, unsafe {
                            sqlite3_column_value(unsafe { (*pCsr).pStmt }, iCol + (1 as i32))
                        })
                    };
                }
            }
        }
    }
    0 as i32;
    return rc;
}

/// This function is the implementation of the xUpdate callback used by
/// FTS3 virtual tables. It is invoked by SQLite each time a row is to be
/// inserted, updated or deleted.
///
/// # Arguments
///
/// * `pVtab` - Virtual table handle
/// * `nArg` - Size of argument array
/// * `apVal` - Array of arguments
/// * `pRowid` - OUT: The affected (or effected) rowid
#[unsafe(link_section = ".text.slate_distinct.fts3.fts3UpdateMethod")]
extern "C-unwind" fn fts3UpdateMethod(
    mut pVtab: *mut sqlite3_vtab,
    mut nArg: i32,
    mut apVal: *mut *mut sqlite3_value,
    mut pRowid: *mut i64,
) -> i32 {
    return unsafe { sqlite3Fts3UpdateMethod(pVtab, nArg, apVal, pRowid) };
}

/// Implementation of xSync() method. Flush the contents of the pending-terms
/// hash-table to the database.
#[unsafe(link_section = ".text.slate_distinct.fts3.fts3SyncMethod")]
extern "C-unwind" fn fts3SyncMethod(mut pVtab: *mut sqlite3_vtab) -> i32 {
    // Following an incremental-merge operation, assuming that the input
    // segments are not completely consumed (the usual case), they are updated
    // in place to remove the entries that have already been merged. This
    // involves updating the leaf block that contains the smallest unmerged
    // entry and each block (if any) between the leaf and the root node. So
    // if the height of the input segment b-trees is N, and input segments
    // are merged eight at a time, updating the input segments at the end
    // of an incremental-merge requires writing (8*(1+N)) blocks. N is usually
    // small - often between 0 and 2. So the overhead of the incremental
    // merge is somewhere between 8 and 24 blocks. To avoid this overhead
    // dwarfing the actual productive work accomplished, the incremental merge
    // is only attempted if it will write at least 64 leaf blocks. Hence
    // nMinMerge.
    //
    // Of course, updating the input segments also involves deleting a bunch
    // of blocks from the segments table. But this is not considered overhead
    // as it would also be required by a crisis-merge that used the same input
    // segments.
    let mut nMinMerge: u32 = (64 as i32) as u32; // Minimum amount of incr-merge work to do
    let mut p: *mut Fts3Table = pVtab as *mut Fts3Table;
    let mut rc: i32 = 0 as i32;
    let mut iLastRowid: i64 = unsafe { sqlite3_last_insert_rowid(unsafe { (*p).db }) };
    rc = unsafe { sqlite3Fts3PendingTermsFlush(p) };
    if rc == (0 as i32)
        && (unsafe { (*p).nLeafAdd }) > nMinMerge / ((16 as i32) as u32)
        && (unsafe { (*p).nAutoincrmerge }) != (0 as i32)
        && (unsafe { (*p).nAutoincrmerge }) != (255 as i32)
    {
        let mut mxLevel: i32 = 0 as i32; // Maximum relative level value in db
        let mut A: i32 = 0 as i32; // Incr-merge parameter A
        rc = unsafe { sqlite3Fts3MaxLevel(p, std::ptr::addr_of_mut!(mxLevel)) };
        0 as i32;
        A = unsafe { (*p).nLeafAdd }.wrapping_mul(mxLevel as u32) as i32;
        let __v2254: i32 = A;
        let __v2255: i32 = __v2254 + A / (2 as i32);
        A = __v2255;
        if A > (nMinMerge as i32) {
            rc = unsafe { sqlite3Fts3Incrmerge(p, A, unsafe { (*p).nAutoincrmerge }) };
        }
    }
    unsafe { sqlite3Fts3SegmentsClose(p) };
    unsafe { sqlite3_set_last_insert_rowid(unsafe { (*p).db }, iLastRowid) };
    return rc;
}

/// If it is currently unknown whether or not the FTS table has an %_stat
/// table (if p->bHasStat==2), attempt to determine this (set p->bHasStat
/// to 0 or 1). Return SQLITE_OK if successful, or an SQLite error code
/// if an error occurs.
fn fts3SetHasStat(mut p: *mut Fts3Table) -> i32 {
    let mut rc: i32 = 0 as i32;
    if (((unsafe { (*p).bHasStat }) as u32) as i32) == (2 as i32) {
        let mut zTbl: *mut i8 = unsafe {
            sqlite3_mprintf((b"%s_stat\0".as_ptr() as *mut i8) as *const i8, unsafe {
                (*p).zName
            })
        };
        if zTbl != std::ptr::null_mut::<i8>() {
            let mut res: i32 = unsafe {
                sqlite3_table_column_metadata(
                    unsafe { (*p).db },
                    unsafe { (*p).zDb },
                    zTbl as *const i8,
                    std::ptr::null::<i8>(),
                    std::ptr::null_mut::<*const i8>(),
                    std::ptr::null_mut::<*const i8>(),
                    std::ptr::null_mut::<i32>(),
                    std::ptr::null_mut::<i32>(),
                    std::ptr::null_mut::<i32>(),
                )
            };
            unsafe { sqlite3_free(zTbl as *mut ()) };
            unsafe {
                (*p).bHasStat = (res == (0 as i32)) as u8;
            }
        } else {
            rc = 7 as i32;
        }
    }
    return rc;
}

/// Implementation of xBegin() method.
#[unsafe(link_section = ".text.slate_distinct.fts3.fts3BeginMethod")]
extern "C-unwind" fn fts3BeginMethod(mut pVtab: *mut sqlite3_vtab) -> i32 {
    let mut p: *mut Fts3Table = pVtab as *mut Fts3Table;
    let mut rc: i32 = 0 as i32;
    pVtab;
    0 as i32;
    0 as i32;
    0 as i32;
    unsafe {
        (*p).nLeafAdd = (0 as i32) as u32;
    }
    rc = fts3SetHasStat(p);
    return rc;
}

/// Implementation of xCommit() method. This is a no-op. The contents of
/// the pending-terms hash-table have already been flushed into the database
/// by fts3SyncMethod().
#[unsafe(link_section = ".text.slate_distinct.fts3.fts3CommitMethod")]
extern "C-unwind" fn fts3CommitMethod(mut pVtab: *mut sqlite3_vtab) -> i32 {
    {}
    pVtab;
    0 as i32;
    0 as i32;
    0 as i32;
    {}
    {}
    return 0 as i32;
}

/// Implementation of xRollback(). Discard the contents of the pending-terms
/// hash-table. Any changes made to the database are reverted by SQLite.
#[unsafe(link_section = ".text.slate_distinct.fts3.fts3RollbackMethod")]
extern "C-unwind" fn fts3RollbackMethod(mut pVtab: *mut sqlite3_vtab) -> i32 {
    let mut p: *mut Fts3Table = pVtab as *mut Fts3Table;
    unsafe { sqlite3Fts3PendingTermsClear(p) };
    0 as i32;
    {}
    {}
    return 0 as i32;
}

/// When called, *ppPoslist must point to the byte immediately following the
/// end of a position-list. i.e. ( (*ppPoslist)[-1]==POS_END ). This function
/// moves *ppPoslist so that it instead points to the first byte of the
/// same position list.
fn fts3ReversePoslist(mut pStart: *mut i8, mut ppPoslist: *mut *mut i8) {
    let mut p: *mut i8 = unsafe { unsafe { *ppPoslist }.offset(-(2 as i32) as isize) };
    let mut c: i8 = (0 as i32) as i8;
    // Skip backwards passed any trailing 0x00 bytes added by NearTrim()
    '__slate_break_1740: loop {
        let __v2256: bool;
        if p > pStart {
            let __v2257: *mut i8 = p;
            let __v2258: *mut i8 = unsafe { __v2257.offset(-((1 as i32) as isize)) };
            p = __v2258;
            let __v2259: i8 = unsafe { *__v2257 };
            c = __v2259;
            __v2256 = (__v2259 as i32) == (0 as i32);
        } else {
            __v2256 = false as bool;
        }
        if !__v2256 {
            break;
        }
        {}
    }
    // Search backwards for a varint with value zero (the end of the previous
    // poslist). This is an 0x00 byte preceded by some byte that does not
    // have the 0x80 bit set.
    '__slate_break_1741: while p > pStart
        && ((unsafe { *p }) as i32) & (128 as i32) | (c as i32) != (0 as i32)
    {
        let __v2260: *mut i8 = p;
        let __v2261: *mut i8 = unsafe { __v2260.offset(-((1 as i32) as isize)) };
        p = __v2261;
        c = unsafe { *__v2260 };
    }
    0 as i32;
    // At this point p points to that preceding byte without the 0x80 bit
    // set. So to find the start of the poslist, skip forward 2 bytes then
    // over a varint.
    //
    // Normally. The other case is that p==pStart and the poslist to return
    // is the first in the doclist. In this case do not skip forward 2 bytes.
    // The second part of the if condition (c==0 && *ppPoslist>&p[2])
    // is required for cases where the first byte of a doclist and the
    // doclist is empty. For example, if the first docid is 10, a doclist
    // that begins with:
    //
    //   0x0A 0x00 <next docid delta varint>
    if p > pStart
        || (c as i32) == (0 as i32)
            && (unsafe { *ppPoslist }) > unsafe { p.offset((2 as i32) as isize) }
    {
        p = unsafe { p.offset((2 as i32) as isize) };
    }
    '__slate_break_1742: loop {
        let __v2262: *mut i8 = p;
        let __v2263: *mut i8 = unsafe { __v2262.offset((1 as i32) as isize) };
        p = __v2263;
        if !(((unsafe { *__v2262 }) as i32) & (128 as i32) != (0 as i32)) {
            break;
        }
        {}
    }
    unsafe {
        *ppPoslist = p;
    }
}

/// Helper function used by the implementation of the overloaded snippet(),
/// offsets() and optimize() SQL functions.
///
/// If the value passed as the third argument is a blob of size
/// sizeof(Fts3Cursor*), then the blob contents are copied to the
/// output variable *ppCsr and SQLITE_OK is returned. Otherwise, an error
/// message is written to context pContext and SQLITE_ERROR returned. The
/// string passed via zFunc is used as part of the error message.
///
/// # Arguments
///
/// * `pContext` - SQL function call context
/// * `zFunc` - Function name
/// * `pVal` - argv[0] passed to function
/// * `ppCsr` - OUT: Store cursor handle here
fn fts3FunctionArg(
    mut pContext: *mut sqlite3_context,
    mut zFunc: *const i8,
    mut pVal: *mut sqlite3_value,
    mut ppCsr: *mut *mut Fts3Cursor,
) -> i32 {
    let mut rc: i32 = 0 as i32;
    unsafe {
        *ppCsr = (unsafe {
            sqlite3_value_pointer(pVal, (b"fts3cursor\0".as_ptr() as *mut i8) as *const i8)
        }) as *mut Fts3Cursor;
    }
    if (unsafe { *ppCsr }) != std::ptr::null_mut::<Fts3Cursor>() {
        rc = 0 as i32;
    } else {
        let mut zErr: *mut i8 = unsafe {
            sqlite3_mprintf(
                (b"illegal first argument to %s\0".as_ptr() as *mut i8) as *const i8,
                zFunc,
            )
        };
        unsafe { sqlite3_result_error(pContext, zErr as *const i8, -(1 as i32)) };
        unsafe { sqlite3_free(zErr as *mut ()) };
        rc = 1 as i32;
    }
    return rc;
}

/// Implementation of the snippet() function for FTS3
///
/// # Arguments
///
/// * `pContext` - SQLite function call context
/// * `nVal` - Size of apVal[] array
/// * `apVal` - Array of arguments
#[unsafe(link_section = ".text.slate_distinct.fts3.fts3SnippetFunc")]
extern "C-unwind" fn fts3SnippetFunc(
    mut pContext: *mut sqlite3_context,
    mut nVal: i32,
    mut apVal: *mut *mut sqlite3_value,
) {
    let mut pTab: *mut Fts3Table = std::ptr::null_mut::<Fts3Table>();
    let mut pCsr: *mut Fts3Cursor = unsafe { std::mem::zeroed() }; // Cursor handle passed through apVal[0]
    let mut zStart: *const i8 = (b"<b>\0".as_ptr() as *mut i8) as *const i8;
    let mut zEnd: *const i8 = (b"</b>\0".as_ptr() as *mut i8) as *const i8;
    let mut zEllipsis: *const i8 = (b"<b>...</b>\0".as_ptr() as *mut i8) as *const i8;
    let mut iCol: i32 = -(1 as i32);
    let mut nToken: i32 = 15 as i32; // Default number of tokens in snippet
    // There must be at least one argument passed to this function (otherwise
    // the non-overloaded version would have been called instead of this one).
    0 as i32;
    if nVal > (6 as i32) {
        unsafe {
            sqlite3_result_error(
                pContext,
                (b"wrong number of arguments to function snippet()\0".as_ptr() as *mut i8)
                    as *const i8,
                -(1 as i32),
            )
        };
        return;
    }
    if fts3FunctionArg(
        pContext,
        (b"snippet\0".as_ptr() as *mut i8) as *const i8,
        unsafe { *unsafe { apVal.offset((0 as i32) as isize) } },
        std::ptr::addr_of_mut!(pCsr),
    ) != (0 as i32)
    {
        return;
    }
    pTab = (unsafe { (*pCsr).base.pVtab }) as *mut Fts3Table;
    // no break
    // no break
    // no break
    // no break
    match nVal {
        6 => {
            nToken = unsafe {
                sqlite3_value_int(unsafe { *unsafe { apVal.offset((5 as i32) as isize) } })
            };
            // no break
            {}
            iCol = unsafe {
                sqlite3_value_int(unsafe { *unsafe { apVal.offset((4 as i32) as isize) } })
            };
            // no break
            {}
            zEllipsis = (unsafe {
                sqlite3_value_text(unsafe { *unsafe { apVal.offset((3 as i32) as isize) } })
            }) as *const i8;
            // no break
            {}
            zEnd = (unsafe {
                sqlite3_value_text(unsafe { *unsafe { apVal.offset((2 as i32) as isize) } })
            }) as *const i8;
            // no break
            {}
            zStart = (unsafe {
                sqlite3_value_text(unsafe { *unsafe { apVal.offset((1 as i32) as isize) } })
            }) as *const i8;
        }
        5 => {
            iCol = unsafe {
                sqlite3_value_int(unsafe { *unsafe { apVal.offset((4 as i32) as isize) } })
            };
            // no break
            {}
            zEllipsis = (unsafe {
                sqlite3_value_text(unsafe { *unsafe { apVal.offset((3 as i32) as isize) } })
            }) as *const i8;
            // no break
            {}
            zEnd = (unsafe {
                sqlite3_value_text(unsafe { *unsafe { apVal.offset((2 as i32) as isize) } })
            }) as *const i8;
            // no break
            {}
            zStart = (unsafe {
                sqlite3_value_text(unsafe { *unsafe { apVal.offset((1 as i32) as isize) } })
            }) as *const i8;
        }
        4 => {
            zEllipsis = (unsafe {
                sqlite3_value_text(unsafe { *unsafe { apVal.offset((3 as i32) as isize) } })
            }) as *const i8;
            // no break
            {}
            zEnd = (unsafe {
                sqlite3_value_text(unsafe { *unsafe { apVal.offset((2 as i32) as isize) } })
            }) as *const i8;
            // no break
            {}
            zStart = (unsafe {
                sqlite3_value_text(unsafe { *unsafe { apVal.offset((1 as i32) as isize) } })
            }) as *const i8;
        }
        3 => {
            zEnd = (unsafe {
                sqlite3_value_text(unsafe { *unsafe { apVal.offset((2 as i32) as isize) } })
            }) as *const i8;
            // no break
            {}
            zStart = (unsafe {
                sqlite3_value_text(unsafe { *unsafe { apVal.offset((1 as i32) as isize) } })
            }) as *const i8;
        }
        2 => {
            zStart = (unsafe {
                sqlite3_value_text(unsafe { *unsafe { apVal.offset((1 as i32) as isize) } })
            }) as *const i8;
        }
        _ => {}
    }
    if !(zEllipsis != std::ptr::null::<i8>())
        || !(zEnd != std::ptr::null::<i8>())
        || !(zStart != std::ptr::null::<i8>())
    {
        unsafe { sqlite3_result_error_nomem(pContext) };
    } else {
        if nToken == (0 as i32) || iCol >= unsafe { (*pTab).nColumn } {
            unsafe {
                sqlite3_result_text(
                    pContext,
                    (b"\0".as_ptr() as *mut i8) as *const i8,
                    -(1 as i32),
                    None,
                )
            };
        } else {
            if (0 as i32) == fts3CursorSeek(pContext, pCsr) {
                unsafe {
                    sqlite3Fts3Snippet(pContext, pCsr, zStart, zEnd, zEllipsis, iCol, nToken)
                };
            }
        }
    }
}

/// Implementation of the offsets() function for FTS3
///
/// # Arguments
///
/// * `pContext` - SQLite function call context
/// * `nVal` - Size of argument array
/// * `apVal` - Array of arguments
#[unsafe(link_section = ".text.slate_distinct.fts3.fts3OffsetsFunc")]
extern "C-unwind" fn fts3OffsetsFunc(
    mut pContext: *mut sqlite3_context,
    mut nVal: i32,
    mut apVal: *mut *mut sqlite3_value,
) {
    let mut pCsr: *mut Fts3Cursor = unsafe { std::mem::zeroed() }; // Cursor handle passed through apVal[0]
    nVal;
    0 as i32;
    if fts3FunctionArg(
        pContext,
        (b"offsets\0".as_ptr() as *mut i8) as *const i8,
        unsafe { *unsafe { apVal.offset((0 as i32) as isize) } },
        std::ptr::addr_of_mut!(pCsr),
    ) != (0 as i32)
    {
        return;
    }
    0 as i32;
    if (0 as i32) == fts3CursorSeek(pContext, pCsr) {
        unsafe { sqlite3Fts3Offsets(pContext, pCsr) };
    }
}

/// Implementation of the special optimize() function for FTS3. This
/// function merges all segments in the database to a single segment.
/// Example usage is:
///
///   SELECT optimize(t) FROM t LIMIT 1;
///
/// where 't' is the name of an FTS3 table.
///
/// # Arguments
///
/// * `pContext` - SQLite function call context
/// * `nVal` - Size of argument array
/// * `apVal` - Array of arguments
#[unsafe(link_section = ".text.slate_distinct.fts3.fts3OptimizeFunc")]
extern "C-unwind" fn fts3OptimizeFunc(
    mut pContext: *mut sqlite3_context,
    mut nVal: i32,
    mut apVal: *mut *mut sqlite3_value,
) {
    let mut rc: i32 = 0 as i32; // Return code
    let mut p: *mut Fts3Table = unsafe { std::mem::zeroed() }; // Virtual table handle
    let mut pCursor: *mut Fts3Cursor = unsafe { std::mem::zeroed() }; // Cursor handle passed through apVal[0]
    nVal;
    0 as i32;
    if fts3FunctionArg(
        pContext,
        (b"optimize\0".as_ptr() as *mut i8) as *const i8,
        unsafe { *unsafe { apVal.offset((0 as i32) as isize) } },
        std::ptr::addr_of_mut!(pCursor),
    ) != (0 as i32)
    {
        return;
    }
    p = (unsafe { (*pCursor).base.pVtab }) as *mut Fts3Table;
    0 as i32;
    rc = unsafe { sqlite3Fts3Optimize(p) };
    '__slate_break_1754: {
        match rc {
            0 => {
                unsafe {
                    sqlite3_result_text(
                        pContext,
                        (b"Index optimized\0".as_ptr() as *mut i8) as *const i8,
                        -(1 as i32),
                        None,
                    )
                };
            }
            101 => {
                unsafe {
                    sqlite3_result_text(
                        pContext,
                        (b"Index already optimal\0".as_ptr() as *mut i8) as *const i8,
                        -(1 as i32),
                        None,
                    )
                };
            }
            _ => {
                unsafe { sqlite3_result_error_code(pContext, rc) };
            }
        }
    }
}

/// Implementation of the matchinfo() function for FTS3
///
/// # Arguments
///
/// * `pContext` - SQLite function call context
/// * `nVal` - Size of argument array
/// * `apVal` - Array of arguments
#[unsafe(link_section = ".text.slate_distinct.fts3.fts3MatchinfoFunc")]
extern "C-unwind" fn fts3MatchinfoFunc(
    mut pContext: *mut sqlite3_context,
    mut nVal: i32,
    mut apVal: *mut *mut sqlite3_value,
) {
    let mut pCsr: *mut Fts3Cursor = unsafe { std::mem::zeroed() }; // Cursor handle passed through apVal[0]
    0 as i32;
    if (0 as i32)
        == fts3FunctionArg(
            pContext,
            (b"matchinfo\0".as_ptr() as *mut i8) as *const i8,
            unsafe { *unsafe { apVal.offset((0 as i32) as isize) } },
            std::ptr::addr_of_mut!(pCsr),
        )
    {
        let mut zArg: *const i8 = std::ptr::null::<i8>();
        if nVal > (1 as i32) {
            zArg = (unsafe {
                sqlite3_value_text(unsafe { *unsafe { apVal.offset((1 as i32) as isize) } })
            }) as *const i8;
        }
        unsafe { sqlite3Fts3Matchinfo(pContext, pCsr, zArg) };
    }
}

/// This routine implements the xFindFunction method for the FTS3
/// virtual table.
///
/// # Arguments
///
/// * `pVtab` - Virtual table handle
/// * `nArg` - Number of SQL function arguments
/// * `zName` - Name of SQL function
/// * `pxFunc` - OUT: Result
/// * `ppArg` - Unused
#[unsafe(link_section = ".text.slate_distinct.fts3.fts3FindFunctionMethod")]
extern "C-unwind" fn fts3FindFunctionMethod(
    mut pVtab: *mut sqlite3_vtab,
    mut nArg: i32,
    mut zName: *const i8,
    mut pxFunc: *mut Option<
        unsafe extern "C-unwind" fn(*mut sqlite3_context, i32, *mut *mut sqlite3_value),
    >,
    mut ppArg: *mut *mut (),
) -> i32 {
    let mut aOverload: __SlateAlign16<[Overloaded; 4]> = __SlateAlign16([
        Overloaded {
            zName: (b"snippet\0".as_ptr() as *mut i8) as *const i8,
            xFunc: Some(fts3SnippetFunc),
        },
        Overloaded {
            zName: (b"offsets\0".as_ptr() as *mut i8) as *const i8,
            xFunc: Some(fts3OffsetsFunc),
        },
        Overloaded {
            zName: (b"optimize\0".as_ptr() as *mut i8) as *const i8,
            xFunc: Some(fts3OptimizeFunc),
        },
        Overloaded {
            zName: (b"matchinfo\0".as_ptr() as *mut i8) as *const i8,
            xFunc: Some(fts3MatchinfoFunc),
        },
    ]);
    let mut i: i32 = 0 as i32; // Iterator variable
    pVtab;
    nArg;
    ppArg;
    i = 0 as i32;
    '__slate_break_1762: loop {
        if !(i < ((((64 as u64) / (16 as u64)) as u32) as i32)) {
            break;
        }
        if (unsafe {
            strcmp(zName, unsafe {
                (*unsafe { (aOverload.0.as_mut_ptr() as *mut Overloaded).offset(i as isize) }).zName
            })
        }) == (0 as i32)
        {
            unsafe {
                *pxFunc = unsafe {
                    (*unsafe { (aOverload.0.as_mut_ptr() as *mut Overloaded).offset(i as isize) })
                        .xFunc
                };
            }
            return 1 as i32;
        }
        let __v2264: i32 = i;
        let __v2265: i32 = __v2264 + (1 as i32);
        i = __v2265;
    }
    // No function of the specified name was found. Return 0.
    return 0 as i32;
}

/// Implementation of FTS3 xRename method. Rename an fts3 table.
///
/// # Arguments
///
/// * `pVtab` - Virtual table handle
/// * `zName` - New name of table
#[unsafe(link_section = ".text.slate_distinct.fts3.fts3RenameMethod")]
extern "C-unwind" fn fts3RenameMethod(mut pVtab: *mut sqlite3_vtab, mut zName: *const i8) -> i32 {
    let mut p: *mut Fts3Table = pVtab as *mut Fts3Table;
    let mut db: *mut sqlite3 = unsafe { (*p).db }; // Database connection
    let mut rc: i32 = 0 as i32; // Return Code
    // At this point it must be known if the %_stat table exists or not.
    // So bHasStat may not be 2.
    rc = fts3SetHasStat(p);
    // As it happens, the pending terms table is always empty here. This is
    // because an "ALTER TABLE RENAME TABLE" statement inside a transaction
    // always opens a savepoint transaction. And the xSavepoint() method
    // flushes the pending terms table. But leave the (no-op) call to
    // PendingTermsFlush() in in case that changes.
    0 as i32;
    if rc == (0 as i32) {
        rc = unsafe { sqlite3Fts3PendingTermsFlush(p) };
    }
    unsafe {
        (*p).bIgnoreSavepoint = ((1 as i32) as i8) as u8;
    }
    if (unsafe { (*p).zContentTbl }) == std::ptr::null_mut::<i8>() {
        unsafe {
            fts3DbExec(
                std::ptr::addr_of_mut!(rc),
                db,
                (b"ALTER TABLE %Q.'%q_content'  RENAME TO '%q_content';\0".as_ptr() as *mut i8)
                    as *const i8,
                unsafe { (*p).zDb },
                unsafe { (*p).zName },
                zName,
            )
        };
    }
    if (unsafe { (*p).bHasDocsize }) != (0 as u8) {
        unsafe {
            fts3DbExec(
                std::ptr::addr_of_mut!(rc),
                db,
                (b"ALTER TABLE %Q.'%q_docsize'  RENAME TO '%q_docsize';\0".as_ptr() as *mut i8)
                    as *const i8,
                unsafe { (*p).zDb },
                unsafe { (*p).zName },
                zName,
            )
        };
    }
    if (unsafe { (*p).bHasStat }) != (0 as u8) {
        unsafe {
            fts3DbExec(
                std::ptr::addr_of_mut!(rc),
                db,
                (b"ALTER TABLE %Q.'%q_stat'  RENAME TO '%q_stat';\0".as_ptr() as *mut i8)
                    as *const i8,
                unsafe { (*p).zDb },
                unsafe { (*p).zName },
                zName,
            )
        };
    }
    unsafe {
        fts3DbExec(
            std::ptr::addr_of_mut!(rc),
            db,
            (b"ALTER TABLE %Q.'%q_segments' RENAME TO '%q_segments';\0".as_ptr() as *mut i8)
                as *const i8,
            unsafe { (*p).zDb },
            unsafe { (*p).zName },
            zName,
        )
    };
    unsafe {
        fts3DbExec(
            std::ptr::addr_of_mut!(rc),
            db,
            (b"ALTER TABLE %Q.'%q_segdir'   RENAME TO '%q_segdir';\0".as_ptr() as *mut i8)
                as *const i8,
            unsafe { (*p).zDb },
            unsafe { (*p).zName },
            zName,
        )
    };
    unsafe {
        (*p).bIgnoreSavepoint = ((0 as i32) as i8) as u8;
    }
    return rc;
}

/// The xSavepoint() method.
///
/// Flush the contents of the pending-terms table to disk.
#[unsafe(link_section = ".text.slate_distinct.fts3.fts3SavepointMethod")]
extern "C-unwind" fn fts3SavepointMethod(mut pVtab: *mut sqlite3_vtab, mut iSavepoint: i32) -> i32 {
    let mut rc: i32 = 0 as i32;
    let mut pTab: *mut Fts3Table = pVtab as *mut Fts3Table;
    0 as i32;
    0 as i32;
    {}
    if (((unsafe { (*pTab).bIgnoreSavepoint }) as u32) as i32) == (0 as i32) {
        if (unsafe {
            (*unsafe {
                std::ptr::addr_of_mut!(
                    (*unsafe { unsafe { (*pTab).aIndex }.offset((0 as i32) as isize) }).hPending
                )
            })
            .count
        }) > (0 as i32)
        {
            let mut zSql: *mut i8 = unsafe {
                sqlite3_mprintf(
                    (b"INSERT INTO %Q.%Q(%Q) VALUES('flush')\0".as_ptr() as *mut i8) as *const i8,
                    unsafe { (*pTab).zDb },
                    unsafe { (*pTab).zName },
                    unsafe { (*pTab).zName },
                )
            };
            if zSql != std::ptr::null_mut::<i8>() {
                unsafe {
                    (*pTab).bIgnoreSavepoint = ((1 as i32) as i8) as u8;
                }
                rc = unsafe {
                    sqlite3_exec(
                        unsafe { (*pTab).db },
                        zSql as *const i8,
                        None,
                        std::ptr::null_mut::<()>(),
                        std::ptr::null_mut::<*mut i8>(),
                    )
                };
                unsafe {
                    (*pTab).bIgnoreSavepoint = ((0 as i32) as i8) as u8;
                }
                unsafe { sqlite3_free(zSql as *mut ()) };
            } else {
                rc = 7 as i32;
            }
        }
        if rc == (0 as i32) {
            unsafe {
                (*pTab).iSavepoint = iSavepoint + (1 as i32);
            }
        }
    }
    return rc;
}

/// The xRelease() method.
///
/// This is a no-op.
#[unsafe(link_section = ".text.slate_distinct.fts3.fts3ReleaseMethod")]
extern "C-unwind" fn fts3ReleaseMethod(mut pVtab: *mut sqlite3_vtab, mut iSavepoint: i32) -> i32 {
    let mut pTab: *mut Fts3Table = pVtab as *mut Fts3Table;
    0 as i32;
    0 as i32;
    {}
    unsafe {
        (*pTab).iSavepoint = iSavepoint;
    }
    return 0 as i32;
}

/// The xRollbackTo() method.
///
/// Discard the contents of the pending terms table.
#[unsafe(link_section = ".text.slate_distinct.fts3.fts3RollbackToMethod")]
extern "C-unwind" fn fts3RollbackToMethod(
    mut pVtab: *mut sqlite3_vtab,
    mut iSavepoint: i32,
) -> i32 {
    let mut pTab: *mut Fts3Table = pVtab as *mut Fts3Table;
    iSavepoint;
    0 as i32;
    {}
    if iSavepoint + (1 as i32) <= unsafe { (*pTab).iSavepoint } {
        unsafe { sqlite3Fts3PendingTermsClear(pTab) };
    }
    return 0 as i32;
}

/// Return true if zName is the extension on one of the shadow tables used
/// by this module.
#[unsafe(link_section = ".text.slate_distinct.fts3.fts3ShadowName")]
extern "C-unwind" fn fts3ShadowName(mut zName: *const i8) -> i32 {
    let mut i: u32 = 0 as u32;
    i = (0 as i32) as u32;
    '__slate_break_1774: while (i as u64) < (40 as u64) / (8 as u64) {
        if (unsafe {
            sqlite3_stricmp(zName, unsafe {
                *unsafe {
                    unsafe { std::ptr::addr_of_mut!(azName.0) as *mut *const i8 }.offset(i as isize)
                }
            })
        }) == (0 as i32)
        {
            return 1 as i32;
        }
        let __v2266: u32 = i;
        let __v2267: u32 = __v2266.wrapping_add((1 as i32) as u32);
        i = __v2267;
    }
    return 0 as i32;
}

static mut azName: __SlateAlign16<[*const i8; 5]> = __SlateAlign16([
    (b"content\0".as_ptr() as *mut i8) as *const i8,
    (b"docsize\0".as_ptr() as *mut i8) as *const i8,
    (b"segdir\0".as_ptr() as *mut i8) as *const i8,
    (b"segments\0".as_ptr() as *mut i8) as *const i8,
    (b"stat\0".as_ptr() as *mut i8) as *const i8,
]);

/// Implementation of the xIntegrity() method on the FTS3/FTS4 virtual
/// table.
///
/// # Arguments
///
/// * `pVtab` - The virtual table to be checked
/// * `zSchema` - Name of schema in which pVtab lives
/// * `zTabname` - Name of the pVTab table
/// * `isQuick` - True if this is a quick_check
/// * `pzErr` - Write error message here
#[unsafe(link_section = ".text.slate_distinct.fts3.fts3IntegrityMethod")]
extern "C-unwind" fn fts3IntegrityMethod(
    mut pVtab: *mut sqlite3_vtab,
    mut zSchema: *const i8,
    mut zTabname: *const i8,
    mut isQuick: i32,
    mut pzErr: *mut *mut i8,
) -> i32 {
    let mut p: *mut Fts3Table = pVtab as *mut Fts3Table;
    let mut rc: i32 = 0 as i32;
    let mut bOk: i32 = 0 as i32;
    isQuick;
    rc = unsafe { sqlite3Fts3IntegrityCheck(p, std::ptr::addr_of_mut!(bOk)) };
    0 as i32;
    0 as i32;
    if rc == (1 as i32) || rc & (255 as i32) == (11 as i32) {
        unsafe {
            *pzErr = unsafe {
                sqlite3_mprintf(
                    (b"unable to validate the inverted index for FTS%d table %s.%s: %s\0".as_ptr()
                        as *mut i8) as *const i8,
                    if (unsafe { (*p).bFts4 }) != (0 as u8) {
                        4 as i32
                    } else {
                        3 as i32
                    },
                    zSchema,
                    zTabname,
                    unsafe { sqlite3_errstr(rc) },
                )
            };
        }
        if (unsafe { *pzErr }) != std::ptr::null_mut::<i8>() {
            rc = 0 as i32;
        }
    } else {
        if rc == (0 as i32) && bOk == (0 as i32) {
            unsafe {
                *pzErr = unsafe {
                    sqlite3_mprintf(
                        (b"malformed inverted index for FTS%d table %s.%s\0".as_ptr() as *mut i8)
                            as *const i8,
                        if (unsafe { (*p).bFts4 }) != (0 as u8) {
                            4 as i32
                        } else {
                            3 as i32
                        },
                        zSchema,
                        zTabname,
                    )
                };
            }
            if (unsafe { *pzErr }) == std::ptr::null_mut::<i8>() {
                rc = 7 as i32;
            }
        }
    }
    unsafe { sqlite3Fts3SegmentsClose(p) };
    return rc;
}

/// iVersion
/// xCreate
/// xConnect
/// xBestIndex
/// xDisconnect
/// xDestroy
/// xOpen
/// xClose
/// xFilter
/// xNext
/// xEof
/// xColumn
/// xRowid
/// xUpdate
/// xBegin
/// xSync
/// xCommit
/// xRollback
/// xFindFunction
/// xRename
/// xSavepoint
/// xRelease
/// xRollbackTo
/// xShadowName
/// xIntegrity
static mut fts3Module: sqlite3_module = sqlite3_module {
    iVersion: 4 as i32,
    xCreate: Some(fts3CreateMethod),
    xConnect: Some(fts3ConnectMethod),
    xBestIndex: Some(fts3BestIndexMethod),
    xDisconnect: Some(fts3DisconnectMethod),
    xDestroy: Some(fts3DestroyMethod),
    xOpen: Some(fts3OpenMethod),
    xClose: Some(fts3CloseMethod),
    xFilter: Some(fts3FilterMethod),
    xNext: Some(fts3NextMethod),
    xEof: Some(fts3EofMethod),
    xColumn: Some(fts3ColumnMethod),
    xRowid: Some(fts3RowidMethod),
    xUpdate: Some(fts3UpdateMethod),
    xBegin: Some(fts3BeginMethod),
    xSync: Some(fts3SyncMethod),
    xCommit: Some(fts3CommitMethod),
    xRollback: Some(fts3RollbackMethod),
    xFindFunction: Some(fts3FindFunctionMethod),
    xRename: Some(fts3RenameMethod),
    xSavepoint: Some(fts3SavepointMethod),
    xRelease: Some(fts3ReleaseMethod),
    xRollbackTo: Some(fts3RollbackToMethod),
    xShadowName: Some(fts3ShadowName),
    xIntegrity: Some(fts3IntegrityMethod),
};

/// This function is registered as the module destructor (called when an
/// FTS3 enabled database connection is closed). It frees the memory
/// allocated for the tokenizer hash table.
#[unsafe(link_section = ".text.slate_distinct.fts3.hashDestroy")]
extern "C-unwind" fn hashDestroy(mut p: *mut ()) {
    let mut pHash: *mut Fts3HashWrapper = p as *mut Fts3HashWrapper;
    let __v2268: *mut Fts3HashWrapper = pHash;
    let __v2269: i32 = unsafe { (*__v2268).nRef };
    let __v2270: i32 = __v2269 - (1 as i32);
    unsafe {
        (*__v2268).nRef = __v2270;
    }
    if (unsafe { (*pHash).nRef }) <= (0 as i32) {
        unsafe { sqlite3Fts3HashClear(unsafe { std::ptr::addr_of_mut!((*pHash).hash) }) };
        unsafe { sqlite3_free(pHash as *mut ()) };
    }
}

// The fts3 built-in tokenizers - "simple", "porter" and "icu"- are
// implemented in files fts3_tokenizer1.c, fts3_porter.c and fts3_icu.c
// respectively. The following three forward declarations are for functions
// declared in these files used to retrieve the respective implementations.
//
// Calling sqlite3Fts3SimpleTokenizerModule() sets the value pointed
// to by the argument to point to the "simple" tokenizer implementation.
// And so on.
/// Initialize the fts3 extension. If this extension is built as part
/// of the sqlite library, then this function is called directly by
/// SQLite. If fts3 is built as a dynamically loadable extension, this
/// function is called by the sqlite3_extension_init() entry point.
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.fts3.sqlite3Fts3Init")]
extern "C-unwind" fn sqlite3Fts3Init(mut db: *mut sqlite3) -> i32 {
    let mut rc: i32 = 0 as i32;
    let mut pHash: *mut Fts3HashWrapper = std::ptr::null_mut::<Fts3HashWrapper>();
    let mut pSimple: *const sqlite3_tokenizer_module = std::ptr::null::<sqlite3_tokenizer_module>();
    let mut pPorter: *const sqlite3_tokenizer_module = std::ptr::null::<sqlite3_tokenizer_module>();
    let mut pUnicode: *const sqlite3_tokenizer_module =
        std::ptr::null::<sqlite3_tokenizer_module>();
    unsafe { sqlite3Fts3UnicodeTokenizer(std::ptr::addr_of_mut!(pUnicode)) };
    rc = unsafe { sqlite3Fts3InitAux(db) };
    if rc != (0 as i32) {
        return rc;
    }
    unsafe { sqlite3Fts3SimpleTokenizerModule(std::ptr::addr_of_mut!(pSimple)) };
    unsafe { sqlite3Fts3PorterTokenizerModule(std::ptr::addr_of_mut!(pPorter)) };
    // Allocate and initialize the hash-table used to store tokenizers.
    pHash = (unsafe { sqlite3_malloc(((40 as u64) as u32) as i32) }) as *mut Fts3HashWrapper;
    if !(pHash != std::ptr::null_mut::<Fts3HashWrapper>()) {
        rc = 7 as i32;
    } else {
        unsafe {
            sqlite3Fts3HashInit(
                unsafe { std::ptr::addr_of_mut!((*pHash).hash) },
                (1 as i32) as i8,
                (1 as i32) as i8,
            )
        };
        unsafe {
            (*pHash).nRef = 0 as i32;
        }
    }
    // Load the built-in tokenizers into the hash table
    if rc == (0 as i32) {
        let __v1934: bool;
        if (unsafe {
            sqlite3Fts3HashInsert(
                unsafe { std::ptr::addr_of_mut!((*pHash).hash) },
                (b"simple\0".as_ptr() as *mut i8) as *const (),
                7 as i32,
                pSimple as *mut (),
            )
        }) != std::ptr::null_mut::<()>()
        {
            __v1934 = true as bool;
        } else {
            __v1934 = (unsafe {
                sqlite3Fts3HashInsert(
                    unsafe { std::ptr::addr_of_mut!((*pHash).hash) },
                    (b"porter\0".as_ptr() as *mut i8) as *const (),
                    7 as i32,
                    pPorter as *mut (),
                )
            }) != std::ptr::null_mut::<()>();
        }
        let __v1935: bool;
        if __v1934 {
            __v1935 = true as bool;
        } else {
            __v1935 = (unsafe {
                sqlite3Fts3HashInsert(
                    unsafe { std::ptr::addr_of_mut!((*pHash).hash) },
                    (b"unicode61\0".as_ptr() as *mut i8) as *const (),
                    10 as i32,
                    pUnicode as *mut (),
                )
            }) != std::ptr::null_mut::<()>();
        }
        if __v1935 {
            rc = 7 as i32;
        }
    }
    // Create the virtual table wrapper around the hash-table and overload
    // the four scalar functions. If this is successful, register the
    // module with sqlite.
    let __v1936: bool;
    if (0 as i32) == rc {
        let __v1937: i32 = unsafe {
            sqlite3Fts3InitHashTable(
                db,
                unsafe { std::ptr::addr_of_mut!((*pHash).hash) },
                (b"fts3_tokenizer\0".as_ptr() as *mut i8) as *const i8,
            )
        };
        rc = __v1937;
        __v1936 = (0 as i32) == __v1937;
    } else {
        __v1936 = false as bool;
    }
    let __v1938: bool;
    if __v1936 {
        let __v1939: i32 = unsafe {
            sqlite3_overload_function(
                db,
                (b"snippet\0".as_ptr() as *mut i8) as *const i8,
                -(1 as i32),
            )
        };
        rc = __v1939;
        __v1938 = (0 as i32) == __v1939;
    } else {
        __v1938 = false as bool;
    }
    let __v1940: bool;
    if __v1938 {
        let __v1941: i32 = unsafe {
            sqlite3_overload_function(
                db,
                (b"offsets\0".as_ptr() as *mut i8) as *const i8,
                1 as i32,
            )
        };
        rc = __v1941;
        __v1940 = (0 as i32) == __v1941;
    } else {
        __v1940 = false as bool;
    }
    let __v1942: bool;
    if __v1940 {
        let __v1943: i32 = unsafe {
            sqlite3_overload_function(
                db,
                (b"matchinfo\0".as_ptr() as *mut i8) as *const i8,
                1 as i32,
            )
        };
        rc = __v1943;
        __v1942 = (0 as i32) == __v1943;
    } else {
        __v1942 = false as bool;
    }
    let __v1944: bool;
    if __v1942 {
        let __v1945: i32 = unsafe {
            sqlite3_overload_function(
                db,
                (b"matchinfo\0".as_ptr() as *mut i8) as *const i8,
                2 as i32,
            )
        };
        rc = __v1945;
        __v1944 = (0 as i32) == __v1945;
    } else {
        __v1944 = false as bool;
    }
    let __v1946: bool;
    if __v1944 {
        let __v1947: i32 = unsafe {
            sqlite3_overload_function(
                db,
                (b"optimize\0".as_ptr() as *mut i8) as *const i8,
                1 as i32,
            )
        };
        rc = __v1947;
        __v1946 = (0 as i32) == __v1947;
    } else {
        __v1946 = false as bool;
    }
    if __v1946 {
        let __v1948: *mut Fts3HashWrapper = pHash;
        let __v1949: i32 = unsafe { (*__v1948).nRef };
        let __v1950: i32 = __v1949 + (1 as i32);
        unsafe {
            (*__v1948).nRef = __v1950;
        }
        rc = unsafe {
            sqlite3_create_module_v2(
                db,
                (b"fts3\0".as_ptr() as *mut i8) as *const i8,
                unsafe { std::ptr::addr_of!(fts3Module) },
                pHash as *mut (),
                Some(hashDestroy),
            )
        };
        if rc == (0 as i32) {
            let __v1951: *mut Fts3HashWrapper = pHash;
            let __v1952: i32 = unsafe { (*__v1951).nRef };
            let __v1953: i32 = __v1952 + (1 as i32);
            unsafe {
                (*__v1951).nRef = __v1953;
            }
            rc = unsafe {
                sqlite3_create_module_v2(
                    db,
                    (b"fts4\0".as_ptr() as *mut i8) as *const i8,
                    unsafe { std::ptr::addr_of!(fts3Module) },
                    pHash as *mut (),
                    Some(hashDestroy),
                )
            };
        }
        if rc == (0 as i32) {
            let __v1954: *mut Fts3HashWrapper = pHash;
            let __v1955: i32 = unsafe { (*__v1954).nRef };
            let __v1956: i32 = __v1955 + (1 as i32);
            unsafe {
                (*__v1954).nRef = __v1956;
            }
            rc = unsafe {
                sqlite3Fts3InitTok(db, (pHash as *mut ()) as *mut Fts3Hash, Some(hashDestroy))
            };
        }
        return rc;
    }
    // An error has occurred. Delete the hash table and return the error code.
    0 as i32;
    if pHash != std::ptr::null_mut::<Fts3HashWrapper>() {
        unsafe { sqlite3Fts3HashClear(unsafe { std::ptr::addr_of_mut!((*pHash).hash) }) };
        unsafe { sqlite3_free(pHash as *mut ()) };
    }
    return rc;
}

/// Allocate an Fts3MultiSegReader for each token in the expression headed
/// by pExpr.
///
/// An Fts3SegReader object is a cursor that can seek or scan a range of
/// entries within a single segment b-tree. An Fts3MultiSegReader uses multiple
/// Fts3SegReader objects internally to provide an interface to seek or scan
/// within the union of all segments of a b-tree. Hence the name.
///
/// If the allocated Fts3MultiSegReader just seeks to a single entry in a
/// segment b-tree (if the term is not a prefix or it is a prefix for which
/// there exists prefix b-tree of the right length) then it may be traversed
/// and merged incrementally. Otherwise, it has to be merged into an in-memory
/// doclist and then traversed.
///
/// # Arguments
///
/// * `pCsr` - FTS cursor handle
/// * `pExpr` - Allocate readers for this expression
/// * `pnToken` - OUT: Total number of tokens in phrase.
/// * `pnOr` - OUT: Total number of OR nodes in expr.
/// * `pRc` - IN/OUT: Error code
fn fts3EvalAllocateReaders(
    mut pCsr: *mut Fts3Cursor,
    mut pExpr: *mut Fts3Expr,
    mut pnToken: *mut i32,
    mut pnOr: *mut i32,
    mut pRc: *mut i32,
) {
    if pExpr != std::ptr::null_mut::<Fts3Expr>() && (0 as i32) == unsafe { *pRc } {
        if (unsafe { (*pExpr).eType }) == (5 as i32) {
            let mut i: i32 = 0 as i32;
            let mut nToken: i32 = unsafe { (*unsafe { (*pExpr).pPhrase }).nToken };
            let __v2271: *mut i32 = pnToken;
            let __v2272: i32 = unsafe { *__v2271 };
            let __v2273: i32 = __v2272 + nToken;
            unsafe {
                *__v2271 = __v2273;
            }
            i = 0 as i32;
            '__slate_break_1791: loop {
                if !(i < nToken) {
                    break;
                }
                let mut pToken: *mut Fts3PhraseToken = unsafe {
                    unsafe {
                        std::ptr::addr_of_mut!((*unsafe { (*pExpr).pPhrase }).aToken)
                            as *mut Fts3PhraseToken
                    }
                    .offset(i as isize)
                };
                let mut rc: i32 = fts3TermSegReaderCursor(
                    pCsr,
                    (unsafe { (*pToken).z }) as *const i8,
                    unsafe { (*pToken).n },
                    unsafe { (*pToken).isPrefix },
                    unsafe { std::ptr::addr_of_mut!((*pToken).pSegcsr) },
                );
                if rc != (0 as i32) {
                    unsafe {
                        *pRc = rc;
                    }
                    return;
                }
                let __v2274: i32 = i;
                let __v2275: i32 = __v2274 + (1 as i32);
                i = __v2275;
            }
            0 as i32;
            unsafe {
                (*unsafe { (*pExpr).pPhrase }).iDoclistToken = -(1 as i32);
            }
        } else {
            let __v2276: *mut i32 = pnOr;
            let __v2277: i32 = unsafe { *__v2276 };
            let __v2278: i32 = __v2277 + (((unsafe { (*pExpr).eType }) == (4 as i32)) as i32);
            unsafe {
                *__v2276 = __v2278;
            }
            fts3EvalAllocateReaders(pCsr, unsafe { (*pExpr).pLeft }, pnToken, pnOr, pRc);
            fts3EvalAllocateReaders(pCsr, unsafe { (*pExpr).pRight }, pnToken, pnOr, pRc);
        }
    }
}

/// Arguments pList/nList contain the doclist for token iToken of phrase p.
/// It is merged into the main doclist stored in p->doclist.aAll/nAll.
///
/// This function assumes that pList points to a buffer allocated using
/// sqlite3_malloc(). This function takes responsibility for eventually
/// freeing the buffer.
///
/// SQLITE_OK is returned if successful, or SQLITE_NOMEM if an error occurs.
///
/// # Arguments
///
/// * `pTab` - FTS Table pointer
/// * `p` - Phrase to merge pList/nList into
/// * `iToken` - Token pList/nList corresponds to
/// * `pList` - Pointer to doclist
/// * `nList` - Number of bytes in pList
fn fts3EvalPhraseMergeToken(
    mut pTab: *mut Fts3Table,
    mut p: *mut Fts3Phrase,
    mut iToken: i32,
    mut pList: *mut i8,
    mut nList: i32,
) -> i32 {
    let mut rc: i32 = 0 as i32;
    0 as i32;
    if pList == std::ptr::null_mut::<i8>() {
        unsafe { sqlite3_free((unsafe { (*p).doclist.aAll }) as *mut ()) };
        unsafe {
            (*p).doclist.aAll = std::ptr::null_mut::<i8>();
        }
        unsafe {
            (*p).doclist.nAll = 0 as i32;
        }
    } else {
        if (unsafe { (*p).iDoclistToken }) < (0 as i32) {
            unsafe {
                (*p).doclist.aAll = pList;
            }
            unsafe {
                (*p).doclist.nAll = nList;
            }
        } else {
            if (unsafe { (*p).doclist.aAll }) == std::ptr::null_mut::<i8>() {
                unsafe { sqlite3_free(pList as *mut ()) };
            } else {
                let mut pLeft: *mut i8 = unsafe { std::mem::zeroed() };
                let mut pRight: *mut i8 = unsafe { std::mem::zeroed() };
                let mut nLeft: i32 = 0 as i32;
                let mut nRight: i32 = 0 as i32;
                let mut nDiff: i32 = 0 as i32;
                if (unsafe { (*p).iDoclistToken }) < iToken {
                    pLeft = unsafe { (*p).doclist.aAll };
                    nLeft = unsafe { (*p).doclist.nAll };
                    pRight = pList;
                    nRight = nList;
                    nDiff = iToken - unsafe { (*p).iDoclistToken };
                } else {
                    pRight = unsafe { (*p).doclist.aAll };
                    nRight = unsafe { (*p).doclist.nAll };
                    pLeft = pList;
                    nLeft = nList;
                    nDiff = (unsafe { (*p).iDoclistToken }) - iToken;
                }
                rc = fts3DoclistPhraseMerge(
                    ((unsafe { (*pTab).bDescIdx }) as u32) as i32,
                    nDiff,
                    pLeft,
                    nLeft,
                    std::ptr::addr_of_mut!(pRight),
                    std::ptr::addr_of_mut!(nRight),
                );
                unsafe { sqlite3_free(pLeft as *mut ()) };
                unsafe {
                    (*p).doclist.aAll = pRight;
                }
                unsafe {
                    (*p).doclist.nAll = nRight;
                }
            }
        }
    }
    if iToken > unsafe { (*p).iDoclistToken } {
        unsafe {
            (*p).iDoclistToken = iToken;
        }
    }
    return rc;
}

/// Load the doclist for phrase p into p->doclist.aAll/nAll. The loaded doclist
/// does not take deferred tokens into account.
///
/// SQLITE_OK is returned if no error occurs, otherwise an SQLite error code.
///
/// # Arguments
///
/// * `pCsr` - FTS Cursor handle
/// * `p` - Phrase object
fn fts3EvalPhraseLoad(mut pCsr: *mut Fts3Cursor, mut p: *mut Fts3Phrase) -> i32 {
    let mut pTab: *mut Fts3Table = (unsafe { (*pCsr).base.pVtab }) as *mut Fts3Table;
    let mut iToken: i32 = 0 as i32;
    let mut rc: i32 = 0 as i32;
    iToken = 0 as i32;
    '__slate_break_1792: loop {
        if !(rc == (0 as i32) && iToken < unsafe { (*p).nToken }) {
            break;
        }
        let mut pToken: *mut Fts3PhraseToken = unsafe {
            unsafe { std::ptr::addr_of_mut!((*p).aToken) as *mut Fts3PhraseToken }
                .offset(iToken as isize)
        };
        0 as i32;
        if (unsafe { (*pToken).pSegcsr }) != std::ptr::null_mut::<Fts3MultiSegReader>() {
            let mut nThis: i32 = 0 as i32;
            let mut pThis: *mut i8 = std::ptr::null_mut::<i8>();
            rc = fts3TermSelect(
                pTab,
                pToken,
                unsafe { (*p).iColumn },
                std::ptr::addr_of_mut!(nThis),
                std::ptr::addr_of_mut!(pThis),
            );
            if rc == (0 as i32) {
                rc = fts3EvalPhraseMergeToken(pTab, p, iToken, pThis, nThis);
            }
        }
        0 as i32;
        let __v2279: i32 = iToken;
        let __v2280: i32 = __v2279 + (1 as i32);
        iToken = __v2280;
    }
    return rc;
}

/// This function is called on each phrase after the position lists for
/// any deferred tokens have been loaded into memory. It updates the phrases
/// current position list to include only those positions that are really
/// instances of the phrase (after considering deferred tokens). If this
/// means that the phrase does not appear in the current row, doclist.pList
/// and doclist.nList are both zeroed.
///
/// SQLITE_OK is returned if no error occurs, otherwise an SQLite error code.
fn fts3EvalDeferredPhrase(mut pCsr: *mut Fts3Cursor, mut pPhrase: *mut Fts3Phrase) -> i32 {
    let mut iToken: i32 = 0 as i32; // Used to iterate through phrase tokens
    let mut aPoslist: *mut i8 = std::ptr::null_mut::<i8>(); // Position list for deferred tokens
    let mut nPoslist: i32 = 0 as i32; // Number of bytes in aPoslist
    let mut iPrev: i32 = -(1 as i32); // Token number of previous deferred token
    let mut aFree: *mut i8 = if (unsafe { (*pPhrase).doclist.bFreeList }) != (0 as i32) {
        unsafe { (*pPhrase).doclist.pList }
    } else {
        std::ptr::null_mut::<i8>()
    };
    iToken = 0 as i32;
    '__slate_break_1793: loop {
        if !(iToken < unsafe { (*pPhrase).nToken }) {
            break;
        }
        let mut pToken: *mut Fts3PhraseToken = unsafe {
            unsafe { std::ptr::addr_of_mut!((*pPhrase).aToken) as *mut Fts3PhraseToken }
                .offset(iToken as isize)
        };
        let mut pDeferred: *mut Fts3DeferredToken = unsafe { (*pToken).pDeferred };
        if pDeferred != std::ptr::null_mut::<Fts3DeferredToken>() {
            let mut pList: *mut i8 = unsafe { std::mem::zeroed() };
            let mut nList: i32 = 0 as i32;
            let mut rc: i32 = unsafe {
                sqlite3Fts3DeferredTokenList(
                    pDeferred,
                    std::ptr::addr_of_mut!(pList),
                    std::ptr::addr_of_mut!(nList),
                )
            };
            if rc != (0 as i32) {
                return rc;
            }
            if pList == std::ptr::null_mut::<i8>() {
                unsafe { sqlite3_free(aPoslist as *mut ()) };
                unsafe { sqlite3_free(aFree as *mut ()) };
                unsafe {
                    (*pPhrase).doclist.pList = std::ptr::null_mut::<i8>();
                }
                unsafe {
                    (*pPhrase).doclist.nList = 0 as i32;
                }
                return 0 as i32;
            } else {
                if aPoslist == std::ptr::null_mut::<i8>() {
                    aPoslist = pList;
                    nPoslist = nList;
                } else {
                    let mut aOut: *mut i8 = pList;
                    let mut p1: *mut i8 = aPoslist;
                    let mut p2: *mut i8 = aOut;
                    0 as i32;
                    fts3PoslistPhraseMerge(
                        std::ptr::addr_of_mut!(aOut),
                        iToken - iPrev,
                        0 as i32,
                        1 as i32,
                        std::ptr::addr_of_mut!(p1),
                        std::ptr::addr_of_mut!(p2),
                    );
                    unsafe { sqlite3_free(aPoslist as *mut ()) };
                    aPoslist = pList;
                    nPoslist = ((unsafe { aOut.offset_from(aPoslist as *mut i8) }) as i64) as i32;
                    if nPoslist == (0 as i32) {
                        unsafe { sqlite3_free(aPoslist as *mut ()) };
                        unsafe { sqlite3_free(aFree as *mut ()) };
                        unsafe {
                            (*pPhrase).doclist.pList = std::ptr::null_mut::<i8>();
                        }
                        unsafe {
                            (*pPhrase).doclist.nList = 0 as i32;
                        }
                        return 0 as i32;
                    }
                }
            }
            iPrev = iToken;
        }
        let __v2281: i32 = iToken;
        let __v2282: i32 = __v2281 + (1 as i32);
        iToken = __v2282;
    }
    if iPrev >= (0 as i32) {
        let mut nMaxUndeferred: i32 = unsafe { (*pPhrase).iDoclistToken };
        if nMaxUndeferred < (0 as i32) {
            unsafe {
                (*pPhrase).doclist.pList = aPoslist;
            }
            unsafe {
                (*pPhrase).doclist.nList = nPoslist;
            }
            unsafe {
                (*pPhrase).doclist.iDocid = unsafe { (*pCsr).iPrevId };
            }
            unsafe {
                (*pPhrase).doclist.bFreeList = 1 as i32;
            }
        } else {
            let mut nDistance: i32 = 0 as i32;
            let mut p1: *mut i8 = unsafe { std::mem::zeroed() };
            let mut p2: *mut i8 = unsafe { std::mem::zeroed() };
            let mut aOut: *mut i8 = unsafe { std::mem::zeroed() };
            let mut nAlloc: i64 = (nPoslist as i64) * ((2 as i32) as i64) + ((8 as i32) as i64);
            if nMaxUndeferred > iPrev {
                p1 = aPoslist;
                p2 = unsafe { (*pPhrase).doclist.pList };
                nDistance = nMaxUndeferred - iPrev;
            } else {
                p1 = unsafe { (*pPhrase).doclist.pList };
                p2 = aPoslist;
                nDistance = iPrev - nMaxUndeferred;
            }
            aOut = (unsafe { sqlite3Fts3MallocZero(nAlloc) }) as *mut i8;
            if !(aOut != std::ptr::null_mut::<i8>()) {
                unsafe { sqlite3_free(aPoslist as *mut ()) };
                return 7 as i32;
            }
            unsafe {
                (*pPhrase).doclist.pList = aOut;
            }
            0 as i32;
            if fts3PoslistPhraseMerge(
                std::ptr::addr_of_mut!(aOut),
                nDistance,
                0 as i32,
                1 as i32,
                std::ptr::addr_of_mut!(p1),
                std::ptr::addr_of_mut!(p2),
            ) != (0 as i32)
            {
                unsafe {
                    (*pPhrase).doclist.bFreeList = 1 as i32;
                }
                unsafe {
                    (*pPhrase).doclist.nList = ((unsafe {
                        aOut.offset_from((unsafe { (*pPhrase).doclist.pList }) as *mut i8)
                    }) as i64) as i32;
                }
            } else {
                unsafe { sqlite3_free(aOut as *mut ()) };
                unsafe {
                    (*pPhrase).doclist.pList = std::ptr::null_mut::<i8>();
                }
                unsafe {
                    (*pPhrase).doclist.nList = 0 as i32;
                }
            }
            unsafe { sqlite3_free(aPoslist as *mut ()) };
        }
    }
    if (unsafe { (*pPhrase).doclist.pList }) != aFree {
        unsafe { sqlite3_free(aFree as *mut ()) };
    }
    return 0 as i32;
}

// Maximum number of tokens a phrase may have to be considered for the
// incremental doclists strategy.
/// This function is called for each Fts3Phrase in a full-text query
/// expression to initialize the mechanism for returning rows. Once this
/// function has been called successfully on an Fts3Phrase, it may be
/// used with fts3EvalPhraseNext() to iterate through the matching docids.
///
/// If parameter bOptOk is true, then the phrase may (or may not) use the
/// incremental loading strategy. Otherwise, the entire doclist is loaded into
/// memory within this call.
///
/// SQLITE_OK is returned if no error occurs, otherwise an SQLite error code.
fn fts3EvalPhraseStart(mut pCsr: *mut Fts3Cursor, mut bOptOk: i32, mut p: *mut Fts3Phrase) -> i32 {
    let mut pTab: *mut Fts3Table = (unsafe { (*pCsr).base.pVtab }) as *mut Fts3Table;
    let mut rc: i32 = 0 as i32; // Error code
    let mut i: i32 = 0 as i32;
    // Determine if doclists may be loaded from disk incrementally. This is
    // possible if the bOptOk argument is true, the FTS doclists will be
    // scanned in forward order, and the phrase consists of
    // MAX_INCR_PHRASE_TOKENS or fewer tokens, none of which are are "^first"
    // tokens or prefix tokens that cannot use a prefix-index.
    let mut bHaveIncr: i32 = 0 as i32;
    let mut bIncrOk: i32 = (bOptOk != (0 as i32)
        && (((unsafe { (*pCsr).bDesc }) as u32) as i32)
            == (((unsafe { (*pTab).bDescIdx }) as u32) as i32)
        && (unsafe { (*p).nToken }) <= (4 as i32)
        && (unsafe { (*p).nToken }) > (0 as i32)) as i32;
    i = 0 as i32;
    '__slate_break_1794: loop {
        if !(bIncrOk == (1 as i32) && i < unsafe { (*p).nToken }) {
            break;
        }
        let mut pToken: *mut Fts3PhraseToken = unsafe {
            unsafe { std::ptr::addr_of_mut!((*p).aToken) as *mut Fts3PhraseToken }
                .offset(i as isize)
        };
        if (unsafe { (*pToken).bFirst }) != (0 as i32)
            || (unsafe { (*pToken).pSegcsr }) != std::ptr::null_mut::<Fts3MultiSegReader>()
                && !((unsafe { (*unsafe { (*pToken).pSegcsr }).bLookup }) != (0 as i32))
        {
            bIncrOk = 0 as i32;
        }
        if (unsafe { (*pToken).pSegcsr }) != std::ptr::null_mut::<Fts3MultiSegReader>() {
            bHaveIncr = 1 as i32;
        }
        let __v2283: i32 = i;
        let __v2284: i32 = __v2283 + (1 as i32);
        i = __v2284;
    }
    if bIncrOk != (0 as i32) && bHaveIncr != (0 as i32) {
        // Use the incremental approach.
        let mut iCol: i32 = if (unsafe { (*p).iColumn }) >= unsafe { (*pTab).nColumn } {
            -(1 as i32)
        } else {
            unsafe { (*p).iColumn }
        };
        i = 0 as i32;
        '__slate_break_1795: loop {
            if !(rc == (0 as i32) && i < unsafe { (*p).nToken }) {
                break;
            }
            let mut pToken: *mut Fts3PhraseToken = unsafe {
                unsafe { std::ptr::addr_of_mut!((*p).aToken) as *mut Fts3PhraseToken }
                    .offset(i as isize)
            };
            let mut pSegcsr: *mut Fts3MultiSegReader = unsafe { (*pToken).pSegcsr };
            if pSegcsr != std::ptr::null_mut::<Fts3MultiSegReader>() {
                rc = unsafe {
                    sqlite3Fts3MsrIncrStart(
                        pTab,
                        pSegcsr,
                        iCol,
                        (unsafe { (*pToken).z }) as *const i8,
                        unsafe { (*pToken).n },
                    )
                };
            }
            let __v2285: i32 = i;
            let __v2286: i32 = __v2285 + (1 as i32);
            i = __v2286;
        }
        unsafe {
            (*p).bIncr = 1 as i32;
        }
    } else {
        // Load the full doclist for the phrase into memory.
        rc = fts3EvalPhraseLoad(pCsr, p);
        unsafe {
            (*p).bIncr = 0 as i32;
        }
    }
    0 as i32;
    return rc;
}

/// This function is used to iterate backwards (from the end to start)
/// through doclists. It is used by this module to iterate through phrase
/// doclists in reverse and by the fts3_write.c module to iterate through
/// pending-terms lists when writing to databases with "order=desc".
///
/// The doclist may be sorted in ascending (parameter bDescIdx==0) or
/// descending (parameter bDescIdx==1) order of docid. Regardless, this
/// function iterates from the end of the doclist to the beginning.
///
/// # Arguments
///
/// * `bDescIdx` - True if the doclist is desc
/// * `aDoclist` - Pointer to entire doclist
/// * `nDoclist` - Length of aDoclist in bytes
/// * `ppIter` - IN/OUT: Iterator pointer
/// * `piDocid` - IN/OUT: Docid pointer
/// * `pnList` - OUT: List length pointer
/// * `pbEof` - OUT: End-of-file flag
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3Fts3DoclistPrev(
    mut bDescIdx: i32,
    mut aDoclist: *mut i8,
    mut nDoclist: i32,
    mut ppIter: *mut *mut i8,
    mut piDocid: *mut i64,
    mut pnList: *mut i32,
    mut pbEof: *mut u8,
) {
    let mut p: *mut i8 = unsafe { *ppIter };
    0 as i32;
    0 as i32;
    0 as i32;
    0 as i32;
    if p == std::ptr::null_mut::<i8>() {
        let mut iDocid: i64 = (0 as i32) as i64;
        let mut pNext: *mut i8 = std::ptr::null_mut::<i8>();
        let mut pDocid: *mut i8 = aDoclist;
        let mut pEnd: *mut i8 = unsafe { aDoclist.offset(nDoclist as isize) };
        let mut iMul: i32 = 1 as i32;
        '__slate_break_1796: while pDocid < pEnd {
            let mut iDelta: i64 = 0 as i64;
            let __v1889: *mut i8 = pDocid;
            let __v1890: *mut i8 = unsafe {
                __v1889.offset(sqlite3Fts3GetVarint(
                    pDocid as *const i8,
                    std::ptr::addr_of_mut!(iDelta),
                ) as isize)
            };
            pDocid = __v1890;
            let __v1891: i64 = iDocid;
            let __v1892: i64 = __v1891 + (iMul as i64) * iDelta;
            iDocid = __v1892;
            pNext = pDocid;
            fts3PoslistCopy(
                std::ptr::null_mut::<*mut i8>(),
                std::ptr::addr_of_mut!(pDocid),
            );
            '__slate_break_1797: while pDocid < pEnd && ((unsafe { *pDocid }) as i32) == (0 as i32)
            {
                let __v1893: *mut i8 = pDocid;
                let __v1894: *mut i8 = unsafe { __v1893.offset((1 as i32) as isize) };
                pDocid = __v1894;
            }
            iMul = if bDescIdx != (0 as i32) {
                -(1 as i32)
            } else {
                1 as i32
            };
        }
        unsafe {
            *pnList = ((unsafe { pEnd.offset_from(pNext as *mut i8) }) as i64) as i32;
        }
        unsafe {
            *ppIter = pNext;
        }
        unsafe {
            *piDocid = iDocid;
        }
    } else {
        let mut iMul: i32 = if bDescIdx != (0 as i32) {
            -(1 as i32)
        } else {
            1 as i32
        };
        let mut iDelta: i64 = 0 as i64;
        fts3GetReverseVarint(
            std::ptr::addr_of_mut!(p),
            aDoclist,
            std::ptr::addr_of_mut!(iDelta),
        );
        let __v1895: *mut i64 = piDocid;
        let __v1896: i64 = unsafe { *__v1895 };
        let __v1897: i64 = __v1896 - (iMul as i64) * iDelta;
        unsafe {
            *__v1895 = __v1897;
        }
        if p == aDoclist {
            unsafe {
                *pbEof = ((1 as i32) as i8) as u8;
            }
        } else {
            let mut pSave: *mut i8 = p;
            fts3ReversePoslist(aDoclist, std::ptr::addr_of_mut!(p));
            unsafe {
                *pnList = ((unsafe { pSave.offset_from(p as *mut i8) }) as i64) as i32;
            }
        }
        unsafe {
            *ppIter = p;
        }
    }
}

/// Iterate forwards through a doclist.
///
/// # Arguments
///
/// * `bDescIdx` - True if the doclist is desc
/// * `aDoclist` - Pointer to entire doclist
/// * `nDoclist` - Length of aDoclist in bytes
/// * `ppIter` - IN/OUT: Iterator pointer
/// * `piDocid` - IN/OUT: Docid pointer
/// * `pbEof` - OUT: End-of-file flag
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3Fts3DoclistNext(
    mut bDescIdx: i32,
    mut aDoclist: *mut i8,
    mut nDoclist: i32,
    mut ppIter: *mut *mut i8,
    mut piDocid: *mut i64,
    mut pbEof: *mut u8,
) {
    let mut p: *mut i8 = unsafe { *ppIter };
    0 as i32;
    0 as i32;
    0 as i32;
    0 as i32;
    if p == std::ptr::null_mut::<i8>() {
        p = aDoclist;
        let __v2287: *mut i8 = p;
        let __v2288: *mut i8 =
            unsafe { __v2287.offset(sqlite3Fts3GetVarint(p as *const i8, piDocid) as isize) };
        p = __v2288;
    } else {
        fts3PoslistCopy(std::ptr::null_mut::<*mut i8>(), std::ptr::addr_of_mut!(p));
        '__slate_break_1798: while p < unsafe { aDoclist.offset(nDoclist as isize) }
            && ((unsafe { *p }) as i32) == (0 as i32)
        {
            let __v2289: *mut i8 = p;
            let __v2290: *mut i8 = unsafe { __v2289.offset((1 as i32) as isize) };
            p = __v2290;
        }
        if p >= unsafe { aDoclist.offset(nDoclist as isize) } {
            unsafe {
                *pbEof = ((1 as i32) as i8) as u8;
            }
        } else {
            let mut iVar: i64 = 0 as i64;
            let __v2291: *mut i8 = p;
            let __v2292: *mut i8 = unsafe {
                __v2291.offset(
                    sqlite3Fts3GetVarint(p as *const i8, std::ptr::addr_of_mut!(iVar)) as isize,
                )
            };
            p = __v2292;
            let __v2293: *mut i64 = piDocid;
            let __v2294: i64 = unsafe { *__v2293 };
            let __v2295: i64 = __v2294
                + ((if bDescIdx != (0 as i32) {
                    -(1 as i32)
                } else {
                    1 as i32
                }) as i64)
                    * iVar;
            unsafe {
                *__v2293 = __v2295;
            }
        }
    }
    unsafe {
        *ppIter = p;
    }
}

/// Advance the iterator pDL to the next entry in pDL->aAll/nAll. Set *pbEof
/// to true if EOF is reached.
fn fts3EvalDlPhraseNext(mut pTab: *mut Fts3Table, mut pDL: *mut Fts3Doclist, mut pbEof: *mut u8) {
    let mut pIter: *mut i8 = unsafe { std::mem::zeroed() }; // Used to iterate through aAll
    let mut pEnd: *mut i8 = unsafe { std::mem::zeroed() }; // 1 byte past end of aAll
    if (unsafe { (*pDL).pNextDocid }) != std::ptr::null_mut::<i8>() {
        pIter = unsafe { (*pDL).pNextDocid };
        0 as i32;
    } else {
        pIter = unsafe { (*pDL).aAll };
    }
    let __v2296: bool;
    if pIter == std::ptr::null_mut::<i8>() {
        __v2296 = true as bool;
    } else {
        let __v2297: *mut i8 =
            unsafe { unsafe { (*pDL).aAll }.offset((unsafe { (*pDL).nAll }) as isize) };
        pEnd = __v2297;
        __v2296 = pIter >= __v2297;
    }
    if __v2296 {
        // We have already reached the end of this doclist. EOF.
        unsafe {
            *pbEof = ((1 as i32) as i8) as u8;
        }
    } else {
        let mut iDelta: i64 = 0 as i64;
        let __v2298: *mut i8 = pIter;
        let __v2299: *mut i8 = unsafe {
            __v2298.offset(
                sqlite3Fts3GetVarint(pIter as *const i8, std::ptr::addr_of_mut!(iDelta)) as isize,
            )
        };
        pIter = __v2299;
        if (((unsafe { (*pTab).bDescIdx }) as u32) as i32) == (0 as i32)
            || (unsafe { (*pDL).pNextDocid }) == std::ptr::null_mut::<i8>()
        {
            let __v2300: *mut Fts3Doclist = pDL;
            let __v2301: i64 = unsafe { (*__v2300).iDocid };
            let __v2302: i64 = __v2301 + iDelta;
            unsafe {
                (*__v2300).iDocid = __v2302;
            }
        } else {
            let __v2303: *mut Fts3Doclist = pDL;
            let __v2304: i64 = unsafe { (*__v2303).iDocid };
            let __v2305: i64 = __v2304 - iDelta;
            unsafe {
                (*__v2303).iDocid = __v2305;
            }
        }
        unsafe {
            (*pDL).pList = pIter;
        }
        fts3PoslistCopy(
            std::ptr::null_mut::<*mut i8>(),
            std::ptr::addr_of_mut!(pIter),
        );
        unsafe {
            (*pDL).nList = ((unsafe { pIter.offset_from((unsafe { (*pDL).pList }) as *mut i8) })
                as i64) as i32;
        }
        // pIter now points just past the 0x00 that terminates the position-
        // list for document pDL->iDocid. However, if this position-list was
        // edited in place by fts3EvalNearTrim(), then pIter may not actually
        // point to the start of the next docid value. The following line deals
        // with this case by advancing pIter past the zero-padding added by
        // fts3EvalNearTrim().
        '__slate_break_1799: while pIter < pEnd && ((unsafe { *pIter }) as i32) == (0 as i32) {
            let __v2306: *mut i8 = pIter;
            let __v2307: *mut i8 = unsafe { __v2306.offset((1 as i32) as isize) };
            pIter = __v2307;
        }
        unsafe {
            (*pDL).pNextDocid = pIter;
        }
        0 as i32;
        unsafe {
            *pbEof = ((0 as i32) as i8) as u8;
        }
    }
}

/// Helper type used by fts3EvalIncrPhraseNext() and incrPhraseTokenNext().
#[repr(C)]
#[derive(Clone, Copy)]
struct TokenDoclist {
    bIgnore: i32,
    iDocid: i64,
    pList: *mut i8,
    nList: i32,
}

/// Token pToken is an incrementally loaded token that is part of a
/// multi-token phrase. Advance it to the next matching document in the
/// database and populate output variable *p with the details of the new
/// entry. Or, if the iterator has reached EOF, set *pbEof to true.
///
/// If an error occurs, return an SQLite error code. Otherwise, return
/// SQLITE_OK.
///
/// # Arguments
///
/// * `pTab` - Virtual table handle
/// * `pPhrase` - Phrase to advance token of
/// * `iToken` - Specific token to advance
/// * `p` - OUT: Docid and doclist for new entry
/// * `pbEof` - OUT: True if iterator is at EOF
fn incrPhraseTokenNext(
    mut pTab: *mut Fts3Table,
    mut pPhrase: *mut Fts3Phrase,
    mut iToken: i32,
    mut p: *mut TokenDoclist,
    mut pbEof: *mut u8,
) -> i32 {
    let mut rc: i32 = 0 as i32;
    if (unsafe { (*pPhrase).iDoclistToken }) == iToken {
        0 as i32;
        0 as i32;
        fts3EvalDlPhraseNext(
            pTab,
            unsafe { std::ptr::addr_of_mut!((*pPhrase).doclist) },
            pbEof,
        );
        unsafe {
            (*p).pList = unsafe { (*pPhrase).doclist.pList };
        }
        unsafe {
            (*p).nList = unsafe { (*pPhrase).doclist.nList };
        }
        unsafe {
            (*p).iDocid = unsafe { (*pPhrase).doclist.iDocid };
        }
    } else {
        let mut pToken: *mut Fts3PhraseToken = unsafe {
            unsafe { std::ptr::addr_of_mut!((*pPhrase).aToken) as *mut Fts3PhraseToken }
                .offset(iToken as isize)
        };
        0 as i32;
        0 as i32;
        if (unsafe { (*pToken).pSegcsr }) != std::ptr::null_mut::<Fts3MultiSegReader>() {
            0 as i32;
            rc = unsafe {
                sqlite3Fts3MsrIncrNext(
                    pTab,
                    unsafe { (*pToken).pSegcsr },
                    unsafe { std::ptr::addr_of_mut!((*p).iDocid) },
                    unsafe { std::ptr::addr_of_mut!((*p).pList) },
                    unsafe { std::ptr::addr_of_mut!((*p).nList) },
                )
            };
            if (unsafe { (*p).pList }) == std::ptr::null_mut::<i8>() {
                unsafe {
                    *pbEof = ((1 as i32) as i8) as u8;
                }
            }
        } else {
            unsafe {
                (*p).bIgnore = 1 as i32;
            }
        }
    }
    return rc;
}

/// The phrase iterator passed as the second argument:
///
///   * features at least one token that uses an incremental doclist, and
///
///   * does not contain any deferred tokens.
///
/// Advance it to the next matching document in the database and populate
/// the Fts3Doclist.pList and nList fields.
///
/// If there is no "next" entry and no error occurs, then *pbEof is set to
/// 1 before returning. Otherwise, if no error occurs and the iterator is
/// successfully advanced, *pbEof is set to 0.
///
/// If an error occurs, return an SQLite error code. Otherwise, return
/// SQLITE_OK.
///
/// # Arguments
///
/// * `pCsr` - FTS Cursor handle
/// * `p` - Phrase object to advance to next docid
/// * `pbEof` - OUT: Set to 1 if EOF
fn fts3EvalIncrPhraseNext(
    mut pCsr: *mut Fts3Cursor,
    mut p: *mut Fts3Phrase,
    mut pbEof: *mut u8,
) -> i32 {
    let mut rc: i32 = 0 as i32;
    let mut pDL: *mut Fts3Doclist = unsafe { std::ptr::addr_of_mut!((*p).doclist) };
    let mut pTab: *mut Fts3Table = (unsafe { (*pCsr).base.pVtab }) as *mut Fts3Table;
    let mut bEof: u8 = ((0 as i32) as i8) as u8;
    // This is only called if it is guaranteed that the phrase has at least
    // one incremental token. In which case the bIncr flag is set.
    0 as i32;
    if (unsafe { (*p).nToken }) == (1 as i32) {
        rc = unsafe {
            sqlite3Fts3MsrIncrNext(
                pTab,
                unsafe {
                    (*unsafe {
                        unsafe { std::ptr::addr_of_mut!((*p).aToken) as *mut Fts3PhraseToken }
                            .offset((0 as i32) as isize)
                    })
                    .pSegcsr
                },
                unsafe { std::ptr::addr_of_mut!((*pDL).iDocid) },
                unsafe { std::ptr::addr_of_mut!((*pDL).pList) },
                unsafe { std::ptr::addr_of_mut!((*pDL).nList) },
            )
        };
        if (unsafe { (*pDL).pList }) == std::ptr::null_mut::<i8>() {
            bEof = ((1 as i32) as i8) as u8;
        }
    } else {
        let mut bDescDoclist: i32 = ((unsafe { (*pCsr).bDesc }) as u32) as i32;
        let mut a: __SlateAlign16<[TokenDoclist; 4]> =
            __SlateAlign16(unsafe { std::mem::zeroed() });
        unsafe {
            memset(
                (a.0.as_mut_ptr() as *mut TokenDoclist) as *mut (),
                0 as i32,
                128 as u64,
            )
        };
        0 as i32;
        0 as i32;
        '__slate_break_1800: while ((bEof as u32) as i32) == (0 as i32) {
            let mut bMaxSet: i32 = 0 as i32;
            let mut iMax: i64 = (0 as i32) as i64; // Largest docid for all iterators
            let mut i: i32 = 0 as i32; // Used to iterate through tokens
            // Advance the iterator for each token in the phrase once.
            i = 0 as i32;
            '__slate_break_1801: loop {
                if !(rc == (0 as i32)
                    && i < unsafe { (*p).nToken }
                    && ((bEof as u32) as i32) == (0 as i32))
                {
                    break;
                }
                rc = incrPhraseTokenNext(
                    pTab,
                    p,
                    i,
                    unsafe { (a.0.as_mut_ptr() as *mut TokenDoclist).offset(i as isize) },
                    std::ptr::addr_of_mut!(bEof),
                );
                if (unsafe {
                    (*unsafe { (a.0.as_mut_ptr() as *mut TokenDoclist).offset(i as isize) }).bIgnore
                }) == (0 as i32)
                    && (bMaxSet == (0 as i32)
                        || (if bDescDoclist != (0 as i32) {
                            -(1 as i32)
                        } else {
                            1 as i32
                        }) * if iMax
                            > unsafe {
                                (*unsafe {
                                    (a.0.as_mut_ptr() as *mut TokenDoclist).offset(i as isize)
                                })
                                .iDocid
                            } {
                            1 as i32
                        } else {
                            if iMax
                                == unsafe {
                                    (*unsafe {
                                        (a.0.as_mut_ptr() as *mut TokenDoclist).offset(i as isize)
                                    })
                                    .iDocid
                                }
                            {
                                0 as i32
                            } else {
                                -(1 as i32)
                            }
                        } < (0 as i32))
                {
                    iMax = unsafe {
                        (*unsafe { (a.0.as_mut_ptr() as *mut TokenDoclist).offset(i as isize) })
                            .iDocid
                    };
                    bMaxSet = 1 as i32;
                }
                let __v2308: i32 = i;
                let __v2309: i32 = __v2308 + (1 as i32);
                i = __v2309;
            }
            0 as i32;
            0 as i32;
            // Keep advancing iterators until they all point to the same document
            i = 0 as i32;
            '__slate_break_1802: loop {
                if !(i < unsafe { (*p).nToken }) {
                    break;
                }
                '__slate_break_1803: while rc == (0 as i32)
                    && ((bEof as u32) as i32) == (0 as i32)
                    && (unsafe {
                        (*unsafe { (a.0.as_mut_ptr() as *mut TokenDoclist).offset(i as isize) })
                            .bIgnore
                    }) == (0 as i32)
                    && (if bDescDoclist != (0 as i32) {
                        -(1 as i32)
                    } else {
                        1 as i32
                    }) * if (unsafe {
                        (*unsafe { (a.0.as_mut_ptr() as *mut TokenDoclist).offset(i as isize) })
                            .iDocid
                    }) > iMax
                    {
                        1 as i32
                    } else {
                        if (unsafe {
                            (*unsafe { (a.0.as_mut_ptr() as *mut TokenDoclist).offset(i as isize) })
                                .iDocid
                        }) == iMax
                        {
                            0 as i32
                        } else {
                            -(1 as i32)
                        }
                    } < (0 as i32)
                {
                    rc = incrPhraseTokenNext(
                        pTab,
                        p,
                        i,
                        unsafe { (a.0.as_mut_ptr() as *mut TokenDoclist).offset(i as isize) },
                        std::ptr::addr_of_mut!(bEof),
                    );
                    if (if bDescDoclist != (0 as i32) {
                        -(1 as i32)
                    } else {
                        1 as i32
                    }) * if (unsafe {
                        (*unsafe { (a.0.as_mut_ptr() as *mut TokenDoclist).offset(i as isize) })
                            .iDocid
                    }) > iMax
                    {
                        1 as i32
                    } else {
                        if (unsafe {
                            (*unsafe { (a.0.as_mut_ptr() as *mut TokenDoclist).offset(i as isize) })
                                .iDocid
                        }) == iMax
                        {
                            0 as i32
                        } else {
                            -(1 as i32)
                        }
                    } > (0 as i32)
                    {
                        iMax = unsafe {
                            (*unsafe { (a.0.as_mut_ptr() as *mut TokenDoclist).offset(i as isize) })
                                .iDocid
                        };
                        i = 0 as i32;
                    }
                }
                let __v2310: i32 = i;
                let __v2311: i32 = __v2310 + (1 as i32);
                i = __v2311;
            }
            // Check if the current entries really are a phrase match
            if ((bEof as u32) as i32) == (0 as i32) {
                let mut nList: i32 = 0 as i32;
                let mut nByte: i32 = unsafe {
                    (*unsafe {
                        (a.0.as_mut_ptr() as *mut TokenDoclist)
                            .offset(((unsafe { (*p).nToken }) - (1 as i32)) as isize)
                    })
                    .nList
                };
                let mut aDoclist: *mut i8 =
                    (unsafe { sqlite3_malloc64(((nByte as i64) + ((8 as i32) as i64)) as u64) })
                        as *mut i8;
                if !(aDoclist != std::ptr::null_mut::<i8>()) {
                    return 7 as i32;
                }
                unsafe {
                    memcpy(
                        aDoclist as *mut (),
                        (unsafe {
                            (*unsafe {
                                (a.0.as_mut_ptr() as *mut TokenDoclist)
                                    .offset(((unsafe { (*p).nToken }) - (1 as i32)) as isize)
                            })
                            .pList
                        }) as *const (),
                        ((nByte + (1 as i32)) as i64) as u64,
                    )
                };
                unsafe {
                    memset(
                        (unsafe { aDoclist.offset(nByte as isize) }) as *mut (),
                        0 as i32,
                        ((8 as i32) as i64) as u64,
                    )
                };
                i = 0 as i32;
                '__slate_break_1804: loop {
                    if !(i < (unsafe { (*p).nToken }) - (1 as i32)) {
                        break;
                    }
                    if (unsafe {
                        (*unsafe { (a.0.as_mut_ptr() as *mut TokenDoclist).offset(i as isize) })
                            .bIgnore
                    }) == (0 as i32)
                    {
                        let mut pL: *mut i8 = unsafe {
                            (*unsafe { (a.0.as_mut_ptr() as *mut TokenDoclist).offset(i as isize) })
                                .pList
                        };
                        let mut pR: *mut i8 = aDoclist;
                        let mut pOut: *mut i8 = aDoclist;
                        let mut nDist: i32 = (unsafe { (*p).nToken }) - (1 as i32) - i;
                        let mut res: i32 = fts3PoslistPhraseMerge(
                            std::ptr::addr_of_mut!(pOut),
                            nDist,
                            0 as i32,
                            1 as i32,
                            std::ptr::addr_of_mut!(pL),
                            std::ptr::addr_of_mut!(pR),
                        );
                        if res == (0 as i32) {
                            break '__slate_break_1804;
                        }
                        nList = ((unsafe { pOut.offset_from(aDoclist as *mut i8) }) as i64) as i32;
                    }
                    let __v2312: i32 = i;
                    let __v2313: i32 = __v2312 + (1 as i32);
                    i = __v2313;
                }
                if i == (unsafe { (*p).nToken }) - (1 as i32) {
                    unsafe {
                        (*pDL).iDocid = iMax;
                    }
                    unsafe {
                        (*pDL).pList = aDoclist;
                    }
                    unsafe {
                        (*pDL).nList = nList;
                    }
                    unsafe {
                        (*pDL).bFreeList = 1 as i32;
                    }
                    break '__slate_break_1800;
                }
                unsafe { sqlite3_free(aDoclist as *mut ()) };
            }
        }
    }
    unsafe {
        *pbEof = bEof;
    }
    return rc;
}

/// Attempt to move the phrase iterator to point to the next matching docid.
/// If an error occurs, return an SQLite error code. Otherwise, return
/// SQLITE_OK.
///
/// If there is no "next" entry and no error occurs, then *pbEof is set to
/// 1 before returning. Otherwise, if no error occurs and the iterator is
/// successfully advanced, *pbEof is set to 0.
///
/// # Arguments
///
/// * `pCsr` - FTS Cursor handle
/// * `p` - Phrase object to advance to next docid
/// * `pbEof` - OUT: Set to 1 if EOF
fn fts3EvalPhraseNext(
    mut pCsr: *mut Fts3Cursor,
    mut p: *mut Fts3Phrase,
    mut pbEof: *mut u8,
) -> i32 {
    let mut rc: i32 = 0 as i32;
    let mut pDL: *mut Fts3Doclist = unsafe { std::ptr::addr_of_mut!((*p).doclist) };
    let mut pTab: *mut Fts3Table = (unsafe { (*pCsr).base.pVtab }) as *mut Fts3Table;
    if (unsafe { (*p).bIncr }) != (0 as i32) {
        rc = fts3EvalIncrPhraseNext(pCsr, p, pbEof);
    } else {
        if (((unsafe { (*pCsr).bDesc }) as u32) as i32)
            != (((unsafe { (*pTab).bDescIdx }) as u32) as i32)
            && (unsafe { (*pDL).nAll }) != (0 as i32)
        {
            sqlite3Fts3DoclistPrev(
                ((unsafe { (*pTab).bDescIdx }) as u32) as i32,
                unsafe { (*pDL).aAll },
                unsafe { (*pDL).nAll },
                unsafe { std::ptr::addr_of_mut!((*pDL).pNextDocid) },
                unsafe { std::ptr::addr_of_mut!((*pDL).iDocid) },
                unsafe { std::ptr::addr_of_mut!((*pDL).nList) },
                pbEof,
            );
            unsafe {
                (*pDL).pList = unsafe { (*pDL).pNextDocid };
            }
        } else {
            fts3EvalDlPhraseNext(pTab, pDL, pbEof);
        }
    }
    return rc;
}

/// If *pRc is not SQLITE_OK when this function is called, it is a no-op.
/// Otherwise, fts3EvalPhraseStart() is called on all phrases within the
/// expression. Also the Fts3Expr.bDeferred variable is set to true for any
/// expressions for which all descendent tokens are deferred.
///
/// If parameter bOptOk is zero, then it is guaranteed that the
/// Fts3Phrase.doclist.aAll/nAll variables contain the entire doclist for
/// each phrase in the expression (subject to deferred token processing).
/// Or, if bOptOk is non-zero, then one or more tokens within the expression
/// may be loaded incrementally, meaning doclist.aAll/nAll is not available.
///
/// If an error occurs within this function, *pRc is set to an SQLite error
/// code before returning.
///
/// # Arguments
///
/// * `pCsr` - FTS Cursor handle
/// * `pExpr` - Expression to initialize phrases in
/// * `pRc` - IN/OUT: Error code
fn fts3EvalStartReaders(mut pCsr: *mut Fts3Cursor, mut pExpr: *mut Fts3Expr, mut pRc: *mut i32) {
    if pExpr != std::ptr::null_mut::<Fts3Expr>() && (0 as i32) == unsafe { *pRc } {
        if (unsafe { (*pExpr).eType }) == (5 as i32) {
            let mut nToken: i32 = unsafe { (*unsafe { (*pExpr).pPhrase }).nToken };
            if nToken != (0 as i32) {
                let mut i: i32 = 0 as i32;
                i = 0 as i32;
                '__slate_break_1805: loop {
                    if !(i < nToken) {
                        break;
                    }
                    if (unsafe {
                        (*unsafe {
                            unsafe {
                                std::ptr::addr_of_mut!((*unsafe { (*pExpr).pPhrase }).aToken)
                                    as *mut Fts3PhraseToken
                            }
                            .offset(i as isize)
                        })
                        .pDeferred
                    }) == std::ptr::null_mut::<Fts3DeferredToken>()
                    {
                        break '__slate_break_1805;
                    }
                    let __v2314: i32 = i;
                    let __v2315: i32 = __v2314 + (1 as i32);
                    i = __v2315;
                }
                unsafe {
                    (*pExpr).bDeferred = (i == nToken) as u8;
                }
            }
            unsafe {
                *pRc = fts3EvalPhraseStart(pCsr, 1 as i32, unsafe { (*pExpr).pPhrase });
            }
        } else {
            fts3EvalStartReaders(pCsr, unsafe { (*pExpr).pLeft }, pRc);
            fts3EvalStartReaders(pCsr, unsafe { (*pExpr).pRight }, pRc);
            unsafe {
                (*pExpr).bDeferred = ((unsafe { (*unsafe { (*pExpr).pLeft }).bDeferred })
                    != (0 as u8)
                    && (unsafe { (*unsafe { (*pExpr).pRight }).bDeferred }) != (0 as u8))
                    as u8;
            }
        }
    }
}

/// An array of the following structures is assembled as part of the process
/// of selecting tokens to defer before the query starts executing (as part
/// of the xFilter() method). There is one element in the array for each
/// token in the FTS expression.
///
/// Tokens are divided into AND/NEAR clusters. All tokens in a cluster belong
/// to phrases that are connected only by AND and NEAR operators (not OR or
/// NOT). When determining tokens to defer, each AND/NEAR cluster is considered
/// separately. The root of a tokens AND/NEAR cluster is stored in
/// Fts3TokenAndCost.pRoot.
#[repr(C)]
#[derive(Clone, Copy)]
struct Fts3TokenAndCost {
    /// The phrase the token belongs to
    pPhrase: *mut Fts3Phrase,
    /// Position of token in phrase
    iToken: i32,
    /// The token itself
    pToken: *mut Fts3PhraseToken,
    /// Root of NEAR/AND cluster
    pRoot: *mut Fts3Expr,
    /// Number of overflow pages to load doclist
    nOvfl: i32,
    /// The column the token must match
    iCol: i32,
}

/// This function is used to populate an allocated Fts3TokenAndCost array.
///
/// If *pRc is not SQLITE_OK when this function is called, it is a no-op.
/// Otherwise, if an error occurs during execution, *pRc is set to an
/// SQLite error code.
///
/// # Arguments
///
/// * `pCsr` - FTS Cursor handle
/// * `pRoot` - Root of current AND/NEAR cluster
/// * `pExpr` - Expression to consider
/// * `ppTC` - Write new entries to *(*ppTC)++
/// * `ppOr` - Write new OR root to *(*ppOr)++
/// * `pRc` - IN/OUT: Error code
fn fts3EvalTokenCosts(
    mut pCsr: *mut Fts3Cursor,
    mut pRoot: *mut Fts3Expr,
    mut pExpr: *mut Fts3Expr,
    mut ppTC: *mut *mut Fts3TokenAndCost,
    mut ppOr: *mut *mut *mut Fts3Expr,
    mut pRc: *mut i32,
) {
    if (unsafe { *pRc }) == (0 as i32) {
        if (unsafe { (*pExpr).eType }) == (5 as i32) {
            let mut pPhrase: *mut Fts3Phrase = unsafe { (*pExpr).pPhrase };
            let mut i: i32 = 0 as i32;
            i = 0 as i32;
            '__slate_break_1806: loop {
                if !((unsafe { *pRc }) == (0 as i32) && i < unsafe { (*pPhrase).nToken }) {
                    break;
                }
                let mut pTC: *mut Fts3TokenAndCost = unsafe { std::mem::zeroed() };
                let __v2318: *mut *mut Fts3TokenAndCost = ppTC;
                let __v2319: *mut Fts3TokenAndCost = unsafe { *__v2318 };
                let __v2320: *mut Fts3TokenAndCost = unsafe { __v2319.offset((1 as i32) as isize) };
                unsafe {
                    *__v2318 = __v2320;
                }
                pTC = __v2319;
                unsafe {
                    (*pTC).pPhrase = pPhrase;
                }
                unsafe {
                    (*pTC).iToken = i;
                }
                unsafe {
                    (*pTC).pRoot = pRoot;
                }
                unsafe {
                    (*pTC).pToken = unsafe {
                        unsafe { std::ptr::addr_of_mut!((*pPhrase).aToken) as *mut Fts3PhraseToken }
                            .offset(i as isize)
                    };
                }
                unsafe {
                    (*pTC).iCol = unsafe { (*pPhrase).iColumn };
                }
                unsafe {
                    *pRc = unsafe {
                        sqlite3Fts3MsrOvfl(
                            pCsr,
                            unsafe { (*unsafe { (*pTC).pToken }).pSegcsr },
                            unsafe { std::ptr::addr_of_mut!((*pTC).nOvfl) },
                        )
                    };
                }
                let __v2316: i32 = i;
                let __v2317: i32 = __v2316 + (1 as i32);
                i = __v2317;
            }
        } else {
            if (unsafe { (*pExpr).eType }) != (2 as i32) {
                0 as i32;
                0 as i32;
                if (unsafe { (*pExpr).eType }) == (4 as i32) {
                    pRoot = unsafe { (*pExpr).pLeft };
                    unsafe {
                        *unsafe { *ppOr } = pRoot;
                    }
                    let __v2321: *mut *mut *mut Fts3Expr = ppOr;
                    let __v2322: *mut *mut Fts3Expr = unsafe { *__v2321 };
                    let __v2323: *mut *mut Fts3Expr =
                        unsafe { __v2322.offset((1 as i32) as isize) };
                    unsafe {
                        *__v2321 = __v2323;
                    }
                }
                fts3EvalTokenCosts(pCsr, pRoot, unsafe { (*pExpr).pLeft }, ppTC, ppOr, pRc);
                if (unsafe { (*pExpr).eType }) == (4 as i32) {
                    pRoot = unsafe { (*pExpr).pRight };
                    unsafe {
                        *unsafe { *ppOr } = pRoot;
                    }
                    let __v2324: *mut *mut *mut Fts3Expr = ppOr;
                    let __v2325: *mut *mut Fts3Expr = unsafe { *__v2324 };
                    let __v2326: *mut *mut Fts3Expr =
                        unsafe { __v2325.offset((1 as i32) as isize) };
                    unsafe {
                        *__v2324 = __v2326;
                    }
                }
                fts3EvalTokenCosts(pCsr, pRoot, unsafe { (*pExpr).pRight }, ppTC, ppOr, pRc);
            }
        }
    }
}

/// Determine the average document (row) size in pages. If successful,
/// write this value to *pnPage and return SQLITE_OK. Otherwise, return
/// an SQLite error code.
///
/// The average document size in pages is calculated by first calculating
/// determining the average size in bytes, B. If B is less than the amount
/// of data that will fit on a single leaf page of an intkey table in
/// this database, then the average docsize is 1. Otherwise, it is 1 plus
/// the number of overflow pages consumed by a record B bytes in size.
fn fts3EvalAverageDocsize(mut pCsr: *mut Fts3Cursor, mut pnPage: *mut i32) -> i32 {
    let mut rc: i32 = 0 as i32;
    if (unsafe { (*pCsr).nRowAvg }) == (0 as i32) {
        // The average document size, which is required to calculate the cost
        // of each doclist, has not yet been determined. Read the required
        // data from the %_stat table to calculate it.
        //
        // Entry 0 of the %_stat table is a blob containing (nCol+1) FTS3
        // varints, where nCol is the number of columns in the FTS3 table.
        // The first varint is the number of documents currently stored in
        // the table. The following nCol varints contain the total amount of
        // data stored in all rows of each column of the table, from left
        // to right.
        let mut p: *mut Fts3Table = (unsafe { (*pCsr).base.pVtab }) as *mut Fts3Table;
        let mut pStmt: *mut sqlite3_stmt = unsafe { std::mem::zeroed() };
        let mut nDoc: i64 = (0 as i32) as i64;
        let mut nByte: i64 = (0 as i32) as i64;
        let mut pEnd: *const i8 = unsafe { std::mem::zeroed() };
        let mut a: *const i8 = unsafe { std::mem::zeroed() };
        rc = unsafe { sqlite3Fts3SelectDoctotal(p, std::ptr::addr_of_mut!(pStmt)) };
        if rc != (0 as i32) {
            return rc;
        }
        a = (unsafe { sqlite3_column_blob(pStmt, 0 as i32) }) as *const i8;
        {}
        // If %_stat.value set to X''
        if a != std::ptr::null::<i8>() {
            pEnd = unsafe { a.offset((unsafe { sqlite3_column_bytes(pStmt, 0 as i32) }) as isize) };
            let __v2327: *const i8 = a;
            let __v2328: *const i8 = unsafe {
                __v2327.offset(
                    sqlite3Fts3GetVarintBounded(a, pEnd, std::ptr::addr_of_mut!(nDoc)) as isize,
                )
            };
            a = __v2328;
            '__slate_break_1807: while a < pEnd {
                let __v2329: *const i8 = a;
                let __v2330: *const i8 = unsafe {
                    __v2329.offset(sqlite3Fts3GetVarintBounded(
                        a,
                        pEnd,
                        std::ptr::addr_of_mut!(nByte),
                    ) as isize)
                };
                a = __v2330;
            }
        }
        if nDoc == ((0 as i32) as i64) || nByte == ((0 as i32) as i64) {
            unsafe { sqlite3_reset(pStmt) };
            return (11 as i32) | (1 as i32) << (8 as i32);
        }
        unsafe {
            (*pCsr).nDoc = nDoc;
        }
        unsafe {
            (*pCsr).nRowAvg = ((nByte / nDoc + ((unsafe { (*p).nPgsz }) as i64))
                / ((unsafe { (*p).nPgsz }) as i64)) as i32;
        }
        0 as i32;
        rc = unsafe { sqlite3_reset(pStmt) };
    }
    unsafe {
        *pnPage = unsafe { (*pCsr).nRowAvg };
    }
    return rc;
}

/// This function is called to select the tokens (if any) that will be
/// deferred. The array aTC[] has already been populated when this is
/// called.
///
/// This function is called once for each AND/NEAR cluster in the
/// expression. Each invocation determines which tokens to defer within
/// the cluster with root node pRoot. See comments above the definition
/// of struct Fts3TokenAndCost for more details.
///
/// If no error occurs, SQLITE_OK is returned and sqlite3Fts3DeferToken()
/// called on each token to defer. Otherwise, an SQLite error code is
/// returned.
///
/// # Arguments
///
/// * `pCsr` - FTS Cursor handle
/// * `pRoot` - Consider tokens with this root node
/// * `aTC` - Array of expression tokens and costs
/// * `nTC` - Number of entries in aTC[]
fn fts3EvalSelectDeferred(
    mut pCsr: *mut Fts3Cursor,
    mut pRoot: *mut Fts3Expr,
    mut aTC: *mut Fts3TokenAndCost,
    mut nTC: i32,
) -> i32 {
    let mut pTab: *mut Fts3Table = (unsafe { (*pCsr).base.pVtab }) as *mut Fts3Table;
    let mut nDocSize: i32 = 0 as i32; // Number of pages per doc loaded
    let mut rc: i32 = 0 as i32; // Return code
    let mut ii: i32 = 0 as i32; // Iterator variable for various purposes
    let mut nOvfl: i32 = 0 as i32; // Total overflow pages used by doclists
    let mut nToken: i32 = 0 as i32; // Total number of tokens in cluster
    let mut nMinEst: i32 = 0 as i32; // The minimum count for any phrase so far.
    let mut nLoad4: i32 = 1 as i32; // (Phrases that will be loaded)^4.
    // Tokens are never deferred for FTS tables created using the content=xxx
    // option. The reason being that it is not guaranteed that the content
    // table actually contains the same data as the index. To prevent this from
    // causing any problems, the deferred token optimization is completely
    // disabled for content=xxx tables.
    if (unsafe { (*pTab).zContentTbl }) != std::ptr::null_mut::<i8>() {
        return 0 as i32;
    }
    // Count the tokens in this AND/NEAR cluster. If none of the doclists
    // associated with the tokens spill onto overflow pages, or if there is
    // only 1 token, exit early. No tokens to defer in this case.
    ii = 0 as i32;
    '__slate_break_1808: loop {
        if !(ii < nTC) {
            break;
        }
        if (unsafe { (*unsafe { aTC.offset(ii as isize) }).pRoot }) == pRoot {
            let __v2333: i32 = nOvfl;
            let __v2334: i32 = __v2333 + unsafe { (*unsafe { aTC.offset(ii as isize) }).nOvfl };
            nOvfl = __v2334;
            let __v2335: i32 = nToken;
            let __v2336: i32 = __v2335 + (1 as i32);
            nToken = __v2336;
        }
        let __v2331: i32 = ii;
        let __v2332: i32 = __v2331 + (1 as i32);
        ii = __v2332;
    }
    if nOvfl == (0 as i32) || nToken < (2 as i32) {
        return 0 as i32;
    }
    // Obtain the average docsize (in pages).
    rc = fts3EvalAverageDocsize(pCsr, std::ptr::addr_of_mut!(nDocSize));
    0 as i32;
    // Iterate through all tokens in this AND/NEAR cluster, in ascending order
    // of the number of overflow pages that will be loaded by the pager layer
    // to retrieve the entire doclist for the token from the full-text index.
    // Load the doclists for tokens that are either:
    //
    //   a. The cheapest token in the entire query (i.e. the one visited by the
    //      first iteration of this loop), or
    //
    //   b. Part of a multi-token phrase.
    //
    // After each token doclist is loaded, merge it with the others from the
    // same phrase and count the number of documents that the merged doclist
    // contains. Set variable "nMinEst" to the smallest number of documents in
    // any phrase doclist for which 1 or more token doclists have been loaded.
    // Let nOther be the number of other phrases for which it is certain that
    // one or more tokens will not be deferred.
    //
    // Then, for each token, defer it if loading the doclist would result in
    // loading N or more overflow pages into memory, where N is computed as:
    //
    //    (nMinEst + 4^nOther - 1) / (4^nOther)
    ii = 0 as i32;
    '__slate_break_1809: loop {
        if !(ii < nToken && rc == (0 as i32)) {
            break;
        }
        let mut iTC: i32 = 0 as i32; // Used to iterate through aTC[] array.
        let mut pTC: *mut Fts3TokenAndCost = std::ptr::null_mut::<Fts3TokenAndCost>(); // Set to cheapest remaining token.
        // Set pTC to point to the cheapest remaining token.
        iTC = 0 as i32;
        '__slate_break_1810: loop {
            if !(iTC < nTC) {
                break;
            }
            if (unsafe { (*unsafe { aTC.offset(iTC as isize) }).pToken })
                != std::ptr::null_mut::<Fts3PhraseToken>()
                && (unsafe { (*unsafe { aTC.offset(iTC as isize) }).pRoot }) == pRoot
                && (!(pTC != std::ptr::null_mut::<Fts3TokenAndCost>())
                    || (unsafe { (*unsafe { aTC.offset(iTC as isize) }).nOvfl })
                        < unsafe { (*pTC).nOvfl })
            {
                pTC = unsafe { aTC.offset(iTC as isize) };
            }
            let __v2339: i32 = iTC;
            let __v2340: i32 = __v2339 + (1 as i32);
            iTC = __v2340;
        }
        0 as i32;
        if ii != (0 as i32)
            && (unsafe { (*pTC).nOvfl })
                >= (nMinEst + nLoad4 / (4 as i32) - (1 as i32)) / (nLoad4 / (4 as i32)) * nDocSize
        {
            // The number of overflow pages to load for this (and therefore all
            // subsequent) tokens is greater than the estimated number of pages
            // that will be loaded if all subsequent tokens are deferred.
            let mut pToken: *mut Fts3PhraseToken = unsafe { (*pTC).pToken };
            rc = unsafe { sqlite3Fts3DeferToken(pCsr, pToken, unsafe { (*pTC).iCol }) };
            fts3SegReaderCursorFree(unsafe { (*pToken).pSegcsr });
            unsafe {
                (*pToken).pSegcsr = std::ptr::null_mut::<Fts3MultiSegReader>();
            }
        } else {
            // Set nLoad4 to the value of (4^nOther) for the next iteration of the
            // for-loop. Except, limit the value to 2^24 to prevent it from
            // overflowing the 32-bit integer it is stored in.
            if ii < (12 as i32) {
                nLoad4 = nLoad4 * (4 as i32);
            }
            if ii == (0 as i32)
                || (unsafe { (*unsafe { (*pTC).pPhrase }).nToken }) > (1 as i32)
                    && ii != nToken - (1 as i32)
            {
                // Either this is the cheapest token in the entire query, or it is
                // part of a multi-token phrase. Either way, the entire doclist will
                // (eventually) be loaded into memory. It may as well be now.
                let mut pToken: *mut Fts3PhraseToken = unsafe { (*pTC).pToken };
                let mut nList: i32 = 0 as i32;
                let mut pList: *mut i8 = std::ptr::null_mut::<i8>();
                rc = fts3TermSelect(
                    pTab,
                    pToken,
                    unsafe { (*pTC).iCol },
                    std::ptr::addr_of_mut!(nList),
                    std::ptr::addr_of_mut!(pList),
                );
                0 as i32;
                if rc == (0 as i32) {
                    rc = fts3EvalPhraseMergeToken(
                        pTab,
                        unsafe { (*pTC).pPhrase },
                        unsafe { (*pTC).iToken },
                        pList,
                        nList,
                    );
                }
                if rc == (0 as i32) {
                    let mut nCount: i32 = 0 as i32;
                    nCount = fts3DoclistCountDocids(
                        unsafe { (*unsafe { (*pTC).pPhrase }).doclist.aAll },
                        unsafe { (*unsafe { (*pTC).pPhrase }).doclist.nAll },
                    );
                    if ii == (0 as i32) || nCount < nMinEst {
                        nMinEst = nCount;
                    }
                }
            }
        }
        unsafe {
            (*pTC).pToken = std::ptr::null_mut::<Fts3PhraseToken>();
        }
        let __v2337: i32 = ii;
        let __v2338: i32 = __v2337 + (1 as i32);
        ii = __v2338;
    }
    return rc;
}

/// This function is called from within the xFilter method. It initializes
/// the full-text query currently stored in pCsr->pExpr. To iterate through
/// the results of a query, the caller does:
///
///    fts3EvalStart(pCsr);
///    while( 1 ){
///      fts3EvalNext(pCsr);
///      if( pCsr->bEof ) break;
///      ... return row pCsr->iPrevId to the caller ...
///    }
fn fts3EvalStart(mut pCsr: *mut Fts3Cursor) -> i32 {
    let mut pTab: *mut Fts3Table = (unsafe { (*pCsr).base.pVtab }) as *mut Fts3Table;
    let mut rc: i32 = 0 as i32;
    let mut nToken: i32 = 0 as i32;
    let mut nOr: i32 = 0 as i32;
    // Allocate a MultiSegReader for each token in the expression.
    fts3EvalAllocateReaders(
        pCsr,
        unsafe { (*pCsr).pExpr },
        std::ptr::addr_of_mut!(nToken),
        std::ptr::addr_of_mut!(nOr),
        std::ptr::addr_of_mut!(rc),
    );
    // Determine which, if any, tokens in the expression should be deferred.
    if rc == (0 as i32) && nToken > (1 as i32) && (unsafe { (*pTab).bFts4 }) != (0 as u8) {
        let mut aTC: *mut Fts3TokenAndCost = unsafe { std::mem::zeroed() };
        aTC = (unsafe {
            sqlite3_malloc64(
                (40 as u64)
                    .wrapping_mul((nToken as i64) as u64)
                    .wrapping_add(
                        (8 as u64)
                            .wrapping_mul((nOr as i64) as u64)
                            .wrapping_mul(((2 as i32) as i64) as u64),
                    ),
            )
        }) as *mut Fts3TokenAndCost;
        if !(aTC != std::ptr::null_mut::<Fts3TokenAndCost>()) {
            rc = 7 as i32;
        } else {
            let mut apOr: *mut *mut Fts3Expr =
                (unsafe { aTC.offset(nToken as isize) }) as *mut *mut Fts3Expr;
            let mut ii: i32 = 0 as i32;
            let mut pTC: *mut Fts3TokenAndCost = aTC;
            let mut ppOr: *mut *mut Fts3Expr = apOr;
            fts3EvalTokenCosts(
                pCsr,
                std::ptr::null_mut::<Fts3Expr>(),
                unsafe { (*pCsr).pExpr },
                std::ptr::addr_of_mut!(pTC),
                std::ptr::addr_of_mut!(ppOr),
                std::ptr::addr_of_mut!(rc),
            );
            nToken = ((unsafe { pTC.offset_from(aTC as *mut Fts3TokenAndCost) }) as i64) as i32;
            nOr = ((unsafe { ppOr.offset_from(apOr as *mut *mut Fts3Expr) }) as i64) as i32;
            if rc == (0 as i32) {
                rc = fts3EvalSelectDeferred(pCsr, std::ptr::null_mut::<Fts3Expr>(), aTC, nToken);
                ii = 0 as i32;
                '__slate_break_1811: loop {
                    if !(rc == (0 as i32) && ii < nOr) {
                        break;
                    }
                    rc = fts3EvalSelectDeferred(
                        pCsr,
                        unsafe { *unsafe { apOr.offset(ii as isize) } },
                        aTC,
                        nToken,
                    );
                    let __v1958: i32 = ii;
                    let __v1959: i32 = __v1958 + (1 as i32);
                    ii = __v1959;
                }
            }
            unsafe { sqlite3_free(aTC as *mut ()) };
        }
    }
    fts3EvalStartReaders(pCsr, unsafe { (*pCsr).pExpr }, std::ptr::addr_of_mut!(rc));
    return rc;
}

/// Invalidate the current position list for phrase pPhrase.
fn fts3EvalInvalidatePoslist(mut pPhrase: *mut Fts3Phrase) {
    if (unsafe { (*pPhrase).doclist.bFreeList }) != (0 as i32) {
        unsafe { sqlite3_free((unsafe { (*pPhrase).doclist.pList }) as *mut ()) };
    }
    unsafe {
        (*pPhrase).doclist.pList = std::ptr::null_mut::<i8>();
    }
    unsafe {
        (*pPhrase).doclist.nList = 0 as i32;
    }
    unsafe {
        (*pPhrase).doclist.bFreeList = 0 as i32;
    }
}

/// This function is called to edit the position list associated with
/// the phrase object passed as the fifth argument according to a NEAR
/// condition. For example:
///
///     abc NEAR/5 "def ghi"
///
/// Parameter nNear is passed the NEAR distance of the expression (5 in
/// the example above). When this function is called, *paPoslist points to
/// the position list, and *pnToken is the number of phrase tokens in the
/// phrase on the other side of the NEAR operator to pPhrase. For example,
/// if pPhrase refers to the "def ghi" phrase, then *paPoslist points to
/// the position list associated with phrase "abc".
///
/// All positions in the pPhrase position list that are not sufficiently
/// close to a position in the *paPoslist position list are removed. If this
/// leaves 0 positions, zero is returned. Otherwise, non-zero.
///
/// Before returning, *paPoslist is set to point to the position lsit
/// associated with pPhrase. And *pnToken is set to the number of tokens in
/// pPhrase.
///
/// # Arguments
///
/// * `nNear` - NEAR distance. As in "NEAR/nNear".
/// * `aTmp` - Temporary space to use
/// * `paPoslist` - IN/OUT: Position list
/// * `pnToken` - IN/OUT: Tokens in phrase of *paPoslist
/// * `pPhrase` - The phrase object to trim the doclist of
fn fts3EvalNearTrim(
    mut nNear: i32,
    mut aTmp: *mut i8,
    mut paPoslist: *mut *mut i8,
    mut pnToken: *mut i32,
    mut pPhrase: *mut Fts3Phrase,
) -> i32 {
    let mut nParam1: i32 = nNear + unsafe { (*pPhrase).nToken };
    let mut nParam2: i32 = nNear + unsafe { *pnToken };
    let mut nNew: i32 = 0 as i32;
    let mut p2: *mut i8 = unsafe { std::mem::zeroed() };
    let mut pOut: *mut i8 = unsafe { std::mem::zeroed() };
    let mut res: i32 = 0 as i32;
    0 as i32;
    let __v2341: *mut i8 = unsafe { (*pPhrase).doclist.pList };
    pOut = __v2341;
    p2 = __v2341;
    res = fts3PoslistNearMerge(
        std::ptr::addr_of_mut!(pOut),
        aTmp,
        nParam1,
        nParam2,
        paPoslist,
        std::ptr::addr_of_mut!(p2),
    );
    if res != (0 as i32) {
        nNew = (((unsafe { pOut.offset_from((unsafe { (*pPhrase).doclist.pList }) as *mut i8) })
            as i64) as i32)
            - (1 as i32);
        0 as i32;
        if nNew >= (0 as i32) && nNew <= unsafe { (*pPhrase).doclist.nList } {
            0 as i32;
            unsafe {
                memset(
                    (unsafe { unsafe { (*pPhrase).doclist.pList }.offset(nNew as isize) })
                        as *mut (),
                    0 as i32,
                    (((unsafe { (*pPhrase).doclist.nList }) - nNew) as i64) as u64,
                )
            };
            unsafe {
                (*pPhrase).doclist.nList = nNew;
            }
        }
        unsafe {
            *paPoslist = unsafe { (*pPhrase).doclist.pList };
        }
        unsafe {
            *pnToken = unsafe { (*pPhrase).nToken };
        }
    }
    return res;
}

/// This function is a no-op if *pRc is other than SQLITE_OK when it is called.
/// Otherwise, it advances the expression passed as the second argument to
/// point to the next matching row in the database. Expressions iterate through
/// matching rows in docid order. Ascending order if Fts3Cursor.bDesc is zero,
/// or descending if it is non-zero.
///
/// If an error occurs, *pRc is set to an SQLite error code. Otherwise, if
/// successful, the following variables in pExpr are set:
///
///   Fts3Expr.bEof                (non-zero if EOF - there is no next row)
///   Fts3Expr.iDocid              (valid if bEof==0. The docid of the next row)
///
/// If the expression is of type FTSQUERY_PHRASE, and the expression is not
/// at EOF, then the following variables are populated with the position list
/// for the phrase for the visited row:
///
///   FTs3Expr.pPhrase->doclist.nList        (length of pList in bytes)
///   FTs3Expr.pPhrase->doclist.pList        (pointer to position list)
///
/// It says above that this function advances the expression to the next
/// matching row. This is usually true, but there are the following exceptions:
///
///   1. Deferred tokens are not taken into account. If a phrase consists
///      entirely of deferred tokens, it is assumed to match every row in
///      the db. In this case the position-list is not populated at all.
///
///      Or, if a phrase contains one or more deferred tokens and one or
///      more non-deferred tokens, then the expression is advanced to the
///      next possible match, considering only non-deferred tokens. In other
///      words, if the phrase is "A B C", and "B" is deferred, the expression
///      is advanced to the next row that contains an instance of "A * C",
///      where "*" may match any single token. The position list in this case
///      is populated as for "A * C" before returning.
///
///   2. NEAR is treated as AND. If the expression is "x NEAR y", it is
///      advanced to point to the next row that matches "x AND y".
///
/// See sqlite3Fts3EvalTestDeferred() for details on testing if a row is
/// really a match, taking into account deferred tokens and NEAR operators.
///
/// # Arguments
///
/// * `pCsr` - FTS Cursor handle
/// * `pExpr` - Expr. to advance to next matching row
/// * `pRc` - IN/OUT: Error code
fn fts3EvalNextRow(mut pCsr: *mut Fts3Cursor, mut pExpr: *mut Fts3Expr, mut pRc: *mut i32) {
    if (unsafe { *pRc }) == (0 as i32) && (((unsafe { (*pExpr).bEof }) as u32) as i32) == (0 as i32)
    {
        let mut bDescDoclist: i32 = ((unsafe { (*pCsr).bDesc }) as u32) as i32; // Used by DOCID_CMP() macro
        unsafe {
            (*pExpr).bStart = ((1 as i32) as i8) as u8;
        }
        '__slate_break_1812: {
            match unsafe { (*pExpr).eType } {
                1 | 3 => {
                    let mut pLeft: *mut Fts3Expr = unsafe { (*pExpr).pLeft };
                    let mut pRight: *mut Fts3Expr = unsafe { (*pExpr).pRight };
                    0 as i32;
                    if (unsafe { (*pLeft).bDeferred }) != (0 as u8) {
                        // LHS is entirely deferred. So we assume it matches every row.
                        // Advance the RHS iterator to find the next row visited.
                        fts3EvalNextRow(pCsr, pRight, pRc);
                        unsafe {
                            (*pExpr).iDocid = unsafe { (*pRight).iDocid };
                        }
                        unsafe {
                            (*pExpr).bEof = unsafe { (*pRight).bEof };
                        }
                    } else {
                        if (unsafe { (*pRight).bDeferred }) != (0 as u8) {
                            // RHS is entirely deferred. So we assume it matches every row.
                            // Advance the LHS iterator to find the next row visited.
                            fts3EvalNextRow(pCsr, pLeft, pRc);
                            unsafe {
                                (*pExpr).iDocid = unsafe { (*pLeft).iDocid };
                            }
                            unsafe {
                                (*pExpr).bEof = unsafe { (*pLeft).bEof };
                            }
                        } else {
                            // Neither the RHS or LHS are deferred.
                            fts3EvalNextRow(pCsr, pLeft, pRc);
                            fts3EvalNextRow(pCsr, pRight, pRc);
                            '__slate_break_1813: while !((unsafe { (*pLeft).bEof }) != (0 as u8))
                                && !((unsafe { (*pRight).bEof }) != (0 as u8))
                                && (unsafe { *pRc }) == (0 as i32)
                            {
                                let mut iDiff: i64 = ((if bDescDoclist != (0 as i32) {
                                    -(1 as i32)
                                } else {
                                    1 as i32
                                }) * if (unsafe { (*pLeft).iDocid })
                                    > unsafe { (*pRight).iDocid }
                                {
                                    1 as i32
                                } else {
                                    if (unsafe { (*pLeft).iDocid }) == unsafe { (*pRight).iDocid } {
                                        0 as i32
                                    } else {
                                        -(1 as i32)
                                    }
                                }) as i64;
                                if iDiff == ((0 as i32) as i64) {
                                    break '__slate_break_1813;
                                }
                                if iDiff < ((0 as i32) as i64) {
                                    fts3EvalNextRow(pCsr, pLeft, pRc);
                                } else {
                                    fts3EvalNextRow(pCsr, pRight, pRc);
                                }
                            }
                            unsafe {
                                (*pExpr).iDocid = unsafe { (*pLeft).iDocid };
                            }
                            unsafe {
                                (*pExpr).bEof = ((unsafe { (*pLeft).bEof }) != (0 as u8)
                                    || (unsafe { (*pRight).bEof }) != (0 as u8))
                                    as u8;
                            }
                            if (unsafe { (*pExpr).eType }) == (1 as i32)
                                && (unsafe { (*pExpr).bEof }) != (0 as u8)
                            {
                                0 as i32;
                                if (unsafe { (*unsafe { (*pRight).pPhrase }).doclist.aAll })
                                    != std::ptr::null_mut::<i8>()
                                {
                                    let mut pDl: *mut Fts3Doclist = unsafe {
                                        std::ptr::addr_of_mut!(
                                            (*unsafe { (*pRight).pPhrase }).doclist
                                        )
                                    };
                                    '__slate_break_1814: while (unsafe { *pRc }) == (0 as i32)
                                        && (((unsafe { (*pRight).bEof }) as u32) as i32)
                                            == (0 as i32)
                                    {
                                        unsafe {
                                            memset(
                                                (unsafe { (*pDl).pList }) as *mut (),
                                                0 as i32,
                                                ((unsafe { (*pDl).nList }) as i64) as u64,
                                            )
                                        };
                                        fts3EvalNextRow(pCsr, pRight, pRc);
                                    }
                                }
                                if (unsafe { (*pLeft).pPhrase })
                                    != std::ptr::null_mut::<Fts3Phrase>()
                                    && (unsafe { (*unsafe { (*pLeft).pPhrase }).doclist.aAll })
                                        != std::ptr::null_mut::<i8>()
                                {
                                    let mut pDl: *mut Fts3Doclist = unsafe {
                                        std::ptr::addr_of_mut!(
                                            (*unsafe { (*pLeft).pPhrase }).doclist
                                        )
                                    };
                                    '__slate_break_1815: while (unsafe { *pRc }) == (0 as i32)
                                        && (((unsafe { (*pLeft).bEof }) as u32) as i32)
                                            == (0 as i32)
                                    {
                                        unsafe {
                                            memset(
                                                (unsafe { (*pDl).pList }) as *mut (),
                                                0 as i32,
                                                ((unsafe { (*pDl).nList }) as i64) as u64,
                                            )
                                        };
                                        fts3EvalNextRow(pCsr, pLeft, pRc);
                                    }
                                }
                                unsafe {
                                    (*pLeft).bEof = ((1 as i32) as i8) as u8;
                                }
                                unsafe {
                                    (*pRight).bEof = ((1 as i32) as i8) as u8;
                                }
                            }
                        }
                    }
                }
                4 => {
                    let mut pLeft: *mut Fts3Expr = unsafe { (*pExpr).pLeft };
                    let mut pRight: *mut Fts3Expr = unsafe { (*pExpr).pRight };
                    let mut iCmp: i64 =
                        ((if bDescDoclist != (0 as i32) {
                            -(1 as i32)
                        } else {
                            1 as i32
                        }) * if (unsafe { (*pLeft).iDocid }) > unsafe { (*pRight).iDocid } {
                            1 as i32
                        } else {
                            if (unsafe { (*pLeft).iDocid }) == unsafe { (*pRight).iDocid } {
                                0 as i32
                            } else {
                                -(1 as i32)
                            }
                        }) as i64;
                    0 as i32;
                    0 as i32;
                    if (unsafe { (*pRight).bEof }) != (0 as u8)
                        || (((unsafe { (*pLeft).bEof }) as u32) as i32) == (0 as i32)
                            && iCmp < ((0 as i32) as i64)
                    {
                        fts3EvalNextRow(pCsr, pLeft, pRc);
                    } else {
                        if (unsafe { (*pLeft).bEof }) != (0 as u8) || iCmp > ((0 as i32) as i64) {
                            fts3EvalNextRow(pCsr, pRight, pRc);
                        } else {
                            fts3EvalNextRow(pCsr, pLeft, pRc);
                            fts3EvalNextRow(pCsr, pRight, pRc);
                        }
                    }
                    unsafe {
                        (*pExpr).bEof = ((unsafe { (*pLeft).bEof }) != (0 as u8)
                            && (unsafe { (*pRight).bEof }) != (0 as u8))
                            as u8;
                    }
                    iCmp = ((if bDescDoclist != (0 as i32) {
                        -(1 as i32)
                    } else {
                        1 as i32
                    }) * if (unsafe { (*pLeft).iDocid }) > unsafe { (*pRight).iDocid } {
                        1 as i32
                    } else {
                        if (unsafe { (*pLeft).iDocid }) == unsafe { (*pRight).iDocid } {
                            0 as i32
                        } else {
                            -(1 as i32)
                        }
                    }) as i64;
                    if (unsafe { (*pRight).bEof }) != (0 as u8)
                        || (((unsafe { (*pLeft).bEof }) as u32) as i32) == (0 as i32)
                            && iCmp < ((0 as i32) as i64)
                    {
                        unsafe {
                            (*pExpr).iDocid = unsafe { (*pLeft).iDocid };
                        }
                    } else {
                        unsafe {
                            (*pExpr).iDocid = unsafe { (*pRight).iDocid };
                        }
                    }
                }
                2 => {
                    let mut pLeft: *mut Fts3Expr = unsafe { (*pExpr).pLeft };
                    let mut pRight: *mut Fts3Expr = unsafe { (*pExpr).pRight };
                    if (((unsafe { (*pRight).bStart }) as u32) as i32) == (0 as i32) {
                        fts3EvalNextRow(pCsr, pRight, pRc);
                        0 as i32;
                    }
                    fts3EvalNextRow(pCsr, pLeft, pRc);
                    if (((unsafe { (*pLeft).bEof }) as u32) as i32) == (0 as i32) {
                        '__slate_break_1816: while !((unsafe { *pRc }) != (0 as i32))
                            && !((unsafe { (*pRight).bEof }) != (0 as u8))
                            && (if bDescDoclist != (0 as i32) {
                                -(1 as i32)
                            } else {
                                1 as i32
                            }) * if (unsafe { (*pLeft).iDocid }) > unsafe { (*pRight).iDocid } {
                                1 as i32
                            } else {
                                if (unsafe { (*pLeft).iDocid }) == unsafe { (*pRight).iDocid } {
                                    0 as i32
                                } else {
                                    -(1 as i32)
                                }
                            } > (0 as i32)
                        {
                            fts3EvalNextRow(pCsr, pRight, pRc);
                        }
                    }
                    unsafe {
                        (*pExpr).iDocid = unsafe { (*pLeft).iDocid };
                    }
                    unsafe {
                        (*pExpr).bEof = unsafe { (*pLeft).bEof };
                    }
                }
                _ => {
                    let mut pPhrase: *mut Fts3Phrase = unsafe { (*pExpr).pPhrase };
                    fts3EvalInvalidatePoslist(pPhrase);
                    unsafe {
                        *pRc = fts3EvalPhraseNext(pCsr, pPhrase, unsafe {
                            std::ptr::addr_of_mut!((*pExpr).bEof)
                        });
                    }
                    unsafe {
                        (*pExpr).iDocid = unsafe { (*pPhrase).doclist.iDocid };
                    }
                }
            }
        }
    }
}

/// If *pRc is not SQLITE_OK, or if pExpr is not the root node of a NEAR
/// cluster, then this function returns 1 immediately.
///
/// Otherwise, it checks if the current row really does match the NEAR
/// expression, using the data currently stored in the position lists
/// (Fts3Expr->pPhrase.doclist.pList/nList) for each phrase in the expression.
///
/// If the current row is a match, the position list associated with each
/// phrase in the NEAR expression is edited in place to contain only those
/// phrase instances sufficiently close to their peers to satisfy all NEAR
/// constraints. In this case it returns 1. If the NEAR expression does not
/// match the current row, 0 is returned. The position lists may or may not
/// be edited if 0 is returned.
fn fts3EvalNearTest(mut pExpr: *mut Fts3Expr, mut pRc: *mut i32) -> i32 {
    let mut res: i32 = 1 as i32;
    // The following block runs if pExpr is the root of a NEAR query.
    // For example, the query:
    //
    //         "w" NEAR "x" NEAR "y" NEAR "z"
    //
    // which is represented in tree form as:
    //
    //                               |
    //                          +--NEAR--+      <-- root of NEAR query
    //                          |        |
    //                     +--NEAR--+   "z"
    //                     |        |
    //                +--NEAR--+   "y"
    //                |        |
    //               "w"      "x"
    //
    // The right-hand child of a NEAR node is always a phrase. The
    // left-hand child may be either a phrase or a NEAR node. There are
    // no exceptions to this - it's the way the parser in fts3_expr.c works.
    if (unsafe { *pRc }) == (0 as i32)
        && (unsafe { (*pExpr).eType }) == (1 as i32)
        && ((unsafe { (*pExpr).pParent }) == std::ptr::null_mut::<Fts3Expr>()
            || (unsafe { (*unsafe { (*pExpr).pParent }).eType }) != (1 as i32))
    {
        let mut p: *mut Fts3Expr = unsafe { std::mem::zeroed() };
        let mut nTmp: i64 = (0 as i32) as i64; // Bytes of temp space
        let mut aTmp: *mut i8 = unsafe { std::mem::zeroed() }; // Temp space for PoslistNearMerge()
        // Allocate temporary working space.
        p = pExpr;
        '__slate_break_1817: while (unsafe { (*p).pLeft }) != std::ptr::null_mut::<Fts3Expr>() {
            0 as i32;
            let __v2342: i64 = nTmp;
            let __v2343: i64 = __v2342
                + ((unsafe {
                    (*unsafe { (*unsafe { (*p).pRight }).pPhrase })
                        .doclist
                        .nList
                }) as i64);
            nTmp = __v2343;
            p = unsafe { (*p).pLeft };
        }
        let __v2344: i64 = nTmp;
        let __v2345: i64 = __v2344 + ((unsafe { (*unsafe { (*p).pPhrase }).doclist.nList }) as i64);
        nTmp = __v2345;
        aTmp = (unsafe {
            sqlite3_malloc64((nTmp * ((2 as i32) as i64) + ((10 as i32) as i64)) as u64)
        }) as *mut i8;
        if !(aTmp != std::ptr::null_mut::<i8>()) {
            unsafe {
                *pRc = 7 as i32;
            }
            res = 0 as i32;
        } else {
            let mut aPoslist: *mut i8 = unsafe { (*unsafe { (*p).pPhrase }).doclist.pList };
            let mut nToken: i32 = unsafe { (*unsafe { (*p).pPhrase }).nToken };
            p = unsafe { (*p).pParent };
            '__slate_break_1818: while res != (0 as i32)
                && p != std::ptr::null_mut::<Fts3Expr>()
                && (unsafe { (*p).eType }) == (1 as i32)
            {
                let mut pPhrase: *mut Fts3Phrase = unsafe { (*unsafe { (*p).pRight }).pPhrase };
                let mut nNear: i32 = unsafe { (*p).nNear };
                res = fts3EvalNearTrim(
                    nNear,
                    aTmp,
                    std::ptr::addr_of_mut!(aPoslist),
                    std::ptr::addr_of_mut!(nToken),
                    pPhrase,
                );
                p = unsafe { (*p).pParent };
            }
            aPoslist = unsafe {
                (*unsafe { (*unsafe { (*pExpr).pRight }).pPhrase })
                    .doclist
                    .pList
            };
            nToken = unsafe { (*unsafe { (*unsafe { (*pExpr).pRight }).pPhrase }).nToken };
            p = unsafe { (*pExpr).pLeft };
            '__slate_break_1819: while p != std::ptr::null_mut::<Fts3Expr>() && res != (0 as i32) {
                let mut nNear: i32 = 0 as i32;
                let mut pPhrase: *mut Fts3Phrase = unsafe { std::mem::zeroed() };
                0 as i32;
                nNear = unsafe { (*unsafe { (*p).pParent }).nNear };
                pPhrase = if (unsafe { (*p).eType }) == (1 as i32) {
                    unsafe { (*unsafe { (*p).pRight }).pPhrase }
                } else {
                    unsafe { (*p).pPhrase }
                };
                res = fts3EvalNearTrim(
                    nNear,
                    aTmp,
                    std::ptr::addr_of_mut!(aPoslist),
                    std::ptr::addr_of_mut!(nToken),
                    pPhrase,
                );
                p = unsafe { (*p).pLeft };
            }
        }
        unsafe { sqlite3_free(aTmp as *mut ()) };
    }
    return res;
}

/// This function is a helper function for sqlite3Fts3EvalTestDeferred().
/// Assuming no error occurs or has occurred, It returns non-zero if the
/// expression passed as the second argument matches the row that pCsr
/// currently points to, or zero if it does not.
///
/// If *pRc is not SQLITE_OK when this function is called, it is a no-op.
/// If an error occurs during execution of this function, *pRc is set to
/// the appropriate SQLite error code. In this case the returned value is
/// undefined.
///
/// # Arguments
///
/// * `pCsr` - FTS cursor handle
/// * `pExpr` - Expr to test. May or may not be root.
/// * `pRc` - IN/OUT: Error code
fn fts3EvalTestExpr(mut pCsr: *mut Fts3Cursor, mut pExpr: *mut Fts3Expr, mut pRc: *mut i32) -> i32 {
    let mut bHit: i32 = 1 as i32; // Return value
    if (unsafe { *pRc }) == (0 as i32) {
        '__slate_break_1820: {
            match unsafe { (*pExpr).eType } {
                1 | 3 => {
                    let __v2346: bool;
                    if fts3EvalTestExpr(pCsr, unsafe { (*pExpr).pLeft }, pRc) != (0 as i32) {
                        __v2346 =
                            fts3EvalTestExpr(pCsr, unsafe { (*pExpr).pRight }, pRc) != (0 as i32);
                    } else {
                        __v2346 = false as bool;
                    }
                    let __v2347: bool;
                    if __v2346 {
                        __v2347 = fts3EvalNearTest(pExpr, pRc) != (0 as i32);
                    } else {
                        __v2347 = false as bool;
                    }
                    bHit = __v2347 as i32;
                    // If the NEAR expression does not match any rows, zero the doclist for
                    // all phrases involved in the NEAR. This is because the snippet(),
                    // offsets() and matchinfo() functions are not supposed to recognize
                    // any instances of phrases that are part of unmatched NEAR queries.
                    // For example if this expression:
                    //
                    //    ... MATCH 'a OR (b NEAR c)'
                    //
                    // is matched against a row containing:
                    //
                    //        'a b d e'
                    //
                    // then any snippet() should ony highlight the "a" term, not the "b"
                    // (as "b" is part of a non-matching NEAR clause).
                    if bHit == (0 as i32)
                        && (unsafe { (*pExpr).eType }) == (1 as i32)
                        && ((unsafe { (*pExpr).pParent }) == std::ptr::null_mut::<Fts3Expr>()
                            || (unsafe { (*unsafe { (*pExpr).pParent }).eType }) != (1 as i32))
                    {
                        let mut p: *mut Fts3Expr = unsafe { std::mem::zeroed() };
                        p = pExpr;
                        '__slate_break_1821: while (unsafe { (*p).pPhrase })
                            == std::ptr::null_mut::<Fts3Phrase>()
                        {
                            if (unsafe { (*unsafe { (*p).pRight }).iDocid })
                                == unsafe { (*pCsr).iPrevId }
                            {
                                fts3EvalInvalidatePoslist(unsafe {
                                    (*unsafe { (*p).pRight }).pPhrase
                                });
                            }
                            p = unsafe { (*p).pLeft };
                        }
                        if (unsafe { (*p).iDocid }) == unsafe { (*pCsr).iPrevId } {
                            fts3EvalInvalidatePoslist(unsafe { (*p).pPhrase });
                        }
                    }
                }
                4 => {
                    let mut bHit1: i32 = fts3EvalTestExpr(pCsr, unsafe { (*pExpr).pLeft }, pRc);
                    let mut bHit2: i32 = fts3EvalTestExpr(pCsr, unsafe { (*pExpr).pRight }, pRc);
                    bHit = (bHit1 != (0 as i32) || bHit2 != (0 as i32)) as i32;
                }
                2 => {
                    let __v2348: bool;
                    if fts3EvalTestExpr(pCsr, unsafe { (*pExpr).pLeft }, pRc) != (0 as i32) {
                        __v2348 = !(fts3EvalTestExpr(pCsr, unsafe { (*pExpr).pRight }, pRc)
                            != (0 as i32));
                    } else {
                        __v2348 = false as bool;
                    }
                    bHit = __v2348 as i32;
                }
                _ => {
                    if (unsafe { (*pCsr).pDeferred }) != std::ptr::null_mut::<Fts3DeferredToken>()
                        && ((unsafe { (*pExpr).bDeferred }) != (0 as u8)
                            || (unsafe { (*pExpr).iDocid }) == unsafe { (*pCsr).iPrevId }
                                && (unsafe { (*unsafe { (*pExpr).pPhrase }).doclist.pList })
                                    != std::ptr::null_mut::<i8>())
                    {
                        let mut pPhrase: *mut Fts3Phrase = unsafe { (*pExpr).pPhrase };
                        if (unsafe { (*pExpr).bDeferred }) != (0 as u8) {
                            fts3EvalInvalidatePoslist(pPhrase);
                        }
                        unsafe {
                            *pRc = fts3EvalDeferredPhrase(pCsr, pPhrase);
                        }
                        bHit = ((unsafe { (*pPhrase).doclist.pList }) != std::ptr::null_mut::<i8>())
                            as i32;
                        unsafe {
                            (*pExpr).iDocid = unsafe { (*pCsr).iPrevId };
                        }
                    } else {
                        bHit = ((((unsafe { (*pExpr).bEof }) as u32) as i32) == (0 as i32)
                            && (unsafe { (*pExpr).iDocid }) == unsafe { (*pCsr).iPrevId }
                            && (unsafe { (*unsafe { (*pExpr).pPhrase }).doclist.nList })
                                > (0 as i32)) as i32;
                    }
                }
            }
        }
    }
    return bHit;
}

/// This function is called as the second part of each xNext operation when
/// iterating through the results of a full-text query. At this point the
/// cursor points to a row that matches the query expression, with the
/// following caveats:
///
///   * Up until this point, "NEAR" operators in the expression have been
///     treated as "AND".
///
///   * Deferred tokens have not yet been considered.
///
/// If *pRc is not SQLITE_OK when this function is called, it immediately
/// returns 0. Otherwise, it tests whether or not after considering NEAR
/// operators and deferred tokens the current row is still a match for the
/// expression. It returns 1 if both of the following are true:
///
///   1. *pRc is SQLITE_OK when this function returns, and
///
///   2. After scanning the current FTS table row for the deferred tokens,
///      it is determined that the row does *not* match the query.
///
/// Or, if no error occurs and it seems the current row does match the FTS
/// query, return 0.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3Fts3EvalTestDeferred(
    mut pCsr: *mut Fts3Cursor,
    mut pRc: *mut i32,
) -> i32 {
    let mut rc: i32 = unsafe { *pRc };
    let mut bMiss: i32 = 0 as i32;
    if rc == (0 as i32) {
        // If there are one or more deferred tokens, load the current row into
        // memory and scan it to determine the position list for each deferred
        // token. Then, see if this row is really a match, considering deferred
        // tokens and NEAR operators (neither of which were taken into account
        // earlier, by fts3EvalNextRow()).
        if (unsafe { (*pCsr).pDeferred }) != std::ptr::null_mut::<Fts3DeferredToken>() {
            rc = fts3CursorSeek(std::ptr::null_mut::<sqlite3_context>(), pCsr);
            if rc == (0 as i32) {
                rc = unsafe { sqlite3Fts3CacheDeferredDoclists(pCsr) };
            }
        }
        bMiss = ((0 as i32)
            == fts3EvalTestExpr(pCsr, unsafe { (*pCsr).pExpr }, std::ptr::addr_of_mut!(rc)))
            as i32;
        // Free the position-lists accumulated for each deferred token above.
        unsafe { sqlite3Fts3FreeDeferredDoclists(pCsr) };
        unsafe {
            *pRc = rc;
        }
    }
    return (rc == (0 as i32) && bMiss != (0 as i32)) as i32;
}

/// Advance to the next document that matches the FTS expression in
/// Fts3Cursor.pExpr.
fn fts3EvalNext(mut pCsr: *mut Fts3Cursor) -> i32 {
    let mut rc: i32 = 0 as i32; // Return Code
    let mut pExpr: *mut Fts3Expr = unsafe { (*pCsr).pExpr };
    0 as i32;
    if pExpr == std::ptr::null_mut::<Fts3Expr>() {
        unsafe {
            (*pCsr).isEof = ((1 as i32) as i8) as u8;
        }
    } else {
        '__slate_break_1822: loop {
            if (((unsafe { (*pCsr).isRequireSeek }) as u32) as i32) == (0 as i32) {
                unsafe { sqlite3_reset(unsafe { (*pCsr).pStmt }) };
            }
            0 as i32;
            fts3EvalNextRow(pCsr, pExpr, std::ptr::addr_of_mut!(rc));
            unsafe {
                (*pCsr).isEof = unsafe { (*pExpr).bEof };
            }
            unsafe {
                (*pCsr).isRequireSeek = ((1 as i32) as i8) as u8;
            }
            unsafe {
                (*pCsr).isMatchinfoNeeded = 1 as i32;
            }
            unsafe {
                (*pCsr).iPrevId = unsafe { (*pExpr).iDocid };
            }
            let __v1957: bool;
            if (((unsafe { (*pCsr).isEof }) as u32) as i32) == (0 as i32) {
                __v1957 =
                    sqlite3Fts3EvalTestDeferred(pCsr, std::ptr::addr_of_mut!(rc)) != (0 as i32);
            } else {
                __v1957 = false as bool;
            }
            if !__v1957 {
                break;
            }
        }
    }
    // Check if the cursor is past the end of the docid range specified
    // by Fts3Cursor.iMinDocid/iMaxDocid. If so, set the EOF flag.
    if rc == (0 as i32)
        && ((((unsafe { (*pCsr).bDesc }) as u32) as i32) == (0 as i32)
            && (unsafe { (*pCsr).iPrevId }) > unsafe { (*pCsr).iMaxDocid }
            || (((unsafe { (*pCsr).bDesc }) as u32) as i32) != (0 as i32)
                && (unsafe { (*pCsr).iPrevId }) < unsafe { (*pCsr).iMinDocid })
    {
        unsafe {
            (*pCsr).isEof = ((1 as i32) as i8) as u8;
        }
    }
    return rc;
}

/// Restart iteration for expression pExpr so that the next call to
/// fts3EvalNext() visits the first row. Do not allow incremental
/// loading or merging of phrase doclists for this iteration.
///
/// If *pRc is other than SQLITE_OK when this function is called, it is
/// a no-op. If an error occurs within this function, *pRc is set to an
/// SQLite error code before returning.
fn fts3EvalRestart(mut pCsr: *mut Fts3Cursor, mut pExpr: *mut Fts3Expr, mut pRc: *mut i32) {
    if pExpr != std::ptr::null_mut::<Fts3Expr>() && (unsafe { *pRc }) == (0 as i32) {
        let mut pPhrase: *mut Fts3Phrase = unsafe { (*pExpr).pPhrase };
        if pPhrase != std::ptr::null_mut::<Fts3Phrase>() {
            fts3EvalInvalidatePoslist(pPhrase);
            if (unsafe { (*pPhrase).bIncr }) != (0 as i32) {
                let mut i: i32 = 0 as i32;
                i = 0 as i32;
                '__slate_break_1823: loop {
                    if !(i < unsafe { (*pPhrase).nToken }) {
                        break;
                    }
                    let mut pToken: *mut Fts3PhraseToken = unsafe {
                        unsafe { std::ptr::addr_of_mut!((*pPhrase).aToken) as *mut Fts3PhraseToken }
                            .offset(i as isize)
                    };
                    0 as i32;
                    if (unsafe { (*pToken).pSegcsr }) != std::ptr::null_mut::<Fts3MultiSegReader>()
                    {
                        unsafe { sqlite3Fts3MsrIncrRestart(unsafe { (*pToken).pSegcsr }) };
                    }
                    let __v2349: i32 = i;
                    let __v2350: i32 = __v2349 + (1 as i32);
                    i = __v2350;
                }
                unsafe {
                    *pRc = fts3EvalPhraseStart(pCsr, 0 as i32, pPhrase);
                }
            }
            unsafe {
                (*pPhrase).doclist.pNextDocid = std::ptr::null_mut::<i8>();
            }
            unsafe {
                (*pPhrase).doclist.iDocid = (0 as i32) as i64;
            }
            unsafe {
                (*pPhrase).pOrPoslist = std::ptr::null_mut::<i8>();
            }
        }
        unsafe {
            (*pExpr).iDocid = (0 as i32) as i64;
        }
        unsafe {
            (*pExpr).bEof = ((0 as i32) as i8) as u8;
        }
        unsafe {
            (*pExpr).bStart = ((0 as i32) as i8) as u8;
        }
        fts3EvalRestart(pCsr, unsafe { (*pExpr).pLeft }, pRc);
        fts3EvalRestart(pCsr, unsafe { (*pExpr).pRight }, pRc);
    }
}

/// Expression node pExpr is an MSR phrase. This function restarts pExpr
/// so that it is a regular phrase query, not an MSR. SQLITE_OK is returned
/// if successful, or an SQLite error code otherwise.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3Fts3MsrCancel(
    mut pCsr: *mut Fts3Cursor,
    mut pExpr: *mut Fts3Expr,
) -> i32 {
    let mut rc: i32 = 0 as i32;
    if (((unsafe { (*pExpr).bEof }) as u32) as i32) == (0 as i32) {
        let mut iDocid: i64 = unsafe { (*pExpr).iDocid };
        fts3EvalRestart(pCsr, pExpr, std::ptr::addr_of_mut!(rc));
        '__slate_break_1824: while rc == (0 as i32) && (unsafe { (*pExpr).iDocid }) != iDocid {
            fts3EvalNextRow(pCsr, pExpr, std::ptr::addr_of_mut!(rc));
            if (unsafe { (*pExpr).bEof }) != (0 as u8) {
                rc = (11 as i32) | (1 as i32) << (8 as i32);
            }
        }
    }
    return rc;
}

/// After allocating the Fts3Expr.aMI[] array for each phrase in the
/// expression rooted at pExpr, the cursor iterates through all rows matched
/// by pExpr, calling this function for each row. This function increments
/// the values in Fts3Expr.aMI[] according to the position-list currently
/// found in Fts3Expr.pPhrase->doclist.pList for each of the phrase
/// expression nodes.
fn fts3EvalUpdateCounts(mut pExpr: *mut Fts3Expr, mut nCol: i32) {
    if pExpr != std::ptr::null_mut::<Fts3Expr>() {
        let mut pPhrase: *mut Fts3Phrase = unsafe { (*pExpr).pPhrase };
        if pPhrase != std::ptr::null_mut::<Fts3Phrase>()
            && (unsafe { (*pPhrase).doclist.pList }) != std::ptr::null_mut::<i8>()
        {
            let mut iCol: i32 = 0 as i32;
            let mut p: *mut i8 = unsafe { (*pPhrase).doclist.pList };
            '__slate_break_1825: loop {
                let mut c: u8 = ((0 as i32) as i8) as u8;
                let mut iCnt: i32 = 0 as i32;
                '__slate_break_1826: while (254 as i32)
                    & (((unsafe { *p }) as i32) | ((c as u32) as i32))
                    != (0 as i32)
                {
                    if ((c as u32) as i32) & (128 as i32) == (0 as i32) {
                        let __v2351: i32 = iCnt;
                        let __v2352: i32 = __v2351 + (1 as i32);
                        iCnt = __v2352;
                    }
                    let __v2353: *mut i8 = p;
                    let __v2354: *mut i8 = unsafe { __v2353.offset((1 as i32) as isize) };
                    p = __v2354;
                    c = ((((unsafe { *__v2353 }) as i32) & (128 as i32)) as i8) as u8;
                }
                // aMI[iCol*3 + 1] = Number of occurrences
                // aMI[iCol*3 + 2] = Number of rows containing at least one instance
                let __v2355: *mut u32 = unsafe {
                    unsafe { (*pExpr).aMI }.offset((iCol * (3 as i32) + (1 as i32)) as isize)
                };
                let __v2356: u32 = unsafe { *__v2355 };
                let __v2357: u32 = __v2356.wrapping_add(iCnt as u32);
                unsafe {
                    *__v2355 = __v2357;
                }
                let __v2358: *mut u32 = unsafe {
                    unsafe { (*pExpr).aMI }.offset((iCol * (3 as i32) + (2 as i32)) as isize)
                };
                let __v2359: u32 = unsafe { *__v2358 };
                let __v2360: u32 = __v2359.wrapping_add(((iCnt > (0 as i32)) as i32) as u32);
                unsafe {
                    *__v2358 = __v2360;
                }
                if ((unsafe { *p }) as i32) == (0 as i32) {
                    break '__slate_break_1825;
                }
                let __v2361: *mut i8 = p;
                let __v2362: *mut i8 = unsafe { __v2361.offset((1 as i32) as isize) };
                p = __v2362;
                let __v2363: *mut i8 = p;
                let __v2364: i32;
                if (((unsafe { *(p as *mut u8) }) as u32) as i32) & (128 as i32) != (0 as i32) {
                    __v2364 = sqlite3Fts3GetVarint32(p as *const i8, std::ptr::addr_of_mut!(iCol));
                } else {
                    unsafe {
                        *std::ptr::addr_of_mut!(iCol) =
                            ((unsafe { *(p as *mut u8) }) as u32) as i32;
                    }
                    __v2364 = 1 as i32;
                }
                let __v2365: *mut i8 = unsafe { __v2363.offset(__v2364 as isize) };
                p = __v2365;
                if !(iCol < nCol) {
                    break;
                }
            }
        }
        fts3EvalUpdateCounts(unsafe { (*pExpr).pLeft }, nCol);
        fts3EvalUpdateCounts(unsafe { (*pExpr).pRight }, nCol);
    }
}

/// This is an sqlite3Fts3ExprIterate() callback. If the Fts3Expr.aMI[] array
/// has not yet been allocated, allocate and zero it. Otherwise, just zero
/// it.
#[unsafe(link_section = ".text.slate_distinct.fts3.fts3AllocateMSI")]
extern "C-unwind" fn fts3AllocateMSI(
    mut pExpr: *mut Fts3Expr,
    mut iPhrase: i32,
    mut pCtx: *mut (),
) -> i32 {
    let mut pTab: *mut Fts3Table = pCtx as *mut Fts3Table;
    iPhrase;
    if (unsafe { (*pExpr).aMI }) == std::ptr::null_mut::<u32>() {
        unsafe {
            (*pExpr).aMI = (unsafe {
                sqlite3_malloc64(
                    ((((unsafe { (*pTab).nColumn }) * (3 as i32)) as i64) as u64)
                        .wrapping_mul(4 as u64),
                )
            }) as *mut u32;
        }
        if (unsafe { (*pExpr).aMI }) == std::ptr::null_mut::<u32>() {
            return 7 as i32;
        }
    }
    unsafe {
        memset(
            (unsafe { (*pExpr).aMI }) as *mut (),
            0 as i32,
            ((((unsafe { (*pTab).nColumn }) * (3 as i32)) as i64) as u64).wrapping_mul(4 as u64),
        )
    };
    return 0 as i32;
}

/// Expression pExpr must be of type FTSQUERY_PHRASE.
///
/// If it is not already allocated and populated, this function allocates and
/// populates the Fts3Expr.aMI[] array for expression pExpr. If pExpr is part
/// of a NEAR expression, then it also allocates and populates the same array
/// for all other phrases that are part of the NEAR expression.
///
/// SQLITE_OK is returned if the aMI[] array is successfully allocated and
/// populated. Otherwise, if an error occurs, an SQLite error code is returned.
///
/// # Arguments
///
/// * `pCsr` - Cursor object
/// * `pExpr` - FTSQUERY_PHRASE expression
fn fts3EvalGatherStats(mut pCsr: *mut Fts3Cursor, mut pExpr: *mut Fts3Expr) -> i32 {
    let mut rc: i32 = 0 as i32; // Return code
    0 as i32;
    if (unsafe { (*pExpr).aMI }) == std::ptr::null_mut::<u32>() {
        let mut pTab: *mut Fts3Table = (unsafe { (*pCsr).base.pVtab }) as *mut Fts3Table;
        let mut pRoot: *mut Fts3Expr = unsafe { std::mem::zeroed() }; // Root of NEAR expression
        let mut iPrevId: i64 = unsafe { (*pCsr).iPrevId };
        let mut iDocid: i64 = 0 as i64;
        let mut bEof: u8 = 0 as u8;
        // Find the root of the NEAR expression
        pRoot = pExpr;
        '__slate_break_1827: while (unsafe { (*pRoot).pParent }) != std::ptr::null_mut::<Fts3Expr>()
            && ((unsafe { (*unsafe { (*pRoot).pParent }).eType }) == (1 as i32)
                || (unsafe { (*pRoot).bDeferred }) != (0 as u8))
        {
            pRoot = unsafe { (*pRoot).pParent };
        }
        iDocid = unsafe { (*pRoot).iDocid };
        bEof = unsafe { (*pRoot).bEof };
        0 as i32;
        // Allocate space for the aMSI[] array of each FTSQUERY_PHRASE node
        rc = unsafe { sqlite3Fts3ExprIterate(pRoot, Some(fts3AllocateMSI), pTab as *mut ()) };
        if rc != (0 as i32) {
            return rc;
        }
        fts3EvalRestart(pCsr, pRoot, std::ptr::addr_of_mut!(rc));
        '__slate_break_1828: while (((unsafe { (*pCsr).isEof }) as u32) as i32) == (0 as i32)
            && rc == (0 as i32)
        {
            '__slate_break_1829: loop {
                // Ensure the %_content statement is reset.
                if (((unsafe { (*pCsr).isRequireSeek }) as u32) as i32) == (0 as i32) {
                    unsafe { sqlite3_reset(unsafe { (*pCsr).pStmt }) };
                }
                0 as i32;
                // Advance to the next document
                fts3EvalNextRow(pCsr, pRoot, std::ptr::addr_of_mut!(rc));
                unsafe {
                    (*pCsr).isEof = unsafe { (*pRoot).bEof };
                }
                unsafe {
                    (*pCsr).isRequireSeek = ((1 as i32) as i8) as u8;
                }
                unsafe {
                    (*pCsr).isMatchinfoNeeded = 1 as i32;
                }
                unsafe {
                    (*pCsr).iPrevId = unsafe { (*pRoot).iDocid };
                }
                let __v2366: bool;
                if (((unsafe { (*pCsr).isEof }) as u32) as i32) == (0 as i32)
                    && (unsafe { (*pRoot).eType }) == (1 as i32)
                {
                    __v2366 =
                        sqlite3Fts3EvalTestDeferred(pCsr, std::ptr::addr_of_mut!(rc)) != (0 as i32);
                } else {
                    __v2366 = false as bool;
                }
                if !__v2366 {
                    break;
                }
            }
            if rc == (0 as i32) && (((unsafe { (*pCsr).isEof }) as u32) as i32) == (0 as i32) {
                fts3EvalUpdateCounts(pRoot, unsafe { (*pTab).nColumn });
            }
        }
        unsafe {
            (*pCsr).isEof = ((0 as i32) as i8) as u8;
        }
        unsafe {
            (*pCsr).iPrevId = iPrevId;
        }
        if bEof != (0 as u8) {
            unsafe {
                (*pRoot).bEof = bEof;
            }
        } else {
            // Caution: pRoot may iterate through docids in ascending or descending
            // order. For this reason, even though it seems more defensive, the
            // do loop can not be written:
            //
            //   do {...} while( pRoot->iDocid<iDocid && rc==SQLITE_OK );
            fts3EvalRestart(pCsr, pRoot, std::ptr::addr_of_mut!(rc));
            '__slate_break_1830: loop {
                fts3EvalNextRow(pCsr, pRoot, std::ptr::addr_of_mut!(rc));
                0 as i32;
                if (unsafe { (*pRoot).bEof }) != (0 as u8) {
                    rc = (11 as i32) | (1 as i32) << (8 as i32);
                }
                if !((unsafe { (*pRoot).iDocid }) != iDocid && rc == (0 as i32)) {
                    break;
                }
            }
        }
    }
    return rc;
}

/// This function is used by the matchinfo() module to query a phrase
/// expression node for the following information:
///
///   1. The total number of occurrences of the phrase in each column of
///      the FTS table (considering all rows), and
///
///   2. For each column, the number of rows in the table for which the
///      column contains at least one instance of the phrase.
///
/// If no error occurs, SQLITE_OK is returned and the values for each column
/// written into the array aiOut as follows:
///
///   aiOut[iCol*3 + 1] = Number of occurrences
///   aiOut[iCol*3 + 2] = Number of rows containing at least one instance
///
/// Caveats:
///
///   * If a phrase consists entirely of deferred tokens, then all output
///     values are set to the number of documents in the table. In other
///     words we assume that very common tokens occur exactly once in each
///     column of each row of the table.
///
///   * If a phrase contains some deferred tokens (and some non-deferred
///     tokens), count the potential occurrence identified by considering
///     the non-deferred tokens instead of actual phrase occurrences.
///
///   * If the phrase is part of a NEAR expression, then only phrase instances
///     that meet the NEAR constraint are included in the counts.
///
/// # Arguments
///
/// * `pCsr` - FTS cursor handle
/// * `pExpr` - Phrase expression
/// * `aiOut` - Array to write results into (see above)
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3Fts3EvalPhraseStats(
    mut pCsr: *mut Fts3Cursor,
    mut pExpr: *mut Fts3Expr,
    mut aiOut: *mut u32,
) -> i32 {
    let mut pTab: *mut Fts3Table = (unsafe { (*pCsr).base.pVtab }) as *mut Fts3Table;
    let mut rc: i32 = 0 as i32;
    let mut iCol: i32 = 0 as i32;
    if (unsafe { (*pExpr).bDeferred }) != (0 as u8)
        && (unsafe { (*unsafe { (*pExpr).pParent }).eType }) != (1 as i32)
    {
        0 as i32;
        iCol = 0 as i32;
        '__slate_break_1831: loop {
            if !(iCol < unsafe { (*pTab).nColumn }) {
                break;
            }
            unsafe {
                *unsafe { aiOut.offset((iCol * (3 as i32) + (1 as i32)) as isize) } =
                    ((unsafe { (*pCsr).nDoc }) as i32) as u32;
            }
            unsafe {
                *unsafe { aiOut.offset((iCol * (3 as i32) + (2 as i32)) as isize) } =
                    ((unsafe { (*pCsr).nDoc }) as i32) as u32;
            }
            let __v1898: i32 = iCol;
            let __v1899: i32 = __v1898 + (1 as i32);
            iCol = __v1899;
        }
    } else {
        rc = fts3EvalGatherStats(pCsr, pExpr);
        if rc == (0 as i32) {
            0 as i32;
            iCol = 0 as i32;
            '__slate_break_1832: loop {
                if !(iCol < unsafe { (*pTab).nColumn }) {
                    break;
                }
                unsafe {
                    *unsafe { aiOut.offset((iCol * (3 as i32) + (1 as i32)) as isize) } = unsafe {
                        *unsafe {
                            unsafe { (*pExpr).aMI }
                                .offset((iCol * (3 as i32) + (1 as i32)) as isize)
                        }
                    };
                }
                unsafe {
                    *unsafe { aiOut.offset((iCol * (3 as i32) + (2 as i32)) as isize) } = unsafe {
                        *unsafe {
                            unsafe { (*pExpr).aMI }
                                .offset((iCol * (3 as i32) + (2 as i32)) as isize)
                        }
                    };
                }
                let __v1900: i32 = iCol;
                let __v1901: i32 = __v1900 + (1 as i32);
                iCol = __v1901;
            }
        }
    }
    return rc;
}

/// The expression pExpr passed as the second argument to this function
/// must be of type FTSQUERY_PHRASE.
///
/// The returned value is either NULL or a pointer to a buffer containing
/// a position-list indicating the occurrences of the phrase in column iCol
/// of the current row.
///
/// More specifically, the returned buffer contains 1 varint for each
/// occurrence of the phrase in the column, stored using the normal (delta+2)
/// compression and is terminated by either an 0x01 or 0x00 byte. For example,
/// if the requested column contains "a b X c d X X" and the position-list
/// for 'X' is requested, the buffer returned may contain:
///
///     0x04 0x05 0x03 0x01   or   0x04 0x05 0x03 0x00
///
/// This function works regardless of whether or not the phrase is deferred,
/// incremental, or neither.
///
/// # Arguments
///
/// * `pCsr` - FTS3 cursor object
/// * `pExpr` - Phrase to return doclist for
/// * `iCol` - Column to return position list for
/// * `ppOut` - OUT: Pointer to position list
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3Fts3EvalPhrasePoslist(
    mut pCsr: *mut Fts3Cursor,
    mut pExpr: *mut Fts3Expr,
    mut iCol: i32,
    mut ppOut: *mut *mut i8,
) -> i32 {
    let mut pPhrase: *mut Fts3Phrase = unsafe { (*pExpr).pPhrase };
    let mut pTab: *mut Fts3Table = (unsafe { (*pCsr).base.pVtab }) as *mut Fts3Table;
    let mut pIter: *mut i8 = unsafe { std::mem::zeroed() };
    let mut iThis: i32 = 0 as i32;
    let mut iDocid: i64 = 0 as i64;
    // If this phrase is applies specifically to some column other than
    // column iCol, return a NULL pointer.
    unsafe {
        *ppOut = std::ptr::null_mut::<i8>();
    }
    0 as i32;
    if (unsafe { (*pPhrase).iColumn }) < unsafe { (*pTab).nColumn }
        && (unsafe { (*pPhrase).iColumn }) != iCol
    {
        return 0 as i32;
    }
    iDocid = unsafe { (*pExpr).iDocid };
    pIter = unsafe { (*pPhrase).doclist.pList };
    if iDocid != unsafe { (*pCsr).iPrevId } || (unsafe { (*pExpr).bEof }) != (0 as u8) {
        let mut rc: i32 = 0 as i32;
        let mut bDescDoclist: i32 = ((unsafe { (*pTab).bDescIdx }) as u32) as i32; // For DOCID_CMP macro
        let mut bOr: i32 = 0 as i32;
        let mut bTreeEof: u8 = ((0 as i32) as i8) as u8;
        let mut p: *mut Fts3Expr = unsafe { std::mem::zeroed() }; // Used to iterate from pExpr to root
        let mut pNear: *mut Fts3Expr = unsafe { std::mem::zeroed() }; // Most senior NEAR ancestor (or pExpr)
        let mut pRun: *mut Fts3Expr = unsafe { std::mem::zeroed() }; // Closest non-deferred ancestor of pNear
        let mut bMatch: i32 = 0 as i32;
        // Check if this phrase descends from an OR expression node. If not,
        // return NULL. Otherwise, the entry that corresponds to docid
        // pCsr->iPrevId may lie earlier in the doclist buffer. Or, if the
        // tree that the node is part of has been marked as EOF, but the node
        // itself is not EOF, then it may point to an earlier entry.
        pNear = pExpr;
        p = unsafe { (*pExpr).pParent };
        '__slate_break_1833: while p != std::ptr::null_mut::<Fts3Expr>() {
            if (unsafe { (*p).eType }) == (4 as i32) {
                bOr = 1 as i32;
            }
            if (unsafe { (*p).eType }) == (1 as i32) {
                pNear = p;
            }
            if (unsafe { (*p).bEof }) != (0 as u8) {
                bTreeEof = ((1 as i32) as i8) as u8;
            }
            p = unsafe { (*p).pParent };
        }
        if bOr == (0 as i32) {
            return 0 as i32;
        }
        pRun = pNear;
        '__slate_break_1834: while (unsafe { (*pRun).bDeferred }) != (0 as u8) {
            0 as i32;
            pRun = unsafe { (*pRun).pParent };
        }
        // This is the descendent of an OR node. In this case we cannot use
        // an incremental phrase. Load the entire doclist for the phrase
        // into memory in this case.
        if (unsafe { (*pPhrase).bIncr }) != (0 as i32) {
            let mut bEofSave: i32 = ((unsafe { (*pRun).bEof }) as u32) as i32;
            fts3EvalRestart(pCsr, pRun, std::ptr::addr_of_mut!(rc));
            '__slate_break_1835: while rc == (0 as i32) && !((unsafe { (*pRun).bEof }) != (0 as u8))
            {
                fts3EvalNextRow(pCsr, pRun, std::ptr::addr_of_mut!(rc));
                if bEofSave == (0 as i32) && (unsafe { (*pRun).iDocid }) == iDocid {
                    break '__slate_break_1835;
                }
            }
            0 as i32;
            if rc == (0 as i32) && (((unsafe { (*pRun).bEof }) as u32) as i32) != bEofSave {
                rc = (11 as i32) | (1 as i32) << (8 as i32);
            }
        }
        if bTreeEof != (0 as u8) {
            '__slate_break_1836: while rc == (0 as i32) && !((unsafe { (*pRun).bEof }) != (0 as u8))
            {
                fts3EvalNextRow(pCsr, pRun, std::ptr::addr_of_mut!(rc));
            }
        }
        if rc != (0 as i32) {
            return rc;
        }
        bMatch = 1 as i32;
        p = pNear;
        '__slate_break_1837: while p != std::ptr::null_mut::<Fts3Expr>() {
            let mut bEof: u8 = ((0 as i32) as i8) as u8;
            let mut pTest: *mut Fts3Expr = p;
            let mut pPh: *mut Fts3Phrase = unsafe { std::mem::zeroed() };
            0 as i32;
            if (unsafe { (*pTest).eType }) == (1 as i32) {
                pTest = unsafe { (*pTest).pRight };
            }
            0 as i32;
            pPh = unsafe { (*pTest).pPhrase };
            pIter = unsafe { (*pPh).pOrPoslist };
            iDocid = unsafe { (*pPh).iOrDocid };
            if (((unsafe { (*pCsr).bDesc }) as u32) as i32) == bDescDoclist {
                bEof = (!((unsafe { (*pPh).doclist.nAll }) != (0 as i32))
                    || pIter
                        >= unsafe {
                            unsafe { (*pPh).doclist.aAll }
                                .offset((unsafe { (*pPh).doclist.nAll }) as isize)
                        }) as u8;
                '__slate_break_1838: while (pIter == std::ptr::null_mut::<i8>()
                    || (if bDescDoclist != (0 as i32) {
                        -(1 as i32)
                    } else {
                        1 as i32
                    }) * if iDocid > unsafe { (*pCsr).iPrevId } {
                        1 as i32
                    } else {
                        if iDocid == unsafe { (*pCsr).iPrevId } {
                            0 as i32
                        } else {
                            -(1 as i32)
                        }
                    } < (0 as i32))
                    && ((bEof as u32) as i32) == (0 as i32)
                {
                    sqlite3Fts3DoclistNext(
                        bDescDoclist,
                        unsafe { (*pPh).doclist.aAll },
                        unsafe { (*pPh).doclist.nAll },
                        std::ptr::addr_of_mut!(pIter),
                        std::ptr::addr_of_mut!(iDocid),
                        std::ptr::addr_of_mut!(bEof),
                    );
                }
            } else {
                bEof = (!((unsafe { (*pPh).doclist.nAll }) != (0 as i32))
                    || pIter != std::ptr::null_mut::<i8>()
                        && pIter <= unsafe { (*pPh).doclist.aAll }) as u8;
                '__slate_break_1839: while (pIter == std::ptr::null_mut::<i8>()
                    || (if bDescDoclist != (0 as i32) {
                        -(1 as i32)
                    } else {
                        1 as i32
                    }) * if iDocid > unsafe { (*pCsr).iPrevId } {
                        1 as i32
                    } else {
                        if iDocid == unsafe { (*pCsr).iPrevId } {
                            0 as i32
                        } else {
                            -(1 as i32)
                        }
                    } > (0 as i32))
                    && ((bEof as u32) as i32) == (0 as i32)
                {
                    let mut dummy: i32 = 0 as i32;
                    sqlite3Fts3DoclistPrev(
                        bDescDoclist,
                        unsafe { (*pPh).doclist.aAll },
                        unsafe { (*pPh).doclist.nAll },
                        std::ptr::addr_of_mut!(pIter),
                        std::ptr::addr_of_mut!(iDocid),
                        std::ptr::addr_of_mut!(dummy),
                        std::ptr::addr_of_mut!(bEof),
                    );
                }
            }
            unsafe {
                (*pPh).pOrPoslist = pIter;
            }
            unsafe {
                (*pPh).iOrDocid = iDocid;
            }
            if bEof != (0 as u8) || iDocid != unsafe { (*pCsr).iPrevId } {
                bMatch = 0 as i32;
            }
            p = unsafe { (*p).pLeft };
        }
        if bMatch != (0 as i32) {
            pIter = unsafe { (*pPhrase).pOrPoslist };
        } else {
            pIter = std::ptr::null_mut::<i8>();
        }
    }
    if pIter == std::ptr::null_mut::<i8>() {
        return 0 as i32;
    }
    if ((unsafe { *pIter }) as i32) == (1 as i32) {
        let __v1924: *mut i8 = pIter;
        let __v1925: *mut i8 = unsafe { __v1924.offset((1 as i32) as isize) };
        pIter = __v1925;
        let __v1926: *mut i8 = pIter;
        let __v1927: i32;
        if (((unsafe { *(pIter as *mut u8) }) as u32) as i32) & (128 as i32) != (0 as i32) {
            __v1927 = sqlite3Fts3GetVarint32(pIter as *const i8, std::ptr::addr_of_mut!(iThis));
        } else {
            unsafe {
                *std::ptr::addr_of_mut!(iThis) = ((unsafe { *(pIter as *mut u8) }) as u32) as i32;
            }
            __v1927 = 1 as i32;
        }
        let __v1928: *mut i8 = unsafe { __v1926.offset(__v1927 as isize) };
        pIter = __v1928;
    } else {
        iThis = 0 as i32;
    }
    '__slate_break_1840: while iThis < iCol {
        fts3ColumnlistCopy(
            std::ptr::null_mut::<*mut i8>(),
            std::ptr::addr_of_mut!(pIter),
        );
        if ((unsafe { *pIter }) as i32) == (0 as i32) {
            return 0 as i32;
        }
        let __v1929: *mut i8 = pIter;
        let __v1930: *mut i8 = unsafe { __v1929.offset((1 as i32) as isize) };
        pIter = __v1930;
        let __v1931: *mut i8 = pIter;
        let __v1932: i32;
        if (((unsafe { *(pIter as *mut u8) }) as u32) as i32) & (128 as i32) != (0 as i32) {
            __v1932 = sqlite3Fts3GetVarint32(pIter as *const i8, std::ptr::addr_of_mut!(iThis));
        } else {
            unsafe {
                *std::ptr::addr_of_mut!(iThis) = ((unsafe { *(pIter as *mut u8) }) as u32) as i32;
            }
            __v1932 = 1 as i32;
        }
        let __v1933: *mut i8 = unsafe { __v1931.offset(__v1932 as isize) };
        pIter = __v1933;
    }
    if ((unsafe { *pIter }) as i32) == (0 as i32) {
        pIter = std::ptr::null_mut::<i8>();
    }
    unsafe {
        *ppOut = if iCol == iThis {
            pIter
        } else {
            std::ptr::null_mut::<i8>()
        };
    }
    return 0 as i32;
}

/// Free all components of the Fts3Phrase structure that were allocated by
/// the eval module. Specifically, this means to free:
///
///   * the contents of pPhrase->doclist, and
///   * any Fts3MultiSegReader objects held by phrase tokens.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3Fts3EvalPhraseCleanup(mut pPhrase: *mut Fts3Phrase) {
    if pPhrase != std::ptr::null_mut::<Fts3Phrase>() {
        let mut i: i32 = 0 as i32;
        unsafe { sqlite3_free((unsafe { (*pPhrase).doclist.aAll }) as *mut ()) };
        fts3EvalInvalidatePoslist(pPhrase);
        unsafe {
            memset(
                (unsafe { std::ptr::addr_of_mut!((*pPhrase).doclist) }) as *mut (),
                0 as i32,
                56 as u64,
            )
        };
        i = 0 as i32;
        '__slate_break_1841: loop {
            if !(i < unsafe { (*pPhrase).nToken }) {
                break;
            }
            fts3SegReaderCursorFree(unsafe {
                (*unsafe {
                    unsafe { std::ptr::addr_of_mut!((*pPhrase).aToken) as *mut Fts3PhraseToken }
                        .offset(i as isize)
                })
                .pSegcsr
            });
            unsafe {
                (*unsafe {
                    unsafe { std::ptr::addr_of_mut!((*pPhrase).aToken) as *mut Fts3PhraseToken }
                        .offset(i as isize)
                })
                .pSegcsr = std::ptr::null_mut::<Fts3MultiSegReader>();
            }
            let __v1922: i32 = i;
            let __v1923: i32 = __v1922 + (1 as i32);
            i = __v1923;
        }
    }
}

// Return SQLITE_CORRUPT_VTAB.
