unsafe extern "C" {
    static mut sqlite3StdTypeAffinity: [i8; 0];
    static mut sqlite3StdType: [*const i8; 0];
    static mut sqlite3CtypeMap: [u8; 0];
    fn sqlite3_randomness(N: i32, P: *mut ());
    fn sqlite3_stricmp(__v1558: *const i8, __v1559: *const i8) -> i32;
    fn sqlite3_strnicmp(__v1560: *const i8, __v1561: *const i8, __v1562: i32) -> i32;
    fn sqlite3HashInit(__v1563: *mut Hash);
    fn sqlite3HashInsert(__v1564: *mut Hash, pKey: *const i8, pData: *mut ()) -> *mut ();
    fn sqlite3HashFind(__v1567: *const Hash, pKey: *const i8) -> *mut ();
    fn sqlite3HashClear(__v1569: *mut Hash);
    fn memcpy(__dest: *mut (), __src: *const (), __n: u64) -> *mut ();
    fn memset(__s: *mut (), __c: i32, __n: u64) -> *mut ();
    fn strlen(__s: *const i8) -> u64;
    fn sqlite3VdbeCreate(__v1577: *mut Parse) -> *mut Vdbe;
    fn sqlite3VdbeAddOp0(__v1578: *mut Vdbe, __v1579: i32) -> i32;
    fn sqlite3VdbeAddOp1(__v1580: *mut Vdbe, __v1581: i32, __v1582: i32) -> i32;
    fn sqlite3VdbeAddOp2(__v1583: *mut Vdbe, __v1584: i32, __v1585: i32, __v1586: i32) -> i32;
    fn sqlite3VdbeGoto(__v1587: *mut Vdbe, __v1588: i32) -> i32;
    fn sqlite3VdbeAddOp3(
        __v1589: *mut Vdbe,
        __v1590: i32,
        __v1591: i32,
        __v1592: i32,
        __v1593: i32,
    ) -> i32;
    fn sqlite3VdbeAddOp4(
        __v1594: *mut Vdbe,
        __v1595: i32,
        __v1596: i32,
        __v1597: i32,
        __v1598: i32,
        zP4: *const i8,
        __v1600: i32,
    ) -> i32;
    fn sqlite3VdbeAddOp4Int(
        __v1601: *mut Vdbe,
        __v1602: i32,
        __v1603: i32,
        __v1604: i32,
        __v1605: i32,
        __v1606: i32,
    ) -> i32;
    fn sqlite3VdbeEndCoroutine(__v1607: *mut Vdbe, __v1608: i32);
    fn sqlite3VdbeExplain(__v1609: *mut Parse, __v1610: u8, __v1611: *const i8, ...) -> i32;
    fn sqlite3VdbeExplainPop(__v1612: *mut Parse);
    fn sqlite3VdbeChangeOpcode(__v1613: *mut Vdbe, addr: i32, __v1615: u8);
    fn sqlite3VdbeChangeP2(__v1616: *mut Vdbe, addr: i32, P2: i32);
    fn sqlite3VdbeChangeP5(__v1619: *mut Vdbe, P5: u16);
    fn sqlite3VdbeJumpHere(__v1621: *mut Vdbe, addr: i32);
    fn sqlite3VdbeJumpHereOrPopInst(__v1623: *mut Vdbe, addr: i32);
    fn sqlite3VdbeChangeToNoop(__v1625: *mut Vdbe, addr: i32) -> i32;
    fn sqlite3VdbeChangeP4(__v1627: *mut Vdbe, addr: i32, zP4: *const i8, N: i32);
    fn sqlite3VdbeAppendP4(__v1631: *mut Vdbe, pP4: *mut (), p4type: i32);
    fn sqlite3VdbeGetOp(__v1634: *mut Vdbe, __v1635: i32) -> *mut VdbeOp;
    fn sqlite3VdbeMakeLabel(__v1636: *mut Parse) -> i32;
    fn sqlite3VdbeResolveLabel(__v1637: *mut Vdbe, __v1638: i32);
    fn sqlite3VdbeCurrentAddr(__v1639: *mut Vdbe) -> i32;
    fn sqlite3VdbeSetNumCols(__v1640: *mut Vdbe, __v1641: i32);
    fn sqlite3VdbeSetColName(
        __v1642: *mut Vdbe,
        __v1643: i32,
        __v1644: i32,
        __v1645: *const i8,
        __v1646: Option<unsafe extern "C-unwind" fn(*mut ())>,
    ) -> i32;
    fn sqlite3WalkExpr(__v1647: *mut Walker, __v1648: *mut Expr) -> i32;
    fn sqlite3WalkExprNN(__v1649: *mut Walker, __v1650: *mut Expr) -> i32;
    fn sqlite3WalkExprList(__v1651: *mut Walker, __v1652: *mut ExprList) -> i32;
    fn sqlite3WalkSelect(__v1653: *mut Walker, __v1654: *mut Select) -> i32;
    fn sqlite3ExprWalkNoop(__v1655: *mut Walker, __v1656: *mut Expr) -> i32;
    fn sqlite3SelectWalkNoop(__v1657: *mut Walker, __v1658: *mut Select) -> i32;
    fn sqlite3WindowUnlinkFromSelect(__v1661: *mut Window);
    fn sqlite3WindowListDelete(db: *mut sqlite3, p: *mut Window);
    fn sqlite3WindowCodeInit(__v1664: *mut Parse, __v1665: *mut Select);
    fn sqlite3WindowCodeStep(
        __v1666: *mut Parse,
        __v1667: *mut Select,
        __v1668: *mut WhereInfo,
        __v1669: i32,
        __v1670: i32,
    );
    fn sqlite3WindowRewrite(__v1671: *mut Parse, __v1672: *mut Select) -> i32;
    fn sqlite3StrICmp(__v1673: *const i8, __v1674: *const i8) -> i32;
    fn sqlite3Strlen30(__v1675: *const i8) -> i32;
    fn sqlite3ColumnType(__v1676: *mut Column, __v1677: *mut i8) -> *mut i8;
    fn sqlite3DbMallocZero(__v1678: *mut sqlite3, __v1679: u64) -> *mut ();
    fn sqlite3DbMallocRawNN(__v1680: *mut sqlite3, __v1681: u64) -> *mut ();
    fn sqlite3DbStrDup(__v1682: *mut sqlite3, __v1683: *const i8) -> *mut i8;
    fn sqlite3DbReallocOrFree(__v1684: *mut sqlite3, __v1685: *mut (), __v1686: u64) -> *mut ();
    fn sqlite3DbFree(__v1687: *mut sqlite3, __v1688: *mut ());
    fn sqlite3DbFreeNN(__v1689: *mut sqlite3, __v1690: *mut ());
    fn sqlite3DbNNFreeNN(__v1691: *mut sqlite3, __v1692: *mut ());
    fn sqlite3MPrintf(__v1693: *mut sqlite3, __v1694: *const i8, ...) -> *mut i8;
    fn sqlite3ProgressCheck(__v1695: *mut Parse);
    fn sqlite3ErrorMsg(__v1696: *mut Parse, __v1697: *const i8, ...);
    fn sqlite3GetTempReg(__v1698: *mut Parse) -> i32;
    fn sqlite3ReleaseTempReg(__v1699: *mut Parse, __v1700: i32);
    fn sqlite3GetTempRange(__v1701: *mut Parse, __v1702: i32) -> i32;
    fn sqlite3ReleaseTempRange(__v1703: *mut Parse, __v1704: i32, __v1705: i32);
    fn sqlite3ClearTempRegCache(__v1706: *mut Parse);
    fn sqlite3Expr(__v1707: *mut sqlite3, __v1708: i32, __v1709: *const i8) -> *mut Expr;
    fn sqlite3ExprInt32(__v1710: *mut sqlite3, __v1711: i32) -> *mut Expr;
    fn sqlite3PExpr(
        __v1712: *mut Parse,
        __v1713: i32,
        __v1714: *mut Expr,
        __v1715: *mut Expr,
    ) -> *mut Expr;
    fn sqlite3PExprAddSelect(__v1716: *mut Parse, __v1717: *mut Expr, __v1718: *mut Select);
    fn sqlite3ExprAnd(__v1719: *mut Parse, __v1720: *mut Expr, __v1721: *mut Expr) -> *mut Expr;
    fn sqlite3ExprFunction(
        __v1722: *mut Parse,
        __v1723: *mut ExprList,
        __v1724: *const Token,
        __v1725: i32,
    ) -> *mut Expr;
    fn sqlite3ExprDelete(__v1726: *mut sqlite3, __v1727: *mut Expr);
    fn sqlite3ExprListAppend(
        __v1728: *mut Parse,
        __v1729: *mut ExprList,
        __v1730: *mut Expr,
    ) -> *mut ExprList;
    fn sqlite3ExprListDelete(__v1731: *mut sqlite3, __v1732: *mut ExprList);
    fn sqlite3ExprListDeleteGeneric(__v1733: *mut sqlite3, __v1734: *mut ());
    fn sqlite3ExprCanReturnSubtype(__v1735: *mut Parse, __v1736: *mut Expr) -> i32;
    fn sqlite3ColumnSetColl(__v1737: *mut sqlite3, __v1738: *mut Column, zColl: *const i8);
    fn sqlite3PrimaryKeyIndex(__v1753: *mut Table) -> *mut Index;
    fn sqlite3RowSetClear(__v1754: *mut ());
    fn sqlite3ViewGetColumnNames(__v1755: *mut Parse, __v1756: *mut Table) -> i32;
    fn sqlite3DeleteTable(__v1757: *mut sqlite3, __v1758: *mut Table);
    fn sqlite3DeleteTableGeneric(__v1759: *mut sqlite3, __v1760: *mut ());
    fn sqlite3IdListAppend(
        __v1761: *mut Parse,
        __v1762: *mut IdList,
        __v1763: *mut Token,
    ) -> *mut IdList;
    fn sqlite3IdListIndex(__v1764: *mut IdList, __v1765: *const i8) -> i32;
    fn sqlite3SrcListEnlarge(
        __v1766: *mut Parse,
        __v1767: *mut SrcList,
        __v1768: i32,
        __v1769: i32,
    ) -> *mut SrcList;
    fn sqlite3SrcListAppendList(
        pParse: *mut Parse,
        p1: *mut SrcList,
        p2: *mut SrcList,
    ) -> *mut SrcList;
    fn sqlite3SubqueryDetach(__v1773: *mut sqlite3, __v1774: *mut SrcItem) -> *mut Select;
    fn sqlite3SrcItemAttachSubquery(
        __v1775: *mut Parse,
        __v1776: *mut SrcItem,
        __v1777: *mut Select,
        __v1778: i32,
    ) -> i32;
    fn sqlite3SrcListAppendFromTerm(
        __v1779: *mut Parse,
        __v1780: *mut SrcList,
        __v1781: *mut Token,
        __v1782: *mut Token,
        __v1783: *mut Token,
        __v1784: *mut Select,
        __v1785: *mut OnOrUsing,
    ) -> *mut SrcList;
    fn sqlite3SrcListAssignCursors(__v1788: *mut Parse, __v1789: *mut SrcList);
    fn sqlite3IdListDelete(__v1790: *mut sqlite3, __v1791: *mut IdList);
    fn sqlite3SrcListDelete(__v1792: *mut sqlite3, __v1793: *mut SrcList);
    fn sqlite3WhereBegin(
        __v1812: *mut Parse,
        __v1813: *mut SrcList,
        __v1814: *mut Expr,
        __v1815: *mut ExprList,
        __v1816: *mut ExprList,
        __v1817: *mut Select,
        __v1818: u16,
        __v1819: i32,
    ) -> *mut WhereInfo;
    fn sqlite3WhereEnd(__v1820: *mut WhereInfo);
    fn sqlite3WhereOutputRowCount(__v1821: *mut WhereInfo) -> i16;
    fn sqlite3WhereIsDistinct(__v1822: *mut WhereInfo) -> i32;
    fn sqlite3WhereIsOrdered(__v1823: *mut WhereInfo) -> i32;
    fn sqlite3WhereOrderByLimitOptLabel(__v1824: *mut WhereInfo) -> i32;
    fn sqlite3WhereMinMaxOptEarlyOut(__v1825: *mut Vdbe, __v1826: *mut WhereInfo);
    fn sqlite3WhereIsSorted(__v1827: *mut WhereInfo) -> i32;
    fn sqlite3WhereContinueLabel(__v1828: *mut WhereInfo) -> i32;
    fn sqlite3WhereBreakLabel(__v1829: *mut WhereInfo) -> i32;
    fn sqlite3ExprCodeMove(__v1830: *mut Parse, __v1831: i32, __v1832: i32, __v1833: i32);
    fn sqlite3ExprToRegister(pExpr: *mut Expr, iReg: i32);
    fn sqlite3ExprCode(__v1836: *mut Parse, __v1837: *mut Expr, __v1838: i32);
    fn sqlite3ExprNullRegisterRange(__v1839: *mut Parse, __v1840: i32, __v1841: i32);
    fn sqlite3ExprCodeExprList(
        __v1842: *mut Parse,
        __v1843: *mut ExprList,
        __v1844: i32,
        __v1845: i32,
        __v1846: u8,
    ) -> i32;
    fn sqlite3ExprIfFalse(__v1847: *mut Parse, __v1848: *mut Expr, __v1849: i32, __v1850: i32);
    fn sqlite3LocateTableItem(__v1851: *mut Parse, flags: u32, __v1853: *mut SrcItem)
    -> *mut Table;
    fn sqlite3ExprListCompare(
        __v1854: *const ExprList,
        __v1855: *const ExprList,
        __v1856: i32,
    ) -> i32;
    fn sqlite3ExprImpliesNonNullRow(__v1857: *mut Expr, __v1858: i32, __v1859: i32) -> i32;
    fn sqlite3AggInfoPersistWalkerInit(__v1860: *mut Walker, __v1861: *mut Parse);
    fn sqlite3ExprAnalyzeAggregates(__v1862: *mut NameContext, __v1863: *mut Expr);
    fn sqlite3ExprAnalyzeAggList(__v1864: *mut NameContext, __v1865: *mut ExprList);
    fn sqlite3CodeVerifySchema(__v1867: *mut Parse, __v1868: i32);
    fn sqlite3IsTrueOrFalse(__v1869: *const i8) -> u32;
    fn sqlite3ExprTruthValue(__v1870: *const Expr) -> i32;
    fn sqlite3ExprIsConstant(__v1871: *mut Parse, __v1872: *mut Expr) -> i32;
    fn sqlite3ExprIsConstantOrGroupBy(
        __v1873: *mut Parse,
        __v1874: *mut Expr,
        __v1875: *mut ExprList,
    ) -> i32;
    fn sqlite3ExprIsSingleTableConstraint(
        __v1876: *mut Expr,
        __v1877: *const SrcList,
        __v1878: i32,
        __v1879: i32,
    ) -> i32;
    fn sqlite3ExprIsInteger(
        __v1880: *const Expr,
        __v1881: *mut i32,
        __v1882: *mut Parse,
        __v1883: i32,
    ) -> i32;
    fn sqlite3ExprCanBeNull(__v1884: *const Expr) -> i32;
    fn sqlite3RowidAlias(pTab: *mut Table) -> *const i8;
    fn sqlite3ExprDup(__v1886: *mut sqlite3, __v1887: *const Expr, __v1888: i32) -> *mut Expr;
    fn sqlite3ExprListDup(
        __v1889: *mut sqlite3,
        __v1890: *const ExprList,
        __v1891: i32,
    ) -> *mut ExprList;
    fn sqlite3SelectDup(__v1892: *mut sqlite3, __v1893: *const Select, __v1894: i32)
    -> *mut Select;
    fn sqlite3AuthCheck(
        __v1906: *mut Parse,
        __v1907: i32,
        __v1908: *const i8,
        __v1909: *const i8,
        __v1910: *const i8,
    ) -> i32;
    fn sqlite3LogEst(__v1911: u64) -> i16;
    fn sqlite3LogEstAdd(__v1912: i16, __v1913: i16) -> i16;
    fn sqlite3ExprAffinity(pExpr: *const Expr) -> i8;
    fn sqlite3ExprDataType(pExpr: *const Expr) -> i32;
    fn sqlite3IsBinary(__v1916: *const CollSeq) -> i32;
    fn sqlite3ExprCollSeq(pParse: *mut Parse, pExpr: *const Expr) -> *mut CollSeq;
    fn sqlite3ExprNNCollSeq(pParse: *mut Parse, pExpr: *const Expr) -> *mut CollSeq;
    fn sqlite3ExprAddCollateString(
        __v1921: *const Parse,
        __v1922: *mut Expr,
        __v1923: *const i8,
    ) -> *mut Expr;
    fn sqlite3ExprSkipCollateAndLikely(__v1924: *mut Expr) -> *mut Expr;
    fn sqlite3MatchEName(
        __v1932: *const ExprList_item,
        __v1933: *const i8,
        __v1934: *const i8,
        __v1935: *const i8,
        __v1936: *mut i32,
    ) -> i32;
    fn sqlite3ExprColUsed(__v1937: *mut Expr) -> u64;
    fn sqlite3StrIHash(__v1938: *const i8) -> u8;
    fn sqlite3ResolveSelectNames(
        __v1939: *mut Parse,
        __v1940: *mut Select,
        __v1941: *mut NameContext,
    );
    fn sqlite3ResolveOrderGroupBy(
        __v1942: *mut Parse,
        __v1943: *mut Select,
        __v1944: *mut ExprList,
        __v1945: *const i8,
    ) -> i32;
    fn sqlite3RenameTokenRemap(__v1946: *mut Parse, pTo: *const (), pFrom: *const ());
    fn sqlite3AffinityType(__v1949: *const i8, __v1950: *mut Column) -> i8;
    fn sqlite3SchemaToIndex(db: *mut sqlite3, __v1952: *mut Schema) -> i32;
    fn sqlite3KeyInfoOfIndex(__v1958: *mut Parse, __v1959: *mut Index) -> *mut KeyInfo;
    fn sqlite3OomFault(__v1965: *mut sqlite3) -> *mut ();
    fn sqlite3CreateColumnExpr(
        __v1969: *mut sqlite3,
        __v1970: *mut SrcList,
        __v1971: i32,
        __v1972: i32,
    ) -> *mut Expr;
    fn sqlite3TableLock(
        __v1973: *mut Parse,
        __v1974: i32,
        __v1975: u32,
        __v1976: u8,
        __v1977: *const i8,
    );
    fn sqlite3ParserAddCleanup(
        __v1978: *mut Parse,
        __v1979: Option<unsafe extern "C-unwind" fn(*mut sqlite3, *mut ())>,
        __v1980: *mut (),
    ) -> *mut ();
    fn sqlite3ExprCompareCollSeq(__v1981: *mut Parse, __v1982: *const Expr) -> *mut CollSeq;
    fn sqlite3WithDelete(__v1983: *mut sqlite3, __v1984: *mut With);
    fn sqlite3WithDeleteGeneric(__v1985: *mut sqlite3, __v1986: *mut ());
    fn sqlite3SelectExprHeight(__v1990: *const Select) -> i32;
    fn sqlite3ExprSetErrorOffset(__v1991: *mut Expr, __v1992: i32);
    fn sqlite3ExprIsVector(pExpr: *const Expr) -> i32;
    fn sqlite3VectorErrorMsg(__v1994: *mut Parse, __v1995: *mut Expr);
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
    trace: __SlateRecord167,
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
    u1: __SlateRecord168,
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
    __slate_bits_0: __slate_bits::__SlateBits62U0,
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
    u: __SlateRecord178,
    pLeft: *mut Expr,
    pRight: *mut Expr,
    x: __SlateRecord179,
    nHeight: i32,
    iTable: i32,
    iColumn: i16,
    iAgg: i16,
    w: __SlateRecord180,
    pAggInfo: *mut AggInfo,
    y: __SlateRecord181,
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
    u: __SlateRecord169,
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
    __slate_bits_0: __slate_bits::__SlateBits88U0,
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
    uNC: __SlateRecord193,
    pNext: *mut NameContext,
    nRef: i32,
    nNcErr: i32,
    ncFlags: i32,
    nNestedSelect: u32,
    pWinSelect: *mut Select,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct OnOrUsing {
    pOn: *mut Expr,
    pUsing: *mut IdList,
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
    __slate_bits_0: __slate_bits::__SlateBits104U0,
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
    u1: __SlateRecord195,
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
    fg: __SlateRecord188,
    iCursor: i32,
    colUsed: u64,
    u1: __SlateRecord189,
    u2: __SlateRecord190,
    u3: __SlateRecord191,
    u4: __SlateRecord192,
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
    u: __SlateRecord170,
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
    u: __SlateRecord198,
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
    __slate_bits_0: __slate_bits::__SlateBits166U0,
    azInit: *mut *const i8,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord167 {
    xLegacy: Option<unsafe extern "C-unwind" fn(*mut (), *const i8)>,
    xV2: Option<unsafe extern "C-unwind" fn(u32, *mut (), *mut (), *mut ()) -> i32>,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord168 {
    isInterrupted: i32,
    notUsed1: f64,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord169 {
    pHash: *mut FuncDef,
    pDestructor: *mut FuncDestructor,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord170 {
    tab: __SlateRecord171,
    view: __SlateRecord172,
    vtab: __SlateRecord173,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord171 {
    addColOffset: i32,
    pFKey: *mut FKey,
    pDfltList: *mut ExprList,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord172 {
    pSelect: *mut Select,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord173 {
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
union __SlateRecord178 {
    zToken: *mut i8,
    iValue: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord179 {
    pList: *mut ExprList,
    pSelect: *mut Select,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord180 {
    iJoin: i32,
    iOfst: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord181 {
    pTab: *mut Table,
    pWin: *mut Window,
    nReg: i32,
    sub: __SlateRecord182,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord182 {
    iAddr: i32,
    regReturn: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct ExprList_item {
    pExpr: *mut Expr,
    zEName: *mut i8,
    fg: __SlateRecord184,
    u: __SlateRecord185,
}

#[repr(C, align(4))]
#[derive(Clone, Copy)]
struct __SlateRecord184 {
    sortFlags: u8,
    __slate_bits_0: __slate_bits::__SlateBits184U0,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord185 {
    x: __SlateRecord186,
    iConstExprReg: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord186 {
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
struct __SlateRecord188 {
    jointype: u8,
    __slate_bits_0: __slate_bits::__SlateBits188U0,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord189 {
    zIndexedBy: *mut i8,
    pFuncArg: *mut ExprList,
    nRow: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord190 {
    pIBIndex: *mut Index,
    pCteUse: *mut CteUse,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord191 {
    pOn: *mut Expr,
    pUsing: *mut IdList,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord192 {
    pSchema: *mut Schema,
    zDatabase: *mut i8,
    pSubq: *mut Subquery,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord193 {
    pEList: *mut ExprList,
    pAggInfo: *mut AggInfo,
    pUpsert: *mut Upsert,
    iBaseReg: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord195 {
    cr: __SlateRecord196,
    d: __SlateRecord197,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord196 {
    addrCrTab: i32,
    regRowid: i32,
    regRoot: i32,
    constraintName: Token,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord197 {
    pReturning: *mut Returning,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord198 {
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
// ** 2001 September 15
// **
// ** The author disclaims copyright to this source code.  In place of
// ** a legal notice, here is a blessing:
// **
// **    May you do good and not evil.
// **    May you find forgiveness for yourself and forgive others.
// **    May you share freely, never taking more than you give.
// **
// *************************************************************************
// ** This file contains C code routines that are called by the parser
// ** to handle SELECT statements in SQLite.
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

// /* Parsing context */
// /* The parent or outer SELECT statement */
// /* Index in p->pSrc->a[] of the inner subquery */
// /* True if outer SELECT uses aggregate functions */
// /* !defined(SQLITE_OMIT_SUBQUERY) || !defined(SQLITE_OMIT_VIEW) */
// /*
// ** A structure to keep track of all of the column values that are fixed to
// ** a known value due to WHERE clause constraints of the form COLUMN=VALUE.
// */
#[repr(C)]
#[derive(Clone, Copy)]
struct WhereConst {
    pParse: *mut Parse,
    // /* Parsing context */
    pOomFault: *mut u8,
    // /* Pointer to pParse->db->mallocFailed */
    nConst: i32,
    // /* Number for COLUMN=CONSTANT terms */
    nChng: i32,
    // /* Number of times a constant is propagated */
    bHasAffBlob: i32,
    // /* At least one column in apExpr[] as affinity BLOB */
    mExcludeOn: u32,
    // /* Which ON expressions to exclude from considertion.
    //                    ** Either EP_OuterON or EP_InnerON|EP_OuterON */
    apExpr: *mut *mut Expr,
    // /* [i*2] is COLUMN and [i*2+1] is VALUE */
}

#[repr(C)]
#[derive(Clone, Copy)]
struct RenameCtx {}

#[repr(C)]
#[derive(Clone, Copy)]
struct CoveringIndexCheck {}

// /* Parsing context */
// /* The SELECT statement being optimized */
// /* part of the WHERE clause currently being examined */
// /*
// ** Type used for Walker callbacks by selectCheckOnClauses().
// */
#[repr(C)]
#[derive(Clone, Copy)]
struct CheckOnCtx {
    pSrc: *mut SrcList,
    // /* SrcList for this context */
    iJoin: i32,
    // /* Cursors must be left of this one, if not zero */
    bFuncArg: i32,
    // /* True for table-function arg */
    pParent: *mut CheckOnCtx,
    // /* Parent context */
}

// /*
// ** An instance of the following object is used to record information about
// ** how to process the DISTINCT keyword, to simplify passing that information
// ** into the selectInnerLoop() routine.
// */
#[repr(C)]
#[derive(Clone, Copy)]
struct DistinctCtx {
    isTnct: u8,
    // /* 0: Not distinct. 1: DISTINCT  2: DISTINCT and ORDER BY */
    eTnctType: u8,
    // /* One of the WHERE_DISTINCT_* operators */
    tabTnct: i32,
    // /* Ephemeral table used for DISTINCT processing */
    addrTnct: i32,
}

// /* Address of OP_OpenEphemeral opcode for tabTnct */
// /*
// ** An instance of the following object is used to record information about
// ** the ORDER BY (or GROUP BY) clause of query is being coded.
// **
// ** The aDefer[] array is used by the sorter-references optimization. For
// ** example, assuming there is no index that can be used for the ORDER BY,
// ** for the query:
// **
// **     SELECT a, bigblob FROM t1 ORDER BY a LIMIT 10;
// **
// ** it may be more efficient to add just the "a" values to the sorter, and
// ** retrieve the associated "bigblob" values directly from table t1 as the
// ** 10 smallest "a" values are extracted from the sorter.
// **
// ** When the sorter-reference optimization is used, there is one entry in the
// ** aDefer[] array for each database table that may be read as values are
// ** extracted from the sorter.
// */
#[repr(C)]
#[derive(Clone, Copy)]
struct SortCtx {
    pOrderBy: *mut ExprList,
    // /* The ORDER BY (or GROUP BY clause) */
    nOBSat: i32,
    // /* Number of ORDER BY terms satisfied by indices */
    iECursor: i32,
    // /* Cursor number for the sorter */
    regReturn: i32,
    // /* Register holding block-output return address */
    labelBkOut: i32,
    // /* Start label for the block-output subroutine */
    addrSortIndex: i32,
    // /* Address of the OP_SorterOpen or OP_OpenEphemeral */
    labelDone: i32,
    // /* Jump here when done, ex: LIMIT reached */
    labelOBLopt: i32,
    // /* Jump here when sorter is full */
    sortFlags: u8,
    // /* Zero or more SORTFLAG_* bits */
    pDeferredRowLoad: *mut RowLoadInfo,
}

// /*
// ** An instance of this object holds information (beyond pParse and pSelect)
// ** needed to load the next result row that is to be added to the sorter.
// */
#[repr(C)]
#[derive(Clone, Copy)]
struct RowLoadInfo {
    regResult: i32,
    // /* Store results in array of registers here */
    ecelFlags: u8,
    // /* Flag argument to ExprCodeExprList() */
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord212 {
    i: u8,
    // /* Beginning of keyword text in zKeyText[] */
    nChar: u8,
    // /* Length of the keyword in characters */
    code: u8,
    // /* Join type mask */
}

// /* Parsing context */
// /* The right-most of SELECTs to be coded */
// /* What to do with query results */
// /* An instance of the SubstContext object describes an substitution edit
// ** to be performed on a parse tree.
// **
// ** All references to columns in table iTable are to be replaced by corresponding
// ** expressions in pEList.
// **
// ** ## About "isOuterJoin":
// **
// ** The isOuterJoin column indicates that the replacement will occur into a
// ** position in the parent that is NULL-able due to an OUTER JOIN.  Either the
// ** target slot in the parent is the right operand of a LEFT JOIN, or one of
// ** the left operands of a RIGHT JOIN.  In either case, we need to potentially
// ** bypass the substituted expression with OP_IfNullRow.
// **
// ** Suppose the original expression is an integer constant. Even though the table
// ** has the nullRow flag set, because the expression is an integer constant,
// ** it will not be NULLed out.  So instead, we insert an OP_IfNullRow opcode
// ** that checks to see if the nullRow flag is set on the table.  If the nullRow
// ** flag is set, then the value in the register is set to NULL and the original
// ** expression is bypassed.  If the nullRow flag is not set, then the original
// ** expression runs to populate the register.
// **
// ** Example where this is needed:
// **
// **      CREATE TABLE t1(a INTEGER PRIMARY KEY, b INT);
// **      CREATE TABLE t2(x INT UNIQUE);
// **
// **      SELECT a,b,m,x FROM t1 LEFT JOIN (SELECT 59 AS m,x FROM t2) ON b=x;
// **
// ** When the subquery on the right side of the LEFT JOIN is flattened, we
// ** have to add OP_IfNullRow in front of the OP_Integer that implements the
// ** "m" value of the subquery so that a NULL will be loaded instead of 59
// ** when processing a non-matched row of the left.
// */
#[repr(C)]
#[derive(Clone, Copy)]
struct SubstContext {
    pParse: *mut Parse,
    // /* The parsing context */
    iTable: i32,
    // /* Replace references to this table */
    iNewTable: i32,
    // /* New table number */
    isOuterJoin: i32,
    // /* Add TK_IF_NULL_ROW opcodes on each replacement */
    nSelDepth: i32,
    // /* Depth of sub-query recursion.  Top==1 */
    pEList: *mut ExprList,
    // /* Replacement expressions */
    pCList: *mut ExprList,
    // /* Collation sequences for replacement expr */
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
    pub struct __SlateBits62U0 {
        #[bits(4)]
        pub notNull: u32,
        #[bits(4)]
        pub eCType: u32,
    }
    #[bitfields::bitfield([u8; 2], c_names = true, new = false, from_into_bits = false, from_traits = false, default = false, debug = false, builder = false, bit_ops = false)]
    pub struct __SlateBits184U0 {
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
    pub struct __SlateBits188U0 {
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
    pub struct __SlateBits166U0 {
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
    pub struct __SlateBits104U0 {
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

static mut zKeyText: __SlateAlign16<[i8; 34]> = __SlateAlign16([
    110 as i8, 97 as i8, 116 as i8, 117 as i8, 114 as i8, 97 as i8, 108 as i8, 101 as i8,
    102 as i8, 116 as i8, 111 as i8, 117 as i8, 116 as i8, 101 as i8, 114 as i8, 105 as i8,
    103 as i8, 104 as i8, 116 as i8, 102 as i8, 117 as i8, 108 as i8, 108 as i8, 105 as i8,
    110 as i8, 110 as i8, 101 as i8, 114 as i8, 99 as i8, 114 as i8, 111 as i8, 115 as i8,
    115 as i8, 0 as i8,
]);

static mut aKeyword: __SlateAlign16<[__SlateRecord212; 7]> = __SlateAlign16([
    __SlateRecord212 {
        i: ((0 as i32) as i8) as u8,
        nChar: ((7 as i32) as i8) as u8,
        code: ((4 as i32) as i8) as u8,
    },
    __SlateRecord212 {
        i: ((6 as i32) as i8) as u8,
        nChar: ((4 as i32) as i8) as u8,
        code: (((8 as i32) | (32 as i32)) as i8) as u8,
    },
    __SlateRecord212 {
        i: ((10 as i32) as i8) as u8,
        nChar: ((5 as i32) as i8) as u8,
        code: ((32 as i32) as i8) as u8,
    },
    __SlateRecord212 {
        i: ((14 as i32) as i8) as u8,
        nChar: ((5 as i32) as i8) as u8,
        code: (((16 as i32) | (32 as i32)) as i8) as u8,
    },
    __SlateRecord212 {
        i: ((19 as i32) as i8) as u8,
        nChar: ((4 as i32) as i8) as u8,
        code: (((8 as i32) | (16 as i32) | (32 as i32)) as i8) as u8,
    },
    __SlateRecord212 {
        i: ((23 as i32) as i8) as u8,
        nChar: ((5 as i32) as i8) as u8,
        code: ((1 as i32) as i8) as u8,
    },
    __SlateRecord212 {
        i: ((28 as i32) as i8) as u8,
        nChar: ((5 as i32) as i8) as u8,
        code: (((1 as i32) | (2 as i32)) as i8) as u8,
    },
]);

static mut tkCoalesce: Token = Token {
    z: (b"coalesce\0".as_ptr() as *mut i8) as *const i8,
    n: (8 as i32) as u32,
};

// /* The parsing context */
// /* Current tree walker */
// /* The FROM clause term to check */
// /*
// ** If the SELECT passed as the second argument has an associated WITH
// ** clause, pop it from the stack stored as part of the Parse object.
// **
// ** This function is used as the xSelectCallback2() callback by
// ** sqlite3SelectExpand() when walking a SELECT tree to resolve table
// ** names and other FROM clause elements.
// */
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.select.sqlite3SelectPopWith")]
extern "C-unwind" fn sqlite3SelectPopWith(mut pWalker: *mut Walker, mut p: *mut Select) {
    let mut pParse: *mut Parse = unsafe { (*pWalker).pParse };
    if (unsafe { (*pParse).pWith }) != std::ptr::null_mut::<With>()
        && (unsafe { (*p).pPrior }) == std::ptr::null_mut::<Select>()
    {
        let mut pWith: *mut With = unsafe { (*findRightmost(p)).pWith };
        if pWith != std::ptr::null_mut::<With>() {
            0 as i32;
            unsafe {
                (*pParse).pWith = unsafe { (*pWith).pOuter };
            }
        }
    }
}

// /* Parser context */
// /* List of tables */
// /* Expressions defining the result set */
// /*
// ** Compute the column names for a SELECT statement.
// **
// ** The only guarantee that SQLite makes about column names is that if the
// ** column has an AS clause assigning it a name, that will be the name used.
// ** That is the only documented guarantee.  However, countless applications
// ** developed over the years have made baseless assumptions about column names
// ** and will break if those assumptions changes.  Hence, use extreme caution
// ** when modifying this routine to avoid breaking legacy.
// **
// ** See Also: sqlite3ColumnsFromExprList()
// **
// ** The PRAGMA short_column_names and PRAGMA full_column_names settings are
// ** deprecated.  The default setting is short=ON, full=OFF.  99.9% of all
// ** applications should operate this way.  Nevertheless, we need to support the
// ** other modes for legacy:
// **
// **    short=OFF, full=OFF:      Column name is the text of the expression has it
// **                              originally appears in the SELECT statement.  In
// **                              other words, the zSpan of the result expression.
// **
// **    short=ON, full=OFF:       (This is the default setting).  If the result
// **                              refers directly to a table column, then the
// **                              result column name is just the table column
// **                              name: COLUMN.  Otherwise use zSpan.
// **
// **    full=ON, short=ANY:       If the result refers directly to a table column,
// **                              then the result column name with the table name
// **                              prefix, ex: TABLE.COLUMN.  Otherwise use zSpan.
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3GenerateColumnNames(mut pParse: *mut Parse, mut pSelect: *mut Select) {
    let mut v: *mut Vdbe = unsafe { (*pParse).pVdbe };
    let mut i: i32 = 0 as i32;
    let mut pTab: *mut Table = unsafe { std::mem::zeroed() };
    let mut pTabList: *mut SrcList = unsafe { std::mem::zeroed() };
    let mut pEList: *mut ExprList = unsafe { std::mem::zeroed() };
    let mut db: *mut sqlite3 = unsafe { (*pParse).db };
    // /* TABLE.COLUMN if no AS clause and is a direct table ref */
    let mut fullName: i32 = 0 as i32;
    // /* COLUMN or TABLE.COLUMN if no AS clause and is direct */
    let mut srcName: i32 = 0 as i32;
    if ((unsafe { (*pParse).__slate_bits_0.__get_colNamesSet() }) as i32) != (0 as i32) {
        return;
    }
    // /* Column names are determined by the left-most term of a compound select */
    '__slate_break_2045: while (unsafe { (*pSelect).pPrior }) != std::ptr::null_mut::<Select>() {
        pSelect = unsafe { (*pSelect).pPrior };
    }
    {}
    pTabList = unsafe { (*pSelect).pSrc };
    pEList = unsafe { (*pSelect).pEList };
    0 as i32;
    0 as i32;
    unsafe {
        (*pParse)
            .__slate_bits_0
            .__set_colNamesSet((1 as i32) as u32);
    }
    fullName = ((unsafe { (*db).flags }) & (((4 as i32) as i64) as u64)
        != (((0 as i32) as i64) as u64)) as i32;
    srcName = ((unsafe { (*db).flags }) & (((64 as i32) as i64) as u64)
        != (((0 as i32) as i64) as u64)
        || fullName != (0 as i32)) as i32;
    unsafe { sqlite3VdbeSetNumCols(v, unsafe { (*pEList).nExpr }) };
    i = 0 as i32;
    '__slate_break_2046: loop {
        if !(i < unsafe { (*pEList).nExpr }) {
            break;
        }
        let mut p: *mut Expr = unsafe {
            (*unsafe {
                unsafe { std::ptr::addr_of_mut!((*pEList).a) as *mut ExprList_item }
                    .offset(i as isize)
            })
            .pExpr
        };
        0 as i32;
        // /* Agg processing has not run yet */
        0 as i32;
        0 as i32;
        // /* Covering idx not yet coded */
        if (unsafe {
            (*unsafe {
                unsafe { std::ptr::addr_of_mut!((*pEList).a) as *mut ExprList_item }
                    .offset(i as isize)
            })
            .zEName
        }) != std::ptr::null_mut::<i8>()
            && ((unsafe {
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
            // /* An AS clause always takes first priority */
            let mut zName: *mut i8 = unsafe {
                (*unsafe {
                    unsafe { std::ptr::addr_of_mut!((*pEList).a) as *mut ExprList_item }
                        .offset(i as isize)
                })
                .zEName
            };
            unsafe {
                sqlite3VdbeSetColName(v, i, 0 as i32, zName as *const i8, unsafe {
                    std::mem::transmute::<usize, Option<unsafe extern "C-unwind" fn(*mut ())>>(
                        -(1 as i32) as usize,
                    )
                })
            };
        } else {
            if srcName != (0 as i32) && (((unsafe { (*p).op }) as u32) as i32) == (168 as i32) {
                let mut zCol: *mut i8 = unsafe { std::mem::zeroed() };
                let mut iCol: i32 = (unsafe { (*p).iColumn }) as i32;
                pTab = unsafe { (*p).y.pTab };
                0 as i32;
                if iCol < (0 as i32) {
                    iCol = (unsafe { (*pTab).iPKey }) as i32;
                }
                0 as i32;
                if iCol < (0 as i32) {
                    zCol = b"rowid\0".as_ptr() as *mut i8;
                } else {
                    zCol = unsafe {
                        (*unsafe { unsafe { (*pTab).aCol }.offset(iCol as isize) }).zCnName
                    };
                }
                if fullName != (0 as i32) {
                    let mut zName: *mut i8 = std::ptr::null_mut::<i8>();
                    zName = unsafe {
                        sqlite3MPrintf(
                            db,
                            (b"%s.%s\0".as_ptr() as *mut i8) as *const i8,
                            unsafe { (*pTab).zName },
                            zCol,
                        )
                    };
                    unsafe {
                        sqlite3VdbeSetColName(v, i, 0 as i32, zName as *const i8, unsafe {
                            std::mem::transmute::<
                                *const (),
                                Option<unsafe extern "C-unwind" fn(*mut ())>,
                            >(sqlite3RowSetClear as *const ())
                        })
                    };
                } else {
                    unsafe {
                        sqlite3VdbeSetColName(v, i, 0 as i32, zCol as *const i8, unsafe {
                            std::mem::transmute::<usize, Option<unsafe extern "C-unwind" fn(*mut ())>>(
                                -(1 as i32) as usize,
                            )
                        })
                    };
                }
            } else {
                let mut z: *const i8 = (unsafe {
                    (*unsafe {
                        unsafe { std::ptr::addr_of_mut!((*pEList).a) as *mut ExprList_item }
                            .offset(i as isize)
                    })
                    .zEName
                }) as *const i8;
                let __v2231: *mut i8;
                if z == std::ptr::null::<i8>() {
                    __v2231 = unsafe {
                        sqlite3MPrintf(
                            db,
                            (b"column%d\0".as_ptr() as *mut i8) as *const i8,
                            i + (1 as i32),
                        )
                    };
                } else {
                    __v2231 = unsafe { sqlite3DbStrDup(db, z) };
                }
                z = __v2231 as *const i8;
                unsafe {
                    sqlite3VdbeSetColName(v, i, 0 as i32, z, unsafe {
                        std::mem::transmute::<*const (), Option<unsafe extern "C-unwind" fn(*mut ())>>(
                            sqlite3RowSetClear as *const (),
                        )
                    })
                };
            }
        }
        let __v2229: i32 = i;
        let __v2230: i32 = __v2229 + (1 as i32);
        i = __v2230;
    }
    generateColumnTypes(pParse, pTabList, pEList);
}

// /* Parser context */
// /* Generate column names for this SELECT statement */
// /*
// ** Given an expression list (which is really the list of expressions
// ** that form the result set of a SELECT statement) compute appropriate
// ** column names for a table that would hold the expression list.
// **
// ** All column names will be unique.
// **
// ** Only the column names are computed.  Column.zType, Column.zColl,
// ** and other fields of Column are zeroed.
// **
// ** Return SQLITE_OK on success.  If a memory allocation error occurs,
// ** store NULL in *paCol and 0 in *pnCol and return SQLITE_NOMEM.
// **
// ** The only guarantee that SQLite makes about column names is that if the
// ** column has an AS clause assigning it a name, that will be the name used.
// ** That is the only documented guarantee.  However, countless applications
// ** developed over the years have made baseless assumptions about column names
// ** and will break if those assumptions changes.  Hence, use extreme caution
// ** when modifying this routine to avoid breaking legacy.
// **
// ** See Also: sqlite3GenerateColumnNames()
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3ColumnsFromExprList(
    mut pParse: *mut Parse,
    mut pEList: *mut ExprList,
    mut pnCol: *mut i16,
    mut paCol: *mut *mut Column,
) -> i32 {
    // /* Database connection */
    let mut db: *mut sqlite3 = unsafe { (*pParse).db };
    // /* Loop counters */
    let mut i: i32 = 0 as i32;
    let mut j: i32 = 0 as i32;
    // /* Index added to make the name unique */
    let mut cnt: u32 = 0 as u32;
    // /* For looping over result columns */
    let mut aCol: *mut Column = unsafe { std::mem::zeroed() };
    let mut pCol: *mut Column = unsafe { std::mem::zeroed() };
    // /* Number of columns in the result set */
    let mut nCol: i32 = 0 as i32;
    // /* Column name */
    let mut zName: *mut i8 = unsafe { std::mem::zeroed() };
    // /* Size of name in zName[] */
    let mut nName: i32 = 0 as i32;
    // /* Hash table of column names */
    let mut ht: Hash = unsafe { std::mem::zeroed() };
    let mut pTab: *mut Table = unsafe { std::mem::zeroed() };
    unsafe { sqlite3HashInit(std::ptr::addr_of_mut!(ht)) };
    if pEList != std::ptr::null_mut::<ExprList>() {
        nCol = unsafe { (*pEList).nExpr };
        aCol = (unsafe { sqlite3DbMallocZero(db, (16 as u64).wrapping_mul((nCol as i64) as u64)) })
            as *mut Column;
        {}
        if nCol > (32767 as i32) {
            nCol = 32767 as i32;
        }
    } else {
        nCol = 0 as i32;
        aCol = std::ptr::null_mut::<Column>();
    }
    0 as i32;
    unsafe {
        *pnCol = nCol as i16;
    }
    unsafe {
        *paCol = aCol;
    }
    i = 0 as i32;
    let __v2232: *mut Column = aCol;
    pCol = __v2232;
    '__slate_break_2050: while i < nCol && !((unsafe { (*pParse).nErr }) != (0 as i32)) {
        let mut pX: *mut ExprList_item = unsafe {
            unsafe { std::ptr::addr_of_mut!((*pEList).a) as *mut ExprList_item }.offset(i as isize)
        };
        let mut pCollide: *mut ExprList_item = unsafe { std::mem::zeroed() };
        // /* Get an appropriate name for the column
        //     */
        let __v2237: *mut i8 = unsafe { (*pX).zEName };
        zName = __v2237;
        if __v2237 != std::ptr::null_mut::<i8>()
            && ((unsafe { (*pX).fg.__slate_bits_0.__get_eEName() }) as i32) == (0 as i32)
        {
            // /* If the column contains an "AS <name>" phrase, use <name> as the name */
        } else {
            let mut pColExpr: *mut Expr =
                unsafe { sqlite3ExprSkipCollateAndLikely(unsafe { (*pX).pExpr }) };
            '__slate_break_2051: while pColExpr != std::ptr::null_mut::<Expr>()
                && (((unsafe { (*pColExpr).op }) as u32) as i32) == (142 as i32)
            {
                pColExpr = unsafe { (*pColExpr).pRight };
                0 as i32;
            }
            if (((unsafe { (*pColExpr).op }) as u32) as i32) == (168 as i32)
                && (unsafe { (*pColExpr).flags }) & (((16777216 as i32) | (33554432 as i32)) as u32)
                    == ((0 as i32) as u32)
                && (unsafe { (*pColExpr).y.pTab }) != std::ptr::null_mut::<Table>()
            {
                // /* For columns use the column name name */
                let mut iCol: i32 = (unsafe { (*pColExpr).iColumn }) as i32;
                pTab = unsafe { (*pColExpr).y.pTab };
                if iCol < (0 as i32) {
                    iCol = (unsafe { (*pTab).iPKey }) as i32;
                }
                zName = if iCol >= (0 as i32) {
                    unsafe { (*unsafe { unsafe { (*pTab).aCol }.offset(iCol as isize) }).zCnName }
                } else {
                    b"rowid\0".as_ptr() as *mut i8
                };
            } else {
                if (((unsafe { (*pColExpr).op }) as u32) as i32) == (60 as i32) {
                    0 as i32;
                    zName = unsafe { (*pColExpr).u.zToken };
                } else {
                    // /* Use the original text of the column expression as its name */
                    // /* pointer comparison intended */
                    0 as i32;
                }
            }
        }
        let __v2238: bool;
        if zName != std::ptr::null_mut::<i8>() {
            __v2238 = !((unsafe { sqlite3IsTrueOrFalse(zName as *const i8) }) != (0 as u32));
        } else {
            __v2238 = false as bool;
        }
        if __v2238 {
            zName = unsafe { sqlite3DbStrDup(db, zName as *const i8) };
        } else {
            zName = unsafe {
                sqlite3MPrintf(
                    db,
                    (b"column%d\0".as_ptr() as *mut i8) as *const i8,
                    i + (1 as i32),
                )
            };
        }
        // /* Make sure the column name is unique.  If the name is not unique,
        //     ** append an integer to the name so that it becomes unique.
        //     */
        cnt = (0 as i32) as u32;
        '__slate_break_2054: loop {
            let __v2239: bool;
            if zName != std::ptr::null_mut::<i8>() {
                let __v2240: *mut ExprList_item = (unsafe {
                    sqlite3HashFind(
                        std::ptr::addr_of_mut!(ht) as *const Hash,
                        zName as *const i8,
                    )
                }) as *mut ExprList_item;
                pCollide = __v2240;
                __v2239 = __v2240 != std::ptr::null_mut::<ExprList_item>();
            } else {
                __v2239 = false as bool;
            }
            if !__v2239 {
                break;
            }
            if ((unsafe { (*pCollide).fg.__slate_bits_0.__get_bUsingTerm() }) as i32) != (0 as i32)
            {
                let __v2241: *mut Column = pCol;
                let __v2242: u16 = unsafe { (*__v2241).colFlags };
                let __v2243: u16 = ((((__v2242 as u32) as i32) | (1024 as i32)) as i16) as u16;
                unsafe {
                    (*__v2241).colFlags = __v2243;
                }
            }
            nName = unsafe { sqlite3Strlen30(zName as *const i8) };
            if nName > (0 as i32) {
                j = nName - (1 as i32);
                '__slate_break_2055: loop {
                    if !(j > (0 as i32)
                        && (((unsafe {
                            *unsafe {
                                unsafe { std::ptr::addr_of!(sqlite3CtypeMap) as *const u8 }.offset(
                                    ((((unsafe { *unsafe { zName.offset(j as isize) } }) as u8)
                                        as u32) as i32)
                                        as isize,
                                )
                            }
                        }) as u32) as i32)
                            & (4 as i32)
                            != (0 as i32))
                    {
                        break;
                    }
                    let __v2244: i32 = j;
                    let __v2245: i32 = __v2244 - (1 as i32);
                    j = __v2245;
                }
                if ((unsafe { *unsafe { zName.offset(j as isize) } }) as i32) == (58 as i32) {
                    nName = j;
                }
            }
            let __v2246: u32 = cnt;
            let __v2247: u32 = __v2246.wrapping_add((1 as i32) as u32);
            cnt = __v2247;
            zName = unsafe {
                sqlite3MPrintf(
                    db,
                    (b"%.*z:%u\0".as_ptr() as *mut i8) as *const i8,
                    nName,
                    zName,
                    __v2247,
                )
            };
            unsafe { sqlite3ProgressCheck(pParse) };
            if cnt > ((3 as i32) as u32) {
                unsafe {
                    sqlite3_randomness(
                        ((4 as u64) as u32) as i32,
                        std::ptr::addr_of_mut!(cnt) as *mut (),
                    )
                };
            }
        }
        unsafe {
            (*pCol).zCnName = zName;
        }
        unsafe {
            (*pCol).hName = unsafe { sqlite3StrIHash(zName as *const i8) };
        }
        if ((unsafe { (*pX).fg.__slate_bits_0.__get_bNoExpand() }) as i32) != (0 as i32) {
            let __v2248: *mut Column = pCol;
            let __v2249: u16 = unsafe { (*__v2248).colFlags };
            let __v2250: u16 = ((((__v2249 as u32) as i32) | (1024 as i32)) as i16) as u16;
            unsafe {
                (*__v2248).colFlags = __v2250;
            }
        }
        {}
        let __v2251: bool;
        if zName != std::ptr::null_mut::<i8>() {
            __v2251 = (unsafe {
                sqlite3HashInsert(
                    std::ptr::addr_of_mut!(ht),
                    zName as *const i8,
                    pX as *mut (),
                )
            }) == (pX as *mut ());
        } else {
            __v2251 = false as bool;
        }
        if __v2251 {
            unsafe { sqlite3OomFault(db) };
        }
        let __v2233: i32 = i;
        let __v2234: i32 = __v2233 + (1 as i32);
        i = __v2234;
        let __v2235: *mut Column = pCol;
        let __v2236: *mut Column = unsafe { __v2235.offset((1 as i32) as isize) };
        pCol = __v2236;
    }
    unsafe { sqlite3HashClear(std::ptr::addr_of_mut!(ht)) };
    if (unsafe { (*pParse).nErr }) != (0 as i32) {
        j = 0 as i32;
        '__slate_break_2057: loop {
            if !(j < i) {
                break;
            }
            unsafe {
                sqlite3DbFree(
                    db,
                    (unsafe { (*unsafe { aCol.offset(j as isize) }).zCnName }) as *mut (),
                )
            };
            let __v2252: i32 = j;
            let __v2253: i32 = __v2252 + (1 as i32);
            j = __v2253;
        }
        unsafe { sqlite3DbFree(db, aCol as *mut ()) };
        unsafe {
            *paCol = std::ptr::null_mut::<Column>();
        }
        unsafe {
            *pnCol = (0 as i32) as i16;
        }
        return unsafe { (*pParse).rc };
    }
    return 0 as i32;
}

// /* Parsing context */
// /* Expr list from which to derive column names */
// /* Write the number of columns here */
// /* Write the new column list here */
// /*
// ** pTab is a transient Table object that represents a subquery of some
// ** kind (maybe a parenthesized subquery in the FROM clause of a larger
// ** query, or a VIEW, or a CTE).  This routine computes type information
// ** for that Table object based on the Select object that implements the
// ** subquery.  For the purposes of this routine, "type information" means:
// **
// **    *   The datatype name, as it might appear in a CREATE TABLE statement
// **    *   Which collating sequence to use for the column
// **    *   The affinity of the column
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3SubqueryColumnTypes(
    mut pParse: *mut Parse,
    mut pTab: *mut Table,
    mut pSelect: *mut Select,
    mut aff: i8,
) {
    let mut db: *mut sqlite3 = unsafe { (*pParse).db };
    let mut pCol: *mut Column = unsafe { std::mem::zeroed() };
    let mut pColl: *mut CollSeq = unsafe { std::mem::zeroed() };
    let mut i: i32 = 0 as i32;
    let mut j: i32 = 0 as i32;
    let mut p: *mut Expr = unsafe { std::mem::zeroed() };
    let mut a: *mut ExprList_item = unsafe { std::mem::zeroed() };
    let mut sNC: NameContext = unsafe { std::mem::zeroed() };
    0 as i32;
    0 as i32;
    0 as i32;
    0 as i32;
    if (unsafe { (*db).mallocFailed }) != (0 as u8)
        || (((unsafe { (*pParse).eParseMode }) as u32) as i32) >= (2 as i32)
    {
        return;
    }
    '__slate_break_2058: while (unsafe { (*pSelect).pPrior }) != std::ptr::null_mut::<Select>() {
        pSelect = unsafe { (*pSelect).pPrior };
    }
    a = unsafe { std::ptr::addr_of_mut!((*unsafe { (*pSelect).pEList }).a) as *mut ExprList_item };
    unsafe { memset(std::ptr::addr_of_mut!(sNC) as *mut (), 0 as i32, 56 as u64) };
    sNC.pSrcList = unsafe { (*pSelect).pSrc };
    i = 0 as i32;
    let __v2254: *mut Column = unsafe { (*pTab).aCol };
    pCol = __v2254;
    '__slate_break_2059: while i < ((unsafe { (*pTab).nCol }) as i32) {
        let mut zType: *const i8 = unsafe { std::mem::zeroed() };
        let mut n: i64 = 0 as i64;
        let mut m: i32 = 0 as i32;
        let mut pS2: *mut Select = pSelect;
        let __v2259: *mut Table = pTab;
        let __v2260: u32 = unsafe { (*__v2259).tabFlags };
        let __v2261: u32 =
            __v2260 | (((((unsafe { (*pCol).colFlags }) as u32) as i32) & (98 as i32)) as u32);
        unsafe {
            (*__v2259).tabFlags = __v2261;
        }
        p = unsafe { (*unsafe { a.offset(i as isize) }).pExpr };
        // /* pCol->szEst = ... // Column size est for SELECT tables never used */
        unsafe {
            (*pCol).affinity = unsafe { sqlite3ExprAffinity(p as *const Expr) };
        }
        '__slate_break_2060: while ((unsafe { (*pCol).affinity }) as i32) <= (64 as i32)
            && (unsafe { (*pS2).pNext }) != std::ptr::null_mut::<Select>()
        {
            let __v2262: i32 = m;
            let __v2263: i32 = __v2262
                | unsafe {
                    sqlite3ExprDataType(
                        (unsafe {
                            (*unsafe {
                                unsafe {
                                    std::ptr::addr_of_mut!((*unsafe { (*pS2).pEList }).a)
                                        as *mut ExprList_item
                                }
                                .offset(i as isize)
                            })
                            .pExpr
                        }) as *const Expr,
                    )
                };
            m = __v2263;
            pS2 = unsafe { (*pS2).pNext };
            unsafe {
                (*pCol).affinity = unsafe {
                    sqlite3ExprAffinity(
                        (unsafe {
                            (*unsafe {
                                unsafe {
                                    std::ptr::addr_of_mut!((*unsafe { (*pS2).pEList }).a)
                                        as *mut ExprList_item
                                }
                                .offset(i as isize)
                            })
                            .pExpr
                        }) as *const Expr,
                    )
                };
            }
        }
        if ((unsafe { (*pCol).affinity }) as i32) <= (64 as i32) {
            unsafe {
                (*pCol).affinity = aff;
            }
        }
        if ((unsafe { (*pCol).affinity }) as i32) >= (66 as i32)
            && ((unsafe { (*pS2).pNext }) != std::ptr::null_mut::<Select>() || pS2 != pSelect)
        {
            pS2 = unsafe { (*pS2).pNext };
            '__slate_break_2061: while pS2 != std::ptr::null_mut::<Select>() {
                let __v2264: i32 = m;
                let __v2265: i32 = __v2264
                    | unsafe {
                        sqlite3ExprDataType(
                            (unsafe {
                                (*unsafe {
                                    unsafe {
                                        std::ptr::addr_of_mut!((*unsafe { (*pS2).pEList }).a)
                                            as *mut ExprList_item
                                    }
                                    .offset(i as isize)
                                })
                                .pExpr
                            }) as *const Expr,
                        )
                    };
                m = __v2265;
                pS2 = unsafe { (*pS2).pNext };
            }
            if ((unsafe { (*pCol).affinity }) as i32) == (66 as i32) && m & (1 as i32) != (0 as i32)
            {
                unsafe {
                    (*pCol).affinity = (65 as i32) as i8;
                }
            } else {
                if ((unsafe { (*pCol).affinity }) as i32) >= (67 as i32)
                    && m & (2 as i32) != (0 as i32)
                {
                    unsafe {
                        (*pCol).affinity = (65 as i32) as i8;
                    }
                }
            }
            if ((unsafe { (*pCol).affinity }) as i32) >= (67 as i32)
                && (((unsafe { (*p).op }) as u32) as i32) == (36 as i32)
            {
                unsafe {
                    (*pCol).affinity = (70 as i32) as i8;
                }
            }
        }
        zType = columnTypeImpl(std::ptr::addr_of_mut!(sNC), p);
        let __v2266: bool;
        if zType == std::ptr::null::<i8>() {
            __v2266 = true as bool;
        } else {
            __v2266 = ((unsafe { (*pCol).affinity }) as i32)
                != ((unsafe { sqlite3AffinityType(zType, std::ptr::null_mut::<Column>()) }) as i32);
        }
        if __v2266 {
            if ((unsafe { (*pCol).affinity }) as i32) == (67 as i32)
                || ((unsafe { (*pCol).affinity }) as i32) == (70 as i32)
            {
                zType = (b"NUM\0".as_ptr() as *mut i8) as *const i8;
            } else {
                zType = std::ptr::null::<i8>();
                j = 1 as i32;
                '__slate_break_2063: loop {
                    if !(j < (6 as i32)) {
                        break;
                    }
                    if ((unsafe {
                        *unsafe {
                            unsafe { std::ptr::addr_of!(sqlite3StdTypeAffinity) as *const i8 }
                                .offset(j as isize)
                        }
                    }) as i32)
                        == ((unsafe { (*pCol).affinity }) as i32)
                    {
                        zType = unsafe {
                            *unsafe {
                                unsafe { std::ptr::addr_of_mut!(sqlite3StdType) as *mut *const i8 }
                                    .offset(j as isize)
                            }
                        };
                        break '__slate_break_2063;
                    }
                    let __v2267: i32 = j;
                    let __v2268: i32 = __v2267 + (1 as i32);
                    j = __v2268;
                }
            }
        }
        if zType != std::ptr::null::<i8>() {
            let mut k: i64 = (unsafe { strlen(zType) }) as i64;
            n = (unsafe { strlen((unsafe { (*pCol).zCnName }) as *const i8) }) as i64;
            unsafe {
                (*pCol).zCnName = (unsafe {
                    sqlite3DbReallocOrFree(
                        db,
                        (unsafe { (*pCol).zCnName }) as *mut (),
                        (n + k + ((2 as i32) as i64)) as u64,
                    )
                }) as *mut i8;
            }
            let __v2269: *mut Column = pCol;
            let __v2270: u16 = unsafe { (*__v2269).colFlags };
            let __v2271: u16 =
                ((((__v2270 as u32) as i32) & !((4 as i32) | (512 as i32))) as i16) as u16;
            unsafe {
                (*__v2269).colFlags = __v2271;
            }
            if (unsafe { (*pCol).zCnName }) != std::ptr::null_mut::<i8>() {
                unsafe {
                    memcpy(
                        (unsafe {
                            unsafe { (*pCol).zCnName }.offset((n + ((1 as i32) as i64)) as isize)
                        }) as *mut (),
                        zType as *const (),
                        (k + ((1 as i32) as i64)) as u64,
                    )
                };
                let __v2272: *mut Column = pCol;
                let __v2273: u16 = unsafe { (*__v2272).colFlags };
                let __v2274: u16 = ((((__v2273 as u32) as i32) | (4 as i32)) as i16) as u16;
                unsafe {
                    (*__v2272).colFlags = __v2274;
                }
            }
        }
        pColl = unsafe { sqlite3ExprCollSeq(pParse, p as *const Expr) };
        if pColl != std::ptr::null_mut::<CollSeq>() {
            0 as i32;
            unsafe { sqlite3ColumnSetColl(db, pCol, (unsafe { (*pColl).zName }) as *const i8) };
        }
        let __v2255: i32 = i;
        let __v2256: i32 = __v2255 + (1 as i32);
        i = __v2256;
        let __v2257: *mut Column = pCol;
        let __v2258: *mut Column = unsafe { __v2257.offset((1 as i32) as isize) };
        pCol = __v2258;
    }
    // /* Any non-zero value works */
    unsafe {
        (*pTab).szTabRow = (1 as i32) as i16;
    }
}

// /* Parsing contexts */
// /* Add column type information to this table */
// /* SELECT used to determine types and collations */
// /* Default affinity. */
// /*
// ** Given a SELECT statement, generate a Table structure that describes
// ** the result set of that SELECT.
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3ResultSetOfSelect(
    mut pParse: *mut Parse,
    mut pSelect: *mut Select,
    mut aff: i8,
) -> *mut Table {
    let mut pTab: *mut Table = unsafe { std::mem::zeroed() };
    let mut db: *mut sqlite3 = unsafe { (*pParse).db };
    let mut savedFlags: u64 = 0 as u64;
    let __v2275: *mut Parse = pParse;
    let __v2276: i32 = unsafe { (*__v2275).nNestSel };
    let __v2277: i32 = __v2276 + (1 as i32);
    unsafe {
        (*__v2275).nNestSel = __v2277;
    }
    if (unsafe { (*pParse).nNestSel })
        >= unsafe {
            *unsafe { unsafe { (*db).aLimit.as_mut_ptr() as *mut i32 }.offset((3 as i32) as isize) }
        }
    {
        unsafe {
            sqlite3ErrorMsg(
                pParse,
                (b"VIEWs and/or subqueries nested too deep\0".as_ptr() as *mut i8) as *const i8,
            )
        };
        return std::ptr::null_mut::<Table>();
    }
    savedFlags = unsafe { (*db).flags };
    let __v2278: *mut sqlite3 = db;
    let __v2279: u64 = unsafe { (*__v2278).flags };
    let __v2280: u64 = __v2279 & !(((4 as i32) as i64) as u64);
    unsafe {
        (*__v2278).flags = __v2280;
    }
    let __v2281: *mut sqlite3 = db;
    let __v2282: u64 = unsafe { (*__v2281).flags };
    let __v2283: u64 = __v2282 | (((64 as i32) as i64) as u64);
    unsafe {
        (*__v2281).flags = __v2283;
    }
    sqlite3SelectPrep(pParse, pSelect, std::ptr::null_mut::<NameContext>());
    unsafe {
        (*db).flags = savedFlags;
    }
    if (unsafe { (*pParse).nErr }) != (0 as i32) {
        return std::ptr::null_mut::<Table>();
    }
    '__slate_break_2065: while (unsafe { (*pSelect).pPrior }) != std::ptr::null_mut::<Select>() {
        pSelect = unsafe { (*pSelect).pPrior };
    }
    pTab = (unsafe { sqlite3DbMallocZero(db, 120 as u64) }) as *mut Table;
    if pTab == std::ptr::null_mut::<Table>() {
        return std::ptr::null_mut::<Table>();
    }
    unsafe {
        (*pTab).nTabRef = (1 as i32) as u32;
    }
    unsafe {
        (*pTab).zName = std::ptr::null_mut::<i8>();
    }
    unsafe {
        (*pTab).nRowLogEst = (200 as i32) as i16;
    }
    0 as i32;
    sqlite3ColumnsFromExprList(
        pParse,
        unsafe { (*pSelect).pEList },
        unsafe { std::ptr::addr_of_mut!((*pTab).nCol) },
        unsafe { std::ptr::addr_of_mut!((*pTab).aCol) },
    );
    sqlite3SubqueryColumnTypes(pParse, pTab, pSelect, aff);
    unsafe {
        (*pTab).iPKey = -(1 as i32) as i16;
    }
    if (unsafe { (*db).mallocFailed }) != (0 as u8) {
        unsafe { sqlite3DeleteTable(db, pTab) };
        return std::ptr::null_mut::<Table>();
    }
    let __v2284: *mut Parse = pParse;
    let __v2285: i32 = unsafe { (*__v2284).nNestSel };
    let __v2286: i32 = __v2285 - (1 as i32);
    unsafe {
        (*__v2284).nNestSel = __v2286;
    }
    0 as i32;
    return pTab;
}

// /*
// ** If the source-list item passed as an argument was augmented with an
// ** INDEXED BY clause, then try to locate the specified index. If there
// ** was such a clause and the named index cannot be found, return
// ** SQLITE_ERROR and leave an error in pParse. Otherwise, populate
// ** pFrom->pIndex and return SQLITE_OK.
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3IndexedByLookup(
    mut pParse: *mut Parse,
    mut pFrom: *mut SrcItem,
) -> i32 {
    let mut pTab: *mut Table = unsafe { (*pFrom).pSTab };
    let mut zIndexedBy: *mut i8 = unsafe { (*pFrom).u1.zIndexedBy };
    let mut pIdx: *mut Index = unsafe { std::mem::zeroed() };
    0 as i32;
    0 as i32;
    pIdx = unsafe { (*pTab).pIndex };
    '__slate_break_2137: loop {
        let __v2287: bool;
        if pIdx != std::ptr::null_mut::<Index>() {
            __v2287 = (unsafe {
                sqlite3StrICmp(
                    (unsafe { (*pIdx).zName }) as *const i8,
                    zIndexedBy as *const i8,
                )
            }) != (0 as i32);
        } else {
            __v2287 = false as bool;
        }
        if !__v2287 {
            break;
        }
        {}
        pIdx = unsafe { (*pIdx).pNext };
    }
    if !(pIdx != std::ptr::null_mut::<Index>()) {
        unsafe {
            sqlite3ErrorMsg(
                pParse,
                (b"no such index: %s\0".as_ptr() as *mut i8) as *const i8,
                zIndexedBy,
                0 as i32,
            )
        };
        unsafe {
            (*pParse)
                .__slate_bits_0
                .__set_checkSchema((1 as i32) as u32);
        }
        return 1 as i32;
    }
    0 as i32;
    unsafe {
        (*pFrom).u2.pIBIndex = pIdx;
    }
    return 0 as i32;
}

// /*
// ** Generate byte-code for the SELECT statement given in the p argument.
// **
// ** The results are returned according to the SelectDest structure.
// ** See comments in sqliteInt.h for further information.
// **
// ** This routine returns the number of errors.  If any errors are
// ** encountered, then an appropriate error message is left in
// ** pParse->zErrMsg.
// **
// ** This routine does NOT free the Select structure passed in.  The
// ** calling function needs to do that.
// **
// ** This is a long function.  The following is an outline of the processing
// ** steps, with tags referencing various milestones:
// **
// **  *  Resolve names and similar preparation                tag-select-0100
// **  *  Scan of the FROM clause                              tag-select-0200
// **      +  OUTER JOIN strength reduction                      tag-select-0220
// **      +  Sub-query ORDER BY removal                         tag-select-0230
// **      +  Query flattening                                   tag-select-0240
// **  *  Separate subroutine for compound-SELECT              tag-select-0300
// **  *  WHERE-clause constant propagation                    tag-select-0330
// **  *  Count()-of-VIEW optimization                         tag-select-0350
// **  *  Scan of the FROM clause again                        tag-select-0400
// **      +  Authorize unreferenced tables                      tag-select-0410
// **      +  Predicate push-down optimization                   tag-select-0420
// **      +  Omit unused subquery columns optimization          tag-select-0440
// **      +  Generate code to implement subqueries              tag-select-0480
// **         -  Co-routines                                       tag-select-0482
// **         -  Reuse previously computed CTE                     tag-select-0484
// **         -  REuse previously computed VIEW                    tag-select-0486
// **         -  Materialize a VIEW or CTE                         tag-select-0488
// **  *  DISTINCT ORDER BY -> GROUP BY optimization           tag-select-0500
// **  *  Set up for ORDER BY                                  tag-select-0600
// **  *  Create output table                                  tag-select-0630
// **  *  Prepare registers for LIMIT                          tag-select-0650
// **  *  Setup for DISTINCT                                   tag-select-0680
// **  *  Generate code for non-aggregate and non-GROUP BY     tag-select-0700
// **  *  Generate code for aggregate and/or GROUP BY          tag-select-0800
// **      +  GROUP BY queries                                   tag-select-0810
// **      +  non-GROUP BY queries                               tag-select-0820
// **         -  Special case of count() w/o GROUP BY              tag-select-0821
// **         -  General case of non-GROUP BY aggregates           tag-select-0822
// **  *  Sort results, as needed                              tag-select-0900
// **  *  Internal self-checks                                 tag-select-1000
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3Select(
    mut pParse: *mut Parse,
    mut p: *mut Select,
    mut pDest: *mut SelectDest,
) -> i32 {
    let mut __slate_storage_2367: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2367: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_2367) as *mut i32;
    let mut __slate_storage_2366: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2366: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_2366) as *mut i32;
    let mut __slate_storage_2365: std::mem::MaybeUninit<*mut Parse> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_2365: *mut *mut Parse =
        std::ptr::addr_of_mut!(__slate_storage_2365) as *mut *mut Parse;
    let mut __slate_storage_1509: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1509: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1509) as *mut i32;
    let mut __slate_storage_1508: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1508: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1508) as *mut i32;
    let mut __slate_storage_1507: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1507: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1507) as *mut i32;
    let mut __slate_storage_1506: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1506: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1506) as *mut i32;
    let mut __slate_storage_2364: std::mem::MaybeUninit<bool> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2364: *mut bool = std::ptr::addr_of_mut!(__slate_storage_2364) as *mut bool;
    let mut __slate_storage_2363: std::mem::MaybeUninit<i16> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2363: *mut i16 = std::ptr::addr_of_mut!(__slate_storage_2363) as *mut i16;
    let mut __slate_storage_2362: std::mem::MaybeUninit<i16> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2362: *mut i16 = std::ptr::addr_of_mut!(__slate_storage_2362) as *mut i16;
    let mut __slate_storage_2361: std::mem::MaybeUninit<*mut Select> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_2361: *mut *mut Select =
        std::ptr::addr_of_mut!(__slate_storage_2361) as *mut *mut Select;
    let mut __slate_storage_1505: std::mem::MaybeUninit<*mut Window> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1505: *mut *mut Window =
        std::ptr::addr_of_mut!(__slate_storage_1505) as *mut *mut Window;
    let mut __slate_storage_1504: std::mem::MaybeUninit<u16> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1504: *mut u16 = std::ptr::addr_of_mut!(__slate_storage_1504) as *mut u16;
    let mut __slate_storage_1543: std::mem::MaybeUninit<*mut AggInfo_func> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1543: *mut *mut AggInfo_func =
        std::ptr::addr_of_mut!(__slate_storage_1543) as *mut *mut AggInfo_func;
    let mut __slate_storage_2417: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2417: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_2417) as *mut i32;
    let mut __slate_storage_2416: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2416: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_2416) as *mut i32;
    let mut __slate_storage_1542: std::mem::MaybeUninit<*mut Expr> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1542: *mut *mut Expr =
        std::ptr::addr_of_mut!(__slate_storage_1542) as *mut *mut Expr;
    let mut __slate_storage_1541: std::mem::MaybeUninit<*mut Expr> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1541: *mut *mut Expr =
        std::ptr::addr_of_mut!(__slate_storage_1541) as *mut *mut Expr;
    let mut __slate_storage_1540: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1540: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1540) as *mut i32;
    let mut __slate_storage_2415: std::mem::MaybeUninit<bool> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2415: *mut bool = std::ptr::addr_of_mut!(__slate_storage_2415) as *mut bool;
    let mut __slate_storage_2414: std::mem::MaybeUninit<bool> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2414: *mut bool = std::ptr::addr_of_mut!(__slate_storage_2414) as *mut bool;
    let mut __slate_storage_2413: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2413: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_2413) as *mut i32;
    let mut __slate_storage_2412: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2412: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_2412) as *mut i32;
    let mut __slate_storage_2411: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2411: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_2411) as *mut i32;
    let mut __slate_storage_2410: std::mem::MaybeUninit<*mut Parse> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_2410: *mut *mut Parse =
        std::ptr::addr_of_mut!(__slate_storage_2410) as *mut *mut Parse;
    let mut __slate_storage_2407: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2407: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_2407) as *mut i32;
    let mut __slate_storage_2406: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2406: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_2406) as *mut i32;
    let mut __slate_storage_2409: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2409: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_2409) as *mut i32;
    let mut __slate_storage_2408: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2408: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_2408) as *mut i32;
    let mut __slate_storage_1539: std::mem::MaybeUninit<*mut AggInfo_col> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1539: *mut *mut AggInfo_col =
        std::ptr::addr_of_mut!(__slate_storage_1539) as *mut *mut AggInfo_col;
    let mut __slate_storage_2401: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2401: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_2401) as *mut i32;
    let mut __slate_storage_2400: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2400: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_2400) as *mut i32;
    let mut __slate_storage_2405: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2405: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_2405) as *mut i32;
    let mut __slate_storage_2404: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2404: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_2404) as *mut i32;
    let mut __slate_storage_2403: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2403: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_2403) as *mut i32;
    let mut __slate_storage_2402: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2402: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_2402) as *mut i32;
    let mut __slate_storage_1538: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1538: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1538) as *mut i32;
    let mut __slate_storage_1537: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1537: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1537) as *mut i32;
    let mut __slate_storage_1536: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1536: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1536) as *mut i32;
    let mut __slate_storage_1535: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1535: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1535) as *mut i32;
    let mut __slate_storage_2399: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2399: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_2399) as *mut i32;
    let mut __slate_storage_2398: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2398: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_2398) as *mut i32;
    let mut __slate_storage_2397: std::mem::MaybeUninit<*mut Parse> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_2397: *mut *mut Parse =
        std::ptr::addr_of_mut!(__slate_storage_2397) as *mut *mut Parse;
    let mut __slate_storage_2396: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2396: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_2396) as *mut i32;
    let mut __slate_storage_2395: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2395: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_2395) as *mut i32;
    let mut __slate_storage_2394: std::mem::MaybeUninit<*mut Parse> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_2394: *mut *mut Parse =
        std::ptr::addr_of_mut!(__slate_storage_2394) as *mut *mut Parse;
    let mut __slate_storage_2393: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2393: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_2393) as *mut i32;
    let mut __slate_storage_2392: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2392: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_2392) as *mut i32;
    let mut __slate_storage_2391: std::mem::MaybeUninit<*mut Parse> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_2391: *mut *mut Parse =
        std::ptr::addr_of_mut!(__slate_storage_2391) as *mut *mut Parse;
    let mut __slate_storage_2390: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2390: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_2390) as *mut i32;
    let mut __slate_storage_2389: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2389: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_2389) as *mut i32;
    let mut __slate_storage_2388: std::mem::MaybeUninit<*mut Parse> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_2388: *mut *mut Parse =
        std::ptr::addr_of_mut!(__slate_storage_2388) as *mut *mut Parse;
    let mut __slate_storage_2387: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2387: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_2387) as *mut i32;
    let mut __slate_storage_2386: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2386: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_2386) as *mut i32;
    let mut __slate_storage_2385: std::mem::MaybeUninit<*mut Parse> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_2385: *mut *mut Parse =
        std::ptr::addr_of_mut!(__slate_storage_2385) as *mut *mut Parse;
    let mut __slate_storage_2384: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2384: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_2384) as *mut i32;
    let mut __slate_storage_2383: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2383: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_2383) as *mut i32;
    let mut __slate_storage_2382: std::mem::MaybeUninit<*mut Parse> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_2382: *mut *mut Parse =
        std::ptr::addr_of_mut!(__slate_storage_2382) as *mut *mut Parse;
    let mut __slate_storage_2381: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2381: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_2381) as *mut i32;
    let mut __slate_storage_2380: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2380: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_2380) as *mut i32;
    let mut __slate_storage_2379: std::mem::MaybeUninit<*mut Parse> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_2379: *mut *mut Parse =
        std::ptr::addr_of_mut!(__slate_storage_2379) as *mut *mut Parse;
    let mut __slate_storage_1534: std::mem::MaybeUninit<*mut Expr> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1534: *mut *mut Expr =
        std::ptr::addr_of_mut!(__slate_storage_1534) as *mut *mut Expr;
    let mut __slate_storage_1533: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1533: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1533) as *mut i32;
    let mut __slate_storage_1532: std::mem::MaybeUninit<u16> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1532: *mut u16 = std::ptr::addr_of_mut!(__slate_storage_1532) as *mut u16;
    let mut __slate_storage_1531: std::mem::MaybeUninit<*mut ExprList> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1531: *mut *mut ExprList =
        std::ptr::addr_of_mut!(__slate_storage_1531) as *mut *mut ExprList;
    let mut __slate_storage_1530: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1530: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1530) as *mut i32;
    let mut __slate_storage_1529: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1529: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1529) as *mut i32;
    let mut __slate_storage_1528: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1528: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1528) as *mut i32;
    let mut __slate_storage_1527: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1527: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1527) as *mut i32;
    let mut __slate_storage_1526: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1526: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1526) as *mut i32;
    let mut __slate_storage_1525: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1525: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1525) as *mut i32;
    let mut __slate_storage_1524: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1524: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1524) as *mut i32;
    let mut __slate_storage_1523: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1523: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1523) as *mut i32;
    let mut __slate_storage_1522: std::mem::MaybeUninit<*mut KeyInfo> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1522: *mut *mut KeyInfo =
        std::ptr::addr_of_mut!(__slate_storage_1522) as *mut *mut KeyInfo;
    let mut __slate_storage_1550: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1550: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_1550) as *mut u32;
    let mut __slate_storage_1549: std::mem::MaybeUninit<*mut Index> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1549: *mut *mut Index =
        std::ptr::addr_of_mut!(__slate_storage_1549) as *mut *mut Index;
    let mut __slate_storage_1548: std::mem::MaybeUninit<*mut KeyInfo> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1548: *mut *mut KeyInfo =
        std::ptr::addr_of_mut!(__slate_storage_1548) as *mut *mut KeyInfo;
    let mut __slate_storage_1547: std::mem::MaybeUninit<*mut Index> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1547: *mut *mut Index =
        std::ptr::addr_of_mut!(__slate_storage_1547) as *mut *mut Index;
    let mut __slate_storage_2421: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2421: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_2421) as *mut i32;
    let mut __slate_storage_2420: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2420: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_2420) as *mut i32;
    let mut __slate_storage_2419: std::mem::MaybeUninit<*mut Parse> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_2419: *mut *mut Parse =
        std::ptr::addr_of_mut!(__slate_storage_2419) as *mut *mut Parse;
    let mut __slate_storage_1546: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1546: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1546) as *mut i32;
    let mut __slate_storage_1545: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1545: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1545) as *mut i32;
    let mut __slate_storage_1555: std::mem::MaybeUninit<*mut AggInfo_func> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1555: *mut *mut AggInfo_func =
        std::ptr::addr_of_mut!(__slate_storage_1555) as *mut *mut AggInfo_func;
    let mut __slate_storage_2426: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2426: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_2426) as *mut i32;
    let mut __slate_storage_2425: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2425: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_2425) as *mut i32;
    let mut __slate_storage_2424: std::mem::MaybeUninit<*mut Parse> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_2424: *mut *mut Parse =
        std::ptr::addr_of_mut!(__slate_storage_2424) as *mut *mut Parse;
    let mut __slate_storage_2423: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2423: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_2423) as *mut i32;
    let mut __slate_storage_2422: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2422: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_2422) as *mut i32;
    let mut __slate_storage_1554: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1554: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1554) as *mut i32;
    let mut __slate_storage_1553: std::mem::MaybeUninit<u16> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1553: *mut u16 = std::ptr::addr_of_mut!(__slate_storage_1553) as *mut u16;
    let mut __slate_storage_1552: std::mem::MaybeUninit<*mut ExprList> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1552: *mut *mut ExprList =
        std::ptr::addr_of_mut!(__slate_storage_1552) as *mut *mut ExprList;
    let mut __slate_storage_1551: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1551: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1551) as *mut i32;
    let mut __slate_storage_2418: std::mem::MaybeUninit<*mut Table> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_2418: *mut *mut Table =
        std::ptr::addr_of_mut!(__slate_storage_2418) as *mut *mut Table;
    let mut __slate_storage_1544: std::mem::MaybeUninit<*mut Table> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1544: *mut *mut Table =
        std::ptr::addr_of_mut!(__slate_storage_1544) as *mut *mut Table;
    let mut __slate_storage_2378: std::mem::MaybeUninit<bool> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2378: *mut bool = std::ptr::addr_of_mut!(__slate_storage_2378) as *mut bool;
    let mut __slate_storage_2377: std::mem::MaybeUninit<*mut ExprList_item> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_2377: *mut *mut ExprList_item =
        std::ptr::addr_of_mut!(__slate_storage_2377) as *mut *mut ExprList_item;
    let mut __slate_storage_2376: std::mem::MaybeUninit<*mut ExprList_item> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_2376: *mut *mut ExprList_item =
        std::ptr::addr_of_mut!(__slate_storage_2376) as *mut *mut ExprList_item;
    let mut __slate_storage_2375: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2375: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_2375) as *mut i32;
    let mut __slate_storage_2374: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2374: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_2374) as *mut i32;
    let mut __slate_storage_2373: std::mem::MaybeUninit<*mut ExprList_item> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_2373: *mut *mut ExprList_item =
        std::ptr::addr_of_mut!(__slate_storage_2373) as *mut *mut ExprList_item;
    let mut __slate_storage_2372: std::mem::MaybeUninit<*mut ExprList_item> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_2372: *mut *mut ExprList_item =
        std::ptr::addr_of_mut!(__slate_storage_2372) as *mut *mut ExprList_item;
    let mut __slate_storage_2371: std::mem::MaybeUninit<*mut ExprList_item> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_2371: *mut *mut ExprList_item =
        std::ptr::addr_of_mut!(__slate_storage_2371) as *mut *mut ExprList_item;
    let mut __slate_storage_2370: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2370: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_2370) as *mut i32;
    let mut __slate_storage_2369: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2369: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_2369) as *mut i32;
    let mut __slate_storage_2368: std::mem::MaybeUninit<*mut ExprList_item> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_2368: *mut *mut ExprList_item =
        std::ptr::addr_of_mut!(__slate_storage_2368) as *mut *mut ExprList_item;
    let mut __slate_storage_1521: std::mem::MaybeUninit<*mut ExprList_item> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1521: *mut *mut ExprList_item =
        std::ptr::addr_of_mut!(__slate_storage_1521) as *mut *mut ExprList_item;
    let mut __slate_storage_1520: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1520: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1520) as *mut i32;
    let mut __slate_storage_1519: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1519: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1519) as *mut i32;
    let mut __slate_storage_1518: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1518: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1518) as *mut i32;
    let mut __slate_storage_1517: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1517: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1517) as *mut i32;
    let mut __slate_storage_1516: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1516: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1516) as *mut i32;
    let mut __slate_storage_1515: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1515: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1515) as *mut i32;
    let mut __slate_storage_1514: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1514: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1514) as *mut i32;
    let mut __slate_storage_1513: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1513: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1513) as *mut i32;
    let mut __slate_storage_1512: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1512: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1512) as *mut i32;
    let mut __slate_storage_1511: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1511: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1511) as *mut i32;
    let mut __slate_storage_1510: std::mem::MaybeUninit<NameContext> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1510: *mut NameContext =
        std::ptr::addr_of_mut!(__slate_storage_1510) as *mut NameContext;
    let mut __slate_storage_2360: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2360: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_2360) as *mut i32;
    let mut __slate_storage_2359: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2359: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_2359) as *mut i32;
    let mut __slate_storage_2358: std::mem::MaybeUninit<*mut Parse> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_2358: *mut *mut Parse =
        std::ptr::addr_of_mut!(__slate_storage_2358) as *mut *mut Parse;
    let mut __slate_storage_2357: std::mem::MaybeUninit<u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2357: *mut u8 = std::ptr::addr_of_mut!(__slate_storage_2357) as *mut u8;
    let mut __slate_storage_2356: std::mem::MaybeUninit<u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2356: *mut u8 = std::ptr::addr_of_mut!(__slate_storage_2356) as *mut u8;
    let mut __slate_storage_2355: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2355: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_2355) as *mut i32;
    let mut __slate_storage_2354: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2354: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_2354) as *mut i32;
    let mut __slate_storage_2350: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2350: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_2350) as *mut i32;
    let mut __slate_storage_2349: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2349: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_2349) as *mut i32;
    let mut __slate_storage_2353: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2353: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_2353) as *mut i32;
    let mut __slate_storage_2352: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2352: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_2352) as *mut i32;
    let mut __slate_storage_2351: std::mem::MaybeUninit<*mut ExprList> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_2351: *mut *mut ExprList =
        std::ptr::addr_of_mut!(__slate_storage_2351) as *mut *mut ExprList;
    let mut __slate_storage_1503: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1503: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1503) as *mut i32;
    let mut __slate_storage_2348: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2348: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_2348) as *mut i32;
    let mut __slate_storage_2347: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2347: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_2347) as *mut i32;
    let mut __slate_storage_2346: std::mem::MaybeUninit<*mut Parse> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_2346: *mut *mut Parse =
        std::ptr::addr_of_mut!(__slate_storage_2346) as *mut *mut Parse;
    let mut __slate_storage_1502: std::mem::MaybeUninit<*mut KeyInfo> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1502: *mut *mut KeyInfo =
        std::ptr::addr_of_mut!(__slate_storage_1502) as *mut *mut KeyInfo;
    let mut __slate_storage_2345: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2345: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_2345) as *mut u32;
    let mut __slate_storage_2344: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2344: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_2344) as *mut u32;
    let mut __slate_storage_2343: std::mem::MaybeUninit<*mut Select> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_2343: *mut *mut Select =
        std::ptr::addr_of_mut!(__slate_storage_2343) as *mut *mut Select;
    let mut __slate_storage_2342: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2342: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_2342) as *mut i32;
    let mut __slate_storage_2341: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2341: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_2341) as *mut i32;
    let mut __slate_storage_2340: std::mem::MaybeUninit<*mut ExprList> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_2340: *mut *mut ExprList =
        std::ptr::addr_of_mut!(__slate_storage_2340) as *mut *mut ExprList;
    let mut __slate_storage_2339: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2339: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_2339) as *mut u32;
    let mut __slate_storage_2338: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2338: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_2338) as *mut u32;
    let mut __slate_storage_2337: std::mem::MaybeUninit<*mut Select> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_2337: *mut *mut Select =
        std::ptr::addr_of_mut!(__slate_storage_2337) as *mut *mut Select;
    let mut __slate_storage_2336: std::mem::MaybeUninit<bool> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2336: *mut bool = std::ptr::addr_of_mut!(__slate_storage_2336) as *mut bool;
    let mut __slate_storage_2335: std::mem::MaybeUninit<bool> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2335: *mut bool = std::ptr::addr_of_mut!(__slate_storage_2335) as *mut bool;
    let mut __slate_storage_2319: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2319: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_2319) as *mut i32;
    let mut __slate_storage_2318: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2318: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_2318) as *mut i32;
    let mut __slate_storage_2334: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2334: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_2334) as *mut i32;
    let mut __slate_storage_2333: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2333: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_2333) as *mut i32;
    let mut __slate_storage_2332: std::mem::MaybeUninit<*mut Parse> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_2332: *mut *mut Parse =
        std::ptr::addr_of_mut!(__slate_storage_2332) as *mut *mut Parse;
    let mut __slate_storage_2327: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2327: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_2327) as *mut i32;
    let mut __slate_storage_2326: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2326: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_2326) as *mut i32;
    let mut __slate_storage_2325: std::mem::MaybeUninit<*mut Parse> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_2325: *mut *mut Parse =
        std::ptr::addr_of_mut!(__slate_storage_2325) as *mut *mut Parse;
    let mut __slate_storage_1496: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1496: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1496) as *mut i32;
    let mut __slate_storage_1497: std::mem::MaybeUninit<*mut CteUse> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1497: *mut *mut CteUse =
        std::ptr::addr_of_mut!(__slate_storage_1497) as *mut *mut CteUse;
    let mut __slate_storage_1498: std::mem::MaybeUninit<*mut Subquery> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1498: *mut *mut Subquery =
        std::ptr::addr_of_mut!(__slate_storage_1498) as *mut *mut Subquery;
    let mut __slate_storage_1501: std::mem::MaybeUninit<*mut CteUse> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1501: *mut *mut CteUse =
        std::ptr::addr_of_mut!(__slate_storage_1501) as *mut *mut CteUse;
    let mut __slate_storage_2331: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2331: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_2331) as *mut i32;
    let mut __slate_storage_2330: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2330: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_2330) as *mut i32;
    let mut __slate_storage_2329: std::mem::MaybeUninit<*mut Parse> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_2329: *mut *mut Parse =
        std::ptr::addr_of_mut!(__slate_storage_2329) as *mut *mut Parse;
    let mut __slate_storage_1500: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1500: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1500) as *mut i32;
    let mut __slate_storage_1499: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1499: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1499) as *mut i32;
    let mut __slate_storage_2328: std::mem::MaybeUninit<*mut SrcItem> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_2328: *mut *mut SrcItem =
        std::ptr::addr_of_mut!(__slate_storage_2328) as *mut *mut SrcItem;
    let mut __slate_storage_2324: std::mem::MaybeUninit<bool> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2324: *mut bool = std::ptr::addr_of_mut!(__slate_storage_2324) as *mut bool;
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
    let mut __slate_storage_1495: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1495: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1495) as *mut i32;
    let mut __slate_storage_1494: std::mem::MaybeUninit<*const i8> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1494: *mut *const i8 =
        std::ptr::addr_of_mut!(__slate_storage_1494) as *mut *const i8;
    let mut __slate_storage_1493: std::mem::MaybeUninit<*const i8> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1493: *mut *const i8 =
        std::ptr::addr_of_mut!(__slate_storage_1493) as *mut *const i8;
    let mut __slate_storage_1492: std::mem::MaybeUninit<*mut Select> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1492: *mut *mut Select =
        std::ptr::addr_of_mut!(__slate_storage_1492) as *mut *mut Select;
    let mut __slate_storage_1491: std::mem::MaybeUninit<*mut Subquery> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1491: *mut *mut Subquery =
        std::ptr::addr_of_mut!(__slate_storage_1491) as *mut *mut Subquery;
    let mut __slate_storage_1490: std::mem::MaybeUninit<SelectDest> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1490: *mut SelectDest =
        std::ptr::addr_of_mut!(__slate_storage_1490) as *mut SelectDest;
    let mut __slate_storage_1489: std::mem::MaybeUninit<*mut SrcItem> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1489: *mut *mut SrcItem =
        std::ptr::addr_of_mut!(__slate_storage_1489) as *mut *mut SrcItem;
    let mut __slate_storage_1488: std::mem::MaybeUninit<*mut SrcItem> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1488: *mut *mut SrcItem =
        std::ptr::addr_of_mut!(__slate_storage_1488) as *mut *mut SrcItem;
    let mut __slate_storage_2317: std::mem::MaybeUninit<bool> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2317: *mut bool = std::ptr::addr_of_mut!(__slate_storage_2317) as *mut bool;
    let mut __slate_storage_2316: std::mem::MaybeUninit<bool> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2316: *mut bool = std::ptr::addr_of_mut!(__slate_storage_2316) as *mut bool;
    let mut __slate_storage_2295: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2295: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_2295) as *mut i32;
    let mut __slate_storage_2294: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2294: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_2294) as *mut i32;
    let mut __slate_storage_2312: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2312: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_2312) as *mut i32;
    let mut __slate_storage_2311: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2311: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_2311) as *mut i32;
    let mut __slate_storage_2315: std::mem::MaybeUninit<u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2315: *mut u8 = std::ptr::addr_of_mut!(__slate_storage_2315) as *mut u8;
    let mut __slate_storage_2314: std::mem::MaybeUninit<u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2314: *mut u8 = std::ptr::addr_of_mut!(__slate_storage_2314) as *mut u8;
    let mut __slate_storage_2313: std::mem::MaybeUninit<*mut SrcItem> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_2313: *mut *mut SrcItem =
        std::ptr::addr_of_mut!(__slate_storage_2313) as *mut *mut SrcItem;
    let mut __slate_storage_2304: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2304: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_2304) as *mut i32;
    let mut __slate_storage_2303: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2303: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_2303) as *mut i32;
    let mut __slate_storage_2307: std::mem::MaybeUninit<u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2307: *mut u8 = std::ptr::addr_of_mut!(__slate_storage_2307) as *mut u8;
    let mut __slate_storage_2306: std::mem::MaybeUninit<u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2306: *mut u8 = std::ptr::addr_of_mut!(__slate_storage_2306) as *mut u8;
    let mut __slate_storage_2305: std::mem::MaybeUninit<*mut SrcItem> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_2305: *mut *mut SrcItem =
        std::ptr::addr_of_mut!(__slate_storage_2305) as *mut *mut SrcItem;
    let mut __slate_storage_2310: std::mem::MaybeUninit<u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2310: *mut u8 = std::ptr::addr_of_mut!(__slate_storage_2310) as *mut u8;
    let mut __slate_storage_2309: std::mem::MaybeUninit<u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2309: *mut u8 = std::ptr::addr_of_mut!(__slate_storage_2309) as *mut u8;
    let mut __slate_storage_2308: std::mem::MaybeUninit<*mut SrcItem> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_2308: *mut *mut SrcItem =
        std::ptr::addr_of_mut!(__slate_storage_2308) as *mut *mut SrcItem;
    let mut __slate_storage_1487: std::mem::MaybeUninit<*mut SrcItem> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1487: *mut *mut SrcItem =
        std::ptr::addr_of_mut!(__slate_storage_1487) as *mut *mut SrcItem;
    let mut __slate_storage_2299: std::mem::MaybeUninit<u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2299: *mut u8 = std::ptr::addr_of_mut!(__slate_storage_2299) as *mut u8;
    let mut __slate_storage_2298: std::mem::MaybeUninit<u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2298: *mut u8 = std::ptr::addr_of_mut!(__slate_storage_2298) as *mut u8;
    let mut __slate_storage_2297: std::mem::MaybeUninit<*mut SrcItem> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_2297: *mut *mut SrcItem =
        std::ptr::addr_of_mut!(__slate_storage_2297) as *mut *mut SrcItem;
    let mut __slate_storage_2302: std::mem::MaybeUninit<u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2302: *mut u8 = std::ptr::addr_of_mut!(__slate_storage_2302) as *mut u8;
    let mut __slate_storage_2301: std::mem::MaybeUninit<u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2301: *mut u8 = std::ptr::addr_of_mut!(__slate_storage_2301) as *mut u8;
    let mut __slate_storage_2300: std::mem::MaybeUninit<*mut SrcItem> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_2300: *mut *mut SrcItem =
        std::ptr::addr_of_mut!(__slate_storage_2300) as *mut *mut SrcItem;
    let mut __slate_storage_2296: std::mem::MaybeUninit<bool> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2296: *mut bool = std::ptr::addr_of_mut!(__slate_storage_2296) as *mut bool;
    let mut __slate_storage_1486: std::mem::MaybeUninit<*mut Table> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1486: *mut *mut Table =
        std::ptr::addr_of_mut!(__slate_storage_1486) as *mut *mut Table;
    let mut __slate_storage_1485: std::mem::MaybeUninit<*mut Select> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1485: *mut *mut Select =
        std::ptr::addr_of_mut!(__slate_storage_1485) as *mut *mut Select;
    let mut __slate_storage_1484: std::mem::MaybeUninit<*mut SrcItem> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1484: *mut *mut SrcItem =
        std::ptr::addr_of_mut!(__slate_storage_1484) as *mut *mut SrcItem;
    let mut __slate_storage_2293: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2293: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_2293) as *mut u32;
    let mut __slate_storage_2292: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2292: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_2292) as *mut u32;
    let mut __slate_storage_2291: std::mem::MaybeUninit<*mut Select> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_2291: *mut *mut Select =
        std::ptr::addr_of_mut!(__slate_storage_2291) as *mut *mut Select;
    let mut __slate_storage_1483: std::mem::MaybeUninit<*mut SrcItem> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1483: *mut *mut SrcItem =
        std::ptr::addr_of_mut!(__slate_storage_1483) as *mut *mut SrcItem;
    let mut __slate_storage_2290: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2290: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_2290) as *mut u32;
    let mut __slate_storage_2289: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2289: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_2289) as *mut u32;
    let mut __slate_storage_2288: std::mem::MaybeUninit<*mut Select> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_2288: *mut *mut Select =
        std::ptr::addr_of_mut!(__slate_storage_2288) as *mut *mut Select;
    let mut __slate_storage_1482: std::mem::MaybeUninit<u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1482: *mut u8 = std::ptr::addr_of_mut!(__slate_storage_1482) as *mut u8;
    let mut __slate_storage_1481: std::mem::MaybeUninit<*mut ExprList> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1481: *mut *mut ExprList =
        std::ptr::addr_of_mut!(__slate_storage_1481) as *mut *mut ExprList;
    let mut __slate_storage_1480: std::mem::MaybeUninit<*mut sqlite3> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1480: *mut *mut sqlite3 =
        std::ptr::addr_of_mut!(__slate_storage_1480) as *mut *mut sqlite3;
    let mut __slate_storage_1479: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1479: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1479) as *mut i32;
    let mut __slate_storage_1478: std::mem::MaybeUninit<SortCtx> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1478: *mut SortCtx =
        std::ptr::addr_of_mut!(__slate_storage_1478) as *mut SortCtx;
    let mut __slate_storage_1477: std::mem::MaybeUninit<DistinctCtx> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1477: *mut DistinctCtx =
        std::ptr::addr_of_mut!(__slate_storage_1477) as *mut DistinctCtx;
    let mut __slate_storage_1476: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1476: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1476) as *mut i32;
    let mut __slate_storage_1475: std::mem::MaybeUninit<*mut AggInfo> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1475: *mut *mut AggInfo =
        std::ptr::addr_of_mut!(__slate_storage_1475) as *mut *mut AggInfo;
    let mut __slate_storage_1474: std::mem::MaybeUninit<*mut Expr> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1474: *mut *mut Expr =
        std::ptr::addr_of_mut!(__slate_storage_1474) as *mut *mut Expr;
    let mut __slate_storage_1473: std::mem::MaybeUninit<*mut ExprList> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1473: *mut *mut ExprList =
        std::ptr::addr_of_mut!(__slate_storage_1473) as *mut *mut ExprList;
    let mut __slate_storage_1472: std::mem::MaybeUninit<*mut Expr> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1472: *mut *mut Expr =
        std::ptr::addr_of_mut!(__slate_storage_1472) as *mut *mut Expr;
    let mut __slate_storage_1471: std::mem::MaybeUninit<*mut SrcList> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1471: *mut *mut SrcList =
        std::ptr::addr_of_mut!(__slate_storage_1471) as *mut *mut SrcList;
    let mut __slate_storage_1470: std::mem::MaybeUninit<*mut ExprList> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1470: *mut *mut ExprList =
        std::ptr::addr_of_mut!(__slate_storage_1470) as *mut *mut ExprList;
    let mut __slate_storage_1469: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1469: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1469) as *mut i32;
    let mut __slate_storage_1468: std::mem::MaybeUninit<*mut Vdbe> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1468: *mut *mut Vdbe =
        std::ptr::addr_of_mut!(__slate_storage_1468) as *mut *mut Vdbe;
    let mut __slate_storage_1467: std::mem::MaybeUninit<*mut WhereInfo> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1467: *mut *mut WhereInfo =
        std::ptr::addr_of_mut!(__slate_storage_1467) as *mut *mut WhereInfo;
    let mut __slate_storage_1466: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1466: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1466) as *mut i32;
    let mut __slate_storage_1465: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1465: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1465) as *mut i32;
    unsafe {
        // /* Loop counters */
        // /* Return from sqlite3WhereBegin() */
        // /* The virtual machine under construction */
        // /* True for select lists like "count(*)" */
        // /* List of columns to extract. */
        std::ptr::write(__slate_slot_1470, std::ptr::null_mut::<ExprList>());
        // /* List of tables to select from */
        // /* The WHERE clause.  May be NULL */
        // /* The GROUP BY clause.  May be NULL */
        // /* The HAVING clause.  May be NULL */
        // /* Aggregate information */
        std::ptr::write(__slate_slot_1475, std::ptr::null_mut::<AggInfo>());
        // /* Value to return from this function */
        std::ptr::write(__slate_slot_1476, 1 as i32);
        // /* Info on how to code the DISTINCT keyword */
        // /* Info on how to code the ORDER BY clause */
        // /* Address of the end of the query */
        // /* The database connection */
        // /* Added ORDER BY for min/max queries */
        std::ptr::write(__slate_slot_1481, std::ptr::null_mut::<ExprList>());
        // /* Flag for min/max queries */
        *__slate_slot_1480 = unsafe { (*pParse).db };
        0 as i32;
        *__slate_slot_1468 = sqlite3GetVdbe(pParse);
        if p == std::ptr::null_mut::<Select>() || (unsafe { (*pParse).nErr }) != (0 as i32) {
            return 1 as i32;
        } else {
            0 as i32;
            if (unsafe {
                sqlite3AuthCheck(
                    pParse,
                    21 as i32,
                    std::ptr::null::<i8>(),
                    std::ptr::null::<i8>(),
                    std::ptr::null::<i8>(),
                )
            }) != (0 as i32)
            {
                return 1 as i32;
            } else {
                // /* tag-select-0100 */
                0 as i32;
                0 as i32;
                0 as i32;
                0 as i32;
                if (((unsafe { (*pDest).eDest }) as u32) as i32) <= (4 as i32) {
                    0 as i32;
                    // /* All of these destinations are also able to ignore the ORDER BY clause */
                    if (unsafe { (*p).pOrderBy }) != std::ptr::null_mut::<ExprList>() {
                        unsafe {
                            sqlite3ParserAddCleanup(
                                pParse,
                                unsafe {
                                    std::mem::transmute::<
                                        *const (),
                                        Option<unsafe extern "C-unwind" fn(*mut sqlite3, *mut ())>,
                                    >(
                                        sqlite3ExprListDeleteGeneric as *const ()
                                    )
                                },
                                (unsafe { (*p).pOrderBy }) as *mut (),
                            )
                        };
                        {}
                        unsafe {
                            (*p).pOrderBy = std::ptr::null_mut::<ExprList>();
                        }
                    }
                    std::ptr::write(__slate_slot_2288, p);
                    std::ptr::write(__slate_slot_2289, unsafe {
                        (*(*__slate_slot_2288)).selFlags
                    });
                    std::ptr::write(__slate_slot_2290, *__slate_slot_2289 & !((1 as i32) as u32));
                    unsafe {
                        (*(*__slate_slot_2288)).selFlags = *__slate_slot_2290;
                    }
                }
                '__join_0: {
                    sqlite3SelectPrep(pParse, p, std::ptr::null_mut::<NameContext>());
                    if (unsafe { (*pParse).nErr }) != (0 as i32) {
                    } else {
                        0 as i32;
                        0 as i32;
                        // /* If the SF_UFSrcCheck flag is set, then this function is being called
                        //   ** as part of populating the temp table for an UPDATE...FROM statement.
                        //   ** In this case, it is an error if the target object (pSrc->a[0]) name
                        //   ** or alias is duplicated within FROM clause (pSrc->a[1..n]).
                        //   **
                        //   ** Postgres disallows this case too. The reason is that some other
                        //   ** systems handle this case differently, and not all the same way,
                        //   ** which is just confusing. To avoid this, we follow PG's lead and
                        //   ** disallow it altogether.  */
                        if (unsafe { (*p).selFlags }) & ((8388608 as i32) as u32) != (0 as u32) {
                            std::ptr::write(__slate_slot_1483, unsafe {
                                unsafe {
                                    std::ptr::addr_of_mut!((*unsafe { (*p).pSrc }).a)
                                        as *mut SrcItem
                                }
                                .offset((0 as i32) as isize)
                            });
                            if sameSrcAlias(*__slate_slot_1483, unsafe { (*p).pSrc }) != (0 as i32)
                            {
                                unsafe {
                                    sqlite3ErrorMsg(
                                        pParse,
                                        (b"target object/alias may not appear in FROM clause: %s\0"
                                            .as_ptr()
                                            as *mut i8)
                                            as *const i8,
                                        if (unsafe { (*(*__slate_slot_1483)).zAlias })
                                            != std::ptr::null_mut::<i8>()
                                        {
                                            unsafe { (*(*__slate_slot_1483)).zAlias }
                                        } else {
                                            unsafe {
                                                (*unsafe { (*(*__slate_slot_1483)).pSTab }).zName
                                            }
                                        },
                                    )
                                };
                                break '__join_0;
                            } else {
                                // /* Clear the SF_UFSrcCheck flag. The check has already been performed,
                                //     ** and leaving this flag set can cause errors if a compound sub-query
                                //     ** in p->pSrc is flattened into this query and this function called
                                //     ** again as part of compound SELECT processing.  */
                                std::ptr::write(__slate_slot_2291, p);
                                std::ptr::write(__slate_slot_2292, unsafe {
                                    (*(*__slate_slot_2291)).selFlags
                                });
                                std::ptr::write(
                                    __slate_slot_2293,
                                    *__slate_slot_2292 & !((8388608 as i32) as u32),
                                );
                                unsafe {
                                    (*(*__slate_slot_2291)).selFlags = *__slate_slot_2293;
                                }
                            }
                        }
                        if (((unsafe { (*pDest).eDest }) as u32) as i32) == (7 as i32) {
                            sqlite3GenerateColumnNames(pParse, p);
                        }
                        if (unsafe { sqlite3WindowRewrite(pParse, p) }) != (0 as i32) {
                            0 as i32;
                        } else {
                            // /* SQLITE_OMIT_WINDOWFUNC */
                            *__slate_slot_1471 = unsafe { (*p).pSrc };
                            *__slate_slot_1469 = ((unsafe { (*p).selFlags }) & ((8 as i32) as u32)
                                != ((0 as i32) as u32))
                                as i32;
                            unsafe {
                                memset(
                                    std::ptr::addr_of_mut!(*__slate_slot_1478) as *mut (),
                                    0 as i32,
                                    48 as u64,
                                )
                            };
                            (*__slate_slot_1478).pOrderBy = unsafe { (*p).pOrderBy };
                            // /* Try to do various optimizations (flattening subqueries, and strength
                            //   ** reduction of join operators) in the FROM clause up into the main query
                            //   ** tag-select-0200
                            //   */
                            *__slate_slot_1465 = 0 as i32;
                            '__join_243: {
                                loop {
                                    if !((unsafe { (*p).pPrior }) != std::ptr::null_mut::<Select>())
                                        && *__slate_slot_1465
                                            < unsafe { (*(*__slate_slot_1471)).nSrc }
                                    {
                                        std::ptr::write(__slate_slot_1484, unsafe {
                                            unsafe {
                                                std::ptr::addr_of_mut!((*(*__slate_slot_1471)).a)
                                                    as *mut SrcItem
                                            }
                                            .offset(*__slate_slot_1465 as isize)
                                        });
                                        std::ptr::write(
                                            __slate_slot_1485,
                                            if ((unsafe {
                                                (*(*__slate_slot_1484))
                                                    .fg
                                                    .__slate_bits_0
                                                    .__get_isSubquery()
                                            })
                                                as i32)
                                                != (0 as i32)
                                            {
                                                unsafe {
                                                    (*unsafe { (*(*__slate_slot_1484)).u4.pSubq })
                                                        .pSelect
                                                }
                                            } else {
                                                std::ptr::null_mut::<Select>()
                                            },
                                        );
                                        std::ptr::write(__slate_slot_1486, unsafe {
                                            (*(*__slate_slot_1484)).pSTab
                                        });
                                        // /* The expander should have already created transient Table objects
                                        //     ** even for FROM clause elements such as subqueries that do not correspond
                                        //     ** to a real table */
                                        0 as i32;
                                        // /* Try to simplify joins:
                                        //     **
                                        //     **      LEFT JOIN  ->  JOIN
                                        //     **     RIGHT JOIN  ->  JOIN
                                        //     **      FULL JOIN  ->  RIGHT JOIN
                                        //     **
                                        //     ** If terms of the i-th table are used in the WHERE clause in such a
                                        //     ** way that the i-th table cannot be the NULL row of a join, then
                                        //     ** perform the appropriate simplification. This is called
                                        //     ** "OUTER JOIN strength reduction" in the SQLite documentation.
                                        //     ** tag-select-0220
                                        //     */
                                        if (((unsafe { (*(*__slate_slot_1484)).fg.jointype })
                                            as u32)
                                            as i32)
                                            & ((8 as i32) | (64 as i32))
                                            != (0 as i32)
                                        {
                                            *__slate_slot_2296 = (unsafe {
                                                sqlite3ExprImpliesNonNullRow(
                                                    unsafe { (*p).pWhere },
                                                    unsafe { (*(*__slate_slot_1484)).iCursor },
                                                    (((unsafe {
                                                        (*(*__slate_slot_1484)).fg.jointype
                                                    })
                                                        as u32)
                                                        as i32)
                                                        & (64 as i32),
                                                )
                                            }) != (0 as i32);
                                        } else {
                                            *__slate_slot_2296 = false as bool;
                                        }
                                        '__join_245: {
                                            if *__slate_slot_2296
                                                && (unsafe { (*(*__slate_slot_1480)).dbOptFlags })
                                                    & ((8192 as i32) as u32)
                                                    == ((0 as i32) as u32)
                                            {
                                                if (((unsafe {
                                                    (*(*__slate_slot_1484)).fg.jointype
                                                })
                                                    as u32)
                                                    as i32)
                                                    & (8 as i32)
                                                    != (0 as i32)
                                                {
                                                    if (((unsafe {
                                                        (*(*__slate_slot_1484)).fg.jointype
                                                    })
                                                        as u32)
                                                        as i32)
                                                        & (16 as i32)
                                                        != (0 as i32)
                                                    {
                                                        {}
                                                        std::ptr::write(
                                                            __slate_slot_2297,
                                                            *__slate_slot_1484,
                                                        );
                                                        std::ptr::write(
                                                            __slate_slot_2298,
                                                            unsafe {
                                                                (*(*__slate_slot_2297)).fg.jointype
                                                            },
                                                        );
                                                        std::ptr::write(
                                                            __slate_slot_2299,
                                                            ((((*__slate_slot_2298 as u32) as i32)
                                                                & !(8 as i32))
                                                                as i8)
                                                                as u8,
                                                        );
                                                        unsafe {
                                                            (*(*__slate_slot_2297)).fg.jointype =
                                                                *__slate_slot_2299;
                                                        }
                                                    } else {
                                                        {}
                                                        std::ptr::write(
                                                            __slate_slot_2300,
                                                            *__slate_slot_1484,
                                                        );
                                                        std::ptr::write(
                                                            __slate_slot_2301,
                                                            unsafe {
                                                                (*(*__slate_slot_2300)).fg.jointype
                                                            },
                                                        );
                                                        std::ptr::write(
                                                            __slate_slot_2302,
                                                            ((((*__slate_slot_2301 as u32) as i32)
                                                                & !((8 as i32) | (32 as i32)))
                                                                as i8)
                                                                as u8,
                                                        );
                                                        unsafe {
                                                            (*(*__slate_slot_2300)).fg.jointype =
                                                                *__slate_slot_2302;
                                                        }
                                                        unsetJoinExpr(
                                                            unsafe { (*p).pWhere },
                                                            unsafe {
                                                                (*(*__slate_slot_1484)).iCursor
                                                            },
                                                            0 as i32,
                                                        );
                                                    }
                                                }
                                                if (((unsafe {
                                                    (*(*__slate_slot_1484)).fg.jointype
                                                })
                                                    as u32)
                                                    as i32)
                                                    & (64 as i32)
                                                    != (0 as i32)
                                                {
                                                    *__slate_slot_1466 =
                                                        *__slate_slot_1465 + (1 as i32);
                                                    loop {
                                                        if *__slate_slot_1466
                                                            < unsafe {
                                                                (*(*__slate_slot_1471)).nSrc
                                                            }
                                                        {
                                                            std::ptr::write(
                                                                __slate_slot_1487,
                                                                unsafe {
                                                                    unsafe {
                                                                        std::ptr::addr_of_mut!(
                                                                            (*(*__slate_slot_1471))
                                                                                .a
                                                                        )
                                                                            as *mut SrcItem
                                                                    }
                                                                    .offset(
                                                                        *__slate_slot_1466 as isize,
                                                                    )
                                                                },
                                                            );
                                                            if (((unsafe {
                                                                (*(*__slate_slot_1487)).fg.jointype
                                                            })
                                                                as u32)
                                                                as i32)
                                                                & (16 as i32)
                                                                != (0 as i32)
                                                            {
                                                                if (((unsafe {
                                                                    (*(*__slate_slot_1487))
                                                                        .fg
                                                                        .jointype
                                                                })
                                                                    as u32)
                                                                    as i32)
                                                                    & (8 as i32)
                                                                    != (0 as i32)
                                                                {
                                                                    {}
                                                                    std::ptr::write(
                                                                        __slate_slot_2305,
                                                                        *__slate_slot_1487,
                                                                    );
                                                                    std::ptr::write(
                                                                        __slate_slot_2306,
                                                                        unsafe {
                                                                            (*(*__slate_slot_2305))
                                                                                .fg
                                                                                .jointype
                                                                        },
                                                                    );
                                                                    std::ptr::write(
                                                                        __slate_slot_2307,
                                                                        ((((*__slate_slot_2306
                                                                            as u32)
                                                                            as i32)
                                                                            & !(16 as i32))
                                                                            as i8)
                                                                            as u8,
                                                                    );
                                                                    unsafe {
                                                                        (*(*__slate_slot_2305))
                                                                            .fg
                                                                            .jointype =
                                                                            *__slate_slot_2307;
                                                                    }
                                                                } else {
                                                                    {}
                                                                    std::ptr::write(
                                                                        __slate_slot_2308,
                                                                        *__slate_slot_1487,
                                                                    );
                                                                    std::ptr::write(
                                                                        __slate_slot_2309,
                                                                        unsafe {
                                                                            (*(*__slate_slot_2308))
                                                                                .fg
                                                                                .jointype
                                                                        },
                                                                    );
                                                                    std::ptr::write(
                                                                        __slate_slot_2310,
                                                                        ((((*__slate_slot_2309
                                                                            as u32)
                                                                            as i32)
                                                                            & !((16 as i32)
                                                                                | (32 as i32)))
                                                                            as i8)
                                                                            as u8,
                                                                    );
                                                                    unsafe {
                                                                        (*(*__slate_slot_2308))
                                                                            .fg
                                                                            .jointype =
                                                                            *__slate_slot_2310;
                                                                    }
                                                                    unsetJoinExpr(
                                                                        unsafe { (*p).pWhere },
                                                                        unsafe {
                                                                            (*(*__slate_slot_1487))
                                                                                .iCursor
                                                                        },
                                                                        1 as i32,
                                                                    );
                                                                }
                                                            }
                                                            std::ptr::write(
                                                                __slate_slot_2303,
                                                                *__slate_slot_1466,
                                                            );
                                                            std::ptr::write(
                                                                __slate_slot_2304,
                                                                *__slate_slot_2303 + (1 as i32),
                                                            );
                                                            *__slate_slot_1466 = *__slate_slot_2304;
                                                        } else {
                                                            break;
                                                        }
                                                    }
                                                    *__slate_slot_1466 =
                                                        (unsafe { (*(*__slate_slot_1471)).nSrc })
                                                            - (1 as i32);
                                                    loop {
                                                        if *__slate_slot_1466 >= (0 as i32) {
                                                            std::ptr::write(
                                                                __slate_slot_2313,
                                                                unsafe {
                                                                    unsafe {
                                                                        std::ptr::addr_of_mut!(
                                                                            (*(*__slate_slot_1471))
                                                                                .a
                                                                        )
                                                                            as *mut SrcItem
                                                                    }
                                                                    .offset(
                                                                        *__slate_slot_1466 as isize,
                                                                    )
                                                                },
                                                            );
                                                            std::ptr::write(
                                                                __slate_slot_2314,
                                                                unsafe {
                                                                    (*(*__slate_slot_2313))
                                                                        .fg
                                                                        .jointype
                                                                },
                                                            );
                                                            std::ptr::write(
                                                                __slate_slot_2315,
                                                                ((((*__slate_slot_2314 as u32)
                                                                    as i32)
                                                                    & !(64 as i32))
                                                                    as i8)
                                                                    as u8,
                                                            );
                                                            unsafe {
                                                                (*(*__slate_slot_2313))
                                                                    .fg
                                                                    .jointype = *__slate_slot_2315;
                                                            }
                                                            if (((unsafe {
                                                                (*unsafe {
                                                                    unsafe {
                                                                        std::ptr::addr_of_mut!(
                                                                            (*(*__slate_slot_1471))
                                                                                .a
                                                                        )
                                                                            as *mut SrcItem
                                                                    }
                                                                    .offset(
                                                                        *__slate_slot_1466 as isize,
                                                                    )
                                                                })
                                                                .fg
                                                                .jointype
                                                            })
                                                                as u32)
                                                                as i32)
                                                                & (16 as i32)
                                                                != (0 as i32)
                                                            {
                                                                break '__join_245;
                                                            } else {
                                                                std::ptr::write(
                                                                    __slate_slot_2311,
                                                                    *__slate_slot_1466,
                                                                );
                                                                std::ptr::write(
                                                                    __slate_slot_2312,
                                                                    *__slate_slot_2311 - (1 as i32),
                                                                );
                                                                *__slate_slot_1466 =
                                                                    *__slate_slot_2312;
                                                            }
                                                        } else {
                                                            break '__join_245;
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        // /* No further action if this term of the FROM clause is not a subquery */
                                        if *__slate_slot_1485 == std::ptr::null_mut::<Select>() {
                                        } else {
                                            // /* Catch mismatch in the declared columns of a view and the number of
                                            //     ** columns in the SELECT on the RHS */
                                            if ((unsafe { (*(*__slate_slot_1486)).nCol }) as i32)
                                                != unsafe {
                                                    (*unsafe { (*(*__slate_slot_1485)).pEList })
                                                        .nExpr
                                                }
                                            {
                                                break '__join_243;
                                            } else {
                                                // /* Do not attempt the usual optimizations (flattening and ORDER BY
                                                //     ** elimination) on a MATERIALIZED common table expression because
                                                //     ** a MATERIALIZED common table expression is an optimization fence.
                                                //     */
                                                if ((unsafe {
                                                    (*(*__slate_slot_1484))
                                                        .fg
                                                        .__slate_bits_0
                                                        .__get_isCte()
                                                })
                                                    as i32)
                                                    != (0 as i32)
                                                    && (((unsafe {
                                                        (*unsafe {
                                                            (*(*__slate_slot_1484)).u2.pCteUse
                                                        })
                                                        .eM10d
                                                    })
                                                        as u32)
                                                        as i32)
                                                        == (0 as i32)
                                                {
                                                } else {
                                                    // /* Do not try to flatten an aggregate subquery.
                                                    //     **
                                                    //     ** Flattening an aggregate subquery is only possible if the outer query
                                                    //     ** is not a join.  But if the outer query is not a join, then the subquery
                                                    //     ** will be implemented as a co-routine and there is no advantage to
                                                    //     ** flattening in that case.
                                                    //     */
                                                    if (unsafe { (*(*__slate_slot_1485)).selFlags })
                                                        & ((8 as i32) as u32)
                                                        != ((0 as i32) as u32)
                                                    {
                                                    } else {
                                                        0 as i32;
                                                        // /* tag-select-0230:
                                                        //     ** If a FROM-clause subquery has an ORDER BY clause that is not
                                                        //     ** really doing anything, then delete it now so that it does not
                                                        //     ** interfere with query flattening.  See the discussion at
                                                        //     ** https://sqlite.org/forum/forumpost/2d76f2bcf65d256a
                                                        //     **
                                                        //     ** Beware of these cases where the ORDER BY clause may not be safely
                                                        //     ** omitted:
                                                        //     **
                                                        //     **    (1)   There is also a LIMIT clause
                                                        //     **    (2)   The subquery was added to help with window-function
                                                        //     **          processing
                                                        //     **    (3)   The subquery is in the FROM clause of an UPDATE
                                                        //     **    (4)   The outer query uses an aggregate function other than
                                                        //     **          the built-in count(), min(), or max().
                                                        //     **    (5)   The ORDER BY isn't going to accomplish anything because
                                                        //     **          one of:
                                                        //     **            (a)  The outer query has a different ORDER BY clause
                                                        //     **            (b)  The subquery is part of a join
                                                        //     **          See forum post 062d576715d277c8
                                                        //     **    (6)   The subquery is not a recursive CTE.  ORDER BY has a different
                                                        //     **          meaning for recursive CTEs and this optimization does not
                                                        //     **          apply.
                                                        //     **
                                                        //     ** Also retain the ORDER BY if the OmitOrderBy optimization is disabled.
                                                        //     */
                                                        if (unsafe {
                                                            (*(*__slate_slot_1485)).pOrderBy
                                                        }) != std::ptr::null_mut::<ExprList>()
                                                            && ((unsafe { (*p).pOrderBy })
                                                                != std::ptr::null_mut::<ExprList>()
                                                                || (unsafe {
                                                                    (*(*__slate_slot_1471)).nSrc
                                                                }) > (1 as i32))
                                                            && (unsafe {
                                                                (*(*__slate_slot_1485)).pLimit
                                                            }) == std::ptr::null_mut::<Expr>()
                                                            && (unsafe {
                                                                (*(*__slate_slot_1485)).selFlags
                                                            }) & (((134217728 as i32)
                                                                | (8192 as i32))
                                                                as u32)
                                                                == ((0 as i32) as u32)
                                                            && (unsafe { (*p).selFlags })
                                                                & ((134217728 as i32) as u32)
                                                                == ((0 as i32) as u32)
                                                            && (unsafe {
                                                                (*(*__slate_slot_1480)).dbOptFlags
                                                            }) & ((262144 as i32) as u32)
                                                                == ((0 as i32) as u32)
                                                        {
                                                            {}
                                                            unsafe {
                                                                sqlite3ParserAddCleanup(
                                                                    pParse,
                                                                    unsafe {
                                                                        std::mem::transmute::<*const (), Option<unsafe extern "C-unwind" fn(*mut sqlite3, *mut ())>>(sqlite3ExprListDeleteGeneric as *const ())
                                                                    },
                                                                    (unsafe {
                                                                        (*(*__slate_slot_1485))
                                                                            .pOrderBy
                                                                    })
                                                                        as *mut (),
                                                                )
                                                            };
                                                            unsafe {
                                                                (*(*__slate_slot_1485)).pOrderBy =
                                                                    std::ptr::null_mut::<ExprList>(
                                                                    );
                                                            }
                                                        }
                                                        // /* Condition (5) */
                                                        // /* Condition (1) */
                                                        // /* (2) and (6) */
                                                        // /* Condition (3) and (4) */
                                                        // /* If the outer query contains a "complex" result set (that is,
                                                        //     ** if the result set of the outer query uses functions or subqueries)
                                                        //     ** and if the subquery contains an ORDER BY clause and if
                                                        //     ** it will be implemented as a co-routine, then do not flatten.  This
                                                        //     ** restriction allows SQL constructs like this:
                                                        //     **
                                                        //     **  SELECT expensive_function(x)
                                                        //     **    FROM (SELECT x FROM tab ORDER BY y LIMIT 10);
                                                        //     **
                                                        //     ** The expensive_function() is only computed on the 10 rows that
                                                        //     ** are output, rather than every row of the table.
                                                        //     **
                                                        //     ** The requirement that the outer query have a complex result set
                                                        //     ** means that flattening does occur on simpler SQL constraints without
                                                        //     ** the expensive_function() like:
                                                        //     **
                                                        //     **  SELECT x FROM (SELECT x FROM tab ORDER BY y LIMIT 10);
                                                        //     */
                                                        if (unsafe {
                                                            (*(*__slate_slot_1485)).pOrderBy
                                                        }) != std::ptr::null_mut::<ExprList>()
                                                            && *__slate_slot_1465 == (0 as i32)
                                                            && (unsafe { (*p).selFlags })
                                                                & ((262144 as i32) as u32)
                                                                != ((0 as i32) as u32)
                                                            && ((unsafe {
                                                                (*(*__slate_slot_1471)).nSrc
                                                            }) == (1 as i32)
                                                                || (((unsafe {
                                                                    (*unsafe { unsafe { std::ptr::addr_of_mut!((*(*__slate_slot_1471)).a) as *mut SrcItem }.offset((1 as i32) as isize) }).fg.jointype
                                                                })
                                                                    as u32)
                                                                    as i32)
                                                                    & ((32 as i32) | (2 as i32))
                                                                    != (0 as i32))
                                                        {
                                                        } else {
                                                            // /* tag-select-0240 */
                                                            if flattenSubquery(
                                                                pParse,
                                                                p,
                                                                *__slate_slot_1465,
                                                                *__slate_slot_1469,
                                                            ) != (0 as i32)
                                                            {
                                                                if (unsafe { (*pParse).nErr })
                                                                    != (0 as i32)
                                                                {
                                                                    break '__join_0;
                                                                } else {
                                                                    // /* This subquery can be absorbed into its parent. */
                                                                    *__slate_slot_1465 =
                                                                        -(1 as i32);
                                                                }
                                                            }
                                                            *__slate_slot_1471 =
                                                                unsafe { (*p).pSrc };
                                                            if (unsafe {
                                                                (*(*__slate_slot_1480)).mallocFailed
                                                            }) != (0 as u8)
                                                            {
                                                                break '__join_0;
                                                            } else {
                                                                if !((((unsafe { (*pDest).eDest })
                                                                    as u32)
                                                                    as i32)
                                                                    <= (6 as i32))
                                                                {
                                                                    (*__slate_slot_1478).pOrderBy =
                                                                        unsafe { (*p).pOrderBy };
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                        std::ptr::write(__slate_slot_2294, *__slate_slot_1465);
                                        std::ptr::write(
                                            __slate_slot_2295,
                                            *__slate_slot_2294 + (1 as i32),
                                        );
                                        *__slate_slot_1465 = *__slate_slot_2295;
                                    } else {
                                        break;
                                    }
                                }
                                // /* Handle compound SELECT statements using the separate multiSelect()
                                //   ** procedure.  tag-select-0300
                                //   */
                                if (unsafe { (*p).pPrior }) != std::ptr::null_mut::<Select>() {
                                    *__slate_slot_1476 = multiSelect(pParse, p, pDest);
                                    if (unsafe { (*p).pNext }) == std::ptr::null_mut::<Select>() {
                                        unsafe { sqlite3VdbeExplainPop(pParse) };
                                    }
                                    return *__slate_slot_1476;
                                } else {
                                    // /* If there may be an "EXISTS (SELECT ...)" in the WHERE clause, attempt
                                    //   ** to change it into a join.  */
                                    if ((unsafe { (*pParse).__slate_bits_0.__get_bHasExists() })
                                        as i32)
                                        != (0 as i32)
                                        && (unsafe { (*(*__slate_slot_1480)).dbOptFlags })
                                            & ((1073741824 as i32) as u32)
                                            == ((0 as i32) as u32)
                                    {
                                        existsToJoin(pParse, p, unsafe { (*p).pWhere });
                                        *__slate_slot_1471 = unsafe { (*p).pSrc };
                                    }
                                    // /* Do the WHERE-clause constant propagation optimization if this is
                                    //   ** a join.  No need to spend time on this operation for non-join queries
                                    //   ** as the equivalent optimization will be handled by query planner in
                                    //   ** sqlite3WhereBegin().  tag-select-0330
                                    //   */
                                    if (unsafe { (*p).pWhere }) != std::ptr::null_mut::<Expr>()
                                        && (((unsafe { (*unsafe { (*p).pWhere }).op }) as u32)
                                            as i32)
                                            == (44 as i32)
                                        && (unsafe { (*(*__slate_slot_1480)).dbOptFlags })
                                            & ((32768 as i32) as u32)
                                            == ((0 as i32) as u32)
                                    {
                                        *__slate_slot_2316 =
                                            propagateConstants(pParse, p) != (0 as i32);
                                    } else {
                                        *__slate_slot_2316 = false as bool;
                                    }
                                    if *__slate_slot_2316 {
                                    } else {
                                        {}
                                    }
                                    // /* tag-select-0350 */
                                    if (unsafe { (*(*__slate_slot_1480)).dbOptFlags })
                                        & (((1 as i32) | (512 as i32)) as u32)
                                        == ((0 as i32) as u32)
                                    {
                                        *__slate_slot_2317 =
                                            countOfViewOptimization(pParse, p) != (0 as i32);
                                    } else {
                                        *__slate_slot_2317 = false as bool;
                                    }
                                    if *__slate_slot_2317 {
                                        if (unsafe { (*(*__slate_slot_1480)).mallocFailed })
                                            != (0 as u8)
                                        {
                                            break '__join_0;
                                        } else {
                                            *__slate_slot_1471 = unsafe { (*p).pSrc };
                                        }
                                    }
                                    // /* Loop over all terms in the FROM clause and do two things for each term:
                                    //   **
                                    //   **   (1) Authorize unreferenced tables
                                    //   **   (2) Generate code for all sub-queries
                                    //   **
                                    //   ** tag-select-0400
                                    //   */
                                    *__slate_slot_1465 = 0 as i32;
                                    loop {
                                        if *__slate_slot_1465
                                            < unsafe { (*(*__slate_slot_1471)).nSrc }
                                        {
                                            std::ptr::write(__slate_slot_1488, unsafe {
                                                unsafe {
                                                    std::ptr::addr_of_mut!(
                                                        (*(*__slate_slot_1471)).a
                                                    )
                                                        as *mut SrcItem
                                                }
                                                .offset(*__slate_slot_1465 as isize)
                                            });
                                            // /* Authorized unreferenced tables.  tag-select-0410
                                            //     **
                                            //     ** Issue SQLITE_READ authorizations with a fake column name for any
                                            //     ** tables that are referenced but from which no values are extracted.
                                            //     ** Examples of where these kinds of null SQLITE_READ authorizations
                                            //     ** would occur:
                                            //     **
                                            //     **     SELECT count(*) FROM t1;   -- SQLITE_READ t1.""
                                            //     **     SELECT t1.* FROM t1, t2;   -- SQLITE_READ t2.""
                                            //     **
                                            //     ** The fake column name is an empty string.  It is possible for a table to
                                            //     ** have a column named by the empty string, in which case there is no way to
                                            //     ** distinguish between an unreferenced table and an actual reference to the
                                            //     ** "" column. The original design was for the fake column name to be a NULL,
                                            //     ** which would be unambiguous.  But legacy authorization callbacks might
                                            //     ** assume the column name is non-NULL and segfault.  The use of an empty
                                            //     ** string for the fake column name seems safer.
                                            //     */
                                            if (unsafe { (*(*__slate_slot_1488)).colUsed })
                                                == (((0 as i32) as i64) as u64)
                                                && (unsafe { (*(*__slate_slot_1488)).zName })
                                                    != std::ptr::null_mut::<i8>()
                                            {
                                                if ((unsafe {
                                                    (*(*__slate_slot_1488))
                                                        .fg
                                                        .__slate_bits_0
                                                        .__get_fixedSchema()
                                                })
                                                    as i32)
                                                    != (0 as i32)
                                                {
                                                    std::ptr::write(__slate_slot_1495, unsafe {
                                                        sqlite3SchemaToIndex(
                                                            unsafe { (*pParse).db },
                                                            unsafe {
                                                                (*(*__slate_slot_1488)).u4.pSchema
                                                            },
                                                        )
                                                    });
                                                    *__slate_slot_1494 = (unsafe {
                                                        (*unsafe {
                                                            unsafe { (*(*__slate_slot_1480)).aDb }
                                                                .offset(*__slate_slot_1495 as isize)
                                                        })
                                                        .zDbSName
                                                    })
                                                        as *const i8;
                                                } else {
                                                    if ((unsafe {
                                                        (*(*__slate_slot_1488))
                                                            .fg
                                                            .__slate_bits_0
                                                            .__get_isSubquery()
                                                    })
                                                        as i32)
                                                        != (0 as i32)
                                                    {
                                                        *__slate_slot_1494 = std::ptr::null::<i8>();
                                                    } else {
                                                        *__slate_slot_1494 = (unsafe {
                                                            (*(*__slate_slot_1488)).u4.zDatabase
                                                        })
                                                            as *const i8;
                                                    }
                                                }
                                                unsafe {
                                                    sqlite3AuthCheck(
                                                        pParse,
                                                        20 as i32,
                                                        (unsafe { (*(*__slate_slot_1488)).zName })
                                                            as *const i8,
                                                        (b"\0".as_ptr() as *mut i8) as *const i8,
                                                        *__slate_slot_1494,
                                                    )
                                                };
                                            }
                                            // /* Generate code for all sub-queries in the FROM clause
                                            //     */
                                            if ((unsafe {
                                                (*(*__slate_slot_1488))
                                                    .fg
                                                    .__slate_bits_0
                                                    .__get_isSubquery()
                                            })
                                                as i32)
                                                == (0 as i32)
                                            {
                                            } else {
                                                *__slate_slot_1491 =
                                                    unsafe { (*(*__slate_slot_1488)).u4.pSubq };
                                                0 as i32;
                                                *__slate_slot_1492 =
                                                    unsafe { (*(*__slate_slot_1491)).pSelect };
                                                // /* The code for a subquery should only be generated once. */
                                                if (unsafe { (*(*__slate_slot_1491)).addrFillSub })
                                                    != (0 as i32)
                                                {
                                                } else {
                                                    // /* Increment Parse.nHeight by the height of the largest expression
                                                    //     ** tree referred to by this, the parent select. The child select
                                                    //     ** may contain expression trees of at most
                                                    //     ** (SQLITE_MAX_EXPR_DEPTH-Parse.nHeight) height. This is a bit
                                                    //     ** more conservative than necessary, but much easier than enforcing
                                                    //     ** an exact limit.
                                                    //     */
                                                    std::ptr::write(__slate_slot_2320, pParse);
                                                    std::ptr::write(__slate_slot_2321, unsafe {
                                                        (*(*__slate_slot_2320)).nHeight
                                                    });
                                                    std::ptr::write(
                                                        __slate_slot_2322,
                                                        *__slate_slot_2321
                                                            + unsafe {
                                                                sqlite3SelectExprHeight(
                                                                    p as *const Select,
                                                                )
                                                            },
                                                    );
                                                    unsafe {
                                                        (*(*__slate_slot_2320)).nHeight =
                                                            *__slate_slot_2322;
                                                    }
                                                    // /* Make copies of constant WHERE-clause terms in the outer query down
                                                    //     ** inside the subquery.  This can help the subquery to run more efficiently.
                                                    //     ** This is the "predicate push-down optimization".  tag-select-0420
                                                    //     */
                                                    if (unsafe {
                                                        (*(*__slate_slot_1480)).dbOptFlags
                                                    }) & ((4096 as i32) as u32)
                                                        == ((0 as i32) as u32)
                                                        && (((unsafe {
                                                            (*(*__slate_slot_1488))
                                                                .fg
                                                                .__slate_bits_0
                                                                .__get_isCte()
                                                        })
                                                            as i32)
                                                            == (0 as i32)
                                                            || (((unsafe {
                                                                (*unsafe {
                                                                    (*(*__slate_slot_1488))
                                                                        .u2
                                                                        .pCteUse
                                                                })
                                                                .eM10d
                                                            })
                                                                as u32)
                                                                as i32)
                                                                != (0 as i32)
                                                                && (unsafe {
                                                                    (*unsafe {
                                                                        (*(*__slate_slot_1488))
                                                                            .u2
                                                                            .pCteUse
                                                                    })
                                                                    .nUse
                                                                }) < (2 as i32))
                                                    {
                                                        *__slate_slot_2323 = pushDownWhereTerms(
                                                            pParse,
                                                            *__slate_slot_1492,
                                                            unsafe { (*p).pWhere },
                                                            *__slate_slot_1471,
                                                            *__slate_slot_1465,
                                                        ) != (0 as i32);
                                                    } else {
                                                        *__slate_slot_2323 = false as bool;
                                                    }
                                                    if *__slate_slot_2323 {
                                                        0 as i32;
                                                    } else {
                                                        {}
                                                    }
                                                    // /* Convert unused result columns of the subquery into simple NULL
                                                    //     ** expressions, to avoid unneeded searching and computation.
                                                    //     ** tag-select-0440
                                                    //     */
                                                    if (unsafe {
                                                        (*(*__slate_slot_1480)).dbOptFlags
                                                    }) & ((67108864 as i32) as u32)
                                                        == ((0 as i32) as u32)
                                                    {
                                                        *__slate_slot_2324 =
                                                            disableUnusedSubqueryResultColumns(
                                                                *__slate_slot_1488,
                                                            ) != (0 as i32);
                                                    } else {
                                                        *__slate_slot_2324 = false as bool;
                                                    }
                                                    if *__slate_slot_2324 {}
                                                    *__slate_slot_1493 =
                                                        unsafe { (*pParse).zAuthContext };
                                                    unsafe {
                                                        (*pParse).zAuthContext = (unsafe {
                                                            (*(*__slate_slot_1488)).zName
                                                        })
                                                            as *const i8;
                                                    }
                                                    // /* Generate byte-code to implement the subquery  tag-select-0480
                                                    //     */
                                                    if fromClauseTermCanBeCoroutine(
                                                        pParse,
                                                        *__slate_slot_1471,
                                                        *__slate_slot_1465,
                                                        (unsafe { (*p).selFlags }) as i32,
                                                    ) != (0 as i32)
                                                    {
                                                        // /* Implement a co-routine that will return a single row of the result
                                                        //       ** set on each invocation.  tag-select-0482
                                                        //       */
                                                        std::ptr::write(
                                                            __slate_slot_1496,
                                                            (unsafe {
                                                                sqlite3VdbeCurrentAddr(
                                                                    *__slate_slot_1468,
                                                                )
                                                            }) + (1 as i32),
                                                        );
                                                        std::ptr::write(__slate_slot_2325, pParse);
                                                        std::ptr::write(
                                                            __slate_slot_2326,
                                                            unsafe { (*(*__slate_slot_2325)).nMem },
                                                        );
                                                        std::ptr::write(
                                                            __slate_slot_2327,
                                                            *__slate_slot_2326 + (1 as i32),
                                                        );
                                                        unsafe {
                                                            (*(*__slate_slot_2325)).nMem =
                                                                *__slate_slot_2327;
                                                        }
                                                        unsafe {
                                                            (*(*__slate_slot_1491)).regReturn =
                                                                *__slate_slot_2327;
                                                        }
                                                        unsafe {
                                                            sqlite3VdbeAddOp3(
                                                                *__slate_slot_1468,
                                                                11 as i32,
                                                                unsafe {
                                                                    (*(*__slate_slot_1491))
                                                                        .regReturn
                                                                },
                                                                0 as i32,
                                                                *__slate_slot_1496,
                                                            )
                                                        };
                                                        {}
                                                        unsafe {
                                                            (*(*__slate_slot_1491)).addrFillSub =
                                                                *__slate_slot_1496;
                                                        }
                                                        sqlite3SelectDestInit(
                                                            std::ptr::addr_of_mut!(
                                                                *__slate_slot_1490
                                                            ),
                                                            11 as i32,
                                                            unsafe {
                                                                (*(*__slate_slot_1491)).regReturn
                                                            },
                                                        );
                                                        unsafe {
                                                            sqlite3VdbeExplain(
                                                                pParse,
                                                                ((1 as i32) as i8) as u8,
                                                                (b"CO-ROUTINE %!S\0".as_ptr()
                                                                    as *mut i8)
                                                                    as *const i8,
                                                                *__slate_slot_1488,
                                                            )
                                                        };
                                                        sqlite3Select(
                                                            pParse,
                                                            *__slate_slot_1492,
                                                            std::ptr::addr_of_mut!(
                                                                *__slate_slot_1490
                                                            ),
                                                        );
                                                        unsafe {
                                                            (*unsafe {
                                                                (*(*__slate_slot_1488)).pSTab
                                                            })
                                                            .nRowLogEst = unsafe {
                                                                (*(*__slate_slot_1492)).nSelectRow
                                                            };
                                                        }
                                                        unsafe {
                                                            (*(*__slate_slot_1488))
                                                                .fg
                                                                .__slate_bits_0
                                                                .__set_viaCoroutine(
                                                                    (1 as i32) as u32,
                                                                );
                                                        }
                                                        unsafe {
                                                            (*(*__slate_slot_1491)).regResult =
                                                                (*__slate_slot_1490).iSdst;
                                                        }
                                                        unsafe {
                                                            sqlite3VdbeEndCoroutine(
                                                                *__slate_slot_1468,
                                                                unsafe {
                                                                    (*(*__slate_slot_1491))
                                                                        .regReturn
                                                                },
                                                            )
                                                        };
                                                        {}
                                                        unsafe {
                                                            sqlite3VdbeJumpHere(
                                                                *__slate_slot_1468,
                                                                *__slate_slot_1496 - (1 as i32),
                                                            )
                                                        };
                                                        unsafe { sqlite3ClearTempRegCache(pParse) };
                                                    } else {
                                                        if ((unsafe {
                                                            (*(*__slate_slot_1488))
                                                                .fg
                                                                .__slate_bits_0
                                                                .__get_isCte()
                                                        })
                                                            as i32)
                                                            != (0 as i32)
                                                            && (unsafe {
                                                                (*unsafe {
                                                                    (*(*__slate_slot_1488))
                                                                        .u2
                                                                        .pCteUse
                                                                })
                                                                .addrM9e
                                                            }) > (0 as i32)
                                                        {
                                                            // /* This is a CTE for which materialization code has already been
                                                            //       ** generated.  Invoke the subroutine to compute the materialization,
                                                            //       ** then make the pItem->iCursor be a copy of the ephemeral table that
                                                            //       ** holds the result of the materialization. tag-select-0484 */
                                                            std::ptr::write(
                                                                __slate_slot_1497,
                                                                unsafe {
                                                                    (*(*__slate_slot_1488))
                                                                        .u2
                                                                        .pCteUse
                                                                },
                                                            );
                                                            unsafe {
                                                                sqlite3VdbeAddOp2(
                                                                    *__slate_slot_1468,
                                                                    10 as i32,
                                                                    unsafe {
                                                                        (*(*__slate_slot_1497))
                                                                            .regRtn
                                                                    },
                                                                    unsafe {
                                                                        (*(*__slate_slot_1497))
                                                                            .addrM9e
                                                                    },
                                                                )
                                                            };
                                                            if (unsafe {
                                                                (*(*__slate_slot_1488)).iCursor
                                                            }) != unsafe {
                                                                (*(*__slate_slot_1497)).iCur
                                                            } {
                                                                unsafe {
                                                                    sqlite3VdbeAddOp2(
                                                                        *__slate_slot_1468,
                                                                        117 as i32,
                                                                        unsafe {
                                                                            (*(*__slate_slot_1488))
                                                                                .iCursor
                                                                        },
                                                                        unsafe {
                                                                            (*(*__slate_slot_1497))
                                                                                .iCur
                                                                        },
                                                                    )
                                                                };
                                                                {}
                                                            }
                                                            unsafe {
                                                                (*(*__slate_slot_1492))
                                                                    .nSelectRow = unsafe {
                                                                    (*(*__slate_slot_1497)).nRowEst
                                                                };
                                                            }
                                                        } else {
                                                            std::ptr::write(
                                                                __slate_slot_2328,
                                                                isSelfJoinView(
                                                                    *__slate_slot_1471,
                                                                    *__slate_slot_1488,
                                                                    0 as i32,
                                                                    *__slate_slot_1465,
                                                                ),
                                                            );
                                                            *__slate_slot_1489 = *__slate_slot_2328;
                                                            if *__slate_slot_2328
                                                                != std::ptr::null_mut::<SrcItem>()
                                                            {
                                                                // /* This view has already been materialized by a prior entry in
                                                                //       ** this same FROM clause.  Reuse it.  tag-select-0486 */
                                                                0 as i32;
                                                                *__slate_slot_1498 = unsafe {
                                                                    (*(*__slate_slot_1489)).u4.pSubq
                                                                };
                                                                0 as i32;
                                                                if (unsafe {
                                                                    (*(*__slate_slot_1498))
                                                                        .addrFillSub
                                                                }) != (0 as i32)
                                                                {
                                                                    unsafe {
                                                                        sqlite3VdbeAddOp2(
                                                                            *__slate_slot_1468,
                                                                            10 as i32,
                                                                            unsafe {
                                                                                (*(*__slate_slot_1498)).regReturn
                                                                            },
                                                                            unsafe {
                                                                                (*(*__slate_slot_1498)).addrFillSub
                                                                            },
                                                                        )
                                                                    };
                                                                }
                                                                unsafe {
                                                                    sqlite3VdbeAddOp2(
                                                                        *__slate_slot_1468,
                                                                        117 as i32,
                                                                        unsafe {
                                                                            (*(*__slate_slot_1488))
                                                                                .iCursor
                                                                        },
                                                                        unsafe {
                                                                            (*(*__slate_slot_1489))
                                                                                .iCursor
                                                                        },
                                                                    )
                                                                };
                                                                unsafe {
                                                                    (*(*__slate_slot_1492))
                                                                        .nSelectRow = unsafe {
                                                                        (*unsafe {
                                                                            (*(*__slate_slot_1498))
                                                                                .pSelect
                                                                        })
                                                                        .nSelectRow
                                                                    };
                                                                }
                                                            } else {
                                                                // /* Materialize the view.  If the view is not correlated, generate a
                                                                //       ** subroutine to do the materialization so that subsequent uses of
                                                                //       ** the same view can reuse the materialization.  tag-select-0488 */
                                                                std::ptr::write(
                                                                    __slate_slot_1500,
                                                                    0 as i32,
                                                                );
                                                                std::ptr::write(
                                                                    __slate_slot_2329,
                                                                    pParse,
                                                                );
                                                                std::ptr::write(
                                                                    __slate_slot_2330,
                                                                    unsafe {
                                                                        (*(*__slate_slot_2329)).nMem
                                                                    },
                                                                );
                                                                std::ptr::write(
                                                                    __slate_slot_2331,
                                                                    *__slate_slot_2330 + (1 as i32),
                                                                );
                                                                unsafe {
                                                                    (*(*__slate_slot_2329)).nMem =
                                                                        *__slate_slot_2331;
                                                                }
                                                                unsafe {
                                                                    (*(*__slate_slot_1491))
                                                                        .regReturn =
                                                                        *__slate_slot_2331;
                                                                }
                                                                *__slate_slot_1499 = unsafe {
                                                                    sqlite3VdbeAddOp0(
                                                                        *__slate_slot_1468,
                                                                        9 as i32,
                                                                    )
                                                                };
                                                                unsafe {
                                                                    (*(*__slate_slot_1491))
                                                                        .addrFillSub =
                                                                        *__slate_slot_1499
                                                                            + (1 as i32);
                                                                }
                                                                unsafe {
                                                                    (*(*__slate_slot_1488))
                                                                        .fg
                                                                        .__slate_bits_0
                                                                        .__set_isMaterialized(
                                                                            (1 as i32) as u32,
                                                                        );
                                                                }
                                                                if ((unsafe {
                                                                    (*(*__slate_slot_1488))
                                                                        .fg
                                                                        .__slate_bits_0
                                                                        .__get_isCorrelated()
                                                                })
                                                                    as i32)
                                                                    == (0 as i32)
                                                                {
                                                                    // /* If the subquery is not correlated and if we are not inside of
                                                                    //         ** a trigger, then we only need to compute the value of the subquery
                                                                    //         ** once. */
                                                                    *__slate_slot_1500 = unsafe {
                                                                        sqlite3VdbeAddOp0(
                                                                            *__slate_slot_1468,
                                                                            15 as i32,
                                                                        )
                                                                    };
                                                                    {}
                                                                    {}
                                                                } else {
                                                                    {}
                                                                }
                                                                sqlite3SelectDestInit(
                                                                    std::ptr::addr_of_mut!(
                                                                        *__slate_slot_1490
                                                                    ),
                                                                    10 as i32,
                                                                    unsafe {
                                                                        (*(*__slate_slot_1488))
                                                                            .iCursor
                                                                    },
                                                                );
                                                                unsafe {
                                                                    sqlite3VdbeExplain(
                                                                        pParse,
                                                                        ((1 as i32) as i8) as u8,
                                                                        (b"MATERIALIZE %!S\0"
                                                                            .as_ptr()
                                                                            as *mut i8)
                                                                            as *const i8,
                                                                        *__slate_slot_1488,
                                                                    )
                                                                };
                                                                sqlite3Select(
                                                                    pParse,
                                                                    *__slate_slot_1492,
                                                                    std::ptr::addr_of_mut!(
                                                                        *__slate_slot_1490
                                                                    ),
                                                                );
                                                                unsafe {
                                                                    (*unsafe {
                                                                        (*(*__slate_slot_1488))
                                                                            .pSTab
                                                                    })
                                                                    .nRowLogEst = unsafe {
                                                                        (*(*__slate_slot_1492))
                                                                            .nSelectRow
                                                                    };
                                                                }
                                                                if *__slate_slot_1500 != (0 as i32)
                                                                {
                                                                    unsafe {
                                                                        sqlite3VdbeJumpHere(
                                                                            *__slate_slot_1468,
                                                                            *__slate_slot_1500,
                                                                        )
                                                                    };
                                                                }
                                                                unsafe {
                                                                    sqlite3VdbeAddOp2(
                                                                        *__slate_slot_1468,
                                                                        69 as i32,
                                                                        unsafe {
                                                                            (*(*__slate_slot_1491))
                                                                                .regReturn
                                                                        },
                                                                        *__slate_slot_1499
                                                                            + (1 as i32),
                                                                    )
                                                                };
                                                                {}
                                                                {}
                                                                unsafe {
                                                                    sqlite3VdbeJumpHere(
                                                                        *__slate_slot_1468,
                                                                        *__slate_slot_1499,
                                                                    )
                                                                };
                                                                unsafe {
                                                                    sqlite3ClearTempRegCache(pParse)
                                                                };
                                                                if ((unsafe {
                                                                    (*(*__slate_slot_1488))
                                                                        .fg
                                                                        .__slate_bits_0
                                                                        .__get_isCte()
                                                                })
                                                                    as i32)
                                                                    != (0 as i32)
                                                                    && ((unsafe {
                                                                        (*(*__slate_slot_1488))
                                                                            .fg
                                                                            .__slate_bits_0
                                                                            .__get_isCorrelated()
                                                                    })
                                                                        as i32)
                                                                        == (0 as i32)
                                                                {
                                                                    std::ptr::write(
                                                                        __slate_slot_1501,
                                                                        unsafe {
                                                                            (*(*__slate_slot_1488))
                                                                                .u2
                                                                                .pCteUse
                                                                        },
                                                                    );
                                                                    unsafe {
                                                                        (*(*__slate_slot_1501))
                                                                            .addrM9e = unsafe {
                                                                            (*(*__slate_slot_1491))
                                                                                .addrFillSub
                                                                        };
                                                                    }
                                                                    unsafe {
                                                                        (*(*__slate_slot_1501))
                                                                            .regRtn = unsafe {
                                                                            (*(*__slate_slot_1491))
                                                                                .regReturn
                                                                        };
                                                                    }
                                                                    unsafe {
                                                                        (*(*__slate_slot_1501))
                                                                            .iCur = unsafe {
                                                                            (*(*__slate_slot_1488))
                                                                                .iCursor
                                                                        };
                                                                    }
                                                                    unsafe {
                                                                        (*(*__slate_slot_1501))
                                                                            .nRowEst = unsafe {
                                                                            (*(*__slate_slot_1492))
                                                                                .nSelectRow
                                                                        };
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                    if (unsafe {
                                                        (*(*__slate_slot_1480)).mallocFailed
                                                    }) != (0 as u8)
                                                    {
                                                        break '__join_0;
                                                    } else {
                                                        std::ptr::write(__slate_slot_2332, pParse);
                                                        std::ptr::write(
                                                            __slate_slot_2333,
                                                            unsafe {
                                                                (*(*__slate_slot_2332)).nHeight
                                                            },
                                                        );
                                                        std::ptr::write(
                                                            __slate_slot_2334,
                                                            *__slate_slot_2333
                                                                - unsafe {
                                                                    sqlite3SelectExprHeight(
                                                                        p as *const Select,
                                                                    )
                                                                },
                                                        );
                                                        unsafe {
                                                            (*(*__slate_slot_2332)).nHeight =
                                                                *__slate_slot_2334;
                                                        }
                                                        unsafe {
                                                            (*pParse).zAuthContext =
                                                                *__slate_slot_1493;
                                                        }
                                                    }
                                                }
                                            }
                                            std::ptr::write(__slate_slot_2318, *__slate_slot_1465);
                                            std::ptr::write(
                                                __slate_slot_2319,
                                                *__slate_slot_2318 + (1 as i32),
                                            );
                                            *__slate_slot_1465 = *__slate_slot_2319;
                                        } else {
                                            break;
                                        }
                                    }
                                    // /* Various elements of the SELECT copied into local variables for
                                    //   ** convenience */
                                    *__slate_slot_1470 = unsafe { (*p).pEList };
                                    *__slate_slot_1472 = unsafe { (*p).pWhere };
                                    *__slate_slot_1473 = unsafe { (*p).pGroupBy };
                                    *__slate_slot_1474 = unsafe { (*p).pHaving };
                                    (*__slate_slot_1477).isTnct = ((unsafe { (*p).selFlags })
                                        & ((1 as i32) as u32)
                                        != ((0 as i32) as u32))
                                        as u8;
                                    // /* tag-select-0500
                                    //   **
                                    //   ** If the query is DISTINCT with an ORDER BY but is not an aggregate, and
                                    //   ** if the select-list is the same as the ORDER BY list, then this query
                                    //   ** can be rewritten as a GROUP BY. In other words, this:
                                    //   **
                                    //   **     SELECT DISTINCT xyz FROM ... ORDER BY xyz
                                    //   **
                                    //   ** is transformed to:
                                    //   **
                                    //   **     SELECT xyz FROM ... GROUP BY xyz ORDER BY xyz
                                    //   **
                                    //   ** The second form is preferred as a single index (or temp-table) may be
                                    //   ** used for both the ORDER BY and DISTINCT processing. As originally
                                    //   ** written the query must use a temp-table for at least one of the ORDER
                                    //   ** BY and DISTINCT, and an index or separate temp-table for the other.
                                    //   */
                                    if (unsafe { (*p).selFlags })
                                        & (((1 as i32) | (8 as i32)) as u32)
                                        == ((1 as i32) as u32)
                                    {
                                        *__slate_slot_2335 = sqlite3CopySortOrder(
                                            *__slate_slot_1470,
                                            (*__slate_slot_1478).pOrderBy,
                                        ) != (0 as i32);
                                    } else {
                                        *__slate_slot_2335 = false as bool;
                                    }
                                    if *__slate_slot_2335 {
                                        *__slate_slot_2336 = (unsafe {
                                            sqlite3ExprListCompare(
                                                *__slate_slot_1470 as *const ExprList,
                                                (*__slate_slot_1478).pOrderBy as *const ExprList,
                                                -(1 as i32),
                                            )
                                        }) == (0 as i32);
                                    } else {
                                        *__slate_slot_2336 = false as bool;
                                    }
                                    if *__slate_slot_2336
                                        && (unsafe { (*(*__slate_slot_1480)).dbOptFlags })
                                            & ((4 as i32) as u32)
                                            == ((0 as i32) as u32)
                                        && (unsafe { (*p).pWin }) == std::ptr::null_mut::<Window>()
                                    {
                                        '__join_160: {
                                            std::ptr::write(__slate_slot_2337, p);
                                            std::ptr::write(__slate_slot_2338, unsafe {
                                                (*(*__slate_slot_2337)).selFlags
                                            });
                                            std::ptr::write(
                                                __slate_slot_2339,
                                                *__slate_slot_2338 & !((1 as i32) as u32),
                                            );
                                            unsafe {
                                                (*(*__slate_slot_2337)).selFlags =
                                                    *__slate_slot_2339;
                                            }
                                            std::ptr::write(__slate_slot_2340, unsafe {
                                                sqlite3ExprListDup(
                                                    *__slate_slot_1480,
                                                    *__slate_slot_1470 as *const ExprList,
                                                    0 as i32,
                                                )
                                            });
                                            unsafe {
                                                (*p).pGroupBy = *__slate_slot_2340;
                                            }
                                            *__slate_slot_1473 = *__slate_slot_2340;
                                            if *__slate_slot_1473
                                                != std::ptr::null_mut::<ExprList>()
                                            {
                                                *__slate_slot_1465 = 0 as i32;
                                                loop {
                                                    if *__slate_slot_1465
                                                        < unsafe { (*(*__slate_slot_1473)).nExpr }
                                                    {
                                                        unsafe {
                                                            (*unsafe {
                                                                unsafe {
                                                                    std::ptr::addr_of_mut!(
                                                                        (*(*__slate_slot_1473)).a
                                                                    )
                                                                        as *mut ExprList_item
                                                                }
                                                                .offset(*__slate_slot_1465 as isize)
                                                            })
                                                            .u
                                                            .x
                                                            .iOrderByCol = ((*__slate_slot_1465
                                                                + (1 as i32))
                                                                as i16)
                                                                as u16;
                                                        }
                                                        std::ptr::write(
                                                            __slate_slot_2341,
                                                            *__slate_slot_1465,
                                                        );
                                                        std::ptr::write(
                                                            __slate_slot_2342,
                                                            *__slate_slot_2341 + (1 as i32),
                                                        );
                                                        *__slate_slot_1465 = *__slate_slot_2342;
                                                    } else {
                                                        break '__join_160;
                                                    }
                                                }
                                            }
                                        }
                                        std::ptr::write(__slate_slot_2343, p);
                                        std::ptr::write(__slate_slot_2344, unsafe {
                                            (*(*__slate_slot_2343)).selFlags
                                        });
                                        std::ptr::write(
                                            __slate_slot_2345,
                                            *__slate_slot_2344 | ((8 as i32) as u32),
                                        );
                                        unsafe {
                                            (*(*__slate_slot_2343)).selFlags = *__slate_slot_2345;
                                        }
                                        // /* Notice that even thought SF_Distinct has been cleared from p->selFlags,
                                        //     ** the sDistinct.isTnct is still set.  Hence, isTnct represents the
                                        //     ** original setting of the SF_Distinct flag, not the current setting */
                                        0 as i32;
                                        (*__slate_slot_1477).isTnct = ((2 as i32) as i8) as u8;
                                    }
                                    // /* If there is an ORDER BY clause, then create an ephemeral index to
                                    //   ** do the sorting.  But this sorting ephemeral index might end up
                                    //   ** being unused if the data can be extracted in pre-sorted order.
                                    //   ** If that is the case, then the OP_OpenEphemeral instruction will be
                                    //   ** changed to an OP_Noop once we figure out that the sorting index is
                                    //   ** not needed.  The sSort.addrSortIndex variable is used to facilitate
                                    //   ** that change.  tag-select-0600
                                    //   */
                                    if (*__slate_slot_1478).pOrderBy
                                        != std::ptr::null_mut::<ExprList>()
                                    {
                                        *__slate_slot_1502 = sqlite3KeyInfoFromExprList(
                                            pParse,
                                            (*__slate_slot_1478).pOrderBy,
                                            0 as i32,
                                            unsafe { (*(*__slate_slot_1470)).nExpr },
                                        );
                                        std::ptr::write(__slate_slot_2346, pParse);
                                        std::ptr::write(__slate_slot_2347, unsafe {
                                            (*(*__slate_slot_2346)).nTab
                                        });
                                        std::ptr::write(
                                            __slate_slot_2348,
                                            *__slate_slot_2347 + (1 as i32),
                                        );
                                        unsafe {
                                            (*(*__slate_slot_2346)).nTab = *__slate_slot_2348;
                                        }
                                        (*__slate_slot_1478).iECursor = *__slate_slot_2347;
                                        (*__slate_slot_1478).addrSortIndex = unsafe {
                                            sqlite3VdbeAddOp4(
                                                *__slate_slot_1468,
                                                120 as i32,
                                                (*__slate_slot_1478).iECursor,
                                                (unsafe { (*(*__slate_slot_1478).pOrderBy).nExpr })
                                                    + (1 as i32)
                                                    + unsafe { (*(*__slate_slot_1470)).nExpr },
                                                0 as i32,
                                                (*__slate_slot_1502 as *mut i8) as *const i8,
                                                -(9 as i32),
                                            )
                                        };
                                    } else {
                                        (*__slate_slot_1478).addrSortIndex = -(1 as i32);
                                    }
                                    '__join_146: {
                                        // /* If the output is destined for a temporary table, open that table.
                                        //   ** tag-select-0630
                                        //   */
                                        if (((unsafe { (*pDest).eDest }) as u32) as i32)
                                            == (10 as i32)
                                        {
                                            unsafe {
                                                sqlite3VdbeAddOp2(
                                                    *__slate_slot_1468,
                                                    120 as i32,
                                                    unsafe { (*pDest).iSDParm },
                                                    unsafe { (*(*__slate_slot_1470)).nExpr },
                                                )
                                            };
                                            if (unsafe { (*p).selFlags }) & ((2048 as i32) as u32)
                                                != (0 as u32)
                                            {
                                                // /* Delete or NULL-out result columns that will never be used */
                                                *__slate_slot_1503 =
                                                    (unsafe { (*(*__slate_slot_1470)).nExpr })
                                                        - (1 as i32);
                                                loop {
                                                    if *__slate_slot_1503 > (0 as i32)
                                                        && ((unsafe {
                                                            (*unsafe {
                                                                unsafe {
                                                                    std::ptr::addr_of_mut!(
                                                                        (*(*__slate_slot_1470)).a
                                                                    )
                                                                        as *mut ExprList_item
                                                                }
                                                                .offset(*__slate_slot_1503 as isize)
                                                            })
                                                            .fg
                                                            .__slate_bits_0
                                                            .__get_bUsed()
                                                        })
                                                            as i32)
                                                            == (0 as i32)
                                                    {
                                                        unsafe {
                                                            sqlite3ExprDelete(
                                                                *__slate_slot_1480,
                                                                unsafe {
                                                                    (*unsafe { unsafe { std::ptr::addr_of_mut!((*(*__slate_slot_1470)).a) as *mut ExprList_item }.offset(*__slate_slot_1503 as isize) }).pExpr
                                                                },
                                                            )
                                                        };
                                                        unsafe {
                                                            sqlite3DbFree(
                                                                *__slate_slot_1480,
                                                                (unsafe {
                                                                    (*unsafe { unsafe { std::ptr::addr_of_mut!((*(*__slate_slot_1470)).a) as *mut ExprList_item }.offset(*__slate_slot_1503 as isize) }).zEName
                                                                })
                                                                    as *mut (),
                                                            )
                                                        };
                                                        std::ptr::write(
                                                            __slate_slot_2351,
                                                            *__slate_slot_1470,
                                                        );
                                                        std::ptr::write(
                                                            __slate_slot_2352,
                                                            unsafe {
                                                                (*(*__slate_slot_2351)).nExpr
                                                            },
                                                        );
                                                        std::ptr::write(
                                                            __slate_slot_2353,
                                                            *__slate_slot_2352 - (1 as i32),
                                                        );
                                                        unsafe {
                                                            (*(*__slate_slot_2351)).nExpr =
                                                                *__slate_slot_2353;
                                                        }
                                                        std::ptr::write(
                                                            __slate_slot_2349,
                                                            *__slate_slot_1503,
                                                        );
                                                        std::ptr::write(
                                                            __slate_slot_2350,
                                                            *__slate_slot_2349 - (1 as i32),
                                                        );
                                                        *__slate_slot_1503 = *__slate_slot_2350;
                                                    } else {
                                                        break;
                                                    }
                                                }
                                                *__slate_slot_1503 = 0 as i32;
                                                loop {
                                                    if *__slate_slot_1503
                                                        < unsafe { (*(*__slate_slot_1470)).nExpr }
                                                    {
                                                        if ((unsafe {
                                                            (*unsafe {
                                                                unsafe {
                                                                    std::ptr::addr_of_mut!(
                                                                        (*(*__slate_slot_1470)).a
                                                                    )
                                                                        as *mut ExprList_item
                                                                }
                                                                .offset(*__slate_slot_1503 as isize)
                                                            })
                                                            .fg
                                                            .__slate_bits_0
                                                            .__get_bUsed()
                                                        })
                                                            as i32)
                                                            == (0 as i32)
                                                        {
                                                            unsafe {
                                                                (*unsafe { (*unsafe { unsafe { std::ptr::addr_of_mut!((*(*__slate_slot_1470)).a) as *mut ExprList_item }.offset(*__slate_slot_1503 as isize) }).pExpr }).op = ((122 as i32) as i8) as u8;
                                                            }
                                                        }
                                                        std::ptr::write(
                                                            __slate_slot_2354,
                                                            *__slate_slot_1503,
                                                        );
                                                        std::ptr::write(
                                                            __slate_slot_2355,
                                                            *__slate_slot_2354 + (1 as i32),
                                                        );
                                                        *__slate_slot_1503 = *__slate_slot_2355;
                                                    } else {
                                                        break '__join_146;
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    // /* Set the limiter.  tag-select-0650
                                    //   */
                                    *__slate_slot_1479 = unsafe { sqlite3VdbeMakeLabel(pParse) };
                                    if (unsafe { (*p).selFlags }) & ((16384 as i32) as u32)
                                        == ((0 as i32) as u32)
                                    {
                                        // /* 4 billion rows */
                                        unsafe {
                                            (*p).nSelectRow = (320 as i32) as i16;
                                        }
                                    }
                                    if (unsafe { (*p).pLimit }) != std::ptr::null_mut::<Expr>() {
                                        computeLimitRegisters(pParse, p, *__slate_slot_1479);
                                    }
                                    if (unsafe { (*p).iLimit }) == (0 as i32)
                                        && (*__slate_slot_1478).addrSortIndex >= (0 as i32)
                                    {
                                        unsafe {
                                            sqlite3VdbeChangeOpcode(
                                                *__slate_slot_1468,
                                                (*__slate_slot_1478).addrSortIndex,
                                                ((121 as i32) as i8) as u8,
                                            )
                                        };
                                        std::ptr::write(
                                            __slate_slot_2356,
                                            (*__slate_slot_1478).sortFlags,
                                        );
                                        std::ptr::write(
                                            __slate_slot_2357,
                                            ((((*__slate_slot_2356 as u32) as i32) | (1 as i32))
                                                as i8)
                                                as u8,
                                        );
                                        (*__slate_slot_1478).sortFlags = *__slate_slot_2357;
                                    }
                                    // /* Open an ephemeral index to use for the distinct set. tag-select-0680
                                    //   */
                                    if (unsafe { (*p).selFlags }) & ((1 as i32) as u32)
                                        != (0 as u32)
                                    {
                                        std::ptr::write(__slate_slot_2358, pParse);
                                        std::ptr::write(__slate_slot_2359, unsafe {
                                            (*(*__slate_slot_2358)).nTab
                                        });
                                        std::ptr::write(
                                            __slate_slot_2360,
                                            *__slate_slot_2359 + (1 as i32),
                                        );
                                        unsafe {
                                            (*(*__slate_slot_2358)).nTab = *__slate_slot_2360;
                                        }
                                        (*__slate_slot_1477).tabTnct = *__slate_slot_2359;
                                        (*__slate_slot_1477).addrTnct = unsafe {
                                            sqlite3VdbeAddOp4(
                                                *__slate_slot_1468,
                                                120 as i32,
                                                (*__slate_slot_1477).tabTnct,
                                                0 as i32,
                                                0 as i32,
                                                (sqlite3KeyInfoFromExprList(
                                                    pParse,
                                                    unsafe { (*p).pEList },
                                                    0 as i32,
                                                    0 as i32,
                                                )
                                                    as *mut i8)
                                                    as *const i8,
                                                -(9 as i32),
                                            )
                                        };
                                        unsafe {
                                            sqlite3VdbeChangeP5(
                                                *__slate_slot_1468,
                                                ((8 as i32) as i16) as u16,
                                            )
                                        };
                                        (*__slate_slot_1477).eTnctType = ((3 as i32) as i8) as u8;
                                    } else {
                                        (*__slate_slot_1477).eTnctType = ((0 as i32) as i8) as u8;
                                    }
                                    if !(*__slate_slot_1469 != (0 as i32))
                                        && *__slate_slot_1473 == std::ptr::null_mut::<ExprList>()
                                    {
                                        // /* No aggregate functions and no GROUP BY clause.  tag-select-0700 */
                                        std::ptr::write(
                                            __slate_slot_1504,
                                            (((if (*__slate_slot_1477).isTnct != (0 as u8) {
                                                256 as i32
                                            } else {
                                                0 as i32
                                            })
                                                as u32)
                                                | (unsafe { (*p).selFlags })
                                                    & ((16384 as i32) as u32))
                                                as u16,
                                        );
                                        // /* Main window object (or NULL) */
                                        std::ptr::write(__slate_slot_1505, unsafe { (*p).pWin });
                                        if *__slate_slot_1505 != std::ptr::null_mut::<Window>() {
                                            unsafe { sqlite3WindowCodeInit(pParse, p) };
                                        }
                                        0 as i32;
                                        // /* Begin the database scan. */
                                        {}
                                        *__slate_slot_1467 = unsafe {
                                            sqlite3WhereBegin(
                                                pParse,
                                                *__slate_slot_1471,
                                                *__slate_slot_1472,
                                                (*__slate_slot_1478).pOrderBy,
                                                unsafe { (*p).pEList },
                                                p,
                                                *__slate_slot_1504,
                                                (unsafe { (*p).nSelectRow }) as i32,
                                            )
                                        };
                                        if *__slate_slot_1467 == std::ptr::null_mut::<WhereInfo>() {
                                            break '__join_0;
                                        } else {
                                            if ((unsafe {
                                                sqlite3WhereOutputRowCount(*__slate_slot_1467)
                                            })
                                                as i32)
                                                < ((unsafe { (*p).nSelectRow }) as i32)
                                            {
                                                unsafe {
                                                    (*p).nSelectRow = unsafe {
                                                        sqlite3WhereOutputRowCount(
                                                            *__slate_slot_1467,
                                                        )
                                                    };
                                                }
                                                if (((unsafe { (*pDest).eDest }) as u32) as i32)
                                                    <= (4 as i32)
                                                    && (((unsafe { (*pDest).eDest }) as u32) as i32)
                                                        >= (3 as i32)
                                                {
                                                    // /* TUNING: For a UNION CTE, because UNION is implies DISTINCT,
                                                    //         ** reduce the estimated output row count by 8 (LogEst 30).
                                                    //         ** Search for tag-20250414a to see other cases */
                                                    std::ptr::write(__slate_slot_2361, p);
                                                    std::ptr::write(__slate_slot_2362, unsafe {
                                                        (*(*__slate_slot_2361)).nSelectRow
                                                    });
                                                    std::ptr::write(
                                                        __slate_slot_2363,
                                                        ((*__slate_slot_2362 as i32) - (30 as i32))
                                                            as i16,
                                                    );
                                                    unsafe {
                                                        (*(*__slate_slot_2361)).nSelectRow =
                                                            *__slate_slot_2363;
                                                    }
                                                }
                                            }
                                            if (*__slate_slot_1477).isTnct != (0 as u8) {
                                                *__slate_slot_2364 = (unsafe {
                                                    sqlite3WhereIsDistinct(*__slate_slot_1467)
                                                }) != (0 as i32);
                                            } else {
                                                *__slate_slot_2364 = false as bool;
                                            }
                                            if *__slate_slot_2364 {
                                                (*__slate_slot_1477).eTnctType = ((unsafe {
                                                    sqlite3WhereIsDistinct(*__slate_slot_1467)
                                                })
                                                    as i8)
                                                    as u8;
                                            }
                                            if (*__slate_slot_1478).pOrderBy
                                                != std::ptr::null_mut::<ExprList>()
                                            {
                                                (*__slate_slot_1478).nOBSat = unsafe {
                                                    sqlite3WhereIsOrdered(*__slate_slot_1467)
                                                };
                                                (*__slate_slot_1478).labelOBLopt = unsafe {
                                                    sqlite3WhereOrderByLimitOptLabel(
                                                        *__slate_slot_1467,
                                                    )
                                                };
                                                if (*__slate_slot_1478).nOBSat
                                                    == unsafe {
                                                        (*(*__slate_slot_1478).pOrderBy).nExpr
                                                    }
                                                {
                                                    (*__slate_slot_1478).pOrderBy =
                                                        std::ptr::null_mut::<ExprList>();
                                                }
                                            }
                                            {}
                                            // /* If sorting index that was created by a prior OP_OpenEphemeral
                                            //     ** instruction ended up not being needed, then change the OP_OpenEphemeral
                                            //     ** into an OP_Noop.
                                            //     */
                                            if (*__slate_slot_1478).addrSortIndex >= (0 as i32)
                                                && (*__slate_slot_1478).pOrderBy
                                                    == std::ptr::null_mut::<ExprList>()
                                            {
                                                unsafe {
                                                    sqlite3VdbeChangeToNoop(
                                                        *__slate_slot_1468,
                                                        (*__slate_slot_1478).addrSortIndex,
                                                    )
                                                };
                                            }
                                            0 as i32;
                                            if *__slate_slot_1505 != std::ptr::null_mut::<Window>()
                                            {
                                                std::ptr::write(__slate_slot_1506, unsafe {
                                                    sqlite3VdbeMakeLabel(pParse)
                                                });
                                                std::ptr::write(__slate_slot_1507, unsafe {
                                                    sqlite3VdbeMakeLabel(pParse)
                                                });
                                                std::ptr::write(__slate_slot_1508, unsafe {
                                                    sqlite3VdbeMakeLabel(pParse)
                                                });
                                                std::ptr::write(__slate_slot_2365, pParse);
                                                std::ptr::write(__slate_slot_2366, unsafe {
                                                    (*(*__slate_slot_2365)).nMem
                                                });
                                                std::ptr::write(
                                                    __slate_slot_2367,
                                                    *__slate_slot_2366 + (1 as i32),
                                                );
                                                unsafe {
                                                    (*(*__slate_slot_2365)).nMem =
                                                        *__slate_slot_2367;
                                                }
                                                *__slate_slot_1509 = *__slate_slot_2367;
                                                unsafe {
                                                    sqlite3WindowCodeStep(
                                                        pParse,
                                                        p,
                                                        *__slate_slot_1467,
                                                        *__slate_slot_1509,
                                                        *__slate_slot_1506,
                                                    )
                                                };
                                                unsafe {
                                                    sqlite3VdbeAddOp2(
                                                        *__slate_slot_1468,
                                                        9 as i32,
                                                        0 as i32,
                                                        *__slate_slot_1508,
                                                    )
                                                };
                                                unsafe {
                                                    sqlite3VdbeResolveLabel(
                                                        *__slate_slot_1468,
                                                        *__slate_slot_1506,
                                                    )
                                                };
                                                {}
                                                (*__slate_slot_1478).labelOBLopt = 0 as i32;
                                                selectInnerLoop(
                                                    pParse,
                                                    p,
                                                    -(1 as i32),
                                                    std::ptr::addr_of_mut!(*__slate_slot_1478),
                                                    std::ptr::addr_of_mut!(*__slate_slot_1477),
                                                    pDest,
                                                    *__slate_slot_1507,
                                                    *__slate_slot_1508,
                                                );
                                                unsafe {
                                                    sqlite3VdbeResolveLabel(
                                                        *__slate_slot_1468,
                                                        *__slate_slot_1507,
                                                    )
                                                };
                                                unsafe {
                                                    sqlite3VdbeAddOp1(
                                                        *__slate_slot_1468,
                                                        69 as i32,
                                                        *__slate_slot_1509,
                                                    )
                                                };
                                                {}
                                                unsafe {
                                                    sqlite3VdbeResolveLabel(
                                                        *__slate_slot_1468,
                                                        *__slate_slot_1508,
                                                    )
                                                };
                                            } else {
                                                // /* Use the standard inner loop. */
                                                selectInnerLoop(
                                                    pParse,
                                                    p,
                                                    -(1 as i32),
                                                    std::ptr::addr_of_mut!(*__slate_slot_1478),
                                                    std::ptr::addr_of_mut!(*__slate_slot_1477),
                                                    pDest,
                                                    unsafe {
                                                        sqlite3WhereContinueLabel(
                                                            *__slate_slot_1467,
                                                        )
                                                    },
                                                    unsafe {
                                                        sqlite3WhereBreakLabel(*__slate_slot_1467)
                                                    },
                                                );
                                                // /* End the database scan loop.
                                                //       */
                                                {}
                                                unsafe { sqlite3WhereEnd(*__slate_slot_1467) };
                                            }
                                            // /* SQLITE_OMIT_WINDOWFUNC */
                                        }
                                    } else {
                                        // /* This case is for when there exist aggregate functions or a GROUP BY
                                        //     ** clause or both.  tag-select-0800 */
                                        // /* Name context for processing aggregate information */
                                        // /* First Mem address for storing current GROUP BY */
                                        // /* First Mem address for previous GROUP BY */
                                        // /* Mem address holding flag indicating that at least
                                        //                         ** one row of the input to the aggregator has been
                                        //                         ** processed */
                                        // /* Mem address which causes query abort if positive */
                                        // /* Rows come from source in GROUP BY order */
                                        // /* End of processing for this SELECT */
                                        // /* Pseudotable used to decode sorting results */
                                        std::ptr::write(__slate_slot_1517, 0 as i32);
                                        // /* Output register from the sorter */
                                        std::ptr::write(__slate_slot_1518, 0 as i32);
                                        // /* True if the GROUP BY and ORDER BY are the same */
                                        std::ptr::write(__slate_slot_1519, 0 as i32);
                                        // /* Remove any and all aliases between the result set and the
                                        //     ** GROUP BY clause.
                                        //     */
                                        if *__slate_slot_1473 != std::ptr::null_mut::<ExprList>() {
                                            // /* Loop counter */
                                            // /* For looping over expression in a list */
                                            *__slate_slot_1520 =
                                                unsafe { (*unsafe { (*p).pEList }).nExpr };
                                            std::ptr::write(__slate_slot_2368, unsafe {
                                                std::ptr::addr_of_mut!((*unsafe { (*p).pEList }).a)
                                                    as *mut ExprList_item
                                            });
                                            *__slate_slot_1521 = *__slate_slot_2368;
                                            loop {
                                                if *__slate_slot_1520 > (0 as i32) {
                                                    unsafe {
                                                        (*(*__slate_slot_1521)).u.x.iAlias =
                                                            ((0 as i32) as i16) as u16;
                                                    }
                                                    std::ptr::write(
                                                        __slate_slot_2369,
                                                        *__slate_slot_1520,
                                                    );
                                                    std::ptr::write(
                                                        __slate_slot_2370,
                                                        *__slate_slot_2369 - (1 as i32),
                                                    );
                                                    *__slate_slot_1520 = *__slate_slot_2370;
                                                    std::ptr::write(
                                                        __slate_slot_2371,
                                                        *__slate_slot_1521,
                                                    );
                                                    std::ptr::write(__slate_slot_2372, unsafe {
                                                        (*__slate_slot_2371)
                                                            .offset((1 as i32) as isize)
                                                    });
                                                    *__slate_slot_1521 = *__slate_slot_2372;
                                                } else {
                                                    break;
                                                }
                                            }
                                            *__slate_slot_1520 =
                                                unsafe { (*(*__slate_slot_1473)).nExpr };
                                            std::ptr::write(__slate_slot_2373, unsafe {
                                                std::ptr::addr_of_mut!((*(*__slate_slot_1473)).a)
                                                    as *mut ExprList_item
                                            });
                                            *__slate_slot_1521 = *__slate_slot_2373;
                                            loop {
                                                if *__slate_slot_1520 > (0 as i32) {
                                                    unsafe {
                                                        (*(*__slate_slot_1521)).u.x.iAlias =
                                                            ((0 as i32) as i16) as u16;
                                                    }
                                                    std::ptr::write(
                                                        __slate_slot_2374,
                                                        *__slate_slot_1520,
                                                    );
                                                    std::ptr::write(
                                                        __slate_slot_2375,
                                                        *__slate_slot_2374 - (1 as i32),
                                                    );
                                                    *__slate_slot_1520 = *__slate_slot_2375;
                                                    std::ptr::write(
                                                        __slate_slot_2376,
                                                        *__slate_slot_1521,
                                                    );
                                                    std::ptr::write(__slate_slot_2377, unsafe {
                                                        (*__slate_slot_2376)
                                                            .offset((1 as i32) as isize)
                                                    });
                                                    *__slate_slot_1521 = *__slate_slot_2377;
                                                } else {
                                                    break;
                                                }
                                            }
                                            0 as i32;
                                            if ((unsafe { (*p).nSelectRow }) as i32) > (66 as i32) {
                                                unsafe {
                                                    (*p).nSelectRow = (66 as i32) as i16;
                                                }
                                            }
                                            // /* If there is both a GROUP BY and an ORDER BY clause and they are
                                            //       ** identical, then it may be possible to disable the ORDER BY clause
                                            //       ** on the grounds that the GROUP BY will cause elements to come out
                                            //       ** in the correct order. It also may not - the GROUP BY might use a
                                            //       ** database index that causes rows to be grouped together as required
                                            //       ** but not actually sorted. Either way, record the fact that the
                                            //       ** ORDER BY and GROUP BY clauses are the same by setting the orderByGrp
                                            //       ** variable.  */
                                            if sqlite3CopySortOrder(
                                                *__slate_slot_1473,
                                                (*__slate_slot_1478).pOrderBy,
                                            ) != (0 as i32)
                                            {
                                                *__slate_slot_2378 = (unsafe {
                                                    sqlite3ExprListCompare(
                                                        *__slate_slot_1473 as *const ExprList,
                                                        (*__slate_slot_1478).pOrderBy
                                                            as *const ExprList,
                                                        -(1 as i32),
                                                    )
                                                }) == (0 as i32);
                                            } else {
                                                *__slate_slot_2378 = false as bool;
                                            }
                                            if *__slate_slot_2378 {
                                                *__slate_slot_1519 = 1 as i32;
                                            }
                                        } else {
                                            0 as i32;
                                            unsafe {
                                                (*p).nSelectRow = (0 as i32) as i16;
                                            }
                                        }
                                        // /* Create a label to jump to when we want to abort the query */
                                        *__slate_slot_1516 =
                                            unsafe { sqlite3VdbeMakeLabel(pParse) };
                                        // /* Convert TK_COLUMN nodes into TK_AGG_COLUMN and make entries in
                                        //     ** sAggInfo for all TK_AGG_FUNCTION nodes in expressions of the
                                        //     ** SELECT statement.
                                        //     */
                                        *__slate_slot_1475 = (unsafe {
                                            sqlite3DbMallocZero(*__slate_slot_1480, 64 as u64)
                                        })
                                            as *mut AggInfo;
                                        if *__slate_slot_1475 != std::ptr::null_mut::<AggInfo>() {
                                            unsafe {
                                                sqlite3ParserAddCleanup(
                                                    pParse,
                                                    Some(agginfoFree),
                                                    *__slate_slot_1475 as *mut (),
                                                )
                                            };
                                            {}
                                        }
                                        if (unsafe { (*(*__slate_slot_1480)).mallocFailed })
                                            != (0 as u8)
                                        {
                                            break '__join_0;
                                        } else {
                                            unsafe {
                                                (*(*__slate_slot_1475)).selId =
                                                    unsafe { (*p).selId };
                                            }
                                            unsafe {
                                                memset(
                                                    std::ptr::addr_of_mut!(*__slate_slot_1510)
                                                        as *mut (),
                                                    0 as i32,
                                                    56 as u64,
                                                )
                                            };
                                            (*__slate_slot_1510).pParse = pParse;
                                            (*__slate_slot_1510).pSrcList = *__slate_slot_1471;
                                            unsafe {
                                                (*__slate_slot_1510).uNC.pAggInfo =
                                                    *__slate_slot_1475;
                                            }
                                            unsafe {
                                                (*(*__slate_slot_1475)).nSortingColumn =
                                                    (if *__slate_slot_1473
                                                        != std::ptr::null_mut::<ExprList>()
                                                    {
                                                        unsafe { (*(*__slate_slot_1473)).nExpr }
                                                    } else {
                                                        0 as i32
                                                    })
                                                        as u32;
                                            }
                                            unsafe {
                                                (*(*__slate_slot_1475)).pGroupBy =
                                                    *__slate_slot_1473;
                                            }
                                            unsafe {
                                                sqlite3ExprAnalyzeAggList(
                                                    std::ptr::addr_of_mut!(*__slate_slot_1510),
                                                    *__slate_slot_1470,
                                                )
                                            };
                                            unsafe {
                                                sqlite3ExprAnalyzeAggList(
                                                    std::ptr::addr_of_mut!(*__slate_slot_1510),
                                                    (*__slate_slot_1478).pOrderBy,
                                                )
                                            };
                                            if *__slate_slot_1474 != std::ptr::null_mut::<Expr>() {
                                                if *__slate_slot_1473
                                                    != std::ptr::null_mut::<ExprList>()
                                                {
                                                    0 as i32;
                                                    0 as i32;
                                                    0 as i32;
                                                    havingToWhere(pParse, p);
                                                    *__slate_slot_1472 = unsafe { (*p).pWhere };
                                                }
                                                unsafe {
                                                    sqlite3ExprAnalyzeAggregates(
                                                        std::ptr::addr_of_mut!(*__slate_slot_1510),
                                                        *__slate_slot_1474,
                                                    )
                                                };
                                            }
                                            unsafe {
                                                (*(*__slate_slot_1475)).nAccumulator =
                                                    unsafe { (*(*__slate_slot_1475)).nColumn };
                                            }
                                            if (unsafe { (*p).pGroupBy })
                                                == std::ptr::null_mut::<ExprList>()
                                                && (unsafe { (*p).pHaving })
                                                    == std::ptr::null_mut::<Expr>()
                                                && (unsafe { (*(*__slate_slot_1475)).nFunc })
                                                    == (1 as i32)
                                            {
                                                *__slate_slot_1482 = minMaxQuery(
                                                    *__slate_slot_1480,
                                                    unsafe {
                                                        (*unsafe {
                                                            unsafe { (*(*__slate_slot_1475)).aFunc }
                                                                .offset((0 as i32) as isize)
                                                        })
                                                        .pFExpr
                                                    },
                                                    std::ptr::addr_of_mut!(*__slate_slot_1481),
                                                );
                                            } else {
                                                *__slate_slot_1482 = ((0 as i32) as i8) as u8;
                                            }
                                            analyzeAggFuncArgs(
                                                *__slate_slot_1475,
                                                std::ptr::addr_of_mut!(*__slate_slot_1510),
                                            );
                                            if (unsafe { (*(*__slate_slot_1480)).mallocFailed })
                                                != (0 as u8)
                                            {
                                                break '__join_0;
                                            } else {
                                                // /* Processing for aggregates with GROUP BY is very different and
                                                //     ** much more complex than aggregates without a GROUP BY.  tag-select-0810
                                                //     */
                                                if *__slate_slot_1473
                                                    != std::ptr::null_mut::<ExprList>()
                                                {
                                                    // /* Keying information for the group by clause */
                                                    // /* A-vs-B comparison jump */
                                                    // /* Start of subroutine that outputs a result row */
                                                    // /* Return address register for output subroutine */
                                                    // /* Set the abort flag and return */
                                                    // /* Top of the input loop */
                                                    // /* The OP_OpenEphemeral for the sorting index */
                                                    // /* Subroutine for resetting the accumulator */
                                                    // /* Return address register for reset subroutine */
                                                    std::ptr::write(
                                                        __slate_slot_1531,
                                                        std::ptr::null_mut::<ExprList>(),
                                                    );
                                                    std::ptr::write(
                                                        __slate_slot_1532,
                                                        ((0 as i32) as i16) as u16,
                                                    );
                                                    std::ptr::write(__slate_slot_1533, 0 as i32);
                                                    if (unsafe { (*(*__slate_slot_1475)).nFunc })
                                                        == (1 as i32)
                                                        && (unsafe {
                                                            (*unsafe {
                                                                unsafe {
                                                                    (*(*__slate_slot_1475)).aFunc
                                                                }
                                                                .offset((0 as i32) as isize)
                                                            })
                                                            .iDistinct
                                                        }) >= (0 as i32)
                                                        && (unsafe {
                                                            (*unsafe {
                                                                unsafe {
                                                                    (*(*__slate_slot_1475)).aFunc
                                                                }
                                                                .offset((0 as i32) as isize)
                                                            })
                                                            .pFExpr
                                                        }) != std::ptr::null_mut::<Expr>()
                                                        && (unsafe {
                                                            (*unsafe {
                                                                (*unsafe {
                                                                    unsafe {
                                                                        (*(*__slate_slot_1475))
                                                                            .aFunc
                                                                    }
                                                                    .offset((0 as i32) as isize)
                                                                })
                                                                .pFExpr
                                                            })
                                                            .flags
                                                        }) & ((4096 as i32) as u32)
                                                            == ((0 as i32) as u32)
                                                        && (unsafe {
                                                            (*unsafe {
                                                                (*unsafe {
                                                                    unsafe {
                                                                        (*(*__slate_slot_1475))
                                                                            .aFunc
                                                                    }
                                                                    .offset((0 as i32) as isize)
                                                                })
                                                                .pFExpr
                                                            })
                                                            .x
                                                            .pList
                                                        }) != std::ptr::null_mut::<ExprList>()
                                                    {
                                                        std::ptr::write(
                                                            __slate_slot_1534,
                                                            unsafe {
                                                                (*unsafe { unsafe { std::ptr::addr_of_mut!((*unsafe { (*unsafe { (*unsafe { unsafe { (*(*__slate_slot_1475)).aFunc }.offset((0 as i32) as isize) }).pFExpr }).x.pList }).a) as *mut ExprList_item }.offset((0 as i32) as isize) }).pExpr
                                                            },
                                                        );
                                                        *__slate_slot_1534 = unsafe {
                                                            sqlite3ExprDup(
                                                                *__slate_slot_1480,
                                                                *__slate_slot_1534 as *const Expr,
                                                                0 as i32,
                                                            )
                                                        };
                                                        *__slate_slot_1531 = unsafe {
                                                            sqlite3ExprListDup(
                                                                *__slate_slot_1480,
                                                                *__slate_slot_1473
                                                                    as *const ExprList,
                                                                0 as i32,
                                                            )
                                                        };
                                                        *__slate_slot_1531 = unsafe {
                                                            sqlite3ExprListAppend(
                                                                pParse,
                                                                *__slate_slot_1531,
                                                                *__slate_slot_1534,
                                                            )
                                                        };
                                                        *__slate_slot_1532 = ((if *__slate_slot_1531
                                                            != std::ptr::null_mut::<ExprList>()
                                                        {
                                                            (256 as i32) | (1024 as i32)
                                                        } else {
                                                            0 as i32
                                                        })
                                                            as i16)
                                                            as u16;
                                                    }
                                                    // /* If there is a GROUP BY clause we might need a sorting index to
                                                    //       ** implement it.  Allocate that sorting index now.  If it turns out
                                                    //       ** that we do not need it after all, the OP_SorterOpen instruction
                                                    //       ** will be converted into a Noop.
                                                    //       */
                                                    std::ptr::write(__slate_slot_2379, pParse);
                                                    std::ptr::write(__slate_slot_2380, unsafe {
                                                        (*(*__slate_slot_2379)).nTab
                                                    });
                                                    std::ptr::write(
                                                        __slate_slot_2381,
                                                        *__slate_slot_2380 + (1 as i32),
                                                    );
                                                    unsafe {
                                                        (*(*__slate_slot_2379)).nTab =
                                                            *__slate_slot_2381;
                                                    }
                                                    unsafe {
                                                        (*(*__slate_slot_1475)).sortingIdx =
                                                            *__slate_slot_2380;
                                                    }
                                                    *__slate_slot_1522 = sqlite3KeyInfoFromExprList(
                                                        pParse,
                                                        *__slate_slot_1473,
                                                        0 as i32,
                                                        unsafe { (*(*__slate_slot_1475)).nColumn },
                                                    );
                                                    *__slate_slot_1528 = unsafe {
                                                        sqlite3VdbeAddOp4(
                                                            *__slate_slot_1468,
                                                            121 as i32,
                                                            unsafe {
                                                                (*(*__slate_slot_1475)).sortingIdx
                                                            },
                                                            (unsafe {
                                                                (*(*__slate_slot_1475))
                                                                    .nSortingColumn
                                                            })
                                                                as i32,
                                                            0 as i32,
                                                            (*__slate_slot_1522 as *mut i8)
                                                                as *const i8,
                                                            -(9 as i32),
                                                        )
                                                    };
                                                    // /* Initialize memory locations used by GROUP BY aggregate processing
                                                    //       */
                                                    std::ptr::write(__slate_slot_2382, pParse);
                                                    std::ptr::write(__slate_slot_2383, unsafe {
                                                        (*(*__slate_slot_2382)).nMem
                                                    });
                                                    std::ptr::write(
                                                        __slate_slot_2384,
                                                        *__slate_slot_2383 + (1 as i32),
                                                    );
                                                    unsafe {
                                                        (*(*__slate_slot_2382)).nMem =
                                                            *__slate_slot_2384;
                                                    }
                                                    *__slate_slot_1513 = *__slate_slot_2384;
                                                    std::ptr::write(__slate_slot_2385, pParse);
                                                    std::ptr::write(__slate_slot_2386, unsafe {
                                                        (*(*__slate_slot_2385)).nMem
                                                    });
                                                    std::ptr::write(
                                                        __slate_slot_2387,
                                                        *__slate_slot_2386 + (1 as i32),
                                                    );
                                                    unsafe {
                                                        (*(*__slate_slot_2385)).nMem =
                                                            *__slate_slot_2387;
                                                    }
                                                    *__slate_slot_1514 = *__slate_slot_2387;
                                                    std::ptr::write(__slate_slot_2388, pParse);
                                                    std::ptr::write(__slate_slot_2389, unsafe {
                                                        (*(*__slate_slot_2388)).nMem
                                                    });
                                                    std::ptr::write(
                                                        __slate_slot_2390,
                                                        *__slate_slot_2389 + (1 as i32),
                                                    );
                                                    unsafe {
                                                        (*(*__slate_slot_2388)).nMem =
                                                            *__slate_slot_2390;
                                                    }
                                                    *__slate_slot_1525 = *__slate_slot_2390;
                                                    *__slate_slot_1524 =
                                                        unsafe { sqlite3VdbeMakeLabel(pParse) };
                                                    std::ptr::write(__slate_slot_2391, pParse);
                                                    std::ptr::write(__slate_slot_2392, unsafe {
                                                        (*(*__slate_slot_2391)).nMem
                                                    });
                                                    std::ptr::write(
                                                        __slate_slot_2393,
                                                        *__slate_slot_2392 + (1 as i32),
                                                    );
                                                    unsafe {
                                                        (*(*__slate_slot_2391)).nMem =
                                                            *__slate_slot_2393;
                                                    }
                                                    *__slate_slot_1530 = *__slate_slot_2393;
                                                    *__slate_slot_1529 =
                                                        unsafe { sqlite3VdbeMakeLabel(pParse) };
                                                    *__slate_slot_1511 =
                                                        (unsafe { (*pParse).nMem }) + (1 as i32);
                                                    std::ptr::write(__slate_slot_2394, pParse);
                                                    std::ptr::write(__slate_slot_2395, unsafe {
                                                        (*(*__slate_slot_2394)).nMem
                                                    });
                                                    std::ptr::write(
                                                        __slate_slot_2396,
                                                        *__slate_slot_2395
                                                            + unsafe {
                                                                (*(*__slate_slot_1473)).nExpr
                                                            },
                                                    );
                                                    unsafe {
                                                        (*(*__slate_slot_2394)).nMem =
                                                            *__slate_slot_2396;
                                                    }
                                                    *__slate_slot_1512 =
                                                        (unsafe { (*pParse).nMem }) + (1 as i32);
                                                    std::ptr::write(__slate_slot_2397, pParse);
                                                    std::ptr::write(__slate_slot_2398, unsafe {
                                                        (*(*__slate_slot_2397)).nMem
                                                    });
                                                    std::ptr::write(
                                                        __slate_slot_2399,
                                                        *__slate_slot_2398
                                                            + unsafe {
                                                                (*(*__slate_slot_1473)).nExpr
                                                            },
                                                    );
                                                    unsafe {
                                                        (*(*__slate_slot_2397)).nMem =
                                                            *__slate_slot_2399;
                                                    }
                                                    unsafe {
                                                        sqlite3VdbeAddOp2(
                                                            *__slate_slot_1468,
                                                            73 as i32,
                                                            0 as i32,
                                                            *__slate_slot_1514,
                                                        )
                                                    };
                                                    {}
                                                    unsafe {
                                                        sqlite3VdbeAddOp3(
                                                            *__slate_slot_1468,
                                                            77 as i32,
                                                            0 as i32,
                                                            *__slate_slot_1511,
                                                            *__slate_slot_1511
                                                                + unsafe {
                                                                    (*(*__slate_slot_1473)).nExpr
                                                                }
                                                                - (1 as i32),
                                                        )
                                                    };
                                                    unsafe {
                                                        sqlite3ExprNullRegisterRange(
                                                            pParse,
                                                            *__slate_slot_1511,
                                                            unsafe {
                                                                (*(*__slate_slot_1473)).nExpr
                                                            },
                                                        )
                                                    };
                                                    // /* Begin a loop that will extract all source rows in GROUP BY order.
                                                    //       ** This might involve two separate loops with an OP_Sort in between, or
                                                    //       ** it might be a single loop that uses an index to extract information
                                                    //       ** in the right order to begin with.
                                                    //       */
                                                    unsafe {
                                                        sqlite3VdbeAddOp2(
                                                            *__slate_slot_1468,
                                                            10 as i32,
                                                            *__slate_slot_1530,
                                                            *__slate_slot_1529,
                                                        )
                                                    };
                                                    {}
                                                    *__slate_slot_1467 = unsafe {
                                                        sqlite3WhereBegin(
                                                            pParse,
                                                            *__slate_slot_1471,
                                                            *__slate_slot_1472,
                                                            *__slate_slot_1473,
                                                            *__slate_slot_1531,
                                                            p,
                                                            (((if (((*__slate_slot_1477).isTnct
                                                                as u32)
                                                                as i32)
                                                                == (2 as i32)
                                                            {
                                                                128 as i32
                                                            } else {
                                                                64 as i32
                                                            }) | if *__slate_slot_1519
                                                                != (0 as i32)
                                                            {
                                                                512 as i32
                                                            } else {
                                                                0 as i32
                                                            } | ((*__slate_slot_1532 as u32)
                                                                as i32))
                                                                as i16)
                                                                as u16,
                                                            0 as i32,
                                                        )
                                                    };
                                                    if *__slate_slot_1467
                                                        == std::ptr::null_mut::<WhereInfo>()
                                                    {
                                                        unsafe {
                                                            sqlite3ExprListDelete(
                                                                *__slate_slot_1480,
                                                                *__slate_slot_1531,
                                                            )
                                                        };
                                                        break '__join_0;
                                                    } else {
                                                        if (unsafe { (*pParse).pIdxEpr })
                                                            != std::ptr::null_mut::<IndexedExpr>()
                                                        {
                                                            optimizeAggregateUseOfIndexedExpr(
                                                                pParse,
                                                                p,
                                                                *__slate_slot_1475,
                                                                std::ptr::addr_of_mut!(
                                                                    *__slate_slot_1510
                                                                ),
                                                            );
                                                        }
                                                        assignAggregateRegisters(
                                                            pParse,
                                                            *__slate_slot_1475,
                                                        );
                                                        *__slate_slot_1533 = unsafe {
                                                            sqlite3WhereIsDistinct(
                                                                *__slate_slot_1467,
                                                            )
                                                        };
                                                        {}
                                                        if (unsafe {
                                                            sqlite3WhereIsOrdered(
                                                                *__slate_slot_1467,
                                                            )
                                                        }) == unsafe {
                                                            (*(*__slate_slot_1473)).nExpr
                                                        } {
                                                            // /* The optimizer is able to deliver rows in group by order so
                                                            //         ** we do not have to sort.  The OP_OpenEphemeral table will be
                                                            //         ** cancelled later because we still need to use the pKeyInfo
                                                            //         */
                                                            *__slate_slot_1515 = 0 as i32;
                                                        } else {
                                                            // /* Rows are coming out in undetermined order.  We have to push
                                                            //         ** each row into a sorting index, terminate the first loop,
                                                            //         ** then loop over the sorting index in order to get the output
                                                            //         ** in sorted order
                                                            //         */
                                                            unsafe {
                                                                sqlite3VdbeExplain(
                                                                    pParse,
                                                                    ((0 as i32) as i8) as u8,
                                                                    (b"USE TEMP B-TREE FOR %s\0"
                                                                        .as_ptr()
                                                                        as *mut i8)
                                                                        as *const i8,
                                                                    if (*__slate_slot_1477).isTnct
                                                                        != (0 as u8)
                                                                        && (unsafe {
                                                                            (*p).selFlags
                                                                        }) & ((1 as i32) as u32)
                                                                            == ((0 as i32) as u32)
                                                                    {
                                                                        b"DISTINCT\0".as_ptr()
                                                                            as *mut i8
                                                                    } else {
                                                                        b"GROUP BY\0".as_ptr()
                                                                            as *mut i8
                                                                    },
                                                                )
                                                            };
                                                            *__slate_slot_1515 = 1 as i32;
                                                            *__slate_slot_1538 = unsafe {
                                                                (*(*__slate_slot_1473)).nExpr
                                                            };
                                                            *__slate_slot_1537 = *__slate_slot_1538;
                                                            *__slate_slot_1466 = *__slate_slot_1538;
                                                            *__slate_slot_1465 = 0 as i32;
                                                            loop {
                                                                if *__slate_slot_1465
                                                                    < unsafe {
                                                                        (*(*__slate_slot_1475))
                                                                            .nColumn
                                                                    }
                                                                {
                                                                    if (unsafe {
                                                                        (*unsafe { unsafe { (*(*__slate_slot_1475)).aCol }.offset(*__slate_slot_1465 as isize) }).iSorterColumn
                                                                    }) >= *__slate_slot_1466
                                                                    {
                                                                        std::ptr::write(
                                                                            __slate_slot_2402,
                                                                            *__slate_slot_1537,
                                                                        );
                                                                        std::ptr::write(
                                                                            __slate_slot_2403,
                                                                            *__slate_slot_2402
                                                                                + (1 as i32),
                                                                        );
                                                                        *__slate_slot_1537 =
                                                                            *__slate_slot_2403;
                                                                        std::ptr::write(
                                                                            __slate_slot_2404,
                                                                            *__slate_slot_1466,
                                                                        );
                                                                        std::ptr::write(
                                                                            __slate_slot_2405,
                                                                            *__slate_slot_2404
                                                                                + (1 as i32),
                                                                        );
                                                                        *__slate_slot_1466 =
                                                                            *__slate_slot_2405;
                                                                    }
                                                                    std::ptr::write(
                                                                        __slate_slot_2400,
                                                                        *__slate_slot_1465,
                                                                    );
                                                                    std::ptr::write(
                                                                        __slate_slot_2401,
                                                                        *__slate_slot_2400
                                                                            + (1 as i32),
                                                                    );
                                                                    *__slate_slot_1465 =
                                                                        *__slate_slot_2401;
                                                                } else {
                                                                    break;
                                                                }
                                                            }
                                                            *__slate_slot_1535 = unsafe {
                                                                sqlite3GetTempRange(
                                                                    pParse,
                                                                    *__slate_slot_1537,
                                                                )
                                                            };
                                                            unsafe {
                                                                sqlite3ExprCodeExprList(
                                                                    pParse,
                                                                    *__slate_slot_1473,
                                                                    *__slate_slot_1535,
                                                                    0 as i32,
                                                                    ((0 as i32) as i8) as u8,
                                                                )
                                                            };
                                                            *__slate_slot_1466 = *__slate_slot_1538;
                                                            unsafe {
                                                                (*(*__slate_slot_1475))
                                                                    .directMode =
                                                                    ((1 as i32) as i8) as u8;
                                                            }
                                                            *__slate_slot_1465 = 0 as i32;
                                                            loop {
                                                                if *__slate_slot_1465
                                                                    < unsafe {
                                                                        (*(*__slate_slot_1475))
                                                                            .nColumn
                                                                    }
                                                                {
                                                                    std::ptr::write(
                                                                        __slate_slot_1539,
                                                                        unsafe {
                                                                            unsafe { (*(*__slate_slot_1475)).aCol }.offset(*__slate_slot_1465 as isize)
                                                                        },
                                                                    );
                                                                    if (unsafe {
                                                                        (*(*__slate_slot_1539))
                                                                            .iSorterColumn
                                                                    }) >= *__slate_slot_1466
                                                                    {
                                                                        unsafe {
                                                                            sqlite3ExprCode(pParse, unsafe { (*(*__slate_slot_1539)).pCExpr }, *__slate_slot_1466 + *__slate_slot_1535)
                                                                        };
                                                                        std::ptr::write(
                                                                            __slate_slot_2408,
                                                                            *__slate_slot_1466,
                                                                        );
                                                                        std::ptr::write(
                                                                            __slate_slot_2409,
                                                                            *__slate_slot_2408
                                                                                + (1 as i32),
                                                                        );
                                                                        *__slate_slot_1466 =
                                                                            *__slate_slot_2409;
                                                                    }
                                                                    std::ptr::write(
                                                                        __slate_slot_2406,
                                                                        *__slate_slot_1465,
                                                                    );
                                                                    std::ptr::write(
                                                                        __slate_slot_2407,
                                                                        *__slate_slot_2406
                                                                            + (1 as i32),
                                                                    );
                                                                    *__slate_slot_1465 =
                                                                        *__slate_slot_2407;
                                                                } else {
                                                                    break;
                                                                }
                                                            }
                                                            unsafe {
                                                                (*(*__slate_slot_1475))
                                                                    .directMode =
                                                                    ((0 as i32) as i8) as u8;
                                                            }
                                                            *__slate_slot_1536 = unsafe {
                                                                sqlite3GetTempReg(pParse)
                                                            };
                                                            {}
                                                            unsafe {
                                                                sqlite3VdbeAddOp3(
                                                                    *__slate_slot_1468,
                                                                    99 as i32,
                                                                    *__slate_slot_1535,
                                                                    *__slate_slot_1537,
                                                                    *__slate_slot_1536,
                                                                )
                                                            };
                                                            unsafe {
                                                                sqlite3VdbeAddOp2(
                                                                    *__slate_slot_1468,
                                                                    141 as i32,
                                                                    unsafe {
                                                                        (*(*__slate_slot_1475))
                                                                            .sortingIdx
                                                                    },
                                                                    *__slate_slot_1536,
                                                                )
                                                            };
                                                            {}
                                                            unsafe {
                                                                sqlite3ReleaseTempReg(
                                                                    pParse,
                                                                    *__slate_slot_1536,
                                                                )
                                                            };
                                                            unsafe {
                                                                sqlite3ReleaseTempRange(
                                                                    pParse,
                                                                    *__slate_slot_1535,
                                                                    *__slate_slot_1537,
                                                                )
                                                            };
                                                            {}
                                                            unsafe {
                                                                sqlite3WhereEnd(*__slate_slot_1467)
                                                            };
                                                            std::ptr::write(
                                                                __slate_slot_2410,
                                                                pParse,
                                                            );
                                                            std::ptr::write(
                                                                __slate_slot_2411,
                                                                unsafe {
                                                                    (*(*__slate_slot_2410)).nTab
                                                                },
                                                            );
                                                            std::ptr::write(
                                                                __slate_slot_2412,
                                                                *__slate_slot_2411 + (1 as i32),
                                                            );
                                                            unsafe {
                                                                (*(*__slate_slot_2410)).nTab =
                                                                    *__slate_slot_2412;
                                                            }
                                                            std::ptr::write(
                                                                __slate_slot_2413,
                                                                *__slate_slot_2411,
                                                            );
                                                            *__slate_slot_1517 = *__slate_slot_2413;
                                                            unsafe {
                                                                (*(*__slate_slot_1475))
                                                                    .sortingIdxPTab =
                                                                    *__slate_slot_2413;
                                                            }
                                                            *__slate_slot_1518 = unsafe {
                                                                sqlite3GetTempReg(pParse)
                                                            };
                                                            {}
                                                            unsafe {
                                                                sqlite3VdbeAddOp3(
                                                                    *__slate_slot_1468,
                                                                    123 as i32,
                                                                    *__slate_slot_1517,
                                                                    *__slate_slot_1518,
                                                                    *__slate_slot_1537,
                                                                )
                                                            };
                                                            unsafe {
                                                                sqlite3VdbeAddOp2(
                                                                    *__slate_slot_1468,
                                                                    34 as i32,
                                                                    unsafe {
                                                                        (*(*__slate_slot_1475))
                                                                            .sortingIdx
                                                                    },
                                                                    *__slate_slot_1516,
                                                                )
                                                            };
                                                            {}
                                                            {}
                                                            unsafe {
                                                                (*(*__slate_slot_1475))
                                                                    .useSortingIdx =
                                                                    ((1 as i32) as i8) as u8;
                                                            }
                                                            {}
                                                            {}
                                                        }
                                                        // /* If there are entries in pAgggInfo->aFunc[] that contain subexpressions
                                                        //       ** that are indexed (and that were previously identified and tagged
                                                        //       ** in optimizeAggregateUseOfIndexedExpr()) then those subexpressions
                                                        //       ** must now be converted into a TK_AGG_COLUMN node so that the value
                                                        //       ** is correctly pulled from the index rather than being recomputed. */
                                                        if (unsafe { (*pParse).pIdxEpr })
                                                            != std::ptr::null_mut::<IndexedExpr>()
                                                        {
                                                            aggregateConvertIndexedExprRefToColumn(
                                                                *__slate_slot_1475,
                                                            );
                                                        }
                                                        // /* If the index or temporary table used by the GROUP BY sort
                                                        //       ** will naturally deliver rows in the order required by the ORDER BY
                                                        //       ** clause, cancel the ephemeral table open coded earlier.
                                                        //       **
                                                        //       ** This is an optimization - the correct answer should result regardless.
                                                        //       ** Use the SQLITE_GroupByOrder flag with SQLITE_TESTCTRL_OPTIMIZER to
                                                        //       ** disable this optimization for testing purposes.  */
                                                        if *__slate_slot_1519 != (0 as i32)
                                                            && (unsafe {
                                                                (*(*__slate_slot_1480)).dbOptFlags
                                                            }) & ((4 as i32) as u32)
                                                                == ((0 as i32) as u32)
                                                        {
                                                            if *__slate_slot_1515 != (0 as i32) {
                                                                *__slate_slot_2415 = true as bool;
                                                            } else {
                                                                *__slate_slot_2415 = (unsafe {
                                                                    sqlite3WhereIsSorted(
                                                                        *__slate_slot_1467,
                                                                    )
                                                                }) != (0
                                                                    as i32);
                                                            }
                                                            *__slate_slot_2414 = *__slate_slot_2415;
                                                        } else {
                                                            *__slate_slot_2414 = false as bool;
                                                        }
                                                        if *__slate_slot_2414 {
                                                            (*__slate_slot_1478).pOrderBy =
                                                                std::ptr::null_mut::<ExprList>();
                                                            unsafe {
                                                                sqlite3VdbeChangeToNoop(
                                                                    *__slate_slot_1468,
                                                                    (*__slate_slot_1478)
                                                                        .addrSortIndex,
                                                                )
                                                            };
                                                        }
                                                        // /* Evaluate the current GROUP BY terms and store in b0, b1, b2...
                                                        //       ** (b0 is memory location iBMem+0, b1 is iBMem+1, and so forth)
                                                        //       ** Then compare the current GROUP BY terms against the GROUP BY terms
                                                        //       ** from the previous row currently stored in a0, a1, a2...
                                                        //       */
                                                        *__slate_slot_1527 = unsafe {
                                                            sqlite3VdbeCurrentAddr(
                                                                *__slate_slot_1468,
                                                            )
                                                        };
                                                        if *__slate_slot_1515 != (0 as i32) {
                                                            unsafe {
                                                                sqlite3VdbeAddOp3(
                                                                    *__slate_slot_1468,
                                                                    135 as i32,
                                                                    unsafe {
                                                                        (*(*__slate_slot_1475))
                                                                            .sortingIdx
                                                                    },
                                                                    *__slate_slot_1518,
                                                                    *__slate_slot_1517,
                                                                )
                                                            };
                                                        }
                                                        *__slate_slot_1466 = 0 as i32;
                                                        loop {
                                                            if *__slate_slot_1466
                                                                < unsafe {
                                                                    (*(*__slate_slot_1473)).nExpr
                                                                }
                                                            {
                                                                std::ptr::write(
                                                                    __slate_slot_1540,
                                                                    ((unsafe {
                                                                        (*unsafe { unsafe { std::ptr::addr_of_mut!((*(*__slate_slot_1473)).a) as *mut ExprList_item }.offset(*__slate_slot_1466 as isize) }).u.x.iOrderByCol
                                                                    })
                                                                        as u32)
                                                                        as i32,
                                                                );
                                                                if *__slate_slot_1515 != (0 as i32)
                                                                {
                                                                    unsafe {
                                                                        sqlite3VdbeAddOp3(*__slate_slot_1468, 96 as i32, *__slate_slot_1517, *__slate_slot_1466, *__slate_slot_1512 + *__slate_slot_1466)
                                                                    };
                                                                } else {
                                                                    unsafe {
                                                                        (*(*__slate_slot_1475))
                                                                            .directMode =
                                                                            ((1 as i32) as i8)
                                                                                as u8;
                                                                    }
                                                                    unsafe {
                                                                        sqlite3ExprCode(pParse, unsafe { (*unsafe { unsafe { std::ptr::addr_of_mut!((*(*__slate_slot_1473)).a) as *mut ExprList_item }.offset(*__slate_slot_1466 as isize) }).pExpr }, *__slate_slot_1512 + *__slate_slot_1466)
                                                                    };
                                                                }
                                                                if *__slate_slot_1540 != (0 as i32)
                                                                {
                                                                    std::ptr::write(
                                                                        __slate_slot_1541,
                                                                        unsafe {
                                                                            (*unsafe { unsafe { std::ptr::addr_of_mut!((*unsafe { (*p).pEList }).a) as *mut ExprList_item }.offset((*__slate_slot_1540 - (1 as i32)) as isize) }).pExpr
                                                                        },
                                                                    );
                                                                    std::ptr::write(
                                                                        __slate_slot_1542,
                                                                        unsafe {
                                                                            sqlite3ExprSkipCollateAndLikely(*__slate_slot_1541)
                                                                        },
                                                                    );
                                                                    loop {
                                                                        if *__slate_slot_1542
                                                                            != std::ptr::null_mut::<
                                                                                Expr,
                                                                            >(
                                                                            )
                                                                            && (((unsafe {
                                                                                (*(*__slate_slot_1542)).op
                                                                            })
                                                                                as u32)
                                                                                as i32)
                                                                                == (179 as i32)
                                                                        {
                                                                            *__slate_slot_1541 = unsafe {
                                                                                (*(*__slate_slot_1542)).pLeft
                                                                            };
                                                                            *__slate_slot_1542 = unsafe {
                                                                                sqlite3ExprSkipCollateAndLikely(*__slate_slot_1541)
                                                                            };
                                                                        } else {
                                                                            break;
                                                                        }
                                                                    }
                                                                    if *__slate_slot_1542
                                                                        != std::ptr::null_mut::<Expr>(
                                                                        )
                                                                        && (((unsafe {
                                                                            (*(*__slate_slot_1542))
                                                                                .op
                                                                        })
                                                                            as u32)
                                                                            as i32)
                                                                            != (170 as i32)
                                                                        && (((unsafe {
                                                                            (*(*__slate_slot_1542))
                                                                                .op
                                                                        })
                                                                            as u32)
                                                                            as i32)
                                                                            != (176 as i32)
                                                                    {
                                                                        unsafe {
                                                                            sqlite3ExprToRegister(*__slate_slot_1541, *__slate_slot_1511 + *__slate_slot_1466)
                                                                        };
                                                                    }
                                                                }
                                                                std::ptr::write(
                                                                    __slate_slot_2416,
                                                                    *__slate_slot_1466,
                                                                );
                                                                std::ptr::write(
                                                                    __slate_slot_2417,
                                                                    *__slate_slot_2416 + (1 as i32),
                                                                );
                                                                *__slate_slot_1466 =
                                                                    *__slate_slot_2417;
                                                            } else {
                                                                break;
                                                            }
                                                        }
                                                        unsafe {
                                                            sqlite3VdbeAddOp4(
                                                                *__slate_slot_1468,
                                                                92 as i32,
                                                                *__slate_slot_1511,
                                                                *__slate_slot_1512,
                                                                unsafe {
                                                                    (*(*__slate_slot_1473)).nExpr
                                                                },
                                                                (sqlite3KeyInfoRef(
                                                                    *__slate_slot_1522,
                                                                )
                                                                    as *mut i8)
                                                                    as *const i8,
                                                                -(9 as i32),
                                                            )
                                                        };
                                                        *__slate_slot_1523 = unsafe {
                                                            sqlite3VdbeCurrentAddr(
                                                                *__slate_slot_1468,
                                                            )
                                                        };
                                                        unsafe {
                                                            sqlite3VdbeAddOp3(
                                                                *__slate_slot_1468,
                                                                14 as i32,
                                                                *__slate_slot_1523 + (1 as i32),
                                                                0 as i32,
                                                                *__slate_slot_1523 + (1 as i32),
                                                            )
                                                        };
                                                        {}
                                                        // /* Generate code that runs whenever the GROUP BY changes.
                                                        //       ** Changes in the GROUP BY are detected by the previous code
                                                        //       ** block.  If there were no changes, this block is skipped.
                                                        //       **
                                                        //       ** This code copies current group by terms in b0,b1,b2,...
                                                        //       ** over to a0,a1,a2.  It then calls the output subroutine
                                                        //       ** and resets the aggregate accumulator registers in preparation
                                                        //       ** for the next GROUP BY batch.
                                                        //       */
                                                        unsafe {
                                                            sqlite3VdbeAddOp2(
                                                                *__slate_slot_1468,
                                                                10 as i32,
                                                                *__slate_slot_1525,
                                                                *__slate_slot_1524,
                                                            )
                                                        };
                                                        {}
                                                        unsafe {
                                                            sqlite3ExprCodeMove(
                                                                pParse,
                                                                *__slate_slot_1512,
                                                                *__slate_slot_1511,
                                                                unsafe {
                                                                    (*(*__slate_slot_1473)).nExpr
                                                                },
                                                            )
                                                        };
                                                        unsafe {
                                                            sqlite3VdbeAddOp2(
                                                                *__slate_slot_1468,
                                                                61 as i32,
                                                                *__slate_slot_1514,
                                                                *__slate_slot_1516,
                                                            )
                                                        };
                                                        {}
                                                        {}
                                                        unsafe {
                                                            sqlite3VdbeAddOp2(
                                                                *__slate_slot_1468,
                                                                10 as i32,
                                                                *__slate_slot_1530,
                                                                *__slate_slot_1529,
                                                            )
                                                        };
                                                        {}
                                                        // /* Update the aggregate accumulators based on the content of
                                                        //       ** the current row
                                                        //       */
                                                        unsafe {
                                                            sqlite3VdbeJumpHere(
                                                                *__slate_slot_1468,
                                                                *__slate_slot_1523,
                                                            )
                                                        };
                                                        updateAccumulator(
                                                            pParse,
                                                            *__slate_slot_1513,
                                                            *__slate_slot_1475,
                                                            *__slate_slot_1533,
                                                        );
                                                        unsafe {
                                                            sqlite3VdbeAddOp2(
                                                                *__slate_slot_1468,
                                                                73 as i32,
                                                                1 as i32,
                                                                *__slate_slot_1513,
                                                            )
                                                        };
                                                        {}
                                                        // /* End of the loop
                                                        //       */
                                                        if *__slate_slot_1515 != (0 as i32) {
                                                            unsafe {
                                                                sqlite3VdbeAddOp2(
                                                                    *__slate_slot_1468,
                                                                    38 as i32,
                                                                    unsafe {
                                                                        (*(*__slate_slot_1475))
                                                                            .sortingIdx
                                                                    },
                                                                    *__slate_slot_1527,
                                                                )
                                                            };
                                                            {}
                                                        } else {
                                                            {}
                                                            unsafe {
                                                                sqlite3WhereEnd(*__slate_slot_1467)
                                                            };
                                                            unsafe {
                                                                sqlite3VdbeChangeToNoop(
                                                                    *__slate_slot_1468,
                                                                    *__slate_slot_1528,
                                                                )
                                                            };
                                                        }
                                                        unsafe {
                                                            sqlite3ExprListDelete(
                                                                *__slate_slot_1480,
                                                                *__slate_slot_1531,
                                                            )
                                                        };
                                                        // /* Output the final row of result
                                                        //       */
                                                        unsafe {
                                                            sqlite3VdbeAddOp2(
                                                                *__slate_slot_1468,
                                                                10 as i32,
                                                                *__slate_slot_1525,
                                                                *__slate_slot_1524,
                                                            )
                                                        };
                                                        {}
                                                        // /* Jump over the subroutines
                                                        //       */
                                                        unsafe {
                                                            sqlite3VdbeGoto(
                                                                *__slate_slot_1468,
                                                                *__slate_slot_1516,
                                                            )
                                                        };
                                                        // /* Generate a subroutine that outputs a single row of the result
                                                        //       ** set.  This subroutine first looks at the iUseFlag.  If iUseFlag
                                                        //       ** is less than or equal to zero, the subroutine is a no-op.  If
                                                        //       ** the processing calls for the query to abort, this subroutine
                                                        //       ** increments the iAbortFlag memory location before returning in
                                                        //       ** order to signal the caller to abort.
                                                        //       */
                                                        *__slate_slot_1526 = unsafe {
                                                            sqlite3VdbeCurrentAddr(
                                                                *__slate_slot_1468,
                                                            )
                                                        };
                                                        unsafe {
                                                            sqlite3VdbeAddOp2(
                                                                *__slate_slot_1468,
                                                                73 as i32,
                                                                1 as i32,
                                                                *__slate_slot_1514,
                                                            )
                                                        };
                                                        {}
                                                        unsafe {
                                                            sqlite3VdbeAddOp1(
                                                                *__slate_slot_1468,
                                                                69 as i32,
                                                                *__slate_slot_1525,
                                                            )
                                                        };
                                                        unsafe {
                                                            sqlite3VdbeResolveLabel(
                                                                *__slate_slot_1468,
                                                                *__slate_slot_1524,
                                                            )
                                                        };
                                                        *__slate_slot_1524 = unsafe {
                                                            sqlite3VdbeCurrentAddr(
                                                                *__slate_slot_1468,
                                                            )
                                                        };
                                                        unsafe {
                                                            sqlite3VdbeAddOp2(
                                                                *__slate_slot_1468,
                                                                61 as i32,
                                                                *__slate_slot_1513,
                                                                *__slate_slot_1524 + (2 as i32),
                                                            )
                                                        };
                                                        {}
                                                        {}
                                                        unsafe {
                                                            sqlite3VdbeAddOp1(
                                                                *__slate_slot_1468,
                                                                69 as i32,
                                                                *__slate_slot_1525,
                                                            )
                                                        };
                                                        finalizeAggFunctions(
                                                            pParse,
                                                            *__slate_slot_1475,
                                                        );
                                                        unsafe {
                                                            sqlite3ExprIfFalse(
                                                                pParse,
                                                                *__slate_slot_1474,
                                                                *__slate_slot_1524 + (1 as i32),
                                                                16 as i32,
                                                            )
                                                        };
                                                        selectInnerLoop(
                                                            pParse,
                                                            p,
                                                            -(1 as i32),
                                                            std::ptr::addr_of_mut!(
                                                                *__slate_slot_1478
                                                            ),
                                                            std::ptr::addr_of_mut!(
                                                                *__slate_slot_1477
                                                            ),
                                                            pDest,
                                                            *__slate_slot_1524 + (1 as i32),
                                                            *__slate_slot_1526,
                                                        );
                                                        unsafe {
                                                            sqlite3VdbeAddOp1(
                                                                *__slate_slot_1468,
                                                                69 as i32,
                                                                *__slate_slot_1525,
                                                            )
                                                        };
                                                        {}
                                                        // /* Generate a subroutine that will reset the group-by accumulator
                                                        //       */
                                                        unsafe {
                                                            sqlite3VdbeResolveLabel(
                                                                *__slate_slot_1468,
                                                                *__slate_slot_1529,
                                                            )
                                                        };
                                                        resetAccumulator(
                                                            pParse,
                                                            *__slate_slot_1475,
                                                        );
                                                        unsafe {
                                                            sqlite3VdbeAddOp2(
                                                                *__slate_slot_1468,
                                                                73 as i32,
                                                                0 as i32,
                                                                *__slate_slot_1513,
                                                            )
                                                        };
                                                        {}
                                                        unsafe {
                                                            sqlite3VdbeAddOp1(
                                                                *__slate_slot_1468,
                                                                69 as i32,
                                                                *__slate_slot_1530,
                                                            )
                                                        };
                                                        if ((*__slate_slot_1532 as u32) as i32)
                                                            != (0 as i32)
                                                            && *__slate_slot_1533 != (0 as i32)
                                                        {
                                                            std::ptr::write(
                                                                __slate_slot_1543,
                                                                unsafe {
                                                                    unsafe {
                                                                        (*(*__slate_slot_1475))
                                                                            .aFunc
                                                                    }
                                                                    .offset((0 as i32) as isize)
                                                                },
                                                            );
                                                            fixDistinctOpenEph(
                                                                pParse,
                                                                *__slate_slot_1533,
                                                                unsafe {
                                                                    (*(*__slate_slot_1543))
                                                                        .iDistinct
                                                                },
                                                                unsafe {
                                                                    (*(*__slate_slot_1543))
                                                                        .iDistAddr
                                                                },
                                                            );
                                                        }
                                                        // /* endif pGroupBy.  Begin aggregate queries without GROUP BY: */
                                                    }
                                                } else {
                                                    // /* Aggregate functions without GROUP BY. tag-select-0820 */
                                                    std::ptr::write(
                                                        __slate_slot_2418,
                                                        isSimpleCount(p, *__slate_slot_1475),
                                                    );
                                                    *__slate_slot_1544 = *__slate_slot_2418;
                                                    if *__slate_slot_2418
                                                        != std::ptr::null_mut::<Table>()
                                                    {
                                                        // /* tag-select-0821
                                                        //         **
                                                        //         ** If isSimpleCount() returns a pointer to a Table structure, then
                                                        //         ** the SQL statement is of the form:
                                                        //         **
                                                        //         **   SELECT count(*) FROM <tbl>
                                                        //         **
                                                        //         ** where the Table structure returned represents table <tbl>.
                                                        //         **
                                                        //         ** This statement is so common that it is optimized specially. The
                                                        //         ** OP_Count instruction is executed either on the intkey table that
                                                        //         ** contains the data for table <tbl> or on one of its indexes. It
                                                        //         ** is better to execute the op on an index, as indexes are almost
                                                        //         ** always spread across less pages than their corresponding tables.
                                                        //         */
                                                        std::ptr::write(
                                                            __slate_slot_1545,
                                                            unsafe {
                                                                sqlite3SchemaToIndex(
                                                                    unsafe { (*pParse).db },
                                                                    unsafe {
                                                                        (*(*__slate_slot_1544))
                                                                            .pSchema
                                                                    },
                                                                )
                                                            },
                                                        );
                                                        // /* Cursor to scan b-tree */
                                                        std::ptr::write(__slate_slot_2419, pParse);
                                                        std::ptr::write(
                                                            __slate_slot_2420,
                                                            unsafe { (*(*__slate_slot_2419)).nTab },
                                                        );
                                                        std::ptr::write(
                                                            __slate_slot_2421,
                                                            *__slate_slot_2420 + (1 as i32),
                                                        );
                                                        unsafe {
                                                            (*(*__slate_slot_2419)).nTab =
                                                                *__slate_slot_2421;
                                                        }
                                                        *__slate_slot_1546 = *__slate_slot_2420;
                                                        // /* Iterator variable */
                                                        // /* Keyinfo for scanned index */
                                                        std::ptr::write(
                                                            __slate_slot_1548,
                                                            std::ptr::null_mut::<KeyInfo>(),
                                                        );
                                                        // /* Best index found so far */
                                                        std::ptr::write(
                                                            __slate_slot_1549,
                                                            std::ptr::null_mut::<Index>(),
                                                        );
                                                        // /* Root page of scanned b-tree */
                                                        std::ptr::write(
                                                            __slate_slot_1550,
                                                            unsafe { (*(*__slate_slot_1544)).tnum },
                                                        );
                                                        unsafe {
                                                            sqlite3CodeVerifySchema(
                                                                pParse,
                                                                *__slate_slot_1545,
                                                            )
                                                        };
                                                        unsafe {
                                                            sqlite3TableLock(
                                                                pParse,
                                                                *__slate_slot_1545,
                                                                unsafe {
                                                                    (*(*__slate_slot_1544)).tnum
                                                                },
                                                                ((0 as i32) as i8) as u8,
                                                                (unsafe {
                                                                    (*(*__slate_slot_1544)).zName
                                                                })
                                                                    as *const i8,
                                                            )
                                                        };
                                                        // /* Search for the index that has the lowest scan cost.
                                                        //         **
                                                        //         ** (2011-04-15) Do not do a full scan of an unordered index.
                                                        //         **
                                                        //         ** (2013-10-03) Do not count the entries in a partial index.
                                                        //         **
                                                        //         ** In practice the KeyInfo structure will not be used. It is only
                                                        //         ** passed to keep OP_OpenRead happy.
                                                        //         */
                                                        if !((unsafe {
                                                            (*(*__slate_slot_1544)).tabFlags
                                                        }) & ((128 as i32) as u32)
                                                            == ((0 as i32) as u32))
                                                        {
                                                            *__slate_slot_1549 = unsafe {
                                                                sqlite3PrimaryKeyIndex(
                                                                    *__slate_slot_1544,
                                                                )
                                                            };
                                                        }
                                                        '__join_81: {
                                                            if !(((unsafe {
                                                                (*unsafe {
                                                                    unsafe {
                                                                        std::ptr::addr_of_mut!(
                                                                            (*unsafe { (*p).pSrc })
                                                                                .a
                                                                        )
                                                                            as *mut SrcItem
                                                                    }
                                                                    .offset((0 as i32) as isize)
                                                                })
                                                                .fg
                                                                .__slate_bits_0
                                                                .__get_notIndexed()
                                                            })
                                                                as i32)
                                                                != (0 as i32))
                                                            {
                                                                *__slate_slot_1547 = unsafe {
                                                                    (*(*__slate_slot_1544)).pIndex
                                                                };
                                                                loop {
                                                                    if *__slate_slot_1547
                                                                        != std::ptr::null_mut::<Index>(
                                                                        )
                                                                    {
                                                                        if ((unsafe { (*(*__slate_slot_1547)).__slate_bits_0.__get_bUnordered() }) as i32) == (0 as i32) && ((unsafe { (*(*__slate_slot_1547)).szIdxRow }) as i32) < ((unsafe { (*(*__slate_slot_1544)).szTabRow }) as i32) && (unsafe { (*(*__slate_slot_1547)).pPartIdxWhere }) == std::ptr::null_mut::<Expr>() && (!(*__slate_slot_1549 != std::ptr::null_mut::<Index>()) || ((unsafe { (*(*__slate_slot_1547)).szIdxRow }) as i32) < ((unsafe { (*(*__slate_slot_1549)).szIdxRow }) as i32)) {
*__slate_slot_1549 = *__slate_slot_1547;
}
                                                                        *__slate_slot_1547 = unsafe {
                                                                            (*(*__slate_slot_1547))
                                                                                .pNext
                                                                        };
                                                                    } else {
                                                                        break '__join_81;
                                                                    }
                                                                }
                                                            }
                                                        }
                                                        if *__slate_slot_1549
                                                            != std::ptr::null_mut::<Index>()
                                                        {
                                                            *__slate_slot_1550 = unsafe {
                                                                (*(*__slate_slot_1549)).tnum
                                                            };
                                                            *__slate_slot_1548 = unsafe {
                                                                sqlite3KeyInfoOfIndex(
                                                                    pParse,
                                                                    *__slate_slot_1549,
                                                                )
                                                            };
                                                        }
                                                        // /* Open a read-only cursor, execute the OP_Count, close the cursor. */
                                                        unsafe {
                                                            sqlite3VdbeAddOp4Int(
                                                                *__slate_slot_1468,
                                                                114 as i32,
                                                                *__slate_slot_1546,
                                                                *__slate_slot_1550 as i32,
                                                                *__slate_slot_1545,
                                                                1 as i32,
                                                            )
                                                        };
                                                        if *__slate_slot_1548
                                                            != std::ptr::null_mut::<KeyInfo>()
                                                        {
                                                            unsafe {
                                                                sqlite3VdbeChangeP4(
                                                                    *__slate_slot_1468,
                                                                    -(1 as i32),
                                                                    (*__slate_slot_1548 as *mut i8)
                                                                        as *const i8,
                                                                    -(9 as i32),
                                                                )
                                                            };
                                                        }
                                                        assignAggregateRegisters(
                                                            pParse,
                                                            *__slate_slot_1475,
                                                        );
                                                        unsafe {
                                                            sqlite3VdbeAddOp2(
                                                                *__slate_slot_1468,
                                                                100 as i32,
                                                                *__slate_slot_1546,
                                                                (unsafe {
                                                                    (*(*__slate_slot_1475))
                                                                        .iFirstReg
                                                                }) + unsafe {
                                                                    (*(*__slate_slot_1475)).nColumn
                                                                } + (0 as i32),
                                                            )
                                                        };
                                                        unsafe {
                                                            sqlite3VdbeAddOp1(
                                                                *__slate_slot_1468,
                                                                124 as i32,
                                                                *__slate_slot_1546,
                                                            )
                                                        };
                                                        explainSimpleCount(
                                                            pParse,
                                                            *__slate_slot_1544,
                                                            *__slate_slot_1549,
                                                        );
                                                    } else {
                                                        // /* The general case of an aggregate query without GROUP BY
                                                        //         ** tag-select-0822 */
                                                        // /* "populate accumulators" flag */
                                                        std::ptr::write(
                                                            __slate_slot_1551,
                                                            0 as i32,
                                                        );
                                                        std::ptr::write(
                                                            __slate_slot_1552,
                                                            std::ptr::null_mut::<ExprList>(),
                                                        );
                                                        std::ptr::write(
                                                            __slate_slot_1553,
                                                            ((0 as i32) as i16) as u16,
                                                        );
                                                        // /* If there are accumulator registers but no min() or max() functions
                                                        //         ** without FILTER clauses, allocate register regAcc. Register regAcc
                                                        //         ** will contain 0 the first time the inner loop runs, and 1 thereafter.
                                                        //         ** The code generated by updateAccumulator() uses this to ensure
                                                        //         ** that the accumulator registers are (a) updated only once if
                                                        //         ** there are no min() or max functions or (b) always updated for the
                                                        //         ** first row visited by the aggregate, so that they are updated at
                                                        //         ** least once even if the FILTER clause means the min() or max()
                                                        //         ** function visits zero rows.  */
                                                        if (unsafe {
                                                            (*(*__slate_slot_1475)).nAccumulator
                                                        }) != (0 as i32)
                                                        {
                                                            *__slate_slot_1465 = 0 as i32;
                                                            '__loop_101: loop {
                                                                if *__slate_slot_1465
                                                                    < unsafe {
                                                                        (*(*__slate_slot_1475))
                                                                            .nFunc
                                                                    }
                                                                {
                                                                    if (unsafe {
                                                                        (*unsafe { (*unsafe { unsafe { (*(*__slate_slot_1475)).aFunc }.offset(*__slate_slot_1465 as isize) }).pFExpr }).flags
                                                                    }) & ((16777216 as i32)
                                                                        as u32)
                                                                        != ((0 as i32) as u32)
                                                                    {
                                                                    } else {
                                                                        if (unsafe {
                                                                            (*unsafe { (*unsafe { unsafe { (*(*__slate_slot_1475)).aFunc }.offset(*__slate_slot_1465 as isize) }).pFunc }).funcFlags
                                                                        }) & ((32 as i32) as u32)
                                                                            != (0 as u32)
                                                                        {
                                                                            break '__loop_101;
                                                                        }
                                                                    }
                                                                    std::ptr::write(
                                                                        __slate_slot_2422,
                                                                        *__slate_slot_1465,
                                                                    );
                                                                    std::ptr::write(
                                                                        __slate_slot_2423,
                                                                        *__slate_slot_2422
                                                                            + (1 as i32),
                                                                    );
                                                                    *__slate_slot_1465 =
                                                                        *__slate_slot_2423;
                                                                } else {
                                                                    break;
                                                                }
                                                            }
                                                            if *__slate_slot_1465
                                                                == unsafe {
                                                                    (*(*__slate_slot_1475)).nFunc
                                                                }
                                                            {
                                                                std::ptr::write(
                                                                    __slate_slot_2424,
                                                                    pParse,
                                                                );
                                                                std::ptr::write(
                                                                    __slate_slot_2425,
                                                                    unsafe {
                                                                        (*(*__slate_slot_2424)).nMem
                                                                    },
                                                                );
                                                                std::ptr::write(
                                                                    __slate_slot_2426,
                                                                    *__slate_slot_2425 + (1 as i32),
                                                                );
                                                                unsafe {
                                                                    (*(*__slate_slot_2424)).nMem =
                                                                        *__slate_slot_2426;
                                                                }
                                                                *__slate_slot_1551 =
                                                                    *__slate_slot_2426;
                                                                unsafe {
                                                                    sqlite3VdbeAddOp2(
                                                                        *__slate_slot_1468,
                                                                        73 as i32,
                                                                        0 as i32,
                                                                        *__slate_slot_1551,
                                                                    )
                                                                };
                                                            }
                                                        } else {
                                                            if (unsafe {
                                                                (*(*__slate_slot_1475)).nFunc
                                                            }) == (1 as i32)
                                                                && (unsafe {
                                                                    (*unsafe {
                                                                        unsafe {
                                                                            (*(*__slate_slot_1475))
                                                                                .aFunc
                                                                        }
                                                                        .offset((0 as i32) as isize)
                                                                    })
                                                                    .iDistinct
                                                                }) >= (0 as i32)
                                                            {
                                                                0 as i32;
                                                                *__slate_slot_1552 = unsafe {
                                                                    (*unsafe { (*unsafe { unsafe { (*(*__slate_slot_1475)).aFunc }.offset((0 as i32) as isize) }).pFExpr }).x.pList
                                                                };
                                                                *__slate_slot_1553 =
                                                                    ((if *__slate_slot_1552
                                                                        != std::ptr::null_mut::<
                                                                            ExprList,
                                                                        >(
                                                                        )
                                                                    {
                                                                        (256 as i32) | (1024 as i32)
                                                                    } else {
                                                                        0 as i32
                                                                    })
                                                                        as i16)
                                                                        as u16;
                                                            }
                                                        }
                                                        assignAggregateRegisters(
                                                            pParse,
                                                            *__slate_slot_1475,
                                                        );
                                                        // /* This case runs if the aggregate has no GROUP BY clause.  The
                                                        //         ** processing is much simpler since there is only a single row
                                                        //         ** of output.
                                                        //         */
                                                        0 as i32;
                                                        resetAccumulator(
                                                            pParse,
                                                            *__slate_slot_1475,
                                                        );
                                                        // /* If this query is a candidate for the min/max optimization, then
                                                        //         ** minMaxFlag will have been previously set to either
                                                        //         ** WHERE_ORDERBY_MIN or WHERE_ORDERBY_MAX and pMinMaxOrderBy will
                                                        //         ** be an appropriate ORDER BY expression for the optimization.
                                                        //         */
                                                        0 as i32;
                                                        0 as i32;
                                                        {}
                                                        *__slate_slot_1467 = unsafe {
                                                            sqlite3WhereBegin(
                                                                pParse,
                                                                *__slate_slot_1471,
                                                                *__slate_slot_1472,
                                                                *__slate_slot_1481,
                                                                *__slate_slot_1552,
                                                                p,
                                                                ((((*__slate_slot_1482 as u32)
                                                                    as i32)
                                                                    | ((*__slate_slot_1553 as u32)
                                                                        as i32))
                                                                    as i16)
                                                                    as u16,
                                                                0 as i32,
                                                            )
                                                        };
                                                        if *__slate_slot_1467
                                                            == std::ptr::null_mut::<WhereInfo>()
                                                        {
                                                            break '__join_0;
                                                        } else {
                                                            {}
                                                            *__slate_slot_1554 = unsafe {
                                                                sqlite3WhereIsDistinct(
                                                                    *__slate_slot_1467,
                                                                )
                                                            };
                                                            updateAccumulator(
                                                                pParse,
                                                                *__slate_slot_1551,
                                                                *__slate_slot_1475,
                                                                *__slate_slot_1554,
                                                            );
                                                            if *__slate_slot_1554 != (0 as i32) {
                                                                std::ptr::write(
                                                                    __slate_slot_1555,
                                                                    unsafe {
                                                                        (*(*__slate_slot_1475))
                                                                            .aFunc
                                                                    },
                                                                );
                                                                if *__slate_slot_1555
                                                                    != std::ptr::null_mut::<
                                                                        AggInfo_func,
                                                                    >(
                                                                    )
                                                                {
                                                                    fixDistinctOpenEph(
                                                                        pParse,
                                                                        *__slate_slot_1554,
                                                                        unsafe {
                                                                            (*(*__slate_slot_1555))
                                                                                .iDistinct
                                                                        },
                                                                        unsafe {
                                                                            (*(*__slate_slot_1555))
                                                                                .iDistAddr
                                                                        },
                                                                    );
                                                                }
                                                            }
                                                            if *__slate_slot_1551 != (0 as i32) {
                                                                unsafe {
                                                                    sqlite3VdbeAddOp2(
                                                                        *__slate_slot_1468,
                                                                        73 as i32,
                                                                        1 as i32,
                                                                        *__slate_slot_1551,
                                                                    )
                                                                };
                                                            }
                                                            if *__slate_slot_1482 != (0 as u8) {
                                                                unsafe {
                                                                    sqlite3WhereMinMaxOptEarlyOut(
                                                                        *__slate_slot_1468,
                                                                        *__slate_slot_1467,
                                                                    )
                                                                };
                                                            }
                                                            {}
                                                            unsafe {
                                                                sqlite3WhereEnd(*__slate_slot_1467)
                                                            };
                                                            finalizeAggFunctions(
                                                                pParse,
                                                                *__slate_slot_1475,
                                                            );
                                                        }
                                                    }
                                                    (*__slate_slot_1478).pOrderBy =
                                                        std::ptr::null_mut::<ExprList>();
                                                    unsafe {
                                                        sqlite3ExprIfFalse(
                                                            pParse,
                                                            *__slate_slot_1474,
                                                            *__slate_slot_1516,
                                                            16 as i32,
                                                        )
                                                    };
                                                    selectInnerLoop(
                                                        pParse,
                                                        p,
                                                        -(1 as i32),
                                                        std::ptr::null_mut::<SortCtx>(),
                                                        std::ptr::null_mut::<DistinctCtx>(),
                                                        pDest,
                                                        *__slate_slot_1516,
                                                        *__slate_slot_1516,
                                                    );
                                                }
                                                unsafe {
                                                    sqlite3VdbeResolveLabel(
                                                        *__slate_slot_1468,
                                                        *__slate_slot_1516,
                                                    )
                                                };
                                                // /* endif aggregate query */
                                            }
                                        }
                                    }
                                    if (((*__slate_slot_1477).eTnctType as u32) as i32)
                                        == (3 as i32)
                                    {
                                        explainTempTable(
                                            pParse,
                                            (b"DISTINCT\0".as_ptr() as *mut i8) as *const i8,
                                        );
                                    }
                                    // /* If there is an ORDER BY clause, then we need to sort the results
                                    //   ** and send them to the callback one by one.  tag-select-0900
                                    //   */
                                    if (*__slate_slot_1478).pOrderBy
                                        != std::ptr::null_mut::<ExprList>()
                                    {
                                        0 as i32;
                                        generateSortTail(
                                            pParse,
                                            p,
                                            std::ptr::addr_of_mut!(*__slate_slot_1478),
                                            unsafe { (*(*__slate_slot_1470)).nExpr },
                                            pDest,
                                        );
                                    }
                                    // /* Jump here to skip this query
                                    //   */
                                    unsafe {
                                        sqlite3VdbeResolveLabel(
                                            *__slate_slot_1468,
                                            *__slate_slot_1479,
                                        )
                                    };
                                    // /* The SELECT has been coded. If there is an error in the Parse structure,
                                    //   ** set the return code to 1. Otherwise 0. */
                                    *__slate_slot_1476 =
                                        ((unsafe { (*pParse).nErr }) > (0 as i32)) as i32;
                                    // /* Control jumps to here if an error is encountered above, or upon
                                    //   ** successful coding of the SELECT.
                                    //   */
                                    break '__join_0;
                                }
                            }
                            unsafe {
                                sqlite3ErrorMsg(
                                    pParse,
                                    (b"expected %d columns for '%s' but got %d\0".as_ptr()
                                        as *mut i8)
                                        as *const i8,
                                    (unsafe { (*(*__slate_slot_1486)).nCol }) as i32,
                                    unsafe { (*(*__slate_slot_1486)).zName },
                                    unsafe { (*unsafe { (*(*__slate_slot_1485)).pEList }).nExpr },
                                )
                            };
                        }
                    }
                }
                0 as i32;
                0 as i32;
                unsafe { sqlite3ExprListDelete(*__slate_slot_1480, *__slate_slot_1481) };
                unsafe { sqlite3VdbeExplainPop(pParse) };
                return *__slate_slot_1476;
            }
        }
    }
    return unsafe { std::mem::zeroed() };
}

// /*
// ** Allocate a new Select structure and return a pointer to that
// ** structure.
// */
// /* The parser context */
// /* The SELECT statement being coded. */
// /* What to do with the query results */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3SelectNew(
    mut pParse: *mut Parse,
    mut pEList: *mut ExprList,
    mut pSrc: *mut SrcList,
    mut pWhere: *mut Expr,
    mut pGroupBy: *mut ExprList,
    mut pHaving: *mut Expr,
    mut pOrderBy: *mut ExprList,
    mut selFlags: u32,
    mut pLimit: *mut Expr,
) -> *mut Select {
    let mut pNew: *mut Select = unsafe { std::mem::zeroed() };
    let mut pAllocated: *mut Select = unsafe { std::mem::zeroed() };
    let mut standin: Select = unsafe { std::mem::zeroed() };
    let __v2427: *mut Select =
        (unsafe { sqlite3DbMallocRawNN(unsafe { (*pParse).db }, 120 as u64) }) as *mut Select;
    pNew = __v2427;
    pAllocated = __v2427;
    if pNew == std::ptr::null_mut::<Select>() {
        0 as i32;
        pNew = std::ptr::addr_of_mut!(standin);
    }
    if pEList == std::ptr::null_mut::<ExprList>() {
        pEList = unsafe {
            sqlite3ExprListAppend(pParse, std::ptr::null_mut::<ExprList>(), unsafe {
                sqlite3Expr(unsafe { (*pParse).db }, 180 as i32, std::ptr::null::<i8>())
            })
        };
    }
    unsafe {
        (*pNew).pEList = pEList;
    }
    unsafe {
        (*pNew).op = ((139 as i32) as i8) as u8;
    }
    unsafe {
        (*pNew).selFlags = selFlags;
    }
    unsafe {
        (*pNew).iLimit = 0 as i32;
    }
    unsafe {
        (*pNew).iOffset = 0 as i32;
    }
    let __v2428: *mut Parse = pParse;
    let __v2429: i32 = unsafe { (*__v2428).nSelect };
    let __v2430: i32 = __v2429 + (1 as i32);
    unsafe {
        (*__v2428).nSelect = __v2430;
    }
    unsafe {
        (*pNew).selId = __v2430 as u32;
    }
    unsafe {
        (*pNew).nSelectRow = (0 as i32) as i16;
    }
    if pSrc == std::ptr::null_mut::<SrcList>() {
        pSrc = (unsafe {
            sqlite3DbMallocZero(unsafe { (*pParse).db }, (8 as u64).wrapping_add(72 as u64))
        }) as *mut SrcList;
    }
    unsafe {
        (*pNew).pSrc = pSrc;
    }
    unsafe {
        (*pNew).pWhere = pWhere;
    }
    unsafe {
        (*pNew).pGroupBy = pGroupBy;
    }
    unsafe {
        (*pNew).pHaving = pHaving;
    }
    unsafe {
        (*pNew).pOrderBy = pOrderBy;
    }
    unsafe {
        (*pNew).pPrior = std::ptr::null_mut::<Select>();
    }
    unsafe {
        (*pNew).pNext = std::ptr::null_mut::<Select>();
    }
    unsafe {
        (*pNew).pLimit = pLimit;
    }
    unsafe {
        (*pNew).pWith = std::ptr::null_mut::<With>();
    }
    unsafe {
        (*pNew).pWin = std::ptr::null_mut::<Window>();
    }
    unsafe {
        (*pNew).pWinDefn = std::ptr::null_mut::<Window>();
    }
    if (unsafe { (*unsafe { (*pParse).db }).mallocFailed }) != (0 as u8) {
        clearSelect(
            unsafe { (*pParse).db },
            pNew,
            (pNew != std::ptr::addr_of_mut!(standin)) as i32,
        );
        pAllocated = std::ptr::null_mut::<Select>();
    } else {
        0 as i32;
    }
    return pAllocated;
}

// /* Parsing context */
// /* which columns to include in the result */
// /* the FROM clause -- which tables to scan */
// /* the WHERE clause */
// /* the GROUP BY clause */
// /* the HAVING clause */
// /* the ORDER BY clause */
// /* Flag parameters, such as SF_Distinct */
// /* LIMIT value.  NULL means not used */
// /*
// ** Delete the given Select structure and all of its substructures.
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3SelectDelete(mut db: *mut sqlite3, mut p: *mut Select) {
    if p != std::ptr::null_mut::<Select>() {
        clearSelect(db, p, 1 as i32);
    }
}

#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.select.sqlite3SelectDeleteGeneric")]
extern "C-unwind" fn sqlite3SelectDeleteGeneric(mut db: *mut sqlite3, mut p: *mut ()) {
    if p != std::ptr::null_mut::<()>() {
        clearSelect(db, p as *mut Select, 1 as i32);
    }
}

// /*
// ** Check all ON clauses in pSelect to verify that they do not reference
// ** columns to the right.
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3SelectCheckOnClauses(mut pParse: *mut Parse, mut pSelect: *mut Select) {
    let mut w: Walker = unsafe { std::mem::zeroed() };
    let mut sCtx: CheckOnCtx = unsafe { std::mem::zeroed() };
    let mut ii: i32 = 0 as i32;
    0 as i32;
    0 as i32;
    unsafe { memset(std::ptr::addr_of_mut!(w) as *mut (), 0 as i32, 48 as u64) };
    w.pParse = pParse;
    w.xExprCallback = Some(selectCheckOnClausesExpr);
    w.xSelectCallback = Some(selectCheckOnClausesSelect);
    unsafe {
        w.u.pCheckOnCtx = std::ptr::addr_of_mut!(sCtx);
    }
    unsafe { memset(std::ptr::addr_of_mut!(sCtx) as *mut (), 0 as i32, 24 as u64) };
    sCtx.pSrc = unsafe { (*pSelect).pSrc };
    unsafe { sqlite3WalkExpr(std::ptr::addr_of_mut!(w), unsafe { (*pSelect).pWhere }) };
    let __v2431: *mut Select = pSelect;
    let __v2432: u32 = unsafe { (*__v2431).selFlags };
    let __v2433: u32 = __v2432 & (!(1073741824 as i32) as u32);
    unsafe {
        (*__v2431).selFlags = __v2433;
    }
    // /* Check for any table-function args that are attached to virtual tables
    //   ** on the RHS of an outer join. They are subject to the same constraints
    //   ** as ON clauses. */
    sCtx.bFuncArg = 1 as i32;
    ii = 0 as i32;
    '__slate_break_2203: loop {
        if !(ii < unsafe { (*unsafe { (*pSelect).pSrc }).nSrc }) {
            break;
        }
        let mut pItem: *mut SrcItem = unsafe {
            unsafe { std::ptr::addr_of_mut!((*unsafe { (*pSelect).pSrc }).a) as *mut SrcItem }
                .offset(ii as isize)
        };
        if ((unsafe { (*pItem).fg.__slate_bits_0.__get_isTabFunc() }) as i32) != (0 as i32)
            && (((unsafe { (*pItem).fg.jointype }) as u32) as i32) & (32 as i32) != (0 as i32)
        {
            sCtx.iJoin = unsafe { (*pItem).iCursor };
            unsafe {
                sqlite3WalkExprList(std::ptr::addr_of_mut!(w), unsafe { (*pItem).u1.pFuncArg })
            };
        }
        let __v2434: i32 = ii;
        let __v2435: i32 = __v2434 + (1 as i32);
        ii = __v2435;
    }
}

// /*
// ** Get a VDBE for the given parser context.  Create a new one if necessary.
// ** If an error occurs, return NULL and leave a message in pParse.
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3GetVdbe(mut pParse: *mut Parse) -> *mut Vdbe {
    if (unsafe { (*pParse).pVdbe }) != std::ptr::null_mut::<Vdbe>() {
        return unsafe { (*pParse).pVdbe };
    }
    if (unsafe { (*pParse).pToplevel }) == std::ptr::null_mut::<Parse>() {
        unsafe {
            (*pParse)
                .__slate_bits_0
                .__set_okConstFactor((1 as i32) as u32);
        }
    }
    return unsafe { sqlite3VdbeCreate(pParse) };
}

// /*
// ** Given 1 to 3 identifiers preceding the JOIN keyword, determine the
// ** type of join.  Return an integer constant that expresses that type
// ** in terms of the following bit values:
// **
// **     JT_INNER
// **     JT_CROSS
// **     JT_OUTER
// **     JT_NATURAL
// **     JT_LEFT
// **     JT_RIGHT
// **
// ** A full outer join is the combination of JT_LEFT and JT_RIGHT.
// **
// ** If an illegal or unsupported join type is seen, then still return
// ** a join type, but put an error in the pParse structure.
// **
// ** These are the valid join types:
// **
// **
// **      pA       pB       pC               Return Value
// **     -------  -----    -----             ------------
// **     CROSS      -        -                 JT_CROSS
// **     INNER      -        -                 JT_INNER
// **     LEFT       -        -                 JT_LEFT|JT_OUTER
// **     LEFT     OUTER      -                 JT_LEFT|JT_OUTER
// **     RIGHT      -        -                 JT_RIGHT|JT_OUTER
// **     RIGHT    OUTER      -                 JT_RIGHT|JT_OUTER
// **     FULL       -        -                 JT_LEFT|JT_RIGHT|JT_OUTER
// **     FULL     OUTER      -                 JT_LEFT|JT_RIGHT|JT_OUTER
// **     NATURAL  INNER      -                 JT_NATURAL|JT_INNER
// **     NATURAL  LEFT       -                 JT_NATURAL|JT_LEFT|JT_OUTER
// **     NATURAL  LEFT     OUTER               JT_NATURAL|JT_LEFT|JT_OUTER
// **     NATURAL  RIGHT      -                 JT_NATURAL|JT_RIGHT|JT_OUTER
// **     NATURAL  RIGHT    OUTER               JT_NATURAL|JT_RIGHT|JT_OUTER
// **     NATURAL  FULL       -                 JT_NATURAL|JT_LEFT|JT_RIGHT
// **     NATURAL  FULL     OUTER               JT_NATRUAL|JT_LEFT|JT_RIGHT
// **
// ** To preserve historical compatibly, SQLite also accepts a variety
// ** of other non-standard and in many cases nonsensical join types.
// ** This routine makes as much sense at it can from the nonsense join
// ** type and returns a result.  Examples of accepted nonsense join types
// ** include but are not limited to:
// **
// **          INNER CROSS JOIN        ->   same as JOIN
// **          NATURAL CROSS JOIN      ->   same as NATURAL JOIN
// **          OUTER LEFT JOIN         ->   same as LEFT JOIN
// **          LEFT NATURAL JOIN       ->   same as NATURAL LEFT JOIN
// **          LEFT RIGHT JOIN         ->   same as FULL JOIN
// **          RIGHT OUTER FULL JOIN   ->   same as FULL JOIN
// **          CROSS CROSS CROSS JOIN  ->   same as JOIN
// **
// ** The only restrictions on the join type name are:
// **
// **    *   "INNER" cannot appear together with "OUTER", "LEFT", "RIGHT",
// **        or "FULL".
// **
// **    *   "CROSS" cannot appear together with "OUTER", "LEFT", "RIGHT,
// **        or "FULL".
// **
// **    *   If "OUTER" is present then there must also be one of
// **        "LEFT", "RIGHT", or "FULL"
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3JoinType(
    mut pParse: *mut Parse,
    mut pA: *mut Token,
    mut pB: *mut Token,
    mut pC: *mut Token,
) -> i32 {
    let mut jointype: i32 = 0 as i32;
    let mut apAll: __SlateAlign16<[*mut Token; 3]> = __SlateAlign16([0 as *mut Token; 3]);
    let mut p: *mut Token = unsafe { std::mem::zeroed() };
    // /*   0123456789 123456789 123456789 123 */
    // /* (0) natural */
    // /* (1) left    */
    // /* (2) outer   */
    // /* (3) right   */
    // /* (4) full    */
    // /* (5) inner   */
    // /* (6) cross   */
    let mut i: i32 = 0 as i32;
    let mut j: i32 = 0 as i32;
    unsafe {
        *unsafe { (apAll.0.as_mut_ptr() as *mut *mut Token).offset((0 as i32) as isize) } = pA;
    }
    unsafe {
        *unsafe { (apAll.0.as_mut_ptr() as *mut *mut Token).offset((1 as i32) as isize) } = pB;
    }
    unsafe {
        *unsafe { (apAll.0.as_mut_ptr() as *mut *mut Token).offset((2 as i32) as isize) } = pC;
    }
    i = 0 as i32;
    '__slate_break_1999: loop {
        if !(i < (3 as i32)
            && (unsafe {
                *unsafe { (apAll.0.as_mut_ptr() as *mut *mut Token).offset(i as isize) }
            }) != std::ptr::null_mut::<Token>())
        {
            break;
        }
        p = unsafe { *unsafe { (apAll.0.as_mut_ptr() as *mut *mut Token).offset(i as isize) } };
        j = 0 as i32;
        '__slate_break_2000: loop {
            if !(j < ((((21 as u64) / (3 as u64)) as u32) as i32)) {
                break;
            }
            let __v2440: bool;
            if (unsafe { (*p).n })
                == ((((unsafe {
                    (*unsafe {
                        unsafe { std::ptr::addr_of!(aKeyword.0) as *const __SlateRecord212 }
                            .offset(j as isize)
                    })
                    .nChar
                }) as u32) as i32) as u32)
            {
                __v2440 = (unsafe {
                    sqlite3_strnicmp(
                        ((unsafe { (*p).z }) as *mut i8) as *const i8,
                        unsafe {
                            unsafe { std::ptr::addr_of!(zKeyText.0) as *const i8 }.offset(
                                (((unsafe {
                                    (*unsafe {
                                        unsafe {
                                            std::ptr::addr_of!(aKeyword.0)
                                                as *const __SlateRecord212
                                        }
                                        .offset(j as isize)
                                    })
                                    .i
                                }) as u32) as i32) as isize,
                            )
                        },
                        (unsafe { (*p).n }) as i32,
                    )
                }) == (0 as i32);
            } else {
                __v2440 = false as bool;
            }
            if __v2440 {
                let __v2441: i32 = jointype;
                let __v2442: i32 = __v2441
                    | (((unsafe {
                        (*unsafe {
                            unsafe { std::ptr::addr_of!(aKeyword.0) as *const __SlateRecord212 }
                                .offset(j as isize)
                        })
                        .code
                    }) as u32) as i32);
                jointype = __v2442;
                break '__slate_break_2000;
            }
            let __v2438: i32 = j;
            let __v2439: i32 = __v2438 + (1 as i32);
            j = __v2439;
        }
        {}
        if j >= ((((21 as u64) / (3 as u64)) as u32) as i32) {
            let __v2443: i32 = jointype;
            let __v2444: i32 = __v2443 | (128 as i32);
            jointype = __v2444;
            break '__slate_break_1999;
        }
        let __v2436: i32 = i;
        let __v2437: i32 = __v2436 + (1 as i32);
        i = __v2437;
    }
    if jointype & ((1 as i32) | (32 as i32)) == (1 as i32) | (32 as i32)
        || jointype & (128 as i32) != (0 as i32)
        || jointype & ((32 as i32) | (8 as i32) | (16 as i32)) == (32 as i32)
    {
        let mut zSp1: *const i8 = (b" \0".as_ptr() as *mut i8) as *const i8;
        let mut zSp2: *const i8 = (b" \0".as_ptr() as *mut i8) as *const i8;
        if pB == std::ptr::null_mut::<Token>() {
            let __v2445: *const i8 = zSp1;
            let __v2446: *const i8 = unsafe { __v2445.offset((1 as i32) as isize) };
            zSp1 = __v2446;
        }
        if pC == std::ptr::null_mut::<Token>() {
            let __v2447: *const i8 = zSp2;
            let __v2448: *const i8 = unsafe { __v2447.offset((1 as i32) as isize) };
            zSp2 = __v2448;
        }
        unsafe {
            sqlite3ErrorMsg(
                pParse,
                (b"unknown join type: %T%s%T%s%T\0".as_ptr() as *mut i8) as *const i8,
                pA,
                zSp1,
                pB,
                zSp2,
                pC,
            )
        };
        jointype = 1 as i32;
    }
    return jointype;
}

// /*
// ** Return the index of a column in a table.  Return -1 if the column
// ** is not contained in the table.
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3ColumnIndex(mut pTab: *mut Table, mut zCol: *const i8) -> i32 {
    let mut i: i32 = 0 as i32;
    let mut h: u8 = 0 as u8;
    let mut aCol: *const Column = unsafe { std::mem::zeroed() };
    let mut nCol: i32 = 0 as i32;
    h = unsafe { sqlite3StrIHash(zCol) };
    aCol = (unsafe { (*pTab).aCol }) as *const Column;
    nCol = (unsafe { (*pTab).nCol }) as i32;
    // /* See if the aHx gives us a lucky match */
    i = ((unsafe {
        *unsafe {
            unsafe { (*pTab).aHx.as_mut_ptr() as *mut u8 }
                .offset((((((h as u32) as i32) as i64) as u64) % (16 as u64)) as isize)
        }
    }) as u32) as i32;
    0 as i32;
    let __v2449: bool;
    if (((unsafe { (*unsafe { aCol.offset(i as isize) }).hName }) as u32) as i32)
        == ((h as u32) as i32)
    {
        __v2449 = (unsafe {
            sqlite3StrICmp(
                (unsafe { (*unsafe { aCol.offset(i as isize) }).zCnName }) as *const i8,
                zCol,
            )
        }) == (0 as i32);
    } else {
        __v2449 = false as bool;
    }
    if __v2449 {
        return i;
    }
    // /* No lucky match from the hash table.  Do a full search. */
    i = 0 as i32;
    // /*exit-by-break*/
    '__slate_break_2004: while (1 as i32) != (0 as i32) {
        let __v2450: bool;
        if (((unsafe { (*unsafe { aCol.offset(i as isize) }).hName }) as u32) as i32)
            == ((h as u32) as i32)
        {
            __v2450 = (unsafe {
                sqlite3StrICmp(
                    (unsafe { (*unsafe { aCol.offset(i as isize) }).zCnName }) as *const i8,
                    zCol,
                )
            }) == (0 as i32);
        } else {
            __v2450 = false as bool;
        }
        if __v2450 {
            return i;
        }
        let __v2451: i32 = i;
        let __v2452: i32 = __v2451 + (1 as i32);
        i = __v2452;
        if i >= nCol {
            break '__slate_break_2004;
        }
    }
    return -(1 as i32);
}

// /*
// ** Mark a subquery result column as having been used.
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3SrcItemColumnUsed(mut pItem: *mut SrcItem, mut iCol: i32) {
    0 as i32;
    0 as i32;
    if ((unsafe { (*pItem).fg.__slate_bits_0.__get_isNestedFrom() }) as i32) != (0 as i32) {
        let mut pResults: *mut ExprList = unsafe { std::mem::zeroed() };
        0 as i32;
        0 as i32;
        0 as i32;
        pResults = unsafe { (*unsafe { (*unsafe { (*pItem).u4.pSubq }).pSelect }).pEList };
        0 as i32;
        0 as i32;
        unsafe {
            (*unsafe {
                unsafe { std::ptr::addr_of_mut!((*pResults).a) as *mut ExprList_item }
                    .offset(iCol as isize)
            })
            .fg
            .__slate_bits_0
            .__set_bUsed((1 as i32) as u32);
        }
    }
}

// /* Array of tables to search */
// /* First member of pSrc->a[] to check */
// /* Last member of pSrc->a[] to check */
// /* Name of the column we are looking for */
// /* Write index of pSrc->a[] here */
// /* Write index of pSrc->a[*piTab].pSTab->aCol[] here */
// /* Ignore hidden columns */
// /*
// ** Set the EP_OuterON property on all terms of the given expression.
// ** And set the Expr.w.iJoin to iTable for every term in the
// ** expression.
// **
// ** The EP_OuterON property is used on terms of an expression to tell
// ** the OUTER JOIN processing logic that this term is part of the
// ** join restriction specified in the ON or USING clause and not a part
// ** of the more general WHERE clause.  These terms are moved over to the
// ** WHERE clause during join processing but we need to remember that they
// ** originated in the ON or USING clause.
// **
// ** The Expr.w.iJoin tells the WHERE clause processing that the
// ** expression depends on table w.iJoin even if that table is not
// ** explicitly mentioned in the expression.  That information is needed
// ** for cases like this:
// **
// **    SELECT * FROM t1 LEFT JOIN t2 ON t1.a=t2.b AND t1.x=5
// **
// ** The where clause needs to defer the handling of the t1.x=5
// ** term until after the t2 loop of the join.  In that way, a
// ** NULL t2 row will be inserted whenever t1.x!=5.  If we do not
// ** defer the handling of t1.x=5, it will be processed immediately
// ** after the t1 loop and rows with t1.x!=5 will never appear in
// ** the output, which is incorrect.
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3SetJoinExpr(mut p: *mut Expr, mut iTable: i32, mut joinFlag: u32) {
    0 as i32;
    '__slate_break_2006: while p != std::ptr::null_mut::<Expr>() {
        let __v2453: *mut Expr = p;
        let __v2454: u32 = unsafe { (*__v2453).flags };
        let __v2455: u32 = __v2454 | joinFlag;
        unsafe {
            (*__v2453).flags = __v2455;
        }
        0 as i32;
        {}
        unsafe {
            (*p).w.iJoin = iTable;
        }
        if (unsafe { (*p).flags }) & ((4096 as i32) as u32) == ((0 as i32) as u32) {
            if (unsafe { (*p).x.pList }) != std::ptr::null_mut::<ExprList>() {
                let mut i: i32 = 0 as i32;
                i = 0 as i32;
                '__slate_break_2007: loop {
                    if !(i < unsafe { (*unsafe { (*p).x.pList }).nExpr }) {
                        break;
                    }
                    sqlite3SetJoinExpr(
                        unsafe {
                            (*unsafe {
                                unsafe {
                                    std::ptr::addr_of_mut!((*unsafe { (*p).x.pList }).a)
                                        as *mut ExprList_item
                                }
                                .offset(i as isize)
                            })
                            .pExpr
                        },
                        iTable,
                        joinFlag,
                    );
                    let __v2456: i32 = i;
                    let __v2457: i32 = __v2456 + (1 as i32);
                    i = __v2457;
                }
            }
        }
        sqlite3SetJoinExpr(unsafe { (*p).pLeft }, iTable, joinFlag);
        p = unsafe { (*p).pRight };
    }
}

// /*
// ** This routine sets up a SELECT statement for processing.  The
// ** following is accomplished:
// **
// **     *  VDBE Cursor numbers are assigned to all FROM-clause terms.
// **     *  Ephemeral Table objects are created for all FROM-clause subqueries.
// **     *  ON and USING clauses are shifted into WHERE statements
// **     *  Wildcards "*" and "TABLE.*" in result sets are expanded.
// **     *  Identifiers in expression are matched to tables.
// **
// ** This routine acts recursively on all subqueries within the SELECT.
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3SelectPrep(
    mut pParse: *mut Parse,
    mut p: *mut Select,
    mut pOuterNC: *mut NameContext,
) {
    0 as i32;
    0 as i32;
    if (unsafe { (*unsafe { (*pParse).db }).mallocFailed }) != (0 as u8) {
        return;
    }
    if (unsafe { (*p).selFlags }) & ((128 as i32) as u32) != (0 as u32) {
        return;
    }
    sqlite3SelectExpand(pParse, p);
    if (unsafe { (*pParse).nErr }) != (0 as i32) {
        return;
    }
    unsafe { sqlite3ResolveSelectNames(pParse, p, pOuterNC) };
    if (unsafe { (*pParse).nErr }) != (0 as i32) {
        return;
    }
    sqlite3SelectAddTypeInfo(pParse, p);
}

// /*
// ** The SrcItem structure passed as the second argument represents a
// ** sub-query in the FROM clause of a SELECT statement. This function
// ** allocates and populates the SrcItem.pTab object. If successful,
// ** SQLITE_OK is returned. Otherwise, if an OOM error is encountered,
// ** SQLITE_NOMEM.
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3ExpandSubquery(mut pParse: *mut Parse, mut pFrom: *mut SrcItem) -> i32 {
    let mut pSel: *mut Select = unsafe { std::mem::zeroed() };
    let mut pTab: *mut Table = unsafe { std::mem::zeroed() };
    0 as i32;
    0 as i32;
    pSel = unsafe { (*unsafe { (*pFrom).u4.pSubq }).pSelect };
    0 as i32;
    let __v2458: *mut Table =
        (unsafe { sqlite3DbMallocZero(unsafe { (*pParse).db }, 120 as u64) }) as *mut Table;
    pTab = __v2458;
    unsafe {
        (*pFrom).pSTab = __v2458;
    }
    if pTab == std::ptr::null_mut::<Table>() {
        return 7 as i32;
    }
    unsafe {
        (*pTab).nTabRef = (1 as i32) as u32;
    }
    if (unsafe { (*pFrom).zAlias }) != std::ptr::null_mut::<i8>() {
        unsafe {
            (*pTab).zName = unsafe {
                sqlite3DbStrDup(
                    unsafe { (*pParse).db },
                    (unsafe { (*pFrom).zAlias }) as *const i8,
                )
            };
        }
    } else {
        unsafe {
            (*pTab).zName = unsafe {
                sqlite3MPrintf(
                    unsafe { (*pParse).db },
                    (b"%!S\0".as_ptr() as *mut i8) as *const i8,
                    pFrom,
                )
            };
        }
    }
    '__slate_break_2154: while (unsafe { (*pSel).pPrior }) != std::ptr::null_mut::<Select>() {
        pSel = unsafe { (*pSel).pPrior };
    }
    sqlite3ColumnsFromExprList(
        pParse,
        unsafe { (*pSel).pEList },
        unsafe { std::ptr::addr_of_mut!((*pTab).nCol) },
        unsafe { std::ptr::addr_of_mut!((*pTab).aCol) },
    );
    unsafe {
        (*pTab).iPKey = -(1 as i32) as i16;
    }
    unsafe {
        (*pTab).eTabType = ((2 as i32) as i8) as u8;
    }
    unsafe {
        (*pTab).nRowLogEst = (200 as i32) as i16;
    }
    0 as i32;
    // /* The usual case - do not allow ROWID on a subquery */
    let __v2459: *mut Table = pTab;
    let __v2460: u32 = unsafe { (*__v2459).tabFlags };
    let __v2461: u32 = __v2460 | (((16384 as i32) | (512 as i32)) as u32);
    unsafe {
        (*__v2459).tabFlags = __v2461;
    }
    return if (unsafe { (*pParse).nErr }) != (0 as i32) {
        1 as i32
    } else {
        0 as i32
    };
}

// /* Parsing context */
// /* The right-most of SELECTs to be coded */
// /* What to do with query results */
// /* SQLITE_OMIT_COMPOUND_SELECT */
// /*
// ** Error message for when two or more terms of a compound select have different
// ** size result sets.
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3SelectWrongNumTermsError(mut pParse: *mut Parse, mut p: *mut Select) {
    if (unsafe { (*p).selFlags }) & ((512 as i32) as u32) != (0 as u32) {
        unsafe {
            sqlite3ErrorMsg(
                pParse,
                (b"all VALUES must have the same number of terms\0".as_ptr() as *mut i8)
                    as *const i8,
            )
        };
    } else {
        unsafe {
            sqlite3ErrorMsg(pParse, (b"SELECTs to the left and right of %s do not have the same number of result columns\0".as_ptr() as *mut i8) as *const i8, sqlite3SelectOpName(((unsafe { (*p).op }) as u32) as i32))
        };
    }
}

// /* The parser context */
// /* The complete select statement being coded */
// /* Pull data from this table if non-negative */
// /* If not NULL, info on how to process ORDER BY */
// /* If not NULL, info on how to process DISTINCT */
// /* How to dispose of the results */
// /* Jump here to continue with next row */
// /* Jump here to break out of the inner loop */
// /*
// ** Allocate a KeyInfo object sufficient for an index of N key columns and
// ** X extra columns.
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3KeyInfoAlloc(
    mut db: *mut sqlite3,
    mut N: i32,
    mut X: i32,
) -> *mut KeyInfo {
    let mut nExtra: i32 = ((((N + X) as i64) as u64)
        .wrapping_mul((8 as u64).wrapping_add(((1 as i32) as i64) as u64))
        as u32) as i32;
    let mut p: *mut KeyInfo = unsafe { std::mem::zeroed() };
    0 as i32;
    if N + X > (65535 as i32) {
        return (unsafe { sqlite3OomFault(db) }) as *mut KeyInfo;
    }
    p = (unsafe {
        sqlite3DbMallocRawNN(
            db,
            (32 as u64)
                .wrapping_add((((0 as i32) as i64) as u64).wrapping_mul(8 as u64))
                .wrapping_add((nExtra as i64) as u64),
        )
    }) as *mut KeyInfo;
    if p != std::ptr::null_mut::<KeyInfo>() {
        unsafe {
            (*p).aSortFlags = (unsafe {
                unsafe { std::ptr::addr_of_mut!((*p).aColl) as *mut *mut CollSeq }
                    .offset((N + X) as isize)
            }) as *mut u8;
        }
        unsafe {
            (*p).nKeyField = (N as i16) as u16;
        }
        unsafe {
            (*p).nAllField = ((N + X) as i16) as u16;
        }
        unsafe {
            (*p).enc = unsafe { (*db).enc };
        }
        unsafe {
            (*p).db = db;
        }
        unsafe {
            (*p).nRef = (1 as i32) as u32;
        }
        unsafe {
            memset(
                (unsafe { std::ptr::addr_of_mut!((*p).aColl) as *mut *mut CollSeq }) as *mut (),
                0 as i32,
                (nExtra as i64) as u64,
            )
        };
    } else {
        return (unsafe { sqlite3OomFault(db) }) as *mut KeyInfo;
    }
    return p;
}

// /*
// ** Deallocate a KeyInfo object
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3KeyInfoUnref(mut p: *mut KeyInfo) {
    if p != std::ptr::null_mut::<KeyInfo>() {
        0 as i32;
        0 as i32;
        let __v2462: *mut KeyInfo = p;
        let __v2463: u32 = unsafe { (*__v2462).nRef };
        let __v2464: u32 = __v2463.wrapping_sub((1 as i32) as u32);
        unsafe {
            (*__v2462).nRef = __v2464;
        }
        if (unsafe { (*p).nRef }) == ((0 as i32) as u32) {
            unsafe { sqlite3DbNNFreeNN(unsafe { (*p).db }, p as *mut ()) };
        }
    }
}

// /*
// ** Make a new pointer to a KeyInfo object
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3KeyInfoRef(mut p: *mut KeyInfo) -> *mut KeyInfo {
    if p != std::ptr::null_mut::<KeyInfo>() {
        0 as i32;
        let __v2465: *mut KeyInfo = p;
        let __v2466: u32 = unsafe { (*__v2465).nRef };
        let __v2467: u32 = __v2466.wrapping_add((1 as i32) as u32);
        unsafe {
            (*__v2465).nRef = __v2467;
        }
    }
    return p;
}

// /*
// ** Given an expression list, generate a KeyInfo structure that records
// ** the collating sequence for each expression in that expression list.
// **
// ** If the ExprList is an ORDER BY or GROUP BY clause then the resulting
// ** KeyInfo structure is appropriate for initializing a virtual index to
// ** implement that clause.  If the ExprList is the result set of a SELECT
// ** then the KeyInfo structure is appropriate for initializing a virtual
// ** index to implement a DISTINCT test.
// **
// ** Space to hold the KeyInfo structure is obtained from malloc.  The calling
// ** function is responsible for seeing that this structure is eventually
// ** freed.
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3KeyInfoFromExprList(
    mut pParse: *mut Parse,
    mut pList: *mut ExprList,
    mut iStart: i32,
    mut nExtra: i32,
) -> *mut KeyInfo {
    let mut nExpr: i32 = 0 as i32;
    let mut pInfo: *mut KeyInfo = unsafe { std::mem::zeroed() };
    let mut pItem: *mut ExprList_item = unsafe { std::mem::zeroed() };
    let mut db: *mut sqlite3 = unsafe { (*pParse).db };
    let mut i: i32 = 0 as i32;
    nExpr = unsafe { (*pList).nExpr };
    pInfo = sqlite3KeyInfoAlloc(db, nExpr - iStart, nExtra + (1 as i32));
    if pInfo != std::ptr::null_mut::<KeyInfo>() {
        0 as i32;
        i = iStart;
        let __v2468: *mut ExprList_item = unsafe {
            unsafe { std::ptr::addr_of_mut!((*pList).a) as *mut ExprList_item }
                .offset(iStart as isize)
        };
        pItem = __v2468;
        '__slate_break_2026: while i < nExpr {
            unsafe {
                *unsafe {
                    unsafe { std::ptr::addr_of_mut!((*pInfo).aColl) as *mut *mut CollSeq }
                        .offset((i - iStart) as isize)
                } = unsafe {
                    sqlite3ExprNNCollSeq(pParse, (unsafe { (*pItem).pExpr }) as *const Expr)
                };
            }
            unsafe {
                *unsafe { unsafe { (*pInfo).aSortFlags }.offset((i - iStart) as isize) } =
                    unsafe { (*pItem).fg.sortFlags };
            }
            let __v2469: i32 = i;
            let __v2470: i32 = __v2469 + (1 as i32);
            i = __v2470;
            let __v2471: *mut ExprList_item = pItem;
            let __v2472: *mut ExprList_item = unsafe { __v2471.offset((1 as i32) as isize) };
            pItem = __v2472;
        }
    }
    return pInfo;
}

// /* Parsing context */
// /* Form the KeyInfo object from this ExprList */
// /* Begin with this column of pList */
// /* Add this many extra columns to the end */
// /*
// ** Name of the connection operator, used for error messages.
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3SelectOpName(mut id: i32) -> *const i8 {
    let mut z: *mut i8 = unsafe { std::mem::zeroed() };
    '__slate_break_2027: {
        match id {
            136 => {
                z = b"UNION ALL\0".as_ptr() as *mut i8;
            }
            138 => {
                z = b"INTERSECT\0".as_ptr() as *mut i8;
            }
            137 => {
                z = b"EXCEPT\0".as_ptr() as *mut i8;
            }
            _ => {
                z = b"UNION\0".as_ptr() as *mut i8;
            }
        }
    }
    return z as *const i8;
}

// /*
// ** Initialize a SelectDest structure.
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3SelectDestInit(
    mut pDest: *mut SelectDest,
    mut eDest: i32,
    mut iParm: i32,
) {
    unsafe {
        (*pDest).eDest = (eDest as i8) as u8;
    }
    unsafe {
        (*pDest).iSDParm = iParm;
    }
    unsafe {
        (*pDest).iSDParm2 = 0 as i32;
    }
    unsafe {
        (*pDest).zAffSdst = std::ptr::null_mut::<i8>();
    }
    unsafe {
        (*pDest).iSdst = 0 as i32;
    }
    unsafe {
        (*pDest).nSdst = 0 as i32;
    }
}

// /* Current innermost WITH clause */
// /* FROM clause element to resolve */
// /* OUT: WITH clause return value belongs to */
// /* The code generator maintains a stack of active WITH clauses
// ** with the inner-most WITH clause being at the top of the stack.
// **
// ** This routine pushes the WITH clause passed as the second argument
// ** onto the top of the stack. If argument bFree is true, then this
// ** WITH clause will never be popped from the stack but should instead
// ** be freed along with the Parse object. In other cases, when
// ** bFree==0, the With object will be freed along with the SELECT
// ** statement with which it is associated.
// **
// ** This routine returns a copy of pWith.  Or, if bFree is true and
// ** the pWith object is destroyed immediately due to an OOM condition,
// ** then this routine return NULL.
// **
// ** If bFree is true, do not continue to use the pWith pointer after
// ** calling this routine,  Instead, use only the return value.
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3WithPush(
    mut pParse: *mut Parse,
    mut pWith: *mut With,
    mut bFree: u8,
) -> *mut With {
    if pWith != std::ptr::null_mut::<With>() {
        if bFree != (0 as u8) {
            pWith = (unsafe {
                sqlite3ParserAddCleanup(
                    pParse,
                    unsafe {
                        std::mem::transmute::<
                            *const (),
                            Option<unsafe extern "C-unwind" fn(*mut sqlite3, *mut ())>,
                        >(sqlite3WithDeleteGeneric as *const ())
                    },
                    pWith as *mut (),
                )
            }) as *mut With;
            if pWith == std::ptr::null_mut::<With>() {
                return std::ptr::null_mut::<With>();
            }
        }
        if (unsafe { (*pParse).nErr }) == (0 as i32) {
            0 as i32;
            unsafe {
                (*pWith).pOuter = unsafe { (*pParse).pWith };
            }
            unsafe {
                (*pParse).pWith = pWith;
            }
        }
    }
    return pWith;
}

// /* Deferred row loading info or NULL */
// /* Use SorterOpen instead of OpenEphemeral */
// /*
// ** Delete all the content of a Select structure.  Deallocate the structure
// ** itself depending on the value of bFree
// **
// ** If bFree==1, call sqlite3DbFree() on the p object.
// ** If bFree==0, Leave the first Select object unfreed
// */
fn clearSelect(mut db: *mut sqlite3, mut p: *mut Select, mut bFree: i32) {
    0 as i32;
    '__slate_break_1996: while p != std::ptr::null_mut::<Select>() {
        let mut pPrior: *mut Select = unsafe { (*p).pPrior };
        unsafe { sqlite3ExprListDelete(db, unsafe { (*p).pEList }) };
        unsafe { sqlite3SrcListDelete(db, unsafe { (*p).pSrc }) };
        unsafe { sqlite3ExprDelete(db, unsafe { (*p).pWhere }) };
        unsafe { sqlite3ExprListDelete(db, unsafe { (*p).pGroupBy }) };
        unsafe { sqlite3ExprDelete(db, unsafe { (*p).pHaving }) };
        unsafe { sqlite3ExprListDelete(db, unsafe { (*p).pOrderBy }) };
        unsafe { sqlite3ExprDelete(db, unsafe { (*p).pLimit }) };
        if (unsafe { (*p).pWith }) != std::ptr::null_mut::<With>() {
            unsafe { sqlite3WithDelete(db, unsafe { (*p).pWith }) };
        }
        if (unsafe { (*p).pWinDefn }) != std::ptr::null_mut::<Window>() {
            unsafe { sqlite3WindowListDelete(db, unsafe { (*p).pWinDefn }) };
        }
        '__slate_break_1997: while (unsafe { (*p).pWin }) != std::ptr::null_mut::<Window>() {
            0 as i32;
            unsafe { sqlite3WindowUnlinkFromSelect(unsafe { (*p).pWin }) };
        }
        if bFree != (0 as i32) {
            unsafe { sqlite3DbNNFreeNN(db, p as *mut ()) };
        }
        p = pPrior;
        bFree = 1 as i32;
    }
}

// /*
// ** Return a pointer to the right-most SELECT statement in a compound.
// */
fn findRightmost(mut p: *mut Select) -> *mut Select {
    '__slate_break_1998: while (unsafe { (*p).pNext }) != std::ptr::null_mut::<Select>() {
        p = unsafe { (*p).pNext };
    }
    return p;
}

// /*
// ** Search the tables iStart..iEnd (inclusive) in pSrc, looking for a
// ** table that has a column named zCol.  The search is left-to-right.
// ** The first match found is returned.
// **
// ** When found, set *piTab and *piCol to the table index and column index
// ** of the matching column and return TRUE.
// **
// ** If not found, return FALSE.
// */
fn tableAndColumnIndex(
    mut pSrc: *mut SrcList,
    mut iStart: i32,
    mut iEnd: i32,
    mut zCol: *const i8,
    mut piTab: *mut i32,
    mut piCol: *mut i32,
    mut bIgnoreHidden: i32,
) -> i32 {
    // /* For looping over tables in pSrc */
    let mut i: i32 = 0 as i32;
    // /* Index of column matching zCol */
    let mut iCol: i32 = 0 as i32;
    0 as i32;
    0 as i32;
    // /* Both or neither are NULL */
    0 as i32;
    i = iStart;
    '__slate_break_2005: loop {
        if !(i <= iEnd) {
            break;
        }
        iCol = sqlite3ColumnIndex(
            unsafe {
                (*unsafe {
                    unsafe { std::ptr::addr_of_mut!((*pSrc).a) as *mut SrcItem }.offset(i as isize)
                })
                .pSTab
            },
            zCol,
        );
        if iCol >= (0 as i32)
            && (bIgnoreHidden == (0 as i32)
                || (((((unsafe {
                    (*unsafe {
                        unsafe {
                            (*unsafe {
                                (*unsafe {
                                    unsafe { std::ptr::addr_of_mut!((*pSrc).a) as *mut SrcItem }
                                        .offset(i as isize)
                                })
                                .pSTab
                            })
                            .aCol
                        }
                        .offset(iCol as isize)
                    })
                    .colFlags
                }) as u32) as i32)
                    & (2 as i32)
                    != (0 as i32)) as i32)
                    == (0 as i32))
        {
            if piTab != std::ptr::null_mut::<i32>() {
                sqlite3SrcItemColumnUsed(
                    unsafe {
                        unsafe { std::ptr::addr_of_mut!((*pSrc).a) as *mut SrcItem }
                            .offset(i as isize)
                    },
                    iCol,
                );
                unsafe {
                    *piTab = i;
                }
                unsafe {
                    *piCol = iCol;
                }
            }
            return 1 as i32;
        }
        let __v2473: i32 = i;
        let __v2474: i32 = __v2473 + (1 as i32);
        i = __v2474;
    }
    return 0 as i32;
}

// /* Undo the work of sqlite3SetJoinExpr().  This is used when a LEFT JOIN
// ** is simplified into an ordinary JOIN, and when an ON expression is
// ** "pushed down" into the WHERE clause of a subquery.
// **
// ** Convert every term that is marked with EP_OuterON and w.iJoin==iTable into
// ** an ordinary term that omits the EP_OuterON mark.  Or if iTable<0, then
// ** just clear every EP_OuterON and EP_InnerON mark from the expression tree.
// **
// ** If nullable is true, that means that Expr p might evaluate to NULL even
// ** if it is a reference to a NOT NULL column.  This can happen, for example,
// ** if the table that p references is on the left side of a RIGHT JOIN.
// ** If nullable is true, then take care to not remove the EP_CanBeNull bit.
// ** See forum thread https://sqlite.org/forum/forumpost/b40696f50145d21c
// */
fn unsetJoinExpr(mut p: *mut Expr, mut iTable: i32, mut nullable: i32) {
    '__slate_break_2008: while p != std::ptr::null_mut::<Expr>() {
        if iTable < (0 as i32)
            || (unsafe { (*p).flags }) & ((1 as i32) as u32) != ((0 as i32) as u32)
                && (unsafe { (*p).w.iJoin }) == iTable
        {
            let __v2475: *mut Expr = p;
            let __v2476: u32 = unsafe { (*__v2475).flags };
            let __v2477: u32 = __v2476 & !(((1 as i32) | (2 as i32)) as u32);
            unsafe {
                (*__v2475).flags = __v2477;
            }
            if iTable >= (0 as i32) {
                let __v2478: *mut Expr = p;
                let __v2479: u32 = unsafe { (*__v2478).flags };
                let __v2480: u32 = __v2479 | ((2 as i32) as u32);
                unsafe {
                    (*__v2478).flags = __v2480;
                }
            }
        }
        if (((unsafe { (*p).op }) as u32) as i32) == (168 as i32)
            && (unsafe { (*p).iTable }) == iTable
            && !(nullable != (0 as i32))
        {
            let __v2481: *mut Expr = p;
            let __v2482: u32 = unsafe { (*__v2481).flags };
            let __v2483: u32 = __v2482 & !((2097152 as i32) as u32);
            unsafe {
                (*__v2481).flags = __v2483;
            }
        }
        if (((unsafe { (*p).op }) as u32) as i32) == (172 as i32) {
            0 as i32;
            0 as i32;
            if (unsafe { (*p).x.pList }) != std::ptr::null_mut::<ExprList>() {
                let mut i: i32 = 0 as i32;
                i = 0 as i32;
                '__slate_break_2009: loop {
                    if !(i < unsafe { (*unsafe { (*p).x.pList }).nExpr }) {
                        break;
                    }
                    unsetJoinExpr(
                        unsafe {
                            (*unsafe {
                                unsafe {
                                    std::ptr::addr_of_mut!((*unsafe { (*p).x.pList }).a)
                                        as *mut ExprList_item
                                }
                                .offset(i as isize)
                            })
                            .pExpr
                        },
                        iTable,
                        nullable,
                    );
                    let __v2484: i32 = i;
                    let __v2485: i32 = __v2484 + (1 as i32);
                    i = __v2485;
                }
            }
        }
        unsetJoinExpr(unsafe { (*p).pLeft }, iTable, nullable);
        p = unsafe { (*p).pRight };
    }
}

// /*
// ** This routine processes the join information for a SELECT statement.
// **
// **   *  A NATURAL join is converted into a USING join.  After that, we
// **      do not need to be concerned with NATURAL joins and we only have
// **      think about USING joins.
// **
// **   *  ON and USING clauses result in extra terms being added to the
// **      WHERE clause to enforce the specified constraints.  The extra
// **      WHERE clause terms will be tagged with EP_OuterON or
// **      EP_InnerON so that we know that they originated in ON/USING.
// **
// ** The terms of a FROM clause are contained in the Select.pSrc structure.
// ** The left most table is the first entry in Select.pSrc.  The right-most
// ** table is the last entry.  The join operator is held in the entry to
// ** the right.  Thus entry 1 contains the join operator for the join between
// ** entries 0 and 1.  Any ON or USING clauses associated with the join are
// ** also attached to the right entry.
// **
// ** This routine returns the number of errors encountered.
// */
fn sqlite3ProcessJoin(mut pParse: *mut Parse, mut p: *mut Select) -> i32 {
    // /* All tables in the FROM clause */
    let mut pSrc: *mut SrcList = unsafe { std::mem::zeroed() };
    // /* Loop counters */
    let mut i: i32 = 0 as i32;
    let mut j: i32 = 0 as i32;
    // /* Left table being joined */
    let mut pLeft: *mut SrcItem = unsafe { std::mem::zeroed() };
    // /* Right table being joined */
    let mut pRight: *mut SrcItem = unsafe { std::mem::zeroed() };
    pSrc = unsafe { (*p).pSrc };
    pLeft = unsafe {
        unsafe { std::ptr::addr_of_mut!((*pSrc).a) as *mut SrcItem }.offset((0 as i32) as isize)
    };
    pRight = unsafe { pLeft.offset((1 as i32) as isize) };
    i = 0 as i32;
    '__slate_break_2010: while i < (unsafe { (*pSrc).nSrc }) - (1 as i32) {
        let mut pRightTab: *mut Table = unsafe { (*pRight).pSTab };
        let mut joinType: u32 = 0 as u32;
        if (unsafe { (*pLeft).pSTab }) == std::ptr::null_mut::<Table>()
            || pRightTab == std::ptr::null_mut::<Table>()
        {
        } else {
            joinType = (if (((unsafe { (*pRight).fg.jointype }) as u32) as i32) & (32 as i32)
                != (0 as i32)
            {
                1 as i32
            } else {
                2 as i32
            }) as u32;
            // /* If this is a NATURAL join, synthesize an appropriate USING clause
            //     ** to specify which columns should be joined.
            //     */
            if (((unsafe { (*pRight).fg.jointype }) as u32) as i32) & (4 as i32) != (0 as i32) {
                let mut pUsing: *mut IdList = std::ptr::null_mut::<IdList>();
                if ((unsafe { (*pRight).fg.__slate_bits_0.__get_isUsing() }) as i32) != (0 as i32)
                    || (unsafe { (*pRight).u3.pOn }) != std::ptr::null_mut::<Expr>()
                {
                    unsafe {
                        sqlite3ErrorMsg(
                            pParse,
                            (b"a NATURAL join may not have an ON or USING clause\0".as_ptr()
                                as *mut i8) as *const i8,
                            0 as i32,
                        )
                    };
                    return 1 as i32;
                }
                j = 0 as i32;
                '__slate_break_2012: loop {
                    if !(j < ((unsafe { (*pRightTab).nCol }) as i32)) {
                        break;
                    }
                    // /* Name of column in the right table */
                    let mut zName: *mut i8 = unsafe { std::mem::zeroed() };
                    if (((unsafe {
                        (*unsafe { unsafe { (*pRightTab).aCol }.offset(j as isize) }).colFlags
                    }) as u32) as i32)
                        & (2 as i32)
                        != (0 as i32)
                    {
                    } else {
                        zName = unsafe {
                            (*unsafe { unsafe { (*pRightTab).aCol }.offset(j as isize) }).zCnName
                        };
                        if tableAndColumnIndex(
                            pSrc,
                            0 as i32,
                            i,
                            zName as *const i8,
                            std::ptr::null_mut::<i32>(),
                            std::ptr::null_mut::<i32>(),
                            1 as i32,
                        ) != (0 as i32)
                        {
                            pUsing = unsafe {
                                sqlite3IdListAppend(pParse, pUsing, std::ptr::null_mut::<Token>())
                            };
                            if pUsing != std::ptr::null_mut::<IdList>() {
                                0 as i32;
                                0 as i32;
                                unsafe {
                                    (*unsafe {
                                        unsafe {
                                            std::ptr::addr_of_mut!((*pUsing).a) as *mut IdList_item
                                        }
                                        .offset(((unsafe { (*pUsing).nId }) - (1 as i32)) as isize)
                                    })
                                    .zName = unsafe {
                                        sqlite3DbStrDup(unsafe { (*pParse).db }, zName as *const i8)
                                    };
                                }
                            }
                        }
                    }
                    let __v2492: i32 = j;
                    let __v2493: i32 = __v2492 + (1 as i32);
                    j = __v2493;
                }
                if pUsing != std::ptr::null_mut::<IdList>() {
                    unsafe {
                        (*pRight).fg.__slate_bits_0.__set_isUsing((1 as i32) as u32);
                    }
                    unsafe {
                        (*pRight)
                            .fg
                            .__slate_bits_0
                            .__set_isSynthUsing((1 as i32) as u32);
                    }
                    unsafe {
                        (*pRight).u3.pUsing = pUsing;
                    }
                }
                if (unsafe { (*pParse).nErr }) != (0 as i32) {
                    return 1 as i32;
                }
            }
            // /* Create extra terms on the WHERE clause for each column named
            //     ** in the USING clause.  Example: If the two tables to be joined are
            //     ** A and B and the USING clause names X, Y, and Z, then add this
            //     ** to the WHERE clause:    A.X=B.X AND A.Y=B.Y AND A.Z=B.Z
            //     ** Report an error if any column mentioned in the USING clause is
            //     ** not contained in both tables to be joined.
            //     */
            if ((unsafe { (*pRight).fg.__slate_bits_0.__get_isUsing() }) as i32) != (0 as i32) {
                let mut pList: *mut IdList = unsafe { (*pRight).u3.pUsing };
                let mut db: *mut sqlite3 = unsafe { (*pParse).db };
                0 as i32;
                j = 0 as i32;
                '__slate_break_2013: loop {
                    if !(j < unsafe { (*pList).nId }) {
                        break;
                    }
                    // /* Name of the term in the USING clause */
                    let mut zName: *mut i8 = unsafe { std::mem::zeroed() };
                    // /* Table on the left with matching column name */
                    let mut iLeft: i32 = 0 as i32;
                    // /* Column number of matching column on the left */
                    let mut iLeftCol: i32 = 0 as i32;
                    // /* Column number of matching column on the right */
                    let mut iRightCol: i32 = 0 as i32;
                    // /* Reference to the column on the LEFT of the join */
                    let mut pE1: *mut Expr = unsafe { std::mem::zeroed() };
                    // /* Reference to the column on the RIGHT of the join */
                    let mut pE2: *mut Expr = unsafe { std::mem::zeroed() };
                    // /* Equality constraint.  pE1 == pE2 */
                    let mut pEq: *mut Expr = unsafe { std::mem::zeroed() };
                    zName = unsafe {
                        (*unsafe {
                            unsafe { std::ptr::addr_of_mut!((*pList).a) as *mut IdList_item }
                                .offset(j as isize)
                        })
                        .zName
                    };
                    iRightCol = sqlite3ColumnIndex(pRightTab, zName as *const i8);
                    let __v2496: bool;
                    if iRightCol < (0 as i32) {
                        __v2496 = true as bool;
                    } else {
                        __v2496 = tableAndColumnIndex(
                            pSrc,
                            0 as i32,
                            i,
                            zName as *const i8,
                            std::ptr::addr_of_mut!(iLeft),
                            std::ptr::addr_of_mut!(iLeftCol),
                            (unsafe { (*pRight).fg.__slate_bits_0.__get_isSynthUsing() }) as i32,
                        ) == (0 as i32);
                    }
                    if __v2496 {
                        unsafe {
                            sqlite3ErrorMsg(pParse, (b"cannot join using column %s - column not present in both tables\0".as_ptr() as *mut i8) as *const i8, zName)
                        };
                        return 1 as i32;
                    }
                    pE1 = unsafe { sqlite3CreateColumnExpr(db, pSrc, iLeft, iLeftCol) };
                    sqlite3SrcItemColumnUsed(
                        unsafe {
                            unsafe { std::ptr::addr_of_mut!((*pSrc).a) as *mut SrcItem }
                                .offset(iLeft as isize)
                        },
                        iLeftCol,
                    );
                    if (((unsafe {
                        (*unsafe {
                            unsafe { std::ptr::addr_of_mut!((*pSrc).a) as *mut SrcItem }
                                .offset((0 as i32) as isize)
                        })
                        .fg
                        .jointype
                    }) as u32) as i32)
                        & (64 as i32)
                        != (0 as i32)
                        && (unsafe { (*pParse).nErr }) == (0 as i32)
                    {
                        // /* This branch runs if the query contains one or more RIGHT or FULL
                        //           ** JOINs.  If only a single table on the left side of this join
                        //           ** contains the zName column, then this branch is a no-op.
                        //           ** But if there are two or more tables on the left side
                        //           ** of the join, construct a coalesce() function that gathers all
                        //           ** such tables.  Raise an error if more than one of those references
                        //           ** to zName is not also within a prior USING clause.
                        //           **
                        //           ** We really ought to raise an error if there are two or more
                        //           ** non-USING references to zName on the left of an INNER or LEFT
                        //           ** JOIN.  But older versions of SQLite do not do that, so we avoid
                        //           ** adding a new error so as to not break legacy applications.
                        //           */
                        // /* Arguments to the coalesce() */
                        let mut pFuncArgs: *mut ExprList = std::ptr::null_mut::<ExprList>();
                        0 as i32;
                        let __v2497: *mut Expr = pE1;
                        let __v2498: u32 = unsafe { (*__v2497).flags };
                        let __v2499: u32 = __v2498 | ((2097152 as i32) as u32);
                        unsafe {
                            (*__v2497).flags = __v2499;
                        }
                        '__slate_break_2016: while tableAndColumnIndex(
                            pSrc,
                            iLeft + (1 as i32),
                            i,
                            zName as *const i8,
                            std::ptr::addr_of_mut!(iLeft),
                            std::ptr::addr_of_mut!(iLeftCol),
                            (unsafe { (*pRight).fg.__slate_bits_0.__get_isSynthUsing() }) as i32,
                        ) != (0 as i32)
                        {
                            let __v2500: bool;
                            if ((unsafe {
                                (*unsafe {
                                    unsafe { std::ptr::addr_of_mut!((*pSrc).a) as *mut SrcItem }
                                        .offset(iLeft as isize)
                                })
                                .fg
                                .__slate_bits_0
                                .__get_isUsing()
                            }) as i32)
                                == (0 as i32)
                            {
                                __v2500 = true as bool;
                            } else {
                                __v2500 = (unsafe {
                                    sqlite3IdListIndex(
                                        unsafe {
                                            (*unsafe {
                                                unsafe {
                                                    std::ptr::addr_of_mut!((*pSrc).a)
                                                        as *mut SrcItem
                                                }
                                                .offset(iLeft as isize)
                                            })
                                            .u3
                                            .pUsing
                                        },
                                        zName as *const i8,
                                    )
                                }) < (0 as i32);
                            }
                            if __v2500 {
                                unsafe {
                                    sqlite3ErrorMsg(
                                        pParse,
                                        (b"ambiguous reference to %s in USING()\0".as_ptr()
                                            as *mut i8)
                                            as *const i8,
                                        zName,
                                    )
                                };
                                break '__slate_break_2016;
                            }
                            pFuncArgs = unsafe { sqlite3ExprListAppend(pParse, pFuncArgs, pE1) };
                            pE1 = unsafe { sqlite3CreateColumnExpr(db, pSrc, iLeft, iLeftCol) };
                            sqlite3SrcItemColumnUsed(
                                unsafe {
                                    unsafe { std::ptr::addr_of_mut!((*pSrc).a) as *mut SrcItem }
                                        .offset(iLeft as isize)
                                },
                                iLeftCol,
                            );
                        }
                        if pFuncArgs != std::ptr::null_mut::<ExprList>() {
                            pFuncArgs = unsafe { sqlite3ExprListAppend(pParse, pFuncArgs, pE1) };
                            pE1 = unsafe {
                                sqlite3ExprFunction(
                                    pParse,
                                    pFuncArgs,
                                    unsafe { std::ptr::addr_of!(tkCoalesce) },
                                    0 as i32,
                                )
                            };
                            if pE1 != std::ptr::null_mut::<Expr>() {
                                unsafe {
                                    (*pE1).affExpr = (88 as i32) as i8;
                                }
                            }
                        }
                    } else {
                        if (((unsafe {
                            (*unsafe {
                                unsafe { std::ptr::addr_of_mut!((*pSrc).a) as *mut SrcItem }
                                    .offset((i + (1 as i32)) as isize)
                            })
                            .fg
                            .jointype
                        }) as u32) as i32)
                            & (8 as i32)
                            != (0 as i32)
                            && (unsafe { (*pParse).nErr }) == (0 as i32)
                        {
                            0 as i32;
                            let __v2501: *mut Expr = pE1;
                            let __v2502: u32 = unsafe { (*__v2501).flags };
                            let __v2503: u32 = __v2502 | ((2097152 as i32) as u32);
                            unsafe {
                                (*__v2501).flags = __v2503;
                            }
                        }
                    }
                    pE2 = unsafe { sqlite3CreateColumnExpr(db, pSrc, i + (1 as i32), iRightCol) };
                    sqlite3SrcItemColumnUsed(pRight, iRightCol);
                    pEq = unsafe { sqlite3PExpr(pParse, 54 as i32, pE1, pE2) };
                    0 as i32;
                    if pEq != std::ptr::null_mut::<Expr>() {
                        let __v2504: *mut Expr = pEq;
                        let __v2505: u32 = unsafe { (*__v2504).flags };
                        let __v2506: u32 = __v2505 | joinType;
                        unsafe {
                            (*__v2504).flags = __v2506;
                        }
                        0 as i32;
                        {}
                        unsafe {
                            (*pEq).w.iJoin = unsafe { (*pE2).iTable };
                        }
                    }
                    unsafe {
                        (*p).pWhere =
                            unsafe { sqlite3ExprAnd(pParse, unsafe { (*p).pWhere }, pEq) };
                    }
                    let __v2494: i32 = j;
                    let __v2495: i32 = __v2494 + (1 as i32);
                    j = __v2495;
                }
            } else {
                if (unsafe { (*pRight).u3.pOn }) != std::ptr::null_mut::<Expr>() {
                    sqlite3SetJoinExpr(
                        unsafe { (*pRight).u3.pOn },
                        unsafe { (*pRight).iCursor },
                        joinType,
                    );
                    unsafe {
                        (*p).pWhere = unsafe {
                            sqlite3ExprAnd(pParse, unsafe { (*p).pWhere }, unsafe {
                                (*pRight).u3.pOn
                            })
                        };
                    }
                    unsafe {
                        (*pRight).u3.pOn = std::ptr::null_mut::<Expr>();
                    }
                    unsafe {
                        (*pRight).fg.__slate_bits_0.__set_isOn((1 as i32) as u32);
                    }
                    let __v2507: *mut Select = p;
                    let __v2508: u32 = unsafe { (*__v2507).selFlags };
                    let __v2509: u32 = __v2508 | ((1073741824 as i32) as u32);
                    unsafe {
                        (*__v2507).selFlags = __v2509;
                    }
                }
            }
            // /* Add the ON clause to the end of the WHERE clause, connected by
            //     ** an AND operator.
            //     */
            if ((unsafe { (*pRight).fg.__slate_bits_0.__get_isTabFunc() }) as i32) != (0 as i32)
                && joinType == ((1 as i32) as u32)
                && (unsafe { (*pRight).u1.pFuncArg }) != std::ptr::null_mut::<ExprList>()
            {
                let __v2510: *mut Select = p;
                let __v2511: u32 = unsafe { (*__v2510).selFlags };
                let __v2512: u32 = __v2511 | ((1073741824 as i32) as u32);
                unsafe {
                    (*__v2510).selFlags = __v2512;
                }
            }
        }
        let __v2486: i32 = i;
        let __v2487: i32 = __v2486 + (1 as i32);
        i = __v2487;
        let __v2488: *mut SrcItem = pRight;
        let __v2489: *mut SrcItem = unsafe { __v2488.offset((1 as i32) as isize) };
        pRight = __v2489;
        let __v2490: *mut SrcItem = pLeft;
        let __v2491: *mut SrcItem = unsafe { __v2490.offset((1 as i32) as isize) };
        pLeft = __v2491;
    }
    return 0 as i32;
}

// /*
// ** This routine does the work of loading query data into an array of
// ** registers so that it can be added to the sorter.
// */
fn innerLoopLoadRow(mut pParse: *mut Parse, mut pSelect: *mut Select, mut pInfo: *mut RowLoadInfo) {
    unsafe {
        sqlite3ExprCodeExprList(
            pParse,
            unsafe { (*pSelect).pEList },
            unsafe { (*pInfo).regResult },
            0 as i32,
            unsafe { (*pInfo).ecelFlags },
        )
    };
}

// /* Statement under construction */
// /* The query being coded */
// /* Info needed to complete the row load */
// /*
// ** Code the OP_MakeRecord instruction that generates the entry to be
// ** added into the sorter.
// **
// ** Return the register in which the result is stored.
// */
fn makeSorterRecord(
    mut pParse: *mut Parse,
    mut pSort: *mut SortCtx,
    mut pSelect: *mut Select,
    mut regBase: i32,
    mut nBase: i32,
) -> i32 {
    let mut nOBSat: i32 = unsafe { (*pSort).nOBSat };
    let mut v: *mut Vdbe = unsafe { (*pParse).pVdbe };
    let mut regOut: i32 = 0 as i32;
    let __v2513: *mut Parse = pParse;
    let __v2514: i32 = unsafe { (*__v2513).nMem };
    let __v2515: i32 = __v2514 + (1 as i32);
    unsafe {
        (*__v2513).nMem = __v2515;
    }
    regOut = __v2515;
    if (unsafe { (*pSort).pDeferredRowLoad }) != std::ptr::null_mut::<RowLoadInfo>() {
        innerLoopLoadRow(pParse, pSelect, unsafe { (*pSort).pDeferredRowLoad });
    }
    unsafe { sqlite3VdbeAddOp3(v, 99 as i32, regBase + nOBSat, nBase - nOBSat, regOut) };
    return regOut;
}

// /*
// ** Generate code that will push the record in registers regData
// ** through regData+nData-1 onto the sorter.
// */
fn pushOntoSorter(
    mut pParse: *mut Parse,
    mut pSort: *mut SortCtx,
    mut pSelect: *mut Select,
    mut regData: i32,
    mut regOrigData: i32,
    mut nData: i32,
    mut nPrefixReg: i32,
) {
    // /* Stmt under construction */
    let mut v: *mut Vdbe = unsafe { (*pParse).pVdbe };
    let mut bSeq: i32 =
        ((((unsafe { (*pSort).sortFlags }) as u32) as i32) & (1 as i32) == (0 as i32)) as i32;
    // /* No. of ORDER BY terms */
    let mut nExpr: i32 = unsafe { (*unsafe { (*pSort).pOrderBy }).nExpr };
    // /* Fields in sorter record */
    let mut nBase: i32 = nExpr + bSeq + nData;
    // /* Regs for sorter record */
    let mut regBase: i32 = 0 as i32;
    // /* Assembled sorter record */
    let mut regRecord: i32 = 0 as i32;
    // /* ORDER BY terms to skip */
    let mut nOBSat: i32 = unsafe { (*pSort).nOBSat };
    // /* Opcode to add sorter record to sorter */
    let mut op: i32 = 0 as i32;
    // /* LIMIT counter */
    let mut iLimit: i32 = 0 as i32;
    // /* End of the sorter insert loop */
    let mut iSkip: i32 = 0 as i32;
    0 as i32;
    // /* Three cases:
    //   **   (1) The data to be sorted has already been packed into a Record
    //   **       by a prior OP_MakeRecord.  In this case nData==1 and regData
    //   **       will be completely unrelated to regOrigData.
    //   **   (2) All output columns are included in the sort record.  In that
    //   **       case regData==regOrigData.
    //   **   (3) Some output columns are omitted from the sort record due to
    //   **       the SQLITE_ENABLE_SORTER_REFERENCES optimization, or due to the
    //   **       SQLITE_ECEL_OMITREF optimization, or due to the
    //   **       SortCtx.pDeferredRowLoad optimization.  In any of these cases
    //   **       regOrigData is 0 to prevent this routine from trying to copy
    //   **       values that might not yet exist.
    //   */
    0 as i32;
    if nPrefixReg != (0 as i32) {
        0 as i32;
        regBase = regData - nPrefixReg;
    } else {
        regBase = (unsafe { (*pParse).nMem }) + (1 as i32);
        let __v2516: *mut Parse = pParse;
        let __v2517: i32 = unsafe { (*__v2516).nMem };
        let __v2518: i32 = __v2517 + nBase;
        unsafe {
            (*__v2516).nMem = __v2518;
        }
    }
    0 as i32;
    iLimit = if (unsafe { (*pSelect).iOffset }) != (0 as i32) {
        (unsafe { (*pSelect).iOffset }) + (1 as i32)
    } else {
        unsafe { (*pSelect).iLimit }
    };
    unsafe {
        (*pSort).labelDone = unsafe { sqlite3VdbeMakeLabel(pParse) };
    }
    unsafe {
        sqlite3ExprCodeExprList(
            pParse,
            unsafe { (*pSort).pOrderBy },
            regBase,
            regOrigData,
            (((1 as i32)
                | if regOrigData != (0 as i32) {
                    4 as i32
                } else {
                    0 as i32
                }) as i8) as u8,
        )
    };
    if bSeq != (0 as i32) {
        unsafe { sqlite3VdbeAddOp2(v, 128 as i32, unsafe { (*pSort).iECursor }, regBase + nExpr) };
    }
    if nPrefixReg == (0 as i32) && nData > (0 as i32) {
        unsafe { sqlite3ExprCodeMove(pParse, regData, regBase + nExpr + bSeq, nData) };
    }
    if nOBSat > (0 as i32) {
        // /* The first nOBSat columns of the previous row */
        let mut regPrevKey: i32 = 0 as i32;
        // /* Address of the OP_IfNot opcode */
        let mut addrFirst: i32 = 0 as i32;
        // /* Address of the OP_Jump opcode */
        let mut addrJmp: i32 = 0 as i32;
        // /* Opcode that opens the sorter */
        let mut pOp: *mut VdbeOp = unsafe { std::mem::zeroed() };
        // /* Number of sorting key columns, including OP_Sequence */
        let mut nKey: i32 = 0 as i32;
        // /* Original KeyInfo on the sorter table */
        let mut pKI: *mut KeyInfo = unsafe { std::mem::zeroed() };
        regRecord = makeSorterRecord(pParse, pSort, pSelect, regBase, nBase);
        regPrevKey = (unsafe { (*pParse).nMem }) + (1 as i32);
        let __v2519: *mut Parse = pParse;
        let __v2520: i32 = unsafe { (*__v2519).nMem };
        let __v2521: i32 = __v2520 + unsafe { (*pSort).nOBSat };
        unsafe {
            (*__v2519).nMem = __v2521;
        }
        nKey = nExpr - unsafe { (*pSort).nOBSat } + bSeq;
        if bSeq != (0 as i32) {
            addrFirst = unsafe { sqlite3VdbeAddOp1(v, 17 as i32, regBase + nExpr) };
        } else {
            addrFirst = unsafe { sqlite3VdbeAddOp1(v, 122 as i32, unsafe { (*pSort).iECursor }) };
        }
        {}
        unsafe {
            sqlite3VdbeAddOp3(v, 92 as i32, regPrevKey, regBase, unsafe {
                (*pSort).nOBSat
            })
        };
        pOp = unsafe { sqlite3VdbeGetOp(v, unsafe { (*pSort).addrSortIndex }) };
        if (unsafe { (*unsafe { (*pParse).db }).mallocFailed }) != (0 as u8) {
            return;
        }
        unsafe {
            (*pOp).p2 = nKey + nData;
        }
        pKI = unsafe { (*pOp).p4.pKeyInfo };
        // /* Makes OP_Jump testable */
        unsafe {
            memset(
                (unsafe { (*pKI).aSortFlags }) as *mut (),
                0 as i32,
                (unsafe { (*pKI).nKeyField }) as u64,
            )
        };
        unsafe { sqlite3VdbeChangeP4(v, -(1 as i32), (pKI as *mut i8) as *const i8, -(9 as i32)) };
        {}
        unsafe {
            (*pOp).p4.pKeyInfo = sqlite3KeyInfoFromExprList(
                pParse,
                unsafe { (*pSort).pOrderBy },
                nOBSat,
                (((unsafe { (*pKI).nAllField }) as u32) as i32)
                    - (((unsafe { (*pKI).nKeyField }) as u32) as i32)
                    - (1 as i32),
            );
        }
        // /* Ensure pOp not used after sqlite3VdbeAddOp3() */
        pOp = std::ptr::null_mut::<VdbeOp>();
        addrJmp = unsafe { sqlite3VdbeCurrentAddr(v) };
        unsafe {
            sqlite3VdbeAddOp3(
                v,
                14 as i32,
                addrJmp + (1 as i32),
                0 as i32,
                addrJmp + (1 as i32),
            )
        };
        {}
        unsafe {
            (*pSort).labelBkOut = unsafe { sqlite3VdbeMakeLabel(pParse) };
        }
        let __v2522: *mut Parse = pParse;
        let __v2523: i32 = unsafe { (*__v2522).nMem };
        let __v2524: i32 = __v2523 + (1 as i32);
        unsafe {
            (*__v2522).nMem = __v2524;
        }
        unsafe {
            (*pSort).regReturn = __v2524;
        }
        unsafe {
            sqlite3VdbeAddOp2(v, 10 as i32, unsafe { (*pSort).regReturn }, unsafe {
                (*pSort).labelBkOut
            })
        };
        unsafe { sqlite3VdbeAddOp1(v, 148 as i32, unsafe { (*pSort).iECursor }) };
        if iLimit != (0 as i32) {
            unsafe { sqlite3VdbeAddOp2(v, 17 as i32, iLimit, unsafe { (*pSort).labelDone }) };
            {}
        }
        unsafe { sqlite3VdbeJumpHere(v, addrFirst) };
        unsafe { sqlite3ExprCodeMove(pParse, regBase, regPrevKey, unsafe { (*pSort).nOBSat }) };
        unsafe { sqlite3VdbeJumpHere(v, addrJmp) };
    }
    if iLimit != (0 as i32) {
        // /* At this point the values for the new sorter entry are stored
        //     ** in an array of registers. They need to be composed into a record
        //     ** and inserted into the sorter if either (a) there are currently
        //     ** less than LIMIT+OFFSET items or (b) the new record is smaller than
        //     ** the largest record currently in the sorter. If (b) is true and there
        //     ** are already LIMIT+OFFSET items in the sorter, delete the largest
        //     ** entry before inserting the new one. This way there are never more
        //     ** than LIMIT+OFFSET items in the sorter.
        //     **
        //     ** If the new record does not need to be inserted into the sorter,
        //     ** jump to the next iteration of the loop. If the pSort->labelOBLopt
        //     ** value is not zero, then it is a label of where to jump.  Otherwise,
        //     ** just bypass the row insert logic.  See the header comment on the
        //     ** sqlite3WhereOrderByLimitOptLabel() function for additional info.
        //     */
        let mut iCsr: i32 = unsafe { (*pSort).iECursor };
        unsafe {
            sqlite3VdbeAddOp2(
                v,
                62 as i32,
                iLimit,
                (unsafe { sqlite3VdbeCurrentAddr(v) }) + (4 as i32),
            )
        };
        {}
        unsafe { sqlite3VdbeAddOp2(v, 32 as i32, iCsr, 0 as i32) };
        iSkip = unsafe {
            sqlite3VdbeAddOp4Int(
                v,
                41 as i32,
                iCsr,
                0 as i32,
                regBase + nOBSat,
                nExpr - nOBSat,
            )
        };
        {}
        unsafe { sqlite3VdbeAddOp1(v, 132 as i32, iCsr) };
    }
    if regRecord == (0 as i32) {
        regRecord = makeSorterRecord(pParse, pSort, pSelect, regBase, nBase);
    }
    if (((unsafe { (*pSort).sortFlags }) as u32) as i32) & (1 as i32) != (0 as i32) {
        op = 141 as i32;
    } else {
        op = 140 as i32;
    }
    unsafe {
        sqlite3VdbeAddOp4Int(
            v,
            op,
            unsafe { (*pSort).iECursor },
            regRecord,
            regBase + nOBSat,
            nBase - nOBSat,
        )
    };
    if iSkip != (0 as i32) {
        let __v2525: i32;
        if (unsafe { (*pSort).labelOBLopt }) != (0 as i32) {
            __v2525 = unsafe { (*pSort).labelOBLopt };
        } else {
            __v2525 = unsafe { sqlite3VdbeCurrentAddr(v) };
        }
        unsafe { sqlite3VdbeChangeP2(v, iSkip, __v2525) };
    }
}

// /* Parser context */
// /* Information about the ORDER BY clause */
// /* The whole SELECT statement */
// /* First register holding data to be sorted */
// /* First register holding data before packing */
// /* Number of elements in the regData data array */
// /* No. of reg prior to regData available for use */
// /*
// ** Add code to implement the OFFSET
// */
fn codeOffset(mut v: *mut Vdbe, mut iOffset: i32, mut iContinue: i32) {
    if iOffset > (0 as i32) {
        unsafe { sqlite3VdbeAddOp3(v, 61 as i32, iOffset, iContinue, 1 as i32) };
        {}
        {}
    }
}

// /* Generate code into this VM */
// /* Register holding the offset counter */
// /* Jump here to skip the current record */
// /*
// ** Add code that will check to make sure the array of registers starting at
// ** iMem form a distinct entry. This is used by both "SELECT DISTINCT ..." and
// ** distinct aggregates ("SELECT count(DISTINCT <expr>) ..."). Three strategies
// ** are available. Which is used depends on the value of parameter eTnctType,
// ** as follows:
// **
// **   WHERE_DISTINCT_UNORDERED/WHERE_DISTINCT_NOOP:
// **     Build an ephemeral table that contains all entries seen before and
// **     skip entries which have been seen before.
// **
// **     Parameter iTab is the cursor number of an ephemeral table that must
// **     be opened before the VM code generated by this routine is executed.
// **     The ephemeral cursor table is queried for a record identical to the
// **     record formed by the current array of registers. If one is found,
// **     jump to VM address addrRepeat. Otherwise, insert a new record into
// **     the ephemeral cursor and proceed.
// **
// **     The returned value in this case is a copy of parameter iTab.
// **
// **   WHERE_DISTINCT_ORDERED:
// **     In this case rows are being delivered sorted order. The ephemeral
// **     table is not required. Instead, the current set of values
// **     is compared against previous row. If they match, the new row
// **     is not distinct and control jumps to VM address addrRepeat. Otherwise,
// **     the VM program proceeds with processing the new row.
// **
// **     The returned value in this case is the register number of the first
// **     in an array of registers used to store the previous result row so that
// **     it can be compared to the next. The caller must ensure that this
// **     register is initialized to NULL.  (The fixDistinctOpenEph() routine
// **     will take care of this initialization.)
// **
// **   WHERE_DISTINCT_UNIQUE:
// **     In this case it has already been determined that the rows are distinct.
// **     No special action is required. The return value is zero.
// **
// ** Parameter pEList is the list of expressions used to generated the
// ** contents of each row. It is used by this routine to determine (a)
// ** how many elements there are in the array of registers and (b) the
// ** collation sequences that should be used for the comparisons if
// ** eTnctType is WHERE_DISTINCT_ORDERED.
// */
fn codeDistinct(
    mut pParse: *mut Parse,
    mut eTnctType: i32,
    mut iTab: i32,
    mut addrRepeat: i32,
    mut pEList: *mut ExprList,
    mut regElem: i32,
) -> i32 {
    let mut iRet: i32 = 0 as i32;
    let mut nResultCol: i32 = unsafe { (*pEList).nExpr };
    let mut v: *mut Vdbe = unsafe { (*pParse).pVdbe };
    '__slate_break_2018: {
        match eTnctType {
            2 => {
                let mut i: i32 = 0 as i32;
                // /* Jump destination */
                let mut iJump: i32 = 0 as i32;
                // /* Previous row content */
                let mut regPrev: i32 = 0 as i32;
                // /* Allocate space for the previous row */
                let __v2526: i32 = (unsafe { (*pParse).nMem }) + (1 as i32);
                regPrev = __v2526;
                iRet = __v2526;
                let __v2527: *mut Parse = pParse;
                let __v2528: i32 = unsafe { (*__v2527).nMem };
                let __v2529: i32 = __v2528 + nResultCol;
                unsafe {
                    (*__v2527).nMem = __v2529;
                }
                iJump = (unsafe { sqlite3VdbeCurrentAddr(v) }) + nResultCol;
                i = 0 as i32;
                '__slate_break_2019: loop {
                    if !(i < nResultCol) {
                        break;
                    }
                    let mut pColl: *mut CollSeq = unsafe {
                        sqlite3ExprCollSeq(
                            pParse,
                            (unsafe {
                                (*unsafe {
                                    unsafe {
                                        std::ptr::addr_of_mut!((*pEList).a) as *mut ExprList_item
                                    }
                                    .offset(i as isize)
                                })
                                .pExpr
                            }) as *const Expr,
                        )
                    };
                    if i < nResultCol - (1 as i32) {
                        unsafe { sqlite3VdbeAddOp3(v, 53 as i32, regElem + i, iJump, regPrev + i) };
                        {}
                    } else {
                        unsafe {
                            sqlite3VdbeAddOp3(v, 54 as i32, regElem + i, addrRepeat, regPrev + i)
                        };
                        {}
                    }
                    unsafe { sqlite3VdbeChangeP4(v, -(1 as i32), pColl as *const i8, -(2 as i32)) };
                    unsafe { sqlite3VdbeChangeP5(v, ((128 as i32) as i16) as u16) };
                    let __v2530: i32 = i;
                    let __v2531: i32 = __v2530 + (1 as i32);
                    i = __v2531;
                }
                0 as i32;
                unsafe {
                    sqlite3VdbeAddOp3(v, 82 as i32, regElem, regPrev, nResultCol - (1 as i32))
                };
            }
            1 => {
                // /* nothing to do */
            }
            _ => {
                let mut r1: i32 = unsafe { sqlite3GetTempReg(pParse) };
                unsafe {
                    sqlite3VdbeAddOp4Int(v, 29 as i32, iTab, addrRepeat, regElem, nResultCol)
                };
                {}
                unsafe { sqlite3VdbeAddOp3(v, 99 as i32, regElem, nResultCol, r1) };
                unsafe { sqlite3VdbeAddOp4Int(v, 140 as i32, iTab, r1, regElem, nResultCol) };
                unsafe { sqlite3VdbeChangeP5(v, ((16 as i32) as i16) as u16) };
                unsafe { sqlite3ReleaseTempReg(pParse, r1) };
                iRet = iTab;
            }
        }
    }
    return iRet;
}

// /* Parsing and code generating context */
// /* WHERE_DISTINCT_* value */
// /* A sorting index used to test for distinctness */
// /* Jump to here if not distinct */
// /* Expression for each element */
// /* First element */
// /*
// ** This routine runs after codeDistinct().  It makes necessary
// ** adjustments to the OP_OpenEphemeral opcode that the codeDistinct()
// ** routine made use of.  This processing must be done separately since
// ** sometimes codeDistinct is called before the OP_OpenEphemeral is actually
// ** laid down.
// **
// ** WHERE_DISTINCT_NOOP:
// ** WHERE_DISTINCT_UNORDERED:
// **
// **     No adjustments necessary.  This function is a no-op.
// **
// ** WHERE_DISTINCT_UNIQUE:
// **
// **     The ephemeral table is not needed.  So change the
// **     OP_OpenEphemeral opcode into an OP_Noop.
// **
// ** WHERE_DISTINCT_ORDERED:
// **
// **     The ephemeral table is not needed.  But we do need register
// **     iVal to be initialized to NULL.  So change the OP_OpenEphemeral
// **     into an OP_Null on the iVal register.
// */
fn fixDistinctOpenEph(
    mut pParse: *mut Parse,
    mut eTnctType: i32,
    mut iVal: i32,
    mut iOpenEphAddr: i32,
) {
    if (unsafe { (*pParse).nErr }) == (0 as i32)
        && (eTnctType == (1 as i32) || eTnctType == (2 as i32))
    {
        let mut v: *mut Vdbe = unsafe { (*pParse).pVdbe };
        unsafe { sqlite3VdbeChangeToNoop(v, iOpenEphAddr) };
        if (((unsafe { (*unsafe { sqlite3VdbeGetOp(v, iOpenEphAddr + (1 as i32)) }).opcode })
            as u32) as i32)
            == (190 as i32)
        {
            unsafe { sqlite3VdbeChangeToNoop(v, iOpenEphAddr + (1 as i32)) };
        }
        if eTnctType == (2 as i32) {
            // /* Change the OP_OpenEphemeral to an OP_Null that sets the MEM_Cleared
            //       ** bit on the first register of the previous value.  This will cause the
            //       ** OP_Ne added in codeDistinct() to always fail on the first iteration of
            //       ** the loop even if the first row is all NULLs.  */
            let mut pOp: *mut VdbeOp = unsafe { sqlite3VdbeGetOp(v, iOpenEphAddr) };
            unsafe {
                (*pOp).opcode = ((77 as i32) as i8) as u8;
            }
            unsafe {
                (*pOp).p1 = 1 as i32;
            }
            unsafe {
                (*pOp).p2 = iVal;
            }
        }
    }
}

// /* Parsing and code generating context */
// /* WHERE_DISTINCT_* value */
// /* Value returned by codeDistinct() */
// /* Address of OP_OpenEphemeral instruction for iTab */
// /*
// ** This routine generates the code for the inside of the inner loop
// ** of a SELECT.
// **
// ** If srcTab is negative, then the p->pEList expressions
// ** are evaluated in order to get the data for this row.  If srcTab is
// ** zero or more, then data is pulled from srcTab and p->pEList is used only
// ** to get the number of columns and the collation sequence for each column.
// */
fn selectInnerLoop(
    mut pParse: *mut Parse,
    mut p: *mut Select,
    mut srcTab: i32,
    mut pSort: *mut SortCtx,
    mut pDistinct: *mut DistinctCtx,
    mut pDest: *mut SelectDest,
    mut iContinue: i32,
    mut iBreak: i32,
) {
    let mut v: *mut Vdbe = unsafe { (*pParse).pVdbe };
    let mut i: i32 = 0 as i32;
    // /* True if the DISTINCT keyword is present */
    let mut hasDistinct: i32 = 0 as i32;
    // /* How to dispose of results */
    let mut eDest: i32 = ((unsafe { (*pDest).eDest }) as u32) as i32;
    // /* First argument to disposal method */
    let mut iParm: i32 = unsafe { (*pDest).iSDParm };
    // /* Number of result columns */
    let mut nResultCol: i32 = 0 as i32;
    // /* Number of extra registers before regResult */
    let mut nPrefixReg: i32 = 0 as i32;
    // /* Info for deferred row loading */
    let mut sRowLoadInfo: RowLoadInfo = unsafe { std::mem::zeroed() };
    // /* Usually, regResult is the first cell in an array of memory cells
    //   ** containing the current result row. In this case regOrig is set to the
    //   ** same value. However, if the results are being sent to the sorter, the
    //   ** values for any expressions that are also part of the sort-key are omitted
    //   ** from this array. In this case regOrig is set to zero.  */
    // /* Start of memory holding current results */
    let mut regResult: i32 = 0 as i32;
    // /* Start of memory holding full result (or 0) */
    let mut regOrig: i32 = 0 as i32;
    0 as i32;
    0 as i32;
    hasDistinct = if pDistinct != std::ptr::null_mut::<DistinctCtx>() {
        ((unsafe { (*pDistinct).eTnctType }) as u32) as i32
    } else {
        0 as i32
    };
    if pSort != std::ptr::null_mut::<SortCtx>()
        && (unsafe { (*pSort).pOrderBy }) == std::ptr::null_mut::<ExprList>()
    {
        pSort = std::ptr::null_mut::<SortCtx>();
    }
    if pSort == std::ptr::null_mut::<SortCtx>() && !(hasDistinct != (0 as i32)) {
        0 as i32;
        codeOffset(v, unsafe { (*p).iOffset }, iContinue);
    }
    // /* Pull the requested columns.
    //   */
    nResultCol = unsafe { (*unsafe { (*p).pEList }).nExpr };
    if (unsafe { (*pDest).iSdst }) == (0 as i32) {
        if pSort != std::ptr::null_mut::<SortCtx>() {
            nPrefixReg = unsafe { (*unsafe { (*pSort).pOrderBy }).nExpr };
            if !((((unsafe { (*pSort).sortFlags }) as u32) as i32) & (1 as i32) != (0 as i32)) {
                let __v2532: i32 = nPrefixReg;
                let __v2533: i32 = __v2532 + (1 as i32);
                nPrefixReg = __v2533;
            }
            let __v2534: *mut Parse = pParse;
            let __v2535: i32 = unsafe { (*__v2534).nMem };
            let __v2536: i32 = __v2535 + nPrefixReg;
            unsafe {
                (*__v2534).nMem = __v2536;
            }
        }
        unsafe {
            (*pDest).iSdst = (unsafe { (*pParse).nMem }) + (1 as i32);
        }
        let __v2537: *mut Parse = pParse;
        let __v2538: i32 = unsafe { (*__v2537).nMem };
        let __v2539: i32 = __v2538 + nResultCol;
        unsafe {
            (*__v2537).nMem = __v2539;
        }
    } else {
        if (unsafe { (*pDest).iSdst }) + nResultCol > unsafe { (*pParse).nMem } {
            // /* This is an error condition that can result, for example, when a SELECT
            //     ** on the right-hand side of an INSERT contains more result columns than
            //     ** there are columns in the table on the left.  The error will be caught
            //     ** and reported later.  But we need to make sure enough memory is allocated
            //     ** to avoid other spurious errors in the meantime. */
            let __v2540: *mut Parse = pParse;
            let __v2541: i32 = unsafe { (*__v2540).nMem };
            let __v2542: i32 = __v2541 + nResultCol;
            unsafe {
                (*__v2540).nMem = __v2542;
            }
        }
    }
    unsafe {
        (*pDest).nSdst = nResultCol;
    }
    let __v2543: i32 = unsafe { (*pDest).iSdst };
    regResult = __v2543;
    regOrig = __v2543;
    if srcTab >= (0 as i32) {
        i = 0 as i32;
        '__slate_break_2020: loop {
            if !(i < nResultCol) {
                break;
            }
            unsafe { sqlite3VdbeAddOp3(v, 96 as i32, srcTab, i, regResult + i) };
            {}
            let __v2544: i32 = i;
            let __v2545: i32 = __v2544 + (1 as i32);
            i = __v2545;
        }
    } else {
        if eDest != (1 as i32) {
            // /* If the destination is an EXISTS(...) expression, the actual
            //     ** values returned by the SELECT are not required.
            //     */
            // /* "ecel" is an abbreviation of "ExprCodeExprList" */
            let mut ecelFlags: u8 = 0 as u8;
            let mut pEList: *mut ExprList = unsafe { std::mem::zeroed() };
            if eDest == (8 as i32) || eDest == (7 as i32) || eDest == (11 as i32) {
                ecelFlags = ((1 as i32) as i8) as u8;
            } else {
                ecelFlags = ((0 as i32) as i8) as u8;
            }
            if pSort != std::ptr::null_mut::<SortCtx>()
                && hasDistinct == (0 as i32)
                && eDest != (10 as i32)
                && eDest != (12 as i32)
            {
                // /* For each expression in p->pEList that is a copy of an expression in
                //       ** the ORDER BY clause (pSort->pOrderBy), set the associated
                //       ** iOrderByCol value to one more than the index of the ORDER BY
                //       ** expression within the sort-key that pushOntoSorter() will generate.
                //       ** This allows the p->pEList field to be omitted from the sorted record,
                //       ** saving space and CPU cycles.  */
                let __v2546: u8 = ecelFlags;
                let __v2547: u8 =
                    ((((__v2546 as u32) as i32) | ((8 as i32) | (4 as i32))) as i8) as u8;
                ecelFlags = __v2547;
                i = unsafe { (*pSort).nOBSat };
                '__slate_break_2021: loop {
                    if !(i < unsafe { (*unsafe { (*pSort).pOrderBy }).nExpr }) {
                        break;
                    }
                    let mut j: i32 = 0 as i32;
                    let __v2550: i32 = ((unsafe {
                        (*unsafe {
                            unsafe {
                                std::ptr::addr_of_mut!((*unsafe { (*pSort).pOrderBy }).a)
                                    as *mut ExprList_item
                            }
                            .offset(i as isize)
                        })
                        .u
                        .x
                        .iOrderByCol
                    }) as u32) as i32;
                    j = __v2550;
                    if __v2550 > (0 as i32) {
                        unsafe {
                            (*unsafe {
                                unsafe {
                                    std::ptr::addr_of_mut!((*unsafe { (*p).pEList }).a)
                                        as *mut ExprList_item
                                }
                                .offset((j - (1 as i32)) as isize)
                            })
                            .u
                            .x
                            .iOrderByCol =
                                ((i + (1 as i32) - unsafe { (*pSort).nOBSat }) as i16) as u16;
                        }
                    }
                    let __v2548: i32 = i;
                    let __v2549: i32 = __v2548 + (1 as i32);
                    i = __v2549;
                }
                // /* Adjust nResultCol to account for columns that are omitted
                //       ** from the sorter by the optimizations in this branch */
                pEList = unsafe { (*p).pEList };
                i = 0 as i32;
                '__slate_break_2022: loop {
                    if !(i < unsafe { (*pEList).nExpr }) {
                        break;
                    }
                    if (((unsafe {
                        (*unsafe {
                            unsafe { std::ptr::addr_of_mut!((*pEList).a) as *mut ExprList_item }
                                .offset(i as isize)
                        })
                        .u
                        .x
                        .iOrderByCol
                    }) as u32) as i32)
                        > (0 as i32)
                    {
                        let __v2553: i32 = nResultCol;
                        let __v2554: i32 = __v2553 - (1 as i32);
                        nResultCol = __v2554;
                        regOrig = 0 as i32;
                    }
                    let __v2551: i32 = i;
                    let __v2552: i32 = __v2551 + (1 as i32);
                    i = __v2552;
                }
                {}
                {}
                {}
                {}
                {}
                0 as i32;
            }
            sRowLoadInfo.regResult = regResult;
            sRowLoadInfo.ecelFlags = ecelFlags;
            if (unsafe { (*p).iLimit }) != (0 as i32)
                && ((ecelFlags as u32) as i32) & (8 as i32) != (0 as i32)
                && nPrefixReg > (0 as i32)
            {
                0 as i32;
                0 as i32;
                unsafe {
                    (*pSort).pDeferredRowLoad = std::ptr::addr_of_mut!(sRowLoadInfo);
                }
                regOrig = 0 as i32;
            } else {
                innerLoopLoadRow(pParse, p, std::ptr::addr_of_mut!(sRowLoadInfo));
            }
        }
    }
    // /* If the DISTINCT keyword was present on the SELECT statement
    //   ** and this row has been seen before, then do not make this row
    //   ** part of the result.
    //   */
    if hasDistinct != (0 as i32) {
        let mut eType: i32 = ((unsafe { (*pDistinct).eTnctType }) as u32) as i32;
        let mut iTab: i32 = unsafe { (*pDistinct).tabTnct };
        0 as i32;
        iTab = codeDistinct(
            pParse,
            eType,
            iTab,
            iContinue,
            unsafe { (*p).pEList },
            regResult,
        );
        fixDistinctOpenEph(pParse, eType, iTab, unsafe { (*pDistinct).addrTnct });
        if pSort == std::ptr::null_mut::<SortCtx>() {
            codeOffset(v, unsafe { (*p).iOffset }, iContinue);
        }
    }
    '__slate_break_2023: {
        match eDest {
            6 | 3 | 12 | 10 => {
                // /* Store the result as data using a unique key.
                //     */
                let mut r1: i32 = unsafe { sqlite3GetTempRange(pParse, nPrefixReg + (1 as i32)) };
                {}
                {}
                {}
                {}
                unsafe { sqlite3VdbeAddOp3(v, 99 as i32, regResult, nResultCol, r1 + nPrefixReg) };
                if eDest == (3 as i32) {
                    // /* If the destination is DistFifo, then cursor (iParm+1) is open
                    //         ** on an ephemeral index. If the current row is already present
                    //         ** in the index, do not write it to the output. If not, add the
                    //         ** current row to the index and proceed with writing it to the
                    //         ** output table as well.  */
                    let mut addr: i32 = (unsafe { sqlite3VdbeCurrentAddr(v) }) + (4 as i32);
                    unsafe {
                        sqlite3VdbeAddOp4Int(v, 29 as i32, iParm + (1 as i32), addr, r1, 0 as i32)
                    };
                    {}
                    unsafe {
                        sqlite3VdbeAddOp4Int(
                            v,
                            140 as i32,
                            iParm + (1 as i32),
                            r1,
                            regResult,
                            nResultCol,
                        )
                    };
                    0 as i32;
                }
                if pSort != std::ptr::null_mut::<SortCtx>() {
                    0 as i32;
                    pushOntoSorter(
                        pParse,
                        pSort,
                        p,
                        r1 + nPrefixReg,
                        regOrig,
                        1 as i32,
                        nPrefixReg,
                    );
                } else {
                    let mut r2: i32 = unsafe { sqlite3GetTempReg(pParse) };
                    unsafe { sqlite3VdbeAddOp2(v, 129 as i32, iParm, r2) };
                    unsafe { sqlite3VdbeAddOp3(v, 130 as i32, iParm, r1, r2) };
                    unsafe { sqlite3VdbeChangeP5(v, ((8 as i32) as i16) as u16) };
                    unsafe { sqlite3ReleaseTempReg(pParse, r2) };
                }
                unsafe { sqlite3ReleaseTempRange(pParse, r1, nPrefixReg + (1 as i32)) };
            }
            13 => {
                if pSort != std::ptr::null_mut::<SortCtx>() {
                    pushOntoSorter(pParse, pSort, p, regResult, regOrig, nResultCol, nPrefixReg);
                } else {
                    let mut i2: i32 = unsafe { (*pDest).iSDParm2 };
                    let mut r1: i32 = unsafe { sqlite3GetTempReg(pParse) };
                    // /* If the UPDATE FROM join is an aggregate that matches no rows, it
                    //         ** might still be trying to return one row, because that is what
                    //         ** aggregates do.  Don't record that empty row in the output table. */
                    unsafe { sqlite3VdbeAddOp2(v, 51 as i32, regResult, iBreak) };
                    {}
                    unsafe {
                        sqlite3VdbeAddOp3(
                            v,
                            99 as i32,
                            regResult + ((i2 < (0 as i32)) as i32),
                            nResultCol - ((i2 < (0 as i32)) as i32),
                            r1,
                        )
                    };
                    if i2 < (0 as i32) {
                        unsafe { sqlite3VdbeAddOp3(v, 130 as i32, iParm, r1, regResult) };
                    } else {
                        unsafe { sqlite3VdbeAddOp4Int(v, 140 as i32, iParm, r1, regResult, i2) };
                    }
                }
                break '__slate_break_2023;
                // /* If we are creating a set for an "expr IN (SELECT ...)" construct,
                //     ** then there should be a single item on the stack.  Write this
                //     ** item into the set table with bogus data.
                //     */
            }
            9 => {
                if pSort != std::ptr::null_mut::<SortCtx>() {
                    // /* At first glance you would think we could optimize out the
                    //         ** ORDER BY in this case since the order of entries in the set
                    //         ** does not matter.  But there might be a LIMIT clause, in which
                    //         ** case the order does matter */
                    pushOntoSorter(pParse, pSort, p, regResult, regOrig, nResultCol, nPrefixReg);
                    // /* Signal that any Bloom filter is unpopulated */
                    unsafe {
                        (*pDest).iSDParm2 = 0 as i32;
                    }
                } else {
                    let mut r1: i32 = unsafe { sqlite3GetTempReg(pParse) };
                    0 as i32;
                    unsafe {
                        sqlite3VdbeAddOp4(
                            v,
                            99 as i32,
                            regResult,
                            nResultCol,
                            r1,
                            (unsafe { (*pDest).zAffSdst }) as *const i8,
                            nResultCol,
                        )
                    };
                    unsafe {
                        sqlite3VdbeAddOp4Int(v, 140 as i32, iParm, r1, regResult, nResultCol)
                    };
                    if (unsafe { (*pDest).iSDParm2 }) != (0 as i32) {
                        0 as i32;
                        unsafe {
                            sqlite3VdbeAddOp4Int(
                                v,
                                185 as i32,
                                unsafe { (*pDest).iSDParm2 },
                                0 as i32,
                                regResult,
                                nResultCol,
                            )
                        };
                        unsafe {
                            sqlite3VdbeExplain(
                                pParse,
                                ((0 as i32) as i8) as u8,
                                (b"CREATE BLOOM FILTER\0".as_ptr() as *mut i8) as *const i8,
                            )
                        };
                    }
                    unsafe { sqlite3ReleaseTempReg(pParse, r1) };
                }
                break '__slate_break_2023;
                // /* If any row exist in the result set, record that fact and abort.
                //     */
            }
            1 => {
                unsafe { sqlite3VdbeAddOp2(v, 73 as i32, 1 as i32, iParm) };
                // /* The LIMIT clause will terminate the loop for us */
                break '__slate_break_2023;
                // /* If this is a scalar select that is part of an expression, then
                //     ** store the results in the appropriate memory cell or array of
                //     ** memory cells and break out of the scan loop.
                //     */
            }
            8 => {
                if pSort != std::ptr::null_mut::<SortCtx>() {
                    0 as i32;
                    pushOntoSorter(pParse, pSort, p, regResult, regOrig, nResultCol, nPrefixReg);
                    unsafe {
                        (*pDest).iSDParm = regResult;
                    }
                } else {
                    0 as i32;
                    if regResult != iParm {
                        // /* This occurs in cases where the SELECT had both a DISTINCT and
                        //           ** an OFFSET clause.  */
                        unsafe {
                            sqlite3VdbeAddOp3(
                                v,
                                82 as i32,
                                regResult,
                                iParm,
                                nResultCol - (1 as i32),
                            )
                        };
                    }
                    // /* The LIMIT clause will jump out of the loop for us */
                }
                break '__slate_break_2023;
                // /* #ifndef SQLITE_OMIT_SUBQUERY */
                // /* Send data to a co-routine */
            }
            11 | 7 => {
                {}
                {}
                if pSort != std::ptr::null_mut::<SortCtx>() {
                    pushOntoSorter(pParse, pSort, p, regResult, regOrig, nResultCol, nPrefixReg);
                } else {
                    if eDest == (11 as i32) {
                        unsafe { sqlite3VdbeAddOp1(v, 12 as i32, unsafe { (*pDest).iSDParm }) };
                    } else {
                        unsafe { sqlite3VdbeAddOp2(v, 86 as i32, regResult, nResultCol) };
                    }
                }
                break '__slate_break_2023;
                // /* Return the results */
                // /* Write the results into a priority queue that is order according to
                //     ** pDest->pOrderBy (in pSO).  pDest->iSDParm (in iParm) is the cursor for an
                //     ** index with pSO->nExpr+2 columns.  Build a key using pSO for the first
                //     ** pSO->nExpr columns, then make sure all keys are unique by adding a
                //     ** final OP_Sequence column.  The last column is the record as a blob.
                //     */
            }
            4 | 5 => {
                let mut nKey: i32 = 0 as i32;
                let mut r1: i32 = 0 as i32;
                let mut r2: i32 = 0 as i32;
                let mut r3: i32 = 0 as i32;
                let mut addrTest: i32 = 0 as i32;
                let mut pSO: *mut ExprList = unsafe { std::mem::zeroed() };
                pSO = unsafe { (*pDest).pOrderBy };
                0 as i32;
                nKey = unsafe { (*pSO).nExpr };
                r1 = unsafe { sqlite3GetTempReg(pParse) };
                r2 = unsafe { sqlite3GetTempRange(pParse, nKey + (2 as i32)) };
                r3 = r2 + nKey + (1 as i32);
                if eDest == (4 as i32) {
                    // /* If the destination is DistQueue, then cursor (iParm+1) is open
                    //         ** on a second ephemeral index that holds all values every previously
                    //         ** added to the queue. */
                    addrTest = unsafe {
                        sqlite3VdbeAddOp4Int(
                            v,
                            29 as i32,
                            iParm + (1 as i32),
                            0 as i32,
                            regResult,
                            nResultCol,
                        )
                    };
                    {}
                }
                unsafe { sqlite3VdbeAddOp3(v, 99 as i32, regResult, nResultCol, r3) };
                if eDest == (4 as i32) {
                    unsafe { sqlite3VdbeAddOp2(v, 140 as i32, iParm + (1 as i32), r3) };
                    unsafe { sqlite3VdbeChangeP5(v, ((16 as i32) as i16) as u16) };
                }
                i = 0 as i32;
                '__slate_break_2025: loop {
                    if !(i < nKey) {
                        break;
                    }
                    unsafe {
                        sqlite3VdbeAddOp2(
                            v,
                            83 as i32,
                            regResult
                                + (((unsafe {
                                    (*unsafe {
                                        unsafe {
                                            std::ptr::addr_of_mut!((*pSO).a) as *mut ExprList_item
                                        }
                                        .offset(i as isize)
                                    })
                                    .u
                                    .x
                                    .iOrderByCol
                                }) as u32) as i32)
                                - (1 as i32),
                            r2 + i,
                        )
                    };
                    let __v2555: i32 = i;
                    let __v2556: i32 = __v2555 + (1 as i32);
                    i = __v2556;
                }
                unsafe { sqlite3VdbeAddOp2(v, 128 as i32, iParm, r2 + nKey) };
                unsafe { sqlite3VdbeAddOp3(v, 99 as i32, r2, nKey + (2 as i32), r1) };
                unsafe { sqlite3VdbeAddOp4Int(v, 140 as i32, iParm, r1, r2, nKey + (2 as i32)) };
                if addrTest != (0 as i32) {
                    unsafe { sqlite3VdbeJumpHere(v, addrTest) };
                }
                unsafe { sqlite3ReleaseTempReg(pParse, r1) };
                unsafe { sqlite3ReleaseTempRange(pParse, r2, nKey + (2 as i32)) };
                break '__slate_break_2023;
                // /* SQLITE_OMIT_CTE */
                // /* Discard the results.  This is used for SELECT statements inside
                //     ** the body of a TRIGGER.  The purpose of such selects is to call
                //     ** user-defined functions that have side effects.  We do not care
                //     ** about the actual results of the select.
                //     */
            }
            _ => {
                0 as i32;
            }
        }
    }
    // /* Jump to the end of the loop if the LIMIT is reached.  Except, if
    //   ** there is a sorter, in which case the sorter has already limited
    //   ** the output for us.
    //   */
    if pSort == std::ptr::null_mut::<SortCtx>() && (unsafe { (*p).iLimit }) != (0 as i32) {
        unsafe { sqlite3VdbeAddOp2(v, 63 as i32, unsafe { (*p).iLimit }, iBreak) };
        {}
    }
}

// /*
// ** Unless an "EXPLAIN QUERY PLAN" command is being processed, this function
// ** is a no-op. Otherwise, it adds a single row of output to the EQP result,
// ** where the caption is of the form:
// **
// **   "USE TEMP B-TREE FOR xxx"
// **
// ** where xxx is one of "DISTINCT", "ORDER BY" or "GROUP BY". Exactly which
// ** is determined by the zUsage argument.
// */
fn explainTempTable(mut pParse: *mut Parse, mut zUsage: *const i8) {
    unsafe {
        sqlite3VdbeExplain(
            pParse,
            ((0 as i32) as i8) as u8,
            (b"USE TEMP B-TREE FOR %s\0".as_ptr() as *mut i8) as *const i8,
            zUsage,
        )
    };
}

// /*
// ** Assign expression b to lvalue a. A second, no-op, version of this macro
// ** is provided when SQLITE_OMIT_EXPLAIN is defined. This allows the code
// ** in sqlite3Select() to assign values to structure member variables that
// ** only exist if SQLITE_OMIT_EXPLAIN is not defined without polluting the
// ** code with #ifndef directives.
// */
// /*
// ** If the inner loop was generated using a non-null pOrderBy argument,
// ** then the results were placed in a sorter.  After the loop is terminated
// ** we need to run the sorter and output the results.  The following
// ** routine generates the code needed to do that.
// */
fn generateSortTail(
    mut pParse: *mut Parse,
    mut p: *mut Select,
    mut pSort: *mut SortCtx,
    mut nColumn: i32,
    mut pDest: *mut SelectDest,
) {
    // /* The prepared statement */
    let mut v: *mut Vdbe = unsafe { (*pParse).pVdbe };
    // /* Jump here to exit loop */
    let mut addrBreak: i32 = unsafe { (*pSort).labelDone };
    // /* Jump here for next cycle */
    let mut addrContinue: i32 = unsafe { sqlite3VdbeMakeLabel(pParse) };
    // /* Top of output loop. Jump for Next. */
    let mut addr: i32 = 0 as i32;
    let mut addrOnce: i32 = 0 as i32;
    let mut iTab: i32 = 0 as i32;
    let mut pOrderBy: *mut ExprList = unsafe { (*pSort).pOrderBy };
    let mut eDest: i32 = ((unsafe { (*pDest).eDest }) as u32) as i32;
    let mut iParm: i32 = unsafe { (*pDest).iSDParm };
    let mut regRow: i32 = 0 as i32;
    let mut regRowid: i32 = 0 as i32;
    let mut iCol: i32 = 0 as i32;
    // /* Number of key columns in sorter record */
    let mut nKey: i32 = 0 as i32;
    // /* Sorter cursor to read from */
    let mut iSortTab: i32 = 0 as i32;
    let mut i: i32 = 0 as i32;
    // /* True if sorter record includes seq. no. */
    let mut bSeq: i32 = 0 as i32;
    let mut nRefKey: i32 = 0 as i32;
    let mut aOutEx: *mut ExprList_item =
        unsafe { std::ptr::addr_of_mut!((*unsafe { (*p).pEList }).a) as *mut ExprList_item };
    nKey = (unsafe { (*pOrderBy).nExpr }) - unsafe { (*pSort).nOBSat };
    if (unsafe { (*pSort).nOBSat }) == (0 as i32) || nKey == (1 as i32) {
        unsafe {
            sqlite3VdbeExplain(
                pParse,
                ((0 as i32) as i8) as u8,
                (b"USE TEMP B-TREE FOR %sORDER BY\0".as_ptr() as *mut i8) as *const i8,
                if (unsafe { (*pSort).nOBSat }) != (0 as i32) {
                    b"LAST TERM OF \0".as_ptr() as *mut i8
                } else {
                    b"\0".as_ptr() as *mut i8
                },
            )
        };
    } else {
        unsafe {
            sqlite3VdbeExplain(
                pParse,
                ((0 as i32) as i8) as u8,
                (b"USE TEMP B-TREE FOR LAST %d TERMS OF ORDER BY\0".as_ptr() as *mut i8)
                    as *const i8,
                nKey,
            )
        };
    }
    {}
    {}
    0 as i32;
    if (unsafe { (*pSort).labelBkOut }) != (0 as i32) {
        unsafe {
            sqlite3VdbeAddOp2(v, 10 as i32, unsafe { (*pSort).regReturn }, unsafe {
                (*pSort).labelBkOut
            })
        };
        unsafe { sqlite3VdbeGoto(v, addrBreak) };
        unsafe { sqlite3VdbeResolveLabel(v, unsafe { (*pSort).labelBkOut }) };
    }
    iTab = unsafe { (*pSort).iECursor };
    if eDest == (7 as i32) || eDest == (11 as i32) || eDest == (8 as i32) {
        if eDest == (8 as i32) && (unsafe { (*p).iOffset }) != (0 as i32) {
            unsafe { sqlite3VdbeAddOp2(v, 77 as i32, 0 as i32, unsafe { (*pDest).iSdst }) };
        }
        regRowid = 0 as i32;
        regRow = unsafe { (*pDest).iSdst };
    } else {
        regRowid = unsafe { sqlite3GetTempReg(pParse) };
        if eDest == (10 as i32) || eDest == (12 as i32) {
            regRow = unsafe { sqlite3GetTempReg(pParse) };
            nColumn = 0 as i32;
        } else {
            regRow = unsafe { sqlite3GetTempRange(pParse, nColumn) };
        }
    }
    if (((unsafe { (*pSort).sortFlags }) as u32) as i32) & (1 as i32) != (0 as i32) {
        let mut regSortOut: i32 = 0 as i32;
        let __v2557: *mut Parse = pParse;
        let __v2558: i32 = unsafe { (*__v2557).nMem };
        let __v2559: i32 = __v2558 + (1 as i32);
        unsafe {
            (*__v2557).nMem = __v2559;
        }
        regSortOut = __v2559;
        let __v2560: *mut Parse = pParse;
        let __v2561: i32 = unsafe { (*__v2560).nTab };
        let __v2562: i32 = __v2561 + (1 as i32);
        unsafe {
            (*__v2560).nTab = __v2562;
        }
        iSortTab = __v2561;
        if (unsafe { (*pSort).labelBkOut }) != (0 as i32) {
            addrOnce = unsafe { sqlite3VdbeAddOp0(v, 15 as i32) };
            {}
        }
        unsafe {
            sqlite3VdbeAddOp3(
                v,
                123 as i32,
                iSortTab,
                regSortOut,
                nKey + (1 as i32) + nColumn + nRefKey,
            )
        };
        if addrOnce != (0 as i32) {
            unsafe { sqlite3VdbeJumpHere(v, addrOnce) };
        }
        addr = (1 as i32) + unsafe { sqlite3VdbeAddOp2(v, 34 as i32, iTab, addrBreak) };
        {}
        0 as i32;
        unsafe { sqlite3VdbeAddOp3(v, 135 as i32, iTab, regSortOut, iSortTab) };
        bSeq = 0 as i32;
    } else {
        addr = (1 as i32) + unsafe { sqlite3VdbeAddOp2(v, 35 as i32, iTab, addrBreak) };
        {}
        codeOffset(v, unsafe { (*p).iOffset }, addrContinue);
        iSortTab = iTab;
        bSeq = 1 as i32;
        if (unsafe { (*p).iOffset }) > (0 as i32) {
            unsafe { sqlite3VdbeAddOp2(v, 88 as i32, unsafe { (*p).iLimit }, -(1 as i32)) };
        }
    }
    i = 0 as i32;
    let __v2563: i32 = nKey + bSeq - (1 as i32);
    iCol = __v2563;
    '__slate_break_2037: loop {
        if !(i < nColumn) {
            break;
        }
        if (((unsafe { (*unsafe { aOutEx.offset(i as isize) }).u.x.iOrderByCol }) as u32) as i32)
            == (0 as i32)
        {
            let __v2566: i32 = iCol;
            let __v2567: i32 = __v2566 + (1 as i32);
            iCol = __v2567;
        }
        let __v2564: i32 = i;
        let __v2565: i32 = __v2564 + (1 as i32);
        i = __v2565;
    }
    i = nColumn - (1 as i32);
    '__slate_break_2038: loop {
        if !(i >= (0 as i32)) {
            break;
        }
        let mut iRead: i32 = 0 as i32;
        if (unsafe { (*unsafe { aOutEx.offset(i as isize) }).u.x.iOrderByCol }) != (0 as u16) {
            iRead = (((unsafe { (*unsafe { aOutEx.offset(i as isize) }).u.x.iOrderByCol }) as u32)
                as i32)
                - (1 as i32);
        } else {
            let __v2570: i32 = iCol;
            let __v2571: i32 = __v2570 - (1 as i32);
            iCol = __v2571;
            iRead = __v2570;
        }
        unsafe { sqlite3VdbeAddOp3(v, 96 as i32, iSortTab, iRead, regRow + i) };
        {}
        let __v2568: i32 = i;
        let __v2569: i32 = __v2568 - (1 as i32);
        i = __v2569;
    }
    {}
    '__slate_break_2039: {
        match eDest {
            12 | 10 => {
                unsafe { sqlite3VdbeAddOp3(v, 96 as i32, iSortTab, nKey + bSeq, regRow) };
                unsafe { sqlite3VdbeAddOp2(v, 129 as i32, iParm, regRowid) };
                unsafe { sqlite3VdbeAddOp3(v, 130 as i32, iParm, regRow, regRowid) };
                unsafe { sqlite3VdbeChangeP5(v, ((8 as i32) as i16) as u16) };
            }
            9 => {
                0 as i32;
                unsafe {
                    sqlite3VdbeAddOp4(
                        v,
                        99 as i32,
                        regRow,
                        nColumn,
                        regRowid,
                        (unsafe { (*pDest).zAffSdst }) as *const i8,
                        nColumn,
                    )
                };
                unsafe { sqlite3VdbeAddOp4Int(v, 140 as i32, iParm, regRowid, regRow, nColumn) };
            }
            8 => {
                // /* The LIMIT clause will terminate the loop for us */
            }
            13 => {
                let mut i2: i32 = unsafe { (*pDest).iSDParm2 };
                let mut r1: i32 = unsafe { sqlite3GetTempReg(pParse) };
                unsafe {
                    sqlite3VdbeAddOp3(
                        v,
                        99 as i32,
                        regRow + ((i2 < (0 as i32)) as i32),
                        nColumn - ((i2 < (0 as i32)) as i32),
                        r1,
                    )
                };
                if i2 < (0 as i32) {
                    unsafe { sqlite3VdbeAddOp3(v, 130 as i32, iParm, r1, regRow) };
                } else {
                    unsafe { sqlite3VdbeAddOp4Int(v, 140 as i32, iParm, r1, regRow, i2) };
                }
            }
            _ => {
                0 as i32;
                {}
                {}
                if eDest == (7 as i32) {
                    unsafe { sqlite3VdbeAddOp2(v, 86 as i32, unsafe { (*pDest).iSdst }, nColumn) };
                } else {
                    unsafe { sqlite3VdbeAddOp1(v, 12 as i32, unsafe { (*pDest).iSDParm }) };
                }
            }
        }
    }
    if regRowid != (0 as i32) {
        if eDest == (9 as i32) {
            unsafe { sqlite3ReleaseTempRange(pParse, regRow, nColumn) };
        } else {
            unsafe { sqlite3ReleaseTempReg(pParse, regRow) };
        }
        unsafe { sqlite3ReleaseTempReg(pParse, regRowid) };
    }
    // /* The bottom of the loop
    //   */
    unsafe { sqlite3VdbeResolveLabel(v, addrContinue) };
    if (((unsafe { (*pSort).sortFlags }) as u32) as i32) & (1 as i32) != (0 as i32) {
        unsafe { sqlite3VdbeAddOp2(v, 38 as i32, iTab, addr) };
        {}
    } else {
        unsafe { sqlite3VdbeAddOp2(v, 40 as i32, iTab, addr) };
        {}
    }
    {}
    if (unsafe { (*pSort).regReturn }) != (0 as i32) {
        unsafe { sqlite3VdbeAddOp1(v, 69 as i32, unsafe { (*pSort).regReturn }) };
    }
    unsafe { sqlite3VdbeResolveLabel(v, addrBreak) };
}

// /* Parsing context */
// /* The SELECT statement */
// /* Information on the ORDER BY clause */
// /* Number of columns of data */
// /* Write the sorted results here */
// /*
// ** Return a pointer to a string containing the 'declaration type' of the
// ** expression pExpr. The string may be treated as static by the caller.
// **
// ** The declaration type is the exact datatype definition extracted from the
// ** original CREATE TABLE statement if the expression is a column. The
// ** declaration type for a ROWID field is INTEGER. Exactly when an expression
// ** is considered a column can be complex in the presence of subqueries. The
// ** result-set expression in all of the following SELECT statements is
// ** considered a column by this function.
// **
// **   SELECT col FROM tbl;
// **   SELECT (SELECT col FROM tbl;
// **   SELECT (SELECT col FROM tbl);
// **   SELECT abc FROM (SELECT col AS abc FROM tbl);
// **
// ** The declaration type for any expression other than a column is NULL.
// **
// ** This routine has either 3 or 6 parameters depending on whether or not
// ** the SQLITE_ENABLE_COLUMN_METADATA compile-time option is used.
// */
fn columnTypeImpl(mut pNC: *mut NameContext, mut pExpr: *mut Expr) -> *const i8 {
    let mut zType: *const i8 = std::ptr::null::<i8>();
    let mut j: i32 = 0 as i32;
    0 as i32;
    0 as i32;
    '__slate_break_2040: {
        match ((unsafe { (*pExpr).op }) as u32) as i32 {
            168 => {
                // /* The expression is a column. Locate the table the column is being
                //       ** extracted from in NameContext.pSrcList. This table may be real
                //       ** database table or a subquery.
                //       */
                // /* Table structure column is extracted from */
                let mut pTab: *mut Table = std::ptr::null_mut::<Table>();
                // /* Select the column is extracted from */
                let mut pS: *mut Select = std::ptr::null_mut::<Select>();
                // /* Index of column in pTab */
                let mut iCol: i32 = (unsafe { (*pExpr).iColumn }) as i32;
                '__slate_break_2041: while pNC != std::ptr::null_mut::<NameContext>()
                    && !(pTab != std::ptr::null_mut::<Table>())
                {
                    let mut pTabList: *mut SrcList = unsafe { (*pNC).pSrcList };
                    j = 0 as i32;
                    '__slate_break_2042: loop {
                        if !(j < unsafe { (*pTabList).nSrc }
                            && (unsafe {
                                (*unsafe {
                                    unsafe { std::ptr::addr_of_mut!((*pTabList).a) as *mut SrcItem }
                                        .offset(j as isize)
                                })
                                .iCursor
                            }) != unsafe { (*pExpr).iTable })
                        {
                            break;
                        }
                        {}
                        let __v2572: i32 = j;
                        let __v2573: i32 = __v2572 + (1 as i32);
                        j = __v2573;
                    }
                    if j < unsafe { (*pTabList).nSrc } {
                        pTab = unsafe {
                            (*unsafe {
                                unsafe { std::ptr::addr_of_mut!((*pTabList).a) as *mut SrcItem }
                                    .offset(j as isize)
                            })
                            .pSTab
                        };
                        if ((unsafe {
                            (*unsafe {
                                unsafe { std::ptr::addr_of_mut!((*pTabList).a) as *mut SrcItem }
                                    .offset(j as isize)
                            })
                            .fg
                            .__slate_bits_0
                            .__get_isSubquery()
                        }) as i32)
                            != (0 as i32)
                        {
                            pS = unsafe {
                                (*unsafe {
                                    (*unsafe {
                                        unsafe {
                                            std::ptr::addr_of_mut!((*pTabList).a) as *mut SrcItem
                                        }
                                        .offset(j as isize)
                                    })
                                    .u4
                                    .pSubq
                                })
                                .pSelect
                            };
                        } else {
                            pS = std::ptr::null_mut::<Select>();
                        }
                    } else {
                        pNC = unsafe { (*pNC).pNext };
                    }
                }
                if pTab == std::ptr::null_mut::<Table>() {
                    // /* At one time, code such as "SELECT new.x" within a trigger would
                    //         ** cause this condition to run.  Since then, we have restructured how
                    //         ** trigger code is generated and so this condition is no longer
                    //         ** possible. However, it can still be true for statements like
                    //         ** the following:
                    //         **
                    //         **   CREATE TABLE t1(col INTEGER);
                    //         **   SELECT (SELECT t1.col) FROM FROM t1;
                    //         **
                    //         ** when columnType() is called on the expression "t1.col" in the
                    //         ** sub-select. In this case, set the column type to NULL, even
                    //         ** though it should really be "INTEGER".
                    //         **
                    //         ** This is not a problem, as the column type of "t1.col" is never
                    //         ** used. When columnType() is called on the expression
                    //         ** "(SELECT t1.col)", the correct type is returned (see the TK_SELECT
                    //         ** branch below.  */
                } else {
                    0 as i32;
                    if pS != std::ptr::null_mut::<Select>() {
                        // /* The "table" is actually a sub-select or a view in the FROM clause
                        //         ** of the SELECT statement. Return the declaration type and origin
                        //         ** data for the result-set column of the sub-select.
                        //         */
                        if iCol < unsafe { (*unsafe { (*pS).pEList }).nExpr }
                            && (!((0 as i32) != (0 as i32)) || iCol >= (0 as i32))
                        {
                            // /* If iCol is less than zero, then the expression requests the
                            //           ** rowid of the sub-select or view. This expression is legal (see
                            //           ** test case misc2.2.2) - it always evaluates to NULL.
                            //           */
                            let mut sNC: NameContext = unsafe { std::mem::zeroed() };
                            let mut p: *mut Expr = unsafe {
                                (*unsafe {
                                    unsafe {
                                        std::ptr::addr_of_mut!((*unsafe { (*pS).pEList }).a)
                                            as *mut ExprList_item
                                    }
                                    .offset(iCol as isize)
                                })
                                .pExpr
                            };
                            sNC.pSrcList = unsafe { (*pS).pSrc };
                            sNC.pNext = pNC;
                            sNC.pParse = unsafe { (*pNC).pParse };
                            zType = columnTypeImpl(std::ptr::addr_of_mut!(sNC), p);
                        }
                    } else {
                        // /* A real table or a CTE table */
                        0 as i32;
                        0 as i32;
                        if iCol < (0 as i32) {
                            zType = (b"INTEGER\0".as_ptr() as *mut i8) as *const i8;
                        } else {
                            zType = (unsafe {
                                sqlite3ColumnType(
                                    unsafe { unsafe { (*pTab).aCol }.offset(iCol as isize) },
                                    std::ptr::null_mut::<i8>(),
                                )
                            }) as *const i8;
                        }
                    }
                }
            }
            139 => {
                // /* The expression is a sub-select. Return the declaration type and
                //       ** origin info for the single column in the result set of the SELECT
                //       ** statement.
                //       */
                let mut sNC: NameContext = unsafe { std::mem::zeroed() };
                let mut pS: *mut Select = unsafe { std::mem::zeroed() };
                let mut p: *mut Expr = unsafe { std::mem::zeroed() };
                0 as i32;
                pS = unsafe { (*pExpr).x.pSelect };
                p = unsafe {
                    (*unsafe {
                        unsafe {
                            std::ptr::addr_of_mut!((*unsafe { (*pS).pEList }).a)
                                as *mut ExprList_item
                        }
                        .offset((0 as i32) as isize)
                    })
                    .pExpr
                };
                sNC.pSrcList = unsafe { (*pS).pSrc };
                sNC.pNext = pNC;
                sNC.pParse = unsafe { (*pNC).pParse };
                zType = columnTypeImpl(std::ptr::addr_of_mut!(sNC), p);
            }
            _ => {}
        }
    }
    return zType;
}

// /*
// ** Generate code that will tell the VDBE the declaration types of columns
// ** in the result set.
// */
fn generateColumnTypes(
    mut pParse: *mut Parse,
    mut pTabList: *mut SrcList,
    mut pEList: *mut ExprList,
) {
    let mut v: *mut Vdbe = unsafe { (*pParse).pVdbe };
    let mut i: i32 = 0 as i32;
    let mut sNC: NameContext = unsafe { std::mem::zeroed() };
    sNC.pSrcList = pTabList;
    sNC.pParse = pParse;
    sNC.pNext = std::ptr::null_mut::<NameContext>();
    i = 0 as i32;
    '__slate_break_2044: loop {
        if !(i < unsafe { (*pEList).nExpr }) {
            break;
        }
        let mut p: *mut Expr = unsafe {
            (*unsafe {
                unsafe { std::ptr::addr_of_mut!((*pEList).a) as *mut ExprList_item }
                    .offset(i as isize)
            })
            .pExpr
        };
        let mut zType: *const i8 = unsafe { std::mem::zeroed() };
        zType = columnTypeImpl(std::ptr::addr_of_mut!(sNC), p);
        unsafe {
            sqlite3VdbeSetColName(v, i, 1 as i32, zType, unsafe {
                std::mem::transmute::<usize, Option<unsafe extern "C-unwind" fn(*mut ())>>(
                    -(1 as i32) as usize,
                )
            })
        };
        let __v2574: i32 = i;
        let __v2575: i32 = __v2574 + (1 as i32);
        i = __v2575;
    }
}

// /*
// ** Compute the iLimit and iOffset fields of the SELECT based on the
// ** pLimit expressions.  pLimit->pLeft and pLimit->pRight hold the expressions
// ** that appear in the original SQL statement after the LIMIT and OFFSET
// ** keywords.  Or NULL if those keywords are omitted. iLimit and iOffset
// ** are the integer memory register numbers for counters used to compute
// ** the limit and offset.  If there is no limit and/or offset, then
// ** iLimit and iOffset are negative.
// **
// ** This routine changes the values of iLimit and iOffset only if
// ** a limit or offset is defined by pLimit->pLeft and pLimit->pRight.  iLimit
// ** and iOffset should have been preset to appropriate default values (zero)
// ** prior to calling this routine.
// **
// ** The iOffset register (if it exists) is initialized to the value
// ** of the OFFSET.  The iLimit register is initialized to LIMIT.  Register
// ** iOffset+1 is initialized to LIMIT+OFFSET.
// **
// ** Only if pLimit->pLeft!=0 do the limit registers get
// ** redefined.  The UNION ALL operator uses this property to force
// ** the reuse of the same limit and offset registers across multiple
// ** SELECT statements.
// */
fn computeLimitRegisters(mut pParse: *mut Parse, mut p: *mut Select, mut iBreak: i32) {
    let mut v: *mut Vdbe = std::ptr::null_mut::<Vdbe>();
    let mut iLimit: i32 = 0 as i32;
    let mut iOffset: i32 = 0 as i32;
    let mut n: i32 = 0 as i32;
    let mut pLimit: *mut Expr = unsafe { (*p).pLimit };
    if (unsafe { (*p).iLimit }) != (0 as i32) {
        return;
    }
    // /*
    //   ** "LIMIT -1" always shows all rows.  There is some
    //   ** controversy about what the correct behavior should be.
    //   ** The current implementation interprets "LIMIT 0" to mean
    //   ** no rows.
    //   */
    if pLimit != std::ptr::null_mut::<Expr>() {
        0 as i32;
        0 as i32;
        let __v2576: *mut Parse = pParse;
        let __v2577: i32 = unsafe { (*__v2576).nMem };
        let __v2578: i32 = __v2577 + (1 as i32);
        unsafe {
            (*__v2576).nMem = __v2578;
        }
        let __v2579: i32 = __v2578;
        iLimit = __v2579;
        unsafe {
            (*p).iLimit = __v2579;
        }
        v = sqlite3GetVdbe(pParse);
        0 as i32;
        if (unsafe {
            sqlite3ExprIsInteger(
                (unsafe { (*pLimit).pLeft }) as *const Expr,
                std::ptr::addr_of_mut!(n),
                pParse,
                0 as i32,
            )
        }) != (0 as i32)
        {
            unsafe { sqlite3VdbeAddOp2(v, 73 as i32, n, iLimit) };
            {}
            if n == (0 as i32) {
                unsafe { sqlite3VdbeGoto(v, iBreak) };
            } else {
                let __v2580: bool;
                if n >= (0 as i32) {
                    __v2580 = ((unsafe { (*p).nSelectRow }) as i32)
                        > ((unsafe { sqlite3LogEst((n as i64) as u64) }) as i32);
                } else {
                    __v2580 = false as bool;
                }
                if __v2580 {
                    unsafe {
                        (*p).nSelectRow = unsafe { sqlite3LogEst((n as i64) as u64) };
                    }
                    let __v2581: *mut Select = p;
                    let __v2582: u32 = unsafe { (*__v2581).selFlags };
                    let __v2583: u32 = __v2582 | ((16384 as i32) as u32);
                    unsafe {
                        (*__v2581).selFlags = __v2583;
                    }
                }
            }
        } else {
            unsafe { sqlite3ExprCode(pParse, unsafe { (*pLimit).pLeft }, iLimit) };
            unsafe { sqlite3VdbeAddOp1(v, 13 as i32, iLimit) };
            {}
            {}
            unsafe { sqlite3VdbeAddOp2(v, 17 as i32, iLimit, iBreak) };
            {}
        }
        if (unsafe { (*pLimit).pRight }) != std::ptr::null_mut::<Expr>() {
            let __v2584: *mut Parse = pParse;
            let __v2585: i32 = unsafe { (*__v2584).nMem };
            let __v2586: i32 = __v2585 + (1 as i32);
            unsafe {
                (*__v2584).nMem = __v2586;
            }
            let __v2587: i32 = __v2586;
            iOffset = __v2587;
            unsafe {
                (*p).iOffset = __v2587;
            }
            // /* Allocate an extra register for limit+offset */
            let __v2588: *mut Parse = pParse;
            let __v2589: i32 = unsafe { (*__v2588).nMem };
            let __v2590: i32 = __v2589 + (1 as i32);
            unsafe {
                (*__v2588).nMem = __v2590;
            }
            unsafe { sqlite3ExprCode(pParse, unsafe { (*pLimit).pRight }, iOffset) };
            unsafe { sqlite3VdbeAddOp1(v, 13 as i32, iOffset) };
            {}
            {}
            unsafe { sqlite3VdbeAddOp3(v, 162 as i32, iLimit, iOffset + (1 as i32), iOffset) };
            {}
        }
    }
}

// /*
// ** Return the appropriate collating sequence for the iCol-th column of
// ** the result set for the compound-select statement "p".  Return NULL if
// ** the column has no default collating sequence.
// **
// ** The collating sequence for the compound select is taken from the
// ** left-most term of the select that has a collating sequence.
// */
fn multiSelectCollSeq(mut pParse: *mut Parse, mut p: *mut Select, mut iCol: i32) -> *mut CollSeq {
    let mut pRet: *mut CollSeq = unsafe { std::mem::zeroed() };
    if (unsafe { (*p).pPrior }) != std::ptr::null_mut::<Select>() {
        pRet = multiSelectCollSeq(pParse, unsafe { (*p).pPrior }, iCol);
    } else {
        pRet = std::ptr::null_mut::<CollSeq>();
    }
    0 as i32;
    // /* iCol must be less than p->pEList->nExpr.  Otherwise an error would
    //   ** have been thrown during name resolution and we would not have gotten
    //   ** this far */
    if pRet == std::ptr::null_mut::<CollSeq>() && iCol < unsafe { (*unsafe { (*p).pEList }).nExpr }
    {
        pRet = unsafe {
            sqlite3ExprCollSeq(
                pParse,
                (unsafe {
                    (*unsafe {
                        unsafe {
                            std::ptr::addr_of_mut!((*unsafe { (*p).pEList }).a)
                                as *mut ExprList_item
                        }
                        .offset(iCol as isize)
                    })
                    .pExpr
                }) as *const Expr,
            )
        };
    }
    return pRet;
}

// /*
// ** The select statement passed as the second parameter is a compound SELECT
// ** with an ORDER BY clause. This function allocates and returns a KeyInfo
// ** structure suitable for implementing the ORDER BY.
// **
// ** Space to hold the KeyInfo structure is obtained from malloc. The calling
// ** function is responsible for ensuring that this structure is eventually
// ** freed.
// */
fn multiSelectByMergeKeyInfo(
    mut pParse: *mut Parse,
    mut p: *mut Select,
    mut nExtra: i32,
) -> *mut KeyInfo {
    let mut pOrderBy: *mut ExprList = unsafe { (*p).pOrderBy };
    let mut nOrderBy: i32 = if pOrderBy != std::ptr::null_mut::<ExprList>() {
        unsafe { (*pOrderBy).nExpr }
    } else {
        0 as i32
    };
    let mut db: *mut sqlite3 = unsafe { (*pParse).db };
    let mut pRet: *mut KeyInfo = sqlite3KeyInfoAlloc(db, nOrderBy + nExtra, 1 as i32);
    if pRet != std::ptr::null_mut::<KeyInfo>() {
        let mut i: i32 = 0 as i32;
        i = 0 as i32;
        '__slate_break_2066: loop {
            if !(i < nOrderBy) {
                break;
            }
            let mut pItem: *mut ExprList_item = unsafe {
                unsafe { std::ptr::addr_of_mut!((*pOrderBy).a) as *mut ExprList_item }
                    .offset(i as isize)
            };
            let mut pTerm: *mut Expr = unsafe { (*pItem).pExpr };
            let mut pColl: *mut CollSeq = unsafe { std::mem::zeroed() };
            if (unsafe { (*pTerm).flags }) & ((512 as i32) as u32) != (0 as u32) {
                pColl = unsafe { sqlite3ExprCollSeq(pParse, pTerm as *const Expr) };
            } else {
                pColl = multiSelectCollSeq(
                    pParse,
                    p,
                    (((unsafe { (*pItem).u.x.iOrderByCol }) as u32) as i32) - (1 as i32),
                );
                if pColl == std::ptr::null_mut::<CollSeq>() {
                    pColl = unsafe { (*db).pDfltColl };
                }
                unsafe {
                    (*unsafe {
                        unsafe { std::ptr::addr_of_mut!((*pOrderBy).a) as *mut ExprList_item }
                            .offset(i as isize)
                    })
                    .pExpr = unsafe {
                        sqlite3ExprAddCollateString(
                            pParse as *const Parse,
                            pTerm,
                            (unsafe { (*pColl).zName }) as *const i8,
                        )
                    };
                }
            }
            0 as i32;
            unsafe {
                *unsafe {
                    unsafe { std::ptr::addr_of_mut!((*pRet).aColl) as *mut *mut CollSeq }
                        .offset(i as isize)
                } = pColl;
            }
            unsafe {
                *unsafe { unsafe { (*pRet).aSortFlags }.offset(i as isize) } = unsafe {
                    (*unsafe {
                        unsafe { std::ptr::addr_of_mut!((*pOrderBy).a) as *mut ExprList_item }
                            .offset(i as isize)
                    })
                    .fg
                    .sortFlags
                };
            }
            let __v2591: i32 = i;
            let __v2592: i32 = __v2591 + (1 as i32);
            i = __v2592;
        }
    }
    return pRet;
}

// /*
// ** This routine generates VDBE code to compute the content of a WITH RECURSIVE
// ** query of the form:
// **
// **   <recursive-table> AS (<setup-query> UNION [ALL] <recursive-query>)
// **                         \___________/             \_______________/
// **                           p->pPrior                      p
// **
// **
// ** There is exactly one reference to the recursive-table in the FROM clause
// ** of recursive-query, marked with the SrcList->a[].fg.isRecursive flag.
// **
// ** The setup-query runs once to generate an initial set of rows that go
// ** into a Queue table.  Rows are extracted from the Queue table one by
// ** one.  Each row extracted from Queue is output to pDest.  Then the single
// ** extracted row (now in the iCurrent table) becomes the content of the
// ** recursive-table for a recursive-query run.  The output of the recursive-query
// ** is added back into the Queue table.  Then another row is extracted from Queue
// ** and the iteration continues until the Queue table is empty.
// **
// ** If the compound query operator is UNION then no duplicate rows are ever
// ** inserted into the Queue table.  The iDistinct table keeps a copy of all rows
// ** that have ever been inserted into Queue and causes duplicates to be
// ** discarded.  If the operator is UNION ALL, then duplicates are allowed.
// **
// ** If the query has an ORDER BY, then entries in the Queue table are kept in
// ** ORDER BY order and the first entry is extracted for each cycle.  Without
// ** an ORDER BY, the Queue table is just a FIFO.
// **
// ** If a LIMIT clause is provided, then the iteration stops after LIMIT rows
// ** have been output to pDest.  A LIMIT of zero means to output no rows and a
// ** negative LIMIT means to output all rows.  If there is also an OFFSET clause
// ** with a positive value, then the first OFFSET outputs are discarded rather
// ** than being sent to pDest.  The LIMIT count does not begin until after OFFSET
// ** rows have been skipped.
// */
fn generateWithRecursiveQuery(
    mut pParse: *mut Parse,
    mut p: *mut Select,
    mut pDest: *mut SelectDest,
) {
    let mut __slate_storage_2608: std::mem::MaybeUninit<*mut *mut CollSeq> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_2608: *mut *mut *mut CollSeq =
        std::ptr::addr_of_mut!(__slate_storage_2608) as *mut *mut *mut CollSeq;
    let mut __slate_storage_2607: std::mem::MaybeUninit<*mut *mut CollSeq> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_2607: *mut *mut *mut CollSeq =
        std::ptr::addr_of_mut!(__slate_storage_2607) as *mut *mut *mut CollSeq;
    let mut __slate_storage_2606: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2606: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_2606) as *mut i32;
    let mut __slate_storage_2605: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2605: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_2605) as *mut i32;
    let mut __slate_storage_2604: std::mem::MaybeUninit<*mut *mut CollSeq> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_2604: *mut *mut *mut CollSeq =
        std::ptr::addr_of_mut!(__slate_storage_2604) as *mut *mut *mut CollSeq;
    let mut __slate_storage_906: std::mem::MaybeUninit<*mut *mut CollSeq> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_906: *mut *mut *mut CollSeq =
        std::ptr::addr_of_mut!(__slate_storage_906) as *mut *mut *mut CollSeq;
    let mut __slate_storage_905: std::mem::MaybeUninit<*mut KeyInfo> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_905: *mut *mut KeyInfo =
        std::ptr::addr_of_mut!(__slate_storage_905) as *mut *mut KeyInfo;
    let mut __slate_storage_904: std::mem::MaybeUninit<*mut KeyInfo> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_904: *mut *mut KeyInfo =
        std::ptr::addr_of_mut!(__slate_storage_904) as *mut *mut KeyInfo;
    let mut __slate_storage_2603: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2603: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_2603) as *mut i32;
    let mut __slate_storage_2602: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2602: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_2602) as *mut i32;
    let mut __slate_storage_2601: std::mem::MaybeUninit<*mut Parse> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_2601: *mut *mut Parse =
        std::ptr::addr_of_mut!(__slate_storage_2601) as *mut *mut Parse;
    let mut __slate_storage_2600: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2600: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_2600) as *mut i32;
    let mut __slate_storage_2599: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2599: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_2599) as *mut i32;
    let mut __slate_storage_2598: std::mem::MaybeUninit<*mut Parse> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_2598: *mut *mut Parse =
        std::ptr::addr_of_mut!(__slate_storage_2598) as *mut *mut Parse;
    let mut __slate_storage_2597: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2597: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_2597) as *mut i32;
    let mut __slate_storage_2596: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2596: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_2596) as *mut i32;
    let mut __slate_storage_2595: std::mem::MaybeUninit<*mut Parse> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_2595: *mut *mut Parse =
        std::ptr::addr_of_mut!(__slate_storage_2595) as *mut *mut Parse;
    let mut __slate_storage_2594: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2594: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_2594) as *mut i32;
    let mut __slate_storage_2593: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2593: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_2593) as *mut i32;
    let mut __slate_storage_903: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_903: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_903) as *mut i32;
    let mut __slate_storage_902: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_902: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_902) as *mut i32;
    let mut __slate_storage_901: std::mem::MaybeUninit<*mut Expr> = std::mem::MaybeUninit::uninit();
    let __slate_slot_901: *mut *mut Expr =
        std::ptr::addr_of_mut!(__slate_storage_901) as *mut *mut Expr;
    let mut __slate_storage_900: std::mem::MaybeUninit<*mut ExprList> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_900: *mut *mut ExprList =
        std::ptr::addr_of_mut!(__slate_storage_900) as *mut *mut ExprList;
    let mut __slate_storage_899: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_899: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_899) as *mut i32;
    let mut __slate_storage_898: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_898: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_898) as *mut i32;
    let mut __slate_storage_897: std::mem::MaybeUninit<SelectDest> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_897: *mut SelectDest =
        std::ptr::addr_of_mut!(__slate_storage_897) as *mut SelectDest;
    let mut __slate_storage_896: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_896: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_896) as *mut i32;
    let mut __slate_storage_895: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_895: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_895) as *mut i32;
    let mut __slate_storage_894: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_894: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_894) as *mut i32;
    let mut __slate_storage_893: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_893: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_893) as *mut i32;
    let mut __slate_storage_892: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_892: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_892) as *mut i32;
    let mut __slate_storage_891: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_891: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_891) as *mut i32;
    let mut __slate_storage_890: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_890: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_890) as *mut i32;
    let mut __slate_storage_889: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_889: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_889) as *mut i32;
    let mut __slate_storage_888: std::mem::MaybeUninit<*mut Select> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_888: *mut *mut Select =
        std::ptr::addr_of_mut!(__slate_storage_888) as *mut *mut Select;
    let mut __slate_storage_887: std::mem::MaybeUninit<*mut Select> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_887: *mut *mut Select =
        std::ptr::addr_of_mut!(__slate_storage_887) as *mut *mut Select;
    let mut __slate_storage_886: std::mem::MaybeUninit<*mut Vdbe> = std::mem::MaybeUninit::uninit();
    let __slate_slot_886: *mut *mut Vdbe =
        std::ptr::addr_of_mut!(__slate_storage_886) as *mut *mut Vdbe;
    let mut __slate_storage_885: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_885: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_885) as *mut i32;
    let mut __slate_storage_884: std::mem::MaybeUninit<*mut SrcList> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_884: *mut *mut SrcList =
        std::ptr::addr_of_mut!(__slate_storage_884) as *mut *mut SrcList;
    unsafe {
        // /* The FROM clause of the recursive query */
        std::ptr::write(__slate_slot_884, unsafe { (*p).pSrc });
        // /* Number of columns in the recursive table */
        std::ptr::write(__slate_slot_885, unsafe { (*unsafe { (*p).pEList }).nExpr });
        // /* The prepared statement under construction */
        std::ptr::write(__slate_slot_886, unsafe { (*pParse).pVdbe });
        // /* The setup query */
        // /* Left-most recursive term */
        // /* Top of the loop */
        // /* CONTINUE and BREAK addresses */
        // /* The Current table */
        std::ptr::write(__slate_slot_892, 0 as i32);
        // /* Register holding Current table */
        // /* The Queue table */
        // /* To ensure unique results if UNION */
        std::ptr::write(__slate_slot_895, 0 as i32);
        // /* How to write to Queue */
        std::ptr::write(__slate_slot_896, 6 as i32);
        // /* SelectDest targeting the Queue table */
        // /* Loop counter */
        // /* Result code */
        // /* The ORDER BY clause */
        // /* Saved LIMIT and OFFSET */
        // /* Registers used by LIMIT and OFFSET */
        if (unsafe { (*p).pWin }) != std::ptr::null_mut::<Window>() {
            unsafe {
                sqlite3ErrorMsg(
                    pParse,
                    (b"cannot use window functions in recursive queries\0".as_ptr() as *mut i8)
                        as *const i8,
                )
            };
            return;
        } else {
            // /* Obtain authorization to do a recursive query */
            if (unsafe {
                sqlite3AuthCheck(
                    pParse,
                    33 as i32,
                    std::ptr::null::<i8>(),
                    std::ptr::null::<i8>(),
                    std::ptr::null::<i8>(),
                )
            }) != (0 as i32)
            {
                return;
            } else {
                // /* Process the LIMIT and OFFSET clauses, if they exist */
                *__slate_slot_891 = unsafe { sqlite3VdbeMakeLabel(pParse) };
                // /* 4 billion rows */
                unsafe {
                    (*p).nSelectRow = (320 as i32) as i16;
                }
                computeLimitRegisters(pParse, p, *__slate_slot_891);
                *__slate_slot_901 = unsafe { (*p).pLimit };
                *__slate_slot_902 = unsafe { (*p).iLimit };
                *__slate_slot_903 = unsafe { (*p).iOffset };
                unsafe {
                    (*p).pLimit = std::ptr::null_mut::<Expr>();
                }
                unsafe {
                    (*p).iOffset = 0 as i32;
                }
                unsafe {
                    (*p).iLimit = 0 as i32;
                }
                *__slate_slot_900 = unsafe { (*p).pOrderBy };
                // /* Locate the cursor number of the Current table */
                *__slate_slot_898 = 0 as i32;
                '__join_28: {
                    loop {
                        if *__slate_slot_898 < unsafe { (*(*__slate_slot_884)).nSrc } {
                            if ((unsafe {
                                (*unsafe {
                                    unsafe {
                                        std::ptr::addr_of_mut!((*(*__slate_slot_884)).a)
                                            as *mut SrcItem
                                    }
                                    .offset(*__slate_slot_898 as isize)
                                })
                                .fg
                                .__slate_bits_0
                                .__get_isRecursive()
                            }) as i32)
                                != (0 as i32)
                            {
                                break;
                            } else {
                                std::ptr::write(__slate_slot_2593, *__slate_slot_898);
                                std::ptr::write(__slate_slot_2594, *__slate_slot_2593 + (1 as i32));
                                *__slate_slot_898 = *__slate_slot_2594;
                            }
                        } else {
                            break '__join_28;
                        }
                    }
                    *__slate_slot_892 = unsafe {
                        (*unsafe {
                            unsafe {
                                std::ptr::addr_of_mut!((*(*__slate_slot_884)).a) as *mut SrcItem
                            }
                            .offset(*__slate_slot_898 as isize)
                        })
                        .iCursor
                    };
                }
                // /* Allocate cursors numbers for Queue and Distinct.  The cursor number for
                //   ** the Distinct table must be exactly one greater than Queue in order
                //   ** for the SRT_DistFifo and SRT_DistQueue destinations to work. */
                std::ptr::write(__slate_slot_2595, pParse);
                std::ptr::write(__slate_slot_2596, unsafe { (*(*__slate_slot_2595)).nTab });
                std::ptr::write(__slate_slot_2597, *__slate_slot_2596 + (1 as i32));
                unsafe {
                    (*(*__slate_slot_2595)).nTab = *__slate_slot_2597;
                }
                *__slate_slot_894 = *__slate_slot_2596;
                if (((unsafe { (*p).op }) as u32) as i32) == (135 as i32) {
                    *__slate_slot_896 = if *__slate_slot_900 != std::ptr::null_mut::<ExprList>() {
                        4 as i32
                    } else {
                        3 as i32
                    };
                    std::ptr::write(__slate_slot_2598, pParse);
                    std::ptr::write(__slate_slot_2599, unsafe { (*(*__slate_slot_2598)).nTab });
                    std::ptr::write(__slate_slot_2600, *__slate_slot_2599 + (1 as i32));
                    unsafe {
                        (*(*__slate_slot_2598)).nTab = *__slate_slot_2600;
                    }
                    *__slate_slot_895 = *__slate_slot_2599;
                } else {
                    *__slate_slot_896 = if *__slate_slot_900 != std::ptr::null_mut::<ExprList>() {
                        5 as i32
                    } else {
                        6 as i32
                    };
                }
                sqlite3SelectDestInit(
                    std::ptr::addr_of_mut!(*__slate_slot_897),
                    *__slate_slot_896,
                    *__slate_slot_894,
                );
                // /* Allocate cursors for Current, Queue, and Distinct. */
                std::ptr::write(__slate_slot_2601, pParse);
                std::ptr::write(__slate_slot_2602, unsafe { (*(*__slate_slot_2601)).nMem });
                std::ptr::write(__slate_slot_2603, *__slate_slot_2602 + (1 as i32));
                unsafe {
                    (*(*__slate_slot_2601)).nMem = *__slate_slot_2603;
                }
                *__slate_slot_893 = *__slate_slot_2603;
                unsafe {
                    sqlite3VdbeAddOp3(
                        *__slate_slot_886,
                        123 as i32,
                        *__slate_slot_892,
                        *__slate_slot_893,
                        *__slate_slot_885,
                    )
                };
                if *__slate_slot_900 != std::ptr::null_mut::<ExprList>() {
                    std::ptr::write(
                        __slate_slot_904,
                        multiSelectByMergeKeyInfo(pParse, p, 1 as i32),
                    );
                    unsafe {
                        sqlite3VdbeAddOp4(
                            *__slate_slot_886,
                            120 as i32,
                            *__slate_slot_894,
                            (unsafe { (*(*__slate_slot_900)).nExpr }) + (2 as i32),
                            0 as i32,
                            (*__slate_slot_904 as *mut i8) as *const i8,
                            -(9 as i32),
                        )
                    };
                    (*__slate_slot_897).pOrderBy = *__slate_slot_900;
                } else {
                    unsafe {
                        sqlite3VdbeAddOp2(
                            *__slate_slot_886,
                            120 as i32,
                            *__slate_slot_894,
                            *__slate_slot_885,
                        )
                    };
                }
                {}
                if *__slate_slot_895 != (0 as i32) {
                    // /* Generate an ephemeral table used to enforce distinctness on the
                    //     ** output of the recursive part of the CTE.
                    //     */
                    // /* Collating sequence for the result set */
                    // /* For looping through pKeyInfo->aColl[] */
                    0 as i32;
                    0 as i32;
                    *__slate_slot_885 = unsafe { (*unsafe { (*p).pEList }).nExpr };
                    *__slate_slot_905 =
                        sqlite3KeyInfoAlloc(unsafe { (*pParse).db }, *__slate_slot_885, 1 as i32);
                    if *__slate_slot_905 != std::ptr::null_mut::<KeyInfo>() {
                        *__slate_slot_898 = 0 as i32;
                        std::ptr::write(__slate_slot_2604, unsafe {
                            std::ptr::addr_of_mut!((*(*__slate_slot_905)).aColl)
                                as *mut *mut CollSeq
                        });
                        *__slate_slot_906 = *__slate_slot_2604;
                        loop {
                            if *__slate_slot_898 < *__slate_slot_885 {
                                unsafe {
                                    *(*__slate_slot_906) =
                                        multiSelectCollSeq(pParse, p, *__slate_slot_898);
                                }
                                if std::ptr::null_mut::<CollSeq>()
                                    == unsafe { *(*__slate_slot_906) }
                                {
                                    unsafe {
                                        *(*__slate_slot_906) =
                                            unsafe { (*unsafe { (*pParse).db }).pDfltColl };
                                    }
                                }
                                std::ptr::write(__slate_slot_2605, *__slate_slot_898);
                                std::ptr::write(__slate_slot_2606, *__slate_slot_2605 + (1 as i32));
                                *__slate_slot_898 = *__slate_slot_2606;
                                std::ptr::write(__slate_slot_2607, *__slate_slot_906);
                                std::ptr::write(__slate_slot_2608, unsafe {
                                    (*__slate_slot_2607).offset((1 as i32) as isize)
                                });
                                *__slate_slot_906 = *__slate_slot_2608;
                            } else {
                                break;
                            }
                        }
                        unsafe {
                            sqlite3VdbeAddOp4(
                                *__slate_slot_886,
                                120 as i32,
                                *__slate_slot_895,
                                *__slate_slot_885,
                                0 as i32,
                                (*__slate_slot_905 as *mut ()) as *const i8,
                                -(9 as i32),
                            )
                        };
                    } else {
                        0 as i32;
                    }
                }
                // /* Detach the ORDER BY clause from the compound SELECT */
                unsafe {
                    (*p).pOrderBy = std::ptr::null_mut::<ExprList>();
                }
                // /* Figure out how many elements of the compound SELECT are part of the
                //   ** recursive query.  Make sure no recursive elements use aggregate
                //   ** functions.  Mark the recursive elements as UNION ALL even if they
                //   ** are really UNION because the distinctness will be enforced by the
                //   ** iDistinct table.  pFirstRec is left pointing to the left-most
                //   ** recursive term of the CTE.
                //   */
                *__slate_slot_888 = p;
                '__join_0: {
                    '__join_11: {
                        loop {
                            if *__slate_slot_888 != std::ptr::null_mut::<Select>() {
                                if (unsafe { (*(*__slate_slot_888)).selFlags })
                                    & ((8 as i32) as u32)
                                    != (0 as u32)
                                {
                                    break '__join_11;
                                } else {
                                    unsafe {
                                        (*(*__slate_slot_888)).op = ((136 as i32) as i8) as u8;
                                    }
                                    if (unsafe {
                                        (*unsafe { (*(*__slate_slot_888)).pPrior }).selFlags
                                    }) & ((8192 as i32) as u32)
                                        == ((0 as i32) as u32)
                                    {
                                        break;
                                    } else {
                                        *__slate_slot_888 =
                                            unsafe { (*(*__slate_slot_888)).pPrior };
                                    }
                                }
                            } else {
                                break;
                            }
                        }
                        // /* Store the results of the setup-query in Queue. */
                        *__slate_slot_887 = unsafe { (*(*__slate_slot_888)).pPrior };
                        unsafe {
                            (*(*__slate_slot_887)).pNext = std::ptr::null_mut::<Select>();
                        }
                        unsafe {
                            sqlite3VdbeExplain(
                                pParse,
                                ((1 as i32) as i8) as u8,
                                (b"SETUP\0".as_ptr() as *mut i8) as *const i8,
                            )
                        };
                        *__slate_slot_899 = sqlite3Select(
                            pParse,
                            *__slate_slot_887,
                            std::ptr::addr_of_mut!(*__slate_slot_897),
                        );
                        unsafe {
                            (*(*__slate_slot_887)).pNext = p;
                        }
                        if *__slate_slot_899 != (0 as i32) {
                            break '__join_0;
                        } else {
                            // /* Find the next row in the Queue and output that row */
                            *__slate_slot_889 = unsafe {
                                sqlite3VdbeAddOp2(
                                    *__slate_slot_886,
                                    36 as i32,
                                    *__slate_slot_894,
                                    *__slate_slot_891,
                                )
                            };
                            {}
                            // /* Transfer the next row in Queue over to Current */
                            // /* To reset column cache */
                            unsafe {
                                sqlite3VdbeAddOp1(*__slate_slot_886, 138 as i32, *__slate_slot_892)
                            };
                            if *__slate_slot_900 != std::ptr::null_mut::<ExprList>() {
                                unsafe {
                                    sqlite3VdbeAddOp3(
                                        *__slate_slot_886,
                                        96 as i32,
                                        *__slate_slot_894,
                                        (unsafe { (*(*__slate_slot_900)).nExpr }) + (1 as i32),
                                        *__slate_slot_893,
                                    )
                                };
                            } else {
                                unsafe {
                                    sqlite3VdbeAddOp2(
                                        *__slate_slot_886,
                                        136 as i32,
                                        *__slate_slot_894,
                                        *__slate_slot_893,
                                    )
                                };
                            }
                            unsafe {
                                sqlite3VdbeAddOp1(*__slate_slot_886, 132 as i32, *__slate_slot_894)
                            };
                            // /* Output the single row in Current */
                            *__slate_slot_890 = unsafe { sqlite3VdbeMakeLabel(pParse) };
                            codeOffset(*__slate_slot_886, *__slate_slot_903, *__slate_slot_890);
                            selectInnerLoop(
                                pParse,
                                p,
                                *__slate_slot_892,
                                std::ptr::null_mut::<SortCtx>(),
                                std::ptr::null_mut::<DistinctCtx>(),
                                pDest,
                                *__slate_slot_890,
                                *__slate_slot_891,
                            );
                            if *__slate_slot_902 != (0 as i32) {
                                unsafe {
                                    sqlite3VdbeAddOp2(
                                        *__slate_slot_886,
                                        63 as i32,
                                        *__slate_slot_902,
                                        *__slate_slot_891,
                                    )
                                };
                                {}
                            }
                            unsafe {
                                sqlite3VdbeResolveLabel(*__slate_slot_886, *__slate_slot_890)
                            };
                            // /* Execute the recursive SELECT taking the single row in Current as
                            //   ** the value for the recursive-table. Store the results in the Queue.
                            //   */
                            unsafe {
                                (*(*__slate_slot_888)).pPrior = std::ptr::null_mut::<Select>();
                            }
                            unsafe {
                                sqlite3VdbeExplain(
                                    pParse,
                                    ((1 as i32) as i8) as u8,
                                    (b"RECURSIVE STEP\0".as_ptr() as *mut i8) as *const i8,
                                )
                            };
                            sqlite3Select(pParse, p, std::ptr::addr_of_mut!(*__slate_slot_897));
                            0 as i32;
                            unsafe {
                                (*(*__slate_slot_888)).pPrior = *__slate_slot_887;
                            }
                            // /* Keep running the loop until the Queue is empty */
                            unsafe { sqlite3VdbeGoto(*__slate_slot_886, *__slate_slot_889) };
                            unsafe {
                                sqlite3VdbeResolveLabel(*__slate_slot_886, *__slate_slot_891)
                            };
                            break '__join_0;
                        }
                    }
                    unsafe {
                        sqlite3ErrorMsg(
                            pParse,
                            (b"recursive aggregate queries not supported\0".as_ptr() as *mut i8)
                                as *const i8,
                        )
                    };
                }
                unsafe { sqlite3ExprListDelete(unsafe { (*pParse).db }, unsafe { (*p).pOrderBy }) };
                unsafe {
                    (*p).pOrderBy = *__slate_slot_900;
                }
                unsafe {
                    (*p).pLimit = *__slate_slot_901;
                }
                return;
            }
        }
    }
}

// /* Parsing context */
// /* The SELECT statement */
// /* Coroutine supplying data */
// /* Where to send the data */
// /* The return address register */
// /* Previous result register.  No uniqueness if 0 */
// /* For comparing with previous entry */
// /* Jump here if we hit the LIMIT */
// /*
// ** Generate code for a compound SELECT statement using a merge
// ** algorithm.  The compound must have an ORDER BY clause for this
// ** to work.
// **
// ** We assume a query of the following form:
// **
// **      <selectA>  <operator>  <selectB>  ORDER BY <orderbylist>
// **
// ** <operator> is one of UNION ALL, UNION, EXCEPT, or INTERSECT.  The idea
// ** is to code both <selectA> and <selectB> with the ORDER BY clause as
// ** co-routines.  Then run the co-routines in parallel and merge the results
// ** into the output.  In addition to the two coroutines (called selectA and
// ** selectB) there are 7 subroutines:
// **
// **    outA:    Move the output of the selectA coroutine into the output
// **             of the compound query.
// **
// **    outB:    Move the output of the selectB coroutine into the output
// **             of the compound query.  (Only generated for UNION and
// **             UNION ALL.  EXCEPT and INTERSECT never output a row that
// **             appears only in B.)
// **
// **    AltB:    Called when there is data from both coroutines and A<B.
// **
// **    AeqB:    Called when there is data from both coroutines and A==B.
// **
// **    AgtB:    Called when there is data from both coroutines and A>B.
// **
// **    EofA:    Called when data is exhausted from selectA.
// **
// **    EofB:    Called when data is exhausted from selectB.
// **
// ** The implementation of the latter five subroutines depend on which
// ** <operator> is used:
// **
// **
// **             UNION ALL         UNION            EXCEPT          INTERSECT
// **          -------------  -----------------  --------------  -----------------
// **   AltB:   outA, nextA      outA, nextA       outA, nextA         nextA
// **
// **   AeqB:   outA, nextA         nextA             nextA         outA, nextA
// **
// **   AgtB:   outB, nextB      outB, nextB          nextB            nextB
// **
// **   EofA:   outB, nextB      outB, nextB          halt             halt
// **
// **   EofB:   outA, nextA      outA, nextA       outA, nextA         halt
// **
// ** In the AltB, AeqB, and AgtB subroutines, an EOF on A following nextA
// ** causes an immediate jump to EofA and an EOF on B following nextB causes
// ** an immediate jump to EofB.  Within EofA and EofB, and EOF on entry or
// ** following nextX causes a jump to the end of the select processing.
// **
// ** Duplicate removal in the UNION, EXCEPT, and INTERSECT cases is handled
// ** within the output subroutine.  The regPrev register set holds the previously
// ** output value.  A comparison is made against this value and the output
// ** is skipped if the next results would be the same as the previous.
// **
// ** The implementation plan is to implement the two coroutines and seven
// ** subroutines first, then put the control logic at the bottom.  Like this:
// **
// **          goto Init
// **     coA: coroutine for left query (A)
// **     coB: coroutine for right query (B)
// **    outA: output one row of A
// **    outB: output one row of B (UNION and UNION ALL only)
// **    EofA: ...
// **    EofB: ...
// **    AltB: ...
// **    AeqB: ...
// **    AgtB: ...
// **    Init: initialize coroutine registers
// **          yield coA, on eof goto EofA
// **          yield coB, on eof goto EofB
// **    Cmpr: Compare A, B
// **          Jump AltB, AeqB, AgtB
// **     End: ...
// **
// ** We call AltB, AeqB, AgtB, EofA, and EofB "subroutines" but they are not
// ** actually called using Gosub and they do not Return.  EofA and EofB loop
// ** until all data is exhausted then jump to the "end" label.  AltB, AeqB,
// ** and AgtB jump to either Cmpr or to one of EofA or EofB.
// */
fn multiSelectByMerge(
    mut pParse: *mut Parse,
    mut p: *mut Select,
    mut pDest: *mut SelectDest,
) -> i32 {
    // /* Loop counters */
    let mut i: i32 = 0 as i32;
    let mut j: i32 = 0 as i32;
    // /* Another SELECT immediately to our left */
    let mut pPrior: *mut Select = unsafe { std::mem::zeroed() };
    // /* Left-most SELECT in the right-hand group */
    let mut pSplit: *mut Select = unsafe { std::mem::zeroed() };
    // /* Number of SELECT statements in the compound */
    let mut nSelect: i32 = 0 as i32;
    // /* Generate code to this VDBE */
    let mut v: *mut Vdbe = unsafe { std::mem::zeroed() };
    // /* Destination for coroutine A */
    let mut destA: SelectDest = unsafe { std::mem::zeroed() };
    // /* Destination for coroutine B */
    let mut destB: SelectDest = unsafe { std::mem::zeroed() };
    // /* Address register for select-A coroutine */
    let mut regAddrA: i32 = 0 as i32;
    // /* Address register for select-B coroutine */
    let mut regAddrB: i32 = 0 as i32;
    // /* Address of the select-A coroutine */
    let mut addrSelectA: i32 = 0 as i32;
    // /* Address of the select-B coroutine */
    let mut addrSelectB: i32 = 0 as i32;
    // /* Address register for the output-A subroutine */
    let mut regOutA: i32 = 0 as i32;
    // /* Address register for the output-B subroutine */
    let mut regOutB: i32 = 0 as i32;
    // /* Address of the output-A subroutine */
    let mut addrOutA: i32 = 0 as i32;
    // /* Address of the output-B subroutine */
    let mut addrOutB: i32 = 0 as i32;
    // /* Address of the select-A-exhausted subroutine */
    let mut addrEofA: i32 = 0 as i32;
    // /* Alternate addrEofA if B is uninitialized */
    let mut addrEofA_noB: i32 = 0 as i32;
    // /* Address of the select-B-exhausted subroutine */
    let mut addrEofB: i32 = 0 as i32;
    // /* Address of the A<B subroutine */
    let mut addrAltB: i32 = 0 as i32;
    // /* Address of the A==B subroutine */
    let mut addrAeqB: i32 = 0 as i32;
    // /* Address of the A>B subroutine */
    let mut addrAgtB: i32 = 0 as i32;
    // /* Limit register for select-A */
    let mut regLimitA: i32 = 0 as i32;
    // /* Limit register for select-A */
    let mut regLimitB: i32 = 0 as i32;
    // /* A range of registers to hold previous output */
    let mut regPrev: i32 = 0 as i32;
    // /* Saved value of p->iLimit */
    let mut savedLimit: i32 = 0 as i32;
    // /* Saved value of p->iOffset */
    let mut savedOffset: i32 = 0 as i32;
    // /* Label for the start of the merge algorithm */
    let mut labelCmpr: i32 = 0 as i32;
    // /* Label for the end of the overall SELECT stmt */
    let mut labelEnd: i32 = 0 as i32;
    // /* Jump instructions that get retargeted */
    let mut addr1: i32 = 0 as i32;
    // /* One of TK_ALL, TK_UNION, TK_EXCEPT, TK_INTERSECT */
    let mut op: i32 = 0 as i32;
    // /* Comparison information for duplicate removal */
    let mut pKeyDup: *mut KeyInfo = std::ptr::null_mut::<KeyInfo>();
    // /* Comparison information for merging rows */
    let mut pKeyMerge: *mut KeyInfo = unsafe { std::mem::zeroed() };
    // /* Database connection */
    let mut db: *mut sqlite3 = unsafe { std::mem::zeroed() };
    // /* The ORDER BY clause */
    let mut pOrderBy: *mut ExprList = unsafe { std::mem::zeroed() };
    // /* Number of terms in the ORDER BY clause */
    let mut nOrderBy: i32 = 0 as i32;
    // /* Mapping from ORDER BY terms to result set columns */
    let mut aPermute: *mut u32 = unsafe { std::mem::zeroed() };
    0 as i32;
    // /* "Managed" code needs this.  Ticket #3382. */
    0 as i32;
    db = unsafe { (*pParse).db };
    v = unsafe { (*pParse).pVdbe };
    // /* Already thrown the error if VDBE alloc failed */
    0 as i32;
    labelEnd = unsafe { sqlite3VdbeMakeLabel(pParse) };
    labelCmpr = unsafe { sqlite3VdbeMakeLabel(pParse) };
    // /* Patch up the ORDER BY clause
    //   */
    op = ((unsafe { (*p).op }) as u32) as i32;
    0 as i32;
    pOrderBy = unsafe { (*p).pOrderBy };
    0 as i32;
    nOrderBy = unsafe { (*pOrderBy).nExpr };
    // /* For operators other than UNION ALL we have to make sure that
    //   ** the ORDER BY clause covers every term of the result set.  Add
    //   ** terms to the ORDER BY clause as necessary.
    //   */
    if op != (136 as i32) {
        i = 1 as i32;
        '__slate_break_2091: loop {
            if !((((unsafe { (*db).mallocFailed }) as u32) as i32) == (0 as i32)
                && i <= unsafe { (*unsafe { (*p).pEList }).nExpr })
            {
                break;
            }
            let mut pItem: *mut ExprList_item = unsafe { std::mem::zeroed() };
            j = 0 as i32;
            let __v2611: *mut ExprList_item =
                unsafe { std::ptr::addr_of_mut!((*pOrderBy).a) as *mut ExprList_item };
            pItem = __v2611;
            '__slate_break_2092: while j < nOrderBy {
                0 as i32;
                0 as i32;
                if (((unsafe { (*pItem).u.x.iOrderByCol }) as u32) as i32) == i {
                    break '__slate_break_2092;
                }
                let __v2612: i32 = j;
                let __v2613: i32 = __v2612 + (1 as i32);
                j = __v2613;
                let __v2614: *mut ExprList_item = pItem;
                let __v2615: *mut ExprList_item = unsafe { __v2614.offset((1 as i32) as isize) };
                pItem = __v2615;
            }
            if j == nOrderBy {
                let mut pNew: *mut Expr = unsafe { sqlite3ExprInt32(db, i) };
                if pNew == std::ptr::null_mut::<Expr>() {
                    return 7 as i32;
                }
                let __v2616: *mut ExprList =
                    unsafe { sqlite3ExprListAppend(pParse, pOrderBy, pNew) };
                pOrderBy = __v2616;
                unsafe {
                    (*p).pOrderBy = __v2616;
                }
                if pOrderBy != std::ptr::null_mut::<ExprList>() {
                    let __v2617: i32 = nOrderBy;
                    let __v2618: i32 = __v2617 + (1 as i32);
                    nOrderBy = __v2618;
                    unsafe {
                        (*unsafe {
                            unsafe { std::ptr::addr_of_mut!((*pOrderBy).a) as *mut ExprList_item }
                                .offset(__v2617 as isize)
                        })
                        .u
                        .x
                        .iOrderByCol = (i as i16) as u16;
                    }
                }
            }
            let __v2609: i32 = i;
            let __v2610: i32 = __v2609 + (1 as i32);
            i = __v2610;
        }
    }
    // /* Compute the comparison permutation and keyinfo that is used with
    //   ** the permutation to determine if the next row of results comes
    //   ** from selectA or selectB.  Also add literal collations to the
    //   ** ORDER BY clause terms so that when selectA and selectB are
    //   ** evaluated, they use the correct collation.
    //   */
    aPermute = (unsafe {
        sqlite3DbMallocRawNN(
            db,
            (4 as u64).wrapping_mul(((nOrderBy + (1 as i32)) as i64) as u64),
        )
    }) as *mut u32;
    if aPermute != std::ptr::null_mut::<u32>() {
        let mut pItem: *mut ExprList_item = unsafe { std::mem::zeroed() };
        let mut bKeep: i32 = 0 as i32;
        unsafe {
            *unsafe { aPermute.offset((0 as i32) as isize) } = nOrderBy as u32;
        }
        i = 1 as i32;
        let __v2619: *mut ExprList_item =
            unsafe { std::ptr::addr_of_mut!((*pOrderBy).a) as *mut ExprList_item };
        pItem = __v2619;
        '__slate_break_2093: while i <= nOrderBy {
            0 as i32;
            0 as i32;
            0 as i32;
            unsafe {
                *unsafe { aPermute.offset(i as isize) } =
                    ((((unsafe { (*pItem).u.x.iOrderByCol }) as u32) as i32) - (1 as i32)) as u32;
            }
            if (unsafe { *unsafe { aPermute.offset(i as isize) } })
                != (i as u32).wrapping_sub((1 as i32) as u32)
            {
                bKeep = 1 as i32;
            }
            let __v2620: i32 = i;
            let __v2621: i32 = __v2620 + (1 as i32);
            i = __v2621;
            let __v2622: *mut ExprList_item = pItem;
            let __v2623: *mut ExprList_item = unsafe { __v2622.offset((1 as i32) as isize) };
            pItem = __v2623;
        }
        if bKeep == (0 as i32) {
            unsafe { sqlite3DbFreeNN(db, aPermute as *mut ()) };
            aPermute = std::ptr::null_mut::<u32>();
        }
    }
    pKeyMerge = multiSelectByMergeKeyInfo(pParse, p, 1 as i32);
    // /* Allocate a range of temporary registers and the KeyInfo needed
    //   ** for the logic that removes duplicate result rows when the
    //   ** operator is UNION, EXCEPT, or INTERSECT (but not UNION ALL).
    //   */
    if op == (136 as i32) {
        regPrev = 0 as i32;
    } else {
        let mut nExpr: i32 = unsafe { (*unsafe { (*p).pEList }).nExpr };
        0 as i32;
        regPrev = (unsafe { (*pParse).nMem }) + (1 as i32);
        let __v2624: *mut Parse = pParse;
        let __v2625: i32 = unsafe { (*__v2624).nMem };
        let __v2626: i32 = __v2625 + (nExpr + (1 as i32));
        unsafe {
            (*__v2624).nMem = __v2626;
        }
        unsafe { sqlite3VdbeAddOp2(v, 73 as i32, 0 as i32, regPrev) };
        pKeyDup = sqlite3KeyInfoAlloc(db, nExpr, 1 as i32);
        if pKeyDup != std::ptr::null_mut::<KeyInfo>() {
            0 as i32;
            i = 0 as i32;
            '__slate_break_2094: loop {
                if !(i < nExpr) {
                    break;
                }
                unsafe {
                    *unsafe {
                        unsafe { std::ptr::addr_of_mut!((*pKeyDup).aColl) as *mut *mut CollSeq }
                            .offset(i as isize)
                    } = multiSelectCollSeq(pParse, p, i);
                }
                unsafe {
                    *unsafe { unsafe { (*pKeyDup).aSortFlags }.offset(i as isize) } =
                        ((0 as i32) as i8) as u8;
                }
                let __v2627: i32 = i;
                let __v2628: i32 = __v2627 + (1 as i32);
                i = __v2628;
            }
        }
    }
    // /* Separate the left and the right query from one another
    //   */
    nSelect = 1 as i32;
    if (op == (136 as i32) || op == (135 as i32))
        && (unsafe { (*db).dbOptFlags }) & ((2097152 as i32) as u32) == ((0 as i32) as u32)
    {
        pSplit = p;
        '__slate_break_2095: while (unsafe { (*pSplit).pPrior }) != std::ptr::null_mut::<Select>()
            && (((unsafe { (*pSplit).op }) as u32) as i32) == op
        {
            let __v2629: i32 = nSelect;
            let __v2630: i32 = __v2629 + (1 as i32);
            nSelect = __v2630;
            0 as i32;
            pSplit = unsafe { (*pSplit).pPrior };
        }
    }
    if nSelect <= (3 as i32) {
        pSplit = p;
    } else {
        pSplit = p;
        i = 2 as i32;
        '__slate_break_2096: loop {
            if !(i < nSelect) {
                break;
            }
            pSplit = unsafe { (*pSplit).pPrior };
            let __v2631: i32 = i;
            let __v2632: i32 = __v2631 + (2 as i32);
            i = __v2632;
        }
    }
    pPrior = unsafe { (*pSplit).pPrior };
    0 as i32;
    unsafe {
        (*pSplit).pPrior = std::ptr::null_mut::<Select>();
    }
    unsafe {
        (*pPrior).pNext = std::ptr::null_mut::<Select>();
    }
    0 as i32;
    0 as i32;
    unsafe {
        (*pPrior).pOrderBy = unsafe {
            sqlite3ExprListDup(
                unsafe { (*pParse).db },
                pOrderBy as *const ExprList,
                0 as i32,
            )
        };
    }
    unsafe {
        sqlite3ResolveOrderGroupBy(
            pParse,
            p,
            unsafe { (*p).pOrderBy },
            (b"ORDER\0".as_ptr() as *mut i8) as *const i8,
        )
    };
    unsafe {
        sqlite3ResolveOrderGroupBy(
            pParse,
            pPrior,
            unsafe { (*pPrior).pOrderBy },
            (b"ORDER\0".as_ptr() as *mut i8) as *const i8,
        )
    };
    // /* Compute the limit registers */
    computeLimitRegisters(pParse, p, labelEnd);
    if (unsafe { (*p).iLimit }) != (0 as i32) && op == (136 as i32) {
        let __v2633: *mut Parse = pParse;
        let __v2634: i32 = unsafe { (*__v2633).nMem };
        let __v2635: i32 = __v2634 + (1 as i32);
        unsafe {
            (*__v2633).nMem = __v2635;
        }
        regLimitA = __v2635;
        let __v2636: *mut Parse = pParse;
        let __v2637: i32 = unsafe { (*__v2636).nMem };
        let __v2638: i32 = __v2637 + (1 as i32);
        unsafe {
            (*__v2636).nMem = __v2638;
        }
        regLimitB = __v2638;
        unsafe {
            sqlite3VdbeAddOp2(
                v,
                82 as i32,
                if (unsafe { (*p).iOffset }) != (0 as i32) {
                    (unsafe { (*p).iOffset }) + (1 as i32)
                } else {
                    unsafe { (*p).iLimit }
                },
                regLimitA,
            )
        };
        unsafe { sqlite3VdbeAddOp2(v, 82 as i32, regLimitA, regLimitB) };
    } else {
        regLimitB = 0 as i32;
        regLimitA = 0 as i32;
    }
    unsafe { sqlite3ExprDelete(db, unsafe { (*p).pLimit }) };
    unsafe {
        (*p).pLimit = std::ptr::null_mut::<Expr>();
    }
    let __v2639: *mut Parse = pParse;
    let __v2640: i32 = unsafe { (*__v2639).nMem };
    let __v2641: i32 = __v2640 + (1 as i32);
    unsafe {
        (*__v2639).nMem = __v2641;
    }
    regAddrA = __v2641;
    let __v2642: *mut Parse = pParse;
    let __v2643: i32 = unsafe { (*__v2642).nMem };
    let __v2644: i32 = __v2643 + (1 as i32);
    unsafe {
        (*__v2642).nMem = __v2644;
    }
    regAddrB = __v2644;
    let __v2645: *mut Parse = pParse;
    let __v2646: i32 = unsafe { (*__v2645).nMem };
    let __v2647: i32 = __v2646 + (1 as i32);
    unsafe {
        (*__v2645).nMem = __v2647;
    }
    regOutA = __v2647;
    let __v2648: *mut Parse = pParse;
    let __v2649: i32 = unsafe { (*__v2648).nMem };
    let __v2650: i32 = __v2649 + (1 as i32);
    unsafe {
        (*__v2648).nMem = __v2650;
    }
    regOutB = __v2650;
    sqlite3SelectDestInit(std::ptr::addr_of_mut!(destA), 11 as i32, regAddrA);
    sqlite3SelectDestInit(std::ptr::addr_of_mut!(destB), 11 as i32, regAddrB);
    unsafe {
        sqlite3VdbeExplain(
            pParse,
            ((1 as i32) as i8) as u8,
            (b"MERGE (%s)\0".as_ptr() as *mut i8) as *const i8,
            sqlite3SelectOpName(((unsafe { (*p).op }) as u32) as i32),
        )
    };
    // /* Generate a coroutine to evaluate the SELECT statement to the
    //   ** left of the compound operator - the "A" select.
    //   */
    addrSelectA = (unsafe { sqlite3VdbeCurrentAddr(v) }) + (1 as i32);
    addr1 = unsafe { sqlite3VdbeAddOp3(v, 11 as i32, regAddrA, 0 as i32, addrSelectA) };
    {}
    unsafe {
        (*pPrior).iLimit = regLimitA;
    }
    unsafe {
        sqlite3VdbeExplain(
            pParse,
            ((1 as i32) as i8) as u8,
            (b"LEFT\0".as_ptr() as *mut i8) as *const i8,
        )
    };
    sqlite3Select(pParse, pPrior, std::ptr::addr_of_mut!(destA));
    unsafe { sqlite3VdbeEndCoroutine(v, regAddrA) };
    unsafe { sqlite3VdbeJumpHere(v, addr1) };
    // /* Generate a coroutine to evaluate the SELECT statement on
    //   ** the right - the "B" select
    //   */
    addrSelectB = (unsafe { sqlite3VdbeCurrentAddr(v) }) + (1 as i32);
    addr1 = unsafe { sqlite3VdbeAddOp3(v, 11 as i32, regAddrB, 0 as i32, addrSelectB) };
    {}
    savedLimit = unsafe { (*p).iLimit };
    savedOffset = unsafe { (*p).iOffset };
    unsafe {
        (*p).iLimit = regLimitB;
    }
    unsafe {
        (*p).iOffset = 0 as i32;
    }
    unsafe {
        sqlite3VdbeExplain(
            pParse,
            ((1 as i32) as i8) as u8,
            (b"RIGHT\0".as_ptr() as *mut i8) as *const i8,
        )
    };
    sqlite3Select(pParse, p, std::ptr::addr_of_mut!(destB));
    unsafe {
        (*p).iLimit = savedLimit;
    }
    unsafe {
        (*p).iOffset = savedOffset;
    }
    unsafe { sqlite3VdbeEndCoroutine(v, regAddrB) };
    // /* Generate a subroutine that outputs the current row of the A
    //   ** select as the next output row of the compound select.
    //   */
    {}
    addrOutA = generateOutputSubroutine(
        pParse,
        p,
        std::ptr::addr_of_mut!(destA),
        pDest,
        regOutA,
        regPrev,
        pKeyDup,
        labelEnd,
    );
    // /* Generate a subroutine that outputs the current row of the B
    //   ** select as the next output row of the compound select.
    //   */
    if op == (136 as i32) || op == (135 as i32) {
        {}
        addrOutB = generateOutputSubroutine(
            pParse,
            p,
            std::ptr::addr_of_mut!(destB),
            pDest,
            regOutB,
            regPrev,
            pKeyDup,
            labelEnd,
        );
    }
    sqlite3KeyInfoUnref(pKeyDup);
    // /* Generate a subroutine to run when the results from select A
    //   ** are exhausted and only data in select B remains.
    //   */
    if op == (137 as i32) || op == (138 as i32) {
        let __v2651: i32 = labelEnd;
        addrEofA = __v2651;
        addrEofA_noB = __v2651;
    } else {
        {}
        addrEofA = unsafe { sqlite3VdbeAddOp2(v, 10 as i32, regOutB, addrOutB) };
        {}
        addrEofA_noB = unsafe { sqlite3VdbeAddOp2(v, 12 as i32, regAddrB, labelEnd) };
        {}
        {}
        unsafe { sqlite3VdbeGoto(v, addrEofA) };
        unsafe {
            (*p).nSelectRow = unsafe {
                sqlite3LogEstAdd(unsafe { (*p).nSelectRow }, unsafe { (*pPrior).nSelectRow })
            };
        }
    }
    // /* Generate a subroutine to run when the results from select B
    //   ** are exhausted and only data in select A remains.
    //   */
    if op == (138 as i32) {
        addrEofB = addrEofA;
        if ((unsafe { (*p).nSelectRow }) as i32) > ((unsafe { (*pPrior).nSelectRow }) as i32) {
            unsafe {
                (*p).nSelectRow = unsafe { (*pPrior).nSelectRow };
            }
        }
    } else {
        {}
        addrEofB = unsafe { sqlite3VdbeAddOp2(v, 10 as i32, regOutA, addrOutA) };
        {}
        unsafe { sqlite3VdbeAddOp2(v, 12 as i32, regAddrA, labelEnd) };
        {}
        {}
        unsafe { sqlite3VdbeGoto(v, addrEofB) };
    }
    // /* Generate code to handle the case of A<B
    //   */
    addrAltB = unsafe { sqlite3VdbeAddOp2(v, 10 as i32, regOutA, addrOutA) };
    {}
    unsafe { sqlite3VdbeAddOp2(v, 12 as i32, regAddrA, addrEofA) };
    {}
    {}
    unsafe { sqlite3VdbeGoto(v, labelCmpr) };
    // /* Generate code to handle the case of A==B
    //   */
    if op == (136 as i32) {
        addrAeqB = addrAltB;
    } else {
        if op == (138 as i32) {
            addrAeqB = addrAltB;
            let __v2652: i32 = addrAltB;
            let __v2653: i32 = __v2652 + (1 as i32);
            addrAltB = __v2653;
        } else {
            addrAeqB = addrAltB + (1 as i32);
        }
    }
    // /* Generate code to handle the case of A>B
    //   */
    addrAgtB = unsafe { sqlite3VdbeCurrentAddr(v) };
    if op == (136 as i32) || op == (135 as i32) {
        unsafe { sqlite3VdbeAddOp2(v, 10 as i32, regOutB, addrOutB) };
        {}
        unsafe { sqlite3VdbeAddOp2(v, 12 as i32, regAddrB, addrEofB) };
        {}
        {}
        unsafe { sqlite3VdbeGoto(v, labelCmpr) };
    } else {
        // /* Just do next-B.  Might as well use the next-B call
        //                 ** in the next code block */
        let __v2654: i32 = addrAgtB;
        let __v2655: i32 = __v2654 + (1 as i32);
        addrAgtB = __v2655;
    }
    // /* This code runs once to initialize everything.
    //   */
    unsafe { sqlite3VdbeJumpHere(v, addr1) };
    unsafe { sqlite3VdbeAddOp2(v, 12 as i32, regAddrA, addrEofA_noB) };
    {}
    {}
    // /* v---  Also the A>B case for EXCEPT and INTERSECT */
    unsafe { sqlite3VdbeAddOp2(v, 12 as i32, regAddrB, addrEofB) };
    {}
    {}
    // /* Implement the main merge loop
    //   */
    if aPermute != std::ptr::null_mut::<u32>() {
        unsafe {
            sqlite3VdbeAddOp4(
                v,
                91 as i32,
                0 as i32,
                0 as i32,
                0 as i32,
                (aPermute as *mut i8) as *const i8,
                -(13 as i32),
            )
        };
    }
    unsafe { sqlite3VdbeResolveLabel(v, labelCmpr) };
    unsafe {
        sqlite3VdbeAddOp4(
            v,
            92 as i32,
            destA.iSdst,
            destB.iSdst,
            nOrderBy,
            (pKeyMerge as *mut i8) as *const i8,
            -(9 as i32),
        )
    };
    if aPermute != std::ptr::null_mut::<u32>() {
        unsafe { sqlite3VdbeChangeP5(v, ((1 as i32) as i16) as u16) };
    }
    unsafe { sqlite3VdbeAddOp3(v, 14 as i32, addrAltB, addrAeqB, addrAgtB) };
    {}
    {}
    {}
    {}
    // /* Jump to the this point in order to terminate the query.
    //   */
    unsafe { sqlite3VdbeResolveLabel(v, labelEnd) };
    // /* Make arrangements to free the 2nd and subsequent arms of the compound
    //   ** after the parse has finished */
    if (unsafe { (*pSplit).pPrior }) != std::ptr::null_mut::<Select>() {
        unsafe {
            sqlite3ParserAddCleanup(
                pParse,
                Some(sqlite3SelectDeleteGeneric),
                (unsafe { (*pSplit).pPrior }) as *mut (),
            )
        };
    }
    unsafe {
        (*pSplit).pPrior = pPrior;
    }
    unsafe {
        (*pPrior).pNext = pSplit;
    }
    unsafe { sqlite3ExprListDelete(db, unsafe { (*pPrior).pOrderBy }) };
    unsafe {
        (*pPrior).pOrderBy = std::ptr::null_mut::<ExprList>();
    }
    // /*** TBD:  Insert subroutine calls to close cursors on incomplete
    //   **** subqueries ****/
    unsafe { sqlite3VdbeExplainPop(pParse) };
    return ((unsafe { (*pParse).nErr }) != (0 as i32)) as i32;
}

// /* Parsing context */
// /* The recursive SELECT to be coded */
// /* What to do with query results */
// /* SQLITE_OMIT_CTE */
// /* Forward references */
// /* Parsing context */
// /* The right-most of SELECTs to be coded */
// /* What to do with query results */
// /*
// ** Handle the special case of a compound-select that originates from a
// ** VALUES clause.  By handling this as a special case, we avoid deep
// ** recursion, and thus do not need to enforce the SQLITE_LIMIT_COMPOUND_SELECT
// ** on a VALUES clause.
// **
// ** Because the Select object originates from a VALUES clause:
// **   (1) There is no LIMIT or OFFSET or else there is a LIMIT of exactly 1
// **   (2) All terms are UNION ALL
// **   (3) There is no ORDER BY clause
// **
// ** The "LIMIT of exactly 1" case of condition (1) comes about when a VALUES
// ** clause occurs within scalar expression (ex: "SELECT (VALUES(1),(2),(3))").
// ** The sqlite3CodeSubselect will have added the LIMIT 1 clause in tht case.
// ** Since the limit is exactly 1, we only need to evaluate the left-most VALUES.
// */
fn multiSelectValues(
    mut pParse: *mut Parse,
    mut p: *mut Select,
    mut pDest: *mut SelectDest,
) -> i32 {
    let mut nRow: i32 = 1 as i32;
    let mut rc: i32 = 0 as i32;
    let mut bShowAll: i32 = ((unsafe { (*p).pLimit }) == std::ptr::null_mut::<Expr>()) as i32;
    0 as i32;
    '__slate_break_2077: loop {
        0 as i32;
        0 as i32;
        0 as i32;
        if (unsafe { (*p).pWin }) != std::ptr::null_mut::<Window>() {
            return -(1 as i32);
        }
        if (unsafe { (*p).pPrior }) == std::ptr::null_mut::<Select>() {
            break '__slate_break_2077;
        }
        0 as i32;
        p = unsafe { (*p).pPrior };
        let __v2656: i32 = nRow;
        let __v2657: i32 = __v2656 + bShowAll;
        nRow = __v2657;
        if !((1 as i32) != (0 as i32)) {
            break;
        }
    }
    unsafe {
        sqlite3VdbeExplain(
            pParse,
            ((0 as i32) as i8) as u8,
            (b"SCAN %d CONSTANT ROW%s\0".as_ptr() as *mut i8) as *const i8,
            nRow,
            if nRow == (1 as i32) {
                b"\0".as_ptr() as *mut i8
            } else {
                b"S\0".as_ptr() as *mut i8
            },
        )
    };
    '__slate_break_2081: while p != std::ptr::null_mut::<Select>() {
        selectInnerLoop(
            pParse,
            p,
            -(1 as i32),
            std::ptr::null_mut::<SortCtx>(),
            std::ptr::null_mut::<DistinctCtx>(),
            pDest,
            1 as i32,
            1 as i32,
        );
        if !(bShowAll != (0 as i32)) {
            break '__slate_break_2081;
        }
        unsafe {
            (*p).nSelectRow = nRow as i16;
        }
        p = unsafe { (*p).pNext };
    }
    return rc;
}

// /* Parsing context */
// /* The right-most of SELECTs to be coded */
// /* What to do with query results */
// /*
// ** Return true if the SELECT statement which is known to be the recursive
// ** part of a recursive CTE still has its anchor terms attached.  If the
// ** anchor terms have already been removed, then return false.
// */
fn hasAnchor(mut p: *mut Select) -> i32 {
    '__slate_break_2082: while p != std::ptr::null_mut::<Select>()
        && (unsafe { (*p).selFlags }) & ((8192 as i32) as u32) != ((0 as i32) as u32)
    {
        p = unsafe { (*p).pPrior };
    }
    return (p != std::ptr::null_mut::<Select>()) as i32;
}

// /*
// ** This routine is called to process a compound query form from
// ** two or more separate queries using UNION, UNION ALL, EXCEPT, or
// ** INTERSECT
// **
// ** "p" points to the right-most of the two queries.  the query on the
// ** left is p->pPrior.  The left query could also be a compound query
// ** in which case this routine will be called recursively.
// **
// ** The results of the total query are to be written into a destination
// ** of type eDest with parameter iParm.
// **
// ** Example 1:  Consider a three-way compound SQL statement.
// **
// **     SELECT a FROM t1 UNION SELECT b FROM t2 UNION SELECT c FROM t3
// **
// ** This statement is parsed up as follows:
// **
// **     SELECT c FROM t3
// **      |
// **      `----->  SELECT b FROM t2
// **                |
// **                `------>  SELECT a FROM t1
// **
// ** The arrows in the diagram above represent the Select.pPrior pointer.
// ** So if this routine is called with p equal to the t3 query, then
// ** pPrior will be the t2 query.  p->op will be TK_UNION in this case.
// **
// ** Notice that because of the way SQLite parses compound SELECTs, the
// ** individual selects always group from left to right.
// */
fn multiSelect(mut pParse: *mut Parse, mut p: *mut Select, mut pDest: *mut SelectDest) -> i32 {
    let mut __slate_storage_931: std::mem::MaybeUninit<*mut Expr> = std::mem::MaybeUninit::uninit();
    let __slate_slot_931: *mut *mut Expr =
        std::ptr::addr_of_mut!(__slate_storage_931) as *mut *mut Expr;
    let mut __slate_storage_2660: std::mem::MaybeUninit<bool> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2660: *mut bool = std::ptr::addr_of_mut!(__slate_storage_2660) as *mut bool;
    let mut __slate_storage_2659: std::mem::MaybeUninit<bool> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2659: *mut bool = std::ptr::addr_of_mut!(__slate_storage_2659) as *mut bool;
    let mut __slate_storage_933: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_933: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_933) as *mut i32;
    let mut __slate_storage_932: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_932: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_932) as *mut i32;
    let mut __slate_storage_2658: std::mem::MaybeUninit<bool> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2658: *mut bool = std::ptr::addr_of_mut!(__slate_storage_2658) as *mut bool;
    let mut __slate_storage_930: std::mem::MaybeUninit<*mut sqlite3> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_930: *mut *mut sqlite3 =
        std::ptr::addr_of_mut!(__slate_storage_930) as *mut *mut sqlite3;
    let mut __slate_storage_929: std::mem::MaybeUninit<*mut Select> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_929: *mut *mut Select =
        std::ptr::addr_of_mut!(__slate_storage_929) as *mut *mut Select;
    let mut __slate_storage_928: std::mem::MaybeUninit<SelectDest> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_928: *mut SelectDest =
        std::ptr::addr_of_mut!(__slate_storage_928) as *mut SelectDest;
    let mut __slate_storage_927: std::mem::MaybeUninit<*mut Vdbe> = std::mem::MaybeUninit::uninit();
    let __slate_slot_927: *mut *mut Vdbe =
        std::ptr::addr_of_mut!(__slate_storage_927) as *mut *mut Vdbe;
    let mut __slate_storage_926: std::mem::MaybeUninit<*mut Select> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_926: *mut *mut Select =
        std::ptr::addr_of_mut!(__slate_storage_926) as *mut *mut Select;
    let mut __slate_storage_925: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_925: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_925) as *mut i32;
    unsafe {
        '__join_33: {
            // /* Success code from a subroutine */
            std::ptr::write(__slate_slot_925, 0 as i32);
            // /* Another SELECT immediately to our left */
            // /* Generate code to this VDBE */
            // /* Alternative data destination */
            // /* Chain of simple selects to delete */
            std::ptr::write(__slate_slot_929, std::ptr::null_mut::<Select>());
            // /* Database connection */
            // /* Make sure there is no ORDER BY or LIMIT clause on prior SELECTs.  Only
            //   ** the last (right-most) SELECT in the series may have an ORDER BY or LIMIT.
            //   */
            // /* Calling function guarantees this much */
            0 as i32;
            0 as i32;
            0 as i32;
            *__slate_slot_930 = unsafe { (*pParse).db };
            *__slate_slot_926 = unsafe { (*p).pPrior };
            *__slate_slot_928 = unsafe { *pDest };
            0 as i32;
            0 as i32;
            *__slate_slot_927 = sqlite3GetVdbe(pParse);
            // /* The VDBE already created by calling function */
            0 as i32;
            // /* Create the destination temporary table if necessary
            //   */
            if (((*__slate_slot_928).eDest as u32) as i32) == (10 as i32) {
                0 as i32;
                unsafe {
                    sqlite3VdbeAddOp2(
                        *__slate_slot_927,
                        120 as i32,
                        (*__slate_slot_928).iSDParm,
                        unsafe { (*unsafe { (*p).pEList }).nExpr },
                    )
                };
                (*__slate_slot_928).eDest = ((12 as i32) as i8) as u8;
            }
        }
        '__join_2: {
            // /* Special handling for a compound-select that originates as a VALUES clause.
            //   */
            if (unsafe { (*p).selFlags }) & ((1024 as i32) as u32) != (0 as u32) {
                *__slate_slot_925 =
                    multiSelectValues(pParse, p, std::ptr::addr_of_mut!(*__slate_slot_928));
                if *__slate_slot_925 >= (0 as i32) {
                    break '__join_2;
                } else {
                    *__slate_slot_925 = 0 as i32;
                }
            }
            // /* Make sure all SELECTs in the statement have the same number of elements
            //   ** in their result sets.
            //   */
            0 as i32;
            0 as i32;
            if (unsafe { (*p).selFlags }) & ((8192 as i32) as u32) != ((0 as i32) as u32) {
                *__slate_slot_2658 = hasAnchor(p) != (0 as i32);
            } else {
                *__slate_slot_2658 = false as bool;
            }
            if *__slate_slot_2658 {
                generateWithRecursiveQuery(pParse, p, std::ptr::addr_of_mut!(*__slate_slot_928));
            } else {
                if (unsafe { (*p).pOrderBy }) != std::ptr::null_mut::<ExprList>() {
                    // /* If the compound has an ORDER BY clause, then always use the merge
                    //     ** algorithm. */
                    return multiSelectByMerge(pParse, p, pDest);
                } else {
                    if (((unsafe { (*p).op }) as u32) as i32) != (136 as i32) {
                        // /* If the compound is EXCEPT, INTERSECT, or UNION (anything other than
                        //     ** UNION ALL) then also always use the merge algorithm.  However, the
                        //     ** multiSelectByMerge() routine requires that the compound have an
                        //     ** ORDER BY clause, and it doesn't right now.  So invent one first. */
                        std::ptr::write(__slate_slot_931, unsafe {
                            sqlite3ExprInt32(*__slate_slot_930, 1 as i32)
                        });
                        unsafe {
                            (*p).pOrderBy = unsafe {
                                sqlite3ExprListAppend(
                                    pParse,
                                    std::ptr::null_mut::<ExprList>(),
                                    *__slate_slot_931,
                                )
                            };
                        }
                        if (unsafe { (*pParse).nErr }) != (0 as i32) {
                        } else {
                            0 as i32;
                            unsafe {
                                (*unsafe {
                                    unsafe {
                                        std::ptr::addr_of_mut!((*unsafe { (*p).pOrderBy }).a)
                                            as *mut ExprList_item
                                    }
                                    .offset((0 as i32) as isize)
                                })
                                .u
                                .x
                                .iOrderByCol = ((1 as i32) as i16) as u16;
                            }
                            return multiSelectByMerge(pParse, p, pDest);
                        }
                    } else {
                        // /* For a UNION ALL compound without ORDER BY, simply run the left
                        //     ** query, then run the right query */
                        std::ptr::write(__slate_slot_932, 0 as i32);
                        // /* Initialize to suppress harmless compiler warning */
                        std::ptr::write(__slate_slot_933, 0 as i32);
                        if (unsafe { (*(*__slate_slot_926)).pPrior })
                            == std::ptr::null_mut::<Select>()
                        {
                            unsafe {
                                sqlite3VdbeExplain(
                                    pParse,
                                    ((1 as i32) as i8) as u8,
                                    (b"COMPOUND QUERY\0".as_ptr() as *mut i8) as *const i8,
                                )
                            };
                            unsafe {
                                sqlite3VdbeExplain(
                                    pParse,
                                    ((1 as i32) as i8) as u8,
                                    (b"LEFT-MOST SUBQUERY\0".as_ptr() as *mut i8) as *const i8,
                                )
                            };
                        }
                        0 as i32;
                        unsafe {
                            (*(*__slate_slot_926)).iLimit = unsafe { (*p).iLimit };
                        }
                        unsafe {
                            (*(*__slate_slot_926)).iOffset = unsafe { (*p).iOffset };
                        }
                        unsafe {
                            (*(*__slate_slot_926)).pLimit = unsafe {
                                sqlite3ExprDup(
                                    *__slate_slot_930,
                                    (unsafe { (*p).pLimit }) as *const Expr,
                                    0 as i32,
                                )
                            };
                        }
                        {}
                        *__slate_slot_925 = sqlite3Select(
                            pParse,
                            *__slate_slot_926,
                            std::ptr::addr_of_mut!(*__slate_slot_928),
                        );
                        unsafe {
                            sqlite3ExprDelete(*__slate_slot_930, unsafe {
                                (*(*__slate_slot_926)).pLimit
                            })
                        };
                        unsafe {
                            (*(*__slate_slot_926)).pLimit = std::ptr::null_mut::<Expr>();
                        }
                        if *__slate_slot_925 != (0 as i32) {
                        } else {
                            unsafe {
                                (*p).pPrior = std::ptr::null_mut::<Select>();
                            }
                            unsafe {
                                (*p).iLimit = unsafe { (*(*__slate_slot_926)).iLimit };
                            }
                            unsafe {
                                (*p).iOffset = unsafe { (*(*__slate_slot_926)).iOffset };
                            }
                            if (unsafe { (*p).iLimit }) != (0 as i32) {
                                *__slate_slot_932 = unsafe {
                                    sqlite3VdbeAddOp1(*__slate_slot_927, 17 as i32, unsafe {
                                        (*p).iLimit
                                    })
                                };
                                {}
                                {}
                                if (unsafe { (*p).iOffset }) != (0 as i32) {
                                    unsafe {
                                        sqlite3VdbeAddOp3(
                                            *__slate_slot_927,
                                            162 as i32,
                                            unsafe { (*p).iLimit },
                                            (unsafe { (*p).iOffset }) + (1 as i32),
                                            unsafe { (*p).iOffset },
                                        )
                                    };
                                }
                            }
                            unsafe {
                                sqlite3VdbeExplain(
                                    pParse,
                                    ((1 as i32) as i8) as u8,
                                    (b"UNION ALL\0".as_ptr() as *mut i8) as *const i8,
                                )
                            };
                            {}
                            *__slate_slot_925 =
                                sqlite3Select(pParse, p, std::ptr::addr_of_mut!(*__slate_slot_928));
                            {}
                            *__slate_slot_929 = unsafe { (*p).pPrior };
                            unsafe {
                                (*p).pPrior = *__slate_slot_926;
                            }
                            unsafe {
                                (*p).nSelectRow = unsafe {
                                    sqlite3LogEstAdd(unsafe { (*p).nSelectRow }, unsafe {
                                        (*(*__slate_slot_926)).nSelectRow
                                    })
                                };
                            }
                            if (unsafe { (*p).pLimit }) != std::ptr::null_mut::<Expr>() {
                                *__slate_slot_2659 = (unsafe {
                                    sqlite3ExprIsInteger(
                                        (unsafe { (*unsafe { (*p).pLimit }).pLeft }) as *const Expr,
                                        std::ptr::addr_of_mut!(*__slate_slot_933),
                                        pParse,
                                        0 as i32,
                                    )
                                }) != (0 as i32);
                            } else {
                                *__slate_slot_2659 = false as bool;
                            }
                            if *__slate_slot_2659 && *__slate_slot_933 > (0 as i32) {
                                *__slate_slot_2660 = ((unsafe { (*p).nSelectRow }) as i32)
                                    > ((unsafe { sqlite3LogEst((*__slate_slot_933 as i64) as u64) })
                                        as i32);
                            } else {
                                *__slate_slot_2660 = false as bool;
                            }
                            if *__slate_slot_2660 {
                                unsafe {
                                    (*p).nSelectRow =
                                        unsafe { sqlite3LogEst((*__slate_slot_933 as i64) as u64) };
                                }
                            }
                            if *__slate_slot_932 != (0 as i32) {
                                unsafe {
                                    sqlite3VdbeJumpHere(*__slate_slot_927, *__slate_slot_932)
                                };
                            }
                            if (unsafe { (*p).pNext }) == std::ptr::null_mut::<Select>() {
                                unsafe { sqlite3VdbeExplainPop(pParse) };
                            }
                        }
                    }
                }
            }
        }
        unsafe {
            (*pDest).iSdst = (*__slate_slot_928).iSdst;
        }
        unsafe {
            (*pDest).nSdst = (*__slate_slot_928).nSdst;
        }
        unsafe {
            (*pDest).iSDParm2 = (*__slate_slot_928).iSDParm2;
        }
        if *__slate_slot_929 != std::ptr::null_mut::<Select>() {
            unsafe {
                sqlite3ParserAddCleanup(
                    pParse,
                    Some(sqlite3SelectDeleteGeneric),
                    *__slate_slot_929 as *mut (),
                )
            };
        }
        return *__slate_slot_925;
    }
    return unsafe { std::mem::zeroed() };
}

// /*
// ** Code an output subroutine for a coroutine implementation of a
// ** SELECT statement.
// **
// ** The data to be output is contained in an array of pIn->nSdst registers
// ** starting at register pIn->iSdst.  pDest is where the output should
// ** be sent.
// **
// ** regReturn is the number of the register holding the subroutine
// ** return address.
// **
// ** If regPrev>0 then it is the first register in a vector that
// ** records the previous output.  mem[regPrev] is a flag that is false
// ** if there has been no previous output.  If regPrev>0 then code is
// ** generated to suppress duplicates.  pKeyInfo is used for comparing
// ** keys.
// **
// ** If the LIMIT found in p->iLimit is reached, jump immediately to
// ** iBreak.
// */
fn generateOutputSubroutine(
    mut pParse: *mut Parse,
    mut p: *mut Select,
    mut pIn: *mut SelectDest,
    mut pDest: *mut SelectDest,
    mut regReturn: i32,
    mut regPrev: i32,
    mut pKeyInfo: *mut KeyInfo,
    mut iBreak: i32,
) -> i32 {
    let mut v: *mut Vdbe = unsafe { (*pParse).pVdbe };
    let mut iContinue: i32 = 0 as i32;
    let mut addr: i32 = 0 as i32;
    0 as i32;
    addr = unsafe { sqlite3VdbeCurrentAddr(v) };
    iContinue = unsafe { sqlite3VdbeMakeLabel(pParse) };
    // /* Suppress duplicates for UNION, EXCEPT, and INTERSECT
    //   */
    if regPrev != (0 as i32) {
        let mut addr1: i32 = 0 as i32;
        let mut addr2: i32 = 0 as i32;
        addr1 = unsafe { sqlite3VdbeAddOp1(v, 17 as i32, regPrev) };
        {}
        addr2 = unsafe {
            sqlite3VdbeAddOp4(
                v,
                92 as i32,
                unsafe { (*pIn).iSdst },
                regPrev + (1 as i32),
                unsafe { (*pIn).nSdst },
                (sqlite3KeyInfoRef(pKeyInfo) as *mut i8) as *const i8,
                -(9 as i32),
            )
        };
        unsafe {
            sqlite3VdbeAddOp3(
                v,
                14 as i32,
                addr2 + (2 as i32),
                iContinue,
                addr2 + (2 as i32),
            )
        };
        {}
        unsafe { sqlite3VdbeJumpHere(v, addr1) };
        unsafe {
            sqlite3VdbeAddOp3(
                v,
                82 as i32,
                unsafe { (*pIn).iSdst },
                regPrev + (1 as i32),
                (unsafe { (*pIn).nSdst }) - (1 as i32),
            )
        };
        unsafe { sqlite3VdbeAddOp2(v, 73 as i32, 1 as i32, regPrev) };
    }
    if (unsafe { (*unsafe { (*pParse).db }).mallocFailed }) != (0 as u8) {
        return 0 as i32;
    }
    // /* Suppress the first OFFSET entries if there is an OFFSET clause
    //   */
    codeOffset(v, unsafe { (*p).iOffset }, iContinue);
    '__slate_break_2088: {
        match ((unsafe { (*pDest).eDest }) as u32) as i32 {
            6 | 3 | 12 | 10 => {
                // /* Store the result as data using a unique key.
                //     */
                let mut r1: i32 = unsafe { sqlite3GetTempReg(pParse) };
                let mut r2: i32 = unsafe { sqlite3GetTempReg(pParse) };
                let mut iParm: i32 = unsafe { (*pDest).iSDParm };
                {}
                {}
                {}
                {}
                unsafe {
                    sqlite3VdbeAddOp3(
                        v,
                        99 as i32,
                        unsafe { (*pIn).iSdst },
                        unsafe { (*pIn).nSdst },
                        r1,
                    )
                };
                if (((unsafe { (*pDest).eDest }) as u32) as i32) == (3 as i32) {
                    // /* If the destination is DistFifo, then cursor (iParm+1) is open
                    //         ** on an ephemeral index that is used to enforce uniqueness on the
                    //         ** total result.  At this point, we are processing the setup portion
                    //         ** of the recursive CTE using the merge algorithm, so the results are
                    //         ** guaranteed to be unique anyhow.  But we still need to populate the
                    //         ** (iParm+1) cursor for use by the subsequent recursive phase.
                    //         */
                    unsafe {
                        sqlite3VdbeAddOp4Int(
                            v,
                            140 as i32,
                            iParm + (1 as i32),
                            r1,
                            unsafe { (*pIn).iSdst },
                            unsafe { (*pIn).nSdst },
                        )
                    };
                }
                unsafe { sqlite3VdbeAddOp2(v, 129 as i32, iParm, r2) };
                unsafe { sqlite3VdbeAddOp3(v, 130 as i32, iParm, r1, r2) };
                unsafe { sqlite3VdbeChangeP5(v, ((8 as i32) as i16) as u16) };
                unsafe { sqlite3ReleaseTempReg(pParse, r2) };
                unsafe { sqlite3ReleaseTempReg(pParse, r1) };
                break '__slate_break_2088;
                // /* If any row exist in the result set, record that fact and abort.
                //     */
            }
            1 => {
                unsafe { sqlite3VdbeAddOp2(v, 73 as i32, 1 as i32, unsafe { (*pDest).iSDParm }) };
                // /* The LIMIT clause will terminate the loop for us */
                break '__slate_break_2088;
                // /* If we are creating a set for an "expr IN (SELECT ...)".
                //     */
            }
            9 => {
                let mut r1: i32 = 0 as i32;
                {}
                r1 = unsafe { sqlite3GetTempReg(pParse) };
                unsafe {
                    sqlite3VdbeAddOp4(
                        v,
                        99 as i32,
                        unsafe { (*pIn).iSdst },
                        unsafe { (*pIn).nSdst },
                        r1,
                        (unsafe { (*pDest).zAffSdst }) as *const i8,
                        unsafe { (*pIn).nSdst },
                    )
                };
                unsafe {
                    sqlite3VdbeAddOp4Int(
                        v,
                        140 as i32,
                        unsafe { (*pDest).iSDParm },
                        r1,
                        unsafe { (*pIn).iSdst },
                        unsafe { (*pIn).nSdst },
                    )
                };
                if (unsafe { (*pDest).iSDParm2 }) > (0 as i32) {
                    0 as i32;
                    unsafe {
                        sqlite3VdbeAddOp4Int(
                            v,
                            185 as i32,
                            unsafe { (*pDest).iSDParm2 },
                            0 as i32,
                            unsafe { (*pIn).iSdst },
                            unsafe { (*pIn).nSdst },
                        )
                    };
                    unsafe {
                        sqlite3VdbeExplain(
                            pParse,
                            ((0 as i32) as i8) as u8,
                            (b"CREATE BLOOM FILTER\0".as_ptr() as *mut i8) as *const i8,
                        )
                    };
                }
                unsafe { sqlite3ReleaseTempReg(pParse, r1) };
                break '__slate_break_2088;
                // /* If this is a scalar select that is part of an expression, then
                //     ** store the results in the appropriate memory cell and break out
                //     ** of the scan loop.  Note that the select might return multiple columns
                //     ** if it is the RHS of a row-value IN operator.
                //     */
            }
            8 => {
                {}
                unsafe {
                    sqlite3ExprCodeMove(
                        pParse,
                        unsafe { (*pIn).iSdst },
                        unsafe { (*pDest).iSDParm },
                        unsafe { (*pIn).nSdst },
                    )
                };
                // /* The LIMIT clause will jump out of the loop for us */
                break '__slate_break_2088;
                // /* #ifndef SQLITE_OMIT_SUBQUERY */
                // /* The results are stored in a sequence of registers
                //     ** starting at pDest->iSdst.  Then the co-routine yields.
                //     */
            }
            11 => {
                if (unsafe { (*pDest).iSdst }) == (0 as i32) {
                    unsafe {
                        (*pDest).iSdst =
                            unsafe { sqlite3GetTempRange(pParse, unsafe { (*pIn).nSdst }) };
                    }
                    unsafe {
                        (*pDest).nSdst = unsafe { (*pIn).nSdst };
                    }
                }
                unsafe {
                    sqlite3ExprCodeMove(
                        pParse,
                        unsafe { (*pIn).iSdst },
                        unsafe { (*pDest).iSdst },
                        unsafe { (*pIn).nSdst },
                    )
                };
                unsafe { sqlite3VdbeAddOp1(v, 12 as i32, unsafe { (*pDest).iSDParm }) };
                break '__slate_break_2088;
                // /* Write the results into a priority queue that is order according to
                //     ** pDest->pOrderBy (in pSO).  pDest->iSDParm (in iParm) is the cursor for an
                //     ** index with pSO->nExpr+2 columns.  Build a key using pSO for the first
                //     ** pSO->nExpr columns, then make sure all keys are unique by adding a
                //     ** final OP_Sequence column.  The last column is the record as a blob.
                //     */
            }
            4 | 5 => {
                let mut nKey: i32 = 0 as i32;
                let mut r1: i32 = 0 as i32;
                let mut r2: i32 = 0 as i32;
                let mut r3: i32 = 0 as i32;
                let mut ii: i32 = 0 as i32;
                let mut pSO: *mut ExprList = unsafe { std::mem::zeroed() };
                let mut iParm: i32 = unsafe { (*pDest).iSDParm };
                pSO = unsafe { (*pDest).pOrderBy };
                0 as i32;
                nKey = unsafe { (*pSO).nExpr };
                r1 = unsafe { sqlite3GetTempReg(pParse) };
                r2 = unsafe { sqlite3GetTempRange(pParse, nKey + (2 as i32)) };
                r3 = r2 + nKey + (1 as i32);
                unsafe {
                    sqlite3VdbeAddOp3(
                        v,
                        99 as i32,
                        unsafe { (*pIn).iSdst },
                        unsafe { (*pIn).nSdst },
                        r3,
                    )
                };
                if (((unsafe { (*pDest).eDest }) as u32) as i32) == (4 as i32) {
                    unsafe { sqlite3VdbeAddOp2(v, 140 as i32, iParm + (1 as i32), r3) };
                }
                ii = 0 as i32;
                '__slate_break_2090: loop {
                    if !(ii < nKey) {
                        break;
                    }
                    unsafe {
                        sqlite3VdbeAddOp2(
                            v,
                            83 as i32,
                            (unsafe { (*pIn).iSdst })
                                + (((unsafe {
                                    (*unsafe {
                                        unsafe {
                                            std::ptr::addr_of_mut!((*pSO).a) as *mut ExprList_item
                                        }
                                        .offset(ii as isize)
                                    })
                                    .u
                                    .x
                                    .iOrderByCol
                                }) as u32) as i32)
                                - (1 as i32),
                            r2 + ii,
                        )
                    };
                    let __v2661: i32 = ii;
                    let __v2662: i32 = __v2661 + (1 as i32);
                    ii = __v2662;
                }
                unsafe { sqlite3VdbeAddOp2(v, 128 as i32, iParm, r2 + nKey) };
                unsafe { sqlite3VdbeAddOp3(v, 99 as i32, r2, nKey + (2 as i32), r1) };
                unsafe { sqlite3VdbeAddOp4Int(v, 140 as i32, iParm, r1, r2, nKey + (2 as i32)) };
                unsafe { sqlite3ReleaseTempReg(pParse, r1) };
                unsafe { sqlite3ReleaseTempRange(pParse, r2, nKey + (2 as i32)) };
                break '__slate_break_2088;
                // /* SQLITE_OMIT_CTE */
                // /* Ignore the output */
            }
            2 => {
                break '__slate_break_2088;
                // /* If none of the above, then the result destination must be
                //     ** SRT_Output.
                //     **
                //     ** For SRT_Output, results are stored in a sequence of registers.
                //     ** Then the OP_ResultRow opcode is used to cause sqlite3_step() to
                //     ** return the next row of result.
                //     */
            }
            _ => {
                0 as i32;
                unsafe {
                    sqlite3VdbeAddOp2(v, 86 as i32, unsafe { (*pIn).iSdst }, unsafe {
                        (*pIn).nSdst
                    })
                };
            }
        }
    }
    // /* Jump to the end of the loop if the LIMIT is reached.
    //   */
    if (unsafe { (*p).iLimit }) != (0 as i32) {
        unsafe { sqlite3VdbeAddOp2(v, 63 as i32, unsafe { (*p).iLimit }, iBreak) };
        {}
    }
    // /* Generate the subroutine return
    //   */
    unsafe { sqlite3VdbeResolveLabel(v, iContinue) };
    unsafe { sqlite3VdbeAddOp1(v, 69 as i32, regReturn) };
    return addr;
}

// /* Description of the substitution */
// /* Expr in which substitution occurs */
fn substExprList(mut pSubst: *mut SubstContext, mut pList: *mut ExprList) {
    let mut i: i32 = 0 as i32;
    if pList == std::ptr::null_mut::<ExprList>() {
        return;
    }
    i = 0 as i32;
    '__slate_break_2108: loop {
        if !(i < unsafe { (*pList).nExpr }) {
            break;
        }
        unsafe {
            (*unsafe {
                unsafe { std::ptr::addr_of_mut!((*pList).a) as *mut ExprList_item }
                    .offset(i as isize)
            })
            .pExpr = substExpr(pSubst, unsafe {
                (*unsafe {
                    unsafe { std::ptr::addr_of_mut!((*pList).a) as *mut ExprList_item }
                        .offset(i as isize)
                })
                .pExpr
            });
        }
        let __v2663: i32 = i;
        let __v2664: i32 = __v2663 + (1 as i32);
        i = __v2664;
    }
}

// /* Description of the substitution */
// /* List to scan and in which to make substitutes */
fn substSelect(mut pSubst: *mut SubstContext, mut p: *mut Select, mut doPrior: i32) {
    let mut pSrc: *mut SrcList = unsafe { std::mem::zeroed() };
    let mut pItem: *mut SrcItem = unsafe { std::mem::zeroed() };
    let mut i: i32 = 0 as i32;
    if !(p != std::ptr::null_mut::<Select>()) {
        return;
    }
    let __v2665: *mut SubstContext = pSubst;
    let __v2666: i32 = unsafe { (*__v2665).nSelDepth };
    let __v2667: i32 = __v2666 + (1 as i32);
    unsafe {
        (*__v2665).nSelDepth = __v2667;
    }
    '__slate_break_2109: loop {
        substExprList(pSubst, unsafe { (*p).pEList });
        substExprList(pSubst, unsafe { (*p).pGroupBy });
        substExprList(pSubst, unsafe { (*p).pOrderBy });
        unsafe {
            (*p).pHaving = substExpr(pSubst, unsafe { (*p).pHaving });
        }
        unsafe {
            (*p).pWhere = substExpr(pSubst, unsafe { (*p).pWhere });
        }
        pSrc = unsafe { (*p).pSrc };
        0 as i32;
        i = unsafe { (*pSrc).nSrc };
        let __v2668: *mut SrcItem = unsafe { std::ptr::addr_of_mut!((*pSrc).a) as *mut SrcItem };
        pItem = __v2668;
        '__slate_break_2110: while i > (0 as i32) {
            if ((unsafe { (*pItem).fg.__slate_bits_0.__get_isSubquery() }) as i32) != (0 as i32) {
                substSelect(
                    pSubst,
                    unsafe { (*unsafe { (*pItem).u4.pSubq }).pSelect },
                    1 as i32,
                );
            }
            if ((unsafe { (*pItem).fg.__slate_bits_0.__get_isTabFunc() }) as i32) != (0 as i32) {
                substExprList(pSubst, unsafe { (*pItem).u1.pFuncArg });
            }
            let __v2669: i32 = i;
            let __v2670: i32 = __v2669 - (1 as i32);
            i = __v2670;
            let __v2671: *mut SrcItem = pItem;
            let __v2672: *mut SrcItem = unsafe { __v2671.offset((1 as i32) as isize) };
            pItem = __v2672;
        }
        let __v2673: bool;
        if doPrior != (0 as i32) {
            let __v2674: *mut Select = unsafe { (*p).pPrior };
            p = __v2674;
            __v2673 = __v2674 != std::ptr::null_mut::<Select>();
        } else {
            __v2673 = false as bool;
        }
        if !__v2673 {
            break;
        }
    }
    let __v2675: *mut SubstContext = pSubst;
    let __v2676: i32 = unsafe { (*__v2675).nSelDepth };
    let __v2677: i32 = __v2676 - (1 as i32);
    unsafe {
        (*__v2675).nSelDepth = __v2677;
    }
}

// /* Forward Declarations */
// /*
// ** Scan through the expression pExpr.  Replace every reference to
// ** a column in table number iTable with a copy of the iColumn-th
// ** entry in pEList.  (But leave references to the ROWID column
// ** unchanged.)
// **
// ** This routine is part of the flattening procedure.  A subquery
// ** whose result set is defined by pEList appears as entry in the
// ** FROM clause of a SELECT such that the VDBE cursor assigned to that
// ** FORM clause entry is iTable.  This routine makes the necessary
// ** changes to pExpr so that it refers directly to the source table
// ** of the subquery rather the result set of the subquery.
// */
fn substExpr(mut pSubst: *mut SubstContext, mut pExpr: *mut Expr) -> *mut Expr {
    if pExpr == std::ptr::null_mut::<Expr>() {
        return std::ptr::null_mut::<Expr>();
    }
    if (unsafe { (*pExpr).flags }) & (((1 as i32) | (2 as i32)) as u32) != ((0 as i32) as u32)
        && (unsafe { (*pExpr).w.iJoin }) == unsafe { (*pSubst).iTable }
    {
        {}
        unsafe {
            (*pExpr).w.iJoin = unsafe { (*pSubst).iNewTable };
        }
    }
    if (((unsafe { (*pExpr).op }) as u32) as i32) == (168 as i32)
        && (unsafe { (*pExpr).iTable }) == unsafe { (*pSubst).iTable }
        && !((unsafe { (*pExpr).flags }) & ((32 as i32) as u32) != ((0 as i32) as u32))
    {
        let mut pNew: *mut Expr = unsafe { std::mem::zeroed() };
        let mut iColumn: i32 = 0 as i32;
        let mut pCopy: *mut Expr = unsafe { std::mem::zeroed() };
        let mut ifNullRow: Expr = unsafe { std::mem::zeroed() };
        iColumn = (unsafe { (*pExpr).iColumn }) as i32;
        0 as i32;
        0 as i32;
        0 as i32;
        pCopy = unsafe {
            (*unsafe {
                unsafe {
                    std::ptr::addr_of_mut!((*unsafe { (*pSubst).pEList }).a) as *mut ExprList_item
                }
                .offset(iColumn as isize)
            })
            .pExpr
        };
        if (unsafe { sqlite3ExprIsVector(pCopy as *const Expr) }) != (0 as i32) {
            unsafe { sqlite3VectorErrorMsg(unsafe { (*pSubst).pParse }, pCopy) };
        } else {
            let mut db: *mut sqlite3 = unsafe { (*unsafe { (*pSubst).pParse }).db };
            if (unsafe { (*pSubst).isOuterJoin }) != (0 as i32)
                && ((((unsafe { (*pCopy).op }) as u32) as i32) != (168 as i32)
                    || (unsafe { (*pCopy).iTable }) != unsafe { (*pSubst).iNewTable })
            {
                unsafe {
                    memset(
                        std::ptr::addr_of_mut!(ifNullRow) as *mut (),
                        0 as i32,
                        72 as u64,
                    )
                };
                ifNullRow.op = ((179 as i32) as i8) as u8;
                ifNullRow.pLeft = pCopy;
                ifNullRow.iTable = unsafe { (*pSubst).iNewTable };
                ifNullRow.iColumn = -(99 as i32) as i16;
                ifNullRow.flags = (262144 as i32) as u32;
                pCopy = std::ptr::addr_of_mut!(ifNullRow);
            }
            {}
            pNew = unsafe { sqlite3ExprDup(db, pCopy as *const Expr, 0 as i32) };
            if (unsafe { (*db).mallocFailed }) != (0 as u8) {
                unsafe { sqlite3ExprDelete(db, pNew) };
                return pExpr;
            }
            if (unsafe { (*pSubst).isOuterJoin }) != (0 as i32) {
                let __v2678: *mut Expr = pNew;
                let __v2679: u32 = unsafe { (*__v2678).flags };
                let __v2680: u32 = __v2679 | ((2097152 as i32) as u32);
                unsafe {
                    (*__v2678).flags = __v2680;
                }
            }
            if (((unsafe { (*pNew).op }) as u32) as i32) == (171 as i32) {
                unsafe {
                    (*pNew).u.iValue = unsafe { sqlite3ExprTruthValue(pNew as *const Expr) };
                }
                unsafe {
                    (*pNew).op = ((156 as i32) as i8) as u8;
                }
                let __v2681: *mut Expr = pNew;
                let __v2682: u32 = unsafe { (*__v2681).flags };
                let __v2683: u32 = __v2682 | ((2048 as i32) as u32);
                unsafe {
                    (*__v2681).flags = __v2683;
                }
            }
            // /* Ensure that the expression now has an implicit collation sequence,
            //         ** just as it did when it was a column of a view or sub-query. */
            let mut pNat: *mut CollSeq =
                unsafe { sqlite3ExprCollSeq(unsafe { (*pSubst).pParse }, pNew as *const Expr) };
            let mut pColl: *mut CollSeq = unsafe {
                sqlite3ExprCollSeq(
                    unsafe { (*pSubst).pParse },
                    (unsafe {
                        (*unsafe {
                            unsafe {
                                std::ptr::addr_of_mut!((*unsafe { (*pSubst).pCList }).a)
                                    as *mut ExprList_item
                            }
                            .offset(iColumn as isize)
                        })
                        .pExpr
                    }) as *const Expr,
                )
            };
            if pNat != pColl
                || (((unsafe { (*pNew).op }) as u32) as i32) != (168 as i32)
                    && (((unsafe { (*pNew).op }) as u32) as i32) != (114 as i32)
            {
                pNew = unsafe {
                    sqlite3ExprAddCollateString(
                        (unsafe { (*pSubst).pParse }) as *const Parse,
                        pNew,
                        (if pColl != std::ptr::null_mut::<CollSeq>() {
                            unsafe { (*pColl).zName }
                        } else {
                            b"BINARY\0".as_ptr() as *mut i8
                        }) as *const i8,
                    )
                };
            }
            let __v2684: *mut Expr = pNew;
            let __v2685: u32 = unsafe { (*__v2684).flags };
            let __v2686: u32 = __v2685 & !((512 as i32) as u32);
            unsafe {
                (*__v2684).flags = __v2686;
            }
            if (unsafe { (*pExpr).flags }) & (((1 as i32) | (2 as i32)) as u32)
                != ((0 as i32) as u32)
            {
                sqlite3SetJoinExpr(
                    pNew,
                    unsafe { (*pExpr).w.iJoin },
                    (unsafe { (*pExpr).flags }) & (((1 as i32) | (2 as i32)) as u32),
                );
            }
            unsafe { sqlite3ExprDelete(db, pExpr) };
            pExpr = pNew;
        }
    } else {
        if (((unsafe { (*pExpr).op }) as u32) as i32) == (179 as i32)
            && (unsafe { (*pExpr).iTable }) == unsafe { (*pSubst).iTable }
        {
            unsafe {
                (*pExpr).iTable = unsafe { (*pSubst).iNewTable };
            }
        }
        if (((unsafe { (*pExpr).op }) as u32) as i32) == (169 as i32)
            && (((unsafe { (*pExpr).op2 }) as u32) as i32) >= unsafe { (*pSubst).nSelDepth }
        {
            let __v2687: *mut Expr = pExpr;
            let __v2688: u8 = unsafe { (*__v2687).op2 };
            let __v2689: u8 = ((((__v2688 as u32) as i32) - (1 as i32)) as i8) as u8;
            unsafe {
                (*__v2687).op2 = __v2689;
            }
        }
        unsafe {
            (*pExpr).pLeft = substExpr(pSubst, unsafe { (*pExpr).pLeft });
        }
        unsafe {
            (*pExpr).pRight = substExpr(pSubst, unsafe { (*pExpr).pRight });
        }
        if (unsafe { (*pExpr).flags }) & ((4096 as i32) as u32) != ((0 as i32) as u32) {
            substSelect(pSubst, unsafe { (*pExpr).x.pSelect }, 1 as i32);
        } else {
            substExprList(pSubst, unsafe { (*pExpr).x.pList });
        }
        if (unsafe { (*pExpr).flags }) & ((16777216 as i32) as u32) != ((0 as i32) as u32) {
            let mut pWin: *mut Window = unsafe { (*pExpr).y.pWin };
            unsafe {
                (*pWin).pFilter = substExpr(pSubst, unsafe { (*pWin).pFilter });
            }
            substExprList(pSubst, unsafe { (*pWin).pPartition });
            substExprList(pSubst, unsafe { (*pWin).pOrderBy });
        }
    }
    return pExpr;
}

// /* Description of the substitution */
// /* SELECT statement in which to make substitutions */
// /* Do substitutes on p->pPrior too */
// /* !defined(SQLITE_OMIT_SUBQUERY) || !defined(SQLITE_OMIT_VIEW) */
// /*
// ** pSelect is a SELECT statement and pSrcItem is one item in the FROM
// ** clause of that SELECT.
// **
// ** This routine scans the entire SELECT statement and recomputes the
// ** pSrcItem->colUsed mask.
// */
#[unsafe(link_section = ".text.slate_distinct.select.recomputeColumnsUsedExpr")]
extern "C-unwind" fn recomputeColumnsUsedExpr(
    mut pWalker: *mut Walker,
    mut pExpr: *mut Expr,
) -> i32 {
    let mut pItem: *mut SrcItem = unsafe { std::mem::zeroed() };
    if (((unsafe { (*pExpr).op }) as u32) as i32) != (168 as i32) {
        return 0 as i32;
    }
    pItem = unsafe { (*pWalker).u.pSrcItem };
    if (unsafe { (*pItem).iCursor }) != unsafe { (*pExpr).iTable } {
        return 0 as i32;
    }
    if ((unsafe { (*pExpr).iColumn }) as i32) < (0 as i32) {
        return 0 as i32;
    }
    let __v2690: *mut SrcItem = pItem;
    let __v2691: u64 = unsafe { (*__v2690).colUsed };
    let __v2692: u64 = __v2691 | unsafe { sqlite3ExprColUsed(pExpr) };
    unsafe {
        (*__v2690).colUsed = __v2692;
    }
    return 0 as i32;
}

fn recomputeColumnsUsed(mut pSelect: *mut Select, mut pSrcItem: *mut SrcItem) {
    let mut w: Walker = unsafe { std::mem::zeroed() };
    if (unsafe { (*pSrcItem).pSTab }) == std::ptr::null_mut::<Table>() {
        return;
    }
    unsafe { memset(std::ptr::addr_of_mut!(w) as *mut (), 0 as i32, 48 as u64) };
    w.xExprCallback = Some(recomputeColumnsUsedExpr);
    w.xSelectCallback = unsafe {
        std::mem::transmute::<
            *const (),
            Option<unsafe extern "C-unwind" fn(*mut Walker, *mut Select) -> i32>,
        >(sqlite3SelectWalkNoop as *const ())
    };
    unsafe {
        w.u.pSrcItem = pSrcItem;
    }
    unsafe {
        (*pSrcItem).colUsed = ((0 as i32) as i64) as u64;
    }
    unsafe { sqlite3WalkSelect(std::ptr::addr_of_mut!(w), pSelect) };
}

// /* The complete SELECT statement */
// /* Which FROM clause item to recompute */
// /* !defined(SQLITE_OMIT_SUBQUERY) || !defined(SQLITE_OMIT_VIEW) */
// /*
// ** Assign new cursor numbers to each of the items in pSrc. For each
// ** new cursor number assigned, set an entry in the aCsrMap[] array
// ** to map the old cursor number to the new:
// **
// **     aCsrMap[iOld+1] = iNew;
// **
// ** The array is guaranteed by the caller to be large enough for all
// ** existing cursor numbers in pSrc.  aCsrMap[0] is the array size.
// **
// ** If pSrc contains any sub-selects, call this routine recursively
// ** on the FROM clause of each such sub-select, with iExcept set to -1.
// */
fn srclistRenumberCursors(
    mut pParse: *mut Parse,
    mut aCsrMap: *mut i32,
    mut pSrc: *mut SrcList,
    mut iExcept: i32,
) {
    let mut i: i32 = 0 as i32;
    let mut pItem: *mut SrcItem = unsafe { std::mem::zeroed() };
    i = 0 as i32;
    let __v2693: *mut SrcItem = unsafe { std::ptr::addr_of_mut!((*pSrc).a) as *mut SrcItem };
    pItem = __v2693;
    '__slate_break_2111: while i < unsafe { (*pSrc).nSrc } {
        if i != iExcept {
            let mut p: *mut Select = unsafe { std::mem::zeroed() };
            0 as i32;
            if !(((unsafe { (*pItem).fg.__slate_bits_0.__get_isRecursive() }) as i32) != (0 as i32))
                || (unsafe {
                    *unsafe {
                        aCsrMap.offset(((unsafe { (*pItem).iCursor }) + (1 as i32)) as isize)
                    }
                }) == (0 as i32)
            {
                let __v2698: *mut Parse = pParse;
                let __v2699: i32 = unsafe { (*__v2698).nTab };
                let __v2700: i32 = __v2699 + (1 as i32);
                unsafe {
                    (*__v2698).nTab = __v2700;
                }
                unsafe {
                    *unsafe {
                        aCsrMap.offset(((unsafe { (*pItem).iCursor }) + (1 as i32)) as isize)
                    } = __v2699;
                }
            }
            unsafe {
                (*pItem).iCursor = unsafe {
                    *unsafe {
                        aCsrMap.offset(((unsafe { (*pItem).iCursor }) + (1 as i32)) as isize)
                    }
                };
            }
            if ((unsafe { (*pItem).fg.__slate_bits_0.__get_isSubquery() }) as i32) != (0 as i32) {
                p = unsafe { (*unsafe { (*pItem).u4.pSubq }).pSelect };
                '__slate_break_2112: while p != std::ptr::null_mut::<Select>() {
                    srclistRenumberCursors(pParse, aCsrMap, unsafe { (*p).pSrc }, -(1 as i32));
                    p = unsafe { (*p).pPrior };
                }
            }
        }
        let __v2694: i32 = i;
        let __v2695: i32 = __v2694 + (1 as i32);
        i = __v2695;
        let __v2696: *mut SrcItem = pItem;
        let __v2697: *mut SrcItem = unsafe { __v2696.offset((1 as i32) as isize) };
        pItem = __v2697;
    }
}

// /* Parse context */
// /* Array to store cursor mappings in */
// /* FROM clause to renumber */
// /* FROM clause item to skip */
// /*
// ** *piCursor is a cursor number.  Change it if it needs to be mapped.
// */
fn renumberCursorDoMapping(mut pWalker: *mut Walker, mut piCursor: *mut i32) {
    let mut aCsrMap: *mut i32 = unsafe { (*pWalker).u.aiCol };
    let mut iCsr: i32 = unsafe { *piCursor };
    if iCsr < unsafe { *unsafe { aCsrMap.offset((0 as i32) as isize) } }
        && (unsafe { *unsafe { aCsrMap.offset((iCsr + (1 as i32)) as isize) } }) > (0 as i32)
    {
        unsafe {
            *piCursor = unsafe { *unsafe { aCsrMap.offset((iCsr + (1 as i32)) as isize) } };
        }
    }
}

// /*
// ** Expression walker callback used by renumberCursors() to update
// ** Expr objects to match newly assigned cursor numbers.
// */
#[unsafe(link_section = ".text.slate_distinct.select.renumberCursorsCb")]
extern "C-unwind" fn renumberCursorsCb(mut pWalker: *mut Walker, mut pExpr: *mut Expr) -> i32 {
    let mut op: i32 = ((unsafe { (*pExpr).op }) as u32) as i32;
    if op == (168 as i32) || op == (179 as i32) {
        renumberCursorDoMapping(pWalker, unsafe { std::ptr::addr_of_mut!((*pExpr).iTable) });
    }
    if (unsafe { (*pExpr).flags }) & ((1 as i32) as u32) != ((0 as i32) as u32) {
        renumberCursorDoMapping(pWalker, unsafe { std::ptr::addr_of_mut!((*pExpr).w.iJoin) });
    }
    return 0 as i32;
}

// /*
// ** Assign a new cursor number to each cursor in the FROM clause (Select.pSrc)
// ** of the SELECT statement passed as the second argument, and to each
// ** cursor in the FROM clause of any FROM clause sub-selects, recursively.
// ** Except, do not assign a new cursor number to the iExcept'th element in
// ** the FROM clause of (*p). Update all expressions and other references
// ** to refer to the new cursor numbers.
// **
// ** Argument aCsrMap is an array that may be used for temporary working
// ** space. Two guarantees are made by the caller:
// **
// **   * the array is larger than the largest cursor number used within the
// **     select statement passed as an argument, and
// **
// **   * the array entries for all cursor numbers that do *not* appear in
// **     FROM clauses of the select statement as described above are
// **     initialized to zero.
// */
fn renumberCursors(
    mut pParse: *mut Parse,
    mut p: *mut Select,
    mut iExcept: i32,
    mut aCsrMap: *mut i32,
) {
    let mut w: Walker = unsafe { std::mem::zeroed() };
    srclistRenumberCursors(pParse, aCsrMap, unsafe { (*p).pSrc }, iExcept);
    unsafe { memset(std::ptr::addr_of_mut!(w) as *mut (), 0 as i32, 48 as u64) };
    unsafe {
        w.u.aiCol = aCsrMap;
    }
    w.xExprCallback = Some(renumberCursorsCb);
    w.xSelectCallback = unsafe {
        std::mem::transmute::<
            *const (),
            Option<unsafe extern "C-unwind" fn(*mut Walker, *mut Select) -> i32>,
        >(sqlite3SelectWalkNoop as *const ())
    };
    unsafe { sqlite3WalkSelect(std::ptr::addr_of_mut!(w), p) };
}

// /* Parse context */
// /* Select to renumber cursors within */
// /* FROM clause item to skip */
// /* Working space */
// /* !defined(SQLITE_OMIT_SUBQUERY) || !defined(SQLITE_OMIT_VIEW) */
// /*
// ** If pSel is not part of a compound SELECT, return a pointer to its
// ** expression list. Otherwise, return a pointer to the expression list
// ** of the leftmost SELECT in the compound.
// */
fn findLeftmostExprlist(mut pSel: *mut Select) -> *mut ExprList {
    '__slate_break_2113: while (unsafe { (*pSel).pPrior }) != std::ptr::null_mut::<Select>() {
        pSel = unsafe { (*pSel).pPrior };
    }
    return unsafe { (*pSel).pEList };
}

// /*
// ** Return true if any of the result-set columns in the compound query
// ** have incompatible affinities on one or more arms of the compound.
// */
fn compoundHasDifferentAffinities(mut p: *mut Select) -> i32 {
    let mut ii: i32 = 0 as i32;
    let mut pList: *mut ExprList = unsafe { std::mem::zeroed() };
    0 as i32;
    0 as i32;
    0 as i32;
    pList = unsafe { (*p).pEList };
    ii = 0 as i32;
    '__slate_break_2114: loop {
        if !(ii < unsafe { (*pList).nExpr }) {
            break;
        }
        let mut aff: i8 = 0 as i8;
        let mut pSub1: *mut Select = unsafe { std::mem::zeroed() };
        0 as i32;
        aff = unsafe {
            sqlite3ExprAffinity(
                (unsafe {
                    (*unsafe {
                        unsafe { std::ptr::addr_of_mut!((*pList).a) as *mut ExprList_item }
                            .offset(ii as isize)
                    })
                    .pExpr
                }) as *const Expr,
            )
        };
        pSub1 = unsafe { (*p).pPrior };
        '__slate_break_2115: while pSub1 != std::ptr::null_mut::<Select>() {
            0 as i32;
            0 as i32;
            0 as i32;
            if ((unsafe {
                sqlite3ExprAffinity(
                    (unsafe {
                        (*unsafe {
                            unsafe {
                                std::ptr::addr_of_mut!((*unsafe { (*pSub1).pEList }).a)
                                    as *mut ExprList_item
                            }
                            .offset(ii as isize)
                        })
                        .pExpr
                    }) as *const Expr,
                )
            }) as i32)
                != (aff as i32)
            {
                return 1 as i32;
            }
            pSub1 = unsafe { (*pSub1).pPrior };
        }
        let __v2701: i32 = ii;
        let __v2702: i32 = __v2701 + (1 as i32);
        ii = __v2702;
    }
    return 0 as i32;
}

// /*
// ** This routine attempts to flatten subqueries as a performance optimization.
// ** This routine returns 1 if it makes changes and 0 if no flattening occurs.
// **
// ** To understand the concept of flattening, consider the following
// ** query:
// **
// **     SELECT a FROM (SELECT x+y AS a FROM t1 WHERE z<100) WHERE a>5
// **
// ** The default way of implementing this query is to execute the
// ** subquery first and store the results in a temporary table, then
// ** run the outer query on that temporary table.  This requires two
// ** passes over the data.  Furthermore, because the temporary table
// ** has no indices, the WHERE clause on the outer query cannot be
// ** optimized.
// **
// ** This routine attempts to rewrite queries such as the above into
// ** a single flat select, like this:
// **
// **     SELECT x+y AS a FROM t1 WHERE z<100 AND a>5
// **
// ** The code generated for this simplification gives the same result
// ** but only has to scan the data once.  And because indices might
// ** exist on the table t1, a complete scan of the data might be
// ** avoided.
// **
// ** Flattening is subject to the following constraints:
// **
// **  (**)  We no longer attempt to flatten aggregate subqueries. Was:
// **        The subquery and the outer query cannot both be aggregates.
// **
// **  (**)  We no longer attempt to flatten aggregate subqueries. Was:
// **        (2) If the subquery is an aggregate then
// **        (2a) the outer query must not be a join and
// **        (2b) the outer query must not use subqueries
// **             other than the one FROM-clause subquery that is a candidate
// **             for flattening.  (This is due to ticket [2f7170d73bf9abf80]
// **             from 2015-02-09.)
// **
// **   (3)  If the subquery is the right operand of a LEFT JOIN then
// **        (3a) the subquery may not be a join
// **        (**) Was (3b): "the FROM clause of the subquery may not contain
// **             a virtual table"
// **        (**) Was: "The outer query may not have a GROUP BY." This case
// **             is now managed correctly
// **        (3d) the outer query may not be DISTINCT.
// **        See also (26) for restrictions on RIGHT JOIN.
// **
// **   (4)  The subquery can not be DISTINCT.
// **
// **  (**)  At one point restrictions (4) and (5) defined a subset of DISTINCT
// **        sub-queries that were excluded from this optimization. Restriction
// **        (4) has since been expanded to exclude all DISTINCT subqueries.
// **
// **  (**)  We no longer attempt to flatten aggregate subqueries.  Was:
// **        If the subquery is aggregate, the outer query may not be DISTINCT.
// **
// **   (7)  The subquery must have a FROM clause.  TODO:  For subqueries without
// **        A FROM clause, consider adding a FROM clause with the special
// **        table sqlite_once that consists of a single row containing a
// **        single NULL.
// **
// **   (8)  If the subquery uses LIMIT then the outer query may not be a join.
// **
// **   (9)  If the subquery uses LIMIT then the outer query may not be aggregate.
// **
// **  (**)  Restriction (10) was removed from the code on 2005-02-05 but we
// **        accidentally carried the comment forward until 2014-09-15.  Original
// **        constraint: "If the subquery is aggregate then the outer query
// **        may not use LIMIT."
// **
// **  (11)  The subquery and the outer query may not both have ORDER BY clauses.
// **
// **  (**)  Not implemented.  Subsumed into restriction (3).  Was previously
// **        a separate restriction deriving from ticket #350.
// **
// **  (13)  The subquery and outer query may not both use LIMIT.
// **
// **  (14)  The subquery may not use OFFSET.
// **
// **  (15)  If the outer query is part of a compound select, then the
// **        subquery may not use LIMIT.
// **        (See ticket #2339 and ticket [02a8e81d44]).
// **
// **  (16)  If the outer query is aggregate, then the subquery may not
// **        use ORDER BY.  (Ticket #2942)  This used to not matter
// **        until we introduced the group_concat() function.
// **
// **  (17)  If the subquery is a compound select, then
// **        (17a) all compound operators must be a UNION ALL, and
// **        (17b) no terms within the subquery compound may be aggregate
// **              or DISTINCT, and
// **        (17c) every term within the subquery compound must have a FROM clause
// **        (17d) the outer query may not be
// **              (17d1) aggregate, or
// **              (17d2) DISTINCT
// **        (17e) the subquery may not contain window functions, and
// **        (17f) the subquery must not be the RHS of a LEFT JOIN.
// **        (17g) either the subquery is the first element of the outer
// **              query or there are no RIGHT or FULL JOINs in any arm
// **              of the subquery.  (This is a duplicate of condition (27b).)
// **        (17h) The corresponding result set expressions in all arms of the
// **              compound must have the same affinity.
// **
// **        The parent and sub-query may contain WHERE clauses. Subject to
// **        rules (11), (13) and (14), they may also contain ORDER BY,
// **        LIMIT and OFFSET clauses.  The subquery cannot use any compound
// **        operator other than UNION ALL because all the other compound
// **        operators have an implied DISTINCT which is disallowed by
// **        restriction (4).
// **
// **        Also, each component of the sub-query must return the same number
// **        of result columns. This is actually a requirement for any compound
// **        SELECT statement, but all the code here does is make sure that no
// **        such (illegal) sub-query is flattened. The caller will detect the
// **        syntax error and return a detailed message.
// **
// **  (18)  If the sub-query is a compound select, then all terms of the
// **        ORDER BY clause of the parent must be copies of a term returned
// **        by the parent query.
// **
// **  (19)  If the subquery uses LIMIT then the outer query may not
// **        have a WHERE clause.
// **
// **  (20)  If the sub-query is a compound select, then it must not use
// **        an ORDER BY clause.  Ticket #3773.  We could relax this constraint
// **        somewhat by saying that the terms of the ORDER BY clause must
// **        appear as unmodified result columns in the outer query.  But we
// **        have other optimizations in mind to deal with that case.
// **
// **  (21)  If the subquery uses LIMIT then the outer query may not be
// **        DISTINCT.  (See ticket [752e1646fc]).
// **
// **  (22)  The subquery may not be a recursive CTE.
// **
// **  (23)  If the outer query is a recursive CTE, then the sub-query may not be
// **        a compound query.  This restriction is because transforming the
// **        parent to a compound query confuses the code that handles
// **        recursive queries in multiSelect().
// **
// **  (**)  We no longer attempt to flatten aggregate subqueries.  Was:
// **        The subquery may not be an aggregate that uses the built-in min() or
// **        or max() functions.  (Without this restriction, a query like:
// **        "SELECT x FROM (SELECT max(y), x FROM t1)" would not necessarily
// **        return the value X for which Y was maximal.)
// **
// **  (25)  If either the subquery or the parent query contains a window
// **        function in the select list or ORDER BY clause, flattening
// **        is not attempted.
// **
// **  (26)  The subquery may not be the right operand of a RIGHT JOIN.
// **        See also (3) for restrictions on LEFT JOIN.
// **
// **  (27)  The subquery may not contain a FULL or RIGHT JOIN unless it
// **        is the first element of the parent query.  Two subcases:
// **        (27a) the subquery is not a compound query.
// **        (27b) the subquery is a compound query and the RIGHT JOIN occurs
// **              in any arm of the compound query.  (See also (17g).)
// **
// **  (28)  The subquery is not a MATERIALIZED CTE.  (This is handled
// **        in the caller before ever reaching this routine.)
// **
// **
// ** In this routine, the "p" parameter is a pointer to the outer query.
// ** The subquery is p->pSrc->a[iFrom].  isAgg is true if the outer query
// ** uses aggregates.
// **
// ** If flattening is not attempted, this routine is a no-op and returns 0.
// ** If flattening is attempted this routine returns 1.
// **
// ** All of the expression analysis must occur on both the outer query and
// ** the subquery before this routine runs.
// */
fn flattenSubquery(
    mut pParse: *mut Parse,
    mut p: *mut Select,
    mut iFrom: i32,
    mut isAgg: i32,
) -> i32 {
    let mut zSavedAuthContext: *const i8 = unsafe { (*pParse).zAuthContext };
    // /* Current UNION ALL term of the other query */
    let mut pParent: *mut Select = unsafe { std::mem::zeroed() };
    // /* The inner query or "subquery" */
    let mut pSub: *mut Select = unsafe { std::mem::zeroed() };
    // /* Pointer to the rightmost select in sub-query */
    let mut pSub1: *mut Select = unsafe { std::mem::zeroed() };
    // /* The FROM clause of the outer query */
    let mut pSrc: *mut SrcList = unsafe { std::mem::zeroed() };
    // /* The FROM clause of the subquery */
    let mut pSubSrc: *mut SrcList = unsafe { std::mem::zeroed() };
    // /* VDBE cursor number of the pSub result set temp table */
    let mut iParent: i32 = 0 as i32;
    // /* Replacement table for iParent */
    let mut iNewParent: i32 = -(1 as i32);
    // /* True if pSub is the right side of a LEFT JOIN */
    let mut isOuterJoin: i32 = 0 as i32;
    // /* Loop counter */
    let mut i: i32 = 0 as i32;
    // /* The WHERE clause */
    let mut pWhere: *mut Expr = unsafe { std::mem::zeroed() };
    // /* The subquery */
    let mut pSubitem: *mut SrcItem = unsafe { std::mem::zeroed() };
    let mut db: *mut sqlite3 = unsafe { (*pParse).db };
    // /* Walker to persist agginfo data */
    let mut w: Walker = unsafe { std::mem::zeroed() };
    let mut aCsrMap: *mut i32 = std::ptr::null_mut::<i32>();
    // /* Check to see if flattening is permitted.  Return 0 if not.
    //   */
    0 as i32;
    0 as i32;
    if (unsafe { (*db).dbOptFlags }) & ((1 as i32) as u32) != ((0 as i32) as u32) {
        return 0 as i32;
    }
    pSrc = unsafe { (*p).pSrc };
    0 as i32;
    pSubitem = unsafe {
        unsafe { std::ptr::addr_of_mut!((*pSrc).a) as *mut SrcItem }.offset(iFrom as isize)
    };
    iParent = unsafe { (*pSubitem).iCursor };
    0 as i32;
    pSub = unsafe { (*unsafe { (*pSubitem).u4.pSubq }).pSelect };
    0 as i32;
    // /* Restriction (25) */
    if (unsafe { (*p).pWin }) != std::ptr::null_mut::<Window>()
        || (unsafe { (*pSub).pWin }) != std::ptr::null_mut::<Window>()
    {
        return 0 as i32;
    }
    pSubSrc = unsafe { (*pSub).pSrc };
    0 as i32;
    // /* Prior to version 3.1.2, when LIMIT and OFFSET had to be simple constants,
    //   ** not arbitrary expressions, we allowed some combining of LIMIT and OFFSET
    //   ** because they could be computed at compile-time.  But when LIMIT and OFFSET
    //   ** became arbitrary expressions, we were forced to add restrictions (13)
    //   ** and (14). */
    // /* Restriction (13) */
    if (unsafe { (*pSub).pLimit }) != std::ptr::null_mut::<Expr>()
        && (unsafe { (*p).pLimit }) != std::ptr::null_mut::<Expr>()
    {
        return 0 as i32;
    }
    // /* Restriction (14) */
    if (unsafe { (*pSub).pLimit }) != std::ptr::null_mut::<Expr>()
        && (unsafe { (*unsafe { (*pSub).pLimit }).pRight }) != std::ptr::null_mut::<Expr>()
    {
        return 0 as i32;
    }
    if (unsafe { (*p).selFlags }) & ((256 as i32) as u32) != ((0 as i32) as u32)
        && (unsafe { (*pSub).pLimit }) != std::ptr::null_mut::<Expr>()
    {
        // /* Restriction (15) */
        return 0 as i32;
    }
    // /* Restriction (7)  */
    if (unsafe { (*pSubSrc).nSrc }) == (0 as i32) {
        return 0 as i32;
    }
    // /* Restriction (4)  */
    if (unsafe { (*pSub).selFlags }) & ((1 as i32) as u32) != (0 as u32) {
        return 0 as i32;
    }
    if (unsafe { (*pSub).pLimit }) != std::ptr::null_mut::<Expr>()
        && ((unsafe { (*pSrc).nSrc }) > (1 as i32) || isAgg != (0 as i32))
    {
        // /* Restrictions (8)(9) */
        return 0 as i32;
    }
    if (unsafe { (*p).pOrderBy }) != std::ptr::null_mut::<ExprList>()
        && (unsafe { (*pSub).pOrderBy }) != std::ptr::null_mut::<ExprList>()
    {
        // /* Restriction (11) */
        return 0 as i32;
    }
    // /* Restriction (16) */
    if isAgg != (0 as i32) && (unsafe { (*pSub).pOrderBy }) != std::ptr::null_mut::<ExprList>() {
        return 0 as i32;
    }
    // /* Restriction (19) */
    if (unsafe { (*pSub).pLimit }) != std::ptr::null_mut::<Expr>()
        && (unsafe { (*p).pWhere }) != std::ptr::null_mut::<Expr>()
    {
        return 0 as i32;
    }
    if (unsafe { (*pSub).pLimit }) != std::ptr::null_mut::<Expr>()
        && (unsafe { (*p).selFlags }) & ((1 as i32) as u32) != ((0 as i32) as u32)
    {
        // /* Restriction (21) */
        return 0 as i32;
    }
    if (unsafe { (*pSub).selFlags }) & ((8192 as i32) as u32) != (0 as u32) {
        // /* Restrictions (22) */
        return 0 as i32;
    }
    // /*
    //   ** If the subquery is the right operand of a LEFT JOIN, then the
    //   ** subquery may not be a join itself (3a). Example of why this is not
    //   ** allowed:
    //   **
    //   **         t1 LEFT OUTER JOIN (t2 JOIN t3)
    //   **
    //   ** If we flatten the above, we would get
    //   **
    //   **         (t1 LEFT OUTER JOIN t2) JOIN t3
    //   **
    //   ** which is not at all the same thing.
    //   **
    //   ** See also tickets #306, #350, and #3300.
    //   */
    if (((unsafe { (*pSubitem).fg.jointype }) as u32) as i32) & ((32 as i32) | (64 as i32))
        != (0 as i32)
    {
        // /* (3a) */
        if (unsafe { (*pSubSrc).nSrc }) > (1 as i32)
            || (unsafe { (*p).selFlags }) & ((1 as i32) as u32) != ((0 as i32) as u32)
            || (((unsafe { (*pSubitem).fg.jointype }) as u32) as i32) & (16 as i32) != (0 as i32)
        {
            return 0 as i32;
        }
        // /**** || IsVirtual(pSubSrc->a[0].pSTab)      (3b)-omitted */
        // /* (3d) */
        // /* (26) */
        isOuterJoin = 1 as i32;
    }
    // /* True by restriction (7) */
    0 as i32;
    if iFrom > (0 as i32)
        && (((unsafe {
            (*unsafe {
                unsafe { std::ptr::addr_of_mut!((*pSubSrc).a) as *mut SrcItem }
                    .offset((0 as i32) as isize)
            })
            .fg
            .jointype
        }) as u32) as i32)
            & (64 as i32)
            != (0 as i32)
    {
        // /* Restriction (27a) */
        return 0 as i32;
    }
    // /* Condition (28) is blocked by the caller */
    0 as i32;
    // /* Restriction (17): If the sub-query is a compound SELECT, then it must
    //   ** use only the UNION ALL operator. And none of the simple select queries
    //   ** that make up the compound SELECT are allowed to be aggregate or distinct
    //   ** queries.
    //   */
    if (unsafe { (*pSub).pPrior }) != std::ptr::null_mut::<Select>() {
        let mut ii: i32 = 0 as i32;
        if (unsafe { (*pSub).pOrderBy }) != std::ptr::null_mut::<ExprList>() {
            // /* Restriction (20) */
            return 0 as i32;
        }
        if isAgg != (0 as i32)
            || (unsafe { (*p).selFlags }) & ((1 as i32) as u32) != ((0 as i32) as u32)
            || isOuterJoin > (0 as i32)
        {
            // /* (17d1), (17d2), or (17f) */
            return 0 as i32;
        }
        pSub1 = pSub;
        '__slate_break_2116: while pSub1 != std::ptr::null_mut::<Select>() {
            {}
            {}
            0 as i32;
            0 as i32;
            0 as i32;
            // /* (17b) */
            if (unsafe { (*pSub1).selFlags }) & (((1 as i32) | (8 as i32)) as u32)
                != ((0 as i32) as u32)
                || (unsafe { (*pSub1).pPrior }) != std::ptr::null_mut::<Select>()
                    && (((unsafe { (*pSub1).op }) as u32) as i32) != (136 as i32)
                || (unsafe { (*unsafe { (*pSub1).pSrc }).nSrc }) < (1 as i32)
                || (unsafe { (*pSub1).pWin }) != std::ptr::null_mut::<Window>()
            {
                return 0 as i32;
            }
            // /* (17a) */
            // /* (17c) */
            // /* (17e) */
            if iFrom > (0 as i32)
                && (((unsafe {
                    (*unsafe {
                        unsafe {
                            std::ptr::addr_of_mut!((*unsafe { (*pSub1).pSrc }).a) as *mut SrcItem
                        }
                        .offset((0 as i32) as isize)
                    })
                    .fg
                    .jointype
                }) as u32) as i32)
                    & (64 as i32)
                    != (0 as i32)
            {
                // /* Without this restriction, the JT_LTORJ flag would end up being
                //         ** omitted on left-hand tables of the right join that is being
                //         ** flattened. */
                // /* Restrictions (17g), (27b) */
                return 0 as i32;
            }
            {}
            pSub1 = unsafe { (*pSub1).pPrior };
        }
        // /* Restriction (18). */
        if (unsafe { (*p).pOrderBy }) != std::ptr::null_mut::<ExprList>() {
            ii = 0 as i32;
            '__slate_break_2117: loop {
                if !(ii < unsafe { (*unsafe { (*p).pOrderBy }).nExpr }) {
                    break;
                }
                if (((unsafe {
                    (*unsafe {
                        unsafe {
                            std::ptr::addr_of_mut!((*unsafe { (*p).pOrderBy }).a)
                                as *mut ExprList_item
                        }
                        .offset(ii as isize)
                    })
                    .u
                    .x
                    .iOrderByCol
                }) as u32) as i32)
                    == (0 as i32)
                {
                    return 0 as i32;
                }
                let __v2703: i32 = ii;
                let __v2704: i32 = __v2703 + (1 as i32);
                ii = __v2704;
            }
        }
        // /* Restriction (23) */
        if (unsafe { (*p).selFlags }) & ((8192 as i32) as u32) != (0 as u32) {
            return 0 as i32;
        }
        // /* Restriction (17h) */
        if compoundHasDifferentAffinities(pSub) != (0 as i32) {
            return 0 as i32;
        }
        if (unsafe { (*pSrc).nSrc }) > (1 as i32) {
            if (unsafe { (*pParse).nSelect }) > (500 as i32) {
                return 0 as i32;
            }
            if (unsafe { (*db).dbOptFlags }) & ((8388608 as i32) as u32) != ((0 as i32) as u32) {
                return 0 as i32;
            }
            aCsrMap = (unsafe {
                sqlite3DbMallocZero(
                    db,
                    ((((unsafe { (*pParse).nTab }) as i64) + ((1 as i32) as i64)) as u64)
                        .wrapping_mul(4 as u64),
                )
            }) as *mut i32;
            if aCsrMap != std::ptr::null_mut::<i32>() {
                unsafe {
                    *unsafe { aCsrMap.offset((0 as i32) as isize) } = unsafe { (*pParse).nTab };
                }
            }
        }
    }
    // /***** If we reach this point, flattening is permitted. *****/
    {}
    // /* Authorize the subquery */
    unsafe {
        (*pParse).zAuthContext = (unsafe { (*pSubitem).zName }) as *const i8;
    }
    unsafe {
        sqlite3AuthCheck(
            pParse,
            21 as i32,
            std::ptr::null::<i8>(),
            std::ptr::null::<i8>(),
            std::ptr::null::<i8>(),
        )
    };
    {}
    unsafe {
        (*pParse).zAuthContext = zSavedAuthContext;
    }
    // /* Delete the transient structures associated with the subquery */
    if ((unsafe { (*pSubitem).fg.__slate_bits_0.__get_isSubquery() }) as i32) != (0 as i32) {
        pSub1 = unsafe { sqlite3SubqueryDetach(db, pSubitem) };
    } else {
        pSub1 = std::ptr::null_mut::<Select>();
    }
    0 as i32;
    0 as i32;
    unsafe { sqlite3DbFree(db, (unsafe { (*pSubitem).zName }) as *mut ()) };
    unsafe { sqlite3DbFree(db, (unsafe { (*pSubitem).zAlias }) as *mut ()) };
    unsafe {
        (*pSubitem).zName = std::ptr::null_mut::<i8>();
    }
    unsafe {
        (*pSubitem).zAlias = std::ptr::null_mut::<i8>();
    }
    0 as i32;
    // /* If the sub-query is a compound SELECT statement, then (by restrictions
    //   ** 17 and 18 above) it must be a UNION ALL and the parent query must
    //   ** be of the form:
    //   **
    //   **     SELECT <expr-list> FROM (<sub-query>) <where-clause>
    //   **
    //   ** followed by any ORDER BY, LIMIT and/or OFFSET clauses. This block
    //   ** creates N-1 copies of the parent query without any ORDER BY, LIMIT or
    //   ** OFFSET clauses and joins them to the left-hand-side of the original
    //   ** using UNION ALL operators. In this case N is the number of simple
    //   ** select statements in the compound sub-query.
    //   **
    //   ** Example:
    //   **
    //   **     SELECT a+1 FROM (
    //   **        SELECT x FROM tab
    //   **        UNION ALL
    //   **        SELECT y FROM tab
    //   **        UNION ALL
    //   **        SELECT abs(z*2) FROM tab2
    //   **     ) WHERE a!=5 ORDER BY 1
    //   **
    //   ** Transformed into:
    //   **
    //   **     SELECT x+1 FROM tab WHERE x+1!=5
    //   **     UNION ALL
    //   **     SELECT y+1 FROM tab WHERE y+1!=5
    //   **     UNION ALL
    //   **     SELECT abs(z*2)+1 FROM tab2 WHERE abs(z*2)+1!=5
    //   **     ORDER BY 1
    //   **
    //   ** We call this the "compound-subquery flattening".
    //   */
    pSub = unsafe { (*pSub).pPrior };
    '__slate_break_2118: while pSub != std::ptr::null_mut::<Select>() {
        let mut pNew: *mut Select = unsafe { std::mem::zeroed() };
        let mut pOrderBy: *mut ExprList = unsafe { (*p).pOrderBy };
        let mut pLimit: *mut Expr = unsafe { (*p).pLimit };
        let mut pPrior: *mut Select = unsafe { (*p).pPrior };
        let mut pItemTab: *mut Table = unsafe { (*pSubitem).pSTab };
        unsafe {
            (*pSubitem).pSTab = std::ptr::null_mut::<Table>();
        }
        unsafe {
            (*p).pOrderBy = std::ptr::null_mut::<ExprList>();
        }
        unsafe {
            (*p).pPrior = std::ptr::null_mut::<Select>();
        }
        unsafe {
            (*p).pLimit = std::ptr::null_mut::<Expr>();
        }
        pNew = unsafe { sqlite3SelectDup(db, p as *const Select, 0 as i32) };
        unsafe {
            (*p).pLimit = pLimit;
        }
        unsafe {
            (*p).pOrderBy = pOrderBy;
        }
        unsafe {
            (*p).op = ((136 as i32) as i8) as u8;
        }
        unsafe {
            (*pSubitem).pSTab = pItemTab;
        }
        if pNew == std::ptr::null_mut::<Select>() {
            unsafe {
                (*p).pPrior = pPrior;
            }
        } else {
            let __v2705: *mut Parse = pParse;
            let __v2706: i32 = unsafe { (*__v2705).nSelect };
            let __v2707: i32 = __v2706 + (1 as i32);
            unsafe {
                (*__v2705).nSelect = __v2707;
            }
            unsafe {
                (*pNew).selId = __v2707 as u32;
            }
            if aCsrMap != std::ptr::null_mut::<i32>()
                && (((unsafe { (*db).mallocFailed }) as u32) as i32) == (0 as i32)
            {
                renumberCursors(pParse, pNew, iFrom, aCsrMap);
            }
            unsafe {
                (*pNew).pPrior = pPrior;
            }
            if pPrior != std::ptr::null_mut::<Select>() {
                unsafe {
                    (*pPrior).pNext = pNew;
                }
            }
            unsafe {
                (*pNew).pNext = p;
            }
            unsafe {
                (*p).pPrior = pNew;
            }
            {}
        }
        0 as i32;
        pSub = unsafe { (*pSub).pPrior };
    }
    unsafe { sqlite3DbFree(db, aCsrMap as *mut ()) };
    if (unsafe { (*db).mallocFailed }) != (0 as u8) {
        0 as i32;
        0 as i32;
        0 as i32;
        unsafe { sqlite3SrcItemAttachSubquery(pParse, pSubitem, pSub1, 0 as i32) };
        return 1 as i32;
    }
    // /* Defer deleting the Table object associated with the
    //   ** subquery until code generation is
    //   ** complete, since there may still exist Expr.pTab entries that
    //   ** refer to the subquery even after flattening.  Ticket #3346.
    //   **
    //   ** pSubitem->pSTab is always non-NULL by test restrictions and tests above.
    //   */
    if (unsafe { (*pSubitem).pSTab }) != std::ptr::null_mut::<Table>() {
        let mut pTabToDel: *mut Table = unsafe { (*pSubitem).pSTab };
        if (unsafe { (*pTabToDel).nTabRef }) == ((1 as i32) as u32) {
            let mut pToplevel: *mut Parse =
                if (unsafe { (*pParse).pToplevel }) != std::ptr::null_mut::<Parse>() {
                    unsafe { (*pParse).pToplevel }
                } else {
                    pParse
                };
            unsafe {
                sqlite3ParserAddCleanup(
                    pToplevel,
                    unsafe {
                        std::mem::transmute::<
                            *const (),
                            Option<unsafe extern "C-unwind" fn(*mut sqlite3, *mut ())>,
                        >(sqlite3DeleteTableGeneric as *const ())
                    },
                    pTabToDel as *mut (),
                )
            };
            {}
        } else {
            let __v2708: *mut Table = pTabToDel;
            let __v2709: u32 = unsafe { (*__v2708).nTabRef };
            let __v2710: u32 = __v2709.wrapping_sub((1 as i32) as u32);
            unsafe {
                (*__v2708).nTabRef = __v2710;
            }
        }
        unsafe {
            (*pSubitem).pSTab = std::ptr::null_mut::<Table>();
        }
    }
    // /* The following loop runs once for each term in a compound-subquery
    //   ** flattening (as described above).  If we are doing a different kind
    //   ** of flattening - a flattening other than a compound-subquery flattening -
    //   ** then this loop only runs once.
    //   **
    //   ** This loop moves all of the FROM elements of the subquery into the
    //   ** the FROM clause of the outer query.  Before doing this, remember
    //   ** the cursor number for the original outer query FROM element in
    //   ** iParent.  The iParent cursor will never be used.  Subsequent code
    //   ** will scan expressions looking for iParent references and replace
    //   ** those references with expressions that resolve to the subquery FROM
    //   ** elements we are now copying in.
    //   */
    pSub = pSub1;
    pParent = p;
    '__slate_break_2119: loop {
        if !(pParent != std::ptr::null_mut::<Select>()) {
            break;
        }
        let mut nSubSrc: i32 = 0 as i32;
        let mut jointype: u8 = unsafe { (*pSubitem).fg.jointype };
        0 as i32;
        // /* FROM clause of subquery */
        pSubSrc = unsafe { (*pSub).pSrc };
        // /* Number of terms in subquery FROM clause */
        nSubSrc = unsafe { (*pSubSrc).nSrc };
        // /* FROM clause of the outer query */
        pSrc = unsafe { (*pParent).pSrc };
        // /* The subquery uses a single slot of the FROM clause of the outer
        //     ** query.  If the subquery has more than one element in its FROM clause,
        //     ** then expand the outer query to make space for it to hold all elements
        //     ** of the subquery.
        //     **
        //     ** Example:
        //     **
        //     **    SELECT * FROM tabA, (SELECT * FROM sub1, sub2), tabB;
        //     **
        //     ** The outer query has 3 slots in its FROM clause.  One slot of the
        //     ** outer query (the middle slot) is used by the subquery.  The next
        //     ** block of code will expand the outer query FROM clause to 4 slots.
        //     ** The middle slot is expanded to two slots in order to make space
        //     ** for the two elements in the FROM clause of the subquery.
        //     */
        if nSubSrc > (1 as i32) {
            pSrc = unsafe {
                sqlite3SrcListEnlarge(pParse, pSrc, nSubSrc - (1 as i32), iFrom + (1 as i32))
            };
            if pSrc == std::ptr::null_mut::<SrcList>() {
                break '__slate_break_2119;
            }
            unsafe {
                (*pParent).pSrc = pSrc;
            }
            pSubitem = unsafe {
                unsafe { std::ptr::addr_of_mut!((*pSrc).a) as *mut SrcItem }.offset(iFrom as isize)
            };
        }
        // /* Transfer the FROM clause terms from the subquery into the
        //     ** outer query.
        //     */
        iNewParent = unsafe {
            (*unsafe {
                unsafe { std::ptr::addr_of_mut!((*pSubSrc).a) as *mut SrcItem }
                    .offset((0 as i32) as isize)
            })
            .iCursor
        };
        i = 0 as i32;
        '__slate_break_2120: loop {
            if !(i < nSubSrc) {
                break;
            }
            let mut pItem: *mut SrcItem = unsafe {
                unsafe { std::ptr::addr_of_mut!((*pSrc).a) as *mut SrcItem }
                    .offset((i + iFrom) as isize)
            };
            0 as i32;
            0 as i32;
            if ((unsafe { (*pItem).fg.__slate_bits_0.__get_isUsing() }) as i32) != (0 as i32) {
                unsafe { sqlite3IdListDelete(db, unsafe { (*pItem).u3.pUsing }) };
            }
            unsafe {
                *pItem = unsafe {
                    *unsafe {
                        unsafe { std::ptr::addr_of_mut!((*pSubSrc).a) as *mut SrcItem }
                            .offset(i as isize)
                    }
                };
            }
            let __v2714: *mut SrcItem = pItem;
            let __v2715: u8 = unsafe { (*__v2714).fg.jointype };
            let __v2716: u8 = ((((__v2715 as u32) as i32)
                | ((jointype as u32) as i32) & (64 as i32)) as i8)
                as u8;
            unsafe {
                (*__v2714).fg.jointype = __v2716;
            }
            unsafe {
                memset(
                    (unsafe {
                        unsafe { std::ptr::addr_of_mut!((*pSubSrc).a) as *mut SrcItem }
                            .offset(i as isize)
                    }) as *mut (),
                    0 as i32,
                    72 as u64,
                )
            };
            let __v2712: i32 = i;
            let __v2713: i32 = __v2712 + (1 as i32);
            i = __v2713;
        }
        let __v2717: *mut SrcItem = pSubitem;
        let __v2718: u8 = unsafe { (*__v2717).fg.jointype };
        let __v2719: u8 = ((((__v2718 as u32) as i32) | ((jointype as u32) as i32)) as i8) as u8;
        unsafe {
            (*__v2717).fg.jointype = __v2719;
        }
        // /* Begin substituting subquery result set expressions for
        //     ** references to the iParent in the outer query.
        //     **
        //     ** Example:
        //     **
        //     **   SELECT a+5, b*10 FROM (SELECT x*3 AS a, y+10 AS b FROM t1) WHERE a>b;
        //     **   \                     \_____________ subquery __________/          /
        //     **    \_____________________ outer query ______________________________/
        //     **
        //     ** We look at every expression in the outer query and every place we see
        //     ** "a" we substitute "x*3" and every place we see "b" we substitute "y+10".
        //     */
        if (unsafe { (*pSub).pOrderBy }) != std::ptr::null_mut::<ExprList>() {
            // /* At this point, any non-zero iOrderByCol values indicate that the
            //       ** ORDER BY column expression is identical to the iOrderByCol'th
            //       ** expression returned by SELECT statement pSub. Since these values
            //       ** do not necessarily correspond to columns in SELECT statement pParent,
            //       ** zero them before transferring the ORDER BY clause.
            //       **
            //       ** Not doing this may cause an error if a subsequent call to this
            //       ** function attempts to flatten a compound sub-query into pParent.
            //       ** See ticket [d11a6e908f].
            //       */
            let mut pOrderBy: *mut ExprList = unsafe { (*pSub).pOrderBy };
            i = 0 as i32;
            '__slate_break_2121: loop {
                if !(i < unsafe { (*pOrderBy).nExpr }) {
                    break;
                }
                unsafe {
                    (*unsafe {
                        unsafe { std::ptr::addr_of_mut!((*pOrderBy).a) as *mut ExprList_item }
                            .offset(i as isize)
                    })
                    .u
                    .x
                    .iOrderByCol = ((0 as i32) as i16) as u16;
                }
                let __v2720: i32 = i;
                let __v2721: i32 = __v2720 + (1 as i32);
                i = __v2721;
            }
            0 as i32;
            unsafe {
                (*pParent).pOrderBy = pOrderBy;
            }
            unsafe {
                (*pSub).pOrderBy = std::ptr::null_mut::<ExprList>();
            }
        }
        pWhere = unsafe { (*pSub).pWhere };
        unsafe {
            (*pSub).pWhere = std::ptr::null_mut::<Expr>();
        }
        if isOuterJoin > (0 as i32) {
            0 as i32;
            sqlite3SetJoinExpr(pWhere, iNewParent, (1 as i32) as u32);
        }
        if pWhere != std::ptr::null_mut::<Expr>() {
            if (unsafe { (*pParent).pWhere }) != std::ptr::null_mut::<Expr>() {
                unsafe {
                    (*pParent).pWhere = unsafe {
                        sqlite3PExpr(pParse, 44 as i32, pWhere, unsafe { (*pParent).pWhere })
                    };
                }
            } else {
                unsafe {
                    (*pParent).pWhere = pWhere;
                }
            }
        }
        if (((unsafe { (*db).mallocFailed }) as u32) as i32) == (0 as i32) {
            let mut x: SubstContext = unsafe { std::mem::zeroed() };
            x.pParse = pParse;
            x.iTable = iParent;
            x.iNewTable = iNewParent;
            x.isOuterJoin = isOuterJoin;
            x.nSelDepth = 0 as i32;
            x.pEList = unsafe { (*pSub).pEList };
            x.pCList = findLeftmostExprlist(pSub);
            substSelect(std::ptr::addr_of_mut!(x), pParent, 0 as i32);
        }
        // /* The flattened query is a compound if either the inner or the
        //     ** outer query is a compound. */
        let __v2722: *mut Select = pParent;
        let __v2723: u32 = unsafe { (*__v2722).selFlags };
        let __v2724: u32 = __v2723 | (unsafe { (*pSub).selFlags }) & ((256 as i32) as u32);
        unsafe {
            (*__v2722).selFlags = __v2724;
        }
        // /* restriction (17b) */
        0 as i32;
        // /*
        //     ** SELECT ... FROM (SELECT ... LIMIT a OFFSET b) LIMIT x OFFSET y;
        //     **
        //     ** One is tempted to try to add a and b to combine the limits.  But this
        //     ** does not work if either limit is negative.
        //     */
        if (unsafe { (*pSub).pLimit }) != std::ptr::null_mut::<Expr>() {
            unsafe {
                (*pParent).pLimit = unsafe { (*pSub).pLimit };
            }
            unsafe {
                (*pSub).pLimit = std::ptr::null_mut::<Expr>();
            }
        }
        // /* Recompute the SrcItem.colUsed masks for the flattened
        //     ** tables. */
        i = 0 as i32;
        '__slate_break_2122: loop {
            if !(i < nSubSrc) {
                break;
            }
            recomputeColumnsUsed(pParent, unsafe {
                unsafe { std::ptr::addr_of_mut!((*pSrc).a) as *mut SrcItem }
                    .offset((i + iFrom) as isize)
            });
            let __v2725: i32 = i;
            let __v2726: i32 = __v2725 + (1 as i32);
            i = __v2726;
        }
        pParent = unsafe { (*pParent).pPrior };
        let __v2711: *mut Select = unsafe { (*pSub).pPrior };
        pSub = __v2711;
    }
    // /* Finally, delete what is left of the subquery and return success.
    //   */
    unsafe { sqlite3AggInfoPersistWalkerInit(std::ptr::addr_of_mut!(w), pParse) };
    unsafe { sqlite3WalkSelect(std::ptr::addr_of_mut!(w), pSub1) };
    sqlite3SelectDelete(db, pSub1);
    return 1 as i32;
}

// /*
// ** Add a new entry to the pConst object, if appropriate.
// **
// **    *   Do not add if pValue is not constant.  (This is enforced by
// **        the caller)
// **
// **    *   Do not add duplicate pColumn entries
// **
// **    *   Do not add if pValue has an affinity
// **
// **    *   Do not add if the comparison uses a collating sequence other
// **        than binary.
// **
// **    *   Do not add if pValue is a function that might return a subtype
// **
// ** The caller guarantees the pColumn is a column.
// */
fn constInsert(
    mut pConst: *mut WhereConst,
    mut pColumn: *mut Expr,
    mut pValue: *mut Expr,
    mut pExpr: *mut Expr,
) {
    let mut i: i32 = 0 as i32;
    0 as i32;
    0 as i32;
    if (unsafe { (*pColumn).flags }) & ((32 as i32) as u32) != ((0 as i32) as u32) {
        return;
    }
    if ((unsafe { sqlite3ExprAffinity(pValue as *const Expr) }) as i32) != (0 as i32) {
        return;
    }
    if !((unsafe {
        sqlite3IsBinary(
            (unsafe {
                sqlite3ExprCompareCollSeq(unsafe { (*pConst).pParse }, pExpr as *const Expr)
            }) as *const CollSeq,
        )
    }) != (0 as i32))
    {
        return;
    }
    if (unsafe { sqlite3ExprCanReturnSubtype(unsafe { (*pConst).pParse }, pValue) }) != (0 as i32) {
        return;
    }
    // /* 2018-10-25 ticket [cf5ed20f]
    //   ** Make sure the same pColumn is not inserted more than once */
    i = 0 as i32;
    '__slate_break_2123: loop {
        if !(i < unsafe { (*pConst).nConst }) {
            break;
        }
        let mut pE2: *const Expr =
            (unsafe { *unsafe { unsafe { (*pConst).apExpr }.offset((i * (2 as i32)) as isize) } })
                as *const Expr;
        0 as i32;
        if (unsafe { (*pE2).iTable }) == unsafe { (*pColumn).iTable }
            && ((unsafe { (*pE2).iColumn }) as i32) == ((unsafe { (*pColumn).iColumn }) as i32)
        {
            // /* Already present.  Return without doing anything. */
            return;
        }
        let __v2727: i32 = i;
        let __v2728: i32 = __v2727 + (1 as i32);
        i = __v2728;
    }
    0 as i32;
    if ((unsafe { sqlite3ExprAffinity(pColumn as *const Expr) }) as i32) <= (65 as i32) {
        unsafe {
            (*pConst).bHasAffBlob = 1 as i32;
        }
    }
    let __v2729: *mut WhereConst = pConst;
    let __v2730: i32 = unsafe { (*__v2729).nConst };
    let __v2731: i32 = __v2730 + (1 as i32);
    unsafe {
        (*__v2729).nConst = __v2731;
    }
    unsafe {
        (*pConst).apExpr = (unsafe {
            sqlite3DbReallocOrFree(
                unsafe { (*unsafe { (*pConst).pParse }).db },
                (unsafe { (*pConst).apExpr }) as *mut (),
                ((((unsafe { (*pConst).nConst }) * (2 as i32)) as i64) as u64)
                    .wrapping_mul(8 as u64),
            )
        }) as *mut *mut Expr;
    }
    if (unsafe { (*pConst).apExpr }) == std::ptr::null_mut::<*mut Expr>() {
        unsafe {
            (*pConst).nConst = 0 as i32;
        }
    } else {
        unsafe {
            *unsafe {
                unsafe { (*pConst).apExpr }
                    .offset(((unsafe { (*pConst).nConst }) * (2 as i32) - (2 as i32)) as isize)
            } = pColumn;
        }
        unsafe {
            *unsafe {
                unsafe { (*pConst).apExpr }
                    .offset(((unsafe { (*pConst).nConst }) * (2 as i32) - (1 as i32)) as isize)
            } = pValue;
        }
    }
}

// /* The WhereConst into which we are inserting */
// /* The COLUMN part of the constraint */
// /* The VALUE part of the constraint */
// /* Overall expression: COLUMN=VALUE or VALUE=COLUMN */
// /*
// ** Find all terms of COLUMN=VALUE or VALUE=COLUMN in pExpr where VALUE
// ** is a constant expression and where the term must be true because it
// ** is part of the AND-connected terms of the expression.  For each term
// ** found, add it to the pConst structure.
// */
fn findConstInWhere(mut pConst: *mut WhereConst, mut pExpr: *mut Expr) {
    let mut pRight: *mut Expr = unsafe { std::mem::zeroed() };
    let mut pLeft: *mut Expr = unsafe { std::mem::zeroed() };
    if pExpr == std::ptr::null_mut::<Expr>() {
        return;
    }
    if (unsafe { (*pExpr).flags }) & unsafe { (*pConst).mExcludeOn } != ((0 as i32) as u32) {
        {}
        {}
        return;
    }
    if (((unsafe { (*pExpr).op }) as u32) as i32) == (44 as i32) {
        findConstInWhere(pConst, unsafe { (*pExpr).pRight });
        findConstInWhere(pConst, unsafe { (*pExpr).pLeft });
        return;
    }
    if (((unsafe { (*pExpr).op }) as u32) as i32) != (54 as i32) {
        return;
    }
    pRight = unsafe { (*pExpr).pRight };
    pLeft = unsafe { (*pExpr).pLeft };
    0 as i32;
    0 as i32;
    let __v2732: bool;
    if (((unsafe { (*pRight).op }) as u32) as i32) == (168 as i32) {
        __v2732 =
            (unsafe { sqlite3ExprIsConstant(unsafe { (*pConst).pParse }, pLeft) }) != (0 as i32);
    } else {
        __v2732 = false as bool;
    }
    if __v2732 {
        constInsert(pConst, pRight, pLeft, pExpr);
    }
    let __v2733: bool;
    if (((unsafe { (*pLeft).op }) as u32) as i32) == (168 as i32) {
        __v2733 =
            (unsafe { sqlite3ExprIsConstant(unsafe { (*pConst).pParse }, pRight) }) != (0 as i32);
    } else {
        __v2733 = false as bool;
    }
    if __v2733 {
        constInsert(pConst, pLeft, pRight, pExpr);
    }
}

// /*
// ** This is a helper function for Walker callback propagateConstantExprRewrite().
// **
// ** Argument pExpr is a candidate expression to be replaced by a value. If
// ** pExpr is equivalent to one of the columns named in pWalker->u.pConst,
// ** then overwrite it with the corresponding value. Except, do not do so
// ** if argument bIgnoreAffBlob is non-zero and the affinity of pExpr
// ** is SQLITE_AFF_BLOB.
// */
fn propagateConstantExprRewriteOne(
    mut pConst: *mut WhereConst,
    mut pExpr: *mut Expr,
    mut bIgnoreAffBlob: i32,
) -> i32 {
    let mut i: i32 = 0 as i32;
    if (unsafe { *unsafe { unsafe { (*pConst).pOomFault }.offset((0 as i32) as isize) } })
        != (0 as u8)
    {
        return 1 as i32;
    }
    if (((unsafe { (*pExpr).op }) as u32) as i32) != (168 as i32) {
        return 0 as i32;
    }
    if (unsafe { (*pExpr).flags }) & (((32 as i32) as u32) | unsafe { (*pConst).mExcludeOn })
        != ((0 as i32) as u32)
    {
        {}
        {}
        {}
        return 0 as i32;
    }
    i = 0 as i32;
    '__slate_break_2124: loop {
        if !(i < unsafe { (*pConst).nConst }) {
            break;
        }
        let mut pColumn: *mut Expr =
            unsafe { *unsafe { unsafe { (*pConst).apExpr }.offset((i * (2 as i32)) as isize) } };
        if pColumn == pExpr {
        } else {
            if (unsafe { (*pColumn).iTable }) != unsafe { (*pExpr).iTable } {
            } else {
                if ((unsafe { (*pColumn).iColumn }) as i32)
                    != ((unsafe { (*pExpr).iColumn }) as i32)
                {
                } else {
                    0 as i32;
                    let __v2736: bool;
                    if bIgnoreAffBlob != (0 as i32) {
                        __v2736 = ((unsafe { sqlite3ExprAffinity(pColumn as *const Expr) }) as i32)
                            <= (65 as i32);
                    } else {
                        __v2736 = false as bool;
                    }
                    if __v2736 {
                        break '__slate_break_2124;
                    }
                    // /* A match is found.  Add the EP_FixedCol property */
                    let __v2737: *mut WhereConst = pConst;
                    let __v2738: i32 = unsafe { (*__v2737).nChng };
                    let __v2739: i32 = __v2738 + (1 as i32);
                    unsafe {
                        (*__v2737).nChng = __v2739;
                    }
                    let __v2740: *mut Expr = pExpr;
                    let __v2741: u32 = unsafe { (*__v2740).flags };
                    let __v2742: u32 = __v2741 & !((8388608 as i32) as u32);
                    unsafe {
                        (*__v2740).flags = __v2742;
                    }
                    let __v2743: *mut Expr = pExpr;
                    let __v2744: u32 = unsafe { (*__v2743).flags };
                    let __v2745: u32 = __v2744 | ((32 as i32) as u32);
                    unsafe {
                        (*__v2743).flags = __v2745;
                    }
                    0 as i32;
                    unsafe {
                        (*pExpr).pLeft = unsafe {
                            sqlite3ExprDup(
                                unsafe { (*unsafe { (*pConst).pParse }).db },
                                (unsafe {
                                    *unsafe {
                                        unsafe { (*pConst).apExpr }
                                            .offset((i * (2 as i32) + (1 as i32)) as isize)
                                    }
                                }) as *const Expr,
                                0 as i32,
                            )
                        };
                    }
                    if (unsafe { (*unsafe { (*unsafe { (*pConst).pParse }).db }).mallocFailed })
                        != (0 as u8)
                    {
                        return 1 as i32;
                    }
                    break '__slate_break_2124;
                }
            }
        }
        let __v2734: i32 = i;
        let __v2735: i32 = __v2734 + (1 as i32);
        i = __v2735;
    }
    return 1 as i32;
}

// /*
// ** This is a Walker expression callback. pExpr is a node from the WHERE
// ** clause of a SELECT statement. This function examines pExpr to see if
// ** any substitutions based on the contents of pWalker->u.pConst should
// ** be made to pExpr or its immediate children.
// **
// ** A substitution is made if:
// **
// **   + pExpr is a column with an affinity other than BLOB that matches
// **     one of the columns in pWalker->u.pConst, or
// **
// **   + pExpr is a binary comparison operator (=, <=, >=, <, >) that
// **     uses an affinity other than TEXT and one of its immediate
// **     children is a column that matches one of the columns in
// **     pWalker->u.pConst.
// */
#[unsafe(link_section = ".text.slate_distinct.select.propagateConstantExprRewrite")]
extern "C-unwind" fn propagateConstantExprRewrite(
    mut pWalker: *mut Walker,
    mut pExpr: *mut Expr,
) -> i32 {
    let mut pConst: *mut WhereConst = unsafe { (*pWalker).u.pConst };
    0 as i32;
    0 as i32;
    0 as i32;
    0 as i32;
    if (unsafe { (*pConst).bHasAffBlob }) != (0 as i32) {
        if (((unsafe { (*pExpr).op }) as u32) as i32) >= (54 as i32)
            && (((unsafe { (*pExpr).op }) as u32) as i32) <= (58 as i32)
            || (((unsafe { (*pExpr).op }) as u32) as i32) == (45 as i32)
        {
            propagateConstantExprRewriteOne(pConst, unsafe { (*pExpr).pLeft }, 0 as i32);
            if (unsafe { *unsafe { unsafe { (*pConst).pOomFault }.offset((0 as i32) as isize) } })
                != (0 as u8)
            {
                return 1 as i32;
            }
            if ((unsafe { sqlite3ExprAffinity((unsafe { (*pExpr).pLeft }) as *const Expr) }) as i32)
                != (66 as i32)
            {
                propagateConstantExprRewriteOne(pConst, unsafe { (*pExpr).pRight }, 0 as i32);
            }
        }
    }
    return propagateConstantExprRewriteOne(pConst, pExpr, unsafe { (*pConst).bHasAffBlob });
}

// /*
// ** The WHERE-clause constant propagation optimization.
// **
// ** If the WHERE clause contains terms of the form COLUMN=CONSTANT or
// ** CONSTANT=COLUMN that are top-level AND-connected terms that are not
// ** part of a ON clause from a LEFT JOIN, then throughout the query
// ** replace all other occurrences of COLUMN with CONSTANT.
// **
// ** For example, the query:
// **
// **      SELECT * FROM t1, t2, t3 WHERE t1.a=39 AND t2.b=t1.a AND t3.c=t2.b
// **
// ** Is transformed into
// **
// **      SELECT * FROM t1, t2, t3 WHERE t1.a=39 AND t2.b=39 AND t3.c=39
// **
// ** Return true if any transformations where made and false if not.
// **
// ** Implementation note:  Constant propagation is tricky due to affinity
// ** and collating sequence interactions.  Consider this example:
// **
// **    CREATE TABLE t1(a INT,b TEXT);
// **    INSERT INTO t1 VALUES(123,'0123');
// **    SELECT * FROM t1 WHERE a=123 AND b=a;
// **    SELECT * FROM t1 WHERE a=123 AND b=123;
// **
// ** The two SELECT statements above should return different answers.  b=a
// ** is always true because the comparison uses numeric affinity, but b=123
// ** is false because it uses text affinity and '0123' is not the same as '123'.
// ** To work around this, the expression tree is not actually changed from
// ** "b=a" to "b=123" but rather the "a" in "b=a" is tagged with EP_FixedCol
// ** and the "123" value is hung off of the pLeft pointer.  Code generator
// ** routines know to generate the constant "123" instead of looking up the
// ** column value.  Also, to avoid collation problems, this optimization is
// ** only attempted if the "a=123" term uses the default BINARY collation.
// **
// ** 2021-05-25 forum post 6a06202608: Another troublesome case is...
// **
// **    CREATE TABLE t1(x);
// **    INSERT INTO t1 VALUES(10.0);
// **    SELECT 1 FROM t1 WHERE x=10 AND x LIKE 10;
// **
// ** The query should return no rows, because the t1.x value is '10.0' not '10'
// ** and '10.0' is not LIKE '10'.  But if we are not careful, the first WHERE
// ** term "x=10" will cause the second WHERE term to become "10 LIKE 10",
// ** resulting in a false positive.  To avoid this, constant propagation for
// ** columns with BLOB affinity is only allowed if the constant is used with
// ** operators ==, <=, <, >=, >, or IS in a way that will cause the correct
// ** type conversions to occur.  See logic associated with the bHasAffBlob flag
// ** for details.
// */
fn propagateConstants(mut pParse: *mut Parse, mut p: *mut Select) -> i32 {
    let mut x: WhereConst = unsafe { std::mem::zeroed() };
    let mut w: Walker = unsafe { std::mem::zeroed() };
    let mut nChng: i32 = 0 as i32;
    x.pParse = pParse;
    x.pOomFault = unsafe { std::ptr::addr_of_mut!((*unsafe { (*pParse).db }).mallocFailed) };
    '__slate_break_2125: loop {
        x.nConst = 0 as i32;
        x.nChng = 0 as i32;
        x.apExpr = std::ptr::null_mut::<*mut Expr>();
        x.bHasAffBlob = 0 as i32;
        if (unsafe { (*p).pSrc }) != std::ptr::null_mut::<SrcList>()
            && (unsafe { (*unsafe { (*p).pSrc }).nSrc }) > (0 as i32)
            && (((unsafe {
                (*unsafe {
                    unsafe { std::ptr::addr_of_mut!((*unsafe { (*p).pSrc }).a) as *mut SrcItem }
                        .offset((0 as i32) as isize)
                })
                .fg
                .jointype
            }) as u32) as i32)
                & (64 as i32)
                != (0 as i32)
        {
            // /* Do not propagate constants on any ON clause if there is a
            //       ** RIGHT JOIN anywhere in the query */
            x.mExcludeOn = ((2 as i32) | (1 as i32)) as u32;
        } else {
            // /* Do not propagate constants through the ON clause of a LEFT JOIN */
            x.mExcludeOn = (1 as i32) as u32;
        }
        findConstInWhere(std::ptr::addr_of_mut!(x), unsafe { (*p).pWhere });
        if x.nConst != (0 as i32) {
            unsafe { memset(std::ptr::addr_of_mut!(w) as *mut (), 0 as i32, 48 as u64) };
            w.pParse = pParse;
            w.xExprCallback = Some(propagateConstantExprRewrite);
            w.xSelectCallback = unsafe {
                std::mem::transmute::<
                    *const (),
                    Option<unsafe extern "C-unwind" fn(*mut Walker, *mut Select) -> i32>,
                >(sqlite3SelectWalkNoop as *const ())
            };
            w.xSelectCallback2 = None;
            w.walkerDepth = 0 as i32;
            unsafe {
                w.u.pConst = std::ptr::addr_of_mut!(x);
            }
            unsafe { sqlite3WalkExpr(std::ptr::addr_of_mut!(w), unsafe { (*p).pWhere }) };
            unsafe { sqlite3DbFree(unsafe { (*x.pParse).db }, x.apExpr as *mut ()) };
            let __v2746: i32 = nChng;
            let __v2747: i32 = __v2746 + x.nChng;
            nChng = __v2747;
        }
        if !(x.nChng != (0 as i32)) {
            break;
        }
    }
    return nChng;
}

// /* The parsing context */
// /* The query in which to propagate constants */
// /*
// ** This function is called to determine whether or not it is safe to
// ** push WHERE clause expression pExpr down to FROM clause sub-query
// ** pSubq, which contains at least one window function. Return 1
// ** if it is safe and the expression should be pushed down, or 0
// ** otherwise.
// **
// ** It is only safe to push the expression down if it consists only
// ** of constants and copies of expressions that appear in the PARTITION
// ** BY clause of all window function used by the sub-query. It is safe
// ** to filter out entire partitions, but not rows within partitions, as
// ** this may change the results of the window functions.
// **
// ** At the time this function is called it is guaranteed that
// **
// **   * the sub-query uses only one distinct window frame, and
// **   * that the window frame has a PARTITION BY clause.
// */
fn pushDownWindowCheck(
    mut pParse: *mut Parse,
    mut pSubq: *mut Select,
    mut pExpr: *mut Expr,
) -> i32 {
    0 as i32;
    0 as i32;
    0 as i32;
    return unsafe {
        sqlite3ExprIsConstantOrGroupBy(pParse, pExpr, unsafe {
            (*unsafe { (*pSubq).pWin }).pPartition
        })
    };
}

// /* SQLITE_OMIT_WINDOWFUNC */
// /* !defined(SQLITE_OMIT_SUBQUERY) || !defined(SQLITE_OMIT_VIEW) */
// /*
// ** Make copies of relevant WHERE clause terms of the outer query into
// ** the WHERE clause of subquery.  Example:
// **
// **    SELECT * FROM (SELECT a AS x, c-d AS y FROM t1) WHERE x=5 AND y=10;
// **
// ** Transformed into:
// **
// **    SELECT * FROM (SELECT a AS x, c-d AS y FROM t1 WHERE a=5 AND c-d=10)
// **     WHERE x=5 AND y=10;
// **
// ** The hope is that the terms added to the inner query will make it more
// ** efficient.
// **
// ** NAME AMBIGUITY
// **
// ** This optimization is called the "WHERE-clause push-down optimization"
// ** or sometimes the "predicate push-down optimization".
// **
// ** Do not confuse this optimization with another unrelated optimization
// ** with a similar name:  The "MySQL push-down optimization" causes WHERE
// ** clause terms that can be evaluated using only the index and without
// ** reference to the table are run first, so that if they are false,
// ** unnecessary table seeks are avoided.
// **
// ** RULES
// **
// ** Do not attempt this optimization if:
// **
// **   (1) (** This restriction was removed on 2017-09-29.  We used to
// **           disallow this optimization for aggregate subqueries, but now
// **           it is allowed by putting the extra terms on the HAVING clause.
// **           The added HAVING clause is pointless if the subquery lacks
// **           a GROUP BY clause.  But such a HAVING clause is also harmless
// **           so there does not appear to be any reason to add extra logic
// **           to suppress it. **)
// **
// **   (2) The inner query is the recursive part of a common table expression.
// **
// **   (3) The inner query has a LIMIT clause (since the changes to the WHERE
// **       clause would change the meaning of the LIMIT).
// **
// **   (4) The inner query is the right operand of a LEFT JOIN and the
// **       expression to be pushed down does not come from the ON clause
// **       on that LEFT JOIN.
// **
// **   (5) The WHERE clause expression originates in the ON or USING clause
// **       of a LEFT JOIN where iCursor is not the right-hand table of that
// **       left join.  An example:
// **
// **           SELECT *
// **           FROM (SELECT 1 AS a1 UNION ALL SELECT 2) AS aa
// **           JOIN (SELECT 1 AS b2 UNION ALL SELECT 2) AS bb ON (a1=b2)
// **           LEFT JOIN (SELECT 8 AS c3 UNION ALL SELECT 9) AS cc ON (b2=2);
// **
// **       The correct answer is three rows:  (1,1,NULL),(2,2,8),(2,2,9).
// **       But if the (b2=2) term were to be pushed down into the bb subquery,
// **       then the (1,1,NULL) row would be suppressed.
// **
// **   (6) Window functions make things tricky as changes to the WHERE clause
// **       of the inner query could change the window over which window
// **       functions are calculated. Therefore, do not attempt the optimization
// **       if:
// **
// **     (6a) The inner query uses multiple incompatible window partitions.
// **
// **     (6b) The inner query is a compound and uses window-functions.
// **
// **     (6c) The WHERE clause does not consist entirely of constants and
// **          copies of expressions found in the PARTITION BY clause of
// **          all window-functions used by the sub-query. It is safe to
// **          filter out entire partitions, as this does not change the
// **          window over which any window-function is calculated.
// **
// **   (7) The inner query is a Common Table Expression (CTE) that should
// **       be materialized.  (This restriction is implemented in the calling
// **       routine.)
// **
// **   (8) If the subquery is a compound that uses UNION, INTERSECT,
// **       or EXCEPT, then all of the result set columns for all arms of
// **       the compound must use the BINARY collating sequence.
// **
// **   (9) All three of the following are true:
// **
// **       (9a) The WHERE clause expression originates in the ON or USING clause
// **            of a join (either an INNER or an OUTER join), and
// **
// **       (9b) The subquery is to the right of the ON/USING clause
// **
// **       (9c) There is a RIGHT JOIN (or FULL JOIN) in between the ON/USING
// **            clause and the subquery.
// **
// **       Without this restriction, the WHERE-clause push-down optimization
// **       might move the ON/USING filter expression from the left side of a
// **       RIGHT JOIN over to the right side, which leads to incorrect answers.
// **       See also restriction (6) in sqlite3ExprIsSingleTableConstraint().
// **
// **  (10) The inner query is not the right-hand table of a RIGHT JOIN.
// **
// **  (11) The subquery is not a VALUES clause
// **
// **  (12) The WHERE clause is not "rowid ISNULL" or the equivalent.  This
// **       case only comes up if SQLite is compiled using
// **       SQLITE_ALLOW_ROWID_IN_VIEW.
// **
// ** Return 0 if no changes are made and non-zero if one or more WHERE clause
// ** terms are duplicated into the subquery.
// */
fn pushDownWhereTerms(
    mut pParse: *mut Parse,
    mut pSubq: *mut Select,
    mut pWhere: *mut Expr,
    mut pSrcList: *mut SrcList,
    mut iSrc: i32,
) -> i32 {
    let mut pNew: *mut Expr = unsafe { std::mem::zeroed() };
    // /* The subquery FROM term into which WHERE is pushed */
    let mut pSrc: *mut SrcItem = unsafe { std::mem::zeroed() };
    let mut nChng: i32 = 0 as i32;
    pSrc = unsafe {
        unsafe { std::ptr::addr_of_mut!((*pSrcList).a) as *mut SrcItem }.offset(iSrc as isize)
    };
    if pWhere == std::ptr::null_mut::<Expr>() {
        return 0 as i32;
    }
    if (unsafe { (*pSubq).selFlags }) & (((8192 as i32) | (33554432 as i32)) as u32) != (0 as u32) {
        // /* restrictions (2) and (11) */
        return 0 as i32;
    }
    if (((unsafe { (*pSrc).fg.jointype }) as u32) as i32) & ((64 as i32) | (16 as i32))
        != (0 as i32)
    {
        // /* restrictions (10) */
        return 0 as i32;
    }
    if (unsafe { (*pSubq).pPrior }) != std::ptr::null_mut::<Select>() {
        let mut pSel: *mut Select = unsafe { std::mem::zeroed() };
        let mut notUnionAll: i32 = 0 as i32;
        pSel = pSubq;
        '__slate_break_2126: while pSel != std::ptr::null_mut::<Select>() {
            let mut op: u8 = unsafe { (*pSel).op };
            0 as i32;
            if ((op as u32) as i32) != (136 as i32) && ((op as u32) as i32) != (139 as i32) {
                notUnionAll = 1 as i32;
            }
            // /* restriction (6b) */
            if (unsafe { (*pSel).pWin }) != std::ptr::null_mut::<Window>() {
                return 0 as i32;
            }
            pSel = unsafe { (*pSel).pPrior };
        }
        if notUnionAll != (0 as i32) {
            // /* If any of the compound arms are connected using UNION, INTERSECT,
            //       ** or EXCEPT, then we must ensure that none of the columns use a
            //       ** non-BINARY collating sequence. */
            pSel = pSubq;
            '__slate_break_2127: while pSel != std::ptr::null_mut::<Select>() {
                let mut ii: i32 = 0 as i32;
                let mut pList: *const ExprList = (unsafe { (*pSel).pEList }) as *const ExprList;
                0 as i32;
                ii = 0 as i32;
                '__slate_break_2128: loop {
                    if !(ii < unsafe { (*pList).nExpr }) {
                        break;
                    }
                    let mut pColl: *mut CollSeq = unsafe {
                        sqlite3ExprCollSeq(
                            pParse,
                            (unsafe {
                                (*unsafe {
                                    unsafe {
                                        std::ptr::addr_of!((*pList).a) as *const ExprList_item
                                    }
                                    .offset(ii as isize)
                                })
                                .pExpr
                            }) as *const Expr,
                        )
                    };
                    if !((unsafe { sqlite3IsBinary(pColl as *const CollSeq) }) != (0 as i32)) {
                        // /* Restriction (8) */
                        return 0 as i32;
                    }
                    let __v2748: i32 = ii;
                    let __v2749: i32 = __v2748 + (1 as i32);
                    ii = __v2749;
                }
                pSel = unsafe { (*pSel).pPrior };
            }
        }
    } else {
        if (unsafe { (*pSubq).pWin }) != std::ptr::null_mut::<Window>()
            && (unsafe { (*unsafe { (*pSubq).pWin }).pPartition })
                == std::ptr::null_mut::<ExprList>()
        {
            return 0 as i32;
        }
    }
    if (unsafe { (*pSubq).pLimit }) != std::ptr::null_mut::<Expr>() {
        // /* restriction (3) */
        return 0 as i32;
    }
    '__slate_break_2129: while (((unsafe { (*pWhere).op }) as u32) as i32) == (44 as i32) {
        let __v2750: i32 = nChng;
        let __v2751: i32 = __v2750
            + pushDownWhereTerms(pParse, pSubq, unsafe { (*pWhere).pRight }, pSrcList, iSrc);
        nChng = __v2751;
        pWhere = unsafe { (*pWhere).pLeft };
    }
    // /* These checks now done by sqlite3ExprIsSingleTableConstraint() */
    if (unsafe {
        sqlite3ExprIsSingleTableConstraint(pWhere, pSrcList as *const SrcList, iSrc, 1 as i32)
    }) != (0 as i32)
    {
        let __v2752: i32 = nChng;
        let __v2753: i32 = __v2752 + (1 as i32);
        nChng = __v2753;
        let __v2754: *mut Select = pSubq;
        let __v2755: u32 = unsafe { (*__v2754).selFlags };
        let __v2756: u32 = __v2755 | ((16777216 as i32) as u32);
        unsafe {
            (*__v2754).selFlags = __v2756;
        }
        '__slate_break_2130: while pSubq != std::ptr::null_mut::<Select>() {
            let mut x: SubstContext = unsafe { std::mem::zeroed() };
            pNew =
                unsafe { sqlite3ExprDup(unsafe { (*pParse).db }, pWhere as *const Expr, 0 as i32) };
            unsetJoinExpr(pNew, -(1 as i32), 1 as i32);
            x.pParse = pParse;
            x.iTable = unsafe { (*pSrc).iCursor };
            x.iNewTable = unsafe { (*pSrc).iCursor };
            x.isOuterJoin = 0 as i32;
            x.nSelDepth = 0 as i32;
            x.pEList = unsafe { (*pSubq).pEList };
            x.pCList = findLeftmostExprlist(pSubq);
            pNew = substExpr(std::ptr::addr_of_mut!(x), pNew);
            0 as i32;
            if (unsafe { (*pParse).nErr }) == (0 as i32)
                && (((unsafe { (*pNew).op }) as u32) as i32) == (50 as i32)
                && (unsafe { (*pNew).flags }) & ((4096 as i32) as u32) != ((0 as i32) as u32)
            {
                0 as i32;
                let __v2757: *mut Select = unsafe { (*pNew).x.pSelect };
                let __v2758: u32 = unsafe { (*__v2757).selFlags };
                let __v2759: u32 = __v2758 | ((32 as i32) as u32);
                unsafe {
                    (*__v2757).selFlags = __v2759;
                }
                0 as i32;
                0 as i32;
                0 as i32;
                0 as i32;
                let __v2760: *mut Select = unsafe { (*pWhere).x.pSelect };
                let __v2761: u32 = unsafe { (*__v2760).selFlags };
                let __v2762: u32 = __v2761 | ((32 as i32) as u32);
                unsafe {
                    (*__v2760).selFlags = __v2762;
                }
            }
            let __v2763: bool;
            if (unsafe { (*pSubq).pWin }) != std::ptr::null_mut::<Window>() {
                __v2763 = (0 as i32) == pushDownWindowCheck(pParse, pSubq, pNew);
            } else {
                __v2763 = false as bool;
            }
            if __v2763 {
                // /* Restriction 6c has prevented push-down in this case */
                unsafe { sqlite3ExprDelete(unsafe { (*pParse).db }, pNew) };
                let __v2764: i32 = nChng;
                let __v2765: i32 = __v2764 - (1 as i32);
                nChng = __v2765;
                break '__slate_break_2130;
            }
            if (unsafe { (*pSubq).selFlags }) & ((8 as i32) as u32) != (0 as u32) {
                unsafe {
                    (*pSubq).pHaving =
                        unsafe { sqlite3ExprAnd(pParse, unsafe { (*pSubq).pHaving }, pNew) };
                }
            } else {
                unsafe {
                    (*pSubq).pWhere =
                        unsafe { sqlite3ExprAnd(pParse, unsafe { (*pSubq).pWhere }, pNew) };
                }
            }
            pSubq = unsafe { (*pSubq).pPrior };
        }
    }
    return nChng;
}

// /* Parse context (for malloc() and error reporting) */
// /* The subquery whose WHERE clause is to be augmented */
// /* The WHERE clause of the outer query */
// /* The complete from clause of the outer query */
// /* Which FROM clause term to try to push into  */
// /* !defined(SQLITE_OMIT_SUBQUERY) || !defined(SQLITE_OMIT_VIEW) */
// /*
// ** Check to see if a subquery contains result-set columns that are
// ** never used.  If it does, change the value of those result-set columns
// ** to NULL so that they do not cause unnecessary work to compute.
// **
// ** Return the number of column that were changed to NULL.
// */
fn disableUnusedSubqueryResultColumns(mut pItem: *mut SrcItem) -> i32 {
    let mut nCol: i32 = 0 as i32;
    // /* The subquery to be simplified */
    let mut pSub: *mut Select = unsafe { std::mem::zeroed() };
    // /* For looping over compound elements of pSub */
    let mut pX: *mut Select = unsafe { std::mem::zeroed() };
    // /* The table that describes the subquery */
    let mut pTab: *mut Table = unsafe { std::mem::zeroed() };
    // /* Column number */
    let mut j: i32 = 0 as i32;
    // /* Number of columns converted to NULL */
    let mut nChng: i32 = 0 as i32;
    // /* Columns that may not be NULLed out */
    let mut colUsed: u64 = 0 as u64;
    0 as i32;
    if ((unsafe { (*pItem).fg.__slate_bits_0.__get_isCorrelated() }) as i32) != (0 as i32)
        || ((unsafe { (*pItem).fg.__slate_bits_0.__get_isCte() }) as i32) != (0 as i32)
    {
        return 0 as i32;
    }
    0 as i32;
    pTab = unsafe { (*pItem).pSTab };
    0 as i32;
    pSub = unsafe { (*unsafe { (*pItem).u4.pSubq }).pSelect };
    0 as i32;
    pX = pSub;
    '__slate_break_2131: while pX != std::ptr::null_mut::<Select>() {
        if (unsafe { (*pX).selFlags }) & (((1 as i32) | (8 as i32)) as u32) != ((0 as i32) as u32) {
            {}
            {}
            return 0 as i32;
        }
        if (unsafe { (*pX).pPrior }) != std::ptr::null_mut::<Select>()
            && (((unsafe { (*pX).op }) as u32) as i32) != (136 as i32)
        {
            // /* This optimization does not work for compound subqueries that
            //       ** use UNION, INTERSECT, or EXCEPT.  Only UNION ALL is allowed. */
            return 0 as i32;
        }
        if (unsafe { (*pX).pWin }) != std::ptr::null_mut::<Window>() {
            // /* This optimization does not work for subqueries that use window
            //       ** functions. */
            return 0 as i32;
        }
        pX = unsafe { (*pX).pPrior };
    }
    colUsed = unsafe { (*pItem).colUsed };
    if (unsafe { (*pSub).pOrderBy }) != std::ptr::null_mut::<ExprList>() {
        let mut pList: *mut ExprList = unsafe { (*pSub).pOrderBy };
        j = 0 as i32;
        '__slate_break_2132: loop {
            if !(j < unsafe { (*pList).nExpr }) {
                break;
            }
            let mut iCol: u16 = unsafe {
                (*unsafe {
                    unsafe { std::ptr::addr_of_mut!((*pList).a) as *mut ExprList_item }
                        .offset(j as isize)
                })
                .u
                .x
                .iOrderByCol
            };
            if ((iCol as u32) as i32) > (0 as i32) {
                let __v2768: u16 = iCol;
                let __v2769: u16 = ((((__v2768 as u32) as i32) - (1 as i32)) as i16) as u16;
                iCol = __v2769;
                let __v2770: u64 = colUsed;
                let __v2771: u64 = __v2770
                    | (((1 as i32) as i64) as u64)
                        << if ((iCol as u32) as i32)
                            >= (((8 as u64).wrapping_mul(((8 as i32) as i64) as u64) as u32) as i32)
                        {
                            (((8 as u64).wrapping_mul(((8 as i32) as i64) as u64) as u32) as i32)
                                - (1 as i32)
                        } else {
                            (iCol as u32) as i32
                        };
                colUsed = __v2771;
            }
            let __v2766: i32 = j;
            let __v2767: i32 = __v2766 + (1 as i32);
            j = __v2767;
        }
    }
    nCol = (unsafe { (*pTab).nCol }) as i32;
    j = 0 as i32;
    '__slate_break_2133: loop {
        if !(j < nCol) {
            break;
        }
        let mut m: u64 = if j
            < (((8 as u64).wrapping_mul(((8 as i32) as i64) as u64) as u32) as i32) - (1 as i32)
        {
            (((1 as i32) as i64) as u64) << j
        } else {
            (((1 as i32) as i64) as u64)
                << (((8 as u64).wrapping_mul(((8 as i32) as i64) as u64) as u32) as i32)
                    - (1 as i32)
        };
        if m & colUsed != (((0 as i32) as i64) as u64) {
        } else {
            pX = pSub;
            '__slate_break_2134: while pX != std::ptr::null_mut::<Select>() {
                let mut pY: *mut Expr = unsafe {
                    (*unsafe {
                        unsafe {
                            std::ptr::addr_of_mut!((*unsafe { (*pX).pEList }).a)
                                as *mut ExprList_item
                        }
                        .offset(j as isize)
                    })
                    .pExpr
                };
                if (((unsafe { (*pY).op }) as u32) as i32) == (122 as i32) {
                } else {
                    unsafe {
                        (*pY).op = ((122 as i32) as i8) as u8;
                    }
                    let __v2774: *mut Expr = pY;
                    let __v2775: u32 = unsafe { (*__v2774).flags };
                    let __v2776: u32 = __v2775 & !(((8192 as i32) | (524288 as i32)) as u32);
                    unsafe {
                        (*__v2774).flags = __v2776;
                    }
                    let __v2777: *mut Select = pX;
                    let __v2778: u32 = unsafe { (*__v2777).selFlags };
                    let __v2779: u32 = __v2778 | ((16777216 as i32) as u32);
                    unsafe {
                        (*__v2777).selFlags = __v2779;
                    }
                    let __v2780: i32 = nChng;
                    let __v2781: i32 = __v2780 + (1 as i32);
                    nChng = __v2781;
                }
                pX = unsafe { (*pX).pPrior };
            }
        }
        let __v2772: i32 = j;
        let __v2773: i32 = __v2772 + (1 as i32);
        j = __v2773;
    }
    return nChng;
}

// /*
// ** The pFunc is the only aggregate function in the query.  Check to see
// ** if the query is a candidate for the min/max optimization.
// **
// ** If the query is a candidate for the min/max optimization, then set
// ** *ppMinMax to be an ORDER BY clause to be used for the optimization
// ** and return either WHERE_ORDERBY_MIN or WHERE_ORDERBY_MAX depending on
// ** whether pFunc is a min() or max() function.
// **
// ** If the query is not a candidate for the min/max optimization, return
// ** WHERE_ORDERBY_NORMAL (which must be zero).
// **
// ** This routine must be called after aggregate functions have been
// ** located but before their arguments have been subjected to aggregate
// ** analysis.
// */
fn minMaxQuery(mut db: *mut sqlite3, mut pFunc: *mut Expr, mut ppMinMax: *mut *mut ExprList) -> u8 {
    // /* Return value */
    let mut eRet: i32 = 0 as i32;
    // /* Arguments to agg function */
    let mut pEList: *mut ExprList = unsafe { std::mem::zeroed() };
    // /* Name of aggregate function pFunc */
    let mut zFunc: *const i8 = unsafe { std::mem::zeroed() };
    let mut pOrderBy: *mut ExprList = unsafe { std::mem::zeroed() };
    let mut sortFlags: u8 = ((0 as i32) as i8) as u8;
    0 as i32;
    0 as i32;
    0 as i32;
    0 as i32;
    pEList = unsafe { (*pFunc).x.pList };
    if pEList == std::ptr::null_mut::<ExprList>()
        || (unsafe { (*pEList).nExpr }) != (1 as i32)
        || (unsafe { (*pFunc).flags }) & ((16777216 as i32) as u32) != ((0 as i32) as u32)
        || (unsafe { (*db).dbOptFlags }) & ((65536 as i32) as u32) != ((0 as i32) as u32)
    {
        return (eRet as i8) as u8;
    }
    0 as i32;
    zFunc = (unsafe { (*pFunc).u.zToken }) as *const i8;
    if (unsafe { sqlite3StrICmp(zFunc, (b"min\0".as_ptr() as *mut i8) as *const i8) }) == (0 as i32)
    {
        eRet = 1 as i32;
        if (unsafe {
            sqlite3ExprCanBeNull(
                (unsafe {
                    (*unsafe {
                        unsafe { std::ptr::addr_of_mut!((*pEList).a) as *mut ExprList_item }
                            .offset((0 as i32) as isize)
                    })
                    .pExpr
                }) as *const Expr,
            )
        }) != (0 as i32)
        {
            sortFlags = ((2 as i32) as i8) as u8;
        }
    } else {
        if (unsafe { sqlite3StrICmp(zFunc, (b"max\0".as_ptr() as *mut i8) as *const i8) })
            == (0 as i32)
        {
            eRet = 2 as i32;
            sortFlags = ((1 as i32) as i8) as u8;
        } else {
            return (eRet as i8) as u8;
        }
    }
    let __v2782: *mut ExprList =
        unsafe { sqlite3ExprListDup(db, pEList as *const ExprList, 0 as i32) };
    pOrderBy = __v2782;
    unsafe {
        *ppMinMax = __v2782;
    }
    0 as i32;
    if pOrderBy != std::ptr::null_mut::<ExprList>() {
        unsafe {
            (*unsafe {
                unsafe { std::ptr::addr_of_mut!((*pOrderBy).a) as *mut ExprList_item }
                    .offset((0 as i32) as isize)
            })
            .fg
            .sortFlags = sortFlags;
        }
    }
    return (eRet as i8) as u8;
}

// /*
// ** The select statement passed as the first argument is an aggregate query.
// ** The second argument is the associated aggregate-info object. This
// ** function tests if the SELECT is of the form:
// **
// **   SELECT count(*) FROM <tbl>
// **
// ** where table is a database table, not a sub-select or view. If the query
// ** does match this pattern, then a pointer to the Table object representing
// ** <tbl> is returned. Otherwise, NULL is returned.
// **
// ** This routine checks to see if it is safe to use the count optimization.
// ** A correct answer is still obtained (though perhaps more slowly) if
// ** this routine returns NULL when it could have returned a table pointer.
// ** But returning the pointer when NULL should have been returned can
// ** result in incorrect answers and/or crashes.  So, when in doubt, return NULL.
// */
fn isSimpleCount(mut p: *mut Select, mut pAggInfo: *mut AggInfo) -> *mut Table {
    let mut pTab: *mut Table = unsafe { std::mem::zeroed() };
    let mut pExpr: *mut Expr = unsafe { std::mem::zeroed() };
    0 as i32;
    if (unsafe { (*p).pWhere }) != std::ptr::null_mut::<Expr>()
        || (unsafe { (*unsafe { (*p).pEList }).nExpr }) != (1 as i32)
        || (unsafe { (*unsafe { (*p).pSrc }).nSrc }) != (1 as i32)
        || ((unsafe {
            (*unsafe {
                unsafe { std::ptr::addr_of_mut!((*unsafe { (*p).pSrc }).a) as *mut SrcItem }
                    .offset((0 as i32) as isize)
            })
            .fg
            .__slate_bits_0
            .__get_isSubquery()
        }) as i32)
            != (0 as i32)
        || (unsafe { (*pAggInfo).nFunc }) != (1 as i32)
        || (unsafe { (*p).pHaving }) != std::ptr::null_mut::<Expr>()
    {
        return std::ptr::null_mut::<Table>();
    }
    pTab = unsafe {
        (*unsafe {
            unsafe { std::ptr::addr_of_mut!((*unsafe { (*p).pSrc }).a) as *mut SrcItem }
                .offset((0 as i32) as isize)
        })
        .pSTab
    };
    0 as i32;
    0 as i32;
    if !((((unsafe { (*pTab).eTabType }) as u32) as i32) == (0 as i32)) {
        return std::ptr::null_mut::<Table>();
    }
    pExpr = unsafe {
        (*unsafe {
            unsafe { std::ptr::addr_of_mut!((*unsafe { (*p).pEList }).a) as *mut ExprList_item }
                .offset((0 as i32) as isize)
        })
        .pExpr
    };
    0 as i32;
    if (((unsafe { (*pExpr).op }) as u32) as i32) != (169 as i32) {
        return std::ptr::null_mut::<Table>();
    }
    if (unsafe { (*pExpr).pAggInfo }) != pAggInfo {
        return std::ptr::null_mut::<Table>();
    }
    if (unsafe {
        (*unsafe { (*unsafe { unsafe { (*pAggInfo).aFunc }.offset((0 as i32) as isize) }).pFunc })
            .funcFlags
    }) & ((256 as i32) as u32)
        == ((0 as i32) as u32)
    {
        return std::ptr::null_mut::<Table>();
    }
    0 as i32;
    {}
    {}
    if (unsafe { (*pExpr).flags }) & (((4 as i32) | (16777216 as i32)) as u32)
        != ((0 as i32) as u32)
    {
        return std::ptr::null_mut::<Table>();
    }
    return pTab;
}

// /*
// ** Detect compound SELECT statements that use an ORDER BY clause with
// ** an alternative collating sequence.
// **
// **    SELECT ... FROM t1 EXCEPT SELECT ... FROM t2 ORDER BY .. COLLATE ...
// **
// ** These are rewritten as a subquery:
// **
// **    SELECT * FROM (SELECT ... FROM t1 EXCEPT SELECT ... FROM t2)
// **     ORDER BY ... COLLATE ...
// **
// ** This transformation is necessary because the multiSelectByMerge() routine
// ** above that generates the code for a compound SELECT with an ORDER BY clause
// ** uses a merge algorithm that requires the same collating sequence on the
// ** result columns as on the ORDER BY clause.  See ticket
// ** http://sqlite.org/src/info/6709574d2a
// **
// ** This transformation is only needed for EXCEPT, INTERSECT, and UNION.
// ** The UNION ALL operator works fine with multiSelectByMerge() even when
// ** there are COLLATE terms in the ORDER BY.
// */
#[unsafe(link_section = ".text.slate_distinct.select.convertCompoundSelectToSubquery")]
extern "C-unwind" fn convertCompoundSelectToSubquery(
    mut pWalker: *mut Walker,
    mut p: *mut Select,
) -> i32 {
    let mut i: i32 = 0 as i32;
    let mut pNew: *mut Select = unsafe { std::mem::zeroed() };
    let mut pX: *mut Select = unsafe { std::mem::zeroed() };
    let mut db: *mut sqlite3 = unsafe { std::mem::zeroed() };
    let mut a: *mut ExprList_item = unsafe { std::mem::zeroed() };
    let mut pNewSrc: *mut SrcList = unsafe { std::mem::zeroed() };
    let mut pParse: *mut Parse = unsafe { std::mem::zeroed() };
    let mut dummy: Token = unsafe { std::mem::zeroed() };
    if (unsafe { (*p).pPrior }) == std::ptr::null_mut::<Select>() {
        return 0 as i32;
    }
    if (unsafe { (*p).pOrderBy }) == std::ptr::null_mut::<ExprList>() {
        return 0 as i32;
    }
    pX = p;
    '__slate_break_2139: while pX != std::ptr::null_mut::<Select>()
        && ((((unsafe { (*pX).op }) as u32) as i32) == (136 as i32)
            || (((unsafe { (*pX).op }) as u32) as i32) == (139 as i32))
    {
        pX = unsafe { (*pX).pPrior };
    }
    if pX == std::ptr::null_mut::<Select>() {
        return 0 as i32;
    }
    a = unsafe { std::ptr::addr_of_mut!((*unsafe { (*p).pOrderBy }).a) as *mut ExprList_item };
    // /* If iOrderByCol is already non-zero, then it has already been matched
    //   ** to a result column of the SELECT statement. This occurs when the
    //   ** SELECT is rewritten for window-functions processing and then passed
    //   ** to sqlite3SelectPrep() and similar a second time. The rewriting done
    //   ** by this function is not required in this case. */
    if (unsafe { (*unsafe { a.offset((0 as i32) as isize) }).u.x.iOrderByCol }) != (0 as u16) {
        return 0 as i32;
    }
    i = (unsafe { (*unsafe { (*p).pOrderBy }).nExpr }) - (1 as i32);
    '__slate_break_2140: loop {
        if !(i >= (0 as i32)) {
            break;
        }
        if (unsafe { (*unsafe { (*unsafe { a.offset(i as isize) }).pExpr }).flags })
            & ((512 as i32) as u32)
            != (0 as u32)
        {
            break '__slate_break_2140;
        }
        let __v2783: i32 = i;
        let __v2784: i32 = __v2783 - (1 as i32);
        i = __v2784;
    }
    if i < (0 as i32) {
        return 0 as i32;
    }
    // /* If we reach this point, that means the transformation is required. */
    pParse = unsafe { (*pWalker).pParse };
    db = unsafe { (*pParse).db };
    pNew = (unsafe { sqlite3DbMallocZero(db, 120 as u64) }) as *mut Select;
    if pNew == std::ptr::null_mut::<Select>() {
        return 2 as i32;
    }
    unsafe {
        memset(
            std::ptr::addr_of_mut!(dummy) as *mut (),
            0 as i32,
            16 as u64,
        )
    };
    pNewSrc = unsafe {
        sqlite3SrcListAppendFromTerm(
            pParse,
            std::ptr::null_mut::<SrcList>(),
            std::ptr::null_mut::<Token>(),
            std::ptr::null_mut::<Token>(),
            std::ptr::addr_of_mut!(dummy),
            pNew,
            std::ptr::null_mut::<OnOrUsing>(),
        )
    };
    0 as i32;
    if (unsafe { (*pParse).nErr }) != (0 as i32) {
        unsafe { sqlite3SrcListDelete(db, pNewSrc) };
        return 2 as i32;
    }
    unsafe {
        *pNew = unsafe { *p };
    }
    unsafe {
        (*p).pSrc = pNewSrc;
    }
    unsafe {
        (*p).pEList = unsafe {
            sqlite3ExprListAppend(pParse, std::ptr::null_mut::<ExprList>(), unsafe {
                sqlite3Expr(db, 180 as i32, std::ptr::null::<i8>())
            })
        };
    }
    unsafe {
        (*p).op = ((139 as i32) as i8) as u8;
    }
    unsafe {
        (*p).pWhere = std::ptr::null_mut::<Expr>();
    }
    unsafe {
        (*pNew).pGroupBy = std::ptr::null_mut::<ExprList>();
    }
    unsafe {
        (*pNew).pHaving = std::ptr::null_mut::<Expr>();
    }
    unsafe {
        (*pNew).pOrderBy = std::ptr::null_mut::<ExprList>();
    }
    unsafe {
        (*p).pPrior = std::ptr::null_mut::<Select>();
    }
    unsafe {
        (*p).pNext = std::ptr::null_mut::<Select>();
    }
    unsafe {
        (*p).pWith = std::ptr::null_mut::<With>();
    }
    unsafe {
        (*p).pWinDefn = std::ptr::null_mut::<Window>();
    }
    let __v2785: *mut Select = p;
    let __v2786: u32 = unsafe { (*__v2785).selFlags };
    let __v2787: u32 = __v2786 & !((256 as i32) as u32);
    unsafe {
        (*__v2785).selFlags = __v2787;
    }
    0 as i32;
    let __v2788: *mut Select = p;
    let __v2789: u32 = unsafe { (*__v2788).selFlags };
    let __v2790: u32 = __v2789 | ((65536 as i32) as u32);
    unsafe {
        (*__v2788).selFlags = __v2790;
    }
    0 as i32;
    unsafe {
        (*unsafe { (*pNew).pPrior }).pNext = pNew;
    }
    unsafe {
        (*pNew).pLimit = std::ptr::null_mut::<Expr>();
    }
    return 0 as i32;
}

// /*
// ** Check to see if the FROM clause term pFrom has table-valued function
// ** arguments.  If it does, leave an error message in pParse and return
// ** non-zero, since pFrom is not allowed to be a table-valued function.
// */
fn cannotBeFunction(mut pParse: *mut Parse, mut pFrom: *mut SrcItem) -> i32 {
    if ((unsafe { (*pFrom).fg.__slate_bits_0.__get_isTabFunc() }) as i32) != (0 as i32) {
        unsafe {
            sqlite3ErrorMsg(
                pParse,
                (b"'%s' is not a function\0".as_ptr() as *mut i8) as *const i8,
                unsafe { (*pFrom).zName },
            )
        };
        return 1 as i32;
    }
    return 0 as i32;
}

// /*
// ** Argument pWith (which may be NULL) points to a linked list of nested
// ** WITH contexts, from inner to outermost. If the table identified by
// ** FROM clause element pItem is really a common-table-expression (CTE)
// ** then return a pointer to the CTE definition for that table. Otherwise
// ** return NULL.
// **
// ** If a non-NULL value is returned, set *ppContext to point to the With
// ** object that the returned CTE belongs to.
// */
fn searchWith(
    mut pWith: *mut With,
    mut pItem: *mut SrcItem,
    mut ppContext: *mut *mut With,
) -> *mut Cte {
    let mut zName: *const i8 = (unsafe { (*pItem).zName }) as *const i8;
    let mut p: *mut With = unsafe { std::mem::zeroed() };
    0 as i32;
    0 as i32;
    p = pWith;
    '__slate_break_2142: while p != std::ptr::null_mut::<With>() {
        let mut i: i32 = 0 as i32;
        i = 0 as i32;
        '__slate_break_2143: loop {
            if !(i < unsafe { (*p).nCte }) {
                break;
            }
            if (unsafe {
                sqlite3StrICmp(
                    zName,
                    (unsafe {
                        (*unsafe {
                            unsafe { std::ptr::addr_of_mut!((*p).a) as *mut Cte }.offset(i as isize)
                        })
                        .zName
                    }) as *const i8,
                )
            }) == (0 as i32)
            {
                unsafe {
                    *ppContext = p;
                }
                return unsafe {
                    unsafe { std::ptr::addr_of_mut!((*p).a) as *mut Cte }.offset(i as isize)
                };
            }
            let __v2791: i32 = i;
            let __v2792: i32 = __v2791 + (1 as i32);
            i = __v2792;
        }
        if (unsafe { (*p).bView }) != (0 as i32) {
            break '__slate_break_2142;
        }
        p = unsafe { (*p).pOuter };
    }
    return std::ptr::null_mut::<Cte>();
}

// /*
// ** This function checks if argument pFrom refers to a CTE declared by
// ** a WITH clause on the stack currently maintained by the parser (on the
// ** pParse->pWith linked list).  And if currently processing a CTE
// ** CTE expression, through routine checks to see if the reference is
// ** a recursive reference to the CTE.
// **
// ** If pFrom matches a CTE according to either of these two above, pFrom->pSTab
// ** and other fields are populated accordingly.
// **
// ** Return 0 if no match is found.
// ** Return 1 if a match is found.
// ** Return 2 if an error condition is detected.
// */
fn resolveFromTermToCte(
    mut pParse: *mut Parse,
    mut pWalker: *mut Walker,
    mut pFrom: *mut SrcItem,
) -> i32 {
    // /* Matched CTE (or NULL if no match) */
    let mut pCte: *mut Cte = unsafe { std::mem::zeroed() };
    // /* The matching WITH */
    let mut pWith: *mut With = unsafe { std::mem::zeroed() };
    0 as i32;
    if (unsafe { (*pParse).pWith }) == std::ptr::null_mut::<With>() {
        // /* There are no WITH clauses in the stack.  No match is possible */
        return 0 as i32;
    }
    if (unsafe { (*pParse).nErr }) != (0 as i32) {
        // /* Prior errors might have left pParse->pWith in a goofy state, so
        //     ** go no further. */
        return 0 as i32;
    }
    0 as i32;
    if ((unsafe { (*pFrom).fg.__slate_bits_0.__get_fixedSchema() }) as i32) == (0 as i32)
        && (unsafe { (*pFrom).u4.zDatabase }) != std::ptr::null_mut::<i8>()
    {
        // /* The FROM term contains a schema qualifier (ex: main.t1) and so
        //     ** it cannot possibly be a CTE reference. */
        return 0 as i32;
    }
    if ((unsafe { (*pFrom).fg.__slate_bits_0.__get_notCte() }) as i32) != (0 as i32) {
        // /* The FROM term is specifically excluded from matching a CTE.
        //     **   (1)  It is part of a trigger that used to have zDatabase but had
        //     **        zDatabase removed by sqlite3FixTriggerStep().
        //     **   (2)  This is the first term in the FROM clause of an UPDATE.
        //     */
        return 0 as i32;
    }
    pCte = searchWith(
        unsafe { (*pParse).pWith },
        pFrom,
        std::ptr::addr_of_mut!(pWith),
    );
    if pCte != std::ptr::null_mut::<Cte>() {
        let mut db: *mut sqlite3 = unsafe { (*pParse).db };
        let mut pTab: *mut Table = unsafe { std::mem::zeroed() };
        let mut pEList: *mut ExprList = unsafe { std::mem::zeroed() };
        let mut pSel: *mut Select = unsafe { std::mem::zeroed() };
        // /* Left-most SELECT statement */
        let mut pLeft: *mut Select = unsafe { std::mem::zeroed() };
        // /* Left-most recursive term */
        let mut pRecTerm: *mut Select = unsafe { std::mem::zeroed() };
        // /* True if compound joined by UNION [ALL] */
        let mut bMayRecursive: i32 = 0 as i32;
        // /* Initial value of pParse->pWith */
        let mut pSavedWith: *mut With = unsafe { std::mem::zeroed() };
        // /* Cursor for recursive table */
        let mut iRecTab: i32 = -(1 as i32);
        let mut pCteUse: *mut CteUse = unsafe { std::mem::zeroed() };
        // /* If pCte->zCteErr is non-NULL at this point, then this is an illegal
        //     ** recursive reference to CTE pCte. Leave an error in pParse and return
        //     ** early. If pCte->zCteErr is NULL, then this is not a recursive reference.
        //     ** In this case, proceed.  */
        if (unsafe { (*pCte).zCteErr }) != std::ptr::null::<i8>() {
            unsafe {
                sqlite3ErrorMsg(pParse, unsafe { (*pCte).zCteErr }, unsafe { (*pCte).zName })
            };
            return 2 as i32;
        }
        if cannotBeFunction(pParse, pFrom) != (0 as i32) {
            return 2 as i32;
        }
        0 as i32;
        pTab = (unsafe { sqlite3DbMallocZero(db, 120 as u64) }) as *mut Table;
        if pTab == std::ptr::null_mut::<Table>() {
            return 2 as i32;
        }
        pCteUse = unsafe { (*pCte).pUse };
        if pCteUse == std::ptr::null_mut::<CteUse>() {
            let __v2793: *mut CteUse =
                (unsafe { sqlite3DbMallocZero(db, 20 as u64) }) as *mut CteUse;
            pCteUse = __v2793;
            unsafe {
                (*pCte).pUse = __v2793;
            }
            let __v2794: bool;
            if pCteUse == std::ptr::null_mut::<CteUse>() {
                __v2794 = true as bool;
            } else {
                __v2794 = (unsafe {
                    sqlite3ParserAddCleanup(
                        pParse,
                        unsafe {
                            std::mem::transmute::<
                                *const (),
                                Option<unsafe extern "C-unwind" fn(*mut sqlite3, *mut ())>,
                            >(sqlite3DbFree as *const ())
                        },
                        pCteUse as *mut (),
                    )
                }) == std::ptr::null_mut::<()>();
            }
            if __v2794 {
                unsafe { sqlite3DbFree(db, pTab as *mut ()) };
                return 2 as i32;
            }
            unsafe {
                (*pCteUse).eM10d = unsafe { (*pCte).eM10d };
            }
        }
        unsafe {
            (*pFrom).pSTab = pTab;
        }
        unsafe {
            (*pTab).nTabRef = (1 as i32) as u32;
        }
        unsafe {
            (*pTab).zName = unsafe { sqlite3DbStrDup(db, (unsafe { (*pCte).zName }) as *const i8) };
        }
        unsafe {
            (*pTab).iPKey = -(1 as i32) as i16;
        }
        unsafe {
            (*pTab).nRowLogEst = (200 as i32) as i16;
        }
        0 as i32;
        let __v2795: *mut Table = pTab;
        let __v2796: u32 = unsafe { (*__v2795).tabFlags };
        let __v2797: u32 = __v2796 | (((16384 as i32) | (512 as i32)) as u32);
        unsafe {
            (*__v2795).tabFlags = __v2797;
        }
        unsafe {
            sqlite3SrcItemAttachSubquery(pParse, pFrom, unsafe { (*pCte).pSelect }, 1 as i32)
        };
        if (unsafe { (*db).mallocFailed }) != (0 as u8) {
            return 2 as i32;
        }
        0 as i32;
        pSel = unsafe { (*unsafe { (*pFrom).u4.pSubq }).pSelect };
        0 as i32;
        let __v2798: *mut Select = pSel;
        let __v2799: u32 = unsafe { (*__v2798).selFlags };
        let __v2800: u32 = __v2799 | ((67108864 as i32) as u32);
        unsafe {
            (*__v2798).selFlags = __v2800;
        }
        if ((unsafe { (*pFrom).fg.__slate_bits_0.__get_isIndexedBy() }) as i32) != (0 as i32) {
            unsafe {
                sqlite3ErrorMsg(
                    pParse,
                    (b"no such index: \"%s\"\0".as_ptr() as *mut i8) as *const i8,
                    unsafe { (*pFrom).u1.zIndexedBy },
                )
            };
            return 2 as i32;
        }
        0 as i32;
        unsafe {
            (*pFrom).fg.__slate_bits_0.__set_isCte((1 as i32) as u32);
        }
        unsafe {
            (*pFrom).u2.pCteUse = pCteUse;
        }
        let __v2801: *mut CteUse = pCteUse;
        let __v2802: i32 = unsafe { (*__v2801).nUse };
        let __v2803: i32 = __v2802 + (1 as i32);
        unsafe {
            (*__v2801).nUse = __v2803;
        }
        // /* Check if this is a recursive CTE. */
        pRecTerm = pSel;
        bMayRecursive = ((((unsafe { (*pSel).op }) as u32) as i32) == (136 as i32)
            || (((unsafe { (*pSel).op }) as u32) as i32) == (135 as i32))
            as i32;
        '__slate_break_2145: while bMayRecursive != (0 as i32)
            && (((unsafe { (*pRecTerm).op }) as u32) as i32)
                == (((unsafe { (*pSel).op }) as u32) as i32)
        {
            let mut i: i32 = 0 as i32;
            let mut pSrc: *mut SrcList = unsafe { (*pRecTerm).pSrc };
            0 as i32;
            i = 0 as i32;
            '__slate_break_2146: loop {
                if !(i < unsafe { (*pSrc).nSrc }) {
                    break;
                }
                let mut pItem: *mut SrcItem = unsafe {
                    unsafe { std::ptr::addr_of_mut!((*pSrc).a) as *mut SrcItem }.offset(i as isize)
                };
                let __v2806: bool;
                if (unsafe { (*pItem).zName }) != std::ptr::null_mut::<i8>()
                    && !(((unsafe { (*pItem).fg.__slate_bits_0.__get_hadSchema() }) as i32)
                        != (0 as i32))
                    && !(((unsafe { (*pItem).fg.__slate_bits_0.__get_isSubquery() }) as i32)
                        != (0 as i32))
                    && (((unsafe { (*pItem).fg.__slate_bits_0.__get_fixedSchema() }) as i32)
                        != (0 as i32)
                        || (unsafe { (*pItem).u4.zDatabase }) == std::ptr::null_mut::<i8>())
                {
                    __v2806 = (0 as i32)
                        == unsafe {
                            sqlite3StrICmp(
                                (unsafe { (*pItem).zName }) as *const i8,
                                (unsafe { (*pCte).zName }) as *const i8,
                            )
                        };
                } else {
                    __v2806 = false as bool;
                }
                if __v2806 {
                    unsafe {
                        (*pItem).pSTab = pTab;
                    }
                    let __v2807: *mut Table = pTab;
                    let __v2808: u32 = unsafe { (*__v2807).nTabRef };
                    let __v2809: u32 = __v2808.wrapping_add((1 as i32) as u32);
                    unsafe {
                        (*__v2807).nTabRef = __v2809;
                    }
                    unsafe {
                        (*pItem)
                            .fg
                            .__slate_bits_0
                            .__set_isRecursive((1 as i32) as u32);
                    }
                    if (unsafe { (*pRecTerm).selFlags }) & ((8192 as i32) as u32) != (0 as u32) {
                        unsafe {
                            sqlite3ErrorMsg(
                                pParse,
                                (b"multiple references to recursive table: %s\0".as_ptr()
                                    as *mut i8) as *const i8,
                                unsafe { (*pCte).zName },
                            )
                        };
                        return 2 as i32;
                    }
                    let __v2810: *mut Select = pRecTerm;
                    let __v2811: u32 = unsafe { (*__v2810).selFlags };
                    let __v2812: u32 = __v2811 | ((8192 as i32) as u32);
                    unsafe {
                        (*__v2810).selFlags = __v2812;
                    }
                    if iRecTab < (0 as i32) {
                        let __v2813: *mut Parse = pParse;
                        let __v2814: i32 = unsafe { (*__v2813).nTab };
                        let __v2815: i32 = __v2814 + (1 as i32);
                        unsafe {
                            (*__v2813).nTab = __v2815;
                        }
                        iRecTab = __v2814;
                    }
                    unsafe {
                        (*pItem).iCursor = iRecTab;
                    }
                }
                let __v2804: i32 = i;
                let __v2805: i32 = __v2804 + (1 as i32);
                i = __v2805;
            }
            if (unsafe { (*pRecTerm).selFlags }) & ((8192 as i32) as u32) == ((0 as i32) as u32) {
                break '__slate_break_2145;
            }
            pRecTerm = unsafe { (*pRecTerm).pPrior };
        }
        unsafe {
            (*pCte).zCteErr = (b"circular reference: %s\0".as_ptr() as *mut i8) as *const i8;
        }
        pSavedWith = unsafe { (*pParse).pWith };
        unsafe {
            (*pParse).pWith = pWith;
        }
        if (unsafe { (*pSel).selFlags }) & ((8192 as i32) as u32) != (0 as u32) {
            let mut rc: i32 = 0 as i32;
            0 as i32;
            0 as i32;
            0 as i32;
            0 as i32;
            0 as i32;
            unsafe {
                (*pRecTerm).pWith = unsafe { (*pSel).pWith };
            }
            rc = unsafe { sqlite3WalkSelect(pWalker, pRecTerm) };
            unsafe {
                (*pRecTerm).pWith = std::ptr::null_mut::<With>();
            }
            if rc != (0 as i32) {
                unsafe {
                    (*pParse).pWith = pSavedWith;
                }
                return 2 as i32;
            }
        } else {
            if (unsafe { sqlite3WalkSelect(pWalker, pSel) }) != (0 as i32) {
                unsafe {
                    (*pParse).pWith = pSavedWith;
                }
                return 2 as i32;
            }
        }
        unsafe {
            (*pParse).pWith = pWith;
        }
        pLeft = pSel;
        '__slate_break_2149: while (unsafe { (*pLeft).pPrior }) != std::ptr::null_mut::<Select>() {
            {}
            pLeft = unsafe { (*pLeft).pPrior };
        }
        pEList = unsafe { (*pLeft).pEList };
        if (unsafe { (*pCte).pCols }) != std::ptr::null_mut::<ExprList>() {
            if pEList != std::ptr::null_mut::<ExprList>()
                && (unsafe { (*pEList).nExpr }) != unsafe { (*unsafe { (*pCte).pCols }).nExpr }
            {
                unsafe {
                    sqlite3ErrorMsg(
                        pParse,
                        (b"table %s has %d values for %d columns\0".as_ptr() as *mut i8)
                            as *const i8,
                        unsafe { (*pCte).zName },
                        unsafe { (*pEList).nExpr },
                        unsafe { (*unsafe { (*pCte).pCols }).nExpr },
                    )
                };
                unsafe {
                    (*pParse).pWith = pSavedWith;
                }
                return 2 as i32;
            }
            pEList = unsafe { (*pCte).pCols };
        }
        sqlite3ColumnsFromExprList(
            pParse,
            pEList,
            unsafe { std::ptr::addr_of_mut!((*pTab).nCol) },
            unsafe { std::ptr::addr_of_mut!((*pTab).aCol) },
        );
        if bMayRecursive != (0 as i32) {
            if (unsafe { (*pSel).selFlags }) & ((8192 as i32) as u32) != (0 as u32) {
                unsafe {
                    (*pCte).zCteErr =
                        (b"multiple recursive references: %s\0".as_ptr() as *mut i8) as *const i8;
                }
            } else {
                unsafe {
                    (*pCte).zCteErr = (b"recursive reference in a subquery: %s\0".as_ptr()
                        as *mut i8) as *const i8;
                }
            }
            unsafe { sqlite3WalkSelect(pWalker, pSel) };
        }
        unsafe {
            (*pCte).zCteErr = std::ptr::null::<i8>();
        }
        unsafe {
            (*pParse).pWith = pSavedWith;
        }
        // /* Success */
        return 1 as i32;
    }
    // /* No match */
    return 0 as i32;
}

// /*
// ** Check the N SrcItem objects to the right of pBase.  (N might be zero!)
// ** If any of those SrcItem objects have a USING clause containing zName
// ** then return true.
// **
// ** If N is zero, or none of the N SrcItem objects to the right of pBase
// ** contains a USING clause, or if none of the USING clauses contain zName,
// ** then return false.
// */
fn inAnyUsingClause(mut zName: *const i8, mut pBase: *mut SrcItem, mut N: i32) -> i32 {
    '__slate_break_2155: while N > (0 as i32) {
        let __v2816: i32 = N;
        let __v2817: i32 = __v2816 - (1 as i32);
        N = __v2817;
        let __v2818: *mut SrcItem = pBase;
        let __v2819: *mut SrcItem = unsafe { __v2818.offset((1 as i32) as isize) };
        pBase = __v2819;
        if ((unsafe { (*pBase).fg.__slate_bits_0.__get_isUsing() }) as i32) == (0 as i32) {
        } else {
            if (unsafe { (*pBase).u3.pUsing }) == std::ptr::null_mut::<IdList>() {
            } else {
                if (unsafe { sqlite3IdListIndex(unsafe { (*pBase).u3.pUsing }, zName) })
                    >= (0 as i32)
                {
                    return 1 as i32;
                }
            }
        }
    }
    return 0 as i32;
}

// /* Name we are looking for */
// /* The base SrcItem.  Looking at pBase[1] and following */
// /* How many SrcItems to check */
// /*
// ** This routine is a Walker callback for "expanding" a SELECT statement.
// ** "Expanding" means to do the following:
// **
// **    (1)  Make sure VDBE cursor numbers have been assigned to every
// **         element of the FROM clause.
// **
// **    (2)  Fill in the pTabList->a[].pTab fields in the SrcList that
// **         defines FROM clause.  When views appear in the FROM clause,
// **         fill pTabList->a[].pSelect with a copy of the SELECT statement
// **         that implements the view.  A copy is made of the view's SELECT
// **         statement so that we can freely modify or delete that statement
// **         without worrying about messing up the persistent representation
// **         of the view.
// **
// **    (3)  Add terms to the WHERE clause to accommodate the NATURAL keyword
// **         on joins and the ON and USING clause of joins.
// **
// **    (4)  Scan the list of columns in the result set (pEList) looking
// **         for instances of the "*" operator or the TABLE.* operator.
// **         If found, expand each "*" to be every column in every table
// **         and TABLE.* to be every column in TABLE.
// **
// */
#[unsafe(link_section = ".text.slate_distinct.select.selectExpander")]
extern "C-unwind" fn selectExpander(mut pWalker: *mut Walker, mut p: *mut Select) -> i32 {
    let mut pParse: *mut Parse = unsafe { (*pWalker).pParse };
    let mut i: i32 = 0 as i32;
    let mut j: i32 = 0 as i32;
    let mut k: i32 = 0 as i32;
    let mut rc: i32 = 0 as i32;
    let mut pTabList: *mut SrcList = unsafe { std::mem::zeroed() };
    let mut pEList: *mut ExprList = unsafe { std::mem::zeroed() };
    let mut pFrom: *mut SrcItem = unsafe { std::mem::zeroed() };
    let mut db: *mut sqlite3 = unsafe { (*pParse).db };
    let mut pE: *mut Expr = unsafe { std::mem::zeroed() };
    let mut pRight: *mut Expr = unsafe { std::mem::zeroed() };
    let mut pExpr: *mut Expr = unsafe { std::mem::zeroed() };
    let mut selFlags: u16 = (unsafe { (*p).selFlags }) as u16;
    let mut elistFlags: u32 = (0 as i32) as u32;
    let __v2820: *mut Select = p;
    let __v2821: u32 = unsafe { (*__v2820).selFlags };
    let __v2822: u32 = __v2821 | ((64 as i32) as u32);
    unsafe {
        (*__v2820).selFlags = __v2822;
    }
    if (unsafe { (*db).mallocFailed }) != (0 as u8) {
        return 2 as i32;
    }
    0 as i32;
    if ((selFlags as u32) as i32) & (64 as i32) != (0 as i32) {
        return 1 as i32;
    }
    if (unsafe { (*pWalker).eCode }) != (0 as u16) {
        // /* Renumber selId because it has been copied from a view */
        let __v2823: *mut Parse = pParse;
        let __v2824: i32 = unsafe { (*__v2823).nSelect };
        let __v2825: i32 = __v2824 + (1 as i32);
        unsafe {
            (*__v2823).nSelect = __v2825;
        }
        unsafe {
            (*p).selId = __v2825 as u32;
        }
    }
    pTabList = unsafe { (*p).pSrc };
    pEList = unsafe { (*p).pEList };
    if (unsafe { (*pParse).pWith }) != std::ptr::null_mut::<With>()
        && (unsafe { (*p).selFlags }) & ((2097152 as i32) as u32) != (0 as u32)
    {
        if (unsafe { (*p).pWith }) == std::ptr::null_mut::<With>() {
            unsafe {
                (*p).pWith = (unsafe {
                    sqlite3DbMallocZero(
                        db,
                        (16 as u64)
                            .wrapping_add((((1 as i32) as i64) as u64).wrapping_mul(48 as u64)),
                    )
                }) as *mut With;
            }
            if (unsafe { (*p).pWith }) == std::ptr::null_mut::<With>() {
                return 2 as i32;
            }
        }
        unsafe {
            (*unsafe { (*p).pWith }).bView = 1 as i32;
        }
    }
    sqlite3WithPush(pParse, unsafe { (*p).pWith }, ((0 as i32) as i8) as u8);
    // /* Make sure cursor numbers have been assigned to all entries in
    //   ** the FROM clause of the SELECT statement.
    //   */
    unsafe { sqlite3SrcListAssignCursors(pParse, pTabList) };
    // /* Look up every table named in the FROM clause of the select.  If
    //   ** an entry of the FROM clause is a subquery instead of a table or view,
    //   ** then create a transient table structure to describe the subquery.
    //   */
    i = 0 as i32;
    let __v2826: *mut SrcItem = unsafe { std::ptr::addr_of_mut!((*pTabList).a) as *mut SrcItem };
    pFrom = __v2826;
    '__slate_break_2156: while i < unsafe { (*pTabList).nSrc } {
        let mut pTab: *mut Table = unsafe { std::mem::zeroed() };
        0 as i32;
        if (unsafe { (*pFrom).pSTab }) != std::ptr::null_mut::<Table>() {
        } else {
            0 as i32;
            if (unsafe { (*pFrom).zName }) == std::ptr::null_mut::<i8>() {
                let mut pSel: *mut Select = unsafe { std::mem::zeroed() };
                0 as i32;
                pSel = unsafe { (*unsafe { (*pFrom).u4.pSubq }).pSelect };
                // /* A sub-query in the FROM clause of a SELECT */
                0 as i32;
                0 as i32;
                if (unsafe { sqlite3WalkSelect(pWalker, pSel) }) != (0 as i32) {
                    return 2 as i32;
                }
                if sqlite3ExpandSubquery(pParse, pFrom) != (0 as i32) {
                    return 2 as i32;
                }
            } else {
                let __v2831: i32 = resolveFromTermToCte(pParse, pWalker, pFrom);
                rc = __v2831;
                if __v2831 != (0 as i32) {
                    if rc > (1 as i32) {
                        return 2 as i32;
                    }
                    pTab = unsafe { (*pFrom).pSTab };
                    0 as i32;
                } else {
                    // /* An ordinary table or view name in the FROM clause */
                    0 as i32;
                    let __v2832: *mut Table =
                        unsafe { sqlite3LocateTableItem(pParse, (0 as i32) as u32, pFrom) };
                    pTab = __v2832;
                    unsafe {
                        (*pFrom).pSTab = __v2832;
                    }
                    if pTab == std::ptr::null_mut::<Table>() {
                        return 2 as i32;
                    }
                    if (unsafe { (*pTab).nTabRef }) >= ((65535 as i32) as u32) {
                        unsafe {
                            sqlite3ErrorMsg(
                                pParse,
                                (b"too many references to \"%s\": max 65535\0".as_ptr() as *mut i8)
                                    as *const i8,
                                unsafe { (*pTab).zName },
                            )
                        };
                        unsafe {
                            (*pFrom).pSTab = std::ptr::null_mut::<Table>();
                        }
                        return 2 as i32;
                    }
                    let __v2833: *mut Table = pTab;
                    let __v2834: u32 = unsafe { (*__v2833).nTabRef };
                    let __v2835: u32 = __v2834.wrapping_add((1 as i32) as u32);
                    unsafe {
                        (*__v2833).nTabRef = __v2835;
                    }
                    let __v2836: bool;
                    if !((((unsafe { (*pTab).eTabType }) as u32) as i32) == (1 as i32)) {
                        __v2836 = cannotBeFunction(pParse, pFrom) != (0 as i32);
                    } else {
                        __v2836 = false as bool;
                    }
                    if __v2836 {
                        return 2 as i32;
                    }
                    if !((((unsafe { (*pTab).eTabType }) as u32) as i32) == (0 as i32)) {
                        let mut nCol: i16 = 0 as i16;
                        let mut eCodeOrig: u8 = (unsafe { (*pWalker).eCode }) as u8;
                        if (unsafe { sqlite3ViewGetColumnNames(pParse, pTab) }) != (0 as i32) {
                            return 2 as i32;
                        }
                        0 as i32;
                        if (((unsafe { (*pTab).eTabType }) as u32) as i32) == (2 as i32) {
                            if (unsafe { (*db).flags }) & ((2147483648 as u32) as u64)
                                == (((0 as i32) as i64) as u64)
                                && (unsafe { (*pTab).pSchema })
                                    != unsafe {
                                        (*unsafe {
                                            unsafe { (*db).aDb }.offset((1 as i32) as isize)
                                        })
                                        .pSchema
                                    }
                            {
                                unsafe {
                                    sqlite3ErrorMsg(
                                        pParse,
                                        (b"access to view \"%s\" prohibited\0".as_ptr() as *mut i8)
                                            as *const i8,
                                        unsafe { (*pTab).zName },
                                    )
                                };
                            }
                            unsafe {
                                sqlite3SrcItemAttachSubquery(
                                    pParse,
                                    pFrom,
                                    unsafe { (*pTab).u.view.pSelect },
                                    1 as i32,
                                )
                            };
                        } else {
                            if (((unsafe { (*pTab).eTabType }) as u32) as i32) == (1 as i32)
                                && (((unsafe { (*pFrom).fg.__slate_bits_0.__get_fromDDL() })
                                    as i32)
                                    != (0 as i32)
                                    || (((unsafe { (*pParse).prepFlags }) as u32) as i32)
                                        & (32 as i32)
                                        != (0 as i32))
                                && (unsafe { (*pTab).u.vtab.p }) != std::ptr::null_mut::<VTable>()
                                && (((unsafe { (*unsafe { (*pTab).u.vtab.p }).eVtabRisk }) as u32)
                                    as i32)
                                    > (((unsafe { (*db).flags }) & (((128 as i32) as i64) as u64)
                                        != (((0 as i32) as i64) as u64))
                                        as i32)
                            {
                                unsafe {
                                    sqlite3ErrorMsg(
                                        pParse,
                                        (b"unsafe use of virtual table \"%s\"\0".as_ptr()
                                            as *mut i8)
                                            as *const i8,
                                        unsafe { (*pTab).zName },
                                    )
                                };
                            }
                        }
                        0 as i32;
                        nCol = unsafe { (*pTab).nCol };
                        unsafe {
                            (*pTab).nCol = -(1 as i32) as i16;
                        }
                        // /* Turn on Select.selId renumbering */
                        unsafe {
                            (*pWalker).eCode = ((1 as i32) as i16) as u16;
                        }
                        if ((unsafe { (*pFrom).fg.__slate_bits_0.__get_isSubquery() }) as i32)
                            != (0 as i32)
                        {
                            unsafe {
                                sqlite3WalkSelect(pWalker, unsafe {
                                    (*unsafe { (*pFrom).u4.pSubq }).pSelect
                                })
                            };
                        }
                        unsafe {
                            (*pWalker).eCode = eCodeOrig as u16;
                        }
                        unsafe {
                            (*pTab).nCol = nCol;
                        }
                    }
                }
            }
            // /* Locate the index named by the INDEXED BY clause, if any. */
            let __v2837: bool;
            if ((unsafe { (*pFrom).fg.__slate_bits_0.__get_isIndexedBy() }) as i32) != (0 as i32) {
                __v2837 = sqlite3IndexedByLookup(pParse, pFrom) != (0 as i32);
            } else {
                __v2837 = false as bool;
            }
            if __v2837 {
                return 2 as i32;
            }
        }
        let __v2827: i32 = i;
        let __v2828: i32 = __v2827 + (1 as i32);
        i = __v2828;
        let __v2829: *mut SrcItem = pFrom;
        let __v2830: *mut SrcItem = unsafe { __v2829.offset((1 as i32) as isize) };
        pFrom = __v2830;
    }
    // /* Process NATURAL keywords, and ON and USING clauses of joins.
    //   */
    0 as i32;
    let __v2838: bool;
    if (unsafe { (*pParse).nErr }) != (0 as i32) {
        __v2838 = true as bool;
    } else {
        __v2838 = sqlite3ProcessJoin(pParse, p) != (0 as i32);
    }
    if __v2838 {
        return 2 as i32;
    }
    // /* For every "*" that occurs in the column list, insert the names of
    //   ** all columns in all tables.  And for every TABLE.* insert the names
    //   ** of all columns in TABLE.  The parser inserted a special expression
    //   ** with the TK_ASTERISK operator for each "*" that it found in the column
    //   ** list.  The following code just has to locate the TK_ASTERISK
    //   ** expressions and expand each one to the list of all columns in
    //   ** all tables.
    //   **
    //   ** The first loop just checks to see if there are any "*" operators
    //   ** that need expanding.
    //   */
    k = 0 as i32;
    '__slate_break_2160: loop {
        if !(k < unsafe { (*pEList).nExpr }) {
            break;
        }
        pE = unsafe {
            (*unsafe {
                unsafe { std::ptr::addr_of_mut!((*pEList).a) as *mut ExprList_item }
                    .offset(k as isize)
            })
            .pExpr
        };
        if (((unsafe { (*pE).op }) as u32) as i32) == (180 as i32) {
            break '__slate_break_2160;
        }
        0 as i32;
        0 as i32;
        if (((unsafe { (*pE).op }) as u32) as i32) == (142 as i32)
            && (((unsafe { (*unsafe { (*pE).pRight }).op }) as u32) as i32) == (180 as i32)
        {
            break '__slate_break_2160;
        }
        let __v2841: u32 = elistFlags;
        let __v2842: u32 = __v2841 | unsafe { (*pE).flags };
        elistFlags = __v2842;
        let __v2839: i32 = k;
        let __v2840: i32 = __v2839 + (1 as i32);
        k = __v2840;
    }
    if k < unsafe { (*pEList).nExpr } {
        // /*
        //     ** If we get here it means the result set contains one or more "*"
        //     ** operators that need to be expanded.  Loop through each expression
        //     ** in the result set and expand them one by one.
        //     */
        let mut a: *mut ExprList_item =
            unsafe { std::ptr::addr_of_mut!((*pEList).a) as *mut ExprList_item };
        let mut pNew: *mut ExprList = std::ptr::null_mut::<ExprList>();
        let mut flags: i32 = ((unsafe { (*unsafe { (*pParse).db }).flags }) as u32) as i32;
        let mut longNames: i32 =
            (flags & (4 as i32) != (0 as i32) && flags & (64 as i32) == (0 as i32)) as i32;
        k = 0 as i32;
        '__slate_break_2161: loop {
            if !(k < unsafe { (*pEList).nExpr }) {
                break;
            }
            pE = unsafe { (*unsafe { a.offset(k as isize) }).pExpr };
            let __v2845: u32 = elistFlags;
            let __v2846: u32 = __v2845 | unsafe { (*pE).flags };
            elistFlags = __v2846;
            pRight = unsafe { (*pE).pRight };
            0 as i32;
            if (((unsafe { (*pE).op }) as u32) as i32) != (180 as i32)
                && ((((unsafe { (*pE).op }) as u32) as i32) != (142 as i32)
                    || (((unsafe { (*pRight).op }) as u32) as i32) != (180 as i32))
            {
                // /* This particular expression does not need to be expanded.
                //         */
                pNew = unsafe {
                    sqlite3ExprListAppend(pParse, pNew, unsafe {
                        (*unsafe { a.offset(k as isize) }).pExpr
                    })
                };
                if pNew != std::ptr::null_mut::<ExprList>() {
                    unsafe {
                        (*unsafe {
                            unsafe { std::ptr::addr_of_mut!((*pNew).a) as *mut ExprList_item }
                                .offset(((unsafe { (*pNew).nExpr }) - (1 as i32)) as isize)
                        })
                        .zEName = unsafe { (*unsafe { a.offset(k as isize) }).zEName };
                    }
                    unsafe {
                        (*unsafe {
                            unsafe { std::ptr::addr_of_mut!((*pNew).a) as *mut ExprList_item }
                                .offset(((unsafe { (*pNew).nExpr }) - (1 as i32)) as isize)
                        })
                        .fg
                        .__slate_bits_0
                        .__set_eEName(
                            ((unsafe {
                                (*unsafe { a.offset(k as isize) })
                                    .fg
                                    .__slate_bits_0
                                    .__get_eEName()
                            }) as i32) as u32,
                        );
                    }
                    unsafe {
                        (*unsafe { a.offset(k as isize) }).zEName = std::ptr::null_mut::<i8>();
                    }
                }
                unsafe {
                    (*unsafe { a.offset(k as isize) }).pExpr = std::ptr::null_mut::<Expr>();
                }
            } else {
                // /* This expression is a "*" or a "TABLE.*" and needs to be
                //         ** expanded. */
                // /* Set to 1 when TABLE matches */
                let mut tableSeen: i32 = 0 as i32;
                // /* text of name of TABLE */
                let mut zTName: *mut i8 = std::ptr::null_mut::<i8>();
                let mut iErrOfst: i32 = 0 as i32;
                if (((unsafe { (*pE).op }) as u32) as i32) == (142 as i32) {
                    0 as i32;
                    0 as i32;
                    0 as i32;
                    zTName = unsafe { (*unsafe { (*pE).pLeft }).u.zToken };
                    0 as i32;
                    iErrOfst = unsafe { (*unsafe { (*pE).pRight }).w.iOfst };
                } else {
                    0 as i32;
                    iErrOfst = unsafe { (*pE).w.iOfst };
                }
                i = 0 as i32;
                let __v2847: *mut SrcItem =
                    unsafe { std::ptr::addr_of_mut!((*pTabList).a) as *mut SrcItem };
                pFrom = __v2847;
                '__slate_break_2162: while i < unsafe { (*pTabList).nSrc } {
                    '__slate_continue_2162: {
                        // /* Number of cols including rowid */
                        let mut nAdd: i32 = 0 as i32;
                        // /* Table for this data source */
                        let mut pTab: *mut Table = unsafe { (*pFrom).pSTab };
                        // /* Result-set of a nested FROM clause */
                        let mut pNestedFrom: *mut ExprList = unsafe { std::mem::zeroed() };
                        // /* AS name for this data source */
                        let mut zTabName: *mut i8 = unsafe { std::mem::zeroed() };
                        // /* Schema name for this data source */
                        let mut zSchemaName: *const i8 = std::ptr::null::<i8>();
                        // /* Schema index for this data src */
                        let mut iDb: i32 = 0 as i32;
                        // /* USING clause for pFrom[1] */
                        let mut pUsing: *mut IdList = unsafe { std::mem::zeroed() };
                        let __v2852: *mut i8 = unsafe { (*pFrom).zAlias };
                        zTabName = __v2852;
                        if __v2852 == std::ptr::null_mut::<i8>() {
                            zTabName = unsafe { (*pTab).zName };
                        }
                        if (unsafe { (*db).mallocFailed }) != (0 as u8) {
                            break '__slate_break_2162;
                        }
                        0 as i32;
                        if ((unsafe { (*pFrom).fg.__slate_bits_0.__get_isNestedFrom() }) as i32)
                            != (0 as i32)
                        {
                            0 as i32;
                            0 as i32;
                            pNestedFrom = unsafe {
                                (*unsafe { (*unsafe { (*pFrom).u4.pSubq }).pSelect }).pEList
                            };
                            0 as i32;
                            0 as i32;
                            0 as i32;
                        } else {
                            let __v2853: bool;
                            if zTName != std::ptr::null_mut::<i8>() {
                                __v2853 = (unsafe {
                                    sqlite3StrICmp(zTName as *const i8, zTabName as *const i8)
                                }) != (0 as i32);
                            } else {
                                __v2853 = false as bool;
                            }
                            if __v2853 {
                                break '__slate_continue_2162;
                            }
                            pNestedFrom = std::ptr::null_mut::<ExprList>();
                            iDb = unsafe { sqlite3SchemaToIndex(db, unsafe { (*pTab).pSchema }) };
                            zSchemaName = (if iDb >= (0 as i32) {
                                unsafe {
                                    (*unsafe { unsafe { (*db).aDb }.offset(iDb as isize) }).zDbSName
                                }
                            } else {
                                b"*\0".as_ptr() as *mut i8
                            }) as *const i8;
                        }
                        if i + (1 as i32) < unsafe { (*pTabList).nSrc }
                            && ((unsafe {
                                (*unsafe { pFrom.offset((1 as i32) as isize) })
                                    .fg
                                    .__slate_bits_0
                                    .__get_isUsing()
                            }) as i32)
                                != (0 as i32)
                            && ((selFlags as u32) as i32) & (2048 as i32) != (0 as i32)
                        {
                            let mut ii: i32 = 0 as i32;
                            pUsing = unsafe {
                                (*unsafe { pFrom.offset((1 as i32) as isize) }).u3.pUsing
                            };
                            ii = 0 as i32;
                            '__slate_break_2164: loop {
                                if !(ii < unsafe { (*pUsing).nId }) {
                                    break;
                                }
                                let mut zUName: *const i8 = (unsafe {
                                    (*unsafe {
                                        unsafe {
                                            std::ptr::addr_of_mut!((*pUsing).a) as *mut IdList_item
                                        }
                                        .offset(ii as isize)
                                    })
                                    .zName
                                })
                                    as *const i8;
                                pRight = unsafe { sqlite3Expr(db, 60 as i32, zUName) };
                                unsafe { sqlite3ExprSetErrorOffset(pRight, iErrOfst) };
                                pNew = unsafe { sqlite3ExprListAppend(pParse, pNew, pRight) };
                                if pNew != std::ptr::null_mut::<ExprList>() {
                                    let mut pX: *mut ExprList_item = unsafe {
                                        unsafe {
                                            std::ptr::addr_of_mut!((*pNew).a) as *mut ExprList_item
                                        }
                                        .offset(((unsafe { (*pNew).nExpr }) - (1 as i32)) as isize)
                                    };
                                    0 as i32;
                                    unsafe {
                                        (*pX).zEName = unsafe {
                                            sqlite3MPrintf(
                                                db,
                                                (b"..%s\0".as_ptr() as *mut i8) as *const i8,
                                                zUName,
                                            )
                                        };
                                    }
                                    unsafe {
                                        (*pX).fg.__slate_bits_0.__set_eEName((2 as i32) as u32);
                                    }
                                    unsafe {
                                        (*pX).fg.__slate_bits_0.__set_bUsingTerm((1 as i32) as u32);
                                    }
                                }
                                let __v2854: i32 = ii;
                                let __v2855: i32 = __v2854 + (1 as i32);
                                ii = __v2855;
                            }
                        } else {
                            pUsing = std::ptr::null_mut::<IdList>();
                        }
                        nAdd = (unsafe { (*pTab).nCol }) as i32;
                        if (unsafe { (*pTab).tabFlags }) & ((512 as i32) as u32)
                            == ((0 as i32) as u32)
                            && ((selFlags as u32) as i32) & (2048 as i32) != (0 as i32)
                        {
                            let __v2856: i32 = nAdd;
                            let __v2857: i32 = __v2856 + (1 as i32);
                            nAdd = __v2857;
                        }
                        j = 0 as i32;
                        '__slate_break_2166: loop {
                            if !(j < nAdd) {
                                break;
                            }
                            '__slate_continue_2166: {
                                let mut zName: *const i8 = unsafe { std::mem::zeroed() };
                                // /* Newly added ExprList term */
                                let mut pX: *mut ExprList_item = unsafe { std::mem::zeroed() };
                                if j == ((unsafe { (*pTab).nCol }) as i32) {
                                    zName = unsafe { sqlite3RowidAlias(pTab) };
                                    if zName == std::ptr::null::<i8>() {
                                        break '__slate_continue_2166;
                                    }
                                } else {
                                    zName = (unsafe {
                                        (*unsafe { unsafe { (*pTab).aCol }.offset(j as isize) })
                                            .zCnName
                                    }) as *const i8;
                                    // /* If pTab is actually an SF_NestedFrom sub-select, do not
                                    //               ** expand any ENAME_ROWID columns.  */
                                    if pNestedFrom != std::ptr::null_mut::<ExprList>()
                                        && ((unsafe {
                                            (*unsafe {
                                                unsafe {
                                                    std::ptr::addr_of_mut!((*pNestedFrom).a)
                                                        as *mut ExprList_item
                                                }
                                                .offset(j as isize)
                                            })
                                            .fg
                                            .__slate_bits_0
                                            .__get_eEName()
                                        }) as i32)
                                            == (3 as i32)
                                    {
                                        break '__slate_continue_2166;
                                    }
                                    let __v2860: bool;
                                    if zTName != std::ptr::null_mut::<i8>()
                                        && pNestedFrom != std::ptr::null_mut::<ExprList>()
                                    {
                                        __v2860 = (unsafe {
                                            sqlite3MatchEName(
                                                (unsafe {
                                                    unsafe {
                                                        std::ptr::addr_of_mut!((*pNestedFrom).a)
                                                            as *mut ExprList_item
                                                    }
                                                    .offset(j as isize)
                                                })
                                                    as *const ExprList_item,
                                                std::ptr::null::<i8>(),
                                                zTName as *const i8,
                                                std::ptr::null::<i8>(),
                                                std::ptr::null_mut::<i32>(),
                                            )
                                        }) == (0 as i32);
                                    } else {
                                        __v2860 = false as bool;
                                    }
                                    if __v2860 {
                                        break '__slate_continue_2166;
                                    }
                                    // /* If a column is marked as 'hidden', omit it from the expanded
                                    //               ** result-set list unless the SELECT has the SF_IncludeHidden
                                    //               ** bit set.
                                    //               */
                                    if (unsafe { (*p).selFlags }) & ((131072 as i32) as u32)
                                        == ((0 as i32) as u32)
                                        && (((unsafe {
                                            (*unsafe { unsafe { (*pTab).aCol }.offset(j as isize) })
                                                .colFlags
                                        }) as u32)
                                            as i32)
                                            & (2 as i32)
                                            != (0 as i32)
                                    {
                                        break '__slate_continue_2166;
                                    }
                                    if (((unsafe {
                                        (*unsafe { unsafe { (*pTab).aCol }.offset(j as isize) })
                                            .colFlags
                                    }) as u32) as i32)
                                        & (1024 as i32)
                                        != (0 as i32)
                                        && zTName == std::ptr::null_mut::<i8>()
                                        && ((selFlags as u32) as i32) & (2048 as i32) == (0 as i32)
                                    {
                                        break '__slate_continue_2166;
                                    }
                                }
                                0 as i32;
                                tableSeen = 1 as i32;
                                if i > (0 as i32)
                                    && zTName == std::ptr::null_mut::<i8>()
                                    && ((selFlags as u32) as i32) & (2048 as i32) == (0 as i32)
                                {
                                    let __v2861: bool;
                                    if ((unsafe { (*pFrom).fg.__slate_bits_0.__get_isUsing() })
                                        as i32)
                                        != (0 as i32)
                                    {
                                        __v2861 = (unsafe {
                                            sqlite3IdListIndex(unsafe { (*pFrom).u3.pUsing }, zName)
                                        }) >= (0 as i32);
                                    } else {
                                        __v2861 = false as bool;
                                    }
                                    if __v2861 {
                                        // /* In a join with a USING clause, omit columns in the
                                        //                 ** using clause from the table on the right. */
                                        break '__slate_continue_2166;
                                    }
                                }
                                pRight = unsafe { sqlite3Expr(db, 60 as i32, zName) };
                                let __v2862: bool;
                                if (unsafe { (*pTabList).nSrc }) > (1 as i32) {
                                    let __v2863: bool;
                                    if (((unsafe { (*pFrom).fg.jointype }) as u32) as i32)
                                        & (64 as i32)
                                        == (0 as i32)
                                        || ((selFlags as u32) as i32) & (2048 as i32) != (0 as i32)
                                    {
                                        __v2863 = true as bool;
                                    } else {
                                        __v2863 = !(inAnyUsingClause(
                                            zName,
                                            pFrom,
                                            (unsafe { (*pTabList).nSrc }) - i - (1 as i32),
                                        ) != (0 as i32));
                                    }
                                    __v2862 = __v2863;
                                } else {
                                    __v2862 = false as bool;
                                }
                                if __v2862
                                    || (((unsafe { (*pParse).eParseMode }) as u32) as i32)
                                        >= (2 as i32)
                                {
                                    let mut pLeft: *mut Expr = unsafe { std::mem::zeroed() };
                                    pLeft = unsafe {
                                        sqlite3Expr(db, 60 as i32, zTabName as *const i8)
                                    };
                                    pExpr =
                                        unsafe { sqlite3PExpr(pParse, 142 as i32, pLeft, pRight) };
                                    if (((unsafe { (*pParse).eParseMode }) as u32) as i32)
                                        >= (2 as i32)
                                        && (unsafe { (*pE).pLeft }) != std::ptr::null_mut::<Expr>()
                                    {
                                        unsafe {
                                            sqlite3RenameTokenRemap(
                                                pParse,
                                                pLeft as *const (),
                                                (unsafe { (*pE).pLeft }) as *const (),
                                            )
                                        };
                                    }
                                    if zSchemaName != std::ptr::null::<i8>() {
                                        pLeft = unsafe { sqlite3Expr(db, 60 as i32, zSchemaName) };
                                        pExpr = unsafe {
                                            sqlite3PExpr(pParse, 142 as i32, pLeft, pExpr)
                                        };
                                    }
                                } else {
                                    pExpr = pRight;
                                }
                                unsafe { sqlite3ExprSetErrorOffset(pExpr, iErrOfst) };
                                pNew = unsafe { sqlite3ExprListAppend(pParse, pNew, pExpr) };
                                if pNew == std::ptr::null_mut::<ExprList>() {
                                    // /* OOM */
                                    break '__slate_break_2166;
                                }
                                pX = unsafe {
                                    unsafe {
                                        std::ptr::addr_of_mut!((*pNew).a) as *mut ExprList_item
                                    }
                                    .offset(((unsafe { (*pNew).nExpr }) - (1 as i32)) as isize)
                                };
                                0 as i32;
                                if ((selFlags as u32) as i32) & (2048 as i32) != (0 as i32)
                                    && !((((unsafe { (*pParse).eParseMode }) as u32) as i32)
                                        >= (2 as i32))
                                {
                                    if pNestedFrom != std::ptr::null_mut::<ExprList>()
                                        && (!((0 as i32) != (0 as i32))
                                            || j < unsafe { (*pNestedFrom).nExpr })
                                    {
                                        0 as i32;
                                        unsafe {
                                            (*pX).zEName = unsafe {
                                                sqlite3DbStrDup(
                                                    db,
                                                    (unsafe {
                                                        (*unsafe {
                                                            unsafe {
                                                                std::ptr::addr_of_mut!(
                                                                    (*pNestedFrom).a
                                                                )
                                                                    as *mut ExprList_item
                                                            }
                                                            .offset(j as isize)
                                                        })
                                                        .zEName
                                                    })
                                                        as *const i8,
                                                )
                                            };
                                        }
                                        {}
                                    } else {
                                        unsafe {
                                            (*pX).zEName = unsafe {
                                                sqlite3MPrintf(
                                                    db,
                                                    (b"%s.%s.%s\0".as_ptr() as *mut i8)
                                                        as *const i8,
                                                    zSchemaName,
                                                    zTabName,
                                                    zName,
                                                )
                                            };
                                        }
                                        {}
                                    }
                                    unsafe {
                                        (*pX).fg.__slate_bits_0.__set_eEName(
                                            (if j == ((unsafe { (*pTab).nCol }) as i32) {
                                                3 as i32
                                            } else {
                                                2 as i32
                                            }) as u32,
                                        );
                                    }
                                    let __v2864: bool;
                                    if ((unsafe { (*pFrom).fg.__slate_bits_0.__get_isUsing() })
                                        as i32)
                                        != (0 as i32)
                                    {
                                        __v2864 = (unsafe {
                                            sqlite3IdListIndex(unsafe { (*pFrom).u3.pUsing }, zName)
                                        }) >= (0 as i32);
                                    } else {
                                        __v2864 = false as bool;
                                    }
                                    let __v2865: bool;
                                    if __v2864 {
                                        __v2865 = true as bool;
                                    } else {
                                        let __v2866: bool;
                                        if pUsing != std::ptr::null_mut::<IdList>() {
                                            __v2866 =
                                                (unsafe { sqlite3IdListIndex(pUsing, zName) })
                                                    >= (0 as i32);
                                        } else {
                                            __v2866 = false as bool;
                                        }
                                        __v2865 = __v2866;
                                    }
                                    if __v2865
                                        || j < ((unsafe { (*pTab).nCol }) as i32)
                                            && (((unsafe {
                                                (*unsafe {
                                                    unsafe { (*pTab).aCol }.offset(j as isize)
                                                })
                                                .colFlags
                                            })
                                                as u32)
                                                as i32)
                                                & (1024 as i32)
                                                != (0 as i32)
                                    {
                                        unsafe {
                                            (*pX)
                                                .fg
                                                .__slate_bits_0
                                                .__set_bNoExpand((1 as i32) as u32);
                                        }
                                    }
                                } else {
                                    if longNames != (0 as i32) {
                                        unsafe {
                                            (*pX).zEName = unsafe {
                                                sqlite3MPrintf(
                                                    db,
                                                    (b"%s.%s\0".as_ptr() as *mut i8) as *const i8,
                                                    zTabName,
                                                    zName,
                                                )
                                            };
                                        }
                                        unsafe {
                                            (*pX).fg.__slate_bits_0.__set_eEName((0 as i32) as u32);
                                        }
                                    } else {
                                        unsafe {
                                            (*pX).zEName = unsafe { sqlite3DbStrDup(db, zName) };
                                        }
                                        unsafe {
                                            (*pX).fg.__slate_bits_0.__set_eEName((0 as i32) as u32);
                                        }
                                    }
                                }
                            }
                            let __v2858: i32 = j;
                            let __v2859: i32 = __v2858 + (1 as i32);
                            j = __v2859;
                        }
                    }
                    let __v2848: i32 = i;
                    let __v2849: i32 = __v2848 + (1 as i32);
                    i = __v2849;
                    let __v2850: *mut SrcItem = pFrom;
                    let __v2851: *mut SrcItem = unsafe { __v2850.offset((1 as i32) as isize) };
                    pFrom = __v2851;
                }
                if !(tableSeen != (0 as i32)) {
                    if zTName != std::ptr::null_mut::<i8>() {
                        unsafe {
                            sqlite3ErrorMsg(
                                pParse,
                                (b"no such table: %s\0".as_ptr() as *mut i8) as *const i8,
                                zTName,
                            )
                        };
                    } else {
                        unsafe {
                            sqlite3ErrorMsg(
                                pParse,
                                (b"no tables specified\0".as_ptr() as *mut i8) as *const i8,
                            )
                        };
                    }
                }
            }
            let __v2843: i32 = k;
            let __v2844: i32 = __v2843 + (1 as i32);
            k = __v2844;
        }
        unsafe { sqlite3ExprListDelete(db, pEList) };
        unsafe {
            (*p).pEList = pNew;
        }
    }
    if (unsafe { (*p).pEList }) != std::ptr::null_mut::<ExprList>() {
        if (unsafe { (*unsafe { (*p).pEList }).nExpr })
            > unsafe {
                *unsafe {
                    unsafe { (*db).aLimit.as_mut_ptr() as *mut i32 }.offset((2 as i32) as isize)
                }
            }
        {
            unsafe {
                sqlite3ErrorMsg(
                    pParse,
                    (b"too many columns in result set\0".as_ptr() as *mut i8) as *const i8,
                )
            };
            return 2 as i32;
        }
        if elistFlags & (((8 as i32) | (4194304 as i32)) as u32) != ((0 as i32) as u32) {
            let __v2867: *mut Select = p;
            let __v2868: u32 = unsafe { (*__v2867).selFlags };
            let __v2869: u32 = __v2868 | ((262144 as i32) as u32);
            unsafe {
                (*__v2867).selFlags = __v2869;
            }
        }
    }
    return 0 as i32;
}

// /*
// ** This routine "expands" a SELECT statement and all of its subqueries.
// ** For additional information on what it means to "expand" a SELECT
// ** statement, see the comment on the selectExpand worker callback above.
// **
// ** Expanding a SELECT statement is the first step in processing a
// ** SELECT statement.  The SELECT statement must be expanded before
// ** name resolution is performed.
// **
// ** If anything goes wrong, an error message is written into pParse.
// ** The calling function can detect the problem by looking at pParse->nErr
// ** and/or pParse->db->mallocFailed.
// */
fn sqlite3SelectExpand(mut pParse: *mut Parse, mut pSelect: *mut Select) {
    let mut w: Walker = unsafe { std::mem::zeroed() };
    w.xExprCallback = unsafe {
        std::mem::transmute::<
            *const (),
            Option<unsafe extern "C-unwind" fn(*mut Walker, *mut Expr) -> i32>,
        >(sqlite3ExprWalkNoop as *const ())
    };
    w.pParse = pParse;
    if ((unsafe { (*pParse).__slate_bits_0.__get_hasCompound() }) as i32) != (0 as i32) {
        w.xSelectCallback = Some(convertCompoundSelectToSubquery);
        w.xSelectCallback2 = None;
        unsafe { sqlite3WalkSelect(std::ptr::addr_of_mut!(w), pSelect) };
    }
    w.xSelectCallback = Some(selectExpander);
    w.xSelectCallback2 = Some(sqlite3SelectPopWith);
    w.eCode = ((0 as i32) as i16) as u16;
    unsafe { sqlite3WalkSelect(std::ptr::addr_of_mut!(w), pSelect) };
}

// /*
// ** This is a Walker.xSelectCallback callback for the sqlite3SelectTypeInfo()
// ** interface.
// **
// ** For each FROM-clause subquery, add Column.zType, Column.zColl, and
// ** Column.affinity information to the Table structure that represents
// ** the result set of that subquery.
// **
// ** The Table structure that represents the result set was constructed
// ** by selectExpander() but the type and collation and affinity information
// ** was omitted at that point because identifiers had not yet been resolved.
// ** This routine is called after identifier resolution.
// */
#[unsafe(link_section = ".text.slate_distinct.select.selectAddSubqueryTypeInfo")]
extern "C-unwind" fn selectAddSubqueryTypeInfo(mut pWalker: *mut Walker, mut p: *mut Select) {
    let mut pParse: *mut Parse = unsafe { std::mem::zeroed() };
    let mut i: i32 = 0 as i32;
    let mut pTabList: *mut SrcList = unsafe { std::mem::zeroed() };
    let mut pFrom: *mut SrcItem = unsafe { std::mem::zeroed() };
    if (unsafe { (*p).selFlags }) & ((128 as i32) as u32) != (0 as u32) {
        return;
    }
    let __v2870: *mut Select = p;
    let __v2871: u32 = unsafe { (*__v2870).selFlags };
    let __v2872: u32 = __v2871 | ((128 as i32) as u32);
    unsafe {
        (*__v2870).selFlags = __v2872;
    }
    pParse = unsafe { (*pWalker).pParse };
    0 as i32;
    pTabList = unsafe { (*p).pSrc };
    i = 0 as i32;
    let __v2873: *mut SrcItem = unsafe { std::ptr::addr_of_mut!((*pTabList).a) as *mut SrcItem };
    pFrom = __v2873;
    '__slate_break_2172: while i < unsafe { (*pTabList).nSrc } {
        let mut pTab: *mut Table = unsafe { (*pFrom).pSTab };
        0 as i32;
        if (unsafe { (*pTab).tabFlags }) & ((16384 as i32) as u32) != ((0 as i32) as u32)
            && ((unsafe { (*pFrom).fg.__slate_bits_0.__get_isSubquery() }) as i32) != (0 as i32)
        {
            // /* A sub-query in the FROM clause of a SELECT */
            let mut pSel: *mut Select = unsafe { (*unsafe { (*pFrom).u4.pSubq }).pSelect };
            sqlite3SubqueryColumnTypes(pParse, pTab, pSel, (64 as i32) as i8);
        }
        let __v2874: i32 = i;
        let __v2875: i32 = __v2874 + (1 as i32);
        i = __v2875;
        let __v2876: *mut SrcItem = pFrom;
        let __v2877: *mut SrcItem = unsafe { __v2876.offset((1 as i32) as isize) };
        pFrom = __v2877;
    }
}

// /*
// ** This routine adds datatype and collating sequence information to
// ** the Table structures of all FROM-clause subqueries in a
// ** SELECT statement.
// **
// ** Use this routine after name resolution.
// */
fn sqlite3SelectAddTypeInfo(mut pParse: *mut Parse, mut pSelect: *mut Select) {
    let mut w: Walker = unsafe { std::mem::zeroed() };
    w.xSelectCallback = unsafe {
        std::mem::transmute::<
            *const (),
            Option<unsafe extern "C-unwind" fn(*mut Walker, *mut Select) -> i32>,
        >(sqlite3SelectWalkNoop as *const ())
    };
    w.xSelectCallback2 = Some(selectAddSubqueryTypeInfo);
    w.xExprCallback = unsafe {
        std::mem::transmute::<
            *const (),
            Option<unsafe extern "C-unwind" fn(*mut Walker, *mut Expr) -> i32>,
        >(sqlite3ExprWalkNoop as *const ())
    };
    w.pParse = pParse;
    unsafe { sqlite3WalkSelect(std::ptr::addr_of_mut!(w), pSelect) };
}

// /* The parser context */
// /* The SELECT statement being coded. */
// /* Name context for container */
// /*
// ** Analyze the arguments to aggregate functions.  Create new pAggInfo->aCol[]
// ** entries for columns that are arguments to aggregate functions but which
// ** are not otherwise used.
// **
// ** The aCol[] entries in AggInfo prior to nAccumulator are columns that
// ** are referenced outside of aggregate functions.  These might be columns
// ** that are part of the GROUP by clause, for example.  Other database engines
// ** would throw an error if there is a column reference that is not in the
// ** GROUP BY clause and that is not part of an aggregate function argument.
// ** But SQLite allows this.
// **
// ** The aCol[] entries beginning with the aCol[nAccumulator] and following
// ** are column references that are used exclusively as arguments to
// ** aggregate functions.  This routine is responsible for computing
// ** (or recomputing) those aCol[] entries.
// */
fn analyzeAggFuncArgs(mut pAggInfo: *mut AggInfo, mut pNC: *mut NameContext) {
    let mut i: i32 = 0 as i32;
    0 as i32;
    0 as i32;
    let __v2878: *mut NameContext = pNC;
    let __v2879: i32 = unsafe { (*__v2878).ncFlags };
    let __v2880: i32 = __v2879 | (131072 as i32);
    unsafe {
        (*__v2878).ncFlags = __v2880;
    }
    i = 0 as i32;
    '__slate_break_2173: loop {
        if !(i < unsafe { (*pAggInfo).nFunc }) {
            break;
        }
        let mut pExpr: *mut Expr =
            unsafe { (*unsafe { unsafe { (*pAggInfo).aFunc }.offset(i as isize) }).pFExpr };
        0 as i32;
        0 as i32;
        unsafe { sqlite3ExprAnalyzeAggList(pNC, unsafe { (*pExpr).x.pList }) };
        if (unsafe { (*pExpr).pLeft }) != std::ptr::null_mut::<Expr>() {
            0 as i32;
            0 as i32;
            unsafe {
                sqlite3ExprAnalyzeAggList(pNC, unsafe { (*unsafe { (*pExpr).pLeft }).x.pList })
            };
        }
        0 as i32;
        if (unsafe { (*pExpr).flags }) & ((16777216 as i32) as u32) != ((0 as i32) as u32) {
            unsafe {
                sqlite3ExprAnalyzeAggregates(pNC, unsafe { (*unsafe { (*pExpr).y.pWin }).pFilter })
            };
        }
        let __v2881: i32 = i;
        let __v2882: i32 = __v2881 + (1 as i32);
        i = __v2882;
    }
    let __v2883: *mut NameContext = pNC;
    let __v2884: i32 = unsafe { (*__v2883).ncFlags };
    let __v2885: i32 = __v2884 & !(131072 as i32);
    unsafe {
        (*__v2883).ncFlags = __v2885;
    }
}

// /*
// ** An index on expressions is being used in the inner loop of an
// ** aggregate query with a GROUP BY clause.  This routine attempts
// ** to adjust the AggInfo object to take advantage of index and to
// ** perhaps use the index as a covering index.
// **
// */
fn optimizeAggregateUseOfIndexedExpr(
    mut pParse: *mut Parse,
    mut pSelect: *mut Select,
    mut pAggInfo: *mut AggInfo,
    mut pNC: *mut NameContext,
) {
    0 as i32;
    0 as i32;
    0 as i32;
    unsafe {
        (*pAggInfo).nColumn = unsafe { (*pAggInfo).nAccumulator };
    }
    if (unsafe { (*pAggInfo).nSortingColumn }) > ((0 as i32) as u32) {
        let mut mx: i32 = (unsafe { (*unsafe { (*pSelect).pGroupBy }).nExpr }) - (1 as i32);
        let mut j: i32 = 0 as i32;
        let mut k: i32 = 0 as i32;
        j = 0 as i32;
        '__slate_break_2174: loop {
            if !(j < unsafe { (*pAggInfo).nColumn }) {
                break;
            }
            k = unsafe {
                (*unsafe { unsafe { (*pAggInfo).aCol }.offset(j as isize) }).iSorterColumn
            };
            if k > mx {
                mx = k;
            }
            let __v2886: i32 = j;
            let __v2887: i32 = __v2886 + (1 as i32);
            j = __v2887;
        }
        unsafe {
            (*pAggInfo).nSortingColumn = (mx + (1 as i32)) as u32;
        }
    }
    analyzeAggFuncArgs(pAggInfo, pNC);
    pSelect;
    pParse;
}

// /* Parsing context */
// /* The SELECT statement being processed */
// /* The aggregate info */
// /* Name context used to resolve agg-func args */
// /*
// ** Walker callback for aggregateConvertIndexedExprRefToColumn().
// */
#[unsafe(link_section = ".text.slate_distinct.select.aggregateIdxEprRefToColCallback")]
extern "C-unwind" fn aggregateIdxEprRefToColCallback(
    mut pWalker: *mut Walker,
    mut pExpr: *mut Expr,
) -> i32 {
    let mut pAggInfo: *mut AggInfo = unsafe { std::mem::zeroed() };
    let mut pCol: *mut AggInfo_col = unsafe { std::mem::zeroed() };
    pWalker;
    if (unsafe { (*pExpr).pAggInfo }) == std::ptr::null_mut::<AggInfo>() {
        return 0 as i32;
    }
    if (((unsafe { (*pExpr).op }) as u32) as i32) == (170 as i32) {
        return 0 as i32;
    }
    if (((unsafe { (*pExpr).op }) as u32) as i32) == (169 as i32) {
        return 0 as i32;
    }
    if (((unsafe { (*pExpr).op }) as u32) as i32) == (179 as i32) {
        return 0 as i32;
    }
    pAggInfo = unsafe { (*pExpr).pAggInfo };
    if ((unsafe { (*pExpr).iAgg }) as i32) >= unsafe { (*pAggInfo).nColumn } {
        return 0 as i32;
    }
    0 as i32;
    pCol =
        unsafe { unsafe { (*pAggInfo).aCol }.offset(((unsafe { (*pExpr).iAgg }) as i32) as isize) };
    unsafe {
        (*pExpr).op = ((170 as i32) as i8) as u8;
    }
    unsafe {
        (*pExpr).iTable = unsafe { (*pCol).iTable };
    }
    unsafe {
        (*pExpr).iColumn = (unsafe { (*pCol).iColumn }) as i16;
    }
    let __v2888: *mut Expr = pExpr;
    let __v2889: u32 = unsafe { (*__v2888).flags };
    let __v2890: u32 = __v2889 & !(((8192 as i32) | (512 as i32) | (524288 as i32)) as u32);
    unsafe {
        (*__v2888).flags = __v2890;
    }
    return 1 as i32;
}

// /*
// ** Convert every pAggInfo->aFunc[].pExpr such that any node within
// ** those expressions that has pAppInfo set is changed into a TK_AGG_COLUMN
// ** opcode.
// */
fn aggregateConvertIndexedExprRefToColumn(mut pAggInfo: *mut AggInfo) {
    let mut i: i32 = 0 as i32;
    let mut w: Walker = unsafe { std::mem::zeroed() };
    unsafe { memset(std::ptr::addr_of_mut!(w) as *mut (), 0 as i32, 48 as u64) };
    w.xExprCallback = Some(aggregateIdxEprRefToColCallback);
    i = 0 as i32;
    '__slate_break_2175: loop {
        if !(i < unsafe { (*pAggInfo).nFunc }) {
            break;
        }
        unsafe {
            sqlite3WalkExpr(std::ptr::addr_of_mut!(w), unsafe {
                (*unsafe { unsafe { (*pAggInfo).aFunc }.offset(i as isize) }).pFExpr
            })
        };
        let __v2891: i32 = i;
        let __v2892: i32 = __v2891 + (1 as i32);
        i = __v2892;
    }
}

// /*
// ** Allocate a block of registers so that there is one register for each
// ** pAggInfo->aCol[] and pAggInfo->aFunc[] entry in pAggInfo.  The first
// ** register in this block is stored in pAggInfo->iFirstReg.
// **
// ** This routine may only be called once for each AggInfo object.  Prior
// ** to calling this routine:
// **
// **     *  The aCol[] and aFunc[] arrays may be modified
// **     *  The AggInfoColumnReg() and AggInfoFuncReg() macros may not be used
// **
// ** After calling this routine:
// **
// **     *  The aCol[] and aFunc[] arrays are fixed
// **     *  The AggInfoColumnReg() and AggInfoFuncReg() macros may be used
// **
// */
fn assignAggregateRegisters(mut pParse: *mut Parse, mut pAggInfo: *mut AggInfo) {
    0 as i32;
    0 as i32;
    unsafe {
        (*pAggInfo).iFirstReg = (unsafe { (*pParse).nMem }) + (1 as i32);
    }
    let __v2893: *mut Parse = pParse;
    let __v2894: i32 = unsafe { (*__v2893).nMem };
    let __v2895: i32 = __v2894 + ((unsafe { (*pAggInfo).nColumn }) + unsafe { (*pAggInfo).nFunc });
    unsafe {
        (*__v2893).nMem = __v2895;
    }
}

// /*
// ** Reset the aggregate accumulator.
// **
// ** The aggregate accumulator is a set of memory cells that hold
// ** intermediate results while calculating an aggregate.  This
// ** routine generates code that stores NULLs in all of those memory
// ** cells.
// */
fn resetAccumulator(mut pParse: *mut Parse, mut pAggInfo: *mut AggInfo) {
    let mut v: *mut Vdbe = unsafe { (*pParse).pVdbe };
    let mut i: i32 = 0 as i32;
    let mut pFunc: *mut AggInfo_func = unsafe { std::mem::zeroed() };
    let mut nReg: i32 = (unsafe { (*pAggInfo).nFunc }) + unsafe { (*pAggInfo).nColumn };
    0 as i32;
    0 as i32;
    0 as i32;
    if nReg == (0 as i32) {
        return;
    }
    if (unsafe { (*pParse).nErr }) != (0 as i32) {
        return;
    }
    unsafe {
        sqlite3VdbeAddOp3(
            v,
            77 as i32,
            0 as i32,
            unsafe { (*pAggInfo).iFirstReg },
            (unsafe { (*pAggInfo).iFirstReg }) + nReg - (1 as i32),
        )
    };
    pFunc = unsafe { (*pAggInfo).aFunc };
    i = 0 as i32;
    '__slate_break_2176: while i < unsafe { (*pAggInfo).nFunc } {
        if (unsafe { (*pFunc).iDistinct }) >= (0 as i32) {
            let mut pE: *mut Expr = unsafe { (*pFunc).pFExpr };
            0 as i32;
            if (unsafe { (*pE).x.pList }) == std::ptr::null_mut::<ExprList>()
                || (unsafe { (*unsafe { (*pE).x.pList }).nExpr }) != (1 as i32)
            {
                unsafe {
                    sqlite3ErrorMsg(
                        pParse,
                        (b"DISTINCT aggregates must have exactly one argument\0".as_ptr()
                            as *mut i8) as *const i8,
                    )
                };
                unsafe {
                    (*pFunc).iDistinct = -(1 as i32);
                }
            } else {
                let mut pKeyInfo: *mut KeyInfo = sqlite3KeyInfoFromExprList(
                    pParse,
                    unsafe { (*pE).x.pList },
                    0 as i32,
                    0 as i32,
                );
                unsafe {
                    (*pFunc).iDistAddr = unsafe {
                        sqlite3VdbeAddOp4(
                            v,
                            120 as i32,
                            unsafe { (*pFunc).iDistinct },
                            0 as i32,
                            0 as i32,
                            (pKeyInfo as *mut i8) as *const i8,
                            -(9 as i32),
                        )
                    };
                }
                unsafe {
                    sqlite3VdbeExplain(
                        pParse,
                        ((0 as i32) as i8) as u8,
                        (b"USE TEMP B-TREE FOR %s(DISTINCT)\0".as_ptr() as *mut i8) as *const i8,
                        unsafe { (*unsafe { (*pFunc).pFunc }).zName },
                    )
                };
            }
        }
        if (unsafe { (*pFunc).iOBTab }) >= (0 as i32) {
            let mut pOBList: *mut ExprList = unsafe { std::mem::zeroed() };
            let mut pKeyInfo: *mut KeyInfo = unsafe { std::mem::zeroed() };
            let mut nExtra: i32 = 0 as i32;
            0 as i32;
            0 as i32;
            0 as i32;
            0 as i32;
            pOBList = unsafe { (*unsafe { (*unsafe { (*pFunc).pFExpr }).pLeft }).x.pList };
            if !((unsafe { (*pFunc).bOBUnique }) != (0 as u8)) {
                // /* One extra column for the OP_Sequence */
                let __v2900: i32 = nExtra;
                let __v2901: i32 = __v2900 + (1 as i32);
                nExtra = __v2901;
            }
            if (unsafe { (*pFunc).bOBPayload }) != (0 as u8) {
                // /* extra columns for the function arguments */
                0 as i32;
                0 as i32;
                let __v2902: i32 = nExtra;
                let __v2903: i32 =
                    __v2902 + unsafe { (*unsafe { (*unsafe { (*pFunc).pFExpr }).x.pList }).nExpr };
                nExtra = __v2903;
            }
            if (unsafe { (*pFunc).bUseSubtype }) != (0 as u8) {
                let __v2904: i32 = nExtra;
                let __v2905: i32 =
                    __v2904 + unsafe { (*unsafe { (*unsafe { (*pFunc).pFExpr }).x.pList }).nExpr };
                nExtra = __v2905;
            }
            pKeyInfo = sqlite3KeyInfoFromExprList(pParse, pOBList, 0 as i32, nExtra);
            if !((unsafe { (*pFunc).bOBUnique }) != (0 as u8))
                && (unsafe { (*pParse).nErr }) == (0 as i32)
            {
                let __v2906: *mut KeyInfo = pKeyInfo;
                let __v2907: u16 = unsafe { (*__v2906).nKeyField };
                let __v2908: u16 = ((((__v2907 as u32) as i32) + (1 as i32)) as i16) as u16;
                unsafe {
                    (*__v2906).nKeyField = __v2908;
                }
            }
            unsafe {
                sqlite3VdbeAddOp4(
                    v,
                    120 as i32,
                    unsafe { (*pFunc).iOBTab },
                    (unsafe { (*pOBList).nExpr }) + nExtra,
                    0 as i32,
                    (pKeyInfo as *mut i8) as *const i8,
                    -(9 as i32),
                )
            };
            unsafe {
                sqlite3VdbeExplain(
                    pParse,
                    ((0 as i32) as i8) as u8,
                    (b"USE TEMP B-TREE FOR %s(ORDER BY)\0".as_ptr() as *mut i8) as *const i8,
                    unsafe { (*unsafe { (*pFunc).pFunc }).zName },
                )
            };
        }
        let __v2896: i32 = i;
        let __v2897: i32 = __v2896 + (1 as i32);
        i = __v2897;
        let __v2898: *mut AggInfo_func = pFunc;
        let __v2899: *mut AggInfo_func = unsafe { __v2898.offset((1 as i32) as isize) };
        pFunc = __v2899;
    }
}

// /*
// ** Invoke the OP_AggFinalize opcode for every aggregate function
// ** in the AggInfo structure.
// */
fn finalizeAggFunctions(mut pParse: *mut Parse, mut pAggInfo: *mut AggInfo) {
    let mut v: *mut Vdbe = unsafe { (*pParse).pVdbe };
    let mut i: i32 = 0 as i32;
    let mut pF: *mut AggInfo_func = unsafe { std::mem::zeroed() };
    i = 0 as i32;
    let __v2909: *mut AggInfo_func = unsafe { (*pAggInfo).aFunc };
    pF = __v2909;
    '__slate_break_2180: while i < unsafe { (*pAggInfo).nFunc } {
        let mut pList: *mut ExprList = unsafe { std::mem::zeroed() };
        0 as i32;
        if (unsafe { (*pParse).nErr }) != (0 as i32) {
            return;
        }
        pList = unsafe { (*unsafe { (*pF).pFExpr }).x.pList };
        if (unsafe { (*pF).iOBTab }) >= (0 as i32) {
            // /* For an ORDER BY aggregate, calls to OP_AggStep were deferred.  Inputs
            //       ** were stored in emphermal table pF->iOBTab.  Here, we extract those
            //       ** inputs (in ORDER BY order) and make all calls to OP_AggStep
            //       ** before doing the OP_AggFinal call. */
            // /* Start of loop for extracting columns */
            let mut iTop: i32 = 0 as i32;
            // /* Number of columns to extract */
            let mut nArg: i32 = 0 as i32;
            // /* Key columns to be skipped */
            let mut nKey: i32 = 0 as i32;
            // /* Extract into this array */
            let mut regAgg: i32 = 0 as i32;
            // /* Loop counter */
            let mut j: i32 = 0 as i32;
            0 as i32;
            nArg = unsafe { (*pList).nExpr };
            regAgg = unsafe { sqlite3GetTempRange(pParse, nArg) };
            if (((unsafe { (*pF).bOBPayload }) as u32) as i32) == (0 as i32) {
                nKey = 0 as i32;
            } else {
                0 as i32;
                0 as i32;
                0 as i32;
                nKey = unsafe {
                    (*unsafe { (*unsafe { (*unsafe { (*pF).pFExpr }).pLeft }).x.pList }).nExpr
                };
                if !((unsafe { (*pF).bOBUnique }) != (0 as u8)) {
                    let __v2914: i32 = nKey;
                    let __v2915: i32 = __v2914 + (1 as i32);
                    nKey = __v2915;
                }
            }
            iTop = unsafe { sqlite3VdbeAddOp1(v, 36 as i32, unsafe { (*pF).iOBTab }) };
            {}
            j = nArg - (1 as i32);
            '__slate_break_2181: loop {
                if !(j >= (0 as i32)) {
                    break;
                }
                unsafe {
                    sqlite3VdbeAddOp3(v, 96 as i32, unsafe { (*pF).iOBTab }, nKey + j, regAgg + j)
                };
                let __v2916: i32 = j;
                let __v2917: i32 = __v2916 - (1 as i32);
                j = __v2917;
            }
            if (unsafe { (*pF).bUseSubtype }) != (0 as u8) {
                let mut regSubtype: i32 = unsafe { sqlite3GetTempReg(pParse) };
                let mut iBaseCol: i32 = nKey
                    + nArg
                    + (((((unsafe { (*pF).bOBPayload }) as u32) as i32) == (0 as i32)
                        && (((unsafe { (*pF).bOBUnique }) as u32) as i32) == (0 as i32))
                        as i32);
                j = nArg - (1 as i32);
                '__slate_break_2182: loop {
                    if !(j >= (0 as i32)) {
                        break;
                    }
                    unsafe {
                        sqlite3VdbeAddOp3(
                            v,
                            96 as i32,
                            unsafe { (*pF).iOBTab },
                            iBaseCol + j,
                            regSubtype,
                        )
                    };
                    unsafe { sqlite3VdbeAddOp2(v, 184 as i32, regSubtype, regAgg + j) };
                    let __v2918: i32 = j;
                    let __v2919: i32 = __v2918 - (1 as i32);
                    j = __v2919;
                }
                unsafe { sqlite3ReleaseTempReg(pParse, regSubtype) };
            }
            unsafe {
                sqlite3VdbeAddOp3(
                    v,
                    164 as i32,
                    0 as i32,
                    regAgg,
                    (unsafe { (*pAggInfo).iFirstReg }) + unsafe { (*pAggInfo).nColumn } + i,
                )
            };
            unsafe { sqlite3VdbeAppendP4(v, (unsafe { (*pF).pFunc }) as *mut (), -(8 as i32)) };
            unsafe { sqlite3VdbeChangeP5(v, (nArg as i16) as u16) };
            unsafe { sqlite3VdbeAddOp2(v, 40 as i32, unsafe { (*pF).iOBTab }, iTop + (1 as i32)) };
            {}
            unsafe { sqlite3VdbeJumpHere(v, iTop) };
            unsafe { sqlite3ReleaseTempRange(pParse, regAgg, nArg) };
        }
        unsafe {
            sqlite3VdbeAddOp2(
                v,
                167 as i32,
                (unsafe { (*pAggInfo).iFirstReg }) + unsafe { (*pAggInfo).nColumn } + i,
                if pList != std::ptr::null_mut::<ExprList>() {
                    unsafe { (*pList).nExpr }
                } else {
                    0 as i32
                },
            )
        };
        unsafe { sqlite3VdbeAppendP4(v, (unsafe { (*pF).pFunc }) as *mut (), -(8 as i32)) };
        let __v2910: i32 = i;
        let __v2911: i32 = __v2910 + (1 as i32);
        i = __v2911;
        let __v2912: *mut AggInfo_func = pF;
        let __v2913: *mut AggInfo_func = unsafe { __v2912.offset((1 as i32) as isize) };
        pF = __v2913;
    }
}

// /*
// ** Generate code that will update the accumulator memory cells for an
// ** aggregate based on the current cursor position.
// **
// ** If regAcc is non-zero and there are no min() or max() aggregates
// ** in pAggInfo, then only populate the pAggInfo->nAccumulator accumulator
// ** registers if register regAcc contains 0. The caller will take care
// ** of setting and clearing regAcc.
// **
// ** For an ORDER BY aggregate, the actual accumulator memory cell update
// ** is deferred until after all input rows have been received, so that they
// ** can be run in the requested order.  In that case, instead of invoking
// ** OP_AggStep to update the accumulator, just add the arguments that would
// ** have been passed into OP_AggStep into the sorting ephemeral table
// ** (along with the appropriate sort key).
// */
fn updateAccumulator(
    mut pParse: *mut Parse,
    mut regAcc: i32,
    mut pAggInfo: *mut AggInfo,
    mut eDistinctType: i32,
) {
    let mut v: *mut Vdbe = unsafe { (*pParse).pVdbe };
    let mut i: i32 = 0 as i32;
    let mut regHit: i32 = 0 as i32;
    let mut addrHitTest: i32 = 0 as i32;
    let mut pF: *mut AggInfo_func = unsafe { std::mem::zeroed() };
    let mut pC: *mut AggInfo_col = unsafe { std::mem::zeroed() };
    0 as i32;
    if (unsafe { (*pParse).nErr }) != (0 as i32) {
        return;
    }
    unsafe {
        (*pAggInfo).directMode = ((1 as i32) as i8) as u8;
    }
    i = 0 as i32;
    let __v2920: *mut AggInfo_func = unsafe { (*pAggInfo).aFunc };
    pF = __v2920;
    '__slate_break_2183: while i < unsafe { (*pAggInfo).nFunc } {
        let mut nArg: i32 = 0 as i32;
        let mut addrNext: i32 = 0 as i32;
        let mut regAgg: i32 = 0 as i32;
        let mut regAggSz: i32 = 0 as i32;
        let mut regDistinct: i32 = 0 as i32;
        let mut pList: *mut ExprList = unsafe { std::mem::zeroed() };
        0 as i32;
        0 as i32;
        0 as i32;
        pList = unsafe { (*unsafe { (*pF).pFExpr }).x.pList };
        if (unsafe { (*unsafe { (*pF).pFExpr }).flags }) & ((16777216 as i32) as u32)
            != ((0 as i32) as u32)
        {
            let mut pFilter: *mut Expr =
                unsafe { (*unsafe { (*unsafe { (*pF).pFExpr }).y.pWin }).pFilter };
            if (unsafe { (*pAggInfo).nAccumulator }) != (0 as i32)
                && (unsafe { (*unsafe { (*pF).pFunc }).funcFlags }) & ((32 as i32) as u32)
                    != (0 as u32)
                && regAcc != (0 as i32)
            {
                // /* If regAcc==0, there there exists some min() or max() function
                //         ** without a FILTER clause that will ensure the magnet registers
                //         ** are populated. */
                if regHit == (0 as i32) {
                    let __v2925: *mut Parse = pParse;
                    let __v2926: i32 = unsafe { (*__v2925).nMem };
                    let __v2927: i32 = __v2926 + (1 as i32);
                    unsafe {
                        (*__v2925).nMem = __v2927;
                    }
                    regHit = __v2927;
                }
                // /* If this is the first row of the group (regAcc contains 0), clear the
                //         ** "magnet" register regHit so that the accumulator registers
                //         ** are populated if the FILTER clause jumps over the the
                //         ** invocation of min() or max() altogether. Or, if this is not
                //         ** the first row (regAcc contains 1), set the magnet register so that
                //         ** the accumulators are not populated unless the min()/max() is invoked
                //         ** and indicates that they should be.  */
                unsafe { sqlite3VdbeAddOp2(v, 82 as i32, regAcc, regHit) };
            }
            addrNext = unsafe { sqlite3VdbeMakeLabel(pParse) };
            unsafe { sqlite3ExprIfFalse(pParse, pFilter, addrNext, 16 as i32) };
        }
        if (unsafe { (*pF).iOBTab }) >= (0 as i32) {
            // /* Instead of invoking AggStep, we must push the arguments that would
            //       ** have been passed to AggStep onto the sorting table. */
            // /* Registered used so far in building the record */
            let mut jj: i32 = 0 as i32;
            // /* The ORDER BY clause */
            let mut pOBList: *mut ExprList = unsafe { std::mem::zeroed() };
            0 as i32;
            nArg = unsafe { (*pList).nExpr };
            0 as i32;
            0 as i32;
            0 as i32;
            0 as i32;
            pOBList = unsafe { (*unsafe { (*unsafe { (*pF).pFExpr }).pLeft }).x.pList };
            0 as i32;
            0 as i32;
            regAggSz = unsafe { (*pOBList).nExpr };
            if !((unsafe { (*pF).bOBUnique }) != (0 as u8)) {
                // /* One register for OP_Sequence */
                let __v2928: i32 = regAggSz;
                let __v2929: i32 = __v2928 + (1 as i32);
                regAggSz = __v2929;
            }
            if (unsafe { (*pF).bOBPayload }) != (0 as u8) {
                let __v2930: i32 = regAggSz;
                let __v2931: i32 = __v2930 + nArg;
                regAggSz = __v2931;
            }
            if (unsafe { (*pF).bUseSubtype }) != (0 as u8) {
                let __v2932: i32 = regAggSz;
                let __v2933: i32 = __v2932 + nArg;
                regAggSz = __v2933;
            }
            // /* One extra register to hold result of MakeRecord */
            let __v2934: i32 = regAggSz;
            let __v2935: i32 = __v2934 + (1 as i32);
            regAggSz = __v2935;
            regAgg = unsafe { sqlite3GetTempRange(pParse, regAggSz) };
            regDistinct = regAgg;
            unsafe {
                sqlite3ExprCodeExprList(pParse, pOBList, regAgg, 0 as i32, ((1 as i32) as i8) as u8)
            };
            jj = unsafe { (*pOBList).nExpr };
            if !((unsafe { (*pF).bOBUnique }) != (0 as u8)) {
                unsafe { sqlite3VdbeAddOp2(v, 128 as i32, unsafe { (*pF).iOBTab }, regAgg + jj) };
                let __v2936: i32 = jj;
                let __v2937: i32 = __v2936 + (1 as i32);
                jj = __v2937;
            }
            if (unsafe { (*pF).bOBPayload }) != (0 as u8) {
                regDistinct = regAgg + jj;
                unsafe {
                    sqlite3ExprCodeExprList(
                        pParse,
                        pList,
                        regDistinct,
                        0 as i32,
                        ((1 as i32) as i8) as u8,
                    )
                };
                let __v2938: i32 = jj;
                let __v2939: i32 = __v2938 + nArg;
                jj = __v2939;
            }
            if (unsafe { (*pF).bUseSubtype }) != (0 as u8) {
                let mut kk: i32 = 0 as i32;
                let mut regBase: i32 = if (unsafe { (*pF).bOBPayload }) != (0 as u8) {
                    regDistinct
                } else {
                    regAgg
                };
                kk = 0 as i32;
                '__slate_break_2184: loop {
                    if !(kk < nArg) {
                        break;
                    }
                    unsafe { sqlite3VdbeAddOp2(v, 183 as i32, regBase + kk, regAgg + jj) };
                    let __v2940: i32 = kk;
                    let __v2941: i32 = __v2940 + (1 as i32);
                    kk = __v2941;
                    let __v2942: i32 = jj;
                    let __v2943: i32 = __v2942 + (1 as i32);
                    jj = __v2943;
                }
            }
        } else {
            if pList != std::ptr::null_mut::<ExprList>() {
                nArg = unsafe { (*pList).nExpr };
                regAgg = unsafe { sqlite3GetTempRange(pParse, nArg) };
                regDistinct = regAgg;
                unsafe {
                    sqlite3ExprCodeExprList(
                        pParse,
                        pList,
                        regAgg,
                        0 as i32,
                        ((1 as i32) as i8) as u8,
                    )
                };
            } else {
                nArg = 0 as i32;
                regAgg = 0 as i32;
            }
        }
        if (unsafe { (*pF).iDistinct }) >= (0 as i32) && pList != std::ptr::null_mut::<ExprList>() {
            if addrNext == (0 as i32) {
                addrNext = unsafe { sqlite3VdbeMakeLabel(pParse) };
            }
            unsafe {
                (*pF).iDistinct = codeDistinct(
                    pParse,
                    eDistinctType,
                    unsafe { (*pF).iDistinct },
                    addrNext,
                    pList,
                    regDistinct,
                );
            }
        }
        if (unsafe { (*pF).iOBTab }) >= (0 as i32) {
            // /* Insert a new record into the ORDER BY table */
            unsafe {
                sqlite3VdbeAddOp3(
                    v,
                    99 as i32,
                    regAgg,
                    regAggSz - (1 as i32),
                    regAgg + regAggSz - (1 as i32),
                )
            };
            unsafe {
                sqlite3VdbeAddOp4Int(
                    v,
                    140 as i32,
                    unsafe { (*pF).iOBTab },
                    regAgg + regAggSz - (1 as i32),
                    regAgg,
                    regAggSz - (1 as i32),
                )
            };
            unsafe { sqlite3ReleaseTempRange(pParse, regAgg, regAggSz) };
        } else {
            // /* Invoke the AggStep function */
            if (unsafe { (*unsafe { (*pF).pFunc }).funcFlags }) & ((32 as i32) as u32) != (0 as u32)
            {
                let mut pColl: *mut CollSeq = std::ptr::null_mut::<CollSeq>();
                let mut pItem: *mut ExprList_item = unsafe { std::mem::zeroed() };
                let mut j: i32 = 0 as i32;
                // /* pList!=0 if pF->pFunc has NEEDCOLL */
                0 as i32;
                j = 0 as i32;
                let __v2944: *mut ExprList_item =
                    unsafe { std::ptr::addr_of_mut!((*pList).a) as *mut ExprList_item };
                pItem = __v2944;
                '__slate_break_2185: while !(pColl != std::ptr::null_mut::<CollSeq>()) && j < nArg {
                    pColl = unsafe {
                        sqlite3ExprCollSeq(pParse, (unsafe { (*pItem).pExpr }) as *const Expr)
                    };
                    let __v2945: i32 = j;
                    let __v2946: i32 = __v2945 + (1 as i32);
                    j = __v2946;
                    let __v2947: *mut ExprList_item = pItem;
                    let __v2948: *mut ExprList_item =
                        unsafe { __v2947.offset((1 as i32) as isize) };
                    pItem = __v2948;
                }
                if !(pColl != std::ptr::null_mut::<CollSeq>()) {
                    pColl = unsafe { (*unsafe { (*pParse).db }).pDfltColl };
                }
                if regHit == (0 as i32) && (unsafe { (*pAggInfo).nAccumulator }) != (0 as i32) {
                    let __v2949: *mut Parse = pParse;
                    let __v2950: i32 = unsafe { (*__v2949).nMem };
                    let __v2951: i32 = __v2950 + (1 as i32);
                    unsafe {
                        (*__v2949).nMem = __v2951;
                    }
                    regHit = __v2951;
                }
                unsafe {
                    sqlite3VdbeAddOp4(
                        v,
                        87 as i32,
                        regHit,
                        0 as i32,
                        0 as i32,
                        (pColl as *mut i8) as *const i8,
                        -(2 as i32),
                    )
                };
            }
            unsafe {
                sqlite3VdbeAddOp3(
                    v,
                    164 as i32,
                    0 as i32,
                    regAgg,
                    (unsafe { (*pAggInfo).iFirstReg }) + unsafe { (*pAggInfo).nColumn } + i,
                )
            };
            unsafe { sqlite3VdbeAppendP4(v, (unsafe { (*pF).pFunc }) as *mut (), -(8 as i32)) };
            unsafe { sqlite3VdbeChangeP5(v, (nArg as i16) as u16) };
            unsafe { sqlite3ReleaseTempRange(pParse, regAgg, nArg) };
        }
        if addrNext != (0 as i32) {
            unsafe { sqlite3VdbeResolveLabel(v, addrNext) };
        }
        if (unsafe { (*pParse).nErr }) != (0 as i32) {
            return;
        }
        let __v2921: i32 = i;
        let __v2922: i32 = __v2921 + (1 as i32);
        i = __v2922;
        let __v2923: *mut AggInfo_func = pF;
        let __v2924: *mut AggInfo_func = unsafe { __v2923.offset((1 as i32) as isize) };
        pF = __v2924;
    }
    if regHit == (0 as i32) && (unsafe { (*pAggInfo).nAccumulator }) != (0 as i32) {
        regHit = regAcc;
    }
    if regHit != (0 as i32) {
        addrHitTest = unsafe { sqlite3VdbeAddOp1(v, 16 as i32, regHit) };
        {}
    }
    i = 0 as i32;
    let __v2952: *mut AggInfo_col = unsafe { (*pAggInfo).aCol };
    pC = __v2952;
    '__slate_break_2186: while i < unsafe { (*pAggInfo).nAccumulator } {
        unsafe {
            sqlite3ExprCode(
                pParse,
                unsafe { (*pC).pCExpr },
                (unsafe { (*pAggInfo).iFirstReg }) + i,
            )
        };
        if (unsafe { (*pParse).nErr }) != (0 as i32) {
            return;
        }
        let __v2953: i32 = i;
        let __v2954: i32 = __v2953 + (1 as i32);
        i = __v2954;
        let __v2955: *mut AggInfo_col = pC;
        let __v2956: *mut AggInfo_col = unsafe { __v2955.offset((1 as i32) as isize) };
        pC = __v2956;
    }
    unsafe {
        (*pAggInfo).directMode = ((0 as i32) as i8) as u8;
    }
    if addrHitTest != (0 as i32) {
        unsafe { sqlite3VdbeJumpHereOrPopInst(v, addrHitTest) };
    }
}

// /*
// ** Add a single OP_Explain instruction to the VDBE to explain a simple
// ** count(*) query ("SELECT count(*) FROM pTab").
// */
fn explainSimpleCount(mut pParse: *mut Parse, mut pTab: *mut Table, mut pIdx: *mut Index) {
    if (((unsafe { (*pParse).explain }) as u32) as i32) == (2 as i32) {
        let mut bCover: i32 = (pIdx != std::ptr::null_mut::<Index>()
            && ((unsafe { (*pTab).tabFlags }) & ((128 as i32) as u32) == ((0 as i32) as u32)
                || !(((unsafe { (*pIdx).__slate_bits_0.__get_idxType() }) as i32) == (2 as i32))))
            as i32;
        unsafe {
            sqlite3VdbeExplain(
                pParse,
                ((0 as i32) as i8) as u8,
                (b"SCAN %s%s%s\0".as_ptr() as *mut i8) as *const i8,
                unsafe { (*pTab).zName },
                if bCover != (0 as i32) {
                    b" USING COVERING INDEX \0".as_ptr() as *mut i8
                } else {
                    b"\0".as_ptr() as *mut i8
                },
                if bCover != (0 as i32) {
                    unsafe { (*pIdx).zName }
                } else {
                    b"\0".as_ptr() as *mut i8
                },
            )
        };
    }
}

// /* Parse context */
// /* Table being queried */
// /* Index used to optimize scan, or NULL */
// /*
// ** sqlite3WalkExpr() callback used by havingToWhere().
// **
// ** If the node passed to the callback is a TK_AND node, return
// ** WRC_Continue to tell sqlite3WalkExpr() to iterate through child nodes.
// **
// ** Otherwise, return WRC_Prune. In this case, also check if the
// ** sub-expression matches the criteria for being moved to the WHERE
// ** clause. If so, add it to the WHERE clause and replace the sub-expression
// ** within the HAVING expression with a constant "1".
// */
#[unsafe(link_section = ".text.slate_distinct.select.havingToWhereExprCb")]
extern "C-unwind" fn havingToWhereExprCb(mut pWalker: *mut Walker, mut pExpr: *mut Expr) -> i32 {
    if (((unsafe { (*pExpr).op }) as u32) as i32) != (44 as i32) {
        let mut pS: *mut Select = unsafe { (*pWalker).u.pSelect };
        // /* This routine is called before the HAVING clause of the current
        //     ** SELECT is analyzed for aggregates. So if pExpr->pAggInfo is set
        //     ** here, it indicates that the expression is a correlated reference to a
        //     ** column from an outer aggregate query, or an aggregate function that
        //     ** belongs to an outer query. Do not move the expression to the WHERE
        //     ** clause in this obscure case, as doing so may corrupt the outer Select
        //     ** statements AggInfo structure.  */
        if (unsafe {
            sqlite3ExprIsConstantOrGroupBy(unsafe { (*pWalker).pParse }, pExpr, unsafe {
                (*pS).pGroupBy
            })
        }) != (0 as i32)
            && (((unsafe { (*pExpr).flags }) & (((1 as i32) | (536870912 as i32)) as u32)
                == ((536870912 as i32) as u32)) as i32)
                == (0 as i32)
            && (unsafe { (*pExpr).pAggInfo }) == std::ptr::null_mut::<AggInfo>()
        {
            let mut db: *mut sqlite3 = unsafe { (*unsafe { (*pWalker).pParse }).db };
            let mut pNew: *mut Expr = unsafe { sqlite3ExprInt32(db, 1 as i32) };
            if pNew != std::ptr::null_mut::<Expr>() {
                let mut pWhere: *mut Expr = unsafe { (*pS).pWhere };
                let mut t: Expr = unsafe { *pNew };
                unsafe {
                    *pNew = unsafe { *pExpr };
                }
                unsafe {
                    *pExpr = t;
                }
                {}
                pNew = unsafe { sqlite3ExprAnd(unsafe { (*pWalker).pParse }, pWhere, pNew) };
                unsafe {
                    (*pS).pWhere = pNew;
                }
                unsafe {
                    (*pWalker).eCode = ((1 as i32) as i16) as u16;
                }
            }
        }
        return 1 as i32;
    }
    return 0 as i32;
}

// /*
// ** Transfer eligible terms from the HAVING clause of a query, which is
// ** processed after grouping, to the WHERE clause, which is processed before
// ** grouping. For example, the query:
// **
// **   SELECT * FROM <tables> WHERE a=? GROUP BY b HAVING b=? AND c=?
// **
// ** can be rewritten as:
// **
// **   SELECT * FROM <tables> WHERE a=? AND b=? GROUP BY b HAVING c=?
// **
// ** A term of the HAVING expression is eligible for transfer if it consists
// ** entirely of constants and expressions that are also GROUP BY terms that
// ** use the "BINARY" collation sequence.
// */
fn havingToWhere(mut pParse: *mut Parse, mut p: *mut Select) {
    let mut sWalker: Walker = unsafe { std::mem::zeroed() };
    unsafe {
        memset(
            std::ptr::addr_of_mut!(sWalker) as *mut (),
            0 as i32,
            48 as u64,
        )
    };
    sWalker.pParse = pParse;
    sWalker.xExprCallback = Some(havingToWhereExprCb);
    unsafe {
        sWalker.u.pSelect = p;
    }
    unsafe { sqlite3WalkExpr(std::ptr::addr_of_mut!(sWalker), unsafe { (*p).pHaving }) };
}

// /*
// ** Check to see if the pThis entry of pTabList is a self-join of another view.
// ** Search FROM-clause entries in the range of iFirst..iEnd, including iFirst
// ** but stopping before iEnd.
// **
// ** If pThis is a self-join, then return the SrcItem for the first other
// ** instance of that view found.  If pThis is not a self-join then return 0.
// */
fn isSelfJoinView(
    mut pTabList: *mut SrcList,
    mut pThis: *mut SrcItem,
    mut iFirst: i32,
    mut iEnd: i32,
) -> *mut SrcItem {
    let mut pItem: *mut SrcItem = unsafe { std::mem::zeroed() };
    let mut pSel: *mut Select = unsafe { std::mem::zeroed() };
    0 as i32;
    pSel = unsafe { (*unsafe { (*pThis).u4.pSubq }).pSelect };
    0 as i32;
    if (unsafe { (*pSel).selFlags }) & ((16777216 as i32) as u32) != (0 as u32) {
        return std::ptr::null_mut::<SrcItem>();
    }
    '__slate_break_2191: while iFirst < iEnd {
        let mut pS1: *mut Select = unsafe { std::mem::zeroed() };
        let __v2957: i32 = iFirst;
        let __v2958: i32 = __v2957 + (1 as i32);
        iFirst = __v2958;
        pItem = unsafe {
            unsafe { std::ptr::addr_of_mut!((*pTabList).a) as *mut SrcItem }
                .offset(__v2957 as isize)
        };
        if !(((unsafe { (*pItem).fg.__slate_bits_0.__get_isSubquery() }) as i32) != (0 as i32)) {
        } else {
            if ((unsafe { (*pItem).fg.__slate_bits_0.__get_viaCoroutine() }) as i32) != (0 as i32) {
            } else {
                if (unsafe { (*pItem).zName }) == std::ptr::null_mut::<i8>() {
                } else {
                    0 as i32;
                    0 as i32;
                    if (unsafe { (*unsafe { (*pItem).pSTab }).pSchema })
                        != unsafe { (*unsafe { (*pThis).pSTab }).pSchema }
                    {
                    } else {
                        if (unsafe {
                            sqlite3_stricmp(
                                (unsafe { (*pItem).zName }) as *const i8,
                                (unsafe { (*pThis).zName }) as *const i8,
                            )
                        }) != (0 as i32)
                        {
                        } else {
                            pS1 = unsafe { (*unsafe { (*pItem).u4.pSubq }).pSelect };
                            if (unsafe { (*unsafe { (*pItem).pSTab }).pSchema })
                                == std::ptr::null_mut::<Schema>()
                                && (unsafe { (*pSel).selId }) != unsafe { (*pS1).selId }
                            {
                                // /* The query flattener left two different CTE tables with identical
                                //       ** names in the same FROM clause. */
                            } else {
                                if (unsafe { (*pS1).selFlags }) & ((16777216 as i32) as u32)
                                    != (0 as u32)
                                {
                                    // /* The view was modified by some other optimization such as
                                    //       ** pushDownWhereTerms() */
                                } else {
                                    return pItem;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    return std::ptr::null_mut::<SrcItem>();
}

// /* Search for self-joins in this FROM clause */
// /* Search for prior reference to this subquery */
// /* Range of FROM-clause entries to search. */
// /*
// ** Deallocate a single AggInfo object
// */
#[unsafe(link_section = ".text.slate_distinct.select.agginfoFree")]
extern "C-unwind" fn agginfoFree(mut db: *mut sqlite3, mut pArg: *mut ()) {
    let mut p: *mut AggInfo = pArg as *mut AggInfo;
    unsafe { sqlite3DbFree(db, (unsafe { (*p).aCol }) as *mut ()) };
    unsafe { sqlite3DbFree(db, (unsafe { (*p).aFunc }) as *mut ()) };
    unsafe { sqlite3DbFreeNN(db, p as *mut ()) };
}

// /*
// ** Attempt to transform a query of the form
// **
// **    SELECT count(*) FROM (SELECT x FROM t1 UNION ALL SELECT y FROM t2)
// **
// ** Into this:
// **
// **    SELECT (SELECT count(*) FROM t1)+(SELECT count(*) FROM t2)
// **
// ** The transformation only works if all of the following are true:
// **
// **   *  The subquery is a UNION ALL of two or more terms
// **   *  The subquery does not have a LIMIT clause
// **   *  There is no WHERE or GROUP BY or HAVING clauses on the subqueries
// **   *  The outer query is a simple count(*) with no WHERE clause or other
// **      extraneous syntax.
// **   *  None of the subqueries are DISTINCT (forumpost/a860f5fb2e 2025-03-10)
// **
// ** Return TRUE if the optimization is undertaken.
// */
fn countOfViewOptimization(mut pParse: *mut Parse, mut p: *mut Select) -> i32 {
    let mut pSub: *mut Select = unsafe { std::mem::zeroed() };
    let mut pPrior: *mut Select = unsafe { std::mem::zeroed() };
    let mut pExpr: *mut Expr = unsafe { std::mem::zeroed() };
    let mut pCount: *mut Expr = unsafe { std::mem::zeroed() };
    let mut db: *mut sqlite3 = unsafe { std::mem::zeroed() };
    let mut pFrom: *mut SrcItem = unsafe { std::mem::zeroed() };
    // /* This is an aggregate */
    if (unsafe { (*p).selFlags }) & ((8 as i32) as u32) == ((0 as i32) as u32) {
        return 0 as i32;
    }
    // /* Single result column */
    if (unsafe { (*unsafe { (*p).pEList }).nExpr }) != (1 as i32) {
        return 0 as i32;
    }
    if (unsafe { (*p).pWhere }) != std::ptr::null_mut::<Expr>() {
        return 0 as i32;
    }
    if (unsafe { (*p).pHaving }) != std::ptr::null_mut::<Expr>() {
        return 0 as i32;
    }
    if (unsafe { (*p).pGroupBy }) != std::ptr::null_mut::<ExprList>() {
        return 0 as i32;
    }
    if (unsafe { (*p).pOrderBy }) != std::ptr::null_mut::<ExprList>() {
        return 0 as i32;
    }
    pExpr = unsafe {
        (*unsafe {
            unsafe { std::ptr::addr_of_mut!((*unsafe { (*p).pEList }).a) as *mut ExprList_item }
                .offset((0 as i32) as isize)
        })
        .pExpr
    };
    // /* Result is an aggregate */
    if (((unsafe { (*pExpr).op }) as u32) as i32) != (169 as i32) {
        return 0 as i32;
    }
    0 as i32;
    // /* Is count() */
    if (unsafe {
        sqlite3_stricmp(
            (unsafe { (*pExpr).u.zToken }) as *const i8,
            (b"count\0".as_ptr() as *mut i8) as *const i8,
        )
    }) != (0 as i32)
    {
        return 0 as i32;
    }
    0 as i32;
    // /* Must be count(*) */
    if (unsafe { (*pExpr).x.pList }) != std::ptr::null_mut::<ExprList>() {
        return 0 as i32;
    }
    // /* One table in FROM  */
    if (unsafe { (*unsafe { (*p).pSrc }).nSrc }) != (1 as i32) {
        return 0 as i32;
    }
    // /* Not a window function */
    if (unsafe { (*pExpr).flags }) & ((16777216 as i32) as u32) != ((0 as i32) as u32) {
        return 0 as i32;
    }
    pFrom = unsafe { std::ptr::addr_of_mut!((*unsafe { (*p).pSrc }).a) as *mut SrcItem };
    // /* FROM is a subquery */
    if ((unsafe { (*pFrom).fg.__slate_bits_0.__get_isSubquery() }) as i32) == (0 as i32) {
        return 0 as i32;
    }
    // /* Not a correlated subq */
    if (unsafe { (*p).selFlags }) & ((536870912 as i32) as u32) != ((0 as i32) as u32) {
        return 0 as i32;
    }
    pSub = unsafe { (*unsafe { (*pFrom).u4.pSubq }).pSelect };
    // /* Must be a compound */
    if (unsafe { (*pSub).pPrior }) == std::ptr::null_mut::<Select>() {
        return 0 as i32;
    }
    // /* Not a CTE */
    if (unsafe { (*pSub).selFlags }) & ((67108864 as i32) as u32) != (0 as u32) {
        return 0 as i32;
    }
    '__slate_break_2193: loop {
        // /* Must be UNION ALL */
        if (((unsafe { (*pSub).op }) as u32) as i32) != (136 as i32)
            && (unsafe { (*pSub).pPrior }) != std::ptr::null_mut::<Select>()
        {
            return 0 as i32;
        }
        // /* No WHERE clause */
        if (unsafe { (*pSub).pWhere }) != std::ptr::null_mut::<Expr>() {
            return 0 as i32;
        }
        // /* No LIMIT clause */
        if (unsafe { (*pSub).pLimit }) != std::ptr::null_mut::<Expr>() {
            return 0 as i32;
        }
        if (unsafe { (*pSub).selFlags }) & (((8 as i32) | (1 as i32)) as u32) != (0 as u32) {
            {}
            {}
            // /* Not an aggregate nor DISTINCT */
            return 0 as i32;
        }
        // /* Due to the previous */
        0 as i32;
        // /* Repeat over compound */
        pSub = unsafe { (*pSub).pPrior };
        if !(pSub != std::ptr::null_mut::<Select>()) {
            break;
        }
    }
    // /* If we reach this point then it is OK to perform the transformation */
    db = unsafe { (*pParse).db };
    pCount = pExpr;
    pExpr = std::ptr::null_mut::<Expr>();
    pSub = unsafe { sqlite3SubqueryDetach(db, pFrom) };
    unsafe { sqlite3SrcListDelete(db, unsafe { (*p).pSrc }) };
    unsafe {
        (*p).pSrc = (unsafe {
            sqlite3DbMallocZero(unsafe { (*pParse).db }, (8 as u64).wrapping_add(72 as u64))
        }) as *mut SrcList;
    }
    '__slate_break_2194: while pSub != std::ptr::null_mut::<Select>() {
        let mut pTerm: *mut Expr = unsafe { std::mem::zeroed() };
        pPrior = unsafe { (*pSub).pPrior };
        unsafe {
            (*pSub).pPrior = std::ptr::null_mut::<Select>();
        }
        unsafe {
            (*pSub).pNext = std::ptr::null_mut::<Select>();
        }
        let __v2959: *mut Select = pSub;
        let __v2960: u32 = unsafe { (*__v2959).selFlags };
        let __v2961: u32 = __v2960 | ((8 as i32) as u32);
        unsafe {
            (*__v2959).selFlags = __v2961;
        }
        let __v2962: *mut Select = pSub;
        let __v2963: u32 = unsafe { (*__v2962).selFlags };
        let __v2964: u32 = __v2963 & !((256 as i32) as u32);
        unsafe {
            (*__v2962).selFlags = __v2964;
        }
        unsafe {
            (*pSub).nSelectRow = (0 as i32) as i16;
        }
        unsafe {
            sqlite3ParserAddCleanup(
                pParse,
                unsafe {
                    std::mem::transmute::<
                        *const (),
                        Option<unsafe extern "C-unwind" fn(*mut sqlite3, *mut ())>,
                    >(sqlite3ExprListDeleteGeneric as *const ())
                },
                (unsafe { (*pSub).pEList }) as *mut (),
            )
        };
        let __v2965: *mut Expr;
        if pPrior != std::ptr::null_mut::<Select>() {
            __v2965 = unsafe { sqlite3ExprDup(db, pCount as *const Expr, 0 as i32) };
        } else {
            __v2965 = pCount;
        }
        pTerm = __v2965;
        unsafe {
            (*pSub).pEList =
                unsafe { sqlite3ExprListAppend(pParse, std::ptr::null_mut::<ExprList>(), pTerm) };
        }
        pTerm = unsafe {
            sqlite3PExpr(
                pParse,
                139 as i32,
                std::ptr::null_mut::<Expr>(),
                std::ptr::null_mut::<Expr>(),
            )
        };
        unsafe { sqlite3PExprAddSelect(pParse, pTerm, pSub) };
        if pExpr == std::ptr::null_mut::<Expr>() {
            pExpr = pTerm;
        } else {
            pExpr = unsafe { sqlite3PExpr(pParse, 107 as i32, pTerm, pExpr) };
        }
        pSub = pPrior;
    }
    unsafe {
        (*unsafe {
            unsafe { std::ptr::addr_of_mut!((*unsafe { (*p).pEList }).a) as *mut ExprList_item }
                .offset((0 as i32) as isize)
        })
        .pExpr = pExpr;
    }
    let __v2966: *mut Select = p;
    let __v2967: u32 = unsafe { (*__v2966).selFlags };
    let __v2968: u32 = __v2967 & !((8 as i32) as u32);
    unsafe {
        (*__v2966).selFlags = __v2968;
    }
    return 1 as i32;
}

// /*
// ** If any term of pSrc, or any SF_NestedFrom sub-query, is not the same
// ** as pSrcItem but has the same alias as p0, then return true.
// ** Otherwise return false.
// */
fn sameSrcAlias(mut p0: *mut SrcItem, mut pSrc: *mut SrcList) -> i32 {
    let mut i: i32 = 0 as i32;
    i = 0 as i32;
    '__slate_break_2195: loop {
        if !(i < unsafe { (*pSrc).nSrc }) {
            break;
        }
        let mut p1: *mut SrcItem = unsafe {
            unsafe { std::ptr::addr_of_mut!((*pSrc).a) as *mut SrcItem }.offset(i as isize)
        };
        if p1 == p0 {
        } else {
            let __v2971: bool;
            if (unsafe { (*p0).pSTab }) == unsafe { (*p1).pSTab } {
                __v2971 = (0 as i32)
                    == unsafe {
                        sqlite3_stricmp(
                            (unsafe { (*p0).zAlias }) as *const i8,
                            (unsafe { (*p1).zAlias }) as *const i8,
                        )
                    };
            } else {
                __v2971 = false as bool;
            }
            if __v2971 {
                return 1 as i32;
            }
            let __v2972: bool;
            if ((unsafe { (*p1).fg.__slate_bits_0.__get_isSubquery() }) as i32) != (0 as i32)
                && (unsafe { (*unsafe { (*unsafe { (*p1).u4.pSubq }).pSelect }).selFlags })
                    & ((2048 as i32) as u32)
                    != ((0 as i32) as u32)
            {
                __v2972 = sameSrcAlias(p0, unsafe {
                    (*unsafe { (*unsafe { (*p1).u4.pSubq }).pSelect }).pSrc
                }) != (0 as i32);
            } else {
                __v2972 = false as bool;
            }
            if __v2972 {
                return 1 as i32;
            }
        }
        let __v2969: i32 = i;
        let __v2970: i32 = __v2969 + (1 as i32);
        i = __v2970;
    }
    return 0 as i32;
}

// /*
// ** Return TRUE (non-zero) if the i-th entry in the pTabList SrcList can
// ** be implemented as a co-routine.  The i-th entry is guaranteed to be
// ** a subquery.
// **
// ** The subquery is implemented as a co-routine if all of the following are
// ** true:
// **
// **    (1)  The subquery will likely be implemented in the outer loop of
// **         the query.  This will be the case if any one of the following
// **         conditions hold:
// **         (a)  The subquery is the only term in the FROM clause
// **         (b)  The subquery is the left-most term and a CROSS JOIN or similar
// **              requires it to be the outer loop
// **         (c)  All of the following are true:
// **                (i) The subquery is the left-most subquery in the FROM clause
// **               (ii) There is nothing that would prevent the subquery from
// **                    being used as the outer loop if the sqlite3WhereBegin()
// **                    routine nominates it to that position.
// **              (iii) The query is not a UPDATE ... FROM
// **    (2)  The subquery is not a CTE that should be materialized because
// **         (a) the AS MATERIALIZED keyword is used, or
// **         (b) the CTE is used multiple times and does not have the
// **             NOT MATERIALIZED keyword
// **    (3)  The subquery is not part of a left operand for a RIGHT JOIN
// **    (4)  The SQLITE_Coroutine optimization disable flag is not set
// **    (5)  The subquery is not self-joined
// */
fn fromClauseTermCanBeCoroutine(
    mut pParse: *mut Parse,
    mut pTabList: *mut SrcList,
    mut i: i32,
    mut selFlags: i32,
) -> i32 {
    let mut pItem: *mut SrcItem = unsafe {
        unsafe { std::ptr::addr_of_mut!((*pTabList).a) as *mut SrcItem }.offset(i as isize)
    };
    if ((unsafe { (*pItem).fg.__slate_bits_0.__get_isCte() }) as i32) != (0 as i32) {
        let mut pCteUse: *const CteUse = (unsafe { (*pItem).u2.pCteUse }) as *const CteUse;
        // /* (2a) */
        if (((unsafe { (*pCteUse).eM10d }) as u32) as i32) == (0 as i32) {
            return 0 as i32;
        }
        // /* (2b) */
        if (unsafe { (*pCteUse).nUse }) >= (2 as i32)
            && (((unsafe { (*pCteUse).eM10d }) as u32) as i32) != (2 as i32)
        {
            return 0 as i32;
        }
    }
    // /* (3)  */
    if (((unsafe {
        (*unsafe {
            unsafe { std::ptr::addr_of_mut!((*pTabList).a) as *mut SrcItem }
                .offset((0 as i32) as isize)
        })
        .fg
        .jointype
    }) as u32) as i32)
        & (64 as i32)
        != (0 as i32)
    {
        return 0 as i32;
    }
    // /* (4)  */
    if (unsafe { (*unsafe { (*pParse).db }).dbOptFlags }) & ((33554432 as i32) as u32)
        != ((0 as i32) as u32)
    {
        return 0 as i32;
    }
    if isSelfJoinView(pTabList, pItem, i + (1 as i32), unsafe { (*pTabList).nSrc })
        != std::ptr::null_mut::<SrcItem>()
    {
        // /* (5) */
        return 0 as i32;
    }
    if i == (0 as i32) {
        // /* (1a) */
        if (unsafe { (*pTabList).nSrc }) == (1 as i32) {
            return 1 as i32;
        }
        // /* (1b) */
        if (((unsafe {
            (*unsafe {
                unsafe { std::ptr::addr_of_mut!((*pTabList).a) as *mut SrcItem }
                    .offset((1 as i32) as isize)
            })
            .fg
            .jointype
        }) as u32) as i32)
            & (2 as i32)
            != (0 as i32)
        {
            return 1 as i32;
        }
        // /* (1c-iii) */
        if selFlags & (268435456 as i32) != (0 as i32) {
            return 0 as i32;
        }
        return 1 as i32;
    }
    // /* (1c-iii) */
    if selFlags & (268435456 as i32) != (0 as i32) {
        return 0 as i32;
    }
    // /*exit-by-break*/
    '__slate_break_2196: while (1 as i32) != (0 as i32) {
        // /* (1c-ii) */
        if (((unsafe { (*pItem).fg.jointype }) as u32) as i32) & ((32 as i32) | (2 as i32))
            != (0 as i32)
        {
            return 0 as i32;
        }
        if i == (0 as i32) {
            break '__slate_break_2196;
        }
        let __v2973: i32 = i;
        let __v2974: i32 = __v2973 - (1 as i32);
        i = __v2974;
        let __v2975: *mut SrcItem = pItem;
        let __v2976: *mut SrcItem = unsafe { __v2975.offset(-((1 as i32) as isize)) };
        pItem = __v2976;
        // /* (1c-i) */
        if ((unsafe { (*pItem).fg.__slate_bits_0.__get_isSubquery() }) as i32) != (0 as i32) {
            return 0 as i32;
        }
    }
    return 1 as i32;
}

// /* Parsing context */
// /* FROM clause */
// /* Which term of the FROM clause holds the subquery */
// /* Flags on the SELECT statement */
// /*
// ** Argument pWhere is the WHERE clause belonging to SELECT statement p. This
// ** function attempts to transform expressions of the form:
// **
// **     EXISTS (SELECT ...)
// **
// ** into joins. For example, given
// **
// **    CREATE TABLE sailors(sid INTEGER PRIMARY KEY, name TEXT);
// **    CREATE TABLE reserves(sid INT, day DATE, PRIMARY KEY(sid, day));
// **
// **    SELECT name FROM sailors AS S WHERE EXISTS (
// **      SELECT * FROM reserves AS R WHERE S.sid = R.sid AND R.day = '2022-10-25'
// **    );
// **
// ** the SELECT statement may be transformed as follows:
// **
// **    SELECT name FROM sailors AS S, reserves AS R
// **      WHERE S.sid = R.sid AND R.day = '2022-10-25';
// **
// ** **Approximately**.  Really, we have to ensure that the FROM-clause term
// ** that was formerly inside the EXISTS is only executed once.  This is handled
// ** by setting the SrcItem.fg.fromExists flag, which then causes code in
// ** the where.c file to exit the corresponding loop after the first successful
// ** match (if any).
// */
fn existsToJoin(mut pParse: *mut Parse, mut p: *mut Select, mut pWhere: *mut Expr) {
    if (unsafe { (*pParse).nErr }) == (0 as i32)
        && pWhere != std::ptr::null_mut::<Expr>()
        && !((unsafe { (*pWhere).flags }) & (((1 as i32) | (2 as i32)) as u32)
            != ((0 as i32) as u32))
        && (unsafe { (*p).pSrc }) != std::ptr::null_mut::<SrcList>()
        && (unsafe { (*unsafe { (*p).pSrc }).nSrc })
            < (((8 as u64).wrapping_mul(((8 as i32) as i64) as u64) as u32) as i32)
        && ((unsafe { (*p).pLimit }) == std::ptr::null_mut::<Expr>()
            || (unsafe { (*unsafe { (*p).pLimit }).pRight }) == std::ptr::null_mut::<Expr>())
    {
        if (((unsafe { (*pWhere).op }) as u32) as i32) == (44 as i32) {
            let mut pRight: *mut Expr = unsafe { (*pWhere).pRight };
            existsToJoin(pParse, p, unsafe { (*pWhere).pLeft });
            existsToJoin(pParse, p, pRight);
        } else {
            if (((unsafe { (*pWhere).op }) as u32) as i32) == (20 as i32) {
                let mut pSub: *mut Select = unsafe { (*pWhere).x.pSelect };
                let mut pSubWhere: *mut Expr = unsafe { (*pSub).pWhere };
                if (unsafe { (*unsafe { (*pSub).pSrc }).nSrc }) == (1 as i32)
                    && (unsafe { (*pSub).selFlags }) & ((8 as i32) as u32) == ((0 as i32) as u32)
                    && !(((unsafe {
                        (*unsafe {
                            unsafe {
                                std::ptr::addr_of_mut!((*unsafe { (*pSub).pSrc }).a) as *mut SrcItem
                            }
                            .offset((0 as i32) as isize)
                        })
                        .fg
                        .__slate_bits_0
                        .__get_isSubquery()
                    }) as i32)
                        != (0 as i32))
                    && (unsafe { (*pSub).pLimit }) == std::ptr::null_mut::<Expr>()
                    && (unsafe { (*pSub).pPrior }) == std::ptr::null_mut::<Select>()
                {
                    // /* Before combining the sub-select with the parent, renumber the
                    //         ** cursor used by the subselect. This is because the EXISTS expression
                    //         ** might be a copy of another EXISTS expression from somewhere
                    //         ** else in the tree, and in this case it is important that it use
                    //         ** a unique cursor number.  */
                    let mut db: *mut sqlite3 = unsafe { (*pParse).db };
                    let mut aCsrMap: *mut i32 = (unsafe {
                        sqlite3DbMallocZero(
                            db,
                            ((((unsafe { (*pParse).nTab }) + (2 as i32)) as i64) as u64)
                                .wrapping_mul(4 as u64),
                        )
                    }) as *mut i32;
                    if aCsrMap == std::ptr::null_mut::<i32>() {
                        return;
                    }
                    unsafe {
                        *unsafe { aCsrMap.offset((0 as i32) as isize) } =
                            (unsafe { (*pParse).nTab }) + (1 as i32);
                    }
                    renumberCursors(pParse, pSub, -(1 as i32), aCsrMap);
                    unsafe { sqlite3DbFree(db, aCsrMap as *mut ()) };
                    unsafe { memset(pWhere as *mut (), 0 as i32, 72 as u64) };
                    unsafe {
                        (*pWhere).op = ((156 as i32) as i8) as u8;
                    }
                    unsafe {
                        (*pWhere).u.iValue = 1 as i32;
                    }
                    let __v2977: *mut Expr = pWhere;
                    let __v2978: u32 = unsafe { (*__v2977).flags };
                    let __v2979: u32 = __v2978 | ((2048 as i32) as u32);
                    unsafe {
                        (*__v2977).flags = __v2979;
                    }
                    0 as i32;
                    unsafe {
                        (*unsafe {
                            unsafe {
                                std::ptr::addr_of_mut!((*unsafe { (*pSub).pSrc }).a) as *mut SrcItem
                            }
                            .offset((0 as i32) as isize)
                        })
                        .fg
                        .__slate_bits_0
                        .__set_fromExists((1 as i32) as u32);
                    }
                    unsafe {
                        (*p).pSrc = unsafe {
                            sqlite3SrcListAppendList(pParse, unsafe { (*p).pSrc }, unsafe {
                                (*pSub).pSrc
                            })
                        };
                    }
                    if pSubWhere != std::ptr::null_mut::<Expr>() {
                        unsafe {
                            (*p).pWhere = unsafe {
                                sqlite3PExpr(pParse, 44 as i32, unsafe { (*p).pWhere }, pSubWhere)
                            };
                        }
                        unsafe {
                            (*pSub).pWhere = std::ptr::null_mut::<Expr>();
                        }
                    }
                    unsafe {
                        (*pSub).pSrc = std::ptr::null_mut::<SrcList>();
                    }
                    unsafe {
                        sqlite3ParserAddCleanup(
                            pParse,
                            Some(sqlite3SelectDeleteGeneric),
                            pSub as *mut (),
                        )
                    };
                    recomputeColumnsUsed(p, unsafe {
                        unsafe { std::ptr::addr_of_mut!((*unsafe { (*p).pSrc }).a) as *mut SrcItem }
                            .offset(
                                ((unsafe { (*unsafe { (*p).pSrc }).nSrc }) - (1 as i32)) as isize,
                            )
                    });
                }
            }
        }
    }
}

// /*
// ** True if the SrcList passed as the only argument contains at least
// ** one RIGHT or FULL JOIN. False otherwise.
// */
// /*
// ** The xExpr callback for the search of invalid ON clause terms.
// */
#[unsafe(link_section = ".text.slate_distinct.select.selectCheckOnClausesExpr")]
extern "C-unwind" fn selectCheckOnClausesExpr(
    mut pWalker: *mut Walker,
    mut pExpr: *mut Expr,
) -> i32 {
    let mut pCtx: *mut CheckOnCtx = unsafe { (*pWalker).u.pCheckOnCtx };
    // /* Check if pExpr is root or near-root of an ON clause constraint that needs
    //   ** to be checked to ensure that it does not refer to tables in its FROM
    //   ** clause to the right of itself. i.e. it is either:
    //   **
    //   **   + an ON clause on an OUTER join, or
    //   **   + an ON clause on an INNER join within a FROM that features at
    //   **     least one RIGHT or FULL join.
    //   */
    if (unsafe { (*pExpr).flags }) & ((1 as i32) as u32) != ((0 as i32) as u32)
        || (unsafe { (*pExpr).flags }) & ((2 as i32) as u32) != ((0 as i32) as u32)
            && (((unsafe {
                (*unsafe {
                    unsafe { std::ptr::addr_of_mut!((*unsafe { (*pCtx).pSrc }).a) as *mut SrcItem }
                        .offset((0 as i32) as isize)
                })
                .fg
                .jointype
            }) as u32) as i32)
                & (64 as i32)
                != (0 as i32)
    {
        // /* If CheckOnCtx.iJoin is already set, then fall through and process
        //     ** this expression node as normal. Or, if CheckOnCtx.iJoin is still 0,
        //     ** set it to the cursor number of the RHS of the join to which this
        //     ** ON expression was attached and then iterate through the entire
        //     ** expression.  */
        0 as i32;
        if (unsafe { (*pCtx).iJoin }) == (0 as i32) {
            unsafe {
                (*pCtx).iJoin = unsafe { (*pExpr).w.iJoin };
            }
            unsafe { sqlite3WalkExprNN(pWalker, pExpr) };
            unsafe {
                (*pCtx).iJoin = 0 as i32;
            }
            return 1 as i32;
        }
    }
    if (((unsafe { (*pExpr).op }) as u32) as i32) == (168 as i32) {
        // /* A column expression. Find the SrcList (if any) to which it refers.
        //     ** Then, if CheckOnCtx.iJoin indicates that this expression is part of an
        //     ** ON clause from that SrcList (i.e. if iJoin is non-zero), check that it
        //     ** does not refer to a table to the right of CheckOnCtx.iJoin. */
        let mut iTab: i32 = unsafe { (*pExpr).iTable };
        '__slate_break_2197: loop {
            let mut pSrc: *mut SrcList = unsafe { (*pCtx).pSrc };
            let mut nSrc: i32 = unsafe { (*pSrc).nSrc };
            let mut ii: i32 = 0 as i32;
            ii = 0 as i32;
            '__slate_break_2198: loop {
                if !(ii < nSrc
                    && (unsafe {
                        (*unsafe {
                            unsafe { std::ptr::addr_of_mut!((*pSrc).a) as *mut SrcItem }
                                .offset(ii as isize)
                        })
                        .iCursor
                    }) != iTab)
                {
                    break;
                }
                let __v2980: i32 = ii;
                let __v2981: i32 = __v2980 + (1 as i32);
                ii = __v2981;
            }
            if ii < nSrc {
                // /* pSrc is the FROM clause that contains iTab */
                if (unsafe { (*pCtx).iJoin }) != (0 as i32) {
                    let __v2982: i32 = ii;
                    let __v2983: i32 = __v2982 - (1 as i32);
                    ii = __v2983;
                    '__slate_break_2199: loop {
                        if !(ii >= (0 as i32)
                            && (unsafe {
                                (*unsafe {
                                    unsafe { std::ptr::addr_of_mut!((*pSrc).a) as *mut SrcItem }
                                        .offset(ii as isize)
                                })
                                .iCursor
                            }) != unsafe { (*pCtx).iJoin })
                        {
                            break;
                        }
                        let __v2984: i32 = ii;
                        let __v2985: i32 = __v2984 - (1 as i32);
                        ii = __v2985;
                    }
                    if ii >= (0 as i32) {
                        // /* Table iJoin appears to the left of table iTab in the SrcList.
                        //             ** Therefore the expression refers to a table to its right. */
                        unsafe {
                            sqlite3ErrorMsg(
                                unsafe { (*pWalker).pParse },
                                (b"%s references tables to its right\0".as_ptr() as *mut i8)
                                    as *const i8,
                                if (unsafe { (*pCtx).bFuncArg }) != (0 as i32) {
                                    b"table-function argument\0".as_ptr() as *mut i8
                                } else {
                                    b"ON clause\0".as_ptr() as *mut i8
                                },
                            )
                        };
                        return 2 as i32;
                    }
                }
                break '__slate_break_2197;
            }
            pCtx = unsafe { (*pCtx).pParent };
            if !(pCtx != std::ptr::null_mut::<CheckOnCtx>()) {
                break;
            }
        }
    }
    return 0 as i32;
}

// /*
// ** The xSelect callback for the search of invalid ON clause terms.
// */
#[unsafe(link_section = ".text.slate_distinct.select.selectCheckOnClausesSelect")]
extern "C-unwind" fn selectCheckOnClausesSelect(
    mut pWalker: *mut Walker,
    mut pSelect: *mut Select,
) -> i32 {
    let mut pCtx: *mut CheckOnCtx = unsafe { (*pWalker).u.pCheckOnCtx };
    if (unsafe { (*pSelect).pSrc }) == unsafe { (*pCtx).pSrc }
        || (unsafe { (*unsafe { (*pSelect).pSrc }).nSrc }) == (0 as i32)
    {
        return 0 as i32;
    } else {
        let mut sCtx: CheckOnCtx = unsafe { std::mem::zeroed() };
        unsafe { memset(std::ptr::addr_of_mut!(sCtx) as *mut (), 0 as i32, 24 as u64) };
        sCtx.pSrc = unsafe { (*pSelect).pSrc };
        sCtx.pParent = pCtx;
        unsafe {
            (*pWalker).u.pCheckOnCtx = std::ptr::addr_of_mut!(sCtx);
        }
        unsafe { sqlite3WalkSelect(pWalker, pSelect) };
        unsafe {
            (*pWalker).u.pCheckOnCtx = pCtx;
        }
        let __v2986: *mut Select = pSelect;
        let __v2987: u32 = unsafe { (*__v2986).selFlags };
        let __v2988: u32 = __v2987 & (!(1073741824 as i32) as u32);
        unsafe {
            (*__v2986).selFlags = __v2988;
        }
        return 1 as i32;
    }
    return unsafe { std::mem::zeroed() };
}

// /*
// ** If p2 exists and p1 and p2 have the same number of terms, then change
// ** every term of p1 to have the same sort order as p2 and return true.
// **
// ** If p2 is NULL or p1 and p2 are different lengths, then make no changes
// ** and return false.
// **
// ** p1 must be non-NULL.
// */
fn sqlite3CopySortOrder(mut p1: *mut ExprList, mut p2: *mut ExprList) -> i32 {
    0 as i32;
    if p2 != std::ptr::null_mut::<ExprList>() && (unsafe { (*p1).nExpr }) == unsafe { (*p2).nExpr }
    {
        let mut ii: i32 = 0 as i32;
        ii = 0 as i32;
        '__slate_break_2204: loop {
            if !(ii < unsafe { (*p1).nExpr }) {
                break;
            }
            let mut sortFlags: u8 = 0 as u8;
            sortFlags = (((((unsafe {
                (*unsafe {
                    unsafe { std::ptr::addr_of_mut!((*p2).a) as *mut ExprList_item }
                        .offset(ii as isize)
                })
                .fg
                .sortFlags
            }) as u32) as i32)
                & (1 as i32)) as i8) as u8;
            unsafe {
                (*unsafe {
                    unsafe { std::ptr::addr_of_mut!((*p1).a) as *mut ExprList_item }
                        .offset(ii as isize)
                })
                .fg
                .sortFlags = sortFlags;
            }
            let __v2989: i32 = ii;
            let __v2990: i32 = __v2989 + (1 as i32);
            ii = __v2990;
        }
        return 1 as i32;
    } else {
        return 0 as i32;
    }
    return unsafe { std::mem::zeroed() };
}
