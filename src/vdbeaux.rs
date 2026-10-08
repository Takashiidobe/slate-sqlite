//! 2003 September 6
//!
//! The author disclaims copyright to this source code.  In place of
//! a legal notice, here is a blessing:
//!
//!    May you do good and not evil.
//!    May you find forgiveness for yourself and forgive others.
//!    May you share freely, never taking more than you give.
//!
//!
//! This file contains code used for creating, destroying, and populating
//! a VDBE (or an "sqlite3_stmt" as it is known to the outside world.)
unsafe extern "C" {
    static mut sqlite3OpcodeProperty: [u8; 0];
    fn sqlite3_mprintf(__v1217: *const i8, ...) -> *mut i8;
    fn sqlite3_snprintf(__v1218: i32, __v1219: *mut i8, __v1220: *const i8, ...) -> *mut i8;
    fn sqlite3_free(__v1221: *mut ());
    fn sqlite3_randomness(N: i32, P: *mut ());
    fn sqlite3_result_error(__v1224: *mut sqlite3_context, __v1225: *const i8, __v1226: i32);
    fn sqlite3_str_appendf(__v1227: *mut sqlite3_str, zFormat: *const i8, ...);
    fn sqlite3_str_append(__v1229: *mut sqlite3_str, zIn: *const i8, N: i32);
    fn sqlite3_str_appendall(__v1232: *mut sqlite3_str, zIn: *const i8);
    fn sqlite3_str_appendchar(__v1234: *mut sqlite3_str, N: i32, C: i8);
    fn sqlite3_log(iErrCode: i32, zFormat: *const i8, ...);
    fn memcpy(__dest: *mut (), __src: *const (), __n: u64) -> *mut ();
    fn memset(__s: *mut (), __c: i32, __n: u64) -> *mut ();
    fn memcmp(__s1: *const (), __s2: *const (), __n: u64) -> i32;
    fn strcmp(__s1: *const i8, __s2: *const i8) -> i32;
    fn strncmp(__s1: *const i8, __s2: *const i8, __n: u64) -> i32;
    fn sqlite3OsWrite(__v1253: *mut sqlite3_file, __v1254: *const (), amt: i32, offset: i64)
    -> i32;
    fn sqlite3OsSync(__v1257: *mut sqlite3_file, __v1258: i32) -> i32;
    fn sqlite3OsDeviceCharacteristics(id: *mut sqlite3_file) -> i32;
    fn sqlite3OsDelete(__v1260: *mut sqlite3_vfs, __v1261: *const i8, __v1262: i32) -> i32;
    fn sqlite3OsAccess(
        __v1263: *mut sqlite3_vfs,
        __v1264: *const i8,
        __v1265: i32,
        pResOut: *mut i32,
    ) -> i32;
    fn sqlite3OsOpenMalloc(
        __v1267: *mut sqlite3_vfs,
        __v1268: *const i8,
        __v1269: *mut *mut sqlite3_file,
        __v1270: i32,
        __v1271: *mut i32,
    ) -> i32;
    fn sqlite3OsCloseFree(__v1272: *mut sqlite3_file);
    fn sqlite3PagerGetJournalMode(__v1273: *mut Pager) -> i32;
    fn sqlite3PagerExclusiveLock(__v1274: *mut Pager) -> i32;
    fn sqlite3PagerIsMemdb(__v1275: *mut Pager) -> i32;
    fn sqlite3BtreeCommitPhaseOne(__v1276: *mut Btree, __v1277: *const i8) -> i32;
    fn sqlite3BtreeCommitPhaseTwo(__v1278: *mut Btree, __v1279: i32) -> i32;
    fn sqlite3BtreeTxnState(__v1280: *mut Btree) -> i32;
    fn sqlite3BtreeSavepoint(__v1281: *mut Btree, __v1282: i32, __v1283: i32) -> i32;
    fn sqlite3BtreeGetFilename(__v1284: *mut Btree) -> *const i8;
    fn sqlite3BtreeGetJournalname(__v1285: *mut Btree) -> *const i8;
    fn sqlite3BtreeCloseCursor(__v1286: *mut BtCursor) -> i32;
    fn sqlite3BtreeTableMoveto(
        __v1287: *mut BtCursor,
        intKey: i64,
        bias: i32,
        pRes: *mut i32,
    ) -> i32;
    fn sqlite3BtreeCursorHasMoved(__v1291: *mut BtCursor) -> i32;
    fn sqlite3BtreeCursorRestore(__v1292: *mut BtCursor, __v1293: *mut i32) -> i32;
    fn sqlite3BtreeFirst(__v1294: *mut BtCursor, pRes: *mut i32) -> i32;
    fn sqlite3BtreeNext(__v1296: *mut BtCursor, flags: i32) -> i32;
    fn sqlite3BtreeEof(__v1298: *mut BtCursor) -> i32;
    fn sqlite3BtreePrevious(__v1299: *mut BtCursor, flags: i32) -> i32;
    fn sqlite3BtreePayload(__v1301: *mut BtCursor, offset: u32, amt: u32, __v1304: *mut ()) -> i32;
    fn sqlite3BtreePayloadSize(__v1305: *mut BtCursor) -> u32;
    fn sqlite3BtreePager(__v1306: *mut Btree) -> *mut Pager;
    fn sqlite3BtreeCursorIsValidNN(__v1307: *mut BtCursor) -> i32;
    fn sqlite3BtreeEnter(__v1308: *mut Btree);
    fn sqlite3BtreeSharable(__v1309: *mut Btree) -> i32;
    fn sqlite3BtreeLeave(__v1310: *mut Btree);
    fn sqlite3CorruptError(__v1478: i32) -> i32;
    fn sqlite3Strlen30(__v1479: *const i8) -> i32;
    fn sqlite3MallocZero(__v1480: u64) -> *mut ();
    fn sqlite3DbMallocRaw(__v1481: *mut sqlite3, __v1482: u64) -> *mut ();
    fn sqlite3DbMallocRawNN(__v1483: *mut sqlite3, __v1484: u64) -> *mut ();
    fn sqlite3DbStrDup(__v1485: *mut sqlite3, __v1486: *const i8) -> *mut i8;
    fn sqlite3DbStrNDup(__v1487: *mut sqlite3, __v1488: *const i8, __v1489: u64) -> *mut i8;
    fn sqlite3DbReallocOrFree(__v1490: *mut sqlite3, __v1491: *mut (), __v1492: u64) -> *mut ();
    fn sqlite3DbRealloc(__v1493: *mut sqlite3, __v1494: *mut (), __v1495: u64) -> *mut ();
    fn sqlite3DbFree(__v1496: *mut sqlite3, __v1497: *mut ());
    fn sqlite3DbNNFreeNN(__v1498: *mut sqlite3, __v1499: *mut ());
    fn sqlite3DbMallocSize(__v1500: *mut sqlite3, __v1501: *const ()) -> i32;
    fn sqlite3IsNaN(__v1502: f64) -> i32;
    fn sqlite3MPrintf(__v1503: *mut sqlite3, __v1504: *const i8, ...) -> *mut i8;
    fn sqlite3VMPrintf(
        __v1505: *mut sqlite3,
        __v1506: *const i8,
        __v1507: core::ffi::VaList<'_>,
    ) -> *mut i8;
    fn sqlite3ProgressCheck(__v1508: *mut Parse);
    fn sqlite3CommitInternalChanges(__v1509: *mut sqlite3);
    fn sqlite3DeleteTable(__v1510: *mut sqlite3, __v1511: *mut Table);
    fn sqlite3RollbackAll(__v1512: *mut sqlite3, __v1513: i32);
    fn sqlite3CloseSavepoints(__v1514: *mut sqlite3);
    fn sqlite3MayAbort(__v1515: *mut Parse);
    fn sqlite3GetVarint32(__v1516: *const u8, __v1517: *mut u32) -> u8;
    fn sqlite3VarintLen(v: u64) -> i32;
    fn sqlite3SystemError(__v1519: *mut sqlite3, __v1520: i32);
    fn sqlite3ErrStr(__v1521: i32) -> *const i8;
    fn sqlite3ValueText(__v1524: *mut sqlite3_value, __v1525: u8) -> *const ();
    fn sqlite3ValueSetStr(
        __v1526: *mut sqlite3_value,
        __v1527: i32,
        __v1528: *const (),
        __v1529: u8,
        __v1530: Option<unsafe extern "C-unwind" fn(*mut ())>,
    );
    fn sqlite3ValueSetNull(__v1531: *mut sqlite3_value);
    fn sqlite3ValueFree(__v1532: *mut sqlite3_value);
    fn sqlite3ValueNew(__v1533: *mut sqlite3) -> *mut sqlite3_value;
    fn sqlite3ValueApplyAffinity(__v1534: *mut sqlite3_value, __v1535: u8, __v1536: u8);
    fn sqlite3KeyInfoUnref(__v1539: *mut KeyInfo);
    fn sqlite3KeyInfoOfIndex(__v1540: *mut Parse, __v1541: *mut Index) -> *mut KeyInfo;
    fn sqlite3OomFault(__v1542: *mut sqlite3) -> *mut ();
    fn sqlite3RCStrUnref(__v1543: *mut ());
    fn sqlite3StrAccumInit(
        __v1544: *mut sqlite3_str,
        __v1545: *mut sqlite3,
        __v1546: *mut i8,
        __v1547: i32,
        __v1548: i32,
    );
    fn sqlite3StrAccumFinish(__v1549: *mut sqlite3_str) -> *mut i8;
    fn sqlite3VtabSync(db: *mut sqlite3, __v1551: *mut Vdbe) -> i32;
    fn sqlite3VtabCommit(db: *mut sqlite3) -> i32;
    fn sqlite3VtabLock(__v1553: *mut VTable);
    fn sqlite3VtabUnlock(__v1554: *mut VTable);
    fn sqlite3VtabSavepoint(__v1555: *mut sqlite3, __v1556: i32, __v1557: i32) -> i32;
    fn sqlite3BeginBenignMalloc();
    fn sqlite3EndBenignMalloc();
    fn sqlite3Get8byte(__v1560: *const u8) -> u64;
    fn sqlite3VdbeMemCopy(__v1599: *mut sqlite3_value, __v1600: *const sqlite3_value) -> i32;
    fn sqlite3VdbeMemShallowCopy(
        __v1601: *mut sqlite3_value,
        __v1602: *const sqlite3_value,
        __v1603: i32,
    );
    fn sqlite3VdbeMemSetStr(
        __v1604: *mut sqlite3_value,
        __v1605: *const i8,
        __v1606: i64,
        __v1607: u8,
        __v1608: Option<unsafe extern "C-unwind" fn(*mut ())>,
    ) -> i32;
    fn sqlite3VdbeMemSetText(
        __v1609: *mut sqlite3_value,
        __v1610: *const i8,
        __v1611: i64,
        __v1612: Option<unsafe extern "C-unwind" fn(*mut ())>,
    ) -> i32;
    fn sqlite3VdbeMemSetInt64(__v1613: *mut sqlite3_value, __v1614: i64);
    fn sqlite3VdbeMemInit(__v1615: *mut sqlite3_value, __v1616: *mut sqlite3, __v1617: u16);
    fn sqlite3VdbeMemSetNull(__v1618: *mut sqlite3_value);
    fn sqlite3VdbeMemFromBtreeZeroOffset(
        __v1621: *mut BtCursor,
        __v1622: u32,
        __v1623: *mut sqlite3_value,
    ) -> i32;
    fn sqlite3VdbeMemRelease(p: *mut sqlite3_value);
    fn sqlite3VdbeMemReleaseMalloc(p: *mut sqlite3_value);
    fn sqlite3OpcodeName(__v1626: i32) -> *const i8;
    fn sqlite3VdbeMemGrow(pMem: *mut sqlite3_value, n: i32, preserve: i32) -> i32;
    fn sqlite3VdbeSorterClose(__v1641: *mut sqlite3, __v1642: *mut VdbeCursor);
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
struct _ht {
    count: u32,
    chain: *mut HashElem,
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
    trace: __SlateRecord174,
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
    u1: __SlateRecord175,
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
    u: __SlateRecord176,
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
    __slate_bits_0: __slate_bits::__SlateBits71U0,
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
    u: __SlateRecord177,
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
    u: __SlateRecord182,
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
    __slate_bits_0: __slate_bits::__SlateBits95U0,
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
    u: __SlateRecord186,
    pLeft: *mut Expr,
    pRight: *mut Expr,
    x: __SlateRecord187,
    nHeight: i32,
    iTable: i32,
    iColumn: i16,
    iAgg: i16,
    w: __SlateRecord188,
    pAggInfo: *mut AggInfo,
    y: __SlateRecord189,
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
struct RenameToken {}

#[repr(C)]
#[derive(Clone, Copy)]
struct SrcItem {
    zName: *mut i8,
    zAlias: *mut i8,
    pSTab: *mut Table,
    fg: __SlateRecord196,
    iCursor: i32,
    colUsed: u64,
    u1: __SlateRecord197,
    u2: __SlateRecord198,
    u3: __SlateRecord199,
    u4: __SlateRecord200,
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
struct TableLock {}

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
    u1: __SlateRecord202,
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
struct VtabCtx {}

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
struct Pager {}

#[repr(C)]
#[derive(Clone, Copy)]
struct Btree {}

#[repr(C)]
#[derive(Clone, Copy)]
struct BtCursor {}

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
    __slate_bits_0: __slate_bits::__SlateBits213U0,
    seekHit: u16,
    ub: __SlateRecord215,
    seqCount: i64,
    cacheStatus: u32,
    seekResult: i32,
    pAltCursor: *mut VdbeCursor,
    uc: __SlateRecord216,
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
struct VdbeTxtBlbCache {
    pCValue: *mut i8,
    iOffset: i64,
    iCol: i32,
    cacheStatus: u32,
    colCacheCtr: u32,
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
struct sqlite3InitInfo {
    newTnum: u32,
    iDb: u8,
    busy: u8,
    __slate_bits_0: __slate_bits::__SlateBits173U0,
    azInit: *mut *const i8,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord174 {
    xLegacy: Option<unsafe extern "C-unwind" fn(*mut (), *const i8)>,
    xV2: Option<unsafe extern "C-unwind" fn(u32, *mut (), *mut (), *mut ()) -> i32>,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord175 {
    isInterrupted: i32,
    notUsed1: f64,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord176 {
    pHash: *mut FuncDef,
    pDestructor: *mut FuncDestructor,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord177 {
    tab: __SlateRecord178,
    view: __SlateRecord179,
    vtab: __SlateRecord180,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord178 {
    addColOffset: i32,
    pFKey: *mut FKey,
    pDfltList: *mut ExprList,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord179 {
    pSelect: *mut Select,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord180 {
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
union __SlateRecord182 {
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
union __SlateRecord186 {
    zToken: *mut i8,
    iValue: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord187 {
    pList: *mut ExprList,
    pSelect: *mut Select,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord188 {
    iJoin: i32,
    iOfst: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord189 {
    pTab: *mut Table,
    pWin: *mut Window,
    nReg: i32,
    sub: __SlateRecord190,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord190 {
    iAddr: i32,
    regReturn: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct ExprList_item {
    pExpr: *mut Expr,
    zEName: *mut i8,
    fg: __SlateRecord192,
    u: __SlateRecord193,
}

#[repr(C, align(4))]
#[derive(Clone, Copy)]
struct __SlateRecord192 {
    sortFlags: u8,
    __slate_bits_0: __slate_bits::__SlateBits192U0,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord193 {
    x: __SlateRecord194,
    iConstExprReg: i32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord194 {
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
struct __SlateRecord196 {
    jointype: u8,
    __slate_bits_0: __slate_bits::__SlateBits196U0,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord197 {
    zIndexedBy: *mut i8,
    pFuncArg: *mut ExprList,
    nRow: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord198 {
    pIBIndex: *mut Index,
    pCteUse: *mut CteUse,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord199 {
    pOn: *mut Expr,
    pUsing: *mut IdList,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord200 {
    pSchema: *mut Schema,
    zDatabase: *mut i8,
    pSubq: *mut Subquery,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord202 {
    cr: __SlateRecord203,
    d: __SlateRecord204,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord203 {
    addrCrTab: i32,
    regRowid: i32,
    regRoot: i32,
    constraintName: Token,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct __SlateRecord204 {
    pReturning: *mut Returning,
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
union __SlateRecord215 {
    pBtx: *mut Btree,
    aAltMap: *mut u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
union __SlateRecord216 {
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
    __slate_bits_0: __slate_bits::__SlateBits159U0,
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

/// Create a new virtual database engine.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3VdbeCreate(mut pParse: *mut Parse) -> *mut Vdbe {
    let mut db: *mut sqlite3 = unsafe { (*pParse).db };
    let mut p: *mut Vdbe = unsafe { std::mem::zeroed() };
    p = (unsafe { sqlite3DbMallocRawNN(db, 312 as u64) }) as *mut Vdbe;
    if p == std::ptr::null_mut::<Vdbe>() {
        return std::ptr::null_mut::<Vdbe>();
    }
    unsafe {
        memset(
            (unsafe { std::ptr::addr_of_mut!((*p).aOp) }) as *mut (),
            0 as i32,
            (312 as u64).wrapping_sub(136 as u64),
        )
    };
    unsafe {
        (*p).db = db;
    }
    if (unsafe { (*db).pVdbe }) != std::ptr::null_mut::<Vdbe>() {
        unsafe {
            (*unsafe { (*db).pVdbe }).ppVPrev = unsafe { std::ptr::addr_of_mut!((*p).pVNext) };
        }
    }
    unsafe {
        (*p).pVNext = unsafe { (*db).pVdbe };
    }
    unsafe {
        (*p).ppVPrev = unsafe { std::ptr::addr_of_mut!((*db).pVdbe) };
    }
    unsafe {
        (*db).pVdbe = p;
    }
    0 as i32;
    unsafe {
        (*p).pParse = pParse;
    }
    unsafe {
        (*pParse).pVdbe = p;
    }
    0 as i32;
    0 as i32;
    0 as i32;
    sqlite3VdbeAddOp2(p, 8 as i32, 0 as i32, 1 as i32);
    return p;
}

#[repr(C, align(16))]
struct __SlateAlign16<T>(T);

mod __slate_bits {
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
    pub struct __SlateBits71U0 {
        #[bits(4)]
        pub notNull: u32,
        #[bits(4)]
        pub eCType: u32,
    }
    #[bitfields::bitfield([u8; 2], c_names = true, new = false, from_into_bits = false, from_traits = false, default = false, debug = false, builder = false, bit_ops = false)]
    pub struct __SlateBits95U0 {
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
    pub struct __SlateBits196U0 {
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
    pub struct __SlateBits192U0 {
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
    pub struct __SlateBits213U0 {
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
    pub struct __SlateBits159U0 {
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
    pub struct __SlateBits173U0 {
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

/// Return the Parse object that owns a Vdbe object.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3VdbeParser(mut p: *mut Vdbe) -> *mut Parse {
    return unsafe { (*p).pParse };
}

/// Change the error string stored in Vdbe.zErrMsg
#[unsafe(no_mangle)]
unsafe extern "C-unwind" fn sqlite3VdbeError(
    mut p: *mut Vdbe,
    mut zFormat: *const i8,
    mut __va_args: ...
) {
    let mut ap: core::ffi::VaList<'_> = unsafe { std::mem::zeroed() };
    unsafe { sqlite3DbFree(unsafe { (*p).db }, (unsafe { (*p).zErrMsg }) as *mut ()) };
    ap = __va_args.clone();
    unsafe {
        (*p).zErrMsg = unsafe { sqlite3VMPrintf(unsafe { (*p).db }, zFormat, ap.clone()) };
    }
    {}
}

/// Remember the SQL string for a prepared statement.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3VdbeSetSql(
    mut p: *mut Vdbe,
    mut z: *const i8,
    mut n: i32,
    mut prepFlags: u8,
) {
    if p == std::ptr::null_mut::<Vdbe>() {
        return;
    }
    unsafe {
        (*p).prepFlags = prepFlags;
    }
    if ((prepFlags as u32) as i32) & (128 as i32) == (0 as i32) {
        unsafe {
            (*p).expmask = (0 as i32) as u32;
        }
    }
    0 as i32;
    unsafe {
        (*p).zSql = unsafe { sqlite3DbStrNDup(unsafe { (*p).db }, z, (n as i64) as u64) };
    }
}

/// Swap byte-code between two VDBE structures.
///
/// This happens after pB was previously run and returned
/// SQLITE_SCHEMA.  The statement was then reprepared in pA.
/// This routine transfers the new bytecode in pA over to pB
/// so that pB can be run again.  The old pB byte code is
/// moved back to pA so that it will be cleaned up when pA is
/// finalized.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3VdbeSwap(mut pA: *mut Vdbe, mut pB: *mut Vdbe) {
    let mut tmp: Vdbe = unsafe { std::mem::zeroed() };
    let mut pTmp: *mut Vdbe = unsafe { std::mem::zeroed() };
    let mut ppTmp: *mut *mut Vdbe = unsafe { std::mem::zeroed() };
    let mut zTmp: *mut i8 = unsafe { std::mem::zeroed() };
    0 as i32;
    tmp = unsafe { *pA };
    unsafe {
        *pA = unsafe { *pB };
    }
    unsafe {
        *pB = tmp;
    }
    pTmp = unsafe { (*pA).pVNext };
    unsafe {
        (*pA).pVNext = unsafe { (*pB).pVNext };
    }
    unsafe {
        (*pB).pVNext = pTmp;
    }
    ppTmp = unsafe { (*pA).ppVPrev };
    unsafe {
        (*pA).ppVPrev = unsafe { (*pB).ppVPrev };
    }
    unsafe {
        (*pB).ppVPrev = ppTmp;
    }
    zTmp = unsafe { (*pA).zSql };
    unsafe {
        (*pA).zSql = unsafe { (*pB).zSql };
    }
    unsafe {
        (*pB).zSql = zTmp;
    }
    let __v1800: *mut Vdbe = pB;
    let __v1801: u32 = unsafe { (*__v1800).expmask };
    let __v1802: u32 = __v1801 | unsafe { (*pA).expmask };
    unsafe {
        (*__v1800).expmask = __v1802;
    }
    let __v1803: *mut Vdbe = pB;
    let __v1804: u32 = unsafe { (*__v1803).smimask };
    let __v1805: u32 = __v1804 | unsafe { (*pA).smimask };
    unsafe {
        (*__v1803).smimask = __v1805;
    }
    unsafe {
        (*pB).prepFlags = unsafe { (*pA).prepFlags };
    }
    unsafe {
        memcpy(
            (unsafe { (*pB).aCounter.as_mut_ptr() as *mut u32 }) as *mut (),
            (unsafe { (*pA).aCounter.as_mut_ptr() as *mut u32 }) as *const (),
            36 as u64,
        )
    };
    let __v1806: *mut u32 =
        unsafe { unsafe { (*pB).aCounter.as_mut_ptr() as *mut u32 }.offset((5 as i32) as isize) };
    let __v1807: u32 = unsafe { *__v1806 };
    let __v1808: u32 = __v1807.wrapping_add((1 as i32) as u32);
    unsafe {
        *__v1806 = __v1808;
    }
}

/// Resize the Vdbe.aOp array so that it is at least nOp elements larger
/// than its current size. nOp is guaranteed to be less than or equal
/// to 1024/sizeof(Op).
///
/// If an out-of-memory error occurs while resizing the array, return
/// SQLITE_NOMEM. In this case Vdbe.aOp and Vdbe.nOpAlloc remain
/// unchanged (this is so that any opcodes already allocated can be
/// correctly deallocated along with the rest of the Vdbe).
fn growOpArray(mut v: *mut Vdbe, mut nOp: i32) -> i32 {
    let mut pNew: *mut VdbeOp = unsafe { std::mem::zeroed() };
    let mut p: *mut Parse = unsafe { (*v).pParse };
    // The SQLITE_TEST_REALLOC_STRESS compile-time option is designed to force
    // more frequent reallocs and hence provide more opportunities for
    // simulated OOM faults.  SQLITE_TEST_REALLOC_STRESS is generally used
    // during testing only.  With SQLITE_TEST_REALLOC_STRESS grow the op array
    // by the minimum* amount required until the size reaches 512.  Normal
    // operation (without SQLITE_TEST_REALLOC_STRESS) is to double the current
    // size of the op array or add 1KB of space, whichever is smaller.
    let mut nNew: i64 = if (unsafe { (*v).nOpAlloc }) != (0 as i32) {
        ((2 as i32) as i64) * ((unsafe { (*v).nOpAlloc }) as i64)
    } else {
        ((((1024 as i32) as i64) as u64) / (32 as u64)) as i64
    };
    nOp;
    // Ensure that the size of a VDBE does not grow too large
    if nNew
        > ((unsafe {
            *unsafe {
                unsafe { (*unsafe { (*p).db }).aLimit.as_mut_ptr() as *mut i32 }
                    .offset((5 as i32) as isize)
            }
        }) as i64)
    {
        unsafe { sqlite3OomFault(unsafe { (*p).db }) };
        return 7 as i32;
    }
    0 as i32;
    0 as i32;
    pNew = (unsafe {
        sqlite3DbRealloc(
            unsafe { (*p).db },
            (unsafe { (*v).aOp }) as *mut (),
            (nNew as u64).wrapping_mul(32 as u64),
        )
    }) as *mut VdbeOp;
    if pNew != std::ptr::null_mut::<VdbeOp>() {
        unsafe {
            (*v).nOpAlloc =
                (((((unsafe { sqlite3DbMallocSize(unsafe { (*p).db }, pNew as *const ()) }) as i64)
                    as u64)
                    / (32 as u64)) as u32) as i32;
        }
        unsafe {
            (*v).aOp = pNew;
        }
    }
    return if pNew != std::ptr::null_mut::<VdbeOp>() {
        0 as i32
    } else {
        7 as i32
    };
}

/// Slow paths for sqlite3VdbeAddOp3() and sqlite3VdbeAddOp4Int() for the
/// unusual case when we need to increase the size of the Vdbe.aOp[] array
/// before adding the new opcode.
fn growOp3(mut p: *mut Vdbe, mut op: i32, mut p1: i32, mut p2: i32, mut p3: i32) -> i32 {
    0 as i32;
    if growOpArray(p, 1 as i32) != (0 as i32) {
        return 1 as i32;
    }
    0 as i32;
    return sqlite3VdbeAddOp3(p, op, p1, p2, p3);
}

/// # Arguments
///
/// * `p` - Add the opcode to this VM
/// * `op` - The new opcode
/// * `p1` - The P1 operand
/// * `p2` - The P2 operand
/// * `p3` - The P3 operand
/// * `p4` - The P4 operand as an integer
fn addOp4IntSlow(
    mut p: *mut Vdbe,
    mut op: i32,
    mut p1: i32,
    mut p2: i32,
    mut p3: i32,
    mut p4: i32,
) -> i32 {
    let mut addr: i32 = sqlite3VdbeAddOp3(p, op, p1, p2, p3);
    if (((unsafe { (*unsafe { (*p).db }).mallocFailed }) as u32) as i32) == (0 as i32) {
        let mut pOp: *mut VdbeOp = unsafe { unsafe { (*p).aOp }.offset(addr as isize) };
        unsafe {
            (*pOp).p4type = -(3 as i32) as i8;
        }
        unsafe {
            (*pOp).p4.i = p4;
        }
    }
    return addr;
}

/// Add a new instruction to the list of instructions current in the
/// VDBE.  Return the address of the new instruction.
///
/// Parameters:
///
///    p               Pointer to the VDBE
///
///    op              The opcode for this instruction
///
///    p1, p2, p3, p4  Operands
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3VdbeAddOp0(mut p: *mut Vdbe, mut op: i32) -> i32 {
    return sqlite3VdbeAddOp3(p, op, 0 as i32, 0 as i32, 0 as i32);
}

#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3VdbeAddOp1(mut p: *mut Vdbe, mut op: i32, mut p1: i32) -> i32 {
    return sqlite3VdbeAddOp3(p, op, p1, 0 as i32, 0 as i32);
}

#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3VdbeAddOp2(
    mut p: *mut Vdbe,
    mut op: i32,
    mut p1: i32,
    mut p2: i32,
) -> i32 {
    return sqlite3VdbeAddOp3(p, op, p1, p2, 0 as i32);
}

#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3VdbeAddOp3(
    mut p: *mut Vdbe,
    mut op: i32,
    mut p1: i32,
    mut p2: i32,
    mut p3: i32,
) -> i32 {
    let mut i: i32 = 0 as i32;
    let mut pOp: *mut VdbeOp = unsafe { std::mem::zeroed() };
    i = unsafe { (*p).nOp };
    0 as i32;
    0 as i32;
    if (unsafe { (*p).nOpAlloc }) <= i {
        return growOp3(p, op, p1, p2, p3);
    }
    0 as i32;
    let __v1754: *mut Vdbe = p;
    let __v1755: i32 = unsafe { (*__v1754).nOp };
    let __v1756: i32 = __v1755 + (1 as i32);
    unsafe {
        (*__v1754).nOp = __v1756;
    }
    pOp = unsafe { unsafe { (*p).aOp }.offset(i as isize) };
    0 as i32;
    unsafe {
        (*pOp).opcode = (op as i8) as u8;
    }
    unsafe {
        (*pOp).p5 = ((0 as i32) as i16) as u16;
    }
    unsafe {
        (*pOp).p1 = p1;
    }
    unsafe {
        (*pOp).p2 = p2;
    }
    unsafe {
        (*pOp).p3 = p3;
    }
    unsafe {
        (*pOp).p4.p = std::ptr::null_mut::<()>();
    }
    unsafe {
        (*pOp).p4type = (0 as i32) as i8;
    }
    // Replicate this logic in sqlite3VdbeAddOp4Int()
    // vvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvv
    unsafe {
        (*pOp).zComment = std::ptr::null_mut::<i8>();
    }
    // ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    // Replicate in sqlite3VdbeAddOp4Int()
    return i;
}

/// # Arguments
///
/// * `p` - Add the opcode to this VM
/// * `op` - The new opcode
/// * `p1` - The P1 operand
/// * `p2` - The P2 operand
/// * `p3` - The P3 operand
/// * `p4` - The P4 operand as an integer
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3VdbeAddOp4Int(
    mut p: *mut Vdbe,
    mut op: i32,
    mut p1: i32,
    mut p2: i32,
    mut p3: i32,
    mut p4: i32,
) -> i32 {
    let mut i: i32 = 0 as i32;
    let mut pOp: *mut VdbeOp = unsafe { std::mem::zeroed() };
    i = unsafe { (*p).nOp };
    if (unsafe { (*p).nOpAlloc }) <= i {
        return addOp4IntSlow(p, op, p1, p2, p3, p4);
    }
    let __v1757: *mut Vdbe = p;
    let __v1758: i32 = unsafe { (*__v1757).nOp };
    let __v1759: i32 = __v1758 + (1 as i32);
    unsafe {
        (*__v1757).nOp = __v1759;
    }
    pOp = unsafe { unsafe { (*p).aOp }.offset(i as isize) };
    0 as i32;
    unsafe {
        (*pOp).opcode = (op as i8) as u8;
    }
    unsafe {
        (*pOp).p5 = ((0 as i32) as i16) as u16;
    }
    unsafe {
        (*pOp).p1 = p1;
    }
    unsafe {
        (*pOp).p2 = p2;
    }
    unsafe {
        (*pOp).p3 = p3;
    }
    unsafe {
        (*pOp).p4.i = p4;
    }
    unsafe {
        (*pOp).p4type = -(3 as i32) as i8;
    }
    // Replicate this logic in sqlite3VdbeAddOp3()
    // vvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvv
    unsafe {
        (*pOp).zComment = std::ptr::null_mut::<i8>();
    }
    // ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    // Replicate in sqlite3VdbeAddOp3()
    return i;
}

/// Generate an opcode that loads a 64-bit integer into register iDest
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3VdbeAddInt64(mut p: *mut Vdbe, mut iDest: i32, mut iVal: i64) -> i32 {
    return sqlite3VdbeAddOp3(
        p,
        74 as i32,
        (((iVal as u64) & ((4294967295 as u32) as u64)) as u32) as i32,
        iDest,
        (((iVal as u64) >> (32 as i32)) as u32) as i32,
    );
}

/// Generate an opcode that loads a 64-floating point value into register iDest.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3VdbeAddDouble(mut p: *mut Vdbe, mut iDest: i32, mut rVal: f64) -> i32 {
    let mut iVal: i64 = 0 as i64;
    unsafe {
        memcpy(
            std::ptr::addr_of_mut!(iVal) as *mut (),
            std::ptr::addr_of_mut!(rVal) as *const (),
            ((8 as i32) as i64) as u64,
        )
    };
    return sqlite3VdbeAddOp3(
        p,
        154 as i32,
        (((iVal as u64) & ((4294967295 as u32) as u64)) as u32) as i32,
        iDest,
        (((iVal as u64) >> (32 as i32)) as u32) as i32,
    );
}

/// Generate code for an unconditional jump to instruction iDest
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3VdbeGoto(mut p: *mut Vdbe, mut iDest: i32) -> i32 {
    return sqlite3VdbeAddOp3(p, 9 as i32, 0 as i32, iDest, 0 as i32);
}

/// Generate code to cause the string zStr to be loaded into
/// register iDest
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3VdbeLoadString(
    mut p: *mut Vdbe,
    mut iDest: i32,
    mut zStr: *const i8,
) -> i32 {
    return sqlite3VdbeAddOp4(p, 118 as i32, 0 as i32, iDest, 0 as i32, zStr, 0 as i32);
}

/// Generate code that initializes multiple registers to string or integer
/// constants.  The registers begin with iDest and increase consecutively.
/// One register is initialized for each characgter in zTypes[].  For each
/// "s" character in zTypes[], the register is a string if the argument is
/// not NULL, or OP_Null if the value is a null pointer.  For each "i" character
/// in zTypes[], the register is initialized to an integer.
///
/// If the input string does not end with "X" then an OP_ResultRow instruction
/// is generated for the values inserted.
#[unsafe(no_mangle)]
unsafe extern "C-unwind" fn sqlite3VdbeMultiLoad(
    mut p: *mut Vdbe,
    mut iDest: i32,
    mut zTypes: *const i8,
    mut __va_args: ...
) {
    let mut __slate_storage_1751: std::mem::MaybeUninit<i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1751: *mut i8 = std::ptr::addr_of_mut!(__slate_storage_1751) as *mut i8;
    let mut __slate_storage_1753: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1753: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1753) as *mut i32;
    let mut __slate_storage_1752: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1752: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1752) as *mut i32;
    let mut __slate_storage_661: std::mem::MaybeUninit<*const i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_661: *mut *const i8 =
        std::ptr::addr_of_mut!(__slate_storage_661) as *mut *const i8;
    let mut __slate_storage_660: std::mem::MaybeUninit<i8> = std::mem::MaybeUninit::uninit();
    let __slate_slot_660: *mut i8 = std::ptr::addr_of_mut!(__slate_storage_660) as *mut i8;
    let mut __slate_storage_659: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_659: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_659) as *mut i32;
    let mut __slate_storage_658: std::mem::MaybeUninit<core::ffi::VaList<'_>> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_658: *mut core::ffi::VaList<'_> =
        std::ptr::addr_of_mut!(__slate_storage_658) as *mut core::ffi::VaList<'_>;
    unsafe {
        *__slate_slot_658 = __va_args.clone();
        *__slate_slot_659 = 0 as i32;
        '__join_0: {
            loop {
                std::ptr::write(__slate_slot_1751, unsafe {
                    *unsafe { zTypes.offset(*__slate_slot_659 as isize) }
                });
                *__slate_slot_660 = *__slate_slot_1751;
                if (*__slate_slot_1751 as i32) != (0 as i32) {
                    if (*__slate_slot_660 as i32) == (115 as i32) {
                        std::ptr::write(__slate_slot_661, unsafe {
                            (*__slate_slot_658).next_arg::<*const i8>()
                        });
                        sqlite3VdbeAddOp4(
                            p,
                            if *__slate_slot_661 == std::ptr::null::<i8>() {
                                77 as i32
                            } else {
                                118 as i32
                            },
                            0 as i32,
                            iDest + *__slate_slot_659,
                            0 as i32,
                            *__slate_slot_661,
                            0 as i32,
                        );
                    } else {
                        if (*__slate_slot_660 as i32) == (105 as i32) {
                            sqlite3VdbeAddOp2(
                                p,
                                73 as i32,
                                unsafe { (*__slate_slot_658).next_arg::<i32>() },
                                iDest + *__slate_slot_659,
                            );
                        } else {
                            break '__join_0;
                        }
                    }
                    std::ptr::write(__slate_slot_1752, *__slate_slot_659);
                    std::ptr::write(__slate_slot_1753, *__slate_slot_1752 + (1 as i32));
                    *__slate_slot_659 = *__slate_slot_1753;
                } else {
                    break;
                }
            }
            sqlite3VdbeAddOp2(p, 86 as i32, iDest, *__slate_slot_659);
        }
        {}
    }
}

/// Add an opcode that includes the p4 value as a pointer.
///
/// # Arguments
///
/// * `p` - Add the opcode to this VM
/// * `op` - The new opcode
/// * `p1` - The P1 operand
/// * `p2` - The P2 operand
/// * `p3` - The P3 operand
/// * `zP4` - The P4 operand
/// * `p4type` - P4 operand type
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3VdbeAddOp4(
    mut p: *mut Vdbe,
    mut op: i32,
    mut p1: i32,
    mut p2: i32,
    mut p3: i32,
    mut zP4: *const i8,
    mut p4type: i32,
) -> i32 {
    let mut addr: i32 = sqlite3VdbeAddOp3(p, op, p1, p2, p3);
    sqlite3VdbeChangeP4(p, addr, zP4, p4type);
    return addr;
}

/// Add an OP_Function or OP_PureFunc opcode.
///
/// The eCallCtx argument is information (typically taken from Expr.op2)
/// that describes the calling context of the function.  0 means a general
/// function call.  NC_IsCheck means called by a check constraint,
/// NC_IdxExpr means called as part of an index expression.  NC_PartIdx
/// means in the WHERE clause of a partial index.  NC_GenCol means called
/// while computing a generated column value.  0 is the usual case.
///
/// # Arguments
///
/// * `pParse` - Parsing context
/// * `p1` - Constant argument mask
/// * `p2` - First argument register
/// * `p3` - Register into which results are written
/// * `nArg` - Number of argument
/// * `pFunc` - The function to be invoked
/// * `eCallCtx` - Calling context
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3VdbeAddFunctionCall(
    mut pParse: *mut Parse,
    mut p1: i32,
    mut p2: i32,
    mut p3: i32,
    mut nArg: i32,
    mut pFunc: *const FuncDef,
    mut eCallCtx: i32,
) -> i32 {
    let mut v: *mut Vdbe = unsafe { (*pParse).pVdbe };
    let mut addr: i32 = 0 as i32;
    let mut pCtx: *mut sqlite3_context = unsafe { std::mem::zeroed() };
    0 as i32;
    pCtx = (unsafe {
        sqlite3DbMallocRawNN(
            unsafe { (*pParse).db },
            (48 as u64).wrapping_add(((nArg as i64) as u64).wrapping_mul(8 as u64)),
        )
    }) as *mut sqlite3_context;
    if pCtx == std::ptr::null_mut::<sqlite3_context>() {
        0 as i32;
        freeEphemeralFunction(unsafe { (*pParse).db }, pFunc as *mut FuncDef);
        return 0 as i32;
    }
    unsafe {
        (*pCtx).pOut = std::ptr::null_mut::<sqlite3_value>();
    }
    unsafe {
        (*pCtx).pFunc = pFunc as *mut FuncDef;
    }
    unsafe {
        (*pCtx).pVdbe = std::ptr::null_mut::<Vdbe>();
    }
    unsafe {
        (*pCtx).isError = 0 as i32;
    }
    unsafe {
        (*pCtx).argc = (nArg as i16) as u16;
    }
    unsafe {
        (*pCtx).iOp = sqlite3VdbeCurrentAddr(v);
    }
    addr = sqlite3VdbeAddOp4(
        v,
        if eCallCtx != (0 as i32) {
            67 as i32
        } else {
            68 as i32
        },
        p1,
        p2,
        p3,
        (pCtx as *mut i8) as *const i8,
        -(14 as i32),
    );
    sqlite3VdbeChangeP5(v, ((eCallCtx & (46 as i32)) as i16) as u16);
    unsafe { sqlite3MayAbort(pParse) };
    return addr;
}

/// Return the address of the current EXPLAIN QUERY PLAN baseline.
/// 0 means "none".
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3VdbeExplainParent(mut pParse: *mut Parse) -> i32 {
    let mut pOp: *mut VdbeOp = unsafe { std::mem::zeroed() };
    if (unsafe { (*pParse).addrExplain }) == (0 as i32) {
        return 0 as i32;
    }
    pOp = sqlite3VdbeGetOp(unsafe { (*pParse).pVdbe }, unsafe { (*pParse).addrExplain });
    return unsafe { (*pOp).p2 };
}

/// Set a debugger breakpoint on the following routine in order to
/// monitor the EXPLAIN QUERY PLAN code generation.
/// Add a new OP_Explain opcode.
///
/// If the bPush flag is true, then make this opcode the parent for
/// subsequent Explains until sqlite3VdbeExplainPop() is called.
#[unsafe(no_mangle)]
unsafe extern "C-unwind" fn sqlite3VdbeExplain(
    mut pParse: *mut Parse,
    mut bPush: u8,
    mut zFmt: *const i8,
    mut __va_args: ...
) -> i32 {
    let mut addr: i32 = 0 as i32;
    // Always include the OP_Explain opcodes if SQLITE_DEBUG is defined.
    // But omit them (for performance) during production builds
    if (((unsafe { (*pParse).explain }) as u32) as i32) == (2 as i32) || (0 as i32) != (0 as i32) {
        let mut zMsg: *mut i8 = unsafe { std::mem::zeroed() };
        let mut v: *mut Vdbe = unsafe { std::mem::zeroed() };
        let mut ap: core::ffi::VaList<'_> = unsafe { std::mem::zeroed() };
        let mut iThis: i32 = 0 as i32;
        ap = __va_args.clone();
        zMsg = unsafe { sqlite3VMPrintf(unsafe { (*pParse).db }, zFmt, ap.clone()) };
        {}
        v = unsafe { (*pParse).pVdbe };
        iThis = unsafe { (*v).nOp };
        addr = sqlite3VdbeAddOp4(
            v,
            190 as i32,
            iThis,
            unsafe { (*pParse).addrExplain },
            0 as i32,
            zMsg as *const i8,
            -(7 as i32),
        );
        {}
        if bPush != (0 as u8) {
            unsafe {
                (*pParse).addrExplain = iThis;
            }
        }
        {}
    }
    return addr;
}

/// Pop the EXPLAIN QUERY PLAN stack one level.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3VdbeExplainPop(mut pParse: *mut Parse) {
    {}
    unsafe {
        (*pParse).addrExplain = sqlite3VdbeExplainParent(pParse);
    }
}

/// Add an OP_ParseSchema opcode.  This routine is broken out from
/// sqlite3VdbeAddOp4() since it needs to also needs to mark all btrees
/// as having been used.
///
/// zWhere is a WHERE clause that defines which entries of the schema
/// to reparse.  If zWhere==0, that means all entries.  p5 is a mask
/// of INITFLAG_* values for the parse.
///
/// In the current usage, the following are always true:
///
///     ALTER TABLE:     zWhere==0,  p5!=0
///     Otherwise:       zWhere!=0,  p5==0
///
/// The zWhere string must have been obtained from sqlite3DbMalloc().
/// This routine will take ownership of the allocated memory.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3VdbeAddParseSchemaOp(
    mut p: *mut Vdbe,
    mut iDb: i32,
    mut zWhere: *mut i8,
    mut p5: u16,
) {
    let mut j: i32 = 0 as i32;
    0 as i32;
    sqlite3VdbeAddOp4(
        p,
        151 as i32,
        iDb,
        0 as i32,
        0 as i32,
        zWhere as *const i8,
        -(7 as i32),
    );
    sqlite3VdbeChangeP5(p, p5);
    j = 0 as i32;
    '__slate_break_1653: loop {
        if !(j < unsafe { (*unsafe { (*p).db }).nDb }) {
            break;
        }
        sqlite3VdbeUsesBtree(p, j);
        let __v1774: i32 = j;
        let __v1775: i32 = __v1774 + (1 as i32);
        j = __v1775;
    }
    unsafe { sqlite3MayAbort(unsafe { (*p).pParse }) };
}

/// Insert the end of a co-routine
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3VdbeEndCoroutine(mut v: *mut Vdbe, mut regYield: i32) {
    sqlite3VdbeAddOp1(v, 70 as i32, regYield);
    // Clear the temporary register cache, thereby ensuring that each
    // co-routine has its own independent set of registers, because co-routines
    // might expect their registers to be preserved across an OP_Yield, and
    // that could cause problems if two or more co-routines are using the same
    // temporary register.
    unsafe {
        (*unsafe { (*v).pParse }).nTempReg = ((0 as i32) as i8) as u8;
    }
    unsafe {
        (*unsafe { (*v).pParse }).nRangeReg = 0 as i32;
    }
}

/// Create a new symbolic label for an instruction that has yet to be
/// coded.  The symbolic label is really just a negative number.  The
/// label can be used as the P2 value of an operation.  Later, when
/// the label is resolved to a specific address, the VDBE will scan
/// through its operation list and change all values of P2 which match
/// the label into the resolved address.
///
/// The VDBE knows that a P2 value is a label because labels are
/// always negative and P2 values are suppose to be non-negative.
/// Hence, a negative P2 value is a label that has yet to be resolved.
/// (Later:) This is only true for opcodes that have the OPFLG_JUMP
/// property.
///
/// Variable usage notes:
///
///     Parse.aLabel[x]     Stores the address that the x-th label resolves
///                         into.  For testing (SQLITE_DEBUG), unresolved
///                         labels stores -1, but that is not required.
///     Parse.nLabelAlloc   Number of slots allocated to Parse.aLabel[]
///     Parse.nLabel        The *negative* of the number of labels that have
///                         been issued.  The negative is stored because
///                         that gives a performance improvement over storing
///                         the equivalent positive value.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3VdbeMakeLabel(mut pParse: *mut Parse) -> i32 {
    let __v1789: *mut Parse = pParse;
    let __v1790: i32 = unsafe { (*__v1789).nLabel };
    let __v1791: i32 = __v1790 - (1 as i32);
    unsafe {
        (*__v1789).nLabel = __v1791;
    }
    return __v1791;
}

/// Resolve label "x" to be the address of the next instruction to
/// be inserted.  The parameter "x" must have been obtained from
/// a prior call to sqlite3VdbeMakeLabel().
fn resizeResolveLabel(mut p: *mut Parse, mut v: *mut Vdbe, mut j: i32) {
    let mut nNewSize: i32 = (25 as i32) - unsafe { (*p).nLabel };
    unsafe {
        (*p).aLabel = (unsafe {
            sqlite3DbReallocOrFree(
                unsafe { (*p).db },
                (unsafe { (*p).aLabel }) as *mut (),
                ((nNewSize as i64) as u64).wrapping_mul(4 as u64),
            )
        }) as *mut i32;
    }
    if (unsafe { (*p).aLabel }) == std::ptr::null_mut::<i32>() {
        unsafe {
            (*p).nLabelAlloc = 0 as i32;
        }
    } else {
        if nNewSize >= (100 as i32)
            && nNewSize / (100 as i32) > (unsafe { (*p).nLabelAlloc }) / (100 as i32)
        {
            unsafe { sqlite3ProgressCheck(p) };
        }
        unsafe {
            (*p).nLabelAlloc = nNewSize;
        }
        unsafe {
            *unsafe { unsafe { (*p).aLabel }.offset(j as isize) } = unsafe { (*v).nOp };
        }
    }
}

#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3VdbeResolveLabel(mut v: *mut Vdbe, mut x: i32) {
    let mut p: *mut Parse = unsafe { (*v).pParse };
    let mut j: i32 = !x;
    0 as i32;
    0 as i32;
    0 as i32;
    if (unsafe { (*p).nLabelAlloc }) + unsafe { (*p).nLabel } < (0 as i32) {
        resizeResolveLabel(p, v, j);
    } else {
        0 as i32; // Labels may only be resolved once
        unsafe {
            *unsafe { unsafe { (*p).aLabel }.offset(j as isize) } = unsafe { (*v).nOp };
        }
    }
}

/// Mark the VDBE as one that can only be run one time.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3VdbeRunOnlyOnce(mut p: *mut Vdbe) {
    sqlite3VdbeAddOp2(p, 168 as i32, 1 as i32, 1 as i32);
}

/// Mark the VDBE as one that can be run multiple times.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3VdbeReusable(mut p: *mut Vdbe) {
    let mut i: i32 = 0 as i32;
    i = 1 as i32;
    '__slate_break_1654: loop {
        if !(i < unsafe { (*p).nOp }) {
            break;
        }
        if (((unsafe { (*unsafe { unsafe { (*p).aOp }.offset(i as isize) }).opcode }) as u32)
            as i32)
            == (168 as i32)
        {
            unsafe {
                (*unsafe { unsafe { (*p).aOp }.offset((1 as i32) as isize) }).opcode =
                    ((189 as i32) as i8) as u8;
            }
            break '__slate_break_1654;
        }
        let __v1792: i32 = i;
        let __v1793: i32 = __v1792 + (1 as i32);
        i = __v1793;
    }
}

/// This routine is called after all opcodes have been inserted.  It loops
/// through all the opcodes and fixes up some details.
///
/// (1) For each jump instruction with a negative P2 value (a label)
///     resolve the P2 value to an actual address.
///
/// (2) Compute the maximum number of arguments used by the xUpdate/xFilter
///     methods of any virtual table and store that value in *pMaxVtabArgs.
///
/// (3) Update the Vdbe.readOnly and Vdbe.bIsReader flags to accurately
///     indicate what the prepared statement actually does.
///
/// (4) (discontinued)
///
/// (5) Reclaim the memory allocated for storing labels.
///
/// This routine will only function correctly if the mkopcodeh.tcl generator
/// script numbers the opcodes correctly.  Changes to this routine must be
/// coordinated with changes to mkopcodeh.tcl.
fn resolveP2Values(mut p: *mut Vdbe, mut pMaxVtabArgs: *mut i32) {
    let mut __slate_storage_1927: std::mem::MaybeUninit<*mut VdbeOp> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1927: *mut *mut VdbeOp =
        std::ptr::addr_of_mut!(__slate_storage_1927) as *mut *mut VdbeOp;
    let mut __slate_storage_1926: std::mem::MaybeUninit<*mut VdbeOp> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1926: *mut *mut VdbeOp =
        std::ptr::addr_of_mut!(__slate_storage_1926) as *mut *mut VdbeOp;
    let mut __slate_storage_719: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_719: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_719) as *mut i32;
    let mut __slate_storage_718: std::mem::MaybeUninit<*mut i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_718: *mut *mut i32 =
        std::ptr::addr_of_mut!(__slate_storage_718) as *mut *mut i32;
    let mut __slate_storage_717: std::mem::MaybeUninit<*mut Parse> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_717: *mut *mut Parse =
        std::ptr::addr_of_mut!(__slate_storage_717) as *mut *mut Parse;
    let mut __slate_storage_716: std::mem::MaybeUninit<*mut VdbeOp> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_716: *mut *mut VdbeOp =
        std::ptr::addr_of_mut!(__slate_storage_716) as *mut *mut VdbeOp;
    let mut __slate_storage_715: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_715: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_715) as *mut i32;
    unsafe {
        std::ptr::write(__slate_slot_715, unsafe { *pMaxVtabArgs });
        std::ptr::write(__slate_slot_717, unsafe { (*p).pParse });
        std::ptr::write(__slate_slot_718, unsafe { (*(*__slate_slot_717)).aLabel });
        0 as i32; // tag-20230419-1
        unsafe {
            (*p).__slate_bits_0.__set_readOnly((1 as i32) as u32);
        }
        unsafe {
            (*p).__slate_bits_0.__set_bIsReader((0 as i32) as u32);
        }
        *__slate_slot_716 =
            unsafe { unsafe { (*p).aOp }.offset(((unsafe { (*p).nOp }) - (1 as i32)) as isize) };
        0 as i32;
        '__join_2: {
            '__loop_3: loop {
                if (1 as i32) != (0 as i32) {
                    // Loop terminates when it reaches the OP_Init opcode
                    // Only JUMP opcodes and the short list of special opcodes in the switch
                    // below need to be considered.  The mkopcodeh.tcl generator script groups
                    // all these opcodes together near the front of the opcode list.  Skip
                    // any opcode that does not need processing by virtual of the fact that
                    // it is larger than SQLITE_MX_JUMP_OPCODE, as a performance optimization.
                    if (((unsafe { (*(*__slate_slot_716)).opcode }) as u32) as i32) <= (66 as i32) {
                        '__join_5: {
                            '__join_16: {
                                '__join_15: {
                                    // NOTE: Be sure to update mkopcodeh.tcl when adding or removing
                                    // cases from this switch!
                                    let __t0: i32 =
                                        ((unsafe { (*(*__slate_slot_716)).opcode }) as u32) as i32;
                                    if __t0 == (2 as i32) {
                                        '__join_17: {
                                            if (unsafe { (*(*__slate_slot_716)).p2 }) != (0 as i32)
                                            {
                                                unsafe {
                                                    (*p).__slate_bits_0
                                                        .__set_readOnly((0 as i32) as u32);
                                                }
                                            }
                                        }
                                        // no break
                                        {}
                                        break '__join_16;
                                    } else {
                                        if __t0 == (1 as i32) {
                                            break '__join_16;
                                        } else {
                                            if __t0 == (0 as i32) {
                                                break '__join_16;
                                            } else {
                                                if __t0 == (3 as i32) {
                                                    break '__join_15;
                                                } else {
                                                    if __t0 == (5 as i32) {
                                                        break '__join_15;
                                                    } else {
                                                        if __t0 == (4 as i32) {
                                                            break '__join_15;
                                                        } else {
                                                            if __t0 == (8 as i32) {
                                                                break '__loop_3;
                                                            } else {
                                                                if __t0 == (7 as i32) {
                                                                    if (unsafe {
                                                                        (*(*__slate_slot_716)).p2
                                                                    }) > *__slate_slot_715
                                                                    {
                                                                        *__slate_slot_715 = unsafe {
                                                                            (*(*__slate_slot_716))
                                                                                .p2
                                                                        };
                                                                        break '__join_5;
                                                                    } else {
                                                                        break '__join_5;
                                                                    }
                                                                } else {
                                                                    if __t0 == (6 as i32) {
                                                                        // The instruction immediately prior to VFilter will be an
                                                                        // OP_Integer that sets the "argc" value for the VFilter.  See
                                                                        // the code where OP_VFilter is generated at tag-20250207a.
                                                                        0 as i32;
                                                                        0 as i32;
                                                                        0 as i32;
                                                                        *__slate_slot_719 = unsafe {
                                                                            (*unsafe {
                                                                                (*__slate_slot_716)
                                                                                    .offset(
                                                                                    -(1 as i32)
                                                                                        as isize,
                                                                                )
                                                                            })
                                                                            .p1
                                                                        };
                                                                        if *__slate_slot_719
                                                                            > *__slate_slot_715
                                                                        {
                                                                            *__slate_slot_715 =
                                                                                *__slate_slot_719;
                                                                        }
                                                                        // Fall through into the default case
                                                                        //
                                                                        // no break
                                                                        {}
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    if (unsafe { (*(*__slate_slot_716)).p2 }) < (0 as i32) {
                                        // The mkopcodeh.tcl script has so arranged things that the only
                                        // non-jump opcodes less than SQLITE_MX_JUMP_CODE are guaranteed to
                                        // have non-negative values for P2.
                                        0 as i32;
                                        0 as i32;
                                        0 as i32; // True because of tag-20230419-1
                                        unsafe {
                                            (*(*__slate_slot_716)).p2 = unsafe {
                                                *unsafe {
                                                    (*__slate_slot_718).offset(!unsafe {
                                                        (*(*__slate_slot_716)).p2
                                                    }
                                                        as isize)
                                                }
                                            };
                                        }
                                    }
                                    // OPFLG_JUMP opcodes never have P2==0, though OPFLG_JUMP0 opcodes
                                    // might
                                    0 as i32;
                                    // Jumps never go off the end of the bytecode array
                                    0 as i32;
                                    break '__join_5;
                                }
                                unsafe {
                                    (*p).__slate_bits_0.__set_readOnly((0 as i32) as u32);
                                }
                                unsafe {
                                    (*p).__slate_bits_0.__set_bIsReader((1 as i32) as u32);
                                }
                                break '__join_5;
                            }
                            unsafe {
                                (*p).__slate_bits_0.__set_bIsReader((1 as i32) as u32);
                            }
                        }
                        // The mkopcodeh.tcl script has so arranged things that the only
                        // non-jump opcodes less than SQLITE_MX_JUMP_CODE are guaranteed to
                        // have non-negative values for P2.
                        0 as i32;
                    }
                    0 as i32;
                    std::ptr::write(__slate_slot_1926, *__slate_slot_716);
                    std::ptr::write(__slate_slot_1927, unsafe {
                        (*__slate_slot_1926).offset(-((1 as i32) as isize))
                    });
                    *__slate_slot_716 = *__slate_slot_1927;
                } else {
                    break '__join_2;
                }
            }
            0 as i32;
        }
        if *__slate_slot_718 != std::ptr::null_mut::<i32>() {
            unsafe {
                sqlite3DbNNFreeNN(
                    unsafe { (*p).db },
                    (unsafe { (*(*__slate_slot_717)).aLabel }) as *mut (),
                )
            };
            unsafe {
                (*(*__slate_slot_717)).aLabel = std::ptr::null_mut::<i32>();
            }
        }
        unsafe {
            (*(*__slate_slot_717)).nLabel = 0 as i32;
        }
        unsafe {
            *pMaxVtabArgs = *__slate_slot_715;
        }
        0 as i32;
    }
}

/// Return the address of the next instruction to be inserted.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3VdbeCurrentAddr(mut p: *mut Vdbe) -> i32 {
    0 as i32;
    return unsafe { (*p).nOp };
}

/// Verify that at least N opcode slots are available in p without
/// having to malloc for more space (except when compiled using
/// SQLITE_TEST_REALLOC_STRESS).  This interface is used during testing
/// to verify that certain calls to sqlite3VdbeAddOpList() can never
/// fail due to a OOM fault and hence that the return value from
/// sqlite3VdbeAddOpList() will always be non-NULL.
/// Verify that the VM passed as the only argument does not contain
/// an OP_ResultRow opcode. Fail an assert() if it does. This is used
/// by code in pragma.c to ensure that the implementation of certain
/// pragmas comports with the flags specified in the mkpragmatab.tcl
/// script.
/// Generate code (a single OP_Abortable opcode) that will
/// verify that the VDBE program can safely call Abort in the current
/// context.
/// This function returns a pointer to the array of opcodes associated with
/// the Vdbe passed as the first argument. It is the callers responsibility
/// to arrange for the returned array to be eventually freed using the
/// vdbeFreeOpArray() function.
///
/// Before returning, *pnOp is set to the number of entries in the returned
/// array. Also, *pnMaxArg is set to the larger of its current value and
/// the number of entries in the Vdbe.apArg[] array required to execute the
/// returned program.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3VdbeTakeOpArray(
    mut p: *mut Vdbe,
    mut pnOp: *mut i32,
    mut pnMaxArg: *mut i32,
) -> *mut VdbeOp {
    let mut aOp: *mut VdbeOp = unsafe { (*p).aOp };
    0 as i32;
    // Check that sqlite3VdbeUsesBtree() was not called on this VM
    0 as i32;
    resolveP2Values(p, pnMaxArg);
    unsafe {
        *pnOp = unsafe { (*p).nOp };
    }
    unsafe {
        (*p).aOp = std::ptr::null_mut::<VdbeOp>();
    }
    return aOp;
}

/// Add a whole list of operations to the operation stack.  Return a
/// pointer to the first operation inserted.
///
/// Non-zero P2 arguments to jump instructions are automatically adjusted
/// so that the jump target is relative to the first operation inserted.
///
/// # Arguments
///
/// * `p` - Add opcodes to the prepared statement
/// * `nOp` - Number of opcodes to add
/// * `aOp` - The opcodes to be added
/// * `iLineno` - Source-file line number of first opcode
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3VdbeAddOpList(
    mut p: *mut Vdbe,
    mut nOp: i32,
    mut aOp: *const VdbeOpList,
    mut iLineno: i32,
) -> *mut VdbeOp {
    let mut i: i32 = 0 as i32;
    let mut pOut: *mut VdbeOp = unsafe { std::mem::zeroed() };
    let mut pFirst: *mut VdbeOp = unsafe { std::mem::zeroed() };
    0 as i32;
    0 as i32;
    let __v1760: bool;
    if (unsafe { (*p).nOp }) + nOp > unsafe { (*p).nOpAlloc } {
        __v1760 = growOpArray(p, nOp) != (0 as i32);
    } else {
        __v1760 = false as bool;
    }
    if __v1760 {
        return std::ptr::null_mut::<VdbeOp>();
    }
    let __v1761: *mut VdbeOp =
        unsafe { unsafe { (*p).aOp }.offset((unsafe { (*p).nOp }) as isize) };
    pOut = __v1761;
    pFirst = __v1761;
    i = 0 as i32;
    '__slate_break_1657: while i < nOp {
        unsafe {
            (*pOut).opcode = unsafe { (*aOp).opcode };
        }
        unsafe {
            (*pOut).p1 = (unsafe { (*aOp).p1 }) as i32;
        }
        unsafe {
            (*pOut).p2 = (unsafe { (*aOp).p2 }) as i32;
        }
        0 as i32;
        if (((unsafe {
            *unsafe {
                unsafe { std::ptr::addr_of!(sqlite3OpcodeProperty) as *const u8 }
                    .offset((((unsafe { (*aOp).opcode }) as u32) as i32) as isize)
            }
        }) as u32) as i32)
            & (1 as i32)
            != (0 as i32)
            && ((unsafe { (*aOp).p2 }) as i32) > (0 as i32)
        {
            let __v1768: *mut VdbeOp = pOut;
            let __v1769: i32 = unsafe { (*__v1768).p2 };
            let __v1770: i32 = __v1769 + unsafe { (*p).nOp };
            unsafe {
                (*__v1768).p2 = __v1770;
            }
        }
        unsafe {
            (*pOut).p3 = (unsafe { (*aOp).p3 }) as i32;
        }
        unsafe {
            (*pOut).p4type = (0 as i32) as i8;
        }
        unsafe {
            (*pOut).p4.p = std::ptr::null_mut::<()>();
        }
        unsafe {
            (*pOut).p5 = ((0 as i32) as i16) as u16;
        }
        unsafe {
            (*pOut).zComment = std::ptr::null_mut::<i8>();
        }
        iLineno;
        let __v1762: i32 = i;
        let __v1763: i32 = __v1762 + (1 as i32);
        i = __v1763;
        let __v1764: *const VdbeOpList = aOp;
        let __v1765: *const VdbeOpList = unsafe { __v1764.offset((1 as i32) as isize) };
        aOp = __v1765;
        let __v1766: *mut VdbeOp = pOut;
        let __v1767: *mut VdbeOp = unsafe { __v1766.offset((1 as i32) as isize) };
        pOut = __v1767;
    }
    let __v1771: *mut Vdbe = p;
    let __v1772: i32 = unsafe { (*__v1771).nOp };
    let __v1773: i32 = __v1772 + nOp;
    unsafe {
        (*__v1771).nOp = __v1773;
    }
    return pFirst;
}

/// Change the value of the opcode, or P1, P2, P3, or P5 operands
/// for a specific instruction.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3VdbeChangeOpcode(mut p: *mut Vdbe, mut addr: i32, mut iNewOpcode: u8) {
    0 as i32;
    unsafe {
        (*sqlite3VdbeGetOp(p, addr)).opcode = iNewOpcode;
    }
}

#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3VdbeChangeP1(mut p: *mut Vdbe, mut addr: i32, mut val: i32) {
    0 as i32;
    unsafe {
        (*sqlite3VdbeGetOp(p, addr)).p1 = val;
    }
}

#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3VdbeChangeP2(mut p: *mut Vdbe, mut addr: i32, mut val: i32) {
    0 as i32;
    unsafe {
        (*sqlite3VdbeGetOp(p, addr)).p2 = val;
    }
}

#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3VdbeChangeP3(mut p: *mut Vdbe, mut addr: i32, mut val: i32) {
    0 as i32;
    unsafe {
        (*sqlite3VdbeGetOp(p, addr)).p3 = val;
    }
}

#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3VdbeChangeP5(mut p: *mut Vdbe, mut p5: u16) {
    0 as i32;
    if (unsafe { (*p).nOp }) > (0 as i32) {
        unsafe {
            (*unsafe {
                unsafe { (*p).aOp }.offset(((unsafe { (*p).nOp }) - (1 as i32)) as isize)
            })
            .p5 = p5;
        }
    }
}

/// If the previous opcode is an OP_Column that delivers results
/// into register iDest, then add the OPFLAG_TYPEOFARG flag to that
/// opcode.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3VdbeTypeofColumn(mut p: *mut Vdbe, mut iDest: i32) {
    let mut pOp: *mut VdbeOp = sqlite3VdbeGetLastOp(p);
    if (unsafe { (*pOp).p3 }) == iDest
        && (((unsafe { (*pOp).opcode }) as u32) as i32) == (96 as i32)
    {
        let __v1776: *mut VdbeOp = pOp;
        let __v1777: u16 = unsafe { (*__v1776).p5 };
        let __v1778: u16 = ((((__v1777 as u32) as i32) | (128 as i32)) as i16) as u16;
        unsafe {
            (*__v1776).p5 = __v1778;
        }
    }
}

/// Change the P2 operand of instruction addr so that it points to
/// the address of the next instruction to be coded.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3VdbeJumpHere(mut p: *mut Vdbe, mut addr: i32) {
    sqlite3VdbeChangeP2(p, addr, unsafe { (*p).nOp });
}

/// Change the P2 operand of the jump instruction at addr so that
/// the jump lands on the next opcode.  Or if the jump instruction was
/// the previous opcode (and is thus a no-op) then simply back up
/// the next instruction counter by one slot so that the jump is
/// overwritten by the next inserted opcode.
///
/// This routine is an optimization of sqlite3VdbeJumpHere() that
/// strives to omit useless byte-code like this:
///
///        7   Once 0 8 0
///        8   ...
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3VdbeJumpHereOrPopInst(mut p: *mut Vdbe, mut addr: i32) {
    if addr == (unsafe { (*p).nOp }) - (1 as i32) {
        0 as i32;
        0 as i32;
        let __v1779: *mut Vdbe = p;
        let __v1780: i32 = unsafe { (*__v1779).nOp };
        let __v1781: i32 = __v1780 - (1 as i32);
        unsafe {
            (*__v1779).nOp = __v1781;
        }
    } else {
        sqlite3VdbeChangeP2(p, addr, unsafe { (*p).nOp });
    }
}

/// If the input FuncDef structure is ephemeral, then free it.  If
/// the FuncDef is not ephemeral, then do nothing.
fn freeEphemeralFunction(mut db: *mut sqlite3, mut pDef: *mut FuncDef) {
    0 as i32;
    if (unsafe { (*pDef).funcFlags }) & ((16 as i32) as u32) != ((0 as i32) as u32) {
        unsafe { sqlite3DbNNFreeNN(db, pDef as *mut ()) };
    }
}

/// Delete a P4 value if necessary.
fn freeP4Mem(mut db: *mut sqlite3, mut p: *mut sqlite3_value) {
    if (unsafe { (*p).szMalloc }) != (0 as i32) {
        unsafe { sqlite3DbFree(db, (unsafe { (*p).zMalloc }) as *mut ()) };
    }
    unsafe { sqlite3DbNNFreeNN(db, p as *mut ()) };
}

fn freeP4FuncCtx(mut db: *mut sqlite3, mut p: *mut sqlite3_context) {
    0 as i32;
    freeEphemeralFunction(db, unsafe { (*p).pFunc });
    unsafe { sqlite3DbNNFreeNN(db, p as *mut ()) };
}

fn freeP4(mut db: *mut sqlite3, mut p4type: i32, mut p4: *mut ()) {
    0 as i32;
    '__slate_break_1658: {
        match p4type {
            -14 => {
                freeP4FuncCtx(db, p4 as *mut sqlite3_context);
            }
            -7 | -13 => {
                if p4 != std::ptr::null_mut::<()>() {
                    unsafe { sqlite3DbNNFreeNN(db, p4) };
                }
            }
            -9 => {
                if (unsafe { (*db).pnBytesFreed }) == std::ptr::null_mut::<i32>() {
                    unsafe { sqlite3KeyInfoUnref(p4 as *mut KeyInfo) };
                }
            }
            -8 => {
                freeEphemeralFunction(db, p4 as *mut FuncDef);
            }
            -11 => {
                if (unsafe { (*db).pnBytesFreed }) == std::ptr::null_mut::<i32>() {
                    unsafe { sqlite3ValueFree(p4 as *mut sqlite3_value) };
                } else {
                    freeP4Mem(db, p4 as *mut sqlite3_value);
                }
            }
            -12 => {
                if (unsafe { (*db).pnBytesFreed }) == std::ptr::null_mut::<i32>() {
                    unsafe { sqlite3VtabUnlock(p4 as *mut VTable) };
                }
            }
            -15 => {
                if (unsafe { (*db).pnBytesFreed }) == std::ptr::null_mut::<i32>() {
                    unsafe { sqlite3DeleteTable(db, p4 as *mut Table) };
                }
            }
            -16 => {
                let mut pSig: *mut SubrtnSig = p4 as *mut SubrtnSig;
                unsafe { sqlite3DbFree(db, (unsafe { (*pSig).zAff }) as *mut ()) };
                unsafe { sqlite3DbFree(db, pSig as *mut ()) };
            }
            _ => {}
        }
    }
}

/// Free the space allocated for aOp and any p4 values allocated for the
/// opcodes contained within. If aOp is not NULL it is assumed to contain
/// nOp entries.
fn vdbeFreeOpArray(mut db: *mut sqlite3, mut aOp: *mut VdbeOp, mut nOp: i32) {
    0 as i32;
    0 as i32;
    if aOp != std::ptr::null_mut::<VdbeOp>() {
        let mut pOp: *mut VdbeOp = unsafe { aOp.offset((nOp - (1 as i32)) as isize) };
        '__slate_break_1659: while (1 as i32) != (0 as i32) {
            // Exit via break
            if ((unsafe { (*pOp).p4type }) as i32) <= -(7 as i32) {
                freeP4(db, (unsafe { (*pOp).p4type }) as i32, unsafe {
                    (*pOp).p4.p
                });
            }
            unsafe { sqlite3DbFree(db, (unsafe { (*pOp).zComment }) as *mut ()) };
            if pOp == aOp {
                break '__slate_break_1659;
            }
            let __v1924: *mut VdbeOp = pOp;
            let __v1925: *mut VdbeOp = unsafe { __v1924.offset(-((1 as i32) as isize)) };
            pOp = __v1925;
        }
        unsafe { sqlite3DbNNFreeNN(db, aOp as *mut ()) };
    }
}

/// Link the SubProgram object passed as the second argument into the linked
/// list at Vdbe.pSubProgram. This list is used to delete all sub-program
/// objects when the VM is no longer required.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3VdbeLinkSubProgram(mut pVdbe: *mut Vdbe, mut p: *mut SubProgram) {
    unsafe {
        (*p).pNext = unsafe { (*pVdbe).pProgram };
    }
    unsafe {
        (*pVdbe).pProgram = p;
    }
}

/// Return true if the given Vdbe has any SubPrograms.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3VdbeHasSubProgram(mut pVdbe: *mut Vdbe) -> i32 {
    return ((unsafe { (*pVdbe).pProgram }) != std::ptr::null_mut::<SubProgram>()) as i32;
}

/// Change the opcode at addr into OP_Noop
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3VdbeChangeToNoop(mut p: *mut Vdbe, mut addr: i32) -> i32 {
    let mut pOp: *mut VdbeOp = unsafe { std::mem::zeroed() };
    if (unsafe { (*unsafe { (*p).db }).mallocFailed }) != (0 as u8) {
        return 0 as i32;
    }
    0 as i32;
    pOp = unsafe { unsafe { (*p).aOp }.offset(addr as isize) };
    freeP4(
        unsafe { (*p).db },
        (unsafe { (*pOp).p4type }) as i32,
        unsafe { (*pOp).p4.p },
    );
    unsafe {
        (*pOp).p4type = (0 as i32) as i8;
    }
    unsafe {
        (*pOp).p4.z = std::ptr::null_mut::<i8>();
    }
    unsafe {
        (*pOp).opcode = ((189 as i32) as i8) as u8;
    }
    return 1 as i32;
}

/// If the last opcode is "op" and it is not a jump destination,
/// then remove it.  Return true if and only if an opcode was removed.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3VdbeDeletePriorOpcode(mut p: *mut Vdbe, mut op: u8) -> i32 {
    if (unsafe { (*p).nOp }) > (0 as i32)
        && (((unsafe {
            (*unsafe { unsafe { (*p).aOp }.offset(((unsafe { (*p).nOp }) - (1 as i32)) as isize) })
                .opcode
        }) as u32) as i32)
            == ((op as u32) as i32)
    {
        return sqlite3VdbeChangeToNoop(p, (unsafe { (*p).nOp }) - (1 as i32));
    } else {
        return 0 as i32;
    }
    return unsafe { std::mem::zeroed() };
}

/// Change the value of the P4 operand for a specific instruction.
/// This routine is useful when a large program is loaded from a
/// static array using sqlite3VdbeAddOpList but we want to make a
/// few minor changes to the program.
///
/// If n>=0 then the P4 operand is dynamic, meaning that a copy of
/// the string is made into memory obtained from sqlite3_malloc().
/// A value of n==0 means copy bytes of zP4 up to and including the
/// first null byte.  If n>0 then copy n+1 bytes of zP4.
///
/// Other values of n (P4_STATIC, P4_COLLSEQ etc.) indicate that zP4 points
/// to a string or structure that is guaranteed to exist for the lifetime of
/// the Vdbe. In these cases we can just copy the pointer.
///
/// If addr<0 then change P4 on the most recently inserted instruction.
fn vdbeChangeP4Full(mut p: *mut Vdbe, mut pOp: *mut VdbeOp, mut zP4: *const i8, mut n: i32) {
    if (unsafe { (*pOp).p4type }) != (0 as i8) {
        0 as i32;
        unsafe {
            (*pOp).p4type = (0 as i32) as i8;
        }
        unsafe {
            (*pOp).p4.p = std::ptr::null_mut::<()>();
        }
    }
    if n < (0 as i32) {
        sqlite3VdbeChangeP4(
            p,
            ((unsafe { pOp.offset_from((unsafe { (*p).aOp }) as *mut VdbeOp) }) as i64) as i32,
            zP4,
            n,
        );
    } else {
        if n == (0 as i32) {
            n = unsafe { sqlite3Strlen30(zP4) };
        }
        unsafe {
            (*pOp).p4.z = unsafe { sqlite3DbStrNDup(unsafe { (*p).db }, zP4, (n as i64) as u64) };
        }
        unsafe {
            (*pOp).p4type = -(7 as i32) as i8;
        }
    }
}

#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3VdbeChangeP4(
    mut p: *mut Vdbe,
    mut addr: i32,
    mut zP4: *const i8,
    mut n: i32,
) {
    let mut pOp: *mut VdbeOp = unsafe { std::mem::zeroed() };
    let mut db: *mut sqlite3 = unsafe { std::mem::zeroed() };
    0 as i32;
    db = unsafe { (*p).db };
    0 as i32;
    0 as i32;
    if (unsafe { (*db).mallocFailed }) != (0 as u8) {
        if n != -(12 as i32) {
            freeP4(
                db,
                n,
                (unsafe { *(std::ptr::addr_of_mut!(zP4) as *mut *mut i8) }) as *mut (),
            );
        }
        return;
    }
    0 as i32;
    0 as i32;
    if addr < (0 as i32) {
        addr = (unsafe { (*p).nOp }) - (1 as i32);
    }
    pOp = unsafe { unsafe { (*p).aOp }.offset(addr as isize) };
    if n >= (0 as i32) || (unsafe { (*pOp).p4type }) != (0 as i8) {
        vdbeChangeP4Full(p, pOp, zP4, n);
        return;
    }
    if n == -(3 as i32) {
        // Note: this cast is safe, because the origin data point was an int
        // that was cast to a (const char *).
        unsafe {
            (*pOp).p4.i = (zP4 as i64) as i32;
        }
        unsafe {
            (*pOp).p4type = -(3 as i32) as i8;
        }
    } else {
        if zP4 != std::ptr::null::<i8>() {
            0 as i32;
            unsafe {
                (*pOp).p4.p = zP4 as *mut ();
            }
            unsafe {
                (*pOp).p4type = n as i8;
            }
            if n == -(12 as i32) {
                unsafe { sqlite3VtabLock(zP4 as *mut VTable) };
            }
        }
    }
}

/// Change the P4 operand of the most recently coded instruction
/// to the value defined by the arguments.  This is a high-speed
/// version of sqlite3VdbeChangeP4().
///
/// The P4 operand must not have been previously defined.  And the new
/// P4 must not be P4_INT32.  Use sqlite3VdbeChangeP4() in either of
/// those cases.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3VdbeAppendP4(mut p: *mut Vdbe, mut pP4: *mut (), mut n: i32) {
    let mut pOp: *mut VdbeOp = unsafe { std::mem::zeroed() };
    0 as i32;
    0 as i32;
    if (unsafe { (*unsafe { (*p).db }).mallocFailed }) != (0 as u8) {
        freeP4(unsafe { (*p).db }, n, pP4);
    } else {
        0 as i32;
        0 as i32;
        pOp = unsafe { unsafe { (*p).aOp }.offset(((unsafe { (*p).nOp }) - (1 as i32)) as isize) };
        0 as i32;
        unsafe {
            (*pOp).p4type = n as i8;
        }
        unsafe {
            (*pOp).p4.p = pP4;
        }
    }
}

/// Set the P4 on the most recently added opcode to the KeyInfo for the
/// index given.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3VdbeSetP4KeyInfo(mut pParse: *mut Parse, mut pIdx: *mut Index) {
    let mut v: *mut Vdbe = unsafe { (*pParse).pVdbe };
    let mut pKeyInfo: *mut KeyInfo = unsafe { std::mem::zeroed() };
    0 as i32;
    0 as i32;
    pKeyInfo = unsafe { sqlite3KeyInfoOfIndex(pParse, pIdx) };
    if pKeyInfo != std::ptr::null_mut::<KeyInfo>() {
        sqlite3VdbeAppendP4(v, pKeyInfo as *mut (), -(9 as i32));
    }
}

/// Change the comment on the most recently coded instruction.  Or
/// insert a No-op and add the comment to that new instruction.  This
/// makes the code easier to read during debugging.  None of this happens
/// in a production build.
fn vdbeVComment(mut p: *mut Vdbe, mut zFormat: *const i8, mut ap: core::ffi::VaList<'_>) {
    0 as i32;
    0 as i32;
    if (unsafe { (*p).nOp }) != (0 as i32) {
        0 as i32;
        unsafe {
            sqlite3DbFree(
                unsafe { (*p).db },
                (unsafe {
                    (*unsafe {
                        unsafe { (*p).aOp }.offset(((unsafe { (*p).nOp }) - (1 as i32)) as isize)
                    })
                    .zComment
                }) as *mut (),
            )
        };
        unsafe {
            (*unsafe {
                unsafe { (*p).aOp }.offset(((unsafe { (*p).nOp }) - (1 as i32)) as isize)
            })
            .zComment = unsafe { sqlite3VMPrintf(unsafe { (*p).db }, zFormat, ap.clone()) };
        }
    }
}

#[unsafe(no_mangle)]
unsafe extern "C-unwind" fn sqlite3VdbeComment(
    mut p: *mut Vdbe,
    mut zFormat: *const i8,
    mut __va_args: ...
) {
    let mut ap: core::ffi::VaList<'_> = unsafe { std::mem::zeroed() };
    if p != std::ptr::null_mut::<Vdbe>() {
        ap = __va_args.clone();
        vdbeVComment(p, zFormat, ap.clone());
        {}
    }
}

#[unsafe(no_mangle)]
unsafe extern "C-unwind" fn sqlite3VdbeNoopComment(
    mut p: *mut Vdbe,
    mut zFormat: *const i8,
    mut __va_args: ...
) {
    let mut ap: core::ffi::VaList<'_> = unsafe { std::mem::zeroed() };
    if p != std::ptr::null_mut::<Vdbe>() {
        sqlite3VdbeAddOp0(p, 189 as i32);
        ap = __va_args.clone();
        vdbeVComment(p, zFormat, ap.clone());
        {}
    }
}

/// Return the opcode for a given address.  The address must be non-negative.
/// See sqlite3VdbeGetLastOp() to get the most recently added opcode.
///
/// If a memory allocation error has occurred prior to the calling of this
/// routine, then a pointer to a dummy VdbeOp will be returned.  That opcode
/// is readable but not writable, though it is cast to a writable value.
/// The return of a dummy opcode allows the call to continue functioning
/// after an OOM fault without having to check to see if the return from
/// this routine is a valid pointer.  But because the dummy.opcode is 0,
/// dummy will never be written to.  This is verified by code inspection and
/// by running with Valgrind.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3VdbeGetOp(mut p: *mut Vdbe, mut addr: i32) -> *mut VdbeOp {
    // C89 specifies that the constant "dummy" will be initialized to all
    // zeros, which is correct.  MSVC generates a warning, nevertheless.
    // Ignore the MSVC warning about no initializer
    0 as i32;
    0 as i32;
    if (unsafe { (*unsafe { (*p).db }).mallocFailed }) != (0 as u8) {
        return unsafe { std::ptr::addr_of_mut!(dummy) };
    } else {
        return unsafe { unsafe { (*p).aOp }.offset(addr as isize) };
    }
    return unsafe { std::mem::zeroed() };
}

