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
//! Main file for the SQLite library.  The routines in this file
//! implement the programmer interface to the library.  Routines in
//! other files are for internal use by SQLite and should not be
//! accessed by users of the library.
unsafe extern "C" {
    static mut sqlite3TreeTrace: u32;
    static mut sqlite3WhereTrace: u32;
    static mut sqlite3StrBINARY: [i8; 0];
    static mut sqlite3StdType: [*const i8; 0];
    static mut sqlite3CtypeMap: [u8; 0];
    static mut sqlite3Config: Sqlite3Config;
    static mut sqlite3BuiltinFunctions: FuncDefHash;
    static mut sqlite3PendingByte: i32;
    fn sqlite3_os_end() -> i32;
    fn sqlite3_mprintf(__v1288: *const i8, ...) -> *mut i8;
    fn sqlite3_malloc64(__v1289: u64) -> *mut ();
    fn sqlite3_free(__v1290: *mut ());
    fn sqlite3_randomness(N: i32, P: *mut ());
    fn sqlite3_value_text(__v1381: *mut sqlite3_value) -> *const u8;
    fn sqlite3_value_text16(__v1382: *mut sqlite3_value) -> *const ();
    fn sqlite3_user_data(__v1383: *mut sqlite3_context) -> *mut ();
    fn sqlite3_result_error(__v1390: *mut sqlite3_context, __v1391: *const i8, __v1392: i32);
    fn sqlite3_reset_auto_extension();
    fn sqlite3_vfs_find(zVfsName: *const i8) -> *mut sqlite3_vfs;
    fn sqlite3_mutex_free(__v1452: *mut sqlite3_mutex);
    fn sqlite3_mutex_enter(__v1453: *mut sqlite3_mutex);
    fn sqlite3_mutex_leave(__v1454: *mut sqlite3_mutex);
    fn sqlite3_strnicmp(__v1461: *const i8, __v1462: *const i8, __v1463: i32) -> i32;
    fn sqlite3_log(iErrCode: i32, zFormat: *const i8, ...);
    fn sqlite3HashInit(__v1480: *mut Hash);
    fn sqlite3HashFind(__v1481: *const Hash, pKey: *const i8) -> *mut ();
    fn sqlite3HashClear(__v1483: *mut Hash);
    fn memcpy(__dest: *mut (), __src: *const (), __n: u64) -> *mut ();
    fn memset(__s: *mut (), __c: i32, __n: u64) -> *mut ();
    fn memcmp(__s1: *const (), __s2: *const (), __n: u64) -> i32;
    fn strcmp(__s1: *const i8, __s2: *const i8) -> i32;
    fn strlen(__s: *const i8) -> u64;
    fn sqlite3OsInit() -> i32;
    fn sqlite3OsFileControl(__v1496: *mut sqlite3_file, __v1497: i32, __v1498: *mut ()) -> i32;
    fn sqlite3OsSleep(__v1499: *mut sqlite3_vfs, __v1500: i32) -> i32;
    fn sqlite3PagerShrink(__v1501: *mut Pager);
    fn sqlite3PagerFlush(__v1502: *mut Pager) -> i32;
    fn sqlite3PagerDataVersion(__v1503: *mut Pager) -> u32;
    fn sqlite3PagerVfs(__v1504: *mut Pager) -> *mut sqlite3_vfs;
    fn sqlite3PagerFile(__v1505: *mut Pager) -> *mut sqlite3_file;
    fn sqlite3PagerJrnlFile(__v1506: *mut Pager) -> *mut sqlite3_file;
    fn sqlite3BtreeOpen(
        pVfs: *mut sqlite3_vfs,
        zFilename: *const i8,
        db: *mut sqlite3,
        ppBtree: *mut *mut Btree,
        flags: i32,
        vfsFlags: i32,
    ) -> i32;
    fn sqlite3BtreeClose(__v1513: *mut Btree) -> i32;
    fn sqlite3BtreeSetPageSize(p: *mut Btree, nPagesize: i32, nReserve: i32, eFix: i32) -> i32;
    fn sqlite3BtreeGetRequestedReserve(__v1518: *mut Btree) -> i32;
    fn sqlite3BtreeRollback(__v1519: *mut Btree, __v1520: i32, __v1521: i32) -> i32;
    fn sqlite3BtreeTxnState(__v1522: *mut Btree) -> i32;
    fn sqlite3BtreeIsInBackup(__v1523: *mut Btree) -> i32;
    fn sqlite3BtreeCheckpoint(
        __v1524: *mut Btree,
        __v1525: i32,
        __v1526: *mut i32,
        __v1527: *mut i32,
    ) -> i32;
    fn sqlite3BtreeGetFilename(__v1528: *mut Btree) -> *const i8;
    fn sqlite3BtreePager(__v1529: *mut Btree) -> *mut Pager;
    fn sqlite3BtreeIsReadonly(pBt: *mut Btree) -> i32;
    fn sqlite3HeaderSizeBtree() -> i32;
    fn sqlite3BtreeClearCache(__v1535: *mut Btree);
    fn sqlite3BtreeEnter(__v1536: *mut Btree);
    fn sqlite3BtreeEnterAll(__v1537: *mut sqlite3);
    fn sqlite3BtreeLeave(__v1538: *mut Btree);
    fn sqlite3BtreeLeaveAll(__v1539: *mut sqlite3);
    fn sqlite3VdbeBytecodeVtabInit(__v1540: *mut sqlite3) -> i32;
    fn sqlite3PcacheInitialize() -> i32;
    fn sqlite3PcacheShutdown();
    fn sqlite3PCacheBufferSetup(__v1541: *mut (), sz: i32, n: i32);
    fn sqlite3PCacheSetDefault();
    fn sqlite3HeaderSizePcache() -> i32;
    fn sqlite3HeaderSizePcache1() -> i32;
    fn sqlite3IsIdChar(__v1550: u8) -> i32;
    fn sqlite3Strlen30(__v1551: *const i8) -> i32;
    fn sqlite3ColumnType(__v1552: *mut Column, __v1553: *mut i8) -> *mut i8;
    fn sqlite3MallocInit() -> i32;
    fn sqlite3MallocEnd();
    fn sqlite3Malloc(__v1554: u64) -> *mut ();
    fn sqlite3MallocZero(__v1555: u64) -> *mut ();
    fn sqlite3DbFree(__v1556: *mut sqlite3, __v1557: *mut ());
    fn sqlite3MallocSize(__v1558: *const ()) -> i32;
    fn sqlite3MemSetDefault();
    fn sqlite3BenignMallocHooks(
        __v1559: Option<unsafe extern "C-unwind" fn()>,
        __v1560: Option<unsafe extern "C-unwind" fn()>,
    );
    fn sqlite3MutexAlloc(__v1561: i32) -> *mut sqlite3_mutex;
    fn sqlite3MutexInit() -> i32;
    fn sqlite3MutexEnd() -> i32;
    fn sqlite3MemoryBarrier();
    fn sqlite3LookasideUsed(__v1562: *mut sqlite3, __v1563: *mut i32) -> i32;
    fn sqlite3MPrintf(__v1564: *mut sqlite3, __v1565: *const i8, ...) -> *mut i8;
    fn sqlite3Init(__v1566: *mut sqlite3, __v1567: *mut *mut i8) -> i32;
    fn sqlite3ResetAllSchemasOfConnection(__v1568: *mut sqlite3);
    fn sqlite3CollapseDatabaseArray(__v1569: *mut sqlite3);
    fn sqlite3ColumnColl(__v1570: *mut Column) -> *const i8;
    fn sqlite3FaultSim(__v1579: i32) -> i32;
    fn sqlite3BitvecBuiltinTest(__v1580: i32, __v1581: *mut i32) -> i32;
    fn sqlite3FindTable(
        __v1582: *mut sqlite3,
        __v1583: *const i8,
        __v1584: *const i8,
    ) -> *mut Table;
    fn sqlite3PrngSaveState();
    fn sqlite3PrngRestoreState();
    fn sqlite3IsRowid(__v1589: *const i8) -> i32;
    fn sqlite3FindFunction(
        __v1590: *mut sqlite3,
        __v1591: *const i8,
        __v1592: i32,
        __v1593: u8,
        __v1594: u8,
    ) -> *mut FuncDef;
    fn sqlite3RegisterBuiltinFunctions();
    fn sqlite3RegisterPerConnectionBuiltinFunctions(__v1595: *mut sqlite3);
    fn sqlite3SafetyCheckOk(__v1596: *mut sqlite3) -> i32;
    fn sqlite3SafetyCheckSickOrOk(__v1597: *mut sqlite3) -> i32;
    fn sqlite3ColumnIndex(pTab: *mut Table, zCol: *const i8) -> i32;
    fn sqlite3AtoF(z: *const i8, __v1601: *mut f64) -> i32;
    fn sqlite3LogEst(__v1602: u64) -> i16;
    fn sqlite3LogEstFromDouble(__v1603: f64) -> i16;
    fn sqlite3LogEstToInt(__v1604: i16) -> u64;
    fn sqlite3DecOrHexToI64(__v1605: *const i8, __v1606: *mut i64) -> i32;
    fn sqlite3ErrorWithMsg(__v1607: *mut sqlite3, __v1608: i32, __v1609: *const i8, ...);
    fn sqlite3Error(__v1610: *mut sqlite3, __v1611: i32);
    fn sqlite3HexToInt(h: i32) -> u8;
    fn sqlite3MemdbInit() -> i32;
    fn sqlite3FindCollSeq(
        __v1614: *mut sqlite3,
        enc: u8,
        __v1616: *const i8,
        __v1617: i32,
    ) -> *mut CollSeq;
    fn sqlite3SetTextEncoding(db: *mut sqlite3, __v1620: u8);
    fn sqlite3GetBoolean(z: *const i8, __v1622: u8) -> u8;
    fn sqlite3ValueText(__v1623: *mut sqlite3_value, __v1624: u8) -> *const ();
    fn sqlite3ValueSetStr(
        __v1625: *mut sqlite3_value,
        __v1626: i32,
        __v1627: *const (),
        __v1628: u8,
        __v1629: Option<unsafe extern "C-unwind" fn(*mut ())>,
    );
    fn sqlite3ValueFree(__v1630: *mut sqlite3_value);
    fn sqlite3ResultIntReal(__v1631: *mut sqlite3_context);
    fn sqlite3ValueNew(__v1632: *mut sqlite3) -> *mut sqlite3_value;
    fn sqlite3Utf16to8(
        __v1633: *mut sqlite3,
        __v1634: *const (),
        __v1635: i32,
        __v1636: u8,
    ) -> *mut i8;
    fn sqlite3ExpirePreparedStatements(__v1637: *mut sqlite3, __v1638: i32);
    fn sqlite3FindDbName(__v1640: *mut sqlite3, __v1641: *const i8) -> i32;
    fn sqlite3SchemaClear(__v1642: *mut ());
    fn sqlite3SchemaGet(__v1643: *mut sqlite3, __v1644: *mut Btree) -> *mut Schema;
    fn sqlite3OomFault(__v1656: *mut sqlite3) -> *mut ();
    fn sqlite3OomClear(__v1657: *mut sqlite3);
    fn sqlite3ApiExit(db: *mut sqlite3, __v1659: i32) -> i32;
    fn sqlite3AutoLoadExtensions(__v1660: *mut sqlite3);
    fn sqlite3CloseExtensions(__v1661: *mut sqlite3);
    fn sqlite3VtabDisconnect(db: *mut sqlite3, p: *mut Table);
    fn sqlite3VtabRollback(db: *mut sqlite3) -> i32;
    fn sqlite3VtabModuleUnref(__v1665: *mut sqlite3, __v1666: *mut Module);
    fn sqlite3VtabUnlockList(__v1667: *mut sqlite3);
    fn sqlite3VtabEponymousTableClear(__v1668: *mut sqlite3, __v1669: *mut Module);
    fn sqlite3BeginBenignMalloc();
    fn sqlite3EndBenignMalloc();
    fn sqlite3DbpageRegister(__v1680: *mut sqlite3) -> i32;
    fn sqlite3DbstatRegister(__v1681: *mut sqlite3) -> i32;
    fn sqlite3CompileOptions(pnOpt: *mut i32) -> *mut *const i8;
    fn sqlite3Fts3Init(db: *mut sqlite3) -> i32;
    fn sqlite3RtreeInit(db: *mut sqlite3) -> i32;
    fn sqlite3StmtVtabInit(__v1685: *mut sqlite3) -> i32;
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
struct sqlite3_pcache {}

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
    trace: __SlateRecord173,
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
    u1: __SlateRecord174,
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
    u: __SlateRecord175,
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
    __slate_bits_0: __slate_bits::__SlateBits76U0,
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
    u: __SlateRecord176,
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
    __slate_bits_0: __slate_bits::__SlateBits102U0,
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
    u: __SlateRecord184,
    pLeft: *mut Expr,
    pRight: *mut Expr,
    x: __SlateRecord185,
    nHeight: i32,
    iTable: i32,
    iColumn: i16,
    iAgg: i16,
    w: __SlateRecord186,
    pAggInfo: *mut AggInfo,
    y: __SlateRecord187,
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
    fg: __SlateRecord194,
    iCursor: i32,
    colUsed: u64,
    u1: __SlateRecord195,
    u2: __SlateRecord196,
    u3: __SlateRecord197,
    u4: __SlateRecord198,
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
    __slate_bits_0: __slate_bits::__SlateBits114U0,
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
    u1: __SlateRecord200,
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
struct Vdbe {}

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
    __slate_bits_0: __slate_bits::__SlateBits172U0,
    azInit: *mut *const i8,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord173 {
    xLegacy: Option<unsafe extern "C-unwind" fn(*mut (), *const i8)>,
    xV2: Option<unsafe extern "C-unwind" fn(u32, *mut (), *mut (), *mut ()) -> i32>,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord174 {
    isInterrupted: i32,
    notUsed1: f64,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord175 {
    pHash: *mut FuncDef,
    pDestructor: *mut FuncDestructor,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord176 {
    tab: __SlateRecord177,
    view: __SlateRecord178,
    vtab: __SlateRecord179,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord177 {
    addColOffset: i32,
    pFKey: *mut FKey,
    pDfltList: *mut ExprList,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord178 {
    pSelect: *mut Select,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord179 {
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
union __SlateRecord184 {
    zToken: *mut i8,
    iValue: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord185 {
    pList: *mut ExprList,
    pSelect: *mut Select,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord186 {
    iJoin: i32,
    iOfst: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord187 {
    pTab: *mut Table,
    pWin: *mut Window,
    nReg: i32,
    sub: __SlateRecord188,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord188 {
    iAddr: i32,
    regReturn: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct ExprList_item {
    pExpr: *mut Expr,
    zEName: *mut i8,
    fg: __SlateRecord190,
    u: __SlateRecord191,
}

#[repr(C, align(4))]
#[derive(Clone, Copy)]
struct __SlateRecord190 {
    sortFlags: u8,
    __slate_bits_0: __slate_bits::__SlateBits190U0,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord191 {
    x: __SlateRecord192,
    iConstExprReg: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord192 {
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
struct __SlateRecord194 {
    jointype: u8,
    __slate_bits_0: __slate_bits::__SlateBits194U0,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord195 {
    zIndexedBy: *mut i8,
    pFuncArg: *mut ExprList,
    nRow: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord196 {
    pIBIndex: *mut Index,
    pCteUse: *mut CteUse,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord197 {
    pOn: *mut Expr,
    pUsing: *mut IdList,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord198 {
    pSchema: *mut Schema,
    zDatabase: *mut i8,
    pSubq: *mut Subquery,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord200 {
    cr: __SlateRecord201,
    d: __SlateRecord202,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord201 {
    addrCrTab: i32,
    regRowid: i32,
    regRoot: i32,
    constraintName: Token,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord202 {
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

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord205 {
    /// The opcode
    op: i32,
    /// Mask of the bit in sqlite3.flags to set/clear
    mask: u64,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct OpenMode {
    z: *const i8,
    mode: i32,
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
    pub struct __SlateBits76U0 {
        #[bits(4)]
        pub notNull: u32,
        #[bits(4)]
        pub eCType: u32,
    }
    #[bitfields::bitfield([u8; 2], c_names = true, new = false, from_into_bits = false, from_traits = false, default = false, debug = false, builder = false, bit_ops = false)]
    pub struct __SlateBits190U0 {
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
    pub struct __SlateBits194U0 {
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
    pub struct __SlateBits102U0 {
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
    pub struct __SlateBits172U0 {
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
    pub struct __SlateBits114U0 {
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

/// This is an extension initializer that is a no-op and always
/// succeeds, except that it fails if the fault-simulation is set
/// to 500.
#[unsafe(link_section = ".text.slate_distinct.main.sqlite3TestExtInit")]
extern "C-unwind" fn sqlite3TestExtInit(mut db: *mut sqlite3) -> i32 {
    db;
    return unsafe { sqlite3FaultSim(500 as i32) };
}

// Forward declarations of external module initializer functions
// for modules that need them.
/// An array of pointers to extension initializer functions for
/// built-in extensions.
static mut sqlite3BuiltinExtensions: __SlateAlign16<
    [Option<unsafe extern "C-unwind" fn(*mut sqlite3) -> i32>; 7],
> = __SlateAlign16([
    unsafe {
        std::mem::transmute::<*const (), Option<unsafe extern "C-unwind" fn(*mut sqlite3) -> i32>>(
            sqlite3Fts3Init as *const (),
        )
    },
    unsafe {
        std::mem::transmute::<*const (), Option<unsafe extern "C-unwind" fn(*mut sqlite3) -> i32>>(
            sqlite3RtreeInit as *const (),
        )
    },
    unsafe {
        std::mem::transmute::<*const (), Option<unsafe extern "C-unwind" fn(*mut sqlite3) -> i32>>(
            sqlite3DbpageRegister as *const (),
        )
    },
    unsafe {
        std::mem::transmute::<*const (), Option<unsafe extern "C-unwind" fn(*mut sqlite3) -> i32>>(
            sqlite3DbstatRegister as *const (),
        )
    },
    Some(sqlite3TestExtInit),
    unsafe {
        std::mem::transmute::<*const (), Option<unsafe extern "C-unwind" fn(*mut sqlite3) -> i32>>(
            sqlite3StmtVtabInit as *const (),
        )
    },
    unsafe {
        std::mem::transmute::<*const (), Option<unsafe extern "C-unwind" fn(*mut sqlite3) -> i32>>(
            sqlite3VdbeBytecodeVtabInit as *const (),
        )
    },
]);

/// IMPLEMENTATION-OF: R-46656-45156 The sqlite3_version[] string constant
/// contains the text of SQLITE_VERSION macro.
#[unsafe(no_mangle)]
static mut sqlite3_version: [i8; 7] = [
    51 as i8, 46 as i8, 53 as i8, 52 as i8, 46 as i8, 48 as i8, 0 as i8,
];

/// IMPLEMENTATION-OF: R-53536-42575 The sqlite3_libversion() function returns
/// a pointer to the to the sqlite3_version[] string constant.
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.main.sqlite3_libversion")]
extern "C-unwind" fn sqlite3_libversion() -> *const i8 {
    return unsafe { std::ptr::addr_of!(sqlite3_version) as *const i8 };
}

/// IMPLEMENTATION-OF: R-25063-23286 The sqlite3_sourceid() function returns a
/// pointer to a string constant whose value is the same as the
/// SQLITE_SOURCE_ID C preprocessor macro. Except if SQLite is built using
/// an edited copy of the amalgamation, then the last four characters of
/// the hash might be different from SQLITE_SOURCE_ID.
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.main.sqlite3_sourceid")]
extern "C-unwind" fn sqlite3_sourceid() -> *const i8 {
    return (b"2026-08-22 19:27:30 db0cb462aaf2014cfe8cfc90f7cddda07458a5439b2154dc2781420154bd3098\0".as_ptr() as *mut i8) as *const i8;
}

/// IMPLEMENTATION-OF: R-35210-63508 The sqlite3_libversion_number() function
/// returns an integer equal to SQLITE_VERSION_NUMBER.
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.main.sqlite3_libversion_number")]
extern "C-unwind" fn sqlite3_libversion_number() -> i32 {
    return 3054000 as i32;
}

/// IMPLEMENTATION-OF: R-20790-14025 The sqlite3_threadsafe() function returns
/// zero if and only if SQLite was compiled with mutexing code omitted due to
/// the SQLITE_THREADSAFE compile-time option being set to 0.
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.main.sqlite3_threadsafe")]
extern "C-unwind" fn sqlite3_threadsafe() -> i32 {
    return 1 as i32;
}

// When compiling the test fixture or with debugging enabled (on Win32),
// this variable being set to non-zero will cause OSTRACE macros to emit
// extra diagnostic information.
/// If the following global variable points to a string which is the
/// name of a directory, then that directory will be used to store
/// temporary files.
///
/// See also the "PRAGMA temp_store_directory" SQL command.
#[unsafe(no_mangle)]
static mut sqlite3_temp_directory: *mut i8 = std::ptr::null_mut::<i8>();

/// If the following global variable points to a string which is the
/// name of a directory, then that directory will be used to store
/// all database files specified with a relative pathname.
///
/// See also the "PRAGMA data_store_directory" SQL command.
#[unsafe(no_mangle)]
static mut sqlite3_data_directory: *mut i8 = std::ptr::null_mut::<i8>();

/// Initialize SQLite.
///
/// The sqlite3_initialize() routine must be called to initialize the
/// memory allocation, VFS, and mutex subsystems prior to doing any
/// serious work.  As long as you do not compile with SQLITE_OMIT_AUTOINIT
/// this routine will be called automatically by key routines such as
/// sqlite3_open().
///
/// This routine is a no-op except on its very first call for the process,
/// or for the first call after a call to sqlite3_shutdown.  Most calls
/// to sqlite3_initialize() are, in fact, no-ops.  For that reason, the
/// routine is broken into two pieces:
///
///     sqlite3Initialize()     Does the actual work of initialization
///
///     sqlite3_initialize()    Checks to see if initialization is needed
///                             and invokes sqlite3Initialize() if it is.
///
/// The sqlite3_initialize() interface is called frequently, but
/// sqlite3Initialize() runs rarely.  The function is broken up this way
/// to avoid wasting CPU cycles with unnecessary stack setup for local
/// variables in cases where it is not needed.
///
/// The first thread to call sqlite3_initialize() runs the initialization to
/// completion. If subsequent threads call sqlite3_initialize() before the first
/// thread has finished the initialization process, then the subsequent
/// threads must block until the first thread finishes with the initialization.
///
/// The first thread might call this routine recursively.  Recursive
/// calls to this routine should not block, of course.  Otherwise the
/// initialization process would never complete.
///
/// Let X be the first thread to enter sqlite3_initialize().  Let Y be some
/// other thread.  While the initial invocation of sqlite3_initialize() by X
/// is incomplete, it is required that:
///
///    *  Calls to sqlite3_initialize() from Y must block until the outer-most
///       call by X completes.
///
///    *  Recursive calls to sqlite3_initialize() from thread X return
///       immediately without blocking.
fn sqlite3Initialize() -> i32 {
    let mut pMainMtx: *mut sqlite3_mutex = unsafe { std::mem::zeroed() }; // The main static mutex
    let mut rc: i32 = 0 as i32; // Result code
    // If the following assert() fails on some obscure processor/compiler
    // combination to warn that SQLite has been mis-compiled.  If you hit
    // this assert(), that means you need to recompile with the
    // -DSQLITE_PTRSIZE=n compile-time option to set the correct pointer size.
    0 as i32;
    // Make sure the mutex subsystem is initialized.  If unable to
    // initialize the mutex subsystem, return early with the error.
    // If the system is so sick that we are unable to allocate a mutex,
    // there is not much SQLite is going to be able to do.
    //
    // The mutex subsystem must take care of serializing its own
    // initialization.
    rc = unsafe { sqlite3MutexInit() };
    if rc != (0 as i32) {
        return rc;
    }
    // Initialize the malloc() system and the recursive pInitMutex mutex.
    // This operation is protected by the STATIC_MAIN mutex.  Note that
    // MutexAlloc() is called for a static mutex prior to initializing the
    // malloc subsystem - this implies that the allocation of a static
    // mutex must not require support from the malloc subsystem.
    pMainMtx = unsafe { sqlite3MutexAlloc(2 as i32) };
    unsafe { sqlite3_mutex_enter(pMainMtx) };
    unsafe {
        sqlite3Config.isMutexInit = 1 as i32;
    }
    if !((unsafe { sqlite3Config.isMallocInit }) != (0 as i32)) {
        rc = unsafe { sqlite3MallocInit() };
    }
    if rc == (0 as i32) {
        unsafe {
            sqlite3Config.isMallocInit = 1 as i32;
        }
        if !((unsafe { sqlite3Config.pInitMutex }) != std::ptr::null_mut::<sqlite3_mutex>()) {
            unsafe {
                sqlite3Config.pInitMutex = unsafe { sqlite3MutexAlloc(1 as i32) };
            }
            if (unsafe { sqlite3Config.bCoreMutex }) != (0 as u8)
                && !((unsafe { sqlite3Config.pInitMutex }) != std::ptr::null_mut::<sqlite3_mutex>())
            {
                rc = 7 as i32;
            }
        }
    }
    if rc == (0 as i32) {
        let __v1944: i32 = unsafe { sqlite3Config.nRefInitMutex };
        let __v1945: i32 = __v1944 + (1 as i32);
        unsafe {
            sqlite3Config.nRefInitMutex = __v1945;
        }
    }
    unsafe { sqlite3_mutex_leave(pMainMtx) };
    // If rc is not SQLITE_OK at this point, then either the malloc
    // subsystem could not be initialized or the system failed to allocate
    // the pInitMutex mutex. Return an error in either case.
    if rc != (0 as i32) {
        return rc;
    }
    // Do the rest of the initialization under the recursive mutex so
    // that we will be able to handle recursive calls into
    // sqlite3_initialize().  The recursive calls normally come through
    // sqlite3_os_init() when it invokes sqlite3_vfs_register(), but other
    // recursive calls might also be possible.
    //
    // IMPLEMENTATION-OF: R-00140-37445 SQLite automatically serializes calls
    // to the xInit method, so the xInit method need not be threadsafe.
    //
    // The following mutex is what serializes access to the appdef pcache xInit
    // methods.  The sqlite3_pcache_methods.xInit() all is embedded in the
    // call to sqlite3PcacheInitialize().
    unsafe { sqlite3_mutex_enter(unsafe { sqlite3Config.pInitMutex }) };
    if (unsafe { sqlite3Config.isInit }) == (0 as i32)
        && (unsafe { sqlite3Config.inProgress }) == (0 as i32)
    {
        unsafe {
            sqlite3Config.inProgress = 1 as i32;
        }
        unsafe {
            memset(
                (unsafe { std::ptr::addr_of_mut!(sqlite3BuiltinFunctions) }) as *mut (),
                0 as i32,
                184 as u64,
            )
        };
        unsafe { sqlite3RegisterBuiltinFunctions() };
        if (unsafe { sqlite3Config.isPCacheInit }) == (0 as i32) {
            rc = unsafe { sqlite3PcacheInitialize() };
        }
        if rc == (0 as i32) {
            unsafe {
                sqlite3Config.isPCacheInit = 1 as i32;
            }
            rc = unsafe { sqlite3OsInit() };
        }
        if rc == (0 as i32) {
            rc = unsafe { sqlite3MemdbInit() };
        }
        if rc == (0 as i32) {
            unsafe {
                sqlite3PCacheBufferSetup(
                    unsafe { sqlite3Config.pPage },
                    unsafe { sqlite3Config.szPage },
                    unsafe { sqlite3Config.nPage },
                )
            };
        }
        if rc == (0 as i32) {
            unsafe { sqlite3MemoryBarrier() };
            unsafe {
                sqlite3Config.isInit = 1 as i32;
            }
        }
        unsafe {
            sqlite3Config.inProgress = 0 as i32;
        }
    }
    unsafe { sqlite3_mutex_leave(unsafe { sqlite3Config.pInitMutex }) };
    // Go back under the static mutex and clean up the recursive
    // mutex to prevent a resource leak.
    unsafe { sqlite3_mutex_enter(pMainMtx) };
    let __v1946: i32 = unsafe { sqlite3Config.nRefInitMutex };
    let __v1947: i32 = __v1946 - (1 as i32);
    unsafe {
        sqlite3Config.nRefInitMutex = __v1947;
    }
    if (unsafe { sqlite3Config.nRefInitMutex }) <= (0 as i32) {
        0 as i32;
        unsafe { sqlite3_mutex_free(unsafe { sqlite3Config.pInitMutex }) };
        unsafe {
            sqlite3Config.pInitMutex = std::ptr::null_mut::<sqlite3_mutex>();
        }
    }
    unsafe { sqlite3_mutex_leave(pMainMtx) };
    // The following is just a sanity check to make sure SQLite has
    // been compiled correctly.  It is important to run this code, but
    // we don't want to run it too often and soak up CPU cycles for no
    // reason.  So we run it once during initialization.
    //
    // Do extra initialization steps requested by the SQLITE_EXTRA_INIT
    // compile-time option.
    return rc;
}

#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3_initialize() -> i32 {
    // If this build does not support writable static data (WSD) natively
    // then we have to invoke the (application-supplied) WSD initialization
    // routine before doing anything else.
    if (unsafe { sqlite3Config.isInit }) != (0 as i32) {
        // SQLite has already been initialized.  Fast early-out.
        unsafe { sqlite3MemoryBarrier() };
        return 0 as i32;
    } else {
        // Invoke sqlite3Initialize() to do the actual work.
        return sqlite3Initialize();
    }
    return unsafe { std::mem::zeroed() };
}

/// Undo the effects of sqlite3_initialize().  Must not be called while
/// there are outstanding database connections or memory allocations or
/// while any part of SQLite is otherwise in use in any thread.  This
/// routine is not threadsafe.  But it is safe to invoke this routine
/// on when SQLite is already shut down.  If SQLite is already shut down
/// when this routine is invoked, then this routine is a harmless no-op.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3_shutdown() -> i32 {
    if (unsafe { sqlite3Config.isInit }) != (0 as i32) {
        unsafe { sqlite3_os_end() };
        unsafe { sqlite3_reset_auto_extension() };
        unsafe {
            sqlite3Config.isInit = 0 as i32;
        }
    }
    if (unsafe { sqlite3Config.isPCacheInit }) != (0 as i32) {
        unsafe { sqlite3PcacheShutdown() };
        unsafe {
            sqlite3Config.isPCacheInit = 0 as i32;
        }
    }
    if (unsafe { sqlite3Config.isMallocInit }) != (0 as i32) {
        unsafe { sqlite3MallocEnd() };
        unsafe {
            sqlite3Config.isMallocInit = 0 as i32;
        }
        // The heap subsystem has now been shutdown and these values are supposed
        // to be NULL or point to memory that was obtained from sqlite3_malloc(),
        // which would rely on that heap subsystem; therefore, make sure these
        // values cannot refer to heap memory that was just invalidated when the
        // heap subsystem was shutdown.  This is only done if the current call to
        // this function resulted in the heap subsystem actually being shutdown.
        unsafe {
            sqlite3_data_directory = std::ptr::null_mut::<i8>();
        }
        unsafe {
            sqlite3_temp_directory = std::ptr::null_mut::<i8>();
        }
    }
    if (unsafe { sqlite3Config.isMutexInit }) != (0 as i32) {
        unsafe { sqlite3MutexEnd() };
        unsafe {
            sqlite3Config.isMutexInit = 0 as i32;
        }
    }
    return 0 as i32;
}

/// This API allows applications to modify the global configuration of
/// the SQLite library at run-time.
///
/// This routine should only be called when there are no outstanding
/// database connections or memory allocations.  This routine is not
/// threadsafe.  Failure to heed these warnings can lead to unpredictable
/// behavior.
#[unsafe(no_mangle)]
unsafe extern "C-unwind" fn sqlite3_config(mut op: i32, mut __va_args: ...) -> i32 {
    let mut ap: core::ffi::VaList<'_> = unsafe { std::mem::zeroed() };
    let mut rc: i32 = 0 as i32;
    // sqlite3_config() normally returns SQLITE_MISUSE if it is invoked while
    // the SQLite library is in use.  Except, a few selected opcodes
    // are allowed.
    if (unsafe { sqlite3Config.isInit }) != (0 as i32) {
        if op < (0 as i32)
            || op > (63 as i32)
            || (((1 as i32) as i64) as u64) << op & unsafe { mAnytimeConfigOption }
                == (((0 as i32) as i64) as u64)
        {
            return sqlite3MisuseError(457 as i32);
        }
        {}
        {}
    }
    ap = __va_args.clone();
    '__slate_break_1687: {
        match op {
            1 => {
                // Mutex configuration options are only available in a threadsafe
                // compile.
                // EVIDENCE-OF: R-02748-19096 This option sets the threading mode to
                // Single-thread.
                unsafe {
                    sqlite3Config.bCoreMutex = ((0 as i32) as i8) as u8;
                }
                // Disable mutex on core
                unsafe {
                    sqlite3Config.bFullMutex = ((0 as i32) as i8) as u8;
                }
                // Disable mutex on connections
            }
            2 => {
                // EVIDENCE-OF: R-14374-42468 This option sets the threading mode to
                // Multi-thread.
                unsafe {
                    sqlite3Config.bCoreMutex = ((1 as i32) as i8) as u8;
                }
                // Enable mutex on core
                unsafe {
                    sqlite3Config.bFullMutex = ((0 as i32) as i8) as u8;
                }
                // Disable mutex on connections
            }
            3 => {
                // EVIDENCE-OF: R-41220-51800 This option sets the threading mode to
                // Serialized.
                unsafe {
                    sqlite3Config.bCoreMutex = ((1 as i32) as i8) as u8;
                }
                // Enable mutex on core
                unsafe {
                    sqlite3Config.bFullMutex = ((1 as i32) as i8) as u8;
                }
                // Enable mutex on connections
            }
            10 => {
                // Specify an alternative mutex implementation
                unsafe {
                    sqlite3Config.mutex =
                        unsafe { *unsafe { ap.next_arg::<*mut sqlite3_mutex_methods>() } };
                }
            }
            11 => {
                // Retrieve the current mutex implementation
                unsafe {
                    *unsafe { ap.next_arg::<*mut sqlite3_mutex_methods>() } =
                        unsafe { sqlite3Config.mutex };
                }
            }
            4 => {
                // EVIDENCE-OF: R-55594-21030 The SQLITE_CONFIG_MALLOC option takes a
                // single argument which is a pointer to an instance of the
                // sqlite3_mem_methods structure. The argument specifies alternative
                // low-level memory allocation routines to be used in place of the memory
                // allocation routines built into SQLite.
                unsafe {
                    sqlite3Config.m =
                        unsafe { *unsafe { ap.next_arg::<*mut sqlite3_mem_methods>() } };
                }
            }
            5 => {
                // EVIDENCE-OF: R-51213-46414 The SQLITE_CONFIG_GETMALLOC option takes a
                // single argument which is a pointer to an instance of the
                // sqlite3_mem_methods structure. The sqlite3_mem_methods structure is
                // filled with the currently defined memory allocation routines.
                if (unsafe { sqlite3Config.m.xMalloc }) == None {
                    unsafe { sqlite3MemSetDefault() };
                }
                unsafe {
                    *unsafe { ap.next_arg::<*mut sqlite3_mem_methods>() } =
                        unsafe { sqlite3Config.m };
                }
            }
            9 => {
                0 as i32; // Cannot change at runtime
                // EVIDENCE-OF: R-61275-35157 The SQLITE_CONFIG_MEMSTATUS option takes
                // single argument of type int, interpreted as a boolean, which enables
                // or disables the collection of memory allocation statistics.
                unsafe {
                    sqlite3Config.bMemstat = unsafe { ap.next_arg::<i32>() };
                }
            }
            27 => unsafe {
                sqlite3Config.bSmallMalloc =
                    ((unsafe { ap.next_arg::<i32>() }) != (0 as i32)) as u8;
            },
            7 => {
                // EVIDENCE-OF: R-18761-36601 There are three arguments to
                // SQLITE_CONFIG_PAGECACHE: A pointer to 8-byte aligned memory (pMem),
                // the size of each page cache line (sz), and the number of cache lines
                // (N).
                unsafe {
                    sqlite3Config.pPage = unsafe { ap.next_arg::<*mut ()>() };
                }
                unsafe {
                    sqlite3Config.szPage = unsafe { ap.next_arg::<i32>() };
                }
                unsafe {
                    sqlite3Config.nPage = unsafe { ap.next_arg::<i32>() };
                }
            }
            24 => {
                // EVIDENCE-OF: R-39100-27317 The SQLITE_CONFIG_PCACHE_HDRSZ option takes
                // a single parameter which is a pointer to an integer and writes into
                // that integer the number of extra bytes per page required for each page
                // in SQLITE_CONFIG_PAGECACHE.
                unsafe {
                    *unsafe { ap.next_arg::<*mut i32>() } = (unsafe { sqlite3HeaderSizeBtree() })
                        + unsafe { sqlite3HeaderSizePcache() }
                        + unsafe { sqlite3HeaderSizePcache1() };
                }
            }
            14 => {
                // no-op
            }
            15 => {
                // now an error
                rc = 1 as i32;
            }
            18 => {
                // EVIDENCE-OF: R-63325-48378 The SQLITE_CONFIG_PCACHE2 option takes a
                // single argument which is a pointer to an sqlite3_pcache_methods2
                // object. This object specifies the interface to a custom page cache
                // implementation.
                unsafe {
                    sqlite3Config.pcache2 =
                        unsafe { *unsafe { ap.next_arg::<*mut sqlite3_pcache_methods2>() } };
                }
            }
            19 => {
                // EVIDENCE-OF: R-22035-46182 The SQLITE_CONFIG_GETPCACHE2 option takes a
                // single argument which is a pointer to an sqlite3_pcache_methods2
                // object. SQLite copies of the current page cache implementation into
                // that object.
                if (unsafe { sqlite3Config.pcache2.xInit }) == None {
                    unsafe { sqlite3PCacheSetDefault() };
                }
                unsafe {
                    *unsafe { ap.next_arg::<*mut sqlite3_pcache_methods2>() } =
                        unsafe { sqlite3Config.pcache2 };
                }
                break '__slate_break_1687;
                // EVIDENCE-OF: R-06626-12911 The SQLITE_CONFIG_HEAP option is only
                // available if SQLite is compiled with either SQLITE_ENABLE_MEMSYS3 or
                // SQLITE_ENABLE_MEMSYS5 and returns SQLITE_ERROR if invoked otherwise.
            }
            13 => {
                unsafe {
                    sqlite3Config.szLookaside = unsafe { ap.next_arg::<i32>() };
                }
                unsafe {
                    sqlite3Config.nLookaside = unsafe { ap.next_arg::<i32>() };
                }
                break '__slate_break_1687;
                // Record a pointer to the logger function and its first argument.
                // The default is NULL.  Logging is disabled if the function pointer is
                // NULL.
            }
            16 => {
                // MSVC is picky about pulling func ptrs from va lists.
                // http://support.microsoft.com/kb/47961
                // sqlite3GlobalConfig.xLog = va_arg(ap, void(*)(void*,int,const char*));
                let mut xLog: Option<unsafe extern "C-unwind" fn(*mut (), i32, *const i8)> = unsafe {
                    unsafe {
                        std::mem::transmute::<
                            *const u8,
                            Option<unsafe extern "C-unwind" fn(*mut (), i32, *const i8)>,
                        >(ap.next_arg::<*const u8>())
                    }
                };
                let mut pLogArg: *mut () = unsafe { ap.next_arg::<*mut ()>() };
                unsafe {
                    std::sync::atomic::AtomicPtr::from_ptr(
                        (unsafe { std::ptr::addr_of_mut!(sqlite3Config.xLog) }) as *mut *mut u8,
                    )
                    .store(
                        unsafe {
                            std::mem::transmute::<
                                Option<unsafe extern "C-unwind" fn(*mut (), i32, *const i8)>,
                                *mut u8,
                            >(xLog)
                        },
                        std::sync::atomic::Ordering::Relaxed,
                    )
                };
                unsafe {
                    std::sync::atomic::AtomicPtr::from_ptr(
                        (unsafe { std::ptr::addr_of_mut!(sqlite3Config.pLogArg) }) as *mut *mut (),
                    )
                    .store(pLogArg as *mut (), std::sync::atomic::Ordering::Relaxed)
                };
                break '__slate_break_1687;
                // EVIDENCE-OF: R-55548-33817 The compile-time setting for URI filenames
                // can be changed at start-time using the
                // sqlite3_config(SQLITE_CONFIG_URI,1) or
                // sqlite3_config(SQLITE_CONFIG_URI,0) configuration calls.
            }
            17 => {
                // EVIDENCE-OF: R-25451-61125 The SQLITE_CONFIG_URI option takes a single
                // argument of type int. If non-zero, then URI handling is globally
                // enabled. If the parameter is zero, then URI handling is globally
                // disabled.
                let mut bOpenUri: i32 = unsafe { ap.next_arg::<i32>() };
                unsafe {
                    std::sync::atomic::AtomicU8::from_ptr(
                        (unsafe { std::ptr::addr_of_mut!(sqlite3Config.bOpenUri) }) as *mut u8,
                    )
                    .store((bOpenUri as i8) as u8, std::sync::atomic::Ordering::Relaxed)
                };
            }
            20 => {
                // EVIDENCE-OF: R-36592-02772 The SQLITE_CONFIG_COVERING_INDEX_SCAN
                // option takes a single integer argument which is interpreted as a
                // boolean in order to enable or disable the use of covering indices for
                // full table scans in the query optimizer.
                unsafe {
                    sqlite3Config.bUseCis = ((unsafe { ap.next_arg::<i32>() }) as i8) as u8;
                }
            }
            22 => {
                // EVIDENCE-OF: R-58063-38258 SQLITE_CONFIG_MMAP_SIZE takes two 64-bit
                // integer (sqlite3_int64) values that are the default mmap size limit
                // (the default setting for PRAGMA mmap_size) and the maximum allowed
                // mmap size limit.
                let mut szMmap: i64 = unsafe { ap.next_arg::<i64>() };
                let mut mxMmap: i64 = unsafe { ap.next_arg::<i64>() };
                // EVIDENCE-OF: R-53367-43190 If either argument to this option is
                // negative, then that argument is changed to its compile-time default.
                //
                // EVIDENCE-OF: R-34993-45031 The maximum allowed mmap size will be
                // silently truncated if necessary so that it does not exceed the
                // compile-time maximum mmap size set by the SQLITE_MAX_MMAP_SIZE
                // compile-time option.
                if mxMmap < ((0 as i32) as i64) || mxMmap > ((2147418112 as i32) as i64) {
                    mxMmap = (2147418112 as i32) as i64;
                }
                if szMmap < ((0 as i32) as i64) {
                    szMmap = (0 as i32) as i64;
                }
                if szMmap > mxMmap {
                    szMmap = mxMmap;
                }
                unsafe {
                    sqlite3Config.mxMmap = mxMmap;
                }
                unsafe {
                    sqlite3Config.szMmap = szMmap;
                }
            }
            25 => unsafe {
                sqlite3Config.szPma = unsafe { ap.next_arg::<u32>() };
            },
            26 => unsafe {
                sqlite3Config.nStmtSpill = unsafe { ap.next_arg::<i32>() };
            },
            29 => unsafe {
                sqlite3Config.mxMemdbSize = unsafe { ap.next_arg::<i64>() };
            },
            30 => {
                let mut pVal: *mut i32 = unsafe { ap.next_arg::<*mut i32>() };
                unsafe {
                    *pVal = 0 as i32;
                }
            }
            _ => {
                rc = 1 as i32;
            }
        }
    }
    {}
    return rc;
}

static mut mAnytimeConfigOption: u64 = (((0 as i32) as i64) as u64)
    | (((1 as i32) as i64) as u64) << (16 as i32)
    | (((1 as i32) as i64) as u64) << (24 as i32);

/// Set up the lookaside buffers for a database connection.
/// Return SQLITE_OK on success.
/// If lookaside is already active, return SQLITE_BUSY.
///
/// The sz parameter is the number of bytes in each lookaside slot.
/// The cnt parameter is the number of slots.  If pBuf is NULL the
/// space for the lookaside memory is obtained from sqlite3_malloc()
/// or similar.  If pBuf is not NULL then it is sz*cnt bytes of memory
/// to use for the lookaside memory.
///
/// # Arguments
///
/// * `db` - Database connection being configured
/// * `pBuf` - Memory to use for lookaside.  May be NULL
/// * `sz` - Desired size of each lookaside memory slot
/// * `cnt` - Number of slots to allocate
fn setupLookaside(mut db: *mut sqlite3, mut pBuf: *mut (), mut sz: i32, mut cnt: i32) -> i32 {
    let mut pStart: *mut () = unsafe { std::mem::zeroed() }; // Start of the lookaside buffer
    let mut szAlloc: i64 = 0 as i64; // Total space set aside for lookaside memory
    let mut nBig: i32 = 0 as i32; // Number of full-size slots
    let mut nSm: i32 = 0 as i32; // Number smaller LOOKASIDE_SMALL-byte slots
    if (unsafe { sqlite3LookasideUsed(db, std::ptr::null_mut::<i32>()) }) > (0 as i32) {
        return 5 as i32;
    }
    // Free any existing lookaside buffer for this handle before
    // allocating a new one so we don't have to have space for
    // both at the same time.
    if (unsafe { (*db).lookaside.bMalloced }) != (0 as u8) {
        unsafe { sqlite3_free(unsafe { (*db).lookaside.pStart }) };
    }
    // The size of a lookaside slot after ROUNDDOWN8 needs to be larger
    // than a pointer and small enough to fit in a u16.
    sz = sz & !(7 as i32);
    if sz <= (((8 as u64) as u32) as i32) {
        sz = 0 as i32;
    }
    if sz > (65528 as i32) {
        sz = 65528 as i32;
    }
    // Count must be at least 1 to be useful, but not so large as to use
    // more than 0x7fff0000 total bytes for lookaside.
    if cnt < (1 as i32) {
        cnt = 0 as i32;
    }
    if sz > (0 as i32) && cnt > (2147418112 as i32) / sz {
        cnt = (2147418112 as i32) / sz;
    }
    szAlloc = (sz as i64) * (cnt as i64);
    if szAlloc == ((0 as i32) as i64) {
        sz = 0 as i32;
        pStart = std::ptr::null_mut::<()>();
    } else {
        if pBuf == std::ptr::null_mut::<()>() {
            unsafe { sqlite3BeginBenignMalloc() };
            pStart = unsafe { sqlite3Malloc(szAlloc as u64) };
            unsafe { sqlite3EndBenignMalloc() };
            if pStart != std::ptr::null_mut::<()>() {
                szAlloc = (unsafe { sqlite3MallocSize(pStart as *const ()) }) as i64;
            }
        } else {
            pStart = pBuf;
        }
    }
    if sz >= (128 as i32) * (3 as i32) {
        nBig = (szAlloc / (((3 as i32) * (128 as i32) + sz) as i64)) as i32;
        nSm = ((szAlloc - (sz as i64) * (nBig as i64)) / ((128 as i32) as i64)) as i32;
    } else {
        if sz >= (128 as i32) * (2 as i32) {
            nBig = (szAlloc / (((128 as i32) + sz) as i64)) as i32;
            nSm = ((szAlloc - (sz as i64) * (nBig as i64)) / ((128 as i32) as i64)) as i32;
        } else {
            if sz > (0 as i32) {
                nBig = (szAlloc / (sz as i64)) as i32;
                nSm = 0 as i32;
            } else {
                nSm = 0 as i32;
                nBig = 0 as i32;
            }
        }
    }
    unsafe {
        (*db).lookaside.pStart = pStart;
    }
    unsafe {
        (*db).lookaside.pInit = std::ptr::null_mut::<LookasideSlot>();
    }
    unsafe {
        (*db).lookaside.pFree = std::ptr::null_mut::<LookasideSlot>();
    }
    unsafe {
        (*db).lookaside.sz = (sz as i16) as u16;
    }
    unsafe {
        (*db).lookaside.szTrue = (sz as i16) as u16;
    }
    if pStart != std::ptr::null_mut::<()>() {
        let mut i: i32 = 0 as i32;
        let mut p: *mut LookasideSlot = unsafe { std::mem::zeroed() };
        0 as i32;
        p = pStart as *mut LookasideSlot;
        i = 0 as i32;
        '__slate_break_1688: loop {
            if !(i < nBig) {
                break;
            }
            unsafe {
                (*p).pNext = unsafe { (*db).lookaside.pInit };
            }
            unsafe {
                (*db).lookaside.pInit = p;
            }
            p = (unsafe { (p as *mut u8).offset(sz as isize) }) as *mut LookasideSlot;
            let __v1948: i32 = i;
            let __v1949: i32 = __v1948 + (1 as i32);
            i = __v1949;
        }
        unsafe {
            (*db).lookaside.pSmallInit = std::ptr::null_mut::<LookasideSlot>();
        }
        unsafe {
            (*db).lookaside.pSmallFree = std::ptr::null_mut::<LookasideSlot>();
        }
        unsafe {
            (*db).lookaside.pMiddle = p as *mut ();
        }
        i = 0 as i32;
        '__slate_break_1689: loop {
            if !(i < nSm) {
                break;
            }
            unsafe {
                (*p).pNext = unsafe { (*db).lookaside.pSmallInit };
            }
            unsafe {
                (*db).lookaside.pSmallInit = p;
            }
            p = (unsafe { (p as *mut u8).offset((128 as i32) as isize) }) as *mut LookasideSlot;
            let __v1950: i32 = i;
            let __v1951: i32 = __v1950 + (1 as i32);
            i = __v1951;
        }
        0 as i32;
        unsafe {
            (*db).lookaside.pEnd = p as *mut ();
        }
        unsafe {
            (*db).lookaside.bDisable = (0 as i32) as u32;
        }
        unsafe {
            (*db).lookaside.bMalloced = ((if pBuf == std::ptr::null_mut::<()>() {
                1 as i32
            } else {
                0 as i32
            }) as i8) as u8;
        }
        unsafe {
            (*db).lookaside.nSlot = (nBig + nSm) as u32;
        }
    } else {
        unsafe {
            (*db).lookaside.pStart = std::ptr::null_mut::<()>();
        }
        unsafe {
            (*db).lookaside.pSmallInit = std::ptr::null_mut::<LookasideSlot>();
        }
        unsafe {
            (*db).lookaside.pSmallFree = std::ptr::null_mut::<LookasideSlot>();
        }
        unsafe {
            (*db).lookaside.pMiddle = std::ptr::null_mut::<()>();
        }
        unsafe {
            (*db).lookaside.pEnd = std::ptr::null_mut::<()>();
        }
        unsafe {
            (*db).lookaside.bDisable = (1 as i32) as u32;
        }
        unsafe {
            (*db).lookaside.sz = ((0 as i32) as i16) as u16;
        }
        unsafe {
            (*db).lookaside.bMalloced = ((0 as i32) as i8) as u8;
        }
        unsafe {
            (*db).lookaside.nSlot = (0 as i32) as u32;
        }
    }
    unsafe {
        (*db).lookaside.pTrueEnd = unsafe { (*db).lookaside.pEnd };
    }
    0 as i32;
    return 0 as i32;
}

/// Return the mutex associated with a database connection.
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.main.sqlite3_db_mutex")]
extern "C-unwind" fn sqlite3_db_mutex(mut db: *mut sqlite3) -> *mut sqlite3_mutex {
    return unsafe { (*db).mutex };
}

/// Free up as much memory as we can from the given database
/// connection.
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.main.sqlite3_db_release_memory")]
extern "C-unwind" fn sqlite3_db_release_memory(mut db: *mut sqlite3) -> i32 {
    let mut i: i32 = 0 as i32;
    unsafe { sqlite3_mutex_enter(unsafe { (*db).mutex }) };
    unsafe { sqlite3BtreeEnterAll(db) };
    i = 0 as i32;
    '__slate_break_1690: loop {
        if !(i < unsafe { (*db).nDb }) {
            break;
        }
        let mut pBt: *mut Btree =
            unsafe { (*unsafe { unsafe { (*db).aDb }.offset(i as isize) }).pBt };
        if pBt != std::ptr::null_mut::<Btree>() {
            let mut pPager: *mut Pager = unsafe { sqlite3BtreePager(pBt) };
            unsafe { sqlite3PagerShrink(pPager) };
        }
        let __v1864: i32 = i;
        let __v1865: i32 = __v1864 + (1 as i32);
        i = __v1865;
    }
    unsafe { sqlite3BtreeLeaveAll(db) };
    unsafe { sqlite3_mutex_leave(unsafe { (*db).mutex }) };
    return 0 as i32;
}

/// Flush any dirty pages in the pager-cache for any attached database
/// to disk.
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.main.sqlite3_db_cacheflush")]
extern "C-unwind" fn sqlite3_db_cacheflush(mut db: *mut sqlite3) -> i32 {
    let mut i: i32 = 0 as i32;
    let mut rc: i32 = 0 as i32;
    let mut bSeenBusy: i32 = 0 as i32;
    unsafe { sqlite3_mutex_enter(unsafe { (*db).mutex }) };
    unsafe { sqlite3BtreeEnterAll(db) };
    i = 0 as i32;
    '__slate_break_1691: loop {
        if !(rc == (0 as i32) && i < unsafe { (*db).nDb }) {
            break;
        }
        let mut pBt: *mut Btree =
            unsafe { (*unsafe { unsafe { (*db).aDb }.offset(i as isize) }).pBt };
        let __v1881: bool;
        if pBt != std::ptr::null_mut::<Btree>() {
            __v1881 = (unsafe { sqlite3BtreeTxnState(pBt) }) == (2 as i32);
        } else {
            __v1881 = false as bool;
        }
        if __v1881 {
            let mut pPager: *mut Pager = unsafe { sqlite3BtreePager(pBt) };
            rc = unsafe { sqlite3PagerFlush(pPager) };
            if rc == (5 as i32) {
                bSeenBusy = 1 as i32;
                rc = 0 as i32;
            }
        }
        let __v1879: i32 = i;
        let __v1880: i32 = __v1879 + (1 as i32);
        i = __v1880;
    }
    unsafe { sqlite3BtreeLeaveAll(db) };
    unsafe { sqlite3_mutex_leave(unsafe { (*db).mutex }) };
    return if rc == (0 as i32) && bSeenBusy != (0 as i32) {
        5 as i32
    } else {
        rc
    };
}

/// Configuration settings for an individual database connection
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.main.sqlite3_db_config")]
unsafe extern "C-unwind" fn sqlite3_db_config(
    mut db: *mut sqlite3,
    mut op: i32,
    mut __va_args: ...
) -> i32 {
    let mut ap: core::ffi::VaList<'_> = unsafe { std::mem::zeroed() };
    let mut rc: i32 = 0 as i32;
    unsafe { sqlite3_mutex_enter(unsafe { (*db).mutex }) };
    ap = __va_args.clone();
    '__slate_break_1692: {
        match op {
            1000 => {
                // IMP: R-06824-28531
                //
                // IMP: R-36257-52125
                unsafe {
                    (*unsafe { unsafe { (*db).aDb }.offset((0 as i32) as isize) }).zDbSName =
                        unsafe { ap.next_arg::<*mut i8>() };
                }
                rc = 0 as i32;
            }
            1001 => {
                let mut pBuf: *mut () = unsafe { ap.next_arg::<*mut ()>() }; // IMP: R-26835-10964
                let mut sz: i32 = unsafe { ap.next_arg::<i32>() }; // IMP: R-47871-25994
                let mut cnt: i32 = unsafe { ap.next_arg::<i32>() }; // IMP: R-04460-53386
                rc = setupLookaside(db, pBuf, sz, cnt);
            }
            1023 => {
                let mut nIn: i32 = unsafe { ap.next_arg::<i32>() };
                let mut pOut: *mut i32 = unsafe { ap.next_arg::<*mut i32>() };
                if nIn > (3 as i32) && nIn < (24 as i32) {
                    unsafe {
                        (*db).nFpDigit = (nIn as i8) as u8;
                    }
                }
                if pOut != std::ptr::null_mut::<i32>() {
                    unsafe {
                        *pOut = ((unsafe { (*db).nFpDigit }) as u32) as i32;
                    }
                }
                rc = 0 as i32;
            }
            _ => {
                let mut i: u32 = 0 as u32;
                rc = 1 as i32; // IMP: R-42790-23372
                i = (0 as i32) as u32;
                '__slate_break_1693: while i
                    < (((((336 as u64) / (16 as u64)) as u32) as i32) as u32)
                {
                    if (unsafe {
                        (*unsafe {
                            unsafe { std::ptr::addr_of!(aFlagOp.0) as *const __SlateRecord205 }
                                .offset(i as isize)
                        })
                        .op
                    }) == op
                    {
                        let mut onoff: i32 = unsafe { ap.next_arg::<i32>() };
                        let mut pRes: *mut i32 = unsafe { ap.next_arg::<*mut i32>() };
                        let mut oldFlags: u64 = unsafe { (*db).flags };
                        if onoff > (0 as i32) {
                            let __v1808: *mut sqlite3 = db;
                            let __v1809: u64 = unsafe { (*__v1808).flags };
                            let __v1810: u64 = __v1809
                                | unsafe {
                                    (*unsafe {
                                        unsafe {
                                            std::ptr::addr_of!(aFlagOp.0) as *const __SlateRecord205
                                        }
                                        .offset(i as isize)
                                    })
                                    .mask
                                };
                            unsafe {
                                (*__v1808).flags = __v1810;
                            }
                        } else {
                            if onoff == (0 as i32) {
                                let __v1811: *mut sqlite3 = db;
                                let __v1812: u64 = unsafe { (*__v1811).flags };
                                let __v1813: u64 = __v1812
                                    & !unsafe {
                                        (*unsafe {
                                            unsafe {
                                                std::ptr::addr_of!(aFlagOp.0)
                                                    as *const __SlateRecord205
                                            }
                                            .offset(i as isize)
                                        })
                                        .mask
                                    };
                                unsafe {
                                    (*__v1811).flags = __v1813;
                                }
                            }
                        }
                        if oldFlags != unsafe { (*db).flags } {
                            unsafe { sqlite3ExpirePreparedStatements(db, 0 as i32) };
                        }
                        if pRes != std::ptr::null_mut::<i32>() {
                            unsafe {
                                *pRes = ((unsafe { (*db).flags })
                                    & unsafe {
                                        (*unsafe {
                                            unsafe {
                                                std::ptr::addr_of!(aFlagOp.0)
                                                    as *const __SlateRecord205
                                            }
                                            .offset(i as isize)
                                        })
                                        .mask
                                    }
                                    != (((0 as i32) as i64) as u64))
                                    as i32;
                            }
                        }
                        rc = 0 as i32;
                        break '__slate_break_1693;
                    }
                    let __v1806: u32 = i;
                    let __v1807: u32 = __v1806.wrapping_add((1 as i32) as u32);
                    i = __v1807;
                }
            }
        }
    }
    {}
    unsafe { sqlite3_mutex_leave(unsafe { (*db).mutex }) };
    return rc;
}

static mut aFlagOp: __SlateAlign16<[__SlateRecord205; 21]> = __SlateAlign16([
    __SlateRecord205 {
        op: 1002 as i32,
        mask: ((16384 as i32) as i64) as u64,
    },
    __SlateRecord205 {
        op: 1003 as i32,
        mask: ((262144 as i32) as i64) as u64,
    },
    __SlateRecord205 {
        op: 1015 as i32,
        mask: (2147483648 as u32) as u64,
    },
    __SlateRecord205 {
        op: 1004 as i32,
        mask: ((4194304 as i32) as i64) as u64,
    },
    __SlateRecord205 {
        op: 1005 as i32,
        mask: ((65536 as i32) as i64) as u64,
    },
    __SlateRecord205 {
        op: 1006 as i32,
        mask: ((2048 as i32) as i64) as u64,
    },
    __SlateRecord205 {
        op: 1007 as i32,
        mask: ((8388608 as i32) as i64) as u64,
    },
    __SlateRecord205 {
        op: 1008 as i32,
        mask: ((16777216 as i32) as i64) as u64,
    },
    __SlateRecord205 {
        op: 1009 as i32,
        mask: ((33554432 as i32) as i64) as u64,
    },
    __SlateRecord205 {
        op: 1010 as i32,
        mask: ((268435456 as i32) as i64) as u64,
    },
    __SlateRecord205 {
        op: 1011 as i32,
        mask: (((1 as i32) | (134217728 as i32)) as i64) as u64,
    },
    __SlateRecord205 {
        op: 1012 as i32,
        mask: ((67108864 as i32) as i64) as u64,
    },
    __SlateRecord205 {
        op: 1014 as i32,
        mask: ((536870912 as i32) as i64) as u64,
    },
    __SlateRecord205 {
        op: 1013 as i32,
        mask: ((1073741824 as i32) as i64) as u64,
    },
    __SlateRecord205 {
        op: 1016 as i32,
        mask: ((2 as i32) as i64) as u64,
    },
    __SlateRecord205 {
        op: 1017 as i32,
        mask: ((128 as i32) as i64) as u64,
    },
    __SlateRecord205 {
        op: 1018 as i32,
        mask: ((1024 as i32) as i64) as u64,
    },
    __SlateRecord205 {
        op: 1019 as i32,
        mask: ((4096 as i32) as i64) as u64,
    },
    __SlateRecord205 {
        op: 1020 as i32,
        mask: (((16 as i32) as i64) as u64) << (32 as i32),
    },
    __SlateRecord205 {
        op: 1021 as i32,
        mask: (((32 as i32) as i64) as u64) << (32 as i32),
    },
    __SlateRecord205 {
        op: 1022 as i32,
        mask: (((64 as i32) as i64) as u64) << (32 as i32),
    },
]);

/// This is the default collating function named "BINARY" which is always
/// available.
#[unsafe(link_section = ".text.slate_distinct.main.binCollFunc")]
extern "C-unwind" fn binCollFunc(
    mut NotUsed: *mut (),
    mut nKey1: i32,
    mut pKey1: *const (),
    mut nKey2: i32,
    mut pKey2: *const (),
) -> i32 {
    let mut rc: i32 = 0 as i32;
    let mut n: i32 = 0 as i32;
    NotUsed;
    n = if nKey1 < nKey2 { nKey1 } else { nKey2 };
    // EVIDENCE-OF: R-65033-28449 The built-in BINARY collation compares
    // strings byte by byte using the memcmp() function from the standard C
    // library.
    0 as i32;
    rc = unsafe { memcmp(pKey1, pKey2, (n as i64) as u64) };
    if rc == (0 as i32) {
        rc = nKey1 - nKey2;
    }
    return rc;
}

/// This is the collating function named "RTRIM" which is always
/// available.  Ignore trailing spaces.
#[unsafe(link_section = ".text.slate_distinct.main.rtrimCollFunc")]
extern "C-unwind" fn rtrimCollFunc(
    mut pUser: *mut (),
    mut nKey1: i32,
    mut pKey1: *const (),
    mut nKey2: i32,
    mut pKey2: *const (),
) -> i32 {
    let mut pK1: *const u8 = pKey1 as *const u8;
    let mut pK2: *const u8 = pKey2 as *const u8;
    '__slate_break_1694: while nKey1 != (0 as i32)
        && (((unsafe { *unsafe { pK1.offset((nKey1 - (1 as i32)) as isize) } }) as u32) as i32)
            == (32 as i32)
    {
        let __v1952: i32 = nKey1;
        let __v1953: i32 = __v1952 - (1 as i32);
        nKey1 = __v1953;
    }
    '__slate_break_1695: while nKey2 != (0 as i32)
        && (((unsafe { *unsafe { pK2.offset((nKey2 - (1 as i32)) as isize) } }) as u32) as i32)
            == (32 as i32)
    {
        let __v1954: i32 = nKey2;
        let __v1955: i32 = __v1954 - (1 as i32);
        nKey2 = __v1955;
    }
    return binCollFunc(pUser, nKey1, pKey1, nKey2, pKey2);
}

/// Return true if CollSeq is the default built-in BINARY.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3IsBinary(mut p: *const CollSeq) -> i32 {
    0 as i32;
    return (p == std::ptr::null::<CollSeq>() || (unsafe { (*p).xCmp }) == Some(binCollFunc))
        as i32;
}

/// Another built-in collating sequence: NOCASE.
///
/// This collating sequence is intended to be used for "case independent
/// comparison". SQLite's knowledge of upper and lower case equivalents
/// extends only to the 26 characters used in the English language.
///
/// At the moment there is only a UTF-8 implementation.
#[unsafe(link_section = ".text.slate_distinct.main.nocaseCollatingFunc")]
extern "C-unwind" fn nocaseCollatingFunc(
    mut NotUsed: *mut (),
    mut nKey1: i32,
    mut pKey1: *const (),
    mut nKey2: i32,
    mut pKey2: *const (),
) -> i32 {
    let mut r: i32 = unsafe {
        sqlite3_strnicmp(
            pKey1 as *const i8,
            pKey2 as *const i8,
            if nKey1 < nKey2 { nKey1 } else { nKey2 },
        )
    };
    NotUsed;
    if (0 as i32) == r {
        r = nKey1 - nKey2;
    }
    return r;
}

/// Return the ROWID of the most recent insert
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.main.sqlite3_last_insert_rowid")]
extern "C-unwind" fn sqlite3_last_insert_rowid(mut db: *mut sqlite3) -> i64 {
    let mut iRet: i64 = 0 as i64;
    unsafe { sqlite3_mutex_enter(unsafe { (*db).mutex }) };
    iRet = unsafe { (*db).lastRowid };
    unsafe { sqlite3_mutex_leave(unsafe { (*db).mutex }) };
    return iRet;
}

/// Set the value returned by the sqlite3_last_insert_rowid() API function.
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.main.sqlite3_set_last_insert_rowid")]
extern "C-unwind" fn sqlite3_set_last_insert_rowid(mut db: *mut sqlite3, mut iRowid: i64) {
    unsafe { sqlite3_mutex_enter(unsafe { (*db).mutex }) };
    unsafe {
        (*db).lastRowid = iRowid;
    }
    unsafe { sqlite3_mutex_leave(unsafe { (*db).mutex }) };
}

/// Return the number of changes in the most recently executed DML
/// statement.
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.main.sqlite3_changes64")]
extern "C-unwind" fn sqlite3_changes64(mut db: *mut sqlite3) -> i64 {
    let mut iRet: i64 = 0 as i64;
    unsafe { sqlite3_mutex_enter(unsafe { (*db).mutex }) };
    iRet = unsafe { (*db).nChange };
    unsafe { sqlite3_mutex_leave(unsafe { (*db).mutex }) };
    return iRet;
}

#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.main.sqlite3_changes")]
extern "C-unwind" fn sqlite3_changes(mut db: *mut sqlite3) -> i32 {
    return sqlite3_changes64(db) as i32;
}

/// Return the number of changes since the database handle was opened.
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.main.sqlite3_total_changes64")]
extern "C-unwind" fn sqlite3_total_changes64(mut db: *mut sqlite3) -> i64 {
    let mut iRet: i64 = 0 as i64;
    unsafe { sqlite3_mutex_enter(unsafe { (*db).mutex }) };
    iRet = unsafe { (*db).nTotalChange };
    unsafe { sqlite3_mutex_leave(unsafe { (*db).mutex }) };
    return iRet;
}

#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.main.sqlite3_total_changes")]
extern "C-unwind" fn sqlite3_total_changes(mut db: *mut sqlite3) -> i32 {
    return sqlite3_total_changes64(db) as i32;
}

/// Close all open savepoints. This function only manipulates fields of the
/// database handle object, it does not close any savepoints that may be open
/// at the b-tree/pager level.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3CloseSavepoints(mut db: *mut sqlite3) {
    '__slate_break_1696: while (unsafe { (*db).pSavepoint }) != std::ptr::null_mut::<Savepoint>() {
        let mut pTmp: *mut Savepoint = unsafe { (*db).pSavepoint };
        unsafe {
            (*db).pSavepoint = unsafe { (*pTmp).pNext };
        }
        unsafe { sqlite3DbFree(db, pTmp as *mut ()) };
    }
    unsafe {
        (*db).nSavepoint = 0 as i32;
    }
    unsafe {
        (*db).nStatement = 0 as i32;
    }
    unsafe {
        (*db).isTransactionSavepoint = ((0 as i32) as i8) as u8;
    }
}

/// Invoke the destructor function associated with FuncDef p, if any. Except,
/// if this is not the last copy of the function, do not invoke it. Multiple
/// copies of a single function are created when create_function() is called
/// with SQLITE_ANY as the encoding.
fn functionDestroy(mut db: *mut sqlite3, mut p: *mut FuncDef) {
    let mut pDestructor: *mut FuncDestructor = unsafe { std::mem::zeroed() };
    0 as i32;
    pDestructor = unsafe { (*p).u.pDestructor };
    if pDestructor != std::ptr::null_mut::<FuncDestructor>() {
        let __v1956: *mut FuncDestructor = pDestructor;
        let __v1957: i32 = unsafe { (*__v1956).nRef };
        let __v1958: i32 = __v1957 - (1 as i32);
        unsafe {
            (*__v1956).nRef = __v1958;
        }
        if (unsafe { (*pDestructor).nRef }) == (0 as i32) {
            unsafe {
                unsafe { (*pDestructor).xDestroy }.unwrap()(unsafe { (*pDestructor).pUserData })
            };
            unsafe { sqlite3DbFree(db, pDestructor as *mut ()) };
        }
    }
}

/// Disconnect all sqlite3_vtab objects that belong to database connection
/// db. This is called when db is being closed.
fn disconnectAllVtab(mut db: *mut sqlite3) {
    let mut i: i32 = 0 as i32;
    let mut p: *mut HashElem = unsafe { std::mem::zeroed() };
    unsafe { sqlite3BtreeEnterAll(db) };
    i = 0 as i32;
    '__slate_break_1697: loop {
        if !(i < unsafe { (*db).nDb }) {
            break;
        }
        let mut pSchema: *mut Schema =
            unsafe { (*unsafe { unsafe { (*db).aDb }.offset(i as isize) }).pSchema };
        if pSchema != std::ptr::null_mut::<Schema>() {
            p = unsafe { (*unsafe { std::ptr::addr_of_mut!((*pSchema).tblHash) }).first };
            '__slate_break_1698: while p != std::ptr::null_mut::<HashElem>() {
                let mut pTab: *mut Table = (unsafe { (*p).data }) as *mut Table;
                if (((unsafe { (*pTab).eTabType }) as u32) as i32) == (1 as i32) {
                    unsafe { sqlite3VtabDisconnect(db, pTab) };
                }
                p = unsafe { (*p).next };
            }
        }
        let __v1959: i32 = i;
        let __v1960: i32 = __v1959 + (1 as i32);
        i = __v1960;
    }
    p = unsafe { (*unsafe { std::ptr::addr_of_mut!((*db).aModule) }).first };
    '__slate_break_1699: while p != std::ptr::null_mut::<HashElem>() {
        let mut pMod: *mut Module = (unsafe { (*p).data }) as *mut Module;
        if (unsafe { (*pMod).pEpoTab }) != std::ptr::null_mut::<Table>() {
            unsafe { sqlite3VtabDisconnect(db, unsafe { (*pMod).pEpoTab }) };
        }
        p = unsafe { (*p).next };
    }
    unsafe { sqlite3VtabUnlockList(db) };
    unsafe { sqlite3BtreeLeaveAll(db) };
}

/// Return TRUE if database connection db has unfinalized prepared
/// statements or unfinished sqlite3_backup objects.
fn connectionIsBusy(mut db: *mut sqlite3) -> i32 {
    let mut j: i32 = 0 as i32;
    0 as i32;
    if (unsafe { (*db).pVdbe }) != std::ptr::null_mut::<Vdbe>() {
        return 1 as i32;
    }
    j = 0 as i32;
    '__slate_break_1700: loop {
        if !(j < unsafe { (*db).nDb }) {
            break;
        }
        let mut pBt: *mut Btree =
            unsafe { (*unsafe { unsafe { (*db).aDb }.offset(j as isize) }).pBt };
        let __v1963: bool;
        if pBt != std::ptr::null_mut::<Btree>() {
            __v1963 = (unsafe { sqlite3BtreeIsInBackup(pBt) }) != (0 as i32);
        } else {
            __v1963 = false as bool;
        }
        if __v1963 {
            return 1 as i32;
        }
        let __v1961: i32 = j;
        let __v1962: i32 = __v1961 + (1 as i32);
        j = __v1962;
    }
    return 0 as i32;
}

/// Close an existing SQLite database
fn sqlite3Close(mut db: *mut sqlite3, mut forceZombie: i32) -> i32 {
    if !(db != std::ptr::null_mut::<sqlite3>()) {
        // EVIDENCE-OF: R-63257-11740 Calling sqlite3_close() or
        // sqlite3_close_v2() with a NULL pointer argument is a harmless no-op.
        return 0 as i32;
    }
    if !((unsafe { sqlite3SafetyCheckSickOrOk(db) }) != (0 as i32)) {
        return sqlite3MisuseError(1291 as i32);
    }
    unsafe { sqlite3_mutex_enter(unsafe { (*db).mutex }) };
    if (((unsafe { (*db).mTrace }) as u32) as i32) & (8 as i32) != (0 as i32) {
        unsafe {
            unsafe { (*db).trace.xV2 }.unwrap()(
                (8 as i32) as u32,
                unsafe { (*db).pTraceArg },
                db as *mut (),
                std::ptr::null_mut::<()>(),
            )
        };
    }
    // Force xDisconnect calls on all virtual tables
    disconnectAllVtab(db);
    // If a transaction is open, the disconnectAllVtab() call above
    // will not have called the xDisconnect() method on any virtual
    // tables in the db->aVTrans[] array. The following sqlite3VtabRollback()
    // call will do so. We need to do this before the check for active
    // SQL statements below, as the v-table implementation may be storing
    // some prepared statements internally.
    unsafe { sqlite3VtabRollback(db) };
    // Legacy behavior (sqlite3_close() behavior) is to return
    // SQLITE_BUSY if the connection can not be closed immediately.
    let __v1964: bool;
    if !(forceZombie != (0 as i32)) {
        __v1964 = connectionIsBusy(db) != (0 as i32);
    } else {
        __v1964 = false as bool;
    }
    if __v1964 {
        unsafe {
            sqlite3ErrorWithMsg(
                db,
                5 as i32,
                (b"unable to close due to unfinalized statements or unfinished backups\0".as_ptr()
                    as *mut i8) as *const i8,
            )
        };
        unsafe { sqlite3_mutex_leave(unsafe { (*db).mutex }) };
        return 5 as i32;
    }
    '__slate_break_1702: while (unsafe { (*db).pDbData }) != std::ptr::null_mut::<DbClientData>() {
        let mut p: *mut DbClientData = unsafe { (*db).pDbData };
        unsafe {
            (*db).pDbData = unsafe { (*p).pNext };
        }
        0 as i32;
        if (unsafe { (*p).xDestructor }) != None {
            unsafe { unsafe { (*p).xDestructor }.unwrap()(unsafe { (*p).pData }) };
        }
        unsafe { sqlite3_free(p as *mut ()) };
    }
    // Convert the connection into a zombie and then close it.
    unsafe {
        (*db).eOpenState = ((167 as i32) as i8) as u8;
    }
    sqlite3LeaveMutexAndCloseZombie(db);
    return 0 as i32;
}

/// Return the transaction state for a single databse, or the maximum
/// transaction state over all attached databases if zSchema is null.
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.main.sqlite3_txn_state")]
extern "C-unwind" fn sqlite3_txn_state(mut db: *mut sqlite3, mut zSchema: *const i8) -> i32 {
    let mut iDb: i32 = 0 as i32;
    let mut nDb: i32 = 0 as i32;
    let mut iTxn: i32 = -(1 as i32);
    unsafe { sqlite3_mutex_enter(unsafe { (*db).mutex }) };
    if zSchema != std::ptr::null::<i8>() {
        let __v1858: i32 = unsafe { sqlite3FindDbName(db, zSchema) };
        iDb = __v1858;
        nDb = __v1858;
        if iDb < (0 as i32) {
            let __v1859: i32 = nDb;
            let __v1860: i32 = __v1859 - (1 as i32);
            nDb = __v1860;
        }
    } else {
        iDb = 0 as i32;
        nDb = (unsafe { (*db).nDb }) - (1 as i32);
    }
    '__slate_break_1703: loop {
        if !(iDb <= nDb) {
            break;
        }
        let mut pBt: *mut Btree =
            unsafe { (*unsafe { unsafe { (*db).aDb }.offset(iDb as isize) }).pBt };
        let mut x: i32 = 0 as i32;
        let __v1863: i32;
        if pBt != std::ptr::null_mut::<Btree>() {
            __v1863 = unsafe { sqlite3BtreeTxnState(pBt) };
        } else {
            __v1863 = 0 as i32;
        }
        x = __v1863;
        if x > iTxn {
            iTxn = x;
        }
        let __v1861: i32 = iDb;
        let __v1862: i32 = __v1861 + (1 as i32);
        iDb = __v1862;
    }
    unsafe { sqlite3_mutex_leave(unsafe { (*db).mutex }) };
    return iTxn;
}

/// Two variations on the public interface for closing a database
/// connection. The sqlite3_close() version returns SQLITE_BUSY and
/// leaves the connection open if there are unfinalized prepared
/// statements or unfinished sqlite3_backups.  The sqlite3_close_v2()
/// version forces the connection to become a zombie if there are
/// unclosed resources, and arranges for deallocation when the last
/// prepare statement or sqlite3_backup closes.
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.main.sqlite3_close")]
extern "C-unwind" fn sqlite3_close(mut db: *mut sqlite3) -> i32 {
    return sqlite3Close(db, 0 as i32);
}

#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.main.sqlite3_close_v2")]
extern "C-unwind" fn sqlite3_close_v2(mut db: *mut sqlite3) -> i32 {
    return sqlite3Close(db, 1 as i32);
}

/// Close the mutex on database connection db.
///
/// Furthermore, if database connection db is a zombie (meaning that there
/// has been a prior call to sqlite3_close(db) or sqlite3_close_v2(db)) and
/// every sqlite3_stmt has now been finalized and every sqlite3_backup has
/// finished, then free all resources.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3LeaveMutexAndCloseZombie(mut db: *mut sqlite3) {
    let mut i: *mut HashElem = unsafe { std::mem::zeroed() }; // Hash table iterator
    let mut j: i32 = 0 as i32;
    // If there are outstanding sqlite3_stmt or sqlite3_backup objects
    // or if the connection has not yet been closed by sqlite3_close_v2(),
    // then just leave the mutex and return.
    let __v1924: bool;
    if (((unsafe { (*db).eOpenState }) as u32) as i32) != (167 as i32) {
        __v1924 = true as bool;
    } else {
        __v1924 = connectionIsBusy(db) != (0 as i32);
    }
    if __v1924 {
        unsafe { sqlite3_mutex_leave(unsafe { (*db).mutex }) };
        return;
    }
    // If we reach this point, it means that the database connection has
    // closed all sqlite3_stmt and sqlite3_backup objects and has been
    // passed to sqlite3_close (meaning that it is a zombie).  Therefore,
    // go ahead and free all resources.
    // If a transaction is open, roll it back. This also ensures that if
    // any database schemas have been modified by an uncommitted transaction
    // they are reset. And that the required b-tree mutex is held to make
    // the pager rollback and schema reset an atomic operation.
    sqlite3RollbackAll(db, 0 as i32);
    // Free any outstanding Savepoint structures.
    sqlite3CloseSavepoints(db);
    // Close all database connections
    j = 0 as i32;
    '__slate_break_1704: loop {
        if !(j < unsafe { (*db).nDb }) {
            break;
        }
        let mut pDb: *mut Db = unsafe { unsafe { (*db).aDb }.offset(j as isize) };
        if (unsafe { (*pDb).pBt }) != std::ptr::null_mut::<Btree>() {
            unsafe { sqlite3BtreeClose(unsafe { (*pDb).pBt }) };
            unsafe {
                (*pDb).pBt = std::ptr::null_mut::<Btree>();
            }
            if j != (1 as i32) {
                unsafe {
                    (*pDb).pSchema = std::ptr::null_mut::<Schema>();
                }
            }
        }
        let __v1925: i32 = j;
        let __v1926: i32 = __v1925 + (1 as i32);
        j = __v1926;
    }
    // Clear the TEMP schema separately and last
    if (unsafe { (*unsafe { unsafe { (*db).aDb }.offset((1 as i32) as isize) }).pSchema })
        != std::ptr::null_mut::<Schema>()
    {
        unsafe {
            sqlite3SchemaClear(
                (unsafe { (*unsafe { unsafe { (*db).aDb }.offset((1 as i32) as isize) }).pSchema })
                    as *mut (),
            )
        };
        0 as i32;
    }
    unsafe { sqlite3VtabUnlockList(db) };
    // Free up the array of auxiliary databases
    unsafe { sqlite3CollapseDatabaseArray(db) };
    0 as i32;
    0 as i32;
    // Tell the code in notify.c that the connection no longer holds any
    // locks and does not require any further unlock-notify callbacks.
    {}
    i = unsafe { (*unsafe { std::ptr::addr_of_mut!((*db).aFunc) }).first };
    '__slate_break_1705: while i != std::ptr::null_mut::<HashElem>() {
        let mut pNext: *mut FuncDef = unsafe { std::mem::zeroed() };
        let mut p: *mut FuncDef = unsafe { std::mem::zeroed() };
        p = (unsafe { (*i).data }) as *mut FuncDef;
        '__slate_break_1706: loop {
            functionDestroy(db, p);
            pNext = unsafe { (*p).pNext };
            unsafe { sqlite3DbFree(db, p as *mut ()) };
            p = pNext;
            if !(p != std::ptr::null_mut::<FuncDef>()) {
                break;
            }
        }
        i = unsafe { (*i).next };
    }
    unsafe { sqlite3HashClear(unsafe { std::ptr::addr_of_mut!((*db).aFunc) }) };
    i = unsafe { (*unsafe { std::ptr::addr_of_mut!((*db).aCollSeq) }).first };
    '__slate_break_1707: while i != std::ptr::null_mut::<HashElem>() {
        let mut pColl: *mut CollSeq = (unsafe { (*i).data }) as *mut CollSeq;
        // Invoke any destructors registered for collation sequence user data.
        j = 0 as i32;
        '__slate_break_1708: loop {
            if !(j < (3 as i32)) {
                break;
            }
            if (unsafe { (*unsafe { pColl.offset(j as isize) }).xDel }) != None {
                unsafe {
                    unsafe { (*unsafe { pColl.offset(j as isize) }).xDel }.unwrap()(unsafe {
                        (*unsafe { pColl.offset(j as isize) }).pUser
                    })
                };
            }
            let __v1927: i32 = j;
            let __v1928: i32 = __v1927 + (1 as i32);
            j = __v1928;
        }
        unsafe { sqlite3DbFree(db, pColl as *mut ()) };
        i = unsafe { (*i).next };
    }
    unsafe { sqlite3HashClear(unsafe { std::ptr::addr_of_mut!((*db).aCollSeq) }) };
    i = unsafe { (*unsafe { std::ptr::addr_of_mut!((*db).aModule) }).first };
    '__slate_break_1709: while i != std::ptr::null_mut::<HashElem>() {
        let mut pMod: *mut Module = (unsafe { (*i).data }) as *mut Module;
        unsafe { sqlite3VtabEponymousTableClear(db, pMod) };
        unsafe { sqlite3VtabModuleUnref(db, pMod) };
        i = unsafe { (*i).next };
    }
    unsafe { sqlite3HashClear(unsafe { std::ptr::addr_of_mut!((*db).aModule) }) };
    unsafe { sqlite3Error(db, 0 as i32) }; // Deallocates any cached error strings.
    unsafe { sqlite3ValueFree(unsafe { (*db).pErr }) };
    unsafe { sqlite3CloseExtensions(db) };
    unsafe {
        (*db).eOpenState = ((213 as i32) as i8) as u8;
    }
    // The temp-database schema is allocated differently from the other schema
    // objects (using sqliteMalloc() directly, instead of sqlite3BtreeSchema()).
    // So it needs to be freed here. Todo: Why not roll the temp schema into
    // the same sqliteMalloc() as the one that allocates the database
    // structure?
    unsafe {
        sqlite3DbFree(
            db,
            (unsafe { (*unsafe { unsafe { (*db).aDb }.offset((1 as i32) as isize) }).pSchema })
                as *mut (),
        )
    };
    if (unsafe { (*db).xAutovacDestr }) != None {
        unsafe { unsafe { (*db).xAutovacDestr }.unwrap()(unsafe { (*db).pAutovacPagesArg }) };
    }
    unsafe { sqlite3_mutex_leave(unsafe { (*db).mutex }) };
    unsafe {
        (*db).eOpenState = ((206 as i32) as i8) as u8;
    }
    unsafe { sqlite3_mutex_free(unsafe { (*db).mutex }) };
    0 as i32;
    if (unsafe { (*db).lookaside.bMalloced }) != (0 as u8) {
        unsafe { sqlite3_free(unsafe { (*db).lookaside.pStart }) };
    }
    unsafe { sqlite3_free(db as *mut ()) };
}

/// Rollback all database files.  If tripCode is not SQLITE_OK, then
/// any write cursors are invalidated ("tripped" - as in "tripping a circuit
/// breaker") and made to return tripCode if there are any further
/// attempts to use that cursor.  Read cursors remain open and valid
/// but are "saved" in case the table pages are moved around.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3RollbackAll(mut db: *mut sqlite3, mut tripCode: i32) {
    let mut i: i32 = 0 as i32;
    let mut inTrans: i32 = 0 as i32;
    let mut schemaChange: i32 = 0 as i32;
    0 as i32;
    unsafe { sqlite3BeginBenignMalloc() };
    // Obtain all b-tree mutexes before making any calls to BtreeRollback().
    // This is important in case the transaction being rolled back has
    // modified the database schema. If the b-tree mutexes are not taken
    // here, then another shared-cache connection might sneak in between
    // the database rollback and schema reset, which can cause false
    // corruption reports in some cases.
    unsafe { sqlite3BtreeEnterAll(db) };
    schemaChange = ((unsafe { (*db).mDbFlags }) & ((1 as i32) as u32) != ((0 as i32) as u32)
        && (((unsafe { (*db).init.busy }) as u32) as i32) == (0 as i32)) as i32;
    i = 0 as i32;
    '__slate_break_1710: loop {
        if !(i < unsafe { (*db).nDb }) {
            break;
        }
        let mut p: *mut Btree =
            unsafe { (*unsafe { unsafe { (*db).aDb }.offset(i as isize) }).pBt };
        if p != std::ptr::null_mut::<Btree>() {
            if (unsafe { sqlite3BtreeTxnState(p) }) == (2 as i32) {
                inTrans = 1 as i32;
            }
            unsafe { sqlite3BtreeRollback(p, tripCode, !(schemaChange != (0 as i32)) as i32) };
        }
        let __v1919: i32 = i;
        let __v1920: i32 = __v1919 + (1 as i32);
        i = __v1920;
    }
    unsafe { sqlite3VtabRollback(db) };
    unsafe { sqlite3EndBenignMalloc() };
    if schemaChange != (0 as i32) {
        unsafe { sqlite3ExpirePreparedStatements(db, 0 as i32) };
        unsafe { sqlite3ResetAllSchemasOfConnection(db) };
    }
    unsafe { sqlite3BtreeLeaveAll(db) };
    // Any deferred constraint violations have now been resolved.
    unsafe {
        (*db).nDeferredCons = (0 as i32) as i64;
    }
    unsafe {
        (*db).nDeferredImmCons = (0 as i32) as i64;
    }
    let __v1921: *mut sqlite3 = db;
    let __v1922: u64 = unsafe { (*__v1921).flags };
    let __v1923: u64 = __v1922
        & !((((524288 as i32) as i64) as u64) | (((2 as i32) as i64) as u64) << (32 as i32));
    unsafe {
        (*__v1921).flags = __v1923;
    }
    // If one has been configured, invoke the rollback-hook callback
    if (unsafe { (*db).xRollbackCallback }) != None
        && (inTrans != (0 as i32) || !((unsafe { (*db).autoCommit }) != (0 as u8)))
    {
        unsafe { unsafe { (*db).xRollbackCallback }.unwrap()(unsafe { (*db).pRollbackArg }) };
    }
}

/// Return a static string containing the name corresponding to the error code
/// specified in the argument.
/// Return a static string that describes the kind of error specified in the
/// argument.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3ErrStr(mut rc: i32) -> *const i8 {
    // SQLITE_OK
    // SQLITE_ERROR
    // SQLITE_INTERNAL
    // SQLITE_PERM
    // SQLITE_ABORT
    // SQLITE_BUSY
    // SQLITE_LOCKED
    // SQLITE_NOMEM
    // SQLITE_READONLY
    // SQLITE_INTERRUPT
    // SQLITE_IOERR
    // SQLITE_CORRUPT
    // SQLITE_NOTFOUND
    // SQLITE_FULL
    // SQLITE_CANTOPEN
    // SQLITE_PROTOCOL
    // SQLITE_EMPTY
    // SQLITE_SCHEMA
    // SQLITE_TOOBIG
    // SQLITE_CONSTRAINT
    // SQLITE_MISMATCH
    // SQLITE_MISUSE
    // SQLITE_NOLFS
    // SQLITE_AUTH
    // SQLITE_FORMAT
    // SQLITE_RANGE
    // SQLITE_NOTADB
    // SQLITE_NOTICE
    // SQLITE_WARNING
    let mut zErr: *const i8 = (b"unknown error\0".as_ptr() as *mut i8) as *const i8;
    '__slate_break_1737: {
        match rc {
            516 => {
                zErr = (b"abort due to ROLLBACK\0".as_ptr() as *mut i8) as *const i8;
            }
            100 => {
                zErr = (b"another row available\0".as_ptr() as *mut i8) as *const i8;
            }
            101 => {
                zErr = (b"no more rows available\0".as_ptr() as *mut i8) as *const i8;
            }
            _ => {
                let __v1929: i32 = rc;
                let __v1930: i32 = __v1929 & (255 as i32);
                rc = __v1930;
                if rc >= (0 as i32)
                    && rc < ((((232 as u64) / (8 as u64)) as u32) as i32)
                    && (unsafe {
                        *unsafe {
                            unsafe { std::ptr::addr_of!(aMsg.0) as *const *const i8 }
                                .offset(rc as isize)
                        }
                    }) != std::ptr::null::<i8>()
                {
                    zErr = unsafe {
                        *unsafe {
                            unsafe { std::ptr::addr_of!(aMsg.0) as *const *const i8 }
                                .offset(rc as isize)
                        }
                    };
                }
            }
        }
    }
    return zErr;
}

static mut aMsg: __SlateAlign16<[*const i8; 29]> = __SlateAlign16([
    (b"not an error\0".as_ptr() as *mut i8) as *const i8,
    (b"SQL logic error\0".as_ptr() as *mut i8) as *const i8,
    std::ptr::null::<i8>(),
    (b"access permission denied\0".as_ptr() as *mut i8) as *const i8,
    (b"query aborted\0".as_ptr() as *mut i8) as *const i8,
    (b"database is locked\0".as_ptr() as *mut i8) as *const i8,
    (b"database table is locked\0".as_ptr() as *mut i8) as *const i8,
    (b"out of memory\0".as_ptr() as *mut i8) as *const i8,
    (b"attempt to write a readonly database\0".as_ptr() as *mut i8) as *const i8,
    (b"interrupted\0".as_ptr() as *mut i8) as *const i8,
    (b"disk I/O error\0".as_ptr() as *mut i8) as *const i8,
    (b"database disk image is malformed\0".as_ptr() as *mut i8) as *const i8,
    (b"unknown operation\0".as_ptr() as *mut i8) as *const i8,
    (b"database or disk is full\0".as_ptr() as *mut i8) as *const i8,
    (b"unable to open database file\0".as_ptr() as *mut i8) as *const i8,
    (b"locking protocol\0".as_ptr() as *mut i8) as *const i8,
    std::ptr::null::<i8>(),
    (b"database schema has changed\0".as_ptr() as *mut i8) as *const i8,
    (b"string or blob too big\0".as_ptr() as *mut i8) as *const i8,
    (b"constraint failed\0".as_ptr() as *mut i8) as *const i8,
    (b"datatype mismatch\0".as_ptr() as *mut i8) as *const i8,
    (b"bad parameter or other API misuse\0".as_ptr() as *mut i8) as *const i8,
    std::ptr::null::<i8>(),
    (b"authorization denied\0".as_ptr() as *mut i8) as *const i8,
    std::ptr::null::<i8>(),
    (b"column index out of range\0".as_ptr() as *mut i8) as *const i8,
    (b"file is not a database\0".as_ptr() as *mut i8) as *const i8,
    (b"notification message\0".as_ptr() as *mut i8) as *const i8,
    (b"warning message\0".as_ptr() as *mut i8) as *const i8,
]);

/// This routine implements a busy callback that sleeps and tries
/// again until a timeout value is reached.  The timeout value is
/// an integer number of milliseconds passed in as the first
/// argument.
///
/// Return non-zero to retry the lock.  Return zero to stop trying
/// and cause SQLite to return SQLITE_BUSY.
///
/// # Arguments
///
/// * `ptr` - Database connection
/// * `count` - Number of times table has been busy
#[unsafe(link_section = ".text.slate_distinct.main.sqliteDefaultBusyCallback")]
extern "C-unwind" fn sqliteDefaultBusyCallback(mut ptr: *mut (), mut count: i32) -> i32 {
    // This case is for systems that have support for sleeping for fractions of
    // a second.  Examples:  All windows systems, unix systems with nanosleep()
    let mut db: *mut sqlite3 = ptr as *mut sqlite3;
    let mut tmout: i32 = unsafe { (*db).busyTimeout };
    let mut delay: i32 = 0 as i32;
    let mut prior: i32 = 0 as i32;
    0 as i32;
    if count < ((((12 as u64) / (1 as u64)) as u32) as i32) {
        delay = ((unsafe {
            *unsafe { unsafe { std::ptr::addr_of!(delays) as *const u8 }.offset(count as isize) }
        }) as u32) as i32;
        prior = ((unsafe {
            *unsafe { unsafe { std::ptr::addr_of!(totals) as *const u8 }.offset(count as isize) }
        }) as u32) as i32;
    } else {
        delay = ((unsafe {
            *unsafe {
                unsafe { std::ptr::addr_of!(delays) as *const u8 }
                    .offset((((((12 as u64) / (1 as u64)) as u32) as i32) - (1 as i32)) as isize)
            }
        }) as u32) as i32;
        prior = (((unsafe {
            *unsafe {
                unsafe { std::ptr::addr_of!(totals) as *const u8 }
                    .offset((((((12 as u64) / (1 as u64)) as u32) as i32) - (1 as i32)) as isize)
            }
        }) as u32) as i32)
            + delay * (count - (((((12 as u64) / (1 as u64)) as u32) as i32) - (1 as i32)));
    }
    if prior + delay > tmout {
        delay = tmout - prior;
        if delay <= (0 as i32) {
            return 0 as i32;
        }
    }
    unsafe { sqlite3OsSleep(unsafe { (*db).pVfs }, delay * (1000 as i32)) };
    return 1 as i32;
}

static mut delays: [u8; 12] = [
    ((1 as i32) as i8) as u8,
    ((2 as i32) as i8) as u8,
    ((5 as i32) as i8) as u8,
    ((10 as i32) as i8) as u8,
    ((15 as i32) as i8) as u8,
    ((20 as i32) as i8) as u8,
    ((25 as i32) as i8) as u8,
    ((25 as i32) as i8) as u8,
    ((25 as i32) as i8) as u8,
    ((50 as i32) as i8) as u8,
    ((50 as i32) as i8) as u8,
    ((100 as i32) as i8) as u8,
];

static mut totals: [u8; 12] = [
    ((0 as i32) as i8) as u8,
    ((1 as i32) as i8) as u8,
    ((3 as i32) as i8) as u8,
    ((8 as i32) as i8) as u8,
    ((18 as i32) as i8) as u8,
    ((33 as i32) as i8) as u8,
    ((53 as i32) as i8) as u8,
    ((78 as i32) as i8) as u8,
    ((103 as i32) as i8) as u8,
    ((128 as i32) as i8) as u8,
    ((178 as i32) as i8) as u8,
    ((228 as i32) as i8) as u8,
];

/// Invoke the given busy handler.
///
/// This routine is called when an operation failed to acquire a
/// lock on VFS file pFile.
///
/// If this routine returns non-zero, the lock is retried.  If it
/// returns 0, the operation aborts with an SQLITE_BUSY error.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3InvokeBusyHandler(mut p: *mut BusyHandler) -> i32 {
    let mut rc: i32 = 0 as i32;
    if (unsafe { (*p).xBusyHandler }) == None || (unsafe { (*p).nBusy }) < (0 as i32) {
        return 0 as i32;
    }
    rc = unsafe {
        unsafe { (*p).xBusyHandler }.unwrap()(unsafe { (*p).pBusyArg }, unsafe { (*p).nBusy })
    };
    if rc == (0 as i32) {
        unsafe {
            (*p).nBusy = -(1 as i32);
        }
    } else {
        let __v1931: *mut BusyHandler = p;
        let __v1932: i32 = unsafe { (*__v1931).nBusy };
        let __v1933: i32 = __v1932 + (1 as i32);
        unsafe {
            (*__v1931).nBusy = __v1933;
        }
    }
    return rc;
}

/// This routine sets the busy callback for an Sqlite database to the
/// given callback function with the given argument.
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.main.sqlite3_busy_handler")]
extern "C-unwind" fn sqlite3_busy_handler(
    mut db: *mut sqlite3,
    mut xBusy: Option<unsafe extern "C-unwind" fn(*mut (), i32) -> i32>,
    mut pArg: *mut (),
) -> i32 {
    unsafe { sqlite3_mutex_enter(unsafe { (*db).mutex }) };
    unsafe {
        (*db).busyHandler.xBusyHandler = xBusy;
    }
    unsafe {
        (*db).busyHandler.pBusyArg = pArg;
    }
    unsafe {
        (*db).busyHandler.nBusy = 0 as i32;
    }
    unsafe {
        (*db).busyTimeout = 0 as i32;
    }
    unsafe { sqlite3_mutex_leave(unsafe { (*db).mutex }) };
    return 0 as i32;
}

/// This routine sets the progress callback for an Sqlite database to the
/// given callback function with the given argument. The progress callback will
/// be invoked every nOps opcodes.
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.main.sqlite3_progress_handler")]
extern "C-unwind" fn sqlite3_progress_handler(
    mut db: *mut sqlite3,
    mut nOps: i32,
    mut xProgress: Option<unsafe extern "C-unwind" fn(*mut ()) -> i32>,
    mut pArg: *mut (),
) {
    unsafe { sqlite3_mutex_enter(unsafe { (*db).mutex }) };
    if nOps > (0 as i32) {
        unsafe {
            (*db).xProgress = xProgress;
        }
        unsafe {
            (*db).nProgressOps = nOps as u32;
        }
        unsafe {
            (*db).pProgressArg = pArg;
        }
    } else {
        unsafe {
            (*db).xProgress = None;
        }
        unsafe {
            (*db).nProgressOps = (0 as i32) as u32;
        }
        unsafe {
            (*db).pProgressArg = std::ptr::null_mut::<()>();
        }
    }
    unsafe { sqlite3_mutex_leave(unsafe { (*db).mutex }) };
}

/// This routine installs a default busy handler that waits for the
/// specified number of milliseconds before returning 0.
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.main.sqlite3_busy_timeout")]
extern "C-unwind" fn sqlite3_busy_timeout(mut db: *mut sqlite3, mut ms: i32) -> i32 {
    unsafe { sqlite3_mutex_enter(unsafe { (*db).mutex }) };
    if ms > (0 as i32) {
        sqlite3_busy_handler(db, Some(sqliteDefaultBusyCallback), db as *mut ());
        unsafe {
            (*db).busyTimeout = ms;
        }
    } else {
        sqlite3_busy_handler(db, None, std::ptr::null_mut::<()>());
    }
    unsafe { sqlite3_mutex_leave(unsafe { (*db).mutex }) };
    return 0 as i32;
}

/// Set the setlk timeout value.
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.main.sqlite3_setlk_timeout")]
extern "C-unwind" fn sqlite3_setlk_timeout(
    mut db: *mut sqlite3,
    mut ms: i32,
    mut flags: i32,
) -> i32 {
    if ms < -(1 as i32) {
        return 25 as i32;
    }
    db;
    flags;
    return 0 as i32;
}

/// Cause any pending operation to stop at its earliest opportunity.
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.main.sqlite3_interrupt")]
extern "C-unwind" fn sqlite3_interrupt(mut db: *mut sqlite3) {
    unsafe {
        std::sync::atomic::AtomicI32::store_volatile(
            std::sync::atomic::AtomicI32::from_ptr_raw(
                (unsafe { std::ptr::addr_of_mut!((*db).u1.isInterrupted) }) as *mut i32,
            ),
            1 as i32,
            std::sync::atomic::Ordering::Relaxed,
        )
    };
}

/// Return true or false depending on whether or not an interrupt is
/// pending on connection db.
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.main.sqlite3_is_interrupted")]
extern "C-unwind" fn sqlite3_is_interrupted(mut db: *mut sqlite3) -> i32 {
    return ((unsafe {
        std::sync::atomic::AtomicI32::load_volatile(
            std::sync::atomic::AtomicI32::from_ptr_raw(
                (unsafe { std::ptr::addr_of_mut!((*db).u1.isInterrupted) }) as *mut i32,
            ),
            std::sync::atomic::Ordering::Relaxed,
        )
    }) != (0 as i32)) as i32;
}

/// This function is exactly the same as sqlite3_create_function(), except
/// that it is designed to be called by internal code. The difference is
/// that if a malloc() fails in sqlite3_create_function(), an error code
/// is returned and the mallocFailed flag cleared.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3CreateFunc(
    mut db: *mut sqlite3,
    mut zFunctionName: *const i8,
    mut nArg: i32,
    mut enc: i32,
    mut pUserData: *mut (),
    mut xSFunc: Option<
        unsafe extern "C-unwind" fn(*mut sqlite3_context, i32, *mut *mut sqlite3_value),
    >,
    mut xStep: Option<
        unsafe extern "C-unwind" fn(*mut sqlite3_context, i32, *mut *mut sqlite3_value),
    >,
    mut xFinal: Option<unsafe extern "C-unwind" fn(*mut sqlite3_context)>,
    mut xValue: Option<unsafe extern "C-unwind" fn(*mut sqlite3_context)>,
    mut xInverse: Option<
        unsafe extern "C-unwind" fn(*mut sqlite3_context, i32, *mut *mut sqlite3_value),
    >,
    mut pDestructor: *mut FuncDestructor,
) -> i32 {
    let mut p: *mut FuncDef = unsafe { std::mem::zeroed() };
    let mut extraFlags: i32 = 0 as i32;
    0 as i32;
    0 as i32;
    let __v1934: bool;
    if zFunctionName == std::ptr::null::<i8>()
        || xSFunc != None && xFinal != None
        || ((xFinal == None) as i32) != ((xStep == None) as i32)
        || ((xValue == None) as i32) != ((xInverse == None) as i32)
        || (nArg < -(1 as i32) || nArg > (1000 as i32))
    {
        __v1934 = true as bool;
    } else {
        __v1934 = (255 as i32) < unsafe { sqlite3Strlen30(zFunctionName) };
    }
    if __v1934 {
        return sqlite3MisuseError(1988 as i32);
    }
    // Must have a valid name
    // Not both xSFunc and xFinal
    // Both or neither of xFinal and xStep
    // Both or neither of xValue, xInverse
    0 as i32;
    0 as i32;
    extraFlags = enc
        & ((2048 as i32)
            | (524288 as i32)
            | (1048576 as i32)
            | (2097152 as i32)
            | (16777216 as i32)
            | (33554432 as i32));
    let __v1935: i32 = enc;
    let __v1936: i32 = __v1935 & ((3 as i32) | (5 as i32));
    enc = __v1936;
    // The SQLITE_INNOCUOUS flag is the same bit as SQLITE_FUNC_UNSAFE.  But
    // the meaning is inverted.  So flip the bit.
    0 as i32;
    let __v1937: i32 = extraFlags;
    let __v1938: i32 = __v1937 ^ (2097152 as i32);
    extraFlags = __v1938; // tag-20230109-1
    // If SQLITE_UTF16 is specified as the encoding type, transform this
    // to one of SQLITE_UTF16LE or SQLITE_UTF16BE using the
    // SQLITE_UTF16NATIVE macro. SQLITE_UTF16 is not used internally.
    //
    // If SQLITE_ANY is specified, add three versions of the function
    // to the hash table.
    '__slate_break_1741: {
        match enc {
            4 => {
                enc = 2 as i32;
            }
            5 => {
                let mut rc: i32 = 0 as i32;
                rc = sqlite3CreateFunc(
                    db,
                    zFunctionName,
                    nArg,
                    ((1 as i32) | extraFlags) ^ (2097152 as i32),
                    pUserData,
                    xSFunc,
                    xStep,
                    xFinal,
                    xValue,
                    xInverse,
                    pDestructor,
                ); // tag-20230109-1
                if rc == (0 as i32) {
                    rc = sqlite3CreateFunc(
                        db,
                        zFunctionName,
                        nArg,
                        ((2 as i32) | extraFlags) ^ (2097152 as i32),
                        pUserData,
                        xSFunc,
                        xStep,
                        xFinal,
                        xValue,
                        xInverse,
                        pDestructor,
                    ); // tag-20230109-1
                }
                if rc != (0 as i32) {
                    return rc;
                }
                enc = 3 as i32;
            }
            1 | 2 | 3 => {}
            _ => {
                enc = 1 as i32;
            }
        }
    }
    // Check if an existing function is being overridden or deleted. If so,
    // and there are active VMs, then return SQLITE_BUSY. If a function
    // is being overridden/deleted but there are no active VMs, allow the
    // operation to continue but invalidate all precompiled statements.
    p = unsafe {
        sqlite3FindFunction(
            db,
            zFunctionName,
            nArg,
            (enc as i8) as u8,
            ((0 as i32) as i8) as u8,
        )
    };
    if p != std::ptr::null_mut::<FuncDef>()
        && (unsafe { (*p).funcFlags }) & ((3 as i32) as u32) == (enc as u32)
        && ((unsafe { (*p).nArg }) as i32) == nArg
    {
        if (unsafe { (*db).nVdbeActive }) != (0 as i32) {
            unsafe {
                sqlite3ErrorWithMsg(
                    db,
                    5 as i32,
                    (b"unable to delete/modify user-function due to active statements\0".as_ptr()
                        as *mut i8) as *const i8,
                )
            };
            0 as i32;
            return 5 as i32;
        } else {
            unsafe { sqlite3ExpirePreparedStatements(db, 0 as i32) };
        }
    } else {
        if xSFunc == None && xFinal == None {
            // Trying to delete a function that does not exist.  This is a no-op.
            // https://sqlite.org/forum/forumpost/726219164b
            return 0 as i32;
        }
    }
    p = unsafe {
        sqlite3FindFunction(
            db,
            zFunctionName,
            nArg,
            (enc as i8) as u8,
            ((1 as i32) as i8) as u8,
        )
    };
    0 as i32;
    if !(p != std::ptr::null_mut::<FuncDef>()) {
        return 7 as i32;
    }
    // If an older version of the function with a configured destructor is
    // being replaced invoke the destructor function here.
    functionDestroy(db, p);
    if pDestructor != std::ptr::null_mut::<FuncDestructor>() {
        let __v1939: *mut FuncDestructor = pDestructor;
        let __v1940: i32 = unsafe { (*__v1939).nRef };
        let __v1941: i32 = __v1940 + (1 as i32);
        unsafe {
            (*__v1939).nRef = __v1941;
        }
    }
    unsafe {
        (*p).u.pDestructor = pDestructor;
    }
    unsafe {
        (*p).funcFlags = (unsafe { (*p).funcFlags }) & ((3 as i32) as u32) | (extraFlags as u32);
    }
    {}
    {}
    unsafe {
        (*p).xSFunc = {
            let __t0: Option<
                unsafe extern "C-unwind" fn(*mut sqlite3_context, i32, *mut *mut sqlite3_value),
            > = if xSFunc != None { xSFunc } else { xStep };
            __t0
        };
    }
    unsafe {
        (*p).xFinalize = xFinal;
    }
    unsafe {
        (*p).xValue = xValue;
    }
    unsafe {
        (*p).xInverse = xInverse;
    }
    unsafe {
        (*p).pUserData = pUserData;
    }
    unsafe {
        (*p).nArg = ((nArg as i16) as u16) as i16;
    }
    return 0 as i32;
}

