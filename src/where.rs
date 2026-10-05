unsafe extern "C" {
    static mut sqlite3StrBINARY: [i8; 0];
    static mut sqlite3Config: Sqlite3Config;
    fn sqlite3_free(__v1407: *mut ());
    fn sqlite3_stricmp(__v1408: *const i8, __v1409: *const i8) -> i32;
    fn sqlite3_log(iErrCode: i32, zFormat: *const i8, ...);
    fn memcpy(__dest: *mut (), __src: *const (), __n: u64) -> *mut ();
    fn memmove(__dest: *mut (), __src: *const (), __n: u64) -> *mut ();
    fn memset(__s: *mut (), __c: i32, __n: u64) -> *mut ();
    fn sqlite3VdbeAddOp0(__v1430: *mut Vdbe, __v1431: i32) -> i32;
    fn sqlite3VdbeAddOp1(__v1432: *mut Vdbe, __v1433: i32, __v1434: i32) -> i32;
    fn sqlite3VdbeAddOp2(__v1435: *mut Vdbe, __v1436: i32, __v1437: i32, __v1438: i32) -> i32;
    fn sqlite3VdbeGoto(__v1439: *mut Vdbe, __v1440: i32) -> i32;
    fn sqlite3VdbeAddOp3(
        __v1441: *mut Vdbe,
        __v1442: i32,
        __v1443: i32,
        __v1444: i32,
        __v1445: i32,
    ) -> i32;
    fn sqlite3VdbeAddOp4(
        __v1446: *mut Vdbe,
        __v1447: i32,
        __v1448: i32,
        __v1449: i32,
        __v1450: i32,
        zP4: *const i8,
        __v1452: i32,
    ) -> i32;
    fn sqlite3VdbeAddOp4Int(
        __v1453: *mut Vdbe,
        __v1454: i32,
        __v1455: i32,
        __v1456: i32,
        __v1457: i32,
        __v1458: i32,
    ) -> i32;
    fn sqlite3VdbeExplain(__v1459: *mut Parse, __v1460: u8, __v1461: *const i8, ...) -> i32;
    fn sqlite3VdbeChangeP2(__v1462: *mut Vdbe, addr: i32, P2: i32);
    fn sqlite3VdbeChangeP5(__v1465: *mut Vdbe, P5: u16);
    fn sqlite3VdbeJumpHere(__v1467: *mut Vdbe, addr: i32);
    fn sqlite3VdbeChangeP4(__v1469: *mut Vdbe, addr: i32, zP4: *const i8, N: i32);
    fn sqlite3VdbeAppendP4(__v1473: *mut Vdbe, pP4: *mut (), p4type: i32);
    fn sqlite3VdbeSetP4KeyInfo(__v1476: *mut Parse, __v1477: *mut Index);
    fn sqlite3VdbeGetOp(__v1478: *mut Vdbe, __v1479: i32) -> *mut VdbeOp;
    fn sqlite3VdbeMakeLabel(__v1480: *mut Parse) -> i32;
    fn sqlite3VdbeResolveLabel(__v1481: *mut Vdbe, __v1482: i32);
    fn sqlite3VdbeCurrentAddr(__v1483: *mut Vdbe) -> i32;
    fn sqlite3WalkExpr(__v1484: *mut Walker, __v1485: *mut Expr) -> i32;
    fn sqlite3WalkSelect(__v1486: *mut Walker, __v1487: *mut Select) -> i32;
    fn sqlite3SelectWalkNoop(__v1488: *mut Walker, __v1489: *mut Select) -> i32;
    fn sqlite3SelectWalkFail(__v1490: *mut Walker, __v1491: *mut Select) -> i32;
    fn sqlite3MisuseError(__v1492: i32) -> i32;
    fn sqlite3StrICmp(__v1493: *const i8, __v1494: *const i8) -> i32;
    fn sqlite3DbMallocZero(__v1495: *mut sqlite3, __v1496: u64) -> *mut ();
    fn sqlite3DbMallocRaw(__v1497: *mut sqlite3, __v1498: u64) -> *mut ();
    fn sqlite3DbMallocRawNN(__v1499: *mut sqlite3, __v1500: u64) -> *mut ();
    fn sqlite3DbFree(__v1501: *mut sqlite3, __v1502: *mut ());
    fn sqlite3DbFreeNN(__v1503: *mut sqlite3, __v1504: *mut ());
    fn sqlite3DbNNFreeNN(__v1505: *mut sqlite3, __v1506: *mut ());
    fn sqlite3ProgressCheck(__v1507: *mut Parse);
    fn sqlite3ErrorMsg(__v1508: *mut Parse, __v1509: *const i8, ...);
    fn sqlite3GetTempReg(__v1510: *mut Parse) -> i32;
    fn sqlite3ReleaseTempReg(__v1511: *mut Parse, __v1512: i32);
    fn sqlite3GetTempRange(__v1513: *mut Parse, __v1514: i32) -> i32;
    fn sqlite3ReleaseTempRange(__v1515: *mut Parse, __v1516: i32, __v1517: i32);
    fn sqlite3ExprAnd(__v1518: *mut Parse, __v1519: *mut Expr, __v1520: *mut Expr) -> *mut Expr;
    fn sqlite3ExprDelete(__v1521: *mut sqlite3, __v1522: *mut Expr);
    fn sqlite3ColumnExpr(__v1523: *mut Table, __v1524: *mut Column) -> *mut Expr;
    fn sqlite3ColumnColl(__v1525: *mut Column) -> *const i8;
    fn sqlite3PrimaryKeyIndex(__v1526: *mut Table) -> *mut Index;
    fn sqlite3TableColumnToIndex(__v1527: *mut Index, __v1528: i32) -> i32;
    fn sqlite3StorageColumnToTable(__v1529: *mut Table, __v1530: i16) -> i16;
    fn sqlite3FaultSim(__v1531: i32) -> i32;
    fn sqlite3AllocateIndexObject(
        __v1532: *mut sqlite3,
        __v1533: i32,
        __v1534: i32,
        __v1535: *mut *mut i8,
    ) -> *mut Index;
    fn sqlite3IndexBloomable(__v1536: *const Index, __v1537: i32) -> i32;
    fn sqlite3OpenTable(
        __v1538: *mut Parse,
        iCur: i32,
        iDb: i32,
        __v1541: *mut Table,
        __v1542: i32,
    );
    fn sqlite3ExprCodeLoadIndexColumn(
        __v1564: *mut Parse,
        __v1565: *mut Index,
        __v1566: i32,
        __v1567: i32,
        __v1568: i32,
    );
    fn sqlite3ExprIfFalse(__v1569: *mut Parse, __v1570: *mut Expr, __v1571: i32, __v1572: i32);
    fn sqlite3ExprCompare(
        __v1573: *const Parse,
        __v1574: *const Expr,
        __v1575: *const Expr,
        __v1576: i32,
    ) -> i32;
    fn sqlite3ExprCompareSkip(__v1577: *mut Expr, __v1578: *mut Expr, __v1579: i32) -> i32;
    fn sqlite3ExprImpliesExpr(
        __v1580: *const Parse,
        __v1581: *const Expr,
        __v1582: *const Expr,
        __v1583: i32,
    ) -> i32;
    fn sqlite3ExprCoveredByIndex(__v1584: *mut Expr, iCur: i32, pIdx: *mut Index) -> i32;
    fn sqlite3CodeVerifySchema(__v1587: *mut Parse, __v1588: i32);
    fn sqlite3ExprIsConstant(__v1589: *mut Parse, __v1590: *mut Expr) -> i32;
    fn sqlite3ExprIsSingleTableConstraint(
        __v1591: *mut Expr,
        __v1592: *const SrcList,
        __v1593: i32,
        __v1594: i32,
    ) -> i32;
    fn sqlite3ExprIsInteger(
        __v1595: *const Expr,
        __v1596: *mut i32,
        __v1597: *mut Parse,
        __v1598: i32,
    ) -> i32;
    fn sqlite3ExprIsLikeOperator(__v1599: *const Expr) -> i32;
    fn sqlite3GenerateIndexKey(
        __v1600: *mut Parse,
        __v1601: *mut Index,
        __v1602: i32,
        __v1603: i32,
        __v1604: i32,
        __v1605: *mut i32,
        __v1606: *mut Index,
        __v1607: i32,
    ) -> i32;
    fn sqlite3BeginWriteOperation(__v1608: *mut Parse, __v1609: i32, __v1610: i32);
    fn sqlite3ExprDup(__v1611: *mut sqlite3, __v1612: *const Expr, __v1613: i32) -> *mut Expr;
    fn sqlite3LogEst(__v1614: u64) -> i16;
    fn sqlite3LogEstAdd(__v1615: i16, __v1616: i16) -> i16;
    fn sqlite3LogEstFromDouble(__v1617: f64) -> i16;
    fn sqlite3LogEstToInt(__v1618: i16) -> u64;
    fn sqlite3IndexAffinityStr(__v1619: *mut sqlite3, __v1620: *mut Index) -> *const i8;
    fn sqlite3CompareAffinity(pExpr: *const Expr, aff2: i8) -> i8;
    fn sqlite3IndexAffinityOk(pExpr: *const Expr, idx_affinity: i8) -> i32;
    fn sqlite3TableColumnAffinity(__v1625: *const Table, __v1626: i32) -> i8;
    fn sqlite3ExprAffinity(pExpr: *const Expr) -> i8;
    fn sqlite3ErrStr(__v1628: i32) -> *const i8;
    fn sqlite3IsBinary(__v1629: *const CollSeq) -> i32;
    fn sqlite3ExprNNCollSeq(pParse: *mut Parse, pExpr: *const Expr) -> *mut CollSeq;
    fn sqlite3ExprSkipCollateAndLikely(__v1632: *mut Expr) -> *mut Expr;
    fn sqlite3ValueFree(__v1633: *mut sqlite3_value);
    fn sqlite3ValueFromExpr(
        __v1634: *mut sqlite3,
        __v1635: *const Expr,
        __v1636: u8,
        __v1637: u8,
        __v1638: *mut *mut sqlite3_value,
    ) -> i32;
    fn sqlite3SchemaToIndex(db: *mut sqlite3, __v1640: *mut Schema) -> i32;
    fn sqlite3KeyInfoAlloc(__v1641: *mut sqlite3, __v1642: i32, __v1643: i32) -> *mut KeyInfo;
    fn sqlite3OomFault(__v1644: *mut sqlite3) -> *mut ();
    fn sqlite3TableLock(
        __v1645: *mut Parse,
        __v1646: i32,
        __v1647: u32,
        __v1648: u8,
        __v1649: *const i8,
    );
    fn sqlite3GetVTable(__v1650: *mut sqlite3, __v1651: *mut Table) -> *mut VTable;
    fn sqlite3ParserAddCleanup(
        __v1653: *mut Parse,
        __v1654: Option<unsafe extern "C-unwind" fn(*mut sqlite3, *mut ())>,
        __v1655: *mut (),
    ) -> *mut ();
    fn sqlite3ExprCompareCollSeq(__v1656: *mut Parse, __v1657: *const Expr) -> *mut CollSeq;
    fn sqlite3BinaryCompareCollSeq(
        __v1658: *mut Parse,
        __v1659: *const Expr,
        __v1660: *const Expr,
    ) -> *mut CollSeq;
    fn sqlite3ExprVectorSize(pExpr: *const Expr) -> i32;
    fn sqlite3ExprIsVector(pExpr: *const Expr) -> i32;
    fn sqlite3WhereExplainOneScan(
        pParse: *mut Parse,
        pTabList: *mut SrcList,
        pLevel: *mut WhereLevel,
        wctrlFlags: u16,
    ) -> i32;
    fn sqlite3WhereExplainBloomFilter(
        pParse: *const Parse,
        pWInfo: *const WhereInfo,
        pLevel: *const WhereLevel,
    ) -> i32;
    fn sqlite3WhereAddExplainText(
        pParse: *mut Parse,
        addr: i32,
        pTabList: *mut SrcList,
        pLevel: *mut WhereLevel,
        wctrlFlags: u16,
    );
    fn sqlite3WhereCodeOneLoopStart(
        pParse: *mut Parse,
        v: *mut Vdbe,
        pWInfo: *mut WhereInfo,
        iLevel: i32,
        pLevel: *mut WhereLevel,
        notReady: u64,
    ) -> u64;
    fn sqlite3WhereRightJoinLoop(pWInfo: *mut WhereInfo, iLevel: i32, pLevel: *mut WhereLevel);
    fn sqlite3WhereClauseInit(__v1697: *mut WhereClause, __v1698: *mut WhereInfo);
    fn sqlite3WhereClauseClear(__v1699: *mut WhereClause);
    fn sqlite3WhereSplit(__v1700: *mut WhereClause, __v1701: *mut Expr, __v1702: u8);
    fn sqlite3WhereAddLimit(__v1703: *mut WhereClause, __v1704: *mut Select);
    fn sqlite3WhereExprUsage(__v1705: *mut WhereMaskSet, __v1706: *mut Expr) -> u64;
    fn sqlite3WhereExprListUsage(__v1707: *mut WhereMaskSet, __v1708: *mut ExprList) -> u64;
    fn sqlite3WhereExprAnalyze(__v1709: *mut SrcList, __v1710: *mut WhereClause);
    fn sqlite3WhereTabFuncArgs(
        __v1711: *mut Parse,
        __v1712: *mut SrcItem,
        __v1713: *mut WhereClause,
    );
    fn sqlite3WhereLoopBloomable(__v1714: *const WhereLoop) -> i32;
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
    trace: __SlateRecord176,
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
    u1: __SlateRecord177,
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
struct sqlite3_pcache {}

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
    __slate_bits_0: __slate_bits::__SlateBits75U0,
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
    u: __SlateRecord187,
    pLeft: *mut Expr,
    pRight: *mut Expr,
    x: __SlateRecord188,
    nHeight: i32,
    iTable: i32,
    iColumn: i16,
    iAgg: i16,
    w: __SlateRecord189,
    pAggInfo: *mut AggInfo,
    y: __SlateRecord190,
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
    u: __SlateRecord178,
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
    __slate_bits_0: __slate_bits::__SlateBits101U0,
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
    uNC: __SlateRecord202,
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
    fg: __SlateRecord197,
    iCursor: i32,
    colUsed: u64,
    u1: __SlateRecord198,
    u2: __SlateRecord199,
    u3: __SlateRecord200,
    u4: __SlateRecord201,
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
    u: __SlateRecord179,
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
    u: __SlateRecord208,
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
    __slate_bits_0: __slate_bits::__SlateBits153U0,
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
    __slate_bits_0: __slate_bits::__SlateBits175U0,
    azInit: *mut *const i8,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord176 {
    xLegacy: Option<unsafe extern "C-unwind" fn(*mut (), *const i8)>,
    xV2: Option<unsafe extern "C-unwind" fn(u32, *mut (), *mut (), *mut ()) -> i32>,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord177 {
    isInterrupted: i32,
    notUsed1: f64,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord178 {
    pHash: *mut FuncDef,
    pDestructor: *mut FuncDestructor,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord179 {
    tab: __SlateRecord180,
    view: __SlateRecord181,
    vtab: __SlateRecord182,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord180 {
    addColOffset: i32,
    pFKey: *mut FKey,
    pDfltList: *mut ExprList,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord181 {
    pSelect: *mut Select,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord182 {
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
union __SlateRecord187 {
    zToken: *mut i8,
    iValue: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord188 {
    pList: *mut ExprList,
    pSelect: *mut Select,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord189 {
    iJoin: i32,
    iOfst: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord190 {
    pTab: *mut Table,
    pWin: *mut Window,
    nReg: i32,
    sub: __SlateRecord191,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord191 {
    iAddr: i32,
    regReturn: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct ExprList_item {
    pExpr: *mut Expr,
    zEName: *mut i8,
    fg: __SlateRecord193,
    u: __SlateRecord194,
}

#[repr(C, align(4))]
#[derive(Clone, Copy)]
struct __SlateRecord193 {
    sortFlags: u8,
    __slate_bits_0: __slate_bits::__SlateBits193U0,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord194 {
    x: __SlateRecord195,
    iConstExprReg: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord195 {
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
struct __SlateRecord197 {
    jointype: u8,
    __slate_bits_0: __slate_bits::__SlateBits197U0,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord198 {
    zIndexedBy: *mut i8,
    pFuncArg: *mut ExprList,
    nRow: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord199 {
    pIBIndex: *mut Index,
    pCteUse: *mut CteUse,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord200 {
    pOn: *mut Expr,
    pUsing: *mut IdList,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord201 {
    pSchema: *mut Schema,
    zDatabase: *mut i8,
    pSubq: *mut Subquery,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord202 {
    pEList: *mut ExprList,
    pAggInfo: *mut AggInfo,
    pUpsert: *mut Upsert,
    iBaseReg: i32,
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
union __SlateRecord208 {
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
// ** This module contains C code that generates VDBE code used to process
// ** the WHERE clause of SQL statements.  This module is responsible for
// ** generating the code that loops through a table looking for applicable
// ** rows.  Indices are selected and used to speed the search when doing
// ** so is applicable.  Because this module is responsible for selecting
// ** indices, you might also think of this module as the "query optimizer".
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

#[repr(C)]
#[derive(Clone, Copy)]
struct RenameCtx {}

// /*
// ** Structure passed to the whereIsCoveringIndex Walker callback.
// */
#[repr(C)]
#[derive(Clone, Copy)]
struct CoveringIndexCheck {
    pIdx: *mut Index,
    // /* The index */
    iTabCur: i32,
    // /* Cursor number for the corresponding table */
    bExpr: u8,
    // /* Uses an indexed expression */
    bUnidx: u8,
    // /* Uses an unindexed column not within an indexed expr */
}

#[repr(C)]
#[derive(Clone, Copy)]
struct CheckOnCtx {}

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
struct WhereMaskSet {
    bVarSelect: i32,
    n: i32,
    ix: [i32; 64],
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
    u: __SlateRecord245,
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
    u: __SlateRecord248,
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
struct WherePath {
    maskLoop: u64,
    revLoop: u64,
    nRow: i16,
    rCost: i16,
    rUnsort: i16,
    isOrdered: i8,
    aLoop: *mut *mut WhereLoop,
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
    u: __SlateRecord251,
    prereqRight: u64,
    prereqAll: u64,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct WhereLoopBuilder {
    pWInfo: *mut WhereInfo,
    pWC: *mut WhereClause,
    pNew: *mut WhereLoop,
    pOrSet: *mut WhereOrSet,
    bldFlags1: u8,
    bldFlags2: u8,
    iPlanLimit: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct WhereScan {
    pOrigWC: *mut WhereClause,
    pWC: *mut WhereClause,
    zCollName: *const i8,
    pIdxExpr: *mut Expr,
    k: i32,
    opMask: u32,
    idxaff: i8,
    iEquiv: u8,
    nEquiv: u8,
    aiCur: [i32; 11],
    aiColumn: [i16; 11],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct WhereOrCost {
    prereq: u64,
    rRun: i16,
    nOut: i16,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct WhereOrSet {
    n: u16,
    a: [WhereOrCost; 3],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct WhereMemBlock {
    pNext: *mut WhereMemBlock,
    sz: u64,
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
union __SlateRecord245 {
    r#in: __SlateRecord246,
    pCoveringIdx: *mut Index,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord246 {
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
union __SlateRecord248 {
    btree: __SlateRecord249,
    vtab: __SlateRecord250,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord249 {
    nEq: u16,
    nBtm: u16,
    nTop: u16,
    nDistinctCol: u16,
    pIndex: *mut Index,
    pOrderBy: *mut ExprList,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord250 {
    idxNum: i32,
    __slate_bits_0: __slate_bits::__SlateBits250U0,
    isOrdered: i8,
    omitMask: u16,
    idxStr: *mut i8,
    mHandleIn: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord251 {
    x: __SlateRecord252,
    pOrInfo: *mut WhereOrInfo,
    pAndInfo: *mut WhereAndInfo,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord252 {
    leftColumn: i32,
    iField: i32,
}

// /*
// ** Extra information appended to the end of sqlite3_index_info but not
// ** visible to the xBestIndex function, at least not directly.  The
// ** sqlite3_vtab_collation() interface knows how to reach it, however.
// **
// ** This object is not an API and can be changed from one release to the
// ** next.  As long as allocateIndexInfo() and sqlite3_vtab_collation()
// ** agree on the structure, all will be well.
// */
#[repr(C)]
#[derive(Clone, Copy)]
struct HiddenIndexInfo {
    pWC: *mut WhereClause,
    // /* The Where clause being analyzed */
    pParse: *mut Parse,
    // /* The parsing context */
    eDistinct: i32,
    // /* Value to return from sqlite3_vtab_distinct() */
    mIn: u32,
    // /* Mask of terms that are <col> IN (...) */
    mHandleIn: u32,
    // /* Terms that vtab will handle as <col> IN (...) */
    aRhs: [*mut sqlite3_value; 0],
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
    pub struct __SlateBits75U0 {
        #[bits(4)]
        pub notNull: u32,
        #[bits(4)]
        pub eCType: u32,
    }
    #[bitfields::bitfield([u8; 2], c_names = true, new = false, from_into_bits = false, from_traits = false, default = false, debug = false, builder = false, bit_ops = false)]
    pub struct __SlateBits193U0 {
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
    pub struct __SlateBits197U0 {
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
    pub struct __SlateBits101U0 {
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
    pub struct __SlateBits175U0 {
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
    pub struct __SlateBits250U0 {
        #[bits(1)]
        pub needFree: u32,
        #[bits(1)]
        pub bOmitOffset: u32,
        #[bits(1)]
        pub bIdxNumHex: u32,
        #[bits(5, access = na)]
        pub __slate_pad_3: u8,
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
    pub struct __SlateBits153U0 {
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
}

// /* Mask of tables that must be used. */
// /* Mask of usable tables */
// /* Exclude terms using these operators */
// /* Populated object for xBestIndex */
// /* Do not omit these constraints */
// /* OUT: True if plan uses an IN(...) op */
// /* OUT: Retry without LIMIT/OFFSET */
// /*
// ** Return the collating sequence for a constraint passed into xBestIndex.
// **
// ** pIdxInfo must be an sqlite3_index_info structure passed into xBestIndex.
// ** This routine depends on there being a HiddenIndexInfo structure immediately
// ** following the sqlite3_index_info structure.
// **
// ** Return a pointer to the collation name:
// **
// **    1. If there is an explicit COLLATE operator on the constraint, return it.
// **
// **    2. Else, if the column has an alternative collation, return that.
// **
// **    3. Otherwise, return "BINARY".
// */
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.where.sqlite3_vtab_collation")]
extern "C-unwind" fn sqlite3_vtab_collation(
    mut pIdxInfo: *mut sqlite3_index_info,
    mut iCons: i32,
) -> *const i8 {
    let mut pHidden: *mut HiddenIndexInfo =
        (unsafe { pIdxInfo.offset((1 as i32) as isize) }) as *mut HiddenIndexInfo;
    let mut zRet: *const i8 = std::ptr::null::<i8>();
    if iCons >= (0 as i32) && iCons < unsafe { (*pIdxInfo).nConstraint } {
        let mut pC: *mut CollSeq = std::ptr::null_mut::<CollSeq>();
        let mut iTerm: i32 = unsafe {
            (*unsafe { unsafe { (*pIdxInfo).aConstraint }.offset(iCons as isize) }).iTermOffset
        };
        let mut pX: *mut Expr =
            unsafe { (*termFromWhereClause(unsafe { (*pHidden).pWC }, iTerm)).pExpr };
        if (unsafe { (*pX).pLeft }) != std::ptr::null_mut::<Expr>() {
            pC = unsafe {
                sqlite3ExprCompareCollSeq(unsafe { (*pHidden).pParse }, pX as *const Expr)
            };
        }
        zRet = if pC != std::ptr::null_mut::<CollSeq>() {
            (unsafe { (*pC).zName }) as *const i8
        } else {
            unsafe { std::ptr::addr_of!(sqlite3StrBINARY) as *const i8 }
        };
    }
    return zRet;
}

// /* Copy of first argument to xBestIndex */
// /* Constraint for which RHS is wanted */
// /* Write value extracted here */
// /*
// ** Return true if ORDER BY clause may be handled as DISTINCT.
// */
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.where.sqlite3_vtab_distinct")]
extern "C-unwind" fn sqlite3_vtab_distinct(mut pIdxInfo: *mut sqlite3_index_info) -> i32 {
    let mut pHidden: *mut HiddenIndexInfo =
        (unsafe { pIdxInfo.offset((1 as i32) as isize) }) as *mut HiddenIndexInfo;
    0 as i32;
    return unsafe { (*pHidden).eDistinct };
}

// /*
// ** Return true if constraint iCons is really an IN(...) constraint, or
// ** false otherwise. If iCons is an IN(...) constraint, set (if bHandle!=0)
// ** or clear (if bHandle==0) the flag to handle it using an iterator.
// */
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.where.sqlite3_vtab_in")]
extern "C-unwind" fn sqlite3_vtab_in(
    mut pIdxInfo: *mut sqlite3_index_info,
    mut iCons: i32,
    mut bHandle: i32,
) -> i32 {
    let mut pHidden: *mut HiddenIndexInfo =
        (unsafe { pIdxInfo.offset((1 as i32) as isize) }) as *mut HiddenIndexInfo;
    let mut m: u32 = if iCons <= (31 as i32) {
        ((1 as i32) as u32) << iCons
    } else {
        (0 as i32) as u32
    };
    if m & unsafe { (*pHidden).mIn } != (0 as u32) {
        if bHandle == (0 as i32) {
            let __v1853: *mut HiddenIndexInfo = pHidden;
            let __v1854: u32 = unsafe { (*__v1853).mHandleIn };
            let __v1855: u32 = __v1854 & !m;
            unsafe {
                (*__v1853).mHandleIn = __v1855;
            }
        } else {
            if bHandle > (0 as i32) {
                let __v1856: *mut HiddenIndexInfo = pHidden;
                let __v1857: u32 = unsafe { (*__v1856).mHandleIn };
                let __v1858: u32 = __v1857 | m;
                unsafe {
                    (*__v1856).mHandleIn = __v1858;
                }
            }
        }
        return 1 as i32;
    }
    return 0 as i32;
}

// /*
// ** This interface is callable from within the xBestIndex callback only.
// **
// ** If possible, set (*ppVal) to point to an object containing the value
// ** on the right-hand-side of constraint iCons.
// */
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.where.sqlite3_vtab_rhs_value")]
extern "C-unwind" fn sqlite3_vtab_rhs_value(
    mut pIdxInfo: *mut sqlite3_index_info,
    mut iCons: i32,
    mut ppVal: *mut *mut sqlite3_value,
) -> i32 {
    let mut pH: *mut HiddenIndexInfo =
        (unsafe { pIdxInfo.offset((1 as i32) as isize) }) as *mut HiddenIndexInfo;
    let mut pVal: *mut sqlite3_value = std::ptr::null_mut::<sqlite3_value>();
    let mut rc: i32 = 0 as i32;
    if iCons < (0 as i32) || iCons >= unsafe { (*pIdxInfo).nConstraint } {
        // /* EV: R-30545-25046 */
        rc = unsafe { sqlite3MisuseError(4606 as i32) };
    } else {
        if (unsafe {
            *unsafe {
                unsafe { std::ptr::addr_of_mut!((*pH).aRhs) as *mut *mut sqlite3_value }
                    .offset(iCons as isize)
            }
        }) == std::ptr::null_mut::<sqlite3_value>()
        {
            let mut pTerm: *mut WhereTerm = termFromWhereClause(unsafe { (*pH).pWC }, unsafe {
                (*unsafe { unsafe { (*pIdxInfo).aConstraint }.offset(iCons as isize) }).iTermOffset
            });
            rc = unsafe {
                sqlite3ValueFromExpr(
                    unsafe { (*unsafe { (*pH).pParse }).db },
                    (unsafe { (*unsafe { (*pTerm).pExpr }).pRight }) as *const Expr,
                    unsafe { (*unsafe { (*unsafe { (*pH).pParse }).db }).enc },
                    ((65 as i32) as i8) as u8,
                    unsafe {
                        unsafe { std::ptr::addr_of_mut!((*pH).aRhs) as *mut *mut sqlite3_value }
                            .offset(iCons as isize)
                    },
                )
            };
            {}
        }
        pVal = unsafe {
            *unsafe {
                unsafe { std::ptr::addr_of_mut!((*pH).aRhs) as *mut *mut sqlite3_value }
                    .offset(iCons as isize)
            }
        };
    }
    unsafe {
        *ppVal = pVal;
    }
    // /* IMP: R-19933-32160 */
    if rc == (0 as i32) && pVal == std::ptr::null_mut::<sqlite3_value>() {
        // /* IMP: R-36424-56542 */
        rc = 12 as i32;
    }
    return rc;
}

// /*
// ** Generate the beginning of the loop used for WHERE clause processing.
// ** The return value is a pointer to an opaque structure that contains
// ** information needed to terminate the loop.  Later, the calling routine
// ** should invoke sqlite3WhereEnd() with the return value of this function
// ** in order to complete the WHERE clause processing.
// **
// ** If an error occurs, this routine returns NULL.
// **
// ** The basic idea is to do a nested loop, one loop for each table in
// ** the FROM clause of a select.  (INSERT and UPDATE statements are the
// ** same as a SELECT with only a single table in the FROM clause.)  For
// ** example, if the SQL is this:
// **
// **       SELECT * FROM t1, t2, t3 WHERE ...;
// **
// ** Then the code generated is conceptually like the following:
// **
// **      foreach row1 in t1 do       \    Code generated
// **        foreach row2 in t2 do      |-- by sqlite3WhereBegin()
// **          foreach row3 in t3 do   /
// **            ...
// **          end                     \    Code generated
// **        end                        |-- by sqlite3WhereEnd()
// **      end                         /
// **
// ** Note that the loops might not be nested in the order in which they
// ** appear in the FROM clause if a different order is better able to make
// ** use of indices.  Note also that when the IN operator appears in
// ** the WHERE clause, it might result in additional nested loops for
// ** scanning through all values on the right-hand side of the IN.
// **
// ** There are Btree cursors associated with each table.  t1 uses cursor
// ** number pTabList->a[0].iCursor.  t2 uses the cursor pTabList->a[1].iCursor.
// ** And so forth.  This routine generates code to open those VDBE cursors
// ** and sqlite3WhereEnd() generates the code to close them.
// **
// ** The code that sqlite3WhereBegin() generates leaves the cursors named
// ** in pTabList pointing at their appropriate entries.  The [...] code
// ** can use OP_Column and OP_Rowid opcodes on these cursors to extract
// ** data from the various tables of the loop.
// **
// ** If the WHERE clause is empty, the foreach loops must each scan their
// ** entire tables.  Thus a three-way join is an O(N^3) operation.  But if
// ** the tables have indices and there are terms in the WHERE clause that
// ** refer to those indices, a complete table scan can be avoided and the
// ** code will run much faster.  Most of the work of this routine is checking
// ** to see if there are indices that can be used to speed up the loop.
// **
// ** Terms of the WHERE clause are also used to limit which rows actually
// ** make it to the "..." in the middle of the loop.  After each "foreach",
// ** terms of the WHERE clause that use only terms in that loop and outer
// ** loops are evaluated and if false a jump is made around all subsequent
// ** inner loops (or around the "..." if the test occurs within the inner-
// ** most loop)
// **
// ** OUTER JOINS
// **
// ** An outer join of tables t1 and t2 is conceptually coded as follows:
// **
// **    foreach row1 in t1 do
// **      flag = 0
// **      foreach row2 in t2 do
// **        start:
// **          ...
// **          flag = 1
// **      end
// **      if flag==0 then
// **        move the row2 cursor to a null row
// **        goto start
// **      fi
// **    end
// **
// ** ORDER BY CLAUSE PROCESSING
// **
// ** pOrderBy is a pointer to the ORDER BY clause (or the GROUP BY clause
// ** if the WHERE_GROUPBY flag is set in wctrlFlags) of a SELECT statement
// ** if there is one.  If there is no ORDER BY clause or if this routine
// ** is called from an UPDATE or DELETE statement, then pOrderBy is NULL.
// **
// ** The iIdxCur parameter is the cursor number of an index.  If
// ** WHERE_OR_SUBCLAUSE is set, iIdxCur is the cursor number of an index
// ** to use for OR clause processing.  The WHERE clause should use this
// ** specific cursor.  If WHERE_ONEPASS_DESIRED is set, then iIdxCur is
// ** the first cursor in an array of cursors for all indices.  iIdxCur should
// ** be used to compute the appropriate cursor depending on which index is
// ** used.
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3WhereBegin(
    mut pParse: *mut Parse,
    mut pTabList: *mut SrcList,
    mut pWhere: *mut Expr,
    mut pOrderBy: *mut ExprList,
    mut pResultSet: *mut ExprList,
    mut pSelect: *mut Select,
    mut wctrlFlags: u16,
    mut iAuxArg: i32,
) -> *mut WhereInfo {
    let mut __slate_storage_1916: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1916: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1916) as *mut i32;
    let mut __slate_storage_1915: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1915: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1915) as *mut i32;
    let mut __slate_storage_1368: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1368: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1368) as *mut i32;
    let mut __slate_storage_1367: std::mem::MaybeUninit<*mut Subquery> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1367: *mut *mut Subquery =
        std::ptr::addr_of_mut!(__slate_storage_1367) as *mut *mut Subquery;
    let mut __slate_storage_1366: std::mem::MaybeUninit<*mut SrcItem> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1366: *mut *mut SrcItem =
        std::ptr::addr_of_mut!(__slate_storage_1366) as *mut *mut SrcItem;
    let mut __slate_storage_1365: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1365: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1365) as *mut i32;
    let mut __slate_storage_1364: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1364: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1364) as *mut i32;
    let mut __slate_storage_1893: std::mem::MaybeUninit<*mut WhereLevel> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1893: *mut *mut WhereLevel =
        std::ptr::addr_of_mut!(__slate_storage_1893) as *mut *mut WhereLevel;
    let mut __slate_storage_1892: std::mem::MaybeUninit<*mut WhereLevel> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1892: *mut *mut WhereLevel =
        std::ptr::addr_of_mut!(__slate_storage_1892) as *mut *mut WhereLevel;
    let mut __slate_storage_1891: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1891: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1891) as *mut i32;
    let mut __slate_storage_1890: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1890: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1890) as *mut i32;
    let mut __slate_storage_1914: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1914: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_1914) as *mut u32;
    let mut __slate_storage_1913: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1913: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_1913) as *mut u32;
    let mut __slate_storage_1912: std::mem::MaybeUninit<*mut WhereLoop> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1912: *mut *mut WhereLoop =
        std::ptr::addr_of_mut!(__slate_storage_1912) as *mut *mut WhereLoop;
    let mut __slate_storage_1911: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1911: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1911) as *mut i32;
    let mut __slate_storage_1910: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1910: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1910) as *mut i32;
    let mut __slate_storage_1909: std::mem::MaybeUninit<*mut Parse> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1909: *mut *mut Parse =
        std::ptr::addr_of_mut!(__slate_storage_1909) as *mut *mut Parse;
    let mut __slate_storage_1362: std::mem::MaybeUninit<*mut KeyInfo> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1362: *mut *mut KeyInfo =
        std::ptr::addr_of_mut!(__slate_storage_1362) as *mut *mut KeyInfo;
    let mut __slate_storage_1363: std::mem::MaybeUninit<*mut Index> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1363: *mut *mut Index =
        std::ptr::addr_of_mut!(__slate_storage_1363) as *mut *mut Index;
    let mut __slate_storage_1908: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1908: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1908) as *mut i32;
    let mut __slate_storage_1907: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1907: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1907) as *mut i32;
    let mut __slate_storage_1906: std::mem::MaybeUninit<*mut Parse> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1906: *mut *mut Parse =
        std::ptr::addr_of_mut!(__slate_storage_1906) as *mut *mut Parse;
    let mut __slate_storage_1905: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1905: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1905) as *mut i32;
    let mut __slate_storage_1904: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1904: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1904) as *mut i32;
    let mut __slate_storage_1903: std::mem::MaybeUninit<*mut Parse> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1903: *mut *mut Parse =
        std::ptr::addr_of_mut!(__slate_storage_1903) as *mut *mut Parse;
    let mut __slate_storage_1361: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1361: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1361) as *mut i32;
    let mut __slate_storage_1360: std::mem::MaybeUninit<*mut WhereRightJoin> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1360: *mut *mut WhereRightJoin =
        std::ptr::addr_of_mut!(__slate_storage_1360) as *mut *mut WhereRightJoin;
    let mut __slate_storage_1902: std::mem::MaybeUninit<*mut WhereRightJoin> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1902: *mut *mut WhereRightJoin =
        std::ptr::addr_of_mut!(__slate_storage_1902) as *mut *mut WhereRightJoin;
    let mut __slate_storage_1901: std::mem::MaybeUninit<bool> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1901: *mut bool = std::ptr::addr_of_mut!(__slate_storage_1901) as *mut bool;
    let mut __slate_storage_1897: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1897: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1897) as *mut i32;
    let mut __slate_storage_1896: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1896: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1896) as *mut i32;
    let mut __slate_storage_1359: std::mem::MaybeUninit<*mut Index> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1359: *mut *mut Index =
        std::ptr::addr_of_mut!(__slate_storage_1359) as *mut *mut Index;
    let mut __slate_storage_1900: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1900: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1900) as *mut i32;
    let mut __slate_storage_1899: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1899: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1899) as *mut i32;
    let mut __slate_storage_1898: std::mem::MaybeUninit<*mut Parse> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1898: *mut *mut Parse =
        std::ptr::addr_of_mut!(__slate_storage_1898) as *mut *mut Parse;
    let mut __slate_storage_1358: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1358: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1358) as *mut i32;
    let mut __slate_storage_1357: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1357: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1357) as *mut i32;
    let mut __slate_storage_1356: std::mem::MaybeUninit<*mut Index> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1356: *mut *mut Index =
        std::ptr::addr_of_mut!(__slate_storage_1356) as *mut *mut Index;
    let mut __slate_storage_1352: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1352: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1352) as *mut i32;
    let mut __slate_storage_1351: std::mem::MaybeUninit<*const i8> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1351: *mut *const i8 =
        std::ptr::addr_of_mut!(__slate_storage_1351) as *mut *const i8;
    let mut __slate_storage_1895: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1895: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1895) as *mut i32;
    let mut __slate_storage_1894: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1894: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1894) as *mut i32;
    let mut __slate_storage_1355: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1355: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1355) as *mut i32;
    let mut __slate_storage_1354: std::mem::MaybeUninit<u64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1354: *mut u64 = std::ptr::addr_of_mut!(__slate_storage_1354) as *mut u64;
    let mut __slate_storage_1353: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1353: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1353) as *mut i32;
    let mut __slate_storage_1350: std::mem::MaybeUninit<*mut SrcItem> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1350: *mut *mut SrcItem =
        std::ptr::addr_of_mut!(__slate_storage_1350) as *mut *mut SrcItem;
    let mut __slate_storage_1349: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1349: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1349) as *mut i32;
    let mut __slate_storage_1348: std::mem::MaybeUninit<*mut Table> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1348: *mut *mut Table =
        std::ptr::addr_of_mut!(__slate_storage_1348) as *mut *mut Table;
    let mut __slate_storage_1889: std::mem::MaybeUninit<*mut WhereLevel> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1889: *mut *mut WhereLevel =
        std::ptr::addr_of_mut!(__slate_storage_1889) as *mut *mut WhereLevel;
    let mut __slate_storage_1347: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1347: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1347) as *mut i32;
    let mut __slate_storage_1346: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1346: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1346) as *mut i32;
    let mut __slate_storage_1888: std::mem::MaybeUninit<i16> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1888: *mut i16 = std::ptr::addr_of_mut!(__slate_storage_1888) as *mut i16;
    let mut __slate_storage_1887: std::mem::MaybeUninit<i16> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1887: *mut i16 = std::ptr::addr_of_mut!(__slate_storage_1887) as *mut i16;
    let mut __slate_storage_1886: std::mem::MaybeUninit<*mut Parse> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1886: *mut *mut Parse =
        std::ptr::addr_of_mut!(__slate_storage_1886) as *mut *mut Parse;
    let mut __slate_storage_1885: std::mem::MaybeUninit<i16> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1885: *mut i16 = std::ptr::addr_of_mut!(__slate_storage_1885) as *mut i16;
    let mut __slate_storage_1884: std::mem::MaybeUninit<i16> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1884: *mut i16 = std::ptr::addr_of_mut!(__slate_storage_1884) as *mut i16;
    let mut __slate_storage_1883: std::mem::MaybeUninit<*mut WhereInfo> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1883: *mut *mut WhereInfo =
        std::ptr::addr_of_mut!(__slate_storage_1883) as *mut *mut WhereInfo;
    let mut __slate_storage_1882: std::mem::MaybeUninit<bool> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1882: *mut bool = std::ptr::addr_of_mut!(__slate_storage_1882) as *mut bool;
    let mut __slate_storage_1878: std::mem::MaybeUninit<u16> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1878: *mut u16 = std::ptr::addr_of_mut!(__slate_storage_1878) as *mut u16;
    let mut __slate_storage_1877: std::mem::MaybeUninit<u16> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1877: *mut u16 = std::ptr::addr_of_mut!(__slate_storage_1877) as *mut u16;
    let mut __slate_storage_1876: std::mem::MaybeUninit<*mut WhereInfo> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1876: *mut *mut WhereInfo =
        std::ptr::addr_of_mut!(__slate_storage_1876) as *mut *mut WhereInfo;
    let mut __slate_storage_1875: std::mem::MaybeUninit<u16> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1875: *mut u16 = std::ptr::addr_of_mut!(__slate_storage_1875) as *mut u16;
    let mut __slate_storage_1874: std::mem::MaybeUninit<u16> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1874: *mut u16 = std::ptr::addr_of_mut!(__slate_storage_1874) as *mut u16;
    let mut __slate_storage_1881: std::mem::MaybeUninit<u16> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1881: *mut u16 = std::ptr::addr_of_mut!(__slate_storage_1881) as *mut u16;
    let mut __slate_storage_1880: std::mem::MaybeUninit<u16> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1880: *mut u16 = std::ptr::addr_of_mut!(__slate_storage_1880) as *mut u16;
    let mut __slate_storage_1879: std::mem::MaybeUninit<*mut WhereInfo> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1879: *mut *mut WhereInfo =
        std::ptr::addr_of_mut!(__slate_storage_1879) as *mut *mut WhereInfo;
    let mut __slate_storage_1868: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1868: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1868) as *mut i32;
    let mut __slate_storage_1867: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1867: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1867) as *mut i32;
    let mut __slate_storage_1873: std::mem::MaybeUninit<u16> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1873: *mut u16 = std::ptr::addr_of_mut!(__slate_storage_1873) as *mut u16;
    let mut __slate_storage_1872: std::mem::MaybeUninit<u16> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1872: *mut u16 = std::ptr::addr_of_mut!(__slate_storage_1872) as *mut u16;
    let mut __slate_storage_1871: std::mem::MaybeUninit<*mut WhereTerm> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1871: *mut *mut WhereTerm =
        std::ptr::addr_of_mut!(__slate_storage_1871) as *mut *mut WhereTerm;
    let mut __slate_storage_1870: std::mem::MaybeUninit<bool> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1870: *mut bool = std::ptr::addr_of_mut!(__slate_storage_1870) as *mut bool;
    let mut __slate_storage_1869: std::mem::MaybeUninit<bool> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1869: *mut bool = std::ptr::addr_of_mut!(__slate_storage_1869) as *mut bool;
    let mut __slate_storage_1345: std::mem::MaybeUninit<*mut Expr> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1345: *mut *mut Expr =
        std::ptr::addr_of_mut!(__slate_storage_1345) as *mut *mut Expr;
    let mut __slate_storage_1344: std::mem::MaybeUninit<*mut WhereTerm> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1344: *mut *mut WhereTerm =
        std::ptr::addr_of_mut!(__slate_storage_1344) as *mut *mut WhereTerm;
    let mut __slate_storage_1866: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1866: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1866) as *mut i32;
    let mut __slate_storage_1865: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1865: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1865) as *mut i32;
    let mut __slate_storage_1864: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1864: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1864) as *mut i32;
    let mut __slate_storage_1863: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1863: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1863) as *mut i32;
    let mut __slate_storage_1862: std::mem::MaybeUninit<u16> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1862: *mut u16 = std::ptr::addr_of_mut!(__slate_storage_1862) as *mut u16;
    let mut __slate_storage_1861: std::mem::MaybeUninit<u16> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1861: *mut u16 = std::ptr::addr_of_mut!(__slate_storage_1861) as *mut u16;
    let mut __slate_storage_1860: std::mem::MaybeUninit<u16> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1860: *mut u16 = std::ptr::addr_of_mut!(__slate_storage_1860) as *mut u16;
    let mut __slate_storage_1859: std::mem::MaybeUninit<u16> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1859: *mut u16 = std::ptr::addr_of_mut!(__slate_storage_1859) as *mut u16;
    let mut __slate_storage_1343: std::mem::MaybeUninit<u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1343: *mut u8 = std::ptr::addr_of_mut!(__slate_storage_1343) as *mut u8;
    let mut __slate_storage_1342: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1342: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1342) as *mut i32;
    let mut __slate_storage_1341: std::mem::MaybeUninit<*mut sqlite3> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1341: *mut *mut sqlite3 =
        std::ptr::addr_of_mut!(__slate_storage_1341) as *mut *mut sqlite3;
    let mut __slate_storage_1340: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1340: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1340) as *mut i32;
    let mut __slate_storage_1339: std::mem::MaybeUninit<*mut WhereLoop> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1339: *mut *mut WhereLoop =
        std::ptr::addr_of_mut!(__slate_storage_1339) as *mut *mut WhereLoop;
    let mut __slate_storage_1338: std::mem::MaybeUninit<*mut WhereLevel> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1338: *mut *mut WhereLevel =
        std::ptr::addr_of_mut!(__slate_storage_1338) as *mut *mut WhereLevel;
    let mut __slate_storage_1337: std::mem::MaybeUninit<*mut WhereMaskSet> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1337: *mut *mut WhereMaskSet =
        std::ptr::addr_of_mut!(__slate_storage_1337) as *mut *mut WhereMaskSet;
    let mut __slate_storage_1336: std::mem::MaybeUninit<WhereLoopBuilder> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1336: *mut WhereLoopBuilder =
        std::ptr::addr_of_mut!(__slate_storage_1336) as *mut WhereLoopBuilder;
    let mut __slate_storage_1335: std::mem::MaybeUninit<u64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1335: *mut u64 = std::ptr::addr_of_mut!(__slate_storage_1335) as *mut u64;
    let mut __slate_storage_1334: std::mem::MaybeUninit<*mut Vdbe> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1334: *mut *mut Vdbe =
        std::ptr::addr_of_mut!(__slate_storage_1334) as *mut *mut Vdbe;
    let mut __slate_storage_1333: std::mem::MaybeUninit<*mut WhereInfo> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1333: *mut *mut WhereInfo =
        std::ptr::addr_of_mut!(__slate_storage_1333) as *mut *mut WhereInfo;
    let mut __slate_storage_1332: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1332: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1332) as *mut i32;
    let mut __slate_storage_1331: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1331: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1331) as *mut i32;
    unsafe {
        '__join_137: {
            // /* Num. bytes allocated for WhereInfo struct */
            // /* Number of elements in pTabList */
            // /* Will become the return value of this function */
            // /* The virtual database engine */
            std::ptr::write(__slate_slot_1334, unsafe { (*pParse).pVdbe });
            // /* Cursors that are not yet positioned */
            // /* The WhereLoop builder */
            // /* The expression mask set */
            // /* A single level in pWInfo->a[] */
            // /* Pointer to a single WhereLoop object */
            // /* Loop counter */
            // /* Database connection */
            // /* Return code */
            // /* OPFLAG_FORDELETE or zero, as appropriate */
            std::ptr::write(__slate_slot_1343, ((0 as i32) as i8) as u8);
            0 as i32;
            // /* Only one of WHERE_OR_SUBCLAUSE or WHERE_USE_LIMIT */
            0 as i32;
            // /* Variable initialization */
            *__slate_slot_1341 = unsafe { (*pParse).db };
            unsafe {
                memset(
                    std::ptr::addr_of_mut!(*__slate_slot_1336) as *mut (),
                    0 as i32,
                    40 as u64,
                )
            };
            // /* An ORDER/GROUP BY clause of more than 63 terms cannot be optimized */
            {}
            if pOrderBy != std::ptr::null_mut::<ExprList>()
                && (unsafe { (*pOrderBy).nExpr })
                    >= (((8 as u64).wrapping_mul(((8 as i32) as i64) as u64) as u32) as i32)
            {
                pOrderBy = std::ptr::null_mut::<ExprList>();
                std::ptr::write(__slate_slot_1859, wctrlFlags);
                std::ptr::write(
                    __slate_slot_1860,
                    ((((*__slate_slot_1859 as u32) as i32) & !(256 as i32)) as i16) as u16,
                );
                wctrlFlags = *__slate_slot_1860;
                // /* Disable omit-noop-join opt */
                std::ptr::write(__slate_slot_1861, wctrlFlags);
                std::ptr::write(
                    __slate_slot_1862,
                    ((((*__slate_slot_1861 as u32) as i32) | (8192 as i32)) as i16) as u16,
                );
                wctrlFlags = *__slate_slot_1862;
            }
        }
        // /* The number of tables in the FROM clause is limited by the number of
        //   ** bits in a Bitmask
        //   */
        {}
        if (unsafe { (*pTabList).nSrc })
            > (((8 as u64).wrapping_mul(((8 as i32) as i64) as u64) as u32) as i32)
        {
            unsafe {
                sqlite3ErrorMsg(
                    pParse,
                    (b"at most %d tables in a join\0".as_ptr() as *mut i8) as *const i8,
                    ((8 as u64).wrapping_mul(((8 as i32) as i64) as u64) as u32) as i32,
                )
            };
            return std::ptr::null_mut::<WhereInfo>();
        } else {
            '__join_2: {
                // /* This function normally generates a nested loop for all tables in
                //   ** pTabList.  But if the WHERE_OR_SUBCLAUSE flag is set, then we should
                //   ** only generate code for the first table in pTabList and assume that
                //   ** any cursors associated with subsequent tables are uninitialized.
                //   */
                *__slate_slot_1332 = if ((wctrlFlags as u32) as i32) & (32 as i32) != (0 as i32) {
                    1 as i32
                } else {
                    unsafe { (*pTabList).nSrc }
                };
                // /* Allocate and initialize the WhereInfo structure that will become the
                //   ** return value. A single allocation is used to store the WhereInfo
                //   ** struct, the contents of WhereInfo.a[], the WhereClause structure
                //   ** and the WhereMaskSet structure. Since WhereClause contains an 8-byte
                //   ** field (type Bitmask) it must be aligned on an 8-byte boundary on
                //   ** some architectures. Hence the ROUND8() below.
                //   */
                *__slate_slot_1331 = (((856 as u64)
                    .wrapping_add(((*__slate_slot_1332 as i64) as u64).wrapping_mul(120 as u64))
                    .wrapping_add(((7 as i32) as i64) as u64)
                    & ((!(7 as i32) as i64) as u64)) as u32)
                    as i32;
                *__slate_slot_1333 = (unsafe {
                    sqlite3DbMallocRawNN(
                        *__slate_slot_1341,
                        ((*__slate_slot_1331 as i64) as u64).wrapping_add(104 as u64),
                    )
                }) as *mut WhereInfo;
                if (unsafe { (*(*__slate_slot_1341)).mallocFailed }) != (0 as u8) {
                    unsafe { sqlite3DbFree(*__slate_slot_1341, *__slate_slot_1333 as *mut ()) };
                    *__slate_slot_1333 = std::ptr::null_mut::<WhereInfo>();
                } else {
                    '__join_124: {
                        unsafe {
                            (*(*__slate_slot_1333)).pParse = pParse;
                        }
                        unsafe {
                            (*(*__slate_slot_1333)).pTabList = pTabList;
                        }
                        unsafe {
                            (*(*__slate_slot_1333)).pOrderBy = pOrderBy;
                        }
                        unsafe {
                            (*(*__slate_slot_1333)).pResultSet = pResultSet;
                        }
                        std::ptr::write(__slate_slot_1863, -(1 as i32));
                        unsafe {
                            *unsafe {
                                unsafe {
                                    (*(*__slate_slot_1333)).aiCurOnePass.as_mut_ptr() as *mut i32
                                }
                                .offset((1 as i32) as isize)
                            } = *__slate_slot_1863;
                        }
                        unsafe {
                            *unsafe {
                                unsafe {
                                    (*(*__slate_slot_1333)).aiCurOnePass.as_mut_ptr() as *mut i32
                                }
                                .offset((0 as i32) as isize)
                            } = *__slate_slot_1863;
                        }
                        unsafe {
                            (*(*__slate_slot_1333)).nLevel = (*__slate_slot_1332 as i8) as u8;
                        }
                        std::ptr::write(__slate_slot_1864, unsafe { sqlite3VdbeMakeLabel(pParse) });
                        unsafe {
                            (*(*__slate_slot_1333)).iContinue = *__slate_slot_1864;
                        }
                        unsafe {
                            (*(*__slate_slot_1333)).iBreak = *__slate_slot_1864;
                        }
                        unsafe {
                            (*(*__slate_slot_1333)).wctrlFlags = wctrlFlags;
                        }
                        unsafe {
                            (*(*__slate_slot_1333)).iLimit = iAuxArg as i16;
                        }
                        unsafe {
                            (*(*__slate_slot_1333)).savedNQueryLoop =
                                (unsafe { (*pParse).nQueryLoop }) as i32;
                        }
                        unsafe {
                            (*(*__slate_slot_1333)).pSelect = pSelect;
                        }
                        unsafe {
                            memset(
                                (unsafe { std::ptr::addr_of_mut!((*(*__slate_slot_1333)).nOBSat) })
                                    as *mut (),
                                0 as i32,
                                (104 as u64).wrapping_sub(65 as u64),
                            )
                        };
                        unsafe {
                            memset(
                                (unsafe {
                                    unsafe {
                                        std::ptr::addr_of_mut!((*(*__slate_slot_1333)).a)
                                            as *mut WhereLevel
                                    }
                                    .offset((0 as i32) as isize)
                                }) as *mut (),
                                0 as i32,
                                (104 as u64).wrapping_add(
                                    ((*__slate_slot_1332 as i64) as u64).wrapping_mul(120 as u64),
                                ),
                            )
                        };
                        // /* ONEPASS defaults to OFF */
                        0 as i32;
                        *__slate_slot_1337 =
                            unsafe { std::ptr::addr_of_mut!((*(*__slate_slot_1333)).sMaskSet) };
                        unsafe {
                            (*(*__slate_slot_1337)).n = 0 as i32;
                        }
                        // /* Initialize ix[0] to a value that can never be
                        //                          ** a valid cursor number, to avoid an initial
                        //                          ** test for pMaskSet->n==0 in sqlite3WhereGetMask() */
                        unsafe {
                            *unsafe {
                                unsafe { (*(*__slate_slot_1337)).ix.as_mut_ptr() as *mut i32 }
                                    .offset((0 as i32) as isize)
                            } = -(99 as i32);
                        }
                        (*__slate_slot_1336).pWInfo = *__slate_slot_1333;
                        (*__slate_slot_1336).pWC =
                            unsafe { std::ptr::addr_of_mut!((*(*__slate_slot_1333)).sWC) };
                        (*__slate_slot_1336).pNew = (unsafe {
                            (*__slate_slot_1333 as *mut i8).offset(*__slate_slot_1331 as isize)
                        }) as *mut WhereLoop;
                        0 as i32;
                        whereLoopInit((*__slate_slot_1336).pNew);
                        // /* Split the WHERE clause into separate subexpressions where each
                        //   ** subexpression is separated by an AND operator.
                        //   */
                        unsafe {
                            sqlite3WhereClauseInit(
                                unsafe { std::ptr::addr_of_mut!((*(*__slate_slot_1333)).sWC) },
                                *__slate_slot_1333,
                            )
                        };
                        unsafe {
                            sqlite3WhereSplit(
                                unsafe { std::ptr::addr_of_mut!((*(*__slate_slot_1333)).sWC) },
                                pWhere,
                                ((44 as i32) as i8) as u8,
                            )
                        };
                        // /* Special case: No FROM clause
                        //   */
                        if *__slate_slot_1332 == (0 as i32) {
                            if pOrderBy != std::ptr::null_mut::<ExprList>() {
                                unsafe {
                                    (*(*__slate_slot_1333)).nOBSat =
                                        (unsafe { (*pOrderBy).nExpr }) as i8;
                                }
                            }
                            if ((wctrlFlags as u32) as i32) & (256 as i32) != (0 as i32)
                                && (unsafe { (*(*__slate_slot_1341)).dbOptFlags })
                                    & ((16 as i32) as u32)
                                    == ((0 as i32) as u32)
                            {
                                unsafe {
                                    (*(*__slate_slot_1333)).eDistinct = ((1 as i32) as i8) as u8;
                                }
                            }
                            if (unsafe { (*(*__slate_slot_1333)).pSelect })
                                != std::ptr::null_mut::<Select>()
                                && (unsafe {
                                    (*unsafe { (*(*__slate_slot_1333)).pSelect }).selFlags
                                }) & ((1024 as i32) as u32)
                                    == ((0 as i32) as u32)
                            {
                                unsafe {
                                    sqlite3VdbeExplain(
                                        pParse,
                                        ((0 as i32) as i8) as u8,
                                        (b"SCAN CONSTANT ROW\0".as_ptr() as *mut i8) as *const i8,
                                    )
                                };
                            }
                        } else {
                            // /* Assign a bit from the bitmask to every term in the FROM clause.
                            //     **
                            //     ** The N-th term of the FROM clause is assigned a bitmask of 1<<N.
                            //     **
                            //     ** The rule of the previous sentence ensures that if X is the bitmask for
                            //     ** a table T, then X-1 is the bitmask for all other tables to the left of T.
                            //     ** Knowing the bitmask for all tables to the left of a left join is
                            //     ** important.  Ticket #3015.
                            //     **
                            //     ** Note that bitmasks are created for all pTabList->nSrc tables in
                            //     ** pTabList, not just the first nTabList tables.  nTabList is normally
                            //     ** equal to pTabList->nSrc but might be shortened to 1 if the
                            //     ** WHERE_OR_SUBCLAUSE flag is set.
                            //     */
                            *__slate_slot_1340 = 0 as i32;
                            loop {
                                createMask(*__slate_slot_1337, unsafe {
                                    (*unsafe {
                                        unsafe {
                                            std::ptr::addr_of_mut!((*pTabList).a) as *mut SrcItem
                                        }
                                        .offset(*__slate_slot_1340 as isize)
                                    })
                                    .iCursor
                                });
                                unsafe {
                                    sqlite3WhereTabFuncArgs(
                                        pParse,
                                        unsafe {
                                            unsafe {
                                                std::ptr::addr_of_mut!((*pTabList).a)
                                                    as *mut SrcItem
                                            }
                                            .offset(*__slate_slot_1340 as isize)
                                        },
                                        unsafe {
                                            std::ptr::addr_of_mut!((*(*__slate_slot_1333)).sWC)
                                        },
                                    )
                                };
                                std::ptr::write(__slate_slot_1865, *__slate_slot_1340);
                                std::ptr::write(__slate_slot_1866, *__slate_slot_1865 + (1 as i32));
                                *__slate_slot_1340 = *__slate_slot_1866;
                                if !(*__slate_slot_1866 < unsafe { (*pTabList).nSrc }) {
                                    break '__join_124;
                                }
                            }
                        }
                    }
                    // /* Analyze all of the subexpressions. */
                    unsafe {
                        sqlite3WhereExprAnalyze(pTabList, unsafe {
                            std::ptr::addr_of_mut!((*(*__slate_slot_1333)).sWC)
                        })
                    };
                    if pSelect != std::ptr::null_mut::<Select>()
                        && (unsafe { (*pSelect).pLimit }) != std::ptr::null_mut::<Expr>()
                    {
                        unsafe {
                            sqlite3WhereAddLimit(
                                unsafe { std::ptr::addr_of_mut!((*(*__slate_slot_1333)).sWC) },
                                pSelect,
                            )
                        };
                    }
                    if (unsafe { (*pParse).nErr }) != (0 as i32) {
                    } else {
                        // /* The False-WHERE-Term-Bypass optimization:
                        //   **
                        //   ** If there are WHERE terms that are false, then no rows will be output,
                        //   ** so skip over all of the code generated here.
                        //   **
                        //   ** Conditions:
                        //   **
                        //   **   (1)  The WHERE term must not refer to any tables in the join.
                        //   **   (2)  The term must not come from an ON clause on the
                        //   **        right-hand side of a LEFT or FULL JOIN.
                        //   **   (3)  The term must not come from an ON clause, or there must be
                        //   **        no RIGHT or FULL OUTER joins in pTabList.
                        //   **   (4)  If the expression contains non-deterministic functions
                        //   **        that are not within a sub-select. This is not required
                        //   **        for correctness but rather to preserves SQLite's legacy
                        //   **        behaviour in the following two cases:
                        //   **
                        //   **          WHERE random()>0;           -- eval random() once per row
                        //   **          WHERE (SELECT random())>0;  -- eval random() just once overall
                        //   **
                        //   ** Note that the Where term need not be a constant in order for this
                        //   ** optimization to apply, though it does need to be constant relative to
                        //   ** the current subquery (condition 1).  The term might include variables
                        //   ** from outer queries so that the value of the term changes from one
                        //   ** invocation of the current subquery to the next.
                        //   */
                        *__slate_slot_1340 = 0 as i32;
                        loop {
                            if *__slate_slot_1340 < unsafe { (*(*__slate_slot_1336).pWC).nBase } {
                                // /* A term of the WHERE clause */
                                std::ptr::write(__slate_slot_1344, unsafe {
                                    unsafe { (*(*__slate_slot_1336).pWC).a }
                                        .offset(*__slate_slot_1340 as isize)
                                });
                                // /* The expression of pT */
                                if (((unsafe { (*(*__slate_slot_1344)).wtFlags }) as u32) as i32)
                                    & (2 as i32)
                                    != (0 as i32)
                                {
                                } else {
                                    *__slate_slot_1345 = unsafe { (*(*__slate_slot_1344)).pExpr };
                                    0 as i32;
                                    0 as i32;
                                    // /* Conditions (1) and (2) */
                                    if (unsafe { (*(*__slate_slot_1344)).prereqAll })
                                        == (((0 as i32) as i64) as u64)
                                    {
                                        if *__slate_slot_1332 == (0 as i32) {
                                            *__slate_slot_1870 = true as bool;
                                        } else {
                                            *__slate_slot_1870 =
                                                exprIsDeterministic(*__slate_slot_1345)
                                                    != (0 as i32);
                                        }
                                        *__slate_slot_1869 = *__slate_slot_1870;
                                    } else {
                                        *__slate_slot_1869 = false as bool;
                                    }
                                    if *__slate_slot_1869
                                        && !((unsafe { (*(*__slate_slot_1345)).flags })
                                            & ((2 as i32) as u32)
                                            != ((0 as i32) as u32)
                                            && (((unsafe {
                                                (*unsafe {
                                                    unsafe {
                                                        std::ptr::addr_of_mut!((*pTabList).a)
                                                            as *mut SrcItem
                                                    }
                                                    .offset((0 as i32) as isize)
                                                })
                                                .fg
                                                .jointype
                                            })
                                                as u32)
                                                as i32)
                                                & (64 as i32)
                                                != (0 as i32))
                                    {
                                        unsafe {
                                            sqlite3ExprIfFalse(
                                                pParse,
                                                *__slate_slot_1345,
                                                unsafe { (*(*__slate_slot_1333)).iBreak },
                                                16 as i32,
                                            )
                                        };
                                        std::ptr::write(__slate_slot_1871, *__slate_slot_1344);
                                        std::ptr::write(__slate_slot_1872, unsafe {
                                            (*(*__slate_slot_1871)).wtFlags
                                        });
                                        std::ptr::write(
                                            __slate_slot_1873,
                                            ((((*__slate_slot_1872 as u32) as i32) | (4 as i32))
                                                as i16)
                                                as u16,
                                        );
                                        unsafe {
                                            (*(*__slate_slot_1871)).wtFlags = *__slate_slot_1873;
                                        }
                                    }
                                    // /* Condition (4) */
                                    // /* Condition (3) */
                                }
                                std::ptr::write(__slate_slot_1867, *__slate_slot_1340);
                                std::ptr::write(__slate_slot_1868, *__slate_slot_1867 + (1 as i32));
                                *__slate_slot_1340 = *__slate_slot_1868;
                            } else {
                                break;
                            }
                        }
                        if ((wctrlFlags as u32) as i32) & (256 as i32) != (0 as i32) {
                            if (unsafe { (*(*__slate_slot_1341)).dbOptFlags })
                                & ((16 as i32) as u32)
                                != ((0 as i32) as u32)
                            {
                                // /* Disable the DISTINCT optimization if SQLITE_DistinctOpt is set via
                                //       ** sqlite3_test_ctrl(SQLITE_TESTCTRL_OPTIMIZATIONS,...) */
                                std::ptr::write(__slate_slot_1874, wctrlFlags);
                                std::ptr::write(
                                    __slate_slot_1875,
                                    ((((*__slate_slot_1874 as u32) as i32) & !(256 as i32)) as i16)
                                        as u16,
                                );
                                wctrlFlags = *__slate_slot_1875;
                                std::ptr::write(__slate_slot_1876, *__slate_slot_1333);
                                std::ptr::write(__slate_slot_1877, unsafe {
                                    (*(*__slate_slot_1876)).wctrlFlags
                                });
                                std::ptr::write(
                                    __slate_slot_1878,
                                    ((((*__slate_slot_1877 as u32) as i32) & !(256 as i32)) as i16)
                                        as u16,
                                );
                                unsafe {
                                    (*(*__slate_slot_1876)).wctrlFlags = *__slate_slot_1878;
                                }
                            } else {
                                if isDistinctRedundant(
                                    pParse,
                                    pTabList,
                                    unsafe { std::ptr::addr_of_mut!((*(*__slate_slot_1333)).sWC) },
                                    pResultSet,
                                ) != (0 as i32)
                                {
                                    // /* The DISTINCT marking is pointless.  Ignore it. */
                                    unsafe {
                                        (*(*__slate_slot_1333)).eDistinct =
                                            ((1 as i32) as i8) as u8;
                                    }
                                } else {
                                    if pOrderBy == std::ptr::null_mut::<ExprList>() {
                                        // /* Try to ORDER BY the result set to make distinct processing easier */
                                        std::ptr::write(__slate_slot_1879, *__slate_slot_1333);
                                        std::ptr::write(__slate_slot_1880, unsafe {
                                            (*(*__slate_slot_1879)).wctrlFlags
                                        });
                                        std::ptr::write(
                                            __slate_slot_1881,
                                            ((((*__slate_slot_1880 as u32) as i32) | (128 as i32))
                                                as i16)
                                                as u16,
                                        );
                                        unsafe {
                                            (*(*__slate_slot_1879)).wctrlFlags = *__slate_slot_1881;
                                        }
                                        unsafe {
                                            (*(*__slate_slot_1333)).pOrderBy = pResultSet;
                                        }
                                    }
                                }
                            }
                        }
                        // /* Construct the WhereLoop objects */
                        if *__slate_slot_1332 != (1 as i32) {
                            *__slate_slot_1882 = true as bool;
                        } else {
                            *__slate_slot_1882 =
                                whereShortCut(std::ptr::addr_of_mut!(*__slate_slot_1336))
                                    == (0 as i32);
                        }
                        if *__slate_slot_1882 {
                            *__slate_slot_1342 =
                                whereLoopAddAll(std::ptr::addr_of_mut!(*__slate_slot_1336));
                            if *__slate_slot_1342 != (0 as i32) {
                                break '__join_2;
                            } else {
                                {}
                                wherePathSolver(*__slate_slot_1333, (0 as i32) as i16);
                                if (unsafe { (*(*__slate_slot_1341)).mallocFailed }) != (0 as u8) {
                                    break '__join_2;
                                } else {
                                    if (unsafe { (*(*__slate_slot_1333)).pOrderBy })
                                        != std::ptr::null_mut::<ExprList>()
                                    {
                                        whereInterstageHeuristic(*__slate_slot_1333);
                                        wherePathSolver(
                                            *__slate_slot_1333,
                                            (if ((unsafe { (*(*__slate_slot_1333)).nRowOut })
                                                as i32)
                                                < (0 as i32)
                                            {
                                                1 as i32
                                            } else {
                                                ((unsafe { (*(*__slate_slot_1333)).nRowOut })
                                                    as i32)
                                                    + (1 as i32)
                                            }) as i16,
                                        );
                                        if (unsafe { (*(*__slate_slot_1341)).mallocFailed })
                                            != (0 as u8)
                                        {
                                            break '__join_2;
                                        }
                                    }
                                    // /* TUNING:  Assume that a DISTINCT clause on a subquery reduces
                                    //     ** the output size by a factor of 8 (LogEst -30).  Search for
                                    //     ** tag-20250414a to see other cases.
                                    //     */
                                    if (((unsafe { (*(*__slate_slot_1333)).wctrlFlags }) as u32)
                                        as i32)
                                        & (256 as i32)
                                        != (0 as i32)
                                    {
                                        {}
                                        std::ptr::write(__slate_slot_1883, *__slate_slot_1333);
                                        std::ptr::write(__slate_slot_1884, unsafe {
                                            (*(*__slate_slot_1883)).nRowOut
                                        });
                                        std::ptr::write(
                                            __slate_slot_1885,
                                            ((*__slate_slot_1884 as i32) - (30 as i32)) as i16,
                                        );
                                        unsafe {
                                            (*(*__slate_slot_1883)).nRowOut = *__slate_slot_1885;
                                        }
                                    }
                                }
                            }
                        }
                        0 as i32;
                        if (unsafe { (*(*__slate_slot_1333)).pOrderBy })
                            == std::ptr::null_mut::<ExprList>()
                            && (unsafe { (*(*__slate_slot_1341)).flags })
                                & (((4096 as i32) as i64) as u64)
                                != (((0 as i32) as i64) as u64)
                        {
                            whereReverseScanOrder(*__slate_slot_1333);
                        }
                        if (unsafe { (*pParse).nErr }) != (0 as i32) {
                        } else {
                            0 as i32;
                            // /* Attempt to omit tables from a join that do not affect the result.
                            //   ** See the comment on whereOmitNoopJoin() for further information.
                            //   **
                            //   ** This query optimization is factored out into a separate "no-inline"
                            //   ** procedure to keep the sqlite3WhereBegin() procedure from becoming
                            //   ** too large.  If sqlite3WhereBegin() becomes too large, that prevents
                            //   ** some C-compiler optimizers from in-lining the
                            //   ** sqlite3WhereCodeOneLoopStart() procedure, and it is important to
                            //   ** in-line sqlite3WhereCodeOneLoopStart() for performance reasons.
                            //   */
                            *__slate_slot_1335 = !(((0 as i32) as i64) as u64);
                            // /* Must be a join, or this opt8n is pointless */
                            if (((unsafe { (*(*__slate_slot_1333)).nLevel }) as u32) as i32)
                                >= (2 as i32)
                                && pResultSet != std::ptr::null_mut::<ExprList>()
                                && (0 as i32)
                                    == ((wctrlFlags as u32) as i32)
                                        & ((1024 as i32) | (8192 as i32))
                                && (unsafe { (*(*__slate_slot_1341)).dbOptFlags })
                                    & ((256 as i32) as u32)
                                    == ((0 as i32) as u32)
                            {
                                *__slate_slot_1335 =
                                    whereOmitNoopJoin(*__slate_slot_1333, *__slate_slot_1335);
                                *__slate_slot_1332 =
                                    ((unsafe { (*(*__slate_slot_1333)).nLevel }) as u32) as i32;
                                0 as i32;
                            }
                            // /* Condition (1) */
                            // /* (1),(6) */
                            // /* (7) */
                            // /* Check to see if there are any SEARCH loops that might benefit from
                            //   ** using a Bloom filter.
                            //   */
                            if (((unsafe { (*(*__slate_slot_1333)).nLevel }) as u32) as i32)
                                >= (2 as i32)
                                && (unsafe { (*(*__slate_slot_1341)).dbOptFlags })
                                    & ((524288 as i32) as u32)
                                    == ((0 as i32) as u32)
                            {
                                whereCheckIfBloomFilterIsUseful(
                                    *__slate_slot_1333 as *const WhereInfo,
                                );
                            }
                            std::ptr::write(__slate_slot_1886, unsafe {
                                (*(*__slate_slot_1333)).pParse
                            });
                            std::ptr::write(__slate_slot_1887, unsafe {
                                (*(*__slate_slot_1886)).nQueryLoop
                            });
                            std::ptr::write(
                                __slate_slot_1888,
                                ((*__slate_slot_1887 as i32)
                                    + ((unsafe { (*(*__slate_slot_1333)).nRowOut }) as i32))
                                    as i16,
                            );
                            unsafe {
                                (*(*__slate_slot_1886)).nQueryLoop = *__slate_slot_1888;
                            }
                            // /* If the caller is an UPDATE or DELETE statement that is requesting
                            //   ** to use a one-pass algorithm, determine if this is appropriate.
                            //   **
                            //   ** A one-pass approach can be used if the caller has requested one
                            //   ** and either (a) the scan visits at most one row or (b) each
                            //   ** of the following are true:
                            //   **
                            //   **   * the caller has indicated that a one-pass approach can be used
                            //   **     with multiple rows (by setting WHERE_ONEPASS_MULTIROW), and
                            //   **   * the table is not a virtual table, and
                            //   **   * either the scan does not use the OR optimization or the caller
                            //   **     is a DELETE operation (WHERE_DUPLICATES_OK is only specified
                            //   **     for DELETE).
                            //   **
                            //   ** The last qualification is because an UPDATE statement uses
                            //   ** WhereInfo.aiCurOnePass[1] to determine whether or not it really can
                            //   ** use a one-pass approach, and this is not set accurately for scans
                            //   ** that use the OR optimization.
                            //   */
                            0 as i32;
                            if ((wctrlFlags as u32) as i32) & (4 as i32) != (0 as i32) {
                                std::ptr::write(
                                    __slate_slot_1346,
                                    (unsafe {
                                        (*unsafe {
                                            (*unsafe {
                                                unsafe {
                                                    std::ptr::addr_of_mut!(
                                                        (*(*__slate_slot_1333)).a
                                                    )
                                                        as *mut WhereLevel
                                                }
                                                .offset((0 as i32) as isize)
                                            })
                                            .pWLoop
                                        })
                                        .wsFlags
                                    }) as i32,
                                );
                                std::ptr::write(
                                    __slate_slot_1347,
                                    (*__slate_slot_1346 & (4096 as i32) != (0 as i32)) as i32,
                                );
                                0 as i32;
                                if *__slate_slot_1347 != (0 as i32)
                                    || (0 as i32) != ((wctrlFlags as u32) as i32) & (8 as i32)
                                        && !((((unsafe {
                                            (*unsafe {
                                                (*unsafe {
                                                    unsafe {
                                                        std::ptr::addr_of_mut!((*pTabList).a)
                                                            as *mut SrcItem
                                                    }
                                                    .offset((0 as i32) as isize)
                                                })
                                                .pSTab
                                            })
                                            .eTabType
                                        }) as u32)
                                            as i32)
                                            == (1 as i32))
                                        && ((0 as i32) == *__slate_slot_1346 & (8192 as i32)
                                            || ((wctrlFlags as u32) as i32) & (16 as i32)
                                                != (0 as i32))
                                        && (unsafe { (*(*__slate_slot_1341)).dbOptFlags })
                                            & ((134217728 as i32) as u32)
                                            == ((0 as i32) as u32)
                                {
                                    unsafe {
                                        (*(*__slate_slot_1333)).eOnePass =
                                            ((if *__slate_slot_1347 != (0 as i32) {
                                                1 as i32
                                            } else {
                                                2 as i32
                                            }) as i8)
                                                as u8;
                                    }
                                    if (unsafe {
                                        (*unsafe {
                                            (*unsafe {
                                                unsafe {
                                                    std::ptr::addr_of_mut!((*pTabList).a)
                                                        as *mut SrcItem
                                                }
                                                .offset((0 as i32) as isize)
                                            })
                                            .pSTab
                                        })
                                        .tabFlags
                                    }) & ((128 as i32) as u32)
                                        == ((0 as i32) as u32)
                                        && *__slate_slot_1346 & (64 as i32) != (0 as i32)
                                    {
                                        if ((wctrlFlags as u32) as i32) & (8 as i32) != (0 as i32) {
                                            *__slate_slot_1343 = ((8 as i32) as i8) as u8;
                                        }
                                        unsafe {
                                            (*unsafe {
                                                (*unsafe {
                                                    unsafe {
                                                        std::ptr::addr_of_mut!(
                                                            (*(*__slate_slot_1333)).a
                                                        )
                                                            as *mut WhereLevel
                                                    }
                                                    .offset((0 as i32) as isize)
                                                })
                                                .pWLoop
                                            })
                                            .wsFlags = (*__slate_slot_1346 & !(64 as i32)) as u32;
                                        }
                                    }
                                }
                            }
                            // /* Open all tables in the pTabList and any indices selected for
                            //   ** searching those tables.
                            //   */
                            *__slate_slot_1340 = 0 as i32;
                            std::ptr::write(__slate_slot_1889, unsafe {
                                std::ptr::addr_of_mut!((*(*__slate_slot_1333)).a) as *mut WhereLevel
                            });
                            *__slate_slot_1338 = *__slate_slot_1889;
                            loop {
                                if *__slate_slot_1340 < *__slate_slot_1332 {
                                    // /* Table to open */
                                    // /* Index of database containing table/index */
                                    *__slate_slot_1350 = unsafe {
                                        unsafe {
                                            std::ptr::addr_of_mut!((*pTabList).a) as *mut SrcItem
                                        }
                                        .offset(
                                            (((unsafe { (*(*__slate_slot_1338)).iFrom }) as u32)
                                                as i32)
                                                as isize,
                                        )
                                    };
                                    *__slate_slot_1348 = unsafe { (*(*__slate_slot_1350)).pSTab };
                                    *__slate_slot_1349 = unsafe {
                                        sqlite3SchemaToIndex(*__slate_slot_1341, unsafe {
                                            (*(*__slate_slot_1348)).pSchema
                                        })
                                    };
                                    *__slate_slot_1339 = unsafe { (*(*__slate_slot_1338)).pWLoop };
                                    unsafe {
                                        (*(*__slate_slot_1338)).addrBrk =
                                            unsafe { sqlite3VdbeMakeLabel(pParse) };
                                    }
                                    if *__slate_slot_1340 == (0 as i32)
                                        || (((unsafe {
                                            (*unsafe {
                                                (*__slate_slot_1350).offset((0 as i32) as isize)
                                            })
                                            .fg
                                            .jointype
                                        }) as u32)
                                            as i32)
                                            & (8 as i32)
                                            != (0 as i32)
                                    {
                                        unsafe {
                                            (*(*__slate_slot_1338)).addrHalt =
                                                unsafe { (*(*__slate_slot_1338)).addrBrk };
                                        }
                                    } else {
                                        if (unsafe {
                                            (*unsafe {
                                                unsafe {
                                                    std::ptr::addr_of_mut!(
                                                        (*(*__slate_slot_1333)).a
                                                    )
                                                        as *mut WhereLevel
                                                }
                                                .offset((*__slate_slot_1340 - (1 as i32)) as isize)
                                            })
                                            .pRJ
                                        }) != std::ptr::null_mut::<WhereRightJoin>()
                                        {
                                            unsafe {
                                                (*(*__slate_slot_1338)).addrHalt = unsafe {
                                                    (*unsafe {
                                                        unsafe {
                                                            std::ptr::addr_of_mut!(
                                                                (*(*__slate_slot_1333)).a
                                                            )
                                                                as *mut WhereLevel
                                                        }
                                                        .offset(
                                                            (*__slate_slot_1340 - (1 as i32))
                                                                as isize,
                                                        )
                                                    })
                                                    .addrBrk
                                                };
                                            }
                                        } else {
                                            unsafe {
                                                (*(*__slate_slot_1338)).addrHalt = unsafe {
                                                    (*unsafe {
                                                        unsafe {
                                                            std::ptr::addr_of_mut!(
                                                                (*(*__slate_slot_1333)).a
                                                            )
                                                                as *mut WhereLevel
                                                        }
                                                        .offset(
                                                            (*__slate_slot_1340 - (1 as i32))
                                                                as isize,
                                                        )
                                                    })
                                                    .addrHalt
                                                };
                                            }
                                        }
                                    }
                                    if (unsafe { (*(*__slate_slot_1348)).tabFlags })
                                        & ((16384 as i32) as u32)
                                        != ((0 as i32) as u32)
                                        || (((unsafe { (*(*__slate_slot_1348)).eTabType }) as u32)
                                            as i32)
                                            == (2 as i32)
                                    {
                                        // /* Do nothing */
                                    } else {
                                        if (unsafe { (*(*__slate_slot_1339)).wsFlags })
                                            & ((1024 as i32) as u32)
                                            != ((0 as i32) as u32)
                                        {
                                            std::ptr::write(
                                                __slate_slot_1351,
                                                (unsafe {
                                                    sqlite3GetVTable(
                                                        *__slate_slot_1341,
                                                        *__slate_slot_1348,
                                                    )
                                                })
                                                    as *const i8,
                                            );
                                            std::ptr::write(__slate_slot_1352, unsafe {
                                                (*(*__slate_slot_1350)).iCursor
                                            });
                                            unsafe {
                                                sqlite3VdbeAddOp4(
                                                    *__slate_slot_1334,
                                                    175 as i32,
                                                    *__slate_slot_1352,
                                                    0 as i32,
                                                    0 as i32,
                                                    *__slate_slot_1351,
                                                    -(12 as i32),
                                                )
                                            };
                                        } else {
                                            if (((unsafe { (*(*__slate_slot_1348)).eTabType })
                                                as u32)
                                                as i32)
                                                == (1 as i32)
                                            {
                                                // /* noop */
                                            } else {
                                                if (unsafe { (*(*__slate_slot_1339)).wsFlags })
                                                    & ((64 as i32) as u32)
                                                    == ((0 as i32) as u32)
                                                    && ((wctrlFlags as u32) as i32) & (32 as i32)
                                                        == (0 as i32)
                                                    || (((unsafe {
                                                        (*(*__slate_slot_1350)).fg.jointype
                                                    })
                                                        as u32)
                                                        as i32)
                                                        & ((64 as i32) | (16 as i32))
                                                        != (0 as i32)
                                                {
                                                    std::ptr::write(__slate_slot_1353, 114 as i32);
                                                    if (((unsafe {
                                                        (*(*__slate_slot_1333)).eOnePass
                                                    })
                                                        as u32)
                                                        as i32)
                                                        != (0 as i32)
                                                    {
                                                        *__slate_slot_1353 = 116 as i32;
                                                        unsafe {
                                                            *unsafe {
                                                                unsafe {
                                                                    (*(*__slate_slot_1333))
                                                                        .aiCurOnePass
                                                                        .as_mut_ptr()
                                                                        as *mut i32
                                                                }
                                                                .offset((0 as i32) as isize)
                                                            } = unsafe {
                                                                (*(*__slate_slot_1350)).iCursor
                                                            };
                                                        }
                                                    }
                                                    {}
                                                    unsafe {
                                                        sqlite3OpenTable(
                                                            pParse,
                                                            unsafe {
                                                                (*(*__slate_slot_1350)).iCursor
                                                            },
                                                            *__slate_slot_1349,
                                                            *__slate_slot_1348,
                                                            *__slate_slot_1353,
                                                        )
                                                    };
                                                    0 as i32;
                                                    {}
                                                    {}
                                                    if (((unsafe {
                                                        (*(*__slate_slot_1333)).eOnePass
                                                    })
                                                        as u32)
                                                        as i32)
                                                        == (0 as i32)
                                                        && ((unsafe {
                                                            (*(*__slate_slot_1348)).nCol
                                                        })
                                                            as i32)
                                                            < (((8 as u64).wrapping_mul(
                                                                ((8 as i32) as i64) as u64,
                                                            )
                                                                as u32)
                                                                as i32)
                                                        && (unsafe {
                                                            (*(*__slate_slot_1348)).tabFlags
                                                        }) & (((96 as i32) | (128 as i32))
                                                            as u32)
                                                            == ((0 as i32) as u32)
                                                        && (unsafe {
                                                            (*(*__slate_slot_1339)).wsFlags
                                                        }) & (((16384 as i32) | (4194304 as i32))
                                                            as u32)
                                                            == ((0 as i32) as u32)
                                                    {
                                                        // /* If we know that only a prefix of the record will be used,
                                                        //         ** it is advantageous to reduce the "column count" field in
                                                        //         ** the P4 operand of the OP_OpenRead/Write opcode. */
                                                        std::ptr::write(
                                                            __slate_slot_1354,
                                                            unsafe {
                                                                (*(*__slate_slot_1350)).colUsed
                                                            },
                                                        );
                                                        std::ptr::write(
                                                            __slate_slot_1355,
                                                            0 as i32,
                                                        );
                                                        loop {
                                                            if *__slate_slot_1354 != (0 as u64) {
                                                                *__slate_slot_1354 =
                                                                    *__slate_slot_1354
                                                                        >> (1 as i32);
                                                                std::ptr::write(
                                                                    __slate_slot_1894,
                                                                    *__slate_slot_1355,
                                                                );
                                                                std::ptr::write(
                                                                    __slate_slot_1895,
                                                                    *__slate_slot_1894 + (1 as i32),
                                                                );
                                                                *__slate_slot_1355 =
                                                                    *__slate_slot_1895;
                                                            } else {
                                                                break;
                                                            }
                                                        }
                                                        unsafe {
                                                            sqlite3VdbeChangeP4(
                                                                *__slate_slot_1334,
                                                                -(1 as i32),
                                                                ((*__slate_slot_1355 as i64)
                                                                    as *mut ())
                                                                    as *const i8,
                                                                -(3 as i32),
                                                            )
                                                        };
                                                        0 as i32;
                                                    }
                                                    unsafe {
                                                        sqlite3VdbeChangeP5(
                                                            *__slate_slot_1334,
                                                            *__slate_slot_1343 as u16,
                                                        )
                                                    };
                                                    if *__slate_slot_1340 >= (2 as i32)
                                                        && (((unsafe {
                                                            (*unsafe {
                                                                (*__slate_slot_1350)
                                                                    .offset((0 as i32) as isize)
                                                            })
                                                            .fg
                                                            .jointype
                                                        })
                                                            as u32)
                                                            as i32)
                                                            & ((64 as i32) | (8 as i32))
                                                            == (0 as i32)
                                                        && (unsafe {
                                                            (*(*__slate_slot_1338)).addrHalt
                                                        }) == unsafe {
                                                            (*unsafe {
                                                                unsafe {
                                                                    std::ptr::addr_of_mut!(
                                                                        (*(*__slate_slot_1333)).a
                                                                    )
                                                                        as *mut WhereLevel
                                                                }
                                                                .offset((0 as i32) as isize)
                                                            })
                                                            .addrHalt
                                                        }
                                                    {
                                                        unsafe {
                                                            sqlite3VdbeAddOp2(
                                                                *__slate_slot_1334,
                                                                37 as i32,
                                                                unsafe {
                                                                    (*(*__slate_slot_1350)).iCursor
                                                                },
                                                                unsafe {
                                                                    (*(*__slate_slot_1333)).iBreak
                                                                },
                                                            )
                                                        };
                                                        {}
                                                    }
                                                } else {
                                                    unsafe {
                                                        sqlite3TableLock(
                                                            pParse,
                                                            *__slate_slot_1349,
                                                            unsafe { (*(*__slate_slot_1348)).tnum },
                                                            ((0 as i32) as i8) as u8,
                                                            (unsafe {
                                                                (*(*__slate_slot_1348)).zName
                                                            })
                                                                as *const i8,
                                                        )
                                                    };
                                                }
                                            }
                                        }
                                    }
                                    if (unsafe { (*(*__slate_slot_1339)).wsFlags })
                                        & ((512 as i32) as u32)
                                        != (0 as u32)
                                    {
                                        std::ptr::write(__slate_slot_1356, unsafe {
                                            (*(*__slate_slot_1339)).u.btree.pIndex
                                        });
                                        std::ptr::write(__slate_slot_1358, 114 as i32);
                                        // /* iAuxArg is always set to a positive value if ONEPASS is possible */
                                        0 as i32;
                                        if !((unsafe { (*(*__slate_slot_1348)).tabFlags })
                                            & ((128 as i32) as u32)
                                            == ((0 as i32) as u32))
                                            && ((unsafe {
                                                (*(*__slate_slot_1356))
                                                    .__slate_bits_0
                                                    .__get_idxType()
                                            })
                                                as i32)
                                                == (2 as i32)
                                            && ((wctrlFlags as u32) as i32) & (32 as i32)
                                                != (0 as i32)
                                        {
                                            // /* This is one term of an OR-optimization using the PRIMARY KEY of a
                                            //         ** WITHOUT ROWID table.  No need for a separate index */
                                            *__slate_slot_1357 =
                                                unsafe { (*(*__slate_slot_1338)).iTabCur };
                                            *__slate_slot_1358 = 0 as i32;
                                        } else {
                                            if (((unsafe { (*(*__slate_slot_1333)).eOnePass })
                                                as u32)
                                                as i32)
                                                != (0 as i32)
                                            {
                                                std::ptr::write(__slate_slot_1359, unsafe {
                                                    (*unsafe { (*(*__slate_slot_1350)).pSTab })
                                                        .pIndex
                                                });
                                                *__slate_slot_1357 = iAuxArg;
                                                0 as i32;
                                                loop {
                                                    if *__slate_slot_1359
                                                        != std::ptr::null_mut::<Index>()
                                                        && *__slate_slot_1359 != *__slate_slot_1356
                                                    {
                                                        std::ptr::write(
                                                            __slate_slot_1896,
                                                            *__slate_slot_1357,
                                                        );
                                                        std::ptr::write(
                                                            __slate_slot_1897,
                                                            *__slate_slot_1896 + (1 as i32),
                                                        );
                                                        *__slate_slot_1357 = *__slate_slot_1897;
                                                        *__slate_slot_1359 = unsafe {
                                                            (*(*__slate_slot_1359)).pNext
                                                        };
                                                    } else {
                                                        break;
                                                    }
                                                }
                                                *__slate_slot_1358 = 116 as i32;
                                                unsafe {
                                                    *unsafe {
                                                        unsafe {
                                                            (*(*__slate_slot_1333))
                                                                .aiCurOnePass
                                                                .as_mut_ptr()
                                                                as *mut i32
                                                        }
                                                        .offset((1 as i32) as isize)
                                                    } = *__slate_slot_1357;
                                                }
                                            } else {
                                                if iAuxArg != (0 as i32)
                                                    && ((wctrlFlags as u32) as i32) & (32 as i32)
                                                        != (0 as i32)
                                                {
                                                    *__slate_slot_1357 = iAuxArg;
                                                    *__slate_slot_1358 = 113 as i32;
                                                } else {
                                                    std::ptr::write(__slate_slot_1898, pParse);
                                                    std::ptr::write(__slate_slot_1899, unsafe {
                                                        (*(*__slate_slot_1898)).nTab
                                                    });
                                                    std::ptr::write(
                                                        __slate_slot_1900,
                                                        *__slate_slot_1899 + (1 as i32),
                                                    );
                                                    unsafe {
                                                        (*(*__slate_slot_1898)).nTab =
                                                            *__slate_slot_1900;
                                                    }
                                                    *__slate_slot_1357 = *__slate_slot_1899;
                                                    if ((unsafe {
                                                        (*(*__slate_slot_1356))
                                                            .__slate_bits_0
                                                            .__get_bHasExpr()
                                                    })
                                                        as i32)
                                                        != (0 as i32)
                                                        && (unsafe {
                                                            (*(*__slate_slot_1341)).dbOptFlags
                                                        }) & ((16777216 as i32) as u32)
                                                            == ((0 as i32) as u32)
                                                    {
                                                        whereAddIndexedExpr(
                                                            pParse,
                                                            *__slate_slot_1356,
                                                            *__slate_slot_1357,
                                                            *__slate_slot_1350,
                                                        );
                                                    }
                                                    if (unsafe {
                                                        (*(*__slate_slot_1356)).pPartIdxWhere
                                                    }) != std::ptr::null_mut::<Expr>()
                                                        && (((unsafe {
                                                            (*(*__slate_slot_1350)).fg.jointype
                                                        })
                                                            as u32)
                                                            as i32)
                                                            & (16 as i32)
                                                            == (0 as i32)
                                                    {
                                                        wherePartIdxExpr(
                                                            pParse,
                                                            *__slate_slot_1356,
                                                            unsafe {
                                                                (*(*__slate_slot_1356))
                                                                    .pPartIdxWhere
                                                            },
                                                            std::ptr::null_mut::<u64>(),
                                                            *__slate_slot_1357,
                                                            *__slate_slot_1350,
                                                        );
                                                    }
                                                }
                                            }
                                        }
                                        unsafe {
                                            (*(*__slate_slot_1338)).iIdxCur = *__slate_slot_1357;
                                        }
                                        0 as i32;
                                        0 as i32;
                                        0 as i32;
                                        if *__slate_slot_1358 != (0 as i32) {
                                            unsafe {
                                                sqlite3VdbeAddOp3(
                                                    *__slate_slot_1334,
                                                    *__slate_slot_1358,
                                                    *__slate_slot_1357,
                                                    (unsafe { (*(*__slate_slot_1356)).tnum })
                                                        as i32,
                                                    *__slate_slot_1349,
                                                )
                                            };
                                            unsafe {
                                                sqlite3VdbeSetP4KeyInfo(pParse, *__slate_slot_1356)
                                            };
                                            if (unsafe { (*(*__slate_slot_1339)).wsFlags })
                                                & ((15 as i32) as u32)
                                                != ((0 as i32) as u32)
                                                && (unsafe { (*(*__slate_slot_1339)).wsFlags })
                                                    & (((2 as i32) | (32768 as i32)) as u32)
                                                    == ((0 as i32) as u32)
                                                && (unsafe { (*(*__slate_slot_1339)).wsFlags })
                                                    & ((524288 as i32) as u32)
                                                    == ((0 as i32) as u32)
                                                && (unsafe { (*(*__slate_slot_1339)).wsFlags })
                                                    & ((1048576 as i32) as u32)
                                                    == ((0 as i32) as u32)
                                                && (((unsafe { (*(*__slate_slot_1333)).wctrlFlags })
                                                    as u32)
                                                    as i32)
                                                    & (1 as i32)
                                                    == (0 as i32)
                                                && (((unsafe { (*(*__slate_slot_1333)).eDistinct })
                                                    as u32)
                                                    as i32)
                                                    != (2 as i32)
                                            {
                                                unsafe {
                                                    sqlite3VdbeChangeP5(
                                                        *__slate_slot_1334,
                                                        ((2 as i32) as i16) as u16,
                                                    )
                                                };
                                            }
                                            {}
                                        }
                                    }
                                    if *__slate_slot_1349 >= (0 as i32) {
                                        unsafe {
                                            sqlite3CodeVerifySchema(pParse, *__slate_slot_1349)
                                        };
                                    }
                                    if (((unsafe { (*(*__slate_slot_1350)).fg.jointype }) as u32)
                                        as i32)
                                        & (16 as i32)
                                        != (0 as i32)
                                    {
                                        std::ptr::write(
                                            __slate_slot_1902,
                                            sqlite3WhereMalloc(*__slate_slot_1333, 20 as u64)
                                                as *mut WhereRightJoin,
                                        );
                                        unsafe {
                                            (*(*__slate_slot_1338)).pRJ = *__slate_slot_1902;
                                        }
                                        *__slate_slot_1901 = *__slate_slot_1902
                                            != std::ptr::null_mut::<WhereRightJoin>();
                                    } else {
                                        *__slate_slot_1901 = false as bool;
                                    }
                                    if *__slate_slot_1901 {
                                        std::ptr::write(__slate_slot_1360, unsafe {
                                            (*(*__slate_slot_1338)).pRJ
                                        });
                                        std::ptr::write(__slate_slot_1361, 0 as i32);
                                        std::ptr::write(__slate_slot_1903, pParse);
                                        std::ptr::write(__slate_slot_1904, unsafe {
                                            (*(*__slate_slot_1903)).nTab
                                        });
                                        std::ptr::write(
                                            __slate_slot_1905,
                                            *__slate_slot_1904 + (1 as i32),
                                        );
                                        unsafe {
                                            (*(*__slate_slot_1903)).nTab = *__slate_slot_1905;
                                        }
                                        unsafe {
                                            (*(*__slate_slot_1360)).iMatch = *__slate_slot_1904;
                                        }
                                        std::ptr::write(__slate_slot_1906, pParse);
                                        std::ptr::write(__slate_slot_1907, unsafe {
                                            (*(*__slate_slot_1906)).nMem
                                        });
                                        std::ptr::write(
                                            __slate_slot_1908,
                                            *__slate_slot_1907 + (1 as i32),
                                        );
                                        unsafe {
                                            (*(*__slate_slot_1906)).nMem = *__slate_slot_1908;
                                        }
                                        unsafe {
                                            (*(*__slate_slot_1360)).regReturn = *__slate_slot_1908;
                                        }
                                        unsafe {
                                            sqlite3VdbeAddOp2(
                                                *__slate_slot_1334,
                                                77 as i32,
                                                0 as i32,
                                                unsafe { (*(*__slate_slot_1360)).regReturn },
                                            )
                                        };
                                        0 as i32;
                                        if (unsafe { (*(*__slate_slot_1348)).tabFlags })
                                            & ((128 as i32) as u32)
                                            == ((0 as i32) as u32)
                                        {
                                            unsafe {
                                                sqlite3VdbeAddOp2(
                                                    *__slate_slot_1334,
                                                    120 as i32,
                                                    unsafe { (*(*__slate_slot_1360)).iMatch },
                                                    1 as i32,
                                                )
                                            };
                                            *__slate_slot_1361 = 1 as i32;
                                            *__slate_slot_1362 = unsafe {
                                                sqlite3KeyInfoAlloc(
                                                    unsafe { (*pParse).db },
                                                    1 as i32,
                                                    0 as i32,
                                                )
                                            };
                                            if *__slate_slot_1362 != std::ptr::null_mut::<KeyInfo>()
                                            {
                                                unsafe {
                                                    *unsafe {
                                                        unsafe {
                                                            std::ptr::addr_of_mut!(
                                                                (*(*__slate_slot_1362)).aColl
                                                            )
                                                                as *mut *mut CollSeq
                                                        }
                                                        .offset((0 as i32) as isize)
                                                    } = std::ptr::null_mut::<CollSeq>();
                                                }
                                                unsafe {
                                                    *unsafe {
                                                        unsafe {
                                                            (*(*__slate_slot_1362)).aSortFlags
                                                        }
                                                        .offset((0 as i32) as isize)
                                                    } = ((0 as i32) as i8) as u8;
                                                }
                                                unsafe {
                                                    sqlite3VdbeAppendP4(
                                                        *__slate_slot_1334,
                                                        *__slate_slot_1362 as *mut (),
                                                        -(9 as i32),
                                                    )
                                                };
                                            }
                                        } else {
                                            std::ptr::write(__slate_slot_1363, unsafe {
                                                sqlite3PrimaryKeyIndex(*__slate_slot_1348)
                                            });
                                            unsafe {
                                                sqlite3VdbeAddOp2(
                                                    *__slate_slot_1334,
                                                    120 as i32,
                                                    unsafe { (*(*__slate_slot_1360)).iMatch },
                                                    ((unsafe { (*(*__slate_slot_1363)).nKeyCol })
                                                        as u32)
                                                        as i32,
                                                )
                                            };
                                            unsafe {
                                                sqlite3VdbeSetP4KeyInfo(pParse, *__slate_slot_1363)
                                            };
                                            *__slate_slot_1361 = unsafe {
                                                sqlite3IndexBloomable(
                                                    *__slate_slot_1363 as *const Index,
                                                    ((unsafe { (*(*__slate_slot_1363)).nKeyCol })
                                                        as u32)
                                                        as i32,
                                                )
                                            };
                                        }
                                        if *__slate_slot_1361 != (0 as i32) {
                                            std::ptr::write(__slate_slot_1909, pParse);
                                            std::ptr::write(__slate_slot_1910, unsafe {
                                                (*(*__slate_slot_1909)).nMem
                                            });
                                            std::ptr::write(
                                                __slate_slot_1911,
                                                *__slate_slot_1910 + (1 as i32),
                                            );
                                            unsafe {
                                                (*(*__slate_slot_1909)).nMem = *__slate_slot_1911;
                                            }
                                            unsafe {
                                                (*(*__slate_slot_1360)).regBloom =
                                                    *__slate_slot_1911;
                                            }
                                            unsafe {
                                                sqlite3VdbeAddOp2(
                                                    *__slate_slot_1334,
                                                    79 as i32,
                                                    65536 as i32,
                                                    unsafe { (*(*__slate_slot_1360)).regBloom },
                                                )
                                            };
                                        } else {
                                            unsafe {
                                                (*(*__slate_slot_1360)).regBloom = 0 as i32;
                                            }
                                        }
                                        std::ptr::write(__slate_slot_1912, *__slate_slot_1339);
                                        std::ptr::write(__slate_slot_1913, unsafe {
                                            (*(*__slate_slot_1912)).wsFlags
                                        });
                                        std::ptr::write(
                                            __slate_slot_1914,
                                            *__slate_slot_1913 & (!(64 as i32) as u32),
                                        );
                                        unsafe {
                                            (*(*__slate_slot_1912)).wsFlags = *__slate_slot_1914;
                                        }
                                        // /* The nature of RIGHT JOIN processing is such that it messes up
                                        //       ** the output order.  So omit any ORDER BY/GROUP BY elimination
                                        //       ** optimizations.  We need to do an actual sort for RIGHT JOIN. */
                                        unsafe {
                                            (*(*__slate_slot_1333)).nOBSat = (0 as i32) as i8;
                                        }
                                        unsafe {
                                            (*(*__slate_slot_1333)).eDistinct =
                                                ((3 as i32) as i8) as u8;
                                        }
                                    }
                                    std::ptr::write(__slate_slot_1890, *__slate_slot_1340);
                                    std::ptr::write(
                                        __slate_slot_1891,
                                        *__slate_slot_1890 + (1 as i32),
                                    );
                                    *__slate_slot_1340 = *__slate_slot_1891;
                                    std::ptr::write(__slate_slot_1892, *__slate_slot_1338);
                                    std::ptr::write(__slate_slot_1893, unsafe {
                                        (*__slate_slot_1892).offset((1 as i32) as isize)
                                    });
                                    *__slate_slot_1338 = *__slate_slot_1893;
                                } else {
                                    break;
                                }
                            }
                            unsafe {
                                (*(*__slate_slot_1333)).iTop =
                                    unsafe { sqlite3VdbeCurrentAddr(*__slate_slot_1334) };
                            }
                            if (unsafe { (*(*__slate_slot_1341)).mallocFailed }) != (0 as u8) {
                            } else {
                                // /* Generate the code to do the search.  Each iteration of the for
                                //   ** loop below generates code for a single nested loop of the VM
                                //   ** program.
                                //   */
                                *__slate_slot_1340 = 0 as i32;
                                loop {
                                    if *__slate_slot_1340 < *__slate_slot_1332 {
                                        if (unsafe { (*pParse).nErr }) != (0 as i32) {
                                            break '__join_2;
                                        } else {
                                            *__slate_slot_1338 = unsafe {
                                                unsafe {
                                                    std::ptr::addr_of_mut!(
                                                        (*(*__slate_slot_1333)).a
                                                    )
                                                        as *mut WhereLevel
                                                }
                                                .offset(*__slate_slot_1340 as isize)
                                            };
                                            *__slate_slot_1365 = (unsafe {
                                                (*unsafe { (*(*__slate_slot_1338)).pWLoop }).wsFlags
                                            })
                                                as i32;
                                            *__slate_slot_1366 = unsafe {
                                                unsafe {
                                                    std::ptr::addr_of_mut!((*pTabList).a)
                                                        as *mut SrcItem
                                                }
                                                .offset(
                                                    (((unsafe { (*(*__slate_slot_1338)).iFrom })
                                                        as u32)
                                                        as i32)
                                                        as isize,
                                                )
                                            };
                                            if ((unsafe {
                                                (*(*__slate_slot_1366))
                                                    .fg
                                                    .__slate_bits_0
                                                    .__get_isMaterialized()
                                            })
                                                as i32)
                                                != (0 as i32)
                                            {
                                                std::ptr::write(__slate_slot_1368, 0 as i32);
                                                0 as i32;
                                                *__slate_slot_1367 =
                                                    unsafe { (*(*__slate_slot_1366)).u4.pSubq };
                                                if ((unsafe {
                                                    (*(*__slate_slot_1366))
                                                        .fg
                                                        .__slate_bits_0
                                                        .__get_isCorrelated()
                                                })
                                                    as i32)
                                                    == (0 as i32)
                                                {
                                                    *__slate_slot_1368 = unsafe {
                                                        sqlite3VdbeAddOp0(
                                                            *__slate_slot_1334,
                                                            15 as i32,
                                                        )
                                                    };
                                                    {}
                                                } else {
                                                    *__slate_slot_1368 = 0 as i32;
                                                }
                                                unsafe {
                                                    sqlite3VdbeAddOp2(
                                                        *__slate_slot_1334,
                                                        10 as i32,
                                                        unsafe {
                                                            (*(*__slate_slot_1367)).regReturn
                                                        },
                                                        unsafe {
                                                            (*(*__slate_slot_1367)).addrFillSub
                                                        },
                                                    )
                                                };
                                                {}
                                                if *__slate_slot_1368 != (0 as i32) {
                                                    unsafe {
                                                        sqlite3VdbeJumpHere(
                                                            *__slate_slot_1334,
                                                            *__slate_slot_1368,
                                                        )
                                                    };
                                                }
                                            }
                                            0 as i32;
                                            if *__slate_slot_1365
                                                & ((16384 as i32) | (4194304 as i32))
                                                != (0 as i32)
                                            {
                                                '__join_8: {
                                                    if *__slate_slot_1365 & (16384 as i32)
                                                        != (0 as i32)
                                                    {
                                                        constructAutomaticIndex(
                                                            pParse,
                                                            unsafe {
                                                                std::ptr::addr_of_mut!(
                                                                    (*(*__slate_slot_1333)).sWC
                                                                )
                                                            },
                                                            *__slate_slot_1335,
                                                            *__slate_slot_1338,
                                                        );
                                                    } else {
                                                        sqlite3ConstructBloomFilter(
                                                            *__slate_slot_1333,
                                                            *__slate_slot_1340,
                                                            *__slate_slot_1338,
                                                            *__slate_slot_1335,
                                                        );
                                                    }
                                                }
                                                if (unsafe { (*(*__slate_slot_1341)).mallocFailed })
                                                    != (0 as u8)
                                                {
                                                    break '__join_2;
                                                }
                                            }
                                            *__slate_slot_1364 = unsafe {
                                                sqlite3WhereExplainOneScan(
                                                    pParse,
                                                    pTabList,
                                                    *__slate_slot_1338,
                                                    wctrlFlags,
                                                )
                                            };
                                            unsafe {
                                                (*(*__slate_slot_1338)).addrBody = unsafe {
                                                    sqlite3VdbeCurrentAddr(*__slate_slot_1334)
                                                };
                                            }
                                            *__slate_slot_1335 = unsafe {
                                                sqlite3WhereCodeOneLoopStart(
                                                    pParse,
                                                    *__slate_slot_1334,
                                                    *__slate_slot_1333,
                                                    *__slate_slot_1340,
                                                    *__slate_slot_1338,
                                                    *__slate_slot_1335,
                                                )
                                            };
                                            unsafe {
                                                (*(*__slate_slot_1333)).iContinue =
                                                    unsafe { (*(*__slate_slot_1338)).addrCont };
                                            }
                                            if *__slate_slot_1365 & (8192 as i32) == (0 as i32)
                                                && ((wctrlFlags as u32) as i32) & (32 as i32)
                                                    == (0 as i32)
                                            {
                                                *__slate_slot_1364;
                                            }
                                            std::ptr::write(__slate_slot_1915, *__slate_slot_1340);
                                            std::ptr::write(
                                                __slate_slot_1916,
                                                *__slate_slot_1915 + (1 as i32),
                                            );
                                            *__slate_slot_1340 = *__slate_slot_1916;
                                        }
                                    } else {
                                        break;
                                    }
                                }
                                // /* Done. */
                                {}
                                unsafe {
                                    (*(*__slate_slot_1333)).iEndWhere =
                                        unsafe { sqlite3VdbeCurrentAddr(*__slate_slot_1334) };
                                }
                                return *__slate_slot_1333;
                            }
                        }
                    }
                }
            }
            if *__slate_slot_1333 != std::ptr::null_mut::<WhereInfo>() {
                unsafe {
                    (*pParse).nQueryLoop =
                        (unsafe { (*(*__slate_slot_1333)).savedNQueryLoop }) as i16;
                }
                whereInfoFree(*__slate_slot_1341, *__slate_slot_1333);
            }
            return std::ptr::null_mut::<WhereInfo>();
        }
    }
    // /* Jump here if malloc fails */
    return unsafe { std::mem::zeroed() };
}

// /* The parser context */
// /* FROM clause: A list of all tables to be scanned */
// /* The WHERE clause */
// /* An ORDER BY (or GROUP BY) clause, or NULL */
// /* Query result set.  Req'd for DISTINCT */
// /* The entire SELECT statement */
// /* The WHERE_* flags defined in sqliteInt.h */
// /* If WHERE_OR_SUBCLAUSE is set, index cursor number
//                           ** If WHERE_USE_LIMIT, then the limit amount */
// /*
// ** Part of sqlite3WhereEnd() will rewrite opcodes to reference the
// ** index rather than the main table.  In SQLITE_DEBUG mode, we want
// ** to trace those changes if PRAGMA vdbe_addoptrace=on.  This routine
// ** does that.
// */
// /* no-op */
// /*
// ** Generate the end of the WHERE loop.  See comments on
// ** sqlite3WhereBegin() for additional information.
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3WhereEnd(mut pWInfo: *mut WhereInfo) {
    let mut pParse: *mut Parse = unsafe { (*pWInfo).pParse };
    let mut v: *mut Vdbe = unsafe { (*pParse).pVdbe };
    let mut i: i32 = 0 as i32;
    let mut pLevel: *mut WhereLevel = unsafe { std::mem::zeroed() };
    let mut pLoop: *mut WhereLoop = unsafe { std::mem::zeroed() };
    let mut pTabList: *mut SrcList = unsafe { (*pWInfo).pTabList };
    let mut db: *mut sqlite3 = unsafe { (*pParse).db };
    let mut iEnd: i32 = unsafe { sqlite3VdbeCurrentAddr(v) };
    let mut nRJ: i32 = 0 as i32;
    let mut addrSeek: i32 = 0 as i32;
    // /* Generate loop termination code.
    //   */
    {}
    i = (((unsafe { (*pWInfo).nLevel }) as u32) as i32) - (1 as i32);
    '__slate_break_1846: loop {
        if !(i >= (0 as i32)) {
            break;
        }
        let mut addr: i32 = 0 as i32;
        pLevel = unsafe {
            unsafe { std::ptr::addr_of_mut!((*pWInfo).a) as *mut WhereLevel }.offset(i as isize)
        };
        if (unsafe { (*pLevel).pRJ }) != std::ptr::null_mut::<WhereRightJoin>() {
            // /* Terminate the subroutine that forms the interior of the loop of
            //       ** the RIGHT JOIN table */
            let mut pRJ: *mut WhereRightJoin = unsafe { (*pLevel).pRJ };
            unsafe { sqlite3VdbeResolveLabel(v, unsafe { (*pLevel).addrCont }) };
            // /* Replace addrCont with a new label that will never be used, just so
            //       ** the subsequent call to resolve pLevel->addrCont will have something
            //       ** to resolve. */
            unsafe {
                (*pLevel).addrCont = unsafe { sqlite3VdbeMakeLabel(pParse) };
            }
            unsafe {
                (*pRJ).endSubrtn = unsafe { sqlite3VdbeCurrentAddr(v) };
            }
            unsafe {
                sqlite3VdbeAddOp3(
                    v,
                    69 as i32,
                    unsafe { (*pRJ).regReturn },
                    unsafe { (*pRJ).addrSubrtn },
                    1 as i32,
                )
            };
            {}
            let __v1919: i32 = nRJ;
            let __v1920: i32 = __v1919 + (1 as i32);
            nRJ = __v1920;
        }
        pLoop = unsafe { (*pLevel).pWLoop };
        if (((unsafe { (*pLevel).op }) as u32) as i32) != (189 as i32) {
            let mut pIdx: *mut Index = unsafe { std::mem::zeroed() };
            let mut n: i32 = 0 as i32;
            let __v1921: bool;
            if (((unsafe { (*pWInfo).eDistinct }) as u32) as i32) == (2 as i32)
                && i == (((unsafe { (*pWInfo).nLevel }) as u32) as i32) - (1 as i32)
                && (unsafe { (*pLoop).wsFlags }) & ((512 as i32) as u32) != ((0 as i32) as u32)
            {
                let __v1922: *mut Index = unsafe { (*pLoop).u.btree.pIndex };
                pIdx = __v1922;
                __v1921 =
                    ((unsafe { (*__v1922).__slate_bits_0.__get_hasStat1() }) as i32) != (0 as i32);
            } else {
                __v1921 = false as bool;
            }
            let __v1923: bool;
            if __v1921 {
                let __v1924: i32 = ((unsafe { (*pLoop).u.btree.nDistinctCol }) as u32) as i32;
                n = __v1924;
                __v1923 = __v1924 > (0 as i32);
            } else {
                __v1923 = false as bool;
            }
            if __v1923
                && ((unsafe { *unsafe { unsafe { (*pIdx).aiRowLogEst }.offset(n as isize) } })
                    as i32)
                    >= (36 as i32)
            {
                let mut r1: i32 = (unsafe { (*pParse).nMem }) + (1 as i32);
                let mut j: i32 = 0 as i32;
                let mut op: i32 = 0 as i32;
                // /* Init to avoid false-positive compiler warning */
                let mut addrIfNull: i32 = 0 as i32;
                if (unsafe { (*pLevel).iLeftJoin }) != (0 as i32) {
                    addrIfNull = unsafe {
                        sqlite3VdbeAddOp2(v, 20 as i32, unsafe { (*pLevel).iIdxCur }, r1)
                    };
                }
                j = 0 as i32;
                '__slate_break_1847: loop {
                    if !(j < n) {
                        break;
                    }
                    unsafe {
                        sqlite3VdbeAddOp3(v, 96 as i32, unsafe { (*pLevel).iIdxCur }, j, r1 + j)
                    };
                    let __v1925: i32 = j;
                    let __v1926: i32 = __v1925 + (1 as i32);
                    j = __v1926;
                }
                let __v1927: *mut Parse = pParse;
                let __v1928: i32 = unsafe { (*__v1927).nMem };
                let __v1929: i32 = __v1928 + (n + (1 as i32));
                unsafe {
                    (*__v1927).nMem = __v1929;
                }
                op = if (((unsafe { (*pLevel).op }) as u32) as i32) == (39 as i32) {
                    21 as i32
                } else {
                    24 as i32
                };
                addrSeek = unsafe {
                    sqlite3VdbeAddOp4Int(v, op, unsafe { (*pLevel).iIdxCur }, 0 as i32, r1, n)
                };
                {}
                {}
                unsafe { sqlite3VdbeAddOp2(v, 9 as i32, 1 as i32, unsafe { (*pLevel).p2 }) };
                if (unsafe { (*pLevel).iLeftJoin }) != (0 as i32) {
                    unsafe { sqlite3VdbeJumpHere(v, addrIfNull) };
                }
            }
            // /* Ticket [ef9318757b152e3] 2017-10-21 */
            // /* SQLITE_DISABLE_SKIPAHEAD_DISTINCT */
        }
        if ((unsafe {
            (*unsafe {
                unsafe { std::ptr::addr_of_mut!((*pTabList).a) as *mut SrcItem }
                    .offset((((unsafe { (*pLevel).iFrom }) as u32) as i32) as isize)
            })
            .fg
            .__slate_bits_0
            .__get_fromExists()
        }) as i32)
            != (0 as i32)
        {
            // /* This is an EXISTS-to-JOIN optimization loop. If this loop sees a
            //       ** successful row, it should break out of itself. */
            unsafe { sqlite3VdbeAddOp2(v, 9 as i32, 0 as i32, unsafe { (*pLevel).addrBrk }) };
            {}
        }
        unsafe { sqlite3VdbeResolveLabel(v, unsafe { (*pLevel).addrCont }) };
        if (((unsafe { (*pLevel).op }) as u32) as i32) != (189 as i32) {
            unsafe {
                sqlite3VdbeAddOp3(
                    v,
                    ((unsafe { (*pLevel).op }) as u32) as i32,
                    unsafe { (*pLevel).p1 },
                    unsafe { (*pLevel).p2 },
                    ((unsafe { (*pLevel).p3 }) as u32) as i32,
                )
            };
            unsafe { sqlite3VdbeChangeP5(v, (unsafe { (*pLevel).p5 }) as u16) };
            {}
            {}
            {}
            {}
            if (unsafe { (*pLevel).regBignull }) != (0 as i32) {
                unsafe { sqlite3VdbeResolveLabel(v, unsafe { (*pLevel).addrBignull }) };
                unsafe {
                    sqlite3VdbeAddOp2(
                        v,
                        63 as i32,
                        unsafe { (*pLevel).regBignull },
                        (unsafe { (*pLevel).p2 }) - (1 as i32),
                    )
                };
                {}
            }
            if addrSeek != (0 as i32) {
                unsafe { sqlite3VdbeJumpHere(v, addrSeek) };
                addrSeek = 0 as i32;
            }
        }
        if (unsafe { (*pLoop).wsFlags }) & ((2048 as i32) as u32) != ((0 as i32) as u32)
            && (unsafe { (*pLevel).u.r#in.nIn }) > (0 as i32)
        {
            let mut pIn: *mut InLoop = unsafe { std::mem::zeroed() };
            let mut j: i32 = 0 as i32;
            unsafe { sqlite3VdbeResolveLabel(v, unsafe { (*pLevel).addrNxt }) };
            j = unsafe { (*pLevel).u.r#in.nIn };
            let __v1930: *mut InLoop =
                unsafe { unsafe { (*pLevel).u.r#in.aInLoop }.offset((j - (1 as i32)) as isize) };
            pIn = __v1930;
            '__slate_break_1848: while j > (0 as i32) {
                0 as i32;
                unsafe { sqlite3VdbeJumpHere(v, (unsafe { (*pIn).addrInTop }) + (1 as i32)) };
                if (((unsafe { (*pIn).eEndLoopOp }) as u32) as i32) != (189 as i32) {
                    if (unsafe { (*pIn).nPrefix }) != (0 as i32) {
                        let mut bEarlyOut: i32 =
                            ((unsafe { (*pLoop).wsFlags }) & ((1024 as i32) as u32)
                                == ((0 as i32) as u32)
                                && (unsafe { (*pLoop).wsFlags }) & ((262144 as i32) as u32)
                                    != ((0 as i32) as u32)) as i32;
                        if (unsafe { (*pLevel).iLeftJoin }) != (0 as i32) {
                            // /* For LEFT JOIN queries, cursor pIn->iCur may not have been
                            //               ** opened yet. This occurs for WHERE clauses such as
                            //               ** "a = ? AND b IN (...)", where the index is on (a, b). If
                            //               ** the RHS of the (a=?) is NULL, then the "b IN (...)" may
                            //               ** never have been coded, but the body of the loop run to
                            //               ** return the null-row. So, if the cursor is not open yet,
                            //               ** jump over the OP_Next or OP_Prev instruction about to
                            //               ** be coded.  */
                            unsafe {
                                sqlite3VdbeAddOp2(
                                    v,
                                    25 as i32,
                                    unsafe { (*pIn).iCur },
                                    (unsafe { sqlite3VdbeCurrentAddr(v) }) + (2 as i32) + bEarlyOut,
                                )
                            };
                            {}
                        }
                        if bEarlyOut != (0 as i32) {
                            unsafe {
                                sqlite3VdbeAddOp4Int(
                                    v,
                                    26 as i32,
                                    unsafe { (*pLevel).iIdxCur },
                                    (unsafe { sqlite3VdbeCurrentAddr(v) }) + (2 as i32),
                                    unsafe { (*pIn).iBase },
                                    unsafe { (*pIn).nPrefix },
                                )
                            };
                            {}
                            // /* Retarget the OP_IsNull against the left operand of IN so
                            //               ** it jumps past the OP_IfNoHope.  This is because the
                            //               ** OP_IsNull also bypasses the OP_Affinity opcode that is
                            //               ** required by OP_IfNoHope. */
                            unsafe {
                                sqlite3VdbeJumpHere(v, (unsafe { (*pIn).addrInTop }) + (1 as i32))
                            };
                        }
                    }
                    unsafe {
                        sqlite3VdbeAddOp2(
                            v,
                            ((unsafe { (*pIn).eEndLoopOp }) as u32) as i32,
                            unsafe { (*pIn).iCur },
                            unsafe { (*pIn).addrInTop },
                        )
                    };
                    {}
                    {}
                    {}
                }
                unsafe { sqlite3VdbeJumpHere(v, (unsafe { (*pIn).addrInTop }) - (1 as i32)) };
                let __v1931: i32 = j;
                let __v1932: i32 = __v1931 - (1 as i32);
                j = __v1932;
                let __v1933: *mut InLoop = pIn;
                let __v1934: *mut InLoop = unsafe { __v1933.offset(-((1 as i32) as isize)) };
                pIn = __v1934;
            }
        }
        unsafe { sqlite3VdbeResolveLabel(v, unsafe { (*pLevel).addrBrk }) };
        if (unsafe { (*pLevel).pRJ }) != std::ptr::null_mut::<WhereRightJoin>() {
            unsafe {
                sqlite3VdbeAddOp3(
                    v,
                    69 as i32,
                    unsafe { (*unsafe { (*pLevel).pRJ }).regReturn },
                    0 as i32,
                    1 as i32,
                )
            };
            {}
        }
        if (unsafe { (*pLevel).addrSkip }) != (0 as i32) {
            unsafe { sqlite3VdbeGoto(v, unsafe { (*pLevel).addrSkip }) };
            {}
            unsafe { sqlite3VdbeJumpHere(v, unsafe { (*pLevel).addrSkip }) };
            unsafe { sqlite3VdbeJumpHere(v, (unsafe { (*pLevel).addrSkip }) - (2 as i32)) };
        }
        if (unsafe { (*pLevel).addrLikeRep }) != (0 as i32) {
            unsafe {
                sqlite3VdbeAddOp2(
                    v,
                    63 as i32,
                    ((unsafe { (*pLevel).iLikeRepCntr }) >> (1 as i32)) as i32,
                    unsafe { (*pLevel).addrLikeRep },
                )
            };
            {}
        }
        if (unsafe { (*pLevel).iLeftJoin }) != (0 as i32) {
            let mut ws: i32 = (unsafe { (*pLoop).wsFlags }) as i32;
            addr = unsafe { sqlite3VdbeAddOp1(v, 61 as i32, unsafe { (*pLevel).iLeftJoin }) };
            {}
            0 as i32;
            if ws & (64 as i32) == (0 as i32) {
                let mut pSrc: *mut SrcItem = unsafe {
                    unsafe { std::ptr::addr_of_mut!((*pTabList).a) as *mut SrcItem }
                        .offset((((unsafe { (*pLevel).iFrom }) as u32) as i32) as isize)
                };
                0 as i32;
                if ((unsafe { (*pSrc).fg.__slate_bits_0.__get_viaCoroutine() }) as i32)
                    != (0 as i32)
                {
                    let mut m: i32 = 0 as i32;
                    let mut n: i32 = 0 as i32;
                    0 as i32;
                    n = unsafe { (*unsafe { (*pSrc).u4.pSubq }).regResult };
                    0 as i32;
                    m = (unsafe { (*unsafe { (*pSrc).pSTab }).nCol }) as i32;
                    unsafe { sqlite3VdbeAddOp3(v, 77 as i32, 0 as i32, n, n + m - (1 as i32)) };
                }
                unsafe { sqlite3VdbeAddOp1(v, 138 as i32, unsafe { (*pLevel).iTabCur }) };
            }
            if ws & (512 as i32) != (0 as i32)
                || ws & (8192 as i32) != (0 as i32)
                    && (unsafe { (*pLevel).u.pCoveringIdx }) != std::ptr::null_mut::<Index>()
            {
                if ws & (8192 as i32) != (0 as i32) {
                    let mut pIx: *mut Index = unsafe { (*pLevel).u.pCoveringIdx };
                    let mut iDb: i32 =
                        unsafe { sqlite3SchemaToIndex(db, unsafe { (*pIx).pSchema }) };
                    unsafe {
                        sqlite3VdbeAddOp3(
                            v,
                            113 as i32,
                            unsafe { (*pLevel).iIdxCur },
                            (unsafe { (*pIx).tnum }) as i32,
                            iDb,
                        )
                    };
                    unsafe { sqlite3VdbeSetP4KeyInfo(pParse, pIx) };
                }
                unsafe { sqlite3VdbeAddOp1(v, 138 as i32, unsafe { (*pLevel).iIdxCur }) };
            }
            if (((unsafe { (*pLevel).op }) as u32) as i32) == (69 as i32) {
                unsafe {
                    sqlite3VdbeAddOp2(v, 10 as i32, unsafe { (*pLevel).p1 }, unsafe {
                        (*pLevel).addrFirst
                    })
                };
            } else {
                unsafe { sqlite3VdbeGoto(v, unsafe { (*pLevel).addrFirst }) };
            }
            unsafe { sqlite3VdbeJumpHere(v, addr) };
        }
        {}
        let __v1917: i32 = i;
        let __v1918: i32 = __v1917 - (1 as i32);
        i = __v1918;
    }
    0 as i32;
    i = 0 as i32;
    let __v1935: *mut WhereLevel =
        unsafe { std::ptr::addr_of_mut!((*pWInfo).a) as *mut WhereLevel };
    pLevel = __v1935;
    '__slate_break_1849: while i < (((unsafe { (*pWInfo).nLevel }) as u32) as i32) {
        let mut k: i32 = 0 as i32;
        let mut last: i32 = 0 as i32;
        let mut pOp: *mut VdbeOp = unsafe { std::mem::zeroed() };
        let mut pLastOp: *mut VdbeOp = unsafe { std::mem::zeroed() };
        let mut pIdx: *mut Index = std::ptr::null_mut::<Index>();
        let mut pTabItem: *mut SrcItem = unsafe {
            unsafe { std::ptr::addr_of_mut!((*pTabList).a) as *mut SrcItem }
                .offset((((unsafe { (*pLevel).iFrom }) as u32) as i32) as isize)
        };
        let mut pTab: *mut Table = unsafe { (*pTabItem).pSTab };
        0 as i32;
        pLoop = unsafe { (*pLevel).pWLoop };
        // /* Do RIGHT JOIN processing.  Generate code that will output the
        //     ** unmatched rows of the right operand of the RIGHT JOIN with
        //     ** all of the columns of the left operand set to NULL.
        //     */
        if (unsafe { (*pLevel).pRJ }) != std::ptr::null_mut::<WhereRightJoin>() {
            unsafe { sqlite3WhereRightJoinLoop(pWInfo, i, pLevel) };
        } else {
            // /* For a co-routine, change all OP_Column references to the table of
            //     ** the co-routine into OP_Copy of result contained in a register.
            //     ** OP_Rowid becomes OP_Null.
            //     */
            if ((unsafe { (*pTabItem).fg.__slate_bits_0.__get_viaCoroutine() }) as i32)
                != (0 as i32)
            {
                {}
                0 as i32;
                0 as i32;
                translateColumnToCopy(
                    pParse,
                    unsafe { (*pLevel).addrBody },
                    unsafe { (*pLevel).iTabCur },
                    unsafe { (*unsafe { (*pTabItem).u4.pSubq }).regResult },
                    0 as i32,
                );
            } else {
                // /* If this scan uses an index, make VDBE code substitutions to read data
                //     ** from the index instead of from the table where possible.  In some cases
                //     ** this optimization prevents the table from ever being read, which can
                //     ** yield a significant performance boost.
                //     **
                //     ** Calls to the code generator in between sqlite3WhereBegin and
                //     ** sqlite3WhereEnd will have created code that references the table
                //     ** directly.  This loop scans all that code looking for opcodes
                //     ** that reference the table and converts them into opcodes that
                //     ** reference the index.
                //     */
                if (unsafe { (*pLoop).wsFlags }) & (((512 as i32) | (64 as i32)) as u32)
                    != (0 as u32)
                {
                    pIdx = unsafe { (*pLoop).u.btree.pIndex };
                } else {
                    if (unsafe { (*pLoop).wsFlags }) & ((8192 as i32) as u32) != (0 as u32) {
                        pIdx = unsafe { (*pLevel).u.pCoveringIdx };
                    }
                }
                if pIdx != std::ptr::null_mut::<Index>()
                    && !((unsafe { (*db).mallocFailed }) != (0 as u8))
                {
                    if (((unsafe { (*pWInfo).eOnePass }) as u32) as i32) == (0 as i32)
                        || !((unsafe { (*unsafe { (*pIdx).pTable }).tabFlags })
                            & ((128 as i32) as u32)
                            == ((0 as i32) as u32))
                    {
                        last = iEnd;
                    } else {
                        last = unsafe { (*pWInfo).iEndWhere };
                    }
                    if ((unsafe { (*pIdx).__slate_bits_0.__get_bHasExpr() }) as i32) != (0 as i32) {
                        let mut p: *mut IndexedExpr = unsafe { (*pParse).pIdxEpr };
                        '__slate_break_1850: while p != std::ptr::null_mut::<IndexedExpr>() {
                            if (unsafe { (*p).iIdxCur }) == unsafe { (*pLevel).iIdxCur } {
                                unsafe {
                                    (*p).iDataCur = -(1 as i32);
                                }
                                unsafe {
                                    (*p).iIdxCur = -(1 as i32);
                                }
                            }
                            p = unsafe { (*p).pIENext };
                        }
                    }
                    k = (unsafe { (*pLevel).addrBody }) + (1 as i32);
                    pOp = unsafe { sqlite3VdbeGetOp(v, k) };
                    pLastOp = unsafe { pOp.offset((last - k) as isize) };
                    0 as i32;
                    '__slate_break_1851: loop {
                        if (unsafe { (*pOp).p1 }) != unsafe { (*pLevel).iTabCur } {
                            // /* no-op */
                        } else {
                            if (((unsafe { (*pOp).opcode }) as u32) as i32) == (96 as i32) {
                                let mut x: i32 = unsafe { (*pOp).p2 };
                                0 as i32;
                                if !((unsafe { (*pTab).tabFlags }) & ((128 as i32) as u32)
                                    == ((0 as i32) as u32))
                                {
                                    let mut pPk: *mut Index =
                                        unsafe { sqlite3PrimaryKeyIndex(pTab) };
                                    x = (unsafe {
                                        *unsafe { unsafe { (*pPk).aiColumn }.offset(x as isize) }
                                    }) as i32;
                                    0 as i32;
                                } else {
                                    {}
                                    x = (unsafe { sqlite3StorageColumnToTable(pTab, x as i16) })
                                        as i32;
                                }
                                x = unsafe { sqlite3TableColumnToIndex(pIdx, x) };
                                if x >= (0 as i32) {
                                    unsafe {
                                        (*pOp).p2 = x;
                                    }
                                    unsafe {
                                        (*pOp).p1 = unsafe { (*pLevel).iIdxCur };
                                    }
                                    {}
                                } else {
                                    if (unsafe { (*pLoop).wsFlags })
                                        & (((64 as i32) | (67108864 as i32)) as u32)
                                        != (0 as u32)
                                    {
                                        if (unsafe { (*pLoop).wsFlags }) & ((64 as i32) as u32)
                                            != (0 as u32)
                                        {
                                            // /* An error. pLoop is supposed to be a covering index loop,
                                            //               ** and yet the VM code refers to a column of the table that
                                            //               ** is not part of the index.  */
                                            unsafe {
                                                sqlite3ErrorMsg(
                                                    pParse,
                                                    (b"internal query planner error\0".as_ptr()
                                                        as *mut i8)
                                                        as *const i8,
                                                )
                                            };
                                            unsafe {
                                                (*pParse).rc = 2 as i32;
                                            }
                                        } else {
                                            // /* The WHERE_EXPRIDX flag is set by the planner when it is likely
                                            //               ** that pLoop is a covering index loop, but it is not possible
                                            //               ** to be 100% sure. In this case, any OP_Explain opcode
                                            //               ** corresponding to this loop describes the index as a "COVERING
                                            //               ** INDEX". But, pOp proves that pLoop is not actually a covering
                                            //               ** index loop. So clear the WHERE_EXPRIDX flag and rewrite the
                                            //               ** text that accompanies the OP_Explain opcode, if any.  */
                                            let __v1940: *mut WhereLoop = pLoop;
                                            let __v1941: u32 = unsafe { (*__v1940).wsFlags };
                                            let __v1942: u32 =
                                                __v1941 & (!(67108864 as i32) as u32);
                                            unsafe {
                                                (*__v1940).wsFlags = __v1942;
                                            }
                                            unsafe {
                                                sqlite3WhereAddExplainText(
                                                    pParse,
                                                    (unsafe { (*pLevel).addrBody }) - (1 as i32),
                                                    pTabList,
                                                    pLevel,
                                                    unsafe { (*pWInfo).wctrlFlags },
                                                )
                                            };
                                        }
                                    }
                                }
                            } else {
                                if (((unsafe { (*pOp).opcode }) as u32) as i32) == (137 as i32) {
                                    unsafe {
                                        (*pOp).p1 = unsafe { (*pLevel).iIdxCur };
                                    }
                                    unsafe {
                                        (*pOp).opcode = ((144 as i32) as i8) as u8;
                                    }
                                    {}
                                } else {
                                    if (((unsafe { (*pOp).opcode }) as u32) as i32) == (20 as i32) {
                                        unsafe {
                                            (*pOp).p1 = unsafe { (*pLevel).iIdxCur };
                                        }
                                        {}
                                    }
                                }
                            }
                        }
                        let __v1943: *mut VdbeOp = pOp;
                        let __v1944: *mut VdbeOp = unsafe { __v1943.offset((1 as i32) as isize) };
                        pOp = __v1944;
                        if !(__v1944 < pLastOp) {
                            break;
                        }
                    }
                }
            }
        }
        let __v1936: i32 = i;
        let __v1937: i32 = __v1936 + (1 as i32);
        i = __v1937;
        let __v1938: *mut WhereLevel = pLevel;
        let __v1939: *mut WhereLevel = unsafe { __v1938.offset((1 as i32) as isize) };
        pLevel = __v1939;
    }
    // /* The "break" point is here, just past the end of the outer loop.
    //   ** Set it.
    //   */
    unsafe { sqlite3VdbeResolveLabel(v, unsafe { (*pWInfo).iBreak }) };
    // /* Final cleanup
    //   */
    unsafe {
        (*pParse).nQueryLoop = (unsafe { (*pWInfo).savedNQueryLoop }) as i16;
    }
    whereInfoFree(db, pWInfo);
    let __v1945: *mut Parse = pParse;
    let __v1946: u8 = unsafe { (*__v1945).withinRJSubrtn };
    let __v1947: u8 = ((((__v1946 as u32) as i32) - nRJ) as i8) as u8;
    unsafe {
        (*__v1945).withinRJSubrtn = __v1947;
    }
    return;
}

// /* RHS values for constraints. MUST BE LAST
//                                    ** Extra space is allocated to hold up
//                                    ** to nTerm such values */
// /* Size (in bytes) of a HiddenIndeInfo object sufficient to hold as
// ** many as N constraints */
// /* Forward declaration of methods */
// /*
// ** Return the estimated number of output rows from a WHERE clause
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3WhereOutputRowCount(mut pWInfo: *mut WhereInfo) -> i16 {
    return unsafe { (*pWInfo).nRowOut };
}

// /*
// ** Return one of the WHERE_DISTINCT_xxxxx values to indicate how this
// ** WHERE clause returns outputs for DISTINCT processing.
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3WhereIsDistinct(mut pWInfo: *mut WhereInfo) -> i32 {
    return ((unsafe { (*pWInfo).eDistinct }) as u32) as i32;
}

// /*
// ** Return the number of ORDER BY terms that are satisfied by the
// ** WHERE clause.  A return of 0 means that the output must be
// ** completely sorted.  A return equal to the number of ORDER BY
// ** terms means that no sorting is needed at all.  A return that
// ** is positive but less than the number of ORDER BY terms means that
// ** block sorting is required.
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3WhereIsOrdered(mut pWInfo: *mut WhereInfo) -> i32 {
    return if ((unsafe { (*pWInfo).nOBSat }) as i32) < (0 as i32) {
        0 as i32
    } else {
        (unsafe { (*pWInfo).nOBSat }) as i32
    };
}

// /*
// ** In the ORDER BY LIMIT optimization, if the inner-most loop is known
// ** to emit rows in increasing order, and if the last row emitted by the
// ** inner-most loop did not fit within the sorter, then we can skip all
// ** subsequent rows for the current iteration of the inner loop (because they
// ** will not fit in the sorter either) and continue with the second inner
// ** loop - the loop immediately outside the inner-most.
// **
// ** When a row does not fit in the sorter (because the sorter already
// ** holds LIMIT+OFFSET rows that are smaller), then a jump is made to the
// ** label returned by this function.
// **
// ** If the ORDER BY LIMIT optimization applies, the jump destination should
// ** be the continuation for the second-inner-most loop.  If the ORDER BY
// ** LIMIT optimization does not apply, then the jump destination should
// ** be the continuation for the inner-most loop.
// **
// ** It is always safe for this routine to return the continuation of the
// ** inner-most loop, in the sense that a correct answer will result.
// ** Returning the continuation the second inner loop is an optimization
// ** that might make the code run a little faster, but should not change
// ** the final answer.
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3WhereOrderByLimitOptLabel(mut pWInfo: *mut WhereInfo) -> i32 {
    let mut pInner: *mut WhereLevel = unsafe { std::mem::zeroed() };
    if !(((unsafe { (*pWInfo).__slate_bits_0.__get_bOrderedInnerLoop() }) as i32) != (0 as i32)) {
        // /* The ORDER BY LIMIT optimization does not apply.  Jump to the
        //     ** continuation of the inner-most loop. */
        return unsafe { (*pWInfo).iContinue };
    }
    pInner = unsafe {
        unsafe { std::ptr::addr_of_mut!((*pWInfo).a) as *mut WhereLevel }
            .offset(((((unsafe { (*pWInfo).nLevel }) as u32) as i32) - (1 as i32)) as isize)
    };
    0 as i32;
    return if (unsafe { (*pInner).pRJ }) != std::ptr::null_mut::<WhereRightJoin>() {
        unsafe { (*pWInfo).iContinue }
    } else {
        unsafe { (*pInner).addrNxt }
    };
}

// /*
// ** While generating code for the min/max optimization, after handling
// ** the aggregate-step call to min() or max(), check to see if any
// ** additional looping is required.  If the output order is such that
// ** we are certain that the correct answer has already been found, then
// ** code an OP_Goto to by pass subsequent processing.
// **
// ** Any extra OP_Goto that is coded here is an optimization.  The
// ** correct answer should be obtained regardless.  This OP_Goto just
// ** makes the answer appear faster.
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3WhereMinMaxOptEarlyOut(mut v: *mut Vdbe, mut pWInfo: *mut WhereInfo) {
    let mut pInner: *mut WhereLevel = unsafe { std::mem::zeroed() };
    let mut i: i32 = 0 as i32;
    if !(((unsafe { (*pWInfo).__slate_bits_0.__get_bOrderedInnerLoop() }) as i32) != (0 as i32)) {
        return;
    }
    if ((unsafe { (*pWInfo).nOBSat }) as i32) == (0 as i32) {
        return;
    }
    i = (((unsafe { (*pWInfo).nLevel }) as u32) as i32) - (1 as i32);
    '__slate_break_1718: loop {
        if !(i >= (0 as i32)) {
            break;
        }
        pInner = unsafe {
            unsafe { std::ptr::addr_of_mut!((*pWInfo).a) as *mut WhereLevel }.offset(i as isize)
        };
        if (unsafe { (*unsafe { (*pInner).pWLoop }).wsFlags }) & ((4 as i32) as u32)
            != ((0 as i32) as u32)
        {
            unsafe { sqlite3VdbeGoto(v, unsafe { (*pInner).addrNxt }) };
            return;
        }
        let __v1948: i32 = i;
        let __v1949: i32 = __v1948 - (1 as i32);
        i = __v1949;
    }
    unsafe { sqlite3VdbeGoto(v, unsafe { (*pWInfo).iBreak }) };
}

// /* The WHERE clause */
// /* ORDER BY or GROUP BY or DISTINCT clause to check */
// /* The WherePath to check */
// /* WHERE_GROUPBY or _DISTINCTBY or _ORDERBY_LIMIT */
// /* Number of entries in pPath->aLoop[] */
// /* Add this WhereLoop to the end of pPath->aLoop[] */
// /* OUT: Mask of WhereLoops to run in reverse order */
// /*
// ** If the WHERE_GROUPBY flag is set in the mask passed to sqlite3WhereBegin(),
// ** the planner assumes that the specified pOrderBy list is actually a GROUP
// ** BY clause - and so any order that groups rows as required satisfies the
// ** request.
// **
// ** Normally, in this case it is not possible for the caller to determine
// ** whether or not the rows are really being delivered in sorted order, or
// ** just in some other order that provides the required grouping. However,
// ** if the WHERE_SORTBYGROUP flag is also passed to sqlite3WhereBegin(), then
// ** this function may be called on the returned WhereInfo object. It returns
// ** true if the rows really will be sorted in the specified order, or false
// ** otherwise.
// **
// ** For example, assuming:
// **
// **   CREATE INDEX i1 ON t1(x, Y);
// **
// ** then
// **
// **   SELECT * FROM t1 GROUP BY x,y ORDER BY x,y;   -- IsSorted()==1
// **   SELECT * FROM t1 GROUP BY y,x ORDER BY y,x;   -- IsSorted()==0
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3WhereIsSorted(mut pWInfo: *mut WhereInfo) -> i32 {
    0 as i32;
    0 as i32;
    return (unsafe { (*pWInfo).__slate_bits_0.__get_sorted() }) as i32;
}

// /*
// ** Return the VDBE address or label to jump to in order to continue
// ** immediately with the next row of a WHERE clause.
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3WhereContinueLabel(mut pWInfo: *mut WhereInfo) -> i32 {
    0 as i32;
    return unsafe { (*pWInfo).iContinue };
}

// /*
// ** Return the VDBE address or label to jump to in order to break
// ** out of a WHERE loop.
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3WhereBreakLabel(mut pWInfo: *mut WhereInfo) -> i32 {
    return unsafe { (*pWInfo).iBreak };
}

// /*
// ** Return ONEPASS_OFF (0) if an UPDATE or DELETE statement is unable to
// ** operate directly on the rowids returned by a WHERE clause.  Return
// ** ONEPASS_SINGLE (1) if the statement can operation directly because only
// ** a single row is to be changed.  Return ONEPASS_MULTI (2) if the one-pass
// ** optimization can be used on multiple
// **
// ** If the ONEPASS optimization is used (if this routine returns true)
// ** then also write the indices of open cursors used by ONEPASS
// ** into aiCur[0] and aiCur[1].  iaCur[0] gets the cursor of the data
// ** table and aiCur[1] gets the cursor used by an auxiliary index.
// ** Either value may be -1, indicating that cursor is not used.
// ** Any cursors returned will have been opened for writing.
// **
// ** aiCur[0] and aiCur[1] both get -1 if the where-clause logic is
// ** unable to use the ONEPASS optimization.
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3WhereOkOnePass(mut pWInfo: *mut WhereInfo, mut aiCur: *mut i32) -> i32 {
    unsafe {
        memcpy(
            aiCur as *mut (),
            (unsafe { (*pWInfo).aiCurOnePass.as_mut_ptr() as *mut i32 }) as *const (),
            (4 as u64).wrapping_mul(((2 as i32) as i64) as u64),
        )
    };
    return ((unsafe { (*pWInfo).eOnePass }) as u32) as i32;
}

// /*
// ** Return TRUE if the WHERE loop uses the OP_DeferredSeek opcode to move
// ** the data cursor to the row selected by the index cursor.
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3WhereUsesDeferredSeek(mut pWInfo: *mut WhereInfo) -> i32 {
    return (unsafe { (*pWInfo).__slate_bits_0.__get_bDeferredSeek() }) as i32;
}

// /*
// ** Cause the prepared statement that is associated with a call to
// ** xBestIndex to potentially use all schemas.  If the statement being
// ** prepared is read-only, then just start read transactions on all
// ** schemas.  But if this is a write operation, start writes on all
// ** schemas.
// **
// ** This is used by the (built-in) sqlite_dbpage virtual table.
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3VtabUsesAllSchemas(mut pParse: *mut Parse) {
    let mut nDb: i32 = unsafe { (*unsafe { (*pParse).db }).nDb };
    let mut i: i32 = 0 as i32;
    i = 0 as i32;
    '__slate_break_1790: loop {
        if !(i < nDb) {
            break;
        }
        unsafe { sqlite3CodeVerifySchema(pParse, i) };
        let __v1950: i32 = i;
        let __v1951: i32 = __v1950 + (1 as i32);
        i = __v1951;
    }
    if (unsafe { (*pParse).writeMask }) != ((0 as i32) as u32) {
        i = 0 as i32;
        '__slate_break_1791: loop {
            if !(i < nDb) {
                break;
            }
            unsafe { sqlite3BeginWriteOperation(pParse, 0 as i32, i) };
            let __v1952: i32 = i;
            let __v1953: i32 = __v1952 + (1 as i32);
            i = __v1953;
        }
    }
}

// /* The WhereOrSet to be updated */
// /* Prerequisites of the new entry */
// /* Run-cost of the new entry */
// /* Number of outputs for the new entry */
// /*
// ** Return the bitmask for the given cursor number.  Return 0 if
// ** iCursor is not in the set.
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3WhereGetMask(mut pMaskSet: *mut WhereMaskSet, mut iCursor: i32) -> u64 {
    let mut i: i32 = 0 as i32;
    0 as i32;
    0 as i32;
    0 as i32;
    if (unsafe {
        *unsafe { unsafe { (*pMaskSet).ix.as_mut_ptr() as *mut i32 }.offset((0 as i32) as isize) }
    }) == iCursor
    {
        return ((1 as i32) as i64) as u64;
    }
    i = 1 as i32;
    '__slate_break_1721: loop {
        if !(i < unsafe { (*pMaskSet).n }) {
            break;
        }
        if (unsafe {
            *unsafe { unsafe { (*pMaskSet).ix.as_mut_ptr() as *mut i32 }.offset(i as isize) }
        }) == iCursor
        {
            return (((1 as i32) as i64) as u64) << i;
        }
        let __v1954: i32 = i;
        let __v1955: i32 = __v1954 + (1 as i32);
        i = __v1955;
    }
    return ((0 as i32) as i64) as u64;
}

// /* The WhereScan object being initialized */
// /* The WHERE clause to be scanned */
// /* Cursor to scan for */
// /* Column to scan for */
// /* Operator(s) to scan for */
// /* Must be compatible with this index */
// /*
// ** Search for a term in the WHERE clause that is of the form "X <op> <expr>"
// ** where X is a reference to the iColumn of table iCur or of index pIdx
// ** if pIdx!=0 and <op> is one of the WO_xx operator codes specified by
// ** the op parameter.  Return a pointer to the term.  Return 0 if not found.
// **
// ** If pIdx!=0 then it must be one of the indexes of table iCur.
// ** Search for terms matching the iColumn-th column of pIdx
// ** rather than the iColumn-th column of table iCur.
// **
// ** The term returned might by Y=<expr> if there is another constraint in
// ** the WHERE clause that specifies that X=Y.  Any such constraints will be
// ** identified by the WO_EQUIV bit in the pTerm->eOperator field.  The
// ** aiCur[]/iaColumn[] arrays hold X and all its equivalents. There are 11
// ** slots in aiCur[]/aiColumn[] so that means we can look for X plus up to 10
// ** other equivalent values.  Hence a search for X will return <expr> if X=A1
// ** and A1=A2 and A2=A3 and ... and A9=A10 and A10=<expr>.
// **
// ** If there are multiple terms in the WHERE clause of the form "X <op> <expr>"
// ** then try for the one with no dependencies on <expr> - in other words where
// ** <expr> is a constant expression of some kind.  Only return entries of
// ** the form "X <op> Y" where Y is a column in another table if no terms of
// ** the form "X <op> <const-expr>" exist.   If no terms with a constant RHS
// ** exist, try to return a term that does not use WO_EQUIV.
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3WhereFindTerm(
    mut pWC: *mut WhereClause,
    mut iCur: i32,
    mut iColumn: i32,
    mut notReady: u64,
    mut op: u32,
    mut pIdx: *mut Index,
) -> *mut WhereTerm {
    let mut pResult: *mut WhereTerm = std::ptr::null_mut::<WhereTerm>();
    let mut p: *mut WhereTerm = unsafe { std::mem::zeroed() };
    let mut scan: WhereScan = unsafe { std::mem::zeroed() };
    p = whereScanInit(std::ptr::addr_of_mut!(scan), pWC, iCur, iColumn, op, pIdx);
    let __v1956: u32 = op;
    let __v1957: u32 = __v1956 & (((2 as i32) | (128 as i32)) as u32);
    op = __v1957;
    '__slate_break_1726: while p != std::ptr::null_mut::<WhereTerm>() {
        if (unsafe { (*p).prereqRight }) & notReady == (((0 as i32) as i64) as u64) {
            if (unsafe { (*p).prereqRight }) == (((0 as i32) as i64) as u64)
                && ((((unsafe { (*p).eOperator }) as u32) as i32) as u32) & op
                    != ((0 as i32) as u32)
            {
                {}
                return p;
            }
            if pResult == std::ptr::null_mut::<WhereTerm>() {
                pResult = p;
            }
        }
        p = whereScanNext(std::ptr::addr_of_mut!(scan));
    }
    return pResult;
}

// /* Allocate memory that is automatically freed when pWInfo is freed.
// */
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3WhereMalloc(mut pWInfo: *mut WhereInfo, mut nByte: u64) -> *mut () {
    let mut pBlock: *mut WhereMemBlock = unsafe { std::mem::zeroed() };
    pBlock = (unsafe {
        sqlite3DbMallocRawNN(
            unsafe { (*unsafe { (*pWInfo).pParse }).db },
            nByte.wrapping_add(16 as u64),
        )
    }) as *mut WhereMemBlock;
    if pBlock != std::ptr::null_mut::<WhereMemBlock>() {
        unsafe {
            (*pBlock).pNext = unsafe { (*pWInfo).pMemToFree };
        }
        unsafe {
            (*pBlock).sz = nByte;
        }
        unsafe {
            (*pWInfo).pMemToFree = pBlock;
        }
        let __v1958: *mut WhereMemBlock = pBlock;
        let __v1959: *mut WhereMemBlock = unsafe { __v1958.offset((1 as i32) as isize) };
        pBlock = __v1959;
    }
    return pBlock as *mut ();
}

#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3WhereRealloc(
    mut pWInfo: *mut WhereInfo,
    mut pOld: *mut (),
    mut nByte: u64,
) -> *mut () {
    let mut pNew: *mut () = sqlite3WhereMalloc(pWInfo, nByte);
    if pNew != std::ptr::null_mut::<()>() && pOld != std::ptr::null_mut::<()>() {
        let mut pOldBlk: *mut WhereMemBlock = pOld as *mut WhereMemBlock;
        let __v1960: *mut WhereMemBlock = pOldBlk;
        let __v1961: *mut WhereMemBlock = unsafe { __v1960.offset(-((1 as i32) as isize)) };
        pOldBlk = __v1961;
        0 as i32;
        unsafe { memcpy(pNew, pOld as *const (), unsafe { (*pOldBlk).sz }) };
    }
    return pNew;
}

// /*
// ** Increase the memory allocation for pLoop->aLTerm[] to be at least n.
// */
fn whereLoopResize(mut db: *mut sqlite3, mut p: *mut WhereLoop, mut n: i32) -> i32 {
    let mut paNew: *mut *mut WhereTerm = unsafe { std::mem::zeroed() };
    if (((unsafe { (*p).nLSlot }) as u32) as i32) >= n {
        return 0 as i32;
    }
    n = n + (7 as i32) & !(7 as i32);
    paNew = (unsafe { sqlite3DbMallocRawNN(db, (8 as u64).wrapping_mul((n as i64) as u64)) })
        as *mut *mut WhereTerm;
    if paNew == std::ptr::null_mut::<*mut WhereTerm>() {
        return 7 as i32;
    }
    unsafe {
        memcpy(
            paNew as *mut (),
            (unsafe { (*p).aLTerm }) as *const (),
            (8 as u64).wrapping_mul(((((unsafe { (*p).nLSlot }) as u32) as i32) as i64) as u64),
        )
    };
    if (unsafe { (*p).aLTerm }) != unsafe { (*p).aLTermSpace.as_mut_ptr() as *mut *mut WhereTerm } {
        unsafe { sqlite3DbFreeNN(db, (unsafe { (*p).aLTerm }) as *mut ()) };
    }
    unsafe {
        (*p).aLTerm = paNew;
    }
    unsafe {
        (*p).nLSlot = (n as i16) as u16;
    }
    return 0 as i32;
}

// /*
// ** Move the content of pSrc into pDest
// */
fn whereOrMove(mut pDest: *mut WhereOrSet, mut pSrc: *mut WhereOrSet) {
    unsafe {
        (*pDest).n = unsafe { (*pSrc).n };
    }
    unsafe {
        memcpy(
            (unsafe { (*pDest).a.as_mut_ptr() as *mut WhereOrCost }) as *mut (),
            (unsafe { (*pSrc).a.as_mut_ptr() as *mut WhereOrCost }) as *const (),
            (((((unsafe { (*pDest).n }) as u32) as i32) as i64) as u64).wrapping_mul(16 as u64),
        )
    };
}

// /*
// ** Try to insert a new prerequisite/cost entry into the WhereOrSet pSet.
// **
// ** The new entry might overwrite an existing entry, or it might be
// ** appended, or it might be discarded.  Do whatever is the right thing
// ** so that pSet keeps the N_OR_COST best entries seen so far.
// */
fn whereOrInsert(mut pSet: *mut WhereOrSet, mut prereq: u64, mut rRun: i16, mut nOut: i16) -> i32 {
    let mut __slate_storage_1969: std::mem::MaybeUninit<u16> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1969: *mut u16 = std::ptr::addr_of_mut!(__slate_storage_1969) as *mut u16;
    let mut __slate_storage_1968: std::mem::MaybeUninit<u16> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1968: *mut u16 = std::ptr::addr_of_mut!(__slate_storage_1968) as *mut u16;
    let mut __slate_storage_1967: std::mem::MaybeUninit<*mut WhereOrSet> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1967: *mut *mut WhereOrSet =
        std::ptr::addr_of_mut!(__slate_storage_1967) as *mut *mut WhereOrSet;
    let mut __slate_storage_1971: std::mem::MaybeUninit<u16> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1971: *mut u16 = std::ptr::addr_of_mut!(__slate_storage_1971) as *mut u16;
    let mut __slate_storage_1970: std::mem::MaybeUninit<u16> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1970: *mut u16 = std::ptr::addr_of_mut!(__slate_storage_1970) as *mut u16;
    let mut __slate_storage_1966: std::mem::MaybeUninit<*mut WhereOrCost> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1966: *mut *mut WhereOrCost =
        std::ptr::addr_of_mut!(__slate_storage_1966) as *mut *mut WhereOrCost;
    let mut __slate_storage_1965: std::mem::MaybeUninit<*mut WhereOrCost> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1965: *mut *mut WhereOrCost =
        std::ptr::addr_of_mut!(__slate_storage_1965) as *mut *mut WhereOrCost;
    let mut __slate_storage_1964: std::mem::MaybeUninit<u16> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1964: *mut u16 = std::ptr::addr_of_mut!(__slate_storage_1964) as *mut u16;
    let mut __slate_storage_1963: std::mem::MaybeUninit<u16> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1963: *mut u16 = std::ptr::addr_of_mut!(__slate_storage_1963) as *mut u16;
    let mut __slate_storage_1962: std::mem::MaybeUninit<*mut WhereOrCost> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1962: *mut *mut WhereOrCost =
        std::ptr::addr_of_mut!(__slate_storage_1962) as *mut *mut WhereOrCost;
    let mut __slate_storage_570: std::mem::MaybeUninit<*mut WhereOrCost> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_570: *mut *mut WhereOrCost =
        std::ptr::addr_of_mut!(__slate_storage_570) as *mut *mut WhereOrCost;
    let mut __slate_storage_569: std::mem::MaybeUninit<u16> = std::mem::MaybeUninit::uninit();
    let __slate_slot_569: *mut u16 = std::ptr::addr_of_mut!(__slate_storage_569) as *mut u16;
    unsafe {
        *__slate_slot_569 = unsafe { (*pSet).n };
        std::ptr::write(__slate_slot_1962, unsafe {
            (*pSet).a.as_mut_ptr() as *mut WhereOrCost
        });
        *__slate_slot_570 = *__slate_slot_1962;
        '__join_2: {
            loop {
                if ((*__slate_slot_569 as u32) as i32) > (0 as i32) {
                    if (rRun as i32) <= ((unsafe { (*(*__slate_slot_570)).rRun }) as i32)
                        && prereq & unsafe { (*(*__slate_slot_570)).prereq } == prereq
                    {
                        break '__join_2;
                    } else {
                        if ((unsafe { (*(*__slate_slot_570)).rRun }) as i32) <= (rRun as i32)
                            && (unsafe { (*(*__slate_slot_570)).prereq }) & prereq
                                == unsafe { (*(*__slate_slot_570)).prereq }
                        {
                            return 0 as i32;
                        } else {
                            std::ptr::write(__slate_slot_1963, *__slate_slot_569);
                            std::ptr::write(
                                __slate_slot_1964,
                                ((((*__slate_slot_1963 as u32) as i32) - (1 as i32)) as i16) as u16,
                            );
                            *__slate_slot_569 = *__slate_slot_1964;
                            std::ptr::write(__slate_slot_1965, *__slate_slot_570);
                            std::ptr::write(__slate_slot_1966, unsafe {
                                (*__slate_slot_1965).offset((1 as i32) as isize)
                            });
                            *__slate_slot_570 = *__slate_slot_1966;
                        }
                    }
                } else {
                    break;
                }
            }
            if (((unsafe { (*pSet).n }) as u32) as i32) < (3 as i32) {
                std::ptr::write(__slate_slot_1967, pSet);
                std::ptr::write(__slate_slot_1968, unsafe { (*(*__slate_slot_1967)).n });
                std::ptr::write(
                    __slate_slot_1969,
                    ((((*__slate_slot_1968 as u32) as i32) + (1 as i32)) as i16) as u16,
                );
                unsafe {
                    (*(*__slate_slot_1967)).n = *__slate_slot_1969;
                }
                *__slate_slot_570 = unsafe {
                    unsafe { (*pSet).a.as_mut_ptr() as *mut WhereOrCost }
                        .offset(((*__slate_slot_1968 as u32) as i32) as isize)
                };
                unsafe {
                    (*(*__slate_slot_570)).nOut = nOut;
                }
            } else {
                *__slate_slot_570 = unsafe { (*pSet).a.as_mut_ptr() as *mut WhereOrCost };
                *__slate_slot_569 = ((1 as i32) as i16) as u16;
                loop {
                    if ((*__slate_slot_569 as u32) as i32)
                        < (((unsafe { (*pSet).n }) as u32) as i32)
                    {
                        '__join_7: {
                            if ((unsafe { (*(*__slate_slot_570)).rRun }) as i32)
                                > ((unsafe {
                                    (*unsafe {
                                        unsafe { (*pSet).a.as_mut_ptr() as *mut WhereOrCost }
                                            .offset(((*__slate_slot_569 as u32) as i32) as isize)
                                    })
                                    .rRun
                                }) as i32)
                            {
                                *__slate_slot_570 = unsafe {
                                    unsafe { (*pSet).a.as_mut_ptr() as *mut WhereOrCost }
                                        .offset(((*__slate_slot_569 as u32) as i32) as isize)
                                };
                            }
                        }
                        std::ptr::write(__slate_slot_1970, *__slate_slot_569);
                        std::ptr::write(
                            __slate_slot_1971,
                            ((((*__slate_slot_1970 as u32) as i32) + (1 as i32)) as i16) as u16,
                        );
                        *__slate_slot_569 = *__slate_slot_1971;
                    } else {
                        break;
                    }
                }
                if ((unsafe { (*(*__slate_slot_570)).rRun }) as i32) <= (rRun as i32) {
                    return 0 as i32;
                }
            }
        }
        unsafe {
            (*(*__slate_slot_570)).prereq = prereq;
        }
        unsafe {
            (*(*__slate_slot_570)).rRun = rRun;
        }
        if ((unsafe { (*(*__slate_slot_570)).nOut }) as i32) > (nOut as i32) {
            unsafe {
                (*(*__slate_slot_570)).nOut = nOut;
            }
        }
        return 1 as i32;
    }
    return unsafe { std::mem::zeroed() };
}

// /*
// ** Create a new mask for cursor iCursor.
// **
// ** There is one cursor per table in the FROM clause.  The number of
// ** tables in the FROM clause is limited by a test early in the
// ** sqlite3WhereBegin() routine.  So we know that the pMaskSet->ix[]
// ** array will never overflow.
// */
fn createMask(mut pMaskSet: *mut WhereMaskSet, mut iCursor: i32) {
    0 as i32;
    let __v1972: *mut WhereMaskSet = pMaskSet;
    let __v1973: i32 = unsafe { (*__v1972).n };
    let __v1974: i32 = __v1973 + (1 as i32);
    unsafe {
        (*__v1972).n = __v1974;
    }
    unsafe {
        *unsafe { unsafe { (*pMaskSet).ix.as_mut_ptr() as *mut i32 }.offset(__v1973 as isize) } =
            iCursor;
    }
}

// /*
// ** If the right-hand branch of the expression is a TK_COLUMN, then return
// ** a pointer to the right-hand branch.  Otherwise, return NULL.
// */
fn whereRightSubexprIsColumn(mut p: *mut Expr) -> *mut Expr {
    p = unsafe { sqlite3ExprSkipCollateAndLikely(unsafe { (*p).pRight }) };
    if p != std::ptr::null_mut::<Expr>()
        && (((unsafe { (*p).op }) as u32) as i32) == (168 as i32)
        && !((unsafe { (*p).flags }) & ((32 as i32) as u32) != ((0 as i32) as u32))
    {
        return p;
    }
    return std::ptr::null_mut::<Expr>();
}

// /*
// ** Term pTerm is guaranteed to be a WO_IN term. It may be a component term
// ** of a vector IN expression of the form "(x, y, ...) IN (SELECT ...)".
// ** This function checks to see if the term is compatible with an index
// ** column with affinity idxaff (one of the SQLITE_AFF_XYZ values). If so,
// ** it returns a pointer to the name of the collation sequence (e.g. "BINARY"
// ** or "NOCASE") used by the comparison in pTerm. If it is not compatible
// ** with affinity idxaff, NULL is returned.
// */
fn indexInAffinityOk(
    mut pParse: *mut Parse,
    mut pTerm: *mut WhereTerm,
    mut idxaff: u8,
) -> *const i8 {
    let mut pX: *mut Expr = unsafe { (*pTerm).pExpr };
    let mut inexpr: Expr = unsafe { std::mem::zeroed() };
    0 as i32;
    if (unsafe { sqlite3ExprIsVector((unsafe { (*pX).pLeft }) as *const Expr) }) != (0 as i32) {
        let mut iField: i32 = (unsafe { (*pTerm).u.x.iField }) - (1 as i32);
        inexpr.flags = (0 as i32) as u32;
        inexpr.op = ((54 as i32) as i8) as u8;
        inexpr.pLeft = unsafe {
            (*unsafe {
                unsafe {
                    std::ptr::addr_of_mut!((*unsafe { (*unsafe { (*pX).pLeft }).x.pList }).a)
                        as *mut ExprList_item
                }
                .offset(iField as isize)
            })
            .pExpr
        };
        0 as i32;
        inexpr.pRight = unsafe {
            (*unsafe {
                unsafe {
                    std::ptr::addr_of_mut!((*unsafe { (*unsafe { (*pX).x.pSelect }).pEList }).a)
                        as *mut ExprList_item
                }
                .offset(iField as isize)
            })
            .pExpr
        };
        pX = std::ptr::addr_of_mut!(inexpr);
    }
    if (unsafe { sqlite3IndexAffinityOk(pX as *const Expr, idxaff as i8) }) != (0 as i32) {
        let mut pRet: *mut CollSeq =
            unsafe { sqlite3ExprCompareCollSeq(pParse, pX as *const Expr) };
        return if pRet != std::ptr::null_mut::<CollSeq>() {
            (unsafe { (*pRet).zName }) as *const i8
        } else {
            unsafe { std::ptr::addr_of!(sqlite3StrBINARY) as *const i8 }
        };
    }
    return std::ptr::null::<i8>();
}

// /*
// ** Advance to the next WhereTerm that matches according to the criteria
// ** established when the pScan object was initialized by whereScanInit().
// ** Return NULL if there are no more matching WhereTerms.
// */
fn whereScanNext(mut pScan: *mut WhereScan) -> *mut WhereTerm {
    // /* The cursor on the LHS of the term */
    let mut iCur: i32 = 0 as i32;
    // /* The column on the LHS of the term.  -1 for IPK */
    let mut iColumn: i16 = 0 as i16;
    // /* An expression being tested */
    let mut pX: *mut Expr = unsafe { std::mem::zeroed() };
    // /* Shorthand for pScan->pWC */
    let mut pWC: *mut WhereClause = unsafe { std::mem::zeroed() };
    // /* The term being tested */
    let mut pTerm: *mut WhereTerm = unsafe { std::mem::zeroed() };
    // /* Where to start scanning */
    let mut k: i32 = unsafe { (*pScan).k };
    0 as i32;
    pWC = unsafe { (*pScan).pWC };
    '__slate_break_1722: while (1 as i32) != (0 as i32) {
        iColumn = unsafe {
            *unsafe {
                unsafe { (*pScan).aiColumn.as_mut_ptr() as *mut i16 }
                    .offset(((((unsafe { (*pScan).iEquiv }) as u32) as i32) - (1 as i32)) as isize)
            }
        };
        iCur = unsafe {
            *unsafe {
                unsafe { (*pScan).aiCur.as_mut_ptr() as *mut i32 }
                    .offset(((((unsafe { (*pScan).iEquiv }) as u32) as i32) - (1 as i32)) as isize)
            }
        };
        0 as i32;
        0 as i32;
        '__slate_break_1723: loop {
            pTerm = unsafe { unsafe { (*pWC).a }.offset(k as isize) };
            '__slate_break_1724: while k < unsafe { (*pWC).nTerm } {
                '__slate_continue_1724: {
                    0 as i32;
                    let __v1979: bool;
                    if (unsafe { (*pTerm).leftCursor }) == iCur
                        && (unsafe { (*pTerm).u.x.leftColumn }) == (iColumn as i32)
                    {
                        let __v1980: bool;
                        if (iColumn as i32) != -(2 as i32) {
                            __v1980 = true as bool;
                        } else {
                            __v1980 = (unsafe {
                                sqlite3ExprCompareSkip(
                                    unsafe { (*unsafe { (*pTerm).pExpr }).pLeft },
                                    unsafe { (*pScan).pIdxExpr },
                                    iCur,
                                )
                            }) == (0 as i32);
                        }
                        __v1979 = __v1980;
                    } else {
                        __v1979 = false as bool;
                    }
                    if __v1979
                        && ((((unsafe { (*pScan).iEquiv }) as u32) as i32) <= (1 as i32)
                            || !((unsafe { (*unsafe { (*pTerm).pExpr }).flags })
                                & ((1 as i32) as u32)
                                != ((0 as i32) as u32)))
                    {
                        let __v1981: bool;
                        if (((unsafe { (*pTerm).eOperator }) as u32) as i32) & (2048 as i32)
                            != (0 as i32)
                            && (((unsafe { (*pScan).nEquiv }) as u32) as i32)
                                < ((((44 as u64) / (4 as u64)) as u32) as i32)
                        {
                            let __v1982: *mut Expr =
                                whereRightSubexprIsColumn(unsafe { (*pTerm).pExpr });
                            pX = __v1982;
                            __v1981 = __v1982 != std::ptr::null_mut::<Expr>();
                        } else {
                            __v1981 = false as bool;
                        }
                        if __v1981 {
                            let mut j: i32 = 0 as i32;
                            j = 0 as i32;
                            '__slate_break_1725: loop {
                                if !(j < (((unsafe { (*pScan).nEquiv }) as u32) as i32)) {
                                    break;
                                }
                                if (unsafe {
                                    *unsafe {
                                        unsafe { (*pScan).aiCur.as_mut_ptr() as *mut i32 }
                                            .offset(j as isize)
                                    }
                                }) == unsafe { (*pX).iTable }
                                    && ((unsafe {
                                        *unsafe {
                                            unsafe { (*pScan).aiColumn.as_mut_ptr() as *mut i16 }
                                                .offset(j as isize)
                                        }
                                    }) as i32)
                                        == ((unsafe { (*pX).iColumn }) as i32)
                                {
                                    break '__slate_break_1725;
                                }
                                let __v1983: i32 = j;
                                let __v1984: i32 = __v1983 + (1 as i32);
                                j = __v1984;
                            }
                            if j == (((unsafe { (*pScan).nEquiv }) as u32) as i32) {
                                unsafe {
                                    *unsafe {
                                        unsafe { (*pScan).aiCur.as_mut_ptr() as *mut i32 }
                                            .offset(j as isize)
                                    } = unsafe { (*pX).iTable };
                                }
                                unsafe {
                                    *unsafe {
                                        unsafe { (*pScan).aiColumn.as_mut_ptr() as *mut i16 }
                                            .offset(j as isize)
                                    } = unsafe { (*pX).iColumn };
                                }
                                let __v1985: *mut WhereScan = pScan;
                                let __v1986: u8 = unsafe { (*__v1985).nEquiv };
                                let __v1987: u8 =
                                    ((((__v1986 as u32) as i32) + (1 as i32)) as i8) as u8;
                                unsafe {
                                    (*__v1985).nEquiv = __v1987;
                                }
                            }
                        }
                        if ((((unsafe { (*pTerm).eOperator }) as u32) as i32) as u32)
                            & unsafe { (*pScan).opMask }
                            != ((0 as i32) as u32)
                        {
                            // /* Verify the affinity and collating sequence match */
                            if (unsafe { (*pScan).zCollName }) != std::ptr::null::<i8>()
                                && (((unsafe { (*pTerm).eOperator }) as u32) as i32) & (256 as i32)
                                    == (0 as i32)
                            {
                                let mut zCollName: *const i8 = unsafe { std::mem::zeroed() };
                                let mut pParse: *mut Parse =
                                    unsafe { (*unsafe { (*pWC).pWInfo }).pParse };
                                pX = unsafe { (*pTerm).pExpr };
                                if (((unsafe { (*pTerm).eOperator }) as u32) as i32) & (1 as i32)
                                    != (0 as i32)
                                {
                                    zCollName = indexInAffinityOk(
                                        pParse,
                                        pTerm,
                                        (unsafe { (*pScan).idxaff }) as u8,
                                    );
                                    if !(zCollName != std::ptr::null::<i8>()) {
                                        break '__slate_continue_1724;
                                    }
                                } else {
                                    let mut pColl: *mut CollSeq = unsafe { std::mem::zeroed() };
                                    if !((unsafe {
                                        sqlite3IndexAffinityOk(pX as *const Expr, unsafe {
                                            (*pScan).idxaff
                                        })
                                    }) != (0 as i32))
                                    {
                                        break '__slate_continue_1724;
                                    }
                                    0 as i32;
                                    pColl = unsafe {
                                        sqlite3ExprCompareCollSeq(pParse, pX as *const Expr)
                                    };
                                    zCollName = if pColl != std::ptr::null_mut::<CollSeq>() {
                                        (unsafe { (*pColl).zName }) as *const i8
                                    } else {
                                        unsafe { std::ptr::addr_of!(sqlite3StrBINARY) as *const i8 }
                                    };
                                }
                                if (unsafe {
                                    sqlite3StrICmp(zCollName, unsafe { (*pScan).zCollName })
                                }) != (0 as i32)
                                {
                                    break '__slate_continue_1724;
                                }
                            }
                            let __v1988: bool;
                            if (((unsafe { (*pTerm).eOperator }) as u32) as i32)
                                & ((2 as i32) | (128 as i32))
                                != (0 as i32)
                            {
                                pX = unsafe { (*unsafe { (*pTerm).pExpr }).pRight };
                                __v1988 = pX != std::ptr::null_mut::<Expr>();
                            } else {
                                __v1988 = false as bool;
                            }
                            if __v1988
                                && (((unsafe { (*pX).op }) as u32) as i32) == (168 as i32)
                                && (unsafe { (*pX).iTable })
                                    == unsafe {
                                        *unsafe {
                                            unsafe { (*pScan).aiCur.as_mut_ptr() as *mut i32 }
                                                .offset((0 as i32) as isize)
                                        }
                                    }
                                && ((unsafe { (*pX).iColumn }) as i32)
                                    == ((unsafe {
                                        *unsafe {
                                            unsafe { (*pScan).aiColumn.as_mut_ptr() as *mut i16 }
                                                .offset((0 as i32) as isize)
                                        }
                                    }) as i32)
                            {
                                {}
                            } else {
                                unsafe {
                                    (*pScan).pWC = pWC;
                                }
                                unsafe {
                                    (*pScan).k = k + (1 as i32);
                                }
                                return pTerm;
                            }
                        }
                    }
                }
                let __v1975: i32 = k;
                let __v1976: i32 = __v1975 + (1 as i32);
                k = __v1976;
                let __v1977: *mut WhereTerm = pTerm;
                let __v1978: *mut WhereTerm = unsafe { __v1977.offset((1 as i32) as isize) };
                pTerm = __v1978;
            }
            pWC = unsafe { (*pWC).pOuter };
            k = 0 as i32;
            if !(pWC != std::ptr::null_mut::<WhereClause>()) {
                break;
            }
        }
        if (((unsafe { (*pScan).iEquiv }) as u32) as i32)
            >= (((unsafe { (*pScan).nEquiv }) as u32) as i32)
        {
            break '__slate_break_1722;
        }
        pWC = unsafe { (*pScan).pOrigWC };
        k = 0 as i32;
        let __v1989: *mut WhereScan = pScan;
        let __v1990: u8 = unsafe { (*__v1989).iEquiv };
        let __v1991: u8 = ((((__v1990 as u32) as i32) + (1 as i32)) as i8) as u8;
        unsafe {
            (*__v1989).iEquiv = __v1991;
        }
    }
    return std::ptr::null_mut::<WhereTerm>();
}

// /*
// ** This is whereScanInit() for the case of an index on an expression.
// ** It is factored out into a separate tail-recursion subroutine so that
// ** the normal whereScanInit() routine, which is a high-runner, does not
// ** need to push registers onto the stack as part of its prologue.
// */
fn whereScanInitIndexExpr(mut pScan: *mut WhereScan) -> *mut WhereTerm {
    unsafe {
        (*pScan).idxaff =
            unsafe { sqlite3ExprAffinity((unsafe { (*pScan).pIdxExpr }) as *const Expr) };
    }
    return whereScanNext(pScan);
}

// /*
// ** Initialize a WHERE clause scanner object.  Return a pointer to the
// ** first match.  Return NULL if there are no matches.
// **
// ** The scanner will be searching the WHERE clause pWC.  It will look
// ** for terms of the form "X <op> <expr>" where X is column iColumn of table
// ** iCur.   Or if pIdx!=0 then X is column iColumn of index pIdx.  pIdx
// ** must be one of the indexes of table iCur.
// **
// ** The <op> must be one of the operators described by opMask.
// **
// ** If the search is for X and the WHERE clause contains terms of the
// ** form X=Y then this routine might also return terms of the form
// ** "Y <op> <expr>".  The number of levels of transitivity is limited,
// ** but is enough to handle most commonly occurring SQL statements.
// **
// ** If X is not the INTEGER PRIMARY KEY then X must be compatible with
// ** index pIdx.
// */
fn whereScanInit(
    mut pScan: *mut WhereScan,
    mut pWC: *mut WhereClause,
    mut iCur: i32,
    mut iColumn: i32,
    mut opMask: u32,
    mut pIdx: *mut Index,
) -> *mut WhereTerm {
    unsafe {
        (*pScan).pOrigWC = pWC;
    }
    unsafe {
        (*pScan).pWC = pWC;
    }
    unsafe {
        (*pScan).pIdxExpr = std::ptr::null_mut::<Expr>();
    }
    unsafe {
        (*pScan).idxaff = (0 as i32) as i8;
    }
    unsafe {
        (*pScan).zCollName = std::ptr::null::<i8>();
    }
    unsafe {
        (*pScan).opMask = opMask;
    }
    unsafe {
        (*pScan).k = 0 as i32;
    }
    unsafe {
        *unsafe {
            unsafe { (*pScan).aiCur.as_mut_ptr() as *mut i32 }.offset((0 as i32) as isize)
        } = iCur;
    }
    unsafe {
        (*pScan).nEquiv = ((1 as i32) as i8) as u8;
    }
    unsafe {
        (*pScan).iEquiv = ((1 as i32) as i8) as u8;
    }
    if pIdx != std::ptr::null_mut::<Index>() {
        let mut j: i32 = iColumn;
        iColumn = (unsafe { *unsafe { unsafe { (*pIdx).aiColumn }.offset(j as isize) } }) as i32;
        if iColumn == ((unsafe { (*unsafe { (*pIdx).pTable }).iPKey }) as i32) {
            iColumn = -(1 as i32);
        } else {
            if iColumn >= (0 as i32) {
                unsafe {
                    (*pScan).idxaff = unsafe {
                        (*unsafe {
                            unsafe { (*unsafe { (*pIdx).pTable }).aCol }.offset(iColumn as isize)
                        })
                        .affinity
                    };
                }
                unsafe {
                    (*pScan).zCollName =
                        unsafe { *unsafe { unsafe { (*pIdx).azColl }.offset(j as isize) } };
                }
            } else {
                if iColumn == -(2 as i32) {
                    unsafe {
                        (*pScan).pIdxExpr = unsafe {
                            (*unsafe {
                                unsafe {
                                    std::ptr::addr_of_mut!((*unsafe { (*pIdx).aColExpr }).a)
                                        as *mut ExprList_item
                                }
                                .offset(j as isize)
                            })
                            .pExpr
                        };
                    }
                    unsafe {
                        (*pScan).zCollName =
                            unsafe { *unsafe { unsafe { (*pIdx).azColl }.offset(j as isize) } };
                    }
                    unsafe {
                        *unsafe {
                            unsafe { (*pScan).aiColumn.as_mut_ptr() as *mut i16 }
                                .offset((0 as i32) as isize)
                        } = -(2 as i32) as i16;
                    }
                    return whereScanInitIndexExpr(pScan);
                }
            }
        }
    } else {
        if iColumn == -(2 as i32) {
            return std::ptr::null_mut::<WhereTerm>();
        }
    }
    unsafe {
        *unsafe {
            unsafe { (*pScan).aiColumn.as_mut_ptr() as *mut i16 }.offset((0 as i32) as isize)
        } = iColumn as i16;
    }
    return whereScanNext(pScan);
}

// /* The WHERE clause to be searched */
// /* Cursor number of LHS */
// /* Column number of LHS */
// /* RHS must not overlap with this mask */
// /* Mask of WO_xx values describing operator */
// /* Must be compatible with this index, if not NULL */
// /*
// ** This function searches pList for an entry that matches the iCol-th column
// ** of index pIdx.
// **
// ** If such an expression is found, its index in pList->a[] is returned. If
// ** no expression is found, -1 is returned.
// */
fn findIndexCol(
    mut pParse: *mut Parse,
    mut pList: *mut ExprList,
    mut iBase: i32,
    mut pIdx: *mut Index,
    mut iCol: i32,
) -> i32 {
    let mut i: i32 = 0 as i32;
    let mut zColl: *const i8 =
        unsafe { *unsafe { unsafe { (*pIdx).azColl }.offset(iCol as isize) } };
    i = 0 as i32;
    '__slate_break_1727: loop {
        if !(i < unsafe { (*pList).nExpr }) {
            break;
        }
        let mut p: *mut Expr = unsafe {
            sqlite3ExprSkipCollateAndLikely(unsafe {
                (*unsafe {
                    unsafe { std::ptr::addr_of_mut!((*pList).a) as *mut ExprList_item }
                        .offset(i as isize)
                })
                .pExpr
            })
        };
        if p != std::ptr::null_mut::<Expr>()
            && ((((unsafe { (*p).op }) as u32) as i32) == (168 as i32)
                || (((unsafe { (*p).op }) as u32) as i32) == (170 as i32))
            && ((unsafe { (*p).iColumn }) as i32)
                == ((unsafe { *unsafe { unsafe { (*pIdx).aiColumn }.offset(iCol as isize) } })
                    as i32)
            && (unsafe { (*p).iTable }) == iBase
        {
            let mut pColl: *mut CollSeq = unsafe {
                sqlite3ExprNNCollSeq(
                    pParse,
                    (unsafe {
                        (*unsafe {
                            unsafe { std::ptr::addr_of_mut!((*pList).a) as *mut ExprList_item }
                                .offset(i as isize)
                        })
                        .pExpr
                    }) as *const Expr,
                )
            };
            if (0 as i32)
                == unsafe { sqlite3StrICmp((unsafe { (*pColl).zName }) as *const i8, zColl) }
            {
                return i;
            }
        }
        let __v1992: i32 = i;
        let __v1993: i32 = __v1992 + (1 as i32);
        i = __v1993;
    }
    return -(1 as i32);
}

// /* Parse context */
// /* Expression list to search */
// /* Cursor for table associated with pIdx */
// /* Index to match column of */
// /* Column of index to match */
// /*
// ** Return TRUE if the iCol-th column of index pIdx is NOT NULL
// */
fn indexColumnNotNull(mut pIdx: *mut Index, mut iCol: i32) -> i32 {
    let mut j: i32 = 0 as i32;
    0 as i32;
    0 as i32;
    j = (unsafe { *unsafe { unsafe { (*pIdx).aiColumn }.offset(iCol as isize) } }) as i32;
    if j >= (0 as i32) {
        return (unsafe {
            (*unsafe { unsafe { (*unsafe { (*pIdx).pTable }).aCol }.offset(j as isize) })
                .__slate_bits_0
                .__get_notNull()
        }) as i32;
    } else {
        if j == -(1 as i32) {
            return 1 as i32;
        } else {
            0 as i32;
            // /* Assume an indexed expression can always yield a NULL */
            return 0 as i32;
        }
    }
    return unsafe { std::mem::zeroed() };
}

// /*
// ** Return true if the DISTINCT expression-list passed as the third argument
// ** is redundant.
// **
// ** A DISTINCT list is redundant if any subset of the columns in the
// ** DISTINCT list are collectively unique and individually non-null.
// */
fn isDistinctRedundant(
    mut pParse: *mut Parse,
    mut pTabList: *mut SrcList,
    mut pWC: *mut WhereClause,
    mut pDistinct: *mut ExprList,
) -> i32 {
    let mut pTab: *mut Table = unsafe { std::mem::zeroed() };
    let mut pIdx: *mut Index = unsafe { std::mem::zeroed() };
    let mut i: i32 = 0 as i32;
    let mut iBase: i32 = 0 as i32;
    // /* If there is more than one table or sub-select in the FROM clause of
    //   ** this query, then it will not be possible to show that the DISTINCT
    //   ** clause is redundant. */
    if (unsafe { (*pTabList).nSrc }) != (1 as i32) {
        return 0 as i32;
    }
    iBase = unsafe {
        (*unsafe {
            unsafe { std::ptr::addr_of_mut!((*pTabList).a) as *mut SrcItem }
                .offset((0 as i32) as isize)
        })
        .iCursor
    };
    pTab = unsafe {
        (*unsafe {
            unsafe { std::ptr::addr_of_mut!((*pTabList).a) as *mut SrcItem }
                .offset((0 as i32) as isize)
        })
        .pSTab
    };
    // /* If any of the expressions is an IPK column on table iBase, then return
    //   ** true. Note: The (p->iTable==iBase) part of this test may be false if the
    //   ** current SELECT is a correlated sub-query.
    //   */
    i = 0 as i32;
    '__slate_break_1728: loop {
        if !(i < unsafe { (*pDistinct).nExpr }) {
            break;
        }
        let mut p: *mut Expr = unsafe {
            sqlite3ExprSkipCollateAndLikely(unsafe {
                (*unsafe {
                    unsafe { std::ptr::addr_of_mut!((*pDistinct).a) as *mut ExprList_item }
                        .offset(i as isize)
                })
                .pExpr
            })
        };
        if p == std::ptr::null_mut::<Expr>() {
        } else {
            if (((unsafe { (*p).op }) as u32) as i32) != (168 as i32)
                && (((unsafe { (*p).op }) as u32) as i32) != (170 as i32)
            {
            } else {
                if (unsafe { (*p).iTable }) == iBase
                    && ((unsafe { (*p).iColumn }) as i32) < (0 as i32)
                {
                    return 1 as i32;
                }
            }
        }
        let __v1994: i32 = i;
        let __v1995: i32 = __v1994 + (1 as i32);
        i = __v1995;
    }
    // /* Loop through all indices on the table, checking each to see if it makes
    //   ** the DISTINCT qualifier redundant. It does so if:
    //   **
    //   **   1. The index is itself UNIQUE, and
    //   **
    //   **   2. All of the columns in the index are either part of the pDistinct
    //   **      list, or else the WHERE clause contains a term of the form "col=X",
    //   **      where X is a constant value. The collation sequences of the
    //   **      comparison and select-list expressions must match those of the index.
    //   **
    //   **   3. All of those index columns for which the WHERE clause does not
    //   **      contain a "col=X" term are subject to a NOT NULL constraint.
    //   */
    pIdx = unsafe { (*pTab).pIndex };
    '__slate_break_1729: while pIdx != std::ptr::null_mut::<Index>() {
        if !((((unsafe { (*pIdx).onError }) as u32) as i32) != (0 as i32)) {
        } else {
            if (unsafe { (*pIdx).pPartIdxWhere }) != std::ptr::null_mut::<Expr>() {
            } else {
                i = 0 as i32;
                '__slate_break_1730: loop {
                    if !(i < (((unsafe { (*pIdx).nKeyCol }) as u32) as i32)) {
                        break;
                    }
                    if std::ptr::null_mut::<WhereTerm>()
                        == sqlite3WhereFindTerm(
                            pWC,
                            iBase,
                            i,
                            !(((0 as i32) as i64) as u64),
                            (2 as i32) as u32,
                            pIdx,
                        )
                    {
                        if findIndexCol(pParse, pDistinct, iBase, pIdx, i) < (0 as i32) {
                            break '__slate_break_1730;
                        }
                        if indexColumnNotNull(pIdx, i) == (0 as i32) {
                            break '__slate_break_1730;
                        }
                    }
                    let __v1996: i32 = i;
                    let __v1997: i32 = __v1996 + (1 as i32);
                    i = __v1997;
                }
                if i == (((unsafe { (*pIdx).nKeyCol }) as u32) as i32) {
                    // /* This index implies that the DISTINCT qualifier is redundant. */
                    return 1 as i32;
                }
            }
        }
        pIdx = unsafe { (*pIdx).pNext };
    }
    return 0 as i32;
}

// /* Parsing context */
// /* The FROM clause */
// /* The WHERE clause */
// /* The result set that needs to be DISTINCT */
// /*
// ** Estimate the logarithm of the input value to base 2.
// */
fn estLog(mut N: i16) -> i16 {
    let __v1998: i32;
    if (N as i32) <= (10 as i32) {
        __v1998 = 0 as i32;
    } else {
        __v1998 = ((unsafe { sqlite3LogEst((N as i64) as u64) }) as i32) - (33 as i32);
    }
    return __v1998 as i16;
}

// /*
// ** Convert OP_Column opcodes to OP_Copy in previously generated code.
// **
// ** This routine runs over generated VDBE code and translates OP_Column
// ** opcodes into OP_Copy when the table is being accessed via co-routine
// ** instead of via table lookup.
// **
// ** If the iAutoidxCur is not zero, then any OP_Rowid instructions on
// ** cursor iTabCur are transformed into OP_Sequence opcode for the
// ** iAutoidxCur cursor, in order to generate unique rowids for the
// ** automatic index being generated.
// */
fn translateColumnToCopy(
    mut pParse: *mut Parse,
    mut iStart: i32,
    mut iTabCur: i32,
    mut iRegister: i32,
    mut iAutoidxCur: i32,
) {
    let mut v: *mut Vdbe = unsafe { (*pParse).pVdbe };
    let mut pOp: *mut VdbeOp = unsafe { sqlite3VdbeGetOp(v, iStart) };
    let mut iEnd: i32 = unsafe { sqlite3VdbeCurrentAddr(v) };
    if (unsafe { (*unsafe { (*pParse).db }).mallocFailed }) != (0 as u8) {
        return;
    }
    '__slate_break_1731: while iStart < iEnd {
        if (unsafe { (*pOp).p1 }) != iTabCur {
        } else {
            if (((unsafe { (*pOp).opcode }) as u32) as i32) == (96 as i32) {
                unsafe {
                    (*pOp).opcode = ((82 as i32) as i8) as u8;
                }
                unsafe {
                    (*pOp).p1 = (unsafe { (*pOp).p2 }) + iRegister;
                }
                unsafe {
                    (*pOp).p2 = unsafe { (*pOp).p3 };
                }
                unsafe {
                    (*pOp).p3 = 0 as i32;
                }
                // /* Cause the MEM_Subtype flag to be cleared */
                unsafe {
                    (*pOp).p5 = ((2 as i32) as i16) as u16;
                }
            } else {
                if (((unsafe { (*pOp).opcode }) as u32) as i32) == (137 as i32) {
                    unsafe {
                        (*pOp).opcode = ((128 as i32) as i8) as u8;
                    }
                    unsafe {
                        (*pOp).p1 = iAutoidxCur;
                    }
                }
            }
        }
        let __v1999: i32 = iStart;
        let __v2000: i32 = __v1999 + (1 as i32);
        iStart = __v2000;
        let __v2001: *mut VdbeOp = pOp;
        let __v2002: *mut VdbeOp = unsafe { __v2001.offset((1 as i32) as isize) };
        pOp = __v2002;
    }
}

// /* Parsing context */
// /* Translate from this opcode to the end */
// /* OP_Column/OP_Rowid references to this table */
// /* The first column is in this register */
// /* If non-zero, cursor of autoindex being generated */
// /*
// ** Two routines for printing the content of an sqlite3_index_info
// ** structure.  Used for testing and debugging only.  If neither
// ** SQLITE_TEST or SQLITE_DEBUG are defined, then these routines
// ** are no-ops.
// */
// /*
// ** We know that pSrc is an operand of an outer join.  Return true if
// ** pTerm is a constraint that is compatible with that join.
// **
// ** pTerm must be EP_OuterON if pSrc is the right operand of an
// ** outer join.  pTerm can be either EP_OuterON or EP_InnerON if pSrc
// ** is the left operand of a RIGHT join.
// **
// ** See https://sqlite.org/forum/forumpost/206d99a16dd9212f
// ** for an example of a WHERE clause constraints that may not be used on
// ** the right table of a RIGHT JOIN because the constraint implies a
// ** not-NULL condition on the left table of the RIGHT JOIN.
// */
fn constraintCompatibleWithOuterJoin(mut pTerm: *const WhereTerm, mut pSrc: *const SrcItem) -> i32 {
    // /* By caller */
    0 as i32;
    {}
    {}
    {}
    if !((unsafe { (*unsafe { (*pTerm).pExpr }).flags }) & (((1 as i32) | (2 as i32)) as u32)
        != ((0 as i32) as u32))
        || (unsafe { (*unsafe { (*pTerm).pExpr }).w.iJoin }) != unsafe { (*pSrc).iCursor }
    {
        return 0 as i32;
    }
    if (((unsafe { (*pSrc).fg.jointype }) as u32) as i32) & ((8 as i32) | (16 as i32)) != (0 as i32)
        && (unsafe { (*unsafe { (*pTerm).pExpr }).flags }) & ((2 as i32) as u32)
            != ((0 as i32) as u32)
    {
        return 0 as i32;
    }
    return 1 as i32;
}

// /* WHERE clause term to check */
// /* Table we are trying to access */
// /*
// ** Return true if column iCol of table pTab seem like it might be a
// ** good column to use as part of a query-time index.
// **
// ** Current algorithm (subject to improvement!):
// **
// **   1.   If iCol is already the left-most column of some other index,
// **        then return false.
// **
// **   2.   If iCol is part of an existing index that has an aiRowLogEst of
// **        more than 20, then return false.
// **
// **   3.   If no disqualifying conditions above are found, return true.
// **
// ** 2025-01-03: I experimented with a new rule that returns false if the
// ** the datatype of the column is "BOOLEAN". This did not improve
// ** performance on any queries at hand, but it did burn CPU cycles, so the
// ** idea was not committed.
// */
fn columnIsGoodIndexCandidate(mut pTab: *const Table, mut iCol: i32) -> i32 {
    let mut pIdx: *const Index = unsafe { std::mem::zeroed() };
    pIdx = (unsafe { (*pTab).pIndex }) as *const Index;
    '__slate_break_1732: while pIdx != std::ptr::null::<Index>() {
        let mut j: i32 = 0 as i32;
        j = 0 as i32;
        '__slate_break_1733: loop {
            if !(j < (((unsafe { (*pIdx).nKeyCol }) as u32) as i32)) {
                break;
            }
            if ((unsafe { *unsafe { unsafe { (*pIdx).aiColumn }.offset(j as isize) } }) as i32)
                == iCol
            {
                if j == (0 as i32) {
                    return 0 as i32;
                }
                if ((unsafe { (*pIdx).__slate_bits_0.__get_hasStat1() }) as i32) != (0 as i32)
                    && ((unsafe {
                        *unsafe { unsafe { (*pIdx).aiRowLogEst }.offset((j + (1 as i32)) as isize) }
                    }) as i32)
                        > (20 as i32)
                {
                    return 0 as i32;
                }
                break '__slate_break_1733;
            }
            let __v2003: i32 = j;
            let __v2004: i32 = __v2003 + (1 as i32);
            j = __v2004;
        }
        pIdx = (unsafe { (*pIdx).pNext }) as *const Index;
    }
    return 1 as i32;
}

// /* SQLITE_OMIT_AUTOMATIC_INDEX */
// /*
// ** Return TRUE if the WHERE clause term pTerm is of a form where it
// ** could be used with an index to access pSrc, assuming an appropriate
// ** index existed.
// */
fn termCanDriveIndex(
    mut pTerm: *const WhereTerm,
    mut pSrc: *const SrcItem,
    mut notReady: u64,
) -> i32 {
    let mut aff: i8 = 0 as i8;
    let mut leftCol: i32 = 0 as i32;
    if (unsafe { (*pTerm).leftCursor }) != unsafe { (*pSrc).iCursor } {
        return 0 as i32;
    }
    if (((unsafe { (*pTerm).eOperator }) as u32) as i32) & ((2 as i32) | (128 as i32)) == (0 as i32)
    {
        return 0 as i32;
    }
    0 as i32;
    let __v2005: bool;
    if (((unsafe { (*pSrc).fg.jointype }) as u32) as i32) & ((8 as i32) | (64 as i32) | (16 as i32))
        != (0 as i32)
    {
        __v2005 = !(constraintCompatibleWithOuterJoin(pTerm, pSrc) != (0 as i32));
    } else {
        __v2005 = false as bool;
    }
    if __v2005 {
        // /* See https://sqlite.org/forum/forumpost/51e6959f61 */
        return 0 as i32;
    }
    if (unsafe { (*pTerm).prereqRight }) & notReady != (((0 as i32) as i64) as u64) {
        return 0 as i32;
    }
    0 as i32;
    leftCol = unsafe { (*pTerm).u.x.leftColumn };
    if leftCol < (0 as i32) {
        return 0 as i32;
    }
    aff = unsafe {
        (*unsafe { unsafe { (*unsafe { (*pSrc).pSTab }).aCol }.offset(leftCol as isize) }).affinity
    };
    if !((unsafe { sqlite3IndexAffinityOk((unsafe { (*pTerm).pExpr }) as *const Expr, aff) })
        != (0 as i32))
    {
        return 0 as i32;
    }
    {}
    return columnIsGoodIndexCandidate((unsafe { (*pSrc).pSTab }) as *const Table, leftCol);
}

// /* WHERE clause term to check */
// /* Table we are trying to access */
// /* Tables in outer loops of the join */
// /*
// ** Generate code to construct the Index object for an automatic index
// ** and to set up the WhereLevel object pLevel so that the code generator
// ** makes use of the automatic index.
// */
fn constructAutomaticIndex(
    mut pParse: *mut Parse,
    mut pWC: *mut WhereClause,
    mut notReady: u64,
    mut pLevel: *mut WhereLevel,
) {
    let mut __slate_storage_2048: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2048: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_2048) as *mut u32;
    let mut __slate_storage_2047: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2047: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_2047) as *mut u32;
    let mut __slate_storage_2046: std::mem::MaybeUninit<*mut WhereLoop> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_2046: *mut *mut WhereLoop =
        std::ptr::addr_of_mut!(__slate_storage_2046) as *mut *mut WhereLoop;
    let mut __slate_storage_713: std::mem::MaybeUninit<*mut Subquery> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_713: *mut *mut Subquery =
        std::ptr::addr_of_mut!(__slate_storage_713) as *mut *mut Subquery;
    let mut __slate_storage_712: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_712: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_712) as *mut i32;
    let mut __slate_storage_2045: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2045: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_2045) as *mut i32;
    let mut __slate_storage_2044: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2044: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_2044) as *mut i32;
    let mut __slate_storage_2043: std::mem::MaybeUninit<*mut Parse> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_2043: *mut *mut Parse =
        std::ptr::addr_of_mut!(__slate_storage_2043) as *mut *mut Parse;
    let mut __slate_storage_2042: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2042: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_2042) as *mut i32;
    let mut __slate_storage_2041: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2041: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_2041) as *mut i32;
    let mut __slate_storage_2040: std::mem::MaybeUninit<*mut Parse> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_2040: *mut *mut Parse =
        std::ptr::addr_of_mut!(__slate_storage_2040) as *mut *mut Parse;
    let mut __slate_storage_2037: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2037: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_2037) as *mut i32;
    let mut __slate_storage_2036: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2036: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_2036) as *mut i32;
    let mut __slate_storage_2039: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2039: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_2039) as *mut i32;
    let mut __slate_storage_2038: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2038: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_2038) as *mut i32;
    let mut __slate_storage_2033: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2033: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_2033) as *mut i32;
    let mut __slate_storage_2032: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2032: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_2032) as *mut i32;
    let mut __slate_storage_2035: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2035: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_2035) as *mut i32;
    let mut __slate_storage_2034: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2034: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_2034) as *mut i32;
    let mut __slate_storage_2027: std::mem::MaybeUninit<*mut WhereTerm> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_2027: *mut *mut WhereTerm =
        std::ptr::addr_of_mut!(__slate_storage_2027) as *mut *mut WhereTerm;
    let mut __slate_storage_2026: std::mem::MaybeUninit<*mut WhereTerm> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_2026: *mut *mut WhereTerm =
        std::ptr::addr_of_mut!(__slate_storage_2026) as *mut *mut WhereTerm;
    let mut __slate_storage_2031: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2031: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_2031) as *mut i32;
    let mut __slate_storage_2030: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2030: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_2030) as *mut i32;
    let mut __slate_storage_2029: std::mem::MaybeUninit<u64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2029: *mut u64 = std::ptr::addr_of_mut!(__slate_storage_2029) as *mut u64;
    let mut __slate_storage_2028: std::mem::MaybeUninit<u64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2028: *mut u64 = std::ptr::addr_of_mut!(__slate_storage_2028) as *mut u64;
    let mut __slate_storage_711: std::mem::MaybeUninit<*mut Expr> = std::mem::MaybeUninit::uninit();
    let __slate_slot_711: *mut *mut Expr =
        std::ptr::addr_of_mut!(__slate_storage_711) as *mut *mut Expr;
    let mut __slate_storage_710: std::mem::MaybeUninit<u64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_710: *mut u64 = std::ptr::addr_of_mut!(__slate_storage_710) as *mut u64;
    let mut __slate_storage_709: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_709: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_709) as *mut i32;
    let mut __slate_storage_2025: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2025: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_2025) as *mut i32;
    let mut __slate_storage_2024: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2024: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_2024) as *mut i32;
    let mut __slate_storage_2021: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2021: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_2021) as *mut i32;
    let mut __slate_storage_2020: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2020: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_2020) as *mut i32;
    let mut __slate_storage_2023: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2023: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_2023) as *mut i32;
    let mut __slate_storage_2022: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2022: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_2022) as *mut i32;
    let mut __slate_storage_2015: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2015: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_2015) as *mut i32;
    let mut __slate_storage_2014: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2014: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_2014) as *mut i32;
    let mut __slate_storage_2019: std::mem::MaybeUninit<u64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2019: *mut u64 = std::ptr::addr_of_mut!(__slate_storage_2019) as *mut u64;
    let mut __slate_storage_2018: std::mem::MaybeUninit<u64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2018: *mut u64 = std::ptr::addr_of_mut!(__slate_storage_2018) as *mut u64;
    let mut __slate_storage_2017: std::mem::MaybeUninit<u64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2017: *mut u64 = std::ptr::addr_of_mut!(__slate_storage_2017) as *mut u64;
    let mut __slate_storage_2016: std::mem::MaybeUninit<u64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2016: *mut u64 = std::ptr::addr_of_mut!(__slate_storage_2016) as *mut u64;
    let mut __slate_storage_2013: std::mem::MaybeUninit<u16> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2013: *mut u16 = std::ptr::addr_of_mut!(__slate_storage_2013) as *mut u16;
    let mut __slate_storage_2007: std::mem::MaybeUninit<*mut WhereTerm> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_2007: *mut *mut WhereTerm =
        std::ptr::addr_of_mut!(__slate_storage_2007) as *mut *mut WhereTerm;
    let mut __slate_storage_2006: std::mem::MaybeUninit<*mut WhereTerm> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_2006: *mut *mut WhereTerm =
        std::ptr::addr_of_mut!(__slate_storage_2006) as *mut *mut WhereTerm;
    let mut __slate_storage_2012: std::mem::MaybeUninit<u64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2012: *mut u64 = std::ptr::addr_of_mut!(__slate_storage_2012) as *mut u64;
    let mut __slate_storage_2011: std::mem::MaybeUninit<u64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2011: *mut u64 = std::ptr::addr_of_mut!(__slate_storage_2011) as *mut u64;
    let mut __slate_storage_2010: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2010: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_2010) as *mut i32;
    let mut __slate_storage_2009: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2009: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_2009) as *mut i32;
    let mut __slate_storage_708: std::mem::MaybeUninit<u64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_708: *mut u64 = std::ptr::addr_of_mut!(__slate_storage_708) as *mut u64;
    let mut __slate_storage_707: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_707: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_707) as *mut i32;
    let mut __slate_storage_2008: std::mem::MaybeUninit<bool> = std::mem::MaybeUninit::uninit();
    let __slate_slot_2008: *mut bool = std::ptr::addr_of_mut!(__slate_storage_2008) as *mut bool;
    let mut __slate_storage_706: std::mem::MaybeUninit<*mut Expr> = std::mem::MaybeUninit::uninit();
    let __slate_slot_706: *mut *mut Expr =
        std::ptr::addr_of_mut!(__slate_storage_706) as *mut *mut Expr;
    let mut __slate_storage_705: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_705: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_705) as *mut i32;
    let mut __slate_storage_704: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_704: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_704) as *mut i32;
    let mut __slate_storage_703: std::mem::MaybeUninit<*mut SrcItem> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_703: *mut *mut SrcItem =
        std::ptr::addr_of_mut!(__slate_storage_703) as *mut *mut SrcItem;
    let mut __slate_storage_702: std::mem::MaybeUninit<*mut SrcList> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_702: *mut *mut SrcList =
        std::ptr::addr_of_mut!(__slate_storage_702) as *mut *mut SrcList;
    let mut __slate_storage_701: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_701: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_701) as *mut i32;
    let mut __slate_storage_700: std::mem::MaybeUninit<*mut Expr> = std::mem::MaybeUninit::uninit();
    let __slate_slot_700: *mut *mut Expr =
        std::ptr::addr_of_mut!(__slate_storage_700) as *mut *mut Expr;
    let mut __slate_storage_699: std::mem::MaybeUninit<u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_699: *mut u8 = std::ptr::addr_of_mut!(__slate_storage_699) as *mut u8;
    let mut __slate_storage_698: std::mem::MaybeUninit<u8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_698: *mut u8 = std::ptr::addr_of_mut!(__slate_storage_698) as *mut u8;
    let mut __slate_storage_697: std::mem::MaybeUninit<u64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_697: *mut u64 = std::ptr::addr_of_mut!(__slate_storage_697) as *mut u64;
    let mut __slate_storage_696: std::mem::MaybeUninit<u64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_696: *mut u64 = std::ptr::addr_of_mut!(__slate_storage_696) as *mut u64;
    let mut __slate_storage_695: std::mem::MaybeUninit<*mut i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_695: *mut *mut i8 =
        std::ptr::addr_of_mut!(__slate_storage_695) as *mut *mut i8;
    let mut __slate_storage_694: std::mem::MaybeUninit<*mut WhereLoop> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_694: *mut *mut WhereLoop =
        std::ptr::addr_of_mut!(__slate_storage_694) as *mut *mut WhereLoop;
    let mut __slate_storage_693: std::mem::MaybeUninit<*mut CollSeq> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_693: *mut *mut CollSeq =
        std::ptr::addr_of_mut!(__slate_storage_693) as *mut *mut CollSeq;
    let mut __slate_storage_692: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_692: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_692) as *mut i32;
    let mut __slate_storage_691: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_691: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_691) as *mut i32;
    let mut __slate_storage_690: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_690: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_690) as *mut i32;
    let mut __slate_storage_689: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_689: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_689) as *mut i32;
    let mut __slate_storage_688: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_688: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_688) as *mut i32;
    let mut __slate_storage_687: std::mem::MaybeUninit<*mut Table> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_687: *mut *mut Table =
        std::ptr::addr_of_mut!(__slate_storage_687) as *mut *mut Table;
    let mut __slate_storage_686: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_686: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_686) as *mut i32;
    let mut __slate_storage_685: std::mem::MaybeUninit<*mut Vdbe> = std::mem::MaybeUninit::uninit();
    let __slate_slot_685: *mut *mut Vdbe =
        std::ptr::addr_of_mut!(__slate_storage_685) as *mut *mut Vdbe;
    let mut __slate_storage_684: std::mem::MaybeUninit<*mut Index> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_684: *mut *mut Index =
        std::ptr::addr_of_mut!(__slate_storage_684) as *mut *mut Index;
    let mut __slate_storage_683: std::mem::MaybeUninit<*mut WhereTerm> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_683: *mut *mut WhereTerm =
        std::ptr::addr_of_mut!(__slate_storage_683) as *mut *mut WhereTerm;
    let mut __slate_storage_682: std::mem::MaybeUninit<*mut WhereTerm> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_682: *mut *mut WhereTerm =
        std::ptr::addr_of_mut!(__slate_storage_682) as *mut *mut WhereTerm;
    let mut __slate_storage_681: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_681: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_681) as *mut i32;
    unsafe {
        // /* Number of columns in the constructed index */
        // /* A single term of the WHERE clause */
        // /* End of pWC->a[] */
        // /* Object describing the transient index */
        // /* Prepared statement under construction */
        // /* Address of the initialization bypass jump */
        // /* The table being indexed */
        // /* Top of the index fill loop */
        // /* Register holding an index record */
        // /* Column counter */
        // /* Loop counter */
        // /* Maximum column in pSrc->colUsed */
        // /* Collating sequence to on a column */
        // /* The Loop object */
        // /* Extra space on the end of pIdx */
        // /* Bitmap of columns used for indexing */
        // /* Bitmap of additional columns */
        // /* True if a warning has been issued */
        std::ptr::write(__slate_slot_698, ((0 as i32) as i8) as u8);
        // /* True to also add a Bloom filter */
        std::ptr::write(__slate_slot_699, ((1 as i32) as i8) as u8);
        // /* Partial Index Expression */
        std::ptr::write(__slate_slot_700, std::ptr::null_mut::<Expr>());
        // /* Jump here to skip excluded rows */
        std::ptr::write(__slate_slot_701, 0 as i32);
        // /* The complete FROM clause */
        // /* The FROM clause term to get the next index */
        // /* Address where integer counter is initialized */
        std::ptr::write(__slate_slot_704, 0 as i32);
        // /* Array of registers where record is assembled */
        // /* Generate code to skip over the creation and initialization of the
        //   ** transient index on 2nd and subsequent iterations of the loop. */
        *__slate_slot_685 = unsafe { (*pParse).pVdbe };
        0 as i32;
        *__slate_slot_686 = unsafe { sqlite3VdbeAddOp0(*__slate_slot_685, 15 as i32) };
        {}
        // /* Count the number of columns that will be added to the index
        //   ** and used to match WHERE clause constraints */
        *__slate_slot_681 = 0 as i32;
        *__slate_slot_702 = unsafe { (*unsafe { (*pWC).pWInfo }).pTabList };
        *__slate_slot_703 = unsafe {
            unsafe { std::ptr::addr_of_mut!((*(*__slate_slot_702)).a) as *mut SrcItem }
                .offset((((unsafe { (*pLevel).iFrom }) as u32) as i32) as isize)
        };
        *__slate_slot_687 = unsafe { (*(*__slate_slot_703)).pSTab };
        *__slate_slot_683 =
            unsafe { unsafe { (*pWC).a }.offset((unsafe { (*pWC).nTerm }) as isize) };
        *__slate_slot_694 = unsafe { (*pLevel).pWLoop };
        *__slate_slot_696 = ((0 as i32) as i64) as u64;
        *__slate_slot_682 = unsafe { (*pWC).a };
        '__join_0: {
            loop {
                if *__slate_slot_682 < *__slate_slot_683 {
                    std::ptr::write(__slate_slot_706, unsafe { (*(*__slate_slot_682)).pExpr });
                    // /* Make the automatic index a partial index if there are terms in the
                    //     ** WHERE clause (or the ON clause of a LEFT join) that constrain which
                    //     ** rows of the target table (pSrc) that can be used. */
                    if (((unsafe { (*(*__slate_slot_682)).wtFlags }) as u32) as i32) & (2 as i32)
                        == (0 as i32)
                    {
                        *__slate_slot_2008 = (unsafe {
                            sqlite3ExprIsSingleTableConstraint(
                                *__slate_slot_706,
                                *__slate_slot_702 as *const SrcList,
                                ((unsafe { (*pLevel).iFrom }) as u32) as i32,
                                0 as i32,
                            )
                        }) != (0 as i32);
                    } else {
                        *__slate_slot_2008 = false as bool;
                    }
                    if *__slate_slot_2008 {
                        *__slate_slot_700 = unsafe {
                            sqlite3ExprAnd(pParse, *__slate_slot_700, unsafe {
                                sqlite3ExprDup(
                                    unsafe { (*pParse).db },
                                    *__slate_slot_706 as *const Expr,
                                    0 as i32,
                                )
                            })
                        };
                    }
                    if termCanDriveIndex(
                        *__slate_slot_682 as *const WhereTerm,
                        *__slate_slot_703 as *const SrcItem,
                        notReady,
                    ) != (0 as i32)
                    {
                        0 as i32;
                        *__slate_slot_707 = unsafe { (*(*__slate_slot_682)).u.x.leftColumn };
                        *__slate_slot_708 = if *__slate_slot_707
                            >= (((8 as u64).wrapping_mul(((8 as i32) as i64) as u64) as u32) as i32)
                        {
                            (((1 as i32) as i64) as u64)
                                << (((8 as u64).wrapping_mul(((8 as i32) as i64) as u64) as u32)
                                    as i32)
                                    - (1 as i32)
                        } else {
                            (((1 as i32) as i64) as u64) << *__slate_slot_707
                        };
                        {}
                        {}
                        if !(*__slate_slot_698 != (0 as u8)) {
                            unsafe {
                                sqlite3_log(
                                    (28 as i32) | (1 as i32) << (8 as i32),
                                    (b"automatic index on %s(%s)\0".as_ptr() as *mut i8)
                                        as *const i8,
                                    unsafe { (*(*__slate_slot_687)).zName },
                                    unsafe {
                                        (*unsafe {
                                            unsafe { (*(*__slate_slot_687)).aCol }
                                                .offset(*__slate_slot_707 as isize)
                                        })
                                        .zCnName
                                    },
                                )
                            };
                            *__slate_slot_698 = ((1 as i32) as i8) as u8;
                        }
                        if *__slate_slot_696 & *__slate_slot_708 == (((0 as i32) as i64) as u64) {
                            if whereLoopResize(
                                unsafe { (*pParse).db },
                                *__slate_slot_694,
                                *__slate_slot_681 + (1 as i32),
                            ) != (0 as i32)
                            {
                                break '__join_0;
                            } else {
                                std::ptr::write(__slate_slot_2009, *__slate_slot_681);
                                std::ptr::write(__slate_slot_2010, *__slate_slot_2009 + (1 as i32));
                                *__slate_slot_681 = *__slate_slot_2010;
                                unsafe {
                                    *unsafe {
                                        unsafe { (*(*__slate_slot_694)).aLTerm }
                                            .offset(*__slate_slot_2009 as isize)
                                    } = *__slate_slot_682;
                                }
                                std::ptr::write(__slate_slot_2011, *__slate_slot_696);
                                std::ptr::write(
                                    __slate_slot_2012,
                                    *__slate_slot_2011 | *__slate_slot_708,
                                );
                                *__slate_slot_696 = *__slate_slot_2012;
                            }
                        }
                    }
                    std::ptr::write(__slate_slot_2006, *__slate_slot_682);
                    std::ptr::write(__slate_slot_2007, unsafe {
                        (*__slate_slot_2006).offset((1 as i32) as isize)
                    });
                    *__slate_slot_682 = *__slate_slot_2007;
                } else {
                    break;
                }
            }
            0 as i32;
            std::ptr::write(__slate_slot_2013, (*__slate_slot_681 as i16) as u16);
            unsafe {
                (*(*__slate_slot_694)).nLTerm = *__slate_slot_2013;
            }
            unsafe {
                (*(*__slate_slot_694)).u.btree.nEq = *__slate_slot_2013;
            }
            unsafe {
                (*(*__slate_slot_694)).wsFlags =
                    ((1 as i32) | (64 as i32) | (512 as i32) | (16384 as i32)) as u32;
            }
            // /* Count the number of additional columns needed to create a
            //   ** covering index.  A "covering index" is an index that contains all
            //   ** columns that are needed by the query.  With a covering index, the
            //   ** original table never needs to be accessed.  Automatic indices must
            //   ** be a covering index because the index will not be updated if the
            //   ** original table changes and the index and table cannot both be used
            //   ** if they go out of sync.
            //   */
            if (((unsafe { (*(*__slate_slot_687)).eTabType }) as u32) as i32) == (2 as i32) {
                *__slate_slot_697 = ((-(1 as i32) as i64) as u64) & !(*__slate_slot_696);
            } else {
                *__slate_slot_697 = (unsafe { (*(*__slate_slot_703)).colUsed })
                    & (!(*__slate_slot_696)
                        | (((1 as i32) as i64) as u64)
                            << (((8 as u64).wrapping_mul(((8 as i32) as i64) as u64) as u32)
                                as i32)
                                - (1 as i32));
            }
            '__join_43: {
                if !((unsafe { (*(*__slate_slot_687)).tabFlags }) & ((128 as i32) as u32)
                    == ((0 as i32) as u32))
                {
                    // /* For WITHOUT ROWID tables, ensure that all PRIMARY KEY columns are
                    //     ** either in the idxCols mask or in the extraCols mask */
                    *__slate_slot_691 = 0 as i32;
                    '__loop_44: loop {
                        if *__slate_slot_691 < ((unsafe { (*(*__slate_slot_687)).nCol }) as i32) {
                            if (((unsafe {
                                (*unsafe {
                                    unsafe { (*(*__slate_slot_687)).aCol }
                                        .offset(*__slate_slot_691 as isize)
                                })
                                .colFlags
                            }) as u32) as i32)
                                & (1 as i32)
                                == (0 as i32)
                            {
                            } else {
                                if *__slate_slot_691
                                    >= (((8 as u64).wrapping_mul(((8 as i32) as i64) as u64) as u32)
                                        as i32)
                                        - (1 as i32)
                                {
                                    break '__loop_44;
                                } else {
                                    if *__slate_slot_696
                                        & (((1 as i32) as i64) as u64) << *__slate_slot_691
                                        != (0 as u64)
                                    {
                                    } else {
                                        std::ptr::write(__slate_slot_2018, *__slate_slot_697);
                                        std::ptr::write(
                                            __slate_slot_2019,
                                            *__slate_slot_2018
                                                | (((1 as i32) as i64) as u64) << *__slate_slot_691,
                                        );
                                        *__slate_slot_697 = *__slate_slot_2019;
                                    }
                                }
                            }
                            std::ptr::write(__slate_slot_2014, *__slate_slot_691);
                            std::ptr::write(__slate_slot_2015, *__slate_slot_2014 + (1 as i32));
                            *__slate_slot_691 = *__slate_slot_2015;
                        } else {
                            break '__join_43;
                        }
                    }
                    std::ptr::write(__slate_slot_2016, *__slate_slot_697);
                    std::ptr::write(
                        __slate_slot_2017,
                        *__slate_slot_2016
                            | (((1 as i32) as i64) as u64)
                                << (((8 as u64).wrapping_mul(((8 as i32) as i64) as u64) as u32)
                                    as i32)
                                    - (1 as i32),
                    );
                    *__slate_slot_697 = *__slate_slot_2017;
                }
            }
            *__slate_slot_692 = if (((8 as u64).wrapping_mul(((8 as i32) as i64) as u64) as u32)
                as i32)
                - (1 as i32)
                < ((unsafe { (*(*__slate_slot_687)).nCol }) as i32)
            {
                (((8 as u64).wrapping_mul(((8 as i32) as i64) as u64) as u32) as i32) - (1 as i32)
            } else {
                (unsafe { (*(*__slate_slot_687)).nCol }) as i32
            };
            {}
            {}
            *__slate_slot_691 = 0 as i32;
            loop {
                if *__slate_slot_691 < *__slate_slot_692 {
                    if *__slate_slot_697 & (((1 as i32) as i64) as u64) << *__slate_slot_691
                        != (0 as u64)
                    {
                        std::ptr::write(__slate_slot_2022, *__slate_slot_681);
                        std::ptr::write(__slate_slot_2023, *__slate_slot_2022 + (1 as i32));
                        *__slate_slot_681 = *__slate_slot_2023;
                    }
                    std::ptr::write(__slate_slot_2020, *__slate_slot_691);
                    std::ptr::write(__slate_slot_2021, *__slate_slot_2020 + (1 as i32));
                    *__slate_slot_691 = *__slate_slot_2021;
                } else {
                    break;
                }
            }
            if (unsafe { (*(*__slate_slot_703)).colUsed })
                & (((1 as i32) as i64) as u64)
                    << (((8 as u64).wrapping_mul(((8 as i32) as i64) as u64) as u32) as i32)
                        - (1 as i32)
                != (0 as u64)
            {
                std::ptr::write(__slate_slot_2024, *__slate_slot_681);
                std::ptr::write(
                    __slate_slot_2025,
                    *__slate_slot_2024
                        + (((unsafe { (*(*__slate_slot_687)).nCol }) as i32)
                            - (((8 as u64).wrapping_mul(((8 as i32) as i64) as u64) as u32)
                                as i32)
                            + (1 as i32)),
                );
                *__slate_slot_681 = *__slate_slot_2025;
            }
            // /* Construct the Index object to describe this index */
            0 as i32;
            // /* ^-- This guarantees that the number of index columns will fit in the u16 */
            *__slate_slot_684 = unsafe {
                sqlite3AllocateIndexObject(
                    unsafe { (*pParse).db },
                    *__slate_slot_681
                        + (((unsafe { (*(*__slate_slot_687)).tabFlags }) & ((128 as i32) as u32)
                            == ((0 as i32) as u32)) as i32),
                    0 as i32,
                    std::ptr::addr_of_mut!(*__slate_slot_695),
                )
            };
            if *__slate_slot_684 == std::ptr::null_mut::<Index>() {
            } else {
                unsafe {
                    (*(*__slate_slot_694)).u.btree.pIndex = *__slate_slot_684;
                }
                unsafe {
                    (*(*__slate_slot_684)).zName = b"auto-index\0".as_ptr() as *mut i8;
                }
                unsafe {
                    (*(*__slate_slot_684)).pTable = *__slate_slot_687;
                }
                *__slate_slot_690 = 0 as i32;
                *__slate_slot_696 = ((0 as i32) as i64) as u64;
                *__slate_slot_682 = unsafe { (*pWC).a };
                loop {
                    if *__slate_slot_682 < *__slate_slot_683 {
                        if termCanDriveIndex(
                            *__slate_slot_682 as *const WhereTerm,
                            *__slate_slot_703 as *const SrcItem,
                            notReady,
                        ) != (0 as i32)
                        {
                            0 as i32;
                            *__slate_slot_709 = unsafe { (*(*__slate_slot_682)).u.x.leftColumn };
                            *__slate_slot_710 = if *__slate_slot_709
                                >= (((8 as u64).wrapping_mul(((8 as i32) as i64) as u64) as u32)
                                    as i32)
                            {
                                (((1 as i32) as i64) as u64)
                                    << (((8 as u64).wrapping_mul(((8 as i32) as i64) as u64) as u32)
                                        as i32)
                                        - (1 as i32)
                            } else {
                                (((1 as i32) as i64) as u64) << *__slate_slot_709
                            };
                            {}
                            {}
                            if *__slate_slot_696 & *__slate_slot_710 == (((0 as i32) as i64) as u64)
                            {
                                std::ptr::write(__slate_slot_711, unsafe {
                                    (*(*__slate_slot_682)).pExpr
                                });
                                std::ptr::write(__slate_slot_2028, *__slate_slot_696);
                                std::ptr::write(
                                    __slate_slot_2029,
                                    *__slate_slot_2028 | *__slate_slot_710,
                                );
                                *__slate_slot_696 = *__slate_slot_2029;
                                unsafe {
                                    *unsafe {
                                        unsafe { (*(*__slate_slot_684)).aiColumn }
                                            .offset(*__slate_slot_690 as isize)
                                    } = (unsafe { (*(*__slate_slot_682)).u.x.leftColumn }) as i16;
                                }
                                *__slate_slot_693 = unsafe {
                                    sqlite3ExprCompareCollSeq(
                                        pParse,
                                        *__slate_slot_711 as *const Expr,
                                    )
                                };
                                // /* TH3 collate01.800 */
                                0 as i32;
                                if !((unsafe {
                                    sqlite3IsBinary(*__slate_slot_693 as *const CollSeq)
                                }) != (0 as i32))
                                {
                                    // /* Disallow the use of a Bloom filter if any non-BINARY collating
                                    //           ** sequence is involved.  tag-202607231411 */
                                    *__slate_slot_699 = ((0 as i32) as i8) as u8;
                                }
                                unsafe {
                                    *unsafe {
                                        unsafe { (*(*__slate_slot_684)).azColl }
                                            .offset(*__slate_slot_690 as isize)
                                    } = if *__slate_slot_693 != std::ptr::null_mut::<CollSeq>() {
                                        (unsafe { (*(*__slate_slot_693)).zName }) as *const i8
                                    } else {
                                        unsafe { std::ptr::addr_of!(sqlite3StrBINARY) as *const i8 }
                                    };
                                }
                                std::ptr::write(__slate_slot_2030, *__slate_slot_690);
                                std::ptr::write(__slate_slot_2031, *__slate_slot_2030 + (1 as i32));
                                *__slate_slot_690 = *__slate_slot_2031;
                            }
                        }
                        std::ptr::write(__slate_slot_2026, *__slate_slot_682);
                        std::ptr::write(__slate_slot_2027, unsafe {
                            (*__slate_slot_2026).offset((1 as i32) as isize)
                        });
                        *__slate_slot_682 = *__slate_slot_2027;
                    } else {
                        break;
                    }
                }
                0 as i32;
                // /* Add additional columns needed to make the automatic index into
                //   ** a covering index */
                *__slate_slot_691 = 0 as i32;
                loop {
                    if *__slate_slot_691 < *__slate_slot_692 {
                        if *__slate_slot_697 & (((1 as i32) as i64) as u64) << *__slate_slot_691
                            != (0 as u64)
                        {
                            unsafe {
                                *unsafe {
                                    unsafe { (*(*__slate_slot_684)).aiColumn }
                                        .offset(*__slate_slot_690 as isize)
                                } = *__slate_slot_691 as i16;
                            }
                            unsafe {
                                *unsafe {
                                    unsafe { (*(*__slate_slot_684)).azColl }
                                        .offset(*__slate_slot_690 as isize)
                                } = unsafe { std::ptr::addr_of!(sqlite3StrBINARY) as *const i8 };
                            }
                            std::ptr::write(__slate_slot_2034, *__slate_slot_690);
                            std::ptr::write(__slate_slot_2035, *__slate_slot_2034 + (1 as i32));
                            *__slate_slot_690 = *__slate_slot_2035;
                        }
                        std::ptr::write(__slate_slot_2032, *__slate_slot_691);
                        std::ptr::write(__slate_slot_2033, *__slate_slot_2032 + (1 as i32));
                        *__slate_slot_691 = *__slate_slot_2033;
                    } else {
                        break;
                    }
                }
                '__join_18: {
                    if (unsafe { (*(*__slate_slot_703)).colUsed })
                        & (((1 as i32) as i64) as u64)
                            << (((8 as u64).wrapping_mul(((8 as i32) as i64) as u64) as u32) as i32)
                                - (1 as i32)
                        != (0 as u64)
                    {
                        *__slate_slot_691 = (((8 as u64).wrapping_mul(((8 as i32) as i64) as u64)
                            as u32) as i32)
                            - (1 as i32);
                        loop {
                            if *__slate_slot_691 < ((unsafe { (*(*__slate_slot_687)).nCol }) as i32)
                            {
                                unsafe {
                                    *unsafe {
                                        unsafe { (*(*__slate_slot_684)).aiColumn }
                                            .offset(*__slate_slot_690 as isize)
                                    } = *__slate_slot_691 as i16;
                                }
                                unsafe {
                                    *unsafe {
                                        unsafe { (*(*__slate_slot_684)).azColl }
                                            .offset(*__slate_slot_690 as isize)
                                    } = unsafe {
                                        std::ptr::addr_of!(sqlite3StrBINARY) as *const i8
                                    };
                                }
                                std::ptr::write(__slate_slot_2038, *__slate_slot_690);
                                std::ptr::write(__slate_slot_2039, *__slate_slot_2038 + (1 as i32));
                                *__slate_slot_690 = *__slate_slot_2039;
                                std::ptr::write(__slate_slot_2036, *__slate_slot_691);
                                std::ptr::write(__slate_slot_2037, *__slate_slot_2036 + (1 as i32));
                                *__slate_slot_691 = *__slate_slot_2037;
                            } else {
                                break '__join_18;
                            }
                        }
                    }
                }
                0 as i32;
                if (unsafe { (*(*__slate_slot_687)).tabFlags }) & ((128 as i32) as u32)
                    == ((0 as i32) as u32)
                {
                    unsafe {
                        *unsafe {
                            unsafe { (*(*__slate_slot_684)).aiColumn }
                                .offset(*__slate_slot_690 as isize)
                        } = -(1 as i32) as i16;
                    }
                    unsafe {
                        *unsafe {
                            unsafe { (*(*__slate_slot_684)).azColl }
                                .offset(*__slate_slot_690 as isize)
                        } = unsafe { std::ptr::addr_of!(sqlite3StrBINARY) as *const i8 };
                    }
                }
                // /* Create the automatic index */
                {}
                0 as i32;
                std::ptr::write(__slate_slot_2040, pParse);
                std::ptr::write(__slate_slot_2041, unsafe { (*(*__slate_slot_2040)).nTab });
                std::ptr::write(__slate_slot_2042, *__slate_slot_2041 + (1 as i32));
                unsafe {
                    (*(*__slate_slot_2040)).nTab = *__slate_slot_2042;
                }
                unsafe {
                    (*pLevel).iIdxCur = *__slate_slot_2041;
                }
                unsafe {
                    sqlite3VdbeAddOp2(
                        *__slate_slot_685,
                        119 as i32,
                        unsafe { (*pLevel).iIdxCur },
                        *__slate_slot_681 + (1 as i32),
                    )
                };
                unsafe { sqlite3VdbeSetP4KeyInfo(pParse, *__slate_slot_684) };
                {}
                if (unsafe { (*unsafe { (*pParse).db }).dbOptFlags }) & ((524288 as i32) as u32)
                    == ((0 as i32) as u32)
                    && *__slate_slot_699 != (0 as u8)
                {
                    unsafe {
                        sqlite3WhereExplainBloomFilter(
                            pParse as *const Parse,
                            (unsafe { (*pWC).pWInfo }) as *const WhereInfo,
                            pLevel as *const WhereLevel,
                        )
                    };
                    std::ptr::write(__slate_slot_2043, pParse);
                    std::ptr::write(__slate_slot_2044, unsafe { (*(*__slate_slot_2043)).nMem });
                    std::ptr::write(__slate_slot_2045, *__slate_slot_2044 + (1 as i32));
                    unsafe {
                        (*(*__slate_slot_2043)).nMem = *__slate_slot_2045;
                    }
                    unsafe {
                        (*pLevel).regFilter = *__slate_slot_2045;
                    }
                    unsafe {
                        sqlite3VdbeAddOp2(*__slate_slot_685, 79 as i32, 10000 as i32, unsafe {
                            (*pLevel).regFilter
                        })
                    };
                }
                // /* Fill the automatic index with content */
                0 as i32;
                if ((unsafe {
                    (*(*__slate_slot_703))
                        .fg
                        .__slate_bits_0
                        .__get_viaCoroutine()
                }) as i32)
                    != (0 as i32)
                {
                    0 as i32;
                    *__slate_slot_713 = unsafe { (*(*__slate_slot_703)).u4.pSubq };
                    0 as i32;
                    *__slate_slot_712 = unsafe { (*(*__slate_slot_713)).regReturn };
                    *__slate_slot_704 = unsafe {
                        sqlite3VdbeAddOp2(*__slate_slot_685, 73 as i32, 0 as i32, 0 as i32)
                    };
                    unsafe {
                        sqlite3VdbeAddOp3(
                            *__slate_slot_685,
                            11 as i32,
                            *__slate_slot_712,
                            0 as i32,
                            unsafe { (*(*__slate_slot_713)).addrFillSub },
                        )
                    };
                    *__slate_slot_688 = unsafe {
                        sqlite3VdbeAddOp1(*__slate_slot_685, 12 as i32, *__slate_slot_712)
                    };
                    {}
                    {}
                } else {
                    0 as i32;
                    *__slate_slot_688 = unsafe {
                        sqlite3VdbeAddOp2(
                            *__slate_slot_685,
                            36 as i32,
                            unsafe { (*pLevel).iTabCur },
                            unsafe { (*pLevel).addrHalt },
                        )
                    };
                    {}
                }
                if *__slate_slot_700 != std::ptr::null_mut::<Expr>() {
                    *__slate_slot_701 = unsafe { sqlite3VdbeMakeLabel(pParse) };
                    unsafe {
                        sqlite3ExprIfFalse(pParse, *__slate_slot_700, *__slate_slot_701, 16 as i32)
                    };
                    std::ptr::write(__slate_slot_2046, *__slate_slot_694);
                    std::ptr::write(__slate_slot_2047, unsafe {
                        (*(*__slate_slot_2046)).wsFlags
                    });
                    std::ptr::write(
                        __slate_slot_2048,
                        *__slate_slot_2047 | ((131072 as i32) as u32),
                    );
                    unsafe {
                        (*(*__slate_slot_2046)).wsFlags = *__slate_slot_2048;
                    }
                }
                *__slate_slot_689 = unsafe { sqlite3GetTempReg(pParse) };
                *__slate_slot_705 = unsafe {
                    sqlite3GenerateIndexKey(
                        pParse,
                        *__slate_slot_684,
                        unsafe { (*pLevel).iTabCur },
                        *__slate_slot_689,
                        0 as i32,
                        std::ptr::null_mut::<i32>(),
                        std::ptr::null_mut::<Index>(),
                        0 as i32,
                    )
                };
                if (unsafe { (*pLevel).regFilter }) != (0 as i32) {
                    0 as i32;
                    unsafe {
                        sqlite3VdbeAddOp4Int(
                            *__slate_slot_685,
                            185 as i32,
                            unsafe { (*pLevel).regFilter },
                            0 as i32,
                            *__slate_slot_705,
                            ((unsafe { (*(*__slate_slot_694)).u.btree.nEq }) as u32) as i32,
                        )
                    };
                }
                {}
                unsafe {
                    sqlite3VdbeAddOp2(
                        *__slate_slot_685,
                        140 as i32,
                        unsafe { (*pLevel).iIdxCur },
                        *__slate_slot_689,
                    )
                };
                unsafe { sqlite3VdbeChangeP5(*__slate_slot_685, ((16 as i32) as i16) as u16) };
                if *__slate_slot_700 != std::ptr::null_mut::<Expr>() {
                    unsafe { sqlite3VdbeResolveLabel(*__slate_slot_685, *__slate_slot_701) };
                }
                if ((unsafe {
                    (*(*__slate_slot_703))
                        .fg
                        .__slate_bits_0
                        .__get_viaCoroutine()
                }) as i32)
                    != (0 as i32)
                {
                    0 as i32;
                    unsafe {
                        sqlite3VdbeChangeP2(
                            *__slate_slot_685,
                            *__slate_slot_704,
                            *__slate_slot_705 + *__slate_slot_690,
                        )
                    };
                    {}
                    0 as i32;
                    translateColumnToCopy(
                        pParse,
                        *__slate_slot_688,
                        unsafe { (*pLevel).iTabCur },
                        unsafe { (*unsafe { (*(*__slate_slot_703)).u4.pSubq }).regResult },
                        unsafe { (*pLevel).iIdxCur },
                    );
                    unsafe { sqlite3VdbeGoto(*__slate_slot_685, *__slate_slot_688) };
                    unsafe {
                        (*(*__slate_slot_703))
                            .fg
                            .__slate_bits_0
                            .__set_viaCoroutine((0 as i32) as u32);
                    }
                    unsafe { sqlite3VdbeJumpHere(*__slate_slot_685, *__slate_slot_688) };
                } else {
                    unsafe {
                        sqlite3VdbeAddOp2(
                            *__slate_slot_685,
                            40 as i32,
                            unsafe { (*pLevel).iTabCur },
                            *__slate_slot_688 + (1 as i32),
                        )
                    };
                    {}
                    unsafe { sqlite3VdbeChangeP5(*__slate_slot_685, ((3 as i32) as i16) as u16) };
                    if (((unsafe { (*(*__slate_slot_703)).fg.jointype }) as u32) as i32)
                        & (8 as i32)
                        != (0 as i32)
                    {
                        unsafe { sqlite3VdbeJumpHere(*__slate_slot_685, *__slate_slot_688) };
                    }
                }
                unsafe { sqlite3ReleaseTempReg(pParse, *__slate_slot_689) };
                // /* Jump here when skipping the initialization */
                unsafe { sqlite3VdbeJumpHere(*__slate_slot_685, *__slate_slot_686) };
                {}
            }
        }
        unsafe { sqlite3ExprDelete(unsafe { (*pParse).db }, *__slate_slot_700) };
    }
}

// /* The parsing context */
// /* The WHERE clause */
// /* Mask of cursors that are not available */
// /* Write new index here */
// /* SQLITE_OMIT_AUTOMATIC_INDEX */
// /*
// ** Generate bytecode that will initialize a Bloom filter that is appropriate
// ** for pLevel.
// **
// ** If there are inner loops within pLevel that have the WHERE_BLOOMFILTER
// ** flag set, initialize a Bloomfilter for them as well.
// **
// ** When the Bloom filter is initialized, the WHERE_BLOOMFILTER flag is cleared
// ** from the loop, but the regFilter value is set to a register that implements
// ** the Bloom filter.  When regFilter is positive, the
// ** sqlite3WhereCodeOneLoopStart() will generate code to test the Bloom filter
// ** and skip the subsequence B-Tree seek if the Bloom filter indicates that
// ** no matching rows exist.
// **
// ** This routine may only be called if it has previously been determined that
// ** the loop would benefit from a Bloom filter, and the WHERE_BLOOMFILTER bit
// ** is set.
// */
fn sqlite3ConstructBloomFilter(
    mut pWInfo: *mut WhereInfo,
    mut iLevel: i32,
    mut pLevel: *mut WhereLevel,
    mut notReady: u64,
) {
    // /* Address of opening OP_Once */
    let mut addrOnce: i32 = 0 as i32;
    // /* Address of OP_Rewind */
    let mut addrTop: i32 = 0 as i32;
    // /* Jump here to skip a row */
    let mut addrCont: i32 = 0 as i32;
    // /* For looping over WHERE clause terms */
    let mut pTerm: *const WhereTerm = unsafe { std::mem::zeroed() };
    // /* Last WHERE clause term */
    let mut pWCEnd: *const WhereTerm = unsafe { std::mem::zeroed() };
    // /* Parsing context */
    let mut pParse: *mut Parse = unsafe { (*pWInfo).pParse };
    // /* VDBE under construction */
    let mut v: *mut Vdbe = unsafe { (*pParse).pVdbe };
    // /* The loop being coded */
    let mut pLoop: *mut WhereLoop = unsafe { (*pLevel).pWLoop };
    // /* Cursor for table getting the filter */
    let mut iCur: i32 = 0 as i32;
    // /* saved copy of Parse.pIdxEpr */
    let mut saved_pIdxEpr: *mut IndexedExpr = unsafe { std::mem::zeroed() };
    // /* saved copy of Parse.pIdxPartExpr */
    let mut saved_pIdxPartExpr: *mut IndexedExpr = unsafe { std::mem::zeroed() };
    saved_pIdxEpr = unsafe { (*pParse).pIdxEpr };
    saved_pIdxPartExpr = unsafe { (*pParse).pIdxPartExpr };
    unsafe {
        (*pParse).pIdxEpr = std::ptr::null_mut::<IndexedExpr>();
    }
    unsafe {
        (*pParse).pIdxPartExpr = std::ptr::null_mut::<IndexedExpr>();
    }
    0 as i32;
    0 as i32;
    0 as i32;
    0 as i32;
    addrOnce = unsafe { sqlite3VdbeAddOp0(v, 15 as i32) };
    {}
    '__slate_break_1742: loop {
        let mut pTabList: *const SrcList = unsafe { std::mem::zeroed() };
        let mut pItem: *const SrcItem = unsafe { std::mem::zeroed() };
        let mut pTab: *const Table = unsafe { std::mem::zeroed() };
        let mut sz: u64 = 0 as u64;
        let mut iSrc: i32 = 0 as i32;
        unsafe {
            sqlite3WhereExplainBloomFilter(
                pParse as *const Parse,
                pWInfo as *const WhereInfo,
                pLevel as *const WhereLevel,
            )
        };
        addrCont = unsafe { sqlite3VdbeMakeLabel(pParse) };
        iCur = unsafe { (*pLevel).iTabCur };
        let __v2049: *mut Parse = pParse;
        let __v2050: i32 = unsafe { (*__v2049).nMem };
        let __v2051: i32 = __v2050 + (1 as i32);
        unsafe {
            (*__v2049).nMem = __v2051;
        }
        unsafe {
            (*pLevel).regFilter = __v2051;
        }
        // /* The Bloom filter is a Blob held in a register.  Initialize it
        //     ** to zero-filled blob of at least 80K bits, but maybe more if the
        //     ** estimated size of the table is larger.  We could actually
        //     ** measure the size of the table at run-time using OP_Count with
        //     ** P3==1 and use that value to initialize the blob.  But that makes
        //     ** testing complicated.  By basing the blob size on the value in the
        //     ** sqlite_stat1 table, testing is much easier.
        //     */
        pTabList = (unsafe { (*pWInfo).pTabList }) as *const SrcList;
        iSrc = ((unsafe { (*pLevel).iFrom }) as u32) as i32;
        pItem = unsafe {
            unsafe { std::ptr::addr_of!((*pTabList).a) as *const SrcItem }.offset(iSrc as isize)
        };
        0 as i32;
        pTab = (unsafe { (*pItem).pSTab }) as *const Table;
        0 as i32;
        sz = unsafe { sqlite3LogEstToInt(unsafe { (*pTab).nRowLogEst }) };
        if sz < (((10000 as i32) as i64) as u64) {
            sz = ((10000 as i32) as i64) as u64;
        } else {
            if sz > (((10000000 as i32) as i64) as u64) {
                sz = ((10000000 as i32) as i64) as u64;
            }
        }
        0 as i32;
        unsafe {
            sqlite3VdbeAddOp2(v, 79 as i32, (sz as u32) as i32, unsafe {
                (*pLevel).regFilter
            })
        };
        addrTop = unsafe { sqlite3VdbeAddOp1(v, 36 as i32, iCur) };
        {}
        pWCEnd = (unsafe {
            unsafe { (*pWInfo).sWC.a }.offset((unsafe { (*pWInfo).sWC.nTerm }) as isize)
        }) as *const WhereTerm;
        pTerm = (unsafe { (*pWInfo).sWC.a }) as *const WhereTerm;
        '__slate_break_1743: while pTerm < pWCEnd {
            let mut pExpr: *mut Expr = unsafe { (*pTerm).pExpr };
            let __v2054: bool;
            if (((unsafe { (*pTerm).wtFlags }) as u32) as i32) & (2 as i32) == (0 as i32) {
                __v2054 = (unsafe {
                    sqlite3ExprIsSingleTableConstraint(pExpr, pTabList, iSrc, 0 as i32)
                }) != (0 as i32);
            } else {
                __v2054 = false as bool;
            }
            if __v2054 {
                unsafe {
                    sqlite3ExprIfFalse(pParse, unsafe { (*pTerm).pExpr }, addrCont, 16 as i32)
                };
            }
            let __v2052: *const WhereTerm = pTerm;
            let __v2053: *const WhereTerm = unsafe { __v2052.offset((1 as i32) as isize) };
            pTerm = __v2053;
        }
        if (unsafe { (*pLoop).wsFlags }) & ((256 as i32) as u32) != (0 as u32) {
            let mut r1: i32 = unsafe { sqlite3GetTempReg(pParse) };
            unsafe { sqlite3VdbeAddOp2(v, 137 as i32, iCur, r1) };
            unsafe {
                sqlite3VdbeAddOp4Int(
                    v,
                    185 as i32,
                    unsafe { (*pLevel).regFilter },
                    0 as i32,
                    r1,
                    1 as i32,
                )
            };
            unsafe { sqlite3ReleaseTempReg(pParse, r1) };
        } else {
            let mut pIdx: *mut Index = unsafe { (*pLoop).u.btree.pIndex };
            let mut n: i32 = ((unsafe { (*pLoop).u.btree.nEq }) as u32) as i32;
            let mut r1: i32 = unsafe { sqlite3GetTempRange(pParse, n) };
            let mut jj: i32 = 0 as i32;
            0 as i32;
            jj = 0 as i32;
            '__slate_break_1744: loop {
                if !(jj < n) {
                    break;
                }
                0 as i32;
                unsafe { sqlite3ExprCodeLoadIndexColumn(pParse, pIdx, iCur, jj, r1 + jj) };
                let __v2055: i32 = jj;
                let __v2056: i32 = __v2055 + (1 as i32);
                jj = __v2056;
            }
            unsafe {
                sqlite3VdbeAddOp4Int(
                    v,
                    185 as i32,
                    unsafe { (*pLevel).regFilter },
                    0 as i32,
                    r1,
                    n,
                )
            };
            unsafe { sqlite3ReleaseTempRange(pParse, r1, n) };
        }
        unsafe { sqlite3VdbeResolveLabel(v, addrCont) };
        unsafe {
            sqlite3VdbeAddOp2(
                v,
                40 as i32,
                unsafe { (*pLevel).iTabCur },
                addrTop + (1 as i32),
            )
        };
        {}
        unsafe { sqlite3VdbeJumpHere(v, addrTop) };
        let __v2057: *mut WhereLoop = pLoop;
        let __v2058: u32 = unsafe { (*__v2057).wsFlags };
        let __v2059: u32 = __v2058 & (!(4194304 as i32) as u32);
        unsafe {
            (*__v2057).wsFlags = __v2059;
        }
        '__slate_break_1745: loop {
            let __v2060: i32 = iLevel;
            let __v2061: i32 = __v2060 + (1 as i32);
            iLevel = __v2061;
            if !(__v2061 < (((unsafe { (*pWInfo).nLevel }) as u32) as i32)) {
                break;
            }
            let mut pTabItem: *const SrcItem = unsafe { std::mem::zeroed() };
            pLevel = unsafe {
                unsafe { std::ptr::addr_of_mut!((*pWInfo).a) as *mut WhereLevel }
                    .offset(iLevel as isize)
            };
            pTabItem = (unsafe {
                unsafe {
                    std::ptr::addr_of_mut!((*unsafe { (*pWInfo).pTabList }).a) as *mut SrcItem
                }
                .offset((((unsafe { (*pLevel).iFrom }) as u32) as i32) as isize)
            }) as *const SrcItem;
            if (((unsafe { (*pTabItem).fg.jointype }) as u32) as i32) & ((8 as i32) | (64 as i32))
                != (0 as i32)
            {
            } else {
                pLoop = unsafe { (*pLevel).pWLoop };
                if pLoop == std::ptr::null_mut::<WhereLoop>() {
                } else {
                    if (unsafe { (*pLoop).prereq }) & notReady != (0 as u64) {
                    } else {
                        if (unsafe { (*pLoop).wsFlags }) & (((4194304 as i32) | (4 as i32)) as u32)
                            == ((4194304 as i32) as u32)
                        {
                            // /* This is a candidate for bloom-filter pull-down (early evaluation).
                            //         ** The test that WHERE_COLUMN_IN is omitted is important, as we are
                            //         ** not able to do early evaluation of bloom filters that make use of
                            //         ** the IN operator */
                            break '__slate_break_1745;
                        }
                    }
                }
            }
        }
        if !(iLevel < (((unsafe { (*pWInfo).nLevel }) as u32) as i32)) {
            break;
        }
    }
    unsafe { sqlite3VdbeJumpHere(v, addrOnce) };
    unsafe {
        (*pParse).pIdxEpr = saved_pIdxEpr;
    }
    unsafe {
        (*pParse).pIdxPartExpr = saved_pIdxPartExpr;
    }
}

// /* The WHERE clause */
// /* Index in pWInfo->a[] that is pLevel */
// /* Make a Bloom filter for this FROM term */
// /* Loops that are not ready */
// /*
// ** Return term iTerm of the WhereClause passed as the first argument. Terms
// ** are numbered from 0 upwards, starting with the terms in pWC->a[], then
// ** those in pWC->pOuter->a[] (if any), and so on.
// */
fn termFromWhereClause(mut pWC: *mut WhereClause, mut iTerm: i32) -> *mut WhereTerm {
    let mut p: *mut WhereClause = unsafe { std::mem::zeroed() };
    p = pWC;
    '__slate_break_1746: while p != std::ptr::null_mut::<WhereClause>() {
        if iTerm < unsafe { (*p).nTerm } {
            return unsafe { unsafe { (*p).a }.offset(iTerm as isize) };
        }
        let __v2062: i32 = iTerm;
        let __v2063: i32 = __v2062 - unsafe { (*p).nTerm };
        iTerm = __v2063;
        p = unsafe { (*p).pOuter };
    }
    return std::ptr::null_mut::<WhereTerm>();
}

// /*
// ** Allocate and populate an sqlite3_index_info structure. It is the
// ** responsibility of the caller to eventually release the structure
// ** by passing the pointer returned by this function to freeIndexInfo().
// */
fn allocateIndexInfo(
    mut pWInfo: *mut WhereInfo,
    mut pWC: *mut WhereClause,
    mut mUnusable: u64,
    mut pSrc: *mut SrcItem,
    mut pmNoOmit: *mut u16,
) -> *mut sqlite3_index_info {
    let mut i: i32 = 0 as i32;
    let mut j: i32 = 0 as i32;
    let mut nTerm: i32 = 0 as i32;
    let mut pParse: *mut Parse = unsafe { (*pWInfo).pParse };
    let mut pIdxCons: *mut sqlite3_index_constraint = unsafe { std::mem::zeroed() };
    let mut pIdxOrderBy: *mut sqlite3_index_orderby = unsafe { std::mem::zeroed() };
    let mut pUsage: *mut sqlite3_index_constraint_usage = unsafe { std::mem::zeroed() };
    let mut pHidden: *mut HiddenIndexInfo = unsafe { std::mem::zeroed() };
    let mut pTerm: *mut WhereTerm = unsafe { std::mem::zeroed() };
    let mut nOrderBy: i32 = 0 as i32;
    let mut pIdxInfo: *mut sqlite3_index_info = unsafe { std::mem::zeroed() };
    let mut mNoOmit: u16 = ((0 as i32) as i16) as u16;
    let mut pTab: *const Table = unsafe { std::mem::zeroed() };
    let mut eDistinct: i32 = 0 as i32;
    let mut pOrderBy: *mut ExprList = unsafe { (*pWInfo).pOrderBy };
    let mut p: *mut WhereClause = unsafe { std::mem::zeroed() };
    0 as i32;
    pTab = (unsafe { (*pSrc).pSTab }) as *const Table;
    0 as i32;
    0 as i32;
    // /* Find all WHERE clause constraints referring to this virtual table.
    //   ** Mark each term with the TERM_OK flag.  Set nTerm to the number of
    //   ** terms found.
    //   */
    p = pWC;
    nTerm = 0 as i32;
    '__slate_break_1747: while p != std::ptr::null_mut::<WhereClause>() {
        i = 0 as i32;
        let __v2064: *mut WhereTerm = unsafe { (*p).a };
        pTerm = __v2064;
        '__slate_break_1748: while i < unsafe { (*p).nTerm } {
            let __v2069: *mut WhereTerm = pTerm;
            let __v2070: u16 = unsafe { (*__v2069).wtFlags };
            let __v2071: u16 = ((((__v2070 as u32) as i32) & !(64 as i32)) as i16) as u16;
            unsafe {
                (*__v2069).wtFlags = __v2071;
            }
            if (unsafe { (*pTerm).leftCursor }) != unsafe { (*pSrc).iCursor } {
            } else {
                if (unsafe { (*pTerm).prereqRight }) & mUnusable != (0 as u64) {
                } else {
                    0 as i32;
                    {}
                    {}
                    {}
                    {}
                    if (((unsafe { (*pTerm).eOperator }) as u32) as i32) & !(2048 as i32)
                        == (0 as i32)
                    {
                    } else {
                        if (((unsafe { (*pTerm).wtFlags }) as u32) as i32) & (128 as i32)
                            != (0 as i32)
                        {
                        } else {
                            0 as i32;
                            0 as i32;
                            0 as i32;
                            let __v2072: bool;
                            if (((unsafe { (*pSrc).fg.jointype }) as u32) as i32)
                                & ((8 as i32) | (64 as i32) | (16 as i32))
                                != (0 as i32)
                            {
                                __v2072 = !(constraintCompatibleWithOuterJoin(
                                    pTerm as *const WhereTerm,
                                    pSrc as *const SrcItem,
                                ) != (0 as i32));
                            } else {
                                __v2072 = false as bool;
                            }
                            if __v2072 {
                            } else {
                                let __v2073: i32 = nTerm;
                                let __v2074: i32 = __v2073 + (1 as i32);
                                nTerm = __v2074;
                                let __v2075: *mut WhereTerm = pTerm;
                                let __v2076: u16 = unsafe { (*__v2075).wtFlags };
                                let __v2077: u16 =
                                    ((((__v2076 as u32) as i32) | (64 as i32)) as i16) as u16;
                                unsafe {
                                    (*__v2075).wtFlags = __v2077;
                                }
                            }
                        }
                    }
                }
            }
            let __v2065: i32 = i;
            let __v2066: i32 = __v2065 + (1 as i32);
            i = __v2066;
            let __v2067: *mut WhereTerm = pTerm;
            let __v2068: *mut WhereTerm = unsafe { __v2067.offset((1 as i32) as isize) };
            pTerm = __v2068;
        }
        p = unsafe { (*p).pOuter };
    }
    // /* If the ORDER BY clause contains only columns in the current
    //   ** virtual table then allocate space for the aOrderBy part of
    //   ** the sqlite3_index_info structure.
    //   */
    nOrderBy = 0 as i32;
    if pOrderBy != std::ptr::null_mut::<ExprList>() {
        let mut n: i32 = unsafe { (*pOrderBy).nExpr };
        i = 0 as i32;
        '__slate_break_1749: loop {
            if !(i < n) {
                break;
            }
            '__slate_continue_1749: {
                let mut pExpr: *mut Expr = unsafe {
                    (*unsafe {
                        unsafe { std::ptr::addr_of_mut!((*pOrderBy).a) as *mut ExprList_item }
                            .offset(i as isize)
                    })
                    .pExpr
                };
                let mut pE2: *mut Expr = unsafe { std::mem::zeroed() };
                // /* Skip over constant terms in the ORDER BY clause */
                if (unsafe { sqlite3ExprIsConstant(std::ptr::null_mut::<Parse>(), pExpr) })
                    != (0 as i32)
                {
                } else {
                    // /* Virtual tables are unable to deal with NULLS FIRST */
                    if (((unsafe {
                        (*unsafe {
                            unsafe { std::ptr::addr_of_mut!((*pOrderBy).a) as *mut ExprList_item }
                                .offset(i as isize)
                        })
                        .fg
                        .sortFlags
                    }) as u32) as i32)
                        & (2 as i32)
                        != (0 as i32)
                    {
                        break '__slate_break_1749;
                    }
                    // /* First case - a direct column references without a COLLATE operator */
                    if (((unsafe { (*pExpr).op }) as u32) as i32) == (168 as i32)
                        && (unsafe { (*pExpr).iTable }) == unsafe { (*pSrc).iCursor }
                    {
                        0 as i32;
                    } else {
                        // /* 2nd case - a column reference with a COLLATE operator.  Only match
                        //       ** of the COLLATE operator matches the collation of the column. */
                        let __v2080: bool;
                        if (((unsafe { (*pExpr).op }) as u32) as i32) == (114 as i32) {
                            let __v2081: *mut Expr = unsafe { (*pExpr).pLeft };
                            pE2 = __v2081;
                            __v2080 = (((unsafe { (*__v2081).op }) as u32) as i32) == (168 as i32);
                        } else {
                            __v2080 = false as bool;
                        }
                        if __v2080 && (unsafe { (*pE2).iTable }) == unsafe { (*pSrc).iCursor } {
                            // /* The collating sequence name */
                            let mut zColl: *const i8 = unsafe { std::mem::zeroed() };
                            0 as i32;
                            0 as i32;
                            0 as i32;
                            unsafe {
                                (*pExpr).iColumn = unsafe { (*pE2).iColumn };
                            }
                            // /* Collseq does not matter for rowid */
                            if ((unsafe { (*pE2).iColumn }) as i32) < (0 as i32) {
                                break '__slate_continue_1749;
                            }
                            zColl = unsafe {
                                sqlite3ColumnColl(unsafe {
                                    unsafe { (*pTab).aCol }
                                        .offset(((unsafe { (*pE2).iColumn }) as i32) as isize)
                                })
                            };
                            if zColl == std::ptr::null::<i8>() {
                                zColl =
                                    unsafe { std::ptr::addr_of!(sqlite3StrBINARY) as *const i8 };
                            }
                            if (unsafe {
                                sqlite3_stricmp((unsafe { (*pExpr).u.zToken }) as *const i8, zColl)
                            }) == (0 as i32)
                            {
                                break '__slate_continue_1749;
                            }
                        }
                        // /* No matches cause a break out of the loop */
                        break '__slate_break_1749;
                    }
                }
            }
            let __v2078: i32 = i;
            let __v2079: i32 = __v2078 + (1 as i32);
            i = __v2079;
        }
        if i == n {
            let mut bSortByGroup: i32 = ((((unsafe { (*pWInfo).wctrlFlags }) as u32) as i32)
                & (512 as i32)
                != (0 as i32)) as i32;
            nOrderBy = n;
            if (((unsafe { (*pWInfo).wctrlFlags }) as u32) as i32) & (128 as i32) != (0 as i32)
                && !(((unsafe { (*pSrc).fg.__slate_bits_0.__get_rowidUsed() }) as i32)
                    != (0 as i32))
            {
                eDistinct = (2 as i32) + bSortByGroup;
            } else {
                if (((unsafe { (*pWInfo).wctrlFlags }) as u32) as i32) & (64 as i32) != (0 as i32) {
                    eDistinct = (1 as i32) - bSortByGroup;
                } else {
                    if (((unsafe { (*pWInfo).wctrlFlags }) as u32) as i32) & (256 as i32)
                        != (0 as i32)
                    {
                        eDistinct = 3 as i32;
                    }
                }
            }
        }
    }
    // /* Allocate the sqlite3_index_info structure
    //   */
    pIdxInfo = (unsafe {
        sqlite3DbMallocZero(
            unsafe { (*pParse).db },
            (96 as u64)
                .wrapping_add(
                    (12 as u64)
                        .wrapping_add(8 as u64)
                        .wrapping_mul((nTerm as i64) as u64),
                )
                .wrapping_add((8 as u64).wrapping_mul((nOrderBy as i64) as u64))
                .wrapping_add(
                    (32 as u64).wrapping_add(((nTerm as i64) as u64).wrapping_mul(8 as u64)),
                ),
        )
    }) as *mut sqlite3_index_info;
    if pIdxInfo == std::ptr::null_mut::<sqlite3_index_info>() {
        unsafe {
            sqlite3ErrorMsg(
                pParse,
                (b"out of memory\0".as_ptr() as *mut i8) as *const i8,
            )
        };
        return std::ptr::null_mut::<sqlite3_index_info>();
    }
    pHidden = (unsafe { pIdxInfo.offset((1 as i32) as isize) }) as *mut HiddenIndexInfo;
    pIdxCons = (unsafe {
        unsafe { std::ptr::addr_of_mut!((*pHidden).aRhs) as *mut *mut sqlite3_value }
            .offset(nTerm as isize)
    }) as *mut sqlite3_index_constraint;
    pIdxOrderBy = (unsafe { pIdxCons.offset(nTerm as isize) }) as *mut sqlite3_index_orderby;
    pUsage =
        (unsafe { pIdxOrderBy.offset(nOrderBy as isize) }) as *mut sqlite3_index_constraint_usage;
    unsafe {
        (*pIdxInfo).aConstraint = pIdxCons;
    }
    unsafe {
        (*pIdxInfo).aOrderBy = pIdxOrderBy;
    }
    unsafe {
        (*pIdxInfo).aConstraintUsage = pUsage;
    }
    unsafe {
        (*pIdxInfo).colUsed = ((unsafe { (*pSrc).colUsed }) as i64) as u64;
    }
    if (((unsafe { (*pTab).tabFlags }) & ((128 as i32) as u32) == ((0 as i32) as u32)) as i32)
        == (0 as i32)
    {
        // /* Ensure that all bits associated with PK columns are set. This is to
        //     ** ensure they are available for cases like RIGHT joins or OR loops. */
        let mut pPk: *mut Index = unsafe { sqlite3PrimaryKeyIndex(pTab as *mut Table) };
        0 as i32;
        i = 0 as i32;
        '__slate_break_1751: loop {
            if !(i < (((unsafe { (*pPk).nKeyCol }) as u32) as i32)) {
                break;
            }
            let mut iCol: i32 =
                (unsafe { *unsafe { unsafe { (*pPk).aiColumn }.offset(i as isize) } }) as i32;
            0 as i32;
            if iCol
                >= (((8 as u64).wrapping_mul(((8 as i32) as i64) as u64) as u32) as i32)
                    - (1 as i32)
            {
                iCol = (((8 as u64).wrapping_mul(((8 as i32) as i64) as u64) as u32) as i32)
                    - (1 as i32);
            }
            let __v2084: *mut sqlite3_index_info = pIdxInfo;
            let __v2085: u64 = unsafe { (*__v2084).colUsed };
            let __v2086: u64 = __v2085 | (((1 as i32) as i64) as u64) << iCol;
            unsafe {
                (*__v2084).colUsed = __v2086;
            }
            let __v2082: i32 = i;
            let __v2083: i32 = __v2082 + (1 as i32);
            i = __v2083;
        }
    }
    unsafe {
        (*pHidden).pWC = pWC;
    }
    unsafe {
        (*pHidden).pParse = pParse;
    }
    unsafe {
        (*pHidden).eDistinct = eDistinct;
    }
    unsafe {
        (*pHidden).mIn = (0 as i32) as u32;
    }
    p = pWC;
    j = 0 as i32;
    i = 0 as i32;
    '__slate_break_1752: while p != std::ptr::null_mut::<WhereClause>() {
        let mut nLast: i32 = i + unsafe { (*p).nTerm };
        {}
        pTerm = unsafe { (*p).a };
        '__slate_break_1753: while i < nLast {
            let mut op: u16 = 0 as u16;
            if (((unsafe { (*pTerm).wtFlags }) as u32) as i32) & (64 as i32) == (0 as i32) {
            } else {
                unsafe {
                    (*unsafe { pIdxCons.offset(j as isize) }).iColumn =
                        unsafe { (*pTerm).u.x.leftColumn };
                }
                unsafe {
                    (*unsafe { pIdxCons.offset(j as isize) }).iTermOffset = i;
                }
                op = (((((unsafe { (*pTerm).eOperator }) as u32) as i32) & (16383 as i32)) as i16)
                    as u16;
                if ((op as u32) as i32) == (1 as i32) {
                    if (((unsafe { (*pTerm).wtFlags }) as u32) as i32) & (32768 as i32)
                        == (0 as i32)
                    {
                        let __v2091: *mut HiddenIndexInfo = pHidden;
                        let __v2092: u32 = unsafe { (*__v2091).mIn };
                        let __v2093: u32 = __v2092
                            | if j <= (31 as i32) {
                                ((1 as i32) as u32) << j
                            } else {
                                (0 as i32) as u32
                            };
                        unsafe {
                            (*__v2091).mIn = __v2093;
                        }
                    }
                    op = ((2 as i32) as i16) as u16;
                }
                if ((op as u32) as i32) == (64 as i32) {
                    unsafe {
                        (*unsafe { pIdxCons.offset(j as isize) }).op = unsafe { (*pTerm).eMatchOp };
                    }
                } else {
                    if ((op as u32) as i32) & ((256 as i32) | (128 as i32)) != (0 as i32) {
                        if ((op as u32) as i32) == (256 as i32) {
                            unsafe {
                                (*unsafe { pIdxCons.offset(j as isize) }).op =
                                    ((71 as i32) as i8) as u8;
                            }
                        } else {
                            unsafe {
                                (*unsafe { pIdxCons.offset(j as isize) }).op =
                                    ((72 as i32) as i8) as u8;
                            }
                        }
                    } else {
                        unsafe {
                            (*unsafe { pIdxCons.offset(j as isize) }).op = op as u8;
                        }
                        // /* The direct assignment in the previous line is possible only because
                        //         ** the WO_ and SQLITE_INDEX_CONSTRAINT_ codes are identical.  The
                        //         ** following asserts verify this fact. */
                        0 as i32;
                        0 as i32;
                        0 as i32;
                        0 as i32;
                        0 as i32;
                        0 as i32;
                        let __v2094: bool;
                        if ((op as u32) as i32)
                            & ((2 as i32) << (57 as i32) - (54 as i32)
                                | (2 as i32) << (56 as i32) - (54 as i32)
                                | (2 as i32) << (55 as i32) - (54 as i32)
                                | (2 as i32) << (58 as i32) - (54 as i32))
                            != (0 as i32)
                        {
                            __v2094 = (unsafe {
                                sqlite3ExprIsVector(
                                    (unsafe { (*unsafe { (*pTerm).pExpr }).pRight }) as *const Expr,
                                )
                            }) != (0 as i32);
                        } else {
                            __v2094 = false as bool;
                        }
                        if __v2094 {
                            {}
                            if j < (16 as i32) {
                                let __v2095: u16 = mNoOmit;
                                let __v2096: u16 =
                                    ((((__v2095 as u32) as i32) | (1 as i32) << j) as i16) as u16;
                                mNoOmit = __v2096;
                            }
                            if ((op as u32) as i32) == (2 as i32) << (57 as i32) - (54 as i32) {
                                unsafe {
                                    (*unsafe { pIdxCons.offset(j as isize) }).op =
                                        (((2 as i32) << (56 as i32) - (54 as i32)) as i8) as u8;
                                }
                            }
                            if ((op as u32) as i32) == (2 as i32) << (55 as i32) - (54 as i32) {
                                unsafe {
                                    (*unsafe { pIdxCons.offset(j as isize) }).op =
                                        (((2 as i32) << (58 as i32) - (54 as i32)) as i8) as u8;
                                }
                            }
                        }
                    }
                }
                let __v2097: i32 = j;
                let __v2098: i32 = __v2097 + (1 as i32);
                j = __v2098;
            }
            let __v2087: i32 = i;
            let __v2088: i32 = __v2087 + (1 as i32);
            i = __v2088;
            let __v2089: *mut WhereTerm = pTerm;
            let __v2090: *mut WhereTerm = unsafe { __v2089.offset((1 as i32) as isize) };
            pTerm = __v2090;
        }
        p = unsafe { (*p).pOuter };
    }
    0 as i32;
    unsafe {
        (*pIdxInfo).nConstraint = j;
    }
    j = 0 as i32;
    i = 0 as i32;
    '__slate_break_1754: loop {
        if !(i < nOrderBy) {
            break;
        }
        let mut pExpr: *mut Expr = unsafe {
            (*unsafe {
                unsafe { std::ptr::addr_of_mut!((*pOrderBy).a) as *mut ExprList_item }
                    .offset(i as isize)
            })
            .pExpr
        };
        if (unsafe { sqlite3ExprIsConstant(std::ptr::null_mut::<Parse>(), pExpr) }) != (0 as i32) {
        } else {
            0 as i32;
            unsafe {
                (*unsafe { pIdxOrderBy.offset(j as isize) }).iColumn =
                    (unsafe { (*pExpr).iColumn }) as i32;
            }
            unsafe {
                (*unsafe { pIdxOrderBy.offset(j as isize) }).desc =
                    (((((unsafe {
                        (*unsafe {
                            unsafe { std::ptr::addr_of_mut!((*pOrderBy).a) as *mut ExprList_item }
                                .offset(i as isize)
                        })
                        .fg
                        .sortFlags
                    }) as u32) as i32)
                        & (1 as i32)) as i8) as u8;
            }
            let __v2101: i32 = j;
            let __v2102: i32 = __v2101 + (1 as i32);
            j = __v2102;
        }
        let __v2099: i32 = i;
        let __v2100: i32 = __v2099 + (1 as i32);
        i = __v2100;
    }
    unsafe {
        (*pIdxInfo).nOrderBy = j;
    }
    unsafe {
        *pmNoOmit = mNoOmit;
    }
    return pIdxInfo;
}

// /* The WHERE clause */
// /* The WHERE clause being analyzed */
// /* Ignore terms with these prereqs */
// /* The FROM clause term that is the vtab */
// /* Mask of terms not to omit */
// /*
// ** Free and zero the sqlite3_index_info.idxStr value if needed.
// */
fn freeIdxStr(mut pIdxInfo: *mut sqlite3_index_info) {
    if (unsafe { (*pIdxInfo).needToFreeIdxStr }) != (0 as i32) {
        unsafe { sqlite3_free((unsafe { (*pIdxInfo).idxStr }) as *mut ()) };
        unsafe {
            (*pIdxInfo).idxStr = std::ptr::null_mut::<i8>();
        }
        unsafe {
            (*pIdxInfo).needToFreeIdxStr = 0 as i32;
        }
    }
}

// /*
// ** Free an sqlite3_index_info structure allocated by allocateIndexInfo()
// ** and possibly modified by xBestIndex methods.
// */
fn freeIndexInfo(mut db: *mut sqlite3, mut pIdxInfo: *mut sqlite3_index_info) {
    let mut pHidden: *mut HiddenIndexInfo = unsafe { std::mem::zeroed() };
    let mut i: i32 = 0 as i32;
    0 as i32;
    pHidden = (unsafe { pIdxInfo.offset((1 as i32) as isize) }) as *mut HiddenIndexInfo;
    0 as i32;
    0 as i32;
    i = 0 as i32;
    '__slate_break_1755: loop {
        if !(i < unsafe { (*pIdxInfo).nConstraint }) {
            break;
        }
        // /* IMP: R-14553-25174 */
        unsafe {
            sqlite3ValueFree(unsafe {
                *unsafe {
                    unsafe { std::ptr::addr_of_mut!((*pHidden).aRhs) as *mut *mut sqlite3_value }
                        .offset(i as isize)
                }
            })
        };
        unsafe {
            *unsafe {
                unsafe { std::ptr::addr_of_mut!((*pHidden).aRhs) as *mut *mut sqlite3_value }
                    .offset(i as isize)
            } = std::ptr::null_mut::<sqlite3_value>();
        }
        let __v2103: i32 = i;
        let __v2104: i32 = __v2103 + (1 as i32);
        i = __v2104;
    }
    freeIdxStr(pIdxInfo);
    unsafe { sqlite3DbFree(db, pIdxInfo as *mut ()) };
}

// /*
// ** The table object reference passed as the second argument to this function
// ** must represent a virtual table. This function invokes the xBestIndex()
// ** method of the virtual table with the sqlite3_index_info object that
// ** comes in as the 3rd argument to this function.
// **
// ** If an error occurs, pParse is populated with an error message and an
// ** appropriate error code is returned.  A return of SQLITE_CONSTRAINT from
// ** xBestIndex is not considered an error.  SQLITE_CONSTRAINT indicates that
// ** the current configuration of "unusable" flags in sqlite3_index_info can
// ** not result in a valid plan.
// **
// ** Whether or not an error is returned, it is the responsibility of the
// ** caller to eventually free p->idxStr if p->needToFreeIdxStr indicates
// ** that this is required.
// */
fn vtabBestIndex(
    mut pParse: *mut Parse,
    mut pTab: *mut Table,
    mut p: *mut sqlite3_index_info,
) -> i32 {
    let mut rc: i32 = 0 as i32;
    let mut pVtab: *mut sqlite3_vtab = unsafe { std::mem::zeroed() };
    0 as i32;
    pVtab = unsafe { (*unsafe { sqlite3GetVTable(unsafe { (*pParse).db }, pTab) }).pVtab };
    {}
    let __v2105: *mut sqlite3 = unsafe { (*pParse).db };
    let __v2106: u32 = unsafe { (*__v2105).nSchemaLock };
    let __v2107: u32 = __v2106.wrapping_add((1 as i32) as u32);
    unsafe {
        (*__v2105).nSchemaLock = __v2107;
    }
    rc = unsafe { unsafe { (*unsafe { (*pVtab).pModule }).xBestIndex }.unwrap()(pVtab, p) };
    let __v2108: *mut sqlite3 = unsafe { (*pParse).db };
    let __v2109: u32 = unsafe { (*__v2108).nSchemaLock };
    let __v2110: u32 = __v2109.wrapping_sub((1 as i32) as u32);
    unsafe {
        (*__v2108).nSchemaLock = __v2110;
    }
    {}
    if rc != (0 as i32) && rc != (19 as i32) {
        if rc == (7 as i32) {
            unsafe { sqlite3OomFault(unsafe { (*pParse).db }) };
        } else {
            if !((unsafe { (*pVtab).zErrMsg }) != std::ptr::null_mut::<i8>()) {
                unsafe {
                    sqlite3ErrorMsg(pParse, (b"%s\0".as_ptr() as *mut i8) as *const i8, unsafe {
                        sqlite3ErrStr(rc)
                    })
                };
            } else {
                unsafe {
                    sqlite3ErrorMsg(pParse, (b"%s\0".as_ptr() as *mut i8) as *const i8, unsafe {
                        (*pVtab).zErrMsg
                    })
                };
            }
        }
    }
    if (unsafe { (*unsafe { (*pTab).u.vtab.p }).bAllSchemas }) != (0 as u8) {
        sqlite3VtabUsesAllSchemas(pParse);
    }
    unsafe { sqlite3_free((unsafe { (*pVtab).zErrMsg }) as *mut ()) };
    unsafe {
        (*pVtab).zErrMsg = std::ptr::null_mut::<i8>();
    }
    return rc;
}

// /* !defined(SQLITE_OMIT_VIRTUALTABLE) */
// /*
// ** If it is not NULL, pTerm is a term that provides an upper or lower
// ** bound on a range scan. Without considering pTerm, it is estimated
// ** that the scan will visit nNew rows. This function returns the number
// ** estimated to be visited after taking pTerm into account.
// **
// ** If the user explicitly specified a likelihood() value for this term,
// ** then the return value is the likelihood multiplied by the number of
// ** input rows. Otherwise, this function assumes that an "IS NOT NULL" term
// ** has a likelihood of 0.50, and any other term a likelihood of 0.25.
// */
fn whereRangeAdjust(mut pTerm: *mut WhereTerm, mut nNew: i16) -> i16 {
    let mut nRet: i16 = nNew;
    if pTerm != std::ptr::null_mut::<WhereTerm>() {
        if ((unsafe { (*pTerm).truthProb }) as i32) <= (0 as i32) {
            let __v2111: i16 = nRet;
            let __v2112: i16 = ((__v2111 as i32) + ((unsafe { (*pTerm).truthProb }) as i32)) as i16;
            nRet = __v2112;
        } else {
            if (((unsafe { (*pTerm).wtFlags }) as u32) as i32) & (128 as i32) == (0 as i32) {
                let __v2113: i16 = nRet;
                let __v2114: i16 = ((__v2113 as i32) - (20 as i32)) as i16;
                nRet = __v2114;
                0 as i32;
            }
        }
    }
    return nRet;
}

// /*
// ** This function is used to estimate the number of rows that will be visited
// ** by scanning an index for a range of values. The range may have an upper
// ** bound, a lower bound, or both. The WHERE clause terms that set the upper
// ** and lower bounds are represented by pLower and pUpper respectively. For
// ** example, assuming that index p is on t1(a):
// **
// **   ... FROM t1 WHERE a > ? AND a < ? ...
// **                    |_____|   |_____|
// **                       |         |
// **                     pLower    pUpper
// **
// ** If either of the upper or lower bound is not present, then NULL is passed in
// ** place of the corresponding WhereTerm.
// **
// ** The value in (pBuilder->pNew->u.btree.nEq) is the number of the index
// ** column subject to the range constraint. Or, equivalently, the number of
// ** equality constraints optimized by the proposed index scan. For example,
// ** assuming index p is on t1(a, b), and the SQL query is:
// **
// **   ... FROM t1 WHERE a = ? AND b > ? AND b < ? ...
// **
// ** then nEq is set to 1 (as the range restricted column, b, is the second
// ** left-most column of the index). Or, if the query is:
// **
// **   ... FROM t1 WHERE a > ? AND a < ? ...
// **
// ** then nEq is set to 0.
// **
// ** When this function is called, *pnOut is set to the sqlite3LogEst() of the
// ** number of rows that the index scan is expected to visit without
// ** considering the range constraints. If nEq is 0, then *pnOut is the number of
// ** rows in the index. Assuming no error occurs, *pnOut is adjusted (reduced)
// ** to account for the range constraints pLower and pUpper.
// **
// ** In the absence of sqlite_stat4 ANALYZE data, or if such data cannot be
// ** used, a single range inequality reduces the search space by a factor of 4.
// ** and a pair of constraints (x>? AND x<?) reduces the expected number of
// ** rows visited by a factor of 64.
// */
fn whereRangeScanEst(
    mut pParse: *mut Parse,
    mut pBuilder: *mut WhereLoopBuilder,
    mut pLower: *mut WhereTerm,
    mut pUpper: *mut WhereTerm,
    mut pLoop: *mut WhereLoop,
) -> i32 {
    let mut rc: i32 = 0 as i32;
    let mut nOut: i32 = (unsafe { (*pLoop).nOut }) as i32;
    let mut nNew: i16 = 0 as i16;
    pParse;
    pBuilder;
    0 as i32;
    0 as i32;
    nNew = whereRangeAdjust(pLower, nOut as i16);
    nNew = whereRangeAdjust(pUpper, nNew);
    // /* TUNING: If there is both an upper and lower limit and neither limit
    //   ** has an application-defined likelihood(), assume the range is
    //   ** reduced by an additional 75%. This means that, by default, an open-ended
    //   ** range query (e.g. col > ?) is assumed to match 1/4 of the rows in the
    //   ** index. While a closed range (e.g. col BETWEEN ? AND ?) is estimated to
    //   ** match 1/64 of the index. */
    if pLower != std::ptr::null_mut::<WhereTerm>()
        && ((unsafe { (*pLower).truthProb }) as i32) > (0 as i32)
        && pUpper != std::ptr::null_mut::<WhereTerm>()
        && ((unsafe { (*pUpper).truthProb }) as i32) > (0 as i32)
    {
        let __v2115: i16 = nNew;
        let __v2116: i16 = ((__v2115 as i32) - (20 as i32)) as i16;
        nNew = __v2116;
    }
    let __v2117: i32 = nOut;
    let __v2118: i32 = __v2117
        - (((pLower != std::ptr::null_mut::<WhereTerm>()) as i32)
            + ((pUpper != std::ptr::null_mut::<WhereTerm>()) as i32));
    nOut = __v2118;
    if (nNew as i32) < (10 as i32) {
        nNew = (10 as i32) as i16;
    }
    if (nNew as i32) < nOut {
        nOut = nNew as i32;
    }
    unsafe {
        (*pLoop).nOut = nOut as i16;
    }
    return rc;
}

// /* Parsing & code generating context */
// /* Lower bound on the range. ex: "x>123" Might be NULL */
// /* Upper bound on the range. ex: "x<455" Might be NULL */
// /* Modify the .nOut and maybe .rRun fields */
// /*
// ** Convert bulk memory into a valid WhereLoop that can be passed
// ** to whereLoopClear harmlessly.
// */
fn whereLoopInit(mut p: *mut WhereLoop) {
    unsafe {
        (*p).aLTerm = unsafe { (*p).aLTermSpace.as_mut_ptr() as *mut *mut WhereTerm };
    }
    unsafe {
        (*p).nLTerm = ((0 as i32) as i16) as u16;
    }
    unsafe {
        (*p).nLSlot = (((((24 as u64) / (8 as u64)) as u32) as i32) as i16) as u16;
    }
    unsafe {
        (*p).wsFlags = (0 as i32) as u32;
    }
}

// /*
// ** Clear the WhereLoop.u union.  Leave WhereLoop.pLTerm intact.
// */
fn whereLoopClearUnion(mut db: *mut sqlite3, mut p: *mut WhereLoop) {
    if (unsafe { (*p).wsFlags }) & (((1024 as i32) | (16384 as i32)) as u32) != (0 as u32) {
        if (unsafe { (*p).wsFlags }) & ((1024 as i32) as u32) != ((0 as i32) as u32)
            && ((unsafe { (*p).u.vtab.__slate_bits_0.__get_needFree() }) as i32) != (0 as i32)
        {
            unsafe { sqlite3_free((unsafe { (*p).u.vtab.idxStr }) as *mut ()) };
            unsafe {
                (*p).u.vtab.__slate_bits_0.__set_needFree((0 as i32) as u32);
            }
            unsafe {
                (*p).u.vtab.idxStr = std::ptr::null_mut::<i8>();
            }
        } else {
            if (unsafe { (*p).wsFlags }) & ((16384 as i32) as u32) != ((0 as i32) as u32)
                && (unsafe { (*p).u.btree.pIndex }) != std::ptr::null_mut::<Index>()
            {
                unsafe {
                    sqlite3DbFree(
                        db,
                        (unsafe { (*unsafe { (*p).u.btree.pIndex }).zColAff }) as *mut (),
                    )
                };
                unsafe { sqlite3DbFreeNN(db, (unsafe { (*p).u.btree.pIndex }) as *mut ()) };
                unsafe {
                    (*p).u.btree.pIndex = std::ptr::null_mut::<Index>();
                }
            }
        }
    }
}

// /*
// ** Deallocate internal memory used by a WhereLoop object.  Leave the
// ** object in an initialized state, as if it had been newly allocated.
// */
fn whereLoopClear(mut db: *mut sqlite3, mut p: *mut WhereLoop) {
    if (unsafe { (*p).aLTerm }) != unsafe { (*p).aLTermSpace.as_mut_ptr() as *mut *mut WhereTerm } {
        unsafe { sqlite3DbFreeNN(db, (unsafe { (*p).aLTerm }) as *mut ()) };
        unsafe {
            (*p).aLTerm = unsafe { (*p).aLTermSpace.as_mut_ptr() as *mut *mut WhereTerm };
        }
        unsafe {
            (*p).nLSlot = (((((24 as u64) / (8 as u64)) as u32) as i32) as i16) as u16;
        }
    }
    whereLoopClearUnion(db, p);
    unsafe {
        (*p).nLTerm = ((0 as i32) as i16) as u16;
    }
    unsafe {
        (*p).wsFlags = (0 as i32) as u32;
    }
}

// /*
// ** Transfer content from the second pLoop into the first.
// */
fn whereLoopXfer(mut db: *mut sqlite3, mut pTo: *mut WhereLoop, mut pFrom: *mut WhereLoop) -> i32 {
    whereLoopClearUnion(db, pTo);
    let __v2119: bool;
    if (((unsafe { (*pFrom).nLTerm }) as u32) as i32) > (((unsafe { (*pTo).nLSlot }) as u32) as i32)
    {
        __v2119 =
            whereLoopResize(db, pTo, ((unsafe { (*pFrom).nLTerm }) as u32) as i32) != (0 as i32);
    } else {
        __v2119 = false as bool;
    }
    if __v2119 {
        unsafe { memset(pTo as *mut (), 0 as i32, 56 as u64) };
        return 7 as i32;
    }
    unsafe { memcpy(pTo as *mut (), pFrom as *const (), 56 as u64) };
    unsafe {
        memcpy(
            (unsafe { (*pTo).aLTerm }) as *mut (),
            (unsafe { (*pFrom).aLTerm }) as *const (),
            (((((unsafe { (*pTo).nLTerm }) as u32) as i32) as i64) as u64).wrapping_mul(8 as u64),
        )
    };
    if (unsafe { (*pFrom).wsFlags }) & ((1024 as i32) as u32) != (0 as u32) {
        unsafe {
            (*pFrom)
                .u
                .vtab
                .__slate_bits_0
                .__set_needFree((0 as i32) as u32);
        }
    } else {
        if (unsafe { (*pFrom).wsFlags }) & ((16384 as i32) as u32) != ((0 as i32) as u32) {
            unsafe {
                (*pFrom).u.btree.pIndex = std::ptr::null_mut::<Index>();
            }
        }
    }
    return 0 as i32;
}

// /*
// ** Delete a WhereLoop object
// */
fn whereLoopDelete(mut db: *mut sqlite3, mut p: *mut WhereLoop) {
    0 as i32;
    whereLoopClear(db, p);
    unsafe { sqlite3DbNNFreeNN(db, p as *mut ()) };
}

// /*
// ** Free a WhereInfo structure
// */
fn whereInfoFree(mut db: *mut sqlite3, mut pWInfo: *mut WhereInfo) {
    0 as i32;
    0 as i32;
    unsafe { sqlite3WhereClauseClear(unsafe { std::ptr::addr_of_mut!((*pWInfo).sWC) }) };
    '__slate_break_1758: while (unsafe { (*pWInfo).pLoops }) != std::ptr::null_mut::<WhereLoop>() {
        let mut p: *mut WhereLoop = unsafe { (*pWInfo).pLoops };
        unsafe {
            (*pWInfo).pLoops = unsafe { (*p).pNextLoop };
        }
        whereLoopDelete(db, p);
    }
    '__slate_break_1759: while (unsafe { (*pWInfo).pMemToFree })
        != std::ptr::null_mut::<WhereMemBlock>()
    {
        let mut pNext: *mut WhereMemBlock = unsafe { (*unsafe { (*pWInfo).pMemToFree }).pNext };
        unsafe { sqlite3DbNNFreeNN(db, (unsafe { (*pWInfo).pMemToFree }) as *mut ()) };
        unsafe {
            (*pWInfo).pMemToFree = pNext;
        }
    }
    unsafe { sqlite3DbNNFreeNN(db, pWInfo as *mut ()) };
}

// /*
// ** Return TRUE if X is a proper subset of Y but is of equal or less cost.
// ** In other words, return true if all constraints of X are also part of Y
// ** and Y has additional constraints that might speed the search that X lacks
// ** but the cost of running X is not more than the cost of running Y.
// **
// ** In other words, return true if the cost relationship between X and Y
// ** is inverted and needs to be adjusted.
// **
// ** Case 1:
// **
// **   (1a)  X and Y use the same index.
// **   (1b)  X has fewer == terms than Y
// **   (1c)  Neither X nor Y use skip-scan
// **   (1d)  X does not have a a greater cost than Y
// **
// ** Case 2:
// **
// **   (2a)  X has the same or lower cost, or returns the same or fewer rows,
// **         than Y.
// **   (2b)  X uses fewer WHERE clause terms than Y
// **   (2c)  Every WHERE clause term used by X is also used by Y
// **   (2d)  X skips at least as many columns as Y
// **   (2e)  If X is a covering index, than Y is too
// */
fn whereLoopCheaperProperSubset(mut pX: *const WhereLoop, mut pY: *const WhereLoop) -> i32 {
    let mut i: i32 = 0 as i32;
    let mut j: i32 = 0 as i32;
    // /* (1d) and (2a) */
    if ((unsafe { (*pX).rRun }) as i32) > ((unsafe { (*pY).rRun }) as i32)
        && ((unsafe { (*pX).nOut }) as i32) > ((unsafe { (*pY).nOut }) as i32)
    {
        return 0 as i32;
    }
    0 as i32;
    0 as i32;
    // /* (1b) */
    if (((unsafe { (*pX).u.btree.nEq }) as u32) as i32)
        < (((unsafe { (*pY).u.btree.nEq }) as u32) as i32)
        && (unsafe { (*pX).u.btree.pIndex }) == unsafe { (*pY).u.btree.pIndex }
        && (((unsafe { (*pX).nSkip }) as u32) as i32) == (0 as i32)
        && (((unsafe { (*pY).nSkip }) as u32) as i32) == (0 as i32)
    {
        // /* Case 1 is true */
        return 1 as i32;
    }
    // /* (1a) */
    // /* (1c) */
    if (((unsafe { (*pX).nLTerm }) as u32) as i32) - (((unsafe { (*pX).nSkip }) as u32) as i32)
        >= (((unsafe { (*pY).nLTerm }) as u32) as i32) - (((unsafe { (*pY).nSkip }) as u32) as i32)
    {
        // /* (2b) */
        return 0 as i32;
    }
    // /* (2d) */
    if (((unsafe { (*pY).nSkip }) as u32) as i32) > (((unsafe { (*pX).nSkip }) as u32) as i32) {
        return 0 as i32;
    }
    i = (((unsafe { (*pX).nLTerm }) as u32) as i32) - (1 as i32);
    '__slate_break_1760: loop {
        if !(i >= (0 as i32)) {
            break;
        }
        if (unsafe { *unsafe { unsafe { (*pX).aLTerm }.offset(i as isize) } })
            == std::ptr::null_mut::<WhereTerm>()
        {
        } else {
            j = (((unsafe { (*pY).nLTerm }) as u32) as i32) - (1 as i32);
            '__slate_break_1761: loop {
                if !(j >= (0 as i32)) {
                    break;
                }
                if (unsafe { *unsafe { unsafe { (*pY).aLTerm }.offset(j as isize) } })
                    == unsafe { *unsafe { unsafe { (*pX).aLTerm }.offset(i as isize) } }
                {
                    break '__slate_break_1761;
                }
                let __v2122: i32 = j;
                let __v2123: i32 = __v2122 - (1 as i32);
                j = __v2123;
            }
            // /* (2c) */
            if j < (0 as i32) {
                return 0 as i32;
            }
        }
        let __v2120: i32 = i;
        let __v2121: i32 = __v2120 - (1 as i32);
        i = __v2121;
    }
    if (unsafe { (*pX).wsFlags }) & ((64 as i32) as u32) != ((0 as i32) as u32)
        && (unsafe { (*pY).wsFlags }) & ((64 as i32) as u32) == ((0 as i32) as u32)
    {
        // /* (2e) */
        return 0 as i32;
    }
    // /* Case 2 is true */
    return 1 as i32;
}

// /* First WhereLoop to compare */
// /* Compare against this WhereLoop */
// /*
// ** Try to adjust the cost and number of output rows of WhereLoop pTemplate
// ** upwards or downwards so that:
// **
// **   (1) pTemplate costs less than any other WhereLoops that are a proper
// **       subset of pTemplate
// **
// **   (2) pTemplate costs more than any other WhereLoops for which pTemplate
// **       is a proper subset.
// **
// ** To say "WhereLoop X is a proper subset of Y" means that X uses fewer
// ** WHERE clause terms than Y and that every WHERE clause term used by X is
// ** also used by Y.
// */
fn whereLoopAdjustCost(mut p: *const WhereLoop, mut pTemplate: *mut WhereLoop) {
    if (unsafe { (*pTemplate).wsFlags }) & ((512 as i32) as u32) == ((0 as i32) as u32) {
        return;
    }
    '__slate_break_1762: while p != std::ptr::null::<WhereLoop>() {
        if (((unsafe { (*p).iTab }) as u32) as i32)
            != (((unsafe { (*pTemplate).iTab }) as u32) as i32)
        {
        } else {
            if (unsafe { (*p).wsFlags }) & ((512 as i32) as u32) == ((0 as i32) as u32) {
            } else {
                if whereLoopCheaperProperSubset(p, pTemplate as *const WhereLoop) != (0 as i32) {
                    // /* Adjust pTemplate cost downward so that it is cheaper than its
                    //       ** subset p. */
                    {}
                    unsafe {
                        (*pTemplate).rRun = (if ((unsafe { (*p).rRun }) as i32)
                            < ((unsafe { (*pTemplate).rRun }) as i32)
                        {
                            (unsafe { (*p).rRun }) as i32
                        } else {
                            (unsafe { (*pTemplate).rRun }) as i32
                        }) as i16;
                    }
                    unsafe {
                        (*pTemplate).nOut = (if ((unsafe { (*p).nOut }) as i32) - (1 as i32)
                            < ((unsafe { (*pTemplate).nOut }) as i32)
                        {
                            ((unsafe { (*p).nOut }) as i32) - (1 as i32)
                        } else {
                            (unsafe { (*pTemplate).nOut }) as i32
                        }) as i16;
                    }
                } else {
                    if whereLoopCheaperProperSubset(pTemplate as *const WhereLoop, p) != (0 as i32)
                    {
                        // /* Adjust pTemplate cost upward so that it is costlier than p since
                        //       ** pTemplate is a proper subset of p */
                        {}
                        unsafe {
                            (*pTemplate).rRun = (if ((unsafe { (*p).rRun }) as i32)
                                > ((unsafe { (*pTemplate).rRun }) as i32)
                            {
                                (unsafe { (*p).rRun }) as i32
                            } else {
                                (unsafe { (*pTemplate).rRun }) as i32
                            }) as i16;
                        }
                        unsafe {
                            (*pTemplate).nOut = (if ((unsafe { (*p).nOut }) as i32) + (1 as i32)
                                > ((unsafe { (*pTemplate).nOut }) as i32)
                            {
                                ((unsafe { (*p).nOut }) as i32) + (1 as i32)
                            } else {
                                (unsafe { (*pTemplate).nOut }) as i32
                            }) as i16;
                        }
                    }
                }
            }
        }
        p = (unsafe { (*p).pNextLoop }) as *const WhereLoop;
    }
}

// /*
// ** Search the list of WhereLoops in *ppPrev looking for one that can be
// ** replaced by pTemplate.
// **
// ** Return NULL if pTemplate does not belong on the WhereLoop list.
// ** In other words if pTemplate ought to be dropped from further consideration.
// **
// ** If pX is a WhereLoop that pTemplate can replace, then return the
// ** link that points to pX.
// **
// ** If pTemplate cannot replace any existing element of the list but needs
// ** to be added to the list as a new entry, then return a pointer to the
// ** tail of the list.
// */
fn whereLoopFindLesser(
    mut ppPrev: *mut *mut WhereLoop,
    mut pTemplate: *const WhereLoop,
) -> *mut *mut WhereLoop {
    let mut p: *mut WhereLoop = unsafe { std::mem::zeroed() };
    p = unsafe { *ppPrev };
    '__slate_break_1763: loop {
        if !(p != std::ptr::null_mut::<WhereLoop>()) {
            break;
        }
        if (((unsafe { (*p).iTab }) as u32) as i32)
            != (((unsafe { (*pTemplate).iTab }) as u32) as i32)
            || (((unsafe { (*p).iSortIdx }) as u32) as i32)
                != (((unsafe { (*pTemplate).iSortIdx }) as u32) as i32)
        {
            // /* If either the iTab or iSortIdx values for two WhereLoop are different
            //       ** then those WhereLoops need to be considered separately.  Neither is
            //       ** a candidate to replace the other. */
        } else {
            // /* In the current implementation, the rSetup value is either zero
            //     ** or the cost of building an automatic index (NlogN) and the NlogN
            //     ** is the same for compatible WhereLoops. */
            0 as i32;
            // /* whereLoopAddBtree() always generates and inserts the automatic index
            //     ** case first.  Hence compatible candidate WhereLoops never have a larger
            //     ** rSetup. Call this SETUP-INVARIANT */
            0 as i32;
            // /* Any loop using an application-defined index (or PRIMARY KEY or
            //     ** UNIQUE constraint) with one or more == constraints is better
            //     ** than an automatic index. Unless it is a skip-scan. */
            if (unsafe { (*p).wsFlags }) & ((16384 as i32) as u32) != ((0 as i32) as u32)
                && (((unsafe { (*pTemplate).nSkip }) as u32) as i32) == (0 as i32)
                && (unsafe { (*pTemplate).wsFlags }) & ((512 as i32) as u32) != ((0 as i32) as u32)
                && (unsafe { (*pTemplate).wsFlags }) & ((1 as i32) as u32) != ((0 as i32) as u32)
                && (unsafe { (*p).prereq }) & unsafe { (*pTemplate).prereq }
                    == unsafe { (*pTemplate).prereq }
            {
                break '__slate_break_1763;
            }
            // /* If existing WhereLoop p is better than pTemplate, pTemplate can be
            //     ** discarded.  WhereLoop p is better if:
            //     **   (1)  p has no more dependencies than pTemplate, and
            //     **   (2)  p has an equal or lower cost than pTemplate
            //     */
            // /* (1)  */
            if (unsafe { (*p).prereq }) & unsafe { (*pTemplate).prereq } == unsafe { (*p).prereq }
                && ((unsafe { (*p).rSetup }) as i32) <= ((unsafe { (*pTemplate).rSetup }) as i32)
                && ((unsafe { (*p).rRun }) as i32) <= ((unsafe { (*pTemplate).rRun }) as i32)
                && ((unsafe { (*p).nOut }) as i32) <= ((unsafe { (*pTemplate).nOut }) as i32)
            {
                // /* Discard pTemplate */
                return std::ptr::null_mut::<*mut WhereLoop>();
            }
            // /* (2a) */
            // /* (2b) */
            // /* (2c) */
            // /* If pTemplate is always better than p, then cause p to be overwritten
            //     ** with pTemplate.  pTemplate is better than p if:
            //     **   (1)  pTemplate has no more dependencies than p, and
            //     **   (2)  pTemplate has an equal or lower cost than p.
            //     */
            // /* (1)  */
            if (unsafe { (*p).prereq }) & unsafe { (*pTemplate).prereq }
                == unsafe { (*pTemplate).prereq }
                && ((unsafe { (*p).rRun }) as i32) >= ((unsafe { (*pTemplate).rRun }) as i32)
                && ((unsafe { (*p).nOut }) as i32) >= ((unsafe { (*pTemplate).nOut }) as i32)
            {
                // /* SETUP-INVARIANT above */
                0 as i32;
                // /* Cause p to be overwritten by pTemplate */
                break '__slate_break_1763;
            }
            // /* (2a) */
            // /* (2b) */
        }
        ppPrev = unsafe { std::ptr::addr_of_mut!((*p).pNextLoop) };
        let __v2124: *mut WhereLoop = unsafe { *ppPrev };
        p = __v2124;
    }
    return ppPrev;
}

// /*
// ** Insert or replace a WhereLoop entry using the template supplied.
// **
// ** An existing WhereLoop entry might be overwritten if the new template
// ** is better and has fewer dependencies.  Or the template will be ignored
// ** and no insert will occur if an existing WhereLoop is faster and has
// ** fewer dependencies than the template.  Otherwise a new WhereLoop is
// ** added based on the template.
// **
// ** If pBuilder->pOrSet is not NULL then we care about only the
// ** prerequisites and rRun and nOut costs of the N best loops.  That
// ** information is gathered in the pBuilder->pOrSet object.  This special
// ** processing mode is used only for OR clause processing.
// **
// ** When accumulating multiple loops (when pBuilder->pOrSet is NULL) we
// ** still might overwrite similar loops with the new template if the
// ** new template is better.  Loops may be overwritten if the following
// ** conditions are met:
// **
// **    (1)  They have the same iTab.
// **    (2)  They have the same iSortIdx.
// **    (3)  The template has same or fewer dependencies than the current loop
// **    (4)  The template has the same or lower cost than the current loop
// */
fn whereLoopInsert(mut pBuilder: *mut WhereLoopBuilder, mut pTemplate: *mut WhereLoop) -> i32 {
    let mut ppPrev: *mut *mut WhereLoop = unsafe { std::mem::zeroed() };
    let mut p: *mut WhereLoop = unsafe { std::mem::zeroed() };
    let mut pWInfo: *mut WhereInfo = unsafe { (*pBuilder).pWInfo };
    let mut db: *mut sqlite3 = unsafe { (*unsafe { (*pWInfo).pParse }).db };
    let mut rc: i32 = 0 as i32;
    // /* Stop the search once we hit the query planner search limit */
    if (unsafe { (*pBuilder).iPlanLimit }) == ((0 as i32) as u32) {
        {}
        if (unsafe { (*pBuilder).pOrSet }) != std::ptr::null_mut::<WhereOrSet>() {
            unsafe {
                (*unsafe { (*pBuilder).pOrSet }).n = ((0 as i32) as i16) as u16;
            }
        }
        return 101 as i32;
    }
    let __v2125: *mut WhereLoopBuilder = pBuilder;
    let __v2126: u32 = unsafe { (*__v2125).iPlanLimit };
    let __v2127: u32 = __v2126.wrapping_sub((1 as i32) as u32);
    unsafe {
        (*__v2125).iPlanLimit = __v2127;
    }
    whereLoopAdjustCost((unsafe { (*pWInfo).pLoops }) as *const WhereLoop, pTemplate);
    // /* If pBuilder->pOrSet is defined, then only keep track of the costs
    //   ** and prereqs.
    //   */
    if (unsafe { (*pBuilder).pOrSet }) != std::ptr::null_mut::<WhereOrSet>() {
        if (unsafe { (*pTemplate).nLTerm }) != (0 as u16) {
            whereOrInsert(
                unsafe { (*pBuilder).pOrSet },
                unsafe { (*pTemplate).prereq },
                unsafe { (*pTemplate).rRun },
                unsafe { (*pTemplate).nOut },
            );
            // /* 0x8 */
        }
        return 0 as i32;
    }
    // /* Look for an existing WhereLoop to replace with pTemplate
    //   */
    ppPrev = whereLoopFindLesser(
        unsafe { std::ptr::addr_of_mut!((*pWInfo).pLoops) },
        pTemplate as *const WhereLoop,
    );
    if ppPrev == std::ptr::null_mut::<*mut WhereLoop>() {
        // /* There already exists a WhereLoop on the list that is better
        //     ** than pTemplate, so just ignore pTemplate */
        // /* 0x8 */
        return 0 as i32;
    } else {
        p = unsafe { *ppPrev };
    }
    // /* If we reach this point it means that either p[] should be overwritten
    //   ** with pTemplate[] if p[] exists, or if p==NULL then allocate a new
    //   ** WhereLoop and insert it.
    //   */
    // /* 0x8 */
    if p == std::ptr::null_mut::<WhereLoop>() {
        // /* Allocate a new WhereLoop to add to the end of the list */
        let __v2128: *mut WhereLoop =
            (unsafe { sqlite3DbMallocRawNN(db, 104 as u64) }) as *mut WhereLoop;
        p = __v2128;
        unsafe {
            *ppPrev = __v2128;
        }
        if p == std::ptr::null_mut::<WhereLoop>() {
            return 7 as i32;
        }
        whereLoopInit(p);
        unsafe {
            (*p).pNextLoop = std::ptr::null_mut::<WhereLoop>();
        }
    } else {
        // /* We will be overwriting WhereLoop p[].  But before we do, first
        //     ** go through the rest of the list and delete any other entries besides
        //     ** p[] that are also supplanted by pTemplate */
        let mut ppTail: *mut *mut WhereLoop = unsafe { std::ptr::addr_of_mut!((*p).pNextLoop) };
        let mut pToDel: *mut WhereLoop = unsafe { std::mem::zeroed() };
        '__slate_break_1764: while (unsafe { *ppTail }) != std::ptr::null_mut::<WhereLoop>() {
            ppTail = whereLoopFindLesser(ppTail, pTemplate as *const WhereLoop);
            if ppTail == std::ptr::null_mut::<*mut WhereLoop>() {
                break '__slate_break_1764;
            }
            pToDel = unsafe { *ppTail };
            if pToDel == std::ptr::null_mut::<WhereLoop>() {
                break '__slate_break_1764;
            }
            unsafe {
                *ppTail = unsafe { (*pToDel).pNextLoop };
            }
            // /* 0x8 */
            whereLoopDelete(db, pToDel);
        }
    }
    rc = whereLoopXfer(db, p, pTemplate);
    if (unsafe { (*p).wsFlags }) & ((1024 as i32) as u32) == ((0 as i32) as u32) {
        let mut pIndex: *mut Index = unsafe { (*p).u.btree.pIndex };
        if pIndex != std::ptr::null_mut::<Index>()
            && ((unsafe { (*pIndex).__slate_bits_0.__get_idxType() }) as i32) == (3 as i32)
        {
            unsafe {
                (*p).u.btree.pIndex = std::ptr::null_mut::<Index>();
            }
        }
    }
    return rc;
}

// /*
// ** Callback for estLikePatternLength().
// **
// ** If this node is a string literal that is longer pWalker->sz, then set
// ** pWalker->sz to the byte length of that string literal.
// **
// ** pWalker->eCode indicates how to count characters:
// **
// **    eCode==0     Count as a GLOB pattern
// **    eCode==1     Count as a LIKE pattern
// */
#[unsafe(link_section = ".text.slate_distinct.where.exprNodePatternLengthEst")]
extern "C-unwind" fn exprNodePatternLengthEst(
    mut pWalker: *mut Walker,
    mut pExpr: *mut Expr,
) -> i32 {
    if (((unsafe { (*pExpr).op }) as u32) as i32) == (118 as i32) {
        // /* Pattern size in bytes */
        let mut sz: i32 = 0 as i32;
        // /* The pattern */
        let mut z: *mut u8 = (unsafe { (*pExpr).u.zToken }) as *mut u8;
        // /* Next character of the pattern */
        let mut c: u8 = 0 as u8;
        // /* Wildcards */
        let mut c1: u8 = 0 as u8;
        let mut c2: u8 = 0 as u8;
        let mut c3: u8 = 0 as u8;
        if (unsafe { (*pWalker).eCode }) != (0 as u16) {
            c1 = ((37 as i32) as i8) as u8;
            c2 = ((95 as i32) as i8) as u8;
            c3 = ((0 as i32) as i8) as u8;
        } else {
            c1 = ((42 as i32) as i8) as u8;
            c2 = ((63 as i32) as i8) as u8;
            c3 = ((91 as i32) as i8) as u8;
        }
        '__slate_break_1765: loop {
            let __v2129: *mut u8 = z;
            let __v2130: *mut u8 = unsafe { __v2129.offset((1 as i32) as isize) };
            z = __v2130;
            let __v2131: u8 = unsafe { *__v2129 };
            c = __v2131;
            if !(((__v2131 as u32) as i32) != (0 as i32)) {
                break;
            }
            if ((c as u32) as i32) == ((c3 as u32) as i32) {
                if (unsafe { *z }) != (0 as u8) {
                    let __v2132: *mut u8 = z;
                    let __v2133: *mut u8 = unsafe { __v2132.offset((1 as i32) as isize) };
                    z = __v2133;
                }
                '__slate_break_1766: while (unsafe { *z }) != (0 as u8)
                    && (((unsafe { *z }) as u32) as i32) != (93 as i32)
                {
                    let __v2134: *mut u8 = z;
                    let __v2135: *mut u8 = unsafe { __v2134.offset((1 as i32) as isize) };
                    z = __v2135;
                }
            } else {
                if ((c as u32) as i32) != ((c1 as u32) as i32)
                    && ((c as u32) as i32) != ((c2 as u32) as i32)
                {
                    let __v2136: i32 = sz;
                    let __v2137: i32 = __v2136 + (1 as i32);
                    sz = __v2137;
                }
            }
        }
        if sz > unsafe { (*pWalker).u.sz } {
            unsafe {
                (*pWalker).u.sz = sz;
            }
        }
    }
    return 0 as i32;
}

// /*
// ** Return the length of the longest string literal in the given
// ** expression.
// **
// ** eCode indicates how to count characters:
// **
// **    eCode==0     Count as a GLOB pattern
// **    eCode==1     Count as a LIKE pattern
// */
fn estLikePatternLength(mut p: *mut Expr, mut eCode: u16) -> i32 {
    let mut w: Walker = unsafe { std::mem::zeroed() };
    unsafe {
        w.u.sz = 0 as i32;
    }
    w.eCode = eCode;
    w.xExprCallback = Some(exprNodePatternLengthEst);
    w.xSelectCallback = unsafe {
        std::mem::transmute::<
            *const (),
            Option<unsafe extern "C-unwind" fn(*mut Walker, *mut Select) -> i32>,
        >(sqlite3SelectWalkFail as *const ())
    };
    unsafe { sqlite3WalkExpr(std::ptr::addr_of_mut!(w), p) };
    return unsafe { w.u.sz };
}

// /*
// ** Adjust the WhereLoop.nOut value downward to account for terms of the
// ** WHERE clause that reference the loop but which are not used by an
// ** index.
// *
// ** For every WHERE clause term that is not used by the index
// ** and which has a truth probability assigned by one of the likelihood(),
// ** likely(), or unlikely() SQL functions, reduce the estimated number
// ** of output rows by the probability specified.
// **
// ** TUNING:  For every WHERE clause term that is not used by the index
// ** and which does not have an assigned truth probability, heuristics
// ** described below are used to try to estimate the truth probability.
// ** TODO --> Perhaps this is something that could be improved by better
// ** table statistics.
// **
// ** Heuristic 1:  Estimate the truth probability as 93.75%.  The 93.75%
// ** value corresponds to -1 in LogEst notation, so this means decrement
// ** the WhereLoop.nOut field for every such WHERE clause term.
// **
// ** Heuristic 2:  If there exists one or more WHERE clause terms of the
// ** form "x==EXPR" and EXPR is not a constant 0 or 1, then make sure the
// ** final output row estimate is no greater than 1/4 of the total number
// ** of rows in the table.  In other words, assume that x==EXPR will filter
// ** out at least 3 out of 4 rows.  If EXPR is -1 or 0 or 1, then maybe the
// ** "x" column is boolean or else -1 or 0 or 1 is a common default value
// ** on the "x" column and so in that case only cap the output row estimate
// ** at 1/2 instead of 1/4.
// **
// ** Heuristic 3:  If there is a LIKE or GLOB (or REGEXP or MATCH) operator
// ** with a large constant pattern, then reduce the size of the search
// ** space according to the length of the pattern, under the theory that
// ** longer patterns are less likely to match.  This heuristic was added
// ** to give better output-row count estimates when preparing queries for
// ** the Join-Order Benchmarks.  See forum thread 2026-01-30T09:57:54z
// */
fn whereLoopOutputAdjust(mut pWC: *mut WhereClause, mut pLoop: *mut WhereLoop, mut nRow: i16) {
    let mut pTerm: *mut WhereTerm = unsafe { std::mem::zeroed() };
    let mut pX: *mut WhereTerm = unsafe { std::mem::zeroed() };
    let mut notAllowed: u64 = !((unsafe { (*pLoop).prereq }) | unsafe { (*pLoop).maskSelf });
    let mut i: i32 = 0 as i32;
    let mut j: i32 = 0 as i32;
    // /* pLoop->nOut should not exceed nRow-iReduce */
    let mut iReduce: i16 = (0 as i32) as i16;
    // /* Skip all this if the FROM clause of the query is a single table and
    //   ** there is no ORDER BY. In this case it doesn't matter how accurate
    //   ** the WhereLoop.nOut values are.  */
    if (unsafe { (*unsafe { (*unsafe { (*pWC).pWInfo }).pTabList }).nSrc }) <= (1 as i32)
        && (unsafe { (*unsafe { (*pWC).pWInfo }).pOrderBy }) == std::ptr::null_mut::<ExprList>()
    {
        return;
    }
    0 as i32;
    i = unsafe { (*pWC).nBase };
    let __v2138: *mut WhereTerm = unsafe { (*pWC).a };
    pTerm = __v2138;
    '__slate_break_1767: while i > (0 as i32) {
        0 as i32;
        if (unsafe { (*pTerm).prereqAll }) & notAllowed != (((0 as i32) as i64) as u64) {
        } else {
            if (unsafe { (*pTerm).prereqAll }) & unsafe { (*pLoop).maskSelf }
                == (((0 as i32) as i64) as u64)
            {
            } else {
                if (((unsafe { (*pTerm).wtFlags }) as u32) as i32) & (2 as i32) != (0 as i32) {
                } else {
                    j = (((unsafe { (*pLoop).nLTerm }) as u32) as i32) - (1 as i32);
                    '__slate_break_1768: loop {
                        if !(j >= (0 as i32)) {
                            break;
                        }
                        pX = unsafe { *unsafe { unsafe { (*pLoop).aLTerm }.offset(j as isize) } };
                        if pX == std::ptr::null_mut::<WhereTerm>() {
                        } else {
                            if pX == pTerm {
                                break '__slate_break_1768;
                            }
                            if (unsafe { (*pX).iParent }) >= (0 as i32)
                                && (unsafe {
                                    unsafe { (*pWC).a }.offset((unsafe { (*pX).iParent }) as isize)
                                }) == pTerm
                            {
                                break '__slate_break_1768;
                            }
                        }
                        let __v2143: i32 = j;
                        let __v2144: i32 = __v2143 - (1 as i32);
                        j = __v2144;
                    }
                    if j < (0 as i32) {
                        unsafe {
                            sqlite3ProgressCheck(unsafe { (*unsafe { (*pWC).pWInfo }).pParse })
                        };
                        if (unsafe { (*pLoop).maskSelf }) == unsafe { (*pTerm).prereqAll } {
                            // /* If there are extra terms in the WHERE clause not used by an index
                            //         ** that depend only on the table being scanned, and that will tend to
                            //         ** cause many rows to be omitted, then mark that table as
                            //         ** "self-culling".
                            //         **
                            //         ** 2022-03-24:  Self-culling only applies if either the extra terms
                            //         ** are straight comparison operators that are non-true with NULL
                            //         ** operand, or if the loop is not an OUTER JOIN.
                            //         */
                            if (((unsafe { (*pTerm).eOperator }) as u32) as i32) & (63 as i32)
                                != (0 as i32)
                                || (((unsafe {
                                    (*unsafe {
                                        unsafe {
                                            std::ptr::addr_of_mut!(
                                                (*unsafe { (*unsafe { (*pWC).pWInfo }).pTabList })
                                                    .a
                                            )
                                                as *mut SrcItem
                                        }
                                        .offset(
                                            (((unsafe { (*pLoop).iTab }) as u32) as i32) as isize,
                                        )
                                    })
                                    .fg
                                    .jointype
                                }) as u32) as i32)
                                    & ((8 as i32) | (64 as i32))
                                    == (0 as i32)
                            {
                                let __v2145: *mut WhereLoop = pLoop;
                                let __v2146: u32 = unsafe { (*__v2145).wsFlags };
                                let __v2147: u32 = __v2146 | ((8388608 as i32) as u32);
                                unsafe {
                                    (*__v2145).wsFlags = __v2147;
                                }
                            }
                        }
                        if ((unsafe { (*pTerm).truthProb }) as i32) <= (0 as i32) {
                            // /* If a truth probability is specified using the likelihood() hints,
                            //         ** then use the probability provided by the application. */
                            let __v2148: *mut WhereLoop = pLoop;
                            let __v2149: i16 = unsafe { (*__v2148).nOut };
                            let __v2150: i16 = ((__v2149 as i32)
                                + ((unsafe { (*pTerm).truthProb }) as i32))
                                as i16;
                            unsafe {
                                (*__v2148).nOut = __v2150;
                            }
                        } else {
                            // /* In the absence of explicit truth probabilities, use heuristics to
                            //         ** guess a reasonable truth probability. */
                            let mut pOpExpr: *mut Expr = unsafe { (*pTerm).pExpr };
                            let __v2151: *mut WhereLoop = pLoop;
                            let __v2152: i16 = unsafe { (*__v2151).nOut };
                            let __v2153: i16 = ((__v2152 as i32) - (1 as i32)) as i16;
                            unsafe {
                                (*__v2151).nOut = __v2153;
                            }
                            if (((unsafe { (*pTerm).eOperator }) as u32) as i32)
                                & ((2 as i32) | (128 as i32))
                                != (0 as i32)
                                && (((unsafe { (*pTerm).wtFlags }) as u32) as i32) & (0 as i32)
                                    == (0 as i32)
                            {
                                let mut pRight: *mut Expr = unsafe { (*pOpExpr).pRight };
                                let mut pParse: *mut Parse =
                                    unsafe { (*unsafe { (*pWC).pWInfo }).pParse };
                                let mut k: i32 = 0 as i32;
                                {}
                                if (unsafe {
                                    sqlite3ExprIsInteger(
                                        pRight as *const Expr,
                                        std::ptr::addr_of_mut!(k),
                                        pParse,
                                        1 as i32,
                                    )
                                }) != (0 as i32)
                                    && k >= -(1 as i32)
                                    && k <= (1 as i32)
                                {
                                    k = 10 as i32;
                                } else {
                                    k = 20 as i32;
                                }
                                if (iReduce as i32) < k {
                                    let __v2154: *mut WhereTerm = pTerm;
                                    let __v2155: u16 = unsafe { (*__v2154).wtFlags };
                                    let __v2156: u16 =
                                        ((((__v2155 as u32) as i32) | (8192 as i32)) as i16) as u16;
                                    unsafe {
                                        (*__v2154).wtFlags = __v2156;
                                    }
                                    iReduce = k as i16;
                                }
                            } else {
                                if (unsafe { (*pOpExpr).flags }) & ((256 as i32) as u32)
                                    != ((0 as i32) as u32)
                                    && (((unsafe { (*pOpExpr).op }) as u32) as i32) == (172 as i32)
                                {
                                    let mut eOp: i32 = 0 as i32;
                                    0 as i32;
                                    0 as i32;
                                    eOp = unsafe {
                                        sqlite3ExprIsLikeOperator(pOpExpr as *const Expr)
                                    };
                                    if eOp > (0 as i32) {
                                        let mut szPattern: i32 = 0 as i32;
                                        let mut pRHS: *mut Expr = unsafe {
                                            (*unsafe {
                                                unsafe {
                                                    std::ptr::addr_of_mut!(
                                                        (*unsafe { (*pOpExpr).x.pList }).a
                                                    )
                                                        as *mut ExprList_item
                                                }
                                                .offset((0 as i32) as isize)
                                            })
                                            .pExpr
                                        };
                                        eOp = (eOp == (65 as i32)) as i32;
                                        szPattern = estLikePatternLength(pRHS, (eOp as i16) as u16);
                                        if szPattern > (0 as i32) {
                                            let __v2157: *mut WhereLoop = pLoop;
                                            let __v2158: i16 = unsafe { (*__v2157).nOut };
                                            let __v2159: i16 =
                                                ((__v2158 as i32) - szPattern * (2 as i32)) as i16;
                                            unsafe {
                                                (*__v2157).nOut = __v2159;
                                            }
                                        }
                                    }
                                }
                            }
                            // /* tag-20200224-1 */
                        }
                    }
                }
            }
        }
        let __v2139: i32 = i;
        let __v2140: i32 = __v2139 - (1 as i32);
        i = __v2140;
        let __v2141: *mut WhereTerm = pTerm;
        let __v2142: *mut WhereTerm = unsafe { __v2141.offset((1 as i32) as isize) };
        pTerm = __v2142;
    }
    if ((unsafe { (*pLoop).nOut }) as i32) > (nRow as i32) - (iReduce as i32) {
        unsafe {
            (*pLoop).nOut = ((nRow as i32) - (iReduce as i32)) as i16;
        }
    }
}

// /* The WHERE clause */
// /* The loop to adjust downward */
// /* Number of rows in the entire table */
// /*
// ** Term pTerm is a vector range comparison operation. The first comparison
// ** in the vector can be optimized using column nEq of the index. This
// ** function returns the total number of vector elements that can be used
// ** as part of the range comparison.
// **
// ** For example, if the query is:
// **
// **   WHERE a = ? AND (b, c, d) > (?, ?, ?)
// **
// ** and the index:
// **
// **   CREATE INDEX ... ON (a, b, c, d, e)
// **
// ** then this function would be invoked with nEq=1. The value returned in
// ** this case is 3.
// */
fn whereRangeVectorLen(
    mut pParse: *mut Parse,
    mut iCur: i32,
    mut pIdx: *mut Index,
    mut nEq: i32,
    mut pTerm: *mut WhereTerm,
) -> i32 {
    let mut nCmp: i32 = unsafe {
        sqlite3ExprVectorSize((unsafe { (*unsafe { (*pTerm).pExpr }).pLeft }) as *const Expr)
    };
    let mut i: i32 = 0 as i32;
    nCmp = if nCmp < (((unsafe { (*pIdx).nColumn }) as u32) as i32) - nEq {
        nCmp
    } else {
        (((unsafe { (*pIdx).nColumn }) as u32) as i32) - nEq
    };
    i = 1 as i32;
    '__slate_break_1769: loop {
        if !(i < nCmp) {
            break;
        }
        // /* Test if comparison i of pTerm is compatible with column (i+nEq)
        //     ** of the index. If not, exit the loop.  */
        // /* Comparison affinity */
        let mut aff: i8 = 0 as i8;
        // /* Indexed columns affinity */
        let mut idxaff: i8 = (0 as i32) as i8;
        // /* Comparison collation sequence */
        let mut pColl: *mut CollSeq = unsafe { std::mem::zeroed() };
        let mut pLhs: *mut Expr = unsafe { std::mem::zeroed() };
        let mut pRhs: *mut Expr = unsafe { std::mem::zeroed() };
        0 as i32;
        pLhs = unsafe {
            (*unsafe {
                unsafe {
                    std::ptr::addr_of_mut!(
                        (*unsafe { (*unsafe { (*unsafe { (*pTerm).pExpr }).pLeft }).x.pList }).a
                    ) as *mut ExprList_item
                }
                .offset(i as isize)
            })
            .pExpr
        };
        pRhs = unsafe { (*unsafe { (*pTerm).pExpr }).pRight };
        if (unsafe { (*pRhs).flags }) & ((4096 as i32) as u32) != ((0 as i32) as u32) {
            pRhs = unsafe {
                (*unsafe {
                    unsafe {
                        std::ptr::addr_of_mut!(
                            (*unsafe { (*unsafe { (*pRhs).x.pSelect }).pEList }).a
                        ) as *mut ExprList_item
                    }
                    .offset(i as isize)
                })
                .pExpr
            };
        } else {
            pRhs = unsafe {
                (*unsafe {
                    unsafe {
                        std::ptr::addr_of_mut!((*unsafe { (*pRhs).x.pList }).a)
                            as *mut ExprList_item
                    }
                    .offset(i as isize)
                })
                .pExpr
            };
        }
        // /* Check that the LHS of the comparison is a column reference to
        //     ** the right column of the right source table. And that the sort
        //     ** order of the index column is the same as the sort order of the
        //     ** leftmost index column.  */
        if (((unsafe { (*pLhs).op }) as u32) as i32) != (168 as i32)
            || (unsafe { (*pLhs).iTable }) != iCur
            || ((unsafe { (*pLhs).iColumn }) as i32)
                != ((unsafe { *unsafe { unsafe { (*pIdx).aiColumn }.offset((i + nEq) as isize) } })
                    as i32)
            || (((unsafe { *unsafe { unsafe { (*pIdx).aSortOrder }.offset((i + nEq) as isize) } })
                as u32) as i32)
                != (((unsafe { *unsafe { unsafe { (*pIdx).aSortOrder }.offset(nEq as isize) } })
                    as u32) as i32)
        {
            break '__slate_break_1769;
        }
        {}
        aff = unsafe {
            sqlite3CompareAffinity(pRhs as *const Expr, unsafe {
                sqlite3ExprAffinity(pLhs as *const Expr)
            })
        };
        idxaff = unsafe {
            sqlite3TableColumnAffinity(
                (unsafe { (*pIdx).pTable }) as *const Table,
                (unsafe { (*pLhs).iColumn }) as i32,
            )
        };
        if (aff as i32) != (idxaff as i32) {
            break '__slate_break_1769;
        }
        if (unsafe { (*unsafe { (*pTerm).pExpr }).flags }) & ((1024 as i32) as u32)
            != ((0 as i32) as u32)
        {
            let mut t: *mut Expr = pRhs;
            pRhs = pLhs;
            pLhs = t;
        }
        {}
        pColl = unsafe {
            sqlite3BinaryCompareCollSeq(pParse, pLhs as *const Expr, pRhs as *const Expr)
        };
        if pColl == std::ptr::null_mut::<CollSeq>() {
            break '__slate_break_1769;
        }
        if (unsafe {
            sqlite3StrICmp((unsafe { (*pColl).zName }) as *const i8, unsafe {
                *unsafe { unsafe { (*pIdx).azColl }.offset((i + nEq) as isize) }
            })
        }) != (0 as i32)
        {
            break '__slate_break_1769;
        }
        let __v2160: i32 = i;
        let __v2161: i32 = __v2160 + (1 as i32);
        i = __v2161;
    }
    return i;
}

// /* Parsing context */
// /* Cursor open on pIdx */
// /* The index to be used for a inequality constraint */
// /* Number of prior equality constraints on same index */
// /* The vector inequality constraint */
// /*
// ** Adjust the cost C by the costMult factor T.  This only occurs if
// ** compiled with -DSQLITE_ENABLE_COSTMULT
// */
// /*
// ** We have so far matched pBuilder->pNew->u.btree.nEq terms of the
// ** index pIndex. Try to match one more.
// **
// ** When this function is called, pBuilder->pNew->nOut contains the
// ** number of rows expected to be visited by filtering using the nEq
// ** terms only. If it is modified, this value is restored before this
// ** function returns.
// **
// ** If pProbe->idxType==SQLITE_IDXTYPE_IPK, that means pIndex is
// ** a fake index used for the INTEGER PRIMARY KEY.
// */
fn whereLoopAddBtreeIndex(
    mut pBuilder: *mut WhereLoopBuilder,
    mut pSrc: *mut SrcItem,
    mut pProbe: *mut Index,
    mut nInMul: i16,
) -> i32 {
    // /* WHERE analyze context */
    let mut pWInfo: *mut WhereInfo = unsafe { (*pBuilder).pWInfo };
    // /* Parsing context */
    let mut pParse: *mut Parse = unsafe { (*pWInfo).pParse };
    // /* Database connection malloc context */
    let mut db: *mut sqlite3 = unsafe { (*pParse).db };
    // /* Template WhereLoop under construction */
    let mut pNew: *mut WhereLoop = unsafe { std::mem::zeroed() };
    // /* A WhereTerm under consideration */
    let mut pTerm: *mut WhereTerm = unsafe { std::mem::zeroed() };
    // /* Valid operators for constraints */
    let mut opMask: i32 = 0 as i32;
    // /* Iterator for WHERE terms */
    let mut scan: WhereScan = unsafe { std::mem::zeroed() };
    // /* Original value of pNew->prereq */
    let mut saved_prereq: u64 = 0 as u64;
    // /* Original value of pNew->nLTerm */
    let mut saved_nLTerm: u16 = 0 as u16;
    // /* Original value of pNew->u.btree.nEq */
    let mut saved_nEq: u16 = 0 as u16;
    // /* Original value of pNew->u.btree.nBtm */
    let mut saved_nBtm: u16 = 0 as u16;
    // /* Original value of pNew->u.btree.nTop */
    let mut saved_nTop: u16 = 0 as u16;
    // /* Original value of pNew->nSkip */
    let mut saved_nSkip: u16 = 0 as u16;
    // /* Original value of pNew->wsFlags */
    let mut saved_wsFlags: u32 = 0 as u32;
    // /* Original value of pNew->nOut */
    let mut saved_nOut: i16 = 0 as i16;
    // /* Return code */
    let mut rc: i32 = 0 as i32;
    // /* Number of rows in the table */
    let mut rSize: i16 = 0 as i16;
    // /* Logarithm of table size */
    let mut rLogSize: i16 = 0 as i16;
    // /* Top and bottom range constraints */
    let mut pTop: *mut WhereTerm = std::ptr::null_mut::<WhereTerm>();
    let mut pBtm: *mut WhereTerm = std::ptr::null_mut::<WhereTerm>();
    pNew = unsafe { (*pBuilder).pNew };
    0 as i32;
    if (unsafe { (*pParse).nErr }) != (0 as i32) {
        return unsafe { (*pParse).rc };
    }
    {}
    0 as i32;
    0 as i32;
    if (unsafe { (*pNew).wsFlags }) & ((32 as i32) as u32) != (0 as u32) {
        opMask = (2 as i32) << (57 as i32) - (54 as i32) | (2 as i32) << (56 as i32) - (54 as i32);
    } else {
        0 as i32;
        opMask = (2 as i32)
            | (1 as i32)
            | (2 as i32) << (55 as i32) - (54 as i32)
            | (2 as i32) << (58 as i32) - (54 as i32)
            | (2 as i32) << (57 as i32) - (54 as i32)
            | (2 as i32) << (56 as i32) - (54 as i32)
            | (256 as i32)
            | (128 as i32);
    }
    if ((unsafe { (*pProbe).__slate_bits_0.__get_bUnordered() }) as i32) != (0 as i32) {
        let __v2162: i32 = opMask;
        let __v2163: i32 = __v2162
            & !((2 as i32) << (55 as i32) - (54 as i32)
                | (2 as i32) << (58 as i32) - (54 as i32)
                | (2 as i32) << (57 as i32) - (54 as i32)
                | (2 as i32) << (56 as i32) - (54 as i32));
        opMask = __v2163;
    }
    0 as i32;
    0 as i32;
    saved_nEq = unsafe { (*pNew).u.btree.nEq };
    saved_nBtm = unsafe { (*pNew).u.btree.nBtm };
    saved_nTop = unsafe { (*pNew).u.btree.nTop };
    saved_nSkip = unsafe { (*pNew).nSkip };
    saved_nLTerm = unsafe { (*pNew).nLTerm };
    saved_wsFlags = unsafe { (*pNew).wsFlags };
    saved_prereq = unsafe { (*pNew).prereq };
    saved_nOut = unsafe { (*pNew).nOut };
    pTerm = whereScanInit(
        std::ptr::addr_of_mut!(scan),
        unsafe { (*pBuilder).pWC },
        unsafe { (*pSrc).iCursor },
        (saved_nEq as u32) as i32,
        opMask as u32,
        pProbe,
    );
    unsafe {
        (*pNew).rSetup = (0 as i32) as i16;
    }
    rSize = unsafe { *unsafe { unsafe { (*pProbe).aiRowLogEst }.offset((0 as i32) as isize) } };
    rLogSize = estLog(rSize);
    '__slate_break_1770: while rc == (0 as i32) && pTerm != std::ptr::null_mut::<WhereTerm>() {
        '__slate_continue_1770: {
            // /* Shorthand for pTerm->eOperator */
            let mut eOp: u16 = unsafe { (*pTerm).eOperator };
            let mut rCostIdx: i16 = 0 as i16;
            // /* nOut before IN() and WHERE adjustments */
            let mut nOutUnadjusted: i16 = 0 as i16;
            let mut nIn: i32 = 0 as i32;
            let __v2164: bool;
            if ((eOp as u32) as i32) == (256 as i32)
                || (((unsafe { (*pTerm).wtFlags }) as u32) as i32) & (128 as i32) != (0 as i32)
            {
                __v2164 = indexColumnNotNull(pProbe, (saved_nEq as u32) as i32) != (0 as i32);
            } else {
                __v2164 = false as bool;
            }
            if __v2164 {
                // /* ignore IS [NOT] NULL constraints on NOT NULL columns */
            } else {
                if (unsafe { (*pTerm).prereqRight }) & unsafe { (*pNew).maskSelf } != (0 as u64) {
                } else {
                    // /* Do not allow the upper bound of a LIKE optimization range constraint
                    //     ** to mix with a lower range bound from some other source */
                    if (((unsafe { (*pTerm).wtFlags }) as u32) as i32) & (256 as i32) != (0 as i32)
                        && (((unsafe { (*pTerm).eOperator }) as u32) as i32)
                            == (2 as i32) << (57 as i32) - (54 as i32)
                    {
                    } else {
                        let __v2165: bool;
                        if (((unsafe { (*pSrc).fg.jointype }) as u32) as i32)
                            & ((8 as i32) | (64 as i32) | (16 as i32))
                            != (0 as i32)
                        {
                            __v2165 = !(constraintCompatibleWithOuterJoin(
                                pTerm as *const WhereTerm,
                                pSrc as *const SrcItem,
                            ) != (0 as i32));
                        } else {
                            __v2165 = false as bool;
                        }
                        if __v2165 {
                        } else {
                            if (((unsafe { (*pProbe).onError }) as u32) as i32) != (0 as i32)
                                && ((saved_nEq as u32) as i32)
                                    == (((unsafe { (*pProbe).nKeyCol }) as u32) as i32) - (1 as i32)
                            {
                                let __v2166: *mut WhereLoopBuilder = pBuilder;
                                let __v2167: u8 = unsafe { (*__v2166).bldFlags1 };
                                let __v2168: u8 =
                                    ((((__v2167 as u32) as i32) | (2 as i32)) as i8) as u8;
                                unsafe {
                                    (*__v2166).bldFlags1 = __v2168;
                                }
                            } else {
                                let __v2169: *mut WhereLoopBuilder = pBuilder;
                                let __v2170: u8 = unsafe { (*__v2169).bldFlags1 };
                                let __v2171: u8 =
                                    ((((__v2170 as u32) as i32) | (1 as i32)) as i8) as u8;
                                unsafe {
                                    (*__v2169).bldFlags1 = __v2171;
                                }
                            }
                            unsafe {
                                (*pNew).wsFlags = saved_wsFlags;
                            }
                            unsafe {
                                (*pNew).u.btree.nEq = saved_nEq;
                            }
                            unsafe {
                                (*pNew).u.btree.nBtm = saved_nBtm;
                            }
                            unsafe {
                                (*pNew).u.btree.nTop = saved_nTop;
                            }
                            unsafe {
                                (*pNew).nLTerm = saved_nLTerm;
                            }
                            let __v2172: bool;
                            if (((unsafe { (*pNew).nLTerm }) as u32) as i32)
                                >= (((unsafe { (*pNew).nLSlot }) as u32) as i32)
                            {
                                __v2172 = whereLoopResize(
                                    db,
                                    pNew,
                                    (((unsafe { (*pNew).nLTerm }) as u32) as i32) + (1 as i32),
                                ) != (0 as i32);
                            } else {
                                __v2172 = false as bool;
                            }
                            if __v2172 {
                                // /* OOM while trying to enlarge the pNew->aLTerm array */
                                break '__slate_break_1770;
                            }
                            let __v2173: *mut WhereLoop = pNew;
                            let __v2174: u16 = unsafe { (*__v2173).nLTerm };
                            let __v2175: u16 =
                                ((((__v2174 as u32) as i32) + (1 as i32)) as i16) as u16;
                            unsafe {
                                (*__v2173).nLTerm = __v2175;
                            }
                            unsafe {
                                *unsafe {
                                    unsafe { (*pNew).aLTerm }
                                        .offset(((__v2174 as u32) as i32) as isize)
                                } = pTerm;
                            }
                            unsafe {
                                (*pNew).prereq = (saved_prereq | unsafe { (*pTerm).prereqRight })
                                    & !unsafe { (*pNew).maskSelf };
                            }
                            0 as i32;
                            if ((eOp as u32) as i32) & (1 as i32) != (0 as i32) {
                                let mut pExpr: *mut Expr = unsafe { (*pTerm).pExpr };
                                if (unsafe { (*pExpr).flags }) & ((4096 as i32) as u32)
                                    != ((0 as i32) as u32)
                                {
                                    // /* "x IN (SELECT ...)":  TUNING: the SELECT returns 25 rows */
                                    let mut i: i32 = 0 as i32;
                                    let mut bRedundant: i32 = 0 as i32;
                                    nIn = 46 as i32;
                                    0 as i32;
                                    // /* The expression may actually be of the form (x, y) IN (SELECT...).
                                    //         ** In this case there is a separate term for each of (x) and (y).
                                    //         ** However, the nIn multiplier should only be applied once, not once
                                    //         ** for each such term. The following loop checks that pTerm is the
                                    //         ** first such term in use, and sets nIn back to 0 if it is not. */
                                    i = 0 as i32;
                                    '__slate_break_1771: loop {
                                        if !(i
                                            < (((unsafe { (*pNew).nLTerm }) as u32) as i32)
                                                - (1 as i32))
                                        {
                                            break;
                                        }
                                        if (unsafe {
                                            *unsafe { unsafe { (*pNew).aLTerm }.offset(i as isize) }
                                        }) != std::ptr::null_mut::<WhereTerm>()
                                            && (unsafe {
                                                (*unsafe {
                                                    *unsafe {
                                                        unsafe { (*pNew).aLTerm }.offset(i as isize)
                                                    }
                                                })
                                                .pExpr
                                            }) == pExpr
                                        {
                                            nIn = 0 as i32;
                                            if (unsafe {
                                                (*unsafe {
                                                    *unsafe {
                                                        unsafe { (*pNew).aLTerm }.offset(i as isize)
                                                    }
                                                })
                                                .u
                                                .x
                                                .iField
                                            }) == unsafe { (*pTerm).u.x.iField }
                                            {
                                                // /* Detect when two or more columns of an index match the same
                                                //               ** column of a vector IN operater, and avoid adding the column
                                                //               ** to the WhereLoop more than once.  See tag-20250707-01
                                                //               ** in test/rowvalue.test */
                                                bRedundant = 1 as i32;
                                            }
                                        }
                                        let __v2176: i32 = i;
                                        let __v2177: i32 = __v2176 + (1 as i32);
                                        i = __v2177;
                                    }
                                    if bRedundant != (0 as i32) {
                                        let __v2178: *mut WhereLoop = pNew;
                                        let __v2179: u16 = unsafe { (*__v2178).nLTerm };
                                        let __v2180: u16 = ((((__v2179 as u32) as i32) - (1 as i32))
                                            as i16)
                                            as u16;
                                        unsafe {
                                            (*__v2178).nLTerm = __v2180;
                                        }
                                        break '__slate_continue_1770;
                                    }
                                } else {
                                    if (unsafe { (*pExpr).x.pList })
                                        != std::ptr::null_mut::<ExprList>()
                                        && (unsafe { (*unsafe { (*pExpr).x.pList }).nExpr })
                                            != (0 as i32)
                                    {
                                        // /* "x IN (value, value, ...)" */
                                        nIn = (unsafe {
                                            sqlite3LogEst(
                                                ((unsafe { (*unsafe { (*pExpr).x.pList }).nExpr })
                                                    as i64)
                                                    as u64,
                                            )
                                        }) as i32;
                                    }
                                }
                                if ((unsafe { (*pProbe).__slate_bits_0.__get_hasStat1() }) as i32)
                                    != (0 as i32)
                                    && (rLogSize as i32) >= (10 as i32)
                                {
                                    let mut M: i16 = 0 as i16;
                                    let mut logK: i16 = 0 as i16;
                                    let mut x: i16 = 0 as i16;
                                    // /* Let:
                                    //         **   N = the total number of rows in the table
                                    //         **   K = the number of entries on the RHS of the IN operator
                                    //         **   M = the number of rows in the table that match terms to the
                                    //         **       to the left in the same index.  If the IN operator is on
                                    //         **       the left-most index column, M==N.
                                    //         **
                                    //         ** Given the definitions above, it is better to omit the IN operator
                                    //         ** from the index lookup and instead do a scan of the M elements,
                                    //         ** testing each scanned row against the IN operator separately, if:
                                    //         **
                                    //         **        M*log(K) < K*log(N)
                                    //         **
                                    //         ** Our estimates for M, K, and N might be inaccurate, so we build in
                                    //         ** a safety margin of 2 (LogEst: 10) that favors using the IN operator
                                    //         ** with the index, as using an index has better worst-case behavior.
                                    //         ** If we do not have real sqlite_stat1 data, always prefer to use
                                    //         ** the index.  Do not bother with this optimization on very small
                                    //         ** tables (less than 2 rows) as it is pointless in that case.
                                    //         */
                                    M = unsafe {
                                        *unsafe {
                                            unsafe { (*pProbe).aiRowLogEst }
                                                .offset(((saved_nEq as u32) as i32) as isize)
                                        }
                                    };
                                    logK = estLog(nIn as i16);
                                    // /* TUNING      v-----  10 to bias toward indexed IN */
                                    x = ((M as i32) + (logK as i32) + (10 as i32)
                                        - (nIn + (rLogSize as i32)))
                                        as i16;
                                    if (x as i32) >= (0 as i32) {
                                        {}
                                    } else {
                                        if (nInMul as i32) < (2 as i32)
                                            && (unsafe { (*db).dbOptFlags })
                                                & ((131072 as i32) as u32)
                                                == ((0 as i32) as u32)
                                        {
                                            {}
                                            let __v2181: *mut WhereLoop = pNew;
                                            let __v2182: u32 = unsafe { (*__v2181).wsFlags };
                                            let __v2183: u32 = __v2182 | ((1048576 as i32) as u32);
                                            unsafe {
                                                (*__v2181).wsFlags = __v2183;
                                            }
                                        } else {
                                            {}
                                            break '__slate_continue_1770;
                                        }
                                    }
                                }
                                let __v2184: *mut WhereLoop = pNew;
                                let __v2185: u32 = unsafe { (*__v2184).wsFlags };
                                let __v2186: u32 = __v2185 | ((4 as i32) as u32);
                                unsafe {
                                    (*__v2184).wsFlags = __v2186;
                                }
                            } else {
                                if ((eOp as u32) as i32) & ((2 as i32) | (128 as i32)) != (0 as i32)
                                {
                                    let mut iCol: i32 = (unsafe {
                                        *unsafe {
                                            unsafe { (*pProbe).aiColumn }
                                                .offset(((saved_nEq as u32) as i32) as isize)
                                        }
                                    })
                                        as i32;
                                    let __v2187: *mut WhereLoop = pNew;
                                    let __v2188: u32 = unsafe { (*__v2187).wsFlags };
                                    let __v2189: u32 = __v2188 | ((1 as i32) as u32);
                                    unsafe {
                                        (*__v2187).wsFlags = __v2189;
                                    }
                                    0 as i32;
                                    if iCol == -(1 as i32)
                                        || iCol >= (0 as i32)
                                            && (nInMul as i32) == (0 as i32)
                                            && ((saved_nEq as u32) as i32)
                                                == (((unsafe { (*pProbe).nKeyCol }) as u32) as i32)
                                                    - (1 as i32)
                                    {
                                        if iCol == -(1 as i32)
                                            || ((unsafe {
                                                (*pProbe).__slate_bits_0.__get_uniqNotNull()
                                            })
                                                as i32)
                                                != (0 as i32)
                                            || (((unsafe { (*pProbe).nKeyCol }) as u32) as i32)
                                                == (1 as i32)
                                                && (unsafe { (*pProbe).onError }) != (0 as u8)
                                                && ((eOp as u32) as i32) & (2 as i32) != (0 as i32)
                                        {
                                            let __v2190: *mut WhereLoop = pNew;
                                            let __v2191: u32 = unsafe { (*__v2190).wsFlags };
                                            let __v2192: u32 = __v2191 | ((4096 as i32) as u32);
                                            unsafe {
                                                (*__v2190).wsFlags = __v2192;
                                            }
                                        } else {
                                            let __v2193: *mut WhereLoop = pNew;
                                            let __v2194: u32 = unsafe { (*__v2193).wsFlags };
                                            let __v2195: u32 = __v2194 | ((65536 as i32) as u32);
                                            unsafe {
                                                (*__v2193).wsFlags = __v2195;
                                            }
                                        }
                                    }
                                    if ((scan.iEquiv as u32) as i32) > (1 as i32) {
                                        let __v2196: *mut WhereLoop = pNew;
                                        let __v2197: u32 = unsafe { (*__v2196).wsFlags };
                                        let __v2198: u32 = __v2197 | ((2097152 as i32) as u32);
                                        unsafe {
                                            (*__v2196).wsFlags = __v2198;
                                        }
                                    }
                                } else {
                                    if ((eOp as u32) as i32) & (256 as i32) != (0 as i32) {
                                        let __v2199: *mut WhereLoop = pNew;
                                        let __v2200: u32 = unsafe { (*__v2199).wsFlags };
                                        let __v2201: u32 = __v2200 | ((8 as i32) as u32);
                                        unsafe {
                                            (*__v2199).wsFlags = __v2201;
                                        }
                                    } else {
                                        let mut nVecLen: i32 = whereRangeVectorLen(
                                            pParse,
                                            unsafe { (*pSrc).iCursor },
                                            pProbe,
                                            (saved_nEq as u32) as i32,
                                            pTerm,
                                        );
                                        if ((eOp as u32) as i32)
                                            & ((2 as i32) << (55 as i32) - (54 as i32)
                                                | (2 as i32) << (58 as i32) - (54 as i32))
                                            != (0 as i32)
                                        {
                                            {}
                                            {}
                                            let __v2202: *mut WhereLoop = pNew;
                                            let __v2203: u32 = unsafe { (*__v2202).wsFlags };
                                            let __v2204: u32 =
                                                __v2203 | (((2 as i32) | (32 as i32)) as u32);
                                            unsafe {
                                                (*__v2202).wsFlags = __v2204;
                                            }
                                            unsafe {
                                                (*pNew).u.btree.nBtm = (nVecLen as i16) as u16;
                                            }
                                            pBtm = pTerm;
                                            pTop = std::ptr::null_mut::<WhereTerm>();
                                            if (((unsafe { (*pTerm).wtFlags }) as u32) as i32)
                                                & (256 as i32)
                                                != (0 as i32)
                                            {
                                                // /* Range constraints that come from the LIKE optimization are
                                                //           ** always used in pairs. */
                                                pTop = unsafe { pTerm.offset((1 as i32) as isize) };
                                                0 as i32;
                                                0 as i32;
                                                0 as i32;
                                                // /* OOM */
                                                if whereLoopResize(
                                                    db,
                                                    pNew,
                                                    (((unsafe { (*pNew).nLTerm }) as u32) as i32)
                                                        + (1 as i32),
                                                ) != (0 as i32)
                                                {
                                                    break '__slate_break_1770;
                                                }
                                                let __v2205: *mut WhereLoop = pNew;
                                                let __v2206: u16 = unsafe { (*__v2205).nLTerm };
                                                let __v2207: u16 = ((((__v2206 as u32) as i32)
                                                    + (1 as i32))
                                                    as i16)
                                                    as u16;
                                                unsafe {
                                                    (*__v2205).nLTerm = __v2207;
                                                }
                                                unsafe {
                                                    *unsafe {
                                                        unsafe { (*pNew).aLTerm }.offset(
                                                            ((__v2206 as u32) as i32) as isize,
                                                        )
                                                    } = pTop;
                                                }
                                                let __v2208: *mut WhereLoop = pNew;
                                                let __v2209: u32 = unsafe { (*__v2208).wsFlags };
                                                let __v2210: u32 = __v2209 | ((16 as i32) as u32);
                                                unsafe {
                                                    (*__v2208).wsFlags = __v2210;
                                                }
                                                unsafe {
                                                    (*pNew).u.btree.nTop =
                                                        ((1 as i32) as i16) as u16;
                                                }
                                            }
                                        } else {
                                            0 as i32;
                                            {}
                                            {}
                                            let __v2211: *mut WhereLoop = pNew;
                                            let __v2212: u32 = unsafe { (*__v2211).wsFlags };
                                            let __v2213: u32 =
                                                __v2212 | (((2 as i32) | (16 as i32)) as u32);
                                            unsafe {
                                                (*__v2211).wsFlags = __v2213;
                                            }
                                            unsafe {
                                                (*pNew).u.btree.nTop = (nVecLen as i16) as u16;
                                            }
                                            pTop = pTerm;
                                            pBtm = if (unsafe { (*pNew).wsFlags })
                                                & ((32 as i32) as u32)
                                                != ((0 as i32) as u32)
                                            {
                                                unsafe {
                                                    *unsafe {
                                                        unsafe { (*pNew).aLTerm }.offset(
                                                            ((((unsafe { (*pNew).nLTerm }) as u32)
                                                                as i32)
                                                                - (2 as i32))
                                                                as isize,
                                                        )
                                                    }
                                                }
                                            } else {
                                                std::ptr::null_mut::<WhereTerm>()
                                            };
                                        }
                                    }
                                }
                            }
                            // /* At this point pNew->nOut is set to the number of rows expected to
                            //     ** be visited by the index scan before considering term pTerm, or the
                            //     ** values of nIn and nInMul. In other words, assuming that all
                            //     ** "x IN(...)" terms are replaced with "x = ?". This block updates
                            //     ** the value of pNew->nOut to account for pTerm (but not nIn/nInMul).  */
                            0 as i32;
                            if (unsafe { (*pNew).wsFlags }) & ((2 as i32) as u32) != (0 as u32) {
                                // /* Adjust nOut using stat4 data. Or, if there is no stat4
                                //       ** data, using some other estimate.  */
                                whereRangeScanEst(pParse, pBuilder, pBtm, pTop, pNew);
                            } else {
                                let mut nEq: i32 = 0 as i32;
                                let __v2214: *mut WhereLoop = pNew;
                                let __v2215: u16 = unsafe { (*__v2214).u.btree.nEq };
                                let __v2216: u16 =
                                    ((((__v2215 as u32) as i32) + (1 as i32)) as i16) as u16;
                                unsafe {
                                    (*__v2214).u.btree.nEq = __v2216;
                                }
                                nEq = (__v2216 as u32) as i32;
                                0 as i32;
                                0 as i32;
                                if ((unsafe { (*pTerm).truthProb }) as i32) <= (0 as i32)
                                    && ((unsafe {
                                        *unsafe {
                                            unsafe { (*pProbe).aiColumn }
                                                .offset(((saved_nEq as u32) as i32) as isize)
                                        }
                                    }) as i32)
                                        >= (0 as i32)
                                {
                                    0 as i32;
                                    {}
                                    let __v2217: *mut WhereLoop = pNew;
                                    let __v2218: i16 = unsafe { (*__v2217).nOut };
                                    let __v2219: i16 = ((__v2218 as i32)
                                        + ((unsafe { (*pTerm).truthProb }) as i32))
                                        as i16;
                                    unsafe {
                                        (*__v2217).nOut = __v2219;
                                    }
                                    let __v2220: *mut WhereLoop = pNew;
                                    let __v2221: i16 = unsafe { (*__v2220).nOut };
                                    let __v2222: i16 = ((__v2221 as i32) - nIn) as i16;
                                    unsafe {
                                        (*__v2220).nOut = __v2222;
                                    }
                                } else {
                                    let __v2223: *mut WhereLoop = pNew;
                                    let __v2224: i16 = unsafe { (*__v2223).nOut };
                                    let __v2225: i16 = ((__v2224 as i32)
                                        + (((unsafe {
                                            *unsafe {
                                                unsafe { (*pProbe).aiRowLogEst }
                                                    .offset(nEq as isize)
                                            }
                                        }) as i32)
                                            - ((unsafe {
                                                *unsafe {
                                                    unsafe { (*pProbe).aiRowLogEst }
                                                        .offset((nEq - (1 as i32)) as isize)
                                                }
                                            })
                                                as i32)))
                                        as i16;
                                    unsafe {
                                        (*__v2223).nOut = __v2225;
                                    }
                                    if ((eOp as u32) as i32) & (256 as i32) != (0 as i32) {
                                        // /* TUNING: If there is no likelihood() value, assume that a
                                        //             ** "col IS NULL" expression matches twice as many rows
                                        //             ** as (col=?). */
                                        let __v2226: *mut WhereLoop = pNew;
                                        let __v2227: i16 = unsafe { (*__v2226).nOut };
                                        let __v2228: i16 = ((__v2227 as i32) + (10 as i32)) as i16;
                                        unsafe {
                                            (*__v2226).nOut = __v2228;
                                        }
                                    }
                                }
                            }
                            // /* Set rCostIdx to the estimated cost of visiting selected rows in the
                            //     ** index.  The estimate is the sum of two values:
                            //     **   1.  The cost of doing one search-by-key to find the first matching
                            //     **       entry
                            //     **   2.  Stepping forward in the index pNew->nOut times to find all
                            //     **       additional matching entries.
                            //     */
                            0 as i32;
                            if ((unsafe { (*pProbe).__slate_bits_0.__get_idxType() }) as i32)
                                == (3 as i32)
                            {
                                // /* The pProbe->szIdxRow is low for an IPK table since the interior
                                //       ** pages are small.  Thus szIdxRow gives a good estimate of seek cost.
                                //       ** But the leaf pages are full-size, so pProbe->szIdxRow would badly
                                //       ** under-estimate the scanning cost. */
                                rCostIdx =
                                    (((unsafe { (*pNew).nOut }) as i32) + (16 as i32)) as i16;
                            } else {
                                rCostIdx = (((unsafe { (*pNew).nOut }) as i32)
                                    + (1 as i32)
                                    + (15 as i32) * ((unsafe { (*pProbe).szIdxRow }) as i32)
                                        / ((unsafe { (*unsafe { (*pSrc).pSTab }).szTabRow })
                                            as i32))
                                    as i16;
                            }
                            rCostIdx = unsafe { sqlite3LogEstAdd(rLogSize, rCostIdx) };
                            // /* Estimate the cost of running the loop.  If all data is coming
                            //     ** from the index, then this is just the cost of doing the index
                            //     ** lookup and scan.  But if some data is coming out of the main table,
                            //     ** we also have to add in the cost of doing pNew->nOut searches to
                            //     ** locate the row in the main table that corresponds to the index entry.
                            //     */
                            unsafe {
                                (*pNew).rRun = rCostIdx;
                            }
                            if (unsafe { (*pNew).wsFlags })
                                & (((64 as i32) | (256 as i32) | (67108864 as i32)) as u32)
                                == ((0 as i32) as u32)
                            {
                                unsafe {
                                    (*pNew).rRun = unsafe {
                                        sqlite3LogEstAdd(
                                            unsafe { (*pNew).rRun },
                                            (((unsafe { (*pNew).nOut }) as i32) + (16 as i32))
                                                as i16,
                                        )
                                    };
                                }
                            }
                            {}
                            nOutUnadjusted = unsafe { (*pNew).nOut };
                            let __v2229: *mut WhereLoop = pNew;
                            let __v2230: i16 = unsafe { (*__v2229).rRun };
                            let __v2231: i16 = ((__v2230 as i32) + ((nInMul as i32) + nIn)) as i16;
                            unsafe {
                                (*__v2229).rRun = __v2231;
                            }
                            let __v2232: *mut WhereLoop = pNew;
                            let __v2233: i16 = unsafe { (*__v2232).nOut };
                            let __v2234: i16 = ((__v2233 as i32) + ((nInMul as i32) + nIn)) as i16;
                            unsafe {
                                (*__v2232).nOut = __v2234;
                            }
                            whereLoopOutputAdjust(unsafe { (*pBuilder).pWC }, pNew, rSize);
                            if ((unsafe { (*pSrc).fg.__slate_bits_0.__get_fromExists() }) as i32)
                                != (0 as i32)
                            {
                                unsafe {
                                    (*pNew).nOut = (0 as i32) as i16;
                                }
                            }
                            rc = whereLoopInsert(pBuilder, pNew);
                            if (unsafe { (*pNew).wsFlags }) & ((2 as i32) as u32) != (0 as u32) {
                                unsafe {
                                    (*pNew).nOut = saved_nOut;
                                }
                            } else {
                                unsafe {
                                    (*pNew).nOut = nOutUnadjusted;
                                }
                            }
                            if (unsafe { (*pNew).wsFlags }) & ((16 as i32) as u32)
                                == ((0 as i32) as u32)
                                && (((unsafe { (*pNew).u.btree.nEq }) as u32) as i32)
                                    < (((unsafe { (*pProbe).nColumn }) as u32) as i32)
                                && ((((unsafe { (*pNew).u.btree.nEq }) as u32) as i32)
                                    < (((unsafe { (*pProbe).nKeyCol }) as u32) as i32)
                                    || ((unsafe { (*pProbe).__slate_bits_0.__get_idxType() })
                                        as i32)
                                        != (2 as i32))
                            {
                                if (((unsafe { (*pNew).u.btree.nEq }) as u32) as i32) > (3 as i32) {
                                    unsafe { sqlite3ProgressCheck(pParse) };
                                }
                                whereLoopAddBtreeIndex(
                                    pBuilder,
                                    pSrc,
                                    pProbe,
                                    ((nInMul as i32) + nIn) as i16,
                                );
                            }
                            unsafe {
                                (*pNew).nOut = saved_nOut;
                            }
                        }
                    }
                }
            }
        }
        pTerm = whereScanNext(std::ptr::addr_of_mut!(scan));
    }
    unsafe {
        (*pNew).prereq = saved_prereq;
    }
    unsafe {
        (*pNew).u.btree.nEq = saved_nEq;
    }
    unsafe {
        (*pNew).u.btree.nBtm = saved_nBtm;
    }
    unsafe {
        (*pNew).u.btree.nTop = saved_nTop;
    }
    unsafe {
        (*pNew).nSkip = saved_nSkip;
    }
    unsafe {
        (*pNew).wsFlags = saved_wsFlags;
    }
    unsafe {
        (*pNew).nOut = saved_nOut;
    }
    unsafe {
        (*pNew).nLTerm = saved_nLTerm;
    }
    // /* Consider using a skip-scan if there are no WHERE clause constraints
    //   ** available for the left-most terms of the index, and if the average
    //   ** number of repeats in the left-most terms is at least 18.
    //   **
    //   ** The magic number 18 is selected on the basis that scanning 17 rows
    //   ** is almost always quicker than an index seek (even though if the index
    //   ** contains fewer than 2^17 rows we assume otherwise in other parts of
    //   ** the code). And, even if it is not, it should not be too much slower.
    //   ** On the other hand, the extra seeks could end up being significantly
    //   ** more expensive.  */
    0 as i32;
    let __v2235: bool;
    if ((saved_nEq as u32) as i32) == ((saved_nSkip as u32) as i32)
        && ((saved_nEq as u32) as i32) + (1 as i32)
            < (((unsafe { (*pProbe).nKeyCol }) as u32) as i32)
        && ((saved_nEq as u32) as i32) == (((unsafe { (*pNew).nLTerm }) as u32) as i32)
        && ((unsafe { (*pProbe).__slate_bits_0.__get_noSkipScan() }) as i32) == (0 as i32)
        && ((unsafe { (*pProbe).__slate_bits_0.__get_hasStat1() }) as i32) != (0 as i32)
        && (unsafe { (*db).dbOptFlags }) & ((16384 as i32) as u32) == ((0 as i32) as u32)
        && ((unsafe {
            *unsafe {
                unsafe { (*pProbe).aiRowLogEst }
                    .offset((((saved_nEq as u32) as i32) + (1 as i32)) as isize)
            }
        }) as i32)
            >= (42 as i32)
        && ((unsafe { (*pSrc).fg.__slate_bits_0.__get_fromExists() }) as i32) == (0 as i32)
    {
        let __v2236: i32 = whereLoopResize(
            db,
            pNew,
            (((unsafe { (*pNew).nLTerm }) as u32) as i32) + (1 as i32),
        );
        rc = __v2236;
        __v2235 = __v2236 == (0 as i32);
    } else {
        __v2235 = false as bool;
    }
    if __v2235 {
        let mut nIter: i16 = 0 as i16;
        let __v2237: *mut WhereLoop = pNew;
        let __v2238: u16 = unsafe { (*__v2237).u.btree.nEq };
        let __v2239: u16 = ((((__v2238 as u32) as i32) + (1 as i32)) as i16) as u16;
        unsafe {
            (*__v2237).u.btree.nEq = __v2239;
        }
        let __v2240: *mut WhereLoop = pNew;
        let __v2241: u16 = unsafe { (*__v2240).nSkip };
        let __v2242: u16 = ((((__v2241 as u32) as i32) + (1 as i32)) as i16) as u16;
        unsafe {
            (*__v2240).nSkip = __v2242;
        }
        let __v2243: *mut WhereLoop = pNew;
        let __v2244: u16 = unsafe { (*__v2243).nLTerm };
        let __v2245: u16 = ((((__v2244 as u32) as i32) + (1 as i32)) as i16) as u16;
        unsafe {
            (*__v2243).nLTerm = __v2245;
        }
        unsafe {
            *unsafe { unsafe { (*pNew).aLTerm }.offset(((__v2244 as u32) as i32) as isize) } =
                std::ptr::null_mut::<WhereTerm>();
        }
        let __v2246: *mut WhereLoop = pNew;
        let __v2247: u32 = unsafe { (*__v2246).wsFlags };
        let __v2248: u32 = __v2247 | ((32768 as i32) as u32);
        unsafe {
            (*__v2246).wsFlags = __v2248;
        }
        nIter = (((unsafe {
            *unsafe {
                unsafe { (*pProbe).aiRowLogEst }.offset(((saved_nEq as u32) as i32) as isize)
            }
        }) as i32)
            - ((unsafe {
                *unsafe {
                    unsafe { (*pProbe).aiRowLogEst }
                        .offset((((saved_nEq as u32) as i32) + (1 as i32)) as isize)
                }
            }) as i32)) as i16;
        let __v2249: *mut WhereLoop = pNew;
        let __v2250: i16 = unsafe { (*__v2249).nOut };
        let __v2251: i16 = ((__v2250 as i32) - (nIter as i32)) as i16;
        unsafe {
            (*__v2249).nOut = __v2251;
        }
        // /* TUNING:  Because uncertainties in the estimates for skip-scan queries,
        //     ** add a 1.375 fudge factor to make skip-scan slightly less likely. */
        let __v2252: i16 = nIter;
        let __v2253: i16 = ((__v2252 as i32) + (5 as i32)) as i16;
        nIter = __v2253;
        whereLoopAddBtreeIndex(
            pBuilder,
            pSrc,
            pProbe,
            ((nIter as i32) + (nInMul as i32)) as i16,
        );
        unsafe {
            (*pNew).nOut = saved_nOut;
        }
        unsafe {
            (*pNew).u.btree.nEq = saved_nEq;
        }
        unsafe {
            (*pNew).nSkip = saved_nSkip;
        }
        unsafe {
            (*pNew).wsFlags = saved_wsFlags;
        }
    }
    // /* TUNING: Minimum for skip-scan */
    {}
    return rc;
}

// /* The WhereLoop factory */
// /* FROM clause term being analyzed */
// /* An index on pSrc */
// /* log(Number of iterations due to IN) */
// /*
// ** Return True if it is possible that pIndex might be useful in
// ** implementing the ORDER BY clause in pBuilder.
// **
// ** Return False if pBuilder does not contain an ORDER BY clause or
// ** if there is no way for pIndex to be useful in implementing that
// ** ORDER BY clause.
// */
fn indexMightHelpWithOrderBy(
    mut pBuilder: *mut WhereLoopBuilder,
    mut pIndex: *mut Index,
    mut iCursor: i32,
) -> i32 {
    let mut pOB: *mut ExprList = unsafe { std::mem::zeroed() };
    let mut aColExpr: *mut ExprList = unsafe { std::mem::zeroed() };
    let mut ii: i32 = 0 as i32;
    let mut jj: i32 = 0 as i32;
    if ((unsafe { (*pIndex).__slate_bits_0.__get_bUnordered() }) as i32) != (0 as i32) {
        return 0 as i32;
    }
    let __v2254: *mut ExprList = unsafe { (*unsafe { (*pBuilder).pWInfo }).pOrderBy };
    pOB = __v2254;
    if __v2254 == std::ptr::null_mut::<ExprList>() {
        return 0 as i32;
    }
    ii = 0 as i32;
    '__slate_break_1772: loop {
        if !(ii < unsafe { (*pOB).nExpr }) {
            break;
        }
        let mut pExpr: *mut Expr = unsafe {
            sqlite3ExprSkipCollateAndLikely(unsafe {
                (*unsafe {
                    unsafe { std::ptr::addr_of_mut!((*pOB).a) as *mut ExprList_item }
                        .offset(ii as isize)
                })
                .pExpr
            })
        };
        if pExpr == std::ptr::null_mut::<Expr>() {
        } else {
            if ((((unsafe { (*pExpr).op }) as u32) as i32) == (168 as i32)
                || (((unsafe { (*pExpr).op }) as u32) as i32) == (170 as i32))
                && (unsafe { (*pExpr).iTable }) == iCursor
            {
                if ((unsafe { (*pExpr).iColumn }) as i32) < (0 as i32) {
                    return 1 as i32;
                }
                jj = 0 as i32;
                '__slate_break_1773: loop {
                    if !(jj < (((unsafe { (*pIndex).nKeyCol }) as u32) as i32)) {
                        break;
                    }
                    if ((unsafe { (*pExpr).iColumn }) as i32)
                        == ((unsafe {
                            *unsafe { unsafe { (*pIndex).aiColumn }.offset(jj as isize) }
                        }) as i32)
                    {
                        return 1 as i32;
                    }
                    let __v2257: i32 = jj;
                    let __v2258: i32 = __v2257 + (1 as i32);
                    jj = __v2258;
                }
            } else {
                let __v2259: *mut ExprList = unsafe { (*pIndex).aColExpr };
                aColExpr = __v2259;
                if __v2259 != std::ptr::null_mut::<ExprList>() {
                    jj = 0 as i32;
                    '__slate_break_1774: loop {
                        if !(jj < (((unsafe { (*pIndex).nKeyCol }) as u32) as i32)) {
                            break;
                        }
                        if ((unsafe {
                            *unsafe { unsafe { (*pIndex).aiColumn }.offset(jj as isize) }
                        }) as i32)
                            != -(2 as i32)
                        {
                        } else {
                            if (unsafe {
                                sqlite3ExprCompareSkip(
                                    pExpr,
                                    unsafe {
                                        (*unsafe {
                                            unsafe {
                                                std::ptr::addr_of_mut!((*aColExpr).a)
                                                    as *mut ExprList_item
                                            }
                                            .offset(jj as isize)
                                        })
                                        .pExpr
                                    },
                                    iCursor,
                                )
                            }) == (0 as i32)
                            {
                                return 1 as i32;
                            }
                        }
                        let __v2260: i32 = jj;
                        let __v2261: i32 = __v2260 + (1 as i32);
                        jj = __v2261;
                    }
                }
            }
        }
        let __v2255: i32 = ii;
        let __v2256: i32 = __v2255 + (1 as i32);
        ii = __v2256;
    }
    return 0 as i32;
}

// /* Check to see if a partial index with pPartIndexWhere can be used
// ** in the current query.  Return true if it can be and false if not.
// */
fn whereUsablePartialIndex(
    mut iTab: i32,
    mut jointype: u8,
    mut pWC: *mut WhereClause,
    mut pWhere: *mut Expr,
) -> i32 {
    let mut i: i32 = 0 as i32;
    let mut pTerm: *mut WhereTerm = unsafe { std::mem::zeroed() };
    let mut pParse: *mut Parse = unsafe { std::mem::zeroed() };
    if ((jointype as u32) as i32) & (64 as i32) != (0 as i32) {
        return 0 as i32;
    }
    pParse = unsafe { (*unsafe { (*pWC).pWInfo }).pParse };
    '__slate_break_1775: while (((unsafe { (*pWhere).op }) as u32) as i32) == (44 as i32) {
        if !(whereUsablePartialIndex(iTab, jointype, pWC, unsafe { (*pWhere).pLeft }) != (0 as i32))
        {
            return 0 as i32;
        }
        pWhere = unsafe { (*pWhere).pRight };
    }
    i = 0 as i32;
    let __v2262: *mut WhereTerm = unsafe { (*pWC).a };
    pTerm = __v2262;
    '__slate_break_1776: while i < unsafe { (*pWC).nTerm } {
        let mut pExpr: *mut Expr = unsafe { std::mem::zeroed() };
        pExpr = unsafe { (*pTerm).pExpr };
        let __v2267: bool;
        if (!((unsafe { (*pExpr).flags }) & ((1 as i32) as u32) != ((0 as i32) as u32))
            || (unsafe { (*pExpr).w.iJoin }) == iTab)
            && (((jointype as u32) as i32) & (32 as i32) == (0 as i32)
                || (unsafe { (*pExpr).flags }) & ((1 as i32) as u32) != ((0 as i32) as u32))
        {
            __v2267 = (unsafe {
                sqlite3ExprImpliesExpr(
                    pParse as *const Parse,
                    pExpr as *const Expr,
                    pWhere as *const Expr,
                    iTab,
                )
            }) != (0 as i32);
        } else {
            __v2267 = false as bool;
        }
        let __v2268: bool;
        if __v2267 {
            __v2268 = !((unsafe {
                sqlite3ExprImpliesExpr(
                    pParse as *const Parse,
                    pExpr as *const Expr,
                    pWhere as *const Expr,
                    -(1 as i32),
                )
            }) != (0 as i32));
        } else {
            __v2268 = false as bool;
        }
        if __v2268 && (((unsafe { (*pTerm).wtFlags }) as u32) as i32) & (128 as i32) == (0 as i32) {
            return 1 as i32;
        }
        let __v2263: i32 = i;
        let __v2264: i32 = __v2263 + (1 as i32);
        i = __v2264;
        let __v2265: *mut WhereTerm = pTerm;
        let __v2266: *mut WhereTerm = unsafe { __v2265.offset((1 as i32) as isize) };
        pTerm = __v2266;
    }
    return 0 as i32;
}

// /* The table for which we want an index */
// /* The JT_* flags on the join */
// /* The WHERE clause of the query */
// /* The WHERE clause from the partial index */
// /*
// ** pIdx is an index containing expressions.  Check it see if any of the
// ** expressions in the index match the pExpr expression.
// */
fn exprIsCoveredByIndex(mut pExpr: *const Expr, mut pIdx: *const Index, mut iTabCur: i32) -> i32 {
    let mut i: i32 = 0 as i32;
    i = 0 as i32;
    '__slate_break_1777: loop {
        if !(i < (((unsafe { (*pIdx).nColumn }) as u32) as i32)) {
            break;
        }
        let __v2271: bool;
        if ((unsafe { *unsafe { unsafe { (*pIdx).aiColumn }.offset(i as isize) } }) as i32)
            == -(2 as i32)
        {
            __v2271 = (unsafe {
                sqlite3ExprCompare(
                    std::ptr::null::<Parse>(),
                    pExpr,
                    (unsafe {
                        (*unsafe {
                            unsafe {
                                std::ptr::addr_of_mut!((*unsafe { (*pIdx).aColExpr }).a)
                                    as *mut ExprList_item
                            }
                            .offset(i as isize)
                        })
                        .pExpr
                    }) as *const Expr,
                    iTabCur,
                )
            }) == (0 as i32);
        } else {
            __v2271 = false as bool;
        }
        if __v2271 {
            return 1 as i32;
        }
        let __v2269: i32 = i;
        let __v2270: i32 = __v2269 + (1 as i32);
        i = __v2270;
    }
    return 0 as i32;
}

// /*
// ** Information passed in is pWalk->u.pCovIdxCk.  Call it pCk.
// **
// ** If the Expr node references the table with cursor pCk->iTabCur, then
// ** make sure that column is covered by the index pCk->pIdx.  We know that
// ** all columns less than 63 (really BMS-1) are covered, so we don't need
// ** to check them.  But we do need to check any column at 63 or greater.
// **
// ** If the index does not cover the column, then set pWalk->eCode to
// ** non-zero and return WRC_Abort to stop the search.
// **
// ** If this node does not disprove that the index can be a covering index,
// ** then just return WRC_Continue, to continue the search.
// **
// ** If pCk->pIdx contains indexed expressions and one of those expressions
// ** matches pExpr, then prune the search.
// */
#[unsafe(link_section = ".text.slate_distinct.where.whereIsCoveringIndexWalkCallback")]
extern "C-unwind" fn whereIsCoveringIndexWalkCallback(
    mut pWalk: *mut Walker,
    mut pExpr: *mut Expr,
) -> i32 {
    // /* Loop counter */
    let mut i: i32 = 0 as i32;
    // /* The index of interest */
    let mut pIdx: *const Index = unsafe { std::mem::zeroed() };
    // /* Columns contained in the index */
    let mut aiColumn: *const i16 = unsafe { std::mem::zeroed() };
    // /* Number of columns in the index */
    let mut nColumn: u16 = 0 as u16;
    // /* Info about this search */
    let mut pCk: *mut CoveringIndexCheck = unsafe { std::mem::zeroed() };
    pCk = unsafe { (*pWalk).u.pCovIdxCk };
    pIdx = (unsafe { (*pCk).pIdx }) as *const Index;
    if (((unsafe { (*pExpr).op }) as u32) as i32) == (168 as i32)
        || (((unsafe { (*pExpr).op }) as u32) as i32) == (170 as i32)
    {
        // /* if( pExpr->iColumn<(BMS-1) && pIdx->bHasExpr==0 ) return WRC_Continue;*/
        if (unsafe { (*pExpr).iTable }) != unsafe { (*pCk).iTabCur } {
            return 0 as i32;
        }
        pIdx = (unsafe { (*unsafe { (*pWalk).u.pCovIdxCk }).pIdx }) as *const Index;
        aiColumn = (unsafe { (*pIdx).aiColumn }) as *const i16;
        nColumn = unsafe { (*pIdx).nColumn };
        i = 0 as i32;
        '__slate_break_1778: loop {
            if !(i < ((nColumn as u32) as i32)) {
                break;
            }
            if ((unsafe { *unsafe { aiColumn.offset(i as isize) } }) as i32)
                == ((unsafe { (*pExpr).iColumn }) as i32)
            {
                return 0 as i32;
            }
            let __v2272: i32 = i;
            let __v2273: i32 = __v2272 + (1 as i32);
            i = __v2273;
        }
        unsafe {
            (*pCk).bUnidx = ((1 as i32) as i8) as u8;
        }
        return 2 as i32;
    } else {
        let __v2274: bool;
        if ((unsafe { (*pIdx).__slate_bits_0.__get_bHasExpr() }) as i32) != (0 as i32) {
            __v2274 = exprIsCoveredByIndex(pExpr as *const Expr, pIdx, unsafe {
                (*unsafe { (*pWalk).u.pCovIdxCk }).iTabCur
            }) != (0 as i32);
        } else {
            __v2274 = false as bool;
        }
        if __v2274 {
            unsafe {
                (*pCk).bExpr = ((1 as i32) as i8) as u8;
            }
            return 1 as i32;
        }
    }
    return 0 as i32;
}

// /*
// ** pIdx is an index that covers all of the low-number columns used by
// ** pWInfo->pSelect (columns from 0 through 62) or an index that has
// ** expressions terms.  Hence, we cannot determine whether or not it is
// ** a covering index by using the colUsed bitmasks.  We have to do a search
// ** to see if the index is covering.  This routine does that search.
// **
// ** The return value is one of these:
// **
// **      0                The index is definitely not a covering index
// **
// **      WHERE_IDX_ONLY   The index is definitely a covering index
// **
// **      WHERE_EXPRIDX    The index is likely a covering index, but it is
// **                       difficult to determine precisely because of the
// **                       expressions that are indexed.  Score it as a
// **                       covering index, but still keep the main table open
// **                       just in case we need it.
// **
// ** This routine is an optimization.  It is always safe to return zero.
// ** But returning one of the other two values when zero should have been
// ** returned can lead to incorrect bytecode and assertion faults.
// */
fn whereIsCoveringIndex(mut pWInfo: *mut WhereInfo, mut pIdx: *mut Index, mut iTabCur: i32) -> u32 {
    let mut i: i32 = 0 as i32;
    let mut rc: i32 = 0 as i32;
    let mut ck: CoveringIndexCheck = unsafe { std::mem::zeroed() };
    let mut w: Walker = unsafe { std::mem::zeroed() };
    if (unsafe { (*pWInfo).pSelect }) == std::ptr::null_mut::<Select>() {
        // /* We don't have access to the full query, so we cannot check to see
        //     ** if pIdx is covering.  Assume it is not. */
        return (0 as i32) as u32;
    }
    if ((unsafe { (*pIdx).__slate_bits_0.__get_bHasExpr() }) as i32) == (0 as i32) {
        i = 0 as i32;
        '__slate_break_1779: loop {
            if !(i < (((unsafe { (*pIdx).nColumn }) as u32) as i32)) {
                break;
            }
            if ((unsafe { *unsafe { unsafe { (*pIdx).aiColumn }.offset(i as isize) } }) as i32)
                >= (((8 as u64).wrapping_mul(((8 as i32) as i64) as u64) as u32) as i32)
                    - (1 as i32)
            {
                break '__slate_break_1779;
            }
            let __v2275: i32 = i;
            let __v2276: i32 = __v2275 + (1 as i32);
            i = __v2276;
        }
        if i >= (((unsafe { (*pIdx).nColumn }) as u32) as i32) {
            // /* pIdx does not index any columns greater than 62, but we know from
            //       ** colMask that columns greater than 62 are used, so this is not a
            //       ** covering index */
            return (0 as i32) as u32;
        }
    }
    ck.pIdx = pIdx;
    ck.iTabCur = iTabCur;
    ck.bExpr = ((0 as i32) as i8) as u8;
    ck.bUnidx = ((0 as i32) as i8) as u8;
    unsafe { memset(std::ptr::addr_of_mut!(w) as *mut (), 0 as i32, 48 as u64) };
    w.xExprCallback = Some(whereIsCoveringIndexWalkCallback);
    w.xSelectCallback = unsafe {
        std::mem::transmute::<
            *const (),
            Option<unsafe extern "C-unwind" fn(*mut Walker, *mut Select) -> i32>,
        >(sqlite3SelectWalkNoop as *const ())
    };
    unsafe {
        w.u.pCovIdxCk = std::ptr::addr_of_mut!(ck);
    }
    unsafe { sqlite3WalkSelect(std::ptr::addr_of_mut!(w), unsafe { (*pWInfo).pSelect }) };
    if ck.bUnidx != (0 as u8) {
        rc = 0 as i32;
    } else {
        if ck.bExpr != (0 as u8) {
            rc = 67108864 as i32;
        } else {
            rc = 64 as i32;
        }
    }
    return rc as u32;
}

// /* The WHERE clause context */
// /* Index that is being tested */
// /* Cursor for the table being indexed */
// /*
// ** This is an sqlite3ParserAddCleanup() callback that is invoked to
// ** free the Parse->pIdxEpr list when the Parse object is destroyed.
// */
#[unsafe(link_section = ".text.slate_distinct.where.whereIndexedExprCleanup")]
extern "C-unwind" fn whereIndexedExprCleanup(mut db: *mut sqlite3, mut pObject: *mut ()) {
    let mut pp: *mut *mut IndexedExpr = pObject as *mut *mut IndexedExpr;
    '__slate_break_1780: while (unsafe { *pp }) != std::ptr::null_mut::<IndexedExpr>() {
        let mut p: *mut IndexedExpr = unsafe { *pp };
        unsafe {
            *pp = unsafe { (*p).pIENext };
        }
        unsafe { sqlite3ExprDelete(db, unsafe { (*p).pExpr }) };
        unsafe { sqlite3DbFreeNN(db, p as *mut ()) };
    }
}

// /*
// ** This function is called for a partial index - one with a WHERE clause - in
// ** two scenarios. In both cases, it determines whether or not the WHERE
// ** clause on the index implies that a column of the table may be safely
// ** replaced by a constant expression. For example, in the following
// ** SELECT:
// **
// **   CREATE INDEX i1 ON t1(b, c) WHERE a=<expr>;
// **   SELECT a, b, c FROM t1 WHERE a=<expr> AND b=?;
// **
// ** The "a" in the select-list may be replaced by <expr>, iff:
// **
// **    (a) <expr> is a constant expression, and
// **    (b) The (a=<expr>) comparison uses the BINARY collation sequence, and
// **    (c) Column "a" has an affinity other than NONE or BLOB.
// **
// ** If argument pItem is NULL, then pMask must not be NULL. In this case this
// ** function is being called as part of determining whether or not pIdx
// ** is a covering index. This function clears any bits in (*pMask)
// ** corresponding to columns that may be replaced by constants as described
// ** above.
// **
// ** Otherwise, if pItem is not NULL, then this function is being called
// ** as part of coding a loop that uses index pIdx. In this case, add entries
// ** to the Parse.pIdxPartExpr list for each column that can be replaced
// ** by a constant.
// */
fn wherePartIdxExpr(
    mut pParse: *mut Parse,
    mut pIdx: *mut Index,
    mut pPart: *mut Expr,
    mut pMask: *mut u64,
    mut iIdxCur: i32,
    mut pItem: *mut SrcItem,
) {
    0 as i32;
    0 as i32;
    if (((unsafe { (*pPart).op }) as u32) as i32) == (44 as i32) {
        wherePartIdxExpr(
            pParse,
            pIdx,
            unsafe { (*pPart).pRight },
            pMask,
            iIdxCur,
            pItem,
        );
        pPart = unsafe { (*pPart).pLeft };
    }
    if (((unsafe { (*pPart).op }) as u32) as i32) == (54 as i32)
        || (((unsafe { (*pPart).op }) as u32) as i32) == (45 as i32)
    {
        let mut pLeft: *mut Expr = unsafe { (*pPart).pLeft };
        let mut pRight: *mut Expr = unsafe { (*pPart).pRight };
        let mut aff: u8 = 0 as u8;
        if (((unsafe { (*pLeft).op }) as u32) as i32) != (168 as i32) {
            return;
        }
        if !((unsafe { sqlite3ExprIsConstant(std::ptr::null_mut::<Parse>(), pRight) })
            != (0 as i32))
        {
            return;
        }
        if !((unsafe {
            sqlite3IsBinary(
                (unsafe { sqlite3ExprCompareCollSeq(pParse, pPart as *const Expr) })
                    as *const CollSeq,
            )
        }) != (0 as i32))
        {
            return;
        }
        if ((unsafe { (*pLeft).iColumn }) as i32) < (0 as i32) {
            return;
        }
        aff = (unsafe {
            (*unsafe {
                unsafe { (*unsafe { (*pIdx).pTable }).aCol }
                    .offset(((unsafe { (*pLeft).iColumn }) as i32) as isize)
            })
            .affinity
        }) as u8;
        if ((aff as u32) as i32) >= (66 as i32) {
            if pItem != std::ptr::null_mut::<SrcItem>() {
                let mut db: *mut sqlite3 = unsafe { (*pParse).db };
                let mut p: *mut IndexedExpr =
                    (unsafe { sqlite3DbMallocRaw(db, 32 as u64) }) as *mut IndexedExpr;
                if p != std::ptr::null_mut::<IndexedExpr>() {
                    let mut bNullRow: i32 = ((((unsafe { (*pItem).fg.jointype }) as u32) as i32)
                        & ((8 as i32) | (64 as i32))
                        != (0 as i32)) as i32;
                    unsafe {
                        (*p).pExpr = unsafe { sqlite3ExprDup(db, pRight as *const Expr, 0 as i32) };
                    }
                    unsafe {
                        (*p).iDataCur = unsafe { (*pItem).iCursor };
                    }
                    unsafe {
                        (*p).iIdxCur = iIdxCur;
                    }
                    unsafe {
                        (*p).iIdxCol = (unsafe { (*pLeft).iColumn }) as i32;
                    }
                    unsafe {
                        (*p).bMaybeNullRow = (bNullRow as i8) as u8;
                    }
                    unsafe {
                        (*p).pIENext = unsafe { (*pParse).pIdxPartExpr };
                    }
                    unsafe {
                        (*p).aff = aff;
                    }
                    unsafe {
                        (*pParse).pIdxPartExpr = p;
                    }
                    if (unsafe { (*p).pIENext }) == std::ptr::null_mut::<IndexedExpr>() {
                        let mut pArg: *mut () =
                            (unsafe { std::ptr::addr_of_mut!((*pParse).pIdxPartExpr) }) as *mut ();
                        unsafe {
                            sqlite3ParserAddCleanup(pParse, Some(whereIndexedExprCleanup), pArg)
                        };
                    }
                }
            } else {
                if ((unsafe { (*pLeft).iColumn }) as i32)
                    < (((8 as u64).wrapping_mul(((8 as i32) as i64) as u64) as u32) as i32)
                        - (1 as i32)
                {
                    let __v2277: *mut u64 = pMask;
                    let __v2278: u64 = unsafe { *__v2277 };
                    let __v2279: u64 = __v2278
                        & !((((1 as i32) as i64) as u64) << ((unsafe { (*pLeft).iColumn }) as i32));
                    unsafe {
                        *__v2277 = __v2279;
                    }
                }
            }
        }
    }
}

// /* Parse context */
// /* Partial index being processed */
// /* WHERE clause being processed */
// /* Mask to clear bits in */
// /* Cursor number for index */
// /* The FROM clause entry for the table */
// /*
// ** Add all WhereLoop objects for a single table of the join where the table
// ** is identified by pBuilder->pNew->iTab.  That table is guaranteed to be
// ** a b-tree table, not a virtual table.
// **
// ** The costs (WhereLoop.rRun) of the b-tree loops added by this function
// ** are calculated as follows:
// **
// ** For a full scan, assuming the table (or index) contains nRow rows:
// **
// **     cost = nRow * 3.0                    // full-table scan
// **     cost = nRow * K                      // scan of covering index
// **     cost = nRow * (K+3.0)                // scan of non-covering index
// **
// ** where K is a value between 1.1 and 3.0 set based on the relative
// ** estimated average size of the index and table records.
// **
// ** For an index scan, where nVisit is the number of index rows visited
// ** by the scan, and nSeek is the number of seek operations required on
// ** the index b-tree:
// **
// **     cost = nSeek * (log(nRow) + K * nVisit)          // covering index
// **     cost = nSeek * (log(nRow) + (K+3.0) * nVisit)    // non-covering index
// **
// ** Normally, nSeek is 1. nSeek values greater than 1 come about if the
// ** WHERE clause includes "x IN (....)" terms used in place of "x=?". Or when
// ** implicit "x IN (SELECT x FROM tbl)" terms are added for skip-scans.
// **
// ** The estimated values (nRow, nVisit, nSeek) often contain a large amount
// ** of uncertainty.  For this reason, scoring is designed to pick plans that
// ** "do the least harm" if the estimates are inaccurate.  For example, a
// ** log(nRow) factor is omitted from a non-covering index scan in order to
// ** bias the scoring in favor of using an index, since the worst-case
// ** performance of using an index is far better than the worst-case performance
// ** of a full table scan.
// */
fn whereLoopAddBtree(mut pBuilder: *mut WhereLoopBuilder, mut mPrereq: u64) -> i32 {
    // /* WHERE analysis context */
    let mut pWInfo: *mut WhereInfo = unsafe { std::mem::zeroed() };
    // /* An index we are evaluating */
    let mut pProbe: *mut Index = unsafe { std::mem::zeroed() };
    // /* A fake index object for the primary key */
    let mut sPk: Index = unsafe { std::mem::zeroed() };
    // /* The aiRowLogEst[] value for the sPk index */
    let mut aiRowEstPk: [i16; 2] = [0 as i16; 2];
    // /* The aColumn[] value for the sPk index */
    let mut aiColumnPk: i16 = -(1 as i32) as i16;
    // /* The FROM clause */
    let mut pTabList: *mut SrcList = unsafe { std::mem::zeroed() };
    // /* The FROM clause btree term to add */
    let mut pSrc: *mut SrcItem = unsafe { std::mem::zeroed() };
    // /* Template WhereLoop object */
    let mut pNew: *mut WhereLoop = unsafe { std::mem::zeroed() };
    // /* Return code */
    let mut rc: i32 = 0 as i32;
    // /* Index number */
    let mut iSortIdx: i32 = 1 as i32;
    // /* A boolean value */
    let mut b: i32 = 0 as i32;
    // /* number of rows in the table */
    let mut rSize: i16 = 0 as i16;
    // /* The parsed WHERE clause */
    let mut pWC: *mut WhereClause = unsafe { std::mem::zeroed() };
    // /* Table being queried */
    let mut pTab: *mut Table = unsafe { std::mem::zeroed() };
    pNew = unsafe { (*pBuilder).pNew };
    pWInfo = unsafe { (*pBuilder).pWInfo };
    pTabList = unsafe { (*pWInfo).pTabList };
    pSrc = unsafe {
        unsafe { std::ptr::addr_of_mut!((*pTabList).a) as *mut SrcItem }
            .offset((((unsafe { (*pNew).iTab }) as u32) as i32) as isize)
    };
    pTab = unsafe { (*pSrc).pSTab };
    pWC = unsafe { (*pBuilder).pWC };
    0 as i32;
    if ((unsafe { (*pSrc).fg.__slate_bits_0.__get_isIndexedBy() }) as i32) != (0 as i32) {
        0 as i32;
        // /* An INDEXED BY clause specifies a particular index to use */
        pProbe = unsafe { (*pSrc).u2.pIBIndex };
    } else {
        if !((unsafe { (*pTab).tabFlags }) & ((128 as i32) as u32) == ((0 as i32) as u32)) {
            pProbe = unsafe { (*pTab).pIndex };
        } else {
            // /* There is no INDEXED BY clause.  Create a fake Index object in local
            //     ** variable sPk to represent the rowid primary key index.  Make this
            //     ** fake index the first in a chain of Index objects with all of the real
            //     ** indices to follow */
            // /* First of real indices on the table */
            let mut pFirst: *mut Index = unsafe { std::mem::zeroed() };
            unsafe { memset(std::ptr::addr_of_mut!(sPk) as *mut (), 0 as i32, 112 as u64) };
            sPk.nKeyCol = ((1 as i32) as i16) as u16;
            sPk.nColumn = ((1 as i32) as i16) as u16;
            sPk.aiColumn = std::ptr::addr_of_mut!(aiColumnPk);
            sPk.aiRowLogEst = aiRowEstPk.as_mut_ptr() as *mut i16;
            sPk.onError = ((5 as i32) as i8) as u8;
            sPk.pTable = pTab;
            // /* TUNING: Interior rows of IPK table are very small */
            sPk.szIdxRow = (3 as i32) as i16;
            sPk.__slate_bits_0.__set_idxType((3 as i32) as u32);
            unsafe {
                *unsafe { (aiRowEstPk.as_mut_ptr() as *mut i16).offset((0 as i32) as isize) } =
                    unsafe { (*pTab).nRowLogEst };
            }
            unsafe {
                *unsafe { (aiRowEstPk.as_mut_ptr() as *mut i16).offset((1 as i32) as isize) } =
                    (0 as i32) as i16;
            }
            pFirst = unsafe { (*unsafe { (*pSrc).pSTab }).pIndex };
            if ((unsafe { (*pSrc).fg.__slate_bits_0.__get_notIndexed() }) as i32) == (0 as i32) {
                // /* The real indices of the table are only considered if the
                //       ** NOT INDEXED qualifier is omitted from the FROM clause */
                sPk.pNext = pFirst;
            }
            pProbe = std::ptr::addr_of_mut!(sPk);
        }
    }
    rSize = unsafe { (*pTab).nRowLogEst };
    // /* Automatic indexes */
    // /* Not part of an OR optimization */
    if !((unsafe { (*pBuilder).pOrSet }) != std::ptr::null_mut::<WhereOrSet>())
        && (((unsafe { (*pWInfo).wctrlFlags }) as u32) as i32) & ((4096 as i32) | (32 as i32))
            == (0 as i32)
        && (unsafe { (*unsafe { (*unsafe { (*pWInfo).pParse }).db }).flags })
            & (((32768 as i32) as i64) as u64)
            != (((0 as i32) as i64) as u64)
        && !(((unsafe { (*pSrc).fg.__slate_bits_0.__get_isIndexedBy() }) as i32) != (0 as i32))
        && !(((unsafe { (*pSrc).fg.__slate_bits_0.__get_notIndexed() }) as i32) != (0 as i32))
        && !(((unsafe { (*pSrc).fg.__slate_bits_0.__get_isCorrelated() }) as i32) != (0 as i32))
        && !(((unsafe { (*pSrc).fg.__slate_bits_0.__get_isRecursive() }) as i32) != (0 as i32))
        && (((unsafe { (*pSrc).fg.jointype }) as u32) as i32) & (16 as i32) == (0 as i32)
    {
        // /* Generate auto-index WhereLoops */
        // /* Logarithm of the number of rows in the table */
        let mut rLogSize: i16 = 0 as i16;
        let mut pTerm: *mut WhereTerm = unsafe { std::mem::zeroed() };
        let mut pWCEnd: *mut WhereTerm =
            unsafe { unsafe { (*pWC).a }.offset((unsafe { (*pWC).nTerm }) as isize) };
        rLogSize = estLog(rSize);
        pTerm = unsafe { (*pWC).a };
        '__slate_break_1781: while rc == (0 as i32) && pTerm < pWCEnd {
            if (unsafe { (*pTerm).prereqRight }) & unsafe { (*pNew).maskSelf } != (0 as u64) {
            } else {
                if termCanDriveIndex(
                    pTerm as *const WhereTerm,
                    pSrc as *const SrcItem,
                    ((0 as i32) as i64) as u64,
                ) != (0 as i32)
                {
                    unsafe {
                        (*pNew).u.btree.nEq = ((1 as i32) as i16) as u16;
                    }
                    unsafe {
                        (*pNew).nSkip = ((0 as i32) as i16) as u16;
                    }
                    unsafe {
                        (*pNew).u.btree.pIndex = std::ptr::null_mut::<Index>();
                    }
                    unsafe {
                        (*pNew).nLTerm = ((1 as i32) as i16) as u16;
                    }
                    unsafe {
                        *unsafe { unsafe { (*pNew).aLTerm }.offset((0 as i32) as isize) } = pTerm;
                    }
                    // /* TUNING: One-time cost for computing the automatic index is
                    //         ** estimated to be X*N*log2(N) where N is the number of rows in
                    //         ** the table being indexed and where X is 7 (LogEst=28) for normal
                    //         ** tables or 0.5 (LogEst=-10) for views and subqueries.  The value
                    //         ** of X is smaller for views and subqueries so that the query planner
                    //         ** will be more aggressive about generating automatic indexes for
                    //         ** those objects, since there is no opportunity to add schema
                    //         ** indexes on subqueries and views. */
                    unsafe {
                        (*pNew).rSetup = ((rLogSize as i32) + (rSize as i32)) as i16;
                    }
                    if !((((unsafe { (*pTab).eTabType }) as u32) as i32) == (2 as i32))
                        && (unsafe { (*pTab).tabFlags }) & ((16384 as i32) as u32)
                            == ((0 as i32) as u32)
                    {
                        let __v2282: *mut WhereLoop = pNew;
                        let __v2283: i16 = unsafe { (*__v2282).rSetup };
                        let __v2284: i16 = ((__v2283 as i32) + (28 as i32)) as i16;
                        unsafe {
                            (*__v2282).rSetup = __v2284;
                        }
                    } else {
                        // /* Greatly reduced setup cost for auto indexes
                        //                                ** on ephemeral materializations of views */
                        let __v2285: *mut WhereLoop = pNew;
                        let __v2286: i16 = unsafe { (*__v2285).rSetup };
                        let __v2287: i16 = ((__v2286 as i32) - (25 as i32)) as i16;
                        unsafe {
                            (*__v2285).rSetup = __v2287;
                        }
                    }
                    {}
                    if ((unsafe { (*pNew).rSetup }) as i32) < (0 as i32) {
                        unsafe {
                            (*pNew).rSetup = (0 as i32) as i16;
                        }
                    }
                    // /* TUNING: Each index lookup yields 20 rows in the table.  This
                    //         ** is more than the usual guess of 10 rows, since we have no way
                    //         ** of knowing how selective the index will ultimately be.  It would
                    //         ** not be unreasonable to make this value much larger. */
                    unsafe {
                        (*pNew).nOut = (43 as i32) as i16;
                    }
                    0 as i32;
                    unsafe {
                        (*pNew).rRun =
                            unsafe { sqlite3LogEstAdd(rLogSize, unsafe { (*pNew).nOut }) };
                    }
                    unsafe {
                        (*pNew).wsFlags = (16384 as i32) as u32;
                    }
                    unsafe {
                        (*pNew).prereq = mPrereq | unsafe { (*pTerm).prereqRight };
                    }
                    rc = whereLoopInsert(pBuilder, pNew);
                }
            }
            let __v2280: *mut WhereTerm = pTerm;
            let __v2281: *mut WhereTerm = unsafe { __v2280.offset((1 as i32) as isize) };
            pTerm = __v2281;
        }
    }
    // /* Has no INDEXED BY clause */
    // /* Has no NOT INDEXED clause */
    // /* Not a correlated subquery */
    // /* Not a recursive common table expression. */
    // /* Not the right tab of a RIGHT JOIN */
    // /* SQLITE_OMIT_AUTOMATIC_INDEX */
    // /* Loop over all indices. If there was an INDEXED BY clause, then only
    //   ** consider index pProbe.  */
    '__slate_break_1782: loop {
        if !(rc == (0 as i32) && pProbe != std::ptr::null_mut::<Index>()) {
            break;
        }
        let __v2290: bool;
        if (unsafe { (*pProbe).pPartIdxWhere }) != std::ptr::null_mut::<Expr>() {
            __v2290 = !(whereUsablePartialIndex(
                unsafe { (*pSrc).iCursor },
                unsafe { (*pSrc).fg.jointype },
                pWC,
                unsafe { (*pProbe).pPartIdxWhere },
            ) != (0 as i32));
        } else {
            __v2290 = false as bool;
        }
        if __v2290 {
            // /* See ticket [98d973b8f5] */
            {}
        // /* Partial index inappropriate for this query */
        } else {
            if ((unsafe { (*pProbe).__slate_bits_0.__get_bNoQuery() }) as i32) != (0 as i32) {
            } else {
                rSize = unsafe {
                    *unsafe { unsafe { (*pProbe).aiRowLogEst }.offset((0 as i32) as isize) }
                };
                unsafe {
                    (*pNew).u.btree.nEq = ((0 as i32) as i16) as u16;
                }
                unsafe {
                    (*pNew).u.btree.nBtm = ((0 as i32) as i16) as u16;
                }
                unsafe {
                    (*pNew).u.btree.nTop = ((0 as i32) as i16) as u16;
                }
                unsafe {
                    (*pNew).u.btree.nDistinctCol = ((0 as i32) as i16) as u16;
                }
                unsafe {
                    (*pNew).nSkip = ((0 as i32) as i16) as u16;
                }
                unsafe {
                    (*pNew).nLTerm = ((0 as i32) as i16) as u16;
                }
                unsafe {
                    (*pNew).iSortIdx = ((0 as i32) as i8) as u8;
                }
                unsafe {
                    (*pNew).rSetup = (0 as i32) as i16;
                }
                unsafe {
                    (*pNew).prereq = mPrereq;
                }
                unsafe {
                    (*pNew).nOut = rSize;
                }
                unsafe {
                    (*pNew).u.btree.pIndex = pProbe;
                }
                unsafe {
                    (*pNew).u.btree.pOrderBy = std::ptr::null_mut::<ExprList>();
                }
                b = indexMightHelpWithOrderBy(pBuilder, pProbe, unsafe { (*pSrc).iCursor });
                // /* The ONEPASS_DESIRED flags never occurs together with ORDER BY */
                0 as i32;
                if ((unsafe { (*pProbe).__slate_bits_0.__get_idxType() }) as i32) == (3 as i32) {
                    // /* Integer primary key index */
                    unsafe {
                        (*pNew).wsFlags = (256 as i32) as u32;
                    }
                    // /* Full table scan */
                    unsafe {
                        (*pNew).iSortIdx =
                            ((if b != (0 as i32) { iSortIdx } else { 0 as i32 }) as i8) as u8;
                    }
                    // /* TUNING: Cost of full table scan is 3.0*N.  The 3.0 factor is an
                    //       ** extra cost designed to discourage the use of full table scans,
                    //       ** since index lookups have better worst-case performance if our
                    //       ** stat guesses are wrong.  Reduce the 3.0 penalty slightly
                    //       ** (to 2.75) if we have valid STAT4 information for the table.
                    //       ** At 2.75, a full table scan is preferred over using an index on
                    //       ** a column with just two distinct values where each value has about
                    //       ** an equal number of appearances.  Without STAT4 data, we still want
                    //       ** to use an index in that case, since the constraint might be for
                    //       ** the scarcer of the two values, and in that case an index lookup is
                    //       ** better.
                    //       */
                    unsafe {
                        (*pNew).rRun = ((rSize as i32) + (16 as i32)) as i16;
                    }
                    {}
                    whereLoopOutputAdjust(pWC, pNew, rSize);
                    if ((unsafe { (*pSrc).fg.__slate_bits_0.__get_isSubquery() }) as i32)
                        != (0 as i32)
                    {
                        if ((unsafe { (*pSrc).fg.__slate_bits_0.__get_viaCoroutine() }) as i32)
                            != (0 as i32)
                        {
                            let __v2291: *mut WhereLoop = pNew;
                            let __v2292: u32 = unsafe { (*__v2291).wsFlags };
                            let __v2293: u32 = __v2292 | ((33554432 as i32) as u32);
                            unsafe {
                                (*__v2291).wsFlags = __v2293;
                            }
                        }
                        // /* Do not set btree.pOrderBy for a recursive CTE. In this case
                        //         ** the ORDER BY clause does not determine the overall order that
                        //         ** rows are emitted from the CTE in.  */
                        if (unsafe {
                            (*unsafe { (*unsafe { (*pSrc).u4.pSubq }).pSelect }).selFlags
                        }) & ((8192 as i32) as u32)
                            == ((0 as i32) as u32)
                        {
                            unsafe {
                                (*pNew).u.btree.pOrderBy = unsafe {
                                    (*unsafe { (*unsafe { (*pSrc).u4.pSubq }).pSelect }).pOrderBy
                                };
                            }
                        }
                    } else {
                        if ((unsafe { (*pSrc).fg.__slate_bits_0.__get_fromExists() }) as i32)
                            != (0 as i32)
                        {
                            unsafe {
                                (*pNew).nOut = (0 as i32) as i16;
                            }
                        }
                    }
                    rc = whereLoopInsert(pBuilder, pNew);
                    unsafe {
                        (*pNew).nOut = rSize;
                    }
                    if rc != (0 as i32) {
                        break '__slate_break_1782;
                    }
                } else {
                    let mut m: u64 = 0 as u64;
                    if ((unsafe { (*pProbe).__slate_bits_0.__get_isCovering() }) as i32)
                        != (0 as i32)
                    {
                        m = ((0 as i32) as i64) as u64;
                        unsafe {
                            (*pNew).wsFlags = ((64 as i32) | (512 as i32)) as u32;
                        }
                    } else {
                        m = (unsafe { (*pSrc).colUsed }) & unsafe { (*pProbe).colNotIdxed };
                        if (unsafe { (*pProbe).pPartIdxWhere }) != std::ptr::null_mut::<Expr>() {
                            wherePartIdxExpr(
                                unsafe { (*pWInfo).pParse },
                                pProbe,
                                unsafe { (*pProbe).pPartIdxWhere },
                                std::ptr::addr_of_mut!(m),
                                0 as i32,
                                std::ptr::null_mut::<SrcItem>(),
                            );
                        }
                        unsafe {
                            (*pNew).wsFlags = (512 as i32) as u32;
                        }
                        if m == (((1 as i32) as i64) as u64)
                            << (((8 as u64).wrapping_mul(((8 as i32) as i64) as u64) as u32) as i32)
                                - (1 as i32)
                            || ((unsafe { (*pProbe).__slate_bits_0.__get_bHasExpr() }) as i32)
                                != (0 as i32)
                                && !(((unsafe { (*pProbe).__slate_bits_0.__get_bHasVCol() })
                                    as i32)
                                    != (0 as i32))
                                && m != (((0 as i32) as i64) as u64)
                        {
                            let mut isCov: u32 =
                                whereIsCoveringIndex(pWInfo, pProbe, unsafe { (*pSrc).iCursor });
                            if isCov == ((0 as i32) as u32) {
                                {}
                                0 as i32;
                            } else {
                                m = ((0 as i32) as i64) as u64;
                                let __v2294: *mut WhereLoop = pNew;
                                let __v2295: u32 = unsafe { (*__v2294).wsFlags };
                                let __v2296: u32 = __v2295 | isCov;
                                unsafe {
                                    (*__v2294).wsFlags = __v2296;
                                }
                                if isCov & ((64 as i32) as u32) != (0 as u32) {
                                    {}
                                } else {
                                    0 as i32;
                                    {}
                                }
                            }
                        } else {
                            let __v2297: bool;
                            if m == (((0 as i32) as i64) as u64) {
                                let __v2298: bool;
                                if (unsafe { (*pTab).tabFlags }) & ((128 as i32) as u32)
                                    == ((0 as i32) as u32)
                                    || (unsafe { (*pWInfo).pSelect })
                                        != std::ptr::null_mut::<Select>()
                                {
                                    __v2298 = true as bool;
                                } else {
                                    __v2298 =
                                        (unsafe { sqlite3FaultSim(700 as i32) }) != (0 as i32);
                                }
                                __v2297 = __v2298;
                            } else {
                                __v2297 = false as bool;
                            }
                            if __v2297 {
                                {}
                                unsafe {
                                    (*pNew).wsFlags = ((64 as i32) | (512 as i32)) as u32;
                                }
                            }
                        }
                    }
                    // /* Full scan via index */
                    if b != (0 as i32)
                        || !((unsafe { (*pTab).tabFlags }) & ((128 as i32) as u32)
                            == ((0 as i32) as u32))
                        || (unsafe { (*pProbe).pPartIdxWhere }) != std::ptr::null_mut::<Expr>()
                        || ((unsafe { (*pSrc).fg.__slate_bits_0.__get_isIndexedBy() }) as i32)
                            != (0 as i32)
                        || m == (((0 as i32) as i64) as u64)
                            && ((unsafe { (*pProbe).__slate_bits_0.__get_bUnordered() }) as i32)
                                == (0 as i32)
                            && ((unsafe { (*pProbe).szIdxRow }) as i32)
                                < ((unsafe { (*pTab).szTabRow }) as i32)
                            && (((unsafe { (*pWInfo).wctrlFlags }) as u32) as i32) & (4 as i32)
                                == (0 as i32)
                            && (unsafe { sqlite3Config.bUseCis }) != (0 as u8)
                            && (unsafe {
                                (*unsafe { (*unsafe { (*pWInfo).pParse }).db }).dbOptFlags
                            }) & ((32 as i32) as u32)
                                == ((0 as i32) as u32)
                    {
                        unsafe {
                            (*pNew).iSortIdx =
                                ((if b != (0 as i32) { iSortIdx } else { 0 as i32 }) as i8) as u8;
                        }
                        // /* The cost of visiting the index rows is N*K, where K is
                        //         ** between 1.1 and 3.0, depending on the relative sizes of the
                        //         ** index and table rows. */
                        unsafe {
                            (*pNew).rRun = ((rSize as i32)
                                + (1 as i32)
                                + (15 as i32) * ((unsafe { (*pProbe).szIdxRow }) as i32)
                                    / ((unsafe { (*pTab).szTabRow }) as i32))
                                as i16;
                        }
                        if m != (((0 as i32) as i64) as u64) {
                            // /* If this is a non-covering index scan, add in the cost of
                            //           ** doing table lookups.  The cost will be 3x the number of
                            //           ** lookups.  Take into account WHERE clause terms that can be
                            //           ** satisfied using just the index, and that do not require a
                            //           ** table lookup. */
                            // /* Base cost:  N*3 */
                            let mut nLookup: i16 = ((rSize as i32) + (16 as i32)) as i16;
                            let mut ii: i32 = 0 as i32;
                            let mut iCur: i32 = unsafe { (*pSrc).iCursor };
                            let mut pWC2: *mut WhereClause =
                                unsafe { std::ptr::addr_of_mut!((*pWInfo).sWC) };
                            ii = 0 as i32;
                            '__slate_break_1783: loop {
                                if !(ii < unsafe { (*pWC2).nTerm }) {
                                    break;
                                }
                                let mut pTerm: *mut WhereTerm =
                                    unsafe { unsafe { (*pWC2).a }.offset(ii as isize) };
                                if !((unsafe {
                                    sqlite3ExprCoveredByIndex(
                                        unsafe { (*pTerm).pExpr },
                                        iCur,
                                        pProbe,
                                    )
                                }) != (0 as i32))
                                {
                                    break '__slate_break_1783;
                                }
                                // /* pTerm can be evaluated using just the index.  So reduce
                                //             ** the expected number of table lookups accordingly */
                                if ((unsafe { (*pTerm).truthProb }) as i32) <= (0 as i32) {
                                    let __v2301: i16 = nLookup;
                                    let __v2302: i16 = ((__v2301 as i32)
                                        + ((unsafe { (*pTerm).truthProb }) as i32))
                                        as i16;
                                    nLookup = __v2302;
                                } else {
                                    let __v2303: i16 = nLookup;
                                    let __v2304: i16 = ((__v2303 as i32) - (1 as i32)) as i16;
                                    nLookup = __v2304;
                                    if (((unsafe { (*pTerm).eOperator }) as u32) as i32)
                                        & ((2 as i32) | (128 as i32))
                                        != (0 as i32)
                                    {
                                        let __v2305: i16 = nLookup;
                                        let __v2306: i16 = ((__v2305 as i32) - (19 as i32)) as i16;
                                        nLookup = __v2306;
                                    }
                                }
                                let __v2299: i32 = ii;
                                let __v2300: i32 = __v2299 + (1 as i32);
                                ii = __v2300;
                            }
                            unsafe {
                                (*pNew).rRun =
                                    unsafe { sqlite3LogEstAdd(unsafe { (*pNew).rRun }, nLookup) };
                            }
                        }
                        {}
                        whereLoopOutputAdjust(pWC, pNew, rSize);
                        if (((unsafe { (*pSrc).fg.jointype }) as u32) as i32) & (16 as i32)
                            != (0 as i32)
                            && (unsafe { (*pProbe).aColExpr }) != std::ptr::null_mut::<ExprList>()
                        {
                            // /* Do not do an SCAN of a index-on-expression in a RIGHT JOIN
                            //           ** because the cursor used to access the index might not be
                            //           ** positioned to the correct row during the right-join no-match
                            //           ** loop. */
                        } else {
                            if ((unsafe { (*pSrc).fg.__slate_bits_0.__get_fromExists() }) as i32)
                                != (0 as i32)
                            {
                                unsafe {
                                    (*pNew).nOut = (0 as i32) as i16;
                                }
                            }
                            rc = whereLoopInsert(pBuilder, pNew);
                        }
                        unsafe {
                            (*pNew).nOut = rSize;
                        }
                        if rc != (0 as i32) {
                            break '__slate_break_1782;
                        }
                    }
                }
                unsafe {
                    (*pBuilder).bldFlags1 = ((0 as i32) as i8) as u8;
                }
                rc = whereLoopAddBtreeIndex(pBuilder, pSrc, pProbe, (0 as i32) as i16);
                if (((unsafe { (*pBuilder).bldFlags1 }) as u32) as i32) == (1 as i32) {
                    // /* If a non-unique index is used, or if a prefix of the key for
                    //       ** unique index is used (making the index functionally non-unique)
                    //       ** then the sqlite_stat1 data becomes important for scoring the
                    //       ** plan */
                    let __v2307: *mut Table = pTab;
                    let __v2308: u32 = unsafe { (*__v2307).tabFlags };
                    let __v2309: u32 = __v2308 | ((256 as i32) as u32);
                    unsafe {
                        (*__v2307).tabFlags = __v2309;
                    }
                }
            }
        }
        pProbe =
            if ((unsafe { (*pSrc).fg.__slate_bits_0.__get_isIndexedBy() }) as i32) != (0 as i32) {
                std::ptr::null_mut::<Index>()
            } else {
                unsafe { (*pProbe).pNext }
            };
        let __v2288: i32 = iSortIdx;
        let __v2289: i32 = __v2288 + (1 as i32);
        iSortIdx = __v2289;
    }
    return rc;
}

// /* WHERE clause information */
// /* Extra prerequisites for using this table */
// /*
// ** Return true if pTerm is a virtual table LIMIT or OFFSET term.
// */
fn isLimitTerm(mut pTerm: *mut WhereTerm) -> i32 {
    0 as i32;
    return ((((unsafe { (*pTerm).eMatchOp }) as u32) as i32) >= (73 as i32)
        && (((unsafe { (*pTerm).eMatchOp }) as u32) as i32) <= (74 as i32)) as i32;
}

// /*
// ** Return true if the first nCons constraints in the pUsage array are
// ** marked as in-use (have argvIndex>0). False otherwise.
// */
fn allConstraintsUsed(mut aUsage: *mut sqlite3_index_constraint_usage, mut nCons: i32) -> i32 {
    let mut ii: i32 = 0 as i32;
    ii = 0 as i32;
    '__slate_break_1784: loop {
        if !(ii < nCons) {
            break;
        }
        if (unsafe { (*unsafe { aUsage.offset(ii as isize) }).argvIndex }) <= (0 as i32) {
            return 0 as i32;
        }
        let __v2310: i32 = ii;
        let __v2311: i32 = __v2310 + (1 as i32);
        ii = __v2311;
    }
    return 1 as i32;
}

// /*
// ** Argument pIdxInfo is already populated with all constraints that may
// ** be used by the virtual table identified by pBuilder->pNew->iTab. This
// ** function marks a subset of those constraints usable, invokes the
// ** xBestIndex method and adds the returned plan to pBuilder.
// **
// ** A constraint is marked usable if:
// **
// **   * Argument mUsable indicates that its prerequisites are available, and
// **
// **   * It is not one of the operators specified in the mExclude mask passed
// **     as the fourth argument (which in practice is either WO_IN or 0).
// **
// ** Argument mPrereq is a mask of tables that must be scanned before the
// ** virtual table in question. These are added to the plans prerequisites
// ** before it is added to pBuilder.
// **
// ** Output parameter *pbIn is set to true if the plan added to pBuilder
// ** uses one or more WO_IN terms, or false otherwise.
// */
fn whereLoopAddVirtualOne(
    mut pBuilder: *mut WhereLoopBuilder,
    mut mPrereq: u64,
    mut mUsable: u64,
    mut mExclude: u16,
    mut pIdxInfo: *mut sqlite3_index_info,
    mut mNoOmit: u16,
    mut pbIn: *mut i32,
    mut pbRetryLimit: *mut i32,
) -> i32 {
    let mut pWC: *mut WhereClause = unsafe { (*pBuilder).pWC };
    let mut pHidden: *mut HiddenIndexInfo =
        (unsafe { pIdxInfo.offset((1 as i32) as isize) }) as *mut HiddenIndexInfo;
    let mut pIdxCons: *mut sqlite3_index_constraint = unsafe { std::mem::zeroed() };
    let mut pUsage: *mut sqlite3_index_constraint_usage = unsafe { (*pIdxInfo).aConstraintUsage };
    let mut i: i32 = 0 as i32;
    let mut mxTerm: i32 = 0 as i32;
    let mut rc: i32 = 0 as i32;
    let mut pNew: *mut WhereLoop = unsafe { (*pBuilder).pNew };
    let mut pParse: *mut Parse = unsafe { (*unsafe { (*pBuilder).pWInfo }).pParse };
    let mut pSrc: *mut SrcItem = unsafe {
        unsafe {
            std::ptr::addr_of_mut!((*unsafe { (*unsafe { (*pBuilder).pWInfo }).pTabList }).a)
                as *mut SrcItem
        }
        .offset((((unsafe { (*pNew).iTab }) as u32) as i32) as isize)
    };
    let mut nConstraint: i32 = unsafe { (*pIdxInfo).nConstraint };
    0 as i32;
    unsafe {
        *pbIn = 0 as i32;
    }
    unsafe {
        (*pNew).prereq = mPrereq;
    }
    // /* Set the usable flag on the subset of constraints identified by
    //   ** arguments mUsable and mExclude. */
    pIdxCons = unsafe { *unsafe { std::ptr::addr_of_mut!((*pIdxInfo).aConstraint) } };
    i = 0 as i32;
    '__slate_break_1785: while i < nConstraint {
        let mut pTerm: *mut WhereTerm =
            termFromWhereClause(pWC, unsafe { (*pIdxCons).iTermOffset });
        unsafe {
            (*pIdxCons).usable = ((0 as i32) as i8) as u8;
        }
        let __v2316: bool;
        if (unsafe { (*pTerm).prereqRight }) & mUsable == unsafe { (*pTerm).prereqRight }
            && (((unsafe { (*pTerm).eOperator }) as u32) as i32) & ((mExclude as u32) as i32)
                == (0 as i32)
        {
            let __v2317: bool;
            if pbRetryLimit != std::ptr::null_mut::<i32>() {
                __v2317 = true as bool;
            } else {
                __v2317 = !(isLimitTerm(pTerm) != (0 as i32));
            }
            __v2316 = __v2317;
        } else {
            __v2316 = false as bool;
        }
        if __v2316 {
            unsafe {
                (*pIdxCons).usable = ((1 as i32) as i8) as u8;
            }
        }
        let __v2312: i32 = i;
        let __v2313: i32 = __v2312 + (1 as i32);
        i = __v2313;
        let __v2314: *mut sqlite3_index_constraint = pIdxCons;
        let __v2315: *mut sqlite3_index_constraint = unsafe { __v2314.offset((1 as i32) as isize) };
        pIdxCons = __v2315;
    }
    // /* Initialize the output fields of the sqlite3_index_info structure */
    unsafe {
        memset(
            pUsage as *mut (),
            0 as i32,
            (8 as u64).wrapping_mul((nConstraint as i64) as u64),
        )
    };
    0 as i32;
    unsafe {
        (*pIdxInfo).idxStr = std::ptr::null_mut::<i8>();
    }
    unsafe {
        (*pIdxInfo).idxNum = 0 as i32;
    }
    unsafe {
        (*pIdxInfo).orderByConsumed = 0 as i32;
    }
    unsafe {
        (*pIdxInfo).estimatedCost = 1e99f64 / ((2 as i32) as f64);
    }
    unsafe {
        (*pIdxInfo).estimatedRows = (25 as i32) as i64;
    }
    unsafe {
        (*pIdxInfo).idxFlags = 0 as i32;
    }
    unsafe {
        (*pHidden).mHandleIn = (0 as i32) as u32;
    }
    // /* Invoke the virtual table xBestIndex() method */
    rc = vtabBestIndex(pParse, unsafe { (*pSrc).pSTab }, pIdxInfo);
    if rc != (0 as i32) {
        if rc == (19 as i32) {
            // /* If the xBestIndex method returns SQLITE_CONSTRAINT, that means
            //       ** that the particular combination of parameters provided is unusable.
            //       ** Make no entries in the loop table.
            //       */
            {}
            freeIdxStr(pIdxInfo);
            return 0 as i32;
        }
        return rc;
    }
    mxTerm = -(1 as i32);
    0 as i32;
    unsafe {
        memset(
            (unsafe { (*pNew).aLTerm }) as *mut (),
            0 as i32,
            (8 as u64).wrapping_mul((nConstraint as i64) as u64),
        )
    };
    unsafe {
        memset(
            (unsafe { std::ptr::addr_of_mut!((*pNew).u.vtab) }) as *mut (),
            0 as i32,
            24 as u64,
        )
    };
    pIdxCons = unsafe { *unsafe { std::ptr::addr_of_mut!((*pIdxInfo).aConstraint) } };
    i = 0 as i32;
    '__slate_break_1786: while i < nConstraint {
        let mut iTerm: i32 = 0 as i32;
        let __v2322: i32 =
            (unsafe { (*unsafe { pUsage.offset(i as isize) }).argvIndex }) - (1 as i32);
        iTerm = __v2322;
        if __v2322 >= (0 as i32) {
            let mut pTerm: *mut WhereTerm = unsafe { std::mem::zeroed() };
            let mut j: i32 = unsafe { (*pIdxCons).iTermOffset };
            let __v2323: bool;
            if iTerm >= nConstraint || j < (0 as i32) {
                __v2323 = true as bool;
            } else {
                let __v2324: *mut WhereTerm = termFromWhereClause(pWC, j);
                pTerm = __v2324;
                __v2323 = __v2324 == std::ptr::null_mut::<WhereTerm>();
            }
            if __v2323
                || (unsafe { *unsafe { unsafe { (*pNew).aLTerm }.offset(iTerm as isize) } })
                    != std::ptr::null_mut::<WhereTerm>()
                || (((unsafe { (*pIdxCons).usable }) as u32) as i32) == (0 as i32)
            {
                unsafe {
                    sqlite3ErrorMsg(
                        pParse,
                        (b"%s.xBestIndex malfunction\0".as_ptr() as *mut i8) as *const i8,
                        unsafe { (*unsafe { (*pSrc).pSTab }).zName },
                    )
                };
                freeIdxStr(pIdxInfo);
                return 1 as i32;
            }
            {}
            {}
            {}
            let __v2325: *mut WhereLoop = pNew;
            let __v2326: u64 = unsafe { (*__v2325).prereq };
            let __v2327: u64 = __v2326 | unsafe { (*pTerm).prereqRight };
            unsafe {
                (*__v2325).prereq = __v2327;
            }
            0 as i32;
            unsafe {
                *unsafe { unsafe { (*pNew).aLTerm }.offset(iTerm as isize) } = pTerm;
            }
            if iTerm > mxTerm {
                mxTerm = iTerm;
            }
            {}
            {}
            if (unsafe { (*unsafe { pUsage.offset(i as isize) }).omit }) != (0 as u8) {
                if i < (16 as i32) && (1 as i32) << i & ((mNoOmit as u32) as i32) == (0 as i32) {
                    {}
                    let __v2328: *mut WhereLoop = pNew;
                    let __v2329: u16 = unsafe { (*__v2328).u.vtab.omitMask };
                    let __v2330: u16 =
                        ((((__v2329 as u32) as i32) | (1 as i32) << iTerm) as i16) as u16;
                    unsafe {
                        (*__v2328).u.vtab.omitMask = __v2330;
                    }
                } else {
                    {}
                }
                if (((unsafe { (*pTerm).eMatchOp }) as u32) as i32) == (74 as i32) {
                    unsafe {
                        (*pNew)
                            .u
                            .vtab
                            .__slate_bits_0
                            .__set_bOmitOffset((1 as i32) as u32);
                    }
                }
            }
            if (if i <= (31 as i32) {
                ((1 as i32) as u32) << i
            } else {
                (0 as i32) as u32
            }) & unsafe { (*pHidden).mHandleIn }
                != (0 as u32)
            {
                let __v2331: *mut WhereLoop = pNew;
                let __v2332: u32 = unsafe { (*__v2331).u.vtab.mHandleIn };
                let __v2333: u32 = __v2332 | ((1 as i32) as u32) << iTerm;
                unsafe {
                    (*__v2331).u.vtab.mHandleIn = __v2333;
                }
            } else {
                if (((unsafe { (*pTerm).eOperator }) as u32) as i32) & (1 as i32) != (0 as i32) {
                    // /* A virtual table that is constrained by an IN clause may not
                    //         ** consume the ORDER BY clause because (1) the order of IN terms
                    //         ** is not necessarily related to the order of output terms and
                    //         ** (2) Multiple outputs from a single IN value will not merge
                    //         ** together.  */
                    unsafe {
                        (*pIdxInfo).orderByConsumed = 0 as i32;
                    }
                    let __v2334: *mut sqlite3_index_info = pIdxInfo;
                    let __v2335: i32 = unsafe { (*__v2334).idxFlags };
                    let __v2336: i32 = __v2335 & !(1 as i32);
                    unsafe {
                        (*__v2334).idxFlags = __v2336;
                    }
                    unsafe {
                        *pbIn = 1 as i32;
                    }
                    0 as i32;
                }
            }
            // /* Unless pbRetryLimit is non-NULL, there should be no LIMIT/OFFSET
            //       ** terms. And if there are any, they should follow all other terms. */
            0 as i32;
            0 as i32;
            0 as i32;
            let __v2337: bool;
            if isLimitTerm(pTerm) != (0 as i32) {
                let __v2338: bool;
                if (unsafe { *pbIn }) != (0 as i32) {
                    __v2338 = true as bool;
                } else {
                    __v2338 = !(allConstraintsUsed(pUsage, i) != (0 as i32));
                }
                __v2337 = __v2338;
            } else {
                __v2337 = false as bool;
            }
            if __v2337 {
                // /* If there is an IN(...) term handled as an == (separate call to
                //         ** xFilter for each value on the RHS of the IN) and a LIMIT or
                //         ** OFFSET term handled as well, the plan is unusable. Similarly,
                //         ** if there is a LIMIT/OFFSET and there are other unused terms,
                //         ** the plan cannot be used. In these cases set variable *pbRetryLimit
                //         ** to true to tell the caller to retry with LIMIT and OFFSET
                //         ** disabled. */
                freeIdxStr(pIdxInfo);
                unsafe {
                    *pbRetryLimit = 1 as i32;
                }
                return 0 as i32;
            }
        }
        let __v2318: i32 = i;
        let __v2319: i32 = __v2318 + (1 as i32);
        i = __v2319;
        let __v2320: *mut sqlite3_index_constraint = pIdxCons;
        let __v2321: *mut sqlite3_index_constraint = unsafe { __v2320.offset((1 as i32) as isize) };
        pIdxCons = __v2321;
    }
    unsafe {
        (*pNew).nLTerm = ((mxTerm + (1 as i32)) as i16) as u16;
    }
    i = 0 as i32;
    '__slate_break_1788: loop {
        if !(i <= mxTerm) {
            break;
        }
        if (unsafe { *unsafe { unsafe { (*pNew).aLTerm }.offset(i as isize) } })
            == std::ptr::null_mut::<WhereTerm>()
        {
            // /* The non-zero argvIdx values must be contiguous.  Raise an
            //       ** error if they are not */
            unsafe {
                sqlite3ErrorMsg(
                    pParse,
                    (b"%s.xBestIndex malfunction\0".as_ptr() as *mut i8) as *const i8,
                    unsafe { (*unsafe { (*pSrc).pSTab }).zName },
                )
            };
            freeIdxStr(pIdxInfo);
            return 1 as i32;
        }
        let __v2339: i32 = i;
        let __v2340: i32 = __v2339 + (1 as i32);
        i = __v2340;
    }
    0 as i32;
    unsafe {
        (*pNew).u.vtab.idxNum = unsafe { (*pIdxInfo).idxNum };
    }
    unsafe {
        (*pNew)
            .u
            .vtab
            .__slate_bits_0
            .__set_needFree((unsafe { (*pIdxInfo).needToFreeIdxStr }) as u32);
    }
    unsafe {
        (*pIdxInfo).needToFreeIdxStr = 0 as i32;
    }
    unsafe {
        (*pNew).u.vtab.idxStr = unsafe { (*pIdxInfo).idxStr };
    }
    unsafe {
        (*pNew).u.vtab.isOrdered = (if (unsafe { (*pIdxInfo).orderByConsumed }) != (0 as i32) {
            unsafe { (*pIdxInfo).nOrderBy }
        } else {
            0 as i32
        }) as i8;
    }
    unsafe {
        (*pNew).u.vtab.__slate_bits_0.__set_bIdxNumHex(
            ((unsafe { (*pIdxInfo).idxFlags }) & (2 as i32) != (0 as i32)) as u32,
        );
    }
    unsafe {
        (*pNew).rSetup = (0 as i32) as i16;
    }
    unsafe {
        (*pNew).rRun = unsafe { sqlite3LogEstFromDouble(unsafe { (*pIdxInfo).estimatedCost }) };
    }
    unsafe {
        (*pNew).nOut = unsafe { sqlite3LogEst((unsafe { (*pIdxInfo).estimatedRows }) as u64) };
    }
    // /* Set the WHERE_ONEROW flag if the xBestIndex() method indicated
    //   ** that the scan will visit at most one row. Clear it otherwise. */
    if (unsafe { (*pIdxInfo).idxFlags }) & (1 as i32) != (0 as i32) {
        let __v2341: *mut WhereLoop = pNew;
        let __v2342: u32 = unsafe { (*__v2341).wsFlags };
        let __v2343: u32 = __v2342 | ((4096 as i32) as u32);
        unsafe {
            (*__v2341).wsFlags = __v2343;
        }
    } else {
        let __v2344: *mut WhereLoop = pNew;
        let __v2345: u32 = unsafe { (*__v2344).wsFlags };
        let __v2346: u32 = __v2345 & (!(4096 as i32) as u32);
        unsafe {
            (*__v2344).wsFlags = __v2346;
        }
    }
    rc = whereLoopInsert(pBuilder, pNew);
    if ((unsafe { (*pNew).u.vtab.__slate_bits_0.__get_needFree() }) as i32) != (0 as i32) {
        unsafe { sqlite3_free((unsafe { (*pNew).u.vtab.idxStr }) as *mut ()) };
        unsafe {
            (*pNew)
                .u
                .vtab
                .__slate_bits_0
                .__set_needFree((0 as i32) as u32);
        }
    }
    {}
    return rc;
}

// /*
// ** Add all WhereLoop objects for a table of the join identified by
// ** pBuilder->pNew->iTab.  That table is guaranteed to be a virtual table.
// **
// ** If there are no LEFT or CROSS JOIN joins in the query, both mPrereq and
// ** mUnusable are set to 0. Otherwise, mPrereq is a mask of all FROM clause
// ** entries that occur before the virtual table in the FROM clause and are
// ** separated from it by at least one LEFT or CROSS JOIN. Similarly, the
// ** mUnusable mask contains all FROM clause entries that occur after the
// ** virtual table and are separated from it by at least one LEFT or
// ** CROSS JOIN.
// **
// ** For example, if the query were:
// **
// **   ... FROM t1, t2 LEFT JOIN t3, t4, vt CROSS JOIN t5, t6;
// **
// ** then mPrereq corresponds to (t1, t2) and mUnusable to (t5, t6).
// **
// ** All the tables in mPrereq must be scanned before the current virtual
// ** table. So any terms for which all prerequisites are satisfied by
// ** mPrereq may be specified as "usable" in all calls to xBestIndex.
// ** Conversely, all tables in mUnusable must be scanned after the current
// ** virtual table, so any terms for which the prerequisites overlap with
// ** mUnusable should always be configured as "not-usable" for xBestIndex.
// */
fn whereLoopAddVirtual(
    mut pBuilder: *mut WhereLoopBuilder,
    mut mPrereq: u64,
    mut mUnusable: u64,
) -> i32 {
    // /* Return code */
    let mut rc: i32 = 0 as i32;
    // /* WHERE analysis context */
    let mut pWInfo: *mut WhereInfo = unsafe { std::mem::zeroed() };
    // /* The parsing context */
    let mut pParse: *mut Parse = unsafe { std::mem::zeroed() };
    // /* The WHERE clause */
    let mut pWC: *mut WhereClause = unsafe { std::mem::zeroed() };
    // /* The FROM clause term to search */
    let mut pSrc: *mut SrcItem = unsafe { std::mem::zeroed() };
    // /* Object to pass to xBestIndex() */
    let mut p: *mut sqlite3_index_info = unsafe { std::mem::zeroed() };
    // /* Number of constraints in p */
    let mut nConstraint: i32 = 0 as i32;
    // /* True if plan uses IN(...) operator */
    let mut bIn: i32 = 0 as i32;
    let mut pNew: *mut WhereLoop = unsafe { std::mem::zeroed() };
    // /* Tables used by best possible plan */
    let mut mBest: u64 = 0 as u64;
    let mut mNoOmit: u16 = 0 as u16;
    // /* True to retry with LIMIT/OFFSET disabled */
    let mut bRetry: i32 = 0 as i32;
    0 as i32;
    pWInfo = unsafe { (*pBuilder).pWInfo };
    pParse = unsafe { (*pWInfo).pParse };
    pWC = unsafe { (*pBuilder).pWC };
    pNew = unsafe { (*pBuilder).pNew };
    pSrc = unsafe {
        unsafe { std::ptr::addr_of_mut!((*unsafe { (*pWInfo).pTabList }).a) as *mut SrcItem }
            .offset((((unsafe { (*pNew).iTab }) as u32) as i32) as isize)
    };
    0 as i32;
    p = allocateIndexInfo(
        pWInfo,
        pWC,
        mUnusable,
        pSrc,
        std::ptr::addr_of_mut!(mNoOmit),
    );
    if p == std::ptr::null_mut::<sqlite3_index_info>() {
        return 7 as i32;
    }
    unsafe {
        (*pNew).rSetup = (0 as i32) as i16;
    }
    unsafe {
        (*pNew).wsFlags = (1024 as i32) as u32;
    }
    unsafe {
        (*pNew).nLTerm = ((0 as i32) as i16) as u16;
    }
    unsafe {
        (*pNew)
            .u
            .vtab
            .__slate_bits_0
            .__set_needFree((0 as i32) as u32);
    }
    nConstraint = unsafe { (*p).nConstraint };
    if whereLoopResize(unsafe { (*pParse).db }, pNew, nConstraint) != (0 as i32) {
        freeIndexInfo(unsafe { (*pParse).db }, p);
        return 7 as i32;
    }
    // /* First call xBestIndex() with all constraints usable. */
    {}
    {}
    rc = whereLoopAddVirtualOne(
        pBuilder,
        mPrereq,
        (-(1 as i32) as i64) as u64,
        ((0 as i32) as i16) as u16,
        p,
        mNoOmit,
        std::ptr::addr_of_mut!(bIn),
        std::ptr::addr_of_mut!(bRetry),
    );
    if bRetry != (0 as i32) {
        0 as i32;
        rc = whereLoopAddVirtualOne(
            pBuilder,
            mPrereq,
            (-(1 as i32) as i64) as u64,
            ((0 as i32) as i16) as u16,
            p,
            mNoOmit,
            std::ptr::addr_of_mut!(bIn),
            std::ptr::null_mut::<i32>(),
        );
    }
    // /* If the call to xBestIndex() with all terms enabled produced a plan
    //   ** that does not require any source tables (IOW: a plan with mBest==0)
    //   ** and does not use an IN(...) operator, then there is no point in making
    //   ** any further calls to xBestIndex() since they will all return the same
    //   ** result (if the xBestIndex() implementation is sane). */
    let __v2347: bool;
    if rc == (0 as i32) {
        let __v2348: u64 = (unsafe { (*pNew).prereq }) & !mPrereq;
        mBest = __v2348;
        __v2347 = __v2348 != (((0 as i32) as i64) as u64) || bIn != (0 as i32);
    } else {
        __v2347 = false as bool;
    }
    if __v2347 {
        // /* True if a plan with no prereqs seen */
        let mut seenZero: i32 = 0 as i32;
        // /* Plan with no prereqs and no IN(...) seen */
        let mut seenZeroNoIN: i32 = 0 as i32;
        let mut mPrev: u64 = ((0 as i32) as i64) as u64;
        let mut mBestNoIn: u64 = ((0 as i32) as i64) as u64;
        // /* If the plan produced by the earlier call uses an IN(...) term, call
        //     ** xBestIndex again, this time with IN(...) terms disabled. */
        if bIn != (0 as i32) {
            {}
            rc = whereLoopAddVirtualOne(
                pBuilder,
                mPrereq,
                (-(1 as i32) as i64) as u64,
                ((1 as i32) as i16) as u16,
                p,
                mNoOmit,
                std::ptr::addr_of_mut!(bIn),
                std::ptr::null_mut::<i32>(),
            );
            0 as i32;
            mBestNoIn = (unsafe { (*pNew).prereq }) & !mPrereq;
            if mBestNoIn == (((0 as i32) as i64) as u64) {
                seenZero = 1 as i32;
                seenZeroNoIN = 1 as i32;
            }
        }
        // /* Call xBestIndex once for each distinct value of (prereqRight & ~mPrereq)
        //     ** in the set of terms that apply to the current virtual table.  */
        '__slate_break_1792: while rc == (0 as i32) {
            let mut i: i32 = 0 as i32;
            let mut mNext: u64 = (-(1 as i32) as i64) as u64;
            0 as i32;
            i = 0 as i32;
            '__slate_break_1793: loop {
                if !(i < nConstraint) {
                    break;
                }
                let mut iTerm: i32 = unsafe {
                    (*unsafe { unsafe { (*p).aConstraint }.offset(i as isize) }).iTermOffset
                };
                let mut mThis: u64 =
                    (unsafe { (*termFromWhereClause(pWC, iTerm)).prereqRight }) & !mPrereq;
                if mThis > mPrev && mThis < mNext {
                    mNext = mThis;
                }
                let __v2349: i32 = i;
                let __v2350: i32 = __v2349 + (1 as i32);
                i = __v2350;
            }
            mPrev = mNext;
            if mNext == ((-(1 as i32) as i64) as u64) {
                break '__slate_break_1792;
            }
            if mNext == mBest || mNext == mBestNoIn {
            } else {
                {}
                rc = whereLoopAddVirtualOne(
                    pBuilder,
                    mPrereq,
                    mNext | mPrereq,
                    ((0 as i32) as i16) as u16,
                    p,
                    mNoOmit,
                    std::ptr::addr_of_mut!(bIn),
                    std::ptr::null_mut::<i32>(),
                );
                if (unsafe { (*pNew).prereq }) == mPrereq {
                    seenZero = 1 as i32;
                    if bIn == (0 as i32) {
                        seenZeroNoIN = 1 as i32;
                    }
                }
            }
        }
        // /* If the calls to xBestIndex() in the above loop did not find a plan
        //     ** that requires no source tables at all (i.e. one guaranteed to be
        //     ** usable), make a call here with all source tables disabled */
        if rc == (0 as i32) && seenZero == (0 as i32) {
            {}
            rc = whereLoopAddVirtualOne(
                pBuilder,
                mPrereq,
                mPrereq,
                ((0 as i32) as i16) as u16,
                p,
                mNoOmit,
                std::ptr::addr_of_mut!(bIn),
                std::ptr::null_mut::<i32>(),
            );
            if bIn == (0 as i32) {
                seenZeroNoIN = 1 as i32;
            }
        }
        // /* If the calls to xBestIndex() have so far failed to find a plan
        //     ** that requires no source tables at all and does not use an IN(...)
        //     ** operator, make a final call to obtain one here.  */
        if rc == (0 as i32) && seenZeroNoIN == (0 as i32) {
            {}
            rc = whereLoopAddVirtualOne(
                pBuilder,
                mPrereq,
                mPrereq,
                ((1 as i32) as i16) as u16,
                p,
                mNoOmit,
                std::ptr::addr_of_mut!(bIn),
                std::ptr::null_mut::<i32>(),
            );
        }
    }
    freeIndexInfo(unsafe { (*pParse).db }, p);
    {}
    return rc;
}

// /* WHERE clause information */
// /* Tables that must be scanned before this one */
// /* Tables that must be scanned after this one */
// /* SQLITE_OMIT_VIRTUALTABLE */
// /*
// ** Add WhereLoop entries to handle OR terms.  This works for either
// ** btrees or virtual tables.
// */
fn whereLoopAddOr(
    mut pBuilder: *mut WhereLoopBuilder,
    mut mPrereq: u64,
    mut mUnusable: u64,
) -> i32 {
    let mut pWInfo: *mut WhereInfo = unsafe { (*pBuilder).pWInfo };
    let mut pWC: *mut WhereClause = unsafe { std::mem::zeroed() };
    let mut pNew: *mut WhereLoop = unsafe { std::mem::zeroed() };
    let mut pTerm: *mut WhereTerm = unsafe { std::mem::zeroed() };
    let mut pWCEnd: *mut WhereTerm = unsafe { std::mem::zeroed() };
    let mut rc: i32 = 0 as i32;
    let mut iCur: i32 = 0 as i32;
    let mut tempWC: WhereClause = unsafe { std::mem::zeroed() };
    let mut sSubBuild: WhereLoopBuilder = unsafe { std::mem::zeroed() };
    let mut sSum: WhereOrSet = unsafe { std::mem::zeroed() };
    let mut sCur: WhereOrSet = unsafe { std::mem::zeroed() };
    let mut pItem: *mut SrcItem = unsafe { std::mem::zeroed() };
    pWC = unsafe { (*pBuilder).pWC };
    pWCEnd = unsafe { unsafe { (*pWC).a }.offset((unsafe { (*pWC).nTerm }) as isize) };
    pNew = unsafe { (*pBuilder).pNew };
    unsafe { memset(std::ptr::addr_of_mut!(sSum) as *mut (), 0 as i32, 56 as u64) };
    pItem = unsafe {
        unsafe { std::ptr::addr_of_mut!((*unsafe { (*pWInfo).pTabList }).a) as *mut SrcItem }
            .offset((((unsafe { (*pNew).iTab }) as u32) as i32) as isize)
    };
    iCur = unsafe { (*pItem).iCursor };
    // /* The multi-index OR optimization does not work for RIGHT and FULL JOIN */
    if (((unsafe { (*pItem).fg.jointype }) as u32) as i32) & (16 as i32) != (0 as i32) {
        return 0 as i32;
    }
    pTerm = unsafe { (*pWC).a };
    '__slate_break_1794: while pTerm < pWCEnd && rc == (0 as i32) {
        if (((unsafe { (*pTerm).eOperator }) as u32) as i32) & (512 as i32) != (0 as i32)
            && (unsafe { (*unsafe { (*pTerm).u.pOrInfo }).indexable }) & unsafe { (*pNew).maskSelf }
                != (((0 as i32) as i64) as u64)
        {
            let mut pOrWC: *mut WhereClause =
                unsafe { std::ptr::addr_of_mut!((*unsafe { (*pTerm).u.pOrInfo }).wc) };
            let mut pOrWCEnd: *mut WhereTerm =
                unsafe { unsafe { (*pOrWC).a }.offset((unsafe { (*pOrWC).nTerm }) as isize) };
            let mut pOrTerm: *mut WhereTerm = unsafe { std::mem::zeroed() };
            let mut once: i32 = 1 as i32;
            let mut i: i32 = 0 as i32;
            let mut j: i32 = 0 as i32;
            sSubBuild = unsafe { *pBuilder };
            sSubBuild.pOrSet = std::ptr::addr_of_mut!(sCur);
            {}
            pOrTerm = unsafe { (*pOrWC).a };
            '__slate_break_1795: while pOrTerm < pOrWCEnd {
                '__slate_continue_1795: {
                    if (((unsafe { (*pOrTerm).eOperator }) as u32) as i32) & (1024 as i32)
                        != (0 as i32)
                    {
                        sSubBuild.pWC = unsafe {
                            std::ptr::addr_of_mut!((*unsafe { (*pOrTerm).u.pAndInfo }).wc)
                        };
                    } else {
                        if (unsafe { (*pOrTerm).leftCursor }) == iCur {
                            tempWC.pWInfo = unsafe { (*pWC).pWInfo };
                            tempWC.pOuter = pWC;
                            tempWC.op = ((44 as i32) as i8) as u8;
                            tempWC.nTerm = 1 as i32;
                            tempWC.nBase = 1 as i32;
                            tempWC.a = pOrTerm;
                            sSubBuild.pWC = std::ptr::addr_of_mut!(tempWC);
                        } else {
                            break '__slate_continue_1795;
                        }
                    }
                    sCur.n = ((0 as i32) as i16) as u16;
                    if (((unsafe { (*unsafe { (*pItem).pSTab }).eTabType }) as u32) as i32)
                        == (1 as i32)
                    {
                        rc = whereLoopAddVirtual(
                            std::ptr::addr_of_mut!(sSubBuild),
                            mPrereq,
                            mUnusable,
                        );
                    } else {
                        rc = whereLoopAddBtree(std::ptr::addr_of_mut!(sSubBuild), mPrereq);
                    }
                    if rc == (0 as i32) {
                        rc = whereLoopAddOr(std::ptr::addr_of_mut!(sSubBuild), mPrereq, mUnusable);
                    }
                    {}
                    {}
                    if ((sCur.n as u32) as i32) == (0 as i32) {
                        sSum.n = ((0 as i32) as i16) as u16;
                        break '__slate_break_1795;
                    } else {
                        if once != (0 as i32) {
                            whereOrMove(std::ptr::addr_of_mut!(sSum), std::ptr::addr_of_mut!(sCur));
                            once = 0 as i32;
                        } else {
                            let mut sPrev: WhereOrSet = unsafe { std::mem::zeroed() };
                            whereOrMove(
                                std::ptr::addr_of_mut!(sPrev),
                                std::ptr::addr_of_mut!(sSum),
                            );
                            sSum.n = ((0 as i32) as i16) as u16;
                            i = 0 as i32;
                            '__slate_break_1796: loop {
                                if !(i < ((sPrev.n as u32) as i32)) {
                                    break;
                                }
                                j = 0 as i32;
                                '__slate_break_1797: loop {
                                    if !(j < ((sCur.n as u32) as i32)) {
                                        break;
                                    }
                                    whereOrInsert(
                                        std::ptr::addr_of_mut!(sSum),
                                        (unsafe {
                                            (*unsafe {
                                                (sPrev.a.as_mut_ptr() as *mut WhereOrCost)
                                                    .offset(i as isize)
                                            })
                                            .prereq
                                        }) | unsafe {
                                            (*unsafe {
                                                (sCur.a.as_mut_ptr() as *mut WhereOrCost)
                                                    .offset(j as isize)
                                            })
                                            .prereq
                                        },
                                        unsafe {
                                            sqlite3LogEstAdd(
                                                unsafe {
                                                    (*unsafe {
                                                        (sPrev.a.as_mut_ptr() as *mut WhereOrCost)
                                                            .offset(i as isize)
                                                    })
                                                    .rRun
                                                },
                                                unsafe {
                                                    (*unsafe {
                                                        (sCur.a.as_mut_ptr() as *mut WhereOrCost)
                                                            .offset(j as isize)
                                                    })
                                                    .rRun
                                                },
                                            )
                                        },
                                        unsafe {
                                            sqlite3LogEstAdd(
                                                unsafe {
                                                    (*unsafe {
                                                        (sPrev.a.as_mut_ptr() as *mut WhereOrCost)
                                                            .offset(i as isize)
                                                    })
                                                    .nOut
                                                },
                                                unsafe {
                                                    (*unsafe {
                                                        (sCur.a.as_mut_ptr() as *mut WhereOrCost)
                                                            .offset(j as isize)
                                                    })
                                                    .nOut
                                                },
                                            )
                                        },
                                    );
                                    let __v2357: i32 = j;
                                    let __v2358: i32 = __v2357 + (1 as i32);
                                    j = __v2358;
                                }
                                let __v2355: i32 = i;
                                let __v2356: i32 = __v2355 + (1 as i32);
                                i = __v2356;
                            }
                        }
                    }
                }
                let __v2353: *mut WhereTerm = pOrTerm;
                let __v2354: *mut WhereTerm = unsafe { __v2353.offset((1 as i32) as isize) };
                pOrTerm = __v2354;
            }
            unsafe {
                (*pNew).nLTerm = ((1 as i32) as i16) as u16;
            }
            unsafe {
                *unsafe { unsafe { (*pNew).aLTerm }.offset((0 as i32) as isize) } = pTerm;
            }
            unsafe {
                (*pNew).wsFlags = (8192 as i32) as u32;
            }
            unsafe {
                (*pNew).rSetup = (0 as i32) as i16;
            }
            unsafe {
                (*pNew).iSortIdx = ((0 as i32) as i8) as u8;
            }
            unsafe {
                memset(
                    (unsafe { std::ptr::addr_of_mut!((*pNew).u) }) as *mut (),
                    0 as i32,
                    24 as u64,
                )
            };
            i = 0 as i32;
            '__slate_break_1798: loop {
                if !(rc == (0 as i32) && i < ((sSum.n as u32) as i32)) {
                    break;
                }
                // /* TUNING: Currently sSum.a[i].rRun is set to the sum of the costs
                //         ** of all sub-scans required by the OR-scan. However, due to rounding
                //         ** errors, it may be that the cost of the OR-scan is equal to its
                //         ** most expensive sub-scan. Add the smallest possible penalty
                //         ** (equivalent to multiplying the cost by 1.07) to ensure that
                //         ** this does not happen. Otherwise, for WHERE clauses such as the
                //         ** following where there is an index on "y":
                //         **
                //         **     WHERE likelihood(x=?, 0.99) OR y=?
                //         **
                //         ** the planner may elect to "OR" together a full-table scan and an
                //         ** index lookup. And other similarly odd results.  */
                unsafe {
                    (*pNew).rRun = (((unsafe {
                        (*unsafe { (sSum.a.as_mut_ptr() as *mut WhereOrCost).offset(i as isize) })
                            .rRun
                    }) as i32)
                        + (1 as i32)) as i16;
                }
                unsafe {
                    (*pNew).nOut = unsafe {
                        (*unsafe { (sSum.a.as_mut_ptr() as *mut WhereOrCost).offset(i as isize) })
                            .nOut
                    };
                }
                unsafe {
                    (*pNew).prereq = unsafe {
                        (*unsafe { (sSum.a.as_mut_ptr() as *mut WhereOrCost).offset(i as isize) })
                            .prereq
                    };
                }
                rc = whereLoopInsert(pBuilder, pNew);
                let __v2359: i32 = i;
                let __v2360: i32 = __v2359 + (1 as i32);
                i = __v2360;
            }
            {}
        }
        let __v2351: *mut WhereTerm = pTerm;
        let __v2352: *mut WhereTerm = unsafe { __v2351.offset((1 as i32) as isize) };
        pTerm = __v2352;
    }
    return rc;
}

// /*
// ** Add all WhereLoop objects for all tables
// */
fn whereLoopAddAll(mut pBuilder: *mut WhereLoopBuilder) -> i32 {
    let mut pWInfo: *mut WhereInfo = unsafe { (*pBuilder).pWInfo };
    let mut mPrereq: u64 = ((0 as i32) as i64) as u64;
    let mut mPrior: u64 = ((0 as i32) as i64) as u64;
    let mut iTab: i32 = 0 as i32;
    let mut pTabList: *mut SrcList = unsafe { (*pWInfo).pTabList };
    let mut pItem: *mut SrcItem = unsafe { std::mem::zeroed() };
    let mut pEnd: *mut SrcItem = unsafe {
        unsafe { std::ptr::addr_of_mut!((*pTabList).a) as *mut SrcItem }
            .offset((((unsafe { (*pWInfo).nLevel }) as u32) as i32) as isize)
    };
    let mut db: *mut sqlite3 = unsafe { (*unsafe { (*pWInfo).pParse }).db };
    let mut rc: i32 = 0 as i32;
    let mut bFirstPastRJ: i32 = 0 as i32;
    let mut hasRightCrossJoin: i32 = 0 as i32;
    let mut pNew: *mut WhereLoop = unsafe { std::mem::zeroed() };
    // /* Loop over the tables in the join, from left to right */
    pNew = unsafe { (*pBuilder).pNew };
    // /* Verify that pNew has already been initialized */
    0 as i32;
    0 as i32;
    0 as i32;
    0 as i32;
    unsafe {
        (*pBuilder).iPlanLimit = (20000 as i32) as u32;
    }
    iTab = 0 as i32;
    let __v2361: *mut SrcItem = unsafe { std::ptr::addr_of_mut!((*pTabList).a) as *mut SrcItem };
    pItem = __v2361;
    '__slate_break_1799: while pItem < pEnd {
        let mut mUnusable: u64 = ((0 as i32) as i64) as u64;
        unsafe {
            (*pNew).iTab = (iTab as i8) as u8;
        }
        let __v2366: *mut WhereLoopBuilder = pBuilder;
        let __v2367: u32 = unsafe { (*__v2366).iPlanLimit };
        let __v2368: u32 = __v2367.wrapping_add((1000 as i32) as u32);
        unsafe {
            (*__v2366).iPlanLimit = __v2368;
        }
        unsafe {
            (*pNew).maskSelf = sqlite3WhereGetMask(
                unsafe { std::ptr::addr_of_mut!((*pWInfo).sMaskSet) },
                unsafe { (*pItem).iCursor },
            );
        }
        if bFirstPastRJ != (0 as i32)
            || (((unsafe { (*pItem).fg.jointype }) as u32) as i32)
                & ((32 as i32) | (2 as i32) | (64 as i32))
                != (0 as i32)
        {
            // /* Add prerequisites to prevent reordering of FROM clause terms
            //       ** across CROSS joins and outer joins.  The bFirstPastRJ boolean
            //       ** prevents the right operand of a RIGHT JOIN from being swapped with
            //       ** other elements even further to the right.
            //       **
            //       ** The hasRightCrossJoin flag prevent FROM-clause terms from moving
            //       ** from the right side of a LEFT JOIN or CROSS JOIN over to the
            //       ** left side of that same join.  This is a required restriction in
            //       ** the case of LEFT JOIN - an incorrect answer may results if it is
            //       ** not enforced.  This restriction is not required for CROSS JOIN.
            //       ** It is provided merely as a means of controlling join order, under
            //       ** the theory that no real-world queries that care about performance
            //       ** actually use the CROSS JOIN syntax.
            //       */
            if (((unsafe { (*pItem).fg.jointype }) as u32) as i32) & ((64 as i32) | (2 as i32))
                != (0 as i32)
            {
                {}
                {}
                hasRightCrossJoin = 1 as i32;
            }
            let __v2369: u64 = mPrereq;
            let __v2370: u64 = __v2369 | mPrior;
            mPrereq = __v2370;
            bFirstPastRJ = ((((unsafe { (*pItem).fg.jointype }) as u32) as i32) & (16 as i32)
                != (0 as i32)) as i32;
        } else {
            if ((unsafe { (*pItem).fg.__slate_bits_0.__get_fromExists() }) as i32) != (0 as i32) {
                // /* joins that result from the EXISTS-to-JOIN optimization should not
                //       ** be moved to the left of any of their dependencies */
                let mut pWC: *mut WhereClause = unsafe { std::ptr::addr_of_mut!((*pWInfo).sWC) };
                let mut pTerm: *mut WhereTerm = unsafe { std::mem::zeroed() };
                let mut i: i32 = 0 as i32;
                i = unsafe { (*pWC).nBase };
                let __v2371: *mut WhereTerm = unsafe { (*pWC).a };
                pTerm = __v2371;
                '__slate_break_1800: while i > (0 as i32) {
                    if (unsafe { (*pNew).maskSelf }) & unsafe { (*pTerm).prereqAll }
                        != (((0 as i32) as i64) as u64)
                    {
                        let __v2376: u64 = mPrereq;
                        let __v2377: u64 = __v2376
                            | (unsafe { (*pTerm).prereqAll })
                                & unsafe { (*pNew).maskSelf }
                                    .wrapping_sub(((1 as i32) as i64) as u64);
                        mPrereq = __v2377;
                    }
                    let __v2372: i32 = i;
                    let __v2373: i32 = __v2372 - (1 as i32);
                    i = __v2373;
                    let __v2374: *mut WhereTerm = pTerm;
                    let __v2375: *mut WhereTerm = unsafe { __v2374.offset((1 as i32) as isize) };
                    pTerm = __v2375;
                }
            } else {
                if !(hasRightCrossJoin != (0 as i32)) {
                    mPrereq = ((0 as i32) as i64) as u64;
                }
            }
        }
        if (((unsafe { (*unsafe { (*pItem).pSTab }).eTabType }) as u32) as i32) == (1 as i32) {
            let mut p: *mut SrcItem = unsafe { std::mem::zeroed() };
            p = unsafe { pItem.offset((1 as i32) as isize) };
            '__slate_break_1801: while p < pEnd {
                if (((unsafe { (*p).fg.jointype }) as u32) as i32) & ((32 as i32) | (2 as i32))
                    != (0 as i32)
                {
                    let __v2380: u64 = mUnusable;
                    let __v2381: u64 = __v2380
                        | sqlite3WhereGetMask(
                            unsafe { std::ptr::addr_of_mut!((*pWInfo).sMaskSet) },
                            unsafe { (*p).iCursor },
                        );
                    mUnusable = __v2381;
                }
                let __v2378: *mut SrcItem = p;
                let __v2379: *mut SrcItem = unsafe { __v2378.offset((1 as i32) as isize) };
                p = __v2379;
            }
            rc = whereLoopAddVirtual(pBuilder, mPrereq, mUnusable);
        } else {
            rc = whereLoopAddBtree(pBuilder, mPrereq);
        }
        // /* SQLITE_OMIT_VIRTUALTABLE */
        if rc == (0 as i32) && (unsafe { (*unsafe { (*pBuilder).pWC }).hasOr }) != (0 as u8) {
            rc = whereLoopAddOr(pBuilder, mPrereq, mUnusable);
        }
        let __v2382: u64 = mPrior;
        let __v2383: u64 = __v2382 | unsafe { (*pNew).maskSelf };
        mPrior = __v2383;
        if rc != (0 as i32) || (unsafe { (*db).mallocFailed }) != (0 as u8) {
            if rc == (101 as i32) {
                // /* We hit the query planner search limit set by iPlanLimit */
                unsafe {
                    sqlite3_log(
                        28 as i32,
                        (b"abbreviated query algorithm search\0".as_ptr() as *mut i8) as *const i8,
                    )
                };
                rc = 0 as i32;
            } else {
                break '__slate_break_1799;
            }
        }
        let __v2362: i32 = iTab;
        let __v2363: i32 = __v2362 + (1 as i32);
        iTab = __v2363;
        let __v2364: *mut SrcItem = pItem;
        let __v2365: *mut SrcItem = unsafe { __v2364.offset((1 as i32) as isize) };
        pItem = __v2365;
    }
    whereLoopClear(db, pNew);
    return rc;
}

// /* Implementation of the order-by-subquery optimization:
// **
// ** WhereLoop pLoop, which the iLoop-th term of the nested loop, is really
// ** a subquery or CTE that has an ORDER BY clause.  See if any of the terms
// ** in the subquery ORDER BY clause will satisfy pOrderBy from the outer
// ** query.  Mark off all satisfied terms (by setting bits in *pOBSat) and
// ** return TRUE if they do.  If not, return false.
// **
// ** Example:
// **
// **    CREATE TABLE t1(a,b,c, PRIMARY KEY(a,b));
// **    CREATE TABLE t2(x,y);
// **    WITH t3(p,q) AS MATERIALIZED (SELECT x+y, x-y FROM t2 ORDER BY x+y)
// **       SELECT * FROM t3 JOIN t1 ON a=q ORDER BY p, b;
// **
// ** The CTE named "t3" comes out in the natural order of "p", so the first
// ** first them of "ORDER BY p,b" is satisfied by a sequential scan of "t3"
// ** and sorting only needs to occur on the second term "b".
// **
// ** Limitations:
// **
// ** (1)  The optimization is not applied if the outer ORDER BY contains
// **      a COLLATE clause.  The optimization might be applied if the
// **      outer ORDER BY uses NULLS FIRST, NULLS LAST, ASC, and/or DESC as
// **      long as the subquery ORDER BY does the same.  But if the
// **      outer ORDER BY uses COLLATE, even a redundant COLLATE, the
// **      optimization is bypassed.
// **
// ** (2)  The subquery ORDER BY terms must exactly match subquery result
// **      columns, including any COLLATE annotations.  This routine relies
// **      on iOrderByCol to do matching between order by terms and result
// **      columns, and iOrderByCol will not be set if the result column
// **      and ORDER BY collations differ.
// **
// ** (3)  The subquery and outer ORDER BY can be in opposite directions as
// **      long as  the subquery is materialized.  If the subquery is
// **      implemented as a co-routine, the sort orders must be in the same
// **      direction because there is no way to run a co-routine backwards.
// */
fn wherePathMatchSubqueryOB(
    mut pWInfo: *mut WhereInfo,
    mut pLoop: *mut WhereLoop,
    mut iLoop: i32,
    mut iCur: i32,
    mut pOrderBy: *mut ExprList,
    mut pRevMask: *mut u64,
    mut pOBSat: *mut u64,
) -> i32 {
    // /* Index into pOrderBy->a[] */
    let mut iOB: i32 = 0 as i32;
    // /* Index into pSubOB->a[] */
    let mut jSub: i32 = 0 as i32;
    // /* True if iOB and jSub sort in opposite directions */
    let mut rev: u8 = ((0 as i32) as i8) as u8;
    // /* Sort direction for jSub */
    let mut revIdx: u8 = ((0 as i32) as i8) as u8;
    // /* Current term of outer ORDER BY */
    let mut pOBExpr: *mut Expr = unsafe { std::mem::zeroed() };
    // /* Complete ORDER BY on the subquery */
    let mut pSubOB: *mut ExprList = unsafe { std::mem::zeroed() };
    pSubOB = unsafe { (*pLoop).u.btree.pOrderBy };
    0 as i32;
    iOB = 0 as i32;
    '__slate_break_1803: loop {
        if !((((1 as i32) as i64) as u64) << iOB & unsafe { *pOBSat }
            != (((0 as i32) as i64) as u64))
        {
            break;
        }
        let __v2384: i32 = iOB;
        let __v2385: i32 = __v2384 + (1 as i32);
        iOB = __v2385;
    }
    jSub = 0 as i32;
    '__slate_break_1804: loop {
        if !(jSub < unsafe { (*pSubOB).nExpr } && iOB < unsafe { (*pOrderBy).nExpr }) {
            break;
        }
        if (((unsafe {
            (*unsafe {
                unsafe { std::ptr::addr_of_mut!((*pSubOB).a) as *mut ExprList_item }
                    .offset(jSub as isize)
            })
            .u
            .x
            .iOrderByCol
        }) as u32) as i32)
            == (0 as i32)
        {
            break '__slate_break_1804;
        }
        pOBExpr = unsafe {
            (*unsafe {
                unsafe { std::ptr::addr_of_mut!((*pOrderBy).a) as *mut ExprList_item }
                    .offset(iOB as isize)
            })
            .pExpr
        };
        if (((unsafe { (*pOBExpr).op }) as u32) as i32) != (168 as i32)
            && (((unsafe { (*pOBExpr).op }) as u32) as i32) != (170 as i32)
        {
            break '__slate_break_1804;
        }
        if (unsafe { (*pOBExpr).iTable }) != iCur {
            break '__slate_break_1804;
        }
        if ((unsafe { (*pOBExpr).iColumn }) as i32)
            != (((unsafe {
                (*unsafe {
                    unsafe { std::ptr::addr_of_mut!((*pSubOB).a) as *mut ExprList_item }
                        .offset(jSub as isize)
                })
                .u
                .x
                .iOrderByCol
            }) as u32) as i32)
                - (1 as i32)
        {
            break '__slate_break_1804;
        }
        if (((unsafe { (*pWInfo).wctrlFlags }) as u32) as i32) & (64 as i32) == (0 as i32) {
            // /* sortFlags for iOB */
            let mut sfOB: u8 = unsafe {
                (*unsafe {
                    unsafe { std::ptr::addr_of_mut!((*pOrderBy).a) as *mut ExprList_item }
                        .offset(iOB as isize)
                })
                .fg
                .sortFlags
            };
            // /* sortFlags for jSub */
            let mut sfSub: u8 = unsafe {
                (*unsafe {
                    unsafe { std::ptr::addr_of_mut!((*pSubOB).a) as *mut ExprList_item }
                        .offset(jSub as isize)
                })
                .fg
                .sortFlags
            };
            if ((sfSub as u32) as i32) & (2 as i32) != ((sfOB as u32) as i32) & (2 as i32) {
                break '__slate_break_1804;
            }
            revIdx = ((((sfSub as u32) as i32) & (1 as i32)) as i8) as u8;
            if jSub > (0 as i32) {
                if ((rev as u32) as i32) ^ ((revIdx as u32) as i32)
                    != ((sfOB as u32) as i32) & (1 as i32)
                {
                    break '__slate_break_1804;
                }
            } else {
                rev =
                    ((((revIdx as u32) as i32) ^ ((sfOB as u32) as i32) & (1 as i32)) as i8) as u8;
                if rev != (0 as u8) {
                    if (unsafe { (*pLoop).wsFlags }) & ((33554432 as i32) as u32)
                        != ((0 as i32) as u32)
                    {
                        // /* Cannot run a co-routine in reverse order */
                        break '__slate_break_1804;
                    }
                    let __v2390: *mut u64 = pRevMask;
                    let __v2391: u64 = unsafe { *__v2390 };
                    let __v2392: u64 = __v2391 | (((1 as i32) as i64) as u64) << iLoop;
                    unsafe {
                        *__v2390 = __v2392;
                    }
                }
            }
        }
        let __v2393: *mut u64 = pOBSat;
        let __v2394: u64 = unsafe { *__v2393 };
        let __v2395: u64 = __v2394 | (((1 as i32) as i64) as u64) << iOB;
        unsafe {
            *__v2393 = __v2395;
        }
        let __v2386: i32 = jSub;
        let __v2387: i32 = __v2386 + (1 as i32);
        jSub = __v2387;
        let __v2388: i32 = iOB;
        let __v2389: i32 = __v2388 + (1 as i32);
        iOB = __v2389;
    }
    return (jSub > (0 as i32)) as i32;
}

// /* The WHERE clause */
// /* The nested loop term that is a subquery */
// /* Which level of the nested loop.  0==outermost */
// /* Cursor used by the this loop */
// /* The ORDER BY clause on the whole query */
// /* When loops need to go in reverse order */
// /* Which terms of pOrderBy are satisfied so far */
// /*
// ** Examine a WherePath (with the addition of the extra WhereLoop of the 6th
// ** parameters) to see if it outputs rows in the requested ORDER BY
// ** (or GROUP BY) without requiring a separate sort operation.  Return N:
// **
// **   N>0:   N terms of the ORDER BY clause are satisfied
// **   N==0:  No terms of the ORDER BY clause are satisfied
// **   N<0:   Unknown yet how many terms of ORDER BY might be satisfied.
// **
// ** Note that processing for WHERE_GROUPBY and WHERE_DISTINCTBY is not as
// ** strict.  With GROUP BY and DISTINCT the only requirement is that
// ** equivalent rows appear immediately adjacent to one another.  GROUP BY
// ** and DISTINCT do not require rows to appear in any particular order as long
// ** as equivalent rows are grouped together.  Thus for GROUP BY and DISTINCT
// ** the pOrderBy terms can be matched in any order.  With ORDER BY, the
// ** pOrderBy terms must be matched in strict left-to-right order.
// */
fn wherePathSatisfiesOrderBy(
    mut pWInfo: *mut WhereInfo,
    mut pOrderBy: *mut ExprList,
    mut pPath: *mut WherePath,
    mut wctrlFlags: u16,
    mut nLoop: u16,
    mut pLast: *mut WhereLoop,
    mut pRevMask: *mut u64,
) -> i8 {
    // /* True if rev is known */
    let mut revSet: u8 = 0 as u8;
    // /* Composite sort order */
    let mut rev: u8 = 0 as u8;
    // /* Index sort order */
    let mut revIdx: u8 = 0 as u8;
    // /* All prior WhereLoops are order-distinct */
    let mut isOrderDistinct: u8 = 0 as u8;
    // /* True if the loop has UNIQUE NOT NULL columns */
    let mut distinctColumns: u8 = 0 as u8;
    // /* iColumn matches a term of the ORDER BY clause */
    let mut isMatch: u8 = 0 as u8;
    // /* Allowed equality operators */
    let mut eqOpMask: u16 = 0 as u16;
    // /* Number of key columns in pIndex */
    let mut nKeyCol: u16 = 0 as u16;
    // /* Total number of ordered columns in the index */
    let mut nColumn: u16 = 0 as u16;
    // /* Number terms in the ORDER BY clause */
    let mut nOrderBy: u16 = 0 as u16;
    // /* Index of WhereLoop in pPath being processed */
    let mut iLoop: i32 = 0 as i32;
    // /* Loop counters */
    let mut i: i32 = 0 as i32;
    let mut j: i32 = 0 as i32;
    // /* Cursor number for current WhereLoop */
    let mut iCur: i32 = 0 as i32;
    // /* A column number within table iCur */
    let mut iColumn: i32 = 0 as i32;
    // /* Current WhereLoop being processed. */
    let mut pLoop: *mut WhereLoop = std::ptr::null_mut::<WhereLoop>();
    // /* A single term of the WHERE clause */
    let mut pTerm: *mut WhereTerm = unsafe { std::mem::zeroed() };
    // /* An expression from the ORDER BY clause */
    let mut pOBExpr: *mut Expr = unsafe { std::mem::zeroed() };
    // /* COLLATE function from an ORDER BY clause term */
    let mut pColl: *mut CollSeq = unsafe { std::mem::zeroed() };
    // /* The index associated with pLoop */
    let mut pIndex: *mut Index = unsafe { std::mem::zeroed() };
    // /* Database connection */
    let mut db: *mut sqlite3 = unsafe { (*unsafe { (*pWInfo).pParse }).db };
    // /* Mask of ORDER BY terms satisfied so far */
    let mut obSat: u64 = ((0 as i32) as i64) as u64;
    // /* Mask of all ORDER BY terms */
    let mut obDone: u64 = 0 as u64;
    // /* Mask of all well-ordered loops */
    let mut orderDistinctMask: u64 = 0 as u64;
    // /* Mask of inner loops */
    let mut ready: u64 = 0 as u64;
    // /*
    //   ** We say the WhereLoop is "one-row" if it generates no more than one
    //   ** row of output.  A WhereLoop is one-row if all of the following are true:
    //   **  (a) All index columns match with WHERE_COLUMN_EQ.
    //   **  (b) The index is unique
    //   ** Any WhereLoop with an WHERE_COLUMN_EQ constraint on the rowid is one-row.
    //   ** Every one-row WhereLoop will have the WHERE_ONEROW bit set in wsFlags.
    //   **
    //   ** We say the WhereLoop is "order-distinct" if the set of columns from
    //   ** that WhereLoop that are in the ORDER BY clause are different for every
    //   ** row of the WhereLoop.  Every one-row WhereLoop is automatically
    //   ** order-distinct.   A WhereLoop that has no columns in the ORDER BY clause
    //   ** is not order-distinct. To be order-distinct is not quite the same as being
    //   ** UNIQUE since a UNIQUE column or index can have multiple rows that
    //   ** are NULL and NULL values are equivalent for the purpose of order-distinct.
    //   ** To be order-distinct, the columns must be UNIQUE and NOT NULL.
    //   **
    //   ** The rowid for a table is always UNIQUE and NOT NULL so whenever the
    //   ** rowid appears in the ORDER BY clause, the corresponding WhereLoop is
    //   ** automatically order-distinct.
    //   */
    0 as i32;
    if nLoop != (0 as u16)
        && (unsafe { (*db).dbOptFlags }) & ((64 as i32) as u32) != ((0 as i32) as u32)
    {
        return (0 as i32) as i8;
    }
    nOrderBy = ((unsafe { (*pOrderBy).nExpr }) as i16) as u16;
    {}
    // /* Cannot optimize overly large ORDER BYs */
    if ((nOrderBy as u32) as i32)
        > (((8 as u64).wrapping_mul(((8 as i32) as i64) as u64) as u32) as i32) - (1 as i32)
    {
        return (0 as i32) as i8;
    }
    isOrderDistinct = ((1 as i32) as i8) as u8;
    obDone = ((((1 as i32) as i64) as u64) << ((nOrderBy as u32) as i32))
        .wrapping_sub(((1 as i32) as i64) as u64);
    orderDistinctMask = ((0 as i32) as i64) as u64;
    ready = ((0 as i32) as i64) as u64;
    eqOpMask = (((2 as i32) | (128 as i32) | (256 as i32)) as i16) as u16;
    if ((wctrlFlags as u32) as i32) & ((2048 as i32) | (2 as i32) | (1 as i32)) != (0 as i32) {
        let __v2396: u16 = eqOpMask;
        let __v2397: u16 = ((((__v2396 as u32) as i32) | (1 as i32)) as i16) as u16;
        eqOpMask = __v2397;
    }
    iLoop = 0 as i32;
    '__slate_break_1805: loop {
        if !(isOrderDistinct != (0 as u8) && obSat < obDone && iLoop <= ((nLoop as u32) as i32)) {
            break;
        }
        '__slate_continue_1805: {
            if iLoop > (0 as i32) {
                let __v2400: u64 = ready;
                let __v2401: u64 = __v2400 | unsafe { (*pLoop).maskSelf };
                ready = __v2401;
            }
            if iLoop < ((nLoop as u32) as i32) {
                pLoop = unsafe { *unsafe { unsafe { (*pPath).aLoop }.offset(iLoop as isize) } };
                if ((wctrlFlags as u32) as i32) & (2048 as i32) != (0 as i32) {
                    break '__slate_continue_1805;
                }
            } else {
                pLoop = pLast;
            }
            if (unsafe { (*pLoop).wsFlags }) & ((1024 as i32) as u32) != (0 as u32) {
                if (unsafe { (*pLoop).u.vtab.isOrdered }) != (0 as i8)
                    && (unsafe { (*pWInfo).pOrderBy }) == pOrderBy
                {
                    obSat = obDone;
                } else {
                    // /* No further ORDER BY terms may be matched. So this call should
                    //         ** return >=0, not -1. Clear isOrderDistinct to ensure it does so. */
                    isOrderDistinct = ((0 as i32) as i8) as u8;
                }
                break '__slate_break_1805;
            }
            iCur = unsafe {
                (*unsafe {
                    unsafe {
                        std::ptr::addr_of_mut!((*unsafe { (*pWInfo).pTabList }).a) as *mut SrcItem
                    }
                    .offset((((unsafe { (*pLoop).iTab }) as u32) as i32) as isize)
                })
                .iCursor
            };
            // /* Mark off any ORDER BY term X that is a column in the table of
            //     ** the current loop for which there is term in the WHERE
            //     ** clause of the form X IS NULL or X=? that reference only outer
            //     ** loops.
            //     */
            i = 0 as i32;
            '__slate_break_1806: loop {
                if !(i < ((nOrderBy as u32) as i32)) {
                    break;
                }
                '__slate_continue_1806: {
                    if (((1 as i32) as i64) as u64) << i & obSat != (0 as u64) {
                    } else {
                        pOBExpr = unsafe {
                            sqlite3ExprSkipCollateAndLikely(unsafe {
                                (*unsafe {
                                    unsafe {
                                        std::ptr::addr_of_mut!((*pOrderBy).a) as *mut ExprList_item
                                    }
                                    .offset(i as isize)
                                })
                                .pExpr
                            })
                        };
                        if pOBExpr == std::ptr::null_mut::<Expr>() {
                        } else {
                            if (((unsafe { (*pOBExpr).op }) as u32) as i32) != (168 as i32)
                                && (((unsafe { (*pOBExpr).op }) as u32) as i32) != (170 as i32)
                            {
                            } else {
                                if (unsafe { (*pOBExpr).iTable }) != iCur {
                                } else {
                                    pTerm = sqlite3WhereFindTerm(
                                        unsafe { std::ptr::addr_of_mut!((*pWInfo).sWC) },
                                        iCur,
                                        (unsafe { (*pOBExpr).iColumn }) as i32,
                                        !ready,
                                        eqOpMask as u32,
                                        std::ptr::null_mut::<Index>(),
                                    );
                                    if pTerm == std::ptr::null_mut::<WhereTerm>() {
                                    } else {
                                        if (((unsafe { (*pTerm).eOperator }) as u32) as i32)
                                            == (1 as i32)
                                        {
                                            // /* IN terms are only valid for sorting in the ORDER BY LIMIT
                                            //         ** optimization, and then only if they are actually used
                                            //         ** by the query plan */
                                            0 as i32;
                                            j = 0 as i32;
                                            '__slate_break_1807: loop {
                                                if !(j
                                                    < (((unsafe { (*pLoop).nLTerm }) as u32)
                                                        as i32)
                                                    && pTerm
                                                        != unsafe {
                                                            *unsafe {
                                                                unsafe { (*pLoop).aLTerm }
                                                                    .offset(j as isize)
                                                            }
                                                        })
                                                {
                                                    break;
                                                }
                                                let __v2404: i32 = j;
                                                let __v2405: i32 = __v2404 + (1 as i32);
                                                j = __v2405;
                                            }
                                            if j >= (((unsafe { (*pLoop).nLTerm }) as u32) as i32) {
                                                break '__slate_continue_1806;
                                            }
                                        }
                                        if (((unsafe { (*pTerm).eOperator }) as u32) as i32)
                                            & ((2 as i32) | (128 as i32))
                                            != (0 as i32)
                                            && ((unsafe { (*pOBExpr).iColumn }) as i32)
                                                >= (0 as i32)
                                        {
                                            let mut pParse: *mut Parse =
                                                unsafe { (*pWInfo).pParse };
                                            let mut pColl1: *mut CollSeq = unsafe {
                                                sqlite3ExprNNCollSeq(
                                                    pParse,
                                                    (unsafe {
                                                        (*unsafe {
                                                            unsafe {
                                                                std::ptr::addr_of_mut!(
                                                                    (*pOrderBy).a
                                                                )
                                                                    as *mut ExprList_item
                                                            }
                                                            .offset(i as isize)
                                                        })
                                                        .pExpr
                                                    })
                                                        as *const Expr,
                                                )
                                            };
                                            let mut pColl2: *mut CollSeq = unsafe {
                                                sqlite3ExprCompareCollSeq(
                                                    pParse,
                                                    (unsafe { (*pTerm).pExpr }) as *const Expr,
                                                )
                                            };
                                            0 as i32;
                                            let __v2406: bool;
                                            if pColl2 == std::ptr::null_mut::<CollSeq>() {
                                                __v2406 = true as bool;
                                            } else {
                                                __v2406 = (unsafe {
                                                    sqlite3StrICmp(
                                                        (unsafe { (*pColl1).zName }) as *const i8,
                                                        (unsafe { (*pColl2).zName }) as *const i8,
                                                    )
                                                }) != (0 as i32);
                                            }
                                            if __v2406 {
                                                break '__slate_continue_1806;
                                            }
                                            {}
                                        }
                                        let __v2407: u64 = obSat;
                                        let __v2408: u64 =
                                            __v2407 | (((1 as i32) as i64) as u64) << i;
                                        obSat = __v2408;
                                    }
                                }
                            }
                        }
                    }
                }
                let __v2402: i32 = i;
                let __v2403: i32 = __v2402 + (1 as i32);
                i = __v2403;
            }
            if (unsafe { (*pLoop).wsFlags }) & ((4096 as i32) as u32) == ((0 as i32) as u32) {
                if (unsafe { (*pLoop).wsFlags }) & ((256 as i32) as u32) != (0 as u32) {
                    let __v2409: bool;
                    if (unsafe { (*pLoop).u.btree.pOrderBy }) != std::ptr::null_mut::<ExprList>()
                        && (unsafe { (*db).dbOptFlags }) & ((268435456 as i32) as u32)
                            == ((0 as i32) as u32)
                    {
                        __v2409 = wherePathMatchSubqueryOB(
                            pWInfo,
                            pLoop,
                            iLoop,
                            iCur,
                            pOrderBy,
                            pRevMask,
                            std::ptr::addr_of_mut!(obSat),
                        ) != (0 as i32);
                    } else {
                        __v2409 = false as bool;
                    }
                    if __v2409 {
                        nColumn = ((0 as i32) as i16) as u16;
                        isOrderDistinct = ((0 as i32) as i8) as u8;
                    } else {
                        nColumn = ((1 as i32) as i16) as u16;
                    }
                    pIndex = std::ptr::null_mut::<Index>();
                    nKeyCol = ((0 as i32) as i16) as u16;
                } else {
                    let __v2410: *mut Index = unsafe { (*pLoop).u.btree.pIndex };
                    pIndex = __v2410;
                    if __v2410 == std::ptr::null_mut::<Index>()
                        || ((unsafe { (*pIndex).__slate_bits_0.__get_bUnordered() }) as i32)
                            != (0 as i32)
                    {
                        return (0 as i32) as i8;
                    } else {
                        nKeyCol = unsafe { (*pIndex).nKeyCol };
                        nColumn = unsafe { (*pIndex).nColumn };
                        0 as i32;
                        0 as i32;
                        // /* All relevant terms of the index must also be non-NULL in order
                        //         ** for isOrderDistinct to be true.  So the isOrderDistinct value
                        //         ** computed here might be a false positive.  Corrections will be
                        //         ** made at tag-20210426-1 below */
                        isOrderDistinct =
                            ((((unsafe { (*pIndex).onError }) as u32) as i32) != (0 as i32)
                                && (unsafe { (*pLoop).wsFlags }) & ((32768 as i32) as u32)
                                    == ((0 as i32) as u32)) as u8;
                    }
                }
                // /* Loop through all columns of the index and deal with the ones
                //       ** that are not constrained by == or IN.
                //       */
                revSet = ((0 as i32) as i8) as u8;
                rev = ((0 as i32) as i8) as u8;
                distinctColumns = ((0 as i32) as i8) as u8;
                j = 0 as i32;
                '__slate_break_1808: loop {
                    if !(j < ((nColumn as u32) as i32)) {
                        break;
                    }
                    '__slate_continue_1808: {
                        // /* True to run the ORDER BY search loop */
                        let mut bOnce: u8 = ((1 as i32) as i8) as u8;
                        0 as i32;
                        if j < (((unsafe { (*pLoop).u.btree.nEq }) as u32) as i32)
                            && j >= (((unsafe { (*pLoop).nSkip }) as u32) as i32)
                        {
                            let mut eOp: u16 = unsafe {
                                (*unsafe {
                                    *unsafe { unsafe { (*pLoop).aLTerm }.offset(j as isize) }
                                })
                                .eOperator
                            };
                            // /* Skip over == and IS and ISNULL terms.  (Also skip IN terms when
                            //           ** doing WHERE_ORDERBY_LIMIT processing).  Except, IS and ISNULL
                            //           ** terms imply that the index is not UNIQUE NOT NULL in which case
                            //           ** the loop need to be marked as not order-distinct because it can
                            //           ** have repeated NULL rows.
                            //           **
                            //           ** If the current term is a column of an ((?,?) IN (SELECT...))
                            //           ** expression for which the SELECT returns more than one column,
                            //           ** check that it is the only column used by this loop. Otherwise,
                            //           ** if it is one of two or more, none of the columns can be
                            //           ** considered to match an ORDER BY term.
                            //           */
                            if ((eOp as u32) as i32) & ((eqOpMask as u32) as i32) != (0 as i32) {
                                if ((eOp as u32) as i32) & ((256 as i32) | (128 as i32))
                                    != (0 as i32)
                                {
                                    {}
                                    {}
                                    {}
                                    isOrderDistinct = ((0 as i32) as i8) as u8;
                                }
                                break '__slate_continue_1808;
                            } else {
                                if ((eOp as u32) as i32) & (1 as i32) != (0 as i32) {
                                    // /* ALWAYS() justification: eOp is an equality operator due to the
                                    //             ** j<pLoop->u.btree.nEq constraint above.  Any equality other
                                    //             ** than WO_IN is captured by the previous "if".  So this one
                                    //             ** always has to be WO_IN. */
                                    let mut pX: *mut Expr = unsafe {
                                        (*unsafe {
                                            *unsafe {
                                                unsafe { (*pLoop).aLTerm }.offset(j as isize)
                                            }
                                        })
                                        .pExpr
                                    };
                                    i = j + (1 as i32);
                                    '__slate_break_1809: loop {
                                        if !(i
                                            < (((unsafe { (*pLoop).u.btree.nEq }) as u32) as i32))
                                        {
                                            break;
                                        }
                                        if (unsafe {
                                            (*unsafe {
                                                *unsafe {
                                                    unsafe { (*pLoop).aLTerm }.offset(i as isize)
                                                }
                                            })
                                            .pExpr
                                        }) == pX
                                        {
                                            0 as i32;
                                            bOnce = ((0 as i32) as i8) as u8;
                                            break '__slate_break_1809;
                                        }
                                        let __v2413: i32 = i;
                                        let __v2414: i32 = __v2413 + (1 as i32);
                                        i = __v2414;
                                    }
                                }
                            }
                        }
                        // /* Get the column number in the table (iColumn) and sort order
                        //         ** (revIdx) for the j-th column of the index.
                        //         */
                        if pIndex != std::ptr::null_mut::<Index>() {
                            iColumn = (unsafe {
                                *unsafe { unsafe { (*pIndex).aiColumn }.offset(j as isize) }
                            }) as i32;
                            revIdx = (((((unsafe {
                                *unsafe { unsafe { (*pIndex).aSortOrder }.offset(j as isize) }
                            }) as u32) as i32)
                                & (1 as i32)) as i8) as u8;
                            if iColumn == ((unsafe { (*unsafe { (*pIndex).pTable }).iPKey }) as i32)
                            {
                                iColumn = -(1 as i32);
                            }
                        } else {
                            iColumn = -(1 as i32);
                            revIdx = ((0 as i32) as i8) as u8;
                        }
                        // /* An unconstrained column that might be NULL means that this
                        //         ** WhereLoop is not well-ordered.  tag-20210426-1
                        //         */
                        if isOrderDistinct != (0 as u8) {
                            if iColumn >= (0 as i32)
                                && j >= (((unsafe { (*pLoop).u.btree.nEq }) as u32) as i32)
                                && ((unsafe {
                                    (*unsafe {
                                        unsafe { (*unsafe { (*pIndex).pTable }).aCol }
                                            .offset(iColumn as isize)
                                    })
                                    .__slate_bits_0
                                    .__get_notNull()
                                }) as i32)
                                    == (0 as i32)
                            {
                                isOrderDistinct = ((0 as i32) as i8) as u8;
                            }
                            if iColumn == -(2 as i32) {
                                isOrderDistinct = ((0 as i32) as i8) as u8;
                            }
                        }
                        // /* Find the ORDER BY term that corresponds to the j-th column
                        //         ** of the index and mark that ORDER BY term having been satisfied.
                        //         */
                        isMatch = ((0 as i32) as i8) as u8;
                        i = 0 as i32;
                        '__slate_break_1810: loop {
                            if !(bOnce != (0 as u8) && i < ((nOrderBy as u32) as i32)) {
                                break;
                            }
                            '__slate_continue_1810: {
                                if (((1 as i32) as i64) as u64) << i & obSat != (0 as u64) {
                                } else {
                                    pOBExpr = unsafe {
                                        sqlite3ExprSkipCollateAndLikely(unsafe {
                                            (*unsafe {
                                                unsafe {
                                                    std::ptr::addr_of_mut!((*pOrderBy).a)
                                                        as *mut ExprList_item
                                                }
                                                .offset(i as isize)
                                            })
                                            .pExpr
                                        })
                                    };
                                    {}
                                    {}
                                    if pOBExpr == std::ptr::null_mut::<Expr>() {
                                    } else {
                                        if ((wctrlFlags as u32) as i32)
                                            & ((64 as i32) | (128 as i32))
                                            == (0 as i32)
                                        {
                                            bOnce = ((0 as i32) as i8) as u8;
                                        }
                                        if iColumn >= -(1 as i32) {
                                            if (((unsafe { (*pOBExpr).op }) as u32) as i32)
                                                != (168 as i32)
                                                && (((unsafe { (*pOBExpr).op }) as u32) as i32)
                                                    != (170 as i32)
                                            {
                                                break '__slate_continue_1810;
                                            }
                                            if (unsafe { (*pOBExpr).iTable }) != iCur {
                                                break '__slate_continue_1810;
                                            }
                                            if ((unsafe { (*pOBExpr).iColumn }) as i32) != iColumn {
                                                break '__slate_continue_1810;
                                            }
                                        } else {
                                            let mut pIxExpr: *mut Expr = unsafe {
                                                (*unsafe {
                                                    unsafe {
                                                        std::ptr::addr_of_mut!(
                                                            (*unsafe { (*pIndex).aColExpr }).a
                                                        )
                                                            as *mut ExprList_item
                                                    }
                                                    .offset(j as isize)
                                                })
                                                .pExpr
                                            };
                                            if (unsafe {
                                                sqlite3ExprCompareSkip(pOBExpr, pIxExpr, iCur)
                                            }) != (0 as i32)
                                            {
                                                break '__slate_continue_1810;
                                            }
                                        }
                                        if iColumn != -(1 as i32) {
                                            pColl = unsafe {
                                                sqlite3ExprNNCollSeq(
                                                    unsafe { (*pWInfo).pParse },
                                                    (unsafe {
                                                        (*unsafe {
                                                            unsafe {
                                                                std::ptr::addr_of_mut!(
                                                                    (*pOrderBy).a
                                                                )
                                                                    as *mut ExprList_item
                                                            }
                                                            .offset(i as isize)
                                                        })
                                                        .pExpr
                                                    })
                                                        as *const Expr,
                                                )
                                            };
                                            if (unsafe {
                                                sqlite3StrICmp(
                                                    (unsafe { (*pColl).zName }) as *const i8,
                                                    unsafe {
                                                        *unsafe {
                                                            unsafe { (*pIndex).azColl }
                                                                .offset(j as isize)
                                                        }
                                                    },
                                                )
                                            }) != (0 as i32)
                                            {
                                                break '__slate_continue_1810;
                                            }
                                        }
                                        if ((wctrlFlags as u32) as i32) & (128 as i32) != (0 as i32)
                                        {
                                            unsafe {
                                                (*pLoop).u.btree.nDistinctCol =
                                                    ((j + (1 as i32)) as i16) as u16;
                                            }
                                        }
                                        isMatch = ((1 as i32) as i8) as u8;
                                        break '__slate_break_1810;
                                    }
                                }
                            }
                            let __v2415: i32 = i;
                            let __v2416: i32 = __v2415 + (1 as i32);
                            i = __v2416;
                        }
                        if isMatch != (0 as u8)
                            && ((wctrlFlags as u32) as i32) & (64 as i32) == (0 as i32)
                        {
                            // /* Make sure the sort order is compatible in an ORDER BY clause.
                            //           ** Sort order is irrelevant for a GROUP BY clause. */
                            if revSet != (0 as u8) {
                                if ((rev as u32) as i32) ^ ((revIdx as u32) as i32)
                                    != (((unsafe {
                                        (*unsafe {
                                            unsafe {
                                                std::ptr::addr_of_mut!((*pOrderBy).a)
                                                    as *mut ExprList_item
                                            }
                                            .offset(i as isize)
                                        })
                                        .fg
                                        .sortFlags
                                    }) as u32) as i32)
                                        & (1 as i32)
                                {
                                    isMatch = ((0 as i32) as i8) as u8;
                                }
                            } else {
                                rev = ((((revIdx as u32) as i32)
                                    ^ (((unsafe {
                                        (*unsafe {
                                            unsafe {
                                                std::ptr::addr_of_mut!((*pOrderBy).a)
                                                    as *mut ExprList_item
                                            }
                                            .offset(i as isize)
                                        })
                                        .fg
                                        .sortFlags
                                    }) as u32) as i32)
                                        & (1 as i32)) as i8)
                                    as u8;
                                if rev != (0 as u8) {
                                    let __v2417: *mut u64 = pRevMask;
                                    let __v2418: u64 = unsafe { *__v2417 };
                                    let __v2419: u64 =
                                        __v2418 | (((1 as i32) as i64) as u64) << iLoop;
                                    unsafe {
                                        *__v2417 = __v2419;
                                    }
                                }
                                revSet = ((1 as i32) as i8) as u8;
                            }
                        }
                        if isMatch != (0 as u8)
                            && (((unsafe {
                                (*unsafe {
                                    unsafe {
                                        std::ptr::addr_of_mut!((*pOrderBy).a) as *mut ExprList_item
                                    }
                                    .offset(i as isize)
                                })
                                .fg
                                .sortFlags
                            }) as u32) as i32)
                                & (2 as i32)
                                != (0 as i32)
                        {
                            if j == (((unsafe { (*pLoop).u.btree.nEq }) as u32) as i32) {
                                let __v2420: *mut WhereLoop = pLoop;
                                let __v2421: u32 = unsafe { (*__v2420).wsFlags };
                                let __v2422: u32 = __v2421 | ((524288 as i32) as u32);
                                unsafe {
                                    (*__v2420).wsFlags = __v2422;
                                }
                            } else {
                                isMatch = ((0 as i32) as i8) as u8;
                            }
                        }
                        if isMatch != (0 as u8) {
                            if iColumn == -(1 as i32) {
                                {}
                                distinctColumns = ((1 as i32) as i8) as u8;
                            }
                            let __v2423: u64 = obSat;
                            let __v2424: u64 = __v2423 | (((1 as i32) as i64) as u64) << i;
                            obSat = __v2424;
                        } else {
                            // /* No match found */
                            if j == (0 as i32) || j < ((nKeyCol as u32) as i32) {
                                {}
                                isOrderDistinct = ((0 as i32) as i8) as u8;
                            }
                            break '__slate_break_1808;
                        }
                        // /* end Loop over all index columns */
                    }
                    let __v2411: i32 = j;
                    let __v2412: i32 = __v2411 + (1 as i32);
                    j = __v2412;
                }
                if distinctColumns != (0 as u8) {
                    {}
                    isOrderDistinct = ((1 as i32) as i8) as u8;
                }
                // /* end-if not one-row */
            }
            // /* Mark off any other ORDER BY terms that reference pLoop */
            if isOrderDistinct != (0 as u8) {
                let __v2425: u64 = orderDistinctMask;
                let __v2426: u64 = __v2425 | unsafe { (*pLoop).maskSelf };
                orderDistinctMask = __v2426;
                i = 0 as i32;
                '__slate_break_1811: loop {
                    if !(i < ((nOrderBy as u32) as i32)) {
                        break;
                    }
                    let mut p: *mut Expr = unsafe { std::mem::zeroed() };
                    let mut mTerm: u64 = 0 as u64;
                    if (((1 as i32) as i64) as u64) << i & obSat != (0 as u64) {
                    } else {
                        p = unsafe {
                            (*unsafe {
                                unsafe {
                                    std::ptr::addr_of_mut!((*pOrderBy).a) as *mut ExprList_item
                                }
                                .offset(i as isize)
                            })
                            .pExpr
                        };
                        mTerm = unsafe {
                            sqlite3WhereExprUsage(
                                unsafe { std::ptr::addr_of_mut!((*pWInfo).sMaskSet) },
                                p,
                            )
                        };
                        let __v2429: bool;
                        if mTerm == (((0 as i32) as i64) as u64) {
                            __v2429 = !((unsafe {
                                sqlite3ExprIsConstant(std::ptr::null_mut::<Parse>(), p)
                            }) != (0 as i32));
                        } else {
                            __v2429 = false as bool;
                        }
                        if __v2429 {
                        } else {
                            if mTerm & !orderDistinctMask == (((0 as i32) as i64) as u64) {
                                let __v2430: u64 = obSat;
                                let __v2431: u64 = __v2430 | (((1 as i32) as i64) as u64) << i;
                                obSat = __v2431;
                            }
                        }
                    }
                    let __v2427: i32 = i;
                    let __v2428: i32 = __v2427 + (1 as i32);
                    i = __v2428;
                }
            }
            // /* End the loop over all WhereLoops from outer-most down to inner-most */
        }
        let __v2398: i32 = iLoop;
        let __v2399: i32 = __v2398 + (1 as i32);
        iLoop = __v2399;
    }
    if obSat == obDone {
        return (nOrderBy as u8) as i8;
    }
    if !(isOrderDistinct != (0 as u8)) {
        i = ((nOrderBy as u32) as i32) - (1 as i32);
        '__slate_break_1812: loop {
            if !(i > (0 as i32)) {
                break;
            }
            let mut m: u64 =
                if i < (((8 as u64).wrapping_mul(((8 as i32) as i64) as u64) as u32) as i32) {
                    ((((1 as i32) as i64) as u64) << i).wrapping_sub(((1 as i32) as i64) as u64)
                } else {
                    ((0 as i32) as i64) as u64
                };
            if obSat & m == m {
                return i as i8;
            }
            let __v2432: i32 = i;
            let __v2433: i32 = __v2432 - (1 as i32);
            i = __v2433;
        }
        return (0 as i32) as i8;
    }
    return -(1 as i32) as i8;
}

// /*
// ** Return the cost of sorting nRow rows, assuming that the keys have
// ** nOrderby columns and that the first nSorted columns are already in
// ** order.
// */
fn whereSortingCost(
    mut pWInfo: *mut WhereInfo,
    mut nRow: i16,
    mut nOrderBy: i32,
    mut nSorted: i32,
) -> i16 {
    // /* Estimated cost of a full external sort, where N is
    //   ** the number of rows to sort is:
    //   **
    //   **   cost = (K * N * log(N)).
    //   **
    //   ** Or, if the order-by clause has X terms but only the last Y
    //   ** terms are out of order, then block-sorting will reduce the
    //   ** sorting cost to:
    //   **
    //   **   cost = (K * N * log(N)) * (Y/X)
    //   **
    //   ** The constant K is at least 2.0 but will be larger if there are a
    //   ** large number of columns to be sorted, as the sorting time is
    //   ** proportional to the amount of content to be sorted.  The algorithm
    //   ** does not currently distinguish between fat columns (BLOBs and TEXTs)
    //   ** and skinny columns (INTs).  It just uses the number of columns as
    //   ** an approximation for the row width.
    //   **
    //   ** And extra factor of 2.0 or 3.0 is added to the sorting cost if the sort
    //   ** is built using OP_IdxInsert and OP_Sort rather than with OP_SorterInsert.
    //   */
    let mut rSortCost: i16 = 0 as i16;
    let mut nCol: i16 = 0 as i16;
    0 as i32;
    0 as i32;
    // /* TUNING: sorting cost proportional to the number of output columns: */
    nCol = unsafe {
        sqlite3LogEst(
            ((((unsafe { (*unsafe { (*unsafe { (*pWInfo).pSelect }).pEList }).nExpr })
                + (59 as i32))
                / (30 as i32)) as i64) as u64,
        )
    };
    rSortCost = ((nRow as i32) + (nCol as i32)) as i16;
    if nSorted > (0 as i32) {
        // /* Scale the result by (Y/X) */
        let __v2434: i16 = rSortCost;
        let __v2435: i16 = ((__v2434 as i32)
            + (((unsafe {
                sqlite3LogEst((((nOrderBy - nSorted) * (100 as i32) / nOrderBy) as i64) as u64)
            }) as i32)
                - (66 as i32))) as i16;
        rSortCost = __v2435;
    }
    // /* Multiple by log(M) where M is the number of output rows.
    //   ** Use the LIMIT for M if it is smaller.  Or if this sort is for
    //   ** a DISTINCT operator, M will be the number of distinct output
    //   ** rows, so fudge it downwards a bit.
    //   */
    if (((unsafe { (*pWInfo).wctrlFlags }) as u32) as i32) & (16384 as i32) != (0 as i32) {
        // /* TUNING: Extra 2.0x if using LIMIT */
        let __v2436: i16 = rSortCost;
        let __v2437: i16 = ((__v2436 as i32) + (10 as i32)) as i16;
        rSortCost = __v2437;
        if nSorted != (0 as i32) {
            // /* TUNING: Extra 1.5x if also using partial sort */
            let __v2438: i16 = rSortCost;
            let __v2439: i16 = ((__v2438 as i32) + (6 as i32)) as i16;
            rSortCost = __v2439;
        }
        if ((unsafe { (*pWInfo).iLimit }) as i32) < (nRow as i32) {
            nRow = unsafe { (*pWInfo).iLimit };
        }
    } else {
        if (((unsafe { (*pWInfo).wctrlFlags }) as u32) as i32) & (256 as i32) != (0 as i32) {
            // /* TUNING: In the sort for a DISTINCT operator, assume that the DISTINCT
            //     ** reduces the number of output rows by a factor of 2 */
            if (nRow as i32) > (10 as i32) {
                let __v2440: i16 = nRow;
                let __v2441: i16 = ((__v2440 as i32) - (10 as i32)) as i16;
                nRow = __v2441;
                0 as i32;
            }
        }
    }
    let __v2442: i16 = rSortCost;
    let __v2443: i16 = ((__v2442 as i32) + (estLog(nRow) as i32)) as i16;
    rSortCost = __v2443;
    return rSortCost;
}

// /* Query planning context */
// /* Estimated number of rows to sort */
// /* Number of ORDER BY clause terms */
// /* Number of initial ORDER BY terms naturally in order */
// /*
// ** Compute the maximum number of paths in the solver algorithm, for
// ** queries that have three or more terms in the FROM clause.  Queries with
// ** two or fewer FROM clause terms are handled by the caller.
// **
// ** Query planning is NP-hard.  We must limit the number of paths at
// ** each step of the solver search algorithm to avoid exponential behavior.
// **
// ** The value returned is a tuning parameter.  Currently the value is:
// **
// **     18    for star queries
// **     12    otherwise
// **
// ** For the purposes of this heuristic, a star-query is defined as a query
// ** with a central "fact" table that is joined against multiple
// ** "dimension" tables, subject to the following constraints:
// **
// **   (aa)  Only a five-way or larger join is considered for this
// **         optimization.  If there are fewer than four terms in the FROM
// **         clause, this heuristic does not apply.
// **
// **   (bb)  The join between the fact table and the dimension tables must
// **         be an INNER join.  CROSS and OUTER JOINs do not qualify.
// **
// **   (cc)  A table must have 3 or more dimension tables in order to be
// **         considered a fact table. (Was 4 prior to 2026-02-10.)
// **
// **   (dd)  A table that is a self-join cannot be a dimension table.
// **         Dimension tables are joined against fact tables.
// **
// ** SIDE EFFECT:  (and really the whole point of this subroutine)
// **
// ** If pWInfo describes a star-query, then the cost for SCANs of dimension
// ** WhereLoops is increased to be slightly larger than the cost of a SCAN
// ** in the fact table.  Only SCAN costs are increased.  SEARCH costs are
// ** unchanged. This heuristic helps keep fact tables in outer loops. Without
// ** this heuristic, paths with fact tables in outer loops tend to get pruned
// ** by the mxChoice limit on the number of paths, resulting in poor query
// ** plans.  See the starschema1.test test module for examples of queries
// ** that need this heuristic to find good query plans.
// **
// ** This heuristic can be completely disabled, so that no query is
// ** considered a star-query, using SQLITE_TESTCTRL_OPTIMIZATION to
// ** disable the SQLITE_StarQuery optimization.  In the CLI, the command
// ** to do that is:  ".testctrl opt -starquery".
// **
// ** HISTORICAL NOTES:
// **
// ** This optimization was first added on 2024-05-09 by check-in 38db9b5c83d.
// ** The original optimization reduced the cost and output size estimate for
// ** fact tables to help them move to outer loops.  But months later (as people
// ** started upgrading) performance regression reports started caming in,
// ** including:
// **
// **    forum post b18ef983e68d06d1 (2024-12-21)
// **    forum post 0025389d0860af82 (2025-01-14)
// **    forum post d87570a145599033 (2025-01-17)
// **
// ** To address these, the criteria for a star-query was tightened to exclude
// ** cases where the fact and dimensions are separated by an outer join, and
// ** the affect of star-schema detection was changed to increase the rRun cost
// ** on just full table scans of dimension tables, rather than reducing costs
// ** in the all access methods of the fact table.
// */
fn computeMxChoice(mut pWInfo: *mut WhereInfo) -> i32 {
    // /* Number of terms in the join */
    let mut nLoop: i32 = ((unsafe { (*pWInfo).nLevel }) as u32) as i32;
    // /* For looping over WhereLoops */
    let mut pWLoop: *mut WhereLoop = unsafe { std::mem::zeroed() };
    // /* Constraint (aa) */
    if nLoop >= (4 as i32)
        && !(((unsafe { (*pWInfo).__slate_bits_0.__get_bStarDone() }) as i32) != (0 as i32))
        && (unsafe { (*unsafe { (*unsafe { (*pWInfo).pParse }).db }).dbOptFlags })
            & ((536870912 as i32) as u32)
            == ((0 as i32) as u32)
    {
        // /* All terms of the FROM clause */
        let mut aFromTabs: *mut SrcItem = unsafe { std::mem::zeroed() };
        // /* Term of FROM clause is the candidate fact-table */
        let mut iFromIdx: i32 = 0 as i32;
        // /* Bitmask for candidate fact-table */
        let mut m: u64 = 0 as u64;
        // /* Tables that cannot be dimension tables */
        let mut mSelfJoin: u64 = ((0 as i32) as i64) as u64;
        // /* Where to start searching for dimension-tables */
        let mut pStart: *mut WhereLoop = unsafe { std::mem::zeroed() };
        // /* Only do this computation once */
        unsafe {
            (*pWInfo).__slate_bits_0.__set_bStarDone((1 as i32) as u32);
        }
        // /* Look for fact tables with three or more dimensions where the
        //     ** dimension tables are not separately from the fact tables by an outer
        //     ** or cross join.  Adjust cost weights if found.
        //     */
        0 as i32;
        aFromTabs =
            unsafe { std::ptr::addr_of_mut!((*unsafe { (*pWInfo).pTabList }).a) as *mut SrcItem };
        pStart = unsafe { (*pWInfo).pLoops };
        iFromIdx = 0 as i32;
        m = ((1 as i32) as i64) as u64;
        '__slate_break_1813: loop {
            if !(iFromIdx < nLoop) {
                break;
            }
            // /* Number of dimension tables */
            let mut nDep: i32 = 0 as i32;
            // /* Maximum SCAN cost of a fact table */
            let mut mxRun: i16 = 0 as i16;
            // /* Mask of dimension tables */
            let mut mSeen: u64 = ((0 as i32) as i64) as u64;
            // /* The candidate fact table */
            let mut pFactTab: *mut SrcItem = unsafe { std::mem::zeroed() };
            pFactTab = unsafe { aFromTabs.offset(iFromIdx as isize) };
            if (((unsafe { (*pFactTab).fg.jointype }) as u32) as i32) & ((32 as i32) | (2 as i32))
                != (0 as i32)
            {
                // /* If the candidate fact-table is the right table of an outer join
                //         ** restrict the search for dimension-tables to be tables to the right
                //         ** of the fact-table.  Constraint (bb) */
                if iFromIdx + (3 as i32) > nLoop {
                    // /* ^-- Impossible to reach nDep>=2 - Constraint (cc) */
                    break '__slate_break_1813;
                }
                '__slate_break_1814: while pStart != std::ptr::null_mut::<WhereLoop>()
                    && (((unsafe { (*pStart).iTab }) as u32) as i32) <= iFromIdx
                {
                    pStart = unsafe { (*pStart).pNextLoop };
                }
            }
            pWLoop = pStart;
            '__slate_break_1815: while pWLoop != std::ptr::null_mut::<WhereLoop>() {
                if (((unsafe {
                    (*unsafe {
                        aFromTabs.offset((((unsafe { (*pWLoop).iTab }) as u32) as i32) as isize)
                    })
                    .fg
                    .jointype
                }) as u32) as i32)
                    & ((32 as i32) | (2 as i32))
                    != (0 as i32)
                {
                    // /* Constraint (bb) */
                    break '__slate_break_1815;
                }
                // /* pWInfo depends on iFromIdx */
                if (unsafe { (*pWLoop).prereq }) & m != (((0 as i32) as i64) as u64)
                    && (unsafe { (*pWLoop).maskSelf }) & mSeen == (((0 as i32) as i64) as u64)
                    && (unsafe { (*pWLoop).maskSelf }) & mSelfJoin == (((0 as i32) as i64) as u64)
                {
                    if (unsafe {
                        (*unsafe {
                            aFromTabs.offset((((unsafe { (*pWLoop).iTab }) as u32) as i32) as isize)
                        })
                        .pSTab
                    }) == unsafe { (*pFactTab).pSTab }
                    {
                        let __v2448: u64 = mSelfJoin;
                        let __v2449: u64 = __v2448 | m;
                        mSelfJoin = __v2449;
                    } else {
                        let __v2450: i32 = nDep;
                        let __v2451: i32 = __v2450 + (1 as i32);
                        nDep = __v2451;
                        let __v2452: u64 = mSeen;
                        let __v2453: u64 = __v2452 | unsafe { (*pWLoop).maskSelf };
                        mSeen = __v2453;
                    }
                }
                // /* pWInfo not already a dependency */
                // /* Not a self-join */
                pWLoop = unsafe { (*pWLoop).pNextLoop };
            }
            if nDep <= (2 as i32) {
                // /* Constraint (cc) */
            } else {
                // /* If we reach this point, it means that pFactTab is a fact table
                //       ** with four or more dimensions connected by inner joins.  Proceed
                //       ** to make cost adjustments. */
                // /* 0x80000 */
                unsafe {
                    (*pWInfo).__slate_bits_0.__set_bStarUsed((1 as i32) as u32);
                }
                // /* Compute the maximum cost of any WhereLoop for the
                //       ** fact table plus one epsilon */
                mxRun = -(32768 as i32) as i16;
                pWLoop = pStart;
                '__slate_break_1816: while pWLoop != std::ptr::null_mut::<WhereLoop>() {
                    if (((unsafe { (*pWLoop).iTab }) as u32) as i32) < iFromIdx {
                    } else {
                        if (((unsafe { (*pWLoop).iTab }) as u32) as i32) > iFromIdx {
                            break '__slate_break_1816;
                        }
                        if ((unsafe { (*pWLoop).rRun }) as i32) > (mxRun as i32) {
                            mxRun = unsafe { (*pWLoop).rRun };
                        }
                    }
                    pWLoop = unsafe { (*pWLoop).pNextLoop };
                }
                if (mxRun as i32) < (32767 as i32) {
                    let __v2454: i16 = mxRun;
                    let __v2455: i16 = ((__v2454 as i32) + (1 as i32)) as i16;
                    mxRun = __v2455;
                }
                // /* Increase the cost of table scans for dimension tables to be
                //       ** slightly more than the maximum cost of the fact table */
                pWLoop = pStart;
                '__slate_break_1817: while pWLoop != std::ptr::null_mut::<WhereLoop>() {
                    if (unsafe { (*pWLoop).maskSelf }) & mSeen == (((0 as i32) as i64) as u64) {
                    } else {
                        if (unsafe { (*pWLoop).nLTerm }) != (0 as u16) {
                        } else {
                            if ((unsafe { (*pWLoop).rRun }) as i32) < (mxRun as i32) {
                                // /* 0x80000 */
                                unsafe {
                                    (*pWLoop).rRun = mxRun;
                                }
                            }
                        }
                    }
                    pWLoop = unsafe { (*pWLoop).pNextLoop };
                }
            }
            let __v2444: i32 = iFromIdx;
            let __v2445: i32 = __v2444 + (1 as i32);
            iFromIdx = __v2445;
            let __v2446: u64 = m;
            let __v2447: u64 = __v2446 << (1 as i32);
            m = __v2447;
        }
        // /* 0x80000 */
    }
    return if ((unsafe { (*pWInfo).__slate_bits_0.__get_bStarUsed() }) as i32) != (0 as i32) {
        18 as i32
    } else {
        12 as i32
    };
}

// /*
// ** Two WhereLoop objects, pCandidate and pBaseline, are known to have the
// ** same cost.  Look deep into each to see if pCandidate is even slightly
// ** better than pBaseline.  Return false if it is, if pCandidate is is preferred.
// ** Return true if pBaseline is preferred or if we cannot tell the difference.
// **
// **    Result       Meaning
// **    --------     ----------------------------------------------------------
// **    true         We cannot tell the difference in pCandidate and pBaseline
// **    false        pCandidate seems like a better choice than pBaseline
// */
fn whereLoopIsNoBetter(mut pCandidate: *const WhereLoop, mut pBaseline: *const WhereLoop) -> i32 {
    if (unsafe { (*pCandidate).wsFlags }) & ((512 as i32) as u32) == ((0 as i32) as u32) {
        return 1 as i32;
    }
    if (unsafe { (*pBaseline).wsFlags }) & ((512 as i32) as u32) == ((0 as i32) as u32) {
        return 1 as i32;
    }
    if ((unsafe { (*unsafe { (*pCandidate).u.btree.pIndex }).szIdxRow }) as i32)
        < ((unsafe { (*unsafe { (*pBaseline).u.btree.pIndex }).szIdxRow }) as i32)
    {
        return 0 as i32;
    }
    return 1 as i32;
}

// /*
// ** Given the list of WhereLoop objects at pWInfo->pLoops, this routine
// ** attempts to find the lowest cost path that visits each WhereLoop
// ** once.  This path is then loaded into the pWInfo->a[].pWLoop fields.
// **
// ** Assume that the total number of output rows that will need to be sorted
// ** will be nRowEst (in the 10*log2 representation).  Or, ignore sorting
// ** costs if nRowEst==0.
// **
// ** Return SQLITE_OK on success or SQLITE_NOMEM of a memory allocation
// ** error occurs.
// */
fn wherePathSolver(mut pWInfo: *mut WhereInfo, mut nRowEst: i16) -> i32 {
    // /* Maximum number of simultaneous paths tracked */
    let mut mxChoice: i32 = 0 as i32;
    // /* Number of terms in the join */
    let mut nLoop: i32 = 0 as i32;
    // /* Parsing context */
    let mut pParse: *mut Parse = unsafe { std::mem::zeroed() };
    // /* Loop counter over the terms of the join */
    let mut iLoop: i32 = 0 as i32;
    // /* Loop counters */
    let mut ii: i32 = 0 as i32;
    let mut jj: i32 = 0 as i32;
    // /* Index of next entry to replace */
    let mut mxI: i32 = 0 as i32;
    // /* Number of ORDER BY clause terms */
    let mut nOrderBy: i32 = 0 as i32;
    // /* Maximum cost of a set of paths */
    let mut mxCost: i16 = (0 as i32) as i16;
    // /* Maximum unsorted cost of a set of path */
    let mut mxUnsort: i16 = (0 as i32) as i16;
    // /* Number of valid entries in aTo[] and aFrom[] */
    let mut nTo: i32 = 0 as i32;
    let mut nFrom: i32 = 0 as i32;
    // /* All nFrom paths at the previous level */
    let mut aFrom: *mut WherePath = unsafe { std::mem::zeroed() };
    // /* The nTo best paths at the current level */
    let mut aTo: *mut WherePath = unsafe { std::mem::zeroed() };
    // /* An element of aFrom[] that we are working on */
    let mut pFrom: *mut WherePath = unsafe { std::mem::zeroed() };
    // /* An element of aTo[] that we are working on */
    let mut pTo: *mut WherePath = unsafe { std::mem::zeroed() };
    // /* One of the WhereLoop objects */
    let mut pWLoop: *mut WhereLoop = unsafe { std::mem::zeroed() };
    // /* Used to divy up the pSpace memory */
    let mut pX: *mut *mut WhereLoop = unsafe { std::mem::zeroed() };
    // /* Sorting and partial sorting costs */
    let mut aSortCost: *mut i16 = std::ptr::null_mut::<i16>();
    // /* Temporary memory used by this routine */
    let mut pSpace: *mut i8 = unsafe { std::mem::zeroed() };
    // /* Bytes of space allocated at pSpace */
    let mut nSpace: i32 = 0 as i32;
    pParse = unsafe { (*pWInfo).pParse };
    nLoop = ((unsafe { (*pWInfo).nLevel }) as u32) as i32;
    {}
    // /* TUNING: mxChoice is the maximum number of possible paths to preserve
    //   ** at each step.  Based on the number of loops in the FROM clause:
    //   **
    //   **     nLoop      mxChoice
    //   **     -----      --------
    //   **       1            1            // the most common case
    //   **       2            5
    //   **       3+        12 or 18        // see computeMxChoice()
    //   */
    if nLoop <= (1 as i32) {
        mxChoice = 1 as i32;
    } else {
        if nLoop == (2 as i32) {
            mxChoice = 5 as i32;
        } else {
            if (unsafe { (*pParse).nErr }) != (0 as i32) {
                mxChoice = 1 as i32;
            } else {
                mxChoice = computeMxChoice(pWInfo);
            }
        }
    }
    0 as i32;
    // /* If nRowEst is zero and there is an ORDER BY clause, ignore it. In this
    //   ** case the purpose of this call is to estimate the number of rows returned
    //   ** by the overall query. Once this estimate has been obtained, the caller
    //   ** will invoke this function a second time, passing the estimate as the
    //   ** nRowEst parameter.  */
    if (unsafe { (*pWInfo).pOrderBy }) == std::ptr::null_mut::<ExprList>()
        || (nRowEst as i32) == (0 as i32)
    {
        nOrderBy = 0 as i32;
    } else {
        nOrderBy = unsafe { (*unsafe { (*pWInfo).pOrderBy }).nExpr };
    }
    // /* Allocate and initialize space for aTo, aFrom and aSortCost[] */
    nSpace = ((32 as u64)
        .wrapping_add((8 as u64).wrapping_mul((nLoop as i64) as u64))
        .wrapping_mul((mxChoice as i64) as u64)
        .wrapping_mul(((2 as i32) as i64) as u64) as u32) as i32;
    let __v2456: i32 = nSpace;
    let __v2457: i32 = (((__v2456 as i64) as u64)
        .wrapping_add((2 as u64).wrapping_mul((nOrderBy as i64) as u64))
        as u32) as i32;
    nSpace = __v2457;
    pSpace = (unsafe { sqlite3DbMallocRawNN(unsafe { (*pParse).db }, (nSpace as i64) as u64) })
        as *mut i8;
    if pSpace == std::ptr::null_mut::<i8>() {
        return 7 as i32;
    }
    aTo = pSpace as *mut WherePath;
    aFrom = unsafe { aTo.offset(mxChoice as isize) };
    unsafe { memset(aFrom as *mut (), 0 as i32, 32 as u64) };
    pX = (unsafe { aFrom.offset(mxChoice as isize) }) as *mut *mut WhereLoop;
    ii = mxChoice * (2 as i32);
    let __v2458: *mut WherePath = aTo;
    pFrom = __v2458;
    '__slate_break_1818: while ii > (0 as i32) {
        unsafe {
            (*pFrom).aLoop = pX;
        }
        let __v2459: i32 = ii;
        let __v2460: i32 = __v2459 - (1 as i32);
        ii = __v2460;
        let __v2461: *mut WherePath = pFrom;
        let __v2462: *mut WherePath = unsafe { __v2461.offset((1 as i32) as isize) };
        pFrom = __v2462;
        let __v2463: *mut *mut WhereLoop = pX;
        let __v2464: *mut *mut WhereLoop = unsafe { __v2463.offset(nLoop as isize) };
        pX = __v2464;
    }
    if nOrderBy != (0 as i32) {
        // /* If there is an ORDER BY clause and it is not being ignored, set up
        //     ** space for the aSortCost[] array. Each element of the aSortCost array
        //     ** is either zero - meaning it has not yet been initialized - or the
        //     ** cost of sorting nRowEst rows of data where the first X terms of
        //     ** the ORDER BY clause are already in order, where X is the array
        //     ** index.  */
        aSortCost = pX as *mut i16;
        unsafe {
            memset(
                aSortCost as *mut (),
                0 as i32,
                (2 as u64).wrapping_mul((nOrderBy as i64) as u64),
            )
        };
    }
    0 as i32;
    0 as i32;
    // /* Seed the search with a single WherePath containing zero WhereLoops.
    //   **
    //   ** TUNING: Do not let the number of iterations go above 28.  If the cost
    //   ** of computing an automatic index is not paid back within the first 28
    //   ** rows, then do not use the automatic index. */
    unsafe {
        (*unsafe { aFrom.offset((0 as i32) as isize) }).nRow =
            (if ((unsafe { (*pParse).nQueryLoop }) as i32) < (48 as i32) {
                (unsafe { (*pParse).nQueryLoop }) as i32
            } else {
                48 as i32
            }) as i16;
    }
    0 as i32;
    nFrom = 1 as i32;
    0 as i32;
    if nOrderBy != (0 as i32) {
        // /* If nLoop is zero, then there are no FROM terms in the query. Since
        //     ** in this case the query may return a maximum of one row, the results
        //     ** are already in the requested order. Set isOrdered to nOrderBy to
        //     ** indicate this. Or, if nLoop is greater than zero, set isOrdered to
        //     ** -1, indicating that the result set may or may not be ordered,
        //     ** depending on the loops added to the current plan.  */
        unsafe {
            (*unsafe { aFrom.offset((0 as i32) as isize) }).isOrdered = (if nLoop > (0 as i32) {
                -(1 as i32)
            } else {
                nOrderBy
            }) as i8;
        }
    }
    // /* Compute successively longer WherePaths using the previous generation
    //   ** of WherePaths as the basis for the next.  Keep track of the mxChoice
    //   ** best paths at each generation */
    iLoop = 0 as i32;
    '__slate_break_1819: loop {
        if !(iLoop < nLoop) {
            break;
        }
        nTo = 0 as i32;
        ii = 0 as i32;
        let __v2467: *mut WherePath = aFrom;
        pFrom = __v2467;
        '__slate_break_1820: while ii < nFrom {
            pWLoop = unsafe { (*pWInfo).pLoops };
            '__slate_break_1821: while pWLoop != std::ptr::null_mut::<WhereLoop>() {
                '__slate_continue_1821: {
                    // /* Rows visited by (pFrom+pWLoop) */
                    let mut nOut: i16 = 0 as i16;
                    // /* Cost of path (pFrom+pWLoop) */
                    let mut rCost: i16 = 0 as i16;
                    // /* Unsorted cost of (pFrom+pWLoop) */
                    let mut rUnsort: i16 = 0 as i16;
                    // /* isOrdered for (pFrom+pWLoop) */
                    let mut isOrdered: i8 = 0 as i8;
                    // /* Mask of src visited by (..) */
                    let mut maskNew: u64 = 0 as u64;
                    // /* Mask of rev-order loops for (..) */
                    let mut revMask: u64 = 0 as u64;
                    if (unsafe { (*pWLoop).prereq }) & !unsafe { (*pFrom).maskLoop }
                        != (((0 as i32) as i64) as u64)
                    {
                    } else {
                        if (unsafe { (*pWLoop).maskSelf }) & unsafe { (*pFrom).maskLoop }
                            != (((0 as i32) as i64) as u64)
                        {
                        } else {
                            if (unsafe { (*pWLoop).wsFlags }) & ((16384 as i32) as u32)
                                != ((0 as i32) as u32)
                                && ((unsafe { (*pFrom).nRow }) as i32) < (3 as i32)
                            {
                                // /* Do not use an automatic index if the this loop is expected
                                //           ** to run less than 1.25 times.  It is tempting to also exclude
                                //           ** automatic index usage on an outer loop, but sometimes an automatic
                                //           ** index is useful in the outer loop of a correlated subquery. */
                                0 as i32;
                            } else {
                                // /* At this point, pWLoop is a candidate to be the next loop.
                                //         ** Compute its cost */
                                rUnsort = (((unsafe { (*pWLoop).rRun }) as i32)
                                    + ((unsafe { (*pFrom).nRow }) as i32))
                                    as i16;
                                if (unsafe { (*pWLoop).rSetup }) != (0 as i16) {
                                    rUnsort = unsafe {
                                        sqlite3LogEstAdd(unsafe { (*pWLoop).rSetup }, rUnsort)
                                    };
                                }
                                rUnsort = unsafe {
                                    sqlite3LogEstAdd(rUnsort, unsafe { (*pFrom).rUnsort })
                                };
                                nOut = (((unsafe { (*pFrom).nRow }) as i32)
                                    + ((unsafe { (*pWLoop).nOut }) as i32))
                                    as i16;
                                maskNew =
                                    (unsafe { (*pFrom).maskLoop }) | unsafe { (*pWLoop).maskSelf };
                                isOrdered = unsafe { (*pFrom).isOrdered };
                                if (isOrdered as i32) < (0 as i32) {
                                    revMask = ((0 as i32) as i64) as u64;
                                    isOrdered = wherePathSatisfiesOrderBy(
                                        pWInfo,
                                        unsafe { (*pWInfo).pOrderBy },
                                        pFrom,
                                        unsafe { (*pWInfo).wctrlFlags },
                                        (iLoop as i16) as u16,
                                        pWLoop,
                                        std::ptr::addr_of_mut!(revMask),
                                    );
                                } else {
                                    revMask = unsafe { (*pFrom).revLoop };
                                }
                                if (isOrdered as i32) >= (0 as i32) && (isOrdered as i32) < nOrderBy
                                {
                                    if ((unsafe {
                                        *unsafe { aSortCost.offset((isOrdered as i32) as isize) }
                                    }) as i32)
                                        == (0 as i32)
                                    {
                                        unsafe {
                                            *unsafe {
                                                aSortCost.offset((isOrdered as i32) as isize)
                                            } = whereSortingCost(
                                                pWInfo,
                                                nRowEst,
                                                nOrderBy,
                                                isOrdered as i32,
                                            );
                                        }
                                    }
                                    // /* TUNING:  Add a small extra penalty (3) to sorting as an
                                    //           ** extra encouragement to the query planner to select a plan
                                    //           ** where the rows emerge in the correct order without any sorting
                                    //           ** required. */
                                    rCost = (((unsafe {
                                        sqlite3LogEstAdd(rUnsort, unsafe {
                                            *unsafe {
                                                aSortCost.offset((isOrdered as i32) as isize)
                                            }
                                        })
                                    }) as i32)
                                        + (3 as i32))
                                        as i16;
                                    {}
                                } else {
                                    rCost = rUnsort;
                                    // /* TUNING:  Slight bias in favor of no-sort plans */
                                    let __v2472: i16 = rUnsort;
                                    let __v2473: i16 = ((__v2472 as i32) - (2 as i32)) as i16;
                                    rUnsort = __v2473;
                                }
                                // /* Check to see if pWLoop should be added to the set of
                                //         ** mxChoice best-so-far paths.
                                //         **
                                //         ** First look for an existing path among best-so-far paths
                                //         ** that:
                                //         **     (1) covers the same set of loops, and
                                //         **     (2) has a compatible isOrdered value.
                                //         **
                                //         ** "Compatible isOrdered value" means either
                                //         **     (A) both have isOrdered==-1, or
                                //         **     (B) both have isOrder>=0, or
                                //         **     (C) ordering does not matter because this is the last round
                                //         **         of the solver.
                                //         **
                                //         ** The term "((pTo->isOrdered^isOrdered)&0x80)==0" is equivalent
                                //         ** to (pTo->isOrdered==(-1))==(isOrdered==(-1))" for the range
                                //         ** of legal values for isOrdered, -1..64.
                                //         */
                                {}
                                jj = 0 as i32;
                                let __v2474: *mut WherePath = aTo;
                                pTo = __v2474;
                                '__slate_break_1822: while jj < nTo {
                                    if (unsafe { (*pTo).maskLoop }) == maskNew
                                        && ((((unsafe { (*pTo).isOrdered }) as i32)
                                            ^ (isOrdered as i32))
                                            & (128 as i32)
                                            == (0 as i32)
                                            || iLoop == nLoop - (1 as i32))
                                    {
                                        {}
                                        break '__slate_break_1822;
                                    }
                                    let __v2475: i32 = jj;
                                    let __v2476: i32 = __v2475 + (1 as i32);
                                    jj = __v2476;
                                    let __v2477: *mut WherePath = pTo;
                                    let __v2478: *mut WherePath =
                                        unsafe { __v2477.offset((1 as i32) as isize) };
                                    pTo = __v2478;
                                }
                                if jj >= nTo {
                                    // /* None of the existing best-so-far paths match the candidate. */
                                    if nTo >= mxChoice
                                        && ((rCost as i32) > (mxCost as i32)
                                            || (rCost as i32) == (mxCost as i32)
                                                && (rUnsort as i32) >= (mxUnsort as i32))
                                    {
                                        // /* The current candidate is no better than any of the mxChoice
                                        //             ** paths currently in the best-so-far buffer.  So discard
                                        //             ** this candidate as not viable. */
                                        // /* 0x4 */
                                        break '__slate_continue_1821;
                                    }
                                    // /* If we reach this points it means that the new candidate path
                                    //           ** needs to be added to the set of best-so-far paths. */
                                    if nTo < mxChoice {
                                        // /* Increase the size of the aTo set by one */
                                        let __v2479: i32 = nTo;
                                        let __v2480: i32 = __v2479 + (1 as i32);
                                        nTo = __v2480;
                                        jj = __v2479;
                                    } else {
                                        // /* New path replaces the prior worst to keep count below mxChoice */
                                        jj = mxI;
                                    }
                                    pTo = unsafe { aTo.offset(jj as isize) };
                                // /* 0x4 */
                                } else {
                                    // /* Control reaches here if best-so-far path pTo=aTo[jj] covers the
                                    //           ** same set of loops and has the same isOrdered setting as the
                                    //           ** candidate path.  Check to see if the candidate should replace
                                    //           ** pTo or if the candidate should be skipped.
                                    //           **
                                    //           ** The conditional is an expanded vector comparison equivalent to:
                                    //           **   (pTo->rCost,pTo->nRow,pTo->rUnsort) <= (rCost,nOut,rUnsort)
                                    //           */
                                    let __v2481: bool;
                                    if ((unsafe { (*pTo).rCost }) as i32) < (rCost as i32)
                                        || ((unsafe { (*pTo).rCost }) as i32) == (rCost as i32)
                                            && ((unsafe { (*pTo).nRow }) as i32) < (nOut as i32)
                                        || ((unsafe { (*pTo).rCost }) as i32) == (rCost as i32)
                                            && ((unsafe { (*pTo).nRow }) as i32) == (nOut as i32)
                                            && ((unsafe { (*pTo).rUnsort }) as i32)
                                                < (rUnsort as i32)
                                    {
                                        __v2481 = true as bool;
                                    } else {
                                        let __v2482: bool;
                                        if ((unsafe { (*pTo).rCost }) as i32) == (rCost as i32)
                                            && ((unsafe { (*pTo).nRow }) as i32) == (nOut as i32)
                                            && ((unsafe { (*pTo).rUnsort }) as i32)
                                                == (rUnsort as i32)
                                        {
                                            __v2482 = whereLoopIsNoBetter(
                                                pWLoop as *const WhereLoop,
                                                (unsafe {
                                                    *unsafe {
                                                        unsafe { (*pTo).aLoop }
                                                            .offset(iLoop as isize)
                                                    }
                                                })
                                                    as *const WhereLoop,
                                            ) != (0 as i32);
                                        } else {
                                            __v2482 = false as bool;
                                        }
                                        __v2481 = __v2482;
                                    }
                                    if __v2481 {
                                        // /* 0x4 */
                                        // /* Discard the candidate path from further consideration */
                                        {}
                                        break '__slate_continue_1821;
                                    }
                                    {}
                                    // /* Control reaches here if the candidate path is better than the
                                    //           ** pTo path.  Replace pTo with the candidate. */
                                    // /* 0x4 */
                                }
                                // /* pWLoop is a winner.  Add it to the set of best so far */
                                unsafe {
                                    (*pTo).maskLoop = (unsafe { (*pFrom).maskLoop })
                                        | unsafe { (*pWLoop).maskSelf };
                                }
                                unsafe {
                                    (*pTo).revLoop = revMask;
                                }
                                unsafe {
                                    (*pTo).nRow = nOut;
                                }
                                unsafe {
                                    (*pTo).rCost = rCost;
                                }
                                unsafe {
                                    (*pTo).rUnsort = rUnsort;
                                }
                                unsafe {
                                    (*pTo).isOrdered = isOrdered;
                                }
                                unsafe {
                                    memcpy(
                                        (unsafe { (*pTo).aLoop }) as *mut (),
                                        (unsafe { (*pFrom).aLoop }) as *const (),
                                        (8 as u64).wrapping_mul((iLoop as i64) as u64),
                                    )
                                };
                                unsafe {
                                    *unsafe { unsafe { (*pTo).aLoop }.offset(iLoop as isize) } =
                                        pWLoop;
                                }
                                if nTo >= mxChoice {
                                    mxI = 0 as i32;
                                    mxCost = unsafe {
                                        (*unsafe { aTo.offset((0 as i32) as isize) }).rCost
                                    };
                                    mxUnsort = unsafe {
                                        (*unsafe { aTo.offset((0 as i32) as isize) }).nRow
                                    };
                                    jj = 1 as i32;
                                    let __v2483: *mut WherePath =
                                        unsafe { aTo.offset((1 as i32) as isize) };
                                    pTo = __v2483;
                                    '__slate_break_1823: while jj < mxChoice {
                                        if ((unsafe { (*pTo).rCost }) as i32) > (mxCost as i32)
                                            || ((unsafe { (*pTo).rCost }) as i32) == (mxCost as i32)
                                                && ((unsafe { (*pTo).rUnsort }) as i32)
                                                    > (mxUnsort as i32)
                                        {
                                            mxCost = unsafe { (*pTo).rCost };
                                            mxUnsort = unsafe { (*pTo).rUnsort };
                                            mxI = jj;
                                        }
                                        let __v2484: i32 = jj;
                                        let __v2485: i32 = __v2484 + (1 as i32);
                                        jj = __v2485;
                                        let __v2486: *mut WherePath = pTo;
                                        let __v2487: *mut WherePath =
                                            unsafe { __v2486.offset((1 as i32) as isize) };
                                        pTo = __v2487;
                                    }
                                }
                            }
                        }
                    }
                }
                pWLoop = unsafe { (*pWLoop).pNextLoop };
            }
            let __v2468: i32 = ii;
            let __v2469: i32 = __v2468 + (1 as i32);
            ii = __v2469;
            let __v2470: *mut WherePath = pFrom;
            let __v2471: *mut WherePath = unsafe { __v2470.offset((1 as i32) as isize) };
            pFrom = __v2471;
        }
        // /* >=2 */
        // /* Swap the roles of aFrom and aTo for the next generation */
        pFrom = aTo;
        aTo = aFrom;
        aFrom = pFrom;
        nFrom = nTo;
        let __v2465: i32 = iLoop;
        let __v2466: i32 = __v2465 + (1 as i32);
        iLoop = __v2466;
    }
    if nFrom == (0 as i32) {
        unsafe {
            sqlite3ErrorMsg(
                pParse,
                (b"no query solution\0".as_ptr() as *mut i8) as *const i8,
            )
        };
        unsafe { sqlite3DbFreeNN(unsafe { (*pParse).db }, pSpace as *mut ()) };
        return 1 as i32;
    }
    // /* Only one path is available, which is the best path */
    0 as i32;
    pFrom = aFrom;
    0 as i32;
    // /* Load the lowest cost path into pWInfo */
    iLoop = 0 as i32;
    '__slate_break_1825: loop {
        if !(iLoop < nLoop) {
            break;
        }
        let mut pLevel: *mut WhereLevel = unsafe {
            unsafe { std::ptr::addr_of_mut!((*pWInfo).a) as *mut WhereLevel }.offset(iLoop as isize)
        };
        let __v2490: *mut WhereLoop =
            unsafe { *unsafe { unsafe { (*pFrom).aLoop }.offset(iLoop as isize) } };
        pWLoop = __v2490;
        unsafe {
            (*pLevel).pWLoop = __v2490;
        }
        unsafe {
            (*pLevel).iFrom = unsafe { (*pWLoop).iTab };
        }
        unsafe {
            (*pLevel).iTabCur = unsafe {
                (*unsafe {
                    unsafe {
                        std::ptr::addr_of_mut!((*unsafe { (*pWInfo).pTabList }).a) as *mut SrcItem
                    }
                    .offset((((unsafe { (*pLevel).iFrom }) as u32) as i32) as isize)
                })
                .iCursor
            };
        }
        let __v2488: i32 = iLoop;
        let __v2489: i32 = __v2488 + (1 as i32);
        iLoop = __v2489;
    }
    if (((unsafe { (*pWInfo).wctrlFlags }) as u32) as i32) & (256 as i32) != (0 as i32)
        && (((unsafe { (*pWInfo).wctrlFlags }) as u32) as i32) & (128 as i32) == (0 as i32)
        && (((unsafe { (*pWInfo).eDistinct }) as u32) as i32) == (0 as i32)
        && nRowEst != (0 as i16)
    {
        let mut notUsed: u64 = 0 as u64;
        let mut rc: i32 = wherePathSatisfiesOrderBy(
            pWInfo,
            unsafe { (*pWInfo).pResultSet },
            pFrom,
            ((128 as i32) as i16) as u16,
            ((nLoop - (1 as i32)) as i16) as u16,
            unsafe { *unsafe { unsafe { (*pFrom).aLoop }.offset((nLoop - (1 as i32)) as isize) } },
            std::ptr::addr_of_mut!(notUsed),
        ) as i32;
        if rc == unsafe { (*unsafe { (*pWInfo).pResultSet }).nExpr } {
            unsafe {
                (*pWInfo).eDistinct = ((2 as i32) as i8) as u8;
            }
        }
    }
    unsafe {
        (*pWInfo)
            .__slate_bits_0
            .__set_bOrderedInnerLoop((0 as i32) as u32);
    }
    if (unsafe { (*pWInfo).pOrderBy }) != std::ptr::null_mut::<ExprList>() {
        unsafe {
            (*pWInfo).nOBSat = unsafe { (*pFrom).isOrdered };
        }
        if (((unsafe { (*pWInfo).wctrlFlags }) as u32) as i32) & (128 as i32) != (0 as i32) {
            if ((unsafe { (*pFrom).isOrdered }) as i32)
                == unsafe { (*unsafe { (*pWInfo).pOrderBy }).nExpr }
            {
                unsafe {
                    (*pWInfo).eDistinct = ((2 as i32) as i8) as u8;
                }
            }
            // /* vvv--- See check-in [12ad822d9b827777] on 2023-03-16 ---vvv */
            0 as i32;
        } else {
            unsafe {
                (*pWInfo).revMask = unsafe { (*pFrom).revLoop };
            }
            if ((unsafe { (*pWInfo).nOBSat }) as i32) <= (0 as i32) {
                unsafe {
                    (*pWInfo).nOBSat = (0 as i32) as i8;
                }
                if nLoop > (0 as i32) {
                    let mut wsFlags: u32 = unsafe {
                        (*unsafe {
                            *unsafe {
                                unsafe { (*pFrom).aLoop }.offset((nLoop - (1 as i32)) as isize)
                            }
                        })
                        .wsFlags
                    };
                    if wsFlags & ((4096 as i32) as u32) == ((0 as i32) as u32)
                        && wsFlags & (((256 as i32) | (4 as i32)) as u32)
                            != (((256 as i32) | (4 as i32)) as u32)
                    {
                        let mut m: u64 = ((0 as i32) as i64) as u64;
                        let mut rc: i32 = wherePathSatisfiesOrderBy(
                            pWInfo,
                            unsafe { (*pWInfo).pOrderBy },
                            pFrom,
                            ((2048 as i32) as i16) as u16,
                            ((nLoop - (1 as i32)) as i16) as u16,
                            unsafe {
                                *unsafe {
                                    unsafe { (*pFrom).aLoop }.offset((nLoop - (1 as i32)) as isize)
                                }
                            },
                            std::ptr::addr_of_mut!(m),
                        ) as i32;
                        {}
                        {}
                        if rc == unsafe { (*unsafe { (*pWInfo).pOrderBy }).nExpr } {
                            unsafe {
                                (*pWInfo)
                                    .__slate_bits_0
                                    .__set_bOrderedInnerLoop((1 as i32) as u32);
                            }
                            unsafe {
                                (*pWInfo).revMask = m;
                            }
                        }
                    }
                }
            } else {
                if nLoop != (0 as i32)
                    && ((unsafe { (*pWInfo).nOBSat }) as i32) == (1 as i32)
                    && (((unsafe { (*pWInfo).wctrlFlags }) as u32) as i32)
                        & ((1 as i32) | (2 as i32))
                        != (0 as i32)
                {
                    unsafe {
                        (*pWInfo)
                            .__slate_bits_0
                            .__set_bOrderedInnerLoop((1 as i32) as u32);
                    }
                }
            }
        }
        if (((unsafe { (*pWInfo).wctrlFlags }) as u32) as i32) & (512 as i32) != (0 as i32)
            && ((unsafe { (*pWInfo).nOBSat }) as i32)
                == unsafe { (*unsafe { (*pWInfo).pOrderBy }).nExpr }
            && nLoop > (0 as i32)
        {
            let mut revMask: u64 = ((0 as i32) as i64) as u64;
            let mut nOrder: i32 = wherePathSatisfiesOrderBy(
                pWInfo,
                unsafe { (*pWInfo).pOrderBy },
                pFrom,
                ((0 as i32) as i16) as u16,
                ((nLoop - (1 as i32)) as i16) as u16,
                unsafe {
                    *unsafe { unsafe { (*pFrom).aLoop }.offset((nLoop - (1 as i32)) as isize) }
                },
                std::ptr::addr_of_mut!(revMask),
            ) as i32;
            0 as i32;
            if nOrder == unsafe { (*unsafe { (*pWInfo).pOrderBy }).nExpr } {
                unsafe {
                    (*pWInfo).__slate_bits_0.__set_sorted((1 as i32) as u32);
                }
                unsafe {
                    (*pWInfo).revMask = revMask;
                }
            }
        }
    }
    unsafe {
        (*pWInfo).nRowOut = unsafe { (*pFrom).nRow };
    }
    // /* Free temporary memory and return success */
    unsafe { sqlite3DbFreeNN(unsafe { (*pParse).db }, pSpace as *mut ()) };
    return 0 as i32;
}

// /*
// ** This routine implements a heuristic designed to improve query planning.
// ** This routine is called in between the first and second call to
// ** wherePathSolver().  Hence the name "Interstage" "Heuristic".
// **
// ** The first call to wherePathSolver() (hereafter just "solver()") computes
// ** the best path without regard to the order of the outputs.  The second call
// ** to the solver() builds upon the first call to try to find an alternative
// ** path that satisfies the ORDER BY clause.
// **
// ** This routine looks at the results of the first solver() run, and for
// ** every FROM clause term in the resulting query plan that uses an equality
// ** constraint against an index, disable other WhereLoops for that same
// ** FROM clause term that would try to do a full-table scan.  This prevents
// ** an index search from being converted into a full-table scan in order to
// ** satisfy an ORDER BY clause, since even though we might get slightly better
// ** performance using the full-scan without sorting if the output size
// ** estimates are very precise, we might also get severe performance
// ** degradation using the full-scan if the output size estimate is too large.
// ** It is better to err on the side of caution.
// **
// ** Except, if the first solver() call generated a full-table scan in an outer
// ** loop then stop this analysis at the first full-scan, since the second
// ** solver() run might try to swap that full-scan for another in order to
// ** get the output into the correct order.  In other words, we allow a
// ** rewrite like this:
// **
// **     First Solver()                      Second Solver()
// **       |-- SCAN t1                         |-- SCAN t2
// **       |-- SEARCH t2                       `-- SEARCH t1
// **       `-- SORT USING B-TREE
// **
// ** The purpose of this routine is to disallow rewrites such as:
// **
// **     First Solver()                      Second Solver()
// **       |-- SEARCH t1                       |-- SCAN t2     <--- bad!
// **       |-- SEARCH t2                       `-- SEARCH t1
// **       `-- SORT USING B-TREE
// **
// ** See test cases in test/whereN.test for the real-world query that
// ** originally provoked this heuristic.
// */
fn whereInterstageHeuristic(mut pWInfo: *mut WhereInfo) {
    let mut i: i32 = 0 as i32;
    i = 0 as i32;
    '__slate_break_1826: loop {
        if !(i < (((unsafe { (*pWInfo).nLevel }) as u32) as i32)) {
            break;
        }
        let mut p: *mut WhereLoop = unsafe {
            (*unsafe {
                unsafe { std::ptr::addr_of_mut!((*pWInfo).a) as *mut WhereLevel }.offset(i as isize)
            })
            .pWLoop
        };
        if p == std::ptr::null_mut::<WhereLoop>() {
            break '__slate_break_1826;
        }
        if (unsafe { (*p).wsFlags }) & ((1024 as i32) as u32) != ((0 as i32) as u32) {
            // /* Treat a vtab scan as similar to a full-table scan */
            break '__slate_break_1826;
        }
        if (unsafe { (*p).wsFlags }) & (((1 as i32) | (8 as i32) | (4 as i32)) as u32)
            != ((0 as i32) as u32)
        {
            let mut iTab: u8 = unsafe { (*p).iTab };
            let mut pLoop: *mut WhereLoop = unsafe { std::mem::zeroed() };
            pLoop = unsafe { (*pWInfo).pLoops };
            '__slate_break_1827: while pLoop != std::ptr::null_mut::<WhereLoop>() {
                if (((unsafe { (*pLoop).iTab }) as u32) as i32) != ((iTab as u32) as i32) {
                } else {
                    if (unsafe { (*pLoop).wsFlags }) & (((15 as i32) | (16384 as i32)) as u32)
                        != ((0 as i32) as u32)
                    {
                        // /* Auto-index and index-constrained loops allowed to remain */
                    } else {
                        // /* Prevent 2nd solver() from using this one */
                        unsafe {
                            (*pLoop).prereq = (-(1 as i32) as i64) as u64;
                        }
                    }
                }
                pLoop = unsafe { (*pLoop).pNextLoop };
            }
        } else {
            break '__slate_break_1826;
        }
        let __v2491: i32 = i;
        let __v2492: i32 = __v2491 + (1 as i32);
        i = __v2492;
    }
}

// /*
// ** Most queries use only a single table (they are not joins) and have
// ** simple == constraints against indexed fields.  This routine attempts
// ** to plan those simple cases using much less ceremony than the
// ** general-purpose query planner, and thereby yield faster sqlite3_prepare()
// ** times for the common case.
// **
// ** Return non-zero on success, if this query can be handled by this
// ** no-frills query planner.  Return zero if this query needs the
// ** general-purpose query planner.
// */
fn whereShortCut(mut pBuilder: *mut WhereLoopBuilder) -> i32 {
    let mut pWInfo: *mut WhereInfo = unsafe { std::mem::zeroed() };
    let mut pItem: *mut SrcItem = unsafe { std::mem::zeroed() };
    let mut pWC: *mut WhereClause = unsafe { std::mem::zeroed() };
    let mut pTerm: *mut WhereTerm = unsafe { std::mem::zeroed() };
    let mut pLoop: *mut WhereLoop = unsafe { std::mem::zeroed() };
    let mut iCur: i32 = 0 as i32;
    let mut j: i32 = 0 as i32;
    let mut pTab: *mut Table = unsafe { std::mem::zeroed() };
    let mut pIdx: *mut Index = unsafe { std::mem::zeroed() };
    let mut scan: WhereScan = unsafe { std::mem::zeroed() };
    pWInfo = unsafe { (*pBuilder).pWInfo };
    if (((unsafe { (*pWInfo).wctrlFlags }) as u32) as i32) & (32 as i32) != (0 as i32) {
        return 0 as i32;
    }
    0 as i32;
    pItem = unsafe { std::ptr::addr_of_mut!((*unsafe { (*pWInfo).pTabList }).a) as *mut SrcItem };
    pTab = unsafe { (*pItem).pSTab };
    if (((unsafe { (*pTab).eTabType }) as u32) as i32) == (1 as i32) {
        return 0 as i32;
    }
    if ((unsafe { (*pItem).fg.__slate_bits_0.__get_isIndexedBy() }) as i32) != (0 as i32)
        || ((unsafe { (*pItem).fg.__slate_bits_0.__get_notIndexed() }) as i32) != (0 as i32)
    {
        {}
        {}
        return 0 as i32;
    }
    iCur = unsafe { (*pItem).iCursor };
    pWC = unsafe { std::ptr::addr_of_mut!((*pWInfo).sWC) };
    pLoop = unsafe { (*pBuilder).pNew };
    unsafe {
        (*pLoop).wsFlags = (0 as i32) as u32;
    }
    unsafe {
        (*pLoop).nSkip = ((0 as i32) as i16) as u16;
    }
    pTerm = whereScanInit(
        std::ptr::addr_of_mut!(scan),
        pWC,
        iCur,
        -(1 as i32),
        ((2 as i32) | (128 as i32)) as u32,
        std::ptr::null_mut::<Index>(),
    );
    '__slate_break_1828: while pTerm != std::ptr::null_mut::<WhereTerm>()
        && (unsafe { (*pTerm).prereqRight }) != (0 as u64)
    {
        pTerm = whereScanNext(std::ptr::addr_of_mut!(scan));
    }
    if pTerm != std::ptr::null_mut::<WhereTerm>() {
        {}
        unsafe {
            (*pLoop).wsFlags = ((1 as i32) | (256 as i32) | (4096 as i32)) as u32;
        }
        unsafe {
            *unsafe { unsafe { (*pLoop).aLTerm }.offset((0 as i32) as isize) } = pTerm;
        }
        unsafe {
            (*pLoop).nLTerm = ((1 as i32) as i16) as u16;
        }
        unsafe {
            (*pLoop).u.btree.nEq = ((1 as i32) as i16) as u16;
        }
        // /* TUNING: Cost of a rowid lookup is 10 */
        // /* 33==sqlite3LogEst(10) */
        unsafe {
            (*pLoop).rRun = (33 as i32) as i16;
        }
    } else {
        pIdx = unsafe { (*pTab).pIndex };
        '__slate_break_1829: while pIdx != std::ptr::null_mut::<Index>() {
            let mut opMask: i32 = 0 as i32;
            0 as i32;
            if !((((unsafe { (*pIdx).onError }) as u32) as i32) != (0 as i32))
                || (unsafe { (*pIdx).pPartIdxWhere }) != std::ptr::null_mut::<Expr>()
                || (((unsafe { (*pIdx).nKeyCol }) as u32) as i32)
                    > ((((24 as u64) / (8 as u64)) as u32) as i32)
            {
            } else {
                opMask = if ((unsafe { (*pIdx).__slate_bits_0.__get_uniqNotNull() }) as i32)
                    != (0 as i32)
                {
                    (2 as i32) | (128 as i32)
                } else {
                    2 as i32
                };
                j = 0 as i32;
                '__slate_break_1830: loop {
                    if !(j < (((unsafe { (*pIdx).nKeyCol }) as u32) as i32)) {
                        break;
                    }
                    pTerm = whereScanInit(
                        std::ptr::addr_of_mut!(scan),
                        pWC,
                        iCur,
                        j,
                        opMask as u32,
                        pIdx,
                    );
                    '__slate_break_1831: while pTerm != std::ptr::null_mut::<WhereTerm>()
                        && (unsafe { (*pTerm).prereqRight }) != (0 as u64)
                    {
                        pTerm = whereScanNext(std::ptr::addr_of_mut!(scan));
                    }
                    if pTerm == std::ptr::null_mut::<WhereTerm>() {
                        break '__slate_break_1830;
                    }
                    {}
                    unsafe {
                        *unsafe { unsafe { (*pLoop).aLTerm }.offset(j as isize) } = pTerm;
                    }
                    let __v2493: i32 = j;
                    let __v2494: i32 = __v2493 + (1 as i32);
                    j = __v2494;
                }
                if j != (((unsafe { (*pIdx).nKeyCol }) as u32) as i32) {
                } else {
                    unsafe {
                        (*pLoop).wsFlags = ((1 as i32) | (4096 as i32) | (512 as i32)) as u32;
                    }
                    if ((unsafe { (*pIdx).__slate_bits_0.__get_isCovering() }) as i32) != (0 as i32)
                        || (unsafe { (*pItem).colUsed }) & unsafe { (*pIdx).colNotIdxed }
                            == (((0 as i32) as i64) as u64)
                    {
                        let __v2495: *mut WhereLoop = pLoop;
                        let __v2496: u32 = unsafe { (*__v2495).wsFlags };
                        let __v2497: u32 = __v2496 | ((64 as i32) as u32);
                        unsafe {
                            (*__v2495).wsFlags = __v2497;
                        }
                    }
                    unsafe {
                        (*pLoop).nLTerm = (j as i16) as u16;
                    }
                    unsafe {
                        (*pLoop).u.btree.nEq = (j as i16) as u16;
                    }
                    unsafe {
                        (*pLoop).u.btree.pIndex = pIdx;
                    }
                    // /* TUNING: Cost of a unique index lookup is 15 */
                    // /* 39==sqlite3LogEst(15) */
                    unsafe {
                        (*pLoop).rRun = (39 as i32) as i16;
                    }
                    break '__slate_break_1829;
                }
            }
            pIdx = unsafe { (*pIdx).pNext };
        }
    }
    if (unsafe { (*pLoop).wsFlags }) != (0 as u32) {
        unsafe {
            (*pLoop).nOut = (1 as i32) as i16;
        }
        unsafe {
            (*unsafe {
                unsafe { std::ptr::addr_of_mut!((*pWInfo).a) as *mut WhereLevel }
                    .offset((0 as i32) as isize)
            })
            .pWLoop = pLoop;
        }
        0 as i32;
        // /* sqlite3WhereGetMask(&pWInfo->sMaskSet, iCur); */
        unsafe {
            (*pLoop).maskSelf = ((1 as i32) as i64) as u64;
        }
        unsafe {
            (*unsafe {
                unsafe { std::ptr::addr_of_mut!((*pWInfo).a) as *mut WhereLevel }
                    .offset((0 as i32) as isize)
            })
            .iTabCur = iCur;
        }
        unsafe {
            (*pWInfo).nRowOut = (1 as i32) as i16;
        }
        if (unsafe { (*pWInfo).pOrderBy }) != std::ptr::null_mut::<ExprList>() {
            unsafe {
                (*pWInfo).nOBSat = (unsafe { (*unsafe { (*pWInfo).pOrderBy }).nExpr }) as i8;
            }
        }
        if (((unsafe { (*pWInfo).wctrlFlags }) as u32) as i32) & (256 as i32) != (0 as i32) {
            unsafe {
                (*pWInfo).eDistinct = ((1 as i32) as i8) as u8;
            }
        }
        if ((scan.iEquiv as u32) as i32) > (1 as i32) {
            let __v2498: *mut WhereLoop = pLoop;
            let __v2499: u32 = unsafe { (*__v2498).wsFlags };
            let __v2500: u32 = __v2499 | ((2097152 as i32) as u32);
            unsafe {
                (*__v2498).wsFlags = __v2500;
            }
        }
        return 1 as i32;
    }
    return 0 as i32;
}

// /*
// ** Helper function for exprIsDeterministic().
// */
#[unsafe(link_section = ".text.slate_distinct.where.exprNodeIsDeterministic")]
extern "C-unwind" fn exprNodeIsDeterministic(
    mut pWalker: *mut Walker,
    mut pExpr: *mut Expr,
) -> i32 {
    if (((unsafe { (*pExpr).op }) as u32) as i32) == (172 as i32)
        && (((unsafe { (*pExpr).flags }) & ((1048576 as i32) as u32) != ((0 as i32) as u32)) as i32)
            == (0 as i32)
    {
        unsafe {
            (*pWalker).eCode = ((0 as i32) as i16) as u16;
        }
        return 2 as i32;
    }
    return 0 as i32;
}

// /*
// ** Return true if the expression contains no non-deterministic SQL
// ** functions. Do not consider non-deterministic SQL functions that are
// ** part of sub-select statements.
// */
fn exprIsDeterministic(mut p: *mut Expr) -> i32 {
    let mut w: Walker = unsafe { std::mem::zeroed() };
    unsafe { memset(std::ptr::addr_of_mut!(w) as *mut (), 0 as i32, 48 as u64) };
    w.eCode = ((1 as i32) as i16) as u16;
    w.xExprCallback = Some(exprNodeIsDeterministic);
    w.xSelectCallback = unsafe {
        std::mem::transmute::<
            *const (),
            Option<unsafe extern "C-unwind" fn(*mut Walker, *mut Select) -> i32>,
        >(sqlite3SelectWalkFail as *const ())
    };
    unsafe { sqlite3WalkExpr(std::ptr::addr_of_mut!(w), p) };
    return (w.eCode as u32) as i32;
}

// /* Attempt to omit tables from a join that do not affect the result.
// ** For a table to not affect the result, the following must be true:
// **
// **   1) The query must not be an aggregate.
// **   2) The table must be the RHS of a LEFT JOIN.
// **   3) Either the query must be DISTINCT, or else the ON or USING clause
// **      must contain a constraint that limits the scan of the table to
// **      at most a single row.
// **   4) The table must not be referenced by any part of the query apart
// **      from its own USING or ON clause.
// **   5) The table must not have an inner-join ON or USING clause if there is
// **      a RIGHT JOIN anywhere in the query.  Otherwise the ON/USING clause
// **      might move from the right side to the left side of the RIGHT JOIN.
// **      Note: Due to (2), this condition can only arise if the table is
// **      the right-most table of a subquery that was flattened into the
// **      main query and that subquery was the right-hand operand of an
// **      inner join that held an ON or USING clause.
// **   6) The ORDER BY clause has 63 or fewer terms
// **   7) The omit-noop-join optimization is enabled.
// **
// ** Items (1), (6), and (7) are checked by the caller.
// **
// ** For example, given:
// **
// **     CREATE TABLE t1(ipk INTEGER PRIMARY KEY, v1);
// **     CREATE TABLE t2(ipk INTEGER PRIMARY KEY, v2);
// **     CREATE TABLE t3(ipk INTEGER PRIMARY KEY, v3);
// **
// ** then table t2 can be omitted from the following:
// **
// **     SELECT v1, v3 FROM t1
// **       LEFT JOIN t2 ON (t1.ipk=t2.ipk)
// **       LEFT JOIN t3 ON (t1.ipk=t3.ipk)
// **
// ** or from:
// **
// **     SELECT DISTINCT v1, v3 FROM t1
// **       LEFT JOIN t2
// **       LEFT JOIN t3 ON (t1.ipk=t3.ipk)
// */
fn whereOmitNoopJoin(mut pWInfo: *mut WhereInfo, mut notReady: u64) -> u64 {
    let mut i: i32 = 0 as i32;
    let mut tabUsed: u64 = 0 as u64;
    let mut hasRightJoin: i32 = 0 as i32;
    // /* Preconditions checked by the caller */
    0 as i32;
    0 as i32;
    // /* These two preconditions checked by the caller combine to guarantee
    //   ** condition (1) of the header comment */
    0 as i32;
    0 as i32;
    tabUsed = unsafe {
        sqlite3WhereExprListUsage(
            unsafe { std::ptr::addr_of_mut!((*pWInfo).sMaskSet) },
            unsafe { (*pWInfo).pResultSet },
        )
    };
    if (unsafe { (*pWInfo).pOrderBy }) != std::ptr::null_mut::<ExprList>() {
        let __v2501: u64 = tabUsed;
        let __v2502: u64 = __v2501
            | unsafe {
                sqlite3WhereExprListUsage(
                    unsafe { std::ptr::addr_of_mut!((*pWInfo).sMaskSet) },
                    unsafe { (*pWInfo).pOrderBy },
                )
            };
        tabUsed = __v2502;
    }
    hasRightJoin = ((((unsafe {
        (*unsafe {
            unsafe { std::ptr::addr_of_mut!((*unsafe { (*pWInfo).pTabList }).a) as *mut SrcItem }
                .offset((0 as i32) as isize)
        })
        .fg
        .jointype
    }) as u32) as i32)
        & (64 as i32)
        != (0 as i32)) as i32;
    i = (((unsafe { (*pWInfo).nLevel }) as u32) as i32) - (1 as i32);
    '__slate_break_1832: loop {
        if !(i >= (1 as i32)) {
            break;
        }
        let mut pTerm: *mut WhereTerm = unsafe { std::mem::zeroed() };
        let mut pEnd: *mut WhereTerm = unsafe { std::mem::zeroed() };
        let mut pItem: *mut SrcItem = unsafe { std::mem::zeroed() };
        let mut pLoop: *mut WhereLoop = unsafe { std::mem::zeroed() };
        let mut m1: u64 = 0 as u64;
        pLoop = unsafe {
            (*unsafe {
                unsafe { std::ptr::addr_of_mut!((*pWInfo).a) as *mut WhereLevel }.offset(i as isize)
            })
            .pWLoop
        };
        pItem = unsafe {
            unsafe { std::ptr::addr_of_mut!((*unsafe { (*pWInfo).pTabList }).a) as *mut SrcItem }
                .offset((((unsafe { (*pLoop).iTab }) as u32) as i32) as isize)
        };
        if (((unsafe { (*pItem).fg.jointype }) as u32) as i32) & ((8 as i32) | (16 as i32))
            != (8 as i32)
        {
        } else {
            if (((unsafe { (*pWInfo).wctrlFlags }) as u32) as i32) & (256 as i32) == (0 as i32)
                && (unsafe { (*pLoop).wsFlags }) & ((4096 as i32) as u32) == ((0 as i32) as u32)
            {
            } else {
                if tabUsed & unsafe { (*pLoop).maskSelf } != (((0 as i32) as i64) as u64) {
                } else {
                    pEnd = unsafe {
                        unsafe { (*pWInfo).sWC.a }.offset((unsafe { (*pWInfo).sWC.nTerm }) as isize)
                    };
                    pTerm = unsafe { (*pWInfo).sWC.a };
                    '__slate_break_1833: while pTerm < pEnd {
                        if (unsafe { (*pTerm).prereqAll }) & unsafe { (*pLoop).maskSelf }
                            != (((0 as i32) as i64) as u64)
                        {
                            if !((unsafe { (*unsafe { (*pTerm).pExpr }).flags })
                                & ((1 as i32) as u32)
                                != ((0 as i32) as u32))
                                || (unsafe { (*unsafe { (*pTerm).pExpr }).w.iJoin })
                                    != unsafe { (*pItem).iCursor }
                            {
                                break '__slate_break_1833;
                            }
                        }
                        if hasRightJoin != (0 as i32)
                            && (unsafe { (*unsafe { (*pTerm).pExpr }).flags }) & ((2 as i32) as u32)
                                != ((0 as i32) as u32)
                            && (unsafe { (*unsafe { (*pTerm).pExpr }).w.iJoin })
                                == unsafe { (*pItem).iCursor }
                        {
                            // /* restriction (5) */
                            break '__slate_break_1833;
                        }
                        let __v2505: *mut WhereTerm = pTerm;
                        let __v2506: *mut WhereTerm =
                            unsafe { __v2505.offset((1 as i32) as isize) };
                        pTerm = __v2506;
                    }
                    if pTerm < pEnd {
                    } else {
                        {}
                        m1 = ((((1 as i32) as i64) as u64) << i)
                            .wrapping_sub(((1 as i32) as i64) as u64);
                        {}
                        unsafe {
                            (*pWInfo).revMask = m1 & unsafe { (*pWInfo).revMask }
                                | (unsafe { (*pWInfo).revMask }) >> (1 as i32) & !m1;
                        }
                        let __v2507: u64 = notReady;
                        let __v2508: u64 = __v2507 & !unsafe { (*pLoop).maskSelf };
                        notReady = __v2508;
                        pTerm = unsafe { (*pWInfo).sWC.a };
                        '__slate_break_1834: while pTerm < pEnd {
                            if (unsafe { (*pTerm).prereqAll }) & unsafe { (*pLoop).maskSelf }
                                != (((0 as i32) as i64) as u64)
                            {
                                let __v2511: *mut WhereTerm = pTerm;
                                let __v2512: u16 = unsafe { (*__v2511).wtFlags };
                                let __v2513: u16 =
                                    ((((__v2512 as u32) as i32) | (4 as i32)) as i16) as u16;
                                unsafe {
                                    (*__v2511).wtFlags = __v2513;
                                }
                                unsafe {
                                    (*pTerm).prereqAll = ((0 as i32) as i64) as u64;
                                }
                            }
                            let __v2509: *mut WhereTerm = pTerm;
                            let __v2510: *mut WhereTerm =
                                unsafe { __v2509.offset((1 as i32) as isize) };
                            pTerm = __v2510;
                        }
                        if i != (((unsafe { (*pWInfo).nLevel }) as u32) as i32) - (1 as i32) {
                            let mut nByte: i32 =
                                (((((((unsafe { (*pWInfo).nLevel }) as u32) as i32)
                                    - (1 as i32)
                                    - i) as i64) as u64)
                                    .wrapping_mul(120 as u64)
                                    as u32) as i32;
                            unsafe {
                                memmove(
                                    (unsafe {
                                        unsafe {
                                            std::ptr::addr_of_mut!((*pWInfo).a) as *mut WhereLevel
                                        }
                                        .offset(i as isize)
                                    }) as *mut (),
                                    (unsafe {
                                        unsafe {
                                            std::ptr::addr_of_mut!((*pWInfo).a) as *mut WhereLevel
                                        }
                                        .offset((i + (1 as i32)) as isize)
                                    }) as *const (),
                                    (nByte as i64) as u64,
                                )
                            };
                        }
                        let __v2514: *mut WhereInfo = pWInfo;
                        let __v2515: u8 = unsafe { (*__v2514).nLevel };
                        let __v2516: u8 = ((((__v2515 as u32) as i32) - (1 as i32)) as i8) as u8;
                        unsafe {
                            (*__v2514).nLevel = __v2516;
                        }
                        0 as i32;
                    }
                }
            }
        }
        let __v2503: i32 = i;
        let __v2504: i32 = __v2503 - (1 as i32);
        i = __v2504;
    }
    return notReady;
}

// /*
// ** Check to see if there are any SEARCH loops that might benefit from
// ** using a Bloom filter.  Consider a Bloom filter if:
// **
// **   (1)  The SEARCH happens more than N times where N is the number
// **        of rows in the table that is being considered for the Bloom
// **        filter.
// **   (2)  Some searches are expected to find zero rows.  (This is determined
// **        by the WHERE_SELFCULL flag on the term.)
// **   (3)  Bloom-filter processing is not disabled.  (Checked by the
// **        caller.)
// **   (4)  The size of the table being searched is known by ANALYZE.
// **
// ** This block of code merely checks to see if a Bloom filter would be
// ** appropriate, and if so sets the WHERE_BLOOMFILTER flag on the
// ** WhereLoop.  The implementation of the Bloom filter comes further
// ** down where the code for each WhereLoop is generated.
// */
fn whereCheckIfBloomFilterIsUseful(mut pWInfo: *const WhereInfo) {
    let mut i: i32 = 0 as i32;
    let mut nSearch: i16 = (0 as i32) as i16;
    0 as i32;
    0 as i32;
    i = 0 as i32;
    '__slate_break_1835: loop {
        if !(i < (((unsafe { (*pWInfo).nLevel }) as u32) as i32)) {
            break;
        }
        let mut pLoop: *mut WhereLoop = unsafe {
            (*unsafe {
                unsafe { std::ptr::addr_of!((*pWInfo).a) as *const WhereLevel }.offset(i as isize)
            })
            .pWLoop
        };
        let mut reqFlags: u32 = ((8388608 as i32) | (1 as i32)) as u32;
        let mut pItem: *mut SrcItem = unsafe {
            unsafe { std::ptr::addr_of_mut!((*unsafe { (*pWInfo).pTabList }).a) as *mut SrcItem }
                .offset((((unsafe { (*pLoop).iTab }) as u32) as i32) as isize)
        };
        let mut pTab: *mut Table = unsafe { (*pItem).pSTab };
        if (unsafe { (*pTab).tabFlags }) & ((16 as i32) as u32) == ((0 as i32) as u32) {
            break '__slate_break_1835;
        }
        let __v2519: *mut Table = pTab;
        let __v2520: u32 = unsafe { (*__v2519).tabFlags };
        let __v2521: u32 = __v2520 | ((256 as i32) as u32);
        unsafe {
            (*__v2519).tabFlags = __v2521;
        }
        let __v2522: bool;
        if i >= (1 as i32) && (unsafe { (*pLoop).wsFlags }) & reqFlags == reqFlags {
            __v2522 =
                (unsafe { sqlite3WhereLoopBloomable(pLoop as *const WhereLoop) }) != (0 as i32);
        } else {
            __v2522 = false as bool;
        }
        if __v2522 && (nSearch as i32) > ((unsafe { (*pTab).nRowLogEst }) as i32) {
            {}
            let __v2523: *mut WhereLoop = pLoop;
            let __v2524: u32 = unsafe { (*__v2523).wsFlags };
            let __v2525: u32 = __v2524 | ((4194304 as i32) as u32);
            unsafe {
                (*__v2523).wsFlags = __v2525;
            }
            let __v2526: *mut WhereLoop = pLoop;
            let __v2527: u32 = unsafe { (*__v2526).wsFlags };
            let __v2528: u32 = __v2527 & (!(64 as i32) as u32);
            unsafe {
                (*__v2526).wsFlags = __v2528;
            }
            {}
        }
        let __v2529: i16 = nSearch;
        let __v2530: i16 = ((__v2529 as i32) + ((unsafe { (*pLoop).nOut }) as i32)) as i16;
        nSearch = __v2530;
        let __v2517: i32 = i;
        let __v2518: i32 = __v2517 + (1 as i32);
        i = __v2518;
    }
}

// /*
// ** The index pIdx is used by a query and contains one or more expressions.
// ** In other words pIdx is an index on an expression.  iIdxCur is the cursor
// ** number for the index and iDataCur is the cursor number for the corresponding
// ** table.
// **
// ** This routine adds IndexedExpr entries to the Parse->pIdxEpr field for
// ** each of the expressions in the index so that the expression code generator
// ** will know to replace occurrences of the indexed expression with
// ** references to the corresponding column of the index.
// */
fn whereAddIndexedExpr(
    mut pParse: *mut Parse,
    mut pIdx: *mut Index,
    mut iIdxCur: i32,
    mut pTabItem: *mut SrcItem,
) {
    let mut i: i32 = 0 as i32;
    let mut p: *mut IndexedExpr = unsafe { std::mem::zeroed() };
    let mut pTab: *mut Table = unsafe { std::mem::zeroed() };
    0 as i32;
    pTab = unsafe { (*pIdx).pTable };
    i = 0 as i32;
    '__slate_break_1836: loop {
        if !(i < (((unsafe { (*pIdx).nColumn }) as u32) as i32)) {
            break;
        }
        '__slate_continue_1836: {
            let mut pExpr: *mut Expr = unsafe { std::mem::zeroed() };
            let mut j: i32 =
                (unsafe { *unsafe { unsafe { (*pIdx).aiColumn }.offset(i as isize) } }) as i32;
            if j == -(2 as i32) {
                pExpr = unsafe {
                    (*unsafe {
                        unsafe {
                            std::ptr::addr_of_mut!((*unsafe { (*pIdx).aColExpr }).a)
                                as *mut ExprList_item
                        }
                        .offset(i as isize)
                    })
                    .pExpr
                };
            } else {
                if j >= (0 as i32)
                    && (((unsafe {
                        (*unsafe { unsafe { (*pTab).aCol }.offset(j as isize) }).colFlags
                    }) as u32) as i32)
                        & (32 as i32)
                        != (0 as i32)
                {
                    pExpr = unsafe {
                        sqlite3ColumnExpr(pTab, unsafe {
                            unsafe { (*pTab).aCol }.offset(j as isize)
                        })
                    };
                } else {
                    break '__slate_continue_1836;
                }
            }
            if (unsafe { sqlite3ExprIsConstant(std::ptr::null_mut::<Parse>(), pExpr) })
                != (0 as i32)
            {
            } else {
                p = (unsafe { sqlite3DbMallocRaw(unsafe { (*pParse).db }, 32 as u64) })
                    as *mut IndexedExpr;
                if p == std::ptr::null_mut::<IndexedExpr>() {
                    break '__slate_break_1836;
                }
                unsafe {
                    (*p).pIENext = unsafe { (*pParse).pIdxEpr };
                }
                unsafe {
                    (*p).pExpr = unsafe {
                        sqlite3ExprDup(unsafe { (*pParse).db }, pExpr as *const Expr, 0 as i32)
                    };
                }
                unsafe {
                    (*p).iDataCur = unsafe { (*pTabItem).iCursor };
                }
                unsafe {
                    (*p).iIdxCur = iIdxCur;
                }
                unsafe {
                    (*p).iIdxCol = i;
                }
                unsafe {
                    (*p).bMaybeNullRow = ((((unsafe { (*pTabItem).fg.jointype }) as u32) as i32)
                        & ((8 as i32) | (64 as i32) | (16 as i32))
                        != (0 as i32)) as u8;
                }
                if (unsafe { sqlite3IndexAffinityStr(unsafe { (*pParse).db }, pIdx) })
                    != std::ptr::null::<i8>()
                {
                    unsafe {
                        (*p).aff =
                            (unsafe { *unsafe { unsafe { (*pIdx).zColAff }.offset(i as isize) } })
                                as u8;
                    }
                }
                unsafe {
                    (*pParse).pIdxEpr = p;
                }
                if (unsafe { (*p).pIENext }) == std::ptr::null_mut::<IndexedExpr>() {
                    let mut pArg: *mut () =
                        (unsafe { std::ptr::addr_of_mut!((*pParse).pIdxEpr) }) as *mut ();
                    unsafe { sqlite3ParserAddCleanup(pParse, Some(whereIndexedExprCleanup), pArg) };
                }
            }
        }
        let __v2531: i32 = i;
        let __v2532: i32 = __v2531 + (1 as i32);
        i = __v2532;
    }
}

// /* Add IndexedExpr entries to pParse->pIdxEpr */
// /* The index-on-expression that contains the expressions */
// /* Cursor number for pIdx */
// /* The FROM clause entry for the table */
// /*
// ** Set the reverse-scan order mask to one for all tables in the query
// ** with the exception of MATERIALIZED common table expressions that have
// ** their own internal ORDER BY clauses.
// **
// ** This implements the PRAGMA reverse_unordered_selects=ON setting.
// ** (Also SQLITE_DBCONFIG_REVERSE_SCANORDER).
// */
fn whereReverseScanOrder(mut pWInfo: *mut WhereInfo) {
    let mut ii: i32 = 0 as i32;
    ii = 0 as i32;
    '__slate_break_1837: loop {
        if !(ii < unsafe { (*unsafe { (*pWInfo).pTabList }).nSrc }) {
            break;
        }
        let mut pItem: *mut SrcItem = unsafe {
            unsafe { std::ptr::addr_of_mut!((*unsafe { (*pWInfo).pTabList }).a) as *mut SrcItem }
                .offset(ii as isize)
        };
        if !(((unsafe { (*pItem).fg.__slate_bits_0.__get_isCte() }) as i32) != (0 as i32))
            || (((unsafe { (*unsafe { (*pItem).u2.pCteUse }).eM10d }) as u32) as i32) != (0 as i32)
            || ((unsafe { (*pItem).fg.__slate_bits_0.__get_isSubquery() }) as i32) == (0 as i32)
            || (unsafe { (*unsafe { (*unsafe { (*pItem).u4.pSubq }).pSelect }).pOrderBy })
                == std::ptr::null_mut::<ExprList>()
        {
            let __v2535: *mut WhereInfo = pWInfo;
            let __v2536: u64 = unsafe { (*__v2535).revMask };
            let __v2537: u64 = __v2536 | (((1 as i32) as i64) as u64) << ii;
            unsafe {
                (*__v2535).revMask = __v2537;
            }
        }
        let __v2533: i32 = ii;
        let __v2534: i32 = __v2533 + (1 as i32);
        ii = __v2534;
    }
}