static mut dummy: VdbeOp = unsafe { std::mem::zeroed() };

/// Return the most recently added opcode
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3VdbeGetLastOp(mut p: *mut Vdbe) -> *mut VdbeOp {
    return sqlite3VdbeGetOp(p, (unsafe { (*p).nOp }) - (1 as i32));
}

/// Return an integer value for one of the parameters to the opcode pOp
/// determined by character c.
fn translateP(mut c: i8, mut pOp: *const VdbeOp) -> i32 {
    if (c as i32) == (49 as i32) {
        return unsafe { (*pOp).p1 };
    }
    if (c as i32) == (50 as i32) {
        return unsafe { (*pOp).p2 };
    }
    if (c as i32) == (51 as i32) {
        return unsafe { (*pOp).p3 };
    }
    if (c as i32) == (52 as i32) {
        return unsafe { (*pOp).p4.i };
    }
    return ((unsafe { (*pOp).p5 }) as u32) as i32;
}

/// Compute a string for the "comment" field of a VDBE opcode listing.
///
/// The Synopsis: field in comments in the vdbe.c source file gets converted
/// to an extra string that is appended to the sqlite3OpcodeName().  In the
/// absence of other comments, this synopsis becomes the comment on the opcode.
/// Some translation occurs:
///
///       "PX"      ->  "r[X]"
///       "PX@PY"   ->  "r[X..X+Y-1]"  or "r[x]" if y is 0 or 1
///       "PX@PY+1" ->  "r[X..X+Y]"    or "r[x]" if y is 0
///       "PY..PY"  ->  "r[X..Y]"      or "r[x]" if y<=x
///       "PINT13"  ->  int(P1|P3<<32)
///       "PDBL13"  ->  real(P1|P3<<32)
///       "PHEX23"  ->  hex(P2|P3<<32)
///
/// # Arguments
///
/// * `db` - Optional - Oom error reporting only
/// * `pOp` - The opcode to be commented
/// * `zP4` - Previously obtained value for P4
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3VdbeDisplayComment(
    mut db: *mut sqlite3,
    mut pOp: *const VdbeOp,
    mut zP4: *const i8,
) -> *mut i8 {
    let mut zOpName: *const i8 = unsafe { std::mem::zeroed() };
    let mut zSynopsis: *const i8 = unsafe { std::mem::zeroed() };
    let mut nOpName: i32 = 0 as i32;
    let mut ii: i32 = 0 as i32;
    let mut zAlt: __SlateAlign16<[i8; 50]> = __SlateAlign16([0 as i8; 50]);
    let mut x: sqlite3_str = unsafe { std::mem::zeroed() };
    unsafe {
        sqlite3StrAccumInit(
            std::ptr::addr_of_mut!(x),
            std::ptr::null_mut::<sqlite3>(),
            std::ptr::null_mut::<i8>(),
            0 as i32,
            1000000000 as i32,
        )
    };
    zOpName = unsafe { sqlite3OpcodeName(((unsafe { (*pOp).opcode }) as u32) as i32) };
    nOpName = unsafe { sqlite3Strlen30(zOpName) };
    if (unsafe { *unsafe { zOpName.offset((nOpName + (1 as i32)) as isize) } }) != (0 as i8) {
        let mut seenCom: i32 = 0 as i32;
        let mut c: i8 = 0 as i8;
        zSynopsis =
            unsafe { unsafe { zOpName.offset(nOpName as isize) }.offset((1 as i32) as isize) };
        if (unsafe {
            strncmp(
                zSynopsis,
                (b"IF \0".as_ptr() as *mut i8) as *const i8,
                ((3 as i32) as i64) as u64,
            )
        }) == (0 as i32)
        {
            unsafe {
                sqlite3_snprintf(
                    ((50 as u64) as u32) as i32,
                    zAlt.0.as_mut_ptr() as *mut i8,
                    (b"if %s goto P2\0".as_ptr() as *mut i8) as *const i8,
                    unsafe { zSynopsis.offset((3 as i32) as isize) },
                )
            };
            zSynopsis = (zAlt.0.as_mut_ptr() as *mut i8) as *const i8;
        }
        ii = 0 as i32;
        '__slate_break_1662: loop {
            let __v1868: i8 = unsafe { *unsafe { zSynopsis.offset(ii as isize) } };
            c = __v1868;
            if !((__v1868 as i32) != (0 as i32)) {
                break;
            }
            if (c as i32) == (80 as i32) {
                let __v1871: i32 = ii;
                let __v1872: i32 = __v1871 + (1 as i32);
                ii = __v1872;
                c = unsafe { *unsafe { zSynopsis.offset(__v1872 as isize) } };
                if (c as i32) == (52 as i32) {
                    unsafe { sqlite3_str_appendall(std::ptr::addr_of_mut!(x), zP4) };
                } else {
                    if (c as i32) == (88 as i32) {
                        if (unsafe { (*pOp).zComment }) != std::ptr::null_mut::<i8>()
                            && (unsafe {
                                *unsafe { unsafe { (*pOp).zComment }.offset((0 as i32) as isize) }
                            }) != (0 as i8)
                        {
                            unsafe {
                                sqlite3_str_appendall(
                                    std::ptr::addr_of_mut!(x),
                                    (unsafe { (*pOp).zComment }) as *const i8,
                                )
                            };
                            seenCom = 1 as i32;
                            break '__slate_break_1662;
                        }
                    } else {
                        if (unsafe {
                            strncmp(
                                unsafe { zSynopsis.offset(ii as isize) },
                                (b"INT13\0".as_ptr() as *mut i8) as *const i8,
                                ((5 as i32) as i64) as u64,
                            )
                        }) == (0 as i32)
                        {
                            unsafe {
                                sqlite3_str_appendf(
                                    std::ptr::addr_of_mut!(x),
                                    (b"%lld\0".as_ptr() as *mut i8) as *const i8,
                                    ((((unsafe { (*pOp).p3 }) as u32) as u64) << (32 as i32)
                                        | (((unsafe { (*pOp).p1 }) as u32) as u64))
                                        as i64,
                                )
                            };
                            let __v1873: i32 = ii;
                            let __v1874: i32 = __v1873 + (4 as i32);
                            ii = __v1874;
                        } else {
                            if (unsafe {
                                strncmp(
                                    unsafe { zSynopsis.offset(ii as isize) },
                                    (b"DBL13\0".as_ptr() as *mut i8) as *const i8,
                                    ((5 as i32) as i64) as u64,
                                )
                            }) == (0 as i32)
                            {
                                let mut iVal: i64 = ((((unsafe { (*pOp).p3 }) as u32) as u64)
                                    << (32 as i32)
                                    | (((unsafe { (*pOp).p1 }) as u32) as u64))
                                    as i64;
                                let mut r: f64 = 0 as f64;
                                unsafe {
                                    memcpy(
                                        std::ptr::addr_of_mut!(r) as *mut (),
                                        std::ptr::addr_of_mut!(iVal) as *const (),
                                        ((8 as i32) as i64) as u64,
                                    )
                                };
                                unsafe {
                                    sqlite3_str_appendf(
                                        std::ptr::addr_of_mut!(x),
                                        (b"%.17g\0".as_ptr() as *mut i8) as *const i8,
                                        r,
                                    )
                                };
                                let __v1875: i32 = ii;
                                let __v1876: i32 = __v1875 + (4 as i32);
                                ii = __v1876;
                            } else {
                                let mut v1: i32 = translateP(c, pOp);
                                let mut v2: i32 = 0 as i32;
                                if (unsafe {
                                    strncmp(
                                        unsafe {
                                            unsafe { zSynopsis.offset(ii as isize) }
                                                .offset((1 as i32) as isize)
                                        },
                                        (b"@P\0".as_ptr() as *mut i8) as *const i8,
                                        ((2 as i32) as i64) as u64,
                                    )
                                }) == (0 as i32)
                                {
                                    let __v1877: i32 = ii;
                                    let __v1878: i32 = __v1877 + (3 as i32);
                                    ii = __v1878;
                                    v2 = translateP(
                                        unsafe { *unsafe { zSynopsis.offset(ii as isize) } },
                                        pOp,
                                    );
                                    if (unsafe {
                                        strncmp(
                                            unsafe {
                                                unsafe { zSynopsis.offset(ii as isize) }
                                                    .offset((1 as i32) as isize)
                                            },
                                            (b"+1\0".as_ptr() as *mut i8) as *const i8,
                                            ((2 as i32) as i64) as u64,
                                        )
                                    }) == (0 as i32)
                                    {
                                        let __v1879: i32 = ii;
                                        let __v1880: i32 = __v1879 + (2 as i32);
                                        ii = __v1880;
                                        let __v1881: i32 = v2;
                                        let __v1882: i32 = __v1881 + (1 as i32);
                                        v2 = __v1882;
                                    }
                                    if v2 < (2 as i32) {
                                        unsafe {
                                            sqlite3_str_appendf(
                                                std::ptr::addr_of_mut!(x),
                                                (b"%d\0".as_ptr() as *mut i8) as *const i8,
                                                v1,
                                            )
                                        };
                                    } else {
                                        unsafe {
                                            sqlite3_str_appendf(
                                                std::ptr::addr_of_mut!(x),
                                                (b"%d..%d\0".as_ptr() as *mut i8) as *const i8,
                                                v1,
                                                v1 + v2 - (1 as i32),
                                            )
                                        };
                                    }
                                } else {
                                    if (unsafe {
                                        strncmp(
                                            unsafe {
                                                unsafe { zSynopsis.offset(ii as isize) }
                                                    .offset((1 as i32) as isize)
                                            },
                                            (b"@NP\0".as_ptr() as *mut i8) as *const i8,
                                            ((3 as i32) as i64) as u64,
                                        )
                                    }) == (0 as i32)
                                    {
                                        let mut pCtx: *mut sqlite3_context =
                                            unsafe { (*pOp).p4.pCtx };
                                        if ((unsafe { (*pOp).p4type }) as i32) != -(14 as i32)
                                            || (((unsafe { (*pCtx).argc }) as u32) as i32)
                                                == (1 as i32)
                                        {
                                            unsafe {
                                                sqlite3_str_appendf(
                                                    std::ptr::addr_of_mut!(x),
                                                    (b"%d\0".as_ptr() as *mut i8) as *const i8,
                                                    v1,
                                                )
                                            };
                                        } else {
                                            if (((unsafe { (*pCtx).argc }) as u32) as i32)
                                                > (1 as i32)
                                            {
                                                unsafe {
                                                    sqlite3_str_appendf(
                                                        std::ptr::addr_of_mut!(x),
                                                        (b"%d..%d\0".as_ptr() as *mut i8)
                                                            as *const i8,
                                                        v1,
                                                        v1 + (((unsafe { (*pCtx).argc }) as u32)
                                                            as i32)
                                                            - (1 as i32),
                                                    )
                                                };
                                            } else {
                                                if ((x.accError as u32) as i32) == (0 as i32) {
                                                    0 as i32;
                                                    let __v1883: u32 = x.nChar;
                                                    let __v1884: u32 =
                                                        __v1883.wrapping_sub((2 as i32) as u32);
                                                    x.nChar = __v1884;
                                                    let __v1885: i32 = ii;
                                                    let __v1886: i32 = __v1885 + (1 as i32);
                                                    ii = __v1886;
                                                }
                                            }
                                        }
                                        let __v1887: i32 = ii;
                                        let __v1888: i32 = __v1887 + (3 as i32);
                                        ii = __v1888;
                                    } else {
                                        unsafe {
                                            sqlite3_str_appendf(
                                                std::ptr::addr_of_mut!(x),
                                                (b"%d\0".as_ptr() as *mut i8) as *const i8,
                                                v1,
                                            )
                                        };
                                        if (unsafe {
                                            strncmp(
                                                unsafe {
                                                    unsafe { zSynopsis.offset(ii as isize) }
                                                        .offset((1 as i32) as isize)
                                                },
                                                (b"..P3\0".as_ptr() as *mut i8) as *const i8,
                                                ((4 as i32) as i64) as u64,
                                            )
                                        }) == (0 as i32)
                                            && (unsafe { (*pOp).p3 }) == (0 as i32)
                                        {
                                            let __v1889: i32 = ii;
                                            let __v1890: i32 = __v1889 + (4 as i32);
                                            ii = __v1890;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            } else {
                unsafe { sqlite3_str_appendchar(std::ptr::addr_of_mut!(x), 1 as i32, c) };
            }
            let __v1869: i32 = ii;
            let __v1870: i32 = __v1869 + (1 as i32);
            ii = __v1870;
        }
        if !(seenCom != (0 as i32)) && (unsafe { (*pOp).zComment }) != std::ptr::null_mut::<i8>() {
            unsafe {
                sqlite3_str_appendf(
                    std::ptr::addr_of_mut!(x),
                    (b"; %s\0".as_ptr() as *mut i8) as *const i8,
                    unsafe { (*pOp).zComment },
                )
            };
        }
    } else {
        if (unsafe { (*pOp).zComment }) != std::ptr::null_mut::<i8>() {
            unsafe {
                sqlite3_str_appendall(
                    std::ptr::addr_of_mut!(x),
                    (unsafe { (*pOp).zComment }) as *const i8,
                )
            };
        }
    }
    if ((x.accError as u32) as i32) & (7 as i32) != (0 as i32)
        && db != std::ptr::null_mut::<sqlite3>()
    {
        unsafe { sqlite3OomFault(db) };
    }
    return unsafe { sqlite3StrAccumFinish(std::ptr::addr_of_mut!(x)) };
}

/// Compute a string that describes the P4 parameter for an opcode.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3VdbeDisplayP4(mut db: *mut sqlite3, mut pOp: *mut VdbeOp) -> *mut i8 {
    let mut zP4: *mut i8 = std::ptr::null_mut::<i8>();
    let mut x: sqlite3_str = unsafe { std::mem::zeroed() };
    unsafe {
        sqlite3StrAccumInit(
            std::ptr::addr_of_mut!(x),
            std::ptr::null_mut::<sqlite3>(),
            std::ptr::null_mut::<i8>(),
            0 as i32,
            1000000000 as i32,
        )
    };
    match (unsafe { (*pOp).p4type }) as i32 {
        -9 => {
            let mut j: i32 = 0 as i32;
            let mut pKeyInfo: *mut KeyInfo = unsafe { (*pOp).p4.pKeyInfo };
            0 as i32;
            unsafe {
                sqlite3_str_appendf(
                    std::ptr::addr_of_mut!(x),
                    (b"k(%d\0".as_ptr() as *mut i8) as *const i8,
                    ((unsafe { (*pKeyInfo).nKeyField }) as u32) as i32,
                )
            };
            j = 0 as i32;
            '__slate_break_1679: loop {
                if !(j < (((unsafe { (*pKeyInfo).nKeyField }) as u32) as i32)) {
                    break;
                }
                let mut pColl: *mut CollSeq = unsafe {
                    *unsafe {
                        unsafe { std::ptr::addr_of_mut!((*pKeyInfo).aColl) as *mut *mut CollSeq }
                            .offset(j as isize)
                    }
                };
                let mut zColl: *const i8 = (if pColl != std::ptr::null_mut::<CollSeq>() {
                    unsafe { (*pColl).zName }
                } else {
                    b"\0".as_ptr() as *mut i8
                }) as *const i8;
                if (unsafe { strcmp(zColl, (b"BINARY\0".as_ptr() as *mut i8) as *const i8) })
                    == (0 as i32)
                {
                    zColl = (b"B\0".as_ptr() as *mut i8) as *const i8;
                }
                unsafe {
                    sqlite3_str_appendf(
                        std::ptr::addr_of_mut!(x),
                        (b",%s%s%s\0".as_ptr() as *mut i8) as *const i8,
                        if (((unsafe {
                            *unsafe { unsafe { (*pKeyInfo).aSortFlags }.offset(j as isize) }
                        }) as u32) as i32)
                            & (1 as i32)
                            != (0 as i32)
                        {
                            b"-\0".as_ptr() as *mut i8
                        } else {
                            b"\0".as_ptr() as *mut i8
                        },
                        if (((unsafe {
                            *unsafe { unsafe { (*pKeyInfo).aSortFlags }.offset(j as isize) }
                        }) as u32) as i32)
                            & (2 as i32)
                            != (0 as i32)
                        {
                            b"N.\0".as_ptr() as *mut i8
                        } else {
                            b"\0".as_ptr() as *mut i8
                        },
                        zColl,
                    )
                };
                let __v1864: i32 = j;
                let __v1865: i32 = __v1864 + (1 as i32);
                j = __v1865;
            }
            unsafe {
                sqlite3_str_append(
                    std::ptr::addr_of_mut!(x),
                    (b")\0".as_ptr() as *mut i8) as *const i8,
                    1 as i32,
                )
            };
        }
        -2 => {
            let mut pColl: *mut CollSeq = unsafe { (*pOp).p4.pColl };
            0 as i32;
            unsafe {
                sqlite3_str_appendf(
                    std::ptr::addr_of_mut!(x),
                    (b"%.18s-%s\0".as_ptr() as *mut i8) as *const i8,
                    unsafe { (*pColl).zName },
                    unsafe {
                        *unsafe {
                            unsafe { std::ptr::addr_of!(encnames.0) as *const *const i8 }
                                .offset((((unsafe { (*pColl).enc }) as u32) as i32) as isize)
                        }
                    },
                )
            };
        }
        -8 => {
            let mut pDef: *mut FuncDef = unsafe { (*pOp).p4.pFunc };
            unsafe {
                sqlite3_str_appendf(
                    std::ptr::addr_of_mut!(x),
                    (b"%s(%d)\0".as_ptr() as *mut i8) as *const i8,
                    unsafe { (*pDef).zName },
                    (unsafe { (*pDef).nArg }) as i32,
                )
            };
        }
        -14 => {
            let mut pDef: *mut FuncDef = unsafe { (*unsafe { (*pOp).p4.pCtx }).pFunc };
            unsafe {
                sqlite3_str_appendf(
                    std::ptr::addr_of_mut!(x),
                    (b"%s(%d)\0".as_ptr() as *mut i8) as *const i8,
                    unsafe { (*pDef).zName },
                    (unsafe { (*pDef).nArg }) as i32,
                )
            };
        }
        -3 => {
            unsafe {
                sqlite3_str_appendf(
                    std::ptr::addr_of_mut!(x),
                    (b"%d\0".as_ptr() as *mut i8) as *const i8,
                    unsafe { (*pOp).p4.i },
                )
            };
        }
        -11 => {
            let mut pMem: *mut sqlite3_value = unsafe { (*pOp).p4.pMem };
            if (((unsafe { (*pMem).flags }) as u32) as i32) & (2 as i32) != (0 as i32) {
                zP4 = unsafe { (*pMem).z };
            } else {
                if (((unsafe { (*pMem).flags }) as u32) as i32) & ((4 as i32) | (32 as i32))
                    != (0 as i32)
                {
                    unsafe {
                        sqlite3_str_appendf(
                            std::ptr::addr_of_mut!(x),
                            (b"%lld\0".as_ptr() as *mut i8) as *const i8,
                            unsafe { (*pMem).u.i },
                        )
                    };
                } else {
                    if (((unsafe { (*pMem).flags }) as u32) as i32) & (8 as i32) != (0 as i32) {
                        unsafe {
                            sqlite3_str_appendf(
                                std::ptr::addr_of_mut!(x),
                                (b"%.16g\0".as_ptr() as *mut i8) as *const i8,
                                unsafe { (*pMem).u.r },
                            )
                        };
                    } else {
                        if (((unsafe { (*pMem).flags }) as u32) as i32) & (1 as i32) != (0 as i32) {
                            zP4 = b"NULL\0".as_ptr() as *mut i8;
                        } else {
                            0 as i32;
                            zP4 = b"(blob)\0".as_ptr() as *mut i8;
                        }
                    }
                }
            }
        }
        -12 => {
            let mut pVtab: *mut sqlite3_vtab = unsafe { (*unsafe { (*pOp).p4.pVtab }).pVtab };
            unsafe {
                sqlite3_str_appendf(
                    std::ptr::addr_of_mut!(x),
                    (b"vtab:%p\0".as_ptr() as *mut i8) as *const i8,
                    pVtab,
                )
            };
        }
        -13 => {
            let mut i: u32 = 0 as u32;
            let mut ai: *mut u32 = unsafe { (*pOp).p4.ai };
            let mut n: u32 = unsafe { *unsafe { ai.offset((0 as i32) as isize) } };
            // The first element of an INTARRAY is always the
            // count of the number of elements to follow
            i = (1 as i32) as u32;
            '__slate_break_1702: while i <= n {
                unsafe {
                    sqlite3_str_appendf(
                        std::ptr::addr_of_mut!(x),
                        (b"%c%u\0".as_ptr() as *mut i8) as *const i8,
                        if i == ((1 as i32) as u32) {
                            91 as i32
                        } else {
                            44 as i32
                        },
                        unsafe { *unsafe { ai.offset(i as isize) } },
                    )
                };
                let __v1866: u32 = i;
                let __v1867: u32 = __v1866.wrapping_add((1 as i32) as u32);
                i = __v1867;
            }
            unsafe {
                sqlite3_str_append(
                    std::ptr::addr_of_mut!(x),
                    (b"]\0".as_ptr() as *mut i8) as *const i8,
                    1 as i32,
                )
            };
        }
        -4 => {
            zP4 = b"program\0".as_ptr() as *mut i8;
        }
        -5 => {
            zP4 = unsafe { (*unsafe { (*pOp).p4.pTab }).zName };
        }
        -6 => {
            zP4 = unsafe { (*unsafe { (*pOp).p4.pIdx }).zName };
        }
        -16 => {
            let mut pSig: *mut SubrtnSig = unsafe { (*pOp).p4.pSubrtnSig };
            unsafe {
                sqlite3_str_appendf(
                    std::ptr::addr_of_mut!(x),
                    (b"subrtnsig:%d,%s\0".as_ptr() as *mut i8) as *const i8,
                    unsafe { (*pSig).selId },
                    unsafe { (*pSig).zAff },
                )
            };
        }
        _ => {
            zP4 = unsafe { (*pOp).p4.z };
        }
    }
    if zP4 != std::ptr::null_mut::<i8>() {
        unsafe { sqlite3_str_appendall(std::ptr::addr_of_mut!(x), zP4 as *const i8) };
    }
    if ((x.accError as u32) as i32) & (7 as i32) != (0 as i32) {
        unsafe { sqlite3OomFault(db) };
    }
    return unsafe { sqlite3StrAccumFinish(std::ptr::addr_of_mut!(x)) };
}

static mut encnames: __SlateAlign16<[*const i8; 4]> = __SlateAlign16([
    (b"?\0".as_ptr() as *mut i8) as *const i8,
    (b"8\0".as_ptr() as *mut i8) as *const i8,
    (b"16LE\0".as_ptr() as *mut i8) as *const i8,
    (b"16BE\0".as_ptr() as *mut i8) as *const i8,
]);

/// Declare to the Vdbe that the BTree object at db->aDb[i] is used.
///
/// The prepared statements need to know in advance the complete set of
/// attached databases that will be use.  A mask of these databases
/// is maintained in p->btreeMask.  The p->lockMask value is the subset of
/// p->btreeMask of databases that will require a lock.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3VdbeUsesBtree(mut p: *mut Vdbe, mut i: i32) {
    0 as i32;
    0 as i32;
    let __v1782: *mut Vdbe = p;
    let __v1783: u32 = unsafe { (*__v1782).btreeMask };
    let __v1784: u32 = __v1783 | ((1 as i32) as u32) << i;
    unsafe {
        (*__v1782).btreeMask = __v1784;
    }
    let __v1785: bool;
    if i != (1 as i32) {
        __v1785 = (unsafe {
            sqlite3BtreeSharable(unsafe {
                (*unsafe { unsafe { (*unsafe { (*p).db }).aDb }.offset(i as isize) }).pBt
            })
        }) != (0 as i32);
    } else {
        __v1785 = false as bool;
    }
    if __v1785 {
        let __v1786: *mut Vdbe = p;
        let __v1787: u32 = unsafe { (*__v1786).lockMask };
        let __v1788: u32 = __v1787 | ((1 as i32) as u32) << i;
        unsafe {
            (*__v1786).lockMask = __v1788;
        }
    }
}

/// If SQLite is compiled to support shared-cache mode and to be threadsafe,
/// this routine obtains the mutex associated with each BtShared structure
/// that may be accessed by the VM passed as an argument. In doing so it also
/// sets the BtShared.db member of each of the BtShared structures, ensuring
/// that the correct busy-handler callback is invoked if required.
///
/// If SQLite is not threadsafe but does support shared-cache mode, then
/// sqlite3BtreeEnter() is invoked to set the BtShared.db variables
/// of all of BtShared structures accessible via the database handle
/// associated with the VM.
///
/// If SQLite is not threadsafe and does not support shared-cache mode, this
/// function is a no-op.
///
/// The p->btreeMask field is a bitmask of all btrees that the prepared
/// statement p will ever use.  Let N be the number of bits in p->btreeMask
/// corresponding to btrees that use shared cache.  Then the runtime of
/// this routine is N*N.  But as N is rarely more than 1, this should not
/// be a problem.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3VdbeEnter(mut p: *mut Vdbe) {
    let mut i: i32 = 0 as i32;
    let mut db: *mut sqlite3 = unsafe { std::mem::zeroed() };
    let mut aDb: *mut Db = unsafe { std::mem::zeroed() };
    let mut nDb: i32 = 0 as i32;
    if (unsafe { (*p).lockMask }) == ((0 as i32) as u32) {
        return;
    }
    // The common case
    db = unsafe { (*p).db };
    aDb = unsafe { (*db).aDb };
    nDb = unsafe { (*db).nDb };
    i = 0 as i32;
    '__slate_break_1707: loop {
        if !(i < nDb) {
            break;
        }
        if i != (1 as i32)
            && (unsafe { (*p).lockMask }) & ((1 as i32) as u32) << i != ((0 as i32) as u32)
            && (unsafe { (*unsafe { aDb.offset(i as isize) }).pBt })
                != std::ptr::null_mut::<Btree>()
        {
            unsafe { sqlite3BtreeEnter(unsafe { (*unsafe { aDb.offset(i as isize) }).pBt }) };
        }
        let __v1922: i32 = i;
        let __v1923: i32 = __v1922 + (1 as i32);
        i = __v1923;
    }
}