/// Worker function used by utf-8 APIs that create new functions:
///
///    sqlite3_create_function()
///    sqlite3_create_function_v2()
///    sqlite3_create_window_function()
fn createFunctionApi(
    mut db: *mut sqlite3,
    mut zFunc: *const i8,
    mut nArg: i32,
    mut enc: i32,
    mut p: *mut (),
    mut xSFunc: Option<
        unsafe extern "C-unwind" fn(*mut sqlite3_context, i32, *mut *mut sqlite3_value),
    >,
    mut xStep: Option<
        unsafe extern "C-unwind" fn(*mut sqlite3_context, i32, *mut *mut sqlite3_value),
    >,
    mut xFinal: Option<unsafe extern "C-unwind" fn(*mut sqlite3_context)>,
    mut xValue: Option<unsafe extern "C-unwind" fn(*mut sqlite3_context)>,
    mut xInverse: Option<
        unsafe extern "C-unwind" fn(*mut sqlite3_context, i32, *mut *mut sqlite3_value),
    >,
    mut xDestroy: Option<unsafe extern "C-unwind" fn(*mut ())>,
) -> i32 {
    let mut __slate_storage_892: std::mem::MaybeUninit<*mut FuncDestructor> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_892: *mut *mut FuncDestructor =
        std::ptr::addr_of_mut!(__slate_storage_892) as *mut *mut FuncDestructor;
    let mut __slate_storage_891: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_891: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_891) as *mut i32;
    unsafe {
        '__join_0: {
            std::ptr::write(__slate_slot_891, 1 as i32);
            std::ptr::write(__slate_slot_892, std::ptr::null_mut::<FuncDestructor>());
            unsafe { sqlite3_mutex_enter(unsafe { (*db).mutex }) };
            if xDestroy != None {
                *__slate_slot_892 = (unsafe { sqlite3Malloc(24 as u64) }) as *mut FuncDestructor;
                if !(*__slate_slot_892 != std::ptr::null_mut::<FuncDestructor>()) {
                    unsafe { sqlite3OomFault(db) };
                    unsafe { xDestroy.unwrap()(p) };
                    break '__join_0;
                } else {
                    unsafe {
                        (*(*__slate_slot_892)).nRef = 0 as i32;
                    }
                    unsafe {
                        (*(*__slate_slot_892)).xDestroy = xDestroy;
                    }
                    unsafe {
                        (*(*__slate_slot_892)).pUserData = p;
                    }
                }
            }
            *__slate_slot_891 = sqlite3CreateFunc(
                db,
                zFunc,
                nArg,
                enc,
                p,
                xSFunc,
                xStep,
                xFinal,
                xValue,
                xInverse,
                *__slate_slot_892,
            );
            if *__slate_slot_892 != std::ptr::null_mut::<FuncDestructor>()
                && (unsafe { (*(*__slate_slot_892)).nRef }) == (0 as i32)
            {
                0 as i32;
                unsafe { xDestroy.unwrap()(p) };
                unsafe { sqlite3_free(*__slate_slot_892 as *mut ()) };
            }
        }
        *__slate_slot_891 = unsafe { sqlite3ApiExit(db, *__slate_slot_891) };
        unsafe { sqlite3_mutex_leave(unsafe { (*db).mutex }) };
        return *__slate_slot_891;
    }
    return unsafe { std::mem::zeroed() };
}

