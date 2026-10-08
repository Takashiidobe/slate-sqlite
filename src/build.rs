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
//! This file contains C code routines that are called by the SQLite parser
//! when syntax rules are reduced.  The routines in this file handle the
//! following kinds of SQL syntax:
//!
//!     CREATE TABLE
//!     DROP TABLE
//!     CREATE INDEX
//!     DROP INDEX
//!     creating ID lists
//!     BEGIN TRANSACTION
//!     COMMIT
//!     ROLLBACK
unsafe extern "C" {
    static mut sqlite3StrBINARY: [i8; 0];
    static mut sqlite3StdTypeLen: [u8; 0];
    static mut sqlite3StdTypeAffinity: [i8; 0];
    static mut sqlite3StdType: [*const i8; 0];
    static mut sqlite3UpperToLower: [u8; 0];
    static mut sqlite3CtypeMap: [u8; 0];
    static mut sqlite3Config: Sqlite3Config;
    fn sqlite3_snprintf(__v1416: i32, __v1417: *mut i8, __v1418: *const i8, ...) -> *mut i8;
    fn sqlite3_str_appendf(__v1419: *mut sqlite3_str, zFormat: *const i8, ...);
    fn sqlite3_str_append(__v1421: *mut sqlite3_str, zIn: *const i8, N: i32);
    fn sqlite3_str_appendall(__v1424: *mut sqlite3_str, zIn: *const i8);
    fn sqlite3_stricmp(__v1426: *const i8, __v1427: *const i8) -> i32;
    fn sqlite3_strnicmp(__v1428: *const i8, __v1429: *const i8, __v1430: i32) -> i32;
    fn sqlite3HashInsert(__v1431: *mut Hash, pKey: *const i8, pData: *mut ()) -> *mut ();
    fn sqlite3HashFind(__v1434: *const Hash, pKey: *const i8) -> *mut ();
    fn memcpy(__dest: *mut (), __src: *const (), __n: u64) -> *mut ();
    fn memset(__s: *mut (), __c: i32, __n: u64) -> *mut ();
    fn strcmp(__s1: *const i8, __s2: *const i8) -> i32;
    fn strrchr(__s: *const i8, __c: i32) -> *mut i8;
    fn sqlite3BtreeOpen(
        pVfs: *mut sqlite3_vfs,
        zFilename: *const i8,
        db: *mut sqlite3,
        ppBtree: *mut *mut Btree,
        flags_1450: i32,
        vfsFlags: i32,
    ) -> i32;
    fn sqlite3BtreeSetPageSize(p: *mut Btree, nPagesize: i32, nReserve: i32, eFix: i32) -> i32;
    fn sqlite3BtreeIsReadonly(pBt: *mut Btree) -> i32;
    fn sqlite3BtreeEnterAll(__v1457: *mut sqlite3);
    fn sqlite3BtreeSharable(__v1458: *mut Btree) -> i32;
    fn sqlite3BtreeLeaveAll(__v1459: *mut sqlite3);
    fn sqlite3VdbeAddOp0(__v1460: *mut Vdbe, __v1461: i32) -> i32;
    fn sqlite3VdbeAddOp1(__v1462: *mut Vdbe, __v1463: i32, __v1464: i32) -> i32;
    fn sqlite3VdbeAddOp2(__v1465: *mut Vdbe, __v1466: i32, __v1467: i32, __v1468: i32) -> i32;
    fn sqlite3VdbeGoto(__v1469: *mut Vdbe, __v1470: i32) -> i32;
    fn sqlite3VdbeAddOp3(
        __v1471: *mut Vdbe,
        __v1472: i32,
        __v1473: i32,
        __v1474: i32,
        __v1475: i32,
    ) -> i32;
    fn sqlite3VdbeAddOp4(
        __v1476: *mut Vdbe,
        __v1477: i32,
        __v1478: i32,
        __v1479: i32,
        __v1480: i32,
        zP4: *const i8,
        __v1482: i32,
    ) -> i32;
    fn sqlite3VdbeAddOp4Int(
        __v1483: *mut Vdbe,
        __v1484: i32,
        __v1485: i32,
        __v1486: i32,
        __v1487: i32,
        __v1488: i32,
    ) -> i32;
    fn sqlite3VdbeEndCoroutine(__v1489: *mut Vdbe, __v1490: i32);
    fn sqlite3VdbeAddParseSchemaOp(
        __v1491: *mut Vdbe,
        __v1492: i32,
        __v1493: *mut i8,
        __v1494: u16,
    );
    fn sqlite3VdbeChangeOpcode(__v1495: *mut Vdbe, addr: i32, __v1497: u8);
    fn sqlite3VdbeChangeP3(__v1498: *mut Vdbe, addr: i32, P3: i32);
    fn sqlite3VdbeChangeP5(__v1501: *mut Vdbe, P5: u16);
    fn sqlite3VdbeJumpHere(__v1503: *mut Vdbe, addr: i32);
    fn sqlite3VdbeUsesBtree(__v1505: *mut Vdbe, __v1506: i32);
    fn sqlite3VdbeMakeReady(__v1507: *mut Vdbe, __v1508: *mut Parse);
    fn sqlite3VdbeCurrentAddr(__v1509: *mut Vdbe) -> i32;
    fn sqlite3VdbeComment(__v1510: *mut Vdbe, __v1511: *const i8, ...);
    fn sqlite3CorruptError(__v1512: i32) -> i32;
    fn sqlite3StrICmp(__v1513: *const i8, __v1514: *const i8) -> i32;
    fn sqlite3Strlen30(__v1515: *const i8) -> i32;
    fn sqlite3ColumnType(__v1516: *mut Column, __v1517: *mut i8) -> *mut i8;
    fn sqlite3DbMallocZero(__v1518: *mut sqlite3, __v1519: u64) -> *mut ();
    fn sqlite3DbMallocRaw(__v1520: *mut sqlite3, __v1521: u64) -> *mut ();
    fn sqlite3DbMallocRawNN(__v1522: *mut sqlite3, __v1523: u64) -> *mut ();
    fn sqlite3DbStrDup(__v1524: *mut sqlite3, __v1525: *const i8) -> *mut i8;
    fn sqlite3DbStrNDup(__v1526: *mut sqlite3, __v1527: *const i8, __v1528: u64) -> *mut i8;
    fn sqlite3DbSpanDup(__v1529: *mut sqlite3, __v1530: *const i8, __v1531: *const i8) -> *mut i8;
    fn sqlite3DbReallocOrFree(__v1532: *mut sqlite3, __v1533: *mut (), __v1534: u64) -> *mut ();
    fn sqlite3DbRealloc(__v1535: *mut sqlite3, __v1536: *mut (), __v1537: u64) -> *mut ();
    fn sqlite3DbFree(__v1538: *mut sqlite3, __v1539: *mut ());
    fn sqlite3DbNNFreeNN(__v1540: *mut sqlite3, __v1541: *mut ());
    fn sqlite3MPrintf(__v1542: *mut sqlite3, __v1543: *const i8, ...) -> *mut i8;
    fn sqlite3VMPrintf(
        __v1544: *mut sqlite3,
        __v1545: *const i8,
        __v1546: core::ffi::VaList<'_>,
    ) -> *mut i8;
    fn sqlite3ErrorMsg(__v1547: *mut Parse, __v1548: *const i8, ...);
    fn sqlite3Dequote(__v1549: *mut i8);
    fn sqlite3DequoteToken(__v1550: *mut Token);
    fn sqlite3TokenInit(__v1551: *mut Token, __v1552: *mut i8);
    fn sqlite3KeywordCode(__v1553: *const u8, __v1554: i32) -> i32;
    fn sqlite3RunParser(__v1555: *mut Parse, __v1556: *const i8) -> i32;
    fn sqlite3GetTempReg(__v1558: *mut Parse) -> i32;
    fn sqlite3ReleaseTempReg(__v1559: *mut Parse, __v1560: i32);
    fn sqlite3ExprAlloc(
        __v1561: *mut sqlite3,
        __v1562: i32,
        __v1563: *const Token,
        __v1564: i32,
    ) -> *mut Expr;
    fn sqlite3PExpr(
        __v1565: *mut Parse,
        __v1566: i32,
        __v1567: *mut Expr,
        __v1568: *mut Expr,
    ) -> *mut Expr;
    fn sqlite3ExprDelete(__v1569: *mut sqlite3, __v1570: *mut Expr);
    fn sqlite3ExprListAppend(
        __v1571: *mut Parse,
        __v1572: *mut ExprList,
        __v1573: *mut Expr,
    ) -> *mut ExprList;
    fn sqlite3ExprListSetSortOrder(__v1574: *mut ExprList, __v1575: i32, __v1576: i32);
    fn sqlite3ExprListSetName(
        __v1577: *mut Parse,
        __v1578: *mut ExprList,
        __v1579: *const Token,
        __v1580: i32,
    );
    fn sqlite3ExprListDelete(__v1581: *mut sqlite3, __v1582: *mut ExprList);
    fn sqlite3IndexHasDuplicateRootPage(__v1583: *mut Index) -> i32;
    fn sqlite3PragmaVtabRegister(__v1584: *mut sqlite3, zName: *const i8) -> *mut Module;
    fn sqlite3ColumnsFromExprList(
        __v1603: *mut Parse,
        __v1604: *mut ExprList,
        __v1605: *mut i16,
        __v1606: *mut *mut Column,
    ) -> i32;
    fn sqlite3SubqueryColumnTypes(
        __v1607: *mut Parse,
        __v1608: *mut Table,
        __v1609: *mut Select,
        __v1610: i8,
    );
    fn sqlite3ResultSetOfSelect(
        __v1611: *mut Parse,
        __v1612: *mut Select,
        __v1613: i8,
    ) -> *mut Table;
    fn sqlite3AutoincrementBegin(pParse: *mut Parse);
    fn sqlite3ClearOnOrUsing(__v1733: *mut sqlite3, __v1734: *mut OnOrUsing);
    fn sqlite3Select(__v1757: *mut Parse, __v1758: *mut Select, __v1759: *mut SelectDest) -> i32;
    fn sqlite3SelectDelete(__v1760: *mut sqlite3, __v1761: *mut Select);
    fn sqlite3SrcListLookup(__v1762: *mut Parse, __v1763: *mut SrcList) -> *mut Table;
    fn sqlite3OpenTable(
        __v1764: *mut Parse,
        iCur: i32,
        iDb: i32,
        __v1767: *mut Table,
        __v1768: i32,
    );
    fn sqlite3ExprCode(__v1769: *mut Parse, __v1770: *mut Expr, __v1771: i32);
    fn sqlite3GetVdbe(__v1794: *mut Parse) -> *mut Vdbe;
    fn sqlite3ExprIsConstantOrFunction(__v1806: *mut Expr, __v1807: u8) -> i32;
    fn sqlite3GenerateIndexKey(
        __v1808: *mut Parse,
        __v1809: *mut Index,
        __v1810: i32,
        __v1811: i32,
        __v1812: i32,
        __v1813: *mut i32,
        __v1814: *mut Index,
        __v1815: i32,
    ) -> i32;
    fn sqlite3ResolvePartIdxLabel(__v1816: *mut Parse, __v1817: i32);
    fn sqlite3ExprDup(__v1835: *mut sqlite3, __v1836: *const Expr, __v1837: i32) -> *mut Expr;
    fn sqlite3ExprListDup(
        __v1838: *mut sqlite3,
        __v1839: *const ExprList,
        __v1840: i32,
    ) -> *mut ExprList;
    fn sqlite3SelectDup(__v1841: *mut sqlite3, __v1842: *const Select, __v1843: i32)
    -> *mut Select;
    fn sqlite3JsonVtabRegister(__v1844: *mut sqlite3, __v1845: *const i8) -> *mut Module;
    fn sqlite3DropTriggerPtr(__v1848: *mut Parse, __v1849: *mut Trigger);
    fn sqlite3TriggerList(__v1850: *mut Parse, __v1851: *mut Table) -> *mut Trigger;
    fn sqlite3ColumnIndex(pTab: *mut Table, zCol: *const i8) -> i32;
    fn sqlite3AuthCheck(
        __v1861: *mut Parse,
        __v1862: i32,
        __v1863: *const i8,
        __v1864: *const i8,
        __v1865: *const i8,
    ) -> i32;
    fn sqlite3DbIsNamed(db: *mut sqlite3, iDb: i32, zName: *const i8) -> i32;
    fn sqlite3FixInit(
        __v1869: *mut DbFixer,
        __v1870: *mut Parse,
        __v1871: i32,
        __v1872: *const i8,
        __v1873: *const Token,
    );
    fn sqlite3FixSrcList(__v1874: *mut DbFixer, __v1875: *mut SrcList) -> i32;
    fn sqlite3FixSelect(__v1876: *mut DbFixer, __v1877: *mut Select) -> i32;
    fn sqlite3GetInt32(__v1878: *const i8, __v1879: *mut i32) -> i32;
    fn sqlite3LogEst(__v1880: u64) -> i16;
    fn sqlite3TableAffinity(__v1881: *mut Vdbe, __v1882: *mut Table, __v1883: i32);
    fn sqlite3ReadSchema(pParse: *mut Parse) -> i32;
    fn sqlite3FindCollSeq(
        __v1889: *mut sqlite3,
        enc: u8,
        __v1891: *const i8,
        __v1892: i32,
    ) -> *mut CollSeq;
    fn sqlite3LocateCollSeq(pParse: *mut Parse, zName: *const i8) -> *mut CollSeq;
    fn sqlite3ExprSkipCollate(__v1895: *mut Expr) -> *mut Expr;
    fn sqlite3StrIHash(__v1910: *const i8) -> u8;
    fn sqlite3ResolveSelfReference(
        __v1911: *mut Parse,
        __v1912: *mut Table,
        __v1913: i32,
        __v1914: *mut Expr,
        __v1915: *mut ExprList,
    ) -> i32;
    fn sqlite3RenameTokenMap(
        __v1916: *mut Parse,
        __v1917: *const (),
        __v1918: *const Token,
    ) -> *const ();
    fn sqlite3RenameTokenRemap(__v1919: *mut Parse, pTo: *const (), pFrom: *const ());
    fn sqlite3RenameExprUnmap(__v1922: *mut Parse, __v1923: *mut Expr);
    fn sqlite3RenameExprlistUnmap(__v1924: *mut Parse, __v1925: *mut ExprList);
    fn sqlite3DeleteIndexSamples(__v1932: *mut sqlite3, __v1933: *mut Index);
    fn sqlite3SchemaClear(__v1935: *mut ());
    fn sqlite3SchemaToIndex(db: *mut sqlite3, __v1937: *mut Schema) -> i32;
    fn sqlite3KeyInfoAlloc(__v1938: *mut sqlite3, __v1939: i32, __v1940: i32) -> *mut KeyInfo;
    fn sqlite3KeyInfoUnref(__v1941: *mut KeyInfo);
    fn sqlite3KeyInfoRef(__v1942: *mut KeyInfo) -> *mut KeyInfo;
    fn sqlite3OomFault(__v1947: *mut sqlite3) -> *mut ();
    fn sqlite3StrAccumInit(
        __v1949: *mut sqlite3_str,
        __v1950: *mut sqlite3,
        __v1951: *mut i8,
        __v1952: i32,
        __v1953: i32,
    );
    fn sqlite3StrAccumFinish(__v1954: *mut sqlite3_str) -> *mut i8;
    fn sqlite3SelectDestInit(__v1955: *mut SelectDest, __v1956: i32, __v1957: i32);
    fn sqlite3VtabClear(db: *mut sqlite3, __v1964: *mut Table);
    fn sqlite3VtabUnlockList(__v1965: *mut sqlite3);
    fn sqlite3GetVTable(__v1966: *mut sqlite3, __v1967: *mut Table) -> *mut VTable;
    fn sqlite3VtabEponymousTableInit(__v1976: *mut Parse, __v1977: *mut Module) -> i32;
    fn sqlite3VtabCallConnect(__v1978: *mut Parse, __v1979: *mut Table) -> i32;
    fn sqlite3ParserAddCleanup(
        __v1980: *mut Parse,
        __v1981: Option<unsafe extern "C-unwind" fn(*mut sqlite3, *mut ())>,
        __v1982: *mut (),
    ) -> *mut ();
    fn sqlite3ExprListCheckLength(__v1983: *mut Parse, __v1984: *mut ExprList, __v1985: *const i8);
    fn sqlite3FkDropTable(__v2000: *mut Parse, __v2001: *mut SrcList, __v2002: *mut Table);
    fn sqlite3FkDelete(__v2003: *mut sqlite3, __v2004: *mut Table);
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
    trace: __SlateRecord183,
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
    u1: __SlateRecord184,
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
    u: __SlateRecord185,
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
    __slate_bits_0: __slate_bits::__SlateBits79U0,
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
    u: __SlateRecord186,
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
    __slate_bits_0: __slate_bits::__SlateBits105U0,
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
    u: __SlateRecord194,
    pLeft: *mut Expr,
    pRight: *mut Expr,
    x: __SlateRecord195,
    nHeight: i32,
    iTable: i32,
    iColumn: i16,
    iAgg: i16,
    w: __SlateRecord196,
    pAggInfo: *mut AggInfo,
    y: __SlateRecord197,
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
    fg: __SlateRecord204,
    iCursor: i32,
    colUsed: u64,
    u1: __SlateRecord205,
    u2: __SlateRecord206,
    u3: __SlateRecord207,
    u4: __SlateRecord208,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct OnOrUsing {
    pOn: *mut Expr,
    pUsing: *mut IdList,
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
struct RenameToken {}

#[repr(C)]
#[derive(Clone, Copy)]
struct NameContext {
    pParse: *mut Parse,
    pSrcList: *mut SrcList,
    uNC: __SlateRecord209,
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
struct SelectDest {
    eDest: u8,
    iSDParm: i32,
    iSDParm2: i32,
    iSdst: i32,
    nSdst: i32,
    zAffSdst: *mut i8,
    pOrderBy: *mut ExprList,
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
    __slate_bits_0: __slate_bits::__SlateBits121U0,
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
    u1: __SlateRecord211,
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
struct Walker {
    pParse: *mut Parse,
    xExprCallback: Option<unsafe extern "C-unwind" fn(*mut Walker, *mut Expr) -> i32>,
    xSelectCallback: Option<unsafe extern "C-unwind" fn(*mut Walker, *mut Select) -> i32>,
    xSelectCallback2: Option<unsafe extern "C-unwind" fn(*mut Walker, *mut Select)>,
    walkerDepth: i32,
    eCode: u16,
    mWFlags: u16,
    u: __SlateRecord215,
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
    __slate_bits_0: __slate_bits::__SlateBits182U0,
    azInit: *mut *const i8,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord183 {
    xLegacy: Option<unsafe extern "C-unwind" fn(*mut (), *const i8)>,
    xV2: Option<unsafe extern "C-unwind" fn(u32, *mut (), *mut (), *mut ()) -> i32>,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord184 {
    isInterrupted: i32,
    notUsed1: f64,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord185 {
    pHash: *mut FuncDef,
    pDestructor: *mut FuncDestructor,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord186 {
    tab: __SlateRecord187,
    view: __SlateRecord188,
    vtab: __SlateRecord189,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord187 {
    addColOffset: i32,
    pFKey: *mut FKey,
    pDfltList: *mut ExprList,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord188 {
    pSelect: *mut Select,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord189 {
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
union __SlateRecord194 {
    zToken: *mut i8,
    iValue: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord195 {
    pList: *mut ExprList,
    pSelect: *mut Select,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord196 {
    iJoin: i32,
    iOfst: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord197 {
    pTab: *mut Table,
    pWin: *mut Window,
    nReg: i32,
    sub: __SlateRecord198,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord198 {
    iAddr: i32,
    regReturn: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct ExprList_item {
    pExpr: *mut Expr,
    zEName: *mut i8,
    fg: __SlateRecord200,
    u: __SlateRecord201,
}

#[repr(C, align(4))]
#[derive(Clone, Copy)]
struct __SlateRecord200 {
    sortFlags: u8,
    __slate_bits_0: __slate_bits::__SlateBits200U0,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord201 {
    x: __SlateRecord202,
    iConstExprReg: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord202 {
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
struct __SlateRecord204 {
    jointype: u8,
    __slate_bits_0: __slate_bits::__SlateBits204U0,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord205 {
    zIndexedBy: *mut i8,
    pFuncArg: *mut ExprList,
    nRow: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord206 {
    pIBIndex: *mut Index,
    pCteUse: *mut CteUse,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord207 {
    pOn: *mut Expr,
    pUsing: *mut IdList,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord208 {
    pSchema: *mut Schema,
    zDatabase: *mut i8,
    pSubq: *mut Subquery,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord209 {
    pEList: *mut ExprList,
    pAggInfo: *mut AggInfo,
    pUpsert: *mut Upsert,
    iBaseReg: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord211 {
    cr: __SlateRecord212,
    d: __SlateRecord213,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord212 {
    addrCrTab: i32,
    regRowid: i32,
    regRoot: i32,
    constraintName: Token,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord213 {
    pReturning: *mut Returning,
}

/// The TableLock structure is only used by the sqlite3TableLock() and
/// codeTableLocks() functions.
#[repr(C)]
#[derive(Clone, Copy)]
struct TableLock {
    /// The database containing the table to be locked
    iDb: i32,
    /// The root page of the table to be locked
    iTab: u32,
    /// True for write lock.  False for a read lock
    isWriteLock: u8,
    /// Name of the table
    zLockName: *const i8,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord215 {
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
    pub struct __SlateBits79U0 {
        #[bits(4)]
        pub notNull: u32,
        #[bits(4)]
        pub eCType: u32,
    }
    #[bitfields::bitfield([u8; 2], c_names = true, new = false, from_into_bits = false, from_traits = false, default = false, debug = false, builder = false, bit_ops = false)]
    pub struct __SlateBits200U0 {
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
    pub struct __SlateBits204U0 {
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
    pub struct __SlateBits105U0 {
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
    pub struct __SlateBits182U0 {
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
    pub struct __SlateBits121U0 {
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

/// Record the fact that we want to lock a table at run-time.
///
/// The table to be locked has root page iTab and is found in database iDb.
/// A read or a write lock can be taken depending on isWritelock.
///
/// This routine just records the fact that the lock is desired.  The
/// code to make the lock occur is generated by a later call to
/// codeTableLocks() which occurs during sqlite3FinishCoding().
///
/// # Arguments
///
/// * `pParse` - Parsing context
/// * `iDb` - Index of the database containing the table to lock
/// * `iTab` - Root page number of the table to be locked
/// * `isWriteLock` - True for a write lock
/// * `zName` - Name of the table to be locked
fn lockTable(
    mut pParse: *mut Parse,
    mut iDb: i32,
    mut iTab: u32,
    mut isWriteLock: u8,
    mut zName: *const i8,
) {
    let mut pToplevel: *mut Parse = unsafe { std::mem::zeroed() };
    let mut i: i32 = 0 as i32;
    let mut nBytes: i32 = 0 as i32;
    let mut p: *mut TableLock = unsafe { std::mem::zeroed() };
    0 as i32;
    pToplevel = if (unsafe { (*pParse).pToplevel }) != std::ptr::null_mut::<Parse>() {
        unsafe { (*pParse).pToplevel }
    } else {
        pParse
    };
    i = 0 as i32;
    '__slate_break_2005: loop {
        if !(i < unsafe { (*pToplevel).nTableLock }) {
            break;
        }
        p = unsafe { unsafe { (*pToplevel).aTableLock }.offset(i as isize) };
        if (unsafe { (*p).iDb }) == iDb && (unsafe { (*p).iTab }) == iTab {
            unsafe {
                (*p).isWriteLock =
                    ((unsafe { (*p).isWriteLock }) != (0 as u8) || isWriteLock != (0 as u8)) as u8;
            }
            return;
        }
        let __v2675: i32 = i;
        let __v2676: i32 = __v2675 + (1 as i32);
        i = __v2676;
    }
    0 as i32;
    nBytes = ((24 as u64)
        .wrapping_mul((((unsafe { (*pToplevel).nTableLock }) + (1 as i32)) as i64) as u64)
        as u32) as i32;
    if (unsafe { (*pToplevel).nTableLock }) == (0 as i32) {
        unsafe {
            (*pToplevel).aTableLock = std::ptr::null_mut::<TableLock>();
        }
    }
    unsafe {
        (*pToplevel).aTableLock = (unsafe {
            sqlite3DbReallocOrFree(
                unsafe { (*pToplevel).db },
                (unsafe { (*pToplevel).aTableLock }) as *mut (),
                (nBytes as i64) as u64,
            )
        }) as *mut TableLock;
    }
    if (unsafe { (*pToplevel).aTableLock }) != std::ptr::null_mut::<TableLock>() {
        let __v2677: *mut Parse = pToplevel;
        let __v2678: i32 = unsafe { (*__v2677).nTableLock };
        let __v2679: i32 = __v2678 + (1 as i32);
        unsafe {
            (*__v2677).nTableLock = __v2679;
        }
        p = unsafe { unsafe { (*pToplevel).aTableLock }.offset(__v2678 as isize) };
        unsafe {
            (*p).iDb = iDb;
        }
        unsafe {
            (*p).iTab = iTab;
        }
        unsafe {
            (*p).isWriteLock = isWriteLock;
        }
        unsafe {
            (*p).zLockName = zName;
        }
    } else {
        unsafe {
            (*pToplevel).nTableLock = 0 as i32;
        }
        unsafe { sqlite3OomFault(unsafe { (*pToplevel).db }) };
    }
}

/// # Arguments
///
/// * `pParse` - Parsing context
/// * `iDb` - Index of the database containing the table to lock
/// * `iTab` - Root page number of the table to be locked
/// * `isWriteLock` - True for a write lock
/// * `zName` - Name of the table to be locked
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3TableLock(
    mut pParse: *mut Parse,
    mut iDb: i32,
    mut iTab: u32,
    mut isWriteLock: u8,
    mut zName: *const i8,
) {
    if iDb == (1 as i32) {
        return;
    }
    if !((unsafe {
        sqlite3BtreeSharable(unsafe {
            (*unsafe { unsafe { (*unsafe { (*pParse).db }).aDb }.offset(iDb as isize) }).pBt
        })
    }) != (0 as i32))
    {
        return;
    }
    lockTable(pParse, iDb, iTab, isWriteLock, zName);
}

/// Code an OP_TableLock instruction for each table locked by the
/// statement (configured by calls to sqlite3TableLock()).
fn codeTableLocks(mut pParse: *mut Parse) {
    let mut i: i32 = 0 as i32;
    let mut pVdbe: *mut Vdbe = unsafe { (*pParse).pVdbe };
    0 as i32;
    i = 0 as i32;
    '__slate_break_2006: loop {
        if !(i < unsafe { (*pParse).nTableLock }) {
            break;
        }
        let mut p: *mut TableLock = unsafe { unsafe { (*pParse).aTableLock }.offset(i as isize) };
        let mut p1: i32 = unsafe { (*p).iDb };
        unsafe {
            sqlite3VdbeAddOp4(
                pVdbe,
                171 as i32,
                p1,
                (unsafe { (*p).iTab }) as i32,
                ((unsafe { (*p).isWriteLock }) as u32) as i32,
                unsafe { (*p).zLockName },
                -(1 as i32),
            )
        };
        let __v2680: i32 = i;
        let __v2681: i32 = __v2680 + (1 as i32);
        i = __v2681;
    }
}

/// Return TRUE if the given yDbMask object is empty - if it contains no
/// 1 bits.  This routine is used by the DbMaskAllZero() and DbMaskNotZero()
/// macros when SQLITE_MAX_ATTACHED is greater than 30.
/// This routine is called after a single SQL statement has been
/// parsed and a VDBE program to execute that statement has been
/// prepared.  This routine puts the finishing touches on the
/// VDBE program and resets the pParse structure for the next
/// parse.
///
/// Note that if an error occurred, it might be the case that
/// no VDBE code was generated.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3FinishCoding(mut pParse: *mut Parse) {
    let mut db: *mut sqlite3 = unsafe { std::mem::zeroed() };
    let mut v: *mut Vdbe = unsafe { std::mem::zeroed() };
    let mut iDb: i32 = 0 as i32;
    let mut i: i32 = 0 as i32;
    0 as i32;
    db = unsafe { (*pParse).db };
    0 as i32;
    if (unsafe { (*pParse).nested }) != (0 as u8) {
        return;
    }
    if (unsafe { (*pParse).nErr }) != (0 as i32) {
        if (unsafe { (*db).mallocFailed }) != (0 as u8) {
            unsafe {
                (*pParse).rc = 7 as i32;
            }
        }
        return;
    }
    0 as i32;
    // Begin by generating some termination code at the end of the
    // vdbe program
    v = unsafe { (*pParse).pVdbe };
    if v == std::ptr::null_mut::<Vdbe>() {
        if (unsafe { (*db).init.busy }) != (0 as u8) {
            unsafe {
                (*pParse).rc = 101 as i32;
            }
            return;
        }
        v = unsafe { sqlite3GetVdbe(pParse) };
        if v == std::ptr::null_mut::<Vdbe>() {
            unsafe {
                (*pParse).rc = 1 as i32;
            }
        }
    }
    0 as i32;
    if v != std::ptr::null_mut::<Vdbe>() {
        if ((unsafe { (*pParse).__slate_bits_0.__get_bReturning() }) as i32) != (0 as i32) {
            let mut pReturning: *mut Returning = unsafe { std::mem::zeroed() };
            let mut addrRewind: i32 = 0 as i32;
            let mut reg: i32 = 0 as i32;
            0 as i32;
            pReturning = unsafe { (*pParse).u1.d.pReturning };
            if (unsafe { (*pReturning).nRetCol }) != (0 as i32) {
                unsafe { sqlite3VdbeAddOp0(v, 85 as i32) };
                addrRewind =
                    unsafe { sqlite3VdbeAddOp1(v, 36 as i32, unsafe { (*pReturning).iRetCur }) };
                {}
                reg = unsafe { (*pReturning).iRetReg };
                i = 0 as i32;
                '__slate_break_2007: loop {
                    if !(i < unsafe { (*pReturning).nRetCol }) {
                        break;
                    }
                    unsafe {
                        sqlite3VdbeAddOp3(
                            v,
                            96 as i32,
                            unsafe { (*pReturning).iRetCur },
                            i,
                            reg + i,
                        )
                    };
                    let __v2261: i32 = i;
                    let __v2262: i32 = __v2261 + (1 as i32);
                    i = __v2262;
                }
                unsafe { sqlite3VdbeAddOp2(v, 86 as i32, reg, i) };
                unsafe {
                    sqlite3VdbeAddOp2(
                        v,
                        40 as i32,
                        unsafe { (*pReturning).iRetCur },
                        addrRewind + (1 as i32),
                    )
                };
                {}
                unsafe { sqlite3VdbeJumpHere(v, addrRewind) };
            }
        }
        unsafe { sqlite3VdbeAddOp0(v, 72 as i32) };
        // The cookie mask contains one bit for each database file open.
        // (Bit 0 is for main, bit 1 is for temp, and so forth.)  Bits are
        // set for each database that is used.  Generate code to start a
        // transaction on each used database and to verify the schema cookie
        // on each used database.
        0 as i32;
        unsafe { sqlite3VdbeJumpHere(v, 0 as i32) };
        0 as i32;
        iDb = 0 as i32;
        '__slate_break_2008: loop {
            let mut pSchema: *mut Schema = unsafe { std::mem::zeroed() };
            if (((unsafe { (*pParse).cookieMask }) & ((1 as i32) as u32) << iDb
                != ((0 as i32) as u32)) as i32)
                == (0 as i32)
            {
            } else {
                unsafe { sqlite3VdbeUsesBtree(v, iDb) };
                pSchema =
                    unsafe { (*unsafe { unsafe { (*db).aDb }.offset(iDb as isize) }).pSchema };
                unsafe {
                    sqlite3VdbeAddOp4Int(
                        v,
                        2 as i32,
                        iDb,
                        ((unsafe { (*pParse).writeMask }) & ((1 as i32) as u32) << iDb
                            != ((0 as i32) as u32)) as i32,
                        unsafe { (*pSchema).schema_cookie },
                        unsafe { (*pSchema).iGeneration },
                    )
                }; // Opcode
                // P1
                // P2
                // P3
                // P4
                if (((unsafe { (*db).init.busy }) as u32) as i32) == (0 as i32) {
                    unsafe { sqlite3VdbeChangeP5(v, ((1 as i32) as i16) as u16) };
                }
                unsafe {
                    sqlite3VdbeComment(
                        v,
                        (b"usesStmtJournal=%d\0".as_ptr() as *mut i8) as *const i8,
                        (((unsafe { (*pParse).__slate_bits_0.__get_mayAbort() }) as i32)
                            != (0 as i32)
                            && (unsafe { (*pParse).isMultiWrite }) != (0 as u8))
                            as i32,
                    )
                };
            }
            let __v2263: i32 = iDb;
            let __v2264: i32 = __v2263 + (1 as i32);
            iDb = __v2264;
            if !(__v2264 < unsafe { (*db).nDb }) {
                break;
            }
        }
        i = 0 as i32;
        '__slate_break_2010: loop {
            if !(i < unsafe { (*pParse).nVtabLock }) {
                break;
            }
            let mut vtab: *mut i8 = (unsafe {
                sqlite3GetVTable(db, unsafe {
                    *unsafe { unsafe { (*pParse).apVtabLock }.offset(i as isize) }
                })
            }) as *mut i8;
            unsafe {
                sqlite3VdbeAddOp4(
                    v,
                    172 as i32,
                    0 as i32,
                    0 as i32,
                    0 as i32,
                    vtab as *const i8,
                    -(12 as i32),
                )
            };
            let __v2265: i32 = i;
            let __v2266: i32 = __v2265 + (1 as i32);
            i = __v2266;
        }
        unsafe {
            (*pParse).nVtabLock = 0 as i32;
        }
        // Once all the cookies have been verified and transactions opened,
        // obtain the required table-locks. This is a no-op unless the
        // shared-cache feature is enabled.
        if (unsafe { (*pParse).nTableLock }) != (0 as i32) {
            codeTableLocks(pParse);
        }
        // Initialize any AUTOINCREMENT data structures required.
        if ((unsafe { (*pParse).__slate_bits_0.__get_usesAinc() }) as i32) != (0 as i32) {
            unsafe { sqlite3AutoincrementBegin(pParse) };
        }
        // Code constant expressions that were factored out of inner loops.
        if (unsafe { (*pParse).pConstExpr }) != std::ptr::null_mut::<ExprList>() {
            let mut pEL: *mut ExprList = unsafe { (*pParse).pConstExpr };
            unsafe {
                (*pParse)
                    .__slate_bits_0
                    .__set_okConstFactor((0 as i32) as u32);
            }
            i = 0 as i32;
            '__slate_break_2011: loop {
                if !(i < unsafe { (*pEL).nExpr }) {
                    break;
                }
                0 as i32;
                unsafe {
                    sqlite3ExprCode(
                        pParse,
                        unsafe {
                            (*unsafe {
                                unsafe { std::ptr::addr_of_mut!((*pEL).a) as *mut ExprList_item }
                                    .offset(i as isize)
                            })
                            .pExpr
                        },
                        unsafe {
                            (*unsafe {
                                unsafe { std::ptr::addr_of_mut!((*pEL).a) as *mut ExprList_item }
                                    .offset(i as isize)
                            })
                            .u
                            .iConstExprReg
                        },
                    )
                };
                let __v2267: i32 = i;
                let __v2268: i32 = __v2267 + (1 as i32);
                i = __v2268;
            }
        }
        if ((unsafe { (*pParse).__slate_bits_0.__get_bReturning() }) as i32) != (0 as i32) {
            let mut pRet: *mut Returning = unsafe { std::mem::zeroed() };
            0 as i32;
            pRet = unsafe { (*pParse).u1.d.pReturning };
            if (unsafe { (*pRet).nRetCol }) != (0 as i32) {
                unsafe {
                    sqlite3VdbeAddOp2(v, 120 as i32, unsafe { (*pRet).iRetCur }, unsafe {
                        (*pRet).nRetCol
                    })
                };
            }
        }
        // Finally, jump back to the beginning of the executable code.
        unsafe { sqlite3VdbeGoto(v, 1 as i32) };
    }
    // Get the VDBE program ready for execution
    0 as i32;
    0 as i32;
    if (unsafe { (*pParse).nErr }) == (0 as i32) {
        // A minimum of one cursor is required if autoincrement is used
        // See ticket [a696379c1f08866]
        0 as i32;
        unsafe { sqlite3VdbeMakeReady(v, pParse) };
        unsafe {
            (*pParse).rc = 101 as i32;
        }
    } else {
        unsafe {
            (*pParse).rc = 1 as i32;
        }
    }
}

/// Run the parser and code generator recursively in order to generate
/// code for the SQL statement given onto the end of the pParse context
/// currently under construction.  Notes:
///
///   *  The final OP_Halt is not appended and other initialization
///      and finalization steps are omitted because those are handling by the
///      outermost parser.
///
///   *  Built-in SQL functions always take precedence over application-defined
///      SQL functions.  In other words, it is not possible to override a
///      built-in function.
#[unsafe(no_mangle)]
unsafe extern "C-unwind" fn sqlite3NestedParse(
    mut pParse: *mut Parse,
    mut zFormat: *const i8,
    mut __va_args: ...
) {
    let mut ap: core::ffi::VaList<'_> = unsafe { std::mem::zeroed() };
    let mut zSql: *mut i8 = unsafe { std::mem::zeroed() };
    let mut db: *mut sqlite3 = unsafe { (*pParse).db };
    let mut savedDbFlags: u32 = unsafe { (*db).mDbFlags };
    let mut saveBuf: __SlateAlign16<[i8; 152]> = __SlateAlign16([0 as i8; 152]);
    if (unsafe { (*pParse).nErr }) != (0 as i32) {
        return;
    }
    if (unsafe { (*pParse).eParseMode }) != (0 as u8) {
        return;
    }
    0 as i32; // Nesting should only be of limited depth
    ap = __va_args.clone();
    zSql = unsafe { sqlite3VMPrintf(db, zFormat, ap.clone()) };
    {}
    if zSql == std::ptr::null_mut::<i8>() {
        // This can result either from an OOM or because the formatted string
        // exceeds SQLITE_LIMIT_LENGTH.  In the latter case, we need to set
        // an error
        if !((unsafe { (*db).mallocFailed }) != (0 as u8)) {
            unsafe {
                (*pParse).rc = 18 as i32;
            }
        }
        let __v2631: *mut Parse = pParse;
        let __v2632: i32 = unsafe { (*__v2631).nErr };
        let __v2633: i32 = __v2632 + (1 as i32);
        unsafe {
            (*__v2631).nErr = __v2633;
        }
        return;
    }
    let __v2634: *mut Parse = pParse;
    let __v2635: u8 = unsafe { (*__v2634).nested };
    let __v2636: u8 = ((((__v2635 as u32) as i32) + (1 as i32)) as i8) as u8;
    unsafe {
        (*__v2634).nested = __v2636;
    }
    unsafe {
        memcpy(
            (saveBuf.0.as_mut_ptr() as *mut i8) as *mut (),
            (unsafe { (pParse as *mut i8).offset((280 as u64) as isize) }) as *const (),
            (432 as u64).wrapping_sub(280 as u64),
        )
    };
    unsafe {
        memset(
            (unsafe { (pParse as *mut i8).offset((280 as u64) as isize) }) as *mut (),
            0 as i32,
            (432 as u64).wrapping_sub(280 as u64),
        )
    };
    let __v2637: *mut sqlite3 = db;
    let __v2638: u32 = unsafe { (*__v2637).mDbFlags };
    let __v2639: u32 = __v2638 | ((2 as i32) as u32);
    unsafe {
        (*__v2637).mDbFlags = __v2639;
    }
    unsafe { sqlite3RunParser(pParse, zSql as *const i8) };
    unsafe {
        (*db).mDbFlags = savedDbFlags;
    }
    unsafe { sqlite3DbFree(db, zSql as *mut ()) };
    unsafe {
        memcpy(
            (unsafe { (pParse as *mut i8).offset((280 as u64) as isize) }) as *mut (),
            (saveBuf.0.as_mut_ptr() as *mut i8) as *const (),
            (432 as u64).wrapping_sub(280 as u64),
        )
    };
    let __v2640: *mut Parse = pParse;
    let __v2641: u8 = unsafe { (*__v2640).nested };
    let __v2642: u8 = ((((__v2641 as u32) as i32) - (1 as i32)) as i8) as u8;
    unsafe {
        (*__v2640).nested = __v2642;
    }
}

/// Locate the in-memory structure that describes a particular database
/// table given the name of that table and (optionally) the name of the
/// database containing the table.  Return NULL if not found.
///
/// If zDatabase is 0, all databases are searched for the table and the
/// first matching table is returned.  (No checking for duplicate table
/// names is done.)  The search order is TEMP first, then MAIN, then any
/// auxiliary databases added using the ATTACH command.
///
/// See also sqlite3LocateTable().
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3FindTable(
    mut db: *mut sqlite3,
    mut zName: *const i8,
    mut zDatabase: *const i8,
) -> *mut Table {
    let mut p: *mut Table = std::ptr::null_mut::<Table>();
    let mut i: i32 = 0 as i32;
    // All mutexes are required for schema access.  Make sure we hold them.
    0 as i32;
    if zDatabase != std::ptr::null::<i8>() {
        i = 0 as i32;
        '__slate_break_2012: loop {
            if !(i < unsafe { (*db).nDb }) {
                break;
            }
            if (unsafe {
                sqlite3StrICmp(
                    zDatabase,
                    (unsafe { (*unsafe { unsafe { (*db).aDb }.offset(i as isize) }).zDbSName })
                        as *const i8,
                )
            }) == (0 as i32)
            {
                break '__slate_break_2012;
            }
            let __v2560: i32 = i;
            let __v2561: i32 = __v2560 + (1 as i32);
            i = __v2561;
        }
        if i >= unsafe { (*db).nDb } {
            // No match against the official names.  But always match "main"
            // to schema 0 as a legacy fallback.
            if (unsafe { sqlite3StrICmp(zDatabase, (b"main\0".as_ptr() as *mut i8) as *const i8) })
                == (0 as i32)
            {
                i = 0 as i32;
            } else {
                return std::ptr::null_mut::<Table>();
            }
        }
        p = (unsafe {
            sqlite3HashFind(
                (unsafe {
                    std::ptr::addr_of_mut!(
                        (*unsafe { (*unsafe { unsafe { (*db).aDb }.offset(i as isize) }).pSchema })
                            .tblHash
                    )
                }) as *const Hash,
                zName,
            )
        }) as *mut Table;
        let __v2562: bool;
        if p == std::ptr::null_mut::<Table>() {
            __v2562 = (unsafe {
                sqlite3_strnicmp(
                    zName,
                    (b"sqlite_\0".as_ptr() as *mut i8) as *const i8,
                    7 as i32,
                )
            }) == (0 as i32);
        } else {
            __v2562 = false as bool;
        }
        if __v2562 {
            if i == (1 as i32) {
                let __v2563: bool;
                if (unsafe {
                    sqlite3StrICmp(
                        unsafe { zName.offset((7 as i32) as isize) },
                        (unsafe {
                            (b"sqlite_temp_schema\0".as_ptr() as *mut i8)
                                .offset((7 as i32) as isize)
                        }) as *const i8,
                    )
                }) == (0 as i32)
                {
                    __v2563 = true as bool;
                } else {
                    __v2563 = (unsafe {
                        sqlite3StrICmp(
                            unsafe { zName.offset((7 as i32) as isize) },
                            (unsafe {
                                (b"sqlite_schema\0".as_ptr() as *mut i8).offset((7 as i32) as isize)
                            }) as *const i8,
                        )
                    }) == (0 as i32);
                }
                let __v2564: bool;
                if __v2563 {
                    __v2564 = true as bool;
                } else {
                    __v2564 = (unsafe {
                        sqlite3StrICmp(
                            unsafe { zName.offset((7 as i32) as isize) },
                            (unsafe {
                                (b"sqlite_master\0".as_ptr() as *mut i8).offset((7 as i32) as isize)
                            }) as *const i8,
                        )
                    }) == (0 as i32);
                }
                if __v2564 {
                    p = (unsafe {
                        sqlite3HashFind(
                            (unsafe {
                                std::ptr::addr_of_mut!(
                                    (*unsafe {
                                        (*unsafe {
                                            unsafe { (*db).aDb }.offset((1 as i32) as isize)
                                        })
                                        .pSchema
                                    })
                                    .tblHash
                                )
                            }) as *const Hash,
                            (b"sqlite_temp_master\0".as_ptr() as *mut i8) as *const i8,
                        )
                    }) as *mut Table;
                }
            } else {
                if (unsafe {
                    sqlite3StrICmp(
                        unsafe { zName.offset((7 as i32) as isize) },
                        (unsafe {
                            (b"sqlite_schema\0".as_ptr() as *mut i8).offset((7 as i32) as isize)
                        }) as *const i8,
                    )
                }) == (0 as i32)
                {
                    p = (unsafe {
                        sqlite3HashFind(
                            (unsafe {
                                std::ptr::addr_of_mut!(
                                    (*unsafe {
                                        (*unsafe { unsafe { (*db).aDb }.offset(i as isize) })
                                            .pSchema
                                    })
                                    .tblHash
                                )
                            }) as *const Hash,
                            (b"sqlite_master\0".as_ptr() as *mut i8) as *const i8,
                        )
                    }) as *mut Table;
                }
            }
        }
    } else {
        // Match against TEMP first
        p = (unsafe {
            sqlite3HashFind(
                (unsafe {
                    std::ptr::addr_of_mut!(
                        (*unsafe {
                            (*unsafe { unsafe { (*db).aDb }.offset((1 as i32) as isize) }).pSchema
                        })
                        .tblHash
                    )
                }) as *const Hash,
                zName,
            )
        }) as *mut Table;
        if p != std::ptr::null_mut::<Table>() {
            return p;
        }
        // The main database is second
        p = (unsafe {
            sqlite3HashFind(
                (unsafe {
                    std::ptr::addr_of_mut!(
                        (*unsafe {
                            (*unsafe { unsafe { (*db).aDb }.offset((0 as i32) as isize) }).pSchema
                        })
                        .tblHash
                    )
                }) as *const Hash,
                zName,
            )
        }) as *mut Table;
        if p != std::ptr::null_mut::<Table>() {
            return p;
        }
        // Attached databases are in order of attachment
        i = 2 as i32;
        '__slate_break_2021: loop {
            if !(i < unsafe { (*db).nDb }) {
                break;
            }
            0 as i32;
            p = (unsafe {
                sqlite3HashFind(
                    (unsafe {
                        std::ptr::addr_of_mut!(
                            (*unsafe {
                                (*unsafe { unsafe { (*db).aDb }.offset(i as isize) }).pSchema
                            })
                            .tblHash
                        )
                    }) as *const Hash,
                    zName,
                )
            }) as *mut Table;
            if p != std::ptr::null_mut::<Table>() {
                break '__slate_break_2021;
            }
            let __v2565: i32 = i;
            let __v2566: i32 = __v2565 + (1 as i32);
            i = __v2566;
        }
        let __v2567: bool;
        if p == std::ptr::null_mut::<Table>() {
            __v2567 = (unsafe {
                sqlite3_strnicmp(
                    zName,
                    (b"sqlite_\0".as_ptr() as *mut i8) as *const i8,
                    7 as i32,
                )
            }) == (0 as i32);
        } else {
            __v2567 = false as bool;
        }
        if __v2567 {
            if (unsafe {
                sqlite3StrICmp(
                    unsafe { zName.offset((7 as i32) as isize) },
                    (unsafe {
                        (b"sqlite_schema\0".as_ptr() as *mut i8).offset((7 as i32) as isize)
                    }) as *const i8,
                )
            }) == (0 as i32)
            {
                p = (unsafe {
                    sqlite3HashFind(
                        (unsafe {
                            std::ptr::addr_of_mut!(
                                (*unsafe {
                                    (*unsafe { unsafe { (*db).aDb }.offset((0 as i32) as isize) })
                                        .pSchema
                                })
                                .tblHash
                            )
                        }) as *const Hash,
                        (b"sqlite_master\0".as_ptr() as *mut i8) as *const i8,
                    )
                }) as *mut Table;
            } else {
                if (unsafe {
                    sqlite3StrICmp(
                        unsafe { zName.offset((7 as i32) as isize) },
                        (unsafe {
                            (b"sqlite_temp_schema\0".as_ptr() as *mut i8)
                                .offset((7 as i32) as isize)
                        }) as *const i8,
                    )
                }) == (0 as i32)
                {
                    p = (unsafe {
                        sqlite3HashFind(
                            (unsafe {
                                std::ptr::addr_of_mut!(
                                    (*unsafe {
                                        (*unsafe {
                                            unsafe { (*db).aDb }.offset((1 as i32) as isize)
                                        })
                                        .pSchema
                                    })
                                    .tblHash
                                )
                            }) as *const Hash,
                            (b"sqlite_temp_master\0".as_ptr() as *mut i8) as *const i8,
                        )
                    }) as *mut Table;
                }
            }
        }
    }
    return p;
}

/// Locate the in-memory structure that describes a particular database
/// table given the name of that table and (optionally) the name of the
/// database containing the table.  Return NULL if not found.  Also leave an
/// error message in pParse->zErrMsg.
///
/// The difference between this routine and sqlite3FindTable() is that this
/// routine leaves an error message in pParse->zErrMsg where
/// sqlite3FindTable() does not.
///
/// # Arguments
///
/// * `pParse` - context in which to report errors
/// * `flags_651` - LOCATE_VIEW or LOCATE_NOERR
/// * `zName` - Name of the table we are looking for
/// * `zDbase` - Name of the database.  Might be NULL
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3LocateTable(
    mut pParse: *mut Parse,
    mut flags_651: u32,
    mut zName: *const i8,
    mut zDbase: *const i8,
) -> *mut Table {
    let mut p: *mut Table = unsafe { std::mem::zeroed() };
    let mut db: *mut sqlite3 = unsafe { (*pParse).db };
    // Read the database schema. If an error occurs, leave an error message
    // and code in pParse and return NULL.
    let __v2568: bool;
    if (unsafe { (*db).mDbFlags }) & ((16 as i32) as u32) == ((0 as i32) as u32) {
        __v2568 = (0 as i32) != unsafe { sqlite3ReadSchema(pParse) };
    } else {
        __v2568 = false as bool;
    }
    if __v2568 {
        return std::ptr::null_mut::<Table>();
    }
    p = sqlite3FindTable(db, zName, zDbase);
    if p == std::ptr::null_mut::<Table>() {
        // If zName is the not the name of a table in the schema created using
        // CREATE, then check to see if it is the name of an virtual table that
        // can be an eponymous virtual table.
        if (((unsafe { (*pParse).prepFlags }) as u32) as i32) & (4 as i32) == (0 as i32)
            && (((unsafe { (*db).init.busy }) as u32) as i32) == (0 as i32)
        {
            let mut pMod: *mut Module = (unsafe {
                sqlite3HashFind(
                    (unsafe { std::ptr::addr_of_mut!((*db).aModule) }) as *const Hash,
                    zName,
                )
            }) as *mut Module;
            let __v2569: bool;
            if pMod == std::ptr::null_mut::<Module>() {
                __v2569 = (unsafe {
                    sqlite3_strnicmp(
                        zName,
                        (b"pragma_\0".as_ptr() as *mut i8) as *const i8,
                        7 as i32,
                    )
                }) == (0 as i32);
            } else {
                __v2569 = false as bool;
            }
            if __v2569 {
                pMod = unsafe { sqlite3PragmaVtabRegister(db, zName) };
            }
            let __v2570: bool;
            if pMod == std::ptr::null_mut::<Module>() {
                __v2570 = (unsafe {
                    sqlite3_strnicmp(
                        zName,
                        (b"json\0".as_ptr() as *mut i8) as *const i8,
                        4 as i32,
                    )
                }) == (0 as i32);
            } else {
                __v2570 = false as bool;
            }
            if __v2570 {
                pMod = unsafe { sqlite3JsonVtabRegister(db, zName) };
            }
            let __v2571: bool;
            if pMod != std::ptr::null_mut::<Module>() {
                __v2571 = (unsafe { sqlite3VtabEponymousTableInit(pParse, pMod) }) != (0 as i32);
            } else {
                __v2571 = false as bool;
            }
            if __v2571 {
                {}
                return unsafe { (*pMod).pEpoTab };
            }
        }
        if flags_651 & ((2 as i32) as u32) != (0 as u32) {
            return std::ptr::null_mut::<Table>();
        }
        unsafe {
            (*pParse)
                .__slate_bits_0
                .__set_checkSchema((1 as i32) as u32);
        }
    } else {
        if (((unsafe { (*p).eTabType }) as u32) as i32) == (1 as i32)
            && (((unsafe { (*pParse).prepFlags }) as u32) as i32) & (4 as i32) != (0 as i32)
        {
            p = std::ptr::null_mut::<Table>();
        }
    }
    if p == std::ptr::null_mut::<Table>() {
        let mut zMsg: *const i8 = (if flags_651 & ((1 as i32) as u32) != (0 as u32) {
            b"no such view\0".as_ptr() as *mut i8
        } else {
            b"no such table\0".as_ptr() as *mut i8
        }) as *const i8;
        if zDbase != std::ptr::null::<i8>() {
            unsafe {
                sqlite3ErrorMsg(
                    pParse,
                    (b"%s: %s.%s\0".as_ptr() as *mut i8) as *const i8,
                    zMsg,
                    zDbase,
                    zName,
                )
            };
        } else {
            unsafe {
                sqlite3ErrorMsg(
                    pParse,
                    (b"%s: %s\0".as_ptr() as *mut i8) as *const i8,
                    zMsg,
                    zName,
                )
            };
        }
    } else {
        0 as i32;
    }
    return p;
}

/// Locate the table identified by *p.
///
/// This is a wrapper around sqlite3LocateTable(). The difference between
/// sqlite3LocateTable() and this function is that this function restricts
/// the search to schema (p->pSchema) if it is not NULL. p->pSchema may be
/// non-NULL if it is part of a view or trigger program definition. See
/// sqlite3FixSrcList() for details.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3LocateTableItem(
    mut pParse: *mut Parse,
    mut flags_659: u32,
    mut p: *mut SrcItem,
) -> *mut Table {
    let mut zDb: *const i8 = unsafe { std::mem::zeroed() };
    if ((unsafe { (*p).fg.__slate_bits_0.__get_fixedSchema() }) as i32) != (0 as i32) {
        let mut iDb: i32 =
            unsafe { sqlite3SchemaToIndex(unsafe { (*pParse).db }, unsafe { (*p).u4.pSchema }) };
        0 as i32;
        zDb = (unsafe {
            (*unsafe { unsafe { (*unsafe { (*pParse).db }).aDb }.offset(iDb as isize) }).zDbSName
        }) as *const i8;
    } else {
        0 as i32;
        zDb = (unsafe { (*p).u4.zDatabase }) as *const i8;
    }
    return sqlite3LocateTable(pParse, flags_659, (unsafe { (*p).zName }) as *const i8, zDb);
}

/// Return the preferred table name for system tables.  Translate legacy
/// names into the new preferred names, as appropriate.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3PreferredTableName(mut zName: *const i8) -> *const i8 {
    if (unsafe {
        sqlite3_strnicmp(
            zName,
            (b"sqlite_\0".as_ptr() as *mut i8) as *const i8,
            7 as i32,
        )
    }) == (0 as i32)
    {
        if (unsafe {
            sqlite3StrICmp(
                unsafe { zName.offset((7 as i32) as isize) },
                (unsafe { (b"sqlite_master\0".as_ptr() as *mut i8).offset((7 as i32) as isize) })
                    as *const i8,
            )
        }) == (0 as i32)
        {
            return (b"sqlite_schema\0".as_ptr() as *mut i8) as *const i8;
        }
        if (unsafe {
            sqlite3StrICmp(
                unsafe { zName.offset((7 as i32) as isize) },
                (unsafe {
                    (b"sqlite_temp_master\0".as_ptr() as *mut i8).offset((7 as i32) as isize)
                }) as *const i8,
            )
        }) == (0 as i32)
        {
            return (b"sqlite_temp_schema\0".as_ptr() as *mut i8) as *const i8;
        }
    }
    return zName;
}

/// Locate the in-memory structure that describes
/// a particular index given the name of that index
/// and the name of the database that contains the index.
/// Return NULL if not found.
///
/// If zDatabase is 0, all databases are searched for the
/// table and the first matching index is returned.  (No checking
/// for duplicate index names is done.)  The search order is
/// TEMP first, then MAIN, then any auxiliary databases added
/// using the ATTACH command.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3FindIndex(
    mut db: *mut sqlite3,
    mut zName: *const i8,
    mut zDb: *const i8,
) -> *mut Index {
    let mut p: *mut Index = std::ptr::null_mut::<Index>();
    let mut i: i32 = 0 as i32;
    // All mutexes are required for schema access.  Make sure we hold them.
    0 as i32;
    i = 0 as i32;
    '__slate_break_2038: loop {
        if !(i < unsafe { (*db).nDb }) {
            break;
        }
        let mut j: i32 = if i < (2 as i32) { i ^ (1 as i32) } else { i }; // Search TEMP before MAIN
        let mut pSchema: *mut Schema =
            unsafe { (*unsafe { unsafe { (*db).aDb }.offset(j as isize) }).pSchema };
        0 as i32;
        let __v2574: bool;
        if zDb != std::ptr::null::<i8>() {
            __v2574 = (unsafe { sqlite3DbIsNamed(db, j, zDb) }) == (0 as i32);
        } else {
            __v2574 = false as bool;
        }
        if __v2574 {
        } else {
            0 as i32;
            p = (unsafe {
                sqlite3HashFind(
                    (unsafe { std::ptr::addr_of_mut!((*pSchema).idxHash) }) as *const Hash,
                    zName,
                )
            }) as *mut Index;
            if p != std::ptr::null_mut::<Index>() {
                break '__slate_break_2038;
            }
        }
        let __v2572: i32 = i;
        let __v2573: i32 = __v2572 + (1 as i32);
        i = __v2573;
    }
    return p;
}

/// Reclaim the memory used by an index
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3FreeIndex(mut db: *mut sqlite3, mut p: *mut Index) {
    unsafe { sqlite3DeleteIndexSamples(db, p) };
    unsafe { sqlite3ExprDelete(db, unsafe { (*p).pPartIdxWhere }) };
    unsafe { sqlite3ExprListDelete(db, unsafe { (*p).aColExpr }) };
    unsafe { sqlite3DbFree(db, (unsafe { (*p).zColAff }) as *mut ()) };
    if ((unsafe { (*p).__slate_bits_0.__get_isResized() }) as i32) != (0 as i32) {
        unsafe { sqlite3DbFree(db, (unsafe { (*p).azColl }) as *mut ()) };
    }
    unsafe { sqlite3DbFree(db, p as *mut ()) };
}

/// For the index called zIdxName which is found in the database iDb,
/// unlike that index from its Table then remove the index from
/// the index hash table and free all memory structures associated
/// with the index.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3UnlinkAndDeleteIndex(
    mut db: *mut sqlite3,
    mut iDb: i32,
    mut zIdxName: *const i8,
) {
    let mut pIndex: *mut Index = unsafe { std::mem::zeroed() };
    let mut pHash: *mut Hash = unsafe { std::mem::zeroed() };
    0 as i32;
    pHash = unsafe {
        std::ptr::addr_of_mut!(
            (*unsafe { (*unsafe { unsafe { (*db).aDb }.offset(iDb as isize) }).pSchema }).idxHash
        )
    };
    pIndex =
        (unsafe { sqlite3HashInsert(pHash, zIdxName, std::ptr::null_mut::<()>()) }) as *mut Index;
    if pIndex != std::ptr::null_mut::<Index>() {
        if (unsafe { (*unsafe { (*pIndex).pTable }).pIndex }) == pIndex {
            unsafe {
                (*unsafe { (*pIndex).pTable }).pIndex = unsafe { (*pIndex).pNext };
            }
        } else {
            let mut p: *mut Index = unsafe { std::mem::zeroed() };
            // Justification of ALWAYS();  The index must be on the list of
            // indices.
            p = unsafe { (*unsafe { (*pIndex).pTable }).pIndex };
            '__slate_break_2039: while p != std::ptr::null_mut::<Index>()
                && (unsafe { (*p).pNext }) != pIndex
            {
                p = unsafe { (*p).pNext };
            }
            if p != std::ptr::null_mut::<Index>() && (unsafe { (*p).pNext }) == pIndex {
                unsafe {
                    (*p).pNext = unsafe { (*pIndex).pNext };
                }
            }
        }
        sqlite3FreeIndex(db, pIndex);
    }
    let __v2578: *mut sqlite3 = db;
    let __v2579: u32 = unsafe { (*__v2578).mDbFlags };
    let __v2580: u32 = __v2579 | ((1 as i32) as u32);
    unsafe {
        (*__v2578).mDbFlags = __v2580;
    }
}

/// Look through the list of open database files in db->aDb[] and if
/// any have been closed, remove them from the list.  Reallocate the
/// db->aDb[] structure to a smaller size, if possible.
///
/// Entry 0 (the "main" database) and entry 1 (the "temp" database)
/// are never candidates for being collapsed.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3CollapseDatabaseArray(mut db: *mut sqlite3) {
    let mut i: i32 = 0 as i32;
    let mut j: i32 = 0 as i32;
    j = 2 as i32;
    i = 2 as i32;
    '__slate_break_2040: loop {
        if !(i < unsafe { (*db).nDb }) {
            break;
        }
        let mut pDb: *mut Db = unsafe { unsafe { (*db).aDb }.offset(i as isize) };
        if (unsafe { (*pDb).pBt }) == std::ptr::null_mut::<Btree>() {
            unsafe { sqlite3DbFree(db, (unsafe { (*pDb).zDbSName }) as *mut ()) };
            unsafe {
                (*pDb).zDbSName = std::ptr::null_mut::<i8>();
            }
        } else {
            if j < i {
                unsafe {
                    *unsafe { unsafe { (*db).aDb }.offset(j as isize) } =
                        unsafe { *unsafe { unsafe { (*db).aDb }.offset(i as isize) } };
                }
            }
            let __v2290: i32 = j;
            let __v2291: i32 = __v2290 + (1 as i32);
            j = __v2291;
        }
        let __v2288: i32 = i;
        let __v2289: i32 = __v2288 + (1 as i32);
        i = __v2289;
    }
    unsafe {
        (*db).nDb = j;
    }
    if (unsafe { (*db).nDb }) <= (2 as i32)
        && (unsafe { (*db).aDb }) != unsafe { (*db).aDbStatic.as_mut_ptr() as *mut Db }
    {
        unsafe {
            memcpy(
                (unsafe { (*db).aDbStatic.as_mut_ptr() as *mut Db }) as *mut (),
                (unsafe { (*db).aDb }) as *const (),
                (((2 as i32) as i64) as u64).wrapping_mul(32 as u64),
            )
        };
        unsafe { sqlite3DbFree(db, (unsafe { (*db).aDb }) as *mut ()) };
        unsafe {
            (*db).aDb = unsafe { (*db).aDbStatic.as_mut_ptr() as *mut Db };
        }
    }
}

/// Reset the schema for the database at index iDb.  Also reset the
/// TEMP schema.  The reset is deferred if db->nSchemaLock is not zero.
/// Deferred resets may be run by calling with iDb<0.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3ResetOneSchema(mut db: *mut sqlite3, mut iDb: i32) {
    let mut i: i32 = 0 as i32;
    0 as i32;
    if iDb >= (0 as i32) {
        0 as i32;
        let __v2277: *mut Schema =
            unsafe { (*unsafe { unsafe { (*db).aDb }.offset(iDb as isize) }).pSchema };
        let __v2278: u16 = unsafe { (*__v2277).schemaFlags };
        let __v2279: u16 = ((((__v2278 as u32) as i32) | (8 as i32)) as i16) as u16;
        unsafe {
            (*__v2277).schemaFlags = __v2279;
        }
        let __v2280: *mut Schema =
            unsafe { (*unsafe { unsafe { (*db).aDb }.offset((1 as i32) as isize) }).pSchema };
        let __v2281: u16 = unsafe { (*__v2280).schemaFlags };
        let __v2282: u16 = ((((__v2281 as u32) as i32) | (8 as i32)) as i16) as u16;
        unsafe {
            (*__v2280).schemaFlags = __v2282;
        }
        let __v2283: *mut sqlite3 = db;
        let __v2284: u32 = unsafe { (*__v2283).mDbFlags };
        let __v2285: u32 = __v2284 & (!(16 as i32) as u32);
        unsafe {
            (*__v2283).mDbFlags = __v2285;
        }
    }
    if (unsafe { (*db).nSchemaLock }) == ((0 as i32) as u32) {
        i = 0 as i32;
        '__slate_break_2041: loop {
            if !(i < unsafe { (*db).nDb }) {
                break;
            }
            if (((unsafe {
                (*unsafe { (*unsafe { unsafe { (*db).aDb }.offset(i as isize) }).pSchema })
                    .schemaFlags
            }) as u32) as i32)
                & (8 as i32)
                == (8 as i32)
            {
                unsafe {
                    sqlite3SchemaClear(
                        (unsafe { (*unsafe { unsafe { (*db).aDb }.offset(i as isize) }).pSchema })
                            as *mut (),
                    )
                };
            }
            let __v2286: i32 = i;
            let __v2287: i32 = __v2286 + (1 as i32);
            i = __v2287;
        }
    }
}

/// Erase all schema information from all attached databases (including
/// "main" and "temp") for a single database connection.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3ResetAllSchemasOfConnection(mut db: *mut sqlite3) {
    let mut i: i32 = 0 as i32;
    unsafe { sqlite3BtreeEnterAll(db) };
    i = 0 as i32;
    '__slate_break_2042: loop {
        if !(i < unsafe { (*db).nDb }) {
            break;
        }
        let mut pDb: *mut Db = unsafe { unsafe { (*db).aDb }.offset(i as isize) };
        if (unsafe { (*pDb).pSchema }) != std::ptr::null_mut::<Schema>() {
            if (unsafe { (*db).nSchemaLock }) == ((0 as i32) as u32) {
                unsafe { sqlite3SchemaClear((unsafe { (*pDb).pSchema }) as *mut ()) };
            } else {
                let __v2271: *mut Schema =
                    unsafe { (*unsafe { unsafe { (*db).aDb }.offset(i as isize) }).pSchema };
                let __v2272: u16 = unsafe { (*__v2271).schemaFlags };
                let __v2273: u16 = ((((__v2272 as u32) as i32) | (8 as i32)) as i16) as u16;
                unsafe {
                    (*__v2271).schemaFlags = __v2273;
                }
            }
        }
        let __v2269: i32 = i;
        let __v2270: i32 = __v2269 + (1 as i32);
        i = __v2270;
    }
    let __v2274: *mut sqlite3 = db;
    let __v2275: u32 = unsafe { (*__v2274).mDbFlags };
    let __v2276: u32 = __v2275 & (!((1 as i32) | (16 as i32)) as u32);
    unsafe {
        (*__v2274).mDbFlags = __v2276;
    }
    unsafe { sqlite3VtabUnlockList(db) };
    unsafe { sqlite3BtreeLeaveAll(db) };
    if (unsafe { (*db).nSchemaLock }) == ((0 as i32) as u32) {
        sqlite3CollapseDatabaseArray(db);
    }
}

/// This routine is called when a commit occurs.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3CommitInternalChanges(mut db: *mut sqlite3) {
    let __v2292: *mut sqlite3 = db;
    let __v2293: u32 = unsafe { (*__v2292).mDbFlags };
    let __v2294: u32 = __v2293 & (!(1 as i32) as u32);
    unsafe {
        (*__v2292).mDbFlags = __v2294;
    }
}

/// Set the expression associated with a column.  This is usually
/// the DEFAULT value, but might also be the expression that computes
/// the value for a generated column.
///
/// # Arguments
///
/// * `pParse` - Parsing context
/// * `pTab` - The table containing the column
/// * `pCol` - The column to receive the new DEFAULT expression
/// * `pExpr` - The new default expression
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3ColumnSetExpr(
    mut pParse: *mut Parse,
    mut pTab: *mut Table,
    mut pCol: *mut Column,
    mut pExpr: *mut Expr,
) {
    let mut pList: *mut ExprList = unsafe { std::mem::zeroed() };
    0 as i32;
    pList = unsafe { (*pTab).u.tab.pDfltList };
    if (((unsafe { (*pCol).iDflt }) as u32) as i32) == (0 as i32)
        || pList == std::ptr::null_mut::<ExprList>()
        || (unsafe { (*pList).nExpr }) < (((unsafe { (*pCol).iDflt }) as u32) as i32)
    {
        unsafe {
            (*pCol).iDflt = ((if pList == std::ptr::null_mut::<ExprList>() {
                1 as i32
            } else {
                (unsafe { (*pList).nExpr }) + (1 as i32)
            }) as i16) as u16;
        }
        unsafe {
            (*pTab).u.tab.pDfltList = unsafe { sqlite3ExprListAppend(pParse, pList, pExpr) };
        }
    } else {
        unsafe {
            sqlite3ExprDelete(unsafe { (*pParse).db }, unsafe {
                (*unsafe {
                    unsafe { std::ptr::addr_of_mut!((*pList).a) as *mut ExprList_item }.offset(
                        ((((unsafe { (*pCol).iDflt }) as u32) as i32) - (1 as i32)) as isize,
                    )
                })
                .pExpr
            })
        };
        unsafe {
            (*unsafe {
                unsafe { std::ptr::addr_of_mut!((*pList).a) as *mut ExprList_item }
                    .offset(((((unsafe { (*pCol).iDflt }) as u32) as i32) - (1 as i32)) as isize)
            })
            .pExpr = pExpr;
        }
    }
}

/// Return the expression associated with a column.  The expression might be
/// the DEFAULT clause or the AS clause of a generated column.
/// Return NULL if the column has no associated expression.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3ColumnExpr(mut pTab: *mut Table, mut pCol: *mut Column) -> *mut Expr {
    if (((unsafe { (*pCol).iDflt }) as u32) as i32) == (0 as i32) {
        return std::ptr::null_mut::<Expr>();
    }
    if !((((unsafe { (*pTab).eTabType }) as u32) as i32) == (0 as i32)) {
        return std::ptr::null_mut::<Expr>();
    }
    if (unsafe { (*pTab).u.tab.pDfltList }) == std::ptr::null_mut::<ExprList>() {
        return std::ptr::null_mut::<Expr>();
    }
    if (unsafe { (*unsafe { (*pTab).u.tab.pDfltList }).nExpr })
        < (((unsafe { (*pCol).iDflt }) as u32) as i32)
    {
        return std::ptr::null_mut::<Expr>();
    }
    return unsafe {
        (*unsafe {
            unsafe {
                std::ptr::addr_of_mut!((*unsafe { (*pTab).u.tab.pDfltList }).a)
                    as *mut ExprList_item
            }
            .offset(((((unsafe { (*pCol).iDflt }) as u32) as i32) - (1 as i32)) as isize)
        })
        .pExpr
    };
}

// Suppress false-positive warning message generated with -O3 in GCC
// on the second call to sqlite3Strlen30() in the sqlite3ColumnSetColl()
// function below.  See the forum thread beginning on 2026-05-10T01:11:22Z.
//
// See also the "pop" pragma to undo this warning suppression immediately
// after the function.
/// Set the collating sequence name for a column.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3ColumnSetColl(
    mut db: *mut sqlite3,
    mut pCol: *mut Column,
    mut zColl: *const i8,
) {
    let mut nColl: i64 = 0 as i64;
    let mut n: i64 = 0 as i64;
    let mut zNew: *mut i8 = unsafe { std::mem::zeroed() };
    0 as i32;
    n = ((unsafe { sqlite3Strlen30((unsafe { (*pCol).zCnName }) as *const i8) }) + (1 as i32))
        as i64;
    if (((unsafe { (*pCol).colFlags }) as u32) as i32) & (4 as i32) != (0 as i32) {
        let __v2295: i64 = n;
        let __v2296: i64 = __v2295
            + (((unsafe {
                sqlite3Strlen30(
                    (unsafe { unsafe { (*pCol).zCnName }.offset(n as isize) }) as *const i8,
                )
            }) + (1 as i32)) as i64);
        n = __v2296;
    }
    nColl = ((unsafe { sqlite3Strlen30(zColl) }) + (1 as i32)) as i64;
    zNew = (unsafe {
        sqlite3DbRealloc(
            db,
            (unsafe { (*pCol).zCnName }) as *mut (),
            (nColl + n) as u64,
        )
    }) as *mut i8;
    if zNew != std::ptr::null_mut::<i8>() {
        unsafe {
            (*pCol).zCnName = zNew;
        }
        unsafe {
            memcpy(
                (unsafe { unsafe { (*pCol).zCnName }.offset(n as isize) }) as *mut (),
                zColl as *const (),
                nColl as u64,
            )
        };
        let __v2297: *mut Column = pCol;
        let __v2298: u16 = unsafe { (*__v2297).colFlags };
        let __v2299: u16 = ((((__v2298 as u32) as i32) | (512 as i32)) as i16) as u16;
        unsafe {
            (*__v2297).colFlags = __v2299;
        }
    }
}

// Undo the false-positive warning suppression above.
/// Return the collating sequence name for a column
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3ColumnColl(mut pCol: *mut Column) -> *const i8 {
    let mut z: *const i8 = unsafe { std::mem::zeroed() };
    if (((unsafe { (*pCol).colFlags }) as u32) as i32) & (512 as i32) == (0 as i32) {
        return std::ptr::null::<i8>();
    }
    z = (unsafe { (*pCol).zCnName }) as *const i8;
    '__slate_break_2043: while (unsafe { *z }) != (0 as i8) {
        let __v2300: *const i8 = z;
        let __v2301: *const i8 = unsafe { __v2300.offset((1 as i32) as isize) };
        z = __v2301;
    }
    if (((unsafe { (*pCol).colFlags }) as u32) as i32) & (4 as i32) != (0 as i32) {
        '__slate_break_2044: loop {
            let __v2302: *const i8 = z;
            let __v2303: *const i8 = unsafe { __v2302.offset((1 as i32) as isize) };
            z = __v2303;
            if !((unsafe { *z }) != (0 as i8)) {
                break;
            }
        }
    }
    return unsafe { z.offset((1 as i32) as isize) };
}

/// Delete memory allocated for the column names of a table or view (the
/// Table.aCol[] array).
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3DeleteColumnNames(mut db: *mut sqlite3, mut pTable: *mut Table) {
    let mut i: i32 = 0 as i32;
    let mut pCol: *mut Column = unsafe { std::mem::zeroed() };
    0 as i32;
    0 as i32;
    let __v2304: *mut Column = unsafe { (*pTable).aCol };
    pCol = __v2304;
    if __v2304 != std::ptr::null_mut::<Column>() {
        i = 0 as i32;
        '__slate_break_2045: while i < ((unsafe { (*pTable).nCol }) as i32) {
            0 as i32;
            unsafe { sqlite3DbFree(db, (unsafe { (*pCol).zCnName }) as *mut ()) };
            let __v2305: i32 = i;
            let __v2306: i32 = __v2305 + (1 as i32);
            i = __v2306;
            let __v2307: *mut Column = pCol;
            let __v2308: *mut Column = unsafe { __v2307.offset((1 as i32) as isize) };
            pCol = __v2308;
        }
        unsafe { sqlite3DbNNFreeNN(db, (unsafe { (*pTable).aCol }) as *mut ()) };
        if (((unsafe { (*pTable).eTabType }) as u32) as i32) == (0 as i32) {
            unsafe { sqlite3ExprListDelete(db, unsafe { (*pTable).u.tab.pDfltList }) };
        }
        if (unsafe { (*db).pnBytesFreed }) == std::ptr::null_mut::<i32>() {
            unsafe {
                (*pTable).aCol = std::ptr::null_mut::<Column>();
            }
            unsafe {
                (*pTable).nCol = (0 as i32) as i16;
            }
            if (((unsafe { (*pTable).eTabType }) as u32) as i32) == (0 as i32) {
                unsafe {
                    (*pTable).u.tab.pDfltList = std::ptr::null_mut::<ExprList>();
                }
            }
        }
    }
}

/// Remove the memory data structures associated with the given
/// Table.  No changes are made to disk by this routine.
///
/// This routine just deletes the data structure.  It does not unlink
/// the table data structure from the hash table.  But it does destroy
/// memory structures of the indices and foreign keys associated with
/// the table.
///
/// The db parameter is optional.  It is needed if the Table object
/// contains lookaside memory.  (Table objects in the schema do not use
/// lookaside memory, but some ephemeral Table objects do.)  Or the
/// db parameter can be used with db->pnBytesFreed to measure the memory
/// used by the Table object.
fn deleteTable(mut db: *mut sqlite3, mut pTable: *mut Table) {
    let mut pIndex: *mut Index = unsafe { std::mem::zeroed() };
    let mut pNext: *mut Index = unsafe { std::mem::zeroed() };
    // Delete all indices associated with this table.
    pIndex = unsafe { (*pTable).pIndex };
    '__slate_break_2046: while pIndex != std::ptr::null_mut::<Index>() {
        pNext = unsafe { (*pIndex).pNext };
        0 as i32;
        if (unsafe { (*db).pnBytesFreed }) == std::ptr::null_mut::<i32>()
            && !((((unsafe { (*pTable).eTabType }) as u32) as i32) == (1 as i32))
        {
            let mut zName: *mut i8 = unsafe { (*pIndex).zName };
            unsafe {
                sqlite3HashInsert(
                    unsafe { std::ptr::addr_of_mut!((*unsafe { (*pIndex).pSchema }).idxHash) },
                    zName as *const i8,
                    std::ptr::null_mut::<()>(),
                )
            };
            0 as i32;
            0 as i32;
        }
        sqlite3FreeIndex(db, pIndex);
        pIndex = pNext;
    }
    if (((unsafe { (*pTable).eTabType }) as u32) as i32) == (0 as i32) {
        unsafe { sqlite3FkDelete(db, pTable) };
    } else {
        if (((unsafe { (*pTable).eTabType }) as u32) as i32) == (1 as i32) {
            unsafe { sqlite3VtabClear(db, pTable) };
        } else {
            0 as i32;
            unsafe { sqlite3SelectDelete(db, unsafe { (*pTable).u.view.pSelect }) };
        }
    }
    // Delete the Table structure itself.
    sqlite3DeleteColumnNames(db, pTable);
    unsafe { sqlite3DbFree(db, (unsafe { (*pTable).zName }) as *mut ()) };
    unsafe { sqlite3DbFree(db, (unsafe { (*pTable).zColAff }) as *mut ()) };
    unsafe { sqlite3ExprListDelete(db, unsafe { (*pTable).pCheck }) };
    unsafe { sqlite3DbFree(db, pTable as *mut ()) };
    // Verify that no lookaside memory was used by schema tables
    0 as i32;
}

#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3DeleteTable(mut db: *mut sqlite3, mut pTable: *mut Table) {
    // Do not delete the table until the reference count reaches zero.
    0 as i32;
    if !(pTable != std::ptr::null_mut::<Table>()) {
        return;
    }
    let __v2456: bool;
    if (unsafe { (*db).pnBytesFreed }) == std::ptr::null_mut::<i32>() {
        let __v2457: *mut Table = pTable;
        let __v2458: u32 = unsafe { (*__v2457).nTabRef };
        let __v2459: u32 = __v2458.wrapping_sub((1 as i32) as u32);
        unsafe {
            (*__v2457).nTabRef = __v2459;
        }
        __v2456 = __v2459 > ((0 as i32) as u32);
    } else {
        __v2456 = false as bool;
    }
    if __v2456 {
        return;
    }
    deleteTable(db, pTable);
}

#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.build.sqlite3DeleteTableGeneric")]
extern "C-unwind" fn sqlite3DeleteTableGeneric(mut db: *mut sqlite3, mut pTable: *mut ()) {
    sqlite3DeleteTable(db, pTable as *mut Table);
}

/// Unlink the given table from the hash tables and the delete the
/// table structure with all its indices and foreign keys.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3UnlinkAndDeleteTable(
    mut db: *mut sqlite3,
    mut iDb: i32,
    mut zTabName: *const i8,
) {
    let mut p: *mut Table = unsafe { std::mem::zeroed() };
    let mut pDb: *mut Db = unsafe { std::mem::zeroed() };
    0 as i32;
    0 as i32;
    0 as i32;
    0 as i32;
    {}
    // Zero-length table names are allowed
    pDb = unsafe { unsafe { (*db).aDb }.offset(iDb as isize) };
    p = (unsafe {
        sqlite3HashInsert(
            unsafe { std::ptr::addr_of_mut!((*unsafe { (*pDb).pSchema }).tblHash) },
            zTabName,
            std::ptr::null_mut::<()>(),
        )
    }) as *mut Table;
    sqlite3DeleteTable(db, p);
    let __v2575: *mut sqlite3 = db;
    let __v2576: u32 = unsafe { (*__v2575).mDbFlags };
    let __v2577: u32 = __v2576 | ((1 as i32) as u32);
    unsafe {
        (*__v2575).mDbFlags = __v2577;
    }
}

/// Given a token, return a string that consists of the text of that
/// token.  Space to hold the returned string
/// is obtained from sqliteMalloc() and must be freed by the calling
/// function.
///
/// Any quotation marks (ex:  "name", 'name', [name], or `name`) that
/// surround the body of the token are removed.
///
/// Tokens are often just pointers into the original SQL text and so
/// are not \000 terminated and are not persistent.  The returned string
/// is \000 terminated and is persistent.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3NameFromToken(
    mut db: *mut sqlite3,
    mut pName: *const Token,
) -> *mut i8 {
    let mut zName: *mut i8 = unsafe { std::mem::zeroed() };
    if pName != std::ptr::null::<Token>() {
        zName =
            unsafe { sqlite3DbStrNDup(db, unsafe { (*pName).z }, (unsafe { (*pName).n }) as u64) };
        unsafe { sqlite3Dequote(zName) };
    } else {
        zName = std::ptr::null_mut::<i8>();
    }
    return zName;
}

/// Open the sqlite_schema table stored in database number iDb for
/// writing. The table is opened using cursor 0.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3OpenSchemaTable(mut p: *mut Parse, mut iDb: i32) {
    let mut v: *mut Vdbe = unsafe { sqlite3GetVdbe(p) };
    sqlite3TableLock(
        p,
        iDb,
        (1 as i32) as u32,
        ((1 as i32) as i8) as u8,
        (b"sqlite_master\0".as_ptr() as *mut i8) as *const i8,
    );
    unsafe { sqlite3VdbeAddOp4Int(v, 116 as i32, 0 as i32, 1 as i32, iDb, 5 as i32) };
    if (unsafe { (*p).nTab }) == (0 as i32) {
        unsafe {
            (*p).nTab = 1 as i32;
        }
    }
}

/// Parameter zName points to a nul-terminated buffer containing the name
/// of a database ("main", "temp" or the name of an attached db). This
/// function returns the index of the named database in db->aDb[], or
/// -1 if the named db cannot be found.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3FindDbName(mut db: *mut sqlite3, mut zName: *const i8) -> i32 {
    let mut i: i32 = -(1 as i32); // Database number
    if zName != std::ptr::null::<i8>() {
        let mut pDb: *mut Db = unsafe { std::mem::zeroed() };
        i = (unsafe { (*db).nDb }) - (1 as i32);
        let __v2647: *mut Db = unsafe { unsafe { (*db).aDb }.offset(i as isize) };
        pDb = __v2647;
        '__slate_break_2048: while i >= (0 as i32) {
            if (0 as i32)
                == unsafe { sqlite3_stricmp((unsafe { (*pDb).zDbSName }) as *const i8, zName) }
            {
                break '__slate_break_2048;
            }
            // "main" is always an acceptable alias for the primary database
            // even if it has been renamed using SQLITE_DBCONFIG_MAINDBNAME.
            let __v2652: bool;
            if i == (0 as i32) {
                __v2652 = (0 as i32)
                    == unsafe {
                        sqlite3_stricmp((b"main\0".as_ptr() as *mut i8) as *const i8, zName)
                    };
            } else {
                __v2652 = false as bool;
            }
            if __v2652 {
                break '__slate_break_2048;
            }
            let __v2648: i32 = i;
            let __v2649: i32 = __v2648 - (1 as i32);
            i = __v2649;
            let __v2650: *mut Db = pDb;
            let __v2651: *mut Db = unsafe { __v2650.offset(-((1 as i32) as isize)) };
            pDb = __v2651;
        }
    }
    return i;
}

/// The token *pName contains the name of a database (either "main" or
/// "temp" or the name of an attached db). This routine returns the
/// index of the named database in db->aDb[], or -1 if the named db
/// does not exist.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3FindDb(mut db: *mut sqlite3, mut pName: *mut Token) -> i32 {
    let mut i: i32 = 0 as i32; // Database number
    let mut zName: *mut i8 = unsafe { std::mem::zeroed() }; // Name we are searching for
    zName = sqlite3NameFromToken(db, pName as *const Token);
    i = sqlite3FindDbName(db, zName as *const i8);
    unsafe { sqlite3DbFree(db, zName as *mut ()) };
    return i;
}

/// The table or view or trigger name is passed to this routine via tokens
/// pName1 and pName2. If the table name was fully qualified, for example:
///
/// CREATE TABLE xxx.yyy (...);
///
/// Then pName1 is set to "xxx" and pName2 "yyy". On the other hand if
/// the table name is not fully qualified, i.e.:
///
/// CREATE TABLE yyy(...);
///
/// Then pName1 is set to "yyy" and pName2 is "".
///
/// This routine sets the *ppUnqual pointer to point at the token (pName1 or
/// pName2) that stores the unqualified table name.  The index of the
/// database "xxx" is returned.
///
/// # Arguments
///
/// * `pParse` - Parsing and code generating context
/// * `pName1` - The "xxx" in the name "xxx.yyy" or "xxx"
/// * `pName2` - The "yyy" in the name "xxx.yyy"
/// * `pUnqual` - Write the unqualified object name here
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3TwoPartName(
    mut pParse: *mut Parse,
    mut pName1: *mut Token,
    mut pName2: *mut Token,
    mut pUnqual: *mut *mut Token,
) -> i32 {
    let mut iDb: i32 = 0 as i32; // Database holding the object
    let mut db: *mut sqlite3 = unsafe { (*pParse).db };
    0 as i32;
    if (unsafe { (*pName2).n }) > ((0 as i32) as u32) {
        if (unsafe { (*db).init.busy }) != (0 as u8) {
            unsafe {
                sqlite3ErrorMsg(
                    pParse,
                    (b"corrupt database\0".as_ptr() as *mut i8) as *const i8,
                )
            };
            return -(1 as i32);
        }
        unsafe {
            *pUnqual = pName2;
        }
        iDb = sqlite3FindDb(db, pName1);
        if iDb < (0 as i32) {
            unsafe {
                sqlite3ErrorMsg(
                    pParse,
                    (b"unknown database %T\0".as_ptr() as *mut i8) as *const i8,
                    pName1,
                )
            };
            return -(1 as i32);
        }
    } else {
        0 as i32;
        iDb = ((unsafe { (*db).init.iDb }) as u32) as i32;
        unsafe {
            *pUnqual = pName1;
        }
    }
    return iDb;
}

/// True if PRAGMA writable_schema is ON
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3WritableSchema(mut db: *mut sqlite3) -> i32 {
    {}
    {}
    {}
    {}
    return ((unsafe { (*db).flags }) & ((((1 as i32) | (268435456 as i32)) as i64) as u64)
        == (((1 as i32) as i64) as u64)) as i32;
}

/// This routine is used to check if the UTF-8 string zName is a legal
/// unqualified name for a new schema object (table, index, view or
/// trigger). All names are legal except those that begin with the string
/// "sqlite_" (in upper, lower or mixed case). This portion of the namespace
/// is reserved for internal use.
///
/// When parsing the sqlite_schema table, this routine also checks to
/// make sure the "type", "name", and "tbl_name" columns are consistent
/// with the SQL.
///
/// # Arguments
///
/// * `pParse` - Parsing context
/// * `zName` - Name of the object to check
/// * `zType` - Type of this object
/// * `zTblName` - Parent table name for triggers and indexes
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3CheckObjectName(
    mut pParse: *mut Parse,
    mut zName: *const i8,
    mut zType: *const i8,
    mut zTblName: *const i8,
) -> i32 {
    let mut db: *mut sqlite3 = unsafe { (*pParse).db };
    if sqlite3WritableSchema(db) != (0 as i32)
        || ((unsafe { (*db).init.__slate_bits_0.__get_imposterTable() }) as i32) != (0 as i32)
        || !((unsafe { sqlite3Config.bExtraSchemaChecks }) != (0 as u8))
    {
        // Skip these error checks for writable_schema=ON
        return 0 as i32;
    }
    if (unsafe { (*db).init.busy }) != (0 as u8) {
        let __v2613: bool;
        if (unsafe {
            sqlite3_stricmp(zType, unsafe {
                *unsafe { unsafe { (*db).init.azInit }.offset((0 as i32) as isize) }
            })
        }) != (0 as i32)
        {
            __v2613 = true as bool;
        } else {
            __v2613 = (unsafe {
                sqlite3_stricmp(zName, unsafe {
                    *unsafe { unsafe { (*db).init.azInit }.offset((1 as i32) as isize) }
                })
            }) != (0 as i32);
        }
        let __v2614: bool;
        if __v2613 {
            __v2614 = true as bool;
        } else {
            __v2614 = (unsafe {
                sqlite3_stricmp(zTblName, unsafe {
                    *unsafe { unsafe { (*db).init.azInit }.offset((2 as i32) as isize) }
                })
            }) != (0 as i32);
        }
        if __v2614 {
            unsafe { sqlite3ErrorMsg(pParse, (b"\0".as_ptr() as *mut i8) as *const i8) }; // corruptSchema() will supply the error
            return 1 as i32;
        }
    } else {
        let __v2615: bool;
        if (((unsafe { (*pParse).nested }) as u32) as i32) == (0 as i32) {
            __v2615 = (0 as i32)
                == unsafe {
                    sqlite3_strnicmp(
                        zName,
                        (b"sqlite_\0".as_ptr() as *mut i8) as *const i8,
                        7 as i32,
                    )
                };
        } else {
            __v2615 = false as bool;
        }
        let __v2616: bool;
        if __v2615 {
            __v2616 = true as bool;
        } else {
            let __v2617: bool;
            if sqlite3ReadOnlyShadowTables(db) != (0 as i32) {
                __v2617 = sqlite3ShadowTableName(db, zName) != (0 as i32);
            } else {
                __v2617 = false as bool;
            }
            __v2616 = __v2617;
        }
        if __v2616 {
            unsafe {
                sqlite3ErrorMsg(
                    pParse,
                    (b"object name reserved for internal use: %s\0".as_ptr() as *mut i8)
                        as *const i8,
                    zName,
                )
            };
            return 1 as i32;
        }
    }
    return 0 as i32;
}

/// Return the PRIMARY KEY index of a table
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3PrimaryKeyIndex(mut pTab: *mut Table) -> *mut Index {
    let mut p: *mut Index = unsafe { std::mem::zeroed() };
    p = unsafe { (*pTab).pIndex };
    '__slate_break_2055: while p != std::ptr::null_mut::<Index>()
        && !(((unsafe { (*p).__slate_bits_0.__get_idxType() }) as i32) == (2 as i32))
    {
        p = unsafe { (*p).pNext };
    }
    return p;
}

/// Convert an table column number into a index column number.  That is,
/// for the column iCol in the table (as defined by the CREATE TABLE statement)
/// find the (first) offset of that column in index pIdx.  Or return -1
/// if column iCol is not used in index pIdx.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3TableColumnToIndex(mut pIdx: *mut Index, mut iCol: i32) -> i32 {
    let mut i: i32 = 0 as i32;
    let mut iCol16: i16 = 0 as i16;
    0 as i32;
    0 as i32;
    iCol16 = iCol as i16;
    i = 0 as i32;
    '__slate_break_2056: loop {
        if !(i < (((unsafe { (*pIdx).nColumn }) as u32) as i32)) {
            break;
        }
        if (iCol16 as i32)
            == ((unsafe { *unsafe { unsafe { (*pIdx).aiColumn }.offset(i as isize) } }) as i32)
        {
            return i;
        }
        let __v2309: i32 = i;
        let __v2310: i32 = __v2309 + (1 as i32);
        i = __v2310;
    }
    return -(1 as i32);
}

/// Convert a storage column number into a table column number.
///
/// The storage column number (0,1,2,....) is the index of the value
/// as it appears in the record on disk.  The true column number
/// is the index (0,1,2,...) of the column in the CREATE TABLE statement.
///
/// The storage column number is less than the table column number if
/// and only there are VIRTUAL columns to the left.
///
/// If SQLITE_OMIT_GENERATED_COLUMNS, this routine is a no-op macro.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3StorageColumnToTable(mut pTab: *mut Table, mut iCol: i16) -> i16 {
    if (unsafe { (*pTab).tabFlags }) & ((32 as i32) as u32) != (0 as u32) {
        let mut i: i32 = 0 as i32;
        i = 0 as i32;
        '__slate_break_2057: loop {
            if !(i <= (iCol as i32)) {
                break;
            }
            if (((unsafe { (*unsafe { unsafe { (*pTab).aCol }.offset(i as isize) }).colFlags })
                as u32) as i32)
                & (32 as i32)
                != (0 as i32)
            {
                let __v2317: i16 = iCol;
                let __v2318: i16 = ((__v2317 as i32) + (1 as i32)) as i16;
                iCol = __v2318;
            }
            let __v2315: i32 = i;
            let __v2316: i32 = __v2315 + (1 as i32);
            i = __v2316;
        }
    }
    return iCol;
}

/// Convert a table column number into a storage column number.
///
/// The storage column number (0,1,2,....) is the index of the value
/// as it appears in the record on disk.  Or, if the input column is
/// the N-th virtual column (zero-based) then the storage number is
/// the number of non-virtual columns in the table plus N.
///
/// The true column number is the index (0,1,2,...) of the column in
/// the CREATE TABLE statement.
///
/// If the input column is a VIRTUAL column, then it should not appear
/// in storage.  But the value sometimes is cached in registers that
/// follow the range of registers used to construct storage.  This
/// avoids computing the same VIRTUAL column multiple times, and provides
/// values for use by OP_Param opcodes in triggers.  Hence, if the
/// input column is a VIRTUAL table, put it after all the other columns.
///
/// In the following, N means "normal column", S means STORED, and
/// V means VIRTUAL.  Suppose the CREATE TABLE has columns like this:
///
///        CREATE TABLE ex(N,S,V,N,S,V,N,S,V);
///                     -- 0 1 2 3 4 5 6 7 8
///
/// Then the mapping from this function is as follows:
///
///    INPUTS:     0 1 2 3 4 5 6 7 8
///    OUTPUTS:    0 1 6 2 3 7 4 5 8
///
/// So, in other words, this routine shifts all the virtual columns to
/// the end.
///
/// If SQLITE_OMIT_GENERATED_COLUMNS then there are no virtual columns and
/// this routine is a no-op macro.  If the pTab does not have any virtual
/// columns, then this routine is no-op that always return iCol.  If iCol
/// is negative (indicating the ROWID column) then this routine return iCol.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3TableColumnToStorage(mut pTab: *mut Table, mut iCol: i16) -> i16 {
    let mut i: i32 = 0 as i32;
    let mut n: i16 = 0 as i16;
    0 as i32;
    if (unsafe { (*pTab).tabFlags }) & ((32 as i32) as u32) == ((0 as i32) as u32)
        || (iCol as i32) < (0 as i32)
    {
        return iCol;
    }
    i = 0 as i32;
    n = (0 as i32) as i16;
    '__slate_break_2058: loop {
        if !(i < (iCol as i32)) {
            break;
        }
        if (((unsafe { (*unsafe { unsafe { (*pTab).aCol }.offset(i as isize) }).colFlags }) as u32)
            as i32)
            & (32 as i32)
            == (0 as i32)
        {
            let __v2313: i16 = n;
            let __v2314: i16 = ((__v2313 as i32) + (1 as i32)) as i16;
            n = __v2314;
        }
        let __v2311: i32 = i;
        let __v2312: i32 = __v2311 + (1 as i32);
        i = __v2312;
    }
    if (((unsafe { (*unsafe { unsafe { (*pTab).aCol }.offset(i as isize) }).colFlags }) as u32)
        as i32)
        & (32 as i32)
        != (0 as i32)
    {
        // iCol is a virtual column itself
        return (((unsafe { (*pTab).nNVCol }) as i32) + i - (n as i32)) as i16;
    } else {
        // iCol is a normal or stored column
        return n;
    }
    return unsafe { std::mem::zeroed() };
}

/// Insert a single OP_JournalMode query opcode in order to force the
/// prepared statement to return false for sqlite3_stmt_readonly().  This
/// is used by CREATE TABLE IF NOT EXISTS and similar if the table already
/// exists, so that the prepared statement for CREATE TABLE IF NOT EXISTS
/// will return false for sqlite3_stmt_readonly() even if that statement
/// is a read-only no-op.
fn sqlite3ForceNotReadOnly(mut pParse: *mut Parse) {
    let mut iReg: i32 = 0 as i32;
    let __v2682: *mut Parse = pParse;
    let __v2683: i32 = unsafe { (*__v2682).nMem };
    let __v2684: i32 = __v2683 + (1 as i32);
    unsafe {
        (*__v2682).nMem = __v2684;
    }
    iReg = __v2684;
    let mut v: *mut Vdbe = unsafe { sqlite3GetVdbe(pParse) };
    if v != std::ptr::null_mut::<Vdbe>() {
        unsafe { sqlite3VdbeAddOp3(v, 4 as i32, 0 as i32, iReg, -(1 as i32)) };
        unsafe { sqlite3VdbeUsesBtree(v, 0 as i32) };
    }
}

/// Begin constructing a new table representation in memory.  This is
/// the first of several action routines that get called in response
/// to a CREATE TABLE statement.  In particular, this routine is called
/// after seeing tokens "CREATE" and "TABLE" and the table name. The isTemp
/// flag is true if the table should be stored in the auxiliary database
/// file instead of in the main database file.  This is normally the case
/// when the "TEMP" or "TEMPORARY" keyword occurs in between
/// CREATE and TABLE.
///
/// The new table record is initialized and put in pParse->pNewTable.
/// As more of the CREATE TABLE statement is parsed, additional action
/// routines will be called to add more information to this record.
/// At the end of the CREATE TABLE statement, the sqlite3EndTable() routine
/// is called to complete the construction of the new table record.
///
/// # Arguments
///
/// * `pParse` - Parser context
/// * `pName1` - First part of the name of the table or view
/// * `pName2` - Second part of the name of the table or view
/// * `isTemp` - True if this is a TEMP table
/// * `isView` - True if this is a VIEW
/// * `isVirtual` - True if this is a VIRTUAL table
/// * `noErr` - Do nothing if table already exists
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3StartTable(
    mut pParse: *mut Parse,
    mut pName1: *mut Token,
    mut pName2: *mut Token,
    mut isTemp: i32,
    mut isView: i32,
    mut isVirtual: i32,
    mut noErr: i32,
) {
    let mut __slate_storage_2335: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2335: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_2335) as *mut i32;
    let mut __slate_storage_2334: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2334: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_2334) as *mut i32;
    let mut __slate_storage_2333: std::mem::MaybeUninit<*mut Parse> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_2333: *mut *mut Parse =
        std::ptr::addr_of_mut!(__slate_storage_2333) as *mut *mut Parse;
    let mut __slate_storage_2332: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2332: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_2332) as *mut i32;
    let mut __slate_storage_2331: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2331: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_2331) as *mut i32;
    let mut __slate_storage_2330: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2330: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_2330) as *mut i32;
    let mut __slate_storage_2329: std::mem::MaybeUninit<*mut Parse> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_2329: *mut *mut Parse =
        std::ptr::addr_of_mut!(__slate_storage_2329) as *mut *mut Parse;
    let mut __slate_storage_2328: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2328: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_2328) as *mut i32;
    let mut __slate_storage_2327: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2327: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_2327) as *mut i32;
    let mut __slate_storage_2326: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2326: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_2326) as *mut i32;
    let mut __slate_storage_2325: std::mem::MaybeUninit<*mut Parse> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_2325: *mut *mut Parse =
        std::ptr::addr_of_mut!(__slate_storage_2325) as *mut *mut Parse;
    let mut __slate_storage_788: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_788: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_788) as *mut i32;
    let mut __slate_storage_787: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_787: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_787) as *mut i32;
    let mut __slate_storage_786: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_786: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_786) as *mut i32;
    let mut __slate_storage_785: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_785: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_785) as *mut i32;
    let mut __slate_storage_784: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_784: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_784) as *mut i32;
    let mut __slate_storage_2341: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2341: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_2341) as *mut u32;
    let mut __slate_storage_2340: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2340: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_2340) as *mut u32;
    let mut __slate_storage_2339: std::mem::MaybeUninit<*mut Table> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_2339: *mut *mut Table =
        std::ptr::addr_of_mut!(__slate_storage_2339) as *mut *mut Table;
    let mut __slate_storage_2338: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2338: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_2338) as *mut u32;
    let mut __slate_storage_2337: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2337: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_2337) as *mut u32;
    let mut __slate_storage_2336: std::mem::MaybeUninit<*mut Table> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_2336: *mut *mut Table =
        std::ptr::addr_of_mut!(__slate_storage_2336) as *mut *mut Table;
    let mut __slate_storage_2324: std::mem::MaybeUninit<*mut Vdbe> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_2324: *mut *mut Vdbe =
        std::ptr::addr_of_mut!(__slate_storage_2324) as *mut *mut Vdbe;
    // Begin generating the code that will insert the table record into
    // the schema table.  Note in particular that we must go ahead
    // and allocate the record number for the table entry now.  Before any
    // PRIMARY KEY or UNIQUE keywords are parsed.  Those keywords will cause
    // indices to be created and the table record must come before the
    // indices.  Hence, the record number for the table must be allocated
    // now.
    let mut __slate_storage_2323: std::mem::MaybeUninit<bool> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2323: *mut bool = std::ptr::addr_of_mut!(__slate_storage_2323) as *mut bool;
    let mut __slate_storage_2322: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2322: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_2322) as *mut i32;
    let mut __slate_storage_2321: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2321: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_2321) as *mut i32;
    let mut __slate_storage_2320: std::mem::MaybeUninit<*mut Parse> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_2320: *mut *mut Parse =
        std::ptr::addr_of_mut!(__slate_storage_2320) as *mut *mut Parse;
    let mut __slate_storage_783: std::mem::MaybeUninit<*mut i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_783: *mut *mut i8 =
        std::ptr::addr_of_mut!(__slate_storage_783) as *mut *mut i8;
    let mut __slate_storage_2319: std::mem::MaybeUninit<bool> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2319: *mut bool = std::ptr::addr_of_mut!(__slate_storage_2319) as *mut bool;
    let mut __slate_storage_782: std::mem::MaybeUninit<*mut i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_782: *mut *mut i8 =
        std::ptr::addr_of_mut!(__slate_storage_782) as *mut *mut i8; // Unqualified name of the table to create
    let mut __slate_storage_780: std::mem::MaybeUninit<*mut Token> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_780: *mut *mut Token =
        std::ptr::addr_of_mut!(__slate_storage_780) as *mut *mut Token; // Database number to create the table in
    let mut __slate_storage_779: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_779: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_779) as *mut i32;
    let mut __slate_storage_778: std::mem::MaybeUninit<*mut Vdbe> = std::mem::MaybeUninit::uninit();
    let __slate_slot_778: *mut *mut Vdbe =
        std::ptr::addr_of_mut!(__slate_storage_778) as *mut *mut Vdbe;
    let mut __slate_storage_777: std::mem::MaybeUninit<*mut sqlite3> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_777: *mut *mut sqlite3 =
        std::ptr::addr_of_mut!(__slate_storage_777) as *mut *mut sqlite3; // The name of the new table
    let mut __slate_storage_776: std::mem::MaybeUninit<*mut i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_776: *mut *mut i8 =
        std::ptr::addr_of_mut!(__slate_storage_776) as *mut *mut i8;
    let mut __slate_storage_775: std::mem::MaybeUninit<*mut Table> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_775: *mut *mut Table =
        std::ptr::addr_of_mut!(__slate_storage_775) as *mut *mut Table;
    unsafe {
        '__join_34: {
            std::ptr::write(__slate_slot_776, std::ptr::null_mut::<i8>());
            std::ptr::write(__slate_slot_777, unsafe { (*pParse).db });
            if (unsafe { (*(*__slate_slot_777)).init.busy }) != (0 as u8)
                && (unsafe { (*(*__slate_slot_777)).init.newTnum }) == ((1 as i32) as u32)
            {
                // Special case:  Parsing the sqlite_schema or sqlite_temp_schema schema
                *__slate_slot_779 = ((unsafe { (*(*__slate_slot_777)).init.iDb }) as u32) as i32;
                *__slate_slot_776 = unsafe {
                    sqlite3DbStrDup(
                        *__slate_slot_777,
                        (if !((0 as i32) != (0 as i32)) && *__slate_slot_779 == (1 as i32) {
                            b"sqlite_temp_master\0".as_ptr() as *mut i8
                        } else {
                            b"sqlite_master\0".as_ptr() as *mut i8
                        }) as *const i8,
                    )
                };
                *__slate_slot_780 = pName1;
            } else {
                // The common case
                *__slate_slot_779 = sqlite3TwoPartName(
                    pParse,
                    pName1,
                    pName2,
                    std::ptr::addr_of_mut!(*__slate_slot_780),
                );
                if *__slate_slot_779 < (0 as i32) {
                    return;
                } else {
                    if !((0 as i32) != (0 as i32))
                        && isTemp != (0 as i32)
                        && (unsafe { (*pName2).n }) > ((0 as i32) as u32)
                        && *__slate_slot_779 != (1 as i32)
                    {
                        // If creating a temp table, the name may not be qualified. Unless
                        // the database name is "temp" anyway.
                        unsafe {
                            sqlite3ErrorMsg(
                                pParse,
                                (b"temporary table name must be unqualified\0".as_ptr() as *mut i8)
                                    as *const i8,
                            )
                        };
                        return;
                    } else {
                        '__join_37: {
                            if !((0 as i32) != (0 as i32)) && isTemp != (0 as i32) {
                                *__slate_slot_779 = 1 as i32;
                            }
                        }
                        *__slate_slot_776 = sqlite3NameFromToken(
                            *__slate_slot_777,
                            *__slate_slot_780 as *const Token,
                        );
                        if (((unsafe { (*pParse).eParseMode }) as u32) as i32) >= (2 as i32) {
                            unsafe {
                                sqlite3RenameTokenMap(
                                    pParse,
                                    (*__slate_slot_776 as *mut ()) as *const (),
                                    *__slate_slot_780 as *const Token,
                                )
                            };
                        }
                    }
                }
            }
        }
        unsafe {
            (*pParse).sNameToken = unsafe { *(*__slate_slot_780) };
        }
        if *__slate_slot_776 == std::ptr::null_mut::<i8>() {
            return;
        } else {
            '__join_0: {
                if sqlite3CheckObjectName(
                    pParse,
                    *__slate_slot_776 as *const i8,
                    (if isView != (0 as i32) {
                        b"view\0".as_ptr() as *mut i8
                    } else {
                        b"table\0".as_ptr() as *mut i8
                    }) as *const i8,
                    *__slate_slot_776 as *const i8,
                ) != (0 as i32)
                {
                } else {
                    if (((unsafe { (*(*__slate_slot_777)).init.iDb }) as u32) as i32) == (1 as i32)
                    {
                        isTemp = 1 as i32;
                    }
                    0 as i32;
                    0 as i32;
                    std::ptr::write(__slate_slot_782, unsafe {
                        (*unsafe {
                            unsafe { (*(*__slate_slot_777)).aDb }.offset(*__slate_slot_779 as isize)
                        })
                        .zDbSName
                    });
                    if (unsafe {
                        sqlite3AuthCheck(
                            pParse,
                            18 as i32,
                            (if !((0 as i32) != (0 as i32)) && isTemp == (1 as i32) {
                                b"sqlite_temp_master\0".as_ptr() as *mut i8
                            } else {
                                b"sqlite_master\0".as_ptr() as *mut i8
                            }) as *const i8,
                            std::ptr::null::<i8>(),
                            *__slate_slot_782 as *const i8,
                        )
                    }) != (0 as i32)
                    {
                    } else {
                        if !(isVirtual != (0 as i32)) {
                            *__slate_slot_2319 = (unsafe {
                                sqlite3AuthCheck(
                                    pParse,
                                    ((unsafe {
                                        *unsafe {
                                            unsafe { std::ptr::addr_of!(aCode) as *const u8 }
                                                .offset((isTemp + (2 as i32) * isView) as isize)
                                        }
                                    }) as u32) as i32,
                                    *__slate_slot_776 as *const i8,
                                    std::ptr::null::<i8>(),
                                    *__slate_slot_782 as *const i8,
                                )
                            }) != (0 as i32);
                        } else {
                            *__slate_slot_2319 = false as bool;
                        }
                        if *__slate_slot_2319 {
                        } else {
                            // Make sure the new table name does not collide with an existing
                            // index or table name in the same database.  Issue an error message if
                            // it does. The exception is if the statement being parsed was passed
                            // to an sqlite3_declare_vtab() call. In that case only the column names
                            // and types will be used, so there is no need to test for namespace
                            // collisions.
                            if !((((unsafe { (*pParse).eParseMode }) as u32) as i32) != (0 as i32))
                            {
                                std::ptr::write(__slate_slot_783, unsafe {
                                    (*unsafe {
                                        unsafe { (*(*__slate_slot_777)).aDb }
                                            .offset(*__slate_slot_779 as isize)
                                    })
                                    .zDbSName
                                });
                                if (0 as i32) != unsafe { sqlite3ReadSchema(pParse) } {
                                    break '__join_0;
                                } else {
                                    *__slate_slot_775 = sqlite3FindTable(
                                        *__slate_slot_777,
                                        *__slate_slot_776 as *const i8,
                                        *__slate_slot_783 as *const i8,
                                    );
                                    if *__slate_slot_775 != std::ptr::null_mut::<Table>() {
                                        if !(noErr != (0 as i32)) {
                                            unsafe {
                                                sqlite3ErrorMsg(
                                                    pParse,
                                                    (b"%s %T already exists\0".as_ptr() as *mut i8)
                                                        as *const i8,
                                                    if (((unsafe {
                                                        (*(*__slate_slot_775)).eTabType
                                                    })
                                                        as u32)
                                                        as i32)
                                                        == (2 as i32)
                                                    {
                                                        b"view\0".as_ptr() as *mut i8
                                                    } else {
                                                        b"table\0".as_ptr() as *mut i8
                                                    },
                                                    *__slate_slot_780,
                                                )
                                            };
                                            break '__join_0;
                                        } else {
                                            0 as i32;
                                            sqlite3CodeVerifySchema(pParse, *__slate_slot_779);
                                            sqlite3ForceNotReadOnly(pParse);
                                            break '__join_0;
                                        }
                                    } else {
                                        if sqlite3FindIndex(
                                            *__slate_slot_777,
                                            *__slate_slot_776 as *const i8,
                                            *__slate_slot_783 as *const i8,
                                        ) != std::ptr::null_mut::<Index>()
                                        {
                                            unsafe {
                                                sqlite3ErrorMsg(
                                                    pParse,
                                                    (b"there is already an index named %s\0"
                                                        .as_ptr()
                                                        as *mut i8)
                                                        as *const i8,
                                                    *__slate_slot_776,
                                                )
                                            };
                                            break '__join_0;
                                        }
                                    }
                                }
                            }
                            *__slate_slot_775 =
                                (unsafe { sqlite3DbMallocZero(*__slate_slot_777, 120 as u64) })
                                    as *mut Table;
                            if *__slate_slot_775 == std::ptr::null_mut::<Table>() {
                                0 as i32;
                                unsafe {
                                    (*pParse).rc = 7 as i32;
                                }
                                std::ptr::write(__slate_slot_2320, pParse);
                                std::ptr::write(__slate_slot_2321, unsafe {
                                    (*(*__slate_slot_2320)).nErr
                                });
                                std::ptr::write(__slate_slot_2322, *__slate_slot_2321 + (1 as i32));
                                unsafe {
                                    (*(*__slate_slot_2320)).nErr = *__slate_slot_2322;
                                }
                            } else {
                                unsafe {
                                    (*(*__slate_slot_775)).zName = *__slate_slot_776;
                                }
                                unsafe {
                                    (*(*__slate_slot_775)).iPKey = -(1 as i32) as i16;
                                }
                                unsafe {
                                    (*(*__slate_slot_775)).pSchema = unsafe {
                                        (*unsafe {
                                            unsafe { (*(*__slate_slot_777)).aDb }
                                                .offset(*__slate_slot_779 as isize)
                                        })
                                        .pSchema
                                    };
                                }
                                unsafe {
                                    (*(*__slate_slot_775)).nTabRef = (1 as i32) as u32;
                                }
                                unsafe {
                                    (*(*__slate_slot_775)).nRowLogEst = (200 as i32) as i16;
                                }
                                0 as i32;
                                0 as i32;
                                unsafe {
                                    (*pParse).pNewTable = *__slate_slot_775;
                                }
                                if !((unsafe { (*(*__slate_slot_777)).init.busy }) != (0 as u8)) {
                                    std::ptr::write(__slate_slot_2324, unsafe {
                                        sqlite3GetVdbe(pParse)
                                    });
                                    *__slate_slot_778 = *__slate_slot_2324;
                                    *__slate_slot_2323 =
                                        *__slate_slot_2324 != std::ptr::null_mut::<Vdbe>();
                                } else {
                                    *__slate_slot_2323 = false as bool;
                                }
                                if *__slate_slot_2323 {
                                    // nullRow[] is an OP_Record encoding of a row containing 5 NULLs
                                    sqlite3BeginWriteOperation(pParse, 1 as i32, *__slate_slot_779);
                                    if isVirtual != (0 as i32) {
                                        unsafe { sqlite3VdbeAddOp0(*__slate_slot_778, 172 as i32) };
                                    }
                                    // If the file format and encoding in the database have not been set,
                                    // set them now.
                                    0 as i32;
                                    std::ptr::write(__slate_slot_2325, pParse);
                                    std::ptr::write(__slate_slot_2326, unsafe {
                                        (*(*__slate_slot_2325)).nMem
                                    });
                                    std::ptr::write(
                                        __slate_slot_2327,
                                        *__slate_slot_2326 + (1 as i32),
                                    );
                                    unsafe {
                                        (*(*__slate_slot_2325)).nMem = *__slate_slot_2327;
                                    }
                                    std::ptr::write(__slate_slot_2328, *__slate_slot_2327);
                                    unsafe {
                                        (*pParse).u1.cr.regRowid = *__slate_slot_2328;
                                    }
                                    *__slate_slot_786 = *__slate_slot_2328;
                                    std::ptr::write(__slate_slot_2329, pParse);
                                    std::ptr::write(__slate_slot_2330, unsafe {
                                        (*(*__slate_slot_2329)).nMem
                                    });
                                    std::ptr::write(
                                        __slate_slot_2331,
                                        *__slate_slot_2330 + (1 as i32),
                                    );
                                    unsafe {
                                        (*(*__slate_slot_2329)).nMem = *__slate_slot_2331;
                                    }
                                    std::ptr::write(__slate_slot_2332, *__slate_slot_2331);
                                    unsafe {
                                        (*pParse).u1.cr.regRoot = *__slate_slot_2332;
                                    }
                                    *__slate_slot_787 = *__slate_slot_2332;
                                    std::ptr::write(__slate_slot_2333, pParse);
                                    std::ptr::write(__slate_slot_2334, unsafe {
                                        (*(*__slate_slot_2333)).nMem
                                    });
                                    std::ptr::write(
                                        __slate_slot_2335,
                                        *__slate_slot_2334 + (1 as i32),
                                    );
                                    unsafe {
                                        (*(*__slate_slot_2333)).nMem = *__slate_slot_2335;
                                    }
                                    *__slate_slot_788 = *__slate_slot_2335;
                                    unsafe {
                                        sqlite3VdbeAddOp3(
                                            *__slate_slot_778,
                                            101 as i32,
                                            *__slate_slot_779,
                                            *__slate_slot_788,
                                            2 as i32,
                                        )
                                    };
                                    unsafe {
                                        sqlite3VdbeUsesBtree(*__slate_slot_778, *__slate_slot_779)
                                    };
                                    *__slate_slot_784 = unsafe {
                                        sqlite3VdbeAddOp1(
                                            *__slate_slot_778,
                                            16 as i32,
                                            *__slate_slot_788,
                                        )
                                    };
                                    {}
                                    *__slate_slot_785 = if (unsafe { (*(*__slate_slot_777)).flags })
                                        & (((2 as i32) as i64) as u64)
                                        != (((0 as i32) as i64) as u64)
                                    {
                                        1 as i32
                                    } else {
                                        4 as i32
                                    };
                                    unsafe {
                                        sqlite3VdbeAddOp3(
                                            *__slate_slot_778,
                                            102 as i32,
                                            *__slate_slot_779,
                                            2 as i32,
                                            *__slate_slot_785,
                                        )
                                    };
                                    unsafe {
                                        sqlite3VdbeAddOp3(
                                            *__slate_slot_778,
                                            102 as i32,
                                            *__slate_slot_779,
                                            5 as i32,
                                            ((unsafe { (*(*__slate_slot_777)).enc }) as u32) as i32,
                                        )
                                    };
                                    unsafe {
                                        sqlite3VdbeJumpHere(*__slate_slot_778, *__slate_slot_784)
                                    };
                                    // This just creates a place-holder record in the sqlite_schema table.
                                    // The record created does not contain anything yet.  It will be replaced
                                    // by the real entry in code generated at sqlite3EndTable().
                                    //
                                    // The rowid for the new entry is left in register pParse->u1.cr.regRowid.
                                    // The root page of the new table is left in reg pParse->u1.cr.regRoot.
                                    // The rowid and root page number values are needed by the code that
                                    // sqlite3EndTable will generate.
                                    if isView != (0 as i32) || isVirtual != (0 as i32) {
                                        unsafe {
                                            sqlite3VdbeAddOp2(
                                                *__slate_slot_778,
                                                73 as i32,
                                                0 as i32,
                                                *__slate_slot_787,
                                            )
                                        };
                                    } else {
                                        0 as i32;
                                        unsafe {
                                            (*pParse).u1.cr.addrCrTab = unsafe {
                                                sqlite3VdbeAddOp3(
                                                    *__slate_slot_778,
                                                    149 as i32,
                                                    *__slate_slot_779,
                                                    *__slate_slot_787,
                                                    1 as i32,
                                                )
                                            };
                                        }
                                    }
                                    sqlite3OpenSchemaTable(pParse, *__slate_slot_779);
                                    unsafe {
                                        sqlite3VdbeAddOp2(
                                            *__slate_slot_778,
                                            129 as i32,
                                            0 as i32,
                                            *__slate_slot_786,
                                        )
                                    };
                                    unsafe {
                                        sqlite3VdbeAddOp4(
                                            *__slate_slot_778,
                                            79 as i32,
                                            6 as i32,
                                            *__slate_slot_788,
                                            0 as i32,
                                            unsafe { std::ptr::addr_of!(nullRow) as *const i8 },
                                            -(1 as i32),
                                        )
                                    };
                                    unsafe {
                                        sqlite3VdbeAddOp3(
                                            *__slate_slot_778,
                                            130 as i32,
                                            0 as i32,
                                            *__slate_slot_788,
                                            *__slate_slot_786,
                                        )
                                    };
                                    unsafe {
                                        sqlite3VdbeChangeP5(
                                            *__slate_slot_778,
                                            ((8 as i32) as i16) as u16,
                                        )
                                    };
                                    unsafe { sqlite3VdbeAddOp0(*__slate_slot_778, 124 as i32) };
                                } else {
                                    if ((unsafe {
                                        (*(*__slate_slot_777))
                                            .init
                                            .__slate_bits_0
                                            .__get_imposterTable()
                                    }) as i32)
                                        != (0 as i32)
                                    {
                                        std::ptr::write(__slate_slot_2336, *__slate_slot_775);
                                        std::ptr::write(__slate_slot_2337, unsafe {
                                            (*(*__slate_slot_2336)).tabFlags
                                        });
                                        std::ptr::write(
                                            __slate_slot_2338,
                                            *__slate_slot_2337 | ((131072 as i32) as u32),
                                        );
                                        unsafe {
                                            (*(*__slate_slot_2336)).tabFlags = *__slate_slot_2338;
                                        }
                                        if ((unsafe {
                                            (*(*__slate_slot_777))
                                                .init
                                                .__slate_bits_0
                                                .__get_imposterTable()
                                        }) as i32)
                                            >= (2 as i32)
                                        {
                                            std::ptr::write(__slate_slot_2339, *__slate_slot_775);
                                            std::ptr::write(__slate_slot_2340, unsafe {
                                                (*(*__slate_slot_2339)).tabFlags
                                            });
                                            std::ptr::write(
                                                __slate_slot_2341,
                                                *__slate_slot_2340 | ((1 as i32) as u32),
                                            );
                                            unsafe {
                                                (*(*__slate_slot_2339)).tabFlags =
                                                    *__slate_slot_2341;
                                            }
                                        }
                                    }
                                }
                                // Normal (non-error) return.
                                return;
                            }
                        }
                    }
                }
            }
            unsafe {
                (*pParse)
                    .__slate_bits_0
                    .__set_checkSchema((1 as i32) as u32);
            }
            unsafe { sqlite3DbFree(*__slate_slot_777, *__slate_slot_776 as *mut ()) };
            return;
        }
    }
    // If an error occurs, we jump here
}

static mut aCode: [u8; 4] = [
    ((2 as i32) as i8) as u8,
    ((4 as i32) as i8) as u8,
    ((8 as i32) as i8) as u8,
    ((6 as i32) as i8) as u8,
];

static mut nullRow: [i8; 6] = [
    (6 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
    (0 as i32) as i8,
];

/// Set properties of a table column based on the (magical)
/// name of the column.
/// Clean up the data structures associated with the RETURNING clause.
#[unsafe(link_section = ".text.slate_distinct.build.sqlite3DeleteReturning")]
extern "C-unwind" fn sqlite3DeleteReturning(mut db: *mut sqlite3, mut pArg: *mut ()) {
    let mut pRet: *mut Returning = pArg as *mut Returning;
    let mut pHash: *mut Hash = unsafe { std::mem::zeroed() };
    pHash = unsafe {
        std::ptr::addr_of_mut!(
            (*unsafe { (*unsafe { unsafe { (*db).aDb }.offset((1 as i32) as isize) }).pSchema })
                .trigHash
        )
    };
    unsafe {
        sqlite3HashInsert(
            pHash,
            (unsafe { (*pRet).zName.as_mut_ptr() as *mut i8 }) as *const i8,
            std::ptr::null_mut::<()>(),
        )
    };
    unsafe { sqlite3ExprListDelete(db, unsafe { (*pRet).pReturnEL }) };
    unsafe { sqlite3DbFree(db, pRet as *mut ()) };
}

/// Add the RETURNING clause to the parse currently underway.
///
/// This routine creates a special TEMP trigger that will fire for each row
/// of the DML statement.  That TEMP trigger contains a single SELECT
/// statement with a result set that is the argument of the RETURNING clause.
/// The trigger has the Trigger.bReturning flag and an opcode of
/// TK_RETURNING instead of TK_SELECT, so that the trigger code generator
/// knows to handle it specially.  The TEMP trigger is automatically
/// removed at the end of the parse.
///
/// When this routine is called, we do not yet know if the RETURNING clause
/// is attached to a DELETE, INSERT, or UPDATE, so construct it as a
/// RETURNING trigger instead.  It will then be converted into the appropriate
/// type on the first call to sqlite3TriggersExist().
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3AddReturning(mut pParse: *mut Parse, mut pList: *mut ExprList) {
    let mut pRet: *mut Returning = unsafe { std::mem::zeroed() };
    let mut pHash: *mut Hash = unsafe { std::mem::zeroed() };
    let mut db: *mut sqlite3 = unsafe { (*pParse).db };
    if (unsafe { (*pParse).pNewTrigger }) != std::ptr::null_mut::<Trigger>() {
        unsafe {
            sqlite3ErrorMsg(
                pParse,
                (b"cannot use RETURNING in a trigger\0".as_ptr() as *mut i8) as *const i8,
            )
        };
    } else {
        0 as i32;
    }
    unsafe {
        (*pParse).__slate_bits_0.__set_bReturning((1 as i32) as u32);
    }
    pRet = (unsafe { sqlite3DbMallocZero(db, 232 as u64) }) as *mut Returning;
    if pRet == std::ptr::null_mut::<Returning>() {
        unsafe { sqlite3ExprListDelete(db, pList) };
        return;
    }
    0 as i32;
    unsafe {
        (*pParse).u1.d.pReturning = pRet;
    }
    unsafe {
        (*pRet).pParse = pParse;
    }
    unsafe {
        (*pRet).pReturnEL = pList;
    }
    unsafe { sqlite3ParserAddCleanup(pParse, Some(sqlite3DeleteReturning), pRet as *mut ()) };
    {}
    if (unsafe { (*db).mallocFailed }) != (0 as u8) {
        return;
    }
    unsafe {
        sqlite3_snprintf(
            ((40 as u64) as u32) as i32,
            unsafe { (*pRet).zName.as_mut_ptr() as *mut i8 },
            (b"sqlite_returning_%p\0".as_ptr() as *mut i8) as *const i8,
            pParse,
        )
    };
    unsafe {
        (*pRet).retTrig.zName = unsafe { (*pRet).zName.as_mut_ptr() as *mut i8 };
    }
    unsafe {
        (*pRet).retTrig.op = ((151 as i32) as i8) as u8;
    }
    unsafe {
        (*pRet).retTrig.tr_tm = ((2 as i32) as i8) as u8;
    }
    unsafe {
        (*pRet).retTrig.bReturning = ((1 as i32) as i8) as u8;
    }
    unsafe {
        (*pRet).retTrig.pSchema =
            unsafe { (*unsafe { unsafe { (*db).aDb }.offset((1 as i32) as isize) }).pSchema };
    }
    unsafe {
        (*pRet).retTrig.pTabSchema =
            unsafe { (*unsafe { unsafe { (*db).aDb }.offset((1 as i32) as isize) }).pSchema };
    }
    unsafe {
        (*pRet).retTrig.step_list = unsafe { std::ptr::addr_of_mut!((*pRet).retTStep) };
    }
    unsafe {
        (*pRet).retTStep.op = ((151 as i32) as i8) as u8;
    }
    unsafe {
        (*pRet).retTStep.pTrig = unsafe { std::ptr::addr_of_mut!((*pRet).retTrig) };
    }
    unsafe {
        (*pRet).retTStep.pExprList = pList;
    }
    pHash = unsafe {
        std::ptr::addr_of_mut!(
            (*unsafe { (*unsafe { unsafe { (*db).aDb }.offset((1 as i32) as isize) }).pSchema })
                .trigHash
        )
    };
    0 as i32;
    if (unsafe {
        sqlite3HashInsert(
            pHash,
            (unsafe { (*pRet).zName.as_mut_ptr() as *mut i8 }) as *const i8,
            (unsafe { std::ptr::addr_of_mut!((*pRet).retTrig) }) as *mut (),
        )
    }) == ((unsafe { std::ptr::addr_of_mut!((*pRet).retTrig) }) as *mut ())
    {
        unsafe { sqlite3OomFault(db) };
    }
}

/// Add a new column to the table currently being constructed.
///
/// The parser calls this routine once for each column declaration
/// in a CREATE TABLE statement.  sqlite3StartTable() gets called
/// first to get things going.  Then this routine is called for each
/// column.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3AddColumn(mut pParse: *mut Parse, mut sName: Token, mut sType: Token) {
    let mut p: *mut Table = unsafe { std::mem::zeroed() };
    let mut i: i32 = 0 as i32;
    let mut z: *mut i8 = unsafe { std::mem::zeroed() };
    let mut zType: *mut i8 = unsafe { std::mem::zeroed() };
    let mut pCol: *mut Column = unsafe { std::mem::zeroed() };
    let mut db: *mut sqlite3 = unsafe { (*pParse).db };
    let mut aNew: *mut Column = unsafe { std::mem::zeroed() };
    let mut eType: u8 = ((0 as i32) as i8) as u8;
    let mut szEst: u8 = ((1 as i32) as i8) as u8;
    let mut affinity: i8 = (65 as i32) as i8;
    let __v2342: *mut Table = unsafe { (*pParse).pNewTable };
    p = __v2342;
    if __v2342 == std::ptr::null_mut::<Table>() {
        return;
    }
    if ((unsafe { (*p).nCol }) as i32) + (1 as i32)
        > unsafe {
            *unsafe { unsafe { (*db).aLimit.as_mut_ptr() as *mut i32 }.offset((2 as i32) as isize) }
        }
    {
        unsafe {
            sqlite3ErrorMsg(
                pParse,
                (b"too many columns on %s\0".as_ptr() as *mut i8) as *const i8,
                unsafe { (*p).zName },
            )
        };
        return;
    }
    if !((((unsafe { (*pParse).eParseMode }) as u32) as i32) >= (2 as i32)) {
        unsafe { sqlite3DequoteToken(std::ptr::addr_of_mut!(sName)) };
    }
    // Because keywords GENERATE ALWAYS can be converted into identifiers
    // by the parser, we can sometimes end up with a typename that ends
    // with "generated always".  Check for this case and omit the surplus
    // text.
    let __v2343: bool;
    if sType.n >= ((16 as i32) as u32) {
        __v2343 = (unsafe {
            sqlite3_strnicmp(
                unsafe {
                    sType
                        .z
                        .offset(sType.n.wrapping_sub((6 as i32) as u32) as isize)
                },
                (b"always\0".as_ptr() as *mut i8) as *const i8,
                6 as i32,
            )
        }) == (0 as i32);
    } else {
        __v2343 = false as bool;
    }
    if __v2343 {
        let __v2344: u32 = sType.n;
        let __v2345: u32 = __v2344.wrapping_sub((6 as i32) as u32);
        sType.n = __v2345;
        '__slate_break_2074: while sType.n > ((0 as i32) as u32)
            && (((unsafe {
                *unsafe {
                    unsafe { std::ptr::addr_of!(sqlite3CtypeMap) as *const u8 }.offset(
                        ((((unsafe {
                            *unsafe {
                                sType
                                    .z
                                    .offset(sType.n.wrapping_sub((1 as i32) as u32) as isize)
                            }
                        }) as u8) as u32) as i32) as isize,
                    )
                }
            }) as u32) as i32)
                & (1 as i32)
                != (0 as i32)
        {
            let __v2346: u32 = sType.n;
            let __v2347: u32 = __v2346.wrapping_sub((1 as i32) as u32);
            sType.n = __v2347;
        }
        let __v2348: bool;
        if sType.n >= ((9 as i32) as u32) {
            __v2348 = (unsafe {
                sqlite3_strnicmp(
                    unsafe {
                        sType
                            .z
                            .offset(sType.n.wrapping_sub((9 as i32) as u32) as isize)
                    },
                    (b"generated\0".as_ptr() as *mut i8) as *const i8,
                    9 as i32,
                )
            }) == (0 as i32);
        } else {
            __v2348 = false as bool;
        }
        if __v2348 {
            let __v2349: u32 = sType.n;
            let __v2350: u32 = __v2349.wrapping_sub((9 as i32) as u32);
            sType.n = __v2350;
            '__slate_break_2076: while sType.n > ((0 as i32) as u32)
                && (((unsafe {
                    *unsafe {
                        unsafe { std::ptr::addr_of!(sqlite3CtypeMap) as *const u8 }.offset(
                            ((((unsafe {
                                *unsafe {
                                    sType
                                        .z
                                        .offset(sType.n.wrapping_sub((1 as i32) as u32) as isize)
                                }
                            }) as u8) as u32) as i32) as isize,
                        )
                    }
                }) as u32) as i32)
                    & (1 as i32)
                    != (0 as i32)
            {
                let __v2351: u32 = sType.n;
                let __v2352: u32 = __v2351.wrapping_sub((1 as i32) as u32);
                sType.n = __v2352;
            }
        }
    }
    // Check for standard typenames.  For standard typenames we will
    // set the Column.eType field rather than storing the typename after
    // the column name, in order to save space.
    if sType.n >= ((3 as i32) as u32) {
        unsafe { sqlite3DequoteToken(std::ptr::addr_of_mut!(sType)) };
        i = 0 as i32;
        '__slate_break_2077: loop {
            if !(i < (6 as i32)) {
                break;
            }
            let __v2355: bool;
            if sType.n
                == ((((unsafe {
                    *unsafe {
                        unsafe { std::ptr::addr_of!(sqlite3StdTypeLen) as *const u8 }
                            .offset(i as isize)
                    }
                }) as u32) as i32) as u32)
            {
                __v2355 = (unsafe {
                    sqlite3_strnicmp(
                        sType.z,
                        unsafe {
                            *unsafe {
                                unsafe { std::ptr::addr_of_mut!(sqlite3StdType) as *mut *const i8 }
                                    .offset(i as isize)
                            }
                        },
                        sType.n as i32,
                    )
                }) == (0 as i32);
            } else {
                __v2355 = false as bool;
            }
            if __v2355 {
                sType.n = (0 as i32) as u32;
                eType = ((i + (1 as i32)) as i8) as u8;
                affinity = unsafe {
                    *unsafe {
                        unsafe { std::ptr::addr_of!(sqlite3StdTypeAffinity) as *const i8 }
                            .offset(i as isize)
                    }
                };
                if (affinity as i32) <= (66 as i32) {
                    szEst = ((5 as i32) as i8) as u8;
                }
                break '__slate_break_2077;
            }
            let __v2353: i32 = i;
            let __v2354: i32 = __v2353 + (1 as i32);
            i = __v2354;
        }
    }
    z = (unsafe {
        sqlite3DbMallocRaw(
            db,
            (((sName.n as u64) as i64)
                + ((1 as i32) as i64)
                + ((sType.n as u64) as i64)
                + (((sType.n > ((0 as i32) as u32)) as i32) as i64)) as u64,
        )
    }) as *mut i8;
    if z == std::ptr::null_mut::<i8>() {
        return;
    }
    if (((unsafe { (*pParse).eParseMode }) as u32) as i32) >= (2 as i32) {
        unsafe {
            sqlite3RenameTokenMap(
                pParse,
                (z as *mut ()) as *const (),
                std::ptr::addr_of_mut!(sName) as *const Token,
            )
        };
    }
    unsafe { memcpy(z as *mut (), sName.z as *const (), sName.n as u64) };
    unsafe {
        *unsafe { z.offset(sName.n as isize) } = (0 as i32) as i8;
    }
    unsafe { sqlite3Dequote(z) };
    let __v2356: bool;
    if (unsafe { (*p).nCol }) != (0 as i16) {
        __v2356 = (unsafe { sqlite3ColumnIndex(p, z as *const i8) }) >= (0 as i32);
    } else {
        __v2356 = false as bool;
    }
    if __v2356 {
        unsafe {
            sqlite3ErrorMsg(
                pParse,
                (b"duplicate column name: %s\0".as_ptr() as *mut i8) as *const i8,
                z,
            )
        };
        unsafe { sqlite3DbFree(db, z as *mut ()) };
        return;
    }
    aNew = (unsafe {
        sqlite3DbRealloc(
            db,
            (unsafe { (*p).aCol }) as *mut (),
            ((((unsafe { (*p).nCol }) as i64) + ((1 as i32) as i64)) as u64)
                .wrapping_mul(16 as u64),
        )
    }) as *mut Column;
    if aNew == std::ptr::null_mut::<Column>() {
        unsafe { sqlite3DbFree(db, z as *mut ()) };
        return;
    }
    unsafe {
        (*p).aCol = aNew;
    }
    pCol = unsafe { unsafe { (*p).aCol }.offset(((unsafe { (*p).nCol }) as i32) as isize) };
    unsafe { memset(pCol as *mut (), 0 as i32, 16 as u64) };
    unsafe {
        (*pCol).zCnName = z;
    }
    unsafe {
        (*pCol).hName = unsafe { sqlite3StrIHash(z as *const i8) };
    }
    {}
    if sType.n == ((0 as i32) as u32) {
        // If there is no type specified, columns have the default affinity
        // 'BLOB' with a default size of 4 bytes.
        unsafe {
            (*pCol).affinity = affinity;
        }
        unsafe {
            (*pCol).__slate_bits_0.__set_eCType(eType as u32);
        }
        unsafe {
            (*pCol).szEst = szEst;
        }
    } else {
        zType = unsafe {
            unsafe { z.offset((unsafe { sqlite3Strlen30(z as *const i8) }) as isize) }
                .offset((1 as i32) as isize)
        };
        unsafe { memcpy(zType as *mut (), sType.z as *const (), sType.n as u64) };
        unsafe {
            *unsafe { zType.offset(sType.n as isize) } = (0 as i32) as i8;
        }
        unsafe { sqlite3Dequote(zType) };
        unsafe {
            (*pCol).affinity = sqlite3AffinityType(zType as *const i8, pCol);
        }
        let __v2357: *mut Column = pCol;
        let __v2358: u16 = unsafe { (*__v2357).colFlags };
        let __v2359: u16 = ((((__v2358 as u32) as i32) | (4 as i32)) as i16) as u16;
        unsafe {
            (*__v2357).colFlags = __v2359;
        }
    }
    if ((unsafe { (*p).nCol }) as i32) <= (255 as i32) {
        let mut h: u8 =
            ((((((unsafe { (*pCol).hName }) as u32) as i32) as i64) as u64) % (16 as u64)) as u8;
        unsafe {
            *unsafe {
                unsafe { (*p).aHx.as_mut_ptr() as *mut u8 }.offset(((h as u32) as i32) as isize)
            } = ((unsafe { (*p).nCol }) as i8) as u8;
        }
    }
    let __v2360: *mut Table = p;
    let __v2361: i16 = unsafe { (*__v2360).nCol };
    let __v2362: i16 = ((__v2361 as i32) + (1 as i32)) as i16;
    unsafe {
        (*__v2360).nCol = __v2362;
    }
    let __v2363: *mut Table = p;
    let __v2364: i16 = unsafe { (*__v2363).nNVCol };
    let __v2365: i16 = ((__v2364 as i32) + (1 as i32)) as i16;
    unsafe {
        (*__v2363).nNVCol = __v2365;
    }
    0 as i32;
    unsafe {
        (*pParse).u1.cr.constraintName.n = (0 as i32) as u32;
    }
}

/// This routine is called by the parser while in the middle of
/// parsing a CREATE TABLE statement.  A "NOT NULL" constraint has
/// been seen on a column.  This routine sets the notNull flag on
/// the column currently under construction.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3AddNotNull(mut pParse: *mut Parse, mut onError: i32) {
    let mut p: *mut Table = unsafe { std::mem::zeroed() };
    let mut pCol: *mut Column = unsafe { std::mem::zeroed() };
    p = unsafe { (*pParse).pNewTable };
    if p == std::ptr::null_mut::<Table>() || ((unsafe { (*p).nCol }) as i32) < (1 as i32) {
        return;
    }
    pCol = unsafe {
        unsafe { (*p).aCol }.offset((((unsafe { (*p).nCol }) as i32) - (1 as i32)) as isize)
    };
    unsafe {
        (*pCol)
            .__slate_bits_0
            .__set_notNull(((onError as i8) as u8) as u32);
    }
    let __v2366: *mut Table = p;
    let __v2367: u32 = unsafe { (*__v2366).tabFlags };
    let __v2368: u32 = __v2367 | ((2048 as i32) as u32);
    unsafe {
        (*__v2366).tabFlags = __v2368;
    }
    // Set the uniqNotNull flag on any UNIQUE or PK indexes already created
    // on this column.
    if (((unsafe { (*pCol).colFlags }) as u32) as i32) & (8 as i32) != (0 as i32) {
        let mut pIdx: *mut Index = unsafe { std::mem::zeroed() };
        pIdx = unsafe { (*p).pIndex };
        '__slate_break_2079: while pIdx != std::ptr::null_mut::<Index>() {
            0 as i32;
            if ((unsafe { *unsafe { unsafe { (*pIdx).aiColumn }.offset((0 as i32) as isize) } })
                as i32)
                == ((unsafe { (*p).nCol }) as i32) - (1 as i32)
            {
                unsafe {
                    (*pIdx).__slate_bits_0.__set_uniqNotNull((1 as i32) as u32);
                }
            }
            pIdx = unsafe { (*pIdx).pNext };
        }
    }
}

/// Scan the column type name zType (length nType) and return the
/// associated affinity type.
///
/// This routine does a case-independent search of zType for the
/// substrings in the following table. If one of the substrings is
/// found, the corresponding affinity is returned. If zType contains
/// more than one of the substrings, entries toward the top of
/// the table take priority. For example, if zType is 'BLOBINT',
/// SQLITE_AFF_INTEGER is returned.
///
/// Substring     | Affinity
/// 'INT'         | SQLITE_AFF_INTEGER
/// 'CHAR'        | SQLITE_AFF_TEXT
/// 'CLOB'        | SQLITE_AFF_TEXT
/// 'TEXT'        | SQLITE_AFF_TEXT
/// 'BLOB'        | SQLITE_AFF_BLOB
/// 'REAL'        | SQLITE_AFF_REAL
/// 'FLOA'        | SQLITE_AFF_REAL
/// 'DOUB'        | SQLITE_AFF_REAL
///
/// If none of the substrings in the above table are found,
/// SQLITE_AFF_NUMERIC is returned.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3AffinityType(mut zIn: *const i8, mut pCol: *mut Column) -> i8 {
    let mut h: u32 = (0 as i32) as u32;
    let mut aff: i8 = (67 as i32) as i8;
    let mut zChar: *const i8 = std::ptr::null::<i8>();
    0 as i32;
    '__slate_break_2080: while (unsafe { *unsafe { zIn.offset((0 as i32) as isize) } }) != (0 as i8)
    {
        let mut x: u8 = unsafe { *(zIn as *mut u8) };
        h = (h << (8 as i32)).wrapping_add(
            (((unsafe {
                *unsafe {
                    unsafe { std::ptr::addr_of!(sqlite3UpperToLower) as *const u8 }
                        .offset(((x as u32) as i32) as isize)
                }
            }) as u32) as i32) as u32,
        );
        let __v2643: *const i8 = zIn;
        let __v2644: *const i8 = unsafe { __v2643.offset((1 as i32) as isize) };
        zIn = __v2644;
        if h == ((((99 as i32) << (24 as i32))
            + ((104 as i32) << (16 as i32))
            + ((97 as i32) << (8 as i32))
            + (114 as i32)) as u32)
        {
            // CHAR
            aff = (66 as i32) as i8;
            zChar = zIn;
        } else {
            if h == ((((99 as i32) << (24 as i32))
                + ((108 as i32) << (16 as i32))
                + ((111 as i32) << (8 as i32))
                + (98 as i32)) as u32)
            {
                // CLOB
                aff = (66 as i32) as i8;
            } else {
                if h == ((((116 as i32) << (24 as i32))
                    + ((101 as i32) << (16 as i32))
                    + ((120 as i32) << (8 as i32))
                    + (116 as i32)) as u32)
                {
                    // TEXT
                    aff = (66 as i32) as i8;
                } else {
                    if h == ((((98 as i32) << (24 as i32))
                        + ((108 as i32) << (16 as i32))
                        + ((111 as i32) << (8 as i32))
                        + (98 as i32)) as u32)
                        && ((aff as i32) == (67 as i32) || (aff as i32) == (69 as i32))
                    {
                        aff = (65 as i32) as i8;
                        if ((unsafe { *unsafe { zIn.offset((0 as i32) as isize) } }) as i32)
                            == (40 as i32)
                        {
                            zChar = zIn;
                        }
                    } else {
                        if h == ((((114 as i32) << (24 as i32))
                            + ((101 as i32) << (16 as i32))
                            + ((97 as i32) << (8 as i32))
                            + (108 as i32)) as u32)
                            && (aff as i32) == (67 as i32)
                        {
                            aff = (69 as i32) as i8;
                        } else {
                            if h == ((((102 as i32) << (24 as i32))
                                + ((108 as i32) << (16 as i32))
                                + ((111 as i32) << (8 as i32))
                                + (97 as i32)) as u32)
                                && (aff as i32) == (67 as i32)
                            {
                                aff = (69 as i32) as i8;
                            } else {
                                if h == ((((100 as i32) << (24 as i32))
                                    + ((111 as i32) << (16 as i32))
                                    + ((117 as i32) << (8 as i32))
                                    + (98 as i32)) as u32)
                                    && (aff as i32) == (67 as i32)
                                {
                                    aff = (69 as i32) as i8;
                                } else {
                                    if h & ((16777215 as i32) as u32)
                                        == ((((105 as i32) << (16 as i32))
                                            + ((110 as i32) << (8 as i32))
                                            + (116 as i32))
                                            as u32)
                                    {
                                        // INT
                                        aff = (68 as i32) as i8;
                                        break '__slate_break_2080;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        // BLOB
        // REAL
        // FLOA
        // DOUB
    }
    // If pCol is not NULL, store an estimate of the field size.  The
    // estimate is scaled so that the size of an integer is 1.
    if pCol != std::ptr::null_mut::<Column>() {
        let mut v: i32 = 0 as i32; // default size is approx 4 bytes
        if (aff as i32) < (67 as i32) {
            if zChar != std::ptr::null::<i8>() {
                '__slate_break_2081: while (unsafe {
                    *unsafe { zChar.offset((0 as i32) as isize) }
                }) != (0 as i8)
                {
                    if (((unsafe {
                        *unsafe {
                            unsafe { std::ptr::addr_of!(sqlite3CtypeMap) as *const u8 }.offset(
                                ((((unsafe { *unsafe { zChar.offset((0 as i32) as isize) } }) as u8)
                                    as u32) as i32) as isize,
                            )
                        }
                    }) as u32) as i32)
                        & (4 as i32)
                        != (0 as i32)
                    {
                        // BLOB(k), VARCHAR(k), CHAR(k) -> r=(k/4+1)
                        unsafe { sqlite3GetInt32(zChar, std::ptr::addr_of_mut!(v)) };
                        break '__slate_break_2081;
                    }
                    let __v2645: *const i8 = zChar;
                    let __v2646: *const i8 = unsafe { __v2645.offset((1 as i32) as isize) };
                    zChar = __v2646;
                }
            } else {
                v = 16 as i32; // BLOB, TEXT, CLOB -> r=5  (approx 20 bytes)
            }
        }
        v = v / (4 as i32) + (1 as i32);
        if v > (255 as i32) {
            v = 255 as i32;
        }
        unsafe {
            (*pCol).szEst = (v as i8) as u8;
        }
    }
    return aff;
}

/// The expression is the default value for the most recently added column
/// of the table currently under construction.
///
/// Default value expressions must be constant.  Raise an exception if this
/// is not the case.
///
/// This routine is called by the parser while in the middle of
/// parsing a CREATE TABLE statement.
///
/// # Arguments
///
/// * `pParse` - Parsing context
/// * `pExpr` - The parsed expression of the default value
/// * `zStart` - Start of the default value text
/// * `zEnd` - First character past end of default value text
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3AddDefaultValue(
    mut pParse: *mut Parse,
    mut pExpr: *mut Expr,
    mut zStart: *const i8,
    mut zEnd: *const i8,
) {
    let mut p: *mut Table = unsafe { std::mem::zeroed() };
    let mut pCol: *mut Column = unsafe { std::mem::zeroed() };
    let mut db: *mut sqlite3 = unsafe { (*pParse).db };
    p = unsafe { (*pParse).pNewTable };
    if p != std::ptr::null_mut::<Table>() {
        let mut isInit: i32 = ((unsafe { (*db).init.busy }) != (0 as u8)
            && (((unsafe { (*db).init.iDb }) as u32) as i32) != (1 as i32))
            as i32;
        pCol = unsafe {
            unsafe { (*p).aCol }.offset((((unsafe { (*p).nCol }) as i32) - (1 as i32)) as isize)
        };
        if !((unsafe { sqlite3ExprIsConstantOrFunction(pExpr, (isInit as i8) as u8) })
            != (0 as i32))
        {
            unsafe {
                sqlite3ErrorMsg(
                    pParse,
                    (b"default value of column [%s] is not constant\0".as_ptr() as *mut i8)
                        as *const i8,
                    unsafe { (*pCol).zCnName },
                )
            };
        } else {
            if (((unsafe { (*pCol).colFlags }) as u32) as i32) & (96 as i32) != (0 as i32) {
                {}
                {}
                unsafe {
                    sqlite3ErrorMsg(
                        pParse,
                        (b"cannot use DEFAULT on a generated column\0".as_ptr() as *mut i8)
                            as *const i8,
                    )
                };
            } else {
                // A copy of pExpr is used instead of the original, as pExpr contains
                // tokens that point to volatile memory.
                let mut x: Expr = unsafe { std::mem::zeroed() };
                let mut pDfltExpr: *mut Expr = unsafe { std::mem::zeroed() };
                unsafe { memset(std::ptr::addr_of_mut!(x) as *mut (), 0 as i32, 72 as u64) };
                x.op = ((181 as i32) as i8) as u8;
                unsafe {
                    x.u.zToken = unsafe { sqlite3DbSpanDup(db, zStart, zEnd) };
                }
                x.pLeft = pExpr;
                x.flags = (8192 as i32) as u32;
                pDfltExpr = unsafe {
                    sqlite3ExprDup(db, std::ptr::addr_of_mut!(x) as *const Expr, 1 as i32)
                };
                unsafe { sqlite3DbFree(db, (unsafe { x.u.zToken }) as *mut ()) };
                sqlite3ColumnSetExpr(pParse, p, pCol, pDfltExpr);
            }
        }
    }
    if (((unsafe { (*pParse).eParseMode }) as u32) as i32) >= (2 as i32) {
        unsafe { sqlite3RenameExprUnmap(pParse, pExpr) };
    }
    unsafe { sqlite3ExprDelete(db, pExpr) };
}

/// Backwards Compatibility Hack:
///
/// Historical versions of SQLite accepted strings as column names in
/// indexes and PRIMARY KEY constraints and in UNIQUE constraints.  Example:
///
///     CREATE TABLE xyz(a,b,c,d,e,PRIMARY KEY('a'),UNIQUE('b','c' COLLATE trim)
///     CREATE INDEX abc ON xyz('c','d' DESC,'e' COLLATE nocase DESC);
///
/// This is goofy.  But to preserve backwards compatibility we continue to
/// accept it.  This routine does the necessary conversion.  It converts
/// the expression given in its argument from a TK_STRING into a TK_ID
/// if the expression is just a TK_STRING with an optional COLLATE clause.
/// If the expression is anything other than TK_STRING, the expression is
/// unchanged.
fn sqlite3StringToId(mut p: *mut Expr) {
    if (((unsafe { (*p).op }) as u32) as i32) == (118 as i32) {
        unsafe {
            (*p).op = ((60 as i32) as i8) as u8;
        }
    } else {
        if (((unsafe { (*p).op }) as u32) as i32) == (114 as i32)
            && (((unsafe { (*unsafe { (*p).pLeft }).op }) as u32) as i32) == (118 as i32)
        {
            unsafe {
                (*unsafe { (*p).pLeft }).op = ((60 as i32) as i8) as u8;
            }
        }
    }
}

/// Tag the given column as being part of the PRIMARY KEY
fn makeColumnPartOfPrimaryKey(mut pParse: *mut Parse, mut pCol: *mut Column) {
    let __v2685: *mut Column = pCol;
    let __v2686: u16 = unsafe { (*__v2685).colFlags };
    let __v2687: u16 = ((((__v2686 as u32) as i32) | (1 as i32)) as i16) as u16;
    unsafe {
        (*__v2685).colFlags = __v2687;
    }
    if (((unsafe { (*pCol).colFlags }) as u32) as i32) & (96 as i32) != (0 as i32) {
        {}
        {}
        unsafe {
            sqlite3ErrorMsg(
                pParse,
                (b"generated columns cannot be part of the PRIMARY KEY\0".as_ptr() as *mut i8)
                    as *const i8,
            )
        };
    }
}

/// Designate the PRIMARY KEY for the table.  pList is a list of names
/// of columns that form the primary key.  If pList is NULL, then the
/// most recently added column of the table is the primary key.
///
/// A table can have at most one primary key.  If the table already has
/// a primary key (and this is the second primary key) then create an
/// error.
///
/// If the PRIMARY KEY is on a single column whose datatype is INTEGER,
/// then we will try to use that column as the rowid.  Set the Table.iPKey
/// field of the table under construction to be the index of the
/// INTEGER PRIMARY KEY column.  Table.iPKey is set to -1 if there is
/// no INTEGER PRIMARY KEY.
///
/// If the key is not an INTEGER PRIMARY KEY, then create a unique
/// index for the key.  No index is created for INTEGER PRIMARY KEYs.
///
/// # Arguments
///
/// * `pParse` - Parsing context
/// * `pList` - List of field names to be indexed
/// * `onError` - What to do with a uniqueness conflict
/// * `autoInc` - True if the AUTOINCREMENT keyword is present
/// * `sortOrder` - SQLITE_SO_ASC or SQLITE_SO_DESC
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3AddPrimaryKey(
    mut pParse: *mut Parse,
    mut pList: *mut ExprList,
    mut onError: i32,
    mut autoInc: i32,
    mut sortOrder: i32,
) {
    let mut __slate_storage_2376: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2376: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_2376) as *mut u32;
    let mut __slate_storage_2375: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2375: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_2375) as *mut u32;
    let mut __slate_storage_2374: std::mem::MaybeUninit<*mut Table> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_2374: *mut *mut Table =
        std::ptr::addr_of_mut!(__slate_storage_2374) as *mut *mut Table;
    let mut __slate_storage_853: std::mem::MaybeUninit<*mut Expr> = std::mem::MaybeUninit::uninit();
    let __slate_slot_853: *mut *mut Expr =
        std::ptr::addr_of_mut!(__slate_storage_853) as *mut *mut Expr;
    let mut __slate_storage_2373: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2373: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_2373) as *mut i32;
    let mut __slate_storage_2372: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2372: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_2372) as *mut i32;
    let mut __slate_storage_852: std::mem::MaybeUninit<*mut Expr> = std::mem::MaybeUninit::uninit();
    let __slate_slot_852: *mut *mut Expr =
        std::ptr::addr_of_mut!(__slate_storage_852) as *mut *mut Expr;
    let mut __slate_storage_2371: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2371: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_2371) as *mut u32;
    let mut __slate_storage_2370: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2370: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_2370) as *mut u32;
    let mut __slate_storage_2369: std::mem::MaybeUninit<*mut Table> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_2369: *mut *mut Table =
        std::ptr::addr_of_mut!(__slate_storage_2369) as *mut *mut Table;
    let mut __slate_storage_851: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_851: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_851) as *mut i32;
    let mut __slate_storage_850: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_850: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_850) as *mut i32;
    let mut __slate_storage_849: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_849: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_849) as *mut i32;
    let mut __slate_storage_848: std::mem::MaybeUninit<*mut Column> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_848: *mut *mut Column =
        std::ptr::addr_of_mut!(__slate_storage_848) as *mut *mut Column;
    let mut __slate_storage_847: std::mem::MaybeUninit<*mut Table> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_847: *mut *mut Table =
        std::ptr::addr_of_mut!(__slate_storage_847) as *mut *mut Table;
    unsafe {
        '__join_0: {
            std::ptr::write(__slate_slot_847, unsafe { (*pParse).pNewTable });
            std::ptr::write(__slate_slot_848, std::ptr::null_mut::<Column>());
            std::ptr::write(__slate_slot_849, -(1 as i32));
            if *__slate_slot_847 == std::ptr::null_mut::<Table>() {
            } else {
                if (unsafe { (*(*__slate_slot_847)).tabFlags }) & ((4 as i32) as u32) != (0 as u32)
                {
                    unsafe {
                        sqlite3ErrorMsg(
                            pParse,
                            (b"table \"%s\" has more than one primary key\0".as_ptr() as *mut i8)
                                as *const i8,
                            unsafe { (*(*__slate_slot_847)).zName },
                        )
                    };
                } else {
                    '__join_9: {
                        std::ptr::write(__slate_slot_2369, *__slate_slot_847);
                        std::ptr::write(__slate_slot_2370, unsafe {
                            (*(*__slate_slot_2369)).tabFlags
                        });
                        std::ptr::write(
                            __slate_slot_2371,
                            *__slate_slot_2370 | ((4 as i32) as u32),
                        );
                        unsafe {
                            (*(*__slate_slot_2369)).tabFlags = *__slate_slot_2371;
                        }
                        if pList == std::ptr::null_mut::<ExprList>() {
                            *__slate_slot_849 =
                                ((unsafe { (*(*__slate_slot_847)).nCol }) as i32) - (1 as i32);
                            *__slate_slot_848 = unsafe {
                                unsafe { (*(*__slate_slot_847)).aCol }
                                    .offset(*__slate_slot_849 as isize)
                            };
                            makeColumnPartOfPrimaryKey(pParse, *__slate_slot_848);
                            *__slate_slot_851 = 1 as i32;
                        } else {
                            *__slate_slot_851 = unsafe { (*pList).nExpr };
                            *__slate_slot_850 = 0 as i32;
                            loop {
                                if *__slate_slot_850 < *__slate_slot_851 {
                                    std::ptr::write(__slate_slot_852, unsafe {
                                        sqlite3ExprSkipCollate(unsafe {
                                            (*unsafe {
                                                unsafe {
                                                    std::ptr::addr_of_mut!((*pList).a)
                                                        as *mut ExprList_item
                                                }
                                                .offset(*__slate_slot_850 as isize)
                                            })
                                            .pExpr
                                        })
                                    });
                                    0 as i32;
                                    sqlite3StringToId(*__slate_slot_852);
                                    if (((unsafe { (*(*__slate_slot_852)).op }) as u32) as i32)
                                        == (60 as i32)
                                    {
                                        0 as i32;
                                        *__slate_slot_849 = unsafe {
                                            sqlite3ColumnIndex(
                                                *__slate_slot_847,
                                                (unsafe { (*(*__slate_slot_852)).u.zToken })
                                                    as *const i8,
                                            )
                                        };
                                        if *__slate_slot_849 >= (0 as i32) {
                                            *__slate_slot_848 = unsafe {
                                                unsafe { (*(*__slate_slot_847)).aCol }
                                                    .offset(*__slate_slot_849 as isize)
                                            };
                                            makeColumnPartOfPrimaryKey(pParse, *__slate_slot_848);
                                        }
                                    }
                                    std::ptr::write(__slate_slot_2372, *__slate_slot_850);
                                    std::ptr::write(
                                        __slate_slot_2373,
                                        *__slate_slot_2372 + (1 as i32),
                                    );
                                    *__slate_slot_850 = *__slate_slot_2373;
                                } else {
                                    break '__join_9;
                                }
                            }
                        }
                    }
                    if *__slate_slot_851 == (1 as i32)
                        && *__slate_slot_848 != std::ptr::null_mut::<Column>()
                        && ((unsafe { (*(*__slate_slot_848)).__slate_bits_0.__get_eCType() })
                            as i32)
                            == (4 as i32)
                        && sortOrder != (1 as i32)
                    {
                        if (((unsafe { (*pParse).eParseMode }) as u32) as i32) >= (2 as i32)
                            && pList != std::ptr::null_mut::<ExprList>()
                        {
                            std::ptr::write(__slate_slot_853, unsafe {
                                sqlite3ExprSkipCollate(unsafe {
                                    (*unsafe {
                                        unsafe {
                                            std::ptr::addr_of_mut!((*pList).a) as *mut ExprList_item
                                        }
                                        .offset((0 as i32) as isize)
                                    })
                                    .pExpr
                                })
                            });
                            unsafe {
                                sqlite3RenameTokenRemap(
                                    pParse,
                                    (unsafe {
                                        std::ptr::addr_of_mut!((*(*__slate_slot_847)).iPKey)
                                    }) as *const (),
                                    *__slate_slot_853 as *const (),
                                )
                            };
                        }
                        unsafe {
                            (*(*__slate_slot_847)).iPKey = *__slate_slot_849 as i16;
                        }
                        unsafe {
                            (*(*__slate_slot_847)).keyConf = (onError as i8) as u8;
                        }
                        0 as i32;
                        std::ptr::write(__slate_slot_2374, *__slate_slot_847);
                        std::ptr::write(__slate_slot_2375, unsafe {
                            (*(*__slate_slot_2374)).tabFlags
                        });
                        std::ptr::write(
                            __slate_slot_2376,
                            *__slate_slot_2375 | ((autoInc * (8 as i32)) as u32),
                        );
                        unsafe {
                            (*(*__slate_slot_2374)).tabFlags = *__slate_slot_2376;
                        }
                        if pList != std::ptr::null_mut::<ExprList>() {
                            unsafe {
                                (*pParse).iPkSortOrder = unsafe {
                                    (*unsafe {
                                        unsafe {
                                            std::ptr::addr_of_mut!((*pList).a) as *mut ExprList_item
                                        }
                                        .offset((0 as i32) as isize)
                                    })
                                    .fg
                                    .sortFlags
                                };
                            }
                        }
                        sqlite3HasExplicitNulls(pParse, pList);
                    } else {
                        if autoInc != (0 as i32) {
                            unsafe {
                                sqlite3ErrorMsg(
                                    pParse,
                                    (b"AUTOINCREMENT is only allowed on an INTEGER PRIMARY KEY\0"
                                        .as_ptr() as *mut i8)
                                        as *const i8,
                                )
                            };
                        } else {
                            sqlite3CreateIndex(
                                pParse,
                                std::ptr::null_mut::<Token>(),
                                std::ptr::null_mut::<Token>(),
                                std::ptr::null_mut::<SrcList>(),
                                pList,
                                onError,
                                std::ptr::null_mut::<Token>(),
                                std::ptr::null_mut::<Expr>(),
                                sortOrder,
                                0 as i32,
                                ((2 as i32) as i8) as u8,
                            );
                            pList = std::ptr::null_mut::<ExprList>();
                        }
                    }
                }
            }
        }
        unsafe { sqlite3ExprListDelete(unsafe { (*pParse).db }, pList) };
        return;
    }
}

/// Add a new CHECK constraint to the table currently under construction.
///
/// # Arguments
///
/// * `pParse` - Parsing context
/// * `pCheckExpr` - The check expression
/// * `zStart` - Opening "("
/// * `zEnd` - Closing ")"
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3AddCheckConstraint(
    mut pParse: *mut Parse,
    mut pCheckExpr: *mut Expr,
    mut zStart: *const i8,
    mut zEnd: *const i8,
) {
    let mut pTab: *mut Table = unsafe { (*pParse).pNewTable };
    let mut db: *mut sqlite3 = unsafe { (*pParse).db };
    let __v2377: bool;
    if pTab != std::ptr::null_mut::<Table>()
        && !((((unsafe { (*pParse).eParseMode }) as u32) as i32) == (1 as i32))
    {
        __v2377 = !((unsafe {
            sqlite3BtreeIsReadonly(unsafe {
                (*unsafe {
                    unsafe { (*db).aDb }
                        .offset((((unsafe { (*db).init.iDb }) as u32) as i32) as isize)
                })
                .pBt
            })
        }) != (0 as i32));
    } else {
        __v2377 = false as bool;
    }
    if __v2377 {
        unsafe {
            (*pTab).pCheck =
                unsafe { sqlite3ExprListAppend(pParse, unsafe { (*pTab).pCheck }, pCheckExpr) };
        }
        0 as i32;
        if (unsafe { (*pParse).u1.cr.constraintName.n }) != (0 as u32) {
            unsafe {
                sqlite3ExprListSetName(
                    pParse,
                    unsafe { (*pTab).pCheck },
                    (unsafe { std::ptr::addr_of_mut!((*pParse).u1.cr.constraintName) })
                        as *const Token,
                    1 as i32,
                )
            };
        } else {
            let mut t: Token = unsafe { std::mem::zeroed() };
            let __v2378: *const i8 = zStart;
            let __v2379: *const i8 = unsafe { __v2378.offset((1 as i32) as isize) };
            zStart = __v2379;
            '__slate_break_2088: while (((unsafe {
                *unsafe {
                    unsafe { std::ptr::addr_of!(sqlite3CtypeMap) as *const u8 }.offset(
                        ((((unsafe { *unsafe { zStart.offset((0 as i32) as isize) } }) as u8)
                            as u32) as i32) as isize,
                    )
                }
            }) as u32) as i32)
                & (1 as i32)
                != (0 as i32)
            {
                let __v2380: *const i8 = zStart;
                let __v2381: *const i8 = unsafe { __v2380.offset((1 as i32) as isize) };
                zStart = __v2381;
            }
            '__slate_break_2089: while (((unsafe {
                *unsafe {
                    unsafe { std::ptr::addr_of!(sqlite3CtypeMap) as *const u8 }.offset(
                        ((((unsafe { *unsafe { zEnd.offset(-(1 as i32) as isize) } }) as u8) as u32)
                            as i32) as isize,
                    )
                }
            }) as u32) as i32)
                & (1 as i32)
                != (0 as i32)
            {
                let __v2382: *const i8 = zEnd;
                let __v2383: *const i8 = unsafe { __v2382.offset(-((1 as i32) as isize)) };
                zEnd = __v2383;
            }
            t.z = zStart;
            t.n = (((unsafe { zEnd.offset_from(t.z as *const i8) }) as i64) as i32) as u32;
            unsafe {
                sqlite3ExprListSetName(
                    pParse,
                    unsafe { (*pTab).pCheck },
                    std::ptr::addr_of_mut!(t) as *const Token,
                    1 as i32,
                )
            };
        }
    } else {
        unsafe { sqlite3ExprDelete(unsafe { (*pParse).db }, pCheckExpr) };
    }
}

/// Set the collation function of the most recently parsed table column
/// to the CollSeq given.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3AddCollateType(mut pParse: *mut Parse, mut pToken: *mut Token) {
    let mut p: *mut Table = unsafe { std::mem::zeroed() };
    let mut i: i32 = 0 as i32;
    let mut zColl: *mut i8 = unsafe { std::mem::zeroed() }; // Dequoted name of collation sequence
    let mut db: *mut sqlite3 = unsafe { std::mem::zeroed() };
    let __v2384: *mut Table = unsafe { (*pParse).pNewTable };
    p = __v2384;
    if __v2384 == std::ptr::null_mut::<Table>()
        || (((unsafe { (*pParse).eParseMode }) as u32) as i32) >= (2 as i32)
    {
        return;
    }
    i = ((unsafe { (*p).nCol }) as i32) - (1 as i32);
    db = unsafe { (*pParse).db };
    zColl = sqlite3NameFromToken(db, pToken as *const Token);
    if !(zColl != std::ptr::null_mut::<i8>()) {
        return;
    }
    if (unsafe { sqlite3LocateCollSeq(pParse, zColl as *const i8) })
        != std::ptr::null_mut::<CollSeq>()
    {
        let mut pIdx: *mut Index = unsafe { std::mem::zeroed() };
        sqlite3ColumnSetColl(
            db,
            unsafe { unsafe { (*p).aCol }.offset(i as isize) },
            zColl as *const i8,
        );
        // If the column is declared as "<name> PRIMARY KEY COLLATE <type>",
        // then an index may have been created on this column before the
        // collation type was added. Correct this if it is the case.
        pIdx = unsafe { (*p).pIndex };
        '__slate_break_2090: while pIdx != std::ptr::null_mut::<Index>() {
            0 as i32;
            if ((unsafe { *unsafe { unsafe { (*pIdx).aiColumn }.offset((0 as i32) as isize) } })
                as i32)
                == i
            {
                unsafe {
                    *unsafe { unsafe { (*pIdx).azColl }.offset((0 as i32) as isize) } =
                        sqlite3ColumnColl(unsafe { unsafe { (*p).aCol }.offset(i as isize) });
                }
            }
            pIdx = unsafe { (*pIdx).pNext };
        }
    }
    unsafe { sqlite3DbFree(db, zColl as *mut ()) };
}

/// Change the most recently parsed column to be a GENERATED ALWAYS AS
/// column.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3AddGenerated(
    mut pParse: *mut Parse,
    mut pExpr: *mut Expr,
    mut pType: *mut Token,
) {
    let mut __slate_storage_2395: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2395: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_2395) as *mut u32;
    let mut __slate_storage_2394: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2394: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_2394) as *mut u32;
    let mut __slate_storage_2393: std::mem::MaybeUninit<*mut Table> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_2393: *mut *mut Table =
        std::ptr::addr_of_mut!(__slate_storage_2393) as *mut *mut Table;
    let mut __slate_storage_2392: std::mem::MaybeUninit<u16> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2392: *mut u16 = std::ptr::addr_of_mut!(__slate_storage_2392) as *mut u16;
    let mut __slate_storage_2391: std::mem::MaybeUninit<u16> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2391: *mut u16 = std::ptr::addr_of_mut!(__slate_storage_2391) as *mut u16;
    let mut __slate_storage_2390: std::mem::MaybeUninit<*mut Column> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_2390: *mut *mut Column =
        std::ptr::addr_of_mut!(__slate_storage_2390) as *mut *mut Column;
    let mut __slate_storage_2389: std::mem::MaybeUninit<i16> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2389: *mut i16 = std::ptr::addr_of_mut!(__slate_storage_2389) as *mut i16;
    let mut __slate_storage_2388: std::mem::MaybeUninit<i16> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2388: *mut i16 = std::ptr::addr_of_mut!(__slate_storage_2388) as *mut i16;
    let mut __slate_storage_2387: std::mem::MaybeUninit<*mut Table> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_2387: *mut *mut Table =
        std::ptr::addr_of_mut!(__slate_storage_2387) as *mut *mut Table;
    let mut __slate_storage_2386: std::mem::MaybeUninit<bool> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2386: *mut bool = std::ptr::addr_of_mut!(__slate_storage_2386) as *mut bool;
    let mut __slate_storage_2385: std::mem::MaybeUninit<bool> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2385: *mut bool = std::ptr::addr_of_mut!(__slate_storage_2385) as *mut bool;
    let mut __slate_storage_875: std::mem::MaybeUninit<*mut Column> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_875: *mut *mut Column =
        std::ptr::addr_of_mut!(__slate_storage_875) as *mut *mut Column;
    let mut __slate_storage_874: std::mem::MaybeUninit<*mut Table> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_874: *mut *mut Table =
        std::ptr::addr_of_mut!(__slate_storage_874) as *mut *mut Table;
    let mut __slate_storage_873: std::mem::MaybeUninit<u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_873: *mut u8 = std::ptr::addr_of_mut!(__slate_storage_873) as *mut u8;
    unsafe {
        '__join_0: {
            std::ptr::write(__slate_slot_873, ((32 as i32) as i8) as u8);
            std::ptr::write(__slate_slot_874, unsafe { (*pParse).pNewTable });
            if *__slate_slot_874 == std::ptr::null_mut::<Table>() {
                // generated column in an CREATE TABLE IF NOT EXISTS that already exists
            } else {
                *__slate_slot_875 = unsafe {
                    unsafe { (*(*__slate_slot_874)).aCol }.offset(
                        (((unsafe { (*(*__slate_slot_874)).nCol }) as i32) - (1 as i32)) as isize,
                    )
                };
                if (((unsafe { (*pParse).eParseMode }) as u32) as i32) == (1 as i32) {
                    unsafe {
                        sqlite3ErrorMsg(
                            pParse,
                            (b"virtual tables cannot use computed columns\0".as_ptr() as *mut i8)
                                as *const i8,
                        )
                    };
                } else {
                    '__join_1: {
                        if (((unsafe { (*(*__slate_slot_875)).iDflt }) as u32) as i32) > (0 as i32)
                        {
                        } else {
                            if pType != std::ptr::null_mut::<Token>() {
                                if (unsafe { (*pType).n }) == ((7 as i32) as u32) {
                                    *__slate_slot_2385 = (unsafe {
                                        sqlite3_strnicmp(
                                            (b"virtual\0".as_ptr() as *mut i8) as *const i8,
                                            unsafe { (*pType).z },
                                            7 as i32,
                                        )
                                    }) == (0 as i32);
                                } else {
                                    *__slate_slot_2385 = false as bool;
                                }
                                if *__slate_slot_2385 {
                                    // no-op
                                } else {
                                    if (unsafe { (*pType).n }) == ((6 as i32) as u32) {
                                        *__slate_slot_2386 = (unsafe {
                                            sqlite3_strnicmp(
                                                (b"stored\0".as_ptr() as *mut i8) as *const i8,
                                                unsafe { (*pType).z },
                                                6 as i32,
                                            )
                                        }) == (0 as i32);
                                    } else {
                                        *__slate_slot_2386 = false as bool;
                                    }
                                    if *__slate_slot_2386 {
                                        *__slate_slot_873 = ((64 as i32) as i8) as u8;
                                    } else {
                                        break '__join_1;
                                    }
                                }
                            }
                            if ((*__slate_slot_873 as u32) as i32) == (32 as i32) {
                                std::ptr::write(__slate_slot_2387, *__slate_slot_874);
                                std::ptr::write(__slate_slot_2388, unsafe {
                                    (*(*__slate_slot_2387)).nNVCol
                                });
                                std::ptr::write(
                                    __slate_slot_2389,
                                    ((*__slate_slot_2388 as i32) - (1 as i32)) as i16,
                                );
                                unsafe {
                                    (*(*__slate_slot_2387)).nNVCol = *__slate_slot_2389;
                                }
                            }
                            std::ptr::write(__slate_slot_2390, *__slate_slot_875);
                            std::ptr::write(__slate_slot_2391, unsafe {
                                (*(*__slate_slot_2390)).colFlags
                            });
                            std::ptr::write(
                                __slate_slot_2392,
                                ((((*__slate_slot_2391 as u32) as i32)
                                    | ((*__slate_slot_873 as u32) as i32))
                                    as i16) as u16,
                            );
                            unsafe {
                                (*(*__slate_slot_2390)).colFlags = *__slate_slot_2392;
                            }
                            0 as i32;
                            0 as i32;
                            std::ptr::write(__slate_slot_2393, *__slate_slot_874);
                            std::ptr::write(__slate_slot_2394, unsafe {
                                (*(*__slate_slot_2393)).tabFlags
                            });
                            std::ptr::write(
                                __slate_slot_2395,
                                *__slate_slot_2394 | (((*__slate_slot_873 as u32) as i32) as u32),
                            );
                            unsafe {
                                (*(*__slate_slot_2393)).tabFlags = *__slate_slot_2395;
                            }
                            if (((unsafe { (*(*__slate_slot_875)).colFlags }) as u32) as i32)
                                & (1 as i32)
                                != (0 as i32)
                            {
                                makeColumnPartOfPrimaryKey(pParse, *__slate_slot_875); // For the error message
                            }
                            if pExpr != std::ptr::null_mut::<Expr>()
                                && (((unsafe { (*pExpr).op }) as u32) as i32) == (60 as i32)
                            {
                                // The value of a generated column needs to be a real expression, not
                                // just a reference to another column, in order for covering index
                                // optimizations to work correctly.  So if the value is not an expression,
                                // turn it into one by adding a unary "+" operator.
                                pExpr = unsafe {
                                    sqlite3PExpr(
                                        pParse,
                                        173 as i32,
                                        pExpr,
                                        std::ptr::null_mut::<Expr>(),
                                    )
                                };
                            }
                            if pExpr != std::ptr::null_mut::<Expr>()
                                && (((unsafe { (*pExpr).op }) as u32) as i32) != (72 as i32)
                            {
                                unsafe {
                                    (*pExpr).affExpr = unsafe { (*(*__slate_slot_875)).affinity };
                                }
                            }
                            sqlite3ColumnSetExpr(
                                pParse,
                                *__slate_slot_874,
                                *__slate_slot_875,
                                pExpr,
                            );
                            pExpr = std::ptr::null_mut::<Expr>();
                            break '__join_0;
                        }
                    }
                    unsafe {
                        sqlite3ErrorMsg(
                            pParse,
                            (b"error in generated column \"%s\"\0".as_ptr() as *mut i8)
                                as *const i8,
                            unsafe { (*(*__slate_slot_875)).zCnName },
                        )
                    };
                }
            }
        }
        unsafe { sqlite3ExprDelete(unsafe { (*pParse).db }, pExpr) };
    }
}

/// Generate code that will increment the schema cookie.
///
/// The schema cookie is used to determine when the schema for the
/// database changes.  After each schema change, the cookie value
/// changes.  When a process first reads the schema it records the
/// cookie.  Thereafter, whenever it goes to access the database,
/// it checks the cookie to make sure the schema has not changed
/// since it was last read.
///
/// This plan is not completely bullet-proof.  It is possible for
/// the schema to change multiple times and for the cookie to be
/// set back to prior value.  But schema changes are infrequent
/// and the probability of hitting the same cookie value is only
/// 1 chance in 2^32.  So we're safe enough.
///
/// IMPLEMENTATION-OF: R-34230-56049 SQLite automatically increments
/// the schema-version whenever the schema changes.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3ChangeCookie(mut pParse: *mut Parse, mut iDb: i32) {
    let mut db: *mut sqlite3 = unsafe { (*pParse).db };
    let mut v: *mut Vdbe = unsafe { (*pParse).pVdbe };
    0 as i32;
    unsafe {
        sqlite3VdbeAddOp3(
            v,
            102 as i32,
            iDb,
            1 as i32,
            ((1 as i32) as u32).wrapping_add(
                (unsafe {
                    (*unsafe { (*unsafe { unsafe { (*db).aDb }.offset(iDb as isize) }).pSchema })
                        .schema_cookie
                }) as u32,
            ) as i32,
        )
    };
}

/// Measure the number of characters needed to output the given
/// identifier.  The number returned includes any quotes used
/// but does not include the null terminator.
///
/// The estimate is conservative.  It might be larger that what is
/// really needed.
fn identLength(mut z: *const i8) -> i64 {
    let mut n: i64 = 0 as i64;
    n = (0 as i32) as i64;
    '__slate_break_2095: while (unsafe { *z }) != (0 as i8) {
        if ((unsafe { *z }) as i32) == (34 as i32) {
            let __v2692: i64 = n;
            let __v2693: i64 = __v2692 + ((1 as i32) as i64);
            n = __v2693;
        }
        let __v2688: i64 = n;
        let __v2689: i64 = __v2688 + ((1 as i32) as i64);
        n = __v2689;
        let __v2690: *const i8 = z;
        let __v2691: *const i8 = unsafe { __v2690.offset((1 as i32) as isize) };
        z = __v2691;
    }
    return n + ((2 as i32) as i64);
}

/// The first parameter is a pointer to an output buffer. The second
/// parameter is a pointer to an integer that contains the offset at
/// which to write into the output buffer. This function copies the
/// nul-terminated string pointed to by the third parameter, zSignedIdent,
/// to the specified offset in the buffer and updates *pIdx to refer
/// to the first byte after the last byte written before returning.
///
/// If the string zSignedIdent consists entirely of alphanumeric
/// characters, does not begin with a digit and is not an SQL keyword,
/// then it is copied to the output buffer exactly as it is. Otherwise,
/// it is quoted using double-quotes.
fn identPut(mut z: *mut i8, mut pIdx: *mut i32, mut zSignedIdent: *mut i8) {
    let mut zIdent: *mut u8 = zSignedIdent as *mut u8;
    let mut i: i32 = 0 as i32;
    let mut j: i32 = 0 as i32;
    let mut needQuote: i32 = 0 as i32;
    i = unsafe { *pIdx };
    j = 0 as i32;
    '__slate_break_2096: loop {
        if !((unsafe { *unsafe { zIdent.offset(j as isize) } }) != (0 as u8)) {
            break;
        }
        if !((((unsafe {
            *unsafe {
                unsafe { std::ptr::addr_of!(sqlite3CtypeMap) as *const u8 }.offset(
                    (((unsafe { *unsafe { zIdent.offset(j as isize) } }) as u32) as i32) as isize,
                )
            }
        }) as u32) as i32)
            & (6 as i32)
            != (0 as i32))
            && (((unsafe { *unsafe { zIdent.offset(j as isize) } }) as u32) as i32) != (95 as i32)
        {
            break '__slate_break_2096;
        }
        let __v2694: i32 = j;
        let __v2695: i32 = __v2694 + (1 as i32);
        j = __v2695;
    }
    let __v2696: bool;
    if (((unsafe {
        *unsafe {
            unsafe { std::ptr::addr_of!(sqlite3CtypeMap) as *const u8 }.offset(
                (((unsafe { *unsafe { zIdent.offset((0 as i32) as isize) } }) as u32) as i32)
                    as isize,
            )
        }
    }) as u32) as i32)
        & (4 as i32)
        != (0 as i32)
    {
        __v2696 = true as bool;
    } else {
        __v2696 = (unsafe { sqlite3KeywordCode(zIdent as *const u8, j) }) != (60 as i32);
    }
    needQuote = (__v2696
        || (((unsafe { *unsafe { zIdent.offset(j as isize) } }) as u32) as i32) != (0 as i32)
        || j == (0 as i32)) as i32;
    if needQuote != (0 as i32) {
        let __v2697: i32 = i;
        let __v2698: i32 = __v2697 + (1 as i32);
        i = __v2698;
        unsafe {
            *unsafe { z.offset(__v2697 as isize) } = (34 as i32) as i8;
        }
    }
    j = 0 as i32;
    '__slate_break_2097: loop {
        if !((unsafe { *unsafe { zIdent.offset(j as isize) } }) != (0 as u8)) {
            break;
        }
        let __v2701: i32 = i;
        let __v2702: i32 = __v2701 + (1 as i32);
        i = __v2702;
        unsafe {
            *unsafe { z.offset(__v2701 as isize) } =
                (unsafe { *unsafe { zIdent.offset(j as isize) } }) as i8;
        }
        if (((unsafe { *unsafe { zIdent.offset(j as isize) } }) as u32) as i32) == (34 as i32) {
            let __v2703: i32 = i;
            let __v2704: i32 = __v2703 + (1 as i32);
            i = __v2704;
            unsafe {
                *unsafe { z.offset(__v2703 as isize) } = (34 as i32) as i8;
            }
        }
        let __v2699: i32 = j;
        let __v2700: i32 = __v2699 + (1 as i32);
        j = __v2700;
    }
    if needQuote != (0 as i32) {
        let __v2705: i32 = i;
        let __v2706: i32 = __v2705 + (1 as i32);
        i = __v2706;
        unsafe {
            *unsafe { z.offset(__v2705 as isize) } = (34 as i32) as i8;
        }
    }
    unsafe {
        *unsafe { z.offset(i as isize) } = (0 as i32) as i8;
    }
    unsafe {
        *pIdx = i;
    }
}

/// Generate a CREATE TABLE statement appropriate for the given
/// table.  Memory to hold the text of the statement is obtained
/// from sqliteMalloc() and must be freed by the calling function.
fn createTableStmt(mut db: *mut sqlite3, mut p: *mut Table) -> *mut i8 {
    let mut i: i32 = 0 as i32;
    let mut k: i32 = 0 as i32;
    let mut len: i32 = 0 as i32;
    let mut n: i64 = 0 as i64;
    let mut zStmt: *mut i8 = unsafe { std::mem::zeroed() };
    let mut zSep: *mut i8 = unsafe { std::mem::zeroed() };
    let mut zSep2: *mut i8 = unsafe { std::mem::zeroed() };
    let mut zEnd: *mut i8 = unsafe { std::mem::zeroed() };
    let mut pCol: *mut Column = unsafe { std::mem::zeroed() };
    n = (0 as i32) as i64;
    pCol = unsafe { (*p).aCol };
    i = 0 as i32;
    '__slate_break_2098: while i < ((unsafe { (*p).nCol }) as i32) {
        let __v2711: i64 = n;
        let __v2712: i64 = __v2711
            + (identLength((unsafe { (*pCol).zCnName }) as *const i8) + ((5 as i32) as i64));
        n = __v2712;
        let __v2707: i32 = i;
        let __v2708: i32 = __v2707 + (1 as i32);
        i = __v2708;
        let __v2709: *mut Column = pCol;
        let __v2710: *mut Column = unsafe { __v2709.offset((1 as i32) as isize) };
        pCol = __v2710;
    }
    let __v2713: i64 = n;
    let __v2714: i64 = __v2713 + identLength((unsafe { (*p).zName }) as *const i8);
    n = __v2714;
    if n < ((50 as i32) as i64) {
        zSep = b"\0".as_ptr() as *mut i8;
        zSep2 = b",\0".as_ptr() as *mut i8;
        zEnd = b")\0".as_ptr() as *mut i8;
    } else {
        zSep = b"\n  \0".as_ptr() as *mut i8;
        zSep2 = b",\n  \0".as_ptr() as *mut i8;
        zEnd = b"\n)\0".as_ptr() as *mut i8;
    }
    let __v2715: i64 = n;
    let __v2716: i64 =
        __v2715 + (((35 as i32) + (6 as i32) * ((unsafe { (*p).nCol }) as i32)) as i64);
    n = __v2716;
    zStmt = (unsafe { sqlite3DbMallocRaw(std::ptr::null_mut::<sqlite3>(), n as u64) }) as *mut i8;
    if zStmt == std::ptr::null_mut::<i8>() {
        unsafe { sqlite3OomFault(db) };
        return std::ptr::null_mut::<i8>();
    }
    0 as i32;
    unsafe {
        memcpy(
            zStmt as *mut (),
            (b"CREATE TABLE \0".as_ptr() as *mut i8) as *const (),
            ((13 as i32) as i64) as u64,
        )
    };
    k = 13 as i32;
    identPut(zStmt, std::ptr::addr_of_mut!(k), unsafe { (*p).zName });
    let __v2717: i32 = k;
    let __v2718: i32 = __v2717 + (1 as i32);
    k = __v2718;
    unsafe {
        *unsafe { zStmt.offset(__v2717 as isize) } = (40 as i32) as i8;
    }
    pCol = unsafe { (*p).aCol };
    i = 0 as i32;
    '__slate_break_2106: while i < ((unsafe { (*p).nCol }) as i32) {
        // SQLITE_AFF_BLOB
        // SQLITE_AFF_TEXT
        // SQLITE_AFF_NUMERIC
        // SQLITE_AFF_INTEGER
        // SQLITE_AFF_REAL
        // SQLITE_AFF_FLEXNUM
        let mut zType: *const i8 = unsafe { std::mem::zeroed() };
        len = unsafe { sqlite3Strlen30(zSep as *const i8) };
        0 as i32;
        unsafe {
            memcpy(
                (unsafe { zStmt.offset(k as isize) }) as *mut (),
                zSep as *const (),
                (len as i64) as u64,
            )
        };
        let __v2723: i32 = k;
        let __v2724: i32 = __v2723 + len;
        k = __v2724;
        zSep = zSep2;
        identPut(zStmt, std::ptr::addr_of_mut!(k), unsafe { (*pCol).zCnName });
        0 as i32;
        0 as i32;
        0 as i32;
        {}
        {}
        {}
        {}
        {}
        {}
        zType = unsafe {
            *unsafe {
                unsafe { std::ptr::addr_of!(azType.0) as *const *const i8 }
                    .offset((((unsafe { (*pCol).affinity }) as i32) - (65 as i32)) as isize)
            }
        };
        len = unsafe { sqlite3Strlen30(zType) };
        0 as i32;
        0 as i32;
        unsafe {
            memcpy(
                (unsafe { zStmt.offset(k as isize) }) as *mut (),
                zType as *const (),
                (len as i64) as u64,
            )
        };
        let __v2725: i32 = k;
        let __v2726: i32 = __v2725 + len;
        k = __v2726;
        0 as i32;
        let __v2719: i32 = i;
        let __v2720: i32 = __v2719 + (1 as i32);
        i = __v2720;
        let __v2721: *mut Column = pCol;
        let __v2722: *mut Column = unsafe { __v2721.offset((1 as i32) as isize) };
        pCol = __v2722;
    }
    len = unsafe { sqlite3Strlen30(zEnd as *const i8) };
    0 as i32;
    unsafe {
        memcpy(
            (unsafe { zStmt.offset(k as isize) }) as *mut (),
            zEnd as *const (),
            ((len + (1 as i32)) as i64) as u64,
        )
    };
    return zStmt;
}

static mut azType: __SlateAlign16<[*const i8; 6]> = __SlateAlign16([
    (b"\0".as_ptr() as *mut i8) as *const i8,
    (b" TEXT\0".as_ptr() as *mut i8) as *const i8,
    (b" NUM\0".as_ptr() as *mut i8) as *const i8,
    (b" INT\0".as_ptr() as *mut i8) as *const i8,
    (b" REAL\0".as_ptr() as *mut i8) as *const i8,
    (b" NUM\0".as_ptr() as *mut i8) as *const i8,
]);

/// Resize an Index object to hold N columns total.  Return SQLITE_OK
/// on success and SQLITE_NOMEM on an OOM error.
fn resizeIndexObject(mut pParse: *mut Parse, mut pIdx: *mut Index, mut N: i32) -> i32 {
    let mut zExtra: *mut i8 = unsafe { std::mem::zeroed() };
    let mut nByte: u64 = 0 as u64;
    let mut db: *mut sqlite3 = unsafe { std::mem::zeroed() };
    if (((unsafe { (*pIdx).nColumn }) as u32) as i32) >= N {
        return 0 as i32;
    }
    db = unsafe { (*pParse).db };
    0 as i32;
    0 as i32; // tag-20250221-1
    {}
    0 as i32;
    nByte = (8 as u64)
        .wrapping_add(2 as u64)
        .wrapping_add(2 as u64)
        .wrapping_add(((1 as i32) as i64) as u64)
        .wrapping_mul((N as i64) as u64);
    zExtra = (unsafe { sqlite3DbMallocZero(db, nByte) }) as *mut i8;
    if zExtra == std::ptr::null_mut::<i8>() {
        return 7 as i32;
    }
    unsafe {
        memcpy(
            zExtra as *mut (),
            (unsafe { (*pIdx).azColl }) as *const (),
            (8 as u64).wrapping_mul(((((unsafe { (*pIdx).nColumn }) as u32) as i32) as i64) as u64),
        )
    };
    unsafe {
        (*pIdx).azColl = zExtra as *mut *const i8;
    }
    let __v2727: *mut i8 = zExtra;
    let __v2728: *mut i8 =
        unsafe { __v2727.offset((8 as u64).wrapping_mul((N as i64) as u64) as isize) };
    zExtra = __v2728;
    unsafe {
        memcpy(
            zExtra as *mut (),
            (unsafe { (*pIdx).aiRowLogEst }) as *const (),
            (2 as u64).wrapping_mul(
                (((((unsafe { (*pIdx).nKeyCol }) as u32) as i32) + (1 as i32)) as i64) as u64,
            ),
        )
    };
    unsafe {
        (*pIdx).aiRowLogEst = zExtra as *mut i16;
    }
    let __v2729: *mut i8 = zExtra;
    let __v2730: *mut i8 =
        unsafe { __v2729.offset((2 as u64).wrapping_mul((N as i64) as u64) as isize) };
    zExtra = __v2730;
    unsafe {
        memcpy(
            zExtra as *mut (),
            (unsafe { (*pIdx).aiColumn }) as *const (),
            (2 as u64).wrapping_mul(((((unsafe { (*pIdx).nColumn }) as u32) as i32) as i64) as u64),
        )
    };
    unsafe {
        (*pIdx).aiColumn = zExtra as *mut i16;
    }
    let __v2731: *mut i8 = zExtra;
    let __v2732: *mut i8 =
        unsafe { __v2731.offset((2 as u64).wrapping_mul((N as i64) as u64) as isize) };
    zExtra = __v2732;
    unsafe {
        memcpy(
            zExtra as *mut (),
            (unsafe { (*pIdx).aSortOrder }) as *const (),
            (unsafe { (*pIdx).nColumn }) as u64,
        )
    };
    unsafe {
        (*pIdx).aSortOrder = zExtra as *mut u8;
    }
    unsafe {
        (*pIdx).nColumn = (N as i16) as u16;
    }
    // See tag-20250221-1 above for proof of safety
    unsafe {
        (*pIdx).__slate_bits_0.__set_isResized((1 as i32) as u32);
    }
    return 0 as i32;
}

/// Return true if the index pIdx can support a Bloom filter on its
/// first N columns.  Specifically, return true if all of the first N
/// columns have the BINARY collating sequence or no collating sequence
/// at all, and return false if there are any non-BINARY collating
/// seqeuences on any of the first N columns.  tag-202607231411
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3IndexBloomable(mut pIdx: *const Index, mut N: i32) -> i32 {
    let mut i: i32 = 0 as i32;
    0 as i32;
    0 as i32;
    i = 0 as i32;
    '__slate_break_2113: loop {
        if !(i < N) {
            break;
        }
        if (unsafe {
            sqlite3StrICmp(
                unsafe { *unsafe { unsafe { (*pIdx).azColl }.offset(i as isize) } },
                (b"BINARY\0".as_ptr() as *mut i8) as *const i8,
            )
        }) != (0 as i32)
        {
            return 0 as i32;
        }
        let __v2514: i32 = i;
        let __v2515: i32 = __v2514 + (1 as i32);
        i = __v2515;
    }
    return 1 as i32;
}

/// Estimate the total row width for a table.
fn estimateTableWidth(mut pTab: *mut Table) {
    let mut wTable: u32 = (0 as i32) as u32;
    let mut pTabCol: *const Column = unsafe { std::mem::zeroed() };
    let mut i: i32 = 0 as i32;
    i = (unsafe { (*pTab).nCol }) as i32;
    let __v2733: *const Column = (unsafe { (*pTab).aCol }) as *const Column;
    pTabCol = __v2733;
    '__slate_break_2115: while i > (0 as i32) {
        let __v2738: u32 = wTable;
        let __v2739: u32 =
            __v2738.wrapping_add((((unsafe { (*pTabCol).szEst }) as u32) as i32) as u32);
        wTable = __v2739;
        let __v2734: i32 = i;
        let __v2735: i32 = __v2734 - (1 as i32);
        i = __v2735;
        let __v2736: *const Column = pTabCol;
        let __v2737: *const Column = unsafe { __v2736.offset((1 as i32) as isize) };
        pTabCol = __v2737;
    }
    if ((unsafe { (*pTab).iPKey }) as i32) < (0 as i32) {
        let __v2740: u32 = wTable;
        let __v2741: u32 = __v2740.wrapping_add((1 as i32) as u32);
        wTable = __v2741;
    }
    unsafe {
        (*pTab).szTabRow = unsafe { sqlite3LogEst(wTable.wrapping_mul((4 as i32) as u32) as u64) };
    }
}

/// Estimate the average size of a row for an index.
fn estimateIndexWidth(mut pIdx: *mut Index) {
    let mut wIndex: u32 = (0 as i32) as u32;
    let mut i: i32 = 0 as i32;
    let mut aCol: *const Column = (unsafe { (*unsafe { (*pIdx).pTable }).aCol }) as *const Column;
    i = 0 as i32;
    '__slate_break_2116: loop {
        if !(i < (((unsafe { (*pIdx).nColumn }) as u32) as i32)) {
            break;
        }
        let mut x: i16 = unsafe { *unsafe { unsafe { (*pIdx).aiColumn }.offset(i as isize) } };
        0 as i32;
        let __v2744: u32 = wIndex;
        let __v2745: u32 = __v2744.wrapping_add(
            (if (x as i32) < (0 as i32) {
                1 as i32
            } else {
                ((unsafe { (*unsafe { aCol.offset((x as i32) as isize) }).szEst }) as u32) as i32
            }) as u32,
        );
        wIndex = __v2745;
        let __v2742: i32 = i;
        let __v2743: i32 = __v2742 + (1 as i32);
        i = __v2743;
    }
    unsafe {
        (*pIdx).szIdxRow = unsafe { sqlite3LogEst(wIndex.wrapping_mul((4 as i32) as u32) as u64) };
    }
}

/// Return true if column number x is any of the first nCol entries of aiCol[].
/// This is used to determine if the column number x appears in any of the
/// first nCol entries of an index.
fn hasColumn(mut aiCol: *const i16, mut nCol: i32, mut x: i32) -> i32 {
    '__slate_break_2117: loop {
        let __v2746: i32 = nCol;
        let __v2747: i32 = __v2746 - (1 as i32);
        nCol = __v2747;
        if !(__v2746 > (0 as i32)) {
            break;
        }
        let __v2748: *const i16 = aiCol;
        let __v2749: *const i16 = unsafe { __v2748.offset((1 as i32) as isize) };
        aiCol = __v2749;
        if x == ((unsafe { *__v2748 }) as i32) {
            return 1 as i32;
        }
    }
    return 0 as i32;
}

/// Return true if any of the first nKey entries of index pIdx exactly
/// match the iCol-th entry of pPk.  pPk is always a WITHOUT ROWID
/// PRIMARY KEY index.  pIdx is an index on the same table.  pIdx may
/// or may not be the same index as pPk.
///
/// The first nKey entries of pIdx are guaranteed to be ordinary columns,
/// not a rowid or expression.
///
/// This routine differs from hasColumn() in that both the column and the
/// collating sequence must match for this routine, but for hasColumn() only
/// the column name must match.
fn isDupColumn(mut pIdx: *mut Index, mut nKey: i32, mut pPk: *mut Index, mut iCol: i32) -> i32 {
    let mut i: i32 = 0 as i32;
    let mut j: i32 = 0 as i32;
    0 as i32;
    0 as i32;
    0 as i32;
    0 as i32;
    0 as i32;
    {}
    j = (unsafe { *unsafe { unsafe { (*pPk).aiColumn }.offset(iCol as isize) } }) as i32;
    0 as i32;
    i = 0 as i32;
    '__slate_break_2118: loop {
        if !(i < nKey) {
            break;
        }
        0 as i32;
        let __v2752: bool;
        if ((unsafe { *unsafe { unsafe { (*pIdx).aiColumn }.offset(i as isize) } }) as i32) == j {
            __v2752 = (unsafe {
                sqlite3StrICmp(
                    unsafe { *unsafe { unsafe { (*pIdx).azColl }.offset(i as isize) } },
                    unsafe { *unsafe { unsafe { (*pPk).azColl }.offset(iCol as isize) } },
                )
            }) == (0 as i32);
        } else {
            __v2752 = false as bool;
        }
        if __v2752 {
            return 1 as i32;
        }
        let __v2750: i32 = i;
        let __v2751: i32 = __v2750 + (1 as i32);
        i = __v2751;
    }
    return 0 as i32;
}

/// Recompute the colNotIdxed field of the Index.
///
/// colNotIdxed is a bitmask that has a 0 bit representing each indexed
/// columns that are within the first 63 columns of the table and a 1 for
/// all other bits (all columns that are not in the index).  The
/// high-order bit of colNotIdxed is always 1.  All unindexed columns
/// of the table have a 1.
///
/// 2019-10-24:  For the purpose of this computation, virtual columns are
/// not considered to be covered by the index, even if they are in the
/// index, because we do not trust the logic in whereIndexExprTrans() to be
/// able to find all instances of a reference to the indexed table column
/// and convert them into references to the index.  Hence we always want
/// the actual table at hand in order to recompute the virtual column, if
/// necessary.
///
/// The colNotIdxed mask is AND-ed with the SrcList.a[].colUsed mask
/// to determine if the index is covering index.
fn recomputeColumnsNotIndexed(mut pIdx: *mut Index) {
    let mut m: u64 = ((0 as i32) as i64) as u64;
    let mut j: i32 = 0 as i32;
    let mut pTab: *mut Table = unsafe { (*pIdx).pTable };
    j = (((unsafe { (*pIdx).nColumn }) as u32) as i32) - (1 as i32);
    '__slate_break_2119: loop {
        if !(j >= (0 as i32)) {
            break;
        }
        let mut x: i32 =
            (unsafe { *unsafe { unsafe { (*pIdx).aiColumn }.offset(j as isize) } }) as i32;
        if x >= (0 as i32)
            && (((unsafe { (*unsafe { unsafe { (*pTab).aCol }.offset(x as isize) }).colFlags })
                as u32) as i32)
                & (32 as i32)
                == (0 as i32)
        {
            {}
            {}
            if x < (((8 as u64).wrapping_mul(((8 as i32) as i64) as u64) as u32) as i32)
                - (1 as i32)
            {
                let __v2755: u64 = m;
                let __v2756: u64 = __v2755 | (((1 as i32) as i64) as u64) << x;
                m = __v2756;
            }
        }
        let __v2753: i32 = j;
        let __v2754: i32 = __v2753 - (1 as i32);
        j = __v2754;
    }
    unsafe {
        (*pIdx).colNotIdxed = !m;
    }
    0 as i32; // See note-20221022-a
}

/// This routine runs at the end of parsing a CREATE TABLE statement that
/// has a WITHOUT ROWID clause.  The job of this routine is to convert both
/// internal schema data structures and the generated VDBE code so that they
/// are appropriate for a WITHOUT ROWID table instead of a rowid table.
/// Changes include:
///
///     (1)  Set all columns of the PRIMARY KEY schema object to be NOT NULL.
///     (2)  Convert P3 parameter of the OP_CreateBtree from BTREE_INTKEY
///          into BTREE_BLOBKEY.
///     (3)  Bypass the creation of the sqlite_schema table entry
///          for the PRIMARY KEY as the primary key index is now
///          identified by the sqlite_schema table entry of the table itself.
///     (4)  Set the Index.tnum of the PRIMARY KEY Index object in the
///          schema to the rootpage from the main table.
///     (5)  Add all table columns to the PRIMARY KEY Index object
///          so that the PRIMARY KEY is a covering index.  The surplus
///          columns are part of KeyInfo.nAllField and are not used for
///          sorting or lookup or uniqueness checks.
///     (6)  Replace the rowid tail on all automatically generated UNIQUE
///          indices with the PRIMARY KEY columns.
///
/// For virtual tables, only (1) is performed.
fn convertToWithoutRowidTable(mut pParse: *mut Parse, mut pTab: *mut Table) {
    let mut pIdx: *mut Index = unsafe { std::mem::zeroed() };
    let mut pPk: *mut Index = unsafe { std::mem::zeroed() };
    let mut nPk: i32 = 0 as i32;
    let mut nExtra: i32 = 0 as i32;
    let mut i: i32 = 0 as i32;
    let mut j: i32 = 0 as i32;
    let mut db: *mut sqlite3 = unsafe { (*pParse).db };
    let mut v: *mut Vdbe = unsafe { (*pParse).pVdbe };
    // Mark every PRIMARY KEY column as NOT NULL (except for imposter tables)
    if !(((unsafe { (*db).init.__slate_bits_0.__get_imposterTable() }) as i32) != (0 as i32)) {
        i = 0 as i32;
        '__slate_break_2120: loop {
            if !(i < ((unsafe { (*pTab).nCol }) as i32)) {
                break;
            }
            if (((unsafe { (*unsafe { unsafe { (*pTab).aCol }.offset(i as isize) }).colFlags })
                as u32) as i32)
                & (1 as i32)
                != (0 as i32)
                && ((unsafe {
                    (*unsafe { unsafe { (*pTab).aCol }.offset(i as isize) })
                        .__slate_bits_0
                        .__get_notNull()
                }) as i32)
                    == (0 as i32)
            {
                unsafe {
                    (*unsafe { unsafe { (*pTab).aCol }.offset(i as isize) })
                        .__slate_bits_0
                        .__set_notNull((2 as i32) as u32);
                }
            }
            let __v2757: i32 = i;
            let __v2758: i32 = __v2757 + (1 as i32);
            i = __v2758;
        }
        let __v2759: *mut Table = pTab;
        let __v2760: u32 = unsafe { (*__v2759).tabFlags };
        let __v2761: u32 = __v2760 | ((2048 as i32) as u32);
        unsafe {
            (*__v2759).tabFlags = __v2761;
        }
    }
    // Convert the P3 operand of the OP_CreateBtree opcode from BTREE_INTKEY
    // into BTREE_BLOBKEY.
    0 as i32;
    if (unsafe { (*pParse).u1.cr.addrCrTab }) != (0 as i32) {
        0 as i32;
        unsafe { sqlite3VdbeChangeP3(v, unsafe { (*pParse).u1.cr.addrCrTab }, 2 as i32) };
    }
    // Locate the PRIMARY KEY index.  Or, if this table was originally
    // an INTEGER PRIMARY KEY table, create a new PRIMARY KEY index.
    if ((unsafe { (*pTab).iPKey }) as i32) >= (0 as i32) {
        let mut pList: *mut ExprList = unsafe { std::mem::zeroed() };
        let mut ipkToken: Token = unsafe { std::mem::zeroed() };
        unsafe {
            sqlite3TokenInit(std::ptr::addr_of_mut!(ipkToken), unsafe {
                (*unsafe {
                    unsafe { (*pTab).aCol }.offset(((unsafe { (*pTab).iPKey }) as i32) as isize)
                })
                .zCnName
            })
        };
        pList = unsafe {
            sqlite3ExprListAppend(pParse, std::ptr::null_mut::<ExprList>(), unsafe {
                sqlite3ExprAlloc(
                    db,
                    60 as i32,
                    std::ptr::addr_of_mut!(ipkToken) as *const Token,
                    0 as i32,
                )
            })
        };
        if pList == std::ptr::null_mut::<ExprList>() {
            let __v2762: *mut Table = pTab;
            let __v2763: u32 = unsafe { (*__v2762).tabFlags };
            let __v2764: u32 = __v2763 & (!(128 as i32) as u32);
            unsafe {
                (*__v2762).tabFlags = __v2764;
            }
            return;
        }
        if (((unsafe { (*pParse).eParseMode }) as u32) as i32) >= (2 as i32) {
            unsafe {
                sqlite3RenameTokenRemap(
                    pParse,
                    (unsafe {
                        (*unsafe {
                            unsafe { std::ptr::addr_of_mut!((*pList).a) as *mut ExprList_item }
                                .offset((0 as i32) as isize)
                        })
                        .pExpr
                    }) as *const (),
                    (unsafe { std::ptr::addr_of_mut!((*pTab).iPKey) }) as *const (),
                )
            };
        }
        unsafe {
            (*unsafe {
                unsafe { std::ptr::addr_of_mut!((*pList).a) as *mut ExprList_item }
                    .offset((0 as i32) as isize)
            })
            .fg
            .sortFlags = unsafe { (*pParse).iPkSortOrder };
        }
        0 as i32;
        unsafe {
            (*pTab).iPKey = -(1 as i32) as i16;
        }
        sqlite3CreateIndex(
            pParse,
            std::ptr::null_mut::<Token>(),
            std::ptr::null_mut::<Token>(),
            std::ptr::null_mut::<SrcList>(),
            pList,
            ((unsafe { (*pTab).keyConf }) as u32) as i32,
            std::ptr::null_mut::<Token>(),
            std::ptr::null_mut::<Expr>(),
            0 as i32,
            0 as i32,
            ((2 as i32) as i8) as u8,
        );
        if (unsafe { (*pParse).nErr }) != (0 as i32) {
            let __v2765: *mut Table = pTab;
            let __v2766: u32 = unsafe { (*__v2765).tabFlags };
            let __v2767: u32 = __v2766 & (!(128 as i32) as u32);
            unsafe {
                (*__v2765).tabFlags = __v2767;
            }
            return;
        }
        0 as i32;
        pPk = sqlite3PrimaryKeyIndex(pTab);
        0 as i32;
    } else {
        pPk = sqlite3PrimaryKeyIndex(pTab);
        0 as i32;
        // Remove all redundant columns from the PRIMARY KEY.  For example, change
        // "PRIMARY KEY(a,b,a,b,c,b,c,d)" into just "PRIMARY KEY(a,b,c,d)".  Later
        // code assumes the PRIMARY KEY contains no repeated columns.
        j = 1 as i32;
        i = 1 as i32;
        '__slate_break_2121: loop {
            if !(i < (((unsafe { (*pPk).nKeyCol }) as u32) as i32)) {
                break;
            }
            if isDupColumn(pPk, j, pPk, i) != (0 as i32) {
                let __v2770: *mut Index = pPk;
                let __v2771: u16 = unsafe { (*__v2770).nColumn };
                let __v2772: u16 = ((((__v2771 as u32) as i32) - (1 as i32)) as i16) as u16;
                unsafe {
                    (*__v2770).nColumn = __v2772;
                }
            } else {
                {}
                unsafe {
                    *unsafe { unsafe { (*pPk).azColl }.offset(j as isize) } =
                        unsafe { *unsafe { unsafe { (*pPk).azColl }.offset(i as isize) } };
                }
                unsafe {
                    *unsafe { unsafe { (*pPk).aSortOrder }.offset(j as isize) } =
                        unsafe { *unsafe { unsafe { (*pPk).aSortOrder }.offset(i as isize) } };
                }
                let __v2773: i32 = j;
                let __v2774: i32 = __v2773 + (1 as i32);
                j = __v2774;
                unsafe {
                    *unsafe { unsafe { (*pPk).aiColumn }.offset(__v2773 as isize) } =
                        unsafe { *unsafe { unsafe { (*pPk).aiColumn }.offset(i as isize) } };
                }
            }
            let __v2768: i32 = i;
            let __v2769: i32 = __v2768 + (1 as i32);
            i = __v2769;
        }
        unsafe {
            (*pPk).nKeyCol = (j as i16) as u16;
        }
    }
    0 as i32;
    unsafe {
        (*pPk).__slate_bits_0.__set_isCovering((1 as i32) as u32);
    }
    if !(((unsafe { (*db).init.__slate_bits_0.__get_imposterTable() }) as i32) != (0 as i32)) {
        unsafe {
            (*pPk).__slate_bits_0.__set_uniqNotNull((1 as i32) as u32);
        }
    }
    let __v2775: u16 = unsafe { (*pPk).nKeyCol };
    unsafe {
        (*pPk).nColumn = __v2775;
    }
    nPk = (__v2775 as u32) as i32;
    // Bypass the creation of the PRIMARY KEY btree and the sqlite_schema
    // table entry. This is only required if currently generating VDBE
    // code for a CREATE TABLE (not when parsing one as part of reading
    // a database schema).
    if v != std::ptr::null_mut::<Vdbe>() && (unsafe { (*pPk).tnum }) > ((0 as i32) as u32) {
        0 as i32;
        unsafe {
            sqlite3VdbeChangeOpcode(v, (unsafe { (*pPk).tnum }) as i32, ((9 as i32) as i8) as u8)
        };
    }
    // The root page of the PRIMARY KEY is the table root page
    unsafe {
        (*pPk).tnum = unsafe { (*pTab).tnum };
    }
    // Update the in-memory representation of all UNIQUE indices by converting
    // the final rowid column into one or more columns of the PRIMARY KEY.
    pIdx = unsafe { (*pTab).pIndex };
    '__slate_break_2122: while pIdx != std::ptr::null_mut::<Index>() {
        let mut n: i32 = 0 as i32;
        if ((unsafe { (*pIdx).__slate_bits_0.__get_idxType() }) as i32) == (2 as i32) {
        } else {
            n = 0 as i32;
            i = 0 as i32;
            '__slate_break_2123: loop {
                if !(i < nPk) {
                    break;
                }
                if !(isDupColumn(pIdx, ((unsafe { (*pIdx).nKeyCol }) as u32) as i32, pPk, i)
                    != (0 as i32))
                {
                    {}
                    let __v2778: i32 = n;
                    let __v2779: i32 = __v2778 + (1 as i32);
                    n = __v2779;
                }
                let __v2776: i32 = i;
                let __v2777: i32 = __v2776 + (1 as i32);
                i = __v2777;
            }
            if n == (0 as i32) {
                // This index is a superset of the primary key
                unsafe {
                    (*pIdx).nColumn = unsafe { (*pIdx).nKeyCol };
                }
            } else {
                if resizeIndexObject(
                    pParse,
                    pIdx,
                    (((unsafe { (*pIdx).nKeyCol }) as u32) as i32) + n,
                ) != (0 as i32)
                {
                    return;
                }
                i = 0 as i32;
                let __v2780: i32 = ((unsafe { (*pIdx).nKeyCol }) as u32) as i32;
                j = __v2780;
                '__slate_break_2124: loop {
                    if !(i < nPk) {
                        break;
                    }
                    if !(isDupColumn(pIdx, ((unsafe { (*pIdx).nKeyCol }) as u32) as i32, pPk, i)
                        != (0 as i32))
                    {
                        {}
                        unsafe {
                            *unsafe { unsafe { (*pIdx).aiColumn }.offset(j as isize) } = unsafe {
                                *unsafe { unsafe { (*pPk).aiColumn }.offset(i as isize) }
                            };
                        }
                        unsafe {
                            *unsafe { unsafe { (*pIdx).azColl }.offset(j as isize) } =
                                unsafe { *unsafe { unsafe { (*pPk).azColl }.offset(i as isize) } };
                        }
                        if (unsafe { *unsafe { unsafe { (*pPk).aSortOrder }.offset(i as isize) } })
                            != (0 as u8)
                        {
                            // See ticket https://sqlite.org/src/info/bba7b69f9849b5bf
                            unsafe {
                                (*pIdx).__slate_bits_0.__set_bAscKeyBug((1 as i32) as u32);
                            }
                        }
                        let __v2783: i32 = j;
                        let __v2784: i32 = __v2783 + (1 as i32);
                        j = __v2784;
                    }
                    let __v2781: i32 = i;
                    let __v2782: i32 = __v2781 + (1 as i32);
                    i = __v2782;
                }
                0 as i32;
                0 as i32;
            }
        }
        pIdx = unsafe { (*pIdx).pNext };
    }
    // Add all table columns to the PRIMARY KEY index
    nExtra = 0 as i32;
    i = 0 as i32;
    '__slate_break_2125: loop {
        if !(i < ((unsafe { (*pTab).nCol }) as i32)) {
            break;
        }
        if !(hasColumn((unsafe { (*pPk).aiColumn }) as *const i16, nPk, i) != (0 as i32))
            && (((unsafe { (*unsafe { unsafe { (*pTab).aCol }.offset(i as isize) }).colFlags })
                as u32) as i32)
                & (32 as i32)
                == (0 as i32)
        {
            let __v2787: i32 = nExtra;
            let __v2788: i32 = __v2787 + (1 as i32);
            nExtra = __v2788;
        }
        let __v2785: i32 = i;
        let __v2786: i32 = __v2785 + (1 as i32);
        i = __v2786;
    }
    if resizeIndexObject(pParse, pPk, nPk + nExtra) != (0 as i32) {
        return;
    }
    i = 0 as i32;
    let __v2789: i32 = nPk;
    j = __v2789;
    '__slate_break_2126: loop {
        if !(i < ((unsafe { (*pTab).nCol }) as i32)) {
            break;
        }
        if !(hasColumn((unsafe { (*pPk).aiColumn }) as *const i16, j, i) != (0 as i32))
            && (((unsafe { (*unsafe { unsafe { (*pTab).aCol }.offset(i as isize) }).colFlags })
                as u32) as i32)
                & (32 as i32)
                == (0 as i32)
        {
            let mut zColl: *const i8 =
                sqlite3ColumnColl(unsafe { unsafe { (*pTab).aCol }.offset(i as isize) });
            0 as i32;
            unsafe {
                *unsafe { unsafe { (*pPk).aiColumn }.offset(j as isize) } = i as i16;
            }
            unsafe {
                *unsafe { unsafe { (*pPk).azColl }.offset(j as isize) } =
                    if zColl != std::ptr::null::<i8>() {
                        zColl
                    } else {
                        unsafe { std::ptr::addr_of!(sqlite3StrBINARY) as *const i8 }
                    };
            }
            let __v2792: i32 = j;
            let __v2793: i32 = __v2792 + (1 as i32);
            j = __v2793;
        }
        let __v2790: i32 = i;
        let __v2791: i32 = __v2790 + (1 as i32);
        i = __v2791;
    }
    0 as i32;
    0 as i32;
    recomputeColumnsNotIndexed(pPk);
}

/// Return true if pTab is a virtual table and zName is a shadow table name
/// for that virtual table.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3IsShadowTableOf(
    mut db: *mut sqlite3,
    mut pTab: *mut Table,
    mut zName: *const i8,
) -> i32 {
    let mut nName: i32 = 0 as i32; // Length of zName
    let mut pMod: *mut Module = unsafe { std::mem::zeroed() }; // Module for the virtual table
    if !((((unsafe { (*pTab).eTabType }) as u32) as i32) == (1 as i32)) {
        return 0 as i32;
    }
    nName = unsafe { sqlite3Strlen30((unsafe { (*pTab).zName }) as *const i8) };
    if (unsafe { sqlite3_strnicmp(zName, (unsafe { (*pTab).zName }) as *const i8, nName) })
        != (0 as i32)
    {
        return 0 as i32;
    }
    if ((unsafe { *unsafe { zName.offset(nName as isize) } }) as i32) != (95 as i32) {
        return 0 as i32;
    }
    pMod = (unsafe {
        sqlite3HashFind(
            (unsafe { std::ptr::addr_of_mut!((*db).aModule) }) as *const Hash,
            (unsafe { *unsafe { unsafe { (*pTab).u.vtab.azArg }.offset((0 as i32) as isize) } })
                as *const i8,
        )
    }) as *mut Module;
    if pMod == std::ptr::null_mut::<Module>() {
        return 0 as i32;
    }
    if (unsafe { (*unsafe { (*pMod).pModule }).iVersion }) < (3 as i32) {
        return 0 as i32;
    }
    if (unsafe { (*unsafe { (*pMod).pModule }).xShadowName }) == None {
        return 0 as i32;
    }
    return unsafe {
        unsafe { (*unsafe { (*pMod).pModule }).xShadowName }.unwrap()(unsafe {
            unsafe { zName.offset(nName as isize) }.offset((1 as i32) as isize)
        })
    };
}

/// Table pTab is a virtual table.  If it the virtual table implementation
/// exists and has an xShadowName method, then loop over all other ordinary
/// tables within the same schema looking for shadow tables of pTab, and mark
/// any shadow tables seen using the TF_Shadow flag.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3MarkAllShadowTablesOf(mut db: *mut sqlite3, mut pTab: *mut Table) {
    let mut nName: i32 = 0 as i32; // Length of pTab->zName
    let mut pMod: *mut Module = unsafe { std::mem::zeroed() }; // Module for the virtual table
    let mut k: *mut HashElem = unsafe { std::mem::zeroed() }; // For looping through the symbol table
    0 as i32;
    pMod = (unsafe {
        sqlite3HashFind(
            (unsafe { std::ptr::addr_of_mut!((*db).aModule) }) as *const Hash,
            (unsafe { *unsafe { unsafe { (*pTab).u.vtab.azArg }.offset((0 as i32) as isize) } })
                as *const i8,
        )
    }) as *mut Module;
    if pMod == std::ptr::null_mut::<Module>() {
        return;
    }
    if (unsafe { (*pMod).pModule }) == std::ptr::null::<sqlite3_module>() {
        return;
    }
    if (unsafe { (*unsafe { (*pMod).pModule }).iVersion }) < (3 as i32) {
        return;
    }
    if (unsafe { (*unsafe { (*pMod).pModule }).xShadowName }) == None {
        return;
    }
    0 as i32;
    nName = unsafe { sqlite3Strlen30((unsafe { (*pTab).zName }) as *const i8) };
    k = unsafe {
        (*unsafe { std::ptr::addr_of_mut!((*unsafe { (*pTab).pSchema }).tblHash) }).first
    };
    '__slate_break_2127: while k != std::ptr::null_mut::<HashElem>() {
        let mut pOther: *mut Table = (unsafe { (*k).data }) as *mut Table;
        0 as i32;
        if !((((unsafe { (*pOther).eTabType }) as u32) as i32) == (0 as i32)) {
        } else {
            if (unsafe { (*pOther).tabFlags }) & ((4096 as i32) as u32) != (0 as u32) {
            } else {
                let __v2664: bool;
                if (unsafe {
                    sqlite3_strnicmp(
                        (unsafe { (*pOther).zName }) as *const i8,
                        (unsafe { (*pTab).zName }) as *const i8,
                        nName,
                    )
                }) == (0 as i32)
                    && ((unsafe { *unsafe { unsafe { (*pOther).zName }.offset(nName as isize) } })
                        as i32)
                        == (95 as i32)
                {
                    __v2664 = (unsafe {
                        unsafe { (*unsafe { (*pMod).pModule }).xShadowName }.unwrap()(
                            (unsafe {
                                unsafe { unsafe { (*pOther).zName }.offset(nName as isize) }
                                    .offset((1 as i32) as isize)
                            }) as *const i8,
                        )
                    }) != (0 as i32);
                } else {
                    __v2664 = false as bool;
                }
                if __v2664 {
                    let __v2665: *mut Table = pOther;
                    let __v2666: u32 = unsafe { (*__v2665).tabFlags };
                    let __v2667: u32 = __v2666 | ((4096 as i32) as u32);
                    unsafe {
                        (*__v2665).tabFlags = __v2667;
                    }
                }
            }
        }
        k = unsafe { (*k).next };
    }
}

/// Return true if zName is a shadow table name in the current database
/// connection.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3ShadowTableName(mut db: *mut sqlite3, mut zName: *const i8) -> i32 {
    let mut zTail: *const i8 = unsafe { std::mem::zeroed() }; // Pointer to the last "_" in zName
    let mut pTab: *mut Table = unsafe { std::mem::zeroed() }; // Table that zName is a shadow of
    let mut zCopy: *mut i8 = unsafe { std::mem::zeroed() }; // Transient copy of zName after last "_"
    zTail = (unsafe { strrchr(zName, 95 as i32) }) as *const i8;
    if zTail == std::ptr::null::<i8>() {
        return 0 as i32;
    }
    zCopy = unsafe {
        sqlite3DbStrNDup(
            db,
            zName,
            ((((unsafe { zTail.offset_from(zName as *const i8) }) as i64) as i32) as i64) as u64,
        )
    };
    let __v2663: *mut Table;
    if zCopy != std::ptr::null_mut::<i8>() {
        __v2663 = sqlite3FindTable(db, zCopy as *const i8, std::ptr::null::<i8>());
    } else {
        __v2663 = std::ptr::null_mut::<Table>();
    }
    pTab = __v2663;
    unsafe { sqlite3DbFree(db, zCopy as *mut ()) };
    if pTab == std::ptr::null_mut::<Table>() {
        return 0 as i32;
    }
    if !((((unsafe { (*pTab).eTabType }) as u32) as i32) == (1 as i32)) {
        return 0 as i32;
    }
    return sqlite3IsShadowTableOf(db, pTab, zName);
}

// no-op
/// This routine is called to report the final ")" that terminates
/// a CREATE TABLE statement.
///
/// The table structure that other action routines have been building
/// is added to the internal hash tables, assuming no errors have
/// occurred.
///
/// An entry for the table is made in the schema table on disk, unless
/// this is a temporary table or db->init.busy==1.  When db->init.busy==1
/// it means we are reading the sqlite_schema table because we just
/// connected to the database or because the sqlite_schema table has
/// recently changed, so the entry for this table already exists in
/// the sqlite_schema table.  We do not want to create it again.
///
/// If the pSelect argument is not NULL, it means that this routine
/// was called to create a table generated from a
/// "CREATE TABLE ... AS SELECT ..." statement.  The column names of
/// the new table will match the result set of the SELECT.
///
/// # Arguments
///
/// * `pParse` - Parse context
/// * `pCons` - The ',' token after the last column defn.
/// * `pEnd` - The ')' before options in the CREATE TABLE
/// * `tabOpts` - Extra table options. Usually 0.
/// * `pSelect` - Select from a "CREATE ... AS SELECT"
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3EndTable(
    mut pParse: *mut Parse,
    mut pCons: *mut Token,
    mut pEnd: *mut Token,
    mut tabOpts: u32,
    mut pSelect: *mut Select,
) {
    let mut p: *mut Table = unsafe { std::mem::zeroed() }; // The new table
    let mut db: *mut sqlite3 = unsafe { (*pParse).db }; // The database connection
    let mut iDb: i32 = 0 as i32; // Database in which the table lives
    let mut pIdx: *mut Index = unsafe { std::mem::zeroed() }; // An implied index of the table
    if pEnd == std::ptr::null_mut::<Token>() && pSelect == std::ptr::null_mut::<Select>() {
        return;
    }
    p = unsafe { (*pParse).pNewTable };
    if p == std::ptr::null_mut::<Table>() {
        return;
    }
    let __v2396: bool;
    if pSelect == std::ptr::null_mut::<Select>() {
        __v2396 = sqlite3ShadowTableName(db, (unsafe { (*p).zName }) as *const i8) != (0 as i32);
    } else {
        __v2396 = false as bool;
    }
    if __v2396 {
        let __v2397: *mut Table = p;
        let __v2398: u32 = unsafe { (*__v2397).tabFlags };
        let __v2399: u32 = __v2398 | ((4096 as i32) as u32);
        unsafe {
            (*__v2397).tabFlags = __v2399;
        }
    }
    // If the db->init.busy is 1 it means we are reading the SQL off the
    // "sqlite_schema" or "sqlite_temp_schema" table on the disk.
    // So do not write to the disk again.  Extract the root page number
    // for the table from the db->init.newTnum field.  (The page number
    // should have been put there by the sqliteOpenCb routine.)
    //
    // If the root page number is 1, that means this is the sqlite_schema
    // table itself.  So mark it read-only.
    if (unsafe { (*db).init.busy }) != (0 as u8) {
        if pSelect != std::ptr::null_mut::<Select>()
            || !((((unsafe { (*p).eTabType }) as u32) as i32) == (0 as i32))
                && (unsafe { (*db).init.newTnum }) != (0 as u32)
        {
            unsafe { sqlite3ErrorMsg(pParse, (b"\0".as_ptr() as *mut i8) as *const i8) };
            return;
        }
        unsafe {
            (*p).tnum = unsafe { (*db).init.newTnum };
        }
        if (unsafe { (*p).tnum }) == ((1 as i32) as u32) {
            let __v2400: *mut Table = p;
            let __v2401: u32 = unsafe { (*__v2400).tabFlags };
            let __v2402: u32 = __v2401 | ((1 as i32) as u32);
            unsafe {
                (*__v2400).tabFlags = __v2402;
            }
        }
    }
    // Special processing for tables that include the STRICT keyword:
    //
    // *  Do not allow custom column datatypes.  Every column must have
    //    a datatype that is one of INT, INTEGER, REAL, TEXT, or BLOB.
    //
    // *  If a PRIMARY KEY is defined, other than the INTEGER PRIMARY KEY,
    //    then all columns of the PRIMARY KEY must have a NOT NULL
    //    constraint.
    if tabOpts & ((65536 as i32) as u32) != (0 as u32) {
        let mut ii: i32 = 0 as i32;
        let __v2403: *mut Table = p;
        let __v2404: u32 = unsafe { (*__v2403).tabFlags };
        let __v2405: u32 = __v2404 | ((65536 as i32) as u32);
        unsafe {
            (*__v2403).tabFlags = __v2405;
        }
        ii = 0 as i32;
        '__slate_break_2129: loop {
            if !(ii < ((unsafe { (*p).nCol }) as i32)) {
                break;
            }
            let mut pCol: *mut Column = unsafe { unsafe { (*p).aCol }.offset(ii as isize) };
            if ((unsafe { (*pCol).__slate_bits_0.__get_eCType() }) as i32) == (0 as i32) {
                if (((unsafe { (*pCol).colFlags }) as u32) as i32) & (4 as i32) != (0 as i32) {
                    unsafe {
                        sqlite3ErrorMsg(
                            pParse,
                            (b"unknown datatype for %s.%s: \"%s\"\0".as_ptr() as *mut i8)
                                as *const i8,
                            unsafe { (*p).zName },
                            unsafe { (*pCol).zCnName },
                            unsafe { sqlite3ColumnType(pCol, b"\0".as_ptr() as *mut i8) },
                        )
                    };
                } else {
                    unsafe {
                        sqlite3ErrorMsg(
                            pParse,
                            (b"missing datatype for %s.%s\0".as_ptr() as *mut i8) as *const i8,
                            unsafe { (*p).zName },
                            unsafe { (*pCol).zCnName },
                        )
                    };
                }
                return;
            } else {
                if ((unsafe { (*pCol).__slate_bits_0.__get_eCType() }) as i32) == (1 as i32) {
                    unsafe {
                        (*pCol).affinity = (65 as i32) as i8;
                    }
                }
            }
            if (((unsafe { (*pCol).colFlags }) as u32) as i32) & (1 as i32) != (0 as i32)
                && ((unsafe { (*p).iPKey }) as i32) != ii
                && ((unsafe { (*pCol).__slate_bits_0.__get_notNull() }) as i32) == (0 as i32)
            {
                unsafe {
                    (*pCol).__slate_bits_0.__set_notNull((2 as i32) as u32);
                }
                let __v2408: *mut Table = p;
                let __v2409: u32 = unsafe { (*__v2408).tabFlags };
                let __v2410: u32 = __v2409 | ((2048 as i32) as u32);
                unsafe {
                    (*__v2408).tabFlags = __v2410;
                }
            }
            let __v2406: i32 = ii;
            let __v2407: i32 = __v2406 + (1 as i32);
            ii = __v2407;
        }
    }
    0 as i32;
    0 as i32;
    // Special processing for WITHOUT ROWID Tables
    if tabOpts & ((128 as i32) as u32) != (0 as u32) {
        if (unsafe { (*p).tabFlags }) & ((8 as i32) as u32) != (0 as u32) {
            unsafe {
                sqlite3ErrorMsg(
                    pParse,
                    (b"AUTOINCREMENT not allowed on WITHOUT ROWID tables\0".as_ptr() as *mut i8)
                        as *const i8,
                )
            };
            return;
        }
        if (unsafe { (*p).tabFlags }) & ((4 as i32) as u32) == ((0 as i32) as u32) {
            unsafe {
                sqlite3ErrorMsg(
                    pParse,
                    (b"PRIMARY KEY missing on table %s\0".as_ptr() as *mut i8) as *const i8,
                    unsafe { (*p).zName },
                )
            };
            return;
        }
        let __v2411: *mut Table = p;
        let __v2412: u32 = unsafe { (*__v2411).tabFlags };
        let __v2413: u32 = __v2412 | (((128 as i32) | (512 as i32)) as u32);
        unsafe {
            (*__v2411).tabFlags = __v2413;
        }
        convertToWithoutRowidTable(pParse, p);
    }
    iDb = unsafe { sqlite3SchemaToIndex(db, unsafe { (*p).pSchema }) };
    0 as i32;
    // Resolve names in all CHECK constraint expressions.
    if (unsafe { (*p).pCheck }) != std::ptr::null_mut::<ExprList>() {
        unsafe {
            sqlite3ResolveSelfReference(pParse, p, 4 as i32, std::ptr::null_mut::<Expr>(), unsafe {
                (*p).pCheck
            })
        };
        if (unsafe { (*pParse).nErr }) != (0 as i32) {
            // If errors are seen, delete the CHECK constraints now, else they might
            // actually be used if PRAGMA writable_schema=ON is set.
            unsafe { sqlite3ExprListDelete(db, unsafe { (*p).pCheck }) };
            unsafe {
                (*p).pCheck = std::ptr::null_mut::<ExprList>();
            }
        } else {
            {}
        }
    }
    if (unsafe { (*p).tabFlags }) & ((96 as i32) as u32) != (0 as u32) {
        let mut ii: i32 = 0 as i32;
        let mut nNG: i32 = 0 as i32;
        {}
        {}
        ii = 0 as i32;
        '__slate_break_2135: loop {
            if !(ii < ((unsafe { (*p).nCol }) as i32)) {
                break;
            }
            let mut colFlags: u32 =
                (unsafe { (*unsafe { unsafe { (*p).aCol }.offset(ii as isize) }).colFlags }) as u32;
            if colFlags & ((96 as i32) as u32) != ((0 as i32) as u32) {
                let mut pX: *mut Expr =
                    sqlite3ColumnExpr(p, unsafe { unsafe { (*p).aCol }.offset(ii as isize) });
                {}
                {}
                if (unsafe {
                    sqlite3ResolveSelfReference(
                        pParse,
                        p,
                        8 as i32,
                        pX,
                        std::ptr::null_mut::<ExprList>(),
                    )
                }) != (0 as i32)
                {
                    // If there are errors in resolving the expression, change the
                    // expression to a NULL.  This prevents code generators that operate
                    // on the expression from inserting extra parts into the expression
                    // tree that have been allocated from lookaside memory, which is
                    // illegal in a schema and will lead to errors or heap corruption
                    // when the database connection closes.
                    sqlite3ColumnSetExpr(
                        pParse,
                        p,
                        unsafe { unsafe { (*p).aCol }.offset(ii as isize) },
                        unsafe {
                            sqlite3ExprAlloc(db, 122 as i32, std::ptr::null::<Token>(), 0 as i32)
                        },
                    );
                }
            } else {
                let __v2416: i32 = nNG;
                let __v2417: i32 = __v2416 + (1 as i32);
                nNG = __v2417;
            }
            let __v2414: i32 = ii;
            let __v2415: i32 = __v2414 + (1 as i32);
            ii = __v2415;
        }
        if nNG == (0 as i32) {
            unsafe {
                sqlite3ErrorMsg(
                    pParse,
                    (b"must have at least one non-generated column\0".as_ptr() as *mut i8)
                        as *const i8,
                )
            };
            return;
        }
    }
    // Estimate the average row size for the table and for all implied indices
    estimateTableWidth(p);
    pIdx = unsafe { (*p).pIndex };
    '__slate_break_2137: while pIdx != std::ptr::null_mut::<Index>() {
        estimateIndexWidth(pIdx);
        pIdx = unsafe { (*pIdx).pNext };
    }
    // If not initializing, then create a record for the new table
    // in the schema table of the database.
    //
    // If this is a TEMPORARY table, write the entry into the auxiliary
    // file instead of into the main database file.
    if !((unsafe { (*db).init.busy }) != (0 as u8)) {
        let mut n: i32 = 0 as i32;
        let mut v: *mut Vdbe = unsafe { std::mem::zeroed() };
        let mut zType: *mut i8 = unsafe { std::mem::zeroed() }; // "view" or "table"
        let mut zType2: *mut i8 = unsafe { std::mem::zeroed() }; // "VIEW" or "TABLE"
        let mut zStmt: *mut i8 = unsafe { std::mem::zeroed() }; // Text of the CREATE TABLE or CREATE VIEW statement
        v = unsafe { sqlite3GetVdbe(pParse) };
        if v == std::ptr::null_mut::<Vdbe>() {
            return;
        }
        unsafe { sqlite3VdbeAddOp1(v, 124 as i32, 0 as i32) };
        // Initialize zType for the new view or table.
        if (((unsafe { (*p).eTabType }) as u32) as i32) == (0 as i32) {
            // A regular table
            zType = b"table\0".as_ptr() as *mut i8;
            zType2 = b"TABLE\0".as_ptr() as *mut i8;
        } else {
            // A view
            zType = b"view\0".as_ptr() as *mut i8;
            zType2 = b"VIEW\0".as_ptr() as *mut i8;
        }
        // If this is a CREATE TABLE xx AS SELECT ..., execute the SELECT
        // statement to populate the new table. The root-page number for the
        // new table is in register pParse->u1.cr.regRoot.
        //
        // Once the SELECT has been coded by sqlite3Select(), it is in a
        // suitable state to query for the column names and types to be used
        // by the new table.
        //
        // A shared-cache write-lock is not required to write to the new table,
        // as a schema-lock must have already been obtained to create it. Since
        // a schema-lock excludes all other database users, the write-lock would
        // be redundant.
        if pSelect != std::ptr::null_mut::<Select>() {
            let mut dest: SelectDest = unsafe { std::mem::zeroed() }; // Where the SELECT should store results
            let mut regYield: i32 = 0 as i32; // Register holding co-routine entry-point
            let mut addrTop: i32 = 0 as i32; // Top of the co-routine
            let mut regRec: i32 = 0 as i32; // A record to be insert into the new table
            let mut regRowid: i32 = 0 as i32; // Rowid of the next row to insert
            let mut addrInsLoop: i32 = 0 as i32; // Top of the loop for inserting rows
            let mut pSelTab: *mut Table = unsafe { std::mem::zeroed() }; // A table that describes the SELECT results
            let mut iCsr: i32 = 0 as i32; // Write cursor on the new table
            if (((unsafe { (*pParse).eParseMode }) as u32) as i32) != (0 as i32) {
                unsafe {
                    (*pParse).rc = 1 as i32;
                }
                let __v2418: *mut Parse = pParse;
                let __v2419: i32 = unsafe { (*__v2418).nErr };
                let __v2420: i32 = __v2419 + (1 as i32);
                unsafe {
                    (*__v2418).nErr = __v2420;
                }
                return;
            }
            let __v2421: *mut Parse = pParse;
            let __v2422: i32 = unsafe { (*__v2421).nTab };
            let __v2423: i32 = __v2422 + (1 as i32);
            unsafe {
                (*__v2421).nTab = __v2423;
            }
            iCsr = __v2422;
            let __v2424: *mut Parse = pParse;
            let __v2425: i32 = unsafe { (*__v2424).nMem };
            let __v2426: i32 = __v2425 + (1 as i32);
            unsafe {
                (*__v2424).nMem = __v2426;
            }
            regYield = __v2426;
            let __v2427: *mut Parse = pParse;
            let __v2428: i32 = unsafe { (*__v2427).nMem };
            let __v2429: i32 = __v2428 + (1 as i32);
            unsafe {
                (*__v2427).nMem = __v2429;
            }
            regRec = __v2429;
            let __v2430: *mut Parse = pParse;
            let __v2431: i32 = unsafe { (*__v2430).nMem };
            let __v2432: i32 = __v2431 + (1 as i32);
            unsafe {
                (*__v2430).nMem = __v2432;
            }
            regRowid = __v2432;
            sqlite3MayAbort(pParse);
            0 as i32;
            unsafe {
                sqlite3VdbeAddOp3(v, 116 as i32, iCsr, unsafe { (*pParse).u1.cr.regRoot }, iDb)
            };
            unsafe { sqlite3VdbeChangeP5(v, ((16 as i32) as i16) as u16) };
            addrTop = (unsafe { sqlite3VdbeCurrentAddr(v) }) + (1 as i32);
            unsafe { sqlite3VdbeAddOp3(v, 11 as i32, regYield, 0 as i32, addrTop) };
            if (unsafe { (*pParse).nErr }) != (0 as i32) {
                return;
            }
            pSelTab = unsafe { sqlite3ResultSetOfSelect(pParse, pSelect, (65 as i32) as i8) };
            if pSelTab == std::ptr::null_mut::<Table>() {
                return;
            }
            0 as i32;
            let __v2433: i16 = unsafe { (*pSelTab).nCol };
            unsafe {
                (*p).nNVCol = __v2433;
            }
            unsafe {
                (*p).nCol = __v2433;
            }
            unsafe {
                (*p).aCol = unsafe { (*pSelTab).aCol };
            }
            unsafe {
                (*pSelTab).nCol = (0 as i32) as i16;
            }
            unsafe {
                (*pSelTab).aCol = std::ptr::null_mut::<Column>();
            }
            sqlite3DeleteTable(db, pSelTab);
            unsafe { sqlite3SelectDestInit(std::ptr::addr_of_mut!(dest), 11 as i32, regYield) };
            unsafe { sqlite3Select(pParse, pSelect, std::ptr::addr_of_mut!(dest)) };
            if (unsafe { (*pParse).nErr }) != (0 as i32) {
                return;
            }
            unsafe { sqlite3VdbeEndCoroutine(v, regYield) };
            unsafe { sqlite3VdbeJumpHere(v, addrTop - (1 as i32)) };
            addrInsLoop = unsafe { sqlite3VdbeAddOp1(v, 12 as i32, dest.iSDParm) };
            {}
            unsafe { sqlite3VdbeAddOp3(v, 99 as i32, dest.iSdst, dest.nSdst, regRec) };
            unsafe { sqlite3TableAffinity(v, p, 0 as i32) };
            unsafe { sqlite3VdbeAddOp2(v, 129 as i32, iCsr, regRowid) };
            unsafe { sqlite3VdbeAddOp3(v, 130 as i32, iCsr, regRec, regRowid) };
            unsafe { sqlite3VdbeGoto(v, addrInsLoop) };
            unsafe { sqlite3VdbeJumpHere(v, addrInsLoop) };
            unsafe { sqlite3VdbeAddOp1(v, 124 as i32, iCsr) };
        }
        // Compute the complete text of the CREATE statement
        if pSelect != std::ptr::null_mut::<Select>() {
            zStmt = createTableStmt(db, p);
        } else {
            let mut pEnd2: *mut Token = if tabOpts != (0 as u32) {
                unsafe { std::ptr::addr_of_mut!((*pParse).sLastToken) }
            } else {
                pEnd
            };
            n = ((unsafe {
                unsafe { (*pEnd2).z }.offset_from((unsafe { (*pParse).sNameToken.z }) as *const i8)
            }) as i64) as i32;
            if ((unsafe { *unsafe { unsafe { (*pEnd2).z }.offset((0 as i32) as isize) } }) as i32)
                != (59 as i32)
            {
                let __v2434: i32 = n;
                let __v2435: i32 = (__v2434 as u32).wrapping_add(unsafe { (*pEnd2).n }) as i32;
                n = __v2435;
            }
            zStmt = unsafe {
                sqlite3MPrintf(
                    db,
                    (b"CREATE %s %.*s\0".as_ptr() as *mut i8) as *const i8,
                    zType2,
                    n,
                    unsafe { (*pParse).sNameToken.z },
                )
            };
        }
        // A slot for the record has already been allocated in the
        // schema table.  We just need to update that slot with all
        // the information we've collected.
        0 as i32;
        unsafe {
            sqlite3NestedParse(pParse, (b"UPDATE %Q.sqlite_master SET type='%s', name=%Q, tbl_name=%Q, rootpage=#%d, sql=%Q WHERE rowid=#%d\0".as_ptr() as *mut i8) as *const i8, unsafe { (*unsafe { unsafe { (*db).aDb }.offset(iDb as isize) }).zDbSName }, zType, unsafe { (*p).zName }, unsafe { (*p).zName }, unsafe { (*pParse).u1.cr.regRoot }, zStmt, unsafe { (*pParse).u1.cr.regRowid })
        };
        unsafe { sqlite3DbFree(db, zStmt as *mut ()) };
        sqlite3ChangeCookie(pParse, iDb);
        // Check to see if we need to create an sqlite_sequence table for
        // keeping track of autoincrement keys.
        if (unsafe { (*p).tabFlags }) & ((8 as i32) as u32) != ((0 as i32) as u32)
            && !((((unsafe { (*pParse).eParseMode }) as u32) as i32) != (0 as i32))
        {
            let mut pDb: *mut Db = unsafe { unsafe { (*db).aDb }.offset(iDb as isize) };
            0 as i32;
            if (unsafe { (*unsafe { (*pDb).pSchema }).pSeqTab }) == std::ptr::null_mut::<Table>() {
                unsafe {
                    sqlite3NestedParse(
                        pParse,
                        (b"CREATE TABLE %Q.sqlite_sequence(name,seq)\0".as_ptr() as *mut i8)
                            as *const i8,
                        unsafe { (*pDb).zDbSName },
                    )
                };
            }
        }
        // Reparse everything to update our internal data structures
        unsafe {
            sqlite3VdbeAddParseSchemaOp(
                v,
                iDb,
                unsafe {
                    sqlite3MPrintf(
                        db,
                        (b"tbl_name='%q' AND type!='trigger'\0".as_ptr() as *mut i8) as *const i8,
                        unsafe { (*p).zName },
                    )
                },
                ((0 as i32) as i16) as u16,
            )
        };
        // Test for cycles in generated columns and illegal expressions
        // in CHECK constraints and in DEFAULT clauses.
        if (unsafe { (*p).tabFlags }) & ((96 as i32) as u32) != (0 as u32) {
            unsafe {
                sqlite3VdbeAddOp4(
                    v,
                    150 as i32,
                    1 as i32,
                    0 as i32,
                    0 as i32,
                    (unsafe {
                        sqlite3MPrintf(
                            db,
                            (b"SELECT*FROM\"%w\".\"%w\"\0".as_ptr() as *mut i8) as *const i8,
                            unsafe {
                                (*unsafe { unsafe { (*db).aDb }.offset(iDb as isize) }).zDbSName
                            },
                            unsafe { (*p).zName },
                        )
                    }) as *const i8,
                    -(7 as i32),
                )
            };
        }
    }
    // Add the table to the in-memory representation of the database.
    if (unsafe { (*db).init.busy }) != (0 as u8) {
        let mut pOld: *mut Table = unsafe { std::mem::zeroed() };
        let mut pSchema: *mut Schema = unsafe { (*p).pSchema };
        0 as i32;
        0 as i32;
        pOld = (unsafe {
            sqlite3HashInsert(
                unsafe { std::ptr::addr_of_mut!((*pSchema).tblHash) },
                (unsafe { (*p).zName }) as *const i8,
                p as *mut (),
            )
        }) as *mut Table;
        if pOld != std::ptr::null_mut::<Table>() {
            0 as i32; // Malloc must have failed inside HashInsert()
            unsafe { sqlite3OomFault(db) };
            return;
        }
        unsafe {
            (*pParse).pNewTable = std::ptr::null_mut::<Table>();
        }
        let __v2436: *mut sqlite3 = db;
        let __v2437: u32 = unsafe { (*__v2436).mDbFlags };
        let __v2438: u32 = __v2437 | ((1 as i32) as u32);
        unsafe {
            (*__v2436).mDbFlags = __v2438;
        }
        // If this is the magic sqlite_sequence table used by autoincrement,
        // then record a pointer to this table in the main database structure
        // so that INSERT can find the table easily.
        0 as i32;
        if (unsafe {
            strcmp(
                (unsafe { (*p).zName }) as *const i8,
                (b"sqlite_sequence\0".as_ptr() as *mut i8) as *const i8,
            )
        }) == (0 as i32)
        {
            0 as i32;
            unsafe {
                (*unsafe { (*p).pSchema }).pSeqTab = p;
            }
        }
    }
    if !(pSelect != std::ptr::null_mut::<Select>())
        && (((unsafe { (*p).eTabType }) as u32) as i32) == (0 as i32)
    {
        0 as i32;
        if (unsafe { (*pCons).z }) == std::ptr::null::<i8>() {
            pCons = pEnd;
        }
        unsafe {
            (*p).u.tab.addColOffset = (13 as i32)
                + (((unsafe {
                    unsafe { (*pCons).z }
                        .offset_from((unsafe { (*pParse).sNameToken.z }) as *const i8)
                }) as i64) as i32);
        }
    }
}

/// The parser calls this routine in order to create a new VIEW
///
/// # Arguments
///
/// * `pParse` - The parsing context
/// * `pBegin` - The CREATE token that begins the statement
/// * `pName1` - The token that holds the name of the view
/// * `pName2` - The token that holds the name of the view
/// * `pCNames` - Optional list of view column names
/// * `pSelect` - A SELECT statement that will become the new view
/// * `isTemp` - TRUE for a TEMPORARY view
/// * `noErr` - Suppress error messages if VIEW already exists
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3CreateView(
    mut pParse: *mut Parse,
    mut pBegin: *mut Token,
    mut pName1: *mut Token,
    mut pName2: *mut Token,
    mut pCNames: *mut ExprList,
    mut pSelect: *mut Select,
    mut isTemp: i32,
    mut noErr: i32,
) {
    let mut __slate_storage_2448: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2448: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_2448) as *mut i32;
    let mut __slate_storage_2447: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2447: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_2447) as *mut i32;
    let mut __slate_storage_2446: std::mem::MaybeUninit<*const i8> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_2446: *mut *const i8 =
        std::ptr::addr_of_mut!(__slate_storage_2446) as *mut *const i8;
    let mut __slate_storage_2445: std::mem::MaybeUninit<*const i8> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_2445: *mut *const i8 =
        std::ptr::addr_of_mut!(__slate_storage_2445) as *mut *const i8;
    let mut __slate_storage_2444: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2444: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_2444) as *mut u32;
    let mut __slate_storage_2443: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2443: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_2443) as *mut u32;
    // Make a copy of the entire SELECT statement that defines the view.
    // This will force all the Expr.token.z values to be dynamically
    // allocated rather than point to the input string - which means that
    // they will persist after the current sqlite3_exec() call returns.
    let mut __slate_storage_2442: std::mem::MaybeUninit<*mut Select> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_2442: *mut *mut Select =
        std::ptr::addr_of_mut!(__slate_storage_2442) as *mut *mut Select;
    let mut __slate_storage_2441: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2441: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_2441) as *mut u32;
    let mut __slate_storage_2440: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2440: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_2440) as *mut u32;
    // Legacy versions of SQLite allowed the use of the magic "rowid" column
    // on a view, even though views do not have rowids.  The following flag
    // setting fixes this problem.  But the fix can be disabled by compiling
    // with -DSQLITE_ALLOW_ROWID_IN_VIEW in case there are legacy apps that
    // depend upon the old buggy behavior.  The ability can also be toggled
    // using sqlite3_config(SQLITE_CONFIG_ROWID_IN_VIEW,...)
    let mut __slate_storage_2439: std::mem::MaybeUninit<*mut Table> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_2439: *mut *mut Table =
        std::ptr::addr_of_mut!(__slate_storage_2439) as *mut *mut Table;
    let mut __slate_storage_1022: std::mem::MaybeUninit<*mut sqlite3> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1022: *mut *mut sqlite3 =
        std::ptr::addr_of_mut!(__slate_storage_1022) as *mut *mut sqlite3;
    let mut __slate_storage_1021: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1021: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1021) as *mut i32;
    let mut __slate_storage_1020: std::mem::MaybeUninit<*mut Token> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1020: *mut *mut Token =
        std::ptr::addr_of_mut!(__slate_storage_1020) as *mut *mut Token;
    let mut __slate_storage_1019: std::mem::MaybeUninit<DbFixer> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1019: *mut DbFixer =
        std::ptr::addr_of_mut!(__slate_storage_1019) as *mut DbFixer;
    let mut __slate_storage_1018: std::mem::MaybeUninit<Token> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1018: *mut Token = std::ptr::addr_of_mut!(__slate_storage_1018) as *mut Token;
    let mut __slate_storage_1017: std::mem::MaybeUninit<*const i8> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1017: *mut *const i8 =
        std::ptr::addr_of_mut!(__slate_storage_1017) as *mut *const i8;
    let mut __slate_storage_1016: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1016: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1016) as *mut i32;
    let mut __slate_storage_1015: std::mem::MaybeUninit<*mut Table> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1015: *mut *mut Table =
        std::ptr::addr_of_mut!(__slate_storage_1015) as *mut *mut Table;
    unsafe {
        '__join_2: {
            std::ptr::write(__slate_slot_1020, std::ptr::null_mut::<Token>());
            std::ptr::write(__slate_slot_1022, unsafe { (*pParse).db });
            if ((unsafe { (*pParse).nVar }) as i32) > (0 as i32) {
                unsafe {
                    sqlite3ErrorMsg(
                        pParse,
                        (b"parameters are not allowed in views\0".as_ptr() as *mut i8) as *const i8,
                    )
                };
            } else {
                sqlite3StartTable(pParse, pName1, pName2, isTemp, 1 as i32, 0 as i32, noErr);
                *__slate_slot_1015 = unsafe { (*pParse).pNewTable };
                if *__slate_slot_1015 == std::ptr::null_mut::<Table>()
                    || (unsafe { (*pParse).nErr }) != (0 as i32)
                {
                } else {
                    std::ptr::write(__slate_slot_2439, *__slate_slot_1015);
                    std::ptr::write(__slate_slot_2440, unsafe {
                        (*(*__slate_slot_2439)).tabFlags
                    });
                    std::ptr::write(
                        __slate_slot_2441,
                        *__slate_slot_2440 | ((512 as i32) as u32),
                    );
                    unsafe {
                        (*(*__slate_slot_2439)).tabFlags = *__slate_slot_2441;
                    }
                    // Never allow rowid in view
                    sqlite3TwoPartName(
                        pParse,
                        pName1,
                        pName2,
                        std::ptr::addr_of_mut!(*__slate_slot_1020),
                    );
                    *__slate_slot_1021 = unsafe {
                        sqlite3SchemaToIndex(*__slate_slot_1022, unsafe {
                            (*(*__slate_slot_1015)).pSchema
                        })
                    };
                    0 as i32;
                    unsafe {
                        sqlite3FixInit(
                            std::ptr::addr_of_mut!(*__slate_slot_1019),
                            pParse,
                            *__slate_slot_1021,
                            (b"view\0".as_ptr() as *mut i8) as *const i8,
                            *__slate_slot_1020 as *const Token,
                        )
                    };
                    if (unsafe {
                        sqlite3FixSelect(std::ptr::addr_of_mut!(*__slate_slot_1019), pSelect)
                    }) != (0 as i32)
                    {
                    } else {
                        std::ptr::write(__slate_slot_2442, pSelect);
                        std::ptr::write(__slate_slot_2443, unsafe {
                            (*(*__slate_slot_2442)).selFlags
                        });
                        std::ptr::write(
                            __slate_slot_2444,
                            *__slate_slot_2443 | ((2097152 as i32) as u32),
                        );
                        unsafe {
                            (*(*__slate_slot_2442)).selFlags = *__slate_slot_2444;
                        }
                        if (((unsafe { (*pParse).eParseMode }) as u32) as i32) >= (2 as i32) {
                            unsafe {
                                (*(*__slate_slot_1015)).u.view.pSelect = pSelect;
                            }
                            pSelect = std::ptr::null_mut::<Select>();
                        } else {
                            unsafe {
                                (*(*__slate_slot_1015)).u.view.pSelect = unsafe {
                                    sqlite3SelectDup(
                                        *__slate_slot_1022,
                                        pSelect as *const Select,
                                        1 as i32,
                                    )
                                };
                            }
                        }
                        unsafe {
                            (*(*__slate_slot_1015)).pCheck = unsafe {
                                sqlite3ExprListDup(
                                    *__slate_slot_1022,
                                    pCNames as *const ExprList,
                                    1 as i32,
                                )
                            };
                        }
                        unsafe {
                            (*(*__slate_slot_1015)).eTabType = ((2 as i32) as i8) as u8;
                        }
                        if (unsafe { (*(*__slate_slot_1022)).mallocFailed }) != (0 as u8) {
                        } else {
                            // Locate the end of the CREATE VIEW statement.  Make sEnd point to
                            // the end.
                            *__slate_slot_1018 = unsafe { (*pParse).sLastToken };
                            0 as i32;
                            if ((unsafe {
                                *unsafe { (*__slate_slot_1018).z.offset((0 as i32) as isize) }
                            }) as i32)
                                != (59 as i32)
                            {
                                std::ptr::write(__slate_slot_2445, (*__slate_slot_1018).z);
                                std::ptr::write(__slate_slot_2446, unsafe {
                                    (*__slate_slot_2445).offset((*__slate_slot_1018).n as isize)
                                });
                                (*__slate_slot_1018).z = *__slate_slot_2446;
                            }
                            (*__slate_slot_1018).n = (0 as i32) as u32;
                            *__slate_slot_1016 = ((unsafe {
                                (*__slate_slot_1018)
                                    .z
                                    .offset_from((unsafe { (*pBegin).z }) as *const i8)
                            }) as i64) as i32;
                            0 as i32;
                            *__slate_slot_1017 = unsafe { (*pBegin).z };
                            loop {
                                if (((unsafe {
                                    *unsafe {
                                        unsafe { std::ptr::addr_of!(sqlite3CtypeMap) as *const u8 }
                                            .offset(
                                                ((((unsafe {
                                                    *unsafe {
                                                        (*__slate_slot_1017).offset(
                                                            (*__slate_slot_1016 - (1 as i32))
                                                                as isize,
                                                        )
                                                    }
                                                })
                                                    as u8)
                                                    as u32)
                                                    as i32)
                                                    as isize,
                                            )
                                    }
                                }) as u32) as i32)
                                    & (1 as i32)
                                    != (0 as i32)
                                {
                                    std::ptr::write(__slate_slot_2447, *__slate_slot_1016);
                                    std::ptr::write(
                                        __slate_slot_2448,
                                        *__slate_slot_2447 - (1 as i32),
                                    );
                                    *__slate_slot_1016 = *__slate_slot_2448;
                                } else {
                                    break;
                                }
                            }
                            (*__slate_slot_1018).z = unsafe {
                                (*__slate_slot_1017)
                                    .offset((*__slate_slot_1016 - (1 as i32)) as isize)
                            };
                            (*__slate_slot_1018).n = (1 as i32) as u32;
                            // Use sqlite3EndTable() to add the view to the schema table
                            sqlite3EndTable(
                                pParse,
                                std::ptr::null_mut::<Token>(),
                                std::ptr::addr_of_mut!(*__slate_slot_1018),
                                (0 as i32) as u32,
                                std::ptr::null_mut::<Select>(),
                            );
                        }
                    }
                }
            }
        }
        unsafe { sqlite3SelectDelete(*__slate_slot_1022, pSelect) };
        if (((unsafe { (*pParse).eParseMode }) as u32) as i32) >= (2 as i32) {
            unsafe { sqlite3RenameExprlistUnmap(pParse, pCNames) };
        }
        unsafe { sqlite3ExprListDelete(*__slate_slot_1022, pCNames) };
        return;
    }
}

/// The Table structure pTable is really a VIEW.  Fill in the names of
/// the columns of the view in the pTable structure.  Return non-zero if
/// there are errors.  If an error is seen an error message is left
/// in pParse->zErrMsg.
fn viewGetColumnNames(mut pParse: *mut Parse, mut pTable: *mut Table) -> i32 {
    let mut pSelTab: *mut Table = unsafe { std::mem::zeroed() }; // A fake table from which we get the result set
    let mut pSel: *mut Select = unsafe { std::mem::zeroed() }; // Copy of the SELECT that implements the view
    let mut nErr: i32 = 0 as i32; // Number of errors encountered
    let mut db: *mut sqlite3 = unsafe { (*pParse).db }; // Database connection for malloc errors
    let mut rc: i32 = 0 as i32;
    let mut xAuth: Option<
        unsafe extern "C-unwind" fn(
            *mut (),
            i32,
            *const i8,
            *const i8,
            *const i8,
            *const i8,
        ) -> i32,
    > = unsafe { std::mem::zeroed() }; // Saved xAuth pointer
    0 as i32;
    if (((unsafe { (*pTable).eTabType }) as u32) as i32) == (1 as i32) {
        let __v2794: *mut sqlite3 = db;
        let __v2795: u32 = unsafe { (*__v2794).nSchemaLock };
        let __v2796: u32 = __v2795.wrapping_add((1 as i32) as u32);
        unsafe {
            (*__v2794).nSchemaLock = __v2796;
        }
        rc = unsafe { sqlite3VtabCallConnect(pParse, pTable) };
        let __v2797: *mut sqlite3 = db;
        let __v2798: u32 = unsafe { (*__v2797).nSchemaLock };
        let __v2799: u32 = __v2798.wrapping_sub((1 as i32) as u32);
        unsafe {
            (*__v2797).nSchemaLock = __v2799;
        }
        return rc;
    }
    // A positive nCol means the columns names for this view are
    // already known.  This routine is not called unless either the
    // table is virtual or nCol is zero.
    0 as i32;
    // A negative nCol is a special marker meaning that we are currently
    // trying to compute the column names.  If we enter this routine with
    // a negative nCol, it means two or more views form a loop, like this:
    //
    //     CREATE VIEW one AS SELECT * FROM two;
    //     CREATE VIEW two AS SELECT * FROM one;
    //
    // Actually, the error above is now caught prior to reaching this point.
    // But the following test is still important as it does come up
    // in the following:
    //
    //     CREATE TABLE main.ex1(a);
    //     CREATE TEMP VIEW ex1 AS SELECT a FROM ex1;
    //     SELECT * FROM temp.ex1;
    if ((unsafe { (*pTable).nCol }) as i32) < (0 as i32) {
        unsafe {
            sqlite3ErrorMsg(
                pParse,
                (b"view %s is circularly defined\0".as_ptr() as *mut i8) as *const i8,
                unsafe { (*pTable).zName },
            )
        };
        return 1 as i32;
    }
    0 as i32;
    // If we get this far, it means we need to compute the table names.
    // Note that the call to sqlite3ResultSetOfSelect() will expand any
    // "*" elements in the results set of the view and will assign cursors
    // to the elements of the FROM clause.  But we do not want these changes
    // to be permanent.  So the computation is done on a copy of the SELECT
    // statement that defines the view.
    0 as i32;
    pSel = unsafe {
        sqlite3SelectDup(
            db,
            (unsafe { (*pTable).u.view.pSelect }) as *const Select,
            0 as i32,
        )
    };
    if pSel != std::ptr::null_mut::<Select>() {
        let mut eParseMode: u8 = unsafe { (*pParse).eParseMode };
        let mut nTab: i32 = unsafe { (*pParse).nTab };
        let mut nSelect: i32 = unsafe { (*pParse).nSelect };
        unsafe {
            (*pParse).eParseMode = ((0 as i32) as i8) as u8;
        }
        sqlite3SrcListAssignCursors(pParse, unsafe { (*pSel).pSrc });
        unsafe {
            (*pTable).nCol = -(1 as i32) as i16;
        }
        let __v2800: *mut sqlite3 = db;
        let __v2801: u32 = unsafe { (*__v2800).lookaside.bDisable };
        let __v2802: u32 = __v2801.wrapping_add((1 as i32) as u32);
        unsafe {
            (*__v2800).lookaside.bDisable = __v2802;
        }
        unsafe {
            (*db).lookaside.sz = ((0 as i32) as i16) as u16;
        }
        xAuth = unsafe { (*db).xAuth };
        unsafe {
            (*db).xAuth = None;
        }
        pSelTab = unsafe { sqlite3ResultSetOfSelect(pParse, pSel, (64 as i32) as i8) };
        unsafe {
            (*db).xAuth = xAuth;
        }
        unsafe {
            (*pParse).nTab = nTab;
        }
        unsafe {
            (*pParse).nSelect = nSelect;
        }
        if pSelTab == std::ptr::null_mut::<Table>() {
            unsafe {
                (*pTable).nCol = (0 as i32) as i16;
            }
            let __v2803: i32 = nErr;
            let __v2804: i32 = __v2803 + (1 as i32);
            nErr = __v2804;
        } else {
            if (unsafe { (*pTable).pCheck }) != std::ptr::null_mut::<ExprList>() {
                // CREATE VIEW name(arglist) AS ...
                // The names of the columns in the table are taken from
                // arglist which is stored in pTable->pCheck.  The pCheck field
                // normally holds CHECK constraints on an ordinary table, but for
                // a VIEW it holds the list of column names.
                unsafe {
                    sqlite3ColumnsFromExprList(
                        pParse,
                        unsafe { (*pTable).pCheck },
                        unsafe { std::ptr::addr_of_mut!((*pTable).nCol) },
                        unsafe { std::ptr::addr_of_mut!((*pTable).aCol) },
                    )
                };
                if (unsafe { (*pParse).nErr }) == (0 as i32)
                    && ((unsafe { (*pTable).nCol }) as i32)
                        == unsafe { (*unsafe { (*pSel).pEList }).nExpr }
                {
                    0 as i32;
                    unsafe { sqlite3SubqueryColumnTypes(pParse, pTable, pSel, (64 as i32) as i8) };
                }
            } else {
                // CREATE VIEW name AS...  without an argument list.  Construct
                // the column names from the SELECT statement that defines the view.
                0 as i32;
                unsafe {
                    (*pTable).nCol = unsafe { (*pSelTab).nCol };
                }
                unsafe {
                    (*pTable).aCol = unsafe { (*pSelTab).aCol };
                }
                let __v2805: *mut Table = pTable;
                let __v2806: u32 = unsafe { (*__v2805).tabFlags };
                let __v2807: u32 =
                    __v2806 | (unsafe { (*pSelTab).tabFlags }) & ((98 as i32) as u32);
                unsafe {
                    (*__v2805).tabFlags = __v2807;
                }
                unsafe {
                    (*pSelTab).nCol = (0 as i32) as i16;
                }
                unsafe {
                    (*pSelTab).aCol = std::ptr::null_mut::<Column>();
                }
                0 as i32;
            }
        }
        unsafe {
            (*pTable).nNVCol = unsafe { (*pTable).nCol };
        }
        sqlite3DeleteTable(db, pSelTab);
        unsafe { sqlite3SelectDelete(db, pSel) };
        let __v2808: *mut sqlite3 = db;
        let __v2809: u32 = unsafe { (*__v2808).lookaside.bDisable };
        let __v2810: u32 = __v2809.wrapping_sub((1 as i32) as u32);
        unsafe {
            (*__v2808).lookaside.bDisable = __v2810;
        }
        unsafe {
            (*db).lookaside.sz = ((if (unsafe { (*db).lookaside.bDisable }) != (0 as u32) {
                0 as i32
            } else {
                ((unsafe { (*db).lookaside.szTrue }) as u32) as i32
            }) as i16) as u16;
        }
        unsafe {
            (*pParse).eParseMode = eParseMode;
        }
    } else {
        let __v2811: i32 = nErr;
        let __v2812: i32 = __v2811 + (1 as i32);
        nErr = __v2812;
    }
    let __v2813: *mut Schema = unsafe { (*pTable).pSchema };
    let __v2814: u16 = unsafe { (*__v2813).schemaFlags };
    let __v2815: u16 = ((((__v2814 as u32) as i32) | (2 as i32)) as i16) as u16;
    unsafe {
        (*__v2813).schemaFlags = __v2815;
    }
    if (unsafe { (*db).mallocFailed }) != (0 as u8) {
        sqlite3DeleteColumnNames(db, pTable);
    }
    return nErr + unsafe { (*pParse).nErr };
}

#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3ViewGetColumnNames(
    mut pParse: *mut Parse,
    mut pTable: *mut Table,
) -> i32 {
    0 as i32;
    if !((((unsafe { (*pTable).eTabType }) as u32) as i32) == (1 as i32))
        && ((unsafe { (*pTable).nCol }) as i32) > (0 as i32)
    {
        return 0 as i32;
    }
    return viewGetColumnNames(pParse, pTable);
}

/// Clear the column names from every VIEW in database idx.
fn sqliteViewResetAll(mut db: *mut sqlite3, mut idx: i32) {
    let mut i: *mut HashElem = unsafe { std::mem::zeroed() };
    0 as i32;
    if !((((unsafe {
        (*unsafe { (*unsafe { unsafe { (*db).aDb }.offset(idx as isize) }).pSchema }).schemaFlags
    }) as u32) as i32)
        & (2 as i32)
        == (2 as i32))
    {
        return;
    }
    i = unsafe {
        (*unsafe {
            std::ptr::addr_of_mut!(
                (*unsafe { (*unsafe { unsafe { (*db).aDb }.offset(idx as isize) }).pSchema })
                    .tblHash
            )
        })
        .first
    };
    '__slate_break_2152: while i != std::ptr::null_mut::<HashElem>() {
        let mut pTab: *mut Table = (unsafe { (*i).data }) as *mut Table;
        if (((unsafe { (*pTab).eTabType }) as u32) as i32) == (2 as i32) {
            sqlite3DeleteColumnNames(db, pTab);
        }
        i = unsafe { (*i).next };
    }
    let __v2816: *mut Schema =
        unsafe { (*unsafe { unsafe { (*db).aDb }.offset(idx as isize) }).pSchema };
    let __v2817: u16 = unsafe { (*__v2816).schemaFlags };
    let __v2818: u16 = ((((__v2817 as u32) as i32) & !(2 as i32)) as i16) as u16;
    unsafe {
        (*__v2816).schemaFlags = __v2818;
    }
}

/// This function is called by the VDBE to adjust the internal schema
/// used by SQLite when the btree layer moves a table root page. The
/// root-page of a table or index in database iDb has changed from iFrom
/// to iTo.
///
/// Ticket #1728:  The symbol table might still contain information
/// on tables and/or indices that are the process of being deleted.
/// If you are unlucky, one of those deleted indices or tables might
/// have the same rootpage number as the real table or index that is
/// being moved.  So we cannot stop searching after the first match
/// because the first match might be for one of the deleted indices
/// or tables and not the table/index that is actually being moved.
/// We must continue looping until all tables and indices with
/// rootpage==iFrom have been converted to have a rootpage of iTo
/// in order to be certain that we got the right one.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3RootPageMoved(
    mut db: *mut sqlite3,
    mut iDb: i32,
    mut iFrom: u32,
    mut iTo: u32,
) {
    let mut pElem: *mut HashElem = unsafe { std::mem::zeroed() };
    let mut pHash: *mut Hash = unsafe { std::mem::zeroed() };
    let mut pDb: *mut Db = unsafe { std::mem::zeroed() };
    0 as i32;
    pDb = unsafe { unsafe { (*db).aDb }.offset(iDb as isize) };
    pHash = unsafe { std::ptr::addr_of_mut!((*unsafe { (*pDb).pSchema }).tblHash) };
    pElem = unsafe { (*pHash).first };
    '__slate_break_2153: while pElem != std::ptr::null_mut::<HashElem>() {
        let mut pTab: *mut Table = (unsafe { (*pElem).data }) as *mut Table;
        if (unsafe { (*pTab).tnum }) == iFrom {
            unsafe {
                (*pTab).tnum = iTo;
            }
        }
        pElem = unsafe { (*pElem).next };
    }
    pHash = unsafe { std::ptr::addr_of_mut!((*unsafe { (*pDb).pSchema }).idxHash) };
    pElem = unsafe { (*pHash).first };
    '__slate_break_2154: while pElem != std::ptr::null_mut::<HashElem>() {
        let mut pIdx: *mut Index = (unsafe { (*pElem).data }) as *mut Index;
        if (unsafe { (*pIdx).tnum }) == iFrom {
            unsafe {
                (*pIdx).tnum = iTo;
            }
        }
        pElem = unsafe { (*pElem).next };
    }
}

/// Write code to erase the table with root-page iTable from database iDb.
/// Also write code to modify the sqlite_schema table and internal schema
/// if a root-page of another table is moved by the btree-layer whilst
/// erasing iTable (this can happen with an auto-vacuum database).
fn destroyRootPage(mut pParse: *mut Parse, mut iTable: i32, mut iDb: i32) {
    let mut v: *mut Vdbe = unsafe { sqlite3GetVdbe(pParse) };
    let mut r1: i32 = unsafe { sqlite3GetTempReg(pParse) };
    if iTable < (2 as i32) {
        unsafe {
            sqlite3ErrorMsg(
                pParse,
                (b"corrupt schema\0".as_ptr() as *mut i8) as *const i8,
            )
        };
    }
    unsafe { sqlite3VdbeAddOp3(v, 146 as i32, iTable, r1, iDb) };
    sqlite3MayAbort(pParse);
    // OP_Destroy stores an in integer r1. If this integer
    // is non-zero, then it is the root page number of a table moved to
    // location iTable. The following code modifies the sqlite_schema table to
    // reflect this.
    //
    // The "#NNN" in the SQL is a special constant that means whatever value
    // is in register NNN.  See grammar rules associated with the TK_REGISTER
    // token for additional information.
    unsafe {
        sqlite3NestedParse(
            pParse,
            (b"UPDATE %Q.sqlite_master SET rootpage=%d WHERE #%d AND rootpage=#%d\0".as_ptr()
                as *mut i8) as *const i8,
            unsafe {
                (*unsafe { unsafe { (*unsafe { (*pParse).db }).aDb }.offset(iDb as isize) })
                    .zDbSName
            },
            iTable,
            r1,
            r1,
        )
    };
    unsafe { sqlite3ReleaseTempReg(pParse, r1) };
}

/// Write VDBE code to erase table pTab and all associated indices on disk.
/// Code to update the sqlite_schema tables and internal schema definitions
/// in case a root-page belonging to another table is moved by the btree layer
/// is also added (this can happen with an auto-vacuum database).
fn destroyTable(mut pParse: *mut Parse, mut pTab: *mut Table) {
    // If the database may be auto-vacuum capable (if SQLITE_OMIT_AUTOVACUUM
    // is not defined), then it is important to call OP_Destroy on the
    // table and index root-pages in order, starting with the numerically
    // largest root-page number. This guarantees that none of the root-pages
    // to be destroyed is relocated by an earlier OP_Destroy. i.e. if the
    // following were coded:
    //
    // OP_Destroy 4 0
    // ...
    // OP_Destroy 5 0
    //
    // and root page 5 happened to be the largest root-page number in the
    // database, then root page 5 would be moved to page 4 by the
    // "OP_Destroy 4 0" opcode. The subsequent "OP_Destroy 5 0" would hit
    // a free-list page.
    let mut iTab: u32 = unsafe { (*pTab).tnum };
    let mut iDestroyed: u32 = (0 as i32) as u32;
    '__slate_break_2157: while (1 as i32) != (0 as i32) {
        let mut pIdx: *mut Index = unsafe { std::mem::zeroed() };
        let mut iLargest: u32 = (0 as i32) as u32;
        if iDestroyed == ((0 as i32) as u32) || iTab < iDestroyed {
            iLargest = iTab;
        }
        pIdx = unsafe { (*pTab).pIndex };
        '__slate_break_2158: while pIdx != std::ptr::null_mut::<Index>() {
            let mut iIdx: u32 = unsafe { (*pIdx).tnum };
            0 as i32;
            if (iDestroyed == ((0 as i32) as u32) || iIdx < iDestroyed) && iIdx > iLargest {
                iLargest = iIdx;
            }
            pIdx = unsafe { (*pIdx).pNext };
        }
        if iLargest == ((0 as i32) as u32) {
            return;
        } else {
            let mut iDb: i32 = unsafe {
                sqlite3SchemaToIndex(unsafe { (*pParse).db }, unsafe { (*pTab).pSchema })
            };
            0 as i32;
            destroyRootPage(pParse, iLargest as i32, iDb);
            iDestroyed = iLargest;
        }
    }
}

/// Remove entries from the sqlite_statN tables (for N in (1,2,3))
/// after a DROP INDEX or DROP TABLE command.
///
/// # Arguments
///
/// * `pParse` - The parsing context
/// * `iDb` - The database number
/// * `zType` - "idx" or "tbl"
/// * `zName` - Name of index or table
fn sqlite3ClearStatTables(
    mut pParse: *mut Parse,
    mut iDb: i32,
    mut zType: *const i8,
    mut zName: *const i8,
) {
    let mut i: i32 = 0 as i32;
    let mut zDbName: *const i8 = (unsafe {
        (*unsafe { unsafe { (*unsafe { (*pParse).db }).aDb }.offset(iDb as isize) }).zDbSName
    }) as *const i8;
    i = 1 as i32;
    '__slate_break_2159: loop {
        if !(i <= (4 as i32)) {
            break;
        }
        let mut zTab: __SlateAlign16<[i8; 24]> = __SlateAlign16([0 as i8; 24]);
        unsafe {
            sqlite3_snprintf(
                ((24 as u64) as u32) as i32,
                zTab.0.as_mut_ptr() as *mut i8,
                (b"sqlite_stat%d\0".as_ptr() as *mut i8) as *const i8,
                i,
            )
        };
        if sqlite3FindTable(
            unsafe { (*pParse).db },
            (zTab.0.as_mut_ptr() as *mut i8) as *const i8,
            zDbName,
        ) != std::ptr::null_mut::<Table>()
        {
            unsafe {
                sqlite3NestedParse(
                    pParse,
                    (b"DELETE FROM %Q.%s WHERE %s=%Q\0".as_ptr() as *mut i8) as *const i8,
                    zDbName,
                    zTab.0.as_mut_ptr() as *mut i8,
                    zType,
                    zName,
                )
            };
        }
        let __v2819: i32 = i;
        let __v2820: i32 = __v2819 + (1 as i32);
        i = __v2820;
    }
}

/// Generate code to drop a table.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3CodeDropTable(
    mut pParse: *mut Parse,
    mut pTab: *mut Table,
    mut iDb: i32,
    mut isView: i32,
) {
    let mut v: *mut Vdbe = unsafe { std::mem::zeroed() };
    let mut db: *mut sqlite3 = unsafe { (*pParse).db };
    let mut pTrigger: *mut Trigger = unsafe { std::mem::zeroed() };
    let mut pDb: *mut Db = unsafe { unsafe { (*db).aDb }.offset(iDb as isize) };
    v = unsafe { sqlite3GetVdbe(pParse) };
    0 as i32;
    sqlite3BeginWriteOperation(pParse, 1 as i32, iDb);
    if (((unsafe { (*pTab).eTabType }) as u32) as i32) == (1 as i32) {
        unsafe { sqlite3VdbeAddOp0(v, 172 as i32) };
    }
    // Drop all triggers associated with the table being dropped. Code
    // is generated to remove entries from sqlite_schema and/or
    // sqlite_temp_schema if required.
    pTrigger = unsafe { sqlite3TriggerList(pParse, pTab) };
    '__slate_break_2162: while pTrigger != std::ptr::null_mut::<Trigger>() {
        0 as i32;
        unsafe { sqlite3DropTriggerPtr(pParse, pTrigger) };
        pTrigger = unsafe { (*pTrigger).pNext };
    }
    // Remove any entries of the sqlite_sequence table associated with
    // the table being dropped. This is done before the table is dropped
    // at the btree level, in case the sqlite_sequence table needs to
    // move as a result of the drop (can happen in auto-vacuum mode).
    if (unsafe { (*pTab).tabFlags }) & ((8 as i32) as u32) != (0 as u32) {
        unsafe {
            sqlite3NestedParse(
                pParse,
                (b"DELETE FROM %Q.sqlite_sequence WHERE name=%Q\0".as_ptr() as *mut i8)
                    as *const i8,
                unsafe { (*pDb).zDbSName },
                unsafe { (*pTab).zName },
            )
        };
    }
    // Drop all entries in the schema table that refer to the
    // table. The program name loops through the schema table and deletes
    // every row that refers to a table of the same name as the one being
    // dropped. Triggers are handled separately because a trigger can be
    // created in the temp database that refers to a table in another
    // database.
    unsafe {
        sqlite3NestedParse(
            pParse,
            (b"DELETE FROM %Q.sqlite_master WHERE tbl_name=%Q and type!='trigger'\0".as_ptr()
                as *mut i8) as *const i8,
            unsafe { (*pDb).zDbSName },
            unsafe { (*pTab).zName },
        )
    };
    if !(isView != (0 as i32)) && !((((unsafe { (*pTab).eTabType }) as u32) as i32) == (1 as i32)) {
        destroyTable(pParse, pTab);
    }
    // Remove the table entry from SQLite's internal schema and modify
    // the schema cookie.
    if (((unsafe { (*pTab).eTabType }) as u32) as i32) == (1 as i32) {
        unsafe {
            sqlite3VdbeAddOp4(
                v,
                174 as i32,
                iDb,
                0 as i32,
                0 as i32,
                (unsafe { (*pTab).zName }) as *const i8,
                0 as i32,
            )
        };
        sqlite3MayAbort(pParse);
    }
    unsafe {
        sqlite3VdbeAddOp4(
            v,
            153 as i32,
            iDb,
            0 as i32,
            0 as i32,
            (unsafe { (*pTab).zName }) as *const i8,
            0 as i32,
        )
    };
    sqlite3ChangeCookie(pParse, iDb);
    sqliteViewResetAll(db, iDb);
}

/// Return TRUE if shadow tables should be read-only in the current
/// context.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3ReadOnlyShadowTables(mut db: *mut sqlite3) -> i32 {
    if (unsafe { (*db).flags }) & (((268435456 as i32) as i64) as u64)
        != (((0 as i32) as i64) as u64)
        && (unsafe { (*db).pVtabCtx }) == std::ptr::null_mut::<VtabCtx>()
        && (unsafe { (*db).nVdbeExec }) == (0 as i32)
        && !((unsafe { (*db).nVTrans }) > (0 as i32)
            && (unsafe { (*db).aVTrans }) == std::ptr::null_mut::<*mut VTable>())
    {
        return 1 as i32;
    }
    return 0 as i32;
}

/// Return true if it is not allowed to drop the given table
fn tableMayNotBeDropped(mut db: *mut sqlite3, mut pTab: *mut Table) -> i32 {
    if (unsafe {
        sqlite3_strnicmp(
            (unsafe { (*pTab).zName }) as *const i8,
            (b"sqlite_\0".as_ptr() as *mut i8) as *const i8,
            7 as i32,
        )
    }) == (0 as i32)
    {
        if (unsafe {
            sqlite3_strnicmp(
                (unsafe { unsafe { (*pTab).zName }.offset((7 as i32) as isize) }) as *const i8,
                (b"stat\0".as_ptr() as *mut i8) as *const i8,
                4 as i32,
            )
        }) == (0 as i32)
        {
            return 0 as i32;
        }
        if (unsafe {
            sqlite3_strnicmp(
                (unsafe { unsafe { (*pTab).zName }.offset((7 as i32) as isize) }) as *const i8,
                (b"parameters\0".as_ptr() as *mut i8) as *const i8,
                10 as i32,
            )
        }) == (0 as i32)
        {
            return 0 as i32;
        }
        return 1 as i32;
    }
    let __v2821: bool;
    if (unsafe { (*pTab).tabFlags }) & ((4096 as i32) as u32) != ((0 as i32) as u32) {
        __v2821 = sqlite3ReadOnlyShadowTables(db) != (0 as i32);
    } else {
        __v2821 = false as bool;
    }
    if __v2821 {
        return 1 as i32;
    }
    if (unsafe { (*pTab).tabFlags }) & ((32768 as i32) as u32) != (0 as u32) {
        return 1 as i32;
    }
    return 0 as i32;
}

/// This routine is called to do the work of a DROP TABLE statement.
/// pName is the name of the table to be dropped.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3DropTable(
    mut pParse: *mut Parse,
    mut pName: *mut SrcList,
    mut isView: i32,
    mut noErr: i32,
) {
    let mut __slate_storage_1098: std::mem::MaybeUninit<*const i8> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1098: *mut *const i8 =
        std::ptr::addr_of_mut!(__slate_storage_1098) as *mut *const i8;
    let mut __slate_storage_1097: std::mem::MaybeUninit<*const i8> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1097: *mut *const i8 =
        std::ptr::addr_of_mut!(__slate_storage_1097) as *mut *const i8;
    let mut __slate_storage_1096: std::mem::MaybeUninit<*const i8> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1096: *mut *const i8 =
        std::ptr::addr_of_mut!(__slate_storage_1096) as *mut *const i8;
    let mut __slate_storage_1095: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1095: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1095) as *mut i32;
    // If pTab is a virtual table, call ViewGetColumnNames() to ensure
    // it is initialized.
    let mut __slate_storage_2455: std::mem::MaybeUninit<bool> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2455: *mut bool = std::ptr::addr_of_mut!(__slate_storage_2455) as *mut bool;
    let mut __slate_storage_2454: std::mem::MaybeUninit<u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2454: *mut u8 = std::ptr::addr_of_mut!(__slate_storage_2454) as *mut u8;
    let mut __slate_storage_2453: std::mem::MaybeUninit<u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2453: *mut u8 = std::ptr::addr_of_mut!(__slate_storage_2453) as *mut u8;
    let mut __slate_storage_2452: std::mem::MaybeUninit<*mut sqlite3> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_2452: *mut *mut sqlite3 =
        std::ptr::addr_of_mut!(__slate_storage_2452) as *mut *mut sqlite3;
    let mut __slate_storage_2451: std::mem::MaybeUninit<u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2451: *mut u8 = std::ptr::addr_of_mut!(__slate_storage_2451) as *mut u8;
    let mut __slate_storage_2450: std::mem::MaybeUninit<u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2450: *mut u8 = std::ptr::addr_of_mut!(__slate_storage_2450) as *mut u8;
    let mut __slate_storage_2449: std::mem::MaybeUninit<*mut sqlite3> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_2449: *mut *mut sqlite3 =
        std::ptr::addr_of_mut!(__slate_storage_2449) as *mut *mut sqlite3;
    let mut __slate_storage_1094: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1094: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1094) as *mut i32;
    let mut __slate_storage_1093: std::mem::MaybeUninit<*mut sqlite3> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1093: *mut *mut sqlite3 =
        std::ptr::addr_of_mut!(__slate_storage_1093) as *mut *mut sqlite3;
    let mut __slate_storage_1092: std::mem::MaybeUninit<*mut Vdbe> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1092: *mut *mut Vdbe =
        std::ptr::addr_of_mut!(__slate_storage_1092) as *mut *mut Vdbe;
    let mut __slate_storage_1091: std::mem::MaybeUninit<*mut Table> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1091: *mut *mut Table =
        std::ptr::addr_of_mut!(__slate_storage_1091) as *mut *mut Table;
    unsafe {
        '__join_0: {
            std::ptr::write(__slate_slot_1093, unsafe { (*pParse).db });
            if (unsafe { (*(*__slate_slot_1093)).mallocFailed }) != (0 as u8) {
            } else {
                0 as i32;
                0 as i32;
                0 as i32;
                0 as i32;
                if (unsafe { sqlite3ReadSchema(pParse) }) != (0 as i32) {
                } else {
                    if noErr != (0 as i32) {
                        std::ptr::write(__slate_slot_2449, *__slate_slot_1093);
                        std::ptr::write(__slate_slot_2450, unsafe {
                            (*(*__slate_slot_2449)).suppressErr
                        });
                        std::ptr::write(
                            __slate_slot_2451,
                            ((((*__slate_slot_2450 as u32) as i32) + (1 as i32)) as i8) as u8,
                        );
                        unsafe {
                            (*(*__slate_slot_2449)).suppressErr = *__slate_slot_2451;
                        }
                    }
                    0 as i32;
                    *__slate_slot_1091 = sqlite3LocateTableItem(pParse, isView as u32, unsafe {
                        unsafe { std::ptr::addr_of_mut!((*pName).a) as *mut SrcItem }
                            .offset((0 as i32) as isize)
                    });
                    if noErr != (0 as i32) {
                        std::ptr::write(__slate_slot_2452, *__slate_slot_1093);
                        std::ptr::write(__slate_slot_2453, unsafe {
                            (*(*__slate_slot_2452)).suppressErr
                        });
                        std::ptr::write(
                            __slate_slot_2454,
                            ((((*__slate_slot_2453 as u32) as i32) - (1 as i32)) as i8) as u8,
                        );
                        unsafe {
                            (*(*__slate_slot_2452)).suppressErr = *__slate_slot_2454;
                        }
                    }
                    if *__slate_slot_1091 == std::ptr::null_mut::<Table>() {
                        if noErr != (0 as i32) {
                            sqlite3CodeVerifyNamedSchema(
                                pParse,
                                (unsafe {
                                    (*unsafe {
                                        unsafe {
                                            std::ptr::addr_of_mut!((*pName).a) as *mut SrcItem
                                        }
                                        .offset((0 as i32) as isize)
                                    })
                                    .u4
                                    .zDatabase
                                }) as *const i8,
                            );
                            sqlite3ForceNotReadOnly(pParse);
                        }
                    } else {
                        *__slate_slot_1094 = unsafe {
                            sqlite3SchemaToIndex(*__slate_slot_1093, unsafe {
                                (*(*__slate_slot_1091)).pSchema
                            })
                        };
                        0 as i32;
                        if (((unsafe { (*(*__slate_slot_1091)).eTabType }) as u32) as i32)
                            == (1 as i32)
                        {
                            *__slate_slot_2455 =
                                sqlite3ViewGetColumnNames(pParse, *__slate_slot_1091) != (0 as i32);
                        } else {
                            *__slate_slot_2455 = false as bool;
                        }
                        if *__slate_slot_2455 {
                        } else {
                            std::ptr::write(
                                __slate_slot_1096,
                                (if !((0 as i32) != (0 as i32)) && *__slate_slot_1094 == (1 as i32)
                                {
                                    b"sqlite_temp_master\0".as_ptr() as *mut i8
                                } else {
                                    b"sqlite_master\0".as_ptr() as *mut i8
                                }) as *const i8,
                            );
                            std::ptr::write(
                                __slate_slot_1097,
                                (unsafe {
                                    (*unsafe {
                                        unsafe { (*(*__slate_slot_1093)).aDb }
                                            .offset(*__slate_slot_1094 as isize)
                                    })
                                    .zDbSName
                                }) as *const i8,
                            );
                            std::ptr::write(__slate_slot_1098, std::ptr::null::<i8>());
                            if (unsafe {
                                sqlite3AuthCheck(
                                    pParse,
                                    9 as i32,
                                    *__slate_slot_1096,
                                    std::ptr::null::<i8>(),
                                    *__slate_slot_1097,
                                )
                            }) != (0 as i32)
                            {
                            } else {
                                if isView != (0 as i32) {
                                    if !((0 as i32) != (0 as i32))
                                        && *__slate_slot_1094 == (1 as i32)
                                    {
                                        *__slate_slot_1095 = 15 as i32;
                                    } else {
                                        *__slate_slot_1095 = 17 as i32;
                                    }
                                } else {
                                    if (((unsafe { (*(*__slate_slot_1091)).eTabType }) as u32)
                                        as i32)
                                        == (1 as i32)
                                    {
                                        *__slate_slot_1095 = 30 as i32;
                                        *__slate_slot_1098 = unsafe {
                                            (*unsafe {
                                                (*unsafe {
                                                    sqlite3GetVTable(
                                                        *__slate_slot_1093,
                                                        *__slate_slot_1091,
                                                    )
                                                })
                                                .pMod
                                            })
                                            .zName
                                        };
                                    } else {
                                        if !((0 as i32) != (0 as i32))
                                            && *__slate_slot_1094 == (1 as i32)
                                        {
                                            *__slate_slot_1095 = 13 as i32;
                                        } else {
                                            *__slate_slot_1095 = 11 as i32;
                                        }
                                    }
                                }
                                if (unsafe {
                                    sqlite3AuthCheck(
                                        pParse,
                                        *__slate_slot_1095,
                                        (unsafe { (*(*__slate_slot_1091)).zName }) as *const i8,
                                        *__slate_slot_1098,
                                        *__slate_slot_1097,
                                    )
                                }) != (0 as i32)
                                {
                                } else {
                                    if (unsafe {
                                        sqlite3AuthCheck(
                                            pParse,
                                            9 as i32,
                                            (unsafe { (*(*__slate_slot_1091)).zName }) as *const i8,
                                            std::ptr::null::<i8>(),
                                            *__slate_slot_1097,
                                        )
                                    }) != (0 as i32)
                                    {
                                    } else {
                                        if tableMayNotBeDropped(
                                            *__slate_slot_1093,
                                            *__slate_slot_1091,
                                        ) != (0 as i32)
                                        {
                                            unsafe {
                                                sqlite3ErrorMsg(
                                                    pParse,
                                                    (b"table %s may not be dropped\0".as_ptr()
                                                        as *mut i8)
                                                        as *const i8,
                                                    unsafe { (*(*__slate_slot_1091)).zName },
                                                )
                                            };
                                        } else {
                                            // Ensure DROP TABLE is not used on a view, and DROP VIEW is not used
                                            // on a table.
                                            if isView != (0 as i32)
                                                && !((((unsafe { (*(*__slate_slot_1091)).eTabType })
                                                    as u32)
                                                    as i32)
                                                    == (2 as i32))
                                            {
                                                unsafe {
                                                    sqlite3ErrorMsg(
                                                        pParse,
                                                        (b"use DROP TABLE to delete table %s\0"
                                                            .as_ptr()
                                                            as *mut i8)
                                                            as *const i8,
                                                        unsafe { (*(*__slate_slot_1091)).zName },
                                                    )
                                                };
                                            } else {
                                                if !(isView != (0 as i32))
                                                    && (((unsafe {
                                                        (*(*__slate_slot_1091)).eTabType
                                                    })
                                                        as u32)
                                                        as i32)
                                                        == (2 as i32)
                                                {
                                                    unsafe {
                                                        sqlite3ErrorMsg(
                                                            pParse,
                                                            (b"use DROP VIEW to delete view %s\0"
                                                                .as_ptr()
                                                                as *mut i8)
                                                                as *const i8,
                                                            unsafe {
                                                                (*(*__slate_slot_1091)).zName
                                                            },
                                                        )
                                                    };
                                                } else {
                                                    // Generate code to remove the table from the schema table
                                                    // on disk.
                                                    *__slate_slot_1092 =
                                                        unsafe { sqlite3GetVdbe(pParse) };
                                                    if *__slate_slot_1092
                                                        != std::ptr::null_mut::<Vdbe>()
                                                    {
                                                        sqlite3BeginWriteOperation(
                                                            pParse,
                                                            1 as i32,
                                                            *__slate_slot_1094,
                                                        );
                                                        if !(isView != (0 as i32)) {
                                                            sqlite3ClearStatTables(
                                                                pParse,
                                                                *__slate_slot_1094,
                                                                (b"tbl\0".as_ptr() as *mut i8)
                                                                    as *const i8,
                                                                (unsafe {
                                                                    (*(*__slate_slot_1091)).zName
                                                                })
                                                                    as *const i8,
                                                            );
                                                            unsafe {
                                                                sqlite3FkDropTable(
                                                                    pParse,
                                                                    pName,
                                                                    *__slate_slot_1091,
                                                                )
                                                            };
                                                        }
                                                        sqlite3CodeDropTable(
                                                            pParse,
                                                            *__slate_slot_1091,
                                                            *__slate_slot_1094,
                                                            isView,
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
        }
        sqlite3SrcListDelete(*__slate_slot_1093, pName);
    }
}

/// This routine is called to create a new foreign key on the table
/// currently under construction.  pFromCol determines which columns
/// in the current table point to the foreign key.  If pFromCol==0 then
/// connect the key to the last column inserted.  pTo is the name of
/// the table referred to (a.k.a the "parent" table).  pToCol is a list
/// of tables in the parent pTo table.  flags contains all
/// information about the conflict resolution algorithms specified
/// in the ON DELETE, ON UPDATE and ON INSERT clauses.
///
/// An FKey structure is created and added to the table currently
/// under construction in the pParse->pNewTable field.
///
/// The foreign key is set for IMMEDIATE processing.  A subsequent call
/// to sqlite3DeferForeignKey() might change this to DEFERRED.
///
/// # Arguments
///
/// * `pParse` - Parsing context
/// * `pFromCol` - Columns in this table that point to other table
/// * `pTo` - Name of the other table
/// * `pToCol` - Columns in the other table
/// * `flags_1104` - Conflict resolution algorithms.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3CreateForeignKey(
    mut pParse: *mut Parse,
    mut pFromCol: *mut ExprList,
    mut pTo: *mut Token,
    mut pToCol: *mut ExprList,
    mut flags_1104: i32,
) {
    let mut __slate_storage_2608: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2608: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_2608) as *mut i32;
    let mut __slate_storage_2607: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2607: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_2607) as *mut i32;
    let mut __slate_storage_2610: std::mem::MaybeUninit<*mut i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2610: *mut *mut i8 =
        std::ptr::addr_of_mut!(__slate_storage_2610) as *mut *mut i8;
    let mut __slate_storage_2609: std::mem::MaybeUninit<*mut i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2609: *mut *mut i8 =
        std::ptr::addr_of_mut!(__slate_storage_2609) as *mut *mut i8;
    let mut __slate_storage_1115: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1115: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1115) as *mut i32;
    let mut __slate_storage_2604: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2604: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_2604) as *mut i32;
    let mut __slate_storage_2603: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2603: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_2603) as *mut i32;
    let mut __slate_storage_2606: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2606: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_2606) as *mut i32;
    let mut __slate_storage_2605: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2605: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_2605) as *mut i32;
    let mut __slate_storage_1114: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1114: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1114) as *mut i32;
    let mut __slate_storage_2602: std::mem::MaybeUninit<*mut i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2602: *mut *mut i8 =
        std::ptr::addr_of_mut!(__slate_storage_2602) as *mut *mut i8;
    let mut __slate_storage_2601: std::mem::MaybeUninit<*mut i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2601: *mut *mut i8 =
        std::ptr::addr_of_mut!(__slate_storage_2601) as *mut *mut i8;
    let mut __slate_storage_2598: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2598: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_2598) as *mut i32;
    let mut __slate_storage_2597: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2597: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_2597) as *mut i32;
    let mut __slate_storage_2600: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2600: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_2600) as *mut i64;
    let mut __slate_storage_2599: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2599: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_2599) as *mut i64;
    let mut __slate_storage_1113: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1113: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1113) as *mut i32;
    let mut __slate_storage_1112: std::mem::MaybeUninit<*mut i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1112: *mut *mut i8 =
        std::ptr::addr_of_mut!(__slate_storage_1112) as *mut *mut i8;
    let mut __slate_storage_1111: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1111: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1111) as *mut i32;
    let mut __slate_storage_1110: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1110: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1110) as *mut i32;
    let mut __slate_storage_1109: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1109: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_1109) as *mut i64;
    let mut __slate_storage_1108: std::mem::MaybeUninit<*mut Table> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1108: *mut *mut Table =
        std::ptr::addr_of_mut!(__slate_storage_1108) as *mut *mut Table;
    let mut __slate_storage_1107: std::mem::MaybeUninit<*mut FKey> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1107: *mut *mut FKey =
        std::ptr::addr_of_mut!(__slate_storage_1107) as *mut *mut FKey;
    let mut __slate_storage_1106: std::mem::MaybeUninit<*mut FKey> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1106: *mut *mut FKey =
        std::ptr::addr_of_mut!(__slate_storage_1106) as *mut *mut FKey;
    let mut __slate_storage_1105: std::mem::MaybeUninit<*mut sqlite3> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1105: *mut *mut sqlite3 =
        std::ptr::addr_of_mut!(__slate_storage_1105) as *mut *mut sqlite3;
    unsafe {
        '__join_0: {
            std::ptr::write(__slate_slot_1105, unsafe { (*pParse).db });
            std::ptr::write(__slate_slot_1106, std::ptr::null_mut::<FKey>());
            std::ptr::write(__slate_slot_1108, unsafe { (*pParse).pNewTable });
            0 as i32;
            if *__slate_slot_1108 == std::ptr::null_mut::<Table>()
                || (((unsafe { (*pParse).eParseMode }) as u32) as i32) == (1 as i32)
            {
            } else {
                if pFromCol == std::ptr::null_mut::<ExprList>() {
                    std::ptr::write(
                        __slate_slot_1113,
                        ((unsafe { (*(*__slate_slot_1108)).nCol }) as i32) - (1 as i32),
                    );
                    if *__slate_slot_1113 < (0 as i32) {
                        break '__join_0;
                    } else {
                        if pToCol != std::ptr::null_mut::<ExprList>()
                            && (unsafe { (*pToCol).nExpr }) != (1 as i32)
                        {
                            unsafe {
                                sqlite3ErrorMsg(pParse, (b"foreign key on %s should reference only one column of table %T\0".as_ptr() as *mut i8) as *const i8, unsafe { (*unsafe { unsafe { (*(*__slate_slot_1108)).aCol }.offset(*__slate_slot_1113 as isize) }).zCnName }, pTo)
                            };
                            break '__join_0;
                        } else {
                            *__slate_slot_1111 = 1 as i32;
                        }
                    }
                } else {
                    if pToCol != std::ptr::null_mut::<ExprList>()
                        && (unsafe { (*pToCol).nExpr }) != unsafe { (*pFromCol).nExpr }
                    {
                        unsafe {
                            sqlite3ErrorMsg(pParse, (b"number of columns in foreign key does not match the number of columns in the referenced table\0".as_ptr() as *mut i8) as *const i8)
                        };
                        break '__join_0;
                    } else {
                        *__slate_slot_1111 = unsafe { (*pFromCol).nExpr };
                    }
                }
                '__join_28: {
                    *__slate_slot_1109 = (64 as u64)
                        .wrapping_add(((*__slate_slot_1111 as i64) as u64).wrapping_mul(16 as u64))
                        .wrapping_add((unsafe { (*pTo).n }) as u64)
                        .wrapping_add(((1 as i32) as i64) as u64)
                        as i64;
                    if pToCol != std::ptr::null_mut::<ExprList>() {
                        *__slate_slot_1110 = 0 as i32;
                        loop {
                            if *__slate_slot_1110 < unsafe { (*pToCol).nExpr } {
                                std::ptr::write(__slate_slot_2599, *__slate_slot_1109);
                                std::ptr::write(
                                    __slate_slot_2600,
                                    *__slate_slot_2599
                                        + (((unsafe {
                                            sqlite3Strlen30(
                                                (unsafe {
                                                    (*unsafe {
                                                        unsafe {
                                                            std::ptr::addr_of_mut!((*pToCol).a)
                                                                as *mut ExprList_item
                                                        }
                                                        .offset(*__slate_slot_1110 as isize)
                                                    })
                                                    .zEName
                                                })
                                                    as *const i8,
                                            )
                                        }) + (1 as i32))
                                            as i64),
                                );
                                *__slate_slot_1109 = *__slate_slot_2600;
                                std::ptr::write(__slate_slot_2597, *__slate_slot_1110);
                                std::ptr::write(__slate_slot_2598, *__slate_slot_2597 + (1 as i32));
                                *__slate_slot_1110 = *__slate_slot_2598;
                            } else {
                                break '__join_28;
                            }
                        }
                    }
                }
                *__slate_slot_1106 =
                    (unsafe { sqlite3DbMallocZero(*__slate_slot_1105, *__slate_slot_1109 as u64) })
                        as *mut FKey;
                if *__slate_slot_1106 == std::ptr::null_mut::<FKey>() {
                } else {
                    unsafe {
                        (*(*__slate_slot_1106)).pFrom = *__slate_slot_1108;
                    }
                    0 as i32;
                    unsafe {
                        (*(*__slate_slot_1106)).pNextFrom =
                            unsafe { (*(*__slate_slot_1108)).u.tab.pFKey };
                    }
                    *__slate_slot_1112 = (unsafe {
                        unsafe {
                            std::ptr::addr_of_mut!((*(*__slate_slot_1106)).aCol) as *mut sColMap
                        }
                        .offset(*__slate_slot_1111 as isize)
                    }) as *mut i8;
                    unsafe {
                        (*(*__slate_slot_1106)).zTo = *__slate_slot_1112;
                    }
                    if (((unsafe { (*pParse).eParseMode }) as u32) as i32) >= (2 as i32) {
                        unsafe {
                            sqlite3RenameTokenMap(
                                pParse,
                                (*__slate_slot_1112 as *mut ()) as *const (),
                                pTo as *const Token,
                            )
                        };
                    }
                    '__join_11: {
                        unsafe {
                            memcpy(
                                *__slate_slot_1112 as *mut (),
                                (unsafe { (*pTo).z }) as *const (),
                                (unsafe { (*pTo).n }) as u64,
                            )
                        };
                        unsafe {
                            *unsafe {
                                (*__slate_slot_1112).offset((unsafe { (*pTo).n }) as isize)
                            } = (0 as i32) as i8;
                        }
                        unsafe { sqlite3Dequote(*__slate_slot_1112) };
                        std::ptr::write(__slate_slot_2601, *__slate_slot_1112);
                        std::ptr::write(__slate_slot_2602, unsafe {
                            (*__slate_slot_2601).offset(unsafe { (*pTo).n }.wrapping_add((1 as i32) as u32) as isize)
                        });
                        *__slate_slot_1112 = *__slate_slot_2602;
                        unsafe {
                            (*(*__slate_slot_1106)).nCol = *__slate_slot_1111;
                        }
                        if pFromCol == std::ptr::null_mut::<ExprList>() {
                            unsafe {
                                (*unsafe {
                                    unsafe {
                                        std::ptr::addr_of_mut!((*(*__slate_slot_1106)).aCol)
                                            as *mut sColMap
                                    }
                                    .offset((0 as i32) as isize)
                                })
                                .iFrom =
                                    ((unsafe { (*(*__slate_slot_1108)).nCol }) as i32) - (1 as i32);
                            }
                        } else {
                            *__slate_slot_1110 = 0 as i32;
                            loop {
                                if *__slate_slot_1110 < *__slate_slot_1111 {
                                    *__slate_slot_1114 = 0 as i32;
                                    '__join_18: {
                                        loop {
                                            if *__slate_slot_1114
                                                < ((unsafe { (*(*__slate_slot_1108)).nCol }) as i32)
                                            {
                                                if (unsafe {
                                                    sqlite3StrICmp(
                                                        (unsafe {
                                                            (*unsafe {
                                                                unsafe {
                                                                    (*(*__slate_slot_1108)).aCol
                                                                }
                                                                .offset(*__slate_slot_1114 as isize)
                                                            })
                                                            .zCnName
                                                        })
                                                            as *const i8,
                                                        (unsafe {
                                                            (*unsafe {
                                                                unsafe {
                                                                    std::ptr::addr_of_mut!(
                                                                        (*pFromCol).a
                                                                    )
                                                                        as *mut ExprList_item
                                                                }
                                                                .offset(*__slate_slot_1110 as isize)
                                                            })
                                                            .zEName
                                                        })
                                                            as *const i8,
                                                    )
                                                }) == (0 as i32)
                                                {
                                                    break;
                                                } else {
                                                    std::ptr::write(
                                                        __slate_slot_2605,
                                                        *__slate_slot_1114,
                                                    );
                                                    std::ptr::write(
                                                        __slate_slot_2606,
                                                        *__slate_slot_2605 + (1 as i32),
                                                    );
                                                    *__slate_slot_1114 = *__slate_slot_2606;
                                                }
                                            } else {
                                                break '__join_18;
                                            }
                                        }
                                        unsafe {
                                            (*unsafe {
                                                unsafe {
                                                    std::ptr::addr_of_mut!(
                                                        (*(*__slate_slot_1106)).aCol
                                                    )
                                                        as *mut sColMap
                                                }
                                                .offset(*__slate_slot_1110 as isize)
                                            })
                                            .iFrom = *__slate_slot_1114;
                                        }
                                    }
                                    if *__slate_slot_1114
                                        >= ((unsafe { (*(*__slate_slot_1108)).nCol }) as i32)
                                    {
                                        break;
                                    } else {
                                        if (((unsafe { (*pParse).eParseMode }) as u32) as i32)
                                            >= (2 as i32)
                                        {
                                            unsafe {
                                                sqlite3RenameTokenRemap(
                                                    pParse,
                                                    (unsafe {
                                                        unsafe {
                                                            std::ptr::addr_of_mut!(
                                                                (*(*__slate_slot_1106)).aCol
                                                            )
                                                                as *mut sColMap
                                                        }
                                                        .offset(*__slate_slot_1110 as isize)
                                                    })
                                                        as *const (),
                                                    (unsafe {
                                                        (*unsafe {
                                                            unsafe {
                                                                std::ptr::addr_of_mut!(
                                                                    (*pFromCol).a
                                                                )
                                                                    as *mut ExprList_item
                                                            }
                                                            .offset(*__slate_slot_1110 as isize)
                                                        })
                                                        .zEName
                                                    })
                                                        as *const (),
                                                )
                                            };
                                        }
                                        std::ptr::write(__slate_slot_2603, *__slate_slot_1110);
                                        std::ptr::write(
                                            __slate_slot_2604,
                                            *__slate_slot_2603 + (1 as i32),
                                        );
                                        *__slate_slot_1110 = *__slate_slot_2604;
                                    }
                                } else {
                                    break '__join_11;
                                }
                            }
                            unsafe {
                                sqlite3ErrorMsg(
                                    pParse,
                                    (b"unknown column \"%s\" in foreign key definition\0".as_ptr()
                                        as *mut i8)
                                        as *const i8,
                                    unsafe {
                                        (*unsafe {
                                            unsafe {
                                                std::ptr::addr_of_mut!((*pFromCol).a)
                                                    as *mut ExprList_item
                                            }
                                            .offset(*__slate_slot_1110 as isize)
                                        })
                                        .zEName
                                    },
                                )
                            };
                            break '__join_0;
                        }
                    }
                    '__join_5: {
                        if pToCol != std::ptr::null_mut::<ExprList>() {
                            *__slate_slot_1110 = 0 as i32;
                            loop {
                                if *__slate_slot_1110 < *__slate_slot_1111 {
                                    std::ptr::write(__slate_slot_1115, unsafe {
                                        sqlite3Strlen30(
                                            (unsafe {
                                                (*unsafe {
                                                    unsafe {
                                                        std::ptr::addr_of_mut!((*pToCol).a)
                                                            as *mut ExprList_item
                                                    }
                                                    .offset(*__slate_slot_1110 as isize)
                                                })
                                                .zEName
                                            })
                                                as *const i8,
                                        )
                                    });
                                    unsafe {
                                        (*unsafe {
                                            unsafe {
                                                std::ptr::addr_of_mut!((*(*__slate_slot_1106)).aCol)
                                                    as *mut sColMap
                                            }
                                            .offset(*__slate_slot_1110 as isize)
                                        })
                                        .zCol = *__slate_slot_1112;
                                    }
                                    if (((unsafe { (*pParse).eParseMode }) as u32) as i32)
                                        >= (2 as i32)
                                    {
                                        unsafe {
                                            sqlite3RenameTokenRemap(
                                                pParse,
                                                *__slate_slot_1112 as *const (),
                                                (unsafe {
                                                    (*unsafe {
                                                        unsafe {
                                                            std::ptr::addr_of_mut!((*pToCol).a)
                                                                as *mut ExprList_item
                                                        }
                                                        .offset(*__slate_slot_1110 as isize)
                                                    })
                                                    .zEName
                                                })
                                                    as *const (),
                                            )
                                        };
                                    }
                                    unsafe {
                                        memcpy(
                                            *__slate_slot_1112 as *mut (),
                                            (unsafe {
                                                (*unsafe {
                                                    unsafe {
                                                        std::ptr::addr_of_mut!((*pToCol).a)
                                                            as *mut ExprList_item
                                                    }
                                                    .offset(*__slate_slot_1110 as isize)
                                                })
                                                .zEName
                                            })
                                                as *const (),
                                            (*__slate_slot_1115 as i64) as u64,
                                        )
                                    };
                                    unsafe {
                                        *unsafe {
                                            (*__slate_slot_1112).offset(*__slate_slot_1115 as isize)
                                        } = (0 as i32) as i8;
                                    }
                                    std::ptr::write(__slate_slot_2609, *__slate_slot_1112);
                                    std::ptr::write(__slate_slot_2610, unsafe {
                                        (*__slate_slot_2609)
                                            .offset((*__slate_slot_1115 + (1 as i32)) as isize)
                                    });
                                    *__slate_slot_1112 = *__slate_slot_2610;
                                    std::ptr::write(__slate_slot_2607, *__slate_slot_1110);
                                    std::ptr::write(
                                        __slate_slot_2608,
                                        *__slate_slot_2607 + (1 as i32),
                                    );
                                    *__slate_slot_1110 = *__slate_slot_2608;
                                } else {
                                    break '__join_5;
                                }
                            }
                        }
                    }
                    unsafe {
                        (*(*__slate_slot_1106)).isDeferred = ((0 as i32) as i8) as u8;
                    }
                    unsafe {
                        *unsafe {
                            unsafe { (*(*__slate_slot_1106)).aAction.as_mut_ptr() as *mut u8 }
                                .offset((0 as i32) as isize)
                        } = ((flags_1104 & (255 as i32)) as i8) as u8;
                    }
                    // ON DELETE action
                    unsafe {
                        *unsafe {
                            unsafe { (*(*__slate_slot_1106)).aAction.as_mut_ptr() as *mut u8 }
                                .offset((1 as i32) as isize)
                        } = ((flags_1104 >> (8 as i32) & (255 as i32)) as i8) as u8;
                    }
                    // ON UPDATE action
                    0 as i32;
                    *__slate_slot_1107 = (unsafe {
                        sqlite3HashInsert(
                            unsafe {
                                std::ptr::addr_of_mut!(
                                    (*unsafe { (*(*__slate_slot_1108)).pSchema }).fkeyHash
                                )
                            },
                            (unsafe { (*(*__slate_slot_1106)).zTo }) as *const i8,
                            *__slate_slot_1106 as *mut (),
                        )
                    }) as *mut FKey;
                    if *__slate_slot_1107 == *__slate_slot_1106 {
                        unsafe { sqlite3OomFault(*__slate_slot_1105) };
                    } else {
                        if *__slate_slot_1107 != std::ptr::null_mut::<FKey>() {
                            0 as i32;
                            unsafe {
                                (*(*__slate_slot_1106)).pNextTo = *__slate_slot_1107;
                            }
                            unsafe {
                                (*(*__slate_slot_1107)).pPrevTo = *__slate_slot_1106;
                            }
                        }
                        // Link the foreign key to the table as the last step.
                        0 as i32;
                        unsafe {
                            (*(*__slate_slot_1108)).u.tab.pFKey = *__slate_slot_1106;
                        }
                        *__slate_slot_1106 = std::ptr::null_mut::<FKey>();
                    }
                }
            }
        }
        unsafe { sqlite3DbFree(*__slate_slot_1105, *__slate_slot_1106 as *mut ()) };
        unsafe { sqlite3ExprListDelete(*__slate_slot_1105, pFromCol) };
        unsafe { sqlite3ExprListDelete(*__slate_slot_1105, pToCol) };
    }
}

/// This routine is called when an INITIALLY IMMEDIATE or INITIALLY DEFERRED
/// clause is seen as part of a foreign key definition.  The isDeferred
/// parameter is 1 for INITIALLY DEFERRED and 0 for INITIALLY IMMEDIATE.
/// The behavior of the most recently created foreign key is adjusted
/// accordingly.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3DeferForeignKey(mut pParse: *mut Parse, mut isDeferred: i32) {
    let mut pTab: *mut Table = unsafe { std::mem::zeroed() };
    let mut pFKey: *mut FKey = unsafe { std::mem::zeroed() };
    let __v2611: *mut Table = unsafe { (*pParse).pNewTable };
    pTab = __v2611;
    if __v2611 == std::ptr::null_mut::<Table>() {
        return;
    }
    if !((((unsafe { (*pTab).eTabType }) as u32) as i32) == (0 as i32)) {
        return;
    }
    let __v2612: *mut FKey = unsafe { (*pTab).u.tab.pFKey };
    pFKey = __v2612;
    if __v2612 == std::ptr::null_mut::<FKey>() {
        return;
    }
    0 as i32; // EV: R-30323-21917
    unsafe {
        (*pFKey).isDeferred = (isDeferred as i8) as u8;
    }
}

/// Generate code that will erase and refill index *pIdx.  This is
/// used to initialize a newly created index or to recompute the
/// content of an index in response to a REINDEX command.
///
/// if memRootPage is not negative, it means that the index is newly
/// created.  The register specified by memRootPage contains the
/// root page number of the index.  If memRootPage is negative, then
/// the index already exists and must be cleared before being refilled and
/// the root page number of the index is taken from pIndex->tnum.
fn sqlite3RefillIndex(mut pParse: *mut Parse, mut pIndex: *mut Index, mut memRootPage: i32) {
    let mut pTab: *mut Table = unsafe { (*pIndex).pTable }; // The table that is indexed
    let mut iTab: i32 = 0 as i32;
    let __v2822: *mut Parse = pParse;
    let __v2823: i32 = unsafe { (*__v2822).nTab };
    let __v2824: i32 = __v2823 + (1 as i32);
    unsafe {
        (*__v2822).nTab = __v2824;
    }
    iTab = __v2823; // Btree cursor used for pTab
    let mut iIdx: i32 = 0 as i32;
    let __v2825: *mut Parse = pParse;
    let __v2826: i32 = unsafe { (*__v2825).nTab };
    let __v2827: i32 = __v2826 + (1 as i32);
    unsafe {
        (*__v2825).nTab = __v2827;
    }
    iIdx = __v2826; // Btree cursor used for pIndex
    let mut iSorter: i32 = 0 as i32; // Cursor opened by OpenSorter (if in use)
    let mut addr1: i32 = 0 as i32; // Address of top of loop
    let mut addr2: i32 = 0 as i32; // Address to jump to for next iteration
    let mut tnum: u32 = 0 as u32; // Root page of index
    let mut iPartIdxLabel: i32 = 0 as i32; // Jump to this label to skip a row
    let mut v: *mut Vdbe = unsafe { std::mem::zeroed() }; // Generate code into this virtual machine
    let mut pKey: *mut KeyInfo = unsafe { std::mem::zeroed() }; // KeyInfo for index
    let mut regRecord: i32 = 0 as i32; // Register holding assembled index record
    let mut db: *mut sqlite3 = unsafe { (*pParse).db }; // The database connection
    let mut iDb: i32 = unsafe { sqlite3SchemaToIndex(db, unsafe { (*pIndex).pSchema }) };
    if (unsafe {
        sqlite3AuthCheck(
            pParse,
            27 as i32,
            (unsafe { (*pIndex).zName }) as *const i8,
            std::ptr::null::<i8>(),
            (unsafe { (*unsafe { unsafe { (*db).aDb }.offset(iDb as isize) }).zDbSName })
                as *const i8,
        )
    }) != (0 as i32)
    {
        return;
    }
    // Require a write-lock on the table to perform this operation
    sqlite3TableLock(
        pParse,
        iDb,
        unsafe { (*pTab).tnum },
        ((1 as i32) as i8) as u8,
        (unsafe { (*pTab).zName }) as *const i8,
    );
    v = unsafe { sqlite3GetVdbe(pParse) };
    if v == std::ptr::null_mut::<Vdbe>() {
        return;
    }
    if memRootPage >= (0 as i32) {
        tnum = memRootPage as u32;
    } else {
        tnum = unsafe { (*pIndex).tnum };
    }
    pKey = sqlite3KeyInfoOfIndex(pParse, pIndex);
    0 as i32;
    // Open the sorter cursor if we are to use one.
    let __v2828: *mut Parse = pParse;
    let __v2829: i32 = unsafe { (*__v2828).nTab };
    let __v2830: i32 = __v2829 + (1 as i32);
    unsafe {
        (*__v2828).nTab = __v2830;
    }
    iSorter = __v2829;
    unsafe {
        sqlite3VdbeAddOp4(
            v,
            121 as i32,
            iSorter,
            0 as i32,
            ((unsafe { (*pIndex).nKeyCol }) as u32) as i32,
            ((unsafe { sqlite3KeyInfoRef(pKey) }) as *mut i8) as *const i8,
            -(9 as i32),
        )
    };
    // Open the table. Loop through all rows of the table, inserting index
    // records into the sorter.
    unsafe { sqlite3OpenTable(pParse, iTab, iDb, pTab, 114 as i32) };
    addr1 = unsafe { sqlite3VdbeAddOp2(v, 36 as i32, iTab, 0 as i32) };
    {}
    regRecord = unsafe { sqlite3GetTempReg(pParse) };
    sqlite3MultiWrite(pParse);
    unsafe {
        sqlite3GenerateIndexKey(
            pParse,
            pIndex,
            iTab,
            regRecord,
            0 as i32,
            std::ptr::addr_of_mut!(iPartIdxLabel),
            std::ptr::null_mut::<Index>(),
            0 as i32,
        )
    };
    unsafe { sqlite3VdbeAddOp2(v, 141 as i32, iSorter, regRecord) };
    unsafe { sqlite3ResolvePartIdxLabel(pParse, iPartIdxLabel) };
    unsafe { sqlite3VdbeAddOp2(v, 40 as i32, iTab, addr1 + (1 as i32)) };
    {}
    unsafe { sqlite3VdbeJumpHere(v, addr1) };
    if memRootPage < (0 as i32) {
        unsafe { sqlite3VdbeAddOp2(v, 147 as i32, tnum as i32, iDb) };
    }
    unsafe {
        sqlite3VdbeAddOp4(
            v,
            116 as i32,
            iIdx,
            tnum as i32,
            iDb,
            (pKey as *mut i8) as *const i8,
            -(9 as i32),
        )
    };
    unsafe {
        sqlite3VdbeChangeP5(
            v,
            (((1 as i32)
                | if memRootPage >= (0 as i32) {
                    16 as i32
                } else {
                    0 as i32
                }) as i16) as u16,
        )
    };
    addr1 = unsafe { sqlite3VdbeAddOp2(v, 34 as i32, iSorter, 0 as i32) };
    {}
    if (((unsafe { (*pIndex).onError }) as u32) as i32) != (0 as i32) {
        let mut j2: i32 = unsafe { sqlite3VdbeGoto(v, 1 as i32) };
        addr2 = unsafe { sqlite3VdbeCurrentAddr(v) };
        {}
        unsafe {
            sqlite3VdbeAddOp4Int(
                v,
                134 as i32,
                iSorter,
                j2,
                regRecord,
                ((unsafe { (*pIndex).nKeyCol }) as u32) as i32,
            )
        };
        {}
        sqlite3UniqueConstraint(pParse, 2 as i32, pIndex);
        unsafe { sqlite3VdbeJumpHere(v, j2) };
    } else {
        // Most CREATE INDEX and REINDEX statements that are not UNIQUE can not
        // abort. The exception is if one of the indexed expressions contains a
        // user function that throws an exception when it is evaluated. But the
        // overhead of adding a statement journal to a CREATE INDEX statement is
        // very small (since most of the pages written do not contain content that
        // needs to be restored if the statement aborts), so we call
        // sqlite3MayAbort() for all CREATE INDEX statements.
        sqlite3MayAbort(pParse);
        addr2 = unsafe { sqlite3VdbeCurrentAddr(v) };
    }
    unsafe { sqlite3VdbeAddOp3(v, 135 as i32, iSorter, regRecord, iIdx) };
    if !(((unsafe { (*pIndex).__slate_bits_0.__get_bAscKeyBug() }) as i32) != (0 as i32)) {
        // This OP_SeekEnd opcode makes index insert for a REINDEX go much
        // faster by avoiding unnecessary seeks.  But the optimization does
        // not work for UNIQUE constraint indexes on WITHOUT ROWID tables
        // with DESC primary keys, since those indexes have there keys in
        // a different order from the main table.
        // See ticket: https://sqlite.org/src/info/bba7b69f9849b5bf
        unsafe { sqlite3VdbeAddOp1(v, 139 as i32, iIdx) };
    }
    unsafe { sqlite3VdbeAddOp2(v, 140 as i32, iIdx, regRecord) };
    unsafe { sqlite3VdbeChangeP5(v, ((16 as i32) as i16) as u16) };
    unsafe { sqlite3ReleaseTempReg(pParse, regRecord) };
    unsafe { sqlite3VdbeAddOp2(v, 38 as i32, iSorter, addr2) };
    {}
    unsafe { sqlite3VdbeJumpHere(v, addr1) };
    unsafe { sqlite3VdbeAddOp1(v, 124 as i32, iTab) };
    unsafe { sqlite3VdbeAddOp1(v, 124 as i32, iIdx) };
    unsafe { sqlite3VdbeAddOp1(v, 124 as i32, iSorter) };
}

/// Allocate heap space to hold an Index object with nCol columns.
///
/// Increase the allocation size to provide an extra nExtra bytes
/// of 8-byte aligned space after the Index object and return a
/// pointer to this extra space in *ppExtra.
///
/// # Arguments
///
/// * `db` - Database connection
/// * `nCol` - Total number of columns in the index
/// * `nExtra` - Number of bytes of extra space to alloc
/// * `ppExtra` - Pointer to the "extra" space
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3AllocateIndexObject(
    mut db: *mut sqlite3,
    mut nCol: i32,
    mut nExtra: i32,
    mut ppExtra: *mut *mut i8,
) -> *mut Index {
    let mut p: *mut Index = unsafe { std::mem::zeroed() }; // Allocated index object
    let mut nByte: i64 = 0 as i64; // Bytes of space for Index object + arrays
    0 as i32;
    nByte = ((112 as u64).wrapping_add(((7 as i32) as i64) as u64) & ((!(7 as i32) as i64) as u64))
        .wrapping_add(
            (8 as u64)
                .wrapping_mul((nCol as i64) as u64)
                .wrapping_add(((7 as i32) as i64) as u64)
                & ((!(7 as i32) as i64) as u64),
        )
        .wrapping_add(
            (2 as u64)
                .wrapping_mul(((nCol + (1 as i32)) as i64) as u64)
                .wrapping_add((2 as u64).wrapping_mul((nCol as i64) as u64))
                .wrapping_add((1 as u64).wrapping_mul((nCol as i64) as u64))
                .wrapping_add(((7 as i32) as i64) as u64)
                & ((!(7 as i32) as i64) as u64),
        ) as i64; // Index structure
    // Index.azColl
    // Index.aiRowLogEst
    // Index.aiColumn
    // Index.aSortOrder
    p = (unsafe { sqlite3DbMallocZero(db, (nByte + (nExtra as i64)) as u64) }) as *mut Index;
    if p != std::ptr::null_mut::<Index>() {
        let mut pExtra: *mut i8 = unsafe {
            (p as *mut i8).offset(
                ((112 as u64).wrapping_add(((7 as i32) as i64) as u64)
                    & ((!(7 as i32) as i64) as u64)) as isize,
            )
        };
        unsafe {
            (*p).azColl = pExtra as *mut *const i8;
        }
        let __v2508: *mut i8 = pExtra;
        let __v2509: *mut i8 = unsafe {
            __v2508.offset(
                ((8 as u64)
                    .wrapping_mul((nCol as i64) as u64)
                    .wrapping_add(((7 as i32) as i64) as u64)
                    & ((!(7 as i32) as i64) as u64)) as isize,
            )
        };
        pExtra = __v2509;
        unsafe {
            (*p).aiRowLogEst = pExtra as *mut i16;
        }
        let __v2510: *mut i8 = pExtra;
        let __v2511: *mut i8 = unsafe {
            __v2510.offset((2 as u64).wrapping_mul(((nCol + (1 as i32)) as i64) as u64) as isize)
        };
        pExtra = __v2511;
        unsafe {
            (*p).aiColumn = pExtra as *mut i16;
        }
        let __v2512: *mut i8 = pExtra;
        let __v2513: *mut i8 =
            unsafe { __v2512.offset((2 as u64).wrapping_mul((nCol as i64) as u64) as isize) };
        pExtra = __v2513;
        unsafe {
            (*p).aSortOrder = pExtra as *mut u8;
        }
        0 as i32;
        unsafe {
            (*p).nColumn = (nCol as i16) as u16;
        }
        unsafe {
            (*p).nKeyCol = ((nCol - (1 as i32)) as i16) as u16;
        }
        unsafe {
            *ppExtra = unsafe { (p as *mut i8).offset(nByte as isize) };
        }
    }
    return p;
}

/// If expression list pList contains an expression that was parsed with
/// an explicit "NULLS FIRST" or "NULLS LAST" clause, leave an error in
/// pParse and return non-zero. Otherwise, return zero.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3HasExplicitNulls(
    mut pParse: *mut Parse,
    mut pList: *mut ExprList,
) -> i32 {
    if pList != std::ptr::null_mut::<ExprList>() {
        let mut i: i32 = 0 as i32;
        i = 0 as i32;
        '__slate_break_2181: loop {
            if !(i < unsafe { (*pList).nExpr }) {
                break;
            }
            if ((unsafe {
                (*unsafe {
                    unsafe { std::ptr::addr_of_mut!((*pList).a) as *mut ExprList_item }
                        .offset(i as isize)
                })
                .fg
                .__slate_bits_0
                .__get_bNulls()
            }) as i32)
                != (0 as i32)
            {
                let mut sf: u8 = unsafe {
                    (*unsafe {
                        unsafe { std::ptr::addr_of_mut!((*pList).a) as *mut ExprList_item }
                            .offset(i as isize)
                    })
                    .fg
                    .sortFlags
                };
                unsafe {
                    sqlite3ErrorMsg(
                        pParse,
                        (b"unsupported use of NULLS %s\0".as_ptr() as *mut i8) as *const i8,
                        if ((sf as u32) as i32) == (0 as i32) || ((sf as u32) as i32) == (3 as i32)
                        {
                            b"FIRST\0".as_ptr() as *mut i8
                        } else {
                            b"LAST\0".as_ptr() as *mut i8
                        },
                    )
                };
                return 1 as i32;
            }
            let __v2661: i32 = i;
            let __v2662: i32 = __v2661 + (1 as i32);
            i = __v2662;
        }
    }
    return 0 as i32;
}

/// Create a new index for an SQL table.  pName1.pName2 is the name of the index
/// and pTblList is the name of the table that is to be indexed.  Both will
/// be NULL for a primary key or an index that is created to satisfy a
/// UNIQUE constraint.  If pTable and pIndex are NULL, use pParse->pNewTable
/// as the table to be indexed.  pParse->pNewTable is a table that is
/// currently being constructed by a CREATE TABLE statement.
///
/// pList is a list of columns to be indexed.  pList will be NULL if this
/// is a primary key or unique-constraint on the most recent column added
/// to the table currently under construction.
///
/// # Arguments
///
/// * `pParse` - All information about this parse
/// * `pName1` - First part of index name. May be NULL
/// * `pName2` - Second part of index name. May be NULL
/// * `pTblName` - Table to index. Use pParse->pNewTable if 0
/// * `pList` - A list of columns to be indexed
/// * `onError` - OE_Abort, OE_Ignore, OE_Replace, or OE_None
/// * `pStart` - The CREATE token that begins this statement
/// * `pPIWhere` - WHERE clause for partial indices
/// * `sortOrder` - Sort order of primary key when pList==NULL
/// * `ifNotExist` - Omit error if index already exists
/// * `idxType` - The index type
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3CreateIndex(
    mut pParse: *mut Parse,
    mut pName1: *mut Token,
    mut pName2: *mut Token,
    mut pTblName: *mut SrcList,
    mut pList: *mut ExprList,
    mut onError: i32,
    mut pStart: *mut Token,
    mut pPIWhere: *mut Expr,
    mut sortOrder: i32,
    mut ifNotExist: i32,
    mut idxType: u8,
) {
    let mut __slate_storage_2558: std::mem::MaybeUninit<*mut Index> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_2558: *mut *mut Index =
        std::ptr::addr_of_mut!(__slate_storage_2558) as *mut *mut Index;
    let mut __slate_storage_2559: std::mem::MaybeUninit<*mut Index> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_2559: *mut *mut Index =
        std::ptr::addr_of_mut!(__slate_storage_2559) as *mut *mut Index;
    let mut __slate_storage_1200: std::mem::MaybeUninit<*mut Index> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1200: *mut *mut Index =
        std::ptr::addr_of_mut!(__slate_storage_1200) as *mut *mut Index;
    let mut __slate_storage_1199: std::mem::MaybeUninit<*mut Index> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1199: *mut *mut Index =
        std::ptr::addr_of_mut!(__slate_storage_1199) as *mut *mut Index;
    // Ensure all REPLACE indexes on pTab are at the end of the pIndex list.
    // The list was already ordered when this routine was entered, so at this
    // point at most a single index (the newly added index) will be out of
    // order.  So we have to reorder at most one index.
    let mut __slate_storage_1198: std::mem::MaybeUninit<*mut *mut Index> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1198: *mut *mut *mut Index =
        std::ptr::addr_of_mut!(__slate_storage_1198) as *mut *mut *mut Index;
    let mut __slate_storage_2552: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2552: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_2552) as *mut u32;
    let mut __slate_storage_2551: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2551: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_2551) as *mut u32;
    let mut __slate_storage_2550: std::mem::MaybeUninit<*mut sqlite3> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_2550: *mut *mut sqlite3 =
        std::ptr::addr_of_mut!(__slate_storage_2550) as *mut *mut sqlite3;
    let mut __slate_storage_1193: std::mem::MaybeUninit<*mut Index> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1193: *mut *mut Index =
        std::ptr::addr_of_mut!(__slate_storage_1193) as *mut *mut Index;
    let mut __slate_storage_2557: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2557: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_2557) as *mut i32;
    let mut __slate_storage_2556: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2556: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_2556) as *mut i32;
    let mut __slate_storage_1197: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1197: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1197) as *mut i32;
    let mut __slate_storage_2555: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2555: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_2555) as *mut i32;
    let mut __slate_storage_2554: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2554: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_2554) as *mut i32;
    let mut __slate_storage_2553: std::mem::MaybeUninit<*mut Parse> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_2553: *mut *mut Parse =
        std::ptr::addr_of_mut!(__slate_storage_2553) as *mut *mut Parse;
    let mut __slate_storage_1196: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1196: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1196) as *mut i32;
    let mut __slate_storage_1195: std::mem::MaybeUninit<*mut i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1195: *mut *mut i8 =
        std::ptr::addr_of_mut!(__slate_storage_1195) as *mut *mut i8;
    let mut __slate_storage_1194: std::mem::MaybeUninit<*mut Vdbe> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1194: *mut *mut Vdbe =
        std::ptr::addr_of_mut!(__slate_storage_1194) as *mut *mut Vdbe;
    let mut __slate_storage_2549: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2549: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_2549) as *mut i32;
    let mut __slate_storage_2548: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2548: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_2548) as *mut i32;
    let mut __slate_storage_1192: std::mem::MaybeUninit<*const i8> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1192: *mut *const i8 =
        std::ptr::addr_of_mut!(__slate_storage_1192) as *mut *const i8;
    let mut __slate_storage_1191: std::mem::MaybeUninit<*const i8> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1191: *mut *const i8 =
        std::ptr::addr_of_mut!(__slate_storage_1191) as *mut *const i8;
    let mut __slate_storage_1190: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1190: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1190) as *mut i32;
    // This routine has been called to create an automatic index as a
    // result of a PRIMARY KEY or UNIQUE clause on a column definition, or
    // a PRIMARY KEY or UNIQUE clause following the column definitions.
    // i.e. one of:
    //
    // CREATE TABLE t(x PRIMARY KEY, y);
    // CREATE TABLE t(x, y, UNIQUE(x, y));
    //
    // Either way, check to see if the table already has such an index. If
    // so, don't bother creating this one. This only applies to
    // automatically created indices. Users can do as they wish with
    // explicit indices.
    //
    // Two UNIQUE or PRIMARY KEY constraints are considered equivalent
    // (and thus suppressing the second one) even if they have different
    // sort orders.
    //
    // If there are different collating sequences or if the columns of
    // the constraint occur in different orders, then the constraints are
    // considered distinct and both result in separate indices.
    let mut __slate_storage_1189: std::mem::MaybeUninit<*mut Index> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1189: *mut *mut Index =
        std::ptr::addr_of_mut!(__slate_storage_1189) as *mut *mut Index;
    let mut __slate_storage_2547: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2547: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_2547) as *mut i32;
    let mut __slate_storage_2546: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2546: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_2546) as *mut i32;
    let mut __slate_storage_2540: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2540: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_2540) as *mut i32;
    let mut __slate_storage_2539: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2539: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_2539) as *mut i32;
    let mut __slate_storage_2543: std::mem::MaybeUninit<u16> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2543: *mut u16 = std::ptr::addr_of_mut!(__slate_storage_2543) as *mut u16;
    let mut __slate_storage_2542: std::mem::MaybeUninit<u16> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2542: *mut u16 = std::ptr::addr_of_mut!(__slate_storage_2542) as *mut u16;
    let mut __slate_storage_2541: std::mem::MaybeUninit<*mut Index> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_2541: *mut *mut Index =
        std::ptr::addr_of_mut!(__slate_storage_2541) as *mut *mut Index;
    let mut __slate_storage_2545: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2545: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_2545) as *mut i32;
    let mut __slate_storage_2544: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2544: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_2544) as *mut i32;
    let mut __slate_storage_1188: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1188: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1188) as *mut i32;
    let mut __slate_storage_2533: std::mem::MaybeUninit<*mut ExprList_item> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_2533: *mut *mut ExprList_item =
        std::ptr::addr_of_mut!(__slate_storage_2533) as *mut *mut ExprList_item;
    let mut __slate_storage_2532: std::mem::MaybeUninit<*mut ExprList_item> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_2532: *mut *mut ExprList_item =
        std::ptr::addr_of_mut!(__slate_storage_2532) as *mut *mut ExprList_item;
    let mut __slate_storage_2531: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2531: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_2531) as *mut i32;
    let mut __slate_storage_2530: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2530: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_2530) as *mut i32;
    let mut __slate_storage_2538: std::mem::MaybeUninit<bool> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2538: *mut bool = std::ptr::addr_of_mut!(__slate_storage_2538) as *mut bool;
    let mut __slate_storage_2537: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2537: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_2537) as *mut i32;
    let mut __slate_storage_2536: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2536: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_2536) as *mut i32;
    let mut __slate_storage_2535: std::mem::MaybeUninit<*mut i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2535: *mut *mut i8 =
        std::ptr::addr_of_mut!(__slate_storage_2535) as *mut *mut i8;
    let mut __slate_storage_2534: std::mem::MaybeUninit<*mut i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2534: *mut *mut i8 =
        std::ptr::addr_of_mut!(__slate_storage_2534) as *mut *mut i8;
    let mut __slate_storage_1187: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1187: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1187) as *mut i32; // Collation sequence name
    let mut __slate_storage_1186: std::mem::MaybeUninit<*const i8> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1186: *mut *const i8 =
        std::ptr::addr_of_mut!(__slate_storage_1186) as *mut *const i8; // ASC or DESC on the i-th expression
    let mut __slate_storage_1185: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1185: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1185) as *mut i32; // The i-th index expression
    let mut __slate_storage_1184: std::mem::MaybeUninit<*mut Expr> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1184: *mut *mut Expr =
        std::ptr::addr_of_mut!(__slate_storage_1184) as *mut *mut Expr;
    let mut __slate_storage_2529: std::mem::MaybeUninit<*mut i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2529: *mut *mut i8 =
        std::ptr::addr_of_mut!(__slate_storage_2529) as *mut *mut i8;
    let mut __slate_storage_2528: std::mem::MaybeUninit<*mut i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2528: *mut *mut i8 =
        std::ptr::addr_of_mut!(__slate_storage_2528) as *mut *mut i8;
    let mut __slate_storage_2525: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2525: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_2525) as *mut i32;
    let mut __slate_storage_2524: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2524: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_2524) as *mut i32;
    let mut __slate_storage_2527: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2527: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_2527) as *mut i32;
    let mut __slate_storage_2526: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2526: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_2526) as *mut i32;
    let mut __slate_storage_1183: std::mem::MaybeUninit<*mut Expr> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1183: *mut *mut Expr =
        std::ptr::addr_of_mut!(__slate_storage_1183) as *mut *mut Expr;
    let mut __slate_storage_2523: std::mem::MaybeUninit<u16> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2523: *mut u16 = std::ptr::addr_of_mut!(__slate_storage_2523) as *mut u16;
    let mut __slate_storage_2522: std::mem::MaybeUninit<u16> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2522: *mut u16 = std::ptr::addr_of_mut!(__slate_storage_2522) as *mut u16;
    let mut __slate_storage_2521: std::mem::MaybeUninit<*mut Column> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_2521: *mut *mut Column =
        std::ptr::addr_of_mut!(__slate_storage_2521) as *mut *mut Column;
    let mut __slate_storage_1182: std::mem::MaybeUninit<*mut Column> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1182: *mut *mut Column =
        std::ptr::addr_of_mut!(__slate_storage_1182) as *mut *mut Column;
    let mut __slate_storage_1181: std::mem::MaybeUninit<Token> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1181: *mut Token = std::ptr::addr_of_mut!(__slate_storage_1181) as *mut Token;
    let mut __slate_storage_1180: std::mem::MaybeUninit<*const i8> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1180: *mut *const i8 =
        std::ptr::addr_of_mut!(__slate_storage_1180) as *mut *const i8;
    let mut __slate_storage_2520: std::mem::MaybeUninit<i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2520: *mut i8 = std::ptr::addr_of_mut!(__slate_storage_2520) as *mut i8;
    let mut __slate_storage_2519: std::mem::MaybeUninit<i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2519: *mut i8 = std::ptr::addr_of_mut!(__slate_storage_2519) as *mut i8;
    let mut __slate_storage_2518: std::mem::MaybeUninit<*mut i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2518: *mut *mut i8 =
        std::ptr::addr_of_mut!(__slate_storage_2518) as *mut *mut i8;
    let mut __slate_storage_2517: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2517: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_2517) as *mut i32;
    let mut __slate_storage_2516: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2516: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_2516) as *mut i32;
    let mut __slate_storage_1179: std::mem::MaybeUninit<*mut Index> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1179: *mut *mut Index =
        std::ptr::addr_of_mut!(__slate_storage_1179) as *mut *mut Index;
    let mut __slate_storage_1178: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1178: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1178) as *mut i32; // PRIMARY KEY index for WITHOUT ROWID tables
    let mut __slate_storage_1177: std::mem::MaybeUninit<*mut Index> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1177: *mut *mut Index =
        std::ptr::addr_of_mut!(__slate_storage_1177) as *mut *mut Index; // Extra space after the Index object
    let mut __slate_storage_1176: std::mem::MaybeUninit<*mut i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1176: *mut *mut i8 =
        std::ptr::addr_of_mut!(__slate_storage_1176) as *mut *mut i8; // Number of extra columns needed
    let mut __slate_storage_1175: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1175: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1175) as *mut i32; // Space allocated for zExtra[]
    let mut __slate_storage_1174: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1174: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1174) as *mut i32; // For looping over pList
    let mut __slate_storage_1173: std::mem::MaybeUninit<*mut ExprList_item> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1173: *mut *mut ExprList_item =
        std::ptr::addr_of_mut!(__slate_storage_1173) as *mut *mut ExprList_item; // Unqualified name of the index to create
    let mut __slate_storage_1172: std::mem::MaybeUninit<*mut Token> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1172: *mut *mut Token =
        std::ptr::addr_of_mut!(__slate_storage_1172) as *mut *mut Token; // Index of the database that is being written
    let mut __slate_storage_1171: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1171: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1171) as *mut i32; // The specific table containing the indexed database
    let mut __slate_storage_1170: std::mem::MaybeUninit<*mut Db> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1170: *mut *mut Db =
        std::ptr::addr_of_mut!(__slate_storage_1170) as *mut *mut Db;
    let mut __slate_storage_1169: std::mem::MaybeUninit<*mut sqlite3> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1169: *mut *mut sqlite3 =
        std::ptr::addr_of_mut!(__slate_storage_1169) as *mut *mut sqlite3; // 1 to honor DESC in index.  0 to ignore.
    let mut __slate_storage_1168: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1168: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1168) as *mut i32; // For assigning database names to pTable
    let mut __slate_storage_1167: std::mem::MaybeUninit<DbFixer> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1167: *mut DbFixer =
        std::ptr::addr_of_mut!(__slate_storage_1167) as *mut DbFixer;
    let mut __slate_storage_1166: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1166: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1166) as *mut i32;
    let mut __slate_storage_1165: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1165: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1165) as *mut i32; // Number of characters in zName
    let mut __slate_storage_1164: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1164: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1164) as *mut i32; // Name of the index
    let mut __slate_storage_1163: std::mem::MaybeUninit<*mut i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1163: *mut *mut i8 =
        std::ptr::addr_of_mut!(__slate_storage_1163) as *mut *mut i8; // The index to be created
    let mut __slate_storage_1162: std::mem::MaybeUninit<*mut Index> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1162: *mut *mut Index =
        std::ptr::addr_of_mut!(__slate_storage_1162) as *mut *mut Index; // Table to be indexed
    let mut __slate_storage_1161: std::mem::MaybeUninit<*mut Table> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1161: *mut *mut Table =
        std::ptr::addr_of_mut!(__slate_storage_1161) as *mut *mut Table;
    unsafe {
        '__join_9: {
            std::ptr::write(__slate_slot_1161, std::ptr::null_mut::<Table>());
            std::ptr::write(__slate_slot_1162, std::ptr::null_mut::<Index>());
            std::ptr::write(__slate_slot_1163, std::ptr::null_mut::<i8>());
            std::ptr::write(__slate_slot_1169, unsafe { (*pParse).db });
            std::ptr::write(__slate_slot_1172, std::ptr::null_mut::<Token>());
            std::ptr::write(__slate_slot_1174, 0 as i32);
            std::ptr::write(__slate_slot_1176, std::ptr::null_mut::<i8>());
            std::ptr::write(__slate_slot_1177, std::ptr::null_mut::<Index>());
            0 as i32;
            if (unsafe { (*pParse).nErr }) != (0 as i32) {
            } else {
                0 as i32;
                if (((unsafe { (*pParse).eParseMode }) as u32) as i32) == (1 as i32)
                    && ((idxType as u32) as i32) != (2 as i32)
                {
                } else {
                    if (0 as i32) != unsafe { sqlite3ReadSchema(pParse) } {
                    } else {
                        if sqlite3HasExplicitNulls(pParse, pList) != (0 as i32) {
                        } else {
                            // Find the table that is to be indexed.  Return early if not found.
                            if pTblName != std::ptr::null_mut::<SrcList>() {
                                // Use the two-part index name to determine the database
                                // to search for the table. 'Fix' the table name to this db
                                // before looking up the table.
                                0 as i32;
                                *__slate_slot_1171 = sqlite3TwoPartName(
                                    pParse,
                                    pName1,
                                    pName2,
                                    std::ptr::addr_of_mut!(*__slate_slot_1172),
                                );
                                if *__slate_slot_1171 < (0 as i32) {
                                    break '__join_9;
                                } else {
                                    0 as i32;
                                    // If the index name was unqualified, check if the table
                                    // is a temp table. If so, set the database to 1. Do not do this
                                    // if initializing a database schema.
                                    if !((unsafe { (*(*__slate_slot_1169)).init.busy })
                                        != (0 as u8))
                                    {
                                        *__slate_slot_1161 =
                                            unsafe { sqlite3SrcListLookup(pParse, pTblName) };
                                        if (unsafe { (*pName2).n }) == ((0 as i32) as u32)
                                            && *__slate_slot_1161 != std::ptr::null_mut::<Table>()
                                            && (unsafe { (*(*__slate_slot_1161)).pSchema })
                                                == unsafe {
                                                    (*unsafe {
                                                        unsafe { (*(*__slate_slot_1169)).aDb }
                                                            .offset((1 as i32) as isize)
                                                    })
                                                    .pSchema
                                                }
                                        {
                                            *__slate_slot_1171 = 1 as i32;
                                        }
                                    }
                                    unsafe {
                                        sqlite3FixInit(
                                            std::ptr::addr_of_mut!(*__slate_slot_1167),
                                            pParse,
                                            *__slate_slot_1171,
                                            (b"index\0".as_ptr() as *mut i8) as *const i8,
                                            *__slate_slot_1172 as *const Token,
                                        )
                                    };
                                    if (unsafe {
                                        sqlite3FixSrcList(
                                            std::ptr::addr_of_mut!(*__slate_slot_1167),
                                            pTblName,
                                        )
                                    }) != (0 as i32)
                                    {
                                        // Because the parser constructs pTblName from a single identifier,
                                        // sqlite3FixSrcList can never fail.
                                        0 as i32;
                                    }
                                    *__slate_slot_1161 =
                                        sqlite3LocateTableItem(pParse, (0 as i32) as u32, unsafe {
                                            unsafe {
                                                std::ptr::addr_of_mut!((*pTblName).a)
                                                    as *mut SrcItem
                                            }
                                            .offset((0 as i32) as isize)
                                        });
                                    0 as i32;
                                    if *__slate_slot_1161 == std::ptr::null_mut::<Table>() {
                                        break '__join_9;
                                    } else {
                                        if *__slate_slot_1171 == (1 as i32)
                                            && (unsafe {
                                                (*unsafe {
                                                    unsafe { (*(*__slate_slot_1169)).aDb }
                                                        .offset(*__slate_slot_1171 as isize)
                                                })
                                                .pSchema
                                            }) != unsafe { (*(*__slate_slot_1161)).pSchema }
                                        {
                                            unsafe {
                                                sqlite3ErrorMsg(pParse, (b"cannot create a TEMP index on non-TEMP table \"%s\"\0".as_ptr() as *mut i8) as *const i8, unsafe { (*(*__slate_slot_1161)).zName })
                                            };
                                            break '__join_9;
                                        } else {
                                            if !((unsafe { (*(*__slate_slot_1161)).tabFlags })
                                                & ((128 as i32) as u32)
                                                == ((0 as i32) as u32))
                                            {
                                                *__slate_slot_1177 =
                                                    sqlite3PrimaryKeyIndex(*__slate_slot_1161);
                                            }
                                        }
                                    }
                                }
                            } else {
                                0 as i32;
                                0 as i32;
                                *__slate_slot_1161 = unsafe { (*pParse).pNewTable };
                                if !(*__slate_slot_1161 != std::ptr::null_mut::<Table>()) {
                                    break '__join_9;
                                } else {
                                    *__slate_slot_1171 = unsafe {
                                        sqlite3SchemaToIndex(*__slate_slot_1169, unsafe {
                                            (*(*__slate_slot_1161)).pSchema
                                        })
                                    };
                                }
                            }
                            *__slate_slot_1170 = unsafe {
                                unsafe { (*(*__slate_slot_1169)).aDb }
                                    .offset(*__slate_slot_1171 as isize)
                            };
                            0 as i32;
                            if (unsafe {
                                sqlite3_strnicmp(
                                    (unsafe { (*(*__slate_slot_1161)).zName }) as *const i8,
                                    (b"sqlite_\0".as_ptr() as *mut i8) as *const i8,
                                    7 as i32,
                                )
                            }) == (0 as i32)
                                && (((unsafe { (*(*__slate_slot_1169)).init.busy }) as u32) as i32)
                                    == (0 as i32)
                                && pTblName != std::ptr::null_mut::<SrcList>()
                            {
                                unsafe {
                                    sqlite3ErrorMsg(
                                        pParse,
                                        (b"table %s may not be indexed\0".as_ptr() as *mut i8)
                                            as *const i8,
                                        unsafe { (*(*__slate_slot_1161)).zName },
                                    )
                                };
                            } else {
                                if (((unsafe { (*(*__slate_slot_1161)).eTabType }) as u32) as i32)
                                    == (2 as i32)
                                {
                                    unsafe {
                                        sqlite3ErrorMsg(
                                            pParse,
                                            (b"views may not be indexed\0".as_ptr() as *mut i8)
                                                as *const i8,
                                        )
                                    };
                                } else {
                                    if (((unsafe { (*(*__slate_slot_1161)).eTabType }) as u32)
                                        as i32)
                                        == (1 as i32)
                                    {
                                        unsafe {
                                            sqlite3ErrorMsg(
                                                pParse,
                                                (b"virtual tables may not be indexed\0".as_ptr()
                                                    as *mut i8)
                                                    as *const i8,
                                            )
                                        };
                                    } else {
                                        // Find the name of the index.  Make sure there is not already another
                                        // index or table with the same name.
                                        //
                                        // Exception:  If we are reading the names of permanent indices from the
                                        // sqlite_schema table (because some other process changed the schema) and
                                        // one of the index names collides with the name of a temporary table or
                                        // index, then we will continue to process this index.
                                        //
                                        // If pName==0 it means that we are
                                        // dealing with a primary key or UNIQUE constraint.  We have to invent our
                                        // own name.
                                        if *__slate_slot_1172 != std::ptr::null_mut::<Token>() {
                                            *__slate_slot_1163 = sqlite3NameFromToken(
                                                *__slate_slot_1169,
                                                *__slate_slot_1172 as *const Token,
                                            );
                                            if *__slate_slot_1163 == std::ptr::null_mut::<i8>() {
                                                break '__join_9;
                                            } else {
                                                0 as i32;
                                                if (0 as i32)
                                                    != sqlite3CheckObjectName(
                                                        pParse,
                                                        *__slate_slot_1163 as *const i8,
                                                        (b"index\0".as_ptr() as *mut i8)
                                                            as *const i8,
                                                        (unsafe { (*(*__slate_slot_1161)).zName })
                                                            as *const i8,
                                                    )
                                                {
                                                    break '__join_9;
                                                } else {
                                                    if !((((unsafe { (*pParse).eParseMode }) as u32)
                                                        as i32)
                                                        >= (2 as i32))
                                                    {
                                                        if !((unsafe {
                                                            (*(*__slate_slot_1169)).init.busy
                                                        }) != (0 as u8))
                                                        {
                                                            if sqlite3FindTable(
                                                                *__slate_slot_1169,
                                                                *__slate_slot_1163 as *const i8,
                                                                (unsafe {
                                                                    (*(*__slate_slot_1170)).zDbSName
                                                                })
                                                                    as *const i8,
                                                            ) != std::ptr::null_mut::<Table>()
                                                            {
                                                                unsafe {
                                                                    sqlite3ErrorMsg(pParse, (b"there is already a table named %s\0".as_ptr() as *mut i8) as *const i8, *__slate_slot_1163)
                                                                };
                                                                break '__join_9;
                                                            }
                                                        }
                                                        if sqlite3FindIndex(
                                                            *__slate_slot_1169,
                                                            *__slate_slot_1163 as *const i8,
                                                            (unsafe {
                                                                (*(*__slate_slot_1170)).zDbSName
                                                            })
                                                                as *const i8,
                                                        ) != std::ptr::null_mut::<Index>()
                                                        {
                                                            if !(ifNotExist != (0 as i32)) {
                                                                unsafe {
                                                                    sqlite3ErrorMsg(pParse, (b"index %s already exists\0".as_ptr() as *mut i8) as *const i8, *__slate_slot_1163)
                                                                };
                                                                break '__join_9;
                                                            } else {
                                                                if (unsafe {
                                                                    (*(*__slate_slot_1169))
                                                                        .init
                                                                        .busy
                                                                }) != (0 as u8)
                                                                {
                                                                    unsafe {
                                                                        sqlite3ErrorMsg(
                                                                            pParse,
                                                                            (b"\0".as_ptr()
                                                                                as *mut i8)
                                                                                as *const i8,
                                                                        )
                                                                    }; // corruptSchema() will do the error
                                                                    break '__join_9;
                                                                } else {
                                                                    sqlite3CodeVerifySchema(
                                                                        pParse,
                                                                        *__slate_slot_1171,
                                                                    );
                                                                    sqlite3ForceNotReadOnly(pParse);
                                                                    break '__join_9;
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        } else {
                                            *__slate_slot_1179 =
                                                unsafe { (*(*__slate_slot_1161)).pIndex };
                                            *__slate_slot_1178 = 1 as i32;
                                            loop {
                                                if *__slate_slot_1179
                                                    != std::ptr::null_mut::<Index>()
                                                {
                                                    *__slate_slot_1179 =
                                                        unsafe { (*(*__slate_slot_1179)).pNext };
                                                    std::ptr::write(
                                                        __slate_slot_2516,
                                                        *__slate_slot_1178,
                                                    );
                                                    std::ptr::write(
                                                        __slate_slot_2517,
                                                        *__slate_slot_2516 + (1 as i32),
                                                    );
                                                    *__slate_slot_1178 = *__slate_slot_2517;
                                                } else {
                                                    break;
                                                }
                                            }
                                            *__slate_slot_1163 = unsafe {
                                                sqlite3MPrintf(
                                                    *__slate_slot_1169,
                                                    (b"sqlite_autoindex_%s_%d\0".as_ptr()
                                                        as *mut i8)
                                                        as *const i8,
                                                    unsafe { (*(*__slate_slot_1161)).zName },
                                                    *__slate_slot_1178,
                                                )
                                            };
                                            if *__slate_slot_1163 == std::ptr::null_mut::<i8>() {
                                                break '__join_9;
                                            } else {
                                                // Automatic index names generated from within sqlite3_declare_vtab()
                                                // must have names that are distinct from normal automatic index names.
                                                // The following statement converts "sqlite3_autoindex..." into
                                                // "sqlite3_butoindex..." in order to make the names distinct.
                                                // The "vtab_err.test" test demonstrates the need of this statement.
                                                if (((unsafe { (*pParse).eParseMode }) as u32)
                                                    as i32)
                                                    != (0 as i32)
                                                {
                                                    std::ptr::write(__slate_slot_2518, unsafe {
                                                        (*__slate_slot_1163)
                                                            .offset((7 as i32) as isize)
                                                    });
                                                    std::ptr::write(__slate_slot_2519, unsafe {
                                                        *(*__slate_slot_2518)
                                                    });
                                                    std::ptr::write(
                                                        __slate_slot_2520,
                                                        ((*__slate_slot_2519 as i32) + (1 as i32))
                                                            as i8,
                                                    );
                                                    unsafe {
                                                        *(*__slate_slot_2518) = *__slate_slot_2520;
                                                    }
                                                }
                                            }
                                        }
                                        // Check for authorization to create an index.
                                        if !((((unsafe { (*pParse).eParseMode }) as u32) as i32)
                                            >= (2 as i32))
                                        {
                                            std::ptr::write(
                                                __slate_slot_1180,
                                                (unsafe { (*(*__slate_slot_1170)).zDbSName })
                                                    as *const i8,
                                            );
                                            if (unsafe {
                                                sqlite3AuthCheck(
                                                    pParse,
                                                    18 as i32,
                                                    (if !((0 as i32) != (0 as i32))
                                                        && *__slate_slot_1171 == (1 as i32)
                                                    {
                                                        b"sqlite_temp_master\0".as_ptr() as *mut i8
                                                    } else {
                                                        b"sqlite_master\0".as_ptr() as *mut i8
                                                    })
                                                        as *const i8,
                                                    std::ptr::null::<i8>(),
                                                    *__slate_slot_1180,
                                                )
                                            }) != (0 as i32)
                                            {
                                                break '__join_9;
                                            } else {
                                                *__slate_slot_1165 = 1 as i32;
                                                if !((0 as i32) != (0 as i32))
                                                    && *__slate_slot_1171 == (1 as i32)
                                                {
                                                    *__slate_slot_1165 = 3 as i32;
                                                }
                                                if (unsafe {
                                                    sqlite3AuthCheck(
                                                        pParse,
                                                        *__slate_slot_1165,
                                                        *__slate_slot_1163 as *const i8,
                                                        (unsafe { (*(*__slate_slot_1161)).zName })
                                                            as *const i8,
                                                        *__slate_slot_1180,
                                                    )
                                                }) != (0 as i32)
                                                {
                                                    break '__join_9;
                                                }
                                            }
                                        }
                                        // If pList==0, it means this routine was called to make a primary
                                        // key out of the last column added to the table under construction.
                                        // So create a fake list to simulate this.
                                        if pList == std::ptr::null_mut::<ExprList>() {
                                            std::ptr::write(__slate_slot_1182, unsafe {
                                                unsafe { (*(*__slate_slot_1161)).aCol }.offset(
                                                    (((unsafe { (*(*__slate_slot_1161)).nCol })
                                                        as i32)
                                                        - (1 as i32))
                                                        as isize,
                                                )
                                            });
                                            std::ptr::write(__slate_slot_2521, *__slate_slot_1182);
                                            std::ptr::write(__slate_slot_2522, unsafe {
                                                (*(*__slate_slot_2521)).colFlags
                                            });
                                            std::ptr::write(
                                                __slate_slot_2523,
                                                ((((*__slate_slot_2522 as u32) as i32) | (8 as i32))
                                                    as i16)
                                                    as u16,
                                            );
                                            unsafe {
                                                (*(*__slate_slot_2521)).colFlags =
                                                    *__slate_slot_2523;
                                            }
                                            unsafe {
                                                sqlite3TokenInit(
                                                    std::ptr::addr_of_mut!(*__slate_slot_1181),
                                                    unsafe { (*(*__slate_slot_1182)).zCnName },
                                                )
                                            };
                                            pList = unsafe {
                                                sqlite3ExprListAppend(
                                                    pParse,
                                                    std::ptr::null_mut::<ExprList>(),
                                                    unsafe {
                                                        sqlite3ExprAlloc(
                                                            *__slate_slot_1169,
                                                            60 as i32,
                                                            std::ptr::addr_of_mut!(
                                                                *__slate_slot_1181
                                                            )
                                                                as *const Token,
                                                            0 as i32,
                                                        )
                                                    },
                                                )
                                            };
                                            if pList == std::ptr::null_mut::<ExprList>() {
                                                break '__join_9;
                                            } else {
                                                0 as i32;
                                                unsafe {
                                                    sqlite3ExprListSetSortOrder(
                                                        pList,
                                                        sortOrder,
                                                        -(1 as i32),
                                                    )
                                                };
                                            }
                                        } else {
                                            unsafe {
                                                sqlite3ExprListCheckLength(
                                                    pParse,
                                                    pList,
                                                    (b"index\0".as_ptr() as *mut i8) as *const i8,
                                                )
                                            };
                                            if (unsafe { (*pParse).nErr }) != (0 as i32) {
                                                break '__join_9;
                                            }
                                        }
                                        // Figure out how many bytes of space are required to store explicitly
                                        // specified collation sequence names.
                                        *__slate_slot_1165 = 0 as i32;
                                        loop {
                                            if *__slate_slot_1165 < unsafe { (*pList).nExpr } {
                                                std::ptr::write(__slate_slot_1183, unsafe {
                                                    (*unsafe {
                                                        unsafe {
                                                            std::ptr::addr_of_mut!((*pList).a)
                                                                as *mut ExprList_item
                                                        }
                                                        .offset(*__slate_slot_1165 as isize)
                                                    })
                                                    .pExpr
                                                });
                                                0 as i32;
                                                if (((unsafe { (*(*__slate_slot_1183)).op }) as u32)
                                                    as i32)
                                                    == (114 as i32)
                                                {
                                                    0 as i32;
                                                    std::ptr::write(
                                                        __slate_slot_2526,
                                                        *__slate_slot_1174,
                                                    );
                                                    std::ptr::write(
                                                        __slate_slot_2527,
                                                        *__slate_slot_2526
                                                            + ((1 as i32)
                                                                + unsafe {
                                                                    sqlite3Strlen30(
                                                                        (unsafe {
                                                                            (*(*__slate_slot_1183))
                                                                                .u
                                                                                .zToken
                                                                        })
                                                                            as *const i8,
                                                                    )
                                                                }),
                                                    );
                                                    *__slate_slot_1174 = *__slate_slot_2527;
                                                }
                                                std::ptr::write(
                                                    __slate_slot_2524,
                                                    *__slate_slot_1165,
                                                );
                                                std::ptr::write(
                                                    __slate_slot_2525,
                                                    *__slate_slot_2524 + (1 as i32),
                                                );
                                                *__slate_slot_1165 = *__slate_slot_2525;
                                            } else {
                                                break;
                                            }
                                        }
                                        // Allocate the index structure.
                                        *__slate_slot_1164 = unsafe {
                                            sqlite3Strlen30(*__slate_slot_1163 as *const i8)
                                        };
                                        *__slate_slot_1175 = if *__slate_slot_1177
                                            != std::ptr::null_mut::<Index>()
                                        {
                                            ((unsafe { (*(*__slate_slot_1177)).nKeyCol }) as u32)
                                                as i32
                                        } else {
                                            1 as i32
                                        };
                                        0 as i32; // Fits in i16
                                        *__slate_slot_1162 = sqlite3AllocateIndexObject(
                                            *__slate_slot_1169,
                                            (unsafe { (*pList).nExpr }) + *__slate_slot_1175,
                                            *__slate_slot_1164 + *__slate_slot_1174 + (1 as i32),
                                            std::ptr::addr_of_mut!(*__slate_slot_1176),
                                        );
                                        if (unsafe { (*(*__slate_slot_1169)).mallocFailed })
                                            != (0 as u8)
                                        {
                                        } else {
                                            0 as i32;
                                            0 as i32;
                                            unsafe {
                                                (*(*__slate_slot_1162)).zName = *__slate_slot_1176;
                                            }
                                            std::ptr::write(__slate_slot_2528, *__slate_slot_1176);
                                            std::ptr::write(__slate_slot_2529, unsafe {
                                                (*__slate_slot_2528).offset(
                                                    (*__slate_slot_1164 + (1 as i32)) as isize,
                                                )
                                            });
                                            *__slate_slot_1176 = *__slate_slot_2529;
                                            unsafe {
                                                memcpy(
                                                    (unsafe { (*(*__slate_slot_1162)).zName })
                                                        as *mut (),
                                                    *__slate_slot_1163 as *const (),
                                                    ((*__slate_slot_1164 + (1 as i32)) as i64)
                                                        as u64,
                                                )
                                            };
                                            unsafe {
                                                (*(*__slate_slot_1162)).pTable = *__slate_slot_1161;
                                            }
                                            unsafe {
                                                (*(*__slate_slot_1162)).onError =
                                                    (onError as i8) as u8;
                                            }
                                            unsafe {
                                                (*(*__slate_slot_1162))
                                                    .__slate_bits_0
                                                    .__set_uniqNotNull(
                                                        (onError != (0 as i32)) as u32,
                                                    );
                                            }
                                            unsafe {
                                                (*(*__slate_slot_1162))
                                                    .__slate_bits_0
                                                    .__set_idxType(idxType as u32);
                                            }
                                            unsafe {
                                                (*(*__slate_slot_1162)).pSchema = unsafe {
                                                    (*unsafe {
                                                        unsafe { (*(*__slate_slot_1169)).aDb }
                                                            .offset(*__slate_slot_1171 as isize)
                                                    })
                                                    .pSchema
                                                };
                                            }
                                            unsafe {
                                                (*(*__slate_slot_1162)).nKeyCol =
                                                    ((unsafe { (*pList).nExpr }) as i16) as u16;
                                            }
                                            if pPIWhere != std::ptr::null_mut::<Expr>() {
                                                unsafe {
                                                    sqlite3ResolveSelfReference(
                                                        pParse,
                                                        *__slate_slot_1161,
                                                        2 as i32,
                                                        pPIWhere,
                                                        std::ptr::null_mut::<ExprList>(),
                                                    )
                                                };
                                                unsafe {
                                                    (*(*__slate_slot_1162)).pPartIdxWhere =
                                                        pPIWhere;
                                                }
                                                pPIWhere = std::ptr::null_mut::<Expr>();
                                            }
                                            0 as i32;
                                            // Check to see if we should honor DESC requests on index columns
                                            if (((unsafe {
                                                (*unsafe { (*(*__slate_slot_1170)).pSchema })
                                                    .file_format
                                            })
                                                as u32)
                                                as i32)
                                                >= (4 as i32)
                                            {
                                                *__slate_slot_1168 = -(1 as i32); // Honor DESC
                                            } else {
                                                *__slate_slot_1168 = 0 as i32; // Ignore DESC
                                            }
                                            // Analyze the list of expressions that form the terms of the index and
                                            // report any errors.  In the common case where the expression is exactly
                                            // a table column, store that column in aiColumn[].  For general expressions,
                                            // populate pIndex->aColExpr and store XN_EXPR (-2) in aiColumn[].
                                            //
                                            // TODO: Issue a warning if two or more columns of the index are identical.
                                            // TODO: Issue a warning if the table primary key is used as part of the
                                            // index key.
                                            *__slate_slot_1173 = unsafe {
                                                std::ptr::addr_of_mut!((*pList).a)
                                                    as *mut ExprList_item
                                            };
                                            if (((unsafe { (*pParse).eParseMode }) as u32) as i32)
                                                >= (2 as i32)
                                            {
                                                unsafe {
                                                    (*(*__slate_slot_1162)).aColExpr = pList;
                                                }
                                                pList = std::ptr::null_mut::<ExprList>();
                                            }
                                            *__slate_slot_1165 = 0 as i32;
                                            '__join_87: {
                                                loop {
                                                    if *__slate_slot_1165
                                                        < (((unsafe {
                                                            (*(*__slate_slot_1162)).nKeyCol
                                                        })
                                                            as u32)
                                                            as i32)
                                                    {
                                                        sqlite3StringToId(unsafe {
                                                            (*(*__slate_slot_1173)).pExpr
                                                        });
                                                        unsafe {
                                                            sqlite3ResolveSelfReference(
                                                                pParse,
                                                                *__slate_slot_1161,
                                                                32 as i32,
                                                                unsafe {
                                                                    (*(*__slate_slot_1173)).pExpr
                                                                },
                                                                std::ptr::null_mut::<ExprList>(),
                                                            )
                                                        };
                                                        if (unsafe { (*pParse).nErr }) != (0 as i32)
                                                        {
                                                            break '__join_9;
                                                        } else {
                                                            *__slate_slot_1184 = unsafe {
                                                                sqlite3ExprSkipCollate(unsafe {
                                                                    (*(*__slate_slot_1173)).pExpr
                                                                })
                                                            };
                                                            if (((unsafe {
                                                                (*(*__slate_slot_1184)).op
                                                            })
                                                                as u32)
                                                                as i32)
                                                                != (168 as i32)
                                                            {
                                                                if *__slate_slot_1161
                                                                    == unsafe {
                                                                        (*pParse).pNewTable
                                                                    }
                                                                {
                                                                    break '__join_87;
                                                                } else {
                                                                    if (unsafe {
                                                                        (*(*__slate_slot_1162))
                                                                            .aColExpr
                                                                    }) == std::ptr::null_mut::<
                                                                        ExprList,
                                                                    >(
                                                                    ) {
                                                                        unsafe {
                                                                            (*(*__slate_slot_1162)).aColExpr = pList;
                                                                        }
                                                                        pList = std::ptr::null_mut::<
                                                                            ExprList,
                                                                        >(
                                                                        );
                                                                    }
                                                                    *__slate_slot_1166 =
                                                                        -(2 as i32);
                                                                    unsafe {
                                                                        *unsafe {
                                                                            unsafe { (*(*__slate_slot_1162)).aiColumn }.offset(*__slate_slot_1165 as isize)
                                                                        } = -(2 as i32) as i16;
                                                                    }
                                                                    unsafe {
                                                                        (*(*__slate_slot_1162))
                                                                            .__slate_bits_0
                                                                            .__set_uniqNotNull(
                                                                                (0 as i32) as u32,
                                                                            );
                                                                    }
                                                                    unsafe {
                                                                        (*(*__slate_slot_1162))
                                                                            .__slate_bits_0
                                                                            .__set_bHasExpr(
                                                                                (1 as i32) as u32,
                                                                            );
                                                                    }
                                                                }
                                                            } else {
                                                                *__slate_slot_1166 = (unsafe {
                                                                    (*(*__slate_slot_1184)).iColumn
                                                                })
                                                                    as i32;
                                                                0 as i32;
                                                                if *__slate_slot_1166 < (0 as i32) {
                                                                    *__slate_slot_1166 = (unsafe {
                                                                        (*(*__slate_slot_1161))
                                                                            .iPKey
                                                                    })
                                                                        as i32;
                                                                } else {
                                                                    if ((unsafe {
                                                                        (*unsafe { unsafe { (*(*__slate_slot_1161)).aCol }.offset(*__slate_slot_1166 as isize) }).__slate_bits_0.__get_notNull()
                                                                    })
                                                                        as i32)
                                                                        == (0 as i32)
                                                                    {
                                                                        unsafe {
                                                                            (*(*__slate_slot_1162))
                                                                                .__slate_bits_0
                                                                                .__set_uniqNotNull(
                                                                                    (0 as i32)
                                                                                        as u32,
                                                                                );
                                                                        }
                                                                    }
                                                                    if (((unsafe {
                                                                        (*unsafe { unsafe { (*(*__slate_slot_1161)).aCol }.offset(*__slate_slot_1166 as isize) }).colFlags
                                                                    })
                                                                        as u32)
                                                                        as i32)
                                                                        & (32 as i32)
                                                                        != (0 as i32)
                                                                    {
                                                                        unsafe {
                                                                            (*(*__slate_slot_1162))
                                                                                .__slate_bits_0
                                                                                .__set_bHasVCol(
                                                                                    (1 as i32)
                                                                                        as u32,
                                                                                );
                                                                        }
                                                                        unsafe {
                                                                            (*(*__slate_slot_1162))
                                                                                .__slate_bits_0
                                                                                .__set_bHasExpr(
                                                                                    (1 as i32)
                                                                                        as u32,
                                                                                );
                                                                        }
                                                                    }
                                                                }
                                                                unsafe {
                                                                    *unsafe {
                                                                        unsafe {
                                                                            (*(*__slate_slot_1162))
                                                                                .aiColumn
                                                                        }
                                                                        .offset(
                                                                            *__slate_slot_1165
                                                                                as isize,
                                                                        )
                                                                    } = *__slate_slot_1166 as i16;
                                                                }
                                                            }
                                                            *__slate_slot_1186 =
                                                                std::ptr::null::<i8>();
                                                            if (((unsafe {
                                                                (*unsafe {
                                                                    (*(*__slate_slot_1173)).pExpr
                                                                })
                                                                .op
                                                            })
                                                                as u32)
                                                                as i32)
                                                                == (114 as i32)
                                                            {
                                                                0 as i32;
                                                                *__slate_slot_1186 = (unsafe {
                                                                    (*unsafe {
                                                                        (*(*__slate_slot_1173))
                                                                            .pExpr
                                                                    })
                                                                    .u
                                                                    .zToken
                                                                })
                                                                    as *const i8;
                                                                *__slate_slot_1187 = (unsafe {
                                                                    sqlite3Strlen30(
                                                                        *__slate_slot_1186,
                                                                    )
                                                                }) + (1
                                                                    as i32);
                                                                0 as i32;
                                                                unsafe {
                                                                    memcpy(
                                                                        *__slate_slot_1176
                                                                            as *mut (),
                                                                        *__slate_slot_1186
                                                                            as *const (),
                                                                        (*__slate_slot_1187 as i64)
                                                                            as u64,
                                                                    )
                                                                };
                                                                *__slate_slot_1186 =
                                                                    *__slate_slot_1176 as *const i8;
                                                                std::ptr::write(
                                                                    __slate_slot_2534,
                                                                    *__slate_slot_1176,
                                                                );
                                                                std::ptr::write(
                                                                    __slate_slot_2535,
                                                                    unsafe {
                                                                        (*__slate_slot_2534).offset(
                                                                            *__slate_slot_1187
                                                                                as isize,
                                                                        )
                                                                    },
                                                                );
                                                                *__slate_slot_1176 =
                                                                    *__slate_slot_2535;
                                                                std::ptr::write(
                                                                    __slate_slot_2536,
                                                                    *__slate_slot_1174,
                                                                );
                                                                std::ptr::write(
                                                                    __slate_slot_2537,
                                                                    *__slate_slot_2536
                                                                        - *__slate_slot_1187,
                                                                );
                                                                *__slate_slot_1174 =
                                                                    *__slate_slot_2537;
                                                            } else {
                                                                if *__slate_slot_1166 >= (0 as i32)
                                                                {
                                                                    *__slate_slot_1186 =
                                                                        sqlite3ColumnColl(unsafe {
                                                                            unsafe { (*(*__slate_slot_1161)).aCol }.offset(*__slate_slot_1166 as isize)
                                                                        });
                                                                }
                                                            }
                                                            if !(*__slate_slot_1186
                                                                != std::ptr::null::<i8>())
                                                            {
                                                                *__slate_slot_1186 = unsafe {
                                                                    std::ptr::addr_of!(
                                                                        sqlite3StrBINARY
                                                                    )
                                                                        as *const i8
                                                                };
                                                            }
                                                            if !((unsafe {
                                                                (*(*__slate_slot_1169)).init.busy
                                                            }) != (0 as u8))
                                                            {
                                                                *__slate_slot_2538 = !((unsafe {
                                                                    sqlite3LocateCollSeq(
                                                                        pParse,
                                                                        *__slate_slot_1186,
                                                                    )
                                                                })
                                                                    != std::ptr::null_mut::<CollSeq>(
                                                                    ));
                                                            } else {
                                                                *__slate_slot_2538 = false as bool;
                                                            }
                                                            if *__slate_slot_2538 {
                                                                break '__join_9;
                                                            } else {
                                                                unsafe {
                                                                    *unsafe {
                                                                        unsafe {
                                                                            (*(*__slate_slot_1162))
                                                                                .azColl
                                                                        }
                                                                        .offset(
                                                                            *__slate_slot_1165
                                                                                as isize,
                                                                        )
                                                                    } = *__slate_slot_1186;
                                                                }
                                                                *__slate_slot_1185 = (((unsafe {
                                                                    (*(*__slate_slot_1173))
                                                                        .fg
                                                                        .sortFlags
                                                                })
                                                                    as u32)
                                                                    as i32)
                                                                    & *__slate_slot_1168;
                                                                unsafe {
                                                                    *unsafe {
                                                                        unsafe {
                                                                            (*(*__slate_slot_1162))
                                                                                .aSortOrder
                                                                        }
                                                                        .offset(
                                                                            *__slate_slot_1165
                                                                                as isize,
                                                                        )
                                                                    } = (*__slate_slot_1185 as i8)
                                                                        as u8;
                                                                }
                                                                std::ptr::write(
                                                                    __slate_slot_2530,
                                                                    *__slate_slot_1165,
                                                                );
                                                                std::ptr::write(
                                                                    __slate_slot_2531,
                                                                    *__slate_slot_2530 + (1 as i32),
                                                                );
                                                                *__slate_slot_1165 =
                                                                    *__slate_slot_2531;
                                                                std::ptr::write(
                                                                    __slate_slot_2532,
                                                                    *__slate_slot_1173,
                                                                );
                                                                std::ptr::write(
                                                                    __slate_slot_2533,
                                                                    unsafe {
                                                                        (*__slate_slot_2532).offset(
                                                                            (1 as i32) as isize,
                                                                        )
                                                                    },
                                                                );
                                                                *__slate_slot_1173 =
                                                                    *__slate_slot_2533;
                                                            }
                                                        }
                                                    } else {
                                                        break;
                                                    }
                                                }
                                                // Append the table key to the end of the index.  For WITHOUT ROWID
                                                // tables (when pPk!=0) this will be the declared PRIMARY KEY.  For
                                                // normal tables (when pPk==0) this will be the rowid.
                                                if *__slate_slot_1177
                                                    != std::ptr::null_mut::<Index>()
                                                {
                                                    *__slate_slot_1166 = 0 as i32;
                                                    loop {
                                                        if *__slate_slot_1166
                                                            < (((unsafe {
                                                                (*(*__slate_slot_1177)).nKeyCol
                                                            })
                                                                as u32)
                                                                as i32)
                                                        {
                                                            std::ptr::write(
                                                                __slate_slot_1188,
                                                                (unsafe {
                                                                    *unsafe {
                                                                        unsafe {
                                                                            (*(*__slate_slot_1177))
                                                                                .aiColumn
                                                                        }
                                                                        .offset(
                                                                            *__slate_slot_1166
                                                                                as isize,
                                                                        )
                                                                    }
                                                                })
                                                                    as i32,
                                                            );
                                                            0 as i32;
                                                            if isDupColumn(
                                                                *__slate_slot_1162,
                                                                ((unsafe {
                                                                    (*(*__slate_slot_1162)).nKeyCol
                                                                })
                                                                    as u32)
                                                                    as i32,
                                                                *__slate_slot_1177,
                                                                *__slate_slot_1166,
                                                            ) != (0 as i32)
                                                            {
                                                                std::ptr::write(
                                                                    __slate_slot_2541,
                                                                    *__slate_slot_1162,
                                                                );
                                                                std::ptr::write(
                                                                    __slate_slot_2542,
                                                                    unsafe {
                                                                        (*(*__slate_slot_2541))
                                                                            .nColumn
                                                                    },
                                                                );
                                                                std::ptr::write(
                                                                    __slate_slot_2543,
                                                                    ((((*__slate_slot_2542 as u32)
                                                                        as i32)
                                                                        - (1 as i32))
                                                                        as i16)
                                                                        as u16,
                                                                );
                                                                unsafe {
                                                                    (*(*__slate_slot_2541))
                                                                        .nColumn =
                                                                        *__slate_slot_2543;
                                                                }
                                                            } else {
                                                                {}
                                                                unsafe {
                                                                    *unsafe {
                                                                        unsafe {
                                                                            (*(*__slate_slot_1162))
                                                                                .aiColumn
                                                                        }
                                                                        .offset(
                                                                            *__slate_slot_1165
                                                                                as isize,
                                                                        )
                                                                    } = *__slate_slot_1188 as i16;
                                                                }
                                                                unsafe {
                                                                    *unsafe {
                                                                        unsafe {
                                                                            (*(*__slate_slot_1162))
                                                                                .azColl
                                                                        }
                                                                        .offset(
                                                                            *__slate_slot_1165
                                                                                as isize,
                                                                        )
                                                                    } = unsafe {
                                                                        *unsafe {
                                                                            unsafe { (*(*__slate_slot_1177)).azColl }.offset(*__slate_slot_1166 as isize)
                                                                        }
                                                                    };
                                                                }
                                                                unsafe {
                                                                    *unsafe {
                                                                        unsafe {
                                                                            (*(*__slate_slot_1162))
                                                                                .aSortOrder
                                                                        }
                                                                        .offset(
                                                                            *__slate_slot_1165
                                                                                as isize,
                                                                        )
                                                                    } = unsafe {
                                                                        *unsafe {
                                                                            unsafe { (*(*__slate_slot_1177)).aSortOrder }.offset(*__slate_slot_1166 as isize)
                                                                        }
                                                                    };
                                                                }
                                                                std::ptr::write(
                                                                    __slate_slot_2544,
                                                                    *__slate_slot_1165,
                                                                );
                                                                std::ptr::write(
                                                                    __slate_slot_2545,
                                                                    *__slate_slot_2544 + (1 as i32),
                                                                );
                                                                *__slate_slot_1165 =
                                                                    *__slate_slot_2545;
                                                            }
                                                            std::ptr::write(
                                                                __slate_slot_2539,
                                                                *__slate_slot_1166,
                                                            );
                                                            std::ptr::write(
                                                                __slate_slot_2540,
                                                                *__slate_slot_2539 + (1 as i32),
                                                            );
                                                            *__slate_slot_1166 = *__slate_slot_2540;
                                                        } else {
                                                            break;
                                                        }
                                                    }
                                                    0 as i32;
                                                } else {
                                                    unsafe {
                                                        *unsafe {
                                                            unsafe {
                                                                (*(*__slate_slot_1162)).aiColumn
                                                            }
                                                            .offset(*__slate_slot_1165 as isize)
                                                        } = -(1 as i32) as i16;
                                                    }
                                                    unsafe {
                                                        *unsafe {
                                                            unsafe {
                                                                (*(*__slate_slot_1162)).azColl
                                                            }
                                                            .offset(*__slate_slot_1165 as isize)
                                                        } = unsafe {
                                                            std::ptr::addr_of!(sqlite3StrBINARY)
                                                                as *const i8
                                                        };
                                                    }
                                                }
                                                sqlite3DefaultRowEst(*__slate_slot_1162);
                                                if (unsafe { (*pParse).pNewTable })
                                                    == std::ptr::null_mut::<Table>()
                                                {
                                                    estimateIndexWidth(*__slate_slot_1162);
                                                }
                                                '__join_53: {
                                                    // If this index contains every column of its table, then mark
                                                    // it as a covering index
                                                    0 as i32;
                                                    recomputeColumnsNotIndexed(*__slate_slot_1162);
                                                    if pTblName != std::ptr::null_mut::<SrcList>()
                                                        && (((unsafe {
                                                            (*(*__slate_slot_1162)).nColumn
                                                        })
                                                            as u32)
                                                            as i32)
                                                            >= ((unsafe {
                                                                (*(*__slate_slot_1161)).nCol
                                                            })
                                                                as i32)
                                                    {
                                                        unsafe {
                                                            (*(*__slate_slot_1162))
                                                                .__slate_bits_0
                                                                .__set_isCovering(
                                                                    (1 as i32) as u32,
                                                                );
                                                        }
                                                        *__slate_slot_1166 = 0 as i32;
                                                        '__loop_54: loop {
                                                            if *__slate_slot_1166
                                                                < ((unsafe {
                                                                    (*(*__slate_slot_1161)).nCol
                                                                })
                                                                    as i32)
                                                            {
                                                                '__join_55: {
                                                                    if *__slate_slot_1166
                                                                        == ((unsafe {
                                                                            (*(*__slate_slot_1161))
                                                                                .iPKey
                                                                        })
                                                                            as i32)
                                                                    {
                                                                    } else {
                                                                        if sqlite3TableColumnToIndex(
                                                                            *__slate_slot_1162,
                                                                            *__slate_slot_1166,
                                                                        ) >= (0 as i32)
                                                                        {
                                                                        } else {
                                                                            break '__loop_54;
                                                                        }
                                                                    }
                                                                }
                                                                std::ptr::write(
                                                                    __slate_slot_2546,
                                                                    *__slate_slot_1166,
                                                                );
                                                                std::ptr::write(
                                                                    __slate_slot_2547,
                                                                    *__slate_slot_2546 + (1 as i32),
                                                                );
                                                                *__slate_slot_1166 =
                                                                    *__slate_slot_2547;
                                                            } else {
                                                                break '__join_53;
                                                            }
                                                        }
                                                        unsafe {
                                                            (*(*__slate_slot_1162))
                                                                .__slate_bits_0
                                                                .__set_isCovering(
                                                                    (0 as i32) as u32,
                                                                );
                                                        }
                                                    }
                                                }
                                                '__join_33: {
                                                    if *__slate_slot_1161
                                                        == unsafe { (*pParse).pNewTable }
                                                    {
                                                        *__slate_slot_1189 = unsafe {
                                                            (*(*__slate_slot_1161)).pIndex
                                                        };
                                                        '__loop_34: loop {
                                                            if *__slate_slot_1189
                                                                != std::ptr::null_mut::<Index>()
                                                            {
                                                                0 as i32;
                                                                0 as i32;
                                                                0 as i32;
                                                                if (((unsafe {
                                                                    (*(*__slate_slot_1189)).nKeyCol
                                                                })
                                                                    as u32)
                                                                    as i32)
                                                                    != (((unsafe {
                                                                        (*(*__slate_slot_1162))
                                                                            .nKeyCol
                                                                    })
                                                                        as u32)
                                                                        as i32)
                                                                {
                                                                } else {
                                                                    *__slate_slot_1190 = 0 as i32;
                                                                    loop {
                                                                        if *__slate_slot_1190
                                                                            < (((unsafe {
                                                                                (*(*__slate_slot_1189)).nKeyCol
                                                                            })
                                                                                as u32)
                                                                                as i32)
                                                                        {
                                                                            0 as i32;
                                                                            if ((unsafe {
                                                                                *unsafe {
                                                                                    unsafe { (*(*__slate_slot_1189)).aiColumn }.offset(*__slate_slot_1190 as isize)
                                                                                }
                                                                            })
                                                                                as i32)
                                                                                != ((unsafe {
                                                                                    *unsafe {
                                                                                        unsafe { (*(*__slate_slot_1162)).aiColumn }.offset(*__slate_slot_1190 as isize)
                                                                                    }
                                                                                })
                                                                                    as i32)
                                                                            {
                                                                                break;
                                                                            } else {
                                                                                *__slate_slot_1191 = unsafe { *unsafe { unsafe { (*(*__slate_slot_1189)).azColl }.offset(*__slate_slot_1190 as isize) } };
                                                                                *__slate_slot_1192 = unsafe { *unsafe { unsafe { (*(*__slate_slot_1162)).azColl }.offset(*__slate_slot_1190 as isize) } };
                                                                                if (unsafe {
                                                                                    sqlite3StrICmp(*__slate_slot_1191, *__slate_slot_1192)
                                                                                }) != (0 as i32)
                                                                                {
                                                                                    break;
                                                                                } else {
                                                                                    std::ptr::write(__slate_slot_2548, *__slate_slot_1190);
                                                                                    std::ptr::write(__slate_slot_2549, *__slate_slot_2548 + (1 as i32));
                                                                                    *__slate_slot_1190 = *__slate_slot_2549;
                                                                                }
                                                                            }
                                                                        } else {
                                                                            break;
                                                                        }
                                                                    }
                                                                    if *__slate_slot_1190
                                                                        == (((unsafe {
                                                                            (*(*__slate_slot_1189))
                                                                                .nKeyCol
                                                                        })
                                                                            as u32)
                                                                            as i32)
                                                                    {
                                                                        break '__loop_34;
                                                                    }
                                                                }
                                                                *__slate_slot_1189 = unsafe {
                                                                    (*(*__slate_slot_1189)).pNext
                                                                };
                                                            } else {
                                                                break '__join_33;
                                                            }
                                                        }
                                                        if (((unsafe {
                                                            (*(*__slate_slot_1189)).onError
                                                        })
                                                            as u32)
                                                            as i32)
                                                            != (((unsafe {
                                                                (*(*__slate_slot_1162)).onError
                                                            })
                                                                as u32)
                                                                as i32)
                                                        {
                                                            // This constraint creates the same index as a previous
                                                            // constraint specified somewhere in the CREATE TABLE statement.
                                                            // However the ON CONFLICT clauses are different. If both this
                                                            // constraint and the previous equivalent constraint have explicit
                                                            // ON CONFLICT clauses this is an error. Otherwise, use the
                                                            // explicitly specified behavior for the index.
                                                            if !((((unsafe {
                                                                (*(*__slate_slot_1189)).onError
                                                            })
                                                                as u32)
                                                                as i32)
                                                                == (11 as i32)
                                                                || (((unsafe {
                                                                    (*(*__slate_slot_1162)).onError
                                                                })
                                                                    as u32)
                                                                    as i32)
                                                                    == (11 as i32))
                                                            {
                                                                unsafe {
                                                                    sqlite3ErrorMsg(pParse, (b"conflicting ON CONFLICT clauses specified\0".as_ptr() as *mut i8) as *const i8, 0 as i32)
                                                                };
                                                            }
                                                            if (((unsafe {
                                                                (*(*__slate_slot_1189)).onError
                                                            })
                                                                as u32)
                                                                as i32)
                                                                == (11 as i32)
                                                            {
                                                                unsafe {
                                                                    (*(*__slate_slot_1189))
                                                                        .onError = unsafe {
                                                                        (*(*__slate_slot_1162))
                                                                            .onError
                                                                    };
                                                                }
                                                            }
                                                        }
                                                        if ((idxType as u32) as i32) == (2 as i32) {
                                                            unsafe {
                                                                (*(*__slate_slot_1189))
                                                                    .__slate_bits_0
                                                                    .__set_idxType(idxType as u32);
                                                            }
                                                        }
                                                        if (((unsafe { (*pParse).eParseMode })
                                                            as u32)
                                                            as i32)
                                                            >= (2 as i32)
                                                        {
                                                            unsafe {
                                                                (*(*__slate_slot_1162)).pNext =
                                                                    unsafe { (*pParse).pNewIndex };
                                                            }
                                                            unsafe {
                                                                (*pParse).pNewIndex =
                                                                    *__slate_slot_1162;
                                                            }
                                                            *__slate_slot_1162 =
                                                                std::ptr::null_mut::<Index>();
                                                            break '__join_9;
                                                        } else {
                                                            break '__join_9;
                                                        }
                                                    }
                                                }
                                                if !((((unsafe { (*pParse).eParseMode }) as u32)
                                                    as i32)
                                                    >= (2 as i32))
                                                {
                                                    // Link the new Index structure to its table and to the other
                                                    // in-memory database structures.
                                                    0 as i32;
                                                    if (unsafe {
                                                        (*(*__slate_slot_1169)).init.busy
                                                    }) != (0 as u8)
                                                    {
                                                        0 as i32;
                                                        0 as i32;
                                                        if pTblName
                                                            != std::ptr::null_mut::<SrcList>()
                                                        {
                                                            unsafe {
                                                                (*(*__slate_slot_1162)).tnum = unsafe {
                                                                    (*(*__slate_slot_1169))
                                                                        .init
                                                                        .newTnum
                                                                };
                                                            }
                                                            if (unsafe {
                                                                sqlite3IndexHasDuplicateRootPage(
                                                                    *__slate_slot_1162,
                                                                )
                                                            }) != (0 as i32)
                                                            {
                                                                unsafe {
                                                                    sqlite3ErrorMsg(
                                                                        pParse,
                                                                        (b"invalid rootpage\0"
                                                                            .as_ptr()
                                                                            as *mut i8)
                                                                            as *const i8,
                                                                    )
                                                                };
                                                                unsafe {
                                                                    (*pParse).rc = unsafe {
                                                                        sqlite3CorruptError(
                                                                            4430 as i32,
                                                                        )
                                                                    };
                                                                }
                                                                break '__join_9;
                                                            }
                                                        }
                                                        *__slate_slot_1193 = (unsafe {
                                                            sqlite3HashInsert(
                                                                unsafe {
                                                                    std::ptr::addr_of_mut!(
                                                                        (*unsafe {
                                                                            (*(*__slate_slot_1162))
                                                                                .pSchema
                                                                        })
                                                                        .idxHash
                                                                    )
                                                                },
                                                                (unsafe {
                                                                    (*(*__slate_slot_1162)).zName
                                                                })
                                                                    as *const i8,
                                                                *__slate_slot_1162 as *mut (),
                                                            )
                                                        })
                                                            as *mut Index;
                                                        if *__slate_slot_1193
                                                            != std::ptr::null_mut::<Index>()
                                                        {
                                                            0 as i32; // Malloc must have failed
                                                            unsafe {
                                                                sqlite3OomFault(*__slate_slot_1169)
                                                            };
                                                            break '__join_9;
                                                        } else {
                                                            std::ptr::write(
                                                                __slate_slot_2550,
                                                                *__slate_slot_1169,
                                                            );
                                                            std::ptr::write(
                                                                __slate_slot_2551,
                                                                unsafe {
                                                                    (*(*__slate_slot_2550)).mDbFlags
                                                                },
                                                            );
                                                            std::ptr::write(
                                                                __slate_slot_2552,
                                                                *__slate_slot_2551
                                                                    | ((1 as i32) as u32),
                                                            );
                                                            unsafe {
                                                                (*(*__slate_slot_2550)).mDbFlags =
                                                                    *__slate_slot_2552;
                                                            }
                                                        }
                                                    } else {
                                                        if (unsafe {
                                                            (*(*__slate_slot_1161)).tabFlags
                                                        }) & ((128 as i32) as u32)
                                                            == ((0 as i32) as u32)
                                                            || pTblName
                                                                != std::ptr::null_mut::<SrcList>()
                                                        {
                                                            std::ptr::write(
                                                                __slate_slot_2553,
                                                                pParse,
                                                            );
                                                            std::ptr::write(
                                                                __slate_slot_2554,
                                                                unsafe {
                                                                    (*(*__slate_slot_2553)).nMem
                                                                },
                                                            );
                                                            std::ptr::write(
                                                                __slate_slot_2555,
                                                                *__slate_slot_2554 + (1 as i32),
                                                            );
                                                            unsafe {
                                                                (*(*__slate_slot_2553)).nMem =
                                                                    *__slate_slot_2555;
                                                            }
                                                            *__slate_slot_1196 = *__slate_slot_2555;
                                                            *__slate_slot_1194 =
                                                                unsafe { sqlite3GetVdbe(pParse) };
                                                            if *__slate_slot_1194
                                                                == std::ptr::null_mut::<Vdbe>()
                                                            {
                                                                break '__join_9;
                                                            } else {
                                                                sqlite3BeginWriteOperation(
                                                                    pParse,
                                                                    1 as i32,
                                                                    *__slate_slot_1171,
                                                                );
                                                                // Create the rootpage for the index using CreateIndex. But before
                                                                // doing so, code a Noop instruction and store its address in
                                                                // Index.tnum. This is required in case this index is actually a
                                                                // PRIMARY KEY and the table is actually a WITHOUT ROWID table. In
                                                                // that case the convertToWithoutRowidTable() routine will replace
                                                                // the Noop with a Goto to jump over the VDBE code generated below.
                                                                unsafe {
                                                                    (*(*__slate_slot_1162)).tnum =
                                                                        (unsafe {
                                                                            sqlite3VdbeAddOp0(
                                                                                *__slate_slot_1194,
                                                                                189 as i32,
                                                                            )
                                                                        })
                                                                            as u32;
                                                                }
                                                                unsafe {
                                                                    sqlite3VdbeAddOp3(
                                                                        *__slate_slot_1194,
                                                                        149 as i32,
                                                                        *__slate_slot_1171,
                                                                        *__slate_slot_1196,
                                                                        2 as i32,
                                                                    )
                                                                };
                                                                // Gather the complete text of the CREATE INDEX statement into
                                                                // the zStmt variable
                                                                0 as i32;
                                                                if pStart
                                                                    != std::ptr::null_mut::<Token>()
                                                                {
                                                                    std::ptr::write(__slate_slot_1197, ((((unsafe { unsafe { (*pParse).sLastToken.z }.offset_from((unsafe { (*(*__slate_slot_1172)).z }) as *const i8) }) as i64) as i32) as u32).wrapping_add(unsafe { (*pParse).sLastToken.n }) as i32);
                                                                    if ((unsafe {
                                                                        *unsafe {
                                                                            unsafe { (*(*__slate_slot_1172)).z }.offset((*__slate_slot_1197 - (1 as i32)) as isize)
                                                                        }
                                                                    })
                                                                        as i32)
                                                                        == (59 as i32)
                                                                    {
                                                                        std::ptr::write(
                                                                            __slate_slot_2556,
                                                                            *__slate_slot_1197,
                                                                        );
                                                                        std::ptr::write(
                                                                            __slate_slot_2557,
                                                                            *__slate_slot_2556
                                                                                - (1 as i32),
                                                                        );
                                                                        *__slate_slot_1197 =
                                                                            *__slate_slot_2557;
                                                                    }
                                                                    // A named index with an explicit CREATE INDEX statement
                                                                    *__slate_slot_1195 = unsafe {
                                                                        sqlite3MPrintf(*__slate_slot_1169, (b"CREATE%s INDEX %.*s\0".as_ptr() as *mut i8) as *const i8, if onError == (0 as i32) { b"\0".as_ptr() as *mut i8 } else { b" UNIQUE\0".as_ptr() as *mut i8 }, *__slate_slot_1197, unsafe { (*(*__slate_slot_1172)).z })
                                                                    };
                                                                } else {
                                                                    // An automatic index created by a PRIMARY KEY or UNIQUE constraint
                                                                    //
                                                                    // zStmt = sqlite3MPrintf("");
                                                                    *__slate_slot_1195 =
                                                                        std::ptr::null_mut::<i8>();
                                                                }
                                                                // Add an entry in sqlite_schema for this index
                                                                unsafe {
                                                                    sqlite3NestedParse(pParse, (b"INSERT INTO %Q.sqlite_master VALUES('index',%Q,%Q,#%d,%Q);\0".as_ptr() as *mut i8) as *const i8, unsafe { (*unsafe { unsafe { (*(*__slate_slot_1169)).aDb }.offset(*__slate_slot_1171 as isize) }).zDbSName }, unsafe { (*(*__slate_slot_1162)).zName }, unsafe { (*(*__slate_slot_1161)).zName }, *__slate_slot_1196, *__slate_slot_1195)
                                                                };
                                                                unsafe {
                                                                    sqlite3DbFree(
                                                                        *__slate_slot_1169,
                                                                        *__slate_slot_1195
                                                                            as *mut (),
                                                                    )
                                                                };
                                                                // Fill the index with data and reparse the schema. Code an OP_Expire
                                                                // to invalidate all pre-compiled statements.
                                                                if pTblName
                                                                    != std::ptr::null_mut::<SrcList>(
                                                                    )
                                                                {
                                                                    sqlite3RefillIndex(
                                                                        pParse,
                                                                        *__slate_slot_1162,
                                                                        *__slate_slot_1196,
                                                                    );
                                                                    sqlite3ChangeCookie(
                                                                        pParse,
                                                                        *__slate_slot_1171,
                                                                    );
                                                                    unsafe {
                                                                        sqlite3VdbeAddParseSchemaOp(
                                                                            *__slate_slot_1194,
                                                                            *__slate_slot_1171,
                                                                            unsafe {
                                                                                sqlite3MPrintf(*__slate_slot_1169, (b"name='%q' AND type='index'\0".as_ptr() as *mut i8) as *const i8, unsafe { (*(*__slate_slot_1162)).zName })
                                                                            },
                                                                            ((0 as i32) as i16)
                                                                                as u16,
                                                                        )
                                                                    };
                                                                    unsafe {
                                                                        sqlite3VdbeAddOp2(
                                                                            *__slate_slot_1194,
                                                                            168 as i32,
                                                                            0 as i32,
                                                                            1 as i32,
                                                                        )
                                                                    };
                                                                }
                                                                unsafe {
                                                                    sqlite3VdbeJumpHere(
                                                                        *__slate_slot_1194,
                                                                        (unsafe {
                                                                            (*(*__slate_slot_1162))
                                                                                .tnum
                                                                        })
                                                                            as i32,
                                                                    )
                                                                };
                                                            }
                                                        }
                                                    }
                                                    // If this is the initial CREATE INDEX statement (or CREATE TABLE if the
                                                    // index is an implied index for a UNIQUE or PRIMARY KEY constraint) then
                                                    // emit code to allocate the index rootpage on disk and make an entry for
                                                    // the index in the sqlite_schema table and populate the index with
                                                    // content.  But, do not do this if we are simply reading the sqlite_schema
                                                    // table to parse the schema, or if this index is the PRIMARY KEY index
                                                    // of a WITHOUT ROWID table.
                                                    //
                                                    // If pTblName==0 it means this index is generated as an implied PRIMARY KEY
                                                    // or UNIQUE index in a CREATE TABLE statement.  Since the table
                                                    // has just been created, it contains no data and the index initialization
                                                    // step can be skipped.
                                                }
                                                if (unsafe { (*(*__slate_slot_1169)).init.busy })
                                                    != (0 as u8)
                                                    || pTblName == std::ptr::null_mut::<SrcList>()
                                                {
                                                    unsafe {
                                                        (*(*__slate_slot_1162)).pNext = unsafe {
                                                            (*(*__slate_slot_1161)).pIndex
                                                        };
                                                    }
                                                    unsafe {
                                                        (*(*__slate_slot_1161)).pIndex =
                                                            *__slate_slot_1162;
                                                    }
                                                    *__slate_slot_1162 =
                                                        std::ptr::null_mut::<Index>();
                                                } else {
                                                    if (((unsafe { (*pParse).eParseMode }) as u32)
                                                        as i32)
                                                        >= (2 as i32)
                                                    {
                                                        0 as i32;
                                                        unsafe {
                                                            (*pParse).pNewIndex =
                                                                *__slate_slot_1162;
                                                        }
                                                        *__slate_slot_1162 =
                                                            std::ptr::null_mut::<Index>();
                                                    }
                                                }
                                                // Clean up before exiting
                                                break '__join_9;
                                            }
                                            unsafe {
                                                sqlite3ErrorMsg(pParse, (b"expressions prohibited in PRIMARY KEY and UNIQUE constraints\0".as_ptr() as *mut i8) as *const i8)
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
        if *__slate_slot_1162 != std::ptr::null_mut::<Index>() {
            sqlite3FreeIndex(*__slate_slot_1169, *__slate_slot_1162);
        }
        '__join_0: {
            if *__slate_slot_1161 != std::ptr::null_mut::<Table>() {
                *__slate_slot_1198 =
                    unsafe { std::ptr::addr_of_mut!((*(*__slate_slot_1161)).pIndex) };
                loop {
                    std::ptr::write(__slate_slot_2558, unsafe { *(*__slate_slot_1198) });
                    *__slate_slot_1199 = *__slate_slot_2558;
                    if *__slate_slot_2558 != std::ptr::null_mut::<Index>() {
                        if (((unsafe { (*(*__slate_slot_1199)).onError }) as u32) as i32)
                            != (5 as i32)
                        {
                            *__slate_slot_1198 =
                                unsafe { std::ptr::addr_of_mut!((*(*__slate_slot_1199)).pNext) };
                        } else {
                            break;
                        }
                    } else {
                        break '__join_0;
                    }
                }
                loop {
                    std::ptr::write(__slate_slot_2559, unsafe { (*(*__slate_slot_1199)).pNext });
                    *__slate_slot_1200 = *__slate_slot_2559;
                    if *__slate_slot_2559 != std::ptr::null_mut::<Index>()
                        && (((unsafe { (*(*__slate_slot_1200)).onError }) as u32) as i32)
                            != (5 as i32)
                    {
                        unsafe {
                            *(*__slate_slot_1198) = *__slate_slot_1200;
                        }
                        unsafe {
                            (*(*__slate_slot_1199)).pNext =
                                unsafe { (*(*__slate_slot_1200)).pNext };
                        }
                        unsafe {
                            (*(*__slate_slot_1200)).pNext = *__slate_slot_1199;
                        }
                        *__slate_slot_1198 =
                            unsafe { std::ptr::addr_of_mut!((*(*__slate_slot_1200)).pNext) };
                    } else {
                        break '__join_0;
                    }
                }
            }
        }
        unsafe { sqlite3ExprDelete(*__slate_slot_1169, pPIWhere) };
        unsafe { sqlite3ExprListDelete(*__slate_slot_1169, pList) };
        sqlite3SrcListDelete(*__slate_slot_1169, pTblName);
        unsafe { sqlite3DbFree(*__slate_slot_1169, *__slate_slot_1163 as *mut ()) };
    }
}

/// Fill the Index.aiRowEst[] array with default information - information
/// to be used when we have not run the ANALYZE command.
///
/// aiRowEst[0] is supposed to contain the number of elements in the index.
/// Since we do not know, guess 1 million.  aiRowEst[1] is an estimate of the
/// number of rows in the table that match any particular value of the
/// first column of the index.  aiRowEst[2] is an estimate of the number
/// of rows that match any particular combination of the first 2 columns
/// of the index.  And so forth.  It must always be the case that
///
///           aiRowEst[N]<=aiRowEst[N-1]
///           aiRowEst[N]>=1
///
/// Apart from that, we have little to go on besides intuition as to
/// how aiRowEst[] should be initialized.  The numbers generated here
/// are based on typical values found in actual indices.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3DefaultRowEst(mut pIdx: *mut Index) {
    //                10,  9,  8,  7,  6
    let mut a: *mut i16 = unsafe { (*pIdx).aiRowLogEst };
    let mut x: i16 = 0 as i16;
    let mut nCopy: i32 = if ((((10 as u64) / (2 as u64)) as u32) as i32)
        < (((unsafe { (*pIdx).nKeyCol }) as u32) as i32)
    {
        (((10 as u64) / (2 as u64)) as u32) as i32
    } else {
        ((unsafe { (*pIdx).nKeyCol }) as u32) as i32
    };
    let mut i: i32 = 0 as i32;
    // Indexes with default row estimates should not have stat1 data
    0 as i32;
    // Set the first entry (number of rows in the index) to the estimated
    // number of rows in the table, or half the number of rows in the table
    // for a partial index.
    //
    // 2020-05-27:  If some of the stat data is coming from the sqlite_stat1
    // table but other parts we are having to guess at, then do not let the
    // estimated number of rows in the table be less than 1000 (LogEst 99).
    // Failure to do this can cause the indexes for which we do not have
    // stat1 data to be ignored by the query planner.
    x = unsafe { (*unsafe { (*pIdx).pTable }).nRowLogEst };
    0 as i32;
    if (x as i32) < (99 as i32) {
        x = (99 as i32) as i16;
        unsafe {
            (*unsafe { (*pIdx).pTable }).nRowLogEst = (99 as i32) as i16;
        }
    }
    if (unsafe { (*pIdx).pPartIdxWhere }) != std::ptr::null_mut::<Expr>() {
        let __v2653: i16 = x;
        let __v2654: i16 = ((__v2653 as i32) - (10 as i32)) as i16;
        x = __v2654;
        0 as i32;
    }
    unsafe {
        *unsafe { a.offset((0 as i32) as isize) } = x;
    }
    // Estimate that a[1] is 10, a[2] is 9, a[3] is 8, a[4] is 7, a[5] is
    // 6 and each subsequent value (if any) is 5.
    unsafe {
        memcpy(
            (unsafe { a.offset((1 as i32) as isize) }) as *mut (),
            (unsafe { std::ptr::addr_of!(aVal) as *const i16 }) as *const (),
            ((nCopy as i64) as u64).wrapping_mul(2 as u64),
        )
    };
    i = nCopy + (1 as i32);
    '__slate_break_2216: loop {
        if !(i <= (((unsafe { (*pIdx).nKeyCol }) as u32) as i32)) {
            break;
        }
        unsafe {
            *unsafe { a.offset(i as isize) } = (23 as i32) as i16;
        }
        0 as i32;
        let __v2655: i32 = i;
        let __v2656: i32 = __v2655 + (1 as i32);
        i = __v2656;
    }
    0 as i32;
    if (((unsafe { (*pIdx).onError }) as u32) as i32) != (0 as i32) {
        unsafe {
            *unsafe { a.offset((((unsafe { (*pIdx).nKeyCol }) as u32) as i32) as isize) } =
                (0 as i32) as i16;
        }
    }
}

static mut aVal: [i16; 5] = [
    (33 as i32) as i16,
    (32 as i32) as i16,
    (30 as i32) as i16,
    (28 as i32) as i16,
    (26 as i32) as i16,
];

/// This routine will drop an existing named index.  This routine
/// implements the DROP INDEX statement.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3DropIndex(
    mut pParse: *mut Parse,
    mut pName: *mut SrcList,
    mut ifExists: i32,
) {
    let mut __slate_storage_1218: std::mem::MaybeUninit<*const i8> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1218: *mut *const i8 =
        std::ptr::addr_of_mut!(__slate_storage_1218) as *mut *const i8;
    let mut __slate_storage_1217: std::mem::MaybeUninit<*const i8> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1217: *mut *const i8 =
        std::ptr::addr_of_mut!(__slate_storage_1217) as *mut *const i8;
    let mut __slate_storage_1216: std::mem::MaybeUninit<*mut Table> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1216: *mut *mut Table =
        std::ptr::addr_of_mut!(__slate_storage_1216) as *mut *mut Table;
    let mut __slate_storage_1215: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1215: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1215) as *mut i32;
    let mut __slate_storage_1214: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1214: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1214) as *mut i32;
    let mut __slate_storage_1213: std::mem::MaybeUninit<*mut sqlite3> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1213: *mut *mut sqlite3 =
        std::ptr::addr_of_mut!(__slate_storage_1213) as *mut *mut sqlite3;
    let mut __slate_storage_1212: std::mem::MaybeUninit<*mut Vdbe> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1212: *mut *mut Vdbe =
        std::ptr::addr_of_mut!(__slate_storage_1212) as *mut *mut Vdbe;
    let mut __slate_storage_1211: std::mem::MaybeUninit<*mut Index> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1211: *mut *mut Index =
        std::ptr::addr_of_mut!(__slate_storage_1211) as *mut *mut Index;
    unsafe {
        '__join_0: {
            std::ptr::write(__slate_slot_1213, unsafe { (*pParse).db });
            if (unsafe { (*(*__slate_slot_1213)).mallocFailed }) != (0 as u8) {
            } else {
                0 as i32; // Never called with prior non-OOM errors
                0 as i32;
                0 as i32;
                0 as i32;
                if (0 as i32) != unsafe { sqlite3ReadSchema(pParse) } {
                } else {
                    *__slate_slot_1211 = sqlite3FindIndex(
                        *__slate_slot_1213,
                        (unsafe {
                            (*unsafe {
                                unsafe { std::ptr::addr_of_mut!((*pName).a) as *mut SrcItem }
                                    .offset((0 as i32) as isize)
                            })
                            .zName
                        }) as *const i8,
                        (unsafe {
                            (*unsafe {
                                unsafe { std::ptr::addr_of_mut!((*pName).a) as *mut SrcItem }
                                    .offset((0 as i32) as isize)
                            })
                            .u4
                            .zDatabase
                        }) as *const i8,
                    );
                    if *__slate_slot_1211 == std::ptr::null_mut::<Index>() {
                        if !(ifExists != (0 as i32)) {
                            unsafe {
                                sqlite3ErrorMsg(
                                    pParse,
                                    (b"no such index: %S\0".as_ptr() as *mut i8) as *const i8,
                                    unsafe { std::ptr::addr_of_mut!((*pName).a) as *mut SrcItem },
                                )
                            };
                        } else {
                            sqlite3CodeVerifyNamedSchema(
                                pParse,
                                (unsafe {
                                    (*unsafe {
                                        unsafe {
                                            std::ptr::addr_of_mut!((*pName).a) as *mut SrcItem
                                        }
                                        .offset((0 as i32) as isize)
                                    })
                                    .u4
                                    .zDatabase
                                }) as *const i8,
                            );
                            sqlite3ForceNotReadOnly(pParse);
                        }
                        unsafe {
                            (*pParse)
                                .__slate_bits_0
                                .__set_checkSchema((1 as i32) as u32);
                        }
                    } else {
                        if ((unsafe { (*(*__slate_slot_1211)).__slate_bits_0.__get_idxType() })
                            as i32)
                            != (0 as i32)
                        {
                            unsafe {
                                sqlite3ErrorMsg(pParse, (b"index associated with UNIQUE or PRIMARY KEY constraint cannot be dropped\0".as_ptr() as *mut i8) as *const i8, 0 as i32)
                            };
                        } else {
                            *__slate_slot_1214 = unsafe {
                                sqlite3SchemaToIndex(*__slate_slot_1213, unsafe {
                                    (*(*__slate_slot_1211)).pSchema
                                })
                            };
                            0 as i32;
                            std::ptr::write(__slate_slot_1215, 10 as i32);
                            std::ptr::write(__slate_slot_1216, unsafe {
                                (*(*__slate_slot_1211)).pTable
                            });
                            std::ptr::write(
                                __slate_slot_1217,
                                (unsafe {
                                    (*unsafe {
                                        unsafe { (*(*__slate_slot_1213)).aDb }
                                            .offset(*__slate_slot_1214 as isize)
                                    })
                                    .zDbSName
                                }) as *const i8,
                            );
                            std::ptr::write(
                                __slate_slot_1218,
                                (if !((0 as i32) != (0 as i32)) && *__slate_slot_1214 == (1 as i32)
                                {
                                    b"sqlite_temp_master\0".as_ptr() as *mut i8
                                } else {
                                    b"sqlite_master\0".as_ptr() as *mut i8
                                }) as *const i8,
                            );
                            if (unsafe {
                                sqlite3AuthCheck(
                                    pParse,
                                    9 as i32,
                                    *__slate_slot_1218,
                                    std::ptr::null::<i8>(),
                                    *__slate_slot_1217,
                                )
                            }) != (0 as i32)
                            {
                            } else {
                                if !((0 as i32) != (0 as i32)) && *__slate_slot_1214 == (1 as i32) {
                                    *__slate_slot_1215 = 12 as i32;
                                }
                                if (unsafe {
                                    sqlite3AuthCheck(
                                        pParse,
                                        *__slate_slot_1215,
                                        (unsafe { (*(*__slate_slot_1211)).zName }) as *const i8,
                                        (unsafe { (*(*__slate_slot_1216)).zName }) as *const i8,
                                        *__slate_slot_1217,
                                    )
                                }) != (0 as i32)
                                {
                                } else {
                                    // Generate code to remove the index and from the schema table
                                    *__slate_slot_1212 = unsafe { sqlite3GetVdbe(pParse) };
                                    if *__slate_slot_1212 != std::ptr::null_mut::<Vdbe>() {
                                        sqlite3BeginWriteOperation(
                                            pParse,
                                            1 as i32,
                                            *__slate_slot_1214,
                                        );
                                        unsafe {
                                            sqlite3NestedParse(pParse, (b"DELETE FROM %Q.sqlite_master WHERE name=%Q AND type='index'\0".as_ptr() as *mut i8) as *const i8, unsafe { (*unsafe { unsafe { (*(*__slate_slot_1213)).aDb }.offset(*__slate_slot_1214 as isize) }).zDbSName }, unsafe { (*(*__slate_slot_1211)).zName })
                                        };
                                        sqlite3ClearStatTables(
                                            pParse,
                                            *__slate_slot_1214,
                                            (b"idx\0".as_ptr() as *mut i8) as *const i8,
                                            (unsafe { (*(*__slate_slot_1211)).zName }) as *const i8,
                                        );
                                        sqlite3ChangeCookie(pParse, *__slate_slot_1214);
                                        destroyRootPage(
                                            pParse,
                                            (unsafe { (*(*__slate_slot_1211)).tnum }) as i32,
                                            *__slate_slot_1214,
                                        );
                                        unsafe {
                                            sqlite3VdbeAddOp4(
                                                *__slate_slot_1212,
                                                155 as i32,
                                                *__slate_slot_1214,
                                                0 as i32,
                                                0 as i32,
                                                (unsafe { (*(*__slate_slot_1211)).zName })
                                                    as *const i8,
                                                0 as i32,
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
        sqlite3SrcListDelete(*__slate_slot_1213, pName);
    }
}

/// pArray is a pointer to an array of objects. Each object in the
/// array is szEntry bytes in size. This routine uses sqlite3DbRealloc()
/// to extend the array so that there is space for a new object at the end.
///
/// When this function is called, *pnEntry contains the current size of
/// the array (in entries - so the allocation is ((*pnEntry) * szEntry) bytes
/// in total).
///
/// If the realloc() is successful (i.e. if no OOM condition occurs), the
/// space allocated for the new object is zeroed, *pnEntry updated to
/// reflect the new size of the array and a pointer to the new allocation
/// returned. *pIdx is set to the index of the new array entry in this case.
///
/// Otherwise, if the realloc() fails, *pIdx is set to -1, *pnEntry remains
/// unchanged and a copy of pArray returned.
///
/// # Arguments
///
/// * `db` - Connection to notify of malloc failures
/// * `pArray` - Array of objects.  Might be reallocated
/// * `szEntry` - Size of each object in the array
/// * `pnEntry` - Number of objects currently in use
/// * `pIdx` - Write the index of a new slot here
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3ArrayAllocate(
    mut db: *mut sqlite3,
    mut pArray: *mut (),
    mut szEntry: i32,
    mut pnEntry: *mut i32,
    mut pIdx: *mut i32,
) -> *mut () {
    let mut z: *mut i8 = unsafe { std::mem::zeroed() };
    let mut n: i64 = 0 as i64;
    let __v2460: i32 = unsafe { *pnEntry };
    unsafe {
        *pIdx = __v2460;
    }
    n = __v2460 as i64;
    if n & n - ((1 as i32) as i64) == ((0 as i32) as i64) {
        let mut sz: i64 = if n == ((0 as i32) as i64) {
            (1 as i32) as i64
        } else {
            ((2 as i32) as i64) * n
        };
        let mut pNew: *mut () =
            unsafe { sqlite3DbRealloc(db, pArray, (sz * (szEntry as i64)) as u64) };
        if pNew == std::ptr::null_mut::<()>() {
            unsafe {
                *pIdx = -(1 as i32);
            }
            return pArray;
        }
        pArray = pNew;
    }
    z = pArray as *mut i8;
    unsafe {
        memset(
            (unsafe { z.offset((n * (szEntry as i64)) as isize) }) as *mut (),
            0 as i32,
            (szEntry as i64) as u64,
        )
    };
    let __v2461: *mut i32 = pnEntry;
    let __v2462: i32 = unsafe { *__v2461 };
    let __v2463: i32 = __v2462 + (1 as i32);
    unsafe {
        *__v2461 = __v2463;
    }
    return pArray;
}

/// Append a new element to the given IdList.  Create a new IdList if
/// need be.
///
/// A new IdList is returned, or NULL if malloc() fails.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3IdListAppend(
    mut pParse: *mut Parse,
    mut pList: *mut IdList,
    mut pToken: *mut Token,
) -> *mut IdList {
    let mut db: *mut sqlite3 = unsafe { (*pParse).db };
    let mut i: i32 = 0 as i32;
    if pList == std::ptr::null_mut::<IdList>() {
        pList = (unsafe {
            sqlite3DbMallocZero(
                db,
                (8 as u64).wrapping_add((((1 as i32) as i64) as u64).wrapping_mul(8 as u64)),
            )
        }) as *mut IdList;
        if pList == std::ptr::null_mut::<IdList>() {
            return std::ptr::null_mut::<IdList>();
        }
    } else {
        let mut pNew: *mut IdList = unsafe { std::mem::zeroed() };
        pNew = (unsafe {
            sqlite3DbRealloc(
                db,
                pList as *mut (),
                (8 as u64).wrapping_add(
                    ((((unsafe { (*pList).nId }) + (1 as i32)) as i64) as u64)
                        .wrapping_mul(8 as u64),
                ),
            )
        }) as *mut IdList;
        if pNew == std::ptr::null_mut::<IdList>() {
            sqlite3IdListDelete(db, pList);
            return std::ptr::null_mut::<IdList>();
        }
        pList = pNew;
    }
    let __v2464: *mut IdList = pList;
    let __v2465: i32 = unsafe { (*__v2464).nId };
    let __v2466: i32 = __v2465 + (1 as i32);
    unsafe {
        (*__v2464).nId = __v2466;
    }
    i = __v2465;
    unsafe {
        (*unsafe {
            unsafe { std::ptr::addr_of_mut!((*pList).a) as *mut IdList_item }.offset(i as isize)
        })
        .zName = sqlite3NameFromToken(db, pToken as *const Token);
    }
    if (((unsafe { (*pParse).eParseMode }) as u32) as i32) >= (2 as i32)
        && (unsafe {
            (*unsafe {
                unsafe { std::ptr::addr_of_mut!((*pList).a) as *mut IdList_item }.offset(i as isize)
            })
            .zName
        }) != std::ptr::null_mut::<i8>()
    {
        unsafe {
            sqlite3RenameTokenMap(
                pParse,
                ((unsafe {
                    (*unsafe {
                        unsafe { std::ptr::addr_of_mut!((*pList).a) as *mut IdList_item }
                            .offset(i as isize)
                    })
                    .zName
                }) as *mut ()) as *const (),
                pToken as *const Token,
            )
        };
    }
    return pList;
}

/// Delete an IdList.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3IdListDelete(mut db: *mut sqlite3, mut pList: *mut IdList) {
    let mut i: i32 = 0 as i32;
    0 as i32;
    if pList == std::ptr::null_mut::<IdList>() {
        return;
    }
    i = 0 as i32;
    '__slate_break_2223: loop {
        if !(i < unsafe { (*pList).nId }) {
            break;
        }
        unsafe {
            sqlite3DbFree(
                db,
                (unsafe {
                    (*unsafe {
                        unsafe { std::ptr::addr_of_mut!((*pList).a) as *mut IdList_item }
                            .offset(i as isize)
                    })
                    .zName
                }) as *mut (),
            )
        };
        let __v2502: i32 = i;
        let __v2503: i32 = __v2502 + (1 as i32);
        i = __v2503;
    }
    unsafe { sqlite3DbNNFreeNN(db, pList as *mut ()) };
}

/// Return the index in pList of the identifier named zId.  Return -1
/// if not found.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3IdListIndex(mut pList: *mut IdList, mut zName: *const i8) -> i32 {
    let mut i: i32 = 0 as i32;
    0 as i32;
    i = 0 as i32;
    '__slate_break_2224: loop {
        if !(i < unsafe { (*pList).nId }) {
            break;
        }
        if (unsafe {
            sqlite3StrICmp(
                (unsafe {
                    (*unsafe {
                        unsafe { std::ptr::addr_of_mut!((*pList).a) as *mut IdList_item }
                            .offset(i as isize)
                    })
                    .zName
                }) as *const i8,
                zName,
            )
        }) == (0 as i32)
        {
            return i;
        }
        let __v2467: i32 = i;
        let __v2468: i32 = __v2467 + (1 as i32);
        i = __v2468;
    }
    return -(1 as i32);
}

// Maximum size of a SrcList object.
// The SrcList object is used to represent the FROM clause of a
// SELECT statement, and the query planner cannot deal with more
// than 64 tables in a join.  So any value larger than 64 here
// is sufficient for most uses.  Smaller values, like say 10, are
// appropriate for small and memory-limited applications.
/// Expand the space allocated for the given SrcList object by
/// creating nExtra new slots beginning at iStart.  iStart is zero based.
/// New slots are zeroed.
///
/// For example, suppose a SrcList initially contains two entries: A,B.
/// To append 3 new entries onto the end, do this:
///
///    sqlite3SrcListEnlarge(db, pSrclist, 3, 2);
///
/// After the call above it would contain:  A, B, nil, nil, nil.
/// If the iStart argument had been 1 instead of 2, then the result
/// would have been:  A, nil, nil, nil, B.  To prepend the new slots,
/// the iStart value would be 0.  The result then would
/// be: nil, nil, nil, A, B.
///
/// If a memory allocation fails or the SrcList becomes too large, leave
/// the original SrcList unchanged, return NULL, and leave an error message
/// in pParse.
///
/// # Arguments
///
/// * `pParse` - Parsing context into which errors are reported
/// * `pSrc` - The SrcList to be enlarged
/// * `nExtra` - Number of new slots to add to pSrc->a[]
/// * `iStart` - Index in pSrc->a[] of first new slot
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3SrcListEnlarge(
    mut pParse: *mut Parse,
    mut pSrc: *mut SrcList,
    mut nExtra: i32,
    mut iStart: i32,
) -> *mut SrcList {
    let mut i: i32 = 0 as i32;
    // Sanity checking on calling parameters
    0 as i32;
    0 as i32;
    0 as i32;
    0 as i32;
    // Allocate additional space if needed
    if ((unsafe { (*pSrc).nSrc }) as u32).wrapping_add(nExtra as u32) > unsafe { (*pSrc).nAlloc } {
        let mut pNew: *mut SrcList = unsafe { std::mem::zeroed() };
        let mut nAlloc: i64 =
            ((2 as i32) as i64) * ((unsafe { (*pSrc).nSrc }) as i64) + (nExtra as i64);
        let mut db: *mut sqlite3 = unsafe { (*pParse).db };
        if (unsafe { (*pSrc).nSrc }) + nExtra >= (200 as i32) {
            unsafe {
                sqlite3ErrorMsg(
                    pParse,
                    (b"too many FROM clause terms, max: %d\0".as_ptr() as *mut i8) as *const i8,
                    200 as i32,
                )
            };
            return std::ptr::null_mut::<SrcList>();
        }
        if nAlloc > ((200 as i32) as i64) {
            nAlloc = (200 as i32) as i64;
        }
        pNew = (unsafe {
            sqlite3DbRealloc(
                db,
                pSrc as *mut (),
                (8 as u64).wrapping_add((nAlloc as u64).wrapping_mul(72 as u64)),
            )
        }) as *mut SrcList;
        if pNew == std::ptr::null_mut::<SrcList>() {
            0 as i32;
            return std::ptr::null_mut::<SrcList>();
        }
        pSrc = pNew;
        unsafe {
            (*pSrc).nAlloc = (nAlloc as i32) as u32;
        }
    }
    // Move existing slots that come after the newly inserted slots
    // out of the way
    i = (unsafe { (*pSrc).nSrc }) - (1 as i32);
    '__slate_break_2226: loop {
        if !(i >= iStart) {
            break;
        }
        unsafe {
            *unsafe {
                unsafe { std::ptr::addr_of_mut!((*pSrc).a) as *mut SrcItem }
                    .offset((i + nExtra) as isize)
            } = unsafe {
                *unsafe {
                    unsafe { std::ptr::addr_of_mut!((*pSrc).a) as *mut SrcItem }.offset(i as isize)
                }
            };
        }
        let __v2469: i32 = i;
        let __v2470: i32 = __v2469 - (1 as i32);
        i = __v2470;
    }
    let __v2471: *mut SrcList = pSrc;
    let __v2472: i32 = unsafe { (*__v2471).nSrc };
    let __v2473: i32 = __v2472 + nExtra;
    unsafe {
        (*__v2471).nSrc = __v2473;
    }
    // Zero the newly allocated slots
    unsafe {
        memset(
            (unsafe {
                unsafe { std::ptr::addr_of_mut!((*pSrc).a) as *mut SrcItem }.offset(iStart as isize)
            }) as *mut (),
            0 as i32,
            (72 as u64).wrapping_mul((nExtra as i64) as u64),
        )
    };
    i = iStart;
    '__slate_break_2227: loop {
        if !(i < iStart + nExtra) {
            break;
        }
        unsafe {
            (*unsafe {
                unsafe { std::ptr::addr_of_mut!((*pSrc).a) as *mut SrcItem }.offset(i as isize)
            })
            .iCursor = -(1 as i32);
        }
        let __v2474: i32 = i;
        let __v2475: i32 = __v2474 + (1 as i32);
        i = __v2475;
    }
    // Return a pointer to the enlarged SrcList
    return pSrc;
}

/// Append a new table name to the given SrcList.  Create a new SrcList if
/// need be.  A new entry is created in the SrcList even if pTable is NULL.
///
/// A SrcList is returned, or NULL if there is an OOM error or if the
/// SrcList grows to large.  The returned
/// SrcList might be the same as the SrcList that was input or it might be
/// a new one.  If an OOM error does occurs, then the prior value of pList
/// that is input to this routine is automatically freed.
///
/// If pDatabase is not null, it means that the table has an optional
/// database name prefix.  Like this:  "database.table".  The pDatabase
/// points to the table name and the pTable points to the database name.
/// The SrcList.a[].zName field is filled with the table name which might
/// come from pTable (if pDatabase is NULL) or from pDatabase.
/// SrcList.a[].zDatabase is filled with the database name from pTable,
/// or with NULL if no database is specified.
///
/// In other words, if call like this:
///
///         sqlite3SrcListAppend(D,A,B,0);
///
/// Then B is a table name and the database name is unspecified.  If called
/// like this:
///
///         sqlite3SrcListAppend(D,A,B,C);
///
/// Then C is the table name and B is the database name.  If C is defined
/// then so is B.  In other words, we never have a case where:
///
///         sqlite3SrcListAppend(D,A,0,C);
///
/// Both pTable and pDatabase are assumed to be quoted.  They are dequoted
/// before being added to the SrcList.
///
/// # Arguments
///
/// * `pParse` - Parsing context, in which errors are reported
/// * `pList` - Append to this SrcList. NULL creates a new SrcList
/// * `pTable` - Table to append
/// * `pDatabase` - Database of the table
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3SrcListAppend(
    mut pParse: *mut Parse,
    mut pList: *mut SrcList,
    mut pTable: *mut Token,
    mut pDatabase: *mut Token,
) -> *mut SrcList {
    let mut pItem: *mut SrcItem = unsafe { std::mem::zeroed() };
    let mut db: *mut sqlite3 = unsafe { std::mem::zeroed() };
    0 as i32; // Cannot have C without B
    0 as i32;
    0 as i32;
    db = unsafe { (*pParse).db };
    if pList == std::ptr::null_mut::<SrcList>() {
        pList = (unsafe {
            sqlite3DbMallocRawNN(
                unsafe { (*pParse).db },
                (8 as u64).wrapping_add((((1 as i32) as i64) as u64).wrapping_mul(72 as u64)),
            )
        }) as *mut SrcList;
        if pList == std::ptr::null_mut::<SrcList>() {
            return std::ptr::null_mut::<SrcList>();
        }
        unsafe {
            (*pList).nAlloc = (1 as i32) as u32;
        }
        unsafe {
            (*pList).nSrc = 1 as i32;
        }
        unsafe {
            memset(
                (unsafe {
                    unsafe { std::ptr::addr_of_mut!((*pList).a) as *mut SrcItem }
                        .offset((0 as i32) as isize)
                }) as *mut (),
                0 as i32,
                72 as u64,
            )
        };
        unsafe {
            (*unsafe {
                unsafe { std::ptr::addr_of_mut!((*pList).a) as *mut SrcItem }
                    .offset((0 as i32) as isize)
            })
            .iCursor = -(1 as i32);
        }
    } else {
        let mut pNew: *mut SrcList =
            sqlite3SrcListEnlarge(pParse, pList, 1 as i32, unsafe { (*pList).nSrc });
        if pNew == std::ptr::null_mut::<SrcList>() {
            sqlite3SrcListDelete(db, pList);
            return std::ptr::null_mut::<SrcList>();
        } else {
            pList = pNew;
        }
    }
    pItem = unsafe {
        unsafe { std::ptr::addr_of_mut!((*pList).a) as *mut SrcItem }
            .offset(((unsafe { (*pList).nSrc }) - (1 as i32)) as isize)
    };
    if pDatabase != std::ptr::null_mut::<Token>()
        && (unsafe { (*pDatabase).z }) == std::ptr::null::<i8>()
    {
        pDatabase = std::ptr::null_mut::<Token>();
    }
    0 as i32;
    0 as i32;
    if pDatabase != std::ptr::null_mut::<Token>() {
        unsafe {
            (*pItem).zName = sqlite3NameFromToken(db, pDatabase as *const Token);
        }
        unsafe {
            (*pItem).u4.zDatabase = sqlite3NameFromToken(db, pTable as *const Token);
        }
    } else {
        unsafe {
            (*pItem).zName = sqlite3NameFromToken(db, pTable as *const Token);
        }
        unsafe {
            (*pItem).u4.zDatabase = std::ptr::null_mut::<i8>();
        }
    }
    return pList;
}

/// Assign VdbeCursor index numbers to all tables in a SrcList
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3SrcListAssignCursors(mut pParse: *mut Parse, mut pList: *mut SrcList) {
    let mut i: i32 = 0 as i32;
    let mut pItem: *mut SrcItem = unsafe { std::mem::zeroed() };
    0 as i32;
    if pList != std::ptr::null_mut::<SrcList>() {
        i = 0 as i32;
        let __v2494: *mut SrcItem = unsafe { std::ptr::addr_of_mut!((*pList).a) as *mut SrcItem };
        pItem = __v2494;
        '__slate_break_2228: while i < unsafe { (*pList).nSrc } {
            if (unsafe { (*pItem).iCursor }) >= (0 as i32) {
            } else {
                let __v2499: *mut Parse = pParse;
                let __v2500: i32 = unsafe { (*__v2499).nTab };
                let __v2501: i32 = __v2500 + (1 as i32);
                unsafe {
                    (*__v2499).nTab = __v2501;
                }
                unsafe {
                    (*pItem).iCursor = __v2500;
                }
                if ((unsafe { (*pItem).fg.__slate_bits_0.__get_isSubquery() }) as i32) != (0 as i32)
                {
                    0 as i32;
                    0 as i32;
                    0 as i32;
                    sqlite3SrcListAssignCursors(pParse, unsafe {
                        (*unsafe { (*unsafe { (*pItem).u4.pSubq }).pSelect }).pSrc
                    });
                }
            }
            let __v2495: i32 = i;
            let __v2496: i32 = __v2495 + (1 as i32);
            i = __v2496;
            let __v2497: *mut SrcItem = pItem;
            let __v2498: *mut SrcItem = unsafe { __v2497.offset((1 as i32) as isize) };
            pItem = __v2498;
        }
    }
}

/// Delete a Subquery object and its substructure.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3SubqueryDelete(mut db: *mut sqlite3, mut pSubq: *mut Subquery) {
    0 as i32;
    unsafe { sqlite3SelectDelete(db, unsafe { (*pSubq).pSelect }) };
    unsafe { sqlite3DbFree(db, pSubq as *mut ()) };
}

/// Remove a Subquery from a SrcItem.  Return the associated Select object.
/// The returned Select becomes the responsibility of the caller.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3SubqueryDetach(
    mut db: *mut sqlite3,
    mut pItem: *mut SrcItem,
) -> *mut Select {
    let mut pSel: *mut Select = unsafe { std::mem::zeroed() };
    0 as i32;
    0 as i32;
    pSel = unsafe { (*unsafe { (*pItem).u4.pSubq }).pSelect };
    unsafe { sqlite3DbFree(db, (unsafe { (*pItem).u4.pSubq }) as *mut ()) };
    unsafe {
        (*pItem).u4.pSubq = std::ptr::null_mut::<Subquery>();
    }
    unsafe {
        (*pItem)
            .fg
            .__slate_bits_0
            .__set_isSubquery((0 as i32) as u32);
    }
    return pSel;
}

/// Delete an entire SrcList including all its substructure.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3SrcListDelete(mut db: *mut sqlite3, mut pList: *mut SrcList) {
    let mut i: i32 = 0 as i32;
    let mut pItem: *mut SrcItem = unsafe { std::mem::zeroed() };
    0 as i32;
    if pList == std::ptr::null_mut::<SrcList>() {
        return;
    }
    pItem = unsafe { std::ptr::addr_of_mut!((*pList).a) as *mut SrcItem };
    i = 0 as i32;
    '__slate_break_2229: while i < unsafe { (*pList).nSrc } {
        // Check invariants on SrcItem
        0 as i32;
        0 as i32;
        0 as i32;
        0 as i32;
        if (unsafe { (*pItem).zName }) != std::ptr::null_mut::<i8>() {
            unsafe { sqlite3DbNNFreeNN(db, (unsafe { (*pItem).zName }) as *mut ()) };
        }
        if (unsafe { (*pItem).zAlias }) != std::ptr::null_mut::<i8>() {
            unsafe { sqlite3DbNNFreeNN(db, (unsafe { (*pItem).zAlias }) as *mut ()) };
        }
        if ((unsafe { (*pItem).fg.__slate_bits_0.__get_isSubquery() }) as i32) != (0 as i32) {
            sqlite3SubqueryDelete(db, unsafe { (*pItem).u4.pSubq });
        } else {
            if ((unsafe { (*pItem).fg.__slate_bits_0.__get_fixedSchema() }) as i32) == (0 as i32)
                && (unsafe { (*pItem).u4.zDatabase }) != std::ptr::null_mut::<i8>()
            {
                unsafe { sqlite3DbNNFreeNN(db, (unsafe { (*pItem).u4.zDatabase }) as *mut ()) };
            }
        }
        if ((unsafe { (*pItem).fg.__slate_bits_0.__get_isIndexedBy() }) as i32) != (0 as i32) {
            unsafe { sqlite3DbFree(db, (unsafe { (*pItem).u1.zIndexedBy }) as *mut ()) };
        }
        if ((unsafe { (*pItem).fg.__slate_bits_0.__get_isTabFunc() }) as i32) != (0 as i32) {
            unsafe { sqlite3ExprListDelete(db, unsafe { (*pItem).u1.pFuncArg }) };
        }
        sqlite3DeleteTable(db, unsafe { (*pItem).pSTab });
        if ((unsafe { (*pItem).fg.__slate_bits_0.__get_isUsing() }) as i32) != (0 as i32) {
            sqlite3IdListDelete(db, unsafe { (*pItem).u3.pUsing });
        } else {
            if (unsafe { (*pItem).u3.pOn }) != std::ptr::null_mut::<Expr>() {
                unsafe { sqlite3ExprDelete(db, unsafe { (*pItem).u3.pOn }) };
            }
        }
        let __v2504: i32 = i;
        let __v2505: i32 = __v2504 + (1 as i32);
        i = __v2505;
        let __v2506: *mut SrcItem = pItem;
        let __v2507: *mut SrcItem = unsafe { __v2506.offset((1 as i32) as isize) };
        pItem = __v2507;
    }
    unsafe { sqlite3DbNNFreeNN(db, pList as *mut ()) };
}

/// Attach a Subquery object to pItem->uv.pSubq.  Set the
/// pSelect value but leave all the other values initialized
/// to zero.
///
/// A copy of the Select object is made if dupSelect is true, and the
/// SrcItem takes responsibility for deleting the copy.  If dupSelect is
/// false, ownership of the Select passes to the SrcItem.  Either way,
/// the SrcItem will take responsibility for deleting the Select.
///
/// When dupSelect is zero, that means the Select might get deleted right
/// away if there is an OOM error.  Beware.
///
/// Return non-zero on success.  Return zero on an OOM error.
///
/// # Arguments
///
/// * `pParse` - Parsing context
/// * `pItem` - Item to which the subquery is to be attached
/// * `pSelect` - The subquery SELECT.  Must be non-NULL
/// * `dupSelect` - If true, attach a copy of pSelect, not pSelect itself.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3SrcItemAttachSubquery(
    mut pParse: *mut Parse,
    mut pItem: *mut SrcItem,
    mut pSelect: *mut Select,
    mut dupSelect: i32,
) -> i32 {
    let mut p: *mut Subquery = unsafe { std::mem::zeroed() };
    0 as i32;
    0 as i32;
    if ((unsafe { (*pItem).fg.__slate_bits_0.__get_fixedSchema() }) as i32) != (0 as i32) {
        unsafe {
            (*pItem).u4.pSchema = std::ptr::null_mut::<Schema>();
        }
        unsafe {
            (*pItem)
                .fg
                .__slate_bits_0
                .__set_fixedSchema((0 as i32) as u32);
        }
    } else {
        if (unsafe { (*pItem).u4.zDatabase }) != std::ptr::null_mut::<i8>() {
            unsafe {
                sqlite3DbFree(
                    unsafe { (*pParse).db },
                    (unsafe { (*pItem).u4.zDatabase }) as *mut (),
                )
            };
            unsafe {
                (*pItem).u4.zDatabase = std::ptr::null_mut::<i8>();
            }
        }
    }
    if dupSelect != (0 as i32) {
        pSelect = unsafe {
            sqlite3SelectDup(unsafe { (*pParse).db }, pSelect as *const Select, 0 as i32)
        };
        if pSelect == std::ptr::null_mut::<Select>() {
            return 0 as i32;
        }
    }
    let __v2479: *mut Subquery =
        (unsafe { sqlite3DbMallocRawNN(unsafe { (*pParse).db }, 24 as u64) }) as *mut Subquery;
    unsafe {
        (*pItem).u4.pSubq = __v2479;
    }
    p = __v2479;
    if p == std::ptr::null_mut::<Subquery>() {
        unsafe { sqlite3SelectDelete(unsafe { (*pParse).db }, pSelect) };
        return 0 as i32;
    }
    unsafe {
        (*pItem)
            .fg
            .__slate_bits_0
            .__set_isSubquery((1 as i32) as u32);
    }
    unsafe {
        (*p).pSelect = pSelect;
    }
    0 as i32;
    unsafe {
        memset(
            (unsafe { (p as *mut i8).offset((8 as u64) as isize) }) as *mut (),
            0 as i32,
            (24 as u64).wrapping_sub(8 as u64),
        )
    };
    return 1 as i32;
}

/// This routine is called by the parser to add a new term to the
/// end of a growing FROM clause.  The "p" parameter is the part of
/// the FROM clause that has already been constructed.  "p" is NULL
/// if this is the first term of the FROM clause.  pTable and pDatabase
/// are the name of the table and database named in the FROM clause term.
/// pDatabase is NULL if the database name qualifier is missing - the
/// usual case.  If the term has an alias, then pAlias points to the
/// alias token.  If the term is a subquery, then pSubquery is the
/// SELECT statement that the subquery encodes.  The pTable and
/// pDatabase parameters are NULL for subqueries.  The pOn and pUsing
/// parameters are the content of the ON and USING clauses.
///
/// Return a new SrcList which encodes is the FROM with the new
/// term added.
///
/// # Arguments
///
/// * `pParse` - Parsing context
/// * `p` - The left part of the FROM clause already seen
/// * `pTable` - Name of the table to add to the FROM clause
/// * `pDatabase` - Name of the database containing pTable
/// * `pAlias` - The right-hand side of the AS subexpression
/// * `pSubquery` - A subquery used in place of a table name
/// * `pOnUsing` - Either the ON clause or the USING clause
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3SrcListAppendFromTerm(
    mut pParse: *mut Parse,
    mut p: *mut SrcList,
    mut pTable: *mut Token,
    mut pDatabase: *mut Token,
    mut pAlias: *mut Token,
    mut pSubquery: *mut Select,
    mut pOnUsing: *mut OnOrUsing,
) -> *mut SrcList {
    let mut __slate_storage_1283: std::mem::MaybeUninit<*mut Token> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1283: *mut *mut Token =
        std::ptr::addr_of_mut!(__slate_storage_1283) as *mut *mut Token;
    let mut __slate_storage_1282: std::mem::MaybeUninit<*mut sqlite3> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1282: *mut *mut sqlite3 =
        std::ptr::addr_of_mut!(__slate_storage_1282) as *mut *mut sqlite3;
    let mut __slate_storage_1281: std::mem::MaybeUninit<*mut SrcItem> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1281: *mut *mut SrcItem =
        std::ptr::addr_of_mut!(__slate_storage_1281) as *mut *mut SrcItem;
    unsafe {
        '__join_0: {
            std::ptr::write(__slate_slot_1282, unsafe { (*pParse).db });
            if !(p != std::ptr::null_mut::<SrcList>())
                && pOnUsing != std::ptr::null_mut::<OnOrUsing>()
                && ((unsafe { (*pOnUsing).pOn }) != std::ptr::null_mut::<Expr>()
                    || (unsafe { (*pOnUsing).pUsing }) != std::ptr::null_mut::<IdList>())
            {
                unsafe {
                    sqlite3ErrorMsg(
                        pParse,
                        (b"a JOIN clause is required before %s\0".as_ptr() as *mut i8) as *const i8,
                        if (unsafe { (*pOnUsing).pOn }) != std::ptr::null_mut::<Expr>() {
                            b"ON\0".as_ptr() as *mut i8
                        } else {
                            b"USING\0".as_ptr() as *mut i8
                        },
                    )
                };
            } else {
                p = sqlite3SrcListAppend(pParse, p, pTable, pDatabase);
                if p == std::ptr::null_mut::<SrcList>() {
                } else {
                    0 as i32;
                    *__slate_slot_1281 = unsafe {
                        unsafe { std::ptr::addr_of_mut!((*p).a) as *mut SrcItem }
                            .offset(((unsafe { (*p).nSrc }) - (1 as i32)) as isize)
                    };
                    0 as i32;
                    0 as i32;
                    if (((unsafe { (*pParse).eParseMode }) as u32) as i32) >= (2 as i32)
                        && (unsafe { (*(*__slate_slot_1281)).zName }) != std::ptr::null_mut::<i8>()
                    {
                        std::ptr::write(
                            __slate_slot_1283,
                            if pDatabase != std::ptr::null_mut::<Token>()
                                && (unsafe { (*pDatabase).z }) != std::ptr::null::<i8>()
                            {
                                pDatabase
                            } else {
                                pTable
                            },
                        );
                        unsafe {
                            sqlite3RenameTokenMap(
                                pParse,
                                (unsafe { (*(*__slate_slot_1281)).zName }) as *const (),
                                *__slate_slot_1283 as *const Token,
                            )
                        };
                    }
                    0 as i32;
                    if (unsafe { (*pAlias).n }) != (0 as u32) {
                        unsafe {
                            (*(*__slate_slot_1281)).zAlias =
                                sqlite3NameFromToken(*__slate_slot_1282, pAlias as *const Token);
                        }
                    }
                    0 as i32;
                    if pSubquery != std::ptr::null_mut::<Select>() {
                        if sqlite3SrcItemAttachSubquery(
                            pParse,
                            *__slate_slot_1281,
                            pSubquery,
                            0 as i32,
                        ) != (0 as i32)
                        {
                            if (unsafe { (*pSubquery).selFlags }) & ((2048 as i32) as u32)
                                != (0 as u32)
                            {
                                unsafe {
                                    (*(*__slate_slot_1281))
                                        .fg
                                        .__slate_bits_0
                                        .__set_isNestedFrom((1 as i32) as u32);
                                }
                            }
                        }
                    }
                    0 as i32;
                    0 as i32;
                    if pOnUsing == std::ptr::null_mut::<OnOrUsing>() {
                        unsafe {
                            (*(*__slate_slot_1281)).u3.pOn = std::ptr::null_mut::<Expr>();
                        }
                    } else {
                        if (unsafe { (*pOnUsing).pUsing }) != std::ptr::null_mut::<IdList>() {
                            unsafe {
                                (*(*__slate_slot_1281))
                                    .fg
                                    .__slate_bits_0
                                    .__set_isUsing((1 as i32) as u32);
                            }
                            unsafe {
                                (*(*__slate_slot_1281)).u3.pUsing = unsafe { (*pOnUsing).pUsing };
                            }
                        } else {
                            unsafe {
                                (*(*__slate_slot_1281)).u3.pOn = unsafe { (*pOnUsing).pOn };
                            }
                        }
                    }
                    return p;
                }
            }
        }
        0 as i32;
        unsafe { sqlite3ClearOnOrUsing(*__slate_slot_1282, pOnUsing) };
        unsafe { sqlite3SelectDelete(*__slate_slot_1282, pSubquery) };
        return std::ptr::null_mut::<SrcList>();
    }
    return unsafe { std::mem::zeroed() };
}

/// Add an INDEXED BY or NOT INDEXED clause to the most recently added
/// element of the source-list passed as the second argument.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3SrcListIndexedBy(
    mut pParse: *mut Parse,
    mut p: *mut SrcList,
    mut pIndexedBy: *mut Token,
) {
    0 as i32;
    if p != std::ptr::null_mut::<SrcList>() && (unsafe { (*pIndexedBy).n }) > ((0 as i32) as u32) {
        let mut pItem: *mut SrcItem = unsafe { std::mem::zeroed() };
        0 as i32;
        pItem = unsafe {
            unsafe { std::ptr::addr_of_mut!((*p).a) as *mut SrcItem }
                .offset(((unsafe { (*p).nSrc }) - (1 as i32)) as isize)
        };
        0 as i32;
        0 as i32;
        0 as i32;
        if (unsafe { (*pIndexedBy).n }) == ((1 as i32) as u32)
            && !((unsafe { (*pIndexedBy).z }) != std::ptr::null::<i8>())
        {
            // A "NOT INDEXED" clause was supplied. See parse.y
            // construct "indexed_opt" for details.
            unsafe {
                (*pItem)
                    .fg
                    .__slate_bits_0
                    .__set_notIndexed((1 as i32) as u32);
            }
        } else {
            unsafe {
                (*pItem).u1.zIndexedBy =
                    sqlite3NameFromToken(unsafe { (*pParse).db }, pIndexedBy as *const Token);
            }
            unsafe {
                (*pItem)
                    .fg
                    .__slate_bits_0
                    .__set_isIndexedBy((1 as i32) as u32);
            }
            0 as i32; // No collision on union u2
        }
    }
}

/// Append the contents of SrcList p2 to SrcList p1 and return the resulting
/// SrcList. Or, if an error occurs, return NULL. In all cases, p1 and p2
/// are deleted by this function.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3SrcListAppendList(
    mut pParse: *mut Parse,
    mut p1: *mut SrcList,
    mut p2: *mut SrcList,
) -> *mut SrcList {
    0 as i32;
    0 as i32;
    0 as i32;
    {}
    if p2 != std::ptr::null_mut::<SrcList>() {
        let mut nOld: i32 = unsafe { (*p1).nSrc };
        let mut pNew: *mut SrcList = sqlite3SrcListEnlarge(pParse, p1, unsafe { (*p2).nSrc }, nOld);
        if pNew == std::ptr::null_mut::<SrcList>() {
            sqlite3SrcListDelete(unsafe { (*pParse).db }, p2);
        } else {
            p1 = pNew;
            unsafe {
                memcpy(
                    (unsafe {
                        unsafe { std::ptr::addr_of_mut!((*p1).a) as *mut SrcItem }
                            .offset(nOld as isize)
                    }) as *mut (),
                    (unsafe { std::ptr::addr_of_mut!((*p2).a) as *mut SrcItem }) as *const (),
                    (((unsafe { (*p2).nSrc }) as i64) as u64).wrapping_mul(72 as u64),
                )
            };
            0 as i32;
            0 as i32;
            let __v2476: *mut SrcItem = unsafe {
                unsafe { std::ptr::addr_of_mut!((*p1).a) as *mut SrcItem }
                    .offset((0 as i32) as isize)
            };
            let __v2477: u8 = unsafe { (*__v2476).fg.jointype };
            let __v2478: u8 = ((((__v2477 as u32) as i32)
                | (64 as i32)
                    & (((unsafe {
                        (*unsafe {
                            unsafe { std::ptr::addr_of_mut!((*p2).a) as *mut SrcItem }
                                .offset((0 as i32) as isize)
                        })
                        .fg
                        .jointype
                    }) as u32) as i32)) as i8) as u8;
            unsafe {
                (*__v2476).fg.jointype = __v2478;
            }
            unsafe { sqlite3DbFree(unsafe { (*pParse).db }, p2 as *mut ()) };
        }
    }
    return p1;
}

/// Add the list of function arguments to the SrcList entry for a
/// table-valued-function.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3SrcListFuncArgs(
    mut pParse: *mut Parse,
    mut p: *mut SrcList,
    mut pList: *mut ExprList,
) {
    if p != std::ptr::null_mut::<SrcList>() {
        let mut pItem: *mut SrcItem = unsafe {
            unsafe { std::ptr::addr_of_mut!((*p).a) as *mut SrcItem }
                .offset(((unsafe { (*p).nSrc }) - (1 as i32)) as isize)
        };
        0 as i32;
        0 as i32;
        0 as i32;
        unsafe {
            (*pItem).u1.pFuncArg = pList;
        }
        unsafe {
            (*pItem)
                .fg
                .__slate_bits_0
                .__set_isTabFunc((1 as i32) as u32);
        }
    } else {
        unsafe { sqlite3ExprListDelete(unsafe { (*pParse).db }, pList) };
    }
}

/// When building up a FROM clause in the parser, the join operator
/// is initially attached to the left operand.  But the code generator
/// expects the join operator to be on the right operand.  This routine
/// Shifts all join operators from left to right for an entire FROM
/// clause.
///
/// Example: Suppose the join is like this:
///
///           A natural cross join B
///
/// The operator is "natural cross join".  The A and B operands are stored
/// in p->a[0] and p->a[1], respectively.  The parser initially stores the
/// operator with A.  This routine shifts that operator over to B.
///
/// Additional changes:
///
///   *   All tables to the left of the right-most RIGHT JOIN are tagged with
///       JT_LTORJ (mnemonic: Left Table Of Right Join) so that the
///       code generator can easily tell that the table is part of
///       the left operand of at least one RIGHT JOIN.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3SrcListShiftJoinType(mut pParse: *mut Parse, mut p: *mut SrcList) {
    pParse;
    if p != std::ptr::null_mut::<SrcList>() && (unsafe { (*p).nSrc }) > (1 as i32) {
        let mut i: i32 = (unsafe { (*p).nSrc }) - (1 as i32);
        let mut allFlags: u8 = ((0 as i32) as i8) as u8;
        '__slate_break_2233: loop {
            let __v2480: u8 = allFlags;
            let __v2481: u8 = unsafe {
                (*unsafe {
                    unsafe { std::ptr::addr_of_mut!((*p).a) as *mut SrcItem }
                        .offset((i - (1 as i32)) as isize)
                })
                .fg
                .jointype
            };
            unsafe {
                (*unsafe {
                    unsafe { std::ptr::addr_of_mut!((*p).a) as *mut SrcItem }.offset(i as isize)
                })
                .fg
                .jointype = __v2481;
            }
            let __v2482: u8 = ((((__v2480 as u32) as i32) | ((__v2481 as u32) as i32)) as i8) as u8;
            allFlags = __v2482;
            let __v2483: i32 = i;
            let __v2484: i32 = __v2483 - (1 as i32);
            i = __v2484;
            if !(__v2484 > (0 as i32)) {
                break;
            }
        }
        unsafe {
            (*unsafe {
                unsafe { std::ptr::addr_of_mut!((*p).a) as *mut SrcItem }
                    .offset((0 as i32) as isize)
            })
            .fg
            .jointype = ((0 as i32) as i8) as u8;
        }
        // All terms to the left of a RIGHT JOIN should be tagged with the
        // JT_LTORJ flags
        if ((allFlags as u32) as i32) & (16 as i32) != (0 as i32) {
            i = (unsafe { (*p).nSrc }) - (1 as i32);
            '__slate_break_2234: loop {
                if !(i > (0 as i32)
                    && (((unsafe {
                        (*unsafe {
                            unsafe { std::ptr::addr_of_mut!((*p).a) as *mut SrcItem }
                                .offset(i as isize)
                        })
                        .fg
                        .jointype
                    }) as u32) as i32)
                        & (16 as i32)
                        == (0 as i32))
                {
                    break;
                }
                let __v2485: i32 = i;
                let __v2486: i32 = __v2485 - (1 as i32);
                i = __v2486;
            }
            let __v2487: i32 = i;
            let __v2488: i32 = __v2487 - (1 as i32);
            i = __v2488;
            0 as i32;
            '__slate_break_2235: loop {
                let __v2489: *mut SrcItem = unsafe {
                    unsafe { std::ptr::addr_of_mut!((*p).a) as *mut SrcItem }.offset(i as isize)
                };
                let __v2490: u8 = unsafe { (*__v2489).fg.jointype };
                let __v2491: u8 = ((((__v2490 as u32) as i32) | (64 as i32)) as i8) as u8;
                unsafe {
                    (*__v2489).fg.jointype = __v2491;
                }
                let __v2492: i32 = i;
                let __v2493: i32 = __v2492 - (1 as i32);
                i = __v2493;
                if !(__v2493 >= (0 as i32)) {
                    break;
                }
            }
        }
    }
}

/// Generate VDBE code for a BEGIN statement.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3BeginTransaction(mut pParse: *mut Parse, mut r#type: i32) {
    let mut db: *mut sqlite3 = unsafe { std::mem::zeroed() };
    let mut v: *mut Vdbe = unsafe { std::mem::zeroed() };
    let mut i: i32 = 0 as i32;
    0 as i32;
    db = unsafe { (*pParse).db };
    0 as i32;
    if (unsafe {
        sqlite3AuthCheck(
            pParse,
            22 as i32,
            (b"BEGIN\0".as_ptr() as *mut i8) as *const i8,
            std::ptr::null::<i8>(),
            std::ptr::null::<i8>(),
        )
    }) != (0 as i32)
    {
        return;
    }
    v = unsafe { sqlite3GetVdbe(pParse) };
    if !(v != std::ptr::null_mut::<Vdbe>()) {
        return;
    }
    if r#type != (7 as i32) {
        i = 0 as i32;
        '__slate_break_2237: loop {
            if !(i < unsafe { (*db).nDb }) {
                break;
            }
            let mut eTxnType: i32 = 0 as i32;
            let mut pBt: *mut Btree =
                unsafe { (*unsafe { unsafe { (*db).aDb }.offset(i as isize) }).pBt };
            let __v2587: bool;
            if pBt != std::ptr::null_mut::<Btree>() {
                __v2587 = (unsafe { sqlite3BtreeIsReadonly(pBt) }) != (0 as i32);
            } else {
                __v2587 = false as bool;
            }
            if __v2587 {
                eTxnType = 0 as i32; // Read txn
            } else {
                if r#type == (9 as i32) {
                    eTxnType = 2 as i32; // Exclusive txn
                } else {
                    eTxnType = 1 as i32; // Write txn
                }
            }
            unsafe { sqlite3VdbeAddOp2(v, 2 as i32, i, eTxnType) };
            unsafe { sqlite3VdbeUsesBtree(v, i) };
            let __v2585: i32 = i;
            let __v2586: i32 = __v2585 + (1 as i32);
            i = __v2586;
        }
    }
    unsafe { sqlite3VdbeAddOp0(v, 1 as i32) };
}

/// Generate VDBE code for a COMMIT or ROLLBACK statement.
/// Code for ROLLBACK is generated if eType==TK_ROLLBACK.  Otherwise
/// code is generated for a COMMIT.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3EndTransaction(mut pParse: *mut Parse, mut eType: i32) {
    let mut v: *mut Vdbe = unsafe { std::mem::zeroed() };
    let mut isRollback: i32 = 0 as i32;
    0 as i32;
    0 as i32;
    0 as i32;
    isRollback = (eType == (12 as i32)) as i32;
    if (unsafe {
        sqlite3AuthCheck(
            pParse,
            22 as i32,
            (if isRollback != (0 as i32) {
                b"ROLLBACK\0".as_ptr() as *mut i8
            } else {
                b"COMMIT\0".as_ptr() as *mut i8
            }) as *const i8,
            std::ptr::null::<i8>(),
            std::ptr::null::<i8>(),
        )
    }) != (0 as i32)
    {
        return;
    }
    v = unsafe { sqlite3GetVdbe(pParse) };
    if v != std::ptr::null_mut::<Vdbe>() {
        unsafe { sqlite3VdbeAddOp2(v, 1 as i32, 1 as i32, isRollback) };
    }
}

/// This function is called by the parser when it parses a command to create,
/// release or rollback an SQL savepoint.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3Savepoint(mut pParse: *mut Parse, mut op: i32, mut pName: *mut Token) {
    let mut zName: *mut i8 = sqlite3NameFromToken(unsafe { (*pParse).db }, pName as *const Token);
    if zName != std::ptr::null_mut::<i8>() {
        let mut v: *mut Vdbe = unsafe { sqlite3GetVdbe(pParse) };
        0 as i32;
        let __v2588: bool;
        if !(v != std::ptr::null_mut::<Vdbe>()) {
            __v2588 = true as bool;
        } else {
            __v2588 = (unsafe {
                sqlite3AuthCheck(
                    pParse,
                    32 as i32,
                    unsafe {
                        *unsafe {
                            unsafe { std::ptr::addr_of!(az.0) as *const *const i8 }
                                .offset(op as isize)
                        }
                    },
                    zName as *const i8,
                    std::ptr::null::<i8>(),
                )
            }) != (0 as i32);
        }
        if __v2588 {
            unsafe { sqlite3DbFree(unsafe { (*pParse).db }, zName as *mut ()) };
            return;
        }
        unsafe {
            sqlite3VdbeAddOp4(
                v,
                0 as i32,
                op,
                0 as i32,
                0 as i32,
                zName as *const i8,
                -(7 as i32),
            )
        };
    }
}

static mut az: __SlateAlign16<[*const i8; 3]> = __SlateAlign16([
    (b"BEGIN\0".as_ptr() as *mut i8) as *const i8,
    (b"RELEASE\0".as_ptr() as *mut i8) as *const i8,
    (b"ROLLBACK\0".as_ptr() as *mut i8) as *const i8,
]);

/// Make sure the TEMP database is open and available for use.  Return
/// the number of errors.  Leave any error messages in the pParse structure.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3OpenTempDatabase(mut pParse: *mut Parse) -> i32 {
    let mut db: *mut sqlite3 = unsafe { (*pParse).db };
    if (unsafe { (*unsafe { unsafe { (*db).aDb }.offset((1 as i32) as isize) }).pBt })
        == std::ptr::null_mut::<Btree>()
        && !((unsafe { (*pParse).explain }) != (0 as u8))
    {
        let mut rc: i32 = 0 as i32;
        let mut pBt: *mut Btree = unsafe { std::mem::zeroed() };
        rc = unsafe {
            sqlite3BtreeOpen(
                unsafe { (*db).pVfs },
                std::ptr::null::<i8>(),
                db,
                std::ptr::addr_of_mut!(pBt),
                0 as i32,
                unsafe { flags },
            )
        };
        if rc != (0 as i32) {
            unsafe {
                sqlite3ErrorMsg(
                    pParse,
                    (b"unable to open a temporary database file for storing temporary tables\0"
                        .as_ptr() as *mut i8) as *const i8,
                )
            };
            unsafe {
                (*pParse).rc = rc;
            }
            return 1 as i32;
        }
        unsafe {
            (*unsafe { unsafe { (*db).aDb }.offset((1 as i32) as isize) }).pBt = pBt;
        }
        0 as i32;
        if (7 as i32)
            == unsafe {
                sqlite3BtreeSetPageSize(pBt, unsafe { (*db).nextPagesize }, 0 as i32, 0 as i32)
            }
        {
            unsafe { sqlite3OomFault(db) };
            return 1 as i32;
        }
    }
    return 0 as i32;
}

static mut flags: i32 = (2 as i32) | (4 as i32) | (16 as i32) | (8 as i32) | (512 as i32);

/// Record the fact that the schema cookie will need to be verified
/// for database iDb.  The code to actually verify the schema cookie
/// will occur at the end of the top-level VDBE and will be generated
/// later, by sqlite3FinishCoding().
fn sqlite3CodeVerifySchemaAtToplevel(mut pToplevel: *mut Parse, mut iDb: i32) {
    0 as i32;
    0 as i32;
    0 as i32;
    0 as i32;
    if (((unsafe { (*pToplevel).cookieMask }) & ((1 as i32) as u32) << iDb != ((0 as i32) as u32))
        as i32)
        == (0 as i32)
    {
        let __v2831: *mut Parse = pToplevel;
        let __v2832: u32 = unsafe { (*__v2831).cookieMask };
        let __v2833: u32 = __v2832 | ((1 as i32) as u32) << iDb;
        unsafe {
            (*__v2831).cookieMask = __v2833;
        }
        if !((0 as i32) != (0 as i32)) && iDb == (1 as i32) {
            sqlite3OpenTempDatabase(pToplevel);
        }
    }
}

#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3CodeVerifySchema(mut pParse: *mut Parse, mut iDb: i32) {
    sqlite3CodeVerifySchemaAtToplevel(
        if (unsafe { (*pParse).pToplevel }) != std::ptr::null_mut::<Parse>() {
            unsafe { (*pParse).pToplevel }
        } else {
            pParse
        },
        iDb,
    );
}

/// If argument zDb is NULL, then call sqlite3CodeVerifySchema() for each
/// attached database. Otherwise, invoke it for the database named zDb only.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3CodeVerifyNamedSchema(mut pParse: *mut Parse, mut zDb: *const i8) {
    let mut db: *mut sqlite3 = unsafe { (*pParse).db };
    let mut i: i32 = 0 as i32;
    i = 0 as i32;
    '__slate_break_2244: loop {
        if !(i < unsafe { (*db).nDb }) {
            break;
        }
        let mut pDb: *mut Db = unsafe { unsafe { (*db).aDb }.offset(i as isize) };
        let __v2583: bool;
        if (unsafe { (*pDb).pBt }) != std::ptr::null_mut::<Btree>() {
            let __v2584: bool;
            if !(zDb != std::ptr::null::<i8>()) {
                __v2584 = true as bool;
            } else {
                __v2584 = (0 as i32)
                    == unsafe { sqlite3StrICmp(zDb, (unsafe { (*pDb).zDbSName }) as *const i8) };
            }
            __v2583 = __v2584;
        } else {
            __v2583 = false as bool;
        }
        if __v2583 {
            sqlite3CodeVerifySchema(pParse, i);
        }
        let __v2581: i32 = i;
        let __v2582: i32 = __v2581 + (1 as i32);
        i = __v2582;
    }
}

/// Generate VDBE code that prepares for doing an operation that
/// might change the database.
///
/// This routine starts a new transaction if we are not already within
/// a transaction.  If we are already within a transaction, then a checkpoint
/// is set if the setStatement parameter is true.  A checkpoint should
/// be set for operations that might fail (due to a constraint) part of
/// the way through and which will need to undo some writes without having to
/// rollback the whole transaction.  For operations where all constraints
/// can be checked before any changes are made to the database, it is never
/// necessary to undo a write and the checkpoint should not be set.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3BeginWriteOperation(
    mut pParse: *mut Parse,
    mut setStatement: i32,
    mut iDb: i32,
) {
    let mut pToplevel: *mut Parse =
        if (unsafe { (*pParse).pToplevel }) != std::ptr::null_mut::<Parse>() {
            unsafe { (*pParse).pToplevel }
        } else {
            pParse
        };
    sqlite3CodeVerifySchemaAtToplevel(pToplevel, iDb);
    let __v2589: *mut Parse = pToplevel;
    let __v2590: u32 = unsafe { (*__v2589).writeMask };
    let __v2591: u32 = __v2590 | ((1 as i32) as u32) << iDb;
    unsafe {
        (*__v2589).writeMask = __v2591;
    }
    let __v2592: *mut Parse = pToplevel;
    let __v2593: u8 = unsafe { (*__v2592).isMultiWrite };
    let __v2594: u8 = ((((__v2593 as u32) as i32) | setStatement) as i8) as u8;
    unsafe {
        (*__v2592).isMultiWrite = __v2594;
    }
}

/// Indicate that the statement currently under construction might write
/// more than one entry (example: deleting one row then inserting another,
/// inserting multiple rows in a table, or inserting a row and index entries.)
/// If an abort occurs after some of these writes have completed, then it will
/// be necessary to undo the completed writes.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3MultiWrite(mut pParse: *mut Parse) {
    let mut pToplevel: *mut Parse =
        if (unsafe { (*pParse).pToplevel }) != std::ptr::null_mut::<Parse>() {
            unsafe { (*pParse).pToplevel }
        } else {
            pParse
        };
    unsafe {
        (*pToplevel).isMultiWrite = ((1 as i32) as i8) as u8;
    }
}

/// The code generator calls this routine if is discovers that it is
/// possible to abort a statement prior to completion.  In order to
/// perform this abort without corrupting the database, we need to make
/// sure that the statement is protected by a statement transaction.
///
/// Technically, we only need to set the mayAbort flag if the
/// isMultiWrite flag was previously set.  There is a time dependency
/// such that the abort must occur after the multiwrite.  This makes
/// some statements involving the REPLACE conflict resolution algorithm
/// go a little faster.  But taking advantage of this time dependency
/// makes it more difficult to prove that the code is correct (in
/// particular, it prevents us from writing an effective
/// implementation of sqlite3AssertMayAbort()) and so we have chosen
/// to take the safe route and skip the optimization.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3MayAbort(mut pParse: *mut Parse) {
    let mut pToplevel: *mut Parse =
        if (unsafe { (*pParse).pToplevel }) != std::ptr::null_mut::<Parse>() {
            unsafe { (*pParse).pToplevel }
        } else {
            pParse
        };
    unsafe {
        (*pToplevel)
            .__slate_bits_0
            .__set_mayAbort((1 as i32) as u32);
    }
}

/// Code an OP_Halt that causes the vdbe to return an SQLITE_CONSTRAINT
/// error. The onError parameter determines which (if any) of the statement
/// and/or current transaction is rolled back.
///
/// # Arguments
///
/// * `pParse` - Parsing context
/// * `errCode` - extended error code
/// * `onError` - Constraint type
/// * `p4` - Error message
/// * `p4type` - P4_STATIC or P4_TRANSIENT
/// * `p5Errmsg` - P5_ErrMsg type
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3HaltConstraint(
    mut pParse: *mut Parse,
    mut errCode: i32,
    mut onError: i32,
    mut p4: *mut i8,
    mut p4type: i8,
    mut p5Errmsg: u8,
) {
    let mut v: *mut Vdbe = unsafe { std::mem::zeroed() };
    0 as i32;
    v = unsafe { sqlite3GetVdbe(pParse) };
    0 as i32;
    if onError == (2 as i32) {
        sqlite3MayAbort(pParse);
    }
    unsafe {
        sqlite3VdbeAddOp4(
            v,
            72 as i32,
            errCode,
            onError,
            0 as i32,
            p4 as *const i8,
            p4type as i32,
        )
    };
    unsafe { sqlite3VdbeChangeP5(v, p5Errmsg as u16) };
}

/// Code an OP_Halt due to UNIQUE or PRIMARY KEY constraint violation.
///
/// # Arguments
///
/// * `pParse` - Parsing context
/// * `onError` - Constraint type
/// * `pIdx` - The index that triggers the constraint
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3UniqueConstraint(
    mut pParse: *mut Parse,
    mut onError: i32,
    mut pIdx: *mut Index,
) {
    let mut zErr: *mut i8 = unsafe { std::mem::zeroed() };
    let mut j: i32 = 0 as i32;
    let mut errMsg: sqlite3_str = unsafe { std::mem::zeroed() };
    let mut pTab: *mut Table = unsafe { (*pIdx).pTable };
    unsafe {
        sqlite3StrAccumInit(
            std::ptr::addr_of_mut!(errMsg),
            unsafe { (*pParse).db },
            std::ptr::null_mut::<i8>(),
            0 as i32,
            unsafe {
                *unsafe {
                    unsafe { (*unsafe { (*pParse).db }).aLimit.as_mut_ptr() as *mut i32 }
                        .offset((0 as i32) as isize)
                }
            },
        )
    };
    if (unsafe { (*pIdx).aColExpr }) != std::ptr::null_mut::<ExprList>() {
        unsafe {
            sqlite3_str_appendf(
                std::ptr::addr_of_mut!(errMsg),
                (b"index '%q'\0".as_ptr() as *mut i8) as *const i8,
                unsafe { (*pIdx).zName },
            )
        };
    } else {
        j = 0 as i32;
        '__slate_break_2246: loop {
            if !(j < (((unsafe { (*pIdx).nKeyCol }) as u32) as i32)) {
                break;
            }
            let mut zCol: *mut i8 = unsafe { std::mem::zeroed() };
            0 as i32;
            zCol = unsafe {
                (*unsafe {
                    unsafe { (*pTab).aCol }.offset(
                        ((unsafe { *unsafe { unsafe { (*pIdx).aiColumn }.offset(j as isize) } })
                            as i32) as isize,
                    )
                })
                .zCnName
            };
            if j != (0 as i32) {
                unsafe {
                    sqlite3_str_append(
                        std::ptr::addr_of_mut!(errMsg),
                        (b", \0".as_ptr() as *mut i8) as *const i8,
                        2 as i32,
                    )
                };
            }
            unsafe {
                sqlite3_str_appendall(
                    std::ptr::addr_of_mut!(errMsg),
                    (unsafe { (*pTab).zName }) as *const i8,
                )
            };
            unsafe {
                sqlite3_str_append(
                    std::ptr::addr_of_mut!(errMsg),
                    (b".\0".as_ptr() as *mut i8) as *const i8,
                    1 as i32,
                )
            };
            unsafe { sqlite3_str_appendall(std::ptr::addr_of_mut!(errMsg), zCol as *const i8) };
            let __v2595: i32 = j;
            let __v2596: i32 = __v2595 + (1 as i32);
            j = __v2596;
        }
    }
    zErr = unsafe { sqlite3StrAccumFinish(std::ptr::addr_of_mut!(errMsg)) };
    sqlite3HaltConstraint(
        pParse,
        if ((unsafe { (*pIdx).__slate_bits_0.__get_idxType() }) as i32) == (2 as i32) {
            (19 as i32) | (6 as i32) << (8 as i32)
        } else {
            (19 as i32) | (8 as i32) << (8 as i32)
        },
        onError,
        zErr,
        -(7 as i32) as i8,
        ((2 as i32) as i8) as u8,
    );
}

/// Code an OP_Halt due to non-unique rowid.
///
/// # Arguments
///
/// * `pParse` - Parsing context
/// * `onError` - Conflict resolution algorithm
/// * `pTab` - The table with the non-unique rowid
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3RowidConstraint(
    mut pParse: *mut Parse,
    mut onError: i32,
    mut pTab: *mut Table,
) {
    let mut zMsg: *mut i8 = unsafe { std::mem::zeroed() };
    let mut rc: i32 = 0 as i32;
    if ((unsafe { (*pTab).iPKey }) as i32) >= (0 as i32) {
        zMsg = unsafe {
            sqlite3MPrintf(
                unsafe { (*pParse).db },
                (b"%s.%s\0".as_ptr() as *mut i8) as *const i8,
                unsafe { (*pTab).zName },
                unsafe {
                    (*unsafe {
                        unsafe { (*pTab).aCol }.offset(((unsafe { (*pTab).iPKey }) as i32) as isize)
                    })
                    .zCnName
                },
            )
        };
        rc = (19 as i32) | (6 as i32) << (8 as i32);
    } else {
        zMsg = unsafe {
            sqlite3MPrintf(
                unsafe { (*pParse).db },
                (b"%s.rowid\0".as_ptr() as *mut i8) as *const i8,
                unsafe { (*pTab).zName },
            )
        };
        rc = (19 as i32) | (10 as i32) << (8 as i32);
    }
    sqlite3HaltConstraint(
        pParse,
        rc,
        onError,
        zMsg,
        -(7 as i32) as i8,
        ((2 as i32) as i8) as u8,
    );
}

/// Return true if any column of pIndex uses the zColl collation
fn collationMatch(mut zColl: *const i8, mut pIndex: *mut Index) -> i32 {
    let mut i: i32 = 0 as i32;
    0 as i32;
    i = 0 as i32;
    '__slate_break_2251: loop {
        if !(i < (((unsafe { (*pIndex).nColumn }) as u32) as i32)) {
            break;
        }
        let mut z: *const i8 =
            unsafe { *unsafe { unsafe { (*pIndex).azColl }.offset(i as isize) } };
        0 as i32;
        if (0 as i32) == unsafe { sqlite3StrICmp(z, zColl) } {
            return 1 as i32;
        }
        let __v2834: i32 = i;
        let __v2835: i32 = __v2834 + (1 as i32);
        i = __v2835;
    }
    return 0 as i32;
}

/// Generate code for the REINDEX command.
///
///        REINDEX                            -- 1
///        REINDEX  <collation>               -- 2
///        REINDEX  ?<database>.?<indexname>  -- 3
///        REINDEX  ?<database>.?<tablename>  -- 4
///        REINDEX  EXPRESSIONS               -- 5
///
/// Form 1 causes all indexes in all attached databases to be rebuilt.
/// Form 2 rebuilds all indexes in all databases that use the named
/// collating function.  Forms 3 and 4 rebuild the named index or all
/// indexes associated with the named table, respectively.  Form 5
/// rebuilds all expression indexes in addition to all collations,
/// indexes, or tables named "EXPRESSIONS".
///
/// If the name is ambiguous such that it matches two or more of
/// forms 2 through 5, then rebuild the union of all matching indexes,
/// taken care to avoid rebuilding the same index more than once.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3Reindex(
    mut pParse: *mut Parse,
    mut pName1: *mut Token,
    mut pName2: *mut Token,
) {
    let mut z: *mut i8 = std::ptr::null_mut::<i8>(); // Name of a table or index or collation
    let mut zDb: *const i8 = std::ptr::null::<i8>(); // Name of the database
    let mut iReDb: i32 = -(1 as i32); // The database index number
    let mut db: *mut sqlite3 = unsafe { (*pParse).db }; // The database connection
    let mut pObjName: *mut Token = unsafe { std::mem::zeroed() }; // Name of the table or index to be reindexed
    let mut bMatch: i32 = 0 as i32; // At least one name match
    let mut zColl: *const i8 = std::ptr::null::<i8>(); // Rebuild indexes using this collation
    let mut pReTab: *mut Table = std::ptr::null_mut::<Table>(); // Rebuild all indexes of this table
    let mut pReIndex: *mut Index = std::ptr::null_mut::<Index>(); // Rebuild this index
    let mut isExprIdx: i32 = 0 as i32; // Rebuild all expression indexes
    let mut bAll: i32 = 0 as i32; // Rebuild all indexes
    // Read the database schema. If an error occurs, leave an error message
    // and code in pParse and return NULL.
    if (0 as i32) != unsafe { sqlite3ReadSchema(pParse) } {
        return;
    }
    if pName1 == std::ptr::null_mut::<Token>() {
        // rebuild all indexes
        bMatch = 1 as i32;
        bAll = 1 as i32;
    } else {
        if pName2 == std::ptr::null_mut::<Token>()
            || (unsafe { (*pName2).z }) == std::ptr::null::<i8>()
        {
            0 as i32;
            z = sqlite3NameFromToken(unsafe { (*pParse).db }, pName1 as *const Token);
            if z == std::ptr::null_mut::<i8>() {
                return;
            }
        } else {
            iReDb = sqlite3TwoPartName(pParse, pName1, pName2, std::ptr::addr_of_mut!(pObjName));
            if iReDb < (0 as i32) {
                return;
            }
            z = sqlite3NameFromToken(db, pObjName as *const Token);
            if z == std::ptr::null_mut::<i8>() {
                return;
            }
            zDb = (unsafe { (*unsafe { unsafe { (*db).aDb }.offset(iReDb as isize) }).zDbSName })
                as *const i8;
        }
    }
    if !(bAll != (0 as i32)) {
        let __v2618: bool;
        if zDb == std::ptr::null::<i8>() {
            __v2618 = (unsafe {
                sqlite3StrICmp(
                    z as *const i8,
                    (b"expressions\0".as_ptr() as *mut i8) as *const i8,
                )
            }) == (0 as i32);
        } else {
            __v2618 = false as bool;
        }
        if __v2618 {
            isExprIdx = 1 as i32;
            bMatch = 1 as i32;
        }
        let __v2619: bool;
        if zDb == std::ptr::null::<i8>() {
            __v2619 =
                (unsafe { sqlite3FindCollSeq(db, unsafe { (*db).enc }, z as *const i8, 0 as i32) })
                    != std::ptr::null_mut::<CollSeq>();
        } else {
            __v2619 = false as bool;
        }
        if __v2619 {
            zColl = z as *const i8;
            bMatch = 1 as i32;
        }
        let __v2620: bool;
        if zColl == std::ptr::null::<i8>() {
            let __v2621: *mut Table = sqlite3FindTable(db, z as *const i8, zDb);
            pReTab = __v2621;
            __v2620 = __v2621 != std::ptr::null_mut::<Table>();
        } else {
            __v2620 = false as bool;
        }
        if __v2620 {
            bMatch = 1 as i32;
        }
        let __v2622: bool;
        if zColl == std::ptr::null::<i8>() {
            let __v2623: *mut Index = sqlite3FindIndex(db, z as *const i8, zDb);
            pReIndex = __v2623;
            __v2622 = __v2623 != std::ptr::null_mut::<Index>();
        } else {
            __v2622 = false as bool;
        }
        if __v2622 {
            bMatch = 1 as i32;
        }
    }
    if bMatch != (0 as i32) {
        let mut iDb: i32 = 0 as i32;
        let mut k: *mut HashElem = unsafe { std::mem::zeroed() };
        let mut pTab: *mut Table = unsafe { std::mem::zeroed() };
        let mut pIdx: *mut Index = unsafe { std::mem::zeroed() };
        let mut pDb: *mut Db = unsafe { std::mem::zeroed() };
        iDb = 0 as i32;
        let __v2624: *mut Db = unsafe { (*db).aDb };
        pDb = __v2624;
        '__slate_break_2253: while iDb < unsafe { (*db).nDb } {
            0 as i32;
            if iReDb >= (0 as i32) && iReDb != iDb {
            } else {
                k = unsafe {
                    (*unsafe { std::ptr::addr_of_mut!((*unsafe { (*pDb).pSchema }).tblHash) }).first
                };
                '__slate_break_2254: while k != std::ptr::null_mut::<HashElem>() {
                    pTab = (unsafe { (*k).data }) as *mut Table;
                    if (((unsafe { (*pTab).eTabType }) as u32) as i32) == (1 as i32) {
                    } else {
                        pIdx = unsafe { (*pTab).pIndex };
                        '__slate_break_2255: while pIdx != std::ptr::null_mut::<Index>() {
                            let __v2629: bool;
                            if bAll != (0 as i32)
                                || pTab == pReTab
                                || pIdx == pReIndex
                                || isExprIdx != (0 as i32)
                                    && ((unsafe { (*pIdx).__slate_bits_0.__get_bHasExpr() }) as i32)
                                        != (0 as i32)
                            {
                                __v2629 = true as bool;
                            } else {
                                let __v2630: bool;
                                if zColl != std::ptr::null::<i8>() {
                                    __v2630 = collationMatch(zColl, pIdx) != (0 as i32);
                                } else {
                                    __v2630 = false as bool;
                                }
                                __v2629 = __v2630;
                            }
                            if __v2629 {
                                sqlite3BeginWriteOperation(pParse, 0 as i32, iDb);
                                sqlite3RefillIndex(pParse, pIdx, -(1 as i32));
                            }
                            pIdx = unsafe { (*pIdx).pNext };
                        }
                        // End loop over indexes of pTab
                    }
                    k = unsafe { (*k).next };
                }
                // End loop over tables of iDb
            }
            let __v2625: i32 = iDb;
            let __v2626: i32 = __v2625 + (1 as i32);
            iDb = __v2626;
            let __v2627: *mut Db = pDb;
            let __v2628: *mut Db = unsafe { __v2627.offset((1 as i32) as isize) };
            pDb = __v2628;
        }
    // End loop over databases
    } else {
        unsafe {
            sqlite3ErrorMsg(
                pParse,
                (b"unable to identify the object to be reindexed\0".as_ptr() as *mut i8)
                    as *const i8,
            )
        };
    }
    unsafe { sqlite3DbFree(db, z as *mut ()) };
    return;
}

/// Return a KeyInfo structure that is appropriate for the given Index.
///
/// The caller should invoke sqlite3KeyInfoUnref() on the returned object
/// when it has finished using it.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3KeyInfoOfIndex(
    mut pParse: *mut Parse,
    mut pIdx: *mut Index,
) -> *mut KeyInfo {
    let mut i: i32 = 0 as i32;
    let mut nCol: i32 = ((unsafe { (*pIdx).nColumn }) as u32) as i32;
    let mut nKey: i32 = ((unsafe { (*pIdx).nKeyCol }) as u32) as i32;
    let mut pKey: *mut KeyInfo = unsafe { std::mem::zeroed() };
    if (unsafe { (*pParse).nErr }) != (0 as i32) {
        return std::ptr::null_mut::<KeyInfo>();
    }
    if ((unsafe { (*pIdx).__slate_bits_0.__get_uniqNotNull() }) as i32) != (0 as i32) {
        pKey = unsafe { sqlite3KeyInfoAlloc(unsafe { (*pParse).db }, nKey, nCol - nKey) };
    } else {
        pKey = unsafe { sqlite3KeyInfoAlloc(unsafe { (*pParse).db }, nCol, 0 as i32) };
    }
    if pKey != std::ptr::null_mut::<KeyInfo>() {
        0 as i32;
        i = 0 as i32;
        '__slate_break_2257: loop {
            if !(i < nCol) {
                break;
            }
            let mut zColl: *const i8 =
                unsafe { *unsafe { unsafe { (*pIdx).azColl }.offset(i as isize) } };
            let __v2659: *mut CollSeq;
            if zColl == unsafe { std::ptr::addr_of!(sqlite3StrBINARY) as *const i8 } {
                __v2659 = std::ptr::null_mut::<CollSeq>();
            } else {
                __v2659 = unsafe { sqlite3LocateCollSeq(pParse, zColl) };
            }
            unsafe {
                *unsafe {
                    unsafe { std::ptr::addr_of_mut!((*pKey).aColl) as *mut *mut CollSeq }
                        .offset(i as isize)
                } = __v2659;
            }
            unsafe {
                *unsafe { unsafe { (*pKey).aSortFlags }.offset(i as isize) } =
                    unsafe { *unsafe { unsafe { (*pIdx).aSortOrder }.offset(i as isize) } };
            }
            0 as i32;
            let __v2657: i32 = i;
            let __v2658: i32 = __v2657 + (1 as i32);
            i = __v2658;
        }
        if (unsafe { (*pParse).nErr }) != (0 as i32) {
            0 as i32;
            let __v2660: bool;
            if ((unsafe { (*pIdx).__slate_bits_0.__get_bNoQuery() }) as i32) == (0 as i32) {
                __v2660 = (unsafe {
                    sqlite3HashFind(
                        (unsafe { std::ptr::addr_of_mut!((*unsafe { (*pIdx).pSchema }).idxHash) })
                            as *const Hash,
                        (unsafe { (*pIdx).zName }) as *const i8,
                    )
                }) != std::ptr::null_mut::<()>();
            } else {
                __v2660 = false as bool;
            }
            if __v2660 {
                // Deactivate the index because it contains an unknown collating
                // sequence.  The only way to reactive the index is to reload the
                // schema.  Adding the missing collating sequence later does not
                // reactive the index.  The application had the chance to register
                // the missing index using the collation-needed callback.  For
                // simplicity, SQLite will not give the application a second chance.
                //
                // Except, do not do this if the index is not in the schema hash
                // table. In this case the index is currently being constructed
                // by a CREATE INDEX statement, and retrying will not help.
                unsafe {
                    (*pIdx).__slate_bits_0.__set_bNoQuery((1 as i32) as u32);
                }
                unsafe {
                    (*pParse).rc = (1 as i32) | (2 as i32) << (8 as i32);
                }
            }
            unsafe { sqlite3KeyInfoUnref(pKey) };
            pKey = std::ptr::null_mut::<KeyInfo>();
        }
    }
    return pKey;
}

/// Create a new CTE object
///
/// # Arguments
///
/// * `pParse` - Parsing context
/// * `pName` - Name of the common-table
/// * `pArglist` - Optional column name list for the table
/// * `pQuery` - Query used to initialize the table
/// * `eM10d` - The MATERIALIZED flag
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3CteNew(
    mut pParse: *mut Parse,
    mut pName: *mut Token,
    mut pArglist: *mut ExprList,
    mut pQuery: *mut Select,
    mut eM10d: u8,
) -> *mut Cte {
    let mut pNew: *mut Cte = unsafe { std::mem::zeroed() };
    let mut db: *mut sqlite3 = unsafe { (*pParse).db };
    pNew = (unsafe { sqlite3DbMallocZero(db, 48 as u64) }) as *mut Cte;
    0 as i32;
    if (unsafe { (*db).mallocFailed }) != (0 as u8) {
        unsafe { sqlite3ExprListDelete(db, pArglist) };
        unsafe { sqlite3SelectDelete(db, pQuery) };
    } else {
        unsafe {
            (*pNew).pSelect = pQuery;
        }
        unsafe {
            (*pNew).pCols = pArglist;
        }
        unsafe {
            (*pNew).zName = sqlite3NameFromToken(unsafe { (*pParse).db }, pName as *const Token);
        }
        unsafe {
            (*pNew).eM10d = eM10d;
        }
    }
    return pNew;
}

/// Clear information from a Cte object, but do not deallocate storage
/// for the object itself.
fn cteClear(mut db: *mut sqlite3, mut pCte: *mut Cte) {
    0 as i32;
    unsafe { sqlite3ExprListDelete(db, unsafe { (*pCte).pCols }) };
    unsafe { sqlite3SelectDelete(db, unsafe { (*pCte).pSelect }) };
    unsafe { sqlite3DbFree(db, (unsafe { (*pCte).zName }) as *mut ()) };
}

/// Free the contents of the CTE object passed as the second argument.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3CteDelete(mut db: *mut sqlite3, mut pCte: *mut Cte) {
    0 as i32;
    cteClear(db, pCte);
    unsafe { sqlite3DbFree(db, pCte as *mut ()) };
}

/// This routine is invoked once per CTE by the parser while parsing a
/// WITH clause.  The CTE described by the third argument is added to
/// the WITH clause of the second argument.  If the second argument is
/// NULL, then a new WITH argument is created.
///
/// # Arguments
///
/// * `pParse` - Parsing context
/// * `pWith` - Existing WITH clause, or NULL
/// * `pCte` - CTE to add to the WITH clause
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3WithAdd(
    mut pParse: *mut Parse,
    mut pWith: *mut With,
    mut pCte: *mut Cte,
) -> *mut With {
    let mut db: *mut sqlite3 = unsafe { (*pParse).db };
    let mut pNew: *mut With = unsafe { std::mem::zeroed() };
    let mut zName: *mut i8 = unsafe { std::mem::zeroed() };
    if pCte == std::ptr::null_mut::<Cte>() {
        return pWith;
    }
    // Check that the CTE name is unique within this WITH clause. If
    // not, store an error in the Parse structure.
    zName = unsafe { (*pCte).zName };
    if zName != std::ptr::null_mut::<i8>() && pWith != std::ptr::null_mut::<With>() {
        let mut i: i32 = 0 as i32;
        i = 0 as i32;
        '__slate_break_2258: loop {
            if !(i < unsafe { (*pWith).nCte }) {
                break;
            }
            if (unsafe {
                sqlite3StrICmp(
                    zName as *const i8,
                    (unsafe {
                        (*unsafe {
                            unsafe { std::ptr::addr_of_mut!((*pWith).a) as *mut Cte }
                                .offset(i as isize)
                        })
                        .zName
                    }) as *const i8,
                )
            }) == (0 as i32)
            {
                unsafe {
                    sqlite3ErrorMsg(
                        pParse,
                        (b"duplicate WITH table name: %s\0".as_ptr() as *mut i8) as *const i8,
                        zName,
                    )
                };
            }
            let __v2668: i32 = i;
            let __v2669: i32 = __v2668 + (1 as i32);
            i = __v2669;
        }
    }
    if pWith != std::ptr::null_mut::<With>() {
        pNew = (unsafe {
            sqlite3DbRealloc(
                db,
                pWith as *mut (),
                (16 as u64).wrapping_add(
                    ((((unsafe { (*pWith).nCte }) + (1 as i32)) as i64) as u64)
                        .wrapping_mul(48 as u64),
                ),
            )
        }) as *mut With;
    } else {
        pNew = (unsafe {
            sqlite3DbMallocZero(
                db,
                (16 as u64).wrapping_add((((1 as i32) as i64) as u64).wrapping_mul(48 as u64)),
            )
        }) as *mut With;
    }
    0 as i32;
    if (unsafe { (*db).mallocFailed }) != (0 as u8) {
        sqlite3CteDelete(db, pCte);
        pNew = pWith;
    } else {
        let __v2670: *mut With = pNew;
        let __v2671: i32 = unsafe { (*__v2670).nCte };
        let __v2672: i32 = __v2671 + (1 as i32);
        unsafe {
            (*__v2670).nCte = __v2672;
        }
        unsafe {
            *unsafe {
                unsafe { std::ptr::addr_of_mut!((*pNew).a) as *mut Cte }.offset(__v2671 as isize)
            } = unsafe { *pCte };
        }
        unsafe { sqlite3DbFree(db, pCte as *mut ()) };
    }
    return pNew;
}

/// Free the contents of the With object passed as the second argument.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3WithDelete(mut db: *mut sqlite3, mut pWith: *mut With) {
    if pWith != std::ptr::null_mut::<With>() {
        let mut i: i32 = 0 as i32;
        i = 0 as i32;
        '__slate_break_2260: loop {
            if !(i < unsafe { (*pWith).nCte }) {
                break;
            }
            cteClear(db, unsafe {
                unsafe { std::ptr::addr_of_mut!((*pWith).a) as *mut Cte }.offset(i as isize)
            });
            let __v2673: i32 = i;
            let __v2674: i32 = __v2673 + (1 as i32);
            i = __v2674;
        }
        unsafe { sqlite3DbFree(db, pWith as *mut ()) };
    }
}

#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.build.sqlite3WithDeleteGeneric")]
extern "C-unwind" fn sqlite3WithDeleteGeneric(mut db: *mut sqlite3, mut pWith: *mut ()) {
    sqlite3WithDelete(db, pWith as *mut With);
}