/// Unlock all of the btrees previously locked by a call to sqlite3VdbeEnter().
fn vdbeLeave(mut p: *mut Vdbe) {
    let mut i: i32 = 0 as i32;
    let mut db: *mut sqlite3 = unsafe { std::mem::zeroed() };
    let mut aDb: *mut Db = unsafe { std::mem::zeroed() };
    let mut nDb: i32 = 0 as i32;
    db = unsafe { (*p).db };
    aDb = unsafe { (*db).aDb };
    nDb = unsafe { (*db).nDb };
    i = 0 as i32;
    '__slate_break_1708: loop {
        if !(i < nDb) {
            break;
        }
        if i != (1 as i32)
            && (unsafe { (*p).lockMask }) & ((1 as i32) as u32) << i != ((0 as i32) as u32)
            && (unsafe { (*unsafe { aDb.offset(i as isize) }).pBt })
                != std::ptr::null_mut::<Btree>()
        {
            unsafe { sqlite3BtreeLeave(unsafe { (*unsafe { aDb.offset(i as isize) }).pBt }) };
        }
        let __v1928: i32 = i;
        let __v1929: i32 = __v1928 + (1 as i32);
        i = __v1929;
    }
}

#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3VdbeLeave(mut p: *mut Vdbe) {
    if (unsafe { (*p).lockMask }) == ((0 as i32) as u32) {
        return;
    }
    // The common case
    vdbeLeave(p);
}

/// Initialize an array of N Mem element.
///
/// This is a high-runner, so only those fields that really do need to
/// be initialized are set.  The Mem structure is organized so that
/// the fields that get initialized are nearby and hopefully on the same
/// cache line.
///
///    Mem.flags = flags
///    Mem.db = db
///    Mem.szMalloc = 0
///
/// All other fields of Mem can safely remain uninitialized for now.  They
/// will be initialized before use.
fn initMemArray(mut p: *mut sqlite3_value, mut N: i32, mut db: *mut sqlite3, mut flags: u16) {
    0 as i32;
    if N > (0 as i32) {
        '__slate_break_1709: loop {
            unsafe {
                (*p).flags = flags;
            }
            unsafe {
                (*p).db = db;
            }
            unsafe {
                (*p).szMalloc = 0 as i32;
            }
            let __v1930: *mut sqlite3_value = p;
            let __v1931: *mut sqlite3_value = unsafe { __v1930.offset((1 as i32) as isize) };
            p = __v1931;
            let __v1932: i32 = N;
            let __v1933: i32 = __v1932 - (1 as i32);
            N = __v1933;
            if !(__v1933 > (0 as i32)) {
                break;
            }
        }
    }
}

/// Release auxiliary memory held in an array of N Mem elements.
///
/// After this routine returns, all Mem elements in the array will still
/// be valid.  Those Mem elements that were not holding auxiliary resources
/// will be unchanged.  Mem elements which had something freed will be
/// set to MEM_Undefined.
fn releaseMemArray(mut p: *mut sqlite3_value, mut N: i32) {
    if p != std::ptr::null_mut::<sqlite3_value>() && N != (0 as i32) {
        let mut pEnd: *mut sqlite3_value = unsafe { p.offset(N as isize) };
        let mut db: *mut sqlite3 = unsafe { (*p).db };
        0 as i32;
        if (unsafe { (*db).pnBytesFreed }) != std::ptr::null_mut::<i32>() {
            '__slate_break_1710: loop {
                if (unsafe { (*p).szMalloc }) != (0 as i32) {
                    unsafe { sqlite3DbFree(db, (unsafe { (*p).zMalloc }) as *mut ()) };
                }
                let __v1934: *mut sqlite3_value = p;
                let __v1935: *mut sqlite3_value = unsafe { __v1934.offset((1 as i32) as isize) };
                p = __v1935;
                if !(__v1935 < pEnd) {
                    break;
                }
            }
            return;
        }
        '__slate_break_1711: loop {
            0 as i32;
            0 as i32;
            // This block is really an inlined version of sqlite3VdbeMemRelease()
            // that takes advantage of the fact that the memory cell value is
            // being set to NULL after releasing any dynamic resources.
            //
            // The justification for duplicating code is that according to
            // callgrind, this causes a certain test case to hit the CPU 4.7
            // percent less (x86 linux, gcc version 4.1.2, -O6) than if
            // sqlite3MemRelease() were called from here. With -O2, this jumps
            // to 6.6 percent. The test case is inserting 1000 rows into a table
            // with no indexes using a single prepared INSERT statement, bind()
            // and reset(). Inserts are grouped into a transaction.
            {}
            {}
            if (((unsafe { (*p).flags }) as u32) as i32) & ((32768 as i32) | (4096 as i32))
                != (0 as i32)
            {
                {}
                unsafe { sqlite3VdbeMemRelease(p) };
                unsafe {
                    (*p).flags = ((0 as i32) as i16) as u16;
                }
            } else {
                if (unsafe { (*p).szMalloc }) != (0 as i32) {
                    unsafe { sqlite3DbNNFreeNN(db, (unsafe { (*p).zMalloc }) as *mut ()) };
                    unsafe {
                        (*p).szMalloc = 0 as i32;
                    }
                    unsafe {
                        (*p).flags = ((0 as i32) as i16) as u16;
                    }
                }
            }
            let __v1936: *mut sqlite3_value = p;
            let __v1937: *mut sqlite3_value = unsafe { __v1936.offset((1 as i32) as isize) };
            p = __v1937;
            if !(__v1937 < pEnd) {
                break;
            }
        }
    }
}