/// Create new user functions.
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.main.sqlite3_create_function")]
extern "C-unwind" fn sqlite3_create_function(
    mut db: *mut sqlite3,
    mut zFunc: *const i8,
    mut nArg: i32,
    mut enc: i32,
    mut p: *mut (),
    mut xSFunc: Option<
        unsafe extern "C-unwind" fn(*mut sqlite3_context, i32, *mut *mut sqlite3_value),
    >,
    mut xStep: Option<
        unsafe extern "C-unwind" fn(*mut sqlite3_context, i32, *mut *mut sqlite3_value),
    >,
    mut xFinal: Option<unsafe extern "C-unwind" fn(*mut sqlite3_context)>,
) -> i32 {
    return createFunctionApi(
        db, zFunc, nArg, enc, p, xSFunc, xStep, xFinal, None, None, None,
    );
}

#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3_create_function_v2(
    mut db: *mut sqlite3,
    mut zFunc: *const i8,
    mut nArg: i32,
    mut enc: i32,
    mut p: *mut (),
    mut xSFunc: Option<
        unsafe extern "C-unwind" fn(*mut sqlite3_context, i32, *mut *mut sqlite3_value),
    >,
    mut xStep: Option<
        unsafe extern "C-unwind" fn(*mut sqlite3_context, i32, *mut *mut sqlite3_value),
    >,
    mut xFinal: Option<unsafe extern "C-unwind" fn(*mut sqlite3_context)>,
    mut xDestroy: Option<unsafe extern "C-unwind" fn(*mut ())>,
) -> i32 {
    return createFunctionApi(
        db, zFunc, nArg, enc, p, xSFunc, xStep, xFinal, None, None, xDestroy,
    );
}

#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3_create_window_function(
    mut db: *mut sqlite3,
    mut zFunc: *const i8,
    mut nArg: i32,
    mut enc: i32,
    mut p: *mut (),
    mut xStep: Option<
        unsafe extern "C-unwind" fn(*mut sqlite3_context, i32, *mut *mut sqlite3_value),
    >,
    mut xFinal: Option<unsafe extern "C-unwind" fn(*mut sqlite3_context)>,
    mut xValue: Option<unsafe extern "C-unwind" fn(*mut sqlite3_context)>,
    mut xInverse: Option<
        unsafe extern "C-unwind" fn(*mut sqlite3_context, i32, *mut *mut sqlite3_value),
    >,
    mut xDestroy: Option<unsafe extern "C-unwind" fn(*mut ())>,
) -> i32 {
    return createFunctionApi(
        db, zFunc, nArg, enc, p, None, xStep, xFinal, xValue, xInverse, xDestroy,
    );
}

#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.main.sqlite3_create_function16")]
extern "C-unwind" fn sqlite3_create_function16(
    mut db: *mut sqlite3,
    mut zFunctionName: *const (),
    mut nArg: i32,
    mut eTextRep: i32,
    mut p: *mut (),
    mut xSFunc: Option<
        unsafe extern "C-unwind" fn(*mut sqlite3_context, i32, *mut *mut sqlite3_value),
    >,
    mut xStep: Option<
        unsafe extern "C-unwind" fn(*mut sqlite3_context, i32, *mut *mut sqlite3_value),
    >,
    mut xFinal: Option<unsafe extern "C-unwind" fn(*mut sqlite3_context)>,
) -> i32 {
    let mut rc: i32 = 0 as i32;
    let mut zFunc8: *mut i8 = unsafe { std::mem::zeroed() };
    unsafe { sqlite3_mutex_enter(unsafe { (*db).mutex }) };
    0 as i32;
    zFunc8 = unsafe { sqlite3Utf16to8(db, zFunctionName, -(1 as i32), ((2 as i32) as i8) as u8) };
    rc = sqlite3CreateFunc(
        db,
        zFunc8 as *const i8,
        nArg,
        eTextRep,
        p,
        xSFunc,
        xStep,
        xFinal,
        None,
        None,
        std::ptr::null_mut::<FuncDestructor>(),
    );
    unsafe { sqlite3DbFree(db, zFunc8 as *mut ()) };
    rc = unsafe { sqlite3ApiExit(db, rc) };
    unsafe { sqlite3_mutex_leave(unsafe { (*db).mutex }) };
    return rc;
}

/// The following is the implementation of an SQL function that always
/// fails with an error message stating that the function is used in the
/// wrong context.  The sqlite3_overload_function() API might construct
/// SQL function that use this routine so that the functions will exist
/// for name resolution but are actually overloaded by the xFindFunction
/// method of virtual tables.
///
/// # Arguments
///
/// * `context` - The function calling context
/// * `NotUsed` - Number of arguments to the function
/// * `NotUsed2` - Value of each argument
#[unsafe(link_section = ".text.slate_distinct.main.sqlite3InvalidFunction")]
extern "C-unwind" fn sqlite3InvalidFunction(
    mut context: *mut sqlite3_context,
    mut NotUsed: i32,
    mut NotUsed2: *mut *mut sqlite3_value,
) {
    let mut zName: *const i8 = (unsafe { sqlite3_user_data(context) }) as *const i8;
    let mut zErr: *mut i8 = unsafe { std::mem::zeroed() };
    NotUsed;
    NotUsed2;
    zErr = unsafe {
        sqlite3_mprintf(
            (b"unable to use function %s in the requested context\0".as_ptr() as *mut i8)
                as *const i8,
            zName,
        )
    };
    unsafe { sqlite3_result_error(context, zErr as *const i8, -(1 as i32)) };
    unsafe { sqlite3_free(zErr as *mut ()) };
}

/// Declare that a function has been overloaded by a virtual table.
///
/// If the function already exists as a regular global function, then
/// this routine is a no-op.  If the function does not exist, then create
/// a new one that always throws a run-time error.
///
/// When virtual tables intend to provide an overloaded function, they
/// should call this routine to make sure the global function exists.
/// A global function must exist in order for name resolution to work
/// properly.
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.main.sqlite3_overload_function")]
extern "C-unwind" fn sqlite3_overload_function(
    mut db: *mut sqlite3,
    mut zName: *const i8,
    mut nArg: i32,
) -> i32 {
    let mut rc: i32 = 0 as i32;
    let mut zCopy: *mut i8 = unsafe { std::mem::zeroed() };
    unsafe { sqlite3_mutex_enter(unsafe { (*db).mutex }) };
    rc = ((unsafe {
        sqlite3FindFunction(
            db,
            zName,
            nArg,
            ((1 as i32) as i8) as u8,
            ((0 as i32) as i8) as u8,
        )
    }) != std::ptr::null_mut::<FuncDef>()) as i32;
    unsafe { sqlite3_mutex_leave(unsafe { (*db).mutex }) };
    if rc != (0 as i32) {
        return 0 as i32;
    }
    zCopy = unsafe { sqlite3_mprintf((b"%s\0".as_ptr() as *mut i8) as *const i8, zName) };
    if zCopy == std::ptr::null_mut::<i8>() {
        return 7 as i32;
    }
    return sqlite3_create_function_v2(
        db,
        zName,
        nArg,
        1 as i32,
        zCopy as *mut (),
        Some(sqlite3InvalidFunction),
        None,
        None,
        unsafe {
            std::mem::transmute::<*const (), Option<unsafe extern "C-unwind" fn(*mut ())>>(
                sqlite3_free as *const (),
            )
        },
    );
}

/// Register a trace function.  The pArg from the previously registered trace
/// is returned.
///
/// A NULL trace function means that no tracing is executes.  A non-NULL
/// trace is a pointer to a function that is invoked at the start of each
/// SQL statement.
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.main.sqlite3_trace")]
extern "C-unwind" fn sqlite3_trace(
    mut db: *mut sqlite3,
    mut xTrace: Option<unsafe extern "C-unwind" fn(*mut (), *const i8)>,
    mut pArg: *mut (),
) -> *mut () {
    let mut pOld: *mut () = unsafe { std::mem::zeroed() };
    unsafe { sqlite3_mutex_enter(unsafe { (*db).mutex }) };
    pOld = unsafe { (*db).pTraceArg };
    unsafe {
        (*db).mTrace = ((if xTrace != None { 64 as i32 } else { 0 as i32 }) as i8) as u8;
    }
    unsafe {
        (*db).trace.xLegacy = xTrace;
    }
    unsafe {
        (*db).pTraceArg = pArg;
    }
    unsafe { sqlite3_mutex_leave(unsafe { (*db).mutex }) };
    return pOld;
}

/// Register a trace callback using the version-2 interface.
///
/// # Arguments
///
/// * `db` - Trace this connection
/// * `mTrace` - Mask of events to be traced
/// * `xTrace` - Callback to invoke
/// * `pArg` - Context
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.main.sqlite3_trace_v2")]
extern "C-unwind" fn sqlite3_trace_v2(
    mut db: *mut sqlite3,
    mut mTrace: u32,
    mut xTrace: Option<unsafe extern "C-unwind" fn(u32, *mut (), *mut (), *mut ()) -> i32>,
    mut pArg: *mut (),
) -> i32 {
    unsafe { sqlite3_mutex_enter(unsafe { (*db).mutex }) };
    if mTrace == ((0 as i32) as u32) {
        xTrace = None;
    }
    if xTrace == None {
        mTrace = (0 as i32) as u32;
    }
    unsafe {
        (*db).mTrace = mTrace as u8;
    }
    unsafe {
        (*db).trace.xV2 = xTrace;
    }
    unsafe {
        (*db).pTraceArg = pArg;
    }
    unsafe { sqlite3_mutex_leave(unsafe { (*db).mutex }) };
    return 0 as i32;
}

/// Register a profile function.  The pArg from the previously registered
/// profile function is returned.
///
/// A NULL profile function means that no profiling is executes.  A non-NULL
/// profile is a pointer to a function that is invoked at the conclusion of
/// each SQL statement that is run.
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.main.sqlite3_profile")]
extern "C-unwind" fn sqlite3_profile(
    mut db: *mut sqlite3,
    mut xProfile: Option<unsafe extern "C-unwind" fn(*mut (), *const i8, u64)>,
    mut pArg: *mut (),
) -> *mut () {
    let mut pOld: *mut () = unsafe { std::mem::zeroed() };
    unsafe { sqlite3_mutex_enter(unsafe { (*db).mutex }) };
    pOld = unsafe { (*db).pProfileArg };
    unsafe {
        (*db).xProfile = xProfile;
    }
    unsafe {
        (*db).pProfileArg = pArg;
    }
    let __v1814: *mut sqlite3 = db;
    let __v1815: u8 = unsafe { (*__v1814).mTrace };
    let __v1816: u8 = ((((__v1815 as u32) as i32) & (15 as i32)) as i8) as u8;
    unsafe {
        (*__v1814).mTrace = __v1816;
    }
    if (unsafe { (*db).xProfile }) != None {
        let __v1817: *mut sqlite3 = db;
        let __v1818: u8 = unsafe { (*__v1817).mTrace };
        let __v1819: u8 = ((((__v1818 as u32) as i32) | (128 as i32)) as i8) as u8;
        unsafe {
            (*__v1817).mTrace = __v1819;
        }
    }
    unsafe { sqlite3_mutex_leave(unsafe { (*db).mutex }) };
    return pOld;
}

/// Register a function to be invoked when a transaction commits.
/// If the invoked function returns non-zero, then the commit becomes a
/// rollback.
///
/// # Arguments
///
/// * `db` - Attach the hook to this database
/// * `xCallback` - Function to invoke on each commit
/// * `pArg` - Argument to the function
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.main.sqlite3_commit_hook")]
extern "C-unwind" fn sqlite3_commit_hook(
    mut db: *mut sqlite3,
    mut xCallback: Option<unsafe extern "C-unwind" fn(*mut ()) -> i32>,
    mut pArg: *mut (),
) -> *mut () {
    let mut pOld: *mut () = unsafe { std::mem::zeroed() };
    unsafe { sqlite3_mutex_enter(unsafe { (*db).mutex }) };
    pOld = unsafe { (*db).pCommitArg };
    unsafe {
        (*db).xCommitCallback = xCallback;
    }
    unsafe {
        (*db).pCommitArg = pArg;
    }
    unsafe { sqlite3_mutex_leave(unsafe { (*db).mutex }) };
    return pOld;
}

/// Register a callback to be invoked each time a row is updated,
/// inserted or deleted using this database connection.
///
/// # Arguments
///
/// * `db` - Attach the hook to this database
/// * `pArg` - Argument to the function
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.main.sqlite3_update_hook")]
extern "C-unwind" fn sqlite3_update_hook(
    mut db: *mut sqlite3,
    mut xCallback: Option<unsafe extern "C-unwind" fn(*mut (), i32, *const i8, *const i8, i64)>,
    mut pArg: *mut (),
) -> *mut () {
    let mut pRet: *mut () = unsafe { std::mem::zeroed() };
    unsafe { sqlite3_mutex_enter(unsafe { (*db).mutex }) };
    pRet = unsafe { (*db).pUpdateArg };
    unsafe {
        (*db).xUpdateCallback = xCallback;
    }
    unsafe {
        (*db).pUpdateArg = pArg;
    }
    unsafe { sqlite3_mutex_leave(unsafe { (*db).mutex }) };
    return pRet;
}

/// Register a callback to be invoked each time a transaction is rolled
/// back by this database connection.
///
/// # Arguments
///
/// * `db` - Attach the hook to this database
/// * `xCallback` - Callback function
/// * `pArg` - Argument to the function
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.main.sqlite3_rollback_hook")]
extern "C-unwind" fn sqlite3_rollback_hook(
    mut db: *mut sqlite3,
    mut xCallback: Option<unsafe extern "C-unwind" fn(*mut ())>,
    mut pArg: *mut (),
) -> *mut () {
    let mut pRet: *mut () = unsafe { std::mem::zeroed() };
    unsafe { sqlite3_mutex_enter(unsafe { (*db).mutex }) };
    pRet = unsafe { (*db).pRollbackArg };
    unsafe {
        (*db).xRollbackCallback = xCallback;
    }
    unsafe {
        (*db).pRollbackArg = pArg;
    }
    unsafe { sqlite3_mutex_leave(unsafe { (*db).mutex }) };
    return pRet;
}

/// Register a function to be invoked prior to each autovacuum that
/// determines the number of pages to vacuum.
///
/// # Arguments
///
/// * `db` - Attach the hook to this database
/// * `pArg` - Argument to the function
/// * `xDestructor` - Destructor for pArg
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.main.sqlite3_autovacuum_pages")]
extern "C-unwind" fn sqlite3_autovacuum_pages(
    mut db: *mut sqlite3,
    mut xCallback: Option<unsafe extern "C-unwind" fn(*mut (), *const i8, u32, u32, u32) -> u32>,
    mut pArg: *mut (),
    mut xDestructor: Option<unsafe extern "C-unwind" fn(*mut ())>,
) -> i32 {
    unsafe { sqlite3_mutex_enter(unsafe { (*db).mutex }) };
    if (unsafe { (*db).xAutovacDestr }) != None {
        unsafe { unsafe { (*db).xAutovacDestr }.unwrap()(unsafe { (*db).pAutovacPagesArg }) };
    }
    unsafe {
        (*db).xAutovacPages = xCallback;
    }
    unsafe {
        (*db).pAutovacPagesArg = pArg;
    }
    unsafe {
        (*db).xAutovacDestr = xDestructor;
    }
    unsafe { sqlite3_mutex_leave(unsafe { (*db).mutex }) };
    return 0 as i32;
}

/// The sqlite3_wal_hook() callback registered by sqlite3_wal_autocheckpoint().
/// Invoke sqlite3_wal_checkpoint if the number of frames in the log file
/// is greater than sqlite3.pWalArg cast to an integer (the value configured by
/// wal_autocheckpoint()).
///
/// # Arguments
///
/// * `pClientData` - Argument
/// * `db` - Connection
/// * `zDb` - Database
/// * `nFrame` - Size of WAL
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.main.sqlite3WalDefaultHook")]
extern "C-unwind" fn sqlite3WalDefaultHook(
    mut pClientData: *mut (),
    mut db: *mut sqlite3,
    mut zDb: *const i8,
    mut nFrame: i32,
) -> i32 {
    if nFrame >= ((pClientData as i64) as i32) {
        unsafe { sqlite3BeginBenignMalloc() };
        sqlite3_wal_checkpoint(db, zDb);
        unsafe { sqlite3EndBenignMalloc() };
    }
    return 0 as i32;
}

/// Configure an sqlite3_wal_hook() callback to automatically checkpoint
/// a database after committing a transaction if there are nFrame or
/// more frames in the log file. Passing zero or a negative value as the
/// nFrame parameter disables automatic checkpoints entirely.
///
/// The callback registered by this function replaces any existing callback
/// registered using sqlite3_wal_hook(). Likewise, registering a callback
/// using sqlite3_wal_hook() disables the automatic checkpoint mechanism
/// configured by this function.
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.main.sqlite3_wal_autocheckpoint")]
extern "C-unwind" fn sqlite3_wal_autocheckpoint(mut db: *mut sqlite3, mut nFrame: i32) -> i32 {
    if nFrame > (0 as i32) {
        sqlite3_wal_hook(db, Some(sqlite3WalDefaultHook), (nFrame as i64) as *mut ());
    } else {
        sqlite3_wal_hook(db, None, std::ptr::null_mut::<()>());
    }
    return 0 as i32;
}

/// Register a callback to be invoked each time a transaction is written
/// into the write-ahead-log by this database connection.
///
/// # Arguments
///
/// * `db` - Attach the hook to this db handle
/// * `pArg` - First argument passed to xCallback()
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.main.sqlite3_wal_hook")]
extern "C-unwind" fn sqlite3_wal_hook(
    mut db: *mut sqlite3,
    mut xCallback: Option<
        unsafe extern "C-unwind" fn(*mut (), *mut sqlite3, *const i8, i32) -> i32,
    >,
    mut pArg: *mut (),
) -> *mut () {
    let mut pRet: *mut () = unsafe { std::mem::zeroed() };
    unsafe { sqlite3_mutex_enter(unsafe { (*db).mutex }) };
    pRet = unsafe { (*db).pWalArg };
    unsafe {
        (*db).xWalCallback = xCallback;
    }
    unsafe {
        (*db).pWalArg = pArg;
    }
    unsafe { sqlite3_mutex_leave(unsafe { (*db).mutex }) };
    return pRet;
}

/// Checkpoint database zDb.
///
/// # Arguments
///
/// * `db` - Database handle
/// * `zDb` - Name of attached database (or NULL)
/// * `eMode` - SQLITE_CHECKPOINT_* value
/// * `pnLog` - OUT: Size of WAL log in frames
/// * `pnCkpt` - OUT: Total number of frames checkpointed
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.main.sqlite3_wal_checkpoint_v2")]
extern "C-unwind" fn sqlite3_wal_checkpoint_v2(
    mut db: *mut sqlite3,
    mut zDb: *const i8,
    mut eMode: i32,
    mut pnLog: *mut i32,
    mut pnCkpt: *mut i32,
) -> i32 {
    let mut rc: i32 = 0 as i32; // Return code
    let mut iDb: i32 = 0 as i32; // Schema to checkpoint
    // Initialize the output variables to -1 in case an error occurs.
    if pnLog != std::ptr::null_mut::<i32>() {
        unsafe {
            *pnLog = -(1 as i32);
        }
    }
    if pnCkpt != std::ptr::null_mut::<i32>() {
        unsafe {
            *pnCkpt = -(1 as i32);
        }
    }
    0 as i32;
    0 as i32;
    0 as i32;
    0 as i32;
    0 as i32;
    if eMode < -(1 as i32) || eMode > (3 as i32) {
        // EVIDENCE-OF: R-03996-12088 The M parameter must be a valid checkpoint
        // mode:
        return sqlite3MisuseError(2613 as i32);
    }
    unsafe { sqlite3_mutex_enter(unsafe { (*db).mutex }) };
    if zDb != std::ptr::null::<i8>()
        && (unsafe { *unsafe { zDb.offset((0 as i32) as isize) } }) != (0 as i8)
    {
        iDb = unsafe { sqlite3FindDbName(db, zDb) };
    } else {
        iDb = (10 as i32) + (2 as i32); // This means process all schemas
    }
    if iDb < (0 as i32) {
        rc = 1 as i32;
        unsafe {
            sqlite3ErrorWithMsg(
                db,
                1 as i32,
                (b"unknown database: %s\0".as_ptr() as *mut i8) as *const i8,
                zDb,
            )
        };
    } else {
        unsafe {
            (*db).busyHandler.nBusy = 0 as i32;
        }
        rc = sqlite3Checkpoint(db, iDb, eMode, pnLog, pnCkpt);
        unsafe { sqlite3Error(db, rc) };
    }
    rc = unsafe { sqlite3ApiExit(db, rc) };
    // If there are no active statements, clear the interrupt flag at this
    // point.
    if (unsafe { (*db).nVdbeActive }) == (0 as i32) {
        unsafe {
            std::sync::atomic::AtomicI32::store_volatile(
                std::sync::atomic::AtomicI32::from_ptr_raw(
                    (unsafe { std::ptr::addr_of_mut!((*db).u1.isInterrupted) }) as *mut i32,
                ),
                0 as i32,
                std::sync::atomic::Ordering::Relaxed,
            )
        };
    }
    unsafe { sqlite3_mutex_leave(unsafe { (*db).mutex }) };
    return rc;
}

/// Checkpoint database zDb. If zDb is NULL, or if the buffer zDb points
/// to contains a zero-length string, all attached databases are
/// checkpointed.
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.main.sqlite3_wal_checkpoint")]
extern "C-unwind" fn sqlite3_wal_checkpoint(mut db: *mut sqlite3, mut zDb: *const i8) -> i32 {
    // EVIDENCE-OF: R-41613-20553 The sqlite3_wal_checkpoint(D,X) is equivalent to
    // sqlite3_wal_checkpoint_v2(D,X,SQLITE_CHECKPOINT_PASSIVE,0,0).
    return sqlite3_wal_checkpoint_v2(
        db,
        zDb,
        0 as i32,
        std::ptr::null_mut::<i32>(),
        std::ptr::null_mut::<i32>(),
    );
}

/// Run a checkpoint on database iDb. This is a no-op if database iDb is
/// not currently open in WAL mode.
///
/// If a transaction is open on the database being checkpointed, this
/// function returns SQLITE_LOCKED and a checkpoint is not attempted. If
/// an error occurs while running the checkpoint, an SQLite error code is
/// returned (i.e. SQLITE_IOERR). Otherwise, SQLITE_OK.
///
/// The mutex on database handle db should be held by the caller. The mutex
/// associated with the specific b-tree being checkpointed is taken by
/// this function while the checkpoint is running.
///
/// If iDb is passed SQLITE_MAX_DB then all attached databases are
/// checkpointed. If an error is encountered it is returned immediately -
/// no attempt is made to checkpoint any remaining databases.
///
/// Parameter eMode is one of SQLITE_CHECKPOINT_PASSIVE, FULL, RESTART
/// or TRUNCATE.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3Checkpoint(
    mut db: *mut sqlite3,
    mut iDb: i32,
    mut eMode: i32,
    mut pnLog: *mut i32,
    mut pnCkpt: *mut i32,
) -> i32 {
    let mut rc: i32 = 0 as i32; // Return code
    let mut i: i32 = 0 as i32; // Used to iterate through attached dbs
    let mut bBusy: i32 = 0 as i32; // True if SQLITE_BUSY has been encountered
    0 as i32;
    0 as i32;
    0 as i32;
    {}
    // See forum post a006d86f72
    {}
    i = 0 as i32;
    '__slate_break_1746: loop {
        if !(i < unsafe { (*db).nDb } && rc == (0 as i32)) {
            break;
        }
        if i == iDb || iDb == (10 as i32) + (2 as i32) {
            rc = unsafe {
                sqlite3BtreeCheckpoint(
                    unsafe { (*unsafe { unsafe { (*db).aDb }.offset(i as isize) }).pBt },
                    eMode,
                    pnLog,
                    pnCkpt,
                )
            };
            pnLog = std::ptr::null_mut::<i32>();
            pnCkpt = std::ptr::null_mut::<i32>();
            if rc == (5 as i32) {
                bBusy = 1 as i32;
                rc = 0 as i32;
            }
        }
        let __v1942: i32 = i;
        let __v1943: i32 = __v1942 + (1 as i32);
        i = __v1943;
    }
    return if rc == (0 as i32) && bBusy != (0 as i32) {
        5 as i32
    } else {
        rc
    };
}

/// This function returns true if main-memory should be used instead of
/// a temporary file for transient pager files and statement journals.
/// The value returned depends on the value of db->temp_store (runtime
/// parameter) and the compile time value of SQLITE_TEMP_STORE. The
/// following table describes the relationship between these two values
/// and this functions return value.
///
///   SQLITE_TEMP_STORE     db->temp_store     Location of temporary database
///   -----------------     --------------     ------------------------------
///   0                     any                file      (return 0)
///   1                     1                  file      (return 0)
///   1                     2                  memory    (return 1)
///   1                     0                  file      (return 0)
///   2                     1                  file      (return 0)
///   2                     2                  memory    (return 1)
///   2                     0                  memory    (return 1)
///   3                     any                memory    (return 1)
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3TempInMemory(mut db: *const sqlite3) -> i32 {
    return ((((unsafe { (*db).temp_store }) as u32) as i32) == (2 as i32)) as i32;
}

/// Return UTF-8 encoded English language explanation of the most recent
/// error.
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.main.sqlite3_errmsg")]
extern "C-unwind" fn sqlite3_errmsg(mut db: *mut sqlite3) -> *const i8 {
    let mut z: *const i8 = unsafe { std::mem::zeroed() };
    if !(db != std::ptr::null_mut::<sqlite3>()) {
        return sqlite3ErrStr(7 as i32);
    }
    if !((unsafe { sqlite3SafetyCheckSickOrOk(db) }) != (0 as i32)) {
        return sqlite3ErrStr(sqlite3MisuseError(2749 as i32));
    }
    unsafe { sqlite3_mutex_enter(unsafe { (*db).mutex }) };
    if (unsafe { (*db).mallocFailed }) != (0 as u8) {
        z = sqlite3ErrStr(7 as i32);
    } else {
        {}
        let __v1854: *mut i8;
        if (unsafe { (*db).errCode }) != (0 as i32) {
            __v1854 = (unsafe { sqlite3_value_text(unsafe { (*db).pErr }) }) as *mut i8;
        } else {
            __v1854 = std::ptr::null_mut::<i8>();
        }
        z = __v1854 as *const i8;
        0 as i32;
        if z == std::ptr::null::<i8>() {
            z = sqlite3ErrStr(unsafe { (*db).errCode });
        }
    }
    unsafe { sqlite3_mutex_leave(unsafe { (*db).mutex }) };
    return z;
}

/// Set the error code and error message associated with the database handle.
///
/// This routine is intended to be called by outside extensions (ex: the
/// Session extension). Internal logic should invoke sqlite3Error() or
/// sqlite3ErrorWithMsg() directly.
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.main.sqlite3_set_errmsg")]
extern "C-unwind" fn sqlite3_set_errmsg(
    mut db: *mut sqlite3,
    mut errcode: i32,
    mut zMsg: *const i8,
) -> i32 {
    let mut rc: i32 = 0 as i32;
    if !((unsafe { sqlite3SafetyCheckOk(db) }) != (0 as i32)) {
        return sqlite3MisuseError(2776 as i32);
    }
    unsafe { sqlite3_mutex_enter(unsafe { (*db).mutex }) };
    if zMsg != std::ptr::null::<i8>() {
        unsafe {
            sqlite3ErrorWithMsg(
                db,
                errcode,
                (b"%s\0".as_ptr() as *mut i8) as *const i8,
                zMsg,
            )
        };
    } else {
        unsafe { sqlite3Error(db, errcode) };
    }
    rc = unsafe { sqlite3ApiExit(db, rc) };
    unsafe { sqlite3_mutex_leave(unsafe { (*db).mutex }) };
    return rc;
}

/// Return the byte offset of the most recent error
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.main.sqlite3_error_offset")]
extern "C-unwind" fn sqlite3_error_offset(mut db: *mut sqlite3) -> i32 {
    let mut iOffset: i32 = -(1 as i32);
    let __v1855: bool;
    if db != std::ptr::null_mut::<sqlite3>() {
        __v1855 = (unsafe { sqlite3SafetyCheckSickOrOk(db) }) != (0 as i32);
    } else {
        __v1855 = false as bool;
    }
    if __v1855 {
        unsafe { sqlite3_mutex_enter(unsafe { (*db).mutex }) };
        if (unsafe { (*db).errCode }) != (0 as i32) {
            iOffset = unsafe { (*db).errByteOffset };
        }
        unsafe { sqlite3_mutex_leave(unsafe { (*db).mutex }) };
    }
    return iOffset;
}