/// This is a destructor on a Mem object (which is really an sqlite3_value)
/// that deletes the Frame object that is attached to it as a blob.
///
/// This routine does not delete the Frame right away.  It merely adds the
/// frame to a list of frames to be deleted when the Vdbe halts.
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.vdbeaux.sqlite3VdbeFrameMemDel")]
extern "C-unwind" fn sqlite3VdbeFrameMemDel(mut pArg: *mut ()) {
    let mut pFrame: *mut VdbeFrame = pArg as *mut VdbeFrame;
    0 as i32;
    unsafe {
        (*pFrame).pParent = unsafe { (*unsafe { (*pFrame).v }).pDelFrame };
    }
    unsafe {
        (*unsafe { (*pFrame).v }).pDelFrame = pFrame;
    }
}

/// Locate the next opcode to be displayed in EXPLAIN or EXPLAIN
/// QUERY PLAN output.
///
/// Return SQLITE_ROW on success.  Return SQLITE_DONE if there are no
/// more opcodes to be displayed.
///
/// # Arguments
///
/// * `p` - The statement being explained
/// * `pSub` - Storage for keeping track of subprogram nesting
/// * `eMode` - 0: normal.  1: EQP.  2:  TablesUsed
/// * `piPc` - IN/OUT: Current rowid.  Overwritten with next rowid
/// * `piAddr` - OUT: Write index into (*paOp)[] here
/// * `paOp` - OUT: Write the opcode array here
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3VdbeNextOpcode(
    mut p: *mut Vdbe,
    mut pSub: *mut sqlite3_value,
    mut eMode: i32,
    mut piPc: *mut i32,
    mut piAddr: *mut i32,
    mut paOp: *mut *mut VdbeOp,
) -> i32 {
    let mut nRow: i32 = 0 as i32; // Stop when row count reaches this
    let mut nSub: i32 = 0 as i32; // Number of sub-vdbes seen so far
    let mut apSub: *mut *mut SubProgram = std::ptr::null_mut::<*mut SubProgram>(); // Array of sub-vdbes
    let mut i: i32 = 0 as i32; // Next instruction address
    let mut rc: i32 = 0 as i32; // Result code
    let mut aOp: *mut VdbeOp = std::ptr::null_mut::<VdbeOp>(); // Opcode array
    let mut iPc: i32 = 0 as i32; // Rowid.  Copy of value in *piPc
    // When the number of output rows reaches nRow, that means the
    // listing has finished and sqlite3_step() should return SQLITE_DONE.
    // nRow is the sum of the number of rows in the main program, plus
    // the sum of the number of rows in all trigger subprograms encountered
    // so far.  The nRow value will increase as new trigger subprograms are
    // encountered, but p->pc will eventually catch up to nRow.
    nRow = unsafe { (*p).nOp };
    if pSub != std::ptr::null_mut::<sqlite3_value>() {
        if (((unsafe { (*pSub).flags }) as u32) as i32) & (16 as i32) != (0 as i32) {
            // pSub is initiallly NULL.  It is initialized to a BLOB by
            // the P4_SUBPROGRAM processing logic below
            nSub = (((((unsafe { (*pSub).n }) as i64) as u64) / (8 as u64)) as u32) as i32;
            apSub = (unsafe { (*pSub).z }) as *mut *mut SubProgram;
        }
        i = 0 as i32;
        '__slate_break_1712: loop {
            if !(i < nSub) {
                break;
            }
            let __v1848: i32 = nRow;
            let __v1849: i32 =
                __v1848 + unsafe { (*unsafe { *unsafe { apSub.offset(i as isize) } }).nOp };
            nRow = __v1849;
            let __v1846: i32 = i;
            let __v1847: i32 = __v1846 + (1 as i32);
            i = __v1847;
        }
    }
    iPc = unsafe { *piPc };
    '__slate_break_1713: while (1 as i32) != (0 as i32) {
        // Loop exits via break
        let __v1850: i32 = iPc;
        let __v1851: i32 = __v1850 + (1 as i32);
        iPc = __v1851;
        i = __v1850;
        if i >= nRow {
            unsafe {
                (*p).rc = 0 as i32;
            }
            rc = 101 as i32;
            break '__slate_break_1713;
        }
        if i < unsafe { (*p).nOp } {
            // The rowid is small enough that we are still in the
            // main program.
            aOp = unsafe { (*p).aOp };
        } else {
            // We are currently listing subprograms.  Figure out which one and
            // pick up the appropriate opcode.
            let mut j: i32 = 0 as i32;
            let __v1852: i32 = i;
            let __v1853: i32 = __v1852 - unsafe { (*p).nOp };
            i = __v1853;
            0 as i32;
            0 as i32;
            j = 0 as i32;
            '__slate_break_1714: loop {
                if !(i >= unsafe { (*unsafe { *unsafe { apSub.offset(j as isize) } }).nOp }) {
                    break;
                }
                let __v1856: i32 = i;
                let __v1857: i32 =
                    __v1856 - unsafe { (*unsafe { *unsafe { apSub.offset(j as isize) } }).nOp };
                i = __v1857;
                0 as i32;
                let __v1854: i32 = j;
                let __v1855: i32 = __v1854 + (1 as i32);
                j = __v1855;
            }
            aOp = unsafe { (*unsafe { *unsafe { apSub.offset(j as isize) } }).aOp };
        }
        // When an OP_Program opcode is encounter (the only opcode that has
        // a P4_SUBPROGRAM argument), expand the size of the array of subprograms
        // kept in p->aMem[9].z to hold the new program - assuming this subprogram
        // has not already been seen.
        if pSub != std::ptr::null_mut::<sqlite3_value>()
            && ((unsafe { (*unsafe { aOp.offset(i as isize) }).p4type }) as i32) == -(4 as i32)
        {
            let mut nByte: i32 =
                ((((nSub + (1 as i32)) as i64) as u64).wrapping_mul(8 as u64) as u32) as i32;
            let mut j: i32 = 0 as i32;
            j = 0 as i32;
            '__slate_break_1715: loop {
                if !(j < nSub) {
                    break;
                }
                if (unsafe { *unsafe { apSub.offset(j as isize) } })
                    == unsafe { (*unsafe { aOp.offset(i as isize) }).p4.pProgram }
                {
                    break '__slate_break_1715;
                }
                let __v1858: i32 = j;
                let __v1859: i32 = __v1858 + (1 as i32);
                j = __v1859;
            }
            if j == nSub {
                unsafe {
                    (*p).rc =
                        unsafe { sqlite3VdbeMemGrow(pSub, nByte, (nSub != (0 as i32)) as i32) };
                }
                if (unsafe { (*p).rc }) != (0 as i32) {
                    rc = 1 as i32;
                    break '__slate_break_1713;
                }
                apSub = (unsafe { (*pSub).z }) as *mut *mut SubProgram;
                let __v1860: i32 = nSub;
                let __v1861: i32 = __v1860 + (1 as i32);
                nSub = __v1861;
                unsafe {
                    *unsafe { apSub.offset(__v1860 as isize) } =
                        unsafe { (*unsafe { aOp.offset(i as isize) }).p4.pProgram };
                }
                unsafe {
                    (*pSub).flags = (((((unsafe { (*pSub).flags }) as u32) as i32)
                        & !((3519 as i32) | (1024 as i32))
                        | (16 as i32)) as i16) as u16;
                }
                unsafe {
                    (*pSub).n = (((nSub as i64) as u64).wrapping_mul(8 as u64) as u32) as i32;
                }
                let __v1862: i32 = nRow;
                let __v1863: i32 = __v1862
                    + unsafe { (*unsafe { (*unsafe { aOp.offset(i as isize) }).p4.pProgram }).nOp };
                nRow = __v1863;
            }
        }
        if eMode == (0 as i32) {
            break '__slate_break_1713;
        }
        if eMode == (2 as i32) {
            let mut pOp: *mut VdbeOp = unsafe { aOp.offset(i as isize) };
            if (((unsafe { (*pOp).opcode }) as u32) as i32) == (114 as i32) {
                break '__slate_break_1713;
            }
            if (((unsafe { (*pOp).opcode }) as u32) as i32) == (116 as i32)
                && (((unsafe { (*pOp).p5 }) as u32) as i32) & (16 as i32) == (0 as i32)
            {
                break '__slate_break_1713;
            }
            if (((unsafe { (*pOp).opcode }) as u32) as i32) == (113 as i32) {
                break '__slate_break_1713;
            }
        } else {
            0 as i32;
            if (((unsafe { (*unsafe { aOp.offset(i as isize) }).opcode }) as u32) as i32)
                == (190 as i32)
            {
                break '__slate_break_1713;
            }
            if (((unsafe { (*unsafe { aOp.offset(i as isize) }).opcode }) as u32) as i32)
                == (8 as i32)
                && iPc > (1 as i32)
            {
                break '__slate_break_1713;
            }
        }
    }
    unsafe {
        *piPc = iPc;
    }
    unsafe {
        *piAddr = i;
    }
    unsafe {
        *paOp = aOp;
    }
    return rc;
}

/// Delete a VdbeFrame object and its contents. VdbeFrame objects are
/// allocated by the OP_Program opcode in sqlite3VdbeExec().
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3VdbeFrameDelete(mut p: *mut VdbeFrame) {
    let mut i: i32 = 0 as i32;
    let mut aMem: *mut sqlite3_value = (unsafe {
        (p as *mut u8).offset(
            ((112 as u64).wrapping_add(((7 as i32) as i64) as u64) & ((!(7 as i32) as i64) as u64))
                as isize,
        )
    }) as *mut sqlite3_value;
    let mut apCsr: *mut *mut VdbeCursor =
        (unsafe { aMem.offset((unsafe { (*p).nChildMem }) as isize) }) as *mut *mut VdbeCursor;
    0 as i32;
    i = 0 as i32;
    '__slate_break_1716: loop {
        if !(i < unsafe { (*p).nChildCsr }) {
            break;
        }
        if (unsafe { *unsafe { apCsr.offset(i as isize) } }) != std::ptr::null_mut::<VdbeCursor>() {
            sqlite3VdbeFreeCursorNN(unsafe { (*p).v }, unsafe {
                *unsafe { apCsr.offset(i as isize) }
            });
        }
        let __v1906: i32 = i;
        let __v1907: i32 = __v1906 + (1 as i32);
        i = __v1907;
    }
    releaseMemArray(aMem, unsafe { (*p).nChildMem });
    sqlite3VdbeDeleteAuxData(
        unsafe { (*unsafe { (*p).v }).db },
        unsafe { std::ptr::addr_of_mut!((*p).pAuxData) },
        -(1 as i32),
        0 as i32,
    );
    unsafe { sqlite3DbFree(unsafe { (*unsafe { (*p).v }).db }, p as *mut ()) };
}

/// Give a listing of the program in the virtual machine.
///
/// The interface is the same as sqlite3VdbeExec().  But instead of
/// running the code, it invokes the callback once for each instruction.
/// This feature is used to implement "EXPLAIN".
///
/// When p->explain==1, each instruction is listed.  When
/// p->explain==2, only OP_Explain instructions are listed and these
/// are shown in a different format.  p->explain==2 is used to implement
/// EXPLAIN QUERY PLAN.
/// 2018-04-24:  In p->explain==2 mode, the OP_Init opcodes of triggers
/// are also shown, so that the boundaries between the main program and
/// each trigger are clear.
///
/// When p->explain==1, first the main program is listed, then each of
/// the trigger subprograms are listed one by one.
///
/// # Arguments
///
/// * `p` - The VDBE
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3VdbeList(mut p: *mut Vdbe) -> i32 {
    let mut pSub: *mut sqlite3_value = std::ptr::null_mut::<sqlite3_value>(); // Memory cell hold array of subprogs
    let mut db: *mut sqlite3 = unsafe { (*p).db }; // The database connection
    let mut i: i32 = 0 as i32; // Loop counter
    let mut rc: i32 = 0 as i32; // Return code
    let mut pMem: *mut sqlite3_value = unsafe { unsafe { (*p).aMem }.offset((1 as i32) as isize) }; // First Mem of result set
    let mut bListSubprogs: i32 = (((unsafe { (*p).__slate_bits_0.__get_explain() }) as i32)
        == (1 as i32)
        || (unsafe { (*db).flags }) & (((16777216 as i32) as i64) as u64)
            != (((0 as i32) as i64) as u64)) as i32;
    let mut aOp: *mut VdbeOp = unsafe { std::mem::zeroed() }; // Array of opcodes
    let mut pOp: *mut VdbeOp = unsafe { std::mem::zeroed() }; // Current opcode
    0 as i32;
    0 as i32;
    0 as i32;
    // Even though this opcode does not use dynamic strings for
    // the result, result columns may become dynamic if the user calls
    // sqlite3_column_text16(), causing a translation to UTF-16 encoding.
    releaseMemArray(pMem, 8 as i32);
    if (unsafe { (*p).rc }) == (7 as i32) {
        // This happens if a malloc() inside a call to sqlite3_column_text() or
        // sqlite3_column_text16() failed.
        unsafe { sqlite3OomFault(db) };
        return 1 as i32;
    }
    if bListSubprogs != (0 as i32) {
        // The first 8 memory cells are used for the result set.  So we will
        // commandeer the 9th cell to use as storage for an array of pointers
        // to trigger subprograms.  The VDBE is guaranteed to have at least 9
        // cells.
        0 as i32;
        pSub = unsafe { unsafe { (*p).aMem }.offset((9 as i32) as isize) };
    } else {
        pSub = std::ptr::null_mut::<sqlite3_value>();
    }
    // Figure out which opcode is next to display
    rc = sqlite3VdbeNextOpcode(
        p,
        pSub,
        (((unsafe { (*p).__slate_bits_0.__get_explain() }) as i32) == (2 as i32)) as i32,
        unsafe { std::ptr::addr_of_mut!((*p).pc) },
        std::ptr::addr_of_mut!(i),
        std::ptr::addr_of_mut!(aOp),
    );
    if rc == (0 as i32) {
        pOp = unsafe { aOp.offset(i as isize) };
        if (unsafe {
            std::sync::atomic::AtomicI32::load_volatile(
                std::sync::atomic::AtomicI32::from_ptr_raw(
                    (unsafe { std::ptr::addr_of_mut!((*db).u1.isInterrupted) }) as *mut i32,
                ),
                std::sync::atomic::Ordering::Relaxed,
            )
        }) != (0 as i32)
        {
            unsafe {
                (*p).rc = 9 as i32;
            }
            rc = 1 as i32;
            unsafe { sqlite3VdbeError(p, unsafe { sqlite3ErrStr(unsafe { (*p).rc }) }) };
        } else {
            let mut zP4: *mut i8 = sqlite3VdbeDisplayP4(db, pOp);
            if ((unsafe { (*p).__slate_bits_0.__get_explain() }) as i32) == (2 as i32) {
                unsafe { sqlite3VdbeMemSetInt64(pMem, (unsafe { (*pOp).p1 }) as i64) };
                unsafe {
                    sqlite3VdbeMemSetInt64(
                        unsafe { pMem.offset((1 as i32) as isize) },
                        (unsafe { (*pOp).p2 }) as i64,
                    )
                };
                unsafe {
                    sqlite3VdbeMemSetInt64(
                        unsafe { pMem.offset((2 as i32) as isize) },
                        (unsafe { (*pOp).p3 }) as i64,
                    )
                };
                unsafe {
                    sqlite3VdbeMemSetStr(
                        unsafe { pMem.offset((3 as i32) as isize) },
                        zP4 as *const i8,
                        -(1 as i32) as i64,
                        ((1 as i32) as i8) as u8,
                        unsafe {
                            std::mem::transmute::<
                                *const (),
                                Option<unsafe extern "C-unwind" fn(*mut ())>,
                            >(sqlite3_free as *const ())
                        },
                    )
                };
                0 as i32;
            } else {
                unsafe {
                    sqlite3VdbeMemSetInt64(unsafe { pMem.offset((0 as i32) as isize) }, i as i64)
                };
                unsafe {
                    sqlite3VdbeMemSetStr(
                        unsafe { pMem.offset((1 as i32) as isize) },
                        ((unsafe { sqlite3OpcodeName(((unsafe { (*pOp).opcode }) as u32) as i32) })
                            as *mut i8) as *const i8,
                        -(1 as i32) as i64,
                        ((1 as i32) as i8) as u8,
                        None,
                    )
                };
                unsafe {
                    sqlite3VdbeMemSetInt64(
                        unsafe { pMem.offset((2 as i32) as isize) },
                        (unsafe { (*pOp).p1 }) as i64,
                    )
                };
                unsafe {
                    sqlite3VdbeMemSetInt64(
                        unsafe { pMem.offset((3 as i32) as isize) },
                        (unsafe { (*pOp).p2 }) as i64,
                    )
                };
                unsafe {
                    sqlite3VdbeMemSetInt64(
                        unsafe { pMem.offset((4 as i32) as isize) },
                        (unsafe { (*pOp).p3 }) as i64,
                    )
                };
                // pMem+5 for p4 is done last
                unsafe {
                    sqlite3VdbeMemSetInt64(
                        unsafe { pMem.offset((6 as i32) as isize) },
                        ((unsafe { (*pOp).p5 }) as u64) as i64,
                    )
                };
                let mut zCom: *mut i8 =
                    sqlite3VdbeDisplayComment(db, pOp as *const VdbeOp, zP4 as *const i8);
                unsafe {
                    sqlite3VdbeMemSetStr(
                        unsafe { pMem.offset((7 as i32) as isize) },
                        zCom as *const i8,
                        -(1 as i32) as i64,
                        ((1 as i32) as i8) as u8,
                        unsafe {
                            std::mem::transmute::<
                                *const (),
                                Option<unsafe extern "C-unwind" fn(*mut ())>,
                            >(sqlite3_free as *const ())
                        },
                    )
                };
                unsafe {
                    sqlite3VdbeMemSetStr(
                        unsafe { pMem.offset((5 as i32) as isize) },
                        zP4 as *const i8,
                        -(1 as i32) as i64,
                        ((1 as i32) as i8) as u8,
                        unsafe {
                            std::mem::transmute::<
                                *const (),
                                Option<unsafe extern "C-unwind" fn(*mut ())>,
                            >(sqlite3_free as *const ())
                        },
                    )
                };
                0 as i32;
            }
            unsafe {
                (*p).pResultRow = pMem;
            }
            if (unsafe { (*db).mallocFailed }) != (0 as u8) {
                unsafe {
                    (*p).rc = 7 as i32;
                }
                rc = 1 as i32;
            } else {
                unsafe {
                    (*p).rc = 0 as i32;
                }
                rc = 100 as i32;
            }
        }
    }
    return rc;
}

/// An instance of this object describes bulk memory available for use
/// by subcomponents of a prepared statement.  Space is allocated out
/// of a ReusableSpace object by the allocSpace() routine below.
#[repr(C)]
#[derive(Clone, Copy)]
struct ReusableSpace {
    /// Available memory
    pSpace: *mut u8,
    /// Bytes of available memory
    nFree: i64,
    /// Total bytes that could not be allocated
    nNeeded: i64,
}

/// Try to allocate nByte bytes of 8-byte aligned bulk memory for pBuf
/// from the ReusableSpace object.  Return a pointer to the allocated
/// memory on success.  If insufficient memory is available in the
/// ReusableSpace object, increase the ReusableSpace.nNeeded
/// value by the amount needed and return NULL.
///
/// If pBuf is not initially NULL, that means that the memory has already
/// been allocated by a prior call to this routine, so just return a copy
/// of pBuf and leave ReusableSpace unchanged.
///
/// This allocator is employed to repurpose unused slots at the end of the
/// opcode array of prepared state for other memory needs of the prepared
/// statement.
///
/// # Arguments
///
/// * `p` - Bulk memory available for allocation
/// * `pBuf` - Pointer to a prior allocation
/// * `nByte` - Bytes of memory needed.
fn allocSpace(mut p: *mut ReusableSpace, mut pBuf: *mut (), mut nByte: i64) -> *mut () {
    0 as i32;
    if pBuf == std::ptr::null_mut::<()>() {
        nByte = nByte;
        if nByte <= unsafe { (*p).nFree } {
            let __v1938: *mut ReusableSpace = p;
            let __v1939: i64 = unsafe { (*__v1938).nFree };
            let __v1940: i64 = __v1939 - nByte;
            unsafe {
                (*__v1938).nFree = __v1940;
            }
            pBuf = (unsafe { unsafe { (*p).pSpace }.offset((unsafe { (*p).nFree }) as isize) })
                as *mut ();
        } else {
            let __v1941: *mut ReusableSpace = p;
            let __v1942: i64 = unsafe { (*__v1941).nNeeded };
            let __v1943: i64 = __v1942 + nByte;
            unsafe {
                (*__v1941).nNeeded = __v1943;
            }
        }
    }
    0 as i32;
    return pBuf;
}

/// Rewind the VDBE back to the beginning in preparation for
/// running it.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3VdbeRewind(mut p: *mut Vdbe) {
    0 as i32;
    0 as i32;
    // There should be at least one opcode.
    0 as i32;
    unsafe {
        (*p).eVdbeState = ((1 as i32) as i8) as u8;
    }
    unsafe {
        (*p).pc = -(1 as i32);
    }
    unsafe {
        (*p).rc = 0 as i32;
    }
    unsafe {
        (*p).errorAction = ((2 as i32) as i8) as u8;
    }
    unsafe {
        (*p).nChange = (0 as i32) as i64;
    }
    unsafe {
        (*p).cacheCtr = (1 as i32) as u32;
    }
    unsafe {
        (*p).minWriteFileFormat = ((255 as i32) as i8) as u8;
    }
    unsafe {
        (*p).iStatement = 0 as i32;
    }
    unsafe {
        (*p).nFkConstraint = (0 as i32) as i64;
    }
}

/// Prepare a virtual machine for execution for the first time after
/// creating the virtual machine.  This involves things such
/// as allocating registers and initializing the program counter.
/// After the VDBE has be prepped, it can be executed by one or more
/// calls to sqlite3VdbeExec().
///
/// This function may be called exactly once on each virtual machine.
/// After this routine is called the VM has been "packaged" and is ready
/// to run.  After this routine is called, further calls to
/// sqlite3VdbeAddOp() functions are prohibited.  This routine disconnects
/// the Vdbe from the Parse object that helped generate it so that the
/// the Vdbe becomes an independent entity and the Parse object can be
/// destroyed.
///
/// Use the sqlite3VdbeRewind() procedure to restore a virtual machine back
/// to its initial state after it has been run.
///
/// # Arguments
///
/// * `p` - The VDBE
/// * `pParse` - Parsing context
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3VdbeMakeReady(mut p: *mut Vdbe, mut pParse: *mut Parse) {
    let mut db: *mut sqlite3 = unsafe { std::mem::zeroed() }; // The database connection
    let mut nVar: i32 = 0 as i32; // Number of parameters
    let mut nMem: i32 = 0 as i32; // Number of VM memory registers
    let mut nCursor: i32 = 0 as i32; // Number of cursors required
    let mut nArg: i32 = 0 as i32; // Max number args to xFilter or xUpdate
    let mut n: i32 = 0 as i32; // Loop counter
    let mut x: ReusableSpace = unsafe { std::mem::zeroed() }; // Reusable bulk memory
    0 as i32;
    0 as i32;
    0 as i32;
    0 as i32;
    0 as i32;
    0 as i32;
    unsafe {
        (*p).pVList = unsafe { (*pParse).pVList };
    }
    unsafe {
        (*pParse).pVList = std::ptr::null_mut::<i32>();
    }
    db = unsafe { (*p).db };
    0 as i32;
    nVar = (unsafe { (*pParse).nVar }) as i32;
    nMem = unsafe { (*pParse).nMem };
    nCursor = unsafe { (*pParse).nTab };
    nArg = unsafe { (*pParse).nMaxArg };
    // Each cursor uses a memory cell.  The first cursor (cursor 0) can
    // use aMem[0] which is not otherwise used by the VDBE program.  Allocate
    // space at the end of aMem[] for cursors 1 and greater.
    // See also: allocateCursor().
    let __v1794: i32 = nMem;
    let __v1795: i32 = __v1794 + nCursor;
    nMem = __v1795;
    if nCursor == (0 as i32) && nMem > (0 as i32) {
        let __v1796: i32 = nMem;
        let __v1797: i32 = __v1796 + (1 as i32);
        nMem = __v1797;
    }
    // Space for aMem[0] even if not used
    // Figure out how much reusable memory is available at the end of the
    // opcode array.  This extra memory will be reallocated for other elements
    // of the prepared statement.
    n = ((32 as u64).wrapping_mul(((unsafe { (*p).nOp }) as i64) as u64) as u32) as i32; // Bytes of opcode memory used
    x.pSpace = unsafe { ((unsafe { (*p).aOp }) as *mut u8).offset(n as isize) }; // Unused opcode memory
    0 as i32;
    x.nFree = (((((unsafe { (*p).nOpAlloc }) - unsafe { (*p).nOp }) as i64) as u64)
        .wrapping_mul(32 as u64)
        & ((!(7 as i32) as i64) as u64)) as i64; // Bytes unused mem
    0 as i32;
    0 as i32;
    resolveP2Values(p, std::ptr::addr_of_mut!(nArg));
    unsafe {
        (*p).__slate_bits_0.__set_usesStmtJournal(
            (((unsafe { (*pParse).isMultiWrite }) != (0 as u8)
                && ((unsafe { (*pParse).__slate_bits_0.__get_mayAbort() }) as i32) != (0 as i32))
                as u8) as u32,
        );
    }
    if (unsafe { (*pParse).explain }) != (0 as u8) {
        if nMem < (10 as i32) {
            nMem = 10 as i32;
        }
        unsafe {
            (*p).__slate_bits_0
                .__set_explain((unsafe { (*pParse).explain }) as u32);
        }
        unsafe {
            (*p).nResColumn = (((12 as i32)
                - (4 as i32) * ((unsafe { (*p).__slate_bits_0.__get_explain() }) as i32))
                as i16) as u16;
        }
    }
    unsafe {
        (*p).__slate_bits_0.__set_expired((0 as i32) as u32);
    }
    // Memory for registers, parameters, cursor, etc, is allocated in one or two
    // passes.  On the first pass, we try to reuse unused memory at the
    // end of the opcode array.  If we are unable to satisfy all memory
    // requirements by reusing the opcode array tail, then the second
    // pass will fill in the remainder using a fresh memory allocation.
    //
    // This two-pass approach that reuses as much memory as possible from
    // the leftover memory at the end of the opcode array.  This can significantly
    // reduce the amount of memory held by a prepared statement.
    x.nNeeded = (0 as i32) as i64;
    unsafe {
        (*p).aMem = allocSpace(
            std::ptr::addr_of_mut!(x),
            std::ptr::null_mut::<()>(),
            ((nMem as i64) as u64).wrapping_mul(56 as u64) as i64,
        ) as *mut sqlite3_value;
    }
    unsafe {
        (*p).aVar = allocSpace(
            std::ptr::addr_of_mut!(x),
            std::ptr::null_mut::<()>(),
            ((nVar as i64) as u64).wrapping_mul(56 as u64) as i64,
        ) as *mut sqlite3_value;
    }
    unsafe {
        (*p).apArg = allocSpace(
            std::ptr::addr_of_mut!(x),
            std::ptr::null_mut::<()>(),
            ((nArg as i64) as u64).wrapping_mul(8 as u64) as i64,
        ) as *mut *mut sqlite3_value;
    }
    unsafe {
        (*p).apCsr = allocSpace(
            std::ptr::addr_of_mut!(x),
            std::ptr::null_mut::<()>(),
            ((nCursor as i64) as u64).wrapping_mul(8 as u64) as i64,
        ) as *mut *mut VdbeCursor;
    }
    if x.nNeeded != (0 as i64) {
        let __v1798: *mut () = unsafe { sqlite3DbMallocRawNN(db, x.nNeeded as u64) };
        unsafe {
            (*p).pFree = __v1798;
        }
        x.pSpace = __v1798 as *mut u8;
        x.nFree = x.nNeeded;
        if !((unsafe { (*db).mallocFailed }) != (0 as u8)) {
            unsafe {
                (*p).aMem = allocSpace(
                    std::ptr::addr_of_mut!(x),
                    (unsafe { (*p).aMem }) as *mut (),
                    ((nMem as i64) as u64).wrapping_mul(56 as u64) as i64,
                ) as *mut sqlite3_value;
            }
            unsafe {
                (*p).aVar = allocSpace(
                    std::ptr::addr_of_mut!(x),
                    (unsafe { (*p).aVar }) as *mut (),
                    ((nVar as i64) as u64).wrapping_mul(56 as u64) as i64,
                ) as *mut sqlite3_value;
            }
            unsafe {
                (*p).apArg = allocSpace(
                    std::ptr::addr_of_mut!(x),
                    (unsafe { (*p).apArg }) as *mut (),
                    ((nArg as i64) as u64).wrapping_mul(8 as u64) as i64,
                ) as *mut *mut sqlite3_value;
            }
            unsafe {
                (*p).apCsr = allocSpace(
                    std::ptr::addr_of_mut!(x),
                    (unsafe { (*p).apCsr }) as *mut (),
                    ((nCursor as i64) as u64).wrapping_mul(8 as u64) as i64,
                ) as *mut *mut VdbeCursor;
            }
        }
    }
    if (unsafe { (*db).mallocFailed }) != (0 as u8) {
        unsafe {
            (*p).nVar = (0 as i32) as i16;
        }
        unsafe {
            (*p).nCursor = 0 as i32;
        }
        unsafe {
            (*p).nMem = 0 as i32;
        }
    } else {
        unsafe {
            (*p).nCursor = nCursor;
        }
        unsafe {
            (*p).nVar = nVar as i16;
        }
        initMemArray(unsafe { (*p).aVar }, nVar, db, ((1 as i32) as i16) as u16);
        unsafe {
            (*p).nMem = nMem;
        }
        initMemArray(unsafe { (*p).aMem }, nMem, db, ((0 as i32) as i16) as u16);
        unsafe {
            memset(
                (unsafe { (*p).apCsr }) as *mut (),
                0 as i32,
                ((nCursor as i64) as u64).wrapping_mul(8 as u64),
            )
        };
    }
    sqlite3VdbeRewind(p);
}

/// Close a VDBE cursor and release all the resources that cursor
/// happens to hold.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3VdbeFreeCursor(mut p: *mut Vdbe, mut pCx: *mut VdbeCursor) {
    if pCx != std::ptr::null_mut::<VdbeCursor>() {
        sqlite3VdbeFreeCursorNN(p, pCx);
    }
}

fn freeCursorWithCache(mut p: *mut Vdbe, mut pCx: *mut VdbeCursor) {
    let mut pCache: *mut VdbeTxtBlbCache = unsafe { (*pCx).pCache };
    0 as i32;
    unsafe {
        (*pCx).__slate_bits_0.__set_colCache((0 as i32) as u32);
    }
    unsafe {
        (*pCx).pCache = std::ptr::null_mut::<VdbeTxtBlbCache>();
    }
    if (unsafe { (*pCache).pCValue }) != std::ptr::null_mut::<i8>() {
        unsafe { sqlite3RCStrUnref((unsafe { (*pCache).pCValue }) as *mut ()) };
        unsafe {
            (*pCache).pCValue = std::ptr::null_mut::<i8>();
        }
    }
    unsafe { sqlite3DbFree(unsafe { (*p).db }, pCache as *mut ()) };
    sqlite3VdbeFreeCursorNN(p, pCx);
}

#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3VdbeFreeCursorNN(mut p: *mut Vdbe, mut pCx: *mut VdbeCursor) {
    if ((unsafe { (*pCx).__slate_bits_0.__get_colCache() }) as i32) != (0 as i32) {
        freeCursorWithCache(p, pCx);
        return;
    }
    '__slate_break_1717: {
        match ((unsafe { (*pCx).eCurType }) as u32) as i32 {
            1 => {
                unsafe { sqlite3VdbeSorterClose(unsafe { (*p).db }, pCx) };
            }
            0 => {
                0 as i32;
                unsafe { sqlite3BtreeCloseCursor(unsafe { (*pCx).uc.pCursor }) };
            }
            2 => {
                let mut pVCur: *mut sqlite3_vtab_cursor = unsafe { (*pCx).uc.pVCur };
                let mut pModule: *const sqlite3_module =
                    unsafe { (*unsafe { (*pVCur).pVtab }).pModule };
                0 as i32;
                let __v1843: *mut sqlite3_vtab = unsafe { (*pVCur).pVtab };
                let __v1844: i32 = unsafe { (*__v1843).nRef };
                let __v1845: i32 = __v1844 - (1 as i32);
                unsafe {
                    (*__v1843).nRef = __v1845;
                }
                unsafe { unsafe { (*pModule).xClose }.unwrap()(pVCur) };
            }
            _ => {}
        }
    }
}

/// Close all cursors in the current frame.
fn closeCursorsInFrame(mut p: *mut Vdbe) {
    let mut i: i32 = 0 as i32;
    i = 0 as i32;
    '__slate_break_1718: loop {
        if !(i < unsafe { (*p).nCursor }) {
            break;
        }
        let mut pC: *mut VdbeCursor =
            unsafe { *unsafe { unsafe { (*p).apCsr }.offset(i as isize) } };
        if pC != std::ptr::null_mut::<VdbeCursor>() {
            sqlite3VdbeFreeCursorNN(p, pC);
            unsafe {
                *unsafe { unsafe { (*p).apCsr }.offset(i as isize) } =
                    std::ptr::null_mut::<VdbeCursor>();
            }
        }
        let __v1944: i32 = i;
        let __v1945: i32 = __v1944 + (1 as i32);
        i = __v1945;
    }
}

/// Copy the values stored in the VdbeFrame structure to its Vdbe. This
/// is used, for example, when a trigger sub-program is halted to restore
/// control to the main program.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3VdbeFrameRestore(mut pFrame: *mut VdbeFrame) -> i32 {
    let mut v: *mut Vdbe = unsafe { (*pFrame).v };
    closeCursorsInFrame(v);
    unsafe {
        (*v).aOp = unsafe { (*pFrame).aOp };
    }
    unsafe {
        (*v).nOp = unsafe { (*pFrame).nOp };
    }
    unsafe {
        (*v).aMem = unsafe { (*pFrame).aMem };
    }
    unsafe {
        (*v).nMem = unsafe { (*pFrame).nMem };
    }
    unsafe {
        (*v).apCsr = unsafe { (*pFrame).apCsr };
    }
    unsafe {
        (*v).nCursor = unsafe { (*pFrame).nCursor };
    }
    unsafe {
        (*unsafe { (*v).db }).lastRowid = unsafe { (*pFrame).lastRowid };
    }
    unsafe {
        (*v).nChange = unsafe { (*pFrame).nChange };
    }
    unsafe {
        (*unsafe { (*v).db }).nChange = unsafe { (*pFrame).nDbChange };
    }
    sqlite3VdbeDeleteAuxData(
        unsafe { (*v).db },
        unsafe { std::ptr::addr_of_mut!((*v).pAuxData) },
        -(1 as i32),
        0 as i32,
    );
    unsafe {
        (*v).pAuxData = unsafe { (*pFrame).pAuxData };
    }
    unsafe {
        (*pFrame).pAuxData = std::ptr::null_mut::<AuxData>();
    }
    return unsafe { (*pFrame).pc };
}

/// Close all cursors.
///
/// Also release any dynamic memory held by the VM in the Vdbe.aMem memory
/// cell array. This is necessary as the memory cell array may contain
/// pointers to VdbeFrame objects, which may in turn contain pointers to
/// open cursors.
fn closeAllCursors(mut p: *mut Vdbe) {
    if (unsafe { (*p).pFrame }) != std::ptr::null_mut::<VdbeFrame>() {
        let mut pFrame: *mut VdbeFrame = unsafe { std::mem::zeroed() };
        pFrame = unsafe { (*p).pFrame };
        '__slate_break_1719: while (unsafe { (*pFrame).pParent })
            != std::ptr::null_mut::<VdbeFrame>()
        {
            {}
            pFrame = unsafe { (*pFrame).pParent };
        }
        sqlite3VdbeFrameRestore(pFrame);
        unsafe {
            (*p).pFrame = std::ptr::null_mut::<VdbeFrame>();
        }
        unsafe {
            (*p).nFrame = 0 as i32;
        }
    }
    0 as i32;
    closeCursorsInFrame(p);
    releaseMemArray(unsafe { (*p).aMem }, unsafe { (*p).nMem });
    '__slate_break_1720: while (unsafe { (*p).pDelFrame }) != std::ptr::null_mut::<VdbeFrame>() {
        let mut pDel: *mut VdbeFrame = unsafe { (*p).pDelFrame };
        unsafe {
            (*p).pDelFrame = unsafe { (*pDel).pParent };
        }
        sqlite3VdbeFrameDelete(pDel);
    }
    // Delete any auxdata allocations made by the VM
    if (unsafe { (*p).pAuxData }) != std::ptr::null_mut::<AuxData>() {
        sqlite3VdbeDeleteAuxData(
            unsafe { (*p).db },
            unsafe { std::ptr::addr_of_mut!((*p).pAuxData) },
            -(1 as i32),
            0 as i32,
        );
    }
    0 as i32;
}