/// Return UTF-16 encoded English language explanation of the most recent
/// error.
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.main.sqlite3_errmsg16")]
extern "C-unwind" fn sqlite3_errmsg16(mut db: *mut sqlite3) -> *const () {
    let mut z: *const () = unsafe { std::mem::zeroed() };
    if !(db != std::ptr::null_mut::<sqlite3>()) {
        return ((unsafe { std::ptr::addr_of!(outOfMem.0) as *const u16 }) as *mut ()) as *const ();
    }
    if !((unsafe { sqlite3SafetyCheckSickOrOk(db) }) != (0 as i32)) {
        return ((unsafe { std::ptr::addr_of!(misuse.0) as *const u16 }) as *mut ()) as *const ();
    }
    unsafe { sqlite3_mutex_enter(unsafe { (*db).mutex }) };
    if (unsafe { (*db).mallocFailed }) != (0 as u8) {
        z = ((unsafe { std::ptr::addr_of!(outOfMem.0) as *const u16 }) as *mut ()) as *const ();
    } else {
        z = unsafe { sqlite3_value_text16(unsafe { (*db).pErr }) };
        if z == std::ptr::null::<()>() {
            unsafe {
                sqlite3ErrorWithMsg(
                    db,
                    unsafe { (*db).errCode },
                    sqlite3ErrStr(unsafe { (*db).errCode }),
                )
            };
            z = unsafe { sqlite3_value_text16(unsafe { (*db).pErr }) };
        }
        // A malloc() may have failed within the call to sqlite3_value_text16()
        // above. If this is the case, then the db->mallocFailed flag needs to
        // be cleared before returning. Do this directly, instead of via
        // sqlite3ApiExit(), to avoid setting the database handle error message.
        unsafe { sqlite3OomClear(db) };
    }
    unsafe { sqlite3_mutex_leave(unsafe { (*db).mutex }) };
    return z;
}

static mut outOfMem: __SlateAlign16<[u16; 14]> = __SlateAlign16([
    ((111 as i32) as i16) as u16,
    ((117 as i32) as i16) as u16,
    ((116 as i32) as i16) as u16,
    ((32 as i32) as i16) as u16,
    ((111 as i32) as i16) as u16,
    ((102 as i32) as i16) as u16,
    ((32 as i32) as i16) as u16,
    ((109 as i32) as i16) as u16,
    ((101 as i32) as i16) as u16,
    ((109 as i32) as i16) as u16,
    ((111 as i32) as i16) as u16,
    ((114 as i32) as i16) as u16,
    ((121 as i32) as i16) as u16,
    ((0 as i32) as i16) as u16,
]);

static mut misuse: __SlateAlign16<[u16; 34]> = __SlateAlign16([
    ((98 as i32) as i16) as u16,
    ((97 as i32) as i16) as u16,
    ((100 as i32) as i16) as u16,
    ((32 as i32) as i16) as u16,
    ((112 as i32) as i16) as u16,
    ((97 as i32) as i16) as u16,
    ((114 as i32) as i16) as u16,
    ((97 as i32) as i16) as u16,
    ((109 as i32) as i16) as u16,
    ((101 as i32) as i16) as u16,
    ((116 as i32) as i16) as u16,
    ((101 as i32) as i16) as u16,
    ((114 as i32) as i16) as u16,
    ((32 as i32) as i16) as u16,
    ((111 as i32) as i16) as u16,
    ((114 as i32) as i16) as u16,
    ((32 as i32) as i16) as u16,
    ((111 as i32) as i16) as u16,
    ((116 as i32) as i16) as u16,
    ((104 as i32) as i16) as u16,
    ((101 as i32) as i16) as u16,
    ((114 as i32) as i16) as u16,
    ((32 as i32) as i16) as u16,
    ((65 as i32) as i16) as u16,
    ((80 as i32) as i16) as u16,
    ((73 as i32) as i16) as u16,
    ((32 as i32) as i16) as u16,
    ((109 as i32) as i16) as u16,
    ((105 as i32) as i16) as u16,
    ((115 as i32) as i16) as u16,
    ((117 as i32) as i16) as u16,
    ((115 as i32) as i16) as u16,
    ((101 as i32) as i16) as u16,
    ((0 as i32) as i16) as u16,
]);

/// Return the most recent error code generated by an SQLite routine. If NULL is
/// passed to this function, we assume a malloc() failed during sqlite3_open().
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.main.sqlite3_errcode")]
extern "C-unwind" fn sqlite3_errcode(mut db: *mut sqlite3) -> i32 {
    let mut iRet: i32 = 0 as i32;
    if !(db != std::ptr::null_mut::<sqlite3>()) {
        return 7 as i32;
    }
    if !((unsafe { sqlite3SafetyCheckSickOrOk(db) }) != (0 as i32)) {
        return sqlite3MisuseError(2855 as i32);
    }
    unsafe { sqlite3_mutex_enter(unsafe { (*db).mutex }) };
    if (unsafe { (*db).mallocFailed }) != (0 as u8) {
        iRet = 7 as i32;
    } else {
        iRet = (unsafe { (*db).errCode }) & unsafe { (*db).errMask };
    }
    unsafe { sqlite3_mutex_leave(unsafe { (*db).mutex }) };
    return iRet;
}

#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.main.sqlite3_extended_errcode")]
extern "C-unwind" fn sqlite3_extended_errcode(mut db: *mut sqlite3) -> i32 {
    let mut iRet: i32 = 0 as i32;
    if !(db != std::ptr::null_mut::<sqlite3>()) {
        return 7 as i32;
    }
    if !((unsafe { sqlite3SafetyCheckSickOrOk(db) }) != (0 as i32)) {
        return sqlite3MisuseError(2870 as i32);
    }
    unsafe { sqlite3_mutex_enter(unsafe { (*db).mutex }) };
    if (unsafe { (*db).mallocFailed }) != (0 as u8) {
        iRet = 7 as i32;
    } else {
        iRet = unsafe { (*db).errCode };
    }
    unsafe { sqlite3_mutex_leave(unsafe { (*db).mutex }) };
    return iRet;
}

#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.main.sqlite3_system_errno")]
extern "C-unwind" fn sqlite3_system_errno(mut db: *mut sqlite3) -> i32 {
    let mut iRet: i32 = 0 as i32;
    if db != std::ptr::null_mut::<sqlite3>() {
        unsafe { sqlite3_mutex_enter(unsafe { (*db).mutex }) };
        iRet = unsafe { (*db).iSysErrno };
        unsafe { sqlite3_mutex_leave(unsafe { (*db).mutex }) };
    }
    return iRet;
}

/// Return a string that describes the kind of error specified in the
/// argument.  For now, this simply calls the internal sqlite3ErrStr()
/// function.
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.main.sqlite3_errstr")]
extern "C-unwind" fn sqlite3_errstr(mut rc: i32) -> *const i8 {
    return sqlite3ErrStr(rc);
}

/// Create a new collating function for database "db".  The name is zName
/// and the encoding is enc.
fn createCollation(
    mut db: *mut sqlite3,
    mut zName: *const i8,
    mut enc: u8,
    mut pCtx: *mut (),
    mut xCompare: Option<
        unsafe extern "C-unwind" fn(*mut (), i32, *const (), i32, *const ()) -> i32,
    >,
    mut xDel: Option<unsafe extern "C-unwind" fn(*mut ())>,
) -> i32 {
    let mut pColl: *mut CollSeq = unsafe { std::mem::zeroed() };
    let mut enc2: i32 = 0 as i32;
    0 as i32;
    // If SQLITE_UTF16 is specified as the encoding type, transform this
    // to one of SQLITE_UTF16LE or SQLITE_UTF16BE using the
    // SQLITE_UTF16NATIVE macro. SQLITE_UTF16 is not used internally.
    enc2 = (enc as u32) as i32;
    {}
    {}
    if enc2 == (4 as i32) || enc2 == (8 as i32) {
        enc2 = 2 as i32;
    }
    if enc2 < (1 as i32) || enc2 > (3 as i32) {
        return sqlite3MisuseError(2928 as i32);
    }
    // Check if this call is removing or replacing an existing collation
    // sequence. If so, and there are active VMs, return busy. If there
    // are no active VMs, invalidate any pre-compiled statements.
    pColl = unsafe { sqlite3FindCollSeq(db, (enc2 as i8) as u8, zName, 0 as i32) };
    if pColl != std::ptr::null_mut::<CollSeq>() && (unsafe { (*pColl).xCmp }) != None {
        if (unsafe { (*db).nVdbeActive }) != (0 as i32) {
            unsafe {
                sqlite3ErrorWithMsg(
                    db,
                    5 as i32,
                    (b"unable to delete/modify collation sequence due to active statements\0"
                        .as_ptr() as *mut i8) as *const i8,
                )
            };
            return 5 as i32;
        }
        unsafe { sqlite3ExpirePreparedStatements(db, 0 as i32) };
        // If collation sequence pColl was created directly by a call to
        // sqlite3_create_collation, and not generated by synthCollSeq(),
        // then any copies made by synthCollSeq() need to be invalidated.
        // Also, collation destructor - CollSeq.xDel() - function may need
        // to be called.
        if (((unsafe { (*pColl).enc }) as u32) as i32) & !(8 as i32) == enc2 {
            let mut aColl: *mut CollSeq = (unsafe {
                sqlite3HashFind(
                    (unsafe { std::ptr::addr_of_mut!((*db).aCollSeq) }) as *const Hash,
                    zName,
                )
            }) as *mut CollSeq;
            let mut j: i32 = 0 as i32;
            j = 0 as i32;
            '__slate_break_1749: loop {
                if !(j < (3 as i32)) {
                    break;
                }
                let mut p: *mut CollSeq = unsafe { aColl.offset(j as isize) };
                if (((unsafe { (*p).enc }) as u32) as i32)
                    == (((unsafe { (*pColl).enc }) as u32) as i32)
                {
                    if (unsafe { (*p).xDel }) != None {
                        unsafe { unsafe { (*p).xDel }.unwrap()(unsafe { (*p).pUser }) };
                    }
                    unsafe {
                        (*p).xCmp = None;
                    }
                }
                let __v1965: i32 = j;
                let __v1966: i32 = __v1965 + (1 as i32);
                j = __v1966;
            }
        }
    }
    pColl = unsafe { sqlite3FindCollSeq(db, (enc2 as i8) as u8, zName, 1 as i32) };
    if pColl == std::ptr::null_mut::<CollSeq>() {
        return 7 as i32;
    }
    unsafe {
        (*pColl).xCmp = xCompare;
    }
    unsafe {
        (*pColl).pUser = pCtx;
    }
    unsafe {
        (*pColl).xDel = xDel;
    }
    unsafe {
        (*pColl).enc = ((enc2 | ((enc as u32) as i32) & (8 as i32)) as i8) as u8;
    }
    unsafe { sqlite3Error(db, 0 as i32) };
    return 0 as i32;
}

/// This array defines hard upper bounds on limit values.  The
/// initializer must be kept in sync with the SQLITE_LIMIT_*
/// #defines in sqlite3.h.
/// IMP: R-38091-32352
static mut aHardLimit: __SlateAlign16<[i32; 15]> = __SlateAlign16([
    1000000000 as i32,
    1000000000 as i32,
    2000 as i32,
    1000 as i32,
    500 as i32,
    250000000 as i32,
    1000 as i32,
    10 as i32,
    50000 as i32,
    32766 as i32,
    1000 as i32,
    8 as i32,
    2500 as i32,
    10000000 as i32,
    65000 as i32,
]);

// Make sure the hard limits are set to reasonable values
/// Change the value of a limit.  Report the old value.
/// If an invalid limit index is supplied, report -1.
/// Make no changes but still report the old value if the
/// new limit is negative.
///
/// A new lower limit does not shrink existing constructs.
/// It merely prevents new constructs that exceed the limit
/// from forming.
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.main.sqlite3_limit")]
extern "C-unwind" fn sqlite3_limit(
    mut db: *mut sqlite3,
    mut limitId: i32,
    mut newLimit: i32,
) -> i32 {
    let mut oldLimit: i32 = 0 as i32;
    // EVIDENCE-OF: R-30189-54097 For each limit category SQLITE_LIMIT_NAME
    // there is a hard upper bound set at compile-time by a C preprocessor
    // macro called SQLITE_MAX_NAME. (The "_LIMIT_" in the name is changed to
    // "_MAX_".)
    0 as i32;
    0 as i32;
    0 as i32;
    0 as i32;
    0 as i32;
    0 as i32;
    0 as i32;
    0 as i32;
    0 as i32;
    0 as i32;
    0 as i32;
    0 as i32;
    0 as i32;
    0 as i32;
    0 as i32;
    0 as i32;
    if limitId < (0 as i32) || limitId >= (14 as i32) + (1 as i32) {
        return -(1 as i32);
    }
    unsafe { sqlite3_mutex_enter(unsafe { (*db).mutex }) };
    oldLimit = unsafe {
        *unsafe { unsafe { (*db).aLimit.as_mut_ptr() as *mut i32 }.offset(limitId as isize) }
    };
    if newLimit >= (0 as i32) {
        // IMP: R-52476-28732
        if newLimit
            > unsafe {
                *unsafe {
                    unsafe { std::ptr::addr_of!(aHardLimit.0) as *const i32 }
                        .offset(limitId as isize)
                }
            }
        {
            newLimit = unsafe {
                *unsafe {
                    unsafe { std::ptr::addr_of!(aHardLimit.0) as *const i32 }
                        .offset(limitId as isize)
                }
            }; // IMP: R-51463-25634
        } else {
            if newLimit < (30 as i32) && limitId == (0 as i32) {
                newLimit = 30 as i32;
            }
        }
        unsafe {
            *unsafe { unsafe { (*db).aLimit.as_mut_ptr() as *mut i32 }.offset(limitId as isize) } =
                newLimit;
        }
    }
    unsafe { sqlite3_mutex_leave(unsafe { (*db).mutex }) };
    return oldLimit; // IMP: R-53341-35419
}