/// Set the number of result columns that will be returned by this SQL
/// statement. This is now set at compile time, rather than during
/// execution of the vdbe program so that sqlite3_column_count() can
/// be called on an SQL statement before sqlite3_step().
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3VdbeSetNumCols(mut p: *mut Vdbe, mut nResColumn: i32) {
    let mut n: i32 = 0 as i32;
    let mut db: *mut sqlite3 = unsafe { (*p).db };
    if (unsafe { (*p).nResAlloc }) != (0 as u16) {
        releaseMemArray(
            unsafe { (*p).aColName },
            (((unsafe { (*p).nResAlloc }) as u32) as i32) * (2 as i32),
        );
        unsafe { sqlite3DbFree(db, (unsafe { (*p).aColName }) as *mut ()) };
    }
    n = nResColumn * (2 as i32);
    let __v1799: u16 = (nResColumn as i16) as u16;
    unsafe {
        (*p).nResAlloc = __v1799;
    }
    unsafe {
        (*p).nResColumn = __v1799;
    }
    unsafe {
        (*p).aColName =
            (unsafe { sqlite3DbMallocRawNN(db, (56 as u64).wrapping_mul((n as i64) as u64)) })
                as *mut sqlite3_value;
    }
    if (unsafe { (*p).aColName }) == std::ptr::null_mut::<sqlite3_value>() {
        return;
    }
    initMemArray(unsafe { (*p).aColName }, n, db, ((1 as i32) as i16) as u16);
}

/// Set the name of the idx'th column to be returned by the SQL statement.
/// zName must be a pointer to a nul terminated string.
///
/// This call must be made after a call to sqlite3VdbeSetNumCols().
///
/// The final parameter, xDel, must be one of SQLITE_DYNAMIC, SQLITE_STATIC
/// or SQLITE_TRANSIENT. If it is SQLITE_DYNAMIC, then the buffer pointed
/// to by zName will be freed by sqlite3DbFree() when the vdbe is destroyed.
///
/// # Arguments
///
/// * `p` - Vdbe being configured
/// * `idx` - Index of column zName applies to
/// * `var` - One of the COLNAME_* constants
/// * `zName` - Pointer to buffer containing name
/// * `xDel` - Memory management strategy for zName
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3VdbeSetColName(
    mut p: *mut Vdbe,
    mut idx: i32,
    mut var: i32,
    mut zName: *const i8,
    mut xDel: Option<unsafe extern "C-unwind" fn(*mut ())>,
) -> i32 {
    let mut rc: i32 = 0 as i32;
    let mut pColName: *mut sqlite3_value = unsafe { std::mem::zeroed() };
    0 as i32;
    0 as i32;
    if (unsafe { (*unsafe { (*p).db }).mallocFailed }) != (0 as u8) {
        0 as i32;
        return 7 as i32;
    }
    0 as i32;
    pColName = unsafe {
        unsafe { (*p).aColName }
            .offset((idx + var * (((unsafe { (*p).nResAlloc }) as u32) as i32)) as isize)
    };
    rc = unsafe { sqlite3VdbeMemSetText(pColName, zName, -(1 as i32) as i64, xDel) };
    0 as i32;
    return rc;
}

/// A read or write transaction may or may not be active on database handle
/// db. If a transaction is active, commit it. If there is a
/// write-transaction spanning more than one database file, this routine
/// takes care of the super-journal trickery.
fn vdbeCommit(mut db: *mut sqlite3, mut p: *mut Vdbe) -> i32 {
    let mut i: i32 = 0 as i32;
    let mut nTrans: i32 = 0 as i32;
    // Number of databases with an active write-transaction
    // that are candidates for a two-phase commit using a
    // super-journal
    let mut rc: i32 = 0 as i32;
    let mut needXcommit: i32 = 0 as i32;
    // Before doing anything else, call the xSync() callback for any
    // virtual module tables written in this transaction. This has to
    // be done before determining whether a super-journal file is
    // required, as an xSync() callback may add an attached database
    // to the transaction.
    rc = unsafe { sqlite3VtabSync(db, p) };
    // This loop determines (a) if the commit hook should be invoked and
    // (b) how many database files have open write transactions, not
    // including the temp database. (b) is important because if more than
    // one database file has an open write transaction, a super-journal
    // file is required for an atomic commit.
    i = 0 as i32;
    '__slate_break_1721: loop {
        if !(rc == (0 as i32) && i < unsafe { (*db).nDb }) {
            break;
        }
        let mut pBt: *mut Btree =
            unsafe { (*unsafe { unsafe { (*db).aDb }.offset(i as isize) }).pBt };
        if (unsafe { sqlite3BtreeTxnState(pBt) }) == (2 as i32) {
            // Whether or not a database might need a super-journal depends upon
            // its journal mode (among other things).  This matrix determines which
            // journal modes use a super-journal and which do not
            // DELETE
            // PERSIST
            // OFF
            // TRUNCATE
            // MEMORY
            // WAL
            let mut pPager: *mut Pager = unsafe { std::mem::zeroed() }; // Pager associated with pBt
            needXcommit = 1 as i32;
            unsafe { sqlite3BtreeEnter(pBt) };
            pPager = unsafe { sqlite3BtreePager(pBt) };
            let __v1948: bool;
            if (((unsafe { (*unsafe { unsafe { (*db).aDb }.offset(i as isize) }).safety_level })
                as u32) as i32)
                != (1 as i32)
            {
                __v1948 = (unsafe {
                    *unsafe {
                        unsafe { std::ptr::addr_of!(aMJNeeded) as *const u8 }
                            .offset((unsafe { sqlite3PagerGetJournalMode(pPager) }) as isize)
                    }
                }) != (0 as u8);
            } else {
                __v1948 = false as bool;
            }
            let __v1949: bool;
            if __v1948 {
                __v1949 = (unsafe { sqlite3PagerIsMemdb(pPager) }) == (0 as i32);
            } else {
                __v1949 = false as bool;
            }
            if __v1949 {
                0 as i32;
                let __v1950: i32 = nTrans;
                let __v1951: i32 = __v1950 + (1 as i32);
                nTrans = __v1951;
            }
            rc = unsafe { sqlite3PagerExclusiveLock(pPager) };
            unsafe { sqlite3BtreeLeave(pBt) };
        }
        let __v1946: i32 = i;
        let __v1947: i32 = __v1946 + (1 as i32);
        i = __v1947;
    }
    if rc != (0 as i32) {
        return rc;
    }
    // If there are any write-transactions at all, invoke the commit hook
    if needXcommit != (0 as i32) && (unsafe { (*db).xCommitCallback }) != None {
        rc = unsafe { unsafe { (*db).xCommitCallback }.unwrap()(unsafe { (*db).pCommitArg }) };
        if rc != (0 as i32) {
            return (19 as i32) | (2 as i32) << (8 as i32);
        }
    }
    // The simple case - no more than one database file (not counting the
    // TEMP database) has a transaction active.   There is no need for the
    // super-journal.
    //
    // If the return value of sqlite3BtreeGetFilename() is a zero length
    // string, it means the main database is :memory: or a temp file.  In
    // that case we do not support atomic multi-file commits, so use the
    // simple case then too.
    if (0 as i32)
        == unsafe {
            sqlite3Strlen30(unsafe {
                sqlite3BtreeGetFilename(unsafe {
                    (*unsafe { unsafe { (*db).aDb }.offset((0 as i32) as isize) }).pBt
                })
            })
        }
        || nTrans <= (1 as i32)
    {
        if needXcommit != (0 as i32) {
            i = 0 as i32;
            '__slate_break_1722: loop {
                if !(rc == (0 as i32) && i < unsafe { (*db).nDb }) {
                    break;
                }
                let mut pBt: *mut Btree =
                    unsafe { (*unsafe { unsafe { (*db).aDb }.offset(i as isize) }).pBt };
                if (unsafe { sqlite3BtreeTxnState(pBt) }) >= (2 as i32) {
                    rc = unsafe { sqlite3BtreeCommitPhaseOne(pBt, std::ptr::null::<i8>()) };
                }
                let __v1952: i32 = i;
                let __v1953: i32 = __v1952 + (1 as i32);
                i = __v1953;
            }
        }
        // Do the commit only if all databases successfully complete phase 1.
        // If one of the BtreeCommitPhaseOne() calls fails, this indicates an
        // IO error while deleting or truncating a journal file. It is unlikely,
        // but could happen. In this case abandon processing and return the error.
        i = 0 as i32;
        '__slate_break_1723: loop {
            if !(rc == (0 as i32) && i < unsafe { (*db).nDb }) {
                break;
            }
            let mut pBt: *mut Btree =
                unsafe { (*unsafe { unsafe { (*db).aDb }.offset(i as isize) }).pBt };
            let mut txn: i32 = unsafe { sqlite3BtreeTxnState(pBt) };
            if txn != (0 as i32) {
                0 as i32;
                rc = unsafe { sqlite3BtreeCommitPhaseTwo(pBt, 0 as i32) };
            }
            let __v1954: i32 = i;
            let __v1955: i32 = __v1954 + (1 as i32);
            i = __v1955;
        }
        if rc == (0 as i32) {
            unsafe { sqlite3VtabCommit(db) };
        }
    } else {
        let mut pVfs: *mut sqlite3_vfs = unsafe { (*db).pVfs };
        let mut zSuper: *mut i8 = std::ptr::null_mut::<i8>(); // File-name for the super-journal
        let mut zMainFile: *const i8 = unsafe {
            sqlite3BtreeGetFilename(unsafe {
                (*unsafe { unsafe { (*db).aDb }.offset((0 as i32) as isize) }).pBt
            })
        };
        let mut pSuperJrnl: *mut sqlite3_file = std::ptr::null_mut::<sqlite3_file>();
        let mut offset: i64 = (0 as i32) as i64;
        let mut res: i32 = 0 as i32;
        let mut retryCount: i32 = 0 as i32;
        let mut nMainFile: i32 = 0 as i32;
        // Select a super-journal file name
        nMainFile = unsafe { sqlite3Strlen30(zMainFile) };
        zSuper = unsafe {
            sqlite3MPrintf(
                db,
                (b"%.4c%s%.16c\0".as_ptr() as *mut i8) as *const i8,
                0 as i32,
                zMainFile,
                0 as i32,
            )
        };
        if zSuper == std::ptr::null_mut::<i8>() {
            return 7 as i32;
        }
        let __v1956: *mut i8 = zSuper;
        let __v1957: *mut i8 = unsafe { __v1956.offset((4 as i32) as isize) };
        zSuper = __v1957;
        '__slate_break_1725: loop {
            let mut iRandom: u32 = 0 as u32;
            if retryCount != (0 as i32) {
                if retryCount > (100 as i32) {
                    unsafe {
                        sqlite3_log(
                            13 as i32,
                            (b"MJ delete: %s\0".as_ptr() as *mut i8) as *const i8,
                            zSuper,
                        )
                    };
                    unsafe { sqlite3OsDelete(pVfs, zSuper as *const i8, 0 as i32) };
                    break '__slate_break_1725;
                } else {
                    if retryCount == (1 as i32) {
                        unsafe {
                            sqlite3_log(
                                13 as i32,
                                (b"MJ collide: %s\0".as_ptr() as *mut i8) as *const i8,
                                zSuper,
                            )
                        };
                    }
                }
            }
            let __v1958: i32 = retryCount;
            let __v1959: i32 = __v1958 + (1 as i32);
            retryCount = __v1959;
            unsafe {
                sqlite3_randomness(
                    ((4 as u64) as u32) as i32,
                    std::ptr::addr_of_mut!(iRandom) as *mut (),
                )
            };
            unsafe {
                sqlite3_snprintf(
                    13 as i32,
                    unsafe { zSuper.offset(nMainFile as isize) },
                    (b"-mj%06X9%02X\0".as_ptr() as *mut i8) as *const i8,
                    iRandom >> (8 as i32) & ((16777215 as i32) as u32),
                    iRandom & ((255 as i32) as u32),
                )
            };
            // The antipenultimate character of the super-journal name must
            // be "9" to avoid name collisions when using 8+3 filenames.
            0 as i32;
            {}
            rc = unsafe {
                sqlite3OsAccess(
                    pVfs,
                    zSuper as *const i8,
                    0 as i32,
                    std::ptr::addr_of_mut!(res),
                )
            };
            if !(rc == (0 as i32) && res != (0 as i32)) {
                break;
            }
        }
        if rc == (0 as i32) {
            // Open the super-journal.
            rc = unsafe {
                sqlite3OsOpenMalloc(
                    pVfs,
                    zSuper as *const i8,
                    std::ptr::addr_of_mut!(pSuperJrnl),
                    (2 as i32) | (4 as i32) | (16 as i32) | (16384 as i32),
                    std::ptr::null_mut::<i32>(),
                )
            };
        }
        if rc != (0 as i32) {
            unsafe {
                sqlite3DbFree(
                    db,
                    (unsafe { zSuper.offset(-((4 as i32) as isize)) }) as *mut (),
                )
            };
            return rc;
        }
        // Write the name of each database file in the transaction into the new
        // super-journal file. If an error occurs at this point close
        // and delete the super-journal file. All the individual journal files
        // still have 'null' as the super-journal pointer, so they will roll
        // back independently if a failure occurs.
        i = 0 as i32;
        '__slate_break_1729: loop {
            if !(i < unsafe { (*db).nDb }) {
                break;
            }
            '__slate_continue_1729: {
                let mut pBt: *mut Btree =
                    unsafe { (*unsafe { unsafe { (*db).aDb }.offset(i as isize) }).pBt };
                if (unsafe { sqlite3BtreeTxnState(pBt) }) == (2 as i32) {
                    let mut zFile: *const i8 = unsafe { sqlite3BtreeGetJournalname(pBt) };
                    if zFile == std::ptr::null::<i8>() {
                        break '__slate_continue_1729; // Ignore TEMP and :memory: databases
                    }
                    0 as i32;
                    rc = unsafe {
                        sqlite3OsWrite(
                            pSuperJrnl,
                            zFile as *const (),
                            (unsafe { sqlite3Strlen30(zFile) }) + (1 as i32),
                            offset,
                        )
                    };
                    let __v1962: i64 = offset;
                    let __v1963: i64 =
                        __v1962 + (((unsafe { sqlite3Strlen30(zFile) }) + (1 as i32)) as i64);
                    offset = __v1963;
                    if rc != (0 as i32) {
                        unsafe { sqlite3OsCloseFree(pSuperJrnl) };
                        unsafe { sqlite3OsDelete(pVfs, zSuper as *const i8, 0 as i32) };
                        unsafe {
                            sqlite3DbFree(
                                db,
                                (unsafe { zSuper.offset(-((4 as i32) as isize)) }) as *mut (),
                            )
                        };
                        return rc;
                    }
                }
            }
            let __v1960: i32 = i;
            let __v1961: i32 = __v1960 + (1 as i32);
            i = __v1961;
        }
        // Sync the super-journal file. If the IOCAP_SEQUENTIAL device
        // flag is set this is not required.
        let __v1964: bool;
        if (0 as i32) == (unsafe { sqlite3OsDeviceCharacteristics(pSuperJrnl) }) & (1024 as i32) {
            let __v1965: i32 = unsafe { sqlite3OsSync(pSuperJrnl, 2 as i32) };
            rc = __v1965;
            __v1964 = (0 as i32) != __v1965;
        } else {
            __v1964 = false as bool;
        }
        if __v1964 {
            unsafe { sqlite3OsCloseFree(pSuperJrnl) };
            unsafe { sqlite3OsDelete(pVfs, zSuper as *const i8, 0 as i32) };
            unsafe {
                sqlite3DbFree(
                    db,
                    (unsafe { zSuper.offset(-((4 as i32) as isize)) }) as *mut (),
                )
            };
            return rc;
        }
        // Sync all the db files involved in the transaction. The same call
        // sets the super-journal pointer in each individual journal. If
        // an error occurs here, do not delete the super-journal file.
        //
        // If the error occurs during the first call to
        // sqlite3BtreeCommitPhaseOne(), then there is a chance that the
        // super-journal file will be orphaned. But we cannot delete it,
        // in case the super-journal file name was written into the journal
        // file before the failure occurred.
        i = 0 as i32;
        '__slate_break_1730: loop {
            if !(rc == (0 as i32) && i < unsafe { (*db).nDb }) {
                break;
            }
            let mut pBt: *mut Btree =
                unsafe { (*unsafe { unsafe { (*db).aDb }.offset(i as isize) }).pBt };
            if pBt != std::ptr::null_mut::<Btree>() {
                rc = unsafe { sqlite3BtreeCommitPhaseOne(pBt, zSuper as *const i8) };
            }
            let __v1966: i32 = i;
            let __v1967: i32 = __v1966 + (1 as i32);
            i = __v1967;
        }
        unsafe { sqlite3OsCloseFree(pSuperJrnl) };
        0 as i32;
        if rc != (0 as i32) {
            unsafe {
                sqlite3DbFree(
                    db,
                    (unsafe { zSuper.offset(-((4 as i32) as isize)) }) as *mut (),
                )
            };
            return rc;
        }
        // Delete the super-journal file. This commits the transaction. After
        // doing this the directory is synced again before any individual
        // transaction files are deleted.
        rc = unsafe { sqlite3OsDelete(pVfs, zSuper as *const i8, 1 as i32) };
        unsafe {
            sqlite3DbFree(
                db,
                (unsafe { zSuper.offset(-((4 as i32) as isize)) }) as *mut (),
            )
        };
        zSuper = std::ptr::null_mut::<i8>();
        if rc != (0 as i32) {
            return rc;
        }
        // All files and directories have already been synced, so the following
        // calls to sqlite3BtreeCommitPhaseTwo() are only closing files and
        // deleting or truncating journals. If something goes wrong while
        // this is happening we don't really care. The integrity of the
        // transaction is already guaranteed, but some stray 'cold' journals
        // may be lying around. Returning an error code won't help matters.
        {}
        unsafe { sqlite3BeginBenignMalloc() };
        i = 0 as i32;
        '__slate_break_1731: loop {
            if !(i < unsafe { (*db).nDb }) {
                break;
            }
            let mut pBt: *mut Btree =
                unsafe { (*unsafe { unsafe { (*db).aDb }.offset(i as isize) }).pBt };
            if pBt != std::ptr::null_mut::<Btree>() {
                unsafe { sqlite3BtreeCommitPhaseTwo(pBt, 1 as i32) };
            }
            let __v1968: i32 = i;
            let __v1969: i32 = __v1968 + (1 as i32);
            i = __v1969;
        }
        unsafe { sqlite3EndBenignMalloc() };
        {}
        unsafe { sqlite3VtabCommit(db) };
    }
    // The complex case - There is a multi-file write-transaction active.
    // This requires a super-journal file to ensure the transaction is
    // committed atomically.
    return rc;
}

static mut aMJNeeded: [u8; 6] = [
    ((1 as i32) as i8) as u8,
    ((1 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((1 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
];

/// This routine checks that the sqlite3.nVdbeActive count variable
/// matches the number of vdbe's in the list sqlite3.pVdbe that are
/// currently active. An assertion fails if the two counts do not match.
/// This is an internal self-check only - it is not an essential processing
/// step.
///
/// This is a no-op if NDEBUG is defined.
/// If the Vdbe passed as the first argument opened a statement-transaction,
/// close it now. Argument eOp must be either SAVEPOINT_ROLLBACK or
/// SAVEPOINT_RELEASE. If it is SAVEPOINT_ROLLBACK, then the statement
/// transaction is rolled back. If eOp is SAVEPOINT_RELEASE, then the
/// statement transaction is committed.
///
/// If an IO error occurs, an SQLITE_IOERR_XXX error code is returned.
/// Otherwise SQLITE_OK.
fn vdbeCloseStatement(mut p: *mut Vdbe, mut eOp: i32) -> i32 {
    let mut db: *mut sqlite3 = unsafe { (*p).db };
    let mut rc: i32 = 0 as i32;
    let mut i: i32 = 0 as i32;
    let mut iSavepoint: i32 = (unsafe { (*p).iStatement }) - (1 as i32);
    0 as i32;
    0 as i32;
    0 as i32;
    i = 0 as i32;
    '__slate_break_1732: loop {
        if !(i < unsafe { (*db).nDb }) {
            break;
        }
        let mut rc2: i32 = 0 as i32;
        let mut pBt: *mut Btree =
            unsafe { (*unsafe { unsafe { (*db).aDb }.offset(i as isize) }).pBt };
        if pBt != std::ptr::null_mut::<Btree>() {
            if eOp == (2 as i32) {
                rc2 = unsafe { sqlite3BtreeSavepoint(pBt, 2 as i32, iSavepoint) };
            }
            if rc2 == (0 as i32) {
                rc2 = unsafe { sqlite3BtreeSavepoint(pBt, 1 as i32, iSavepoint) };
            }
            if rc == (0 as i32) {
                rc = rc2;
            }
        }
        let __v1970: i32 = i;
        let __v1971: i32 = __v1970 + (1 as i32);
        i = __v1971;
    }
    let __v1972: *mut sqlite3 = db;
    let __v1973: i32 = unsafe { (*__v1972).nStatement };
    let __v1974: i32 = __v1973 - (1 as i32);
    unsafe {
        (*__v1972).nStatement = __v1974;
    }
    unsafe {
        (*p).iStatement = 0 as i32;
    }
    if rc == (0 as i32) {
        if eOp == (2 as i32) {
            rc = unsafe { sqlite3VtabSavepoint(db, 2 as i32, iSavepoint) };
        }
        if rc == (0 as i32) {
            rc = unsafe { sqlite3VtabSavepoint(db, 1 as i32, iSavepoint) };
        }
    }
    // If the statement transaction is being rolled back, also restore the
    // database handles deferred constraint counter to the value it had when
    // the statement transaction was opened.
    if eOp == (2 as i32) {
        unsafe {
            (*db).nDeferredCons = unsafe { (*p).nStmtDefCons };
        }
        unsafe {
            (*db).nDeferredImmCons = unsafe { (*p).nStmtDefImmCons };
        }
    }
    return rc;
}

#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3VdbeCloseStatement(mut p: *mut Vdbe, mut eOp: i32) -> i32 {
    if (unsafe { (*unsafe { (*p).db }).nStatement }) != (0 as i32)
        && (unsafe { (*p).iStatement }) != (0 as i32)
    {
        return vdbeCloseStatement(p, eOp);
    }
    return 0 as i32;
}

/// These functions are called when a transaction opened by the database
/// handle associated with the VM passed as an argument is about to be
/// committed. If there are outstanding foreign key constraint violations
/// return an error code. Otherwise, SQLITE_OK.
///
/// If there are outstanding FK violations and this function returns
/// non-zero, set the result of the VM to SQLITE_CONSTRAINT_FOREIGNKEY
/// and write an error message to it.
fn vdbeFkError(mut p: *mut Vdbe) -> i32 {
    unsafe {
        (*p).rc = (19 as i32) | (3 as i32) << (8 as i32);
    }
    unsafe {
        (*p).errorAction = ((2 as i32) as i8) as u8;
    }
    unsafe {
        sqlite3VdbeError(
            p,
            (b"FOREIGN KEY constraint failed\0".as_ptr() as *mut i8) as *const i8,
        )
    };
    if (((unsafe { (*p).prepFlags }) as u32) as i32) & (128 as i32) == (0 as i32) {
        return 1 as i32;
    }
    return (19 as i32) | (3 as i32) << (8 as i32);
}

#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3VdbeCheckFkImmediate(mut p: *mut Vdbe) -> i32 {
    if (unsafe { (*p).nFkConstraint }) == ((0 as i32) as i64) {
        return 0 as i32;
    }
    return vdbeFkError(p);
}

#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3VdbeCheckFkDeferred(mut p: *mut Vdbe) -> i32 {
    let mut db: *mut sqlite3 = unsafe { (*p).db };
    if (unsafe { (*db).nDeferredCons }) + unsafe { (*db).nDeferredImmCons } == ((0 as i32) as i64) {
        return 0 as i32;
    }
    return vdbeFkError(p);
}

/// This routine is called the when a VDBE tries to halt.  If the VDBE
/// has made changes and is in autocommit mode, then commit those
/// changes.  If a rollback is needed, then do the rollback.
///
/// This routine is the only way to move the sqlite3eOpenState of a VM from
/// SQLITE_STATE_RUN to SQLITE_STATE_HALT.  It is harmless to
/// call this on a VM that is in the SQLITE_STATE_HALT state.
///
/// Return an error code.  If the commit could not complete because of
/// lock contention, return SQLITE_BUSY.  If SQLITE_BUSY is returned, it
/// means the close did not happen and needs to be repeated.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3VdbeHalt(mut p: *mut Vdbe) -> i32 {
    let mut rc: i32 = 0 as i32; // Used to store transient return codes
    let mut db: *mut sqlite3 = unsafe { (*p).db };
    // This function contains the logic that determines if a statement or
    // transaction will be committed or rolled back as a result of the
    // execution of this virtual machine.
    //
    // If any of the following errors occur:
    //
    //     SQLITE_NOMEM
    //     SQLITE_IOERR
    //     SQLITE_FULL
    //     SQLITE_INTERRUPT
    //
    // Then the internal cache might have been left in an inconsistent
    // state.  We need to rollback the statement transaction, if there is
    // one, or the complete transaction if there is no statement transaction.
    0 as i32;
    if (unsafe { (*db).mallocFailed }) != (0 as u8) {
        unsafe {
            (*p).rc = 7 as i32;
        }
    }
    closeAllCursors(p);
    {}
    // No commit or rollback needed if the program never started or if the
    // SQL statement does not read or write a database file.
    if ((unsafe { (*p).__slate_bits_0.__get_bIsReader() }) as i32) != (0 as i32) {
        let mut mrc: i32 = 0 as i32; // Primary error code from p->rc
        let mut eStatementOp: i32 = 0 as i32;
        let mut isSpecialError: i32 = 0 as i32; // Set to true if a 'special' error
        // Lock all btrees used by the statement
        sqlite3VdbeEnter(p);
        // Check for one of the special errors
        if (unsafe { (*p).rc }) != (0 as i32) {
            mrc = (unsafe { (*p).rc }) & (255 as i32);
            isSpecialError = (mrc == (7 as i32)
                || mrc == (10 as i32)
                || mrc == (9 as i32)
                || mrc == (13 as i32)) as i32;
        } else {
            isSpecialError = 0 as i32;
            mrc = 0 as i32;
        }
        if isSpecialError != (0 as i32) {
            // If the query was read-only and the error code is SQLITE_INTERRUPT,
            // no rollback is necessary. Otherwise, at least a savepoint
            // transaction must be rolled back to restore the database to a
            // consistent state.
            //
            // Even if the statement is read-only, it is important to perform
            // a statement or transaction rollback operation. If the error
            // occurred while writing to the journal, sub-journal or database
            // file as part of an effort to free up cache space (see function
            // pagerStress() in pager.c), the rollback is required to restore
            // the pager to a consistent state.
            if !(((unsafe { (*p).__slate_bits_0.__get_readOnly() }) as i32) != (0 as i32))
                || mrc != (9 as i32)
            {
                if (mrc == (7 as i32) || mrc == (13 as i32))
                    && ((unsafe { (*p).__slate_bits_0.__get_usesStmtJournal() }) as i32)
                        != (0 as i32)
                {
                    eStatementOp = 2 as i32;
                } else {
                    // We are forced to roll back the active transaction. Before doing
                    // so, abort any other statements this handle currently has active.
                    unsafe { sqlite3RollbackAll(db, (4 as i32) | (2 as i32) << (8 as i32)) };
                    unsafe { sqlite3CloseSavepoints(db) };
                    unsafe {
                        (*db).autoCommit = ((1 as i32) as i8) as u8;
                    }
                    unsafe {
                        (*p).nChange = (0 as i32) as i64;
                    }
                }
            }
        }
        // Check for immediate foreign key violations.
        if (unsafe { (*p).rc }) == (0 as i32)
            || (((unsafe { (*p).errorAction }) as u32) as i32) == (3 as i32)
                && !(isSpecialError != (0 as i32))
        {
            sqlite3VdbeCheckFkImmediate(p);
        }
        // If the auto-commit flag is set and this is the only active writer
        // VM, then we do either a commit or rollback of the current transaction.
        //
        // Note: This block also runs if one of the special errors handled
        // above has occurred.
        if !((unsafe { (*db).nVTrans }) > (0 as i32)
            && (unsafe { (*db).aVTrans }) == std::ptr::null_mut::<*mut VTable>())
            && (unsafe { (*db).autoCommit }) != (0 as u8)
            && (unsafe { (*db).nVdbeWrite })
                == ((((unsafe { (*p).__slate_bits_0.__get_readOnly() }) as i32) == (0 as i32))
                    as i32)
        {
            if (unsafe { (*p).rc }) == (0 as i32)
                || (((unsafe { (*p).errorAction }) as u32) as i32) == (3 as i32)
                    && !(isSpecialError != (0 as i32))
            {
                rc = sqlite3VdbeCheckFkDeferred(p);
                if rc != (0 as i32) {
                    if ((unsafe { (*p).__slate_bits_0.__get_readOnly() }) as i32) != (0 as i32) {
                        sqlite3VdbeLeave(p);
                        return 1 as i32;
                    }
                    rc = (19 as i32) | (3 as i32) << (8 as i32);
                } else {
                    if (unsafe { (*db).flags }) & (((2 as i32) as i64) as u64) << (32 as i32)
                        != (0 as u64)
                    {
                        rc = 11 as i32;
                        let __v1891: *mut sqlite3 = db;
                        let __v1892: u64 = unsafe { (*__v1891).flags };
                        let __v1893: u64 = __v1892 & !((((2 as i32) as i64) as u64) << (32 as i32));
                        unsafe {
                            (*__v1891).flags = __v1893;
                        }
                    } else {
                        // The auto-commit flag is true, the vdbe program was successful
                        // or hit an 'OR FAIL' constraint and there are no deferred foreign
                        // key constraints to hold up the transaction. This means a commit
                        // is required.
                        rc = vdbeCommit(db, p);
                    }
                }
                if rc == (5 as i32)
                    && ((unsafe { (*p).__slate_bits_0.__get_readOnly() }) as i32) != (0 as i32)
                {
                    sqlite3VdbeLeave(p);
                    return 5 as i32;
                } else {
                    if rc != (0 as i32) {
                        unsafe { sqlite3SystemError(db, rc) };
                        unsafe {
                            (*p).rc = rc;
                        }
                        unsafe { sqlite3RollbackAll(db, 0 as i32) };
                        unsafe {
                            (*p).nChange = (0 as i32) as i64;
                        }
                    } else {
                        unsafe {
                            (*db).nDeferredCons = (0 as i32) as i64;
                        }
                        unsafe {
                            (*db).nDeferredImmCons = (0 as i32) as i64;
                        }
                        let __v1894: *mut sqlite3 = db;
                        let __v1895: u64 = unsafe { (*__v1894).flags };
                        let __v1896: u64 = __v1895 & !(((524288 as i32) as i64) as u64);
                        unsafe {
                            (*__v1894).flags = __v1896;
                        }
                        unsafe { sqlite3CommitInternalChanges(db) };
                    }
                }
            } else {
                if (unsafe { (*p).rc }) == (17 as i32)
                    && (unsafe { (*db).nVdbeActive }) > (1 as i32)
                {
                    unsafe {
                        (*p).nChange = (0 as i32) as i64;
                    }
                } else {
                    unsafe { sqlite3RollbackAll(db, 0 as i32) };
                    unsafe {
                        (*p).nChange = (0 as i32) as i64;
                    }
                }
            }
            unsafe {
                (*db).nStatement = 0 as i32;
            }
        } else {
            if eStatementOp == (0 as i32) {
                if (unsafe { (*p).rc }) == (0 as i32)
                    || (((unsafe { (*p).errorAction }) as u32) as i32) == (3 as i32)
                {
                    eStatementOp = 1 as i32;
                } else {
                    if (((unsafe { (*p).errorAction }) as u32) as i32) == (2 as i32) {
                        eStatementOp = 2 as i32;
                    } else {
                        unsafe { sqlite3RollbackAll(db, (4 as i32) | (2 as i32) << (8 as i32)) };
                        unsafe { sqlite3CloseSavepoints(db) };
                        unsafe {
                            (*db).autoCommit = ((1 as i32) as i8) as u8;
                        }
                        unsafe {
                            (*p).nChange = (0 as i32) as i64;
                        }
                    }
                }
            }
        }
        // If eStatementOp is non-zero, then a statement transaction needs to
        // be committed or rolled back. Call sqlite3VdbeCloseStatement() to
        // do so. If this operation returns an error, and the current statement
        // error code is SQLITE_OK or SQLITE_CONSTRAINT, then promote the
        // current statement error code.
        if eStatementOp != (0 as i32) {
            rc = sqlite3VdbeCloseStatement(p, eStatementOp);
            if rc != (0 as i32) {
                if (unsafe { (*p).rc }) == (0 as i32)
                    || (unsafe { (*p).rc }) & (255 as i32) == (19 as i32)
                {
                    unsafe {
                        (*p).rc = rc;
                    }
                    unsafe { sqlite3DbFree(db, (unsafe { (*p).zErrMsg }) as *mut ()) };
                    unsafe {
                        (*p).zErrMsg = std::ptr::null_mut::<i8>();
                    }
                }
                unsafe { sqlite3RollbackAll(db, (4 as i32) | (2 as i32) << (8 as i32)) };
                unsafe { sqlite3CloseSavepoints(db) };
                unsafe {
                    (*db).autoCommit = ((1 as i32) as i8) as u8;
                }
                unsafe {
                    (*p).nChange = (0 as i32) as i64;
                }
            }
        }
        // If this was an INSERT, UPDATE or DELETE and no statement transaction
        // has been rolled back, update the database connection change-counter.
        if ((unsafe { (*p).__slate_bits_0.__get_changeCntOn() }) as i32) != (0 as i32) {
            if eStatementOp != (2 as i32) {
                sqlite3VdbeSetChanges(db, unsafe { (*p).nChange });
            } else {
                sqlite3VdbeSetChanges(db, (0 as i32) as i64);
            }
            unsafe {
                (*p).nChange = (0 as i32) as i64;
            }
        }
        // Release the locks
        sqlite3VdbeLeave(p);
    }
    // We have successfully halted and closed the VM.  Record this fact.
    let __v1897: *mut sqlite3 = db;
    let __v1898: i32 = unsafe { (*__v1897).nVdbeActive };
    let __v1899: i32 = __v1898 - (1 as i32);
    unsafe {
        (*__v1897).nVdbeActive = __v1899;
    }
    if !(((unsafe { (*p).__slate_bits_0.__get_readOnly() }) as i32) != (0 as i32)) {
        let __v1900: *mut sqlite3 = db;
        let __v1901: i32 = unsafe { (*__v1900).nVdbeWrite };
        let __v1902: i32 = __v1901 - (1 as i32);
        unsafe {
            (*__v1900).nVdbeWrite = __v1902;
        }
    }
    if ((unsafe { (*p).__slate_bits_0.__get_bIsReader() }) as i32) != (0 as i32) {
        let __v1903: *mut sqlite3 = db;
        let __v1904: i32 = unsafe { (*__v1903).nVdbeRead };
        let __v1905: i32 = __v1904 - (1 as i32);
        unsafe {
            (*__v1903).nVdbeRead = __v1905;
        }
    }
    0 as i32;
    0 as i32;
    0 as i32;
    unsafe {
        (*p).eVdbeState = ((3 as i32) as i8) as u8;
    }
    {}
    if (unsafe { (*db).mallocFailed }) != (0 as u8) {
        unsafe {
            (*p).rc = 7 as i32;
        }
    }
    // If the auto-commit flag is set to true, then any locks that were held
    // by connection db have now been released. Call sqlite3ConnectionUnlocked()
    // to invoke any required unlock-notify callbacks.
    if (unsafe { (*db).autoCommit }) != (0 as u8) {
        {}
    }
    0 as i32;
    return if (unsafe { (*p).rc }) == (5 as i32) {
        5 as i32
    } else {
        0 as i32
    };
}

/// Each VDBE holds the result of the most recent sqlite3_step() call
/// in p->rc.  This routine sets that result back to SQLITE_OK.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3VdbeResetStepResult(mut p: *mut Vdbe) {
    unsafe {
        (*p).rc = 0 as i32;
    }
}