/// This function is used to parse both URIs and non-URI filenames passed by the
/// user to API functions sqlite3_open() or sqlite3_open_v2(), and for database
/// URIs specified as part of ATTACH statements.
///
/// The first argument to this function is the name of the VFS to use (or
/// a NULL to signify the default VFS) if the URI does not contain a "vfs=xxx"
/// query parameter. The second argument contains the URI (or non-URI filename)
/// itself. When this function is called the *pFlags variable should contain
/// the default flags to open the database handle with. The value stored in
/// *pFlags may be updated before returning if the URI filename contains
/// "cache=xxx" or "mode=xxx" query parameters.
///
/// If successful, SQLITE_OK is returned. In this case *ppVfs is set to point to
/// the VFS that should be used to open the database file. *pzFile is set to
/// point to a buffer containing the name of the file to open.  The value
/// stored in *pzFile is a database name acceptable to sqlite3_uri_parameter()
/// and is in the same format as names created using sqlite3_create_filename().
/// The caller must invoke sqlite3_free_filename() (not sqlite3_free()!) on
/// the value returned in *pzFile to avoid a memory leak.
///
/// If an error occurs, then an SQLite error code is returned and *pzErrMsg
/// may be set to point to a buffer containing an English language error
/// message. It is the responsibility of the caller to eventually release
/// this buffer by calling sqlite3_free().
///
/// # Arguments
///
/// * `zDefaultVfs` - VFS to use if no "vfs=xxx" query option
/// * `zUri` - Nul-terminated URI to parse
/// * `pFlags` - IN/OUT: SQLITE_OPEN_XXX flags
/// * `ppVfs` - OUT: VFS to use
/// * `pzFile` - OUT: Filename component of URI
/// * `pzErrMsg` - OUT: Error message (if rc!=SQLITE_OK)
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3ParseUri(
    mut zDefaultVfs: *const i8,
    mut zUri: *const i8,
    mut pFlags: *mut u32,
    mut ppVfs: *mut *mut sqlite3_vfs,
    mut pzFile: *mut *mut i8,
    mut pzErrMsg: *mut *mut i8,
) -> i32 {
    let mut __slate_storage_1913: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1913: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1913) as *mut i32;
    let mut __slate_storage_1912: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1912: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1912) as *mut i32;
    let mut __slate_storage_1064: std::mem::MaybeUninit<*const i8> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1064: *mut *const i8 =
        std::ptr::addr_of_mut!(__slate_storage_1064) as *mut *const i8;
    let mut __slate_storage_1063: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1063: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1063) as *mut i32;
    let mut __slate_storage_1062: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1062: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1062) as *mut i32;
    let mut __slate_storage_1059: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1059: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1059) as *mut i32;
    let mut __slate_storage_1058: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1058: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1058) as *mut i32;
    let mut __slate_storage_1057: std::mem::MaybeUninit<*mut i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1057: *mut *mut i8 =
        std::ptr::addr_of_mut!(__slate_storage_1057) as *mut *mut i8;
    let mut __slate_storage_1056: std::mem::MaybeUninit<*mut OpenMode> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1056: *mut *mut OpenMode =
        std::ptr::addr_of_mut!(__slate_storage_1056) as *mut *mut OpenMode;
    let mut __slate_storage_1054: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1054: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_1054) as *mut i64;
    let mut __slate_storage_1053: std::mem::MaybeUninit<*mut i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1053: *mut *mut i8 =
        std::ptr::addr_of_mut!(__slate_storage_1053) as *mut *mut i8;
    let mut __slate_storage_1052: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1052: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_1052) as *mut i64;
    let mut __slate_storage_1911: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1911: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_1911) as *mut i64;
    let mut __slate_storage_1910: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1910: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_1910) as *mut i64;
    let mut __slate_storage_1892: std::mem::MaybeUninit<i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1892: *mut i8 = std::ptr::addr_of_mut!(__slate_storage_1892) as *mut i8;
    let mut __slate_storage_1909: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1909: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_1909) as *mut i64;
    let mut __slate_storage_1908: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1908: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_1908) as *mut i64;
    let mut __slate_storage_1901: std::mem::MaybeUninit<i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1901: *mut i8 = std::ptr::addr_of_mut!(__slate_storage_1901) as *mut i8;
    let mut __slate_storage_1903: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1903: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_1903) as *mut i64;
    let mut __slate_storage_1902: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1902: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_1902) as *mut i64;
    let mut __slate_storage_1900: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1900: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1900) as *mut i32;
    let mut __slate_storage_1899: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1899: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_1899) as *mut i64;
    let mut __slate_storage_1898: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1898: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_1898) as *mut i64;
    let mut __slate_storage_1897: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1897: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1897) as *mut i32;
    let mut __slate_storage_1896: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1896: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_1896) as *mut i64;
    let mut __slate_storage_1895: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1895: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_1895) as *mut i64;
    let mut __slate_storage_1051: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1051: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1051) as *mut i32;
    let mut __slate_storage_1907: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1907: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_1907) as *mut i64;
    let mut __slate_storage_1906: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1906: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_1906) as *mut i64;
    let mut __slate_storage_1905: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1905: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_1905) as *mut i64;
    let mut __slate_storage_1904: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1904: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_1904) as *mut i64;
    let mut __slate_storage_1894: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1894: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_1894) as *mut i64;
    let mut __slate_storage_1893: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1893: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_1893) as *mut i64;
    let mut __slate_storage_1891: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1891: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_1891) as *mut i64;
    let mut __slate_storage_1890: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1890: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_1890) as *mut i64;
    let mut __slate_storage_1889: std::mem::MaybeUninit<*mut i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1889: *mut *mut i8 =
        std::ptr::addr_of_mut!(__slate_storage_1889) as *mut *mut i8;
    let mut __slate_storage_1888: std::mem::MaybeUninit<*mut i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1888: *mut *mut i8 =
        std::ptr::addr_of_mut!(__slate_storage_1888) as *mut *mut i8;
    let mut __slate_storage_1885: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1885: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_1885) as *mut i64;
    let mut __slate_storage_1884: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1884: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_1884) as *mut i64;
    let mut __slate_storage_1887: std::mem::MaybeUninit<u64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1887: *mut u64 = std::ptr::addr_of_mut!(__slate_storage_1887) as *mut u64;
    let mut __slate_storage_1886: std::mem::MaybeUninit<u64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1886: *mut u64 = std::ptr::addr_of_mut!(__slate_storage_1886) as *mut u64;
    let mut __slate_storage_1883: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1883: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_1883) as *mut u32;
    // Make sure the SQLITE_OPEN_URI flag is set to indicate to the VFS xOpen
    // method that there may be extra parameters following the file-name.
    let mut __slate_storage_1882: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1882: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_1882) as *mut u32; // Bytes of space to allocate
    let mut __slate_storage_1050: std::mem::MaybeUninit<u64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1050: *mut u64 = std::ptr::addr_of_mut!(__slate_storage_1050) as *mut u64; // Output character index
    let mut __slate_storage_1049: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1049: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_1049) as *mut i64; // Input character index
    let mut __slate_storage_1048: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1048: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_1048) as *mut i64; // Parser state when parsing URI
    let mut __slate_storage_1047: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1047: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1047) as *mut i32;
    let mut __slate_storage_1046: std::mem::MaybeUninit<*mut i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1046: *mut *mut i8 =
        std::ptr::addr_of_mut!(__slate_storage_1046) as *mut *mut i8;
    let mut __slate_storage_1917: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1917: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_1917) as *mut u32;
    let mut __slate_storage_1916: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1916: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_1916) as *mut u32;
    let mut __slate_storage_1915: std::mem::MaybeUninit<*mut i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1915: *mut *mut i8 =
        std::ptr::addr_of_mut!(__slate_storage_1915) as *mut *mut i8;
    let mut __slate_storage_1914: std::mem::MaybeUninit<*mut i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1914: *mut *mut i8 =
        std::ptr::addr_of_mut!(__slate_storage_1914) as *mut *mut i8;
    let mut __slate_storage_1045: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1045: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_1045) as *mut i64;
    let mut __slate_storage_1044: std::mem::MaybeUninit<i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1044: *mut i8 = std::ptr::addr_of_mut!(__slate_storage_1044) as *mut i8;
    let mut __slate_storage_1043: std::mem::MaybeUninit<*mut i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1043: *mut *mut i8 =
        std::ptr::addr_of_mut!(__slate_storage_1043) as *mut *mut i8;
    let mut __slate_storage_1042: std::mem::MaybeUninit<*const i8> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1042: *mut *const i8 =
        std::ptr::addr_of_mut!(__slate_storage_1042) as *mut *const i8;
    let mut __slate_storage_1041: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1041: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_1041) as *mut u32;
    let mut __slate_storage_1040: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1040: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1040) as *mut i32;
    unsafe {
        '__join_2: {
            '__join_4: {
                std::ptr::write(__slate_slot_1040, 0 as i32);
                std::ptr::write(__slate_slot_1041, unsafe { *pFlags });
                std::ptr::write(__slate_slot_1042, zDefaultVfs);
                std::ptr::write(__slate_slot_1045, (unsafe { strlen(zUri) }) as i64);
                0 as i32;
                if (*__slate_slot_1041 & ((64 as i32) as u32) != (0 as u32)
                    || (unsafe {
                        std::sync::atomic::AtomicU8::from_ptr(
                            (unsafe { std::ptr::addr_of_mut!(sqlite3Config.bOpenUri) }) as *mut u8,
                        )
                        .load(std::sync::atomic::Ordering::Relaxed)
                    }) != (0 as u8))
                    && *__slate_slot_1045 >= ((5 as i32) as i64)
                    && (unsafe {
                        memcmp(
                            zUri as *const (),
                            (b"file:\0".as_ptr() as *mut i8) as *const (),
                            ((5 as i32) as i64) as u64,
                        )
                    }) == (0 as i32)
                {
                    std::ptr::write(__slate_slot_1049, (0 as i32) as i64);
                    std::ptr::write(
                        __slate_slot_1050,
                        (*__slate_slot_1045 + ((8 as i32) as i64)) as u64,
                    );
                    std::ptr::write(__slate_slot_1882, *__slate_slot_1041);
                    std::ptr::write(__slate_slot_1883, *__slate_slot_1882 | ((64 as i32) as u32));
                    *__slate_slot_1041 = *__slate_slot_1883;
                    *__slate_slot_1048 = (0 as i32) as i64;
                    loop {
                        if *__slate_slot_1048 < *__slate_slot_1045 {
                            std::ptr::write(__slate_slot_1886, *__slate_slot_1050);
                            std::ptr::write(
                                __slate_slot_1887,
                                (*__slate_slot_1886).wrapping_add(
                                    (((((unsafe {
                                        *unsafe { zUri.offset(*__slate_slot_1048 as isize) }
                                    }) as i32)
                                        == (38 as i32))
                                        as i32) as i64) as u64,
                                ),
                            );
                            *__slate_slot_1050 = *__slate_slot_1887;
                            std::ptr::write(__slate_slot_1884, *__slate_slot_1048);
                            std::ptr::write(
                                __slate_slot_1885,
                                *__slate_slot_1884 + ((1 as i32) as i64),
                            );
                            *__slate_slot_1048 = *__slate_slot_1885;
                        } else {
                            break;
                        }
                    }
                    *__slate_slot_1043 =
                        (unsafe { sqlite3_malloc64(*__slate_slot_1050) }) as *mut i8;
                    if !(*__slate_slot_1043 != std::ptr::null_mut::<i8>()) {
                        return 7 as i32;
                    } else {
                        unsafe {
                            memset(
                                *__slate_slot_1043 as *mut (),
                                0 as i32,
                                ((4 as i32) as i64) as u64,
                            )
                        }; // 4-byte of 0x00 is the start of DB name marker
                        std::ptr::write(__slate_slot_1888, *__slate_slot_1043);
                        std::ptr::write(__slate_slot_1889, unsafe {
                            (*__slate_slot_1888).offset((4 as i32) as isize)
                        });
                        *__slate_slot_1043 = *__slate_slot_1889;
                        *__slate_slot_1048 = (5 as i32) as i64;
                        // Discard the scheme and authority segments of the URI.
                        if ((unsafe { *unsafe { zUri.offset((5 as i32) as isize) } }) as i32)
                            == (47 as i32)
                            && ((unsafe { *unsafe { zUri.offset((6 as i32) as isize) } }) as i32)
                                == (47 as i32)
                        {
                            *__slate_slot_1048 = (7 as i32) as i64;
                            loop {
                                if (unsafe { *unsafe { zUri.offset(*__slate_slot_1048 as isize) } })
                                    != (0 as i8)
                                    && ((unsafe {
                                        *unsafe { zUri.offset(*__slate_slot_1048 as isize) }
                                    }) as i32)
                                        != (47 as i32)
                                {
                                    std::ptr::write(__slate_slot_1890, *__slate_slot_1048);
                                    std::ptr::write(
                                        __slate_slot_1891,
                                        *__slate_slot_1890 + ((1 as i32) as i64),
                                    );
                                    *__slate_slot_1048 = *__slate_slot_1891;
                                } else {
                                    break;
                                }
                            }
                            if *__slate_slot_1048 != ((7 as i32) as i64)
                                && (*__slate_slot_1048 != ((16 as i32) as i64)
                                    || (unsafe {
                                        memcmp(
                                            (b"localhost\0".as_ptr() as *mut i8) as *const (),
                                            (unsafe { zUri.offset((7 as i32) as isize) })
                                                as *const (),
                                            ((9 as i32) as i64) as u64,
                                        )
                                    }) != (0 as i32))
                            {
                                unsafe {
                                    *pzErrMsg = unsafe {
                                        sqlite3_mprintf(
                                            (b"invalid uri authority: %.*s\0".as_ptr() as *mut i8)
                                                as *const i8,
                                            (*__slate_slot_1048 - ((7 as i32) as i64)) as i32,
                                            unsafe { zUri.offset((7 as i32) as isize) },
                                        )
                                    };
                                }
                                *__slate_slot_1040 = 1 as i32;
                                break '__join_2;
                            }
                        }
                        // Copy the filename and any query parameters into the zFile buffer.
                        // Decode %HH escape codes along the way.
                        //
                        // Within this loop, variable eState may be set to 0, 1 or 2, depending
                        // on the parsing context. As follows:
                        //
                        //   0: Parsing file-name.
                        //   1: Parsing name section of a name=value query parameter.
                        //   2: Parsing value section of a name=value query parameter.
                        *__slate_slot_1047 = 0 as i32;
                        '__loop_27: loop {
                            std::ptr::write(__slate_slot_1892, unsafe {
                                *unsafe { zUri.offset(*__slate_slot_1048 as isize) }
                            });
                            *__slate_slot_1044 = *__slate_slot_1892;
                            if (*__slate_slot_1892 as i32) != (0 as i32)
                                && (*__slate_slot_1044 as i32) != (35 as i32)
                            {
                                std::ptr::write(__slate_slot_1893, *__slate_slot_1048);
                                std::ptr::write(
                                    __slate_slot_1894,
                                    *__slate_slot_1893 + ((1 as i32) as i64),
                                );
                                *__slate_slot_1048 = *__slate_slot_1894;
                                if (*__slate_slot_1044 as i32) == (37 as i32)
                                    && (((unsafe {
                                        *unsafe {
                                            unsafe {
                                                std::ptr::addr_of!(sqlite3CtypeMap) as *const u8
                                            }
                                            .offset(
                                                ((((unsafe {
                                                    *unsafe {
                                                        zUri.offset(*__slate_slot_1048 as isize)
                                                    }
                                                })
                                                    as u8)
                                                    as u32)
                                                    as i32)
                                                    as isize,
                                            )
                                        }
                                    }) as u32) as i32)
                                        & (8 as i32)
                                        != (0 as i32)
                                    && (((unsafe {
                                        *unsafe {
                                            unsafe {
                                                std::ptr::addr_of!(sqlite3CtypeMap) as *const u8
                                            }
                                            .offset(
                                                ((((unsafe {
                                                    *unsafe {
                                                        zUri.offset(
                                                            (*__slate_slot_1048
                                                                + ((1 as i32) as i64))
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
                                        & (8 as i32)
                                        != (0 as i32)
                                {
                                    std::ptr::write(__slate_slot_1895, *__slate_slot_1048);
                                    std::ptr::write(
                                        __slate_slot_1896,
                                        *__slate_slot_1895 + ((1 as i32) as i64),
                                    );
                                    *__slate_slot_1048 = *__slate_slot_1896;
                                    *__slate_slot_1051 = (((unsafe {
                                        sqlite3HexToInt(
                                            (unsafe {
                                                *unsafe { zUri.offset(*__slate_slot_1895 as isize) }
                                            }) as i32,
                                        )
                                    })
                                        as u32)
                                        as i32)
                                        << (4 as i32);
                                    std::ptr::write(__slate_slot_1897, *__slate_slot_1051);
                                    std::ptr::write(__slate_slot_1898, *__slate_slot_1048);
                                    std::ptr::write(
                                        __slate_slot_1899,
                                        *__slate_slot_1898 + ((1 as i32) as i64),
                                    );
                                    *__slate_slot_1048 = *__slate_slot_1899;
                                    std::ptr::write(
                                        __slate_slot_1900,
                                        *__slate_slot_1897
                                            + (((unsafe {
                                                sqlite3HexToInt(
                                                    (unsafe {
                                                        *unsafe {
                                                            zUri.offset(*__slate_slot_1898 as isize)
                                                        }
                                                    })
                                                        as i32,
                                                )
                                            })
                                                as u32)
                                                as i32),
                                    );
                                    *__slate_slot_1051 = *__slate_slot_1900;
                                    0 as i32;
                                    if *__slate_slot_1051 == (0 as i32) {
                                        // This branch is taken when "%00" appears within the URI. In this
                                        // case we ignore all text in the remainder of the path, name or
                                        // value currently being parsed. So ignore the current character
                                        // and skip to the next "?", "=" or "&", as appropriate.
                                        loop {
                                            std::ptr::write(__slate_slot_1901, unsafe {
                                                *unsafe { zUri.offset(*__slate_slot_1048 as isize) }
                                            });
                                            *__slate_slot_1044 = *__slate_slot_1901;
                                            if (*__slate_slot_1901 as i32) != (0 as i32)
                                                && (*__slate_slot_1044 as i32) != (35 as i32)
                                                && (*__slate_slot_1047 != (0 as i32)
                                                    || (*__slate_slot_1044 as i32) != (63 as i32))
                                                && (*__slate_slot_1047 != (1 as i32)
                                                    || (*__slate_slot_1044 as i32) != (61 as i32)
                                                        && (*__slate_slot_1044 as i32)
                                                            != (38 as i32))
                                                && (*__slate_slot_1047 != (2 as i32)
                                                    || (*__slate_slot_1044 as i32) != (38 as i32))
                                            {
                                                std::ptr::write(
                                                    __slate_slot_1902,
                                                    *__slate_slot_1048,
                                                );
                                                std::ptr::write(
                                                    __slate_slot_1903,
                                                    *__slate_slot_1902 + ((1 as i32) as i64),
                                                );
                                                *__slate_slot_1048 = *__slate_slot_1903;
                                            } else {
                                                continue '__loop_27;
                                            }
                                        }
                                    } else {
                                        *__slate_slot_1044 = *__slate_slot_1051 as i8;
                                    }
                                } else {
                                    if *__slate_slot_1047 == (1 as i32)
                                        && ((*__slate_slot_1044 as i32) == (38 as i32)
                                            || (*__slate_slot_1044 as i32) == (61 as i32))
                                    {
                                        if ((unsafe {
                                            *unsafe {
                                                (*__slate_slot_1043).offset(
                                                    (*__slate_slot_1049 - ((1 as i32) as i64))
                                                        as isize,
                                                )
                                            }
                                        }) as i32)
                                            == (0 as i32)
                                        {
                                            // An empty option name. Ignore this option altogether.
                                            loop {
                                                if (unsafe {
                                                    *unsafe {
                                                        zUri.offset(*__slate_slot_1048 as isize)
                                                    }
                                                }) != (0 as i8)
                                                    && ((unsafe {
                                                        *unsafe {
                                                            zUri.offset(*__slate_slot_1048 as isize)
                                                        }
                                                    })
                                                        as i32)
                                                        != (35 as i32)
                                                    && ((unsafe {
                                                        *unsafe {
                                                            zUri.offset(
                                                                (*__slate_slot_1048
                                                                    - ((1 as i32) as i64))
                                                                    as isize,
                                                            )
                                                        }
                                                    })
                                                        as i32)
                                                        != (38 as i32)
                                                {
                                                    std::ptr::write(
                                                        __slate_slot_1904,
                                                        *__slate_slot_1048,
                                                    );
                                                    std::ptr::write(
                                                        __slate_slot_1905,
                                                        *__slate_slot_1904 + ((1 as i32) as i64),
                                                    );
                                                    *__slate_slot_1048 = *__slate_slot_1905;
                                                } else {
                                                    continue '__loop_27;
                                                }
                                            }
                                        } else {
                                            if (*__slate_slot_1044 as i32) == (38 as i32) {
                                                std::ptr::write(
                                                    __slate_slot_1906,
                                                    *__slate_slot_1049,
                                                );
                                                std::ptr::write(
                                                    __slate_slot_1907,
                                                    *__slate_slot_1906 + ((1 as i32) as i64),
                                                );
                                                *__slate_slot_1049 = *__slate_slot_1907;
                                                unsafe {
                                                    *unsafe {
                                                        (*__slate_slot_1043)
                                                            .offset(*__slate_slot_1906 as isize)
                                                    } = (0 as i32) as i8;
                                                }
                                            } else {
                                                *__slate_slot_1047 = 2 as i32;
                                            }
                                            *__slate_slot_1044 = (0 as i32) as i8;
                                        }
                                    } else {
                                        if *__slate_slot_1047 == (0 as i32)
                                            && (*__slate_slot_1044 as i32) == (63 as i32)
                                            || *__slate_slot_1047 == (2 as i32)
                                                && (*__slate_slot_1044 as i32) == (38 as i32)
                                        {
                                            *__slate_slot_1044 = (0 as i32) as i8;
                                            *__slate_slot_1047 = 1 as i32;
                                        }
                                    }
                                }
                                std::ptr::write(__slate_slot_1908, *__slate_slot_1049);
                                std::ptr::write(
                                    __slate_slot_1909,
                                    *__slate_slot_1908 + ((1 as i32) as i64),
                                );
                                *__slate_slot_1049 = *__slate_slot_1909;
                                unsafe {
                                    *unsafe {
                                        (*__slate_slot_1043).offset(*__slate_slot_1908 as isize)
                                    } = *__slate_slot_1044;
                                }
                            } else {
                                break;
                            }
                        }
                        if *__slate_slot_1047 == (1 as i32) {
                            std::ptr::write(__slate_slot_1910, *__slate_slot_1049);
                            std::ptr::write(
                                __slate_slot_1911,
                                *__slate_slot_1910 + ((1 as i32) as i64),
                            );
                            *__slate_slot_1049 = *__slate_slot_1911;
                            unsafe {
                                *unsafe {
                                    (*__slate_slot_1043).offset(*__slate_slot_1910 as isize)
                                } = (0 as i32) as i8;
                            }
                        }
                        unsafe {
                            memset(
                                (unsafe {
                                    (*__slate_slot_1043).offset(*__slate_slot_1049 as isize)
                                }) as *mut (),
                                0 as i32,
                                ((4 as i32) as i64) as u64,
                            )
                        }; // end-of-options + empty journal filenames
                        // Check if there were any options specified that should be interpreted
                        // here. Options that are interpreted here include "vfs" and those that
                        // correspond to flags that may be passed to the sqlite3_open_v2()
                        // method.
                        *__slate_slot_1046 = unsafe {
                            (*__slate_slot_1043).offset(
                                unsafe { strlen(*__slate_slot_1043 as *const i8) }
                                    .wrapping_add(((1 as i32) as i64) as u64)
                                    as isize,
                            )
                        };
                        '__join_11: {
                            '__loop_5: loop {
                                if (unsafe {
                                    *unsafe { (*__slate_slot_1046).offset((0 as i32) as isize) }
                                }) != (0 as i8)
                                {
                                    std::ptr::write(
                                        __slate_slot_1052,
                                        (unsafe { strlen(*__slate_slot_1046 as *const i8) }) as i64,
                                    );
                                    std::ptr::write(__slate_slot_1053, unsafe {
                                        (*__slate_slot_1046).offset(
                                            (*__slate_slot_1052 + ((1 as i32) as i64)) as isize,
                                        )
                                    });
                                    std::ptr::write(
                                        __slate_slot_1054,
                                        (unsafe { strlen(*__slate_slot_1053 as *const i8) }) as i64,
                                    );
                                    if *__slate_slot_1052 == ((3 as i32) as i64)
                                        && (unsafe {
                                            memcmp(
                                                (b"vfs\0".as_ptr() as *mut i8) as *const (),
                                                *__slate_slot_1046 as *const (),
                                                ((3 as i32) as i64) as u64,
                                            )
                                        }) == (0 as i32)
                                    {
                                        *__slate_slot_1042 = *__slate_slot_1053 as *const i8;
                                    } else {
                                        std::ptr::write(
                                            __slate_slot_1056,
                                            std::ptr::null_mut::<OpenMode>(),
                                        );
                                        std::ptr::write(
                                            __slate_slot_1057,
                                            std::ptr::null_mut::<i8>(),
                                        );
                                        std::ptr::write(__slate_slot_1058, 0 as i32);
                                        std::ptr::write(__slate_slot_1059, 0 as i32);
                                        if *__slate_slot_1052 == ((5 as i32) as i64)
                                            && (unsafe {
                                                memcmp(
                                                    (b"cache\0".as_ptr() as *mut i8) as *const (),
                                                    *__slate_slot_1046 as *const (),
                                                    ((5 as i32) as i64) as u64,
                                                )
                                            }) == (0 as i32)
                                        {
                                            *__slate_slot_1058 = (131072 as i32) | (262144 as i32);
                                            *__slate_slot_1056 = unsafe {
                                                std::ptr::addr_of_mut!(aCacheMode.0)
                                                    as *mut OpenMode
                                            };
                                            *__slate_slot_1059 = *__slate_slot_1058;
                                            *__slate_slot_1057 = b"cache\0".as_ptr() as *mut i8;
                                        }
                                        if *__slate_slot_1052 == ((4 as i32) as i64)
                                            && (unsafe {
                                                memcmp(
                                                    (b"mode\0".as_ptr() as *mut i8) as *const (),
                                                    *__slate_slot_1046 as *const (),
                                                    ((4 as i32) as i64) as u64,
                                                )
                                            }) == (0 as i32)
                                        {
                                            *__slate_slot_1058 =
                                                (1 as i32) | (2 as i32) | (4 as i32) | (128 as i32);
                                            *__slate_slot_1056 = unsafe {
                                                std::ptr::addr_of_mut!(aOpenMode.0) as *mut OpenMode
                                            };
                                            *__slate_slot_1059 = ((*__slate_slot_1058 as u32)
                                                & *__slate_slot_1041)
                                                as i32;
                                            *__slate_slot_1057 = b"access\0".as_ptr() as *mut i8;
                                        }
                                        if *__slate_slot_1056 != std::ptr::null_mut::<OpenMode>() {
                                            std::ptr::write(__slate_slot_1063, 0 as i32);
                                            *__slate_slot_1062 = 0 as i32;
                                            '__join_12: {
                                                loop {
                                                    if (unsafe {
                                                        (*unsafe {
                                                            (*__slate_slot_1056)
                                                                .offset(*__slate_slot_1062 as isize)
                                                        })
                                                        .z
                                                    }) != std::ptr::null::<i8>()
                                                    {
                                                        std::ptr::write(
                                                            __slate_slot_1064,
                                                            unsafe {
                                                                (*unsafe {
                                                                    (*__slate_slot_1056).offset(
                                                                        *__slate_slot_1062 as isize,
                                                                    )
                                                                })
                                                                .z
                                                            },
                                                        );
                                                        if *__slate_slot_1054
                                                            == ((unsafe {
                                                                strlen(*__slate_slot_1064)
                                                            })
                                                                as i64)
                                                            && (0 as i32)
                                                                == unsafe {
                                                                    memcmp(
                                                                        *__slate_slot_1053
                                                                            as *const (),
                                                                        *__slate_slot_1064
                                                                            as *const (),
                                                                        *__slate_slot_1054 as u64,
                                                                    )
                                                                }
                                                        {
                                                            break;
                                                        } else {
                                                            std::ptr::write(
                                                                __slate_slot_1912,
                                                                *__slate_slot_1062,
                                                            );
                                                            std::ptr::write(
                                                                __slate_slot_1913,
                                                                *__slate_slot_1912 + (1 as i32),
                                                            );
                                                            *__slate_slot_1062 = *__slate_slot_1913;
                                                        }
                                                    } else {
                                                        break '__join_12;
                                                    }
                                                }
                                                *__slate_slot_1063 = unsafe {
                                                    (*unsafe {
                                                        (*__slate_slot_1056)
                                                            .offset(*__slate_slot_1062 as isize)
                                                    })
                                                    .mode
                                                };
                                            }
                                            if *__slate_slot_1063 == (0 as i32) {
                                                break '__join_11;
                                            } else {
                                                if *__slate_slot_1063 & !(128 as i32)
                                                    > *__slate_slot_1059
                                                {
                                                    break '__loop_5;
                                                } else {
                                                    *__slate_slot_1041 = *__slate_slot_1041
                                                        & (!(*__slate_slot_1058) as u32)
                                                        | (*__slate_slot_1063 as u32);
                                                }
                                            }
                                        }
                                    }
                                    *__slate_slot_1046 = unsafe {
                                        (*__slate_slot_1053).offset(
                                            (*__slate_slot_1054 + ((1 as i32) as i64)) as isize,
                                        )
                                    };
                                } else {
                                    break '__join_4;
                                }
                            }
                            unsafe {
                                *pzErrMsg = unsafe {
                                    sqlite3_mprintf(
                                        (b"%s mode not allowed: %s\0".as_ptr() as *mut i8)
                                            as *const i8,
                                        *__slate_slot_1057,
                                        *__slate_slot_1053,
                                    )
                                };
                            }
                            *__slate_slot_1040 = 3 as i32;
                            break '__join_2;
                        }
                        unsafe {
                            *pzErrMsg = unsafe {
                                sqlite3_mprintf(
                                    (b"no such %s mode: %s\0".as_ptr() as *mut i8) as *const i8,
                                    *__slate_slot_1057,
                                    *__slate_slot_1053,
                                )
                            };
                        }
                        *__slate_slot_1040 = 1 as i32;
                        break '__join_2;
                    }
                } else {
                    *__slate_slot_1043 = (unsafe {
                        sqlite3_malloc64((*__slate_slot_1045 + ((8 as i32) as i64)) as u64)
                    }) as *mut i8;
                    if !(*__slate_slot_1043 != std::ptr::null_mut::<i8>()) {
                        return 7 as i32;
                    } else {
                        unsafe {
                            memset(
                                *__slate_slot_1043 as *mut (),
                                0 as i32,
                                ((4 as i32) as i64) as u64,
                            )
                        };
                        std::ptr::write(__slate_slot_1914, *__slate_slot_1043);
                        std::ptr::write(__slate_slot_1915, unsafe {
                            (*__slate_slot_1914).offset((4 as i32) as isize)
                        });
                        *__slate_slot_1043 = *__slate_slot_1915;
                        if *__slate_slot_1045 != (0 as i64) {
                            unsafe {
                                memcpy(
                                    *__slate_slot_1043 as *mut (),
                                    zUri as *const (),
                                    *__slate_slot_1045 as u64,
                                )
                            };
                        }
                        unsafe {
                            memset(
                                (unsafe {
                                    (*__slate_slot_1043).offset(*__slate_slot_1045 as isize)
                                }) as *mut (),
                                0 as i32,
                                ((4 as i32) as i64) as u64,
                            )
                        };
                        std::ptr::write(__slate_slot_1916, *__slate_slot_1041);
                        std::ptr::write(
                            __slate_slot_1917,
                            *__slate_slot_1916 & (!(64 as i32) as u32),
                        );
                        *__slate_slot_1041 = *__slate_slot_1917;
                    }
                }
            }
            // IMP: R-48725-32206
            // IMP: R-51689-46548
            // IMP: R-57884-37496
            unsafe {
                *ppVfs = unsafe { sqlite3_vfs_find(*__slate_slot_1042) };
            }
            if (unsafe { *ppVfs }) == std::ptr::null_mut::<sqlite3_vfs>() {
                unsafe {
                    *pzErrMsg = unsafe {
                        sqlite3_mprintf(
                            (b"no such vfs: %s\0".as_ptr() as *mut i8) as *const i8,
                            *__slate_slot_1042,
                        )
                    };
                }
                *__slate_slot_1040 = 1 as i32;
            }
        }
        if *__slate_slot_1040 != (0 as i32) {
            sqlite3_free_filename(*__slate_slot_1043 as *const i8);
            *__slate_slot_1043 = std::ptr::null_mut::<i8>();
        }
        unsafe {
            *pFlags = *__slate_slot_1041;
        }
        unsafe {
            *pzFile = *__slate_slot_1043;
        }
        return *__slate_slot_1040;
    }
    return unsafe { std::mem::zeroed() };
}

static mut aCacheMode: __SlateAlign16<[OpenMode; 3]> = __SlateAlign16([
    OpenMode {
        z: (b"shared\0".as_ptr() as *mut i8) as *const i8,
        mode: 131072 as i32,
    },
    OpenMode {
        z: (b"private\0".as_ptr() as *mut i8) as *const i8,
        mode: 262144 as i32,
    },
    OpenMode {
        z: std::ptr::null::<i8>(),
        mode: 0 as i32,
    },
]);

static mut aOpenMode: __SlateAlign16<[OpenMode; 5]> = __SlateAlign16([
    OpenMode {
        z: (b"ro\0".as_ptr() as *mut i8) as *const i8,
        mode: 1 as i32,
    },
    OpenMode {
        z: (b"rw\0".as_ptr() as *mut i8) as *const i8,
        mode: 2 as i32,
    },
    OpenMode {
        z: (b"rwc\0".as_ptr() as *mut i8) as *const i8,
        mode: (2 as i32) | (4 as i32),
    },
    OpenMode {
        z: (b"memory\0".as_ptr() as *mut i8) as *const i8,
        mode: 128 as i32,
    },
    OpenMode {
        z: std::ptr::null::<i8>(),
        mode: 0 as i32,
    },
]);

/// This routine does the core work of extracting URI parameters from a
/// database filename for the sqlite3_uri_parameter() interface.
fn uriParameter(mut zFilename: *const i8, mut zParam: *const i8) -> *const i8 {
    let __v1967: *const i8 = zFilename;
    let __v1968: *const i8 =
        unsafe { __v1967.offset(((unsafe { sqlite3Strlen30(zFilename) }) + (1 as i32)) as isize) };
    zFilename = __v1968;
    '__slate_break_1774: while zFilename != std::ptr::null::<i8>()
        && (unsafe { *unsafe { zFilename.offset((0 as i32) as isize) } }) != (0 as i8)
    {
        let mut x: i32 = unsafe { strcmp(zFilename, zParam) };
        let __v1969: *const i8 = zFilename;
        let __v1970: *const i8 = unsafe {
            __v1969.offset(((unsafe { sqlite3Strlen30(zFilename) }) + (1 as i32)) as isize)
        };
        zFilename = __v1970;
        if x == (0 as i32) {
            return zFilename;
        }
        let __v1971: *const i8 = zFilename;
        let __v1972: *const i8 = unsafe {
            __v1971.offset(((unsafe { sqlite3Strlen30(zFilename) }) + (1 as i32)) as isize)
        };
        zFilename = __v1972;
    }
    return std::ptr::null::<i8>();
}

/// This routine does the work of opening a database on behalf of
/// sqlite3_open() and sqlite3_open16(). The database filename "zFilename"
/// is UTF-8 encoded.
///
/// # Arguments
///
/// * `zFilename` - Database filename UTF-8 encoded
/// * `ppDb` - OUT: Returned database handle
/// * `flags` - Operational flags
/// * `zVfs` - Name of the VFS to use
fn openDatabase(
    mut zFilename: *const i8,
    mut ppDb: *mut *mut sqlite3,
    mut flags: u32,
    mut zVfs: *const i8,
) -> i32 {
    let mut __slate_storage_1983: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1983: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1983) as *mut i32;
    let mut __slate_storage_1982: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1982: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1982) as *mut i32;
    let mut __slate_storage_1981: std::mem::MaybeUninit<u64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1981: *mut u64 = std::ptr::addr_of_mut!(__slate_storage_1981) as *mut u64;
    let mut __slate_storage_1980: std::mem::MaybeUninit<u64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1980: *mut u64 = std::ptr::addr_of_mut!(__slate_storage_1980) as *mut u64;
    let mut __slate_storage_1979: std::mem::MaybeUninit<*mut sqlite3> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1979: *mut *mut sqlite3 =
        std::ptr::addr_of_mut!(__slate_storage_1979) as *mut *mut sqlite3;
    let mut __slate_storage_1978: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1978: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_1978) as *mut u32;
    // Remove harmful bits from the flags parameter
    //
    // The SQLITE_OPEN_NOMUTEX and SQLITE_OPEN_FULLMUTEX flags were
    // dealt with in the previous code block.  Besides these, the only
    // valid input flags for sqlite3_open_v2() are SQLITE_OPEN_READONLY,
    // SQLITE_OPEN_READWRITE, SQLITE_OPEN_CREATE, SQLITE_OPEN_SHAREDCACHE,
    // SQLITE_OPEN_PRIVATECACHE, SQLITE_OPEN_EXRESCODE, and some reserved
    // bits.  Silently mask off all other flags.
    let mut __slate_storage_1977: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1977: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_1977) as *mut u32;
    let mut __slate_storage_1974: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1974: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_1974) as *mut u32;
    let mut __slate_storage_1973: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1973: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_1973) as *mut u32;
    let mut __slate_storage_1976: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1976: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_1976) as *mut u32;
    let mut __slate_storage_1975: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1975: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_1975) as *mut u32; // Loop counter
    let mut __slate_storage_1080: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1080: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1080) as *mut i32; // Error message from sqlite3ParseUri()
    let mut __slate_storage_1079: std::mem::MaybeUninit<*mut i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1079: *mut *mut i8 =
        std::ptr::addr_of_mut!(__slate_storage_1079) as *mut *mut i8; // Filename argument to pass to BtreeOpen()
    let mut __slate_storage_1078: std::mem::MaybeUninit<*mut i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1078: *mut *mut i8 =
        std::ptr::addr_of_mut!(__slate_storage_1078) as *mut *mut i8; // True for threadsafe connections
    let mut __slate_storage_1077: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1077: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1077) as *mut i32; // Return code
    let mut __slate_storage_1076: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1076: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1076) as *mut i32; // Store allocated handle here
    let mut __slate_storage_1075: std::mem::MaybeUninit<*mut sqlite3> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1075: *mut *mut sqlite3 =
        std::ptr::addr_of_mut!(__slate_storage_1075) as *mut *mut sqlite3;
    unsafe {
        std::ptr::write(__slate_slot_1078, std::ptr::null_mut::<i8>());
        std::ptr::write(__slate_slot_1079, std::ptr::null_mut::<i8>());
        unsafe {
            *ppDb = std::ptr::null_mut::<sqlite3>();
        }
        *__slate_slot_1076 = sqlite3_initialize();
        if *__slate_slot_1076 != (0 as i32) {
            return *__slate_slot_1076;
        } else {
            if (((unsafe { sqlite3Config.bCoreMutex }) as u32) as i32) == (0 as i32) {
                *__slate_slot_1077 = 0 as i32;
            } else {
                if flags & ((32768 as i32) as u32) != (0 as u32) {
                    *__slate_slot_1077 = 0 as i32;
                } else {
                    if flags & ((65536 as i32) as u32) != (0 as u32) {
                        *__slate_slot_1077 = 1 as i32;
                    } else {
                        *__slate_slot_1077 = ((unsafe { sqlite3Config.bFullMutex }) as u32) as i32;
                    }
                }
            }
            if flags & ((262144 as i32) as u32) != (0 as u32) {
                std::ptr::write(__slate_slot_1973, flags);
                std::ptr::write(
                    __slate_slot_1974,
                    *__slate_slot_1973 & (!(131072 as i32) as u32),
                );
                flags = *__slate_slot_1974;
            } else {
                if (unsafe { sqlite3Config.sharedCacheEnabled }) != (0 as i32) {
                    std::ptr::write(__slate_slot_1975, flags);
                    std::ptr::write(
                        __slate_slot_1976,
                        *__slate_slot_1975 | ((131072 as i32) as u32),
                    );
                    flags = *__slate_slot_1976;
                }
            }
            '__join_6: {
                std::ptr::write(__slate_slot_1977, flags);
                std::ptr::write(
                    __slate_slot_1978,
                    *__slate_slot_1977
                        & (!((8 as i32)
                            | (16 as i32)
                            | (256 as i32)
                            | (512 as i32)
                            | (1024 as i32)
                            | (2048 as i32)
                            | (4096 as i32)
                            | (8192 as i32)
                            | (16384 as i32)
                            | (32768 as i32)
                            | (65536 as i32)
                            | (524288 as i32)) as u32),
                );
                flags = *__slate_slot_1978;
                // Allocate the sqlite data structure
                *__slate_slot_1075 = (unsafe { sqlite3MallocZero(800 as u64) }) as *mut sqlite3;
                if *__slate_slot_1075 == std::ptr::null_mut::<sqlite3>() {
                } else {
                    if *__slate_slot_1077 != (0 as i32) {
                        unsafe {
                            (*(*__slate_slot_1075)).mutex = unsafe { sqlite3MutexAlloc(1 as i32) };
                        }
                        if (unsafe { (*(*__slate_slot_1075)).mutex })
                            == std::ptr::null_mut::<sqlite3_mutex>()
                        {
                            unsafe { sqlite3_free(*__slate_slot_1075 as *mut ()) };
                            *__slate_slot_1075 = std::ptr::null_mut::<sqlite3>();
                            break '__join_6;
                        } else {
                            if *__slate_slot_1077 == (0 as i32) {
                                {}
                            }
                        }
                    }
                    unsafe { sqlite3_mutex_enter(unsafe { (*(*__slate_slot_1075)).mutex }) };
                    unsafe {
                        (*(*__slate_slot_1075)).errMask =
                            (if flags & ((33554432 as i32) as u32) != ((0 as i32) as u32) {
                                4294967295 as u32
                            } else {
                                (255 as i32) as u32
                            }) as i32;
                    }
                    unsafe {
                        (*(*__slate_slot_1075)).nDb = 2 as i32;
                    }
                    unsafe {
                        (*(*__slate_slot_1075)).eOpenState = ((109 as i32) as i8) as u8;
                    }
                    unsafe {
                        (*(*__slate_slot_1075)).aDb =
                            unsafe { (*(*__slate_slot_1075)).aDbStatic.as_mut_ptr() as *mut Db };
                    }
                    unsafe {
                        (*(*__slate_slot_1075)).lookaside.bDisable = (1 as i32) as u32;
                    }
                    unsafe {
                        (*(*__slate_slot_1075)).lookaside.sz = ((0 as i32) as i16) as u16;
                    }
                    unsafe {
                        (*(*__slate_slot_1075)).nFpDigit = ((17 as i32) as i8) as u8;
                    }
                    0 as i32;
                    unsafe {
                        memcpy(
                            (unsafe { (*(*__slate_slot_1075)).aLimit.as_mut_ptr() as *mut i32 })
                                as *mut (),
                            (unsafe { std::ptr::addr_of!(aHardLimit.0) as *const i32 })
                                as *const (),
                            60 as u64,
                        )
                    };
                    unsafe {
                        *unsafe {
                            unsafe { (*(*__slate_slot_1075)).aLimit.as_mut_ptr() as *mut i32 }
                                .offset((11 as i32) as isize)
                        } = 0 as i32;
                    }
                    unsafe {
                        (*(*__slate_slot_1075)).autoCommit = ((1 as i32) as i8) as u8;
                    }
                    unsafe {
                        (*(*__slate_slot_1075)).nextAutovac = -(1 as i32) as i8;
                    }
                    unsafe {
                        (*(*__slate_slot_1075)).szMmap = unsafe { sqlite3Config.szMmap };
                    }
                    unsafe {
                        (*(*__slate_slot_1075)).nextPagesize = 0 as i32;
                    }
                    unsafe {
                        (*(*__slate_slot_1075)).init.azInit =
                            unsafe { std::ptr::addr_of_mut!(sqlite3StdType) as *mut *const i8 };
                    }
                    // Any array of string ptrs will do
                    std::ptr::write(__slate_slot_1979, *__slate_slot_1075);
                    std::ptr::write(__slate_slot_1980, unsafe { (*(*__slate_slot_1979)).flags });
                    std::ptr::write(
                        __slate_slot_1981,
                        *__slate_slot_1980
                            | ((((((64 as i32) | (262144 as i32)) as u32)
                                | (2147483648 as u32)
                                | ((32 as i32) as u32)) as u64)
                                | (((16 as i32) as i64) as u64) << (32 as i32)
                                | (((32 as i32) as i64) as u64) << (32 as i32)
                                | (((64 as i32) as i64) as u64) << (32 as i32)
                                | (((128 as i32) as i64) as u64)
                                | (((32768 as i32) as i64) as u64)),
                    );
                    unsafe {
                        (*(*__slate_slot_1979)).flags = *__slate_slot_1981;
                    }
                    // The SQLITE_DQS compile-time option determines the default settings
                    // for SQLITE_DBCONFIG_DQS_DDL and SQLITE_DBCONFIG_DQS_DML.
                    //
                    //    SQLITE_DQS     SQLITE_DBCONFIG_DQS_DDL    SQLITE_DBCONFIG_DQS_DML
                    //    ----------     -----------------------    -----------------------
                    //     undefined               on                          on
                    //         3                   on                          on
                    //         2                   on                         off
                    //         1                  off                          on
                    //         0                  off                         off
                    //
                    // Legacy behavior is 3 (double-quoted string literals are allowed anywhere)
                    // and so that is the default.  But developers are encouraged to use
                    // -DSQLITE_DQS=0 (best) or -DSQLITE_DQS=1 (second choice) if possible.
                    unsafe {
                        sqlite3HashInit(unsafe {
                            std::ptr::addr_of_mut!((*(*__slate_slot_1075)).aCollSeq)
                        })
                    };
                    unsafe {
                        sqlite3HashInit(unsafe {
                            std::ptr::addr_of_mut!((*(*__slate_slot_1075)).aModule)
                        })
                    };
                    // Add the default collation sequence BINARY. BINARY works for both UTF-8
                    // and UTF-16, so add a version for each to avoid any unnecessary
                    // conversions. The only error that can occur here is a malloc() failure.
                    //
                    // EVIDENCE-OF: R-52786-44878 SQLite defines three built-in collating
                    // functions:
                    createCollation(
                        *__slate_slot_1075,
                        unsafe { std::ptr::addr_of!(sqlite3StrBINARY) as *const i8 },
                        ((1 as i32) as i8) as u8,
                        std::ptr::null_mut::<()>(),
                        Some(binCollFunc),
                        None,
                    );
                    createCollation(
                        *__slate_slot_1075,
                        unsafe { std::ptr::addr_of!(sqlite3StrBINARY) as *const i8 },
                        ((3 as i32) as i8) as u8,
                        std::ptr::null_mut::<()>(),
                        Some(binCollFunc),
                        None,
                    );
                    createCollation(
                        *__slate_slot_1075,
                        unsafe { std::ptr::addr_of!(sqlite3StrBINARY) as *const i8 },
                        ((2 as i32) as i8) as u8,
                        std::ptr::null_mut::<()>(),
                        Some(binCollFunc),
                        None,
                    );
                    createCollation(
                        *__slate_slot_1075,
                        (b"NOCASE\0".as_ptr() as *mut i8) as *const i8,
                        ((1 as i32) as i8) as u8,
                        std::ptr::null_mut::<()>(),
                        Some(nocaseCollatingFunc),
                        None,
                    );
                    createCollation(
                        *__slate_slot_1075,
                        (b"RTRIM\0".as_ptr() as *mut i8) as *const i8,
                        ((1 as i32) as i8) as u8,
                        std::ptr::null_mut::<()>(),
                        Some(rtrimCollFunc),
                        None,
                    );
                    if (unsafe { (*(*__slate_slot_1075)).mallocFailed }) != (0 as u8) {
                    } else {
                        // Parse the filename/URI argument
                        //
                        // Only allow sensible combinations of bits in the flags argument.
                        // Throw an error if any non-sense combination is used.  If we
                        // do not block illegal combinations here, it could trigger
                        // assert() statements in deeper layers.  Sensible combinations
                        // are:
                        //
                        //  1:  SQLITE_OPEN_READONLY
                        //  2:  SQLITE_OPEN_READWRITE
                        //  6:  SQLITE_OPEN_READWRITE | SQLITE_OPEN_CREATE
                        unsafe {
                            (*(*__slate_slot_1075)).openFlags = flags;
                        }
                        0 as i32;
                        0 as i32;
                        0 as i32;
                        {}
                        // READONLY
                        {}
                        // READWRITE
                        {}
                        // READWRITE | CREATE
                        if (1 as i32) << (flags & ((7 as i32) as u32)) & (70 as i32) == (0 as i32) {
                            *__slate_slot_1076 = sqlite3MisuseError(3618 as i32); // IMP: R-18321-05872
                        } else {
                            if zFilename == std::ptr::null::<i8>() {
                                zFilename = (b":memory:\0".as_ptr() as *mut i8) as *const i8;
                            }
                            *__slate_slot_1076 = sqlite3ParseUri(
                                zVfs,
                                zFilename,
                                std::ptr::addr_of_mut!(flags),
                                unsafe { std::ptr::addr_of_mut!((*(*__slate_slot_1075)).pVfs) },
                                std::ptr::addr_of_mut!(*__slate_slot_1078),
                                std::ptr::addr_of_mut!(*__slate_slot_1079),
                            );
                        }
                        if *__slate_slot_1076 != (0 as i32) {
                            if *__slate_slot_1076 == (7 as i32) {
                                unsafe { sqlite3OomFault(*__slate_slot_1075) };
                            }
                            unsafe {
                                sqlite3ErrorWithMsg(
                                    *__slate_slot_1075,
                                    *__slate_slot_1076,
                                    (if *__slate_slot_1079 != std::ptr::null_mut::<i8>() {
                                        b"%s\0".as_ptr() as *mut i8
                                    } else {
                                        std::ptr::null_mut::<i8>()
                                    }) as *const i8,
                                    *__slate_slot_1079,
                                )
                            };
                            unsafe { sqlite3_free(*__slate_slot_1079 as *mut ()) };
                        } else {
                            0 as i32;
                            // Open the backend database driver
                            *__slate_slot_1076 = unsafe {
                                sqlite3BtreeOpen(
                                    unsafe { (*(*__slate_slot_1075)).pVfs },
                                    *__slate_slot_1078 as *const i8,
                                    *__slate_slot_1075,
                                    unsafe {
                                        std::ptr::addr_of_mut!(
                                            (*unsafe {
                                                unsafe { (*(*__slate_slot_1075)).aDb }
                                                    .offset((0 as i32) as isize)
                                            })
                                            .pBt
                                        )
                                    },
                                    0 as i32,
                                    (flags | ((256 as i32) as u32)) as i32,
                                )
                            };
                            if *__slate_slot_1076 != (0 as i32) {
                                if *__slate_slot_1076 == (10 as i32) | (12 as i32) << (8 as i32) {
                                    *__slate_slot_1076 = 7 as i32;
                                }
                                unsafe { sqlite3Error(*__slate_slot_1075, *__slate_slot_1076) };
                            } else {
                                unsafe {
                                    sqlite3BtreeEnter(unsafe {
                                        (*unsafe {
                                            unsafe { (*(*__slate_slot_1075)).aDb }
                                                .offset((0 as i32) as isize)
                                        })
                                        .pBt
                                    })
                                };
                                unsafe {
                                    (*unsafe {
                                        unsafe { (*(*__slate_slot_1075)).aDb }
                                            .offset((0 as i32) as isize)
                                    })
                                    .pSchema = unsafe {
                                        sqlite3SchemaGet(*__slate_slot_1075, unsafe {
                                            (*unsafe {
                                                unsafe { (*(*__slate_slot_1075)).aDb }
                                                    .offset((0 as i32) as isize)
                                            })
                                            .pBt
                                        })
                                    };
                                }
                                if !((unsafe { (*(*__slate_slot_1075)).mallocFailed }) != (0 as u8))
                                {
                                    unsafe {
                                        sqlite3SetTextEncoding(*__slate_slot_1075, unsafe {
                                            (*unsafe {
                                                (*unsafe {
                                                    unsafe { (*(*__slate_slot_1075)).aDb }
                                                        .offset((0 as i32) as isize)
                                                })
                                                .pSchema
                                            })
                                            .enc
                                        })
                                    };
                                }
                                unsafe {
                                    sqlite3BtreeLeave(unsafe {
                                        (*unsafe {
                                            unsafe { (*(*__slate_slot_1075)).aDb }
                                                .offset((0 as i32) as isize)
                                        })
                                        .pBt
                                    })
                                };
                                unsafe {
                                    (*unsafe {
                                        unsafe { (*(*__slate_slot_1075)).aDb }
                                            .offset((1 as i32) as isize)
                                    })
                                    .pSchema = unsafe {
                                        sqlite3SchemaGet(
                                            *__slate_slot_1075,
                                            std::ptr::null_mut::<Btree>(),
                                        )
                                    };
                                }
                                // The default safety_level for the main database is FULL; for the temp
                                // database it is OFF. This matches the pager layer defaults.
                                unsafe {
                                    (*unsafe {
                                        unsafe { (*(*__slate_slot_1075)).aDb }
                                            .offset((0 as i32) as isize)
                                    })
                                    .zDbSName = b"main\0".as_ptr() as *mut i8;
                                }
                                unsafe {
                                    (*unsafe {
                                        unsafe { (*(*__slate_slot_1075)).aDb }
                                            .offset((0 as i32) as isize)
                                    })
                                    .safety_level = (((2 as i32) + (1 as i32)) as i8) as u8;
                                }
                                unsafe {
                                    (*unsafe {
                                        unsafe { (*(*__slate_slot_1075)).aDb }
                                            .offset((1 as i32) as isize)
                                    })
                                    .zDbSName = b"temp\0".as_ptr() as *mut i8;
                                }
                                unsafe {
                                    (*unsafe {
                                        unsafe { (*(*__slate_slot_1075)).aDb }
                                            .offset((1 as i32) as isize)
                                    })
                                    .safety_level = ((1 as i32) as i8) as u8;
                                }
                                unsafe {
                                    (*(*__slate_slot_1075)).eOpenState = ((118 as i32) as i8) as u8;
                                }
                                if (unsafe { (*(*__slate_slot_1075)).mallocFailed }) != (0 as u8) {
                                } else {
                                    // Register all built-in functions, but do not attempt to read the
                                    // database schema yet. This is delayed until the first time the database
                                    // is accessed.
                                    unsafe { sqlite3Error(*__slate_slot_1075, 0 as i32) };
                                    unsafe {
                                        sqlite3RegisterPerConnectionBuiltinFunctions(
                                            *__slate_slot_1075,
                                        )
                                    };
                                    *__slate_slot_1076 = sqlite3_errcode(*__slate_slot_1075);
                                    // Load compiled-in extensions
                                    *__slate_slot_1080 = 0 as i32;
                                    loop {
                                        if *__slate_slot_1076 == (0 as i32)
                                            && *__slate_slot_1080
                                                < ((((56 as u64) / (8 as u64)) as u32) as i32)
                                        {
                                            *__slate_slot_1076 = unsafe {
                                                unsafe { *unsafe { unsafe { std::ptr::addr_of!(sqlite3BuiltinExtensions.0) as *const Option<unsafe extern "C-unwind" fn(*mut sqlite3) -> i32> }.offset(*__slate_slot_1080 as isize) } }.unwrap()(*__slate_slot_1075)
                                            };
                                            std::ptr::write(__slate_slot_1982, *__slate_slot_1080);
                                            std::ptr::write(
                                                __slate_slot_1983,
                                                *__slate_slot_1982 + (1 as i32),
                                            );
                                            *__slate_slot_1080 = *__slate_slot_1983;
                                        } else {
                                            break;
                                        }
                                    }
                                    // Load automatic extensions - extensions that have been registered
                                    // using the sqlite3_automatic_extension() API.
                                    if *__slate_slot_1076 == (0 as i32) {
                                        unsafe { sqlite3AutoLoadExtensions(*__slate_slot_1075) };
                                        *__slate_slot_1076 = sqlite3_errcode(*__slate_slot_1075);
                                        if *__slate_slot_1076 != (0 as i32) {
                                            break '__join_6;
                                        }
                                    }
                                    // -DSQLITE_DEFAULT_LOCKING_MODE=1 makes EXCLUSIVE the default locking
                                    // mode.  -DSQLITE_DEFAULT_LOCKING_MODE=0 make NORMAL the default locking
                                    // mode.  Doing nothing at all also makes NORMAL the default.
                                    if *__slate_slot_1076 != (0 as i32) {
                                        unsafe {
                                            sqlite3Error(*__slate_slot_1075, *__slate_slot_1076)
                                        };
                                    }
                                    // Enable the lookaside-malloc subsystem
                                    setupLookaside(
                                        *__slate_slot_1075,
                                        std::ptr::null_mut::<()>(),
                                        unsafe { sqlite3Config.szLookaside },
                                        unsafe { sqlite3Config.nLookaside },
                                    );
                                    sqlite3_wal_autocheckpoint(*__slate_slot_1075, 1000 as i32);
                                }
                            }
                        }
                    }
                }
            }
            if *__slate_slot_1075 != std::ptr::null_mut::<sqlite3>() {
                0 as i32;
                unsafe { sqlite3_mutex_leave(unsafe { (*(*__slate_slot_1075)).mutex }) };
            }
            *__slate_slot_1076 = sqlite3_errcode(*__slate_slot_1075);
            0 as i32;
            if *__slate_slot_1076 & (255 as i32) == (7 as i32) {
                sqlite3_close(*__slate_slot_1075);
                *__slate_slot_1075 = std::ptr::null_mut::<sqlite3>();
            } else {
                if *__slate_slot_1076 != (0 as i32) {
                    unsafe {
                        (*(*__slate_slot_1075)).eOpenState = ((186 as i32) as i8) as u8;
                    }
                }
            }
            unsafe {
                *ppDb = *__slate_slot_1075;
            }
            sqlite3_free_filename(*__slate_slot_1078 as *const i8);
            return *__slate_slot_1076;
        }
    }
    return unsafe { std::mem::zeroed() };
}