/// Copy the error code and error message belonging to the VDBE passed
/// as the first argument to its database handle (so that they will be
/// returned by calls to sqlite3_errcode() and sqlite3_errmsg()).
///
/// This function does not clear the VDBE error code or message, just
/// copies them to the database handle.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3VdbeTransferError(mut p: *mut Vdbe) -> i32 {
    let mut db: *mut sqlite3 = unsafe { (*p).db };
    let mut rc: i32 = unsafe { (*p).rc };
    if (unsafe { (*p).zErrMsg }) != std::ptr::null_mut::<i8>() {
        let __v1908: *mut sqlite3 = db;
        let __v1909: u8 = unsafe { (*__v1908).bBenignMalloc };
        let __v1910: u8 = ((((__v1909 as u32) as i32) + (1 as i32)) as i8) as u8;
        unsafe {
            (*__v1908).bBenignMalloc = __v1910;
        }
        unsafe { sqlite3BeginBenignMalloc() };
        if (unsafe { (*db).pErr }) == std::ptr::null_mut::<sqlite3_value>() {
            unsafe {
                (*db).pErr = unsafe { sqlite3ValueNew(db) };
            }
        }
        unsafe {
            sqlite3ValueSetStr(
                unsafe { (*db).pErr },
                -(1 as i32),
                (unsafe { (*p).zErrMsg }) as *const (),
                ((1 as i32) as i8) as u8,
                unsafe {
                    std::mem::transmute::<usize, Option<unsafe extern "C-unwind" fn(*mut ())>>(
                        -(1 as i32) as usize,
                    )
                },
            )
        };
        unsafe { sqlite3EndBenignMalloc() };
        let __v1911: *mut sqlite3 = db;
        let __v1912: u8 = unsafe { (*__v1911).bBenignMalloc };
        let __v1913: u8 = ((((__v1912 as u32) as i32) - (1 as i32)) as i8) as u8;
        unsafe {
            (*__v1911).bBenignMalloc = __v1913;
        }
    } else {
        if (unsafe { (*db).pErr }) != std::ptr::null_mut::<sqlite3_value>() {
            unsafe { sqlite3ValueSetNull(unsafe { (*db).pErr }) };
        }
    }
    unsafe {
        (*db).errCode = rc;
    }
    unsafe {
        (*db).errByteOffset = -(1 as i32);
    }
    return rc;
}

/// Clean up a VDBE after execution but do not delete the VDBE just yet.
/// Write any error messages into *pzErrMsg.  Return the result code.
///
/// After this routine is run, the VDBE should be ready to be executed
/// again.
///
/// To look at it another way, this routine resets the state of the
/// virtual machine from VDBE_RUN_STATE or VDBE_HALT_STATE back to
/// VDBE_READY_STATE.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3VdbeReset(mut p: *mut Vdbe) -> i32 {
    let mut db: *mut sqlite3 = unsafe { std::mem::zeroed() };
    db = unsafe { (*p).db };
    // If the VM did not run to completion or if it encountered an
    // error, then it might not have been halted properly.  So halt
    // it now.
    if (((unsafe { (*p).eVdbeState }) as u32) as i32) == (2 as i32) {
        sqlite3VdbeHalt(p);
    }
    // If the VDBE has been run even partially, then transfer the error code
    // and error message from the VDBE into the main database structure.  But
    // if the VDBE has just been set to run but has not actually executed any
    // instructions yet, leave the main database error information unchanged.
    if (unsafe { (*p).pc }) >= (0 as i32) {
        {}
        if (unsafe { (*db).pErr }) != std::ptr::null_mut::<sqlite3_value>()
            || (unsafe { (*p).zErrMsg }) != std::ptr::null_mut::<i8>()
        {
            sqlite3VdbeTransferError(p);
        } else {
            unsafe {
                (*db).errCode = unsafe { (*p).rc };
            }
        }
    }
    // Reset register contents and reclaim error message memory.
    if (unsafe { (*p).zErrMsg }) != std::ptr::null_mut::<i8>() {
        unsafe { sqlite3DbFree(db, (unsafe { (*p).zErrMsg }) as *mut ()) };
        unsafe {
            (*p).zErrMsg = std::ptr::null_mut::<i8>();
        }
    }
    unsafe {
        (*p).pResultRow = std::ptr::null_mut::<sqlite3_value>();
    }
    // Save profiling information from this VDBE run.
    return (unsafe { (*p).rc }) & unsafe { (*db).errMask };
}

/// Clean up and delete a VDBE after execution.  Return an integer which is
/// the result code.  Write any error message text into *pzErrMsg.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3VdbeFinalize(mut p: *mut Vdbe) -> i32 {
    let mut rc: i32 = 0 as i32;
    0 as i32;
    0 as i32;
    0 as i32;
    if (((unsafe { (*p).eVdbeState }) as u32) as i32) >= (1 as i32) {
        rc = sqlite3VdbeReset(p);
        0 as i32;
    }
    sqlite3VdbeDelete(p);
    return rc;
}

/// If parameter iOp is less than zero, then invoke the destructor for
/// all auxiliary data pointers currently cached by the VM passed as
/// the first argument.
///
/// Or, if iOp is greater than or equal to zero, then the destructor is
/// only invoked for those auxiliary data pointers created by the user
/// function invoked by the OP_Function opcode at instruction iOp of
/// VM pVdbe, and only then if:
///
///    * the associated function parameter is the 32nd or later (counting
///      from left to right), or
///
///    * the corresponding bit in argument mask is clear (where the first
///      function parameter corresponds to bit 0 etc.).
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3VdbeDeleteAuxData(
    mut db: *mut sqlite3,
    mut pp: *mut *mut AuxData,
    mut iOp: i32,
    mut mask: i32,
) {
    '__slate_break_1734: while (unsafe { *pp }) != std::ptr::null_mut::<AuxData>() {
        let mut pAux: *mut AuxData = unsafe { *pp };
        if iOp < (0 as i32)
            || (unsafe { (*pAux).iAuxOp }) == iOp
                && (unsafe { (*pAux).iAuxArg }) >= (0 as i32)
                && ((unsafe { (*pAux).iAuxArg }) > (31 as i32)
                    || !((mask as u32) & ((1 as i32) as u32) << unsafe { (*pAux).iAuxArg }
                        != (0 as u32)))
        {
            {}
            if (unsafe { (*pAux).xDeleteAux }) != None {
                unsafe { unsafe { (*pAux).xDeleteAux }.unwrap()(unsafe { (*pAux).pAux }) };
            }
            unsafe {
                *pp = unsafe { (*pAux).pNextAux };
            }
            unsafe { sqlite3DbFree(db, pAux as *mut ()) };
        } else {
            pp = unsafe { std::ptr::addr_of_mut!((*pAux).pNextAux) };
        }
    }
}

/// Free all memory associated with the Vdbe passed as the second argument,
/// except for object itself, which is preserved.
///
/// The difference between this function and sqlite3VdbeDelete() is that
/// VdbeDelete() also unlinks the Vdbe from the list of VMs associated with
/// the database connection and frees the object itself.
fn sqlite3VdbeClearObject(mut db: *mut sqlite3, mut p: *mut Vdbe) {
    let mut pSub: *mut SubProgram = unsafe { std::mem::zeroed() };
    let mut pNext: *mut SubProgram = unsafe { std::mem::zeroed() };
    0 as i32;
    0 as i32;
    if (unsafe { (*p).aColName }) != std::ptr::null_mut::<sqlite3_value>() {
        releaseMemArray(
            unsafe { (*p).aColName },
            (((unsafe { (*p).nResAlloc }) as u32) as i32) * (2 as i32),
        );
        unsafe { sqlite3DbNNFreeNN(db, (unsafe { (*p).aColName }) as *mut ()) };
    }
    pSub = unsafe { (*p).pProgram };
    '__slate_break_1735: while pSub != std::ptr::null_mut::<SubProgram>() {
        pNext = unsafe { (*pSub).pNext };
        vdbeFreeOpArray(db, unsafe { (*pSub).aOp }, unsafe { (*pSub).nOp });
        unsafe { sqlite3DbFree(db, pSub as *mut ()) };
        pSub = pNext;
    }
    if (((unsafe { (*p).eVdbeState }) as u32) as i32) != (0 as i32) {
        releaseMemArray(unsafe { (*p).aVar }, (unsafe { (*p).nVar }) as i32);
        if (unsafe { (*p).pVList }) != std::ptr::null_mut::<i32>() {
            unsafe { sqlite3DbNNFreeNN(db, (unsafe { (*p).pVList }) as *mut ()) };
        }
        if (unsafe { (*p).pFree }) != std::ptr::null_mut::<()>() {
            unsafe { sqlite3DbNNFreeNN(db, unsafe { (*p).pFree }) };
        }
    }
    vdbeFreeOpArray(db, unsafe { (*p).aOp }, unsafe { (*p).nOp });
    if (unsafe { (*p).zSql }) != std::ptr::null_mut::<i8>() {
        unsafe { sqlite3DbNNFreeNN(db, (unsafe { (*p).zSql }) as *mut ()) };
    }
}

/// Delete an entire VDBE.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3VdbeDelete(mut p: *mut Vdbe) {
    let mut db: *mut sqlite3 = unsafe { std::mem::zeroed() };
    0 as i32;
    db = unsafe { (*p).db };
    0 as i32;
    0 as i32;
    sqlite3VdbeClearObject(db, p);
    if (unsafe { (*db).pnBytesFreed }) == std::ptr::null_mut::<i32>() {
        0 as i32;
        unsafe {
            *unsafe { (*p).ppVPrev } = unsafe { (*p).pVNext };
        }
        if (unsafe { (*p).pVNext }) != std::ptr::null_mut::<Vdbe>() {
            unsafe {
                (*unsafe { (*p).pVNext }).ppVPrev = unsafe { (*p).ppVPrev };
            }
        }
    }
    unsafe { sqlite3DbNNFreeNN(db, p as *mut ()) };
}

/// The cursor "p" has a pending seek operation that has not yet been
/// carried out.  Seek the cursor now.  If an error occurs, return
/// the appropriate error code.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3VdbeFinishMoveto(mut p: *mut VdbeCursor) -> i32 {
    let mut res: i32 = 0 as i32;
    let mut rc: i32 = 0 as i32;
    0 as i32;
    0 as i32;
    0 as i32;
    rc = unsafe {
        sqlite3BtreeTableMoveto(
            unsafe { (*p).uc.pCursor },
            unsafe { (*p).movetoTarget },
            0 as i32,
            std::ptr::addr_of_mut!(res),
        )
    };
    if rc != (0 as i32) {
        return rc;
    }
    if res != (0 as i32) {
        return unsafe { sqlite3CorruptError(3822 as i32) };
    }
    unsafe {
        (*p).deferredMoveto = ((0 as i32) as i8) as u8;
    }
    unsafe {
        (*p).cacheStatus = (0 as i32) as u32;
    }
    return 0 as i32;
}

/// Something has moved cursor "p" out of place.  Maybe the row it was
/// pointed to was deleted out from under it.  Or maybe the btree was
/// rebalanced.  Whatever the cause, try to restore "p" to the place it
/// is supposed to be pointing.  If the row was deleted out from under the
/// cursor, set the cursor to point to a NULL row.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3VdbeHandleMovedCursor(mut p: *mut VdbeCursor) -> i32 {
    let mut isDifferentRow: i32 = 0 as i32;
    let mut rc: i32 = 0 as i32;
    0 as i32;
    0 as i32;
    0 as i32;
    rc = unsafe {
        sqlite3BtreeCursorRestore(
            unsafe { (*p).uc.pCursor },
            std::ptr::addr_of_mut!(isDifferentRow),
        )
    };
    unsafe {
        (*p).cacheStatus = (0 as i32) as u32;
    }
    if isDifferentRow != (0 as i32) {
        unsafe {
            (*p).nullRow = ((1 as i32) as i8) as u8;
        }
    }
    return rc;
}

/// Check to ensure that the cursor is valid.  Restore the cursor
/// if need be.  Return any I/O error from the restore operation.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3VdbeCursorRestore(mut p: *mut VdbeCursor) -> i32 {
    0 as i32;
    if (unsafe { sqlite3BtreeCursorHasMoved(unsafe { (*p).uc.pCursor }) }) != (0 as i32) {
        return sqlite3VdbeHandleMovedCursor(p);
    }
    return 0 as i32;
}

// The following functions:
//
// sqlite3VdbeSerialType()
// sqlite3VdbeSerialTypeLen()
// sqlite3VdbeSerialLen()
// sqlite3VdbeSerialPut()  <--- in-lined into OP_MakeRecord as of 2022-04-02
// sqlite3VdbeSerialGet()
//
// encapsulate the code that serializes values for storage in SQLite
// data and index records. Each serialized value consists of a
// 'serial-type' and a blob of data. The serial type is an 8-byte unsigned
// integer, stored as a varint.
//
// In an SQLite index record, the serial type is stored directly before
// the blob of data that it corresponds to. In a table record, all serial
// types are stored at the start of the record, and the blobs of data at
// the end. Hence these functions allow the caller to handle the
// serial-type and data blob separately.
//
// The following table describes the various storage classes for data:
//
//   serial type        bytes of data      type
//   --------------     ---------------    ---------------
//      0                     0            NULL
//      1                     1            signed integer
//      2                     2            signed integer
//      3                     3            signed integer
//      4                     4            signed integer
//      5                     6            signed integer
//      6                     8            signed integer
//      7                     8            IEEE float
//      8                     0            Integer constant 0
//      9                     0            Integer constant 1
//     10,11                               reserved for expansion
//    N>=12 and even       (N-12)/2        BLOB
//    N>=13 and odd        (N-13)/2        text
//
// The 8 and 9 types were added in 3.3.0, file format 4.  Prior versions
// of SQLite will not understand those serial types.
/// The sizes for serial types less than 128
///  0   1   2   3   4   5   6   7   8   9
///
///   0
///  10
///  20
///  30
///  40
///  50
///  60
///  70
///  80
///  90
/// 100
/// 110
/// 120
#[unsafe(no_mangle)]
static mut sqlite3SmallTypeSizes: __SlateAlign16<[u8; 128]> = __SlateAlign16([
    ((0 as i32) as i8) as u8,
    ((1 as i32) as i8) as u8,
    ((2 as i32) as i8) as u8,
    ((3 as i32) as i8) as u8,
    ((4 as i32) as i8) as u8,
    ((6 as i32) as i8) as u8,
    ((8 as i32) as i8) as u8,
    ((8 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
    ((1 as i32) as i8) as u8,
    ((1 as i32) as i8) as u8,
    ((2 as i32) as i8) as u8,
    ((2 as i32) as i8) as u8,
    ((3 as i32) as i8) as u8,
    ((3 as i32) as i8) as u8,
    ((4 as i32) as i8) as u8,
    ((4 as i32) as i8) as u8,
    ((5 as i32) as i8) as u8,
    ((5 as i32) as i8) as u8,
    ((6 as i32) as i8) as u8,
    ((6 as i32) as i8) as u8,
    ((7 as i32) as i8) as u8,
    ((7 as i32) as i8) as u8,
    ((8 as i32) as i8) as u8,
    ((8 as i32) as i8) as u8,
    ((9 as i32) as i8) as u8,
    ((9 as i32) as i8) as u8,
    ((10 as i32) as i8) as u8,
    ((10 as i32) as i8) as u8,
    ((11 as i32) as i8) as u8,
    ((11 as i32) as i8) as u8,
    ((12 as i32) as i8) as u8,
    ((12 as i32) as i8) as u8,
    ((13 as i32) as i8) as u8,
    ((13 as i32) as i8) as u8,
    ((14 as i32) as i8) as u8,
    ((14 as i32) as i8) as u8,
    ((15 as i32) as i8) as u8,
    ((15 as i32) as i8) as u8,
    ((16 as i32) as i8) as u8,
    ((16 as i32) as i8) as u8,
    ((17 as i32) as i8) as u8,
    ((17 as i32) as i8) as u8,
    ((18 as i32) as i8) as u8,
    ((18 as i32) as i8) as u8,
    ((19 as i32) as i8) as u8,
    ((19 as i32) as i8) as u8,
    ((20 as i32) as i8) as u8,
    ((20 as i32) as i8) as u8,
    ((21 as i32) as i8) as u8,
    ((21 as i32) as i8) as u8,
    ((22 as i32) as i8) as u8,
    ((22 as i32) as i8) as u8,
    ((23 as i32) as i8) as u8,
    ((23 as i32) as i8) as u8,
    ((24 as i32) as i8) as u8,
    ((24 as i32) as i8) as u8,
    ((25 as i32) as i8) as u8,
    ((25 as i32) as i8) as u8,
    ((26 as i32) as i8) as u8,
    ((26 as i32) as i8) as u8,
    ((27 as i32) as i8) as u8,
    ((27 as i32) as i8) as u8,
    ((28 as i32) as i8) as u8,
    ((28 as i32) as i8) as u8,
    ((29 as i32) as i8) as u8,
    ((29 as i32) as i8) as u8,
    ((30 as i32) as i8) as u8,
    ((30 as i32) as i8) as u8,
    ((31 as i32) as i8) as u8,
    ((31 as i32) as i8) as u8,
    ((32 as i32) as i8) as u8,
    ((32 as i32) as i8) as u8,
    ((33 as i32) as i8) as u8,
    ((33 as i32) as i8) as u8,
    ((34 as i32) as i8) as u8,
    ((34 as i32) as i8) as u8,
    ((35 as i32) as i8) as u8,
    ((35 as i32) as i8) as u8,
    ((36 as i32) as i8) as u8,
    ((36 as i32) as i8) as u8,
    ((37 as i32) as i8) as u8,
    ((37 as i32) as i8) as u8,
    ((38 as i32) as i8) as u8,
    ((38 as i32) as i8) as u8,
    ((39 as i32) as i8) as u8,
    ((39 as i32) as i8) as u8,
    ((40 as i32) as i8) as u8,
    ((40 as i32) as i8) as u8,
    ((41 as i32) as i8) as u8,
    ((41 as i32) as i8) as u8,
    ((42 as i32) as i8) as u8,
    ((42 as i32) as i8) as u8,
    ((43 as i32) as i8) as u8,
    ((43 as i32) as i8) as u8,
    ((44 as i32) as i8) as u8,
    ((44 as i32) as i8) as u8,
    ((45 as i32) as i8) as u8,
    ((45 as i32) as i8) as u8,
    ((46 as i32) as i8) as u8,
    ((46 as i32) as i8) as u8,
    ((47 as i32) as i8) as u8,
    ((47 as i32) as i8) as u8,
    ((48 as i32) as i8) as u8,
    ((48 as i32) as i8) as u8,
    ((49 as i32) as i8) as u8,
    ((49 as i32) as i8) as u8,
    ((50 as i32) as i8) as u8,
    ((50 as i32) as i8) as u8,
    ((51 as i32) as i8) as u8,
    ((51 as i32) as i8) as u8,
    ((52 as i32) as i8) as u8,
    ((52 as i32) as i8) as u8,
    ((53 as i32) as i8) as u8,
    ((53 as i32) as i8) as u8,
    ((54 as i32) as i8) as u8,
    ((54 as i32) as i8) as u8,
    ((55 as i32) as i8) as u8,
    ((55 as i32) as i8) as u8,
    ((56 as i32) as i8) as u8,
    ((56 as i32) as i8) as u8,
    ((57 as i32) as i8) as u8,
    ((57 as i32) as i8) as u8,
]);

/// Return the length of the data corresponding to the supplied serial-type.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3VdbeSerialTypeLen(mut serial_type: u32) -> u32 {
    if serial_type >= ((128 as i32) as u32) {
        return serial_type.wrapping_sub((12 as i32) as u32) / ((2 as i32) as u32);
    } else {
        0 as i32;
        return (unsafe {
            *unsafe {
                unsafe { std::ptr::addr_of!(sqlite3SmallTypeSizes.0) as *const u8 }
                    .offset(serial_type as isize)
            }
        }) as u32;
    }
    return unsafe { std::mem::zeroed() };
}

#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3VdbeOneByteSerialTypeLen(mut serial_type: u8) -> u8 {
    0 as i32;
    return unsafe {
        *unsafe {
            unsafe { std::ptr::addr_of!(sqlite3SmallTypeSizes.0) as *const u8 }
                .offset(((serial_type as u32) as i32) as isize)
        }
    };
}

/// If we are on an architecture with mixed-endian floating
/// points (ex: ARM7) then swap the lower 4 bytes with the
/// upper 4 bytes.  Return the result.
///
/// For most architectures, this is a no-op.
///
/// (later):  It is reported to me that the mixed-endian problem
/// on ARM7 is an issue with GCC, not with the ARM7 chip.  It seems
/// that early versions of GCC stored the two words of a 64-bit
/// float in the wrong order.  And that error has been propagated
/// ever since.  The blame is not necessarily with GCC, though.
/// GCC might have just copying the problem from a prior compiler.
/// I am also told that newer versions of GCC that follow a different
/// ABI get the byte order right.
///
/// Developers using SQLite on an ARM7 should compile and run their
/// application using -DSQLITE_DEBUG=1 at least once.  With DEBUG
/// enabled, some asserts below will ensure that the byte order of
/// floating point values is correct.
///
/// (2007-08-30)  Frank van Vugt has studied this problem closely
/// and has send his findings to the SQLite developers.  Frank
/// writes that some Linux kernels offer floating point hardware
/// emulation that uses only 32-bit mantissas instead of a full
/// 48-bits as required by the IEEE standard.  (This is the
/// CONFIG_FPE_FASTFPE option.)  On such systems, floating point
/// byte swapping becomes very complicated.  To avoid problems,
/// the necessary byte swapping is carried out using a 64-bit integer
/// rather than a 64-bit float.  Frank assures us that the code here
/// works for him.  We, the developers, have no way to independently
/// verify this, but Frank seems to know what he is talking about
/// so we trust him.
/// Deserialize the REAL number pointed to by buf and store it in pMem.
///
/// # Arguments
///
/// * `buf` - Buffer to deserialize from
/// * `pMem` - Memory cell to write value into
fn sqlite3VdbeSerialGet7(mut buf: *const u8, mut pMem: *mut sqlite3_value) -> i32 {
    // EVIDENCE-OF: R-57343-49114 Value is a big-endian IEEE 754-2008 64-bit
    // floating point number.
    let mut x: u64 = unsafe { sqlite3Get8byte(buf) };
    0 as i32;
    {}
    unsafe {
        memcpy(
            (unsafe { std::ptr::addr_of_mut!((*pMem).u.r) }) as *mut (),
            std::ptr::addr_of_mut!(x) as *const (),
            8 as u64,
        )
    };
    if x & (((2047 as i32) as i64) as u64) << (52 as i32)
        == (((2047 as i32) as i64) as u64) << (52 as i32)
        && x & ((((1 as i32) as i64) as u64) << (52 as i32))
            .wrapping_sub(((1 as i32) as i64) as u64)
            != (((0 as i32) as i64) as u64)
    {
        unsafe {
            (*pMem).flags = ((1 as i32) as i16) as u16;
        }
        return 1 as i32;
    }
    unsafe {
        (*pMem).flags = ((8 as i32) as i16) as u16;
    }
    return 0 as i32;
}

/// Deserialize the data blob pointed to by buf as serial type serial_type
/// and store the result in pMem.
///
/// Similar code is found in the implementation of the OP_Column opcode.
///
/// # Arguments
///
/// * `buf` - Buffer to deserialize from
/// * `serial_type` - Serial type to deserialize
/// * `pMem` - Memory cell to write value into
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3VdbeSerialGet(
    mut buf: *const u8,
    mut serial_type: u32,
    mut pMem: *mut sqlite3_value,
) {
    match serial_type {
        10 => {
            // Internal use only: NULL with virtual table
            // UPDATE no-change flag set
            unsafe {
                (*pMem).flags = (((1 as i32) | (1024 as i32)) as i16) as u16;
            }
            unsafe {
                (*pMem).n = 0 as i32;
            }
            unsafe {
                (*pMem).u.nZero = 0 as i32;
            }
            return;
        }
        11 | 0 => {
            // Null
            // EVIDENCE-OF: R-24078-09375 Value is a NULL.
            unsafe {
                (*pMem).flags = ((1 as i32) as i16) as u16;
            }
            return;
            // Reserved for future use
        }
        1 => {
            // EVIDENCE-OF: R-44885-25196 Value is an 8-bit twos-complement
            // integer.
            unsafe {
                (*pMem).u.i =
                    ((unsafe { *unsafe { buf.offset((0 as i32) as isize) } }) as i8) as i64;
            }
            unsafe {
                (*pMem).flags = ((4 as i32) as i16) as u16;
            }
            {}
            return;
        }
        2 => {
            // 2-byte signed integer
            // EVIDENCE-OF: R-49794-35026 Value is a big-endian 16-bit
            // twos-complement integer.
            unsafe {
                (*pMem).u.i = ((256 as i32)
                    * (((unsafe { *unsafe { buf.offset((0 as i32) as isize) } }) as i8) as i32)
                    | (((unsafe { *unsafe { buf.offset((1 as i32) as isize) } }) as u32) as i32))
                    as i64;
            }
            unsafe {
                (*pMem).flags = ((4 as i32) as i16) as u16;
            }
            {}
            return;
        }
        3 => {
            // 3-byte signed integer
            // EVIDENCE-OF: R-37839-54301 Value is a big-endian 24-bit
            // twos-complement integer.
            unsafe {
                (*pMem).u.i = ((65536 as i32)
                    * (((unsafe { *unsafe { buf.offset((0 as i32) as isize) } }) as i8) as i32)
                    | (((unsafe { *unsafe { buf.offset((1 as i32) as isize) } }) as u32) as i32)
                        << (8 as i32)
                    | (((unsafe { *unsafe { buf.offset((2 as i32) as isize) } }) as u32) as i32))
                    as i64;
            }
            unsafe {
                (*pMem).flags = ((4 as i32) as i16) as u16;
            }
            {}
            return;
        }
        4 => {
            // 4-byte signed integer
            // EVIDENCE-OF: R-01849-26079 Value is a big-endian 32-bit
            // twos-complement integer.
            unsafe {
                (*pMem).u.i = ((((unsafe { *unsafe { buf.offset((0 as i32) as isize) } }) as u32)
                    << (24 as i32)
                    | (((((unsafe { *unsafe { buf.offset((1 as i32) as isize) } }) as u32) as i32)
                        << (16 as i32)) as u32)
                    | (((((unsafe { *unsafe { buf.offset((2 as i32) as isize) } }) as u32) as i32)
                        << (8 as i32)) as u32)
                    | ((((unsafe { *unsafe { buf.offset((3 as i32) as isize) } }) as u32) as i32)
                        as u32)) as i32) as i64;
            }
            unsafe {
                (*pMem).flags = ((4 as i32) as i16) as u16;
            }
            {}
            return;
        }
        5 => {
            // 6-byte signed integer
            // EVIDENCE-OF: R-50385-09674 Value is a big-endian 48-bit
            // twos-complement integer.
            unsafe {
                (*pMem).u.i = (((((unsafe {
                    *unsafe {
                        unsafe { buf.offset((2 as i32) as isize) }.offset((0 as i32) as isize)
                    }
                }) as u32)
                    << (24 as i32)
                    | (((((unsafe {
                        *unsafe {
                            unsafe { buf.offset((2 as i32) as isize) }.offset((1 as i32) as isize)
                        }
                    }) as u32) as i32)
                        << (16 as i32)) as u32)
                    | (((((unsafe {
                        *unsafe {
                            unsafe { buf.offset((2 as i32) as isize) }.offset((2 as i32) as isize)
                        }
                    }) as u32) as i32)
                        << (8 as i32)) as u32)
                    | ((((unsafe {
                        *unsafe {
                            unsafe { buf.offset((2 as i32) as isize) }.offset((3 as i32) as isize)
                        }
                    }) as u32) as i32) as u32)) as u64) as i64)
                    + (4294967296 as i64)
                        * (((256 as i32)
                            * (((unsafe { *unsafe { buf.offset((0 as i32) as isize) } }) as i8)
                                as i32)
                            | (((unsafe { *unsafe { buf.offset((1 as i32) as isize) } }) as u32)
                                as i32)) as i64);
            }
            unsafe {
                (*pMem).flags = ((4 as i32) as i16) as u16;
            }
            {}
            return;
        }
        6 => {
            // 8-byte signed integer
            // EVIDENCE-OF: R-29851-52272 Value is a big-endian 64-bit
            // twos-complement integer.
            unsafe {
                (*pMem).u.i = (unsafe { sqlite3Get8byte(buf) }) as i64;
            }
            unsafe {
                (*pMem).flags = ((4 as i32) as i16) as u16;
            }
            {}
            return;
        }
        7 => {
            // IEEE floating point
            sqlite3VdbeSerialGet7(buf, pMem);
            return;
        }
        8 | 9 => {
            // Integer 1
            // EVIDENCE-OF: R-12976-22893 Value is the integer 0.
            //
            // EVIDENCE-OF: R-18143-12121 Value is the integer 1.
            unsafe {
                (*pMem).u.i = (serial_type.wrapping_sub((8 as i32) as u32) as u64) as i64;
            }
            unsafe {
                (*pMem).flags = ((4 as i32) as i16) as u16;
            }
            return;
            // Integer 0
        }
        _ => {
            // EVIDENCE-OF: R-14606-31564 Value is a BLOB that is (N-12)/2 bytes in
            // length.
            // EVIDENCE-OF: R-28401-00140 Value is a string in the text encoding and
            // (N-13)/2 bytes in length.
            unsafe {
                (*pMem).z = buf as *mut i8;
            }
            unsafe {
                (*pMem).n =
                    (serial_type.wrapping_sub((12 as i32) as u32) / ((2 as i32) as u32)) as i32;
            }
            unsafe {
                (*pMem).flags = unsafe {
                    *unsafe {
                        unsafe { std::ptr::addr_of!(aFlag) as *const u16 }
                            .offset((serial_type & ((1 as i32) as u32)) as isize)
                    }
                };
            }
            return;
        }
    }
    return;
}

static mut aFlag: [u16; 2] = [
    (((16 as i32) | (16384 as i32)) as i16) as u16,
    (((2 as i32) | (16384 as i32)) as i16) as u16,
];

/// Allocate sufficient space for an UnpackedRecord structure large enough
/// to hold a decoded index record for pKeyInfo.
///
/// The space is allocated using sqlite3DbMallocRaw().  If an OOM error
/// occurs, NULL is returned.
///
/// # Arguments
///
/// * `pKeyInfo` - Description of the record
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3VdbeAllocUnpackedRecord(
    mut pKeyInfo: *mut KeyInfo,
) -> *mut UnpackedRecord {
    let mut p: *mut UnpackedRecord = unsafe { std::mem::zeroed() }; // Unpacked record to return
    let mut nByte: u64 = 0 as u64; // Number of bytes required for *p
    0 as i32;
    nByte = (40 as u64).wrapping_add((56 as u64).wrapping_mul(
        (((((unsafe { (*pKeyInfo).nKeyField }) as u32) as i32) + (1 as i32)) as i64) as u64,
    ));
    p = (unsafe { sqlite3DbMallocRaw(unsafe { (*pKeyInfo).db }, nByte) }) as *mut UnpackedRecord;
    if !(p != std::ptr::null_mut::<UnpackedRecord>()) {
        return std::ptr::null_mut::<UnpackedRecord>();
    }
    unsafe {
        (*p).aMem = (unsafe { (p as *mut i8).offset((40 as u64) as isize) }) as *mut sqlite3_value;
    }
    unsafe {
        (*p).pKeyInfo = pKeyInfo;
    }
    unsafe {
        (*p).nField =
            (((((unsafe { (*pKeyInfo).nKeyField }) as u32) as i32) + (1 as i32)) as i16) as u16;
    }
    return p;
}

/// Given the nKey-byte encoding of a record in pKey[], populate the
/// UnpackedRecord structure indicated by the fourth argument with the
/// contents of the decoded record.
///
/// # Arguments
///
/// * `nKey` - Size of the binary record
/// * `pKey` - The binary record
/// * `p` - Populate this structure before returning.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3VdbeRecordUnpack(
    mut nKey: i32,
    mut pKey: *const (),
    mut p: *mut UnpackedRecord,
) {
    let mut aKey: *const u8 = pKey as *const u8;
    let mut d: u32 = 0 as u32;
    let mut idx: u32 = 0 as u32; // Offset in aKey[] to read from
    let mut u: u16 = 0 as u16; // Unsigned loop counter
    let mut szHdr: u32 = 0 as u32;
    let mut pMem: *mut sqlite3_value = unsafe { (*p).aMem };
    let mut pKeyInfo: *mut KeyInfo = unsafe { (*p).pKeyInfo };
    unsafe {
        (*p).default_rc = (0 as i32) as i8;
    }
    0 as i32;
    let __v1815: i32;
    if (((unsafe { *aKey }) as u32) as i32) < (((((128 as i32) as i8) as u8) as u32) as i32) {
        szHdr = (unsafe { *aKey }) as u32;
        __v1815 = 1 as i32;
    } else {
        __v1815 =
            ((unsafe { sqlite3GetVarint32(aKey, std::ptr::addr_of_mut!(szHdr)) }) as u32) as i32;
    }
    idx = ((__v1815 as i8) as u8) as u32;
    d = szHdr;
    u = ((0 as i32) as i16) as u16;
    '__slate_break_1737: while idx < szHdr && d <= (nKey as u32) {
        let mut serial_type: u32 = 0 as u32;
        let __v1816: u32 = idx;
        let __v1817: i32;
        if (((unsafe { *unsafe { aKey.offset(idx as isize) } }) as u32) as i32)
            < (((((128 as i32) as i8) as u8) as u32) as i32)
        {
            serial_type = (unsafe { *unsafe { aKey.offset(idx as isize) } }) as u32;
            __v1817 = 1 as i32;
        } else {
            __v1817 = ((unsafe {
                sqlite3GetVarint32(
                    unsafe { aKey.offset(idx as isize) },
                    std::ptr::addr_of_mut!(serial_type),
                )
            }) as u32) as i32;
        }
        let __v1818: u32 = __v1816.wrapping_add(((((__v1817 as i8) as u8) as u32) as i32) as u32);
        idx = __v1818;
        unsafe {
            (*pMem).enc = unsafe { (*pKeyInfo).enc };
        }
        unsafe {
            (*pMem).db = unsafe { (*pKeyInfo).db };
        }
        // pMem->flags = 0; // sqlite3VdbeSerialGet() will set this for us
        unsafe {
            (*pMem).szMalloc = 0 as i32;
        }
        unsafe {
            (*pMem).z = std::ptr::null_mut::<i8>();
        }
        sqlite3VdbeSerialGet(unsafe { aKey.offset(d as isize) }, serial_type, pMem);
        let __v1819: u32 = d;
        let __v1820: u32 = __v1819.wrapping_add(sqlite3VdbeSerialTypeLen(serial_type));
        d = __v1820;
        let __v1821: u16 = u;
        let __v1822: u16 = ((((__v1821 as u32) as i32) + (1 as i32)) as i16) as u16;
        u = __v1822;
        if ((__v1822 as u32) as i32) >= (((unsafe { (*p).nField }) as u32) as i32) {
            break '__slate_break_1737;
        }
        let __v1823: *mut sqlite3_value = pMem;
        let __v1824: *mut sqlite3_value = unsafe { __v1823.offset((1 as i32) as isize) };
        pMem = __v1824;
    }
    if d > (nKey as u32) && u != (0 as u16) {
        0 as i32;
        // In a corrupt record entry, the last pMem might have been set up using
        // uninitialized memory. Overwrite its value with NULL, to prevent
        // warnings from MSAN.
        unsafe {
            sqlite3VdbeMemSetNull(unsafe {
                pMem.offset(
                    -(((((u as u32) as i32) < (((unsafe { (*p).nField }) as u32) as i32)) as i32)
                        as isize),
                )
            })
        };
    }
    {}
    {}
    0 as i32;
    unsafe {
        (*p).nField = u;
    }
}

/// Both *pMem1 and *pMem2 contain string values. Compare the two values
/// using the collation sequence pColl. As usual, return a negative , zero
/// or positive value if *pMem1 is less than, equal to or greater than
/// *pMem2, respectively. Similar in spirit to "rc = (*pMem1) - (*pMem2);".
///
/// # Arguments
///
/// * `prcErr` - If an OOM occurs, set to SQLITE_NOMEM
fn vdbeCompareMemStringWithEncodingChange(
    mut pMem1: *const sqlite3_value,
    mut pMem2: *const sqlite3_value,
    mut pColl: *const CollSeq,
    mut prcErr: *mut u8,
) -> i32 {
    let mut rc: i32 = 0 as i32;
    let mut v1: *const () = unsafe { std::mem::zeroed() };
    let mut v2: *const () = unsafe { std::mem::zeroed() };
    let mut c1: sqlite3_value = unsafe { std::mem::zeroed() };
    let mut c2: sqlite3_value = unsafe { std::mem::zeroed() };
    unsafe {
        sqlite3VdbeMemInit(
            std::ptr::addr_of_mut!(c1),
            unsafe { (*pMem1).db },
            ((1 as i32) as i16) as u16,
        )
    };
    unsafe {
        sqlite3VdbeMemInit(
            std::ptr::addr_of_mut!(c2),
            unsafe { (*pMem1).db },
            ((1 as i32) as i16) as u16,
        )
    };
    unsafe { sqlite3VdbeMemShallowCopy(std::ptr::addr_of_mut!(c1), pMem1, 16384 as i32) };
    unsafe { sqlite3VdbeMemShallowCopy(std::ptr::addr_of_mut!(c2), pMem2, 16384 as i32) };
    v1 = unsafe { sqlite3ValueText(std::ptr::addr_of_mut!(c1), unsafe { (*pColl).enc }) };
    v2 = unsafe { sqlite3ValueText(std::ptr::addr_of_mut!(c2), unsafe { (*pColl).enc }) };
    if v1 == std::ptr::null::<()>() || v2 == std::ptr::null::<()>() {
        if prcErr != std::ptr::null_mut::<u8>() {
            unsafe {
                *prcErr = ((7 as i32) as i8) as u8;
            }
        }
        rc = 0 as i32;
    } else {
        rc = unsafe {
            unsafe { (*pColl).xCmp }.unwrap()(unsafe { (*pColl).pUser }, c1.n, v1, c2.n, v2)
        };
    }
    unsafe { sqlite3VdbeMemReleaseMalloc(std::ptr::addr_of_mut!(c1)) };
    unsafe { sqlite3VdbeMemReleaseMalloc(std::ptr::addr_of_mut!(c2)) };
    return rc;
}

/// # Arguments
///
/// * `prcErr` - If an OOM occurs, set to SQLITE_NOMEM
fn vdbeCompareMemString(
    mut pMem1: *const sqlite3_value,
    mut pMem2: *const sqlite3_value,
    mut pColl: *const CollSeq,
    mut prcErr: *mut u8,
) -> i32 {
    if (((unsafe { (*pMem1).enc }) as u32) as i32) == (((unsafe { (*pColl).enc }) as u32) as i32) {
        // The strings are already in the correct encoding.  Call the
        // comparison function directly
        return unsafe {
            unsafe { (*pColl).xCmp }.unwrap()(
                unsafe { (*pColl).pUser },
                unsafe { (*pMem1).n },
                (unsafe { (*pMem1).z }) as *const (),
                unsafe { (*pMem2).n },
                (unsafe { (*pMem2).z }) as *const (),
            )
        };
    } else {
        return vdbeCompareMemStringWithEncodingChange(pMem1, pMem2, pColl, prcErr);
    }
    return unsafe { std::mem::zeroed() };
}

/// The input pBlob is guaranteed to be a Blob that is not marked
/// with MEM_Zero.  Return true if it could be a zero-blob.
fn isAllZero(mut z: *const i8, mut n: i32) -> i32 {
    let mut i: i32 = 0 as i32;
    i = 0 as i32;
    '__slate_break_1738: loop {
        if !(i < n) {
            break;
        }
        if (unsafe { *unsafe { z.offset(i as isize) } }) != (0 as i8) {
            return 0 as i32;
        }
        let __v1975: i32 = i;
        let __v1976: i32 = __v1975 + (1 as i32);
        i = __v1976;
    }
    return 1 as i32;
}

/// Compare two blobs.  Return negative, zero, or positive if the first
/// is less than, equal to, or greater than the second, respectively.
/// If one blob is a prefix of the other, then the shorter is the lessor.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3BlobCompare(
    mut pB1: *const sqlite3_value,
    mut pB2: *const sqlite3_value,
) -> i32 {
    let mut c: i32 = 0 as i32;
    let mut n1: i32 = unsafe { (*pB1).n };
    let mut n2: i32 = unsafe { (*pB2).n };
    // It is possible to have a Blob value that has some non-zero content
    // followed by zero content.  But that only comes up for Blobs formed
    // by the OP_MakeRecord opcode, and such Blobs never get passed into
    // sqlite3MemCompare().
    0 as i32;
    0 as i32;
    if ((((unsafe { (*pB1).flags }) as u32) as i32) | (((unsafe { (*pB2).flags }) as u32) as i32))
        & (1024 as i32)
        != (0 as i32)
    {
        if (((unsafe { (*pB1).flags }) as u32) as i32)
            & (((unsafe { (*pB2).flags }) as u32) as i32)
            & (1024 as i32)
            != (0 as i32)
        {
            return (unsafe { (*pB1).u.nZero }) - unsafe { (*pB2).u.nZero };
        } else {
            if (((unsafe { (*pB1).flags }) as u32) as i32) & (1024 as i32) != (0 as i32) {
                if !(isAllZero((unsafe { (*pB2).z }) as *const i8, unsafe { (*pB2).n })
                    != (0 as i32))
                {
                    return -(1 as i32);
                }
                return (unsafe { (*pB1).u.nZero }) - n2;
            } else {
                if !(isAllZero((unsafe { (*pB1).z }) as *const i8, unsafe { (*pB1).n })
                    != (0 as i32))
                {
                    return 1 as i32;
                }
                return n1 - unsafe { (*pB2).u.nZero };
            }
        }
    }
    c = unsafe {
        memcmp(
            (unsafe { (*pB1).z }) as *const (),
            (unsafe { (*pB2).z }) as *const (),
            ((if n1 > n2 { n2 } else { n1 }) as i64) as u64,
        )
    };
    if c != (0 as i32) {
        return c;
    }
    return n1 - n2;
}

/// The following two functions are used only within testcase() to prove
/// test coverage.  These functions do no exist for production builds.
/// We must use separate SQLITE_NOINLINE functions here, since otherwise
/// optimizer code movement causes gcov to become very confused.
/// Do a comparison between a 64-bit signed integer and a 64-bit floating-point
/// number.  Return negative, zero, or positive if the first (i64) is less than,
/// equal to, or greater than the second (double).
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3IntFloatCompare(mut i: i64, mut r: f64) -> i32 {
    if (unsafe { sqlite3IsNaN(r) }) != (0 as i32) {
        // SQLite considers NaN to be a NULL. And all integer values are greater
        // than NULL
        return 1 as i32;
    } else {
        let mut y: i64 = 0 as i64;
        if r < -9.223372036854776e18f64 {
            return 1 as i32;
        }
        if r >= 9.223372036854776e18f64 {
            return -(1 as i32);
        }
        y = r as i64;
        if i < y {
            return -(1 as i32);
        }
        if i > y {
            return 1 as i32;
        }
        {}
        {}
        {}
        return if (i as f64) < r {
            -(1 as i32)
        } else {
            ((i as f64) > r) as i32
        };
    }
    return unsafe { std::mem::zeroed() };
}

/// Compare the values contained by the two memory cells, returning
/// negative, zero or positive if pMem1 is less than, equal to, or greater
/// than pMem2. Sorting order is NULL's first, followed by numbers (integers
/// and reals) sorted numerically, followed by text ordered by the collating
/// sequence pColl and finally blob's ordered by memcmp().
///
/// Two NULL values are considered equal by this function.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3MemCompare(
    mut pMem1: *const sqlite3_value,
    mut pMem2: *const sqlite3_value,
    mut pColl: *const CollSeq,
) -> i32 {
    let mut f1: i32 = 0 as i32;
    let mut f2: i32 = 0 as i32;
    let mut combined_flags: i32 = 0 as i32;
    f1 = ((unsafe { (*pMem1).flags }) as u32) as i32;
    f2 = ((unsafe { (*pMem2).flags }) as u32) as i32;
    combined_flags = f1 | f2;
    0 as i32;
    // If one value is NULL, it is less than the other. If both values
    // are NULL, return 0.
    if combined_flags & (1 as i32) != (0 as i32) {
        return (f2 & (1 as i32)) - (f1 & (1 as i32));
    }
    // At least one of the two values is a number
    if combined_flags & ((4 as i32) | (8 as i32) | (32 as i32)) != (0 as i32) {
        {}
        {}
        {}
        if f1 & f2 & ((4 as i32) | (32 as i32)) != (0 as i32) {
            {}
            {}
            if (unsafe { (*pMem1).u.i }) < unsafe { (*pMem2).u.i } {
                return -(1 as i32);
            }
            if (unsafe { (*pMem1).u.i }) > unsafe { (*pMem2).u.i } {
                return 1 as i32;
            }
            return 0 as i32;
        }
        if f1 & f2 & (8 as i32) != (0 as i32) {
            if (unsafe { (*pMem1).u.r }) < unsafe { (*pMem2).u.r } {
                return -(1 as i32);
            }
            if (unsafe { (*pMem1).u.r }) > unsafe { (*pMem2).u.r } {
                return 1 as i32;
            }
            return 0 as i32;
        }
        if f1 & ((4 as i32) | (32 as i32)) != (0 as i32) {
            {}
            {}
            if f2 & (8 as i32) != (0 as i32) {
                return sqlite3IntFloatCompare(unsafe { (*pMem1).u.i }, unsafe { (*pMem2).u.r });
            } else {
                if f2 & ((4 as i32) | (32 as i32)) != (0 as i32) {
                    if (unsafe { (*pMem1).u.i }) < unsafe { (*pMem2).u.i } {
                        return -(1 as i32);
                    }
                    if (unsafe { (*pMem1).u.i }) > unsafe { (*pMem2).u.i } {
                        return 1 as i32;
                    }
                    return 0 as i32;
                } else {
                    return -(1 as i32);
                }
            }
        }
        if f1 & (8 as i32) != (0 as i32) {
            if f2 & ((4 as i32) | (32 as i32)) != (0 as i32) {
                {}
                {}
                return -sqlite3IntFloatCompare(unsafe { (*pMem2).u.i }, unsafe { (*pMem1).u.r });
            } else {
                return -(1 as i32);
            }
        }
        return 1 as i32;
    }
    // If one value is a string and the other is a blob, the string is less.
    // If both are strings, compare using the collating functions.
    if combined_flags & (2 as i32) != (0 as i32) {
        if f1 & (2 as i32) == (0 as i32) {
            return 1 as i32;
        }
        if f2 & (2 as i32) == (0 as i32) {
            return -(1 as i32);
        }
        0 as i32;
        0 as i32;
        // The collation sequence must be defined at this point, even if
        // the user deletes the collation sequence after the vdbe program is
        // compiled (this was not always the case).
        0 as i32;
        if pColl != std::ptr::null::<CollSeq>() {
            return vdbeCompareMemString(pMem1, pMem2, pColl, std::ptr::null_mut::<u8>());
        }
        // If a NULL pointer was passed as the collate function, fall through
        // to the blob case and use memcmp().
    }
    // Both values must be blobs.  Compare using memcmp().
    return sqlite3BlobCompare(pMem1, pMem2);
}

/// The first argument passed to this function is a serial-type that
/// corresponds to an integer - all values between 1 and 9 inclusive
/// except 7. The second points to a buffer containing an integer value
/// serialized according to serial_type. This function deserializes
/// and returns the value.
fn vdbeRecordDecodeInt(mut serial_type: u32, mut aKey: *const u8) -> i64 {
    let mut y: u32 = 0 as u32;
    0 as i32;
    match serial_type {
        0 | 1 => {
            {}
            return ((unsafe { *unsafe { aKey.offset((0 as i32) as isize) } }) as i8) as i64;
        }
        2 => {
            {}
            return ((256 as i32)
                * (((unsafe { *unsafe { aKey.offset((0 as i32) as isize) } }) as i8) as i32)
                | (((unsafe { *unsafe { aKey.offset((1 as i32) as isize) } }) as u32) as i32))
                as i64;
        }
        3 => {
            {}
            return ((65536 as i32)
                * (((unsafe { *unsafe { aKey.offset((0 as i32) as isize) } }) as i8) as i32)
                | (((unsafe { *unsafe { aKey.offset((1 as i32) as isize) } }) as u32) as i32)
                    << (8 as i32)
                | (((unsafe { *unsafe { aKey.offset((2 as i32) as isize) } }) as u32) as i32))
                as i64;
        }
        4 => {
            {}
            y = ((unsafe { *unsafe { aKey.offset((0 as i32) as isize) } }) as u32) << (24 as i32)
                | (((((unsafe { *unsafe { aKey.offset((1 as i32) as isize) } }) as u32) as i32)
                    << (16 as i32)) as u32)
                | (((((unsafe { *unsafe { aKey.offset((2 as i32) as isize) } }) as u32) as i32)
                    << (8 as i32)) as u32)
                | ((((unsafe { *unsafe { aKey.offset((3 as i32) as isize) } }) as u32) as i32)
                    as u32);
            return (unsafe { *(std::ptr::addr_of_mut!(y) as *mut i32) }) as i64;
        }
        5 => {
            {}
            return (((((unsafe {
                *unsafe { unsafe { aKey.offset((2 as i32) as isize) }.offset((0 as i32) as isize) }
            }) as u32)
                << (24 as i32)
                | (((((unsafe {
                    *unsafe {
                        unsafe { aKey.offset((2 as i32) as isize) }.offset((1 as i32) as isize)
                    }
                }) as u32) as i32)
                    << (16 as i32)) as u32)
                | (((((unsafe {
                    *unsafe {
                        unsafe { aKey.offset((2 as i32) as isize) }.offset((2 as i32) as isize)
                    }
                }) as u32) as i32)
                    << (8 as i32)) as u32)
                | ((((unsafe {
                    *unsafe {
                        unsafe { aKey.offset((2 as i32) as isize) }.offset((3 as i32) as isize)
                    }
                }) as u32) as i32) as u32)) as u64) as i64)
                + (((1 as i32) as i64) << (32 as i32))
                    * (((256 as i32)
                        * (((unsafe { *unsafe { aKey.offset((0 as i32) as isize) } }) as i8)
                            as i32)
                        | (((unsafe { *unsafe { aKey.offset((1 as i32) as isize) } }) as u32)
                            as i32)) as i64);
        }
        6 => {
            let mut x: u64 = (((unsafe { *unsafe { aKey.offset((0 as i32) as isize) } }) as u32)
                << (24 as i32)
                | (((((unsafe { *unsafe { aKey.offset((1 as i32) as isize) } }) as u32) as i32)
                    << (16 as i32)) as u32)
                | (((((unsafe { *unsafe { aKey.offset((2 as i32) as isize) } }) as u32) as i32)
                    << (8 as i32)) as u32)
                | ((((unsafe { *unsafe { aKey.offset((3 as i32) as isize) } }) as u32) as i32)
                    as u32)) as u64;
            {}
            x = x << (32 as i32)
                | ((((unsafe {
                    *unsafe {
                        unsafe { aKey.offset((4 as i32) as isize) }.offset((0 as i32) as isize)
                    }
                }) as u32)
                    << (24 as i32)
                    | (((((unsafe {
                        *unsafe {
                            unsafe { aKey.offset((4 as i32) as isize) }.offset((1 as i32) as isize)
                        }
                    }) as u32) as i32)
                        << (16 as i32)) as u32)
                    | (((((unsafe {
                        *unsafe {
                            unsafe { aKey.offset((4 as i32) as isize) }.offset((2 as i32) as isize)
                        }
                    }) as u32) as i32)
                        << (8 as i32)) as u32)
                    | ((((unsafe {
                        *unsafe {
                            unsafe { aKey.offset((4 as i32) as isize) }.offset((3 as i32) as isize)
                        }
                    }) as u32) as i32) as u32)) as u64);
            return unsafe { *(std::ptr::addr_of_mut!(x) as *mut i64) };
        }
        _ => {}
    }
    return (serial_type.wrapping_sub((8 as i32) as u32) as u64) as i64;
}

/// This function compares the two table rows or index records
/// specified by {nKey1, pKey1} and pPKey2.  It returns a negative, zero
/// or positive integer if key1 is less than, equal to or
/// greater than key2.  The {nKey1, pKey1} key must be a blob
/// created by the OP_MakeRecord opcode of the VDBE.  The pPKey2
/// key must be a parsed key such as obtained from
/// sqlite3VdbeParseRecord.
///
/// If argument bSkip is non-zero, it is assumed that the caller has already
/// determined that the first fields of the keys are equal.
///
/// Key1 and Key2 do not have to contain the same number of fields. If all
/// fields that appear in both keys are equal, then pPKey2->default_rc is
/// returned.
///
/// If database corruption is discovered, set pPKey2->errCode to
/// SQLITE_CORRUPT and return 0. If an OOM error is encountered,
/// pPKey2->errCode is set to SQLITE_NOMEM and, if it is not NULL, the
/// malloc-failed flag set on database handle (pPKey2->pKeyInfo->db).
///
/// # Arguments
///
/// * `pKey1` - Left key
/// * `pPKey2` - Right key
/// * `bSkip` - If true, skip the first field
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3VdbeRecordCompareWithSkip(
    mut nKey1: i32,
    mut pKey1: *const (),
    mut pPKey2: *mut UnpackedRecord,
    mut bSkip: i32,
) -> i32 {
    let mut d1: u32 = 0 as u32; // Offset into aKey[] of next data element
    let mut i: i32 = 0 as i32; // Index of next field to compare
    let mut szHdr1: u32 = 0 as u32; // Size of record header in bytes
    let mut idx1: u32 = 0 as u32; // Offset of first type in header
    let mut rc: i32 = 0 as i32; // Return value
    let mut pRhs: *mut sqlite3_value = unsafe { (*pPKey2).aMem }; // Next field of pPKey2 to compare
    let mut pKeyInfo: *mut KeyInfo = unsafe { std::mem::zeroed() };
    let mut aKey1: *const u8 = pKey1 as *const u8;
    let mut mem1: sqlite3_value = unsafe { std::mem::zeroed() };
    // If bSkip is true, then the caller has already determined that the first
    // two elements in the keys are equal. Fix the various stack variables so
    // that this routine begins comparing at the second field.
    if bSkip != (0 as i32) {
        let mut s1: u32 = (unsafe { *unsafe { aKey1.offset((1 as i32) as isize) } }) as u32;
        if s1 < ((128 as i32) as u32) {
            idx1 = (2 as i32) as u32;
        } else {
            idx1 = ((1 as i32)
                + (((unsafe {
                    sqlite3GetVarint32(
                        unsafe { aKey1.offset((1 as i32) as isize) },
                        std::ptr::addr_of_mut!(s1),
                    )
                }) as u32) as i32)) as u32;
        }
        szHdr1 = (unsafe { *unsafe { aKey1.offset((0 as i32) as isize) } }) as u32;
        d1 = szHdr1.wrapping_add(sqlite3VdbeSerialTypeLen(s1));
        i = 1 as i32;
        let __v1825: *mut sqlite3_value = pRhs;
        let __v1826: *mut sqlite3_value = unsafe { __v1825.offset((1 as i32) as isize) };
        pRhs = __v1826;
    } else {
        let __v1827: u32 = (unsafe { *unsafe { aKey1.offset((0 as i32) as isize) } }) as u32;
        szHdr1 = __v1827;
        if __v1827 < ((128 as i32) as u32) {
            idx1 = (1 as i32) as u32;
        } else {
            idx1 = (unsafe { sqlite3GetVarint32(aKey1, std::ptr::addr_of_mut!(szHdr1)) }) as u32;
        }
        d1 = szHdr1;
        i = 0 as i32;
    }
    if d1 > (nKey1 as u32) {
        unsafe {
            (*pPKey2).errCode = ((unsafe { sqlite3CorruptError(4744 as i32) }) as i8) as u8;
        }
        return 0 as i32; // Corruption
    }
    // Only needed by assert() statements
    0 as i32;
    0 as i32;
    0 as i32;
    0 as i32;
    '__slate_break_1740: while (1 as i32) != (0 as i32) {
        // exit-by-break
        let mut serial_type: u32 = 0 as u32;
        // RHS is an integer
        if (((unsafe { (*pRhs).flags }) as u32) as i32) & ((4 as i32) | (32 as i32)) != (0 as i32) {
            {}
            {}
            serial_type = (unsafe { *unsafe { aKey1.offset(idx1 as isize) } }) as u32;
            {}
            if serial_type >= ((10 as i32) as u32) {
                rc = if serial_type == ((10 as i32) as u32) {
                    -(1 as i32)
                } else {
                    1 as i32
                };
            } else {
                if serial_type == ((0 as i32) as u32) {
                    rc = -(1 as i32);
                } else {
                    if serial_type == ((7 as i32) as u32) {
                        sqlite3VdbeSerialGet7(
                            unsafe { aKey1.offset(d1 as isize) },
                            std::ptr::addr_of_mut!(mem1),
                        );
                        rc = -sqlite3IntFloatCompare(unsafe { (*pRhs).u.i }, unsafe { mem1.u.r });
                    } else {
                        let mut lhs: i64 =
                            vdbeRecordDecodeInt(serial_type, unsafe { aKey1.offset(d1 as isize) });
                        let mut rhs: i64 = unsafe { (*pRhs).u.i };
                        if lhs < rhs {
                            rc = -(1 as i32);
                        } else {
                            if lhs > rhs {
                                rc = 1 as i32;
                            }
                        }
                    }
                }
            }
        } else {
            if (((unsafe { (*pRhs).flags }) as u32) as i32) & (8 as i32) != (0 as i32) {
                serial_type = (unsafe { *unsafe { aKey1.offset(idx1 as isize) } }) as u32;
                if serial_type >= ((10 as i32) as u32) {
                    // Serial types 12 or greater are strings and blobs (greater than
                    // numbers). Types 10 and 11 are currently "reserved for future
                    // use", so it doesn't really matter what the results of comparing
                    // them to numeric values are.
                    rc = if serial_type == ((10 as i32) as u32) {
                        -(1 as i32)
                    } else {
                        1 as i32
                    };
                } else {
                    if serial_type == ((0 as i32) as u32) {
                        rc = -(1 as i32);
                    } else {
                        if serial_type == ((7 as i32) as u32) {
                            if sqlite3VdbeSerialGet7(
                                unsafe { aKey1.offset(d1 as isize) },
                                std::ptr::addr_of_mut!(mem1),
                            ) != (0 as i32)
                            {
                                rc = -(1 as i32); // mem1 is a NaN
                            } else {
                                if (unsafe { mem1.u.r }) < unsafe { (*pRhs).u.r } {
                                    rc = -(1 as i32);
                                } else {
                                    if (unsafe { mem1.u.r }) > unsafe { (*pRhs).u.r } {
                                        rc = 1 as i32;
                                    } else {
                                        0 as i32;
                                    }
                                }
                            }
                        } else {
                            sqlite3VdbeSerialGet(
                                unsafe { aKey1.offset(d1 as isize) },
                                serial_type,
                                std::ptr::addr_of_mut!(mem1),
                            );
                            rc =
                                sqlite3IntFloatCompare(unsafe { mem1.u.i }, unsafe { (*pRhs).u.r });
                        }
                    }
                }
            } else {
                if (((unsafe { (*pRhs).flags }) as u32) as i32) & (2 as i32) != (0 as i32) {
                    serial_type = (unsafe { *unsafe { aKey1.offset(idx1 as isize) } }) as u32;
                    if serial_type >= ((128 as i32) as u32) {
                        unsafe {
                            sqlite3GetVarint32(
                                unsafe { aKey1.offset(idx1 as isize) },
                                std::ptr::addr_of_mut!(serial_type),
                            )
                        };
                    }
                    {}
                    if serial_type < ((12 as i32) as u32) {
                        rc = -(1 as i32);
                    } else {
                        if !(serial_type & ((1 as i32) as u32) != (0 as u32)) {
                            rc = 1 as i32;
                        } else {
                            mem1.n = (serial_type.wrapping_sub((12 as i32) as u32)
                                / ((2 as i32) as u32)) as i32;
                            {}
                            {}
                            let __v1828: bool;
                            if d1.wrapping_add(mem1.n as u32) > (nKey1 as u32) {
                                __v1828 = true as bool;
                            } else {
                                let __v1829: *mut KeyInfo = unsafe { (*pPKey2).pKeyInfo };
                                pKeyInfo = __v1829;
                                __v1828 = (((unsafe { (*__v1829).nAllField }) as u32) as i32) <= i;
                            }
                            if __v1828 {
                                unsafe {
                                    (*pPKey2).errCode =
                                        ((unsafe { sqlite3CorruptError(4825 as i32) }) as i8) as u8;
                                }
                                return 0 as i32; // Corruption
                            } else {
                                if (unsafe {
                                    *unsafe {
                                        unsafe {
                                            std::ptr::addr_of_mut!((*pKeyInfo).aColl)
                                                as *mut *mut CollSeq
                                        }
                                        .offset(i as isize)
                                    }
                                }) != std::ptr::null_mut::<CollSeq>()
                                {
                                    mem1.enc = unsafe { (*pKeyInfo).enc };
                                    mem1.db = unsafe { (*pKeyInfo).db };
                                    mem1.flags = ((2 as i32) as i16) as u16;
                                    mem1.z = (unsafe { aKey1.offset(d1 as isize) }) as *mut i8;
                                    rc = vdbeCompareMemString(
                                        std::ptr::addr_of_mut!(mem1) as *const sqlite3_value,
                                        pRhs as *const sqlite3_value,
                                        (unsafe {
                                            *unsafe {
                                                unsafe {
                                                    std::ptr::addr_of_mut!((*pKeyInfo).aColl)
                                                        as *mut *mut CollSeq
                                                }
                                                .offset(i as isize)
                                            }
                                        })
                                            as *const CollSeq,
                                        unsafe { std::ptr::addr_of_mut!((*pPKey2).errCode) },
                                    );
                                } else {
                                    let mut nCmp: i32 = if mem1.n < unsafe { (*pRhs).n } {
                                        mem1.n
                                    } else {
                                        unsafe { (*pRhs).n }
                                    };
                                    rc = unsafe {
                                        memcmp(
                                            (unsafe { aKey1.offset(d1 as isize) }) as *const (),
                                            (unsafe { (*pRhs).z }) as *const (),
                                            (nCmp as i64) as u64,
                                        )
                                    };
                                    if rc == (0 as i32) {
                                        rc = mem1.n - unsafe { (*pRhs).n };
                                    }
                                }
                            }
                        }
                    }
                } else {
                    if (((unsafe { (*pRhs).flags }) as u32) as i32) & (16 as i32) != (0 as i32) {
                        0 as i32;
                        serial_type = (unsafe { *unsafe { aKey1.offset(idx1 as isize) } }) as u32;
                        if serial_type >= ((128 as i32) as u32) {
                            unsafe {
                                sqlite3GetVarint32(
                                    unsafe { aKey1.offset(idx1 as isize) },
                                    std::ptr::addr_of_mut!(serial_type),
                                )
                            };
                        }
                        {}
                        if serial_type < ((12 as i32) as u32)
                            || serial_type & ((1 as i32) as u32) != (0 as u32)
                        {
                            rc = -(1 as i32);
                        } else {
                            let mut nStr: i32 = (serial_type.wrapping_sub((12 as i32) as u32)
                                / ((2 as i32) as u32))
                                as i32;
                            {}
                            {}
                            if d1.wrapping_add(nStr as u32) > (nKey1 as u32) {
                                unsafe {
                                    (*pPKey2).errCode =
                                        ((unsafe { sqlite3CorruptError(4855 as i32) }) as i8) as u8;
                                }
                                return 0 as i32; // Corruption
                            } else {
                                if (((unsafe { (*pRhs).flags }) as u32) as i32) & (1024 as i32)
                                    != (0 as i32)
                                {
                                    if !(isAllZero(
                                        (unsafe { aKey1.offset(d1 as isize) }) as *const i8,
                                        nStr,
                                    ) != (0 as i32))
                                    {
                                        rc = 1 as i32;
                                    } else {
                                        rc = nStr - unsafe { (*pRhs).u.nZero };
                                    }
                                } else {
                                    let mut nCmp: i32 = if nStr < unsafe { (*pRhs).n } {
                                        nStr
                                    } else {
                                        unsafe { (*pRhs).n }
                                    };
                                    rc = unsafe {
                                        memcmp(
                                            (unsafe { aKey1.offset(d1 as isize) }) as *const (),
                                            (unsafe { (*pRhs).z }) as *const (),
                                            (nCmp as i64) as u64,
                                        )
                                    };
                                    if rc == (0 as i32) {
                                        rc = nStr - unsafe { (*pRhs).n };
                                    }
                                }
                            }
                        }
                    } else {
                        serial_type = (unsafe { *unsafe { aKey1.offset(idx1 as isize) } }) as u32;
                        let __v1830: bool;
                        if serial_type == ((0 as i32) as u32) || serial_type == ((10 as i32) as u32)
                        {
                            __v1830 = true as bool;
                        } else {
                            let __v1831: bool;
                            if serial_type == ((7 as i32) as u32) {
                                __v1831 = sqlite3VdbeSerialGet7(
                                    unsafe { aKey1.offset(d1 as isize) },
                                    std::ptr::addr_of_mut!(mem1),
                                ) != (0 as i32);
                            } else {
                                __v1831 = false as bool;
                            }
                            __v1830 = __v1831;
                        }
                        if __v1830 {
                            0 as i32;
                        } else {
                            rc = 1 as i32;
                        }
                    }
                }
            }
        }
        // RHS is real
        // RHS is a string
        // RHS is a blob
        // RHS is null
        if rc != (0 as i32) {
            let mut sortFlags: i32 = ((unsafe {
                *unsafe {
                    unsafe { (*unsafe { (*pPKey2).pKeyInfo }).aSortFlags }.offset(i as isize)
                }
            }) as u32) as i32;
            if sortFlags != (0 as i32) {
                if sortFlags & (2 as i32) == (0 as i32)
                    || sortFlags & (1 as i32)
                        != ((serial_type == ((0 as i32) as u32)
                            || (((unsafe { (*pRhs).flags }) as u32) as i32) & (1 as i32)
                                != (0 as i32)) as i32)
                {
                    rc = -rc;
                }
            }
            0 as i32;
            0 as i32; // See comment below
            return rc;
        }
        let __v1832: i32 = i;
        let __v1833: i32 = __v1832 + (1 as i32);
        i = __v1833;
        if i == (((unsafe { (*pPKey2).nField }) as u32) as i32) {
            break '__slate_break_1740;
        }
        let __v1834: *mut sqlite3_value = pRhs;
        let __v1835: *mut sqlite3_value = unsafe { __v1834.offset((1 as i32) as isize) };
        pRhs = __v1835;
        let __v1836: u32 = d1;
        let __v1837: u32 = __v1836.wrapping_add(sqlite3VdbeSerialTypeLen(serial_type));
        d1 = __v1837;
        if d1 > (nKey1 as u32) {
            break '__slate_break_1740;
        }
        let __v1838: u32 = idx1;
        let __v1839: u32 =
            __v1838.wrapping_add((unsafe { sqlite3VarintLen(serial_type as u64) }) as u32);
        idx1 = __v1839;
        if idx1 >= szHdr1 {
            unsafe {
                (*pPKey2).errCode = ((unsafe { sqlite3CorruptError(4906 as i32) }) as i8) as u8;
            }
            return 0 as i32; // Corrupt index
        }
    }
    // No memory allocation is ever used on mem1.  Prove this using
    // the following assert().  If the assert() fails, it indicates a
    // memory leak and a need to call sqlite3VdbeMemRelease(&mem1).
    0 as i32;
    // rc==0 here means that one or both of the keys ran out of fields and
    // all the fields up to that point were equal. Return the default_rc
    // value.
    0 as i32;
    unsafe {
        (*pPKey2).eqSeen = ((1 as i32) as i8) as u8;
    }
    return (unsafe { (*pPKey2).default_rc }) as i32;
}

/// # Arguments
///
/// * `pKey1` - Left key
/// * `pPKey2` - Right key
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.slate_distinct.vdbeaux.sqlite3VdbeRecordCompare")]
extern "C-unwind" fn sqlite3VdbeRecordCompare(
    mut nKey1: i32,
    mut pKey1: *const (),
    mut pPKey2: *mut UnpackedRecord,
) -> i32 {
    return sqlite3VdbeRecordCompareWithSkip(nKey1, pKey1, pPKey2, 0 as i32);
}

/// This function is an optimized version of sqlite3VdbeRecordCompare()
/// that (a) the first field of pPKey2 is an integer, and (b) the
/// size-of-header varint at the start of (pKey1/nKey1) fits in a single
/// byte (i.e. is less than 128).
///
/// To avoid concerns about buffer overreads, this routine is only used
/// on schemas where the maximum valid header size is 63 bytes or less.
///
/// # Arguments
///
/// * `pKey1` - Left key
/// * `pPKey2` - Right key
#[unsafe(link_section = ".text.slate_distinct.vdbeaux.vdbeRecordCompareInt")]
extern "C-unwind" fn vdbeRecordCompareInt(
    mut nKey1: i32,
    mut pKey1: *const (),
    mut pPKey2: *mut UnpackedRecord,
) -> i32 {
    let mut aKey: *const u8 = unsafe {
        (pKey1 as *const u8)
            .offset(((((unsafe { *(pKey1 as *const u8) }) as u32) as i32) & (63 as i32)) as isize)
    };
    let mut serial_type: i32 =
        ((unsafe { *unsafe { (pKey1 as *const u8).offset((1 as i32) as isize) } }) as u32) as i32;
    let mut res: i32 = 0 as i32;
    let mut v: i64 = 0 as i64;
    let mut lhs: i64 = 0 as i64;
    {}
    0 as i32;
    // Serial types 1 through 6 are big-endian integers of 1, 2, 3, 4,
    // 6, or 8 bytes.  Rather than handle each width in its own switch
    // case, read 8 bytes and use an arithmetic right shift to drop the
    // unwanted low-order bytes and sign-extend the value.  This helps
    // because the switch tends to mispredict when a key column contains
    // integers of varying sizes.  The first entry of aShift[] is a
    // placeholder so that the table can be indexed by serial_type
    // directly.  Reading 8 bytes is always safe, because a buffer passed
    // to this routine has at least 74 bytes of padding after it, as
    // explained in sqlite3VdbeFindCompare() below.
    if ((serial_type - (1 as i32)) as u32) <= ((5 as i32) as u32) {
        lhs = ((unsafe { sqlite3Get8byte(aKey) }) as i64)
            >> (((unsafe {
                *unsafe {
                    unsafe { std::ptr::addr_of!(aShift) as *const u8 }.offset(serial_type as isize)
                }
            }) as u32) as i32);
        //                                 ^^--- This shift operator
        // needs to be an arithmetic right-shift, which means that
        // if the left-hand operand (LHS) is negative, it will be sign-extended
        // so that the final results is also negative.  All modern C
        // compilers work this way as long as the LHS is a signed integer
        // (which is why the unsigned result from sqlite3Get8byte() is cast
        // into i64), but it is not defined by the C standards, or so Claude
        // tells me.  That the correct result is obtained is verified by the
        // following assert() and testcase() macros:
        0 as i32;
        {}
    } else {
        if serial_type == (8 as i32) || serial_type == (9 as i32) {
            lhs = (serial_type - (8 as i32)) as i64;
        } else {
            return sqlite3VdbeRecordCompare(nKey1, pKey1, pPKey2);
        }
    }
    0 as i32;
    v = unsafe { (*pPKey2).u.i };
    if v > lhs {
        res = (unsafe { (*pPKey2).r1 }) as i32;
    } else {
        if v < lhs {
            res = (unsafe { (*pPKey2).r2 }) as i32;
        } else {
            if (((unsafe { (*pPKey2).nField }) as u32) as i32) > (1 as i32) {
                // The first fields of the two keys are equal. Compare the trailing
                // fields.
                res = sqlite3VdbeRecordCompareWithSkip(nKey1, pKey1, pPKey2, 1 as i32);
            } else {
                // The first fields of the two keys are equal and there are no trailing
                // fields. Return pPKey2->default_rc in this case.
                res = (unsafe { (*pPKey2).default_rc }) as i32;
                unsafe {
                    (*pPKey2).eqSeen = ((1 as i32) as i8) as u8;
                }
            }
        }
    }
    0 as i32;
    return res;
}

static mut aShift: [u8; 7] = [
    ((0 as i32) as i8) as u8,
    ((56 as i32) as i8) as u8,
    ((48 as i32) as i8) as u8,
    ((40 as i32) as i8) as u8,
    ((32 as i32) as i8) as u8,
    ((16 as i32) as i8) as u8,
    ((0 as i32) as i8) as u8,
];