/// Open a new database handle.
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.main.sqlite3_open")]
extern "C-unwind" fn sqlite3_open(mut zFilename: *const i8, mut ppDb: *mut *mut sqlite3) -> i32 {
    return openDatabase(
        zFilename,
        ppDb,
        ((2 as i32) | (4 as i32)) as u32,
        std::ptr::null::<i8>(),
    );
}

/// # Arguments
///
/// * `filename` - Database filename (UTF-8)
/// * `ppDb` - OUT: SQLite db handle
/// * `flags` - Flags
/// * `zVfs` - Name of VFS module to use
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.main.sqlite3_open_v2")]
extern "C-unwind" fn sqlite3_open_v2(
    mut filename: *const i8,
    mut ppDb: *mut *mut sqlite3,
    mut flags: i32,
    mut zVfs: *const i8,
) -> i32 {
    return openDatabase(filename, ppDb, flags as u32, zVfs);
}

/// Open a new database handle.
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.main.sqlite3_open16")]
extern "C-unwind" fn sqlite3_open16(mut zFilename: *const (), mut ppDb: *mut *mut sqlite3) -> i32 {
    let mut zFilename8: *const i8 = unsafe { std::mem::zeroed() }; // zFilename encoded in UTF-8 instead of UTF-16
    let mut pVal: *mut sqlite3_value = unsafe { std::mem::zeroed() };
    let mut rc: i32 = 0 as i32;
    unsafe {
        *ppDb = std::ptr::null_mut::<sqlite3>();
    }
    rc = sqlite3_initialize();
    if rc != (0 as i32) {
        return rc;
    }
    if zFilename == std::ptr::null::<()>() {
        zFilename = (b"\0\0\0".as_ptr() as *mut i8) as *const ();
    }
    pVal = unsafe { sqlite3ValueNew(std::ptr::null_mut::<sqlite3>()) };
    unsafe { sqlite3ValueSetStr(pVal, -(1 as i32), zFilename, ((2 as i32) as i8) as u8, None) };
    zFilename8 = (unsafe { sqlite3ValueText(pVal, ((1 as i32) as i8) as u8) }) as *const i8;
    if zFilename8 != std::ptr::null::<i8>() {
        rc = openDatabase(
            zFilename8,
            ppDb,
            ((2 as i32) | (4 as i32)) as u32,
            std::ptr::null::<i8>(),
        );
        0 as i32;
        if rc == (0 as i32)
            && !((((unsafe {
                (*unsafe {
                    (*unsafe { unsafe { (*unsafe { *ppDb }).aDb }.offset((0 as i32) as isize) })
                        .pSchema
                })
                .schemaFlags
            }) as u32) as i32)
                & (1 as i32)
                == (1 as i32))
        {
            unsafe {
                (*unsafe { *ppDb }).enc = ((2 as i32) as i8) as u8;
            }
            unsafe {
                (*unsafe {
                    (*unsafe { unsafe { (*unsafe { *ppDb }).aDb }.offset((0 as i32) as isize) })
                        .pSchema
                })
                .enc = ((2 as i32) as i8) as u8;
            }
        }
    } else {
        rc = 7 as i32;
    }
    unsafe { sqlite3ValueFree(pVal) };
    return rc & (255 as i32);
}

/// Register a new collation sequence with the database handle db.
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.main.sqlite3_create_collation")]
extern "C-unwind" fn sqlite3_create_collation(
    mut db: *mut sqlite3,
    mut zName: *const i8,
    mut enc: i32,
    mut pCtx: *mut (),
    mut xCompare: Option<
        unsafe extern "C-unwind" fn(*mut (), i32, *const (), i32, *const ()) -> i32,
    >,
) -> i32 {
    return sqlite3_create_collation_v2(db, zName, enc, pCtx, xCompare, None);
}

/// Register a new collation sequence with the database handle db.
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.main.sqlite3_create_collation_v2")]
extern "C-unwind" fn sqlite3_create_collation_v2(
    mut db: *mut sqlite3,
    mut zName: *const i8,
    mut enc: i32,
    mut pCtx: *mut (),
    mut xCompare: Option<
        unsafe extern "C-unwind" fn(*mut (), i32, *const (), i32, *const ()) -> i32,
    >,
    mut xDel: Option<unsafe extern "C-unwind" fn(*mut ())>,
) -> i32 {
    let mut rc: i32 = 0 as i32;
    unsafe { sqlite3_mutex_enter(unsafe { (*db).mutex }) };
    0 as i32;
    rc = createCollation(db, zName, (enc as i8) as u8, pCtx, xCompare, xDel);
    rc = unsafe { sqlite3ApiExit(db, rc) };
    unsafe { sqlite3_mutex_leave(unsafe { (*db).mutex }) };
    return rc;
}

/// Register a new collation sequence with the database handle db.
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.main.sqlite3_create_collation16")]
extern "C-unwind" fn sqlite3_create_collation16(
    mut db: *mut sqlite3,
    mut zName: *const (),
    mut enc: i32,
    mut pCtx: *mut (),
    mut xCompare: Option<
        unsafe extern "C-unwind" fn(*mut (), i32, *const (), i32, *const ()) -> i32,
    >,
) -> i32 {
    let mut rc: i32 = 0 as i32;
    let mut zName8: *mut i8 = unsafe { std::mem::zeroed() };
    unsafe { sqlite3_mutex_enter(unsafe { (*db).mutex }) };
    0 as i32;
    zName8 = unsafe { sqlite3Utf16to8(db, zName, -(1 as i32), ((2 as i32) as i8) as u8) };
    if zName8 != std::ptr::null_mut::<i8>() {
        rc = createCollation(
            db,
            zName8 as *const i8,
            (enc as i8) as u8,
            pCtx,
            xCompare,
            None,
        );
        unsafe { sqlite3DbFree(db, zName8 as *mut ()) };
    }
    rc = unsafe { sqlite3ApiExit(db, rc) };
    unsafe { sqlite3_mutex_leave(unsafe { (*db).mutex }) };
    return rc;
}

/// Register a collation sequence factory callback with the database handle
/// db. Replace any previously installed collation sequence factory.
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.main.sqlite3_collation_needed")]
extern "C-unwind" fn sqlite3_collation_needed(
    mut db: *mut sqlite3,
    mut pCollNeededArg: *mut (),
    mut xCollNeeded: Option<unsafe extern "C-unwind" fn(*mut (), *mut sqlite3, i32, *const i8)>,
) -> i32 {
    unsafe { sqlite3_mutex_enter(unsafe { (*db).mutex }) };
    unsafe {
        (*db).xCollNeeded = xCollNeeded;
    }
    unsafe {
        (*db).xCollNeeded16 = None;
    }
    unsafe {
        (*db).pCollNeededArg = pCollNeededArg;
    }
    unsafe { sqlite3_mutex_leave(unsafe { (*db).mutex }) };
    return 0 as i32;
}

/// Register a collation sequence factory callback with the database handle
/// db. Replace any previously installed collation sequence factory.
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.main.sqlite3_collation_needed16")]
extern "C-unwind" fn sqlite3_collation_needed16(
    mut db: *mut sqlite3,
    mut pCollNeededArg: *mut (),
    mut xCollNeeded16: Option<unsafe extern "C-unwind" fn(*mut (), *mut sqlite3, i32, *const ())>,
) -> i32 {
    unsafe { sqlite3_mutex_enter(unsafe { (*db).mutex }) };
    unsafe {
        (*db).xCollNeeded = None;
    }
    unsafe {
        (*db).xCollNeeded16 = xCollNeeded16;
    }
    unsafe {
        (*db).pCollNeededArg = pCollNeededArg;
    }
    unsafe { sqlite3_mutex_leave(unsafe { (*db).mutex }) };
    return 0 as i32;
}

/// Find existing client data.
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.main.sqlite3_get_clientdata")]
extern "C-unwind" fn sqlite3_get_clientdata(mut db: *mut sqlite3, mut zName: *const i8) -> *mut () {
    let mut p: *mut DbClientData = unsafe { std::mem::zeroed() };
    unsafe { sqlite3_mutex_enter(unsafe { (*db).mutex }) };
    p = unsafe { (*db).pDbData };
    '__slate_break_1783: while p != std::ptr::null_mut::<DbClientData>() {
        if (unsafe {
            strcmp(
                (unsafe { std::ptr::addr_of_mut!((*p).zName) as *mut i8 }) as *const i8,
                zName,
            )
        }) == (0 as i32)
        {
            let mut pResult: *mut () = unsafe { (*p).pData };
            unsafe { sqlite3_mutex_leave(unsafe { (*db).mutex }) };
            return pResult;
        }
        p = unsafe { (*p).pNext };
    }
    unsafe { sqlite3_mutex_leave(unsafe { (*db).mutex }) };
    return std::ptr::null_mut::<()>();
}

/// Add new client data to a database connection.
///
/// # Arguments
///
/// * `db` - Attach client data to this connection
/// * `zName` - Name of the client data
/// * `pData` - The client data itself
/// * `xDestructor` - Destructor
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.main.sqlite3_set_clientdata")]
extern "C-unwind" fn sqlite3_set_clientdata(
    mut db: *mut sqlite3,
    mut zName: *const i8,
    mut pData: *mut (),
    mut xDestructor: Option<unsafe extern "C-unwind" fn(*mut ())>,
) -> i32 {
    let mut p: *mut DbClientData = unsafe { std::mem::zeroed() };
    let mut pp: *mut *mut DbClientData = unsafe { std::mem::zeroed() };
    unsafe { sqlite3_mutex_enter(unsafe { (*db).mutex }) };
    pp = unsafe { std::ptr::addr_of_mut!((*db).pDbData) };
    p = unsafe { (*db).pDbData };
    '__slate_break_1784: while p != std::ptr::null_mut::<DbClientData>()
        && (unsafe {
            strcmp(
                (unsafe { std::ptr::addr_of_mut!((*p).zName) as *mut i8 }) as *const i8,
                zName,
            )
        }) != (0 as i32)
    {
        pp = unsafe { std::ptr::addr_of_mut!((*p).pNext) };
        p = unsafe { (*p).pNext };
    }
    if p != std::ptr::null_mut::<DbClientData>() {
        0 as i32;
        if (unsafe { (*p).xDestructor }) != None {
            unsafe { unsafe { (*p).xDestructor }.unwrap()(unsafe { (*p).pData }) };
        }
        if pData == std::ptr::null_mut::<()>() {
            unsafe {
                *pp = unsafe { (*p).pNext };
            }
            unsafe { sqlite3_free(p as *mut ()) };
            unsafe { sqlite3_mutex_leave(unsafe { (*db).mutex }) };
            return 0 as i32;
        }
    } else {
        if pData == std::ptr::null_mut::<()>() {
            unsafe { sqlite3_mutex_leave(unsafe { (*db).mutex }) };
            return 0 as i32;
        } else {
            let mut n: u64 = unsafe { strlen(zName) };
            p = (unsafe {
                sqlite3_malloc64(
                    (24 as u64).wrapping_add(n.wrapping_add(((1 as i32) as i64) as u64)),
                )
            }) as *mut DbClientData;
            if p == std::ptr::null_mut::<DbClientData>() {
                if xDestructor != None {
                    unsafe { xDestructor.unwrap()(pData) };
                }
                unsafe { sqlite3_mutex_leave(unsafe { (*db).mutex }) };
                return 7 as i32;
            }
            unsafe {
                memcpy(
                    (unsafe { std::ptr::addr_of_mut!((*p).zName) as *mut i8 }) as *mut (),
                    zName as *const (),
                    n.wrapping_add(((1 as i32) as i64) as u64),
                )
            };
            unsafe {
                (*p).pNext = unsafe { (*db).pDbData };
            }
            unsafe {
                (*db).pDbData = p;
            }
        }
    }
    unsafe {
        (*p).pData = pData;
    }
    unsafe {
        (*p).xDestructor = xDestructor;
    }
    unsafe { sqlite3_mutex_leave(unsafe { (*db).mutex }) };
    return 0 as i32;
}

/// This function is now an anachronism. It used to be used to recover from a
/// malloc() failure, but SQLite now does this automatically.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3_global_recover() -> i32 {
    return 0 as i32;
}

/// Test to see whether or not the database connection is in autocommit
/// mode.  Return TRUE if it is and FALSE if not.  Autocommit mode is on
/// by default.  Autocommit is disabled by a BEGIN statement and reenabled
/// by the next COMMIT or ROLLBACK.
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.main.sqlite3_get_autocommit")]
extern "C-unwind" fn sqlite3_get_autocommit(mut db: *mut sqlite3) -> i32 {
    let mut iRet: i32 = 0 as i32;
    unsafe { sqlite3_mutex_enter(unsafe { (*db).mutex }) };
    iRet = ((unsafe { (*db).autoCommit }) as u32) as i32;
    unsafe { sqlite3_mutex_leave(unsafe { (*db).mutex }) };
    return iRet;
}

/// The following routines are substitutes for constants SQLITE_CORRUPT,
/// SQLITE_MISUSE, SQLITE_CANTOPEN, SQLITE_NOMEM and possibly other error
/// constants.  They serve two purposes:
///
///   1.  Serve as a convenient place to set a breakpoint in a debugger
///       to detect when version error conditions occurs.
///
///   2.  Invoke sqlite3_log() to provide the source code location where
///       a low-level error is first detected.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3ReportError(
    mut iErr: i32,
    mut lineno: i32,
    mut zType: *const i8,
) -> i32 {
    unsafe {
        sqlite3_log(
            iErr,
            (b"%s at line %d of [%.10s]\0".as_ptr() as *mut i8) as *const i8,
            zType,
            lineno,
            unsafe { sqlite3_sourceid().offset((20 as i32) as isize) },
        )
    };
    return iErr;
}

#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3CorruptError(mut lineno: i32) -> i32 {
    {}
    return sqlite3ReportError(
        11 as i32,
        lineno,
        (b"database corruption\0".as_ptr() as *mut i8) as *const i8,
    );
}

#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3MisuseError(mut lineno: i32) -> i32 {
    {}
    return sqlite3ReportError(
        21 as i32,
        lineno,
        (b"misuse\0".as_ptr() as *mut i8) as *const i8,
    );
}

#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3CantopenError(mut lineno: i32) -> i32 {
    {}
    return sqlite3ReportError(
        14 as i32,
        lineno,
        (b"cannot open file\0".as_ptr() as *mut i8) as *const i8,
    );
}

/// This is a convenience routine that makes sure that all thread-specific
/// data for this thread has been deallocated.
///
/// SQLite no longer uses thread-specific data so this routine is now a
/// no-op.  It is retained for historical compatibility.
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.main.sqlite3_thread_cleanup")]
extern "C-unwind" fn sqlite3_thread_cleanup() {}

/// Return meta information about a specific column of a database table.
/// See comment in sqlite3.h (sqlite.h.in) for details.
///
/// # Arguments
///
/// * `db` - Connection handle
/// * `zDbName` - Database name or NULL
/// * `zTableName` - Table name
/// * `zColumnName` - Column name
/// * `pzDataType` - OUTPUT: Declared data type
/// * `pzCollSeq` - OUTPUT: Collation sequence name
/// * `pNotNull` - OUTPUT: True if NOT NULL constraint exists
/// * `pPrimaryKey` - OUTPUT: True if column part of PK
/// * `pAutoinc` - OUTPUT: True if column is auto-increment
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3_table_column_metadata(
    mut db: *mut sqlite3,
    mut zDbName: *const i8,
    mut zTableName: *const i8,
    mut zColumnName: *const i8,
    mut pzDataType: *mut *const i8,
    mut pzCollSeq: *mut *const i8,
    mut pNotNull: *mut i32,
    mut pPrimaryKey: *mut i32,
    mut pAutoinc: *mut i32,
) -> i32 {
    let mut __slate_storage_1866: std::mem::MaybeUninit<bool> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1866: *mut bool = std::ptr::addr_of_mut!(__slate_storage_1866) as *mut bool;
    let mut __slate_storage_1157: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1157: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1157) as *mut i32;
    let mut __slate_storage_1156: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1156: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1156) as *mut i32;
    let mut __slate_storage_1155: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1155: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1155) as *mut i32;
    let mut __slate_storage_1154: std::mem::MaybeUninit<*const i8> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1154: *mut *const i8 =
        std::ptr::addr_of_mut!(__slate_storage_1154) as *mut *const i8;
    let mut __slate_storage_1153: std::mem::MaybeUninit<*const i8> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1153: *mut *const i8 =
        std::ptr::addr_of_mut!(__slate_storage_1153) as *mut *const i8;
    let mut __slate_storage_1152: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1152: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1152) as *mut i32;
    let mut __slate_storage_1151: std::mem::MaybeUninit<*mut Column> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1151: *mut *mut Column =
        std::ptr::addr_of_mut!(__slate_storage_1151) as *mut *mut Column;
    let mut __slate_storage_1150: std::mem::MaybeUninit<*mut Table> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1150: *mut *mut Table =
        std::ptr::addr_of_mut!(__slate_storage_1150) as *mut *mut Table;
    let mut __slate_storage_1149: std::mem::MaybeUninit<*mut i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1149: *mut *mut i8 =
        std::ptr::addr_of_mut!(__slate_storage_1149) as *mut *mut i8;
    let mut __slate_storage_1148: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1148: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1148) as *mut i32;
    unsafe {
        '__join_12: {
            std::ptr::write(__slate_slot_1149, std::ptr::null_mut::<i8>());
            std::ptr::write(__slate_slot_1150, std::ptr::null_mut::<Table>());
            std::ptr::write(__slate_slot_1151, std::ptr::null_mut::<Column>());
            std::ptr::write(__slate_slot_1152, 0 as i32);
            std::ptr::write(__slate_slot_1153, std::ptr::null::<i8>());
            std::ptr::write(__slate_slot_1154, std::ptr::null::<i8>());
            std::ptr::write(__slate_slot_1155, 0 as i32);
            std::ptr::write(__slate_slot_1156, 0 as i32);
            std::ptr::write(__slate_slot_1157, 0 as i32);
            // Ensure the database schema has been loaded
            unsafe { sqlite3_mutex_enter(unsafe { (*db).mutex }) };
            unsafe { sqlite3BtreeEnterAll(db) };
            *__slate_slot_1148 =
                unsafe { sqlite3Init(db, std::ptr::addr_of_mut!(*__slate_slot_1149)) };
            if (0 as i32) != *__slate_slot_1148 {
            } else {
                // Locate the table in question
                *__slate_slot_1150 = unsafe { sqlite3FindTable(db, zTableName, zDbName) };
                if !(*__slate_slot_1150 != std::ptr::null_mut::<Table>())
                    || (((unsafe { (*(*__slate_slot_1150)).eTabType }) as u32) as i32) == (2 as i32)
                {
                    *__slate_slot_1150 = std::ptr::null_mut::<Table>();
                } else {
                    // Find the column for which info is requested
                    if zColumnName == std::ptr::null::<i8>() {
                        // Query for existence of table only
                    } else {
                        *__slate_slot_1152 =
                            unsafe { sqlite3ColumnIndex(*__slate_slot_1150, zColumnName) };
                        if *__slate_slot_1152 >= (0 as i32) {
                            *__slate_slot_1151 = unsafe {
                                unsafe { (*(*__slate_slot_1150)).aCol }
                                    .offset(*__slate_slot_1152 as isize)
                            };
                        } else {
                            '__join_22: {
                                if (unsafe { (*(*__slate_slot_1150)).tabFlags })
                                    & ((128 as i32) as u32)
                                    == ((0 as i32) as u32)
                                {
                                    *__slate_slot_1866 =
                                        (unsafe { sqlite3IsRowid(zColumnName) }) != (0 as i32);
                                } else {
                                    *__slate_slot_1866 = false as bool;
                                }
                            }
                            if *__slate_slot_1866 {
                                *__slate_slot_1152 =
                                    (unsafe { (*(*__slate_slot_1150)).iPKey }) as i32;
                                *__slate_slot_1151 = if *__slate_slot_1152 >= (0 as i32) {
                                    unsafe {
                                        unsafe { (*(*__slate_slot_1150)).aCol }
                                            .offset(*__slate_slot_1152 as isize)
                                    }
                                } else {
                                    std::ptr::null_mut::<Column>()
                                };
                            } else {
                                *__slate_slot_1150 = std::ptr::null_mut::<Table>();
                                break '__join_12;
                            }
                        }
                    }
                    // The following block stores the meta information that will be returned
                    // to the caller in local variables zDataType, zCollSeq, notnull, primarykey
                    // and autoinc. At this point there are two possibilities:
                    //
                    //     1. The specified column name was rowid", "oid" or "_rowid_"
                    //        and there is no explicitly declared IPK column.
                    //
                    //     2. The table is not a view and the column name identified an
                    //        explicitly declared column. Copy meta information from *pCol.
                    if *__slate_slot_1151 != std::ptr::null_mut::<Column>() {
                        *__slate_slot_1153 = (unsafe {
                            sqlite3ColumnType(*__slate_slot_1151, std::ptr::null_mut::<i8>())
                        }) as *const i8;
                        *__slate_slot_1154 = unsafe { sqlite3ColumnColl(*__slate_slot_1151) };
                        *__slate_slot_1155 =
                            (((unsafe { (*(*__slate_slot_1151)).__slate_bits_0.__get_notNull() })
                                as i32)
                                != (0 as i32)) as i32;
                        *__slate_slot_1156 = ((((unsafe { (*(*__slate_slot_1151)).colFlags })
                            as u32) as i32)
                            & (1 as i32)
                            != (0 as i32)) as i32;
                        *__slate_slot_1157 = (((unsafe { (*(*__slate_slot_1150)).iPKey }) as i32)
                            == *__slate_slot_1152
                            && (unsafe { (*(*__slate_slot_1150)).tabFlags }) & ((8 as i32) as u32)
                                != ((0 as i32) as u32))
                            as i32;
                    } else {
                        *__slate_slot_1153 = (b"INTEGER\0".as_ptr() as *mut i8) as *const i8;
                        *__slate_slot_1156 = 1 as i32;
                    }
                    if !(*__slate_slot_1154 != std::ptr::null::<i8>()) {
                        *__slate_slot_1154 =
                            unsafe { std::ptr::addr_of!(sqlite3StrBINARY) as *const i8 };
                    }
                }
            }
        }
        unsafe { sqlite3BtreeLeaveAll(db) };
        // Whether the function call succeeded or failed, set the output parameters
        // to whatever their local counterparts contain. If an error did occur,
        // this has the effect of zeroing all output parameters.
        if pzDataType != std::ptr::null_mut::<*const i8>() {
            unsafe {
                *pzDataType = *__slate_slot_1153;
            }
        }
        if pzCollSeq != std::ptr::null_mut::<*const i8>() {
            unsafe {
                *pzCollSeq = *__slate_slot_1154;
            }
        }
        if pNotNull != std::ptr::null_mut::<i32>() {
            unsafe {
                *pNotNull = *__slate_slot_1155;
            }
        }
        if pPrimaryKey != std::ptr::null_mut::<i32>() {
            unsafe {
                *pPrimaryKey = *__slate_slot_1156;
            }
        }
        if pAutoinc != std::ptr::null_mut::<i32>() {
            unsafe {
                *pAutoinc = *__slate_slot_1157;
            }
        }
        if (0 as i32) == *__slate_slot_1148
            && !(*__slate_slot_1150 != std::ptr::null_mut::<Table>())
        {
            unsafe { sqlite3DbFree(db, *__slate_slot_1149 as *mut ()) };
            *__slate_slot_1149 = unsafe {
                sqlite3MPrintf(
                    db,
                    (b"no such table column: %s.%s\0".as_ptr() as *mut i8) as *const i8,
                    zTableName,
                    zColumnName,
                )
            };
            *__slate_slot_1148 = 1 as i32;
        }
        unsafe {
            sqlite3ErrorWithMsg(
                db,
                *__slate_slot_1148,
                (if *__slate_slot_1149 != std::ptr::null_mut::<i8>() {
                    b"%s\0".as_ptr() as *mut i8
                } else {
                    std::ptr::null_mut::<i8>()
                }) as *const i8,
                *__slate_slot_1149,
            )
        };
        unsafe { sqlite3DbFree(db, *__slate_slot_1149 as *mut ()) };
        *__slate_slot_1148 = unsafe { sqlite3ApiExit(db, *__slate_slot_1148) };
        unsafe { sqlite3_mutex_leave(unsafe { (*db).mutex }) };
        return *__slate_slot_1148;
    }
    return unsafe { std::mem::zeroed() };
}

/// Sleep for a little while.  Return the amount of time slept.
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.main.sqlite3_sleep")]
extern "C-unwind" fn sqlite3_sleep(mut ms: i32) -> i32 {
    let mut pVfs: *mut sqlite3_vfs = unsafe { std::mem::zeroed() };
    let mut rc: i32 = 0 as i32;
    pVfs = unsafe { sqlite3_vfs_find(std::ptr::null::<i8>()) };
    if pVfs == std::ptr::null_mut::<sqlite3_vfs>() {
        return 0 as i32;
    }
    // This function works in milliseconds, but the underlying OsSleep()
    // API uses microseconds. Hence the 1000's.
    rc = (unsafe {
        sqlite3OsSleep(
            pVfs,
            if ms < (0 as i32) {
                0 as i32
            } else {
                (1000 as i32) * ms
            },
        )
    }) / (1000 as i32);
    return rc;
}

/// Enable or disable the extended result codes.
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.main.sqlite3_extended_result_codes")]
extern "C-unwind" fn sqlite3_extended_result_codes(mut db: *mut sqlite3, mut onoff: i32) -> i32 {
    unsafe { sqlite3_mutex_enter(unsafe { (*db).mutex }) };
    unsafe {
        (*db).errMask = (if onoff != (0 as i32) {
            4294967295 as u32
        } else {
            (255 as i32) as u32
        }) as i32;
    }
    unsafe { sqlite3_mutex_leave(unsafe { (*db).mutex }) };
    return 0 as i32;
}

/// Invoke the xFileControl method on a particular database.
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.main.sqlite3_file_control")]
extern "C-unwind" fn sqlite3_file_control(
    mut db: *mut sqlite3,
    mut zDbName: *const i8,
    mut op: i32,
    mut pArg: *mut (),
) -> i32 {
    let mut rc: i32 = 1 as i32;
    let mut pBtree: *mut Btree = unsafe { std::mem::zeroed() };
    unsafe { sqlite3_mutex_enter(unsafe { (*db).mutex }) };
    pBtree = sqlite3DbNameToBtree(db, zDbName);
    if pBtree != std::ptr::null_mut::<Btree>() {
        let mut pPager: *mut Pager = unsafe { std::mem::zeroed() };
        let mut fd: *mut sqlite3_file = unsafe { std::mem::zeroed() };
        unsafe { sqlite3BtreeEnter(pBtree) };
        pPager = unsafe { sqlite3BtreePager(pBtree) };
        0 as i32;
        fd = unsafe { sqlite3PagerFile(pPager) };
        0 as i32;
        if op == (7 as i32) {
            unsafe {
                *(pArg as *mut *mut sqlite3_file) = fd;
            }
            rc = 0 as i32;
        } else {
            if op == (27 as i32) {
                unsafe {
                    *(pArg as *mut *mut sqlite3_vfs) = unsafe { sqlite3PagerVfs(pPager) };
                }
                rc = 0 as i32;
            } else {
                if op == (28 as i32) {
                    unsafe {
                        *(pArg as *mut *mut sqlite3_file) = unsafe { sqlite3PagerJrnlFile(pPager) };
                    }
                    rc = 0 as i32;
                } else {
                    if op == (35 as i32) {
                        unsafe {
                            *(pArg as *mut u32) = unsafe { sqlite3PagerDataVersion(pPager) };
                        }
                        rc = 0 as i32;
                    } else {
                        if op == (38 as i32) {
                            let mut iNew: i32 = unsafe { *(pArg as *mut i32) };
                            unsafe {
                                *(pArg as *mut i32) =
                                    unsafe { sqlite3BtreeGetRequestedReserve(pBtree) };
                            }
                            if iNew >= (0 as i32) && iNew <= (255 as i32) {
                                unsafe {
                                    sqlite3BtreeSetPageSize(pBtree, 0 as i32, iNew, 0 as i32)
                                };
                            }
                            rc = 0 as i32;
                        } else {
                            if op == (42 as i32) {
                                unsafe { sqlite3BtreeClearCache(pBtree) };
                                rc = 0 as i32;
                            } else {
                                let mut nSave: i32 = unsafe { (*db).busyHandler.nBusy };
                                rc = unsafe { sqlite3OsFileControl(fd, op, pArg) };
                                unsafe {
                                    (*db).busyHandler.nBusy = nSave;
                                }
                            }
                        }
                    }
                }
            }
        }
        unsafe { sqlite3BtreeLeave(pBtree) };
    }
    unsafe { sqlite3_mutex_leave(unsafe { (*db).mutex }) };
    return rc;
}

/// Interface to the testing logic.
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.main.sqlite3_test_control")]
unsafe extern "C-unwind" fn sqlite3_test_control(mut op: i32, mut __va_args: ...) -> i32 {
    let mut rc: i32 = 0 as i32;
    let mut ap: core::ffi::VaList<'_> = unsafe { std::mem::zeroed() };
    ap = __va_args.clone();
    '__slate_break_1792: {
        match op {
            5 => {
                // Save the current state of the PRNG.
                unsafe { sqlite3PrngSaveState() };
                break '__slate_break_1792;
                // Restore the state of the PRNG to the last state saved using
                // PRNG_SAVE.  If PRNG_SAVE has never before been called, then
                // this verb acts like PRNG_RESET.
            }
            6 => {
                unsafe { sqlite3PrngRestoreState() };
                break '__slate_break_1792;
                //  sqlite3_test_control(SQLITE_TESTCTRL_PRNG_SEED, int x, sqlite3 *db);
                //
                // Control the seed for the pseudo-random number generator (PRNG) that
                // is built into SQLite.  Cases:
                //
                //    x!=0 && db!=0       Seed the PRNG to the current value of the
                //                        schema cookie in the main database for db, or
                //                        x if the schema cookie is zero.  This case
                //                        is convenient to use with database fuzzers
                //                        as it allows the fuzzer some control over the
                //                        the PRNG seed.
                //
                //    x!=0 && db==0       Seed the PRNG to the value of x.
                //
                //    x==0 && db==0       Revert to default behavior of using the
                //                        xRandomness method on the primary VFS.
                //
                // This test-control also resets the PRNG so that the new seed will
                // be used for the next call to sqlite3_randomness().
            }
            28 => {
                let mut x: i32 = unsafe { ap.next_arg::<i32>() };
                let mut y: i32 = 0 as i32;
                let mut db: *mut sqlite3 = unsafe { ap.next_arg::<*mut sqlite3>() };
                0 as i32;
                let __v1867: bool;
                if db != std::ptr::null_mut::<sqlite3>() {
                    let __v1868: i32 = unsafe {
                        (*unsafe {
                            (*unsafe { unsafe { (*db).aDb }.offset((0 as i32) as isize) }).pSchema
                        })
                        .schema_cookie
                    };
                    y = __v1868;
                    __v1867 = __v1868 != (0 as i32);
                } else {
                    __v1867 = false as bool;
                }
                if __v1867 {
                    x = y;
                }
                unsafe {
                    sqlite3Config.iPrngSeed = x as u32;
                }
                unsafe { sqlite3_randomness(0 as i32, std::ptr::null_mut::<()>()) };
                break '__slate_break_1792;
                //  sqlite3_test_control(SQLITE_TESTCTRL_FK_NO_ACTION, sqlite3 *db, int b);
                //
                // If b is true, then activate the SQLITE_FkNoAction setting.  If b is
                // false then clear that setting.  If the SQLITE_FkNoAction setting is
                // enabled, all foreign key ON DELETE and ON UPDATE actions behave as if
                // they were NO ACTION, regardless of how they are defined.
                //
                // NB:  One must usually run "PRAGMA writable_schema=RESET" after
                // using this test-control, before it will take full effect.  failing
                // to reset the schema can result in some unexpected behavior.
            }
            7 => {
                let mut db: *mut sqlite3 = unsafe { ap.next_arg::<*mut sqlite3>() };
                let mut b: i32 = unsafe { ap.next_arg::<i32>() };
                if b != (0 as i32) {
                    let __v1869: *mut sqlite3 = db;
                    let __v1870: u64 = unsafe { (*__v1869).flags };
                    let __v1871: u64 = __v1870 | (((8 as i32) as i64) as u64) << (32 as i32);
                    unsafe {
                        (*__v1869).flags = __v1871;
                    }
                } else {
                    let __v1872: *mut sqlite3 = db;
                    let __v1873: u64 = unsafe { (*__v1872).flags };
                    let __v1874: u64 = __v1873 & !((((8 as i32) as i64) as u64) << (32 as i32));
                    unsafe {
                        (*__v1872).flags = __v1874;
                    }
                }
                break '__slate_break_1792;
                //  sqlite3_test_control(BITVEC_TEST, size, program)
                //
                // Run a test against a Bitvec object of size.  The program argument
                // is an array of integers that defines the test.  Return -1 on a
                // memory allocation error, 0 on success, or non-zero for an error.
                // See the sqlite3BitvecBuiltinTest() for additional information.
            }
            8 => {
                let mut sz: i32 = unsafe { ap.next_arg::<i32>() };
                let mut aProg: *mut i32 = unsafe { ap.next_arg::<*mut i32>() };
                rc = unsafe { sqlite3BitvecBuiltinTest(sz, aProg) };
                break '__slate_break_1792;
                //  sqlite3_test_control(FAULT_INSTALL, xCallback)
                //
                // Arrange to invoke xCallback() whenever sqlite3FaultSim() is called,
                // if xCallback is not NULL.
                //
                // As a test of the fault simulator mechanism itself, sqlite3FaultSim(0)
                // is called immediately after installing the new callback and the return
                // value from sqlite3FaultSim(0) becomes the return from
                // sqlite3_test_control().
            }
            9 => {
                // A bug in MSVC prevents it from understanding pointers to functions
                // types in the second argument to va_arg().  Work around the problem
                // using a typedef.
                // http://support.microsoft.com/kb/47961  <-- dead hyperlink
                // Search at http://web.archive.org/ to find the 2015-03-16 archive
                // of the link above to see the original text.
                // sqlite3GlobalConfig.xTestCallback = va_arg(ap, int(*)(int));
                unsafe {
                    sqlite3Config.xTestCallback = unsafe {
                        unsafe {
                            std::mem::transmute::<
                                *const u8,
                                Option<unsafe extern "C-unwind" fn(i32) -> i32>,
                            >(ap.next_arg::<*const u8>())
                        }
                    };
                }
                rc = unsafe { sqlite3FaultSim(0 as i32) };
                break '__slate_break_1792;
                //  sqlite3_test_control(BENIGN_MALLOC_HOOKS, xBegin, xEnd)
                //
                // Register hooks to call to indicate which malloc() failures
                // are benign.
            }
            10 => {
                let mut xBenignBegin: Option<unsafe extern "C-unwind" fn()> =
                    unsafe { std::mem::zeroed() };
                let mut xBenignEnd: Option<unsafe extern "C-unwind" fn()> =
                    unsafe { std::mem::zeroed() };
                xBenignBegin = unsafe {
                    unsafe {
                        std::mem::transmute::<*const u8, Option<unsafe extern "C-unwind" fn()>>(
                            ap.next_arg::<*const u8>(),
                        )
                    }
                };
                xBenignEnd = unsafe {
                    unsafe {
                        std::mem::transmute::<*const u8, Option<unsafe extern "C-unwind" fn()>>(
                            ap.next_arg::<*const u8>(),
                        )
                    }
                };
                unsafe { sqlite3BenignMallocHooks(xBenignBegin, xBenignEnd) };
                break '__slate_break_1792;
                //  sqlite3_test_control(SQLITE_TESTCTRL_PENDING_BYTE, unsigned int X)
                //
                // Set the PENDING byte to the value in the argument, if X>0.
                // Make no changes if X==0.  Return the value of the pending byte
                // as it existing before this routine was called.
                //
                // IMPORTANT:  Changing the PENDING byte from 0x40000000 results in
                // an incompatible database file format.  Changing the PENDING byte
                // while any database connection is open results in undefined and
                // deleterious behavior.
            }
            11 => {
                rc = unsafe { sqlite3PendingByte };
                let mut newVal: u32 = unsafe { ap.next_arg::<u32>() };
                if newVal != (0 as u32) {
                    unsafe {
                        sqlite3PendingByte = newVal as i32;
                    }
                }
                break '__slate_break_1792;
                //  sqlite3_test_control(SQLITE_TESTCTRL_ASSERT, int X)
                //
                // This action provides a run-time test to see whether or not
                // assert() was enabled at compile-time.  If X is true and assert()
                // is enabled, then the return value is true.  If X is true and
                // assert() is disabled, then the return value is zero.  If X is
                // false and assert() is enabled, then the assertion fires and the
                // process aborts.  If X is false and assert() is disabled, then the
                // return value is zero.
            }
            12 => {
                let mut x: i32 = 0 as i32;
                0 as i32; // side-effects-ok
                rc = unsafe { std::ptr::read_volatile(std::ptr::addr_of!(x)) };
                break '__slate_break_1792;
                //  sqlite3_test_control(SQLITE_TESTCTRL_ALWAYS, int X)
                //
                // This action provides a run-time test to see how the ALWAYS and
                // NEVER macros were defined at compile-time.
                //
                // The return value is ALWAYS(X) if X is true, or 0 if X is false.
                //
                // The recommended test is X==2.  If the return value is 2, that means
                // ALWAYS() and NEVER() are both no-op pass-through macros, which is the
                // default setting.  If the return value is 1, then ALWAYS() is either
                // hard-coded to true or else it asserts if its argument is false.
                // The first behavior (hard-coded to true) is the case if
                // SQLITE_TESTCTRL_ASSERT shows that assert() is disabled and the second
                // behavior (assert if the argument to ALWAYS() is false) is the case if
                // SQLITE_TESTCTRL_ASSERT shows that assert() is enabled.
                //
                // The run-time test procedure might look something like this:
                //
                //    if( sqlite3_test_control(SQLITE_TESTCTRL_ALWAYS, 2)==2 ){
                //      // ALWAYS() and NEVER() are no-op pass-through macros
                //    }else if( sqlite3_test_control(SQLITE_TESTCTRL_ASSERT, 1) ){
                //      // ALWAYS(x) asserts that x is true. NEVER(x) asserts x is false.
                //    }else{
                //      // ALWAYS(x) is a constant 1.  NEVER(x) is a constant 0.
                //    }
            }
            13 => {
                let mut x: i32 = unsafe { ap.next_arg::<i32>() };
                rc = if x != (0 as i32) { x } else { 0 as i32 };
                break '__slate_break_1792;
                //   sqlite3_test_control(SQLITE_TESTCTRL_BYTEORDER);
                //
                // The integer returned reveals the byte-order of the computer on which
                // SQLite is running:
                //
                //       1     big-endian,    determined at run-time
                //      10     little-endian, determined at run-time
                //  432101     big-endian,    determined at compile-time
                //  123410     little-endian, determined at compile-time
            }
            22 => {
                rc = (1234 as i32) * (100 as i32) + (1 as i32) * (10 as i32) + (0 as i32);
                break '__slate_break_1792;
                //  sqlite3_test_control(SQLITE_TESTCTRL_OPTIMIZATIONS, sqlite3 *db, int N)
                //
                // Enable or disable various optimizations for testing purposes.  The
                // argument N is a bitmask of optimizations to be disabled.  For normal
                // operation N should be 0.  The idea is that a test program (like the
                // SQL Logic Test or SLT test module) can run the same SQL multiple times
                // with various optimizations disabled to verify that the same answer
                // is obtained in every case.
            }
            15 => {
                let mut db: *mut sqlite3 = unsafe { ap.next_arg::<*mut sqlite3>() };
                unsafe {
                    (*db).dbOptFlags = unsafe { ap.next_arg::<u32>() };
                }
                break '__slate_break_1792;
                //  sqlite3_test_control(SQLITE_TESTCTRL_GETOPT, sqlite3 *db, int *N)
                //
                // Write the current optimization settings into *N.  A zero bit means that
                // the optimization is on, and a 1 bit means that the optimization is off.
            }
            16 => {
                let mut db: *mut sqlite3 = unsafe { ap.next_arg::<*mut sqlite3>() };
                let mut pN: *mut i32 = unsafe { ap.next_arg::<*mut i32>() };
                unsafe {
                    *pN = (unsafe { (*db).dbOptFlags }) as i32;
                }
                break '__slate_break_1792;
                //   sqlite3_test_control(SQLITE_TESTCTRL_LOCALTIME_FAULT, onoff, xAlt);
                //
                // If parameter onoff is 1, subsequent calls to localtime() fail.
                // If 2, then invoke xAlt() instead of localtime().  If 0, normal
                // processing.
                //
                // xAlt arguments are void pointers, but they really want to be:
                //
                //    int xAlt(const time_t*, struct tm*);
                //
                // xAlt should write results in to struct tm object of its 2nd argument
                // and return zero on success, or return non-zero on failure.
            }
            18 => {
                unsafe {
                    sqlite3Config.bLocaltimeFault = unsafe { ap.next_arg::<i32>() };
                }
                if (unsafe { sqlite3Config.bLocaltimeFault }) == (2 as i32) {
                    unsafe {
                        sqlite3Config.xAltLocaltime = unsafe {
                            unsafe {
                                std::mem::transmute::<
                                    *const u8,
                                    Option<unsafe extern "C-unwind" fn(*const (), *mut ()) -> i32>,
                                >(ap.next_arg::<*const u8>())
                            }
                        };
                    }
                } else {
                    unsafe {
                        sqlite3Config.xAltLocaltime = None;
                    }
                }
                break '__slate_break_1792;
                //   sqlite3_test_control(SQLITE_TESTCTRL_INTERNAL_FUNCTIONS, sqlite3*);
                //
                // Toggle the ability to use internal functions on or off for
                // the database connection given in the argument.
            }
            17 => {
                let mut db: *mut sqlite3 = unsafe { ap.next_arg::<*mut sqlite3>() };
                let __v1875: *mut sqlite3 = db;
                let __v1876: u32 = unsafe { (*__v1875).mDbFlags };
                let __v1877: u32 = __v1876 ^ ((32 as i32) as u32);
                unsafe {
                    (*__v1875).mDbFlags = __v1877;
                }
                break '__slate_break_1792;
                //   sqlite3_test_control(SQLITE_TESTCTRL_NEVER_CORRUPT, int);
                //
                // Set or clear a flag that indicates that the database file is always well-
                // formed and never corrupt.  This flag is clear by default, indicating that
                // database files might have arbitrary corruption.  Setting the flag during
                // testing causes certain assert() statements in the code to be activated
                // that demonstrate invariants on well-formed database files.
            }
            20 => {
                unsafe {
                    sqlite3Config.neverCorrupt = unsafe { ap.next_arg::<i32>() };
                }
                break '__slate_break_1792;
                //   sqlite3_test_control(SQLITE_TESTCTRL_EXTRA_SCHEMA_CHECKS, int);
                //
                // Set or clear a flag that causes SQLite to verify that type, name,
                // and tbl_name fields of the sqlite_schema table.  This is normally
                // on, but it is sometimes useful to turn it off for testing.
                //
                // 2020-07-22:  Disabling EXTRA_SCHEMA_CHECKS also disables the
                // verification of rootpage numbers when parsing the schema.  This
                // is useful to make it easier to reach strange internal error states
                // during testing.  The EXTRA_SCHEMA_CHECKS setting is always enabled
                // in production.
            }
            29 => {
                unsafe {
                    sqlite3Config.bExtraSchemaChecks =
                        ((unsafe { ap.next_arg::<i32>() }) as i8) as u8;
                }
                break '__slate_break_1792;
                // Set the threshold at which OP_Once counters reset back to zero.
                // By default this is 0x7ffffffe (over 2 billion), but that value is
                // too big to test in a reasonable amount of time, so this control is
                // provided to set a small and easily reachable reset value.
            }
            19 => {
                unsafe {
                    sqlite3Config.iOnceResetThreshold = unsafe { ap.next_arg::<i32>() };
                }
                break '__slate_break_1792;
                //   sqlite3_test_control(SQLITE_TESTCTRL_VDBE_COVERAGE, xCallback, ptr);
                //
                // Set the VDBE coverage callback function to xCallback with context
                // pointer ptr.
            }
            21 => {
                break '__slate_break_1792;
                //   sqlite3_test_control(SQLITE_TESTCTRL_SORTER_MMAP, db, nMax);
            }
            24 => {
                let mut db: *mut sqlite3 = unsafe { ap.next_arg::<*mut sqlite3>() };
                unsafe {
                    (*db).nMaxSorterMmap = unsafe { ap.next_arg::<i32>() };
                }
                break '__slate_break_1792;
                //   sqlite3_test_control(SQLITE_TESTCTRL_ISINIT);
                //
                // Return SQLITE_OK if SQLite has been initialized and SQLITE_ERROR if
                // not.
            }
            23 => {
                if (unsafe { sqlite3Config.isInit }) == (0 as i32) {
                    rc = 1 as i32;
                }
                break '__slate_break_1792;
                //  sqlite3_test_control(SQLITE_TESTCTRL_IMPOSTER, db, dbName, mode, tnum);
                //
                // This test control is used to create imposter tables.  "db" is a pointer
                // to the database connection.  dbName is the database name (ex: "main" or
                // "temp") which will receive the imposter.  "mode" turns imposter mode on
                // or off.  mode==0 means imposter mode is off.  mode==1 means imposter mode
                // is on.  mode==2 means imposter mode is on but results in an imposter
                // table that is read-only unless writable_schema is on.  "tnum" is the
                // root page of the b-tree to which the imposter table should connect.
                //
                // Enable imposter mode only when the schema has already been parsed.  Then
                // run a single CREATE TABLE statement to construct the imposter table in
                // the parsed schema.  Then turn imposter mode back off again.
                //
                // If onOff==0 and tnum>0 then reset the schema for all databases, causing
                // the schema to be reparsed the next time it is needed.  This has the
                // effect of erasing all imposter tables.
            }
            25 => {
                let mut db: *mut sqlite3 = unsafe { ap.next_arg::<*mut sqlite3>() };
                let mut iDb: i32 = 0 as i32;
                unsafe { sqlite3_mutex_enter(unsafe { (*db).mutex }) };
                iDb = unsafe { sqlite3FindDbName(db, unsafe { ap.next_arg::<*const i8>() }) };
                if iDb >= (0 as i32) {
                    unsafe {
                        (*db).init.iDb = (iDb as i8) as u8;
                    }
                    let __v1878: u32 = (unsafe { ap.next_arg::<i32>() }) as u32;
                    unsafe {
                        (*db).init.__slate_bits_0.__set_imposterTable(__v1878);
                    }
                    unsafe {
                        (*db).init.busy =
                            (((((__v1878 as u8) & (3 as u8)) as u32) as i32) as i8) as u8;
                    }
                    unsafe {
                        (*db).init.newTnum = (unsafe { ap.next_arg::<i32>() }) as u32;
                    }
                    if (((unsafe { (*db).init.busy }) as u32) as i32) == (0 as i32)
                        && (unsafe { (*db).init.newTnum }) > ((0 as i32) as u32)
                    {
                        unsafe { sqlite3ResetAllSchemasOfConnection(db) };
                    }
                }
                unsafe { sqlite3_mutex_leave(unsafe { (*db).mutex }) };
                break '__slate_break_1792;
                //  sqlite3_test_control(SQLITE_TESTCTRL_RESULT_INTREAL, sqlite3_context*);
                //
                // This test-control causes the most recent sqlite3_result_int64() value
                // to be interpreted as a MEM_IntReal instead of as an MEM_Int.  Normally,
                // MEM_IntReal values only arise during an INSERT operation of integer
                // values into a REAL column, so they can be challenging to test.  This
                // test-control enables us to write an intreal() SQL function that can
                // inject an intreal() value at arbitrary places in an SQL statement,
                // for testing purposes.
            }
            27 => {
                let mut pCtx: *mut sqlite3_context =
                    unsafe { ap.next_arg::<*mut sqlite3_context>() };
                unsafe { sqlite3ResultIntReal(pCtx) };
                break '__slate_break_1792;
                //  sqlite3_test_control(SQLITE_TESTCTRL_SEEK_COUNT,
                //    sqlite3 *db,    // Database connection
                //    u64 *pnSeek     // Write seek count here
                //  );
                //
                // This test-control queries the seek-counter on the "main" database
                // file.  The seek-counter is written into *pnSeek and is then reset.
                // The seek-count is only available if compiled with SQLITE_DEBUG.
            }
            30 => {
                let mut db: *mut sqlite3 = unsafe { ap.next_arg::<*mut sqlite3>() };
                let mut pn: *mut u64 = unsafe { ap.next_arg::<*mut u64>() };
                unsafe {
                    *pn = ((0 as i32) as i64) as u64;
                }
                db; // Silence harmless unused variable warning
                break '__slate_break_1792;
                //  sqlite3_test_control(SQLITE_TESTCTRL_TRACEFLAGS, op, ptr)
                //
                // "ptr" is a pointer to a u32.
                //
                //  op==0       Store the current sqlite3TreeTrace in *ptr
                //  op==1       Set sqlite3TreeTrace to the value *ptr
                //  op==2       Store the current sqlite3WhereTrace in *ptr
                //  op==3       Set sqlite3WhereTrace to the value *ptr
            }
            31 => {
                let mut opTrace: i32 = unsafe { ap.next_arg::<i32>() };
                let mut ptr: *mut u32 = unsafe { ap.next_arg::<*mut u32>() };
                match opTrace {
                    0 => unsafe {
                        *ptr = unsafe { sqlite3TreeTrace };
                    },
                    1 => unsafe {
                        sqlite3TreeTrace = unsafe { *ptr };
                    },
                    2 => unsafe {
                        *ptr = unsafe { sqlite3WhereTrace };
                    },
                    3 => unsafe {
                        sqlite3WhereTrace = unsafe { *ptr };
                    },
                    _ => {}
                }
                break '__slate_break_1792;
                // sqlite3_test_control(SQLITE_TESTCTRL_LOGEST,
                //      double fIn,     // Input value
                //      int *pLogEst,   // sqlite3LogEstFromDouble(fIn)
                //      u64 *pInt,      // sqlite3LogEstToInt(*pLogEst)
                //      int *pLogEst2   // sqlite3LogEst(*pInt)
                // );
                //
                // Test access for the LogEst conversion routines.
            }
            33 => {
                let mut rIn: f64 = unsafe { ap.next_arg::<f64>() };
                let mut rLogEst: i16 = unsafe { sqlite3LogEstFromDouble(rIn) };
                let mut pI1: *mut i32 = unsafe { ap.next_arg::<*mut i32>() };
                let mut pU64: *mut u64 = unsafe { ap.next_arg::<*mut u64>() };
                let mut pI2: *mut i32 = unsafe { ap.next_arg::<*mut i32>() };
                unsafe {
                    *pI1 = rLogEst as i32;
                }
                unsafe {
                    *pU64 = unsafe { sqlite3LogEstToInt(rLogEst) };
                }
                unsafe {
                    *pI2 = (unsafe { sqlite3LogEst(unsafe { *pU64 }) }) as i32;
                }
                break '__slate_break_1792;
                // sqlite3_test_control(SQLITE_TESTCTRL_ATOF, const char *z, double *p);
                //
                // Test access to the sqlite3AtoF() routine.
            }
            34 => {
                let mut z: *const i8 = unsafe { ap.next_arg::<*const i8>() };
                let mut pR: *mut f64 = unsafe { ap.next_arg::<*mut f64>() };
                rc = unsafe { sqlite3AtoF(z, pR) };
                break '__slate_break_1792;
                // sqlite3_test_control(SQLITE_TESTCTRL_JSON_SELFCHECK, &onOff);
                //
                // Activate or deactivate validation of JSONB that is generated from
                // text.  Off by default, as the validation is slow.  Validation is
                // only available if compiled using SQLITE_DEBUG.
                //
                // If onOff is initially 1, then turn it on.  If onOff is initially
                // off, turn it off.  If onOff is initially -1, then change onOff
                // to be the current setting.
            }
            14 => {}
            _ => {}
        }
    }
    {}
    return rc;
}

/// The Pager stores the Database filename, Journal filename, and WAL filename
/// consecutively in memory, in that order.  The database filename is prefixed
/// by four zero bytes.  Locate the start of the database filename by searching
/// backwards for the first byte following four consecutive zero bytes.
///
/// This only works if the filename passed in was obtained from the Pager.
fn databaseName(mut zName: *const i8) -> *const i8 {
    '__slate_break_1794: while ((unsafe { *unsafe { zName.offset(-(1 as i32) as isize) } }) as i32)
        != (0 as i32)
        || ((unsafe { *unsafe { zName.offset(-(2 as i32) as isize) } }) as i32) != (0 as i32)
        || ((unsafe { *unsafe { zName.offset(-(3 as i32) as isize) } }) as i32) != (0 as i32)
        || ((unsafe { *unsafe { zName.offset(-(4 as i32) as isize) } }) as i32) != (0 as i32)
    {
        let __v1984: *const i8 = zName;
        let __v1985: *const i8 = unsafe { __v1984.offset(-((1 as i32) as isize)) };
        zName = __v1985;
    }
    return zName;
}

/// Append text z[] to the end of p[].  Return a pointer to the first
/// character after then zero terminator on the new text in p[].
fn appendText(mut p: *mut i8, mut z: *const i8) -> *mut i8 {
    let mut n: u64 = unsafe { strlen(z) };
    unsafe {
        memcpy(
            p as *mut (),
            z as *const (),
            n.wrapping_add(((1 as i32) as i64) as u64),
        )
    };
    return unsafe { unsafe { p.offset(n as isize) }.offset((1 as i32) as isize) };
}

/// Allocate memory to hold names for a database, journal file, WAL file,
/// and query parameters.  The pointer returned is valid for use by
/// sqlite3_filename_database() and sqlite3_uri_parameter() and related
/// functions.
///
/// Memory layout must be compatible with that generated by the pager
/// and expected by sqlite3_uri_parameter() and databaseName().
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.main.sqlite3_create_filename")]
extern "C-unwind" fn sqlite3_create_filename(
    mut zDatabase: *const i8,
    mut zJournal: *const i8,
    mut zWal: *const i8,
    mut nParam: i32,
    mut azParam: *mut *const i8,
) -> *const i8 {
    let mut nByte: i64 = 0 as i64;
    let mut i: i32 = 0 as i32;
    let mut pResult: *mut i8 = unsafe { std::mem::zeroed() };
    let mut p: *mut i8 = unsafe { std::mem::zeroed() };
    nByte = unsafe { strlen(zDatabase) }
        .wrapping_add(unsafe { strlen(zJournal) })
        .wrapping_add(unsafe { strlen(zWal) })
        .wrapping_add(((10 as i32) as i64) as u64) as i64;
    i = 0 as i32;
    '__slate_break_1795: loop {
        if !(i < nParam * (2 as i32)) {
            break;
        }
        let __v1841: i64 = nByte;
        let __v1842: i64 = (__v1841 as u64).wrapping_add(
            unsafe { strlen(unsafe { *unsafe { azParam.offset(i as isize) } }) }
                .wrapping_add(((1 as i32) as i64) as u64),
        ) as i64;
        nByte = __v1842;
        let __v1839: i32 = i;
        let __v1840: i32 = __v1839 + (1 as i32);
        i = __v1840;
    }
    let __v1843: *mut i8 = (unsafe { sqlite3_malloc64(nByte as u64) }) as *mut i8;
    p = __v1843;
    pResult = __v1843;
    if p == std::ptr::null_mut::<i8>() {
        return std::ptr::null::<i8>();
    }
    unsafe { memset(p as *mut (), 0 as i32, ((4 as i32) as i64) as u64) };
    let __v1844: *mut i8 = p;
    let __v1845: *mut i8 = unsafe { __v1844.offset((4 as i32) as isize) };
    p = __v1845;
    p = appendText(p, zDatabase);
    i = 0 as i32;
    '__slate_break_1796: loop {
        if !(i < nParam * (2 as i32)) {
            break;
        }
        p = appendText(p, unsafe { *unsafe { azParam.offset(i as isize) } });
        let __v1846: i32 = i;
        let __v1847: i32 = __v1846 + (1 as i32);
        i = __v1847;
    }
    let __v1848: *mut i8 = p;
    let __v1849: *mut i8 = unsafe { __v1848.offset((1 as i32) as isize) };
    p = __v1849;
    unsafe {
        *__v1848 = (0 as i32) as i8;
    }
    p = appendText(p, zJournal);
    p = appendText(p, zWal);
    let __v1850: *mut i8 = p;
    let __v1851: *mut i8 = unsafe { __v1850.offset((1 as i32) as isize) };
    p = __v1851;
    unsafe {
        *__v1850 = (0 as i32) as i8;
    }
    let __v1852: *mut i8 = p;
    let __v1853: *mut i8 = unsafe { __v1852.offset((1 as i32) as isize) };
    p = __v1853;
    unsafe {
        *__v1852 = (0 as i32) as i8;
    }
    0 as i32;
    return (unsafe { pResult.offset((4 as i32) as isize) }) as *const i8;
}

/// Free memory obtained from sqlite3_create_filename().  It is a severe
/// error to call this routine with any parameter other than a pointer
/// previously obtained from sqlite3_create_filename() or a NULL pointer.
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.main.sqlite3_free_filename")]
extern "C-unwind" fn sqlite3_free_filename(mut p: *const i8) {
    if p == std::ptr::null::<i8>() {
        return;
    }
    p = databaseName(p);
    unsafe { sqlite3_free((unsafe { (p as *mut i8).offset(-((4 as i32) as isize)) }) as *mut ()) };
}

/// This is a utility routine, useful to VFS implementations, that checks
/// to see if a database file was a URI that contained a specific query
/// parameter, and if so obtains the value of the query parameter.
///
/// The zFilename argument is the filename pointer passed into the xOpen()
/// method of a VFS implementation.  The zParam argument is the name of the
/// query parameter we seek.  This routine returns the value of the zParam
/// parameter if it exists.  If the parameter does not exist, this routine
/// returns a NULL pointer.
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.main.sqlite3_uri_parameter")]
extern "C-unwind" fn sqlite3_uri_parameter(
    mut zFilename: *const i8,
    mut zParam: *const i8,
) -> *const i8 {
    if zFilename == std::ptr::null::<i8>() || zParam == std::ptr::null::<i8>() {
        return std::ptr::null::<i8>();
    }
    zFilename = databaseName(zFilename);
    return uriParameter(zFilename, zParam);
}

/// Return a pointer to the name of Nth query parameter of the filename.
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.main.sqlite3_uri_key")]
extern "C-unwind" fn sqlite3_uri_key(mut zFilename: *const i8, mut N: i32) -> *const i8 {
    if zFilename == std::ptr::null::<i8>() || N < (0 as i32) {
        return std::ptr::null::<i8>();
    }
    zFilename = databaseName(zFilename);
    let __v1822: *const i8 = zFilename;
    let __v1823: *const i8 =
        unsafe { __v1822.offset(((unsafe { sqlite3Strlen30(zFilename) }) + (1 as i32)) as isize) };
    zFilename = __v1823;
    '__slate_break_1797: loop {
        let __v1824: bool;
        if zFilename != std::ptr::null::<i8>()
            && (unsafe { *unsafe { zFilename.offset((0 as i32) as isize) } }) != (0 as i8)
        {
            let __v1825: i32 = N;
            let __v1826: i32 = __v1825 - (1 as i32);
            N = __v1826;
            __v1824 = __v1825 > (0 as i32);
        } else {
            __v1824 = false as bool;
        }
        if !__v1824 {
            break;
        }
        let __v1827: *const i8 = zFilename;
        let __v1828: *const i8 = unsafe {
            __v1827.offset(((unsafe { sqlite3Strlen30(zFilename) }) + (1 as i32)) as isize)
        };
        zFilename = __v1828;
        let __v1829: *const i8 = zFilename;
        let __v1830: *const i8 = unsafe {
            __v1829.offset(((unsafe { sqlite3Strlen30(zFilename) }) + (1 as i32)) as isize)
        };
        zFilename = __v1830;
    }
    return if (unsafe { *unsafe { zFilename.offset((0 as i32) as isize) } }) != (0 as i8) {
        zFilename
    } else {
        std::ptr::null::<i8>()
    };
}

/// Return a boolean value for a query parameter.
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.main.sqlite3_uri_boolean")]
extern "C-unwind" fn sqlite3_uri_boolean(
    mut zFilename: *const i8,
    mut zParam: *const i8,
    mut bDflt: i32,
) -> i32 {
    let mut z: *const i8 = sqlite3_uri_parameter(zFilename, zParam);
    bDflt = (bDflt != (0 as i32)) as i32;
    let __v1820: i32;
    if z != std::ptr::null::<i8>() {
        __v1820 = ((unsafe { sqlite3GetBoolean(z, (bDflt as i8) as u8) }) as u32) as i32;
    } else {
        __v1820 = bDflt;
    }
    return __v1820;
}

/// Return a 64-bit integer value for a query parameter.
///
/// # Arguments
///
/// * `zFilename` - Filename as passed to xOpen
/// * `zParam` - URI parameter sought
/// * `bDflt` - return if parameter is missing
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.main.sqlite3_uri_int64")]
extern "C-unwind" fn sqlite3_uri_int64(
    mut zFilename: *const i8,
    mut zParam: *const i8,
    mut bDflt: i64,
) -> i64 {
    let mut z: *const i8 = sqlite3_uri_parameter(zFilename, zParam);
    let mut v: i64 = 0 as i64;
    let __v1821: bool;
    if z != std::ptr::null::<i8>() {
        __v1821 = (unsafe { sqlite3DecOrHexToI64(z, std::ptr::addr_of_mut!(v)) }) == (0 as i32);
    } else {
        __v1821 = false as bool;
    }
    if __v1821 {
        bDflt = v;
    }
    return bDflt;
}

/// Translate a filename that was handed to a VFS routine into the corresponding
/// database, journal, or WAL file.
///
/// It is an error to pass this routine a filename string that was not
/// passed into the VFS from the SQLite core.  Doing so is similar to
/// passing free() a pointer that was not obtained from malloc() - it is
/// an error that we cannot easily detect but that will likely cause memory
/// corruption.
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.main.sqlite3_filename_database")]
extern "C-unwind" fn sqlite3_filename_database(mut zFilename: *const i8) -> *const i8 {
    if zFilename == std::ptr::null::<i8>() {
        return std::ptr::null::<i8>();
    }
    return databaseName(zFilename);
}

#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.main.sqlite3_filename_journal")]
extern "C-unwind" fn sqlite3_filename_journal(mut zFilename: *const i8) -> *const i8 {
    if zFilename == std::ptr::null::<i8>() {
        return std::ptr::null::<i8>();
    }
    zFilename = databaseName(zFilename);
    let __v1831: *const i8 = zFilename;
    let __v1832: *const i8 =
        unsafe { __v1831.offset(((unsafe { sqlite3Strlen30(zFilename) }) + (1 as i32)) as isize) };
    zFilename = __v1832;
    '__slate_break_1798: while zFilename != std::ptr::null::<i8>()
        && (unsafe { *unsafe { zFilename.offset((0 as i32) as isize) } }) != (0 as i8)
    {
        let __v1833: *const i8 = zFilename;
        let __v1834: *const i8 = unsafe {
            __v1833.offset(((unsafe { sqlite3Strlen30(zFilename) }) + (1 as i32)) as isize)
        };
        zFilename = __v1834;
        let __v1835: *const i8 = zFilename;
        let __v1836: *const i8 = unsafe {
            __v1835.offset(((unsafe { sqlite3Strlen30(zFilename) }) + (1 as i32)) as isize)
        };
        zFilename = __v1836;
    }
    return unsafe { zFilename.offset((1 as i32) as isize) };
}

#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.main.sqlite3_filename_wal")]
extern "C-unwind" fn sqlite3_filename_wal(mut zFilename: *const i8) -> *const i8 {
    zFilename = sqlite3_filename_journal(zFilename);
    if zFilename != std::ptr::null::<i8>() {
        let __v1837: *const i8 = zFilename;
        let __v1838: *const i8 = unsafe {
            __v1837.offset(((unsafe { sqlite3Strlen30(zFilename) }) + (1 as i32)) as isize)
        };
        zFilename = __v1838;
    }
    return zFilename;
}

/// Return the Btree pointer identified by zDbName.  Return NULL if not found.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3DbNameToBtree(
    mut db: *mut sqlite3,
    mut zDbName: *const i8,
) -> *mut Btree {
    let mut iDb: i32 = 0 as i32;
    let __v1918: i32;
    if zDbName != std::ptr::null::<i8>() {
        __v1918 = unsafe { sqlite3FindDbName(db, zDbName) };
    } else {
        __v1918 = 0 as i32;
    }
    iDb = __v1918;
    return if iDb < (0 as i32) {
        std::ptr::null_mut::<Btree>()
    } else {
        unsafe { (*unsafe { unsafe { (*db).aDb }.offset(iDb as isize) }).pBt }
    };
}

/// Return the name of the N-th database schema.  Return NULL if N is out
/// of range.
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.main.sqlite3_db_name")]
extern "C-unwind" fn sqlite3_db_name(mut db: *mut sqlite3, mut N: i32) -> *const i8 {
    let mut zRet: *const i8 = std::ptr::null::<i8>();
    unsafe { sqlite3_mutex_enter(unsafe { (*db).mutex }) };
    if N >= (0 as i32) && N < unsafe { (*db).nDb } {
        zRet = (unsafe { (*unsafe { unsafe { (*db).aDb }.offset(N as isize) }).zDbSName })
            as *const i8;
    }
    unsafe { sqlite3_mutex_leave(unsafe { (*db).mutex }) };
    return zRet;
}

/// Return the filename of the database associated with a database
/// connection.
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.main.sqlite3_db_filename")]
extern "C-unwind" fn sqlite3_db_filename(
    mut db: *mut sqlite3,
    mut zDbName: *const i8,
) -> *const i8 {
    let mut pBt: *mut Btree = unsafe { std::mem::zeroed() };
    pBt = sqlite3DbNameToBtree(db, zDbName);
    let __v1856: *const i8;
    if pBt != std::ptr::null_mut::<Btree>() {
        __v1856 = unsafe { sqlite3BtreeGetFilename(pBt) };
    } else {
        __v1856 = std::ptr::null::<i8>();
    }
    return __v1856;
}

/// Return 1 if database is read-only or 0 if read/write.  Return -1 if
/// no such database exists.
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.main.sqlite3_db_readonly")]
extern "C-unwind" fn sqlite3_db_readonly(mut db: *mut sqlite3, mut zDbName: *const i8) -> i32 {
    let mut pBt: *mut Btree = unsafe { std::mem::zeroed() };
    pBt = sqlite3DbNameToBtree(db, zDbName);
    let __v1857: i32;
    if pBt != std::ptr::null_mut::<Btree>() {
        __v1857 = unsafe { sqlite3BtreeIsReadonly(pBt) };
    } else {
        __v1857 = -(1 as i32);
    }
    return __v1857;
}

/// Given the name of a compile-time option, return true if that option
/// was used and false if not.
///
/// The name can optionally begin with "SQLITE_" but the "SQLITE_" prefix
/// is not required for a match.
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.main.sqlite3_compileoption_used")]
extern "C-unwind" fn sqlite3_compileoption_used(mut zOptName: *const i8) -> i32 {
    let mut i: i32 = 0 as i32;
    let mut n: i32 = 0 as i32;
    let mut nOpt: i32 = 0 as i32;
    let mut azCompileOpt: *mut *const i8 = unsafe { std::mem::zeroed() };
    azCompileOpt = unsafe { sqlite3CompileOptions(std::ptr::addr_of_mut!(nOpt)) };
    if (unsafe {
        sqlite3_strnicmp(
            zOptName,
            (b"SQLITE_\0".as_ptr() as *mut i8) as *const i8,
            7 as i32,
        )
    }) == (0 as i32)
    {
        let __v1801: *const i8 = zOptName;
        let __v1802: *const i8 = unsafe { __v1801.offset((7 as i32) as isize) };
        zOptName = __v1802;
    }
    n = unsafe { sqlite3Strlen30(zOptName) };
    // Since nOpt is normally in single digits, a linear search is
    // adequate. No need for a binary search.
    i = 0 as i32;
    '__slate_break_1800: loop {
        if !(i < nOpt) {
            break;
        }
        let __v1805: bool;
        if (unsafe {
            sqlite3_strnicmp(
                zOptName,
                unsafe { *unsafe { azCompileOpt.offset(i as isize) } },
                n,
            )
        }) == (0 as i32)
        {
            __v1805 = (unsafe {
                sqlite3IsIdChar(
                    (unsafe {
                        *unsafe {
                            unsafe { *unsafe { azCompileOpt.offset(i as isize) } }
                                .offset(n as isize)
                        }
                    }) as u8,
                )
            }) == (0 as i32);
        } else {
            __v1805 = false as bool;
        }
        if __v1805 {
            return 1 as i32;
        }
        let __v1803: i32 = i;
        let __v1804: i32 = __v1803 + (1 as i32);
        i = __v1804;
    }
    return 0 as i32;
}

/// Return the N-th compile-time option string.  If N is out of range,
/// return a NULL pointer.
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.main.sqlite3_compileoption_get")]
extern "C-unwind" fn sqlite3_compileoption_get(mut N: i32) -> *const i8 {
    let mut nOpt: i32 = 0 as i32;
    let mut azCompileOpt: *mut *const i8 = unsafe { std::mem::zeroed() };
    azCompileOpt = unsafe { sqlite3CompileOptions(std::ptr::addr_of_mut!(nOpt)) };
    if N >= (0 as i32) && N < nOpt {
        return unsafe { *unsafe { azCompileOpt.offset(N as isize) } };
    }
    return std::ptr::null::<i8>();
}