/// This function is an optimized version of sqlite3VdbeRecordCompare()
/// that (a) the first field of pPKey2 is a string, that (b) the first field
/// uses the collation sequence BINARY and (c) that the size-of-header varint
/// at the start of (pKey1/nKey1) fits in a single byte.
///
/// # Arguments
///
/// * `pKey1` - Left key
/// * `pPKey2` - Right key
#[unsafe(link_section = ".text.slate_distinct.vdbeaux.vdbeRecordCompareString")]
extern "C-unwind" fn vdbeRecordCompareString(
    mut nKey1: i32,
    mut pKey1: *const (),
    mut pPKey2: *mut UnpackedRecord,
) -> i32 {
    let mut __slate_storage_1133: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1133: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1133) as *mut i32;
    let mut __slate_storage_1132: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1132: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1132) as *mut i32;
    let mut __slate_storage_1131: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1131: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1131) as *mut i32;
    let mut __slate_storage_1130: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1130: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1130) as *mut i32;
    let mut __slate_storage_1129: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1129: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1129) as *mut i32;
    let mut __slate_storage_1128: std::mem::MaybeUninit<*const u8> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1128: *mut *const u8 =
        std::ptr::addr_of_mut!(__slate_storage_1128) as *mut *const u8;
    unsafe {
        std::ptr::write(__slate_slot_1128, pKey1 as *const u8);
        0 as i32;
        0 as i32;
        0 as i32;
        {}
        *__slate_slot_1129 =
            ((unsafe { *unsafe { (*__slate_slot_1128).offset((1 as i32) as isize) } }) as i8)
                as i32;
        '__join_0: {
            '__join_1: {
                '__join_2: {
                    loop {
                        if *__slate_slot_1129 < (12 as i32) {
                            if *__slate_slot_1129 < (0 as i32) {
                                unsafe {
                                    sqlite3GetVarint32(
                                        unsafe { (*__slate_slot_1128).offset((1 as i32) as isize) },
                                        std::ptr::addr_of_mut!(*__slate_slot_1129) as *mut u32,
                                    )
                                };
                                if !(*__slate_slot_1129 >= (12 as i32)) {
                                    break '__join_2;
                                }
                            } else {
                                break '__join_1;
                            }
                        } else {
                            break;
                        }
                    }
                    if !(*__slate_slot_1129 & (1 as i32) != (0 as i32)) {
                        *__slate_slot_1130 = (unsafe { (*pPKey2).r2 }) as i32; // (pKey1/nKey1) is a blob
                        break '__join_0;
                    } else {
                        std::ptr::write(
                            __slate_slot_1133,
                            ((unsafe {
                                *unsafe { (*__slate_slot_1128).offset((0 as i32) as isize) }
                            }) as u32) as i32,
                        );
                        *__slate_slot_1132 = (*__slate_slot_1129 - (12 as i32)) / (2 as i32);
                        if *__slate_slot_1133 + *__slate_slot_1132 > nKey1 {
                            unsafe {
                                (*pPKey2).errCode =
                                    ((unsafe { sqlite3CorruptError(5046 as i32) }) as i8) as u8;
                            }
                            return 0 as i32;
                        } else {
                            *__slate_slot_1131 = if (unsafe { (*pPKey2).n }) < *__slate_slot_1132 {
                                unsafe { (*pPKey2).n }
                            } else {
                                *__slate_slot_1132
                            };
                            *__slate_slot_1130 = unsafe {
                                memcmp(
                                    (unsafe {
                                        (*__slate_slot_1128).offset(*__slate_slot_1133 as isize)
                                    }) as *const (),
                                    (unsafe { (*pPKey2).u.z }) as *const (),
                                    (*__slate_slot_1131 as i64) as u64,
                                )
                            };
                            if *__slate_slot_1130 > (0 as i32) {
                                *__slate_slot_1130 = (unsafe { (*pPKey2).r2 }) as i32;
                                break '__join_0;
                            } else {
                                if *__slate_slot_1130 < (0 as i32) {
                                    *__slate_slot_1130 = (unsafe { (*pPKey2).r1 }) as i32;
                                    break '__join_0;
                                } else {
                                    *__slate_slot_1130 =
                                        *__slate_slot_1132 - unsafe { (*pPKey2).n };
                                    if *__slate_slot_1130 == (0 as i32) {
                                        if (((unsafe { (*pPKey2).nField }) as u32) as i32)
                                            > (1 as i32)
                                        {
                                            *__slate_slot_1130 = sqlite3VdbeRecordCompareWithSkip(
                                                nKey1, pKey1, pPKey2, 1 as i32,
                                            );
                                            break '__join_0;
                                        } else {
                                            *__slate_slot_1130 =
                                                (unsafe { (*pPKey2).default_rc }) as i32;
                                            unsafe {
                                                (*pPKey2).eqSeen = ((1 as i32) as i8) as u8;
                                            }
                                            break '__join_0;
                                        }
                                    } else {
                                        if *__slate_slot_1130 > (0 as i32) {
                                            *__slate_slot_1130 = (unsafe { (*pPKey2).r2 }) as i32;
                                            break '__join_0;
                                        } else {
                                            *__slate_slot_1130 = (unsafe { (*pPKey2).r1 }) as i32;
                                            break '__join_0;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                0 as i32;
            }
            *__slate_slot_1130 = (unsafe { (*pPKey2).r1 }) as i32; // (pKey1/nKey1) is a number or a null
        }
        0 as i32;
        return *__slate_slot_1130;
    }
    // Corruption
    return unsafe { std::mem::zeroed() };
}

/// Return a pointer to an sqlite3VdbeRecordCompare() compatible function
/// suitable for comparing serialized records to the unpacked record passed
/// as the only argument.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3VdbeFindCompare(
    mut p: *mut UnpackedRecord,
) -> Option<unsafe extern "C-unwind" fn(i32, *const (), *mut UnpackedRecord) -> i32> {
    // varintRecordCompareInt() and varintRecordCompareString() both assume
    // that the size-of-header varint that occurs at the start of each record
    // fits in a single byte (i.e. is 127 or less). varintRecordCompareInt()
    // also assumes that it is safe to overread a buffer by at least the
    // maximum possible legal header size plus 8 bytes. Because there is
    // guaranteed to be at least 74 (but not 136) bytes of padding following each
    // buffer passed to varintRecordCompareInt() this makes it convenient to
    // limit the size of the header to 64 bytes in cases where the first field
    // is an integer.
    //
    // The easiest way to enforce this limit is to consider only records with
    // 13 fields or less. If the first field is an integer, the maximum legal
    // header size is (12*5 + 1 + 1) bytes.
    0 as i32;
    if (((unsafe { (*unsafe { (*p).pKeyInfo }).nAllField }) as u32) as i32) <= (13 as i32) {
        let mut flags: i32 =
            ((unsafe { (*unsafe { unsafe { (*p).aMem }.offset((0 as i32) as isize) }).flags })
                as u32) as i32;
        if (unsafe {
            *unsafe {
                unsafe { (*unsafe { (*p).pKeyInfo }).aSortFlags }.offset((0 as i32) as isize)
            }
        }) != (0 as u8)
        {
            if (((unsafe {
                *unsafe {
                    unsafe { (*unsafe { (*p).pKeyInfo }).aSortFlags }.offset((0 as i32) as isize)
                }
            }) as u32) as i32)
                & (2 as i32)
                != (0 as i32)
            {
                return Some(sqlite3VdbeRecordCompare);
            }
            unsafe {
                (*p).r1 = (1 as i32) as i8;
            }
            unsafe {
                (*p).r2 = -(1 as i32) as i8;
            }
        } else {
            unsafe {
                (*p).r1 = -(1 as i32) as i8;
            }
            unsafe {
                (*p).r2 = (1 as i32) as i8;
            }
        }
        if flags & (4 as i32) != (0 as i32) {
            unsafe {
                (*p).u.i = unsafe {
                    (*unsafe { unsafe { (*p).aMem }.offset((0 as i32) as isize) })
                        .u
                        .i
                };
            }
            return Some(vdbeRecordCompareInt);
        }
        {}
        {}
        {}
        if flags & ((8 as i32) | (32 as i32) | (1 as i32) | (16 as i32)) == (0 as i32)
            && (unsafe {
                *unsafe {
                    unsafe {
                        std::ptr::addr_of_mut!((*unsafe { (*p).pKeyInfo }).aColl)
                            as *mut *mut CollSeq
                    }
                    .offset((0 as i32) as isize)
                }
            }) == std::ptr::null_mut::<CollSeq>()
        {
            0 as i32;
            unsafe {
                (*p).u.z =
                    unsafe { (*unsafe { unsafe { (*p).aMem }.offset((0 as i32) as isize) }).z };
            }
            unsafe {
                (*p).n =
                    unsafe { (*unsafe { unsafe { (*p).aMem }.offset((0 as i32) as isize) }).n };
            }
            return Some(vdbeRecordCompareString);
        }
    }
    return Some(sqlite3VdbeRecordCompare);
}

/// pCur points at an index entry created using the OP_MakeRecord opcode.
/// Read the rowid (the last field in the record) and store it in *rowid.
/// Return SQLITE_OK if everything works, or an error code otherwise.
///
/// pCur might be pointing to text obtained from a corrupt database file.
/// So the content cannot be trusted.  Do appropriate checks on the content.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3VdbeIdxRowid(
    mut db: *mut sqlite3,
    mut pCur: *mut BtCursor,
    mut rowid: *mut i64,
) -> i32 {
    let mut __slate_storage_1146: std::mem::MaybeUninit<sqlite3_value> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1146: *mut sqlite3_value =
        std::ptr::addr_of_mut!(__slate_storage_1146) as *mut sqlite3_value;
    let mut __slate_storage_1145: std::mem::MaybeUninit<sqlite3_value> =
        std::mem::MaybeUninit::uninit();
    let __slate_slot_1145: *mut sqlite3_value =
        std::ptr::addr_of_mut!(__slate_storage_1145) as *mut sqlite3_value; // Size of the rowid
    let mut __slate_storage_1144: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1144: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_1144) as *mut u32; // Serial type of the rowid
    let mut __slate_storage_1143: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1143: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_1143) as *mut u32; // Size of the header
    let mut __slate_storage_1142: std::mem::MaybeUninit<u32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1142: *mut u32 = std::ptr::addr_of_mut!(__slate_storage_1142) as *mut u32;
    let mut __slate_storage_1141: std::mem::MaybeUninit<i32> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1141: *mut i32 = std::ptr::addr_of_mut!(__slate_storage_1141) as *mut i32;
    let mut __slate_storage_1140: std::mem::MaybeUninit<i64> = std::mem::MaybeUninit::uninit();
    let __slate_slot_1140: *mut i64 = std::ptr::addr_of_mut!(__slate_storage_1140) as *mut i64;
    unsafe {
        std::ptr::write(__slate_slot_1140, (0 as i32) as i64);
        // Get the size of the index entry.  Only indices entries of less
        // than 2GiB are support - anything large must be database corruption.
        // Any corruption is detected in sqlite3BtreeParseCellPtr(), though, so
        // this code can safely assume that nCellKey is 32-bits
        0 as i32;
        *__slate_slot_1140 = ((unsafe { sqlite3BtreePayloadSize(pCur) }) as u64) as i64;
        0 as i32;
        // Read in the complete content of the index entry
        unsafe {
            sqlite3VdbeMemInit(
                std::ptr::addr_of_mut!(*__slate_slot_1145),
                db,
                ((0 as i32) as i16) as u16,
            )
        };
        *__slate_slot_1141 = unsafe {
            sqlite3VdbeMemFromBtreeZeroOffset(
                pCur,
                (*__slate_slot_1140 as i32) as u32,
                std::ptr::addr_of_mut!(*__slate_slot_1145),
            )
        };
        if *__slate_slot_1141 != (0 as i32) {
            return *__slate_slot_1141;
        } else {
            // The index entry must begin with a header size
            *__slate_slot_1142 = (unsafe { *((*__slate_slot_1145).z as *mut u8) }) as u32;
            if *__slate_slot_1142 >= ((128 as i32) as u32) {
                unsafe {
                    sqlite3GetVarint32(
                        ((*__slate_slot_1145).z as *mut u8) as *const u8,
                        std::ptr::addr_of_mut!(*__slate_slot_1142),
                    )
                };
            }
            {}
            {}
            {}
            0 as i32;
            if *__slate_slot_1142 < ((3 as i32) as u32)
                || *__slate_slot_1142 > ((*__slate_slot_1145).n as u32)
            {
            } else {
                // The last field of the index should be an integer - the ROWID.
                // Verify that the last entry really is an integer.
                *__slate_slot_1143 = (unsafe {
                    *((unsafe {
                        (*__slate_slot_1145)
                            .z
                            .offset((*__slate_slot_1142).wrapping_sub((1 as i32) as u32) as isize)
                    }) as *mut u8)
                }) as u32;
                if *__slate_slot_1143 >= ((128 as i32) as u32) {
                    unsafe {
                        sqlite3GetVarint32(
                            ((unsafe {
                                (*__slate_slot_1145)
                                    .z
                                    .offset((*__slate_slot_1142).wrapping_sub((1 as i32) as u32)
                                        as isize)
                            }) as *mut u8) as *const u8,
                            std::ptr::addr_of_mut!(*__slate_slot_1143),
                        )
                    };
                }
                {}
                {}
                {}
                {}
                {}
                {}
                {}
                {}
                if *__slate_slot_1143 < ((1 as i32) as u32)
                    || *__slate_slot_1143 > ((9 as i32) as u32)
                    || *__slate_slot_1143 == ((7 as i32) as u32)
                {
                } else {
                    *__slate_slot_1144 = (unsafe {
                        *unsafe {
                            unsafe { std::ptr::addr_of!(sqlite3SmallTypeSizes.0) as *const u8 }
                                .offset(*__slate_slot_1143 as isize)
                        }
                    }) as u32;
                    {}
                    if ((*__slate_slot_1145).n as u32)
                        < (*__slate_slot_1142).wrapping_add(*__slate_slot_1144)
                    {
                    } else {
                        // Fetch the integer off the end of the index record
                        sqlite3VdbeSerialGet(
                            ((unsafe {
                                (*__slate_slot_1145).z.offset(
                                    ((*__slate_slot_1145).n as u32).wrapping_sub(*__slate_slot_1144)
                                        as isize,
                                )
                            }) as *mut u8) as *const u8,
                            *__slate_slot_1143,
                            std::ptr::addr_of_mut!(*__slate_slot_1146),
                        );
                        unsafe {
                            *rowid = unsafe { (*__slate_slot_1146).u.i };
                        }
                        unsafe {
                            sqlite3VdbeMemReleaseMalloc(std::ptr::addr_of_mut!(*__slate_slot_1145))
                        };
                        return 0 as i32;
                    }
                }
            }
            {}
            unsafe { sqlite3VdbeMemReleaseMalloc(std::ptr::addr_of_mut!(*__slate_slot_1145)) };
            return unsafe { sqlite3CorruptError(5205 as i32) };
        }
    }
    // Jump here if database corruption is detected after m has been
    // allocated.  Free the m object and return SQLITE_CORRUPT.
    return unsafe { std::mem::zeroed() };
}

/// Compare the key of the index entry that cursor pC is pointing to against
/// the key string in pUnpacked.  Write into *pRes a number
/// that is negative, zero, or positive if pC is less than, equal to,
/// or greater than pUnpacked.  Return SQLITE_OK on success.
///
/// pUnpacked is either created without a rowid or is truncated so that it
/// omits the rowid at the end.  The rowid at the end of the index entry
/// is ignored as well.  Hence, this routine only compares the prefixes
/// of the keys prior to the final rowid, not the entire key.
///
/// # Arguments
///
/// * `db` - Database connection
/// * `pC` - The cursor to compare against
/// * `pUnpacked` - Unpacked version of key
/// * `res` - Write the comparison result here
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3VdbeIdxKeyCompare(
    mut db: *mut sqlite3,
    mut pC: *mut VdbeCursor,
    mut pUnpacked: *mut UnpackedRecord,
    mut res: *mut i32,
) -> i32 {
    let mut nCellKey: i64 = (0 as i32) as i64;
    let mut rc: i32 = 0 as i32;
    let mut pCur: *mut BtCursor = unsafe { std::mem::zeroed() };
    let mut m: sqlite3_value = unsafe { std::mem::zeroed() };
    0 as i32;
    pCur = unsafe { (*pC).uc.pCursor };
    0 as i32;
    nCellKey = ((unsafe { sqlite3BtreePayloadSize(pCur) }) as u64) as i64;
    // nCellKey will always be between 0 and 0xffffffff because of the way
    // that btreeParseCellPtr() and sqlite3GetVarint32() are implemented
    if nCellKey <= ((0 as i32) as i64) || nCellKey > ((2147483647 as i32) as i64) {
        unsafe {
            *res = 0 as i32;
        }
        return unsafe { sqlite3CorruptError(5238 as i32) };
    }
    unsafe { sqlite3VdbeMemInit(std::ptr::addr_of_mut!(m), db, ((0 as i32) as i16) as u16) };
    rc = unsafe {
        sqlite3VdbeMemFromBtreeZeroOffset(pCur, (nCellKey as i32) as u32, std::ptr::addr_of_mut!(m))
    };
    if rc != (0 as i32) {
        return rc;
    }
    unsafe {
        *res = sqlite3VdbeRecordCompareWithSkip(m.n, m.z as *const (), pUnpacked, 0 as i32);
    }
    unsafe { sqlite3VdbeMemReleaseMalloc(std::ptr::addr_of_mut!(m)) };
    return 0 as i32;
}

/// This routine sets the value to be returned by subsequent calls to
/// sqlite3_changes() on the database handle 'db'.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3VdbeSetChanges(mut db: *mut sqlite3, mut nChange: i64) {
    0 as i32;
    unsafe {
        (*db).nChange = nChange;
    }
    let __v1840: *mut sqlite3 = db;
    let __v1841: i64 = unsafe { (*__v1840).nTotalChange };
    let __v1842: i64 = __v1841 + nChange;
    unsafe {
        (*__v1840).nTotalChange = __v1842;
    }
}

/// Set a flag in the vdbe to update the change counter when it is finalised
/// or reset.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3VdbeCountChanges(mut v: *mut Vdbe) {
    unsafe {
        (*v).__slate_bits_0.__set_changeCntOn((1 as i32) as u32);
    }
}

/// Mark every prepared statement associated with a database connection
/// as expired.
///
/// An expired statement means that recompilation of the statement is
/// recommend.  Statements expire when things happen that make their
/// programs obsolete.  Removing user-defined functions or collating
/// sequences, or changing an authorization function are the types of
/// things that make prepared statements obsolete.
///
/// If iCode is 1, then expiration is advisory.  The statement should
/// be reprepared before being restarted, but if it is already running
/// it is allowed to run to completion.
///
/// Internally, this function just sets the Vdbe.expired flag on all
/// prepared statements.  The flag is set to 1 for an immediate expiration
/// and set to 2 for an advisory expiration.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3ExpirePreparedStatements(mut db: *mut sqlite3, mut iCode: i32) {
    let mut p: *mut Vdbe = unsafe { std::mem::zeroed() };
    p = unsafe { (*db).pVdbe };
    '__slate_break_1741: while p != std::ptr::null_mut::<Vdbe>() {
        unsafe {
            (*p).__slate_bits_0
                .__set_expired((iCode + (1 as i32)) as u32);
        }
        p = unsafe { (*p).pVNext };
    }
}

/// Return the database associated with the Vdbe.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3VdbeDb(mut v: *mut Vdbe) -> *mut sqlite3 {
    return unsafe { (*v).db };
}

/// Return the SQLITE_PREPARE flags for a Vdbe.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3VdbePrepareFlags(mut v: *mut Vdbe) -> u8 {
    return unsafe { (*v).prepFlags };
}

/// Return a pointer to an sqlite3_value structure containing the value bound
/// parameter iVar of VM v. Except, if the value is an SQL NULL, return
/// 0 instead. Unless it is NULL, apply affinity aff (one of the SQLITE_AFF_*
/// constants) to the value before returning it.
///
/// The returned value must be freed by the caller using sqlite3ValueFree().
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3VdbeGetBoundValue(
    mut v: *mut Vdbe,
    mut iVar: i32,
    mut aff: u8,
) -> *mut sqlite3_value {
    0 as i32;
    if v != std::ptr::null_mut::<Vdbe>() {
        let mut pMem: *mut sqlite3_value =
            unsafe { unsafe { (*v).aVar }.offset((iVar - (1 as i32)) as isize) };
        0 as i32;
        if (0 as i32) == (((unsafe { (*pMem).flags }) as u32) as i32) & (1 as i32) {
            let mut pRet: *mut sqlite3_value = unsafe { sqlite3ValueNew(unsafe { (*v).db }) };
            if pRet != std::ptr::null_mut::<sqlite3_value>() {
                unsafe { sqlite3VdbeMemCopy(pRet, pMem as *const sqlite3_value) };
                unsafe { sqlite3ValueApplyAffinity(pRet, aff, ((1 as i32) as i8) as u8) };
            }
            return pRet;
        }
    }
    return std::ptr::null_mut::<sqlite3_value>();
}

/// Configure SQL variable iVar so that binding a new value to it signals
/// to sqlite3_reoptimize() that re-preparing the statement may result
/// in a better query plan. If parameter bSmallint is true, then the
/// statement is only re-prepared if the new value is integer value 0 or 1.
///
/// The v->expmask bit is always set.  expmask means that a reprepare is
/// possible.  The v->smimask bit is only set if we want to restrict
/// reprepare when the value changes from (0,1) to something else, or from
/// something else to (0,1).
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3VdbeReprepareOnBind(
    mut v: *mut Vdbe,
    mut iVar: i32,
    mut bSmallint: i32,
) {
    let mut m: u32 = 0 as u32;
    0 as i32;
    0 as i32;
    m = if iVar >= (32 as i32) {
        2147483648 as u32
    } else {
        ((1 as i32) as u32) << iVar - (1 as i32)
    };
    let __v1809: *mut Vdbe = v;
    let __v1810: u32 = unsafe { (*__v1809).expmask };
    let __v1811: u32 = __v1810 | m;
    unsafe {
        (*__v1809).expmask = __v1811;
    }
    if bSmallint != (0 as i32) {
        let __v1812: *mut Vdbe = v;
        let __v1813: u32 = unsafe { (*__v1812).smimask };
        let __v1814: u32 = __v1813 | m;
        unsafe {
            (*__v1812).smimask = __v1814;
        }
    }
    // smimask is always a subset of expmask
    0 as i32;
}

/// Helper function for vdbeIsMatchingIndexKey(). Return true if column
/// iCol should be ignored when comparing a record with a record from
/// an index on disk. The field should be ignored if:
///
///   * the corresponding bit in mask is set, and
///   * either:
///       - bIntegrity is false, or
///       - the two Mem values are both real values that differ by
///         BTREE_ULPDISTORTION or fewer ULPs.
///
/// # Arguments
///
/// * `mask` - Mask of indexed expression fields
/// * `iCol` - Column of index being considered
/// * `pMem1` - Expected index value
/// * `pMem2` - Actual indexed value
/// * `bIntegrity` - True if running PRAGMA integrity_check
fn vdbeSkipField(
    mut mask: u64,
    mut iCol: i32,
    mut pMem1: *mut sqlite3_value,
    mut pMem2: *mut sqlite3_value,
    mut bIntegrity: i32,
) -> i32 {
    if iCol >= (((8 as u64).wrapping_mul(((8 as i32) as i64) as u64) as u32) as i32)
        || mask & (((1 as i32) as i64) as u64) << iCol == (((0 as i32) as i64) as u64)
    {
        return 0 as i32;
    }
    if bIntegrity == (0 as i32) {
        return 1 as i32;
    }
    if (((unsafe { (*pMem1).flags }) as u32) as i32) & (8 as i32) != (0 as i32)
        && (((unsafe { (*pMem2).flags }) as u32) as i32) & (8 as i32) != (0 as i32)
    {
        let mut m1: u64 = 0 as u64;
        let mut m2: u64 = 0 as u64;
        unsafe {
            memcpy(
                std::ptr::addr_of_mut!(m1) as *mut (),
                (unsafe { std::ptr::addr_of_mut!((*pMem1).u.r) }) as *const (),
                ((8 as i32) as i64) as u64,
            )
        };
        unsafe {
            memcpy(
                std::ptr::addr_of_mut!(m2) as *mut (),
                (unsafe { std::ptr::addr_of_mut!((*pMem2).u.r) }) as *const (),
                ((8 as i32) as i64) as u64,
            )
        };
        if (if m1 < m2 {
            m2.wrapping_sub(m1)
        } else {
            m1.wrapping_sub(m2)
        }) <= (((2 as i32) as i64) as u64)
        {
            return 1 as i32;
        }
    }
    return 0 as i32;
}

/// This function compares the unpacked record with the current key that
/// cursor pCur points to. If bInt is false, all fields for which the
/// corresponding bit in parameter "mask" is set are ignored. Or, if
/// bInt is true, then a difference of BTREE_ULPDISTORTION or fewer ULPs
/// in real values is overlooked for fields with the corresponding bit
/// set in mask.
///
/// Return the usual less than zero, zero, or greater than zero if the
/// remaining fields of the cursor cursor key are less than, equal to or
/// greater than those in (*p).
///
/// # Arguments
///
/// * `pCur` - Cursor open on index
/// * `bInt` - True for integrity_check-style search
/// * `mask` - Mask of columns to skip
/// * `p` - Index key being deleted
/// * `piRes` - 0 for a match, non-zero for not a match
fn vdbeIsMatchingIndexKey(
    mut pCur: *mut BtCursor,
    mut bInt: i32,
    mut mask: u64,
    mut p: *mut UnpackedRecord,
    mut piRes: *mut i32,
) -> i32 {
    let mut aRec: *mut u8 = std::ptr::null_mut::<u8>();
    let mut nRec: u32 = (0 as i32) as u32;
    let mut m: sqlite3_value = unsafe { std::mem::zeroed() };
    let mut rc: i32 = 0 as i32;
    unsafe { memset(std::ptr::addr_of_mut!(m) as *mut (), 0 as i32, 56 as u64) };
    m.enc = unsafe { (*unsafe { (*p).pKeyInfo }).enc };
    m.db = unsafe { (*unsafe { (*p).pKeyInfo }).db };
    nRec = unsafe { sqlite3BtreePayloadSize(pCur) };
    if nRec > ((2147483647 as i32) as u32) {
        return unsafe { sqlite3CorruptError(5421 as i32) };
    }
    // Allocate 5 extra bytes at the end of the buffer. This allows the
    // getVarint32() call below to read slightly past the end of the buffer
    // if the record is corrupt.
    aRec = (unsafe { sqlite3MallocZero(nRec.wrapping_add((5 as i32) as u32) as u64) }) as *mut u8;
    if aRec == std::ptr::null_mut::<u8>() {
        rc = 7 as i32;
    } else {
        rc = unsafe { sqlite3BtreePayload(pCur, (0 as i32) as u32, nRec, aRec as *mut ()) };
    }
    if rc == (0 as i32) {
        let mut szHdr: u32 = (0 as i32) as u32; // Size of record header in bytes
        let mut idxHdr: u32 = (0 as i32) as u32; // Current index in header
        let __v1977: i32;
        if (((unsafe { *aRec }) as u32) as i32) < (((((128 as i32) as i8) as u8) as u32) as i32) {
            szHdr = (unsafe { *aRec }) as u32;
            __v1977 = 1 as i32;
        } else {
            __v1977 =
                ((unsafe { sqlite3GetVarint32(aRec as *const u8, std::ptr::addr_of_mut!(szHdr)) })
                    as u32) as i32;
        }
        idxHdr = ((__v1977 as i8) as u8) as u32;
        if szHdr > ((98307 as i32) as u32) {
            rc = 11 as i32;
        } else {
            let mut res: i32 = 0 as i32; // Result of this function call
            let mut idxRec: u32 = szHdr; // Index of next field in record body
            let mut ii: i32 = 0 as i32; // Iterator variable
            let mut nCol: i32 = ((unsafe { (*unsafe { (*p).pKeyInfo }).nAllField }) as u32) as i32;
            ii = 0 as i32;
            '__slate_break_1742: loop {
                if !(ii < nCol && rc == (0 as i32)) {
                    break;
                }
                let mut iSerial: u32 = (0 as i32) as u32;
                let mut nSerial: i32 = 0 as i32;
                if idxHdr >= szHdr {
                    rc = unsafe { sqlite3CorruptError(5452 as i32) };
                    break '__slate_break_1742;
                }
                let __v1980: u32 = idxHdr;
                let __v1981: i32;
                if (((unsafe { *unsafe { aRec.offset(idxHdr as isize) } }) as u32) as i32)
                    < (((((128 as i32) as i8) as u8) as u32) as i32)
                {
                    iSerial = (unsafe { *unsafe { aRec.offset(idxHdr as isize) } }) as u32;
                    __v1981 = 1 as i32;
                } else {
                    __v1981 = ((unsafe {
                        sqlite3GetVarint32(
                            (unsafe { aRec.offset(idxHdr as isize) }) as *const u8,
                            std::ptr::addr_of_mut!(iSerial),
                        )
                    }) as u32) as i32;
                }
                let __v1982: u32 =
                    __v1980.wrapping_add(((((__v1981 as i8) as u8) as u32) as i32) as u32);
                idxHdr = __v1982;
                nSerial = sqlite3VdbeSerialTypeLen(iSerial) as i32;
                if idxRec.wrapping_add(nSerial as u32) > nRec {
                    rc = unsafe { sqlite3CorruptError(5458 as i32) };
                } else {
                    sqlite3VdbeSerialGet(
                        (unsafe { aRec.offset(idxRec as isize) }) as *const u8,
                        iSerial,
                        std::ptr::addr_of_mut!(m),
                    );
                    if vdbeSkipField(
                        mask,
                        ii,
                        unsafe { unsafe { (*p).aMem }.offset(ii as isize) },
                        std::ptr::addr_of_mut!(m),
                        bInt,
                    ) == (0 as i32)
                    {
                        res = sqlite3MemCompare(
                            std::ptr::addr_of_mut!(m) as *const sqlite3_value,
                            (unsafe { unsafe { (*p).aMem }.offset(ii as isize) })
                                as *const sqlite3_value,
                            (unsafe {
                                *unsafe {
                                    unsafe {
                                        std::ptr::addr_of_mut!((*unsafe { (*p).pKeyInfo }).aColl)
                                            as *mut *mut CollSeq
                                    }
                                    .offset(ii as isize)
                                }
                            }) as *const CollSeq,
                        );
                        if res != (0 as i32) {
                            break '__slate_break_1742;
                        }
                    }
                }
                let __v1983: u32 = idxRec;
                let __v1984: u32 = __v1983.wrapping_add(sqlite3VdbeSerialTypeLen(iSerial));
                idxRec = __v1984;
                let __v1978: i32 = ii;
                let __v1979: i32 = __v1978 + (1 as i32);
                ii = __v1979;
            }
            unsafe {
                *piRes = res;
            }
        }
    }
    unsafe { sqlite3_free(aRec as *mut ()) };
    return rc;
}

/// This is called when the record in (*p) should be found in the index
/// opened by cursor pCur, but was not. This may happen as part of a DELETE
/// operation or an integrity check.
///
/// One reason that an exact match was not found may be the EIIB bug - that
/// a text-to-float conversion may have caused a real value in record (*p)
/// to be slightly different from its counterpart on disk. This function
/// attempts to find the right index record. If it does find the right
/// record, it leaves *pCur pointing to it and sets (*pRes) to 0 before
/// returning. Otherwise, (*pRes) is set to non-zero and an SQLite error
/// code returned.
///
/// The algorithm used to find the correct record is:
///
///   * Scan up to BTREE_FDK_RANGE entries either side of the current entry.
///     If parameter bIntegrity is false, then all fields that are indexed
///     expressions or virtual table columns are omitted from the comparison.
///     If bIntegrity is true, then small differences in real values in
///     such fields are overlooked, but they are not omitted from the comparison
///     altogether.
///
///   * If the above fails to find an entry and bIntegrity is false, search
///     the entire index.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3VdbeFindIndexKey(
    mut pCur: *mut BtCursor,
    mut pIdx: *mut Index,
    mut p: *mut UnpackedRecord,
    mut pRes: *mut i32,
    mut bIntegrity: i32,
) -> i32 {
    let mut nStep: i32 = 0 as i32;
    let mut res: i32 = 1 as i32;
    let mut rc: i32 = 0 as i32;
    let mut ii: i32 = 0 as i32;
    // Calculate a mask based on the first 64 columns of the index. The mask
    // bit is set if the corresponding index field is either an expression
    // or a virtual column of the table.
    let mut mask: u64 = ((0 as i32) as i64) as u64;
    ii = 0 as i32;
    '__slate_break_1743: loop {
        if !(ii
            < if (((unsafe { (*pIdx).nColumn }) as u32) as i32)
                < (((8 as u64).wrapping_mul(((8 as i32) as i64) as u64) as u32) as i32)
            {
                ((unsafe { (*pIdx).nColumn }) as u32) as i32
            } else {
                ((8 as u64).wrapping_mul(((8 as i32) as i64) as u64) as u32) as i32
            })
        {
            break;
        }
        let mut iCol: i32 =
            (unsafe { *unsafe { unsafe { (*pIdx).aiColumn }.offset(ii as isize) } }) as i32;
        if iCol == -(2 as i32)
            || iCol >= (0 as i32)
                && (((unsafe {
                    (*unsafe { unsafe { (*unsafe { (*pIdx).pTable }).aCol }.offset(iCol as isize) })
                        .colFlags
                }) as u32) as i32)
                    & (32 as i32)
                    != (0 as i32)
        {
            let __v1916: u64 = mask;
            let __v1917: u64 = __v1916 | (((1 as i32) as i64) as u64) << ii;
            mask = __v1917;
        }
        let __v1914: i32 = ii;
        let __v1915: i32 = __v1914 + (1 as i32);
        ii = __v1915;
    }
    // If the mask is 0 at this point, then the index contains no expressions
    // or virtual columns. So do not search for a match - return so that the
    // caller may declare the db corrupt immediately. Or, if mask is non-zero,
    // proceed.
    if mask != (((0 as i32) as i64) as u64) {
        // Move the cursor back BTREE_FDK_RANGE entries. If this hits an EOF,
        // position the cursor at the first entry in the index and set nStep
        // to -1 so that the first loop below scans the entire index. Otherwise,
        // set nStep to BTREE_FDK_RANGE*2 so that the first loop below scans
        // just that many entries.
        ii = 0 as i32;
        '__slate_break_1744: loop {
            if !((unsafe { sqlite3BtreeEof(pCur) }) == (0 as i32) && ii < (10 as i32)) {
                break;
            }
            rc = unsafe { sqlite3BtreePrevious(pCur, 0 as i32) };
            let __v1918: i32 = ii;
            let __v1919: i32 = __v1918 + (1 as i32);
            ii = __v1919;
        }
        if rc == (101 as i32) {
            rc = unsafe { sqlite3BtreeFirst(pCur, std::ptr::addr_of_mut!(res)) };
            nStep = -(1 as i32);
        } else {
            nStep = (10 as i32) * (2 as i32);
        }
        // This loop runs at most twice to search for a key with matching PK
        // fields in the index. The second iteration always searches the entire
        // index. The first iteration searches nStep entries starting with the
        // current cursor entry if (nStep>=0), or the entire index if (nStep<0).
        '__slate_break_1745: while (unsafe { sqlite3BtreeCursorIsValidNN(pCur) }) != (0 as i32) {
            ii = 0 as i32;
            '__slate_break_1746: loop {
                if !(rc == (0 as i32) && (ii < nStep || nStep < (0 as i32))) {
                    break;
                }
                rc = vdbeIsMatchingIndexKey(pCur, bIntegrity, mask, p, std::ptr::addr_of_mut!(res));
                if res == (0 as i32) || rc != (0 as i32) {
                    break '__slate_break_1746;
                }
                rc = unsafe { sqlite3BtreeNext(pCur, 0 as i32) };
                let __v1920: i32 = ii;
                let __v1921: i32 = __v1920 + (1 as i32);
                ii = __v1921;
            }
            if rc == (101 as i32) {
                rc = 0 as i32;
                0 as i32;
            }
            if nStep < (0 as i32)
                || rc != (0 as i32)
                || res == (0 as i32)
                || bIntegrity != (0 as i32)
            {
                break '__slate_break_1745;
            }
            // The first, non-exhaustive, search failed to find an entry with
            // matching PK fields. So restart for an exhaustive search of the
            // entire index.
            nStep = -(1 as i32);
            rc = unsafe { sqlite3BtreeFirst(pCur, std::ptr::addr_of_mut!(res)) };
        }
    }
    unsafe {
        *pRes = res;
    }
    return rc;
}

/// Cause a function to throw an error if it was call from OP_PureFunc
/// rather than OP_Function.
///
/// OP_PureFunc means that the function must be deterministic, and should
/// throw an error if it is given inputs that would make it non-deterministic.
/// This routine is invoked by date/time functions that use non-deterministic
/// features such as 'now'.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3NotPureFunc(mut pCtx: *mut sqlite3_context) -> i32 {
    let mut pOp: *const VdbeOp = unsafe { std::mem::zeroed() };
    pOp = (unsafe {
        unsafe { (*unsafe { (*pCtx).pVdbe }).aOp }.offset((unsafe { (*pCtx).iOp }) as isize)
    }) as *const VdbeOp;
    if (((unsafe { (*pOp).opcode }) as u32) as i32) == (67 as i32) {
        let mut zContext: *const i8 = unsafe { std::mem::zeroed() };
        let mut zMsg: *mut i8 = unsafe { std::mem::zeroed() };
        if (((unsafe { (*pOp).p5 }) as u32) as i32) & (4 as i32) != (0 as i32) {
            zContext = (b"a CHECK constraint\0".as_ptr() as *mut i8) as *const i8;
        } else {
            if (((unsafe { (*pOp).p5 }) as u32) as i32) & (8 as i32) != (0 as i32) {
                zContext = (b"a generated column\0".as_ptr() as *mut i8) as *const i8;
            } else {
                zContext = (b"an index\0".as_ptr() as *mut i8) as *const i8;
            }
        }
        zMsg = unsafe {
            sqlite3_mprintf(
                (b"non-deterministic use of %s() in %s\0".as_ptr() as *mut i8) as *const i8,
                unsafe { (*unsafe { (*pCtx).pFunc }).zName },
                zContext,
            )
        };
        unsafe { sqlite3_result_error(pCtx, zMsg as *const i8, -(1 as i32)) };
        unsafe { sqlite3_free(zMsg as *mut ()) };
        return 0 as i32;
    }
    return 1 as i32;
}

/// Transfer error message text from an sqlite3_vtab.zErrMsg (text stored
/// in memory obtained from sqlite3_malloc) into a Vdbe.zErrMsg (text stored
/// in memory obtained from sqlite3DbMalloc).
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3VtabImportErrmsg(mut p: *mut Vdbe, mut pVtab: *mut sqlite3_vtab) {
    if (unsafe { (*pVtab).zErrMsg }) != std::ptr::null_mut::<i8>() {
        let mut db: *mut sqlite3 = unsafe { (*p).db };
        unsafe { sqlite3DbFree(db, (unsafe { (*p).zErrMsg }) as *mut ()) };
        unsafe {
            (*p).zErrMsg =
                unsafe { sqlite3DbStrDup(db, (unsafe { (*pVtab).zErrMsg }) as *const i8) };
        }
        unsafe { sqlite3_free((unsafe { (*pVtab).zErrMsg }) as *mut ()) };
        unsafe {
            (*pVtab).zErrMsg = std::ptr::null_mut::<i8>();
        }
    }
}

/// Return the name of an SQL function associated with the sqlite3_context.
#[unsafe(no_mangle)]
extern "C-unwind" fn sqlite3VdbeFuncName(mut pCtx: *const sqlite3_context) -> *const i8 {
    0 as i32;
    0 as i32;
    return unsafe { (*unsafe { (*pCtx).pFunc }).zName };
}
